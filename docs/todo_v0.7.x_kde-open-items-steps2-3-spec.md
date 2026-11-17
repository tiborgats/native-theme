# 0.7.x: issue #4's open KDE items, steps 2 and 3 (the focus ring; a button's border on hover and focus) — specification

- Rationale: [`issues/0004_kde-open-items-rationale.md`](issues/0004_kde-open-items-rationale.md)
  ("the rationale"), decisions D4 (step 2) and D5 (step 3), with what D0 and D10 say
  every step carries. Not approved when this was written; nothing here is done before
  the maintainer approves the rationale, this specification and its plan. It decides
  nothing the rationale did not decide, except the small points under *Decisions taken
  while specifying*; what needs a decision the rationale does not contain is under
  *Open points*.
- It continues [`issues/0004_kde-hand-overrides-rationale.md`](issues/0004_kde-hand-overrides-rationale.md)
  ("the first rationale", §5.1, §5.4) and the backlog at `docs/todo.md:1767-1933` and
  `:2742-2764`. Form: [`archive/todo_v0.7.1_kde-hand-overrides-spec.md`](archive/todo_v0.7.1_kde-hand-overrides-spec.md).
  Step 1 has its own specification, [`todo_v0.7.1_kde-open-items-step1-spec.md`](todo_v0.7.1_kde-open-items-step1-spec.md).
- Written: 2026-11-14, against `v0.7.1-rc2` at `0fa28027`. Every `file:line` is that
  commit's unless it names another. Step 1 lands before these steps and edits some of
  the same files (`kde/colors.rs`, `kde-breeze.toml`, the gpui connector's `colors.rs`);
  it adds no line to `docs/platform-facts.md` (its C.1). So every edit below is placed by
  its quoted text, its function or its key, and a line number is where it stood at
  `0fa28027`.
- **How an edit is applied.** By its anchor: the quoted text, the key, the function's
  or the test's name. Line numbers are `0fa28027`'s and serve to find the place; the
  steps before these (step 1, and the egui 0.37 release D10 proposes before them) move
  them. A count stated here (egui's mapping totals, the leaf count) is today's value
  with this step's delta, and the implementer reads the base from the tree. A citation
  this specification writes into a file (a comment, a doc, a facts cell) names the
  document, with its section, row or a quoted phrase where useful, never a line of a
  tracked file that changes (S6, 2B.6); the existing line citations are left as they
  are. The one exception is what the gpui showcase's tests read by line, its colour
  claims and `HOW` texts (*Lines that must not move*): there a citation is given as
  "the line of" a named statement, and the implementer writes that statement's line in
  the tree it edits. Citations of
  egui's sources (`EG/…`) are read against egui 0.36.2 and are re-read against the egui
  version the tree then has.
- Abbreviations, as in the rationale: `GC`, `GB`, `GP` are the `src/` folders of
  gpui-component 0.9.1, gpui-base 0.9.1 and gpui-pre 0.3.12 in Cargo's registry; `IW`
  iced_widget 0.14.2's, `IC` iced_core 0.14.0's, `IR` iced_runtime 0.14.0's; `EG` egui
  0.36.2's; `CG`, `CI`, `CE` the `src/` folders of the gpui, iced and egui connectors;
  `PF` is `docs/platform-facts.md`; `P8` the checkout of the parked branch `v0.8.0-rc1`
  at `78ee0efd`, `F` its `docs/facts/`.
- **Version and release (D0) are the maintainer's.** This text is written for the
  rationale's proposal, D0 (a): a 0.7.x release that names its break, and, in D10's
  proposed order (step 1 in v0.7.1, then egui 0.37), the release is 0.7.3, on the branch
  `v0.7.3-rc1`. **Steps 2 and 3 are one release** (the maintainer's decision of
  2026-11-15, `docs/todo_v0.7.1_batch-speed-rationale.md`, S5): one branch, one version,
  one CHANGELOG section, one ROADMAP section, one close-out, one asset run and one
  archive at the end; each step keeps its own units, trial, reviews and record, the
  release's texts state what both steps bring, and under D0 (a) the break is named once.
  So "<the release that brings step 2>" and "<the release that brings step 3>" below
  name the same release. Where a number appears below it is that proposal's, and a
  placeholder until the maintainer decides D0 and the order: the batch's proposed
  numbers (D10's order with S5's pairs) are 0.7.1 the GPUI Kit completion and step 1,
  0.7.2 egui 0.37, 0.7.3 steps 2 and 3, 0.7.4 steps 4 and 5, 0.7.5 the widget icons,
  0.7.6 the caret, 0.7.7 steps 6 and 7. What differs under (b) and (c) is in step 2's
  *Order and gates* (2K), which holds for the release.
- **The batch's working rules** are cited as "rule N (batch-speed rationale, Appendix)",
  the consolidated list at the end of `docs/todo_v0.7.1_batch-speed-rationale.md`; below,
  "rule N" is that list's.
- **The gate cadence** is the batch's (`docs/todo_v0.7.1_batch-speed-rationale.md`, S1;
  `--quick` as `docs/todo_v0.7.1_gate-speed-spec.md` defines it), 2K's *Units*.
- Every value below was read today from the file named, or computed today, and says
  which. What could not be verified is marked **not verified**, with the reason.

## How to read it

Two steps, each complete in itself, in one release (S5): step 2's units, then step 3's,
on one branch, then one close-out. Step 3 builds on step 2's tree but on nothing step 2
decides.

| Part | Rationale | Files it owns |
|---|---|---|
| 2A — the model field | D4 (b) | `native-theme/src/model/defaults.rs`, `model/resolved.rs`, `resolve/validate.rs`, `resolve/validate_helpers.rs`, `model/mod.rs` (tests), `native-theme/src/resolve/tests.rs` (2A.4's new test), `native-theme/tests/proptest_roundtrip.rs` |
| 2B — facts, rules, registry, presets | D4, "The values" | `docs/platform-facts.md`, `docs/inheritance-rules.toml`, `docs/property-registry.toml`, `docs/showcase-elements.toml`, the four presets |
| 2C — the KDE reader and the portal's accent | D4, "The values" | `native-theme/src/kde/colors.rs`, `native-theme/src/kde/mod.rs` (its two lists and the accent test's list), `native-theme/src/gnome/mod.rs`, `native-theme/tests/reader_kde.rs` |
| 2D — every reader of the ring colour, before and after | D4 | none (a table the other parts are checked against) |
| 2E — gpui | D4, gpui | `connectors/native-theme-gpui/`, the gpui `focus_ring` entry of `scripts/connector_parity_exceptions.toml` (2E.3) |
| 2F — iced | D4, iced | `connectors/native-theme-iced/` (connector and showcase) |
| 2G — egui | D4, egui | `connectors/native-theme-egui/`, `connectors/native-theme-egui-widgets/src/tests.rs` |
| 2H — the showcases | D4, "A consequence…" | the three showcases' ring statements, tests and screenshots |
| 2I — outside the code | D10 | `CHANGELOG.md`, `ROADMAP.md`, `README.md` and `native-theme/README.md` (2I.2a), the first rationale, the upstream requests, `docs/todo.md`, `docs/archive/todo_closed.md`, the rationale's status line (2I.6) |
| 2J — the fallback | D10's last bullet | — |
| 2K — order and gates | D0, D10 | — |
| 3A–3I | D5 | as step 2, for `ButtonTheme`'s two fields and the gpui trial (`native-theme/src/kde/mod.rs`' lists among them, 3C); in iced also `CI/contract.rs` (the two tests, 3D.3); in egui also `CE/contract.rs` (T10's formulas, 3E.4) and `CE/style/derived.rs` (a test, 3E.3); `docs/showcase-exceptions.toml` (the hover run's exception, 3H.2b); at the close-out `docs/archive/README.md`, the move of this specification and its plan, and the links to them (3H.6) |

**Lines that must not move** (the gpui showcase's `every_colour_claim_is_read_at_the_line_it_cites`,
`CG/showcase.rs:2819`, opens each colour claim at its cited line). At `0fa28027` the
claims cite, in `CG`: `colors.rs` 281–718, `geometry.rs` 230–772, `variants.rs` 62–64,
`widgets/mod.rs` 344, `widgets/checkbox.rs` 123–145, `widgets/switch.rs` 94–144,
`widgets/slider.rs` 77–80, `widgets/tab_bar.rs` 217–239, `widgets/spinner.rs` 93–466,
`widgets/progress.rs` 77–79, `widgets/separator.rs` 30 (recomputed today from every
`native-theme-gpui/<file>.rs:N` in `examples/showcase-gpui/info/*.rs`; an earlier
version gave spinner 49, which is an egui citation). Every edit below in these files
is either a line replaced by one line, or placed after the claimed lines; none needs a
re-point. Per file:
- `colors.rs`: the two functions after `:788` (2E.1); `:223` and the import at `:13`
  one line for one (2E.1).
- `widgets/mod.rs`: `FocusVisible` and `ring_shows`, with its doc, after `:400`
  (2E.2, 2E.3); `FocusRing`'s doc above `:344` is not touched.
- `widgets/checkbox.rs`, `widgets/switch.rs`, `widgets/slider.rs`: the lines of 2E.2,
  all after `:251`, `:366` and `:325`, past the claims; `track`'s doc (`:444-446`)
  lies after `:144`.
- `variants.rs`: `bordered_button` and then `hover_border` after `switch_color`, before
  `:95` (3F.1, 3F.4), past `:62-64`; the trial's test is not in this file (3F.2). From
  unit 3.1 on, where the trial passes, `ButtonKind::Bordered`'s claims (3F.4) cite four
  lines of these two functions: `bordered_button`'s `.foreground(theme.button_foreground)`,
  `.hover(theme.button_hover)` and `.active(theme.button_active)`, each on a line of
  its own as in `ghost_button` (`:62-64`), and the line of `hover_border` that reads
  `n.resolved.button.hover_border_color`. They join this list, so `hover_border` comes
  after `bordered_button`, and a later step adds to `variants.rs` only after
  `hover_border` or re-points the four claims in the same commit.
- `geometry.rs`, `tab_bar.rs`, `spinner.rs`, `progress.rs`, `separator.rs`: not
  edited.

## Decisions taken while specifying

None changes a result of the rationale; each is the smaller or the safer choice.

1. **2A.** `ResolvedDefaults` carries the field as it is, `Option<bool>`: `None` is a
   meaning ("what the connectors do today"), not a gap, so `validate` passes it through
   rather than requiring it. The `validate_defaults!` macro gains a group for such
   fields (2A.3).
2. **2B, 3B. Facts rows.** A new row goes beside the row it belongs with, in the
   file's own form; the lines after it move, and their existing citations are left
   to drift (S6, 2B.6). Nothing reads `docs/platform-facts.md` by line number once
   v0.7.1's S6 unit has made its three gates stop doing so (2B.6), so no row is
   placed to keep lines in place.
   - Step 2: `focus_ring_keyboard_only` goes after `focus_ring_offset` (`PF:1110`),
     the row it belongs with, the table's last.
   - Step 3: `hover_border_color` and `focus_border_color` go directly before
     `hover_background` (`PF:1192`), the button's other hover row.
3. **2C.** The KDE ring colour is computed in one place, a `pub(crate)` function of
   `kde/colors.rs`, which the reader and the portal path both call, so the alpha is
   written once; the portal path's KDE accent becomes a function of its own in
   `gnome/mod.rs`, so it is tested without a portal (2C.2).
4. **2E–2G.** "The focus followed a key" uses the same classes of event in the three
   connectors, gpui's own (`GP/window.rs:5874-5879`): a key press is a key; a pointer
   move, a pointer button press and a touch are the pointer; every other event (a key
   release, a modifier change, a scroll, a button release, a window event) leaves the
   answer as it is. A modifier key pressed alone is not a key: gpui receives it as a
   modifier change (on Wayland a key press reaches gpui as a key only where the keysym
   is no modifier, gpui-pre-linux 0.3.12 `src/linux/wayland/client.rs:2183`, and a
   modifier change is `PlatformInput::ModifiersChanged`, `:2141`); egui-winit sends a
   modifier key as an `Event::Key` press of one of egui's eight left and right
   `Shift`, `Control`, `Alt` and `Super` keys (egui 0.36.2 `src/data/key.rs:192-221`,
   egui-winit 0.36.2 `src/lib.rs:1640-1651`, `:1019-1045`; 0.37.0 `key.rs:253-274`,
   egui-winit `lib.rs:1128`, `:1797`), and the plugin leaves those out by name (2G.1);
   and iced, whose winit conversion
   makes every key a `KeyPressed`, Shift and Control included (iced_winit 0.14.0
   `src/conversion.rs:291-302`), leaves them out by name (2F.1).
5. **2E.** One predicate, "gpui-component's band is on", decides both
   `Theme::focus_ring` and the `ring` token's alpha, so the two cannot disagree (2E.1).
6. **2G.** The plugin keeps its bookkeeping in one map per viewport, of a small struct
   (2G.1), as it keeps `filled` and `last_sent` (`CE/plugin.rs:312-325`), and drops a
   closed viewport's entry where it drops theirs: egui's focus is per viewport, and an
   immediate viewport runs its whole pass, hooks included, inside its parent's
   (`plugin.rs:318-320`), so one shared record would mix the two viewports' inputs.
7. **2B, 3B.** `docs/showcase-elements.toml` lists the new leaves on the elements that
   draw them, so each showcase's Widget Info gives their route (2H.1, 3G.1).
8. **3D.** In iced every button style that wears `button.border` takes the hover border:
   `button`, `button_toggle` (through `button`), and the four class buttons
   (`button_primary`, `button_danger`, `button_success`, `button_warning`). The
   rationale says "the button styles"; the four share one border row with `button` in
   the contract (`CI/contract.rs:448-454`, `native_button_border`), so excluding them
   would split that row in two.
9. **3E.** The egui base and «button» role write the two strokes after `write_states`,
   as the «input» role writes its own (`CE/style/roles.rs:349-352`), rather than through
   a new kind of `BorderSource`.
10. **3D.** iced's `button.focus_border_color` is an `UNREAD_COLOURS` entry with
    `Unread::NoReceiver` (`CI/contract/derived.rs:1277-1344`): an iced button takes no
    keyboard focus. Weighed against an `UNREACHABLE` entry, as `dialog.entrance`'s
    (`:1076-1110`): that form proves the loss per combination through `lost` and needs
    an exception for each combination that states none, 32 today, one fewer or more
    each time a preset starts or stops stating the field; `UNREAD_COLOURS` is the form
    iced already uses for a colour leaf with no receiver, the one 3F uses for the same
    leaf in gpui, and needs no change when a preset starts stating it. Its test,
    `every_colour_leaf_reaches_a_row_or_is_named` (`CI/contract.rs:5578`), asks of an
    entry only that it names a colour leaf no row reads (the list's doc,
    `derived.rs:1335-1337`), which holds in every combination.
11. **2B.** The Material facts cell is tagged **(impl)** with the Compose source only.
    The measurement behind the parked fact (the Android host run 42307873328) lives on
    `v0.8.0-rc1` and is not carried over; the cell names it as such, without a link.
12. **3F.** The trial's pass criterion is the strict one. The rationale names three
    checks (the rest fill, the rest border, the text colour of each state); 3F.3
    compares every state (hover and press fill, disabled, focused, selected as well)
    and passes only when all are equal to the default button's but the hover and press
    border. The other way, shipping a variant that differs from the default button in
    one more state and saying so, would trade a state the connector draws from the
    theme today for the hover outline, against the rule that a connector follows the
    theme's data. A variant that fails in such a state takes the rationale's failure
    branch (no receiver, the upstream request), and the trial's record names the state,
    so the maintainer sees what stood in the way. The criterion is now the rationale's
    own (D5), so the maintainer approves it there, with the rationale.
13. **3E. The other roles keep their rest stroke.** A role cell is the base style
    copied, then the role's writes (`CE/style/roles.rs:19-56`), and seven roles write
    no border (`border: None`): «scrollbar» (`:532`), «slider» (`:559`), «sidebar»
    (`:685`), «toolbar» (`:724`), «status bar» (`:755`), «list» (`:804`), «switch»
    (`:858`). With the base's new strokes they would take the button's hover and focus
    outline into their hovered and pressed states, a toolbar button's and a list row's
    among them, which the rationale gives toolbar buttons in step 5 (D7) and leaves to
    the backlog for a list row and the slider handle (§0). So in step 3 those seven keep
    the stroke they have today (3E.1). The form that touches least: one statement in
    `role_cell`, before `derived::apply_role`, for the roles a named predicate,
    `keeps_resting_stroke`, lists, rather than an edit in each role's function or a
    change of what `write_states` does with `border: None` (the base's own scroll-bar
    write passes `border: None` too, `CE/style/base.rs:378`, `:389`, and would undo the
    base's strokes). The predicate is a function of its own so that step 5 changes one
    place. **What step 5 changes here** (the steps 4–5 specification's B.7 says the
    same): `keeps_resting_stroke` no longer holds for `Role::Toolbar` where
    `toolbar.button_hover_border_color` is stated (the function then takes the theme
    as well); `["button.border.color"]`'s two «toolbar» sinks of 3E.2 gain a `when`
    ("toolbar.button_hover_border_color is None");
    `flat_and_part_scopes_keep_their_resting_stroke` and
    `panel_role_cells_write_their_rows` (3E.3) are narrowed to match; and step 5's T10
    formula for the toolbar's hovered stroke falls back to the cell's rest stroke, not
    to what the cell inherits from the base.
14. **2G. egui's one-pass lag, and the rule that covers it.** egui applies a focus
    move at three different times. Tab moves it inside the pass, as the widgets
    register (`Focus::interested_in_focus`, `EG/memory/mod.rs:647-689`), so the
    plugin's `on_end_pass` of the same pass sees it. Shift+Tab from a focused widget
    stores its target in `id_next_frame` (`:669`), which the next pass's
    `Focus::begin_pass` applies (`:573-575`). The arrow keys move it in
    `Focus::end_pass` (`:623-628`), which `Memory::end_pass` calls (`:809`) from
    `ContextImpl::end_pass` (`EG/context.rs:2673`), after the plugins' `on_end_pass`
    (`:813`). So the plugin sees the last two at the end of the next pass, whose input
    may hold a pointer move or a click. In egui 0.37.0, which D10 puts before this
    step, the same holds: its `Focus::begin_pass` applies `id_next_frame`
    (`src/memory/mod.rs:572`), and `run_pass_dyn` runs `on_end_pass` inside the pass
    (`src/context.rs:990-994`), whose `end_pass` comes after it (`:1111-1113`). The
    rule (2G.1): a focus change follows a key where this pass's input held no pointer
    button press and no touch, and either a key was the last input, or this pass's
    input held a key press, or the previous pass's did. A frame can run more than one
    pass (`EG/context.rs:836-…`, a discarded pass runs again), and a pass after the
    first gets no events (`RawInput::take` moves them, egui 0.36.2
    `src/data/input/raw_input.rs:124-143`), so there "the previous pass" is the one
    that had them. Tab is delayed too where it leaves the last focusable widget: the
    focused widget hands the focus on (`give_to_next`, `EG/memory/mod.rs:666`) and the
    next widget to register takes it (`:660-662`), which, past the last one, is the
    first widget of the next pass; `key_before` covers it as it covers Shift+Tab.
    Its costs, beside decision 4's (a focus the application moves after a key shows
    the ring): `read` changes nothing at a pointer release and keeps `key_last` from
    pass to pass, so **a key pressed while a pointer button is held rings a widget
    that takes the focus at the release** (a drag value, `EG/widgets/drag_value.rs:665-667`)
    **where no pointer move came after the key, or where the key came in the
    release's pass or the pass before it**: a press in pass N, a key alone in N+1, a
    pass with no events in N+2 (a repaint) and the release in N+3 (`key_last` alone);
    a press in N, a key alone in N+1 and the release in N+2 (`key_last` and
    `key_before`); a press in N, then a key and the release in N+1 (`key_last` and
    `key_now`); a press, a key and a pointer move in N, then the release in N+1
    (`key_before` alone: `key_now` of N, with `key_last` cleared by the move). Where a pointer move (a jitter within
    the click threshold) follows the key and the release comes two or more passes
    after the key's pass, `key_last` is cleared and `key_now` and `key_before` are
    false: no ring. In one pass: a Tab and a pointer press together draw no ring
    for the focus the Tab moved (`pointer_now`). And a click in the pass after a
    Shift+Tab, an arrow key or a Tab past the last widget counts that focus move as
    the pointer's. No mechanism change is made for these: each needs a key pressed
    during a drag, or a key and a click in one pass or in two passes in a row.
15. **3C. The portal's accent does not recolour the two button borders.** The line
    runs between the semantic roles `apply_accent` sets (the accent, the selection, the
    ring after step 2, the tab indicator) and each widget's own reads of the selection
    colour (the check box's border, the blends, and now the two button borders). The
    reader writes `button.hover_border_color` and `button.focus_border_color` from
    kdeglobals' `[Colors:Selection] BackgroundNormal`, and `apply_kde_accent` leaves
    them as they are, as it leaves the reader's widget reads of it; the other way,
    setting them to the accent too, gives the same colour wherever the accent is the
    scheme's selection colour and would make them the one widget's own read of the
    selection colour the portal moves (3C).

## Corrections and additions to the rationale found while specifying

No decision changes; each is recorded so the plan and the review see it.

1. **D5, the gpui custom variant.** A custom variant has no border width of its own:
   gpui-component gives the four edges a width only for the `Default` variant or an
   outlined button (`GC/button/button.rs:724-729`). With `geometry::button`'s refinement
   the button gets `button.border.line_width` (`CG/geometry.rs:229`), so the trial
   builds on it as the rationale says. A selected and a disabled button replay the
   caller's refinement over upstream's state style (`GC/button/button.rs:821-836`), so
   a rest fill the caller gives the variant would win in those states; the connector's
   buttons already apply `geometry::button_checked` and `geometry::button_disabled` there
   (`CG/geometry.rs:1328-1380`), and the trial does the same (3F.2). `button_disabled`
   sets a fill in every theme (`:1347-1349`), `button_checked` only where the theme
   states `checked_background` or `active_background` (`:1374-1377`; macos-tahoe states
   neither), so the rest fill goes on an unselected button only (3F.1), and a selected
   one keeps upstream's selected fill, the variant's `active` (`GC/button/button.rs:1337`)
   where the `Default` button has `tokens.button_active` (`:1327`).
2. **D5, the text colours.** No frame read can give the text colour: gpui-pre exposes
   `painted_quads` and `painted_underlines` only (`GP/window.rs:3112-3118`). A custom
   variant letters every enabled state in its `foreground` (`GC/button/button.rs:1080`,
   the hovered and pressed styles take `text_color`, `:1223-1228`, `:1299-1303`), as the
   `Default` variant letters every state in `button_foreground` (`:1034`); the trial
   compares those values (3F.2).
3. **D4, what else becomes false.** Statements and tests the rationale does not list
   that the step makes false, each handled in its part: the iced connector's "KDE's +2"
   (`CI/lib.rs:1533`, `CI/styles.rs:575`), since kde-breeze's offset becomes −1 (2F.6);
   iced, egui-widgets and gpui tests that focus a control under windows-11 or kde-breeze
   by a click or by the program and expect a ring (2E.4, 2F.2, 2G.4, 2H.2); the gpui
   showcase's four unconditional "focus ring, ring at 50%" claims and the `[1.0]` backlog
   entry that waits for a preset with the band off (`docs/todo.md:3315-3320`) (2H.3);
   the egui plugin's doc that names gpui's rule as `focus_ring = width > 0.0`
   (`CE/plugin.rs:15-19`) (2G.1).
4. **D4, the caret rule.** `[per_platform."input.caret_color"]`'s `kde =` value is
   documentation only: `resolve()` fills an unstated caret from `defaults.text_color` on
   every platform (`native-theme/src/resolve/inheritance.rs:126-135`), and kde-breeze
   states no `input.caret_color` (no such key in `kde-breeze.toml`, read today). The rule
   changes as D4 says (2B.3); the preset's caret is recorded under *Problems found*.

## Open points for the maintainer

1. **The version (D0)**, of the one release: (a) as written; (b) and (c) as 2K's
   *Order and gates* says.
2. **The asset run** of the release needs the maintainer's desktop unlocked and
   untouched for 4 to 6 minutes, once, near the end.

---

# Step 2 — D4: the focus ring

## Part 2A — The model field `defaults.focus_ring_keyboard_only`

### 2A.1 `native-theme/src/model/defaults.rs`

After `focus_ring_offset` (`:120-122`), in the `// ---- Focus ring ----` group:

```rust
    /// Whether the platform shows its focus ring only for a focus that came
    /// from the keyboard (`true`), or for any focus (`false`). `None` where the
    /// theme does not say: a connector then draws the ring on any focus.
    /// docs/platform-facts.md §2.1.5.
    pub focus_ring_keyboard_only: Option<bool>,
```

No `#[serde(rename)]`: the TOML key is the field's name. The struct's
`#[serde_with::skip_serializing_none]` (`:34`) leaves it out of a written theme when
`None`, as every other option.

- `impl_merge!` (`:153-168`): add `focus_ring_keyboard_only` to the `option { … }` list
  after `focus_ring_offset`. The macro generates `merge` and `is_empty` from that list,
  so a field missing there would neither merge nor count for `is_empty`.
- Test `default_has_all_none_options` (`:182-212`): `assert!(d.focus_ring_keyboard_only.is_none());`
  after the three focus-ring lines.
- New test `merge_keeps_and_overrides_focus_ring_keyboard_only`: a base with `Some(true)`
  merged with an empty overlay keeps `Some(true)`; an overlay with `Some(false)` wins.
- New test `toml_round_trip_focus_ring_keyboard_only`: `ThemeDefaults` with `Some(true)`
  writes `focus_ring_keyboard_only = true` and reads back equal; with `None` the key is
  absent from the written TOML.

### 2A.2 `native-theme/src/model/resolved.rs`

`ResolvedDefaults` (`:74-152`, `#[non_exhaustive]`, so adding a field is not a break):
after `focus_ring_offset` (`:146-147`):

```rust
    /// Whether the platform shows its focus ring only for a keyboard focus;
    /// `None` where the theme does not say. See
    /// [`ThemeDefaults::focus_ring_keyboard_only`](crate::model::ThemeDefaults::focus_ring_keyboard_only).
    pub focus_ring_keyboard_only: Option<bool>,
```

`#[derive(serde::Serialize, serde::Deserialize)]` without `skip_serializing_none`: a
`None` serialises as `null`, as every soft option of the widget structs does; a missing
key deserialises as `None` (serde's rule for an `Option` field). Test helper
`sample_defaults` (`:343-385`) gains `focus_ring_keyboard_only: None,`.

The struct's doc (`:70-72`), "Fully resolved global theme defaults where every field
is guaranteed populated. … Mirrors [`crate::model::ThemeDefaults`] but with concrete
(non-Option) types.", becomes false and is reworded: "Fully resolved global theme
defaults: every field is populated but `focus_ring_keyboard_only`, a soft option whose
`None` is a meaning (the theme does not say). Mirrors [`crate::model::ThemeDefaults`]
with concrete types for every other field."

### 2A.3 `native-theme/src/resolve/validate_helpers.rs` and `validate.rs`

`validate_defaults!` (`validate_helpers.rs:666-741`) requires every field it lists and
has no group for a field that may stay `None`. Add a sixth group after
`icon_sizes { … }`:

- matcher: `soft { $($soft_field:ident),* $(,)? }`;
- expansion, after Pattern D: for each, `let $soft_field = $src.defaults.$soft_field;`
  (the type is `Copy`; nothing is pushed to `$missing`), with the comment "Pattern E:
  soft options, `None` a meaning of their own, passed through";
- in the `ResolvedDefaults { … }` literal (`:728-738`): `$($soft_field,)*` after
  `$($f32_field,)*`.

`validate.rs:46-89`: the call gains `soft { focus_ring_keyboard_only }` after
`icon_sizes { toolbar, small, large, dialog, panel }`. No range check
(`check_defaults_ranges`, `validate_helpers.rs:769`, is unchanged).

No resolve rule: nothing fills the field (`resolve/inheritance.rs` is unchanged; its
`[defaults_internal]` coverage test, `:369-408`, lists only the rules that exist).

### 2A.4 Tests that name every field

- `native-theme/src/model/mod.rs`, `theme_defaults_field_info_matches_baseline`
  (`:1973-2009`): add `"focus_ring_keyboard_only",` after `"focus_ring_offset_px",`.
  The list is the declaration order the `ThemeFields` derive registers, which is also
  what `lint_toml` accepts, so the key is valid TOML for `lint_toml` with no other edit.
- `native-theme/tests/proptest_roundtrip.rs`, `arb_theme_defaults` (`:121-189`), builds
  `ThemeDefaults` by a struct literal that names every field, outside the crate: it is
  the break D0 names, and stops compiling until the field is added. Add
  `proptest::option::of(proptest::bool::ANY)` to the tuple (after `focus_ring_offset`),
  bind it as `focus_ring_keyboard_only` and set the field; the comment "6 scalar Option
  fields" (`:127`), which lists four today, becomes "5 scalar Option fields".
- New test in `native-theme/src/resolve/tests.rs`: a variant with
  `focus_ring_keyboard_only = Some(true)` validates to `Some(true)`, one with `None` to
  `None`, and neither appears in `ResolutionIncomplete`'s list.
- **Every struct literal** of `ThemeDefaults` and `ResolvedDefaults`, found today with
  `grep -nE '(ThemeDefaults|ResolvedDefaults) \{'` over every tracked file outside
  `docs/archive/` (crates, tests, examples, docs) and each literal read to its closing
  brace with `python3`. Three name every field and must gain the new one:
  `native-theme/tests/proptest_roundtrip.rs:153-186` (above),
  `native-theme/src/model/resolved.rs:345-384` (`sample_defaults`, 2A.2) and the
  `validate_defaults!` literal, `native-theme/src/resolve/validate_helpers.rs:728-738`
  (2A.3). The other 27 end in `..Default::default()` and need nothing:
  `native-theme/src/macos.rs:145`, `:666`, `:693`, `:749`, `:802`, `:806`, `:828`,
  `:966`, `:996`; `native-theme/src/model/defaults.rs:230`, `:239`, `:251`, `:282`,
  `:286`, `:296`, `:307`, `:316`, `:333`, `:340`, `:354`, `:361`, `:377`, `:427`,
  `:446`; `native-theme/tests/merge_behavior.rs:176`, `:183`, `:291`. No literal of
  either struct is in a connector, a showcase, a README or a tracked document but
  the archive's.

## Part 2B — Facts, rules, registry, presets

### 2B.1 `docs/platform-facts.md`: the new row

The new row goes after `:1110`, the `focus_ring_offset` row, the one it belongs with
and the table's last; the blank line before `#### 2.1.6` stays (decision 2). One line, the table's six columns (`Property |
macOS | Windows | KDE | GNOME | Material`, header `:1106`):

```text
| `focus_ring_keyboard_only` | ❓ — no source states when AppKit shows the focus ring; the parked facts leave it pending for both generations (`docs/facts/macos/tokens.toml:47844`, U611, and `:46699`, U331, at `78ee0efd`) · macOS 26: ❓ — the same | **(Fluent)** true: the focus visual is drawn only on an element in the Keyboard focus state (`CFocusManager::GetFocusTarget`, [focusmgr.cpp:2731-2733](https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/dxaml/xcp/core/dll/focusmgr.cpp#L2731-L2733)); a focus the program sets counts as Keyboard where the last input was a key or a gamepad (:3199-3202, :3218-3237) | **(Breeze src)** true: Breeze puts its `QFocusFrame` on a widget only on a focus event whose reason is Tab, Backtab or a shortcut ([breezestyle.cpp:1649-1652](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L1649-L1652)); a focus from the pointer draws no band (`focus_ring_color`) | **(GTK)** keyboard, with a time-out of 3 s: `DEFAULT_VISIBLE_FOCUS_DURATION` ([gtkwindow.c:195](https://gitlab.gnome.org/GNOME/gtk/-/blob/4.24.1/gtk/gtkwindow.c#L195)), taken where `keyboard-focus-visible-timeout` is below 0 ([:6347-6349](https://gitlab.gnome.org/GNOME/gtk/-/blob/4.24.1/gtk/gtkwindow.c#L6347-L6349)), its default -1 (gsettings-desktop-schemas 51.0 `schemas/org.gnome.desktop.a11y.interface.gschema.xml.in:25-26`). Whether a text field shows its ring after a click was not read; adwaita leaves the field unset, as a yes or no cannot carry the time-out | **(impl)** true: Compose shows focus indication in its keyboard input mode, which follows Android's touch mode: a pointer's down or scroll enters touch mode, a key leaves it (`onTouchModeChanged`, [AndroidComposeView.android.kt:3807-3809](https://github.com/androidx/androidx/blob/e4d933c98df7baa7ea2b83eea6e086915961e0ff/compose/ui/ui/src/androidMain/kotlin/androidx/compose/ui/platform/AndroidComposeView.android.kt#L3807-L3809), at androidx `e4d933c9`, the parked facts' pin); measured on the Android host (run 42307873328, on `v0.8.0-rc1`) |
```

Sources, read today in `P8`: `F/kde/tokens.toml:1585-1593` (the Breeze quote, cited at
`breeze@6.8.2`; `docs/todo.md:1770-1771` records `breezestyle.cpp` byte-identical at
`f0b1d753` and `v6.8.2`, the commit every other Breeze link of the row uses);
`F/windows/tokens.toml:211-243` (the four WinUI citations and the classification, at
`microsoft-ui-xaml@2.5.1`, whose commit `ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd` is
`F/../sources.toml:861-871`); `F/gnome/tokens.toml:2008-2027`; `F/material/tokens.toml:10063-10085`;
`F/macos/tokens.toml:46697-46701`, `:47840-47846`. The WinUI link's URL is built from
that commit and the cited path. The androidx link is built from the parked facts' pin,
`androidx@e4d933c9`, whose full commit is `e4d933c98df7baa7ea2b83eea6e086915961e0ff`
(`F/../sources.toml:837`), on the GitHub mirror `androidx/androidx`, which `PF`'s other
androidx links use (`PF:1242`, `:1283-1285`, `:1289`) and which, the source's note says, holds the
same commits (`F/../sources.toml:843`). It is not `PF:869`'s Compose Material3 pin
(`1e677685`), so the cell names its commit. Neither link was **opened** (no network).

The KDE, Windows and Material cells say "true"; GNOME's says why adwaita is unset, its
schema path the parked fact's (`F/gnome/tokens.toml:2026` on `v0.8.0-rc1`);
macOS's is ❓. iOS has no column in `PF`; ios-13's comment cites its source itself
(2B.5), as that preset's comments do (`ios-13.toml:5-18`).

**The rows `:1108-1110` as they stand.** Corrected in v0.7.1 (`0d8fe8d5`) and read
again today: `focus_ring_color`'s KDE cell gives `[Colors:Selection] BackgroundNormal`
at alpha `Blend_Value` 0.3, #3daee94d, drawn after a focus change from the keyboard;
`focus_ring_width`'s gives 2 (`PM_FocusFrameHMargin`, measured 2); `focus_ring_offset`'s
gives −1. They are not edited: the preset and the reader now state what they say.

### 2B.2 `docs/property-registry.toml`

`[defaults]`, `# §2.1.5 Focus Ring` (`:126-129`): after `focus_ring_offset_px`,

```toml
focus_ring_keyboard_only     = "bool"
```

`property_registry_keys_match_derive_inventory` (`model/mod.rs:2055`) checks the key
against the struct; `scripts/check_platform_facts_rows.py` looks for a row named
`focus_ring_keyboard_only` anywhere in §2.1 (its docstring, `[defaults]` rule) and finds
2B.1's. The key goes where its group puts it.

### 2B.3 `docs/inheritance-rules.toml`

- `[no_inheritance]`, the group that opens with "soft_option sizes, enums, flags and the
  tooltip delay: stay None where the platform states none" (`:444-445`): insert as its
  first entry, after `:445`,

  ```toml
      "defaults.focus_ring_keyboard_only", # None: the connectors' ring on any focus; KDE, Windows, Material keyboard only, iOS any (§2.1.5)
  ```

  `no_inheritance_fields_are_not_inherited` (`resolve/inheritance.rs:816`) checks that
  nothing inherits into it.
- `[per_platform."input.caret_color"]`, `:314`, in place, one line:
  today `kde      = "defaults.focus_ring_color"`; new
  `kde      = "reader"                    # [Colors:View] DecorationFocus (platform-facts.md §2.4, caret_color), which the KDE reader writes (kde/colors.rs, the Input block); kde-breeze states none, so without the reader the fallback applies; not the ring, which is the selection colour at 0.3`.
  The reader's write is cited by its block, as 2I.5's appended caret text cites it, not
  by line: `variant.input.caret_color = view_focus;` is in the block under `// Input`
  (`native-theme/src/kde/colors.rs:557-565` at `0fa28027`), and step 1's helpers (its
  B.2) and 2C.1's move it. `"reader"` is a value the section has not
  used: the file's other KDE entries are `"preset"` or a field (`:314-347`), and neither
  is true here, since kde-breeze states no caret (correction 4, *Problems found* 1).
  `per_platform_rules_match_safety_nets` (`resolve/inheritance.rs:626`) compares key sets
  only, so the value is documentation. The value's comment names the facts file's
  section, not a line (S6).
- **Lines that move.** The one inserted line moves every later line by one. The existing
  line citations of the file (13 in `native-theme/src/presets/ios-13.toml` and one in
  `docs/todo.md` name a line after the insertion, read today) are left as they are
  and may drift; nothing re-points them (S6). No gate reads them.

### 2B.4 `docs/showcase-elements.toml`

Element `basic.text_inputs.focused` (`:1204-1209`): its `leaves` gain
`"defaults.focus_ring_keyboard_only"` after `"defaults.focus_ring_offset_px"` (registry
order, the file's rule at `:30-34`). No other element lists the ring's leaves today.

### 2B.5 The presets

Comment style as each file's own, but a new or rewritten comment cites the facts file
in the batch's form, today's with the line number removed, `# platform-facts.md
(§2.1.5): <key>: …` (S6; `docs/todo_v0.7.1_gpui-kit-0.9.1-spec.md`, Part I.3); LN's
checker (GPUI spec I.3) reads it.

**`kde-breeze.toml`, light (`:53-57`, `:67-70`) and dark (`:638-642`, `:652-655`).** The
two blocks are rewritten; nothing reads the preset's lines by number (2B.6). The four-line
comment and the value at `:53-57` (dark `:638-642`) become three lines, in light:

```toml
# platform-facts.md (§2.1.5): focus_ring_color: Breeze's keyboard focus band, [Colors:Selection] BackgroundNormal 61,174,233
# (BreezeLight.colors:87, breeze v6.7.5) at alpha Blend_Value 0.3 (breezemetrics.h:176)
focus_ring_color = "#3daee94d"
```

in dark the same with `(BreezeDark.colors:88, breeze v6.7.5)`. The two-line comment and
the two values at `:67-70` (dark `:652-655`) become six lines, identical in both
variants:

```toml
# platform-facts.md (§2.1.5): focus_ring_width and focus_ring_offset: the band is PM_FocusFrameHMargin, 2, wide, and runs from 1 px inside the
# widget's rectangle to 1 px outside it (breezestyle.cpp:737-739, :6642-6651 at breeze f0b1d75)
focus_ring_width_px = 2.0
focus_ring_offset_px = -1.0
# platform-facts.md (§2.1.5): focus_ring_keyboard_only: drawn only after a focus change from the keyboard (breezestyle.cpp:1649-1652 at breeze f0b1d75)
focus_ring_keyboard_only = true
```

(The `.colors` line numbers are the ones the preset's own `selection_background`
comments give, `kde-breeze.toml:47` and `:632`.)

**`windows-11.toml`**, after `focus_ring_offset_px = 1.0` (light `:66`, dark `:707`), two
lines:

```toml
# platform-facts.md (§2.1.5): focus_ring_keyboard_only: the focus visual is drawn only in the Keyboard focus state (focusmgr.cpp:2731-2733, microsoft-ui-xaml 2.5.1)
focus_ring_keyboard_only = true
```

**`material-3.toml`**, after `focus_ring_offset_px = 2.0` (light `:55`, dark `:574`), two
lines:

```toml
# platform-facts.md (§2.1.5): focus_ring_keyboard_only: Compose shows focus indication in keyboard input mode only (AndroidComposeView.android.kt:3807-3809, androidx e4d933c9)
focus_ring_keyboard_only = true
```

**`ios-13.toml`**, after `focus_ring_offset_px = 1.0` (light `:73`, dark `:419`), two
lines:

```toml
# (UIKit) the focus effect applies whenever the item is focused (UIView.h:166-167, Xcode 16.4 iPhoneSimulator SDK)
focus_ring_keyboard_only = false
```

(source: `F/ios/tokens.toml:21006-21015`.) No `platform-facts.md:` citation, so the
citation script does not read it, as for the file's other `(WebKit)` comments. The three
files' blank lines before `[light.defaults.border]` and `[dark.defaults.border]` stay.
The inserts move later preset lines that comments elsewhere cite (`windows-11.toml`'s
in the egui and iced connectors, read today); those citations are left as they are
and may drift (S6). No gate reads them.

The other thirteen presets state nothing: adwaita for the reason 2B.1's GNOME cell
gives; the macOS presets because their facts are pending; the ten colour schemes
because they have no platform. They resolve to `None`.

### 2B.6 Citations of lines that move (S6)

No citation is re-pointed (`docs/todo_v0.7.1_batch-speed-rationale.md`, S6). Both
steps' rows move later lines of `PF` (2B.1, 3B.1). The existing line citations of every
file a step moves lines in (`PF`, `docs/inheritance-rules.toml`, the presets) are left
as they are and may drift. What reads `docs/platform-facts.md` or a preset by line
number, read today (`git grep` of every `.rs`, `.py` and `.sh` file for code that opens
either file): only the three gates that compare a citation's line with the line it
names, `native-theme/tests/platform_facts_xref.rs` (`line(N)`, from `.lines().nth`),
`native-theme/src/presets/documented_sizes.rs` (`facts_line`, the same) and
`scripts/check_preset_citations.py`. They stop using line positions in v0.7.1 (S6, its
one change), before this release, so no test or gate here reads either file by line
number, and no edit here keeps their lines in place. Nothing reads a preset by line
(`scripts/check_todo_citations.py` flags an undated preset line citation in
`docs/todo.md`; it opens no line). The gpui showcase's citation tests read `.rs:` lines
only (`line_citations`, `CG/showcase.rs:3273`, finds `.rs:`), so they read neither file. A citation this specification has a
unit write names the document (with its section, row or a quoted phrase where useful),
never a line of a tracked file that changes; pinned sources (an upstream crate at a
version, a URL at a commit, a citation "at <sha>") keep their lines. The one exception
is what the gpui showcase's tests read by line (*Lines that must not move*): its colour
claims and `HOW` texts cite connector lines that `every_colour_claim_is_read_at_the_line_it_cites`
and `every_prose_citation_still_exists` check, so those keep their line numbers.

## Part 2C — The KDE reader and the portal's accent

### 2C.1 `native-theme/src/kde/colors.rs`

- **One helper.** After `translucent` (`:24-28`), a new function:

  ```rust
  /// Breeze's keyboard focus band in `highlight`, the selection colour: that colour at
  /// `Metrics::Blend_Value` (breezestyle.cpp:6776-6779, breezemetrics.h:176 at breeze
  /// f0b1d75; docs/platform-facts.md §2.1.5).
  pub(crate) fn focus_ring_of(highlight: Rgba) -> Rgba
  ```

  whose body is `translucent(highlight, BLEND_VALUE)`. `BLEND_VALUE`'s doc (`:12-17`)
  gains "and of the selection colour in the keyboard focus band (breezestyle.cpp:6776-6779)".
- **`populate_colors` (`:476-…`).** Bind `let selection_bg = get_color(ini,
  "Colors:Selection", "BackgroundNormal");` beside `view_focus` (`:478`), and use it at
  `:493` (`selection_background`) and where it is bound today (`:580`, which then goes),
  and at `:534` (the pressed button's mix), each reading the same key today. This changes
  no value. Replace `:490`, `variant.defaults.focus_ring_color = view_focus;`, by
  `variant.defaults.focus_ring_color = selection_bg.map(focus_ring_of);` with the
  comment "Breeze's keyboard focus band: the selection colour at `Blend_Value`; a pointer
  focus draws none (docs/platform-facts.md §2.1.5)".
- `focus_ring_keyboard_only` is **not** written by the reader: no colour scheme key
  changes it. The preset's `true` stands (the reader's output merges over kde-breeze,
  `colors.rs:456-457`).
- The doc of `populate_colors` (`:459-470`), as step 1 leaves it (its B.2 rewrites
  `:463-466`): the list "Some colours Plasma computes from the scheme" gains "the focus
  ring"; the blends on the accent do not (the KDE portal's accent moves the ring, 2C.2);
  the portal function the KDE path calls is named, `gnome::apply_kde_accent` (2C.2), for
  `gnome::apply_accent`; and a sentence on the ring is added. That paragraph
  (`:459-470` at `0fa28027`) becomes:

  ```text
  /// Some colours Plasma computes from the scheme, and the reader computes
  /// them as Breeze, Kirigami and qqc2-desktop-style do, in Qt's arithmetic:
  /// the checked button, the checkbox and progress-bar blends, the switch
  /// tracks and outlines, the toolbar's fill and edge, the popover's and the
  /// card's edges, the menu's hover, separator and panel, the list's hover,
  /// the table's grid, the scroll bar's resting handle and the focus ring. The
  /// blends on the accent (the checked checkbox, the progress fill, the switch's
  /// on track and its outline, and the list's hover) come from kdeglobals'
  /// `[Colors:Selection] BackgroundNormal`, so an application's `with_overlay` accent or the
  /// portal's (`gnome::apply_kde_accent`, applied by `GnomePortalKdeReader`) does
  /// not move them. The menu's hover comes from `[Colors:View] DecorationFocus`,
  /// so the same holds for it. The focus ring is the selection colour at
  /// `Blend_Value`; the KDE portal's accent replaces it (`gnome::apply_kde_accent`).
  ```

  The doc's other paragraphs are unchanged.
- `:564-565` (the caret, `view_focus`) is unchanged.

**Values** (computed today): Breeze and Breeze Dark state `[Colors:Selection]
BackgroundNormal` 61,174,233 (the fixtures `native-theme/tests/fixtures/kde/breeze-light.ini`
and `breeze-dark.ini`, read today); `Rgba::from_f32` rounds 0.3 × 255 = 76.5 up to 77 =
0x4d (`native-theme/src/color.rs:114-121`), as the menu hover already asserts
(`kde/colors.rs:1436`): `Rgba::new(0x3d, 0xae, 0xe9, 0x4d)`, #3daee94d, matching
`PF:1108`. The `custom-accent.ini` fixture's selection, 246,116,0, gives
`Rgba::new(246, 116, 0, 0x4d)`.

**Tests.**

- `native-theme/tests/reader_kde.rs:32`: `Some(Rgba::rgb(61, 174, 233))` →
  `Some(Rgba::new(61, 174, 233, 0x4d))`; `:143`: `Some(Rgba::rgb(246, 116, 0))` →
  `Some(Rgba::new(246, 116, 0, 0x4d))`.
- New in `kde/colors.rs`'s tests: `the_focus_ring_is_the_selection_at_blend_value`, for
  both Breeze fixtures and a scheme whose `[Colors:Selection] BackgroundNormal` differs
  from its `[Colors:View] DecorationFocus` (the custom scheme of
  `a_custom_scheme_moves_the_menu_scrollbar_and_list_colours`, `:1492`, whose two keys
  are 200,40,90 and 20,200,90, read today in that test's input), that the ring is the
  selection colour at 0x4d and the caret is still `DecorationFocus`, opaque.
- **The lists that hold the reader to the preset** (`native-theme/src/kde/mod.rs`'
  tests, read today at `:1246-1588`). `derived_leaves` and `resolved_leaves`
  (`:1248`, `:1309`; 29 leaves today, 31 after step 1, its B.5) do not hold the ring
  today. Both append `("defaults.focus_ring_color", mode.defaults.focus_ring_color)`
  and `("defaults.focus_ring_color", Some(theme.defaults.focus_ring_color))` (the
  resolved field is a required `Rgba`, `model/resolved.rs:143`) after what the earlier
  steps appended; the array length and the doc's count ("The twenty-nine colour leaves
  …", `:1246`) grow by one. `focus_ring_keyboard_only` is not a colour and the reader
  writes none; it joins no list. What each test then gains:
  - `the_derived_colours_are_the_presets_under_breeze` (`:1398-1414`): that the
    reader's ring under Breeze Light and Breeze Dark (selection 61,174,233 in both,
    `BREEZE_LIGHT_SCHEME` and `BREEZE_DARK_SCHEME`, `:1220-1244`) is kde-breeze's
    `focus_ring_color`, #3daee94d in both variants after 2B.5, and that the preset
    states it.
  - `an_accent_moves_the_on_track_the_pressed_button_the_selected_segment_and_the_list_hover`
    (`:1420-1438`, the `selection` key): its expected list gains
    `"defaults.focus_ring_color"` at the end; its doc gains "and the focus ring".
  - `the_view_decorations_move_the_frames_the_tab_hover_and_the_handle` (`:1538-1561`):
    unchanged, and now proves that the `focus` key (`DecorationFocus`) no longer moves
    the ring, which it moved before this step.
  - `the_button_set_moves_…` (`:1441`), `the_header_set_moves_the_toolbar` (`:1470`),
    `the_window_set_moves_the_popover_edge` (`:1486`, both assertions),
    `the_view_set_moves_the_card_edge_and_the_menu_panel` (step 1's name for
    `the_view_set_moves_the_card_edge`, `:1525`; its B.5), `the_frame_contrast_moves_the_edges`
    (`:1564`): unchanged (their keys are not `selection`).

### 2C.2 `native-theme/src/gnome/mod.rs`: the portal's accent on KDE

Today `GnomePortalKdeReader::read` applies the portal accent with `apply_accent`
(`:788-796`), which sets the ring to the accent, opaque (`:153`), and the comment at
`:782-787` says Breeze's ring is opaque in `DecorationFocus`.

- New function after `apply_accent` (`:150-156`):

  ```rust
  /// [`apply_accent`] as Breeze draws the accent the KDE portal gives, `QPalette::Highlight`
  /// (xdg-desktop-portal-kde `src/settings.cpp:282` at v6.7.5): its keyboard focus band is that
  /// colour at `Blend_Value` (docs/platform-facts.md §2.1.5).
  #[cfg(feature = "kde")]
  fn apply_kde_accent(variant: &mut crate::ThemeMode, accent: &crate::Rgba)
  ```

  which calls `apply_accent(variant, accent)` and then sets
  `variant.defaults.focus_ring_color = Some(crate::kde::colors::focus_ring_of(*accent))`.
- `:788-796`: the three `apply_accent(…)` calls become `apply_kde_accent(…)`.
- The comment `:782-787` becomes: "Apply the accent to the reader's variant. The KDE
  portal's accent is the palette's `QPalette::Highlight` (xdg-desktop-portal-kde
  `src/settings.cpp:282` at v6.7.5); Breeze's keyboard focus band is that colour at
  `Blend_Value` (docs/platform-facts.md §2.1.5), so the ring takes the accent at that
  alpha, not libadwaita's standalone derivation."
- The doc of `GnomePortalKdeReader` (`:732-738`), "applied to accent, selection, and
  focus_ring_color fields", gains "(the ring at Breeze's band alpha)".
- New test `the_kde_portal_accent_rings_at_blend_value`: `apply_kde_accent` with
  `Rgba::rgb(246, 116, 0)` gives `accent_color` and `selection_background` that colour
  and `focus_ring_color` `Rgba::new(246, 116, 0, 0x4d)`. The `kde` feature gates it.

Whether the accent the KDE portal gives can differ from the colour scheme's focus
colour is the rationale's §4 open item (its wording); it does not change this rule, which applies Breeze's alpha to
whichever colour `QPalette::Highlight` is.

## Part 2D — Every reader of the ring colour, before and after

Found by `grep -rn 'focus_ring_color'` over `native-theme/`, `connectors/` (all crates
and examples) today; the egui showcase reads none. The ring colour changes on
kde-breeze only; when the ring shows changes on kde-breeze, windows-11 and material-3.
The per-preset values are the presets' `[light|dark.defaults]` keys, read today with
`python3 -c 'import tomllib …'` over the 17 files:

| Preset | Ring today (colour, width, offset) | After | `keyboard_only` |
|---|---|---|---|
| kde-breeze, light and dark | #3daee9, 1, 2 | #3daee94d, 2, −1 | true |
| windows-11 light / dark | #000000e4 / #ffffff, 2, 1 | unchanged | true |
| material-3 light / dark | #625b71 / #ccc2dc, 3, 2 | unchanged | true |
| ios-13 light / dark | #007aff / #0a84ff, 2, 1 | unchanged | false |
| adwaita light / dark | #0461be80 / #81d0ff80, 2, −2 | unchanged | unset |
| macos-monterey, macos-tahoe | #0067f47f / #1aa9ff7f, 3, −1 | unchanged | unset |
| the ten colour schemes | their own, 2, 1 | unchanged | unset |

| Reader | What it shows today | After |
|---|---|---|
| core: `kde/colors.rs:490` | `DecorationFocus`, opaque | the selection at 0x4d (2C.1) |
| core: `gnome/mod.rs:153` via `:790-794` (KDE portal) | the accent, opaque | the accent at 0x4d (2C.2) |
| core: `resolve/inheritance.rs:94-96` (`← accent_color`) | fills an unset ring | unchanged; every bundled preset states the ring |
| gpui: `CG/colors.rs:223` → `ThemeColor::ring` | the colour as stated | opaque where the band is off (2E.1): kde-breeze #3daee9 (as today), windows-11 light #000000 (today #000000e4), windows-11 dark #ffffff, material-3 #625b71 / #ccc2dc (unchanged, opaque already); the other 14 presets unchanged |
| gpui: `Theme::focus_ring` (`CG/lib.rs:362`, reads the width) | on in all 34 combinations | off on kde-breeze, windows-11, material-3; on elsewhere |
| gpui: `FocusRing::of` (`CG/widgets/mod.rs:344`) | the stated ring on any focus | stated ring; on the three presets only for a focus that followed a key (2E.2) |
| iced: `focus_ring::ring_look` (`CI/focus_ring.rs:41`) | as gpui's own widgets | as gpui's own widgets (2F.1) |
| iced: `styles::text_input` (`CI/styles.rs:580`) | ring colour as the focused border where the ring is inset and `input.focus_border_color` is unset | unchanged: no bundled preset has both (adwaita states the border colour; the macOS presets, which do not, have no inset ring) |
| iced: `focus_ring_color()` (`CI/lib.rs:2017-2019`), contract `rows.rs:128-130`, `:624`, `contract.rs:3997` | the colour as stated | the colour as stated (kde-breeze's new value) |
| egui: plugin ring (`CE/plugin.rs:20-51`, painted `:624-669`) | stated ring on any focus | stated ring; on the three presets only after a key (2G.1) |
| egui: `focus_ring_color()` (`CE/accessors.rs:100-104`) | as stated | as stated |
| showcases: gpui `info/inputs.rs:927`, `leaves.rs:664`, `theme_map.rs:128`; iced `showcase-iced.rs:7761`, `:16414`, `:25044`, `:25879` | the stated colour, or the `ring` token | the same readings of the new values; texts per 2H |

## Part 2E — gpui connector

### 2E.1 gpui-component's band and `ring`

- **The predicate.** In `CG/colors.rs`, after `restore_capped` (ends `:788`) and before
  `#[cfg(test)]` (`:790`), so no claimed line moves:

  ```rust
  /// Whether gpui-component's focus band is drawn (`Theme::focus_ring`): where the theme
  /// states a ring with a width, and does not say the platform shows it for keyboard
  /// focus only. gpui-component draws its band on a click as well (into a text field, a
  /// select or a switch) and offers no way to turn that off, so where the platform shows its ring for
  /// keyboard focus only the band is off (docs/issues/0004_kde-open-items-rationale.md D4).
  pub(crate) fn focus_band_on(d: &ResolvedDefaults) -> bool
  ```

  body `d.focus_ring_width > 0.0 && d.focus_ring_keyboard_only != Some(true)`; and

  ```rust
  /// The colour of `ThemeColor::ring`: the theme's ring where the band is on; where it is
  /// off, `ring` is only the colour of a focused control's line, and is opaque.
  pub(crate) fn ring_colour(d: &ResolvedDefaults) -> native_theme::color::Rgba
  ```

  returning `d.focus_ring_color` where `focus_band_on(d)`, else, with `c` that colour,
  `native_theme::color::Rgba::new(c.r, c.g, c.b, 255)` (`native-theme/src/color.rs:70`).
  The file names the model's `Rgba` by its full path, as `rgba_to_hsla` does
  (`CG/colors.rs:17-20`), since it uses `gpui::Rgba` too (`:22`, `:45`). `ResolvedDefaults`
  joins the import of `:13` on the same line, `use native_theme::theme::{ResolvedDefaults,
  ResolvedTheme};` (exported at `native-theme/src/lib.rs:308-313`), one line for one, so
  the claimed lines `281-718` do not move.
- **`CG/colors.rs:223`**, one line for one line: `ring: rgba_to_hsla(d.focus_ring_color),`
  → `ring: rgba_to_hsla(ring_colour(d)),`. The opaque colour is converted, so the token
  is bit-for-bit what an opaque stated colour gives today (kde-breeze keeps exactly
  today's `ring`).
- **`CG/lib.rs:360-362`**: the comment becomes "§8.3 and D4: the band is drawn where
  the ring has a width and the platform does not show it for keyboard focus only; its
  width and offset have no receiver.", and `:362` becomes
  `theme.focus_ring = crate::colors::focus_band_on(d);`.
- **Test** `CG/lib.rs:1997-2008`, `focus_ring_follows_focus_ring_width`, becomes
  `focus_ring_is_the_band_rule` over every preset in both modes (`Theme::list_presets()`,
  `from_preset`): `theme.focus_ring == (width > 0 && keyboard_only != Some(true))`, and
  `focus_ring` is false for kde-breeze, windows-11 and material-3 and true for adwaita
  (so the test cannot pass vacuously). Its doc: "§8.3 and D4: the band's switch."
- New test in `CG/colors.rs`: `ring_is_opaque_where_the_band_is_off`: for every preset
  and mode, `to_theme_color(..).ring` is `rgba_to_hsla` of the stated colour where the
  band is on and of its opaque form where it is off; windows-11 light's ring alpha is 1
  (the one bundled combination where the two differ besides kde-breeze's).

### 2E.2 The connector's own widgets: "the focus followed a key"

GPUI's `Window::last_input_was_keyboard` (`GP/window.rs:3641-3645`) is true while the
last input was a key: `dispatch_event` sets the modality to keyboard on `KeyDown`, to
mouse on `MouseMove` and `MouseDown`, to touch on `Touch`, and leaves it on every other
event (`:5873-5881`); a window starts in mouse modality (`:2405`). So the answer is
taken when a control gets the focus and kept.

In `CG/widgets/mod.rs`, after `FOCUS_RING_SELECTOR` (`:400`), so `:344` does not move:

```rust
/// Whether a theme-drawn control's focus followed a key, decided when it gets the focus
/// and kept until it loses it (docs/issues/0004_kde-open-items-rationale.md D4).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct FocusVisible {
    /// Whether the control held the focus at its last render.
    focused: bool,
    /// `Window::last_input_was_keyboard` at the render that first saw it focused.
    keyboard: bool,
}
```

and a function `fn ring_shows(key: impl Into<ElementId>, focused: bool, keyboard_only:
bool, window: &mut Window, cx: &mut App) -> bool` (the callers pass a tuple, as
`use_keyed_state` takes `impl Into<ElementId>`). It takes the theme's answer as a
`bool`, not the resolved theme: a caller's resolved theme comes from `cx.native()`,
whose `Native` borrows `cx` (`CG/lib.rs:1056-1058`, `Native { pub resolved: &'a
ResolvedTheme, … }`, `:1064-1066`), and `ring_shows` needs `cx` mutably for
`use_keyed_state` (`GP/window.rs:4587-4592`), so passing both would not compile (E0502).
Each caller computes `let keyboard_only = n.resolved.defaults.focus_ring_keyboard_only ==
Some(true);` while `n` is in use, before its first mutable use of `cx` after that.
Its doc carries the paragraph on the limit of sampling at render (2E.3). It:

1. reads the control's `FocusVisible` with `window.use_keyed_state(key, cx, |_, _|
   FocusVisible::default())` (`GP/window.rs:4587-4592`), keyed by the control's id and
   the name `"focus-visible"`, a tuple gpui makes an `ElementId` of
   (`GP/window.rs:8036`), as the switch keys its thumb's spring
   (`CG/widgets/switch.rs:382-383`);
2. where `focused` and the state's `focused` is false, sets `keyboard =
   window.last_input_was_keyboard()`; where not `focused`, sets `keyboard = false`; sets
   the state's `focused` to `focused`. It writes with `Entity::update` and does not
   notify (nothing else reads the state);
3. returns `focused && (!keyboard_only || keyboard)`.

The state is read at render, so a blur and a refocus between two frames keep the
first focus's answer: the control never renders unfocused in between. That is the
limit of sampling at render, stated in `ring_shows`' doc (2E.3); how often it can
happen in an application (how soon gpui draws after a focus change) was not measured.

`FocusRing::of` and `FocusRing::draw` are unchanged (their unit tests, `CG/widgets/tests.rs:2676-2806`,
draw the ring directly and stay valid).

**The users**, each changed after its claimed lines (`checkbox.rs` 123–145,
`switch.rs` 94–144, `slider.rs` 77–80), each in the order the borrows need (read
today):

- `CG/widgets/checkbox.rs`, the shared `render` of `Checkbox` and `Radio` (`:251-300`):
  `n` is bound at `:262` and last used at `:273` (`let ring = FocusRing::of(n.resolved);`);
  the first mutable use of `cx` after it is `indicator_and_label(…, window, cx)` at
  `:274`. So `let keyboard_only = …;` goes after `:273`, and `let shows =
  ring_shows((self.id.clone(), "focus-visible"), focused, keyboard_only, window, cx);`
  after `:279` (`let focused = focus_handle.is_focused(window);`); at `:299`
  `.when_some(ring.filter(|_| focused), …)` becomes `.when_some(ring.filter(|_| shows),
  …)`.
- `CG/widgets/switch.rs`, `Switch::row` (`:366-…`; not `render`, which calls it,
  `:518-…`): `n` is bound at `:367` and last used at `:381` (`label_weight`); the
  first mutable use of `cx` after it is `spring(…, window, cx)` at `:382-388`. So `let
  keyboard_only = …;` goes after `:381`, and `:396`, `let focused =
  focus_handle.is_focused(window);`, becomes `let focused = ring_shows((self.id.clone(),
  "focus-visible"), focus_handle.is_focused(window), keyboard_only, window, cx);`, which
  `track` (`:397`, `:447-…`) then takes as the `focused` it rings
  (`:456`, `focused && !self.disabled && self.focus_ring`). `track` keeps its signature
  (`window: &Window, cx: &App`, read only; it reads `cx.native()` itself at `:457`).
  Its doc ("while `focused`, enabled, and told to") becomes "while `focused` (a focus
  the theme rings, `ring_shows`), enabled, and told to". `focused` has no other use in
  `row` (the implementer confirms).
- `CG/widgets/slider.rs`, `render` (`:325-…`): `n` is bound at `:326` and last used at
  `:333-336` (`look.faded_over(… n.resolved.defaults.background_color)`), with no
  mutable use of `cx` between `:332` and there. So `let keyboard_only = …;` goes after
  `:332` (`let ring = FocusRing::of(n.resolved);`), and `let shows = ring_shows((id.clone(),
  "focus-visible"), focused, keyboard_only, window, cx);` after `:364` (`let focused =
  !disabled && focus_handle.is_focused(window);`); the end thumb's
  `.when_some(ring.filter(|_| focused), …)` (`:375-377`) becomes `ring.filter(|_|
  shows)`. `focused` keeps its other uses in `render`, if any (the implementer checks
  each).

The other signatures this specification gives that take a value from `cx.native()`
and `cx` take `cx` shared, which a live `Native` allows: `Switch::track` (unchanged),
3F.1's `bordered_button(n: Native<'_>, cx: &App)` (a caller passes `cx.native()`'s `n`
and `cx` together, both shared borrows), and 3F.4's `hover_border(n)` (no `cx`).

### 2E.3 Contract and documentation

- **`CG/contract.rs:106-111`**, the `Row { slot: "ring", … }`, leaves `ROWS` and joins
  `COMPUTED_ROWS` (`:737-…`) as
  `ComputedRow { slot: "ring", source: "defaults.focus_ring_color, opaque where gpui-component's band is off (focus_ring_keyboard_only true, or no ring width; D4)", native: …, get: |tc| tc.ring, exceptions: &[] }`,
  whose `native` computes the rule from the resolved theme itself (stated colour, or
  `Rgba::new(r, g, b, 255)` where `focus_ring_keyboard_only == Some(true)` or the width
  is not above 0), not by calling `ring_colour`, and returns it as `rgba_to_hsla(…)`:
  `native` is a `fn(&ResolvedTheme, bool) -> Hsla` (`CG/contract.rs:47`), and the
  other rows convert with `rgba_to_hsla` (`CG/colors.rs:20`), as `button_hover`'s does
  (`CG/contract.rs:755-760`). `every_mapped_field_equals_its_native_source`
  (`:1603-1696`) then holds it over the 34 combinations.
- **`CG/contract.rs:1236-1242`**, `UnreadColour { leaf: "input.focus_border_color", … }`:
  "the token every focus ring shares, which the `ring` row gives
  `defaults.focus_ring_color`" → "…which the `ring` row gives `defaults.focus_ring_color`,
  opaque where the band is off".
- **`CG/contract.rs:2241-2256`**, `UPSTREAM_LIMITS`' evidence: after "its own ring,
  `FOCUS_RING_WIDTH` 3 outside that edge (…)" add "where `Theme::focus_ring` is on, which
  the connector turns off where the theme says keyboard only".
- **`CG/lib.rs:104`, `:110-111`** (crate docs): unchanged in substance; `:111`'s
  "focus ring" stays.
- **`scripts/connector_parity_exceptions.toml:493-497`**, the gpui `focus_ring`
  exception, whose reason says "gpui-component draws a focused Input's ring itself, its
  own 3px ring outside the edge, which no geometry refinement reaches (…)": after that
  parenthesis, before the closing quote, the clause "; it draws that ring only where
  `Theme::focus_ring` is on, which the connector turns off where the theme says the
  platform shows its ring for keyboard focus only (src/colors.rs, `focus_band_on`), and
  there a focused Input shows its border in `ring`" (this specification's evidence for
  the last part, `GC/input/input.rs:775-778`, not part of the inserted text). The
  parity check asks of a `reason` only that it be a non-empty string
  (`scripts/check_connector_parity.py:705-707`), so no gate reads the clause; it goes
  in unit 2.3 with `focus_band_on`. (The entry's citation of gpui-component 0.8.1 is
  older than this step and is not changed here.)
- **`ring_shows`' doc** (2E.2, after `FOCUS_RING_SELECTOR`, `CG/widgets/mod.rs:400`):
  "Where the theme says the platform shows its ring for keyboard focus only
  (`defaults.focus_ring_keyboard_only`), a control draws it only for a focus that
  followed a key press, decided when it gets the focus and kept until it loses it. The
  answer is taken at render, so a control that loses the focus and gets it back with no
  frame between keeps the first answer." `FocusRing`'s doc (`:300-327`) is left as it
  is: a paragraph there would move `:344` (`color: color(d.focus_ring_color),`), which
  the gpui showcase claims (`examples/showcase-gpui/info/inputs.rs:928`).
- **`connectors/native-theme-gpui/README.md`**:
  - `:696-713`, *The focus ring*. Present: "…`theme.focus_ring = false`, as above, turns
    the band off. … Breeze draws its band, 2 px of the selection colour at alpha 0.3,
    only for a focus that came from the keyboard, so on KDE an application whose users
    mostly point and click may prefer the ring off; a keyboard user then loses the band
    Breeze would draw. `apply` sets `focus_ring` from the theme again (on where
    `defaults.focus_ring_width` is above 0), so write it after each `apply`." New, from
    "Breeze draws its band": "Breeze, WinUI and Material draw their focus ring only for a
    focus that came from the keyboard; gpui-component draws its band on a click as well
    and offers no way to turn that off. So
    where the theme says so (`defaults.focus_ring_keyboard_only`, true in kde-breeze,
    windows-11 and material-3), `apply` turns the band off and makes `ring` opaque: a
    focused control then shows its border, or a 1 px line, in `ring` (a borderless
    button but a link, a text or an upstream `.ghost()` one: a 1 px line in its text
    colour at 60 %),
    also when Tab reached it, where the platform draws its ring; the connector's own controls draw the
    theme's ring after a key (below). `apply` sets `focus_ring` from the theme again (on
    where `defaults.focus_ring_width` is above 0 and `focus_ring_keyboard_only` is not
    true), so an application that wants another value writes it after each `apply`."
    The sentence before it, "For a text field that is Breeze's look on any focus; a
    focused button keeps a tinted border, which Breeze shows for keyboard focus only.",
    stays.
  - `:758-763`, *Keyboard focus*. After "…the control having no border of its own."
    insert: "Where the theme says the platform shows its ring for keyboard focus only, a
    control draws it only for a focus that followed a key press: it reads
    `Window::last_input_was_keyboard` when it gets the focus and keeps the answer until
    it loses it." The last sentence, "gpui-component's own widgets draw its 3 px ring at
    half the colour's alpha.", gains ", where `Theme::focus_ring` is on".

### 2E.4 Tests

- **A window under an edited theme.** `window_with` (`CG/widgets/tests.rs:1678-1695`)
  installs a preset as resolved and takes no edit, and `folder_window_counted`
  (`:777-808`) edits only light windows-11 (`windows_11(radius, edit)`, `:752`) and
  draws a tab bar. So `CG/widgets/tests.rs` gains, after `window_with`, a helper
  `window_with_resolved(cx, preset, mode, edit: impl FnOnce(&mut ResolvedTheme),
  build: Build)`, the body of `window_with` with `edit(&mut r)` applied to the
  resolved theme before `crate::to_theme` and `crate::apply`; 2E.4's edited-theme
  case and 3F.2's windows-11 and macos-tahoe combinations draw through it. `window_with` is not
  changed.
- `CG/widgets/tests.rs:2633-2673`, `every_focused_control_draws_the_ring`: on kde-breeze
  the controls are now ringed only after a key. The test gains, before each
  `focus_next`, a key press that moves no focus (`activate_key`, `:1712-1723`, with a
  key no control in the window binds; the implementer asserts the focus did not move),
  for kde-breeze; adwaita (unset) keeps today's sequence. Its doc (`:2629-2631`),
  whose "kde-breeze's outside the control" is false once kde-breeze's ring is 2 px wide
  at offset −1 (2D), becomes: "A focused Checkbox, Radio, Switch and Slider each draw
  the theme's ring (C-06): kde-breeze's after a key press, as the theme says keyboard
  only, across the control's edge, from 1 px inside it to 1 px outside; adwaita's inset
  one as a band inside the edge of the borderless row, track and thumb."
- New `a_control_focused_without_a_key_shows_no_ring_where_the_theme_says_keyboard_only`:
  kde-breeze; a Checkbox, a Switch and the Slider focused by `focus_next` with no key
  before draw no ring (`FOCUS_RING_SELECTOR` absent); the same under kde-breeze with
  `focus_ring_keyboard_only = None` (through `window_with_resolved`, above) draw it.
- New `the_ring_outlasts_a_mouse_move_and_ends_with_the_focus`: kde-breeze, a key, then
  `focus_next`: ringed; `simulate_mouse_move` (gpui-pre `app/test_context.rs:935`):
  still ringed; blur, draw a frame (`redraw`, `CG/widgets/tests.rs:1706-1708`), then focus
  again with no key and draw: none. The frame between the blur and the refocus is
  needed: without it the control is never seen unfocused (above).
- `a_switch_told_not_to_draws_no_focus_ring` (`:2606-2627`) calls `track` with
  `focused` true directly, so its code stays as it is; its doc's "kde-breeze's outside
  the track" (`:2603`) becomes "kde-breeze's across the track's edge, from 1 px inside
  it to 1 px outside", for the same reason.

## Part 2F — iced connector

### 2F.1 `CI/focus_ring.rs`: the wrapper

- `RingLook` (`:19-29`) gains `pub(crate) keyboard_only: bool`, "`defaults.focus_ring_keyboard_only`
  is `Some(true)`: the ring shows only for a focus that followed a key"; `ring_look`
  (`:36-50`) sets it.
- `Focus` (`:87-91`) gains two fields:
  - `key_last: bool` — "whether the last key-or-pointer event the wrapper saw was a key
    press" (starts `false`: a wrapper that has seen nothing has seen no key);
  - `by_key: bool` — "`key_last` at the moment the field gained the focus, kept until it
    loses it".
  `Default` (`:93-109`) sets both `false`; its doc gains a sentence: "A field focused
  before the wrapper saw any input, or by the view that a key built, has seen no key and
  shows no ring where the theme says keyboard only."
- `Focus::ringed` (`:111-116`) takes `keyboard_only: bool` and returns
  `self.field && self.window && (!keyboard_only || self.by_key)`; the free function
  `ringed(tree)` (`:211-218`) takes it too. `draw` (`:286`) and `update` (`:328`, `:339`)
  pass `self.look.is_some_and(|l| l.keyboard_only)`.
- `FocusRing::learn` (`:184-198`): where the probe finds the field focused and
  `focus.field` was false, `focus.by_key = focus.key_last`; where it finds it not
  focused, `focus.by_key = false`; then `focus.field = probe.focused` as today. `diff`
  (`:237-246`), where it sets `field = false`, sets `by_key = false` too.
- `update` (`:308-342`): **first**, before the content's `update`, classify the event:
  `Event::Keyboard(keyboard::Event::KeyPressed { key, .. })` sets `key_last = true`
  unless `key` is one of the named modifier keys, `Key::Named` of `Alt`, `AltGraph`,
  `CapsLock`, `Control`, `Fn`, `FnLock`, `NumLock`, `ScrollLock`, `Shift`, `Symbol`,
  `SymbolLock`, `Meta`, `Hyper` or `Super` (the modifier block of iced_core 0.14.0's
  `keyboard::key::Named`, `src/keyboard/key.rs:133-180`), which leave it (decision 4);
  `Event::Mouse(mouse::Event::CursorMoved { .. } | mouse::Event::ButtonPressed(_))` and
  `Event::Touch(_)` set it `false`; every other event leaves it (decision 4). This runs
  for every event, before the `may_move_focus` return (`:325-327`), which stays. A
  change of `key_last` alone draws nothing, so it requests no redraw; a change of
  `ringed` does, as today (`:339-341`).
- The module doc and `FocusRing`'s doc (`:155-166`) gain: "Where the theme says keyboard
  only, the ring shows for a focus that followed a key: the wrapper remembers whether
  the last key or pointer event it saw was a key, and when its field gains the focus
  keeps that answer until the field loses it. A column or a row hands every event to
  every child, so the wrapper sees them while it exists. Two places keep an event from
  it: a stack does not hand a lower layer what a higher one took (iced_widget 0.14.2
  `src/stack.rs:262-264`), and the base does not get what an overlay took; a field under
  an open dialog can so remember a key where a click came last, and show the ring once."
  (The two places are the rationale's, read there.)

Where the ring is inset at the edge, `styles::text_input` draws it as the focused
border (`CI/styles.rs:580-589`), and a style function sees only the status, not the
input: that route draws it on any focus, keyboard only or not. No bundled preset is
both inset and keyboard only (2D); `focus_ring`'s doc says so (2F.6).

### 2F.2 Tests in `CI/focus_ring.rs`

- `a_focused_field_is_ringed_by_the_themes_ring` (its doc at `:519-523`, `drawn` at
  `:491`) checks the geometry under macos-tahoe and windows-11 with a field focused
  before any input; under windows-11 that is now no ring. It sets
  `r.defaults.focus_ring_keyboard_only = None` on windows-11's resolved theme, says so
  in its doc ("the geometry, with the keyboard rule off"), and keeps its assertions.
- `the_ring_follows_the_focus_through_update` (`:710`, its doc `:704-708`) uses
  windows-11 and a left press: the preset becomes macos-tahoe (unset), the doc's
  "windows-11" too; its geometry assertions read the theme, so they hold.
- New `where_the_theme_says_keyboard_only_a_click_draws_no_ring` (windows-11): a left
  press focuses the field (the test module's field that takes the focus on a left press,
  its doc at `:577-579`): no ring; the field unfocused through `operate`
  (`focusable::unfocus`, `IC/widget/operation/focusable.rs:60`), a `KeyPressed` for Tab
  sent to the wrapper, then `focusable::focus_next` (`:165`) through `operate`: ringed;
  a `CursorMoved`: still ringed; the field unfocused again, then focused by a left
  press: none. `focus_next` is a chained operation, `operation::then(count(), |count|
  FocusNext { … })` (`:196`): its first pass only counts the focusable widgets, and its
  `finish()` returns `Outcome::Chain(next)` (`IC/widget/operation.rs:506-514`), the
  operation that moves the focus. So the test operates the element again with each
  `next` until `finish()` returns no `Chain`, as iced's runtime does
  (iced_winit 0.14.0 `src/lib.rs:1745-1758`; iced_test 0.14.0 `src/emulator.rs:195-205`),
  in a helper of the test module beside `operated`, `operate_chained(element, tree,
  node, operation)`; `unfocus` is a single pass.
- New `a_field_focused_before_any_input_shows_no_ring_where_keyboard_only` (kde-breeze):
  the `drawn` helper's pre-focused field: no quad.
- `the_focus_survives_a_theme_that_draws_no_ring` (`:843-872`) focuses its field by a
  left press under windows-11 and asserts one ring after the theme changes twice
  (`painted == 1`); it tests that the focus survives, not when the ring shows. Its
  windows-11 theme is bound `mut` and given `focus_ring_keyboard_only = None` before the
  loop; the doc gains "windows-11 with the keyboard rule off, so a ring shows the
  focus whatever input gave it".
- `a_replaced_field_loses_the_ring` (`:874-899`) operates on a `Field { focused: true }`
  under windows-11 before any input and asserts the ring before the replacement; it
  tests the replacement. The same edit to its `r`, and the doc gains the same clause.
- New, for decision 4: `a_modifier_key_alone_is_no_key` (windows-11): a field focused
  through `operate` after a `KeyPressed` of `Key::Named(Named::Shift)` and one after
  `Named::Control` draw no ring; one after `Named::Tab` draws it.

The other tests of the module (read today, with each one's preset and what it
asserts): `only_the_windows_focus_events_ask_the_field_again` and
`laying_the_ring_out_asks_its_field_nothing` count the field's operations and assert
no ring; `no_ring_is_drawn_where_the_border_is_the_ring_or_none_is_stated` asserts no
ring under adwaita and edited themes; none changes.

**The search for tests that focus without a key and expect a ring**, across the three
connectors, egui-widgets and the three showcases: every `#[test]` / `#[gpui::test]`
function was read for a ring assertion (`FOCUS_RING_SELECTOR`, `rings_on`, `painted`,
`focus_ring`, a band pixel), a focus without a key (`click_probe`, a pointer press,
`request_focus`, `focus_next` with no key before it, `Field { focused: true }`,
`window.focus`), and kde-breeze, windows-11 or material-3, directly or through a helper
(`ring_round_a_focused_button`, `ring_harness`, `ringed`, `slider_window`); found:
iced's four above and the showcase's (2H.2), gpui's `every_focused_control_draws_the_ring`
(2E.4), egui-widgets' `a_widget_focused_in_the_body_is_ringed_at_the_callers_radius`
(2G.4). Read and unaffected: egui's `the_ring_marks_keyboard_focus_only_while_the_window_has_it`
(Tab, a key), the egui ring tests on adwaita (`install_tests.rs:694-…`, `:847-…`,
`:1139-…`), egui-widgets' `the_switch_rings_its_track` (Tab), gpui's
`the_focus_ring_is_the_themes` and `a_switch_told_not_to_draws_no_focus_ring` (they
draw the ring directly).

### 2F.3 The contract

No row changes: the rows read the ring colour as stated (`rows.rs:128-130`, `:624`), and
the wrapper is not a style function the contract compares. `CI/contract/derived.rs:1058-1062`
(why a ring outside the edge is the wrapper's) is unchanged.

### 2F.4 `CI/lib.rs` and the README

- `focus_ring()`'s doc (`:1525-1551`): "(KDE's +2, Windows' and the colour schemes' +1)
  or across it (macOS's -1 with width 3)" → "(Windows' and the colour schemes' +1) or
  across it (macOS's -1 with width 3, KDE's -1 with width 2)". After "…as the platforms
  and the egui connector show none." add: "Where the theme says the platform shows its
  ring for keyboard focus only (`defaults.focus_ring_keyboard_only`), the wrapper rings
  the field only for a focus that followed a key press. iced moves the focus by Tab only
  where the application does: its text input has no Tab handling, and `focus_next` and
  `focus_previous` are operations an application sends (iced_runtime 0.14.0
  `src/widget/operation.rs:50-56`). A ring inset at the edge is the style function's
  border and shows on any focus."
- `connectors/native-theme-iced/README.md:582-598`, *Keyboard focus*: "A ring that lies
  outside the field's edge or across it (KDE, Windows, macOS)" stays; after "…and none
  while the window has lost the OS's keyboard focus;" insert "where the theme says the
  platform shows its ring for keyboard focus only (`defaults.focus_ring_keyboard_only`:
  KDE, Windows, Material), only for a focus that followed a key press;". After the
  paragraph's last sentence add: "iced moves the focus by Tab only if the application
  does: send `iced::widget::operation::focus_next()` on Tab and `focus_previous()` on
  Shift+Tab, as the showcase does."

### 2F.5 The showcase's Tab handler

`connectors/native-theme-iced/examples/showcase-iced.rs` has no Tab handling today
(`shortcut`, `:4470-4486`, reads Escape and the Ctrl bindings only). Tab reaches it:
`iced::keyboard::listen()` hands on the keyboard events no widget captured
(iced_futures 0.14.0 `src/keyboard.rs:7-18`), and a focused text input does not capture
Tab, a control character it does not insert (`IW/text_input.rs:1014-1036`).

- `Message` (`:2576`): two variants after `CloseOverlay` (`:2626`), `FocusNext` and
  `FocusPrevious`, each with a doc line ("Tab: the keyboard focus to the next focusable
  widget, iced's own operation" and the Shift+Tab one).
- `shortcut` (`:4470`): an arm before `_ => None`:
  `Key::Named(Named::Tab) => Some(if modifiers.shift() { Message::FocusPrevious } else { Message::FocusNext })`.
  The arm is not gated by `menus_in_window()`, as Escape's is not (`:4476`), so its doc
  comment (`:4463-4469`), which names the bound keys and says "Escape alone is bound
  here" where the menus are the system menu bar's, becomes:

  ```rust
  /// The message a key binding sends: Ctrl (Cmd on macOS) with Q, B, K or the
  /// comma, as the gpui showcase binds `secondary-q`, `-b`, `-k` and `-,`
  /// (`showcase-gpui/app.rs`), Escape, which closes a dialog, and Tab and
  /// Shift+Tab, which move the keyboard focus. Only keys no widget took reach
  /// here (`keyboard::listen`). Where the menus are the system menu bar's
  /// ([`menus_in_window`]), their keys are its items' and Escape, Tab and
  /// Shift+Tab alone are bound here, so one route owns each key, as in the egui
  /// showcase.
  ```
- `update_at` (`:2827-…`), beside `Message::Quit => return iced::exit(),` (`:2883`):
  `Message::FocusNext => return iced::widget::operation::focus_next(),` and the
  previous one (`IR/widget/operation.rs:50-56`; `iced::widget` re-exports
  `iced_runtime::widget::*`, iced 0.14.0 `src/lib.rs:634-637`).
- `update_inner` (`:2937-3110`), whose `match` has no `_` arm, so a new variant without
  an arm does not compile: `Message::FocusNext` and `Message::FocusPrevious` join its
  no-op arm (`:3091-3107`), after `Message::Quit`, and the arm's comment (`:3088-3090`),
  "…the screenshot's and the capture's, Quit, Copy and the command palette's runs",
  names them: "…, Quit, the two focus moves, Copy and the command palette's runs".
- A test `tab_moves_the_focus_with_iceds_operations`: `shortcut` maps Tab and Shift+Tab
  to the two messages, and nothing else.
- The test `the_system_menu_bar_takes_the_open_menu_flag_and_the_keys`
  (`:19855-19880`): its doc's "(Escape stays the window's)" (`:19857-19858`) becomes
  "(Escape, Tab and Shift+Tab stay the window's)"; its body, which asserts the four Ctrl
  keys unbound and Escape bound under the system menu bar, is unchanged.

A focused `text_editor` (the Basic page's text area) does not capture Tab either: its
key binding inserts a character only where the key's text has one that is no control
character (`IW/text_editor.rs:1220-1223`), and an event no binding takes is not
captured (`:733-756`).

### 2F.6 Comments that become false

- `CI/styles.rs:575`: "outside the edge (offset 0 or more: KDE's +2, Windows' +1),
  straddles it (macOS's -1 with width 3)" → "outside the edge (offset 0 or more:
  Windows' +1), straddles it (macOS's -1 with width 3, KDE's -1 with width 2)".

## Part 2G — egui connector

### 2G.1 `CE/plugin.rs` and `CE/atlas.rs`

- `atlas.rs:99-102`, `FocusRing`, gains `pub(crate) keyboard_only: bool`;
  `build_focus_ring` (`plugin.rs:20-51`) sets it from
  `theme.defaults.focus_ring_keyboard_only == Some(true)`.
- **What is stored** (decision 6): one map, `focus_origin:
  egui::ViewportIdMap<FocusOrigin>`, in `NativeThemePlugin` (`plugin.rs:312-325`), with
  the doc "per viewport whose input this plugin saw, what its focus followed", of a
  small struct of `plugin.rs`:

  ```rust
  /// What one viewport's focus followed (D4): its inputs' kinds and its focused widget.
  #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
  struct FocusOrigin {
      /// The last key or pointer event of its input was a key press.
      key_last: bool,
      /// Its previous pass's input held a key press.
      key_before: bool,
      /// This pass's input held a key press.
      key_now: bool,
      /// This pass's input held a pointer button press or a touch's start.
      pointer_now: bool,
      /// The widget that held the focus at the end of its last pass.
      focused: Option<egui::Id>,
      /// Whether that widget's focus followed a key.
      by_key: bool,
  }
  ```

  with two methods, so that the rule is tested without a `Context`:
  - `fn read(&mut self, events: &[egui::Event])`: first `key_before = key_now`,
    `key_now = false`, `pointer_now = false`; then, in order, `Event::Key { key,
    pressed: true, .. }` sets `key_last` and `key_now` unless `key` is one of egui's
    eight modifier keys, `Key::ShiftLeft`, `ShiftRight`, `ControlLeft`, `ControlRight`,
    `AltLeft`, `AltRight`, `SuperLeft` or `SuperRight` (egui 0.36.2
    `src/data/key.rs:192-221`, 0.37.0 `:253-274`), which egui-winit sends as
    `Event::Key` presses (egui-winit 0.36.2 `src/lib.rs:1640-1651`, pushed at
    `:1019-1045`; 0.37.0 `:1797-…`, pushed from `:1128`), so a modifier pressed alone
    is no key, as in iced (2F.1) and gpui (decision 4); `PointerButton { pressed: true, .. }`
    and `Touch { phase: egui::TouchPhase::Start, .. }` clear `key_last` and set
    `pointer_now`; `PointerMoved(_)` and every other `Touch { .. }` clear `key_last`;
    every other event changes nothing (decision 4).
  - `fn settle(&mut self, focused: Option<egui::Id>) -> bool`: where `focused` differs
    from `self.focused`, stores it with `by_key = !pointer_now && (key_last || key_now
    || key_before)` (decision 14); returns `by_key`.

  `forget_closed_viewports` (`:347-385`) removes a closed viewport's entry beside its
  `last_sent` (`:378`).
- `input_hook` (`:417-457`): `self.focus_origin.entry(input.viewport_id).or_default().read(&input.events)`.
  The hook runs before egui's `begin_pass` reads the events (`EG/context.rs:966`, cited
  at `plugin.rs:407`), and on every pass, a frame's later passes included
  (`EG/context.rs:855`), with that pass's events.
- `on_end_pass` (`:576-578`): `let by_key = self.focus_origin.get_mut(&ui.ctx().viewport_id()).is_some_and(|o| o.settle(ui.ctx().memory(|m| m.focused())));`
  (`false` for a viewport with no entry), then `paint_focus_ring(ui, by_key)`.
- **The rule against the sequences it must cover** (each one a test of `FocusOrigin`,
  2G.3, and the first three also through a `Context`):
  - Tab, then a `PointerMoved`, in one pass: Tab moves the focus in that pass
    (decision 14); `key_last` is false after the move, but `key_now` holds and
    `pointer_now` does not: a ring.
  - A press on a text field, then a key, in one pass: the field takes the focus from
    the press; `key_last` and `key_now` hold, but `pointer_now` does too: no ring.
  - Shift+Tab from a focused widget in pass N, a `PointerMoved` in pass N+1: the focus
    moves in N+1's `Focus::begin_pass` (decision 14) and `settle` sees it at N+1's end,
    where `key_before` holds and `pointer_now` does not: a ring. With a click in N+1
    instead: no ring (a cost decision 14 names). A Tab past the last focusable widget
    in pass N is the same case (decision 14).
  - Several passes in one frame: pass 2 gets no events, so `read` makes `key_before`
    pass 1's `key_now` and clears `key_now` and `pointer_now` (`key_last` stays as
    pass 1 left it). A focus move pass 1's arrow key made in its `Focus::end_pass` is
    seen at pass 2's end with `key_before`: a ring. A click moves the focus inside
    pass 1 itself, so `settle` sees that change at pass 1's end, with `pointer_now`:
    no ring; at pass 2's end the focused widget is the same, and the stored answer
    stands.
  - Two viewports: each has its own entry, keyed by the input's `viewport_id` in the
    hook and by `Context::viewport_id` in `on_end_pass`; an immediate viewport's pass,
    nested inside its parent's, reads and settles its own entry, so a key in one never
    counts for a focus change in the other, and the parent's `key_before` is its own
    previous pass's, not the child's.
  - A drag value clicked: the press (pass N) clears `key_last`, the release (N+1)
    gives the focus with no key in N or N+1: no ring. A program's `request_focus` with
    no key since the last pointer event: no ring.
- `paint_focus_ring` (`:624-669`) takes `by_key: bool`; after the `atlas.focus_ring`
  read (`:631-633`) it returns where `ring.keyboard_only && !by_key`, with the comment
  "the theme says keyboard only, and this focus did not follow a key (D4)".
- Docs: `plugin.rs:15-19`, "a stated zero width is no ring and no note, as gpui's
  `focus_ring = width > 0.0` (`connectors/native-theme-gpui/src/lib.rs:362`)", becomes
  "a stated zero width is no ring and no note, as a zero width is no band in gpui's
  `focus_band_on` (`connectors/native-theme-gpui/src/colors.rs`)" (that function also
  turns the band off for a keyboard-only theme, which here is `paint_focus_ring`'s
  rule, so only the zero-width part is the same); `paint_focus_ring`'s
  doc (`:619-623`) gains "Where the theme says keyboard only, only for a focus that
  followed a key press: the plugin remembers the kind of the last input across passes
  (a drag value takes the focus in the pass of the release, not of the press) and, when
  the focused widget changes, keeps the answer for that widget."
- `connectors/native-theme-egui/README.md:61-62`: "paints the platform's focus ring
  around the widget with keyboard focus" → "paints the platform's focus ring around the
  widget with keyboard focus (where the theme says the platform shows it for keyboard
  focus only, `defaults.focus_ring_keyboard_only`, only after a key press)".

### 2G.2 `mapping.toml` and `mapping.md`

`connectors/native-theme-egui/mapping.toml`, after `["defaults.focus_ring_offset"]`
(`:261-263`):

```toml
["defaults.focus_ring_keyboard_only"]
verdict   = "derived"
tested_by = "T14(b)"
```

Regenerate `src/mapping.md` (`cargo test -p native-theme-egui --lib -- --ignored regenerate_mapping_md`,
the file's own instruction, `mapping.md:3`). Computed from today's totals
(`mapping.md:11-19`): Foundation 66 → 67 leaves, DERIVED 36 → 37; TOTAL 516 → 517,
DERIVED 171 → 172. `mapping_tests.rs:236-241`: `516` → `517` and the message
"512 ResolvedTheme leaves" → "513". `verdict_totals_match_the_specification`
(`mapping_tests.rs:443-497`) holds each group's totals by hand:
`("foundation", [18, 1, 36, 11])` → `("foundation", [18, 1, 37, 11])`; its sub-tag
counts (`:489-496`) do not change (the row has none). Searched for every other
hard-coded count or list the row changes: `grep` over `connectors/native-theme-egui/src/`
and its showcase for the totals (516, 171, 66), for `assert_eq!` with a literal count
and for lists of leaf names that include `focus_ring_`: the leaf count (`:239`) and
the group totals are the two; `is_clause` (`:188-222`) lists clause labels, and `T14(b)`
is one already; `BUILD_READ_TESTED_BY` (`:743-746`) lists the `f32` leaves the build
reads for T4's hostile floats, and the new leaf, a `bool`, is not one (`with_hostile`,
`:732-737`, writes floats). The showcase's lists name other leaves. T2 needs a preset pair that differs in the leaf: unit 2.1 lands the
presets with the row (2K). The leaf walk (`mapping_tests.rs:156-186`) finds the
new leaf because `ResolvedDefaults` serialises `None` as `null`, a leaf of the walk.
`mapping_md_is_current` (`mapping_doc.rs:347`) proves the regeneration.

### 2G.3 Tests in `CE/install_tests.rs`

The clause T14 (b) of the archived spec's §13 (`docs/archive/todo_v0.6.0_egui-connector-spec.md:7945`);
its two tests are `the_ring_marks_keyboard_focus_only_while_the_window_has_it` (`:729-821`,
kde-breeze, a button: a click gives a button no focus in egui, Tab does) and
`the_ring_takes_the_innermost_scope_s_radius` (`:823-…`, adwaita). Both hold after the
step (the first reaches the ring by Tab, a key; the second's preset is unset). New
tests, each with a doc comment that begins "T14 (b)", so the egui showcase's route
names them (`info::checked_by`, `docs/archive/todo_v0.7.0_code-review-findings.md:12855`):

- `a_click_into_a_text_edit_draws_no_ring_where_keyboard_only`: kde-breeze; a
  text edit clicked (press, then release, as `:711-713`): focused, no ring; the same
  sequence under adwaita draws one (the existing test
  `a_clicked_borderless_text_field_shows_the_focus_ring`, `:694-727`, already proves it,
  and is named).
- `the_ring_after_tab_outlasts_a_pointer_move`: kde-breeze; Tab to a button: one ring; a
  pass whose input is a `PointerMoved`: still one ring.
- `a_drag_value_focused_by_a_click_draws_no_ring_where_keyboard_only`: kde-breeze; a
  `DragValue` clicked (it takes the focus at the release, `EG/widgets/drag_value.rs:665-667`):
  no ring.
- `a_widget_focused_by_the_program_draws_no_ring_where_keyboard_only`: kde-breeze;
  `request_focus` with no key: no ring.
- `shift_tab_rings_a_pass_late_through_a_pointer_move`: kde-breeze, two buttons, the
  second focused by Tab; a pass whose input is Shift+Tab; then a pass whose input is a
  `PointerMoved`: the first button holds the focus and one ring is painted round it.
- `a_click_after_typing_draws_no_ring`: kde-breeze, a text edit focused by Tab and
  typed into (a key press), then in the next pass a click into a second text edit: it
  holds the focus and no ring is painted.
- `tab_then_a_pointer_move_in_one_pass_rings`: kde-breeze, two buttons; one pass whose
  input is Tab followed by a `PointerMoved`: the first button holds the focus and one
  ring is painted.
- `a_press_then_a_key_in_one_pass_draws_no_ring`: kde-breeze, a text edit; one pass
  whose input is a press on it followed by a key press: it holds the focus and no ring
  is painted.

And in `CE/plugin.rs`' tests, of `FocusOrigin` alone (no `Context`), five tests for
five of the six sequences of 2G.1's list, each named for its sequence (the sixth, two
viewports, is a property of the plugin's map, not of `FocusOrigin`, so its test is the
`Context` one below): `focus_origin_tab_then_a_pointer_move`,
`focus_origin_a_press_then_a_key`, `focus_origin_shift_tab_applied_a_pass_late`
(with a `PointerMoved` and, apart, with a click in the later pass),
`focus_origin_a_later_pass_of_the_frame` (a pass with an arrow key and no focus
change, then an empty `read` and a `settle` with a new widget: `true`; and, apart, a
pass with a click whose `settle` sees the new widget: `false`, then an empty `read`
and a `settle` with the same widget: still `false`), `focus_origin_a_drag_value_and_the_program`;
one more, for decision 4, as iced's `a_modifier_key_alone_is_no_key` (2F.2):
`focus_origin_a_modifier_alone_is_no_key`, where, from a fresh `FocusOrigin`, a `read`
of one `Event::Key { key: Key::ShiftLeft, pressed: true, .. }` and a `settle` with a
new widget give `false`, the same with `Key::ControlRight` give `false`, and with
`Key::Tab` give `true`;
and, in `CE/install_tests.rs`, `the_focus_origin_is_kept_per_viewport`. It runs
kde-breeze with a root pass that shows an immediate viewport, set up the way
`a_closed_viewport_s_focus_entry_is_let_go` (`install_tests.rs:3371-…`) and
`a_closed_viewport_s_last_sent_theme_is_let_go` (`:3533-…`) set up theirs:
`set_embed_viewports(false)` and an immediate-viewport renderer
(`set_immediate_viewport_renderer`) that runs the child's pass. The child's input holds
a key press and the root's a `PointerMoved`. It asserts that the two viewports'
entries differ, that a focus the program gives in the root afterwards is not ringed,
and that once the child is closed its entry is gone, as its `last_sent` is (read
through a test accessor in the form of `last_sent_viewports`, `plugin.rs:388-393`).

### 2G.4 `connectors/native-theme-egui-widgets/src/tests.rs`

`a_widget_focused_in_the_body_is_ringed_at_the_callers_radius` (`:858-…`) focuses a
button by `request_focus` under kde-breeze, adwaita, windows-11 and catppuccin-mocha and
expects a ring (`ring_round_a_focused_button`, `:824-850`). It tests the radius, not
when the ring shows. The theme is installed in the test itself
(`installed(&t, &prefs)`, `:865`), not in the helper: `t` (`:863`) is bound `mut` and
given `focus_ring_keyboard_only = None` before the closure that installs it, and the
test's doc says so. `the_switch_rings_its_track`
(`:1229-…`) reaches the switch by Tab and holds.

### 2G.5 What is not changed

A text edit shows its focus by its own frame, which takes `input.focus_border_color`
(`CE/accessors.rs:400-428`): unchanged, on any focus, as Breeze recolours a focused line
edit's frame on any focus (`PF:1108`'s KDE cell).

## Part 2H — The showcases

### 2H.1 The start-up focus

Each showcase gives the Basic page's focused field the focus at start-up: iced by an
operation in `boot` (`showcase-iced.rs:17601-17612`), egui by `request_focus` in the
first frame (`showcase-egui/pages/basic.rs:862-871`), gpui by `state.focus` in the app's
setup (`showcase-gpui/app.rs:1576-1580`). None follows a key, so under kde-breeze,
windows-11 and material-3 the field shows its focused frame without the ring, in all
three; elsewhere unchanged. The comments that say why the field is focused ("so its
focus border shows", `showcase-iced.rs:17601-17604`, `basic.rs:862`, `app.rs:1577-1579`)
stay true. No code change.

### 2H.2 iced

- `iced_route` (`:7749-8552`), the `match` of the leaves the export does not cover, beside
  the ring's lengths (`:8507-8530`): four arms for `("defaults",
  "focus_ring_keyboard_only")`, in this order. `iced_route` prefixes "iced: " to a line
  that does not begin "not reachable" or "not drawn" (`:8546-8551`).
  1. where `ring_as_the_border(resolved)` (the ring inset at the edge, adwaita's):
     "styles::text_input: the ring is the Focused border here, which a style function
     draws on any focus, whatever the theme says of keyboard focus";
  2. where `ring_round_the_field(resolved)` and the field is `Some(true)`:
     "native_theme_iced::focus_ring: the ring only for a focus that followed a key
     press, decided when the field gains the focus";
  3. where `ring_round_the_field(resolved)` otherwise: "native_theme_iced::focus_ring:
     the ring on any focus (the theme does not say keyboard only)";
  4. else: "not drawn: the theme states no focus ring (focus_ring_width 0)", the
     wording the lengths take in that case (`:8510-8514`), true only there.
- `the_focus_ring_routes_name_the_border_that_draws_it` (`:28797-…`) gains the new leaf in
  its leaf loop (`:28810-28813`), and its assertion holds for each case: adwaita's route
  begins "iced: styles::text_input: " (arm 1) and contains no "no focus ring";
  macos-tahoe's begins "iced: native_theme_iced::focus_ring: " (arm 3); windows-11's and
  kde-breeze's begin the same (arm 2). Its doc names the new leaf.
- `the_focused_field_draws_the_themes_ring_round_it` (`:25838-25898`) clicks the field
  under macos-tahoe and windows-11 (`frame_after` over an `iced_test::Simulator`,
  `click_probe`, `:26267`, `:17808`). The Simulator can point, click, type, tap a key,
  feed raw events, find a selector and snapshot (iced_test 0.14.0 `src/simulator.rs:45-307`);
  it offers no call that runs a `Task` or an operation that gives a widget the focus,
  so the `focus_next` task of 2F.5 cannot run there, and a Tab key event alone moves
  no focus (the showcase's subscription, which turns it into `Message::FocusNext`, does
  not run in the Simulator). So the test keeps the click and splits windows-11 in two:
  (a) after `install_in_mode`, `state.current_resolved.defaults.focus_ring_keyboard_only
  = None` (the Basic page reads `state.current_resolved`, `:10374`), then the click: the
  ring is drawn as today, which tests the ring's geometry at +1; (b) the preset as it
  is, the same click: the pixel in the band is the ground's, to the same 3 per channel,
  since a click is no key. macos-tahoe stays as it is. The doc says so and names the
  connector's tests that prove the key path through `operate` (2F.2).

### 2H.3 gpui

- `info/leaves.rs`: a `HOW` entry for `("", "defaults.focus_ring_keyboard_only", …)`
  after the `focus_ring_offset_px` one (`:668-671`): "gpui: whether gpui-component's band
  is drawn (Theme::focus_ring off where true; native-theme-gpui/colors.rs, focus_band_on), and whether the
  theme-drawn Checkbox, Radio, Slider and Switch ring a focus that did not follow a key
  (widgets/mod.rs, ring_shows)". The `focus_ring_color` entry (`:663-666`): "and
  ThemeColor ring (colors.rs)" → "and ThemeColor ring (colors.rs, ring_colour: opaque
  where the band is off)". The `focus_ring_width_px` entry (`:673-676`), whose text ends
  "…, and a width > 0 turns it on", false after this step on kde-breeze, windows-11 and
  material-3: "and a width > 0 turns it on" → "and a width > 0 turns it on where the
  theme does not say keyboard only (native-theme-gpui/colors.rs, focus_band_on)". A
  `file.rs, Symbol` citation with no line passes `every_prose_citation_still_exists`
  where exactly one file of that name holds the symbol (`symbol_citation_finding`,
  `CG/showcase.rs:3392-3417`); the crate name picks the connector's own `src/`
  (`candidates`, `:2576-2585`; the root `cited_source_dirs` adds, `:2681-2684`).
- The Theme Map's Ring row (`info/theme_map.rs:126-129`), whose one note is
  `.config("model", "defaults.focus_ring_color")`: the module's rule (`:1-8`) asks the
  notes to name the model fields the cited line's value is read from and how it is
  made from them, and after 2E.1 the value is `ring_colour`'s. In the form of the Muted
  row (`:107-112`): `model` "defaults.focus_ring_color, defaults.focus_ring_width_px,
  defaults.focus_ring_keyboard_only" and a `derived` note, "the colour as stated where
  gpui-component's band is on; made opaque where it is off, where the theme says
  keyboard only or its ring width is not above 0: then ring is a focused control's
  line, not a band (native-theme-gpui/colors.rs, ring_colour)". The note is read by
  `every_prose_citation_still_exists` as a config line (its doc, `CG/showcase.rs:3669-3672`;
  `config` is one of `NOTE_METHODS`, `:2417`).
- The four unconditional band claims become conditional on `t.focus_ring`, as the
  Carousel's are (`info/layout.rs:1093-1114`), with the focused border claimed where it
  is off: `info/inputs.rs:268-273` (InputGroup, "focus ring, ring at 50%"), `:716-726`
  (Switch), `:1742-1747` (ColorPicker), `:1937-1942` (TimeField). Where `t.focus_ring` is
  false the entry is omitted and the info gains `.config("focus ring", "off: the theme
  says the platform shows its ring for keyboard focus only, and gpui-component draws its
  band on a click as well and offers no way to turn that off (native-theme-gpui/colors.rs,
  focus_band_on)")`. Which info gains
  which claim: the Switch and the TimeField claim no focused border today, so in the
  `else` branch each gains `claim("focused border", "ring", t.ring,
  "gpui-component/styled.rs:270")`, as the Carousel's `else` branch has it
  (`info/layout.rs:1107-1112`): both are bordered (`border_1`, `GC/switch.rs:270`,
  `GC/time/time_field.rs:109`) and focus through `focus_ring_style`
  (`GC/switch.rs:281-282`, `GC/time/time_field.rs:116`), so with the band off their border
  is tinted (`GC/styled.rs:269-270`). The InputGroup and the ColorPicker claim theirs
  already, in either branch (`"focused border"`, `input/group.rs:322`, `inputs.rs:262-267`;
  `color_picker.rs:778`, `inputs.rs:1736-1741`), and keep it. A disabled Switch gains
  neither the "focus ring" config line nor the focused-border claim: its info claims no
  ring today, since a disabled switch takes no focus (the `if disabled` of
  `info/inputs.rs:715-726`, with its comment), and the new branch on `t.focus_ring`
  goes inside that `else` arm, where the ring claim is now.
- `info/inputs.rs:169`, the Input's not-themeable focus ring: "the connector uses the
  platform's focus_ring_width only as a switch: upstream drops the ring where
  Theme::focus_ring is off …" → "the connector uses the platform's focus_ring_width and
  focus_ring_keyboard_only only as a switch: upstream drops the ring where
  Theme::focus_ring is off, which the connector turns off where the width is 0 or the
  theme says keyboard only, …" (rest unchanged).
- `theme_focus_ring` (`info/inputs.rs:898-935`): where the theme states `Some(true)`, the
  config line "{w}px wide, …" gains ", for a focus that followed a key press only
  (native-theme-gpui/widgets/mod.rs, ring_shows)".
- `ring_findings` (`tests.rs:16087-16120`) is unchanged: it checks the claimed colour and
  line, which do not change.
- `docs/todo.md:3315-3320` (`[1.0]` "Two focus claims hold only while
  `Theme::focus_ring` is on") closes: the claims are branched here (2I.5).
- The showcase's Tab traversal: the showcase registers a plugin of gpui-base's `Root`
  (`examples/showcase-gpui/host.rs:75`), whose key bindings turn Tab and Shift+Tab into
  `Window::focus_next` and `focus_prev` (gpui-base 0.9.1 `src/root.rs:44-45`,
  `:222-227`), so the showcase moves the focus with Tab; nothing to add.

### 2H.4 egui

No showcase text names the focus ring (`grep -rni 'focus.ring\|focus ring'` over
`showcase-egui/` today finds none; a bare `ring` finds only the drawn rings of
`demo.rs:1688-1705` and a test message, `tests.rs:6126`, no focus ring). Its Theme Map and Widget Info route for the new leaf come from `mapping.toml`'s
row (2G.2): "checked by native-theme-egui's test …", naming 2G.3's tests.

### 2H.5 Screenshots

The release's assets change where the start-up field shows the ring today: the Linux
captures and theme-switching GIF frames of kde-breeze (light and dark) and material-3
(light and dark) in the three showcases (`SCREENSHOT_ENTRIES`, `scripts/README.md`), and
the Windows captures of the CI run (windows-11). macOS and catppuccin-mocha captures do
not change. They are regenerated by the asset run (2K). Why they change: every capture
opens the Basic page (`scripts/capture_window.sh:457`, `--tab basic --capture`), whose
start-up focus (2H.1) follows no key, and a capture lets no key and no pointer event
reach the page (egui drops them while capturing, `showcase-egui/app.rs:975-988`; iced's
`held_page`, `showcase-iced.rs:3238-3247`; gpui's pointer shield,
`a_capture_hovers_nothing`, `showcase-gpui/tests.rs:6253`; no script or workflow passes
`--pointer`, `grep` over `scripts/` and `.github/` today). So on the three keyboard-only
presets no capture shows a ring after the step, where today the start-up field shows
one. Whether each listed capture shows the ring today was read from the code, not from
the images: **not verified** against the files.

## Part 2I — Outside the code

### 2I.1 `CHANGELOG.md`

The release's one section (S5), `## [0.7.3] - <date>`, written with its number from the
start (never `## [Unreleased]`) and its date filled by the version step, above every
other `## [` heading (`## [0.7.1]`, or `[0.7.2]` if that is released first; at the
merge, 2K, a conflict there is resolved by putting this section first). Unit 2.7 writes it
with step 2's entries below; unit 3.3 adds step 3's (3H.1) under the same headings and
turns the *Breaking Changes* entry into the one that names both structs (3H.1). The
*Breaking Changes* entry follows 0.7.0's wording (its section of `CHANGELOG.md`, the
entry on struct literals); that note is for the reader and is not written into
`CHANGELOG.md`:

- `### Breaking Changes`
  - **native-theme**: `ThemeDefaults` gains `focus_ring_keyboard_only` (see *Added*).
    `ThemeDefaults` is not `#[non_exhaustive]`, so a struct literal of it outside the
    crate needs the new field or `..Default::default()`; its resolved form is
    `#[non_exhaustive]` already.
- `### Added`
  - **native-theme**: `defaults.focus_ring_keyboard_only`, whether the platform shows its
    focus ring only for a focus that came from the keyboard (KDE, Windows, Material), or
    for any focus (iOS); unset, a connector draws the ring on any focus, as before.
    kde-breeze, windows-11 and material-3 state `true`, ios-13 `false`.
  - **native-theme-iced**: `focus_ring` draws the ring, where the theme says keyboard
    only, for a focus that followed a key press, decided when the field gains the focus.
  - **native-theme-egui**: the install plugin paints the ring, where the theme says
    keyboard only, for a focus that followed a key press.
  - **native-theme-gpui**: the theme-drawn `Checkbox`, `Radio`, `Slider` and `Switch`
    draw the ring, where the theme says keyboard only, for a focus that followed a key
    press (`Window::last_input_was_keyboard` when they gain the focus).
  - **Showcases (iced)**: Tab and Shift+Tab move the keyboard focus.
- `### Changed`
  - **native-theme-gpui**: where the theme says keyboard only, `apply` turns
    gpui-component's focus band off (`Theme::focus_ring`) and makes `ring` opaque:
    gpui-component draws its band on a click into a text field, a select or a switch.
    A focused control shows its border, or a 1 px line, in `ring` (a borderless button
    but a link, a text or an upstream `.ghost()` one: a 1 px line in its text colour at
    60 %). windows-11's light
    `ring` loses its last tenth of transparency (#000000e4 to #000000).
  - **Showcases (gpui)**: the Widget Info of the InputGroup, the Switch, the
    ColorPicker and the TimeField claims gpui-component's focus band only where it is
    drawn (`Theme::focus_ring` on), as the Carousel's does; where it is off it says so
    and claims the focused border in `ring` (2H.3).
  - **Showcases**: on kde-breeze, windows-11 and material-3 the Basic page's field
    that has the focus at start-up shows its focused frame without the ring, in all
    three showcases, since that focus follows no key (2H.1); their screenshots change
    (2H.5).
- `### Fixed`
  - **native-theme**: kde-breeze and the KDE reader: the focus ring is Breeze's, the
    selection colour at 30 % (#3daee94d), 2 px wide from 1 px inside the control to 1 px
    outside it, where it was the opaque focus colour, 1 px wide, 2 px away; the KDE
    portal's accent is applied at the same alpha. The iced and egui connectors and the
    gpui connector's own controls draw it only after a key press, as Breeze does.

Under D0 (b) the heading is the minor version the maintainer gives, and the *Breaking
Changes* entry is the same; under (c) the entry is dropped (a field of a
`#[non_exhaustive]` struct is no break) and the release that made the structs
`#[non_exhaustive]` carries the one note.

### 2I.2 `ROADMAP.md`

The release's one section (S5), after the latest release's section, before *Model
completeness* (`:112`): today that is *v0.7.1*, which ends at `:110`; where a release
before this one (egui 0.37, D10) has added a section after it, the new section follows
that one. Unit 2.7 writes its heading, which names both steps, and its first
**Delivered** paragraph, step 2's; unit 3.3 adds step 3's paragraph after it (3H.2):

```markdown
## v0.7.3 — the focus ring as KDE, Windows and Material show it, and a button's outline on hover and focus (issue #4)

**Delivered:** the model says whether a platform shows its focus ring only for a focus
that came from the keyboard (`defaults.focus_ring_keyboard_only`): KDE, Windows and
Material do, iOS shows it on any focus, and where a theme does not say (GNOME, whose
rule has a time-out, macOS, the colour schemes) the connectors draw it as before. The
iced and egui connectors and the gpui connector's own controls draw the ring only for a
focus that followed a key press. gpui-component draws its band on a click as well and
offers no way to turn that off, so the gpui
connector turns its band off on those platforms; a focused control keeps its focused
border, or a focus line (a borderless button but a link, a text or an upstream
`.ghost()` one: a 1 px line in its text colour at 60 %). kde-breeze states Breeze's
ring: the selection colour at 30 %, 2 px from 1 px inside the control to 1 px outside.
```

The v0.7.1 paragraph's sentence "The platform facts for the focus ring, … are corrected
too, and their values wait for a connector design" is history of that release and stays.

*Also in the 0.7.x releases* (`:262-…`) counts the backlog's open `[v0.7.x]` entries,
"tags 29 open items `[v0.7.x]`" (`:264` today; the number the releases before this step
left there). 2I.5 opens one such entry and closes none (the item it marks done is part of an
entry that stays, and the `[1.0]` entry is not one), so the number becomes what `grep -c
'^- \[ \] \[v0.7.x\]' docs/todo.md` prints on the documents unit's tree (2.7), after
2I.5's edits, in the form step 1's specification sets (its G.2); the commit body records
it. Today the grep prints 41, against the 29 the line states.

### 2I.2a The READMEs' lists of optional resolved values

`ResolvedDefaults::focus_ring_keyboard_only` is an `Option<bool>` (2A.2), so it joins the
two READMEs' lists of the values a resolved theme may leave `None`; neither list names a
`defaults.*` field today (read today):

- `native-theme/README.md:51`, the *Behaviour* item of `ResolvedTheme`'s list
  (`:48-52`), one line edited in place: after "…`expander.frame_enabled` and
  `tab.active_indicator_side`, where the toolkit's own value applies too;" insert
  " `defaults.focus_ring_keyboard_only`, where unstated a connector draws the focus ring
  on any focus;", before "and `tab.active_outward_corner_radius`, where unstated the
  selected tab's feet are square."
- `README.md:70-74`, `ResolvedTheme`'s list, whose items have the form "`<field>`:
  where it is `None`, …;": a new item after the `tab.active_outward_corner_radius` one
  (`:73`) and before the switch outlines' (`:74`): "  - `defaults.focus_ring_keyboard_only`:
  where it is `None`, a connector draws the focus ring on any focus;".

No count changes: the field is no colour, and both READMEs' 34 counts colours (3H.2a).

### 2I.3 The first rationale (`docs/issues/0004_kde-hand-overrides-rationale.md`)

- Table row B4 (`:1022`): "a sound choice for now; documented (§4.8); the ring's values
  wait for a design (§5.1)" → "**fixed in 0.7.3**: Breeze's ring values, drawn only after
  a key in the iced and egui connectors and the gpui connector's own controls; on KDE the
  gpui connector turns gpui-component's band off itself (open-items rationale D4), so
  the override is no longer needed".
- **§5.1** (`:837`), a line after the heading, in the form step 1's specification gives
  §5.5 and §5.6 (its G.3): "*Settled by `0004_kde-open-items-rationale.md` D4 (0.7.3):
  kde-breeze and the KDE reader state Breeze's band; `defaults.focus_ring_keyboard_only`
  says KDE, Windows and Material show it for keyboard focus only, which the iced and
  egui connectors and the gpui connector's own controls follow, and the gpui connector
  turns gpui-component's band off where it is set.*"
- **The reply draft (§8).** All of this applies only while the reply has not been
  posted; once it is, this step edits nothing there, and whether a follow-up is written
  is the maintainer's. Step 1's specification edits the draft before this step (its
  G.3), and so may other work before it is posted, so every edit here is placed by
  anchor and by content, never by line or by another specification's wording: a bullet
  is found by its bold title whatever its body then says; what this step fixes moves
  under "Later 0.7.x releases fix more:" with its full text, and what stays open stays
  under "Some things are not fixed yet", never the same item in both; the release is
  written "<the release that brings step 2>" here (and "<the release that brings step
  3>" in 3H.3; one release under S5, and the reply waits for the release of steps 4
  and 5, S5), each replaced by that release's version when the maintainer gives it,
  in the commit that writes it into the draft if the version is known then, else in the
  release's own close-out before the reply is posted; the confirm note keeps every
  sentence still in the draft and its D0/D10 caveat, and gains the new ones.
  1. The bullet whose title is **The focus ring.**, under "Some things are not fixed
     yet", is removed.
  2. After the paragraph that begins "Fixed earlier, in 0.7.0:", a new paragraph and
     list (this step is the first to add it):

     ```markdown
     > Later 0.7.x releases fix more:
     >
     > - **The focus ring** (<the release that brings step 2>). Breeze draws a ring
     >   only when you move with the Tab key; with the mouse it only recolours a text
     >   field's border. native-theme now knows that KDE, Windows and Material show
     >   the ring for keyboard focus only, and gives Breeze's ring its real look, the
     >   highlight colour at 30 %, 2 px wide. gpui-component draws its ring however
     >   the focus came, so on those platforms the connector turns that ring off; a
     >   focused control keeps its coloured border. The check box, radio button,
     >   slider and switch the connector draws itself
     >   (`native_theme_gpui::widgets::{Checkbox, Radio, Slider, Switch}`) draw
     >   Breeze's ring after a key. I have asked gpui-component to draw its ring for
     >   keyboard focus only, which would bring the ring back for Tab.
     ```

     (The four are the theme-drawn controls that draw `FocusRing`, read today:
     `CG/widgets/checkbox.rs:273`, whose rendering `Checkbox` and `Radio` share,
     `widgets/slider.rs:332`, `widgets/switch.rs:457`; they are public through
     `pub mod widgets`, `CG/lib.rs:249`, and `widgets/mod.rs:78-83`.)
  3. After the paragraph that begins "With 0.7.1 (which is built on GPUI Kit 0.9.1)",
     a paragraph of its own: "> With <the release that brings step 2> you can drop
     `focus_ring` as well: the connector turns gpui-component's ring off on KDE
     itself." The sentence of the 0.7.1 paragraph that begins "Still yours to keep:"
     is not edited: it says what 0.7.1 needs, and with 0.7.1 the `focus_ring` override
     is still needed, so it keeps naming `focus_ring` (as step 1 leaves it: "Still
     yours to keep: the 16 px groove (until gpui-component's table is fixed),
     `focus_ring` and `muted` (that one is your application's own choice)."); the new
     paragraph after it then drops `focus_ring` for the later release, and its "as
     well" reads against that list.
  4. The bullet whose title is **Hover borders on buttons.** gets this full text (the
     focus ring and the library update its present wording waits for are then out):

     ```markdown
     > - **Hover borders on buttons.** Breeze shows hover by recolouring a button's
     >   border, not its fill. native-theme has no value for that yet, which is why
     >   you had to type it yourself. It is planned for a coming 0.7.x release. In
     >   GPUI it depends on whether gpui-component's buttons can take it, which I
     >   have yet to try.
     ```
  5. The note after the draft. Of the sentences step 1's note names, "I will ask
     gpui-component", "after an update of the libraries native-theme is built on" and
     "native-theme plans to state that Breeze shows its ring for keyboard focus only"
     leave the draft with item 1, and "It is planned, after the focus ring, in a
     coming 0.7.x release" and "an update of the libraries native-theme is built on
     comes before both" with item 4; so does the parenthesis on putting the update
     first, which no sentence then names. "In GPUI it depends on whether
     gpui-component's buttons can take it, which I have yet to try" stays in item 4's
     bullet and in the note. The note becomes:

     ```markdown
     The sentences "I am reporting it there", "I have asked gpui-component to draw
     its ring for keyboard focus only", "It is planned for a coming 0.7.x release",
     "In GPUI it depends on whether gpui-component's buttons can take it, which I
     have yet to try", "they come in the same series of releases as the hover
     borders" and "it comes last" are the maintainer's to confirm before posting:
     they follow
     `0004_kde-open-items-rationale.md`'s proposed order (D10), and "0.7.x" its
     proposed versions (D0 (a); under (b) or (c) some of these releases are minor
     ones, and the wording changes with them).
     ```

  **The draft's two lists as they stand after this step** (step 1's bullets as its G.3
  gives them; the 0.7.1 list and the paragraphs around it are not repeated):

  ```markdown
  > Later 0.7.x releases fix more:
  >
  > - **The focus ring** (<the release that brings step 2>). [item 2's text]
  >
  > […]
  >
  > Some things are not fixed yet:
  >
  > - **The 16 px scroll bar in tables** is a bug in gpui-component's table, which
  >   ignores the theme's width. I am reporting it there.
  > - **Hover borders on buttons.** [item 4's text]
  > - **Menus in GPUI.** gpui-component paints every menu in its `popover`
  >   colour, the window's on KDE (since 0.7.0, so you no longer need to set
  >   `popover`), and has no colour of its own for a menu, so in GPUI the small
  >   difference from Breeze's menu colour stays.
  ```

### 2I.4 The upstream request (4), `docs/issues/0004_kde-hand-overrides-upstream-requests.md:53-66`

After "…whose focused border should show however it was focused." add: "The decision
would be taken when an element gains the focus, not at each paint: `last_input_was_keyboard`
follows the last event (`src/window.rs:5873-5881`), so a band drawn from it at paint
time would vanish at the next mouse move, as `DataTable`'s focus style does. Until then
native-theme-gpui turns the band off (`Theme::focus_ring`) for themes whose platform
shows its ring for keyboard focus only." The document's other citations stay as they
are (a pinned document, S6).

### 2I.5 The backlog, `docs/todo.md`

- **Done in place**, in step 1's form (its G.5, item 1: an item done inside an entry
  that keeps open items is replaced where it stands, and nothing moves to
  `docs/archive/todo_closed.md` while the entry is open): *KDE values that wait for a
  connector design* (`:1826-1913`), item (1) *The focus ring*. Its text, from "(1)
  **The focus ring**," (`:1829`) to "(the gpui README's overrides section)."
  (`:1854-1855`), becomes "(1) **The focus ring**: done in <the release that brings
  step 2> (below)."; the entry keeps items (2) to (5) and its first sentence. After the
  entry's last line, below step 1's dated note:

  ```markdown
        *Issue #4, open items step 2 (<the release that brings step 2>, <date>, <sha>):*
        (1) done (`docs/issues/0004_kde-open-items-rationale.md` D4). kde-breeze and the
        KDE reader state Breeze's band, the selection colour at alpha 0.3, 2 px from 1 px
        inside the control to 1 px outside (`kde-breeze.toml`
        `[light.defaults] focus_ring_color`, `[dark.defaults] focus_ring_color`), and
        `defaults.focus_ring_keyboard_only` says KDE shows it for keyboard focus only:
        the iced and egui connectors and the gpui connector's own controls draw it after
        a key, and the gpui connector turns gpui-component's band off.
  ```

  Also done in place: in *KDE.* of *Values the v0.7.1 gap map found against their sources*
    (`:1987-2019`), item (9) `defaults.focus_ring_*`: the entry's items run on in one
    paragraph and end in a dated note, "*Issue #4 (0.7.1, 2026-11-14, abb3b761):* …"
    (`:2010-2019`), so the new note goes at the entry's end, after that one, in its
    form: "*Issue #4 (<the release that brings step 2>, <date>, <commit>):* (9) is
    fixed: kde-breeze and the reader state Breeze's band, and the iced and egui
    connectors and the gpui connector's own controls draw it only after a key press."
    The dated note's pointer for (9), "KDE values that wait for a
    connector design" (`:2016-2019`), names an entry whose item (1), *The focus ring*,
    this step marks done in place (above): the note is a record of 0.7.1 and keeps its
    words, and the pointer gains " (its item (1) done in <the release that brings step
    2>)" after the closing quotation mark of "KDE values that wait for a connector
    design" and before the full stop that follows it. The entry stays open for its other
    items.
- **Closed** (moved to `docs/archive/todo_closed.md` under "Closed items from" and their
  heading, with a dated line naming the commit, the file's rule at `:1-13`):
  - `[1.0]` *Two focus claims hold only while `Theme::focus_ring` is on*
    (`:3315-3320`), closed by 2H.3. Its closing line: "*Closed (<the release that
    brings step 2>, <date>, <sha>):* the DataTable panel's keyboard-focus line was
    already conditional ("with the focus ring outside it where the theme's focus_ring
    is on", in `connectors/native-theme-gpui/examples/showcase-gpui/info/data.rs`),
    and the Switch panel's ring claim is now branched on `t.focus_ring`, as
    the Carousel panel's is; kde-breeze, windows-11 and material-3 turn the ring off
    (`docs/issues/0004_kde-open-items-rationale.md` D4)."
- **Changed:**
  - `[upstream]` *GPUI Kit, from issue #4* (`:2742-2764`), request (4): after its
    sentence that ends "…a button's and a check box's border tint is keyboard-only too,
    as Breeze draws them (`docs/platform-facts.md:1108-1110`)." (`:2758-2762`), and
    before "See also", a new sentence: "Until then the connector turns the band off
    where the theme says keyboard only (`defaults.focus_ring_keyboard_only`, 0.7.3)."
    Its pointer
    "*KDE: what a Qt application under Breeze shows that native-theme does not (issue
    #4)*, "KDE values that wait for a connector design", (1)." (`:2762-2764`) names the
    item this step marks done; the item keeps its place (above), so the pointer stays
    and gains ", done in <the release that brings step 2>" after "(1)" and before its
    full stop, written with the placeholder as item (9)'s note is.
  - *platform-facts: which part of a control the focus ring surrounds* (`:511-526`):
    its last sentence, "KDE's ring values wait in *KDE: what a Qt application under
    Breeze shows that native-theme does not (issue #4)*, "KDE values that wait for a
    connector design"." (`:524-526`), becomes "kde-breeze and the KDE reader state
    Breeze's band since 0.7.3; the question of the part it surrounds stays." The
    entry stays open.
  - `[upstream]` *the focus ring's width* (`:2472-…`): append "*0.7.3:* where the theme
    says keyboard only the band is off altogether (`colors.rs`, `focus_band_on`)."
- **Appended**, to item (8) `input.caret_color` of the same *KDE.* entry
  (`:2005-2006`): "kde-breeze states no `input.caret_color`, so the preset's caret is
  `defaults.text_color`, the text colour, while the reader writes `DecorationFocus`
  (`native-theme/src/kde/colors.rs`, the Input block); the rule in
  `docs/inheritance-rules.toml` names `DecorationFocus` since 0.7.3. Measure, then
  state one of the two in the preset." (*Problems found*, 1.)
- **New entry**, `[v0.7.x]`, in the issue-#4 section, right after step 1's *KDE: a
  menu's frame is Qt Quick's, its fill Qt Widgets'* (its G.5, item 1b, which follows
  its item 1a, *Qt Quick's menu fill, for the facts*, after *KDE values that wait for a
  connector design*), and so before *The selection of a view without the focus*:
  "**The focus ring's keyboard rule on GNOME and macOS.** `defaults.focus_ring_keyboard_only`
  is unset for adwaita and the macOS presets (`docs/platform-facts.md`,
  `focus_ring_keyboard_only`). GNOME shows its ring for keyboard focus with a time-out of
  3 s (gtk 4.24.1 `gtkwindow.c:195`, `:6347-6349`), which a yes or no cannot carry; and
  whether a libadwaita text field shows its ring after a click was not read. macOS's
  rule is pending in the parked facts (U331, U611). Read both; then decide whether the
  model needs the time-out."

The citations in this text that name `PF` lines are by key (`focus_ring_keyboard_only`),
not by line (S6). `scripts/check_todo_citations.py` reads preset
key paths and preset line citations: this text has no preset line citation, and the
key paths of item (1)'s dated note, `[light.defaults] focus_ring_color` and
`[dark.defaults] focus_ring_color`, each whole on one line after `kde-breeze.toml`,
resolve in kde-breeze (both keys exist at `0fa28027`, and the step changes their values
only). The note's text, under an entry line, run through the script's `findings()`
gives no finding (read today).

### 2I.6 The rationale's status line

`docs/issues/0004_kde-open-items-rationale.md`'s "Status:" paragraph (`:3-4`, under its
title) gains, as step 1's specification's G.4 and the steps 6–7 specification's 6.H.4
give theirs, one sentence for both steps, written in the release's close-out with 3H.5,
since the documents move to `docs/archive/` in that close-out (3H.6, S5): "Steps 2 (D4)
and 3 (D5) implemented on `<date>` for <the release that brings step 2>, by
`../archive/todo_v0.7.x_kde-open-items-steps2-3-{spec,plan}.md`." (3H.5). The rationale
stays in `docs/issues/` while a step remains.

## Part 2J — The fallback (D10)

Each of the three mechanisms is untried. If one cannot be built as 2E.2, 2F.1 or 2G.1
describe (it does not compile against the toolkit, or its test cannot be made to pass
without a change this specification does not describe), that mechanism keeps drawing
the ring on any focus, which is what the field unset means:

- its code ignores `focus_ring_keyboard_only` (the wrapper's `RingLook::keyboard_only`,
  the plugin's `FocusRing::keyboard_only`, or `ring_shows`, is not added);
- its documentation (the `focus_ring` doc and README of 2F.4, the plugin doc and README
  of 2G.1; for gpui, whose `ring_shows` is then not added and whose `FocusRing` doc is
  not touched, the README of 2E.3, in the form the gpui bullet below gives) says
  instead: "The ring shows on any focus, a click included, also where the theme says the
  platform shows it for keyboard focus only: <toolkit> gives no way to tell", with the
  reason found;
- its tests of 2E.4, 2F.2 or 2G.3 that assert the rule are not added, and the existing
  tests that the rule would have broken stay as they are today: a mechanism that
  ignores the field rings them as before, so the `None` edits of 2E.4, 2F.2, 2G.4 and
  2H.2 are not made for it (for gpui, 2E.4's two doc rewrites are still made, in the
  form the gpui bullet below gives);
- the CHANGELOG's *Added* entry for it is dropped, and a backlog entry records what was
  tried and why it failed;
- the texts of this step that say the connectors draw the ring after a key name only
  the ones that do, and say of the one that fell back that it draws the ring on any
  focus, "<toolkit> gives no way to tell" (with the reason found). Each, with its text
  where <X> is the connector that fell back and <the rest> the others the sentence
  named:
  - the CHANGELOG's *Fixed* entry (2I.1), "The iced and egui connectors and the gpui
    connector's own controls draw it only after a key press, as Breeze does." → "<the
    rest> draw it only after a key press, as Breeze does; <X> draws it on any focus
    (<toolkit> gives no way to tell).";
  - the ROADMAP section (2I.2), "The iced and egui connectors and the gpui connector's
    own controls draw the ring only for a focus that followed a key press." → "<the
    rest> draw the ring only for a focus that followed a key press; <X> draws it on
    any focus, as <toolkit> gives no way to tell.";
  - the first rationale's B4 row (2I.3), "drawn only after a key in the iced and egui
    connectors and the gpui connector's own controls" → "drawn only after a key in
    <the rest>, on any focus in <X>", and its §5.1 line (2I.3), "which the iced and egui
    connectors and the gpui connector's own controls follow" → "which <the rest>
    follow (<X> cannot tell)";
  - the backlog's item (9) note (2I.5), "the iced and egui connectors and the gpui
    connector's own controls draw it only after a key press" → "<the rest> draw it only
    after a key press, <X> on any focus";
  - where <X> is the gpui connector's own controls: the reply's *The focus ring* bullet
    (2I.3, item 2), its sentence "The check box, radio button, slider and switch the
    connector draws itself (…) draw Breeze's ring after a key." ends "draw Breeze's
    ring, for now on any focus." instead;
    in the README (2E.3), *The focus ring*'s rewrite is made without its clause "; the
    connector's own controls draw the theme's ring after a key (below)" (the sentence
    then ends "…also when Tab reached it, where the platform draws its ring."), and the
    fallback sentence above takes the place of *Keyboard focus*'s insert only (that
    section's last sentence still gains ", where `Theme::focus_ring` is on");
    2E.4's two doc rewrites are kept, since the old "outside the control" and "outside
    the track" turn false through unit 2.1's preset values either way:
    `a_switch_told_not_to_draws_no_focus_ring`'s as 2E.4 gives it, and
    `every_focused_control_draws_the_ring`'s without "after a key press": "A focused
    Checkbox, Radio, Switch and Slider each draw the theme's ring (C-06): kde-breeze's
    across the control's edge, from 1 px inside it to 1 px outside; adwaita's inset one
    as a band inside the edge of the borderless row, track and thumb." (its key presses
    are not added);
    the gpui showcase's `HOW` entry (2H.3) loses "and whether the theme-drawn
    Checkbox, Radio, Slider and Switch ring a focus that did not follow a key
    (widgets/mod.rs, ring_shows)", and `theme_focus_ring`'s added clause (2H.3) is
    not added;
  - where <X> is iced: `iced_route`'s arm 2 (2H.2) gets its own fallback text, since
    arm 3's "(the theme does not say keyboard only)" would be false for windows-11 and
    kde-breeze, which say it: where `ring_round_the_field(resolved)` and the field is
    `Some(true)`, "native_theme_iced::focus_ring: the ring on any focus, though the
    theme says keyboard only (iced gives no way to tell a key's focus: <the reason
    found>)"; arm 3 stays for the rest. `the_focus_ring_routes_name_the_border_that_draws_it`
    then expects, for windows-11 and kde-breeze, a route that begins "iced:
    native_theme_iced::focus_ring: " and contains "iced gives no way to tell a key's
    focus", and for macos-tahoe arm 3's line as before. The tests not added are 2F.2's
    `where_the_theme_says_keyboard_only_a_click_draws_no_ring`,
    `a_field_focused_before_any_input_shows_no_ring_where_keyboard_only` and
    `a_modifier_key_alone_is_no_key`, and 2H.2's split (b) of
    `the_focused_field_draws_the_themes_ring_round_it` (the preset unchanged, clicked,
    no ring): that test keeps its windows-11 case as today, with no field set to
    `None`, since the ring then shows on the click. 2F.2's `None` edits of the
    existing tests are not needed either and are not made. The showcase's Tab handler
    (2F.5, its test, the CHANGELOG's *Showcases (iced)* entry) stays: Tab moving the
    focus does not depend on the rule. Of 2F.4, the sentences on how iced moves the
    focus by Tab (the `focus_ring()` doc's "iced moves the focus by Tab only where the
    application does: …" and the README's "iced moves the focus by Tab only if the
    application does: …") stay, and the sentences on the rule are replaced by the
    fallback text above;
  - where <X> is egui: the plugin reads no field, so the `mapping.toml` row of 2G.2
    names no test that checks it: it becomes `verdict = "unmappable"` with
    `sub_tag = "egui-limited"` and an `upstream` naming what egui lacks, and 2G.2's
    group totals and sub-tag counts are taken again from the regenerated `mapping.md`
    (not computed here); the egui showcase's route follows the row.

The rest of the step does not depend on it: the field, the facts, the presets, the
reader, and gpui-component's band switch and opaque `ring` (2E.1).

## Part 2K — Order and gates (step 2, and the release)

One branch for both steps, `v0.7.3-rc1` (S5), a worktree as the project's other work.
It is made from the gated head of the release before it in its own lane (the KDE lane:
v0.7.1 → steps 2 and 3 → steps 4 and 5 → the caret → steps 6 and 7, S3 as amended; here
v0.7.1's, the tree with step 1, D10 (a) being the maintainer's answer of
2026-11-15), as soon as that release's gated head (its documents unit, rule 20) has
passed its gate and its reviews are closed (this release reads those texts), not
from its tag (S2); v0.7.1's gated head is MK4. It is merged with the tag of the release
immediately before it in the maintainer's release order (`<prev-tag>`; under D10's
order egui 0.37's, `v0.7.2`), which may be another lane's: `git -c
merge.conflictStyle=diff3 merge --no-ff --no-commit -m "Merge <prev-tag> into
v0.7.3-rc1" <prev-tag>` once, concluded by `git commit` after the resolutions and
regenerations (rule 17; S2 as amended on 2026-11-15: a merge, not a rebase, so no sha
changes), after the release's last unit is committed and its reviews are closed and
the tag exists, before the release's check X, version and close-out, never while a
trial's or a unit's edits are uncommitted; `<lane-pred-head>` is the lane
predecessor's head the branch was made from, and `<base>` the branch's base
(`<prev-tag>` where the branch was made from it, else `<lane-pred-head>`). Where the
branch was made from `<prev-tag>` itself, there is no merge. The last unit before
the merge, the documents unit 3.3, takes its cadence's gate, `--quick`, with no extra
`--commit` (unit 3.2's `--commit` covers the code beneath it), and its commit is the
lane's gated head that the next release (steps 4 and 5) starts from (rule 20). The merge then runs in rule 21's order,
(1), (3), (6), (7), (4), (5): (1) the merge itself: conflicts resolved by the
orchestrator by both sides' texts, a citation like any other text (S6; but a line a
gate reads by number, S6's fourth gate: the side that re-pointed it wins, and where
both did, the number is the base plus both sides' deltas, confirmed by the warm-up
gate), and named in the record; generated files regenerated, not resolved by hand; counts and verdict
tuples a test or gate checks as the base plus both sides' deltas, a prose count
`<prev-tag>`'s side; in `CHANGELOG.md` this release's section goes first and in
`ROADMAP.md` it follows `<prev-tag>`'s (2I.2); (3) where the merge brought another
version of a toolkit this specification cites (under D10's order `<prev-tag>` is egui
0.37's, which moves egui and egui-winit from 0.36.2 to 0.37.0), every toolkit form a
unit wrote into a comment, a doc or a test (an `EG` citation, the modifier keys of
2G.1) is read again in the new version and rewritten only where its content changed
(rule 25); a citation pinned to its version keeps its version (S6); (6) the path
check of the two documents' paths and links; (7) checks that this release's texts and
counts survived the merge, on HEAD as it stands after (3) and (6), VF's rewrites being
expected differences, never undone; (4) a resolution in any file but Markdown no test
parses, and every commit of steps (3), (6) and (7), is reviewed (`review`, or
`review-hard` where a hard part of a core changes); (5) the warm-up gate (`--commit`
on the clean tree, with the GPUI Fast legs, since the merge brings the other side's
code), last, which covers every commit of those steps and builds the units against the
merged toolkit versions. X then recounts, on the merged tree, every stated count in
the files either side changed, and runs the five documentation checkers (*X*,
below). Nothing of
this release is pushed before that merge, and its close-out starts only after it, so
its compatibility and semver steps compare with the released predecessor.

**Commit (0), the documents.** v0.7.1's commit (0) adds every document of this cycle
the maintainer has approved by then (`docs/todo_v0.7.1_gpui-kit-0.9.1-spec.md`,
*Order and gates*, its item (0), cited by section since that document is still being
edited), the rationale and this specification among them if they
are approved by then. This work's own first commit, commit (0), adds to `docs/` the
approved documents of this work that no earlier commit added: its plan at least, and the
rationale and this specification where v0.7.1's commit (0) did not add them. Where
nothing is left to add, there is no commit (0) and the step starts with unit 2.1. A
document not approved when the work starts stays untracked and outside the work. A text
that cites one of them at a commit cites the commit that added it.

**Units.** Each unit is one commit (or a run of commits whose last one closes it), gated
by the batch's cadence (S1; `--quick` as `docs/todo_v0.7.1_gate-speed-spec.md` defines
it): during a unit, the agent runs only the touched tests (its targeted commands); at
a unit's end, `--quick`, by the orchestrator, as a long run until its time is measured
(the first timed into the record); `--commit` every third code unit of the release,
counted from the last `--commit` (any `--commit`, a warm-up gate's or a fix's
included, restarts the count, while a `--quick`-gated fix does not count toward the
next third), on the release's last code unit before its documents and version units
(rule 6: 3.2, which holds 3H.2b's TOML string so that 3.3 stays a documents unit),
after a lock or toolchain change (the warm-up gates, P2's and the merge's above, and
the version step, whose `Cargo.lock` lists the workspace crates' new versions), and
wherever a unit is structural (rule 24: one that changes a
manifest, `Cargo.lock`, a build script, the toolchain or the gate scripts; none here,
new model fields being no reason on their own, so the count of three marks 2.3 and
2.6 where no other `--commit` falls between, and 3.1 and 3.2 are the first and second
code units after 2.6's); a unit is a documents unit only where it changes nothing but comments,
rustdoc prose, READMEs and Markdown (`docs/platform-facts.md` counts as code: tests
parse it); a documents unit (2.7, and 3.3, the gated head, rule 20) runs the five documentation checkers and `--quick`; the full run where the plan has it (the close-out), unchanged. A commit
between two `--commit` runs is proven by `--quick` only; a failure the next `--commit`
finds is fixed by a new commit on the branch, never by rewriting a pushed one, and the
record names the commit that brought it. The script's full run and `--commit` differ
in the release-only checks `--commit` skips (the release tag, gpui on GPUI Fast, the
ignored stem-index test, the docs.rs emulation, packaging, semver, outdated
dependencies, the asset and compatibility stamps; `scripts/check_release.sh:26-37`).
What a test that walks or counts the model's leaves needs goes into the unit that makes
the test red:

| Unit | Contents | Why these belong together |
|---|---|---|
| 2.1 The field and its values | 2A (the field, `validate`, the three full literals of 2A.4, the baseline and new tests); 2B.1 (the facts row); 2B.2 (the registry line); 2B.3 (the `[no_inheritance]` entry, the caret rule); 2B.5 (the four presets); 2G.2 (egui's mapping row, `mapping.md` regenerated, the leaf count 516 → 517, the foundation totals `[18, 1, 36, 11]` → `[18, 1, 37, 11]`) | The field turns red, at once: `theme_defaults_field_info_matches_baseline` and `property_registry_keys_match_derive_inventory` (`model/mod.rs:1973`, `:2055`) until the registry line; `check_platform_facts_rows.py` until the facts row; egui's `every_leaf_has_exactly_one_row_and_every_row_is_well_formed` (`CE/mapping_tests.rs:234-262`), since `ResolvedDefaults` serialises the unset field as `null`, a leaf of its walk, until the row and the count; `verdict_totals_match_the_specification` (`:443-497`) until its foundation totals; `mapping_md_is_current` (`CE/mapping_doc.rs:347`) until `mapping.md`; egui's T2, `t2_every_row_s_declared_sinks_are_exactly_what_its_leaf_moves` (`:1334-1437`), until a preset pair differs in the leaf: it walks every row that is not unmappable, splices the leaf only between the pairs of `PAIRS` (`:905-918`) where the two differ (`:1205-1216`), and fails a row no pair exercises and that has no `probe` (`:1433-1436`); with the presets in the unit, kde-breeze (`true`) against adwaita (`null`) and material-3 (`true`) against catppuccin-mocha (`null`) differ, so no `probe` is needed (`toml_to_json` would handle a boolean one, `:1218-1234`); the two full literals until they name it. Not red: iced's and gpui's colour-leaf walks (`CI/contract.rs:5578`, `CG/contract.rs:2933`) take a `null` leaf only where a colour splices into it (`CI/contract.rs:4889-4898`, `CG/contract.rs:2696-2706`), and a `bool` does not; no showcase lists the leaf yet; no connector reads the field yet, so no ring test changes. The preset comments name the facts row of the same unit by section and key (2B.5). iced's export, `connectors/native-theme-iced/contract-leaves.toml`, is generated from the colour leaves of the bundled presets (`reach`, `CI/contract.rs:5262-…`; the file's header): the new `bool` leaf is not one, and kde-breeze's new ring colour still moves the one row `defaults.focus_ring_color` moves today, `crate::focus_ring_color`, in every combination (its `[leaves."defaults.focus_ring_color"]` entry has no `except`), since the walk splices a sentinel colour, not the preset's own. So the unit is expected to leave the export as it is; the implementer runs `regenerate_contract_leaves` (`CI/contract.rs:5539`) at its end and commits a difference if one appears, which `every_colour_leaf_reaches_a_row_or_is_named` would report. |
| 2.2 The reader | 2C (the reader, the portal path, their tests, `kde/mod.rs`' two lists and the accent test's list) | No test pins kde-breeze's old ring but the reader's own, which change with it (2C.1); `the_derived_colours_are_the_presets_under_breeze` then holds the reader's new ring to unit 2.1's preset. The reader is not an input of iced's export (the walk splices the bundled presets), so the export does not change here. Contrast pairs: no pair of gpui's (`CG/contract.rs:1917-…`) or iced's (`CI/contract/pairs.rs`) names `ring`, the caret, the focus ring colour or a border (read today with `grep`), and kde-breeze's `ring` token keeps its value (2E.1), so no pair moves. |
| 2.3 gpui | 2E (with 2E.3's clause in `scripts/connector_parity_exceptions.toml`); the gpui showcase's claims and texts of 2H.3 but the new leaf's `HOW` entry, which goes to unit 2.6 with the leaf; the edit of the existing `focus_ring_color` `HOW` entry (adding `ring_colour`, 2H.3) is in this unit, with the `ring_colour` it names, and so are the edit of the `focus_ring_width_px` entry (naming `focus_band_on`) and the Theme Map's Ring row (naming `ring_colour`), 2H.3 | `ring_colour` makes the `ring` row red on kde-breeze and windows-11 light until it is a computed row (2E.3); `ring_shows` makes `every_focused_control_draws_the_ring` red until its key press (2E.4). |
| 2.4 iced | 2F; the showcase's Tab handler (2F.5) and its test `the_focused_field_draws_the_themes_ring_round_it` (2H.2) | The wrapper's rule makes red, until they change: `a_focused_field_is_ringed_by_the_themes_ring`, `the_ring_follows_the_focus_through_update`, `the_focus_survives_a_theme_that_draws_no_ring` and `a_replaced_field_loses_the_ring` (2F.2), and the showcase's test, all of which focus a field under windows-11 by a click or before any input and expect a ring. |
| 2.5 egui | 2G.1, 2G.3, 2G.4, 2G.5 | The plugin's rule makes `a_widget_focused_in_the_body_is_ringed_at_the_callers_radius` red until the test sets the field to `None` on the theme it installs (2G.4, an edit of the test's body, not of its helper). |
| 2.6 Showcase leaf | 2B.4 (the element list); its routes in the three showcases: iced's (2H.2), gpui's `HOW` entry (2H.3), egui's from unit 2.1's mapping row (2H.4) | A listed leaf without a route is red in `the_element_list_parses_and_every_leaf_has_a_route` (`showcase-iced.rs:24982`) and `every_listed_leaf_has_a_route` (`showcase-gpui/tests.rs:15198`); `check_showcase_parity.py --check-list` needs the registry key of unit 2.1. |
| 2.7 Documents | 2I (with `ROADMAP.md:264`'s recount, 2I.2, and the two READMEs' new optional value, 2I.2a) | `check_todo_citations.py` reads the backlog edits. |

Units 2.3 to 2.5 are independent of one another; each may fall back as 2J says.

**X, once for the release, after 3.3 and the merge** (rule 17; S6). No citation is
re-pointed: the existing line citations of the files this release changes are left as
they are and may drift (2B.6). X recounts, on the merged tree, every stated count in
every file either side of the merge changed (the union of `git diff --name-only <base>
<the merge's first parent>` and `git diff --name-only <base> <prev-tag>`, `<base>` the
branch's base: `<prev-tag>` where the branch was made from it, else `<lane-pred-head>`),
outside the pinned documents, and runs the five documentation checkers. A generated file
is regenerated by its command, never edited. With no merge there is no X (rule 17):
nothing was merged, and unit 3.3 ran the five checkers.

**The close-out, once for the release, after step 3's units, the merge and X** (S5, S2, S4):
the version, the compatibility run, the push once it has passed, the full
`scripts/check_release.sh` while CI runs on the push, then, when both are green, the
one asset run (`scripts/generate_assets_release.sh`, the maintainer's desktop) and the
release stamps, the documents' archive last (3H.5, 3H.6). The asset run compares the three showcases at rest under the six captured presets (its
step 6, `scripts/generate_assets_release.sh:158-167`); `scripts/check_showcase_parity.py`
lists at the end each exception whose scope it compared but that matched no difference
(its doc, `:249-250`). The implementer reads that list: this step changes what the
start-up field shows on kde-breeze, windows-11 and material-3 (2H.1, 2H.5), so an
exception written for the old ring can go stale; each one listed is scoped or removed
in a commit of its own (step 3 changes no capture, 3G.2, and its exceptions are read in
the same list). The units of both steps are committed in order and not pushed one by
one (the batch's rule, as step 1's specification gives it in its H.1,
`docs/todo_v0.7.1_kde-open-items-step1-spec.md`): they are pushed together at the
release's end, after step 3's last unit, the merge, X and the version, once the
compatibility run has passed (S4), before the asset run, which refuses an unpushed
branch and dispatches the screenshots workflow on it for the Windows and macOS captures
(`scripts/generate_assets_release.sh:74-103`; 2H.5); the asset run's commit is pushed
after it. CI is checked on each of these pushes, and a red run is fixed before the
next.

**Semver.** The compatibility step (`scripts/check_release.sh:1128-1146` and on, soft,
skipped under `--commit`) of the release's one full run reports `ThemeDefaults`' new
field and `ButtonTheme`'s two (3A.1) on native-theme: under D0 (a) that is the approved
break the CHANGELOG names once (2I.1, 3H.1). The connectors' public APIs gain nothing (no new `pub fn`;
`scripts/check_connector_parity.py` has nothing new to compare). Checked against each
crate's coverage tests: step 2 adds no `pub fn` to gpui's `variants.rs` or
`geometry.rs` (`the_showcase_exercises_every_builder`, `every_geometry_builder_has_a_note`,
`CG/showcase.rs:383-416`, `:572`; its new functions are `pub(crate)` or private), no
widget constructor to the iced showcase (`styles_cover_every_widget_shown`,
`showcase-iced.rs:20919`; 2F.5 adds two messages and a key arm), and no element id to
the egui showcase (`the_layout_dump_names_every_listed_element_the_basic_page_draws`,
`showcase-egui/tests.rs:8037`; 2B.4 adds a leaf to an existing element). Step 3 adds
one `pub fn`, gpui's `bordered_button` where the trial passes (its unit, 3I), and in
iced and egui none, nor a constructor or an element id (3B.4 adds leaves to existing
elements).

**Under D0 (b)** only the version number differs, wherever this step writes it: the
branch name (`v0.7.3-rc1`), the CHANGELOG heading (2I.1), the ROADMAP heading (2I.2),
the first rationale's B4 row ("fixed in 0.7.3") and §5.1 line ("D4 (0.7.3)") (2I.3),
and the backlog lines of 2I.5 (item (9)'s note and its pointer's annotation, written
with the release placeholder in any case, as are item (1)'s "done in" and its dated
note, and request (4)'s pointer annotation; request (4)'s new sentence's "0.7.3"; the
focus-ring-part entry's "since 0.7.3", the ring-width entry's
"*0.7.3:*", the caret item's "since 0.7.3"); the reply's version is the placeholder of
2I.3 in any case. Outside the reply, a release placeholder ("<the release that brings
step 2>" and its step 3 form, in 2I.5, 2I.6, 3H.5 and 3H.6) is written with the
version the maintainer gave for the release, given before the work starts (D0,
whose decision every step from 2 on needs, the rationale's D10). Step 3's number is
the same one (S5), in the same places of 3H (row C, the §5.4 line, the backlog's
"since 0.7.3"); the CHANGELOG and ROADMAP sections and the branch are the release's
one.
**Under (c)** the release that makes the model's input structs `#[non_exhaustive]` comes
first; then `tests/proptest_roundtrip.rs`, an integration test outside the crate, can no
longer build `ThemeDefaults` by struct literal at all, and 2A.4's edit becomes a build
from `Default` with the fields set; the CHANGELOG has no *Breaking Changes* entry here.

**The release** (tag, publish, GitHub release) is the maintainer's; so are sending the
upstream request and posting the reply.

---

# Step 3 — D5: a button's border on hover and on keyboard focus

## Part 3A — The model fields

### 3A.1 `native-theme/src/model/widgets/mod.rs`, `ButtonTheme`

After `checked_text_color` (`:94-97`), before `font` (`:98`), as `InputTheme` declares
its pair (`:141-146`):

```rust
    /// Border colour of a hovered button, which a pressed button keeps (it is under
    /// the pointer). `None` where the theme states none; a connector then draws
    /// `border.color` (docs/platform-facts.md §2.3). The KDE reader derives it from
    /// the scheme's selection colour; an accent the portal gives replaces
    /// `selection_background` and the focus ring but not this border: it moves no
    /// widget's own read of that colour (the check box's border, the blends).
    #[theme(category = "soft_option")]
    pub hover_border_color: Option<Rgba>,
    /// Border colour of a button with the keyboard focus. `None` where the theme
    /// states none; a connector then draws the hover border, else `border.color`, where
    /// it shows a focused button by its border (docs/platform-facts.md §2.3). From the
    /// KDE reader as `hover_border_color`.
    #[theme(category = "soft_option")]
    pub focus_border_color: Option<Rgba>,
```

What the facts say of other platforms (3B.1): GNOME and Material draw no button
outline in any state, Windows keeps its rest outline on hover, macOS 26 draws nothing on
hover; for a focused button on Windows and macOS, and for hover on macOS 14–15, the
outline was not read. So unset means the theme states none, not a platform fact.

The derive makes them `Option<Rgba>` in `ResolvedButtonTheme` too (`soft_option`,
`native-theme-derive/src/gen_structs.rs:38`), which is `#[non_exhaustive]`
(`gen_structs.rs:26`); `ButtonTheme` is not, so the step is the break D0 names.

### 3A.2 Tests that name every field

`native-theme/tests/proptest_roundtrip.rs`, `arb_button_theme` (`:226-259`): the colour
vector grows from 11 to 13 (`11..=11` → `13..=13`), and the literal gains
`hover_border_color: colors[11], focus_border_color: colors[12],`; the comment "all 9
color/scalar fields + 4 more colors" (`:227`) is corrected to the count it then has:
13 colours, 4 scalars, the font and the border.

**Every struct literal** of `ButtonTheme` and `ResolvedButtonTheme`, found today with
`grep -nE '(ButtonTheme|ResolvedButtonTheme) \{'` over every tracked file outside
`docs/archive/` and each literal read to its closing brace with `python3`. Two name
every field and must gain the two new ones:
`native-theme/tests/proptest_roundtrip.rs:239-257` (above), and the in-crate test
`button_validate_widget_extracts_all_fields`, `native-theme/src/model/widgets/mod.rs:1833-1866`,
which gains `hover_border_color: Some(Rgba::rgb(0, 120, 215)),` and
`focus_border_color: None,` after `checked_text_color: None,`, and, beside its
soft-option assertions (`:1876-1879`), `assert_eq!(resolved.hover_border_color,
Some(Rgba::rgb(0, 120, 215)));` and `assert_eq!(resolved.focus_border_color, None);`
(test inputs, as the test's other colours are). The other five end in
`..Default::default()` and need nothing: `native-theme/src/model/widgets/mod.rs:1423`,
`:1433`, `:1442`, `:1458`; `native-theme/tests/merge_behavior.rs:341`.
`ResolvedButtonTheme` is `#[non_exhaustive]` and has no literal outside the derive's
generated code. `docs/todo_v0.7.1_model-completeness-spec.md:1215` declares a
`struct ButtonTheme` of the parked design, not a literal of this one.

## Part 3B — Facts, rules, registry, presets

### 3B.1 `docs/platform-facts.md`: two rows

Directly before `hover_background` (`:1192`), the button's other hover row (decision
2). Two lines:

```text
| `hover_border_color` | macOS 14–15: ❓ — not measured · macOS 26: **(none)** — hover draws nothing on a push button **(measured)** (OS probe run 40969040283, in `docs/facts/macos/state-models.toml:1-3` at `78ee0efd`); its bezel has no stroke (`docs/facts/macos/button.toml:26916-26924` at `78ee0efd`) | **(none)** — the outline does not change: `ButtonBorderBrushPointerOver` is the rest brush, `ControlElevationBorderBrush`, a gradient the model's one border colour cannot state ([Button_themeresources.xaml:38-39](https://github.com/microsoft/microsoft-ui-xaml/blob/ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd/src/controls/dev/CommonStyles/Button_themeresources.xaml#L38-L39)) | **(Breeze src)** `Highlight`, `[Colors:Selection] BackgroundNormal`: a non-flat button's pen turns the active group's highlight colour while it is hovered, has the visual focus or is pressed ([breezehelper.cpp:710-712](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L710-L712), [:672-673](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L672-L673)) — Breeze and Breeze Dark #3daee9; a pressed button keeps it | **(none)** — a button has no border in any state: GTK's border style is `none` by default ([gtkcssstylepropertyimpl.c:1072-1077](https://gitlab.gnome.org/GNOME/gtk/-/blob/4.24.1/gtk/gtkcssstylepropertyimpl.c#L1072-L1077)) and libadwaita sets none (`border.color`) | **(none)** — a filled or tonal button has no outline in any state (`border: BorderStroke? = null`, [Button.kt:146](https://github.com/androidx/androidx/blob/e4d933c98df7baa7ea2b83eea6e086915961e0ff/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Button.kt#L146), at androidx `e4d933c9`, the parked facts' pin) |
| `focus_border_color` | ❓ — AppKit shows keyboard focus with the focus ring (§2.1.5); no source states the bezel line of a focused button · macOS 26: ❓ — the same | ❓ — the facts state no outline of a focused button; the focus visual is §2.1.5's | **(Breeze src)** `Highlight`, as `hover_border_color`: the same pen turns it on keyboard focus (`visualFocus`, [breezehelper.cpp:710-712](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L710-L712)) — #3daee9 | **(none)** — no border (`hover_border_color`) | **(none)** — no outline (`hover_border_color`) |
```

Sources, read today in `P8`: `F/kde/button.toml:2097-2111` (hover), `:2285-2299`
(focus_visible), `:2697-2711` (pressed), each `= active(role.selection.background)`
citing `breezehelper.cpp:710-712` and `:672-673` at `breeze@6.8.2` (the first
specification's facts row `PF:1108` links the same lines at `f0b1d75`);
`F/windows/button.toml:3643-3749` (rest), `:3851-3957` (hover), the same gradient,
citing `Button_themeresources.xaml:38`, `:39` at `microsoft-ui-xaml@2.5.1`;
`F/gnome/button.toml:164-172`; `F/material/button.toml:205-222`;
`F/macos/state-models.toml:1-3`, `F/macos/button.toml:26916-26924`;
`F/ios/button.toml:18116-18124` (iOS: stroke absent, measured; no column). Three links
are built from the parked facts' pinned commits and paths and were **not opened** (no
network): the WinUI one, the GTK one, whose form is `PF`'s other GTK links', and the
androidx one, built as 2B.1's (the full commit of `androidx@e4d933c9`,
`F/../sources.toml:837`, on the GitHub mirror, `:843`). The
focus-visible state of a WinUI or Material button has no stroke fact in `F`
(`grep focus_visible` over `F/windows/button.toml` finds only the behaviour token at
`:17612`): hence Windows' ❓, and Material's "(none)" rests on the stateless `absent`
of `F/material/button.toml:215-217`.

### 3B.2 `docs/property-registry.toml`

`[button]` (`:180-197`): after `checked_text_color` (`:196`),

```toml
hover_border_color           = "color"
focus_border_color           = "color"
```

As 2B.2: no citation of a registry line is re-pointed (S6).

### 3B.3 `docs/inheritance-rules.toml`

`[no_inheritance]`, after `"button.checked_text_color",` (`:412`):

```toml
    "button.hover_border_color",       # None: the theme states none, border.color stays (§2.3)
    "button.focus_border_color",       # None: the theme states none, the hover border or border.color (§2.3)
```

The entries go beside the button's other soft options; the existing citations of the
lines they move are left as they are (S6, 2B.6).

### 3B.4 `docs/showcase-elements.toml`

The elements whose button wears `button.border` and can be hovered:
`basic.buttons.default` (`:734-…`), `basic.buttons.primary` (`:751-…`),
`basic.buttons.tooltip` (`:785-…`), `basic.buttons.toggle_off` (`:823-…`) and
`basic.buttons.toggle_on` (`:842-…`) gain `"button.hover_border_color",
"button.focus_border_color"` immediately before `"button.border.color"`, which in each
list follows the button's own scalar colours: the registry puts the two after
`checked_text_color` and before `border` (3B.2). `basic.buttons.disabled` does not (a
disabled button is not hovered).

### 3B.5 The presets

**kde-breeze**, `[light.button]` after `checked_text_color = "#232629"` (`:147`) and
`[dark.button]` after `checked_text_color = "#fcfcfc"` (`:732`), each found by its
text, four lines each, one comment covering both values as 2B.5's covers the ring's
width and offset:

```toml
# platform-facts.md (§2.3): hover_border_color and focus_border_color: a non-flat button's pen turns Highlight, [Colors:Selection] BackgroundNormal 61,174,233
# (BreezeLight.colors:87, breeze v6.7.5), while hovered, pressed or keyboard-focused (breezehelper.cpp:710-712 at breeze f0b1d75)
hover_border_color = "#3daee9"
focus_border_color = "#3daee9"
```

in dark with `(BreezeDark.colors:88, breeze v6.7.5)`. The inserts move later lines of
`kde-breeze.toml` that comments and assertion messages elsewhere cite (read today);
those citations are left as they are (S6). No gate reads them.

**No other preset states them**, each for its facts row's reason: adwaita (GNOME: no
border in any state), windows-11 (the outline is the rest brush on hover; focus: no
fact), macos-monterey (not measured) and macos-tahoe (hover draws nothing), material-3
(no outline), ios-13 (no stroke, `F/ios/button.toml:18116-18124`), and the ten colour
schemes (no platform).

### 3B.6 Citations the two rows move

The rows move `:1192` and every later line down two (3B.1). The existing citations of
those lines (presets' comments, connectors' comments and messages, `docs/todo.md`) are
left as they are and may drift, and no gate reads a facts line by number from v0.7.1
on (2B.6, S6).

## Part 3C — The KDE reader

`native-theme/src/kde/colors.rs`, the Button block (`:513-541`), after
`variant.button.hover_text_color = button_fg;` (`:541`), using step 2's `selection_bg`
binding (2C.1):

```rust
    // The pen of a non-flat button turns the active group's `Highlight` while it is
    // hovered, has the visual focus or is pressed (breezehelper.cpp:672-673, :710-712 at
    // breeze f0b1d75; docs/platform-facts.md §2.3): the hover border, which a pressed
    // button keeps, and the focus border.
    variant.button.hover_border_color = selection_bg;
    variant.button.focus_border_color = selection_bg;
```

The comment at `:536-539` ("A hovered button keeps its fill and its text: Breeze's hover
changes only the pen …") stays. Values: #3daee9 in both Breeze fixtures; 246,116,0 in
`custom-accent.ini` (read today).

The portal's accent (2C.2, `apply_kde_accent`, through `apply_accent`,
`gnome/mod.rs:150-156`) recolours the semantic roles it sets, the accent, the
selection, the ring (after step 2) and the tab indicator, not these two, which are a
widget's own reads of the selection colour: like the check box's border
(`kde/colors.rs:586-588`), they are read from kdeglobals' `[Colors:Selection]
BackgroundNormal`, as the reader's widget reads of it are (the blends: the checked check
box, the progress fill, the switch's on track, the list's hover), which
`populate_colors`' doc says the portal's accent does not move (`kde/colors.rs:464-470`). Under a portal accent that differs from
the scheme's selection colour, a hovered button keeps the scheme's highlight. The field
docs (3A.1) say so. Whether the KDE portal's accent can differ from the scheme's focus
colour is the rationale's §4 open item. The other way, `apply_kde_accent` setting the
two borders to the accent as well, is not taken: wherever the portal's accent is the
scheme's selection colour (whether it can be another is that open item), it gives the
same colour, and it would make the two borders the one widget's own read of the
selection colour the portal moves, unlike the check box's border beside them; keeping
them with the reader's widget reads of it is the simpler rule.

Tests: in `kde/colors.rs`, `a_buttons_hover_and_focus_border_is_the_selection` for both
Breeze fixtures and the custom scheme of `:1492` (selection 200,40,90); in
`tests/reader_kde.rs`, the dark and custom-accent tests gain the two assertions.

**The lists that hold the reader to the preset** (`kde/mod.rs`, as in 2C.1):
`derived_leaves` and `resolved_leaves` append `("button.hover_border_color",
mode.button.hover_border_color)`, `("button.focus_border_color",
mode.button.focus_border_color)` and, resolved, `theme.button.hover_border_color`,
`theme.button.focus_border_color` (`Option`s already), after step 2's ring; the length
and the doc's count grow by two. Then:

- `the_derived_colours_are_the_presets_under_breeze` checks that the reader's two
  borders under Breeze Light and Dark (selection 61,174,233) are kde-breeze's #3daee9
  of 3B.5, and that the preset states both in both variants.
- The accent test (`:1420`, the `selection` key) gains `"button.hover_border_color",
  "button.focus_border_color"` at the end of its list, after step 2's
  `"defaults.focus_ring_color"`; its doc gains "the button's hover and focus border".
- The other six moved-leaves tests are unchanged: no key but `selection` moves the
  two.

## Part 3D — iced connector

### 3D.1 `CI/styles.rs`

Per status, today and after (the border's width and radius are `button.border`'s in
every status, unchanged):

| Style | `Active` | `Hovered` | `Pressed` | `Disabled` |
|---|---|---|---|---|
| `button` (`:198-251`) | `border.color` | `border.color` → `hover_border_color`, else `border.color` | same as `Hovered` | faded `border.color` (unchanged) |
| `button_toggle` (`:270-…`), off and on | `button`'s | `button`'s (through `..style`) | `button`'s | `button`'s |
| `class_button` (`:330-…`): `button_primary`, `button_danger`, `button_success`, `button_warning` | `border.color` | → `hover_border_color`, else `border.color` | same | faded (unchanged) |
| `button_link`, `segment`, `expander` | not `button.border`: unchanged | unchanged | unchanged | unchanged |

In `button`: after `let border = border_of(&b.border);` (`:223`), `let hovered_border =
Border { color: to_color(b.hover_border_color.unwrap_or(b.border.color)), ..border };`,
and the `Hovered` and `Pressed` arms (`:229-230`) take it. `class_button` the same (its
`border` binding at `:347`, and its `Hovered | Pressed` arm). The docs of `button` (`:181-196`) and
`class_button` gain: "Hovered and pressed, the border is `button.hover_border_color`
where the theme states one (a pressed button is under the pointer); an iced button
takes no keyboard focus (iced_widget 0.14.2 `src/button.rs:257-273`), so
`button.focus_border_color` has no receiver."

### 3D.2 Contract

- `CI/contract/rows.rs:338-351`, `native_button_border`: `Hovered | Pressed` give
  `hover_border_color.unwrap_or(border.color)`; its doc ("the button's own, in every
  status, faded when disabled") gains "and the hover border, where stated, hovered and
  pressed". It serves `BUTTON_BORDER_ROWS` (`:375-380`), `BUTTON_TOGGLE_BORDER_ROWS`
  (`:456-461`) and the four class buttons' (`CI/contract.rs:448-454`).
- `CI/contract/derived.rs`, `UNREAD_COLOURS` (`:1344-…`, decision 10): a new entry
  `UnreadColour { leaf: "button.focus_border_color", why: Unread::NoReceiver("an iced
  button takes no keyboard focus: it implements no `operate` for `Focusable`, and its
  status has no focused variant (`iced_widget-0.14.2/src/button.rs:257-273`,
  `:471-498`)") }`, right after the `input.caret_color` entry (`:1433-1440`) and before
  `combo_box.disabled_background` (`:1441-…`): the list keeps no one order (its
  `defaults.*` entries first, then input, combo box, slider, menu, link, splitter and
  window, the `NoWidget` and `Fallback` entries last, read today), and the caret's,
  another colour of a focused control that iced's `Style` has no field for, is the
  entry it fits beside.
- `button.hover_border_color` reaches the border rows, so the colour-leaf test
  (`CI/contract.rs:5578`, `every_colour_leaf_reaches_a_row_or_is_named`) finds it without an
  entry.

### 3D.3 Tests

`CI/styles.rs` has no test module: the iced button-style tests live in `CI/contract.rs`
(a `#[cfg(test)] mod contract;`, `CI/lib.rs:402-403`). So two new tests go there,
beside `a_cleared_soft_option_copies_the_base_state_value` (`CI/contract.rs:3711-3744`,
under the same `#[cfg(feature = "widgets")]`):
- `a_kde_breeze_button_hovers_and_presses_in_the_selection_border`: under kde-breeze
  (both modes) `styles::button`'s and `styles::button_primary`'s `Hovered` and `Pressed`
  border is #3daee9 and `Active` is `button.border.color`;
- `an_adwaita_button_keeps_its_border_in_every_status`: under adwaita `Active`,
  `Hovered` and `Pressed` are `border.color`, and `Disabled` its faded form, as today.

The contract's rows cover the 34 combinations.

### 3D.4 Showcase

No hand-written route. `iced_route` (`showcase-iced.rs:7749-…`) asks the contract's
export first (`colour_route`, called at `:7766-7773`, before its `match`), and the
export of unit 3.1 serves both leaves: `button.hover_border_color` as a leaf that moves
rows (the border rows of 3D.2 under `Hovered` and `Pressed`; the walk splices a
sentinel into the leaf in all 34 combinations, so it moves them where the preset
states none too), for which `colour_route` gives "iced: <row> (<statuses>); …" over
the statuses the element draws, or its "not drawn here: … in the state shown" line
where it draws neither (`:7593-7628`); and `button.focus_border_color` as a named
no-receiver leaf, for which it gives "not reachable: <the entry's why>" (`:7578-7591`),
the text of 3D.2's `UnreadColour`. So an arm in `iced_route`'s `match` would never be
reached, and none is added. The element list of 3B.4 is what makes the showcase ask;
`the_element_list_parses_and_every_leaf_has_a_route` (`showcase-iced.rs:24982`) holds
it.

## Part 3E — egui connector

### 3E.1 `CE/style/base.rs` and `CE/style/roles.rs`

egui keeps one `bg_stroke` per state, and a button pressed or with the keyboard focus is
`active` (`EG/style.rs:1273-1285`, `:1290-1320`). Two places write the two strokes
after their `write_states`, as the «input» role writes its own (`roles.rs:349-352`).

In `base_style`, where `b` is bound to `&t.button` and `w` to the base's
`&mut s.visuals.widgets` (`base.rs:325-326`), after that `write_states` call
(`:345-374`) and before the scroll bar's:

```rust
    // the hovered button's border, which a pressed one keeps, and the focused one's; egui's
    // `active` is press and keyboard focus alike (`Widgets::style`, egui 0.36.2). `None`
    // keeps `button.border.color`.
    let border = b.border.color;
    w.hovered.bg_stroke.color = to_color32(b.hover_border_color.unwrap_or(border));
    w.active.bg_stroke.color = to_color32(b.focus_border_color.or(b.hover_border_color).unwrap_or(border));
```

In `button` (`roles.rs:197-…`), which binds `b` (`:199`) but no `w` (it hands
`&mut s.visuals.widgets` to `write_states`, `:200-227`, and writes through `s`), the
code adds a binding of its own and then the same three statements: after its
`write_states` call (ends `:227`) and before the `s.visuals.selection` writes (the
comment at `:228`, then `:229-230`), first `let w = &mut s.visuals.widgets;` (the
call's borrow has ended there, and `w` is not used after the three statements, so
the `s.visuals.selection` writes that follow compile), then the three. The widths stay
`write_states`'.

**The roles that write no border keep their rest stroke** (decision 13). In
`roles.rs`, beside `role_cell`, a predicate, and in `role_cell` (`roles.rs:19-56`),
after the `match` and before `crate::style::derived::apply_role` (`:53`), one
statement that uses it:

```rust
/// The roles whose cells keep their resting stroke when hovered or pressed: they draw
/// their controls flat or as parts (a toolbar's or a list's buttons, a slider's
/// handle), so the button's hover and focus outline is not theirs (D5, D7). Each passes
/// `border: None` to `write_states`.
fn keeps_resting_stroke(role: Role) -> bool {
    matches!(
        role,
        Role::Scrollbar
            | Role::Slider
            | Role::Sidebar
            | Role::Toolbar
            | Role::StatusBar
            | Role::List
            | Role::Switch
    )
}
```

```rust
    if keeps_resting_stroke(role) {
        let w = &mut s.visuals.widgets;
        w.hovered.bg_stroke.color = w.inactive.bg_stroke.color;
        w.active.bg_stroke.color = w.inactive.bg_stroke.color;
    }
```

Step 5 changes the predicate, not the statement: it no longer holds for
`Role::Toolbar` where `toolbar.button_hover_border_color` is stated (decision 13).

Their `inactive` stroke is the base's (they write none), and today their hovered and
active strokes are the base's too, which `write_states` makes equal to `inactive`'s
(`states.rs:198-207`), so this gives each exactly today's colour; `derived::apply_role`
writes no hovered or active `bg_stroke` for any of them (`derived.rs`, read today:
corner radii, the splitter's `fg_stroke`, sizes). The widths stay as they are.

**Every role on kde-breeze, hovered and active `bg_stroke` colour, before and after**
(light / dark; the base's rest stroke is `button.border.color`, #d1d1d2 / #535659,
`kde-breeze.toml` `[light.button.border] color`, `[dark.button.border] color`):

| Role cells | Before | After |
|---|---|---|
| base, «button» | #d1d1d2 / #535659 | #3daee9 / #3daee9 |
| «scrollbar», «slider», «sidebar», «toolbar», «status bar», «list», «switch» | #d1d1d2 / #535659 (the base's) | the same (kept) |
| «input», «menu», «checkbox», «tab», «combo box», «segmented control», «expander» | their own border's (a `BorderSource` writes all four entries, `roles.rs:340`, `:285`, `:395`, `:618`, `:924`, `:997`, `:1062`; the input its own hover stroke, `:349-352`) | unchanged |
| «window», «card», «splitter», «tooltip», «popover», «dialog», «progress bar», «separator», «spinner», «link» | the base's, #d1d1d2 / #535659 (they write no state) | the base's, #3daee9: a button drawn in such a scope (a dialog's) is a button |

On every other preset nothing changes: the two fields are unset there, and the base
writes `button.border.color` as today.

**Consequence of the base**: the base's interactive entries are what every control
outside the connector's roles draws with (`base.rs:345`'s comment). On kde-breeze such
a control takes the highlight as its outline under the pointer; the controls read for
this, which are examples and not the whole list: an unscoped check box, radio, combo box
button and drag value, hovered and pressed, a selectable label, hovered and pressed, and
a text field, hovered (egui draws their frames from the state's `bg_stroke`:
`EG/widgets/radio_button.rs:81`, `:95`; a check box's frame, `EG/widget_style.rs:184`,
drawn at `EG/widgets/checkbox.rs:138`; `EG/containers/combo_box.rs:452-461`; a drag value
is a `Button`, `EG/widgets/drag_value.rs:636`; a selectable label is a `Button`,
`EG/ui.rs:1929-1931`, an unselected one framed in every state but rest, `EG/widgets/button.rs:78-81`,
`:364`, with the state's `bg_stroke`, `EG/widget_style.rs:158-161`; a text field not
focused, `EG/widgets/text_edit/builder.rs:737`, `:749`, a focused one taking
`selection.stroke`, `:742-747`). A slider's handle does not change: egui strokes it with
the state's `fg_stroke` (`EG/widgets/slider.rs:817`, `:831`). Breeze's check box and
combo box show hover by an outline in the focus or highlight colour
(`docs/todo.md:1774-1795`), so on KDE this moves them toward Breeze.

### 3E.2 `mapping.toml` and `mapping.md`

After `["button.checked_text_color"]` (`:996-…`):

```toml
["button.hover_border_color"]
verdict = "direct"
sinks   = [
  { path = "visuals.widgets.hovered.bg_stroke.color", when = "stated; None keeps button.border.color" },
  { path = "visuals.widgets.hovered.bg_stroke.color", scope = "button", when = "stated; None keeps button.border.color" },
  { path = "visuals.widgets.active.bg_stroke.color", when = "button.focus_border_color is None" },
  { path = "visuals.widgets.active.bg_stroke.color", scope = "button", when = "button.focus_border_color is None" },
]

["button.focus_border_color"]
verdict = "direct"
sinks   = [
  { path = "visuals.widgets.active.bg_stroke.color", when = "stated; None takes button.hover_border_color, else button.border.color" },
  { path = "visuals.widgets.active.bg_stroke.color", scope = "button", when = "stated; None takes button.hover_border_color, else button.border.color" },
]
```

**`["button.border.color"]`** (`mapping.toml:1037-1049`) is brought in line:

- its four hovered and active sinks, base and «button» (`:1041-1042`, `:1046-1047`),
  gain `when = "button.hover_border_color is None"` (hovered) and `when =
  "button.focus_border_color and button.hover_border_color are None"` (active): the
  base and «button» now write those colours from the new fields where they are stated;
- it gains 14 sinks, `{ path = "visuals.widgets.hovered.bg_stroke.color", scope = "<r>" }`
  and the same for `active`, for `<r>` each of `scrollbar`, `slider`, `sidebar`,
  `toolbar`, `status_bar`, `list` and `switch`, with no `when`: those cells now write the
  two colours themselves, from their resting stroke, which is `button.border.color` in
  every preset. (Step 5 gives the two «toolbar» ones a `when`, decision 13.)

What the mapping tests need for that: T2 reads, for each location built from another
(a role cell from the base), whether the cell writes the field itself or carries the
base's (`own_write`, `cell_own`, `carried_by`, `mapping_tests.rs:1028-1137`). A cell
with no declared sink at a path carries every base sink there, and a base sink without
`when` must then move it. The base's new hovered and active sinks have a `when` (the
table above), so a cell that carried them would not be required to move with them
(`carried_by` sets `required = s.when.is_none()`, `mapping_tests.rs:1108`): the 14
scoped sinks are not there to lift that. They are needed by T10c (below): on
kde-breeze the seven cells' hovered and active strokes differ from the base's, and a
field that differs from what its location inherits must be named by a sink there. With
them, each cell is a location whose own value is stated (`Own::Overrides`,
`mapping_tests.rs:1030-1040`, `:1058`), so the base's sinks are not carried there, and
`button.border.color`'s splice is required to move them, which it does through their
resting stroke. `no_row_write`
(`mapping_tests.rs:1064-1077`) is not used: the write has a row. Read, not run.

Regenerated `mapping.md` totals, computed from step 2's: Buttons 113 → 115 leaves,
DIRECT 11 → 13; TOTAL 517 → 519, DIRECT 46 → 48. `mapping_tests.rs:236-241`: `517` →
`519`, "513 ResolvedTheme leaves" → "515". `verdict_totals_match_the_specification`
(`mapping_tests.rs:443-497`): `("buttons", [11, 46, 31, 25])` → `("buttons", [13, 46, 31, 25])`
(after step 2's foundation change); the sub-tag counts do not change. The search of
2G.2 finds no other hard-coded count or list these rows change.

### 3E.3 Tests

New in `CE/style/roles.rs`' tests (beside `:1242`, which reads the «button» cell):
kde-breeze, both modes: the «button» cell's and the base's `hovered.bg_stroke.color` and
`active.bg_stroke.color` are #3daee9, `inactive`'s is `button.border.color`; adwaita:
all three are `button.border.color`; a theme with only `hover_border_color` stated has
it in `active` too.

New, `flat_and_part_scopes_keep_their_resting_stroke`: for every bundled preset in both
modes (kde-breeze states both fields), each role of `Role::all()` for which
`keeps_resting_stroke` holds has `hovered.bg_stroke.color` and `active.bg_stroke.color`
equal to its `inactive.bg_stroke.color`, which equals the base's
`inactive.bg_stroke.color` (`button.border.color`): today's. It also asserts that the
predicate holds for exactly the seven roles of decision 13, so a change of the list is
a change of the test (step 5 narrows both).

Changed, `hover_and_active_keep_the_resting_stroke_and_radius` (`CE/style/states.rs:301-330`):
it runs `base()` under adwaita light and kde-breeze dark and asserts the base's
hovered, active and open `bg_stroke` equal `inactive`'s. It keeps the width and radius
assertions for every entry and the colour assertion for `open`; for `hovered` and
`active` it compares the colour only where `button.hover_border_color` (and, for
`active`, `button.focus_border_color`) is unset, and under kde-breeze asserts the
stated colour instead. Its doc gains "the button's hover and focus border, where
stated, colour `hovered` and `active` (3E.1)".

**Further tests that read a hovered or active stroke**, found by `grep` for
`bg_stroke`, `hovered`, `entries(` and `all5(`, and for loops over the states
(`for e in [&w.hovered, …]`, `[w.noninteractive, w.inactive, w.hovered, …]`), over the
egui connector's `src/` and `tests/`, egui-widgets' `src/` and the egui showcase, each
read. A test goes red where it reads the base's or the «button» cell's hovered or
active stroke under kde-breeze, the only preset that states the fields, or a cell of
the seven roles against the base's hovered or active stroke:

- `no_border_colour_carries_the_border_opacity` (`CE/style/derived.rs:1054-1120`) builds
  under kde-breeze light (`theme()`, `:395-397`) and asserts, through `entries`
  (`:427-429`), that all five entries of the «button» cell have stroke colour
  `button.border.color` (`:1075-1077`). **Red**; changed: the loop zips the five entries
  with the colours 3E.1 writes, `[border, border, hover, focus, border]` for
  `noninteractive`, `inactive`, `hovered`, `active`, `open`, where `border` is
  `button.border.color`, `hover` is `hover_border_color.unwrap_or(border)` and `focus`
  is `focus_border_color.or(hover_border_color).unwrap_or(border)` (on kde-breeze
  light #d1d1d2, #d1d1d2, #3daee9, #3daee9, #d1d1d2), each converted with
  `to_color32` and none multiplied by the opacity; the doc's "every widget's own border"
  gains "and the button's hover and focus border".
- `panel_role_cells_write_their_rows` (`roles.rs:1900-…`, «sidebar», «toolbar», «status
  bar», over `PRESETS`, which holds kde-breeze dark, `:1298-1305`) asserts each cell's
  `inactive`, `hovered`, `active` and `open` stroke equals the base's same entry
  (`:1946-1957`, "no outline on the panel's buttons"). **Red**; changed: `hovered` and
  `active` are compared with the base's `inactive` stroke, which keeps the test's
  claim.
- `switch_cell_writes_its_rows` (`roles.rs:2109-…`, over `PRESETS`) asserts all five
  entries' stroke equals the base's same entry (`:2140-2150`). **Red**; the same change
  for `hovered` and `active`.
- `button_cell_writes_its_rows` (`roles.rs:1310-…`) asserts all five entries' stroke
  colour is `button.border.color` (`:1343-1351`), but runs only adwaita light and
  windows-11 dark (`:1311-1314`), which state neither field: green as it is. Changed
  all the same, so that it holds the new formula: it expects for `hovered` and `active`
  the colours 3E.1 writes, and gains kde-breeze dark.
- Read and unchanged: `tooltip_and_popover_cells_write_their_rows` (`:1646-…`, the cells
  equal the base's entries, which they still carry); `strong_text_is_the_panel_text_…`
  (`:1241`, text only); the menu, input, checkbox, tab, combo box, segmented control and
  expander cell tests (`:1406`, `:1501`, `:1557`, `:1813`, `:2186`, `:2320`; each
  role's own border); `the_checkbox_selected_cell_fills_the_box_and_outlines_it` and
  the selected-disabled checkbox test (`CE/style/variants.rs:318-347`; the checkbox's
  own outline); the disabled button cell test (`variants.rs:421-424`, the disabled cell
  against the normal one, which carry the same strokes);
  `hover_and_active_keep_the_resting_stroke_and_radius`'s neighbours in `states.rs`
  (`:343-…` under adwaita, and `:365-376`, which calls `write_states` directly);
  `d2_the_knob_is_the_diameter_where_the_row_does_not_bite` (`derived.rs:540`, the
  expansion only); egui-widgets' `a_widget_in_the_body_keeps_the_callers_style`
  (`tests.rs:782-…`, the base's `inactive` stroke), `an_expander_with_no_stated_frame_outlines_nothing`
  (`:739-…`, macos-tahoe and catppuccin-mocha, unset), `a_hovered_segment_lays_the_hover_over_the_group`
  (`:2317`, the segment's own fill); `CE/install_tests.rs`' input-focus tests
  (`:2114-2172`, the «input» cell's own strokes). The egui showcase's tests read no
  `bg_stroke` (`grep`). The iced and gpui tests of the border per status are the
  contract rows of 3D.2 and the trial of 3F.

The mapping tests (T2's splices) prove the sinks; T10 proves the values (3E.4).

### 3E.4 `CE/contract.rs`: T10's formulas

T10 (`t10_every_sink_holds_the_leafs_value`, `CE/contract.rs:1073-1139`) holds every
declared sink to `expected` (`:530-…`). Where no arm matches, `expected` accepts a
location only if exactly one row declares it and that row is not `derived`, and gives
that row's conversion (`:1039-1063`); otherwise it fails with "no formula for this
location". After 3E.2, `visuals.widgets.hovered.bg_stroke.color` in the base and in
«button» is declared by two rows (`button.border.color`, `button.hover_border_color`)
and `active.bg_stroke.color` by three (with `button.focus_border_color`); the only
`bg_stroke.color` arms today are the checkbox's and the input's (`:714-724`). So
`expected` gains two arms, after the input's, under its comment "border colours with
their §6.4 fallbacks, as stated":

```rust
        (None | Some("button"), "normal", None, "visuals.widgets.hovered.bg_stroke.color") => {
            c32(chain(c, &["button.hover_border_color", "button.border.color"])?)
        }
        (None | Some("button"), "normal", None, "visuals.widgets.active.bg_stroke.color") => {
            c32(chain(
                c,
                &[
                    "button.focus_border_color",
                    "button.hover_border_color",
                    "button.border.color",
                ],
            )?)
        }
```

(`chain` takes the first stated leaf, `:514-519`; a base sink has no variant, which
`expected` reads as "normal", `:539`.) Every row that declares the two locations is
held to the one formula, as `expected`'s doc requires (`:521-529`).

The other contract tests, read against the 14 scoped sinks and the base's new strokes:

- T10 for the 14 scoped sinks: each location (a cell of the seven roles, hovered or
  active) is declared by `button.border.color` alone, a `direct` row, so the fallback
  gives `to_color32(button.border.color)`, which the cell holds (its `inactive`
  stroke, the base's, which T10 already holds to that value through the row's base
  `inactive` sink). No arm. (Step 5 adds one for the toolbar's, decision 13.)
- T10b (`t10b_every_style_path_is_a_sink_or_unwritten`, `:1272-1318`) reads paths, not
  locations; both paths are sinks today. Unchanged.
- T10c (`t10c_every_undeclared_field_holds_its_inherited_value`, `:1342-1420`): a field
  that moves from what its location inherits must be named by a sink there. The base
  against egui's own style: the two strokes are named. The seven cells against the
  base: on kde-breeze their hovered and active strokes now differ from the base's,
  and the 14 sinks name them. «button» against the base: equal. A variant cell is
  built from its role's `Normal` cell (`CE/style/mod.rs:79-100`) and carries the
  strokes, so nothing moves there. Holds.
- T10's other tests (`:1148`, `:1206`, `:1427`) read the expander's padding, a
  checkbox's inner width and `NaN`s; T12's pairs (`:1551-…`) name no `bg_stroke`
  (`grep`). Unchanged.

Read, not run.

## Part 3F — gpui connector: the trial

### 3F.1 What is built

In `CG/variants.rs`, after `switch_color` (ends before `#[cfg(test)]` at `:95`; the
claimed lines 62–64 do not move), a candidate function:

```rust
/// A gpui-component `Button` with the platform's hover and press border
/// (`button.hover_border_color`), which the stock `Default` variant cannot take: it draws
/// the `input` token there (gpui-component 0.9.1 button/button.rs:1084-1086). `None`
/// where the theme states no hover border; the caller then keeps the `Default` variant.
#[must_use]
pub fn bordered_button(n: Native<'_>, cx: &App) -> Option<ButtonCustomVariant>
```

which has one form, the trial's and the kept one alike: `hover_border(n).map(|c| …)`,
with `hover_border` the pure function of 3F.4, after `bordered_button` in the same file
(`pub(crate) fn hover_border(n: Native<'_>) -> Option<Hsla>`,
`n.resolved.button.hover_border_color.map(rgba_to_hsla)`), so that where the field is
`Some`, it returns `ButtonCustomVariant::new(cx)` with `.color(c)` (the border in every
state; `GC/button/button.rs:1118-1124`), `.foreground(theme.button_foreground)`,
`.hover(theme.button_hover)`, `.active(theme.button_active)` (the installed tokens the
`Default` variant paints, `CG/colors.rs:380-382`), `.shadow(false)`. The caller builds
the button as the connector's buttons are built, with the rest fill the variant cannot
give (its own rest fill is a fifth of `color`, `GC/button/button.rs:1028`) on an
unselected button only:

- rest: `.custom(v)`, `.refine_style(&geometry::button(n))` and `.bg(cx.theme().button)`;
- selected: `.custom(v)`, `.refine_style(&geometry::button(n))` and
  `geometry::button_checked(n)` (`CG/geometry.rs:1369`), with no `.bg`. Upstream fills a
  selected custom variant with its `active`, here `button_active`
  (`GC/button/button.rs:1337`), as it fills a selected `Default` button with
  `tokens.button_active` (`:1327`), and replays the caller's style over that state
  (`:821-829`); `button_checked` sets a fill only where the theme states
  `checked_background` or `active_background` (`CG/geometry.rs:1374-1377`), so a rest
  fill would stand on a selected button of a theme that states neither (macos-tahoe,
  both modes: `macos-tahoe.toml:113-129`, `:526-542` at `0fa28027`, and neither field
  inherits, `docs/inheritance-rules.toml:409`, `:411`);
- disabled: the rest's three plus `geometry::button_disabled(n)` (`CG/geometry.rs:1344`),
  whose fill replaces the rest fill (`:1347-1349`, set in every theme).

### 3F.2 What is compared, how

A `#[gpui::test]` in `CG/widgets/tests.rs`, beside the connector's other widget tests,
`bordered_button_matches_the_default_button_but_its_hover_border`, drawing two buttons
with the same label side by side: the `Default` variant built as the showcase builds
its default button, and `crate::variants::bordered_button`'s (a `pub fn`). For each
state, the button's own quad of the last frame (`scene_quads`,
`CG/widgets/scene_read.rs:22`, the quad whose bounds are the button's `debug_bounds`)
gives fill, border colour, border widths and corner radii. `CG/widgets/tests.rs`
already includes `scene_read.rs` (`#[path = "scene_read.rs"] mod scene_read;`,
`:23-25`) and draws under gpui's test context, so the test needs no `#[path]` of its
own. It is not put in `variants.rs`' inline test module: there a `#[path =
"widgets/scene_read.rs"]` would resolve against `src/variants/tests/`, the path rule
for a module declared inside an inline module of a non-`mod.rs` file. This
specification gives no other `#[path]`. **Equal** means: widths and radii exactly; each colour converted to 8-bit sRGBA (the
precision the frame is drawn in) and compared channel by channel, exactly. The `Default`
variant paints `cx.theme().tokens.*` values (`GC/button/button.rs:1014-1030`,
`:1158-1240`) where the custom variant paints the `ThemeColor` values `bordered_button`
copies; whether the two are bit-equal as `Hsla` was not read, and a difference that
vanishes at 8 bits is recorded in the trial's record as such. Compared field by field:

| State | How it is reached | Expected |
|---|---|---|
| rest | drawn | equal |
| hovered | `simulate_mouse_move` onto the button | fill equal; border: the variant `hover_border_color`, `Default` the `input` token |
| pressed | mouse down on the button, frame read before the up | as hovered |
| selected | `.selected(true)` with `button_checked`, the variant's without `.bg` (3F.1) | equal |
| disabled | `.disabled(true)` with `button_disabled` | equal |
| focused | a key press, then `focus_next` to the button | equal (both tint the border in `ring`: with the band off, kde-breeze and windows-11, `GC/styled.rs:268-270`; with it on, macos-tahoe, which states no `focus_ring_keyboard_only` and a `focus_ring_width` of 3, `macos-tahoe.toml:71`, `:484`, the border and the band, `GC/styled.rs:257-262`) |

Text colour: not readable from a frame (correction 2). The test asserts by value that
the variant's `foreground` equals `button_foreground` (`ButtonCustomVariant: PartialEq`,
`GC/button/button.rs:38-39`, as `ghost_button`'s test compares, `CG/variants.rs:142-…`)
and records, from `GC/button/button.rs:1032-1081`, `:1223-1228`, `:1299-1303`, that both
variants letter every enabled state in that one colour and a disabled one in
`muted_foreground` at half alpha (`:1369`), where `button_disabled`'s text colour then
wins for both.

Presets: kde-breeze light and dark (the only preset that states the field), and
windows-11 and macos-tahoe, light and dark, with `hover_border_color` set on the
resolved theme in the test (#ff0000, a colour neither preset's file holds, `grep` today;
the window is set up with 2E.4's `window_with_resolved`, from step 2's tree), so a
translucent hover fill (windows-11's, `PF:1192`) and a pressed text colour of its own
(`PF:1197`) are exercised too, and, on macos-tahoe, which states neither
`checked_background` nor `active_background` (3F.1), a selected button whose fill is
upstream's selected fill, not `button_checked`'s. The test runs in unit 3.0, on the
worktree with the fields (3A.1, 3A.2's two full literals) and kde-breeze's lines (3B.5)
applied and not committed (3I), so it reads the field through the code 3F.1 keeps.

### 3F.3 The pass criterion

The trial passes when, on all six combinations, every compared item of every state is
equal, but the hover and press border (the variant's `hover_border_color` where the
`Default` button has `input`), and the text-colour assertion holds. Any other
difference fails it (decision 12), and the trial's record names the state that differed.

### 3F.4 If it passes

- `bordered_button` stays, as written in 3F.1, with a doc example in the form of
  `ghost_button`'s (`CG/variants.rs:40-49`) showing the three states' refinements; the
  trial's test stays where 3F.2 puts it, in `CG/widgets/tests.rs`, as the test that
  holds `bordered_button` to the `Default` button.
- **Contract.** The colour-leaf test's sinks (`CG/contract.rs:2736-…`) take no `App`,
  and a `ButtonCustomVariant` is built only with one (`ButtonCustomVariant::new(cx)`,
  `GC/button/button.rs:103-111`); none of the table's three kinds, `NoReceiver`,
  `Inherited` and `Fallback` (`CG/contract.rs:1044-1064`), describes a value that does
  reach a receiver. That is why `bordered_button` reads its colour through 3F.1's
  `hover_border`, a pure function of the same file, crate-visible since
  `CG/contract.rs` calls it, placed after `bordered_button` (*Lines that must not
  move*); where the trial fails, both go (3F.5). It returns a value, not a `StyleRefinement`, so `sinks` takes it among the
  values it records by their `Debug` text, beside `dialog_entrance` (the list at
  `CG/contract.rs:2825-2838`), not among the refinements (`:2780-2823`, builders that
  return a `StyleRefinement`). That list's loop names each entry `geometry::<name>`
  (`:2837`), so the line goes after the loop, with its own name:
  `out.insert("variants::hover_border".to_owned(), show(&crate::variants::hover_border(n)));`
  (`show`, `:2753`; `n`, the `Native` the refinements take). The leaf then reaches a
  sink and needs no entry.
  `button.focus_border_color`: an `UnreadColour` with `Unread::NoReceiver("with
  gpui-component's band off, a focused Button's border is `ring` where it has a border
  width, and where it has none a 1 px line: inside it in its text colour at 60 %, but
  just outside it in `ring` on a link or a text Button and on its edge in `ring` on an
  upstream ghost Button; with the band on, its border is
  `ring` and the band is drawn round it (styled.rs:257-297, focus_style;
  button/button.rs:644-650, :920-922). The model's one focus colour reaches `ring`
  through the `ring` row")`. What the text rests on, in gpui-component 0.9.1: every
  focused Button whose own focus-ring switch is on gets `focus_style`
  (`GC/button/button.rs:920-922`, `is_focused && self.focus_ring_enabled`; the switch is
  `FocusableExt::focus_ring`, `:554-556`, on by default, `:275`, and none of the
  showcase's buttons turns it off, `grep` today); with the band on it
  sets the border to `ring` and draws the band (`GC/styled.rs:257-262`); with it off it
  sets the border to `ring` where a border width is above 0 (`:269-270`), and otherwise
  draws the line its `FocusLine` gives (`:273-297`; inset, on the edge or outside it,
  `:274-278`): `Inside`, the text colour at
  `FOCUS_LINE_OPACITY` 0.6 (`GC/button/button.rs:21`), for every Button but a link or a
  text (`Outside`, `ring`) and upstream's `.ghost()` (`Edge`, `ring`; the connector's
  `ghost_button` is a custom variant and takes `Inside`) (`:644-650`). A Button refined
  by `geometry::button` has a border width where `button.border.line_width` is above 0
  (`CG/geometry.rs:229`); material-3 and adwaita state 0 (`material-3.toml:136`,
  `:655`; `adwaita.toml:138`, `:575`), so there a focused button shows the line, or, on
  adwaita, where step 2 leaves the band on (2E.1), the band.
- **Showcase: the three buttons.** The Basic page's default, tooltip and toggle-off
  buttons (the elements of 3B.4 but primary and toggle-on; `basic-button`,
  `basic-button-tooltip`, `basic-toggle-off`) are built through `bordered_button` where
  it returns `Some`, with the rest refinements of 3F.1, and are the `Default` Button
  elsewhere. `basic-toggle-on` stays the `Default` Button in gpui, and
  `demo::toggle_button` (`demo.rs:9166-9202`) is not changed: it is built
  `.selected(true)` (`:9193-9199`), and gpui-component registers the press style only
  for a Button that is neither disabled nor selected (`GC/button/button.rs:730-750`)
  and returns from its hover closure unchanged for a selected one (`:755-757`), so a
  selected Button changes nothing under the pointer and a hover border would never be
  drawn on it. Its info's claims of a hover and press look are wrong today already
  (*Problems found*, 4; 3H.4).
  The three's info then has to be the variant's: today it is built as `ButtonKind::Default`
  (`pages/basic.rs:58`, `:81`), whose rest
  fill (`gpui-component/button/button.rs:1020`), hover and press fills (`:1164`,
  `:1248`), text (`:1034`) and variant line (`info/buttons.rs:561`) would claim upstream's
  `Default` reads while the button paints the caller's `.bg(cx.theme().button)` and the
  variant's `colors.hover`, `colors.active` (`:1209`, `:1294`) and `colors.foreground`
  (`:1080`), and `every_colour_claim_is_read_at_the_line_it_cites` would not see it,
  since the cited lines name the same fields. (`DISABLED_BUTTON`, `pages/basic.rs:70-75`,
  is not hovered and stays `Default`, 3B.4; so does every other button of the
  showcase.) So the showcase gets a kind of its own for it, as `Ghost` is
  `ghost_button`'s (`demo.rs:3262-3263`), all in unit 3.1:
  - **`demo::ButtonKind`** (`demo.rs:3254-3270`) gains, after `Ghost`,
    `/// native_theme_gpui::variants::bordered_button, filled with ThemeColor button at
    rest; built only where that returns Some (ButtonKind::built), and never selected:
    a selected Button takes no rest fill.` `Bordered`. `name`
    (`:3273-3288`) gives it "Bordered". A private helper beside the `impl`,
    `fn bordered(cx: &App) -> Option<ButtonCustomVariant>`, is
    `cx.native().and_then(|n| variants::bordered_button(n, cx))` (both borrows shared,
    2E.2). `apply` (`:3290-3306`) gains `Self::Bordered => match bordered(cx) { Some(v)
    => button.custom(v).bg(cx.theme().button), None => button }`, 3F.1's rest fill: the
    two builders that use `Bordered` below, `demo::button` and
    `demo::built_tooltip_button`, hand `apply` a `Button::new(id)` and select it nowhere
    (`demo.rs:3330-3397`, `:6100-6144`, read today), and `basic-toggle-on`, the one
    selected button of 3B.4's elements, stays `Default` (above), so no `Bordered` Button
    is selected; a later builder that selects one leaves the fill out, as 3F.1 says.
    The fill goes on before `native_info` refines the Button with `geometry::button`, which sets no fill
    (`CG/geometry.rs:219-243`), so the order of 3F.1 gives the same style. A new
    `pub(crate) fn built(self, cx: &App) -> Self` returns `Default` for `Bordered` where
    `bordered(cx)` is `None` and `self` otherwise.
  - **How the info learns the kind.** Every builder of the three asks `built` once and
    hands the one answer to both the info and `apply`, so the two cannot disagree:
    `demo::button` (`demo.rs:3330-3397`) starts with `let kind = kind.built(cx);` after
    its destructuring (`:3331-3337`), before `info::buttons::button` (`:3345-3352`) and
    `kind.apply` (`:3365`). `demo::built_tooltip_button` (`:6100-6144`, five parameters
    today) has two callers, the Basic page's tooltip button (`pages/basic.rs:442-448`)
    and the Feedback page's built tooltip (`pages/feedback.rs:465-473`), so it gains a
    sixth parameter, `kind: ButtonKind`, after `text`: it starts with `let kind =
    kind.built(cx);`, hands `Some(kind)` to `info::feedback::tooltip` (below) in place of
    its `true` (`:6112`), and builds its Button as `kind.apply(Button::new(id), cx)` in
    place of `Button::new(id)` (`:6120`); its doc's "A Default Button reading `label`"
    (`:6090`) becomes "A Button of `kind` (as `ButtonKind::built` gives it) reading
    `label`". The Basic page passes `ButtonKind::Bordered`, the Feedback page
    `ButtonKind::Default`, so the Feedback page's trigger stays the `Default` Button and
    "every other button of the showcase" above stays true. `pages/basic.rs`'
    `BUTTONS[0]` (`:55-60`) and `TOGGLE_OFF` (`:78-83`) name `ButtonKind::Bordered`.
    `edge` and the other functions of `info/buttons.rs` then get the kind the Button was
    built as.
  - **The claims, `info/buttons.rs`.** `<F>`, `<H>`, `<A>` are the lines of
    `.foreground(theme.button_foreground)`, `.hover(theme.button_hover)` and
    `.active(theme.button_active)` in `bordered_button`, and `<R>` the line of
    `hover_border` that reads `n.resolved.button.hover_border_color`, in the tree the
    implementer edits (*Lines that must not move*):
    - `at_rest` (`:17-159`): `ButtonKind::Bordered => vec![claim("bg", "button",
      t.button, "showcase"), claim("text", "button_foreground", t.button_foreground,
      "native-theme-gpui/variants.rs:<F>")]`. The fill is cited `"showcase"`: the
      showcase chooses it, and a line of its own would move with every edit above it,
      so the test asks only that the showcase's code mention the field
      (`citation_finding`, `CG/showcase.rs:2732-2746`); `.bg(cx.theme().button)` does.
    - `under_the_pointer` (`:162-333`): `ButtonKind::Bordered => vec![claim("hover",
      "button_hover", t.button_hover, "native-theme-gpui/variants.rs:<H>"),
      claim("active", "button_active", t.button_active,
      "native-theme-gpui/variants.rs:<A>")]`, as `Ghost`'s cite `variants.rs:63-64`.
    - `edge` (`:339-380`): in `hovered`'s `match`, before the `Default` arms,
      `(ButtonKind::Bordered, _) => native.and_then(|b| edge_claim("border, hovered or
      pressed", "hover_border_color", b.hover_border_color, Some(b.border.line_width),
      "native-theme-gpui/variants.rs:<R>"))`; the rest claim stays the shared
      `geometry.rs:230` one. `edge_claim`, so `every_edge_colour_is_claimed_through_edge_claim`
      (`CG/showcase.rs:2990`) holds and no edge is claimed at width 0. `<R>` and not the
      line of `.color(…)` in `bordered_button`: `every_colour_claim_is_read_at_the_line_it_cites`
      (`CG/showcase.rs:2819`) requires the cited span to contain the claimed field as
      an identifier (`mentions`, `CG/showcase.rs:1028-1049`, called at `:2804`), which
      `.color(rgba_to_hsla(c))` does not.
    - `button`'s notes: the idle edge note's `match kind` (`:534-543`) gains
      `ButtonKind::Bordered => "button.hover_border_color, the colour variants::bordered_button gives its custom variant (native-theme-gpui/variants.rs, bordered_button)"`;
      the shadow note (`:550-554`, "none on a standard variant: …", which a custom
      variant is not) becomes a `match kind`: `Ghost` keeps no note, every kind but
      `Ghost` and `Bordered` keeps today's, and `Bordered` takes `info.not_themeable("shadow", "none: variants::bordered_button builds its custom variant with .shadow(false), and upstream's Default variant has none (native-theme-gpui/variants.rs, bordered_button; button/button.rs, ButtonVariant::shadow)")`;
      the variant line's `match kind` (`:560-578`) gains
      `ButtonKind::Bordered => info.instance("variant", "native_theme_gpui::variants::bordered_button: a custom variant with the Default Button's text, hover and press colours (button_foreground, button_hover, button_active) and button.hover_border_color as its border, which upstream's Default variant draws in input (button/button.rs, ButtonVariant::border_color). A custom variant fills itself at rest with its border colour mixed toward transparent (button/button.rs, ButtonVariant::bg_color), so the showcase gives the Button ThemeColor button, the Default Button's rest fill. Built only where the theme states a hover border; elsewhere the Button is the Default variant")`.
      The not-styled disabled note (`:516`), `disabled_fill` and `loading` keep their
      `_` arms: a `Bordered` Button is always styled (it needs a native theme) and
      the page disables and loads none.
  - **The tooltip's trigger.** `info::feedback::tooltip` (`info/feedback.rs:855-909`,
    seven parameters: `t`, `built: bool`, `styled`, `delay`, `defaults`, `label`,
    `text`, `:855-863`) says "a Default Button reading {label} … Its own colours are a
    Default Button's, which the Buttons page's Default Button states" (`:904-907`). It
    takes `built: Option<ButtonKind>` in place of `built: bool`, so it learns the
    trigger's kind with no eighth parameter: `None` is a tooltip given by
    `Button::tooltip`, on a `Default` trigger; `Some(kind)` a tooltip the application
    built, on a trigger of that kind. Seven parameters stay within clippy's
    `too_many_arguments` default of seven, which `-D warnings` would make an error
    (`scripts/check_release.sh:529-530`, `:546-552`; `connectors/native-theme-gpui/clippy.toml`
    sets no other threshold). Its uses of `built` (`:865`, `:874`, `:879`, `:884`, `:889`)
    read `built.is_some()` (the `match (built, delay)` of `:884` as `match
    (built.is_some(), delay)`, its arms unchanged); `ButtonKind` joins the
    `crate::demo` import of `:13`; its doc's "on a Default Button reading `label`"
    (`:846`) becomes "on a Button reading `label`", and "`built` is whether the application built the
    tooltip itself" (`:848-849`) becomes "`built` is the kind of the trigger where the
    application built the tooltip itself, and `None` where `Button::tooltip` gave it";
    the trigger line (`:904-907`) matches `built`: `Some(ButtonKind::Bordered)` reads "a
    Button reading {label}, built with native_theme_gpui::variants::bordered_button,
    which reports through its tooltip. Its own colours are the Basic page's Button's,
    which that Button states", and every other value, `None` and
    `Some(ButtonKind::Default)` (the only other ones a caller passes), keeps today's
    text. The two callers: `demo::tooltip_button` (`demo.rs:6064-6072`) passes `None`
    for its `false` (`:6066`), `built_tooltip_button` `Some(kind)` (above).
  - **The test.** `a_refined_buttons_info_claims_an_edge_only_where_one_is_drawn`
    (`showcase-gpui/tests.rs:15732-15787`) gains `ButtonKind::Bordered` in its list of
    kinds, skipped (`continue`) where `r.button.hover_border_color` is `None`, the
    combinations where it is never built; where it is drawn, the hovered claim is also
    asserted equal to `Some(("hover_border_color", stated(c)))` for the stated `c`.
    `a_disabled_buttons_info_prints_its_fade` keeps `Default` and `Primary`.
- **Showcase: the routes.** `info/leaves.rs`: `HOW` entries for
  `basic.buttons.default`, `.tooltip`, `.toggle_off` and the leaf
  `button.hover_border_color` ("gpui: variants::bordered_button's colour, the border
  hovered and pressed, where the theme states one; at rest, selected and disabled the
  border is geometry::button's button.border.color, which upstream replays over those
  states (gpui-component/button/button.rs, RenderOnce for Button); else the Button is
  upstream's Default (demo::ButtonKind)"); for `basic.buttons.toggle_on` and that leaf,
  under the section's comment "button: the toggle button that is on"
  (`info/leaves.rs:16`), the not-drawn line "not reachable: a selected Button takes no
  hover or press style, so the toggle that is on draws no hover border
  (gpui-component/button/button.rs, RenderOnce for Button)", in the existing exempt
  form: the route listing skips a line that says "not reachable"
  (`CG/showcase.rs:4109`), and `RenderOnce` is in that one file of
  gpui-component (`prose_citations`, `:3222-3256`, reads the symbol up to the space;
  `GC/button/button.rs:620`); and a generic
  one for `button.focus_border_color` ("not reachable: with gpui-component's band off, a
  focused Button's border is ThemeColor ring where it has a border width, and where it
  has none a 1 px line: inside it in its text colour at 60 %, but just outside it in
  ring on a link or a text Button and on its edge in ring on an upstream ghost Button;
  with the band on, its border is ring and the band is drawn round it (styled.rs,
  focus_style)"); `basic.buttons.primary`'s
  `button.hover_border_color`: "not reachable: a Primary Button's hover and press border
  is the primary token (gpui-component/button/button.rs:1088, primary); at rest it is
  geometry::button's button.border.color (native-theme-gpui/geometry.rs:230,
  border_color)", and its `button.focus_border_color` takes the generic entry, which
  holds for it as for every Button. Why the Primary line: a Primary Button's rest,
  selected and disabled border is the caller's `geometry::button` `border_color`
  (`CG/geometry.rs:230`), which upstream replays over each state
  (`GC/button/button.rs:634`, `:764`, `:828`, `:836`); only its hover and press styles
  set the `primary` token (`:738-742`, `:758-760`, from `ButtonVariant::border_color`,
  `:1088`), and gpui applies a hover style after the refinement. A `HOW` line's line
  citation names the symbol its lines hold, in the form of the existing entries
  (`info/leaves.rs:97`), or `every_prose_citation_still_exists` fails it
  (`CG/showcase.rs:3762-3771`; its parser, `line_citations`, `:3273-3387`): `:1088` is
  `Self::Primary => cx.theme().primary,` in gpui-component 0.9.1, and
  `native-theme-gpui/geometry.rs:230` is `.border_color(rgba_to_hsla(b.border.color))`
  (read today; the crate name picks the connector's `src/`, `candidates`,
  `CG/showcase.rs:2576-2585`, and `geometry.rs:230` is among the lines that must not
  move). The generic line's `(styled.rs, focus_style)` needs no line: one file of that
  name holds the symbol (gpui-component's).
- **Which unit each part goes in** (3I). The Basic page's use of `bordered_button`,
  `ButtonKind::Bordered` and its claims and notes, the tooltip's trigger text and the
  test's new kind go in unit 3.1 with the function: `the_showcase_exercises_every_builder`
  (`CG/showcase.rs:383-416`) requires every `pub fn` of `variants.rs` and `geometry.rs`
  (`public_fns`, `:234-246`, lines that begin `pub fn `) to be used in the code of
  `examples/showcase-gpui/`, so a `bordered_button` with no call there is red, and a
  Button built through it with `Default`'s info would claim what it does not paint.
  `hover_border`, `pub(crate)`, needs nothing from the showcase: `bordered_button`
  calls it, and so does `sinks` in `CG/contract.rs`. The
  `HOW` entries go in unit 3.2 with the element lists of 3B.4: `every_how_entry_is_shown`
  (`CG/showcase.rs:3893-…`) requires each entry's leaf to be listed on an element of
  `docs/showcase-elements.toml`. `bordered_button` is a `variants.rs` function, not a
  `geometry.rs` one, so `GEOMETRY_NOTES` (`every_geometry_builder_has_a_note`, `:572`)
  gains no entry.
- **README** (`connectors/native-theme-gpui/README.md`, *Per-widget geometry*, in
  *Flat buttons* after the `variants::switch_color` paragraph, `:439-443`, before "What
  stays upstream work"): "`variants::bordered_button(n, cx)` gives a `Button` the
  platform's hover and press border, `button.hover_border_color` (Breeze's highlight),
  which upstream's `Default` variant draws in `input`, the text input's border colour.
  It is `None` where the theme states no hover border, and the caller keeps the
  `Default` variant. The variant's own rest fill is a fifth of its border colour, so
  the caller gives an unselected button its fill, `.bg(cx.theme().button)`, after
  `geometry::button`, and a disabled one takes `geometry::button_disabled` after that,
  as any other. A selected button takes no such fill: upstream fills it with the
  variant's pressed colour, `button_active`, as it fills a selected `Default` button,
  and replays the caller's style over that state, so a rest fill would stand where
  `geometry::button_checked`, applied after `geometry::button`, sets none." with a
  `rust,no_run` example in the form of `:419-429`.
- **Contract statement** in `CG/lib.rs`' coverage table (`:112`, the `button` row):
  "flat buttons via [`variants::ghost_button`]" gains "; the hover border via
  [`variants::bordered_button`]".

### 3F.5 If it fails

- `bordered_button`, `hover_border` and the test in `CG/widgets/tests.rs` are not kept; the trial's result (each state that
  differed, and how) is recorded in the commit message, the plan's status line and the
  backlog entry of request 5.
- **Contract.** Both leaves are `UnreadColour` entries with `Unread::NoReceiver`:
  `button.hover_border_color`, "the Default Button draws its hover and press border in
  the `input` token, the text input's border (button/button.rs:1084-1086, :1222, :1298),
  and a custom variant cannot stand in for it (<the difference the trial found>)";
  `button.focus_border_color` as in 3F.4.
- **Showcase.** `info/leaves.rs` `HOW` entries for both leaves on the five elements:
  "not reachable: …" with the same reasons, each line citation with the symbol its lines
  hold, as 3F.4's `HOW` entries (the `UnreadColour` texts need no symbol: their test asks
  only for a `.rs:`, `CG/contract.rs:3028`). `button.hover_border_color` on the three
  elements 3F.4 builds through `bordered_button` (default, tooltip, toggle-off): "not
  reachable: the Default Button draws its hover and press border in the input token,
  the text input's border
  (gpui-component/button/button.rs:1084-1086, border_color; :1222, border_color; :1298,
  border_color), and a custom variant cannot stand in for it (<the difference the trial
  found>)": each range carries its symbol, and `line_citations` reads the `; :N, Symbol`
  groups as ranges of the same file, as the existing entry `info/leaves.rs:97` does
  (`:1084` is `fn border_color`, `:1222` and `:1298` call `self.border_color(outline,
  cx)`, read today in gpui-component 0.9.1); on primary, 3F.4's text for primary; on
  toggle-on, 3F.4's not-drawn line, which holds whatever the trial's outcome; the
  generic `button.focus_border_color` entry as in 3F.4. `demo::ButtonKind` gains no
  `Bordered` and the three buttons stay `Default`, so `edge`, the claims and the
  tooltip's trigger text are unchanged.
- **README** (`connectors/native-theme-gpui/README.md`, the *Limits* section): "A
  button's hover and focus border: gpui-component's `Default` Button draws its hover and
  press border in the `input` token, the text input's border colour, and a focused
  Button's border in `ring` (where it has no border width, a 1 px line: in `ring` on a
  link, a text or an upstream `.ghost()` Button, else in its text colour at 60 %; with
  `Theme::focus_ring` on, the band as well); `button.hover_border_color` and
  `button.focus_border_color` have no receiver (request 5 of the upstream requests)."
- **Upstream request 5**, appended to `docs/issues/0004_kde-hand-overrides-upstream-requests.md`
  (and its title "four requests" → "five requests"). Only the backlog entry gains (5)
  (3H.4). The first rationale's §4.7, with its sentence "Four requests, drafted for the
  maintainer to send to GPUI Kit's tracker", stays: it records what that rationale
  drafted, and request 5 is this one's.

  > **5. A button's border colour per state**
  >
  > gpui-component 0.9.1 draws a `Default` button's border in the `input` token at rest,
  > hovered, pressed and selected (`src/button/button.rs:1084-1086`, read by those four
  > styles at `:1144`, `:1222`, `:1298`, `:1340`), and in `ring` when it is focused
  > (`src/styled.rs:257-270`), and its hover and press
  > styles replace a caller's border colour (`:738-743`, `:751-762`). `input` is the text
  > input's border, so a theme cannot give a button a hover outline of its own. KDE
  > Breeze shows hover, keyboard focus and press on a button by turning its outline to
  > the highlight colour and keeps its fill. A custom variant draws one colour as its
  > border in every state (`:1118-1124`), at rest too, so it cannot stand in. Could the
  > theme carry a button's border colour per state (`button_border`, `button_hover_border`,
  > `button_active_border`, defaulting to `input` as today), or could
  > `ButtonCustomVariant` take a border colour per state?

## Part 3G — The showcases

### 3G.1 Widget Info routes

- iced: 3D.4. egui: the two `mapping.toml` rows give the Theme Map's and Widget Info's
  route lines (`direct` sinks). gpui: 3F.4 or 3F.5.
- `scripts/check_showcase_parity.py --check-list` checks the element list; the three
  showcases' route tests fail on a listed leaf without a route, which is the gate for
  3B.4.

### 3G.2 What changes on screen, per preset

Only on kde-breeze (light and dark), the only preset that states the fields: a hovered
or pressed button's outline turns #3daee9 in iced (the button styles that wear `button.border`, decision 8; not `button_link`,
`segment` or `expander`, `CI/styles.rs:497`, `:1947`, `:2034`, 3D.1),
egui (the «button» role, the scopes that carry the base, every unscoped control of
3E.1), and gpui if the trial passes (the showcase's default, tooltip and toggle-off
buttons); a keyboard-focused egui button's outline turns #3daee9 (`active`). What does
not change: in egui, the controls of the «scrollbar», «slider», «sidebar», «toolbar»,
«status bar», «list» and «switch» scopes keep their resting stroke when hovered or
pressed (the showcases' toolbar buttons and list rows among them; step 5 gives a
toolbar button its outline), and the roles with a border of their own keep theirs
(3E.1's table). **No capture changes**: none shows a hovered, pressed or focused
button. A capture lets no pointer event reach the page (egui drops them while
capturing, `showcase-egui/app.rs:975-988`; iced's `held_page` shows no pointer,
`showcase-iced.rs:3238-3247`; gpui's pointer shield, `a_capture_hovers_nothing`,
`showcase-gpui/tests.rs:6253`), no script or workflow passes `--pointer` (`grep` over
`scripts/` and `.github/` today), no key reaches it either (2H.5), and the start-up
focus is a text field's (2H.1), whose focused stroke is the «input» cell's own in
egui and no button border in iced or gpui. Step 3 adds nothing to the release's one
asset run (2K, S5) but what the stamps hash (the sources).

## Part 3H — Outside the code

In 3H, `<A | B>` gives the text for the trial's two outcomes: `A` where it passes
(3F.4), `B` where it fails (3F.5).

### 3H.1 `CHANGELOG.md`

Into the release's one section, `## [0.7.3] - <date>` (2I.1, S5), under its headings,
after step 2's entries of each heading (a `### Changed` heading is there already from
step 2). The *Breaking Changes* entry follows 0.7.0's wording, as 2I.1's does; that note
is for the reader and is not written into `CHANGELOG.md`:

- `### Breaking Changes`: step 2's entry becomes the release's one entry, naming both
  structs (S5: the break is named once):
  - **native-theme**: `ThemeDefaults` gains `focus_ring_keyboard_only` and `ButtonTheme`
    gains `hover_border_color` and `focus_border_color` (see *Added*). Neither is
    `#[non_exhaustive]`, so a struct literal of either outside the crate needs the new
    fields or `..Default::default()`; their resolved forms are `#[non_exhaustive]`
    already.
- `### Added`
  - **native-theme**: `button.hover_border_color` and `button.focus_border_color`, a
    button's outline under the pointer (kept while pressed) and with the keyboard focus.
    kde-breeze and the KDE reader state Breeze's highlight for both (#3daee9); no other
    bundled preset states them (GNOME and Material draw no button outline, Windows keeps
    its rest outline on hover; a focused button's outline on Windows and macOS, and
    macOS 14–15's on hover, were not read). Unset, a connector draws the rest border
    as before.
  - **native-theme-iced**: the button styles draw `button.hover_border_color` as the
    border of a hovered and a pressed button.
  - **native-theme-egui**: the base style and the «button» role draw
    `button.hover_border_color` as the hovered stroke and `button.focus_border_color`
    (else the hover border) as the pressed and focused one. A control egui draws with
    the base style outside the connector's roles takes them too (on KDE an unscoped
    check box, radio, combo box, drag value, selectable label or text field, among
    others, outlines itself in the highlight under the pointer; a slider's handle does
    not); the «scrollbar», «slider», «sidebar», «toolbar», «status bar», «list»
    and «switch» scopes keep their rest stroke, so a toolbar button or a list row shows
    no new outline.
  - **native-theme-gpui** (if the trial passes): `variants::bordered_button`, a Button
    with the platform's hover and press border.
- `### Changed` (if the trial fails): **native-theme-gpui**: the README's *Limits*
  names the button's hover and focus border, which gpui-component cannot take.
- `### Changed` (if the trial passes): **Showcases (gpui)**: the Basic page's default,
  tooltip and toggle-off buttons are built with `variants::bordered_button` where the
  theme states a hover border (kde-breeze), so they take Breeze's outline under the
  pointer and while pressed, and their Widget Info claims the variant's colours (3F.4);
  the toggle that is on stays upstream's `Default` Button, since gpui-component gives
  a selected Button no hover or press style.

### 3H.2 `ROADMAP.md`

In the release's one section, whose heading 2I.2 gives (S5), a second paragraph after
step 2's **Delivered** paragraph: "The model also carries a button's outline under the
pointer and with the keyboard focus, which Breeze turns to the highlight colour; kde-breeze states it, and no
other bundled preset does. The egui connector draws both; the iced connector draws the
hover border (an iced button takes no keyboard focus)<; the gpui connector offers a
button variant with the hover border (a focused gpui-component button's border, where
it has one, is its `ring`) | ; gpui-component has no receiver for either, and the request to add one is
drafted>."

`:264`'s count of open `[v0.7.x]` entries is recounted as 2I.2 says, on the documents
unit's tree (3.3), after 3H.4, which opens two (*macOS: the button's hover fill* and
*gpui showcase: the toggle that is on …*) and closes none; the commit body records it.

### 3H.2a The READMEs' count of optional colours

Five places count the model's optional colours, each with the number 34:
`README.md:71` ("34 colours, most of them state shades such as
`checkbox.hover_background`"), `README.md:74` ("their colours (five of the 34)"),
`native-theme/README.md:48` ("every colour but 34 optional ones"), `:49` ("most of the
34 are state shades") and `:52` ("the track and thumb outlines' colours (five of the
34)"), the lines at `0fa28027`, each found by its text on the tree (step 2's README
item, 2I.2a, comes first). This step edits only the numbers. The 34 are the `#[theme(category = "soft_option")]` fields of type
`Option<Rgba>`, at `0fa28027` all in `native-theme/src/model/widgets/mod.rs` (counted
today with a `python3` read of every such field under `native-theme/src/model/`; step
2's `focus_ring_keyboard_only` is a `bool` and does not count). Step 3's two are such
fields (3A.1), so in the documents unit (3.3) each of the five 34s becomes the count the
same read gives on that tree, 36 where no step between adds one; the steps 4–5
specification's A.12 counts on from it. "five" stays five: the switch's outline colours
are `unchecked_track_border_color`, `checked_track_border_color`,
`disabled_unchecked_track_border_color`, `disabled_checked_track_border_color` and
`thumb_border_color` (`native-theme/src/model/widgets/mod.rs:780`, `:785`, `:789`,
`:792`, `:802`), and step 3 adds a button's colours, no switch's.

### 3H.2b `docs/showcase-exceptions.toml`: the hover run's button edge

The parity group whose one key is `"hover@basic.buttons.default.border"`
(`docs/showcase-exceptions.toml:651-655`) gives as its reason "…, so no style the
application gives reaches the hovered edge; the model states no hovered button edge,
and iced and egui keep button.border.color", which this step makes false: the model
states one, and iced and egui draw it where the theme states it (3D.1, 3E.1). The key
is not scoped to a preset, so the group stays for the presets that state none. In
unit 3.3, the reason's text from "so no style the application gives" to its end
becomes `<A | B>`: A, "so no refinement the application gives reaches the hovered
edge, only a custom variant's colour, which the Basic page's default button takes
through variants::bordered_button where the theme states button.hover_border_color;
where it states none, iced and egui keep button.border.color and gpui draws the input
token"; B, "so no style the application gives reaches the hovered edge; iced and egui
draw button.hover_border_color where the theme states one and button.border.color
elsewhere, and gpui the input token in either case (request 5 of the upstream
requests)". The text before it, with its citations, is unchanged, and so is its key,
which stays unscoped. No gate reads a group's reason text.

### 3H.3 The first rationale (`docs/issues/0004_kde-hand-overrides-rationale.md`)

- Table row C (`:1030`), "the hover and focus border wait for §5.4" → "the hover and
  focus border: **fixed in 0.7.3**: egui draws both; iced draws the hover border (an
  iced button takes no keyboard focus)<; gpui: the hover border through the connector's
  `variants::bordered_button`, a focused button's border, where it has one, being
  gpui-component's `ring` |
  ; gpui-component cannot take either yet (request 5)>".
- **§5.4** (`:907`), a line after the heading, in the form of 2I.3's §5.1 line:
  "*Partly settled by `0004_kde-open-items-rationale.md` D5 (0.7.3): a button's border on
  hover and on keyboard focus (`button.hover_border_color`, `button.focus_border_color`),
  which kde-breeze and the KDE reader state; egui draws both; iced draws the hover
  border (an iced button takes no keyboard focus)<; gpui the hover border, through
  `variants::bordered_button` | ; gpui-component cannot take either yet (request 5)>.
  The check box, the radio and the combo box are that rationale's
  D6, toolbar buttons and the table header its D7; the menu item's and the list row's
  outlines and the menu bar title stay in the backlog (its §0).*"
- **The reply draft (§8)**, under 2I.3's rules (only while it has not been posted; by
  anchor and by content; "<the release that brings step 3>" replaced by its version as
  2I.3 says; the note keeps every sentence still in the draft and its caveat), on the
  draft as step 2 leaves it. Where this step's anchors are step 2's own sentences,
  they are found by the sentence's start, which holds whether step 2's placeholder is
  still in the draft or already replaced by its version: "> With " followed by the
  release and "you can drop `focus_ring`"; the bullet titled **The focus ring**.
  1. The bullet whose title is **Hover borders on buttons.**, under "Some things are not
     fixed yet", is removed where the trial passed. Where it failed, it is replaced, in
     its place, by what stays open:

     ```markdown
     > - **Hover borders in GPUI.** gpui-component's button draws its hover border in
     >   the text field's border colour and cannot take one of its own yet, so I have
     >   asked for it. Until then, setting it yourself, as you did, is the way to get
     >   Breeze's.
     ```
  2. The list "Later 0.7.x releases fix more:" (2I.3, item 2) gains, after its bullet
     **The focus ring**:

     ```markdown
     > - **Hover borders on buttons** (<the release that brings step 3>). Breeze
     >   shows hover by recolouring a button's border, not its fill, and keyboard
     >   focus the same way. native-theme now has values for both, Breeze's
     >   highlight colour read from your colour scheme. The egui connector draws
     >   both; the iced connector draws the hover border (an iced button cannot take
     >   the keyboard focus).
     ```

     Where the trial passed, the bullet goes on: "In GPUI,
     `native_theme_gpui::variants::bordered_button` gives a button the hover border;
     a focused button's border is gpui-component's focus colour, which on KDE is
     that highlight already." (kde-breeze's `ring` token is the opaque ring colour,
     #3daee9, since step 2, 2E.1.) Where it failed, it ends there, and GPUI's part
     is item 1's bullet, so no item stands in both lists.
  3. Where the trial passed, the paragraph that begins "> With " and contains "you can
     drop `focus_ring`" (2I.3, item 3; one release brings both steps, S5, so one
     paragraph) gains, after its last sentence: " You can also give a button its hover
     border with `native_theme_gpui::variants::bordered_button` instead of setting it
     yourself." Where it failed, nothing: the reporter keeps the hand-set border.
  4. "The rest of your list": in its sentence on toolbar buttons, "they come in the
     same series of releases as the hover borders" becomes "they come in a later
     0.7.x release" (the hover borders are then out, and the old words would read as
     if toolbar buttons came with them). The sentence on the pressed sort header
     ("it comes last") stays. The paragraph after this step, from step 1's text (its
     G.3) with this one change, step 2 leaving it as it is:

     ```markdown
     > The rest of your list, briefly. Toolbar buttons: native-theme has no values for
     > their states yet; they come in a later 0.7.x release. The pressed sort header
     > has none either; it comes last, and since no GUI library's table can show it,
     > its values will reach an application only for a table it draws itself. Dialog
     > text is the general font, and Breeze sets no dialog window size. Taking dark or
     > light from `system_is_dark()` for the fallback was right. And for looking at
     > every colour of a theme, the showcase example shows each widget's colours with
     > where they come from.
     ```
  5. The note after the draft. "It is planned for a coming 0.7.x release" and "In
     GPUI it depends on whether gpui-component's buttons can take it, which I have
     yet to try" leave the draft with item 1, and "they come in the same series of
     releases as the hover borders" with item 4; "they come in a later 0.7.x
     release" comes in, and, where the trial failed, "so I have asked for it". The
     note becomes, where the trial passed:

     ```markdown
     The sentences "I am reporting it there", "I have asked gpui-component to draw
     its ring for keyboard focus only", "they come in a later 0.7.x release" and "it
     comes last" are the maintainer's to confirm before posting: they follow
     `0004_kde-open-items-rationale.md`'s proposed order (D10), and "0.7.x" its
     proposed versions (D0 (a); under (b) or (c) some of these releases are minor
     ones, and the wording changes with them).
     ```

     and where it failed the same with "so I have asked for it" after the ring's
     sentence:

     ```markdown
     The sentences "I am reporting it there", "I have asked gpui-component to draw
     its ring for keyboard focus only", "so I have asked for it", "they come in a
     later 0.7.x release" and "it comes last" are the maintainer's to confirm before
     posting: they follow `0004_kde-open-items-rationale.md`'s proposed order (D10),
     and "0.7.x" its proposed versions (D0 (a); under (b) or (c) some of these
     releases are minor ones, and the wording changes with them).
     ```

  Step 5 (toolbar buttons) then finds that sentence by its start, "Toolbar buttons:",
  not by the words this step replaces.

  **The draft's two lists as they stand after this step** (the focus ring's bullet as
  2I.3 gives it):

  ```markdown
  > Later 0.7.x releases fix more:
  >
  > - **The focus ring** (<the release that brings step 2>). […]
  > - **Hover borders on buttons** (<the release that brings step 3>). [item 2's text,
  >   by the trial's outcome]
  >
  > […]
  >
  > Some things are not fixed yet:
  >
  > - **The 16 px scroll bar in tables** is a bug in gpui-component's table, which
  >   ignores the theme's width. I am reporting it there.
  > - **Hover borders in GPUI.** [item 1's text; only where the trial failed]
  > - **Menus in GPUI.** gpui-component paints every menu in its `popover`
  >   colour, the window's on KDE (since 0.7.0, so you no longer need to set
  >   `popover`), and has no colour of its own for a menu, so in GPUI the small
  >   difference from Breeze's menu colour stays.
  ```

### 3H.4 The backlog

- *What Breeze draws that the model has no field for* (`docs/todo.md:1774-1825`): its
  first part ("First, as the next 0.7.x step, … so on KDE a connector's button shows no
  hover at all", `:1774-1780`) is reworded to what remains: "Breeze shows hover on a
  combo box, a check box and a slider handle by recolouring the outline, never the fill
  (open-items rationale D6; a button's outline is in the model since 0.7.3)". Two
  sentences after it named that step and would point at nothing: "The kde-breeze hover
  fills that stand in for an outline wait for it, since removing them first would remove
  the only hover feedback" (`:1780-1782`) becomes "… stand in for an outline wait for
  their fields, since removing them first would remove the only hover feedback", and
  "The same step takes:" (`:1785`) becomes "Breeze also draws, with no field yet:". The
  rest of the entry is unchanged.
- `[upstream]` *GPUI Kit, from issue #4* (`:2742-2764`): if the trial fails, "Four
  requests" → "Five requests" and a sentence for (5) in the entry's form.
- A new `[v0.7.x]` entry in the issue-#4 section: "**macOS: the button's hover fill.**
  `docs/platform-facts.md`'s `button.hover_background` macOS cell gives a measured
  'subtle CoreUI highlight' on 14–15 and ❓ on 26, while the parked facts record that
  hover draws nothing on a macOS 26 push button (`docs/facts/macos/state-models.toml:1-3`
  at `78ee0efd`, OS probe run 40969040283). Re-read both and correct the cell."
  (*Problems found*, 2.)
- A new `[v0.7.x]` entry in the issue-#4 section: "**gpui showcase: the toggle that is
  on claims a hover and press look it does not paint.** `demo::toggle_button`
  (`connectors/native-theme-gpui/examples/showcase-gpui/demo.rs:9166-9202` at
  `0fa28027`, as every line of the connector's files below) builds its info as an idle `ButtonKind::Default` Button's (`:9174-9181`),
  so it claims the Default Button's rest fill `button` (`info/buttons.rs:20-25`), its
  hover and press fills `button_hover` and `button_active` (`:164-177`), its hover and
  press border `input` (`:353-358`) and, in the idle edge note, a repaint in `input`
  under the pointer (`:534-535`), while the Button is `.selected(true)`
  (`demo.rs:9193-9199`): gpui-component paints a selected Button in its selected style,
  `tokens.button_active` (`gpui-component 0.9.1 button/button.rs:1327`) or
  `geometry::button_checked`'s fill where the theme states one, and gives it no hover or
  press style (`:730-762`); under a native theme its label is `button_checked`'s colour,
  not the claimed `button_foreground` (`info/buttons.rs:26-31`). Until this is done, its
  Widget Info shows the true "not reachable" line the hover-border release added for
  `button.hover_border_color`
  beside the old false claim "border, hovered or pressed" in `input`. Give it a selected
  Button's info, as `native_ghost_selected` (`info/buttons.rs:629-652`) does for a
  selected Ghost." (*Problems found*, 4.)

### 3H.5 The rationale's status line

`docs/issues/0004_kde-open-items-rationale.md:3-4` gains, in the release's close-out, as
the steps 6–7 specification's 7.I.4 gives its own, the one sentence for both steps that
2I.6 gives (S5), word for word, with 2I.6's one placeholder: "Steps 2 (D4) and 3 (D5)
implemented on `<date>` for <the release that brings step 2>, by
`../archive/todo_v0.7.x_kde-open-items-steps2-3-{spec,plan}.md`." (the archive's path,
since 3H.6 moves the documents in the same close-out). The rationale
stays in `docs/issues/` while a step remains.

### 3H.6 `docs/archive/`

At the release's one close-out, after step 3 (S5), this specification and its plan move to
`docs/archive/` (`docs/archive/README.md`: implemented design documents live there; the
project's rule that archiving is a plan's last task), and `docs/archive/README.md`
gains a row for <the release that brings step 3> (or a line on that release's row, if it
has one by then): "`todo_v0.7.x_kde-open-items-steps2-3-{spec,plan}.md`: steps 2 and 3
of the open KDE items, one release (the focus ring; a button's border on hover and on
keyboard focus); their rationale stays with the issue, at
`docs/issues/0004_kde-open-items-rationale.md`". Every link to the two documents is
rewritten for the new place, found at the close-out with `git grep -n
'kde-open-items-steps2-3'` over every tracked file outside `docs/archive/`; that grep is
the rule, not a list, since the other documents are still being edited. Among what it
finds: the rationale's status line of 2I.6 and 3H.5 (written with the archive's path
in this close-out), and this specification's own links, in its header, which are
relative to `docs/`: `issues/…` → `../issues/…`,
`archive/todo_v0.7.1_kde-hand-overrides-spec.md` → `todo_v0.7.1_kde-hand-overrides-spec.md`
(the same folder then), and `todo_v0.7.1_kde-open-items-step1-spec.md` to where step 1's
close-out put it (its G.6: `docs/archive/`, the same folder then). A document that has
itself moved to `docs/archive/` by then is found there by the same `git grep`. Every
path mention of the two documents in a tracked file outside `docs/archive/` is
rewritten, a document whose header dates its lines exempt only for its line
citations, never for a path mention; left as they are, by rule 22 (a)'s exemptions,
are a mention that already points into `docs/archive/` (in any relative form or as an
absolute URL), a citation dated "at <sha>", and a path inside another document's
instructions that names its own future target, the last two each named in the
commit body. A
later document not tracked at the close-out's commit is not edited by this work: its
own plan's path check after its merge re-points the paths that moved.

## Part 3I — Order and gates (step 3)

On the release's one branch, `v0.7.3-rc1`, after step 2's units (S5): no branch and no
commit (0) of its own; the step starts with the trial, unit 3.0. Units as 2K defines
them, gated by 2K's cadence (S1): unit 3.0 is uncommitted and has no gate; unit 3.1, the
first code unit after 2.6's `--commit`, runs `--quick` (no unit is structural, rule 24);
unit 3.2, the second, which holds 3H.2b's TOML string, is the release's last code unit
before its documents and version units and runs `--commit` (rule 6); unit 3.3, a
documents unit, is the last unit before the merge and runs the five checkers and
`--quick`, its commit the lane's gated head for steps 4 and 5 (2K, rule 20); the merge's warm-up gate restarts the count, X runs the five checkers and
`--quick`, and the version step runs `--commit` (rule 6: its `Cargo.lock` change).

| Unit | Contents | Why these belong together |
|---|---|---|
| 3.0 The trial | 3F.1-3F.3 run, uncommitted, in the worktree with 3A.1 (the two fields), 3A.2's two full literals and 3B.5's kde-breeze lines applied, so that the trial runs exactly the code 3F.1 keeps: `bordered_button` through `hover_border(n)`, which reads `n.resolved.button.hover_border_color`, on kde-breeze's stated value and, for windows-11 and macos-tahoe, on the value the test sets (3F.2). Its result, and where it fails the state that differed, go into the plan's status line (decision 12). Where it passes, those edits stay in the worktree and unit 3.1 commits them with the rest of its parts; where it fails, the trial's code goes and the model, test and preset edits are reverted with it (`git restore --staged --worktree --source=HEAD -- <the files>`), so unit 3.1 starts from step 2's last commit (unit 2.7's, its fixes included) | 3F.4 or 3F.5 decides what unit 3.1 holds for gpui. |
| 3.1 The fields | 3A (the fields, the two full literals of 3A.2); 3B.1 (the two facts rows); 3B.2; 3B.3; 3B.5 (kde-breeze); 3C (the reader and its tests, `kde/mod.rs`' two lists and the accent test's list); egui's 3E.1 (the base's and «button»'s strokes, the seven roles' kept strokes), 3E.2 (the two new rows, `button.border.color`'s changed and added sinks, `mapping.md`, the count 517 → 519, the buttons totals `[11, 46, 31, 25]` → `[13, 46, 31, 25]`), 3E.3 (the new tests, the four changed ones in `roles.rs` and `states.rs`, and `no_border_colour_carries_the_border_opacity` in `CE/style/derived.rs`) and 3E.4 (T10's two formulas in `CE/contract.rs`); iced's 3D.1-3D.3 (the styles, `native_button_border`, the `UNREAD_COLOURS` entry, the tests) with `contract-leaves.toml` regenerated; gpui's contract and connector part of 3F.4 (`bordered_button`, `hover_border`, the `sinks` entry, the trial's test in `CG/widgets/tests.rs`, the `UnreadColour` of `button.focus_border_color`) with the showcase's use of `bordered_button` on the Basic page's buttons through `ButtonKind::Bordered`, its claims, `edge` branch and notes, the tooltip's trigger text and the info test's new kind, or of 3F.5 (the two `UnreadColour` entries) | From the moment the fields exist: `property_registry_keys_match_derive_inventory` until the registry lines, and `check_platform_facts_rows.py` until the facts rows; egui's leaf count and `mapping_md_is_current` until the rows, the count and the file, and egui's splice tests (T2) until the strokes the rows declare are written; iced's `every_colour_leaf_reaches_a_row_or_is_named` and gpui's `every_colour_leaf_reaches_a_sink_or_is_named` until each new leaf reaches a row or sink or is named, since both splice a colour into a leaf that is `null` in 32 combinations (`CI/contract.rs:4894`, `CG/contract.rs:2701`); iced's test also until `contract-leaves.toml` matches its walk, which the new leaves and kde-breeze's stated values (the `except` lists of the border rows) both change. egui's `verdict_totals_match_the_specification` until the buttons totals; T2 until a preset pair differs in each new leaf (kde-breeze against adwaita does, so the preset is in the unit) and until the seven roles' cells declare their own strokes; `hover_and_active_keep_the_resting_stroke_and_radius`, `panel_role_cells_write_their_rows`, `switch_cell_writes_its_rows` and `no_border_colour_carries_the_border_opacity` until they read the new strokes (3E.3; `button_cell_writes_its_rows` stays green, since its two presets state neither field, and changes with them); T10 (`t10_every_sink_holds_the_leafs_value`) until its two formulas, since the base's and «button»'s hovered and active strokes are then declared by more than one row (3E.4). Where the trial passed, gpui's `the_showcase_exercises_every_builder` (`CG/showcase.rs:383-416`) until the showcase calls the new `pub fn` `bordered_button`, so its use is in this unit (3F.4). The preset is in the unit so that the export is regenerated once; the reader is no input of it (2K, unit 2.2), and is here because `the_derived_colours_are_the_presets_under_breeze` holds it to the preset once its lists name the two leaves. |
| 3.2 Showcases | 3B.4 (the element lists); iced's routes, which come from unit 3.1's export with no code of their own (3D.4); the gpui showcase's `HOW` entries of 3F.4 or 3F.5 (`every_how_entry_is_shown` needs the leaves listed; the buttons' use of `bordered_button` is unit 3.1's); egui's routes from unit 3.1's rows; 3H.2b's exception text (a TOML string, so code, rule 11) | A listed leaf without a route is red in the iced and gpui route tests (2K, unit 2.6); `check_showcase_parity.py --check-list` needs the registry keys; 3H.2b is the showcases' parity exception, and its being here leaves 3.3 a documents unit (rule 20). |
| 3.3 Documents | 3H.1-3H.4 but 3H.2b (3H.2a and `ROADMAP.md:264`'s recount included), the README of 3F.4 or 3F.5, request 5 where the trial failed | — |

Contrast pairs: no pair of gpui's or iced's names a button's border (2K, unit 2.2's
reading), so the hover and focus borders move none. After 3.3: the merge and X, as 2K
says (S5); no citation of a line this step moves is re-pointed (2B.6).

Then the release's one close-out, as 2K says (S5, S4): the version, the compatibility
run, the push of both steps' units, the full check while CI runs, the one asset run
(step 3 changes no capture, 3G.2), and, last, the rationale's status line (3H.5, with
2I.6's step 2 in the same sentence) and the move to `docs/archive/` with its row and
the re-pointed links (3H.6), in one commit.

Semver and D0's variants as 2K says for the release: its one compatibility step
reports `ThemeDefaults` and `ButtonTheme`, and `arb_theme_defaults` and
`arb_button_theme` are the integration tests that name every field.

---

## Problems found

1. **kde-breeze's caret.** kde-breeze states no `input.caret_color`, so the preset
   resolves it to `defaults.text_color` (`resolve/inheritance.rs:133-135`), #232629 /
   #fcfcfc, while `PF:1209`'s KDE cell gives `[Colors:View] DecorationFocus` and the KDE
   reader writes that (`kde/colors.rs:565`); the parked facts read Qt's line edit as
   drawing its caret in the text colour (`F/kde/tokens.toml`, the S4.2 comment at
   `:1595-1597`), and the backlog already asks to measure it (`docs/todo.md:2005-2006`, item
   (8)). Not changed here; that backlog item gets this finding appended (2I.5).
2. **The macOS hover fill.** `PF:1192`'s macOS cell gives "(measured) subtle CoreUI
   highlight" for 14–15 and ❓ for 26, while the parked facts record that hover draws
   nothing on macOS 26 (`F/macos/state-models.toml:1-3`). Outside these steps; recorded
   in the backlog (3H.4).
3. **The ring in iced's inset route** shows on any focus where the theme says keyboard
   only (2F.1). No bundled preset combines the two; the doc says so.
4. **The gpui showcase's toggle that is on.** Its info is an idle `Default` Button's,
   with a rest fill, hover and press fills and an `input` hover border, while the
   Button is selected, which gpui-component paints in its selected style with no hover
   or press style (3F.4, *Showcase: the three buttons*). It is older than these steps
   and not corrected in them: the correction gives `info::buttons::button` a selected
   path through its `variant`, `colours`, `edge`, edge-note and state matches (or a
   function of its own beside `native_ghost_selected`), whose fill claim depends on
   whether the theme states a checked or pressed fill, with a test of its own; step 3
   only gives its `button.hover_border_color` a not-drawn `HOW` line (3F.4). Recorded in
   the backlog (3H.4).

## What could not be verified

- No mechanism of step 2 and no part of step 3's trial was built or run (the rationale's
  §4 says the same); 2J and 3F.5 say what happens if they fail.
- The URLs built from the parked facts' pinned commits (the WinUI ones in 2B.1 and
  3B.1, and the two androidx ones, on the GitHub mirror at the full commit of
  `androidx@e4d933c9`) and the GTK `gtkcssstylepropertyimpl.c` link were not opened (no
  network). That the mirror holds `e4d933c9` rests on the parked source's note
  (`docs/facts/sources.toml:843` on `v0.8.0-rc1`).
- The `HOW` texts of 2H.3, 3F.4 and 3F.5 were checked against `every_prose_citation_still_exists`
  by reading its parsers and by a Python transcription of `line_citations` run on the
  texts and on gpui-component 0.9.1's files (each range parsed with its symbol, and each
  symbol in its lines), not by running the test; `focus_band_on` and `ring_colour`
  exist only after 2E.1.
- Breeze's band outlasting a mouse move rests on the facts' wording, as the rationale's
  §4 says.
- Settled from the sources since the last version, no longer open: a focused iced
  `text_editor` does not capture Tab (2F.5); the gpui showcase moves the focus with Tab
  (2H.3); gpui's test mouse events go through `Window::dispatch_event`
  (`GP/platform/test/window.rs:297-306`, `GP/window.rs:2241-2249`), so they set the
  modality as a real one does; egui's T2 and T4 helpers handle an `Option<bool>` leaf
  (`spliced` and `toml_to_json`, `CE/mapping_tests.rs:1205-1234`; `with_hostile` writes
  floats only, `:732-737`), and iced's and gpui's `colour_leaves` skip it (2K, unit
  2.1); a comment in the batch's form that names two keys is one LN's checker reads
  and compares with nothing (GPUI spec I.3).
- T2's treatment of the seven roles' new scoped sinks (3E.2) was read from
  `own_write`, `cell_own` and `carried_by`, not run.
- Whether the `Default` variant's `tokens.*` colours and the `ThemeColor` values the
  custom variant copies are bit-equal as `Hsla` (3F.2): the comparison is made at 8 bits.
- That no unit of 2K and 3I leaves a gate red was read from the tests named there; no
  gate was run (no cargo command in this task). In particular: T10's two formulas and
  its fallback for the 14 scoped sinks, and T10c over the seven cells (3E.4); that
  unit 2.1 leaves iced's `contract-leaves.toml` as it is (2K); and the exact route
  lines iced's showcase forms for the two new leaves from the regenerated export
  (3D.4).
- egui's rule (2G.1) was checked against the sequences by reading egui 0.36.2's and
  0.37.0's focus code (decision 14), not by a run; its tests (2G.3) are specified, not
  written.
- That today's captures of kde-breeze, material-3 and windows-11 show the start-up
  field's ring (2H.5) was read from the code, not from the image files.
- `ButtonKind::Bordered` (3F.4) was checked against the gates by reading them, not by
  building: that its claims pass `every_colour_claim_is_read_at_the_line_it_cites` and
  `every_edge_colour_is_claimed_through_edge_claim` (the cited lines `<F>`, `<H>`, `<A>`,
  `<R>` exist only after unit 3.1, so the four spans were not opened), that its notes'
  `file.rs, Symbol` citations pass `every_prose_citation_still_exists`, and that no
  other test of the gpui showcase reads the Basic page's buttons' variant name
  ("Default", now "Bordered" on kde-breeze) or their claims: the tests that name
  `basic-button`, `basic-button-tooltip`, `basic-toggle-off` or `basic-toggle-on`
  (`showcase-gpui/tests.rs:5638`, `:5875-5891`, `:6270`, `:6284`, `:6359`, `:6374`) were
  found with `grep` and read where they assert a text, not run.
- The focused-button texts of 3F.4 (the band, the border, the 60 % line) were read from
  gpui-component 0.9.1's `focus_style` and `Button::render`, not drawn; which bundled
  presets leave the band on after step 2 besides adwaita (2E.1 names adwaita as the
  one the test pins) and macos-tahoe (3F.2's focused row, from 2E.1's rule and the
  preset's values, not run) was not counted, and the texts say "where the band is on".
- The egui controls 3E.1 and the CHANGELOG name as taking the base's new strokes are
  the ones read in egui 0.36.2's sources for this; the list is not exhaustive, and
  none was drawn.
