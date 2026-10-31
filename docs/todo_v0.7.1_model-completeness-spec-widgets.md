# v0.7.1 — Model completeness: Specification, widget catalogue

**Status:** done with the specification, 2026-10-29; merged from eight groups' catalogues and revised after each
of eleven check passes and a systematic absence pass (the core's §0 records them); its per-platform details are
confirmed or corrected by the facts in the value phase. Part of the specification ([`todo_v0.7.1_model-completeness-spec.md`](todo_v0.7.1_model-completeness-spec.md), "S…"; its
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
Where groups named one thing differently, the merge kept one name (the rename table, an appendix at the end); where a group needed
something the core lacks, the addition is cited as `ADD-Ann` (a detail inside the rationale's mechanisms), `ADD-Bnn`
(a needed decision the rationale took in R25) or `CONTRA-n` (a contradiction with the rationale, settled there), as
the core's additions register (S22) records them.

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
- `UIButton.Configuration.CornerStyle` (`button` on `ios-*`, B-4): its cases, and so the corner-style variants
  `@corner_fixed` … `@corner_dynamic` and their `exclusive_variants` group with `pill`, are as the gap map reads Apple's
  documentation until the iOS lane confirms them against the pinned Xcode's `UIButtonConfiguration.h` (listed as
  stated; ADD-A122).
- `time_picker` on `macos-*`: whether `NSDatePicker`'s graphical style with time elements alone draws the clock face
  alone (MAC-N04; `inline` not declared there until the probe shows it).
- `spinner` on `ios-*` under `!{indeterminate}` (SwiftUI's determinate circular ProgressView, C-31, to verify): whether
  iOS draws it, with a track, and its geometry; the `track`, `spoke`, `indicator` and `stroke_width_px` grants for it
  are provisional.
- `expander` on `ios-*`: SwiftUI's standalone DisclosureGroup (outside a List), whose look no iOS row states (EX-1–EX-4
  are the list's outline disclosure); `ios-*` declares it `available = false` until the value phase measures it, then
  with `header` and `arrow`.
- `component.casting_picker`: the part and field `AccentColor` and `SelectedAccentColor` paint (W-P4-18†'s capture;
  until then no part refers to them, or the picker leaves them unused).

Core questions this catalogue had are answered in the core (ADD-A100): `token.caret.blink_timeout` is `none` where
the caret never stops blinking (K3.41, MAC-X12, X-CA-2); Kirigami's label-sized icons under a user's font (Q1.1) are
`= kirigami_label_icon_size(body_font_height)` on the parts that use them, `token.icons.sizes.keyed.for_labels`
holding the default font's size for what no part carries; `widget.<part>.line_height` and `.font_size` are named.

The ninth check's core questions are answered (ADD-A104, ADD-A105): key repeat switched off (GN-3.68†, K-P3-16†) is
`none` on `Behavior.key_repeat` (`Option<KeyRepeat>`, whose `delay` and `interval` are never absent); AdwPreferencesDialog's
breakpoints measure its breakpoint bin, `Measured::Part(dialog.content)` (the `dialog` entry).

## Entry format

Each `### <widget>` section states, in this order:

- **Kind:** typed widget, and which platforms draw it, with the rows (counting the cross-check decisions that found it
  drawn inside an element, R14).
- **Available:** the presets that declare the widget. "Every preset", or "every preset but `x`, `y`": each preset
  named there declares `available = false` (S10), because its platform has no such control, and the query answers
  `NotAvailable` for it (S8). Where only one of a preset's flavours draws it, the entry says "`kde-breeze` flavour
  `<f>` only" (or `material-3`'s): the other flavour declares `[declare.flavor.<other>.widget.<w>] available = false`
  (S10).
  Preset identifiers: `adwaita`, `kde-breeze` (flavours `widgets`, `kirigami`), `macos-*` (`macos-monterey`, the macOS
  12–15 generation, and `macos-tahoe`, the macOS 26 generation, describing 26.x), `windows-11`, `ios-*` (`ios-13`, the
  iOS 13–18 generation, and `ios-26`, the iOS 26 generation, describing 26.x; R1.5, D12), `material-3` (flavours `baseline`,
  the default, Material 3's components, and `expressive`, with Material 3 Expressive's additions; an entry marks what
  only `expressive` draws "flavour `expressive`", or selects it by `cond flavor:expressive`) (R1.5).
- **Parts:** each part's name, what it is, and its fields, written `` `part` — what it is: `field`, `field` ``. A line
  (a separator, divider or rule) is a `stroke` on the edge it lies along (its width and paint), its insets from its
  ends the part's `margin`, never `padding` (a stroke lies on its box's edge, S2.4, so padding does not shorten it);
  a band with a height of its own (iOS's thick group separator) is a `fill`. An arc or ring
  along a circle that stays still (a spinner's track, a dial's groove and value arc, a circular gauge's, an activity
  ring) is such a band, not an edge of its part's box: the part's `fill` paints it, and its width and line cap are the
  widget's values, named `stroke_width` and `cap` on every widget (`spinner`'s track, `dial`, `level_indicator`,
  `component.activity_rings`), one structure on every widget. An arc a loop moves and lengthens, a spinner's, is its
  part's `glyph`, as the core states it (S2.7, S2.9): a stroked circle path, `glyph.stroke` its paint, width and cap,
  `draw_start` and `draw_extent` its start and extent, which the loop's keyframes set (on a spinner not
  `{indeterminate}`, `draw_extent = element.progress`). The
  first part is the widget's **root** (S2.11): a container holding the widget's other parts, never absent as a whole
  and never a repeated element (S6.3; its fields may be `Absent`); its bindings apply to every part of the widget and
  its descendants (R5.2). An overlay (a dialog, sheet, sidebar, popover, menu, teaching tip, a search field's view)
  has a root that draws nothing, the layer it is presented in (`layer`, or `split` for the sidebar), whose parts are
  the box and what lies beside the box (its scrim, dismiss overlay or backdrop, the content beside a pane, a cancel
  box): the box binds its surface set, and its presentation, opacity and transform move the box and what nests in it,
  never the scrim beside it. Each part's field list is normative (S2.11): the part has exactly the `PartStyle` fields
  listed, the derive generates that set, and a patch setting another field is refused (S12). A part marked
  *(element)* has states, siblings and data of its own: it is queried as an element (S8) in the scope the widget's
  root returns (or an enclosing *(element)* part's); one nested in a plain part (`title` in `title_bar_content`,
  `toolbar.item_group` in `content`) is queried in the root's scope with the plain part's bindings, opacity and
  transform applied (S2.11), which is the scope the plain part's query returns (S8), so `context w.part` matches the
  plain part too. A nested widget (Nested controls) in a plain part (a response `button` in `dialog.action_area`) is
  queried in that same scope. The parts named as belonging to an *(element)* (an item's label, icon or
  mark) are queried with that element; every other part is queried with the widget's element (S2.11). So a part that
  a rule selects by its own states, its own variants or flags, by its position among its own siblings, or uses as a
  context with states of its own is an *(element)*; a part of the widget's element is selected by the widget's states.
  An *(element)* part's query also sees its widget's variants (S2.11): a row matches the list's `sidebar`. A part
  whose style reads element data declares it on the part (`data = { <part> = [...] }`); a widget value that reads
  element data (a presentation step's duration over `move_distance` included) declares it under `values` (`data = {
  values = [...] }`) (S8, S10). The root's inputs, `element.window_width`, `window_height`, `screen_width` and
  `screen_height`, come through the scope (S8), not from `Element.data`, and are declared nowhere.
- **Nesting** (S2.11): parts nest as the entry nests them, the same on every preset. A part nests in the root unless
  its bullet names its parent (`` `title` *(element)*, in `title_bar_content` ``), or the root's bullet names the box
  every other part nests in (an overlay's `body`); the parts belonging to an *(element)* part nest in it. A part's
  `bindings`, `opacity` and `transform` reach every part nested in it and their elements (the title bar's Header
  binding reaches the title and the caption buttons; the title-bar content's inactive dimming dims them as one
  group). A part nested in a part that answers `NotDrawn` is still queried, in the scope the `NotDrawn` part's
  parent returns (as S8 queries the children of a `NotDrawn` element), and is drawn unless it is absent itself: a list's
  rows nest in `section` and are drawn where no section is; where a nested part is not drawn with its parent (a
  dialog's title when its header is not drawn), it has its own `may_be_absent` permission. The parts nested in a
  part a preset draws as another widget (`drawn_as`, below) are still queried, with that widget's element as their
  host: in the scope returned by the query of the substitute's part that holds content (an overlay's box, `body`; a
  control's root), the innermost enclosing part's scope (S8), so `context <substitute>` matches them. Each keeps its
  own style, absence or `drawn_as`; one whose place the substitute's own parts take (a combo box's own label where a
  date picker's field is drawn as a combo box) is stated `absent = true` by a rule under the same selector as the
  substitution, and the entry lists it. So kirigami-addons' menu actions, in the drawer or dialog the menu's box is
  drawn as, are each `drawn_as` a form row (K-KA-31†).
- **One-edge lines:** a `stroke` drawn on some edges only (a status bar's top line, a toolbar's hairline) states the
  other edges `Absent`: an `Edge`'s `width` and `paint` may be absent, an edge not drawn (S6.3), the model's one route
  for "the platform draws no line here"; width zero is never used for it.
- **Widget values:** typed values beyond parts (gaps, alignment, order, behaviour, placement, breakpoints), each with
  its type. A presentation-transition list is `presentations: Vec<PresentationTransition>` (S2.7), keyed by event,
  one event per occasion: `Appear` and `Disappear` are the element appearing and disappearing, `Move` the element
  moving to a new place (a tab or row dropped at its new position, over `element.move_distance`). The element is the
  one `values()` is queried with (S8): queried with an item *(element)* (a row, a tab, a thumbnail), the events and
  their element data (`move_distance`) are that item's; queried with the widget's own element, they are the widget's
  (a tab strip appearing). A widget lists, per event, the element it presents; every other event
  is keyed (`@event`, declared, S10). A part shown and hidden with a state (a check mark, a radio's dot, a selection
  pill, a slider's value bubble, a picker's popup) is `Absent` by a rule in the states it is hidden in and needs no
  event: a part that becomes drawn enters from transparent with its own geometry, and one that becomes absent leaves
  to transparent, by the `transitions` of the style it goes to or comes from (S2.7), so its fade is its
  `transitions` (group `Opacity`), and a scale or slide while it is drawn its `transform` per state. A motion the
  enter rule cannot state (a part sliding in from an edge, an indicator sliding from the old selection to the new) is
  a keyed event whose steps name the part, a part of the widget or of one of its *(element)* parts (S2.7):
  `@indicator_move`, the selection moving to another item, queried with the widget's own element, its step on the
  item's `indicator` (`tab.indicator`), which the step moves from its box in the previously selected item, where the
  connector last drew it, to its box in the newly selected one (and other steps on what else that occasion moves, a
  content area popping in). One occasion has one
  event name on every widget. A value change animated on a part (a slider's or a
  switch's thumb moving, a progress fill growing, a wheel settling) is that part's `transitions` (group `Geometry`),
  not a widget value or an event: a `Geometry` transition starts when the element's laid-out box changes, which the
  connector reports by querying with the new box (S2.7). A value of type `Placement` is `{ strategy:
  PlacementStrategy, params: PlacementParams }` (S13). Every widget has `focus_visible_when:
  Value<FocusVisibleWhen>` (S2.11, ADD-A16), whose base each preset states, usually as
  `= token.behavior.focus_visible_when` (a reference, not inheritance, R20): an entry lists it only where a row states
  it. Likewise a widget's `drag_hover_delay_ms` (a drag resting on an item activating it) has its base `=
  token.behavior.spring_loading` (ADD-A45) where the platform states one spring-loading delay, and the widget's own
  value where its own source states another: one home per fact. Absence is `Value::Absent` (S2.2): a value that may be absent is a `Value<T>` listed under **may_be_absent**. A value whose
  platform may state "no limit" or "never" is an `Option` with that meaning of `None` stated at the field (S1, as
  `max_lines` and `PlacementParams.max_visible_items`), spelled `"none"` in TOML (S11), never `Absent`, which means
  only that the platform draws no such thing. A compound value listed there (a record of `Value`s such as `MessageWrap`, `Sides`, `Axes`, a `Placement`) follows
  S6.3's compound rule: each `Value` member may be `Absent`, its plain members always stated, and one declared
  `Value<T>` (a `Value<Placement>` some platform has none of at all) may also be absent as a whole; any other compound
  type with a plain member (`Transform`'s `origin`, `Corners`' `curve`) is declared `Value<T>` wherever it may be
  absent.
- **Keys:** the keys of a typed widget's `Value<Key>` value, of its keyed maps (`Vec<(Key, …)>`) and of its plain key
  lists (`Vec<Key>`, and a `Key` inside a list's entries) are declared by each preset in its `[declare.widget.<w>]`
  `keys = { <value> = [ … ] }` (S10), which S12's undeclared-name check checks a stated key against; the entry lists
  them at the value, per preset where the presets' keys differ, or says the value phase lists them from the facts. A
  key is `[a-z0-9][a-z0-9_]*` (S1): a value holding icon names, file names or other text with `-` or capitals is a
  `String`, or a `Glyph` (`GlyphSource::ThemeIcon`) where it names an icon, never a `Key`. A behaviour a
  platform does not have (no action bound to a click) is not a key: the value is listed under **may_be_absent** with
  the presets that have none.
- **Auto:** the core marks `TextStyle.line_height` and, on every part, `min_width`, `min_height` ("no minimum beyond
  the content") and `max_width`, `max_height` ("no maximum") (S2.11, S6.3); the entries do not repeat those marks, and
  a size limit the platform does not set is `auto`, never `Absent`. Any other length field that may be `Auto` (a gap,
  a width or height in a widget value, a length sized by content) says so at the field, `` `<name>_px: Value<Px>`
  (`Auto` where the platform sizes it by its content) ``; S12 allows `Auto` where the catalogue or the core marks it.
- **Units** (S1): the type carries the unit. A length is `Value<Px>`, a duration `Value<Ms>`, an angle `Value<Deg>`,
  an opacity or other fraction `Value<Fraction>`, a unitless number (a factor, a velocity) `Value<f32>`, a count
  `Value<u32>`. A widget value of one of these scalar types is written as its TOML key spells it, the unit's suffix
  added from the type (`icon_text_gap_px: Value<Px>`, `show_delay_ms: Value<Ms>`, `start_angle_deg: Value<Deg>`;
  the Rust field is `icon_text_gap`, `show_delay`, `start_angle`); a path or expression names the Rust field, without
  the suffix (`values.icon_text_gap`, S1), as the traceability does. A keyed component's values carry no suffix even
  in their declaration (*Keyed components*). A value of a compound type (`Sides<Value<Px>>`, `Axes<Value<Px>>`, a
  list, a record) carries no suffix: its members carry their units. A `PartStyle` field is named in paths as S2.11
  names it (`body.min_width`, `part.min_width`); its TOML key adds the suffix the same way (`min_width_px`).
- **Variants:** a variant is typed when two or more platforms share its meaning anywhere in the catalogue (R15; the
  typed variants are one `Variant` enum, and a widget declares the ones it has), keyed (`@name`) otherwise. A typed
  variant's meaning is the declaring widget's (`inline` is a link inside running text on `link`, a tip inside the
  content on `teaching_tip`), and a keyed `@k` and a typed `k` are different names. A content variant is a pair, a
  widget (or one of its *(element)* parts) and a variant name, set by the connector from the element's content on
  every platform (ADD-A61), so it is typed whichever presets style it, except a keyed one whose meaning only one
  platform has, set by the connector by that platform's rule under every preset and ignored where a preset does not
  declare it (S8; ADD-A123). The pairs are exactly these:
  `button`: `icon_only`, `with_icon`, and GTK's keyed `@custom_child`, `@arrow_only`, `@with_arrow` (the `button`
  entry's `adwaita` variants give the rule); `segmented_control`: `icon_only`, `with_icon`; `bottom_tab_bar.item`:
  `icon_only`, `with_icon`; `menu.item`: `icon_only`; `list.row`: `icon_only`, `with_icon`, `with_subtitle`,
  `three_line`, `with_avatar`, `with_image`, `with_video`; `breadcrumb.crumb`: `with_icon`; `chip`: `with_icon`;
  `tab`: `with_icon`; `dialog`: `with_icon`, `paged`, `about`; `checkbox`, `radio`: `with_label`; `badge`:
  `custom_color`; `toolbar`: `with_search`, `with_prompt`; `form`: `collapsed` (KDE's form layout when its wide layout
  does not fit); `form.row`: `bitmap_icon`, `activatable`; `rich_text.code_block`: `fenced`; `window`:
  `no_caption_buttons`, `print`, `page_setup`, `app_chooser`; `rubber_band`: `in_canvas`, `dock_target`; `banner`: `vertical` (the actions wrapped below the
  text, from `actions_wrap`'s outcome); `scroll_view.edge_effect`, `scroll_view.overshoot`: the edge identities `top`,
  `bottom`, `start`, `end` (one element per edge). An entry that adds one says "the content variant" at it and is
  listed here. The same name on any other widget or part is a variant the application asks for (`vertical` on
  `slider`, `bottom` on `sheet`, `activatable` on `card`), which the query answers `NotAvailable` for where the preset
  does not declare it (S8). A variant the connector sets from a size
  threshold is set by a `Breakpoint` value (Shared value types), so it may be keyed: the connector reads the variant
  from the value. Variants of which the platform draws at most one at a time (iOS's configuration styles) form an
  `exclusive_variants` group (S10, ADD-A121); others combine.
- **Sizes and densities** per platform (`ControlSize`, S7.3; Material Expressive's xsmall is `extra_small`); sizes a
  variant alone takes are declared by `sizes_by_variant` (S10: Material's FAB, MAT-N1), and a size a variant does not
  declare answers `NotAvailable`; a variant's own default size, where it differs from the widget's, by
  `default_size_by_variant` (S10, ADD-A121).
- **Flags:** the element's widget-specific flags: shared (`SharedFlag`, below), keyed (`@name`,
  `StateFlag::Widget(Key)`, declared by the preset); and the state flags it uses (S7.2).
- **Exclusive groups:** per platform, the flags of each group (the platform's state groups mapped to flags), written
  in the order of precedence the preset's `exclusive` array states (S10); that order is a fact of the platform's
  state model, which the gate checks (S17), so every entry's groups end "precedence: state models (S17)", its
  recency sets, where it has any, following. A widget's groups
  apply to each element of the widget, its *(element)* parts included (S8 drops the excluded flags per element), so
  "on `item`" names the element whose states the platform's model groups.
  The item also names the widget's **recency sets** (S10's `recency`, S7.2, R3; ADD-A124), each the flags among which
  the platform shows the one set last, written in the set's stated order (its first the latest where the connector
  reports no order), with the source of its members; a rule on the latest of a set is written `latest <set>:<flag>`
  (Selector notation).
  **Material's recency sets** (each `material-3` entry that names Compose's ripple or its elevation): Compose keeps
  each list of interactions in the order they began and shows the last. Its ripple node shows one state layer of
  hover, focus and drag at a time, the most recent (`androidx@e4d933c9
  compose/material3/material3-ripple/src/commonMain/kotlin/androidx/compose/material3/ripple/Ripple.kt:250-280`): the
  set `state_layer`, `[dragged, focus_visible, hover]`, or `[focus_visible, hover]` where the widget draws no dragged
  layer, its order `focus_visible` before `hover` as Material's View library ranks them (an Android `ColorStateList`
  takes its first matching item, and its ripple colour lists put focused before hovered,
  `material-components-android@1.14.0 lib/java/com/google/android/material/button/res/color/m3_button_ripple_color.xml:20-21`,
  `chip/res/color/m3_chip_ripple_color.xml:43-44`), `dragged` first, as a drag begins after the hover or focus it
  starts in. `pressed` is no member: Compose draws the press ripple apart from the state layer (`Ripple.kt:241-248`,
  `:365-371`), beneath it (`drawRipples()` before `drawStateLayers()`, `:373-377`), so a pressed element hovered or
  focused shows both, two states drawn together (R3). Both are the part's `interaction` block (S2.12), never a paint
  layer: the base states its alphas (the state layer's 0) and its colours `= surface.foreground`, the content colour
  that follows every rebinding (MAT-X8), each state-layer rule `state_layer.alpha`, and its `ripple.effect` the press ripple, which the
  press starts (S2.12); the state layer's colour changes at once and its alpha alone is timed, as the ripple node draws
  it in the current colour (`material3-ripple/…/Ripple.kt:373-386`), and the ripple keeps the colour of its first frame (S2.12),
  as Android's patterned `RippleDrawable` fixes it when the ripple first draws (`RippleDrawable.java:986-1018`). It draws over the part's content and beneath its
  stroke, as the ripple node and `Modifier.surface`'s border do (`Ripple.kt:373-377`, `Surface.kt:562-564`,
  `Border.kt:212-214`). Only the state layer (the `interaction` block's,
  on the part or on a widget's `state_layer` part) follows `state_layer`, and only the shadow follows `elevation`: a
  hover, focus or press that Compose draws as a `Surface`'s or clickable's ripple is the `interaction` block wherever
  the row writes it, a whole `fill` included (the year cell, `DatePicker.kt:2414-2428`, whose container follows `selected`
  alone; the time picker's selectors, `TimePicker.kt:3089-3095`, `:3018`; the toolbar's buttons, whose tokens state
  the hovered state layer, `material-web@2.5.0 tokens/versions/latest/sass/_md-comp-toolbar-vibrant.scss:48-51`), and
  is written in the block. Every other field a state changes (a label's colour, a stroke, a shape) follows its own code or
  tokens, which have no notion of order, so it is a plain `{hover}`, `{focus_visible}`, `{dragged}` or `{pressed}` rule,
  those rules combining: in the source's precedence where it has one, the later winning (the Expressive list item's
  shape, pressed over dragged over selected over focused over hovered, `ListItemDefaults.kt:1144-1152`; its colours,
  disabled over dragged over selected, `:751-772`; the split button's shape by its pressed state alone,
  `SplitButton.kt:698`, `:706`), and where the source states none (the outlined button's hovered and focused outlines,
  `_md-comp-outlined-button.scss:84`, `:100`, which Compose's border and material-web's CSS do not rank) in the row's
  order, hover, focus_visible, pressed, dragged, disabled, the later winning, the facts stating such a combination
  (`{hover, focus_visible}` on the outlined button) with that reason and the gate checking it (S17). That order is the
  plain-flag rules'. A control with an elevation shows the elevation of the
  latest of hover, focus and press (`compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Button.kt:1703-1747`,
  `FloatingActionButton.kt:1300-1347`, `ToggleButton.kt:1589-1633`; the split button's halves through the button's
  `ButtonElevation`, `SplitButton.kt:687-1294`, `:709`, `:791`, `:917`), and of drag too on cards and chips
  (`Card.kt:668-723`, `Chip.kt:3612-3666`, `:3751-3805`): the set `elevation`, `[pressed, focus_visible, hover]`, or
  `[dragged, pressed, focus_visible, hover]`, `pressed` before the others as a press begins on an element the
  pointer already hovers or the key's focus already holds, `dragged` before it as a drag begins from a press, and
  `focus_visible` before `hover` as in `state_layer`; an elevated button hovered after it was focused is at level 2,
  focused after it was hovered at level 1 (`tokens/ElevatedButtonTokens.kt:43-74`), and pressed while hovered at
  level 1. The rest elevation's shadow transitions are Compose's outgoing specs, its hovered one `from {hover}` and
  its default one `from {pressed, focus_visible}` (cards and chips: `{pressed, dragged, focus_visible}`), each state's rule states Compose's incoming spec with no
  `from`, and the `{disabled}` rule a zero duration (`internal/Elevation.kt:46-113`, `Button.kt:1750-1770`; S2.7).
  The `from {hover}` transition is stated after the other, so it wins where hover and another flag end in one update,
  where Compose takes the spec of the elevation it showed: exact on buttons, FABs and toggle buttons, where at the pin's
  tokens only the hovered elevation differs from the rest one, and on cards and chips but where a drag and a hover
  end in one update (S2.7). The state layer's own fades are chosen the same way (`material3-ripple/…/Ripple.kt:313-322`,
  `:463-483`): by the interaction entered, or, going to none, by the one left, all linear. So the rest style states the
  outgoing fade with no `from`, then the drag's outgoing one `from {dragged}` where the set holds `dragged`, both of
  the `StateLayer` group; each state-layer rule states its incoming fade; the press ripple is no transition but the
  block's `ripple.effect`, Android's patterned `RippleDrawable`, started by the press and ended by its own exit
  (S2.12); and the container's
  `Fill` takes no transition where Compose snaps it (a selected chip's, `Chip.kt:4070-4076`; a checked toggle
  button's, `ToggleButton.kt:178`, `:1446-1452`; a checked icon button's, `IconButton.kt:1428-1436`). So a toggle
  clicked while hovered snaps its container, its state layer taking the checked colour at once at its alpha
  unchanged, and its ripple running its effect on in the colour of its first frame (S2.12); a hover that ends while the press
  holds fades the state layer's alpha by the outgoing spec while the ripple holds (MAT-X9). Where Compose animates a
  container or content colour itself, the part's `Fill`, or its text or icon, take that transition (`animateColorAsState`, `animateColor`: the list item, the date picker's cells,
  the selectable menu item, the tab's and the navigation bar's content, the toggle's outline; MAT-L, MAT-N16, MAT-M1,
  MAT-TB1, MAT-N12, MAT-B12).
  Compose's buttons, toggle buttons and cards name the state left by the elevation they were heading for, the first of
  pressed, hovered, focused (and dragged) with that value (`Button.kt:1758-1764`, `ToggleButton.kt:1644-1650`,
  `Card.kt:733-740`; the FAB and the chips keep the interaction itself, `FloatingActionButton.kt:1417-1430`,
  `Chip.kt:3672-3684`), which at the pin's tokens is the flag left wherever the elevation changes, their pressed and
  focused elevations equalling the rest one. The entries' Compose citations (`Menu.kt`, `Surface.kt`, …)
  are files of `androidx@e4d933c9 compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/`.
  Compose draws no ripple state layer on the slider's handle, the carousel's items or the sheet's drag handle at that
  pin, so their `material-3` groups, from the published tokens, are exclusive groups, not these sets.
- **may_be_absent** (S6.3): `parts:` whole parts some preset does not draw, with the presets: the preset's base, or a
  rule under a condition, states the part `absent = true` (`PartPatch::Absent`), and the query answers `NotDrawn` for
  it (an absent part needs no fields); a root part is never listed. A part a platform draws with another widget is
  not absent: the base or rule states `drawn_as = { widget = "<w>", part = "<p>", variants = [ … ] }` (`part` left
  out: the whole widget; `PartPatch::DrawnAs`, S6.1, S11),
  the query answers `Styled::DrawnAs`, and the connector builds that widget's element in the part's place, in the
  host's context (Nested controls); the entry states it at the part. `fields:` part fields that may be
  `Absent`, beyond the core's own marks (S6.3: `PartStyle.touch_target`, `margin` and `shape`, `Glyph.stroke`, …),
  which the entries do not repeat. Among those marks are a part's drawing fields, `fill`, `stroke`, `shadow`,
  `text`, `icon`, `glyph`, `focus` and `cursor`, on every part of every widget and component (S6.3: a part that paints
  nothing at rest, a menu item, a flat button's or a tab's fill, states `"absent"`, and so does a preset whose
  platform draws no outline or shadow there); where an entry's `fields:` list still names one of them, the item only
  records which presets state it absent and grants nothing S6.3 does not, so its list of presets is not a limit;
  `values:` widget values that may be `Absent`; each with the presets. The list is about what a platform draws,
  never about an element's content: a part the element's content lacks (a dialog given no icon, an icon-only tab's
  label) is not queried by the connector and is not listed (S6.3); a part a preset draws whenever the content exists
  is required on it. A list value (a `Vec`: `presentations`, `breakpoints`, a map of keyed entries) is never `Absent`:
  where the platform has none it is the empty list, stated with its citation. Where an entry cannot yet name the
  presets that draw no such field, it says the value phase names them from the facts; for a field or value the entry
  itself grants (not a drawing field), the permission is then the preset's only where its facts show the platform
  draws none.
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
- Where one platform draws a host's own part with another widget (KDE's date and time triggers as a combo box or a
  spin box, its date popup as a menu, kirigami-addons' popups as a dialog), the part is `drawn_as` that widget on
  that preset (S6.1, Entry format): the query answers `DrawnAs` with the widget and its variants, and the connector
  builds that element in the host's context, with the host element's states (S8), so it needs no platform knowledge
  and the look is stated once, by that widget's rules in the host's context.
- The element's identity in its host is a variant of its widget, typed where two or more platforms share it (R15):
  the typed identities `button` lists (its entry is the list): `close`, `back`, `previous`, `next`, `overflow`, `add`,
  `remove`, `done`, `clear_field`, `reveal`, `pane_toggle`, the media identities `play`, `volume`, `full_screen`,
  `frame_step`, `route`, and an embedded window's `minimize`, `maximize`, `restore`. Its look there is that widget's
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
| `Placement` | `{ strategy: PlacementStrategy, params: PlacementParams }` (S13); listed under may_be_absent, each of its `Value` members may be absent on its own (S6.3); declared `Value<Placement>` where a platform has no such placement, and then also absent as a whole (Entry format) |
| `EdgeSide` | the core's (S2.7): `Top`, `Bottom`, `Start`, `End`, `TowardAnchor` (a presentation's slide origin) |
| `BoxEdge` | `Top`, `Bottom`, `Start`, `End` (an edge of an element's own box, where no anchor exists: a scroll bar's, an indicator's, an edge effect's; `Start` and `End` mirrored in RTL) |
| `NavButtonsVisibility` | `Always`, `OnHover` (when a widget's previous and next `button` elements show: Windows' FlipView and PipsPager, W2-31, W2-20) |
| `Breakpoint` | the core's (S7.3: its fields, its evaluation and the variant it sets), in a widget's `breakpoints: Vec<Breakpoint>`. Each entry states the platform's own comparison, never an inverted one, and where two variants hold together and conflict, which one's rules come later (S8). A control that adapts to its own width (WinUI's NavigationView, kirigami-addons' FormCard) is stretched by its parent, so it measures `Allocation` |
| `TroughClick` | `Page`, `JumpToPosition` (what a click in a slider's or a scroll bar's trough does, GN-P4-5†, R17) |
| `WidgetValue` | the core's (S1); a keyed component's value is a `Value<WidgetValue>` of the type its declaration names in S10's closed vocabulary (`values = { <name> = "<type>" }`; S10 lists the type names, among them `"enum:<Name>"` for one of the model's public enums or this table's types and `{ key = [ … ] }` for a value of the component's own whose keys its entry lists). An entry writes each value with its Rust type (`Value<Px>` is `"length"`, `Value<LineCap>` is `"enum:LineCap"`, a `Value<Key>` with listed keys is `{ key = [ … ] }`) |

## Shared flags

`SharedFlag` (S7.2) is exactly the widget flags two or more platforms draw on a typed widget; every other widget flag
is keyed. The list, with the widgets that declare each:

| Flag | Meaning | Declared by (platforms) |
|---|---|---|
| `empty` | the element holds no text or value | `input` (Material, Windows, KDE; a search field's text, an `input` in `context search_field`), `text_area`, `combo_box` (Material: no value chosen, MAT-CB1), `progress_bar` (GNOME: value zero), `date_picker` (Windows: no date chosen), `form` (GNOME's entry row) |
| `in_fill` | a tick or stop indicator inside the value fill | `slider` (KDE, Material) |
| `today` | the day is today | `calendar` (all six) |
| `other_month` | a day outside the shown month | `calendar` (GNOME, Windows, KDE, Material, macOS) |
| `current` | the current item of a sequence | `breadcrumb` (KDE, Windows, macOS, GNOME: GN-2.52†), `wizard` (GNOME's highlighted step, GN-2.51), `list` (kirigami-addons' current row), `text_area` (the current match of a find, on `find_match`: macOS, iOS) |
| `multiline` | the element's text wraps to more than one line | `badge` (KDE), `banner` (KDE, Material), `toast` (Material) |
| `filled`, `half` | a rating star filled, or half filled; a level indicator's block the value reaches (`filled`) | `rating` (KDE, Windows, macOS), `level_indicator` (`filled` on `block`: GNOME, macOS) |
| `preview` | the hover preview of a value | `rating` (KDE, Windows) |
| `pulling`, `refreshing` | the pull-to-refresh phases | `pull_to_refresh` (iOS, KDE, Material, Windows) |
| `placed_above`, `placed_below` | a popup attached above or below its anchor (set by the connector from the placement result) | `menu` (KDE; Windows' suggestion list, a `menu` in `context autocomplete`) |
| `route_active` | an output route (AirPlay) is active | `button` on its typed `route` (macOS, iOS) |
| `editing` | a row or cell in edit mode | `list` (GNOME, iOS, macOS: a cell being edited, MAC-L13, MAC-I01, MAC-X14) |
| `swiped` | a row with its swipe actions revealed | `list` (iOS, KDE) |
| `context_menu_open` | the element's context menu or popup is open | `list` (GNOME's `.has-open-popup`, macOS) |
| `sorted`, `descending` | a sorted column header, and its sort direction | `list` (`header_section`: KDE, macOS, GNOME) |
| `overwrite` | overwrite (block-caret) mode | `input`, `text_area`, `code_editor` (GNOME, KDE) |
| `composing` | an input-method composition in progress | `input`, `text_area` (KDE, GNOME) |
| `minimized` | the element in its minimised form | `bottom_tab_bar` (iOS), `sheet` (Material) |
| `placeholder` | the element shows a placeholder in place of its value (a rating with no value yet; content shown as a placeholder while it loads is the state flag `redacted`, S7.2) | `rating` (Windows, macOS) |
| `text_selected` | the element's text has a non-empty selection (the caret parts' absence rules: GNOME, Material, iOS, macOS, Windows hide the caret; KDE keeps it, K3.43); on a span element, the span's text lies in the selection | `input`, `text_area`, `code_editor` (and its `syntax_span`: KDE's per-style selected colours), `selectable_label` (GNOME) |
| `drop_copy`, `drop_move` | the operation a drop would perform, set one at a time by the connector from the drag's current operation (the other operations are keyed: Windows' `@drop_link`, iOS's `@drop_forbidden`, `@drop_cancel`) | `drag_image` (Windows' drop glyphs, W3-49†; iOS's drop badges, P3-10†) |

## Selector notation

The traceability (and "Today's fields") write a rule as `<target> @ <selector>`, the selector's parts separated by
spaces:

| Spelling | Meaning (S7) |
|---|---|
| `{a,b}` | the element's states: all of them hold together |
| `!{a}` | none of these states |
| `latest s:f` | `f` is the latest present flag of the widget's recency set `s` (`Selector.latest`, S7.2, ADD-A124): it implies `{f}`; `{a} latest s:f` holds both |
| `variant v`, `variant @k`, `variants {v,w}`, `!variant v`, `!variants {v,w}`, `w.part[v]` | the element's (or the named ancestor's) variants: `variants {v,w}` all of them, `!variants {v,w}` none of them (`SetMatch.none_of`); `@k` is keyed, and a rule names only the keyed variants its own preset declares (S12) |
| `w.part[size s]` | the named ancestor's control size (`ContextMatch.sizes`, ADD-A84) |
| `cond c`, `!cond c` | a condition (S7.1, snake_case): `window_inactive`, `contrast_high`, `scheme:dark`, `input:touch`, `idiom:pad`, `flavor:kirigami`, `preset:ios-26`, `system_setting:<key>` (ADD-A04), `container_size:sidebar:small`, `tiled:top`, `freeform`, `posture:tabletop`, `device_model:<key>` (ADD-A83) |
| `context w.part [variant v] [{a}]`, `context any w`, `context nearest {A, B} = A` | `ContextMatch`: an ancestor in the chain the `Scope` carries (S8); `AnyAncestor` or `NearestAmong`. The `variant` and `{…}` written after `context w.part` are that ancestor's; the element's own variants and states are written before the first `context` (`button.body.fill @ variant tool context toolbar.bar`). A bare `context w` (`context combo_box`, `context list variant table`) is the same match as `context any w`: an ancestor of widget `w`, whichever of its parts (`ContextMatch.target` is `WidgetOrPart::Widget(w)`, S1); `context any (surface S)` is an ancestor binding the surface set `S`, whatever its widget (`ContextMatch.surface`, ADD-A68) |
| `not_context w.part` | `Selector.not_context` |
| `pos first`, `pos last`, `pos only`, `pos parity odd`, `prev w{…}`, `next w{…}` | `PositionMatch` (parity: ADD-B06; by part: ADD-B07). A `prev` or `next` term that names a part other than the element's own counts among every sibling of that widget (`among {w}`): `prev list.row {hover}` on a `list.row_separator` is the row before it |
| `… among any`, `… among {w, v}` after a `pos` term | `PositionMatch.among`: siblings of any kind (`SiblingScope::Any`, CONTRA-3) or of the kinds named; without it, siblings of the same kind |
| `size s`, `density d`, `h_size_class c`, `v_size_class c` | `SizeMatch` (sizes `extra_small`, `mini`, `small`, `regular`, `large`, `extra_large`; per axis: ADD-A20) |
| `dir rtl` | `Direction` |
| ` / ` between alternatives | one rule per alternative; the selector parts written after the last alternative belong to every alternative, and a bare name after ` / ` continues the term before it (`variant a / b` is `variant a` / `variant b`) |

## Window and chrome

### window

*From group 1's catalogue.*

- **Kind:** typed widget: every platform draws a window; its chrome is drawn by the toolkit where the toolkit draws
  its own frame (R1.1, R13).
- **Available:** every preset.
- **Parts:**
  - `frame` — the window's outer box (the root): `fill` (the decoration's border band: KDE's Frame role, GNOME's
    non-compositing frame), `stroke` (the outline: GNOME's and KDE's outlines, each in its own position, macOS's
    outer edge line, the DWM border), `corners` (per corner, with its curve; on iOS and Android the window fills the
    display, the root's container, whose radius S4.6 routes (the connector's report, else the
    `token.params.display` keys per device model, else none): so the frame's corners are `Radius::Concentric` (each
    with its stated minimum), following the display's, and the concentric corners inside derive from them: X-CR-4,
    MAT-W5†, W-6, R25.11, ADD-A83; not element data), `shadow` (layers, or an elevation entry;
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
    (font, fill, elision), `padding`, `margin` (AdwWindowTitle's negative margins, GN-1.18; GtkAppChooserDialog's
    title box's, GN-2.57†), `min_height` (GtkAppChooserDialog's title box, `_deprecated.scss:20-24`, GN-2.57†),
    `transitions`.
  - `subtitle`, in `title_bar_content` — `text`, `padding`, `margin` (GN-1.18), `opacity`.
  - `title_icon`, in `title_bar_content` — the application icon in the title bar, or macOS's proxy (document) icon:
    `icon`, `padding`.
  - `title_status`, in `title` — text the toolkit appends to the title for a window state (macOS's "Edited"): `text`.
  - `title_chevron`, in `title` — macOS's document-title chevron shown on hover: `glyph`, `opacity`, `transitions`.
  - `caption_buttons` *(element)*, in `title_bar_content` — the group of caption buttons, an element of its own so that rules select on the
    group's hover (macOS shows all three glyphs while the group is hovered: `@ context window.caption_buttons
    {hover}`): `padding` (leading inset, vertical position). The `caption_button` elements are its children: the
    connector queries them in the scope it returns. Absent, with the buttons, under the content variant
    `no_caption_buttons` (GNOME's header bar of an assistant, GN-RL01†).
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
  - `caption_button_capabilities: Vec<(VariantRef, (WindowCapability, WhenMissing))>` (a map keyed by the button's
    identity, S6.1), `WindowCapability {
    Closable, Minimizable, Maximizable, Resizable, Sovereign, ProvidesHelp, Shadeable }`, `WhenMissing { Hide,
    Disable }`: GNOME hides the button that minimises a transient window, Windows greys the button that maximises a
    fixed-size one (W1-148†).
  - `caption_button_menus: Vec<(VariantRef, Vec<MenuEntry>)>` and `caption_button_menu_placement: Value<Placement>`:
    the menu a caption button opens (macOS's zoom-button tiling menu); its look is `menu` in `context
    window.caption_button`. Its keys the value phase lists per preset from the facts (MAC-W13† leaves macOS's item
    names to verify, per generation).
  - `title_alignment: Value<TitleAlignment>`, `TitleAlignment { Start, Center, CenterFullWidth, End }`;
    `CenterFullWidth` centres on the whole bar and falls back to the free side when it meets the buttons (KDE).
  - `title_margin_px`, `title_bar_box_gap_px`, `title_icon_gap_px`, `title_subtitle_gap_px: Value<Px>`.
  - `title_bar_click_actions: TitleBarActions { double_click: Value<Key>, middle_click: Value<Key>, right_click:
    Value<Key> }` (read): keys `menu` (open the window menu: `adwaita`'s `action-right-click-titlebar` default
    `'menu'`, GN-P3-3†; `kde-breeze`'s `CommandActiveTitlebar3` and `CommandInactiveTitlebar3` default "Operations
    menu", K-P4-1†) and, per preset, one key for each other value its read setting takes, which the value phase lists
    from the facts (no row names the double- and middle-click settings' values). A click the platform binds to no
    action is not a key: the field is `Absent` there (may_be_absent).
  - `window_menu_items: Vec<MenuEntry>` (the actions in order, with separators and submenus, K-P4-1†), `MenuEntry {
    Action(Key), Separator, Submenu(Key, Vec<MenuEntry>) }` (also `caption_button_menus`' entries); its keys:
    `kde-breeze` `maximize`, `minimize`, `keep_above`, `more_actions` (a submenu: `move`, `resize`, `keep_below`,
    `full_screen`, `no_border`, `exclude_from_capture`, a separator, `window_shortcut`, `window_rules`,
    `application_rules`), a separator, `close`, in that order (KWin's `useractions.cpp:200-298`, K-P4-1†), and the
    submenus KWin inserts only while its condition for each holds, `desktops` (several virtual desktops),
    `activities` (two or more activities), `move_to_screen` (several outputs, a movable window) and `extensions`
    (scripts' actions, just before `close`), each at the place K-P4-1† states;
    `adwaita` `restore`, `minimize`, `maximize`, `close` (GTK's own fallback menu, GN-P3-3†; the menu GNOME Shell draws
    is the shell's, R1.1); `windows-11`'s the value phase lists from the facts (W1-015 names no items).
    `window_menu_icons: Vec<(Key, Glyph)>` (the icon each action shows, each a `GlyphSource::ThemeIcon`:
    KWin's per-action theme icons, `useractions.cpp:220-296`, K-P4-1†; the keys `window_menu_items`'); `window_menu_placement:
    Value<Placement>` (at the pointer; below a button's rectangle, K-P4-1†); `title_menu_placement: Value<Placement>` (macOS's
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
  - `status_bar_hidden: Value<bool>` (whether the status bar is hidden: iOS's `prefersStatusBarHidden`, its base
    restated by a rule `@ v_size_class compact`, P4-3†); `status_bar_animation: Value<Key>` (the animation its hiding
    and showing runs, keys `none`, `fade`, `slide`: `preferredStatusBarUpdateAnimation`, P4-3†);
    `home_indicator_auto_hidden: Value<bool>` (`prefersHomeIndicatorAutoHidden`, P4-3†, W-8): values the application
    configures for the system to draw with, as `status_bar_style` (R1.1); the bars' own drawing stays the system's.
  - `default_size: Axes<Value<Px>>` (Kirigami's window, in grid units, Q1.6, its mobile size by a rule `@ cond
    mobile_form_factor`; `@ka_settings`, K-KA-41†).
  - `breakpoints: Vec<Breakpoint>` (Shared value types: the header bar treats a lower window as mobile, setting
    the keyed `@mobile`, GN-1.197: measured `Window`, height `Below` the row's threshold; libadwaita 1.10's
    `adw-header-bar.c:155-156` defines the two thresholds but no code in that tree reads them, so the value phase
    states this breakpoint only if the pinned source uses them, and the list is otherwise empty).
  - `keyboard_avoidance: Value<Placement>` (strategy `KeyboardAvoid(…)`: what moves when the on-screen keyboard covers the
    focused field, W-P4-3†, MAT-P4-2†).
  - `attention_flash: Value<AttentionFlash>`, `AttentionFlash { period_ms: Value<Ms>, count: Value<u32> }` (Windows).
  - `windowing_control_style: Value<Key>` (iPadOS 26's `UIWindowScene.WindowingControlStyle`, keys `automatic`,
    `minimal`, `unified`: the application's choice the system draws its window controls with, W-1; R1.1 puts what
    the application configures for the system in, as `status_bar_style`).
  - `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear` (the window opening and closing:
    macOS's `NSWindow.AnimationBehavior` per kind, MAC-X11; Android's activity and window animations the application
    sets, MAT-W2; iPadOS's scene dismissal `standard`, P3-5†), keyed `@title_bar_show` (the full-screen reveal),
    `@launch_exit` (the splash into the first frame), `@predictive_back` (Material's in-app back gesture),
    `@dismiss_commit` and `@dismiss_decline` (`ios-*`: `UIWindowScene.DismissalAnimation`'s `commit` and `decline`,
    the closing animations the application chooses by its intent, P3-5†).
- **Variants:**
  - on the window, typed: `dialog` (a dialog's window: GNOME's message window shadow, the close-only caption of
    GNOME and Windows, macOS alerts and panels), `utility` (Windows' tool window, macOS's utility panel: small caption
    and buttons, own radius and shadow; MAC-N16, W1-148†), `transparent_title_bar` (GNOME's `.flat` header bar,
    macOS's `titlebarAppearsTransparent`); the content variant `no_caption_buttons` (the window's content packs its
    own controls into the title bar and shows no caption buttons: GtkAssistant with `use-header-bar`, a `wizard` in
    `@header_bar` mode, GN-RL01†; set by the connector from that content on every platform, ADD-A61, and ignored by
    the presets that do not declare it, S8); the content variants `print`, `page_setup`, `app_chooser` (the window
    holds GTK's own print, page setup or app chooser dialog, GtkPrintUnixDialog, GtkPageSetupUnixDialog,
    GtkAppChooserDialog, whose window classes `print`, `pagesetup` and `appchooser` libadwaita styles, GN-2.57†: the
    flat title bar of all three, the print dialog's action box in the View set, the app chooser's shrunk title bar,
    flat search bar and content margins; set by the connector from that content on every platform, ADD-A61, and
    ignored by the presets that do not declare them, S8; `adwaita` declares them).
  - on the window, keyed: `adwaita` `@ssd`, `@ssd_frame`, `@default_decoration`, `@standalone`,
    `@collapse_spacing`, `@inline`, `@devel`, `@mobile` (declared only together with its breakpoint, which sets it,
    GN-1.197: where the value phase leaves `breakpoints` empty, `@mobile` is not declared, so S8 never treats it as
    the application's variant); `windows-11` `@tall`,
    `@collapsed`, `@titlebar_control_expanded`,
    `@mica_alt`, `@round_small`, `@compact_overlay`; `macos-*` `@hud` (the HUD panel's material, title bar and close
    button, MAC-N14), `@title_hidden`, `@no_toolbar`, `@toolbar_unified`, `@toolbar_unified_compact`,
    `@toolbar_expanded`, `@toolbar_preference`, `@separator_line`, `@separator_shadow`, `@separator_none`
    (`NSWindow.titlebarSeparatorStyle`'s explicit `.line`, `.shadow` and `.none`; the default `.automatic` is no
    variant: its look is the `title_bar` rules `@ {scrolled}`, MAC-W04); `kde-breeze` `@ka_settings`
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
  - parts: `title_bar` (`kde-breeze` by rules under `cond system_setting:hide_title_bar` and `cond full_screen`:
    KDE's `HideTitleBar`, K1.15, and a full-screen window; `adwaita` by a rule under `cond full_screen`: GtkWindow
    hides its title bar, GN-P3-5†; `windows-11` by a rule under `cond full_screen`, W-P3-2†; `ios-*`: the system draws
    the iPad's window controls and title, W-1, W-9, R1.1; `material-3` by a rule `!cond freeform`: only a
    desktop-windowing window has a caption, which the system draws with the application's title-bar fill, MAT-W4†,
    R1.1), `title_bar_content` (`ios-*`, `material-3`: the system's chrome, R1.1; `kde-breeze` under `cond
    system_setting:hide_title_bar` and `cond full_screen`, `adwaita` and `windows-11` under `cond full_screen`, each by
    a rule of its own with `title_bar`), `subtitle` (`kde-breeze`: KWin's decorations draw one caption line, no
    subtitle; `ios-*`, `material-3`: the system's chrome, R1.1; `adwaita` and `windows-11` under `cond full_screen`,
    each by a rule of its own with `title_bar`), `title` (`ios-*`, `material-3`: the system's chrome, R1.1; `macos-*`
    by a rule under `@title_hidden`: the title goes, the bar and its traffic lights stay, MAC-W14†; `kde-breeze` under
    `cond system_setting:hide_title_bar` and `cond full_screen`, `adwaita` and `windows-11` under `cond full_screen`,
    each by a rule of its own with `title_bar`), `title_icon` (`ios-*`, `material-3`: the system's chrome, R1.1;
    `macos-*` by a rule outside `cond system_setting:title_icons`, MAC-X31†; `kde-breeze` under `cond
    system_setting:hide_title_bar` and `cond full_screen`, `adwaita` and `windows-11` under `cond full_screen`, each by
    a rule of its own with `title_bar`; elsewhere it is drawn where `caption_button_layout` places it), `title_status`
    (all but `macos-*`: only AppKit appends a window state to the title, "Edited", MAC-W11†; `macos-*` by a rule
    `@ not_context window.frame {@document_edited}`: the flag is the window's, and `title_status` is queried with the
    `title` element), `title_chevron` (all but `macos-*`: AppKit's document-title chevron, MAC-W11†),
    `window_menu` (`macos-*`, `ios-*`, `material-3`: no window menu; `kde-breeze` under `cond
    system_setting:hide_title_bar` and `cond full_screen`, `adwaita` and `windows-11` under `cond full_screen`, each by
    a rule of its own with `title_bar`), `resize_zone` (`ios-*`, `material-3`: the system resizes the window, R1.1;
    `adwaita`, `kde-breeze`, `windows-11` by rules under `cond maximized`: a maximised window has no resize edge,
    GN-1.282†, K-P3-1†), `caption_buttons` (`ios-*`, `material-3`: the system's chrome, R1.1; `adwaita` by a rule under
    `no_caption_buttons`, GN-RL01†; `kde-breeze` under `cond system_setting:hide_title_bar` and `cond full_screen`,
    `adwaita` and `windows-11` under `cond full_screen`, each by a rule of its own with `title_bar`), `caption_button`
    (`ios-*`, `material-3`: the system's chrome, R1.1; `adwaita` by a rule under `no_caption_buttons`, GN-RL01†;
    `kde-breeze` under `cond system_setting:hide_title_bar` and `cond full_screen`, `adwaita` and `windows-11` under
    `cond full_screen`, each by a rule of its own with `title_bar`), `caption_button_glyph` (`ios-*`, `material-3`: the
    system's chrome, R1.1; `adwaita` by a rule under `no_caption_buttons`, GN-RL01†; `kde-breeze` under `cond
    system_setting:hide_title_bar` and `cond full_screen`, `adwaita` and `windows-11` under `cond full_screen`, each by
    a rule of its own with `title_bar`), `launch_screen` (`adwaita`, `kde-breeze`, `macos-*`: no launch screen),
    `launch_icon` (`adwaita`, `kde-breeze`, `macos-*`: no launch screen). A part nested in `title_bar` is not drawn
    with it only by a rule of its own (Nesting), which the traces state with the title bar's (K1.15, GN-P3-5†,
    W-P3-2†).
  - fields: `frame.shadow` (`ios-*`, `material-3`: the system draws any window shadow, W-2, MAT-W4†, R1.1;
    `adwaita` by rules under `cond maximized`, `cond full_screen` and `cond not_composited`, GN-1.05, GN-1.07;
    `kde-breeze` by rules under `cond maximized` and `cond tiled:<edge>`, K1.16; `windows-11` by a rule under `!cond
    system_setting:drop_shadow`, W3-53†), `frame.stroke` (`ios-*`, `material-3`: the system's, R1.1; `kde-breeze` by
    rules under `cond maximized` and `!cond system_setting:outline_enabled`, K1.9), `title_bar.stroke` (`adwaita`: its
    bottom shade line is an inset `shadow` layer, GN-1.13; `kde-breeze`, `windows-11`, `ios-*`, `material-3`: their
    title bars draw no separator line, K1.1–K1.18, W1-006–W1-013; `macos-*` by rules outside `{scrolled}` and outside
    the separator styles that draw the line, `.automatic`'s choice (measured) and `@separator_line`, so also under
    `@separator_none` and `@separator_shadow`, MAC-W04), `title_bar.shadow` (`kde-breeze`, `windows-11`, `ios-*`,
    `material-3`: their title bars draw no shade, K1.1–K1.18, W1-006–W1-013; `macos-*` by rules outside `{scrolled}`
    and outside the separator styles that draw the shade, `.automatic`'s choice (measured) and `@separator_shadow`, so
    also under `@separator_none` and `@separator_line`, MAC-W04), `caption_button_glyph.shadow` (all but `adwaita`:
    GNOME's high-contrast ring, GN-1.22; `adwaita` by a rule outside `cond contrast_high`), `caption_button_glyph.stroke`
    (all but `macos-*`: only the traffic lights have a rim, MAC-W02), `caption_button_glyph.icon` (all but
    `kde-breeze`: the window icon on its menu button, K1.6; `kde-breeze` by a rule outside `window_menu`),
    `caption_button_glyph.fill` (`windows-11`: the rectangle is `caption_button`'s own fill, W1-008; `kde-breeze` at
    rest, by a rule outside `{hover}`, `{pressed}`, `{checked}` and `cond system_setting:outline_close_button`, K1.7),
    `caption_button_glyph.corners` (`windows-11`: the rectangle is `caption_button`'s own fill, W1-008). The frame's and
    the caption group's padding is never absent: where no border band or inset exists the preset states zero.
  - values: `resize_band` (`ios-*`, `material-3`: the system resizes the window, R1.1; `adwaita` by rules under `cond
    maximized`, `cond full_screen` and `cond tiled:<edge>`, GN-1.282†; `kde-breeze` on a maximised axis, by rules
    under `cond maximized` and `cond tiled:<edge>`, K-P3-1†; `windows-11` by a rule under `cond maximized`),
    `status_bar_style` (all but `ios-*`, `material-3`: what an application configures for the system bars, W-5,
    MAT-W1), `navigation_bar_contrast_scrim` (all but `material-3`: Android's navigation bar, MAT-W1),
    `caption_bar_transparent` (all but `material-3`: Android desktop windowing's opt-in, MAT-W4†), `attention_flash`
    (all but `windows-11`: `FlashWindowEx`, W1-149†), `default_size` (`adwaita`, `macos-*`, `ios-*`, `material-3`;
    `windows-11` by its base, outside `@compact_overlay`, whose initial size W-P3-2† states; `kde-breeze` by a rule
    under `cond flavor:widgets`: a Qt Widgets window states no default size, Kirigami's window does, Q1.6), `keyboard_avoidance`
    (all but `windows-11`, `material-3`), `windowing_control_style` (all but `ios-26`: iPadOS 26's, W-1), `status_bar_hidden` (all but `ios-*`: P4-3†),
    `status_bar_animation` (all but `ios-*`: P4-3†), `home_indicator_auto_hidden` (all but `ios-*`: P4-3†, W-8);
    `caption_button_layout` (`ios-*`, `material-3`: the system's chrome, R1.1), `caption_button_gap_px` (`ios-*`,
    `material-3`: the system's chrome, R1.1), `caption_spacer_width_px` (`ios-*`, `material-3`: the system's chrome,
    R1.1; and each preset whose caption layout holds no `Spacer`: the value phase names the presets from the facts;
    `kde-breeze`'s has one, K1.4), `title_alignment` (`ios-*`, `material-3`: the system's chrome, R1.1),
    `title_margin_px` (`ios-*`, `material-3`: the system's chrome, R1.1), `title_bar_box_gap_px` (`ios-*`,
    `material-3`: the system's chrome, R1.1), `title_icon_gap_px` (`ios-*`, `material-3`: the system's chrome, R1.1),
    `title_subtitle_gap_px` (`kde-breeze`: KWin draws no subtitle; `ios-*`, `material-3`: the system's chrome, R1.1),
    `title_bar_click_actions.double_click` (`ios-*`, `material-3`: the system's chrome, R1.1; and each preset whose
    platform binds no action to a double click on the title bar: the value phase names the presets from the facts),
    `title_bar_click_actions.middle_click` (`ios-*`, `material-3`: the system's chrome, R1.1; and each preset whose
    platform binds no action to a middle click: the value phase names the presets from the facts),
    `title_bar_click_actions.right_click` (`ios-*`, `material-3`: the system's chrome, R1.1; and each preset whose
    platform binds no action to a right click: the value phase names the presets from the facts; `adwaita` and
    `kde-breeze` bind `menu`, GN-P3-3†, K-P4-1†), `full_screen_title_bar_reveal` (`ios-*`, `material-3`: the system's
    chrome, R1.1); `caption_button_menu_placement` (all but `macos-*`: the zoom button's tiling menu, MAC-W13†),
    `title_menu_placement` (all but `macos-*`: AppKit's title pop-up menu, MAC-P4-1†); `window_menu_placement`
    (`macos-*`, `ios-*`, `material-3`: no window menu). A list value (`caption_button_capabilities`,
    `caption_button_menus`, `window_menu_items`, `window_menu_icons`, `breakpoints`, `presentations`) is empty, not absent, where the
    platform has none.
- **State-model source:**
  - `windows-11`: the DWM caption buttons' states as `AppWindowTitleBar` names them (rest, hover, pressed, inactive;
    per button, with close's own), WinUI's `WindowCaptionButton` style (`generic.xaml:15697`) and the `TitleBar`
    control's visual states (`TitleBar/TitleBar.xaml`, activation and display states).
  - `adwaita`: libadwaita's `_window.scss` (backdrop, tiled, maximized, fullscreen, `.solid-csd`, `.ssd`,
    `.ssd-frame`, HC) and `_header-bar.scss` (backdrop, `.flat`, `.default-decoration`, `.collapse-spacing`, window
    controls :hover/:active), GtkWindowControls' per-window rules (`gtkwindowcontrols.c`); `_dialogs.scss`
    (the `.print` and `.pagesetup` windows) and `_deprecated.scss` (the `.appchooser` window), which select by window
    class alone (GN-2.57†).
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
  `frame.corners`; `border.shadow_enabled` → `frame.shadow` (off is `Absent`, S2.5); `border.padding_*` →
  `frame.padding`.
- **Notes:**
  - Conditions the window's rules use: `cond window_inactive`, `cond not_main` and `cond app_inactive` (macOS),
    `cond maximized`, `cond full_screen`, `cond tiled:top` and the other edges (a window maximised on one axis counts
    as tiled to its two edges, and one maximised on both axes as tiled to all four, `cond maximized` holding too, S7.1), `cond contrast_high`, `cond reduced_transparency`, `cond scheme:dark`, `cond idiom:phone`,
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
    (event `Move`, MAC-SF07†), their opening and closing (events `Appear`, `Disappear`, MAC-X32†) are `tab`'s, and
    so is what the strip does with more tabs than fit: `tab.values.overflow @ variant document context
    window.title_bar` (MAC-P4-5†: the tabs narrowed with their titles truncated, scrolling or an overflow control,
    which the value phase states from the measurement; the strip's values stay with the strip, Nested controls).
  - kirigami-addons: `@ka_settings` sets `default_size`, `content.min_width`, `content.min_height` (K-KA-41†).
  - GTK's own composite dialogs (GN-2.57†), drawn by GTK in the application's process where the application builds
    them or GtkPrintDialog finds no Print portal (`gtkprintdialog.c:1502-1508`; through the portal
    xdg-desktop-portal-gnome draws them, out by R1.1, GN-P4-3†): a `window` (variant `dialog`) whose content is a
    `dialog` element (variant `separate_window`) with standard widgets, its header bar the window's `title_bar`
    (`gtkdialog.ui:4-6`). Under `print`, `page_setup` and `app_chooser` the title bar is flat, as under
    `transparent_title_bar` (`_dialogs.scss:25-27`, `:38-42`, `_deprecated.scss:11-13`, `%headerbar-flat`,
    `_header-bar.scss:112-130`: no fill, no shade, `color: inherit`, so `title_bar.bindings` binds the `Window` set);
    under `app_chooser` it is also shrunk (`%headerbar-shrunk`, `_header-bar.scss:149-165`: `title_bar.min_height`,
    and `title_bar_content.padding`, which adds the window handle's top padding, `_deprecated.scss:16-18`, since the
    handle draws nothing between the bar and the box), and the title box after the start box (`box.start + box`:
    GtkHeaderBar's centre widget follows its start box, `gtkcenterbox.c:430`, `gtkheaderbar.c:660-671`, where the
    title widget is a box, which the value phase confirms for GtkAppChooserDialog) takes negative vertical margins
    and a minimum height (`title.margin`, `title.min_height`, `_deprecated.scss:20-24`). The rules for what the window
    holds are in those widgets' context: `dialog.action_area @ context window variant print` (the action box in the
    View set, `dialog`'s Notes), `dialog.message.padding` and `dialog.action_area.padding @ context window variant
    app_chooser`, the flat search bar `search_field.bar @ context window variant app_chooser`, and the print
    dialog's page preview, `page_preview`.

## Overlays

### dialog

*From group 1's catalogue.*

- **Kind:** typed widget: every platform (AdwDialog and AdwAlertDialog, QDialog, KMessageBox and Kirigami's dialogs,
  NSAlert and sheets, UIAlertController, Material's dialog and full-screen dialog, ContentDialog).
- **Available:** every preset.
- **Parts:**
  - `layer` — the root: the layer over the host the dialog is presented in, which draws nothing; it holds `body`,
    `scrim` and `cancel_platter`, and every other part nests in `body`: `bindings` (empty on every preset: the box
    binds the dialog's surface set, so the scrim keeps its host's).
  - `body` — the dialog box (group 7 called it `platter` for the iOS action sheet): `fill` (colour or
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
    layer header, Q2.58†; the full-screen dialog's header, MAT-N26),
    `shadow` (the full-screen dialog's on-scroll elevation, MAT-N26).
  - `title`, in `header` — `text`, `padding`.
  - `subtitle`, in `header` — `text` (Material's subhead, Kirigami's prompt subtitle).
  - `message` — the body text and content area: `text`, `padding`, `corners` (GNOME's scrolled alert body).
  - `content`, in `body` — AdwPreferencesDialog's breakpoint bin round its pages (`adw-preferences-dialog.ui`,
    GN-1.197, GN-2.10), the part its paged breakpoints measure (`Measured::Part`, S7.3; no `@bar_layout` rule sizes
    it, so the variant those breakpoints set does not feed back into the measurement): `padding`.
  - `icon` — the hero or standard icon: `icon`, `glyph`, `padding`.
  - `icon_badge` — macOS's caution badge over the application icon of a critical alert: `icon`, `padding`.
  - `action_area` — the footer or command area holding the responses: `fill` (WinUI's command area; GTK's print
    dialog's action box, GN-2.57†), `stroke` (the line above it: WinUI's separator, Kirigami's and Material's when the
    content overflows), `padding`, `min_height`, `bindings` (GTK's print dialog's action box binds the `View` set,
    `.dialog-action-box` extending `.view`, `_dialogs.scss:29-31`, `_views.scss:1-4`; it reaches the response
    `button` elements queried in it, S2.11).
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
    KDE's `KdeLayout` orders); `row_order` orders the roles it lists; responses of roles it does not list follow the
    listed ones, and responses of one role keep the order the application added them in, so a platform that keeps
    the application's order states an empty `row_order` (GNOME's, `adw-alert-dialog.c:617-682`, GN-1.284†);
    `StackedOrder { SameAsRow, Reversed, AcceptFirst }`; `StackRule { Never, WhenOverflowing { horizontal_padding_px:
    Value<Px> }, ByCount { max_in_row: Value<u32> } }`.
  - `breakpoints: Vec<Breakpoint>` (Shared value types, R16), per preset, each with the platform's own comparison:
    `adwaita` (AdwDialog's host bin, `priv->bin` in `adw-dialog.c`, which holds the floating or bottom sheet and
    fills the dialog host, so measured `Window`; not the dialog's child bin round its content; GN-SF17†): `bottom_sheet`
    with width `AtMost` and height `AtMost`, `combine` `Any` (`max-width` or `max-height`), `narrow` with width `AtMost`,
    `@short` with height `AtMost` (the `.narrow` and `.short` classes; both hold where both sizes do); `material-3`
    `full_size` (the full-screen dialog; `Window`, its comparison and width to verify, MAT-SF02†); `kde-breeze`
    `full_size` (Kirigami's full-window layer, Q2.58†, by rules under `cond mobile_form_factor`: `Window`, width
    `AtMost`, the complement of `PageRow`'s "wider than" for the in-window dialog, exact since both comparisons
    exist), `narrow` under `@ka_message_dialog` (K-KA-19†: the dialog's own width below a grid-unit width, which in
    that range is the window's width less the dialog's side margins, so measured `Window`, width `Below` the sum of
    the two, an exact restatement) and `@wide` (the host wider than the row's width, K-KA-17†, K-KA-19†: `Window`,
    width `Above`, choosing between the two width formulas of `natural_size`); `adwaita` under `paged`
    (AdwPreferencesDialog's own breakpoint bin inside its content, `adw-preferences-dialog.ui`, GN-1.197, GN-2.10):
    two breakpoints with width `AtMost` (`max-width`, `adw-preferences-dialog.c:315-338`), each setting the keyed
    `@bar_layout`, the first threshold an expression over `element.item_count` in the source's form (it grows with
    the page count, which the connector supplies as the dialog element's `item_count`, declared `data = { values =
    ["item_count"] }`, S8, S10), the second fixed; both measured `Measured::Part(dialog.content)`: the bin is the
    `content` part, whose allocation the connector measures (S7.3).
  - `narrow_when_actions_overflow: Value<bool>` (whether the connector also sets `narrow` while the action row's
    buttons, with any footer check box, do not fit in one row: kirigami-addons' message dialog, K-KA-19†; `adwaita`
    sets `narrow` by its breakpoint only, `adw-dialog.c:466-478`).
  - `window_margin: Axes<EdgeMargin>`, one per axis,
    `EdgeMargin { min_px: Value<Px>, max_px: Value<Px>, from_window_px: Value<Px>, to_window_px: Value<Px> }`: the
    margin to the host's edges, constant (Kirigami, Material) or growing with the window (AdwDialog's floating sheet);
    group 7's `screen_edge_margin` (C-11) is this value.
  - `natural_size: Axes<Value<Px>>` (AdwDialog's default, Kirigami's MenuDialog and SearchDialog).
  - `page_switcher: Value<PageSwitcher>`, `PageSwitcher { Header, BottomBar }` (where a dialog of several pages shows
    its pages' switcher: a `view_switcher` in `context dialog.header`, or a `bottom_tab_bar` at the dialog's bottom
    edge with the header showing the title; AdwPreferencesDialog's `BottomBar` under `@bar_layout`,
    `adw-preferences-dialog.ui:24-27`, GN-2.10: a value the connector reads the same way on every platform).
  - `cancel_platter_gap_px: Value<Px>` (the space between the action sheet's actions and its separate cancel box,
    C-11).
  - `placement: Placement`; `keyboard_avoidance: Value<Placement>` (strategy `KeyboardAvoid(…)`).
  - `presentations: Vec<PresentationTransition>`: events `Appear` (the action sheet's kind `Slide` from the bottom,
    C-11; a macOS sheet attaching to its window, under `@sheet`), `Disappear` (the sheet detaching); their steps
    name the parts they present, `body`, `scrim` and, under `@action_sheet`, `cancel_platter` (which lies beside the
    body and slides in with the actions, C-11), never the root, so the scrim does not move with the box.
  - `button_icons: Value<bool>` (KDE, read); `standard_button_icons: Vec<(Key, Glyph)>` (the icon each standard
    response button shows, each a `GlyphSource::ThemeIcon`, keyed by the standard button, not by
    `ActionRole`, since standard buttons of one role show different icons: Ok and Save, both accepting; Q1.110,
    K-KA-19†); its keys: `kde-breeze` `ok`, `save`, `save_all`, `open`, `yes`, `yes_to_all`,
    `no`, `no_to_all`, `abort`, `retry`, `ignore`, `close`, `cancel`, `discard`, `help`, `apply`, `reset`,
    `restore_defaults` (kirigami-addons' `MessageDialog.qml:390-407`, the names copied from `KStyle::standardIcon()`).
    A dialog type's icon (Kirigami's prompt types, KMessageBox's standard icons, NSAlert's styles) has one home,
    `icon.glyph @ variant <type>` (`info`, `warning`, `error`, `success`, `kde-breeze`'s `@question`), a
    `GlyphSource::ThemeIcon` on `kde-breeze` (Q1.116, K-KA-19†).
  - `message_wrap: MessageWrap { wrap_above_px: Value<Px>, min_width_px: Value<Px>, elide_above_px: Value<Px>,
    scroll_above_px: Value<Px>, item_list_max_width_px: Value<Px>, details_min_lines: Value<u32> }` (KMessageBox,
    K-P3-7†): the message wraps above `wrap_above_px` (`= min(mul(element.screen_width, <fraction>),
    mul(widget.message.char_width, <count>))`, the count a translated sample line), its label is at least `min_width_px`
    wide (the same form), it is elided above `elide_above_px` (`= mul(element.screen_width, <fraction>)`) and scrolls
    above `scroll_above_px` (`= mul(element.screen_height, <fraction>)`); the item list is at most
    `item_list_max_width_px` wide (`= mul(element.screen_width, <fraction>)`), the details text area at least
    `details_min_lines` lines high. The screen size is a root operand, which comes from the root's inputs through the
    scope (S8) and needs no `data` declaration; a widget value has no font of its own, so it measures the `message`
    part's (S2.2: `widget.message.char_width`), the one operand it declares (`data = { values =
    ["widget.message.char_width"] }`, S8, S10).
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
  - on the dialog, keyed: `adwaita` `@short` (set by `breakpoints`), `@wide`, `@message_dialog`, `@bar_layout` (set by
    its `breakpoints` under `paged`, GN-1.197); `kde-breeze`
    `@question`, `@menu_dialog`, `@search_dialog`, `@flat_footer`, `@wide` (set by `breakpoints`, K-KA-17†),
    `@ka_dialog` (the one surface of every kirigami-addons dialog and popup: corners, View binding, stroke, shadow,
    K-KA-18†), `@ka_form_card_dialog` (K-KA-17†), `@ka_message_dialog` (K-KA-19†), `@ka_list_dialog` (the combo
    row's list dialog, K-KA-12†); `macos-*` `@sheet`, `@picture_taker` (ImageKit's picture taker panel, MAC-P4-12†);
    `ios-*` `@action_sheet` (C-11). Material's dialog with a hero
    icon is the content variant `with_icon` (ADD-A61), which the connector sets from the content; so is `paged`, a
    dialog holding several pages and their switcher (AdwPreferencesDialog, GN-2.10; KPageDialog, K2.14); so is `about`, a
    dialog hosting an `about_dialog` (kirigami-addons' About page opens in its own square dialog layer, K-KA-44†),
    set on every platform and ignored where undeclared (S8).
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
  - parts: `scrim` (`kde-breeze` by its base, outside `cond flavor:kirigami`: a Qt Widgets dialog is a window of its
    own, Q1.107; `macos-*` by a rule outside `@sheet`: an app-modal alert dims nothing, MAC-D03), `header`
    (`kde-breeze` by a rule under `separate_window` with `cond flavor:kirigami`: a dialog in its own window draws its
    title in the window's title bar, Q1.107), `title` (`kde-breeze` by rules under `separate_window` with `cond
    flavor:kirigami`, Q1.107, and under `cond flavor:widgets`, where every dialog, a QDialog or KMessageBox, is a
    window of its own and draws its title in the window's title bar, Q1.107; nested in `header`, it is absent with it
    by a rule of its own, Nesting; under `cond flavor:widgets` the header stays drawn, as K1.222's Header-filled tools
    area `{@has_toolbar}` and otherwise as the top separator line, `header.stroke`), `subtitle` (`adwaita`,
    `macos-*`, `ios-*`, `windows-11`: AdwAlertDialog, NSAlert, UIAlertController and ContentDialog have a heading and
    a body, no subtitle; only Material's and Kirigami's dialogs draw one, MAT-DG7, Q1.116; `kde-breeze` by rules under
    `separate_window` with `cond flavor:kirigami` and under `cond flavor:widgets`, as `title`, Q1.107), `content` (all
    but `adwaita`: only AdwPreferencesDialog measures its pages in a breakpoint bin of its own, GN-2.10; `adwaita` by a
    rule outside `paged`), `icon_badge` (all but `macos-*`: only NSAlert badges its icon, MAC-D02; `macos-*` by a rule
    outside `error`, NSAlert's critical style, MAC-D02), `separator` (all but `ios-13`: iOS 13–18's alerts and action
    sheets draw hairlines between their action cells, DL-5, C-11; iOS 26's alerts draw separate capsule buttons with
    gaps, DL-5), `cancel_platter` (all but `ios-13`: iOS 26's action sheet is anchored to its source, C-11; `ios-13` by
    a rule outside `@action_sheet`, C-11).
  - fields: `action_area.fill` (all but `windows-11`, `material-3`, `adwaita`: only WinUI's command area, Material's
    full-screen dialog and GTK's print dialog fill the footer, W1-126, MAT-N26, GN-2.57†; `material-3` by a rule
    outside `full_size`; `adwaita` by its base, outside the rule `context window variant print`),
    `action_area.stroke` (`adwaita`, `macos-*`, `ios-26`: no line above the responses; `ios-13` by a rule outside
    `alert`, DL-5; `kde-breeze`, `material-3` by rules outside `{overflowing}`, Q1.109, MAT-DG4), `header.fill` (all
    but `kde-breeze`, `material-3`: only KDE's tools area and Material's full-screen dialog fill the header;
    `kde-breeze` by a rule outside `{@has_toolbar}`, K1.222; `material-3` by a rule outside `full_size`, MAT-N26),
    `header.stroke` (`adwaita`, `macos-*`, `ios-*`, `windows-11`: no line under the title; `kde-breeze` by a rule
    under `{@has_toolbar}`, K1.222, and, under `cond flavor:kirigami`, outside `{overflowing}`, Q1.114; `material-3` by
    a rule outside `{scrolled}`, MAT-DG4), `header.shadow` (all but `material-3`: only the full-screen dialog lifts its
    header on scroll, MAT-N26; `material-3` by a rule outside `{scrolled}` under `full_size`), `body.stroke` (`ios-*`,
    `macos-*`, `material-3`: iOS's alerts, NSAlert, Material's dialogs draw no outline; `kde-breeze` by a rule under
    `separate_window` with `cond flavor:kirigami`, Q1.107), `body.shadow` (`adwaita`, `kde-breeze`, `macos-*` by rules
    under `separate_window`: a dialog in its own window, whose window casts the shadow, GN-SF20†, Q1.107; `kde-breeze`,
    `material-3`, `windows-11` by rules under `full_size`: a dialog filling the window, Q2.58†, MAT-N26, W1-155†),
    `icon.glyph` (`material-3`: its hero icon is an `icon`, MAT-DG2; the value phase names the other presets that draw
    the icon as an `icon` from the facts), `icon.icon` (the value phase names the presets that draw the icon as a
    `glyph` from the facts; `kde-breeze`'s is a theme-icon glyph, K1.224†, Q1.116).
  - values: `message_wrap` (every member: all but `kde-breeze`: only KMessageBox wraps, elides and scrolls its message
    by these thresholds, K-P3-7†; `kde-breeze` by rules outside `alert` and under `cond flavor:kirigami`),
    `natural_size` (every member: `windows-11`, `ios-*`, `material-3`; `macos-*` by a rule outside `@sheet`,
    MAC-SF10†; `kde-breeze` by a rule outside the variants and contexts whose rules state it, Q1.117, Q1.118, Q2.58†,
    K-KA-12†, K-KA-17†, K-KA-19†, K-KA-31†, K-KA-44†), `page_switcher` (all but `adwaita`: its preferences dialog alone
    moves its switcher, GN-2.10), `keyboard_avoidance` (`adwaita`, `kde-breeze`, `macos-*`), `cancel_platter_gap_px`
    (all but `ios-13`: iOS 26's action sheet is anchored to its source, C-11; `ios-13` by a rule outside
    `@action_sheet`, C-11), `window_margin` (every member: `windows-11`; `kde-breeze` under `cond flavor:widgets`;
    `macos-tahoe` outside `@sheet`; `macos-monterey` everywhere, the floating inset sheet being macOS 26's, MAC-D03;
    `ios-*` outside `@action_sheet`: where the dialog keeps no margin to a host's edges); of its members,
    `from_window_px` (`kde-breeze` under `cond flavor:kirigami`, `material-3`: the margin is constant, `min_px` equal
    to `max_px`, Q1.108, MAT-DG8) and `to_window_px` (`kde-breeze` under `cond flavor:kirigami`, `material-3`: the
    margin is constant, Q1.108, MAT-DG8); the value phase confirms each from the facts. The `Value<bool>` values
    (`button_icons`, `narrow_when_actions_overflow`) are never absent: a platform that does not do it states false.
    The list values (`breakpoints`, `presentations`, `standard_button_icons`) are empty, not absent,
    where the platform has none.
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
    GtkDialog's action area: `action_area.padding`, `values.action_layout.row_gap_px` (`.dialog-action-area`,
    `_dialogs.scss:1-4`, GN-2.57†).
  - GTK's own composite dialogs (GN-2.57†) are this widget in a `window` with the content variant `print`,
    `page_setup` or `app_chooser` (the `window` entry's Notes): the print dialog's action box is `action_area @
    context window variant print` (`fill` and `bindings`, the `View` set, `_dialogs.scss:29-31`); the app chooser's
    content margins are `message.padding @ context window variant app_chooser` (the box GtkAppChooserDialog packs in
    its content area, `.dialog-vbox > box > box:not(.dialog-action-area)`, `_deprecated.scss:37-39`,
    `gtkappchooserdialog.ui:29-44`), its action area's top edge `action_area.padding` there (`_deprecated.scss:41-43`),
    and its search bar a `search_field` whose `bar` is flat in that context (`_deprecated.scss:27-35`, the `search_field`
    entry's Notes); the print dialog's page preview is the typed `page_preview`.
  - kirigami-addons: `@ka_form_card_dialog` with `@has_cancel` (K-KA-17†); `@ka_message_dialog`, its icon above the
    text under `narrow` (K-KA-19†); `@ka_list_dialog` (K-KA-12†).

### popover

*From group 1's catalogue.*

- **Kind:** typed widget: GtkPopover, qqc2 Popup, Breeze popups, NSPopover, UIPopoverPresentationController, WinUI
  Flyout, Material's menu container used as a popover surface.
- **Available:** every preset.
- **Parts:**
  - `layer` — the root: the layer over the window the popover is presented in, which draws nothing; it holds `body`
    and `dismiss_overlay`, and `arrow` nests in `body`: `bindings` (empty on every preset: the box binds the
    popover's surface set).
  - `body` — the popover's box: `fill` (colour or material: acrylic, `.popover`, glass), `stroke`, `corners`, `shadow` (layers
    or an elevation entry; Material's surface tint is an appended fill layer, R8), `padding`, `min_width`,
    `max_width`, `min_height`, `max_height`, `text` (today's `font`),
    `bindings` (`Surface(Popover)`; on `ios-*` the `Popover` set, whose background roles the preset states as the
    elevated values, with a `Binding::Role` from each keyed background role to its elevated twin, as `dialog.body`:
    R2.3, R5.2, D-14), `opacity` (kirigami-addons' translucent media popup, K-KA-35†), `transitions`.
  - `arrow`, in `body` — the tail: `min_width` (base), `min_height` (height), `corners` (tip rounding), `fill`,
    `stroke`.
  - `dismiss_overlay` — WinUI's light-dismiss overlay, beside the box: `fill`.
- **Widget values:** `placement: Placement` (gap to the anchor, screen-edge margin, arrow side; Q1.16, MAC-I06);
  `default_size: Axes<Value<Px>>` (UIKit's size when the application states
  none); `full_size_content: Value<bool>` (macOS); `presentations: Vec<PresentationTransition>`: events `Appear`,
  `Disappear` (their steps name the parts they present, `body` and `dismiss_overlay`, never the root).
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
  - parts: `arrow` (`kde-breeze`, `windows-11`, `material-3`: their popups draw no tail; `macos-*` by a rule under
    `@detached`, MAC-PO04; `adwaita`, `macos-*`, `ios-*` by a rule in any context where the platform draws a popover
    without its arrow, which the connector learns from `NotDrawn`: the value phase names those contexts from the
    facts; a popover the application builds without an arrow, GtkPopover's `has-arrow` off, does not query it,
    S6.3), `dismiss_overlay` (all but `windows-11`: WinUI's light-dismiss overlay, W1-116, W3-11).
  - fields: `body.shadow` (`adwaita`, `kde-breeze` by rules under `cond not_composited`: no shadow without an alpha
    channel, GN-1.183, K1.191), `body.stroke` (`material-3`, `ios-*`: the platform outlines no popover), `arrow.stroke`
    (the value phase names the presets whose tail has no outline of its own from the facts; `adwaita` draws one,
    GN-1.180).
  - values: `default_size` (every member: all but `ios-*`: UIKit's size when the application gives none, PO-7†).
    The `Value<bool>` value (`full_size_content`) is never absent: a platform without a full-size content view states
    false.
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
- **Available:** every preset but `ios-*` (iOS and iPadOS draw no tooltip: `UIToolTipInteraction` shows one only for
  an app running on a Mac, where the tip is macOS's, TT-1).
- **Parts:**
  - `body` — the root: `fill` (colour or material: in-app acrylic, `.toolTip`), `stroke` (KDE's border mixed from
    background and text by the frame contrast: Breeze's `kcu_mix`, under `cond flavor:kirigami` qqc2's `kirigami_lerp`,
    Q1.38, Q3.15), `corners`, `shadow`, `padding`, `min_height` (Material's, MAT-T5; `windows-11`'s base
    `auto`, the annotated scroll bar's tooltip's minimum stated by a rule `@ context scrollbar.body variant
    @annotated`, W2-28; `auto` on the other presets, whose platforms set no minimum, never `Absent`), `max_width` (pixels, or
    characters of the label's font as `mul(widget.label.char_width, <count>)`: `body` has no font of its own, S2.2),
    `bindings` (`Surface(Tooltip)`, Kirigami's `Complementary`).
  - `label` — `text` (font, tracking, line height, wrap).
  - `title` — the rich tooltip's subhead: `text`.
  - `icon` — the tooltip's icon (GNOME): `icon`.
  - `accelerator` — the shortcut shown in a tooltip (W3-26): `text`.
  - `arrow` — Material's caret: `min_width`, `min_height`, `glyph`, `corners`, `fill` (all listed; a preset states
    the shape its platform draws, the glyph or the cornered fill, and the other `Absent` where `may_be_absent`
    allows).
- **Widget values:** `show_delay_ms: Value<Ms>` (today; Kirigami's under `cond flavor:kirigami`, Q1.136, Q1.65,
  Q1.82), `reshow_delay_ms: Value<Ms>` (browse mode), `browse_timeout_ms: Value<Ms>`, `hide_delay_ms: Value<Ms>`
  (after the pointer leaves, MAT-X25), `keyboard_show_delay_ms: Value<Ms>`; `show_on_press: Value<bool>` (tablet mode, touch; with `show_delay_ms` `=
  token.behavior.long_press` under that condition, Q3.28); `display_time: Option<DisplayTime>`, `DisplayTime { base_ms: Value<Ms>,
  per_char_ms: Value<Ms>, free_chars: Value<u32> }` (`None`, `"none"`: the tooltip never times out while hovered);
  `icon_text_gap_px: Value<Px>`; `placement: Placement`; `presentations: Vec<PresentationTransition>`: events
  `Appear`, `Disappear`.
- **Variants:** typed `rich` (Material's rich tooltip, KDE's KToolTipWidget); keyed `macos-*` `@expansion`.
- **Sizes and densities:** none.
- **Flags:** none (a tooltip has no interaction state; the rich tooltip's actions are `button` elements in `context
  tooltip.body variant rich`).
- **Exclusive groups:** none (the tooltip holds no interaction state); precedence: state models (S17).
- **may_be_absent:**
  - parts: `title` (`adwaita`, `macos-*`, `windows-11`: their tooltips have no subhead; `kde-breeze` by a rule outside
    `rich`), `icon` (all but `adwaita`: only GTK's tooltip draws an icon beside its label, GN-1.81), `arrow` (all but
    `material-3`: only Material's tooltip has a caret, MAT-T4), `accelerator` (all but `windows-11`: only WinUI's
    tooltip shows the shortcut, W3-26).
  - fields: `body.stroke` (`material-3`: the platform outlines no tooltip), `body.shadow` (`adwaita`,
    `kde-breeze` by rules under `cond not_composited`: no shadow without an alpha channel, K1.82), `arrow.corners`
    (`material-3`: its caret is a `glyph`, MAT-T4).
  - values: `icon_text_gap_px` (all but `adwaita`: only GTK's tooltip draws an icon, GN-1.81), `reshow_delay_ms`
    (the value phase names the presets among `kde-breeze`, `material-3` from the facts; `adwaita`, `macos-*` and
    `windows-11` state one, GN-1.82, MAC-T04, W1-059), `browse_timeout_ms` (the value phase names the presets among
    `macos-*`, `windows-11`, `material-3` from the facts; `adwaita` and `kde-breeze` state one, GN-1.82, K1.83†),
    `keyboard_show_delay_ms` (the value phase names the presets among `adwaita`, `kde-breeze`, `macos-*`, `material-3`
    from the facts; `windows-11` states one, W1-059).
- **State-model source:** `adwaita`: `_tooltip.scss` and `gtktooltip.c` (browse mode); `kde-breeze`: Breeze's
  `PE_PanelTipLabel`, qtbase `qtooltip.cpp`, qqc2's `ToolTip.qml`; `macos-*`: tool-tip and expansion tool-tip
  behaviour (measured); `windows-11`: ToolTip's visual states and `ToolTipService` timings; `material-3`: the plain
  and rich tooltip tokens and Compose `TooltipBox`.
- **Today's fields:** `background_color` → `body.fill`; `font` → `label.text.font`; `max_width_px` →
  `body.max_width`; `show_delay_ms` → `values.show_delay`; `border.*` → `body.stroke`, `body.corners`,
  `body.padding`, `body.shadow`.
- **Notes:**
  - Groups 2 and 4: Kirigami's tooltips take `show_delay_ms` under `cond flavor:kirigami` (Q1.136, Q1.65, Q1.82),
    also in `context sidebar.pane variant compact`.
  - Group 3: the shortcut tooltip of an action field is this widget in `context input` (Q1.18).
  - `body.max_width` in characters names `widget.label.char_width` (the label's font, S2.2), which `body` declares as
    its data (`data = { body = ["widget.label.char_width"] }`, S8). A maximum width with no limit is `"auto"`, never
    absent (S2.11): `kde-breeze`'s plain-text tooltip under `cond flavor:widgets` states `"auto"` (K1.83†).

### menu

*From group 1's catalogue.*

- **Kind:** typed widget: popover menus and context menus (GtkPopoverMenu, QMenu, qqc2 Menu, kirigami-addons'
  ConvergentContextMenu, NSMenu, UIMenu, MenuFlyout, Material's menu and Compose's text context menu), and the popup
  lists that are menus to the user (Nested controls): a combo box's popup (`context combo_box`), a field's
  suggestion list (`context autocomplete`), an edit menu's overflow (`context edit_menu`), a toolbar's overflow
  menu. The menu bar is `menu_bar`.
- **Available:** every preset.
- **Parts:**
  - `layer` — the root: the layer over the window the menu is presented in, which draws nothing; it holds `body`
    and `dismiss_overlay`, and every other part nests in `body`: `bindings` (empty on every preset).
  - `body` — the popup's box: `fill` (colour, translucency, material: acrylic, KWin blur, glass), `stroke`,
    `corners` (K1.64: square without an alpha channel, joined to its menu bar or combo box), `shadow`, `padding` (the
    inset of items from the popup's edges), `min_width`, `max_width`, `min_height`, `max_height`, `bindings` (the
    menu's surface set where the platform states one; on `ios-*` no elevated set: UIMenu's box is drawn with a material, not
    the elevated background level that `dialog`, `sheet` and `popover` bind, D-14; the value phase confirms it from
    the MN rows' measurements), `transitions`.
  - `item` *(element)*, in `group` — one row: `fill` (highlight; libadwaita's and the tokens' state layers, appended paint layers; Material's are `interaction`), `stroke` (KDE's outline),
    `corners` (the highlight's own radius), `shadow` (GNOME's HC inset ring), `padding`, `margin`
    (the space
    round each item, WinUI's `MenuFlyoutItemMargin`, W1-045), `min_width`, `min_height`, `opacity`, `focus`,
    `transitions`, `interaction` (Material's state layer and press ripple, S2.12). `item_label`, `item_subtitle`, `item_icon`, `item_trailing_icon`, `accelerator`, `check_indicator`,
    `submenu_arrow`, `split_chevron`, `split_divider`, `item_selection_indicator`, `found_item_pointer` belong to
    `item`, nest in it and are queried with it (`split_chevron` as an element of its own inside it).
  - `item_label` — `text` (font, tracking, line height, fill per state), `stroke` (KDE's weak-focus line, drawn a gap
    under the text's own bounding box, not the item's: its bottom edge, position `Outside`, the gap its `outset`,
    `breezehelper.cpp:406-421`, K1.62, K3.21; the other edges `Absent`).
  - `item_subtitle` — `text` (iOS, macOS 14.4+, Material's supporting text).
  - `item_icon` — the leading icon: `icon` (size, fill per state, KDE's selected-mode effect), `opacity`.
  - `item_trailing_icon` — iOS's item image, Material's trailing icon: `icon`.
  - `accelerator` — the shortcut text: `text`, `opacity`, `padding` (W3-26).
  - `check_indicator` — the check, radio or mixed mark and its column, on every preset (Nested controls: it takes
    the item's states): `glyph` (the mark), `fill`, `stroke`, `corners`, `shadow` (the box or ring behind the mark:
    GNOME's radio ring; Breeze draws it with its check box's and radio button's look, K1.69, Q1.31), `min_width`,
    `min_height`, `padding`, `margin` (GNOME's margins, one negative, GN-1.73).
  - `submenu_arrow` — `glyph`, `opacity`, `min_width` (its box).
  - `separator`, in `group` — the separator item: `stroke` (the line: its width and paint, today's `separator_color`;
    the top edge), `margin` (the line's insets from its ends, Entry format), `min_height` (the item's height).
  - `group_separator` — iOS's thick separator between inline sections: `fill`, `min_height`.
  - `group` — a group of items: Material Expressive's item group box (`fill`, `corners`, `padding`); `item`,
    `separator` and `section_header` nest in it, and where no group is drawn (every other preset, and Expressive's
    ungrouped menus) they are queried in `body`'s scope (Nesting).
  - `section_header`, in `group` — KDE's section item, macOS's section header, iOS's menu title, Material Expressive's
    section label: `text`, `stroke` (KDE's line after the text), `padding`, `min_height`.
  - `submenu_header` — GNOME's sliding-submenu header: `glyph` (back arrow), `text`, `padding`.
  - `scroll_arrow` *(element)* — the scroll areas of a menu taller than its space: `glyph`, `fill`, `min_height`.
  - `split_chevron` *(element)*, in `item` — WinUI's split menu item (`@split`, W1-051): the chevron part pressed apart from
    the item, a piece of the item, not a `split_button`: `fill` (its own hover and pressed highlight), `glyph` (the
    chevron), `min_width` (the chevron part's box).
  - `split_divider` — the line between the item and its chevron part (W1-051): `stroke` (the divider's brush and
    width; the start edge), `min_height` (its length).
  - `item_selection_indicator` — WinUI's selection pill on the selected item of a combo box's list (W1-135), in
    `context combo_box`: `fill`, `corners`, `min_width`, `min_height`, `transform` (its scale per state while drawn),
    `transitions` (its scale animation; absent outside `{selected}`, it enters from transparent by these, S2.7).
  - `found_item_pointer` — the pointer macOS draws at a menu item the Help search found (MAC-N33†): `glyph`, `fill`,
    `transitions`.
  - `dismiss_overlay` — WinUI's light-dismiss overlay, beside the box: `fill`.
  - `arrow` — the tail of a popover menu drawn with one (GTK's popover menus; absent by `adwaita`'s rules in the
    contexts where GTK draws its menu without one: a back button's navigation menu, a drop-down's list, the text
    context menu, the window menu, GN-1.278†, GN-P4-2†, GN-P4-8†, GN-P3-3†): `min_width`, `min_height`, `corners`,
    `fill`, `stroke`, as `popover.arrow`.
- **Widget values:** `submenu_open_delay_ms: Value<Ms>`
  (W3-41, MAT-X25, MAC-X12), `submenu_close_delay_ms: Value<Ms>`;
  `submenu_switch: Value<SubmenuSwitch>`, `SubmenuSwitch { Immediate, TimerRestartedOnMotion, CloseTimer,
  SafeTriangle }` (GNOME's timer restarted on motion, KDE's and Windows' close timeout, macOS's triangle);
  `submenu_presentation: Value<SubmenuPresentation>`, `SubmenuPresentation { Cascade, Slide, Inline }`;
  `submenu_placement: Placement`; `placement: Placement` (a menu the application opens at an anchor or the pointer,
  a context menu; a nested menu is placed by its host's value, Nested controls); `check_indicator_side: Value<Side>`
  (the check before or
  after the label: GNOME's combo-box list puts it after, GN-1.234); `presentations: Vec<PresentationTransition>`
  (queried with the menu; the steps name the parts they present, `body` and `dismiss_overlay`, never the root):
  events `Appear`, `Disappear`, keyed `@submenu_push` and `@submenu_pop` (a sliding
  submenu entering and leaving, as `sidebar`'s), `@submenu_expand` (an inline submenu opening in place);
  `convergent_variant: Option<Value<Key>>`
  (the keyed variant the connector sets on a menu the application builds as a convergent one and gives no display
  form of its own: kirigami-addons' ConvergentContextMenu, whose `displayMode` defaults to the bottom drawer on a
  mobile form factor and to the menu elsewhere, `ConvergentContextMenu.qml:200`, K-KA-31†): keys `ka_bottom_drawer`,
  `ka_dialog` (`kde-breeze`'s keyed variants `@ka_bottom_drawer`, `@ka_dialog`, flavour `kirigami`'s); `None`
  (`"none"`) has one meaning on every preset, the connector sets no variant and the menu keeps its own look:
  `kde-breeze` states `"none"` and `ka_bottom_drawer` by a rule `@ cond mobile_form_factor cond flavor:kirigami`,
  every other preset `"none"`;
  `show_item_icons: Value<ItemIcons>`, `ItemIcons {
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
  - On `material-3` the Expressive menus' variants, `horizontal`, `@vibrant` and the item's `icon_only` (MAT-M10), are
    flavour `expressive`'s, declared under `[declare.flavor.expressive.widget.menu]` only: under `flavor:baseline` the
    query answers `NotAvailable` for `horizontal` and `@vibrant`, variants the application asks for, and ignores the
    content variant `icon_only` (S8, R2.7, R15).
  - On `kde-breeze` the menu's `@ka_bottom_drawer` and `@ka_dialog` are flavour `kirigami`'s, declared under
    `[declare.flavor.kirigami.widget.menu]` only: the widgets they substitute (`sheet`, `form`) are flavour
    `kirigami` only, so under `flavor:widgets` the query answers `NotAvailable` for a menu asking for them (S8, R2.7).
- **Sizes and densities:** `ios-*`: the element sizes on `item`: `small`, `regular` (medium), `large`. `macos-*`: none
  of its own: a pop-up button's menu follows the button's control size by rules on the ancestor's size,
  `@ context combo_box.body[size s]` (`ContextMatch.sizes`, MAC-MN15, ADD-A84). No density.
- **Flags:** shared `placed_above`, `placed_below` (`kde-breeze`, `windows-11`, on the menu, set by the connector from
  the placement result: Breeze joins the popup's edge to its menu bar or combo box, K1.64; WinUI squares the
  suggestion list's attached corners, W2-15); keyed none; state flags used: hover (the item under the pointer),
  focused (the item the keyboard highlights: GTK's `:selected` model button, GN-1.67; Qt's active action; NSMenu's and
  UIMenu's highlighted item), pressed, selected (only the chosen value of a combo box's list, `context combo_box`:
  WinUI's ComboBoxItem `Selected`, Material's selected menu item; S7.2: a menu item's highlight is `focused` or
  `hover`, never `selected`), checked, indeterminate, open (submenu shown), disabled, focus_visible, opened_by_touch
  (on the menu).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` on `item` (MenuFlyoutItem's CommonStates;
  CheckStates is `checked`, SubMenuOpened is `open`, PaddingSizeStates is `opened_by_touch`, none of them a group);
  others none (states combine; `material-3`'s state layers are a recency set, below); precedence: state models (S17).
  Recency sets: `material-3`: `state_layer = [focus_visible, hover]` on `item` (Material's recency sets, above:
  Compose's ripple, `Menu.kt:2155-2159`; `pressed` combines; the menu tokens state no dragged layer).
- **may_be_absent:**
  - parts: `item_label` (`kde-breeze` by rules under `@ka_bottom_drawer` and `@ka_dialog`: the form row's own title
    takes its place, K-KA-31†), `item_subtitle` (`kde-breeze`, `windows-11`: QMenu, qqc2's Menu and MenuFlyout draw no
    subtitle; `adwaita` by its base, outside the completion list in `context autocomplete context code_editor`,
    GN-SF13†), `item_icon` (`kde-breeze` by rules under `@ka_bottom_drawer` and `@ka_dialog`: the form row's own icon
    takes its place, K-KA-31†), `item_trailing_icon` (all but `ios-*`, `material-3`: UIMenu's item image and
    Material's trailing icon, MN-5, MAT-M3), `accelerator` (`kde-breeze` by rules under `@ka_bottom_drawer` and
    `@ka_dialog`, whose form rows show no shortcut, K-KA-31†), `submenu_arrow` (`kde-breeze` by rules under
    `@ka_bottom_drawer` and `@ka_dialog`: the form row's arrow, `form.row_arrow`, takes its place, K-KA-31†),
    `group_separator` (all but `ios-*`: UIMenu's thick separator between inline sections, MN-9), `group` (all but
    `material-3`: only Material 3 Expressive groups its items, MAT-M10; `material-3` by a rule under `cond
    flavor:baseline`), `section_header` (`windows-11`: MenuFlyout has no header item, its items being MenuFlyoutItem,
    its toggle, radio and sub-item kinds and the separator; `material-3` by a rule under `cond flavor:baseline`: only
    Expressive labels its sections, MAT-M10; `kde-breeze` by rules under `@ka_bottom_drawer` and `@ka_dialog`,
    K-KA-31†; `adwaita` is not listed: the value phase confirms GTK's section labels, `gtkmenusectionbox.c`, from the
    facts, no GNOME row stating them yet), `submenu_header` (all but `adwaita`: GTK's sliding submenu alone has a
    header, GN-1.75), `scroll_arrow` (`adwaita`, `ios-*`, `material-3`, `windows-11`; `kde-breeze` by rules under
    `@ka_bottom_drawer` and `@ka_dialog`, K-KA-31†), `split_chevron` (all but `windows-11`: WinUI's split menu item,
    W1-051), `split_divider` (all but `windows-11`: WinUI's split menu item, W1-051), `item_selection_indicator` (all
    but `windows-11`: WinUI's selection pill, W1-135; `windows-11` by a rule outside `{selected}`), `dismiss_overlay`
    (all but `windows-11`: WinUI's light-dismiss overlay, W1-055, W3-11), `arrow` (all but `adwaita`: only GTK's
    popover menus draw a tail; `adwaita` by rules in the contexts where GTK draws its menu without one, GN-1.278†,
    GN-P3-3†, GN-P4-2†, GN-P4-8†), `found_item_pointer` (all but `macos-*`: the pointer at a menu item the Help search
    found, MAC-N33†). Under `@ka_bottom_drawer` and `@ka_dialog` the box and the parts that stay are drawn as the
    sheet's, the dialog's and the form's (`drawn_as`, Notes).
  - fields: `item.stroke` (`ios-*`, `material-3`: no outline round an item; `macos-*` by a rule outside `{checked}` on
    `palette`, the selection ring, MAC-MN13; `windows-11` by a rule outside `context combo_box`, W1-134; `adwaita` by
    a rule outside `circular`, GN-1.78; `kde-breeze` by a rule outside `{hover}` and `{focused}` under `cond
    system_setting:menu_item_strong_focus`, K1.60), `item.shadow` (all but `adwaita`: GNOME's high-contrast inset
    ring, GN-1.80; `adwaita` by a rule outside `cond contrast_high`), `check_indicator.fill` (all but `kde-breeze`:
    Breeze alone draws a box behind the mark, its check box's and radio button's, K1.69, Q1.31; GTK's check items have
    no fill, GN-1.73), `check_indicator.stroke` (all but `adwaita`, `kde-breeze`: the mark alone; `adwaita` by a rule
    outside `radio`, whose ring alone is outlined, GN-1.73), `check_indicator.corners` (all but `adwaita`,
    `kde-breeze`: the mark alone; `adwaita` by a rule outside `radio`, GN-1.73), `check_indicator.shadow` (all but
    `kde-breeze`: Breeze alone draws a box behind the mark, K1.69, Q1.31), `body.shadow` (`kde-breeze` by a rule
    under `cond not_composited`: Breeze's menus without an alpha channel, K1.64; `adwaita` by a rule under `cond
    not_composited`, as its popovers, GN-1.183; `windows-11` by a rule under `!cond system_setting:drop_shadow`,
    W3-53†), `body.stroke` (`material-3`, `ios-*`: the platform outlines no menu).
  - values: `palette_columns` (all but `macos-*`, `ios-*`: AppKit's and UIKit's palette menus, MAC-MN13, MN-12),
    `item_order` (all but `ios-*`: UIKit's element order, P4-12†), `indent_step_px` (the value phase names the
    presets whose platform indents no menu item from the facts; `macos-*` states it, MAC-MN11), `group_gap_px` (all
    but `material-3`: only Material 3 Expressive groups its items, MAT-M10; `material-3` by a rule under `cond
    flavor:baseline`), `activation_flash` (all but `macos-*`: NSMenu's selection flash, MAC-X12).
- **State-model source:** `windows-11`: MenuFlyoutItem's, ToggleMenuFlyoutItem's, RadioMenuFlyoutItem's and
  MenuFlyoutSubItem's visual state groups (CommonStates, CheckStates, PaddingSizeStates, SubMenuOpened;
  `MenuFlyout_themeresources.xaml`), ComboBoxItem's in `context combo_box` (`ComboBox_themeresources.xaml`) and the
  suggestion list's ListViewItem states in `context autocomplete`; `adwaita`: `_menus.scss` selectors (`:hover`,
  `:selected`, `:active`,
  `:disabled`, `:checked`, HC) and `gtkmodelbutton.c`; `kde-breeze`: Breeze's `drawMenuItemControl` branches
  (`st:5977-6180`), qqc2's `MenuItem.qml`, kirigami-addons' `ConvergentContextMenu.qml`; `material-3`:
  `_md-comp-menu.scss` and the Expressive menu tokens (rest, hover, focus, pressed, selected, disabled); `macos-*`:
  `NSMenuItem.state` (on, off, mixed), `isEnabled`, highlight, `NSMenu.presentationStyle`; `ios-*`:
  `UIMenuElement.State`, `UIMenuElement.Attributes`, `UIMenu.ElementSize`, `UIMenu.Options`.
- **Today's fields:** `background_color` → `body.fill`; `separator_color` → `separator.stroke` paint; `font` →
  `item_label.text.font`; `row_height_px` → `item.min_height`; `icon_text_gap_px` → `values.icon_text_gap`;
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
    connector sets `@ka_bottom_drawer` or `@ka_dialog` from the application's display mode, or, where the
    application states none, the variant `values.convergent_variant` names (the bottom drawer under `cond
    mobile_form_factor`). Under them the box is another widget: `body` is `drawn_as = { widget = "sheet", variants =
    ["bottom", "@ka_bottom_drawer"] }` / `drawn_as = { widget = "dialog", variants = ["@ka_dialog"] }` (Nested
    controls), so the connector builds that element in `context menu` and one box is drawn. The parts nested in
    `body` are still queried (Nesting), with the sheet or dialog as their host, in the scope its `body` returns, and
    are drawn as kirigami-addons' form rows (`ContextMenuPage.qml`): under each of the two variants `item` states
    `drawn_as = { widget = "form", part = "row", variants = [] }` (`FormButtonDelegate`), `separator` `drawn_as = {
    widget = "form", part = "row_separator", variants = [] }` (`FormDelegateSeparator`), and `check_indicator`
    `drawn_as = { widget = "checkbox", variants = [] }` under the item's `check` and `drawn_as = { widget = "radio",
    variants = [] }` under `radio` (the row's leading check box or
    radio button); the row's own title, icon and trailing arrow (a submenu, `form.row_arrow`) take the place of
    `item_label`, `item_icon` and `submenu_arrow`, which are absent there with the other parts the drawer and dialog
    do not draw (may_be_absent). So the rules for the rows are `form.row @ context menu variant @ka_bottom_drawer`
    and `@ context menu variant @ka_dialog`, and the menu's own parts state only the substitutions.

## Buttons and toggles

### button

*From group 2's catalogue.*

- **Kind:** typed widget: all six platforms (push, tool, toggle and menu buttons; toolbar items, §2 of the merge:
  Qt's QToolButton, WinUI's AppBarButton and AppBarToggleButton, AppKit's toolbar items, iOS's bar button items), and
  every button nested in another widget (Nested controls: dialog responses, close, navigation, overflow, clear and
  reveal buttons, media controls, edit-menu commands).
- **Available:** every preset.
- **Parts:**
  - `body` — the bezel or container (the root): `fill` (libadwaita's state layers are appended paint layers, R3, Material's the `interaction` block, S2.12; an expression on
    `element.color` where the application tints the button: AppKit's `bezelColor`, a toolbar item's
    `backgroundTintColor`, MAC-B07, MAC-TL04, ADD-B16; the parts declare that data, `data = { body = ["color"],
    label = ["color"], icon = ["color"] }`, S8, S10), `stroke`, `corners`, `shadow` (layers or an elevation entry),
    `padding`, `margin` (a nested button's margins in its host, one negative: GNOME's tab and search-bar close buttons,
    GN-1.120, GN-2.21, the file chooser's eject button, GN-P3-22†; WinUI's tab close margin, W1-082; a GTK linked box's
    overlap, each child's negative start or top margin, GN-1.42), `min_width`,
    `min_height`, `max_width` (ContentDialog's button maximum width, W1-127),
    `opacity` (today's `disabled_opacity`), `transform` (Kirigami's RoundButton translated right and down while
    pressed or checked: the whole control moves, `qqc2-desktop-style@6.31.0 org.kde.desktop/RoundButton.qml:27-30`,
    Q2.7), `focus`, `cursor` (`WhatsThis` on `help`, ADD-A59), `touch_target` (Material's
    minimum interactive size round small buttons and icon buttons, MAT-X22), `bindings` (Kirigami's tool-button colour
    set: Button when raised; a flat one binds none and takes its parent's, as `Kirigami.Theme.inherit: flat` makes it
    share its parent's theme data, `ToolButton.qml:18-19`, `kirigami@6.31.0 src/platform/platformtheme.cpp:1060-1090`,
    Q1.9), `transitions`, `interaction` (Material's state layer and press ripple, S2.12).
  - `label` — the title text: `text` (its fill an expression on `element.color` where the application tints the
    content, AppKit's `contentTintColor`, MAC-B07), `padding` (GTK's label padding beside an image or arrow, GN-1.27,
    GN-1.39).
  - `subtitle` — a second text line under the title (iOS `UIButton.Configuration.subtitle`, B-8; KDE's command link
    description, K2.7): `text`.
  - `icon` — the image: `icon` (size, fill per state, rendering mode, weight and scale for SF Symbols; its fill an
    expression on `element.color` under `contentTintColor`, MAC-B07; on `kde-breeze` the clear-field, inline-action and
    chip-remove buttons' size is `= kirigami_label_icon_size(body_font_height)` under `cond flavor:kirigami`, the part declaring `data = { icon = ["body_font_height"] }` (Kirigami's `sizeForLabels`, which follows the user's font, Q1.1, ADD-A100): Q1.15, Q1.18, Q2.24), `glyph` (for system button types, B-12),
    `transitions` (state-triggered icon animations, `TransitionKind::Keyed` of group `Icon`: WinUI's AnimatedIcon
    states, W1-024, W3-28; GtkSvg's state transitions, GN-P3-19†; the SF Symbols content transition, B-16; ADD-A27,
    R25.12).
  - `menu_indicator` — the arrow or chevron of a button that opens a menu (GN-1.40, K1.27, B-18†, MAT-N8; KDE's
    inline small and large arrows, K1.155): `glyph`, `min_width`, `min_height`, `margin` (by
    `menu_indicator_anchor`: an inline indicator's margin in the row, an overlaid one's offsets), `transform` (its rotation, the file chooser's server-list arrow
    turning while `open`, GN-P3-23†), `transitions` (the press animation; that rotation's).
- **Widget values** (`ButtonValues`):
  - `icon_text_gap_px: Value<Px>` — between icon and label (today's `icon_text_gap`; a toolbar's in `context
    toolbar.bar`, K1.154, MAC-TL02).
  - `icon_placement: Value<IconPlacement>`, `IconPlacement { Leading, Trailing, Top, Bottom }` — the image beside or
    above the title (B-9 `imagePlacement`, MAC-B09 `imagePosition`).
  - `label_position: Value<ButtonLabelPosition>` — which of icon and text a tool button shows, and how (K1.153 read
    per toolbar kind, W1-099, MAC-TL02, Q1.9, Q1.76), by rules in `context toolbar.bar`.
  - `subtitle_gap_px: Value<Px>` — iOS `titlePadding` (B-8).
  - `menu_indicator_gap_px: Value<Px>` — between label and indicator (K1.27 `Button_ItemSpacing`).
  - `menu_indicator_anchor: Value<IndicatorAnchor>`, `IndicatorAnchor { Inline, InlineTop, TopEnd, BottomEnd }` —
    where the `menu_indicator` lies (ADD-A122, ADD-A123). `Inline` and `InlineTop` in the content's row, after the
    content, `menu_indicator_gap_px` from it, its `margin` a margin in the row as every part's is (S2.11, CSS's: it adds
    to the row's size, a negative one overlapping, so a start margin of `d` with an end margin of `−d` moves the box `d`
    toward the end and leaves the row's size alone); `Inline` centres its margin box on the row's cross axis,
    `InlineTop` puts it at the row's top, the top margin then the distance from the top of `body`'s content box.
    Where the row is wider than its content and indicator need (a stated minimum width, a stretched button), the
    spare width goes to the content: the indicator keeps the row's end and the content is laid out in the rest as it
    would be alone, centred where the button centres it (Breeze's push button, the arrow at the contents rect's end
    and the text centred in the rest, `breeze@6.8.2 kstyle/breezestyle.cpp:5364-5370`; GTK's label menu button, the
    label expanding before the arrow, `gtk@4.24.1 gtk/gtkmenubutton.c:1174-1186`; ADD-A123), except where the
    platform packs the content and indicator into one centred box, the spare width then going around both (GTK's icon
    menu button, its icon and arrow in a box centred in the button, `gtk/gtkmenubutton.c:1050-1051`, `:1061-1063`).
    `TopEnd`, `BottomEnd` over that corner of `body`, taking no layout space, its box its laid-out size (its content,
    at least `min_width` by `min_height` where it states them) and its `margin`'s top or bottom and its end (mirrored
    with the layout, S1) the distances from `body`'s edges at that corner; its two other sides place nothing, as CSS
    ignores an absolutely positioned box's margins on the sides it is not placed by, and are stated as the platform
    writes them (a zero where it writes none, S17's zero stated by omission). The overlaid anchors state Breeze's
    `@inline_small` arrow, a small arrow
    box at the bottom-right corner inset by fixed amounts (`breeze@6.8.2 kstyle/breezestyle.cpp:2940-2944`, mirrored
    by `visualRect`, `:2950`; K1.155), and WinUI's AppBarButton flyout chevron under the default and icon-only label
    positions (the label below the icon or none), top-aligned with a top and an end
    margin, in a zero-width `Auto` column, which WinUI's arrange stretches to the column less the margins, so its
    negative start margin keeps it out of the layout and, with the end margin, makes its box wider than the glyph,
    which is centred in it: `min_width` is that width, the start margin negated less the end margin
    (`microsoft-ui-xaml@2.5.1 src/controls/dev/CommonStyles/AppBarButton_themeresources.xaml:35`, `:357-361`, `:373`;
    `src/dxaml/xcp/core/core/elements/framework.cpp:1833-1842`, `src/dxaml/xcp/core/core/elements/icon.cpp:809-814`;
    W1-102), `min_height` `auto`. `InlineTop` states that chevron under the label-beside-icon position, inside the
    tabbed command bar, where it sits in the layout after the label, in an `Auto` column sized to it, top-aligned by
    its top margin (`AppBarButton_themeresources.xaml:36`, `:157`, `:177-179`, `:373`, W1-102); every other indicator
    is `Inline`.
  - `menu_placement: Placement` — where the menu a `menu`, `overflow` or `back` button opens goes (Nested controls:
    the host's value; B-18†, K-P3-5†, GN-P4-2†, W1-102, P4-12†; its arrowless popover on GNOME, GN-1.278†).
  - `menu_trigger: Value<MenuTrigger>`, `MenuTrigger { Primary, LongPress, LongPressOrSecondaryClick }` — the
    touch-and-hold variant (B-18†); AdwBackButton's navigation menu opens on a long press or a secondary click
    (GN-1.278†).
  - `shows_icon: Value<bool>` — whether push buttons show icons (K1.28, read from `kdeglobals`).
  - `focus_visible_when` (S2.11's `FocusVisibleWhen`) — when focus becomes visible (K1.21, R17).
  - `repeat_delay_ms: Value<Ms>`, `repeat_interval_ms: Value<Ms>` — durations of an auto-repeating button (K-RL07†,
    W2-19, MAC-X12). One home per fact, as `drag_hover_delay_ms` (Entry format): on `macos-*`, whose platform states
    one system pointer auto-repeat (`NSButtonCell.getPeriodicDelay(_:interval:)`, MAC-X12), the bases are `=
    token.params.pointer_repeat.delay` and `= token.params.pointer_repeat.interval` (S4.6, `unit = "ms"`); the
    widget's own values only where its own source states another: Qt's `autoRepeatDelay` and `autoRepeatInterval` on
    `kde-breeze` (K-RL07†), RepeatButton's `Delay` and `Interval` on `windows-11` (W2-19).
  - `back_title_fallback: Value<bool>` — UIKit shortens the back title, then replaces it by a generic one (C-2).
  - `back_display_mode: Value<BackDisplayMode>`, `BackDisplayMode { Default, Generic, Minimal }` — UIKit's
    `backButtonDisplayMode` (C-2).
  - `help_tooltip_on_click: Value<bool>` — KDE's ContextualHelpButton shows its tooltip on click (Q2.48).
  - `hit_extension: Sides<Value<Px>>` — margins that enlarge the hit area beyond the drawn button (K-KA-26†; a
    minimum size is `body.touch_target`).
  - `display_mode: Value<PasteDisplayMode>`, `PasteDisplayMode { Icon, Label, IconAndLabel }` — iOS's paste control
    (C-17).
  - `scaffold_placement: Value<Placement>` — where Material's scaffold puts the FAB (strategy
    `MaterialScaffoldSlot(Fab)`: end or centre, its spacing from the edge and above the bottom bar; `@ variant @fab`,
    MAT-X38†).
  - `presentations: Vec<PresentationTransition>` — events `Appear`, `Disappear` (a field's clear button fading in and
    out, `Instant` on WinUI: `button @ variant clear_field context input`, K1.46, W1-030), `@spring_load` (the flash
    or blink
    of a spring-loaded button while a drag hovers it, before it activates: MAC-X21, P3-11†), `@title_change` (the
    back button's label changing on a navigation push or pop, queried with the `back` button in `context toolbar.bar`:
    its step names `label`, C-2, X-MO-3; the same occasion as `toolbar`'s `@title_change`, which moves the title).
- **Variants:**
  - typed (a meaning two or more platforms share): `primary` (accent-filled: GNOME `.suggested-action`, WinUI
    `AccentButtonStyle`, Material filled, iOS `filled`, macOS `tintProminence` primary; a toolbar's prominent item in
    `context toolbar.bar`, TB-4, MAC-TL04), `tinted` (tinted, not filled: iOS `tinted`, macOS
    `NSTintProminence.secondary`, Material tonal), `flat` (no bezel at rest: GNOME `.flat`, KDE `setFlat`/auto-raise
    and Kirigami `flat`, WinUI `SubtleButtonStyle`, Material text, iOS `plain`, macOS `isBordered = false`),
    `destructive` (GNOME `.destructive-action`, macOS `hasDestructiveAction`, iOS destructive role), `warning` (KDE's
    neutral highlight; GNOME's `.warning` elsewhere), `pill` (GNOME `.pill`, iOS capsule corner style, macOS 26
    capsule border shape), `circular` (GNOME `.circular`, KDE `_kde_round_button` and RoundButton, macOS circular
    bezel and circle border shape, one variant as they draw alike: the probe's layers of the circular bezel and the
    push bezel with the circle border shape agree in every measured field, at every control size, in every captured
    state and both appearances, macOS 26, run 41006842699, `button-pass` sheets `styles_a` and `styles_c`; ADD-A123), `glass` (iOS 26, macOS 26), `help` (macOS's help bezel MAC-N10, KDE's
    ContextualHelpButton Q2.48), `toggle` (a button with an on state: GtkToggleButton, Qt checkable, WinUI
    `ToggleButton` and AppBarToggleButton, Material toggle, macOS `pushOnPushOff`, iOS
    `changesSelectionAsPrimaryAction`; Material draws its unselected toggle unlike a plain button, MAT-B12), `menu`
    (opens a menu: GtkMenuButton, Qt `HasMenu`/QML menu flag, iOS pull-down, WinUI DropDownButton, Material menu
    button, macOS's `NSComboButton` in its `.unified` style and the menu toolbar item, its default `.split` style
    being the `split_button` widget, `NSComboButton.h:14-20`, `:65-66` in the Xcode 26.6 SDK, MAC-N28†; MAC-N29†, W2-18, MAT-N8, B-18†: iOS's pull-down button is
    this variant only; AppKit's pull-down `NSPopUpButton` is a pop-up button control, `combo_box`'s keyed `macos-*`
    `@pull_down`), `tool` (Qt's QToolButton and qqc2 ToolButton, also outside
    toolbars, Q1.9; WinUI's AppBarButton and AppBarToggleButton, W1-099, W1-100; AppKit's toolbar bezel, MAC-TL02),
    `clear` (the clear glass style: iOS 26's `clearGlass`; the meaning `glass_container`'s `clear` shares, macOS 26),
    and the content variants `icon_only` and `with_icon` (GTK's `.image-button` and `.image-text-button`, Material's
    leading-icon padding and icon buttons MAT-N4, MAT-N5, C-29, AppKit's image-only padding; text only is the base).
    KDE's split tool button (`MenuButtonPopup`, K1.155) is the `split_button` widget.
  - typed identities of a button nested in another widget (Nested controls): `close` (a close or dismiss button:
    dialog headers K-KA-17† and MAT-N26, banners, toasts, sheets, teaching tips, a search bar GN-2.21, tabs, KDE's dock
    widget; iOS's system close type), `back` (a navigation back button in its host's bar: GNOME's AdwBackButton,
    WinUI's TitleBar back button, iOS's navigation-bar back button: GN-1.278†, W1-011, C-2), `previous`, `next` (stepping through items or pages: calendars, carousels, page
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
  - keyed, `adwaita`: `@raised`, `@osd`, `@opaque`, `@card`, `@with_arrow` (`.arrow-button`: a menu button showing
    its arrow beside an icon, a label or a custom child, `gtk@4.24.1 gtk/gtkmenubutton.c:72-75`, `:644-647`; an
    arrow-only menu button is `.image-button`, `icon_only`, `:634-637`; a content variant (Entry format) the connector
    sets on a `menu` button wherever GTK adds the class, the arrow shown and not the only content, `:631-632`,
    `:644-647`, `:658-666`: on text-only content (neither `icon_only`, `with_icon` nor `@custom_child`), which GTK's
    label makes a text button, unless the element holds `@arrow_none`; on other content (an icon, `with_icon`, `@custom_child`; never `icon_only`
    with `@arrow_only`, which GTK leaves without the class, `!has_only_arrow`, `:644`) only where the application
    asks for the arrow (an option of the connector's button the application sets, GTK's `always-show-arrow`) and holds no `@arrow_none`;
    ADD-A123), `@arrow_only` (with
    `icon_only`: a menu button whose only content is its arrow, `has_only_arrow`, `gtkmenubutton.c:631`, `:661-665`,
    which draws the arrow as its `menu_indicator`, where `menu` with `icon_only` alone is an icon menu button whose
    arrow GTK hides, `:661-665`; a content variant the connector sets on a `menu` button with no content but its
    arrow, as it sets `@custom_child`; ADD-A122, ADD-A123), `@arrow_up`,
    `@arrow_start`, `@arrow_end`, `@arrow_none` (GtkMenuButton's direction; down is the base), `@tab_button`
    (AdwTabButton, its counter a `badge` in its context, GN-2.37), `@custom_child` (a content variant: a button whose
    child is neither a label nor an image, so GTK sets neither `.text-button` nor `.image-button`, `gtk/gtkbutton.c:431-432`: the base
    `button` padding, `libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:22`, where the base, text only, is
    `.text-button`'s, `:72-75`), `@image_text_box` (with `with_icon`: an icon and a label in a box the application
    builds and classes `.image-text-button` or `.text-button.image-button`, whose gap and label padding are
    `_buttons.scss:77-91`'s; `with_icon` alone is AdwButtonContent's, the libadwaita form, `:626-645`, which differs in
    gap and label padding, GN-1.27, GN-1.39).
  - keyed, `kde-breeze`: `@highlighted` (QML `Button.highlighted`), `@inline_small`, `@inline_large` (the
    menu indicator's styles of a tool button, K1.155; its split style is `split_button`), `@command_link` (K2.7: its
    title `label`, its description `subtitle`, with `icon`; Windows' command link is Win32, R1.3), `@ka_floating`
    (kirigami-addons' FloatingButton, K-KA-26†), `@key_sequence` (KDE's key-sequence button in a shortcuts editor,
    K-RL02†, K-KA-43†), `@float` (a dock widget's float button, K2.10), `@ka_download` (kirigami-addons' download
    button over a media item not yet downloaded, a Button with a `large` icon in `context media_controls`, K-KA-35†).
  - keyed, `macos-*`: the bezel styles `@flexible_push`, `@small_square`, `@accessory_bar`, `@accessory_bar_action`,
    `@badge`, `@shadowless_square`, one variant per `NSButton.BezelStyle` raw value: the SDK's old names `recessed`,
    `roundRect`, `inline` and `texturedRounded` are `accessoryBar`, `accessoryBarAction`, `badge` and `toolbar` renamed,
    the same raw values (the macOS probe; Apple's `NSButton.BezelStyle` documentation names each old case renamed to the
    new one, deprecated in macOS 27.0, read 2026-10-30), so they are no variants of their own (push is the base: the
    style an application asks for by no variant; `automatic`, the initialisers' default since macOS 14, raw value 0,
    "determined based on the button's contents and position within the window" with no rule stated,
    `NSButtonCell.h:29-31` in the Xcode 26.6 SDK, is no variant of its own: what it draws for a button's contents and placement is a
    look the probe measures per placement before a fact rests on it (the probe's open item), stated then as the
    variant it matches or, drawing none of them, as a keyed variant; ADD-A123;
    the route button's `@bordered`, `isRoutePickerButtonBordered`, MAC-P3-9†;
    borderless is `flat`, circular `circular`, glass `glass`, help `help`, toolbar `tool`; the disclosure bezels are
    `expander`'s, MAC-B01, MAC-EX01); the 26 border shape `@rounded_rectangle` (capsule is `pill`, circle
    `circular`, automatic the base). The fills that follow the accent other than the accent itself (a default
    button's pressed fill, a checked toggle's, `tinted`'s) are `mix` over `input.accent` (S2.2) toward black under `scheme:light` and white
    under `scheme:dark`, each a measured derivation (S17, `inputs = ["input.accent"]`), no function of their own: a
    pressed default button is `mix(input.accent, <black or white>, <t>)`, one fraction `<t>` matching within one
    channel step in both of the probe's accents and appearances (run 41006842699, `button-pass` and
    `button-pass-accent-red`, sheet `push`, `regular-default`; the macOS verifier, queue #62; ADD-A123).
  - keyed, `ios-*`: `@gray` (UIKit's `plain`, `gray`, `tinted` and `filled` styles are `flat`, `@gray`, `tinted`
    and `primary`; its `borderless`, `bordered`, `borderedTinted` and `borderedProminent` are the same four, the
    connector mapping them: the iOS host drew each pixel-identical to its counterpart in every state, size and scheme
    it captured on iOS 18.6 and 26.5, run [40984711960](https://github.com/tiborgats/native-theme/actions/runs/40984711960),
    its captures on the lane branch `lane-ios-slice`, so they are no variants
    of their own until a capture shows a difference; the glass styles are `glass` and `clear`, with `primary` for the
    prominent ones; the configuration styles are two `exclusive_variants` groups, `[flat, @gray, tinted, primary]`
    and `[flat, @gray, tinted, glass, clear]`, so `primary` combines with `glass` and `clear` alone, S10; the state
    model's `variant_sets` lists the sets of styles the platform draws, `[glass, primary]` (`prominentGlass`) and
    `[clear, primary]` (`prominentClearGlass`), and each further set the facts capture, a configuration style with a
    corner style or with `toggle`, S17, never every combination of the declared variants), the corner
    styles `@corner_fixed`, `@corner_small`, `@corner_medium`, `@corner_large`, `@corner_dynamic` (capsule is `pill`),
    one `exclusive_variants` group with `pill`, `[@corner_fixed, @corner_small, @corner_medium, @corner_large,
    @corner_dynamic, pill]`, as a configuration holds one `cornerStyle` (B-4, `UIButton.Configuration.CornerStyle`
    as the gap map reads Apple's documentation, unconfirmed: an Open item until the iOS lane confirms it against the
    pinned Xcode's `UIButtonConfiguration.h`; ADD-A122),
    the role `@cancel` (iOS 26's further bar-item roles are an open item, Open items: added
    here when the facts state them); the system types `@info`, `@contact_add`, `@share` (the share button, its glyph UIKit's,
    X-IC-5; the close type is the typed `close`); the bar items' `@padding_removed` (TB-8; done is the typed `done`);
    `@indicator_none`, `@indicator_popup` (the pull-down indicator; automatic is the base); `@menu_on_hold`; the
    pointer effects `@pointer_highlight`, `@pointer_lift`, `@pointer_hover` (`UIPointerEffect`'s styles, X-PH-1);
    `@paste_control` (C-17).
  - keyed, `material-3`: `@elevated`, `@outlined` (container styles; icon buttons too, MAT-N4, MAT-N5), `@square`
    (the Expressive shape family, round the base; with `icon_only` the square icon button, MAT-N5, MAT-B10), `@narrow`,
    `@wide` (the Expressive icon button's narrow and wide leading and trailing spaces, default the base, under `cond
    flavor:expressive`, MAT-N5), `@fab`, `@extended_fab`, `@lowered` (MAT-N1, MAT-N2; the two forms an `exclusive_variants` group,
    `[@fab, @extended_fab]`, each a composable of its own, `FloatingActionButton` and its sizes,
    `ExtendedFloatingActionButton` and its sizes, `androidx@e4d933c9
    compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/FloatingActionButton.kt:131`, `:601`,
    ADD-A122), and the FAB's colour styles
    `@secondary`, `@tertiary`, `@surface` (its primary style is the typed `primary`, MAT-N1), the Expressive container
    styles `@secondary_container`, `@tertiary_container` and the branded FAB `@branded` (MAT-N1), and in both
    flavours `@primary_container`, Compose's default FAB container colour (`FloatingActionButtonDefaults.containerColor`,
    `FabPrimaryContainerTokens.ContainerColor` = `PrimaryContainer`,
    `androidx@e4d933c9 compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/FloatingActionButton.kt:1044-1046`,
    `tokens/FabPrimaryContainerTokens.kt:21-22`).
  - keyed, `windows-11`: `@repeat` (RepeatButton, W2-19), `@query` (AutoSuggestBox's query button, W2-15),
    `@accept`, `@dismiss` (the date and time picker flyouts' accept and dismiss buttons, W2-04), `@first`, `@last`,
    `@page_number` (PagerControl's first- and last-page and number buttons, W2-21), `@zoom_out` (SemanticZoom's
    zoom-out button, W2-34).
- **Sizes and densities:** `macos-monterey`: `mini`, `small`, `regular`, `large`; `macos-tahoe`: those and
  `extra_large` (`NSControl.ControlSize`; MAC-B08, MAC-B09, MAC-B11†). A size's label font is the one AppKit gives that control size,
  `+[NSFont systemFontSizeForControlSize:]` (`NSFont.h:79` in the Xcode 26.6 SDK), smaller at `mini` and `small`, as
  the probe's button pass sets and measures it on macOS 26 (run 41006842699, `button-pass`, sheet `push`, and
  `macos.fonts.sizes.systemFontSizeForControlSize`). A button whose application sets the control size and not the font keeps
  the font it was created with (the regular size's at every size, the macOS lane found, queue #62): that is the application leaving the size's font unset, not a look of the platform's,
  so the facts state `systemFontSizeForControlSize:`'s (ADD-A123). `ios-*`: `mini`, `small`, `regular` (UIKit
  `medium`), `large` (`UIButton.Configuration.Size`, B-3); `mini` draws as `small` on iPhone (the iOS host's captures
  are byte-identical in every style, state and scheme, iOS 18.6 and 26.5, run
  [40984711960](https://github.com/tiborgats/native-theme/actions/runs/40984711960) on `lane-ios-slice`), so the facts state
  `mini`'s values under `idiom:phone` equal to `small`'s, each cited to its own capture, and on `idiom:pad` from its
  own measurement; no alias is declared (a repeated value is a fact like any other). `material-3` flavour `expressive`: `extra_small`,
  `small`, `regular` (medium), `large`, `extra_large` (xlarge) (MAT-B9); by variant (`sizes_by_variant`, S10): `@fab` `small`,
  `regular` (the standard FAB), `large` (Expressive's medium FAB, declared under flavour `expressive`), `extra_large`
  (the large FAB) (MAT-N1); `@extended_fab` by the same rule, `regular` the unsized `ExtendedFloatingActionButton`
  in both flavours (the baseline container height, `ExtendedFabTextPadding`, `ExtendedFabMinimumWidth`, `LabelLarge`, `androidx@e4d933c9
  compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/FloatingActionButton.kt:601-629`,
  `:877-928`, `:1472-1482`, `tokens/ExtendedFabPrimaryTokens.kt:64`), and declared under flavour `expressive` only
  `small`, `large` and `extra_large`, Expressive's small, medium and large extended FABs
  (`SmallExtendedFloatingActionButton` …, `:400`, `:467`, `:534`, `:1434-1470`) (MAT-N2; ADD-A123: the unsized one
  had no size under Expressive, where `regular` named the medium one). Default sizes (`default_size_by_variant`,
  S10, ADD-A121): `@fab` and `@extended_fab` `regular` in both flavours, the unsized composables, where the
  Expressive buttons' default is `small`. Others: none. Which size an element that states none has is declared (ADD-A60). Densities: none from the
  button rows; W3-39's compact dictionary sizes no button, so `windows-11` declares none; `macos-*` MAC-X17 and
  `material-3` MAT-X22 are the cross-widget density rows, traced at their own lines.
- **Flags:** shared `route_active` (on `route`: macOS, iOS); keyed `ios-*` `@loading` (B-10), `kde-breeze`
  `@recording` (the key-sequence button held pressed while it records, K-RL02†, K-KA-43†), `@keyboard_focus_change`
  (flavour `kirigami`: the element's last focus change, in or out, was by the keyboard, Tab, Backtab or a shortcut,
  as qqc2-desktop-style's style item tests the control's `focusReason`, `qqc2-desktop-style@6.31.0
  plugin/kquickstyleitem.cpp:42-45`, `:884-890`, which Qt Quick sets on focus in and on focus out,
  `qtdeclarative@6.12.0 src/quick/items/qquickitem.cpp:4237`, `:4250`; Breeze draws the pen it passes as focus only
  with it, `breeze@6.8.2 kstyle/breezestyle.cpp:4518`, `:4577`, and `Button.qml` passes `highlighted` as focus, so a
  `@highlighted` button keeps its pen after focus leaves it by Tab; the connector sets it from its toolkit's focus
  reason, Q1.8, Q1.9; ADD-A122); state flags used: hover,
  pressed, focused, focus_visible, disabled, checked, indeterminate (WinUI's toggle), selected (a selected bar item,
  P3-3†, MAC-TL11†), open, default, drop_target (a spring-loaded button under a drag, P3-11†, MAC-X21), dragged (a
  toolbar item dragged in the customization, MAC-P3-6†, P4-10†), over_dark_content (on `clear`: the clear glass's
  dimming layer where the content beneath is bright, X-MT-10).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (the CommonStates of the Button, ToggleButton,
  AppBarButton, AppBarToggleButton, accent and subtle templates; their Checked* and Indeterminate* states name the
  check state with them, so `checked` and `indeterminate` each combine with that group) and `[indeterminate, checked]`
  (the template takes the Indeterminate* states while `IsChecked` is null and the Checked* states only otherwise,
  `microsoft-ui-xaml@2.5.1 src/dxaml/xcp/dxaml/lib/ToggleButton_Partial.cpp:98-133`; ADD-A122); others none (states
  combine: libadwaita's CSS and Breeze's branches combine flags; K1.29's "pressed wins" is a rule order, not a group;
  `material-3`'s are recency sets, below); precedence: state models (S17). Recency sets: `material-3`:
  `state_layer = [focus_visible, hover]` (Material's recency sets, above: Compose's ripple, `Button.kt:156-157`,
  `Surface.kt:253-264`; `pressed` combines; Material draws no dragged layer on a button, MAT-X29) and
  `elevation = [pressed, focus_visible, hover]` (the elevated, filled and tonal buttons, toggle buttons and FABs:
  `Button.kt:1703-1747`, `ToggleButton.kt:1589-1633`, `FloatingActionButton.kt:1300-1347`; MAT-B6, MAT-N1).
- **may_be_absent:**
  - parts: `subtitle` (all but `ios-*`, `kde-breeze`), `icon` (`ios-*` by a rule under `@loading`: a `spinner` element
    in `context button` in its place, B-10), `menu_indicator` (every preset: stated in the base, absent by a
    rule `@ !variants {menu}` on every preset, which `kde-breeze`'s rule extends with its own declared keyed variants,
    `@ !variants {menu,@inline_small,@inline_large}`, and `adwaita`'s by one more, `@ variants {menu}
    !variants {@arrow_only,@with_arrow}`: GTK shows a menu button's arrow only where the arrow is its only content or
    the button is `.arrow-button`, hiding it for an icon, a label whose direction is `none`, AdwButtonContent and a
    custom child without `always-show-arrow` (`gtk@4.24.1 gtk/gtkmenubutton.c:631`, `:644-647`, `:658-666`; ADD-A123):
    each preset's rule names only the variants it declares, S12).
  - fields: `body.shadow` (`windows-11`; `adwaita`, `kde-breeze`, `macos-*`, `ios-*`, `material-3` by rules outside
    the states, variants and conditions their rows draw it in: GN-1.25, GN-1.29, GN-1.38, K1.23, MAC-B04 on
    `macos-monterey`, MAC-X01 under `glass` on `macos-tahoe`, B-17, MAT-B1, MAT-N1), `body.cursor` (`adwaita`,
    `macos-*`, `windows-11`, `ios-*`: GTK's, AppKit's, WinUI's and UIKit's buttons set no cursor, GN-3.67†, MAC-X20,
    W3-29; `kde-breeze` by a rule outside `help`, whose `WhatsThis` cursor is the only one Breeze sets on a button,
    K3.65†; Material's buttons show the pointer, MAT-X33†), `icon.glyph` (`adwaita`, `material-3`: their identity
    buttons draw a themed icon, the part's `icon`, GN-2.22, MAT-N13; `kde-breeze`, `macos-*`, `windows-11`, `ios-*`
    by rules outside the identities and system types they draw a glyph for: K1.46†, MAC-N10, W1-011, B-12).
  - values: `menu_trigger` (all but `ios-*`: `adwaita` states it only where a button opens a menu, AdwBackButton's
    long press, GN-1.278†, its plain buttons opening none), `subtitle_gap_px` (all but `ios-*`), `back_display_mode`
    (all but `ios-*`), `display_mode` (all but `ios-*`), `scaffold_placement` (all but `material-3`), `shows_icon`
    (all but `kde-breeze`: no other platform has the setting, K1.28), `hit_extension` (all but `kde-breeze`),
    `repeat_delay_ms` (`adwaita`, `material-3`, `ios-*`: no auto-repeating button; `windows-11` by a rule outside
    `@repeat`, RepeatButton's alone, W2-19), `repeat_interval_ms` (`adwaita`, `material-3`, `ios-*`: no
    auto-repeating button; `windows-11` likewise), `label_position` (`adwaita`, `ios-*`, `material-3`: no tool
    button with a label choice). `presentations` is a list: empty, not absent, where the platform has none.
    The two flags of iOS's back title and KDE's help button are false, not absent, where the platform does not
    shorten a back title or show a tooltip on click (a `Value<bool>` the platform does not do).
- **State-model source:**
  - `adwaita`: libadwaita's selectors in `_buttons.scss`, `_toolbars.scss` (flat by container), `_linked.scss`,
    `_common.scss` (needs-attention); GTK's `:hover`, `:active`, `:checked`, `:disabled`, `:drop(active)`,
    `.has-open-popup`, `:focus-visible`, `prefers-contrast: more`.
  - `kde-breeze`: Breeze's `Helper::renderButtonFrame` branches (`hp:647-741`) and `Style` `CE_PushButton`,
    `CC_ToolButton` (`st:4511-4538`, `:5322-5375`, `st:4572-4619` for the auto-raise tool button, K1.152); flavour
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
  `values.icon_text_gap`; `primary_background` → `body.fill @ variant primary` (on `macos-*`: `@ {default}`);
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
    (GN-1.278†, W1-011, W1-095, C-2; the back label's push and pop is the button's event `@title_change` there); iOS's
    loading indicator (B-10) is a `spinner` element in `context button {@loading}`; a field's
    `clear_field` and `reveal` in `context input`; `close` in a dialog's, banner's, toast's, sheet's, teaching tip's or
    tab's context; the responses in `context dialog.action_area`.
  - Group 6: `circular` covers KDE's RoundButton (Q2.4–Q2.8: a gloss as an appended linear paint layer, the whole
    button translated under `{pressed}` / `{checked}` as `body.transform`, the Selection binding under `{focused}`);
    Breeze's QWidget round button (K1.26, `_kde_round_button`) is drawn with a radius of half its larger side, which
    Qt clamps per axis to half each side (`breeze@6.8.2 kstyle/breezehelper.cpp:727`, `:738-739`; `qtbase@6.12.0
    src/gui/painting/qpainter.cpp:4015-4018`, `qpaintengineex.cpp:745-746`, the path fallback
    `qpainterpath.cpp:3457-3470`), so a square box is a circle and a box that is not square an
    ellipse: `body.corners` is `Ellipse` on `kde-breeze` (S2.4, ADD-A124), its shadow following the ellipse of its own
    box (S2.5), while Kirigami's RoundButton (`cond flavor:kirigami`) is a capsule, `Full`, its background's radius
    the control's, half its smaller side (`qqc2-desktop-style@6.31.0 org.kde.desktop/RoundButton.qml:83`,
    `qtdeclarative@6.12.0 src/quicktemplates/qquickroundbutton.cpp:48`); `help` takes the
    cursor `WhatsThis` (ADD-A59); Material's icon buttons are `icon_only` with `primary`, `tinted` and `@outlined`
    (MAT-N4, MAT-N5, C-29).
  - Material's state layer and press ripple are `body.interaction` (S2.12), drawn over the label and icon and beneath
    the outline, as Compose's ripple node and its surface's border draw them (`Ripple.kt:373-377`, `Surface.kt:562-564`,
    `Border.kt:212-214`), so the label's and icon's colours are stated as the tokens give them, with no composite
    under the layer (ADD-A121's `mix` there gone, ADD-A124).
  - Material Expressive's buttons, toggle buttons and icon buttons morph only their corners on press and selection
    (Compose animates a `RoundedCornerShape` or `CornerBasedShape`, `Button.kt:1921-1938`, `ToggleButton.kt:1714-1717`,
    `IconButton.kt:1632`, `:1652`): `body.corners` with a `Corners` spring transition (MAT-B11, MAT-X12); `button` has
    no `shape` field, the `MaterialShapes` morph being the loading indicator's (MAT-N24).
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
  - `text` — the link text: `text` (its text style carries the underline as `decoration`; where the platform draws
    the underline from the font's own metrics, S2.8, the decoration's `thickness` is `=
    element.font_underline_thickness` and its `offset` `= element.font_underline_offset`, the part declaring `data = {
    text = ["font_underline_offset", "font_underline_thickness"] }`, and a number only where the platform draws a
    fixed line: the value phase states which per preset from the facts), `focus` (where the
    platform rings the text: KDE draws focus by weight instead, Q1.135), `cursor` (the pointing hand over the text,
    MAC-LK02, MAC-X20).
  - `icon` — the external-link icon after the text (Kirigami `UrlButton`, Q1.136): `icon` (its size `= kirigami_label_icon_size(body_font_height)` under `cond flavor:kirigami`, the part declaring `data = { icon = ["body_font_height"] }` (Kirigami's `sizeForLabels`, which follows the user's font, Q1.1, ADD-A100)).
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
  - fields: `body.fill` (`kde-breeze`, `macos-*`, `material-3`: their links draw the text only; `adwaita`:
    GtkLinkButton's fill is transparent in every state, GN-1.266; `ios-*` by a rule outside `{pressed}`, LK-1;
    `windows-11` by a rule outside `{pressed}` and `{disabled}`, W1-144), `body.stroke` (all but `windows-11`: only
    HyperlinkButton draws a border; `windows-11` by a rule outside `{hover}`, `{pressed}` and `{disabled}`, W1-144),
    `body.focus` (`adwaita`: GTK rings the text, GN-1.264; `kde-breeze`: Kirigami marks focus by weight, Q1.135;
    `windows-11` under `inline`, whose text WinUI rings, W1-145), `text.text.decoration` (`kde-breeze` by a rule
    outside `{hover}` under `@float`, K1.260†; `windows-11` by rules under `inline` in the states WinUI draws no
    underline in, `{hover}` and `{pressed}`, W1-143), `text.focus` (`kde-breeze`: Kirigami marks focus by weight,
    Q1.135; `macos-*`: AppKit rings the `body`, MAC-LK03).
  - values: `icon_text_gap_px` (all but `kde-breeze`: only Kirigami's `UrlButton` draws an icon, Q1.136). The URL
    tooltip flag is false, not absent, where the platform shows no URL tooltip (a `Value<bool>`).
- **State-model source:** `adwaita`: `_links.scss` (`:visited`, `:active`, `:disabled`, `.osd &`, the link-styled
  button); `kde-breeze`: Kirigami `LinkButton.qml`, `UrlButton.qml`, KWidgetsAddons `kurllabel.cpp`; `macos-*`:
  `NSTextView.linkTextAttributes` and the focus ring under Full Keyboard Access; `ios-*`: the text view's link
  interaction (pressed), measured; `windows-11`: HyperlinkButton's template states and `Hyperlink.cpp`'s underline per
  state; `material-3`: Compose's `TextLinkStyles` (style, focused, hovered, pressed).
- **Today's fields:** `font` → `text.text.font`; `visited_text_color` → `text.text.fill @ {visited}`;
  `underline_enabled` → `text.text.decoration` (a decoration or none, its thickness and offset as the `text` part
  says); `background_color` → `body.fill`;
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
    roles, GN-1.247), `transitions`, `interaction` (Material's state layer and press ripple, S2.12). `label`, `icon`, `indicator`, `check`, `menu_indicator` belong to `segment`
    and are queried with it.
  - `label` — a segment's text: `text`, `opacity` (the dimmed label of a disabled group, GN-1.242), `transitions`
    (kirigami-addons' selected label turning to its selected text style, K-KA-25†).
  - `icon` — a segment's image: `icon` (its tint per state, MAC-SG10†; kirigami-addons' radio selector's size `= kirigami_label_icon_size(body_font_height)` under `cond flavor:kirigami`, the part declaring `data = { icon = ["body_font_height"] }` (Kirigami's `sizeForLabels`, which follows the user's font, Q1.1, ADD-A100), K-KA-25†), `opacity` (the dimmed image of a disabled
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
    start edge), `margin` (its insets from its ends, GN-1.243, GN-1.245: a stroke lies on its box's edge, S2.4, so
    padding would not shorten it), `opacity`, `transitions` (its fade as it hides beside a selected segment,
    GN-1.243).
  - `check` — Material's check glyph in a selected segment (MAT-SG1): `glyph`, `transitions` (its show and width
    animation, MAT-SG1: absent outside `{selected}`, it enters from transparent by these, S2.7).
  - `menu_indicator` — AppKit's per-segment menu glyph (MAC-SG07): `glyph`.
- **Widget values:** `segment_width: Value<SegmentWidth>`, `SegmentWidth { Fit, Fill, FillEqually,
  FillProportionally, EqualToWidest }` — `Fit` each its content's width (AdwToggleGroup's default, AppKit `fit`, the
  Toolkit's pivot and button styles), `Fill` content plus an equal share of extra space (AppKit's default `fill`),
  `FillEqually` equal widths across the control (Material, AppKit `fillEqually`, UIKit's default),
  `FillProportionally` (AppKit), `EqualToWidest` (the Toolkit's `EqualPanel`) — GN-P3-25†, MAC-P3-5†, MAT-P3-12†,
  SG-9†, W-P3-5†; `item_gap_px: Value<Px>` (W-P3-5†; negative where segments overlap, K-KA-24†); `compression:
  Value<Key>` (AppKit `activeCompressionOptions`, how segments compress when the control is too narrow, MAC-P3-5†;
  declared `{ key = ["standard", "hide_images", "hide_text", "reduce_metrics", "break_equal_widths"] }`, the members
  of `NSUserInterfaceCompressionOptions`, `standard` AppKit's standard combination of them); `image_scaling:
  Value<Key>` (AppKit `imageScaling`, MAC-SG10†; declared `{ key = ["proportionally_down", "axes_independently", "none",
  "proportionally_up_or_down"] }`, the cases of `NSImageScaling`);
  `indicator_draggable: Value<bool>` (SG-5); `pressed_width_factor: Value<f32>` (Material's standard button group:
  the pressed segment's width is its width times this factor, its neighbours narrowing to make room, applied by the
  connector at layout, MAT-N6); `presentations: Vec<PresentationTransition>` (queried with the control): events
  `@indicator_move` with a step on the segment's part `indicator` (S2.7: the indicator sliding to the new segment:
  SG-5, MAC-SG09, W1-138, K-KA-25†), `@spring_load` (a
  spring-loaded segment's flash or blink while a drag hovers it, MAC-X21, P3-11†).
- **Variants:** typed `pill` (GNOME `.round`, AppKit `.capsule` and the 26 capsule shape), `momentary` (AppKit
  `momentary` tracking, UIKit `isMomentary`), `flat` (GNOME `.flat`: no track), `vertical` (GNOME `.vertical`),
  `menu` on a segment (AppKit `showsMenuIndicator`), the content variants `icon_only`, `with_icon` (GNOME's minimum
  widths, GN-1.244; the Toolkit's item layouts, W1-154†). Keyed: `adwaita` `@osd`; `macos-*` `@rounded`,
  `@round_rect`, `@textured_rounded`, `@textured_square`, `@small_square`, `@separated`, `@rounded_rectangle` (26
  shape), `@select_any`; `windows-11` `@pivot`, `@button` (the default style is the base); `material-3`
  `@standard_group`, `@connected` (the button groups, MAT-N6; flavour `expressive`'s, declared under `[declare.flavor.expressive.widget.segmented_control]` only, so `flavor:baseline` answers `NotAvailable` for them (R2.7, R15)); `kde-breeze` `@ka_segmented_button` (K-KA-24†),
  `@ka_radio_selector` (K-KA-25†).
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular`, `large` (MAC-SG10†). `material-3` flavour
  `expressive`: the button groups' sizes by variant (`sizes_by_variant`, S10), `@standard_group` and `@connected`
  each `extra_small`, `small`, `regular`, `large`, `extra_large` (MAT-N6), so a plain segmented button declares no
  size (R15). `ios-*`: no sizes; the height
  follows Dynamic Type (`token.text.scaling`, S9.4; `uifm_scaled`, R10, SG-9†). Densities: none from these rows.
- **Flags:** shared none; keyed none; state flags used: hover, pressed, selected (S7.2: a chosen segment is the
  current choice among its siblings, `selected` on every platform; under `@select_any` each chosen segment is
  `selected`), disabled, focused, focus_visible.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` on `segment` (SegmentedItem's CommonStates, which
  name Selected, PointerOverSelected and PressedSelected with them, so `selected` combines); others none (states
  combine; `material-3`'s state layers are a recency set, below); precedence: state models (S17). Recency sets:
  `material-3`: `state_layer = [focus_visible, hover]` on `segment` (Material's recency sets, above: a `Surface`'s
  ripple, `SegmentedButton.kt:150-166`; `pressed` combines).
- **may_be_absent:**
  - parts: `indicator` (`adwaita`, `material-3`: the selected segment's own fill; `kde-breeze` by a rule under
    `@ka_segmented_button`, whose selected segment is filled, K-KA-24†; `macos-*`, `ios-*`, `windows-11`,
    `kde-breeze` by a rule outside `{selected}`: SG-1, MAC-SG02, W1-138, K-KA-25†), `separator` (`windows-11`,
    `material-3`, `kde-breeze`: kirigami-addons' joined segments overlap and draw no line, K-KA-24†), `check` (all but
    `material-3`; `material-3` by a rule outside `{selected}`, MAT-SG1), `menu_indicator` (all but `macos-*`;
    `macos-*` by a rule outside `menu`, MAC-SG07).
  - fields: `track.fill` (`material-3`; every preset under `flat`), `track.shadow` (`adwaita` by a rule outside
    `cond contrast_high`, GN-1.251; the value phase names the others from the facts), `segment.shadow` (`adwaita` by
    a rule outside `{selected}`, GN-1.240; `kde-breeze` by rules outside `@ka_segmented_button` and under
    `{pressed}` or `{disabled}`, K-KA-24†; the value phase names the others from the facts), `indicator.shadow`
    (`macos-tahoe`: the raised segment's shadow is macOS 12–15's, MAC-SG02; the value phase names the others from
    the facts).
  - values: `compression` (all but `macos-*`), `image_scaling` (all but `macos-*`), `item_gap_px` (`adwaita`,
    `ios-*`: their segments abut, the `separator` between them, GN-1.243, SG-3; `macos-*` in the base, its segments
    abutting, MAC-SG01, and stated by a rule under `@separated`, whose segments AppKit draws apart, MAC-SG04),
    `pressed_width_factor` (all but `material-3`). The draggable-indicator flag is false, not absent, where the
    indicator cannot be dragged (a `Value<bool>`, SG-5).
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
    state), `transform` (iOS 26's lens; WinUI's knob sliding between off and on, a translation per `{checked}`, as
    WinUI moves it, W1-123), `transitions` (the thumb sliding between off and on, a `Geometry` transition, which
    starts when the thumb's laid-out box moves, S2.7: GN-1.203, SW-5, Q1.102, MAT-SW8; on `windows-11` a `Transform`
    transition of that translation, RepositionThemeAnimation, W1-123, so the knob's size change keeps the `Geometry`
    one; its size and colour changes, MAT-SW8, W1-123).
  - `track_fill`, in `track` — KDE's highlight overlay from the track's start to the thumb's trailing edge (Q1.100): `fill`.
  - `on_glyph`, `off_glyph`, in `track` — the status shapes on the track (GNOME `switch-on/off-symbolic`, iOS's "I" and "O",
    macOS under differentiate without colour): `glyph`, `margin` (their place on the track, as insets from its edges,
    SW-4).
  - `thumb_icon`, in `thumb` — Material's check or close icon on the handle (MAT-SW3): `icon`, `glyph` (the symbol: the check
    under `{checked}`, the close otherwise, MAT-SW3), `opacity` (the disabled icon's opacity, MAT-SW3), `transitions`
    (MAT-SW8's icon transitions).
  - `state_layer`, in `thumb` — Material's halo (MAT-SW4): `interaction` (its state layer and press ripple, Compose's ripple indication, `Switch.kt:191-198`; S2.12),
    `corners` (its shape), `min_width`, `min_height`, `transitions` (the block's fades, MAT-X9).
  - `label` — the switch's label text (SW-7, Q1.103): `text`, `opacity` (Kirigami dims the disabled label alone,
    Q1.103).
  - `state_label` — WinUI's On/Off content text: `text`.
  - `header` — WinUI's header (W1-124): `text`, `padding`.
- **Widget values:** `track_inset_px: Value<Px>` (Kirigami's track inset, an expression on the grid unit, Q1.99;
  ADD-A56); `label_gap_px: Value<Px>`; `pre_content_margin_px`, `post_content_margin_px: Value<Px>` (WinUI, W1-124).
  The thumb's slide is `thumb.transitions` (group `Geometry`), like every value change animated on a part (Entry
  format), not a presentation event; on `windows-11`, whose knob also changes size with its own timing, the slide
  is the thumb's `transform` per `{checked}` with a `Transform` transition (W1-123: one `Geometry` timing per style,
  S2.7). Where the connector lays the thumb out, so that no preset moves it twice: on every preset but
  `windows-11`, at the end of the track its state puts it (the start while unchecked, the end under `{checked}`,
  mirrored in RTL), and the laid-out box moving is the slide; on `windows-11`, at the track's start in both states,
  as WinUI's template aligns the knob (`HorizontalAlignment="Left"`, `ToggleSwitch_themeresources.xaml:509`), the
  `{checked}` translation being the offset to the end (the template's `KnobTranslateTransform` under `On`, `:443-445`),
  so its laid-out box changes only with its size.
- **Variants:** typed none; keyed `material-3` `@icons` (the switch with handle icons).
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular`, `large` (MAC-SW05). Others none.
- **Flags:** shared none; keyed none; state flags used: hover, pressed, focus_visible, disabled, checked.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (ToggleSwitch's CommonStates; ToggleStates is
  `checked`, not a group); others none (states combine; `material-3`'s state layers are a recency set, below);
  precedence: state models (S17). Recency sets: `material-3`: `state_layer = [focus_visible, hover]` (state layers,
  MAT-SW4; Material's recency sets, above: Compose's ripple on the thumb, `Switch.kt:191-198`; `pressed` combines).
- **may_be_absent:**
  - parts: `track_fill` (all but `kde-breeze`), `on_glyph` (all but `adwaita`, `ios-*`, `macos-*`; on those stated
    in the base and absent by a rule outside the condition that shows them: `!cond on_off_labels` on `ios-*`, `!cond
    differentiate_without_color` on `macos-*` and `adwaita`, whose reader sets it from `show-status-shapes`,
    GN-1.202), `off_glyph` (all but `adwaita`, `ios-*`, `macos-*`; on those by the same rules as `on_glyph`:
    `!cond on_off_labels` on `ios-*`, `!cond differentiate_without_color` on `macos-*` and `adwaita`, GN-1.202),
    `thumb_icon` (all but `material-3`; `material-3` by a rule outside `@icons`, MAT-SW3), `state_layer` (all but
    `material-3`), `state_label` (all but `windows-11`), `header` (all but `windows-11`), `label` (`adwaita`,
    `macos-*`: GtkSwitch and NSSwitch draw no label of their own).
  - fields: `thumb.shadow` (`kde-breeze`, `windows-11`: Kirigami's and WinUI's knobs have no shadow in their rows,
    Q1.99–Q1.103, W1-120–W1-124), `thumb.stroke` (`adwaita`, `windows-11`, `ios-*`, `material-3`: their knobs have
    no rim in their rows, GN-1.198–GN-1.207, W1-120–W1-124, SW-1–SW-7, MAT-SW1–MAT-SW9; Breeze's and AppKit's rims are
    Q1.101 and MAC-SW02).
  - values: `track_inset_px` (all but `kde-breeze`), `pre_content_margin_px` (all but `windows-11`),
    `post_content_margin_px` (all but `windows-11`), `label_gap_px` (`adwaita`, `macos-*`: no label, as `label`
    above).
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
    inset inside the box, MAC-C06), `opacity` and `transform` (per state while drawn), `transitions` (its entrance and
    exit: absent while neither checked nor indeterminate, it enters from transparent and leaves to transparent by
    these, S2.7).
  - `label` — the text: `text`, `opacity` (Kirigami dims the disabled label alone, Q1.22), `focus` (where Kirigami
    rings the label, Q1.21).
  - `state_layer` — Material's halo (MAT-C2): `interaction` (its state layer and press ripple, Compose's ripple indication, `Checkbox.kt:554-579`; S2.12),
    `corners` (its shape), `min_width`, `min_height`, `transitions` (the block's fades, MAT-X9).
- **Widget values:** `label_gap_px: Value<Px>`; `indicator_cell_px: Value<Px>` (the cell the drawn box is centred
  in, K1.53); `presentations: Vec<PresentationTransition>`: event `Appear` (iOS's selection column sliding in when
  edit mode starts, CK-2†).
- **Variants:** typed `selection` (the round check of a list in selection mode: GNOME `.selection-mode`, iOS's list
  multiselect circle: CK-1, CK-2†, L-12), `warning` (KDE's neutral highlight), the content variant `with_label` (a
  check with a label: GTK's `.text-button` padding, GN-1.61; the connector sets it from the content, ADD-A61).
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular`, `large`. `ios-*`: the size follows Dynamic Type
  (`uifm_scaled`, `token.text.scaling`, S9.4, R10). Others none.
- **Flags:** shared none; keyed none; state flags used: hover, pressed, focus_visible, disabled, checked,
  indeterminate, invalid (Material's error, MAT-C5).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (CheckBox's CombinedStates, which name the check
  state with each of them, so `checked` and `indeterminate` combine); others none (states combine; `material-3`'s
  state layers are a recency set, below); precedence: state models (S17). Recency sets: `material-3`:
  `state_layer = [focus_visible, hover]` (state layers, MAT-C2; Material's recency sets, above: Compose's ripple,
  `Checkbox.kt:554-579`; `pressed` combines).
- **may_be_absent:**
  - parts: `state_layer` (all but `material-3`), `mark` (every preset by a rule `@ !{checked,indeterminate}`),
    `label` (`ios-*`: the list's multiselect circle, its only checkbox, has no label, CK-1, CK-2†).
  - fields: `body.fill` (the value phase names the presets from the facts: the rows fill only the box, W1-037,
    GN-1.55, K1.51, MAC-C02, MAT-C3), `body.focus` (`kde-breeze`, `material-3`: Breeze and Material ring the box,
    K1.57, MAT-X10), `box.focus` (`adwaita`: GTK rings indicator and label together, the `body`, GN-1.61;
    `kde-breeze` under `cond flavor:kirigami`, where Kirigami rings the label, Q1.21), `label.focus` (all but
    `kde-breeze`; `kde-breeze` by a rule outside `cond flavor:kirigami`, Q1.21), `box.shadow` (`kde-breeze` by a
    rule outside `{hover}`, K1.52; the value phase names the others from the facts; AppKit's bevel is MAC-C05).
  - values: `indicator_cell_px` (all but `kde-breeze`), `label_gap_px` (`ios-*`: no label, as `label` above).
- **State-model source:** `adwaita`: `_checks.scss` (`check`, `radio`, `checkbutton`, `:checked`, `:indeterminate`,
  `:hover`, `:active`, `:disabled`, `.selection-mode`, `prefers-contrast: more`); `kde-breeze`: Breeze's
  `Helper::renderCheckBox` (`hp:831-981`) and `Style` `CE_CheckBox`; flavour `kirigami`: qqc2 `CheckBox.qml`,
  `CheckIndicator.qml`; `macos-*`: `NSButton` checkbox `StateValue` on, off, mixed × highlighted × enabled × control
  size × key window × appearance, with the whole-control rendering (S15); `ios-*`: the list cell's selected, editing
  and disabled states, measured; `windows-11`: CheckBox's CombinedStates (12 states); `material-3`: the checkbox
  tokens (selected, unselected, indeterminate × enabled, hover, focus, pressed, disabled × error).
- **Today's fields:** `background_color` → `body.fill`; `font` → `label.text.font`; `indicator_color` →
  `mark.glyph.fill`; `indicator_width` → `box.min_width`, `box.min_height` (on `kde-breeze` the cell:
  `values.indicator_cell`, K1.53); `check_mark_stroke_width` → `mark.glyph.stroke.width`; `label_gap` →
  `values.label_gap`; `checked_background` → `box.fill @ {checked}`; `disabled_opacity` → `body.opacity @
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
    `min_height` (growing by state, W1-041), `stroke`, `transform` (its scale per state while drawn), `transitions`
    (its scale animation, W1-041; absent while unchecked, it enters from transparent by these, S2.7).
  - `label` — the text: `text`, `opacity` (Kirigami dims the disabled label alone, Q1.22), `focus`.
  - `state_layer` — Material's halo (MAT-C2): `interaction` (its state layer and press ripple, Compose's ripple indication, `RadioButton.kt:163-169`; S2.12),
    `corners` (its shape), `min_width`, `min_height`, `transitions` (the block's fades, MAT-X9).
- **Widget values:** `label_gap_px: Value<Px>`.
- **Variants:** typed `warning` (KDE's neutral highlight: the ring's pen and tint and its hover ring, K1.52, K1.56;
  `checkbox`'s `warning`), the content variant `with_label` (as on `checkbox`, GN-1.61).
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular`, `large`. Others none.
- **Flags:** shared none; keyed none; state flags used: hover, pressed, focus_visible, disabled, checked.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (RadioButton's CommonStates; CheckStates is
  `checked`, not a group); others none (states combine; `material-3`'s state layers are a recency set, below);
  precedence: state models (S17). Recency sets: `material-3`: `state_layer = [focus_visible, hover]` (state layers;
  Material's recency sets, above: Compose's ripple, `RadioButton.kt:163-169`; `pressed` combines).
- **may_be_absent:**
  - parts: `state_layer` (all but `material-3`), `dot` (every preset by a rule `@ !{checked}`).
  - fields: `ring.focus` (`adwaita`: GTK rings indicator and label together, the `body`, GN-1.61; `kde-breeze` under
    `cond flavor:kirigami`, where Kirigami rings the label, Q1.21), `body.focus` (`kde-breeze`: Breeze rings the
    ring, K1.57, K3.20), `label.focus` (all but `kde-breeze`; `kde-breeze` by a rule outside `cond flavor:kirigami`,
    Q1.21), `ring.shadow` (`kde-breeze` by a rule outside `{hover}`, K1.52; the value phase names the others from the
    facts); each preset states the form its dot is drawn in, a glyph or a cornered fill, and the others `Absent`:
    `dot.glyph` (`windows-11`, `material-3`: their dot is a round fill, W1-042, MAT-C1; `kde-breeze` where the value
    phase reads Breeze's dot as a fill, `renderRadioButton`, `hp:995-1075`), `dot.fill` (`adwaita`: its dot is a
    path glyph, GN-P4-7†; `kde-breeze` where the value phase reads Breeze's dot as a glyph), `dot.corners`
    (`adwaita`, `macos-*`: their dot is a glyph, GN-P4-7†, MAC-C07; `kde-breeze` where the value phase reads
    Breeze's dot as a glyph).
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
  - `body` — the group's box (the root), holding the header and the radios, which are `radio` elements in `context
    radio_group.body` (Nested controls): `padding`, `bindings`.
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
- **Notes:** the group's radios are `radio` elements in `context radio_group.body` (Nested controls: a control that
  also stands alone), their look `radio`'s rules there; the group's gaps and header placement stay its values. Group 7
  had placed Windows' `RadioButtons` (W2-25) header colour, disabled colour, padding, `row_gap` and `column_gap` on
  `checkbox`.

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
    host: Kirigami's search dialog field, Q1.118; a GTK linked box's overlap, each entry's negative start or top
    margin, GN-1.42), `min_width`, `min_height`, `max_width`, `opacity` (today's
    `disabled_opacity`), `focus`, `cursor`, `bindings` (GNOME's error, warning and success classes rebind the accent
    for the subtree), `transitions` (the editable label's title-to-field change, RL-2†), `interaction` (the filled field's tokens' hover layer, Material's, S2.12, MAT-I11; drawn beneath the field's content, as material-web draws it, `field/internal/field.ts:134-150`).
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
    trailing icons, a search field's magnifier): `icon` (Kirigami's search icon's size `= kirigami_label_icon_size(body_font_height)` under `cond flavor:kirigami`, the part declaring `data = { icon = ["body_font_height"] }` (Kirigami's `sizeForLabels`, which follows the user's font, Q1.1, ADD-A100), Q1.15, Q1.17), `fill`, `padding`, `opacity`, `cursor`, `transitions`.
  - `caps_lock_indicator` — the caps-lock glyph: in a secure field (GNOME, GN-2.23; macOS's `NSSecureTextField`,
    MAC-I03) and, on macOS, at the insertion point of any text field (the insertion indicator's, MAC-X14): `glyph`,
    `padding`, `opacity` (GNOME draws it dimmed, more opaque in high contrast, GN-2.23).
  - `autofill_affordance` — the AutoFill and strong-password affordance inside the field (macOS, iOS): `icon`,
    `fill`, `corners`, `padding`, `text` (the strong-password suggestion's label, I-9†).
  - `writing_tools_affordance` — macOS's Writing Tools affordance (MAC-P3-8†): `icon`, `fill`, `opacity`,
    `transitions`.
  - `progress` — GNOME's entry progress line, its extent the progress fraction the connector draws: `fill`,
    `corners`, `min_height`, `margin` (its bottom margin, GN-1.48).
  - `caret` — the line caret: `fill` (base `= token.caret.color`), `min_width` (the caret's width, base
    `= token.caret.width`, with rules under KDE's tablet mode, Q1.14), `min_height` (its height against the line,
    X-CA-1), `corners`; its shape is these fields' rules per state and flag (S4.2: macOS's insertion indicator,
    MAC-X14; the overwrite and tablet-mode forms), its colour and width bases and its blink the caret token's
    (`token.caret`, R12, ADD-A24). The caret has no font of its
    own: a length that follows the text (GNOME's width from the line height, GN-3.36; iOS's height against the line,
    X-CA-1) measures the `text` part's font, `widget.text.line_height` (S2.2), which the
    caret declares as its data (`data = { caret = ["widget.text.line_height"] }`, S8). Absent by rule where the
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
  - `preedit` *(element)* — the IME composition text (GN-1.281†, K3.48†): `text` (its decoration, following its own
    font where the platform draws the line from the font: `thickness` `= element.font_underline_thickness`, `offset`
    `= element.font_underline_offset`, S2.8, Notes; the part declaring `data = { preedit = ["font_underline_offset",
    "font_underline_thickness"] }`), `fill`.
  - `misspelling` — the spell-check mark: `text` (its decoration, which lies under the entered text and follows the
    `text` part's font where the platform draws the line from the font: `thickness` `=
    widget.text.font_underline_thickness`, `offset` `= widget.text.font_underline_offset`, S2.2, S2.8, Notes; the part
    declaring `data = { misspelling = ["widget.text.font_underline_offset", "widget.text.font_underline_thickness"]
    }`).
  - `grammar_mark` — macOS's grammar mark (MAC-X28): `text` (its decoration, as `misspelling`'s, the part declaring
    the same data, `data = { grammar_mark = [ … ] }`).
  - `correction` — iOS's autocorrection mark and bubble, Material's suggestion underline: `text` (its decoration, as
    `misspelling`'s, the part declaring the same data, `data = { correction = [ … ] }`), `fill`, `corners`.
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
  field shows its `clear_field` button: I-2, W1-030, K1.46; WinUI's ButtonStates; `Absent` where the field has no
  clear button, may_be_absent, and `Never` only where the platform's own setting says so, UIKit's
  `clearButtonMode`); `reveal_button_visibility:
  Value<ShowWhen>` (its `reveal` button, K1.48); `mask_character: Vec<char>` (a preference list: the first the font
  covers; K1.48, MAC-I03, I-9, MAT-I19, GN-2.58†, W2-16); `mask_reveal_ms: Value<Ms>` (how long the last typed
  character shows; GNOME's password hint timeout; K1.48, I-9, MAT-I19, GN-2.23); `peek_while_pressed: Value<bool>`
  (W2-16); `focus_visible_when` (`Value<FocusVisibleWhen>`, S2.11; GN-1.50); `min_font_size: Value<FontSize>` (the
  shrink-to-fit floor, I-7); `default_width_px: Value<Px>` (implicit width, Q1.12); `magnifier_zoom: Value<f32>` (a
  factor), `magnifier_offset_px: Value<Px>` (above the caret or touch point) (GN-P3-26, MAT-I17); `search_delay_ms:
  Value<Ms>` (Kirigami's search field debounce, Q1.17); `context_menu_items: Vec<Key>` (the text context menu's items
  in order, a section break the key `separator`; each preset declares its keys in `keys`, S10: `adwaita` `cut`,
  `copy`, `paste`, `delete`, `separator`, `select_all`, `change_direction`, `insert_emoji`, GtkText's menu, the
  last two hidden while their action is disabled, GN-P4-8†, `G/gtktext.c:6372-6427`; the value phase lists the
  other presets' keys from the facts, no row naming them); `strength_meter_segments: Value<u32>` (the meter's
  segment count, K-KA-11†);
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
  MAC-X17; which ones, per generation, from the probe). `ios-*`: none (the height follows Dynamic Type, I-10: `uifm_scaled` over the
  category `token.text.scaling` uses, S9.4, R25.15). `material-3`: its density steps (keyed). `windows-11`: density `compact` (WinUI's compact sizing dictionary, W3-39). Others: none.
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
  - parts: `prefix` (all but `material-3`), `suffix` (all but `material-3`), `floating_label` (all but `material-3`),
    `supporting_text` (all but `material-3`, `windows-11`), `counter` (all but `material-3`, `kde-breeze`), `header`
    (all but `windows-11`), `caps_lock_indicator` (all but `macos-*`, `adwaita`; `adwaita` by a rule outside
    `secure`, GN-2.23; `macos-*` draws it in every field, MAC-X14, its secure field's own by a rule under `secure`,
    MAC-I03), `icon` (`windows-11`: WinUI's TextBox has no icon slot, its search box's query icon being a `button`,
    `@query`, W2-15; `macos-*` by a rule outside `context any search_field`: NSTextField draws no icon,
    NSSearchField its magnifier, MAC-N02; on the other presets the field draws the icons its content sets),
    `autofill_affordance` (all but `macos-*`, `ios-*`),
    `writing_tools_affordance` (all but `macos-*`), `dictation_effect` (all but `macos-*`), `grammar_mark` (all but
    `macos-*`), `progress` (all but `adwaita`), `caret` (every preset, by rule where the platform draws none: outside
    the focused field, in an inactive window, `{overwrite}`; `{text_selected}` on all but `kde-breeze`: GN-3.73†,
    MAT-P3-21†, P3-9†, MAC-P3-11†, W-P3-9†), `secondary_caret` (all but `adwaita`, `kde-breeze`), `block_caret` (all
    but `adwaita`, `kde-breeze`; there absent by a rule `!{overwrite}`), `drop_caret` (all but `ios-*`, `adwaita`,
    `windows-11`, `macos-*`: the second caret at the drop position, P3-9†, GN-3.73†, W-P3-9†, MAC-P3-11†; on those by
    a rule `!{drop_target}`; a platform that only recolours its caret on a drop target states `caret.fill @
    {drop_target}` instead), `floating_caret` (all but `ios-*`), `caret_accessory` (all but `ios-*`), `correction`
    (all but `ios-*`, `material-3`), `prediction` (all but `ios-*`, `macos-*`), `selection_handle` (`macos-*`, which
    draws none; every other preset outside touch input, by rules `@ !cond input:touch`: `windows-11`, `kde-breeze`
    and `adwaita` draw handles for touch selections only, and `ios-*` and `material-3` state the rule where the value
    phase's facts show UIKit and Android draw none for a pointer's selection), `magnifier` (all but `adwaita`,
    `material-3`, `ios-*`), `strength_meter` (all but `kde-breeze`), `strength_meter_fill` (all but `kde-breeze`).
  - fields: `body.shadow` (`adwaita` by rules outside `{drop_target}` and `cond contrast_high`, GN-1.46, GN-1.51;
    `kde-breeze` under `flat`, K1.41; the value phase names the others from the facts), `body.focus` (the value
    phase names the presets from the facts where the field marks focus by its stroke alone, as `focus_border_color`
    maps below).
  - values: `notch_padding_px` (all but `material-3`), `floating_label_gap_px` (all but `material-3`), `affix_gap_px`
    (all but `material-3`), `header_gap_px` (all but `windows-11`), `supporting_text_gap_px` (all but `material-3`,
    `windows-11`), `leading_inset_px` (all but `ios-*`: UIKit's overlay-view inset, I-3; the other rows place a
    field's icons by the `icon` element's `padding`, MAT-I7), `trailing_inset_px` (all but `ios-*`, `windows-11`:
    UIKit's overlay views and WinUI's clear button, I-2, I-3, W1-030), `clear_button_visibility` (`material-3`:
    Material 3 has no built-in clear button, the application supplies a trailing icon, MAT-I7, K1.46†'s cross-check;
    `adwaita`, `macos-*` by a rule outside `context any search_field`: GtkEntry and NSTextField have no clear button,
    GtkSearchEntry's and NSSearchField's are the only ones, GN-2.22, MAC-N02), `reveal_button_visibility` (`macos-*`,
    `ios-*`, `material-3`: their secure fields have no reveal button of their own, MAC-I03, I-9†, MAT-I19†;
    `kde-breeze` reads it, K1.48†, and `adwaita` and `windows-11` draw one, GN-2.23, W2-16), `mask_reveal_ms`
    (`macos-*`, `windows-11`: NSSecureTextField and PasswordBox show no typed character and state no such delay,
    MAC-I03, W2-16), `min_font_size` (all but `ios-*`),
    `default_width_px` (all but `kde-breeze`), `magnifier_zoom` (all but `adwaita`, `material-3`, `ios-*`),
    `magnifier_offset_px` (all but `adwaita`, `material-3`, `ios-*`), `search_delay_ms` (all but `kde-breeze`),
    `strength_meter_segments` (all but `kde-breeze`), `strength_meter_gap_px` (all but `kde-breeze`). The
    peek-while-pressed flag is false, not absent, where the reveal button does not peek (a `Value<bool>`, W2-16).
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
  `button`'s presentations), the completion popup an `autocomplete` in `context input` (MAC-P3-7†). The decorations
  of `preedit`, `misspelling`, `grammar_mark` and `correction` follow the font (S2.8): where the platform draws the
  line from the font's own metrics (Pango's, Qt's `QFontMetrics`, AppKit's `NSFont`, DirectWrite's, Android's
  `Paint`), their `thickness` and `offset` are the references the parts state, and a number is stated only where the
  platform draws a fixed line (macOS's spelling and grammar dots, MAC-X28); the value phase states which per preset
  from the facts (GN-1.281†, K1.49†, W1-151†, MAT-I18†). Provenance: group
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
  - `caret` — as on `input` (its shape these fields' rules, S4.2; macOS's insertion indicator, MAC-X14): `fill`,
    `min_width`, `min_height`, `corners`.
  - `secondary_caret` — as on `input` (GtkTextView's split cursor, QTextEdit's bidi caret, GN-3.37): `fill`,
    `min_width`, `min_height`.
  - `block_caret` — as on `input`: `fill`, `text`.
  - `floating_caret` — as on `input` (UITextView's floating cursor, X-CA-3): `fill`, `opacity`, `corners`.
  - `caret_accessory` — as on `input` (X-CA-3): `icon`, `padding`.
  - `dictation_effect` — as on `input` (the dictation effect at the caret of a text area, MAC-X14): `fill`,
    `transitions`.
  - `caps_lock_indicator` — as on `input`: the caps-lock glyph macOS's insertion indicator draws at the insertion
    point of a text view too (MAC-X14): `glyph`, `padding`.
  - `preedit` *(element)* — as on `input` (its decoration following its own font, the part declaring the same data,
    `data = { preedit = [ … ] }`): `text`, `fill`.
  - `misspelling` — as on `input` (its decoration following the `text` part's font, the part declaring the same
    data, `data = { misspelling = [ … ] }`): `text`.
  - `grammar_mark` — as on `input` (NSTextView's grammar mark, MAC-X28; the same data, `data = { grammar_mark = [ …
    ] }`): `text`.
  - `correction` — as on `input` (the same data, `data = { correction = [ … ] }`): `text`, `fill`, `corners`.
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
  - `find_match` *(element)* — a find-and-replace match; the current match is `current` (macOS's find indicator,
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
  `default_height_px: Value<Px>` (Q1.12); `header_gap_px`, `supporting_text_gap_px: Value<Px>` (as on input,
  W1-036); `empty_ranges_draw_carets: Value<bool>` (P4-13);
  `magnifier_zoom: Value<f32>`, `magnifier_offset_px: Value<Px>`
  (as on input;
  GN-P3-26, MAT-I17); `notch_padding_px`, `floating_label_gap_px`, `affix_gap_px: Value<Px>` (as on input, Material's
  multi-line field, MAT-I3, MAT-I4, MAT-I8); `context_menu_items: Vec<Key>` (as on input, its own keys: `adwaita`
  `cut`, `copy`, `paste`, `delete`, `separator`, `undo`, `redo`, `separator`, `select_all`, `insert_emoji`,
  GtkTextView's menu, the last hidden while its action is disabled, GN-P4-8†, `G/gtktextview.c:9248-9310`; the value
  phase lists the other presets' keys from the facts);
  `presentations: Vec<PresentationTransition>`: keyed
  event `@find_indicator` (the find indicator's bounce on the current match: queried with the `find_match` element
  that becomes `current`, the event that match's, its step on that element's own part `find_match`; MAC-X15, TA-2).
  Material's multi-line field has no minimum-rows or resize value: its minimum rows are the application's
  (`minLines`, a parameter of Compose's `TextField`), and the resize handle MAT-I16 measures is material-web's
  textarea, which Compose does not draw (R1.6).
- **Variants:** typed `flat` (no frame: iOS `none`); typed on `invisible` `space`, `tab`, `line_break`,
  `no_break_space`; typed on the `icon` element `leading`, `trailing` (as on `input`). Keyed: `material-3` `@filled`
  (as on `input`); `adwaita` `@inline` (the inline text view, GN-1.54); `ios-*` `@rounded_rect` (iOS 17's rounded rect);
  `ios-*` and `macos-*` on `highlight` one key per highlight style (declared by the preset; macOS's an open item,
  MAC-X28). As on `input` (each stated on this widget, R20): typed on `preedit` `selected_clause`, on
  `selection_handle` the identities `start`, `end`, `insertion` (GN-2.42); keyed `adwaita` on `preedit` `@whole`,
  `@selection`, `@prediction`, `@prefix`, `@suffix`, `@spelling_error`, `@compose_error` (GN-1.281†); `windows-11` on
  `preedit` one key per IME clause attribute.
- **Sizes and densities:** no sizes (`macos-*`: control sizes do not apply to NSTextView); `windows-11`: density `compact` (WinUI's compact sizing dictionary, W3-39).
- **Flags:** shared `empty` (as on input), `overwrite` (the overwrite caret: GNOME, KDE; ADD-A23), `composing` (GNOME
  GN-1.281†, KDE K3.48†), `text_selected` (as on input), `current` (on `find_match`, the current match: macOS, iOS);
  keyed `macos-*` `@incremental_find` (an incremental find runs: `find_dimming` drawn, MAC-X15); state flags used:
  hover, focused, focus_within, focus_visible, disabled, read_only, drop_target.
- **Exclusive groups:** `windows-11`: `[disabled, focused, hover]` (TextBox CommonStates: the multi-line box is a
  TextBox); others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `gutter` (all but `adwaita`), `find_dimming` (all but `macos-*`; `macos-*` by a rule
  outside `{@incremental_find}`, MAC-X15), `highlight` (all but `ios-*`, `macos-*`), `header` (all but `windows-11`),
  `supporting_text` (all but `material-3`, `windows-11`), `placeholder` (`adwaita`, `macos-*`, `ios-*`: GtkTextView,
  NSTextView and UITextView draw no placeholder, the value phase confirming from the facts; WinUI's multi-line
  TextBox draws one, W1-036), `grammar_mark` (all but `macos-*`), `caret` (every preset by rules as on `input`:
  outside the focused view, `{overwrite}`, `{text_selected}` on all but `kde-breeze`, GN-3.73†), `secondary_caret`
  (all but `adwaita`, `kde-breeze`), `floating_caret` (all but `ios-*`), `caret_accessory` (all but `ios-*`),
  `dictation_effect` (all but `macos-*`), `caps_lock_indicator` (all but `macos-*`: the insertion indicator's,
  MAC-X14), `invisible` (all but `macos-*`, `ios-*`), `writing_tools_glow` (all but
  `macos-*`, `ios-*`), `block_caret` (all but `adwaita`, `kde-breeze`; there by a rule `!{overwrite}`, and on
  `adwaita` also `!{focused}` and `{text_selected}`: GtkTextView draws it only while focused and without a
  selection, GN-P4-4†), `drop_caret` (all but `ios-*`, `windows-11`, `macos-*`, `adwaita`; on those by a rule
  `!{drop_target}`, P3-9†, W-P3-9†, MAC-P3-11†, GN-1.53), `prefix` (all but `material-3`), `suffix` (all but
  `material-3`), `floating_label` (all but `material-3`), `counter` (all but `material-3`), `icon` (all but
  `material-3`), `correction` (all but `ios-*`, `material-3`), `prediction` (all but `ios-*`, `macos-*`),
  `selection_handle` (`macos-*`, which draws none; every other preset outside touch input, by rules `@ !cond
  input:touch`: `windows-11`, `kde-breeze` and `adwaita` draw handles for touch selections only, and `ios-*` and
  `material-3` state the rule where the value phase's facts show UIKit and Android draw none for a pointer's
  selection; one wording with `input`, one platform behaviour), `magnifier` (all but `adwaita`, `material-3`,
  `ios-*`), `find_match` (all but `macos-*`, `ios-*`: NSTextFinder's and UIFindInteraction's indicators); fields:
  `body.shadow` (the value phase names the presets from the facts), `body.fill` (every preset by a rule under
  `flat`), `body.focus` (the value phase names the presets from the facts; Breeze rings the frame, K3.20); values:
  `document_margin` (every member where the platform states no inset inside the document: the value phase names the
  presets; Qt's, AppKit's and UIKit's are K-P4-9†, MAC-I10†, P3-1†), `line_padding_px` (all but `macos-*`,
  `ios-*`), `scrollbar_inset` (every member on all but `windows-11`), `default_width_px` (all but `kde-breeze`),
  `default_height_px` (all but `kde-breeze`), `notch_padding_px` (all but `material-3`), `floating_label_gap_px`
  (all but `material-3`), `affix_gap_px` (all but `material-3`), `magnifier_zoom` (all but `adwaita`, `material-3`, `ios-*`), `magnifier_offset_px` (all but
  `adwaita`, `material-3`, `ios-*`), `header_gap_px` (all but `windows-11`), `supporting_text_gap_px` (all but
  `material-3`, `windows-11`). The lists (`context_menu_items`, `presentations`) are empty, not absent, where the
  platform has none.
- **State-model source:** `windows-11` as input; `adwaita` `_views.scss` (the text view, its border nodes, inline,
  drop); `kde-breeze` the QTextEdit frame branches and qqc2 `TextArea.qml`; `material-3` the textarea form of the
  text-field tokens; `macos-*` NSTextView (editable, selectable, first responder, window key) inside NSScrollView (its
  border type); `ios-*` UITextView (editable, `borderStyle`, `UIFindInteraction` states, `textHighlightAttributes`).
- **Today's fields:** `border.*` → `body.stroke`, `body.corners`, `body.padding` (the fixed frame inset; the scrolling
  part moves to `document_margin`), `body.shadow`; the background → `body.fill`. Everything today read from `input` is
  stated on `text_area`'s own
  parts (no model inheritance, R20).
- **Notes:** links in running text are `link @ variant inline context text_area` (the former link part goes); the
  enclosing NSScrollView's border is `scroll_view.frame`, selected by `scroll_view`'s own variants (`flat`, `line`,
  `@bezel`, `@groove`, MAC-I08) where the text view is in one, `text_area` declaring none of them;
  the current find match is `text_area.find_match @ {current}` (MAC-X15). Provenance: the glow is group 3's (rename
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
    the non-flat, non-editable combo), `padding`, `margin` (a GTK linked box's overlap, the drop-down's button's
    negative start or top margin, GN-1.42), `min_width`, `min_height`, `opacity`, `focus`, `bindings` (Kirigami:
    Button set, View set when editable), `transitions`, `interaction` (the filled field's tokens' hover layer, Material's, S2.12, MAT-I11; drawn beneath the field's content, as material-web draws it, `field/internal/field.ts:134-150`).
  - `label` — the current value's text: `text`.
  - `placeholder` — WinUI's placeholder text: `text`.
  - `icon` — the leading icon (Material's), or the current item's icon (Qt's `CE_ComboBoxLabel`, Q1.122; AppKit's
    `NSPopUpButton`): `icon`, `opacity` (its disabled opacity, as on input: MAT-CB1, MAT-I7).
  - `arrow_area` *(element)* — the arrow's sub-control, hovered and pressed on its own: macOS's accent-filled arrow
    area of the pop-up, NSComboBox's separate arrow button, Breeze's editable combo's arrow sub-control, which draws
    no box of its own (its drawing fields `Absent`, S6.3) but whose own hover recolours the arrow (`SC_ComboBoxArrow`,
    `breezestyle.cpp:7884`, K1.240): `fill`, `stroke`, `corners`, `min_width`.
  - `arrow`, in `arrow_area` — the indicator glyph (single or double chevron; on macOS the AppKit rendering, the
    system control `AppKitPopUp`: `GlyphSource::SystemControl { control: AppKitPopUp, replaces: [body, arrow_area,
    arrow] }`, as S15's image is the whole button, so the connector draws none of those parts' own fills, strokes or
    shadows under it and passes the `body`'s laid-out width as `render_system_control`'s `width`; the arrow area is
    the image's `ControlImage.arrow`), a mark that takes the combo box's states (Nested controls: it is queried with the
    combo box's element, so `{hover}` is the combo's; nested in `arrow_area`, `context combo_box.arrow_area {hover}`
    selects the arrow area's own hover, and where no arrow area is drawn it is queried in the root's scope, Nesting):
    `glyph` (its fill per state), `transform` (its rotation while `open`), `transitions` (WinUI's animated chevron,
    W1-132).
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
  `flat` (GNOME's flat drop-down, KDE frameless, macOS borderless), `warning` (KDE's neutral highlight; input's `warning`), `tinted`, `primary` (iOS's
  tinted and prominent trigger configurations, `button`'s meanings, CB-2), `glass`, `clear` (iOS 26's glass trigger
  styles, `button`'s meanings, CB-2). Keyed: `adwaita` `@gtk_combo_box`
  (GN-1.238), `@raised` (the drop-down kept raised in a toolbar, GN-1.151); `macos-*` `@pull_down` (AppKit's
  pull-down `NSPopUpButton`, a pop-up button control with a pull-down menu, MAC-CB01, MAC-N29†: one platform, so keyed;
  iOS's pull-down button is `button` `menu`, B-18†); `kde-breeze` `@dialog_mode`, `@page_mode`
  (the form combo row's display modes, K-KA-12†); `material-3` `@filled`; `ios-*` `@gray`
  (the trigger's `UIButton.Configuration` styles of B-1, as `button` declares them, with its `exclusive_variants`
  groups, CB-2).
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular`, `large`. `material-3`: density (keyed). `windows-11`: density `compact` (WinUI's compact sizing dictionary, W3-39).
  Others: none.
- **Flags:** shared `empty` (Material's select with no value chosen: its label rests inside the field, MAT-CB1); state
  flags used: hover, pressed, focused, focus_visible, disabled, open (the popup shown); hover and pressed on the
  `arrow_area` element too (K1.240, NSComboBox's arrow button).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (ComboBox CommonStates; ComboBoxItem's are
  `menu`'s in context), `[focus_visible, focused]`
  (FocusStates); its DropDownStates are the flag `open`, its InputModeStates the conditions `cond input:touch` and
  `cond input:game_controller`, its EditableModeStates the variant `editable`; others none (states combine);
  precedence: state models (S17).
- **may_be_absent:** parts: `placeholder` (all but `windows-11`), `header` (all but `windows-11`), `icon` (all but
  `material-3`, `kde-breeze`, `macos-*`: Qt's and AppKit's triggers draw the current item's icon, Q1.122; no row of
  GNOME, Windows or iOS draws an icon in the trigger), `floating_label` (all but `material-3`), `supporting_text`
  (all but `material-3`, `windows-11`), `arrow_area` (all but `macos-*`, `kde-breeze`; `kde-breeze` by a rule outside
  `editable`, K1.240); fields: `body.shadow` (`adwaita` by the toolbar rule, GN-1.151; `kde-breeze`, `macos-*` by a
  rule under `flat`, K1.241, MAC-CB05; the value phase names the others from the facts), `arrow_area.corners`
  (`kde-breeze`, whose arrow area draws no box, K1.240); values: `header_gap_px` (all but `windows-11`),
  `supporting_text_gap_px` (all but `windows-11`, `material-3`), `notch_padding_px` (all but `material-3`),
  `floating_label_gap_px` (all but `material-3`).
- **State-model source:** `windows-11` the ComboBox and ComboBoxItem visual state groups
  (`ComboBox/ComboBox_themeresources.xaml`); `adwaita` `_dropdowns.scss`, `_buttons.scss` (the trigger is a toggle
  button, checked while open), `_deprecated.scss`; `kde-breeze` `breezestyle.cpp` combo-box branches (editable, frame,
  mouse over, focus, sunken or open, neutral) and the helper's button frame; qqc2 `ComboBox.qml` for `kirigami`;
  kirigami-addons' `FormComboBoxDelegate` (display modes); `material-3` the outlined and filled select tokens;
  `macos-*` NSPopUpButton (pulls down, bordered, control size, key window) and NSComboBox; `ios-*`
  `UIButton.Configuration` with its indicator and menu states.
- **Today's fields:** `background_color` → `body.fill`; `font` → `label.text.font` (colour `label.text.fill`);
  `min_height_px`, `min_width_px` → `body.min_height`, `body.min_width`; `arrow_icon_size_px` → `arrow.glyph.size`;
  `arrow_area_width_px` → `arrow_area.min_width` (where no arrow area is drawn the part is absent, may_be_absent
  `arrow_area`); `disabled_opacity` →
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
  - `value_fill` — the active track (value fill, GTK's highlight): `fill`, `corners` (the fill's ends, MAC-SL03, MAT-S3),
    `min_height`, `max_height`, `min_width`, `max_width` (its thickness, as the track's), `transitions` (Geometry: the
    animated value change, SL-9; GN-1.106).
  - `fill_level` — GTK's second trough showing the fill level (GN-1.103): `fill`, `corners`.
  - `disabled_range` — the track outside iOS 26's enabled range (SL-7): `fill`.
  - `thumb` *(element)* — the handle (two under the range variant): `fill`, `corners`, `min_width`, `min_height` (its
    shape by corners and size: MAT-S1, SL-2, MAC-SL01), `transform` (GNOME's teardrop rotation, iOS 26's growing
    lens), `shadow` (a rim ring as a spread layer; K3.4), `stroke` (GN-1.96, K1.100, MAT-S10), `focus` (GN-1.104,
    K1.106), `transitions` (Geometry: the animated value change, SL-9; KDE's hover and focus fades, K1.106).
    `thumb_inner`, `state_layer` belong to `thumb` and are queried with it.
  - `thumb_inner` — WinUI's inner accent dot: `fill` (per state), `corners`, `min_width`, `min_height` (W1-067),
    `transform`, `transitions` (its scale per state and that scale's animation, W1-068).
  - `state_layer` — Material's halo round the handle (MAT-S7), named as `switch`'s, `checkbox`'s and `radio`'s: `fill`,
    `corners` (its shape), `opacity` (per state, MAT-S7), `min_width`, `min_height`. Unlike theirs it is no ripple: at
    the pin Compose's slider draws no indication on the handle under the default ripple theme
    (`OpacityFocusThemeConfiguration`, `Ripple.kt:360`), and only an inset focus ring, with the press, hover and drag
    indications off, under an inset-ring theme (`Slider.kt:930-946`, `:1435-1470`), so the halo is the tokens' and its
    states an exclusive group (above).
  - `tick` *(element)* — a tick mark (its fill depends on whether it lies within `value_fill`): `fill`, `corners` (a line
    or a dot, MAC-SL04, MAT-S5), `min_width`, `min_height` (today's `tick_mark_length_px`; GN-1.100, SL-6).
  - `stop_indicator` *(element)* — Material's stop dot (MAT-S4): `fill`, `corners`, `min_width`, `min_height`.
  - `value_label` — GTK's drawn value label (GN-1.102): `text`, `opacity` (dimmed).
  - `value_indicator` — Material's value bubble shown while dragging (MAT-S6): `fill`, `corners`, `min_height`,
    `text`, `transform` (its scale per state while drawn), `transitions` (its show and hide: absent by rule outside
    the states it shows in, it enters from transparent and leaves to transparent by these, S2.7; restated
    `@ cond reduced_motion`, MAT-S6). WinUI's thumb tooltip is a `tooltip` (Notes).
  - `header` — WinUI's header (W1-072): `text`.
  - `minimum_image`, `maximum_image` — iOS's minimum- and maximum-value images (SL-5): `icon`.
  - `inset_icon` — Material Expressive's icon inside the track (MAT-S11): `icon`, `padding`.
- **Widget values:** `tick_gap_px: Value<Px>` (ticks to track; GN-1.100, K1.103, MAC-SL04, SL-6);
  Breeze's tick option is the condition `cond system_setting:slider_tick_marks` (read from breezerc, ADD-A04),
  the `tick` elements absent by a rule outside it (K1.103); `value_label_gap_px: Value<Px>` (GN-1.102);
  `value_label_position: Value<ValueLabelPosition>` (GN-1.102, GN-1.107); `value_indicator_gap_px: Value<Px>` (MAT-S6);
  `thumb_track_gap_px: Value<Px>` (Material's handle–track gap,
  MAT-S2); `stop_indicator_gap_px: Value<Px>` (MAT-S4); `image_gap_px: Value<Px>` (SL-5); `header_gap_px: Value<Px>`
  (W1-072); `snap_to_ticks: Value<bool>` (SL-9); `repeat_delay_ms`, `repeat_interval_ms: Value<Ms>` (the trough's
  page repeat while the pointer holds it: K3.76†, K-P3-17†, GN-3.69†, MAC-X12; one home per fact, as `drag_hover_delay_ms`: on `adwaita` and
  `macos-*`, whose platform states one system pointer auto-repeat, GTK's trough constants, `gtkrange.c`, GN-3.69†,
  and `getPeriodicDelay`, MAC-X12, the bases are `= token.params.pointer_repeat.delay` and `=
  token.params.pointer_repeat.interval`, S4.6, `unit = "ms"`; the widget's own values only where its own source
  states another, as QAbstractSlider's on `kde-breeze`, K-P3-17†); `trough_click: Value<TroughClick>` (Shared value
  types; GN-P4-5†, R17); `range_min_gap_px: Value<Px>` (the least distance between
  a range slider's two thumbs, W1-153†); `range_thumb_order: Value<RangeThumbOrder>`, `RangeThumbOrder { StartAbove,
  EndAbove, ActiveAbove }` (which thumb is drawn above the other where they overlap, W1-153†);
  `thumb_tooltip_placement: Value<Placement>` (where WinUI's thumb tooltip goes, the host's value, W1-073). The
  value bubble's showing and hiding are its `transitions`, not an event (Entry format).
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
  focus_visible, dragged (macOS's thumb while dragged, MAC-SL07), disabled.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (Slider CommonStates; FocusEngagementStates is the
  flag `@focus_engaged`); `material-3`: `[pressed, focus_visible, hover]` (the halo's state layer, `state_layer`;
  MAT-X29 names no dragged layer on a slider); others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `fill_level` (all but `adwaita`), `value_label` (all but `adwaita`, `kde-breeze`;
  `kde-breeze` by a rule outside `context form.row variant @ka_form_card`, whose slider row alone draws one,
  K-KA-9†), `disabled_range` (all but `ios-26`: UISlider's enabled range is iOS 26's, SL-7), `minimum_image` (all
  but `ios-*`), `maximum_image` (all but `ios-*`), `thumb_inner` (all but `windows-11`), `header` (all but
  `windows-11`), `state_layer` (all but `material-3`; `material-3` by a rule outside `{hover}`, `{focus_visible}` and
  `{pressed}`, the states MAT-S7 draws it in, the flags of `material-3`'s group), `stop_indicator` (all but
  `material-3`), `inset_icon` (all but
  `material-3`), `value_indicator` (all but `material-3`; `material-3` by a rule outside the states it shows in);
  `thumb` (`ios-*` by a rule under `@thumbless`, SL-8); `tick` (`kde-breeze`, by a rule `!cond
  system_setting:slider_tick_marks`; `ios-13`, whose UISlider has no tick marks before iOS 26, SL-6); fields:
  `track.stroke` (all but `adwaita`; `adwaita` by a rule outside `cond contrast_high`, GN-1.108), `track.shadow`
  (all but `material-3`), `thumb.stroke` (`macos-*`: AppKit's knob rim is a shadow layer, MAC-SL02; `material-3`
  by a rule outside `{@overlapping}`, MAT-S10), `thumb.shadow` (`windows-11`: WinUI's thumb has a stroke and an
  inner dot and no shadow in its rows, W1-067–W1-069); values: `value_label_gap_px` (all but `adwaita`),
  `value_label_position` (all but `adwaita`, `kde-breeze`: GTK's value label and kirigami-addons' form slider,
  GN-1.102, K-KA-9†), `value_indicator_gap_px` (all but `material-3`), `range_min_gap_px` (`adwaita`, `macos-*`,
  `ios-*`: GtkScale, NSSlider and UISlider have no range form; `kde-breeze`, `material-3` where the value phase's
  facts show their range sliders keep no least distance between the thumbs; WinUI's is W1-153†),
  `range_thumb_order` (`adwaita`, `macos-*`, `ios-*`: no range form), `thumb_track_gap_px` (all but `material-3`),
  `stop_indicator_gap_px` (all but `material-3`), `image_gap_px` (all but `ios-*`), `header_gap_px` (all but
  `windows-11`), `thumb_tooltip_placement` (all but `windows-11`), `tick_gap_px` (`ios-13`, SL-6; `material-3`,
  whose tick marks lie on the track, MAT-S5), `repeat_delay_ms` (`ios-*`, `material-3`: no trough page step),
  `repeat_interval_ms` (`ios-*`, `material-3`: no trough page step), `trough_click` (`ios-*`, `material-3`: no
  trough page step).
- **State-model source:** `windows-11` Slider visual states (`CommonStyles/Slider_themeresources.xaml`); `adwaita`
  `_scale.scss` (hover, active, disabled, marks before and after, fine tune, vertical, high contrast); `kde-breeze`
  `breezestyle.cpp` slider branches and `breezehelper.cpp` handle rendering (mouse over, pressed, focus, neutral),
  breezerc's tick option, qqc2 `Slider.qml` and `RangeSlider.qml` for `kirigami`; `material-3` the slider tokens
  (baseline and Expressive sizes); `macos-*` NSSlider (tick mark position, control size, tint prominence, neutral
  value, key window); `ios-*` UISlider (`Style`, `TrackConfiguration`, `UIControl.State`).
- **Today's fields:** `fill_color` → `value_fill.fill`; `track_color` → `track.fill`; `thumb_color` → `thumb.fill`;
  `track_height_px` → `track.min_height` = `track.max_height` (and `value_fill`'s); `thumb_diameter_px` →
  `thumb.min_width`, `thumb.min_height` with `thumb.corners` full; `tick_mark_length_px` → `tick.min_height`;
  `disabled_opacity` → `body.opacity @ {disabled}`; `thumb_hover_color` → `thumb.fill @ {hover}` (or
  `thumb.fill @ context any slider {hover}` where the whole slider's hover drives it, GTK); `disabled_fill_color`,
  `disabled_track_color`, `disabled_thumb_color` → `value_fill.fill`, `track.fill`, `thumb.fill @ {disabled}`.
- **Notes:** WinUI's thumb tooltip is `tooltip @ context slider` (W1-073: the tooltip's style reused, its placement
  the slider's `thumb_tooltip_placement`; Nested controls). Provenance: group 6 wrote the colour sliders as keyed
  variants in the colour picker (GN-2.53†, W2-05);
  group 8 the thumb's shadow (K3.4) and the trough repeat (K3.76†, K-P3-17†, MAC-X12), one name with the repeat
  values; group 1 the trough click (GN-P4-5†, R17); the value change is the thumb's and `value_fill`'s transitions
  (SL-9), not a widget value. The active track is `value_fill` (not `fill`, which read `slider.fill.fill`), and
  Material's halo `state_layer`, one name with the other Material halos.

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
  - `track` — the full circle under the arc (AdwSpinner, Material, WinUI's background ring), a band along the circle
    that stays still (Entry format): `fill` (it paints the band, its width `values.stroke_width`; GN-1.226, MAT-SN1,
    W1-130).
  - `indicator` — the arc (`{indeterminate}`), the value arc or pie (`!{indeterminate}`, the determinate spinner), or
    Material's loading indicator: `glyph` (the arc, as the core states a spinner's arc, S2.7, S2.9: a stroked circle
    path, `GlyphSource::Path`; its `stroke`, a `GlyphStroke`, the arc's paint, width and cap, GN-1.227, MAT-SN1,
    MAT-SN3, MAT-SN4, Q1.121; its `draw_start` and `draw_extent` the arc's start and extent, which the `loop`'s
    keyframes set while `{indeterminate}`; under `!{indeterminate}` `draw_extent = element.progress`, the value arc
    following the progress, a pull to refresh's pull progress among them (Q2.54†), which the part declares, `data = {
    indicator = ["progress"] }`, ADD-B12; a pie, if MAC-SN04's render shows one, is the same route: the circle path at
    half the pie's radius, stroked as wide as the pie's radius with a `butt` cap, whose swept stroke fills the wedge;
    its `fill` absent, the path being stroked only, granted below; on `adwaita` its `size` the smaller side of the
    element's box, capped as AdwSpinner caps it, a glyph drawn to its box (S8: AdwSpinner measures at its minimum,
    `body.min_width` and `min_height`, and draws the arc at its allocation, `adw-spinner.c:70-81`,
    `adw-spinner-paintable.c:343`), the part then also declaring `width` and `height`), `fill` (the loading indicator's shape colour,
    MAT-N24; absent where the indicator is the arc, whose paint is its glyph's stroke), `transform` (its rotation,
    which the keyframes set), `transitions` (`Glyph`: the determinate arc's extent following a value change,
    MAT-SN6), `shape` (under Material's loading indicator a `ShapeKind::Material` shape, implemented in code, S2.4,
    which the `loop`'s keyframes change and interpolating transitions of the `Shape` property group morph, MAT-X12,
    R25.5).
  - `spoke` — a spoke (iOS) or fin (macOS), one part the connector draws `spoke_count` times: `fill` (the leading
    spoke's colour), `transform` (the spoke wheel's stepped rotation, which the `loop`'s keyframes set: SP-2a,
    MAC-SN02). The spoke's box is the body's box, so the transform's `origin`, a fraction of the part's own box
    (S2.6), is (0.5, 0.5), the spinner's centre; the spoke's own shape inside that box is the spoke values', and the
    connector rotates the spoke at index i by i of `spoke_count` equal parts of a turn beyond the transform, each
    spoke behind the leading one faded by `spoke_opacity_ramp`.
  - `icon` — the rotated or animated icon (KDE's process-working icon, GtkSpinner's animated SVG and its
    reduced-motion replacement) as a glyph: `glyph`, `transform` (its rotation, which the `loop`'s keyframes set:
    K1.230, Q1.119).
- **Widget values:** `diameter_px: Value<Px>` (the default size; today's `diameter_px`, Q1.120);
  `stroke_width_px: Value<Px>` (the track's width, the band its `fill` paints, Entry format; today's `stroke_width_px`;
  MAT-SN1's and MAT-SN3's track thickness; on `adwaita` an expression over the element's size, `= mul(element.width,
  <fraction>)`, the widget value's data declared under `values`, `data = { values = ["width", "height"] }`, GN-1.227,
  and the arc's width, `indicator.glyph.stroke.width`, refers to it (`= values.stroke_width`), AdwSpinner drawing both
  at one width; the arc's width is otherwise its glyph's, `indicator.glyph.stroke.width`, Q1.121, MAT-SN3);
  `stroke_weight: Value<u16>` (the icon weight GTK scales the
  stroke by; GN-1.227, GN-1.232); `indicator_track_gap_px: Value<Px>`
  (MAT-SN1, MAT-SN3); `spoke_count: Value<u32>`
  (SP-1a, MAC-SN01); `spoke_inner_radius_px`, `spoke_length_px`, `spoke_width_px: Value<Px>` (SP-1a, MAC-SN01);
  `spoke_cap: Value<LineCap>` (the spokes' end rounding, ADD-A34; SP-1a, MAC-SN01);
  `spoke_opacity_ramp: Vec<Value<Fraction>>` (per spoke behind the leading one; SP-1a, MAC-SN01); `wave_amplitude_px`,
  `wavelength_px: Value<Px>` (under `@wavy`, MAT-SN2); `loop: Value<Loop>` (S2.7: the motion while `{indeterminate}`;
  a widget value's loop is on no part, so every keyframe names its part, `Keyframe.part` stated; its keyframes on
  part `indicator`: the arc's extension and contraction, `glyph.draw_start` and `glyph.draw_extent`; its rotation,
  `transform`; steps; the four-colour cycle, the paint of `glyph.stroke`; the loading indicator's keyframes naming
  shapes (MAT-X12, R25.5); on part `icon`, KDE's rotating icon, its `transform`; on part `spoke`, the spokes'
  stepped rotation, its `transform`; `Absent` by a rule `@ cond animations_off`, so no loop runs, and by a rule
  `@ !{indeterminate}`; GN-1.229, MAT-SN5, MAT-SN6, SP-2a, MAC-SN02, K1.230, Q1.119, Q1.120, W1-130, GN-P3-19†);
  `presentations:
  Vec<PresentationTransition>`: events `Appear`, `Disappear` (fade in on start, out on stop; SP-5a, MAC-SN05, Q1.120).
- **Variants:** none typed (the determinate spinner, macOS's, WinUI's, Material's and iOS's C-31, is the state
  `!{indeterminate}`, R3: whether the progress is known changes at run time, as on `progress_bar`). Keyed: `adwaita` `@gtk_spinner` (GtkSpinner,
  GN-1.230); `material-3` `@wavy`, `@thick`, `@four_color`, `@loading_indicator` (the Expressive loading indicator,
  MAT-N24), `@contained` (its contained form, MAT-N24); the last two are flavour `expressive`'s, declared under `[declare.flavor.expressive.widget.spinner]` only, so `flavor:baseline` answers `NotAvailable` for them (R2.7, R15).
- **Sizes and densities:** `macos-*`: `mini`, `small`, `regular` (MAC-SN03). `ios-*`: `regular` (UIKit's medium),
  `large` (SP-3a). Others: none (sizes are the body's limits and `diameter`).
- **Flags:** none of its own; state flags used: indeterminate (set by the connector while no progress is known, on
  every busy spinner: WinUI's Active state; the spinner's default motion; `!{indeterminate}` is the determinate
  spinner, a pull to refresh's pull progress included), disabled.
- **Exclusive groups:** none: ProgressRing's states are not a group (Active is `{indeterminate}`, DeterminateActive is
  `!{indeterminate}`, Inactive is the ring not drawn); others none (states combine); precedence: state models
  (S17).
- **may_be_absent:** parts: `track` (all but `adwaita`, `material-3`, `windows-11`, `macos-*`; `macos-*` by a rule
  under `{indeterminate}`, its determinate spinner alone drawing one, MAC-SN04; on `ios-*` provisional: whether iOS's
  determinate circular progress draws a track is C-31's, to verify, Open items), `spoke` (all but `macos-*`, `ios-*`;
  `macos-*` by a rule under `!{indeterminate}`, whose value arc is the `indicator`, MAC-SN04; `ios-*` by a rule under
  `!{indeterminate}` outside `context pull_to_refresh`, provisional on C-31, Open items, iOS's pull to refresh drawing
  its spokes as the pull progresses, C-10), `icon` (all
  but `kde-breeze`, `adwaita`; `ios-*` draws none, SP-4a; `kde-breeze` by a rule in `context pull_to_refresh
  {pulling}`, where the arc alone is drawn, Q2.54†; `adwaita` by a rule outside `@gtk_spinner`, AdwSpinner drawing
  the arc, GN-1.226, GN-1.230), `indicator` (`kde-breeze` by a rule outside `context pull_to_refresh`, where it
  draws neither arc nor spokes, its spinner being the icon; `adwaita` by a rule under `@gtk_spinner`, whose spinner
  is the icon, GN-1.230; `macos-*` by a rule under `{indeterminate}`, whose indeterminate spinner is spokes, MAC-SN01;
  `ios-*` by a rule under `{indeterminate}` and by a rule in `context pull_to_refresh`, its spinners being spokes,
  SP-1a, C-10, so it is drawn under `!{indeterminate}` elsewhere only, provisional on C-31, Open items); fields:
  `body.corners` (all but `material-3`; `material-3` by a rule outside `@contained`: only
  Material's contained loading indicator draws a container round the spinner, MAT-N24), `indicator.glyph`
  (`material-3` by a rule under `@loading_indicator`, whose indicator is a morphing shape, MAT-X12),
  `indicator.glyph.fill` (every preset: the arc's circle path is stroked only, its paint `glyph.stroke`'s; the
  core's mark for a stroked-only path, S6.3),
  `indicator.transform` (`macos-*`, `ios-*`, whose indeterminate spinners are spokes and whose value arc does not
  turn; every preset by a rule under `!{indeterminate}`), `icon.transform` (all but
  `kde-breeze`: GtkSpinner's icon plays its own animated SVG, GN-1.230, and no other platform draws a spinner icon,
  SP-4a), `spoke.transform` (all but `macos-*`, `ios-*`: only the spoke wheels step round, SP-2a, MAC-SN02); values:
  `stroke_width_px` (all but `adwaita`, `material-3`, `windows-11`, `macos-*`: the track's width, absent with the
  track; `macos-*` by a rule under `{indeterminate}`, as for `track`; on `ios-*` provisional on C-31, Open items),
  `stroke_weight` (all but `adwaita`), `indicator_track_gap_px` (all
  but `material-3`), `spoke_count` (all but `macos-*`, `ios-*`), `spoke_inner_radius_px` (all but `macos-*`,
  `ios-*`), `spoke_length_px` (all but `macos-*`, `ios-*`), `spoke_width_px` (all but `macos-*`, `ios-*`),
  `spoke_cap` (all but `macos-*`, `ios-*`),
  `wave_amplitude_px` (all but `material-3`; `material-3` by a rule outside `@wavy`, MAT-SN2), `wavelength_px` (all
  but `material-3`; `material-3` by a rule outside `@wavy`, MAT-SN2), `loop` (every preset, by rules under `cond
  animations_off` and under `!{indeterminate}`, whose arc follows the progress and runs no loop: Q1.120, Q3.25).
  The lists (`spoke_opacity_ramp`, `presentations`) are empty, not absent, where the platform has none. The body's
  padding, the contained indicator's inset, is a zero inset where no container is drawn, not absent.
- **State-model source:** `windows-11` `ProgressRing/ProgressRing.xaml` states; `adwaita` `adw-spinner-paintable.c`,
  `_spinner.scss`, `gtkspinner.c` (reduced motion); `kde-breeze` `kbusyindicatorwidget.cpp`, qqc2 `BusyIndicator.qml`
  (running, visible); `material-3` the circular progress-indicator tokens (determinate, indeterminate, Expressive
  variants) and the loading-indicator tokens; `macos-*` NSProgressIndicator (style spinning, indeterminate, control
  size, displayed when stopped); `ios-*` `UIActivityIndicatorView.Style`, animating, hides when stopped.
- **Today's fields:** `diameter_px` → `diameter`; `min_diameter_px` → `body.min_width`, `body.min_height`;
  `stroke_width_px` → `stroke_width` (the track's) and `indicator.glyph.stroke.width` (the arc's); `fill_color` → the
  paint of `indicator.glyph.stroke` (arc), `indicator.fill` (Material's loading indicator), `spoke.fill` (spokes) or
  `icon.glyph.fill` (icon), per platform. Today's `AnimatedIcon` goes with its types and loaders
  (`load_icon_indicator`, the icon loaders' `load_indicator`, `icon_indicator`) and the bundled pre-rotated frames of
  `spinners.rs` (S2.7, S19.3): a busy indicator is this widget, its look the `icon` part's glyph (or the arc parts) and
  its motion the `loop`, whose keyframes set the `icon` part's `transform` (a spin) or the glyph itself (a sequence of
  frames, GtkSpinner's animated icon).
- **Notes:** in context: a status page's spinner (GN-1.232) and a pull to refresh's progress and busy indicator
  (`!{indeterminate}` while pulling, Q2.54†, Q1.121, C-10) are this widget in their host's context. GtkSvg's
  process-working icon is this spinner's `loop` (GN-P3-19†); GtkSvg's state animations on other
  icons are their part's icon transitions (`PropertyGroup::Icon`, `TransitionKind::Keyed`, R25.12). Provenance: group
  7 wrote the loading indicator as a widget of its own with a container part (here `body`) and a loop of path
  keyframes (ADD-B03), replaced by the shape mechanism (R25.5); group 6 proposed iOS's determinate circular progress
  (C-31) as a progress bar variant; the stroke ratio GTK derives from the size is the stroke width's expression
  (R25.3). The eleventh check gave the arc one route, its glyph's stroke (the arc's width and cap were also the
  widget values `stroke_width` and `cap`; `stroke_width` is now the track's, `cap` the spokes' `spoke_cap`, and
  Material's `track_stroke_width` merged into `stroke_width`), and made the determinate spinner, a typed variant
  `determinate`, the state `!{indeterminate}`.

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
  - `indicator` — the value fill, and the moving indicator when indeterminate: `fill` (KDE's busy stripes, K1.110, a
    repeating gradient the `loop` moves, below), `stroke` (KDE's outline of the fill, K1.111), `corners` (PB-2,
    GN-1.109, MAC-P02, MAT-P5), `min_height`,
    `max_height`, `min_width`, `max_width` (its own thickness, W1-077, MAT-P4), `transform` (under `{indeterminate}`
    the moving indicator's position and length along the track, which the `loop`'s keyframes set: `translate_x` and
    `scale_x`, `translate_y` and `scale_y` under `vertical`, over `element.width` (`element.height` under
    `vertical`), the box of the bar's root part, which the track spans where an indicator moves, as no label lies
    beside it there; the part declares it, `data = { indicator = ["width", "height"] }`. While `{indeterminate}` the
    indicator's laid-out box is the track's full length and the transform's `origin` its start edge, so `scale_x` is
    the indicator's share of the track and `translate_x` its offset; `transform` is outside S8's layout ban; the
    identity at rest; W1-075, MAT-P6, MAC-P01, GN-1.113), `transitions` (Geometry: a value
    change, PB-3, MAC-P04, W1-078, MAT-P6).
  - `secondary_indicator` — the second moving indicator of an indeterminate bar (WinUI, Material): `fill`, `corners`,
    `transform` (as the indicator's: its position and length, which the `loop`'s keyframes set, over the root's box,
    `data = { secondary_indicator = ["width", "height"] }`; W1-075, MAT-P6).
  - `stop_indicator` — Material's end dot (MAT-P3): `fill`, `corners`, `min_width`, `min_height`.
  - `buffer` — Material's buffer (secondary progress) bar (MAT-P7): `fill`, `corners`.
  - `buffer_dots` — the dots of Material's buffer track (MAT-P7): `fill`, `corners`, `min_width`, `min_height` (each
    dot's shape and size).
  - `label` — a title label (SwiftUI's label, PB-4†): `text`.
  - `value_label` — the value text (GTK's shown text, KDE's percentage, SwiftUI's current value label): `text`
    (K1.113, PB-4†), `opacity` (GTK's dimmed text, GN-1.110).
- **Widget values:** `min_indicator_length_px: Value<Px>` (the least fill a non-zero value shows, K1.114; `auto` where
  the platform sets no minimum, never `Absent`);
  `indicator_track_gap_px: Value<Px>` (MAT-P2); `stop_indicator_gap_px: Value<Px>` (MAT-P3); `wave_amplitude_px`,
  `wavelength_px: Value<Px>` (under `@wavy`, MAT-P1); `pulse_block_fraction`, `pulse_step: Value<Fraction>` (GTK's pulse
  block and step, GN-1.113); `loop: Value<Loop>` (the indeterminate motion, a widget value's loop, on no part, so every
  keyframe names its part, `Keyframe.part` stated: W1-075's keyframes on parts `indicator` and `secondary_indicator`,
  each setting the part's `transform.translate_x` (its position, WinUI's keyframes over the start and end positions
  ProgressBar computes from the bar's width, `ProgressBar.xaml:100-111`) and `transform.scale_x` (its width, a share
  of the bar's, which ProgressBar's code computes); MAT-P6's moving indicators the same fields; GN-1.113's pulse
  block on `indicator`, its `scale_x` from `values.pulse_block_fraction` and its position `translate_x`; MAC-P01's
  moving segment or stripes as its render shows, by these fields or as KDE's; K1.110's stripes on `indicator`, each
  keyframe setting its `fill`, the repeating gradient shifted along its axis by one stripe period over the loop
  (Breeze tiles a two-stripe pattern over the rounded fill and offsets it by the busy engine's integer value, run
  from zero to the pattern's period and looped, `breezehelper.cpp:1222-1270`, `breezestyle.cpp:6258-6264`,
  `animations/breezebusyindicatorengine.cpp:94-98`, its duration the read `ProgressBarBusyStepDuration`,
  `animations/breezeanimations.cpp:81`; reversed in right-to-left, `breezehelper.cpp:1245-1247`): the shift is the
  gradient's `offset` (S2.3), from zero to one pattern period over the loop, and a `dir rtl` rule states the
  reversed loop, a paint not being mirrored (S2.3); GN-1.113, K1.110,
  MAC-P01, W1-075, MAT-P6); `label_position: Value<ValueLabelPosition>` (K1.115, PB-4, MAC-P3-4, K1.113); `label_gap_px: Value<Px>`,
  `label_alignment: Value<Alignment>` (K1.113, PB-4, MAC-P3-4); `warning_threshold: Value<Fraction>` (the value
  above which KDE's places panel recolours its capacity bar, K1.142†: the progress is element data, so the recolouring
  is an expression, not a flag the connector sets: under `@capacity`, `indicator.fill = pick(floor(min(add(element.progress,
  sub(1, values.warning_threshold)), 1)), <normal>, <warning>)`, with `data = { indicator = ["progress"] }`; the
  `min(…, 1)` keeps the sum at most 1 (a progress of 1 at a threshold of 0 would otherwise give 2, which `pick` wraps
  to the normal colour), so its floor is 1 exactly from the threshold on, 0 below it: the expression switches at the
  threshold itself, so
  the value phase checks the source's comparison, "above" or "at least", and states the threshold to match).
- **Variants:** typed `vertical` (GNOME's vertical bar, GN-1.111; KDE's QProgressBar in `Qt::Vertical`, which Breeze
  draws in its vertical branches, `breezestyle.cpp:205`, `breezehelper.cpp:1254-1266`; WinUI's ProgressBar and
  Compose's linear indicator have no orientation). Keyed: `adwaita` `@osd`; `kde-breeze` `@capacity`,
  `@capacity_inline`; `ios-*` `@bar`; `material-3` `@wavy`, `@thick`, `@four_color`.
- **Sizes and densities:** `macos-*`: the control sizes AppKit draws for a bar (`regular`, `small`, `mini`). Others:
  none.
- **Flags:** shared `empty` (value zero: GTK hides the fill node); keyed `windows-11` `@paused`, `@error`; state
  flags used: indeterminate, disabled.
- **Exclusive groups:** `windows-11`: `[@error, @paused]` (ProgressBar's error and paused states; `indeterminate`
  combines with them); others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `secondary_indicator` (all but `windows-11`, `material-3`; `windows-11`, `material-3` by
  a rule outside `{indeterminate}`, W1-075, MAT-P6), `stop_indicator` (all but `material-3`), `buffer` (all but
  `material-3`), `buffer_dots` (all but `material-3`), `label` (all but `macos-*`, `ios-*`: SwiftUI's label),
  `value_label` (all but `adwaita`, `kde-breeze`, `macos-*`, `ios-*`); fields: `indicator.stroke` (all but
  `kde-breeze`, K1.111), `track.stroke` (`material-3`: its track is a fill with no outline token, MAT-P2, MAT-P5;
  `adwaita`, `windows-11` by a rule outside `cond contrast_high`, GN-1.115, W1-079; the value phase names the others
  from the facts; AppKit's edge is MAC-P05†), `track.shadow` (the value phase names the presets from the facts);
  values: `indicator_track_gap_px` (all but `material-3`),
  `stop_indicator_gap_px` (all but `material-3`), `wave_amplitude_px` (all but `material-3`; `material-3` by a rule
  outside `@wavy`, MAT-P1), `wavelength_px` (all but `material-3`; `material-3` by a rule outside `@wavy`, MAT-P1),
  `pulse_block_fraction` (all but `adwaita`), `pulse_step` (all but `adwaita`), `label_position` (`windows-11`,
  `material-3`: their bars draw neither a label nor a value label), `label_gap_px` (`windows-11`, `material-3`: no
  label), `label_alignment` (`windows-11`, `material-3`: no label), `warning_threshold` (all but `kde-breeze`), `loop`
  (every preset, by rules outside `{indeterminate}` and under `cond animations_off`: the loop is the indeterminate
  motion only).
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
  arrows' and trough's repeat; Q1.42, MAC-P3-2, W-P4-16, MAC-X12, K-P3-17†, GN-3.69†: one home per fact, as
  `drag_hover_delay_ms`: their base on `adwaita` and `macos-*` is `= token.params.pointer_repeat.delay` and
  `= token.params.pointer_repeat.interval`, the platform's one system pointer auto-repeat, S4.6, GTK's `gtkrange.c`
  constants, GN-3.69†, and AppKit's `getPeriodicDelay(_:interval:)`, MAC-X12; the bar's own value only where its own source
  states another: Qt's scroll bar, K-P3-17†, the Quick scroll bar, Q1.42, WinUI's RepeatButtons, W-P4-16†);
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
- **may_be_absent:** parts: `arrow_button` (`adwaita`, `macos-*`, `ios-*`, `material-3`; `kde-breeze` by a rule
  where its read setting hides them, the arrow-button values at `None`, K1.94; `windows-11` by a
  rule under `cond overlay_scrollbars !{hover}`, the panning indicator being a thin thumb alone, W1-060),
  `annotation_labels` (all but `windows-11`; `windows-11` by a rule outside `@annotated`, W2-28); fields:
  `track.stroke` (`ios-*`: iOS's indicator has no track, SB-1–SB-7; `macos-*` by a rule outside `{hover}` under
  `cond overlay_scrollbars`, MAC-S02; the value phase names the others from the facts), `thumb.stroke`
  (`material-3`: its thumb is a fill, MAT-SC3†; `adwaita` by a rule outside `cond overlay_scrollbars`, GN-1.86),
  `track.fill` (`ios-*`: no track, SB-1–SB-7; `windows-11` by a rule under `cond overlay_scrollbars !{hover}`,
  W1-060; `macos-*`, `adwaita` by a rule outside `{hover}` under `cond overlay_scrollbars`, MAC-S02, GN-1.87);
  values: `hide_after_leave_ms` (all but `adwaita`: GTK's, GN-1.91), `always_visible` (all but `material-3`:
  Compose's, MAT-SC3†), `thumb_max_fraction` (all but `material-3`: Compose's, MAT-SC3†), `arrow_buttons_start` (all
  but `kde-breeze`: read on KDE, K1.94), `arrow_buttons_end` (all but `kde-breeze`: read on KDE, K1.94),
  `repeat_delay_ms` (`ios-*`, `material-3`: their scroll indicators are not interactive), `repeat_interval_ms`
  (`ios-*`, `material-3`: their scroll indicators are not interactive), `trough_click` (`ios-*`, `material-3`: their
  scroll indicators are not interactive, so a trough click does nothing), `auto_hide_delay_ms` (`adwaita`,
  `macos-*` by a rule outside `cond overlay_scrollbars`, whose classic bars stay shown, GN-1.91, MAC-X12;
  `kde-breeze` by a rule outside `cond input:touch` with `cond flavor:kirigami`, the transient touch bar alone
  hiding, Q1.41, and by a rule under `cond flavor:widgets`, QScrollBar never hiding).
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

- **Kind:** typed widget, added by group 3: the scrolling container round a viewport. KDE and macOS draw its frame
  (K2.21, Q1.43, MAC-I08), KDE, macOS and Windows its corner (K1.96, MAC-S06, W1-065); GNOME's undershoot (GN-3.06; under a toolbar view's bar,
  GN-1.156; under a column view's header, GN-1.178) and Apple 26's scroll edge effect (iOS C-18, macOS MAC-X05) are
  its `edge_effect`: two or more platforms, so typed (R14).
- **Available:** every preset.
- **Parts:**
  - `body` — the scrolling container round the viewport (the root): `padding`, `bindings`.
  - `frame` — border and background: KDE's framed background, NSScrollView's border type: `fill`, `stroke`, `corners`.
    On `kde-breeze` it is `drawn_as = { widget = "group_box", variants = ["@styled_panel"] }`, the look stated once,
    by `group_box`'s rules (K2.21): Breeze draws QAbstractScrollArea's frame and qqc2's framed ScrollView background
    as one styled panel (`breezestyle.cpp:3919-3975`; qqc2's `frame` element is a `QFrame::StyledPanel` frame,
    `kquickstyleitem.cpp:496-503`, `:1782-1785`, `ScrollView.qml:35-45`), its hover and focus outlines from the scroll
    area's own state (`breezestyle.cpp:3931-3936`; qqc2 passes `hover` and `hasFocus`, `ScrollView.qml:42-43`), so the
    substitute element carries the scroll view's hover and focus as `{hover}` and `{focused}`, which K2.21's rules
    select.
  - `corner` — the square where two bars meet: `fill`, `transitions` (WinUI's expand and contract timing, W1-065).
  - `edge_effect` *(element)* — the shade or material at one edge, one element per edge, its edge a typed identity
    (`top`, `bottom`, `start`, `end`), drawn while content lies under that edge (its own `scrolled`): GTK's
    undershoot (`undershoot.top`, `.bottom`, `.left`, `.right`, each on its own: a shade line over a short gradient,
    with its high-contrast border, GN-3.06), Apple 26's soft or hard edge under the top bar and the bottom bar each
    (a material paint with blur, its `fill`; the soft edge's fade toward the content is a `Mask` layer over that
    material, a linear gradient from opaque to transparent whose alpha masks it, S2.3, the value phase stating its
    stops from the facts): `fill`, `stroke`.
  - `overshoot` *(element)* — GTK's overshoot gradient at one edge, one element per edge with the same identities,
    drawn while the user scrolls past that edge (its own `{@overshoot}`, GN-3.06): `fill`, `transitions`. Android's
    stretch and glow are `token.behavior.scroll.overscroll_effect`.
- **Widget values:** `scrollbar_margin: Sides<Value<Px>>` (WinUI, W1-065); `wheel_step: Axes<Value<Px>>` (the
  distance one wheel step scrolls this view, per axis: on `adwaita` `= gtk_wheel_step(element.height)` and
  `= gtk_wheel_step(element.width)`, the view's page sizes, which the widget value declares, `data = { values =
  ["width", "height"] }`, GN-P3-15†, ADD-A90; on `macos-*` NSScrollView's `horizontalLineScroll` and
  `verticalLineScroll`, a distance per axis, MAC-P3-17†; elsewhere `= token.behavior.scroll_px_per_step` where the
  platform states pixels); `page_overlap: Axes<Value<Px>>` (the part of the view kept visible when it scrolls page
  by page, per axis: NSScrollView's `horizontalPageScroll` and `verticalPageScroll`, MAC-P3-17†);
  `edge_effect_height_px: Value<Px>` (C-18, MAC-X05; 26 only). Which edges draw an effect is per identity: an edge the
  platform draws none on is absent by a rule on that identity.
- **Variants:** typed `line` (macOS's line border, MAC-I08; as on text_area; input's `line`); typed on `edge_effect` and `overshoot` the
  edge identities `top`, `bottom`, `start`, `end` (`BoxEdge`; `start` and `end` mirrored in RTL), set by the connector,
  one element per edge, as the content variants are (S8 ignores them where a preset does not declare them); typed on
  `edge_effect` `soft_edge`, `hard_edge` (UIKit's and AppKit's styles; their automatic style is the application's
  choice, not a look; C-18, MAC-X05), declared by `ios-26` and `macos-tahoe` only (26 only). Keyed: `kde-breeze` `@framed` (Kirigami's framed background style hint); `macos-*` `@bezel`, `@groove`
  (with `line`, three of `NSScrollView.borderType`'s four types, MAC-I08; the fourth, no border, is the base, the
  frame absent there, as on the other presets' unframed views, so no variant names it);
  `adwaita` `@frame` (the `.frame` style class GNOME's applications put on a scrolled window: GtkFrame's border
  without its radius or label, GN-2.32).
- **Sizes and densities:** none.
- **Flags:** keyed `adwaita` `@overshoot` (on an `overshoot` element: the user scrolls past its edge, GN-3.06: an
  interaction state, not content, a size or a colour, so it is a flag; only GNOME draws a part for it, so it is keyed,
  S7.2; the other platforms' overscroll is `token.behavior.scroll.overscroll_effect`); state flags used: focus_within
  (on the scroll view: KDE's thumb follows the view's focus, K1.92), scrolled (on an `edge_effect` element: content
  lies under its edge, set per edge by the connector, GN-1.156; a bar's own `scrolled`, `toolbar`'s or `window`'s, is
  that element's).
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `frame` (`windows-11`, `ios-*`, `material-3`: no frame round the view, W1-065 naming
  the corner and margin only; `adwaita` by a rule outside `@frame`, GN-2.32; `kde-breeze` by a rule outside
  `@framed` under `cond flavor:kirigami`, Q1.43, Qt Widgets' scroll area, a framed panel by default, drawing its frame as `group_box
  @ variant @styled_panel`, K2.21 (Parts); `macos-*` in its base, the borderless view, `NSScrollView.borderType` none,
  the rules under `line`, `@bezel` and `@groove` stating it, MAC-I08), `corner` (`adwaita`, `ios-*`, `material-3`: drawn by KDE,
  macOS and Windows, K1.96, MAC-S06, W1-065), `edge_effect` (`windows-11`, `kde-breeze`, `material-3`: no edge
  effect; `macos-monterey`, `ios-13`: the scroll edge effect is 26 only, C-18, MAC-X05; `adwaita`, `ios-26`,
  `macos-tahoe` by a rule outside `{scrolled}`, GN-3.06, C-18, MAC-X05; `ios-26`, `macos-tahoe` also by a rule on
  `variant start` and `variant end`, their effect lying under the top and bottom bars only, C-18, MAC-X05),
  `overshoot` (all but `adwaita`; `adwaita` by a rule outside `{@overshoot}`, GN-3.06); fields: `edge_effect.stroke`
  (`ios-26`, `macos-tahoe`: their edge is a material, its fade a mask the core cannot yet state, C-18, MAC-X05; `adwaita` by a rule outside
  `cond contrast_high`, GN-3.06), `frame.fill` (`adwaita`: the `.frame` class draws the border alone, GN-2.32);
  values: `scrollbar_margin` (all but `windows-11`: W1-065), `edge_effect_height_px` (all but `ios-26`,
  `macos-tahoe`: 26 only, C-18, MAC-X05), `wheel_step` (`kde-breeze`, `windows-11`: their wheel steps are lines,
  `token.behavior.scroll_lines_per_step`, not a distance; `ios-*` unless the value phase finds a wheel distance in
  its facts; `adwaita`, `macos-*` and `material-3`, whose step is a distance, GN-P3-15†, MAC-P3-17†, MAT-X32†, state
  it), `page_overlap` (all but `macos-*`: MAC-P3-17†, unless the value phase finds a page overlap in another
  platform's facts).
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
  - `row` *(element)*, in `section` — one item: `padding`, `min_height` (a stated minimum the row grows past with its
    content, scaled with the text where the platform scales it, L-9), `text` (the primary text),
    `opacity` (per-item disabled fade, W1-112), `transform` (reorder scale, W1-110), `transitions`; its focus
    indicator lives on `row_background`. `row_background`, `row_inner_border`, `row_selection_indicator`,
    `leading_icon`, `secondary_text`, `trailing_text`, `overline_text`, `trailing_icon`, `unread_indicator`,
    `swipe_area`, `swipe_handle`, `nested` belong to `row`, nest in it and are queried with it; the *(element)* parts
    `cell`, `disclosure`, `accessory` and `swipe_action` nest in it as elements of their own (a rule on them may
    select the row's states by `context list.row {…}`). The rows nest in `section` on every preset (Nesting): where
    no section is drawn they are queried in the view's scope.
  - `row_background` — the shape drawn for the row's states inside the row (GNOME's whole-row fill, KDE's inset
    bubble, macOS's inset selection, WinUI's rounded backplate, Material's container and Expressive shapes): `fill`,
    `stroke` (KDE's outline, K1.171; kirigami-addons' focused border, K-KA-38†), `corners` (per corner, so merged
    selections round only the ends, MAC-L02; per state, MAT-L8), `shadow` (separate boxed-list cards, dragged
    elevation; iOS's cell background configuration, CD-1: its material fill, stroke with its `outset`, corners and
    shadow, its insets `values.row_background_inset`), `focus` (K1.175), `transitions` (the deselection fade L-11,
    GN-1.165's row transitions), `interaction` (Material's state layer and press ripple, S2.12).
  - `row_inner_border` — WinUI's inner selection border of a grid item (W2-13, W2-14): `stroke`.
  - `row_state_layer`, in `row` — Material's navigation rail's ripple pill (MAT-N13): Compose draws a rail item's
    state layer and press ripple on a box of its own, laid out as `row_selection_indicator` at its full size,
    unaffected by the indicator's width and alpha animation (`NavigationRail.kt:248-282`, `:574-590`, `:628-638`;
    `NavigationItem.kt:798-806`, `:904-912`, `:1232-1238`, the Expressive rail), where the drawer's item draws them
    on its whole `Surface`, `row_background` (`NavigationDrawer.kt:1134-1144`): `interaction` (S2.12), `corners`,
    `min_width`, `min_height`, `transitions` (the block's fades, MAT-X9).
  - `row_selection_indicator` — WinUI's selection pill (W1-107; W1-089 in the sidebar variant; the tree's, W2-12);
    Material's navigation rail and drawer active indicator (MAT-N13, rename row 18): `fill`, `corners`, `min_width`,
    `min_height`, `opacity` (the rail's, zero with the width outside `{selected}`, as Compose grows and fades it in,
    MAT-N13), `transitions` (absent where the row's state shows none, it enters from transparent and leaves to
    transparent by these, S2.7; its slide between rows is the event `@indicator_move`, `Instant` on `material-3`'s rail, whose items grow and fade
    their own, MAT-N13).
  - `row_separator` *(element)*, in `section` (beside the rows, so they are its siblings; queried in the view's scope
    where no section is drawn, as the rows) — the line between two rows, an element whose previous and next siblings
    are rows (it hides beside a hovered, selected or dragged row: GN-1.162, L-3, MAT-L6, boxed-list separators GN-1.165,
    kirigami-addons' K-KA-39†): `stroke`, `margin` (its insets from the row's ends, a line's insets being its
    `margin`: iOS's separator inset, L-3; K-KA-39†; MAT-L6), `opacity`, `transitions`.
  - `column_separator` — the vertical line between cells (GN-1.169, MAC-L06, the column browser MAC-N08): `stroke`.
  - `column_resize_handle` — the column browser's resize handle (MAC-N08): `glyph`.
  - `cell` *(element)* — one cell in a multi-column row: `padding`, `corners` (KDE rounds the selection on a row's
    first and last cells only, by position among cells, K1.170), `fill` (KDE paints the selection per cell; a shaded
    sort column, K1.183), `stroke` (GNOME's editable-cell focus and editing outlines, GN-1.172; kirigami-addons'
    current cell, K-KA-37†).
  - `header` — the header row: `fill`, `stroke` (the header's bottom line), `min_height`, `padding` (its insets per
    table style, MAC-L01).
  - `header_section` *(element)*, in `header` — a column header button: `fill`, `text`, `padding`, `transitions` (hover fade),
    `opacity`, `transform` (while dragged, MAC-L15, GN-1.171), `cursor`, `stroke` (while dragged or a drop target,
    K-KA-36†). `sort_indicator` belongs to `header_section` and is queried with it.
  - `header_separator`, in `header` — the line between header sections (K1.178, MAC-L05, GN-1.169): `stroke`, `cursor` (the
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
    `corners` (Material's chevron container), `opacity` (GNOME's expander arrow, dimmed until `open`, GN-1.257),
    `transform` (rotation), `transitions` (the rotation's and K-KA-40†'s recolouring).
  - `branch_line` — tree branch lines (K1.176, Q1.85, GN-1.279†): `stroke`.
  - `leading_icon` — the row's leading image, icon or avatar (L-8, SD-5, MAT-L2, Q1.80, Q1.87, K1.181, MAC-SB01,
    MAC-SB02): `icon` (its style and size), `corners`, `min_width`, `min_height` (an image's or video's reserved
    size, L-8, MAT-L2), `transitions` (WinUI's settings gear, an AnimatedIcon: `TransitionKind::Keyed` of group `Icon`,
    W3-28, ADD-A27).
  - `secondary_text` — subtitle, supporting or value text (L-7, MAT-L1, Q1.87, MAC-L04): `text`, `transitions` (the Expressive item's colours, MAT-L10).
  - `trailing_text` — trailing supporting text or a value cell's trailing value (MAT-L1, L-7, MAT-SB5): `text`,
    `transitions` (the Expressive item's colours, MAT-L10).
  - `overline_text` — Material's overline (MAT-L1): `text`, `transitions` (the Expressive item's colours, MAT-L10).
  - `trailing_icon` — Material's trailing icon (MAT-L3): `icon`, `transitions` (the Expressive item's colours, MAT-L10).
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
  - `drop_indicator`, in `section` — the insertion line (and its end circle) between rows (MAC-L11, K1.182,
    GN-1.279†), or the drop zone's area where the platform fills one (Material's drop-zone colour, MAT-L10): `stroke`,
    `glyph` (the
    end circle), `corners`, `fill`.
  - `section_index` *(element)* — iOS's A–Z index (L-15): `text`, `fill` (its tracking background is its pressed
    fill).
  - `nested` — the box holding an expanded row's nested rows (GNOME's `list.nested` under an AdwExpanderRow in a
    boxed list, GN-1.260): `fill`, `padding`.
- **Widget values:** `row_background_inset: Sides<Value<Px>>` (rules by position: K1.170's larger first-row top;
  K-KA-38†, K-KA-39†; a sidebar's selected row, MAC-SB04); `row_gap_px: Value<Px>` (GN-1.166, Q1.79, MAT-L6, MAT-L8);
  `intercell_gap: Axes<Value<Px>>`
  (MAC-L07); `row_separator_aligns_to_text: Value<bool>` (L-3; the inset itself is `row_separator.margin`);
  `section_separators: SectionSeparators` with `SectionSeparators { top: Value<bool>, bottom: Value<bool> }` (L-3);
  `section_inset: Sides<Value<Px>>` (L-2, MAC-L01, SD-2); `indent_per_level_px: Value<Px>` (GN-1.173, K1.176,
  MAC-L10, L-21†, Q1.85, W2-12); `indent_includes_disclosure: Value<bool>` (GN-1.279†, K-KA-40†);
  `separators_follow_indent: Value<bool>` (L-21†); `disclosure_gap_px: Value<Px>`; `leading_gap_px: Value<Px>` (MAT-L7,
  Q1.87, L-7, W1-092); `secondary_text_gap_px: Value<Px>` (L-7); `trailing_gap_px: Value<Px>`;
  `secondary_text_stacks: Value<bool>` (the accessibility-size layout, a rule `@ cond accessibility_text_size`, L-19);
  `section_headers_sticky: Value<bool>` (L-4); `filler_rows: Value<bool>`, `filler_row_height_px: Value<Px>` (L-17);
  `header_fills_trailing: Value<bool>` (Q1.86); `header_item_gap_px: Value<Px>` (K1.179);
  `header_text_alignment: Value<Alignment>` (MAC-L05); `sort_indicator_inverted: Value<bool>` (read; K1.179);
  `column_min_width_px: Value<Px>` (K-KA-36†; `auto` where the platform sets no minimum column width, never
  `Absent`); `check_box_placement: Value<Key>` (where an item's
  multi-select check box sits, keys `inline` and `overlay`, `ListViewItemCheckMode`'s two modes, W1-109; the grid
  item's corner check box, W2-13, W2-14); `accessory_placement: Vec<(VariantRef, Side)>` (each accessory identity's side,
  leading or trailing: `UICellAccessory.Placement`, L-12); `autoscroll_step_px: Value<Px>` (an expression on
  Kirigami's grid unit, `token.params.layout`), `autoscroll_interval_ms: Value<Ms>` (the list scrolling while a row
  is dragged by its handle near an edge, Q2.43); `focus_visible_when` (`FocusVisibleWhen`, S2.11; Q1.78, K1.175,
  MAC-L14);
  `swipe_full_expansion: Value<bool>` (L-13); `swipe_threshold_px: Value<Px>` (the swipe distance past which the
  actions open or the full expansion triggers, X-BT-1); `reorder_hint_offset_px: Value<Px>` (W1-110);
  `selection_indicator_edge: Value<BoxEdge>` (the row edge `row_selection_indicator` sits on: the leading edge of a
  NavigationView item, W1-089); `max_columns: Option<Value<u32>>` (the columns a table shows; `None`, `"none"`: no
  limit, every preset's base; SwiftUI's `Table` keeps only the first in a compact width, by a rule `@ variant table
  h_size_class compact`, L-22†); `control_alignment:
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
  GN-1.162; the orientation many widgets share), `destructive` on `swipe_action` (iOS, macOS), `check`, `radio` on
  `row` (rows holding a check box or radio, K-KA-38†: the meaning `menu.item`'s and `form`'s share, a checkable item),
  the content variants
  `icon_only` on `row` (a navigation rail item with no label, MAT-N13; ADD-A61), `with_subtitle` on `row` (a row
  with a secondary line: Kirigami's subtitle delegate Q1.87, iOS's subtitle cell, Material's two-line item MAT-L1;
  ADD-A61), `three_line` on `row` (Material's three-line item, MAT-L1) and `with_avatar`, `with_image`,
  `with_video` on `row` (the leading element's kind, Material's reserved sizes, MAT-L2; ADD-A61), `with_icon` on
  `row` (a row with a leading icon: Kirigami's collapsed drawer fades an icon-less item whole, its label alone
  otherwise, Q1.59; ADD-A61). Keyed: `adwaita` `@rich_list`,
  `@data_table`, `@tree_view`, `@icon_view` (the deprecated GtkTreeView and GtkIconView, GN-1.279†, GN-1.280†),
  `@stack_sidebar` (GtkStackSidebar's rows, GN-2.34), `@new_bookmark`, `@drop_placeholder` (the file chooser's places
  rows, GN-P3-22†); `kde-breeze` `@column_view` (QColumnView, K1.184†), `@ka_rounded`
  (K-KA-38†, K-KA-39†, K-KA-46†), `@ka_table` (K-KA-36†, K-KA-37†), `@ka_tree` (K-KA-40†), `@ka_indicator` (K-KA-39†),
  `@inline_footer` (Kirigami's
  inline view header placed as the view's footer, Q2.42),
  `@emphasized` (the subtitle item's emphasised form, K-KA-39†); `ios-*` `@grouped`,
  `@sidebar_plain`, `@prominent_header`, `@extra_prominent_header` (on `section_header`), `@value_cell`,
  `@accompanied_sidebar_cell`, on `accessory` the identities `@disclosure_indicator`, `@checkmark`, `@detail`,
  `@delete`, `@insert`, `@label`, `@popup_menu` (L-12, CK-1; the reorder control is the typed `reorder`), on
  `disclosure` the outline disclosure's
  styles `@header`, `@cell` (EX-1–EX-3);
  `macos-*` `@full_width` (automatic is the base), `@row_size_small`, `@row_size_medium`, `@row_size_large` (MAC-L08;
  a list
  stating its own row size style has that as a variant), `@dashed_horizontal_grid`, `@browser` (NSBrowser's column
  browser, which draws its column lines whatever the table grid setting, MAC-N08); `windows-11` `@listbox` (W2-33).
- **Sizes and densities:** `macos-*`: the row size by variant (above) and by the condition
  `cond container_size:sidebar:small` (and `medium`, `large`) for source lists at the default row size (MAC-SB01),
  the sizes declared in `container_sizes` with the platform's default first (S10), the default from the facts
  (MAC-SB01 states none);
  `windows-11`: density `compact` (the tree's row height, W2-12; the list, tree and nav item heights, W3-39);
  `ios-*`: none (Dynamic Type, L-9: the row's stated minimum height scales with the text); others none.
- **Flags:** shared `editing` (row, cell; GN-1.172, L-10, L-14), `swiped` (row; L-10, Q1.84), `context_menu_open`
  (row: the row whose popup or context menu is open, GN-1.160, MAC-L19†; kept apart from `open`, which a tree row uses
  for expanded), `sorted`, `descending` (header_section; K1.177, MAC-L18†, K-KA-36†), `current` (row, cell; K-KA-37†,
  K-KA-38†); keyed
  `kde-breeze` `@sort_column` (cell; K1.183),
  `@hidden_place` (row; KDE's hidden places, K1.142†),
  `@expandable` (row: an item with children in a Kirigami drawer, Q1.62, Q1.66), `@unread` (row; K-KA-39†);
  `windows-11`
  `@reorder_target` (W1-110), `@pre_threshold`, `@post_threshold` (swipe_action; W2-29); state flags used: hover,
  pressed, focused, focus_visible, focus_within, selected, checked, disabled, open (expanded), dragged, drop_target,
  redacted (row: a not-yet-loaded item shown as a placeholder, one flag on every platform: Windows styles the row's
  own parts by `{redacted}` rules, W-RV-2†; macOS and iOS draw the `redaction` widget in its context, MAC-P4-13†,
  P4-18†; ADD-A14).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (ListViewItemPresenter's common states; `selected`
  combines with them); others none (states combine; `material-3`'s state layers are a recency set, below);
  precedence: state models (S17). Recency sets: `material-3`: `state_layer = [dragged, focus_visible, hover]`
  (Material's recency sets, above: Compose's ripple on a clickable item, `ListItem.kt:1600-1606`; `pressed` combines;
  the row's dragged shadow, MAT-L4, is a plain `{dragged}` rule, no elevation set holding it).
- **may_be_absent:** parts: `section` (all but `ios-*`: L-2), `section_index` (all but `ios-*`: L-15),
  `section_footer` (all but `ios-*`: L-5), `accessory` (`adwaita`, `macos-*`, `windows-11`, `material-3`: no system
  accessory or reorder handle on their rows, L-12, Q2.43; `kde-breeze` by a rule `cond flavor:widgets`, the handle
  being Kirigami's, Q2.43), `row_inner_border` (all but `windows-11`: W2-13, W2-14), `row_state_layer` (all but `material-3`: MAT-N13;
  `material-3` by a rule outside `variant sidebar context sidebar.pane[compact]`), `row_selection_indicator` (all
  but `windows-11`, `material-3`: W1-107, MAT-N13; `windows-11` by a rule outside `{selected}`; `material-3` by a
  rule outside `{selected}` in the drawer, the rail's drawn at rest at zero width and alpha, and outside `sidebar`,
  MAT-N13), `nested` (all but `adwaita`: GN-1.260), `cell` (`windows-11`: no
  multi-column row among WinUI's lists, W2-12, W2-13, W2-14, W2-33), `column_separator` (`windows-11`, `material-3`:
  no line between cells; `adwaita` by a rule outside `column_separators`, GN-1.169; `macos-*` by a rule outside
  `column_separators` and `@browser`, MAC-L06, MAC-N08; `kde-breeze` by a rule outside `table`, Qt's table grid, K3.53;
  `ios-*` by a rule outside `table`, L-22†), `column_resize_handle` (all but `macos-*`; `macos-*` by a rule outside
  `@browser`, MAC-N08), `header` (`windows-11`: no column header on WinUI's lists, W2-12, W2-13, W2-14, W2-33;
  `adwaita`, `kde-breeze`, `macos-*`, `ios-*`, `material-3` by a rule outside the variants that draw one, `table` and
  their keyed table and tree variants; `ios-*` also by a rule `variant table h_size_class compact`, L-22†),
  `header_section` (`windows-11`: no column header; `adwaita`, `kde-breeze`, `macos-*`, `ios-*`, `material-3` by a
  rule outside the variants that draw a header), `sort_indicator` (`windows-11`: no column header; `adwaita`,
  `kde-breeze`, `macos-*`, `ios-*`, `material-3` by a rule outside the variants that draw a header),
  `header_separator` (`windows-11`: no column header; `adwaita` by a rule outside `column_separators`, GN-1.169;
  `ios-*`, `material-3`: the value phase names them from the facts, L-22† and MAT-L12 naming none),
  `header_corner` (all but `kde-breeze`, `macos-*`: K1.178, MAC-L18†), `footer` (all but `material-3`: MAT-L12;
  `material-3` by a rule outside `table`), `overline_text` (all but `material-3`: MAT-L1), `trailing_icon`
  (`adwaita`, `macos-*`, `windows-11`, `ios-*`: no trailing icon of the row's own, MAT-L3's being Material's and
  K1.142†'s eject action KDE's), `branch_line` (`macos-*`, `windows-11`, `ios-*`, `material-3`: no tree lines, W2-12
  naming none; `adwaita` by a rule outside `@tree_view`, GN-1.279†; `kde-breeze` by a rule
  `!cond system_setting:view_draw_tree_branch_lines`, K1.176), `section_header_line` (all but `kde-breeze`: Q2.41;
  `kde-breeze` by a rule `cond flavor:widgets`, the line being Kirigami's list section header's), `inline_header`
  (all but `kde-breeze`: Q2.42), `unread_indicator` (all but `kde-breeze`: K-KA-39†; `kde-breeze` by a rule outside
  `{@unread}`), `swipe_handle` (all but `kde-breeze`: Q1.88), `disclosure` (`macos-*` by a rule `variant sidebar`
  outside `context list.section_header {hover}`, a source list's group row showing its chevron on hover, MAC-SB03),
  `swipe_area` (`adwaita`: no swipe actions; `kde-breeze`, `ios-*` by a rule outside `{swiped}`, Q1.89, L-13;
  `macos-*`, `windows-11`, `material-3`: the value phase names them from the facts, MAC-L12, W2-29 and MAT-L11
  naming the actions alone), `swipe_action` (`adwaita`: no swipe actions; `ios-*`, `macos-*`, `windows-11`,
  `material-3` by a rule outside `{swiped}`, L-13, MAC-L12, W2-29, MAT-L11), `drop_indicator` (`windows-11`: its
  reorder parts the rows and draws no line, W1-110; `ios-*`: the value phase names it from the facts; `adwaita`,
  `kde-breeze`, `macos-*`, `material-3` by a rule `!{drop_target}`, the list not being a drop target, GN-1.279†,
  K1.182, MAC-L11, MAT-L10), `row_background` (`kde-breeze` by a rule `{swiped} cond flavor:kirigami`, a swiped
  Kirigami row drawing no shape, Q1.84), `row_separator` (`windows-11`: no line between WinUI's list items, none of
  W1-106–W1-113, W2-12, W2-13, W2-14, W2-33 naming one; `kde-breeze` by a rule outside `@ka_indicator` and `table`,
  K-KA-39†, Qt's table grid, K3.53; `ios-*` by a rule under `sidebar`, L-3; `adwaita` by a rule outside
  `row_separators` and `inset`, GN-1.162, GN-1.165; `macos-*` by a rule outside `row_separators` and
  `@dashed_horizontal_grid`, MAC-L06; `material-3` by a rule outside `row_separators`, MAT-L6); fields: `view.shadow`
  (all but `adwaita`: the boxed list's, GN-1.165; `adwaita` by a rule outside `inset`), `section.stroke` (`ios-*`:
  L-2's platters have no outline), `section.shadow` (`ios-*`: L-2), `cell.stroke` (`macos-*`, `ios-*`, `material-3`:
  no cell outline; `adwaita` by a rule outside `{focus_visible}` and `{editing}`, GN-1.172; `kde-breeze` by a rule
  outside `{current}` under `@ka_table`, K-KA-37†), `header_section.stroke` (all but `kde-breeze`; `kde-breeze` by a
  rule outside `{drop_target}` under `@ka_table`, K-KA-36†), `header_section.cursor` (all but `kde-breeze`;
  `kde-breeze` by a rule outside `{dragged}` under `@ka_table`, K-KA-36†), `header_separator.cursor` (all but
  `macos-*`: MAC-X20), `row_background.stroke` (every preset: a row at rest paints nothing on most platforms, and the
  states' rules state it), `row_background.shadow` (every preset: the same), `row_background.fill` (every preset: the
  same), `row_background.focus` (`kde-breeze` by a rule `!cond system_setting:view_draw_focus_indicator`, K1.175;
  `macos-*` where the ring is the table's own, on `view`, MAC-L14; the value phase names the others from the facts,
  where the indicator is on `view` or on the row's control);
  values: `filler_row_height_px` (all but `ios-*`: L-17), `check_box_placement` (all but `windows-11`: W1-109,
  W2-13, W2-14), `reorder_hint_offset_px` (all but `windows-11`: W1-110), `column_min_width_px` (`windows-11`: no
  multi-column row among WinUI's lists, W2-12, W2-13, W2-14, W2-33), `typeahead_hide_ms` (all but `adwaita`: GN-1.279†), `swipe_threshold_px` (`adwaita`: no
  swipe actions), `control_alignment` (all but `ios-*`: L-23†), `selection_indicator_edge` (all but `windows-11`:
  W1-089; Material's active indicator is shaped round the item's icon or the whole item, MAT-N13, MAT-SB1, on no
  edge), `autoscroll_step_px` (`adwaita`, `macos-*`, `windows-11`, `ios-*`, `material-3`; `kde-breeze` by a rule
  `cond flavor:widgets`: Kirigami's drag handle, Q2.43), `autoscroll_interval_ms` (`adwaita`, `macos-*`,
  `windows-11`, `ios-*`, `material-3`; `kde-breeze` by a rule `cond flavor:widgets`: Q2.43), `drag_hover_delay_ms`
  (where a drag hovering a collapsed row activates nothing: the value phase names the presets from the facts;
  `adwaita`, `kde-breeze`, `windows-11` state it, GN-1.279†, K-P4-11†, W-P4-11†). The lists are empty, not absent,
  where the platform has none (`accessory_placement`, `presentations`).
- **State-model source:** `adwaita`: libadwaita selectors in `_lists.scss`, `_column-view.scss`, `_views.scss`,
  `_sidebars.scss`, `_deprecated.scss` (tree and icon views); `kde-breeze`: Breeze's item-view panel, header and
  branch branches (`st:4746-4895`, `st:6803-6895`, `st:5224-5294`) and qqc2's `DefaultListItemBackground.qml` and
  `ItemDelegate.qml` states, kirigami-addons' delegates and table view; `macos-*`: NSTableView's style, row size
  style, the cell view's background style (normal, emphasized), key and main window, first responder; `ios-*`:
  `UICellConfigurationState` (highlighted, selected, focused, disabled, editing, swiped, expanded, drag and drop
  states) per `UICollectionLayoutListConfiguration.Appearance`; `material-3`: the list, data-table, list-expand,
  list-reorder and list-reveal tokens (rest, hover, focus, pressed, dragged, disabled, each selected); `windows-11`:
  ListViewItem's and GridViewItem's visual state groups (CommonStates, DisabledStates, MultiSelectStates,
  DataVirtualizationStates, ReorderHintStates, DragStates), ItemContainer's, ListBoxItem's, SwipeControl's, and
  NavigationViewItem's for the `sidebar` variant.
- **Today's fields:** `background_color` → `view.fill`; `item_font` → `row.text`; `alternate_row_background` →
  `row_background.fill @ pos parity <odd or even> variant alternating_rows`, the parity a fact of each platform
  (ADD-B06; KDE's is `even` counted from 1, S7's `Parity`: Qt's views and qqc2 tint the rows whose zero-based index is odd,
  `qtreeview.cpp`'s `current & 1`, `DefaultListItemBackground.qml:21-28`'s `index % 2`; `ios-*` declares no
  alternation, L-18);
  `selection_background` → `row_background.fill @ {selected}`; `selection_text_color` → `row.text.fill @ {selected}`;
  `header_font` → `header_section.text`; `header_background` → `header.fill` (and `header_section.fill`); `grid_color`
  → `row_separator.stroke` and `column_separator.stroke`; `row_height_px` → `row.min_height`; `header_height_px` →
  `header.min_height`; `hover_background` → `row_background.fill @ {hover}`; `hover_text_color` →
  `row.text.fill @ {hover}`; `disabled_text_color` → `row.text.fill @ {disabled}`; `border` → `view.stroke`,
  `view.corners`, `view.shadow` (from `shadow_enabled`), `row.padding` (its padding, which presets used for the row).
- **Notes:** in context: the marquee selection rectangle is `rubber_band @ context list` (GN-1.175, GN-1.280†); the
  dragged row's image and count are `drag_image @ context list` (K-RL06†, W1-110); a list inside a popover menu is
  `list @ context menu.body`, its rows `list.row` there (GN-1.79; a menu's own rows are `menu.item`); the multi-select check is a `checkbox` in `context list.row` (iOS's
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
  - `body` — the root: the tab widget's box, which draws nothing, holding the strip and its page beside each other
    (`bar` and `pane`), so the strip's bindings do not reach the page: `bindings` (empty on every preset).
  - `bar` — the strip: `fill` (paint with material: Mica Alt, W1-088), `stroke` (its base line or rule on
    the page side: GN-1.116, K1.127, MAT-TB5, GN-1.125's shade line), `padding` (GN-1.121, W1-085), `min_height`
    (GN-1.125), `min_width` (Q1.52), `corners`, `shadow`, `bindings` (the surface set the strip draws on: AdwTabBar's
    Header set, GN-1.125; Kirigami's Window set, Q1.52), `transitions` (AdwTabBar's backdrop change, GN-1.125).
  - `tab_box`, in `bar` — AdwTabBar's box holding the tabs inside the bar (GN-1.125): `padding`.
  - `start_actions`, `end_actions`, in `bar` — AdwTabBar's start and end action boxes (GN-1.125): `padding`.
  - `tab` *(element)*, in `tab_box` (where no tab box is drawn, queried in `bar`'s scope, Nesting) — one tab: `fill`,
    `stroke` (W1-086's gradient border on the sides and top; GN-1.135's
    high-contrast inset border), `corners` (K1.121 on the bar side only), `padding`, `min_width`, `max_width`,
    `min_height`, `max_height` (W1-085, GN-1.129), `opacity`, `shadow` (dragged, GN-1.133), `focus` (AdwTabBar's focus
    ring on the tab, GN-1.134), `transitions` (hover fade K1.132; GN-1.131's background transition; X-MO-6), `interaction` (Material's state layer and press ripple, S2.12). `label`,
    `icon`, `indicator` and `attention_indicator`
    belong to it and are queried with it.
  - `label` — the tab's label: `text` (font, elision: per platform, a fade at the edges on GNOME
    GN-1.129), `focus` (the focus indicator of kind Underline where focus is a line under the label, K1.129), `transitions` (Material's content colour fade, MAT-TB1).
  - `icon` — the tab's icon: `icon` (its style per state, W1-081, K1.130, MAT-TB4), `transitions` (Material's content colour fade, MAT-TB1).
  - `scroll_arrow` *(element)*, in `bar` — overflow arrows (GN-1.119, K1.125, W1-083): `fill`, `glyph`, `padding`,
    `margin` (GNOME's negative margins, GN-1.119), `min_width` (WinUI's scroll-button width, W1-083).
  - `separator` *(element)*, in `tab_box` — the line between two tabs, an element whose previous and next siblings are
    tabs
    (GN-1.126, K1.123, W1-084, MAC-TB06†): `stroke`, `margin` (its insets from its ends, a line's insets being its
    `margin` on every preset: GN-1.126's margins, K1.123's inset, W1-084's `TabViewItemSeparatorMargin`), `opacity`,
    `transitions` (its fade, GN-1.126); hidden beside a selected or hovered tab by position rules on the previous and
    next sibling (ADD-B07).
  - `indicator` — the tab's indicator, drawn under the selected tab (today's active indicator; GNOME's notebook bar,
    also on hover GN-1.117; KDE's strip K1.120; Material's MAT-TB1, MAT-TB2), a part of the `tab` element, absent by
    rule on the others: `fill`, `corners`, `min_width`, `min_height` (its thickness and length; today's
    `active_indicator_width_px`).
  - `attention_indicator` — GNOME's attention line under a tab that needs attention (GN-1.128; a line, not the
    needs-attention dot, which is a `badge`), a part of the `tab` element: `fill`, `min_height` (its thickness),
    `min_width`,
    `max_width` (the bounds of its width), `transform` (its offset below the tab).
  - `drop_indicator`, in `tab_box` — the bar a drop would land at (GN-1.127): `fill`, `min_width` (its thickness),
    `margin` (its margins, outside the box its `fill` paints).
  - `overflow_edge`, in `bar` — KDE's tear indicator where tabs are cut off (K1.126): `stroke`.
  - `pane` — the page the bar belongs to, beside `bar` in `body` (K1.128, MAC-TB01, GN-1.122, W1-088): `fill`,
    `stroke`, `corners` (cut where the bar touches, by a rule under the bar's position), `padding`.
- **Widget values:** `tab_gap_px: Value<Px>` (negative where tabs overlap: Q1.50, K-P3-4†);
  `alignment: Value<Alignment>` (`Start`, `Center`; K1.131, read); `bar_pane_overlap_px: Value<Px>` (MAC-TB02);
  `base_width_px`,
  `pinned_width_px: Value<Px>` (AdwTabBar's widths, GN-1.129); `expanding: Value<bool>` (K1.123: the tabs grow to
  fill the bar); `equal_widths: Value<bool>` (every tab the same width, sharing the strip up to `tab.max_width`, or each
  sized by its content, `false`: TabView's `Equal` and `SizeToContent` width modes, set by rules under `@width_equal` and
  `@width_size_to_content`, W1-085);
  `indicator_edge: Value<BoxEdge>` (rules by the bar's position variant; today's
  `active_indicator_side`); `indicator_length: Value<IndicatorLength>` with `IndicatorLength { Label, Tab }`
  (MAT-TB2); `attention_width_fraction: Value<Fraction>` (GN-1.128); `outward_corner_radius_px: Value<Px>` (today's
  `active_outward_corner_radius_px`: the outward corners the selected tab's shape flares into, a value of the
  strip); `close_button_shown: Value<CloseButtonShown>` with
  `CloseButtonShown { Always, OnHover, SelectedOrHover }` (W1-082), the padding change as rules on the tab's padding
  `@ variant closable`; `overflow: Value<TabOverflow>` with `TabOverflow { Scroll, ScrollButtons, Shrink,
  OverflowMenu }` (K-P3-4†, MAT-P4-6†, GN-1.119; `Shrink`, the tabs narrowing to fit with their titles truncated,
  and `OverflowMenu`, a control listing the tabs that do not fit, are the candidates MAC-P4-5† names for macOS's
  window tabs, `tab @ variant document context window.title_bar`, among which, and `Scroll`, its probe chooses); `scroll_to_selected: Value<ScrollToSelected>` with
  `ScrollToSelected { Center, VisibleWithMargin }` (MAT-P4-6†, K-P3-4†); `scroll_margin_px: Value<Px>`;
  `drag_hover_delay_ms: Value<Ms>` (a drag hovering a tab switches to it; GN-1.132, K1.133†); `focus_visible_when`
  (`FocusVisibleWhen`, S2.11; Q1.51); `icon_text_gap_px: Value<Px>` (K1.130, MAT-TB4);
  `repeat_delay_ms`, `repeat_interval_ms: Value<Ms>` (the scroll arrows'
  auto-repeat while held, K-RL07†; one home per fact, as `drag_hover_delay_ms`: the base `= token.params.pointer_repeat.delay`
  and `= token.params.pointer_repeat.interval`, the platform's one system pointer auto-repeat, S4.6, on a preset
  whose arrows repeat with it, and the strip's own value where its own source states another: Qt's tab bar, its
  buttons' defaults, K-RL07†; GtkNotebook's arrows, its own timers, `gtknotebook.c:196-197`, `:4190-4207`, on
  `adwaita`); `presentations: Vec<PresentationTransition>`, queried with a tab for its own
  events and with the widget's element (the strip and its page) for the strip's: a tab's `Appear`,
  `Disappear` (a tab opening and closing: GN-1.131; macOS's window tabs, MAC-X32†), `Move` (a tab moved to its new
  place: a reordered tab, GN-1.131, and Qt's slide, shorter for a short move, its duration an expression over
  `element.move_distance` and the tab's own `element.width` (`element.height` on vertical tabs), which the values
  declare, `data = { values = ["move_distance", "width", "height"] }`, K1.133†, ADD-A87; tabs
  parting to open a gap for a dragged one, MAC-SF07†; the target strip's tabs while a torn-out window hovers it,
  W-SF09†, if the measurement finds a gap), `@attention` (GN-1.128), `@drag`, `@tear_out` (the source tab tearing
  out, W1-087), `@spring_load` (a tab a drag hovers switching to it, MAC-X21); the strip's `@scroll`, `@resize`
  (GN-1.131) and `@indicator_move` (the selection moving to another tab, one event: a step on the tab's part
  `indicator`, a part of the `tab` element, S2.7, the indicator sliding to the new tab, MAT-TB6, and a step on part
  `pane`, the page switching, MAC-X11).
- **Variants:** typed on the widget `top`, `bottom`, `left`, `right` (bar position: GNOME's notebook, KDE, macOS;
  absolute
  positions, as GtkNotebook, QTabWidget and NSTabView name them, so a platform that mirrors them in RTL states it by
  `dir rtl` rules); typed on `tab` and the widget `document` (browser-style strip: AdwTabBar, KDE's document mode, WinUI
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
  focus_within (AdwTabBar's focus on the tab box child, GN-1.134), disabled, dragged, drop_target (on the widget's
  element, a drag over the strip).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (TabViewItem CommonStates; `selected` combines with
  them); others none (states combine; `material-3`'s state layers are a recency set, below); precedence: state models
  (S17). Recency sets: `material-3`: `state_layer = [focus_visible, hover]` (Material's recency sets, above: Compose's
  ripple state layer, `Tab.kt:166-179`; `pressed` combines).
- **may_be_absent:** parts: `attention_indicator` (all but `adwaita`: GN-1.128; `adwaita` by a rule outside
  `{@needs_attention}`), `drop_indicator` (all but `adwaita`: GN-1.127; `adwaita` by a rule `!{drop_target}`),
  `tab_box` (all but `adwaita`: AdwTabBar's, GN-1.125; `adwaita` by a rule outside `document`, GtkNotebook having
  none), `start_actions` (all but `adwaita`: GN-1.125; `adwaita` by a rule outside `document`), `end_actions` (all but
  `adwaita`: GN-1.125; `adwaita` by a rule outside `document`), `overflow_edge` (all but `kde-breeze`: Qt's tear
  indicator, K1.126; `kde-breeze` by a rule `cond flavor:kirigami`, qqc2's tab bar scrolling without it),
  `indicator` (`macos-*`, `windows-11`: no indicator under the selected tab; `kde-breeze`, `material-3` by a rule
  outside `{selected}`, K1.120, MAT-TB1; `adwaita` by a rule while neither `{selected}` nor `{hover}`, GN-1.117, and
  under `document`, AdwTabBar's strip drawing none), `separator` (`material-3`: no line between its tabs;
  `adwaita` by a rule outside `document`, GtkNotebook drawing none, GN-1.126; `kde-breeze` by a rule outside
  `@static` and beside a selected tab, K1.123), `scroll_arrow` (`material-3`: its scrollable tabs have no arrows,
  MAT-TB7; `kde-breeze` by a rule `cond flavor:kirigami`, qqc2's tab bar; `macos-*`: the value phase names it from
  the facts, MAC-TB07† leaving the overflow control to verify), `pane` (`material-3`: its tabs border no page;
  `adwaita`, `kde-breeze`, `macos-*` by a rule under `document`, a document strip's page drawing no pane,
  GN-1.122, K1.128, MAC-TB01), `label` (`windows-11` by a rule `!{selected} variant @width_compact`, W1-085); fields:
  `bar.shadow` (every preset: no tab strip casts a shadow in the rows K1.127, GN-1.116, GN-1.125, MAT-TB5, W1-088,
  MAC-TB01), `bar.corners` (the value phase names the presets from the facts), `tab.shadow` (`kde-breeze`,
  `material-3`: their tabs cast none; `adwaita`, `macos-*` by a rule outside `{dragged}`, GN-1.133, MAC-W09),
  `label.focus` (all but `kde-breeze`; `kde-breeze` by a rule outside `{selected,focus_visible}`, K1.129),
  `separator.stroke` (`macos-*` beside a selected tab, MAC-TB06†), `scroll_arrow.margin` (all but `adwaita`: GNOME's
  negative margins, GN-1.119); values: `repeat_delay_ms` (`material-3`: no scroll arrows; where the arrows do not
  repeat: the value phase names the presets from the facts, K-RL07† stating KDE's), `repeat_interval_ms`
  (`material-3`: no scroll arrows; where the arrows do not repeat: the value phase names the presets from the facts,
  K-RL07† stating KDE's), `base_width_px` (all but `adwaita`: AdwTabBar's, GN-1.129), `pinned_width_px` (all but
  `adwaita`: GN-1.129), `attention_width_fraction` (all but `adwaita`: GN-1.128), `bar_pane_overlap_px` (all but
  `macos-*`, `kde-breeze`: MAC-TB02, K1.123), `outward_corner_radius_px` (all but `windows-11`: Fluent's selected
  tab flares into the strip, platform-facts' `active_outward_corner_radius`; GNOME's and KDE's tabs have no outward
  corners, by their maps' decisions on that field), `scroll_margin_px` (all but
  `kde-breeze`, `material-3`: K-P3-4†, MAT-P4-6†, unless the value phase finds a scroll margin in another platform's
  facts),
  `indicator_edge` (`macos-*`, `windows-11`: no indicator), `indicator_length` (`macos-*`, `windows-11`: no
  indicator), `close_button_shown` (`material-3`, whose tabs have no close button; `macos-*` by a rule outside
  `context window.title_bar`, NSTabView's tabs having none and the window tabs showing theirs, MAC-W09),
  `drag_hover_delay_ms` (where a drag hovering a tab activates nothing: the value phase names the presets from the
  facts; `adwaita`, `kde-breeze` state it, GN-1.132, K1.133†).
- **State-model source:** `adwaita`: `_notebook.scss`, `_tab-view.scss` selectors and `adw-tab.c`; `kde-breeze`:
  Breeze's tab-shape branches (`hp:1299-1430`, `st:7016-7195`) and qqc2 `TabButton.qml`; `macos-*`: NSTabView
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
  - `split` — the root: the split view or drawer layer the pane is part of, which draws nothing; it holds `pane` and,
    beside it, `content`, `scrim`, `edge_shadow` and `handle`, and `separator`, `header` and `footer` nest in `pane`:
    `bindings` (empty on every preset: the pane binds its surface set, so the content beside it keeps its own). It
    lists no `corners`, so the floating pane's `Concentric` corners (SD-6, MAC-SB06) follow the window's (S2.4: the
    container is the nearest enclosing part or element whose entry lists `corners`, decided by the catalogue, so a
    root that lists none is skipped).
  - `pane` — the side pane; the Material rail's container (MAT-N13): `fill` (paint: material
    on macOS, iOS, WinUI's overlay acrylic), `stroke` (the inner edge only, flipping with the end variant and the
    direction, GN-1.139; the full frame under KDE's option, K1.140), `corners` (per corner: Material rounds the end
    corners only, MAT-SB7; floating glass, SD-6, MAC-SB06), `shadow`, `padding`, `margin` (the floating pane's
    distance from the window's edges, the container's (S2.4), by a rule under `floating`, which its `Concentric` corners' inset counts, S2.4;
    SD-6, MAC-SB06), `min_width`, `max_width`,
    `min_height` (Material's drawer container height, MAT-SB7; Kirigami's top and bottom drawers' minimum, Q1.60),
    `max_height` (Kirigami's top and bottom drawers' cap, Q1.60, below), `bindings` (the surface set it draws on: Kirigami's colour set by modality, Q1.53; GNOME's sidebar set),
    `opacity`, `transitions` (GN-1.136's backdrop transition).
  - `separator`, in `pane` — Kirigami's segmented inner-edge separator drawn apart beside the global toolbar (Q1.55): `stroke`,
    `margin` (its insets from its ends, a line's insets being its `margin`, Q1.55), `transitions` (its margins
    animate, Q1.55).
  - `edge_shadow` — the gradient strip at a modal drawer's edge (Q1.56): `fill` (linear gradient), `min_width`,
    `min_height`, `transitions`.
  - `scrim` — the dim over the covered content (Q1.57, MAT-SB6, SD-9†): `fill`, `opacity` (MAT-SB6's scrim
    opacity), `transitions`.
  - `handle` *(element)* — the floating drawer handle (Q1.61, Q3.31): `fill`, `corners`, `shadow`, `icon`,
    `min_width`, `min_height`, `padding`, `transitions` (its bottom offset animates, Q1.61).
  - `header`, in `pane` — the pane's header area (Q1.63, Q1.66's header toolbar, W1-095's pane toggle, back and search area):
    `text` (title level), `icon`, `padding`, `min_height`.
  - `footer`, in `pane` — the area at the pane's bottom holding NavigationView's footer items and settings item, which are
    `list` rows with variant `sidebar` in `context sidebar.footer` (W1-095): `padding`, `stroke`.
  - `content` — the content area beside the pane (W1-097): `fill`, `stroke`, `corners` (rounded where it meets the
    pane).
- **Widget values:** `width_px: Value<Px>` (the pane's natural width as a length; its limits are `pane.min_width`
  and `pane.max_width`: SD-10†, MAC-SB09†, Q1.66, MAT-SB7, W1-094, K-KA-41†; the inspector's open width, MAC-N19,
  C-41†); `preferred_width_fraction: Value<Fraction>` (the natural width as a share of the split's width, clamped by
  `pane.min_width` and `pane.max_width`: GNOME's `sidebar-width-fraction`, GN-1.147; iOS's
  `preferredPrimaryColumnWidthFraction`, SD-10†); a preset states the natural width by one of `width_px` and
  `preferred_width_fraction`, the other `Absent`; `width_fraction: Value<Fraction>` (a cap: the largest share of the
  window the pane takes, whatever its natural width, Kirigami's drawers, Q1.60; under `@edge_top` and `@edge_bottom`
  the share of the window's height, Q1.60; `pane.max_width` reads it, `= min(…, mul(element.window_width,
  values.width_fraction))`, and under `@edge_top` and `@edge_bottom` `pane.max_height` does, `= min(…,
  mul(element.window_height, values.width_fraction))`, so the share has one home; the first operand `…` is
  Kirigami's stated cap in grid units, a length, never `auto`, `OverlayDrawer.qml:220-228`). Under `@edge_top` and
  `@edge_bottom` `width_px` is the drawer's natural extent away from its edge, its height (the drawer spans the
  window's width), clamped by `pane.min_height` and `pane.max_height`: Kirigami sizes a drawer along that axis by one
  `preferredSize`, else by its content, between its minimum and maximum (`OverlayDrawer.qml:193-228`, `:460-475`),
  so `width_px` is `auto` where the drawer is content-sized, Kirigami's default on every edge;
  `collapsed_width_px: Value<Px>` (Q1.59, W1-094; the inspector's compact width, MAC-N19,
  C-41†); `breakpoints: Vec<Breakpoint>` (Shared value types; the widget's own breakpoints, R16. W-P4-19†'s
  NavigationView, measured `Allocation` (it adapts to its own width, the size its stretching parent offers),
  `NavigationView.cpp:1434-1464`: `compact` with width `Below` the expanded threshold, `@left_minimal` with width
  `Below` the compact threshold; the middle band is where only `compact` holds, the lower band holds both, and
  `@left_minimal`'s rules come after `compact`'s so they win there; above the expanded threshold neither holds (the
  expanded pane). iOS's arrangement per app size, P4-16†, measured `Window` (the app's size); macOS's collapse rule,
  MAC-SB09†, measured `Allocation` (the split view's width: the root `split`'s allocation), with width `Below` its
  threshold setting the keyed `@collapsed`; their comparisons and widths to verify, as the rows say);
  `resize_grip_width_px: Value<Px>` (Q1.60); `header_icon_gap_px: Value<Px>` (the header's title icon to its title,
  Kirigami's global drawer header, Q1.63);
  `drag_hover_delay_ms: Value<Ms>` (a drag hovering a sidebar row
  activates it, GN-1.145); `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear` (the pane
  shown and hidden, a drawer opening and closing; their steps name the parts they present, never the root: the
  `pane`'s slide, the `scrim`'s fade, MAT-SB9, Q1.57; the `handle`'s slide, Q1.61), `@collapse`, `@expand` (the rows' labels fading is a step of these,
  Q1.59), `@tab_sidebar_morph` (SD-7: the sidebar's side of the morph between iPad's tab bar and sidebar, one
  occasion with `bottom_tab_bar`'s), `@submenu_push`, `@submenu_pop` (Q1.64), `@settle` (a dragged surface released
  coming to rest, as `sheet`'s: a drawer released part-way returning, its duration over `element.progress`, which the values declare, `data = { values =
  ["progress"] }`, Q1.58).
- **Variants:** typed `overlay` (modal or overlaying: Kirigami's modal drawers, Material's modal drawer, WinUI's
  overlay pane, iOS's overlay behaviour, GNOME's collapsed overlay split view; GN-2.19, W2-30), `compact` (the
  collapsed rail: Kirigami's collapsed global drawer, WinUI's LeftCompact, Material's navigation rail MAT-N13),
  `floating` (macOS 26, iOS 26 inset glass sidebars, declared by `macos-tahoe` and `ios-26` only: 26 only, SD-6,
  MAC-SB06), `end` (trailing side: GNOME's end sidebar, Kirigami's right
  drawers, WinUI's right pane, the trailing inspector column of macOS MAC-N19 and iOS C-41†). Keyed: `adwaita`
  `@secondary` (middle pane, GN-1.137); `kde-breeze` `@global`, `@context`, `@places` (KIO), `@side_panel` (Qt's side
  panel view), `@edge_top`, `@edge_bottom` (drawers on the top or bottom edge, Q1.60); `windows-11` `@left_minimal`;
  `ios-*` `@displace`; `macos-*` `@collapsed` (the sidebar the split view collapses when it narrows past its collapse
  rule, set by the `breakpoints` value, MAC-SB09†: the pane not drawn, by a rule, its look with the rule to verify);
  `material-3` `@expanded`, `@modal` (the rail's expanded and modal forms, MAT-N13; flavour `expressive`'s,
  declared under `[declare.flavor.expressive.widget.sidebar]` only, so `flavor:baseline` answers `NotAvailable` for them (R2.7, R15)). Typed
  `bottom_sheet` (MAT-SB7's bottom-sheet drawer; the meaning `dialog`'s `bottom_sheet` shares on GNOME).
- **Sizes and densities:** `macos-*`: the sidebar size is the condition `cond container_size:sidebar:small` (and
  `medium`, `large`) (R2.3), the sizes declared in `container_sizes` with the platform's default first (S10), the
  default from the facts (MAC-SB01 states none), selecting `list` rules (MAC-SB01); others none.
- **Flags:** none of its own; state flags used: focus_within (KDE's focused-view edge colour, K1.140), drop_target
  (drag-hover activation); a pane shown or hidden is the element present or not, and its motion `Appear`,
  `Disappear`.
- **Exclusive groups:** none (the pane has no common-state group; rows follow `list`); precedence: state models (S17).
- **may_be_absent:** parts: `pane` (`macos-*` by a rule under `@collapsed`, the collapsed sidebar, MAC-SB09†, to
  verify with its rule), `separator` (all but `kde-breeze`: Kirigami's segmented separator, Q1.55; `kde-breeze`
  by a rule `cond flavor:widgets`), `handle` (all but `kde-breeze`: Kirigami's drawer handle, Q1.61; `kde-breeze` by
  a rule `cond flavor:widgets`), `edge_shadow` (all but `kde-breeze`, `adwaita`: Kirigami's modal drawer's edge
  gradient, Q1.56, AdwOverlaySplitView's transition shadow, GN-2.19; `kde-breeze`, `adwaita` by a rule outside
  `overlay`), `scrim` (`macos-*`: nothing dims beside its sidebar; `adwaita`, `kde-breeze`, `windows-11`,
  `material-3` by a rule outside `overlay`, GN-2.19, Q1.57, W2-30, MAT-SB6; `ios-*` by a rule outside `overlay` and
  `@displace`, SD-9†), `header` (`adwaita`, `macos-*`, `ios-*`: the bar topping their sidebars is a `toolbar`),
  `footer` (all but `windows-11`: W1-095), `content` (all but `windows-11`: W1-097); fields: `pane.stroke`
  (`material-3`: MAT-SB7 and MAT-N13 state no edge line; `macos-tahoe`, `ios-26` by a rule under `floating`, SD-6,
  MAC-SB06; the value phase names the presets whose modal pane, under `overlay`, draws none, from the facts),
  `pane.shadow` (`adwaita`, `kde-breeze`: a modal drawer's shadow is `edge_shadow`, GN-2.19, Q1.56; `windows-11` by
  a rule outside `overlay`, W1-096; `material-3` by a rule outside `overlay` and `compact`, MAT-SB6, MAT-N13;
  `macos-*` by a rule outside `floating`, MAC-SB06; `ios-*` by a rule outside `floating`, `overlay` and `@displace`,
  SD-6, SD-9†); values: `width_px` (`adwaita`: GNOME's natural width is the fraction's, GN-1.147; `ios-*` where the
  value phase states SD-10†'s natural width as the fraction), `preferred_width_fraction` (all but `adwaita`,
  `ios-*`: GN-1.147, SD-10†; `ios-*` where the value phase states SD-10†'s natural width as a length),
  `width_fraction` (all but `kde-breeze`: Kirigami's cap, Q1.60; `kde-breeze` by a rule `cond flavor:widgets`),
  `collapsed_width_px` (`adwaita`: no collapsed rail, GN-2.19), `resize_grip_width_px` (all but `kde-breeze`:
  Q1.60), `header_icon_gap_px` (all but `kde-breeze`: Q1.63), `drag_hover_delay_ms` (where a drag
  hovering a row activates nothing: the value phase names the presets from the facts; `adwaita` states it,
  GN-1.145). The lists are empty, not absent, where the platform has none (`breakpoints`, `presentations`).
- **State-model source:** `adwaita`: `_sidebars.scss` selectors and the split views' properties
  (`adw-navigation-split-view.c`, `adw-overlay-split-view.c`); `kde-breeze`: Kirigami's `OverlayDrawer.qml`,
  `GlobalDrawer.qml`, `ContextDrawer.qml` states (modal, collapsed, open, position) and Breeze's side-panel branch
  (`st:3953-3958`); `macos-*`: the visual effect view's state with the key state, NSSplitViewItem's behaviour and
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
  (GN-P3-22†); `breakpoints` set the display variants (`compact`, `overlay`, `@left_minimal`, `@displace`, `@collapsed`), and rules
  select them by size class too (R16, P4-16†, W-P4-19†). Windows' top NavigationView is
  `view_switcher @ variant @navigation_top` (W1-094's Top mode, W2-11). In a compact width iOS presents the
  inspector as a sheet (C-41†): on `ios-*` the pane is `drawn_as = { widget = "sheet", variants = [ … ] }` by a rule `@ variant end
  h_size_class compact` (Nested controls), the sheet's variants (the list S11 always spells) and look measured in the
  value phase; its nested parts
  are queried with the sheet as their host (Nesting). Provenance: group 6 wrote the inspector as a
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
    MAT-N25; animating between the expanded and collapsed elevations, MAT-P4-7†), `corners`, `margin` (the floating
    bar's distance from the window's edges, by a rule under `floating`: Material's `ScreenOffset`, its
    `ContainerExternalPadding`, on the edge the bar is anchored to only, the others zero, as the application offsets
    the bar on that axis alone, `androidx@e4d933c9 compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/FloatingToolbar.kt:685-689`,
    `src/androidDeviceTest/kotlin/androidx/compose/material3/FloatingToolbarTest.kt:112`, `:204`; MAT-N25), `padding` (the osd bar,
    GN-1.149; the floating bar), `min_height` (per variant: large title, compact, with search or prompt, C-1, TB-3,
    W1-103), `min_width` (the vertical floating toolbar, MAT-N25), `max_height` (Kirigami's global toolbar, Q1.74),
    `bindings` (KDE's Header set by position, Q1.68, Q1.71), `transitions` (MAT-TA1's on-scroll change; the header's
    resize, Q1.74).
  - `item_group` *(element)*, in `content` — items sharing one background (TB-5, MAC-TL03; 26's glass capsules): `fill`
    (material), `corners`, `shadow`, `padding`, `min_height` (the capsule's, MAC-TL03).
  - `separator` *(element)*, in `content` — the line between items (K1.156, Q1.70, W1-101, Q1.73's leading separator,
    MAC-TL08's tracking separator, an element because the tracking one is its own variant): `stroke` (the line, its start edge), `corners`, `margin` (the space on either side, W1-101),
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
  collapse, C-1; `false` where the bar does not collapse); `tooltip_shows_shortcut: Value<bool>` (Q1.76; Kirigami's flat tool buttons are `button` rules in
  `context toolbar.bar`);
  `presentations: Vec<PresentationTransition>` (queried with the bar): events `Appear`, `Disappear` (the bar shown and
  hidden: Material's
  floating toolbar, MAT-N25; AdwToolbarView's revealed bars, GN-P3-5†; iOS's bars hidden on a swipe, P4-3†), `Move`
  (the bars re-laid out when a toolbar moves or a dock opens a gap, K3.38†), `@expand`, `@collapse` (the bar opened
  to show its labels and overflow and closed again: WinUI's command bar, W1-104; Material's floating toolbar,
  MAT-P4-7†), `@items_change` (the items fading and moving when they are
  set, added, removed or moved to the overflow: P3-4†; while the customization editor runs, MAC-P3-6†, P4-10†),
  `@customize` (the items parting while one is dragged into the bar from the customization editor, P4-10†,
  MAC-P3-6†), `@title_change` (the navigation push and pop: the title's cross-fade, C-2, X-MO-3; the back button is
  a nested `button` element, not a part of the toolbar or of one of its *(element)* parts, so its label's
  transition is the `back` button's own `@title_change`). The on-scroll change of the bar
  (MAT-TA1) is its `transitions` under `{scrolled}`; Kirigami's global header resizing and its separator fading
  (Q1.74) are the bar's `transitions`. The search item's expansion is `search_field`'s event `@expand` (MAC-TL07).
- **Variants:** typed `top`, `bottom` (header or footer position: KDE's ToolBar, GNOME's top bar and its bottom bar,
  GtkActionBar and AdwToolbarView's bottom bar, GN-1.157, GN-2.20; iOS's toolbars against navigation bars; Material's
  top and bottom app bars; Q1.68), `floating` (Material's floating toolbar, MAT-P4-7†, MAT-N25; KDE's FloatingToolBar,
  K-KA-28†), `vertical` (Material's vertical floating toolbar, MAT-N25, R25.14). Keyed: `adwaita` `@raised`
  (GN-1.153), `@osd`, `@border`, `@collapse_spacing`, `@inline`, `@shrunk` (the bottom bar's inline and shrunk forms,
  GN-2.20); `kde-breeze` `@main` (the main toolbar against other toolbars, K1.153, K1.158), `@global`, `@page_header`;
  `macos-*` `@tracking` (on the `separator` element: the tracking separator aligned with the split view's divider,
  MAC-TL08); the window's toolbar style is the window's keyed variant, one window property with one name: the
  toolbar's look under each style is a rule `@ context window variant @toolbar_unified / @toolbar_unified_compact /
  @toolbar_expanded / @toolbar_preference` (MAC-TL01); `ios-*` `@large_title` (C-1), `@navigator`, `@browser`, `@editor` (the navigation item's styles,
  C-43†); the content variants `with_search`, `with_prompt` (the navigation bar holding a search bar or a prompt,
  set by the connector from its content, C-1; iOS's compact bar is `@ v_size_class compact`, as UIKit picks it by
  the size class, TB-3); `material-3`
  `@center_aligned`, `@medium`, `@large`, `@medium_flexible`, `@large_flexible` (MAT-TA3), `@docked`, `@vibrant`
  (MAT-N25), `@collapsed` (MAT-P4-7†); `windows-11` `@compact`, `@minimal`, `@hidden` (the closed display modes,
  W1-103). On `material-3` the Expressive toolbars' variants, typed `floating`, `vertical` and keyed `@docked`,
  `@vibrant`, `@collapsed` (MAT-N25, MAT-P4-7†), are flavour `expressive`'s, declared under `[declare.flavor.expressive.widget.toolbar]` only, so `flavor:baseline` answers `NotAvailable` for them (R2.7, R15).
- **Sizes and densities:** `ios-*`: the compact height `@ v_size_class compact` (TB-3, ADD-A20); others by variant.
- **Flags:** none of its own; state flags used: scrolled (on the toolbar: content beneath it is scrolled away from
  its edge, set by the connector; TB-1, MAT-TA1, C-1; the scroll view's own edge effects carry theirs, `scroll_view`),
  open (W1-103's open command bar), drop_target (an item dragged over the bar from the customization editor,
  MAC-P3-6†, P4-10†).
- **Exclusive groups:** none: the bar's states combine; its items are `button` elements whose groups are `button`'s
  (AppBarButton's CommonStates on `windows-11`); precedence: state models (S17).
- **may_be_absent:** parts: `item_group` (all but `ios-26`, `macos-*`: TB-5's glass capsules are 26 only, MAC-TL03
  naming 12–15's segmented groups too), `separator` (`material-3`, `ios-*`: no separator item between their bars'
  items, by their toolbar rows MAT-N25, MAT-TA3, TB-5, TB-8, C-1), `grip` (all but `kde-breeze`: Qt's toolbar grip,
  K1.156), `title` (`adwaita`, `macos-*`: the title in their bar is the window's, `window.title`; `windows-11`:
  CommandBar holds no title), `subtitle` (all but `material-3`, `ios-*`: MAT-TA4, C-1; `material-3` by a rule
  `cond flavor:baseline`, the subtitle being Expressive's, MAT-TA4), `large_title` (all but `ios-*`: C-1; `ios-*` by
  a rule outside `@large_title`), `large_subtitle` (all but `ios-*`: C-1; `ios-*` by a rule outside
  `@large_title`), `prompt` (all but `ios-*`: C-1), `title_menu_chevron` (all but `ios-*`: C-1, C-28),
  `document_header` (all but `ios-*`: C-28), `leading_content` (all but `material-3`: MAT-P4-7†; `material-3` by a
  rule outside `floating`), `trailing_content` (all but `material-3`: MAT-P4-7†; `material-3` by a rule outside
  `floating`); fields: `bar.shadow` (`macos-*`, `windows-11`, `ios-*`: their toolbar rows state no bar shadow;
  `adwaita` by a rule outside `@raised`, GN-1.153; `kde-breeze` by a rule outside `floating`, K-KA-28†; `material-3`
  by a rule outside `{scrolled}` and `floating`, MAT-TA1, MAT-N25), `bar.stroke` (`material-3`: its bars draw no
  line; `ios-26`: the hairline is 13–18's, TB-2, to verify; a line on some edges only states the others' `width`
  and `paint` `Absent`, S6.3, Entry format), `bar.fill` (`macos-tahoe`: the 26 bar is transparent, MAC-TL09;
  `ios-26`: no bar background, C-1, TB-1); values: `group_gap_px` (all but `ios-26`, `macos-tahoe`: the spacing
  between capsules is 26 only, TB-5, MAC-TL03), `fixed_space_width_px` (all but `ios-*`, `macos-*`: TB-8,
  MAC-TL10†, whose cross-checks find no fixed space item on KDE or Windows), `flexible_space_min_width_px` (all but
  `ios-*`, `macos-*`: TB-8, MAC-TL10†), `title_alignment` (`adwaita`, `macos-*`, `windows-11`: no title of the
  bar's own).
- **State-model source:** `adwaita`: `_toolbars.scss` selectors (toolbars, action bar, raised and revealed bars);
  `kde-breeze`: Breeze's tool-button and toolbar branches (`st:4572-4619`, `st:5157-5222`, `st:5521-5680`) and qqc2
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
  in context (`button.values.icon_text_gap`, K1.154, MAC-TL02; `button.values.label_position`, K1.153 read per
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
    `padding`, `min_height` (the content border's height, MAC-ST01), `text` (the bar's own message, today's `font`).
  - `item` — an item: `text` (`= widget.bar.text` where the platform draws its items in the bar's style, so the style
    has one home; the value phase states another only from the facts), `padding`.
- **Widget values:** none (KDE's and macOS's rows state no value beyond the parts).
- **Variants:** keyed `kde-breeze` `@main_window` (the top line is drawn only for a QMainWindow's status bar, K1.160).
- **Sizes and densities:** none.
- **Flags:** none; the window's inactive look is the condition `cond window_inactive` (MAC-ST01).
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: none; fields: `bar.stroke` (`kde-breeze` by a rule outside `@main_window`, the top line
  being a main window's status bar's only, K1.160), `bar.shadow` (`kde-breeze`, `macos-*`: neither bar casts a
  shadow, K1.160 and MAC-ST01 naming the top line alone); values: none.
- **State-model source:** `kde-breeze` Breeze `st:5307-5320`; `macos-*` the window's content border thickness for the
  bottom edge, key and main window.
- **Today's fields:** `font` → `bar.text` (and through it `item.text`); `background_color` → `bar.fill`; `border` → `bar.stroke`, `bar.padding`,
  `bar.corners` (from `corner_radius`), `bar.shadow` (from `shadow_enabled`).
- **Notes:** provenance: group 4 also listed GNOME's bottom
  bar here, with its spacing as an item gap (GN-1.157); it is one widget with the toolbar's bottom bar, the spacing
  `toolbar.values.item_gap @ variant bottom`.

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
- **may_be_absent:** parts: `grip` (`adwaita`, `kde-breeze`: no grip on the divider, W1-157†'s cross-checks;
  `macos-*` by a rule outside `wide`, the thick divider's grip, MAC-SP02), `collapser` (`adwaita`, `macos-*`,
  `windows-11`, `ios-*`, `material-3`: KSplitterCollapserButton is KDE's, K2.17; `kde-breeze` by a rule
  `cond flavor:kirigami`, the button being a Qt Widgets one), `collapser_arrow` (`adwaita`, `macos-*`, `windows-11`,
  `ios-*`, `material-3`: K2.17; `kde-breeze` by a rule `cond flavor:kirigami`); fields: `divider.fill`
  (`macos-tahoe`, `ios-26` by a rule under `sidebar`: no divider beside a floating sidebar, MAC-SP04, SP-1),
  `divider.stroke` (`macos-tahoe`, `ios-26` by a rule under `sidebar`: MAC-SP04, SP-1); values: `hit_extension`
  (all but `kde-breeze`: Q1.92, Q1.93, unless the value phase finds a hit area beyond the drawn divider in another
  platform's facts). The lists are empty where none is stated (`presentations`).
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
    centred where the platform centres it on a half pixel, K1.201; pixel snapping, K-P3-20†, ADD-A33), `margin` (its
    start and end insets per container and orientation, by rules with `context` and by Material's inset variants:
    GNOME's sidebar separator, GN-1.187; the navigation pane's, W1-119; MAT-D1), `min_width`, `min_height` (GNOME's
    spacer, GN-1.188), `bindings` (the surface set its stroke draws from, Q1.95).
- **Widget values:** none (its insets are `line.margin`, one home for a part's insets). A menu's separator item is
  `menu.separator` on every preset, its insets `menu.separator.margin` (GNOME's menu margins, GN-1.187; W1-119); a
  toolbar's is `toolbar.separator`, its insets `toolbar.separator.margin`: a line's insets are its part's `margin`
  on every widget, as a stroke lies on its box's edge and padding does not shorten it (S2.4).
- **Variants:** typed `vertical` (the line running down; horizontal is the base, the orientation many widgets share,
  R25.14). Keyed: `adwaita` `@spacer` (no line, GN-1.188); `kde-breeze` `@light`
  (Q1.95); `ios-*` `@opaque` (SE-2); `material-3` `@inset`, `@inset_start`, `@inset_end` (MAT-D1).
- **Sizes and densities:** none.
- **Flags:** none (no states).
- **Exclusive groups:** none (no states); precedence: state models (S17).
- **may_be_absent:** parts: none; fields: `line.stroke` (`adwaita` by a rule under `@spacer`, the spacer drawing no
  line, GN-1.188); values: none.
- **State-model source:** no states; variants from libadwaita `_misc.scss`, Kirigami's `Separator.qml`, Material's
  divider, UIKit's separator colours, WinUI's separators per container, macOS's `NSBox` of type `separator`
  (MAC-BX02, MAC-SE01).
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
    `transitions`, `interaction` (Material's state layer and press ripple, S2.12).
  - `icon` — Material's card icon (MAT-CA3): `icon`.
  - `banner` — Kirigami's banner image area (Q1.131): `padding`, `min_width`, `min_height`.
  - `banner_band` — the band behind the banner title, its height relative to the title's (Q1.131): `fill`.
  - `banner_title` — the banner's title (Q1.131): `text`, `icon`.
- **Widget values:** `section_gap_px: Value<Px>` (between header, content and footer, Q1.130);
  `banner_band_height_factor: Value<f32>` (Q1.131).
- **Variants:** typed `activatable` (GNOME's activatable card, Kirigami's clickable cards, Material's interactive
  cards). Keyed: `adwaita` `@osd`; `kde-breeze` `@banner`; `material-3` `@elevated`, `@outlined` (two of MAT-CA1's
  three container styles; the third, the filled card, is the base, so no variant names it); `windows-11` `@secondary`, `@tertiary` (the card fills, W1-139; the default card is
  outlined-like).
- **Sizes and densities:** none.
- **Flags:** none of its own; state flags used: hover, pressed, checked (Q1.126), focus_visible, dragged, disabled
  (MAT-CA2).
- **Exclusive groups:** none (states combine; Windows' card surfaces have no states; `material-3`'s state layers and
  elevation are recency sets, below); precedence: state models (S17). Recency sets: `material-3`:
  `state_layer = [dragged, focus_visible, hover]` (MAT-CA2's state layers; Material's recency sets, above: a clickable
  card's `Surface` ripple, `Card.kt:149-150`; `pressed` combines) and `elevation = [dragged, pressed, focus_visible,
  hover]` (MAT-CA2's elevations, `Card.kt:668-723`).
- **may_be_absent:** parts: `icon` (all but `material-3`: Material's card icon, MAT-CA3), `banner` (all but
  `kde-breeze`: Kirigami's banner, Q1.131; `kde-breeze` by a rule outside `@banner`), `banner_band` (all but
  `kde-breeze`: Q1.131; `kde-breeze` by a rule outside `@banner`), `banner_title` (all but `kde-breeze`: Q1.131;
  `kde-breeze` by a rule outside `@banner`); fields: `body.shadow` (`adwaita` by a rule under `@osd`, GN-1.256;
  `material-3` by a rule under `@outlined`, at rest, its hovered and dragged shadows the tokens', MAT-CA2); values:
  `section_gap_px` (all but `kde-breeze`: Q1.130), `banner_band_height_factor` (all but `kde-breeze`: Q1.131).
- **State-model source:** `adwaita` `_misc.scss` card selectors; `kde-breeze` Kirigami's `DefaultCardBackground.qml`;
  `material-3` card tokens (filled, elevated,
  outlined × states); `windows-11` the card fills (no states).
- **Today's fields:** `background_color` → `body.fill`; `border` → `body.stroke`, `body.corners`, `body.padding`,
  `body.shadow` (from `shadow_enabled`).
- **Notes:** the footer is `toolbar @ variant bottom context card.body` (Q1.132). Provenance: group 4 typed Material's
  container styles (the eleventh check named the filled card the base and dropped its keyed `@filled`) and listed Windows' clickable SettingsCard as `activatable`; the card's Windows states moved with
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
- **Available:** every preset but `material-3`, `ios-*` (`available = false`: their expandable item is a list row;
  on `ios-*` SwiftUI's DisclosureGroup is also drawn standalone, outside a List, where it would be this widget with
  `header` and `arrow`, but no iOS row states that look, EX-1–EX-4 being the list's outline disclosure, so `ios-*`
  declares it only once the value phase has measured it, Open items; inside a List it stays the list-row route);
  `kde-breeze` flavour `widgets` only (KCollapsibleGroupBox).
- **Parts:**
  - `body` — the expander (the root), holding the header and the content: `padding`, `bindings`.
  - `header` *(element)* — the header: `fill`, `stroke`, `corners` (GN-1.261), `padding`, `min_height`, `text`,
    `focus`, `shadow`.
  - `arrow` *(element)*, in `header` — the disclosure arrow, an element of its own (macOS's disclosure button has its
    own states), nested in the header so that a rule selects the header's hover and press for it, as GNOME, KDE and
    WinUI draw it (`expander.arrow.opacity @ context expander.header {hover}`, GN-1.261, K1.250, W1-140): `glyph` (source per platform; on macOS the system control rendering of
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
- **may_be_absent:** parts: none; fields: `header.stroke` (`adwaita`, `kde-breeze`, `macos-*`: their expanders draw
  no frame, today's frame off; W1-140 frames Windows'), `content.stroke` (`adwaita`, `kde-breeze`, `macos-*`: the
  same; W1-141), `header.shadow` (every preset: no expander header casts a shadow in the rows GN-1.261, K1.250,
  MAC-EX01, W1-140), `content.fill` (`adwaita`, `kde-breeze`, `macos-*`: the content lies on its container; W1-141
  fills Windows'), `arrow.fill` (`adwaita`, `kde-breeze`, `macos-*`: W1-140's chevron button is Windows'),
  `arrow.stroke` (`adwaita`, `kde-breeze`, `macos-*`: W1-140); values: `drag_hover_delay_ms` (where a drag hovering
  the header expands nothing: the value phase names the presets from the facts; `adwaita` states it, GN-1.262).
- **State-model source:** `adwaita` `_expanders.scss`; `kde-breeze` Breeze's arrow
  branch (`st:4419-4422`) and `kcollapsiblegroupbox.cpp`; `macos-*` NSButton's disclosure states and control sizes;
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
  - `scrim` — the optional scrim behind a levitated pane (Compose's `LevitatedPaneScrim`, MAT-SF04†):
    `fill` (`= role.global.scrim`), `transitions`.
- **Widget values:** `widget_gap_px`, `container_margin_px`, `window_margin_px`, `section_gap_px: Value<Px>` (today's);
  `label_gap_px: Value<Px>` (label to control, per size: MAC-LY01, W1-146); `button_gap_px: Value<Px>` (MAC-LY01,
  W1-146);
  `header_gap_px: Value<Px>` (control to header, W1-146); `surface_text_margin_px: Value<Px>` (W1-146); `item_gap_px`,
  `line_gap_px: Value<Px>` (flow layouts: GN-2.46, GN-P3-24†, RL-3†, MAC-RL01†, W-P4-9†, MAT-RL01†; a linked box's
  overlap is not its gap but each child's own margin, `button.body.margin`, `input.body.margin` and
  `combo_box.body.margin` in `context any layout variant @linked` (the widgets `_linked.scss:1-12` lists),
  GN-1.42); `flow_max_per_line: Option<Value<u32>>` (GN-P3-24†; `None`: no limit); `max_content_width_px`,
  `tightening_threshold_px: Value<Px>` (the `clamp` layout's: AdwClamp, GN-1.195; iOS's readable width, LY-2);
  `pane_gap_px: Value<Px>` (MAT-LA1, MAT-SP3);
  `vertical_pane_gap_px: Value<Px>` (the spacer between two stacked panes, MAT-P3-13†); `max_panes_horizontal`,
  `max_panes_vertical: Value<u32>`, `pane_width_px`, `pane_height_px: Value<Px>`, `avoids_hinge: Value<bool>`
  (MAT-P3-13†; two stacked panes `@ cond posture:tabletop`, ADD-A83); `breakpoints: Vec<Breakpoint>` (Shared value
  types; the widget's own breakpoints, R16: TwoPaneView's wide and tall thresholds, measured `Allocation` (the control
  adapts to its own width and height, which for the stretched control are the size its parent offers), width `Above`
  setting the keyed `@two_pane_wide`, height `Above` setting `@two_pane_tall` (`TwoPaneView.cpp:103-150`, the wide test
  first, so `@two_pane_wide`'s rules come after `@two_pane_tall`'s and win where both hold), W-P4-19†); `levitated_placement: Value<Placement>` (a levitated pane
  floating over the scaffold, strategy `MaterialLevitatedPane`: its alignment the placement's `side` and `align`;
  its scrim the part `scrim`, MAT-SF04†);
  `column_width_px`, `column_min_width_px`, `column_max_width_px: Value<Px>` (Kirigami's ColumnView, Q1.93, Q1.94;
  CardsLayout, Q1.133; kirigami-addons' settings window, K-KA-41†); `max_columns: Option<Value<u32>>` (Q1.133; `None`: no limit);
  `presentations: Vec<PresentationTransition>`: events `@column_scroll` (Q1.94). Lengths that scale with text are
  expressions (ADD-A55).
- **Variants:** typed `flow` (GNOME's wrap box GN-2.46, iOS RL-3†, macOS MAC-RL01†, Windows W-P4-9†),
  `clamp` (content clamped to a readable width: AdwClamp, GN-1.195; iOS's readable content guide, LY-2), `pane` (an
  adaptive layout's pane: Material's levitated pane, MAT-SF04†; WinUI's TwoPaneView, W-P4-19†), `vertical` (a
  vertical box: GTK's vertical linked box, GN-1.42; the orientation many widgets share). Keyed: `adwaita`
  `@preferences_page`, `@linked` (GTK's linked box, GN-1.42); `kde-breeze` `@cards`, `@cards_list`, `@column_view`,
  `@qqc2_pane` (qqc2's Pane, a padded container control, Q1.98; named apart from the typed `pane`, the adaptive
  layout's), `@page` (qqc2's Page, Q1.98), `@kirigami_page` (with
  `@page`: Kirigami's Page, whose padding differs from qqc2's, Q1.98); `windows-11` `@two_pane_wide`, `@two_pane_tall` (W-P4-19†). Material's chip set is the typed `flow`
  (MAT-RL01†): it adds nothing a flow layout lacks.
- **Sizes and densities:** `macos-*` control sizes on `label_gap` and `button_gap` (MAC-LY01), selected by the
  layout element's own control size (`size s`, S7.3), which the connector sets to the size of the controls the layout
  holds; size classes:
  Material's window classes (MAT-LA1, MAT-P3-13†), iOS's compact and regular (LY-1).
- **Flags:** none (no states).
- **Exclusive groups:** none (no states); precedence: state models (S17).
- **may_be_absent:** parts: `scrim` (all but `material-3`: Compose's `LevitatedPaneScrim`, MAT-SF04†; `material-3`
  by a rule outside `pane`); fields: none; values: `label_gap_px` (all but `macos-*`, `windows-11`: MAC-LY01,
  W1-146, unless the value phase finds a label gap in another platform's facts), `button_gap_px` (all but `macos-*`,
  `windows-11`: MAC-LY01, W1-146, unless the value phase finds a button gap in another platform's facts),
  `header_gap_px` (all but `windows-11`: Fluent's control-to-header gap, W1-146), `surface_text_margin_px` (all but
  `windows-11`: W1-146), `item_gap_px` (`kde-breeze`: no flow container in KF6 or Kirigami, the wrap box's
  cross-check), `line_gap_px` (`kde-breeze`: no flow container), `max_content_width_px` (all but `adwaita`, `ios-*`:
  GN-1.195, LY-2), `tightening_threshold_px` (all but `adwaita`: AdwClamp's, GN-1.195, LY-2 stating iOS's readable
  width alone), `pane_gap_px` (all but `material-3`: MAT-LA1, MAT-SP3), `vertical_pane_gap_px` (all but
  `material-3`: MAT-P3-13†), `max_panes_horizontal` (all but `material-3`, `windows-11`: MAT-P3-13†, W-P4-19†),
  `max_panes_vertical` (all but `material-3`, `windows-11`: MAT-P3-13†, W-P4-19†), `pane_width_px` (all but
  `material-3`, `windows-11`: MAT-P3-13†, W-P4-19†), `pane_height_px` (all but `material-3`, `windows-11`:
  MAT-P3-13†, W-P4-19†), `column_width_px` (all but `kde-breeze`: Kirigami's ColumnView and CardsLayout, Q1.94,
  Q1.133, K-KA-41†), `column_min_width_px` (all but `kde-breeze`: Q1.93, Q1.133), `column_max_width_px` (all but
  `kde-breeze`: Q1.133), `levitated_placement` (all but `material-3`: MAT-SF04†). Today's four are stated on every
  preset; the `Value<bool>` values are `false` where the platform does not do it (`avoids_hinge`); the limits are `Option`s, `None` where the platform sets no
  limit, never `Absent` (`flow_max_per_line`, `max_columns`); the lists are empty where the platform has none
  (`breakpoints`, `presentations`).
- **State-model source:** no states; size classes from Material's window size classes and UIKit's traits.
- **Today's fields:** the four keep their names as widget values.
- **Notes:** a label–field form is the `form` widget, on every preset (K1.210, MAC-LY02†). In context: GNOME's wrap box is `layout.values.{item_gap,line_gap} @ variant flow` (GN-2.46); Material's
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
    plain text elsewhere), `margin` (GNOME's space above the pill, GN-2.39; MAC-N32†).
  - `description` — the short description: `text`.
  - `section_title` — a credits or legal section's heading: `text`, `padding`.
  - `credits` — the credits text (macOS's credits text view): `text`, `margin` (MAC-N32†).
  - `copyright` — the copyright line: `text`, `margin` (MAC-N32†).
  - `person_name` — a person's name in the credits (KDE's in its own text style, K-P3-8†): `text`.
  - `person_task` — the person's task line (KDE's dimmed text): `text`, `opacity`.
- **Widget values:** `icon_max_width_fraction: Value<Fraction>` (the icon at most this fraction of the width: Q2.57†,
  K-KA-44†; its margins are `body.padding`, GN-2.39, K-KA-44†); `gap_px: Value<Px>` (the main page's spacing between
  the icon, name, version and the box of groups, GN-2.39; macOS's unequal gaps are the parts' margins, MAC-N32†);
  `group_gap_px: Value<Px>` (the spacing between the main page's groups of rows, the inner box's own spacing, GN-2.39,
  `_misc.scss:91-97`); `groups_margin: Sides<Value<Px>>` (that inner box's margins above and below it, GN-2.39); `subpage_margins: Sides<Value<Px>>`
  (GN-2.39); `width_px: Value<Px>` (macOS's panel, MAC-N32†); `release_gap_px: Value<Px>` (the
  space between releases in kirigami-addons' release history, K-KA-44†).
- **Variants:** keyed `kde-breeze` `@ka_about` (kirigami-addons' AboutPage, K-KA-44†). KDE's two forms are the
  flavour condition, not variants: `cond flavor:widgets` (KAboutApplicationDialog with tabs, K-P3-8†) and
  `cond flavor:kirigami` (the About page on form cards, Q2.57†, K-KA-44†).
- **Sizes and densities:** none (no row states sizes).
- **Flags:** none of its own (its link buttons are `button` elements with their own states).
- **Exclusive groups:** none (the presets that draw it, GNOME, KDE and macOS, combine states); precedence: state
  models (S17).
- **may_be_absent:** parts: `section_title` (`macos-*`: the panel's credits are one text view, MAC-N32†; `kde-breeze` by
  a rule under `cond flavor:kirigami`, where the sections are `form` groups, Q2.57†, K-KA-44†), `person_name`
  (`macos-*`: the panel's credits are one text view, MAC-N32†), `person_task` (`macos-*`: the panel's credits are one
  text view, MAC-N32†), `description` (`macos-*`: the standard About panel draws the icon, name, version, credits and
  copyright only, MAC-N32†), `credits` (`adwaita`; `kde-breeze` by a rule under `cond flavor:kirigami`: the credits text
  is KAboutApplicationDialog's, K-P3-8†, and the macOS panel's, MAC-N32†); fields (the parts' margins are the core's
  may-be-absent `margin`, S6.3, stated by `macos-*`, which places them by margins; GNOME's version pill is `version`'s
  fill, corners and padding): `icon.shadow` (all but `adwaita`: GNOME's drop shadow, GN-2.39, GN-3.07), `version.fill`
  (all but `adwaita`: GNOME's version pill, GN-2.39; a preset drawing the version as plain text states no pill box),
  `version.corners` (all but `adwaita`: GNOME's version pill, GN-2.39; a preset drawing the version as plain text states
  no pill box), and the `version.padding` is not listed: a preset drawing the version as plain text states its zero
  inset; values: `icon_max_width_fraction` (`adwaita`, `macos-*`), `gap_px` (`macos-*`: the panel's unequal gaps are the
  parts' margins, MAC-N32†), `subpage_margins` (`kde-breeze`, `macos-*`), `group_gap_px` (`kde-breeze`, `macos-*`),
  `groups_margin` (`kde-breeze`, `macos-*`), `width_px` (`adwaita`, `kde-breeze`), `release_gap_px` (`adwaita`,
  `macos-*`).
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
  - `body` — the completion's host (the root): `bindings`; it draws nothing of its own, its list being the `menu`,
    which `place()` sets against the field by `values.placement` (its distance from the field the placement's `gap`,
    S13), so the host has no inset of its own.
  - `token` — a chosen suggestion inserted into rich text (W-P3-17†): `fill`, `text`, `corners`, `padding`.
  - `details_pane` — the code editors' details pane beside the list (GN-SF13†, K-P3-13†): `fill`, `stroke`,
    `corners`, `padding`, `text`.
- **Widget values:** `filter_delay_ms: Value<Ms>` (GN-2.56†); `mode: Value<AutocompleteMode>`, `AutocompleteMode {
  Popup, Inline, PopupAndInline }` (a popup, the completion appended as selected text, or both: K2.29†; every
  preset states its platform's mode, `Popup` where the completion is a popup list); `hover_selects: Value<bool>`
  (GN-2.56†; `false` where hovering a suggestion does not select it); `placement: Placement` (where
  its list goes, the host's value by Nested controls: `GtkPopover` GN-2.56†, `QtCompletionBox` K2.29† and K-KA-13†,
  `MaterialMenu` MAT-N27, `WinUiAutoSuggest` W2-15 and W-P3-17†, `AppKitComboList` MAC-CB01, `AppKitMenu` MAC-P3-7†,
  `UiKitMenu` C-38†, `KTextEditorCompletion` K-P3-13† by a rule in `context code_editor`; with its width, the
  field's: `AnchorWidth`, GN-2.56†, K2.29†, K-KA-13†, and its row limit `max_visible_items` among its parameters).
  RichSuggestBox's prefix that opens its list (W-P3-17†) is the application's setting, not a look, so no value
  carries it.
- **Variants:** keyed `ios-*` `@menu` (the inline bar placement), `@list` (the stacked placement) (C-38†). Material's
  filled and outlined fields (MAT-N27) are the field's own (`input @filled`).
- **Sizes and densities:** none (no row states sizes).
- **Flags:** none of its own; the list's rows carry `menu`'s states, and `placed_above`, `placed_below` are `menu`'s
  (Windows: the attached corners square off on the side touching the field, W2-15).
- **Exclusive groups:** none (the completion holds no interaction state; the suggestion rows' groups are `menu`'s:
  the ListViewItem CommonStates on `windows-11`, the menu items' state layers on `material-3`); precedence: state
  models (S17).
- **may_be_absent:** parts: `token` (all but `windows-11`: RichSuggestBox's tokens, W-P3-17†), `details_pane` (all
  but `adwaita`, `kde-breeze`; `adwaita`, `kde-breeze` by a rule outside `context code_editor`: the code editors'
  details pane, GN-SF13†, K-P3-13†); values: `filter_delay_ms` (all but `adwaita`: GN-2.56†). The `mode` and the
  `hover_selects` are stated on every preset (above).
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
    ADD-B14, GN-2.14, Q2.28, K-KA-33†; the body declares `data = { body = ["text"] }`), `stroke`
    (Kirigami's ring, PersonPicture's stroke), `corners`, `padding`, `shadow` (AvatarButton's circle behind it while
    pressed: one layer with no blur and no offset, its spread the button's padding, `= surface.decoration_focus`, by
    a rule `@ {pressed} variants {@ka_avatar,@button}`, K-KA-33†, `AvatarButton.qml:105-109`, where the circle shows
    on `visualFocus || down`), `focus` (the same circle while focus is visible, `{focus_visible}`, as a focus
    indicator is drawn only then, S2.10), `cursor`
    (AvatarButton's pointing hand), `bindings` (AvatarButton's Button set), `min_width`, `min_height`, `max_width`,
    `max_height` (a fixed box, S8: every preset's base states `max_width = part.min_width` and `max_height =
    part.min_height`, so each maximum equals its minimum under every combination, as S12 requires before `initials`'
    text size may read `element.width`; the minimum is `= values.default_size` in the base, restated by the context
    rules that size an avatar, MAT-L2, MAT-TA5, MAT-N9, MAT-N14, Q2.57†, K-KA-44†; AdwAvatar's and Kirigami's size
    requests fix the box, GN-2.14, Q2.28). An avatar of a size the application chooses (an `AdwAvatar` given its
    `size`) is sized the same way: the application's own layer restates `body.min_width` and `body.min_height` by a
    rule selecting that element (a context, or a variant of its own, S8), and the base's `max_width = part.min_width`
    and `max_height = part.min_height` keep the maximum equal to it, so the box stays fixed and the initials'
    expression stays allowed (S12).
  - `initials` — the initials text: `text` (its own text style; its size follows the avatar's, an expression on
    `element.width`, which it declares, `data = { initials = ["width"] }`, ADD-B11, allowed because `body` is a
    fixed box, S12; under `@contrasted` its own
    fill).
  - `fallback_icon` — the icon shown without a name or image (Kirigami's `user`, PersonPicture's default glyph):
    `icon`, `padding`.
  - `image` — the picture, clipped to the body's shape: `corners`, `opacity`.
- **Widget values:** `default_size_px: Value<Px>` (Q2.28, W2-24, MAT-L2); `fallback_icon_inset_px: Value<Px>`
  (Q2.29).
- **Variants:** keyed `adwaita` `@contrasted` (GN-2.14); `kde-breeze` `@ka_avatar` (kirigami-addons' Avatar),
  `@button` (AvatarButton) (K-KA-33†).
- **Sizes and densities:** none (no `ControlSize`: an avatar's size is `body.min_width` and `body.min_height`,
  `default_size` in the base, restated by the context rules that size an avatar and by an application's own layer for
  a size it chooses, as `body` says).
- **Flags:** none of its own; state flags used: focus_visible, pressed (AvatarButton's circle: `body.focus` under
  `{focus_visible}`, `body.shadow` under `{pressed}`, K-KA-33†).
- **Exclusive groups:** none (W2-24 states no interaction states for PersonPicture; KDE's states combine);
  precedence: state models (S17).
- **may_be_absent:** parts: `fallback_icon` (`adwaita`: AdwAvatar's icon fallback is not in GN-2.14; Open items);
  fields: `body.stroke` (`adwaita`: AdwAvatar's circle, GN-2.14, names no ring); values: `fallback_icon_inset_px`
  (`adwaita`, `windows-11`: the inset is Kirigami's, Q2.29).
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
  - `icon` — Windows' icon form, Kirigami's icon (Q2.27): `icon` (Kirigami's size `= kirigami_label_icon_size(body_font_height)` under `cond flavor:kirigami`, the part declaring `data = { icon = ["body_font_height"] }` (Kirigami's `sizeForLabels`, which follows the user's font, Q1.1, ADD-A100)),
    `opacity` (PersonPicture's image badge, W2-24).
- **Widget values:** `anchor_offset_x_px: Value<Px>`, `anchor_offset_y_px: Value<Px>` (the placement on its anchor icon
  or item: MAT-N10, GN-2.37, C-15, GN-1.41); `presentations: Vec<PresentationTransition>`: events `Appear`,
  `Disappear` (the needs-attention dot's appearance, GN-1.41); `anchor_corner: Value<BadgeAnchor>`,
  `BadgeAnchor { TopEnd, TopStart,
  TrailingInline }` (C-15's list-row badge is trailing inline); `cutout_px: Value<Px>` (the gap libadwaita's
  indicator bin masks out of the anchor around the badge, GN-2.36); `max_count: Option<Value<u32>>` (where a count is
  capped; `None`, `"none"`: no cap).
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
- **may_be_absent:** parts: `icon` (`adwaita`, `macos-*`, `material-3`, `ios-*`: the icon form is Windows' and
  Kirigami's, W2-08, Q2.27); fields: `body.stroke` (`adwaita`, `macos-*`, `material-3`, `ios-*`; `windows-11` by a
  rule outside `context avatar`, where PersonPicture's badge has its stroke, W2-24; Kirigami's border is drawn in
  every type, Q2.25, Q2.26), `icon.opacity` (`kde-breeze`; `windows-11` by a rule outside `context avatar`: only
  PersonPicture's image badge is translucent, W2-24); values: `cutout_px` (all but `adwaita`: GN-2.36),
  `anchor_offset_x_px` (`kde-breeze`: Kirigami's Badge is an inline control, Q2.25–Q2.27; `windows-11` by a rule
  outside `context avatar`, where PersonPicture's badge margin places it, W2-24, W2-08 stating no offset; the value
  phase confirming), `anchor_offset_y_px` (`kde-breeze`: Kirigami's Badge is an inline control, Q2.25–Q2.27;
  `windows-11` by a rule outside `context avatar`, where PersonPicture's badge margin places it, W2-24, W2-08 stating
  no offset; the value phase confirming), `anchor_corner` (`kde-breeze`: Kirigami's Badge is an inline control,
  Q2.25–Q2.27; `windows-11` by a rule outside `context avatar`, the corner its offsets are measured from, W2-24; the
  value phase confirming).
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
  Kirigami's text colour follows its background (`kirigami_badge_text`, ADD-A53, its last step `kirigami_lerp` of the
  text colour and the background by 0.17; Q2.26), and its border on an application colour is
  `= kirigami_lerp(element.color, surface.foreground, 0.5)` (Q2.26).

### banner

*From group 6's catalogue.*

- **Kind:** typed widget, the in-window inline message: GNOME's AdwBanner (GN-2.11, GN-1.146; the base) and GtkInfoBar
  (GN-2.31; the keyed `@info_bar`), KDE's KMessageWidget (K2.13, K3.37), Kirigami's InlineMessage (Q2.16–Q2.21),
  kirigami-addons' FormInlineMessage (K-KA-10†) and Banner (K-KA-29†), Material's banner (MAT-N23), Windows' InfoBar
  (W2-07).
- **Available:** every preset but `macos-*`, `ios-*` (`available = false`: no in-window message bar in their rows).
- **Parts:**
  - `body` — the root: `fill` (under `cond flavor:kirigami` Kirigami's outer rectangle in the type colour, which
    shows round `inner` as the message's border, Q2.16, `InlineMessage.qml:16-33`), `stroke` (KMessageWidget's
    inline border, K2.13; W2-07's border), `corners` (per position, Q2.18; Material's shape, MAT-N23), `shadow`
    (Material's elevation, MAT-N23), `padding` (GN-2.11, GN-2.31, Q2.19, W2-07), `min_height` (W2-07).
  - `inner`, in `body` — Kirigami's inner rectangle, inset from the body's edge on the sides the position leaves
    open (Q2.17, `InlineMessage.qml:35-58`): `fill` (the window background with the type colour's layer over it),
    `margin` (its inset from the body, per position), `corners` (each corner `Radius::Px` `=
    mul(widget.body.corners.<corner>, values.inner_corner_factor)`, Q2.17's inner radius; `body` is the root and its
    corners on `kde-breeze` are `Radius::Px`, never `Full` or `Concentric`, S8, S12).
  - `icon` — the severity icon: `icon` (size and colour; recoloured to the type colour, K-KA-10†), `glyph` (Windows'
    severity glyph on its backplate, W2-07), `margin` (InfoBar's per-layout margins, W2-07).
  - `icon_backplate` — Windows' fill behind the icon: `fill`.
  - `title`, `message` — the title and the message text: `text`, `opacity` (GNOME's dimmed content under
    `cond window_inactive`, GN-2.11), `margin` (InfoBar's per-layout margins, W2-07).
  - `action_area` — the action buttons' container: `opacity` (GNOME's dimmed buttons under `cond window_inactive`,
    GN-2.11), `margin` (InfoBar's per-layout margins, W2-07).
  - `divider` — Material's divider: `stroke`.
  - `edge_line` — the line on the inner edge (KDE's header and footer positions, K2.13, K-KA-29†; GtkInfoBar's shade
    line under `@info_bar`, GN-2.31): `stroke`.
  - `image` — Material's rich banner image: `corners`, `min_width`, `min_height` (its size and shape, MAT-N23).
- **Widget values:** `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear` (the banner shown
  and hidden: GNOME's slide-down, GN-2.11; KDE's height and opacity over the timeline, K2.13, K3.37; Kirigami's
  height, opacity and
  content opacity, Q2.21; K-KA-29† states none); `actions_wrap: Value<ActionsWrap>`, `ActionsWrap { Inline, Below,
  BelowWhenNotFitting }` (KDE moves the actions below the text when they do not fit, K2.13; Windows' horizontal
  and vertical layouts, W2-07); `content_gap_px: Value<Px>` (GN-2.11, K-KA-10†, K-KA-29†); `action_gap_px: Value<Px>`
  (GN-2.11, MAT-N23); Material's heights per line count and form factor are `body.min_height` rules under `{multiline}`
  and the size class (MAT-N23; one home); `title_alignment: Value<BannerTitleAlignment>`, `BannerTitleAlignment {
  Start, Center, CenterWhenFits }` (AdwBanner centres the title in the whole banner while it fits with
  `centered_title_gap_px` on both sides of the button's width, and otherwise puts it at the start edge; under
  `vertical` it centres it, GN-2.11, `adw-banner.c:342-347`); `centered_title_gap_px: Value<Px>` (that gap,
  GN-2.11); `inner_corner_factor: Value<f32>` (Kirigami's inner radius as a fraction of the outer, read by
  `inner.corners`, Q2.17; KMessageWidget's own radius is `body.corners`, K2.13); `icon_alignment:
  Value<Alignment>` (the icon's alignment against the text across the banner, `Start` the top: Kirigami's
  InlineMessage and kirigami-addons' messages top-align it under `{multiline}`, Q2.19, K-KA-10†, K-KA-29†, by a rule
  on this value).
- **Variants:** typed severity `info`, `success`, `warning`, `error` (GN-2.31, K2.13, Q2.16, K-KA-10†, K-KA-29†,
  W2-07); typed `vertical` (the actions stacked below the text: InfoBar's vertical layout with its own padding and
  margins, W2-07; AdwBanner's stacked layout with its own button widths and spacings, GN-2.11; the orientation many
  widgets share), a content variant (Entry format) the connector sets from `actions_wrap`'s outcome on every
  platform, ignored where undeclared (S8), and declared wherever the stacked layout has a look of its own (KDE's
  actions below the text too, K2.13, Q2.20, where the value phase finds its spacing differs); keyed `adwaita`
  `@info_bar` (the deprecated GtkInfoBar, the application's choice of widget, as the widget it builds: its shade
  line, padding, gap, per-type fills and action buttons are GN-2.31's, stated under it, while AdwBanner's, GN-2.11,
  are the base; a GtkInfoBar of `GTK_MESSAGE_OTHER`, which has no severity, takes the `@info_bar` rules with no
  severity variant, its fill among them), `@question` (GtkInfoBar's question type, under `@info_bar`, GN-2.31)
  (AdwBanner in a sidebar is a context, `context sidebar.pane`, GN-1.146, rename row 91);
  `kde-breeze` `@inline`, `@header`, `@footer` (K2.13's positions, K-KA-29†), `@ka_inline_message` (K-KA-10†),
  `@ka_banner` (K-KA-29†); `material-3` `@basic`, `@rich`, `@vibrant` (MAT-N23).
- **Sizes and densities:** none (Material's heights per line count are `body.min_height` rules).
- **Flags:** shared `multiline` (KDE's top-aligned icon, K-KA-10†, K-KA-29†; Material's two-line heights,
  MAT-N23); state flags used: none other (its action and close buttons are `button` elements with their own states,
  GN-2.31).
- **Exclusive groups:** none (its close and action buttons are `button` elements, whose groups, the CommonStates on
  `windows-11`, are `button`'s); precedence: state models (S17).
- **may_be_absent:** parts: `icon_backplate` (all but `windows-11`: W2-07's backplate), `divider` (all but `material-3`:
  MAT-N23), `image` (all but `material-3`: MAT-N23's rich banner), `edge_line` (`windows-11`, `material-3`; `adwaita` by
  a rule outside `@info_bar`: AdwBanner draws no line, GN-2.11; `kde-breeze` by a rule outside `@header` and `@footer`:
  the inline position draws none, K2.13, K-KA-29†), `icon` (`adwaita`: AdwBanner and GtkInfoBar draw no severity icon,
  neither GN-2.11 nor GN-2.31 naming one), `inner` (all but `kde-breeze`: Kirigami's inner rectangle, Q2.17;
  `kde-breeze` by a rule under `cond flavor:widgets`, KMessageWidget drawing one rounded rectangle with its pen,
  `kmessagewidget.cpp:364-377`, K2.13, and by a rule under `@ka_inline_message` and under `@ka_banner`, whose
  background is the body's one fill, K-KA-10†, K-KA-29†); fields: `body.shadow` (all but `material-3`: MAT-N23's
  elevation), `icon.glyph` (`material-3`: its icon is an `icon`, MAT-N23), `body.stroke` (`material-3`; `adwaita` by a
  rule outside `context sidebar.pane`, GN-1.146, GN-2.11; `kde-breeze` under `@ka_inline_message`, K-KA-10†, and by a
  rule under
  `cond flavor:kirigami`, whose border is the body's fill showing round `inner`, Q2.16); values:
  `inner_corner_factor` (all but `kde-breeze`: Q2.17), `centered_title_gap_px` (all but `adwaita`: GN-2.11),
  `icon_alignment` (`adwaita`, whose banners draw no icon). A drawn banner's height and its title's alignment are always
  measurable, so neither is listed: every preset states the `body.min_height` and the `title_alignment` (measured where
  no source states them, S17); the `presentations` list is empty where no row states one (`windows-11`, `material-3`).
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
  NavigationView is `sidebar` and, in Top mode, `view_switcher`); `kde-breeze` flavour `kirigami` only. Both macOS
  presets declare it: SwiftUI's tab bar is macOS 15's (MAC-N23), `macos-monterey` describes macOS 15, its
  generation's newest release (R1.5), and `macos-tahoe` macOS 26.
- **Parts:**
  - `bar` — the bar (the root): `fill` (paint with material; transparent at the scroll edge by `scrolled`, T-1),
    `stroke`
    (the top hairline, one edge), `corners`, `shadow` (the floating capsule's, iOS 26, T-9), `margin` (the floating capsule's distance from the window's edges, not the safe area's,
    as T-9 measures it from the screen's edges, by a rule under `floating`), `min_height` (by size class: T-14, ADD-A20), `padding`.
  - `item` *(element)* — one destination: `fill`, `stroke`, `corners`, `padding`, `min_height` (Q2.34), `focus`
    (Q2.36), `bindings` (Kirigami's Window set, Q2.34), `transitions` (Q2.35); per-state colours (normal, selected,
    disabled, focused: T-3, MAT-N12). `item_icon`, `item_label`, `indicator` belong to `item` and are queried with it.
  - `item_icon` — `icon` per state (the selected symbol variant: `item_icon.icon.symbol_variants`, ADD-A37), `glyph`
    (the More item's system glyph, `GlyphSource::NamedSystemImage`, by a rule `@ variant @more`: UIKit draws it for
    `UITabBarItem.SystemItem.more`, its symbol named by P3-2†'s measurement), `transitions` (Material's icon colour, MAT-N12).
  - `item_label` — `text` per state (weight under `{selected}`, elision), `transitions` (Material's label colour, MAT-N12).
  - `indicator` — the selected item's indicator (Material's active indicator, MAT-N12; iOS 26's glass lens, T-8):
    `fill`, `corners`, `min_width`, `min_height`, `opacity` (Material's, zero with the width outside `{selected}`, as
    Compose grows and fades it in, MAT-N12), `transform` (the lens growing while it follows a drag across the bar,
    iOS 26, T-8, as `segmented_control.indicator`'s, SG-4), `transitions`.
  - `state_layer`, in `item` — Material's ripple pill (MAT-N12): Compose draws an item's state layer and press ripple
    on a box of its own, laid out where the indicator lies at the indicator's full size, unaffected by the
    indicator's width and alpha animation (`NavigationBar.kt:247-272`, measured `:564-569`, placed `:613-623`;
    `NavigationItem.kt:690-700`, `:1232-1238`, the Expressive items): `interaction` (S2.12), `corners` (the
    indicator's shape), `min_width`, `min_height` (the indicator's full size), `transitions` (the block's fades,
    MAT-X9). It belongs to `item` and is queried with it.
  - `drop_indicator` — where a dragged tab would land (T-17†): `fill`, `min_width`, `min_height`.
  - `accessory`, in `bar` — iOS 26's bottom accessory, the capsule above or inside the bar (`UITabAccessory`, C-21,
    T-12): `fill` (a `Material` paint), `corners` (a capsule), `shadow`, `padding` (its insets), `min_height` (its
    height, C-21). `accessory_content` nests in it.
  - `accessory_content`, in `accessory` — the application's view area in it: `padding`.
- **Widget values:** `item_gap_px: Value<Px>`, `item_width_px: Value<Px>`,
  `item_positioning: Value<Key>` (keys `automatic`, `fill`, `centered`, the cases of `UITabBar.ItemPositioning`,
  T-3); `max_items: Value<u32>` (P3-2†, by size class); `max_content_width_px: Value<Px>` (Q2.33; `auto` where the
  platform sets no maximum, the items spreading over the bar, never `Absent`), `equal_width_items:
  Value<bool>` (Q2.33; `false` where the platform does not give the items equal widths); `icon_text_gap_px:
  Value<Px>` (an item's icon to its label, Q2.34); the values read the item count, `data = { values = ["item_count"]
  }` (Kirigami's content width, Q2.33): the connector queries `values()` once, with the bar's own element, and
  supplies the bar's item count as that element's `item_count` (`ElementData`, S8, R25.3), so the values and the
  bar's events below are the bar's; `presentations: Vec<PresentationTransition>` (queried with the bar): events
  `@indicator_move` (the selection changing: a step on the item's part `indicator`, the selection slide, Morph, T-8,
  and the items' own change, T-15; `Instant` on `material-3`, whose items grow and fade their own indicators,
  MAT-N12),
  `Appear`, `Disappear` (the bar shown and hidden, `setTabBarHidden(_:animated:)`, T-15; AdwViewSwitcherBar's reveal,
  GN-2.15), `@minimize` (T-10), `@customize` (T-17†, P3-2†), `@tab_sidebar_morph` (iPad's tab bar becoming a sidebar, C-22; one occasion with `sidebar`'s),
  `@spring_load` (an item a drag hovers switching to it, P3-11†), `@accessory_environment` (the bottom accessory
  morphing between its environments, C-21);
  `focus_visible_when` (type `FocusVisibleWhen`; Q2.36, ADD-A16).
- **Variants:** typed `floating` (iOS 26's capsule, T-9); typed item layouts, the application's choice, `stacked`
  (the icon above the label: Kirigami's `TextUnderIcon`, Q2.34; Material Expressive's vertical items, MAT-N12) and
  `inline` (the icon beside the label: Kirigami's `TextBesideIcon`, Q2.34; Material Expressive's horizontal items,
  MAT-N12), on `material-3` flavour `expressive` only (declared under `[declare.flavor.expressive.widget.
  bottom_tab_bar]`, S10, so `flavor:baseline` answers `NotAvailable` for them). iOS's stacked, inline and
  compact-inline layouts (T-3) are no variant: UIKit picks them from the trait collection, so `ios-*` states them as
  rules `@ h_size_class compact` (stacked), `@ h_size_class regular` (inline) and `@ v_size_class compact` (compact
  inline), the later rule winning where two hold, the value phase confirming UIKit's choice;
  the content variants `icon_only` and `with_icon` on `item` (ADD-A61; an item with neither is text-only, Kirigami's
  larger text-only label, Q2.34); keyed `ios-*` `@search` (T-11), `@prominent` (T-16),
  `@tab_sidebar` (iPadOS's top bar, T-13), `@accessory_inline`, `@accessory_expanded` (the bottom
  accessory's environments, `UITabAccessory.Environment`, T-12; Open items), and on `item` `@more` (the More item's
  identity: one item's, not the bar's, P3-2†); `macos-*` `@sidebar_adaptable`
  (MAC-N23).
- **Sizes and densities:** none; the bar's height follows the size class (`v_size_class compact`, T-14).
- **Flags:** shared `minimized` (iOS, T-10); state flags used: hover, pressed, selected (the current destination
  on every platform: Kirigami's checked tab button maps to it, Q2.35; S7.2), disabled, focus_visible, dragged,
  drop_target (on `bar`, T-17†), scrolled (T-1).
- **Exclusive groups:** none (states combine; `material-3`'s state layers are a recency set, below); precedence: state
  models (S17). Recency sets: `material-3`: `state_layer = [focus_visible, hover]` (the navigation bar's state layers,
  MAT-N12; Material's recency sets, above: Compose's ripple, `NavigationBar.kt:268-271`; `pressed` combines).
- **may_be_absent:** parts: `indicator` (`kde-breeze`, `adwaita`: Kirigami's and AdwViewSwitcherBar's items draw
  none, Q2.35, GN-2.15; `ios-13` by a rule outside `@tab_sidebar` and by a rule outside `{selected}`: iPadOS 18's
  top bar marks the selected item, T-13, and the bottom bar draws no indicator before iOS 26, T-8; `ios-26` by a
  rule outside `{selected}`; never on `material-3`, which draws it at rest at zero width and alpha, MAT-N12), `drop_indicator` (all but `ios-*`: T-17†), `state_layer` (all but `material-3`: MAT-N12), `accessory` (all but
  `ios-26`: iOS 26's bottom accessory, C-21, T-12), `accessory_content` (all but `ios-26`: iOS 26's bottom
  accessory, C-21, T-12); fields: `bar.stroke` (`ios-26` by a rule under `floating`: T-2's top hairline is the
  iOS 13–18 bar's), `bar.shadow` (`kde-breeze`, `ios-13`: their bars are flat); values: `max_items` (all but `ios-*`: P3-2†), `item_positioning` (all but `ios-*`: the keys are
  `UITabBar.ItemPositioning`'s, T-3), `icon_text_gap_px` (the value phase names the presets from the facts). The
  `equal_width_items` is a `Value<bool>`, stated on every preset, and the `max_content_width_px` a length or `auto`
  (above).
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
  a sidebar (`@tab_sidebar_morph`, C-22) this widget's, placed at the top; `view_switcher` takes the switchers that are no
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
  - `bottom_separator` — Kirigami's Header-set line (Q2.44): `stroke`, `margin` (its top and bottom margins: a
    line's insets are its `margin`, as a stroke lies on its box's edge, S2.4), `bindings` (the Header set).
- **Widget values:** `icon_text_gap_px: Value<Px>` (between a crumb's icon and its label, GN-2.52†);
  `presentations: Vec<PresentationTransition>`: event `@scroll` (Kirigami's scroll to the current crumb, Q2.44;
  `[]` on `adwaita`, whose path bar scrolls without animation, GN-2.52†, `gtkpathbar.c:195-203`);
  `editable: Value<bool>` (KDE's editable-path
  mode, whose field is an `input` in context, K2.27†); `drag_hover_delay_ms: Value<Ms>` (a drag resting on a
  crumb's arrow opens its sub-folder menu after this delay, K-P4-11†).
- **Variants:** the content variant `with_icon` on `crumb` (GtkPathBar's root, home, desktop and recent buttons,
  GN-2.52†; ADD-A61); keyed `macos-*` `@standard`, `@pop_up` (MAC-N07).
- **Sizes and densities:** none (no row states sizes).
- **Flags:** shared `current` (the current folder's crumb: KDE's label, K2.27†, Q2.44; Windows' current-item
  colour, W2-10; macOS, MAC-N07; GNOME's current folder button, whose toggle is checked, maps to it, GN-2.52†);
  keyed `kde-breeze` `@navigator_active` (the navigator of the application's active view, K2.27†: set by the
  application, which knows its active view, as KUrlNavigator's `setActive` is; it selects nothing where undeclared); state
  flags used: hover, pressed, focus_visible, disabled, open (a crumb's popup shown), drop_target (dragged over,
  K2.27†, K-P4-11†).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (BreadcrumbBarItem CommonStates; `current`
  combines); others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `ellipsis` (all but `windows-11`: W2-10's overflow item), `placeholder` (all but
  `macos-*`: MAC-N07), `crumb_icon` (`windows-11`: BreadcrumbBar's items, W2-10, carry no icon), `separator` (`adwaita`:
  GtkPathBar links its buttons without separators, GN-2.52†), `bottom_separator` (all but `kde-breeze`: Kirigami's
  Header-set line, Q2.44); values: `drag_hover_delay_ms` (all but `kde-breeze`: K-P4-11†), `icon_text_gap_px`
  (`windows-11`: its crumbs show no icon, as `crumb_icon`). The `editable` is a `Value<bool>`, stated on every
  preset (`false` where the path is not editable in place).
- **State-model source:** GNOME `gtkpathbar.c` and the button selectors; KDE `kurlnavigatorbutton*.cpp`,
  `kurlnavigatorbuttonbase.cpp` and `BreadcrumbControl.qml`; macOS `NSPathControl.Style`; Windows
  BreadcrumbBarItem visual states.
- **Today's fields:** none (W2-10's stand-in `link.*` stays `link`'s).
- **Notes:** a crumb is the path control's own piece on every platform (Nested controls): GNOME's path-bar toggle
  buttons are `crumb` elements, their facts GtkPathBar's button selectors and their joined corners `pos` rules
  (GN-2.52†). Nested controls: KDE's places selector is `button @ variant menu context breadcrumb` (KUrlNavigator's,
  K2.27†); the ellipsis's overflow flyout (W2-10) and KDE's sub-folder menus are `menu` elements in
  `context breadcrumb`; the editable path's field an `input` there (K2.27†). GtkPathBar's row of crumbs lies in a
  scrolled window that shows no scroll bar (`scroll_view @ context breadcrumb.bar`: horizontal policy
  `GTK_POLICY_EXTERNAL`, vertical `GTK_POLICY_NEVER`, `gtkpathbar.c:195-198`, so no `scrollbar` element is queried in
  it), overflowing without animation (GN-2.52†). Group 8 had named the drag delay
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
  - `title`, in `header` — the month and year text where it is not a button: `text`.
  - `weekday_label` *(element)* — one weekday's header cell, with its own flags (a non-working day's colour,
    K2.23†): `text` (its own text style; non-working days' header colour, K2.23†),
    `fill` (on `Window`, K2.23†), `stroke` (the line under the header, K2.23†), `opacity` (K-KA-21†).
  - `week_number` — `text`.
  - `day` *(element)* — a day cell: `fill`, `corners`, `stroke`, `padding` (GN-2.25, W2-02), `min_width`,
    `min_height` (Material's cell size, MAT-N16), `opacity` (other-month days: K-KA-21†, MAT-N16), `focus` (GN-2.25,
    K-KA-21†), `transitions` (the `interaction` block's fades; Material's container switches at once, MAT-N16), `interaction` (Material's state layer and press ripple, S2.12).
    `day_label`, `today_marker`, `decoration`, `range_band` belong to `day`, nest in it and are queried with it.
  - `day_label` — its number: `text` (tabular figures, GN-2.25; iOS's `fontDesign`, C-6; Windows' blackout
    strikethrough, W2-02, a `Strike` decoration under `{@blackout}` whose `thickness` and `offset` the value phase
    states from CalendarView's source: references to `element.font_strikeout_thickness` and
    `element.font_strikeout_offset`, declared `data = { day_label = ["font_strikeout_offset",
    "font_strikeout_thickness"] }`, where WinUI draws the line from the font, a number where it draws a fixed line,
    S2.8), `transitions` (Material's number colour, MAT-N16).
  - `today_marker` — GNOME's underline, Material's outline: `stroke`, `corners`.
  - `decoration` — iOS's dot, image or custom decoration (C-6): `fill`, `corners`, `icon`, `min_width`, `min_height`.
  - `range_band` — Material's range selection band behind a day, by the day's own `@range_start`, `@range_end`,
    `@in_range` (MAT-N16): `fill`, `min_height`.
  - `year` *(element: Material's year cells, Windows' year and decade items, K-KA-21†'s year view)*: `fill`, `stroke`,
    `corners`, `text`, `min_width`, `min_height` (MAT-N16), `opacity` (years outside the decade, K-KA-21†), `focus`, `transitions` (Material's container and text colours, MAT-N16), `interaction` (Material's state layer and press ripple, S2.12).
  - `month` *(element: Windows' month items, K-KA-21†'s month view)*: `fill`, `stroke`, `corners`, `text`, `focus`.
  - `clock_face` — macOS's clockAndCalendar style: `fill`, `stroke`.
  - `clock_hand` — macOS's clockAndCalendar style: `stroke`.
  - `time_row` — iOS's time row: `text`, `padding`.
- **Widget values:** `cell_gap_px: Value<Px>`; `grid_padding: Sides<Value<Px>>` (GN-2.25); `presentations:
  Vec<PresentationTransition>`: event `@view_switch` (Windows' zoom between the day, month and year views, W2-02;
  kirigami-addons' view switch, K-KA-20†); `first_weekday_from_locale: Value<bool>`; `scroll_physics:
  Value<ScrollPhysics>` (S4.2's record, absent as a whole: kirigami-addons' endless views, deceleration, snap to a page, K-KA-20†).
- **Variants:** keyed `ios-*` `@wheels` (the month-year wheel, C-6); `kde-breeze` `@ka_date_popup` (K-KA-20†,
  K-KA-21†). The view shown (Windows' month, year and decade views, W2-02; kirigami-addons' day, month and year views,
  K-KA-21†) is no variant: it is which element parts the connector queries, `day`, `month` or `year`, each styled the
  same way on every platform, and the switch between them is the `@view_switch` presentation.
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
  states show); others none (states combine; `material-3`'s state layers are a recency set, below); precedence: state
  models (S17). Recency sets: `material-3`: `state_layer = [focus_visible, hover]` (the date cells' state layers;
  Material's recency sets, above: a `Surface`'s ripple, `DatePicker.kt:2254-2256`; `pressed` combines).
- **may_be_absent:** parts: `title` (`adwaita`, `kde-breeze`, `windows-11`, `ios-*`: the month and year there are
  `button` elements in `context calendar.header`, GN-2.25, K2.4, W2-02, iOS's month-year toggle, C-6, or
  kirigami-addons' segmented button, K-KA-20†),
  `week_number` (all but `kde-breeze`: K2.4), `decoration` (all but `ios-*`: C-6), `time_row` (all but `ios-*`:
  C-6), `range_band` (all but `material-3`: MAT-N16), `clock_face` (all but `macos-*`: MAC-N04), `clock_hand` (all
  but `macos-*`: MAC-N04), `today_marker` (`kde-breeze`, `windows-11`, `ios-*`: today is the day's fill or text,
  K2.23†, K-KA-21†, W2-02, C-6), `month` (`adwaita`, `ios-*`, `material-3`, `macos-*`: the month view is Windows'
  and kirigami-addons', W2-02, K-KA-21†), `year` (`adwaita`, `ios-*`, `macos-*`: the year view is Windows',
  kirigami-addons' and Material's, W2-02, K-KA-21†, MAT-N16); fields: `weekday_label.fill` (all but `kde-breeze`:
  KDateTable's header on `Window`, K2.23†), `weekday_label.stroke` (all but `kde-breeze`: the line under
  KDateTable's header, K2.23†), `weekday_label.opacity` (all but `kde-breeze`: K-KA-21†), `header.stroke` (all but
  `adwaita`, `kde-breeze`: GN-2.25, K2.23†), `frame.fill` (`ios-*`, `material-3`, `macos-*`: the grid draws no frame
  of its own), `frame.stroke` (`ios-*`, `material-3`, `macos-*`: the grid draws no frame of its own); values:
  `scroll_physics` (all but `kde-breeze`: K-KA-20†), `grid_padding` (all but `adwaita`: GN-2.25).
- **State-model source:** GNOME `_calendar.scss` selectors; KDE `KDateTable` and Breeze `PE_PanelItemViewItem`,
  kirigami-addons `DatePicker.qml`, `DatePickerDelegate.qml`; Windows `CalendarView` item states; Material
  date-picker tokens; iOS `UICalendarView` and `UIControl.State`; macOS `NSDatePicker` graphical style.
- **Today's fields:** none.
- **Notes:** Nested controls: the previous and next buttons are `button @ variant previous / next context
  calendar.header`, and Windows' month-year header button and iOS's month-year toggle `button @ !variants
  {previous,next} context calendar.header` (W2-02, C-6; the `!variants` term keeps the navigation buttons out, which
  the same context also matches; iOS's `fontDesign` on its label is `button.label.text.font` under that selector).
  A date picker's grid is this widget in `context date_picker`: Material's (MAT-N16), macOS's
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
- **may_be_absent:** fields: `item.stroke` (all but `material-3`: MAT-N18's item outline), `item.shadow` (all but
  `material-3`: MAT-N18's item elevation), `item.fill` (`adwaita`, `ios-*`: their pages draw no item background);
  values: `scroll_timeout_ms` (all but `adwaita`: GN-2.38), `nav_buttons_visibility` (all but `windows-11`: W2-31);
  the `presentations` list is empty where the platform has none, and the `item_gap_px` is required: AdwCarousel's
  spacing default and FlipView's adjacent pages are gaps the facts state (a zero length where the pages touch), not
  an undrawn thing.
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
  rows); `kde-breeze` flavour `kirigami` only (KQuickCharts). All four Apple presets declare it, each generation
  describing its newest release (R1.5): Swift Charts is macOS 13's (MAC-N37†), so `macos-monterey` (macOS 15) and
  `macos-tahoe` declare it, and iOS 16's (C-45†), so `ios-13` (iOS 18) and `ios-26` declare it.
- **Parts:**
  - `body` — the chart (the root): `fill` (KDE's View background, K-P3-10†), `padding`, `bindings`.
  - `plot_area` — the plotting area, holding the grid lines, the series' marks and the selection mark: `padding` (its
    insets, C-45†).
  - `grid_line_major`, `grid_line_minor`, in `plot_area` — grid lines: `stroke`.
  - `axis_tick` — `stroke` (C-45†).
  - `axis_label` — value labels: `text` (font, colour).
  - `legend` — the legend's box, holding its entries: `padding` (K-P3-10†).
  - `legend_entry` *(element)*, in `legend` — one series' entry, its `item_index` and `item_count` its element data
    (K-P3-10†'s entries, C-45†, MAC-N37†): `padding` (KQuickCharts' entry padding, K-P3-10†). `legend_swatch` and
    `legend_label` belong to it and are queried with it.
  - `legend_swatch` — the entry's colour mark, its colour as the series': `fill`, `min_width` (K-P3-10†).
  - `legend_label` — the entry's label, the legend's one text style (the legend font, C-45†, MAC-N37†): `text`.
  - `series` *(element)*, in `plot_area` — a series' mark: `stroke` (a line series' stroked path, its width and
    paint per series by the same expressions as the fill: KQuickCharts' line chart under `@line`, K-P3-10†; Swift
    Charts' `LineMark`, C-45†, MAC-N37†; `Absent` where the series is drawn filled only, S6.3), `fill` per series,
    `= pick(element.item_index, <c0>, …)` in Swift Charts' default order (C-45†, MAC-N37†) or `=
    kqc_series_color(<seed>, element.item_count, element.item_index)` from the highlight colour on KDE (ADD-A57,
    ADD-B15, K-P3-10†), the parts declaring `data = { series = ["item_index", "item_count"], legend_swatch =
    ["item_index", "item_count"] }` (the swatch reads its entry's data).
  - `selection_mark`, in `plot_area` — Swift Charts' selection mark: `stroke`.
- **Widget values:** `grid_lines: ChartGridLines { horizontal: GridLineCounts, vertical: GridLineCounts }`,
  `GridLineCounts { major_frequency: Value<u32>, minor_frequency: Value<u32>, major_count: Value<u32>, minor_count:
  Value<u32> }` (KQuickCharts' grid: every n-th value line, or n lines, major and minor, K-P3-10†; Swift Charts'
  `AxisMarks` defaults, C-45†); `labels_per_axis: Value<u32>` (K-P3-10†); `legend_gap_px: Value<Px>` (K-P3-10†);
  `default_size_px: Value<Px>` (KQuickCharts' pie control, K-P3-10†).
- **Variants:** keyed `kde-breeze` `@line`, `@pie` (K-P3-10†).
- **Sizes and densities:** none.
- **Flags:** none (the rows state no interaction states; Swift Charts' selection is the `selection_mark` part).
- **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** parts: `selection_mark` (`kde-breeze`: the selection mark is Swift Charts', MAC-N37†; KQuickCharts'
  controls, K-P3-10†, have none); fields: `grid_line_minor.stroke` (`ios-*`: C-45† names one grid line, `AxisGridLine`;
  MAC-N37† traces the minor lines on `macos-*`); values: `labels_per_axis` (`macos-*`, `ios-*`: KQuickCharts' label
  count, K-P3-10†), `default_size_px` (`macos-*`, `ios-*`: KQuickCharts' pie control, K-P3-10†), `grid_lines` (each
  member the platform's grid has no counterpart for: KQuickCharts' grid takes a frequency or a count, K-P3-10†, Swift
  Charts' `AxisMarks` a count, C-45†; the value phase names the presets from the facts).
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
    (MAT-N9), `opacity` (Material's disabled chips, MAT-N9), `focus` (MAT-N9), `transitions` (Material's elevation
    motion, MAT-N9; ADD-A124), `interaction` (Material's state layer and press ripple, S2.12).
  - `label` — `text`.
  - `leading_icon`, `trailing_icon` — `icon`.
  - `menu_arrow` — the token's menu arrow (MAC-N03): `glyph`.
- **Widget values:** `icon_text_gap_px: Value<Px>` (Q2.24); KDE's extra padding when there is no icon is
  `body.padding @ !variant with_icon` (Q2.24; one home).
- **Variants:** keyed `material-3` `@elevated` (against flat), `@assist`, `@filter`, `@input`, `@suggestion`
  (MAT-N9); `kde-breeze` `@interactive` (Q2.24); `macos-*` `@rounded`, `@squared`,
  `@plain_squared` (the token styles, MAC-N03); typed `closable` (Kirigami's chip with its close button, Q2.24: the
  meaning `tab`'s `closable` shares); the content variant `with_icon` (ADD-A61).
- **Sizes and densities:** none (no row states sizes).
- **Flags:** none of its own; state flags used: checked (KDE, Material's filter chip: Q2.22, MAT-N9), selected
  (Material's input chip, Windows' token, macOS's selected token: MAT-N9, W2-43†, MAC-N03), dragged (MAT-N9),
  hover, pressed, focus_visible, disabled.
- **Exclusive groups:** `windows-11`: `[pressed, hover]` (the token's CommonStates: rest, pointer-over, pressed;
  `selected` combines, as its selected-pointer-over and selected-pressed states show, W2-43†); others none (states
  combine; `material-3`'s state layers and elevation are recency sets, below); precedence: state models (S17).
  Recency sets: `material-3`: `state_layer = [dragged, focus_visible, hover]` (the chips' state layers; Material's
  recency sets, above: a `Surface`'s ripple, `Chip.kt:3207-3208`; `pressed` combines) and `elevation = [dragged,
  pressed, focus_visible, hover]` (MAT-N9's elevations, `Chip.kt:3612-3666`, `:3751-3805`).
- **may_be_absent:** parts: `trailing_icon` (all but `material-3`: Material's chips alone draw a trailing icon,
  MAT-N9; Kirigami's and Windows' removing control is a `button`, Q2.24, W2-43†), `leading_icon` (`windows-11`;
  `macos-*`: NSTokenField's tokens hold no image, MAC-N03), `menu_arrow` (all but `macos-*`: MAC-N03); fields:
  `body.shadow` (all but `material-3`; `material-3` by a rule outside `@elevated`, its flat chips, MAT-N9); values:
  `icon_text_gap_px` (`macos-*`, `windows-11`: their tokens show no icon beside the label, as `leading_icon`).
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
    under it, `= element.color`, which the connector supplies as the part's data, K-P3-11†), `min_width`,
    `min_height` (the line, and Vi mode's block, underline and half block, K-P3-11†: the block's width the `text`
    part's character width, `widget.text.char_width`, the caret having no font of its own, S2.2; `data = { caret =
    ["color", "widget.text.char_width"] }`).
  - `secondary_caret` — the bidirectional secondary caret, as on `input` and `text_area` (GtkSourceView's: its
    colour's base `= token.caret.secondary`, GN-3.37; restated by the scheme's `secondary-cursor` where stated, else
    derived from the scheme's `cursor` at the source's alpha factor, GN-SF03†): `fill`, `min_width`, `min_height`.
  - `block_caret` — the block caret in overwrite mode, as on `input` and `text_area`, drawn under `{overwrite}` in
    the line caret's place (GtkSourceView's, drawn only while the view is focused and holds no selection, GN-P4-4†):
    `fill`, `text` (the covered character redrawn in it, GN-P4-4†).
  - `extra_caret` *(element)* — one of KDE's multiple cursors, one element per secondary caret, drawn as the caret
    is (K-P3-11†): `fill` (the colour of the text under that caret, `= element.color`, which the connector supplies
    as the element's data), `min_width`, `min_height` (the line, and under the editor's Vi variants the caret's
    block, underline and half block, by the same rules as `caret`'s; its block width the character width of the
    editor's font), `text` (that font, which it measures and does not draw: a reference to the style the `text`
    part's `text` references, as an *(element)* part reads none of its host's parts, S2.2); `data = { extra_caret =
    ["color", "char_width"] }`.
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
  - `find_match` *(element)* — a search match, as on `text_area` (the schemes state one match style, no current-match
    style: GN-SF08†, K-SF08†,
    K2.30†): `fill`, `text`.
  - `replace_match` (GN-SF08†, K-SF08†) — `fill`, `text`.
  - `right_margin` (GN-SF09†, K-SF09†) — `stroke`.
  - `right_margin_area` (GN-SF09†) — `fill`.
  - `dynamic_wrap_indicator` (K-SF09†) — `fill`.
  - `background_pattern` (GN-SF11†) — `fill`.
  - `syntax_span` *(element)* — a run of one syntax role, selected by its own variant (the role) and its own
    `{text_selected}` (its text lies in the selection: KDE's per-style selected-text colours, K-SF10†) (GN-SF10†,
    K-SF10†): `text`, `fill` (the optional background).
  - `misspelling` — the spell-check mark, as on `input` and `text_area` (GN-P3-27†, K-SF11†): `text` (the
    underline: Pango's error underline, GN-P3-27†, and KTextEditor's spell-check underline, K-SF11†, drawn from the
    font, so its decoration's `thickness` and `offset` are references to `element.font_underline_thickness` and
    `element.font_underline_offset`, S2.8; `data = { misspelling = ["font_underline_offset",
    "font_underline_thickness"] }`).
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
  shapes, the editor's mode, K-P3-11†: the rules under them set `caret`'s and each `extra_caret` element's fill and
  sizes, an *(element)* part's query seeing its widget's variants, S2.11). `invisible`:
  typed `space`, `tab`, `line_break`, `no_break_space` (`text_area`'s, GN-SF06†, K-SF06†). `inline_annotation`: typed
  `warning`, `error`, keyed `adwaita` `@accent` (GN-SF12†).
- **Sizes and densities:** none.
- **Flags:** shared `overwrite` (GNOME, KDE: the block caret), `text_selected` (as on `input`; on `syntax_span`, its
  text in the selection, K-SF10†); keyed `kde-breeze`
  `@line_has_mark_bookmark`,
  `@line_has_mark_breakpoint_active`, `@line_has_mark_breakpoint_reached`, `@line_has_mark_breakpoint_disabled`,
  `@line_has_mark_execution`, `@line_has_mark_warning`, `@line_has_mark_error` (on `current_line`: KDE mixes the
  current line with the marks the line carries, K-SF03†); state flags used: focused (on the editor, which the `selection` rules select by context; on
  `template_placeholder`), read_only (on `template_placeholder`, K-SF11†), hover (on `annotation_border`,
  K-P3-14†; on `minimap`), disabled.
- **Exclusive groups:** none (GtkSourceView's and KTextEditor's states combine); precedence: state models (S17).
- **may_be_absent:** parts: `extra_caret` (`adwaita`: KTextEditor draws it alone, K-P3-11†), `fold_marker`
  (`adwaita`: KTextEditor draws it alone, K-SF04†), `mark` (`adwaita`: KTextEditor draws it alone, K-SF04†),
  `modified_line_marker` (`adwaita`: KTextEditor draws it alone, K-SF05†), `saved_line_marker` (`adwaita`:
  KTextEditor draws it alone, K-SF05†), `indentation_guide` (`adwaita`: KTextEditor draws it alone, K-SF06†),
  `dynamic_wrap_indicator` (`adwaita`: KTextEditor draws it alone, K-SF09†), `template_field` (`adwaita`:
  KTextEditor draws it alone, K-SF11†), `template_placeholder` (`adwaita`: KTextEditor draws it alone, K-SF11†),
  `annotation_border` (`adwaita`: KTextEditor draws it alone, K-P3-14†), `minimap_fade` (`adwaita`: KTextEditor
  draws it alone, K-P3-12†), `scroll_mark` (`adwaita`: KTextEditor draws it alone, K-P3-12†, K-SF05†),
  `bracket_mismatch` (`kde-breeze`: KTextEditor's schemes state `BracketMatching` only, K-SF07†; `adwaita` in its
  base, the Adwaita schemes stating no mismatch style, GN-SF07†; `@classic` restates it where that scheme states
  one), `replace_match` (`adwaita`: GtkSourceView's schemes state a search match only, GN-SF08†),
  `right_margin_area` (`kde-breeze`: KTextEditor fills only the dynamic-wrap area, K-SF09†), `background_pattern`
  (`kde-breeze`: GtkSourceView draws it alone, GN-SF11†), `inline_annotation` (`kde-breeze`: GtkSourceView draws it
  alone, GN-SF12†), `secondary_caret` (`kde-breeze`: GtkSourceView draws it alone, GN-SF03†), `caret` (`adwaita` by
  a rule under `{text_selected}` and by a rule under `{overwrite}`, as on `input`; `kde-breeze` by a rule under
  `{overwrite}`, its caret staying while text is selected, K3.43), `block_caret` (`adwaita` by a rule under
  `!{overwrite}`, by a rule under `!{focused}` and by a rule under `{text_selected}`, GN-P4-4†; `kde-breeze` by a
  rule under `!{overwrite}`); values: `minimap_width_px` (`adwaita`: K-P3-12†'s minimap width is KTextEditor's),
  `gutter_padding` (`kde-breeze`: GN-SF05†'s gutter padding is GtkSourceView's).
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
  `block_caret`, `secondary_caret`, `misspelling` (parts) and `selection` (an element) on `input`, `text_area` and
  `code_editor`; `invisible` (typed `space`, `tab`, `line_break`, `no_break_space`) and `find_match` (on `text_area` the current match `{current}`),
  elements, on `text_area` and `code_editor`. Group 6's `bracket_match_animation` is `bracket_match.transitions`
  (K-SF07†).

### color_button

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's GtkColorDialogButton (GN-2.27), KDE's KColorButton (K2.20) and kirigami-addons'
  colour row swatch (K-KA-14†), macOS's `NSColorWell` (MAC-N05), Windows' Toolkit ColorPickerButton (W2-42†), iOS's
  `UIColorWell` (C-8).
- **Available:** every preset but `material-3` (`available = false`: no colour button in its rows).
- **Parts:**
  - `body` — the button (the root): `fill`, `stroke`, `corners` (its frame: K2.20, GN-2.27), `padding` (the swatch's
    inset in the button: GN-2.27, W2-42†; KColorButton's swatch at the push button's contents rectangle less half
    `PM_ButtonMargin` on each side, `kcolorbutton.cpp:159-161`, K2.20), `focus`.
  - `swatch` — the colour: `corners`, `min_width`, `min_height` (W2-42†), `stroke` (W2-42†'s preview border).
  - `swatch_ring` — GNOME's light-swatch ring (its paint `= gtk_contrast_pick(element.color, <on light>, <on dark>)`,
    GTK's lightness test, GN-2.27, ADD-A94; the part declares `data = { swatch_ring = ["color"] }`), macOS's active
    ring, iOS's multicolour ring: `stroke`, `corners` (GNOME's overlay radius, GN-2.27).
  - `checkerboard` — under a translucent colour (W2-42†): `fill`.
  - `chevron` — macOS's minimal style, Windows' drop-down chevron: `glyph` (per state).
  - `expand_button` *(element, macOS's expanded style)*: `fill`, `glyph`.
- **Widget values:** `swatch_gap_px: Value<Px>` (the space before the row's arrow, K-KA-14†). The swatch's inset is
  `body.padding` on every preset (one home).
- **Variants:** keyed `macos-*` `@minimal`, `@expanded` (MAC-N05; the default style is the base), `@hdr` (a well that
  takes HDR colours,
  `maximumLinearExposure` above one, MAC-N05).
- **Sizes and densities:** `macos-*`: the control sizes (MAC-N05; which: value phase).
- **Flags:** none of its own: macOS's active well (MAC-N05), the well whose colour panel is open, is the state flag
  `open` (S7.2), which the connector sets from the panel it opened on every platform; state flags used: hover,
  pressed (C-8, W2-42†), focus_visible, disabled, open.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (ColorPickerButton's CommonStates, W2-42†);
  others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `chevron` (`adwaita`, `kde-breeze`, `ios-*`: the chevron is macOS's minimal style's
  and Windows' drop-down's, MAC-N05, W2-42†; `macos-*` by a rule outside `@minimal`), `expand_button` (all but
  `macos-*`: MAC-N05's expanded style; `macos-*` by a rule outside `@expanded`), `swatch_ring` (`kde-breeze`,
  `windows-11`: the ring is GNOME's, macOS's and iOS's, GN-2.27, MAC-N05, C-8; `macos-*` by a rule outside
  `{open}`, its active ring marking the well whose colour panel is open, MAC-N05), `checkerboard` (the value phase
  names the presets from the facts; W2-42† draws one); values: `swatch_gap_px` (all but `kde-breeze`: K-KA-14†);
  fields: `swatch.stroke` (`kde-breeze` by a rule in `context form.row`: kirigami-addons' swatch draws no border,
  K-KA-14†, where KColorButton's has one, K2.20).
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
  - `plane` *(element)* — the saturation and value field (Windows' box or ring spectrum), the focusable control with
    states of its own (GTK's `GtkColorPlane` takes the focus, `gtkcolorplane.c:352`, `:387`, GN-2.53†): `stroke`
    (W2-05), `focus` (GNOME's plane focus ring, GN-2.53†), `cursor` (crosshair, ADD-A59), `min_width`, `min_height`
    (GN-2.53†). `crosshair_line_horizontal`, `crosshair_line_vertical`, `crosshair_band_horizontal`,
    `crosshair_band_vertical` and `selection_ellipse` belong to `plane`, nest in it and are queried with it.
  - `crosshair_band_horizontal`, `crosshair_band_vertical` — GNOME's band under each line while the plane shows
    focus, drawn first (`gtkcolorplane.c:95-104`): `fill`, `min_width`, `min_height` (the band's thickness across
    its axis).
  - `crosshair_line_horizontal`, `crosshair_line_vertical` — the line through the chosen point along each axis
    (GNOME's lines, over the bands while the plane shows focus and alone otherwise, `gtkcolorplane.c:95-122`; KDE's
    cross, K2.25†): `fill`, `min_width`, `min_height` (its thickness across its axis, and its length along it
    where the lines do not span the plane, `values.crosshair_spans_plane`).
  - `selection_ellipse` — Windows' selection ellipse at the chosen point (W2-05): `stroke`, `corners`, `min_width`,
    `min_height`.
  - `value_strip` — KDE's value strip (K2.25†): `stroke`.
  - `checkerboard` — under translucent colours: `fill`.
  - `preview` — Windows' previous and current colours, GNOME's sample: `corners`.
  - `swatch` *(element: the chooser's swatches, KDE's colour wells)* — `corners` (GNOME's per-edge radii are
    `swatch.corners` under `pos` rules, GN-2.27), `stroke` (KDE's well frame, K2.25†), `focus`. `swatch_ring` belongs
    to `swatch` and is queried with it.
  - `swatch_ring` — GNOME's drop-target and high-contrast ring: `stroke`.
- **Widget values:** `grid_gap_px: Value<Px>` (GN-2.53†); `crosshair_spans_plane: Value<bool>` (`true`: each
  crosshair line and band runs across the whole plane through the point, GNOME's, `gtkcolorplane.c:101-122`;
  `false` where the lines are a cross of their own lengths centred on the point, KDE's, K2.25†, or where the
  platform draws no lines, `windows-11`).
- **Variants:** keyed `windows-11` `@box`, `@ring` (the spectrum's shape, W2-05); `kde-breeze` `@qt_dialog`
  (K2.25†).
- **Sizes and densities:** none.
- **Flags:** none of its own; state flags used: focus_visible on `plane` (GNOME's focus ring and its focused
  crosshair, drawn while `gtk_widget_has_visible_focus`, `gtkcolorplane.c:95`) and on `swatch`, drop_target (on
  `swatch`, GN-2.27), selected, hover, pressed, disabled.
- **Exclusive groups:** none (W2-05 states no visual-state groups for the spectrum; GNOME's and KDE's states
  combine); precedence: state models (S17).
- **may_be_absent:** parts: `crosshair_band_horizontal` (`kde-breeze`, `windows-11`: the bands are GTK's,
  GN-2.53†; `adwaita` by a rule under `!{focus_visible}`, its unfocused crosshair being the lines alone,
  `gtkcolorplane.c:112-122`), `crosshair_band_vertical` (`kde-breeze`, `windows-11`; `adwaita` by a rule under
  `!{focus_visible}`: as `crosshair_band_horizontal`), `crosshair_line_horizontal` (`windows-11`: its marker is the
  selection ellipse, W2-05), `crosshair_line_vertical` (`windows-11`: as `crosshair_line_horizontal`, W2-05),
  `selection_ellipse` (`adwaita`, `kde-breeze`: their markers are lines, GN-2.53†,
  K2.25†), `value_strip` (`adwaita`, `windows-11`: the value strip is QColorDialog's, K2.25†), `swatch`
  (`windows-11`: W2-05 names no palette of swatches, its preview swatch being `preview`), `swatch_ring` (`kde-breeze`,
  `windows-11`: the ring is GNOME's swatch ring, GN-2.27). A drawn plane always has a size, so every preset states the
  `plane.min_width` and the `plane.min_height` (measured where no source states them, S17).
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
- **may_be_absent:** parts: `inner_frame` (`macos-*`: the second frame line is Qt's, K2.31†); fields:
  `body.shadow` (`kde-breeze`: Qt draws its hatched shadow only when the platform's `DropShadow` hint is true, which
  the KDE platform theme does not set, K2.31†; Open items); values: `mode_cursor` (`macos-*`: which cursor AppKit
  shows is the measurement's, MAC-P4-11†; Open items).
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
- **Available:** every preset but `adwaita` (`available = false`: no date picker in GNOME's rows); `kde-breeze`
  in both flavours, each its own toolkit's picker: Qt Widgets' KDateComboBox and QDateEdit under `cond
  flavor:widgets` (K2.23†), kirigami-addons' date row and popup under `cond flavor:kirigami` (K-KA-20†, K-KA-22†).
- **Parts:**
  - `body` — the picker's box (the root), holding the trigger, or the inline calendar and wheels: `padding`,
    `bindings`.
  - `header` — the control's header label above the field, outside any popup (W2-03, W2-04): `text`.
  - `field` — the button, pill or combo showing the date (the picker's own trigger; absent under `inline`, which has
    none): `fill` (focused fill, W2-03; iOS's pill, C-5), `stroke` (W2-04), `corners` (iOS's pill, C-5), `padding`,
    `min_width`, `max_width` (W2-04). `kde-breeze` under `cond flavor:widgets` draws the trigger with another widget
    (Nested controls): under `with_popup` `field` is `drawn_as = { widget = "combo_box", variants = ["editable"] }`
    (KDateComboBox paints an
    editable combo box, `CC_ComboBox` with its line edit, K1.240, as KTimeComboBox: its constructor calls
    `setEditable(true)` whatever its options, and clearing `EditDate` only makes the line edit read-only,
    `kdatecombobox.cpp:214`, `:103`, kwidgetsaddons at `259b994c`, tag `v6.31.0`), under `text_field` `drawn_as = { widget = "stepper", variants = [] }` (QDateEdit's spin box,
    K2.23†), so the look is stated once, by those widgets' rules in `context date_picker`. Under `cond
    flavor:kirigami` the field is kirigami-addons' own flat button in `context form.row` (K-KA-22†), drawn as this
    part.
  - `field_label`, in `field` — the date text (accent on iOS, C-5): `text`.
  - `placeholder`, in `field` — the text without a date: `text`.
  - `glyph`, in `field` — Windows' calendar glyph (W2-03): `glyph` (per state).
  - `field_segment` *(element: a day, month or year part of a text-field date, selected while edited)*, in `field` —
    `fill`, `text`, `padding` (W2-04, MAC-N04; one structure with `time_picker.field_segment`).
  - `segment_separator`, in `field` — Windows' spacer lines, kirigami-addons' separator between the halves (K-KA-22†): `stroke`,
    `opacity`, `margin` (its inset from its ends, K-KA-22†: a line's insets are its `margin`, as a stroke lies on
    its box's edge, S2.4; one structure with `time_picker.segment_separator`).
  - `popup` — the picker's own flyout, popover or docked container, holding the calendar or the looping selectors
    (one name with `time_picker.popup`): `fill`, `stroke`, `corners` (MAT-N16), `shadow`, `padding`, `min_width`,
    `min_height` (MAT-N16), `transitions` (its open and close: where a field opens it, it is drawn only `{open}` and
    enters from transparent and leaves to transparent by these, S2.7; W2-04's open animation). `popup_header` and
    `divider` nest in it. `kde-breeze` draws it with another widget under `with_popup`, each flavour its own: under
    `cond flavor:widgets` `popup` is `drawn_as = { widget = "menu", variants = [] }` (KDateComboBox opens a menu,
    K2.23†), under `cond flavor:kirigami` `drawn_as = { widget = "dialog", variants = ["@ka_dialog"] }`
    (kirigami-addons' popup, K-KA-20†, whose fade is the dialog's), so one box is drawn.
  - `popup_header`, in `popup` — Material's header inside the picker's container, holding the headline and the
    supporting text (the modal and input pickers' `header-container-*` tokens, MAT-N16): `min_width`, `min_height`.
    `headline` and `supporting_text` nest in it.
  - `headline`, `supporting_text`, in `popup_header` — Material's: `text`.
  - `divider`, in `popup` — Material's: `stroke`.
- **Widget values:** `placement: Placement` (the popup against the field, its gap the placement's `gap`: W2-03's
  flyout, K2.23†'s menu, C-5's popover, MAC-N04's calendar overlay; strategies: value phase). The popup's showing and
  hiding are its `transitions` (above), not an event.
- **Variants:** typed `text_field` (a segmented text-field date: macOS textField and KDE QDateEdit, which open no
  popup; Windows DatePicker, whose field opens its looping-selector flyout, W2-04),
  `with_popup` (Windows CalendarDatePicker, KDE KDateComboBox and kirigami-addons' date row, iOS compact, Material
  docked), `inline` (macOS clockAndCalendar, iOS inline); keyed `macos-*` `@text_field_and_stepper` (MAC-N04),
  `material-3` `@modal`, `@input` (MAT-N16). On `kde-breeze` the flavour, not a variant, tells KDateComboBox from
  kirigami-addons' picker: both are `with_popup` (the field opens the popup), the substitutions are rules under
  `cond flavor:widgets` and `cond flavor:kirigami`, and an application asking for `with_popup` gets its flavour's
  picker with no keyed name of KDE's; `[declare.flavor.kirigami.widget.date_picker]` declares `with_popup` alone of
  the typed forms (`text_field` is QDateEdit's, K2.23†; kirigami-addons draws no text-field date, K-KA-22†).
- **Sizes and densities:** `macos-*`: `NSControl.ControlSize` as AppKit draws it per style (MAC-N04). `windows-11`: density `compact` (WinUI's compact sizing dictionary, W3-39).
- **Flags:** shared `empty` (Windows: no date chosen, the placeholder against the selected text colours, W2-03);
  state flags used: hover, pressed, focused, disabled (W2-03, W2-04), selected (on `field_segment`, MAC-N04), read_only
  (K-KA-22†), open.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (CommonStates); others none (states combine);
  precedence: state models (S17).
- **may_be_absent:** parts (the presets that declare `inline`, `macos-*` and `ios-*`, state the field's parts
  absent by a rule under it, as it has no field; `kde-breeze`'s rules that draw `field` as a combo box or a spin box,
  under `with_popup cond flavor:widgets` and `text_field`, state the field's own parts absent, the substitute's
  parts taking their place, Nesting): `field` (`macos-*`, `ios-*` by a rule under `inline`), `field_label`
  (`macos-*`: its field draws the date as segments, MAC-N04; `windows-11` by a rule under `text_field`:
  DatePicker's segments, W2-04; `ios-*` by a rule under `inline`; `kde-breeze` by a rule under `with_popup cond
  flavor:widgets` and by a rule under `text_field`), `placeholder` (`macos-*`, `ios-*`: the `empty` state that
  shows it is Windows' alone, W2-03; `kde-breeze` by a rule under `with_popup cond flavor:widgets` and by a rule
  under `text_field`), `glyph` (`macos-*`, `ios-*`, `material-3`: the calendar glyph is
  Windows' and kirigami-addons', W2-03, K-KA-22†; `kde-breeze` by a rule outside `context form.row`, where
  kirigami-addons' row draws it, K-KA-22†), `field_segment` (`ios-*`, `material-3`: their fields are not segmented,
  C-5, MAT-N16; `windows-11` by a rule outside `text_field`: CalendarDatePicker's field shows the date whole, W2-03;
  `macos-*` by a rule under `inline`; `kde-breeze` by a rule under `with_popup cond flavor:widgets` and by a rule
  under `text_field`, and by a rule under `cond flavor:kirigami`, kirigami-addons' field showing the date whole,
  K-KA-22†), `segment_separator` (`ios-*`, `material-3`: their fields are not segmented, C-5, MAT-N16; `macos-*`:
  MAC-N04's text field separates its segments by text; `windows-11` by a rule outside `text_field`: the spacer lines
  are DatePicker's, W2-04; `kde-breeze` by a rule under `with_popup cond flavor:widgets` and by a rule under
  `text_field`, and by a rule outside `context form.row`, where kirigami-addons' row draws it between its date and
  time halves, K-KA-22†), `popup` (`windows-11`, `macos-*`, `ios-*`, `material-3`, `kde-breeze` by a rule `!{open}`
  where the field opens it, under `with_popup` (on `kde-breeze` in both flavours) and Windows' and macOS's
  `text_field`, W2-04's flyout and MAC-N04's calendar overlay; `macos-*`, `ios-*` by a rule under `inline`, which
  opens none; `kde-breeze` by a rule under `text_field`, whose QDateEdit opens none, K2.23†), `header` (all but
  `windows-11`: W2-03's and W2-04's control header), `popup_header` (all but `material-3`: MAT-N16; `material-3` by
  a rule outside `@modal` and `@input`, the docked picker's tokens stating no headline header, and by a rule
  `!{open}`, as the `popup` it nests in, a part nested in an undrawn part being drawn unless absent itself, Nesting),
  `headline` (all but `material-3`: MAT-N16; `material-3` by the rules that make `popup_header` absent, the same
  selectors), `supporting_text` (all but `material-3`: MAT-N16; `material-3` as `headline`), `divider` (all but
  `material-3`: MAT-N16; `material-3` by a rule `!{open}`, as `popup_header`); fields: `popup.stroke` (`macos-*`,
  `ios-*`, `material-3`: their popups state a fill, corners and shadow and no outline, MAC-N04, C-5, MAT-N16; W2-04's
  flyout draws one). The `body.padding` is stated on every preset (the platform's inset, cited, zero where it is
  zero).
- **State-model source:** macOS `NSDatePicker.Style` × control states; Windows CalendarDatePicker and DatePicker
  visual state groups; KDE Breeze `CC_ComboBox` and menu branches (`kdatecombobox.cpp`), kirigami-addons
  `FormDateTimeDelegate.qml`, `DatePopup.qml`; Material date-picker token files; iOS `UIDatePickerStyle`.
- **Today's fields:** none (W2-03 and C-5 point at `combo_box.*`, which keeps its own).
- **Notes:** MAC-N04's calendar overlay (`presentsCalendarOverlay`) is a `calendar` in `context date_picker.popup`,
  placed by this widget's `placement`. W2-04's looping selectors are `picker_wheel` elements in `context
  date_picker.popup` (its band `picker_wheel.selection_band`, its item height `picker_wheel.item.min_height`); macOS's
  stepper is a `stepper` in context (MAC-N04); K2.23†'s popup is the `menu` element `popup` is drawn as, in `context
  date_picker`, holding the `calendar` and the word items (Today, Next Week …) as `menu.item` elements.
  kirigami-addons' date and time row (K-KA-22†) puts the field, drawn flat, in `context form.row` (the form's
  `@ka_form_card`); its popup (K-KA-20†) is the `dialog` element `popup` is drawn as, in `context date_picker`
  (`dialog @ variant @ka_dialog context date_picker`: its body, scrim and action area), holding the `calendar`.

### dial

*From group 6's catalogue.*

- **Kind:** typed widget: KDE's QDial and QQC2 Dial (K2.5, Q2.2), macOS's circular slider (MAC-N15).
- **Available:** every preset but `adwaita`, `windows-11`, `ios-*`, `material-3` (`available = false`: no dial in
  their rows).
- **Parts:**
  - `body` — the dial's box (the root): `padding`, `bindings`.
  - `groove` — the ring (K2.5), a band along the circle (Entry format): `fill`.
  - `value_arc` — the value (KDE's highlight arc), a band along the circle: `fill`.
  - `handle` *(element)* — the slider handle with its shadow (K2.5): `fill`, `stroke`, `corners`, `shadow`,
    `min_width`, `min_height`, `focus`.
  - `knob_dot` — macOS's knob dot (MAC-N15): `fill`, `min_width`, `min_height`.
  - `tick` — macOS's ticks: `fill`, `min_width`, `min_height`.
- **Widget values:** `stroke_width_px: Value<Px>` (the width of the groove and value arc, Breeze's
  `Slider_ControlThickness`, K2.5; the handle's size is `handle.min_width`); `cap: Value<LineCap>` (the value arc's
  ends); `start_angle_deg: Value<Deg>`, `end_angle_deg: Value<Deg>`; `default_size_px: Value<Px>` (Q2.2).
- **Variants:** none (a `dial` element on `macos-*` is always the circular slider, `NSSlider`'s circular type, so
  MAC-N15's look is that preset's base, not a variant).
- **Sizes and densities:** `macos-*`: the control sizes (MAC-N15).
- **Flags:** none of its own; state flags used: hover (on `handle`, K2.5), pressed, focus_visible, disabled.
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `value_arc` (`macos-*`: MAC-N15's dial, knob dot and ticks draw no value arc), `handle`
  (`macos-*`: its value is the knob dot, MAC-N15), `knob_dot` (`kde-breeze`: Breeze's dial has the slider handle, K2.5),
  `tick` (`kde-breeze`: the ticks are MAC-N15's, K2.5 naming none); values: `default_size_px` (`macos-*`: the default
  size is the QML Dial's, Q2.2), `cap` (`macos-*`, whose dial draws no value arc).
- **State-model source:** KDE Breeze `CC_Dial` branches (`st:8139-8212`) and the QML Dial; macOS `NSSlider`
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
  - `glyph` — Windows' drop glyph, one per operation by rules on the drop flags (copy, move, link; W3-49†): `glyph`.
  - `drop_badge` — iOS's drop-operation badge, one per operation by rules on the drop flags (P3-10†): `fill`,
    `glyph`.
  - `drop_description` — Windows' shell drop-description label (W3-49†): `text`.
- **Widget values:** `lift_delay_ms: Value<Ms>` (C-24); `hotspot: Value<DragHotspot>`, `DragHotspot { PressPoint
  }` (where the pointer holds the image: at the press point, K-RL06†, the one hotspot the rows state; a further one a
  fact states is added as a member); `presentations: Vec<PresentationTransition>`: events `@lift`, `@drop`, `@cancel`
  (C-24; MAT-P4-8†'s cancelled drag shrinks and fades), `@drop_not_consumed` (MAT-P4-8†: the shadow flies back
  while fading).
- **Variants:** none (the rows state none).
- **Sizes and densities:** none.
- **Flags:** shared `drop_copy`, `drop_move` (Windows' `DataPackageOperation`, which selects the `glyph`, W3-49†;
  iOS's `UIDropOperation`, which selects the `drop_badge`, P3-10†; the connector sets one at a time from the drag's
  current operation); keyed `windows-11` `@drop_link` (W3-49†), `ios-*` `@drop_forbidden`, `@drop_cancel` (P3-10†),
  set the same way; `material-3` `@source_hidden` (the drag hid its source task: both outcomes only fade in place,
  MAT-P4-8†); state flags used: none
  on the image itself; the source element takes `dragged` (K-RL06†, W1-110) and
  the target `drop_target` (MAC-X21, C-24, W3-49†).
- **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** parts: `lift_preview` (all but `ios-*`: iOS's lift, C-24), `caption` (all but `windows-11`:
  W3-49†'s caption), `glyph` (all but `windows-11`: W3-49†'s drop glyphs), `drop_description` (all but
  `windows-11`: W3-49†'s shell label), `drop_badge` (all but `ios-*`: iOS's drop badges, P3-10†; Material draws
  none, MAT-P4-8†); fields: `image.fill` (all but `windows-11`: W1-110's drag fill), `image.shadow` (the value phase
  names the presets from the facts; MAC-X01 draws one); values: `lift_delay_ms` (all but `ios-*`: C-24), `hotspot`
  (the value phase names the presets from the facts, the platforms that position the drag image by no hotspot;
  K-RL06† and W1-110 state one).
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
- **may_be_absent:** parts: `arrow` (`kde-breeze`, `material-3`, `windows-11`: the arrow towards the selection is
  iOS's and GNOME's, C-14, GN-1.286†), `primary_commands` (all but `windows-11`: W2-23's primary row), `divider`
  (`windows-11`, `material-3`: no line between commands). The `presentations` list is empty, not absent, where the
  platform has none.
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
  choosers are the system's, out by R1.1, as is GNOME's portal chooser; Windows' Common Item Dialog is another
  toolkit's, drawn in the application's process, R1.3, W-P4-8†);
  `kde-breeze` flavour `widgets` only (KFileWidget).
- **Parts:**
  - `body` — the chooser (the root): `padding`, `bindings`
  - `frame` — GNOME's line below the chooser, outside its box (GN-P3-23†): `stroke`
  - `preview_pane` — KDE's preview (K2.26†): `fill`
- **Widget values:** `icon_size_slider: Value<bool>` (KDE: a `slider` in context, K2.26†). No row states a gap of the
  chooser's own (GN-2.48, GN-P3-22†, GN-P3-23†, K2.26†), so it has no gap value.
- **Variants:** none (its rows' variants are the `sidebar`'s and `list`'s in context).
- **Sizes and densities:** none.
- **Flags:** none (its elements are other widgets', each with its own states).
- **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** parts: `preview_pane` (`adwaita`: the preview is KFileWidget's, K2.26†), `frame`
  (`kde-breeze`: the line below the chooser is GTK's, GN-P3-23†). The `icon_size_slider` is a `Value<bool>`, stated
  on every preset (`false` where the chooser has no such slider).
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
- **Flags:** a search that finds nothing is the state flag `invalid` (S7.2: the find bar's pattern matches nothing,
  set by the application from its search on every platform; KDE's `NegativeBackground` field and "Not found" label,
  K2.30†); keyed `kde-breeze` `@match`, `@neutral` (a match, also after wrapping; a neutral result, KColorScheme's
  `PositiveBackground` and `NeutralBackground`, K2.30†), which the application sets from its search outcome (a
  platform's own look of an outcome other platforms do not draw; on a preset that does not declare them they select
  nothing); state flags used: invalid (its buttons are `button` elements).
- **Exclusive groups:** none (states combine; the application sets one search outcome at a time); precedence:
  state models (S17).
- **may_be_absent:** parts: `result_count` (`kde-breeze`: the match count is C-25's; K2.30† reports by colour and
  its status label), `status_label` (`ios-*`, `macos-*`: the "Not found" label is KTextEditor's, K2.30†); values:
  `bar_position` (`kde-breeze`: the positions are iOS's and macOS's, C-25, MAC-N18, K2.30† naming no position).
- **State-model source:** iOS `UIFindInteraction`; KDE `katesearchbar.cpp`; macOS `NSTextFinder`.
- **Today's fields:** none.
- **Notes:** Nested controls: the pattern field is an `input` in `context find_bar`, KDE's result colours its rules
  under the bar's flags (`input @ context find_bar {@match}` and so on, K2.30†); the previous and next buttons are
  `button @ variant previous / next context find_bar`, iOS's options button `button @ variant menu context
  find_bar` (C-25), macOS's Done `button @ variant done context
  find_bar` (MAC-N18). The highlights in the text are the searched view's `find_match` elements: `text_area.find_match`
  (C-25, MAC-N18; the current match `{current}`), `code_editor.find_match` on `kde-breeze` (K-SF08†), with the global
  `role.global.find_highlight`.

### font_chooser

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's GtkFontDialogButton (GN-2.28) and GtkFontChooserWidget (GN-P3-20†), KDE's
  KFontRequester and KFontChooser (K2.28†).
- **Available:** every preset but `macos-*`, `windows-11`, `ios-*`, `material-3` (`available = false`: no font
  chooser the toolkit draws; Win32's `ChooseFont` is another toolkit's, R1.3, W-P4-7†; macOS's font panel is MAC-N25's row); `kde-breeze`
  flavour `widgets` only (KFontRequester, KFontChooser).
- **Parts:**
  - `body` — the chooser (the root): `padding`, `bindings`
  - `requester` *(element: GNOME's font dialog button)* — GtkFontDialogButton's `.font` button node, with its states
    and focus indicator (GN-2.28; `gtkfontdialogbutton.c:126-152`): `fill`, `stroke`, `corners`, `padding`,
    `focus`. `requester_label` and `requester_size_label` belong to it and are queried with it.
  - `requester_label` — the font's name (`gtkfontdialogbutton.c:128-129`; in the chosen font where the button uses
    it): `text`
  - `requester_size_label` — the font's size beside the name, in its own text style (`_buttons.scss:309`,
    GN-2.28): `text`
  - `font_label` — KDE's sunken styled-panel label showing the font's name in that font, its frame forced on Breeze
    (K2.28†): `fill`, `stroke`, `corners`, `padding`, `text`
  - `family_row_label` — a family row's name, drawn in its own face (GN-P3-20†): `text`
  - `preview` — the preview text (K2.28†, GN-P3-20†): `text`
  - `filter_title` — the "Filter by" title (GN-P3-20†): `text`
- **Widget values:** `grid_gap_px: Value<Px>` (GN-P3-20†); `row_margins: Sides<Value<Px>>` (GN-P3-20†);
  `preview_min_lines: Value<u32>` (K2.28†); `requester_gap_px: Value<Px>` (the spacing between the requester's
  name and size, GtkFontDialogButton's, GN-2.28).
- **Variants:** none (KDE's requester label is always the sunken panel, K2.28†).
- **Sizes and densities:** none.
- **Flags:** none of its own; state flags used: hover, pressed, focus_visible, disabled (on `requester`).
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `requester` (`kde-breeze`: KFontRequester is a label beside a push button, K2.28†, its
  label `font_label` and its button a `button` element), `requester_label` (`kde-breeze`: as `requester`, K2.28†),
  `requester_size_label` (`kde-breeze`: as `requester`, K2.28†), `font_label`
  (`adwaita`: GtkFontDialogButton shows the name inside its button, GN-2.28), `filter_title` (`kde-breeze`: the
  "Filter by" title is GtkFontChooserWidget's, GN-P3-20†); values: `preview_min_lines` (`adwaita`: KFontChooser's
  preview height, K2.28†), `row_margins` (`kde-breeze`: GtkFontChooserWidget's rows, GN-P3-20†),
  `requester_gap_px` (`kde-breeze`: the gap inside GtkFontDialogButton, GN-2.28).
- **State-model source:** GNOME `_buttons.scss` (the `button` node with class `.font`), `gtkfontchooserwidget.ui`;
  KDE `kfontrequester.cpp`, `kfontchooser.cpp`.
- **Today's fields:** none.
- **Notes:** the family, style and size lists are `list` elements, the size controls `slider` and `stepper`, the
  search entry `search_field`, the filters button a `button` (variant `menu`) with a `popover`, all in `context
  font_chooser` (GN-P3-20†, K2.28†). Nested controls: KDE's requester button, with its `document-edit` icon, is
  `button @ context font_chooser` (K2.28†). GNOME's GtkFontDialogButton, though a GtkButton, stays the host's
  `requester` element: its content, the name and beside it the size in its own text style with the gap between
  them (GN-2.28's spacing and inner label), has no counterpart among `button`'s parts, whose `subtitle` is a line
  under the title. GTK hides the button's separator between the name and the size (`_buttons.scss:307`, GN-2.28), so
  no part carries it.

### form

*From group 6's catalogue.*

- **Kind:** typed widget: preference pages, groups and their rows: GNOME's preference rows, groups and pages
  (GN-2.01–GN-2.10), KDE's FormLayout (Q2.45–Q2.47), Kirigami Forms (Q2.55†, Q2.56†) and kirigami-addons' form
  cards (K-KA-1†–K-KA-9†, K-KA-15†), Qt Widgets' form layout (`QFormLayout` as Breeze styles it, K1.210), macOS's
  SwiftUI grouped form (MAC-N21) and columns form (MAC-LY02†), Windows' Toolkit SettingsCard and
  SettingsExpander (W2-40†) and the clickable card (W1-139). GNOME's preference rows are this widget's in one place:
  an AdwPreferencesGroup's boxed list is `card` and its rows `row`, which state the boxed list's frame, row hover and
  pressed fills, outer corners, focus ring and separators (GN-1.165) on these parts; a plain `.boxed-list` that is not
  a preferences group is `list @ variant inset`, which states the same source on its own parts (R20: each widget its
  own parts). A row's switch, entry, combo and buttons are those widgets' elements in context. It is the one home
  of a label–field form: Qt's form layout and SwiftUI's columns form are this widget, not a `layout` variant.
- **Available:** every preset but `ios-*`, `material-3` (`available = false`: no form in their rows); `kde-breeze`
  in both flavours: Qt Widgets' `QFormLayout` under `cond flavor:widgets` (K1.210: labels and fields, no cards,
  titles or separators, so the parts it does not draw are absent by rules under that condition, below), Kirigami's
  FormLayout and Forms and kirigami-addons' form cards under `cond flavor:kirigami`. Both macOS presets declare it:
  SwiftUI's grouped form is macOS 13's (MAC-N21),
  `macos-monterey` describes macOS 15, its generation's newest release (R1.5), and `macos-tahoe` macOS 26.
- **Parts:**
  - `page` — the form's page (the root; kirigami-addons' page fill, K-KA-6†): `fill`, `padding` (the page's margins,
    GN-2.10), `bindings`
  - `group_title` — a group's title (Q2.55†; kirigami-addons' title in the View set with its own insets, K-KA-5†;
    GNOME's single-line header height, GN-2.10): `text`, `padding`, `min_height`, `bindings`
  - `group_description` — the explanatory text between groups (kirigami-addons' `FormSectionText`, K-KA-5†): `text`,
    `max_width`, `margin` (its margins above, below and at the sides, K-KA-5†)
  - `section_title` — a section title (Q2.47): `text`
  - `field_label` — a field's label (Q2.47; kirigami-addons' label above the field, K-KA-9†): `text`, `padding`
    (K-KA-22†)
  - `card` *(element)* — the boxed group's background, one element per card, hovered and pressed on its own
    (GNOME's preferences group boxed list, GN-1.165; Kirigami's cards, kirigami-addons' card with its border and
    shadow in the View set, Q1.129, K-KA-1†, SwiftUI's section card, Windows' settings card: fill, stroke, padding
    and min sizes per state, W2-40†, W1-139; the outer corners rounded by position rules): `fill`, `stroke`,
    `corners`, `shadow`, `padding`, `min_width`, `min_height`, `bindings`. The rows and row separators of a card nest
    in it and are queried in its scope.
  - `row` *(element)*, in `card` (on every preset: where the form draws no card the rows are queried in the page's
    scope, Nesting) — a row (GN-2.01, GN-2.08; Q2.55†,
    K-KA-3†; the expander items' top line, W2-40†): `fill`,
    `stroke`, `corners`, `padding`, `min_width`, `min_height`, `focus`, `bindings` (AdwEntryRow's `.error`, `.warning`
    and `.success` rebinding the accent to the status colour, GN-2.03), `transitions` (the hover and pressed fills
    and their fade, Q2.55†, K-KA-2†, GN-1.165; the first and last rows' outer corners; GNOME's focus ring, GN-2.03,
    GN-1.165). `row_title_box`, `row_title`, `row_subtitle`, `row_prefix`, `row_suffix`, `row_icon`, `row_edit_icon`,
    `row_indicator`, `row_arrow`, `row_nested` belong to `row`, nest in it and are queried with it.
  - `row_title_box` — the box holding the row's title and subtitle (AdwActionRow's `box.title`, GN-2.01): `margin`
    (its margins; a preset whose row has no such box states them absent).
  - `row_title`, in `row_title_box` — the row's title (wrapped, then elided, K-KA-3†; dimmed in property rows,
    GN-2.02): `text`,
    `opacity`, `transitions` (AdwEntryRow's title shrinking to a caption, GN-2.04)
  - `row_subtitle`, in `row_title_box` — the row's subtitle, dimmed (GN-2.01, Q2.55†, K-KA-15†): `text`, `opacity`,
    `padding` (its inset,
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
    `icon`, `glyph` (an arrow drawn as a glyph: GNOME's expander arrow, stated with the same field kind as
    `list.disclosure.glyph`, GN-1.257), `padding`, `opacity`, `transform`, `transitions`
  - `row_nested` — the box holding an expander row's nested rows (GNOME's `list.nested` fill, GN-1.260; Windows'
    SettingsExpander items, W2-40†): `fill`, `padding`
  - `row_separator` *(element)*, in `card` — the line between two rows, an element whose previous and next siblings
    are rows (GN-1.165; Kirigami's separator, dimmed, Q2.56†, K-KA-4†): `stroke` (the line, its top edge), `opacity`,
    `margin` (its insets from the card's sides, Q2.56†, K-KA-4†: a line's insets are its `margin`, as a stroke lies
    on its box's edge, S2.4), `transitions` (Kirigami's separator hidden beside a hovered entry and its insets
    animating, Q2.56†, K-KA-4†)
- **Widget values:** `group_gap_px: Value<Px>`; `page_gap_px: Value<Px>`;
  `row_gap_px: Value<Px>` (between the rows, Q2.45, Q2.56†, K-KA-4†); `column_gap_px: Value<Px>` (between a label
  and its field, GN-2.10, Q2.45); `title_subtitle_gap_px: Value<Px>` (between a row's title and subtitle, GN-2.01);
  `affix_gap_px: Value<Px>` (between the items inside a row's prefix or suffix box, GN-2.01); `label_alignment:
  Axes<Value<Alignment>>` (the label against its field, horizontally and vertically, Q2.45, Q2.46, Q2.47, restated
  under `collapsed`; Qt's form layout's labels, `SH_FormLayoutLabelAlignment`, K1.210, `breezestyle.cpp:950-951`;
  SwiftUI's columns form's trailing-aligned label column, MAC-LY02†, under `columns`); `baseline_alignment:
  Value<bool>` (a label and its control on one baseline, MAC-LY02†; `false` where the platform does not do it);
  `field_growth: Value<FieldGrowth>`, `FieldGrowth { StayAtSizeHint, ExpandingFieldsGrow, AllNonFixedFieldsGrow }`
  (which fields grow to fill their column, Qt's `QFormLayout::FieldGrowthPolicy`, `SH_FormLayoutFieldGrowthPolicy`,
  K1.210, `breezestyle.cpp:952-953`); `contents_alignment: Axes<Value<Alignment>>` (where the form's rows sit in
  the form's own space, Qt's `SH_FormLayoutFormAlignment`, K1.210, `breezestyle.cpp:948-949`);
  `narrow_max_width_px: Value<Px>` (the collapsed form's width cap, Q2.45, Q2.56†; `auto` where a form that
  collapses sets no cap); `collapse: Value<FormCollapse>`,
  `FormCollapse { WhenWideDoesNotFit, AtBreakpoints }` (how the connector sets `collapsed`: KDE when the wide layout
  does not fit, Q2.45, Q2.56†, a measurement of the content with no threshold length, so it is no `Breakpoint`;
  Windows' wrap thresholds, W2-40†, by `breakpoints`; Qt's form layout never wraps its rows under Breeze,
  `SH_FormLayoutWrapPolicy` `DontWrapRows`, K1.210, `breezestyle.cpp:954-955`, so `collapse` is absent under `cond
  flavor:widgets`);
  `breakpoints: Vec<Breakpoint>` (Shared value types: Windows' SettingsCard wrap thresholds, W2-40†, measured
  `Allocation` (the card's stretched root grid), width `Below` each ("less than", `SettingsCard.xaml:312-333`): the
  wrap threshold setting `collapsed` (RightWrapped), the no-icon threshold setting `@right_wrapped_no_icon`, whose
  rules come after `collapsed`'s and win in the lower band, both under the `@right` layout; kirigami-addons' card
  width limit setting
  `@width_limited`, K-KA-1†: measured `Allocation` (the card's width, which fills its column), width `Above` the
  limit, `FormCard.qml:97`'s "wider than");
  `icon_text_gap_px: Value<Px>` (K-KA-3†); `leading_gap_px: Value<Px>`,
  `trailing_gap_px: Value<Px>` (between the row's leading items and its title, and its title and its trailing items:
  K-KA-3†, K-KA-8†; AdwActionRow's header spacing between its prefix box, title box and suffix box, GN-2.01);
  `arrow_gap_px: Value<Px>`
  (K-KA-7†); `field_label_gap_px: Value<Px>` (K-KA-9†); `grid_columns: Value<u32>`,
  `grid_min_cell_width_px: Value<Px>`, `grid_gap_px: Value<Px>` (K-KA-15†);
  `presentations: Vec<PresentationTransition>` (queried with the row): events `@expand`, `@collapse` (an expander
  row's height and opacity, K-KA-7†).
- **Variants:** typed row kinds `action`, `entry`, `combo`, `switch`, `button`, `expander`, `spin`, `property`
  (GN-2.01–GN-2.10, Q2.56†, K-KA-3†); typed `columns` (a column of labels beside a column of fields: SwiftUI's
  `.formStyle(.columns)`, MAC-LY02†, against macOS's grouped base; Qt's form layout, K1.210, and Kirigami's
  FormLayout, Q2.45, which are `kde-breeze`'s base under each flavour, so it selects no rule there; declared by
  `macos-*` and `kde-breeze`); typed `primary`, `destructive` (button rows, GN-2.08); typed `check`, `radio`
  (rows holding a check box or radio, K-KA-3†, K-KA-8†: `menu.item`'s and `list`'s meaning); typed `warning`,
  `error`, `success` (GNOME's entry rows' `.warning`, `.error`, `.success`; KDE's neutral highlight is `warning`);
  keyed `adwaita` `@monospace` (property and entry rows); typed `flat` (Kirigami Forms' flat style, no card: Q2.56†;
  the meaning `button`'s `flat` shares); typed `collapsed` (the narrow, stacked layout: KDE, Q2.45, Q2.56†; Windows'
  wrapped layout, RightWrapped, W2-40†; set by the connector as `collapse` says, R16: on `windows-11` by
  `breakpoints`, on `kde-breeze` from the connector's own measurement of the content when the wide layout does not
  fit, no breakpoint and no request of the application's, so it is a connector-set variant, as the content variants
  are, Entry format, ignored where undeclared, S8); keyed
  `kde-breeze` `@cards` (Kirigami Forms' card style, Q2.55†), `@ka_form_card` (on the form: a
  kirigami-addons FormCard page, whose rows and nested controls match `context form variant @ka_form_card`,
  K-KA-1†–K-KA-9†), `@width_limited` (set by `breakpoints`, R16; K-KA-1†), `@ka_info_card`
  (K-KA-15†), `@date_time` (K-KA-22†), `@icon_row` (K-KA-14†), `@link` (K-KA-7†); `windows-11` `@right`,
  `@right_wrapped_no_icon` (set by `breakpoints`), `@left` (the content
  layouts, W2-40†; RightWrapped is `collapsed`), and its vertical layout the typed `vertical` (orientation); the content variant `bitmap_icon`
  (on `row`: its icon is a bitmap, which WinUI dims when disabled where it recolours a glyph icon, W2-40†, W3-51†;
  set by the connector from the content, ADD-A61); the content variant `activatable` (on `row`: the row has an
  activation action, from which the connector sets it on every platform, ADD-A61: Kirigami's hover and pressed fills
  only on clickable entries, Q2.55†; AdwActionRow's `.activatable`; the meaning `card`'s `activatable` shares).
- **Sizes and densities:** `macos-*`: the control sizes (`mini`, `small`, `regular`, `large`): the columns form's
  label gap, `column_gap_px` under `columns`, is the label-to-control gap per control size (MAC-LY02†, MAC-LY01).
- **Flags:** shared `empty` (GNOME: AdwEntryRow's title not shrunk while the row is empty, GN-2.04); state flags
  used: hover, pressed, disabled (on `card` and `row`: W2-40†, W1-139, Q2.55†, K-KA-2†), focus_visible from K-KA-2†,
  focus_within (GN-2.04), open (expander rows, K-KA-7†). Position rules (`pos first`, `pos last`) on `row` round the
  card's outer corners; on the `row_separator` element, `prev form.row {hover}` and `next form.row {hover}` hide the separators
  beside a hovered neighbour (Q2.56†, K-KA-4†).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (the settings card's CommonStates, W2-40†);
  others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts (`kde-breeze` under `cond flavor:widgets` draws Qt's form layout, labels beside fields,
  so its rules there state absent every part `QFormLayout` does not draw, K1.210): `group_title` (`windows-11`: the
  Toolkit's SettingsCard and SettingsExpander, W2-40†, have no group heading; `kde-breeze` by a rule under `cond
  flavor:widgets`), `group_description` (`windows-11`: W2-40†'s cards have no text between groups; `kde-breeze` by
  a rule under `cond flavor:widgets`), `section_title` (`adwaita`, `macos-*`, `windows-11`: the section title is
  Kirigami's FormLayout's, Q2.47, Q2.56†; `kde-breeze` by a rule under `cond flavor:widgets`), `field_label`
  (`adwaita`, `windows-11`: the label beside or above a field is KDE's and SwiftUI's columns form's, Q2.47,
  K-KA-9†, K1.210, MAC-LY02†; `macos-*` by a rule outside `columns`), `card` (`kde-breeze` by a rule outside
  `@cards` and `@ka_form_card`, and by a rule under `flat`: Qt's form layout, K1.210, Kirigami's plain FormLayout,
  Q2.45–Q2.47, and its flat Forms draw no card; `macos-*` by a rule under `columns` where the value phase's
  measurement shows its sections draw none, MAC-LY02†; the rows are then queried in the page's scope, Nesting),
  `row_edit_icon` (all but `adwaita`, `kde-breeze`: AdwEntryRow's and kirigami-addons' mobile rows' edit icon,
  GN-2.03, K-KA-9†; `kde-breeze` by a rule under `cond flavor:widgets`), `row_indicator` (all but `adwaita`:
  AdwEntryRow's indicator, GN-2.03), `row_nested` (every preset by a rule outside `expander`), `row_arrow` (every
  preset by a rule outside the row kinds that show it, `action`, `combo`, `expander`; `kde-breeze` by a rule under
  `@ka_form_card` while the row draws no background, K-KA-7†), `row_separator` (`windows-11`: the expander items'
  top line is `row.stroke`, W2-40†; `kde-breeze` by a rule outside `@cards` and `@ka_form_card`, and by a rule
  under `flat`, whose separators are empty gaps and whose groups' line is a `separator`, Q2.56†, Qt's form layout
  drawing none either, K1.210; `macos-*` by a rule under `columns`, as `card`); fields: `row_suffix.icon`
  (`adwaita`, `macos-*`, `windows-11`; `kde-breeze` by a rule outside `@icon_row`, the chosen icon an icon row
  shows, K-KA-14†), `page.fill` (all but `kde-breeze`: kirigami-addons' page fill, K-KA-6†; `kde-breeze` by a rule
  under `cond flavor:widgets`), `row.focus` (`kde-breeze` under `@ka_form_card`: K-KA-2† draws no focus ring),
  `row_arrow.icon` (`adwaita` by a rule under `expander`, whose arrow is a glyph, GN-1.257), `row_arrow.glyph`
  (`kde-breeze`, `windows-11`: their arrows are icons, K-KA-7†, Q2.56†, W2-40†; `adwaita` by a rule outside
  `expander`, its other arrows being icons, GN-2.06), and the `row_separator.margin` is stated wherever the
  separator is drawn (its insets, MAC-N21, Q2.56†, K-KA-4†, zero where the line spans the card); values:
  `label_alignment` (`adwaita`, `windows-11`, each member: the label against its field is KDE's and SwiftUI's
  columns form's, Q2.45–Q2.47, K1.210, MAC-LY02†, as `field_label`; `macos-*` by a rule outside `columns`, and its
  `vertical` member under `columns` too, the label sitting on its control's baseline, `baseline_alignment`,
  MAC-LY02†), `field_growth` (all but `kde-breeze`: Qt's `FieldGrowthPolicy`, K1.210, unless the value phase finds
  one in another platform's facts; `kde-breeze` by a rule under `cond flavor:kirigami` unless the value phase finds
  one in Kirigami's facts), `contents_alignment` (all but `kde-breeze`, each member: Qt's
  `SH_FormLayoutFormAlignment`, K1.210, unless the value phase finds one in another platform's facts; `kde-breeze` by
  a rule under `cond flavor:kirigami` unless the value phase finds one in
  Kirigami's facts), `icon_text_gap_px` (all
  but `kde-breeze`: K-KA-3†), `arrow_gap_px` (all but `kde-breeze`: K-KA-7†), `field_label_gap_px` (all but
  `kde-breeze`: K-KA-9†, Q2.47), `grid_columns` (all but `kde-breeze`: K-KA-15†), `grid_min_cell_width_px` (all but
  `kde-breeze`: K-KA-15†), `grid_gap_px` (all but `kde-breeze`: K-KA-15†), `narrow_max_width_px` (`adwaita`,
  `macos-*`: their forms never collapse, so there is no collapsed width to cap; `kde-breeze` by a rule under `cond
  flavor:widgets`, whose rows never wrap, K1.210; `windows-11`'s wrapped layout states its cap from W2-40†'s facts,
  `auto` where they show none, as the value says), `leading_gap_px` (all but `kde-breeze`, `adwaita`: K-KA-3†,
  GN-2.01), `trailing_gap_px` (all but `kde-breeze`, `adwaita`: K-KA-3†, GN-2.01), `title_subtitle_gap_px` (all but
  `adwaita`: GN-2.01, unless the value phase finds it in another platform's facts), `affix_gap_px` (all but
  `adwaita`: GN-2.01, unless the value phase finds it in another platform's facts), `collapse` (all but
  `kde-breeze`, `windows-11`: Q2.45, Q2.56†, W2-40†, unless the value phase finds a collapse in another platform's
  facts; `kde-breeze` by a rule under `cond flavor:widgets`: Qt's `DontWrapRows`, K1.210). The `baseline_alignment`
  is a `Value<bool>`, stated on every preset. The `presentations` list is empty, not absent, where the platform has
  none, and so is the `breakpoints` list.
- **State-model source:** GNOME `_lists.scss`, `_preferences.scss`, `adw-entry-row.c`; KDE `FormLayout.qml`,
  `forms/cards/*.qml`, `forms/flat/*.qml`, kirigami-addons `formcard/*.qml`, and Qt's `QFormLayout` (no states of
  its own; Breeze's form-layout hints, `breezestyle.cpp:953-960`); macOS SwiftUI `GroupedFormStyle`,
  `ColumnsFormStyle`; Windows SettingsCard and SettingsExpander states.
- **Today's fields:** none (`list.*`, today's `expander` field `header_height` and `layout.*` stand-ins stay
  theirs).
- **Notes:** Windows' SettingsCard has one home, `form.card` (W2-40† and W1-139's clickable card), each card its
  own element (its rows' title colour per state is a rule `@ context form.card {hover}`); `card` variant
  `activatable` keeps GNOME's, Kirigami's and Material's activatable cards. kirigami-addons' rows hold `checkbox`,
  `radio`, `switch`, `combo_box`, `input`, `color_button`, `date_picker` elements in `context form.row`
  (`@ka_form_card`, K-KA-8†, K-KA-12†, K-KA-13†, K-KA-14†, K-KA-22†); their status messages are `banner`
  (`@ka_inline_message`, K-KA-10†). Group 6's `title_shrink` and `separator_hide` values are the `row_title` and
  `row_separator` transitions (GN-2.04, Q2.56†); `has_text` is `!{empty}`. Qt's form layout (K1.210) and SwiftUI's
  columns form (MAC-LY02†) were `layout`'s `form` variant and `form_*` values, moved here so that a label–field form
  has one home: its labels are `field_label`, its fields the nested widgets in `context form.row`, and
  `form_label_alignment`, `form_baseline_alignment`, `form_field_growth`, `form_alignment` are `label_alignment`
  (its `horizontal` member), `baseline_alignment`, `field_growth`, `contents_alignment`, Qt's row wrapping
  (`form_wraps`) being `collapse`.

### glass_container

*From group 7's catalogue.*

- **Kind:** typed widget: iOS's `UIGlassContainerEffect` (C-20, X-MT-8, X-MT-9) and macOS's
  `NSGlassEffectContainerView` (MAC-X04): nearby glass elements merging into one. One `glass_container` element is
  one glass view (a view with `UIGlassEffect`, an `NSGlassEffectView`), with its own material, corners and press
  response; the platform's container (`UIGlassContainerEffect`, `NSGlassEffectContainerView`) draws nothing of its
  own, so it is no element: the glass elements it holds are siblings, and two of them merge where they lie within
  `merge_gap` of each other (C-20, X-MT-9, MAC-X04). Owned jointly with group 8 (MAC-X04).
- **Available:** `macos-tahoe`, `ios-26` only (every other preset `available = false`: Liquid Glass is new in 26).
- **Parts:**
  - `surface` — the root, the one glass view: a `Material` paint with identity `glass` (tint, X-MT-6;
    interactivity a material parameter), its corners (MAC-X04) and the press response's scale (X-MT-8): `fill`,
    `corners`, `transform`,
    `bindings`, `transitions` (the interactive glass's press response and the morph between merged elements,
    X-MT-8, X-MT-9), `shadow` (macOS 26's glass shadow, MAC-X01)
- **Widget values:** `merge_gap_px: Value<Px>` (the distance within which sibling glass elements, each a
  `glass_container` element in the platform's container, merge: C-20, X-MT-9, MAC-X04; the merge's morph is the
  root's `transitions`).
- **Variants:** typed `clear` (C-20, X-MT-5, MAC-X04; the regular glass is the base).
- **Sizes and densities:** none.
- **Flags:** none of its own; state flags used: hover, pressed (the interactive glass's response, X-MT-8),
  over_dark_content (the clear glass's dimming layer, appended where the content beneath is bright, `!{over_dark_content}`,
  X-MT-10, X-SC-1).
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
    surface, C-32; Breeze rings only a checkable title's checkbox, a `checkbox`, K3.20): `fill`, `stroke`, `corners`,
    `padding`
  - `title` — the title (GN-2.32, K2.8, Q2.13, MAC-BX01): `text`
  - `focus_line` — KDE's faded line under a focused title (K2.8): `stroke` (the line, its top edge), `transitions`
    (its fade, K2.8)
- **Widget values:** `title_margin_px: Value<Px>` (GN-2.32, K1.211, MAC-BX01's title-to-border gap);
  `title_position: Value<GroupBoxTitlePosition>`, `GroupBoxTitlePosition { AboveTop, AtTop, BelowTop,
  AboveBottom, AtBottom, BelowBottom }` (MAC-BX01; KDE's title above the content, Q2.13; GtkFrame's label and
  SwiftUI's GroupBox label each sit at one fixed place, which `adwaita` and `ios-*` state, measured where no
  document says it, GN-2.32, C-32). A box with no title (`NSBox`'s `noTitle`) is content: the connector queries no
  `title` for it, so no position names it.
- **Variants:** typed `flat` (KDE's flat QGroupBox, K2.8; GNOME's `.flat`, GN-2.32); keyed `kde-breeze` `@box`,
  `@panel`, `@win_panel`, `@styled_panel` (K2.21), `@quick_frame` (Q2.12), `@quick_group_box` (Q2.13); `macos-*`
  `@primary`, `@custom`, `@transparent` (MAC-BX02; `NSBox`'s separator type is a line to the user, the
  `separator` widget, MAC-BX02, MAC-SE01).
- **Sizes and densities:** none.
- **Flags:** none of its own; state flags used: hover, focused (K2.21's framed panel outlines), focus_visible
  (K2.8's focus line), disabled.
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `focus_line` (all but `kde-breeze`: K2.8's line is Breeze's alone); fields:
  `body.stroke` (`ios-*`: SwiftUI's GroupBox is a filled surface, C-32; `kde-breeze` by a rule under `variant flat`,
  K2.8), `body.fill` (`adwaita`: GtkFrame draws a border only, GN-2.32; `kde-breeze` by a rule under
  `variant @quick_frame`, the transparent QQC2 Frame, Q2.12); values: none (every preset that declares the widget
  states its title's position, above).
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
- **may_be_absent:** parts: none; fields: `body.fill` (`adwaita`: GtkImage and GtkPicture draw no frame, GN-3.07;
  `macos-*` in the base, `NSImageFrameStyle`'s no frame, MAC-N12), `body.stroke` (`adwaita`, as for `body.fill`;
  `macos-*` in the base, MAC-N12), `body.shadow` (`adwaita`, as for `body.fill`; `macos-*` in the base, MAC-N12),
  `picture.shadow` (`macos-*`: its frame styles' shadows are the body's, MAC-N12; `adwaita` by its base, outside
  `variant @icon_dropshadow` and `variant @lowres_icon`, GN-3.07); values: none.
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
- **may_be_absent:** parts: `overlay` (`macos-*` where its facts show none, value phase: MAC-P4-12† names the
  crop outline and the mask only), `outline` (`windows-11` where its facts show none, value phase: W-RV-1† names a
  frame, a mask, an overlay path and thumbs, no outline); fields: none (the frame's padding on `macos-*` is stated
  as measured, a zero inset where ImageKit's cropper has none, never `Absent`).
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
- **Widget values:** `palette_placement: Value<Placement>` (the pen colour flyout, `WinUiFlyout`, W2-36).
- **Variants:** none (the rows state none).
- **Sizes and densities:** none.
- **Flags:** none of its own; its tools are `button` elements (below), the selected tool `{selected}` on both
  platforms (C-40†, W2-36: InkToolbar's checked tool maps to it, S7.2).
- **Exclusive groups:** none (W2-36 states no visual-state groups; iOS's states combine); precedence: state models
  (S17).
- **may_be_absent:** parts: none (the rows name each part for both platforms; Open items); fields: none; values:
  `palette_placement` (`ios-*`: PencilKit's tool picker lays out its own palette, C-40†).
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
- **may_be_absent:** parts: `separator` (`adwaita`: GN-2.40 names no text between the caps, only their spacing);
  fields: `label.focus` (`kde-breeze`: KCommandBar paints its keycaps in a row of its action tree, which K-RL01†
  gives no focus indicator of their own).
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
  - `value`, in `track` — the value's extent; under `circular` the value arc (iOS's fill, C-30; RadialGauge's trail
    arc, W2-41†) (GN-2.26, C-30): `fill`, `corners`
  - `block` *(element)* — a discrete segment, `filled` or empty, its tier by the threshold flags the connector sets
    on it (macOS's tiered capacity, each block coloured by its own tier, MAC-N06), its end corners by position among
    the blocks (`pos first`, `pos last`, GN-2.26): `fill`, `corners`
  - `tick` *(element)* — a tick mark, its width and its length (MAC-N06; RadialGauge's ticks, and its scale ticks,
    sized apart from them by the rules under the keyed `@scale`, W2-41†): `fill`, `min_width`, `min_height`
  - `needle` — Windows' needle and its border, with its size (W2-41†): `fill`, `stroke`, `min_width`, `min_height`
  - `value_label`, `unit_label` — Windows' value and unit text (W2-41†): `text`
- **Widget values:** `block_gap_px: Value<Px>`, `block_min_length_px: Value<Px>` (GN-2.26; `auto` where the platform
  sets no minimum block length, never `Absent`: `macos-*` unless its facts state one, MAC-N06 naming none);
  `stroke_width_px: Value<Px>`
  (the track's and value's width, C-30, W2-41†); `start_angle_deg: Value<Deg>`, `end_angle_deg: Value<Deg>` (circular,
  W2-41†); `cap: Value<LineCap>` (the track's and value's end caps: iOS's end caps, C-30; RadialGauge's round arc caps,
  W2-41†;
  ADD-A34; one home for the end treatment);
  `thresholds: Vec<LevelThreshold>`, `LevelThreshold { flag: Key, comparison: Comparison, value: Value<f32> }` (the
  bands, stated by the platform's defaults: GtkLevelBar's offsets, `NSLevelIndicator`'s warning and critical values;
  each entry's `flag` is the keyed flag the connector sets, and the connector sets the flag of the first entry, in
  stated order, whose `comparison` (S7.3's `Comparison`) of the element's value against `value` holds, and none
  where none holds: so the preset states its platform's rule, GtkLevelBar's lowest offset at or above the value as
  entries `AtMost` in rising order, NSLevelIndicator's warning and critical values as entries `AtLeast` from the
  critical down, or, where warning exceeds critical (AppKit's inverted scale, the values at or below the critical
  value critical), as entries `AtMost` from the critical up, the comparisons' strictness from AppKit's documentation
  (value phase), and the connector applies it the same way everywhere, with no per-preset table);
  `tick_spacing: Value<f32>` (RadialGauge's tick interval, in the gauge's value units, W2-41†).
- **Variants:** typed `continuous`, `discrete` (GNOME, macOS), `circular` (iOS, macOS SwiftUI, Windows' radial
  gauge; linear is the base), `vertical` (GNOME; macOS's orientation); keyed `macos-*` `@relevancy`, `@tiered` (the
  discrete
  capacity style drawing each block in its own tier's colour, `drawsTieredCapacityLevels`) (MAC-N06), `ios-*`
  `@accessory`, `@capacity` (C-30); `windows-11` on `tick` `@scale` (RadialGauge's scale ticks, W2-41†). On
  `macos-*` every `NSLevelIndicator` element carries its style's variant (`continuous`, `discrete`, `@relevancy`,
  MAC-N06), so the linear base, with no style variant, is reached only by SwiftUI's linear `Gauge` (MAC-N22), and
  its circular gauge is `circular`.
- **Sizes and densities:** `macos-*`: the control sizes (MAC-N06).
- **Flags:** shared `filled` (on `block`: a block the value reaches, GN-2.26, MAC-N06); keyed `adwaita` `@low`,
  `@high`, `@full` (GN-2.26); `macos-*` `@warning`, `@critical` (MAC-N06) (each the `flag` of a `thresholds` entry,
  which the connector sets by that value's rule, on the widget for its value and under `@tiered` on each block for
  the block's own position); state flags used: disabled
  (GN-2.26, W2-41†).
- **Exclusive groups:** none (the connector sets one threshold flag at a time; RadialGauge's Normal and Disabled
  are the single flag `disabled`); precedence: state models (S17).
- **may_be_absent:** parts: `needle` (all but `windows-11`: the needle is RadialGauge's alone, W2-41†),
  `unit_label` (all but `windows-11`: RadialGauge's unit text, W2-41†), `value_label` (`adwaita`, `macos-*`,
  `ios-*`: MAC-N22, C-30; Open items), `value` (`adwaita` by a rule under `variant discrete`, whose blocks take its
  place, GN-2.26), `block` (`ios-*`, `windows-11`: their gauges have no segments, C-30, W2-41†; `adwaita` by a rule
  outside `variant discrete`, GN-2.26), `tick` (`adwaita`, `ios-*`: GtkLevelBar and SwiftUI's Gauge draw no tick
  marks, GN-2.26, C-30); values: `block_gap_px` (`ios-*`, `windows-11`: no segments, C-30, W2-41†),
  `block_min_length_px` (`ios-*`, `windows-11`, as for `block_gap_px`), `start_angle_deg` (`adwaita`: GtkLevelBar
  has no circular form, GN-2.26), `end_angle_deg` (`adwaita`, as for `start_angle_deg`), `cap` (`adwaita`:
  GtkLevelBar rounds its ends by its corners, GN-2.26; `macos-*` by a rule outside `variant circular`, the linear
  `NSLevelIndicator` drawing no line caps, MAC-N06), `tick_spacing` (`adwaita`, `ios-*`: they draw no ticks, as
  for `tick`; `macos-*` where its facts show none, value phase: MAC-N06 names tick marks, not RadialGauge's
  interval in value units).
- **State-model source:** GNOME `_level-bar.scss`; macOS `NSLevelIndicator.Style`; iOS and macOS SwiftUI
  `GaugeStyle`; Windows `RadialGauge.xaml` states (Normal, Disabled).
- **Today's fields:** none (MAC-N06's stand-in, today's `progress_bar` field `fill_color`, stays `progress_bar`'s).
- **Notes:** MAC-N06's rating style (stars and placeholders) is the `rating` widget. iOS's `@capacity` is a gauge's
  capacity style; KDE's KCapacityBar is a bar, `progress_bar` `@capacity`: two kinds by R14's cross-check. GNOME's
  high-contrast trough is
  a rule under `cond contrast_high` (GN-2.26). The value's part was named `fill` until the tenth check, which renamed
  it `value`, after what it shows (the value's extent): `fill` named the field its paint is, not the part, where
  `status_page.icon` is an icon.

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
  - `play_overlay` — the image centred over the video (GNOME's OSD play image, a stateless image, GN-2.44): `fill`,
    `corners`, `min_width`, `min_height`, `icon`
- **Widget values:** `auto_hide_ms: Value<Ms>` (the bar hidden after the pointer stops: GN-2.54†, MAC-N30†, K-KA-35†;
  Windows' fade-out timing, W2-35); `volume_popup_hide_delay_ms: Value<Ms>` (K-KA-35†); `gap_px: Value<Px>` (the bar's
  box spacing, GN-2.54†); `cursor_hide_ms: Value<Ms>` (the pointer hidden over the video, `CursorShape::None`, once
  it has rested this long while the video plays full screen: GtkVideo's own timer, apart from the bar's,
  `gtkvideo.c:131-153`, GN-2.54†; the connector sets the cursor back on the next motion);
  `dropout_order: Vec<(VariantRef, u32)>` (the order in which the transport bar drops its buttons when it is too
  narrow for them all: each button's identity, `button`'s typed `play`, `volume`, `full_screen`, `frame_step`,
  `route` or a keyed one, with its `DropoutOrder` priority, MediaTransportControls' template,
  `MediaTransportControls_themeresources.xaml:402-406`, W-P4-18†, W2-35; which end of the order drops first is the
  template's, which the value phase cites; the empty list on every other preset, whose rows name no such order);
  `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear` (the controls
  shown and hidden: GNOME's crossfade
  reveal, GN-2.54†; Windows' fade-out, W2-35; AVKit's fade, C-42†).
- **Variants:** keyed `macos-*` `@inline`, `@minimal` (`AVPlayerViewControlsStyle`, MAC-N30†), and its floating
  style the typed `floating` (the meaning `toolbar`'s and `sheet`'s share: a bar detached over the content);
  `kde-breeze` `@ka_album` (K-KA-35†). The buttons' identities are `button`'s typed `play`, `volume`, `full_screen`,
  `frame_step`, `route` (GN-2.54†, W2-35, MAC-N30†, K-KA-35†, C-42†, W-P4-18†).
- **Sizes and densities:** none (no row states a size).
- **Flags:** none of its own (an active output route is the `route` button's `route_active`; the buttons are
  `button` elements with their own states).
- **Exclusive groups:** none (the buttons' CommonStates on `windows-11` are `button`'s); precedence: state models
  (S17).
- **may_be_absent:** parts: `play_overlay` (`windows-11`, `macos-*`, `ios-*`, `kde-breeze`: the image is GNOME's,
  GN-2.44; W2-35, MAC-N30† and C-42† name none, and kirigami-addons' centred download control is the `button`
  widget, Notes, K-KA-35†). values: `volume_popup_hide_delay_ms` (every preset but `kde-breeze`: the delay is
  kirigami-addons' popup's, K-KA-35†); `gap_px` (every preset but `adwaita`: GtkMediaControls' box spacing,
  GN-2.54†); `cursor_hide_ms` (every preset but `adwaita`: GtkVideo's pointer hiding, GN-2.54†; W2-35, MAC-N30†,
  C-42† and K-KA-35† name none).
- **State-model source:** GNOME `gtkmediacontrols.ui`, `gtkvideo.c`, `gtkscalebutton.ui`; Windows
  MediaTransportControls visual states; iOS `AVPlayerViewController` and macOS `AVPlayerView` control styles (AVKit);
  KDE kirigami-addons `VideoMaximizeDelegate.qml`.
- **Today's fields:** none.
- **Notes:** Nested controls: the buttons are `button` elements in `context media_controls.bar`, their identities
  `play`, `volume`, `full_screen`, `frame_step` (GN-2.54†, W2-35, MAC-N30†, K-KA-35†); the volume button's popup is a
  `popover` in `context media_controls` (GNOME's `.scale-popup`, GN-2.54†; Windows' volume flyout, W2-35;
  kirigami-addons' popup, K-KA-35†) holding the volume scale, a `slider` variant `vertical`; the seek bar is a
  `slider` in `context media_controls`. The output-route button (iOS's AirPlay button, C-42†; Windows'
  cast button, W-P4-18†) is `button @ variant route context media_controls.bar`. kirigami-addons' muted volume
  button shows its other icon, `button @ variant volume {checked} context media_controls.bar` (K-KA-35†). Its
  download button, a QQC2 Button with a `large` icon centred over the video before download, has hover and pressed
  states of its own, so it is `button @ variant @ka_download context media_controls` (a keyed `kde-breeze` variant of
  `button`, which `button` declares; Nested controls, K-KA-35†), not `play_overlay`, which stays GNOME's stateless
  image (GN-2.44). Group 7 proposed `play_overlay`
  (GN-2.44) and the cast button (W-P4-18†, now `button`'s `route`, rename row 82); the kirigami-addons rows added
  `@ka_album` and the volume
  popup's delay
  (K-KA-35†).

### menu_bar

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's GtkPopoverMenuBar (GN-2.30), KDE's QMenuBar (K2.1) and Qt Quick MenuBar (Q2.9,
  Q2.10), macOS's menu bar and status items (MAC-N20) with SwiftUI's `MenuBarExtra` window (MAC-N34†), Windows'
  MenuBar (W2-22), iPadOS's menu bar (C-37†).
- **Available:** every preset but `material-3` and `ios-13` (`available = false`: no such control; iPadOS draws it
  from 26 only, C-37†, so `ios-26` declares it). On a phone the system
  shows no menu bar: the application, not the theme, decides to show none there (the idiom is a condition it reads),
  so it queries no `menu_bar` and nothing is stated for it.
- **Parts:**
  - `bar` — the bar (the root; GNOME's bottom line, GN-2.30; KDE's Header set in the tools area and its inactive
    group, K2.1, Q2.9; macOS's wallpaper-tinted translucent bar, transparent on 26, MAC-N20; Windows'
    `MenuBarBackground`, W2-22; iPadOS's fill or material, C-37†; the bar's height, MAC-N20, W2-22): `fill`,
    `stroke`, `padding`, `min_height`, `bindings`
  - `item` *(element: a top-level title)* — fill and stroke per state (GN-2.30, K2.1, W2-22): `fill`, `stroke`,
    `corners`, `padding`, `min_width`, `min_height` (Kirigami's item height, Q2.9), `transitions` (KDE's highlight
    fade, Q2.10). `item_label` and `app_name` belong to `item` and are queried with it.
  - `item_label` — its text (KDE's highlighted text while hovered, Q2.10): `text`, `stroke` (KDE's weak-focus line
    under the hovered or pressed item's text, as on `menu.item_label`, K2.1, K3.21)
  - `app_name` — macOS's application menu title in its own text style (MAC-N20): `text`. The item that
    opens the application menu (the menu named after the application, the first on macOS) shows its title as
    `app_name` in place of `item_label`, so its open highlight is the item's `{open}` rules like every title's
    (MAC-N20); the application tells the connector which item that is, and only an application on macOS has one
  - `status_item` *(element: a macOS status item)* — its icon size (MAC-N20): `icon`
  - `extra_window` — SwiftUI `MenuBarExtra`'s window (material, MAC-N34†): `fill`, `corners`, `shadow`
- **Widget values:** `item_gap_px: Value<Px>`; `status_item_gap_px: Value<Px>` (MAC-N20);
  `extra_window_gap_px: Value<Px>`
  (MAC-N34†); the bar's height is `bar.min_height` (macOS's, MAC-N20; `MenuBarHeight`, W2-22) and Kirigami's item
  height `item.min_height` (Q2.9); `presentations: Vec<PresentationTransition>`:
  event `Appear` (iPadOS's hidden bar revealed at the top edge, C-37†).
- **Variants:** keyed `kde-breeze` `@quick` (the Kirigami MenuBar where it differs from QMenuBar, Q2.9, Q2.10).
  macOS 27's expanded interface of a status item (`NSStatusItemExpandedInterfaceSession`, MAC-N34†) is not one:
  `macos-tahoe` describes 26 (R1.5, D12), and it is carried to `docs/todo.md`'s macOS 27 entry.
- **Sizes and densities:** none. macOS's "Menu bar size" accessibility setting is the condition
  `cond container_size:menu_bar:large` (S7.1 `ContainerSize`, MAC-N20), not a size: the sizes declared in
  `container_sizes = { menu_bar = ["default", "large"] }` with the platform's default first (S10), the setting's
  Default and Large (MAC-N20), which the macOS reader reads from the setting (the readers section).
- **Flags:** none of its own (KDE's weak-focus look is the condition `!cond system_setting:menu_item_strong_focus`,
  Breeze's `MenuItemDrawStrongFocus`, as on `menu`, K2.1); state flags used: hover, pressed, open (on `item`: its menu
  shown, macOS's highlighted title, Windows' selected state, W2-22), disabled.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` on `item` (CommonStates), `open` (W2-22's selected,
  the menu open) combining; others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `app_name` (every preset but `macos-*`: the application menu's own title is macOS's,
  MAC-N20), `status_item` (every preset but `macos-*`: status items live in macOS's menu bar, MAC-N20),
  `extra_window` (every preset but `macos-*`: SwiftUI's `MenuBarExtra`, MAC-N34†). values: `status_item_gap_px`
  (every preset but `macos-*`, as for `status_item`), `extra_window_gap_px` (every preset but `macos-*`, as for
  `extra_window`).
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
  - `incoming_page` *(element)* — the page a transition brings in (the pushed page; on a pop, the page uncovered):
    `transform`, `opacity` (during a transition, set by the steps that name it).
  - `outgoing_page` *(element)* — the page a transition takes away (the page covered by a push, with iOS's parallax
    and GNOME's dim beneath; on a pop, the page leaving): `transform`, `opacity`. Two parts, so a presentation step
    names the one it moves (S2.7: a step names a part, never a variant), and the connector queries each page of a
    transition as the one it is.
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
  `swipe_back: Value<bool>` (the swipe-back gesture, GN-2.18; `false` where the platform has none, never `Absent`);
  `swipe_duration_min_ms`,
  `swipe_duration_max_ms: Value<Ms>` (the swipe's animation bounds, GN-2.18).
- **Variants:** keyed `kde-breeze` `@layer` (PageRow layers, Q2.15); `macos-*` `@stack_history`, `@stack_book`,
  `@horizontal_strip` (`NSPageController.TransitionStyle`, MAC-P3-10†).
- **Sizes and densities:** none (no row states a size).
- **Flags:** none (no row states a widget flag).
- **Exclusive groups:** none (no interaction states); precedence: state models (S17).
- **may_be_absent:** parts: `edge_shadow` (`kde-breeze`: StackView and PageRow slide and fade without one, Q2.14,
  Q2.15; `windows-11`, `material-3`: WinUI's Frame transitions and Material's transition patterns, W3-32, MAT-X14,
  name none; `macos-*` by a rule outside `variant @stack_history` and `variant @stack_book`, the shadow under the
  top page being theirs, MAC-P3-10†, to verify), `scrim` (`kde-breeze`, as for `edge_shadow`, Q2.14, Q2.15;
  `windows-11`, `material-3`, as for `edge_shadow`, W3-32, MAT-X14; `macos-*` where its facts show none, value
  phase: MAC-P3-10† names the shadow only). values: `swipe_duration_min_ms` (every preset but `adwaita`: the swipe's
  animation bounds are AdwSwipeTracker's, GN-2.18), `swipe_duration_max_ms` (every preset but `adwaita`, as for
  `swipe_duration_min_ms`).
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
  on hover, W2-20); `edge_dots_shrink: Value<bool>` (iOS's shrinking edge dots when overflowing, C-9; `false` where
  the platform does not shrink them, never `Absent`).
- **Variants:** typed `vertical` (iOS's vertical direction, C-9; Windows' orientation, W2-20); keyed `adwaita`
  `@lines` (GN-2.55†); `ios-*` `@prominent`, `@minimal` (background styles, C-9; automatic is the base); `windows-11`
  `@button_panel`, `@combo_box`, `@number_box` (PagerControl's display modes, W2-21: the application's choice of
  which control the pager shows; under `@combo_box` and `@number_box` that control is a `combo_box` or `stepper`
  element in `context page_indicator.body`, Nested controls, so the variants select only the pager's own parts).
- **Sizes and densities:** none (no row states a size).
- **Flags:** keyed `ios-*` `@scrubbing` (C-9: UIPageControl's continuous interaction, a finger dragged along the
  dots; the connector sets it from that interaction on every platform, and it selects nothing where undeclared);
  state flags used: selected, hover, pressed (KDE's pressed opacity, Q2.11), disabled.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` on `dot` (CommonStates), `selected` combining;
  others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `dot` (`windows-11` by a rule under `variant @button_panel`, `variant @combo_box` and
  `variant @number_box`: PagerControl draws number buttons, a combo box or a number box, not PipsPager's pips,
  W2-21); `line` (every preset but `adwaita`: AdwCarouselIndicatorLines, GN-2.55†); `platter` (every preset but
  `ios-*`: UIPageControl's background style, C-9; `ios-*` by a rule outside `{@scrubbing}` and
  `variant @prominent`, the automatic style drawing it only while interacting and `@minimal` never, C-9);
  `progress` (every preset but `ios-*`: UIPageControl's timer fill, C-9); `selection_indicator` (every preset but
  `windows-11`: PagerControl's, W2-21; `windows-11` by a rule outside `variant @button_panel`, the number buttons
  it marks, W2-21). values: `max_visible` (every preset but `windows-11`: PipsPager's, W2-20);
  `nav_buttons_visibility` (every preset but `windows-11`: PipsPager's, W2-20).
- **State-model source:** GNOME `adw-carousel-indicator-dots.c`, `adw-carousel-indicator-lines.c`; iOS
  `UIPageControl`; KDE `PageIndicator.qml`; Windows PipsPager and PagerControl visual states.
- **Today's fields:** none.
- **Notes:** Nested controls: Windows' previous, next, first and last buttons are `button @ variant previous / next
  context page_indicator` and `button @ variant @first / @last context page_indicator` (keyed `windows-11`, W2-20,
  W2-21); PagerControl's number buttons are `button @ variant @page_number context page_indicator` (W2-21: its
  width and states are the button's there; the check pass had a `page_button` part).

### page_preview

*Written for the tenth check (K-P3-9†) as a KDE component; typed in the eleventh check's fix, when GTK's own print
dialog was carried (GN-2.57†): two platforms draw it (R14, S5.1).*

- **Kind:** typed widget: the page preview of a print or page setup dialog the toolkit draws in the application's
  process, in by R1.1: Qt Print Support's `QPagePreview` in QPageSetupDialog (`qpagesetupdialog_unix.cpp:61-142`,
  K-P3-9†; flavour `widgets`) and GtkPrintUnixDialog's preview, its `drawing` node holding the page's `paper` node
  (`_dialogs.scss:10-23`, GN-2.57†). QPrintDialog, QPageSetupDialog and KIO's Open With dialog, and GTK's
  GtkPrintUnixDialog, GtkPageSetupUnixDialog and GtkAppChooserDialog, are `dialog` elements with standard widgets in
  their context (GTK's in a `window` with the content variants `print`, `page_setup`, `app_chooser`, which select
  their own rules); the page preview is the one thing of their own the print dialogs draw. GTK draws its dialogs
  where the application builds them itself, or where GtkPrintDialog finds no Print portal
  (`gtkprintdialog.c:1502-1508`); through the portals, xdg-desktop-portal-gnome (GN-P4-3†) and
  xdg-desktop-portal-kde (K-P3-9†) draw them in their own processes, which stays out (R1.1). Cross-check: macOS's
  print panels are out (MAC-N25, R1.1), Windows' Page Setup is comdlg32's, another toolkit (W-P4-15†, R1.3), iOS's
  print options are system-drawn (P3-8†), Android's print dialog is a system activity (the Material map's GN-2.57†
  row).
- **Available:** `adwaita`; `kde-breeze` flavour `widgets` only (the `kirigami` flavour declares
  `[declare.flavor.kirigami.widget.page_preview] available = false`, S10); every other preset declares `available =
  false` (`macos-*`, `windows-11`, `ios-*`, `material-3`: no row shows a page preview the toolkit draws).
- **Parts:**
  - `body` — the root, the preview's box: `padding` (`kde-breeze`: the inset the page is fitted within, `:92`;
    `adwaita`: the `drawing` node's, zero, `_dialogs.scss:15`, which draws no background and no border there,
    `:13-14`), `min_width`, `min_height` (`kde-breeze`: `:72`; `adwaita`: the drawing area's size request, which the
    value phase states from `gtkprintunixdialog.c`, `auto` where it sets none). The connector fits the page inside
    the padding at the page layout's aspect ratio and centres it (`:91-94`): the page's size and its margins are the
    application's page layout, not theme.
  - `page_shadow` *(element; one per line of the shadow, `element.item_index` its index from zero)* — one of the
    thin lines right of and below the page, each one step further out, in `role.@shade_mid` at an opacity falling
    from line to line, drawn before the page (`:106-113`): `stroke` (its `bottom` and `right` edges only, the others
    `Absent`, One-edge lines; their paint `= alpha(role.@shade_mid, sub(<first opacity>, mul(element.item_index,
    <opacity step>)))`, their width the source's pen, `:110`), `transform` (`translate_x` and `translate_y` `=
    mul(add(element.item_index, 1), <step>)`, the line's step out from the page, `QRect::adjusted`, `:109`);
    `data = { page_shadow = ["item_index"] }`. The connector lays each line's box out on the page's box and queries
    `values.shadow_lines` lines. The route is exact: per line the source strokes the bottom and right sides of the
    page's rectangle moved by its step, in one colour, and no two lines share a pixel (`:107-113`), so one stroked
    box per line, moved by its step, paints each pixel once at its own line's opacity. Offset `shadow` layers
    overlap and compound their alphas, and one `Edge` per side holds one width and one paint, so neither reproduces
    the five lines. Two points the value phase confirms: `alpha` multiplies the colour's alpha where the source's
    `setAlpha` sets it (`:108`), the same while `role.@shade_mid` is opaque; and the source mirrors nothing in a
    right-to-left layout (`:94`, `:107-113`): the stroked edges are physical in the model too (S2.4), and the move,
    a translation the model mirrors (S2.6), is restated by a `dir rtl` rule.
  - `page` — the page: `fill` (`kde-breeze` `= role.@shade_light`, `:114`; `adwaita` the `paper` node's background,
    `_dialogs.scss:18`, inside its border, `background-clip: padding-box`, `:20`), `stroke` (`adwaita`: the `paper`
    node's border on every edge, its width and paint the source's, `:21`; `kde-breeze` draws no outline round the page,
    `:114`).
  - `margins`, in `page` — the page layout's margin rectangle: `stroke` (each edge's `dash` `Dotted`, its paint
    `role.@shade_dark`, `:117-118`), `padding` (the filler text's inset inside the line, `:120`).
  - `filler_text`, in `margins` — the text the preview draws on the page: `kde-breeze`'s sample text filling each
    page block, word-wrapped and centred vertically (`:121-139`); `adwaita`'s, the text GtkPrintUnixDialog draws on
    the `paper` node, in that node's `color` (`_dialogs.scss:19`), its content, font and placement the dialog's code
    (`gtkprintunixdialog.c`, which the value phase reads; nested in `margins`, it is queried and drawn where `margins`
    answers `NotDrawn`, Nesting): `text` (`kde-breeze`: its fill `= role.@shade_dark`, its font size `=
    mul(token.text.roles.body.font.size, <factor>)`, the application font scaled by the source's factor, `:122-125`).
- **Widget values:** `filler_gap_fraction: Value<f32>` (the gap between the filler text's blocks, when the user
  chooses several pages per sheet, as a fraction of the page's width, which the connector multiplies by the page's
  laid-out width, `:130-132`); `shadow_lines: Value<u32>` (how many `page_shadow` lines the shadow has, the source's
  loop, `:107`).
- **Variants:** none (the pages-per-sheet layout is the user's choice in the dialog, data, `:81-86`, `:617-626`).
- **Sizes and densities:** none.
- **Flags:** none (the preview holds no interaction state; its shadow lines differ by `element.item_index` alone).
- **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** parts: `page_shadow` (`adwaita`: the `paper` node has a border and no shadow lines,
  `_dialogs.scss:17-22`); `margins` (`adwaita`: the stylesheet styles no margin rectangle, `_dialogs.scss:10-23`, and
  GN-2.57† names none; the value phase confirms from `gtkprintunixdialog.c` that GTK draws none). values:
  `filler_gap_fraction` (`adwaita`: QPagePreview's block gap, `:130-132`, which GN-2.57† does not name; the value
  phase confirms from `gtkprintunixdialog.c`); `shadow_lines` (`adwaita`: no shadow lines, as for the shadow's part). On
  `kde-breeze` the margin rectangle and its filler text are drawn whenever the page layout leaves a margin rectangle
  (`:116`); one it leaves none of is not queried, as for content (S6.3).
- **State-model source:** none of its own: the preview has no states (`kde-breeze`: `qpagesetupdialog_unix.cpp:67-151`,
  qtbase v6.10.1, K-P3-9†; `adwaita`: `_dialogs.scss:10-23` styles no state of the `drawing` or `paper` node).
- **Today's fields:** none.
- **Notes:** the preview is queried in its dialog's context (`context dialog`; GTK's print dialog's in `context
  window variant print` too), and nests no control. The `drawing` node's own `color` (`_dialogs.scss:12`) reaches
  only what GtkPrintUnixDialog draws outside the page; the value phase checks `gtkprintunixdialog.c` for such
  drawing, and a finding adds the part that carries it. The keyed `component.page_preview` of the tenth check is this
  widget (K-P3-9†); its parts and values are kept, and its values, declared `"number"` and `"integer"` there, are the
  typed `Value<f32>` and `Value<u32>` here.

### picker_wheel

*From group 6's catalogue.*

- **Kind:** typed widget: iOS's UIPickerView and UIDatePicker `.wheels` (C-7), Material Expressive's scroll field
  (MAT-N31†), Windows' looping selectors in the date and time picker flyouts (W2-04), the tumblers of kirigami-addons'
  time popup (K-KA-23†). One `picker_wheel` element is one column (one UIPickerView component, one looping selector,
  one tumbler, Material's scroll field), on every platform: a picker of several columns is several sibling elements
  in its host's context, so a connector builds one tree everywhere.
- **Available:** every preset but `adwaita`, `macos-*` (`available = false`: no such control; macOS's pickers are
  textual and graphical only, MAT-N31†'s cross-check). `kde-breeze` flavour `kirigami` only, in kirigami-addons'
  time popup (K-KA-23†). `material-3` flavour `expressive` only (the scroll field is Material 3 Expressive's,
  MAT-N31†): `[declare.flavor.baseline.widget.picker_wheel] available = false`.
- **Parts:**
  - `container` — the root, always drawn: Material's container, fill per state, its shape by corners and its
    height (MAT-N31†); on the other presets its fill and corners `Absent`, its minimum height `auto` (the height is
    its items', never `Absent`) and its padding a zero inset: `fill`, `corners`, `min_height`, `padding`
  - `item` *(element; the centred item is `selected`)* — `text`, `fill`, `opacity` (the text style's size, line
    height and tracking and the opacity by the distance from the centre, the item's index less the column's position,
    as expressions over `element.scroll_progress`, `element.item_index` and `element.item_count`, which it declares,
    `data = { item = ["scroll_progress", "item_index", "item_count"] }`, ADD-B13: iOS's fading of off-centre
    rows, C-7, kirigami-addons' opacity by displacement, K-KA-23†, Material's text interpolated between two ramp
    roles, MAT-N31†), `padding` (Windows' item padding, W2-04), `min_height` (the item's height, C-7, W2-04,
    K-KA-23†), `focus` (Material's focus ring, MAT-N31†),
    `transitions` (the snap to the centre, group `Geometry` with a spring, C-7, MAT-N31†)
  - `selection_band` — the band (iOS's band, C-7; Windows' highlight band with its height, W2-04; kirigami-addons'
    highlight border, thicker with keyboard focus, K-KA-23†): `fill`, `stroke`, `corners`, `min_height`, `margin` (its
    insets, C-7)
- **Widget values:** `visible_items: Value<u32>` (C-7, MAT-N31†, K-KA-23†);
  `curvature: Value<f32>` (iOS's perspective curvature, C-7); `component_gap_px: Value<Px>` (the space between two
  adjacent columns, sibling `picker_wheel` elements, as `toast.values.stack_gap` is between stacked toasts: iOS's
  gap between a UIPickerView's components, C-7; the space between Windows' looping selectors and between
  kirigami-addons' tumblers is this value too, measured in the value phase where W2-04 and K-KA-23† state none);
  `momentum: ScrollPhysics` (a plain value, S4.2's, C-7).
- **Variants:** none (no row names a style).
- **Sizes and densities:** no sizes (no row states one); `windows-11`: density `compact` (WinUI's compact sizing dictionary, W3-39): its items' padding in the date and time flyouts.
- **Flags:** none of its own; state flags used: selected (the centred item), disabled (Material's disabled colours),
  focus_visible, Material's focus ring (MAT-N31†); hover and pressed (`windows-11`'s looping selector item, the group
  below).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` on `item` (the looping selector item's
  CommonStates), `selected` combining; others none (states combine); precedence: state models (S17).
- **may_be_absent:** fields: `container.fill` (every preset but `material-3`: the container is Material's,
  MAT-N31†; the others' wheels draw no container), `container.corners` (every preset but `material-3`, as for
  `container.fill`); the container's minimum height is `"auto"` on every preset but `material-3` (the others' height
  is their items', C-7, W2-04, K-KA-23†: no minimum beyond the content, S2.11), never `Absent`, and its padding is
  stated on every preset, a zero inset where the wheel has none. values: `curvature` (every preset but `ios-*`:
  UIPickerView's perspective, C-7),
  `component_gap_px` (`material-3`: its scroll field is one column standing alone, MAT-N31†).
- **State-model source:** iOS `UIPickerView`; Material `ScrollField.kt`; Windows `LoopingSelector` visual states; KDE
  kirigami-addons `TimePicker.qml` (its Tumbler from Qt's fallback style, to verify, K-KA-23†).
- **Today's fields:** none.
- **Notes:** values interpolated by the scroll position (Material's text style between two ramp roles, iOS's fade and
  curvature, kirigami-addons' opacity by distance) read element data (`element.scroll_progress`, `element.item_index`,
  `element.item_count`, ADD-B13). The looping selectors (W2-04) are `picker_wheel` elements, one per column, in
  `context time_picker.popup` and `context date_picker.popup`, and kirigami-addons' tumblers (K-KA-23†) in
  `context time_picker`.

### pull_to_refresh

*From group 6's catalogue.*

- **Kind:** typed widget: iOS's UIRefreshControl (C-10), Kirigami ScrollablePage's PullDownIndicator (Q2.54†, Q1.121),
  Material's Compose PullToRefresh (MAT-N28), Windows' RefreshContainer and RefreshVisualizer (W2-26).
- **Available:** every preset but `adwaita`, `macos-monterey`, `macos-tahoe` (`available = false`: GNOME has no such
  control; macOS draws one only from 27, `NSScrollView.refreshController`, MAC-N24, a release `macos-tahoe` does not
  describe, R1.5, D12, carried to `docs/todo.md`'s macOS 27 entry). `kde-breeze` flavour `kirigami` only (Q2.54†).
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
  events `Disappear` (the control going away after a refresh: iOS's collapse after `endRefreshing`, C-10; Material's motion, MAT-N28; the element disappearing, Entry format), `@content_shift` (KDE's
  content moving down, Q2.54†).
- **Variants:** none (no row names a style).
- **Sizes and densities:** none (no row states a size).
- **Flags:** shared `pulling`, `refreshing` (every preset that draws it); state flags used: none other.
- **Exclusive groups:** none (the connector sets one phase at a time); precedence: state models (S17).
- **may_be_absent:** parts: `arrow` (every preset but `windows-11`: RefreshVisualizer's arrow glyph, W2-26);
  `title` (every preset but `ios-*`: UIRefreshControl's attributed title, C-10); `strip` (every preset but
  `kde-breeze`: Kirigami's strip above the content, Q2.54†). fields: `indicator.fill` (`ios-*`, `kde-breeze`:
  UIRefreshControl and Kirigami's indicator draw no disc, C-10, Q2.54†), `indicator.corners` (`ios-*`,
  `kde-breeze`, as for `indicator.fill`), `indicator.shadow` (`ios-*`, `kde-breeze`, as for `indicator.fill`); the
  body's padding is stated on every preset, a zero inset where the control has none. values: `refresh_delay_ms`
  (every preset but `kde-breeze`: Kirigami's `refreshDelay`, Q2.54†); `content_shift_px` (every preset but
  `kde-breeze`: Kirigami's content moving down by the strip, Q2.54†); `loop` (every preset but `windows-11`: the
  others spin the spinner element, W2-26's spin animation being the visualizer's own); `progress_sweep`
  (`windows-11`: RefreshVisualizer draws an arrow glyph in its circle, neither an arc nor spokes, W2-26).
- **State-model source:** iOS `UIRefreshControl`; KDE `PullDownIndicator.qml`, `ScrollablePage.qml`; Material
  Compose `PullToRefresh` states; Windows `RefreshVisualizerState`.
- **Today's fields:** none.
- **Notes:** Nested controls: the pull-progress arc or spokes are a `spinner` in `context pull_to_refresh`,
  `!{indeterminate}` while `{pulling}`, its progress the pull progress (`element.progress`, ADD-B12; KDE's arc with its
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
- **Widget values:** `item_gap_px: Value<Px>` (W2-06); `half_steps: Value<bool>` (K2.19; `false` where the platform
  has no half steps, never `Absent`).
- **Variants:** none (no row names a style).
- **Sizes and densities:** none (no row states a size).
- **Flags:** shared `filled`, `half` (KDE, Windows, macOS), `preview` (the hover preview: KDE, Windows); shared
  `placeholder` (`windows-11`'s placeholder colour, W2-06; `macos-*`'s rating placeholders, MAC-N06); state flags
  used: hover, pressed, disabled.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` (CommonStates); others none (states combine);
  precedence: state models (S17).
- **may_be_absent:** parts: `caption` (every preset but `windows-11`: RatingControl's caption, W2-06). values: none.
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
    MAT-P3-20†): `text`. The underline's and the strike line's `text.decoration` follow the run's font where the
    platform draws them from its metrics (Qt's text layout, AppKit, Android's `Paint`, S2.8): `thickness` `=
    element.font_underline_thickness` and `offset` `= element.font_underline_offset` on `underline`,
    `element.font_strikeout_thickness` and `element.font_strikeout_offset` on `strikethrough`, which the parts
    declare, `data = { underline = ["font_underline_offset", "font_underline_thickness"], strikethrough =
    ["font_strikeout_offset", "font_strikeout_thickness"] }` (S2.8, S8); a number only where a platform's facts show
    a fixed line
  - `inline_code` (K-SF13†, MAC-SF03†, SF-3†): `text`, `fill`
  - `code_block` *(element)* (K-SF13†; an element because the connector sets its content variant `fenced`): `text`,
    `fill`, `margin`
  - `preformatted` (K-P4-14†): `text`, `margin`
  - `block_quote` (K-SF14†: no fill, no bar): `fill`, `stroke`, `margin`
  - `list_marker` — the marker before a list item: `glyph` (a bullet kind's disc, circle or square, K-SF15†; its size
    an expression on the paragraph's line height, `widget.paragraph.line_height`: the bullet has no font of its own,
    S2.2), `text` (an ordered list's number or letter, drawn as text in the item's font by the same layout code,
    `qtextdocumentlayout.cpp`, K-SF15†); which one a marker takes follows its kind in `list_marker_kinds`. The
    glyph's size reads another part's metric, which the part declares, `data = { list_marker =
    ["widget.paragraph.line_height"] }` (S8, S10); a part's `line_height` operand is the line spacing, never `Auto`
    (S2.2)
  - `task_box` *(element)* — the square and its cross when checked, an element of its own with its `checked` state
    (K-SF15†): `stroke`, `glyph`
  - `table` — the table's collapsed border (K-SF16†; per edge, Qt's Outset style giving the top and left edges one
    shade and the bottom and right the other): `stroke`, `margin`
  - `table_cell` — a cell with its padding and border (K-SF16†): `padding`, `stroke`, `text`
  - `table_header_cell` — a header cell, its text centred and heavier (K-SF16†, K-P4-14†): `padding`, `stroke`,
    `text`
  - `rule` — the horizontal rule (K-SF17†): `stroke`, `margin`
  - `definition_term`, `definition_description` (K-P4-14†): `text`, `margin`
- **Widget values:** `list_indent_per_level_px: Value<Px>` (each block part's margins are its `margin`, S2.11);
  `list_marker_gap_px: Value<Px>` (K-SF15†); `list_marker_kinds: Vec<Key>` (per nesting level, K-SF15†; its keys
  `disc`, `circle`, `square`, the bullet kinds, drawn by `list_marker.glyph`, and `decimal`, `lower_alpha`,
  `upper_alpha`, `lower_roman`, `upper_roman`, the numbered kinds of Qt's `QTextListFormat`, drawn as text by
  `list_marker.text`; the value phase confirms the set against `qtextdocumentlayout.cpp:2154-2239`, which K-SF15†
  cites);
  `quote_indent_per_level_px: Value<Px>` (K-SF14†).
- **Variants:** typed `markdown` (KDE's Markdown importer, SwiftUI `Text` Markdown), `html` (Qt's HTML parser, Apple's
  HTML import, Compose `AnnotatedString.fromHtml`). The content variant `fenced` on the `code_block` element (typed,
  as every content variant is, ADD-A61: the connector sets it from the block's source; a fenced block's lines do not
  wrap, K-SF13†).
- **Sizes and densities:** none (the text follows the text ramp).
- **Flags:** none of its own; state flags used: checked (on the `task_box` element, K-SF15†).
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `heading_1` (`macos-*`, `ios-*` by a rule under `variant markdown`: SwiftUI's `Text`
  draws Markdown's inline intents only, no block part, MAC-SF02†, SF-2†), `heading_2` (`macos-*`, `ios-*` by a rule
  under `variant markdown`, as for `heading_1`), `heading_3` (`macos-*`, `ios-*` by a rule under `variant markdown`,
  as for `heading_1`), `heading_4` (`macos-*`, `ios-*` by a rule under `variant markdown`, as for `heading_1`),
  `heading_5` (`macos-*`, `ios-*` by a rule under `variant markdown`, as for `heading_1`), `heading_6` (`macos-*`,
  `ios-*` by a rule under `variant markdown`, as for `heading_1`), `list_marker` (`macos-*`, `ios-*` by a rule under
  `variant markdown`, as for `heading_1`), `task_box` (every preset but `kde-breeze`: the task items are Qt's
  Markdown importer's, K-SF15†), `block_quote` (`macos-*`, `ios-*` by a rule under `variant markdown`, as for
  `heading_1`; `material-3` where its render shows no quote style, value phase: Compose maps no `QuoteSpan`,
  MAT-P3-20†), `code_block` (`macos-*`, `ios-*` by a rule under `variant markdown`, as for `heading_1`;
  `material-3`: Compose's `fromHtml` draws no code block, MAT-P3-20†), `preformatted` (`macos-*`, `ios-*` by a rule
  under `variant markdown`, as for `heading_1`; `material-3`, as for `code_block`), `table` (`macos-*`, `ios-*` by
  a rule under `variant markdown`, as for `heading_1`; `material-3`: Compose's `fromHtml` draws no table,
  MAT-P3-20†), `table_cell` (`macos-*`, `ios-*` by a rule under `variant markdown`, as for `heading_1`;
  `material-3`, as for `table`), `table_header_cell` (`macos-*`, `ios-*` by a rule under `variant markdown`, as for
  `heading_1`; `material-3`, as for `table`), `rule` (`macos-*`, `ios-*` by a rule under `variant markdown`, as for
  `heading_1`, and by a rule under `variant html` where their facts show none, value phase: MAC-SF06† and SF-6†
  name no rule; `material-3`: Compose's `fromHtml` draws no rule, MAT-P3-20†), `definition_term` (every preset but
  `kde-breeze`: the definition lists are Qt's HTML, K-P4-14†), `definition_description` (every preset but
  `kde-breeze`, as for `definition_term`). fields: `inline_code.fill` (`kde-breeze`: Qt
  draws inline code in the monospace font alone, no fill, K-SF13†), `code_block.fill` (`kde-breeze`, as for
  `inline_code.fill`, K-SF13†), `block_quote.fill` (`kde-breeze`: no fill and no bar, K-SF14†),
  `block_quote.stroke` (`kde-breeze`, as for `block_quote.fill`, K-SF14†).
- **State-model source:** KDE `qtextmarkdownimporter.cpp`, `qtextdocumentlayout.cpp`, Qt's HTML parser
  (`qtexthtmlparser.cpp`); Apple `InlinePresentationIntent`, `PresentationIntent`, the HTML importer; Compose
  `Html.android.kt` span mapping.
- **Today's fields:** none. Stand-ins: `defaults.mono_font` (K-SF13†, MAC-SF03†, SF-3†) becomes the text ramp's
  monospace role, referenced from `inline_code.text` (`= token.text.roles.monospace`); `defaults.link_color`
  (MAC-SF04†, SF-4†) the `link` widget's text fill in `context rich_text`.
- **Notes:** links in the text are `link` variant `inline` in `context rich_text`, with hover, pressed and visited
  where the platform draws them (MAC-SF04†, SF-4†, K-P4-14†, MAT-P3-20†). Markdown shown by a text view or label
  (UIKit's `UITextView` and `UILabel`, AppKit's `NSTextView`, given an `AttributedString(markdown:)`, SF-5†,
  MAC-SF05†) is this widget under `variant markdown`, in the text view's context where it is one (`context any
  text_area`): one structure for the platform's rendering of Markdown's presentation intents, whichever view shows
  them; which intents UIKit and AppKit render, and whether they draw them as SwiftUI's `Text` does, the probe shows
  (to verify), and a rendering that differs is a rule in that context.

### rubber_band

*From group 6's catalogue.*

- **Kind:** typed widget: iOS's UIBandSelectionInteraction (C-47†), KDE's QRubberBand (K2.12) and its dock drop zone
  (K-SF18†), macOS's marquee in Finder and `NSCollectionView` (MAC-N31†), GNOME's `rubberband` node in lists and icon
  views (GN-1.175, GN-1.280†) and libpanel's dock drop overlay (GN-SF14†, the kind of K-SF18†'s drop zone).
- **Available:** every preset but `windows-11`, `material-3` (`available = false`: no row lists one); `kde-breeze`
  flavour `widgets` only (QRubberBand).
- **Parts:**
  - `body` — the band, the root: `fill`, `stroke`, `corners`, `bindings` (the selection roles of the view it is drawn
    over, K2.12, GN-1.175), `transitions` (libpanel's overlay fades its fill and ring, GN-SF14†).
- **Widget values:** `presentations: Vec<PresentationTransition>`: event `Disappear` (iOS's fade, C-47†).
- **Variants:** the content variants `in_canvas` (the band is drawn over a free-form drawing surface, Qt's
  `QGraphicsView`, a canvas, K2.12) and `dock_target` (the band marks a dock drop zone over a main window, K-SF18†),
  hosts the model types no widget for, so no `context` rule can tell them apart (every band, a floating dock group's
  included, is queried below a `window`); the connector sets them from the band's host on every platform (ADD-A61),
  `kde-breeze` styles both and `adwaita` `dock_target` (libpanel's drop overlay in `context
  component.dock_frame_drop.frame`, GN-SF14†), and the other presets ignore them (S8); inside a list the `context any list` rule fills
  the band (K2.12).
- **Sizes and densities:** none (the band follows the drag).
- **Flags:** state flag `drop_target` (under `dock_target`: the drop is over this zone, libpanel's stronger fill and
  inset ring, GN-SF14†); no widget flag.
- **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** fields: `body.fill` (`kde-breeze` by its base, outside the rules `context any list`,
  `variant in_canvas` and `variant dock_target` that add the fill: Breeze draws the outline only elsewhere, K2.12,
  K-SF18†; a drawing field, S6.3, listed for the record).
- **State-model source:** iOS `UIBandSelectionInteraction`; KDE Breeze `CE_RubberBand` and `SH_RubberBand_Mask`; macOS
  as drawn by Finder and `NSCollectionView` (measured); GNOME `_views.scss` (`rubberband`) and libpanel's
  `stylesheet.css` (`paneldropcontrols`, GN-SF14†).
- **Today's fields:** none.
- **Notes:** KDE fills it inside an item view, a graphics view or a main window and draws the outline elsewhere: the
  base is the outline, and a `context any list` rule and a `variant in_canvas / dock_target` rule add the fill
  (K2.12; K-SF18†'s dock drop zone over a main window under `dock_target`, the outline over a floating dock group). Group 4 traced the list's
  marquee here (GN-1.175, GN-1.280†): `rubber_band.body` in
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
- **Widget values:** `thickness_px: Value<Px>`; `tick_lengths: Vec<(Key, Value<Px>)>` (its keys, per preset, the
  set a stated key is checked against: `kde-breeze` `end`, `big`, `medium`, `little`, `base`, KRuler's mark levels,
  K2.32†; `macos-*` the mark levels `NSRulerView` draws, which the value phase names from MAC-N17's render).
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
  - `layer` — the root, which draws nothing: it holds the field's box `body` and, beside it, the search view `view`
    and its `scrim`, so the view and the dim are not inside the field's box; `bar`, `recents_indicator` and
    `supporting_text` nest in `body`, and the field (an `input` element) and its buttons nest in `bar` (Notes):
    `bindings` (empty on every preset).
  - `body` — the search field's outer box: `padding`, `bindings`.
  - `bar` — GNOME's search bar, GN-2.21; Material's search bar container, MAT-N14; UISearchBar's background in its
    default and prominent styles, C-3: `fill` (GNOME's header-bar fill;
    Material's container, its on-scroll fill in an app bar, MAT-TA6), `stroke` (GNOME's bottom shade line), `corners`,
    `min_height`, `shadow`, `padding`.
  - `recents_indicator` — macOS's, MAC-N02: `glyph`.
  - `supporting_text` — MAT-N14: `text`.
  - `view` — Material's search view, MAT-N15; kirigami-addons' popup surface, K-KA-32†: `fill`, `stroke`, `corners`,
    `shadow`, `bindings` (kirigami-addons' View set, K-KA-18†, K-KA-32†).
  - `view_header`, in `view` — MAT-N15: `min_height`.
  - `view_divider`, in `view` — MAT-N15; kirigami-addons' separator under the field, K-KA-32†: `stroke` (the line, its top
    edge).
  - `scrim` — the dim behind Material's search view, MAT-X23: `fill`, `opacity`.
- **Widget values:** `placeholder_alignment: Value<Alignment>` (macOS's centred placeholder, which moves to the
  leading edge on focus, MAC-N02; the move is the `placeholder`'s `transitions`); `view_placement: Value<Placement>`
  (kirigami-addons' popup, strategy `KirigamiAddonsSearchPopup`, K-KA-32†); `auto_search_min_chars: Value<u32>`
  (kirigami-addons' `autoAccept`, K-KA-32†); `results_gap_px: Value<Px>` (the space between the bar and its results
  in Expressive's contained docked view, MAT-N15: the results are a `list` in `context search_field.view`, their
  shape that list's `view.corners`); `presentations: Vec<PresentationTransition>`: events `Appear`,
  `Disappear` (GNOME's search bar revealed and hidden, a revealer's slide both ways, GN-3.22, GN-2.21; a step on
  `body`),
  `@view_show`, `@view_hide` (Material's search view, MAT-N15; kirigami-addons' popup growing from the field and
  back, K-KA-32†), `@expand` (macOS's toolbar search item expanding from a button into a field, MAC-TL07).
- **Variants:** typed `flat` (GNOME's flat search bar, GN-2.21; `button`'s meaning); keyed `adwaita` `@inline`,
  `@shrunk` (the search bar, GN-2.21); `ios-*` `@prominent`, `@minimal` (`searchBarStyle`, C-3; the default style
  is the base); `material-3` `@docked`, `@full_screen` (the view, MAT-N15),
  `@contained` (flavour `expressive`, declared under `[declare.flavor.expressive.widget.search_field]`, S10, so
  `flavor:baseline` answers `NotAvailable` for it: Expressive's contained search, MAT-N14); `kde-breeze` `@ka_search_popup` (K-KA-32†), `@delay_search` (Kirigami
  SearchField's `delaySearch`: the field's search delay the human moment instead of the short duration, Q1.17).
- **Sizes and densities:** `macos-*`: its control sizes (MAC-N02).
- **Flags:** none of its own (the text's `empty` is the `input`'s, which hides the clear button); state flags used:
  focus_within, disabled.
- **Exclusive groups:** none of its own: the field's `[disabled, focused, hover]` (the text box's CommonStates on
  `windows-11`) is `input`'s, the buttons' `[disabled, pressed, hover]` `button`'s; precedence: state models (S17).
- **may_be_absent:** parts: `recents_indicator` (every preset but `macos-*`: `NSSearchField`'s recents menu glyph,
  MAC-N02); `supporting_text` (every preset but `material-3`: the search bar's supporting text, MAT-N14);
  `view_header` (every preset but `material-3`: the search view's header, MAT-N15); `view` (every preset but
  `material-3`, `kde-breeze`: Material's search view, MAT-N15, and kirigami-addons' popup, K-KA-32†; `kde-breeze` by
  a rule outside `variant @ka_search_popup`, K-KA-32†); `view_divider` (every preset but `material-3`, `kde-breeze`,
  as for `view`; `kde-breeze` by a rule outside `variant @ka_search_popup`, K-KA-32†); `bar` (`macos-*`,
  `windows-11`, `kde-breeze`: `NSSearchField`, AutoSuggestBox and Kirigami's SearchField are the field alone,
  MAC-N02, W2-15, Q1.15; `ios-*` by a rule under `variant @minimal`, C-3); `scrim` (every preset but `material-3`:
  the dim behind Material's search view, MAT-X23). values: `view_placement` (every preset but `kde-breeze`:
  kirigami-addons' popup's placement, K-KA-32†); `auto_search_min_chars` (every preset but `kde-breeze`:
  kirigami-addons' `autoAccept`, K-KA-32†); `results_gap_px` (every preset but `material-3`: the contained view's,
  MAT-N15; `material-3` by its base, the rule `@ variants {@contained,@docked} cond flavor:expressive` stating it).
- **State-model source:** GNOME `_toolbars.scss` (`searchbar`), `_entries.scss`; iOS `UISearchBar`,
  `UISearchTextField`, `UISearchBar.Style`; macOS `NSSearchField` cell states; Material search-bar and search-view
  tokens; Windows AutoSuggestBox visual states; KDE Kirigami `SearchField` and kirigami-addons `SearchPopupField.qml`.
- **Today's fields:** none (`input.*`, the stand-in, stays `input`'s).
- **Notes:** in a toolbar (MAT-TA6, MAC-TL07; iOS 26's glass capsule in a toolbar or the tab bar, C-3) its rules are
  `@ context toolbar.bar`. Nested controls: the field and its buttons nest in `bar`, the same tree on every preset:
  the connector queries them in the scope `bar`'s query returns, which is `body`'s where `bar` answers `NotDrawn`
  (Nesting), so `context search_field.body` matches them everywhere and `context search_field.bar` where the bar is
  drawn (GNOME's close button, GN-2.21); Material's view holds its own field, in `context search_field.view`
  (MAT-N15). The field is an `input` in `context any search_field` (its bezel
  `input.body`, its text, its `placeholder`, the magnifier `input.icon @ variant leading`, Material's leading icon
  likewise, MAT-N14; KDE's field metrics, Q1.15–Q1.17); the clear button `button @ variant clear_field` there; iOS's
  cancel button `button @ variant @cancel context search_field` (C-3; its appearing and disappearing are the
  button's `Appear`, `Disappear` there, as a field's clear button's are); Windows' query button `button @ variant @query
  context search_field` (W2-15); Material's trailing icon buttons `button @ variant icon_only context search_field`
  and its avatar `avatar @ context search_field` (MAT-N14); GNOME's search-bar close button `button @ variant close
  context search_field.bar` (GN-2.21); iOS's search tokens `chip @ context search_field` (C-3). One tree with
  `autocomplete`: the suggestion list is an `autocomplete` in the field's context (W2-15, C-38†); a search field in a
  form row (GN-2.06) or a sidebar
  header (K-KA-41†) is placed by context. In GTK's app chooser dialog (GN-2.57†) the search bar is flat by rules
  `@ context window variant app_chooser` (the `.appchooser` window's `searchbar` extends `%searchbar-flat`, the placeholder
  `flat` extends too, `_deprecated.scss:27-31`, `_toolbars.scss:169-174`: `bar.fill`, `bar.stroke`, `bar.padding`), a
  selector that outranks the bar's backdrop rule (`_toolbars.scss:134-136`), so the bar stays unfilled under `cond
  window_inactive`; and shrunk where the window is `.csd` (`_deprecated.scss:33-35`, `_toolbars.scss:190-193`), GTK's
  own decoration on a composited display (`gtkwindow.c:3122-3136`), which the model writes `!cond not_composited
  context window variant app_chooser !variant @ssd` (GN-1.07, GN-1.08).

### selectable_label

*Typed after the tenth check (review 10, B-S1): group 6's keyed GNOME component, with KDE's row.*

- **Kind:** typed widget, because two platforms draw a selectable label's own look: GNOME (GtkLabel with `selectable`
  set: the text cursor, `G/gtklabel.c:804-806`, GN-3.67†; the caret in a focused one, `_labels.scss:2`,
  `G/gtklabel.c:1503-1515`, GN-3.39) and KDE (the I-beam over Kirigami's `SelectableLabel` and qqc2-desktop-style's
  `Label`, K3.65†; the GNOME map's cross-check counts K3.65† against GN-3.67†, gap-map-gnome's Appendix). Plain labels
  are their hosts' text parts and no other catalogue widget is a label; the other maps state no selectable-label row
  (the value phase cross-checks them, R14).
- **Available:** every preset but `macos-*`, `windows-11`, `ios-*`, `material-3` (`available = false`: their maps
  state no selectable label's own look; a row found there makes the preset declare it).
- **Parts:**
  - `body` — the root, the label as the pointer meets it: `cursor` (`Text`). The label's text keeps its host's style
    (the host's part that shows it), so the connector queries this element only for a label the application makes
    selectable, in its host's context.
  - `caret` — the insertion caret a focused selectable label draws (GN-3.39), as on `input` (the text widgets'
    caret, ADD-A24): `fill`, `min_width`, `min_height`, `corners`.
- **Widget values:** none.
- **Variants:** none.
- **Sizes and densities:** none.
- **Flags:** shared `text_selected` (GNOME: the caret's absence rule, below); state flags used: focused (the
  caret's).
- **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** parts: `caret` (`adwaita` by a rule outside `{focused}` and under `{text_selected}`, GTK drawing
  it only in a focused label with no selection; `kde-breeze`: the value phase confirms from Kirigami's
  `SelectableLabel`, a read-only `TextEdit`, and qqc2's `Label` whether a caret is drawn).
- **State-model source:** `adwaita`: `G/gtklabel.c:804-806`, `:1503-1515`, `A/w/_labels.scss:2` (GN-3.67†, GN-3.39);
  `kde-breeze`: `kirigami@eecf183a src/controls/SelectableLabel.qml:266`, qqc2-desktop-style `org.kde.desktop/Label.qml:23`
  (K3.65†).
- **Today's fields:** none.
- **Notes:** a link's text is `link` (its own cursor, `link.body.cursor`); a selectable label is not a link, so KDE's
  I-beam over selectable labels is this widget's `body.cursor`, not `link.text` (one home, R20).

### sheet

*From group 6's catalogue.*

- **Kind:** typed widget: GNOME's AdwBottomSheet (GN-2.17), iOS's UISheetPresentationController (C-12) with its
  placement per size class and with the keyboard (SF-7†, SF-8†, P3-6†), KDE's Kirigami OverlaySheet (Q2.37–Q2.40,
  K-SF23†) and kirigami-addons' BottomDrawer (K-KA-30†, K-KA-31†), Material's bottom, side and floating sheets
  (MAT-N19–MAT-N21, MAT-SF03†).
- **Available:** every preset but `macos-*`, `windows-11` (`available = false`: no such control; macOS's sheet is
  `dialog` variant `@sheet`, MAC-D03); `kde-breeze` flavour `kirigami` only (OverlaySheet, BottomDrawer).
- **Parts:**
  - `layer` — the root: the layer over the presenter the sheet is presented in, which draws nothing; it holds
    `body`, `scrim`, `bottom_bar` and `presenter_backdrop`, and every other part nests in `body`: `bindings` (empty on
    every preset: the box binds the sheet's surface set).
  - `body` — the sheet's box: `fill`, `stroke` (GNOME's inner outline, GN-2.17), `corners`, `shadow`, `padding` (the content
    padded by the bottom safe area, K-KA-30†), `margin` (Material's detached side sheet's insets from the window's
    edges, which set its height, MAT-N20), `min_width`, `max_width` (Material's on wide windows, MAT-SF03†; the
    side sheet's fixed docked width, both stated equal by the rules `@ variant @side`, MAT-N20), `max_height`
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
  - `bottom_bar` — AdwBottomSheet's bar, beside the sheet's box, GN-2.17: `fill`, `shadow`.
  - `presenter_backdrop` — the black behind the presenter that iOS 13–18's stacked-card effect scales down (C-12,
    X-SC-1): `fill`.
  - `divider` — Material's side sheet, MAT-N20: `stroke` (the line, its start edge).
- **Widget values:** `detents: Vec<(Key, Value<Px>)>` (its keys, per preset, the set a stated key is checked
  against: `ios-*` `medium`, `large`, iOS's medium and large detents, C-12; `adwaita` and `kde-breeze` the empty
  list, AdwBottomSheet and Kirigami's sheets having no detents, kirigami-addons' height limit being `body.max_height`,
  GN-2.17, Q2.38, K-KA-30†; `material-3` the keys the value phase names from its facts, MAT-N19 naming none);
  `undimmed_up_to: Value<Key>` (iOS's largest undimmed detent, C-12: one of `detents`' keys, `medium` or `large`);
  `min_top_inset_px: Value<Px>` (KDE's top floor, Q2.40, K-SF23†);
  `anchor: Value<SheetAnchor>`,
  `SheetAnchor { BottomCenter, Center, Side(Side), FullScreen }` (where the platform places the sheet:
  bottom-centred: GNOME, Material, iOS in compact width; centred: KDE's OverlaySheet, iOS in regular width; at a side,
  `Start` or `End`: Material's side sheet (docked, spanning the window's full height, its docked container height
  the window's, MAT-N20; detached, inset from the window's edges by `body.margin`); filling the window: iOS's page
  and form sheets in a compact height, SF-7†), stated by rules under the variants and restated by rules
  `@ h_size_class c` and `@ v_size_class c` (SF-7†, ADD-A20); `edge_attached_width: Value<SheetWidth>`,
  `SheetWidth { SafeArea, PreferredContent }` (an edge-attached sheet's width: the container's safe-area width, or
  the content's preferred width under `@width_follows_content`, SF-7†). It is
  not the variants' meaning twice: a variant is the kind of sheet the application chose and selects its look's
  rules; `anchor` is where the platform puts that sheet, which on iOS changes with the size class for one variant
  (`@form` centred in a regular width, at the bottom in a compact one, SF-7†), so a connector reads it from the
  theme; `swipe_to_close_px: Value<Px>` (the swipe threshold, Q2.40; iOS's interactive
  dismiss, C-12); `keyboard_avoidance: Value<Placement>` (where a sheet goes when the keyboard appears, strategy
  `KeyboardAvoid(…)`, P3-6†, to verify); `presenter_transform: Value<Transform>`, `presenter_corners: Value<Corners>` (iOS 13–18's
  stacked-card effect on the presenter, C-12); `top_padding_px: Value<Px>` (the least space AdwBottomSheet leaves
  above the sheet, by the window's height, GN-2.17: on `adwaita` the source's formula as an expression over
  `element.window_height` and the four values below, `= floor(add(values.top_padding_min, mul(sub(values.top_padding_target,
  values.top_padding_min), max(0, div(sub(element.window_height, values.top_padding_min_height),
  sub(values.top_padding_target_height, values.top_padding_min_height))))))`, libadwaita's `adw_lerp` over the
  height's progress from the first height to the second, not clamped above, truncated to whole pixels
  (`adw-bottom-sheet.c:27-30`, `:617-621`); `element.window_height` comes from the root's inputs through the scope
  (S8), so the value declares no element data; `top_padding_min_px`, `top_padding_target_px`,
  `top_padding_min_height_px`,
  `top_padding_target_height_px: Value<Px>` (the formula's four parameters, `TOP_PADDING_MIN_VALUE`,
  `TOP_PADDING_TARGET_VALUE`, `TOP_PADDING_MIN_HEIGHT`, `TOP_PADDING_TARGET_HEIGHT`, each a cited value, GN-2.17); `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear` (springs:
  GNOME, iOS; KDE's opacity and translation, Q2.40; per modal style, X-MO-4; Material's drag motion, MAT-N19),
  `@settle` (a dragged surface released coming to rest, as `sidebar`'s: Material's settle, MAT-N19; iOS's detent
  changes, X-MO-4), `@bottom_bar_show`, `@bottom_bar_hide` (the bottom bar revealed and hidden at the application's
  request, its height growing from zero and back, `adw-bottom-sheet.c:1237-1243`, GN-2.17: a motion the enter rule
  cannot state, Entry format); their steps name the parts they present (`body`, `scrim`, `presenter_backdrop`,
  `bottom_bar`), never the root, so the scrim does not slide with the sheet.
- **Variants:** typed `bottom` (GNOME, iOS, Material, kirigami-addons' BottomDrawer), `floating` (Material's floating
  sheet, MAT-N21; iOS 26's inset sheet at partial detents, to verify, C-12); keyed `adwaita` `@flush` (GN-2.17);
  `ios-*` `@page`, `@form` (SF-8†), `@edge_attached` (compact height, C-12, SF-7†), `@width_follows_content` (the
  application's choice, `widthFollowsPreferredContentSizeWhenEdgeAttached`, which the connector passes on, SF-7†),
  and the modal transition styles
  `@cover_vertical`, `@cross_dissolve`, `@flip_horizontal`, `@partial_curl`, `@zoom` (`UIModalTransitionStyle`, the
  application's choice of presentation, which the connector passes on, each with its own `presentations`, X-MO-4); `material-3` `@side`, `@modal`,
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
- **may_be_absent:** parts: `handle` (`kde-breeze` by a rule outside `cond mobile_form_factor` and
  `variant @ka_bottom_drawer`, whose drawer always draws its handle bar, Q2.39, K-KA-30†; `material-3` by a rule
  under `variant @side`, whose side sheet draws no drag handle, MAT-N20, and by a rule under `variant floating`
  where its facts show none, value phase, MAT-N21 naming none; an application that hides iOS's grabber, C-12, does
  not query it); `header` (`adwaita`, `ios-*`: AdwBottomSheet and UISheetPresentationController have no header or
  footer, GN-2.17, C-12; `material-3` by a rule outside `variant @side`: its header, holding the headline and the
  close button, is the side sheet's, MAT-N20); `title` (`adwaita`, `ios-*`, as for `header`; `material-3` by a rule
  outside `variant @side`, as for `header`); `header_separator` (`adwaita`, `ios-*`, as for `header`;
  `material-3`: MAT-N19 and MAT-N20 name no line under a header, the side sheet's line being its divider);
  `footer` (`adwaita`, `ios-*`, as for `header`; `material-3` where its facts show none, value phase: MAT-N20's
  side-sheet actions are queried in the body); `footer_separator` (`adwaita`, `ios-*`, as for `header`;
  `material-3`, as for `footer`); `bottom_bar` (every preset but `adwaita`: AdwBottomSheet's bar, GN-2.17);
  `divider` (every preset but `material-3`: the side sheet's line, MAT-N20; `material-3` by a rule outside
  `variant @side`, MAT-N20); `presenter_backdrop` (every preset but `ios-*`: iOS 13–18's stacked-card effect, C-12;
  Open items).
  values: `presenter_transform` (every preset but `ios-*`, as for `presenter_backdrop`); `presenter_corners`
  (every preset but `ios-*`, as for `presenter_backdrop`); `undimmed_up_to` (every preset but `ios-*`: iOS's
  largest undimmed detent, C-12); `keyboard_avoidance` (every preset but `ios-*`: P3-6†); `edge_attached_width`
  (every preset but `ios-*`: iOS's edge-attached sheet, SF-7†); `min_top_inset_px` (every preset but `kde-breeze`:
  only Kirigami's OverlaySheet keeps a top floor, Q2.40, K-SF23†); `top_padding_px` (every preset but `adwaita`:
  AdwBottomSheet's space above the sheet, GN-2.17); `top_padding_min_px` (every preset but `adwaita`, as for
  `top_padding_px`); `top_padding_target_px` (every preset but `adwaita`, as for `top_padding_px`);
  `top_padding_min_height_px` (every preset but `adwaita`, as for `top_padding_px`); `top_padding_target_height_px`
  (every preset but `adwaita`, as for `top_padding_px`); fields: `body.stroke` (all but `adwaita`: GNOME's inner
  outline, GN-2.17).
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
- **may_be_absent:** values: `page_gap_px` (`kde-breeze`: the spacing is AdwShortcutsDialog's, GN-2.41;
  KShortcutsDialog's tree and the Quick editor's list lay out no such pages, K-RL02†, K-KA-43†); `natural_size`
  (`adwaita`: the square layer is kirigami-addons', K-KA-43†; `kde-breeze` by a rule outside `variant @ka_quick`,
  KShortcutsDialog's size being its widgets', K-RL02†).
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
- **Available:** every preset but `ios-*` (`available = false`: no such control); `material-3` flavour `expressive`
  only (Material's split button is Expressive's, MAT-N7: the flavour `baseline` declares
  `[declare.flavor.baseline.widget.split_button] available = false`). macOS draws it as `NSComboButton` (MAC-N28†).
  `kde-breeze` both flavours: `widgets` the tool button's `MenuButtonPopup` (K1.155); `kirigami` only as
  kirigami-addons' `@ka_double_floating` (`[declare.flavor.kirigami.widget.split_button] requires_variants =
  ["@ka_double_floating"]`, S10, K-KA-27†).
- **Parts:**
  - `body` — the whole control, the root: `padding`, `corners`, `shadow` (kirigami-addons' shared shadow, K-KA-27†),
    `focus`, `bindings`.
  - `primary` *(element)*, `secondary` *(element: the arrow part; KDE's menu sub-control, K1.155; kirigami-addons'
    second action, K-KA-27†)* — the two halves:
    `fill`, `stroke` (W2-17), `corners` (the inner corners per state, Material, MAT-N7; GNOME's joined inner corners,
    GN-2.29), `shadow` (Material's halves, each with the button's elevation, MAT-N7), `interaction` (Material's state layer and press ripple, S2.12), `padding` (GN-2.29; Material's leading and trailing spaces, MAT-N7), `min_width`, `min_height` (W2-17's
    part sizes), `transitions` (kirigami-addons' state-colour change, K-KA-27†; Material's elevation motion, MAT-N7). `arrow` belongs to `secondary` and is
    queried with it. `primary_label`, `primary_icon` belong to `primary` and are queried with it.
  - `primary_label` — `text`, `transform` (Material's optical centring: the content moved off the geometric centre
    by a coefficient of its half's outer and inner corner difference, `translate_x` an expression over
    `widget.primary.corners`, MAT-X37†, ADD-A80).
  - `primary_icon` — `icon`, `transform` (as `primary_label`'s, MAT-X37†).
  - `arrow` — `glyph` (KDE's arrow colour by state, K1.155; Material's trailing icon size, MAT-N7), `transform` (its
    rotation while `open`, Material, MAT-N7; its optical centring over `widget.secondary.corners`, MAT-X37†),
    `transitions` (that rotation's). The three centred parts read their half's corners, which may resolve `Full` to
    half the smaller of the half's width and height (S8), so each declares that element data:
    `data = { primary_label = ["width", "height"], primary_icon = ["width", "height"], arrow = ["width", "height"] }`
    (S8, S10).
  - `divider` — GN-2.29, W2-17; KDE's separator line, K1.155: `stroke` (the line, its start edge), `margin` (its
    insets from the control's top and bottom, GN-2.29: a line's insets are its `margin`, S2.4), `opacity` (GNOME's
    flat divider, hidden on hover and focus, GN-2.29).
- **Widget values:** `part_gap_px: Value<Px>` (negative where the parts overlap, GN-2.29; Material's between-space,
  MAT-N7); `menu_placement: Value<Placement>` (where the menu its secondary
  half opens goes, the host's value by Nested controls; K1.155, W2-17, MAC-N28†; absent as a whole where the
  secondary half opens no menu, below).
- **Variants:** typed `primary` (GNOME's suggested look; Windows' accent style), `destructive` (GN-2.29), `flat`
  (GN-2.29), `toggle` (Windows' ToggleSplitButton: button's `toggle` meaning, R15; its state `checked`, W2-17); keyed
  `adwaita` `@opaque` (GN-2.29), `@raised` (kept raised in a toolbar, GN-1.151); `kde-breeze` `@ka_double_floating`
  (K-KA-27†); `material-3`: Compose's four forms, as `button`'s (MAT-B1) but for the filled one, which is `button`'s
  `primary` and here the base, `primary` being undeclared on `material-3`
  (`SplitButtonDefaults.LeadingButton` and `TrailingButton`, `SplitButton.kt:681-687`, `:761-767`, `:872-879`), the
  tonal one `tinted` (`TonalLeadingButton`, `TonalTrailingButton`, `:993-999`, `:1053-1060`), and keyed `@outlined`
  (`:1110-1116`, `:1170-1177`, no elevation) and `@elevated` (`:1227-1233`, `:1287-1294`), in one `exclusive_variants`
  group `[tinted, @outlined, @elevated]`; the split-button tokens state sizes and shapes only (`_md-comp-split-button-*.scss`), so the forms' colours
  and elevations are Compose's (R1.6), its `ButtonDefaults` colours and elevations (MAT-N7).
- **Sizes and densities:** `material-3` flavour `expressive`: `extra_small`, `small`, `regular`, `large`,
  `extra_large` (Material's xsmall … xlarge, MAT-N7, ADD-A62); `macos-*`: its control sizes (MAC-N28†).
- **Flags:** none of its own; state flags used, on the `primary` and `secondary` elements (each half's own states,
  the halves being *(element)*s): hover, pressed, disabled, focused, focus_visible, open (on `secondary`, its menu
  shown: rotates `arrow` and changes the trailing half's corners, Material's one trailing-button state, MAT-N7),
  checked (`toggle`); and, for the parts queried with the widget's element (`body`, `divider`), the whole control's
  hover, disabled, focused, focus_visible and checked on that element (GNOME's divider hidden while the control is
  hovered or focused, GN-2.29; WinUI's checked divider, W2-17; `body.focus`).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` on `primary` and `secondary` (each part's
  CommonStates), `checked` combining (`toggle`); others none (states combine; `material-3`'s state layers are a
  recency set, below); precedence: state models (S17). Recency sets: `material-3`: `state_layer = [focus_visible,
  hover]` on `primary` and `secondary` (Material's recency sets, above: a `Surface`'s ripple, `SplitButton.kt:702-703`;
  `pressed` combines) and `elevation = [pressed, focus_visible, hover]` on both (each half's `ButtonElevation`,
  `SplitButton.kt:709`, `:791`, `:917`, the button's `Button.kt:1703-1747`).
- **may_be_absent:** parts: `arrow` (`kde-breeze` by a rule under `variant @ka_double_floating`: DoubleFloatingButton's
  two halves are two actions, neither opening a menu, K-KA-27†), `divider` (`material-3`: the gap between its
  halves stands for it, MAT-N7; `kde-breeze` by a rule under `variant @ka_double_floating`, the pill drawing no
  line between its halves, K-KA-27†). values: `menu_placement` (`kde-breeze` by a rule under
  `variant @ka_double_floating`, as for `arrow`, K-KA-27†); fields: `body.shadow` (all but `kde-breeze`; `kde-breeze` by its base, outside
  `variant @ka_double_floating`, whose shared shadow is the one drawn, K-KA-27†), `primary.shadow` (all but `material-3`; `material-3` by a rule under `@outlined`, whose halves have no
  elevation, `SplitButton.kt:1116`, `:1177`),
  `secondary.shadow` (all but `material-3`; `material-3` by a rule under `@outlined`, as `primary.shadow`).
- **State-model source:** GNOME `_buttons.scss` (`splitbutton`); macOS `NSComboButton.Style.split`; Material
  split-button tokens; Windows SplitButton visual states; KDE Breeze's tool-button branches for `MenuButtonPopup`
  (`st:4426-4465`, `st:4987-5051`, K1.155); kirigami-addons `DoubleFloatingButton.qml`.
- **Today's fields:** none.
- **Notes:** the inner corners follow the state (Material: rest, hovered, pressed, and the trailing half's menu open:
  `primary.corners`, `secondary.corners` rules); GNOME hides the divider on hover and focus under `variant flat` (`divider.opacity`
  rules). Group 6's suggested look is `primary` (rename row 30). kirigami-addons' DoubleFloatingButton (K-KA-27†)
  stays this widget, as the cross-check's decision table places it: two joined halves with only the outer corners
  rounded, one shared shadow and each half's own states, the hovered or pressed half drawn above the other, are
  this widget's structure, which two `button` elements joined by position rules could not state (the one shadow
  under both); its secondary half opens no menu, so `arrow`, `divider` and `menu_placement` are absent under
  `@ka_double_floating` by rule.

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
- **may_be_absent:** parts: none (a page with no actions has no action area queried). fields: `body.fill`
  (`adwaita`, `kde-breeze`: AdwStatusPage and Kirigami's placeholders draw no background of their own, GN-2.13,
  Q2.30, K-KA-16†, where iOS's configuration states one, C-16; `macos-*` where its facts show none, value phase,
  MAC-N35†); the body's maximum width is `"auto"` where the platform sets no maximum (S2.11), never `Absent`.
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
    control sizes, MAC-N01), `bindings` (Q2.1's View set), `focus` (Breeze's keyboard focus frame, which K3.20 lists
    for spin boxes).
  - `increment` *(element)*, `decrement` *(element)* — the two halves: `fill` (per state, C-4, W2-01), `stroke`
    (W2-01), `corners` (GNOME's end radii, GN-2.24; AdwSpinRow's circular buttons, GN-2.05), `padding`, `margin`
    (AdwSpinRow's space outside its filled circles, `_lists.scss:257`, GN-2.05: outside the box the fill paints),
    `min_width`, `min_height` (GN-2.24, GN-2.05; KDE's arrow column, K2.3). `decrement_glyph` belongs to
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
  each implies, so the connector reads one value); `repeat_delay_ms`, `repeat_interval_ms: Value<Ms>` (the arrows'
  auto-repeat while pressed: C-4, MAC-N01, GN-3.69†, K3.76†; on the presets that declare the system pointer
  auto-repeat, `token.params.pointer_repeat` (S4.6), their base is the reference `= token.params.pointer_repeat.delay`
  and `= token.params.pointer_repeat.interval` (one home per fact, as `drag_hover_delay_ms`'s), the widget's own
  value only where its own source states another: on `macos-*` the reference (MAC-X12), unless the facts show
  `NSStepper` timing its own (MAC-N01); on `adwaita` the widget's own, GtkSpinButton's own constants, whose
  interval differs from the range's (`gtkspinbutton.c:63-67` against `gtkrange.c:73-74`, GN-3.69†); on the presets
  that declare no such group, the widget's own (UIStepper's, C-4; Qt's spin-box values, K3.76†));
  `key_repeat_interval_ms: Value<Ms>` (the interval while an arrow key is held, K3.76†; on the
  presets that repeat keys at the pointer's interval its base is the reference `= values.repeat_interval`, R20, so
  the value is never absent and a connector reads one value); `climb_after_repeats: Value<u32>` (GtkSpinButton grows its
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
  (per arrow, K2.3), pressed, disabled, focused, focus_visible, which shows Breeze's keyboard focus frame on `body`
  (K3.20).
- **Exclusive groups:** `windows-11`: `[disabled, focused, hover]` on `body` (the text box's CommonStates),
  `[disabled, pressed, hover]` on `increment` and `decrement` (the spin buttons' CommonStates); others none (states
  combine); precedence: state models (S17).
- **may_be_absent:** parts: `popup` (every preset but `windows-11`: NumberBox's compact popup, W2-01; `windows-11`
  by a rule outside `variant @compact`, W2-01); `header` (every preset but `windows-11`: NumberBox's header label,
  W2-01); `separator` (every preset but `adwaita`, `ios-*`, `macos-*`, whose rows draw the line, GN-2.24, C-4,
  MAC-N01: Breeze's arrow column and NumberBox's spin buttons have none in K2.3 and W2-01). values:
  `popup_offset_x_px` (every preset but `windows-11`, as for `popup`); `popup_offset_y_px` (every preset but
  `windows-11`, as for `popup`); `invalid_input` (every preset but `windows-11`: NumberBox's validation mode,
  W2-01); `field_gap_px` (every preset but `macos-*`: the field-to-stepper gap, MAC-N01); `climb_after_repeats`
  (every preset but `adwaita`: GtkSpinButton's climb, GN-3.69†); fields: `increment.stroke` (all but `windows-11`:
  NumberBox's spin-button border, W2-01), `decrement.stroke` (all but `windows-11`, as for `increment.stroke`).
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
  - `layer` — the root: the layer the tip is presented in (over the window, or its place in the content under
    `inline`), which draws nothing; it holds `body`, `target_frame` and `backdrop`, and every other part nests in
    `body`: `bindings` (empty on every preset: the box binds its set).
  - `body` — the tip's box: `fill` (fill or material), `stroke`, `corners`, `shadow`, `padding`, `min_width`,
    `max_width` (W2-09; K-KA-45†), `min_height` (kirigami-addons' tip, K-KA-45†), `bindings` (kirigami-addons'
    Tooltip set, K-KA-45†).
  - `arrow` — Windows' tail, W2-09; the popover tip's arrow, C-36†, MAC-N36† (one name with `popover.arrow`,
    `tooltip.arrow`, `menu.arrow`): `fill`, `stroke`, `min_width`,
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
- **Widget values:** `placement: Value<Placement>` (the tip against its target: Windows' tail placement, strategy
  `WinUiTeachingTip`, W2-09; kirigami-addons' tip, strategy `KirigamiAddonsOnboardingTip`, K-KA-45†; TipKit's
  popover tip, strategy `UiKitPopover` on `ios-*`, C-36†, and `AppKitPopover` on `macos-*`, MAC-N36†; under
  `variant inline` absent, the inline tip lying in the content where the application lays it out, so nothing places
  it); `light_dismiss: Value<bool>` (W2-09;
  `false` where the platform does not dismiss a tip on a click outside it, never `Absent`);
  `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear`, `@expand`, `@collapse` (Windows' expand
  and contract animation, W2-09: the one name every widget uses); their steps name the parts they present (`body`,
  `backdrop`, `target_frame`), never the root.
- **Variants:** typed `inline`, `popover` (TipKit's inline and popover tips, iOS and macOS); keyed `windows-11`
  `@alternate_close`, `@hero_bottom` (W2-09); `kde-breeze` `@ka_onboarding` (K-KA-45†).
- **Sizes and densities:** none (no row states a size).
- **Flags:** none of its own (its close and action buttons are `button` elements).
- **Exclusive groups:** none (the close button's CommonStates on `windows-11` are `button`'s); precedence: state
  models (S17).
- **may_be_absent:** parts: `arrow` (`ios-*`, `macos-*` by a rule under `variant inline`, TipKit's inline tip having
  no arrow, C-36†, MAC-N36†; `kde-breeze`: kirigami-addons' tip, a ToolTip, has no tail, K-KA-45† naming none);
  `top_highlight` (every preset but `windows-11`: TeachingTip's top highlight band, W2-09); `hero` (every preset
  but `windows-11`: TeachingTip's hero content, W2-09); `subtitle` (every preset but `windows-11`: TeachingTip's
  subtitle, W2-09); `title` (`kde-breeze`: kirigami-addons' tip is a ToolTip of its text and its navigation
  buttons, K-KA-45†); `image` (`kde-breeze`, as for `title`); `target_frame` (every preset but `kde-breeze`: the
  onboarding tour's frame, K-KA-45†); `backdrop`
  (every preset but `kde-breeze`: the tour's blur, K-KA-45†). fields: none (the body's minimum width is `"auto"`
  where the platform sets no minimum, no minimum beyond the content, S2.11, never `Absent`: W2-09 states
  TeachingTip's, the other rows none). values: `placement` (`ios-*`, `macos-*` by a rule under
  `variant inline`, TipKit's inline tip having no anchor, C-36†, MAC-N36†).
- **State-model source:** iOS and macOS TipKit `TipView` styles; Windows TeachingTip visual states; KDE
  kirigami-addons `OnboardToolTip.qml`, `OnboardingOverlay.qml`, `OnboardingEffect.qml`.
- **Today's fields:** none.
- **Notes:** Nested controls: the close button is `button @ variant close context teaching_tip` (W2-09; its
  alternate placement `@alternate_close`), the actions `button` elements in `context teaching_tip.action_area`. KDE's
  What's This popup (K2.31†) is `context_help`, not this widget.

### time_picker

*From group 6's catalogue.*

- **Kind:** typed widget: KDE's KTimeComboBox and QTimeEdit (K2.24†) and kirigami-addons' time row and popup
  (K-KA-22†, K-KA-23†), Material's time picker and time input (MAT-N17), Windows' TimePicker (W2-04), and the
  platform's date picker in its time mode, which the cross-check found drawn as a time picker (R14): macOS's
  `NSDatePicker` with time elements only, a text field with or without a stepper (MAC-N04; gap map macOS "time
  picker"), iOS's `UIDatePicker` in its `.time` mode, the compact pill with its popover of wheels or the inline
  wheels (C-5, C-7; gap map iOS "time picker"). A picker showing a date and a time together (macOS's date and time
  elements, iOS's `.dateAndTime` mode) stays a `date_picker`.
- **Available:** every preset but `adwaita` (`available = false`: no time picker in GNOME's rows); `kde-breeze`
  in both flavours, each its own toolkit's picker: Qt Widgets' KTimeComboBox and QTimeEdit under `cond
  flavor:widgets` (K2.24†), kirigami-addons' time row and popup under `cond flavor:kirigami` (K-KA-22†, K-KA-23†),
  as on `date_picker`.
- **Parts:**
  - `body` — the picker's box (the root): `padding`, `bindings`.
  - `field` — the picker's own trigger (Material's time picker has none: its dial or input sits in `popup`, the
    picker's dialog container, MAT-N17): `fill`, `stroke`, `corners` (iOS's pill, C-5), `focus` (W2-04), `padding`,
    `min_width`, `max_width` (W2-04). `kde-breeze` draws the trigger with another widget under its `widgets`
    flavour (Nested controls; K2.24†): under `variant with_popup cond flavor:widgets` `field` is `drawn_as = {
    widget = "combo_box", variants = ["editable"] }` (KTimeComboBox's editable combo, K1.240), under `variant
    text_field cond flavor:widgets` `drawn_as = { widget = "stepper", variants = [] }` (QTimeEdit's spin box, K2.3),
    as on `date_picker`; the parts nested in it whose place the substitute's own parts take are absent by rules
    under the same selectors (may_be_absent). Under `variant with_popup cond flavor:kirigami` the field is
    kirigami-addons' own flat button, the time half of its date and time row, in `context form.row` (K-KA-22†; its
    click opens the time popup, K-KA-23†, `ka/formcard/FormDateTimeDelegate.qml:402-421`, `:424-426`), drawn as
    this part, as `date_picker.field` is the row's date half.
  - `field_label`, in `field` — the time text (per state, W2-04; the time in the accent on iOS, C-5): `text` (one
    structure with `date_picker.field_label`).
  - `glyph`, in `field` — kirigami-addons' clock glyph before the time (K-KA-22†): `glyph` (one structure with
    `date_picker.glyph`).
  - `field_segment` *(element: an hour, minute or period segment, selected while edited on macOS)*, in `field` — one
    segment of the trigger (W2-04, MAC-N04): `fill` (macOS's selected segment), `text`, `padding`.
  - `segment_separator`, in `field` — W2-04's spacer lines: `stroke` (the line, its start edge), `opacity`,
    `margin` (its insets at the line's ends: a line's insets are its `margin`, S2.4; as
    `date_picker.segment_separator`).
  - `header` — W2-04: `text`.
  - `popup` — the picker's own flyout, holding its looping selectors (`picker_wheel` elements in `context
    time_picker.popup`, whose band and item height are `picker_wheel`'s; one name with `date_picker.popup`), and
    Material's time-picker container, whose colour, shape and elevation are time-picker tokens (MAT-N17, as
    `date_picker.popup` holds the date picker's, MAT-N16): `fill`, `stroke`, `corners`, `shadow`, `transitions`
    (Windows' flyout drawn only `{open}`, entering and leaving by these, S2.7; its open animation, W2-04).
    `kde-breeze` draws kirigami-addons' popup with another widget: under `variant with_popup cond flavor:kirigami`
    `popup` is `drawn_as = { widget = "dialog", variants = ["@ka_dialog"] }` (K-KA-23†, whose fade is the
    dialog's), so one box is drawn, as on `date_picker`.
  - `dial`, in `popup` — MAT-N17's clock dial: `fill`, `corners`, `min_width`, `min_height`.
  - `dial_selector`, in `popup` — the selector handle, MAT-N17: `fill`, `corners`, `min_width`, `min_height`.
  - `dial_track`, in `popup` — the selector track, MAT-N17: `fill`, `min_width`.
  - `dial_center`, in `popup` — the selector centre, MAT-N17: `fill`, `corners`, `min_width`, `min_height`.
  - `dial_label` *(element)*, in `popup` — MAT-N17's dial labels; the one under the selector is `selected`: `text`.
  - `selector_cell` *(element)*, in `popup` — MAT-N17's time selector cells (the dial picker's hour and minute):
    `fill`, `stroke`, `corners`, `min_width`, `min_height`, `text`, `transitions` (the `interaction` block's fades,
    MAT-N17), `interaction` (Material's state layer and press ripple, S2.12). The time input's hour and minute fields are `input` elements (Nested controls, below).
  - `period_selector` *(element)*, in `popup` — MAT-N17's AM/PM selector: `fill`, `stroke`, `corners`, `min_width`,
    `min_height`, `text`, `transitions` (the `interaction` block's fades, MAT-N17), `interaction` (Material's state layer and press ripple, S2.12).
  - `headline`, in `popup` — MAT-N17: `text`.
  - `time_separator`, in `popup` — Material's separator, MAT-N17; kirigami-addons' ":", K-KA-23†: `text`. The parts
    in `popup` are its content: they nest in it on every preset, so the popup's entering and leaving carry them, and
    under `variant with_popup cond flavor:kirigami`, where `popup` is drawn as a `dialog`, they are queried in the
    dialog's scope (`context dialog` matches them, Nesting).
- **Widget values:** `invalid_entry: Value<bool>` (an invalid entry answered with a message box, K2.24†, K1.224†;
  `false` where the platform answers none, never `Absent`);
  `placement: Value<Placement>` (the popup against the field, one name with `date_picker.values.placement`: C-5's
  popover, `UiKitPopover`; W2-04's flyout, its strategy the value phase's).
  The popup's showing and hiding are its `transitions`, as on `date_picker`, not an event.
- **Variants:** typed `vertical` (Material's vertical layout; horizontal is the base, R25.14, MAT-N17), `with_popup`
  (a field that opens a popup: KTimeComboBox's popup list, K2.24†; kirigami-addons' time row and its time popup,
  K-KA-22†, K-KA-23†; iOS's compact pill, C-5), `text_field` (QTimeEdit, macOS's text field: a segmented text field
  that opens none) and `inline` (iOS's inline wheels, C-7), the meanings `date_picker` declares, set by the
  application's choice of control (K2.24†); keyed `material-3` `@dial`, `@input` (MAT-N17); `macos-*`
  `@text_field_and_stepper` (MAC-N04, as on `date_picker`). On `kde-breeze` the flavour, not a variant, tells
  KTimeComboBox from kirigami-addons' picker, as on `date_picker`: both are `with_popup` (the field opens the
  popup), the substitutions are rules under `cond flavor:widgets` and `cond flavor:kirigami`, and an application
  asking for `with_popup` gets its flavour's picker with no keyed name of KDE's;
  `[declare.flavor.kirigami.widget.time_picker]` declares `with_popup` alone of the typed forms (`text_field` is
  QTimeEdit's, K2.24†; kirigami-addons draws no text-field time and no inline time, K-KA-22†, K-KA-23†).
  Whether macOS's graphical style with time elements alone draws its clock face alone (MAC-N04) is the value
  phase's (Open items); until then `macos-*` declares no `inline`.
- **Sizes and densities:** no sizes (no row states one); `windows-11`: density `compact` (WinUI's compact sizing dictionary, W3-39): the field's padding.
- **Flags:** none of its own; state flags used: selected (`selector_cell`, `period_selector`, `dial_label`;
  `field_segment` on `macos-*`, MAC-N04), hover, pressed (iOS's pill, C-5), focused, disabled, open (Windows' flyout,
  iOS's popover shown), focus_visible, read_only (kirigami-addons' row drops the field's fill, K-KA-22†).
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` on `field` (CommonStates); others none (states
  combine; `material-3`'s state layers are a recency set, below); precedence: state models (S17). Recency sets:
  `material-3`: `state_layer = [focus_visible, hover]` on `selector_cell` and `period_selector` (state layers;
  Material's recency sets, above: a `Surface`'s ripple, `TimePicker.kt:3089-3095`, and a `TextButton`'s, `:3018`;
  `pressed` combines).
- **may_be_absent:** parts: `dial` (every preset but `material-3`: the clock dial is Material's, MAT-N17;
  `material-3` by a rule under `variant @input`, the time input drawing no dial, MAT-N17), `dial_selector` (every
  preset but `material-3`, as for `dial`; `material-3` by a rule under `variant @input`, as for `dial`),
  `dial_track` (every preset but `material-3`, as for `dial`; `material-3` by a rule under `variant @input`, as for
  `dial`), `dial_center` (every preset but `material-3`, as for `dial`; `material-3` by a rule under
  `variant @input`, as for `dial`), `dial_label` (every preset but `material-3`, as for `dial`; `material-3` by a
  rule under `variant @input`, as for `dial`), `selector_cell` (every preset but `material-3`: Material's time
  selector cells, MAT-N17; `material-3` by a rule under `variant @input`, whose hour and minute are the `input`
  widget's elements, MAT-N17), `period_selector` (every preset but `material-3`: Material's AM/PM selector,
  MAT-N17), `headline` (every preset but `material-3`: Material's headline, MAT-N17); `popup` (`macos-*`:
  `NSDatePicker` with time elements opens none, MAC-N04; `kde-breeze` by a rule under `cond flavor:widgets`,
  KTimeComboBox's list being its combo box's own and QTimeEdit opening none, K2.24†, and by a rule `!{open}` under
  `variant with_popup cond flavor:kirigami`, where the popup is drawn as the `dialog` widget while open, K-KA-23†;
  `windows-11` and `ios-*` by a rule `!{open}`, the flyout and the popover being drawn while open, W2-04, C-5;
  `ios-*` also by a rule under `variant inline`, whose wheels lie in the body, C-7),
  `field` (`material-3`: its time picker has no trigger, its dial or input lying in the popup, MAT-N17; `ios-*` by
  a rule under `variant inline`, which has no trigger, C-7), `field_label` (`kde-breeze` by the rules that draw the
  field as a combo box or a spin box, under `variant with_popup cond flavor:widgets` and `variant text_field cond
  flavor:widgets`, whose own text takes its place, K2.24†; `material-3`, as for `field`; `ios-*` by a rule under
  `variant inline`, as for `field`), `glyph` (`windows-11`, `macos-*`, `ios-*`, `material-3`: the clock glyph is
  kirigami-addons', K-KA-22†; `kde-breeze` by a rule outside `context form.row`, where kirigami-addons' row draws
  it, K-KA-22†),
  `field_segment` (all but `windows-11`, `macos-*`: the segmented trigger is WinUI's and `NSDatePicker`'s, W2-04,
  MAC-N04), `segment_separator` (all but `windows-11`: TimePicker's spacer lines, W2-04), `header` (all but
  `windows-11`: TimePicker's header label, W2-04), `time_separator` (`windows-11`, whose columns are separated by
  its spacer lines, W2-04; `kde-breeze` by a rule under `cond flavor:widgets`, the separator being
  kirigami-addons' popup's, K-KA-23†; `macos-*`, `ios-*` where their facts show none, value phase); fields:
  `popup.stroke` (`material-3`: its container draws no outline, MAT-N17); the body's padding is stated on every
  preset, a zero inset where the picker has none, and the field's minimum and maximum widths are `"auto"` where the
  platform sets none (no limit beyond the content, S2.11; W2-04 states TimePicker's), never `Absent`;
  values: `placement` (`kde-breeze`: its popups are the combo box's list and the dialog the popup is drawn as,
  placed by those widgets, K2.24†, K-KA-23†; `macos-*`: `NSDatePicker` with time elements opens no popup, MAC-N04).
  On `material-3` the placement is stated: the time picker's dialog container, its popup, is placed
  `CenteredInHost`, as a dialog sits, and the connector builds no dialog element (a dial or an input in a dialog,
  MAT-N17, is this popup with its own tokens).
- **State-model source:** KDE `ktimecombobox.cpp`, kirigami-addons `FormDateTimeDelegate.qml`, `TimePicker.qml`,
  `TimePopup.qml`; Material
  time-picker tokens; Windows TimePicker and DateTimePickerFlyout visual states; macOS `NSDatePicker.Style` with time
  elements × control states; iOS `UIDatePickerStyle` in `.time` mode.
- **Today's fields:** none.
- **Notes:** the looping selectors (W2-04) are `picker_wheel` elements in `context time_picker.popup`, and
  kirigami-addons' tumblers (K-KA-23†) `picker_wheel` elements in `context time_picker`. kirigami-addons' date and
  time row (K-KA-22†) puts the field, drawn flat, in `context form.row` (the form's `@ka_form_card`), as on
  `date_picker`; its popup (K-KA-23†) is the `dialog` element `popup` is drawn as, in `context time_picker`
  (`dialog @ variant @ka_dialog context time_picker`: its body, scrim and action area). Nested controls: the
  flyout's accept and dismiss buttons are `button @ variant @accept / @dismiss context time_picker` (W2-04);
  Material's time input fields, with their outline, label and supporting text, are `input` elements in `context
  time_picker.body[@input]` (MAT-N17); macOS's stepper is a `stepper` in `context time_picker` (MAC-N04), and
  iOS's wheels are `picker_wheel` elements in `context time_picker.popup` (the compact pill's popover) or `context
  time_picker` (inline, C-7).

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
- **Widget values:** `durations: Vec<(Key, Value<Ms>)>` (its keys, per preset, the set a stated key is checked
  against: `adwaita` `default`, GN-2.12; `kde-breeze` `short`, `default`, `long`, Q2.51; `material-3` `short`,
  `long`, MAT-N22, MAT-X25); `natural_width_px: Value<Px>` (AdwToast's natural
  width, the width it takes where it fits, which no size limit states, GN-2.12); its maximum width is `body.max_width`
  (Q2.50) and its distance to the overlay's edges the placement's `edge_margin` (GN-2.12, Q2.50); `stack_gap_px: Value<Px>` (between
  stacked toasts, Q2.50); `content_gap_px: Value<Px>` (between the text and the action: GNOME's spacing, GN-2.12;
  Q2.50); `stack_max_height_px: Value<Px>` (Q2.50); `placement: Placement` (where the toast goes: Material's
  scaffold puts the snackbar by `MaterialScaffoldSlot(Snackbar)`, above the FAB and the bottom bar, MAT-X38†;
  AdwToastOverlay and Kirigami's notification manager at the bottom centre of their overlay, GN-2.12, Q2.50, by
  `OverlayBottomCenter`, S13);
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
- **may_be_absent:** fields: `body.stroke` (`adwaita` by its base, outside `cond contrast_high`, under which GNOME
  outlines the toast, GN-2.12); the body's shadow is drawn on every preset that declares the widget (GN-2.12, Q2.49,
  MAT-N22). values: `natural_width_px` (every preset but `adwaita`: AdwToast's natural width, GN-2.12);
  `stack_gap_px` (every preset but `kde-breeze`: the stacking is Kirigami's notification manager's, Q2.50);
  `stack_max_height_px` (every preset but `kde-breeze`, as for `stack_gap_px`); `content_gap_px` (every preset but
  `kde-breeze`, `adwaita`: GNOME's and
  Kirigami's spacing between the text and the action, GN-2.12, Q2.50); the body's maximum width is `"auto"` where the
  platform sets no maximum (S2.11), never `Absent`.
- **State-model source:** GNOME `_misc.scss` (`toast`), `adw-toast-overlay.c`; KDE `PassiveNotificationsManager.qml`;
  Material snackbar tokens and `SnackbarHost.kt`.
- **Today's fields:** none.
- **Notes:** Nested controls: the action is `button @ context toast.body`, Material's close icon `button @ variant
  close context
  toast` (MAT-N22). Group 8 placed `body` (GN-3.09) and the durations (W3-41, MAT-X25; it wrote `duration_ms`) here. W3-41's
  `UISettings.MessageDuration` is how long the shell's notifications stay, out (R1.1): `windows-11` declares no
  toast, and nothing reads it.

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
  - `body` — the editor, the root (KDE's dialog content: KEditToolBar is a `dialog` element, the editor queried in
    `context dialog.body` and the button box, Defaults, OK, Apply and Cancel, that dialog's own `action_area`,
    K-P4-10†; macOS's customization palette, whose sheet over the window is a `dialog` element `@sheet`, the editor
    queried in its context, MAC-P3-6†): `padding`, `bindings`.
- **Widget values:** none: the items parting while one is dragged in and their re-layout as they are added, removed
  or moved to the overflow happen in the toolbar, whose events `@customize` and `@items_change` they are (one home,
  R20; MAC-P3-6†, P4-10†).
- **Variants:** none (no row names a style).
- **Sizes and densities:** none (no row states a size).
- **Flags:** none of its own (the dragged items and the bar a drag hovers are the toolbar's and its buttons' elements,
  below).
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: none; fields: none (the body's padding is stated on every preset that declares the
  widget, a zero inset where the measurement shows iPadOS's editor draws none of its own, P4-10†); values: none.
- **State-model source:** KDE `kedittoolbar.cpp`, `ktoolbar.cpp` with Breeze's item-view and button branches; AppKit's
  customization palette (`NSToolbar.allowsUserCustomization`, MAC-P3-6†, measured); UIKit's customisable
  navigation-bar items (`UINavigationItem.customizationIdentifier`, P4-10†, measured).
- **Today's fields:** none.
- **Notes:** Nested controls: KDE's two lists are `list` elements in `context toolbar_editor`, the four move buttons
  between them `button @ variant icon_only context toolbar_editor` (K-P4-10†). The two lists differ (Available
  actions: its own minimum size, no drop indicator; Current actions: another minimum, a drop indicator, K-P4-10†),
  and the rules tell them apart by their order among the editor's lists, KEditToolBar's layout putting the
  available list first: `list @ pos first context toolbar_editor` and `list @ pos last context toolbar_editor`
  (`PositionMatch` among the `list` siblings), each with its own `view.min_width` and `view.min_height`, and
  `list.drop_indicator` `absent = true` under the first. The connector reports the lists' order as it does any
  siblings', so it needs no KDE knowledge and no variant. macOS's palette (MAC-P3-6†) is this
  widget: its item grid a `list @ variant grid context toolbar_editor`, its default set a `toolbar` in `context
  toolbar_editor`, its display-mode choice a `combo_box` there and its Done a `button @ variant done` there. iPadOS's
  editor (P4-10†) is this widget too: the row names its available items, its default set and its Done, which take the
  same nested elements; the row leaves its look to measure, so the iOS facts state it. The drop feedback in the window's bar is the toolbar's own elements:
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
  drag-hover switch, GN-2.15, and expand, GN-2.16); the preferences dialog's thresholds (GN-1.197, GN-2.10) are the
  dialog's own breakpoints, `dialog.values.breakpoints` under `paged`, not this widget's (`narrow` and `@wide`
  are AdwViewSwitcher's policies, which the host sets); `presentations: Vec<PresentationTransition>`:
  events `@indicator_move` with `part: indicator`, the item's part (S2.7: SelectorBar's pill slide, W2-27; Pivot's
  header slide, W2-32).
- **Variants:** typed `narrow` (the narrow policy, GN-2.15); keyed `adwaita` `@wide` (the wide policy), `@title`,
  `@sidebar`, `@inline`, `@stack` (GN-2.15, GN-2.16, GN-2.33); `windows-11` `@selector_bar` (W2-27), `@pivot` (W2-32),
  `@navigation_top` (W2-11, W1-094), on `item` `@contextual` (a tabbed command bar's contextual tab, W-RV-3†).
- **Sizes and densities:** none (no row states a size).
- **Flags:** none of its own (the needs-attention dot is a `badge`, variant `attention`, in
  `context view_switcher.item`, GN-1.41, GN-2.33); state flags used: selected, hover, pressed, disabled.
- **Exclusive groups:** `windows-11`: `[disabled, pressed, hover]` on `item` (CommonStates), `selected` combining;
  others none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `indicator` (`adwaita`: GNOME's switchers draw no indicator, GN-2.15, GN-2.16 and
  GN-2.33 naming none; `windows-11` by a rule outside `{selected}`, W2-11, W2-27, W2-32), `item_separator`
  (`adwaita`, as for `indicator`; `windows-11` by a rule outside `variant @navigation_top`, the top
  NavigationView's separator, W2-11), `title` (every preset but `windows-11`: the Pivot's title, W2-32;
  `windows-11` by a rule outside `variant @pivot`, W2-32).
  values: `drag_hover_delay_ms` (every preset but `adwaita`: GNOME's drag-hover switch, GN-2.15, GN-2.16),
  `icon_text_gap_px` (all but `adwaita`: AdwInlineViewSwitcher's image-text spacing, GN-2.16); fields: `bar.fill`
  (all but `windows-11`: the top NavigationView's pane fill, W2-11; `windows-11` by a rule outside
  `variant @navigation_top`, W2-11).
- **State-model source:** GNOME `_view-switcher.scss`, `_toggle-group.scss`, `_buttons.scss` (`stackswitcher`);
  Windows SelectorBarItem, PivotHeaderItem, NavigationViewItem (top) visual states.
- **Today's fields:** none (`segmented_control.*`, `tab.*` stand-ins stay theirs).
- **Notes:** counters and the needs-attention dot are `badge` elements in `context view_switcher.item` (GN-2.15,
  GN-1.41, GN-2.33). Nested controls: Pivot's header buttons are `button
  @ variant previous / next context view_switcher` (W2-32), the top NavigationView's "More" `button @ variant
  overflow context view_switcher` (W2-11). Windows' top NavigationView is this widget's
  `@navigation_top`; `sidebar` keeps the left display modes. Group 4 added the preferences dialog's `breakpoints`
  here (GN-1.197); the ninth check moved them to `dialog`, whose element they measure.

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
- **Widget values:** `button_order: Vec<Key>` (the navigation buttons in their order; its keys, per preset, each
  preset declaring them in `keys` (S10): `adwaita` GtkAssistant's `close`, `cancel`, `last`, `back`, `forward`,
  `apply`, GN-RL01†; `kde-breeze` the keys of QWizard's buttons in its classic layout, which the value phase lists
  from `qwizard.cpp`, K2.22); `header_bar_start: Vec<Key>` (under `@header_bar`, the buttons packed at the header
  bar's start, the others at its end, in `button_order`'s order: `adwaita` `back`, `cancel`, GN-RL01†; `kde-breeze`
  the empty list, QWizard having no header-bar layout, K2.22); `button_gap_px: Value<Px>`; `buttons_equal_width:
  Value<bool>` (GN-RL01†). Whether the buttons go in the header bar is the application's choice, the keyed
  `@header_bar` (one home: no value restates it); where each goes there is `header_bar_start`.
- **Variants:** keyed `adwaita` `@header_bar` (the buttons packed in the header bar, at its start and end as
  `header_bar_start` says; the connector sets the window's content variant `no_caption_buttons`, GN-RL01†). QWizard's classic style is
  the only one Breeze draws (K2.22), so it is `kde-breeze`'s base, not a variant.
- **Sizes and densities:** none (no row states a size).
- **Flags:** shared `current` (the highlighted step, GN-2.51); state flags used: none other.
- **Exclusive groups:** none (states combine); precedence: state models (S17).
- **may_be_absent:** parts: `step_list` (`kde-breeze`: QWizard's classic style draws no step list, K2.22; the
  sidebar is GtkAssistant's, GN-2.51), `step` (`kde-breeze`, as for `step_list`); `header` (`adwaita`: GtkAssistant
  draws no title area in its content, the current page's title being the window title, GN-RL01†); `title`
  (`adwaita`, as for `header`); `subtitle` (`adwaita`, as for `header`: a part nested in a part not drawn has its
  own permission, Nesting); `banner` (`adwaita`, as for `subtitle`); `action_area` (`adwaita` by a rule under
  `variant @header_bar`, the buttons packed in the header bar, GN-RL01†).
- **State-model source:** GNOME `gtkassistant.ui`, `gtkassistant.c`, `_deprecated.scss`; KDE `qwizard.cpp` classic
  style.
- **Today's fields:** none.
- **Notes:** Nested controls: the navigation buttons are `button` elements in `context wizard.action_area` (the
  default layout) or in `context window.title_bar` under `@header_bar`; the default one is `button @ variant primary`
  in that context, `context wizard.action_area` or `context window.title_bar` (GN-RL01†: the title bar is the
  wizard's ancestor, not its descendant, so no `context wizard` term can select a button there).

## Keyed components

One-platform kinds (R14, S5.1): a component is named by a `Key`; its parts and their fields are this catalogue's
(generated, S6.1), a preset states it as a `ComponentBase` in its bases and its rules name it by `PartRef::Component`
and `WidgetRef::Component` (S1, S5.1), and the preset's `[declare.component.<key>]` states its availability, sizes,
variants, flags and its values' types (S10). Each value is a `Value<WidgetValue>` of the type its declaration names
in S10's closed vocabulary (`values = { <name> = "<type>" }`, *Shared value types*: `WidgetValue`): a keyed map, so a value's key carries no unit suffix
(S4.1's rule for map keys; `delay: Value<Ms>`, `button_gap: Value<Px>`), the declaration's type giving the unit, and
a path or expression names it as declared (`values.delay`); its TOML value is a plain number in that unit, or an
expression. A component follows the entry format's rules for parts (its first part a container root, each part's fields
listed) and the nested-control rule (a control of a typed kind inside it is that widget in `context
component.<key>`; likewise a control that is another component of the same preset, KDE's title widget heading a page
view, is that component in `context component.<outer>`, never a copy of its parts, one home per fact, R20). Each
component states what a typed entry states (its kind, availability, parts, values, variants, sizes, flags,
exclusive groups, `may_be_absent` and state-model source), so the gate has its combination list (S17). A
component's variants and flags are keyed (S10's `[declare.component.<key>]`), even where a
typed widget has a variant of the same meaning (`title_widget`'s `@info`, `@warning`, `@error`, `fab_menu`'s
`@primary`): the typed `Variant` enum is the typed widgets'. Group 6 wrote its components as table rows, group 7 as
sections; the ninth check turned the rows into sections. Every component states the same lines: Kind, Available
(the preset, and where it matters the flavour, that declares it; the presets of other platforms declare no such
component), Parts, Widget values, its variants, sizes and flags, Exclusive groups, may_be_absent, State-model source
and Today's fields; a component adds a line of its own (Transitions, Position, Paint, Conditions, Reads) only where
these leave a fact of its rows unsaid. Group 6's `toolbar_editor` is a typed widget after the merge (three platforms
draw it), and so is `context_help` (rename row 83); the media output button is `button`'s typed `route` (rename row
82), and macOS's token field is the structure Windows' has (an `input` holding a flow `layout` of `chip` elements,
rename row 52). A `kde-breeze` component names the flavour that draws it (S10's `[declare.flavor.<f>.component.<key>]`
form, as for widgets: the other flavour declares `available = false`): `tool_box`, `dock_widget`, `mdi_sub_window`,
`title_widget`, `multi_tab_bar`, `command_bar`, `char_select`, `lcd_number`, `led` and `page_view` are flavour
`widgets` only (Qt Widgets); `delay_button` (a QML DelayButton) and `album_viewer` (kirigami-addons) are
flavour `kirigami` only. A component's `may_be_absent` item gives each name it lists its own preset list, which names
the one declaring preset and, where the absence is a rule's, that rule's selector (Entry format).

### GNOME

*From group 6:*

#### component.emoji_chooser (gnome)

- **Kind:** keyed component (GtkEmojiChooser, GTK). Row GN-2.43; one platform lists it (group 6's decision).
- **Available:** `adwaita`.
- **Parts:**
  - `body` — the root, the chooser's box: `fill`, `padding`, `bindings`.
  - `cell` *(element; states hover, focus_visible)* — one emoji: `fill`, `corners`, `text`.
  - `completion_row` *(element; states hover, focus_visible)* — a search completion row: `fill`, `text`, `padding`.
    Its section buttons are `button` elements in `context component.emoji_chooser` (Nested controls).
- **Widget values:** none.
- **Variants/sizes:** none. **Flags:** state flags only (hover, focus_visible on `cell` and `completion_row`, as
  `_emoji-chooser.scss` selects them). **Exclusive groups:** none (libadwaita's CSS combines states); precedence:
  state models (S17).
- **may_be_absent:** none beyond S6.3's drawing fields.
- **State-model source:** `A/w/_emoji-chooser.scss:1-96` (GN-2.43; read rule by rule in the value phase).
- **Today's fields:** none.

#### component.tab_overview (gnome)

- **Kind:** keyed component (AdwTabOverview, AdwTabGrid, AdwTabThumbnail; libadwaita). Rows GN-2.35, GN-3.51; one
  platform lists it (group 6's decision).
- **Available:** `adwaita`.
- **Parts:**
  - `grid` — the root, the overview's grid: `fill`, `padding`, `shadow` (libadwaita switches the grid's own
    `:drop(active)` ring off, `_tab-view.scss:300-304`, so GN-3.66†'s catch-all ring does not reach it: a rule
    `@ {drop_target}` states it `Absent`).
  - `thumbnail` *(element; states hover, selected (the current page's tab), focus_visible, dragged (a thumbnail being
    reordered), drop_target (content dragged over the thumbnail: its tinted fill and its inset ring, a `shadow`
    layer, `@ {drop_target}`, `_tab-view.scss:194-197`); keyed variant `@pinned`, a pinned tab's thumbnail with its
    own colours, GN-2.35; keyed flag `@morph_target`, Transitions)* — `fill`, `corners`, `shadow`, `stroke`,
    `margin`, `min_width`, `max_width` (AdwTabGrid's thumbnail width bounds, GN-2.35: one home, the part's fields, as
    rename rows 116 and 127 move such limits), `transitions`. The drop target look is the thumbnail's own under
    `{drop_target}`, no part of its own (one home, R20). `thumbnail_icon`, `thumbnail_title`, `controls` and
    `unpin_icon` belong to it and are queried with it.
  - `thumbnail_icon`, in `thumbnail` — the page's icon beside its title, below the card, or inside a pinned
    thumbnail's card, which shows no page picture (`A/c/adw-tab-thumbnail.ui:113-138`,
    `A/c/adw-tab-thumbnail.c:368-393`): `icon`, `opacity`, `transitions`.
  - `thumbnail_title`, in `thumbnail` — the page's title, an `AdwFadingLabel` (its overflow faded out): `text`,
    `opacity`, `transitions`.
  - `controls`, in `thumbnail` — what lies over the card: the close button, the indicator button (the page's
    indicator icon), a pinned thumbnail's unpin icon and the needs-attention dot (`A/c/adw-tab-thumbnail.ui:36-110`):
    `opacity`, `transitions`. Its opacity reaches the parts and the nested elements in it (Nesting; a nested widget in
    a plain part is queried in its scope, Entry format).
  - `unpin_icon`, in `controls` — the icon over a pinned thumbnail's card that marks it pinned, an image and no
    button (a `GtkImage` the template connects no handler to, `.tab-unpin-icon`, `A/c/adw-tab-thumbnail.ui:53-65`,
    `_tab-view.scss:250-254`): `icon`, `margin`, `min_width`, `min_height`.
  A thumbnail's needs-attention dot is `badge @ variant attention context component.tab_overview.thumbnail` (one
  structure for the dot, Nested controls), nested in `controls`; its close button (`button @ variant close`) and its
  indicator button are `button` elements nested in `controls`, styled by one set of rules, `button @ context
  component.tab_overview.thumbnail` (libadwaita's `tabthumbnail button`, `_tab-view.scss:256-288`); on a pinned
  thumbnail the unpin icon takes the close button's place (`A/c/adw-tab-thumbnail.c:368-369`). The new-tab button
  is `button @ variant add context component.tab_overview`.
- **Widget values:** `grid_gap: Value<Px>`, `columns_min`, `columns_max: Value<u32>` (declared `"integer"`),
  `drag_hover_delay: Value<Ms>` (a drag hovering a thumbnail switches to it, GN-3.51; its base `=
  token.behavior.spring_loading`, Entry format), `icon_title_gap: Value<Px>` (between `thumbnail_icon` and
  `thumbnail_title`, `.icon-title-box`'s `border-spacing`, `_tab-view.scss:246-248`), `pinned_title_margin:
  Value<Px>` (the least inset of a pinned thumbnail's icon and title from the card's sides, where the unpin icon or
  the indicator button is not wider, `PINNED_MARGIN`, `A/c/adw-tab-thumbnail.c:19`, `:317-357`), `presentations:
  Vec<PresentationTransition>` (queried with the overview: `Appear`, `Disappear`, the overview shown and hidden,
  the current page morphing between the whole view and its thumbnail's place, a crossfade under `cond
  reduced_motion`; queried with a thumbnail: `Move`, a thumbnail moved to its new place).
- **Transitions:** the thumbnail's controls fading back in after the morph (GN-2.35) follow a state, so they are no
  event (Entry format): the connector sets `@morph_target` on the current page's thumbnail from the start of the
  open or close animation that morphs the page into or out of its place to that animation's end (never under
  reduced motion's crossfade, which morphs nothing, `A/c/adw-tab-overview.c:1045-1051`, `:2217-2227`), and
  libadwaita hides the controls at once while it runs and fades them in when it ends (`A/c/adw-tab-thumbnail.c:18`,
  `:258-270`, `:722-739`). So `controls.opacity` is zero `@ {@morph_target}`, and so are `thumbnail_icon.opacity`
  and `thumbnail_title.opacity` `@ variant @pinned {@morph_target}` (only a pinned thumbnail's icon and title lie in
  the card and fade, `:261-264`); each of the three parts' `transitions` holds an `Opacity` transition at rest (the
  fade's duration and easing, GN-2.35) and none under those rules, so the controls vanish at once and fade in when
  the flag clears (S2.7: the style after the change governs). An opacity, not an absence: the hidden controls keep
  their place, and the dot and the buttons are nested widgets, whose roots are never absent (S6.3).
- **Variants:** keyed `@pinned` (on `thumbnail`). **Sizes:** none.
- **Flags:** keyed `@morph_target` (on `thumbnail`, Transitions); state flags (above; `drop_target` on a thumbnail
  while a drop is over it, and on the overview's element while a drag is over the overview).
  **Exclusive groups:** none (libadwaita's CSS combines states); precedence: state models (S17).
- **may_be_absent:** parts: `unpin_icon` (`adwaita` by a rule `!variant @pinned` on `thumbnail`: libadwaita shows
  it on a pinned thumbnail only, `A/c/adw-tab-thumbnail.c:368`, GN-2.35); fields: `grid.shadow` (`adwaita` by a rule
  `{drop_target}`, `_tab-view.scss:300-304`; a drawing field, S6.3).
- **State-model source:** `A/w/_tab-view.scss:183-304`; `A/c/adw-tab-overview.c`, `A/c/adw-tab-grid.c`,
  `A/c/adw-tab-thumbnail.c`, `A/c/adw-tab-thumbnail.ui` (GN-2.35).
- **Today's fields:** none.

#### component.dock_frame_drop (gnome)

*From group 7.*

- **Kind:** keyed component (GNOME's libpanel; R1.2 in). Rows GN-SF14†, GN-SF15†, GN-SF16†. The drop zone itself is
  drawn on KDE too (the GNOME map's cross-check counts KDE's dock drop zone K-SF18† as 2 against GN-SF14†–GN-SF16†,
  gap-map-gnome's Appendix row for K-SF18†), so GN-SF14†'s overlay is the typed `rubber_band @ variant dock_target`
  (R14: a kind two platforms draw is typed); the component keeps only what no other platform draws, the frame's own
  drop look, the five drop-position buttons and the switcher's insertion mark (cross-check: macOS, iOS, Windows,
  Material 3, "G19: KDDockWidgets' artwork is that library's own").
- **Available:** `adwaita`.
- **Parts:**
  - `frame` — the `PanelFrame` itself while a drop is over it: `shadow` (its own `:drop(active)` shadow is switched
    off here, so GN-3.66†'s catch-all rule does not apply: a rule `@ {drop_target}` states its `shadow` `Absent`),
    `padding` (the drop overlay's margin from the frame's edge, GN-SF14†). The overlay shown over the frame while a
    page is dragged over it is `rubber_band @ variant dock_target context component.dock_frame_drop.frame` (its fill,
    its ring `@ {drop_target}`, its corners and transitions are `rubber_band.body`'s there, GN-SF14†).
  - `drop_position_button` *(element; states: drop_target, the button under the drop)* — one of five buttons, its
    identity a keyed variant `@top`, `@right`, `@bottom`, `@left`, `@center`: `fill`, `corners` (the centre one
    circular, the others with a corner radius: rules by identity), `padding`, `glyph` (`ThemeIcon` per identity),
    `shadow`.
  - `switcher_indicator` — the insertion mark in a frame switcher where the dragged page would land: `fill`.
- **Widget values:** `button_gap: Value<Px>` (the gap in the grid of five buttons, which sit centred on the frame).
- **Variants:** none typed; keyed: the identities of `drop_position_button`, `@top`, `@right`, `@bottom`, `@left`,
  `@center`. **Sizes/densities:** none.
- **Flags:** state flag `drop_target`. No widget-specific flag. **Exclusive groups:** none; precedence: state models
  (S17).
- **may_be_absent:** fields: `frame.shadow` (`adwaita` by a rule `{drop_target}`, GN-SF14†).
- **State-model source:** libadwaita-family stylesheet selectors by file: libpanel `stylesheet.css`
  (`paneldropcontrols`, `:drop(active)`), per GN-SF14†..GN-SF16† (P).
- **Today's fields:** none (no dock widget in the model).

### KDE

*From group 6:*

#### component.lcd_number (kde)

- **Kind:** keyed component (QLCDNumber; Qt Widgets). Row K2.6.
- **Available:** `kde-breeze` flavour `widgets` only (the `kirigami` flavour declares
  `[declare.flavor.kirigami.component.lcd_number] available = false`, S10).
- **Parts:**
  - `frame` — the root: `stroke`, `bindings` (the palette colours its digits take).
  - `segment` — a digit's segment: `fill`, `stroke` (the raised segments' edges).
- **Widget values:** none.
- **Variants:** keyed `@outline`, `@filled`, `@flat` (QLCDNumber's segment styles, the application's choice).
  **Sizes:** none.
- **Flags:** state flags only (disabled, through the bound palette's disabled roles). **Exclusive groups:** none;
  precedence: state models (S17).
- **may_be_absent:** fields: `segment.stroke` (`kde-breeze` by a rule `variant @flat`, the flat style's segments
  having no raised edges, as `qlcdnumber.cpp` confirms in the value phase; a drawing field, S6.3).
- **State-model source:** qtbase `qlcdnumber.cpp` (K2.6; its lines verified in the value phase).
- **Today's fields:** none.

#### component.led (kde)

- **Kind:** keyed component (KLed, KWidgetsAddons; R1.2 in). Row K2.18.
- **Available:** `kde-breeze` flavour `widgets` only (the `kirigami` flavour declares
  `[declare.flavor.kirigami.component.led] available = false`, S10).
- **Parts:**
  - `body` — the root, the lamp: `fill`, `stroke`, `corners`, `bindings`. Its colour is the application's
    (`KLed::setColor`), which the connector supplies as `element.color`, passing `values.default_color` where the
    application sets none (KLed's own default): `fill` `= element.color` `@ {@on}` and `= qt_darker(element.color,
    values.dark_factor)` `@ !{@on}` (the off look is the colour darkened by KLed's dark factor), the shape and shading
    by rules per variant; the part declares its data, `data = { body = ["color"] }` (S8, S10).
- **Widget values:** `default_color: Value<Color>` (declared `"color"`: KLed's default colour), `dark_factor:
  Value<u32>` (declared `"number"`: KLed's dark factor, an integer percent, `kled.cpp:19`), both from `kled.cpp` in the value phase.
- **Variants:** keyed `@round`, `@rectangular` (shape), `@raised`, `@sunken`, `@flat` (look), the application's
  choice. **Sizes:** none.
- **Flags:** keyed `@on` (the lamp lit, set by the application); state flags used: disabled. **Exclusive groups:**
  none; precedence: state models (S17).
- **may_be_absent:** none beyond S6.3's drawing fields (every look strokes the lamp, `@flat` with a flat colour and
  `@raised`, `@sunken` with a gradient, `kled.cpp:243-246`, so no rule states `body.stroke` `Absent`).
- **State-model source:** KWidgetsAddons `kled.cpp` (K2.18; its lines verified in the value phase).
- **Today's fields:** none.

#### component.page_view (kde)

- **Kind:** keyed component (KPageView and KPageDialog, KWidgetsAddons; R1.2 in). Row K2.14.
- **Available:** `kde-breeze` flavour `widgets` only (the `kirigami` flavour declares
  `[declare.flavor.kirigami.component.page_view] available = false`, S10).
- **Parts:**
  - `body` — the root, the view's box: `padding`, `bindings`.
  - `header_separator` — the line under the page's title header: `stroke`.
  - `search_container` — the box holding the search field: `padding` (its contents margins round the field, which
    differ between the horizontal and the vertical sides, so a part's `Sides` and not one length,
    `kpageview.cpp:187-188`, K2.14).
  The page title header is KTitleWidget, `component.title_widget` in `context component.page_view` (one home for
  its level font and comment colours, R20); its icon list is a `list` and its search field an `input` in `context
  component.page_view` (Qt Widgets' search fields are line edits; `search_field` is `kirigami` only), the field
  inside `search_container`; KPageView's faces (List, Tree, Tabbed, FlatList) choose which nested widget holds the
  pages, a `list` or, under Tabbed, `tab`, so they are no variant of the component.
- **Widget values:** none (the search field's margins are `search_container.padding`, one home, R20).
- **Variants/sizes:** none. **Flags:** none (its list, field and title widget carry their own states). **Exclusive groups:**
  none; precedence: state models (S17).
- **may_be_absent:** none beyond S6.3's drawing fields (only the List and Tree faces show the search field and the
  separator line, so a page view the application gives another face has no `search_container` or
  `header_separator` and the connector does not query them, as for content, S6.3; `kpageview.cpp:168-185`,
  `:608-612`).
- **State-model source:** KWidgetsAddons `kpageview.cpp:160-220`, `kpageview_p.cpp:399-460` (K2.14).
- **Today's fields:** none.

#### component.delay_button (kde)

- **Kind:** keyed component (qqc2's DelayButton, drawn by qqc2-desktop-style; flavour `kirigami`). Row Q2.3.
- **Available:** `kde-breeze` flavour `kirigami` only (the `widgets` flavour declares
  `[declare.flavor.widgets.component.delay_button] available = false`, S10).
- **Parts:**
  - `body` — the root, the control's box holding the button and the overlay: `bindings`. The button itself is
    Breeze's tool-button panel (`elementType: "delaybutton"` draws `PE_PanelButtonTool`, Q2.3): `button @ variant
    tool context component.delay_button`, whose look `button` states (one home, R20).
  - `progress_overlay` — the highlight-tinted copy of the panel over the button, clipped to the progress fraction:
    `fill`, `corners`, `opacity`, `transitions`; `data = { progress_overlay = ["progress"] }`. No field carries its
    extent: the connector lays the part out at the extent the progress is heading for, the whole button while
    pressed and none while released, and the part's `Geometry` transition (Transitions) carries the clip there from
    the extent shown, as DelayButton's transition carries `progress` (`DelayButton.qml:48-52`). `element.progress`
    is the progress shown when the pressed state last changed, which the connector passes with the query that
    reports the change.
- **Widget values:** `delay: Value<Ms>` (declared `"duration"`: the delay before the button activates, DelayButton's
  `delay`, the platform's default, which an application layer restates).
- **Variants/sizes:** none. **Flags:** state flags used: pressed (on the component's element: the overlay's
  transition below; the nested button carries its own states). **Exclusive groups:** none; precedence: state models
  (S17).
- **Transitions:** the overlay growing while pressed and shrinking when released (Q2.3) is one transition of group
  `Geometry` in `progress_overlay.transitions`, which starts when the connector lays the overlay out at its new
  extent (Entry format; its fill and opacity do not change, so no other group runs). Its duration is the delay times
  the remaining progress while pressed and a fraction of the progress while released, `=
  mul(values.delay, sub(1, element.progress))` under `{pressed}` and `= mul(values.delay, mul(<fraction>,
  element.progress))` under `!{pressed}` (`DelayButton.qml:50`; S2.2's `values.<name>`, ADD-B12; the fraction is the
  source's, value phase), over the progress shown when the state changed: a press from rest takes the whole delay,
  and a release the fraction of it the shown progress gives.
- **may_be_absent:** none beyond S6.3's drawing fields.
- **State-model source:** qqc2-desktop-style `org.kde.desktop/DelayButton.qml:48-96`,
  `plugin/kquickstyleitem.cpp:1903-1925` (Q2.3).
- **Today's fields:** none.

#### component.tool_box (kde)

*From group 7.*

- **Kind:** keyed component (QToolBox; Qt Widgets). Row K2.9. Cross-check: GNOME 3 (expanders are the nearest),
  Windows 3, macOS 3, iOS 3, Material 3.
- **Available:** `kde-breeze` flavour `widgets` only (the `kirigami` flavour declares
  `[declare.flavor.kirigami.component.tool_box] available = false`, S10).
- **Parts:**
  - `body` — the root, the box holding the pages: `padding`, `bindings`.
  - `page_button` *(element; states hover, selected (the open page, the current choice among the pages: QToolBox's
    checked tab maps to it, S7.2), pressed, disabled, focus_visible)* — the tab row of one page: `min_width`
    (`ToolBox_TabMinWidth`).
  - `page_icon`, in `page_button` — `icon`.
  - `page_label`, in `page_button` — `text`.
  - `outline`, in `page_button` — the tab-shaped outline Breeze draws round every tab in every state
    (`renderToolBoxFrame`, `hp:743-782`): `stroke` (the frame outline colour at rest, the hover colour `@ {hover}`,
    the focus colour `@ {selected}`, a later rule, since Breeze takes no hover on the open page, `st:7475`,
    `:7493-7501`), `corners` (per-corner radii), `transitions` (the hover fade).
    The parts in `page_button` belong to it and are queried with it.
  - `page_area` — the content frame of the open page: `padding`.
- **Widget values:** `item_gap: Value<Px>` (`ToolBox_TabItemSpacing`), `tab_margin_width: Value<Px>`
  (`ToolBox_TabMarginWidth`); the tabs' minimum width is `page_button.min_width` alone (one home, R20).
- **Variants/sizes:** none. **Flags:** state flags only. **Exclusive groups:** none (Breeze's branches combine
  states); precedence: state models (S17).
- **Transitions:** the hover fade is a property transition on the outline stroke (`breezetoolboxengine.cpp`), its
  duration through the speed factor (`= mul(token.motion.durations.<key>, token.motion.speed_factor)`, S4.1).
- **may_be_absent:** none beyond S6.3's drawing fields (`outline` is drawn in every state, its colour by state:
  the outline outside `{selected}` and `{hover}` is the frame outline colour, `st:7493-7501`, not an absence).
- **State-model source:** Breeze's state branches (`st:7380-7505`, `st:2767-2799`, `hp:743-782`) and its animation
  engine (K2.9).
- **Today's fields:** none.

#### component.dock_widget (kde)

*From group 7.*

- **Kind:** keyed component (QDockWidget title; Qt Widgets). Row K2.10. Cross-check: the KDE map counts K2.10 in the
  portable "inspector/utility panel" kind (2, with Q1.66); GNOME, macOS, iOS, Material and Windows 3 for the dock
  widget itself. Decision: the panel a dock widget is, docked or floating, is that portable kind's typed home (docked,
  a pane of the main window's content; floating, `window @ variant utility`, as macOS's utility panel), but the
  dock widget's own title bar, its float and close buttons and its optional frame have no counterpart in macOS's
  utility panel or inspector column or in Kirigami's context drawer (Q1.66, `sidebar` `end`), so these pieces are a
  one-platform kind (R14) and stay this component; the appendix *Where group 6's §4.2 kinds go* lists KDE under that
  kind.
- **Available:** `kde-breeze` flavour `widgets` only (the `kirigami` flavour declares
  `[declare.flavor.kirigami.component.dock_widget] available = false`, S10).
- **Parts:**
  - `frame` — the root, the docked widget's box round its title bar and content, a container: `stroke` (the optional
    frame, `Absent` by a rule `!cond system_setting:dock_widget_draw_frame`: breezerc `[Style] DockWidgetDrawFrame`,
    read by the reader, `reader:kde:breezerc.Style.DockWidgetDrawFrame`; a read switch is a condition, ADD-A04).
  - `title_bar`, in `frame` — the title bar: `padding`.
  - `title`, in `title_bar` — `text` (elided as Breeze elides it).
  - `separator`, in `title_bar` — the line under the title: `stroke` (the line, its top edge). The float and close buttons
    (frameless glyph buttons; glyph `ThemeIcon` or the standard pixmap `SP_DockWidgetCloseButton`; states hover,
    pressed, disabled) are `button @ variant @float / close context component.dock_widget`.
- **Widget values:** `presentations: Vec<PresentationTransition>`: event `Move` (a dock widget moved aside when the
  main window's dock layout slides a gap open under a dragged one, `AnimatedDocks`, K-SF18†: the occasion and name of
  `toolbar`'s `Move`, K3.38†).
- **Variants:** keyed `@vertical_title_bar` (the application's `QDockWidget::DockWidgetVerticalTitleBar`: the title
  bar runs down the dock widget's side, its title turned along it, and Breeze draws no separator, `st:7545-7552`).
  **Sizes/flags:** none beyond state flags. **Exclusive groups:** none (Breeze's branches combine states);
  precedence: state models (S17).
- **may_be_absent:** parts: `separator` (`kde-breeze` by a rule `variant @vertical_title_bar`: Breeze draws the line
  under a horizontal title bar only, `st:7545-7552`, K2.10); fields: `frame.stroke` (`kde-breeze` by a rule `!cond
  system_setting:dock_widget_draw_frame`, K2.10).
- **State-model source:** Breeze's branches (`st:7507-7581`, `st:963-964`, `st:2120-2126`); Qt's
  `qmainwindowlayout.cpp` gap indicator (K-SF18†).
- **Today's fields:** none.

#### component.mdi_sub_window (kde)

*From group 7.*

- **Kind:** keyed component (QMdiSubWindow; Qt Widgets). Row K2.11. Windows has no MDI control (3).
- **Available:** `kde-breeze` flavour `widgets` only (the `kirigami` flavour declares
  `[declare.flavor.kirigami.component.mdi_sub_window] available = false`, S10).
- **Parts:**
  - `frame` — the root: `stroke` (the outline; the inactive outline `@ !{@active}`), `shadow` (the MDI window
    shadow, `breezemdiwindowshadow.cpp`), `bindings` (none on `kde-breeze`: the title bar's `[WM]` colours are keyed
    roles its parts refer to, below).
  - `title_bar` — `fill` (`= role.@wm_active_background`, and `role.@wm_inactive_background` `@ !{@active}`: KDE's
    `[WM]` colours, K2.11), `corners` (top-rounded).
  - `title`, in `title_bar` — `text` (its fill `= role.@wm_active_foreground`, and `role.@wm_inactive_foreground` `@
    !{@active}`).
  - `active_separator`, in `title_bar` — the line under the active title bar in the highlight colour: `stroke` (the line, its top
    edge). The caption buttons are `button` elements in `context component.mdi_sub_window` (Nested controls; only
    the window's own caption buttons are the core's exception, S5.2), their identities `button`'s typed `minimize`,
    `maximize`, `restore`, `close`, their glyphs `button.icon.glyph` there; the MDI window shadow is `frame.shadow`
    (one home, R20).
- **Widget values:** `title_margin_width: Value<Px>` (`TitleBar_MarginWidth`).
- **Variants/sizes:** none. **Flags:** `@active` (the sub-window is the active one: per element, so it is a widget
  flag and not the window
  condition). **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** parts: `active_separator` (`kde-breeze` by a rule `!{@active}`, K2.11); fields: `frame.shadow`
  (`kde-breeze` where the reader reads breezerc `[Common] ShadowSize` as none, the shadow then not drawn,
  `breezemdiwindowshadow.cpp:39-41`, K3.3; a drawing field, S6.3).
- **State-model source:** Breeze's branches (`st:8236-8361`, `hp:1581-1645`, `breezemdiwindowshadow.cpp:39-60`).
- **Today's fields:** none. The `[WM]` colours are KDE's keyed colour roles (*Colour roles*, keyed roles per
  platform), which the component's parts refer to; `[WM]` is no surface set, so nothing binds it.

#### component.title_widget (kde)

*From group 7.*

- **Kind:** keyed component (KTitleWidget, KWidgetsAddons; R1.2 in). Row K2.15. Cross-check: GNOME 3 (AdwStatusPage
  and group headers are different), Windows 3, macOS 3, iOS 4, Material 4.
- **Available:** `kde-breeze` flavour `widgets` only (the `kirigami` flavour declares
  `[declare.flavor.kirigami.component.title_widget] available = false`, S10).
- **Parts:**
  - `container` — the root: `padding`.
  - `icon` — `icon`.
  - `title` — `text` (one per level).
  - `comment` — `text`, `fill` (its look by message type, below).
- **Widget values:** none (alignment of icon and text is the layout, `layout.*`).
- **Variants:** keyed text level `@level_1`, `@level_2`, `@level_3`, `@level_4` (KTitleWidget's four levels, each
  per its keyed text style of K1.270); keyed
  message type `@plain`, `@info`, `@warning`, `@error` for `comment`, KTitleWidget's `MessageType` (`Plain`, `Info`,
  `Warning`, `Error`, `ktitlewidget.h:115-120`; it has no positive type). The source gives `@info`, `@warning` and
  `@error` one look, a style sheet naming the palette's highlighted-text and highlight colours for the comment's text
  and background, and `@plain` none (`ktitlewidget.cpp:57-80`): one rule for the three, referring to
  `role.selection.foreground` and `role.selection.background`, where Qt applies that sheet; it passes colour names
  where `palette()` takes role names, so the value phase measures what Qt draws and states the rule from it.
- **Sizes:** none. **Flags:** none. **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** none beyond S6.3's drawing fields (a widget given no icon or comment has none queried, S6.3;
  `@plain`'s comment paints no background, its `fill` `Absent`, a drawing field).
- **State-model source:** the widget's own code (`kwa ktitlewidget.cpp:30-80`; the message types,
  `ktitlewidget.h:115-120`).
- **Today's fields:** `text_scale` (partial): the level fonts map to keyed text-ramp entries.

#### component.multi_tab_bar (kde)

*From group 7.*

- **Kind:** keyed component (KMultiTabBar, KWidgetsAddons; R1.2 in). Row K2.16. Cross-check: GNOME 3 (GtkNotebook with
  side tabs is the typed tab), macOS 3, iOS 3, Windows 4, Material 4.
- **Available:** `kde-breeze` flavour `widgets` only (the `kirigami` flavour declares
  `[declare.flavor.kirigami.component.multi_tab_bar] available = false`, S10).
- **Parts:**
  - `bar` — the root: `padding`.
  - `tab_button` *(element; vertical or horizontal; states hover, pressed, checked, disabled)* — `fill`, `padding`.
  - `tab_icon`, in `tab_button` — `icon`; it belongs to `tab_button` and is queried with it.
  - `tab_label`, in `tab_button` — `text`; likewise.
  - `separator` — the line on the content side of the bar: `stroke` (the line, on the bar's content edge).
- **Widget values:** `item_gap: Value<Px>`.
- **Variants:** keyed bar edge `@left`, `@right`, `@top`, `@bottom` (the `KMultiTabBarPosition` values `Left`,
  `Right`, `Top`, `Bottom`, `kmultitabbar.h:55-60`; K2.16 marks its source lines "to verify").
- **Sizes/flags:** state flags only. **Exclusive groups:** none (Breeze's branches combine states); precedence:
  state models (S17).
- **may_be_absent:** parts: `separator` (`kde-breeze` by a rule `variant @top / @bottom`: Breeze draws the line
  beside a vertical bar only, `st:1407-1409`, K1.201). An icon-only tab's label is not queried (S6.3).
- **State-model source:** Breeze's `st:1395-1417` and the widget's code.
- **Today's fields:** none.

#### component.command_bar (kde)

*From group 7.*

- **Kind:** keyed component (KCommandBar, KConfigWidgets; R1.2 in). Row K-RL01†. Cross-check: Windows 3 (`CommandBar`
  there is the toolbar), macOS 2 (Help-menu search, other row), iOS, GNOME, Material: no palette. Its keycaps are the
  typed `keycap`. Decision: macOS's Help-menu search (MAC-N33†) finds commands too, but both searches are drawn
  with typed widgets, macOS's a `search_field` in `context menu.body` whose results are `menu.item` elements
  (MAC-N33†'s trace), KCommandBar's an `input`, a `list` and `keycap` elements (below); what KCommandBar draws of its
  own, the frame floating over the window with its drop shadow, its size taken from the window and its top-centre
  placement, has no macOS counterpart (the Help menu's box is `menu.body`, placed and sized as a menu), so these
  pieces are a one-platform kind (R14) and stay this component, as `dock_widget` and `char_select` keep theirs.
- **Available:** `kde-breeze` flavour `widgets` only (the `kirigami` flavour declares
  `[declare.flavor.kirigami.component.command_bar] available = false`, S10).
- **Parts:**
  - `frame` — the root: sunken styled-panel frame, frame forced on: `fill`, `stroke`, `padding`, `min_width`,
    `min_height`, `max_width`, `max_height`, `shadow` (the drop shadow, a `ShadowLayer` coloured by the palette's `Dark` role,
    `role.@shade_dark`). Nested controls: the frameless search field is `input @ variant flat context
    component.command_bar` (its leading `search` icon `input.icon @ variant leading`, its clear button `button @
    variant clear_field`); the tree of actions a `list` there, the cell holding the keycaps `list.cell`; the keys
    `keycap` elements.
- **Widget values:** `keycap_hmargin_extra: Value<Px>` (declared `"length"`: added per side to the focus frame
  margin, the source's constant, value phase; a component value, *Keyed components*); `placement: Placement`
  (strategy `KCommandBar`: top-centre of the window's content space, its top offset the placement's
  `edge_margin.top`, ADD-A21). KCommandBar sets its size from the space it opens in, not from its content: the
  space's width and height divided by the source's divisors, at least the source's minimum size (K-RL01†). So both
  `frame.min_width` and `frame.max_width` are `= max(div(element.window_width, <divisor>), <minimum width>)`, and
  `frame.min_height` and `frame.max_height` `= max(div(element.window_height, <divisor>), <minimum height>)`, the
  divisors and minimums the source's (`kcommandbar.cpp:712-731`, value phase); equal minimum and maximum fix the
  size, so the action tree cannot grow it (one home, rename row 116). The space is the main window KCommandBar is
  opened over, `element.window_width` and `element.window_height`: root inputs, which reach the part through the
  scope, so the part declares no `data` for them (S8: the root's inputs are no `Element.data`); the value phase
  confirms from the source that it measures the window and not its content area, which no root input carries (S8).
- **Variants/sizes/flags:** none (the action rows' states are `list`'s: hover, selected). **Exclusive groups:** none;
  precedence: state models (S17).
- **may_be_absent:** none.
- **State-model source:** the widget's code (`kcommandbar.cpp:158-168`, `:299-387`, `:417-440`, `:473-476`,
  `:572-605`, `:712-731`).
- **Today's fields:** none.

#### component.char_select (kde)

*From group 7.*

- **Kind:** keyed component (KCharSelect, KWidgetsAddons; R1.2 in). Row K-RL04†. Cross-check: macOS 2 and iOS 3 (the
  Character Viewer and the keyboard are system-drawn, R1.1), Windows 3, GNOME 3.
- **Available:** `kde-breeze` flavour `widgets` only (the `kirigami` flavour declares
  `[declare.flavor.kirigami.component.char_select] available = false`, S10).
- **Parts:**
  - `table` — the root, the view: `fill` (from the `View` surface's background), `bindings`.
  - `cell` *(element; states hover, selected, focus_visible)* — one character, centred: `fill`, `text`, `focus`,
    `min_width`, `min_height` (its size by the expressions below, its data declared `data = { cell = ["max_char_width",
    "font_x_height", "font_height"] }`, measured on the cell's own font through the toolkit's font metrics, S8); the
    missing-glyph cell is
    `cell` under the widget flag `@glyph_missing`.
- **Widget values:** none of its own: the cell's size is `cell.min_width` and `cell.min_height`, expressions of
  `kcharselect.cpp:255-310`. Width:
  `= add(max(max(min(element.max_char_width, mul(element.font_x_height, <max factor>)), mul(element.font_x_height, <min factor>)), element.font_height), mul(2, <text margin>))`,
  where `element.max_char_width` is the font's widest glyph, KCharSelect's `QFontMetrics::maxWidth` (S2.2, ADD-A98),
  `element.font_x_height` the font's x-height, `element.font_height` its height (ascent plus descent; `line_height`
  is the line spacing), and the text margin the style's focus-frame margin plus the source's constant
  (`token.params.layout`). Height, the source's non-Windows branch, which `kde-breeze` follows (its `Q_OS_WIN`
  branch builds for Windows alone, `kcharselect.cpp:298-302`):
  `= max(mul(element.font_x_height, <factor>), max(<minimum>, add(element.font_height, <constant>)))`. The factors and
  constants are the source's, value phase.
- **Variants/sizes:** none. **Flags:** `@glyph_missing` (the font lacks the glyph or the cell is empty: the cell is
  filled with the `Button` surface's background, else the `View`'s). **Exclusive groups:** none; precedence: state
  models (S17).
- **may_be_absent:** none.
- **State-model source:** the widget's own paint code (`kcharselect.cpp:115-131`, `:255-310`, `:1089-1121`).
- **Today's fields:** none.

#### component.album_viewer (kde)

*From the kirigami-addons rows (K-KA-34†), traced in the merge.*

- **Kind:** keyed component (kirigami-addons' `AlbumMaximizeComponent`, R1.2 in; K-KA-34†): a full-window media viewer
  above everything, on its own overlay layer. No other platform's file lists an in-application media viewer.
- **Available:** `kde-breeze` flavour `kirigami` only (the `widgets` flavour declares
  `[declare.flavor.widgets.component.album_viewer] available = false`, S10).
- **Parts:**
  - `body` — the root: the viewer's layer: `bindings`.
  - `scrim` — the dim behind it: `fill`.
  - `header` — the header bar in the alternate background: `fill`, `padding`.
  - `header_separator`, in `header` — `stroke` (the line, its top edge).
  - `title`, in `header` — its own text style: `text`.
  - `subtitle`, in `header` — `text`.
  - `footer` — the caption footer, height-limited: `fill`, `padding`, `max_height`.
  - `footer_separator`, in `footer` — `stroke` (the line, its top edge).
  - `image` — the shown image with its inset: `transform`, `transitions` (its zoom and rotation animate, group
    `Transform`). Its close button is `button @ variant close context component.album_viewer`, its actions `button`
    elements; previous and next are `button @ variants {previous,@ka_floating}` and `button @ variants {next,@ka_floating}` in context
    (the typed identities, Nested controls); the loading indicator a `spinner` in
    context; video controls the `media_controls` variant `@ka_album` (K-KA-35†).
- **Widget values:** `image_inset: Value<Px>`.
- **Variants/sizes/flags:** none. **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** none (an image given no subtitle or caption has none queried, S6.3).
- **State-model source:** the component's QML as K-KA-34† cites it (`ka/components/AbstractMaximizeComponent.qml`,
  `ka/components/AlbumMaximizeComponent.qml`, `ka/components/ImageMaximizeDelegate.qml`).
- **Today's fields:** none.

### macOS

#### component.rule_editor (macos)

*From group 7.*

- **Kind:** keyed component (`NSRuleEditor`, `NSPredicateEditor`). Row MAC-N09. Cross-check: all other platforms 3
  (macOS table only).
- **Available:** `macos-*`.
- **Parts:**
  - `body` — the root: `fill`, `stroke`, `padding`. Nested controls: the criteria menus are `combo_box` elements
    (their base pop-up look), the value field an `input`, the round plus and minus buttons `button @ variant
    circular`, each in `context component.rule_editor`.
  - `row` *(element)* — a criteria row, with alternating stripe fills by position: `fill`, `padding`, `min_height`.
  - `nested_row_area` — the indented area of nested rows: `fill`, `padding`.
- **Widget values:** `indent_per_level: Value<Px>`.
- **Variants:** none. **Sizes:** declared as a component's are (`[declare.component.rule_editor] sizes`, S10): the
  AppKit control sizes NSRuleEditor's rows draw, each a `ControlSize` slot, which the value phase names from the facts
  (MAC-N09).
  **Flags:** state flags only. **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** none (the last row's remove button is disabled, not absent).
- **Position:** the stripes are rules `@ pos parity odd` / `@ pos parity even` on `row` (ADD-B06).
- **State-model source:** AppKit control states and `NSRuleEditor` row and nesting mode enums (MAC-N09: M).
- **Today's fields:** none.

#### component.touch_bar (macos)

*Written for the seventh check.*

- **Kind:** keyed component (AppKit's `NSTouchBar` items and `NSScrubber`). Row MAC-N26. The application configures the
  items its Touch Bar shows and AppKit draws them in the application's process, so they are what the application
  configures for the system to draw with (R1.1's in-test); only macOS has them.
- **Available:** `macos-*` (the Touch Bar Macs; macOS 26 on the supported Intel Macs, MAC-N26); a Mac without a Touch
  Bar shows none, so an application draws no item there and nothing is queried (as content, S6.3).
- **Parts:**
  - `bar` — the root: the application's area of the Touch Bar, holding its items: `fill`, `padding`.
  - `item` *(element)* — a Touch Bar item (a button, a group's member, a popover item): `fill`, `corners`, `padding`,
    `min_width`, `min_height`, `text`, `icon`. A control of a typed kind in it (a slider, a segmented control) is that
    widget in `context component.touch_bar` (Nested controls).
  - `scrubber` — an `NSScrubber`, the scrolling strip of items: `fill`, `corners`, `padding`.
  - `scrubber_item` *(element)*, in `scrubber` — one scrubber item: `fill`, `corners`, `text`, `icon`.
  - `scrubber_selection`, in `scrubber_item` — the scrubber's selection and highlight overlays
    (`NSScrubberSelectionStyle`): `fill`, `stroke`, `corners`; it belongs to `scrubber_item` and is queried with it,
    so its `!{selected}` rule tests the item's own state.
- **Widget values:** `item_gap: Value<Px>`; `scrubber_item_gap: Value<Px>`.
- **Variants/sizes:** none. **Flags:** state flags only (pressed, selected on `scrubber_item`, disabled).
  **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** parts: `scrubber_selection` (`macos-*` by a rule `!{selected}` on `scrubber_item`, as AppKit
  draws it on the selected item only, MAC-N26).
- **State-model source:** AppKit `NSTouchBarItem`, `NSScrubber` and `NSScrubberSelectionStyle`, measured with the
  Touch Bar simulator (MAC-N26: to verify).
- **Today's fields:** none.

### Windows

#### component.semantic_zoom (windows)

*From group 7.*

- **Kind:** keyed component (WinUI `SemanticZoom`). Row W2-34. Cross-check: all other platforms 3, iOS 2 maps the word
  "zoom" to the window zoom button (another row).
- **Available:** `windows-11`.
- **Parts:**
  - `body` — the root, the zoomable view's box: `bindings`.
  The view's own zoom-out control is `button @ variant @zoom_out context component.semantic_zoom` (Nested
    controls: a button to the user; its fill, stroke, glyph and text per state are the button's there, W2-34).
- **Widget values:** `presentations: Vec<PresentationTransition>`: event `@zoom` (zoom in and out between the views),
  its kind the template's animation as a `PresentationKind` (`Keyed { … }` with a declared key where it is none of the
  typed kinds).
- **Variants/sizes/flags:** none. **Exclusive groups:** none of its own (the zoom-out button's CommonStates are
  `button`'s); precedence: state models (S17).
- **may_be_absent:** none.
- **State-model source:** WinUI visual state groups by template (`dxaml generic.xaml`).
- **Today's fields:** none.

#### component.key_tip (windows)

*From group 7.*

- **Kind:** keyed component (WinUI `KeyTip`, access-key badge). Row W2-37. Cross-check: GNOME, KDE, macOS, iOS,
  Material 3 (all underline mnemonics or draw none).
- **Available:** `windows-11`.
- **Parts:**
  - `badge` — the root, the access-key badge (W2-37): `fill`, `stroke` (its paint and thickness), `corners`,
    `padding`.
  - `text` — the badge's key letters: `text` (text style, foreground; one home for the letters' style, R20).
- **Widget values:** `placement: Placement` (strategy `WinUiKeyTip`; its offset from the target is the placement's
  `gap`, ADD-A21); no `presentations`, as W2-37 states no presentation transition.
- **Variants/sizes/flags:** none. **Exclusive groups:** none; precedence: state models (S17). High contrast and theme dictionaries are rules under the conditions.
- **may_be_absent:** none.
- **State-model source:** WinUI theme dictionaries per theme and high-contrast (`generic.xaml:1010-1012`).
- **Today's fields:** none.

#### component.tabbed_command_bar (windows, Windows Community Toolkit)

*From group 7.*

- **Kind:** keyed component (CTK `TabbedCommandBar`; R1.2 in). Row W-RV-3†. Other platforms: no ribbon (all 3 or
  absent).
- **Available:** `windows-11`.
- **Parts:**
  - `body` — the root: `padding`, `bindings`. Nested controls: the tab row is a `view_switcher` variant
    `@navigation_top` in `context component.tabbed_command_bar` (its look W2-11; a contextual tab its `item` under
    the keyed `@contextual`, text colour in accent; states rest, hover, pressed, disabled, selected,
    `{selected,hover}`, `{selected,pressed}`); the command panel a `toolbar` there (its buttons label to the right of
    their icons, `button.values.label_position` in context).
  - `content_border` — the border holding the tab's command bar: `fill`, `stroke` (its thickness is the stroke's
    width), `corners`; under `cond contrast_high` the window colour and a transparent stroke.
  - `element_container` — each command bar's element container: `padding` (its margin, W-RV-3†).
- **Widget values:** `presentations: Vec<PresentationTransition>`: event `@indicator_move` (the selection moving
  to another tab, the occasion and name `view_switcher`'s and `tab`'s use), a step on part `content_border`, kind
  `Keyed { key: pop_in }` (WinUI's `PopInThemeAnimation` on the content).
- **Variants:** none of its own (the contextual tab is `view_switcher`'s keyed `@contextual`). **Sizes/flags:** none.
  **Exclusive groups:** none of its own (the tab items' `[disabled, pressed, hover]` are `view_switcher`'s);
  precedence: state models (S17).
- **may_be_absent:** none.
- **State-model source:** WinUI visual states by template (`TabbedCommandBarItem.xaml:9-24`, `:83-89`).
- **Today's fields:** `toolbar.*`, `tab.*` (neither has a tab row joined to a command panel).

#### component.casting_picker (windows)

*From group 7.*

- **Kind:** keyed component (`CastingDevicePicker`; the application sets its `Appearance`, the system draws the list:
  R1.1 "configures for the system to draw"). Row W-P4-18†. The button that opens it is `button @ variant route`. The
  list is Windows' alone (macOS's device popover is the system's, R1.1), so it stays keyed.
- **Available:** `windows-11`.
- **Parts:**
  - `picker` — the root, the flyout surface (W-P4-18†: fill, radius, shadow to measure; the appearance's background):
    `fill`, `corners`, `shadow`, `padding` (the list's inset).
  - `title` — the picker's title (the appearance's foreground): `text`.
  - `item` *(element)* — a casting device; states hover, selected; the selected item is `item` `@ {selected}`
    (W-P4-18†: item height to measure; the appearance's background, foreground and their selected twins): `fill`,
    `text`, `min_height`.
- **Widget values:** `placement: Placement` (strategy `WinUiFlyout`, `Show(Rect, Placement)`: WinUI's placement
  mode is the params' `side` and `align`, S13). The appearance's colours (`DevicePickerAppearance`, which the
  application may set) are bases of the parts: `BackgroundColor`, "the background color of the picker UI",
  `picker.fill`; `ForegroundColor` the fill of `title.text` and `item.text`; `SelectedBackgroundColor` and
  `SelectedForegroundColor`, "for a device that is selected", `item.fill` and `item.text`'s fill `@ {selected}`.
  What `AccentColor` ("the accent color of the picker UI") and `SelectedAccentColor` ("the accent color for a
  device that has been selected") paint, the sources in hand do not say (Learn, `DevicePickerAppearance`, updated
  2025-11-21): the value phase finds it in W-P4-18†'s capture and names the part and field here, adding a part
  where none of these carries it, or records that the picker leaves them unused (Open items).
- **Variants/sizes/flags:** none. **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** none (a picker given no title has none queried, S6.3).
- **Reads:** `reader:windows:DevicePickerAppearance getters` (R, to verify whether they return the system defaults).
- **State-model source:** WinUI visual states; the `Appearance` object.
- **Today's fields:** none.

### iOS

#### component.context_menu_preview (ios)

*From group 7.*

- **Kind:** keyed component (UIKit `UIContextMenuInteraction` preview). Row C-13. Cross-check: macOS 3, Windows 3, KDE
  3, GNOME 3, Material 3.
- **Available:** `ios-*`.
- **Parts:**
  - `body` — the presentation over the window (the root): `bindings`.
  - `preview` — the lifted view: `transform` (scale), `corners` (clipping path), `shadow`.
  - `scrim` — the blur and dim over everything else (one name with the other scrims, rename row 16): `fill` (a
    `Material` paint of the platform plus a tint layer), `opacity` (it fades in on `Appear` and out on `Disappear`).
- **Widget values:** `presentations: Vec<PresentationTransition>`: events `Appear` and `Disappear` (the preview
  lifting and dismissing; timing a `Spring`), whose steps name the parts they present, never the root (as the
  overlays', Entry format): a `Scale` step on `preview` and a `Fade` step on `scrim`, so the scrim does not scale with
  the preview; `placement: Placement` (strategy `UiKitContextMenu`: the menu next to the preview,
  ADD-A21).
- **Variants:** none. **Sizes:** none. **Flags:** none. **Exclusive groups:** none; precedence: state models (S17).
  **Conditions:** `cond reduced_transparency` (material to
  opaque) and `cond reduced_motion` as rules.
- **may_be_absent:** none (Open items).
- **State-model source:** UIKit appearances and the interaction's preview parameters.
- **Today's fields:** none. Reader: `readers (R18)`.

#### component.keyboard (ios)

*From group 7.*

- **Kind:** keyed component. The keyboard itself is drawn only by the system (out, R1.1); the application's part is
  what it configures: the appearance and the input accessory it draws. Row C-23.
- **Available:** `ios-*`.
- **Parts:**
  - `input_accessory` — the root, the keyboard toolbar: `fill` (colour or material), `padding`. Nothing for the keys.
- **Widget values:** `appearance: Value<Key>` (declared `{ key = ["default", "light", "dark"] }`:
  `UIKeyboardAppearance`, set by the application so that the keyboard matches the theme; following the scheme is a
  rule `@ cond scheme:dark` stating `dark`);
  `presentations: Vec<PresentationTransition>`: events `Appear` and `Disappear` (the keyboard shown and hidden; kind
  `Slide`), their duration and timing the preset's, measured from the keyboard notifications' values in the value
  phase; the iOS reader confirms them once when it reads the system (R18), never per notification, so showing the
  keyboard is no theme change (S4.3).
- **Variants/sizes/flags:** none. **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** none.
- **Reads:** the keyboard's duration and curve, confirmed once by the reader: `readers (R18)`.
- **State-model source:** UIKit appearances.
- **Today's fields:** none.

#### component.document_launch_view (ios)

*From group 7.*

- **Kind:** keyed component (`UIDocumentViewController.LaunchOptions`, iOS 18+). Row C-44†. Cross-check: GNOME (status
  page), Windows, KDE 3.
- **Available:** `ios-*`: `ios-13` declares it too, as it describes the newest release of its generation, iOS 18,
  where the launch options first appear (R1.5, C-44†); C-44†'s "26 to verify" is about its look on `ios-26`, not its
  availability.
- **Parts:**
  - `background` — the root: `fill`, `padding`. Nested controls: the primary and secondary actions are `button`
    elements (with variants) in `context component.document_launch_view`.
  - `title`, `subtitle` — the launch screen's title and subtitle: `text`.
  - `foreground_accessory`, `background_accessory` — the accessory views: `icon`, `padding`.
  - `browser` — the document browser frame it opens: `fill`, `corners`.
- **Widget values:** none stated. **Variants/sizes/flags:** none. **Exclusive groups:** none;
  precedence: state models (S17).
- **may_be_absent:** none (an accessory or browser the application's configuration sets none of is not queried,
  S6.3).
- **State-model source:** UIKit configuration objects and control states (`UIDocumentViewController`'s launch
  options, measured in the value phase).
- **Today's fields:** none. Reader: `readers (R18)`.

#### component.activity_rings (ios)

*From group 7.*

- **Kind:** keyed component (HealthKitUI `HKActivityRingView`). Row C-46†.
- **Available:** `ios-*`.
- **Parts:**
  - `body` — the root, the rings' box: `padding`.
  - `ring` *(element)* — an arc, a band along the circle (Entry format), its identity a keyed variant: `fill` (its
    paint flat or a gradient, C-46†). `track` belongs to it. No field carries how far the arc sweeps: the connector
    lays each ring's arc out over its `element.progress` (the ring's progress, the application's data, which the
    connector supplies per ring), as it lays out `progress_bar.indicator` over the bar's value.
  - `track`, in `ring` — that ring's background ring: `fill`.
- **Widget values:** `ring_gap: Value<Px>`; `stroke_width: Value<Px>` (the rings' width); `cap: Value<LineCap>`
  (the arc's ends, declared `"enum:LineCap"`, S10); each ring's progress is element data (`element.progress`), not
  theme.
- **Variants:** keyed `@move`, `@exercise`, `@stand` (three rings with an Apple Watch paired, the Move ring only
  without one: the data decides which are drawn, so none is `may_be_absent`). **Sizes:** none.
- **Flags:** none (the rings hold no interaction state). **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** none beyond S6.3's drawing fields.
- **Paint:** ring colours may be gradients (C-46†: "gradient or flat: M-r"): see ADD-A34, ADD-B02.
- **State-model source:** the view's own configuration (HealthKitUI); light and dark appearances.
- **Today's fields:** none. Reader: `readers (R18)`.

#### component.large_content_viewer (ios)

*Written in the gap fixes.*

- **Kind:** keyed component: the Large Content Viewer, the HUD UIKit presents on a long press of a bar item to show
  its symbol and title at an accessibility size (`UILargeContentViewerInteraction`, iOS 13). It is in by R1.1: the
  interaction is a `UIInteraction` on the application's own view, so UIKit draws the viewer in the application's
  process, and a toolkit drawing its own bars draws its own equivalent. Row C-26.
- **Available:** `ios-*`.
- **Parts:**
  - `body` — the HUD (the root): `fill` (its material), `corners`, `padding`, `min_width`, `min_height`.
  - `icon` — the item's symbol at the large size: `icon`.
  - `label` — the item's title: `text`.
- **Widget values:** `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear`.
- **Variants/sizes/flags:** none. **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** none (an item that gives no icon or label has none queried, S6.3).
- **State-model source:** UIKit's interaction (measured, C-26).
- **Today's fields:** none. Reader: `readers (R18)`.

#### component.editing_hud (ios)

*Written in the gap fixes.*

- **Kind:** keyed component: the banner UIKit shows at the top after a three-finger editing gesture, naming the
  action (undo, redo, cut, copy, paste). It is in by R1.1: it answers the first responder's editing interaction, which
  each responder configures (`UIResponder.editingInteractionConfiguration`, `.default` or `.none`, iOS 13), so UIKit
  draws it in the application's process; a toolkit with its own text editing draws its own equivalent. Row C-49†.
- **Available:** `ios-*`.
- **Parts:**
  - `body` — the banner (the root): `fill` (colour or material), `corners`, `padding`.
  - `label` — the action's name: `text`.
- **Widget values:** `presentations: Vec<PresentationTransition>`: events `Appear`, `Disappear`.
- **Variants/sizes/flags:** none. **Exclusive groups:** none; precedence: state models (S17).
- **may_be_absent:** none (the component is drawn only while it shows).
- **State-model source:** UIKit's editing interaction (measured, C-49†; whether `.none` removes the banner is to
  verify).
- **Today's fields:** none. Reader: `readers (R18)`.

### Material

*From group 6:*

`fab_menu` is Material 3 Expressive's (MAT-N3): flavour `expressive` only, the default flavour declaring
`[declare.flavor.baseline.component.fab_menu] available = false` (S10), so `flavor:baseline` answers `NotAvailable`.

#### component.fab_menu (material)

- **Kind:** keyed component (Material 3 Expressive's FAB menu). Row MAT-N3.
- **Available:** `material-3` flavour `expressive` only (the `baseline` flavour declares
  `[declare.flavor.baseline.component.fab_menu] available = false`, S10; above).
- **Parts:**
  - `body` — the root, the menu's box: `padding`.
  - `item` *(element; states hover, pressed, focus_visible)* — a menu item: `fill`, `corners`, `padding`, `shadow`,
    `min_height` (its height, MAT-N3), `transitions`, `interaction` (Material's state layer and press ripple, S2.12). `item_label` and `item_icon` belong to it and are
    queried with it.
  - `item_label`, in `item` — `text`.
  - `item_icon`, in `item` — `icon`.
  Its close button is `button @ variant close context component.fab_menu`.
- **Widget values:** `item_gap: Value<Px>` (declared `"length"`), `presentations: Vec<PresentationTransition>`
  (declared `"presentations"`: events `Appear`, `Disappear`, the menu opening from the FAB and closing, kind `Morph`).
- **Variants:** keyed `@primary`, `@secondary`, `@tertiary` (its colour styles, MAT-N3). **Sizes:** none.
- **Flags:** state flags only (above). **Exclusive groups:** none (its state layers are a recency set); precedence:
  state models (S17). Recency sets: `state_layer = [focus_visible, hover]` on `item` (Material's state layers;
  Material's recency sets, above: a `Surface`'s ripple, `FloatingActionButtonMenu.kt:350`, `:381`; `pressed`
  combines).
- **may_be_absent:** none beyond S6.3's drawing fields.
- **State-model source:** `latest/_md-comp-fab-menu.scss:22-50` and the per-style close-button and container token
  files of the primary, secondary and tertiary styles (MAT-N3).
- **Today's fields:** none.

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
| `Popover` | popovers, menus, flyouts | libadwaita `popover_*` [GN-1.182]; on `ios-*` the elevated backgrounds, bound by a popover's box, `body` (R5.2, D-14; UIMenu's box is drawn with a material, not the elevated level, so `menu.body` binds no elevated set, as its entry says) |
| `Dialog` | dialogs and sheets | libadwaita `dialog_*`; on `ios-*` the elevated backgrounds, bound by a dialog's or sheet's box, `body` (R5.2, D-14) |
| `Tooltip` | tooltips | KDE Tooltip (Q3.1); Kirigami swaps it for `Complementary` (Q1.37) |
| `Selection` | the selected item, and the highlight a platform draws in the selection colours (a menu item's, which is `Focused` or `Hover`, never `Selected`, S7.2); KDE Selection set | every preset (S3); KDE Selection (Q3.1), every platform's list selection |
| `Complementary` | KDE's inverse-scheme set: passive notifications, tooltips in Kirigami | KDE only [K3.54, Q3.1] |
| `Osd` | libadwaita `.osd`: toasts, OSD toolbars and their accent overrides | libadwaita [GN-3.09, GN-3.17] |

Thumbnail and overview colours of libadwaita (`--thumbnail-*`, `--overview-*`) are a background and a foreground only
and no part binds them as a surface: they are keyed roles (below), not sets [GN-1.276].

A preset declares the sets it fills in `[declare] surface_sets = [...]` (ADD-A48). A binding to, or a
reference through, an undeclared set is a validation error when the layer is added (S12), so resolution never meets
one (R2.8); a palette's role or set the current preset does not declare is skipped, not refused (S12, S14). Per
platform:

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
| `background_alternate` | alternate row/band fill (KDE `BackgroundAlternate`) | may_be_absent on every preset but `kde-breeze` (`adwaita`: libadwaita has no even or odd row rule, gap-map-gnome's §2.15 note; `windows-11`: its alternate row colour has no WinUI source, gap-map-seeds; `ios-*`: iOS draws no alternation, L-18; `material-3`: no such `md.sys.color` role, MAT-X1; `macos-*`: no row states a set's alternate colour, and a `macos-*` set whose lists alternate, `list`'s `alternating_rows`, may still state it); `kde-breeze` states it in every set (K3.54) | Q3.13, K3.54 |
| `foreground` | primary text and glyph colour | required | all |
| `foreground_inactive` | the platform's secondary (muted) text level | required | KDE `ForegroundInactive`; today's `muted_color` |
| `foreground_active` | KDE `ForegroundActive`, used as the information colour | may_be_absent on every preset but `kde-breeze` (a KColorScheme role no other platform has) | K3.54, Q3.5 |
| `link`, `link_visited` | link text, visited link text | `link` required; `link_visited` may_be_absent on `macos-*`, `windows-11`, `ios-*`, `material-3` (no visited state: macOS draws no visited link, gap-map-macos's §2.28 note; WinUI's hyperlinks have none, gap-map-windows' `:1695` note; `link`'s state-model sources for `ios-*` and `material-3` name none) | K3.54, GN-1.265 |
| `negative`, `neutral`, `positive` | the set's status text colours (KDE `ForegroundNegative`, `ForegroundNeutral`, `ForegroundPositive`); a platform without per-set values states `negative = role.global.status.error.standalone`, `neutral = role.global.status.warning.standalone`, `positive = role.global.status.success.standalone` (`windows-11`, which has no standalone status colour, the `fill` instead) (KDE's neutral is a warning, rename row 86; `StatusRoles`' own `neutral` is another colour) | `negative` required; `neutral` may_be_absent on `material-3`, `positive` may_be_absent on `material-3` (the status each references is absent there: no warning or success role, below) | K3.54, Q3.10; gap-map-material's colour-role note (no warning or success role) |
| `decoration_focus`, `decoration_hover` | KDE's focus and hover decoration colours of the set | `decoration_focus` may_be_absent on every preset but `kde-breeze`, `decoration_hover` may_be_absent on every preset but `kde-breeze` (KColorScheme's decoration roles; the other platforms state focus by `token.focus` and hover per part) | Q3.11, Q3.12, K3.54 |
| `border` | the set's stroke colour (translucent where the platform's is) | required | all; absorbs today's `border.color` and `border.opacity` as `alpha(...)` |
| `shade` | the translucent shade line/overlay of the set (libadwaita `*_shade_color`) | may_be_absent on every preset but `adwaita` (libadwaita's shade colours, no other platform's); on `adwaita` a set GN-1.270 names no shade of states `= role.window.shade` (`--shade-color`) | GN-1.270 |
| `disabled` | `DisabledRoles`: a record of the colour fields above (the record itself is always there) | each field may_be_absent on the presets the `DisabledRoles` paragraph below names for it | R5.1 |

S3's further fields (ADD-A48): `background_active`, `background_link`, `background_visited`,
`background_negative`, `background_neutral`, `background_positive` (KDE's six computed status backgrounds, tinted from
the
set's foreground roles) [K3.52]; `shade_darker` (libadwaita's `headerbar_darker_shade_color`) [GN-1.15]. Each
may_be_absent, with its presets: `background_active` (every preset but `kde-breeze`), `background_link` (every preset
but `kde-breeze`), `background_visited` (every preset but `kde-breeze`), `background_negative` (every preset but
`kde-breeze`), `background_neutral` (every preset but `kde-breeze`), `background_positive` (every preset but
`kde-breeze`) (KColorScheme computes them in every set, K3.52; the other platforms' status backgrounds are
`role.global.status.<s>.background` or keyed roles, W3-17); `shade_darker` (every preset but `adwaita`, and on `adwaita` every set
but `Header`, whose darker shade libadwaita states, GN-1.15).

`DisabledRoles` has the same colour fields as `SurfaceRoles` except `disabled`. Each may_be_absent, with its presets:
`background` (`adwaita`, `macos-*`, `windows-11`, `material-3`), `background_alternate` (every preset but
`kde-breeze`), `foreground` (`adwaita`), `foreground_inactive` (every preset but `kde-breeze`), `foreground_active`
(every preset but `kde-breeze`), `link` (every preset but `kde-breeze`), `link_visited` (every preset but
`kde-breeze`), `negative` (every preset but `kde-breeze`), `neutral` (every preset but `kde-breeze`), `positive`
(every preset but `kde-breeze`), `decoration_focus` (every preset but `kde-breeze`), `decoration_hover` (every preset
but `kde-breeze`), `shade` (every preset), `shade_darker` (every preset), `border` (every preset but `kde-breeze`),
`background_active` (every preset but `kde-breeze`), `background_link` (every preset but `kde-breeze`),
`background_visited` (every preset but `kde-breeze`), `background_negative` (every preset but `kde-breeze`),
`background_neutral` (every preset but `kde-breeze`), `background_positive` (every preset but `kde-breeze`). So
`kde-breeze` states every field whose active field it states (KColorScheme's disabled group, K3.50: `shade` and
`shade_darker` are not KDE's); `adwaita` states the `background` and `foreground` of two sets and leaves every other
field and set absent, as libadwaita's own widgets dim by opacity (below): the `View` set's, the `.view:disabled` pair
(`libadwaita@1.10.0 src/stylesheet/widgets/_views.scss:6-9`), and the `Window` set's, the GTK 3 compatibility colours
`insensitive_bg_color` and `insensitive_fg_color` (`src/stylesheet/_compat-colors.scss:12-13`), public named colours
libadwaita defines for applications' own drawing and its stylesheet does not use itself (GN-1.176, ADD-A122); `macos-*`, `windows-11`,
`ios-*` and `material-3` state the disabled text colour, `foreground` (today's `defaults.disabled_text_color`; on `ios-*`
D-9, D-20), and `ios-*` its disabled background too (D-20), their other disabled looks being per-part rules. On
KDE each field but `border` is the KColorScheme disabled effect applied to the active value (`kde_effects(c,
background, disabled, <role kind>)`, the kind `foreground`, `decoration` or `background` per field, `background` the
set's active background, `active(role.<set>.background)`, never the effected one, which reads `token.kde.disabled`,
S2.2, S4.8) [K3.50], with one exception KColorScheme makes: the `Selection` set's disabled
group starts from the `Window` set's active values whatever `inactive_change_selection_color` says
(`kcolorscheme.cpp:317-319`, S2.2), so each of those fields of `role.selection.disabled` is `=
kde_effects(active(role.window.<role>), active(role.window.background), disabled, <kind>)` (R5.1). `border` is no
KColorScheme role (below), so no effect applies to it: Breeze computes its frame colour from the palette group the
widget is drawn in, `KColorUtils::mix(Window, WindowText, frameIntensityBias())` (`breeze@c97883a kstyle/breezehelper.cpp:142-144`,
the bias KColorScheme's `frameContrast`, `:105`), so `role.<set>.disabled.border` is the active `border`'s derivation
(`kcu_mix(background, foreground, token.params.color.frame_contrast)` in the set, Q3.15) over the set's disabled roles,
`= kcu_mix(role.<set>.disabled.background, role.<set>.disabled.foreground, token.params.color.frame_contrast)`; under
`cond flavor:kirigami` the role stays Breeze's, since Breeze draws Kirigami's buttons and fields; the frames
Kirigami, qqc2-desktop-style and kirigami-addons draw in their own QML (qqc2's Frame, GroupBox, Menu, Popup, ToolTip,
Dialog, the search field's popup and RangeSlider's handles; Kirigami's card, chip, dialog and separator; kirigami-addons'
dialog surface, segmented button, floating button, banner line, video controls and onboarding tip) stroke with
`kirigami_lerp(surface.background, surface.foreground, token.params.color.frame_contrast)` on those parts, and the
switch's thumb, RoundButton and the double floating button with the same over `role.button` (Q3.15, S2.2). On a
platform that dims by opacity (libadwaita) the record stays absent but for the `View` pair above and the parts' `{disabled}` rules set `opacity`
from `token.params.opacity` instead [GN-1.271, GN-1.272].

The inactive group is no record field: it is a colour-role rule `@ cond window_inactive` on the active field
(libadwaita's
`*_backdrop_color`, KDE's `[Colors:*][Inactive]` inputs and effects, AppKit's unemphasised colours) [K3.51, GN-1.136,
S3.2]. On KDE the inactive effects (`kde_effects(c, background, inactive, kind)`, `background` the set's starting
background, its inactive input role `input.inactive.<set>.background_normal`, never the effected one, S2.2) apply over
every set's inactive start, the `Selection` set's included, whose starting background is the one
`kde_selection_inactive` gives: that set starts from `kde_selection_inactive(<its inactive input>, <Window's inactive
input>, active(role.selection.background), <kind>)` (`kcolorscheme.cpp:304-316`, `:374-378`), the function choosing
between the two sets by `token.kde.inactive_change_selection_color`, and the effects are applied over its result
(`:380-391`; K3.40, K1.173, S2.2).

**KDE's inputs** (`kde-breeze`, S9.1). One key vocabulary names them: `<key>` is a key of a scheme file's colour
group, snake-cased (the twelve keys KColorScheme reads, platform-facts §1.3.2: `BackgroundNormal` is
`background_normal`), and `<set>` the colour set's `SurfaceSet` name in snake_case (`window`, `view`, `button`,
`selection`, `tooltip`, `complementary`, `header`). Three namespaces use it:

- `input.kde_scheme.<set>.<key>`: the scheme file's `[Colors:<Set>]` value before Plasma applies the accent (Breeze's
  in the preset, the file kdeglobals' `[General] ColorScheme` names in the reader; K3.54, R25.1);
- `input.kde_scheme.<set>.inactive.<key>`: the file's `[Colors:<Set>][Inactive]` value, declared for every set; a
  set `input.kde_scheme.inactive_groups` does not name states each as `= input.kde_scheme.<set>.<key>`, what
  KColorScheme reads for a set without an `[Inactive]` group (S9.1);
- `input.inactive.<set>.<key>`: the inactive input role, the colour the set's key takes in an inactive window before
  KColorScheme's inactive effects (the start the inactive colour rules above apply `kde_effects(c, background, inactive, kind)` over),
  for every set and key.

`input.kde_scheme.inactive_groups` holds, per scheme, the list of the sets whose scheme file has an `[Inactive]`
group (Breeze's scheme files state one for `Header`, K3.51, K3.55; the reader's and a port's from their own file),
which `kde_accent` branches on (S2.2, S9.1), with two lists beside it: `input.kde_scheme.groups`, the sets the file
has a group for, and `input.kde_scheme.unset_keys`, the keys those groups leave unset; a group or key the file lacks
is stated as KColorScheme reads it unset (a file without a `Header` group, such as Dracula's port, gives `Header`
`Window`'s values), so no input is missing (S9.1). A fourth namespace, with keys of its own, `input.kde_scheme.wm.<key>`, holds the `[WM]` group's four keys (`active_background`, `active_foreground`,
`inactive_background`, `inactive_foreground`, the core's `KdeWmKey`, S4.8), which `kde_accent_wm` reads (S9.1; the
keyed roles `wm_*` below).

Who states the accent-dependent roles and the inactive input roles (the catalogue's one summary of it; the core's
S2.2 `kde_accent` row, S9.1, S9.2, S12 and S14 are the rule, and win where this summary differs; ADD-A109):

- the `kde-breeze` preset's bases state each as `applyScheme()` writes it on Plasma's default path, "Accent color from
  color scheme" (no `AccentColor`): Breeze's scheme values with no accent derivation applied, and the inactive input
  roles as that path writes them (S9.1); the preset states no `kde_accent`;
- the reader states kdeglobals' results, the `[Colors:*]` values and the `[Colors:*][Inactive]` values (K3.51,
  K3.54), as Plasma wrote them for the user's scheme and accent;
- the accent layer (`set_accent`, while no mapping palette applies) states each as `= kde_accent(<set>, <key>,
  <group>)` over the scheme inputs in force, the reader's, a KDE port's, else Breeze's (S9.2). Group `active` gives
  the active role; group `inactive`, for a set `input.kde_scheme.inactive_groups` names, what `applyScheme()`
  writes there (`plasma-workspace@v6.8.2 kcms/colors/colorsapplicator.cpp:247-284`):
  - while the scheme tints (`token.kde.tint_factor` is `Some`): every set's own file value
    `input.kde_scheme.<set>.inactive.<key>`, `Header`'s included, tinted toward the accent;
  - while it does not: `Header`'s the file's active `Window` value `input.kde_scheme.window.<key>` (Plasma 6.8's
    substitute, BUG 516396), every other set's its own file value, untinted;
  - the accent never replaces an `[Inactive]` group's accent keys;
  - then, where the file has a `Header` group and `token.kde.titlebar_is_accent_colored` holds, `Header`'s inactive
    background and foreground are the file's `Window` background tinted toward the accent and the text on it, over
    either branch above, and also where the list does not name `Header` (`applyScheme()` writes that group either
    way, S2.2); where the file has no `Header` group, `applyScheme()` writes none, and `Header`'s active and inactive
    roles are `Window`'s active results, the title-bar accent being `[WM]`'s alone (S2.2);

  and for a set the list does not name, the set's active result, as KColorScheme reads a set without an `Inactive`
  group (S2.2, R25.1). Which roles are accent-dependent is the core's list (S9.1). Group `disabled` is refused (S12);
- a mapping palette states them itself in `[for_preset.kde-breeze]` (it has no scheme inputs for `kde_accent`): each
  accent-dependent role as the project's colour or `= input.accent` where it follows the accent, each inactive input
  role as the project's colour where it publishes an inactive one, else `= active(role.<set>.<role>)`, as KColorScheme
  reads a scheme with no `[Inactive]` group; it is refused where it leaves an accent-dependent or inactive input role
  to the preset's Breeze value (S12, S14);
- a KDE port converts its file by `applyScheme()`'s rules on Plasma's default path, as the preset is stated, not as
  the raw file reads: its `[Inactive]` groups to `input.kde_scheme.<set>.inactive.*` and `inactive_groups`, each
  inactive input role as that path writes it (a non-tinting port's `Header` the file's active `Window` values), and
  each inactive input role of a set with no group as `= active(role.<set>.<role>)` (S14).

`kde-breeze`'s `[declare] accent_inputs` is `["accent"]` alone: the scheme inputs and the inactive input roles are the
scheme's, which Plasma's derivation reads (S9.1). So a reader's `input.accent` from another platform applies on
`kde-breeze`, where the accent layer derives the accent-dependent roles from it by `kde_accent` over Breeze's scheme
inputs, as for `set_accent` (S9.2); under a preset whose `accent_inputs` go beyond `accent` (`windows-11`, `macos-*`, `ios-*`), `Theme`
leaves a foreign reader's `input.accent` out, so no accent mixes with another platform's derived inputs (S8, S9.5).

Each key names one record field, the same in all three namespaces and in `kde_accent`'s `key` operand:

| KDE scheme key | `SurfaceRoles` field |
|---|---|
| `background_normal` (`BackgroundNormal`) | `background` |
| `background_alternate` (`BackgroundAlternate`) | `background_alternate` |
| `foreground_normal` (`ForegroundNormal`) | `foreground` |
| `foreground_inactive` (`ForegroundInactive`) | `foreground_inactive` |
| `foreground_active` (`ForegroundActive`) | `foreground_active` |
| `foreground_link` (`ForegroundLink`) | `link` |
| `foreground_visited` (`ForegroundVisited`) | `link_visited` |
| `foreground_negative` (`ForegroundNegative`) | `negative` |
| `foreground_neutral` (`ForegroundNeutral`) | `neutral` |
| `foreground_positive` (`ForegroundPositive`) | `positive` |
| `decoration_focus` (`DecorationFocus`) | `decoration_focus` |
| `decoration_hover` (`DecorationHover`) | `decoration_hover` |

The record's other fields have no scheme key: `border` and `shade`, `shade_darker` are not KColorScheme roles, and the
six computed backgrounds (`background_active` …) are KColorScheme's tints of these (K3.52).

#### global roles

`GlobalRoles` (S3 list, closed here; every S3 field has a row):

| Field | Meaning | Required | Reasons |
|---|---|---|---|
| `accent` | the accent fill (libadwaita `accent_bg_color`, today's `accent_color`) | required | GN-3.14 |
| `accent_foreground` | text/glyph on the accent (today's `accent_text_color`) | required | GN-3.14 |
| `accent_standalone` | accent used as text or icon colour on the surface, distinct from the fill (libadwaita `accent_color`, OKLab-derived: `adw_standalone`); on `kde-breeze` `= role.window.foreground_active`, where `applyScheme()` writes the plain accent while the Selection set's background, the fill, takes `accentBackground` (`plasma-workspace@6.8.2 kcms/colors/colorsapplicator.cpp:178-181`, `:197-217`, `:229-236`; Kirigami's `activeTextColor`, Q3.5; ADD-A122); elsewhere `= role.global.accent` | required | GN-1.268, GN-3.13, Q3.5 |
| `accent_ramp` | keyed: WinUI Light1-3, Dark1-3 | a list: empty where the platform has none | W3-12 (other group) |
| `status` | `StatusRoles`: `success`, `warning`, `error`, `info`, `neutral`, `destructive`, each a `StatusColors` record with `fill`, `foreground`, `standalone`, `background`, `background_solid` (S3, ADD-A48); `destructive` and `error` are separate entries [GN-1.269] | required on every preset: `error.fill`; on every preset but `windows-11`: `error.standalone`. may_be_absent, each where the platform has no counterpart, with its presets: `success.fill` (`material-3`), `success.foreground` (`material-3`, `ios-*`; on `macos-*` the value phase names it from the facts), `success.standalone` (`material-3`, `windows-11`), `success.background` (`adwaita`, `macos-*`, `ios-*`, `material-3`), `success.background_solid` (every preset); `warning.fill` (`material-3`), `warning.foreground` (`material-3`, `ios-*`; on `macos-*` the value phase names it from the facts), `warning.standalone` (`material-3`, `windows-11`), `warning.background` (`adwaita`, `macos-*`, `ios-*`, `material-3`), `warning.background_solid` (every preset); `error.foreground` (`ios-*`; on `macos-*` the value phase names it from the facts), `error.background` (`adwaita`, `macos-*`, `ios-*`), `error.standalone` (`windows-11`), `error.background_solid` (every preset); `info.fill` (`adwaita`, `macos-*`, `ios-*`, `material-3`), `info.foreground` (`adwaita`, `macos-*`, `ios-*`, `material-3`), `info.standalone` (`adwaita`, `macos-*`, `ios-*`, `material-3`, `windows-11`), `info.background` (`adwaita`, `macos-*`, `ios-*`, `material-3`), `info.background_solid` (every preset but `windows-11`); `neutral.fill` (every preset but `windows-11`), `neutral.foreground` (every preset), `neutral.standalone` (every preset), `neutral.background` (every preset but `windows-11`), `neutral.background_solid` (every preset but `windows-11`); `destructive.fill` (every preset but `adwaita`), `destructive.foreground` (every preset but `adwaita`), `destructive.standalone` (every preset but `adwaita`), `destructive.background` (every preset), `destructive.background_solid` (every preset). The reasons, per platform: Material states no warning, success or info role, and no component draws such a state (gap-map-material's colour-role note, `latest/_md-sys-color.scss:20-206`); libadwaita states a triple per status, `fill`, `foreground` and `standalone`, for success, warning, error and destructive, and no status background or info or neutral colour (GN-1.269); iOS has no status background, foreground or container roles beyond D-9…D-13 (gap-map-ios's C05 note), nor macOS beyond its system palette (no row names one; whether a text-on-status colour exists there is the value phase's); the solid backgrounds and the neutral status are WinUI's (W3-17, W3-18), its solid backgrounds only the attention and neutral ones (`SystemFillColorSolidAttentionBackground`, `SystemFillColorSolidNeutralBackground`, `microsoft-ui-xaml@2.5.1 src/controls/dev/CommonStyles/Common_themeresources_any.xaml:176-177`) and its neutral status a fill and backgrounds with no text colour (`:169-170`, `:175`, `:177`; ADD-A122), whose status colours are fills alone (`SystemFillColor{Success,Caution,Critical,Attention}`, which its controls use only as backgrounds, `microsoft-ui-xaml@2.5.1 src/controls/dev/InfoBar/InfoBar_themeresources.xaml:9-12`, with `TextFillColorInverse` on them, W3-46†), so `windows-11` states no `standalone`, the information colour WinUI's attention and KDE's `ForegroundActive` (W3-17, Q3.5), the destructive triple libadwaita's alone (GN-1.269). No part of a declared variant refers to an absent status role (a preset declares the typed severity variants, `success` and `warning` of `banner`, `dialog` and the rest, only where it states their role; S12's absent-operand check would refuse such a reference anyway); macOS's and iOS's destructive buttons refer to their system red (`role.@system_red`), not to `role.global.status.destructive`, which those presets leave absent; a platform that draws a status keeps stating it | GN-1.269, W3-17/18, K3.52; the gap-map notes named |
| `shadow_reference` | what the platform names as the shadow colour (today's `shadow_color`; macOS `shadowColor`, Material `shadow`) | may_be_absent on `adwaita`, `windows-11`, `ios-*` (no named shadow colour: their shadows state each layer's colour, GN-3.01, W3-01, X-SH-1); `kde-breeze` names one (breezerc `ShadowColor`, K3.1, K3.2) | today's `shadow_color`, MAT-X5, K3.1 |
| `selection_text_background` | the highlight behind selected text (today's `text_selection_background`) | required on every preset (every platform highlights selected text; a platform that does not distinguish it from selected items states a reference to the `Selection` set, below) | today's fields; GN-3.40, X-SE-1, MAC-X15, W3-25 |
| `selection_text_foreground` | the selected text's colour (today's `text_selection_color`); absent where the text keeps its own colour | may_be_absent on `adwaita` (GN-3.41), `windows-11` (W3-25), `material-3` (Compose's `TextSelectionColors` holds a handle and a background colour only, gap-map-material's note on `compose/MaterialTheme.kt:279-281`), `ios-*` (X-SE-1 names the highlight colour alone) | today's fields |
| `selection_text_background_unfocused` | an unfocused field's selection (WinUI's `SelectionHighlightColorWhenNotFocused`, AppKit's unemphasised selection, libadwaita's selection without `:focus-within`; ADD-A49) | may_be_absent on `kde-breeze`, `ios-*`, `material-3` (no unfocused selection colour: KDE's changes with the window, a `window_inactive` rule, K1.45) | W1-032, W3-25, MAC-I05, GN-3.40 |
| `selection_text_foreground_unfocused` | an unfocused field's selected text (macOS's unemphasised selected text colour) | may_be_absent on every preset but `macos-*` (AppKit's alone, MAC-I05) | MAC-I05 |
| `find_highlight` | the find/search match highlight | may_be_absent on `adwaita`, `kde-breeze`, `windows-11`, `material-3` (no find-highlight colour of the platform's: KDE's editor states its match fill on `code_editor.find_match`, K-SF08†) | MAC-DF04, MAC-X15, TA-2, C-25 |
| `drop_highlight` | the drop-target highlight | may_be_absent on `kde-breeze`, `material-3` (no drop-target colour of their own: their drop indicators are stated on the parts, K1.182, MAT-L10) | R12, GN-3.66†, MAC-X21, W3-49†, C-24 |
| `scrim` | modal backdrop colour (with its alpha) | may_be_absent on `macos-*` (no modal scrim role: gap-map macOS's "no modal scrim on macOS"; whether a sheet dims its parent is MAC-D03's measurement, stated on `dialog.scrim @ variant @sheet` with its own colour if it does) | MAT-X23, W3-11, W1-125 |

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
grid roles, `= kde_shade(role.window.background, <role>, token.params.color.contrast)`, KColorScheme's `shade` with
its luma branches over the Window background, `token.params.color.contrast` holding kdeglobals' `[KDE] contrast` as
stored (S2.2) [K3.53]. They are
keyed, not per set: KDE computes them from Window. `wm_active_background`, `wm_active_foreground`,
`wm_inactive_background`, `wm_inactive_foreground`: the `[WM]` group's title-bar colours Breeze's style reads for its
MDI sub-windows (`hp:87-117`) [K2.11]. Who states them:

- the preset: Breeze's scheme files' `[WM]` values (`activeBackground`, `activeForeground`, `inactiveBackground`,
  `inactiveForeground`; `kcolorscheme@b01fde3 src/color-schemes/BreezeLight.colors:189-195`, BreezeDark's likewise), as
  Plasma's default path copies them into kdeglobals;
- the reader: kdeglobals' `[WM]`, which Plasma's `applyScheme()` always writes
  (`plasma-workspace@v6.8.2 kcms/colors/colorsapplicator.cpp:287-309`): each key from the scheme file's `[WM]` where it
  states one, else the scheme's `Header` set, Normal background and foreground for the active keys, the Inactive
  group's for the inactive ones (`:297-303`), so a user's scheme without `[WM]` reaches the reader too; with an
  accent and tinting or the accent-coloured title bar, `applyScheme()` overrides them (the tinted `Window` background
  and its text; the accent title bar's background and its text, `:318-338`). Where kdeglobals has no `[WM]` (one the
  colours KCM never wrote), Breeze falls back to the palette's `Highlight` and `HighlightedText`, the active group's
  for the active title bar and the disabled group's for the inactive one (`breeze@c97883a kstyle/breezehelper.cpp:98-102`),
  which the reader states, in its `for_preset.kde-breeze` part, as references: `= active(role.selection.background)`,
  `= active(role.selection.foreground)`, `= role.selection.disabled.background`, `= role.selection.disabled.foreground`;
- the accent layer: the accent's override (`:318-338`) is part of Plasma's derivation, so the accent layer states
  these four roles too, each `= kde_accent_wm(<key>)` (S2.2, S9.2), over the scheme inputs in force, among them
  `input.kde_scheme.wm.<key>` (S9.1: the scheme file's `[WM]` key, or the `Header` default above where the file has
  none);
- a mapping palette states the four roles in `[for_preset.kde-breeze]` (the project's colours, or as `applyScheme()`'s
  default takes them from its `Header` roles, S14) and is refused where it leaves them to the preset's Breeze value,
  as for the inactive input roles (S12, S14); a KDE port converts the file's `[WM]`, or where it has none the `Header`
  defaults above, to `input.kde_scheme.wm.<key>` and states each role as `= input.kde_scheme.wm.<key>` (S14).

**macOS (`macos-*`).** Label hierarchy beyond the record: `label_tertiary`, `label_quaternary`, `label_quinary`
(14+) [MAC-DF01]; the system fill ramp `system_fill`, `system_fill_secondary`, `system_fill_tertiary`,
`system_fill_quaternary`, `system_fill_quinary` [MAC-DF02]; the system palette beyond red, orange, green and blue:
`system_yellow`, `system_purple`, `system_pink`, `system_teal`, `system_indigo`, `system_cyan`, `system_mint`,
`system_brown`, `system_gray` (and the four already named, stated as `system_red`, `system_orange`, `system_green`,
`system_blue`) [MAC-DF03]; `under_page_background`, `text_background` (against `View.background` =
`controlBackgroundColor`), `selected_control`, `selected_control_secondary`, `selected_control_text` [MAC-DF04,
MAC-DF05]; `highlight` (the separate highlight setting, an accent input, S9.1) [MAC-DF05]. The accent-derived colours are
inputs
(R2.6), and every one of them, `highlight` included, is in the preset's `[declare] accent_inputs` (S9.1: an
application or palette replacing the accent states the text-selection colour with it).

**iOS (`ios-*`).** Label hierarchy: `label_tertiary`, `label_quaternary` (tertiary no longer conflated with disabled
text: `role.window.disabled.foreground` states disabled separately) [D-9, X-CO-5]; fills `system_fill`,
`system_fill_secondary`,
`system_fill_tertiary`, `system_fill_quaternary` [D-10]; backgrounds `system_background`, `system_background_secondary`,
`system_background_tertiary`, `system_grouped_background`, `system_grouped_background_secondary`,
`system_grouped_background_tertiary` [D-11]; their elevated level, keyed too: `system_background_elevated`,
`system_background_secondary_elevated`, `system_background_tertiary_elevated`,
`system_grouped_background_elevated`, `system_grouped_background_secondary_elevated`,
`system_grouped_background_tertiary_elevated` [D-14]. In a sheet, a popover or an alert the elevated level is a
binding (R2.3, R5.2): the box (`body`) of `dialog`, `sheet` and `popover` binds the `Dialog` or `Popover` set (their
roots,
`layer`, bind nothing), whose
backgrounds the preset states as the elevated values, and a `Binding::Role` from each keyed background role to its
elevated twin (`role.@system_background` to `= role.@system_background_elevated`, and so on); colour rules
`@ cond elevated` (S7.1) restate the backgrounds only where the whole window is elevated (multitasking);
`separator_opaque`
(the translucent separator is `border`) [D-12]; `placeholder` [X-CO-2]; the system palette `system_red`,
`system_orange`,
`system_yellow`, `system_green`, `system_mint`, `system_teal`, `system_cyan`, `system_blue`, `system_indigo`,
`system_purple`, `system_pink`, `system_brown`, `system_gray`, `system_gray2` .. `system_gray6` [D-13]. Tint-dependent
roles are inputs (R2.6), each of them in the preset's `[declare] accent_inputs` with `accent` (S9.1); increased-contrast variants are colour-role rules under `ContrastIncreased` [D-15].

**Windows (`windows-11`).** The WinUI tiers, one keyed role per brush of `Common_themeresources_any.xaml`, in the
families the rows name: text tiers (`text_tertiary`, `text_inverse`, text on accent: secondary, disabled, selected)
and accent
text/fill tiers [W3-12, W3-13, W3-14, W3-46]; stroke tiers (control, strong, card, surface, divider, on-accent) [W3-15];
fill tiers (control, strong, subtle, control-alt, on-image, solid background base..senary, base-alt) [W3-16]; status
backgrounds and neutral [W3-17, W3-18]; the layer fills, the content and commanding layers over the backdrop
(`layer_fill_default`, `layer_fill_alt`, `layer_on_acrylic_fill_default`, `layer_on_accent_acrylic_fill_default`,
`layer_on_mica_base_alt_*`, one per brush the facts list) [W3-09]; the eight high-contrast system colours [W3-35].
The accent ramp's entries are inputs (S9.1), in the preset's `[declare] accent_inputs` with `accent` [W3-12].
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
`m3_role` supplies them from the seed input, R2.6, `input.accent`, the preset's one input and so its `accent_inputs`,
S9.1).

### Text ramp

#### text roles

`TextRoles` (S4.4) has nineteen fields, closed (ADD-A42), so Apple's smallest style, Material's `label-small` and the
emphasised captions each have a slot. Each slot is a **reference to a keyed style**, `= token.text.keyed.<key>`: the
preset names the keyed style, so "the facts say which of its styles fills it" is literal and a style is never stated
twice. Where a platform has fewer styles than slots, neighbouring slots **share** a style (marked "=" below): exactly
one slot per shared style, the unmarked one in the column (its **owner**), references the keyed style, and every slot
marked "=" references the owner slot, `= token.text.roles.<owner>` (`subheadline = "= token.text.roles.body"`), so a
patch on the owner (a reader's font, S9.5) reaches every slot that shares it, under any preset; S12 refuses two slots
that can reference one keyed style directly under one combination (S4.4). The share is a stated fact, never a toolkit
default.

The table's platform columns are a **draft of facts**, not structure: which keyed style fills each slot is a fact of
each platform, and the value phase states it in `docs/facts/` (S17) with its citation; where the facts differ, the
facts win and this table is not consulted. The structure here is the slots and their uses.

| Slot | Use | Apple (macOS/iOS) | Windows | GNOME | KDE | Material |
|---|---|---|---|---|---|---|
| `display_large` | the largest hero text | `large_title` = (iOS: `extra_large_title` where iOS draws it, TS-4) | `display` | `title_1` = | `heading_1` = | `display_large` |
| `display_medium` | hero text | `large_title` = | `display` = | `title_1` = | `heading_1` = | `display_medium` |
| `display_small` | hero text, today's `text_scale.display` | `large_title` = | `display` = | `title_1` = | `heading_1` = | `display_small` |
| `large_title` | page/screen title | `large_title` | `title_large` | `title_1` (AdwStatusPage's title, `_misc.scss:147-148`) | `heading_1` = | `headline_medium` |
| `title1` | dialog/page title, today's `dialog_title` | `title_1` | `title` | `title_2` | `heading_1` (Kirigami's dialog and page title, Q1.5) | `headline_small` |
| `title2` | sheet/section title | `title_2` | `subtitle` | `title_3` | `heading_2` | `title_large` |
| `title3` | sub-section title | `title_3` | `subtitle` = | `title_4` | `heading_3` | `title_large` = |
| `headline` | section heading, today's `section_heading` | `headline` | `subtitle` = | `heading` | `heading_3` = | `title_medium` |
| `subheadline` | secondary line under a heading | `subheadline` | `body` = | `body` = | `heading_4` | `title_small` |
| `body` | running and UI text, today's `defaults.font` | `body` | `body` | `body` | `default` | `body_large` |
| `body_emphasized` | emphasised body | `body_emphasized` | `body_strong` | `heading` = | `heading_5_primary` | `body_large_emphasized` (expressive, the emphasised form of `body`'s `body_large`); `title_small` = (baseline: `= token.text.roles.subheadline`) |
| `callout` | body-sized text, slightly set apart | `callout` | `body_large` | `body` = | `default` = | `body_large` = |
| `footnote` | notes | `footnote` | `caption` = | `caption` = | `smallest_readable` = | `body_small` = |
| `caption` | smallest readable, today's `text_scale.caption` | `caption_1` | `caption` | `caption` | `smallest_readable` | `body_small` |
| `caption_emphasized` | emphasised caption | `caption_1_emphasized` | `caption` = | `caption_heading` | `smallest_readable` = | `body_small_emphasized` (expressive); `label_medium` (baseline) |
| `caption2` | the smallest style | `caption_2` | `caption` = | `caption` = | `smallest_readable` = | `label_small` |
| `label` | control labels | `control_content` (macOS), `body` = (iOS) | `body` = | `body` = | `default` = | `label_large` |
| `monospace` | monospace text, today's `mono_font` | `monospaced` | `monospace` | `monospace` (the `monospace-font-name` setting) | `fixed` | `monospace` |
| `numeric` | tabular figures | `body_monospaced_digits` (D-8) | `body` = | `numeric` | `default` = | `body_large` = |

The assignment of KDE's levels follows Kirigami's own use (Q1.5: dialog and page title level 1, sheet header level 2,
form
section level 3, drawer section level 4, list section header level 5 in its primary type). The Material
"expressive/baseline" alternatives are a flavour rule on the token (`cond flavor:expressive`, S7.1); the emphasised
set exists only in Expressive [MAT-TS4]. Under `flavor:baseline`, `body_emphasized` takes `title_small`, which
`subheadline` owns, so that rule states `body_emphasized = "= token.text.roles.subheadline"`, and
`caption_emphasized` references `label_medium` directly (no other slot takes it); every other `Tokens` rule that
points a slot at a style another slot owns references the owner slot likewise (S4.4, S12). Every
cell above is a proposal: the value phase states the facts, and where a fact names another style, the fact, cited,
replaces the cell.

**Material's column against its sources** (ADD-A122). Compose, Material's reference (R1.6), assigns four slots by a
use the slot names: `body` is `body_large`, Compose's default text style (`androidx@e4d933c9
compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/MaterialTheme.kt:113`), so `callout` and
`numeric`, which share it, reference `body`; `label` is `label_large`, a button's label (`Button.kt:169`); `title1` is
`headline_small`, a dialog's title (`tokens/DialogTokens.kt:51-52`, `AlertDialog.kt:330`; under Compose's precision-pointer
sizing, a keyboard and a mouse attached, `androidx@e4d933c9
compose/material3/material3/src/androidMain/kotlin/androidx/compose/material3/PrecisionPointer.android.kt:46-47`,
the title is `headline_small` resized, with its own line height, `AlertDialog.kt:324-327`, but only where the application
turns `ComposeMaterial3Flags.isPrecisionPointerComponentSizingEnabled` on, false by default,
`ComposeMaterial3Flags.kt:66`: an application's opt-in, not the platform's look, and S7.1 has no condition for "a
keyboard and a mouse attached", so the preset states the default and no rule, a condition joining S7.1 with the row
that needs the opt-in; ADD-A123); `large_title` is
`headline_medium`, the large top app bar's title (`tokens/AppBarLargeTokens.kt:28-29`, `AppBar.kt:727`), where the draft
had `headline_large`; `monospace` is the platform's monospace family (MAT-X36†). The other twelve cells, `display_large`,
`display_medium`, `display_small` (the slots named after Material's display styles, which no Compose component
assigns to a use), `title2`, `title3`, `headline`, `subheadline`, `body_emphasized`, `footnote`, `caption`,
`caption_emphasized` and `caption2`, are **unsourced**: no Compose component or token assigns their use, and two
uses near them point elsewhere (a navigation drawer's section headline is `title_small`,
`tokens/NavigationDrawerTokens.kt:76-77`; a list item's supporting line is `body_medium`, `tokens/ListTokens.kt:320-321`),
so they stay the draft's proposal until the maintainer confirms them or a source assigns them. The letter spacings
Compose states in sp (`tokens/TypeScaleTokens.kt`, read by `tokens/TypographyTokens.kt`, each style's
`letterSpacing` in its own block: `BodyLarge` `:27-35`, `BodyMedium` `:37-45`, `BodySmall` `:47-55`, `DisplayLarge` `:57-65`, `DisplayMedium` `:67-75`, `DisplaySmall` `:77-85`, `HeadlineLarge` `:87-95`, `HeadlineMedium` `:97-105`, `HeadlineSmall` `:107-115`, `LabelLarge` `:117-125`, `LabelMedium` `:127-135`, `LabelSmall` `:137-145`, `TitleLarge` `:147-155`, `TitleMedium` `:157-165`, `TitleSmall` `:167-175`, and the emphasised ones `BodyLargeEmphasized` `:178-186`, `BodyMediumEmphasized` `:188-196`, `BodySmallEmphasized` `:198-206`, `DisplayLargeEmphasized` `:208-216`, `DisplayMediumEmphasized` `:218-226`, `DisplaySmallEmphasized` `:228-236`, `HeadlineLargeEmphasized` `:238-246`, `HeadlineMediumEmphasized` `:248-256`, `HeadlineSmallEmphasized` `:258-266`, `LabelLargeEmphasized` `:268-276`, `LabelMediumEmphasized` `:278-286`, `LabelSmallEmphasized` `:288-296`, `TitleLargeEmphasized` `:298-306`, `TitleMediumEmphasized` `:308-316`, `TitleSmallEmphasized` `:318-326`) are what Compose
draws; where material-web's `latest` tokens state another in rem (body medium, display large, title medium, and the
emphasised body large and display large), the facts cite Compose's, the look R1.6 names being the one Compose
draws, and a rem value is never converted (S17's notations give rem no dimension).

#### keyed text styles per platform

A keyed text style is a `TextStyle` (S2.8) the preset declares in `[declare] text_styles`. Each platform lists the
named styles its rows name, whether or not a slot references them, so nothing the rows trace is lost; a named style
no row names is added with the row that needs it (on macOS, AppKit's `menuFont`, `toolTipsFont` and `titleBarFont`
are in no row; its `userFont` and `userFixedPitchFont` are read into `text_area`'s text and the `monospace` slot,
MAC-I07, not keyed).

**Apple.** The eleven text styles, snake_case: `large_title`, `title_1`, `title_2`, `title_3`, `headline`,
`subheadline`, `body`, `callout`, `footnote`, `caption_1`, `caption_2` [MAC-TS01, TS-1]; for each, the emphasised-weight
form `<style>_emphasized` (the HIG's "emphasized" column; Bold Text uses it) [D-7, MAC-TS01]; for each, the tight and
loose
leading forms `<style>_tight`, `<style>_loose` (font-descriptor traits; they differ in `line_height` only) [D-6,
MAC-TS03]; iOS adds `extra_large_title`, `extra_large_title_2` (iOS 17) [TS-4]; the named AppKit fonts `label`,
`message`,
`palette`, `menu_bar`, `control_content` and `small_system` (MAC-TS02's probed set) [MAC-TS02]; `monospaced`, `body_monospaced_digits`
(below). Per style: `letter_spacing` is
`sf_tracking(font_size, <design>)`, the style's `TrackingDesign` (`sf_pro`, `sf_pro_rounded`, `new_york`) (ADD-A54) [D-1]; `font.axes` carries `opsz` as `FollowsSize` [D-2]; `font.width` and
the
family carry the system design and width [D-3]; `font.features` carries monospaced digits where a style uses them [D-8];
`line_height` is each style's own [D-4]; `monospaced` is the system's monospaced font (macOS `monospacedSystemFont`,
its iOS equivalent) and `body_monospaced_digits` is `body` with monospaced digits in `font.features` [D-8]. Dynamic
Type (each style's size and leading per content-size category, non-linear) is `token.text.scaling = "dynamic_type"`
on `ios-*` (S9.4, R25.15, ADD-A88): the preset states each style at the default category, and the query scales it by
Apple's published table for the reader's or the stand-in's `content_size_category` (`ContentSizeCategory`, S8: UIKit's
twelve categories, `None` the default category; a reader that reads each style's
size reports the default category) [D-5, TS-2, X-DT-1, R10]. So every keyed style of `ios-*` states its
`scaling_role` (S2.8), the `TextRole` whose table row scales it: the eleven styles their own slot's (`title_1` the
`title1` row, `caption_1` the `caption` row, …), their `_emphasized`, `_tight` and `_loose` forms and
`body_monospaced_digits` the base style's; for the styles with no row of their own (`extra_large_title`,
`extra_large_title_2`, `monospaced`), the role UIKit scales them by is the value phase's, from
`preferredFont(forTextStyle:)` [D-5]. Every other preset's `token.text.scaling` is `linear`, but `material-3`'s
(`android_non_linear`, MAT-X19); under those two the curve does not depend on the role, so their keyed styles state
`scaling_role = "none"` (an `Option` leaf is stated, S11). Bold Text is the
`BoldText` condition (a token rule on `font.weight`) [TS-3].

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
`default`, `fixed`, `menu`, `toolbar` (kdeglobals `[General]` `font`, `fixed`, `menuFont`, `toolBarFont`, each with
its extras, K3.46; `taskbarFont` is the Plasma panel's, which the shell draws: out, R1.1, K3.47), which the `body`,
`callout`, `label`, `numeric` and `monospace` slots and the menu and tool-button parts refer to. The KDE reader
patches exactly the slots KDE has a setting for (S9.5): `body` from `[General] font`, `monospace` from `fixed` and
`caption` from `smallestReadableFont`; `callout`, `label` and `numeric` share `body`'s style and `footnote`,
`caption_emphasized` and `caption2` share `caption`'s, so they follow the patch by reference (S4.4); `menuFont` and
`toolBarFont` fill no slot, and the reader states them on the keyed styles `menu` and `toolbar` in its
`for_preset.kde-breeze` part (S9.5).

**Derived sizes and text scaling.** A size derived from another style's (`mul(token.text.roles.body.font.size,
<factor>)` above, the KDE sizes by `add`, `page_preview`'s filler text) reads that size converted and scaled (S2.2),
and the query then scales every part's font size (S9.4), so as the core states it the derived size could be scaled
twice. The entries assume text scaling applies once to a derived size; which of the two readings holds (the operand
read unscaled and the result scaled, or the operand read scaled and the result not scaled again: the same under
`Linear` for a product, different for `add` and under the non-linear curves) is the core's to state (S2.2, S9.4).

**Material.** The fifteen baseline styles, `display_large` .. `label_small` (five groups of large, medium, small:
display,
headline, title, body, label) [MAT-TS1], and the fifteen Expressive emphasised styles `<style>_emphasized` [MAT-TS4].
Per style: `letter_spacing` [MAT-TS2]; `font.family` the brand or plain typeface, with its weight name [MAT-TS3];
the variable axes [MAT-TS5], each by one route (S2.8): `wght` as `font.weight`, `wdth` as `font.width`, and `crsv`,
`fill`, `grad`, `hexp`, `opsz`, `rond`, `slnt` in `font.axes`, which S12 refuses to hold the first two; `monospace`, the
platform default monospace typeface (MAT-X36†).

#### tokens carried by these rows

- **Opacities.** `token.params.opacity` (ADD-A44; a group of unit `fraction`, S4.6): `disabled` (today's
  `disabled_opacity`), `strong_disabled`,
  `dim`, `dimmer`. libadwaita's high-contrast values are Tokens rules under `ContrastHigh` [GN-1.271, GN-1.273]. Parts
  refer to `token.params.opacity.<key>` in `opacity` or in `alpha(...)`.
- **Icon sizes.** `IconSizeTokens` (S4.5, ADD-A43): the five typed contexts of today (`toolbar`, `small`, `large`,
  `dialog`,
  `panel`, each a `Value<IconStyle>`, absent on the presets the token absence table names for it, *tokens* below) and
  `keyed: Vec<(Key, IconStyle)>`, so a context also carries
  weight, scale and rendering mode where the platform's context sets them (in a context only `size` is required, every
  other `IconStyle` `Value` field may be absent, and its lists, `axes`, `symbol_variants`, `status_colors`, are stated,
  `[]` where the context sets none, S4.5; `rendering_mode` is a `Value<RenderingMode>`, S2.9; a `scale` names a key of
  `[declare] symbol_scales`, S10) [D-19, MAC-IC02]. Kirigami: `small`, `small_medium`, `medium`, `large`, `huge`, `enormous`, `for_labels`
  [Q1.1]. Kirigami computes `for_labels` from the default font's height, ascent plus descent (`kirigami_label_icon_size`,
  ADD-A53);
  a token has no font metrics (S2.2, S12), so `for_labels` holds the size at the preset's default font (value phase,
  from the facts) for what no part carries, and every part that uses Kirigami's label size states its `icon.size` as
  `= kirigami_label_icon_size(body_font_height)` under `cond flavor:kirigami`, declaring the metric (`data = { icon =
  ["body_font_height"] }`, S8), so it follows a user's font (ADD-A100): `input.icon`, `link.icon`, `badge.icon`,
  `segmented_control.icon` and the clear-field, inline-action and chip-remove `button.icon` (Q1.15, Q1.17, Q1.18,
  Q1.136, Q2.24, Q2.27, K-KA-25†); KDE's Qt Widgets: `main_toolbar`
  (KIconLoader's
  MainToolbar group, beside the typed `toolbar`) [K1.158]. macOS: `button`, `list_row`, `tab`, `status_item`,
  `sidebar` (Tokens rules under `cond container_size:sidebar:small` / `medium` / `large`, which are
  conditions) [MAC-IC01], the toolbar context being the typed `toolbar` (as MAC-IC01's trace states it); the toolbar icon's size per toolbar size mode is a `button.icon.icon.size` rule in `context
  toolbar.bar` by the tool button's control size (`size small` for NSToolbar's small mode; `button` declares
  `small` on `macos-*`), since Tokens rules take conditions only (S7). The
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
| `defaults.icon_sizes.{toolbar,small,large,dialog,panel}_px` | `token.icons.sizes.{toolbar,small,large,dialog,panel}` (each an `IconStyle`; the size is its `size`, the one field a context requires: its other fields may be absent there, S4.5, so a part takes the size by `= token.icons.sizes.toolbar.size` (or another context's), and a whole-context reference brings the absent fields with it) |
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
wherever the block appears and is cited here, not repeated: among the tokens it covers every `Value` field of
`Behavior`, `TouchBehavior`, `ScrollPhysics` and `PointerTokens` (`hide_pointer_delay`, `hide_pointer_fade`,
`scroll_px_per_step`, `touchpad_factor`, `wheel_step_duration_max`, `touchpad_deceleration`, `touchpad_timing`,
`spring_loading`, `touch.*` and `pointer.color` on the platforms without them; `Behavior.key_repeat` is an `Option`,
`none` where keys never repeat, and its `KeyRepeat`'s `delay` and `interval` are stated, S4.2; every parameter
of a `ScrollDecay` is stated), `IconTokens.theme_fallback`, the `Caret` fields it
names, every `TextRendering` field and a `FocusIndicator`'s `shadow` and `transition` (its `dash` is stated, `solid`
included, S2.4). `KdeTokens` and `KdeEffects` are stated whole on `kde-breeze`, never `Absent` (`tint_factor` an
`Option`, `none` where the scheme does not tint; a reader's and a port's unset keys as KColorScheme and `applyScheme()`
read them, S4.8). The second is this table, for the tokens no block mark covers:

| Token | May be `Absent` on | Why |
|---|---|---|
| `token.kde` | every preset but `kde-breeze` | KDE's colour effects and scheme options (S4.8) |
| `token.material` | every preset but `material-3` | Material's dynamic scheme (S4.8) |
| `token.icons.theme` | each preset whose `token.icons.set` is not `freedesktop`: `macos-*`, `ios-*`, `windows-11`, `material-3` | they draw one fixed symbol set and have no named icon theme the freedesktop lookup could use (today's presets repeat the set's name, `native-theme/src/presets/macos-tahoe.toml:19`, `ios-13.toml:27`, `windows-11.toml:18`, `material-3.toml:10`). This supersedes `docs/platform-facts.md` §2.1.9, which has such a preset state its set's name as `icon_theme`; the value phase updates platform-facts to this row |
| `token.icons.sizes.toolbar` | each preset whose platform names no such context: none of `adwaita`, `kde-breeze`, `macos-*`, `windows-11`, `material-3` (each names one, platform-facts §2.1.8); for `ios-*` the value phase names it from the facts (§2.1.8 has no iOS column; D-19) | today's five typed contexts are not every platform's (ADD-A43) |
| `token.icons.sizes.small` | each preset whose platform names no such context: `material-3` (no small icon token, platform-facts §2.1.8); for `ios-*` the value phase names it from the facts (D-19) | as above |
| `token.icons.sizes.large` | each preset whose platform names no such context: `macos-*`, `material-3` (platform-facts §2.1.8); for `ios-*` the value phase names it from the facts (D-19) | as above |
| `token.icons.sizes.dialog` | each preset whose platform names no such context: `adwaita`, `macos-*`, `windows-11` (platform-facts §2.1.8); for `ios-*` the value phase names it from the facts (D-19) | as above |
| `token.icons.sizes.panel` | each preset whose platform names no such context: `adwaita`, `macos-*`, `windows-11` (platform-facts §2.1.8); for `ios-*` the value phase names it from the facts (D-19) | as above |
| `token.params.display.corner_radius_top_left` | every preset but `ios-*`, `material-3`; there for a device model the preset does not list | the measured display radius per corner and device model (S4.6, R25.11, W-6, X-CR-4, MAT-W5†), in the order of `RootInputs.display_corner_radius` |
| `token.params.display.corner_radius_top_right` | every preset but `ios-*`, `material-3`; there for a device model the preset does not list | as above |
| `token.params.display.corner_radius_bottom_right` | every preset but `ios-*`, `material-3`; there for a device model the preset does not list | as above |
| `token.params.display.corner_radius_bottom_left` | every preset but `ios-*`, `material-3`; there for a device model the preset does not list | as above |

**token.motion.durations** (`Vec<(Key, Value<Ms>)>`, S4.1). A scaled duration is `= mul(token.motion.durations.x,
token.motion.speed_factor)`, an unscaled one a plain duration (R9).

| Platform | Keys (one per duration the platform states) | Rows |
|---|---|---|
| libadwaita | the stylesheet's transition variables `backdrop`, `focus`, `button`, `row`, `toggle` (`$backdrop_transition` …, `libadwaita@1.10.0 src/stylesheet/_common.scss:2-7`, `widgets/_lists.scss:1-3`, `widgets/_toggle-group.scss:5-7`), `scrollbar` (`$_scrollbar_transition`, `widgets/_scrolling.scss:4`) and `needs_attention` (the keyframes' name, `_common.scss:114`), and its inline transitions by S4.1's step 2, the class (its CSS node's widget) and, where one class states different values, the transition in the source's words: `tab_bar` (AdwTabBar's separator and tab fades, one duration, `widgets/_tab-view.scss:39`, `:59`), `tab_thumbnail` (AdwTabThumbnail's shadow, `:199`; its background colour's transition on the same line states no duration, CSS's 0s, so it changes at once and the key does not cover it), `bottom_sheet_border_radius` and `bottom_sheet_box_shadow` (AdwBottomSheet's sheet and outline, `widgets/_bottom-sheet.scss:10`, `:30`), `expander_row` (AdwExpanderRow's arrow rotation, `widgets/_lists.scss:392`), `places_view` (GtkPlacesView's server-list arrow, `widgets/_file-chooser.scss:68`) and `toggle_group` (AdwToggleGroup's separators, `widgets/_toggle-group.scss:142`) (ADD-A123); the timed animations defined in code: AdwToastOverlay's `toast_overlay_show`, `toast_overlay_hide`, `toast_overlay_replace` (`src/adw-toast-overlay.c:20-22`), AdwTab's `tab_close_btn`, `tab_attention_indicator` (`adw-tab.c:19`, `:28`), AdwTabOverview's `tab_overview_scroll` and `tab_overview` (`SCROLL_ANIMATION_DURATION`, `TRANSITION_DURATION`, `adw-tab-overview.c:29-30`), AdwEntryRow's `entry_row` (`adw-entry-row.c:18`), AdwViewStack's `view_stack` (`adw-view-stack.c:1653`), the deprecated AdwFlap's fold, `flap_fold`, and AdwLeaflet's mode transition, `leaflet_mode` (`adw-flap.c:1551`, `adw-leaflet.c:2445`), AdwTabBox's `tab_box_open`, `tab_box_close`, `tab_box_focus`, `tab_box_scroll`, `tab_box_resize`, `tab_box_reorder`, `tab_box_icon_resize` (`adw-tab-box.c:30-36`) and AdwTabGrid's `tab_grid_open`, `tab_grid_close`, `tab_grid_focus`, `tab_grid_resize`, `tab_grid_reorder`, `tab_grid_icon_resize` (`adw-tab-grid.c:29-34`), each class stating different values for its transitions; the revealer and stack defaults, `revealer` and `stack`; the animated scroll, `scrolled_window` (GtkScrolledWindow's `ANIMATION_DURATION`, `gtk@4.24.1 gtk/gtkscrolledwindow.c:189`) | GN-3.18, GN-3.21, GN-3.22, GN-3.24 |
| kde-breeze | `animations` (Breeze's `Animations`, its state engines' duration: while kdeglobals sets `AnimationDurationFactor`, `= floor(mul(<base>, token.motion.speed_factor))`, `<base>` the literal the factor is multiplied by at `breeze@6.8.2 kstyle/breezestyle.cpp:2080` (not breezerc's `AnimationsDuration`), the product truncated to whole ms, a positive result turning Breeze's animations on whatever breezerc says (`:2081-2083`) and zero, which a factor below 0.01 gives, turning them off (`:2085`), so the duration is then zero; while the factor is unset, breezerc's `AnimationsDuration`, zero where its `AnimationsEnabled` is off (`:2076-2078`, `kstyle/animations/breezeanimations.cpp:49-55`); Qt's `SH_Widget_Animation_Duration` too, `breezestyle.cpp:937-938`, which the dock widgets' and toolbars' re-layout and KCollapsibleGroupBox's expansion reference, the last with the source's floor by `max`, `kwidgetsaddons@6.31.0 src/kcollapsiblegroupbox.cpp:114-116`); `decoration_shadow` (the decoration's shadow fade, a base times `token.params.motion.kwin_speed_factor`, the factor KWin's configuration gives, kwinrc's over kdeglobals', `floor`ed, `kdecoration/breezedecoration.cpp:379-389`, K1.17, ADD-A123); `line_edit_icon_button` (Qt's side-icon fade, unscaled, `qtbase@6.12.0 src/widgets/widgets/qlineedit_p.cpp:432`); `tab_bar` (Qt's tab move, `ANIMATION_DURATION`, unscaled, `src/widgets/widgets/qtabbar_p.h:30`); `message_widget` (KMessageWidget's timeline, `kwidgetsaddons@6.31.0 src/kmessagewidget.cpp:60`); `variant` (QVariantAnimation's own default, unscaled, which QTreeView's expand takes, K1.185†, `qtbase@6.12.0 src/corelib/animation/qvariantanimation.cpp:153`); Kirigami `very_short`, `short`, `long`, `very_long` (derived from `long`, with the source's floor by `max`, ADD-A56) and `human_moment` (unscaled) | K1.17, K1.185†, K3.31, K3.37, K3.38†, Q3.23–Q3.25, Q3.27 |
| windows-11 | `control_normal`, `control_fast`, `control_fast_after`, `control_faster` (WinUI's `ControlNormalAnimationDuration`, `ControlFastAnimationDuration`, `ControlFastAnimationAfterDuration`, `ControlFasterAnimationDuration`, `Common_themeresources_any.xaml:603-606`); `connected_service` (ConnectedAnimationService's default, `src/dxaml/xcp/components/animation/ConnectedAnimationService.cpp:26`); `scroll_into_view` (the scroll into view, measured, W-P4-3†); one entry per built-in theme transition the facts measure, named as WinUI names its class less `Transition` and `Animation` (`EntranceThemeTransition` `entrance_theme`: entrance, content, add and delete, reorder, reposition, popup, pane, drill in, press tilt, fade, split open and close, drag item) | W3-30, W3-32, W3-33, W3-34, W-P4-3† |
| macos-* | by family, the facts lanes naming each key: the AppKit durations (`NSAnimationContext`'s default, the window resize time, per-control durations the probe records), `navigation` | MAC-X11 |
| ios-* | `navigation` (push and pop); by family, the facts lanes naming each key: one per modal transition style by its `UIModalTransitionStyle` case (`cover_vertical`, …), the symbol-effect durations | X-MO-1, X-MO-3, X-MO-4, X-MO-9 |
| material-3 | the duration tokens `short1`–`short4`, `medium1`–`medium4`, `long1`–`long4`, `extra_long1`–`extra_long4` (`DurationShort1` …, `androidx@e4d933c9 compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/tokens/MotionTokens.kt:24-39`); by family, the facts lanes naming each key: the ripple's (material-web's `Ripple`), the shadow transition's | MAT-X13, MAT-X9, MAT-X7 |

**token.motion.timings** (`Vec<(Key, Timing)>`): cubic Béziers, springs (`Timing::Spring`), `Timing::Platform(PlatformCurve)` where a
platform's curve is one its toolkit computes by a function of its own (S2.7, R9: Kirigami's velocity-based curves,
`KirigamiVelocity`; Qt's and libadwaita's easings, `Qt(…)`, `Adw(…)`). Keys per platform: libadwaita's
`ease_out_quad` (`$ease-out-quad`, `libadwaita@1.10.0 src/stylesheet/_common.scss:1`), the CSS keyword curves its
stylesheet uses (`ease_out`, `ease_in`, `ease_in_out`, `ease`, `linear`) and AdwTimedAnimation's default
`ease_out_cubic` (`ADW_EASE_OUT_CUBIC`, `adw-timed-animation.c:275`, `Timing::Platform(Adw(EaseOutCubic))`, the polynomial
`adw-easing.c:175-181` computes, S2.7, ADD-A123) (GN-3.19), and the easings its code applies, each named for its
`AdwEasing` (S4.1 step 1) and a `Timing::Platform(Adw(…))`: `ease_in_cubic` (the tab box's and tab grid's drag
autoscroll, `adw-tab-box.c:1450`, `adw-tab-grid.c:1473`), `ease_in_out_sine` (AdwSpinner's arc, `adw-spinner-paintable.c:121`,
`:141`, GN-1.229's `spinner.values.loop`), `ease_in_expo` and `ease_out_expo` (AdwTabOverview's
cross-fade and fades, `adw-tab-overview.c:1307`, `:1336`, `:1347`) (ADD-A123); its code's other easings are
`ADW_LINEAR`, `ADW_EASE` and `ADW_EASE_IN_OUT`, the keys `linear`, `ease` and `ease_in_out`, and its springs below (GN-3.20);
on `kde-breeze` Qt's curves by their `QEasingCurve` names, each `Timing::Platform(Qt(<type>))` (S2.7, ADD-A123): the
decoration's shadow fade, `out_cubic` both ways
(`breeze@6.8.2 kdecoration/breezedecoration.cpp:197`, `:278-279`: the deactivation's in-cubic curve runs backward,
which in S2.7's forward reading is the out-cubic curve, `qtbase@6.12.0 src/corelib/animation/qvariantanimation.cpp:200-202`, K1.17), `in_out_quad` (the tree
expand, the tab move and the dock-widget and toolbar re-layout, `qtbase@6.12.0 src/widgets/itemviews/qtreeview_p.h:89`,
`src/widgets/widgets/qtabbar_p.h:144`, `src/widgets/widgets/qwidgetanimator.cpp:62`), Kirigami's and qqc2-desktop-style's
curves in use, `in_out_quad`, `out_cubic`, `in_cubic`, `in_out_cubic`, `out_quad`, `in_quad` (QML's `Easing.<curve>`,
by search of `kirigami@6.31.0 src` and `qqc2-desktop-style@6.31.0`) and `out_expo` (`kirigami@6.31.0
src/layouts/columnview.cpp:462`) (Q3.26); Breeze's `animations` (`Timing::Steps`, its count breezerc's `AnimationSteps`, K3.31); `in_out_sine`, Qt's
default easing curve for its timelines (`QTimeLine`'s, `qtbase@6.12.0 src/corelib/tools/qtimeline.cpp:24-25`, K3.37); WinUI's entering and exiting curves,
`fast_out_slow_in` and `slow_out_fast_in`, and its control key spline `control_fast_out_slow_in`
(`ControlFastOutSlowInKeySpline`, `Common_themeresources_any.xaml:602`) (W3-31), the connected animation's
`connected_service` (`microsoft-ui-xaml@2.5.1 src/dxaml/xcp/components/animation/ConnectedAnimationService.cpp:27-28`,
W3-33) and the scroll into view's `scroll_into_view` (measured, W-P4-3†); UIKit's default
spring, `default` (X-MO-2), the navigation push and pop's `navigation` (measured, X-MO-3), and AppKit's media-timing curves by family (MAC-X11); Material's easing tokens `standard`,
`standard_accelerate`, `standard_decelerate`, `emphasized`, `emphasized_accelerate`, `emphasized_decelerate`, `legacy`,
`legacy_accelerate`, `legacy_decelerate`, `linear` (`EasingStandardCubicBezier` …, `MotionTokens.kt:40-68`) and spring
tokens `default_spatial`, `default_effects`, `fast_spatial`, `fast_effects`, `slow_spatial`, `slow_effects`
(`SpringDefaultSpatialDamping` and `…Stiffness` …, `tokens/StandardMotionTokens.kt:20-31`,
`ExpressiveMotionTokens.kt:22-33`, the motion scheme the preset draws) (MAT-X13). Every key,
duration or timing, is named by S4.1's rule (ADD-A122): libadwaita's inline springs `floating_sheet_open`,
`floating_sheet_close`, `bottom_sheet_open`, `bottom_sheet_close`, `navigation_view_push`, `navigation_view_pop`,
`overlay_split_view_show`, `overlay_split_view_hide`, `carousel` (GN-3.20), and the deprecated AdwFlap's and
AdwLeaflet's, which applications built on them still draw: `flap_reveal` (`libadwaita@1.10.0 src/adw-flap.c:1598-1599`, clamped at `:1600-1601`)
and `leaflet_child` (`adw-leaflet.c:2474-2476`, clamped at `:2477-2478`), each the class's spring for that transition.

**token.motion.speed_factor** (`Value<f32>`): read on KDE (K3.30, K3.90, Q3.24) and, by the Android reader, on
`material-3` (the animator duration scale, MAT-X20, `readers (R18)`); a constant elsewhere (the platform has no speed
setting; the value phase states it). Rules set it
to zero under `AnimationsOff` where the platform's switch does that (GN-3.25, Q3.25, W3-37, W3-52†, MAT-X20).

**token.theme_change** (S4.3), one kind per preset, measured where no source states it: `adwaita`
`ThemeChange::Suspend` (GN-3.23); `kde-breeze` (K-P3-15†), `windows-11` (W-P3-16†, which also covers the accent change), `macos-*`
(MAC-P4-9†, accent and highlight change), `ios-*` (X-MO-11†), `material-3` (MAT-P3-18†). A stated zero duration on
`Crossfade` or `Suspend` is refused, as that change is `Instant` (S12; a computed zero, under a speed factor of zero,
is not).

**token.elevation** (`Vec<(Key, Vec<ShadowLayer>)>`, S4/R8): the ramp's keys, referred to by `Shadow::Elevation(key)`.

| Platform | Keys | Rows |
|---|---|---|
| windows-11 | the elevation steps WinUI names (1 to 128), each its ambient layer, the dark scheme's intensity as a colour-role-free rule under `scheme:dark` | W3-01, W3-02, W3-03 |
| material-3 | levels 0 to 5, each a key and an ambient layer (layers per level), per-state level by rules | MAT-X2, MAT-X3, MAT-X4, MAT-X5 |
| kde-breeze | the window shadow size steps and the popup shadow size steps (two tables, two layers per step), with `token.params.shadow.strength` and the shadow colour read from breezerc | K3.1, K3.2 |
| Kirigami (flavour `kirigami`) | one entry per Kirigami and qqc2 surface, declared preset-wide (S10 has flavour declarations for widgets and components only) and referred to only by rules under `cond flavor:kirigami` | Q3.21 |
| libadwaita, macos-*, ios-* | (none): layers are stated per part (`PartStyle.shadow.layers`) | GN-3.01, MAC-X01, X-SH-1 |

**token.params** (ADD-A44), keyed scalar tokens in groups, each group of one unit its declaration names (S4.6, S10):
`token.params.shadow.strength` (K3.1, K3.2); `token.params.motion.animation_steps` (K3.31); `token.params.motion.kwin_speed_factor` (K1.17: the speed factor
KWin and its decoration read, kwinrc's `[KDE] AnimationDurationFactor` where it states one, else kdeglobals', which
`token.motion.speed_factor` reads alone, and 1 where neither states one, the decoration's default,
`breeze@6.8.2 kdecoration/breezedecoration.cpp:389`; ADD-A123);
`token.params.color.contrast` and `token.params.color.frame_contrast` (K3.53, K3.91, Q3.14, Q3.15);
`token.params.layout.grid_unit` (Q3.29, a length); `token.params.optics.optical_centering` (MAT-X37†: Material's
unitless optical-centering coefficient, in a group of unit `number` of its own, since `layout` is a group of lengths);
`token.params.caret.aspect_ratio` (GN-3.36); `token.params.shape.<step>` for Material's shape scale from `none` to `extra_extra_large` (MAT-X11; its `full`, half
the height at any size, is no scalar: the parts state it as `Radius::Full { maximum: Absent }`, MAT-X11's trace);
`token.params.display`'s four corner keys (S4.6, R25.11, W-6, X-CR-4: the absence row above);
`token.params.icon_effect.<key>` (K3.60, Q3.18); `token.params.pointer_repeat.delay` and
`token.params.pointer_repeat.interval` (a group of unit `ms`, keys `delay` and `interval`): the platform's one system
pointer auto-repeat, the delay before a held pointer button repeats and the interval between repeats, declared by the
presets whose platform states one system value, `adwaita` (GTK's `gtkrange.c:73-74` constants, GN-3.69†) and
`macos-*` (`NSButtonCell.getPeriodicDelay(_:interval:)`, MAC-X12), the facts adding others. A widget's own
auto-repeat values (`repeat_delay`, `repeat_interval`, rename row 66) keep their fields; on those presets their bases
are `= token.params.pointer_repeat.delay` and `= token.params.pointer_repeat.interval`, and a widget states its own
value only where its own source states another (GTK's spin button's interval, `gtkspinbutton.c:63-67`, GN-3.69†):
one home per fact, as `drag_hover_delay_ms` has.

**token.kde** (`KdeTokens`, S4.8, `kde-breeze` only): `disabled` and `inactive`, each a `KdeEffects` (intensity effect
and amount, colour effect, amount and colour, contrast effect and amount: K3.50, K3.51; the amounts `Value<f32>` as
kdeglobals states them, each function clamping them as KColorScheme does), the inactive group's
`inactive_change_selection_color` and `inactive_enabled` (K3.40), the scheme's `tint_factor` (an `Option`: `none`, the
scheme does not tint) and `titlebar_is_accent_colored` (R25.1), every one stated (S4.8). The icon effects (K3.60, Q3.18) are `IconStyle.state_effect` keys (S2.9, declared
in `icon_state_effects`), their gray, transparency and gamma parameters `token.params.icon_effect.<key>`. A palette from
a KDE port states `token.kde` itself (R19, S14). Who states KDE's accent-dependent roles and inactive input roles,
and how (the preset Breeze's values on Plasma's default path, the reader kdeglobals' results, the accent layer
`kde_accent`, palettes and ports), and `kde-breeze`'s `accent_inputs`, is *KDE's inputs* under *role record*, the
catalogue's one summary of the core's S2.2, S9.1, S9.2, S12 and S14.

**token.behavior** (S4.2): the existing fields (double-click time and distance, drag threshold, hover-open delay,
`key_repeat` (a `KeyRepeat` of a delay and an interval, `none` where keys never repeat), scroll lines per step, hide pointer while typing, scroll physics) carry these rows:
GN-3.47, GN-3.48, GN-3.67†, GN-3.68†, K3.70–K3.72, K-P3-16†, W3-41, MAC-X12, MAC-P3-12†, MAT-P3-14†,
MAT-P3-15†, MAT-X32†, P3-13†, GN-P3-15†. Scroll physics (`token.behavior.scroll`) carries kinetic friction,
overshoot and its spring, animated-scroll duration and curve, snap settle, momentum: GN-3.24, GN-3.06, K-P3-18†,
MAC-P4-4†, MAC-X32†, W3-52†, X-MO-8, MAT-P3-16† (its minimum and maximum fling velocities: `min_fling_velocity`,
`max_fling_velocity`; Kirigami's wheel step whose duration grows with the distance between `wheel_step_duration`
and `wheel_step_duration_max`, K-P3-18†). New fields: ADD-A45; `hide_pointer_delay` (KLineEdit's, K3.65†),
`hide_pointer_fade` (Android's gradual fade, MAT-P3-15†), `scroll_px_per_step` (Android's pixels per wheel step,
MAT-X32†): ADD-A86. GTK's step from the view's page size is `scroll_view.values.wheel_step` (GN-P3-15†), as are
NSScrollView's line-scroll distances per axis; its page overlap is `scroll_view.values.page_overlap` (MAC-P3-17†).
`focus_visible_when` (when focus becomes visible, a `FocusVisibleWhen`, the
default of every widget's value): ADD-A16. On `windows-11` it is `Keyboard { timeout: None }`: WinUI draws the system
focus visual on the focused element only while its focus state is `Keyboard` (`microsoft-ui-xaml@2.5.1
src/dxaml/xcp/core/dll/focusmgr.cpp:2729-2746`, `IsKeyboardFocused`, `src/dxaml/xcp/core/inc/uielement.h:867-870`) and
`UseSystemFocusVisuals` holds, as the templates set it (`src/controls/dev/CommonStyles/Common_themeresources.xaml:9`),
with no timeout (W3-21; ADD-A122).

**token.focus** (S4/R12): the platform's one indicator (width, offset, kind, dash, corners, transition; its `dash` stated on every preset,
`solid` where the ring is not dotted, S2.4, S2.10; an inner ring is a layer of its `shadow`, R7, ADD-A17; its corners `Radius::FollowPart`, R12's concentric default), by reference from every focusable part; parts that differ state their own (KDE's underline under tab
labels, K3.21; Material's inner offset, MAT-X10; WinUI's per-control margin, W3-20). Material's growing focus ring
(an active width, then settling) is material-web's `md-focus-ring` component, which Compose, the reference, does not
draw: `reference (R1.6)` (MAT-X10; the system tokens' thickness and offsets stay `token.focus`'s);
WinUI's opt-in kinds are `FocusKind::Keyed(@reveal)` (the Reveal glow) and the ring with `dash` `Dotted` (DottedLine)
(W3-20, ADD-A82). `windows-11`'s inner ring is `token.focus.shadow`, one `inset` layer cast by the ring's inner outline
(S2.10): spread `FocusVisualSecondaryThickness`'s default (`microsoft-ui-xaml@2.5.1
src/dxaml/xcp/components/DependencyObject/DependencyProperty.cpp:24-25`), colour the secondary brush
`SystemControlFocusVisualSecondaryBrush`, `FocusStrokeColorInnerBrush` (`src/controls/dev/CommonStyles/Common_themeresources.xaml:16`),
drawn just inside the outer ring (`src/dxaml/xcp/components/FocusRect/FocusRectManager.cpp:445-455`); its `offset` is
the inner edge's, `FocusVisualMargin` negated less the ring's width (S2.10) (W3-19, W3-20; ADD-A122). Rows: GN-3.27–GN-3.29,
K3.20–K3.24, Q3.30, W3-19, W3-20, MAC-X13, X-FO-1, MAT-X10.

**token.caret** (S4.2): colour, width, blink on and off times, blink timeout, blink style, secondary colour,
`solid_while_typing` (the caret kept solid while typing, X-CA-2, ADD-A85), `shown_in_read_only`; its shape is the
caret parts' rules, not a token field. Hiding the caret while text is selected and the overwrite block caret are the
text widgets' caret parts' rules (`caret` absent under `{text_selected}` on every preset but `kde-breeze`, which keeps
it, K3.43, *Shared flags*; absent under `{overwrite}`, `block_caret` drawn under `{overwrite}`: S4.2, ADD-A23). A caret that does not
blink is drawn, so it is not
`Absent`: `blink_off` is zero (never off; TOML `blink_off_ms`), as the platforms' "no blink" settings state it (GN-3.32,
X-CA-2). A caret that blinks without ever stopping (KDE's period alone, K3.41; macOS, MAC-X12; iOS, X-CA-2) states
`blink_timeout = "none"` (S4.2's `Option<Value<Ms>>`, `None` never stopping; ADD-A100), never `Absent`, which means
not drawn. Rows:
GN-3.32–GN-3.38, GN-3.73†, K3.41–K3.43, W3-24, W-P3-9†, MAC-X12, MAC-X14, X-CA-1, X-CA-2, P3-9†, MAT-X25. The caret
is also a part of the text widgets (ADD-A24). Additions: ADD-A24.

**token.text** (S4.4): `roles` and `keyed` as S4.4 states, per platform the styles its facts list; `font_dpi`
(`Value<f32>`, logical pixels per inch, required on every preset and stated from its platform's facts, the DPI its
points convert by, S9.4; where the platform has a DPI setting the reader reports it as `ReaderState.font_dpi`, KDE's
`forceFontDPI` or `Xft.dpi`, platform-facts' *Font DPI override*, which then wins over the token, ADD-A103). Keyed
styles one flavour alone uses (Material's `_emphasized` styles, Expressive's) are declared preset-wide, since S10
has flavour declarations for widgets and components only, and are referenced only under `cond flavor:<f>`, as
Kirigami's elevation entries are (`token.elevation`). Group 8 adds
the per-role line height, letter spacing and optical-size axis (MAC-X18, W3-23, MAT-X16,
X-DT-5, GN-3.57's weight), the default size category's sizes on `ios-*` (X-DT-1), `monospace` with a source on
`windows-11` (W3-47†) and the platform default sans and monospace on `material-3` (MAT-X36†). `token.text.fallbacks`
(ADD-A41) holds the family fallbacks, emoji family and per-language fallback map (all FM rows); `token.text.rendering`
(ADD-B04) holds `engine` (Qt Quick's render type, Q3.32), `antialiasing`, `hinting`, `subpixel_order`,
`lcd_filter`, `gamma` and `contrast` (S4); `token.text.scaling` how the
text-scaling factor applies (`AndroidNonLinear` on `material-3`, MAT-X19; `DynamicType` on `ios-*`, Apple's
per-style table for the content-size category, `ReaderState.content_size_category` (`ContentSizeCategory`, S8), D-5,
TS-2, X-DT-1, R25.15; `Linear` elsewhere; S9.4, ADD-A88), with
each keyed style's `scaling_role` (*keyed text styles per platform*).

**token.icons** (`IconTokens`, S4.5): `set`, `theme`, `theme_fallback`, `sizes` and `mirroring` (per icon role,
GN-FM03†, MAT-FM04†, W3-54†, X-SV-3). Under `sizes`, contexts beyond the five groups (GN-3.59, K3.63, W3-27) are keys
of the same token; weight and scale are `IconStyle` fields (X-IC-2), stated in a context where the platform's context
sets them (S4.5) and on parts otherwise; a `scale` is a key of `[declare] symbol_scales` (S10).

**token.pointer** (ADD-A46): cursor theme, cursor size, pointer colours where read; `drag_cursors` keyed by drop
operation (`copy`, `move`, `link`, `ask`, `none`, `not_allowed`) for GN-P3-12†, K-P4-7†, W-P4-5†, MAT-P4-3†, P3-10†.

**token.size_classes**: no row of group 8 adds a threshold list. `ios-*` takes its class from the system (X-SV-1,
read by the connector at the root, R16); Material's width thresholds are stated by the widget groups' rows.

**token.material** (`MaterialScheme`, S4.8, `material-3` only): `variant`, `spec_version` (a key of `[declare]
spec_versions`, the versions `m3_role` implements, S10) and `contrast_level`, which `m3_role` reads and so is never
absent: the base states the standard level, and `Tokens` rules under `cond contrast_increased` and `cond
contrast_high` set the medium and high levels (MAT-X18, MAT-P3-19†); the seed is `input.accent` (S9.1), which `m3_role`
reads (MAT-X17). The preset's `variant` is the one Android applies where the user chose no style, which
SystemUI's theme code chooses (`frameworks/base/packages/SystemUI`, outside the pinned sparse checkout of
`frameworks-base@android-17.0.0_r1`): **not pinned**, so the facts state it measured on the emulator image (the reader's
`variant`, MAT-X17) or cite SystemUI once it is pinned, never assumed (ADD-A123).

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
| `AccessibilityTextSize` | the content size category (`ContentSizeCategory`, S8) is one of its five `Accessibility*` categories; layouts change axis | X-DT-3, X-A11-10 |
| `Idiom` | `Phone`, `Pad` (the idiom axis beside the size class) | X-SV-1 |
| `TintDimmed` | the window or scene is covered by a modal presentation; iOS dims the tint | X-CO-3 |
| `OverlayScrollbars` | the system shows overlay scroll bars (today's scroll-bar field `overlay_mode`): GNOME's overlay scrolling, macOS's overlay scroller style, Windows' auto-hiding scroll bars | GN-1.85, MAC-S01, W1-152† |
| `UnitScale` | the device pixel ratio is 1: Qt Quick's text rendering picks its engine by it | Q3.32 |
| `Freeform` | an Android desktop-windowing (freeform) window, with its own corner radius (ADD-A83) | MAT-W5† |
| `Posture(Posture)` | a foldable's posture (Jetpack WindowManager): two stacked panes in tabletop (ADD-A83) | MAT-P3-13† |
| `DeviceModel(Key)` | the device model, where a measured value differs per model: the display's corner radius (ADD-A83) | W-6, X-CR-4 |
| `SystemSetting(Key)` | a platform-specific switch the reader reads, declared by the preset in `[declare] conditions` (the traceability's `cond system_setting:<key>` lines are the full set, each with its row): Win32 `menu_fade`, `tooltip_fade`, `selection_fade`, `combo_animation`, `listbox_smooth_scroll`, `hot_tracking`, `drop_shadow`, `ui_effects` (W3-53†), `menu_animation` (W1-054), `tooltip_animation` (W1-058), `menu_drop_alignment` (`SPI_GETMENUDROPALIGNMENT`, W3-55†), `accent_on_title_bars` (W1-001, W1-013), `material_policy` (W3-07), `show_sounds` (W-P3-11†, declared `for_applications = true`: only applications read it, through `Theme::system`, S10); KDE `ui_effects` (K-P3-24†), `stacked_widget_transitions` (breezerc `StackedWidgetTransitionsEnabled`, K3.31), `dock_widget_draw_frame` (breezerc `DockWidgetDrawFrame`, K2.10), `outline_enabled` (breezerc `[Common] OutlineEnabled`, K1.9), `rounded_corners`, `no_borders` (K1.10, K-P3-1†), `no_side_borders` (K-P3-1†), `draw_border_on_maximized`, `hide_title_bar` (K1.15), `title_bar_gradient` (K1.14), `outline_close_button` (K1.7), `menu_item_strong_focus` (K1.62), `show_icons_in_menus` (K1.70), `slider_tick_marks` (K1.103), `toolbar_draw_item_separator` (K1.156), `side_panel_draw_frame` (K1.140), `view_draw_focus_indicator` (K1.175), `view_draw_tree_branch_lines` (K1.176), `qqc2_desktop_style` (K-KA-40†), `single_click` (kdeglobals `[KDE] SingleClick`: the file views' hover feedback, K3.73); macOS `wallpaper_tinting` (MAC-W08, MAC-X27) and `title_icons` (MAC-X31†); iOS `glass_look` (X-MT-11) and `pointer_animations` (the Pointer Animations setting, P3-12†, X-PH-1) | W3-53†, W1-054, W1-058, W3-55†, W1-001, W1-013, W3-07, W-P3-11†, K-P3-24†, K3.31, K2.10, K1.9, K1.10, K-P3-1†, K1.15, K1.14, K1.7, K1.62, K1.70, K1.103, K1.156, K1.140, K1.175, K1.176, K-KA-40†, K3.73, MAC-W08, MAC-X27, MAC-X31†, X-MT-11, P3-12† |

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
- overlay-scrolling, `org.gnome.desktop.interface`, into `cond overlay_scrollbars` (read today into
  today's scroll-bar field `overlay_mode`, gap-map-gnome's W1-152† note): GN-1.85.
- show-status-shapes, `org.gnome.desktop.a11y.interface` (lines 11-17), into `cond differentiate_without_color`:
  GN-1.202.

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
- kdeglobals `[WM]` `activeBackground`, `activeForeground`, `inactiveBackground`, `inactiveForeground` (the keyed
  roles `role.@wm_*`, the MDI sub-window's title bar; where kdeglobals has no `[WM]`, the reader states Breeze's
  fallback references to the `Selection` set, *keyed roles per platform*): K2.11.
- kdeglobals `[KDE] contrast` (unread today) and `frameContrast` (the factor kept, not only finished colours):
  K3.53, K3.91, Q3.14.
- kdeglobals `[KDE] CursorBlinkRate`: K3.41; `DoubleClickInterval`: K3.70; `StartDragDist`, `StartDragTime`: K3.71;
  `WheelScrollLines`: K3.72; `SingleClick` (into `cond system_setting:single_click`, under which the preset's
  `Tokens` rule states `token.behavior.single_click_activates`, one route, S4.2): K3.73; `SmoothScroll`: K-P3-18†; `GraphicEffectsLevel` (into `cond
  system_setting:ui_effects`): K-P3-24†.
- breezerc `[Style] MnemonicsMode`: K3.44.
- kdeglobals `[General] XftAntialias`, `XftHintStyle`, `XftSubPixel` (keys to verify in plasma-workspace): K3.45.
- kdeglobals font strings with their extras (`[General] font`, `fixed`, `smallestReadableFont`, `menuFont`,
  `toolBarFont`; the slots and keyed styles each patches, *keyed text styles per platform*): K3.46; kcmfonts `[General]
  forceFontDPI` and `Xft.dpi` into `ReaderState.font_dpi` (platform-facts' *Font DPI override*): K3.46.
- icon theme `index.theme` `*Default` sizes: K3.63.
- kcminputrc `[Mouse] cursorTheme`, `cursorSize`; key repeat (`kwin src/main.cpp:108-109`,
  `src/keyboard_input.cpp:196-209`): K3.64, K-P3-16†.
- `org.kde.KWin.TabletModeManager` D-Bus property (to verify) into `cond tablet_mode`: K3.83.
- Kirigami's `Settings.isMobile` (what it reads, to verify) into `cond mobile_form_factor`: Q3.31, K3.83 (the rules
  under it: Q1.6, Q2.39, Q2.58†, K-KA-9†).
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
- the material policy state (AcrylicBrush's `OnMaterialPolicyStatusChanged`; its inputs to verify) into
  `cond system_setting:material_policy`, and the battery saver state (API to verify) into `cond battery_saver`: W3-07.
- `UISettings.AutoHideScrollBars` into `cond overlay_scrollbars`: W1-152†.
- `UISettings.AnimationsEnabled`, `SPI_GETCLIENTAREAANIMATION`: W3-37; `UISettings.AdvancedEffectsEnabled`, registry
  `EnableTransparency`: W3-38.
- `GetDoubleClickTime` or `UISettings.DoubleClickTime`, `SPI_GETMOUSEHOVERTIME`, `SPI_GETMENUSHOWDELAY`, `SM_CXDRAG`,
  `SM_CYDRAG`, `SPI_GETWHEELSCROLLLINES`, `SPI_GETKEYBOARDDELAY`: W3-41 (its `UISettings.MessageDuration` is out,
  R1.1: the shell's notifications);
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
- `NSEvent.doubleClickInterval`, `keyRepeatDelay`, `keyRepeatInterval`, `NSButtonCell.getPeriodicDelay` (into
  `token.params.pointer_repeat.delay` and `.interval`), user defaults
  `NSTextInsertionPointBlinkPeriodOn` and `…Off`, `NSInitialToolTipDelay`: MAC-X12.
- `NSApplication.isFullKeyboardAccessEnabled` or user default `AppleKeyboardUIMode` (both to verify): MAC-X13.
- `NSWorkspace.accessibilityDisplayShouldDifferentiateWithoutColor` (already probed): MAC-X24.
- the Text size setting (14+; no public API, to verify) into `ReaderState.text_scaling_factor`; the "Show toolbar
  button shapes" setting (API or key to verify) into `cond button_shapes`: MAC-X26.
- `NSScroller.preferredScrollerStyle` into `cond overlay_scrollbars` (read today into today's scroll-bar
  field `overlay_mode`, gap-map-macos's W1-152† note): MAC-S01.
- the user's sidebar icon size (`effectiveRowSizeStyle`; user default `NSTableViewDefaultSizeMode`, to verify) into
  `cond container_size:sidebar:small` / `medium` / `large`: MAC-SB01.
- the Accessibility "Menu bar size" setting (Default or Large; API or key to verify) into `cond
  container_size:menu_bar:large`: MAC-N20.
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
  X-A11-1…X-A11-7, X-A11-10, X-DT-2, X-DT-3, X-DT-4, X-MO-10, X-MT-3; `buttonShapesEnabled` (deprecated, its
  replacement to verify) into `cond button_shapes`: X-A11-5.
- Full Keyboard Access into `cond full_keyboard_access` (no API found, X-FO-3†); the device model into
  `cond device_model:<key>` (API to verify): W-6, X-CR-4; the Pointer Animations setting into
  `cond system_setting:pointer_animations` (none found yet): P3-12†.
- Dynamic Type size per style per content size category (`preferredContentSizeCategory` into
  `ReaderState.content_size_category`, a `ContentSizeCategory`, S8): X-DT-1.
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
| `WinUiFlyout` (existing) | pop-up menus and flyouts at their anchor; whether WinUI's flyouts open to the left or right of it under the Handedness setting is to verify (W3-55†: Win32 menus follow it) | `handedness`, a member of every placement's params (S13), so every preset states it on every placement: `false` in the bases; without confirmation that WinUI's flyouts follow the setting, `windows-11` states no `handedness` rule; if the value phase confirms it, `true` only by the `windows-11` preset's rules under `cond system_setting:menu_drop_alignment`, which the reader reads from `SPI_GETMENUDROPALIGNMENT` (ADD-A04), one source; no other platform has the setting | W3-55† |
| `KeyboardAvoid(WinUiScrollIntoView)` | the focused field is scrolled into view, in its nearest scroll container, when the touch keyboard shows (`InputPane` `Showing`, `Hiding`) | none (the motion is a token, below) | W-P4-3† |
| `KeyboardAvoid(AndroidPan)`, `(AndroidResize)`, `(AndroidInsetsAnimation)` | what moves when the on-screen keyboard opens: pan the window, resize it, or move content by the insets animation | none (the strategy is chosen by `softInputMode`, below) | MAT-P4-2† |
| `MaterialScaffoldSlot(Fab)`, `MaterialScaffoldSlot(Snackbar)` | the floating action button at end or centre with its spacing, the snackbar above it and the bottom bar | the `PlacementParams` fields holding the slot's position and spacing, named by the value phase from Compose's `Scaffold` | MAT-X38† |
| `MaterialLevitatedPane` | a pane floating above the scaffold like a sheet or pop-up, over its optional scrim (`layout.scrim`, a part) | the alignment: `side`, `align` | MAT-SF04† |
| `KCommandBar` | KDE's command bar at the top centre of the window's content space; its size, the space divided by the source's divisors, is its frame's `min_width` and `max_width` (equal) and `min_height` and `max_height` (equal), `component.command_bar.frame`'s | the top offset: `edge_margin.top` | K-RL01† |
| `QtWhatsThis` | KDE's What's This popup by the pointer (`PlacementInput.pointer`; `context_help`) | none | K2.31† |
| `AppKitContextHelp` | macOS's context-help window at or near the given point (a `PlacementInput`; `context_help`) | none | MAC-P4-11† |
| `WinUiKeyTip` | an access-key badge against its element | the offset: `gap` | W2-37 |
| `UiKitContextMenu` | a context menu beside its lifted preview (the preview's rectangle, a `PlacementInput`) | none | C-13 |
| `KirigamiAddonsSearchPopup` | kirigami-addons' search popup under its field (the anchor) | none | K-KA-32† |
| `KirigamiAddonsOnboardingTip` | kirigami-addons' onboarding tip by its target control (the anchor) | none | K-KA-45† |
| `OverlayBottomCenter` | a toast at the bottom centre of its overlay (AdwToastOverlay, Kirigami's notification manager) | the distance to the overlay's edges: `edge_margin` | GN-2.12, Q2.50 |
| `CenteredInHost`, `BottomSheet` | a dialog centred in its host; a sheet attached to the host's bottom edge | `edge_margin`, `host` | W-SF10†, W-SF11†, MAT-SF01†, GN-SF18†, GN-SF19†, K-SF21†, K-SF22† |

The "Parameters" column names `PlacementParams` fields only (S13); what a strategy takes from the connector is a
`PlacementInput` (the anchor, the pointer, the keyboard's rectangle). The keyboard strategies' motion is no parameter:
WinUI's scroll into view is `token.motion.durations.scroll_into_view` and its timing (measured, W-P4-3†), and
Android's insets animation is the platform's (MAT-P4-2†); `softInputMode` is no parameter either, it picks which
`KeyboardAvoid` strategy the window's `keyboard_avoidance` value states (MAT-P4-2†).

Not placement strategies, by R1.1: the shell or compositor moving a window for the keyboard (GN-P3-9†, K-P3-21†).
The keyboard strategies take the keyboard's rectangle from the connector (`PlacementInput.keyboard`), as the
direction and the window rectangle come from it; `handedness` is a parameter, set only by a preset's rules under the
read condition (on `windows-11`, once the value phase confirms that WinUI's flyouts follow it, W3-55†), never by the
connector.

### system controls

None. No row of group 8 needs `SystemControl` (S15): the AppKit whole-control renderings are the checkbox, radio,
pop-up, disclosure and push-disclosure rows of the control groups (MAC-C06; MAC-C07, the radio,
`radio.dot.glyph` as `SystemControl(AppKitRadio)`; MAC-CB01, MAC-EX02, MAC-L10; the push disclosure, `expander @
variant @push_disclosure`, MAC-EX01, ADD-A58). `render_system_control` returns `None` for a control size AppKit lacks on the
running macOS (`extra_small`; `extra_large` before macOS 26), sizes the `macos-*` presets do not declare (S15, S7.3). The gradient fill of the
macOS 12 to 15 checkbox (MAC-X09) is a `PaintKind::Linear` and needs no rendering.

## Rename table (appendix: the merge's history)

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
| 7 | `window.tab_bar.values.presentation` (7) | `tab` variant `document` in `context window.title_bar`, its event `Move` (row 112) | presentations are widget values; the window's tabs are `tab`'s |
| 8 | `window` variant `@utility` (6) | typed `utility` | group 1 typed it (Windows' tool window, macOS's utility panel) |
| 9 | `navigation_bar.back_button` (7) | `button` variant `back` in `context toolbar.bar`; values `back_title_fallback`, `back_display_mode`; the event `@title_change` (rows 125 and the fifth check: the back label's on the `back` button, the title's on `toolbar`) | group 1 typed the back button; group 6 places iOS's navigation bar inside `toolbar` |
| 10 | `navigation_bar` for Material's floating toolbar (7) | `toolbar` variant `floating` (typed: Material and KDE draw it), parts `leading_content`, `trailing_content` | group 6: app bars are `toolbar` |
| 11 | `scroll_edge_effect` (7), `scroll_area` (8) | `scroll_view.edge_effect`; variants `soft_edge`, `hard_edge`; value `edge_effect_height_px`; the edges are `edge_effect` elements per edge (row 124) | group 3 typed `scroll_view` with this part (iOS, macOS, GNOME's undershoot) |
| 12 | `writing_tools.glow`, `writing_tools.affordance` (3) | `text_area.writing_tools_glow` (group 8's name), `input.writing_tools_affordance` | typed parts of the text widgets (macOS and iOS draw them in the app's text view); C-33's panel stays out (R1.1), its glow is carried |
| 13 | `drag_preview` (8), `list.values.drag_image` (4) | `drag_image` (7); `drag_preview.body` → `drag_image.image`; the list value → `drag_image.image.opacity`, `drag_image.values.hotspot` by rules in `context list` | one widget for the dragged image |
| 14 | `placeholder` widget (8) | `redaction`, part `mask` | `placeholder` is the text widgets' part |
| 15 | `list.rubberband` (4) | `rubber_band.body` in `context list` | group 6 typed `rubber_band` |
| 16 | `navigation_view.transition_shadow`, `navigation_view.dimming`, `sheet.dimming`, `sidebar.dimming`, `sidebar.transition_shadow` (6); `sheet.grabber`, `drawer.handle` (8) | `edge_shadow`, `scrim`, `sheet.handle`, `sidebar.handle` | one name per part kind: `scrim` as `dialog.scrim` and `role.global.scrim`, `edge_shadow` and `handle` as group 4's sidebar |
| 17 | `sidebar.body`, `sidebar.divider`, `sidebar.item`, `sidebar.item_icon`, `sidebar.values.icon_gap` (6) | `sidebar.pane`, `sidebar.separator`, `list.row`, `list.leading_icon` (variant `sidebar`), `list.values.leading_gap` | group 4: a sidebar's rows are `list` rows |
| 18 | `sidebar` variant `rail`, parts `container`, `active_indicator`, `icon`, `label`, `menu_button`, `badge` (7) | typed `compact`; `pane`, `list.row_selection_indicator`, `list.leading_icon`, `list.row` text, `button @ variant pane_toggle context sidebar.header` (row 104), `badge` in context | group 4's collapsed rail |
| 19 | `stack` (8) | `navigation_view` | the page stack (GtkStack) |
| 20 | `spin_button` (8) | `stepper` | group 6 |
| 21 | `main_window.dock`, `main_window.values` (8) | `component.dock_widget` | KDE alone: group 7's component |
| 22 | widget `frame` (6), `group_box` (4) | `group_box` | one widget; `frame` is a part of `window`, `scroll_view`, `calendar` |
| 23 | context `button_group` (2) | `layout` keyed variant `@linked`, `values.item_gap` | GTK's `.linked` box; no button-group widget |
| 24 | `checkbox.group_header`, `checkbox.values.row_spacing`, `column_spacing` (7) | `radio_group.header`, `radio_group.values.row_gap`, `column_gap` | group 2 typed `radio_group` (Windows, macOS) |
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
| 39 | `progress_bar` variant `circular` (6) | `spinner` state `!{indeterminate}` | group 3 put the determinate circular indicator in `spinner`; the eleventh check made its typed variant `determinate` the state (row 133) |
| 40 | `expander.glyph` (8) | `expander.arrow` | group 4's |
| 41 | `separator.body` (8) | `separator.line` | group 4's |
| 42 | `switch.on_off_label` (8) | `switch.on_glyph`, `switch.off_glyph` | group 2's |
| 43 | `scrollbar` variant `annotated`, part `scroll_button` (7) | `@annotated`; `arrow_button` | keyed (Windows); group 3's part |
| 44 | `input.loupe`, `inline_prediction`, `spelling_mark` (8) | `magnifier`, `prediction`, `misspelling` | group 3's |
| 45 | `text_area.find_indicator` (8); the current match `{selected}` (group 3) | `text_area.find_match @ {current}` | the current match of a find is the current item of the match sequence (`current`), not a choice among siblings (S7.2; the sixth check) |
| 46 | `input.header_inactive` (1), `input.kde_header_inactive` (4), `input.inactive_roles` (8) | `input.inactive.<set>.<key>` (`<key>` the scheme file's key snake-cased, as `input.kde_scheme.<set>.<key>` and `input.kde_scheme.<set>.inactive.<key>`; the key-to-field table under *role record*) | KDE's Inactive values, one S9 input per set and key, in one key vocabulary with the scheme inputs |
| 47 | `popover.values.anchor_gap` (3) | `popover.values.placement` (its `gap`) | group 1's placement value |
| 48 | `toast.values.duration_ms` (8) | `toast.values.durations` | group 6's keyed list |
| 49 | `combo_box.values.placement` (1) | `combo_box.values.popup_placement` | group 3's |
| 50 | `component.side_tab_bar` (4) | `component.multi_tab_bar` | group 7's (KMultiTabBar) |
| 51 | `toolbar_customization` (4), keyed `component.toolbar_editor` (6) | typed `toolbar_editor` | KDE, macOS and iOS draw it |
| 52 | `component.token_field`, its `token`, `token_menu_arrow`, `values.token_spacing` (7); the second check's `component.token_field.values.chip_gap_px` | `chip` elements in a `layout @ variant flow` in `context any input` (`chip @ context any input`, `chip.menu_arrow`, `layout.values.item_gap`) | a token is a chip (group 7's own note); the third check: one structure for the token field on macOS and Windows (W2-43†'s) |
| 53 | the glass container's merge spacing as `Material.params` (8) | `glass_container.values.merge_gap` | group 7's widget value |
| 54 | `window.content.fill` with the material `background_extension` (8, MAC-X06) | `background_extension.extension` | group 7's widget |
| 55 | `token.lengths` (4), `token.kirigami` (2), `token.opacities` (5), `token.colour_effects.frame_contrast` (2) | `token.params.layout`, `token.params.opacity`, `token.params.color.frame_contrast` | one keyed-scalar token mechanism (ADD-A44); identifiers are American (S1) |
| 56 | value `focus_visible`, `token.behavior.focus_visible`; types `FocusVisiblePolicy`, `FocusVisible`; variants `KeyboardOnly`, `AfterKeyboard`, `WithFullKeyboardAccess`, `Always` (2, 3, 8) | `focus_visible_when`, `token.behavior.focus_visible_when`; `FocusVisibleWhen { Keyboard { timeout }, AnyFocus, FocusWithin, Never }` (`timeout` an `Option`, `None` where focus stays visible); Full Keyboard Access is a rule `@ cond full_keyboard_access` | the bare name is the state flag (ADD-A16); every widget has the value (S2.11), so an entry lists it only where a row states it |
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
| 75 | `wrap_box` (6) | `layout` variant `flow` (Material's chip set too, row 126) | one flow layout (GN-2.46, GN-P3-24†, MAT-RL01†) |
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
| 87 | `card`, `chip` typed `filled`, `elevated`, `outlined` (4, 6) | `@filled`, `@elevated`, `@outlined` (on `card` only `@elevated`, `@outlined`: the filled card is the base, row 133) | Material alone, keyed on every widget (R15) |
| 88 | slider `@hue`, `@alpha`, `@value` (6) | typed `hue`, `alpha`, `value` | GNOME and Windows draw them (GN-2.53†, W2-05) |
| 89 | status_page `compact`, form `monospace` (6) | `@compact`, `@monospace` | GNOME alone (R15) |
| 90 | tab `@icon_and_label` (4) | the content variant `with_icon` | the connector sets it from the content (ADD-A61) |
| 91 | badge `@tab_bar`, `@bar_button`, `@list_row` (6) | `badge` in `context bottom_tab_bar.item`, `context toolbar.bar`, `context list.row` | a context, not a variant (TB-7, C-15) |
| 92 | `role.@*_elevated` (5) | kept as keyed roles (`role.@system_background_elevated`, …), bound by the boxes (`body`) of `dialog`, `sheet` and `popover` on `ios-*` (their roots are `layer` since the sixth check) (the `Dialog` and `Popover` sets and `Binding::Role`); `cond elevated` only for a whole elevated window | R2.3, R5.2: in a sheet or a popover the elevated level is a binding (the second check reverted the first's removal) |
| 93 | flags `has_text`, `has_value`, `opened_above`, `overlapping`, `@current`, `@editing`, `@swiped`, `@context_menu`, `@sorted`, `@overwrite`, `@composing`, `@placed_above`, `@placed_below`, `@route_active`, `needs_attention`, CamelCase flags | `!{empty}`, `!{empty}`, `placed_above`, `@overlapping`, `current`, `editing`, `swiped`, `context_menu_open`, `sorted`, `overwrite`, `composing`, `placed_above`, `placed_below`, `route_active`, `@needs_attention`, snake_case | the shared flags (S7.2, "Shared flags" below) |
| 94 | `thumb_move`, `indicator_move`, `appear` (`PresentationTransition` values); `value_transition`, `indicator_transition`, `title_shrink`, `separator_hide`, `page_transition`, `snap`, `bracket_match_animation` (`Transition` values); `Presentations` | `presentations` entries (a part's own occasion a keyed event naming it, `@indicator_move`, the third check; a thumb's slide `thumb.transitions` (group `Geometry`), the fifth; the element's own move `Move`); the moving part's `transitions`; `presentations: Vec<PresentationTransition>` | presentations and property transitions have one place each (S2.7) |
| 95 | `indeterminate_loop`, `spin` | `loop` | one name for a widget's repeating motion |
| 96 | `spinner.values.stroke_ratio` (3) | `spinner.values.stroke_width` as an expression over `element.width` | element data (R25.3) |
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
| 117 | `toolbar.handle`, `tab.scroll_button`, `input.autofill_button`, `find_bar` flag `@neutral`, `popover` `@magnifier`, `shortcuts_dialog` flag `@recording` | `toolbar.grip`, `tab.scroll_arrow`, `input.autofill_affordance`, `@warning` (reverted by row 131: `find_bar`'s flag is `@neutral`), `input.magnifier`, `button`'s `@recording` | one name per part kind and meaning |
| 118 | `group_box` keyed `@photo`, `@gray_bezel`, `@groove`, `@button` (MAC-N12); keyed `component.image` (GN-3.07) | typed `image` (`body`, `picture`; the same keyed variants) | the third check: one home for the image view's frame and shadow styles, which two platforms draw |
| 119 | `form` keyed `@collapsed`; `checkbox`, `radio` keyed `@text_button`; `badge` keyed `@custom_color`; `list` keyed `@with_subtitle`, `@subtitle_cell`, `@two_line`; `button` part `attention_dot`, value `attention_dot_offset`, flag `@needs_attention` (also on `list`, `view_switcher`; `tab`'s, which draws GNOME's attention line, stays); `list.placeholder`; `menu` and `combo_box` popup placements in `menu.values.placement`; `toolbar.values.overflow_placement`; `calendar` `@out_of_range`; `progress_bar` `@nearly_full`; the widgets' own breakpoint values (`presentation_breakpoint`, `display_mode_breakpoints`, `pane_mode_breakpoints`, `collapse_thresholds`, `card_max_width_px`, `mobile_height_breakpoint_px`) | typed `collapsed`; the content variants `with_label`, `custom_color`, `with_subtitle`; the `badge` widget, variant `attention`, in the host's context; the flag `placeholder` on `list.row`; `combo_box.values.popup_placement`, `autocomplete.values.placement`, `button.values.menu_placement`; `disabled` on a day; an expression over `element.progress`; `breakpoints: Vec<Breakpoint>` | the third check: content variants are typed, one structure per control, the host's values stay the host's, one mechanism for breakpoints |
| 120 | `code_editor.whitespace_mark` (`@space`, `@tab`, `@newline`, `@nbsp`), `misspelled_word`, `search_match`, `caret` `@bidi_secondary`; `input`'s and `text_area`'s missing secondary caret | `code_editor.invisible` (typed `space`, `tab`, `line_break`, `no_break_space`), `misspelling`, `find_match`, `secondary_caret` (on `input`, `text_area`, `code_editor`) | the third check: one name and one variant set per part kind across the text widgets |
| 121 | presentation events: `tab` `@reorder`, `list` `@reorder` (an element moved); `Move` with a part step (`segmented_control`, `bottom_tab_bar`, `view_switcher`, `tab`, `list`'s indicators; `switch`'s thumb); `slider` `Appear`/`Disappear` of `value_indicator`; `date_picker` `Appear`/`Disappear` and `time_picker` `@open` of their popup; `banner` `@reveal`/`@conceal`, `media_controls` and `component.keyboard` `@show`/`@hide`, `sidebar` and `component.tab_overview`, `component.fab_menu` `@open`/`@close`, `toolbar` `@reveal`, `@hide_on_swipe` (the element shown or hidden); `toolbar` `@open`/`@close`; `search_field` `@cancel_appear`/`@cancel_disappear` | `Move`; `@indicator_move`; the switch thumb's `Geometry` transition (the fifth check); `@value_indicator_show`, `@value_indicator_hide`; `@popup_show`, `@popup_hide`; `Appear`/`Disappear`; `@expand`/`@collapse`; the cancel `button`'s `Appear`/`Disappear` in `context search_field` | the third check: one event per occasion per widget, one name per occasion |
| 122 | `segmented_control` segment `{checked}`; `split_button` `{selected}` (Material's trailing half) | `{selected}` (the current choice among siblings, S7.2); `{open}` (the trailing half's menu) | the fourth check: S7.2's portable meanings |
| 123 | `button.activity_indicator`; `component.splitter_collapser`; `component.list_item_drag_handle`; `component.bottom_accessory`; `card` on `ios-*` (`card.values.background_inset`); `expander` `@row` | `spinner @ context button {@loading}`; `splitter.collapser`, `collapser_arrow`; `list.accessory` typed `reorder` with `list.values.autoscroll_*`; `bottom_tab_bar.accessory`, `accessory_content`, its environments `@accessory_inline`, `@accessory_expanded`; `list.row_background` with `list.values.row_background_inset`; `list.row` with `disclosure` and `nested`, or `form.row` `expander` with `row_arrow` and `row_nested` | the fourth check: one structure per control; a one-platform piece of a typed widget is a host part |
| 124 | `time_picker.flyout`, `highlight_band`, `values.item_height_px`, `field_min_width_px`, `field_max_width_px`; `picker_wheel.values.item_height_px`; `level_indicator.scale_arc`, `trail_arc`, `scale_tick`, `cap`, `values.arc_cap`; `scroll_view.values.edge_effect_edges` | `time_picker.popup` (as `date_picker.popup`), `picker_wheel.selection_band`, `picker_wheel.item.min_height`, `time_picker.field.min_width`, `field.max_width`; `level_indicator.track`, `fill` (the arcs under `circular`), `tick @ variant @scale`, `values.cap` (the ninth check: `end_cap`, with `thickness` now `stroke_width`, one pair of arc names); `scroll_view.edge_effect` and `overshoot` elements per edge (`top`, `bottom`, `start`, `end`) | the fourth check: one home per control and per fact |
| 125 | events `dialog` `@sheet_attach`, `@sheet_detach`; `list` `@insert`, `@delete`, `@reveal`; `tab` `@switch`; `bottom_tab_bar` `@selection_change`; `toolbar` `@back_label`, `@height_change`, `@separator_fade`, `@scroll_state`; `sidebar` `@labels_fade`, `@handle_slide`, `@scrim_fade` (and its flag `open`); `teaching_tip` `@contract`; `navigation_view` `@layer_push`, `@layer_pop`, `@layer_replace`; `component.context_menu_preview` `@lift`, `@dismiss` (and part `dim`); `component.list_item_drag_handle` `@drop` | `Appear`, `Disappear` (queried with the element they present); `@swipe`; `@indicator_move`; `@title_change`; the bar's `transitions`; steps of `Appear`, `Disappear`, `@collapse`, `@expand`; `@collapse`; `@push`, `@pop`, `@replace` under `@layer`; `scrim`; `Move` of the row | one event per occasion (SF-1 of the fourth check) |
| 126 | `color_button` `@light_color`; `slider.values.draw_ticks`; `tab.values.width_mode`; `toolbar.values.items_flat_by_default`; `wizard.values.button_placement`; `wizard` `@classic`; `keycap` `@on_selected`; `layout` `@chip_set`; `banner` `@sidebar`; `list` `@avatar`, `@image`, `@video`, `@three_line`; `toolbar` `@with_search`, `@with_prompt`, `ios-*` `@compact` | `gtk_contrast_pick(element.color, …)` on `swatch_ring.stroke`; `cond system_setting:slider_tick_marks`; keyed `@width_equal`, `@width_size_to_content`, `@width_compact`; `button` rules in `context toolbar.bar`; `@header_bar` alone; `kde-breeze`'s base; `context list.row {selected}`; typed `flow`; `context sidebar.pane`; typed content variants `with_avatar`, `with_image`, `with_video`, `three_line`, `with_search`, `with_prompt`; `v_size_class compact` | a look a rule can select, one route each (SF-5, SF-8, SF-9 of the fourth check) |
| 127 | widget values `teaching_tip` `min_width_px`, `max_width_px`, `tail_size_px`, `tail_margin_px`; `toast` `max_width_px`, `margins`; `status_page` `margins`, `max_width_px`; `chip` `padding_without_icon`; `banner` `heights`; `about_dialog` `margins`; `view_switcher` `item_min_width`; `component.command_bar` `preferred_width_divisor`, `preferred_height_divisor`, `top_offset_px`; `code_editor` `invisible_size_px` | the parts' fields (`body.min_width`, `body.max_width`, `tail.min_width`, `tail.margin` (`arrow` since row 130), `body.padding`, `body.padding @ !variant with_icon`, `body.min_height` rules, `item.min_width` rules, `frame.min_width` expressions, `invisible.glyph.size`); toast margins and the command bar's top offset as the placement's `edge_margin` | one value, one home (rename row 116, SF-6 of the fourth check) |
| 128 | `code_editor.caret` *(element)*, its `@vi_*` variants; `combo_box.arrow` *(element)*; `code_editor.selection`, `syntax_span`, `inline_annotation`, `annotation_border`, `minimap`, `current_line`, `calendar.weekday_label`, `form.card`, `level_indicator.tick` as plain parts | `caret` and `arrow` parts taking their widget's states (Vi's shapes the editor's variants); those parts *(element)* | the fourth check: a part with states, variants or flags of its own is an element, a mark taking its host's is not |
| 129 | overlay roots that were the box: `dialog.body`, `popover.body`, `menu.body`, `sheet.body`, `teaching_tip.body`, `search_field.body` (with the view and scrim in it), `sidebar.pane`, `component.dock_widget.title_bar` (with `frame` a part); events `slider` `@value_indicator_show`, `@value_indicator_hide`, `date_picker` and `time_picker` `@popup_show`, `@popup_hide`, `search_field` `@bar_show`, `component.dock_widget` `@gap_open`, `component.tabbed_command_bar` `@tab_change`; `time_picker` `@combo`, `@spin_box`; the KDE triggers and popups `absent = true`; `find_match @ {selected}`; `syntax_span @ {selected}`; `drag_image` `@drop_copy`, `@drop_move` | a container root `layer` (`split` on `sidebar`, `frame` on the dock widget) holding the box and what lies beside it; the parts' `transitions` (a part entering from transparent, S2.7); `Appear`, `Disappear`; `Move`; `@indicator_move`; typed `with_popup`, `text_field`; `drawn_as` (S6.1); `{current}`; `{text_selected}`; the shared `drop_copy`, `drop_move` | the sixth check: an overlay's presentation and bindings move the box, not its scrim; a part's showing is its style's; one typed distinction and one portable route for KDE's substituted parts; S7.2's meanings |
| 130 | `popover.values.has_arrow`, `menu.values.has_arrow`; flag `placeholder` on `list.row`; keyed `list` and `form` `@check`, `@radio`, `chip` `@closable`, `media_controls` `@floating`; `tab.bar` as the root; `navigation_view.page`; `teaching_tip.tail`; `dial.groove.stroke`, `dial.value_arc.stroke`, `component.activity_rings.ring.stroke`, `track.stroke`; `combo_box.arrow_area` as a plain part; `component.mdi_sub_window.frame.bindings` to `[WM]`; `form.values.column_gap_px`, `row_gap_px` for AdwActionRow's spacings; `rubber_band.body.fill @ context any window` | `arrow` absent by rules where the platform draws none (S6.3); the state flag `redacted` (S7.2); typed `check`, `radio`, `closable`, `floating`; a container root `tab.body` holding `bar` and `pane`; `incoming_page`, `outgoing_page`; `teaching_tip.arrow`; their `fill`, with `dial.values.stroke_width` and `cap`, `component.activity_rings.values.stroke_width` and `cap` (the ninth check: the arc's width is `stroke_width` on every widget); `combo_box.arrow_area` *(element)* with `arrow` in it; the keyed roles `role.@wm_*`; `leading_gap`, `trailing_gap`, `title_subtitle_gap`, `affix_gap`, `row_title_box.margin`; `variant @filled` | the seventh check: one home per fact (the arrow's absence), one flag per occasion, typed variants for shared meanings, a strip's bindings that do not reach its page, a step that names the page it moves, one name per part kind, one structure per arc, an element for a sub-control's own hover, a role for a colour group that is no surface set, a value per spacing, a selector that can fail |
| 131 | `combo_box` typed `pull_down`; `level_indicator.values.thickness`, `end_cap`, `dial.values.thickness`, `component.activity_rings.values.ring_width`; `date_picker.segment`; `time_picker.field.text`; `view_switcher.values.breakpoints` and its `@bar_layout`; `component.page_view.title_header`; `component.delay_button.body`'s button look; `component.dock_frame_drop.drop_overlay`; `component.tab_overview.values.thumbnail_width_min`, `thumbnail_width_max`; flags `find_bar` `@no_match`, `@warning`, `form` `@clickable`, `color_button` `@active`; `calendar` `@month_view`, `@year_view`, `@decade_view`; `bottom_tab_bar` `@compact_inline`; `token.params.layout.optical_centering` | keyed `macos-*` `@pull_down` (iOS's pull-down is `button` `menu`); `stroke_width` and `cap` on every arc; `date_picker.field_segment`; `time_picker.field_label`; `dialog.values.breakpoints` under the content variant `paged` setting `dialog`'s `@bar_layout`, with `dialog.values.page_switcher`; `component.title_widget` in `context component.page_view`; `button @ variant tool context component.delay_button`; `rubber_band @ variant dock_target`; `component.tab_overview.thumbnail.min_width`, `max_width`; the state flag `invalid`, `@neutral`, the content variant `activatable`, the state flag `open`; none (the shown view is the `day`, `month` or `year` parts); rules `@ h_size_class` / `v_size_class`; `token.params.optics.optical_centering` | the ninth check: one home per kind (iOS's pull-down, the dock drop zone, KTitleWidget, Breeze's tool button), one pair of names per arc and per trigger structure, flags a connector or application sets by meaning, a breakpoint on the element it measures, and one unit per params group |
| 132 | `component.selectable_label`; `level_indicator`'s part `fill`; `component.page_view.values.margins`; `component.tool_box` parts `selected_outline`, `hover_outline`; `component.tab_overview.drop_target`; `toolbar` keyed `@unified`, `@unified_compact`, `@expanded`, `@preference`; `layout` typed `page`; `dial` keyed `@circular_slider`; `autocomplete.values.trigger_prefix`; `sidebar.values.width_fraction` as GNOME's and iOS's preferred share; `component.title_widget` `@positive`; `Behavior.key_repeat_delay`, `key_repeat_interval` | typed `selectable_label` (GNOME and KDE draw it, K3.65†); `level_indicator.value`; the part `component.page_view.search_container` (its `padding`); `component.tool_box.outline`; the thumbnail's `{drop_target}` look; `toolbar` rules `@ context window variant @toolbar_unified` / … / `@toolbar_preference`; typed `clamp` (iOS's readable width) and keyed `kde-breeze` `@page`, `@qqc2_pane` (row 133); none (MAC-N15 on `dial`'s base); none (the application's setting); `sidebar.values.preferred_width_fraction` (`width_fraction` is Kirigami's cap only); none (KTitleWidget has no positive type); `Behavior.key_repeat: Option<KeyRepeat>` | the tenth check: a kind two platforms draw is typed (R14), a part is not named after a field, one window property has one variant name, one meaning per typed variant and per value, one home per fact (R20) |
| 133 | `dialog.values.type_icons`; `dialog.values.standard_button_icons: Vec<(ActionRole, Key)>`; `window.values.tab_bar_overflow`; `window.values.window_menu_items`, `caption_button_menus` as `Vec<Key>`; `slider.fill`, `slider.halo`; `text_area.values.resize`, `Resize`, `min_rows`; `text_area` typed `line`, keyed `@bezel`, `@groove`; `spinner` typed `determinate`, its arc as a band, `values.cap`, Material's `track_stroke_width`; `scroll_view` typed `flat`; `card` keyed `@filled`; `layout` keyed `@pane`; `layout` typed `form` and its `form_*` values; a line's insets as `padding` (`menu.separator`, `segmented_control.separator`, `tab.separator` with `tab.values.separator_inset`, `sidebar.values.separator_inset`, `list.values.row_separator_inset`, `form.row_separator`, `date_picker.segment_separator`, `time_picker.segment_separator`, `breadcrumb.bottom_separator`, `split_button.divider` with `values.divider_margin_px`); `date_picker`'s Material `header`; `date_picker` keyed `@ka_date_popup`, `time_picker` keyed `@ka_time_popup`; `color_picker.crosshair`; `color_button.values.swatch_inset`; `font_chooser` keyed `@sunken_label`, `requester_separator`; `autocomplete.body.padding`; `file_chooser.values.gap_px`; `chart.legend.text`; `GroupBoxTitlePosition::None`; `component.tab_overview` event `@thumbnail_fade`; `button` keyed `@unpin`; KDE's keyed text style `taskbar`; `windows-11` durations `normal`, `fast`, `faster`; `component.page_preview` | `dialog.icon.glyph @ variant <type>`; `Vec<(Key, Glyph)>`; `tab.values.overflow @ variant document context window.title_bar`; `Vec<MenuEntry>`; `slider.value_fill`, `slider.state_layer`; none (R1.6: Compose draws no resize handle, `minLines` is the application's); `scroll_view`'s own variants; `spinner` state `!{indeterminate}`, `indicator.glyph` (S2.7, S2.9), `values.spoke_cap`, `values.stroke_width`; the borderless base; the filled card as the base; `@qqc2_pane`; the `form` widget (`label_alignment`, `baseline_alignment`, `field_growth`, `contents_alignment`, `collapse`; typed `columns`); the part's `margin`; `popup_header`, in `popup`; `with_popup` under `cond flavor:kirigami`; `crosshair_line_horizontal`, `crosshair_line_vertical`, `crosshair_band_horizontal`, `crosshair_band_vertical`, `selection_ellipse`; `body.padding`; `font_label` and `button @ context font_chooser`, none; none (the placement's `gap`); none; `legend_label.text` in the `legend_entry` element; none (an untitled box is content); the flag `@morph_target` with `Opacity` transitions; the part `component.tab_overview.unpin_icon`; none (the panel is the shell's, K3.47); `control_normal`, `control_fast`, `control_fast_after`, `control_faster`; typed `page_preview` | the eleventh check: one home per fact, a `Key` only for key-grammar names, the core's caret and arc routes (S4.2, S2.7), R1.6's reference, a stroke lies on its box's edge (S2.4), a portable selector for a flavour's picker, a nested widget is not a part, two platforms draw the page preview (GN-2.57†, R14) |

## Where group 6's §4.2 kinds go (appendix: its index, reconciled)

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
| pull-to-refresh | Windows, KDE, Material, iOS (macOS from 27: `docs/todo.md`, D12) | typed `pull_to_refresh` |
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
| edit menu | KDE, Material, iOS, Windows (CommandBarFlyout, W2-23), GNOME (the touch-selection bubble, GN-1.286†, GN-1.184) | typed `edit_menu` |
| file chooser | GNOME, KDE (GTK's own chooser, KFileWidget); GNOME's portal chooser is out (R1.1); Windows' Common Item Dialog is another toolkit's (R1.3) | typed `file_chooser` |
| find bar | macOS, KDE, iOS | typed `find_bar` |
| font chooser | GNOME, KDE (Win32 `ChooseFont`: another toolkit's, R1.3) | typed `font_chooser` |
| group box/frame, image frame | GNOME, iOS, KDE, macOS | typed `group_box` (group 6 wrote `frame`; a frame with an optional title); the image view's frame and shadow styles are the typed `image` (macOS's MAC-N12, GNOME's GN-3.07: the third check) |
| menu button | macOS, Windows, Material | drawn inside `button` (the typed variant `menu` and its `menu_indicator`); a toolbar's tool buttons are `button` variant `tool` in `context toolbar.bar` |
| rubber band | macOS, KDE, iOS, GNOME (GN-1.175, GN-1.280†, GN-SF14†) | typed `rubber_band` |
| shortcuts dialog | GNOME, KDE (iOS's discoverability HUD: out, R1.1) | typed `shortcuts_dialog` |
| time picker | Windows, KDE, Material | typed `time_picker` |
| toast/snackbar | KDE, GNOME, Material | typed `toast`; iOS's editing-gesture HUD is the keyed `component.editing_hud` (C-49†) |
| wizard | GNOME, KDE (Windows' Aero wizard is Win32: preset, R1.3) | typed `wizard` |
| button group/toggle group | KDE (Q2.3 is a DelayButton), Material | Material's button group is drawn inside `segmented_control`; KDE's DelayButton is a one-platform kind: keyed component `delay_button` |
| composite dialogs | GNOME, KDE | in where the toolkit draws them in the application's process (R1.1): GNOME's GtkPrintUnixDialog, GtkPageSetupUnixDialog and GtkAppChooserDialog where the application builds them or GtkPrintDialog finds no Print portal (`gtkprintdialog.c:1502-1508`, GN-2.57†): a `window` with the content variants `print`, `page_setup`, `app_chooser` holding a `dialog` with standard widgets, GtkDialog's action area `dialog`'s; KDE's QPrintDialog, QPageSetupDialog, KOpenWithDialog, Qt/KIO widget dialogs: `dialog` with standard widgets (K-P3-9†); the page preview both print dialogs draw is the typed `page_preview`. Out where a portal draws them in its own process (R1.1): xdg-desktop-portal-gnome's (GN-P4-3†), xdg-desktop-portal-kde's in a sandbox (K-P3-9†) |
| editable label | GNOME, iOS | drawn inside `input` (GNOME's cross-check: GN-2.45 belongs to `[input]`) |
| emoji chooser | GNOME (KDE's plasma-emojier is a separate Plasma program: out, R1.1) | keyed component `emoji_chooser` |
| FAB | KDE (RoundButton), Material | drawn inside `button` (a round button and Material's FAB styles are button variants); Material's FAB menu is a keyed component `fab_menu` |
| full-screen dialog | KDE, Material | drawn inside `dialog` |
| help button | macOS, KDE | drawn inside `button` (the typed `help`) |
| icon button | Material, iOS | drawn inside `button` |
| image cropper | macOS (ImageKit), Windows (CTK) | typed `image_cropper` |
| ink toolbar | Windows, iOS | typed `ink_toolbar` |
| inspector/utility panel | macOS, iOS, KDE | drawn inside `window` (HUD and utility panels) and `sidebar` (the trailing inspector column: its typed `end`; Kirigami's context drawer, Q1.66); KDE's dock widget's own title bar and buttons are `component.dock_widget` (K2.10, its Kind line) |
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
