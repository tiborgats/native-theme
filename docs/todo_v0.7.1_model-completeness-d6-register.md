# The register's decision rows, settled by sources and logic (D6, outcome 14)

2026-11-11, revision 5. Revisions 1 and 2 (2026-11-10) were written by a design agent, revisions 3, 4 and 5 by
other agents; all from the worktree `m1b-close` at 7430a0a2, read-only. Of the fifteen rows of kind `d` of the
ledger's *Unmeasured register* it examines, it settles fourteen, two of them in part (U180: QPainter's underline;
U504: the two mnemonic tokens), and moves one, U555, with the token `single_click_activates` to the unit that brings
the item views; it examines one more row, U473, by D13's criterion. Every outcome is open to the maintainer's veto;
section 7 lists the picks and choices of meaning, and two items that are no pick. The open points stay in the rows
that raise them: the token's question in the token's part (parts 2, 4 and 6, for the containers unit), and U180's
model form for Qt Quick's line position in U180, part 3 (for the flavours unit).

**Corrections of ADD-A147's fix round (2026-11-12), the orchestrator's decisions on the batch's checks** (each open
to the maintainer's veto, `docs/todo_v0.7.1_model-completeness-d6-veto-list.md`): `button`'s `label_position` is
at depth wherever a preset states it with no later name (ledger queue #258), so U697's pending fact is on 0.8.0's
release path, its facts edit due before 0.8.0 (section 1's table and section 4's comparison are corrected below).
U180's open rest is two rows: Qt Quick's line position, U753, takes the non-breaking form, a field on `Decoration`
defined with the flavours unit; QPainter's wavy and strike lines are U754, due with the first unit that states one.
The status text colours (U456): on `macos-*` they refer to `role.global.accent_foreground`
(`alternateSelectedControlTextColor`), measured over each status fill by U756 (fix round N1, withdrawing
`labelColor`, which 0.7.0's preset names the body text beside a status mark); on `ios-*` to the colour UIKit is
measured drawing as a filled button's title over each status fill (U755, decided), 0.7.0's having been a literal with no source; each paired only where its fill
is, and pending while that fill's read is open (U757–U760; fix round N3).

**Correction of the document pass (2026-11-12), the orchestrator's decisions on the full document pass after
ADD-A147** (ADD-A148, each open to the maintainer's veto): in U180's part 4, point 4 (*Typing*), a `<n>px` literal
beside a font size in either of `if_le`'s pairs, and in `div`, is a font size, converted and scaled as one (S9.4),
not a length compared as px; `div` is named beside `add`, `sub`, `min` and `max` where a font size meets a length,
as the code types it (K1-2). U509's Consolas is cited to `fonts.ts:41` alone at M2's task 0, `typography.md:203`
added once U520's checker notation exists, U520 having left 0.8.0's path by the staging (U509's part 4; L-9). The
help bezel's member (U314 and U614, part 4) gets a register row of its own, U762, due with the unit that brings
`help`, in place of U708 gaining it, since U708 retires at M2's task 7 (L's minor 3).

## Revision 5

Made after the narrow check of revision 4, `scratch-keep/d6/register-rows-check4.md` (nothing serious, 2
should-fix, 9 minor). Each finding was verified at its cited lines before anything changed; section 6.4 lists the
eleven. All are applied; one citation inside finding 1's proposed sentence did not hold (it names S5, *Widgets and
parts*; the model's conditions are S7.1 and its states S7.2), and the sentence is applied with S7.1 and S7.2. What
changed:

- **`if_le`'s bound** (finding 1). The `if_le` row gains the sentence that separates it from a condition: it states
  a comparison the platform's code makes, quoted by the fact's citations; a value that changes with a state, a
  setting or a condition the model names is a rule under that condition. "R5.4 bounds their use" is withdrawn
  (R5.4 bounds additions, not uses). The two loose wordings about R5.4 are corrected (finding 9): `alpha` and `mix`
  are what S2.2 counts as the model's own arithmetic, and R5.4's list for lengths has been narrower than S2.2 since
  ADD-A56.
- **U180's Qt Quick form against the patch release** (finding 2). The flavours unit ships as a patch release; of
  the two model forms for Qt Quick's line position, a new `ElementDatum` member is breaking (the enum is public and
  not `#[non_exhaustive]`, and `FontMetrics` is a public trait with no default methods), a field on `Decoration`
  is not (it is `#[non_exhaustive]`). The constraint is stated, and the choice is listed as an open point of the
  flavours unit (U180, parts 3 and 4; section 2.4).
- **The recomputation** (findings 3 and 4). `register/u180_ops_check.py` now holds the two positive controls, each
  mutating both facts (the offset and its antialiased variant), and prints their counts per pen path. It models
  QPen on the antialiased path: a negative width is ignored, a zero width is a 1 px cosmetic pen. On that path the
  form is exact for `t > 0` only: every input with `t ≤ 0` differs (142,742 of them), and the document now states
  the antialiased form for `t > 0` only (U180, part 2; the S2.8 sentence, part 4, item 9; the lane's verifier's
  point; section 5). The facts take the non-antialiased form, exact for every `t`, so no fact changes. The U505
  count is evaluated from the fact's own `if_le` argument, with a control. The figures of U180, part 2, are the
  script's new output.
- **The tools and the code** (findings 5 and 6). `check_facts.py`'s `round_pt` rule must learn `ceil`; the code
  list names `Func::ALL`'s length, the typed tree's new variants, the three doc comments, the five test
  implementations of `FontMetrics` and the datum's match arms, and drops the parser (it reads names through
  `Func::from_name`).
- **`if_le`'s types** (finding 7): `a` and `b` of one type, typed apart from the type the place expects.
- **U505** (finding 8): what still blocks `solid_while_typing`, a source read of WinUI's typing path.
- **The header** (finding 10) says where the open points are.
- **Section 2.4** (finding 11) notes that `[staging2]`'s U180 line will be stale once the batch lands.

## Revision 4

Made after the narrow check of revision 3, `scratch-keep/d6/register-rows-check3.md` (3 serious, 4 should-fix,
7 minor), and two decisions of the orchestrator: (C) `single_click_activates` leaves this round for the unit that
brings the item views; (D) U180 takes `ceil` and one comparison form, added to the model's operators. Each finding
was verified at its cited lines before anything changed; section 6.3 lists the fourteen. Both decisions are applied.
One premise of each was found inexact, and section 6.3 records both with their evidence: `ios-*` does not state the
token (it is pending, U472), and R5.4's last sentence governs every function, the model's arithmetic included. The
staging design's revision 2 (2026-11-11) is cited where these rows need it. What changed, per row:

- **U180.** The outcome is now S2.2's arithmetic with two operators added, `ceil(x)` and `if_le(a, b, x, y)`
  ("`x` where `a ≤ b`, else `y`"), a class 2 change. The offset fact is 215 characters long and holds no literal to
  exempt. Revision 3's 418-character form, its two named forms and their S17 exemptions are withdrawn. The options
  are weighed again with check 3's (a2), (b) and (b′), S2.2's `round_pt` precedent and R5.4 as written. Exactness is
  recomputed from the facts' own text (`register/u180_ops_check.py`: 853,598 inputs, negative thicknesses among
  them, no difference; two positive controls fail as they should; revision 5: "no difference" holds on the
  non-antialiased path for every thickness and on the antialiased path for `t > 0` only, and the controls are now
  in the script). The outcome is limited to QPainter's underline.
  U180 stays, narrowed and still kind `d`, for Qt Quick's line position (Qt Quick rounds the text's absolute
  position), QPainter's wavy line and the strike lines; none of them is on 0.8.0's path. The checker task revision 3
  added is withdrawn. What `check_facts.py`, `check_function_vectors.py`, `verify_facts.py` and `generate_preset.py`
  need is stated. The descent's documentation is no longer shown as a quote of `query/mod.rs`.
- **U505** (not one of the rows examined; U180's operator settles its form). Its timeout is restated with
  `if_le`, `= mul(p, if_le(n, 0, 3, n))`, WinUI's own test, and recomputed over every blink time from 0 to
  10,000 ms with no difference. The zero-fallback form, its `1`s and S17's exemption for them are withdrawn, and with
  them the checker task the fact waited on. U505 does not close: `solid_while_typing` stays open.
- **`single_click_activates`** (U504, U555), by decision C. No S4.2 sentence is written. The facts stay as they
  are. The question goes to the containers unit, which brings `list`, every platform's list, table, tree, grid and
  icon view; it goes with all the evidence the three checks gathered and with the model-form question. The part is
  rewritten as that record, with check 3's additions and corrections in it: WinUI's TreeView, Microsoft's Learn
  pages for ListView, GridView and ItemsView, AppKit's `NSWorkspace`, `NSBrowser` and `NSCollectionView`, the
  Kirigami flavour, and two citations (`Clickable.kt`, `LazyDsl.kt`).
- **U504.** Keeps `mnemonics` `never` and `mnemonic_when_disabled` `absent`; its `single_click_activates` stays
  pending. The two mnemonic tokens are on 0.8.0's path by the staging's revision 2.
- **U555.** No outcome this round: the row stays, its question moved (decision C). Its two corrected citations are
  kept in the evidence record.
- **U122.** Its reading of S6.3 is on the veto list.
- **U515, U1, U473.** Section 1's path column for U515 and U1, and U473's unit (the parking unit *behaviour*),
  follow the staging's revision 2; section 2.4 records how that revision takes up this document's four points.
- **Sections 1, 2.4, 4, 5 and 7** follow; section 6 answers all three checks. Section 7: the token's two items
  leave with it, U180's item is restated, U122's reading is added.

## Revision 3

Made after the narrow check of revision 2, `scratch-keep/d6/register-rows-check2.md` (1 serious, 4 should-fix,
6 minor), and two decisions of the orchestrator: (A) `single_click_activates` reads on item views by the model
criterion alone; (B) U180 takes the arithmetic form, provided the model's operators state Qt's rule exactly. Each
finding was verified at its cited lines before anything changed; section 6.2 lists the eleven. Both decisions are
applied: neither was found to state anything false (section 6.2, after the table). What changed, per row:

- **U180.** The outcome changes from a platform function to the model's arithmetic with today's operators: the
  ceiling written with `floor`, and a comparison selection, both named forms of S2.2. The form is shown exact in all
  three cases of QPainter's code, ties included, case by case and by recomputation
  (`register/u180_form_check.py`: 835,934 inputs over both pen paths, no difference). R5.4's rule and S4.2's reason
  against a function are quoted in full and weighed. The checker's recognition of form constants is a requirement
  shared with U505, with the three shapes it must recognise. `generate_preset.py`'s `FONT_METRICS` and
  `check_spec_names.py` are stated precisely. "No checker change" and "the only form checked against the platform"
  are withdrawn. The two line numbers revision 2 changed are restored (`kde-breeze.toml:465`,
  `kquickstyleitem.cpp:2096-2098`).
- **`single_click_activates`** (U504, U555). Three readings of which views count are set out with what each
  changes; the model criterion alone is applied. The S4.2 sentence is rewritten: it names each toolkit's item views
  and the containers that are none, says how a view is read (its default configuration with only the activation
  turned on, a configuration the toolkit's documentation forbids not counted; the documented convention where the
  toolkit's views have no activation; pending where one view's click and double-click actions are not told apart by
  a pinned source), and is tested on all six platforms in one table. GTK's citations corrected; Qt's quote exact.
- **U504.** `false` now follows from the sentence, not from this row's text. Confidence medium-high.
- **U509.** The Courier New hits left out of the table are seven pages on eight lines, not three.
- **U555.** `absent` becomes `true`, cited from the lazy list and Material's list item, with `absent =
  ["onDoubleClick"]`; Material's sample recorded.
- **U561, U559.** The class of lost icon contexts reaches `macos-*`, `windows-11` and `ios-*`, pending under U102,
  U516 and U127; the plan's amendment names the three rows.
- **U1.** Where its six facts fall is an open point of the staging design's per-unit comparison, for the three
  platforms the catalogue names (section 2.4, (d)).
- **Section 1.** One count of picks, matching section 7.
- **Section 2.4.** (d) restated as an open point; (b) gains its consequence for U180's due point; check 2's verdicts
  on the four disagreements added.
- **Sections 4, 5, 6, 7** follow; section 6 now answers both checks.

## Revision 2

Made after the independent check `scratch-keep/d6/register-rows-check.md` (3 serious, 8 should-fix, 6 minor) and
the coordinator's instructions. Each finding was verified at its cited lines before anything changed; section 6
lists the seventeen, taken or not. The staging design (`scratch-keep/d6/staging-design.md`, itself still being
checked) now exists and is compared in section 2.4. What changed, per row:

- **U180.** The three forms weighed again from scratch with the cost the check found: the constants no quote holds,
  what S17's quote check and the checker need, and the blink timeout still pending under U505 for want of a form
  the checker has never implemented. The choice changes: a platform function for QPainter's underline offset, in
  place of the long expression. `font_descent` stays the one new metric. The form is on 0.8.0's path through the
  link's underline, whatever happens to the mnemonic leaf. The antialiasing argument is corrected.
- **`single_click_activates`** (U504, U555). The token's meaning is settled first, once, in a part of its own
  (section 3, before U504): the one reading under which every stated fact is true as written. From it:
  `windows-11` stays `false`, on WinUI's exclusion of selection and click activation and no longer on defaults;
  `material-3` changes from `true` to `absent`. The S4.2 sentence is rewritten and tested against iOS; it is a
  class 2 change.
- **U504.** One more catalogue line to correct; the consequence for U518 kept.
- **U509.** The labelled mapping stays; the family is now presented as a pick among four candidates, each with its
  pinned sources and their use, one more pinned source found for Consolas (the Toolkit Labs' Markdown control); the
  facts' form is a keyed style with the slot referring to it; two more catalogue lines to correct.
- **U515.** The citation form for the zero is named. The staging note lists the row as leaving 0.8.0; section 2.4
  says why that does not follow from the note's own rule.
- **U681.** `absent = []` in the proposed citation.
- **U555.** See above: `absent`.
- **U559, U561.** The loss against 0.7.0 is carried to the plan's list; one sentence says why outcome 5 does not
  reach the two icon contexts.
- **U1.** The decision is needed by the containers unit of 0.8.0, though the six facts are not.
- **U424.** `auto`'s meaning stated generally.
- **U314, U614.** The image-name counts corrected; U614's citation is at `xcode@16.4`.
- **U122.** The 96 facts for a title on the three `UIButton` types stay pending, kind `b`: the pinned header
  declares a title setter for every type. The content rule covers 294 of the 564, not 389. No specification
  sentence is required.
- **U473.** Which staging the outcome assumes is stated, with its effect on `material-3`'s stated value and on
  `windows-11`'s U503.
- **Section 4** keeps only the claims against register rows that are established; two are withdrawn.

**How to read it.** Section 1 is the table of outcomes. Section 2 states the rules, what 0.7.0's model holds, and
the comparison with the staging design. Section 3 has one entry per row, each in five parts: (1) the row, its facts
and listed paths, and what the specification says; (2) the options; (3) the outcome; (4) what it requires next;
(5) confidence and what a veto would turn on. Section 4 lists what was found on the way. Section 5 lists what was
not verified. Section 6 answers the four checks' findings. Section 7 is the list for the maintainer's veto.

**Names of the files.** `[ledger]`, `[plan]`, `[rationale]` are
`docs/todo_v0.7.1_model-completeness-{ledger,plan,rationale}.md`; `[spec]` is `…-spec.md`, `[catalogue]` is
`…-spec-widgets.md`, `[trace-x]` and `[gap-x]` are `…-spec-trace-<x>.md` and `…-gap-map-<x>.md`; `[outcome]` and
`[brief]` are `scratch-keep/d6/d6-outcome.md` and `d6-decisions.md`; `[staging]` is `scratch-keep/d6/staging-design.md`
as read on 2026-11-10 (its line numbers may move); `[staging2]` is the same file's revision 2 as read on 2026-11-11,
cited by section and line; it was being edited during this revision (two reads an hour apart gave lines two
apart), so its lines are as of the later read. `[check3]` is `scratch-keep/d6/register-rows-check3.md`. Facts are
under `docs/facts/`, presets under
`native-theme/src/presets/`. A pinned source is written as the facts write it, `<id> <path>:<lines>`, and was read
in the source cache `/run/media/tibi/work/cargo-build/native-theme-sources/<id>/` (`scripts/citations.py:87`).
An Apple header is cited by its place in the cache (`xcode@26.6 AppKit/NSImage.h`), which the facts write with the
SDK's full path. `v0.7.0:<path>:<line>` is the released tree (`git show v0.7.0:<path>`). The task's brief names the
model's code as `native-theme/src/next/`; since M1a's switch it is `native-theme/src/` (`[plan]:93-94`).

**Counts** were made by the scripts in `scratch-keep/d6/register/`: `facts_by_id.py` (facts whose `pending` names
the id), `pending_tables.py` (the presets' `[pending]` entries naming it), `u122_breakdown.py` and
`u122_removable.py` (U122). `u180_ops_check.py` recomputes revision 4's U180 and U505 facts from their own text,
with its positive controls (revision 5); `u180_form_check.py` recomputed revision 3's withdrawn form.

---

## 1. The outcomes

| Row | Preset | Outcome, in one line | On 0.8.0's release path | Listed paths empty after it | Confidence |
|---|---|---|---|---|---|
| U180 | `kde-breeze` | One new font metric, `font_descent`, and two operators, `ceil` and `if_le` (decision D); QPainter's thickness and offset then in S2.2's arithmetic, exact in all three cases of Qt's code, with no literal to exempt. Class 2, then four facts; U505's timeout takes `if_le` in the same batch. The row stays, narrowed, kind `d`: Qt Quick's line position, QPainter's wavy line, the strike lines. | Yes for QPainter's underline: the link's underline is 0.7.0's and takes the same form, and the mnemonic leaf stays (`[staging2]`, section 4, `kde-breeze`, `:807`). The narrowed rest no | Yes, both, after the batch and the two button facts | High on the sources and on exactness; medium-high on the form |
| U504 | `windows-11` | `mnemonics` `never`, `mnemonic_when_disabled` `absent`. `single_click_activates` stays pending; its question goes to the containers unit (decision C, the token's part). | The two mnemonic tokens yes, by the staging's revision 2 (a connector draws the kept leaf by them, `[staging2]`, section 1.5, clause 4, `:279-282`), though 0.7.0 held neither; `single_click_activates` no (parked in *behaviour*, `[staging2]`, section 3.1, `:701`) | Two of three by the facts (`windows-11.toml:356`, `:358`); `:360` by the staging | High, high |
| U509 | `windows-11` | Not absent: a keyed style `monospace` as native-theme's labelled mapping (outcome 5), the body style in a fixed-width family. The family is a pick, Consolas recommended, listed for the veto with three other candidates. | Yes: 0.7.0 holds `mono_font` | Yes, once outcome 5's batch gives the label its form and the pick stands | Medium |
| U515 | `windows-11` | `1`, and `0` under `cond animations_off`: R9 already gives the factor that reading. Two facts. | Yes: the staging's revision 2 keeps `token.motion.speed_factor` at depth (`[staging2]`, section 4, `windows-11`, `:810`; section 2.4, (a)) | Yes, by the facts | High |
| U681 | `windows-11` | The keyed map is empty, `{}`: WinUI names no icon context; its sizes are per-control resources on their parts. One fact. | No by D13; the staging note counts it staged until the names are read, which this outcome does | Yes | High |
| U555 | `material-3` | No outcome this round: `single_click_activates` goes to the containers unit with the evidence the three checks gathered (decision C, the token's part). The row stays. | No: 0.7.0 held no such value; parked in *behaviour* (`[staging2]`, section 3.1, `:701`) | Yes, by the staging (`material-3.toml:290`) | — |
| U559 | `material-3` | Both `absent`, as the catalogue's absence table has it. Two facts; the plan's list of losses gains the value. | Yes: 0.7.0 holds `icon_theme` | Yes, both | High |
| U561 | `material-3` | `small` and `large` `absent` (the catalogue's table), a loss the plan's list gains, written for the class (which reaches `adwaita`, `macos-*`, `windows-11` and `ios-*`); `panel` `24px` from `md.comp.navigation-drawer.icon.size`. | Yes: 0.7.0 holds `IconSizes` | Yes, all three | High; medium-high for `panel` |
| U1 | `windows-11` | `context toolbar.bar` states the closed command bar (Compact); the open bar's FullSize is the rule under `context toolbar.bar {open}`. Decided now; needed by the containers unit. | Yes, with the containers unit: the staging's revision 2 has the row leave at the slice and return there, `tool` and `toolbar.bar` both at depth (`[staging2]`, section 4, `:830-832`) | It holds none | High |
| U424 | `kde-breeze` | A model form, never a measurement: `subtitle_gap` may be `auto`, "the subtitle centred in the space left below the title's line". Due with the unit that brings `@command_link`. | No | It holds none | Medium |
| U697 | `kde-breeze` | No catalogue split and no decision: the position is a value, and its default is in the sources. The pending fact goes; a Kirigami rule is added. | Yes since the fix round: `label_position` at depth (ledger queue #258) | It holds none | High |
| U314 | `macos-tahoe` | AppKit's own rendering, a new S15 member for the help bezel; R11 rejected a path traced from captures. Due with the unit that brings `help`. | No | With `help`'s unit; earlier under the staging note, which defers `help` and `route` | High |
| U614 | `macos-monterey` | The same, at `xcode@16.4`. | No | The same | High |
| U122 | `ios-*` | Splits. A subtitle, a menu indicator and the route picker's title are content the five forms cannot hold: no pending fact is owed (294 of 564, under S6.3 as it stands). The rest, the three button types' titles among it, is a measurement, kind `b`. | No: all five forms are beyond 0.7.0's depth | No: every path is shared with other rows | Medium-high |
| U651 | `ios-*` | Outcome 5's labelled mapping: the colour UIKit draws a filled button's title with, measured `#ffffff` in both schemes on both presets; checked again on U706's second-tint run. | Yes: 0.7.0 holds `accent_text_color` | Yes, all four, once outcome 5's batch gives the label its form | Medium-high |
| U473 | `ios-*` | 0.7.0's model holds no key-repeat value on any preset. A later unit's; nothing is the maintainer's to supply. It leaves by the staging of `token.behavior`, which also empties `windows-11`'s U503 path. | No | Yes, both, by the staging; no route otherwise | High |

**Not settled from the sources:** one row, U555, and one part of another, U504's `single_click_activates`: decision
C moves the token's question to the containers unit (the token's part). Six things are picks, choices of meaning or
choices of form, listed as such in section 7, items 1 to 6: the monospace family (U509); the form for QPainter's
rule (U180); U122's reading of S6.3's content rule; the pairing behind U651; the pairing behind `panel` (U561);
`auto` on `subtitle_gap` (U424). Section 7 adds two items that are no pick: an amendment of the plan's list of
losses (item 7) and the four points this document raised with the staging design, which its revision 2 takes up
(item 8). Three outcomes wait on a batch this document does not write: U509 and U651 on the form of outcome 5's
label, U473 on the staging.

---

## 2. The rules, what 0.7.0 holds, and the staging design

### 2.1 The rules, in the order the task gives them

1. Never invent a value: a value is quoted from a pinned source, measured, or the field is declared absent or
   unavailable with a citation that the platform has none.
2. Cover what the platform draws.
3. One rule for every model field: a field enters only when every preset states it (`[plan]:144-146`).
4. D13's scope: "every widget, colour role, token and value 0.7.0's model holds is in the new model, on every preset
   0.7.0 ships for" (`[plan]:489-491`), and under D6's outcome 2 a widget, role and token enters "at the depth
   0.7.0's model distinguishes" (`[outcome]:13`).
5. Prefer the smallest change to the model and the specification that states the truth.

### 2.2 Two rules that settle several rows at once

- R1.3: "A platform preset describes the platform's current native toolkit; the others are candidate presets of
  their own" (`[rationale]:166-170`, naming "WPF and Win32 controls on Windows"). `windows-11` is "Microsoft's
  Windows 11 (WinUI 3)" (`native-theme/src/presets/README.md`, the list's last row). So Win32's behaviour and the
  Win32 shell's values are no fact of `windows-11` (U504, U681, U509's console font).
- D6's outcome 5: "A typed role or slot that a portable application needs and the platform names no counterpart for
  is native-theme's mapping: labelled, outside the gate's platform comparison, never absent, never a stated fact"
  (`[outcome]:16`). It applies to U509 and U651 word for word. Its batch is not written: the label has no form in
  the facts format yet.

### 2.3 What 0.7.0's model holds for each row's field

The released tree, tag `v0.7.0`, caeddd6b.

| Row | Field | In 0.7.0's model | Evidence |
|---|---|---|---|
| U180 | the mnemonic's underline; a link's underline | The first no, the second yes | `mnemonic` occurs nowhere in `v0.7.0:native-theme/src`; `LinkTheme.underline_enabled`, a bool (`v0.7.0:native-theme/src/model/widgets/mod.rs:1062-1063`), is `true` on `kde-breeze` (`v0.7.0:native-theme/src/presets/kde-breeze.toml:465`, under `[light.link]` at `:464`), and the catalogue maps it to the link's `text.text.decoration`, "its thickness and offset as the `text` part says" (`[catalogue]:2196-2198`) |
| U504, U555 | mnemonics, single-click activation | No | searches of `v0.7.0:native-theme/src` for `mnemonic`, `single_click`, `singleclick`, and of the model, the connectors and the platform facts for `single.?click`, `double.?click`, `activate_on` find no value; `ThemeDefaults` holds no behaviour value (`v0.7.0:native-theme/src/model/defaults.rs:37-145`) |
| U509 | the monospace font | Yes | `ThemeDefaults.mono_font` (`defaults.rs:48`); `windows-11` states `Cascadia Mono`, 10.5 pt, weight 400, with no citation (`v0.7.0:native-theme/src/presets/windows-11.toml:82-85`) |
| U515 | the speed factor | Not as a factor; its switch, yes | no `speed_factor`; `AccessibilityPreferences.reduce_motion` is a flag, and 0.7.0's connectors hold the busy indicator still under it (`v0.7.0:connectors/native-theme-egui-widgets/src/spinner.rs:95`, `:112`, `:240-270`) |
| U681 | keyed icon contexts | No | `IconSizes` has five fields and no map (`v0.7.0:native-theme/src/model/icon_sizes.rs:13-29`) |
| U559 | the icon theme | Yes | `ThemeDefaults.icon_theme` (`defaults.rs:145`), `Theme.icon_theme`; `material-3` states `icon_theme = "material"`, its set's name (`v0.7.0:native-theme/src/presets/material-3.toml:9-10`) |
| U561 | the small, large and panel contexts | Yes | `IconSizes.small`, `large`, `panel`; `material-3` states 16, 32 and 24, the first two with no citation (`material-3.toml:75-82`); 0.7.0's connectors hand each context to applications (`v0.7.0:connectors/native-theme-gpui/src/geometry.rs:1155-1176`, `v0.7.0:connectors/native-theme-egui/src/icons.rs:26-36`) |
| U1, U697 | a tool button's padding and label position in a toolbar | No | `ButtonTheme` has no variant beyond `primary` and no label position (`widgets/mod.rs:43-103`); `ToolbarTheme` holds the bar's colour, height, item gap, icon size, font and border (`:547-572`) |
| U424 | the command link, a subtitle | No | `command_link` occurs nowhere in `v0.7.0:native-theme/src`; `subtitle` only as the name of Fluent's type-ramp style (`v0.7.0:native-theme/src/presets/windows-11.toml:103`) |
| U314, U614 | the help button | No | `ButtonTheme` has no variant; what 0.7.0 has is an icon role, `IconRole::Help`, "Help / question mark", SF Symbols' `questionmark.circle` (`v0.7.0:native-theme/src/model/icons.rs:136`, `:646`): an icon of the icon sets, not the help bezel's glyph |
| U122 | paste control, route picker, system button types | No | `ButtonTheme`'s fields, above (`paste` occurs only in the icon role `EditPaste`, `route` only in comments) |
| U651 | the accent's foreground | Yes | `ThemeDefaults.accent_text_color` (`defaults.rs:58`); `ios-13` states `#ffffff` in both variants (`v0.7.0:native-theme/src/presets/ios-13.toml:66`, `:412`) |
| U473 | the key repeat | No, on no preset | fourteen spellings searched (`key_repeat`, `KeyRepeat`, `InitialKeyRepeat`, `KeyboardDelay`, `autorepeat`, `repeat_delay`, `RepeatRate`, …) over `native-theme/src`, the presets, the connectors and `docs/platform-facts.md` at `v0.7.0`: no hit; the model's two `repeat` hits are doc comments on "repeated calls" (`v0.7.0:native-theme/src/model/font.rs:12`, `:23`). The check repeated the search with sixteen patterns over the whole tree and found none |

0.7.0 ships no `ios-26` (`[plan]:830`); D13 keeps it in 0.8.0, where "every preset that exists states every field
the model holds" (`[plan]:510-512`).

### 2.4 Compared with the staging design

Revision 1 said for several rows "unless outcome 2's batch stages it". The staging design now gives that batch a
rule (`[staging]`, 1.3 to 1.5) and lists every register row as staying or leaving (its section 4). Its rule in
three sentences: a widget's entry enters with every part, field and value, in the combinations of the names
0.7.0's model distinguishes (for `button`: `primary`, `toggle`, `with_icon`, seven states, the default size, no
host context in the slice); typed roles enter whole; a token path is at depth where the *today's fields* table
names it, a value at depth reads it, or the core reads it, and is otherwise "staged to the first unit that reads
it" (`[staging]:164-181`). At the slice that stages "all of `token.behavior` but `focus_visible_when`", `token.motion`
and the keyed entries nothing reads (`[staging]:193-198`).

| Row | This document | The staging note | Agree |
|---|---|---|---|
| U180 | on the path (the leaf is a path of `button.label`; the link) | stays, two listed paths (`[staging]:553`) | yes |
| U504 | not 0.7.0's; the mnemonic facts given (revision 4: `single_click_activates` moved, decision C) | leaves: `token.behavior` staged | yes on the row; see (b) |
| U509 | on the path | stays (`[staging]:556`) | yes |
| U515 | carries 0.7.0's reduce-motion switch | "leave 0.8.0's path whole" (`[staging]:663-664`) | **no**: see (a) |
| U681 | `{}`, a stated fact | counted as staged; "What settles it: a source read of the names alone" (`[staging]:1023-1026`) | yes: this outcome is that read; see (c) |
| U555 | not 0.7.0's; fact given (revision 4: moved, decision C) | leaves | yes |
| U559, U561 | on the path | stay (`[staging]:559`) | yes |
| U1 | the six facts off the path; the decision on it | leaves whole (`[staging]:557`) | in part: see (d) |
| U424 | off the path | leaves (`[staging]:555`) | yes |
| U697 | off the path | left (`[staging]:555`); on it since the fix round, `label_position` at depth (ledger queue #258) | yes, then both corrected |
| U314, U614 | off the path | leave (`[staging]:562-566`) | yes |
| U122 | off the path | leaves, with the identities (`[staging]:582`) | yes |
| U651 | on the path | stays whole (`[staging]:567`) | yes |
| U473 | a later unit's | leaves; the note's own example (`[staging]:183-187`) | yes; see (e) |

- **(a) U515.** The note keeps `dialog.values.presentations` and `spinner.values.loop` in 0.8.0 and says "The
  motion tokens those two values read (`token.motion.durations`, `.timings`, `.speed_factor`, `.color_space`), each
  entry with the first of the two units that reads it" (`[staging]:635-637`), and "A typed token field at depth on
  one preset is at depth on every preset" (`[staging]:179-180`). The specification expects a loop to read the
  factor: "a loop whose period evaluates to zero does not run […]: R9's animations off sets the speed factor, and
  so a scaled period, to zero" (`[spec]:573-575`), and 0.7.0's connectors hold the busy indicator still under
  reduced motion (section 2.3). So once any preset's loop or presentation reads the factor, `windows-11` states it
  too, inside 0.8.0, with the indicators or the overlays unit. The note's list has U515 leave whole. By the note's
  own rule the row leaves M2's task 0 and returns with that unit. Which holds is decided by those units' facts,
  which are not written.
- **(b) The mnemonic's tokens against its leaf.** The note keeps `button.label.text.mnemonic_decoration` (U180
  stays) and stages `token.behavior.mnemonics` and `mnemonic_when_disabled`, by which a connector decides whether
  to draw that decoration at all (`[spec]:3524-3525`). No expression reads the two tokens, so the note's closure
  does not see the dependence. 0.8.0 would hold the underline's geometry and not the statement of when it shows.
  This is the staging note's to resolve (the two tokens kept with the leaf, or the leaf staged with them). It does
  not move U180 off the path: the link's underline keeps the form on it either way (U180, part 3). It can move its
  due point: staging the leaf with its tokens would take `kde-breeze`'s two listed paths
  (`native-theme/src/presets/kde-breeze.toml:459`, `:461`) out of M2's task 0, and U180's batch would then be due
  with unit 5, which brings `link` (`[plan]:768`), not before.
- **(c) U681.** The note treats `windows-11`'s keyed icon contexts as "pending as whole maps […] `later` cannot list
  names nobody has read". They are read here: WinUI has none. Nothing is listed under `later`, and the map is `{}`.
- **(d) U1 and `tool`: an open point, not a necessity.** The note puts `toolbar.bar` at depth "from the containers
  unit on" for the variant sets at depth, and defers `tool` to the styles unit (`[staging]:125-126`, `:445`). The
  catalogue makes a toolbar's button `tool` on three platforms: "Qt's tool button, WinUI's AppBarButton and
  AppBarToggleButton and AppKit's toolbar bezel are the typed `button @ variant tool`" (`[catalogue]:4732-4734`).
  On `windows-11` all 31 facts in that context name `tool`: 14 under `tool` and 17 under `tool` with `menu`
  (`docs/facts/windows/button.toml`, recounted over the facts whose `when.context` names `toolbar`'s `bar`). 0.7.0's
  toolbar font and icon size map to `button @ context toolbar.bar` (`[plan]:732`; `[catalogue]:4729-4731`). Whether
  the containers unit needs `tool` at depth to state them is not settled by that: 0.7.0's `windows-11` states no
  toolbar font or icon size of its own (`v0.7.0:native-theme/src/presets/windows-11.toml:390-407`: a height, a gap
  and a border), so both were inherited from the defaults, which the new model keeps as the `body` slot and
  `token.icons.sizes.toolbar`. The staging note's comparison per unit decides it (`[staging]:974-978`): each
  difference between 0.7.0's value and what the preset at depth resolves is a correction of 0.7.0 by the facts or a
  name that belongs at depth, and a recorded loss instead is a change to R21's list and the maintainer's
  (`[staging]:146-147`). If that comparison brings `tool` to depth, it brings it on every preset: "A typed name is
  at depth on every preset or on none" (`[staging]:119`), so `kde-breeze` and `macos-*`, in the same position by
  the catalogue's line, gain their `tool` facts in that context with the containers unit too. U1's six facts are on
  0.8.0's path in that case only.
- **(e) U473's unit.** The note stages a token "to the first unit that reads it". For a field no value reads (the
  key repeat, single-click activation, the double-click time: a connector or an application reads them) it names no
  unit. U473's due point therefore has no name yet.

Check 2's verdicts on the four (`register-rows-check2.md:205-215`): this document is right on (a), (b) and (e), and
right on the gap in (d), which claimed more than the evidence gives (its finding 5); (d) is restated above as the
open point it is. (c) is no disagreement: the note asks for "a source read of the names alone"
(`[staging]:1026`), and U681's outcome is that read. Which way each is closed is the staging design's.

**The staging design's revision 2** (`[staging2]`, still being revised) takes up all four. The table above and the
four points compare with the 2026-11-10 text and are kept as the record. Section 1 cites `[staging2]` for the rows
it changes.
- (a): `token.motion.speed_factor` is at depth, "U515 stays on the path" (`[staging2]:26`; section 4,
  `windows-11`, `:810`).
- (b): a fourth clause of the token rule keeps `mnemonics` and `mnemonic_when_disabled` at depth with the kept
  leaf, "a token by which a connector draws a value at depth" (`:31`; section 1.5, `:279-282`). So U504 stays in
  part (`:810-811`), and U180's due point stays M2's task 0 (`:807`).
- (d): `tool` comes to depth "by the comparison of 0.7.0's `toolbar.font`" (`:23`); U1 leaves at the slice and
  returns with the containers unit (`:830-832`).
- (e): "Every staged path and every condition has a named later unit" (`:24`). The behaviour tokens no value reads
  wait in the parking unit *behaviour*, U473's key repeat and U504's `single_click_activates` among them (section
  3.1, `:701`).

Two points for the staging design that revision 5 adds (check 4, findings 2 and 11):
- *The flavours unit carries U180's open decision.* The staging ships it as "a patch release" (`[staging2]:755`).
  Qt Quick's decorations under `flavor:kirigami` need a model form first (U180, part 3), and of the two forms only
  a field on `Decoration` fits a patch release; a new element datum is a breaking change. That unit is therefore not
  "complete on their presets with published sources" until the form is chosen.
- *`[staging2]:807` will be stale once U180's batch lands.* It reads "Stay: U180(d) whole (the mnemonic decoration's
  operands)". U180 stays whole only until the batch: after it, the narrowed row has no listed path (U180, part 4),
  and its rest leaves 0.8.0's path, Qt Quick's line position with the flavours unit and QPainter's wavy and strike
  lines with the first part that states one (U180, part 3). This document does not edit the staging note.

---

## 3. The rows

### U180 — `kde-breeze`: the offset and thickness of text decorations

**1. The row.** `[ledger]:561`: "`kde-breeze`: every stated decoration's `offset` and `thickness` (a mnemonic's, a
link's, a spelling mark's, a strike line's) […] | no form in the model's operands (no ceiling, no ascent or descent
alone; spec S2.8, ADD-A130). Two drawing paths, one form each […] | decide the operands the forms need (an ascent
and a descent metric as new element data, and a ceiling) as a class 1 change, or a platform function per path
implemented to its source; a capture of each path's underline at two fonts confirms the choice".

- Facts: 4. `docs/facts/kde/button.toml:8768`, `:8773` (`button.label.text.mnemonic_decoration.thickness`,
  `.offset`, no `when`) and `docs/facts/kde/title_widget.toml:403`, `:408` (the same two leaves of
  `component.title_widget.title`, a later unit's).
- Listed paths: 2, `native-theme/src/presets/kde-breeze.toml:459`, `:461`.
- The specification: a decoration's `offset` is "the distance from the baseline to the line's top edge, positive
  downward", its `thickness` the line's height (`[spec]:2352-2354`); a platform that draws from the font's metrics
  states references to them, "a number is stated only where the platform draws a fixed line (R5.4)"
  (`[spec]:2378-2383`); Qt's two forms are described and left to "register U180's until the value phase states each
  path's form" (`[spec]:2388-2399`). The operands a part's expression may read (`[spec]:281-297`): of the font,
  `font_height`, `font_x_height`, `font_underline_offset`, `font_underline_thickness`, `font_strikeout_offset`,
  `font_strikeout_thickness`, the widths. The arithmetic is `mul`, `div`, `add`, `sub`, `min`, `max`, `floor`,
  `round` (`[spec]:464`).
- The source, read at the pin. QPainter (`qtbase@6.12.0 src/gui/painting/qpainter.cpp:6120-6131`): the pen is
  `qMax(fe->lineThickness().round(), QFixed(1))` wide where the painter does not antialias (`QFixed::round` rounds
  a half up, `((val)+32) & -64` in 64ths, `src/gui/painting/qfixed_p.h:46`), and keeps the unrounded thickness where
  it does (`:6089`, `:6122`); the line's centre lies
  at `std::ceil(underlineOffset) + lineThicknessOffset`, and `if (underlineOffset <= fe->descent().toReal())` it is
  limited to `fe->descent().toReal() - lineThicknessOffset`. The strike line is moved `- fe->ascent().toReal() / 3.`
  (`:6151-6153`). `underlineOffset` is the font engine's `underlinePosition()`, which FreeType's engine sets to the
  line's top, `center_position - line_thickness / 2` (`src/gui/text/freetype/qfontengine_ft.cpp:1026-1028`), the
  model's convention. Qt Quick rounds the line's position, `qRound(rect.y() + m_currentLine.ascent() + offset)`,
  keeps the thickness, and takes the strike line from `rawFont.ascent() / -3.0`
  (`qtdeclarative@6.12.0 src/quick/items/qquicktextnodeengine.cpp:208-218`, `:411-427`).
- What every form needs: the descent. In the model's terms, with `t` the font's line thickness, `u` its underline
  offset and `d` its descent, QPainter's thickness is `max(round(t), 1px)`, which today's operators write and whose
  one literal the quote holds; its offset is the ceiling of `u`, and where `u ≤ d` the smaller of that and `d` less
  the thickness. The ascent is `font_height − font_descent`, since the trait defines `height` as "ascent plus
  descent" (`[spec]:4467`). So one new metric, not two.
- What the quote check asks (the check's finding 1, verified). S17: "every literal of `value` […] equals a literal
  of the union of those citations' quotes", with one exemption, "the constants of S2.2's named integer forms, the
  `1`s of the zero-fallback form […] the checker recognising the form by its shape" (`[spec]:6876-6885`). The
  checker has no such recognition: `scripts/check_fact_citations.py` holds `quote_literals`, `string_literals` and
  `value_literals` and names no form. The one fact that needs the exemption is still pending for it: `windows-11`'s
  `blink_timeout` "stays pending (U505) until scripts/check_fact_citations.py leaves the zero-fallback form's 1s
  unchecked" (`docs/facts/windows/tokens.toml:2711-2718`; `[ledger]:667`). The quote for QPainter's rule holds two
  literals, `1` and `2.0`.
- The link. 0.7.0 holds a link's underline on `kde-breeze` (section 2.3); `link` is in 0.8.0's unit 5
  (`[plan]:768`); the trace's K1.260† lines state `link.text.text.decoration` for KUrlLabel (`[trace-kde]:409-411`),
  a Qt Widgets part, which QPainter draws (`[spec]:2399-2400`).

**2. The options.** Below, `u`, `t` and `d` are the font's underline offset, line thickness and descent
(`element.font_underline_offset`, `element.font_underline_thickness`, `element.font_descent`), `w` the pen's width.
Under every form the thickness fact is the same, `max(round(element.font_underline_thickness), 1px)`, written `W`
below. Its one literal, `1px`, equals the `1` of `QFixed(1)` in the quoted line (`qpainter.cpp:6123`), since a
quote's literal without a unit takes the value literal's dimension (`scripts/check_fact_citations.py:191-193`,
`equal`, `:643-654`). The forms differ in the offset. Two rules of the specification bear on every one of them:
- The grammar has one production for every function, `call := name "(" expr { "," expr } ")"` (`[spec]:326`), and
  no names for subexpressions (`[spec]:3434`), so a repeated subexpression is written out each time.
- S2.2's precedent for an operator against an encoding. For `round_pt` the specification added an operator and
  rejected two alternatives: "rounding in the unit a font size is stated in (it rounds the reader's px where the
  preset rounds points, and is written `mul(mul(1pt, round(div(<size>, 1pt))), <factor>)` to escape that, whose
  `1pt` the quote check matches only through the source's `%1pt` placeholder)", and "an S17 quote-check exemption
  for unit literals (a `1pt` or `1px` operand can be a platform value, and which role a literal plays is no notation
  the checker reads)" (`[spec]:465`).

The forms, with the offset fact's length (from `register-check3/u180_alternatives.py` and
`register/u180_ops_check.py`):

- **(a) Today's operators, revision 3's form** (418 characters). Three pieces:
  - the ceiling `C = sub(0px, floor(sub(0px, u)))`, since `−floor(−x)` is the ceiling of every real `x`;
  - a factor `F = min(1, max(0, sub(0, floor(div(sub(d, u), 1px)))))`, 1 where `u > d` and 0 otherwise;
  - the offset `add(mul(F, C), mul(sub(1, F), min(C, sub(d, W))))`, with `F` written twice.

  It needs two named forms in S2.2's row (the ceiling and the selection), and S17's exemption extended to their
  constants (the zeros, `F`'s `0`s and `1`s, the divisor `1px`). The checker must recognise three shapes, the
  zero-fallback form U505 waits for among them, and test the two `F`s for structural identity. That is the
  unit-literal exemption `[spec]:465` rejects, for `0px` and `1px`.
- **(a2) Today's operators, one selector** (316 characters; `[check3]`, finding 3):
  `sub(C, mul(G, max(0px, sub(C, sub(d, W)))))`, with `G = min(1, max(0, add(floor(div(sub(d, u), 1px)), 1)))`, 1
  where `u ≤ d`. The same exemptions and named forms, one shape fewer, no identity test.
- **(b) `ceil` added, the selection kept as a named form** (358 characters; 273 with (a2)'s selector). One
  operator, and still a named selection whose `0`, `1` and divisor `1px` S17 must exempt, against `[spec]:465`.
  (The constants could be built from operands, `sub(x, x)` and `div(x, x)`; that would pass the quote check by
  hiding what it exists to see.)
- **(b′) `ceil` and one comparison-and-selection operator added**, `if_le(a, b, x, y)`, "`x` where `a ≤ b`, else
  `y`" (215 characters): `if_le(u, d, min(ceil(u), sub(d, W)), ceil(u))`, Qt's lines in their own order. It holds
  no constant beyond `W`'s `1`, needs no named form and no exemption. It needs two entries in S2.2's arithmetic,
  their typing and totality, and their unit tests, and it answers S4.2's rejection of a conditional (part 3). Its
  comparison serves U505 too: WinUI's fallback is `if (nPeriods <= 0)` (`TextBoxView.cpp:2756`), so the timeout's
  count is `if_le(n, 0, 3, n)`.
- **The comparison's form, within (b′).**
  - (i) `if_le`, which returns one of its operands.
  - (ii) A step function `le(a, b)` giving 1 or 0, with the selection written in arithmetic. It brings back
    `sub(1, F)` and a `1` to exempt.
  - (iii) `if_lt`. It writes both sources' `<=` with operands and branches exchanged (`if_lt(d, u, ceil(u),
    min(…))`), against the sources' order.
  - (iv) A selection by sign, `if_nonpos(x, y, z)`. It writes Qt's test as `sub(u, d)`: one operand fewer, one
    subtraction more.

  (i) states both tests as written (`qpainter.cpp:6129`, `TextBoxView.cpp:2756`). The other orders follow from it by
  exchange (`a < b` is `if_le(b, a, y, x)`). Equality, which no source needs, would take two.
- **(c) A platform function for QPainter's underline offset,** `qt_painter_underline_offset(offset, thickness,
  descent)`, revision 2's outcome (131 characters).
  - Specification, class 2: the operand `font_descent` and its trait method; one row in S2.2's function table.
    No named form, no change to S17.
  - Checker: none for the form: the fact's value holds a name and three operands and no literal, and "references
    and names are left to the verifier" (`[spec]:6882`). `check_function_vectors.py` asks for its table and
    test as for every platform function (`[plan]:1341-1345`).
  - Model: one function in `native-theme/src/platform/`, written through D10's clean room, since Qt is an LGPL
    reference (`[plan]:450-463`), as `kirigami_label_icon_size.rs` was; a vector table from `tools/kde-vectors`; a
    test. The metric.
  - For it: a short fact, and a statement checked against Qt by vectors, "built from its reference at the pinned
    release" (`[spec]:581-583`), the plan's own concern for a derivation (`[plan]:1343-1345`).
  - Against it, the rationale: "A platform whose arithmetic differs gets its own function" (R5.4,
    `[rationale]:642`). QPainter's arithmetic does not differ in kind from S2.2's: its steps are a ceiling, a
    comparison and a minimum, which (b′) states with one definition each.
  - Against it, S4.2's reason for the blink timeout, in full: "Rejected: a platform function (S2.2's closed set: a
    function, a vector table, a generator or published rows and a test, for a rule the existing operators state
    exactly, and it would hide the timeout and the fallback, two values the facts can cite, inside code)"
    (`[spec]:3439-3441`). Its first half carries over: (c) is a function, a vector table, a route of generation and
    a test, for a rule S2.2's arithmetic states exactly (today's by (a), with two more operators by (b′)). Its
    second half does not: QPainter's rule holds no value the facts can cite (of its two literals the `1` stays
    visible in the thickness, and the `2.0` halves the pen, which cancels in the top-edge convention). Revision 2
    weighed the second half alone.
  - Against it, its check: the vector table cannot be shown for all three cases. `drawTextItemDecoration` is
    file-static (`qpainter.cpp:68`, `:6075`), so the compiled-source route would have to compile the whole of
    `qpainter.cpp` into its harness; the public-API route (the three metrics are public, `qfontmetrics.cpp:1170-1175`,
    `:1807-1812`, `:1842-1847`, and the drawn line can be recorded; `tools/kde-vectors` runs the system's Qt 6.12.0
    on the offscreen platform at 96 DPI with Noto Sans and Hack, `tools/kde-vectors/README.md:32-33`) gives the rows
    the container's fonts give. No font is
    known to put its underline below its descent (case 3), and the ties are reached by chance. So the vectors would
    check two of its three cases, which the row's capture checks under the arithmetic forms as well.
  - And it needs a clean-room description, an implementer and a harness: three steps the arithmetic forms do
    without.
- **(d) A conditional as a grammar construct** (`a <= b ? x : y`, or a boolean type with comparison operators and an
  `if`). S4.2 rejected it: "a general grammar construct, a comparison operator and S12's conservative checks over
  both branches" (`[spec]:3441-3443`). (b′) gives the same selection inside the existing `call` production, with one
  comparison, and with operands S12 checks as it checks any function's.
- **(e) The font's own metrics, a plain `Underline`.** No change. False: "Neither is `QFontMetrics`' `underlinePos`
  or `strikeOutPos`" (`[spec]:2396-2397`).
- **(f) A measured number.** One font, one number; the value follows the font, which "cannot be stated as a result"
  (R5.4, `[rationale]:621-622`).
- **(g) `absent`, or change nothing.** `absent` is false: Breeze underlines a mnemonic
  (`docs/facts/kde/tokens.toml:2047-2051`, `mnemonics` is `on_alt`). Changing nothing leaves `kde-breeze` unable to
  load (`[plan]:996-1001`).

**Exactness of (b′), case by case** (`qpainter.cpp:6121-6130`). The top edge is the centre less half the pen,
`adjustedUnderlineOffset − w/2`, the model's convention (`[spec]:2352-2354`). The forms (a), (a2) and (b) are exact
too (revision 3, `register/u180_form_check.py`; `register-check3/u180_alternatives.py`), so exactness does not
choose among the arithmetic forms.
- *The thickness* (`:6122-6123`). Qt's `w` is `max(round(t), 1)` with a half rounded up; the model's `round`
  rounds a half away from zero (`[spec]:464`). They agree for every `t ≥ 0`. For every `t < 0.5` both give at most
  0, so the maximum is 1 either way: `W` is `w` for every `t`.
- *Case 1: the ceiling alone* (`u ≤ d`, the `if` of `:6129` taken, `qMin` returning its first operand at `:6130`).
  Qt gives `ceil(u) + w/2 − w/2` = `ceil(u)`, where `ceil(u) + w/2 ≤ d − w/2`, that is `ceil(u) ≤ d − w`. In the
  form, `if_le` selects `min(ceil(u), d − W)`, which is `ceil(u)`.
- *Case 2: the limit to the descent* (`u ≤ d`, `qMin` returning its second operand). Qt gives `d − w/2 − w/2` =
  `d − w`. In the form, the minimum is `d − W`. At the tie `ceil(u) = d − w` both operands of `qMin` are equal, and
  so are the two results.
- *Case 3: an offset beyond the descent* (`u > d`, the `if` of `:6129` not taken, `:6128` alone). Qt gives
  `ceil(u)`, and `if_le` selects `ceil(u)`. At the tie `u = d`, Qt's `<=` takes the limit and `if_le` selects its
  first branch: the same.
- *Under antialiasing* (`:6122` false), for `t > 0`, the pen keeps the width `:6089` sets, `t`. The same form with
  `element.font_underline_thickness` in `W`'s place states it, by the same cases. The antialiased form is stated
  for `t > 0` only:
  - QPen ignores a negative width: `if (width < 0.f || width >= (1 << 15)) { qWarning(…); return; }`
    (`qtbase@6.12.0 src/gui/painting/qpen.cpp:658-663`). For `t < 0` the pen keeps the width of the painter's pen
    it was copied from (`const QPen oldPen = painter->pen();`, `QPen pen = oldPen;`, `qpainter.cpp:6084`, `:6087`),
    which is the caller's and no font metric (1 for `QPen()` and `QPen(QColor)`, "with 1 width", `qpen.cpp:225`,
    `:250`).
  - A zero width is a cosmetic pen: "a pen with zero width is equivalent to a cosmetic pen with a width of 1 pixel"
    (`qpen.cpp:488-490`; also `:648-651`, `:792`). For `t = 0` Qt centres the line by a half-width of zero
    (`:6124`) and draws it 1 px thick.
  - The model gives a thickness of `0` in both (a decoration's `thickness` is an unsigned length,
    `native-theme/src/blocks/text.rs:453`, where `offset` is `#[parts(signed)]`, `:458`; the bound zeroes a
    negative, `native-theme/src/resolve/evaluate.rs:161-166`, `[spec]:570-571`). So on this path the fact differs
    from Qt's line wherever `t ≤ 0`.
  - FreeType's thickness is the face's own (`src/gui/text/freetype/qfontengine_ft.cpp:1026`), so `t ≤ 0` is not
    excluded by the engine. Neither flavour's label painter antialiases (part 4), so the facts written here take
    the non-antialiased form, which is exact for every `t`.
- *In floating point.* Qt computes in `qreal`, the model in `f32` (`native-theme/src/units.rs:5`). The inputs are
  the font engine's `QFixed` values, multiples of 1/64 px (`qfixed_p.h:46`), which `QFontMetricsF` returns
  unchanged (`src/gui/text/qfontmetrics.cpp:1170-1175`, `:1807-1812`, `:1842-1847`). A multiple of 1/64 below 2^18 px
  in magnitude fits `f32`'s 24-bit significand. So for metrics below 2^17 px every difference, ceiling and minimum of
  the form is exact, and `if_le` returns an operand unchanged.
- *Recomputed.* `register/u180_ops_check.py` parses the two fact texts of part 4 (the offset and its antialiased
  variant) and evaluates them in `f32` step by step. Each step is made finite as the evaluator makes it
  (`native-theme/src/expr/arith.rs:8-11`; `eval.rs:495-506`), and the thickness takes the field's non-negative
  bound. They are compared with the line Qt's code draws (`qpainter.cpp:6084-6130`, QPen as above), evaluated in
  double. 426,799 triples, multiples of 1/64 px, with `u` and `d` from −3 to 8 px and `t` from −2 to 4 px, each on
  both pen paths (853,598 inputs):
  - *Not antialiased*, every `t`: 426,799 inputs; the three cases 128,959, 89,407 and 208,433 times; the ties
    `u = d` 9,231 times and `ceil(u) = d − w` 1,371 times, `t` at a half 8,075 times, `t` negative 141,435 times.
    No difference.
  - *Antialiased, `t > 0`*: 284,057 inputs; the three cases 81,663, 63,619 and 138,775 times; the ties `u = d`
    6,092 times and `ceil(u) = d − w` 440 times, `t` at a half 5,400 times. No difference.
  - *Antialiased, `t ≤ 0`*, outside the form's stated domain: 142,742 inputs (1,307 with `t = 0`, 141,435 with
    `t < 0`), and every one differs: Qt draws a line at least 1 px thick (the cosmetic pen, or the painter's pen
    of width 1 the script takes), the fact a thickness of 0.

  Two positive controls, each mutating both facts, the offset and its antialiased variant, fail as they should:
  `floor` in place of `ceil`, 359,800 differences not antialiased and 235,112 antialiased with `t > 0`; the
  comparison's operands exchanged, `if_le(d, u, …)`, 288,609 and 196,302. The script checks the facts against a
  transcription of Qt's lines, not against Qt running.

**3. The outcome: (b′), the orchestrator's decision D: `font_descent`, `ceil` and `if_le`, for QPainter's
thickness and offset.** Rule 1 excludes (e), (f) and (g). Exactness holds for every arithmetic form, so it does not
choose among them (`[check3]`, finding 3). What chooses:

- *The quote check, by the specification's own precedent.* (a), (a2) and (b) write constants no platform line
  holds (`0px`, `0`, `1`, the divisor `1px`), and they need S17 to exempt them by shape. `[spec]:465` rejected an
  exemption for unit literals, and it chose an operator for `round_pt`. (b′) holds no such constant: its one literal
  is Qt's.
- *R5.4, as written.* "Their functions are a closed set, each with one definition and its source", and "a function
  is added only when a platform's rule needs it" (`[rationale]:632`, `:642-643`). The set includes `alpha` and
  `mix` (`:633-635`), which S2.2 counts as the model's own arithmetic (`[spec]:586-588`), so the rule governs the
  model's arithmetic too. R5.4's list for lengths and durations, "multiples of the font size, the line height or a
  token, and products of tokens" (`[rationale]:639-641`), holds no rounding and no comparison: it has been narrower
  than S2.2 since ADD-A56, S2.2's arithmetic row holding `min`, `max`, `floor` and `round` (`[spec]:464`; `round`
  beside `floor` by ADD-A122), and adding a function is class 2 by the plan's own text, "Model changes (a field, a
  condition, a function …)" (`[plan]:1383`). Its condition holds for both operators: QPainter's rule is written with `std::ceil` and a `<=` (`qpainter.cpp:6128-6129`), and WinUI's with
  `nPeriods <= 0` (`TextBoxView.cpp:2756`). Each operator has one definition, C++'s. Revision 3 read "needs" as
  "cannot be stated exactly otherwise". That reading would prefer any exact encoding to a one-word operator, and
  `round_pt` shows the specification does not read it so.
- *S4.2's rejection of a conditional* named three costs: "a general grammar construct, a comparison operator and
  S12's conservative checks over both branches, for one use" (`[spec]:3441-3443`).
  - `if_le` is no grammar construct: it is a `call` (`[spec]:326`).
  - It is one comparison.
  - S12 checks its four operands as it checks every function's, and S10 declares the element data of both
    branches, whichever is selected. The evaluation is total (*Totality*, `[spec]:566-577`), so no branch can
    fail.
  - It has two uses.

  The rejection's other premise, "where a conditional count's fallback is arithmetic", rested on S17 exempting the
  zero-fallback form's `1`s, which the checker never implemented (`[ledger]:667`).
- *Against (c).* A function, a vector table, a route of generation and a test, with a clean room (D10,
  `[plan]:450-463`), for a rule the arithmetic states; and its vectors reach two of its three cases (part 2).
- *Against (b′).* Two operators to implement and mirror where (a) adds none (part 4), and one more operator a
  later fact may reach for. Each costs what `floor` costs. R5.4 bounds additions, not uses ("a function is added
  only when a platform's rule needs it", `[rationale]:642-643`): once `if_le` exists, R5.4 does not limit what a
  fact selects with it. What bounds its use is the sentence its row carries (part 4, item 3): it states a comparison
  the platform's code makes and the citations quote, and a value that changes with a state, a setting or a
  condition the model names is a rule under that condition, never an `if_le` on the token the condition sets.

Revision 3's (a) is withdrawn on check 3's finding 3. Of the arithmetic forms it carried the most mechanism: two
named forms, three checker shapes, an identity test, and the unit-literal exemption `[spec]:465` rejects. And it
gave the least readable fact. Revision 2's (c) stays withdrawn for revision 3's reasons (check 2's findings 3 and
4). The row's "class 1" is not the plan's class: an operand, a trait method and two operators are class 2
(`[plan]:1383-1385`).

On the release path: yes, twice over. The mnemonic leaf is a path of `button.label`, and the staging keeps it
with its tokens (section 2.4). And the link's underline is 0.7.0's on `kde-breeze`, in unit 5, drawn by the same
QPainter code: staging the mnemonic leaf out would not take the form off 0.8.0's path.

**What the outcome does not cover: U180 stays, narrowed** (`[check3]`, finding 4). The S2.8 sentence that leaves
`kde-breeze`'s decorations to U180 covers both drawing paths and every decoration (`[spec]:2396-2408`;
`[ledger]:561`). This outcome states QPainter's underline only. What stays in the row, which keeps kind `d` for
Qt Quick's line position:

- *Qt Quick's line position.*
  - Qt Quick sets the line's top to `qRound(rect.y() + m_currentLine.ascent() + offset)`
    (`qtdeclarative@6.12.0 src/quick/items/qquicktextnodeengine.cpp:213`). `rect.y()` is
    `m_position.y() + m_currentLine.y()` (`:276`), the text's and the line's place in the item.
  - The rounded value is a whole pixel in the item, so its distance from the baseline varies with where the
    baseline lies. No font metric gives that, and `font_descent` does not make it statable.
  - A measurement gives one position's value (rule 1; R5.4: "a value that follows … cannot be stated as a result",
    `[rationale]:621-622`). So the parts that need the position need a model form, class 2: an element datum for
    the baseline's place (the offset then `sub(round(add(<it>, u)), <it>)`), or a decoration field by which the
    connector puts the line's top on a whole pixel. Choosing between a measurement and a model form is a decision
    of the register's kind `d` (`[ledger]:472-474`).
  - The choice also decides when the form can ship. It is due with the flavours unit, which the staging ships as
    "a patch release" (`[staging2]:755`). A new element datum is a new member of `ElementDatum`, a public enum
    without `#[non_exhaustive]` (`native-theme/src/expr/mod.rs:157-158`, under `pub mod expr`,
    `native-theme/src/lib.rs:111`); a datum the connector measures is also a new method of `FontMetrics`, a public
    trait with no default methods (`native-theme/src/query/mod.rs:27-46`, re-exported at `lib.rs:300`). Either is a
    breaking change. A field on `Decoration` is not, since `Decoration` is `#[non_exhaustive]`
    (`native-theme/src/blocks/text.rs:442-443`). So in a patch release only the decoration field is allowed, unless
    0.8.0 makes `ElementDatum` `#[non_exhaustive]` and gives the trait's new method a default.
  - This is an open point of the flavours unit, not a pick made here; the unit's Qt Quick decorations are not
    "complete with published sources" until it is settled. (The element datum could otherwise land in 0.8.0
    itself or in a later breaking release, outside that unit's patch release.)
  - Qt Quick's thickness is statable as the model stands: the font's own, the thickest of consecutive underlined
    runs (`:214`, `:413-415`). For a part of one font it is `= element.font_underline_thickness`, a facts edit with
    each part's unit.
- *Which facts.* None is written. KDE's facts hold the button and the title widget (`docs/facts/kde/`). The
  button's label takes QPainter's form under both flavours, since Kirigami's button label is drawn through QStyle
  (`[spec]:2399-2403`); its two facts carry no `when` (`docs/facts/kde/button.toml:8768-8776`). The parts S2.8
  names as Qt Quick text are all under `cond flavor:kirigami` (`[spec]:2403-2408`):
  - the check box's, radio button's, switch's and menu item's labels;
  - the editable combo box's text field;
  - the text area's text and marks, Q1.13's `text_area.misspelling.text.decoration.{thickness,offset}` among
    them (`[trace-kde]:478`).
- *The strike lines, both paths.* QPainter centres its strike line `- fe->ascent().toReal() / 3.` from the baseline
  (`qpainter.cpp:6153`), with the unrounded pen (`:6145`). Its top edge halves the pen, a `2` no line of the strike
  holds (the underline's `pen.widthF() / 2.0`, `:6124`, halves the same pen); that is the strike's unit's to settle,
  by S17's quote check as it stands. Qt Quick puts the strike line's top at `rawFont.ascent() / -3.0`
  (`qquicktextnodeengine.cpp:426`) and rounds it as it rounds the underline (`:402`, `:213`), so it has the underline's position question.
- *QPainter's wavy line* (`qpainter.cpp:6105-6118`). A pixmap of a wave, its amplitude
  `qMin(qMax(underlineOffset, pen.widthF()), maxHeight / qreal(2.))`, `maxHeight` the descent less one (`:6107`,
  `:6114`), drawn from one pixel below the baseline and cut at `qFloor(maxHeight)` (`:6106`, `:6118`). How `offset`
  and `thickness` describe it is the spelling mark's unit's question.
- *On 0.8.0's path: none of them.*
  - 0.7.0's model holds no mnemonic, spelling mark or strike line (section 2.3; `git grep -n -i -E 'strike|spell'
    v0.7.0 -- native-theme/src/model` finds a test's name only, `mod.rs:1647`). It holds a link's underline only as
    a bool.
  - Every Qt Quick part is under `flavor:kirigami`, which the staging's revision 2 stages whole as the *flavours*
    unit (`[staging2]`, section 3.1, `:692`); while that unit is off, `set_flavor` refuses the flavour (`:524-526`).

**4. What it requires.**

*The specification change: class 2* (`[plan]:1383-1385`: "Model changes (a field, a condition, a function, a rule
in S1–S21)"), one batch with its S22 line and independent checks of the batch until one finds nothing serious. It
is not class 3: R5.4's decision stands as written (`[rationale]:628-643`: expressions, a closed set of functions,
each with one definition and its source, a function added only where a platform's rule needs it), and the
rejection it revisits is S4.2's, not a rationale decision. The batch, in the specification's words, each place
quoted as it stands:

1. **The descent.**
   - S2.2's `"element." data` production lists, after `font_height`, `font_descent`, and the `widget.<part>.` list
     gains `.font_descent` (`[spec]:281-297`).
   - S8's `FontMetrics` (`[spec]:4464-4472`) gains `fn descent(&self, font: &ResolvedFont) -> Px;`, commented "the
     font's descent, positive below the baseline". The code's trait (`native-theme/src/query/mod.rs:26-46`) gains
     the same method. Its documentation is new text, "The font's descent, in logical px, positive below the
     baseline.", beside `height`'s "Ascent plus descent." (`:34`).
   - S10's declared data and S12's checks follow, as for every element datum.
2. **`ceil`, in S2.2's arithmetic row** (`[spec]:464`, which now reads "`mul(x, y)`, `div(x, y)`, `add(x, y)`,
   `sub(x, y)`, `min(x, y)`, `max(x, y)`, `floor(x)`, `round(x)` | number, length, duration or font size |
   arithmetic […]. `floor` rounds toward negative infinity; `round` to the nearest integer, a half away from zero
   […]. Both work in their operand's unit as it reaches the function […]"):
   - the first cell gains `ceil(x)` after `floor(x)`;
   - the definition gains, after `floor`'s: "`ceil` rounds toward positive infinity, as C++'s `std::ceil`
     (QPainter's underline offset, `qtbase@6.12.0 src/gui/painting/qpainter.cpp:6128`, U180)";
   - "Both work in their operand's unit" becomes "All three work in their operand's unit".
3. **`if_le`, a row of its own after it** (`core_functions` reads one name and its arity per backticked signature
   of a row's first cell, `scripts/check_facts.py:538-549`):

   > | `if_le(a, b, x, y)` | number, length, duration or font size | `x` where `a ≤ b`, else `y`: a C++ `a <= b ? x :
   > y`, as QPainter tests its underline's offset against the descent (`qtbase@6.12.0
   > src/gui/painting/qpainter.cpp:6129`) and WinUI its blink count against zero (`microsoft-ui-xaml@2.5.1
   > src/dxaml/xcp/core/native/text/Controls/TextBoxView.cpp:2756`); `a` equal to `b` selects `x`. `a` and `b` are
   > compared, and `x` and `y` returned, in their units as they reach the function, as `floor` takes them; the
   > result is `x` or `y` unchanged. The other orders are written with it by exchange: `a < b ? x : y` is
   > `if_le(b, a, y, x)`, `a >= b` is `if_le(b, a, x, y)`, `a > b` is `if_le(a, b, y, x)`. Both `x` and `y` are
   > operands: S12 checks them as every function's, and their element data are the expression's (S10), whichever
   > is selected. A C++ fallback `n <= 0 ? k : n` is `if_le(n, 0, k, n)` (S4.2's blink timeout). `a` and `b` are of
   > one type, a number, length, duration or font size, typed apart from the type the place expects; `x` and `y`
   > are of one such type, typed as the place expects; a font size and a length in either pair are taken as px.
   > `if_le` states a comparison the platform's code makes between two values the model holds, and the fact's
   > citations quote it; where the platform's value changes with a state, a setting or a condition the model names
   > (S7.1, S7.2), the value is a rule under that condition, not an `if_le` on the token that condition sets (R9's
   > speed factor among them) |

   The batch's checks test both uses against that sentence: U180's `u <= d` (`qpainter.cpp:6129`) and U505's
   `nPeriods <= 0` (`TextBoxView.cpp:2756`), each a comparison in the platform's code that the fact's citation
   quotes. A checker rule is not needed for two uses.

   The arithmetic row's last sentence, "a fallback `k` for a non-negative integer `n` that is zero, `n > 0 ? n : k`,
   is `add(n, mul(k, sub(1, min(n, 1))))`, its `1`s the form's own constants (S17's quote check; S4.2's blink
   timeout; ADD-A134)", is withdrawn: that fallback is `if_le`'s last example.
4. **Typing** (`[spec]:546-549`, which reads "in `add`, `sub`, `min` and `max`, and only there, a bare number takes
   the type of the other operand […]; `mul(T, number)`, […] `floor(T)` and `round(T)` give a `T` […]; where one
   operand of `add`, `sub`, `min` or `max` is a font size and the other a length, the length is taken as px and the
   result is a font size"):
   - "in `add`, `sub`, `min` and `max`, and only there" becomes "in `add`, `sub`, `min`, `max` and each of
     `if_le`'s two pairs (`a` with `b`, `x` with `y`), and only there";
   - "`floor(T)` and `round(T)` give a `T`" becomes "`floor(T)`, `ceil(T)` and `round(T)` give a `T`,
     `if_le(T, T, U, U)` a `U`";
   - the font-size rule names `if_le`'s pairs beside `add`, `sub`, `min` and `max`: in either pair a length beside a
     font size is taken as px, and in `(x, y)` the result is then a font size; `(a, b)` gives no result, so there
     the rule only says how the two are compared;
   - at `:564`, "`floor` and `round` take a font size so, in px" becomes "`floor`, `ceil` and `round` take a font
     size so, in px, and `if_le` compares one so".

   Pairs typed as `min`'s, rather than strictly, so that a comparison with a bare zero reads as `max(x, 0)` does.
   The type the place expects reaches `x` and `y` only: the code types a pair with it (`same_type` and `pair`,
   `native-theme/src/expr/ty.rs:992-1052`), and for `(a, b)` it says nothing, so `(a, b)` is typed apart, with no
   expected type, and its type checked by `arithmetic` (`ty.rs:1143-1151`), which an implementer reuses.
5. **Totality** (`[spec]:566-577`) gains: "`if_le` compares two finite values, every operand being finite (a
   non-finite result, or non-finite element data, is zero, below), so it always selects". The code makes each
   step finite already (`native-theme/src/expr/arith.rs:8-11`; `eval.rs:495-506`).
6. **The model's own arithmetic** (`[spec]:586-588`, "(`alpha`, `with_alpha`, `mix`, `mul`, `div`, `add`, `sub`,
   `min`, `max`, `floor`, `round`, `round_pt`, `pick`) is not a platform function: its unit tests follow this
   section's definitions") gains `ceil` and `if_le`.
7. **S4.2's blink timeout** (`[spec]:3425-3445`):
   - "`windows-11`'s `blink_timeout` is `= mul(p, add(n, mul(<the fallback count>, sub(1, min(n, 1)))))`, […]
     `sub(1, min(n, 1))` is 1 where `n` is 0 and 0 where it is a positive integer, so the factor is `n` where a
     period fits and the fallback count where none does, whatever the count" becomes "`windows-11`'s
     `blink_timeout` is `= mul(p, if_le(n, 0, <the fallback count>, n))`, `p` and `n` written out where they stand,
     as the grammar has no names for subexpressions: WinUI's `nPeriods <= 0` (`:2756`) is `if_le(n, 0, …)`".
   - "its `1`s are the form's constants, not WinUI's literals (S17's quote check)" is withdrawn: the fact's
     literals are WinUI's, the fallback count `3` (`:2759`) and the `0` of its test (`:2756`).
   - The sentinel sentence stands, its reading now WinUI's own test: "WinUI's `nPeriods = -1` (`TextBoxView.cpp:2745`)
     is a sentinel, […] which its `nPeriods <= 0` test folds into the fallback (`:2756-2760`)".
   - Of the "Rejected" list, "a conditional expression (a general grammar construct, a comparison operator and
     S12's conservative checks over both branches, for one use, where a conditional count's fallback is
     arithmetic)" becomes "a conditional as a grammar construct (a boolean type, comparison operators and branches
     in the grammar), where S2.2's `if_le` is a function call with one comparison, which U180 uses too; the
     zero-fallback arithmetic `add(n, mul(k, sub(1, min(n, 1))))`, whose `1`s no WinUI line holds, which S17 would
     have to exempt".
8. **S17's quote check** (`[spec]:6883-6885`): the parenthesis "and the constants of S2.2's named integer forms,
   the `1`s of the zero-fallback form `add(n, mul(k, sub(1, min(n, 1))))`, which are the form's, not the
   platform's, the checker recognising the form by its shape and checking `k` and `n`'s operands as any literal,
   ADD-A134" is withdrawn. No named form with constants remains (the zero-fallback is the one S2.2's row has
   constants for, `[spec]:464`), so every literal of a value needs a quote.
9. **S2.8** (`[spec]:2396-2399`, "Neither is `QFontMetrics`' `underlinePos` or `strikeOutPos`, and the model's
   operands do not give them (no ceiling, no ascent or descent alone), so `kde-breeze`'s decorations are register
   U180's until the value phase states each path's form") becomes: "Neither is `QFontMetrics`' `underlinePos` or
   `strikeOutPos`. With the descent, `element.font_descent` (S8; the ascent is `sub(element.font_height,
   element.font_descent)`), S2.2's arithmetic states QPainter's underline: its thickness
   `max(round(element.font_underline_thickness), 1px)` without antialiasing and
   `element.font_underline_thickness` with it for a font whose underline thickness is positive (QPen ignores a
   negative width and draws a zero one as a 1 px cosmetic pen, `qtbase@6.12.0 src/gui/painting/qpen.cpp:488-490`,
   `:658-663`), its offset `if_le(element.font_underline_offset,
   element.font_descent, min(ceil(element.font_underline_offset), sub(element.font_descent, <the thickness>)),
   ceil(element.font_underline_offset))` (`qtbase@6.12.0 src/gui/painting/qpainter.cpp:6121-6130`). QPainter's
   wavy and strike lines and Qt Quick's lines are register U180's until the value phase states each one's form: Qt
   Quick rounds the line's position in the item (`qtdeclarative@6.12.0
   src/quick/items/qquicktextnodeengine.cpp:213`, `:276`), which no font metric gives." The split by drawing path
   that follows (`:2399-2408`) stands.

*The tools.*
- `scripts/check_fact_citations.py` needs nothing.
  - The thickness's and the offset's one literal, `1px`, equals `QFixed(1)`'s `1` (`qpainter.cpp:6123`): a
    quote's literal without a unit takes the value literal's dimension (docstring `:191-193`; `equal`,
    `:643-654`).
  - Function names are no literals: the value reader's `path` group takes them (`VALUE_TOKEN_RX`, `:574-578`).
  - U505's literals are WinUI's: `5000ms` (`m_rCaretBlinkTimeout = 5.0f;`,
    `src/dxaml/xcp/core/text/common/TextSelectionSettings.cpp:39`, under the `seconds_to_ms` conversion, docstring
    `:196`), `0` (`:2756`) and `3` (`:2759`).
  - It no longer needs the zero-fallback recognition, owed since ADD-A134 (ADD-A141's checker follow-up, the one
    thing U505's timeout waited on, `[ledger]:667`), nor the ceiling and selection shapes revision 3 added.
- `scripts/check_facts.py` needs one change. Its `round_pt` rule flags `round(div(<font size>, <n>pt))` "(or
  `floor`)" as S2.2's rejected form of a size rounded in whole points (`round_pt_problems`, the test
  `text in ("round", "floor")` at `:3369`, its docstrings at `:230` and `:3351`); `ceil(div(<font size>, 1pt))`
  would pass it. The test gains `ceil`, and both docstrings say "(or `floor` or `ceil`)". The arity reading needs no
  change: `core_functions` reads the names and arities from S2.2's **Functions** table (`:538-549`), so the
  arithmetic row gives `ceil` one argument and the new row gives `if_le` four. Its self-test fixture
  (`:3840-3851`) and that fixture's expected set (`:5295-5299`) are its own and stay. `[check3]`'s cost table lists
  `check_facts.py:3849` and `:5296` as mirrors to change; they are that fixture and its expectation.
- `scripts/check_function_vectors.py`: `ARITHMETIC` (`:164-165`) and its docstring (`:100-101`) gain `ceil` and
  `if_le`. Otherwise both count as platform functions (`platform_functions`, `:866`), and every fact that calls one
  is a finding, "a platform function not implemented in the platform module" (`:1050`).
- `scripts/verify_facts.py`: `arith` and `call` (`:1258-1296`) gain `ceil` beside `floor` (`:1259-1261`) and
  `if_le` beside the binary operators. Otherwise a lane's and a verifier's spellings of one value are "unresolved"
  (`:1699-1703`), an unknown function being "not evaluated here" (`:1296`).
- `scripts/generate_preset.py` needs the same under every form: `FONT_METRICS` (`:301-303`) lists the metrics a
  `widget.<part>.<metric>` operand may name (`:1538-1539`) and gains `font_descent`, so that a part without text
  can read another part's descent. `element.font_descent` needs no change there (`DATA_OPERAND`, `:299`). It reads
  no function name (no `floor`, `round_pt` or `kde_shade` in the file, by search), so the operators ask nothing of
  it. Under revision 3's form it needed nothing more either.
- `scripts/check_spec_names.py` reads the element data from S2.2's production (`:179`, checked at `:504`): no code
  change. Its self-test fixture (`:947-949`) is a fixture of its own.

*The model's code.* Read, not built (section 5).
- `ceil` and `if_le` go beside `floor`:
  - `Func` (`native-theme/src/expr/mod.rs:318`, `:393`, `:442`), and `Func::ALL`'s length, `[Func; 43]` (`:375`);
  - the typed tree's new `Node` variants, beside `Floor` (`ty.rs:500-527`, `Floor` at `:510`), and the typing
    (`ty.rs:776-784`; `if_le`'s `(x, y)` as `min`'s pair, `(a, b)` typed apart);
  - the evaluator (`eval.rs:505`) and `arith.rs` (`floor` at `:88`);
  - the doc comments that list the model's arithmetic (`mod.rs:275-277`, `arith.rs:1-3`) and its totality
    (`eval.rs:485-489`).
- The parser needs no change: it reads a function's name through `Func::from_name` (`parse.rs:331`;
  `mod.rs:472-474`), which searches `Func::ALL`.
- Nothing outside `expr/` matches on `Func` for arithmetic (`validate/checks.rs:1180-1222` matches `Pick`,
  `RoundPt`, `KirigamiLerp` and KDE's accent functions only), so the validation needs no change: S12 checks the
  new operands as any function's.
- `font_descent`: `FontMetrics::descent` and the operand, and with them what the compiler then requires: the five
  test implementations of `FontMetrics` (`query/tests.rs:29`, `resolve/evaluate/tests.rs:34`, `theme/tests.rs:349`
  and `:1768`, and the facts gate's `native-theme/tests/facts_gate/run.rs:35`), and the datum's lists and match arms
  (`declare.rs:400`, `resolve/evaluate.rs:1868`, `:1899`, `:3283`, `expr/ty.rs:1188`).
- Each operator comes with unit tests from the specification's definitions (`[spec]:586-588`): the ties `a = b`,
  the font-size pair, a bare number, a non-finite operand.
- No function of the platform module, no vector table, no clean room (D10 governs code ported into the crate,
  `[plan]:450-467`; the facts state Qt's rule as every KDE derivation does).

*The facts,* by the KDE lane, on the part declaring the three metrics:
- `button.label.text.mnemonic_decoration.thickness`: `value = "= max(round(element.font_underline_thickness),
  1px)"`, `published`, `qpainter.cpp:6123` (quote `pen.setWidthF(qMax(fe->lineThickness().round(),
  QFixed(1)).toReal());`).
- `button.label.text.mnemonic_decoration.offset`: `value = "= if_le(element.font_underline_offset,
  element.font_descent, min(ceil(element.font_underline_offset), sub(element.font_descent,
  max(round(element.font_underline_thickness), 1px))), ceil(element.font_underline_offset))"` (one line in the
  file, 215 characters after `= `), `published`, `qpainter.cpp:6128-6130` and `:6123` (the thickness inside it).
  The exact quotes are the lane's.
- The two of `component.title_widget` with its unit; the link's with unit 5.

*U505's fact*, by the Windows lane, after the batch's code (the core parses the preset's expression):
- `token.caret.blink_timeout`: `value = "= mul(add(token.caret.blink_on, token.caret.blink_off),
  if_le(floor(div(5000ms, add(token.caret.blink_on, token.caret.blink_off))), 0, 3, floor(div(5000ms,
  add(token.caret.blink_on, token.caret.blink_off)))))"` (206 characters after `= `), `published`. Its citations:
  - `TextBoxView.cpp:2756-2759`, a quote holding `nPeriods <= 0` and `nPeriods = 3;`;
  - `TextSelectionSettings.cpp:39` with `conversion = ["seconds_to_ms"]`.

  The pending fact's comment (`docs/facts/windows/tokens.toml:2711-2718`) is replaced by the form.
- Exact: `register/u180_ops_check.py` evaluates this text for every blink time WinUI can hold, 0 to 10,000 ms
  (`TextBoxHelpers.cpp:78-86`), with `blink_on` and `blink_off` that time. It compares the result with WinUI's
  lines transcribed in `float`: the period `static_cast<float>(ms) / 1000.0f * 2.0f` (`:87-89`) and the count, its
  sentinel and its fallback (`TextBoxView.cpp:2742-2763`). The count, evaluated from the fact's own `if_le`
  argument, equals WinUI's at all 10,001 times; a positive control, that argument with `1` for its `0` (a `<` for
  WinUI's `<=`), differs at 1,250. The timeout agrees within `f32`'s rounding, the largest relative difference
  1.04e-7, from WinUI's seconds against the fact's milliseconds. It also equals the zero-fallback form's value
  everywhere.
- *Whether U505 closes: no.* Its `blink_timeout` no longer waits on a checker task, only on this batch and the
  fact. Its `solid_while_typing` stays open (`[ledger]:667`; `docs/facts/windows/tokens.toml:2744-2747`). The
  ledger's U505 row then names that key alone, and queue #160 (4)'s note (`[ledger]:368`) is updated.
  - What still blocks it is a source read of WinUI's typing path, kind `b`, which the operators do not touch. The
    pending fact has no text of its own; its open point is in the comment above `blink_on`, that the 1 s start
    timer (`m_rCaretBlinkStart = 1.0f;`, `TextSelectionSettings.cpp:40`) "restarts on each caret move while the
    storyboard runs on" (`docs/facts/windows/tokens.toml:2717-2718`), and no line is cited for that restart. In
    `UpdateCaretElement` the storyboard and the start timer restart only where the caret's width or the blinking
    period changes (`microsoft-ui-xaml@2.5.1 src/dxaml/xcp/core/native/text/Controls/TextBoxView.cpp:2598-2633`).
    The read must find what a keystroke does to the caret's blink, against the token's meaning (X-CA-2,
    `[spec]:3328`).
  - The ledger's U505 "next" cell then reads: "`solid_while_typing`: a source read of WinUI's typing path (what a
    caret move or a keystroke does to the blink storyboard and the start timer, `TextBoxView.cpp`), against
    X-CA-2's meaning".

*Two points for the lane's verifier.*
- That neither flavour's painter antialiases when it draws a button's label. Breeze's label code sets no render
  hint (`breeze@6.8.2 kstyle/breezestyle.cpp:5322-5460`, by search). The Kirigami style item is a `QQuickItem`
  (`qqc2-desktop-style@6.31.0 plugin/kquickstyleitem_p.h:48`) that paints into its own image with a new painter,
  `QPainter painter(&m_image); […] paint(&painter);` (`plugin/kquickstyleitem.cpp:2096-2098`), and no file of the
  plugin names `setRenderHint` or `Antialiasing` (by search). A part found to set the hint takes the antialiased
  form of part 2, which is exact for a font whose underline thickness is positive only; where the font's
  thickness is zero or less, Qt's line on that path follows QPen (a 1 px cosmetic pen, or the painter's pen), which
  no form here states.
- The offset's exactness, by the cases of part 2 and `register/u180_ops_check.py`. When the facts are committed,
  that check runs on the committed text, read from the file, rather than on the script's copy (`[check3]`, finding
  12).

*The row's own check*, "a capture of each path's underline at two fonts", confirms the form for the cases those
fonts reach; case 3 only with a font whose underline lies below its descent, which no font is known to have.

*The register.*
- The Decision cell records (b′) for QPainter's underline. The row is narrowed to what part 3 lists: Qt Quick's
  line position, the strike lines and QPainter's wavy line. It stays kind `d`, its due point the flavours unit for
  Qt Quick's parts, and for QPainter's lines the first part that states one.
- The flavours unit carries U180's open point: a measurement or a model form for Qt Quick's line position, and, of
  the model forms, only the decoration field in that unit's patch release (part 3). The unit decides it; it is not
  a pick of this document.
- Both listed paths are empty once the batch and the two button facts are in.

**5. Confidence.** High that the source reads as stated, and high on exactness: case by case, and recomputed from
the facts' own text with no difference on the non-antialiased path, which the facts take, and on the antialiased
path for a positive thickness (part 2). Medium-high on (b′) as the form. A veto would turn on one of two
preferences. One is a model with no comparison operator: then (a2), today's operators with the unit-literal
exemption `[spec]:465` rejects. The other is Qt's rule named by a function and checked by vectors for its first two
cases: then (c), with its third case recorded as read, not measured.

---

### The token `single_click_activates`: the evidence, for the containers unit (U504, U555; decision C)

Revision 3 gave the token one sentence for S4.2, to be read one way on every preset. Check 3 found that it gives no
single value on `windows-11`, on `macos-*` or under `flavor:kirigami`, and that it has no mark for a view it does
not name (`[check3]`, findings 1, 2, 5, 6). Decision C takes the token out of this round. This part is now the
record the containers unit starts from: what exists (part 1), which views count (part 2), the decision and what it
leaves open (part 3), the evidence by platform (part 4), the earlier rounds' readings (part 5), and the question of
the model's form (part 6).

**1. What exists.**

- The specification has the field and no sentence for it: `pub single_click_activates: Value<bool>`
  (`[spec]:3341`); the code's comment is "A single click activates an item." (`native-theme/src/tokens.rs:333-335`).
  No catalogue entry reads the token (by search). 0.7.0's model held nothing under that name or for that behaviour
  (section 2.3).
- Its origin is one row, KDE's K3.73, "`SingleClick` (item views open on one click; hover feedback in file
  views)" (`[gap-kde]:662`; `[trace-kde]:1136-1139`), which is Qt's style hint: "Emit the activated signal when the
  user single clicks on an item in an item in an item view. Otherwise the signal is emitted when the user double
  clicks on an item" (`qtbase@6.12.0 src/widgets/styles/qstyle.cpp:1919-1921`, the repetition Qt's own; the two
  emissions, `src/widgets/itemviews/qabstractitemview.cpp:2024-2026`, `:2053-2055`).
- Two presets state it. `kde-breeze`: `false`, and `true` under `system_setting:single_click`
  (`docs/facts/kde/tokens.toml:1843-1864`). `adwaita`: `false`, with the comment "the item views' activation (KDE's
  sense of the token, K3.73): GtkListView, GtkGridView and GtkColumnView activate an item on a double click unless
  the application sets single-click-activate; GNOME has no setting for it (GtkListBox's and GtkFlowBox's rows are
  no item view)" (`docs/facts/gnome/tokens.toml:2170-2191`).
- Four presets have it pending: `windows-11` (U504, `docs/facts/windows/tokens.toml:2922-2925`), `material-3`
  (U555, `docs/facts/material/tokens.toml:10059-10062`), `ios-*` (U472, `docs/facts/ios/tokens.toml:20800-20803`),
  `macos-*` (U329, U609, `docs/facts/macos/tokens.toml:46662-46666`, `:47803-47807`).
- The catalogue's `list` is every platform's list, table, tree, grid and icon view: "Every platform draws it (list,
  table, tree, grid or icon view)", with each toolkit's views named, and "Trees are lists with disclosures"
  (`[catalogue]:3958-3966`). It comes with the containers unit, unit 8 of 0.8.0 (`[plan]:773-775`).

**2. Which views count: three readings.** The field's name leaves open which views it speaks of. GTK, whose fact
states the token, defines its views by their model: "**_Views_** or **_list widgets_** are the widgets that hold and
manage the lists. Examples of these widgets would be [`class@Gtk.ListView`] or [`class@Gtk.GridView`]", and "Views
display data from a **_model_**" (`gtk@4.24.1 docs/reference/gtk/section-list-widget.md:26-27`, `:29`). The view
makes its items' widgets itself, through a factory (`:61-63`), and "the views only create a limited amount of
listitems and recycle them by binding them to new items" (`:105-108`). Its selection is not the view's: "Views
support selections via a **_selection model_**", whose behaviour "is completely up to the selection model", and
"selection state is now meant to be managed by the selection models" (`:76-88`, `:242-244`). Activation is the
view's: it "means that double clicking or pressing enter while inside a focused row will cause the view to emit a
signal such as [`signal@Gtk.ListView::activate`]" (`:90-92`).

- **(i) The model alone** (GTK's definition). An item view presents the items of a model. A container of row
  widgets is none, even where its rows are made from a model: "A `GtkListBox` only contains `GtkListBoxRow`
  children" (`gtk/gtklistbox.c:50`), and its `bind_model` fills it "with widgets that represent items from @model"
  (`:4055-4056`), which it then holds as children; GTK's page on its views never names GtkListBox (by search).
  - What it changes: no stated fact. `kde-breeze`'s facts and `adwaita`'s, its comment included, stay as written.
  - The pending ones: `material-3` `true`, since Compose's lazy list is "The vertically scrolling list that only
    composes and lays out the currently visible items" (`androidx@e4d933c9
    compose/foundation/foundation/src/commonMain/kotlin/androidx/compose/foundation/lazy/LazyDsl.kt:429`), filled by
    `LazyListScope.items`, which "Adds a list of items." (`:155`, the function at `:169`), and its items' click runs
    their action (part 4); `ios-*` `true`; `windows-11` and `macos-*` with no single value (part 4).
  - Its limit: "presents the items of a model" read loosely would count a GtkListBox bound to a model and bring
    (iii)'s cost. Revision 3's sentence closed it by naming each toolkit's views and the containers that are none,
    which a lane cannot carry to a view it does not name (WinUI's TreeView, Qt Quick's ListView, AppKit's
    `NSBrowser`). GTK's own page gives a mark that can be carried: the view makes its items' widgets through a
    factory and recycles them (`:61-63`, `:105-108`). Under it GtkListBox and a Compose `Column` are containers,
    and `LazyColumn`, WinUI's ListView and TreeView and Qt Quick's ListView are views (`[check3]`, finding 6).
- **(ii) The model, and a selection the view holds** (revision 2's).
  - What it changes: no stated fact; `material-3` `absent`, since Compose's lazy list takes neither a click nor a
    selection (`LazyDsl.kt:470-483`).
  - Against it: the selection clause is not GTK's (above), whose selection lives in an object the application
    supplies. `absent` is allowed only with a citation that the platform has none (section 2.1, rule 1), and the
    pinned reference shows the behaviour: Material's sample `ListItemWithModeChangeOnLongClickSample` draws items
    whose click runs their action (`onClick = { counts[idx]++ }`), whose long click enters a selection mode, and
    whose click there toggles them (`compose/material3/material3/samples/src/main/java/androidx/compose/material3/samples/ListSamples.kt:354-394`).
    And `ios-*` would be `true` and `material-3` `absent` for the same tap on a row, because UIKit's view stores the
    selection and Compose hoists it.
- **(iii) Any list of rows.** GtkListBox and GtkFlowBox count.
  - What it changes: the `adwaita` fact. GtkListBox's `activate-on-single-click` defaults to `TRUE` and its
    `selection-mode` to `GTK_SELECTION_SINGLE` (`gtk/gtklistbox.c:525-544`), so GNOME has two views that answer
    differently, which one bool cannot state; the value would move to the `list` widget by variant. `material-3`
    `true`.

Revision 3 applied (i) by the orchestrator's decision A. Decision C withdraws that application: which views count
is the containers unit's to decide, with the model's form (part 6).

**3. Decision C, and what it leaves open.**

- *Why the token leaves this round.* Three rounds have not found one reading that gives one value on every
  platform. WinUI's own views answer differently, and AppKit's pinned header can be read as deciding (part 4). And
  0.7.0's model never held the token: `git grep -n -i -E 'single.?click|double.?click|activate_on' v0.7.0 --
  native-theme/src connectors` finds three comments of the gpui showcase on a title bar's double click, and no
  value (section 2.3). So by D13 it is not on 0.8.0's path, and the staging parks it with the behaviour tokens
  (`[staging2]`, section 3.1, *behaviour*, `:701`).
- *The facts stay as they are.* `adwaita` and `kde-breeze` state the token (part 1). `windows-11`, `material-3`,
  `ios-*` and `macos-*` keep it pending (part 1). No S4.2 sentence is written now.
- *The register rows and listed paths it leaves open:*
  - U504's `single_click_activates` (`windows-11.toml:360`); U504's two mnemonic tokens are settled (U504);
  - U555, whole (`material-3.toml:290`);
  - and, outside the rows examined here, the token's key in U472 (`ios-13.toml:793`, `ios-26.toml:805`), in U329
    (`macos-tahoe.toml:428`) and in U609 (`macos-monterey.toml:414`), each among other behaviour keys of its row.

  All six paths leave 0.8.0's path by the staging. Its revision 2 parks `token.behavior` but `focus_visible_when`
  and the mnemonic tokens (`[staging2]`, section 1.5, `:313-314`; section 3.1, `:701`). Its section 4 lists U504
  in part, and U555, U472, U329 and U609 among the rows that leave (`:810-811`, `:814-815`, `:816-827`).
- *Where the question goes.* To the containers unit, which brings `list`, every platform's list, table, tree, grid
  and icon view (part 1). Whether the answer then enters 0.8.0 is that unit's to show. 0.7.0's model held no
  activation value (section 2.3), and the token waits in its parking unit until a value at depth reads it
  (`[staging2]`, section 1.5).

**4. The evidence, by platform.** What the three checks and the four revisions found. Each citation was verified
at the pin for this revision.

| Platform | The views, and how each activates | What it gives |
|---|---|---|
| GNOME (`adwaita`) | GTK's definition and its mark (part 2). `GtkListView`, `GtkGridView` and `GtkColumnView`: `single-click-activate`, "Activate rows on single click and select them on hover", default `FALSE` (`gtk@4.24.1 gtk/gtklistview.c:948-956`, the default at `:955`; `gtk/gtkgridview.c:1185-1187`; `gtk/gtkcolumnview.c:934-936`), where activation "means that double clicking or pressing enter […] will cause the view to emit a signal" (`docs/reference/gtk/section-list-widget.md:90-92`). `GtkListBox`: rows as children (`gtk/gtklistbox.c:50`), filled from a model by `bind_model` (`:4055-4056`), `activate-on-single-click` default `TRUE` and `selection-mode` default `GTK_SELECTION_SINGLE` (`:525-544`) | The views: `false`, the stated fact and its comment (`docs/facts/gnome/tokens.toml:2170-2191`). `GtkListBox`, if it counts: `true` |
| KDE, Qt Widgets (`kde-breeze`) | Qt's hint: "Emit the activated signal when the user single clicks on an item in an item in an item view. Otherwise the signal is emitted when the user double clicks on an item" (`qtbase@6.12.0 src/widgets/styles/qstyle.cpp:1919-1921`); the views emit `activated` on the click where the hint holds and on the double click otherwise (`src/widgets/itemviews/qabstractitemview.cpp:2024-2026`, `:2053-2055`); KDE sets the hint from kdeglobals' `SingleClick`, default `false` (`plasma-integration@6.8.2 qt6/src/platformtheme/khintssettings.cpp:90`) | `false`; `true` under `system_setting:single_click`: the stated facts (`docs/facts/kde/tokens.toml:1843-1864`) |
| KDE, Kirigami (`kde-breeze` under `flavor:kirigami`: "facts under `flavor:kirigami` hold where Kirigami differs", `docs/facts/kde/tokens.toml:3-4`, ten uses in the file) | The style item exposes the hint (`qqc2-desktop-style@6.31.0 plugin/kquickstyleitem.cpp:1517-1518`). No QML or code of qqc2-desktop-style, Kirigami or kirigami-addons reads it: a search of the three checkouts for `activateItemOnSingleClick` finds that one file. Qt Quick's `ListView` makes its items from a delegate, "instantiated as needed" (`qtdeclarative@6.12.0 src/quick/items/qquicklistview.cpp:2357`), and can recycle them (`:2305-2309`, off by default): a view by GTK's mark of a factory and a limited number of items. Its delegates are the application's. QQC2's `ItemDelegate` is a `QQuickAbstractButton` (`qtdeclarative@6.12.0 src/quicktemplates/qquickitemdelegate_p.h:25`), which emits `clicked` for one click and `doubleClicked` for two (`src/quicktemplates/qquickabstractbutton.cpp:448-466`). Which signal a Kirigami list connects to its activation was not found at a pin (`[check3]`, finding 5) | Two readings: the style hint for both flavours (`false`), or the delegate's single-click convention (a `true` rule under `flavor:kirigami`) |
| Windows (`windows-11`) | **ListView and GridView** (`ListViewBase`). Item click is the application's to turn on, "The default is **false**" (`winapps-winrt-api@554b7e3a microsoft.ui.xaml.controls/listviewbase_isitemclickenabled.md:22`), and "By default, a user can select items in the view by tapping or clicking on them. You can change this behavior to make the user interaction fire a click event instead of triggering item selection" (`:25`). Selection mode defaults to `Single` (`listviewbase_selectionmode.md:25`). The API page: "If you set the IsItemClickEnabled property to **true**, you must set the [SelectionMode](listviewbase_selectionmode.md) property to [ListViewSelectionMode.None](listviewselectionmode.md)." (`listviewbase_isitemclickenabled.md:27`, as the file writes it; a fact's quote keeps the markup or quotes a part without it, since the quote check reads the file's text). The Learn page documents the same pair as supported: "You can enable IsItemClickEnabled to raise an ItemClick event while SelectionMode is also set to **Single**, **Multiple**, or **Extended**. If you do this, the ItemClick event is raised first, and then the SelectionChanged event is raised" (`windows-dev-docs@f54166ed hub/apps/develop/ui/controls/listview-and-gridview.md:376-377`). The code: "We interpret the primary interaction gesture as an ItemClick if it's enabled or as a selection otherwise" (`microsoft-ui-xaml@2.5.1 src/dxaml/xcp/dxaml/lib/ListViewBase_Partial_Interaction.cpp:252-253`); with both on, one tap raises `ItemClick` and then selects (`:255-274`). None of the 70 files of `src/dxaml/xcp/dxaml/lib/` whose names begin `ListViewBase`, `GridView`, `ListView`, `Selector` or `ItemsControl` names `DoubleTap` (by search). **TreeView.** Its inner list turns item click on itself, `IsItemClickEnabled(true);` (`src/controls/dev/TreeView/TreeViewList.cpp:384`). Its selection mode defaults to `Single` (`TreeView.idl:114-116`; `microsoft.ui.xaml.controls/treeview_selectionmode.md:19`), which it gives the list (`TreeView.cpp:408-410`). It raises `ItemInvoked` from the list's `ItemClick` (`:159-164`, `:421`). Microsoft's page: "Unlike **ListView**, which has the [IsItemClickEnabled](…) property, invoking an item is always enabled on the tree view" (`hub/apps/develop/ui/controls/tree-view.md:383`). **ItemsView.** Invocation is the application's, default `false` (`src/controls/dev/ItemsView/ItemsView.idl:41-42`; `itemsview_isiteminvokedenabled.md:19`); selection defaults to `Single` (`ItemsView.h:21`; `itemsview_selectionmode.md:19`). No item is invoked where invocation is off, nor by a double tap with no selection mode, nor by a tap or the space key with one (`ItemsView.cpp:421-424`). Its Learn page carries ListView's note: "You can enable IsItemInvokedEnabled to raise an ItemInvoked event while SelectionMode is also set to `Single`, `Multiple`, or `Extended`. If you do this, the ItemInvoked event is raised first, and then the SelectionChanged event is raised" (`hub/apps/develop/ui/controls/itemsview.md:262-263`) | TreeView `true`, with no configuration. ListView and GridView: excluded under the API page, `true` under the Learn page and the code. ItemsView: `false` by its code (a tap selects, a double tap invokes), `true` by its page. One bool cannot state all four |
| Material (`material-3`) | Compose's lazy list (`LazyDsl.kt:429`, `:155`, `:169`; part 2) names no click and no selection among its parameters (`:470-483`). Material's `ListItem` overload with an action "handles click events, calling its [onClick] lambda to trigger an action" (`compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ListItem.kt:304-306`), by `Modifier.combinedClickable(interactionSource = …, indication = …, enabled = …, onLongClick = onLongClick, onLongClickLabel = …, onClick = onClick)` (`:1602-1611`). That call passes no `onDoubleClick`, the parameter of the overload it calls (`compose/foundation/foundation/src/commonMain/kotlin/androidx/compose/foundation/Clickable.kt:551`, documented "will be called when user double clicks on the element", `:539`; the overload `:543-553`; the overload at `:322-330` is `DeprecationLevel.HIDDEN`). `onDoubleClick` occurs nowhere in `ListItem.kt` (by search). The plain overload passes `onClick = null` (`:288`); single- and multi-selection overloads (`:386`, `:470`). Material's sample `ListItemWithModeChangeOnLongClickSample`: a click runs the item's action (`onClick = { counts[idx]++ }`), a long click enters a selection mode, a click there toggles the item (`compose/material3/material3/samples/src/main/java/androidx/compose/material3/samples/ListSamples.kt:354-394`); its items stand in a `Column` (`:355`) | `true` by reading (i) with the item's documented convention; `absent` by reading (ii) |
| macOS (`macos-*`) | `NSTableView` (and `NSOutlineView`, its subclass) has a click `action` (`xcode@26.6 AppKit/NSControl.h:25`) and a `doubleAction` (`NSTableView.h:278`); `NSBrowser` a `doubleAction` (`NSBrowser.h:57`); `NSCollectionView` is an `NSView` (`NSCollectionView.h:127`) and declares neither (by search). `NSWorkspace`: "Returns the URL to the default application that would be used to open the given URL, as if the file were double clicked in the Finder" (`NSWorkspace.h:119`, `:134`). All the same lines at `xcode@16.4` | Two readings: `NSWorkspace` names the double click as the way to open an item, the Finder's convention read as AppKit's (`false`); or no pinned source says which of a view's two actions is an item's activation (pending). Candidates that would decide, by name only, neither pinned nor read: Apple's documentation of `NSTableView.doubleAction`, and the Human Interface Guidelines' page on lists and tables. `NSCollectionView` has no action to rank |
| iOS (`ios-*`) | `UITableView` selects by default (`allowsSelection`, "default is YES", `xcode@26.6 UIKit/UITableView.h:448`), and "Primary actions are performed when the user selects a cell without extending an existing selection. This is called after @c willSelectRow and @c didSelectRow" (`:167-170`); `UICollectionView` the same (`UICollectionView.h:152-153`, `:411`) | `true` |

**5. The readings of the earlier rounds, and what each would change.**

- *Revision 1* joined two readings in one sentence (check 1, finding 2). One was (R-A), what one click does in an
  item view as the platform ships it, an application's opt-in excluded. The other was (R-B), which gesture
  activates where a view both selects and activates.
  - Values: `windows-11` `false` on the views' defaults; `material-3` `true` by the `onClick` overload. That took
    R-A for one row and R-B for the other.
  - By R-A an iOS lane reads `UITableView`'s default, a tap that selects, and writes `false`.
- *Revision 2*: R-B over views that show a model's items and hold a selection, reading (ii).
  - Values: `windows-11` `false` on WinUI's exclusion of click and selection; `material-3` `absent`; `ios-*`
    `true`.
  - Check 2's finding 1: `absent` withholds a behaviour the pinned reference shows, and `ios-*` and `material-3`
    differ for the same tap.
- *Revision 3*: reading (i), applied in order. First the toolkit's views with only the activation turned on, a
  configuration its documentation forbids not counted; then the documented convention where the views have no
  activation; and pending where no pinned source ranks a view's two actions.
  - Values: `windows-11` `false`; `material-3` `true`; `ios-*` `true`; `macos-*` pending.
  - Check 3's findings: no single value on `windows-11` (TreeView; the Learn pages), on `macos-*` (`NSWorkspace`)
    or under `flavor:kirigami`; and no mark for an unnamed view.

What would change under each of part 2's readings, on the stated facts: (i) and (ii) none; (iii) `adwaita`'s, its
GtkListBox answering `true`.

**6. The question for the containers unit: the model's form.** Not decided here (decision C). The options, with the
arguments as they stand:

- **(A) One bool per platform, as now** (`Behavior.single_click_activates: Value<bool>`, `[spec]:3341`), with a
  sentence that says which views speak and what a lane writes where they disagree.
  - For: no model change; the form of Qt's hint and of KDE's one setting.
  - Against: on `windows-11` the views disagree (TreeView `true`, ItemsView `false` by its code), and on GNOME
    GtkListBox would, if it counted. So the bool states one view, and a connector drawing another gets that view's
    gesture wrong.
- **(B) A value per view**, a `list` value stated by variant, with the token kept for a user's setting
  (KDE's `SingleClick`, a rule under `system_setting:single_click`).
  - For: each view states its own gesture, and disagreeing views need no rule.
  - Against: a value every preset states for each list variant, so the catalogue's list variants must name the
    views; more facts.
- **(C) Both**: the token as the platform's default and a `list` value where a view differs.
  - For: one fact where the views agree.
  - Against: two homes for one behaviour.

`absent` is no option where the views exist and activate (rule 1). The unit decides, with the form:
- which views count (part 2);
- how a view is read: the default configuration, a documented but non-default configuration, and documentation
  against code (part 4, Windows);
- whether `NSWorkspace` decides AppKit's primary action;
- whether the Kirigami flavour reads the style hint or its delegates.

---

### U504 — `windows-11`: mnemonics, and single-click activation

**1. The row.** `[ledger]:666`: "`windows-11`: `mnemonics`, `mnemonic_when_disabled`, `single_click_activates` (the
Windows facts lane m1b, 57e0461e) | a model choice (Win32 underlines against WinUI KeyTips), and
`IsItemClickEnabled` not read | the choice decided, then a read".

- Facts: 3, `docs/facts/windows/tokens.toml:2910`, `:2915`, `:2922`. Listed paths: 3,
  `native-theme/src/presets/windows-11.toml:356`, `:358`, `:360`.
- The specification: `Mnemonics { Never, OnAlt { delay }, Always }` (`[spec]:3351`); `mnemonic_when_disabled` "is
  `Absent` where `mnemonics` is `Never`" (`[spec]:3525`); every `Value` field of `Behavior` may be absent
  (`[spec]:4059-4060`). Both tokens govern the underline in a label: the connector applies
  `mnemonic_when_disabled` "as it applies `mnemonics`: where it is `false` it draws no `mnemonic_decoration` there"
  (`[spec]:3524-3525`). WinUI's badges are a keyed component of their own, `component.key_tip`, "WinUI `KeyTip`,
  access-key badge", available on `windows-11` (`[catalogue]:9458-9473`). The trace sends W3-48† to the token and
  to a read of `SPI_GETKEYBOARDCUES` (`[trace-windows]:720-721`); the gap map says "Win32 menus and dialogs
  underline access keys only while Alt is held […]; WinUI shows KeyTip badges instead" (`[gap-windows]:374`).
- `single_click_activates`: moved by decision C to the containers unit. WinUI's views and what each gives are in
  the token's part, part 4.

**2. The options, for `mnemonics`.**

- **(a) `never`.** WinUI underlines no access key: Microsoft's pinned page says "access-key underline styling is
  not automatic. If you want visible mnemonic underlines, render them explicitly"
  (`windows-dev-docs@f54166ed hub/apps/design/accessibility/keyboard-accessibility.md:211`). `UIElement.AccessKey`
  is "the access key (mnemonic) for this element" (`winapps-winrt-api@554b7e3a
  microsoft.ui.xaml/uielement_accesskey.md:14`), shown by a KeyTip ("where the access key keytip is placed",
  `microsoft.ui.xaml/uielement_keytipplacementmode.md:14`).
- **(b) `on_alt`, Win32's underline.** Another toolkit's behaviour in WinUI's preset, against R1.3; its delay has
  no source.
- **(c) A new `Mnemonics` member for KeyTips.** A model change for what `component.key_tip` already carries.

**3. The outcome.** `mnemonics` `never`, by R1.3 and the pinned sentence; `mnemonic_when_disabled` `absent`, by
S4.2's own rule. `single_click_activates` has no outcome this round (decision C; the token's part, part 3).
Revisions 1 to 3 gave it `false` on three grounds in turn: the views' defaults, a ground stated only in this row,
and revision 3's S4.2 sentence. Check 3's finding 1 shows that WinUI's views answer differently, so none of the
three grounds gives one value. Neither mnemonic token is 0.7.0's. The staging's revision 2 keeps both at depth,
since a connector draws the kept `mnemonic_decoration` by them (`[staging2]`, section 1.5, clause 4, `:279-282`).
So the two facts are on 0.8.0's path.

**4. What it requires.**

- Facts, in `docs/facts/windows/tokens.toml`, replacing two of the three pending ones:
  - `token.behavior.mnemonics`, `value = "never"`, `published`, the Learn sentence above;
  - `token.behavior.mnemonic_when_disabled`, `value = "absent"`, the same citation, as the `macos-*`, `ios-*` and
    `material-3` facts cite their `mnemonics` fact's source (`docs/facts/macos/tokens.toml:46304-46322`).
  - `token.behavior.single_click_activates` stays pending (U504) until the containers unit decides the token. Its
    comment, "U504's pass reads ListViewBase's IsItemClickEnabled against the model's sense of the token"
    (`:2920-2921`), then names that unit's question instead, since no read of one property settles it (the token's
    part, part 4).
- No specification change.
- Class 1: `[trace-windows]:721`, `reader:windows:SPI_GETKEYBOARDCUES` under W3-48†, is withdrawn (the setting
  governs Win32's underline, R1.3); `[catalogue]:10719`, "`SPI_GETKEYBOARDCUES`: W3-21, W3-48†", loses W3-48†.
- A consequence for U518 (kind `b`): with `never`, `windows-11`'s `button.label.text.mnemonic_decoration`, pending
  there (`docs/facts/windows/button.toml:19029-19032`), draws none, and is stated as S2.8 has it for such a part:
  `"absent"`, or the reference to its style's leaf where the label follows a style (`[spec]:2368-2372`). A facts
  edit, no read.
- Register: the Decision cell records the two mnemonic values. Once their facts are stated, the row narrows to
  `single_click_activates`, its question with the containers unit, and stays kind `d`. Two of its three listed
  paths, `windows-11.toml:356` and `:358`, are then empty. The third, `:360`, leaves 0.8.0's path by the staging
  (the token's part, part 3).

**5. Confidence.** High for the two mnemonic values. A veto would have to put Win32's underline in WinUI's preset,
against R1.3, or give KeyTips a member of `Mnemonics` beside `component.key_tip`.

---

### U509 — `windows-11`: the monospace text role

**1. The row.** `[ledger]:670`: "`windows-11`: `token.text.roles.monospace` (W3-47†) […] | WinUI has no monospace
style | a decision: absent with a citation, or the console font".

- Facts: 1, `docs/facts/windows/tokens.toml:3071`. Listed paths: 1, `windows-11.toml:376`.
- The specification: the nineteen slots are "one field per semantic slot, each filled by every preset (R10)"
  (`[spec]:3609`); R10's decision is "A typed semantic ramp (each preset filling every slot, its facts saying which
  of its styles fills it)" (`[rationale]:1062-1065`). No mark of S6.3 and no row of the catalogue's absence table
  lets a slot be absent (`[spec]:4031-4072`; `[catalogue]:10330-10343`). "A slot is a reference to the platform's
  keyed style (`= token.text.keyed.body`), so a style is stated once" (`[spec]:3631-3633`), and "A keyed style is
  the platform's style" (`[spec]:3640`). The catalogue's draft has the slot filled by a keyed style `monospace` on
  Windows (`[catalogue]:10096`), "sourced as today's `mono_font` is (W3-47†)" (`[catalogue]:10192`). The gap map:
  "WinUI states no monospace role (no XAML file of the MUX tree names Consolas, Cascadia or Courier), but Windows
  draws monospace text in the console host and in Windows Terminal"; 0.7.0's value is a "preset value, no source"
  (`[gap-windows]:373`).
- A precedent for the form: Material names no monospace face either, and `material-3` states a keyed style
  `token.text.keyed.monospace`, its family the system's and every other leaf a reference to the body style's, and
  the slot as `= token.text.keyed.monospace` (`docs/facts/material/tokens.toml:9004-9066`, `:9274-9276`). No fact
  under `docs/facts/` states a leaf of a slot (by search).
- What the pinned sources name. Thirteen sources serve `windows-11` (`docs/facts/sources.toml`), twelve of them
  in the cache; all twelve were searched for the four families. The table gives every hit but a test page
  (`microsoft-ui-xaml@2.5.1 src/controls/dev/CommonStyles/TestUI/CommonStylesPage.xaml.cs:59`) and Courier New's
  hits that are examples of another matter, seven pages on eight lines: a font picker's entry in three Learn pages
  (`windows-dev-docs@f54166ed hub/apps/develop/ui/controls/combo-box.md:106`; `winapps-winrt-api@554b7e3a
  microsoft.ui.xaml.controls/combobox.md:72`; `listbox.md:67`, `:89`), and four GDI pages (`win32@ca8274de
  desktop-src/gdi/font-families.md:19`, `desktop-src/gdi/enumerating-the-installed-fonts.md:15`; `sdk-api@656e9656
  sdk-api-src/content/wingdi/nf-wingdi-createfonta.md:657`, `nf-wingdi-createfontw.md:657`). None bears on the pick.

| Family | Where a pinned source names it | For what use | 0.7.0 |
|---|---|---|---|
| Consolas | `windows-dev-docs@f54166ed hub/apps/design/signature-experiences/typography.md:203`, in the table under "Sans-serif fonts are a great choice for headings and UI elements" (`:195-197`): "Fixed width font that supports European scripts" | a font for the UI of a Windows app, in Microsoft's page on how the Fluent Design language "is applied to Windows apps" (`:20`) | not shipped |
| | `Labs-Windows@251217 components/MarkdownTextBlock/src/MarkdownThemes.cs:89`: `CodeBlockFontFamily { get; set; } = new FontFamily("Consolas")` | code text in a WinUI control, of the Community Toolkit's Labs, which R1.2 keeps out of the model as experimental (`[rationale]:161-163`) | |
| | `fluentui@1.0.0-alpha.24 packages/tokens/src/global/fonts.ts:41`: `fontFamilyMonospace: "Consolas, 'Courier New', Courier, monospace"` | monospace text in Fluent's web toolkit | |
| | `visualstudio-docs@fedd6469 docs/extensibility/inside-the-editor.md:65`: "for example, "Blue Consolas 10 pt"" | an example in Visual Studio's editor documentation | |
| Lucida Console | `microsoft-ui-xaml@2.5.1 src/dxaml/xcp/dxaml/lib/CueStyler.cpp:1699-1700`, in "Converts the caption style into a platform font" (`:1684-1717`): `ClosedCaptionStyle_MonospacedWithoutSerifs` | media closed captions in the user's caption style, in WinUI's own code | not shipped |
| | `src/dxaml/xcp/components/comptree/DCompTreeHost.cpp:1929` | the frame-rate counter, a debug overlay | |
| Courier New | `typography.md:214`, under "Serif fonts are good for presenting large amounts of text" (`:207-209`): "Serif fixed width font" | long text | not shipped |
| | `CueStyler.cpp:1693-1694`: `ClosedCaptionStyle_MonospacedWithSerifs`; `fonts.ts:41`, second in Fluent's stack | captions; a fallback | |
| Cascadia Mono | none: `Cascadia` occurs in none of the twelve checkouts | — | shipped: `Cascadia Mono`, 10.5 pt, 400 (section 2.3) |

  No pinned source states the console's default face either.

**2. The options, for the slot.**

- **(a) Absent, with a citation.** True of WinUI. It changes R10 ("each preset filling every slot"), class 3, the
  maintainer's (`[plan]:1386`); it loses 0.7.0's `mono_font` on `windows-11`, against D13; and outcome 5 rules it
  out for a slot an application needs ("never absent").
- **(b) The console font,** read from `HKCU\Console` (`[trace-windows]:719`). The console host is no WinUI control
  (R1.3); no pinned source gives its default, so the preset would have no value; and which host reads that key on
  Windows 11, "where Windows Terminal is the default", the gap map leaves "to verify" (`[gap-windows]:373`).
- **(c) Outcome 5's labelled mapping: a keyed style `monospace`, the body style in a fixed-width family.**
- **(d) The slot shares `body`'s style.** A monospace slot in a proportional font.
- **(e) A stated fact.** No pinned source makes it one: none says what family a Windows 11 application draws
  monospace text in.

**3. The outcome: (c); the family is a pick.** R10 and outcome 5 exclude (a); R1.3 excludes (b); (d) is false to
the slot's use; (e) has no source. What is native-theme's is the pairing of the slot with a font Microsoft's
pages name for Windows applications, which the label says.

**The pick, by the order the coordinator set.**

1. *A pinned Microsoft source that names the family for monospace text in a Windows 11 application's UI.* None
   does in those words. The nearest in use is the Labs' Markdown control, a WinUI control that draws code in
   Consolas; it is experimental, the Community Toolkit's and not WinUI's, and out of the model by R1.2.
2. *Failing that, the family the pinned sources name for the nearest use.* Consolas, three times: for the UI of a
   Windows app (Learn), for code in a WinUI control (Labs), for monospace text in Fluent's web tokens. Lucida
   Console is named by WinUI itself, for captions and a debug counter, neither an application's text. Courier New
   is the serif one.

**Recommended: Consolas, as a pick listed for the veto, not as a fact of the sources.** It differs from what 0.7.0
shipped.

**What would let Cascadia Mono be chosen.** One pin, better two, which the facts lane can add:

- a Microsoft source for where Windows 11 itself draws monospace text in it: the gap map names Windows Terminal's
  documentation of a profile's font face and its default (`[gap-windows]:373`, "to verify"). Candidates, by name
  only: the terminal's documentation repository (`MicrosoftDocs/terminal`, its page on a profile's appearance), or
  the terminal's own repository (`microsoft/terminal`, its default settings file). The source would have to state
  that the default face of a profile is Cascadia Mono;
- and, so that the preset names a font the platform has, Microsoft's list of the fonts Windows 11 installs (in
  `MicrosoftDocs/typography`), which would have to name the family.

None of these was read (no network; none is pinned): the repositories' names are from memory, and what they
state is to be verified at the pin. With them the
ground would be "the family Windows 11's default terminal draws", which keeps 0.7.0's look. It would still be a
labelled pick: a terminal is no WinUI application's text role.

On the release path: yes, 0.7.0 holds `mono_font` on `windows-11`; the staging note agrees.

**4. What it requires.**

- Outcome 5's batch gives a labelled mapping its form in the facts format and the gate (`[outcome]:16`), and S4.4's
  "A keyed style is the platform's style" (`[spec]:3640`) gains the label's exception.
- Then, in `docs/facts/windows/tokens.toml`, under that label: the keyed style `token.text.keyed.monospace`,
  declared in the preset's `text_styles`, its `font.family` the pick and every other leaf
  `= token.text.keyed.body.<leaf>`; and the slot `token.text.roles.monospace` `= token.text.keyed.monospace`. For
  Consolas the citations are `typography.md:203` (a table cell, which waits on U520's checker notation,
  `docs/facts/windows/tokens.toml:3076-3079`) and, readable by the checker today, `fonts.ts:41`.
- Class 1: `[trace-windows]:719`, the read of `HKCU\Console`, is withdrawn (R1.1, R1.3), and with it
  `[catalogue]:10741`, the readers list's "`HKCU\Console` `FaceName`, `FontSize`, `FontWeight` (host to verify):
  W3-47†"; `[catalogue]:10192`, "`monospace`, sourced as today's `mono_font` is (W3-47†)", and
  `[catalogue]:10539-10540`, "`monospace` with a source on `windows-11` (W3-47†)", each become "a labelled mapping
  (W3-47†)".
- The pairing and the family go on the maintainer's veto list (`[outcome]:33`).
- Register: Decision recorded; the row retires when the mapping is written. The listed path is then empty.

**5. Confidence.** Medium: high that the slot is a labelled mapping, low that any one family is the platform's. A
veto on the family needs nothing from the model; it changes one string.

---

### U515 — `windows-11`: the motion speed factor with animations off

**1. The row.** `[ledger]:674`: "`windows-11`: `token.motion.speed_factor` with and without animations (W3-37,
W3-52†) […] | WinUI changes each animation separately when animations are off, so one factor may not state it | a
decision on the model's reading, then the values".

- Facts: 1, `docs/facts/windows/tokens.toml:2967`. Listed paths: 1, `windows-11.toml:362`.
- The specification and the rationale have already decided the reading. R9: "Which durations the factor scales is
  the platform's, so each scaled duration is stated as a derivation, a token times the factor […] and an unscaled
  one as a plain duration" (`[rationale]:1015-1020`); "under animations off jumps to the end (GN-3.25), which the
  preset states as a rule setting the speed factor to zero. The model never zeroes durations by itself"
  (`[rationale]:1039-1042`). S2.7: "Animations off have one home, the speed factor" (`[spec]:1912-1914`), and of
  WinUI's AnimatedIcon, "one started with `MotionUpdate.speed_factor` zero (animations off), jumps to its end
  (`:431-440`), the factor otherwise scaling nothing" (`[spec]:1697-1698`). The catalogue: the factor is "a constant
  elsewhere (the platform has no speed setting […]). Rules set it to zero under `AnimationsOff` where the platform's
  switch does that (GN-3.25, Q3.25, W3-37, W3-52†, MAT-X20)" (`[catalogue]:10413-10416`), W3-37 among its rows.
  The trace has both lines for W3-37: `PartStyle.transitions @ cond animations_off per animation` and
  `token.motion.speed_factor @ cond animations_off` (`[trace-windows]:771-772`).
- The source, read at the pin. WinUI's clock runs at `initialSpeedRatio = s_slowDownAnimations ? 0.1f : 1.0f`,
  the first only under a developer's runtime feature (`microsoft-ui-xaml@2.5.1
  src/dxaml/xcp/core/animation/timemgr.cpp:293-300`): no user speed setting. With animations off: "Disabling
  animations in response to the user being in accessibility mode is a behavior that we define not at the level of
  individual animations and storyboards, but at the VSM and LayoutTransition layer"
  (`src/dxaml/xcp/components/Collection/Inc/VisualStateGroupCollection.h:101-104`); "When animations aren't enabled
  we skip all non-essential transition storyboards to their end frame"
  (`src/dxaml/xcp/components/vsm/VisualStateManagerActuator.cpp:604-612`); a brush transition is set up only
  `if (dcompTreeHost != nullptr && isAnimationEnabled)` (`src/dxaml/xcp/core/core/elements/Border.cpp:44-49`); an
  AnimatedIcon segment jumps (`src/controls/dev/AnimatedIcon/AnimatedIcon.cpp:431-440`); acrylic's fade takes 1 ms
  (`src/controls/dev/Materials/Acrylic/AcrylicBrush.cpp:105`). So each animation is ended at once, kept, or given a
  time of its own; none is slowed by a fraction.

**2. The options.**

- **(a) `1`, and `0` under `cond animations_off`; the exceptions stated where they are.** A transition the switch
  ends is stated with the factor (or run by an engine that reads it); one it keeps, an essential storyboard, is
  stated plain; one it shortens, acrylic's, states its own value by a rule under the condition, as the trace's
  first line has it. No model change. It is what `adwaita`, `kde-breeze` and `material-3` state
  (`docs/facts/gnome/tokens.toml:659-690`, `docs/facts/kde/tokens.toml:8-37`,
  `docs/facts/material/tokens.toml:1313-1338`).
- **(b) `1` with no rule; every transition its own rule under the condition.** One fact restated per transition,
  and against R9's "one home".
- **(c) `absent`.** No mark allows it, and every `mul(x, token.motion.speed_factor)` would read an absent operand.
- **(d) A new model form,** a flag on a transition for "essential". A plain duration already says "not scaled".

**3. The outcome: (a).** The row's doubt asks of the factor what R9 does not: the factor holds the switch, and
each animation says whether it follows it. WinUI's switch is all or nothing per animation, so the only factor it
applies is zero. On the release path: the factor is how the new model carries 0.7.0's reduce-motion switch to the
two motions outcome 3 keeps (`[spec]:573-575`). Under the staging note it is staged at the slice and returns with
the first of the overlays and indicators units whose value reads it; the note's list has the row leave whole,
which its own rule does not give (section 2.4, (a)). The staging's revision 2 keeps the factor at depth, so the row
stays on 0.8.0's path (`[staging2]`, section 4, `windows-11`, `:810`).

**4. What it requires.**

- Facts, `docs/facts/windows/tokens.toml`, replacing the pending one:
  - `token.motion.speed_factor`, `value = 1`, `published`, `timemgr.cpp:300`;
  - the same key `when = { conditions = ["animations_off"] }`, `value = 0`. WinUI's quotes hold no zero, so the
    form is S17's zero stated by omission in its sense "the code acts at once" (`[spec]:6958-6966`): an absence
    citation over `VisualStateManagerActuator.cpp:604-612`, its head quoted ("When animations aren't enabled we skip
    all non-essential transition storyboards to their end frame"), `absent = []`, with `Border.cpp:44-49` beside
    it. Whether a storyboard ended at once is what that form means is the Windows verifier's to confirm; if it is
    not, the form gains the case by a class 2 line of S17, and the fact waits for it.
- No other specification change. Register: the row retires with the two facts. The listed path is then empty
  (under the 2026-11-10 note it would have left at the slice before that; under its revision 2 it stays until the
  facts).
- A follow-up for the motion unit (section 4, item 2).

**5. Confidence.** High on the reading and the values; the citation form for the zero is the one open detail. A
veto would have to hold that a preset may state the factor zero only where the platform ends every animation;
WinUI keeps the ones it marks essential, which (a) states as plain durations.

---

### U681 — `windows-11`: which icon sizes the keyed map holds

**1. The row.** `[ledger]:682`: "`windows-11`: `token.icons.sizes.keyed` […] | WinUI sets icon sizes per control,
while the Win32 shell's image lists have more sizes (`SHGetImageList`'s extra-large 48 and jumbo 256): which, if
any, are the keyed map's is a decision, beside U516 | that decision, then the facts".

- Facts: 1, `docs/facts/windows/tokens.toml:2643`. Listed paths: 1, `windows-11.toml:310`.
- The specification: `keyed` holds "the platform's further contexts as keys" (`[spec]:3676`). The traces send
  GNOME's, KDE's and macOS's rows to `token.icons.sizes.keyed.<context>` (`[trace-gnome]:1015-1016`,
  `[trace-kde]:1121`, `[trace-macos]:488-494`); the Windows trace has no line with `token.icons.sizes` at all (by
  search), and sends W3-27, "Glyph sizes per control", to the parts: `checkbox.mark.glyph.size;
  expander.arrow.glyph.size; […]` (`[trace-windows]:706-707`). `material-3` states the map empty, "every other icon
  size is a component's own token […], stated on its part" (`docs/facts/material/tokens.toml:1871-1883`).
- The source: every icon or glyph size resource of WinUI's control dictionaries is named for one control
  (`CheckBoxGlyphSize`, `ExpanderChevronGlyphSize`, `ScrollBarButtonArrowIconFontSize`, `AutoSuggestBoxIconFontSize`,
  `InfoBarIconFontSize`, `TabViewItemHeaderIconSize`, …: twenty names, by a search of
  `src/controls/dev/**/*themeresources*.xaml` for `x:Double` keys that hold `Icon` or `Glyph` and end in `Size`,
  `Height` or `Width`), and the shared dictionary, `src/controls/dev/CommonStyles/Common_themeresources_any.xaml`
  (607 lines), holds neither word anywhere.

**2. The options.** (a) The map empty, `{}`, each per-control size on its part. (b) The Win32 shell's image-list
sizes as keys: another toolkit's values (R1.3), and file icons of the shell, which no WinUI control draws. (c) One
key per WinUI per-control resource: a second home for what the trace already gives the parts, and nothing shared
to name.

**3. The outcome: (a),** by R1.3 and by the trace's own mapping of W3-27. Not 0.7.0's. The staging note counts the
row as staged until the names are read (section 2.4, (c)); this is that read.

**4. What it requires.**

- Fact: `token.icons.sizes.keyed`, `value = {}`, an absence citation (`[spec]:6931-6938`) over
  `microsoft-ui-xaml@2.5.1 src/controls/dev/CommonStyles/Common_themeresources_any.xaml:1-607`, its head quoted,
  `absent = []`, as `material-3`'s fact for the same key has it (`docs/facts/material/tokens.toml:1874-1883`). A
  name list would check nothing here: the checker matches a name "at identifier boundaries"
  (`scripts/check_fact_citations.py:29`), so a part of a name such as `GlyphSize` never matches. A comment says
  that the per-control sizes are their parts' (W3-27).
- Catalogue, class 1: `[catalogue]:10551`, "contexts beyond the five groups (GN-3.59, K3.63, W3-27) are keys of the
  same token", loses W3-27, which the trace maps to parts.
- Register: the row retires with the fact. The listed path is then empty. U516 (kind `b`, the typed contexts) is
  untouched.

**5. Confidence.** High. A veto would need a WinUI source that names an icon size shared by several controls;
none was found.

---

### U555 — `material-3`: single-click activation

**1. The row.** `[ledger]:687`: "`material-3`: `single_click_activates` […] | Compose's lists have no activation
of their own | a decision".

- Facts: 1, `docs/facts/material/tokens.toml:10059`. Listed paths: 1,
  `native-theme/src/presets/material-3.toml:290`.
- The token: moved by decision C to the containers unit (the token's part, part 3). Material's evidence is in that
  part's table (part 4), with check 3's two corrections: the double-click parameter is that of the overload
  `ListItem.kt:1602-1611` calls (`Clickable.kt:551`, documented at `:539`, the overload `:543-553`), not of the
  hidden one at `:330`; and "Adds a list of items." is at `LazyDsl.kt:155`, the function at `:169`.

**2. The options,** as the record for that unit.

- **(a) `true`.** Compose's lazy list is an item view by reading (i), with no activation of its own, and
  Material's documented item with an action runs it on one click and wires no double click.
- **(b) `absent`.** Revision 2's outcome, under reading (ii). Against rule 1: `absent` needs a citation that the
  platform has none, and the pinned list item and sample show the behaviour. The citation it proposed
  (`LazyDsl.kt:470-483`, `absent = ["onClick", "selected"]`) shows only that the lazy list takes neither as a
  parameter.
- **(c) `false`.** It would say a double click activates, which Compose's list item never wires.
- **(d) A value of `list` per variant** (the token's part, part 6, (B)). Material's list items differ by overload,
  not by gesture: a click runs the action, and the selection overloads toggle (`ListItem.kt:386`, `:470`).

**3. No outcome this round** (decision C). The row stays, kind `d`. Its one fact stays pending, and its question
goes to the containers unit. Not 0.7.0's, and parked by the staging (`[staging2]`, section 3.1, `:701`; section 4,
`:814-815`).

**4. What it requires now.** Nothing. If the containers unit takes (a), this is the fact as revision 3 drafted it,
its citations verified for this revision:
- `token.behavior.single_click_activates`, `value = true`, `published`, three citations:
  - `LazyDsl.kt:429` (quote `The vertically scrolling list that only composes and lays out the currently visible
    items.`);
  - `ListItem.kt:304` (quote `This overload of [ListItem] handles click events, calling its [onClick] lambda to
    trigger an`);
  - `ListItem.kt:1602-1611` (quote `Modifier.combinedClickable(`), with `absent = ["onDoubleClick"]`, which a
    `published` citation may carry (`[spec]:6812`) and the checker verifies absent from those lines.
- A `true` holds no literal for the quote check (`scripts/check_fact_citations.py:602-603`).
- The listed path, `material-3.toml:290`, leaves 0.8.0's path by the staging before that.

**5. Confidence.** None is claimed: no outcome.

---

### U559 — `material-3`: the icon theme and its fallback

**1. The row.** `[ledger]:691`: "`material-3`: the icon theme and its fallback […] | Material names no icon theme |
a decision: absent with a citation".

- Facts: 2, `docs/facts/material/tokens.toml:10147`, `:10152`. Listed paths: 2, `material-3.toml:300`, `:302`.
- The specification has decided it. The catalogue's absence table: `token.icons.theme` may be `Absent` on "each
  preset whose `token.icons.set` is not `freedesktop`: `macos-*`, `ios-*`, `windows-11`, `material-3`", because
  "they draw one fixed symbol set and have no named icon theme the freedesktop lookup could use (0.7.0's presets
  repeat the set's name […])" (`[catalogue]:10334`); `IconTokens.theme_fallback` may be absent by the core's mark
  (`[spec]:4063`). The gap map: "no icon theme; the icon set and its axes are MAT-X27" (`[gap-material]:582`).
  `macos-*` already state both `absent`, citing the class that would hold the name with `absent = []`
  (`docs/facts/macos/tokens.toml:46477-46510`).

**2. The options.** (a) Both `absent`, cited. (b) The set's name again, `"material"`, as 0.7.0 has it: the
catalogue's row supersedes exactly that, and the name is already `token.icons.set`
(`docs/facts/material/tokens.toml:1841-1848`). No third option arises.

**3. The outcome: (a).** It is the catalogue's. On the release path: 0.7.0 holds `icon_theme`. What it held there
on `material-3` is the set's name, which `token.icons.set` keeps; a consumer that read `icon_theme` reads `absent`
in 0.8.0.

**4. What it requires.**

- Two facts, each `value = "absent"`, an absence citation over the icon token set, which holds a font and a size
  and nothing else (`material-web@2.5.0 tokens/_md-comp-icon.scss:13-27`, quote `$supported-tokens: (`,
  `absent = []`, as the keyed-map fact beside them cites it).
- The plan, an amendment of the maintainer's list: D13's sentence on what users lose (`[plan]:496-499`) gains the
  icon theme's name on the presets of `[catalogue]:10334`, where 0.7.0 repeated the set's name. No specification
  change.
- Register: the row retires. Both listed paths are then empty.

**5. Confidence.** High. A veto would have to reverse the catalogue's row for all six non-freedesktop presets.

---

### U561 — `material-3`: the small, large and panel icon contexts

**1. The row.** `[ledger]:693`: "`material-3`: the small, large and panel icon contexts […] | no token names these
sizes | a decision: absent with a citation".

- Facts: 3, `docs/facts/material/tokens.toml:10362`, `:10367`, `:10372`. Listed paths: 3, `material-3.toml:306`,
  `:308`, `:310`.
- The catalogue's absence table does not treat the three alike. `small` may be absent on `material-3` ("no small
  icon token"), `large` on `macos-*` and `material-3`; `panel` only on `adwaita`, `macos-*` and `windows-11`
  (`[catalogue]:10336-10339`). The gap map agrees: "no small or large icon context; icon sizes are per component"
  (`[gap-material]:581`), and for the drawer's icons, "size is `defaults.icon_sizes.panel_px`"
  (`[gap-material]:184`). 0.7.0's `material-3` states `panel_px = 24.0` from "comp.navigation-drawer icon-size 24"
  and gives `small_px` and `large_px` no citation (`v0.7.0:native-theme/src/presets/material-3.toml:76-82`).
- The source at the pin: "md.comp.navigation-drawer.icon.size (Enabled / Icon) `$icon-size: 24px;`"
  (`material-web@2.5.0 tokens/versions/latest/sass/_md-comp-navigation-drawer.scss:34-35`). The lane already
  states `toolbar` from the app bar's icon size and `dialog` from the dialog's
  (`docs/facts/material/tokens.toml:1851-1869`): a typed context is filled from the component token that names
  that place.

**2. The options.**

- **(a) `small` and `large` absent; `panel` stated, 24px.** The catalogue's table as it stands; keeps 0.7.0's one
  sourced value of the three.
- **(b) All three absent,** the row's suggestion. For `panel` it needs the catalogue's table changed (class 1)
  and drops a value 0.7.0 holds with a source, against D13.
- **(c) `small` and `large` stated from some component's icon size** (the extra-small icon button's 20px, the
  large one's 32px, `_md-comp-icon-button-xsmall.scss:27-28`, `_md-comp-icon-button-large.scss:27-28`). No source
  ties those components to the two contexts. As a fact, rule 1 does not admit it. As a labelled mapping, outcome 5
  does not reach it: that rule is for "a typed role or slot", which no mark lets be absent and which the toolkits
  hold as values that cannot be left empty (`[outcome]:16`); an icon context the catalogue marks `may_be_absent`
  (`[catalogue]:10336-10337`), and an application that asks for one a platform does not name is told so.
- **(d) 0.7.0's 16 and 32.** No source; 0.7.0's own platform facts call both "(none)"
  (`platform-facts.md@2796b26e:1137-1138`).

**3. The outcome: (a).** Rule 1 excludes (c) as a fact and (d); the catalogue excludes (c) as a mapping; D13 and
the catalogue exclude (b) for `panel`. On the release path: yes. Against 0.7.0, `material-3` loses `small` and
`large`, which 0.7.0's connectors handed to applications (section 2.3) and which had no source. Rule 1 comes
before D13, so the loss stands, and it has to be recorded where the losses are listed.

**4. What it requires.**

- Facts: `token.icons.sizes.small` and `.large`, each `value = "absent"`, an absence citation over
  `material-web@2.5.0 tokens/_md-comp-icon.scss:13-27` with `absent = []`, as `adwaita` states its absent
  contexts (`docs/facts/gnome/tokens.toml:7933-7950`); `token.icons.sizes.panel.size`, `value = "24px"`,
  `published`, the drawer token above, and the context's other leaves as the `dialog` context states its own
  (`docs/facts/material/tokens.toml:10265-10358`).
- The plan, an amendment of the maintainer's list: D13's sentence on what users lose (`[plan]:496-499`) gains
  "`material-3`'s small and large icon sizes, which 0.7.0 stated with no source (16 and 32) and Material names no
  context for". The same table leaves other contexts absent that 0.7.0 stated, on `adwaita`, `macos-*`,
  `windows-11` and `ios-*` (section 4, item 7), so the sentence is written for the class: "an icon context the
  platform names none for, which 0.7.0 stated (the catalogue's absence table, `[catalogue]:10336-10339`)", its
  extent decided by U102 (`macos-*`), U516 (`windows-11`) and U127 (`ios-*`), which the amendment names.
- No specification change. Register: the row retires. All three listed paths are then empty.

**5. Confidence.** High for `small` and `large`; medium-high for `panel`. A veto would turn on whether a navigation
drawer is what the model means by "panel". The catalogue and 0.7.0 both say so; no sentence of the specification
defines the context.

---

### U1 — `windows-11`: which view state a toolbar button's facts state

**1. The row.** `[ledger]:487`: "[…] narrowed to `tool` in `context toolbar.bar` outside the tabbed command bar:
`button.body.padding.*`, `margin.bottom` and `min_height` (6 pending facts), where a closed CommandBar draws the
Compact view state (inner border 2,6,2,22, the label collapsed: the host's `command_bar` pass) and an open one
FullSize, which the context does not tell apart | a decision on which view state `context toolbar.bar` states
(the resting closed bar's Compact, with the label's collapse through `label_position`, U517 […]), then the facts
by S2.11: 10 above and below, `sub(0px, 2px)` across, zero by S1".

- Facts: 6, `docs/facts/windows/button.toml:1026`, `:1032`, `:1038`, `:1044`, `:2175`, `:2181`. Listed paths:
  none.
- The model does tell the two apart. A context match carries the ancestor's states, `ContextMatch { …, variants,
  states }` (`[spec]:4094-4099`). The toolbar uses the state flag `open`, "W1-103's open command bar"
  (`[catalogue]:4698`), and has the keyed variants `@compact`, `@minimal`, `@hidden`, "the closed display modes"
  (`[catalogue]:4686-4687`); the catalogue's rule for flags: "A nested *(element)* part, and the plain parts that
  belong to it, read any other flag of their host by `context` (`expander.arrow.glyph @ context expander.body
  {open}`" (`[catalogue]:325-326`). The trace has `toolbar.bar.{fill,stroke,corners} @ {open}` for W1-103
  (`[trace-windows]:308`).
- The source: `BOOLEAN shouldBeCompact = !isOpen;` then `element->put_IsCompact(shouldBeCompact)`
  (`microsoft-ui-xaml@2.5.1 src/dxaml/xcp/dxaml/lib/CommandBar_Partial.cpp:1850-1853`, `:1866`); the closed display
  mode's default is `AppBarClosedDisplayMode_Compact`
  (`src/dxaml/xcp/components/DependencyObject/DependencyProperty.cpp:455-456`); the Compact state sets the inner
  border's margin and collapses the label (`src/controls/dev/CommonStyles/AppBarButton_themeresources.xaml:147-156`,
  `:118`).
- What 0.8.0 needs of the context: 0.7.0's toolbar font and icon size map to `button.label.text @ context
  toolbar.bar` and `button.icon.icon.size @ context toolbar.bar` (`[plan]:732`; `[catalogue]:4729-4731`), in the
  containers unit of 0.8.0 (`[plan]:773-775`).

**2. The options.**

- **(a) `context toolbar.bar` is the closed bar: Compact. The open bar is `context toolbar.bar {open}`:
  FullSize.** A command bar rests closed, and its buttons are compact exactly while it is not open. No model
  change.
- **(b) `context toolbar.bar` is FullSize, and Compact is the content variant `icon_only`,** as the trace's W1-099
  line reads, "(labelled and compact)" (`[trace-windows]:293`). The closed bar's button has a label that the bar
  hides; `icon_only` is a content variant, one the query reads off what the element holds (`[spec]:4491-4494`),
  and a part the content lacks "is simply not queried" (`[spec]:3994-3996`), so it cannot stand for the bar's state.
- **(c) A new keyed variant or condition for the view state.** The `open` flag is that state.

**3. The outcome: (a), decided now.** It is the row's own proposal, with one correction: the model does tell the
two states apart, so FullSize is not dropped but stated under `{open}`. The decision is needed by the containers
unit of 0.8.0: what that context states on `windows-11` (the closed bar, its label collapsed) decides how the
toolbar's label and icon facts there are written. The six pending facts are beyond 0.7.0's depth (0.7.0 holds no
padding of a `tool` button in a toolbar); they are on 0.8.0's path if the staging design's comparison per unit
brings `tool` to depth in that context, and then with the containers unit, on every preset (section 2.4, (d)).
The staging's revision 2 does: U1 leaves at the slice and returns with the containers unit, where `tool` and
`toolbar.bar` are both at depth (`[staging2]`, section 4, `:830-832`). The decision stands either way: the toolbar's label and icon facts are written against the closed bar whichever
variant set states them.

**4. What it requires.**

- With the containers unit: `button.values.label_position` in `context toolbar.bar` `icon_only`, and under `{open}`
  `text_under_icon` (U517's); the toolbar's label and icon facts written against the closed bar as the context's
  base.
- With the unit that holds `tool` in that context: the six pending facts stated for Compact by S2.11 as the row
  has it (the host's `command_bar` pass checks them); the FullSize values under
  `context = [{ widget = "toolbar", part = "bar", states = ["open"] }]` (S7's `ContextMatch.states`; no fact
  uses a context's state yet, so the spelling is the lane's to confirm against S17).
- Trace, class 1: `[trace-windows]:293` reads "`@ variant tool context toolbar.bar` (compact) `/ variant tool
  context toolbar.bar {open}` (labelled)".
- Register: the Decision cell records (a) now; the row stays until the facts are stated. It holds no listed path.

**5. Confidence.** High. A veto would have to hold that a toolbar's base look is the open bar.

---

### U424 — `kde-breeze`: the command link's subtitle gap

**1. The row.** `[ledger]:628`: "`kde-breeze`: `button.values.subtitle_gap` under `@command_link` […] | the
description is centred in the space below the title | a model form for that, or a measurement".

- Facts: 1, `docs/facts/kde/button.toml:13329` (`when` not `flavor:kirigami`, variant `@command_link`). Listed
  paths: none: the base fact states `absent` (`:13316-13326`).
- The catalogue: `subtitle_gap_px: Value<Px>` is "iOS `titlePadding` (B-8)" (`[catalogue]:1624`), and may be absent
  on "all but `ios-*`" (`[catalogue]:2023`). The trace's K2.7 lines name the command link's fill, stroke, icon,
  title and description, and no gap (`[trace-kde]:763-767`).
- The source: Breeze paints the command link itself. The text rectangle is the button less a margin on each side,
  `button->size().height() - 2 * margin` (`breeze@6.8.2 kstyle/breezestyle.cpp:2031`, the margin
  `Button_MarginWidth + Frame_FrameWidth`, `:2007`); the title is drawn `AlignTop` in it (`:2041`); then
  `textRect.setTop(textRect.top() + QFontMetrics(font).height())` (`:2042`), and the description is drawn
  `Qt::AlignLeft | Qt::AlignVCenter | Qt::TextWordWrap` in what is left (`:2049`). The button's height is Qt's size
  hint or whatever its layout gives it (`qtbase@6.12.0 src/widgets/widgets/qcommandlinkbutton.cpp:305-316`). So the
  gap is half of what the button's height leaves after the margins, the title's line and the description's lines:
  it has no fixed value.

**2. The options.**

- **(a) A measurement.** One number, true at one height and one pair of fonts; the value follows the element's
  height, which "cannot be stated as a result" (R5.4, `[rationale]:621-622`).
- **(b) `absent`,** which the catalogue allows here. True that Breeze states no gap; but then nothing states where
  the description is drawn, and a connector would put it under the title, which differs from Breeze wherever the
  button is taller than its text (rule 2).
- **(c) An expression.** It needs the description's laid-out height, which no operand gives (`[spec]:281-297`), and
  would go negative where the text overflows.
- **(d) The value may be `auto`,** its meaning stated once for every preset: the title lies at the top of the
  body's content box and the subtitle is centred in the space left below the title's line. `Auto` is "the platform
  sizes by content" (`[spec]:240-243`), allowed on the gaps the catalogue marks (`[spec]:4027-4028`); no new field.
- **(e) A new value,** an alignment of the subtitle. A field every preset would state for one platform's control.

**3. The outcome: a model form, (d); never a measurement.** Rule 1 excludes (a); rule 2 excludes (b); (d) is
smaller than (c) and (e). Not on 0.8.0's path: `@command_link` is beyond 0.7.0's depth (the staging note's styles
unit), so the form is applied with the unit that brings the variant.

**4. What it requires.** With that unit, a class 2 batch: the catalogue's `subtitle_gap_px` line
(`[catalogue]:1624`) gains "`auto`: the title at the top of `body`'s content box, the subtitle centred in the space
left below the title's line; stated by `kde-breeze` under `@command_link` (`breeze@6.8.2
kstyle/breezestyle.cpp:2031-2049`)". The meaning is the value's, not a preset's: a resolved `Length::Auto` reaches
every connector (`[spec]:3861-3862`). Then the fact: `value = "auto"`, `published`, those lines. Until then the
fact stays pending and the row stays, its Decision cell recording "a model form, `auto`; with the unit of
`@command_link`". It holds no listed path.

**5. Confidence.** Medium. That a measurement cannot state it is certain; the form is one of three that would. A
veto would turn on whether `auto` may carry a layout rule, or a field of its own should.

---

### U697 — `kde-breeze`: the label position of a tool button with an icon

**1. The row.** `[ledger]:708`: "`kde-breeze`: `button.values.label_position @ variants {tool, with_icon}` […] |
QToolButton draws its icon and text beside or under each other by its `toolButtonStyle` […], and QQC2's ToolButton
by its `display`; `with_icon` does not fix which, and in a toolbar the toolbar's style sets it (K1.153); the
existing fact `icon_only` (`qtbase@6.12.0 src/widgets/widgets/qtoolbutton.cpp:187`, the default style) would apply
to the set without this pending fact | a catalogue split of `with_icon` on a tool button by position (beside,
under) […]; or the set's fact stated per toolbar context from the toolbar's style (K1.153)".

- Facts: 1, `docs/facts/kde/button.toml:13218`. Beside it: the base `text_beside_icon` (`:13192`) and
  `icon_only` under `tool`, with no flavour condition (`:13204`). Listed paths: none.
- The catalogue: `label_position` is a value, "which of icon and text a tool button shows, and how […], by rules in
  `context toolbar.bar`" (`[catalogue]:1622-1623`). The trace has three lines: per toolbar kind (K1.153,
  `[trace-kde]:313`), `@ variant tool cond flavor:kirigami` (Q1.9, `:468`) and the Kirigami toolbar's (Q1.76,
  `:603`).
- The source. Qt Widgets: `toolButtonStyle = Qt::ToolButtonIconOnly;` (`qtbase@6.12.0
  src/widgets/widgets/qtoolbutton.cpp:187`), taken unless the application sets another (`:262-265`); under it no
  text is drawn: `bool hasText = toolButtonStyle != Qt::ToolButtonIconOnly && !toolButtonOption->text.isEmpty();`
  (`breeze@6.8.2 kstyle/breezestyle.cpp:5551`). Qt Quick: `QQuickAbstractButton::Display display =
  QQuickAbstractButton::TextBesideIcon;` (`qtdeclarative@6.12.0 src/quicktemplates/qquickabstractbutton_p_p.h:124`),
  which the Kirigami style hands to the style item as `Qt.ToolButtonTextBesideIcon` where the button has an icon
  (`qqc2-desktop-style@6.31.0 org.kde.desktop/ToolButton.qml:65-82`).

**2. The options.**

- **(a) The row's first: split `with_icon` by position in the catalogue.** It turns a value an application sets
  into variants: the position is `label_position` itself, which an application's own rule can state.
- **(b) The row's second: state it per toolbar only.** It leaves the tool button outside a toolbar unstated.
- **(c) State the defaults the sources give, and no more.** A tool button with an icon and a text shows, by
  default, the icon alone under Qt Widgets and the text beside the icon under Kirigami; another position is the
  application's property; in a toolbar the toolbar's style decides, K1.153's rule, with the toolbar's unit.

**3. The outcome: (c).** Nothing is open to decide. Under Qt Widgets the fact the row calls unwanted, `icon_only`,
is the truth for the set: Qt draws no text under its default style. Not on 0.8.0's path (`tool` is beyond 0.7.0's
depth; the staging note defers the value with it), and a facts edit in any case.

**4. What it requires.**

- Facts, `docs/facts/kde/button.toml`: the pending fact at `:13218` is removed (the `tool` fact holds for the
  set); a new fact `button.values.label_position`, `when = { conditions = ["flavor:kirigami"], variants =
  ["tool"] }`, `value = "text_beside_icon"`, `published`, the two Qt Quick citations above, which is the trace's
  Q1.9 line and which the facts lack today.
- No specification change. Register: the row retires with the edit. It holds no listed path.

**5. Confidence.** High. A veto would have to hold that the model should name, as variants, the positions an
application can give one button.

---

### U314 and U614 — `macos-tahoe`, `macos-monterey`: the help button's question mark

**1. The rows.** `[ledger]:585`: "`macos-tahoe`: `button.icon.glyph` under `help` (the help bezel's question mark:
its source, size and fill per state; MAC-N10) […] | no source: AppKit draws the glyph with the bezel and names no
image for it (`NSButton.h:20-181`, `NSButtonCell.h:81-128` at xcode@26.6), S15's `SystemControl` has no member for
the help bezel, and no analysis reads the glyph from the button pass's `styles_b` help captures | a decision on the
glyph's source (an S15 rendering member for the help bezel, or a path measured from the captures), then its size
and fill measured per state from the button pass's captures". `[ledger]:617`: "`macos-monterey`: the help glyph, as
U314 on `macos-tahoe` […] | as U314, at xcode@16.4 | as U314".

- Facts: 1 each, `docs/facts/macos/button.toml:220991`, `:227000`. Listed paths: `macos-tahoe.toml:174`,
  `"widgets.button.icon.glyph" = ["U314", "U323"]`; `macos-monterey.toml:200`, the same path, `"U614"` alone.
- The rationale has decided the question in general. R11: "A glyph is the platform's own, from one of five sources:
  a code point of its symbol font […]; an icon of its icon theme by name; a vector path taken from its source code
  […]; a system image the platform names for that purpose […]; or, where the platform names none, the system's own
  rendering of the whole control" (`[rationale]:1095-1098`). And: "Measuring the mark's geometry from renders was
  weighed and rejected: its outline can only be traced from a picture, which the project's asset rule forbids"
  (`[rationale]:1117-1118`). S15: "The list of `SystemControl`s is the traceability's rows that need it"
  (`[spec]:6368`); on such a source the glyph's `size` and `fill` are absent (`[spec]:4037-4040`).
- The source, the same at both pins: `NSBezelStyleHelpButton` is "A circular button with a question mark providing
  the standard Help button appearance" (`xcode@26.6 AppKit/NSButtonCell.h:45-46`; `xcode@16.4
  AppKit/NSButtonCell.h:45-46`); of the 139 image names `NSImage.h` declares at each pin (153 lines hold
  `NSImageName`), none names help or a question mark (by search).

**2. The options.**

- **(a) A new S15 member for the help bezel,** the bezel and its question mark as AppKit draws them, as
  `AppKitPushDisclosure` is "the bezel and chevron" (`[spec]:6323-6324`). R11's fifth source. Cost: class 2 (a
  member of `SystemControl`, `native-theme/src/blocks/icon.rs:158`, its rendering under the `unsafe` exception the
  maintainer approved for S15, `[spec]:6357-6359`); without the rendering no mark is drawn, the limit R11 already
  records for the checkbox's mark.
- **(b) A path measured from the captures.** R11 rejected it, by the asset rule.
- **(c) A named image, an SF Symbol** (`questionmark`, or `questionmark.circle`, which 0.7.0's icon role `Help`
  names for a help icon). No pinned source says the help bezel draws either; R11's fourth source is "a system image
  the platform names for that purpose".
- **(d) `absent`.** False: the mark is drawn.

**3. The outcome: (a), on both presets.** R11 leaves nothing else. Not on 0.8.0's path: `help` is a variant beyond
0.7.0's depth (the staging note's styles unit), so the member comes with the unit that brings it.

**4. What it requires.**

- With that unit, a class 2 batch: S15's `SystemControl` gains `AppKitHelp`, "the bezel and the question mark";
  S15's parameter and cache rules cover it as they stand. Trace, class 1: `[trace-macos]:610`,
  `button.icon.glyph.fill @ variant help {pressed}`, loses the glyph's fill (absent on a `SystemControl` source);
  the body's measured fill, stroke and corners (`:608`) stay for a toolkit that cannot composite.
- Facts then: `button.icon.glyph` under `help`, its source `{ system_control = { control = "app_kit_help",
  replaces = […] } }`, `size` and `fill` `"absent"`; the row's "size and fill measured per state" falls away. U708
  (whether AppKit draws the S15 controls differently under the pointer or mirrored, `[ledger]:717`) gains the member.
- Register: both Decision cells record (a); the rows stay until the facts are stated. Under the staging note both
  listed paths leave before that: `help` is deferred, and so is `route`, which U323 holds on `macos-tahoe`'s path.

**5. Confidence.** High. A veto would have to reopen R11's rejection of measured outlines, or accept an SF Symbol
that Apple does not name for the purpose.

---

### U122 — `ios-13`, `ios-26`: the buttons that are no configuration

**1. The row.** `[ledger]:517`: "the forms that are no configuration, where no reading exists: `@paste_control`
(every key); `route` in every state but rest; `@info`, `@contact_add`, `close` for the configuration's keys
(corners, values, subtitle, menu indicator, label on the glyph-only ones) and `close`'s glyph colour while pressed
or selected […] | captured, but the capture cannot give these: UIPasteControl draws in a remote slot view […]; run
41020880059 drew the route picker at rest only; the UIButton types have no configuration, so its keys have no
reading; a glyph over a bezel solves only as their composite | a host pass reading UIPasteControl's own
configuration and drawing the route picker in each state; a model decision on the configuration keys of a UIButton
type".

- Facts: 564, all in `docs/facts/ios/button.toml` (`u122_breakdown.py`): by form, `route` 140, `@paste_control`
  122, `close` 101, `@info` 98, `@contact_add` 98, and 5 under the flag `route_active`, which the state model draws
  with `route` alone (`docs/facts/ios/state-models.toml:204`); by part, `label` 141, `subtitle` 131,
  `menu_indicator` 100, `values` 50, `body` 122, `icon` 20.
- Listed paths: 5 in `ios-13.toml` (`:153`, `:155`, `:157`, `:171`, `:175`) and 10 in `ios-26.toml` (`:153`-`:167`,
  `:181`, `:185`), each naming U122 among seven to thirteen ids.
- The specification: `may_be_absent` "is about what a platform draws, never about an element's content: a part the
  element's content lacks […] is simply not queried by the connector […]. So no fact states such a part absent
  where a content variant leaves it no content (a button's `label` under `icon_only`), and its field facts name
  that content variant in neither `variants` nor `not_variants` […], so nothing there is pending or measured";
  rejected there: "facts pending there (a measurement of nothing drawn)" (`[spec]:3994-4003`). The two indicator
  values already have "a base holding where no indicator is drawn too" (`[catalogue]:1625-1627`).
- The headers, read at the pin (`xcode@26.6`).
  - A subtitle and an indicator exist only in a button's configuration: `NSString *subtitle`
    (`UIKit/UIButtonConfiguration.h:135`), `UIButtonConfigurationIndicator` (`:66-73`); `UIButton.h` declares no
    setter for either. A button has a configuration only where the application gives it one (`UIButton.h:67-68`).
  - `UIPasteControlConfiguration` holds a display mode, a corner style and radius, an image placement and two
    colours, and neither a subtitle nor an indicator (`UIKit/UIPasteControl.h:27-35`); its display modes include a
    label (`:20-24`).
  - The route picker is no button and declares no title (`@interface AVRoutePickerView : AVKitPlatformViewClass`,
    `AVKit/AVRoutePickerView.h:83`; no `title` in the file).
  - A title is another matter: `- (void)setTitle:(nullable NSString *)title forState:(UIControlState)state;`
    (`UIKit/UIButton.h:119`) is declared on `UIButton` for every `UIButtonType` (`:21-34`), the info, contact-add
    and close types included. Whether UIKit draws a title given to such a button, no pinned source says.

**2. The options, for the row's decision part.**

- **(a) The forms state the configuration's values.** They have none.
- **(b) The parts absent under those variants.** The catalogue does not allow it on `ios-*` (`[catalogue]:2001`,
  `:2023`), and S6.3 rejected it for content: "it is the content's".
- **(c) S6.3's content rule, for what the headers show a form cannot hold.** A part the control cannot be given
  content for is content it lacks: not queried, its facts not naming the form, nothing pending. That covers
  `subtitle` and `menu_indicator` under all five forms; the three values that only place them (`subtitle_gap`,
  `menu_indicator_gap`, `menu_indicator_anchor`); and, for the route picker, its `label` and the two values that
  place an icon beside one (`icon_placement`, `icon_text_gap`).
- **(d) The same rule for the three button types' titles,** as revision 1 had it. The header is against it: an
  application can give those buttons a title. Removing a pending fact lets the combination fall to a more general
  fact and pass the gate silently; where the platform may draw something, rule 2 forbids that.
- **(e) Leave all 564 pending until a pass.** For those of (c) it is "a measurement of nothing drawn".

**3. The outcome: (c); everything else is a measurement, kind `b`.** By the rule as written in
`u122_removable.py`: 294 of the 564 need no fact (`subtitle` 131, `menu_indicator` 100, the three placing values
30, the route picker's `label` 29, of which one under `route_active`, and its two icon values 4). 270 stay pending:
`body` 122, `icon` 20, `label` 112 (the paste control's 28; the three button types' 84) and 16 values
(`icon_placement` and `icon_text_gap` of the paste control, 4, and of the three types, 12). The check counts 293;
the one fact of difference is the route picker's `label` under `route_active`, which the check itself notes as
treated unlike the `subtitle` beside it. Not on 0.8.0's path: all five forms are beyond 0.7.0's depth (the staging
note's identities).

**4. What it requires.**

- No specification change is required: S6.3's first sentence covers "a part the element's content lacks" as it
  stands. If the orchestrator wants the lanes' reading fixed in words, it is one clause after "(a button's `label`
  under `icon_only`)" at `[spec]:3997`, class 2.
- Facts, with the forms' unit: the 294 pending facts are removed, and the five forms leave the `not_variants`
  lists of the same parts' other facts, so that those hold "as in the same combination without the variant"
  (`[spec]:3998-3999`) and no combination is left without a fact.
- Register: U122 is narrowed to the 270 and becomes kind `b`, two passes: the one the row names (the paste
  control's own configuration, the route picker in each state) and one that gives each of the three button types
  a title and reads what UIKit draws. If that pass shows no title drawn, its 96 facts go as the 294 did.
- Its listed paths do not empty by it: each is shared with other rows.

**5. Confidence.** Medium-high: the 294 rest on the headers; the 96 are kept pending for want of one. A veto would
turn on whether a part a control cannot hold may be treated as content lacking without a clause that says so.

---

### U651 — `ios-13`, `ios-26`: the accent's foreground role

**1. The row.** `[ledger]:681`: "`role.global.accent_foreground` […] | UIColor names no colour for content on the
tint | a source, or a decision that the filled style's measured title colour is this role".

- Facts: 1, `docs/facts/ios/roles.toml:3974`. Listed paths: 4, `ios-13.toml:287`, `:289`; `ios-26.toml:297`,
  `:299`.
- The catalogue: `accent_foreground` is "text/glyph on the accent (today's `accent_text_color`)", required on every
  preset (`[catalogue]:9929`). `macos-*` state it from a colour AppKit names, `alternateSelectedControlTextColor`
  (`docs/facts/macos/roles.toml:3391-3396`); `adwaita` from `accent_fg_color` (`docs/facts/gnome/roles.toml:948-955`).
- The source: UIKit's header gives the filled style no named foreground; `baseForegroundColor`, set to nil, "will
  cede full control to the configuration to select a color appropriate to the style"
  (`xcode@26.6 UIKit/UIButtonConfiguration.h:111-112`).
- The measurement: the filled style's title at rest is `#ffffff` in both schemes on both presets
  (`docs/facts/ios/button.toml:61017-61018`, `:61463-61464`, `:208688-208689`, `:209134-209135`, each measured on
  the iOS host). 0.7.0's `ios-13` states the same value (section 2.3).

**2. The options.**

- **(a) A source.** None exists: the header says so.
- **(b) The role is the filled title's measured colour, stated as a platform fact.** It would record as the
  platform's fact a pairing the platform does not make, which outcome 5 forbids ("never a stated fact").
- **(c) The same pairing as native-theme's labelled mapping.** Outcome 5's rule: the role is one "a portable
  application needs and the platform names no counterpart for". The colour is the platform's, measured; the
  pairing is native-theme's, and labelled.
- **(d) `absent`.** The role is required, and outcome 5 says "never absent".

**3. The outcome: (c).** On the release path: yes, 0.7.0 holds `accent_text_color` on `ios-13`; the staging note
agrees.

**4. What it requires.**

- After the batch that gives outcome 5's label its form: `role.global.accent_foreground` on `ios-*`, the labelled
  mapping, its value `#ffffff` under `scheme:light` and `scheme:dark`, each citing the measured evidence the four
  button facts above cite. It joins the pairings listed for the maintainer's veto (`[outcome]:33`).
- One check that the facts cannot make yet: whether the filled title's colour follows the tint. U706's second-tint
  run (`[ledger]:715`; `[outcome]:23`) shows it; if the colour changes with the tint, the mapping becomes a
  derivation over the accent, as outcome 12 has the tinted values.
- Register: Decision recorded; the row retires when the mapping is written. All four listed paths are then empty.

**5. Confidence.** Medium-high. A veto would turn on whether a filled button's title is the right witness for
"content on the accent"; it is the one control in the slice that draws content on a tint fill.

---

### U473 — `ios-13`, `ios-26`: the key repeat

**1. The row.** `[ledger]:650`: "`ios-13`, `ios-26`: the key repeat […] | no API reports it | a measurement with a
hardware keyboard".

- Facts: 2, `docs/facts/ios/tokens.toml:20806`, `:20811` (`token.behavior.key_repeat.delay`, `.interval`). Listed
  paths: 2, `ios-13.toml:795`, `ios-26.toml:807` (`tokens.behavior.key_repeat`).
- The model: `Behavior.key_repeat: Option<KeyRepeat>`, its `delay` and `interval` never absent (`[spec]:3333`,
  `:3347`; `native-theme/src/tokens.rs:308`). The gap map: the setting exists (Settings, Keyboards & Typing), "the
  default delay and rate (not stated […]), the setting as data; a run-time read: none found" (`[gap-ios]:1162`).
  No catalogue entry reads the token; the one widget value near it, `stepper.values.key_repeat_interval`, is its
  own (`[catalogue]:8347`).

**2. The check D6's outcome 14 asks for.** Does 0.7.0's model hold a key-repeat value, and on an iOS preset? No, on
no preset (section 2.3). `ThemeDefaults` holds fonts, colours, a border, the focus ring, icon sizes and an icon
theme (`v0.7.0:native-theme/src/model/defaults.rs:37-145`), and no behaviour value at all.

**3. The outcome.** By D13's criterion the row is a later unit's and needs no decision now; nothing is the
maintainer's to supply.

- *Which staging it assumes.* Not a rule that singles the key repeat out: the staging note's token rule, under
  which a token path is at depth where the *today's fields* table names it, a value at depth reads it or the core
  reads it, and is staged otherwise (`[staging]:164-181`). 0.7.0 holds no field of `Behavior`, and one value at
  depth reads one of them, so the rule stages "all of `token.behavior` but `focus_visible_when`"
  (`[staging]:193`). Rule 3 is kept: the field leaves on every preset.
- *What that carries with it.* The values other presets already state stay in `docs/facts`, skipped by the gate
  until the field's unit (`[plan]:149-151`): `adwaita`'s 500 and 30 ms, `kde-breeze`'s 600 ms and 25 a second,
  `macos-tahoe`'s 500 and 83.333 ms, `material-3`'s 400 and 50 ms (`native-theme/src/presets/material-3.toml:2426-2428`).
  And `windows-11`'s listed path for the same field, under U503, kind `b` (`native-theme/src/presets/windows-11.toml:354`),
  empties with it, which counts for M2's task 0.
- *Which unit.* The note of 2026-11-10 names none for a field no value reads (section 2.4, (e)). Its revision 2
  names one: the parking unit *behaviour*, "U473's key repeat" among its paths (`[staging2]`, section 3.1, `:701`),
  where "a path no 0.8.0 value reads stays" (section 1.5, `:293`; "so U473 leaves 0.8.0's path, into the behaviour
  unit", `:299`). The batch that writes the staging records it.

**4. What it requires.** The staging task, as the note describes it (`[staging]:398-402`): the field leaves
`Behavior` until its unit, and `generate_preset.py` drops the path from both iOS tables and from `windows-11`'s.
Register: the row keeps kind `d` and gains the unit as its due point; if that unit is ever enabled with `ios-*`,
the value has no route but a hardware keyboard, and the row is then the maintainer's as the ledger says. Until the
staging, both listed paths stand, and nothing else removes them.

**5. Confidence.** High on the check. What a veto would need to know: without the staging, the two iOS presets
cannot load in 0.8.0 for the sake of a value 0.7.0 never had.

---

## 4. Found on the way

These are not the rows' outcomes; each is a correction or a dependency the work above ran into.

1. **`kde-breeze`, `label_position` under Kirigami** (U697, part 4). The fact `icon_only` under `tool` has no
   flavour condition (`docs/facts/kde/button.toml:13204-13212`), so it holds under `flavor:kirigami`, where Qt
   Quick's default is `TextBesideIcon`. The trace's Q1.9 line has no fact.
2. **`windows-11`, the button's fill transition** states `duration_ms = 83` plain under the engine `win_ui`
   (`docs/facts/windows/button.toml:1799-1800`), while WinUI sets up no brush transition with animations off
   (`Border.cpp:44-49`). By R9 a duration the switch ends is stated with the factor, or its engine reads it, as
   `WinUiAnimatedIcon`'s does; the `WinUi` member says neither (`[spec]:1652-1669`). For the motion unit, with U185.
3. **Statements of the register that the sources do not bear out.** Four are established, and the check confirms
   each: U180's "class 1" (it is class 2, `[plan]:1381-1385`); U561's `panel` absent (the catalogue's table excludes
   it, and the token is at the pin); U1's "which the context does not tell apart" (it does, by `{open}`); U697's
   need of a decision. Two that revision 1 listed are withdrawn. U555's "Compose's lists have no activation of
   their own" is true: the activation is the list item's (the token's part, part 4), a case the containers unit's
   reading has to provide for. U180's "no ceiling" is literally true of the operators (the specification's own
   words, `[spec]:2397-2398`); `floor` writes a ceiling exactly, and the batch adds `ceil` (U180, parts 2 and 4).
4. **The catalogue names W3-27 among the keyed icon contexts** (`[catalogue]:10551`); the Windows trace maps it to
   parts (U681, part 4).
5. **The checker's owed form goes with U180's batch.** S17 names the zero-fallback form and its exemption, and
   `check_fact_citations.py` does not implement it. `windows-11`'s `blink_timeout` is pending for that alone
   (`[ledger]:667`). With `if_le` the timeout is stated in WinUI's own test, so the form, its exemption and the
   checker task are withdrawn (U180, part 4). U505 then waits on the batch and its fact, and keeps
   `solid_while_typing`.
6. **The same token on other presets.** `ios-*`'s `single_click_activates` (U472) and `macos-*`'s (U329, U609)
   stay pending with the token (decision C). The evidence for both is in the token's part, part 4: UIKit's headers
   give `true`, with no iPad pass for that key; AppKit's give two readings, `NSWorkspace` read as deciding or not.
7. **The plan's list of losses is short by a class.** 0.7.0 stated all five icon contexts on all seven presets
   (`v0.7.0:native-theme/src/presets/<preset>.toml`, each `[light.defaults.icon_sizes]`): 16, 32, 22 and 20 for
   `small`, `large`, `dialog` and `panel` on `macos-tahoe`, `macos-monterey`, `ios-13` and `windows-11` alike
   (`macos-tahoe.toml:108-111`, `macos-monterey.toml:91-94`, `ios-13.toml:99-102`, `windows-11.toml:90-93`). The
   catalogue's absence table lets contexts be absent that 0.7.0 stated (`[catalogue]:10336-10339`):
   - `material-3`'s `small` and `large` (U561);
   - `adwaita`'s `dialog` and `panel`, already stated absent in the facts (`docs/facts/gnome/tokens.toml:7933-7950`)
     where 0.7.0 stated 22 and 20 (`v0.7.0:native-theme/src/presets/adwaita.toml:90-91`);
   - `macos-*`'s `large`, `dialog` and `panel`, pending under U102 (`[ledger]:500`);
   - `windows-11`'s `dialog` and `panel`, pending under U516 (`[ledger]:675`;
     `docs/facts/windows/tokens.toml:2631-2637`);
   - `ios-*`'s, which the table leaves to the facts, pending under U127 (`[ledger]:522`).
   D13's sentence (`[plan]:496-499`) names none of them. The plan's amendment is written for the class and names
   the three rows that decide its extent (U561, part 4).
8. **Outcome 5's label is needed by two rows of two presets** (U509, U651) before their paths can empty.
9. **`windows-11`'s icon theme and fallback** are pending under U516, kind `b`, "a read of the icon font and the
   sizes" (`[ledger]:675`; `docs/facts/windows/tokens.toml:2605-2613`). The catalogue's row that settles U559
   names `windows-11` too (`[catalogue]:10334`), so these two are a facts edit of the same form, not a read. U516's
   other keys were not examined.

---

## 5. What was not verified

- **No run was made and no build:** every statement about what a platform draws rests on the pinned source or on
  facts already committed.
- **U180.**
  - The form's exactness was shown against Qt's code as read (`qpainter.cpp:6121-6130`) and recomputed against a
    transcription of those lines (`register/u180_ops_check.py`), not against Qt running. No vector was generated
    and no underline captured.
  - On the antialiased path the form is not exact for a thickness of zero or less (U180, part 2); the script's
    figures for `t < 0` take the painter's pen as 1 px wide, which a caller's pen need not be.
  - No font was tested for an underline below its descent.
  - That QPainter's font engine for a run (`fe`) and the connector's `FontMetrics` for the part's font give the
    same three values was not examined beyond `QFontMetricsF`'s lines.
  - Antialiasing on the label's painter is inferred from the code read, as part 4 says.
  - The two operators are specified, not implemented: no code was written or built.
  - The forms of Qt Quick's line position, the strike lines and QPainter's wavy line are not decided (U180, part 3).
- **U505.** The count's recomputation transcribes WinUI's lines (`TextBoxHelpers.cpp:55-89`,
  `TextBoxView.cpp:2742-2763`); nothing ran on Windows. The blink time's value, 530 ms, is the facts' measurement.
- **The quote checker** was not run on any citation proposed here.
- **`single_click_activates`.** Not settled (decision C). AppKit's answer was not established, and the two
  candidate sources the token's part names were not read. UIKit's rests on header comments. Which signal a
  Kirigami list connects to activation was not read at a pin. The Mac idiom of an iOS application was not
  considered (R1.4 leaves it to the `macos-*` presets).
- **U509.** The two documents that would support Cascadia Mono were not read; what they state is unverified.
- **U122.** Whether UIKit draws a title on an info, contact-add or close button: no pinned source. The counts are
  by the rule in `u122_removable.py`, over each fact's key, positive variants and positive states.
- **U1's Compact values** ("10 above and below") were checked only by arithmetic on the template's margins, not
  against the host's `command_bar` pass.
- **U651's independence of the tint** is unmeasured (U706).
- **The staging design** was read as it stood on 2026-11-10 for section 2.4's table. Its revision 2 was read on
  2026-11-11 only where these rows cite `[staging2]`, while it was being edited, so its line numbers may already
  have moved.
- **The label form of outcome 5** is not written; U509's and U651's facts are described, not spelled.
- **Not read:** the `ios-*` and `macos-*` state models beyond the lines cited; the connectors' present code; the
  WinUI Gallery and Windows Terminal, which are not pinned.

---

## 6. The checks' findings

### 6.1 The first check (`register-rows-check.md`), answered in revision 2

Each was verified at its cited lines. None was found not to hold. Three are taken with a difference, set out below
the table. Revision 3 changes two answers: finding 1's (the form is now the operators', 6.2, findings 3 and 4) and
the difference (a) (6.2, finding 1). Revision 4 changes both again: finding 1's form is `ceil` and `if_le`, which
also retire the zero-fallback form the finding found unimplemented (6.3, finding 3), and finding 2's reading moves
with the token to the containers unit (6.3, finding 1).

| # | Finding | Verified at | Taken |
|---|---|---|---|
| 1 | U180: the facts cannot pass the gate as specified; the weighing left the checker's cost out | `[spec]:6876-6885`; `scripts/check_fact_citations.py` (no form recognition); `docs/facts/windows/tokens.toml:2711-2718`; `[ledger]:667` | Yes. Revision 2 changed the choice to a platform function; revision 3 to the operators' form, the checker's work shared with U505 (6.2, finding 3: this finding quoted S4.2's reason by its second half too) |
| 2 | U504, U555: two readings fit the facts; the sentence joined them; U555 used the half U504 set aside | `ItemsView.cpp:421-424`, `ItemsView.idl:41-42`; `ListItem.kt:288`; `gtklistbox.c:525-544`; `docs/facts/gnome/tokens.toml:2170-2172` | Yes, with a difference (a) |
| 3 | U122: 96 facts removed on a premise the header does not bear out | `xcode@26.6 UIKit/UIButton.h:119`, `:21-34` | Yes, with a difference (b) |
| 4 | U509: the family is not settled; a WinUI source for captions was missed | `CueStyler.cpp:1684-1717` | Yes, with a difference (c) |
| 5 | U509: the facts' form contradicts S4.4; catalogue lines left behind | `[spec]:3631-3633`; `docs/facts/material/tokens.toml:9274-9276`; `[catalogue]:10192`, `:10741` | Yes |
| 6 | U504: one catalogue line left behind; the S4.2 sentence has no class | `[catalogue]:10719`; `[plan]:1383-1385` | Yes |
| 7 | U561, U559: the loss is not carried to the plan; outcome 5 not weighed for the two contexts | `[plan]:496-499`; the two connector files at `v0.7.0` | Yes; widened to the class (section 4, item 7) |
| 8 | U180: on 0.8.0's path through the link | `v0.7.0:…/kde-breeze.toml:465` (revision 2 wrote `:463-464` and said the check erred; it did not, 6.2, finding 6); `[plan]:768` | Yes |
| 9 | Section 4, item 3: two claims not established | `[spec]:2397-2398` | Yes: both withdrawn |
| 10 | U473: what the staging carries is understated | `material-3.toml:2426-2428`; `windows-11.toml:354` | Yes |
| 11 | U1: the decision reaches 0.8.0 | `[plan]:732`; `[catalogue]:4729-4731` | Yes |
| 12 | U180: the antialiasing argument cites the wrong mechanism | `kquickstyleitem_p.h:48`; `kquickstyleitem.cpp:2096-2098` (revision 2 wrote `:2095-2097`; the check was right, 6.2, finding 6) | Yes |
| 13 | U314: the count of image names; U614's pin | `NSImage.h` at both pins: 153 lines, 139 declarations | Yes |
| 14 | U681: the `absent` list checks nothing | `scripts/check_fact_citations.py:29` | Yes |
| 15 | U515: the citation for the zero left open | `[spec]:6958-6966` | Yes: the form named, the verifier's call stated |
| 16 | U424: `auto`'s meaning tied to a preset | `[spec]:3861-3862` | Yes |
| 17 | U122: the specification sentence may not be needed | `[spec]:3994-3996`; `[catalogue]:1625-1627` | Yes: not required |

#### Findings taken with a difference

No finding is rejected whole: each holds at its cited lines. On three points revision 2 did not do exactly what
the check proposes; the evidence for each difference is given.

- **(a) Finding 2 asks that the reading be listed for the maintainer "as a real choice of meaning […] not as
  settled".** Superseded in revision 3: check 2's finding 1 showed a second reading under which every stated fact
  holds, and the three readings were listed (6.2, finding 1; revision 3's section 7, item 2; since revision 4 the
  token's part, part 2, for the containers unit). Revision 2's answer, kept for
  the record, in its labels (R-B: which gesture activates, in a view that also selects; V1: views that show a
  model's items and hold a selection, revision 3's reading (ii); V2: any list of rows, reading (iii)): the
  coordinator's instruction is to state the one reading under which all existing stated facts are
  true, and to say which facts are wrong if none fits. One fits: R-B over V1 (the part before U504). The `adwaita`
  fact states V1 in its own comment (`docs/facts/gnome/tokens.toml:2170-2172`), and under V2 that fact is not true
  as written (`gtk@4.24.1 gtk/gtklistbox.c:525-544`). So the reading is settled against the existing facts, and V2
  is listed in section 7 as the alternative, with what it would change. The check's own sentence was not adopted as
  it stands: it leaves "which views count" open, and it is that question which decides `material-3`.
- **(b) Finding 3 counts 293 facts for the content rule; this revision counts 294.** The difference is
  `button.label.text.fill` under the flag `route_active` (`docs/facts/ios/button.toml:303568`), which the state
  model draws with `route` alone (`docs/facts/ios/state-models.toml:204`). The check notes that revision 1 treated
  it unlike the `subtitle` fact beside it; the route picker holds neither, so both go.
- **(c) Finding 4 says "a pick among three with pinned sources" and "searched in all twelve Windows checkouts".**
  Thirteen sources serve `windows-11`, twelve in the cache. A further pinned mention was found that neither
  revision 1 nor the check lists: the Toolkit Labs' Markdown control draws code in Consolas
  (`Labs-Windows@251217 components/MarkdownTextBlock/src/MarkdownThemes.cs:89`), and Visual Studio's editor
  documentation uses "Consolas 10 pt" as its example. The finding's substance stands: the family is a pick.

### 6.2 The second check (`register-rows-check2.md`), answered in revision 3

Each finding was verified at its cited lines; each holds, and each is taken. Check 2 numbers the readings of the
token differently: its (i), (ii), (iii) (model and selection; model alone; rows) are this document's (ii), (i),
(iii), ordered as the orchestrator's decision lists them. Revision 4 changes the answers to findings 1 and 2 (the
token moves, decision C) and to finding 3 (U180's form, decision D); 6.3 says how. The table keeps revision 3's
answers as the record.

| # | Finding | Verified at | Taken, where |
|---|---|---|---|
| 1 | Serious. U555: a second reading keeps every stated fact; `absent` withholds a behaviour the pinned reference shows | `gtk@4.24.1 docs/reference/gtk/section-list-widget.md:29`, `:76-88`, `:242-244`; `gtk/gtklistbox.c:50`; `LazyDsl.kt:429-432`, `:155`, `:169`, `:470-483`; `ListItem.kt:304-306`, `:386-387`, `:1602-1611`; `ListSamples.kt:354-394` | Yes, by decision A: three readings with what each changes (the token's part, part 2; section 7, item 2), the model alone applied, `material-3` `true` (U555), the sentence's `true` without "that selects". One addition: GtkListBox can be bound to a model (`gtklistbox.c:4055-4056`), so the sentence names each toolkit's views and the containers that are none |
| 2 | The sentence does not give one value on `windows-11` or `macos-*` | `ListViewBase_Partial_Interaction.cpp:255-274`; `ItemsView.cpp:421-424`; `NSControl.h:25`; `NSTableView.h:278` | Yes: the sentence's (1) keeps the default selection mode and does not count a forbidden configuration, so `windows-11` `false` follows from it (U504); its (3) leaves `macos-*` pending at the token's unit with the deciding source named. The check's clause ("a view whose documentation withholds click activation while it holds a selection does not count") is taken in a general form |
| 3 | U180: S4.2 quoted by half; R5.4 unweighed; the checker's cost counted twice | `[spec]:3439-3441`; `[rationale]:642-643`; revision 2's U180, part 4 | Yes: both sentences quoted in full and weighed (U180, part 2, (c)); the checker's work is shared with U505 (part 3). With decision B the outcome changes to (a) |
| 4 | U180: the vector table not shown producible for the third case or the ties | `qpainter.cpp:68`, `:6075`; `qfontmetrics.cpp:1170-1175`, `:1807-1812`, `:1842-1847`; `tools/kde-vectors/README.md:32-33` | Yes: it is a reason against (c) (U180, part 2); under (a) no table is needed, and the same limit is recorded for the row's capture (part 4; section 5) |
| 5 | Section 2.4, (d): `tool` stated as a necessity, and for `windows-11` only | `[catalogue]:4732-4734`; `v0.7.0:native-theme/src/presets/windows-11.toml:390-407`; `[staging]:119`, `:146-147`, `:974-978`; the 31 facts recounted | Yes: (d) restated as an open point for the three platforms, decided by the per-unit comparison, a typed name at depth on every preset or none; U1, part 3 |
| 6 | Two line numbers changed from right to wrong | `v0.7.0:native-theme/src/presets/kde-breeze.toml:463-465`; `qqc2-desktop-style@6.31.0 plugin/kquickstyleitem.cpp:2095-2098` | Yes: `:465` and `:2096-2098` restored (section 2.3; U180, part 4; 6.1, rows 8 and 12) |
| 7 | Three citations a few lines off; Qt's quote inexact | `section-list-widget.md:26-27`, `:29`; `gtklistview.c:955`; `qstyle.cpp:1919-1921` | Yes: the token's part cites `:26-27`, `:29` and `gtklistview.c:948-956` (the property's comment through its spec, the default at `:955`), and quotes Qt as written |
| 8 | U509: seven Courier New hits left out, not three | the eight lines, found again by search | Yes: seven pages on eight lines (`listbox.md` twice), listed (U509, part 1) |
| 9 | Section 1 counts four picks, section 7 lists seven | revision 2's sections 1 and 7 | Yes: section 1 counts the seven items 1 to 7 of section 7 and names items 8 and 9 as no picks |
| 10 | U180, part 4: "no checker change"; "the only form checked against the platform" | `scripts/generate_preset.py:299`, `:301-303`, `:1538-1539`; `scripts/check_spec_names.py:179`, `:504`, `:947-949` | Yes, stated precisely: `generate_preset.py` needs `font_descent` for `widget.<part>.<metric>` only; `check_spec_names.py` reads the name from S2.2's production and needs no code change; the second claim withdrawn (U180, part 4) |
| 11 | The class of lost icon contexts reaches three more presets | `v0.7.0` presets' `[light.defaults.icon_sizes]`; `[catalogue]:10336-10339`; `[ledger]:500`, `:522`, `:675` | Yes: section 4, item 7; U561, part 4; section 7, item 8 |

Check 2's verdicts on the four disagreements with the staging design are recorded in section 2.4.

**Decisions not applied: none.** Both were checked for anything false before they were applied.
- Decision A says the model criterion "changes no existing stated fact (`adwaita`, `kde-breeze`, `ios-*`,
  `windows-11` come out as they are)". True under the sentence as written here, which names GtkListBox as a
  container. One qualification, not a falsehood: GtkListBox's `bind_model` fills it from a model
  (`gtklistbox.c:4055-4056`), so "presents the items of a model" read without that naming would count it and
  change the `adwaita` fact; the sentence names it for that reason. Its citations hold: `section-list-widget.md:29`,
  `ListSamples.kt:354-394`, `LazyDsl.kt:429-432`, `ListItem.kt:304-306`, `:1602-1611`.
- Decision B's condition holds: the operators state QPainter's rule exactly in all three cases and at the ties
  (U180, part 2). Its citations hold: `[rationale]:642-643`, `qpainter.cpp:6075`, `generate_preset.py:301`.

### 6.3 The third check (`register-rows-check3.md`), answered in revision 4

Each finding was verified at its cited lines before anything changed. Each holds. Findings 1, 2, 5, 6 and 11 are
answered by decision C, which moves what they concern; findings 3, 10 and 14 by decision D; the rest by
corrections. None is rejected.

| # | Finding | Verified at | Taken, where |
|---|---|---|---|
| 1 | Serious. The S4.2 sentence gives no single value on `windows-11`: TreeView invokes on one click, and the Learn page documents as supported the configuration the sentence calls forbidden | `TreeViewList.cpp:384`; `TreeView.cpp:159-164`, `:408-410`, `:421`; `TreeView.idl:114-116`; `treeview_selectionmode.md:19`; `tree-view.md:383`; `listview-and-gridview.md:376-377`; `itemsview.md:262-263`; `ItemsView.cpp:421-424` | By decision C: no sentence; U504's `false` withdrawn; the evidence recorded (the token's part, part 4, Windows) |
| 2 | Serious. On `macos-*` a pinned header meets clause (3) | `xcode@26.6` and `xcode@16.4` `AppKit/NSWorkspace.h:119`, `:134`; `NSCollectionView.h:127` (no action); `NSBrowser.h:57` | By decision C; both readings recorded (part 4, macOS) |
| 3 | Serious. U180's 418-character form is needlessly complex; R5.4 misread; S2.2's `round_pt` precedent passed over | `[spec]:465`, `:3441-3443`; `[rationale]:632`, `:642-643`; `register-check3/u180_alternatives.py` rerun: 635,934 inputs, no difference | By decision D: (b′), `ceil` and `if_le` (U180, parts 2 to 5) |
| 4 | Should-fix. Qt Quick's position is not statable from font metrics; the row would retire without Qt Quick's forms | `qquicktextnodeengine.cpp:213`, `:276`; `[spec]:2396-2408`; `[ledger]:561` | Yes: the outcome limited to QPainter's underline; U180 narrowed, kind `d`; the S2.8 sentence keeps the deferral (U180, parts 3 and 4) |
| 5 | Should-fix. Under `flavor:kirigami` "Qt's item views" has two readings | `kquickstyleitem.cpp:1517-1518`; the search of the three Kirigami checkouts; `qquickitemdelegate_p.h:25`, `qquickabstractbutton.cpp:448-466` (added) | By decision C; both readings recorded (part 4, Kirigami) |
| 6 | Should-fix. The criterion cannot be applied to an unnamed view; no rule for disagreeing views | `gtklistbox.c:4055-4056`; `section-list-widget.md:61-63`, `:105-108`; `NSBrowser.h:57`; `qquicklistview.cpp:2305-2309`, `:2357` (added) | By decision C; GTK's mark recorded (the token's part, part 2); the disagreeing views are the model-form question (part 6) |
| 7 | Should-fix. Section 7 misses U122's reading; items 3 and 4 no longer hold | `[doc]` revision 3, U122 part 5 and section 7 | Yes: U122's reading is item 3; revision 3's item 3 leaves with the token (decision C); item 4 is restated as item 2; section 1's count follows |
| 8 | Minor. `Clickable.kt:330` is the hidden, deprecated overload | `Clickable.kt:319-330` (`DeprecationLevel.HIDDEN` at `:322`); `:539`, `:543-553`, `:551`; `ListItem.kt:1602-1611` | Yes, in the evidence record (part 4, Material; U555, part 1) |
| 9 | Minor. The descent's text shown as a quote of `query/mod.rs:27` | `native-theme/src/query/mod.rs:26-46` | Yes: a new method of the trait at `:26-46`, its documentation new text (U180, part 4) |
| 10 | Minor. `mul(F, C)` against the specification's `mul(T, number)` | `[spec]:547`; `ty.rs:1064-1065` | Moot: (b′)'s facts hold no `mul` |
| 11 | Minor. The quote at `listviewbase_isitemclickenabled.md:27` must keep its markup | the file's line 27 | Yes: the evidence record quotes it as the file writes it, and says why |
| 12 | Minor. The recomputation checked a hand-written copy, and no negative `t` | `register/u180_form_check.py` | Yes: `register/u180_ops_check.py` parses the facts' own text and tries `t` from −2 px; the lane's check runs on the committed text (U180, part 4) |
| 13 | Minor. "To add a list of items" cited at `LazyDsl.kt:155`, `:169` | `LazyDsl.kt:155`, `:169`, `:429-432` | Yes: `:429` for the list's sentence, "Adds a list of items." at `:155`, the function at `:169` (the token's part, part 2; U555, part 1). `ListItem.kt`'s selection overloads are at `:386` and `:470`, not `:305` and `:386-387` (found on the way) |
| 14 | Minor. S17's wording must widen for a selection form | `[spec]:6883` | Moot: no named form remains, and S17's exemption is withdrawn (U180, part 4) |

One correction to check 3's cost table (finding 3): `check_facts.py:3849` and `:5296`, listed there as mirrors to
change, are the checker's self-test fixture and its expected value. `core_functions` reads the real table
(`scripts/check_facts.py:538-549`), so `check_facts.py` needs no change for the operators' arity (U180, part 4).
The finding's substance stands. (Revision 5, check 4's finding 5: its `round_pt` rule does need `ceil`.)

**Decisions not applied: none. Decisions applied, with two premises corrected.** Both decisions were checked for
anything false before they were applied. Each holds; one premise of each is inexact, and neither changes what the
decision does.
- *Decision C* says "The facts already stated for it (`adwaita`, `kde-breeze`, `ios-*`) stay as they are". `ios-*`
  states no fact for the token: it is pending under U472 (`docs/facts/ios/tokens.toml:20800-20803`; listed at
  `native-theme/src/presets/ios-13.toml:793`, `ios-26.toml:805`). Revision 3's table gave `ios-*` `true` "by the
  sentence", "pending today (U472)". Applied as: `adwaita` and `kde-breeze` stay as stated, and `ios-*` stays
  pending with `windows-11`, `material-3` and `macos-*`. The rest of the decision holds:
  - 0.7.0's model never held the token (`git grep` at `v0.7.0`, the token's part, part 3);
  - the staging parks it (`[staging2]`, section 3.1, `:701`);
  - WinUI's views answer differently (part 4);
  - `NSWorkspace.h:119` and `:134` read as the decision says, at both pins.
- *Decision D* says "R5.4 is a rule for platform functions, not for the model's arithmetic". R5.4's text governs
  the whole closed set: "Their functions are a closed set, each with one definition and its source", a set that
  includes `alpha` and `mix`, which S2.2 counts as the model's own arithmetic (`[spec]:586-588`); and "a function
  is added only when a platform's rule needs it" (`[rationale]:632`, `:642-643`). The decision holds on R5.4 as
  written. (Revision 5, check 4's finding 9: R5.4's list for lengths and durations, `[rationale]:639-641`, has been
  narrower than S2.2 since ADD-A56; adding a function is class 2 by `[plan]:1383`.) The rule that needs `ceil` and `if_le` is QPainter's
  (`qpainter.cpp:6128-6129`), and WinUI's for `if_le` (`TextBoxView.cpp:2756`), as `round_pt` was added for
  KTitleWidget's rounding (`[spec]:465`). U180, part 3 argues it so. The decision's other grounds hold as stated:
  - the 418-character form needs `0px` and `1px` exempted;
  - `[spec]:465` added an operator rather than exempt such literals;
  - a shorter exact form exists.

### 6.4 The fourth check (`register-rows-check4.md`), answered in revision 5

Each finding was verified at its cited lines before anything changed: the specification, rationale, plan and
staging lines, the code at 7430a0a2 (`ElementDatum`, `FontMetrics`, `Decoration`, `Func`, the parser, the typed
tree, the five `FontMetrics` implementations), `check_facts.py`, the Qt and WinUI sources at their pins, and the
U505 facts. Each holds and each is taken. One citation inside finding 1's proposed sentence does not hold: "(S5)"
is the specification's *Widgets and parts* (`[spec]:3807`); the model's conditions are S7.1 (`[spec]:4117`, `Condition`,
`SystemSetting(Key)` among them) and its states S7.2 (`:4183`). The sentence is applied with "(S7.1, S7.2)".

| # | Finding | Verified at | Taken, where |
|---|---|---|---|
| 1 | Should-fix. No rule separates `if_le` from a condition; "R5.4 bounds their use" is not what R5.4 says | `[rationale]:642-643`; `[spec]:573-575`, `:3441-3443`, `:6876-6885`; `combos/mod.rs:2040-2160` (`Term::Call`); `[staging2]:25`, `:700`; `qpainter.cpp:6129`; `TextBoxView.cpp:2756` | Yes, with S7.1 and S7.2 for S5: the sentence in the `if_le` row and both uses tested against it (U180, part 4, item 3); the claim withdrawn (part 3, *Against (b′)*) |
| 2 | Should-fix. The flavours unit is a patch release; one model form for Qt Quick's position is breaking | `[staging2]:755`; `expr/mod.rs:157-158` (no `#[non_exhaustive]`), `lib.rs:111`; `query/mod.rs:27-46` (no default method), `lib.rs:300`; `blocks/text.rs:442-443` (`#[non_exhaustive]`) | Yes: the constraint and the open point (U180, parts 3 and 4); section 2.4's notes for the staging design |
| 3 | Minor. The controls are not in the script; their counts are for one pen path | `register/u180_ops_check.py`; `register-check4/controls.py`, `controls_variants.py` | Yes: both controls in the script, each mutating both facts, counted per path; the U505 count evaluated from the fact's own `if_le` argument, with a control (U180, part 2; part 4, U505) |
| 4 | Minor. The antialiased path does not model QPen for `t ≤ 0` | `qpen.cpp:488-490`, `:658-663`, `:792`; `qfontengine_ft.cpp:1026`; `qpainter.cpp:6084-6089`, `:6123` | Yes: QPen modelled; the antialiased form stated for `t > 0` only; every input with `t ≤ 0` on that path differs, 142,742 (U180, part 2; part 4; section 5) |
| 5 | Minor. `check_facts.py`'s `round_pt` rule lacks `ceil` | `scripts/check_facts.py:230`, `:3351`, `:3369` | Yes (U180, part 4, *The tools*) |
| 6 | Minor. The code list cites a unit test as the parser and leaves places out | `parse.rs:331`, `:767`; `mod.rs:275-277`, `:375`, `:472-474`; `ty.rs:500-527`; `arith.rs:1-3`; `eval.rs:485-489`; the five `impl FontMetrics`; `declare.rs:400`; `evaluate.rs:1868`, `:1899`, `:3283`; `ty.rs:1188`; `validate/checks.rs:1180-1222` | Yes (U180, part 4, *The model's code*) |
| 7 | Minor. `if_le`'s comparison pair's types partly unstated | `ty.rs:992-1052`, `:1143-1151` | Yes: the row states them; item 4 and its note follow |
| 8 | Minor. What blocks U505's `solid_while_typing` is not said | `docs/facts/windows/tokens.toml:2717-2718`, `:2744-2747`; `TextSelectionSettings.cpp:40`; `TextBoxView.cpp:2598-2633`; `[spec]:3328` | Yes, with the ledger's "next" cell (U180, part 4, U505) |
| 9 | Minor. Two loose wordings about R5.4 | `[rationale]:633-635`, `:639-641`; `[spec]:464`, `:586-588`; `[plan]:1383` | Yes (U180, part 3; 6.3, *Decision D*) |
| 10 | Minor. Section 7 lists picks only, though the header says open points too | the header; section 7 | Yes, by the header: it says where the open points are; section 7's introduction says the same |
| 11 | Minor. `[staging2]:807`'s U180 line will be stale after the batch | `[staging2]:807`; U180, part 4 | Yes: a note in section 2.4; the staging note is not edited |

---

## 7. For the maintainer's veto list

Picks and choices of meaning, each with the alternative and what it would change, and two items that are no pick
(7 and 8). The open points are not listed here; they stay in the rows that raise them, each with its unit: the
token's question (the token's part, parts 2, 4 and 6; the containers unit) and the form of Qt Quick's line position
(U180, part 3; the flavours unit, where only the decoration field fits its patch release).

Revision 3's items 2 and 3 have left the list: which views `single_click_activates` speaks of, and how a view is
read. Decision C moves both, with the token, to the containers unit; the token's part, parts 2, 4, 5 and 6, holds
them for it. Revision 3's item 4, the form for QPainter's underline, stays, restated as item 2. Decision D settles
which form this document applies; the specification change is still the maintainer's to veto.

1. **The monospace family on `windows-11` (U509).** Recommended: Consolas, the family the pinned sources name for
   the nearest uses; no source makes it the platform's. Alternatives: Cascadia Mono, which 0.7.0 shipped, after one
   or two pins (U509, part 3); Lucida Console, WinUI's own choice for captions; Courier New. One string changes.
2. **The form for QPainter's underline (U180), and with it U505's timeout.** Applied, by decision D: S2.2's
   arithmetic with two operators added, `ceil(x)` and `if_le(a, b, x, y)`, a class 2 change. The offset fact is
   215 characters long and holds no literal but Qt's; the zero-fallback form and S17's exemption are withdrawn.
   Alternatives:
   - today's operators (316 characters with one selector, 418 as revision 3 wrote it): no operator added, and an
     S17 exemption for `0px` and `1px` that `[spec]:465` rejects, with named forms the checker must recognise;
   - `ceil` alone (273 or 358 characters): the selection still a named form, with the same exemption;
   - a platform function with a vector table (131 characters): the vectors cannot reach the third case, and a clean
     room is needed.
3. **U122's reading of S6.3's content rule.** Applied: a part the control cannot be given content for is "a part
   the element's content lacks" (`[spec]:3994-4003`), so the 294 facts of `subtitle`, `menu_indicator`, their
   placing values and the route picker's label need none (U122, parts 2 and 3). Alternatives, each with what it
   changes:
   - one clause after "(a button's `label` under `icon_only`)" at `[spec]:3997` that says so: class 2, same
     outcome;
   - the parts absent under those variants: the catalogue's change on `ios-*` (`[catalogue]:2001`, `:2023`), which
     S6.3 rejects for content;
   - the 294 left pending until a pass: "a measurement of nothing drawn", which S6.3 also rejects.
4. **The pairing behind `accent_foreground` on `ios-*` (U651):** the filled button's title colour as the witness.
5. **The pairing behind `panel` on `material-3` (U561):** the navigation drawer's icon size, as the catalogue and
   0.7.0 have it.
6. **`auto` on `subtitle_gap` (U424):** a layout rule carried by a value's `auto`, against a field of its own.
7. **Two additions to the list of what 0.8.0 loses against 0.7.0** (the maintainer's list, `[plan]:496-499`), not
   picks: the icon contexts a platform names none for, which 0.7.0 stated on every preset (`material-3`'s `small`
   and `large`, U561; `adwaita`'s `dialog` and `panel`; on `macos-*`, `windows-11` and `ios-*` as far as U102, U516
   and U127 find, section 4, item 7); the icon theme's name on the presets that repeated their set's (U559).
8. **Four points raised with the staging design** (section 2.4), none of them this document's to decide. Its
   revision 2 takes up all four:
   - the speed factor at depth, so U515 stays;
   - the mnemonic tokens kept with their leaf;
   - `tool` at depth with the containers unit;
   - a named unit for every staged path, the behaviour tokens in *behaviour*.

   Nothing is left from this document for the maintainer here.
