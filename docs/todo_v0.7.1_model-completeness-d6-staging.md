# D6, outcomes 2 and 3: the staging design

2026-11-10, revised 2026-11-11 (Revisions 2 and 3) and 2026-11-12 (Revision 4). A design note for the orchestrator and the maintainer. It designs
how 0.8.0 states each widget, role and token at the depth 0.7.0's model distinguishes (outcome 2) and holds the two
motions 0.7.0 has (outcome 3), as `d6-outcome.md` records them. It decides seven things, each with its options;
section 8 lists what the sources did not settle, section 10 the findings of the checks not taken and the
orchestrator's decisions not applied as worded.

## Correction of ADD-A147's fix round (2026-11-12)

The batch that applied this design (ADD-A147) checked 1.3's point 1 against the facts, as that point asks: `button`'s
`label_position` is stated with no later name on `kde-breeze`, and `repeat_delay` and `repeat_interval` at the base
on `kde-breeze` and `macos-*`, so "only names of later units read" them does not hold (ledger queue #258). The
orchestrator's decision on the batch's check A: the three are at depth where a button with no later name reads them
(ADD-A148), as the rule's "every part, field and value" of an entry at depth has it; the other values of 1.3's point 1
stay deferred (open to the maintainer's veto, veto list item 1). The recount, the staging scripts with the three not
deferred (`depth_count_a2.py --values-at-depth`, a copy of `depth_count.py` beside it with that option, which without
it prints `depth_count.py`'s tables unchanged): 1,381 pending facts at depth (was 1,376), 687 listed paths that stay
(was 682). What moves: on every preset the three values' facts (27 more facts at depth, 7,227 against 7,200, each
fact once; on `macos-*` the two `token.params.pointer_repeat` facts the repeat values read come with them); five
pending facts,
all `label_position`'s: U697 on `kde-breeze`, U616 on `macos-monterey`, U333 on `macos-tahoe`, U484 on both iOS
presets (a fact of two presets), U517 on `windows-11`; and their listed path `widgets.button.values.label_position`
on `macos-monterey`, `macos-tahoe`, `ios-13`, `ios-26` and `windows-11`. The figures below are corrected where they
describe revision 4's rule; those that record an earlier revision or a check stay as they were.

## Correction of the document pass (2026-11-12)

The full document pass after ADD-A147 and the orchestrator's decision per finding (ADD-A148, each open to the
maintainer's veto, `docs/todo_v0.7.1_model-completeness-d6-veto-list.md`) change this design as follows; the body
below is not rewritten. Only verified facts reach the branch: the gate lists every new or changed fact since a
platform's verified commit, whatever its unit, and `verify_facts.py units` stays unit-blind, so "and
`verify_facts.py units`" is withdrawn from 2.5's *Who checks it* and from 6.4's step 2 (P-3). The units file lists
the units of depth and the parking units in 3.4's order, the order S17 reads (P-4). The predicate (1.3, 2.5) is
decided exactly, over the combinations at depth: a fact is of the enabled units when its key's path, and the widget a
`drawn_as` value names, are held by them and its `when` holds in at least one combination of the enabled units'
names that its entry's state model draws on that preset. A fact that is not of the enabled units is a later unit's:
the unit at whose enabling, the units enabled one after another in the units file's order, the fact is first of the
enabled units, not the first unit whose names it needs. It is decided from names, without walking combinations: for
each drawn set of its entry's state model that can hold the fact (its part drawn there, by the part's presence
facts), the last unit in the file's order among the names the fact needs in that set (the names its `when` lists
positively, the names the set or the presence fact requires beside them, a name of a kind whose at-depth names the
fact's negative list excludes, its key's path, a host context where the set is drawn only inside one: that host's
unit, `identities` for a context naming no widget); the fact's unit is the first such unit over those sets. A fact
that no drawn set can hold with any units' names is *unplaced*: the gate and `check_facts_trace.py` report it apart,
under no unit, and from the plan's M4 the gate fails it. A fact of the enabled units is checked exactly, over the
combinations at depth; nothing walks the combinations of all units' names, so the "stray fact" of 2.5 and of the revision tables (a `when` no union of units
makes hold, failed by `check_facts.py` from M4) is withdrawn in favour of the unplaced fact (2.5, 3.1; K4-4, K4-8,
the orchestrator's O-14). A part's `shown` stays in 0.8.0 with its rest meaning alone and is not the motion unit's
(2.4, 5.3, 5.4 (a), 6.4's step 1), and every widget's `loop` but the spinner's is the motion unit's (K2-1, K2-7;
veto list item 61). `full_keyboard_access` is at depth model-wide, as `contrast_high` is (1.3 point 6, 1.4; C-4,
item 62). `idiom:phone` is the iPad unit's with `idiom:pad`, so combinations at depth hold no idiom (1.4, 3.1's
table; C-7). The trimming (2.3) also leaves undeclared an exclusive group or a recency set left with fewer than two
flags, a recency set no rule of the enabled units' facts names, and `device_models`, `container_sizes` beyond each
default and `presentation_events` that no rule (for an event, a `presentations` list) of the enabled units' facts
names, a setting declared for applications aside (K3-6). The tests a part's deferred `transitions` leaves without a
part that lists it (the plan's staging task, its step on tests): a check that is a function of the block alone is tested on the block; a check
that needs a part listing the field keeps its tests, compiled and `#[ignore]`d with the motion unit as the reason,
recorded in the ledger as that unit's and un-ignored at its enabling; a test that used the field only as an example
moves to a field at depth; the checks and tests of a staged token field that leaves its struct leave with it, listed
by the unit that brings it back (O-1). The figures below are not recomputed for these changes; the increment that
brings `full_keyboard_access` to depth reports any change of the done-tables (ledger queue #283).

## Revision 4

Revision 3 was checked independently (`scratch-keep/d6/staging-design-check3.md`, "the third check": nothing
serious, 6 should-fix, 9 minor). Every finding was verified at its evidence before anything changed (10.0); all hold
and all are applied. No count of revision 3 changes: 1,376 pending facts at depth and 682 listed paths stand, and
the recounts below reproduce them. The lines of this file move again, so a line citation of revision 3 does not
resolve in revision 4. What changed, and why:

| Change | Answers |
|---|---|
| The look of a button with no variant on `material-3` and `ios-13` is a **correction of 0.7.0's mapping**: it draws the platform's own default button (Compose's `Button`, the filled one, `Button.kt:77-88`, against `FilledTonalButton`, `:439-447`; UIKit's system button, `UIButton.h:22-23`, no default configuration, `UIButtonConfiguration.h:78-94`), 0.7.0's look staying under `tinted` and `@gray`; on the correction lists (1.3, 1.7's two new rows, R-3, P-3, 8.9), and 10.2's "further decision" removed | should-fix 1 |
| Class 4 of 1.7's test grounded in the preset's reference rule, R1.6 for `material-3` (the older token set; material-web's component where Compose has the button) and R1.3 and R1.4 for `ios-13` (WebKit's HTML button against UIKit); "a source the facts do not take", untrue for material-web, replaced (1.7, R-3, P-3, P-8, 8.9) | should-fix 2 |
| Class 2 limited to a name of the entry's own units; class 5 also takes a look only a shared unit's name holds, so `material-3`'s checked toggle, held only under `flavor:expressive`, is class 5 by the test (1.7, P-8) | should-fix 3 |
| 7.10 and 2.3 say that `holes3.py` part B's `rev3` count tests the recency sets alone, the flags, settings and flavours being 0 by the form's definition, and that their real test is S12's own check on the regenerated presets, with a positive control in the staging task's gate; part C now counts by S12's rule (a stated base, or a listed path the synthetic route's bases state), with the same entries as revision 3 (`out/holes4.txt`) | should-fix 4 |
| The motion trial's cap of two harness rounds per Apple harness is stated as decision G's budget; the slice's record (several correcting facts-lane and tie-break rounds per Apple lane, not harness rounds) is cited as it is, and the fallback, both motions lost on every preset, is said to be likely (5.5, 8.9, P-7) | should-fix 5 |
| P-6b gives the plan text of the staging task: its inputs, its steps by file (the generator's state model in force in one place, `declarations()`' `models`), its gate with a positive control, and its done-criterion (the eight `[pending]` tables); P-6 and 2.7 reference it | should-fix 6 |
| 1.6: `tinted` adds 8 on `ios-26`, not 4 (each variant alone recounted: `ios-13` 323 and `ios-26` 337 for either, against 319 and 329) | minor 7 |
| 2.3 and 7.10 say how `gen_bases.py` runs (it restricts `covering_bases()` alone) and cite the third check's `gen_inforce.py`, which restricts all three readers of the state model; re-run for revision 4 on all eight presets with the same results (7.10) | minor 8 |
| "UIKit's plain button" → UIKit's system button with no configuration (1.3, 1.7, C-3d/C-3e's text, 10.2) | minor 9 |
| S12 refuses a `later` name, or a size or flavour of a unit not enabled, with a problem of its own, `ProblemKind::Later`, not as an undeclared one (2.3, S-4) | minor 10 |
| S-3 says S12's load-time combinations still add a deferred flavour's atom, harmless while no rule names it (to be confirmed by the staging task's load) | minor 11 |
| 2.3's reason against an S12 exemption for keyed flags is the declaration's own meaning (spec:7338-7339), not a run-time table's cost | minor 12 |
| The macOS reader's module comment (`macos.rs:15-18`) is edited by the staging task; 2.6 says "no reader's reads change" | minor 13 |
| 1.3 point 2: a keyed name at depth on one preset of a platform is at depth on every preset sharing its facts directory (`@gray` on `ios-26`, which has no 0.7.0 look) | minor 14 |
| 2.3 lists the emptied keyed maps as 7.10 does (`tokens.icons.sizes.keyed` on every preset but `kde-breeze`); 1.7's padding row says the drawn height of one line is unchanged (20 + 2 × 8 = 36 under the 40 px minimum) | minor 15 |

## Revision 3

Revision 2 was checked independently (`scratch-keep/d6/staging-design-check2.md`, "the check2": 3 serious, 6
should-fix, 7 minor), and the orchestrator decided three questions (E, F, G below). Every finding was verified at its
evidence before anything changed, the loading findings with `generate_preset.py`'s own code; what changed, and why:

| Change | Answers |
|---|---|
| The generator, its round trip and the gate read the **state model in force**: an element takes its default size in force alone while the sizes unit is off, the default flavour's model alone is in force while the flavours unit is off, and no element holds a variant not at depth; the declarations of sizes, variants and flavours stay whole (2.3, S-3). Run with the generator's own `covering_bases()` and `unstated_fields()` (`gen_bases.py`): revision 2's form leaves 37 button fields at depth with no base on `macos-monterey`, `macos-tahoe`, `ios-13` and `ios-26`; the check2's correction alone (the sizes) still leaves `macos-*`'s, through `route`'s sizeless entry; revision 3's leaves none on any preset, with no round-trip mismatch (7.10) | the check2's serious 1, its correction widened |
| Keyed flags and system settings are declared only where a rule at depth names them; S12 exempts a flavour of a unit not enabled with its sections, which opens no hole (it cannot be asked for, no rule names it, S-4 refuses any that does); a keyed namespace declares only the entries a base states, a map with none stated empty (2.3, 2.4, S-3, S-4, S-7, S-9). S12's unused check: 6 problems in revision 2's form (`kde-breeze`'s `kirigami`, its `@keyboard_focus_change` and its `@single_click`, which the check2 missed; `material-3`'s `expressive`; `@loading` on `ios-*`), 0 in revision 3's (`holes3.py`) | the check2's serious 2; minor 16 |
| `tinted` (typed, on every preset that declares it) and, on `ios-*`, `@gray` at depth, by catalogue corrections C-3d and C-3e; the figures recomputed: 1,376 pending facts on the path (`macos-tahoe` 546, `ios-13` 327, `ios-26` 345; the rest unchanged), 682 listed paths, no register row moving (1.3, 1.6, section 4) | decision E; the check2's serious 3 |
| 1.7's comparison completed: every mapping of the item (font, stroke, corners, padding, shadow, `icon.icon.fill`) and both schemes, with one test and five classes; each value with no source given its evidence, and four values from a source the facts do not take, all corrections listed beside R19's invented light variants; the four corrections of revision 2 kept, with their evidence; `material-3`'s plain button and `ios-13`'s fill moved to class 2 (1.7, R-3, R-4, P-3, P-8, section 8) | decision E; the check2's serious 3, should-fix 4 and 5, minor 17 |
| `material-3`'s checked text toggle proposed as a loss, with the two alternatives (`icon_only`, now +291 against 1,376) and the caveat that Compose 1.5's Expressive `ToggleButton` makes the flavours unit its return (1.7) | decision F |
| The Apple motion trial runs before M2.1's runtime task, capped at two harness rounds per Apple harness, with a stated criterion for a measurement; on success a motion stays, on failure it leaves 0.8.0 on every preset as a loss; what M2.1 builds is stated per case, following the loop (5.2, 5.5, 5.6, R-3, R-4, S-11, C-5, P-1, P-7) | decision G; the check2's should-fix 6 |
| From M4 `check_facts.py` fails a stray fact (a `when` no union of units makes hold), since under (u3) no fact of the enabled units is ever skipped (2.5, P-2) | should-fix 7 |
| A key with no widget is judged under the condition sets C-4 names, the tokens' included (S-6, C-4) | should-fix 8 |
| S17's "takes with it every combination … no earlier fact applies there" reworded for given names (S-6b) | should-fix 9 |
| Minor corrections: `later_units.py` applies the typed-token promotion (10); the example declares `tinted` truly (11); right-to-left and motion are units of depth that take staged paths (12, 3.1, 3.4, P-13); a one-of list is trimmed, not dropped (13); the KDE reader's `dialog` and `panel` icon sizes (14, 2.6); the order of "the first unit" and a pending presence fact (15, 2.5, S-6); the emptied declarations (16, 1.5) | minors 10 to 16 |
| `ios-*`'s `tool`, a typed variant at depth drawn only in the ink toolbar, declared `later` (1.3 point 8, 2.3, C-2, 7.3) | none: revision 2's own `out/defeat2.txt` listed it |
| Revision 2's hole-test control was described as finding holes "on `ios-13` and `ios-26` and none elsewhere"; its own `out/holes_control.txt` has 168 on `macos-monterey` too (2.3's (g1) row, 7.10) | none: found on rereading the outputs |
| Two hand-written blocks lose their place under the trimming, which the generator reports: `ios-*`'s `role.global.accent` read rule (becomes a base) and `macos-*`'s keyed-role bases (staged) (2.5 *(d)*) | found by `gen_bases.py` |
| The flavours unit is a patch release only if U180's decoration form adds no model member (3.3, 3.4, P-5) | `register-rows.md` revision 5, section 2.4 |
| Section 10 holds the check2's findings taken otherwise and the decisions not applied as worded (E's "plain button", G's "the motions") | the brief |

Every figure of revision 2 that revision 3 changes is replaced in place; `depth_count.py --revision 2` still prints
revision 2's tables. Line citations of this file in other documents (`register-rows.md`'s `[staging2]:<line>`) do not
resolve in revision 3.

## Revision 2

Revision 1 was checked independently (`scratch-keep/d6/staging-design-check.md`, "the check": 4 serious, 8
should-fix, 3 minor), and `scratch-keep/d6/register-rows.md` recorded four disagreements with it (its section 2.4,
(a), (b), (d), (e)), which `register-rows-check2.md`, question 6, found right. Every finding was verified at its
evidence before anything changed; what changed, and why:

| Change | Answers |
|---|---|
| The form for given names: the generator leaves out of a bundled preset **every** mention of a name not at depth (a fact naming it positively, a negative entry naming it, its place in an exclusive group or a recency set); the query is unchanged (2.3) | finding 1 (holes on `ios-*`); the check's correction in part (its query half is not taken, section 10) |
| A fact's unit is the predicate of 1.3 (holds in a combination at depth that its state model draws, its part drawn), not the positive names (2.5, S-6) | finding 2 |
| A fifth clause of the token rule: a path or keyed entry a built system reader states is at depth (1.5); section 2.6 lists each reader's targets | finding 3 |
| `contrast_high` at depth, model-wide (1.3, 1.4); the high-contrast unit gone from the later units | finding 4 |
| The comparison with 0.7.0 run for `button` on the seven 0.7.0 presets (1.7), with its verdicts: one proposed loss, four corrections of a visible value, one catalogue selector (C-3b) | finding 5 |
| A typed variant at depth that a preset's state model draws in no combination at depth is declared `later` on that preset (`material-3`'s `toggle`) | finding 6 |
| `tool` at depth, by the comparison of 0.7.0's `toolbar.font` (KDE's facts state it only under `tool`); positions and directions named as kinds of name; the catalogue part of every Depth line read by script (1.3, 1.7) | finding 7; register-rows (d) |
| Every staged path and every condition has a named later unit; M3a's unit 1 ("tokens and conditions") dissolves into the closure (1.5, 3.1, P-4) | finding 8; register-rows (e) |
| `animations_off` and `reduced_motion` at depth for the speed factor, the spinner's loop and the dialog's presentations (0.7.0's `reduce_motion`); a unit for every condition the facts name (3.1) | finding 9 |
| `token.motion.speed_factor` at depth (U515 stays on the path); `color_space` staged with the engines; `Places` built as an empty type; `IconStyle.animation` staged with motion; the connector's own turning arc as (ii)'s stand-in (section 5) | finding 10; register-rows (a) |
| `later` holds variants only; sizes and flavours keep their declarations whole and are answered `Later` while their unit is off; keyed flags are given names; exclusive groups and recency sets list the flags at depth (2.3) | finding 11 |
| Section 6 changes the sentences left standing (plan:1082-1083, :1092-1093, :1104, :1113-1115; rationale:102-103, :1691-1694; spec:7436-7437), one time for the staging task, C-3 as class 2, S-9's list, the plan's class numbers | finding 12 |
| The Depth lists stated once, in the catalogue; the units file holds the units of depth | finding 13 |
| No `[declare] later` for keyed roles, text styles, elevation entries, icon contexts or parameter keys | finding 14 |
| `mnemonics` and `mnemonic_when_disabled` at depth with the kept `mnemonic_decoration` (a fourth clause of the token rule: a token by which a connector draws a value at depth) | register-rows (b) |
| Minor corrections: 925 → the revision's own count (1,520 facts, 2.3); keyed entries taken whole (15 b); 7.8's two fields (15 c); U422's wording (15 d); five citations (15 f); the host-context row (15 g); `repeat_delay` and `repeat_interval` with `@repeat` (15 h) | finding 15 |
| Every count recomputed under the revised rule (`depth_count.py`, now revision 2 by default; `--revision 1` reprints the first note's tables) | step 2 of the task |

The check's *Decisions I confirm* stand: the depth rule (B), `with_icon` at depth, option 1 with 4 for typed
variants, `later` with `NotAvailable(Later)` (now for variants, sizes and flavours), no entry on four platforms
alone, `ios-*`'s `hover` and `focus_visible` with the iPad unit, the token rule's three clauses (now five), the
motion choice (i) behind a trial with (ii) per value, section 3.5's method.

**Read at:** the worktree `/home/tibi/Rust/native-theme/.claude/worktrees/m1b-close`, branch `m1b-close`, HEAD
`7430a0a2`; 0.7.0 at the tag `v0.7.0` (`caeddd6b`). "plan", "rationale", "spec", "catalogue" and "ledger" are
`docs/todo_v0.7.1_model-completeness-{plan,rationale,spec,spec-widgets,ledger}.md` there; a number after a colon is
a line of that file at that commit.

**The counts** are counts by a stated rule, never measurements. They come from scripts beside this note, in
`scratch-keep/d6/staging/` (section 9 gives each command); their outputs are in `staging/out/` (revision 2's carry a
`2` in the name, or are new). The scripts approximate the gate: they match a fact's `when` against combinations with
no ancestor and no sibling (S11's matching) and do not run the generator. Where a figure depends on a reading of
mine, the text says so.

**Words.** A *name* is what a selector can name beside the scheme: a variant, a control size, a state flag, a host
context, a position among siblings, a direction, a condition, a flavour. An *asked* name is one an application or a
connector puts on an element or on the theme (a variant, a size, a flavour). A *given* name is one the environment
supplies (a state flag the toolkit reports, a keyed flag the connector sets, a host context or a position the
element tree holds, a direction the scope holds, a condition a reader or a window reports). A *unit of depth* is a
later unit that holds names, or paths, of entries whose own unit is already enabled. A name is *at depth* for an
entry when the entry's Depth line (C-1) lists it.

---

## 0. The design in short

1. **The depth rule.** A unit's entry enters with every part, field and value it has (less what S6.1 defers), in
   every combination of the names 0.7.0's model distinguishes for it: the names the *Today's fields* selectors of
   any entry name for it, those a state model requires for one of them, those a preset's `requires_variants`
   lists, and the conditions under which 0.7.0's readers give values. For `button`: the variants `primary`,
   `toggle`, `with_icon`, `tool` and `tinted`, and on `ios-*` the keyed `@gray` (the two that hold 0.7.0's plain
   button on `material-3` and `ios-13`, 1.7); the states `hover`, `pressed`, `disabled`, `checked`, `default` and
   the two focus flags; the default size in force; no host context in the slice; the scheme and `contrast_high`. Typed
   colour roles enter whole, under the scheme, `contrast_high` and the conditions the table names. A token path,
   and a keyed name of any kind, is at depth when 0.7.0 has it, a value at depth reads it, the core reads it, a
   connector reads it to draw a value at depth, or a built system reader states it.
2. **The form.** By what the type system can express: a typed variant beyond depth is not a member of `Variant`
   until its unit (the generator, as S6.1 defers a value); a keyed variant beyond depth stays declared and is listed
   in the widget's `later`, and a size or a flavour of a unit not enabled keeps its declaration, the query and
   `set_flavor` answering `Later`, never "the platform has none"; a given name is mentioned by no rule of a bundled
   preset, so an element under it resolves as without it, and each connector's contract records it. The generator
   reads the state model restricted to the names at depth for its bases (the default size in force, the default
   flavour, the variants at depth), so every field at depth keeps a base; a keyed flag or a system setting is
   declared only where a rule at depth names it, a keyed entry only where a base states it, and S12 exempts a flavour
   whose unit is not enabled. A fact is of the enabled units when it holds in a combination at depth that its state
   model draws (its part drawn); no fact is edited.
3. **The later units.** Units of depth per widget unit (content, styles, menus, identities, states) and shared ones
   (sizes, flavours, increased contrast, window activity, the iPad, accessibility text, interface level, glass,
   right-to-left, motion), and parking units for the token paths no value at depth reads (behaviour, text
   rendering, text styles, icon contexts, elevation, parameters, keyed roles, pointer). Every fact off the path is
   assigned to one (`later_units.py`: none unassigned).
4. **The release path.** Of the slice's 45,766 pending facts, 1,381 stay on 0.8.0's path (3.0 %): `adwaita` 0,
   `kde-breeze` 5, `windows-11` 54, `material-3` 8, `macos-monterey` 231, `macos-tahoe` 547, `ios-13` 328,
   `ios-26` 346 (a fact of two presets counted on each). Of the 1,508 listed paths 687 stay. `adwaita`'s table is
   empty under the rule.
5. **Motion.** 0.8.0 holds `dialog.values.presentations` and `spinner.values.loop`, with the interface of the motion
   runtime, the speed factor and no engine, if a capped Apple motion trial, run before M2.1's runtime task, measures
   both motions on the Apple presets (0.7.0 states neither there, and the model has no truthful way to state a value
   on some presets only). A motion it does not measure leaves 0.8.0 on every preset, a loss for the maintainer; M2.1
   builds `Motion` if the loop stays, and otherwise only for a connector whose design cannot show that the motion
   unit needs no change of its API (5.5, 5.6).
6. **What would defeat it.** Checked on the slice's facts: the deferral changes no fact that holds in a combination
   at depth, no value at depth reads a deferred thing, no fact at depth mentions a name not at depth, every field at
   depth keeps a base in the preset the generator writes (7.10, run with the generator's own code), and no recency set
   or keyed entry is left declared that S12 refuses as unused (the keyed flags, settings and flavours are so by the
   form's definition; S12's own check on the regenerated presets is the staging task's gate). The comparison with
   0.7.0's `button` (1.7), every mapping and both schemes, brings `tinted` and `@gray` to depth, proposes one loss to
   the maintainer (`material-3`'s checked text toggle) and lists the values the facts correct, the look of a button
   with no variant on `material-3` and `ios-13` among them (a correction of 0.7.0's mapping).

---

## 1. What "0.7.0's depth" is

### 1.1 The facts

- 0.7.0's `ButtonTheme` holds 15 fields, a font and a border: a base look, `primary_*`, `hover_*`, `active_*`,
  `disabled_*`, `checked_*` (`git show v0.7.0:native-theme/src/model/widgets/mod.rs`, lines 43-103). A preset is a
  full light and a full dark `ThemeMode` (rationale:89-90). Motion is "`dialog.entrance` and the animated icons"
  (rationale:100-101). 0.7.0 has no control size, no host context, no variant but primary and no flavour.
- The rationale says accessibility is "four run-time flags … with no theme values behind them" (rationale:102-103).
  That is true of 0.7.0's presets, not of its readers: under high contrast the Windows reader wrote the eight system
  colours into the theme (`v0.7.0:native-theme/src/windows.rs:315-446`, `apply_sys_colors`, called at `:597-600`),
  the GNOME reader computed the focus ring under high contrast (`v0.7.0:native-theme/src/gnome/mod.rs:180-182`),
  and the macOS reader's colours follow Increase Contrast; 0.7.0's egui connector says so
  (`v0.7.0:connectors/native-theme-egui/src/lib.rs:116`: "high contrast belongs in the theme data, which a reader
  fills with what the platform draws under it"). Its `reduce_motion` holds the busy indicator still
  (`v0.7.0:connectors/native-theme-egui-widgets/src/spinner.rs:95`, `:112`, `:240-270`) and zeroes every style's
  animation time (`v0.7.0:connectors/native-theme-egui/src/lib.rs:114`).
- The new readers carry the same: the Windows reader states the eight `role.@system_*` in a rule under
  `contrast_high` (`native-theme/src/windows.rs:377-392`); the macOS reader states its roles in a rule under
  `contrast_increased` when Increase Contrast is on (`native-theme/src/macos.rs:359-372`); the GNOME, KDE and
  Windows readers report `animations_off` and `reduced_motion` (`gnome/mod.rs:217-218`, `kde/mod.rs:235-236`,
  `windows.rs:334-335`), the macOS reader `reduced_motion` (`macos.rs:322`).
- The catalogue maps each 0.7.0 field to a target and a selector: per entry in its *Today's fields* item (116 items,
  `grep -c "^- \*\*Today's fields:\*\*"` on the catalogue; `button`'s at catalogue:2054-2064), and for `defaults`
  and the text scale in the *today's fields* table (catalogue:10269-10304). An entry's mapping may target another
  entry: `toolbar`'s `font` is `button.label.text @ context toolbar.bar` and its `icon_size_px`
  `button.icon.icon.size @ context toolbar.bar` (catalogue:4729-4731), `sidebar`'s is `list.row.text @ variant
  sidebar` (catalogue:4579). `python3 todays_selectors.py` lists every selector of the 116 items: across the 30
  entries they name the variants `primary`, `vertical`, `sidebar`, `alternating_rows`; the states `hover`,
  `pressed`, `disabled`, `checked`, `focused`, `selected`, `visited`; the contexts `toolbar.bar` (into `button`)
  and `any slider {hover}`; the condition `window_inactive`; and one position, `list`'s row parity
  (`pos parity … variant alternating_rows`). Only `toolbar`'s two mappings target `button` from another entry.
- The brief's count took the 22 pairs of `button`'s item literally: a key under one of 22 targets with exactly that
  pair's variants and states (`scratch-keep/d6/scripts/depth.py`, docstring): 308 of 63,343 button facts, 23 of them
  pending.
- Every state model draws `checked` only with `toggle`: `{ states = ["checked"], not_variants = ["toggle"] }` is a
  `not_drawn` entry of each (`docs/facts/gnome/state-models.toml:50`, `kde/…:51` and `:272`, `macos/…:59` and
  `:391`, `ios/…:125` and `:941`, `windows/…:28`, `material/…:56` and `:283`).
- A fact's `when` is a conjunction of positive and negative lists, and "the fact that applies is the **last** in file
  order whose `when` holds" (spec:7473-7474). So a fact that names a name in a positive list holds in no combination
  that lacks the name.
- `check_facts.py` fails a pending fact that a later value fact shadows (spec:7476-7479), so a pending fact applies
  wherever its `when` holds unless a later pending fact does.
- S12 judges a missing field "under each assignment of the selectors' atoms (each condition, state flag, variant,
  control size, … the rules name)", and "Every other combination is assumed possible" (spec:6028-6046): an atom
  any rule of the stack names is judged true and false.

### 1.2 The options for the rule

- **(A) The pairs alone**, as the brief counted them: a key is at depth only under the exact selector its 0.7.0
  field maps to. Gives the smallest path (308 facts). Costs: a `primary` button's stroke, a button pressed while
  hovered and the label's padding have no pair, so they resolve from the base or from another state's rule, the case
  R4 was written against ("a combination no rule states gets what the earlier rules give: the platform's own answer
  where the platform composes the same way, and not necessarily elsewhere", rationale:522-523). It is also not
  expressible: 12 of the 22 targets are a block or a leaf of one (`label.text.font`, `label.text.fill`,
  `icon.icon.fill`, `body.stroke`, `body.corners`, `body.padding`, `body.shadow`), whose other leaves are fields of
  the same Rust type and are stated with it unless S6.3 lets them be absent (spec:4031-4072; S12). Rejected.
- **(B) Names by the pairs, paths whole.** The names (variants, states, contexts, conditions) come from the pairs'
  selectors and from what 0.7.0 delivered under them; under every combination of those names the entry is stated
  whole, every part, field and value. Gives the unit of R21 ("every capability filled", rationale:1400) on a smaller
  set of combinations, and one test per axis: is the name one 0.7.0's model distinguishes for this entry? Costs more
  facts than (A): 7,227 against 308 (revision 3's variants, 1.3).
- **(C) (B) with every state flag** of the state model. For: one axis fewer to stage. Against: 1,674 pending facts
  at depth against 1,381 (`python3 depth_count_a2.py --values-at-depth --states=all`); the further flags (`open`, `drop_target`,
  `indeterminate`, `selected`, `dragged`, `route_active`, the keyed flags) are names 0.7.0's button does not
  distinguish, and two of them wait on Apple passes of their own (U693, a mixed-state toggle; U475, a drag over an
  iOS button). Rejected, on the criterion itself.
- **(D) Change nothing:** a unit enters with every name its state models list. Decided against by outcome 2.

**Chosen: (B).** The check confirmed it.

### 1.3 The rule for a widget

For a catalogue entry `W` whose unit is enabled:

1. **Paths.** Every part, field and value of `W`'s entry is at depth, less: the values S6.1 defers (the `Placement`
   values); the fields and values of the motion unit (section 5); a value that only names of later units read, which
   `W`'s entry marks (for `button`: `menu_indicator_gap`, `menu_indicator_anchor`, `menu_trigger`,
   `back_title_fallback`, `back_display_mode`, `help_tooltip_on_click`, `display_mode`; my reading of
   catalogue:1617-1724 and :2024-2027, :2130, checked by the batch that writes the mark, which found
   `label_position`, `repeat_delay` and `repeat_interval` stated with no later name, so they are at depth: the fix
   round's correction above). A part no combination at depth draws (`button.menu_indicator`, drawn under `menu` alone,
   catalogue:2001-2013) stays a part; its fields need no fact where it is not drawn (spec:7047-7049).
2. **Variants.** The typed variants a selector of a *Today's fields* mapping names for a target in `W` (in any
   entry's item), those a state model requires for a state so named, and those a preset's `requires_variants` lists
   (finding C, 7.6). For `button`: `primary` (catalogue:2056-2057), `toggle` (required by `{checked}`), `with_icon`
   (C-3), `tool` (C-3c, by 1.7's comparison of `toolbar`'s two mappings), `tinted` (C-3e) and, on `ios-*`, `@gray`
   (C-3d): the variants under which a preset's facts hold 0.7.0's look of the plain button (1.7). A keyed variant is
   at depth only where a preset's own mapping names it (C-3d); 0.7.0's fields are portable, and a keyed name is one
   platform's. A keyed name at depth for one preset of a platform is at depth on every preset that shares its facts
   directory, so a portable application gets one answer per platform: `@gray` is at depth on `ios-26` too, which
   0.7.0 did not have and which has no 0.7.0 look to keep, by C-3d's per-platform wording ("on `ios-*`"; its +8
   pending facts there, below). A typed name is a member of the model on every preset or on none (section 3.3), so `tinted` is at
   depth on every preset that declares it (`macos-tahoe`, `ios-13`, `ios-26`, `material-3`); a preset whose state
   model draws a typed variant at depth in no combination at depth declares it `later` (point 8).
3. **States.** The flags those selectors name; the flags under which a 0.7.0 token is drawn (`focused` and
   `focus_visible` where a part of `W` lists `focus`: 0.7.0's `focus_ring_*` is `token.focus`, catalogue:10292); and
   every combination of those flags the state model draws. For `button`: `hover`, `pressed`, `disabled`, `checked`,
   `default` (`primary_background` is `@ {default}` on `macos-*`, catalogue:2056, and on `kde-breeze` by C-3b),
   `focused`, `focus_visible`.
4. **Sizes.** The default size in force (S10) alone: 0.7.0 has one size. In a context whose `sizes_by_context`
   entry states another default (`macos-tahoe`'s toolbar, `extra_large`, catalogue:1954-1958), that size is the one
   in force there.
5. **Host contexts, positions, directions.** The contexts a mapping's selector names for a target in `W`, once the
   host's unit is enabled: for `button` none in the slice, `toolbar.bar` from the containers unit on
   (catalogue:4729). Positions where a mapping names one (`list`'s row parity); otherwise the element alone. The
   default direction; `rtl` is the right-to-left unit's. In the slice every fact naming a position or `rtl` also
   names a host context (GNOME's linked layout, 27 facts) or a deferred variant (KDE's `@command_link`, 1), so it
   goes with that unit.
6. **Conditions.** The scheme; the preset's default flavour; `contrast_high`, model-wide, for every entry, role and
   token (0.7.0's Windows and GNOME readers give values under it, 1.1); a condition a mapping's selector names, for
   the entry of its target, whole (`window_inactive` for `window`, whose inactive title bar 0.7.0 states, and for
   the `selection` surface set, catalogue:10287; `overlay_scrollbars` for `scrollbar`, plan:731); `animations_off`
   and `reduced_motion` for the motion values 0.8.0 holds and the tokens they read (section 5: 0.7.0's
   `reduce_motion`; the spinner's `loop` is stated "by rules under `cond animations_off`", catalogue:3611-3613).
7. **A name only a later name draws** goes with it: on `ios-*`, `hover` and `focus_visible` are the iPad's (U112 and
   U113's own words: "under `focus_visible` (iPad)", "under `hover` (iPad pointer)", ledger:508-509), so they are
   the iPad unit's there.
8. **A variant at depth drawn nowhere at depth on a preset** is declared `later` on that preset (2.3), so an
   application asking for it is answered `Later`, not drawn as the base. Two cases (`python3 defeat_checks.py`,
   check 4): `material-3`'s `toggle`, which baseline Material draws only with `icon_only`
   (`docs/facts/material/state-models.toml:56`), the content or the flavours unit's; and `ios-*`'s `tool`, which
   UIKit draws only in PencilKit's ink toolbar (`{ variants = ["tool"], not_context = [{ scope = "any", widget =
   "ink_toolbar" }] }`, a `not_drawn` entry, `docs/facts/ios/state-models.toml:145`, `:964`), its host's unit.
   (Revision 2 named the first alone; its own `out/defeat2.txt` lists both.) It is computable from the state model
   and true: the platform has the variant, and a later unit states it.

A combination is **at depth** when it holds only names at depth. A fact is **at depth** when its key is a path at
depth, its `when` holds in a combination at depth that the state model draws, and its part is drawn there. Every
other fact of `W` belongs to a later unit (section 3; `later_units.py`). This is also the definition of a fact *of
the enabled units* (2.5).

**The corrections the rule needs** (each class 2 by finding 12: a *Today's fields* selector decides what a release
states):

- **C-3, `icon_text_gap`.** `button`'s item maps it to `values.icon_text_gap` with no selector
  (catalogue:2055-2056). On `adwaita` that value's base is the CSS initial `0px` and the gap of a button holding an
  icon and a label is stated under `with_icon`, `6px` (`docs/facts/gnome/button.toml:809-816`, `:3861-3869`);
  0.7.0's `adwaita` states `icon_text_gap_px = 8.0` (from no source at a line, 1.7). With `with_icon` deferred a
  connector would draw an icon
  against its label with no gap. So the selector becomes `@ variant with_icon`, "the base where a preset declares no
  `with_icon`" (`windows-11`, whose base fact is pending U517). On `ios-*` the `icon` part is absent without
  `with_icon` or `icon_only`, so `icon.icon.fill` (the `primary_text_color` mapping) has no combination at depth
  without it either (the check). Its cost: 1,009 pending facts at depth with `with_icon` against 726 without
  (`python3 depth_count.py --variants primary,toggle,with_icon`, `--variants primary,toggle`).
- **C-3b, `primary_*` on `kde-breeze`.** Breeze has no `primary` variant (`native-theme/src/presets/kde-breeze.toml`,
  `[declare.widget.button] variants`); its default button is the `{default}` state, stated at depth
  (`docs/facts/kde/button.toml:1335` on). 0.7.0's `kde-breeze` inherited `primary_background` from the accent,
  `#3daee9` (`v0.7.0:…/kde-breeze.toml:18`). So the item reads "(on `macos-*` and `kde-breeze`: `@ {default}`)". No
  fact moves.
- **C-3c, `toolbar`'s two mappings.** 0.7.0's KDE reader wrote `toolBarFont` into `toolbar.font`
  (`v0.7.0:native-theme/src/kde/fonts.rs:197-200`); the KDE facts state the toolbar's font only for a tool button,
  `button.label.text.font.* @ variant tool` = `token.text.keyed.toolbar.font.*`
  (`docs/facts/kde/button.toml:8990-9087`), which the catalogue's note confirms ("Qt's tool button, WinUI's
  AppBarButton and AppBarToggleButton and AppKit's toolbar bezel are the typed `button @ variant tool`",
  catalogue:4732-4734). Every `windows-11` button fact with a positive `toolbar.bar` context names `tool` (31,
  register-rows-check2, finding 5, recounted by `toolbar_check.py`'s reading). So the two mappings become `@ variant
  tool, context toolbar.bar` where a preset declares `tool`, `@ context toolbar.bar` where it does not (`adwaita`,
  `material-3`), and `tool` is at depth from the slice on. Its cost, on revision 2's list: 1,206 pending facts at
  depth with `tool` against 1,009 without (`python3 depth_count.py --revision 2`, `--variants=primary,toggle,with_icon`):
  the button's pending facts at depth `macos-monterey` 103 → 171 (+68), `macos-tahoe` 213 → 341 (+128),
  `windows-11` 17 → 18 (+1).
- **C-3d, the plain button's fields on `ios-*`: `@ variant @gray`.** 0.7.0's `ios-13` drew its plain button as
  WebKit's iOS button, the grey `opaqueSecondaryFill` (`v0.7.0:native-theme/src/presets/ios-13.toml:132-133`,
  `html.css:895`). UIKit's system button (`UIButton(type: .system)` with no configuration, the facts' base,
  `docs/facts/ios/state-models.toml:19-20`) draws no fill; the same grey is UIKit's gray configuration's, measured
  (`button.body.fill @ variant @gray` = `#78788029` light, `#78788052` dark, `docs/facts/ios/button.toml:3110-3114`,
  `:2470-2474`; composited over the system background, `#ffffff` and `#000000` (`role.@system_background`,
  `docs/facts/ios/roles.toml:297-310`), about `#e9e9eb` and `#272729` against 0.7.0's `#e9e9ea` and `#262629`, one
  level apart). By the orchestrator's decision E the look stays at depth: on `ios-*` every mapping of the item but
  `primary_*`'s takes `@ variant @gray`, beside its own variant where it has one (`icon_text_gap @ variant @gray,
  with_icon`; the catalogue's words are C-3d in 6.3), and `@gray` is at depth there. Its cost: 12 pending facts (`--variants=@gray,primary,toggle,tool,with_icon`: 1,218 against
  1,206; `ios-13` +4, `ios-26` +8).
- **C-3e, the plain button's fields on `material-3`: `@ variant tinted`.** 0.7.0's `material-3` drew its plain
  button as Material's filled tonal button (`v0.7.0:docs/platform-facts.md` §2.3, `background_color`: "`comp.filled-
  tonal-button` `container-color` = `secondary-container`"); the facts state Material's base button as the filled
  one (`md.comp.button`, `= role.@primary`, `docs/facts/material/button.toml:36-44`) and the tonal one under
  `variant tinted` (`= role.@secondary_container`, `:1766-1769`, citing
  `material-web@2.5.0 tokens/versions/latest/sass/_md-comp-button-tonal.scss:29`). By decision E the same mappings
  take `@ variant tinted` on `material-3` (C-3e in 6.3), and `tinted`, a typed variant, is at depth on every preset
  that declares it. Its cost: 158 pending facts (`--variants=primary,tinted,toggle,tool,with_icon`: 1,364 against 1,206;
  `macos-tahoe` +146, `ios-13` +4, `ios-26` +8, `material-3` 0).

Together C-3d and C-3e cost 170 pending facts: 1,376 at depth against 1,206 (`python3 depth_count.py`, the default;
`--revision 2` reprints 1,206; with the fix round's values at depth 1,381 against 1,211, the same 170). What they keep is 0.7.0's look in the model, at depth, under a variant. An element
with no variant draws each platform's own default button: on `material-3` Compose's `Button`, the filled one
("Filled buttons are high-emphasis buttons", `androidx@e4d933c9 …/material3/Button.kt:77-88`; the tonal one is the
separate `FilledTonalButton`, "medium-emphasis", `:439-447`; the facts' base `md.comp.button`, `= role.@primary`,
`docs/facts/material/button.toml:36-44`), and on `ios-*` UIKit's system button with no configuration
(`UIButtonTypeSystem`, "standard system button", `xcode@26.6 UIKit/UIButton.h:22-23`; `UIButtonConfiguration` has no
default, its `new` and `init` unavailable and only the style factories, `plainButtonConfiguration` among them,
offered, `UIButtonConfiguration.h:78-94`; the facts' base `UIButton(type: .system)`,
`docs/facts/ios/state-models.toml:19-20`). This changes the look of a button with no variant on `material-3` and
`ios-13` against 0.7.0, and it is a **correction of 0.7.0's mapping**: 0.7.0 mapped its plain button to a look that
is not the platform's default (Material's tonal button; WebKit's HTML button, `html.css:895`), and 0.7.0's look stays
reachable under `tinted` and `@gray`. It is listed with the corrections (1.7, R-3, P-3, 8.9); a connector that wants
0.7.0's look asks for `tinted` or `@gray` (section 10.2).

`icon_only` stays deferred: no 0.7.0 field describes an icon-only button, and adding it costs 291 more pending
facts (`--variants=@gray,icon_only,primary,tinted,toggle,tool,with_icon`: 1,667 against 1,376, with the fix round's
values at depth 1,672 against 1,381; revision 2's 254 was against its own list).

### 1.4 The rule for colour roles

- **Typed roles enter whole on the path axis:** every typed role of every surface set a preset declares. They are
  fields of shared Rust structs, 0.7.0's `defaults` map into most of them (catalogue:10275-10293), and the slice's
  values read them throughout. Rejected: typed role fields staged by 0.7.0's list (surgery on shared types that
  M3a's units then undo).
- **Their conditions at depth:** the scheme; `contrast_high`, model-wide (1.3, point 6); and `window_inactive` for
  the `selection` surface set, whole (0.7.0's `selection_inactive_background`; 24 facts: `kde` 12 and `macos` 6
  stated, `ios` 6 pending). A role fact under any other condition is its condition's unit's: `contrast_increased`
  (increased contrast), `window_inactive` elsewhere (window activity), `elevated` (interface level), `tint_dimmed`
  (accessibility text), `idiom:pad` (the iPad).
- **A keyed role** (`role.@<name>`) follows the token rule, as every keyed name does.

### 1.5 The rule for tokens, and for every keyed name

A token path, a keyed role, a keyed text style, an elevation entry, a keyed icon context or a parameter key is at
depth when one of five holds:

1. the *today's fields* table maps a 0.7.0 field to it or to the block that holds it (catalogue:10275-10303): the
   slots `body`, `monospace`, `caption`, `headline`, `title1`, `display_small`; `token.focus`;
   `token.params.opacity.disabled`; `token.icons.sizes.{toolbar,small,large,dialog,panel}`, `token.icons.set`,
   `.theme`, `.theme_fallback`;
2. a value at depth reads it, directly or through another value at depth (the reference closure: `token.…`,
   `role.…`, `input.…` in an expression, and a shadow's `{ elevation = "<key>" }`);
3. the core reads it by itself: `token.size_classes` (S8's `size_classes`, spec:4434), `token.text.font_dpi` and
   `token.text.scaling` (S9.4), `token.kde` and `token.material` (S4.8's functions). This list is read from the
   specification; the task that applies the rule confirms it in the code;
4. the specification has a connector read it to draw a value at depth: `token.behavior.mnemonics` and
   `mnemonic_when_disabled`, by which "the connector … draws no `mnemonic_decoration`" (spec:3518-3525), for the
   `mnemonic_decoration` leaf every text block holds (`TextStyle`, spec:2318). The staging task lists these from the
   specification's text, as it lists the core's reads;
5. a built system reader states it (2.6): on `kde-breeze` the keyed text styles `toolbar` and `menu`, the icon
   context `main_toolbar`, the four `role.@wm_*` and `token.params.color`'s `contrast` and `frame_contrast`; on
   `macos-*` the keyed roles `@system_red`, `@system_orange`, `@system_green`, `@system_blue` and
   `@selected_control_secondary`; on `windows-11` the eight `role.@system_*`; and `token.motion.speed_factor`.

A block at depth is at depth whole: its leaves are fields of a shared type; a reference into an entry of a keyed map
of blocks (a keyed text style, icon context or elevation entry) takes the entry whole. A typed token field at depth
on one preset is at depth on every preset. An entry of a keyed map is each preset's own. `input.*` is the model's
core (S9.1) and enters whole. Everything else is **staged to the first unit that reads it**, and waits meanwhile in
a named parking unit (section 3.1): a path an M3a unit's value reads leaves its parking unit in that unit's commit,
which `generate_preset.py --check` verifies; a path no 0.8.0 value reads stays in its parking unit.

This is the M1a design's decision 9 at path grain ("`pointer` to the first unit that reads it … `elevation` and
`size_classes` stay … so the slice is the first unit that reads them", m1a-design:153-164), and it is the test
outcome 14 already applies to U473: "if 0.7.0's model holds no such value on iOS it is a later unit's". 0.7.0 holds
no key repeat (`git grep -i key_repeat v0.7.0 -- native-theme/src/model native-theme/src/presets` prints nothing),
so U473 leaves 0.8.0's path, into the behaviour unit.

Options weighed: tokens whole, as the slice built them (913 listed paths and every decision row among them on the
path, for values 0.7.0 never had); the table alone, with no closure (a value at depth would refer to a path the
model lacks); the table and the closure alone, as revision 1 had it (the readers' layers refused, finding 3, and a
kept leaf whose drawing its staged tokens decide); the rule above. **Chosen: the rule above.** Its code cost, read
from `native-theme/src/tokens.rs`: the staging task removes from the hand-written structs `MotionTokens.durations`,
`.timings`, `.color_space` (`:97-111`), `TextRendering` and `FontFallbacks` from `TextTokens` (`:643-646`), the
nine slots of 1.5's last list from `TextRoles` (`:427-617`), `Caret` from `Tokens` (`:62`), every `Behavior` field
but `mnemonics`, `mnemonic_when_disabled` and `focus_visible_when` (`:290-353`), `IconTokens.mirroring` and
`.animated_visuals` (`:706-711`), `IconStyle.animation` (spec:2511), `FocusIndicator.transition` and
`token.theme_change`, as `Behavior.scroll` was removed (spec:3883-3885). With the structs go the declarations they
read and the rows of `DECLARATIONS` that check them (`validate/checks.rs:352` on, `row("MotionTokens", "durations",
…)`, `"timings"`, the `LcdFilters` row), and in the presets' hand-written `[declare]` blocks the keyed entries no base
at depth states: every preset's `timings` and `durations`, `kde-breeze`'s `lcd_filters` and `elevation` (16 of 16),
and most `text_styles`, `color_roles` and parameter keys (2.3; `python3 holes3.py`, part C, the check2's minor 16). I
did not count the lines or the tests that change; the staging task's first compile settles it (section 8).

At the slice the rule stages (`python3 token_paths.py`, `out/token_paths2.md`): `token.behavior` but
`focus_visible_when`, `mnemonics` and `mnemonic_when_disabled`; `token.caret` (back with the fields unit, whose
`input.caret.fill` reads `token.caret.color`, catalogue:2923); `token.text.fallbacks` and `.rendering`;
`token.icons.mirroring` and `.animated_visuals`; the slots `body_emphasized`, `callout`, `caption2`,
`caption_emphasized`, `display_medium`, `footnote`, `label`, `numeric`, `subheadline`; `token.motion` but
`speed_factor`, and `token.theme_change` (section 5); the parameter groups and keyed entries no value at depth reads
and no reader states.

### 1.6 The counts

`python3 depth_count.py --json out/final3.json --rows --listed > out/final3.md`, at `7430a0a2` (revision 3: the
variants of 1.3 with `tinted` and `@gray`; `--revision 2` reprints revision 2's `out/final2.md`, checked by `diff`). A
fact of two presets counts for each in the first table and once in the second. "Deferred" is a fact of a later unit
(for the roles: one under a condition not at depth, or a keyed role the token rule stages); "staged" a token path the
rule stages. The tables are the fix round's (`python3 depth_count_a2.py --values-at-depth`, the three values of the
correction above at depth); `depth_count.py` still prints revision 4's (1,376 and 682).

| Preset | Button facts at depth | pending | deferred | pending | Roles and inputs at depth | pending | deferred | pending | Tokens at depth | pending | staged or deferred | pending | Listed paths that stay | leave |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `adwaita` | 86 | 0 | 289 | 0 | 495 | 0 | 13 | 0 | 183 | 0 | 335 | 4 | 0 | 4 |
| `kde-breeze` | 177 | 5 | 511 | 12 | 715 | 0 | 77 | 0 | 236 | 0 | 478 | 11 | 4 | 12 |
| `macos-monterey` | 407 | 172 | 8,569 | 4,586 | 209 | 1 | 114 | 0 | 172 | 58 | 922 | 233 | 94 | 264 |
| `macos-tahoe` | 1,139 | 488 | 18,903 | 11,021 | 209 | 1 | 115 | 0 | 172 | 58 | 922 | 100 | 77 | 144 |
| `ios-13` | 539 | 201 | 15,149 | 14,042 | 236 | 72 | 200 | 37 | 154 | 55 | 822 | 122 | 221 | 154 |
| `ios-26` | 551 | 218 | 16,362 | 15,484 | 236 | 72 | 199 | 37 | 154 | 56 | 822 | 122 | 227 | 154 |
| `windows-11` | 537 | 19 | 349 | 26 | 369 | 1 | 150 | 0 | 140 | 34 | 144 | 38 | 55 | 50 |
| `material-3` | 154 | 0 | 1,112 | 15 | 142 | 1 | 36 | 2 | 175 | 7 | 591 | 50 | 9 | 39 |

| Facts directory | Facts | At depth | of them pending | Deferred or staged | of them pending | Other units' | of them pending |
|---|---:|---:|---:|---:|---:|---:|---:|
| `gnome` | 1,401 | 764 | 0 | 637 | 4 | 0 | 0 |
| `kde` | 2,220 | 1,128 | 5 | 1,066 | 23 | 26 | 2 |
| `macos` | 31,940 | 2,306 | 778 | 29,545 | 15,940 | 89 | 63 |
| `ios` | 32,848 | 1,512 | 536 | 31,262 | 28,217 | 74 | 4 |
| `windows` | 1,690 | 1,046 | 54 | 643 | 64 | 1 | 1 |
| `material` | 2,210 | 471 | 8 | 1,739 | 67 | 0 | 0 |
| **all** | 72,309 | 7,227 | 1,381 | 64,892 | 44,315 | 190 | 70 |

The 1,720 facts not in the tables are the KDE accent run's (`scheme_file` and `accent`, S17; 880 role and 840 input
facts, all measured): they stay, with the roles. 72,309 + 1,720 = 74,029; 1,381 + 44,315 + 70 = 45,766.
"Other units'" are the facts lanes wrote ahead for widgets not enabled (`checkbox`, `combo_box`, `menu`, `radio`,
`window`, `component.title_widget`), which the gate skips today.

Pending facts at depth per preset, the sum of the three "at depth" pending columns: `adwaita` 0, `kde-breeze` 5,
`macos-monterey` 231, `macos-tahoe` 547, `ios-13` 328, `ios-26` 346, `windows-11` 54, `material-3` 8 (revision 4:
0, 4, 230, 546, 327, 345, 53, 8, also `python3 rows_summary.py out/final3.json > out/rows3.md`, first table; the
fix round adds one `label_position` fact on each preset but `adwaita` and `material-3`). Against revision 2 (0, 4, 230, 400, 319, 329,
53, 8): `tinted` adds 146 on `macos-tahoe`, 4 on `ios-13` and 8 on `ios-26`, `@gray` 4 on `ios-13` and 8 on
`ios-26`, 170 in all (C-3d, C-3e; each variant alone, revision 4's recount: `python3 depth_count.py
--variants=primary,tinted,toggle,tool,with_icon` gives `ios-13` 323 and `ios-26` 337, `--variants=@gray,primary,toggle,tool,with_icon`
the same two, against 319 and 329); nothing else moves, and no register row moves between leaving and staying (`diff out/rows2.md
out/rows3.md`: the pending counts of U91, U92, U95, U99, U100, U115, U682, U695 and U750 alone). Against revision 1
(0, 4, 168, 278, 325, 335, 51, 8), revision 2's `tool` added 68, 128 and 1 on the two macOS presets and `windows-11`,
`IconStyle.animation` moved 6 a preset on the four Apple presets and 2 on `windows-11` to the motion unit, and the
mnemonic tokens (2) and the speed factor (1) came back on `windows-11`.

The listed paths that stay, by class (`out/rows3.md`; the same as revision 2's, `tinted` and `@gray` holding no
listed path that does not stay already; the fix round adds `widgets.button.values.label_position` on `macos-*`,
`ios-*` and `windows-11`):

| Preset | Button | Colour roles | Inputs | Tokens | All |
|---|---:|---:|---:|---:|---:|
| `adwaita` | 0 | 0 | 0 | 0 | 0 |
| `kde-breeze` | 4 | 0 | 0 | 0 | 4 |
| `macos-monterey` | 34 | 2 | 0 | 58 | 94 |
| `macos-tahoe` | 17 | 2 | 0 | 58 | 77 |
| `ios-13` | 34 | 132 | 0 | 55 | 221 |
| `ios-26` | 39 | 132 | 0 | 56 | 227 |
| `windows-11` | 19 | 2 | 0 | 34 | 55 |
| `material-3` | 0 | 0 | 2 | 7 | 9 |
| **all** | 147 | 270 | 2 | 268 | 687 |

Of the 1,508 listed paths: 315 are button paths, of which 147 stay; 278 colour-role paths, 270 stay (60 on each iOS
preset are U456's status roles, which outcome 5 closes by its mapping); 2 inputs, both stay; 913 token paths, 268
stay. So the test stages the token values (645 of 913 leave) and almost no colour role (8 of 278: `material-3`'s four
keyed menu-role paths, which no value at depth reads, and the four `global.accent` paths of the two iOS presets,
whose pending facts hold under `tint_dimmed` or `idiom:pad` alone).

What the figures rest on, beside the rule: the script's reading of S11's matching; the seven values of 1.3's point
1 (ten before the fix round); the core reads of 1.5; the reader targets of 1.5's clause 5, read from the code (2.6);
and four whole-map facts treated as staged (section 8, point 3). The alternatives, for the record (pending facts at
depth, all directories, `python3 depth_count_a2.py --values-at-depth` with the arguments given, revision 4's
`depth_count.py` figure in brackets; revision 2's numbers where the list has no `tinted` and no `@gray`):
`--variants=primary` 534 (530); `--variants=primary,toggle` 730 (726); `--variants=primary,toggle,with_icon` 1,013
(1,009); `--variants=primary,toggle,tool,with_icon` (revision 2's list) 1,211 (1,206); with `@gray` alone 1,223
(1,218); with `tinted` alone 1,369 (1,364); the chosen list 1,381 (1,376); with `icon_only` beside it 1,672
(1,667); with `hover` and `focus_visible` kept on `ios-*` (`--ios-pad-states=keep`) 1,633 (1,628); with every state
flag (`--states=all`) 1,674 (1,669). `--revision 1` reprints revision 1's tables (1,026; 1,030 with the option).

### 1.7 The comparison with 0.7.0, run for `button`

Finding B's step (7.4), run for the slice. Revision 2 compared fifteen fields in the light scheme (`compare070.py`);
revision 3 compares every mapping of the item (catalogue:2054-2063) in both schemes (the check2's finding 4):
`python3 compare070_2.py > out/compare070_2.md` and, with C-3d and C-3e applied, `python3 compare070_2.py --with
material-3=tinted --with ios-13=@gray > out/compare070_3.md`. Beside the fifteen: `font.color` → `label.text.fill`;
`font.family`, `.size`, `.weight`, `.style` → `label.text.font.*`; `primary_text_color` → `icon.icon.fill`;
`border.color` and `border.line_width` → `body.stroke`'s edges; `border.corner_radius` → `body.corners`;
`border.padding_*` → `body.padding`; `border.shadow_enabled` → `body.shadow`. For each 0.7.0 preset, scheme and field
0.7.0 states, the script prints 0.7.0's value with its own comment and the last fact at depth for the target in the
mapped combination (default flavour, default size, no idiom); 0.7.0's source for each field is its preset comment and
its facts row (`git show v0.7.0:docs/platform-facts.md`, §2.3, lines 1172-1198). It evaluates no expression; where
a verdict rests on arithmetic, the arithmetic is given.

**The test** applied to each difference, the same for every field:
1. *Equal.* The same value, or the same source's value in the model's form: a translucent fill or a state layer
   that 0.7.0 flattened over its surface, or a padding the model measures from the box's edge with the stroke
   inside it (S2.11's box, spec:2768, ADD-A125: "`windows-11`'s padding plus its border", "Breeze's plus its frame").
   No entry.
2. *0.7.0's sourced look held by the facts under a name of the entry's own units that the depth defers* (a variant,
   a state). The name comes to depth (D13: users lose nothing of 0.7.0's model but the listed losses): `with_icon`
   (C-3), `tool` (C-3c), `@gray` (C-3d), `tinted` (C-3e).
3. *0.7.0's value had no source*: none cited, a claim with no value or evidence, or a value its own cited source
   contradicts. A correction, listed on R21's list of what users lose beside R19's invented light variants (R-3).
4. *0.7.0 took the value from a source the preset's reference rule does not choose for it*: on `material-3` R1.6
   (rationale:189-215: the preset is "the published token set … `tokens/versions/latest/`", against "the older
   `v0_192/` set"; "Where the tokens are silent, Compose Material 3 is the reference … material-web's components,
   built on the older token set, only where Compose has no such component"), so an older token set's value, or
   material-web's component's where Compose has the button; on `ios-13` R1.3 and R1.4 (rationale:166-175: a preset
   describes "the platform's current native toolkit", and "AppKit and UIKit are the reference"), so WebKit's HTML
   button, another toolkit's drawing, against UIKit's. The facts at depth state the value the rule chooses for the
   same combination, and no name holds 0.7.0's look (`python3 search070.py > out/search070.txt`: each class-3 and
   class-4 value at its key, under any `when`, is found nowhere, or only in another look: `material-3`'s 10 px vertical
   padding under `flavor:expressive` at the `small` size, Expressive's own design, and its 0.12 disabled container
   under `variant menu, primary`, the filled menu button's). (The Material facts do take material-web, its token sets
   and its component styles, `material-web@2.5.0 button/internal/_shared.scss:131-133` and `:22-23` cited at
   `docs/facts/material/button.toml:834` and `:1554`;
   revision 3's wording, "a source the facts do not take", was untrue for it.) A correction by that rule, listed
   beside the third class with both sources and the rule.
5. *0.7.0's sourced look that no name at depth can hold, or that only a name of a shared unit holds* (a flavour, a
   size, a condition beyond 1.3's point 6; a shared unit is model-wide, 3.2, so bringing its name to depth for one
   entry would bring it for every widget, role and token). A loss proposed to the maintainer (R-3), the shared unit
   named as where the look returns.

**The verdicts** (light; the dark scheme gives the same verdict on every row, its values in `out/compare070_3.md`):

| Preset | Field | 0.7.0 (its source) | At depth | Class |
|---|---|---|---|---|
| `adwaita` | `icon_text_gap_px` | `8.0` ("(Adwaita CSS): 8", no line) | `6px @ variant with_icon` (`_buttons.scss:626-628`, `docs/facts/gnome/button.toml:3860`) | 2 (`with_icon`), then 3: the value |
| `adwaita` | `font.color` and every text colour | `#2e3436`, "❓ no source for #2e3436 at libadwaita 1.10.0" (`v0.7.0:…/adwaita.toml:21`, `:36`) | `= surface.foreground`, the window's `#000006cc` (`docs/facts/gnome/roles.toml:28-30`) | 3 |
| `adwaita` | `min_width_px` | `64.0` (the row: "(Adwaita CSS): none") | `= add(16px, add(part.padding.left, part.padding.right))` | 3 |
| `adwaita` | fills at rest, hover, pressed, checked | flattened over the window (its comment) | `alpha(surface.foreground, 0.1)`, `0.15`, `0.3` | 1 (over the corrected foreground) |
| `kde-breeze` | `primary_background`, `primary_text_color` | inherited, the accent (the row: "← `defaults.accent_color`") | the default button, `{default}` | 3 (C-3b) |
| `kde-breeze` | `toolbar.font` (the reader's `toolBarFont`) | the reader's font | stated only under `variant tool` | 2 (C-3c) |
| `kde-breeze` | `min_height_px` | `32.0` (the row: "(none) — sizes to content") | `auto` (`docs/facts/kde/button.toml:230`) | 3 |
| `kde-breeze` | `min_width_px`, `padding_*` | `80`, `6` (`Button_MinWidth`, `Button_MarginWidth`) | `80 + 2·2 − 2·1.001`, `6 + 2 − 1.001` (`:200`, `:98`, `breezestyle.cpp:3555-3564`) | 1 (Breeze's frame inside the box) |
| `kde-breeze` | `border.color`, fills | `kcu_mix` at frame contrast 0.2 (the row) | the same expressions | 1 |
| `macos-*` | `hover_background` | `#e0e0e5` (the row: "**(measured)** subtle CoreUI highlight", no value or evidence; `macos-tahoe`: "no macOS 26 source … macos-monterey's value", `macos-tahoe.toml:123-125`) | no `hover` in the macOS state models (`docs/facts/macos/state-models.toml`, `axes.states`) | 3 |
| `macos-monterey` | `active_background` | `#d6d6db` / `#4e4e50` (the row: "❓ — AppKit publishes no fill for a pressed push button, and the OS probe renders it at rest only"; the preset: "soft_option: CoreUI pressed highlight -- slightly darker than hover") | `#f0f0f0ff` / `#ffffff5c`, measured (`docs/facts/macos/button.toml:147693`, `:147749`) | 3 |
| `macos-*` | `icon_text_gap_px` | `4.0` (the row: "4 **(measured)** AppKit", no evidence; "macOS 26: ❓") | `2px`, measured (`:156190`, `:26311`) | 3 |
| `macos-*` | `min_width_px` | `64.0` (the row: "(none) — sizes to content"; macOS 26 "no source") | `16px`, `34px` | 3 |
| `macos-*` | `font.color`, fills at rest and disabled, padding, `min_height`, shadow | the probe's values | the facts' (`#262626ff` against `#000000d8` over white, `#272727`; the rest equal) | 1 |
| `macos-monterey` | `primary_*`, `checked_*`, `disabled_opacity` | inherited | pending U141, U612 | on the path (1.6) |
| `ios-13` | the plain button (`background_color` and the fields with no variant) | WebKit's iOS button, `opaqueSecondaryFill` (`html.css:895`) | UIKit's system button (no configuration) draws none; `@gray` draws it (C-3d) | 2 (`@gray`) |
| `ios-13` | the look of a button with no variant | WebKit's HTML button, the grey fill | UIKit's system button, `UIButtonTypeSystem`, "standard system button" (`UIButton.h:22-23`), no fill; the facts' base (`docs/facts/ios/state-models.toml:19-20`) | **a correction of the mapping** (1.3, after C-3e): 0.7.0's look under `@gray` |
| `ios-13` | `hover_background` | `#d1d1d6`, no comment (`v0.7.0:…/ios-13.toml:138`), whose next line says "iOS has no hover state" (`:139`) | `hover` is the iPad's (1.3, point 7) | 3 |
| `ios-13` | `active_background`, `disabled_background` | `#c7c7cc`, `#e5e5ea80`, no comment | `@gray`: `#7878801f`, `#7878801f` (dark `#8b8b925a`, `#7878803d`) | 3 |
| `ios-13` | `min_width_px`, `min_height_px` | `64.0`, no comment; `44.0`, "the HIG's minimum hit target … used as the drawn min_height" | `auto`, `auto` | 3 (a hit target is no drawn height) |
| `ios-13` | `icon_text_gap_px` | `6.0`, no comment | `0px @ variant @gray, with_icon` | 3 |
| `ios-13` | `active_text_color` | `#007aff` (WebKit, `html.css:901-906`) | `@gray`: `#007affbf` (dark `#0a84ff` → `#2390ff`) | 4 (R1.3, R1.4: WebKit's HTML button, another toolkit's, against UIKit's) |
| `ios-13` | `font.color`, `disabled_text_color` | WebKit's | equal under `@gray` | 1 |
| `windows-11` | `disabled_background` | `#f9f9f900` / `#33333300`, where its own row gives `ControlFillColorDisabled` L `#4DF9F9F9`, D `#0BFFFFFF` | `= role.@control_fill_disabled`, `#f9f9f94d` / `#ffffff0b` (`docs/facts/windows/roles.toml:635-638`) | 3 (the value is not its cited source's) |
| `windows-11` | `min_height_px` | `32.0`, where its row gives "27 (derived …; no explicit MinHeight resource)" | `0px` | 3 |
| `windows-11` | `min_width_px` | `64.0` (the row: "(none) — sizes to content") | `0px` | 3 |
| `windows-11` | `hover_background`, `active_background` | flattened over `SolidBackgroundFillColorBase` (its row says so) | `ControlFillColorSecondary`, `Tertiary` | 1 |
| `windows-11` | `padding_*` | WinUI's 11, 5, 11, 6 | `add(11px, part.stroke.left.width)` … | 1 (the stroke inside the box) |
| `windows-11` | `icon_text_gap_px` | `8.0` ("WinUI3: 8", no line) | pending U517 | on the path; compared when measured |
| `material-3` | the plain button (fills, label, weight, stroke, corners, shadow) | the filled tonal button, v0_192 tokens | `@ variant tinted`: `secondary_container`, `on_secondary_container`, the state layer over it (`docs/facts/material/button.toml:2233`) | 2 (`tinted`), equal |
| `material-3` | the look of a button with no variant | Material's filled tonal button | Compose's `Button`, the filled one, "high-emphasis" (`Button.kt:77-88`; the tonal one is `FilledTonalButton`, "medium-emphasis", `:439-447`); the facts' base `md.comp.button` (`docs/facts/material/button.toml:36-44`) | **a correction of the mapping** (1.3, after C-3e): 0.7.0's look under `tinted` |
| `material-3` | `disabled_background`, `disabled_text_color` | container `on-surface` at 0.12 (`v0_192/_md-comp-filled-tonal-button.scss:42-44`) | `alpha(role.@on_surface, 0.1)` (`latest/…_md-comp-button-tonal.scss:23`, `:2667`); label at 0.38, equal | 4 (R1.6: the older `v0_192` token set) |
| `material-3` | `padding_vertical_px` | `10.0` (`(40 − 20) / 2`, material-web's `button/internal/_shared.scss:31-32`) | `8px` (Compose, `Button.kt:915`, `:553`) | 4 (R1.6: material-web's component where Compose has the button). The drawn height of one line of label is unchanged: 20 + 2 × 8 = 36 under the 40 px minimum height (`docs/facts/material/button.toml:683-684`; the label's line height `label_large`'s 20 px, `docs/facts/material/tokens.toml:4117-4118`), as 20 + 2 × 10 = 40 |
| `material-3` | `min_width_px` | `64.0` (material-web, `_shared.scss:91-94`) | `58px` (Compose, `Button.kt:1053`, `:672`) | 4 (R1.6, as above) |
| `material-3` | `checked_background`, `checked_text_color` | `#625b71`, `#ffffff`, "latest comp.button.tonal selected" (`material-3.toml:119-121`) | `toggle` drawn at depth nowhere (1.3, point 8); held only under `flavor:expressive`, a shared unit's name | **5: the proposed loss** (below) |

So, against revision 2's verdicts (the check2's finding 3, decision E): `material-3`'s plain button and `ios-13`'s
grey fill were not corrections. Both had sources, and the facts hold both looks under a deferred name; they now
come to depth (class 2). The other four of revision 2's corrections stand as class 3 with the evidence above
(`macos-*` and `ios-13` hover, `ios-13`'s sizes, `kde-breeze`'s primary). Revision 2's row "`min_width_px` and other
sizes … corrections by the facts" is now one row per value with its evidence. What the complete comparison adds
(mappings revision 2 did not read, values it did not note, or the dark scheme): `adwaita`'s text colour and gap value;
`kde-breeze`'s minimum height; `macos-monterey`'s pressed fill; the `macos-*` gap; `ios-13`'s gap and, under `@gray`,
its pressed and disabled fills; `windows-11`'s disabled fill; and the four class-4 rows (`ios-13`'s pressed label,
`material-3`'s disabled container, vertical padding and minimum width). No difference found a further name to bring
to depth. Revision 4 (the third check's should-fix 1 and 2) adds the two rows of the look of a button with no
variant, a correction of 0.7.0's mapping, and states each class-4 row with the reference rule that decides it.

**The one proposed loss: `material-3`'s checked text toggle** (decision F). 0.7.0's colours had a source: the pinned
latest token set's `_md-comp-button-tonal.scss:79` (`$selected-container-color: md-sys-color.$secondary;`) and `:95`
(`$selected-label-text-color`) (`material-web@2.5.0 tokens/versions/latest/sass/`, the check2's verdict 6). Baseline
Material has no text toggle: its tonal button's baseline tokens hold no `selected` token
(`tokens/versions/v0_192/_md-comp-filled-tonal-button.scss`, 0 such tokens, against 16 in the tonal icon button's),
and the facts draw a toggle only as an icon button (`{ variants = ["toggle"], not_variants = ["icon_only"] }`,
`docs/facts/material/state-models.toml:56`), the content unit's, or under `expressive` (`:283` has no such line, and
its `variant_products` list "the text toggles", `:289`), the flavours unit's. By the test it is class 5, not class
2: the one name that holds the look, `flavor:expressive`, is a shared unit's, which is model-wide (3.2). Neither alternative restores 0.7.0's look: (1) `icon_only` at depth (291 more pending facts, 1,672
against 1,381, 1.3) gives baseline's icon toggle a checked look, not 0.7.0's; (2) the flavours unit in 0.8.0 gives
0.7.0's colours under `set_flavor(expressive)` only, at the unpriced cost of every M3a widget stated under both
flavours of `kde-breeze` and `material-3`. **Caveat:** Compose Material 3 1.5 ships a stable `ToggleButton` and
`FilledTonalToggleButton` in the Expressive design (`androidx@e4d933c9 …/material3/api/current.txt:4684`;
`ToggleButton.kt:157`, the check2's verdict 6), so the flavours unit is where 0.7.0's look returns, under
`expressive`. "Baseline" is the facts lane's split by design version (`docs/facts/material/state-models.toml:41-43`),
not an API boundary. Recorded by R-3, for the maintainer's veto.

The check of the other entries' mappings into `button` (`toolbar`'s two, the only ones, 1.1) is done (C-3c). For the
29 further widgets, each unit's batch runs the same comparison with the same test (P-8).

---

## 2. The form of the deferral

### 2.1 The facts

- The generator "emits the units its list enables … less the values it defers, each to a named later unit … a
  deferral ending in that unit's commit, and `--check` refusing a deferral to a unit already enabled"
  (spec:3879-3883). `scripts/widget_units.toml` holds `[deferred.button] menu_placement = "placement"`; the
  generated `ButtonValues` lacks the field (`native-theme/src/widgets/gen_button.rs:6-7`, `:74-75`).
- `Variant` is generated, one enum over the enabled units' typed variants, 34 members today
  (`native-theme/src/widgets/gen_index.rs:36-107`), of which 5 are at depth (`Primary`, `Tinted`, `Toggle`, `Tool`,
  `WithIcon`; `@gray` is keyed, a `Key`). `ControlSize`, `StateFlag` and `Condition` are hand-written enums every widget shares (spec:4239,
  :4186-4189, :4120-4141).
- The query answers `NotAvailable(Variant)` for "an undeclared variant the application asked for" and
  `NotAvailable(Size)` for "a size the element does not take", "ignores a content or breakpoint variant the widget
  does not declare", and "a keyed flag the widget does not declare selects nothing" (spec:4499-4502;
  `native-theme/src/query/matching.rs:256-296`). The code documents `NotAvailable` as "The platform has no such
  widget, size, variant or part (R2.7)" (`native-theme/src/query/theme.rs:47-49`), and R15 says of it: "an
  application asking for a small button on `adwaita` learns there is none" (rationale:1193-1194).
- "A preset's declarations (availability, variants, sizes, container sizes) are facts too" (plan:38-39), and the
  gate checks each preset's declarations against the state models (spec:7255-7259). "Which flags form an exclusive
  group comes from the platform's state model … the gate checks that order against the state-model data"
  (spec:5577-5581).
- A fact's units today: "the unit of its key, … the unit of every widget its `when`'s positive lists name … and …
  the unit of the widget a `drawn_as` value names, and the gate skips it while any of those units is not enabled.
  Its negative lists … never add a unit" (spec:7428-7433). The gate, `check_facts_trace.py` and `generate_preset.py`
  read the units from one file by one rule (`native-theme/tests/facts_gate/units.rs:1-4`, `run.rs:414-427`;
  `scripts/generate_preset.py:107-115`).
- Today a declared variant no fact names "draws as the rules that apply give it: with none, as the base"
  (spec:7441-7442). `python3 names.py` lists 19 variants of `button` that a state model declares and no fact names
  in a positive list (`maximize`, `minimize`, `restore`, `play`, `volume`, `reveal`, `@raised`, `@arrow_only`,
  `@float`, `@key_sequence`, `@ka_download`, `@ka_floating`, and Windows' seven nested ones): their looks wait for
  their hosts' units. Under the design below the typed ones are not members of `Variant` and the keyed ones are
  `later`.
- A layer stating an entry of a keyed map its preset does not declare is refused (`ProblemKind::Undeclared`,
  `native-theme/src/validate/checks.rs:1410-1416`).
- A connector keeps its 0.7.0 mapping for a widget of a later unit as a **stand-in entry** of its contract table,
  "the widget, … the resolved fields it reads, and the unit that replaces it", which the contract test accepts and
  fails once the unit is enabled (spec:7781-7789).

### 2.2 The options for typed names

| | (a) cannot be asked for | (b) nothing says the platform lacks it | (c) never drawn silently as the base | (d) the preset loads | (e) facts stay, the gate skips exactly them | Cost in what is built |
|---|---|---|---|---|---|---|
| **1. Typed members leave the enums until their unit** | yes, at compile time, for a typed variant; not expressible for a size (one shared enum), a keyed variant or a flavour (a `Key`) | with 4 | yes for a typed variant | with 4 | with 4 | `generate_widgets.py` reads the Depth lines; `gen_index.rs` regenerated (34 members to 5); tests and gate fixtures that name `flat`, `pill`, `icon_only` in TOML rewritten (the largest users: `tests/facts_gate.rs`, `src/query/tests.rs`, `src/validate/*`, section 8 point 7); each unit that adds members is a breaking release (D14) |
| **2. Members stay, behind the units' enabled flag** | no: every member can be named, and is refused at run time | only with a new answer | yes, with that answer | with 4 | with 4 | the same as 1 without the generator and test changes; later units of depth add no enum member, so they could be patch releases |
| **3. A per-fact unit tag** | no | no | no | no | yes | about 65,000 facts edited (64,892 off the path, 1.6), and a checker that the tag agrees with the `when`, which is the predicate of 4 done twice |
| **4. The unit derived from the facts' `when`s** (with 1 or 2) | – | – | – | yes | yes: one rule, read by the gate, the generator and the checkers (2.5) | the predicate in the four readers of the units file |

- Option 3 is rejected: it meets (e) alone, and a second statement of what a fact's `when` already says can
  disagree with it.
- Option 2 is the smaller change and has one real advantage, patch releases. It fails (a) as outcome 2 words it
  ("impossible to ask for until their unit") and as the brief states it ("as the 57 later widgets cannot"): a later
  widget has no member of `WidgetKind`. It also drops, for `Variant`, the meaning R21 gives a generated enum: its
  members are what the model holds (rationale:1362-1364; spec:7734-7735).
- Option 1 is S6.1's own mechanism applied to a variant ("deferred as S6.1 defers placement", the outcome's words).
  The hand-written code of `native-theme/src` names no removed member outside tests (the check).

**Chosen: 1 with 4, and the forms of 2.3 for the names 1 cannot reach.** What would change it: the maintainer
preferring patch releases for the units of depth over a name that cannot be written; then 2, with the same `Later`
answer for typed names.

### 2.3 The form, by kind of name

**A typed variant beyond depth** (asked; `Variant`). Its unit of depth holds it; until then the generator leaves it
out of `Variant`, of `WidgetInfo.variants` and `content_variants`, and of every declaration of every preset (it has
no spelling), as it leaves a deferred value out of the values struct, and names it in a generated comment with its
unit. (a) holds by the type; a typed content variant (`icon_only`) is the same: the connector cannot set it.

**A keyed variant beyond depth, and a typed variant at depth drawn nowhere at depth on a preset** (asked; a `Key`, or
1.3's point 8). The preset keeps its declarations whole (the state model's names, as today) and lists these
variants in the widget's `later` (the first as `macos-tahoe` would hold it, shortened; the second `material-3`'s;
the third `ios-13`'s):

```toml
[declare.widget.button]
variants = ["primary", "tinted", "toggle", "tool", "with_icon", "@flexible_push", "@badge"]   # whole, as the state model
later = { variants = ["@flexible_push", "@badge"] }   # the platform has them; a later unit states them

[declare.widget.button]                                 # material-3
variants = ["primary", "tinted", "toggle", "with_icon", "@elevated"]   # shortened
later = { variants = ["toggle", "@elevated"] }          # toggle: drawn only with icon_only or under expressive

[declare.widget.button]                                 # ios-13
variants = ["@gray", "tinted", "primary", "toggle", "with_icon", "tool", "@corner_small"]   # shortened
later = { variants = ["tool", "@corner_small"] }        # tool: drawn only in the ink toolbar; @gray is at depth (C-3d)
```

(`tinted` is declared on `material-3` and `macos-tahoe` because it is at depth, C-3e; revision 2's example declared it
while deferring it, the check2's minor 11, which decision E makes moot.)

- `generate_preset.py` writes `later` from the state model and the units file (on `ios-*`: `tool` with the keyed
  variants beyond depth; on `material-3`: `toggle` with them). The gate's declarations check stays
  an equality on the declarations (a declaration stays a fact, and (b) holds in the preset itself), and checks
  `later.variants` = the state model's keyed variants not at depth, and its typed variants at depth that it draws in
  no combination at depth.
- The query answers `NotAvailable(Unavailable::Later)` for an element holding a `later` variant, a content variant
  included (refused, not ignored). `Unavailable` gains that one member; `NotAvailable`'s other members keep R15's
  meaning, "the platform has none". So (a) holds as a refusal, (b) in its wording, and (c) because the look is never
  returned: a connector that then draws the base does so by its own recorded choice (below).
- S12 refuses a rule, a reference or a patch that names a `later` variant, in a positive or a negative list, with a
  problem of its own, `ProblemKind::Later` (the name and its declaration; its message says that the theme declares
  the name and a later unit states it), not `ProblemKind::Undeclared`, whose message ("is not declared in …") would
  contradict the declaration (the third check's minor 10). `ProblemKind` is one of the diagnostic enums S16 makes
  `#[non_exhaustive]` (spec:60-64), so the member adds no breaking change.

**A control size or a flavour of a unit not enabled** (asked). The declarations stay whole (`sizes`,
`sizes_by_variant`, `sizes_by_context`, `default_size`; `flavors` and each flavour's section), true as they are. The
generator writes the enabled flags of the two shared units into `gen_index.rs`; while the sizes unit is not
enabled the query answers `NotAvailable(Later)` for a size the element takes (S10: by its variant's and its
context's entries) other than its default in force, and `NotAvailable(Size)` for one it does not take, as today;
while the flavours unit is not enabled `Theme::set_flavor` refuses every declared flavour but the default with a
problem of its own (the error enums are `#[non_exhaustive]`, spec:7737-7740), and `flavors()` lists the default
alone. This settles the check's finding 11's first case: on `ios-*` a plain button asking `small` is answered
`NotAvailable(Size)` (UIKit's button with no configuration style has one size, catalogue:1951-1952), a
`primary` one `Later` (its `sizes_by_variant` entry lists `small`). S12 refuses a rule naming such a size or flavour,
as above.

Rejected for sizes and flavours: per-preset `later.sizes` and `later.flavors` lists, revision 1's form (the sizes
are per variant and per context, so a list per widget answers `Later` for a size the element's variant does not take,
finding 11; the trimmed declarations need a second structure mirroring `sizes_by_variant`); the declarations
trimmed and `NotAvailable(Size)` kept (a trimmed `sizes = ["regular"]` on `macos-tahoe` would be a false fact, the
state model stating five); removing `size` from `Element` until a sizes unit (the size is wired through S7.3, S8,
S10, the declarations and the gate; a breaking change twice).

**The state model in force** (the check2's serious finding 1). Where the generator, its round trip and the gate read a
state model for what an element can hold (the covering bases, `generate_preset.py`'s `covering_bases()`,
`:1212-1317`; an element's size, `element_size()`, `:2228-2286`; the combinations the gate draws), they read it
restricted to the names in force: while the sizes unit is not enabled, an element takes its default size in force
alone (S10: its context entry's default, else its variant entry's, else the widget's); while the flavours unit is not
enabled, the default flavour's state model alone is in force; and no element holds a variant not at depth (a typed
one has no member, a `later` one is refused), so a `sizes_by_variant` entry of such a variant is no element's. The
declarations of sizes, variants and flavours stay whole: this is the reading of them for the elements that can exist, as the query answers `Later`
for every other size, flavour and variant. Then `sizes = ["regular"]` (the default size's fact, kept) covers its key
as today's base does, and the generator gives every field at depth a base. In the generator this is one place: the
`models` dict that `declarations()` returns (`generate_preset.py:1450-1499`, called in `analyse()` at `:845`), which
`analyse()` hands to `covering_bases()` (`:847`) and `pending_table()` (`:859`) and which `round_trip()` reads
through `Combinations(draft.models)` (`:2483`). Restricted there (the default flavour's models only; the button's
sizes the default in force; no `sizes_by_variant` entry of a variant not at depth), all three read the state model in
force: the third check's `staging-check3/gen_inforce.py` does so on all eight presets with the generator's own code
(7.10). (`gen_bases.py` restricts `covering_bases()` alone, its round trip and pending table reading the unrestricted
`draft.models`; the two runs give the same results, 7.10.)

Why the state model and not the sizes alone. The generator builds a covering base from "one fact under each of its
sizes alone", the sizes being "every size an element takes, the widget's and its `sizes_by_variant` entries'"
(`covering_bases()`'s docstring, `generate_preset.py:1213-1235`), and refuses one where "a variant's entry takes no
size and no stated fact under that variant alone covers its elements". Run with the generator's own code on the
trimmed facts (`python3 gen_bases.py`, 7.10), revision 2's form leaves button fields at depth with no base on four
presets, and the check2's correction (the sizes in force alone, with the default flavour) still leaves them on
`macos-monterey` and `macos-tahoe` (`out/gen_sizes_only/`): their state models hold `{ variant = "route", sizes = []
}` (`docs/facts/macos/state-models.toml`), `route` is beyond depth (the identities), and the generator reports "its
variant route takes no size and no fact under it alone gives the value of its elements". Reading the variants in force
too leaves no field without a base (7.10). Options weighed: (s1) the check2's, the sizes alone (fails on `route`);
(s2) the state model in force, chosen; (s3) the facts of the other sizes kept in the bundled preset (they would
mention names the query refuses, state looks no element can have, and bring the sizes' pending facts back as
unlisted paths); (s4) bases written by hand where the covering base fails (the default size's fact, which the
generator already finds, written twice).

**The declarations that name given names** (the check2's serious finding 2). S12 refuses "a keyed selector name (a
variant, flag, condition, container size or recency set) [that] is declared but no rule selects on it", exempting
the default flavour, the containers' default sizes, `requires_variants`, what a connector sets, application settings
and a platform preset's variants (spec:5996-6006); the code pushes a problem for each declared keyed flag no rule
names, a flavour section's included, each declared flavour but the first no rule selects on, each system setting of
`[declare] conditions` not for applications, and each recency set (`checks::unused`, `validate/checks.rs:3013-3110`,
called by `check_preset`, `mod.rs:262`). Revision 2's form leaves such names declared with no rule at depth:
`kde-breeze`'s flavour `kirigami`, its section's flag `@keyboard_focus_change` and its system setting `@single_click`
(the behaviour unit's, 3.1); `material-3`'s flavour `expressive`; `ios-13`'s and `ios-26`'s flag `@loading`
(`python3 holes3.py`, part B; the check2 found all but `@single_click`). So:
- **Keyed flags and system settings** are declared only where a rule at depth names them: the generator writes
  `flags` and `[declare] conditions` from the trimmed facts, as it trims the exclusive groups and recency sets. A keyed
  flag left out is undeclared, so the query ignores it ("a keyed flag the widget does not declare selects nothing",
  spec:4501-4502) and S12 refuses a layer's rule that names it, an application's included, until its unit; a typed
  state flag and a condition are no declarations and still select in an application's or a reader's layer (below).
  This is the declaration's existing meaning: "a declared flag is one some rule selects on (S12 refuses a keyed flag
  none does), so it changes what is drawn" (spec:7338-7339), so a `flags` list trimmed to the flags a rule at depth
  names states what the spec says it states, unlike a trimmed `sizes`, which is the platform's list (above).
  Rejected for that reason: an S12 exemption for a keyed flag of a unit not enabled (a `later = { flags = […] }` list
  beside `later.variants` would carry it with no run-time table, but it would keep declared a flag that no rule
  selects on, against the declaration's meaning; revision 3 gave the run-time table's cost as the reason, which a
  `later.flags` list avoids, the third check's minor 12).
- **A flavour of a unit not enabled** stays declared, as `set_flavor` must answer `Later` for it (S-5), and S12's
  check exempts it, with its `[declare.flavor.<f>]` sections. The exemption opens no hole: such a flavour cannot be
  asked for (`set_flavor` refuses it, S-5), no rule of the bundled preset names it (`holes3.py`, part A: 0 flavour
  mentions on every preset), and S12 refuses any layer's rule naming it (S-4). The flag it reads is the one
  `gen_index.rs` already holds for `set_flavor` (above). Rejected: the flavour left out of `flavors` (then
  `set_flavor` answers "undeclared", which says the platform has none).
- **A declared keyed entry** (a colour role, text style, timing, duration, icon context, elevation entry, parameter
  key) that no base at depth states is left out of its declaration (2.4), since S12 refuses "a keyed name no base
  states" too (`checks.rs:3115-3147`; `Uses::map_key` records a key only while it walks the bases, `:2803-2817`). A
  base at depth states it when a stated fact at depth whose trimmed `when` is a base's (empty, or the scheme and the
  preset alone) gives it, or a pending fact at depth lists its path, which the synthetic route's bases state ("a
  synthetic layer's base states it", spec:5997, ADD-A133; `check_preset` hands `unused` the synthetic layers,
  `validate/mod.rs:262-267`): `ios-*`'s `params.opacity.disabled`, whose one fact at depth is pending (U486), is kept
  so. `holes3.py`, part C, counts them per preset by that rule (revision 4; revision 3 kept an entry any fact at
  depth states, and the two rules give the same entries on all eight presets, `out/holes4.txt`): every preset's `timings` and
  `durations` whole (the motion tokens are staged), most `text_styles` and `color_roles` (all 38 keyed roles on each
  `ios-*` preset), and parameter keys; it does not compute `lcd_filters`, `transition_kinds`, `symbolic_colors`,
  `symbol_scales`, `icon_state_effects`, `rendering_engines`, `spec_versions` or `accent_ramp`, which the staging
  task's load of each regenerated preset checks. A keyed map left with no entry is stated empty (`keyed = {}`,
  `params = {}`), as `accent_ramp = {}` is today (`native-theme/src/presets/ios-13.toml:1199`), since a field the
  bases leave unstated is missing (`validate/checks.rs:2139-2165`) and the generator reports the map so
  (`gen_bases.py`, `out/gen/<preset>.txt`: `colors.<scheme>.keyed` on `adwaita` and `ios-*`,
  `tokens.icons.sizes.keyed` on every preset but `kde-breeze`, `tokens.params` on `macos-*`).

With these, `python3 holes3.py`, part B, finds 0 unused recency sets on all eight presets. That count tests the
recency sets alone: for keyed flags, system settings and flavours its `rev3` column is 0 by the form's definition (a
flag or a setting is declared from the names the trimmed facts use, a flavour of a unit not enabled is exempt), and
the script appends nothing for them. Their real test is S12's own check on the generator's output, which no script of
this note can give without restating S12 in Python, a second statement that could disagree with it: the staging
task's load of each regenerated preset in Rust (`check_preset`, which calls `checks::unused`, 2.7 and P-6b), with
revision 2's form as its positive control (the declarations whole must fail with the 6 problems part B's `rev2`
column lists). What part B does show of them: revision 2's form has 6 such problems, each of a kind the form removes.

**A state flag, a keyed flag, a host context, a position, a direction, a condition** (given). The model changes
nothing: these are inputs the toolkit, the connector, the element tree and the reader supply, and 0.7.0 already
reports conditions with no theme value behind them. Deferral means: **the generator leaves out of the bundled
preset every mention of a name not at depth**: a fact naming it in a positive list (it holds in no combination at
depth, 1.1), a negative entry naming it (`not_states`, `not_conditions`, `not_context`, `not_variants`,
`not_sizes`), and its place in a declared exclusive group or recency set (each trimmed to its flags at depth, a true
statement of their precedence; a group of one goes). The query and S12 are unchanged. So the bundled stack names no
such atom: S12 judges no assignment of it, and an element under it resolves exactly as without it, the look at
depth. A layer that names the atom itself, an application's or a reader's, still selects on it where it is no
declared name (a typed state flag, a condition, a context; a keyed flag or a system setting is declared only with its
unit, below): the macOS reader's
roles under `contrast_increased` (`macos.rs:359-372`) apply when Increase Contrast is on, as 0.7.0's reader gave
them. (c) is met by a record, as R23 records a toolkit's limit: each connector's contract table lists, per enabled
entry, every state it reports, keyed flag it sets, context it builds and condition it supplies that a later unit
holds for that entry, as a **deferred entry** (the entry, the name, the unit that states it), and the contract test
fails a list that is not complete against the units file or that keeps an entry whose unit is enabled. The same
record holds the connector's own stand-ins: where its toolkit has a variant, a size or a content form that only a
deferred name describes (a ghost button, a small button, an icon-only button), it keeps its 0.7.0 mapping from the
values at depth, recorded with the unit that replaces it (S21.3's stand-in entry, extended from a widget to a name).

The options for given names, weighed:

| | Holes (finding 1) | An element under the name | A reader's or an application's rule under the name | Cost |
|---|---|---|---|---|
| **(g1) Revision 1: negative entries naming a state or a condition kept** | 10,408 (key, combination, name) holes on `ios-13`, `ios-26` and `macos-monterey` (`python3 holes.py --keep-negatives`, revision 3's variants; 6,056 with revision 2's): `from_preset` refuses all three | some keys fall to a general fact, others keep the specific one (`kde-breeze`'s default button in an inactive window: 128 key-and-combination cases, the check's `cfallback.py`) | selects | none |
| **(g2) The check's: the generator drops them, and the query drops a flag or condition of a unit not enabled before the groups' reduction** | none | as without it | **never applies**: the macOS reader's Increase Contrast colours, 0.7.0's depth (1.1), are silenced | a generated run-time table of the names not at depth per widget and preset, read by the query |
| **(g3) Chosen: the generator drops every mention; the query unchanged** | none (`python3 holes.py`: 0 on all eight presets; with the state model in force and the declarations below, no field without a base and no unused recency set or keyed entry, 7.10) | as without it | selects | the trimming in `generate_preset.py`; the declarations check reads trimmed groups |

(g3) gives (g2)'s result for the bundled presets with no run-time list, and keeps what the readers deliver. Also
rejected, as in revision 1: removing the fields of deferred conditions from `ReaderState` and `WindowConditions`
(they are built, the readers set them, an application layer may select on them); a query answer for "resolved under
a deferred condition" (the query cannot refuse a button because the window is inactive).

**Negative entries in numbers.** Summed over the eight presets, 1,811 button facts at depth carry a negative entry
naming a name not at depth (`python3 defeat_checks.py > out/defeat3.txt`, check 5, the distinct facts; 1,520 with
revision 2's variants); the form drops each such entry. (Revision 1's "1,407" was a sum over two kinds; its facts
were 925, the check's minor 15 (a).) **A one-of list** (`sizes`) is trimmed to its names at depth, not dropped: a fact
whose list names the default size and another holds at depth (none in the slice, the check2's `sizes_lists.py`: 0),
and S-4's refusal applies to the trimmed list (the check2's minor 13).

### 2.4 The form, for paths

- **A widget value** leaves the generated values struct until its unit: S6.1's deferral as it is
  (`[deferred.button] help_tooltip_on_click = "styles"`).
- **A part's field** leaves the part's `fields(…)` list until its unit: the same table gains part fields
  (`"body.transitions" = "motion"`); the part then does not list the field, and a layer stating it is refused as
  one stating any field the part does not list. The `PartStyle` type is unchanged.
- **A token field, or a leaf of a shared block** (`IconStyle.animation`, `FocusIndicator.transition`), is staged
  with the existing table (`[staged.slice] "token.behavior.touch" = "behaviour"`, as `scripts/widget_units.toml`
  stages the scroll behaviour, `[staged.slice]`) and leaves its Rust struct until the unit that reads it, as
  `Behavior.scroll` did (spec:3883-3885). Every model struct is `#[non_exhaustive]` (rationale:1360), so the
  field's return is no breaking change. No other form is truthful: a `Value` field left in the type must be stated,
  and `absent` "means a platform without the value" (catalogue:1712-1713).
- **A keyed entry** (a keyed role, text style, elevation entry, icon context, parameter key) is declared by a preset
  with the first unit whose value reads it, or now if a reader states it (1.5, clause 5), as the plan already rules
  for a condition key (plan:746-748). Until then it is undeclared, and nothing lists it: no query asks for these
  names, and S12's "undeclared" refers to the theme and states nothing about the platform (the check's finding 14).
  Its declaration lists only such entries, and a map left with none is stated empty (2.3, *The declarations that
  name given names*). Rejected: revision 1's `[declare] later` lists for them (a generator output and a gate check
  with no state model to check them against).

### 2.5 A fact's unit, the units file and the gate

**A fact's unit.** The options:

- **(u1) Revision 1's S-6: the unit of depth of every name its `when`'s positive lists name.** Fails (e): 191 facts
  (98 pending) hold in no drawn combination at depth without naming a deferred name positively (`python3
  cmismatch4.py` in `staging-check/`, recomputed at revision 1's names): `button.menu_indicator.*` with no `when` (its part drawn only under
  `menu`), the iOS system type's `icon` part and `subtitle.text.*`, `not_sizes = ["regular"]` facts, and
  `material-3`'s toggle facts. By (u1) the gate would check them and `check_ledger.py --final` count the 98 against
  0.8.0.
- **(u2) (u1) with a clause per shape:** a part's presence facts add their names' units; a negative size list that
  excludes the default adds the sizes unit; a variant drawn nowhere at depth adds the units that draw it. Stays
  syntactic and names one unit per fact, but each clause reads the state model or the presence facts, so it is the
  predicate below written in pieces, and a fourth shape would need a fourth clause.
- **(u3) Chosen: the predicate of 1.3.** A fact is of the enabled units when its key is held and its `when` holds in
  at least one combination of the names at depth that its state model draws, its part drawn there; every other fact
  is skipped as a later unit's. It is the combination set the gate already draws, and the rule `generate_preset.py`
  and the gate already apply to a listed path (spec:7047-7052); the counts of 1.6 use it. A part whose presence fact
  is pending counts as drawn (a pending fact states nothing, so it neither draws nor hides the part; both counts read
  it so, `depth_count.py`'s `analyse()`, the check2's minor 15). A skipped fact's later unit, which `check_ledger.py`
  and `check_facts_trace.py --unit` report, is the first unit **in the units file's order** whose names, added to the
  enabled ones, make the predicate hold (the order of 3.4 as the file lists it; an order of listing, not a schedule);
  for a fact that names names of two units it waits for both and is reported under the first. **A stray fact** (a
  `when` that the names of all units together do not make hold, such as a misspelt name): from M4 `check_facts.py`
  fails it, computed from the state models and the units file, since under (u3) no fact of the enabled units is ever
  "skipped" and plan:152-153's check alone would never fire (the check2's should-fix 7; P-2). Until M4 it is reported
  by unit "none". A fact still drawn nowhere when the last unit is enabled is ADD-A141's finding, as today.

**The units file.** `scripts/widget_units.toml` gains every unit of depth, with the names it holds per entry (and,
where it is limited to presets, the presets: the iPad unit holds `hover` on `ios-*` alone), an enabled flag, and the
parking units with the paths they hold; it does not restate the entries' Depth lines, which the catalogue states
once and `spec_catalogue.py` reads, as it reads the typed variants (the check's finding 13). What every reader needs
is two functions: the names at depth of (entry, preset), and the predicate.

**Who checks it.**
- `generate_widgets.py --check`: each Depth line's catalogue part against the *Today's fields* selectors that name a
  target in the entry (the reading `todays_selectors.py` makes, in the generator's own parser), and no deferral to an
  enabled unit.
- `check_facts.py`: every name of every state model, every condition any fact names (positive or negative), and
  every token path a fact names lies in exactly one place: an entry's Depth line, one unit of depth, or one parking
  unit; a name in none is a finding, as a key in no unit is at M4 (plan:152-153). On the slice this holds by the
  rules of section 3 (`python3 later_units.py`: 67,404 facts off the path, a fact of two presets twice, none
  assigned to no unit).
- The gate (`units.rs`, `run.rs`): the combinations at depth, the skip by the predicate, and the declarations check
  of 2.3.
- `generate_preset.py --check`: the bundled preset is the facts at depth with 2.3's trimming.
- `check_facts_trace.py`: a trace line's unit by its selector's names, so `--unit <unit>` asks an M3a unit for its
  rows at depth and no more.
- `check_ledger.py` and `verify_facts.py units` take a fact's units by the same predicate, so a register row all of
  whose pending facts are a later unit's is that unit's row without an edit (spec:6982-6983).

**(d).** A listed path stays listed only from a combination at depth, and a staged path is not a field of the model.
Revision 2 concluded from this alone that `Theme::from_preset` refuses a preset only for its paths at depth; the
check2 found two more ways it refuses (fields with no base, declarations no rule uses), both in code revision 2 did
not run. With 2.3's *state model in force* and its *declarations that name given names*, the generator's own code run
on the trimmed facts gives every field at depth a base on all eight presets with no round-trip mismatch
(`python3 gen_bases.py`; with the state model in force given to the whole generator, the third check's
`gen_inforce.py`), no fact at depth mentions a name not at depth, and no recency set or keyed entry is left declared
that S12's unused check refuses (`python3 holes3.py`; the keyed flags, settings and flavours by the form's
definition, 2.3); 7.10 gives the figures. Two kinds of hand-written block then lose their
place, which the generator reports and refuses to carry ("move or remove their lines by hand", `carried()`), so the
staging task edits them: on `ios-*` the rule block of `role.global.accent`'s read fact (`= input.accent`, `{
not_conditions = ["tint_dimmed", "window_inactive"] }`, `ios-13.toml:11244-11247`), which the trimming empties to a
base, so the block moves to the bases of `colors.<scheme>.global`; on `macos-*` the bases blocks of
`colors.<scheme>.keyed`, whose one read fact (`selected_control = "= input.selected_control"`,
`macos-monterey.toml:1263-1266`) is a keyed role the token rule stages, so they go. The full check stays loading each
regenerated preset in Rust (`check_preset`), in the staging task's gate (2.7).

### 2.6 The built system readers

The readers are built and linked since M1b. The staging task changes no reader's reads: every target a reader states
is at depth by 1.5's clause 5, or is a typed role, an input or a slot at depth anyway, and every condition a reader
reports is a given name. It edits one comment: the macOS reader's module comment says the roles that reference an
input "(`role.global.accent`, `role.selection.background`, `role.@selected_control`, …) are left to the preset,
which states them" (`native-theme/src/macos.rs:15-18`), and after 2.5 *(d)* the `macos-*` presets no longer state
`role.@selected_control` (staged with the keyed roles), so the comment drops it from its list (the third check's
minor 13). Read from the code:

| Reader | What its layer states | At depth by |
|---|---|---|
| KDE (`native-theme/src/kde/mod.rs`) | the slots `body`, `monospace`, `caption` (`:261-266`); the keyed text styles `toolbar`, `menu` (`:298-311`); `token.icons.theme` and the sizes `toolbar`, `small`, `large`, `dialog`, `panel` and the keyed `main_toolbar` (`:312-319`, `:475-514`, the five typed sizes at `:507-512`); `token.motion.speed_factor` (`:321-325`); `token.kde` and `token.params.color.{contrast,frame_contrast}` (`:453-472`); the typed roles and four `role.@wm_*` (`:403-428`); `animations_off`, `reduced_motion` (`:235-236`) | the table (slots, icon sizes, theme); clause 5 (keyed styles, `main_toolbar`, `@wm_*`, `params.color`, speed factor); the core (`token.kde`) |
| GNOME (`gnome/mod.rs`) | the slots `body`, `monospace` (`:193-198`); `input.accent`; `token.icons.theme` (`:237-240`); `contrast_high`, `reduced_motion`, `animations_off`, `overlay_scrollbars` (`:216-219`) | the table; inputs; conditions given |
| macOS (`macos.rs`) | inputs (`:180-200`); typed roles and five keyed roles (`:208-261`), as bases or, under Increase Contrast, in a rule under `contrast_increased` (`:359-372`); `reduced_motion` (`:322`) | typed roles; clause 5 (keyed roles); the rule selects on a given condition (2.3) |
| Windows (`windows.rs`) | inputs (`:362-374`); the eight `role.@system_*` in a rule under `contrast_high` (`:377-392`); `animations_off`, `reduced_motion` (`:333-335`) | clause 5, and the closure from the `contrast_high` role facts at depth |

Revision 1's rule would have staged `kde-breeze`'s `toolbar`, `menu` and `main_toolbar` (46 facts, none pending)
and `windows-11`'s system roles, and `checks.rs:1410-1416` would then refuse the KDE layer, and the Windows layer
under high contrast (finding 3). Outcome 4's comparison (`data/readers_070.md`) read against the depth rule: every
0.7.0 read it lists as read or replaced lands on a target at depth (a slot, an input, a typed or reader-stated role,
`contrast_high`, `animations_off`, `reduced_motion`, `overlay_scrollbars`, the icon theme and sizes); its "not read"
values target `window`, `input`, `list` and `scrollbar` fields that 0.7.0's mappings bring to depth with their
units, the slice's icon sizes pending on U516, or nothing (two KDE reads the specification drops), as the comparison
says. Its result therefore holds, now that high contrast is at depth.

### 2.7 When

One time: the staging task runs after the spec batch that states the rule and before M2's task 0. Task 0 brings the
four small tables to empty; with the staging applied first they are those of 1.6 (`adwaita` 0, `kde-breeze` 4,
`material-3` 9, `windows-11` 55 paths, against 4, 16, 48 and 105 today, plan:998), and the connectors' design (task
1) is written against the model 0.8.0 holds. One code and tooling task, which also edits the presets' hand-written
parts the form changes (the keyed declarations no base states, 2.3; the two blocks of 2.5 *(d)*); its gate is the
generators' `--check`, the checkers' self-tests, each regenerated preset loading in Rust, and the facts gate green
with the eight presets regenerated. (Revision 1 named three times:
2.6, P-6 and P-9, finding 12.) The task's text for the plan, with its change list by file, its inputs, its gate and
its done-criterion, is P-6b (6.4).

---

## 3. The later units and their order

### 3.1 The units

Every fact the rule leaves off the path is assigned to one unit (`python3 later_units.py > out/later_units3.md`;
first match in the order the script's docstring gives, a fact of two presets twice; a fact that names names of two
units waits for both, and is listed here under the first). Revision 3's script applies depth_count's promotion of a
typed token field at depth on one preset to every preset, which revision 2's left out (the check2's minor 10: 28
facts, 4 pending, counted off the path that are on it), and the variants of 1.3 (`tinted` and `@gray` at depth):

| Unit | Holds | Facts off the path (pending) |
|---|---|---:|
| **content** (buttons and toggles) | `icon_only`; the keyed content variants (`@custom_child`, `@arrow_only`, `@with_arrow`, `@image_text_box`, `@tab_overflow`); the iOS `subtitle` part's facts | 1,908 (943) |
| **styles** (buttons and toggles) | `flat`, `destructive`, `warning`, `pill`, `circular`, `help`; each platform's keyed styles (AppKit's bezel styles, UIKit's corner styles, Material's container and FAB styles, libadwaita's `@raised`, `@osd`, `@opaque`, `@card`, KDE's `@highlighted`, `@command_link`, WinUI's `@repeat` with `repeat_delay`, `repeat_interval`); `label_position`, `help_tooltip_on_click` | 22,266 (12,961) |
| **menus** (buttons and toggles) | `menu`, `@menu_on_hold`, `@indicator_popup`, the arrow variants; `menu_indicator_gap`, `menu_indicator_anchor`, `menu_trigger`; the `menu_indicator` part's facts | 936 (567) |
| **states** (buttons and toggles) | `open`, `drop_target`, `indeterminate`, `selected`, `dragged`, `route_active`, `@loading`, `@keyboard_focus_change` | 166 (98) |
| **identities**, with each host | the nested-control variants (`close`, `back`, `previous`, `next`, `overflow`, `add`, `remove`, `done`, `clear_field`, `reveal`, `pane_toggle`, the media and caption ones, `route`, `@info`, `@contact_add`, `@share`, `@paste_control`, Windows' seven) and the host contexts, positions and directions they are drawn in (the ink toolbar's `tool` on `ios-*`, 1.3, point 8); `back_*`, `display_mode`; the iOS system type's `icon` part | 11,598 (10,780) |
| **sizes** | every control size but the default in force | 3,905 (1,176) |
| **flavours** | `flavor:kirigami`, `flavor:expressive` | 88 (2), and the 30 of `material-3`'s toggle facts, drawn with `icon_only` or under `expressive` (listed by the script as "content or flavours"; in the units file's order, 3.4, flavours come first) |
| **increased contrast** | `contrast_increased` | 1,406 (735) |
| **window activity** | `window_inactive` beyond 1.3's point 6, `not_main`, `app_inactive` | 2,307 (1,089) |
| **the iPad** | `idiom:pad`; on `ios-*` `hover`, `focus_visible` and the pointer variants | 16,693 (16,557) |
| **accessibility text** | `bold_text`, `button_shapes`, `tint_dimmed` (and `material-3`'s keyed styles under `bold_text`) | 308 (300) |
| **interface level** | `elevated` (`ios-*` roles) | 38 (26) |
| **glass** | `glass`, `clear` | counted under styles (first match) |
| **right-to-left** (a unit of depth that also takes a staged path) | `direction = "rtl"` outside a host; `token.icons.mirroring` | 8 (6) |
| **motion** (a unit of depth that also takes staged paths) | section 5; transitions under `animations_off` and `reduced_motion` | 357 (110) |
| **behaviour** (parking) | `token.behavior` but `focus_visible_when` and the mnemonic tokens (U473's key repeat, U503, U504's `single_click_activates`); `input:touch`, `system_setting:single_click` | 173 (80) |
| **text rendering** (parking) | `token.text.rendering`, `.fallbacks`; `unit_scale` | 88 (63) |
| **text styles** (parking) | the nine staged slots and the keyed text styles no value at depth reads | 4,176 (332) |
| **icon contexts** (parking) | the keyed icon contexts no value reads | 193 (93) |
| **elevation**, **parameters**, **keyed roles** (parking) | the elevation entries, parameter keys and keyed roles no value reads | 26 (9), 32 (0), 435 (2) |
| **pointer** (parking) | S4.7's `pointer` (no 0.7.0 field: `git grep -n -i "cursor\|pointer" v0.7.0 -- native-theme/src/model` finds a tooltip delay's comment and a code comment only) | no fact yet |
| **fields** (an M3a unit) | `token.caret` (`input`'s caret reads it) | 75 (19) |
| **placement**, other widgets' units | as planned | 2 (0), 190 (70) |

All: 67,404 facts off the path, 46,018 pending (a fact of two presets twice), none assigned to no unit. A parking unit
is where a path waits; a path an M3a unit's value reads leaves it in that unit's commit (1.5). `high contrast` is no
longer a later unit: it is at depth (1.3, point 6).

### 3.2 The same split for M3a's 29 further widgets

No list is written now. Each M3a unit's spec batch writes its entries' **Depth** lines by section 1's rule (the
catalogue part is mechanical: `todays_selectors.py`'s reading, 1.1), runs 1.7's comparison, and assigns every other
name of its state models and every condition its facts name to a unit by its kind: a condition to its condition's
unit; a size to **sizes**; a flavour to **flavours**; a host context, a position or a direction to the identities of
its host; its own variants and states to its unit's **content**, **styles** and **states**. Two rules keep the
shared units honest:

- **A shared unit is model-wide.** A condition's unit, the iPad, glass, sizes, flavours and motion cover every
  widget, role and token the model holds when they are enabled, and a widget enabled later states them in its own
  unit's commit, as "a preset added later states every field the model then holds" (plan:145-146). Rejected: a
  condition enabled widget by widget, which draws a high-contrast button beside a standard check box, a mixed look
  by the theme's own data.
- **The facts lanes write an M3a unit at depth.** Step 1 of the per-unit pipeline asks for the unit's combinations
  at depth; what a lane writes beyond them is ahead-work for a unit of depth, under the same usage caps as M3b's
  (plan:1106-1107).

### 3.3 Whether the four source-stated platforms can enter earlier

- **The option:** a name enters on the presets whose facts are verified, the others declaring it `later`. It would
  keep in 0.8.0 the deep button facts already verified on `adwaita`, `kde-breeze`, `windows-11` and `material-3`.
- **Against.** It replaces "a model field enters in the commit where every preset states it" (plan:144-146) by a
  matrix of preset and unit, in the gate, in `check_ledger.py --final`, in M4's conditions and in every connector's
  contract. A portable application would meet `Later` on two platforms for a name that works on four, where the
  model-wide rule gives it one answer per release. Each connector would map the deep names at M2.1 for four
  presets and keep stand-ins for four. And M3a's lanes on the four platforms would be asked for full depth on 29
  more widgets, about a third of the per-platform cost the slice measured (40 M of 126 M, brief, decision 2).
  Outcome 2 already weighed this ("the variants already verified on the four platforms … wait for their unit too").
  **Rejected.** `contrast_high` is not this option: it is 0.7.0's depth (1.3), model-wide, and no Apple state model
  lists it.
- **What the existing rule already gives.** A unit made only of names no Apple state model lists needs no Apple
  fact: its Apple presets have nothing to state. `python3 names.py` lists them: **flavours** (kirigami and
  expressive); the keyed variants of the four platforms; and the typed variants `warning`, `maximize`, `minimize`,
  `restore`, `play`, `volume`, `reveal`, `remove`. A unit of conditions, flavours or keyed names adds no enum
  member, so by D14 its release is a patch release (plan:694), unless the unit also adds a model member for a
  question it carries: the flavours unit carries U180's form for Qt Quick's decorations under `flavor:kirigami`,
  which is a patch release only as a field on `Decoration` and a breaking one as a new element datum
  (`register-rows.md`, revision 5, section 2.4 and U180, part 3). No new rule is needed; only the order.

### 3.4 The order

By dependency first, then by what needs no Apple measurement, then by use:

1. **placement**, as planned (plan:786).
2. **flavours**: complete on their presets with published sources once U180's form is chosen; a patch release if
   that form adds no model member (3.3). It is also where `material-3`'s checked text toggle returns, under
   `expressive` (1.7).
3. **content**, **styles**, **menus** and **states** of the buttons-and-toggles unit, then of each other M3a unit:
   the names portable applications ask for most. They add typed variants, so they ship in a breaking release, with
   a typed-widget batch (plan:1244-1246).
4. **sizes**.
5. **window activity**, **increased contrast**, **accessibility text**, **interface level**.
6. **identities** with their hosts, as the hosts' widgets land (M3b's batches for most).
7. **the iPad**.
8. **motion** (after the Apple motion trial, section 5).
9. **glass** (after U110's decision of form: glass as a material, not a colour layer).
10. The parking units, each as the first application need or connector design asks for it (behaviour, text
    rendering, text styles, icon contexts, elevation, parameters, keyed roles, pointer), and **right-to-left**, a
    unit of depth that also takes a staged path (`token.icons.mirroring`), on the same terms.

Which units a release holds is chosen when its branch is cut (plan:1243-1244); this is the order of readiness, not
a schedule.

### 3.5 The method on a measured platform

A unit of depth on `macos-*` or `ios-*` is closed by the brief's option C, in S17's own form:

1. **One designed run** over a sample of the unit's sets: each variant of the unit alone at the default size, in
   each state at depth; each pair of variant groups once; each size once per style.
2. **A composition decision per key and pair**, recorded as S17's *Sets not yet measured* has it ("a decision that
   the pair's looks compose (one variant's facts holding on the other's sets), recorded per key", spec:7397-7398),
   which replaces the pair's pending facts. A pair the sample shows not composing stays measured or pending: looks
   do not always compose (a flat button with a symbol at the mini size has a 1 px top inset the plain flat one
   lacks, brief, decision 2).
3. **The verifier** recomputes the sample from its evidence and reads each decision against it.

It rests on evidence the rows already hold: U114 records that the keys the iPad pass read "equal the iPhone's"
(ledger:510).

---

## 4. The register and the listed paths

`python3 rows_summary.py out/final3.json > out/rows3.md`. A row *leaves* a preset's path when none of its pending
facts of that preset is at depth and no listed path it is behind stays listed. `(d)` marks a decision row.

| Preset | Rows naming a pending fact or a listed path | Leave | Stay in part | Stay whole | Pending facts at depth | Listed paths that stay | that leave |
|---|---:|---:|---:|---:|---:|---:|---:|
| `adwaita` | 2 | 2 | 0 | 0 | 0 | 0 | 4 |
| `kde-breeze` | 13 | 9 | 2 | 2 | 5 | 4 | 12 |
| `macos-monterey` | 47 | 27 | 14 | 6 | 231 | 94 | 264 |
| `macos-tahoe` | 46 | 27 | 13 | 6 | 547 | 77 | 144 |
| `ios-13` | 62 | 37 | 20 | 5 | 328 | 221 | 154 |
| `ios-26` | 64 | 38 | 20 | 6 | 346 | 227 | 154 |
| `windows-11` | 20 | 12 | 4 | 4 | 54 | 55 | 50 |
| `material-3` | 15 | 10 | 1 | 4 | 8 | 9 | 39 |

(The fix round's table, `python3 rows_summary.py` over `depth_count_a2.py --values-at-depth --json`'s output:
`label_position` at depth moves U697 to staying on `kde-breeze`, U616 and U333 to staying in part on the macOS
presets, and adds a pending fact to U484 and U517, which stayed in part already.)

- **`adwaita`.** Leave: U150, U151 (font fallbacks, the touch delay: staged tokens). Nothing stays: the preset loads
  once the staging is applied.
- **`kde-breeze`.** Stay: U180(d) whole (the mnemonic decoration's operands); U697(d) whole (`label_position`, the
  fix round); U422(d) in part (the body's transform origin stays, the menu indicator's leaves leave); U680 in part
  (the stroke's `fill_clip` at the base). Leave: U402 to U405, U420, U421, U423, U424(d), U425(d).
- **`windows-11`.** Stay whole: U509(d) (the monospace slot), U512, U513, U515(d) (the speed factor, section 5); in
  part: U504(d) (`mnemonics` and `mnemonic_when_disabled` stay, `single_click_activates` leaves with behaviour),
  U516 (icon contexts), U517 (the icon's leaves), U518 (text leaves). Leave: U1(d) (until the containers unit, below),
  U183, U501 to U503, U505, U506, U508, U511, U519(d), U520(d), U681(d).
- **`material-3`.** Stay: U295, U550, U559(d), U561(d), and U562 in part (the `headline` slot). Leave: U551, U552,
  U553, U555(d), U556 to U558, U560, U563, U564.
- **`macos-monterey`.** Stay whole: U104, U130, U178, U600, U611, U612; in part: U102, U131, U132, U135, U136, U139
  to U142, U601, U613, U616 (`label_position`, the fix round), U695(d), U751. Leave: U133, U134, U137, U138, U145,
  U181, U182, U301, U302, U602 to U610, U614(d), U615, U690 to U694, U750, U752.
- **`macos-tahoe`.** Stay whole: U90, U104, U178, U322, U331, U332; in part: U91, U92, U95, U97, U99, U100, U102,
  U324, U325, U333 (`label_position`, the fix round), U695(d), U750, U751. Leave: U93, U94, U96, U98, U143, U159,
  U181, U182, U301, U302, U312 to U321 (U314 a decision row), U323, U328 to U330, U693, U694, U752.
- **`ios-13`, `ios-26`.** Stay whole: U127, U452, U456(d), U486, U651(d) (and U256 on `ios-26`); in part: U112,
  U115, U124, U126, U255, U454, U455, U458, U459, U461, U463, U467, U477 to U480, U482 to U484 (U256 on `ios-13`,
  U682 on `ios-26`). Leave: U113, U114, U116 to U118, U120, U121, U122(d), U123, U125, U128(d), U155, U156, U158,
  U164, U451, U457, U460, U464 to U466, U468, U469, U471, U472, U473(d), U474, U475, U481, U485, U650, U685, U686,
  U703(d), U704, U705 (U684 on `ios-13`; U110(d), U111(d) on `ios-26`).

Against revision 2: no row moves between leaving and staying (`diff out/rows2.md out/rows3.md`); `tinted` and
`@gray` add pending facts at depth to rows already staying in part (U91, U92, U95, U99, U100, U750 and U695 on
`macos-tahoe`, U115 on both iOS presets, U682 on `ios-26`). Against revision 1: U504(d) and U515(d) come back on `windows-11` (register-rows (a) and (b)); U95 stays in part on
`macos-tahoe` (`tool`'s strokes); U613 and U324 move from whole to in part (`IconStyle.animation` to motion).
**U1(d)** (`windows-11`'s `tool` padding in a toolbar, six facts in `context toolbar.bar`) leaves 0.8.0's path at
the slice and returns with the containers unit, where `tool` and `toolbar.bar` are both at depth (register-rows-check2,
finding 5).

**The six rows of appendix A that follow outcome 2.** Pending facts at depth and deferred, per preset:

| Row | What it is | At depth | Deferred | Verdict |
|---|---|---:|---:|---|
| U695 | macOS's combined variant sets | `macos-monterey` 57, `macos-tahoe` 287 | 1,376 and 4,393 | **Does not leave whole.** The pairs and triples among `primary`, `tinted`, `toggle`, `tool` and `with_icon` are at depth: nine pairs and five triples on `macos-tahoe`, three pairs and one triple on `macos-monterey` (no `primary` and no `tinted` there) (`python3 depth_pending.py macos-tahoe`, `macos-monterey`). They are 0.8.0's to close, by section 3.5's method at its smallest: one probe pass drawing them, or the decision that they compose |
| U703 | `ios-*`'s sets holding a pointer style | 0 | 651 and 786 | Leaves, with the iPad unit |
| U110 | `ios-26`'s glass fill, stroke, shadow | 0 | 14 | Leaves, with the glass unit: **confirmed** |
| U111 | `ios-26`'s text and icon colours over glass | 0 | 20 | Leaves, with the glass unit: **confirmed** |
| U122 | `ios-*`'s forms that are no configuration | 0 | 284 and 285 | Leaves, with the identities |
| U682 | `ios-26`'s corner radii and curve | 13 | 368 | **Remains: confirmed**, for the corner keys of the sets at depth (`@gray` and `tinted` among them); its other 368 facts are the styles', the sizes' and the iPad's |

Of the 26 decision rows, 10 keep something on 0.8.0's path: U180, U422, U456 (outcome 5 closes it), U504, U509,
U515, U559, U561, U651, U695. The other 16 leave. Of the eight that the brief found standing before M2's task 0
(U180, U504, U509, U515, U681, U555, U559, U561), six remain there: U180, U504, U509, U515, U559, U561; U504 and U515
are settled by register-rows (`mnemonics` `never`, `mnemonic_when_disabled` `absent`; the factor `1`, and `0` under
`animations_off`), U681 and U555 leave.

So `ios-26`'s listed paths differ from `ios-13`'s by U682's five corner paths and one more icon-context path
(U127), not by glass: 226 against 220.

---

## 5. Motion

### 5.1 The facts

- 0.7.0's dialog entrance is a kind without timings: `entrance: Option<DialogEntranceStyle>`
  (`v0.7.0:native-theme/src/model/widgets/mod.rs:851`), `ScaleFade` or `FadeRise`, and "It names the motion only;
  its durations and curves are the toolkit's" (`v0.7.0:native-theme/src/model/dialog_entrance_style.rs:5-12`).
  **Four presets state it** (`git grep -c entrance v0.7.0 -- native-theme/src/presets`: `adwaita`, `kde-breeze`,
  `windows-11`, `material-3`, twice each); `macos-tahoe`, `macos-monterey` and `ios-13` state none.
- 0.7.0's animated busy icons are `load_icon_indicator`'s: the freedesktop theme's, Material's and Lucide's
  spinner, and `IconSet::SfSymbols | IconSet::SegoeFluent => None` (`v0.7.0:native-theme/src/icons.rs:732-739`), as
  24 pre-rotated frames of 42 ms (`v0.7.0:native-theme/src/spinners.rs`, its head comment). The presets' sets are
  `freedesktop` (`adwaita`, `kde-breeze`), `material` (`material-3`), `segoe-fluent` (`windows-11`) and `sf-symbols`
  (the two macOS presets and `ios-13`). So **0.7.0's icon sets animate on three presets**; where the set has none,
  0.7.0's connectors turn an arc of their own (`v0.7.0:connectors/native-theme-egui-widgets/src/spinner.rs:43-52`,
  `paint_arc` at `:262-270`), so a busy indicator turns on every preset, and is still under `reduce_motion` (1.1).
- Their new homes: `entrance` → `dialog.values.presentations` (event `Appear`) (catalogue:1172-1173), in the
  overlays unit; the animated icon → `spinner.values.loop` (catalogue:3556, :3623-3626), in the indicators unit,
  stated "by rules under `cond animations_off`" (catalogue:3611-3613).
- The model's motion types are built and public: `Transition`, `TransitionEngine`, `PresentationTransition`, `Loop`,
  `Keyframe` and the rest (`native-theme/src/lib.rs:251-260`), `native_theme::timing::{evaluate, spring_duration,
  damping_coefficient}` (`lib.rs:231-237`), `native_theme::transitions::governing` (`lib.rs:225-229`). Not built:
  `Motion`, `Places`, `Presence`, `Drawn`, `MotionUpdate`, `interpolate_color`, Compose's spring (the plan's M2.1
  task, plan:1029-1042).
- A loop runs inside `Motion`: its phase "is the element part's own … under every presence while the `Motion` is
  kept", the connector "hands `Motion` the loops that draw on the part", and "a pure function of the loop and the
  time since its start, the phase the connector's" is rejected (spec:1948-1955, :1970-1978). A presentation step
  "names no engine", its engine being "the presentations batch's question (ADD-A129, queue #118)" (spec:2254-2256).
  `Motion::update` and `Motion::sample` take `places: Option<&mut Places>` and `Option<&Places>` (spec:2005,
  :2011). `MotionUpdate.speed_factor` is "`token.motion.speed_factor` resolved", and `color_space` the colour space
  a run moves its colours in (spec:2035-2039); "R9's animations off sets the speed factor, and so a scaled period,
  to zero" (spec:573-575).
- A list is "empty, not absent, where the platform has none" (catalogue:1159-1160), and a transition not measured is
  "never stated `[]` or `"absent"`, which would say the platform animates nothing where it animates something not
  yet measured" (spec:6795-6796).
- The slice's motion facts (`python3 motion_count.py`): a part's `transitions`: `gnome` 3, `kde` 5, `windows` 8 (2
  pending), `material` 46 (1), `macos` 10 (10), `ios` 38 (38); a focus indicator's `transition`: 4 stated, 10
  pending; `button.values.presentations`: 4 stated, 3 pending; `token.motion.*`: 163 facts, 6 pending;
  `token.theme_change`: `kde` 1 stated, `windows` 1 pending. Together 21 of macOS's 27 and 43 of iOS's 48 are
  pending. An icon style's `animation` leaf (counted apart: keys with an `animation` segment under an icon style):
  `gnome` 8, `kde` 14, `material` 3 stated; `macos` 24, `ios` 6, `windows` 2 pending.

### 5.2 What stays in 0.8.0

What follows is the case where the trial of 5.5 measures both motions; a motion it does not measure leaves 0.8.0 on
every preset, and what M2.1 then builds is 5.6's last paragraph.

- **The types**, as built. Nothing is removed from `blocks/motion.rs`, `timing` or `transitions::governing`.
- **Two values:** `dialog.values.presentations`, whole (a keyed list by event, stated as the query resolves it), with
  the overlays unit; `spinner.values.loop`, with the indicators unit; each under the scheme and under
  `animations_off` and `reduced_motion` (1.3, point 6).
- **`token.motion.speed_factor`**, at depth now in either case (1.5: the KDE reader states it, and the loop reads it,
  spec:573-575), with its facts under `animations_off` (`gnome`, `kde`, `material` stated; `windows-11`'s U515,
  settled by register-rows as `1`, and `0` under `animations_off`). The other motion tokens
  (`token.motion.durations`, `.timings`, `.color_space`) enter with the first of the two units whose value reads
  them (1.5's closure); at the slice none is read.
- **Of the runtime:** the interface and the loop. `Motion`, `Places` (an empty `#[derive(Default)]` struct, whose
  `BreezeTabBar` places come with the motion unit, so that `update` and `sample` keep the signatures S2.7 gives
  them), `MotionUpdate`, `Drawn`, `Presence`, with `Motion::update` and `Motion::sample` as S2.7 signs them
  (spec:1994-2027), running the loops and no transition: no preset states one, so every other change of a resolved
  value is drawn in the frame of the change, which is S2.7's own rule for a group with no transition
  (spec:1242-1244) and what 0.7.0 does. `MotionUpdate` holds the fields the loop and that rule read (`at`,
  `presence`, `speed_factor`, `before`, `after`, `rect_before`, `rect_after`, `change`, `widget_loop`); its engine
  inputs and `color_space` are staged, as two of its fields already are (spec:2050-2054). `color_space` and
  `interpolate_color` come back with the indicators or the overlays unit if a stated loop or presentation
  interpolates a colour (Material's spinner loop names "the four-colour cycle, the paint of `glyph.stroke`",
  catalogue:3613; the Material lane's facts decide), and with them U158's Apple measurement (section 8, point 6).
  A presentation is run by the connector's own animation with `timing::evaluate`, which is built; what an
  interrupted presentation does is ADD-A129's batch, which the overlays unit needs applied first.
- **How much of M2's runtime task that is:** none of its engines, none of its vector tables, no clean-room port. The
  task shrinks to the interface, with a test per clause of S2.7's *A loop*. Compose's spring comes back early only if
  a stated step needs one.

### 5.3 What moves to the motion unit

- A part's `transitions` and `shown`; `FocusIndicator.transition`; `IconStyle.animation` (a loop on an icon, which
  the spinner does not use, catalogue:3626-3627, and neither of 0.7.0's motions); every widget's `presentations` but
  `dialog`'s; `token.icons.animated_visuals`; the entries of `token.motion` that only they read; the transitions'
  facts under `animations_off` and `reduced_motion`.
- **The theme-change token**, S4.3 whole: `token.theme_change`, `Resolution.suspends`, `MotionUpdate.theme_change`
  and `suspended_until`, the engines' theme-change rules, S12's zero-duration check. The plan placed it in M3a's
  tokens unit so as not to release "M2.1's interim theme-change rule" (plan:753-755); with no engine in 0.8.0 that
  rule, every change taken at once, is 0.8.0's stated behaviour, so the reason is gone.
- **The engines:** the code of every `TransitionEngine` member, `Places`' content, `take_slide_end`,
  `layout_fraction`, the engine inputs of `MotionUpdate`, `interpolate_color` with its vector tables (unless a 0.8.0
  loop needs it, 5.2), Compose's spring and `ComposeShapeMorph`, the spec batch for queue #98 and #137.
- **The connectors' use** of transitions.
- **The measurements and rows of transitions:** on `macos-*` U181, U182, U301, U302; on `ios-*` U125, U128(d), U481;
  on `windows-11` U183; on `material-3` U552 (all leave 0.8.0's path whole, section 4). U158 (`color_space`) leaves
  with them unless 5.2's condition brings it back. U515 is no transition's row: it stays (5.2).
- The motion unit's release is a breaking one by D14: `MotionUpdate`, an input its caller fills in full
  (spec:7748-7749), gains fields.

### 5.4 The five conditions for transitions

- **(a)** No part lists `transitions` or `shown` (2.4), `FocusIndicator.transition` and `IconStyle.animation` are
  staged out of their structs, the widgets' `presentations` but the dialog's are deferred values: no layer can
  state them (S12), and no engine exists to run them. `ResolvedPartStyle.transitions` stays a field of the one
  resolved type and is empty on every part; its documentation says that no part lists it until the motion unit.
  Rejected: removing `transitions` from `PartStyle` (the resolution and S12's checks of it are built and tested;
  outcome 3 keeps the types).
- **(b)** No fact, preset line or declaration says a platform animates nothing: the facts stay as they are and the
  presets are generated without those keys.
- **(c)** A state change is drawn at once, as in 0.7.0. It is recorded in each connector's contract (a deferred
  entry for `transitions`) and in the release notes.
- **(d)** The listed motion paths leave the tables: 18 on each macOS preset, 14 on each iOS preset, 3 on
  `windows-11`, 3 on `material-3` (`out/rows3.md`, "motion", as in revision 2); `windows-11`'s `tokens.motion.speed_factor` stays
  listed (U515).
- **(e)** The keys are deferred or staged in `widget_units.toml`; the gate skips a fact whose key the model does not
  hold (spec:7515-7516).

### 5.5 The two motions on the Apple presets

The model has no truthful form for "four presets state this value and four do not yet": a value in the model is
stated by every preset, an empty `presentations` says the dialog appears at once, an absent `loop` says nothing
turns, and a pending fact refuses the preset. D13's criterion asks nothing of Apple's icon sets here (5.1), but the
one rule for every model field does. The options:

- **(i) Both values on all eight presets.** The four source-stated platforms state them from their sources, and the
  alert's presentation and the activity indicator's loop are measured on macOS 15 and 26 and iOS 18 and 26: register
  rows of kind `b` in the overlays and the indicators unit. They are Apple motion measurements no trial has made
  (brief, decision 3), two per platform in place of every engine.
- **(ii) Both values leave with the motion unit.** 0.8.0 then loses the entrance kind on four presets and, as a
  stated value, the busy loop on every preset: two additions to R21's list, the maintainer's. The connector then
  keeps 0.7.0's own turning arc (5.1) as a stand-in, a deferred entry of its contract (S-10), not a still spinner.
- **(iii) A value stated on some presets** (a per-preset `later` for a value). Rejected: `values()` returns a struct
  whose field would hold an empty list or `None` on the others, the false statement of (b).
- **(iv) A value that answers 'later' on the presets that do not state it yet** (the Apple presets, where 0.7.0
  states no entrance): truthful, and it keeps 0.7.0's coverage whatever the trial finds; it costs a model form no
  value has today (a per-preset `later` for a value field, its S12 rule and each connector's handling), a class 2
  change for two values. Not taken: the trial may make it unnecessary; to be weighed again if the trial fails
  (ADD-A147 fix round F1, the check F1's finding 12).

**Chosen: the trial first, then (i) or (ii) by its result** (the orchestrator's decision G). No pinned Apple source
states either motion: `xcode@16.4` and `xcode@26.6` carry the API only (`UIActivityIndicatorView.h:35-37`,
`NSProgressIndicator.h:49-52`, `NSAnimationContext.duration`, `NSWindow animationResizeTime:` for resizes, not sheets),
and WebKit has no spinner or alert timing (the check2's verdict 7). So the Apple motion trial runs **before M2.1's
runtime task**, so that the task's scope is known when it starts (the check2's should-fix 6):

- **What it measures.** The alert's presentation (its `Appear` step: duration and timing curve, or the keyframes)
  and the activity indicator's loop (its period and keyframes), on macOS 15 and 26 (the OS probe) and iOS 18 and 26
  (the iOS host). A read route exists on iOS: `UIViewController.transitionCoordinator`'s `transitionDuration` and
  `completionCurve` (`xcode@26.6 UIKit/UIViewControllerTransitionCoordinator.h:47`, `:54`, read at run time), and on
  both platforms the `CAAnimation` objects a running layer holds (their `duration`, `timingFunction`, `repeatCount`,
  keyframes). **A measurement** is a committed run whose evidence holds those values read from the running
  animation's own objects, or a recording whose frames and analysis are committed with it, stated as measured facts
  with a tolerance (S17) and passing the verifier's recomputation like every measured fact.
- **The cap: two harness rounds per Apple harness, four in all** (the OS probe and the iOS host, each round covering
  both versions of its platform). This is decision G's budget, set by the orchestrator, not a figure derived from
  the slice's evidence. The first round answers the trial's question, whether a route reads the values; the second
  corrects that round's harness or analysis defects. A third round would be a search for a method, which is the
  unmeasured Apple motion outcome 3 gave as its reason to defer motion. The trial's usage is reported at M2's close
  with the rest (outcome 10).
- **What the slice's record says about the cap.** The slice's Apple work needed several correcting rounds per lane
  after its first reading (ledger: on iOS the facts lane's rounds 2, 3 and 5 and the tie-break round 4; on macOS the
  facts lane's rounds 2 and 4 to 6 and the tie-break rounds 7 and 8). Those are facts-lane and tie-break rounds, not
  harness rounds (the ledger's *Spikes and drift runs* counts none), but they show that a first Apple reading was
  corrected more than once. So under a cap of two the fallback, both motions leaving 0.8.0 on every preset as losses,
  is a likely outcome, not an edge case, and the maintainer should weigh decision G with that expectation (8.9). One
  difference could make fewer rounds suffice here: the read route above takes values from the running animation's
  own objects, with no image analysis, and an image analysis's defect was the cause of some of the slice's
  corrections (U751: the icon fills read as the glyph's peak coverage). No harness has read such an object yet, so
  this is no evidence that two rounds suffice (the third check's should-fix 5).
- **On success** (a motion measured on all four versions), that motion is stated on all eight presets: (i).
- **On failure** (a motion with no measurement on some version after the cap), that motion leaves 0.8.0 on every
  preset, as one rule for every model field requires: (ii), for that motion. Each motion is one field, so they leave
  independently. A motion that leaves is a loss for the maintainer (R-3): the dialog's entrance kind on `adwaita`,
  `kde-breeze`, `windows-11` and `material-3` (5.1), and the busy loop as stated data on every preset, including the
  frames 0.7.0 drew from the icon sets on `adwaita`, `kde-breeze` and `material-3` (`AnimatedIcon` goes in 0.8.0,
  plan:734), each connector keeping 0.7.0's own turning arc as a recorded stand-in (S-10).

This puts Apple motion measurements back on 0.8.0's path, which outcome 3 gave as its reason to defer motion; the
trial and its cap bound that change, and it is shown to the maintainer (section 8, point 9), not assumed.

### 5.6 What the connectors' design must show

1. **Where a connector keeps state per element part between frames**: the previous query result and one `Motion`.
   The recency order already needs that store (`Element.state_order`, "which it records, beside the previous query
   results it keeps per element", spec:4223-4225).
2. **That every part is drawn from `Motion::sample`'s `Drawn`**, never from the query's style directly, from the
   slice's button on. With no transition `sample` returns the style after, so nothing is drawn differently.
3. **How it asks for the next frame** while a run is active; the spinner's loop needs it in 0.8.0 if the loop
   stays (5.5).
4. **That its public API carries an element's identity and the frame time** wherever a style reaches the
   application: a widget that owns its state, or a call that takes an id. A function from a status to a style has
   nowhere to keep a run.

If a connector's design shows all four, the motion unit adds engines inside `native_theme::transitions`, `Places`'
content and fields to an input the connector builds itself; no application-facing signature changes. **If it
cannot** (point 4 is the one at risk: a toolkit whose styling is a stateless function), that connector's 0.8.0 API is
designed around a stateful widget now: outcome 3's fallback, "the runtime's API is built now and its engines later".

**What M2.1 builds, by the trial's result** (revision 2 said "the interface is built at M2.1 in both cases", which
held only if the loop stays, the check2's should-fix 6). A loop runs inside `Motion` (spec:1948-1955), and a
presentation is run by the connector's own animation with `timing::evaluate` (5.2), so what is built follows the loop:
- **The loop stays** (the trial measures it): M2.1's runtime task builds 5.2's interface, `Motion`, `Places`
  (empty), `MotionUpdate`, `Drawn` and `Presence`, with `update` and `sample` running the loops and no transition, a
  test per clause of S2.7's *A loop*; every connector draws through `Motion::sample` from then on. The presentation,
  if it stays too, needs nothing more of the runtime.
- **The loop leaves**: M2.1 builds no `Motion`. Each connector's design shows where the four points will sit (the
  store, the draw call, the frame request, the identity and the time in its API), so the motion unit later adds the
  runtime without changing an application-facing signature. If one connector cannot show point 4, the runtime's
  interface is built at M2.1 as in the first case (one interface in `native_theme`, no loop to run), and that
  connector draws every part through `sample`, which then returns the style after. The presentation, if it stays, is
  the connector's own animation. Each connector's spinner turns its own arc, a deferred entry of its contract.

---

## 6. The documents' changes

Classes as the plan's *Spec changes* defines them (plan:1375-1389): **1** a data correction; **2** a model change,
an S22 line, checked until a check finds nothing serious; **3** a change to a rationale decision, the maintainer's,
here given by outcomes 2 and 3 (revision 1 wrote "R"). **P** marks an amendment of the plan itself, which the plan's
classes do not cover: the orchestrator writes it with the batch, as ADD-A146 amended the plan. The spec batch takes
the next free ADD number (ADD-A147 at `7430a0a2`). The plan's *Spec changes* gains, with P-14, that a change to a
*Today's fields* selector or a Depth line is class 2.

### 6.1 The rationale (each class 3)

- **R-1, R21, rationale:1399-1402.** Old: "each release's units are whole, every capability filled on them but
  placement, which 0.8.0's widgets defer to the placement unit (their menus, popovers and dialogs open where the
  toolkit puts them, as in 0.7.0), so no widget shows a mixed look."
  New: "each release's units are whole in what they state: every part, field and value of their entries, in every
  combination of the names the release holds. What a unit does not hold yet is a named later unit's and cannot be
  asked for meanwhile: placement (their menus, popovers and dialogs open where the toolkit puts them, as in
  0.7.0); the variants, sizes, host contexts, states and conditions beyond those 0.7.0's model and readers
  distinguish, each entering whole, on every preset, with a later unit of depth; and the state-change transitions
  (a state changes at once, as in 0.7.0). So no element is drawn from rules of two depths."
- **R-2, R21's decision, rationale:1407-1409.** Old: "0.8.0 is not released until every platform preset, `ios-26`
  included, is complete for the units holding today's model and passes the facts gate (R4); each later release
  enables only units complete on every preset and passing the gate."
  New: "0.8.0 is not released until every platform preset, `ios-26` included, is complete for the units holding
  today's model, at the depth today's model distinguishes, and passes the facts gate (R4); each later release
  enables only units, of widgets or of depth, complete on every preset and passing the gate."
- **R-3, R21, the same option's *For*, rationale:1395-1398, and R25.13's ADD-A146 entry, rationale:1691-1694.** Both
  lists of what users lose ("but the status colours on `macos-*` (R25.17), the invented light variants (R19) and …")
  gain, after "the invented light variants (R19)" (the check2's should-fix 5: invented values beside invented
  values): "the `button` values 0.7.0 stated with no source, or against its own cited source, which the facts
  correct (D6's comparison, the plan's *The depth 0.8.0 states*: the `macos-*` and `ios-13` hover fills,
  `kde-breeze`'s inherited primary, `adwaita`'s text colour `#2e3436`, the unsourced sizes and icon gaps, the
  `macos-monterey` pressed fill, the `ios-13` pressed and disabled fills, the `windows-11` disabled fill); the values
  0.7.0 took from a source the preset's reference rule does not choose for it (R1.6 for `material-3`: the older
  `v0_192` token set's disabled container, and material-web's component's vertical padding and minimum width where
  Compose has the button; R1.3 and R1.4 for `ios-13`: WebKit's HTML button's pressed label against UIKit's), which
  the facts restate by that rule; the look of a button with no variant on `material-3` and `ios-13`, which draws the
  platform's default button (Compose's `Button`, the filled one; UIKit's system button), 0.7.0's look staying under
  `tinted` and `@gray` (0.7.0 mapped its plain button to Material's tonal button and to WebKit's HTML button, neither
  the platform's default: Compose `Button.kt:77-88` against `FilledTonalButton`, `:439-447`; UIKit
  `UIButton.h:22-23`); `material-3`'s checked text-button
  colours (0.7.0 stated the tonal toggle of Material's latest token set; baseline Material draws a toggle only as an
  icon button, the content unit's, and a text toggle under the `expressive` flavour, the flavours unit's, where
  Compose 1.5's Expressive `ToggleButton` brings it back)". The last is proposed for the maintainer's veto (section 8,
  point 9), with its two alternatives, not assumed; the corrections are listed for the same veto. If the motion trial
  fails for a motion (5.5), the list also gains "the dialog's entrance kind (`adwaita`, `kde-breeze`, `windows-11`,
  `material-3`) and the busy indicator's loop as theme data (every preset; the icon sets' frames on `adwaita`,
  `kde-breeze` and `material-3`)", for that motion. The same lists take `register-rows.md`'s two additions (its
  section 7, point 7: the icon contexts a platform names none for, the icon theme's name), which that document
  proposes; this note does not restate them.
- **R-4, R25.13's list**, a new entry after rationale:1696: "Amended by the 0.8.0 plan's checkpoint D6 (2026-11-10,
  outcomes 2 and 3): R21: a unit enters at the depth 0.7.0's model and readers distinguish, its further variants,
  sizes, host contexts, states and conditions in later units of depth; 0.8.0 holds the two motions 0.7.0's model
  has, the dialog's presentation and the busy indicator's loop, with the speed factor and the reduced-motion
  conditions, each if an Apple trial measures it on the Apple presets, and state-change transitions are the motion
  unit's; the invented values D6's comparison finds join R19's on the list of what users lose. R4: the gate checks the combinations of the names
  its enabled units hold. R2.7, R15: a variant, size or flavour the platform has and a later unit states is answered
  'later', never 'not available'."
- **R-5, R4, *Which combinations*, after rationale:540-541** ("The map is a survey; the platforms' state models are
  the list."), one sentence added: "At each release the gate checks the combinations whose names its enabled units
  hold; a combination holding a variant, a size, a host context, a position, a direction, a state or a condition of
  a unit not enabled is that unit's, checked from its commit on." And in *What the gate proves*, rationale:556:
  "every listed combination" → "every listed combination of the enabled units' names".
- **R-6, R2.7, rationale:433-435**, a sentence added after "a density the widget does not declare is ignored
  (R15).": "A variant, a size or a flavour the platform has and a later unit states is answered 'later': the theme
  does not state it yet." **R15, rationale:1193-1194**, the same sentence after "rather than getting an invented one."
- **R-7, *Today's model*, rationale:102-103** (a correction of a description, class 3 by its place). Old:
  "**Accessibility** is four run-time flags on `AccessibilityPreferences` (`text_scaling_factor`, `reduce_motion`,
  `high_contrast`, `reduce_transparency`), with no theme values behind them." New: "… with no preset values behind
  them; under `high_contrast` the Windows reader writes the system colours and the GNOME reader the focus ring into
  the theme, the macOS reader's colours follow Increase Contrast, and the connectors honour `reduce_motion` for the
  busy indicator and their own animations."
- R9 needs no change: it decides the model's motion, which stands; which release states it is R21's.

### 6.2 The specification (core; each class 2)

- **S-1, S6.1, spec:3879-3883.** Old: "(a unit: a set of catalogue entries the list names (the list also names the
  hand-written units, S13's placement among them, each enabled or not), less the values it defers, each to a named
  later unit — the button's `Placement` values to the placement unit, ADD-A120 — a deferral ending in that unit's
  commit, and `--check` refusing a deferral to a unit already enabled), so a unit enters the model in the commit
  whose presets state it;"
  New: "(a unit: a set of catalogue entries the list names (the list also names the hand-written units, S13's
  placement among them, each enabled or not), less what it defers, each to a named later unit, a deferral ending in
  that unit's commit, and `--check` refusing a deferral to a unit already enabled. It defers: a value (the button's
  `Placement` values to the placement unit, ADD-A120); a part's field, or a leaf of a shared block (a part's
  `transitions`, `IconStyle.animation`, to the motion unit); and of an entry's names every typed variant its
  **Depth** line (the catalogue's *Entry format*) does not list, to the unit of depth the units file names. The
  generator writes `Variant`'s members, and each widget's `variants` and `content_variants`, from the Depth lines of
  the enabled entries and the units of depth enabled, and the enabled flags of the sizes and flavours units (S8,
  S16)), so a unit enters the model in the commit whose presets state it;"
  And after "(`Behavior.scroll`, …)" at spec:3884: "A token path is staged so wherever no *today's fields* row names
  it, no value at depth reads it, the core does not read it (`token.size_classes`, `token.text.font_dpi`,
  `token.text.scaling`, `token.kde`, `token.material`), no connector reads it to draw a value at depth (S4.2's
  `mnemonics` and `mnemonic_when_disabled`), and no system reader states it (S9.3); it waits in the parking unit the
  units file names until the first unit whose value reads it. A keyed entry is declared by a preset with the first
  unit whose value reads it, or with the reader that states it."
- **S-2, S8, the code block at spec:4425.** `pub enum Unavailable { Widget, Size, Variant, Part }` → `pub enum
  Unavailable { Widget, Size, Variant, Part, Later }`, with the comment "`Later`: the platform has the variant or
  the size, and a later unit states it (S10)". **And spec:4500-4502**, after "for an undeclared variant the
  application asked for,": "`NotAvailable(Later)` for an element holding a variant, a content variant included, that
  its widget's declarations in force list under `later` (S10), or asking a size it takes that is not its default in
  force while the sizes unit is not enabled (S6.1), an answer that comes before every other but an unavailable
  widget's,".
- **S-3, S10**, a paragraph after spec:5575: "**Later names.** `later = { variants = […] }` in a widget's or a
  component's declaration lists the declared variants the platform has and a unit not yet enabled states: the
  keyed variants of a unit of depth, and a typed variant at depth that the preset's state model draws in no
  combination of the enabled units' names. The declarations stay the state model's whole, a typed variant the
  model does not hold yet left out (it has no spelling). A `later` variant selects nothing and is refused in every
  layer (S12); the query answers `NotAvailable(Later)` for it (S8). A size or a flavour of a unit not enabled keeps
  its declaration; the query answers `Later` for such a size and `set_flavor` refuses such a flavour (S8, S16). An
  exclusive group or a recency set lists only flags of the enabled units' names; `flags` lists only the keyed flags,
  and `[declare] conditions` only the system settings, that a rule of the enabled units' facts names; a keyed
  namespace declares only the entries a base states, a map with none stated empty. **The state model in force:**
  where the generator, its round trip and the gate read a state model for what an element can hold, an element
  takes its default size in force alone while the sizes unit is not enabled, the default flavour's model alone is in
  force while the flavours unit is not enabled, and no element holds a variant of a unit not enabled; the
  declarations stay whole. S12's own combinations at load still add every declared flavour's atom where a state,
  variant or flavour atom is named (`complete()`, `validate/combos/mod.rs:1838-1846`), a flavour of a unit not
  enabled included; no rule of a bundled preset names such a flavour, so the values under it are the default
  flavour's." And the example at spec:5502-5522 gains the `later` line of 2.3. (The sentence on `complete()` is the
  third check's minor 11; that the load stays harmless is the check's reading and mine, confirmed only by the staging
  task's load of each regenerated preset, P-6b.)
- **S-4, S12**, with the undeclared names (spec:5815): "a variant a `later` list holds, or a size or a flavour of a
  unit not enabled, in a positive or a negative list of a selector, a reference or a patch, is refused with a
  problem of its own: the theme declares the name, and a later unit states it (`ProblemKind::Later`, S16's
  diagnostic enums)." (Revision 3 wrote "is refused as an undeclared one", whose message would say the theme lacks
  a name it declares, the third check's minor 10.) **And spec:5996-6006**, the exemptions of the check of declared names no rule uses, gain: "a
  flavour of a unit not enabled (S6.1), with its `[declare.flavor.<f>]` sections" (it cannot be asked for, S16, and
  the sentence above refuses any rule naming it).
- **S-5, S16, spec:6430.** `set_flavor`'s comment "undeclared: refused" → "undeclared: refused; a declared flavour
  but the default while the flavours unit is not enabled (S6.1): refused, with a problem of its own";
  **spec:6434-6436**, `flavors()`: "the current preset's declared flavours" gains "(the default alone while the
  flavours unit is not enabled)".
- **S-6, S17, *While the units are built*, spec:7428-7433.** Old: "A fact belongs to the unit of its key, to the unit
  of every widget its `when`'s positive lists name (`context`, its `scope.nearest`, a `widget`) and to the unit of
  the widget a `drawn_as` value names, and the gate skips it while any of those units is not enabled (ADD-A146). Its
  negative lists (`not_context`, `not_variants`) never add a unit: …"
  New: "A fact is of the enabled units when its key lies in an enabled unit, every widget its `when`'s positive
  lists name (`context`, its `scope.nearest`, a `widget`) and the widget a `drawn_as` value names are enabled, and
  its `when` holds in at least one combination of the enabled units' names (each entry's Depth line, the units of
  depth enabled) that its state model draws, its part drawn there; the gate skips every other fact, its later unit
  being the first in the units file's order whose names make it hold (ADD-A146, ADD-A147). A key that belongs to no
  widget (a colour role, an input, a token) is judged under the condition sets the catalogue's *today's fields*
  table names for it (the scheme and `contrast_high`, `window_inactive` for the `selection` set, `animations_off` and
  `reduced_motion` for the speed factor and the motion values' tokens) and the preset's default flavour. A part whose
  presence fact is pending counts as drawn. A bundled preset mentions no name outside the
  enabled units': a negative entry naming one (`not_context`, `not_variants`, `not_states`, `not_conditions`,
  `not_sizes`) is left out of its rule, and its exclusive groups and recency sets list the enabled units' flags. …"
  (the rest of the sentence, from "`not_context X` applies everywhere but inside X", stands).
- **S-6b, the same paragraph, spec:7436-7437.** Old: "a combination that holds a skipped variant, or a set holding
  one, or a context naming a widget whose unit is not enabled is skipped too." New: "a combination that holds a name
  outside the enabled units' (a variant or a set holding one, a size, a state, a host context or one naming a widget
  whose unit is not enabled, a position, a direction, a condition) is not drawn." **And spec:7433-7436** (the
  check2's should-fix 9). Old: "A skipped fact (…) takes with it every combination where it applies, so that no
  earlier fact applies there in its place (as `generate_preset.py` leaves such a fact's combinations out)". New: "A
  skipped fact (…) is drawn in none of its combinations by the gate (as `generate_preset.py` leaves them out); at run
  time an element under a given name outside the enabled units' resolves as without it (S10), the bundled preset
  mentioning no such name."
- **S-7, S17, the same paragraph, spec:7437-7447.** Old, from "The declarations check skips a state-model variant
  only where facts name it and every one of them is skipped" to "(once facts name it, GN-2.37's badge, it is skipped
  or checked as those facts are)."
  New: "The declarations check compares each preset's declarations with the state model's names that the model's
  types can spell (a typed variant no enabled unit holds is skipped, as a fact whose key the model does not hold),
  its exclusive groups, recency sets and keyed `flags` with the state model's restricted to the enabled units' flags
  (a keyed flag only where a fact of the enabled units names it), and its
  `later.variants` with the state model's keyed variants of units not enabled and its typed variants of enabled units
  that it draws in no combination of their names (S10). A variant of the enabled units that no fact names draws as
  the rules that apply give it, with none as the base, which the lane states by leaving it so (libadwaita's
  unchecked `toggle`)." (Of the paragraph's examples `toggle` stays; `pane_toggle`, `close`, `previous` and the rest
  are the identities', and `@tab_button`'s set is the styles'.)
- **S-8, S17, *A pending fact with nothing beneath it***, a sentence after spec:7052: "a pending fact leaves a path
  listed only from a combination of the enabled units' names; a path a unit stages or defers is no listed path."
- **S-9, S21.1, spec:7728-7729.** Old: "At every release every platform preset, `ios-26` included, states every
  field the model then holds, and the release passes the same gates over its enabled units".
  New: "At every release every platform preset, `ios-26` included, states every field the model then holds, under
  every combination of the names its enabled units hold, and declares every variant, size and flavour its platform
  has that the model's types can spell, a variant of a unit not enabled under `later`, and the keyed flags of the
  enabled units (S10); the release
  passes the same gates over its enabled units".
- **S-10, S21.3, after the stand-in entry, spec:7792**, a paragraph: "**A deferred entry.** For each enabled entry the
  contract table lists every state the connector reports, every keyed flag it sets, every host context it builds and
  every condition it supplies that a unit of depth not yet enabled holds for that entry, and every construct of its
  toolkit (a variant, a size, a content form) that only such a name describes, with what it draws meanwhile (the look
  at depth, or its 0.7.0 mapping from the values at depth) and the unit that states it. The contract test fails a
  table that lacks one against `scripts/widget_units.toml`, and one that keeps an entry whose unit is enabled. A
  part's `transitions` are such an entry until the motion unit." **And spec:7767**, "every content-variant pair of
  the catalogue (S8)" → "every content-variant pair of the catalogue whose variant an enabled unit holds (S8)".
- **S-11, S2.7** (written as below if the trial measures the loop, 5.5; if it does not, the paragraph says that 0.8.0
  holds no `Motion` and every motion is the motion unit's). At spec:1991, before "The connector keeps, per element
  part": "**In 0.8.0** the runtime is its interface and the loop: no part lists `transitions` or `shown`, `FocusIndicator.transition`,
  `IconStyle.animation` and the widgets' `presentations` but the dialog's are the motion unit's (S6.1), so `update`
  starts no transition and `sample` returns the style after with the loops drawn over it; `Places` is an empty
  struct until that unit, and `take_slide_end`, `layout_fraction` and the engines come with it." In the
  `MotionUpdate` block, spec:2033-2059: the fields `key`, `color_space`, `kept_before`, `kept_after`,
  `conditions_before`, `conditions_after`, `activated`, `slide_to`, `replayed`, `pressed_through` gain the comment the
  two fields at spec:2050-2054 carry, reworded: "staged: enters with the motion unit (S6.1)" (`color_space`: "or with
  the first unit whose stated loop or presentation interpolates a colour"); and those two change "the tokens unit" to
  "the motion unit".
- **S-12, S4.3**, its first paragraph (spec:3534 on) gains: "Staged with the motion unit (S6.1): until then a theme
  change is drawn at once."
- **S-13, S2.9, spec:2511** (`IconStyle.animation`'s line) gains: "staged: enters with the motion unit (S6.1)"; and
  spec:413's list of whole `Value` block fields stands (the field returns whole).

### 6.3 The catalogue (each class 2)

- **C-1, *Entry format*, after catalogue:439**, a new item: "**Depth:** the names the entry's own unit holds: its
  typed variants, its state flags, its host contexts, positions and conditions beside the scheme and `contrast_high`,
  each named by a selector of a *Today's fields* mapping that targets the entry (in any entry's item), required by a
  state model for one so named, or listed by a preset's `requires_variants`; the conditions under which 0.7.0's
  readers or connectors gave the entry values (`animations_off` and `reduced_motion` for a motion value); the default
  size in force; and the values that only names of later units read, each with that name. Every other name of the
  entry's state models and facts is a later unit's (`scripts/widget_units.toml`)."
- **C-2, `button`, after catalogue:2064:** "**Depth:** variants `primary`, `toggle` (the state models draw `checked`
  with it alone), `with_icon`, `tool` (the toolbar's mappings), `tinted` (`material-3`'s plain-button mappings),
  and on `ios-*` `@gray` (`ios-*`'s plain-button mappings); states `hover`, `pressed`, `disabled`, `checked`,
  `default`, `focused`, `focus_visible` (on `ios-*` `hover` and `focus_visible` are the iPad's, U112, U113); the
  default size in force; no host context (`toolbar.bar` from the containers unit, `toolbar`'s item); the scheme and
  `contrast_high`. On `material-3` `toggle` is `later` (drawn only with `icon_only` or under `expressive`), and on
  `ios-*` `tool` (drawn only in the ink toolbar). Values of
  later names: `label_position` and `help_tooltip_on_click` (styles), `menu_indicator_gap`,
  `menu_indicator_anchor`, `menu_trigger` (`menu`), `back_title_fallback`, `back_display_mode` (`back`),
  `display_mode` (`@paste_control`), `repeat_delay`, `repeat_interval` (`@repeat`); `presentations` and each part's
  `transitions` are the motion unit's." (The fix round takes `label_position`, `repeat_delay` and `repeat_interval`
  out of this list: the correction at the head of this note.)
- **C-3, `button`'s *Today's fields*, catalogue:2055-2056.** Old: "`icon_text_gap` → `values.icon_text_gap`;". New:
  "`icon_text_gap` → `values.icon_text_gap @ variant with_icon` (the gap of a button holding an icon and a label,
  which `adwaita` states under that content variant; the base where a preset declares no `with_icon`);". (Revision 1
  classed it 1; it decides what a release states, finding 12.)
- **C-3b, the same item, catalogue:2056.** "(on `macos-*`: `@ {default}`)" → "(on `macos-*` and `kde-breeze`:
  `@ {default}`, Breeze's default button)", for `primary_background` and, by the same words, `primary_text_color`.
- **C-3c, `toolbar`'s *Today's fields*, catalogue:4729-4731.** "`font` → `button.label.text @ context toolbar.bar`"
  → "`font` → `button.label.text @ variant tool, context toolbar.bar` where the preset declares `tool` (Qt's tool
  button, WinUI's app-bar buttons, AppKit's toolbar bezel, the note below), else `@ context toolbar.bar`"; and
  `icon_size_px` → `button.icon.icon.size` likewise.
- **C-3d and C-3e, `button`'s *Today's fields*, catalogue:2054-2063** (decision E, 1.3): after the item's mappings,
  "On `ios-*` the plain button's mappings are `@ variant @gray` (UIKit's gray configuration, WebKit's iOS button's
  look, which 0.7.0 drew), and on `material-3` `@ variant tinted` (the filled tonal button, which 0.7.0 drew): every
  mapping but `primary_*`'s takes the variant, beside its own where it has one (`icon_text_gap @ variant @gray,
  with_icon`; `checked_* @ variant tinted, toggle {checked}`, which `material-3` draws nowhere at depth, 1.7). An
  element with no variant draws the platform's default button there, UIKit's system button with no configuration
  (`UIButton.h:22-23`) and Compose's `Button`, the filled one (`Button.kt:77-88`): 0.7.0's mapping of its plain
  button to the look these variants hold is corrected, and 0.7.0's look stays under them." (Revision 3's "UIKit's
  plain one" named the `plain` configuration, the typed `flat`, catalogue:1832, which the base is not, the third
  check's minor 9.)
- **C-4, the *today's fields* table, after catalogue:10304:** "The typed roles enter whole, under the scheme,
  `contrast_high` and the conditions this table names (`window_inactive` for `role.selection.background`). A token
  path this table does not name enters by S6.1's rule; every keyed role, text style, elevation entry and icon context
  is declared with the first unit whose value reads it, or with the reader that states it. A token at depth is
  stated under the scheme, `contrast_high` and the preset's default flavour, and the speed factor and the tokens the
  two motion values read also under `animations_off` and `reduced_motion` (the check2's should-fix 8)."
- **C-5, `dialog`, at its `presentations` value (catalogue:1049), and `spinner`, at `loop` (catalogue:3556):** "Held
  by 0.8.0 (the two motions of 0.7.0's model), under the scheme, `animations_off` and `reduced_motion`; on `macos-*`
  and `ios-*` measured." Written only for a motion the Apple trial measures (5.5); a motion that fails is deferred
  to the motion unit instead, as `transitions` are.

### 6.4 The plan (each P)

- **P-1, D13's consequences, plan:510-512.** Old: "`ios-26` stays in 0.8.0 (S21.1 makes it a release condition, and
  its slice facts are written): every preset that exists states every field the model holds."
  New: the same, and: "By D6 (2026-11-10) the criterion applies inside a unit too: 0.8.0 states each entry at the
  depth 0.7.0's model and readers distinguish (*The depth 0.8.0 states*), and of motion the dialog's presentation
  and the busy indicator's loop, each where the Apple motion trial measures it (M2's task 1)."
- **P-2, *One rule for every model field*, plan:155-158.** Old: "A unit enters whole, its catalogue entries whole,
  less the values S6.1 lets it defer to a later unit (the `Placement` values, to the placement unit); a rule under a
  context of a widget a later unit holds, or drawing a part as one (`drawn_as`), waits for that unit, as
  `check_facts_trace.py` states."
  New: "A unit enters with every part, field and value of its catalogue entries, in every combination of the names
  their **Depth** lines list, less what S6.1 lets it defer to a later unit: a value (the `Placement` values, to the
  placement unit), a part's field or a block's leaf (`transitions`, `IconStyle.animation`, to the motion unit), a
  name beyond its depth (to a unit of depth), a token path nothing at depth reads (to the first unit that reads it,
  waiting in a parking unit). A rule under a context of a widget a later unit holds, or drawing a part as one
  (`drawn_as`), waits for that unit, as `check_facts_trace.py` states. A unit of depth obeys the same rule: it enters
  in the commit where every preset states its names, on every entry then enabled, or declares, as the facts state,
  that its platform has none."
  And plan:149-151, "S17 gives a fact its key's unit, the unit of every widget its `when`'s positive lists name and
  of the widget a `drawn_as` value names" gains "and, of its combinations, those of the enabled units' names". And
  plan:151-152, "A fact *of the enabled units* is one whose units are all enabled." → "A fact *of the enabled units*
  is one whose units are all enabled and which holds in a combination of their names that its state model draws
  (S17)." And plan:152-153, "From M4 the gate fails on a skipped fact of the enabled units and on a fact whose key
  lies in no unit" → "From M4 `check_facts.py` fails a fact whose `when` the names of all units together do not make
  hold (a stray fact, which under S17's predicate is never a skipped fact of the enabled units, so the old check
  would never fire; the check2's should-fix 7), and the gate a fact whose key lies in no unit or that names a name, a
  condition or a staged path no unit holds".
  And the *Rejected* line, plan:160-162, gains: "a deferred name declared but drawn as the base, or left out of the
  declarations with no answer of its own (the first shows a look the platform does not draw for it, the second
  states that the platform lacks it)."
- **P-3, *0.8.0's scope***, a new subsection after plan:739, **The depth 0.8.0 states (D6)**: the rule of section 1
  of this note in five sentences, the tables of 1.6, the comparison of 1.7 with its test, and "Of 0.7.0's `button`
  fields, two sourced looks come to depth under the variant that holds them (`material-3`'s filled tonal button,
  `tinted`; `ios-13`'s grey fill, `@gray`); `material-3`'s and `ios-13`'s button with no variant draws the
  platform's default button (Compose's `Button`, the filled one; UIKit's system button), 0.7.0's look staying under
  `tinted` and `@gray`, a correction of 0.7.0's mapping (Compose `Button.kt:77-88` against `FilledTonalButton`,
  `:439-447`; UIKit `UIButton.h:22-23`); one look is proposed as lost, `material-3`'s checked text toggle (R21), held
  only under the `expressive` flavour, a shared unit's name; the values 0.7.0 stated with no source are corrected
  by the facts and listed with R19's invented light variants (R21): the `macos-*` and `ios-13` hover fills,
  `kde-breeze`'s inherited primary (C-3b), `adwaita`'s `#2e3436`, gap and minimum width, `kde-breeze`'s minimum
  height, `macos-monterey`'s pressed fill, the `macos-*` gap and minimum widths, `ios-13`'s sizes, gap and pressed and
  disabled fills, `windows-11`'s disabled fill and minimum sizes; and the values 0.7.0 took from a source the preset's
  reference rule does not choose for it are restated by that rule (R1.6 for `material-3`: the older `v0_192` token
  set's disabled container, material-web's component's vertical padding and minimum width where Compose has the
  button; R1.3 and R1.4 for `ios-13`: WebKit's HTML button's pressed label against UIKit's)" (the check2's should-fix
  5 and minor 17; the third check's should-fix 1 and 2).
- **P-4, the units, plan:743-755.** Unit 1, **tokens and conditions**, is removed from M3a, and the list renumbered:
  "Each token group of 0.8.0 enters by S6.1's rule with the first unit whose value reads it (D6): S4.6's groups with
  their readers (`window`'s frame `token.params.display`, `input`'s caret `token.params.caret`, `list`'s
  `token.params.layout`), S4.8's with the core; S4.3's theme change is the motion unit's (D6, outcome 3: with no
  engine in 0.8.0 a theme change is drawn at once, which is 0.8.0's stated behaviour and no interim rule); S4.7's
  `pointer`, which 0.7.0's model lacks and the readers unit (now M3b's, outcome 4) was to read first, is the pointer
  unit's. A condition key is declared by the presets with the first unit whose rule selects on it, as S12 refuses a
  declared key no rule selects on, and the reader's read of it comes with that unit's readers step." The *Rejected*
  sentence at plan:753-755 goes.
  **And plan:1082-1083.** Old: "Token groups that add preset fields (S4.3, the rest of S4.6, S4.7, the rest of S4.8)
  and S13's placement are not here: they enter with their M3 units, with their facts and vectors." New: "Token
  groups that add preset fields enter with the first M3 unit whose value reads them (S6.1; S4.3 with the motion
  unit, S4.7 with the pointer unit, both after 0.8.0), and S13's placement with its unit, with their facts and
  vectors." **plan:1092-1093.** Old: "The units, in order: tokens (S4's remaining groups: S4.3's theme change with the
  runtime rules M2.1 leaves out, the rest of S4.6, S4.7, the rest of S4.8) and conditions; placement …". New: "The
  units, in order: placement …", and after "the keyed components" at the list's end: "; the units of depth, the
  parking units and motion (*After 0.8.0*)". **plan:1104.** "tokens and conditions, readers, window and chrome" →
  "window and chrome" (the readers' move is outcome 4's batch).
- **P-5, plan:786-787.** Old: "**After 0.8.0 (M3b):** placement (ending those deferrals); the 57 typed widgets the
  model lacks today, in alphabetical batches of about ten; the 28 keyed components."
  New: the same, and: "; the units of depth of 0.8.0's entries (content, styles, menus, states and identities per
  widget unit; sizes, flavours, increased contrast, window activity, the iPad, accessibility text, interface level,
  glass, right-to-left), each model-wide and closed on the measured platforms by a designed sample run with a
  composition decision per key (S17's *Sets not yet measured*); the parking units (behaviour, text rendering, text
  styles, icon contexts, elevation, parameters, keyed roles, pointer); and motion. A unit no Apple state model names
  (flavours) needs no Apple fact and, adding no enum member, is a patch release (D14) unless a question it carries
  adds a model member (U180's decoration form)."
- **P-6, M2's task 0, plan:996-1000.** The paths per preset become those at depth: "(`adwaita` 0 paths, `kde-breeze`
  4, `material-3` 9, `windows-11` 55 once the staging task is merged; …)", and the staging task (2.7), whose text
  P-6b gives, is named as its predecessor, with its gate. (Revision 1's P-9 is withdrawn: one time, 2.7.)
- **P-6b, the staging task, a new entry of M2 before task 0, plan:996** (unnumbered, so that tasks 0 to 7 and M2.1
  keep their numbers; the third check's should-fix 6). The text:

  > **Before task 0: the depth 0.8.0 states** (D6, outcomes 2 and 3; the D6 staging design). One code and tooling
  > task, after the spec batch that states the depth rule and its form (ADD-A147: R21's amendment, S6.1, S8, S10,
  > S12, S16, S17, S21's lines, the catalogue's Depth lines and *Today's fields* corrections) and before task 0. It
  > brings the slice's model, its generators and checkers and the eight bundled presets to the depth 0.7.0's model
  > and readers distinguish (*The depth 0.8.0 states*). No fact is edited.
  >
  > *Inputs:* the catalogue's Depth lines and *Today's fields* mappings as ADD-A147 leaves them; the facts at the
  > task's base; the D6 staging design's tables, whose figures the task reproduces (it does not fit to them: a count
  > that differs is reported).
  >
  > *Steps, by file:*
  > 1. `scripts/widget_units.toml`: every unit of depth with the names it holds per entry (and its presets where it is
  >    limited to some: the iPad unit's `hover` and `focus_visible` on `ios-*`), every parking unit with its paths, an
  >    `enabled` flag each; `[deferred.button]` gains the values only later names read (C-2's list) and the motion
  >    unit's part fields (`transitions`, `shown`); `[staged.slice]` gains every token path the token rule stages,
  >    each to its parking unit or its unit of depth.
  > 2. The predicate of a fact's units (S17) in the four readers of the units file, by one rule: the gate
  >    (`native-theme/tests/facts_gate/units.rs`, `run.rs`), `scripts/check_facts_trace.py`,
  >    `scripts/generate_preset.py`; `check_ledger.py` and `verify_facts.py units` by the same predicate.
  >    `check_facts.py` checks that every name of every state model, every condition a fact names and every token path
  >    a fact names lies in exactly one place: an entry's Depth line, one unit of depth or one parking unit.
  > 3. `scripts/generate_widgets.py`: reads the Depth lines (`--check`: each line's catalogue part against the *Today's
  >    fields* selectors that name a target in the entry; no deferral to an enabled unit) and writes `Variant`'s
  >    members, each widget's `variants` and `content_variants` from the enabled entries' Depth lines and the enabled
  >    units of depth, the enabled flags of the sizes and flavours units, and the values structs less the deferred
  >    values (`native-theme/src/widgets/gen_index.rs`, `gen_button.rs`); the tests and gate fixtures that name a
  >    removed variant rewritten (`native-theme/tests/facts_gate.rs`, `src/query/tests.rs`, `src/toml/tests.rs`,
  >    `src/widgets/tests.rs`, `src/validate/`).
  > 4. `native-theme/src/tokens.rs` and the hand-written blocks: the staged fields leave their structs
  >    (`MotionTokens.durations`, `.timings`, `.color_space`; `TextRendering` and `FontFallbacks` from `TextTokens`;
  >    the nine staged slots from `TextRoles`; `Caret` from `Tokens`; every `Behavior` field but `mnemonics`,
  >    `mnemonic_when_disabled` and `focus_visible_when`; `IconTokens.mirroring` and `.animated_visuals`;
  >    `IconStyle.animation`; `FocusIndicator.transition`; `token.theme_change`), with the declarations they read and
  >    their `DECLARATIONS` rows (`native-theme/src/validate/checks.rs`); first a code search confirms the core's
  >    token reads and the connectors' reads S4.2 names, so that no path the core reads is staged.
  >    `ResolvedPartStyle.transitions`' documentation says that no part lists it until the motion unit.
  > 5. The query and the theme (S8, S16): `Unavailable::Later`, answered for an element holding a `later` variant (a
  >    content variant included) and, while the sizes unit is not enabled, for a size the element takes other than
  >    its default in force; `Theme::set_flavor` refuses a declared flavour but the default while the flavours unit
  >    is not enabled, with a problem of its own, and `flavors()` lists the default alone.
  > 6. S12 (`native-theme/src/validate/`): the `later` declaration read; a rule, reference or patch naming a `later`
  >    variant, or a size or a flavour of a unit not enabled, refused with `ProblemKind::Later`; `checks::unused`
  >    exempts a flavour of a unit not enabled with its `[declare.flavor.<f>]` sections; the declarations check
  >    compares `later.variants`, the trimmed exclusive groups, recency sets and keyed `flags` with the state model
  >    (S17).
  > 7. `scripts/generate_preset.py`: the state model in force in one place, the `models` dict `declarations()`
  >    returns (the default flavour's models only; the button's sizes the default in force; no `sizes_by_variant`
  >    entry of a variant not at depth), which `covering_bases()`, `pending_table()` and `round_trip()` read; the
  >    facts of the enabled units by the predicate; every mention of a name not at depth left out of a rule (a fact
  >    naming it positively, a negative entry naming it; exclusive groups and recency sets trimmed to the flags at
  >    depth, a group of one dropped; a one-of `sizes` list trimmed); `later.variants` written from the state model
  >    and the units file; `flags` and `[declare] conditions` from the trimmed facts; each keyed namespace's
  >    declaration trimmed to the entries a base states (a stated base, or a listed path the synthetic route's bases
  >    state), a map with none stated empty (`{}`).
  > 8. The eight presets regenerated (`generate_preset.py --write`, then `--check`), and their hand-written parts
  >    edited where the generator reports them: the keyed `[declare]` entries no base states removed; on `ios-*` the
  >    rule block of `role.global.accent`'s read fact moved to the bases of `colors.<scheme>.global`; on `macos-*` the
  >    bases blocks of `colors.<scheme>.keyed` removed.
  > 9. `native-theme/src/macos.rs`'s module comment: `role.@selected_control` leaves its list of roles the preset
  >    states. No reader's reads change.
  >
  > *Gate:* M2's gate per task, and: `generate_widgets.py --check` and `generate_preset.py --check` on all eight
  > presets (no mismatch, no unstated field); the self-tests of `check_facts.py`, `check_facts_trace.py`,
  > `check_ledger.py` and `verify_facts.py`; each regenerated preset loaded in Rust through `check_preset`, and as
  > its positive control a test that S12's unused check refuses `ios-13` with its `@loading` flag declared and
  > `kde-breeze` with its `@single_click` setting declared (the form's definition is what keeps them out, so only the
  > loader shows it holds); the facts gate green over the eight regenerated presets with the predicate in force;
  > `cargo test --workspace` outside `PLAN_EXCLUDED`.
  >
  > *Done when* the gate is green and the eight `[pending]` tables hold exactly the listed paths the design gives as
  > staying: `adwaita` 0, `kde-breeze` 4, `macos-monterey` 94, `macos-tahoe` 77, `ios-13` 221, `ios-26` 227,
  > `windows-11` 55, `material-3` 9 (687 in all; the fix round's figures). A table that differs is reported with the paths that differ, and the
  > task does not close until the orchestrator has decided it.
- **P-7, M2's task 1, plan:1029-1042.** The runtime paragraph, from "Second, `native_theme::transitions`' runtime" to
  "which M3's tokens unit ends.", is replaced by: "Before this task's runtime part, the Apple motion trial (D6,
  outcome 3; the D6 staging design, 5.5): the alert's presentation and the activity indicator's loop read from the
  running animations on macOS 15 and 26 and iOS 18 and 26, at most two harness rounds per Apple harness (D6's
  budget for the trial, not a figure from the slice's evidence, whose Apple lanes needed several correcting rounds
  each; the fallback is a likely outcome); a motion measured on all four is stated on every preset, one that is not
  leaves 0.8.0 on every preset, a loss for the maintainer. Second, if the loop stays, the motion runtime's interface (spec S2.7): `Motion`, `Places` (empty),
  `MotionUpdate`, `Drawn` and `Presence`, with `Motion::update` and `Motion::sample`, running the loops a part's
  style and its widget's `loop` value state and no transition, `MotionUpdate.speed_factor` from the resolved token,
  with a test per clause of S2.7's *A loop*, the connectors drawing every part through `Motion::sample` from then on;
  if the loop leaves, no `Motion` is built but for a connector whose design cannot show that the motion unit needs
  no change of its application-facing API (the D6 staging design, 5.6). The engines, `Places`' content,
  `interpolate_color`, Compose's spring, `ComposeShapeMorph` and S4.3's theme change are the motion unit's (*After
  0.8.0*), `interpolate_color` earlier only with a stated loop that interpolates a colour." And the design's second item, plan:1013-1015, gains: "and, for
  motion, the four points the D6 staging design lists (the state kept per element part, drawing from `Drawn`, the
  request for the next frame, an identity and a frame time in every application-facing call), and the contract's
  deferred entries".
- **P-8, M3, plan:1108-1113.** "The file then lists every unit of M3's order with its entries, prefixes and an
  `enabled` flag" gains: ", every unit of depth with the names it holds per entry, and every parking unit with its
  paths (the entries' Depth lines are the catalogue's)". **And step 1, plan:1125-1126.** Old: "every key, declaration
  and state-model combination of the unit on every preset (S17)". New: "every key, declaration and state-model
  combination of the unit at its depth on every preset (S17); what a lane writes beyond it is ahead-work for a unit
  of depth". And a step after the verification, the **comparison with 0.7.0**: per preset, 0.7.0's value of every
  field in the unit's *Today's fields* (and every other entry's mapping into it) beside what the preset at depth
  resolves for its target, in both schemes; each difference is classed by the D6 staging design's test (1.7): equal
  in the model's form; a sourced look the facts hold under a deferred name of the entry's own units (a variant, a
  state), which comes to depth; a value with no source, or from a source the preset's reference rule does not
  choose for it (R1.3, R1.4, R1.6), corrected and listed beside R19's with the rule; a sourced look that no name at
  depth can hold, or that only a shared unit's name holds (a flavour, a size, a condition), proposed as a loss for
  the maintainer, the shared unit named as where it returns. A changed look of the element with no variant, where
  0.7.0 mapped it to a look that is not the platform's default and the facts hold 0.7.0's look under a variant, is a
  correction of the mapping, listed with the corrections.
- **P-9** (withdrawn, see P-6).
- **P-10, M4, plan:1199-1202.** "no state-model combination of an enabled unit lacking a fact" → "no state-model
  combination of the enabled units' names lacking a fact"; and plan:1170-1175 gains: "a name of a state model, a
  condition of a fact or a staged path in no unit fails as a key in no unit does."
- **P-11, the D6 item, plan:550-558 and :630-633,** are closed with a reference to this design; **D14,
  plan:689-695,** gains: "a unit of depth that adds typed variants adds members to `Variant` likewise; one of
  conditions, flavours, sizes or keyed names adds none."
- **P-12, *How values are checked*, plan:1367.** "Either kind, while open, blocks the release that enables its
  unit" stands; a sentence is added: "A row all of whose pending facts are a later unit's is that unit's."
- **P-13, M3's *Every unit listed*, plan:1113-1115.** "names each staged path's unit as M3 names it (the slice's
  `"token.behavior.scroll" = "scroll_view"` becomes `containers`, there being no `scroll_view` unit)" gains: "and
  every other staged path's parking unit (D6: behaviour, text rendering, text styles, icon contexts, elevation,
  parameters, keyed roles, pointer) or the unit of depth that takes it (right-to-left's `token.icons.mirroring`,
  motion's paths), from which the first unit whose value reads it takes it".
- **P-14, *Spec changes*, plan:1383**, class 2's parenthesis "(a field, a condition, a function, a rule in
  S1–S21)" gains "a *Today's fields* selector or a Depth line of the catalogue (it decides what a release states)".

---

## 7. What would defeat the design

Each case was checked on the slice's facts with revision 3's variants (`python3 defeat_checks.py > out/defeat3.txt`;
`python3 holes.py > out/holes3_given.txt`; `python3 holes3.py > out/holes3.txt`; `python3 gen_bases.py`, its output
in `out/gen/<preset>.txt`).

1. **The deferral changes which fact applies at depth.** It cannot: a fact that names a name not at depth in a
   positive list holds in no combination at depth, and a negative entry naming one is never met there, so dropping
   it changes nothing there. Checked: over the combinations at depth of each preset (196 to 1,152), the number of
   facts that hold there and name a deferred variant, context or state is 0 on all eight (check 3).
2. **A value at depth reads a deferred thing** (a deferred value, a part never drawn at depth, a motion token): none
   on any preset (check 1). The token rule's closure makes this true for tokens and keyed roles by construction.
3. **A state model draws a name at depth in no combination at depth.** Two cases (check 4): `material-3`'s
   `toggle` and `ios-*`'s `tool`. Answered by 1.3's point 8 (`later`) and, for the first, 1.7's proposed loss.
   (Revision 2 named the first alone, though its own `out/defeat2.txt` listed both.)
4. **A 0.7.0 value whose look lives under a deferred name.** Run for `button` (1.7), every mapping in both schemes:
   `adwaita`'s gap, KDE's toolbar font, `ios-13`'s grey fill and `material-3`'s tonal button bring `with_icon`,
   `tool`, `@gray` and `tinted` to depth; the rest are equal, corrections or the one proposed loss. For the 29
   further widgets each unit's batch runs it (P-8).
5. **A given name supplied with the names at depth.** An element in a deferred context, or under a deferred
   condition, state or keyed flag, resolves exactly as without it, because the bundled preset mentions the name
   nowhere (2.3). The look is the platform's for the situation without the name, and not for this one; it is what
   0.7.0 draws, and the contract records it per connector. It would become a false statement only if a document or
   an API called it the platform's look there, which nothing does. Example: `macos-*`'s `window_inactive` and
   `not_main` are the window-activity unit's, so 0.8.0 draws an inactive window's button as an active one, as 0.7.0
   does; `kde-breeze`'s default button keeps its fill in an inactive window instead of revision 1's partial fall
   (128 cases, the check's `cfallback.py`).
6. **A required form is deferred.** A widget whose `requires_variants` names only deferred variants could not be
   drawn at all (the catalogue's examples: iOS's check box only as `selection`, Material's link only as `inline`,
   spec:5633-5634). `button` requires none. **Finding C**: C-1 puts a variant a preset's `requires_variants` lists at
   depth.
7. **The declarations state something false, or one S12 refuses.** They stay whole (2.3), less the typed names the
   model cannot spell; an exclusive group, a recency set or a `flags` list trimmed to the flags at depth stays true
   of them, and a keyed namespace lists only the entries a base states. S12's unused check: `holes3.py`, part B,
   finds 6 unused declarations in revision 2's form and 0 unused recency sets in revision 3's on all eight presets;
   the keyed flags, settings and flavours are 0 there by the form's definition, not by a test (2.3). Not checked: the
   regenerated presets loaded in Rust (no change to the repository; the staging task's gate, 2.7 and P-6b, with its
   positive control), and the declared namespaces part C does not compute.
8. **A mixed look inside one element.** Excluded by the rule: a name is at depth for every key of its entry or for
   none, a condition too (1.3, point 6). What the rule keeps, because 0.7.0 has it, is a difference between entries:
   in an inactive window the title bar and the selection change and nothing else does, until the window-activity
   unit. The two places a 0.7.0 field names a context for another entry are `toolbar`'s `font` and `icon_size_px`
   (catalogue:4729-4731): by the rule the whole button, `tool` included, is then stated in that context for the sets
   at depth, from the containers unit on. On `macos-tahoe` that context's default size is `extra_large`
   (catalogue:1954-1958, U94), so that size is the default in force there. Counted for the slice as deferred (the
   toolbar is not enabled); not priced. Likewise every key of `window` under `window_inactive` is on the path with
   the window-and-chrome unit; not counted, the entry having no facts yet.
9. **A connector draws a deferred asked name as the base without a record.** Possible only where a connector catches
   `Later` and draws the base; the contract test (S-10) fails a table that does not list it.
10. **A field the reader finds missing** (the first check's finding 1, the check2's serious findings 1 and 2).
    Revision 2's hole test, `holes.py`, added a given state or condition to each combination at depth and asked
    whether each key stayed stated; after the trimming no fact at depth mentions such a name, so its 0 was true by
    construction, and it tested no size, no flavour, no base and no declaration (the check2). Revision 3 tests what
    the reader refuses, each part with a control that fails where the form would:
    - **Bases, with the generator's own code** (`python3 gen_bases.py`; it imports `scripts/generate_preset.py`,
      hands its `generate()` the trimmed facts, and lists the fields its `unstated_fields()` reports beyond today's;
      the read facts are transcribed in every form, the baseline too, so the hand-written blocks are judged apart).
      Today's facts: no field unstated on any preset (the baseline). Revision 2's form: button fields at depth with no
      base on four presets, `macos-monterey` 3 (`body.padding`, `icon.icon.size`, `label.text.line_height`),
      `macos-tahoe` 4 (`body.min_width`, `body.padding`, `icon.icon.size`, `label.text.line_height`), `ios-13` 17
      (the four `body.corners`, `body.padding` and twelve leaves of `subtitle.text`), `ios-26` 13 (`body.padding` and
      the twelve `subtitle.text` leaves); none on the other four (a block wholly unstated is listed once,
      `unstated_fields()`'s rule; `out/gen/<preset>.txt`, the `rev2` lines). The check2's correction alone (the sizes
      in force and the default flavour): the same fields on `macos-monterey` and `macos-tahoe`, and the generator's
      own reason, "its variant route takes no size and no fact under it alone gives the value of its elements"
      (`out/gen_sizes_only/`). Revision 3's state model in force: **no field without a base on any of the eight
      presets, and no round-trip mismatch** (`out/gen/<preset>.txt`, the `rev3` lines). `gen_bases.py` restricts
      `covering_bases()` alone; its pending table and round trip read the unrestricted `draft.models`. The third
      check's `staging-check3/gen_inforce.py` gives the restricted models to `declarations()`' result, so that
      `covering_bases()`, `pending_table()` and `round_trip()` all read the state model in force, as 2.3 says the
      generator does; re-run for revision 4 (`python3 -u ../staging-check3/gen_inforce.py <preset>…`, at
      `7430a0a2`), it gives on every preset the same as `gen_bases.py`'s run: no button field at depth unstated, no
      new covering-base gap, no round-trip mismatch, and `[pending]` tables of `adwaita` 0, `kde-breeze` 4,
      `macos-monterey` 93, `macos-tahoe` 76, `ios-13` 220, `ios-26` 226, `windows-11` 54, `material-3` 9 paths (682,
      1.6's listed paths that stay under revision 4's rule; the fix round's rule adds `label_position`'s path on
      five presets, 687, not re-run with this script). The keyed maps left with no
      entry (`colors.<scheme>.keyed` on `adwaita` and `ios-*`, `tokens.icons.sizes.keyed` on all but `kde-breeze`,
      `tokens.params` on `macos-*`) are stated empty by 2.3's rule. The fields the staging takes out of the model (14
      or 15 button fields a preset, the motion leaves and deferred values; 34 to 39 token fields, the staged paths)
      are listed apart as expected (`-v`).
    - **Mentions** (`python3 holes3.py`, part A): the facts at depth that mention a name not at depth for their key,
      by kind (variant, size, flavour, state or keyed flag, condition, context, position, direction, recency). 0 on
      every preset after the trimming; the control, the same facts untrimmed (revision 1's bundle), 1 to 6,773 a kind
      and preset.
    - **Declarations** (`holes3.py`, part B): S12's unused declared names. Revision 2's form 6 (above, 2.3); revision
      3's 0 on all eight presets. That 0 tests the recency sets alone: the script appends to its `rev3` list only for
      a recency set of the main section, and for keyed flags, system settings and flavours the form's definition
      makes it 0 (the third check's should-fix 4; revision 3 said this part was not true by construction, which held
      for the recency sets only). Their real test is S12's own check on the regenerated presets, the staging task's
      load in Rust with its positive control (P-6b); restating S12 in a script would be a second statement that
      could disagree with it.
    - **Keyed declarations** (`holes3.py`, part C, revision 4): the declared keyed entries no base at depth states,
      by S12's rule (a stated base, or a listed path the synthetic route's bases state, 2.3). The same entries on all
      eight presets as revision 3's rule (any fact at depth), `out/holes4.txt`; S12's rule without the listed paths
      would also drop `ios-*`'s `params.opacity.disabled` (the third check's `staging-check3/keyed_bases.py`), which
      the synthetic route keeps.
    - **Given names** (`python3 holes.py`, revision 3's variants): 0 (key, combination, name) holes on all eight
      presets (`out/holes3_given.txt`); its control, `--keep-negatives` (revision 1's form), 10,408: `ios-13` 4,384,
      `ios-26` 5,856, `macos-monterey` 168 (`out/holes3_control.txt`; with revision 2's variants 6,056, of which
      `macos-monterey`'s 168, `out/holes_control.txt`, which revision 2's text called "none elsewhere"). This part is
      the one the check2 found true by construction; of the parts above, the bases and the mentions are tested against a
      control that fails; part B's recency sets are computed (0, with no failing control: revision 2's form has no
      unused recency set either), and its flags, settings and flavours are 0 by definition (above).

    Not checked: the presets loaded in Rust (the staging task's gate); a host context once its host is enabled (the
    toolbar's, at the containers unit), which that unit's gate checks: on `windows-11` 10 button facts carry a
    `not_context` naming the toolbar and every positive toolbar fact names `tool`, now at depth.

---

## 8. What the sources did not settle

1. **The ten values of 1.3, point 1**, are my reading of the catalogue's `button` entry. What settles it: the batch
   that writes C-2 checks each against the entry and the facts (a value at depth must have a fact holding in a
   combination at depth on every preset that does not state it absent). Settled by the fix round: three of the ten
   are at depth (the correction at the head of this note).
2. **Whether `hover` and `focus_visible` on `ios-*` are the iPad's alone.** U112 and U113 say so in words; the iOS
   state models exclude `focus_visible` under `idiom:phone` only, and `hover` nowhere
   (`docs/facts/ios/state-models.toml`, `not_drawn`). If they stay at depth the path grows by 252 pending facts
   (1,628 against 1,376, `--ios-pad-states=keep`; 1,633 against 1,381 with the fix round's values at depth). What settles it: the iOS lane states, from run 41020880059's
   `focus.phone_focus_system` and UIKit's pointer documentation, that the iPhone draws neither, as `not_drawn`
   entries; and splits U112's one `button.body.focus` fact into the iPhone's (absent) and the iPad's (pending).
3. **A keyed namespace whose names are not read yet.** `ios-*`'s elevation ramps (U480) and keyed icon contexts
   (U650) are pending as whole maps; `windows-11`'s U511 and U681 are settled by register-rows (U681: `{}`). The count
   treats the pending ones as staged. What settles it: a source read of the names alone, or the rows stay on the path.
4. **The core's token reads** (1.5, point 3) and the governing reads (point 4) are from the specification. What
   settles it: a code search in the staging task.
5. **Which staged token paths M3a's units read.** Not computable before their facts exist; the catalogue names some
   (`token.caret` by `input`, `token.behavior.hover_time` by `tooltip` on `windows-11`, catalogue:1286-1289). What
   settles it: each unit's closure, run by `generate_preset.py` once it implements the rule.
6. **Whether Apple's two motions can be measured, and whether a third is needed.** Nothing in the sources shows a
   method that works. A third Apple measurement, U158's colour space, joins them if a 0.8.0 loop or presentation
   interpolates a colour (5.2). What settles it: the trial of 5.5, and the Material lane's spinner facts.
7. **How much the tests and fixtures change** when `Variant` loses 29 members. I counted lines that hold a removed
   variant's name as a word (`grep -cE 'flat|pill|circular|icon_only|destructive|"menu"|warning|glass|help'`, revision
   3's list without `tinted`: `native-theme/tests/facts_gate.rs` 44, `src/query/tests.rs` 38, `src/toml/tests.rs` 14,
   `src/widgets/tests.rs` 4, `src/validate/*` 99 between them), an upper bound that also catches other uses of those
   words, not the edits.
   What settles it: the staging task's first compile.
8. **The price of the path that stays** (1,381 pending facts, 687 listed paths, the fix round's figures, the two motion values behind their
   trial, the toolbar context of 7.8). The package's cost per measured fact (3.43 k and 11.3 k of usage, brief,
   decision 2) would give about 2.7 M for macOS (778 facts) and 6.1 M for iOS (536) if a pending fact cost what a
   measured one did, which the package does not claim. What settles it: the first Apple round under the rule.
9. **The maintainer's veto list** (each to accept or overrule):
   - **The proposed loss:** `material-3`'s checked text toggle (1.7; R-3; decision F). Alternatives: `icon_only` at
     depth (+291 pending facts, 1,672 against 1,381, a different look), or the flavours unit in 0.8.0 (0.7.0's
     colours under `expressive` only). Caveat: Compose 1.5's stable Expressive `ToggleButton` makes the flavours unit
     the place it returns.
   - **The two sourced looks kept at depth** (decision E): `material-3`'s tonal button under `tinted` (+158 pending
     facts, 146 of them on `macos-tahoe`) and `ios-13`'s grey fill under `@gray` (+12, 4 on `ios-13` and 8 on
     `ios-26`).
   - **The default button's look, a correction of a mapping** (1.3 after C-3e, 1.7; R-3, P-3): `material-3`'s and
     `ios-13`'s button with no variant draws the platform's default button (Compose's `Button`, the filled one,
     `Button.kt:77-88`, against `FilledTonalButton`, `:439-447`; UIKit's system button, `UIButton.h:22-23`), where
     0.7.0 drew Material's tonal button and WebKit's HTML button; 0.7.0's look stays under `tinted` and `@gray`. A
     visible change for every application that drew 0.7.0's plain button.
   - **Corrections of values with no source** (1.7, class 3), listed with R19's invented light variants (R-3): the
     `macos-*` and `ios-13` hover fills; `kde-breeze`'s inherited primary (C-3b); `adwaita`'s `#2e3436`, gap and
     minimum width; `kde-breeze`'s minimum height; `macos-monterey`'s pressed fill; the `macos-*` gap and minimum
     width; `ios-13`'s sizes, gap and pressed and disabled fills; `windows-11`'s disabled fill and minimum sizes.
   - **Corrections by the preset's reference rule** (1.7, class 4): by R1.6, `material-3`'s disabled container (0.12
     → 0.1, the older `v0_192` token set), vertical padding (10 → 8, the drawn height of one line unchanged at the
     40 px minimum) and minimum width (64 → 58), the last two material-web's component where Compose has the button;
     by R1.3 and R1.4, `ios-13`'s pressed label (WebKit's opaque blue → UIKit's 75 %).
   - **Motion:** the trial (5.5) puts two Apple motion measurements on 0.8.0's path, against outcome 3's stated
     reason; a motion it does not measure leaves 0.8.0 on every preset, a loss (R-3). Its cap, two harness rounds per
     Apple harness, is decision G's budget, not a figure the slice's evidence gives: the slice's Apple lanes needed
     several correcting rounds each (5.5), so the fallback, both motions lost on all eight presets, is likely, and
     the maintainer accepts or overrules the cap with that expectation.
   - **`tool` at depth now:** +197 pending facts (1.3; +198 with the fix round's values at depth, on revision 2's
     list: 1,211 against 1,013), against the loss of KDE's toolbar font at the containers
     unit.

**Not covered:** the 29 further widgets' Depth lines beyond their catalogue part (`todays_selectors.py`) and their
comparison with 0.7.0; the connectors' design itself; the load cost (outcome 1), which the smaller presets lower by an
amount not measured; the generator's and the gate's code, read only where cited; whether WinUI's `ProgressRing` can be
stated as a `Loop` from its source, which 5.5 (i) assumes for `windows-11` (and GNOME's spinner, Breeze's busy
indicator and Compose's indicator likewise, the check2's *Not covered*); the cost of `contrast_high` across M3a's
29 widgets; the keyed namespaces `holes3.py` does not compute (2.3). No cargo build or test was run and no file of the
repository was changed; `generate_preset.py` ran only imported, its text held in memory (`gen_bases.py`). One command
of revision 1's author was refused by the outside-project hook (a Python heredoc holding a `~` character); it was
rewritten as `staging/names.py`. No command of revisions 2 and 3 was refused.

---

## 9. The commands

All from `/home/tibi/.claude/projects/-home-tibi-Rust-native-theme/scratch-keep/d6/staging/`, with `python3` alone.
Each script's docstring states its rule. `depth_count.py` defaults to revision 3; `--revision 2` reprints revision
2's `out/final2.md` exactly (checked by `diff`), `--revision 1` revision 1's figures. Every count below was taken at
`7430a0a2`; `depth_count.py` runs in about 8 s, `holes.py` about two minutes, `gen_bases.py` one to five minutes a
preset (three runs of the generator).

| Command | Gives |
|---|---|
| `python3 load.py` | the cache of `docs/facts/<platform>/*.toml` at the worktree's HEAD: 74,029 facts, 45,766 pending |
| `python3 depth_count.py --json out/final3.json --rows --listed > out/final3.md` | section 1.6's two tables; per register row and preset the pending facts at depth and deferred; every listed path with whether it stays and why not |
| `python3 depth_count.py --variants …` (and `--states all`, `--ios-pad-states keep`, `--revision 1`) | the alternatives of 1.2, 1.3, 1.6 and 8.2 |
| `python3 rows_summary.py out/final3.json > out/rows3.md` | section 4's tables and lists; the listed paths that stay by class; why the others leave |
| `python3 compare070.py > out/compare070.md` | revision 2's comparison (fifteen fields, light) |
| `python3 compare070_2.py > out/compare070_2.md`; `--with material-3=tinted --with ios-13=@gray > out/compare070_3.md` | 1.7's comparison, every mapping, both schemes, 0.7.0's comments beside; the second with C-3d and C-3e |
| `python3 search070.py > out/search070.txt` | 1.7's test: each corrected 0.7.0 value searched for under any `when` |
| `python3 gen_bases.py [--preset ID] [-v]` (`out/gen/<preset>.txt`; the check2's correction alone in `out/gen_sizes_only/`) | 7.10's bases: `generate_preset.py`'s own `generate()` and `unstated_fields()` on today's facts, revision 2's form and revision 3's; the hand-written blocks that lose their place |
| `python3 holes3.py > out/holes3.txt` | 7.10's mentions, S12's unused declarations (2.3), the keyed declarations no base states (revision 3's run) |
| `python3 holes3.py > out/holes4.txt` | the same, revised for revision 4: part B's note that its `rev3` column tests the recency sets alone, part C by S12's rule (a stated base or a listed path) with the entries revision 3's rule judges differently (none) |
| `python3 depth_count.py --variants=primary,tinted,toggle,tool,with_icon`, `--variants=@gray,primary,toggle,tool,with_icon` | 1.6's per-preset recount of each variant alone (revision 4): `ios-13` 323 and `ios-26` 337 for each, `macos-tahoe` 546 with `tinted` |
| `python3 -u ../staging-check3/gen_inforce.py <preset>…` | the third check's run of the generator with the state model in force given to `declarations()`' result, so that `covering_bases()`, `pending_table()` and `round_trip()` all read it (2.3, 7.10); re-run for revision 4 |
| `python3 toolbar_check.py`, `--tool` (`out/toolbar.txt`) | the reading of 1.3's C-3c: which facts a plain and a tool button take in `toolbar.bar` |
| `python3 todays_selectors.py > out/todays_selectors.txt` | every *Today's fields* selector, per entry and target (1.1, 3.2) |
| `python3 holes.py > out/holes3_given.txt`, `--keep-negatives > out/holes3_control.txt` | 7.10's given-name test and its positive control (revision 2's outputs: `out/holes2.txt`, `out/holes_control.txt`) |
| `python3 later_units.py > out/later_units3.md` | 3.1's assignment of every fact off the path to a unit |
| `python3 names.py > out/names.txt` | which state models declare each variant; the facts naming each name (sections 2.1, 3.3) |
| `python3 token_paths.py > out/token_paths2.md` | which token paths and keyed entries are at depth or staged (1.5) |
| `python3 depth_pending.py <preset>` | a preset's pending facts at depth by row and selector (section 4, U695) |
| `python3 defeat_checks.py > out/defeat3.txt` | section 7's checks 1 to 5 |
| `python3 motion_count.py > out/motion.md` | the motion facts of 5.1, per group and directory |
| `python3 ../staging-check/cmismatch4.py`, `python3 ../staging-check/choles.py -v` | the check's findings 2 and 1, recomputed (2.5, 2.3) |
| `git grep -c entrance v0.7.0 -- native-theme/src/presets`; `git show v0.7.0:native-theme/src/icons.rs \| sed -n 732,739p`; `git grep -i key_repeat v0.7.0 -- native-theme/src/model native-theme/src/presets`; `git show v0.7.0:native-theme/src/presets/<id>.toml` | sections 1.5, 1.7 and 5.1 |

---

## 10. Findings not taken

### 10.0 The third check (revision 4)

Every finding of `staging-design-check3.md` (6 should-fix, 9 minor) was verified at its evidence before anything
changed: should-fix 1 at the pinned Compose and UIKit sources and the two facts files' bases; should-fix 2 at
rationale:166-175 and :189-215 and the Material facts' material-web citations; should-fix 3 at
`docs/facts/material/state-models.toml:56`, `:283` and `:289`; should-fix 4 in `holes3.py`'s code and at
`Uses::map_key` (`validate/checks.rs:2803-2817`); should-fix 5 in the ledger's round references and its *Spikes and
drift runs*; should-fix 6 at the generator's `declarations()`, `analyse()` and `round_trip()` (whose line numbers
are `:845`, `:847`, `:859`, `:2483` at `7430a0a2`; the check's `:858` and `:2471` are `:859` and `:2483`); minor 7 by
recount (`depth_count.py`, each variant alone); minor 8 in `gen_bases.py`'s code and by re-running the check's
`gen_inforce.py`; minor 9 at catalogue:1832-1847; minor 10 at `ProblemKind::Undeclared`'s message
(`native-theme/src/error.rs`); minor 11 at `validate/combos/mod.rs:1838-1846`; minor 12 at spec:7338-7339; minor 13
at `native-theme/src/macos.rs:15-18`; minor 14 at C-3d's wording; minor 15 at `out/gen/<preset>.txt` and the Material
facts (the minimum height 40 px, the label's line height 20 px). All hold, and all are applied. What is taken
differently from the check's proposal:

- **Should-fix 4, part C "a base states"**: computed as S12 counts, which is a stated base *or* a listed path, since
  `check_preset` hands `unused` the synthetic route's layers, whose bases state a listed path's keyed names
  (spec:5997, ADD-A133). By bases alone `ios-*`'s `params.opacity.disabled` (pending U486) would be dropped, which
  S12 does not refuse today; with the listed paths the entries are revision 3's on all eight presets
  (`out/holes4.txt`).
- **Should-fix 6, "P-6b with the staging task's text"**: written as an unnumbered entry before task 0, so that the
  plan's task numbers and every "M2.1" reference stay as they are; its done-criterion is the eight `[pending]`
  tables, which the generator's own code reproduces (7.10).
- **Minor 11**: the sentence is added to S-3 as proposed; that the load stays harmless is not verified here (no
  cargo), and the staging task's load of each regenerated preset confirms it.
- **The table at the top**: the brief asked for a "Revision 4" row; it is a *Revision 4* section with its own table,
  as revisions 2 and 3 have.

### 10.1 The second check (revision 3)

Every finding of `staging-design-check2.md` was verified at its evidence before anything changed: serious 1 with
`generate_preset.py`'s own code (`gen_bases.py`), serious 2 at `checks::unused` and with `holes3.py`, serious 3 and
should-fix 4 with `compare070_2.py`, minor 10 by recount (`later_units.py`), minor 14 at `kde/mod.rs:475-514`. All
hold. What is taken differently from the check's proposal:

- **Serious 1, the correction "the sizes an element takes are the default in force alone"**: taken and widened. The
  sizes alone leave `macos-*` without bases, `route`'s `sizes_by_variant` entry taking no size (2.3, 7.10); the form
  reads the whole state model in force (sizes, flavours, variants).
- **Serious 1's counts** (the check's `bases.py`, "an approximation of covering_bases"): replaced by the generator's
  own. They differ: on `macos-tahoe` the generator reports `body.min_width`, `body.padding`, `icon.icon.size` and
  `label.text.line_height`, not the corners, `min_height` or the font size; on `ios-*` it reports the `subtitle`
  part's text leaves, which the approximation did not see, and not the font size (7.10).
- **Serious 2**: taken, with a third case the check did not list, `kde-breeze`'s system setting `@single_click`
  (`holes3.py`, part B), and the keyed namespaces S12 checks too (2.3).
- **Serious 3**: taken by decision E, which keeps both looks at depth rather than listing them as losses.
- **Minor 11** ("drop `tinted` from the example"): moot; `tinted` is at depth by decision E, so the example now
  states it truly.

Beyond the check: `ios-*`'s `tool` is a typed variant at depth drawn nowhere at depth, so it is declared `later`
(1.3, point 8; revision 2's 7.3 named `material-3`'s `toggle` alone); and two hand-written blocks lose their place
under the trimming (2.5, *(d)*).

### 10.2 The orchestrator's decisions not applied as worded

- **Decision E, "`material-3`'s plain button keeps 0.7.0's filled tonal look" (and `ios-13`'s grey fill).** What
  bringing `tinted` and `@gray` to depth achieves is that the model keeps both looks at depth, so a user loses nothing
  of 0.7.0's model (D13's wording, "users lose nothing of 0.7.0's model"). It does not make the plain button, an
  element with no variant, look as in 0.7.0: on `material-3` that element draws Material's base button, the filled
  one (`md.comp.button`, `docs/facts/material/button.toml:36-44`), and on `ios-*` UIKit's system button with no
  configuration, which draws no fill (1.7). Taken literally, the decision would need either each connector to ask for
  `tinted` on `material-3` and `@gray` on `ios-*` for its default button, a per-preset choice in the connector
  against "connectors follow the theme's data", or a new theme datum naming a widget's default variants, a model
  mechanism with no source. Applied: the looks at depth (C-3d, C-3e), and the catalogue's words say what an element
  with no variant draws; a connector that drew 0.7.0's button through 0.7.0's mapping asks for the variant (its 0.7.0
  mapping, S-10's stand-in record). The pinned sources settle the default element's look, so it is no further
  decision (revision 4, the third check's should-fix 1): the platform's own default button is Compose's `Button`, the
  filled one ("Filled buttons are high-emphasis buttons", `androidx@e4d933c9 …/material3/Button.kt:77-88`; the tonal
  one is the separate `FilledTonalButton`, "medium-emphasis", `:439-447`), and UIKit's system button
  (`UIButtonTypeSystem`, "standard system button", `xcode@26.6 UIKit/UIButton.h:22-23`; `UIButtonConfiguration` has
  no default, `UIButtonConfiguration.h:78-94`). 0.7.0 mapped its plain button to neither, so the change is a
  correction of a mapping, listed with the corrections (1.3, 1.7, R-3, P-3) and on the veto list as one (8.9).
- **Decision G, "on failure, the motions leave 0.8.0 on all presets".** Applied per motion: each of the two is one
  field, and the rule the decision cites ("one rule for every field") decides each field by itself, so a motion the
  trial measures stays even if the other does not (5.5). If the decision meant both together, the only change is
  that the measured one leaves too.

### 10.3 The first check (revision 2)

Every finding of the check and every disagreement of register-rows was verified at its evidence (the scripts named
in each place). All hold. What is not taken is a part of a proposed correction, with the reason:

- **Finding 1, the query half of its correction** ("the query drops such a flag or condition before the exclusive
  groups' reduction"). Not taken: with the bundled preset mentioning no name outside the enabled units' (the
  generator half, taken), the query needs no run-time list of deferred names to give the same result, and dropping
  the condition in the query would silence the macOS reader's rule under `contrast_increased`
  (`native-theme/src/macos.rs:359-372`), which carries 0.7.0's Increase Contrast colours (2.3, the table of options).
- **Finding 3, "the KDE reader's speed factor, which returns with the first of the two motion values"**, read as a
  cut of the reader. Not taken: the speed factor is at depth now (clause 5, and the loop 0.8.0 runs reads it), so
  the reader stays as built, and U515 stays on the path, settled by register-rows (5.2).
- **Finding 10, "or count the rows" for `color_space`.** The other branch is taken: `color_space` is staged with the
  engines, and returns, with U158, only if a stated 0.8.0 loop or presentation interpolates a colour (5.2).
- **Finding 5 and *Decisions I confirm*, `ios-*`'s `hover` "a loss to record".** Recorded as a correction, not a
  loss: 0.7.0's `ios-13` value has no source and its next line states "iOS has no hover state"
  (`v0.7.0:native-theme/src/presets/ios-13.toml:138-139`), and the iPhone draws none (U113). It stays on the
  maintainer's veto list as a visible change (8.9). Likewise `macos-*`'s hover (no `hover` in the macOS state
  models) and `material-3`'s plain button (Material's base is the filled one).
- **Finding 15 (e)**, "`staging/out/` does not hold [`out/token_paths.md`]". Not taken: the file is there (7,981
  bytes, written 2026-11-10 12:45, before the check), and revision 2 adds `out/token_paths2.md`.
- **Finding 15 (i)**, `d6-outcome.md`'s "21 of 29 macOS" motion facts: true (21 of 27 at `7430a0a2`), but that file is
  not this note's to edit; the orchestrator corrects it.
- **Register-rows-check2, finding 5**, "`tool` … is stated as a necessity … a prediction" for `windows-11`. Taken in
  its limit (for `windows-11` the containers unit's facts decide), but `tool` comes to depth now on `kde-breeze`'s
  evidence, which is not a prediction: the KDE reader's `toolBarFont`, 0.7.0's `toolbar.font`, is stated only under
  `variant tool` (C-3c).
