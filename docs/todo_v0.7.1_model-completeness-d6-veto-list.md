# D6: what you may veto

2026-11-12. Everything checkpoint D6 decided that you may veto, in one list. **Every item is already applied as the
work's basis and stays so unless you veto it**; nothing here blocks the work. Answer by item number: "veto 12", or
"veto 12: instead …"; an item you do not name stands. Sources, all in the repository:
`docs/todo_v0.7.1_model-completeness-d6-outcome.md` ("outcome N"), `-d6-register.md` ("register"),
`-d6-staging.md` ("staging"), `-d6-load.md` ("load"); `d6-decisions.md` ("brief", which holds the alternatives)
and the batch's checks and decisions (`batch-check/`) stay in the project's working files (`scratch-keep/d6/`).

---

## A. Changes to an earlier recorded decision

**1. Each widget at 0.7.0's depth; the rest in later units (outcome 2).**
- *Decided:* 0.8.0 states each widget, colour role and token (on every *preset*, the data describing one platform's
  look) only in the combinations 0.7.0's model told apart
  (its *depth*; for `button`: the variants `primary`, `toggle`, `with_icon`, `tool`, `tinted` and, on `ios-*`,
  `@gray`; seven states; the default size; no host context). Further variants, sizes, contexts and settings become
  named later *units* (a unit is a group of widgets or values that enters the model in one commit, on every preset
  at once) and cannot be asked for until then. *Facts* already written stay (a fact is one platform value for one
  key under one combination, with its proof; a *pending* fact has its combination written down, not its value).
  `ios-26` stays. This changes R21's "a unit enters whole". Result: 1,381 of the slice's 45,766 pending facts
  (3.0 %) and 687 of its 1,508 listed paths (the paths that keep a preset from loading) stay on 0.8.0's path. These
  figures count `button`'s `label_position`, `repeat_delay` and `repeat_interval` at depth wherever a preset states
  them with no later name (the fix round's recount, ledger queue #258; with the three deferred they were 1,376 and
  682).
- *Why:* D13 ("0.8.0 ships when the new model covers what 0.7.0 covers", taken for time: about 3–4 weeks against
  5–7). Measuring every combination was priced at 814 to 2,958 M of usage, 3 to 11 weeks for the facts alone;
  99.95 % of the pending button facts lie beyond 0.7.0's depth.
- *Veto brings:* every combination measured before 0.8.0 (R21 unchanged): 3 to 11 weeks for the priced parts
  alone, and a row with no route (glass, the iPad's focus look) can block the release. Under that option the brief
  would also move `ios-26` to the first later breaking release.
- *Source:* outcome 2; staging §0, §1.2–1.3, §6.1 R-1, R-2, R-4, R-5, R-6; brief §2 decision 2.

**2. State-change transitions after 0.8.0 (outcome 3).**
- *Decided:* 0.8.0 keeps the two motions 0.7.0 has (the dialog's entrance and the animated busy indicator, with
  the speed factor). The animation between two states of a widget (the runtime's engines, their use in the
  connectors, the Apple measurements, the theme-change token) is a later motion unit. Until then a state changes at
  once, as in 0.7.0. The model's types and the written facts stay. The connectors' design must show that the later
  unit needs no change to their API; if it cannot, the runtime's API is built now and its engines later. This
  changes R21's "every capability but placement".
- *Why:* D13, and the plan's own two reasons for deferring placement, which hold for motion word for word: 0.7.0
  does not have it, and Apple states no timings.
- *Veto brings:* motion in 0.8.0 as planned. M2's runtime task builds every engine, after an Apple motion trial.
- *Source:* outcome 3; staging §5, §6.1 R-1, R-4; brief §2 decision 3.
- *The sources disagree:* (1) staging (the orchestrator's decision G) keeps the two motions in 0.8.0 only if an
  Apple trial measures them, and says this "puts Apple motion measurements back on 0.8.0's path, which outcome 3
  gave as its reason to defer motion" (items 6 and 24). (2) Outcome 3 gives "21 of 29 macOS" motion facts pending
  (the brief's count at a55cdcd2). Staging counts 21 of 27 at 7430a0a2 and leaves the correction to the
  orchestrator.

**3. A role the platform does not name gets native-theme's labelled mapping (outcome 5, with the U651 pairing).**
- *Decided:* a typed colour role that no source of the platform states is native-theme's own mapping where the
  platform draws the role's meaning in a colour of its own, documented or measured (item 56); a text slot every
  preset must fill takes a style or family the platform names. It is labelled as such, kept out of the gate's
  platform comparison and never recorded as the platform's fact (the specification's S3 states the rule). It
  changes R25.17's ground: the status colours of `macos-*` and `ios-*` are no longer absent by decision but stated
  where a source or a measurement shows the platform drawing them. What it brings:
  - on `macos-*` and `ios-*` each status waits on its read (rows U757–U760; a register row is one line of the
    ledger's list of what is not measured yet), its values pending meanwhile (closes U456). No pinned Apple source
    yet uses systemRed, systemOrange, systemGreen or systemBlue for error, warning, success or info. The reads begin
    by pinning and reading the HIG's colour pages; the rows hold the candidates and what closes each. What the
    evidence in hand gives: a candidate element only for macOS's error and warning (the level indicator's two
    fills). For success and info on all four presets and for all four statuses on `ios-*` the pinned headers name
    none, so unless an Apple page not yet read names the colours, those twelve status colours are absent in 0.8.0
    and each connector draws its toolkit's colour there; a preset may then draw one status in the platform's colour
    and another in the toolkit's (item 57);
  - the status text colours (`foreground`, the text on the status fill) are the colour the platform is measured
    drawing over that fill: on `macos-*` `alternateSelectedControlTextColor`, the text on emphasized content, where
    a probe pass (row U756) finds it equal, as the default push button's title is (0.7.0 drew `labelColor` there,
    which its own preset names the body text beside a status mark); on `ios-*` the filled button's measured title
    colour (row U755), checked on the second-tint run (U706); 0.7.0's was a literal with no source;
  - a status's text colour on the surface (its `standalone`) waits on the same read as its fill, for all four
    statuses, paired where the platform is found drawing the status as text or a mark and absent otherwise (the
    specification's S3); the destructive role stays absent on `macos-*` and `ios-*`: no committed source or
    measurement shows it (row U761 retired). The set text colours (`negative`, `neutral`, `positive`) refer to the status's colour there,
    its fill where no text colour is paired, as on `windows-11`, so a connector's error-text field still gets the
    platform's colour;
  - on `adwaita` the info role's fill and text colour refer to the accent's fill and its foreground, the fill the
    colour libadwaita's info bar draws info from, as 0.7.0 drew it; its standalone colour stays absent, since
    libadwaita draws no info as text (item 56);
  - the text slots' existing mappings get the same label;
  - on `ios-*`, `accent_foreground` (text on the accent colour) is paired with the colour UIKit draws a filled
    button's title in, measured `#ffffff` in both schemes on both presets, the same value 0.7.0's `ios-13` had. It
    is checked again on the second-tint run (U706);
  - on `windows-11`, the monospace slot becomes a labelled keyed style (its font family is item 26).
- *Why:* the rule invents no colour: what is native-theme's is the pairing, and unlike R19's picks it rests on a use
  the platform itself makes of the colour. With the role absent where the platform draws the meaning in a colour the
  theme could give, each connector would put either the toolkit's default there (against "connectors follow the
  theme's data") or a pairing of its own. For U651 the filled button is the one control in the slice that draws
  content on a tint fill.
- *Veto brings:* the roles stay absent. 0.8.0 keeps losing the macOS status colours, and the rationale needs a rule
  for what a connector puts in a toolkit field the model leaves empty. For U651 alone: a different witness for
  "content on the accent".
- *Source:* outcome 5; register §2.2, U651, U509, §7 item 4; brief §2 decision 5; the orchestrator's decisions on
  the batch's checks N1 to N6 and F1 (`batch-check/decisions.md`, the checks in `batch-check/`), the rule's present
  form, the set text colours and the destructive role's ground by the decisions F1-1, F1-3 and F1-4, the four
  `standalone`s alike by F1-21.

**56. Which roles the labelled mapping covers (the scope of item 3).**
- *Decided:* the mapping applies to a typed role that no source of the platform states, where the platform draws
  the role's meaning in a colour of its own: a colour its documentation or code uses for that meaning (libadwaita's
  info bar draws info from the accent, `_deprecated.scss:174`), or the colour of an element the platform itself
  colours for that meaning, measured (the filled button's title, U651); a colour that merely exists in its palette
  does not count. A reference the catalogue prescribes where the platform has one colour for two uses (the accent
  as its own text colour; the set text colours as the status colour, on `windows-11` too) is the model's form, not
  this mapping. A text slot is outside this: every preset fills every slot, by a style or family the platform names
  (U509's monospace slot among them). Where the platform draws the meaning in no colour (`material-3`'s warning,
  success and info; an Apple status whose read finds no witness, item 57), the role stays absent, the connector's
  contract records there the colour the toolkit's default theme of the resolved scheme holds in that field, and the
  contract test fails a table that lacks one. This is an exception to your rule that a connector never shows a
  toolkit's own colour; item 48 admits a toolkit's geometry, never its colour. On `material-3` this loses the three
  status colours 0.7.0 stated without a source, a loss R21 lists.
- *Why:* item 3's wording ("a role a portable application needs") named no test. Read by its own reason, it would
  cover every field a toolkit holds as a plain colour, also where the platform has nothing to pair with, which
  would make native-theme invent the colour inside a preset, the platform's look. Where the platform draws the
  meaning in no colour, the theme has no data for the connector to follow.
- *Veto brings:* either the pairings extended to those roles too (`material-3`'s warning, success and info: a
  colour native-theme chooses where the platform has none), or each connector deciding what it puts in a toolkit
  field whose role is absent; for `adwaita`'s info, the role left absent (a colour only a deprecated widget draws
  not counted as the platform's).
- *Source:* the batch check B's finding 3 and the orchestrator's decision B3 (`batch-check/check-B.md`,
  `decisions.md`), corrected by the check of the fix rounds and the decisions N1-2 and N1-3
  (`batch-check/check-N1.md`, `decisions.md`), restated by the decisions F1-1 to F1-3 on the check of the whole batch
  (`batch-check/check-F1.md`); the rationale's R5.1, R10 and R25.17, the specification's S3.

**4. A bundled preset is checked by the crate's tests, not again at load (R2.8, a rationale sentence).**
- *Decided:* the preset's own checks and the checks over combinations are not run again when a theme loads them for
  the stacks built from three kinds of data alone: a bundled preset whose pending table is empty (every one, at a
  release), a bundled palette, and the layers `Theme` derives from them. The crate's build test and the facts gate
  run those checks at every commit. The checks of each layer above the preset still run at every change. What the system and the
  application add, and a system reading with every layer above it, a bundled palette among them, where a change
  alters what the reading leaves out, is still checked when added or changed, its checks over combinations only
  where something they read changed. Queries still cannot fail.
- *Why:* the same checks already run on the same data at every commit. Running them at each load costs more time and
  memory than items 9 and 10 allow.
- *Veto brings:* `from_preset` and `set_preset` validate on every load. Items 9 and 10 (load's lines 1 and 2) are
  then missed by the check's cost and memory peak, each case with a recorded named miss.
- *Source:* load §3.8 (R2.8, class 3), §5 *The maintainer's numbers*, header; §3.3 *What it leaves*, as the
  orchestrator's decision N2-3 words it, corrected by the decision N3-2 (`batch-check/decisions.md`,
  `batch-check/check-N3.md`).

**58. From now on the orchestrator may decide a change to a rationale decision, and a register row's choice of
model form, by your earlier decisions and the project's rules.**
- *Decided:* each such decision is applied at once, marked at its rule as awaiting your veto, and listed in the
  report of the milestone or unit that made it; a pure preference, or a value only you can supply, stays yours (the
  plan's *Spec changes*, class 3; the ledger's rule for kind `d`).
- *Why:* your standing rule, settle by logic first and ask only real preferences.
- *Veto brings:* class 3 changes and kind `d` rows wait for your decision, as before D6.
- *Source:* the check F1's finding 11 and the orchestrator's decision F1-11 (`batch-check/check-F1.md`,
  `batch-check/decisions.md`).

---

## B. Losses against 0.7.0 that its users would notice

**5. `material-3`'s checked text toggle colours.**
- *Decided:* 0.7.0's checked colours for a text toggle on `material-3` (`#625b71` fill, `#ffffff` text, from
  Material's latest tonal-button tokens) are lost in 0.8.0. They return with the flavours unit, under `expressive`.
- *Why:* baseline Material draws a toggle only as an icon button. The only name that holds this look is the
  `expressive` flavour, and a flavour is a model-wide unit: bringing it in for one widget brings it for all.
  Compose 1.5's stable Expressive `ToggleButton` is where the look returns.
- *Veto brings:* either `icon_only` at depth (+291 pending facts, 1,672 against 1,381, which gives a different look,
  not 0.7.0's), or the flavours unit in 0.8.0 (0.7.0's colours only under `set_flavor(expressive)`, at the unpriced
  cost of stating every M3a widget under both flavours of `kde-breeze` and `material-3`).
- *Source:* staging §1.7 *The one proposed loss*, §6.1 R-3, §8 point 9 (decision F).

**6. The two motions, if the Apple trial does not measure them.**
- *Decided:* if the trial (item 24) does not measure a motion on all four Apple versions (macOS 15 and 26, iOS 18
  and 26), that motion leaves 0.8.0 on every preset. The losses would be the dialog's entrance kind on `adwaita`,
  `kde-breeze`, `windows-11` and `material-3`, and the busy loop as theme data on every preset. The icon sets'
  spinner frames on `adwaita`, `kde-breeze` and `material-3` would go too. Connectors would keep 0.7.0's own turning
  arc as a recorded stand-in. Each motion is judged on its own. Staging calls this fallback likely.
- *Why:* one rule for every model field: a value is stated on every preset or on none. No pinned Apple source states
  either motion.
- *Veto brings:* a larger trial (see item 24); a value stated on some presets only was rejected as a false statement.
  If decision G meant the two motions to leave together, the only change is that a measured one leaves too. Or
  (iv) a value that answers 'later' on the presets that do not state it yet (the Apple presets, where 0.7.0 states
  no entrance): truthful, and it keeps 0.7.0's coverage whatever the trial finds; it costs a model form no value has
  today (a per-preset `later` for a value field, its S12 rule and each connector's handling), a class 2 change for
  two values. Not taken: the trial may make it unnecessary; to be weighed again if the trial fails.
- *Source:* staging §5.5, §6.1 R-3, §8 point 9, §10.2.

**7. Icon sizes for contexts the platform names none for.**
- *Decided:* 0.7.0 stated all five icon contexts on every preset. 0.8.0 states a context absent where the platform
  names none: `material-3`'s `small` and `large` (0.7.0: 16 and 32, no source); `adwaita`'s `dialog` and `panel`
  (0.7.0: 22 and 20). On `macos-*`, `windows-11` and `ios-*` it reaches as far as rows U102, U516 and U127 find.
  This is added to the plan's list of losses, written for the whole class.
- *Why:* never invent a value comes before D13. The catalogue's absence table allows these contexts to be absent,
  and no source gives a size.
- *Veto brings:* only a source for each size could keep it; 0.7.0's 16 and 32 have none.
- *Source:* register §4 item 7, §7 item 7, U561.

**8. The icon theme's name on the presets without a freedesktop icon theme.**
- *Decided:* `icon_theme` is absent on `material-3` (0.7.0 repeated the set's name, `"material"`), as the catalogue
  has it for every preset whose icon set is not freedesktop. The set's name stays in `token.icons.set`. This is
  added to the plan's list of losses.
- *Why:* these platforms draw one fixed symbol set and have no named icon theme the freedesktop lookup could use.
- *Veto brings:* the set's name repeated as the icon theme, which reverses the catalogue's row for all six
  non-freedesktop presets.
- *Source:* register U559, §7 item 7.

**57. The Apple status colours, if no Apple source pairs them (with item 3).**
- *Decided:* a status whose read (rows U757–U760) finds on a preset neither Apple documentation naming a system
  colour for that meaning nor a measurement of an element the platform itself colours for it is absent on that
  preset, and each connector fills its toolkit's field with the toolkit's own colour, recorded in its contract;
  0.7.0's status colour there is then lost. A status's text colour on the surface (its `standalone`) waits on the
  same read, for all four statuses alike (item 3). The macOS and iOS lanes make the reads before their facts edit, at the
  latest at M4's pending condition, beginning with the HIG's colour pages; the rows state how a colour found is
  paired (the specification's S3 states the rule). Until a read closes, those values are pending, which no release
  may hold. What the evidence in hand gives: a candidate element only for macOS's error and warning (the level
  indicator's two fills). For success and info on all four presets and for all four statuses on `ios-*` the pinned
  headers name none, so unless an Apple page not yet read names the colours, those twelve status colours are absent
  in 0.8.0 and each connector draws its toolkit's colour there; a preset may then draw one status in the platform's
  colour and another in the toolkit's.
- *Why:* item 56: a colour that merely exists in the platform's palette does not count, and 0.7.0's choice cites no
  source for the meaning. The same rule leaves `material-3`'s warning, success and info absent, where 0.7.0 also
  drew colours.
- *Veto brings:* the system colour kept as native-theme's pick even without an Apple source. That needs a further
  alternative in item 56 (a keyed colour 0.7.0 drew for that meaning), by which `material-3`'s info (`#6750a4`, its
  primary) would qualify too, so item 56 changes with it.
- *Source:* the orchestrator's decisions N2-1, N3-1, N4-1, N5-1, N5-3, N6-2, N6-4 and N6-5 on the batch's checks of
  its fix rounds, the check F1's finding 7 and the decision F1-21 (`batch-check/decisions.md`, the checks in `batch-check/`); the
  specification's S3, the catalogue's *global roles*, `status`; the rationale's R21 and R25.17.

---

## C. Numbers

The load budget (items 9–20) applies per preset, in a release build, on the measuring machine. Its numbers are
proposals and apply until you change them. For every line, a veto means you give another number. It is an edit of
`cost_budget.toml` with its ledger row. The next run that measures the line compares against it, and the triggers
resting on it are evaluated again. Load names eleven lines and a twelfth figure that is reported, not set. "Today"
is the slice with one widget.

| # | Line | Proposed | Why (what sets it) | Today; 0.7.0 | Veto brings (beyond your number) | Source |
|---|---|--:|---|---|---|---|
| 9 | first load of a preset (`Theme::from_preset`) | 16 ms | load fits one 60 Hz frame | 259–17,605 ms; 0.457–0.768 ms (21–35 × given up) | – | load §2.2 line 1 |
| 10 | peak memory during that load | 16 MiB | twice line 11's kept memory, briefly | 18.0–1,827.0 MiB; 0.5–0.8 MiB (20–32 ×) | – | line 2 |
| 11 | memory kept: the theme and one window's resolution | 8 MiB | no more than one full-HD window buffer (7.9 MiB) | theme 4.0–47.6 MiB; under 0.5–0.8 MiB (10–16 × or more) | – | line 3 |
| 12 | a change every application makes (re-read, `pick`, `set_accent`, palette, …) | 0.5 ms | 0.5 + 1 + 300 × 0.05 = 16.5 ms fits one 60 Hz frame | not measured; 0.020–0.023 ms | – | line 4 |
| 13 | `resolve`, something changed | 1 ms | the same sum | 0.23–1.47 ms; 0.020–0.023 ms (43–50 ×) | – | line 5 |
| 14 | `resolve`, nothing changed | 10 µs | per frame: 0.01 + 300 × 0.002 = 0.61 ms, 7.3 % of a 120 Hz frame | 0.23–1.47 ms; as line 13 | – | line 6 |
| 15 | a styled query, computed | 50 µs | 300 of them are 15 ms of line 12's sum | 32–53 µs; a field read, 0.7 ns | – | line 7 |
| 16 | a styled query, repeated in one resolution | 2 µs | line 14's sum | 32–53 µs; 0.7 ns (about 2,900 ×) | binds only a connector that queries every frame | line 8 |
| 17 | bytes a preset adds to a binary | 256 KiB | between two facts: 18–73 kB of compressed data per preset today, and every binary carries all eight presets | 169,338–1,759,947 B; 25,902–62,653 B (2 MiB for eight, 6.8 ×) | the least defensible number of the eleven; six presets already pass it. A feature per preset (only the needed presets compiled in) is built if you ask | line 9; §2.3; §3.2 (8) |
| 18 | `from_system`, the theme's own part | 16 ms | load fits one 60 Hz frame | not measured | – | line 10 |
| 19 | peak memory during `resolve` | 8 MiB | no more than the theme may keep (item 11) | not measured | – | line 11 |
| 20 | cost of adding an application layer that states structure | reported, no line | no figure exists | – | a line set at the calibration after M3a's first unit | line 12; §3.6 |

What 0.7.0 users give up at these numbers: a load 21 to 35 times slower, a resolve 43 to 50 times, ten or more
times the memory, 6.8 times the bytes (load §2.3).

**21. The reference window has 300 parts.**
- *Decided:* the budget assumes a window of 300 parts. Lines 15 and 16 scale with it.
- *Why:* it is the figure the brief used; load calls it "an assumption, the maintainer's to change".
- *Veto brings:* your part count, and lines 15 and 16 rescaled from it.
- *Source:* load §2.1, §7 #11; brief [D]:172-174.

**22. The three rules behind the lines, and the factor of six.**
- *Decided:* per frame, the theme takes under a tenth of a 120 Hz frame. Between two frames, one change, one
  resolve and every part asked again fit one 60 Hz frame. At start-up, the load alone fits one 60 Hz frame. A
  machine six times slower then stays near 100 ms. How each total is split among its lines is a further judgment.
- *Why:* each rule is a judgment. The 100 ms limit for a response felt as immediate is general knowledge, not from
  the project's sources.
- *Veto brings:* other rules, from which the lines follow again.
- *Source:* load §2.1, §2.3, §7 #11.

**23. A met line may be exceeded within its tolerance; growth under a line does not fail.**
- *Decided:* a figure over its line by less than its tolerance (at most 20 %) passes. It is printed "over its line,
  within its tolerance". Two consecutive releases printed so get a recorded named miss. For example, item 9's
  16 ms may read 19.2 ms at every run. A figure that grows while staying under its line does not fail.
- *Why:* one run cannot tell such a figure from noise. Growth under a line is inside the budget by the line's
  definition, and the figures are kept in each release's ledger row.
- *Veto brings:* growth under a line fails too. Each line's bound becomes the smaller of its line and its record.
- *Source:* load §2.4 rules 1 and 3, *What it gives*, *What would change it*.

**24. The Apple motion trial is capped at two harness rounds per Apple harness.**
- *Decided:* before M2's runtime task, a trial reads the alert's presentation and the activity indicator's loop
  from the running animations on macOS 15 and 26 and iOS 18 and 26. It gets at most two harness rounds (CI runs
  that measure on the platform) per harness, four in all.
- *Why:* this is decision G's budget, set by the orchestrator, not derived from evidence. A third round would be a
  search for a method, which is the reason outcome 3 deferred motion. The slice's Apple lanes needed several
  correcting rounds each, so the fallback (item 6) is likely.
- *Veto brings:* a larger cap, which puts more Apple motion work on 0.8.0's path. Or no trial: both motions leave as
  item 6 describes.
- *Source:* staging §5.5, §8 point 9, §6.4 P-7.

**25. The facts gate's limits (outcome 9).**
- *Decided:* 3 h a shard on CI (the facts gate, the test that resolves every preset in every combination and
  compares the result with the facts, runs on CI in parallel parts called shards). 1 h and 12 GB for a local preset
  step. Shards are added as units land. The release run stays one unsharded local run, timed at M1b's close at
  459 min on 8 threads with a peak of 7.65 GB. The limits are reviewed with the first widget unit's measured times.
- *Why:* the figures are measured and inside the limits (the longest shard 1.99 h). The case that passes them, 30
  runner hours a push, arises only if every combination is measured, which item 1 excludes.
- *Veto brings:* past 30 runner hours a push, only the touched units' gates run, with the whole gate nightly. Or a
  speed target the gate must meet before M3a.
- *Source:* outcome 9; brief §2 decision 9.

---

## D. Picks and choices of form, where no source decides

**26. The monospace font family on `windows-11` (U509).**
- *Decided:* Consolas.
- *Why:* WinUI names no monospace style. Consolas is the family the pinned Microsoft sources name for the nearest
  uses: a font for a Windows app's UI, code in a WinUI Labs control, monospace in Fluent's web tokens. No source
  makes any family the platform's, and none of the pinned sources mentions Cascadia.
- *Veto brings:* Cascadia Mono, which 0.7.0 shipped, after one or two sources are pinned (Windows Terminal's
  default face, Microsoft's list of installed fonts); or Lucida Console (WinUI's caption choice); or Courier New.
  One string changes.
- *Source:* register U509 part 3, §7 item 1.

**27. The form of Qt's underline on `kde-breeze` (U180), and with it U505's timeout.**
- *Decided:* the model's arithmetic gains two operators, `ceil(x)` and `if_le(a, b, x, y)`, plus one font metric,
  `font_descent`. This is a model change. QPainter's underline offset is then a 215-character expression with no
  literal but Qt's, exact against Qt's code. `windows-11`'s caret-blink timeout (U505) takes `if_le` in the same
  batch.
- *Why:* the smallest change that states Qt's rule exactly. The other forms need an exemption the specification
  rejects, or a clean room.
- *Veto brings:* today's operators (316 characters, with an exemption for `0px` and `1px` that the specification
  rejects); or `ceil` alone (273 or 358 characters, the same exemption); or a platform function with a table of test
  values (131 characters, cannot reach the third case, needs a clean room).
- *Source:* register U180, §7 item 2 (decision D).
- *With it:* Qt Quick's line position (row U753, the Kirigami flavour's labels) takes the non-breaking form, a
  field on `Decoration` by which the connector puts the line's top on a whole pixel, defined with the flavours unit;
  nothing changes in 0.8.0 for it, and the flavours unit stays a patch release. *Veto brings:* an element datum for
  the baseline's place in the item, a breaking change, or 0.8.0 made ready for it (`ElementDatum`
  `#[non_exhaustive]`, a default for the new font-metric method). *Source:* the orchestrator's decision at the
  batch's fix round (`batch-check/decisions.md`, "U753's form"). QPainter's wavy and strike lines are row U754,
  due with the first unit that states one.

**28. A part a control cannot be given content for is treated as content it lacks (U122, `ios-*`).**
- *Decided:* 294 of 564 pending facts need no value: the subtitle, the menu indicator and their placing values,
  plus the route picker's label, on five iOS button forms that cannot hold them. The check counts 293; the
  difference is one fact. The other 270 stay pending as measurements.
- *Why:* the specification's content rule (S6.3, a part the element's content lacks is not queried) covers it as
  written. UIKit's headers show these forms have no such content.
- *Veto brings:* one clause at the specification's S6.3 that says so (same outcome); or the parts declared absent
  (which S6.3 rejects for content); or the 294 left pending until a run ("a measurement of nothing drawn").
- *Source:* register U122, §7 item 3.

**29. `material-3`'s `panel` icon size is the navigation drawer's (U561).**
- *Decided:* `panel` = 24px, from `md.comp.navigation-drawer.icon.size`.
- *Why:* the catalogue and 0.7.0 both map the context to the drawer, but no specification sentence defines
  "panel".
- *Veto brings:* `panel` absent too. That needs a change to the catalogue's table and drops a value 0.7.0 had with a
  source, against D13.
- *Source:* register U561, §7 item 5.

**30. `auto` carries the command link's subtitle placement on `kde-breeze` (U424).**
- *Decided:* `subtitle_gap` may be `auto`, meaning the subtitle is centred in the space left below the title's line.
  It applies with the unit that brings `@command_link`, after 0.8.0.
- *Why:* Breeze computes the gap from the button's height, so no measurement can state it, and `absent` would draw
  it wrong. This is the smallest of the three forms that state it.
- *Veto brings:* a field of its own (an alignment of the subtitle, which every preset would state), or an expression
  (which needs an operand that does not exist).
- *Source:* register U424, §7 item 6.

**31. A variant beyond 0.7.0's depth leaves the public `Variant` enum until its unit.**
- *Decided:* typed variants not at depth are not members of `Variant` until their later unit. Each later unit that
  adds typed variants is therefore a breaking release (D14).
- *Why:* outcome 2 says such names are "impossible to ask for until their unit", and only removal achieves that for a
  typed variant.
- *Veto brings:* the members stay behind an enabled flag and are refused at run time with a `Later` answer. Later
  units of depth could then be patch releases, but a not-yet-stated variant can be written.
- *Source:* staging §2.2 (*What would change it: the maintainer preferring patch releases*), §6.4 P-11.

---

## E. Corrections against 0.7.0, for information

These replace 0.7.0 values that had no source, contradicted their own cited source, or came from a source the
preset's reference rule does not choose. Each is listed beside R19's invented light variants on the plan's list of
what users lose. A veto would need a source for 0.7.0's value. For items 38 and 39 it would instead need a change of
the reference rule.

**32. The look of a button with no variant on `material-3` and `ios-13`.** *Decided:* it draws the platform's
default button: Compose's filled `Button`, and UIKit's system button with no fill. 0.7.0's looks stay reachable
under `tinted` (Material's tonal button) and `@gray` (the grey fill). *Why:* 0.7.0 mapped its plain button to
neither platform's default: Material's tonal button, and WebKit's HTML button (`Button.kt:77-88` against
`FilledTonalButton`, `:439-447`; `UIButton.h:22-23`). This is visible to every application that drew 0.7.0's
plain button. *Veto brings:* connectors asking for `tinted`/`@gray` per preset (against "connectors follow the
theme's data"), or a new theme datum for a widget's default variant, which has no source. *Source:* staging §1.3,
§1.7, §8 point 9, §10.2.

**33. `adwaita`.** Text colour `#2e3436` (no source) → the window's foreground `#000006cc`. Icon–text gap 8 → 6px.
Minimum width 64 → 16px plus the padding. *Source:* staging §1.7, §8 point 9.

**34. `kde-breeze`.** The primary button inherited the accent → Breeze's default button (`{default}`). Minimum
height 32 → `auto`. *Source:* staging §1.7 (C-3b), §8 point 9.

**35. `macos-*`.** Hover fill `#e0e0e5` → none (macOS has no hover state for this button). `macos-monterey` pressed
fill `#d6d6db`/`#4e4e50` → measured `#f0f0f0ff`/`#ffffff5c`. Icon–text gap 4 → 2px (measured). Minimum width 64 →
16px and 34px. *Source:* staging §1.7, §8 point 9.

**36. `ios-13`, unsourced values.** Hover `#d1d1d6` → none: 0.7.0's next line said "iOS has no hover state", and
hover is the iPad's. Pressed and disabled fills `#c7c7cc`, `#e5e5ea80` → `@gray`'s `#7878801f`. Minimum width 64
and height 44 → `auto` (44 is a hit target, not a drawn height). Icon–text gap 6 → 0px. *Source:* staging §1.7,
§8 point 9.

**37. `windows-11`.** Disabled fill `#f9f9f900`/`#33333300`, against its own cited `ControlFillColorDisabled` →
`#f9f9f94d`/`#ffffff0b`. Minimum height 32 and width 64 → 0px. *Source:* staging §1.7, §8 point 9.

**38. `material-3`, by its reference rule (R1.6: the latest token set, then Compose).** Disabled container
0.12 → 0.1 (0.7.0 used the older `v0_192` tokens). Vertical padding 10 → 8 (one line still draws at the 40 px
minimum). Minimum width 64 → 58. The last two are Compose's values where 0.7.0 used material-web's component.
*Source:* staging §1.7 (class 4), §8 point 9.

**39. `ios-13`, by its reference rule (R1.3, R1.4: UIKit, not WebKit).** Pressed label `#007aff` → `#007affbf`.
*Source:* staging §1.7 (class 4), §8 point 9.

**40. The rationale's description of 0.7.0's accessibility.** *Decided:* "four run-time flags … with no theme values
behind them" becomes "no preset values behind them". Under high contrast, the Windows reader writes system colours
and the GNOME reader the focus ring into the theme. The macOS reader's colours follow Increase Contrast. The
connectors honour `reduce_motion`. *Why:* 0.7.0's code does this (`windows.rs:315-446`, `gnome/mod.rs:180-182`).
*Veto brings:* the old sentence, which that code contradicts. *Source:* staging §1.1, §6.1 R-7 (class 3 by its
place).

---

## F. Confirmations of earlier proposals

**41. Licence classes (outcome 11).** *Decided:* D10 names four classes: the clean room (one agent describes the
licensed source, another implements from the description alone) for every LGPL and GPL reference; an own port of an
Apache-2.0 or MIT reference under a file-level licence with its notice; the manifest's
`(MIT OR Apache-2.0 OR 0BSD) AND Apache-2.0 AND MIT` as it stands. *Why:* you accepted "AND Apache-2.0" on
2026-10-29, and MIT's obligation is the same kind. This is a risk judgement, not a legal conclusion. *Veto brings:*
no code that carries a notice: the MIT and Apache ports are redone through the clean room, and the licence
returns to `MIT OR Apache-2.0 OR 0BSD`. *Source:* outcome 11; brief decision 11.

**42. The five rationale amendments (outcome 12).** *Decided:* the iOS tint is a measured derivation over the accent
(U706's second-tint run is scheduled); a palette is suspended, not refused, under a followed scheme it lacks; the
ten bundled palettes are 0.7.0's ten schemes; the error enums are `#[non_exhaustive]`; no font-DPI read where the
toolkit's scale carries it (the KDE reader's `Xft.dpi` read goes). *Why:* each removes a defect found as a fact,
for example `set_accent` leaving 555 tinted button facts system blue. *Veto brings:* for the tint, `set_accent`
refused on iOS until the run is made; for the others the brief saw no open alternative. *Source:* outcome 12;
brief decision 12.

**43. The plan's ADD-A146 amendments (outcome 13).** *Decided:* they stand: your closing cadence, the register's two
kinds of row, M2's task 0, the connectors' design before their code, which gate run gates what, M5's comparisons.
*Why:* corrections of fact and of orders that could not run. *Veto brings:* the earlier text, item by item.
*Source:* outcome 13; brief decision 13.

**44. Presets generated from the facts (outcome 13).** *Decided:* they stay generated: 97.8 % of the presets' lines,
with `--check` in every gate. *Why:* hand-written values could disagree with the facts. *Veto brings:* hand-written
presets. *Source:* outcome 13; brief decision 13.

**45. The audit rate (outcome 13).** *Decided:* 5 % of agreed facts, at least 10 a unit, and every measured fact
recomputed. *Why:* the audits found 0 wrong of 430 and 1 of 74; the recomputation found the larger defects.
*Veto brings:* a rate you name. *Source:* outcome 13; brief decision 13.

---

## G. The other decisions of D6

The six groups above have no place for these. Each follows the project's rules, and each is open to your veto all
the same.

**46. The load gets a design task now (outcome 1).**
- *Decided:* a profile of the load, then a design note that chooses the load path (`load-design.md`, with the budget
  of items 9–20), then the repair before M3a's first widget unit, with a release-mode timing check at each unit's
  gate.
- *Why:* the plan's rule "if the slice already misses it, a task before M3a costs less than one after it". The
  slice loads in 0.3 to 40 s and up to 1.4 GiB, where 0.7.0 takes under 1 ms.
- *Veto brings:* a budget alone, with a repair only on a miss (which has already happened), or the cost accepted
  and documented (load a theme off the UI thread).
- *Source:* outcome 1; brief decision 1.

**47. The readers unit moves to the start of M3b (outcome 4).** *Decided:* M3a keeps each unit's own readers step.
*Why:* comparing all four readers with 0.7.0's found no read whose target lies outside 0.8.0's other units.
*Veto brings:* the unit stays in M3a (208 trace rows, an estimated 16 to 97 M of usage). *Source:* outcome 4;
brief decision 4.

**48. Widgets a preset declares unavailable (outcome 6).**
- *Decided:* the declarations stay. Per preset, the connectors' contract lists what a connector draws its toolkit's
  widget from: a stand-in entry, or, where the catalogue names none, the toolkit's own geometry with every colour
  and font from the theme's typed roles. A test checks the list is complete. In 0.8.0 the query answers "not
  available" where 0.7.0 resolved `tab`, `card`, `expander` or `status_bar` on every preset. 0.7.0 held values
  under those names, such as the iOS tab bar's height of 49 and the macOS box margins.
- *Why:* the declarations are true; without a rule each connector decides on its own.
- *Veto brings:* `bottom_tab_bar` and `group_box` moved into 0.8.0 (two more units, and the rule is still
  missing), or each connector decides.
- *Source:* outcome 6; brief decision 6.

**49. What a community palette states (outcome 7).** *Decided:* decided with the palettes' design (M2's fourth
task). A palette over a preset whose widgets read keyed colours reaches them by a stated, reviewed mapping method,
or it is not offered over that preset. A palette is not offered over a preset whose widget colours the method
cannot reach, measured literals that follow no role (`macos-*`, `ios-*`: `macos-tahoe`'s button fills are 2,561
measured literals of 2,795 fill lines), a recorded limit (the orchestrator's decision at the batch's fix round).
*Why:* R19 rejects a mixed look, and a method removes the per-pick veto.
*Veto brings:* typed roles only, with the mixed look recorded (for example the Windows button in WinUI's greys
under Catppuccin's colours); over `macos-*` and `ios-*`, the palette offered with the limit recorded (its colours
beside the platform's measured literals). *Source:* outcome 7; brief decision 7.

**50. `docs/platform-facts.md` is no longer committed (outcome 8).** *Decided:* CI and each release generate it.
`check_facts.py` splits a facts file by key prefix past 40 MiB. Local tools that cite its lines read the facts or a
generated copy. *Why:* the file is 47.8 MiB, against GitHub's warning at 50 MiB and GitHub's block at 100 MiB, and it
grows with every fact. *Veto brings:* it is committed per platform in smaller files, or kept as it is. *Source:*
outcome 8; brief decision 8.

**51. No usage cap now (outcome 10).** *Decided:* the work runs to M2's close. The report there gives M2's cost, the
gate's time and the load cost, and M3a's approval is asked with the first real unit costs. No lane writes facts
ahead for M3b meanwhile. Setting the caps for M3a stays yours, at M2's close. *Why:* your instruction "do
everything until M2 is finished". The project's one cost estimate so far was wrong by a factor of 12.8, so a cap
taken from estimates would either not bind or trip at once. *Veto brings:* a cap per milestone (the brief suggests
the upper figures), or approval only up to a checkpoint after M3a's first widget unit. *Source:* outcome 10; brief
decision 10.

**52. The register's choices of model form, settled by the orchestrator (outcome 14).**
- *Decided:* of the fifteen rows examined, fourteen are settled from the sources (two in part). The fifteenth, U555,
  moves with `single_click_activates` to the containers unit. What is yours is listed above: items 26–30, the
  U651 pairing in item 3, and items 7 and 8. For U473 (iOS key repeat), 0.7.0's model holds no such value on any
  preset, so it goes to a later unit (the parking unit *behaviour*) and needs nothing from you.
- *Why:* your standing rule: settle by logic first, ask only real preferences.
- *Veto brings:* each row put to you one by one, or left until its unit is due.
- *Source:* outcome 14; register §1, §2.3 (U473).

**53. The next full check of the documents (outcome 15).** *Decided:* one full pass over all documents after the
batch that applies D6. Its fixes are checked until a check finds nothing serious, and further passes read only what
the fixes changed. Then each unit's batch is checked as it is written. *Why:* your document standard, and your
request to be deliberate with full passes (about 16 M of usage each). *Veto brings:* a full pass now, before the
batch, or no further full pass. *Source:* outcome 15; brief decision 15.

**54. 0.7.0's two sourced plain-button looks stay reachable (decision E).** *Decided:* `material-3`'s tonal button
under `tinted` (+158 pending facts on the path, 146 of them on `macos-tahoe`) and `ios-13`'s grey fill under
`@gray` (+12). *Why:* D13: users lose nothing of 0.7.0's model; both looks have sources and the facts hold them.
*Veto brings:* both looks lost and listed as losses, with 170 fewer pending facts on the path. *Source:* staging
§1.3 (C-3d, C-3e), §8 point 9.

**55. `tool` comes to depth now.** *Decided:* the toolbar button's variant is stated in 0.8.0 (+198 pending facts on
staging revision 2's variant list, +197 before the fix round counted `label_position` at depth).
*Why:* 0.7.0's KDE reader filled `toolbar.font` from `toolBarFont`, which the KDE facts state only under
`variant tool`. *Veto brings:* KDE's toolbar font is lost at the containers unit, with 198 fewer pending facts.
*Source:* staging §1.3 (C-3c), §8 point 9.

---

## Points that may be yours but are not clearly marked

- **The individual status-colour pairings (item 3).** The catalogue's *global roles* `status` row lists each once
  its row (U757–U760) closes, and item 3 names the status text colours, on `ios-*` the filled button's title colour
  (U755).
- **0.7.0's values under `tab`, `card`, `expander`, `status_bar` (item 48).** The query answers "not available"
  where 0.7.0 gave values. No source calls this a loss, so it is not in group B.
- **The load gate's other parameters:** at most 10 runnable threads for a pass to count, a 30-minute bound for the
  release run's re-sweeps, a 5 % tolerance floor. Load calls them judgments but assigns them to no one; its list of
  what is yours (§7 #11) names only the eleven numbers, the 300 parts and the factor of six. Item 23 is included
  because load marks it "what would change it: the maintainer asking".
- **Item 31 (the enum form)** is marked only by staging's "what would change it: the maintainer preferring patch
  releases" and is not on staging's own veto list (§8 point 9). It is included.
- **Item 40 (R-7)** is class 3 (yours) only by its place in the rationale. It corrects a description against the
  code, so it is placed with the corrections.
- **The brief's §3, "What does not need a decision"** (for example the second verification round, the API changes,
  the catalogue changes): you saw these with "say so if you object", and D6's outcome does not restate them. Not
  listed.
- **The plan and specification texts that apply these outcomes** (staging P-1 to P-14, the S8 sentences in load
  §3.8) are the orchestrator's by class. They are cited under items 1, 2 and 4, not listed apart.
- **Open points staging and the register leave to later units**, not to you: `hover` and `focus_visible` on `ios-*`
  (+252 pending facts if kept; staging §8 point 2); `single_click_activates` (the containers unit); QPainter's
  wavy and strike lines (U754, their units). Qt Quick's underline position is now decided in form (item 27).
- **Load is still being revised.** Items 4 and 9–23 follow revision 8 as read on 2026-11-12.
