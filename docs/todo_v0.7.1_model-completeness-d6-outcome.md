# D6: the outcome

2026-11-10. The maintainer asked whether the fifteen decisions of `d6-decisions.md` could be answered from facts and
logic. They can: each follows from a decision the maintainer already made (D13, D14, D10, R19, the project's rules)
and from the slice's measurements. This file records each outcome with what settles it. Every one is open to the
maintainer's veto; the three marked **changes an earlier decision** change a rule the rationale records.

The brief (`d6-decisions.md`) holds the options and their arguments; this file does not repeat them.

| # | Outcome | What settles it |
|---|---|---|
| 1 | A design task now: (1) a profile of the load, (2) a design note choosing the load path, checked as a design document, with a budget proposed from the profile, (3) the repair before M3a's first widget unit, the timing probe then a release-mode check of each unit's gate. | The plan's own rule: "if the slice already misses it, a task before M3a costs less than one after it". The slice loads in 0.3 to 40 s and up to 1.4 GiB where 0.7.0 takes under 1 ms, so the "if" is answered whatever number the budget is. |
| 2 | **Changes an earlier decision (R21, "a unit enters whole").** 0.8.0 states each widget, role and token at the depth 0.7.0's model distinguishes; the further variant sets, sizes, contexts and settings are named later units, deferred as S6.1 defers placement, and impossible to ask for until their unit. The facts already written stay in `docs/facts`. Sample runs with a composition decision (the brief's C) are the method of those later units. `ios-26` stays. | D13: "0.8.0 ships when the new model covers what 0.7.0 covers", taken for time ("about 3–4 weeks against 5–7"). Measuring every combination is priced at 814 to 2,958 M of usage, 3 to 11 weeks, for the facts alone, so it defeats D13's reason; 99.95 % of the pending button facts lie beyond 0.7.0's depth. D13's text keeps `ios-26` and keeps the rigor inside each unit, which the deferral does not touch. |
| 3 | **Changes an earlier decision (R21, "every capability but placement").** 0.8.0 holds the two motions 0.7.0 has (the dialog's entrance, the animated busy icons); state-change transitions (the runtime's engines, their use in the connectors, the Apple measurements) are a later unit. The model's types and the written facts stay. The connectors' design must show that the later unit needs no redesign of their API; if it cannot, the runtime's API is built now and its engines later. | D13's criterion, and the plan's own two reasons for deferring placement, which hold for motion word for word: 0.7.0 does not have it, and Apple states no timings, so each Apple engine rests on a measurement no trial has made (21 of 29 macOS and 43 of 48 iOS motion facts pending). |
| 4 | The readers unit moves to the start of M3b; M3a keeps each unit's own readers step. | The comparison of all four readers with 0.7.0's found no read whose target lies outside 0.8.0's other units. |
| 5 | **Changes an earlier decision (R25.17, the status roles absent on `macos-*`).** One rule: a typed role or slot that a portable application needs and the platform names no counterpart for is native-theme's mapping: labelled, outside the gate's platform comparison, never absent, never a stated fact. On `macos-*` and `ios-*` the status roles refer to the keyed system colours as 0.7.0 drew them (corrected below: the rule rests on a witness, not on necessity; each status paired by its read, U757–U760); the text slots' mappings get the same label. The pairings are listed for the maintainer's veto. Closes U456. | Necessity, not preference: iced, gpui-component and egui hold these as plain colours that cannot be left empty, so with the role absent a connector puts either the toolkit's default (forbidden: connectors follow the theme's data) or a pairing of its own, unlabelled and different per connector. A pairing exists either way; the rule puts it in one labelled place. It invents no colour (the colours are the platform's system colours), and R19 already admits labelled picks. |
| 6 | The declarations stay. The connectors' contract lists, per preset, the entry a connector draws its toolkit's widget from where the preset declares the widget unavailable: a stand-in entry, or, where the catalogue names none, the toolkit's own geometry with every colour and font from the theme's typed roles (never an unthemed toolkit colour). The contract's test checks the list is complete. | The declarations are true. Moving `bottom_tab_bar` and `group_box` into 0.8.0 adds two units against D13 and still leaves the rule missing; no rule leaves each connector to decide, against "connectors follow the theme's data". Where the platform has no such widget there is no platform geometry to follow, so only the colours and fonts can be the theme's. |
| 7 | Decided with the palettes' design, M2's fourth task, by R19: a palette must not give a mixed look. Over a preset whose widgets read keyed colours it reaches them by a stated mapping method (each keyed role mapped once to the scheme's vocabulary, the mapping reviewed), or it is not offered over that preset. | Nothing before M2's fourth task depends on it, and R19 already rejects the mix of two schemes; a method removes the per-pick veto the brief names as the one preference. |
| 8 | `docs/platform-facts.md` is not committed: CI and each release generate it, the gates run the generator and its self-test, and `check_facts.py` enforces a split of a facts file by key prefix past 40 MB. The local tools that cite its lines read the facts or a generated copy. | The file is generated from committed facts, is 47.8 MiB against Git's warning at 50 MiB and GitHub's block at 100 MiB, and grows with every measured fact; GitHub does not render a file of that size, so the committed copy has no reader. |
| 9 | The plan's three numbers: 3 h a shard on CI; 1 h and 12 GB for a local preset step. Shards are added as units land. The release run stays one unsharded local run (queue #213): timed at M1b's close at 459 min on 8 threads, peak 7.65 GB. Reviewed with the first widget unit's measured times. | The figures are measured and inside the limits (longest shard 1.99 h); the one case that passes them, 30 runner hours a push, arises only when every combination is measured, which outcome 2 does not do. |
| 10 | No cap is set now. The work runs to M2's close as the maintainer instructed; the report there gives the cost of M2, the gate's time and the load cost, and M3a's approval is asked with the first real unit costs. No lane writes facts ahead for M3b's units meanwhile. | The maintainer's instruction ("do everything until M2 is finished") already sets the next stop before the brief's proposed checkpoint. A cap from an estimate last wrong by a factor of 12.8 would not bind or would trip at once. |
| 11 | D10 names four classes: the clean room for every LGPL and GPL reference; an own port of an Apache-2.0 or MIT reference under a file-level licence with its notice; the manifest's expression as it stands, `(MIT OR Apache-2.0 OR 0BSD) AND Apache-2.0 AND MIT`. | The maintainer accepted "AND Apache-2.0" for a ported file on 2026-10-29, which already gave up a crate usable with no notice to carry; MIT's obligation (keep the notice) is of the same kind and no wider. The clean room carries no code of the reference, so it serves a GPL reference exactly as an LGPL one. Stated as a risk judgement, as D10 is, not a legal conclusion. |
| 12 | The five rationale amendments stand: the iOS tint as a measured derivation over the accent (U706's second-tint run is scheduled); a palette suspended, not refused, under a followed scheme it lacks; the ten bundled palette variants 0.7.0 ships; `#[non_exhaustive]` error enums; no font DPI read where the toolkit's scale carries it (the KDE reader's `Xft.dpi` read goes). | Each removes a defect that is a fact: `set_accent` leaving 555 tinted button facts system blue; one state being an error or a suspension by the order of two calls; D13's criterion; one new validation problem making a patch release breaking; the setting applied twice. |
| 13 | The plan's ADD-A146 amendments, the generated presets and the audit rate (5 %, at least 10 a unit, every measured fact recomputed) stand. | Corrections of fact and of orders that could not run; 97.8 % of the presets' lines are generated with `--check` in every gate; the audits found 0 wrong of 430 and 1 of 74, and the recomputation found the larger defects. |
| 14 | The orchestrator settles the register's fifteen choices of model form from the sources and the project's rules, the eight that hold listed paths first, and lists the outcomes in one table for the maintainer's veto. Six rows follow outcomes 2, 3 and 5; four wait on a specification correction. U473 (the iOS key repeat) is examined by D13's criterion first: if 0.7.0's model holds no such value on iOS it is a later unit's; only otherwise is it the maintainer's (it needs a hardware keyboard). | The maintainer's standing rule: settle by logic first, ask only real preferences. |
| 15 | A full pass over all documents after the batch that applies these outcomes; its fixes are checked until a check finds nothing serious; a further pass reads the documents the fixes changed and what the findings implicate, not the whole 5.8 MB again. Then each unit's batch is checked as it is written. | The maintainer's document standard, with the instruction to be deliberate with repeated full passes (a full pass with its fixes costs about 16 M of usage). A pass before the batch would read texts the batch rewrites. |

## What stays the maintainer's

Nothing that blocks the work. To come, each with its facts when they exist:

- the load budget's numbers, proposed from the profile (outcome 1);
- the status pairings and the fifteen register outcomes, as veto lists (outcomes 5, 14);
- the caps for M3a, at M2's close (outcome 10);
- U473, if 0.7.0 turns out to hold the value (outcome 14).

## Corrections of the batch's fix rounds (2026-11-12)

The batch that applied these outcomes (ADD-A147) was checked twice, its fix rounds six times more, and the whole
batch once more (F1); the
orchestrator's decisions on the checks correct this file's outcomes as follows, each open to the maintainer's veto
(the veto list, `docs/todo_v0.7.1_model-completeness-d6-veto-list.md`):

- Outcome 2: `button`'s `label_position`, `repeat_delay` and `repeat_interval` are at depth where a button with no
  later name reads them (ADD-A148); the staging design's figures are recounted (ledger queue #258; veto list item 1),
  and the staging task measured 1,387 pending facts and 696 listed paths on 0.8.0's path (2026-11-13; the plan's
  *The depth 0.8.0 states* gives why they exceed the design's count: fifteen keyed text styles that text-role slots
  at depth reference are at depth).
- Outcome 3: 21 of the 27 macOS motion facts pending at `7430a0a2`, by the D6 staging design's count, not 21 of 29.
- Outcome 5: the rule rests on a witness, not on necessity: a typed role no source of the platform states is
  native-theme's labelled mapping where the platform draws its meaning in a colour of its own, documented or
  measured; a text slot is filled by a style or family the platform names ("never absent" and the necessity reason
  are withdrawn; the specification's S3; veto list item 3). Where the platform draws the meaning in no colour
  (`material-3`'s warning, success and info) the role is absent and the connector's contract records the toolkit's
  own colour there, an exception to following the theme's data (veto list item 56). The status text colours are
  the colour the platform is measured drawing over the status fill (ledger U756, U755); on `adwaita` the info
  role's fill and text colour refer to the accent's (ledger queue #278). On `macos-*` and `ios-*` each status, its
  standalone colour too (the specification's S3), waits on its read (U757–U760), its values pending meanwhile and
  absent where a read finds no witness, a loss of 0.7.0's status colour there (veto list item 57); `destructive`
  stays absent, no committed source or measurement showing it (U761 retired); the set text colours refer to the
  status's colour, its fill where no text colour is paired, on every preset.
- Outcome 7: a palette is not offered over a preset whose widget colours are measured literals that follow no role
  (`macos-*`, `ios-*`), a recorded limit (veto list item 49).
- Outcome 8: 40 MiB (41,943,040 bytes), and GitHub's warning at 50 MiB, not Git's (S17).
- Outcome 14: Qt Quick's line position (U753) takes the non-breaking form, a field on `Decoration`; QPainter's wavy
  and strike lines are a row of their own, U754 (veto list item 27).

## Corrections of the document pass (2026-11-12)

The full document pass after ADD-A147 (outcome 15; eight reviewers) and the orchestrator's decision per finding,
applied as ADD-A148 (ledger queue #288), correct this file's outcomes as follows, each open to the maintainer's veto:

- Outcome 2: `full_keyboard_access` is at depth model-wide, as `contrast_high` is, so a macOS widget draws its focus
  ring under keyboard navigation (veto list item 62); a part's `shown` stays at depth with its rest meaning (item
  61); a 0.7.0 `defaults` colour that maps to a role brings no state flag to an entry's depth, the connectors drawing
  those states from the typed roles through their deferred entries (item 64).
- Outcome 3: each of the two motions is kept only if the Apple motion trial measures it on macOS 15 and 26 and iOS
  18 and 26 (the plan's M2 task 1; veto list items 2, 6, 24); every widget's `loop` but the spinner's is the motion
  unit's, and the press ripple stays (item 61).
- Outcome 5: each field needs its own witness: a witness drawn as text or a mark alone pairs the `standalone` alone,
  the `fill` and `foreground` absent, each field absent where its row's read finds no witness for it (the
  specification's S3, U757–U760; veto list item 57). The HIG's Color, Feedback and Gauges pages, pinned and read
  on 2026-11-12, name no system colour for error, warning, success or information on any of the four Apple presets;
  the reads go on with the lanes' survey of measured elements (U757–U760; item 57).
- Outcome 6: the stand-in entry for a widget a preset declares unavailable reads no field of that widget there: the
  toolkit's own geometry, every colour and font from the theme's typed roles (veto list item 48). Where S15's
  rendering is absent, wherever `render_system_control` returns `None`, the connector draws its toolkit's own mark, arrows and disclosure triangle, each colour from
  the theme (item 60).
- Outcome 7 **changes an earlier decision** (R19's "a palette on every platform", the maintainer's, 2026-10-28): a
  preset whose widget colours are measured literals that follow no role declares that it takes no palette
  (`macos-*`, `ios-*`); a mapping states every colour role a value of the preset reads unless that role follows the
  mapping, a reference or a derivation over roles the mapping states or that so follow, or, through references
  alone, an input it states (a literal, and a call that reads an input itself, follow nothing: `material-3`'s ladder
  is mapped as `windows-11`'s literal keyed roles are); the bundled palettes' keyed tables for `windows-11` and `material-3` are written once, at M4, the bundled palettes offered over
  `adwaita` and `kde-breeze` until then (veto list items 49, 63).
- Outcome 10: M3a's approval is asked with M2's measured costs (the staging task, task 0's rounds, the connectors'
  port), not with the first real unit's (veto list item 51).
- Outcome 12: nothing changes; its two reversed decisions, the iOS tint and the palette suspended under a followed
  scheme, are veto list items of their own (65, 66).
- Outcome 14: U473 is not the maintainer's: 0.7.0's model holds no key repeat on iOS, so it is a later unit's (the
  parking unit *behaviour*), and the line "U473, if 0.7.0 turns out to hold the value" under *What stays the
  maintainer's* is withdrawn (veto list item 52).
