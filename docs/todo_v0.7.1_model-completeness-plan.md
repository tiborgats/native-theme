# v0.7.1 → 0.8.0 — Model completeness: Plan

**Status:** approved 2026-10-29 with the maintainer's decisions D1, D4, D5, D7, D9, D10, D11 and D12 (each marked
below); draft 12. Eleven independent reviews; draft 11 consolidated
their fixes into one text, and draft 12 applies the eleventh review's. Draft 13 (2026-11-06) applies the maintainer's
scope decision D13 (0.8.0 covers what 0.7.0 covers, the rest of the catalogue follows in later releases) and D14,
settled by logic (the follow-up releases numbered by SemVer), with an independent check's fixes. Draft 14
(2026-11-09) applies the document check's fixes (spec batch ADD-A146), its amendments kept at D6 (outcome 12),
open to the maintainer's veto. Draft 15
applies the outcomes of the checkpoint D6 (decided 2026-11-10, applied 2026-11-12; spec batch ADD-A147), each open
to the maintainer's veto.

**Goal:** the model the specification states, implemented; the eight platform presets on six platforms written from
facts and passing the facts gate; the bundled palettes checked against their projects' published colours; the four
connectors and the three showcases moved to the model (R21, S21). Released in steps (D13): 0.8.0 when the model holds
everything 0.7.0's model holds (*0.8.0's scope*), then the catalogue's other units in the follow-up releases.

**Inputs:** the gap map ([`todo_v0.7.1_model-completeness-gap-map.md`](todo_v0.7.1_model-completeness-gap-map.md)),
the rationale ([`…-rationale.md`](todo_v0.7.1_model-completeness-rationale.md), "R…") and the specification (core
[`…-spec.md`](todo_v0.7.1_model-completeness-spec.md), "S…"; catalogue
[`…-spec-widgets.md`](todo_v0.7.1_model-completeness-spec-widgets.md); the six traces). This plan decides the order of
the work, its gates, what runs in parallel and how the work is checked; it decides no part of the model. Where the
work finds the specification wrong, the specification is corrected first (*Spec changes*) and the work follows.

**The presets:** `adwaita` (GNOME), `kde-breeze` (KDE), `macos-monterey` (macOS 15, R1.5) and `macos-tahoe` (macOS
26), `ios-13` (iOS 18, R1.5) and `ios-26`, `windows-11`, `material-3` (Android). `ios-26` does not exist yet (its
`docs/todo.md` entry: 38 required fields had no iOS 26 source); S21.1 makes it a release condition, and this plan
creates it.

**Branches:** `v0.8.0-rc1` for the work, cut from `main` after `v0.7.1-rc1` is merged, or from `v0.7.1-rc1` (D5);
`v0.8.0-tools` for the tooling (*Where the work happens*). Each gets a draft pull request after its first commit, so
every push runs CI (`ci.yml` runs on pull requests, not on branch pushes). 0.8.0 is not released before M0–M6 are
closed (R21, D13; 0.7.x patches, D11); each follow-up release has its own branch and gate (*After 0.8.0*).

## Global constraints

- **No invented value.** A value enters a fact only from a source pinned to its platform's release set (kind
  `published`), from an API a reader calls (kind `read`, its definition cited), or from a measurement recorded with
  its method, input, output and the exact OS and toolkit builds (kind `measured`, with its tolerance) (S17). Every
  fact is checked mechanically and by an independent verifier; every platform algorithm the model implements (S2.2's
  platform functions, S13's placement strategies) is checked against its reference (*How values are checked*). A
  preset's declarations (availability, variants, sizes, container sizes) are facts too.
- **No runtime panics.** `unsafe` only inside the existing module exceptions (`native-theme`'s `macos.rs`,
  `windows.rs`, `sficons.rs`, `winicons.rs`, `watch/macos.rs`, `watch/windows.rs`; the probe's `macos/ffi.rs` and
  `windows/ffi.rs`), S15's new module (approved 2026-10-29) and what D4 approves; every `unsafe` block carries its
  safety argument, as `sficons.rs` does.
- **Gates are mechanical.** Every task names the command whose result closes it. A visual comparison is kept where the
  project's rules ask for one, recorded with its screenshots, beside the mechanical gate, never instead of it.
- **One writer per file at a time.** The specification, the catalogue, the traces, the gap maps and the ledger have
  one writer, the orchestrating session, which also makes every commit on `v0.8.0-rc1`.
- **Captures announce themselves** when they take the desktop's focus, and are run and compared by the agent.
- **"Green"** means no failure except `PLAN_EXCLUDED`'s skipped steps; the warnings are read and recorded in the
  ledger.
- **Closing** (ADD-A146: the cadence the maintainer set on 2026-10-31). A code task ends with its own tests and
  `scripts/check_release.sh --quick` green on its last commit, every third task with `--commit`, and the pull
  request's CI green when it touches macOS or Windows code (`--commit` and the full run check and lint both targets
  but run neither's tests; `--quick` checks `native-theme` and its derive alone). A
  tooling task or spike, which runs no `cargo` in its worktree, closes on its CI run, and the per-task gate runs on
  its merge. A milestone ends with the full `scripts/check_release.sh` green, its documents and its
  part of the CHANGELOG's 0.8.0 entry written, a commit and the branch pushed (standing grant for work branches).
  Rejected: `--commit` on every task, the rule until then (the checks it adds to `--quick` seldom move with one
  task's change, and every third task still runs them).
- **The facts gate's cost** (ADD-A146). The gate over the slice alone runs for hours (on CI eight shards, the
  longest 2 h 24 min at 9cf108f0, the ledger's *Milestones*; locally `macos-tahoe` passes 12 GB where more than 8
  of its condition vectors resolve at once, `native-theme/tests/facts_gate/run.rs`'s `thread_count`), and M3a adds
  29 widgets to its one. So each gate names its run:
  1. *Which run gates what.* A code task's gate (*Closing*) runs the gate's fixtures and, through the step cache,
     the per-preset gate steps whose inputs the task changed (a change of `native-theme`'s sources all eight, a
     connector's task none). A unit's gate is the whole gate on CI's shards with the listed-paths merge
     (`ci.yml`'s `facts-gate` and `facts-gate-listed` jobs). The unsharded release run (`FACTS_GATE_RELEASE=1`,
     *M4*) is M4's and each release's, run once, on the thread count the machine's memory holds
     (`NT_FACTS_GATE_THREADS`).
  2. *The limits* (D6, outcome 9; ADD-A147). No CI shard runs past 3 h, and no local per-preset step past 1 h or
     12 GB. `ci.yml`'s shard count is one list (the `facts-gate` matrix), lengthened in the commit of the unit
     that would pass the limit, within the account's limit on concurrent jobs (not in D6's data; the ledger records
     it beside D3's when a run shows it). The release run stays one unsharded local run (the ledger's queue #213),
     timed at M1b's close at 459 min on 8 threads with a peak of 7.65 GB. The limits are reviewed with the first
     widget unit's measured times (3, below). Rejected: past 30 runner hours a push, only the touched units' gates
     with the whole gate nightly (that case arises only where every combination is measured, which *The depth 0.8.0
     states* excludes); a speed target the gate must meet before M3a.
  3. *The calibration.* After M3a's first unit that holds widgets (window and chrome), the gate's time per preset
     and step (`NT_FACTS_GATE_TIMING=1`) is recorded in the ledger and the later units' times are extrapolated from
     it; a unit that would pass a limit at the largest shard count stops the series queue for a gate-speed task
     first.

  Rejected: every task waiting for a whole CI run (hours a task, for a gate most tasks' changes cannot move); a
  budget alone, as D6's item had it (before two speed-ups three of the slice's eight shards were cancelled at the
  six-hour job limit, run 42534030431, and what a unit adds cannot be extrapolated from one widget).
- **The load budget** (ADD-A147; D6, outcome 1). What the model costs an application (a preset's load, a change, a
  resolve, a query, memory and the binary's bytes) is held to *The load budget*'s lines, per preset, in a release
  build on the measuring machine, a case over its line passing only under a named miss with its cause. The work a
  call does (the runs of the checks over combinations, the reads of a preset, a resolution or query built again) is
  gated in `cargo test` on every platform; the times and the bytes by the budget step (`scripts/check_cost_budget.py`
  over the probe's output, against `scripts/cost_budget.toml`), on the measuring machine only, from M2's load task 6:
  a unit run in each unit's gate (*M3*) and the release run in `check_release.sh`'s full run, never in `cargo test`
  or on CI.

## The transition from today's model

### The new model beside today's, then the switch

- **M1a builds the new model beside today's**, in a public module hidden from the documentation
  (`#[doc(hidden)] pub mod next;`, its presets under `src/next/presets/`; `ThemeParts` beside `ThemeWidget` in
  `native-theme-derive`), one part per task with its tests. Today's crate, connectors and showcases stay green, the
  integration tests (the facts gate is `tests/facts_gate.rs`) reach it, and clippy's `-D warnings` raises no dead
  code. Rejected: a private module (dead-code warnings, and integration tests cannot reach it); a cargo feature `next`
  (every check leg gains and later loses it).
- **M1a's last task is the switch.** It moves `next` into place as the public API, applies the module inventory,
  rewrites the README's examples (they are doctests) and fills `PLAN_EXCLUDED`. No second model is kept after M1a.
  Rejected: deleting today's model first (the crate stays red for most of M1a); one task for all of M1a (too large to
  review or gate); red commits inside M1a (breaks the closing rule).

### The module inventory

Before the switch, the ledger gives every module, integration test and `lib.rs` item of `native-theme` one fate, with
its milestone. The plan's examples:

- **Ported beside** in M1a: what the slice's API needs before the switch (`error.rs`'s new errors; the colour type if
  S1 changes it).
- **Kept in place, repointed at the switch:** code whose API `next` does not use before the switch and whose change is
  its imports and types: `sficons.rs`, `winicons.rs`, `rasterize.rs`, `fonts.rs`; `model/icons.rs`, moved whole out of
  `model/` (the enums `IconSet` and `IconRole` made exhaustive by S21.2; `IconData`, `RgbaData`, the `IconProvider`
  trait and `icon_name` as they are), which `native-theme-build`, `sficons.rs` and `winicons.rs` use. No `unsafe`
  file is copied.
- **Split**, where kept code calls a reader: at the switch the file loses its reader functions (a deleting edit) and
  the rest stays linked; the milestone that ports the reader restores them from the switch's parent commit.
  - `detect` (which `native-theme-build` calls) keeps its desktop detection and loses the scheme and accessibility
    functions S16 removes;
  - `macos.rs` keeps `system_ui_font`, which `fonts.rs` calls (so no new `unsafe` file arises);
  - `kde/` keeps `kdeglobals_path` and `kdeglobals_icon_theme` (`read_kcmfonts_key` goes with the reader part, its
    one caller being the reader's DPI chain); `gnome/` is unlinked whole (`detect_portal_backend`'s one caller goes
    with the reader part); `windows.rs` keeps `os_version`, its DPI constant going with the reader part (the
    inventory's F3, F4 and F1; ADD-A146);
  - `pipeline.rs` keeps what linked code calls (its types repointed) and loses its system entry points;
  - `icons.rs` and `freedesktop.rs` lose the indicator loaders for good (S2.7) and their links to `SystemTheme`;
  - a split keeps only what linked code calls; an item whose only callers are unlinked goes with the reader part, so
    clippy's `-D warnings` finds no dead code.
- **Deleted** at the switch: today's `model/` (but `icons.rs`), `resolve/`, the preset table, `documented_sizes.rs`,
  `ThemeWidget`, `SystemTheme`, `AccessibilityPreferences` and their tests; `AnimatedIcon`, `spinners.rs` and the
  loaders that return it (S2.7, S19.3), with the READMEs' spinner animations, `scripts/generate_gifs_spinners.py` and
  its entries in `update_provenance.sh` and `generate_assets_release.sh`; the ten community preset files (the palettes
  are built from the projects' published colours, M2); `docs/inheritance-rules.toml`, `check_wrong_safety_nets.py` and
  their steps, after S19.2's conversion (S18).
- **Unlinked** at the switch (its `mod` line removed, the file kept): the readers and the watchers, linked again at
  M1b. Integration tests of unlinked code (`tests/reader_kde.rs`) are deleted and restored with it, the KDE
  reader's as its unit tests (`src/kde/tests.rs`: `kde` is `pub(crate)`, so no hidden test entry is kept; ADD-A146).
- **Under D10's decision** the existing ports and transcriptions (`gnome/accent.rs`, `kde/colors.rs`'s
  ports, `scripts/generate_qt_color_vectors.py`'s transcriptions and their output `kde/qt_color_vectors.rs`) are
  deleted at the switch and not restored; they are redone by the clean room.

An unlinked module is not compiled, so with the splits the reader features compile and the `--all-features`,
cross-target and per-feature steps keep checking the kept code. Code restored from the switch's parent commit has its
`platform-facts.md` citations rewritten as it is restored (below). The inventory also lists the connectors' showcase
entries unlinked at M2.1. It is `native-theme`'s alone: the four connector crates' public API gets its inventory,
item by item with a fate, in the connectors' design (M2.1; ADD-A146).

### One rule for every model field

A widget's part, a component, a token group, a colour-role group or a declared input enters the model in the commit
where every preset that exists then states it from verified facts, or declares its absence as the facts state; a preset
added later states every field the model then holds. So S12 stays strict, and S19.1's build test and the facts gate run
over the whole model at every commit. Generated declarations are staged by the generator's list of enabled units
(`scripts/generate_widgets.py`); hand-written ones (S3's roles, S4's tokens) are written in their unit's commit. The
facts gate skips a fact while one of its units is not enabled (the lanes run ahead; S17 gives a fact its key's unit,
the unit of every widget its `when`'s positive lists name and of the widget a `drawn_as` value names and, of its
combinations, those of the enabled units' names, ADD-A146, ADD-A147), reading the units from `scripts/widget_units.toml` as
`check_facts_trace.py` does (`tests/facts_gate/units.rs`). A fact *of the enabled units* is one whose units are all
enabled and which holds in a combination of their names that its state model draws (S17; ADD-A147). From M4
`check_facts.py` fails a fact whose `when` the names of all units together do not make hold (a stray fact, which
under S17's predicate is never a skipped fact of the enabled units, so a check of those would never fire), and the gate
a fact whose key lies in no unit or that names a name, a condition or a staged path no unit holds (ADD-A147), and
reports the other skipped facts by unit; so that it can tell a later unit's fact from a stray one, `widget_units.toml`
lists every unit with an enabled flag (*M3*). M4 requires every unit of 0.8.0's scope enabled (D13); each follow-up
release requires its own units enabled; after the last one no fact is skipped. A unit enters with every part, field
and value of its catalogue entries, in every combination of the names their **Depth** lines list, less what S6.1 lets
it defer to a later unit: a value (the `Placement` values, to the placement unit), a part's field or a block's leaf
(`transitions`, `IconStyle.animation`, to the motion unit), a name beyond its depth (to a unit of depth), a token
path nothing at depth reads (to the first unit that reads it, waiting in a parking unit). A rule under a context of
a widget a later unit holds, or drawing a part as one (`drawn_as`), waits for that unit, as `check_facts_trace.py`
states. A unit of depth obeys the same rule: it enters in the commit where every preset states its names, on every
entry then enabled, or declares, as the facts state, that its platform has none (ADD-A147).

Rejected: declaring unwritten widgets `available = false` (the presets would state that the platform does not draw
them, which is false); a lenient validation mode (it hides the gaps the gate exists to find); placeholder preset values
(invented); a deferred name declared but drawn as the base, or left out of the declarations with no answer of its own
(the first shows a look the platform does not draw for it, the second states that the platform lacks it; ADD-A147).

### What cannot build meanwhile: `PLAN_EXCLUDED`

- One variable in `scripts/check_release.sh` names each excluded step and the milestone that brings it back; the
  summary prints it, and excluded steps are skipped by name. A step that covers several crates (`check_features.sh`,
  the per-crate loops) skips only the listed crates, so `native-theme`, `-derive` and `-build` stay checked.
- The ledger states the expected list before the switch, by step (a section of `check_release.sh` can mix steps of
  different fates, so steps are named, not sections):
  - until M2.1: the connectors' library, test, docs and package steps, the "iced connector configurations" and "egui
    connector configurations" sections, and the connector-parity step (`check_connector_parity.py` is rewritten there);
  - until M2.6: `check_platform_facts_rows.py`, `check_preset_citations.py`, `check_todo_citations.py`,
    `check_section_refs.py`;
  - until M5: every step that builds a showcase example (the `--example showcase-*` steps in the cross-target and
    strict-panic sections) and their `ci.yml` copies, `tools/showcase-support`'s steps, "Showcases in CI's runner
    environment", the widget-coverage step and "Showcase parity (static)";
  - never: the README-versions check and every other step that does not fail at the switch.
- The switch's gate: the list equals that expected list. Any other failing step is fixed, or added only with the
  reviewer's agreement and its reason. Each milestone's full run also runs the listed steps once and removes those
  that pass.
- `ci.yml` follows with matrix entries, steps and `--exclude` arguments (the MSRV `--workspace` check), each marked
  `# PLAN_EXCLUDED: <name>`; a `check_release.sh` step checks that the marks and the list name the same set, in both
  directions. M5's gate: the list empty and no mark left.
- A unit not yet enabled adds no step: its fields are not in the model, the gate skips its facts, and the contract
  tables hold no entry for it, so no step fails for it. The list stays empty from M5 through every follow-up release.

### What carries over

- **Today's facts.** M1a imports today's `platform-facts.md` rows into `docs/facts/imported/`, keyed where a gap-map
  row and its trace line name the model path; the rows reference their evidence where it lies. It is lane input that
  neither the gate nor the citation check reads: the lanes turn each row into a verified fact like any other, or record
  it as not a model value, so nothing unverified reaches the gate. The `windows-11-arm` evidence stays usable; the
  Windows Server images' values are dropped where they differ. The folder is empty at M4: a row whose key lies in a
  later unit becomes a fact the gate skips until that unit is enabled.
- **Today's evidence.** The five runs under `docs/evidence/os-probe/` (about 6,400 files, of which today's rows cite a
  few hundred) are listed by run folder in `check_fact_citations.py` as legacy evidence: their files count as cited
  until M4, when every legacy file no fact cites is deleted from the evidence repository (*The evidence repository*;
  the history of both repositories keeps it) and the list is emptied. Rejected:
  deleting the uncited files now (a lane converting a row may need another render of the same run); a warning instead
  of a failure (nothing would catch an uncited file after M4).
- **`platform-facts.md`** becomes generated at the switch, from the verified facts only; today's presets and the
  `platform_facts_xref.rs` test go then. From M2's task *`docs/platform-facts.md` not committed* it leaves the
  repository, CI and each release generating it (D6, outcome 8; ADD-A147).
- **Citations of today's `platform-facts.md` by line.** They are rewritten to `platform-facts.md@<sha>:LINE`, `<sha>`
  being the commit `v0.8.0-rc1` is cut from: in M0 for the gap maps and `docs/todo.md`; at the switch for every other
  tracked file (the CHANGELOG included: an `@<sha>` makes a released entry's citation exact without changing its
  meaning), except `docs/archive/`, `docs/evidence/` (in the evidence repository from its switch) and the checker
  scripts that define the format. From the switch, a
  `check_release.sh` step fails on a `platform-facts.md:N` citation without `@` outside those exceptions. The probe's
  strings that cite the document become fact keys in the next spike that changes the probe.
- **Today's inheritance rules** are converted by S19.2's script at M1a into a staged input file that each unit's preset
  step uses and the gate checks; it is removed at M4: its entries are today's fields, whose new homes all lie in
  0.8.0's units (*0.8.0's scope*), so no later unit needs it.

## Where the work happens

Code tasks on disjoint files run in parallel, each in its own git worktree of `v0.8.0-rc1`, building its own
packages only (`-p`; the shared compiler cache serves a worktree's dependencies, and the task deletes its build folder
at its end), within the build slots the maintainer last set (*How the work runs*); tasks whose files overlap run in
series. The merges into `v0.8.0-rc1`, its commits and its gates stay in series, in the main checkout (ADD-A146: the
maintainer's instructions of 2026-11-06 and 2026-11-08). Rejected: one implementer at a time, the rule until then
(its reason was a whole second build of tens of GB per worktree, which the shared cache, `-p` builds and the
folder's removal at the task's end no longer cost); parallel implementers in one tree (they see each other's
half-written files). Everything else runs where it cannot change what a
code task builds or its gate reads (`docs/facts`, the presets, the probe):

- **Facts lanes and tie-breakers** each work in their own git worktree of `v0.8.0-rc1` and never run
  `cargo` there, so no build folder is created; they write TOML and run only the Python checkers. A lane that
  measures commits its runs in its own clone of the evidence repository (*The evidence repository*).
- **Verifiers** work in a scratchpad folder holding a generated brief (the unit's keys, the pins, the catalogue's
  state-model source lines and, for `measured` facts, copies of the cited evidence and the method's script), and
  write their output there. They read only that folder and the source cache, not the project, whose gap maps and
  `platform-facts.md` state values they could copy, nor the cache's evidence checkout (*The evidence repository*),
  whose analysis outputs they could copy too; the brief holds the evidence they need. That rests on the brief,
  which is why the quote check and the audit sample exist.
- **The orchestrator** compares, brings verified facts into `v0.8.0-rc1`, generates the unit's parts into the eight
  presets from them (`generate_preset.py <id> --write`, the presets' hand-written blocks with them; ADD-A146), and
  runs the gates there.
- **Tooling** (the probe's passes, the hosts, the harnesses, the vector generators, the workflow jobs) works on
  `v0.8.0-tools` in its own worktree, built and run on CI only (`gh workflow run os-probe.yml --ref v0.8.0-tools -f
  jobs=<names> -f images=<list>`; its draft pull request targets `v0.8.0-rc1`). `v0.8.0-tools` merges `v0.8.0-rc1`
  first; the orchestrator merges it back between code tasks.
- **The project's guards:** M0 checks that the panic and invented-value hooks fire for an agent working in a worktree
  (they are registered by a relative path and `.claude/` is git-ignored); if not, D9 covers making their paths
  absolute.
- Worktrees are removed when their work is merged.

### The evidence repository (D6's evidence item, decided by the maintainer on 2026-11-08)

Measured evidence moves out of this repository into a separate evidence repository, pinned by commit. At the decision
`v0.8.0-rc1` held 141 MB under `docs/evidence/` and its `.git` 388 MB, the iOS and macOS facts branches held 353 MB
and 175 MB of evidence, and M3a's six units add more; every clone of this repository downloads all of it, while only
the facts tooling reads it.

- **The repository:** `tiborgats/native-theme-evidence` on GitHub, public (anyone can check a measured fact, and CI
  fetches it without credentials), created by the maintainer. Its tree is this repository's `docs/evidence/` at the
  same paths (`docs/evidence/<harness>/<date>-run-<id>/<image>/`, each run's `README.md`, `run.toml` and `analysis/`,
  the folder's `README.md`), and at its root a `README.md` (what it holds, that this repository pins it, the rules of
  this section) and this repository's licence files. The rules on the files a run holds (S17 *`evidence`* and
  `check_fact_citations.py`'s: cited or an analysis's input, its README, its `run.toml`, its `analysis/`, and its
  records exempt by name, `settings.txt` and `environment.txt`, ADD-A136's rule) apply there unchanged.
- **The pin:** one `[[source]]` of kind `git` in `docs/facts/sources.toml` with `evidence = true`:
  `id = "native-theme-evidence"`, `repository`, `commit`, `tag = "commit"`, `generations = []` and a `note` (why it is
  pinned), and no `project`, `release`, `researched` or `proof` (the commit is this project's own, moved by its own
  facts commits, which is no re-pin: D1 freezes the platform sources; spec S17 *The evidence root*). The entry breaks
  three of `check_sources.py`'s rules today: every entry's `release`, `researched` and `proof` (`COMMON`), a non-empty
  `generations` and the `<name>@<version>` id (its per-entry form check), each of which the tooling task waives for
  the one `evidence = true` entry alone, which it requires to exist exactly once, with the id `native-theme-evidence`
  and `generations = []`; its `note` meets the rule that a git entry has a project, a site or a note.
- **Checkouts:** only the orchestrator's main checkout of this repository reads the evidence from the source cache:
  `scripts/update_sources.sh` checks the entry out at `<cache>/native-theme-evidence/` like every pin, `--verify`
  included. Every other worktree (a lane's, the tooling's, any branch whose pin differs from the main checkout's) sets
  `NATIVE_THEME_EVIDENCE` to its own evidence checkout, a clone at `.claude/worktrees/<branch>-evidence/` (git-ignored
  with the worktrees). `update_sources.sh` skips the evidence entry, `--verify` included, when that variable is set
  and in any worktree (`git rev-parse --git-dir` differs from `--git-common-dir` there), so a worktree that forgets
  the variable cannot move the shared checkout under a gate running in the main checkout; its readers then read the
  cache's checkout, at the main checkout's pin, and fail on a differing pin rather than read the wrong evidence. Three
  `update_sources.sh` changes are the tooling task's, not today's behaviour: the skips above; `--only <id>` fetches
  named entries alone (the whole cache is about 4.4 GB, and running the facts gate needs only this entry); a moved
  evidence pin is fetched into the existing checkout, so only the new runs download (today `fetch_one` fetches every
  entry afresh into `<cache>/<id>.tmp`).
- **The evidence root:** citations keep their `docs/evidence/...` paths. Every reader (today `check_facts.py`,
  `check_fact_citations.py`, `check_function_vectors.py`, `generate_dynamic_type.py` and the facts gate) resolves a
  path `docs/evidence/<rest>` to `<root>/docs/evidence/<rest>`, the root being, in this order: a test fixture's root
  holding its own `docs/evidence/` (the gate's fixtures, the checkers' self-tests), which needs no git checkout and no
  pin; else the directory `NATIVE_THEME_EVIDENCE` names (CI's checkout, a worktree's clone); else the pin's checkout
  in the source cache (`NATIVE_THEME_SOURCES`, else `scripts/citations.py`'s default; the gate holds the same
  default). Outside a fixture a reader fails, naming the root and the pin, unless the root is a clean git checkout
  whose HEAD is the pinned commit, so S17's "committed" means in that commit's tree. Clean means
  `git status --porcelain --ignored --untracked-files=all` prints nothing: ignored files count, since a reader walks
  the files on disk (the uncited check) whatever a user's excludes hide, and the evidence repository needs no ignore
  rules. Without a root it fails, naming `update_sources.sh` (with `--only native-theme-evidence` once the tooling
  task adds it).
  A reader that runs never skips the evidence it cannot find, since a gate that passes without its inputs is no gate
  (`check_release.sh --commit` still skips `check_fact_citations.py` by name where the source cache is absent, as *The
  new scripts* says). The analysis scripts run in the evidence repository's root, as they ran in this one's.
- **Branches and the pin's moves:** the evidence repository's branches follow this repository's. A branch that adds
  evidence commits its runs on an evidence branch of the same name, cut from the commit it pins, and its facts commit
  moves the pin to that branch's head; a worktree commits its runs in its clone before it checks (it may amend a
  commit it has not pushed). The evidence branch is pushed before the commit that pins it (CI's checkout fails
  otherwise). When a branch of this repository merges into another, the source's pinned commit is merged into the
  target's pinned commit (a fast-forward where one contains the other, else a merge commit, never a rebase or a
  squash, so every commit a pin ever named stays reachable), the result is pushed to the target's evidence branch
  (created where it has none), and the merge here pins the result, which also settles a conflict on the pin's line. An
  evidence branch is deleted once merged, or once its branch here is deleted unmerged. Rejected: one linear evidence
  branch every lane pushes to, each branch's checks scoped to the runs its own facts cite: a run no fact ever cites
  would then be caught only at the release's unscoped check, long after the lane that wrote it; with mirrored branches
  every branch's pin holds exactly its own and its merged branches' runs, so S17's "every evidence file is cited"
  holds on every branch, as it does today.
- **CI:** every job whose steps read evidence (today `ci.yml`'s test matrix, whose `cargo test -p native-theme` runs
  the facts gate, and its `parity` job's `check_function_vectors.py`; `dependency-canary.yml`'s tests; the macOS job
  of `os-probe.yml` that checks the SDK-header citations, once it runs `check_fact_citations.py`) checks the evidence
  repository out at the commit `sources.toml` pins (that commit alone, depth 1) and sets `NATIVE_THEME_EVIDENCE` to
  it, through one shared step, so the jobs cannot differ.
- **The step cache** (`check_release.sh --commit`, `--quick`) keys a step that reads evidence on the pin, in
  `sources.toml`: a reader accepts only a clean checkout at the pin, so the pin fixes the evidence it reads. The
  tooling task declares it for each such step (from the switch the traced Python checks read outside the repository).
- **History:** neither repository's pushed history is rewritten (an unpushed branch may be, below). The evidence
  committed here so far stays in this repository's history; the evidence repository deletes a file by a new commit,
  never a force push. From the switch, new runs go to the evidence repository only, and a `check_release.sh` step and
  CI's `parity` job fail on any tracked file under this repository's `docs/evidence/`.
- **The switch**, in this order:
  1. the maintainer creates the repository;
  2. its first two commits, on its `v0.8.0-rc1` branch with `main` at the same commit: the first holds `v0.8.0-rc1`'s
     `docs/evidence/` byte for byte at the commit the switch follows, its message naming that commit, and checked
     mechanically: `git rev-parse <that commit>:docs/evidence` here equals
     `git rev-parse <first commit>:docs/evidence` there (the same tree); the second adds the root README and the
     licence files and restates `docs/evidence/README.md` for its new home (it says the runs are kept "in the
     repository");
  3. the tooling task's branch: its first commit adds the pin, naming the second commit, with `check_sources.py`'s
     waiver for the evidence entry (without it `check_sources.py` fails every later commit and `update_sources.sh`,
     which refuses a `sources.toml` with form problems, cannot fetch the evidence) and CI's shared checkout step; then
     the readers switch to the evidence root one by one, each commit still holding this repository's `docs/evidence/`,
     so a reader not yet switched reads it and a switched one reads the checkout, whose files step 2 proved the same;
     then `update_sources.sh`, the rest of `check_sources.py`, the step cache, the documents (`CONTRIBUTING.md`, whose
     link into `docs/evidence/` points at the evidence repository; `docs/new-os-version-guide.md`, whose recording
     steps copy a run's artifacts into the evidence repository; `scripts/README.md`) and `check_pinned_citations.py`'s
     exemption of `docs/evidence/` dropped; last, `docs/evidence/` removed, with the guard. So every commit of the
     branch has its readers' evidence. The working documents' and the imported rows' `docs/evidence/` paths stay as
     written, naming files by the root rule above (`docs/archive/` is never edited);
  4. the branches open at the switch land as the next item says.
- **Branches open at the switch:** their evidence goes to the evidence repository, not into this repository's history.
  Each one's changes to `docs/evidence/` since its merge base with `v0.8.0-rc1` are applied on its evidence branch,
  cut from the switch's pin (the paths are the same, so the diff applies as it stands), and pushed. Then:
  - a branch never pushed (on 2026-11-08 `facts-ios-m1b3` with its sub-branches, `facts-macos-m1b4`,
    `checkfacts-perf`, `facts-hosts-m1b`) has `docs/evidence/` filtered out of its own commits, every commit and
    message kept, the branches rewritten in one run so that commits they share stay shared. Approved by the
    orchestrator (2026-11-08): the no-rewrite rule protects pushed history only, and no one else holds these branches;
  - a pushed branch holding evidence (on 2026-11-08 `v0.8.0-tools-probe-m2` with 72.6 MB, `v0.8.0-tools-ios-m2` with
    22.8 MB, `v0.8.0-tools-hosts-m1b` with 0.3 MB) is not rewritten: its own commits are cherry-picked in order, each
    without `docs/evidence/` and with its message kept, onto a new branch, which merges in its place, and the old
    branch is deleted, here and on GitHub. Leaving the files out of a merge would not do: the merge's parents would
    still carry them into `v0.8.0-rc1`'s history (about 96 MB for these three);
  - a commit on the branch (or its copy) then moves its pin to its evidence branch's head, and it merges as above.

  Rejected: merging them as they stand (about 620 MB more in every clone for ever, the growth the decision stops); a
  squash merge (it loses the lanes' commit messages, which record their reasoning). They wait for the switch: merged
  before it, their runs would enter this repository's history.
- **Rejected:**
  - a git submodule at `docs/evidence` (a pin git understands, `git clone --recursive`): Cargo clones a git
    dependency's submodules, so every user depending on `native-theme` by git would download the evidence; and it
    would be a second pinning mechanism beside `sources.toml`, outside `update_sources.sh --verify` and the checkers'
    pin check;
  - the evidence tree at the repository's root (`os-probe/...`, the prefix stripped): the analysis outputs'
    `input_image`, the scripts' usage lines and the open branches' diffs name `docs/evidence/...` paths, so each would
    need rewriting or a second mapping, against two folder levels with nothing else in them;
  - one repository per platform or harness: more pins and checkouts for no gain at this size;
  - an id with a version (`native-theme-evidence@<eight hex digits>`, as other untagged pins): a new cache folder and
    a full download at every pin move, the old folders left behind;
  - importing the evidence's history into the new repository: it stays readable here, and a first commit naming its
    source commit is enough;
  - a reader accepting a root that is not clean or whose HEAD differs from the pin, with a note: "committed" would
    have two meanings, and a check could pass on evidence no commit pins;
  - every worktree reading the cache's one checkout: it can be at one pin only, and branches pin different commits;
  - keeping it here and Git LFS (D6's other options).

## Decisions

### For the maintainer (at approval; D13 later)

- **D1 — the source cache and the release sets.**
  - **Location (decided 2026-10-29):** `/run/media/tibi/work/cargo-build/native-theme-sources/`, a folder beside the
    workspaces' build folders on the work SSD: inside the maintainer's standing read and write grant for
    `cargo-build`, outside `/home`'s snapshots, and outside every project's own build folder, which is all
    `sweep-target.sh` cleans. `update_sources.sh` can rebuild it from `sources.toml` at any time. Rejected: a git-ignored
    folder inside the project (several GB in `/home`'s snapshots); the session scratchpad (lost at reboot). M0 seeds it
    from this session's scratchpad checkouts whose commit equals the pin.
  - **The release set** per platform: the newest stable at M0 (R1.5), coherent, frozen until 0.8.0 unless the ledger
    records a re-pin; under D13, frozen until the catalogue's last unit is released, so every unit's facts describe
    one release set (a re-pin, recorded in the ledger as before, re-checks every enabled unit's facts at the new pin;
    each release's drift run, M4, shows how far the runners have moved).
    KDE: the Plasma release, KDE Frameworks, Qt, Kirigami and kirigami-addons one dated Arch Linux
    Archive snapshot carries (the container D2 measures in; today Plasma 6.8.2). GNOME: libadwaita, GTK and the
    gsettings schemas (today 1.10.0 and 4.24.1). Material: material-web's token set (the primary reference, R1.6), a
    Compose release, the Android emulator's system image (API level and revision) and material-color-utilities by
    commit (Compose carries its own colour code). Windows: the microsoft-ui-xaml commit of a Windows App SDK release,
    and the Windows Community Toolkit's release. Apple: as D12 decides, with the Xcode per generation (the runners'
    images move within a release, so each evidence file records its exact build). A newer patch at M0 is fetched instead of today's. Recorded in `docs/facts/sources.toml`,
    one pin per source and generation. Rejected: `master` commits (code no user runs and no harness can run);
    re-pinning per fact (every lane redoes the mapping and versions mix).
  - **What is pinned:** every platform source the core, the catalogue, the traces, the gap maps and `platform-facts.md`
    cite. Other cited code (the toolkits gpui, gpui-component, iced and egui; the project's own files, cited at a
    project commit) is listed in `sources.toml` as not pinned, with its reason. `scripts/check_sources.py` fails on a
    cited project in neither list. A web page is pinned as a commit of its public source repository where it has one
    (Microsoft Learn's MicrosoftDocs repositories), else as a dated snapshot with its SHA-256 (Apple's pages as their
    DocC JSON; Material, Fluent, Android, KDE and GNOME pages); Apple's HIG has no versions, so a snapshot serves only presets
    describing the release current when it was taken (D12, spec S17). Apple's SDK headers exist only on macOS runners, so their citations are checked by a macOS
    job of `os-probe.yml` at the pinned Xcode.
- **D4 — `unsafe` and the measurement hosts** (approved by the maintainer, 2026-10-29).
  - The readers' and watchers' new reads (the traces' `reader` lines on macOS and Windows) grow under their existing
    module exceptions (`macos.rs`, `windows.rs`, `watch/macos.rs`, `watch/windows.rs`), in the same form.
  - The probe's measurements grow under its existing exceptions (`tools/os-probe/src/macos/ffi.rs`, `windows/ffi.rs`).
  - The hosts, none with Rust `unsafe`: iOS, a small Swift app built by Xcode on the runner (rejected: Rust with
    objc2-ui-kit, a third `unsafe` module for a tool); Android, a Kotlin Compose app; KDE, the `qml` runtime with QML
    files (qqc2-desktop-style, Kirigami) and the snapshot's own `pyside6` for Qt Widgets with the Breeze style (pip's
    PySide6 brings its own Qt, which cannot load the snapshot's Breeze plugin); GNOME, a C program modelled on
    libadwaita's `doc/tools/screenshot.c`; WinUI, a small C# Windows App SDK app, for S13's WinUI placement strategies
    (microsoft-ui-xaml's placement code runs only inside a WinUI app) and for state values if spike W shows the XAML
    does not state them.
- **D5 — `v0.7.1-rc1` to `main` first** (approved by the maintainer, 2026-10-29): the maintainer merges the v0.7.1 documents and scripts (no code
  change) before `v0.8.0-rc1` is cut, so `main`'s ROADMAP states the next work and the 0.8.0 pull request holds only
  0.8.0's work. The alternative, cutting from `v0.7.1-rc1` and merging both at 0.8.0, also works.
- **D7 — the budget for M0–M1b** (approved by the maintainer, 2026-10-29; the cap lifted by the maintainer on 2026-10-30): a cap of +15,000k tokens until then, and no token cap since. D6 set none (outcome 10; ADD-A147): the work runs to M2's close, whose report gives M2's cost, the gate's time and the load cost, and M3a's approval is asked there with the first real unit costs, where the maintainer may set caps per milestone; no lane writes facts ahead for M3b's units meanwhile. An estimate, not a measurement: one
  independent review of this plan used 180k to 270k tokens (eleven so far); M0–M1b holds six spikes, eight presets' slice facts with
  their verifiers, two implementation milestones with a reviewer per task, and the reviews.
- **D9 — the project's own guards** (the maintainer's configuration; the recommendations approved, 2026-10-29).
  - `.claude/hooks/no-invented-values.sh` blocks Rust lines with coefficient-like literals (`* 2.0`, `0.299 … 0.587 …
    0.114`), which a faithful implementation of a platform function contains (CAM16, `kde_effects`, Qt's colour
    helpers). Recommended: it exempts the directories holding the platform functions and the placement strategies
    (`*/src/next/platform/*`, `*/src/platform/*`, `*/src/placement/*`), as it already exempts tests, since
    `check_function_vectors.py` proves that every public function there has a vector table. Rejected: a
    `// reference:` comment per function, which the hook cannot see in an edit's fragment.
  - `.claude/hooks/no-runtime-panics.sh` allows `unsafe` only in the probe's two `ffi.rs` files; it gains the existing
    module exceptions (`macos.rs`, `windows.rs`, `sficons.rs`, `winicons.rs`, `watch/macos.rs`, `watch/windows.rs`)
    and S15's module (its path added when M2 creates it).
  - The hooks' paths are made absolute if M0 finds they do not fire in worktrees.
  - `.claude/agents/preset-validator.md` and the `platform-fact` and `add-preset` skills, which describe today's
    `platform-facts.md`, inheritance rules and preset format, are rewritten at the switch.
- **D10 — licences.** This states a risk, not a legal conclusion; the maintainer decides.
  - **The facts.** The crate is MIT OR Apache-2.0 OR 0BSD. The platform functions' references are LGPL
    (`colorsapplicator.cpp` LGPL-2.0-only; `kcolorscheme.cpp`, kguiaddons' `kcolorutils.cpp` and Kirigami's
    `colorutils.cpp` LGPL-2.0-or-later; Qt's `qcolor.cpp`; libadwaita) or Apache-2.0 (material-color-utilities). The
    specification defines several functions only by their reference (`adw_standalone`, `m3_role`, `kde_accent`).
  - **It exists today.** 0.7.0 already ships ports that say so: `gnome/accent.rs` (libadwaita's
    `adw_rgba_to_standalone`) and `kde/colors.rs` (`KColorUtils::mix` and `overlayColors`, Kirigami's
    `linearInterpolation`, `QColor`'s HSV arithmetic and 16-bit storage); `scripts/generate_qt_color_vectors.py` holds
    Python transcriptions of two of them. The readers' format parsers that follow Qt (`kde/fonts.rs`:
    `QFont::fromString`'s field order, the legacy-to-OpenType weight table) are most likely interoperability facts
    rather than ported code; they are listed so the decision covers the whole list. The decision applies to all.
  - **(a) A two-role clean room** for LGPL references: one agent reads the code and writes a behavioural description,
    which the orchestrator puts in the catalogue (the core's function rows, written by agents who read the code,
    already serve as such descriptions for some). An implementer works as a verifier does, in a scratchpad folder
    holding only the description and the vector table (laid out as `<folder>/src/platform/<name>.rs`, which D9's
    pattern covers), writes one self-contained file, tests it with plain `rustc --test` (pure arithmetic: no crate, no
    build folder) and never sees the project, the source cache or git history; the orchestrator moves the file into
    `platform/`; the vectors prove equivalence.
  - **(b) A maintained crate** that passes the vectors, which moves the question to that crate.
  - **(c) A port with the reference's notice**, which conflicts with offering the crate under MIT or 0BSD alone.
  - **Apache-2.0 code** (material-color-utilities) can be ported, but the ported file stays under Apache-2.0 with its
    notice whatever licence a user picks for the rest: the crate's `license` becomes `(MIT OR Apache-2.0 OR 0BSD) AND
    Apache-2.0`, and the README's licence section names the file. Rejected: keeping the field and stating it only in a
    NOTICE file, which leaves the crate's metadata wrong.
  - **Decided by the maintainer (2026-10-29):** (a) for every LGPL reference, no third-party crate; an own port of
    material-color-utilities under a file-level Apache-2.0 licence; the existing ports and transcriptions redone by
    (a); the bundled KDE port files shipped with their licence notices; the tools written from the settings the references pin, not their
    code, though a CI tool may compile a pinned upstream source unchanged to call an internal function (it never ships
    in the crate).
  - **Extended at D6** (2026-11-10, outcome 11; ADD-A147; open to the maintainer's veto, veto list item 41): D10 names
    four classes of reference. An LGPL or a GPL reference goes through the clean room, (a): the clean room carries
    no code of the reference, so it serves a GPL reference exactly as an LGPL one (the motion unit's `Breeze` engine
    follows Breeze's kstyle, GPL-2.0-or-later). An Apache-2.0 or an MIT reference may be ported, the ported file under a
    file-level licence with its notice (`src/platform/android_non_linear.rs` and `src/ripple.rs` port AOSP and
    androidx under Apache-2.0, `src/platform/win_ui.rs` microsoft-ui-xaml under MIT). The crate's `license` is the
    manifest's expression as it stands, `(MIT OR Apache-2.0 OR 0BSD) AND Apache-2.0 AND MIT`
    (`native-theme/Cargo.toml`). A risk judgement, as D10's first part is, not a legal conclusion: accepting "AND
    Apache-2.0" for a ported file already gave up a crate usable with no notice to carry, and MIT's obligation (keep
    the notice) is of the same kind and no wider. Rejected: no code that carries a notice (the MIT and Apache-2.0
    ports redone through the clean room, the licence back to `MIT OR Apache-2.0 OR 0BSD`).
- **D11 — 0.7.x releases during the work** (approved by the maintainer, 2026-10-29): no planned 0.7.x release; `docs/todo.md`'s `[v0.7.x]` items
  are triaged into this plan or carried past its releases, and ROADMAP's *Also in the 0.7.x releases* is updated at M0.
  An emergency (an upstream release breaking 0.7.0, as gpui-kit 0.6.4 broke 0.5.8) is fixed on its own `v0.7.N-rc1`
  from `main` (`v0.7.2-rc1` for the next), released by the maintainer, and `main` is then merged into `v0.8.0-rc1` with its CHANGELOG entry kept
  below `[0.8.0] - Unreleased`. Rejected: planned 0.7.x releases beside the work (two lines of releases for months).

- **D12 — which Apple release `macos-tahoe` and `ios-26` describe.** R1.5 says a preset describes the newest release
  of its generation, and the gap maps put 27 inside it (`macos-tahoe` "26–27"; rows citing iOS 27.1 and 27.2; 27-only
  catalogue entries such as pull-to-refresh and `@expanded_interface`). But the specification's `version_match` calls
  macOS 27 under `macos-tahoe` `PresetOlder` (S16), GitHub offers macOS 27 only as a preview image (`xcode-27`, run with
  `continue-on-error`), and Apple's unversioned pages, snapshotted at M0, describe 27.
  - **(a) 26.x** (decided by the maintainer, 2026-10-29): measured on the generally available `macos-26` image and its iOS 26 simulator, and
    consistent with S16. Then R1.5's wording is amended to "the newest release a supported runner measures" (a
    rationale change, hence the maintainer's), the 27-only catalogue entries move to `docs/todo.md`'s macOS 27 entry
    (a class 2 spec change), and an Apple page counts for a 26 fact only when an archived snapshot from 26's lifetime,
    the pinned Xcode 26 headers or a measurement confirms it. Against: a user on 27 gets a preset one release behind,
    which `version_match` reports.
  - **(b) 27**: R1.5 as written. Against: measured only on a preview image that fails without failing the run, so
    facts may describe a beta; S16's `PresetOlder` example and the 26 facts already gathered change.
  - **(c) both**, a fact per release: the per-release overlay R1.5 rejected.

- **D13 — 0.8.0's scope** (decided by the maintainer, 2026-11-06: (B)). 0.8.0 ships when the new model covers what
  0.7.0 covers: every widget, colour role, token and value 0.7.0's model holds is in the new model, on every preset
  0.7.0 ships for, a widget on every preset whose platform draws the widget (restated at D6, outcome 6; ADD-A147;
  veto list item 48),
  with the connectors running on it. The rest of the catalogue follows in later releases, the
  follow-up releases ("0.8.x" in the decision; D14 numbers them). "Cover everything visible" stays the goal; only its
  release timing changes. Within each unit a release enables, the rigor stays as it is: verified facts, the facts
  gate, one rule for every field.
  - **(A)** the whole catalogue in 0.8.0, as R21 had it. Against: users wait for the widgets 0.7.0 never had.
  - **(B)** 0.8.0 at 0.7.0's coverage, then the other units. For: users lose nothing of 0.7.0's model but the
    invented light variants (R19), the further losses D6 records on R21's list (the `button` values 0.7.0 stated with
    no source or against its own cited source, and those its preset's reference rule restates; the look of a button
    with no variant on `material-3` and `ios-13`, 0.7.0's staying under `tinted` and `@gray`; `material-3`'s checked
    text-button colours; of the two motions 0.7.0 has, one the Apple motion trial does not measure; the icon size of a
    context the platform names none for; the icon theme's name on the presets whose icon set is not freedesktop:
    ADD-A147, veto list items 5 to 8 and 32 to 39) and the widgets a preset declares unavailable where 0.7.0 resolved
    them (`tab`, `card`, `expander`, `status_bar`), which a connector draws by its contract's list (S21.3; D6, outcome
    6; ADD-A147) (what its API does not keep is *Not kept*, below; ADD-A146); they get the new model about 2–3 weeks
    earlier (the maintainer's estimate: about 3–4 weeks against 5–7). Against: several releases, each with its own gate run; D14 (*Settled by
    logic*).
  - **(C)** releases by capability (rules, then shadows, …), R21's rejected stages: each would break the connectors
    again and show mixed looks.
  - Consequences: R21's release decision is amended (rationale, R25.13's list); the specification's release
    statements are restated through *Spec changes* (an S22 line) before M4 builds on them: S21.1 (one release, every
    preset complete) and S17's "0.8.0 is released with none skipped" and "with no pending fact" (S17's *While 0.8.0
    is built* then covers the follow-up releases, each released with no skipped or pending fact of the enabled units,
    *One rule for every model field*); the scope is *0.8.0's scope* below; the release set stays frozen until the
    last unit (D1); M3 splits into M3a (before M4) and M3b (*After 0.8.0*); M4, M5 and M6 apply to 0.8.0's units; the
    v0.7.1 documents and the scripts the later units use stay until the last follow-up release. `ios-26` stays in
    0.8.0 (S21.1 makes it a release condition, and its slice facts are written): every preset that exists states every
    field the model holds. By D6 (2026-11-10) the criterion applies inside a unit too: 0.8.0 states each entry at the
    depth 0.7.0's model and readers distinguish (*The depth 0.8.0 states*), and of motion the dialog's presentation
    and the busy indicator's loop, each where the Apple motion trial measures it (M2's task 1) (ADD-A147; veto list
    item 1).

### Scheduled

- **D6 — the checkpoint after M1b** (spec batch ADD-A147). The maintainer was to approve the scale-up from the slice's
  figures and the choices the document check of 2026-11-09 added (ADD-A146). Its fifteen outcomes are settled by
  logic from the maintainer's earlier decisions (D10, D13, D14, R19), the project's rules and the slice's
  measurements, three design notes working out those that needed design (the D6 staging, load and register
  designs), each checked independently until a check found nothing serious. Each outcome is the work's basis and
  open to the maintainer's veto, which the D6 veto list numbers. **The D6 notes**, the inputs of M2's tasks, are in
  the repository, and every mention of them in the documents means these files: the outcomes
  (`docs/todo_v0.7.1_model-completeness-d6-outcome.md`), the D6 staging design (`…-d6-staging.md`), the D6 load
  design (`…-d6-load.md`), the D6 register design (`…-d6-register.md`) and the veto list (`…-d6-veto-list.md`),
  each copied unchanged from the working files where it was checked, with ADD-A147's fix round's corrections; they
  are archived with this plan at the last follow-up release (*After 0.8.0*). Their scripts stay outside: the staging task reproduces the figures with
  the project's own tooling (ADD-A147). Where each now stands:
  - **The slice's cost, and the caps.** **Decided at D6** (2026-11-10, outcome 10): no cap is set now. The work runs
    to M2's close, whose report gives M2's cost, the gate's time and the load cost, and M3a's approval is asked
    there with the first real unit costs; no lane writes facts ahead for M3b's units meanwhile (D7; *M2*'s milestone
    gate; *M3*; *How the work runs*; veto list item 51). A cap from an estimate last wrong by a factor of 12.8 (the
    slice's measured usage against the estimate at the plan's approval) would not bind or would trip at once.
  - **The evidence's size.** Where it lives was decided by the maintainer on 2026-11-08: a separate evidence
    repository pinned by commit (*The evidence repository*), over keeping it in this repository (recommended up to
    about 500 MB, which the open facts branches already pass) and Git LFS (rejected for its quota and bandwidth
    costs).
  - **The facts gate's run time and its limits.** **Decided at D6** (2026-11-10, outcome 9): 3 h a shard on CI,
    1 h and 12 GB a local preset step, the release run one unsharded local run (the ledger's queue #213), reviewed
    with the first widget unit's measured times (*The facts gate's cost*; veto list item 25). It sets no budget for
    the build's time or the docs.rs build.
  - **The presets' generation, the audit rate and the plan's amendments.** **Decided at D6** (2026-11-10,
    outcome 13): they stand. Every bundled preset is generated from its verified facts (`generate_preset.py`, its
    `--check` a step of `check_release.sh` in every mode); hand-written stay `[preset] generation`, the `[declare]`
    keyed namespaces and the `read` facts' values, in the generator's marked blocks, and S19.2's bases (veto list
    item 44). The audit sample is 5 % of each lane's agreed facts per unit, at least ten, every measured fact
    recomputed (*How values are checked*; item 45): the audits found 0 wrong of 430 and 1 of 74, the recomputation
    the larger defects. The document check's amendments of this plan stand where each is marked `(ADD-A146)`
    (item 43).
  - **The register's open entries, by kind.** **Decided at D6** (2026-11-10, outcome 14; the D6 register design):
    of the register's rows of kind `d`, the fifteen choices of model form the checkpoint examined are fourteen
    settled from the sources (U504 and U180 in part; U180's open rest split off as U753, Qt Quick's line position,
    its form a field on `Decoration`, due with the flavours unit, and as U754, QPainter's wavy and strike lines, due
    with the first unit that states one) and U555, moved with `single_click_activates` to the containers
    unit (*The units 0.8.0 enables*); U473 is by D13's criterion the parking unit *behaviour*'s, 0.7.0's model
    holding no key repeat; the outcomes are in the ledger, each open to the maintainer's veto (veto list items 3, 7,
    8, 26 to 30 and 52).
  - **The slice's pending facts and the depth of a unit** (ADD-A146): whether the slice's 45,766 pending facts of
    74,029 (the eight `[pending]` tables 1,508 paths, the register 235 rows at M1b's close) are closed before the
    scale-up, and whether 0.8.0 states each widget at the depth 0.7.0's model distinguishes, the further variant
    sets deferred to a named later unit as S6.1 defers `Placement`. **Decided at D6** (2026-11-10, outcome 2; the D6
    staging design): each entry at that depth, the rest in named later units of depth (*The depth 0.8.0 states*;
    *One rule for every model field*), the facts already written staying and `ios-26` staying (D13). So 1,381 of
    the pending facts and 687 of the listed paths stay on the path at the slice's scope, the four small tables M2's
    task 0's, the rest closed by each unit's pending condition (*M3*) and at M4 (veto list item 1); the 19 pending
    `token.caret` facts and the pending facts the lanes wrote ahead for M3a's widgets return to it with their units.
  - **`docs/platform-facts.md`** (the ledger's queue #170): whether the generated file stays committed. It is
    50,121,732 bytes since `72dd07b4`, where a push to GitHub is warned at 50 MiB and blocked at 100 MiB (GitHub's
    documentation, as read for D6), and grows with every measured fact; the largest facts file,
    `docs/facts/ios/button.toml`, is 34,075,260 bytes. **Decided at D6** (2026-11-10, outcome 8): not committed; CI
    and each release generate it, the gates running the generator and its self-test; `check_facts.py` enforces a
    split of a facts file by key prefix past 40 MB; the local tools that cite its lines read the facts or a
    generated copy (R22 asks that the readable tables are generated from the facts, not that a copy is committed,
    and GitHub renders no file of that size, so the committed copy has no reader). One task early in M2 (*M2*,
    *`docs/platform-facts.md` not committed*); until it merges, the file stays committed and
    `generate_platform_facts.py --check` stays in the gates (veto list item 50).
  - **The model's cost to an application,** which no gate measured: per preset, in a release build, the load, the
    resolution, the query, the memory and the bytes a preset adds, beside 0.7.0's figures. **Decided at D6**
    (2026-11-10, outcome 1; the D6 load design): the slice misses any budget (four presets profiled loaded in 259 ms
    to 17.6 s and peaked at 18 to 1,827 MiB above the load's start, where 0.7.0 loads in under 1 ms), so the budget
    is *The load budget*, its numbers applying as proposed until the maintainer changes one (veto list items 9 to
    23), and the repair is M2's load path, before M3a's first widget unit (item 46).
  - **D10's licence classes:** whether D10 covers what the crate ships (an MIT port, `src/platform/win_ui.rs`, and
    a GPL reference, Breeze's kstyle for the motion unit's `Breeze` engine). **Decided at D6** (2026-11-10, outcome 11): D10
    names four classes (*D10*, *Extended at D6*; veto list item 41).
  - **The readers unit:** whether it belongs to M3a, its stated need having been met at M1b. **Decided at D6**
    (2026-11-10, outcome 4): it moves to the start of M3b, and M3a keeps each unit's own readers step (*The units
    0.8.0 enables*; *M3*): the comparison of all four readers with 0.7.0's at `2796b26e` found no read whose target
    lies outside 0.8.0's other units (veto list item 47).
  - **The rationale's amendments** of the document check (ADD-A146). **Decided at D6** (2026-11-10, outcome 12):
    the five stand, as the rationale now states them: `ios-*`'s accent alone, what UIKit draws in the tint a measured
    derivation over it, U706's second-tint run scheduled in M2 (R2.6, R4, R19); a palette suspended, not refused,
    under a followed scheme it lacks (R19); the ten bundled palettes 0.7.0's ten schemes, each the variant with
    0.7.0's accent (R19); `#[non_exhaustive]` error enums (R21); no font DPI read where the toolkit's scale carries
    it, the KDE reader's `Xft.dpi` read going (R10) (veto list item 42).
  - **The rationale's open questions** the document check left (ADD-A146), each answered by an outcome:
    - R-1, R21's release condition and R4's Apple list against the slice's numbers (the Apple presets held 45,538
      of the 45,766 pending facts): **Decided at D6** (2026-11-10, outcome 2): a unit staged by its depth (*The depth
      0.8.0 states*), `ios-26` staying in 0.8.0 (D13; the rationale's R4 and R21; veto list item 1).
    - R-3 with R-12, a typed role the platform names no counterpart for: **Decided at D6** (2026-11-10, outcome 5):
      one rule, native-theme's labelled mapping, outside the gate's platform comparison, never absent and never a
      stated fact, where a connector maps the role to a toolkit field that cannot be left empty, no stated fact
      gives it and the platform has a keyed colour of its own to pair it with (veto list item 56); on `macos-*` and
      `ios-*` the status roles refer to the keyed system colours as 0.7.0 drew them, and on `macos-*` the status
      text colours to `labelColor`, the pairings listed in the catalogue's *global roles* (`status`) for the
      maintainer's veto; the text slots' mappings are labelled alike; R25.17 names `ios-*` too (the rationale's
      R5.1, R10, R25.17; the specification's S3, S4.4, S17; veto list item 3). U456 closes with the pairings, the
      iOS status text colour's pairing split off as U755.
    - R-9, motion not weighed for deferral as placement was: **Decided at D6** (2026-11-10, outcome 3; the D6
      staging design): a motion unit after 0.8.0, 0.8.0 keeping the two motions 0.7.0 has where the Apple motion
      trial measures them (M2's task 1), Apple stating no timings (at `7430a0a2` 21 of the 27 macOS and 43 of the 48
      iOS motion facts pending, by the D6 staging design's count) (veto list items 2 and 6).
    - K4-2, what a palette states for the keyed roles a preset's values read: **Decided at D6** (2026-11-10,
      outcome 7): over a preset whose widgets read keyed colours, a palette reaches them by a stated mapping method,
      or it is not offered over that preset, built with the palettes in M2's task 4; nor is it offered over a preset
      whose widget colours are measured literals that follow no role (`macos-*`, `ios-*`), a recorded limit (the
      specification's S14, the rationale's R19; veto list item 49).
    - W3-3 with W5-15, what a connector draws where a preset declares unavailable a widget 0.7.0 resolved there
      (`tab`, `card`, `expander`, `status_bar`) or a stand-in widget whose unit is enabled: **Decided at D6**
      (2026-11-10, outcome 6): the declarations stay, D13's sentence restated to the presets whose platform draws
      the widget, and the connectors' contract lists per preset the entry a connector draws its toolkit's widget
      from (the specification's S21.3; *Widgets 0.7.0 styles through stand-ins*; veto list item 48).
  - **The documents' next check.** **Decided at D6** (2026-11-10, outcome 15): a full pass over all documents after
    the batch that applies D6, then each unit's batch as it is written (*Spec changes*; veto list item 53).

### Settled by logic (recorded)

- **D2 — all Linux measurement on CI.** The KDE, GNOME and Android harnesses run on CI's Linux runners in containers
  pinned to the release set, as jobs of `os-probe.yml`. Each container is built once from its pinned source and pushed
  to `ghcr.io` as a public package (free; spikes K and G check that it pulls without credentials); the jobs pull it
  (the Arch Linux Archive is a volunteer service). Nothing is installed on this machine, nothing touches the desktop,
  and the versions never drift. Rejected: this machine's Plasma (a rolling distribution moves past the pin during the
  work); local packages or a Flatpak runtime for GNOME (installs, and takes Plasma's portal settings); a GNOME OS
  virtual machine (needed only for what the shell draws, out of scope by R1.1); Paparazzi for Android (layoutlib is
  not the device's renderer).
  - **KDE:** the dated Arch Linux Archive snapshot, under `dbus-run-session`, `QT_QPA_PLATFORM=offscreen`. The accent
    script (`scripts/measure_kde_accent.sh`) uses a fresh config directory per (scheme, accent) pair, since
    `applyScheme()` without an accent takes a stored `AccentColor` (`colorsapplicator.cpp:127-136`), and checks that
    none is stored before a no-accent apply. It applies another scheme first (a fresh config already names Breeze Light
    and the tool then writes nothing; `applyScheme` deletes each colour group before writing it,
    `colorsapplicator.cpp:184`), then the accent in a second call (the tool takes the scheme or `--accent-color`, not
    both), and checks that kdeglobals changed.
  - **GNOME:** GTK 4 has no offscreen backend, so the harness runs on a headless display (spike G chooses between
    mutter's headless mode and Xvfb), with `GSK_RENDERER` pinned, `GDK_DEBUG=no-portals` (libadwaita's own tests keep
    portal settings out this way), and the colour scheme, contrast, accent and fonts pinned as `screenshot.c` pins
    them; the image is an Arch Linux Archive snapshot carrying the pinned libadwaita and GTK, or both built from their
    tags in the container.
  - **Android:** the emulator with KVM, at the pinned system image.
- **D3 — macOS, iOS and Windows on GitHub's runners.** The repository is public, so hosted runners cost nothing; the
  limits are concurrent macOS jobs (five) and the six-hour job limit. macOS 15 and 26 on `macos-15` and `macos-26`;
  iOS 18 on `macos-15`'s simulator and iOS 26 on `macos-26`'s (checked by spike I), as D12 decides; Windows 11 on `windows-11-arm`,
  which `os-probe.yml` already measures. Every job is in `os-probe.yml`, dispatched by hand, because
  `workflow_dispatch` needs the file on `main` (`--ref` runs the branch's version); a `jobs` input (an `if:` per job)
  and an `images` input (feeding the probe's matrix through `fromJSON`, since an `if:` cannot read the matrix) run only
  what a round needs. `macos-14`, `xcode-27` (macOS 27, not in the release set) and the two Windows Server images
  leave the default set at M0 (the import reads their committed evidence; `xcode-27` can still be dispatched by name).
  The runners are virtual machines; whether they have a GPU is probed once (spike M).
- **D8 — the ports:** every platform function and placement strategy is implemented in the crate, as D10 decides (no
  third-party crate), and proven by its vector table.
- **The package:** the facts gate reads `docs/facts` at run time (a directory of many files, which `include_str!`
  handles badly); `native-theme`'s package excludes `tests/` (the gate and the vector tables), and the packaging step
  fails above crates.io's 10 MB limit. Both are M2.5's to build (ADD-A146: the manifest has no `exclude` yet, and no
  step reads a `.crate`'s size).

- **D14 — the follow-up releases' version numbers** (settled by logic, 2026-11-06: (a); D13's "0.8.x" is the
  decision's wording for the follow-up releases, whose numbers follow SemVer). A follow-up release that enables a
  typed widget adds members to the generated enums `WidgetKind`, `Variant` and `TypedPart`, which are exhaustive (R21,
  S21.2). Cargo's SemVer rules count a new member of an exhaustive public enum as breaking, and Cargo resolves a
  requirement `0.8.0` to any 0.8.N, so an 0.8.N that adds one breaks every build that matches the enum in full: an
  application's, or an older connector release's built against the newer native-theme. Keyed components
  (`WidgetRef::Component(Key)`) and the placement unit (S13's types are in since M1a) may add none; the semver checks
  `check_release.sh` runs tell, per release. A unit of depth that adds typed variants adds members to `Variant`
  likewise; one of conditions, flavours, sizes or keyed names adds none (ADD-A147).
  - **(a)** the version follows `cargo semver-checks` against the previous release: a breaking release is the next
    minor (0.9.0, …), any other the next patch (0.8.N). For: R21's exhaustive enums and Cargo's rules both kept; D13's
    two reasons do not depend on the number. Against: the typed-widget releases are not 0.8.x as D13 names them.
  - **(b)** every follow-up an 0.8.N, the generated enums `#[non_exhaustive]`: a change of R21 and S21.2 (class 3).
    For: the names D13 uses. Against: a connector's build no longer breaks on a new widget (its contract test still
    fails on the widget's unmapped fields, S21.3), and every application matches with a wildcard arm.
  - **(c)** breaking 0.8.N releases: rejected, Cargo would pull them into the builds they break.
  - **Decided (a):** R21 makes the generated enums exhaustive so that a connector's build breaks on a widget it does
    not map yet, which (b) gives up for a name; D13's two reasons hold under any number.

## 0.8.0's scope (D13)

### What 0.7.0's model holds, and where it goes

Read at `2796b26e`, the switch's parent, where today's model still exists: its `model/`, `presets/`, `presets.rs` and
`resolve/` equal `v0.7.0`'s (`git diff v0.7.0 2796b26e` shows none), and `lib.rs` differs only by `pub mod next`.
Paths below are under `native-theme/src/` at `2796b26e`; `w:` is `model/widgets/mod.rs`. Each widget's fields map
as its catalogue entry's *Today's fields* says; the defaults and the text scale as the catalogue's *today's fields*
table (under *Colour roles and text ramp*) says.

| 0.7.0 group | At | New home | Unit |
|---|---|---|---|
| `ThemeDefaults`: the colours, `disabled_text_color`, the status colours, `shadow_color`, `link_color`, the selections | `model/defaults.rs:37-97` | S3's roles (`role.window.*`, `role.view.*` from `surface_color`, `role.global.*`, `role.selection.*`, `<set>.link`) | slice |
| `ThemeDefaults.font`, `mono_font`, `line_height`; `TextScale` (`ThemeMode.text_scale`) and `TextScaleEntry` | `model/defaults.rs:41-48`; `model/font.rs:298`, `:372`; `model/mod.rs:115` | S4.4's slots (`body`, `monospace`, `caption`, `headline`, `title1`, `display_small`) and each style's `line_height` | slice |
| `ThemeDefaults.focus_ring_*`, `disabled_opacity` | `model/defaults.rs:112-122` | `token.focus`; `token.params.opacity.disabled` | slice |
| `IconSizes`; `ThemeDefaults.icon_theme`, `Theme.icon_set`, `Theme.icon_theme` | `model/icon_sizes.rs:13`; `model/defaults.rs:145`; `model/mod.rs:299`, `:317` | `token.icons.sizes.*`, `token.icons.set`, `token.icons.theme` (S4.5) | slice |
| `DefaultsBorderSpec` (`ThemeDefaults.border`) | `model/border.rs:25`; `model/defaults.rs:102` | `<set>.border`; each part's own stroke, corners, shadow, padding (R20) | slice and each widget's unit |
| `ColorMode`, `Theme.light`, `Theme.dark` | `model/mod.rs:19`, `:274-278` | the `Scheme` condition (S7.1) | M1a's core |
| `AccessibilityPreferences` | `lib.rs:417` | conditions (S7.1) and `ReaderState.text_scaling_factor` (S9) | M1a's core, M1b |
| `Theme.name` | `model/mod.rs:270` | `Theme::preset().display_name()` (S16) | M1a's core (`Theme::preset`), M1b's close (`display_name`) |
| `WindowTheme` | `w:17` | `window` | window and chrome |
| `DialogTheme` (`DialogButtonOrder`, `DialogEntranceStyle`), `PopoverTheme`, `TooltipTheme`, `MenuTheme` | `w:817`, `w:659`, `w:304`, `w:259` | `dialog` (`action_layout.row_order`, `presentations`), `popover`, `tooltip`, `menu` | overlays |
| `ButtonTheme` | `w:43` | `button` | slice (M1a) |
| `LinkTheme`, `SegmentedControlTheme`, `SwitchTheme`, `CheckboxTheme` | `w:1058`, `w:946`, `w:710`, `w:183` | `link`, `segmented_control`, `switch`, `checkbox`; the checkbox's `radio_indicator_width`, `radio_dot_diameter` and shared colours → `radio` | buttons and toggles |
| `InputTheme`, `TextAreaTheme`, `ComboBoxTheme`, `SliderTheme` | `w:114`, `w:166`, `w:893`, `w:366` | `input`, `text_area`, `combo_box`, `slider` | fields |
| `SpinnerTheme`, `ProgressBarTheme`, `ScrollbarTheme` | `w:869`, `w:415`, `w:332` | `spinner`, `progress_bar`, `scrollbar`; `overlay_mode` → `cond overlay_scrollbars` (S7.1's condition, which the GNOME and macOS readers set; ADD-A146) | indicators; M1a's core, M1b |
| `ListTheme`, `TabTheme` (`TabIndicatorSide`), `SidebarTheme`, `ToolbarTheme`, `StatusBarTheme`, `SplitterTheme`, `SeparatorTheme`, `CardTheme`, `ExpanderTheme` (`ArrowSide`), `LayoutTheme` (`Theme.layout`) | `w:603`, `w:443`, `w:518`, `w:547`, `w:583`, `w:676`, `w:694`, `w:987`, `w:1003`, `w:1096`; `model/mod.rs:282` | `list`, `tab` (`indicator_edge`), `sidebar` (its rows `list @ variant sidebar`), `toolbar` (its font and icon size on `button @ variant tool, context toolbar.bar` where the preset declares `tool`, else `@ context toolbar.bar`; ADD-A147), `status_bar`, `splitter`, `separator`, `card`, `expander`, `layout` | containers |
| `FontSpec`, `WidgetBorderSpec` (inside each widget) | `model/font.rs:154`; `model/border.rs:89` | each part's `text`, `stroke`, `corners`, `padding`, `shadow` | each widget's unit |
| `AnimatedIcon` and its loaders | `model/animated.rs` | goes (S2.7, S19.3): a busy indicator is the `spinner` widget | indicators |

No 0.7.0 field maps into a typed widget the model lacks today or into a keyed component: their entries' *Today's
fields* are none, or name stand-ins that stay with the entries above (`badge`'s status colours, `form`'s `list.*`);
`component.title_widget`'s "`text_scale` (partial)" is where `kde-breeze`'s level fonts come from, and their home
is the slice's slots.

### The depth 0.8.0 states (D6)

Decided at the checkpoint D6 (2026-11-10, outcome 2; the D6 staging design; ADD-A147). An entry of an enabled unit is
stated with every part, field and value, less what S6.1 defers (the `Placement` values, the motion unit's fields and
values, and the values only names of later units read, which its **Depth** line marks), in every combination of the
names 0.7.0's model distinguishes for it, which its Depth line lists (the catalogue's *Entry format*). Those names
are the variants and state flags a *Today's fields* selector names for a target in the entry (in any entry's item),
those a state model requires for one so named and those a preset's `requires_variants` lists, the host contexts,
positions and conditions such a selector names, the default size in force, the scheme, the preset's default flavour
and `contrast_high` (model-wide: 0.7.0's Windows and GNOME readers give values under it), and `animations_off` and
`reduced_motion` for the motion values 0.8.0 holds. A typed colour role enters whole under the scheme, `contrast_high`
and the conditions the catalogue's *today's fields* table names; a token path, and a keyed name of any kind, is at
depth when 0.7.0 has it, a value at depth reads it, the core reads it, a connector reads it to draw a value at depth,
or a built system reader states it, and every other waits in a parking unit for the first unit that reads it. Every
other name (the further variants, sizes, host contexts, positions, directions, states and conditions) belongs to a
named later unit of depth, model-wide, and cannot be asked for meanwhile: a typed variant is no member of `Variant`,
a keyed one is declared `later`, a size or a flavour is answered `Later`, and a given name is mentioned by no rule of
a bundled preset, so an element under it resolves as without it (S6.1, S8, S10, S12). A fact is of the enabled units
when its `when` holds in a combination of those names that its state model draws, its part drawn there; every other
fact is a later unit's, and no fact is edited (S17). Rejected: the selector pairs of *Today's fields* alone (a
`primary` button's stroke or a button pressed while hovered would have no fact, and a block's leaves are stated
together); every state flag of the state models (1,674 pending facts at depth against 1,381, for flags 0.7.0's
button does not distinguish).

For `button` (the catalogue's Depth line): the variants `primary`, `toggle`, `with_icon`, `tool`, `tinted` and, on
`ios-*`, `@gray`; the states `hover`, `pressed`, `disabled`, `checked`, `default`, `focused` and `focus_visible` (on
`ios-*` `hover` and `focus_visible` the iPad unit's); the default size in force; no host context in the slice
(`toolbar.bar` from the containers unit); the scheme and `contrast_high`. On `material-3` `toggle`, and on `ios-*`
`tool`, are `later`: their state models draw them in no combination at depth.

**The counts** at `7430a0a2`, by the design's rule (the D6 staging design's scripts, with `button`'s
`label_position`, `repeat_delay` and `repeat_interval` at depth, ledger queue #258; counts by a stated rule, not measurements; a fact of two presets counted for each in the first table and once in the
second; "deferred" a fact of a later unit, "staged" a token path the rule stages):

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
facts, all measured), which stay with the roles: 72,309 + 1,720 = 74,029 facts, and 1,381 + 44,315 + 70 = 45,766
pending. "Other units'" are the facts the lanes wrote ahead for widgets not enabled. Pending facts at depth per preset:
`adwaita` 0, `kde-breeze` 5, `macos-monterey` 231, `macos-tahoe` 547, `ios-13` 328, `ios-26` 346, `windows-11` 54,
`material-3` 8. The listed paths that stay, by class:

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

Of the 1,508 listed paths at M1b's close 687 stay: 147 of 315 button paths, 270 of 278 colour-role paths (60 on each
iOS preset U456's status roles), both inputs, 268 of 913 token paths.

**The comparison with 0.7.0**, run for `button`: per preset and scheme, 0.7.0's value of every field its *Today's
fields* maps (and `toolbar`'s two mappings into `button`) beside the last fact at depth for the target in the mapped
combination (default flavour, default size, no idiom). Each difference is classed by one test:

1. *Equal*: the same value, or the same source's value in the model's form (a translucent fill or a state layer
   0.7.0 flattened over its surface; a padding measured from the box's edge with the stroke inside it, S2.11).
2. *0.7.0's sourced look, held by the facts under a name of the entry's own units that the depth defers* (a variant,
   a state): the name comes to depth (D13: users lose nothing of 0.7.0's model but the listed losses).
3. *0.7.0's value had no source* (none cited, a claim with no value or evidence, or a value its own cited source
   contradicts): corrected by the facts, and listed with R19's invented light variants on R21's list of what users
   lose.
4. *0.7.0 took the value from a source the preset's reference rule does not choose for it* (R1.6 for `material-3`: an
   older token set's value, or material-web's component's where Compose has the button; R1.3 and R1.4 for `ios-13`:
   WebKit's HTML button against UIKit's): restated by that rule, and listed beside the third class with both sources
   and the rule.
5. *0.7.0's sourced look that no name at depth can hold, or that only a name of a shared unit holds* (a flavour, a
   size, a condition beyond the depth; a shared unit is model-wide): a loss proposed to the maintainer, the shared
   unit named as where the look returns.

A changed look of the element with no variant, where 0.7.0 mapped it to a look that is not the platform's default and
the facts hold 0.7.0's look under a variant, is a correction of the mapping, listed with the corrections. The verdicts
(light; the dark scheme gives the same verdict on every row). Compose's files are
`androidx@e4d933c9 compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/`'s, UIKit's
`xcode@26.6 UIKit/`'s, material-web's `material-web@2.5.0`'s, WebKit's `html.css` is
`WebKit@7624.5.1.11.3 Source/WebCore/css/html.css`, and 0.7.0's sources are its preset comments and its
`docs/platform-facts.md` rows (`v0.7.0`, §2.3):

| Preset | Field | 0.7.0 | At depth | Class |
|---|---|---|---|---|
| `adwaita` | `icon_text_gap_px` | `8.0`, no line cited | `6px @ variant with_icon` (`libadwaita@1.10.0 src/stylesheet/widgets/_buttons.scss:626-628`) | 2 (`with_icon`), then 3: the value |
| `adwaita` | `font.color` and every text colour | `#2e3436`, its comment: no source at libadwaita 1.10.0 | `= surface.foreground`, the window's `#000006cc` | 3 |
| `adwaita` | `min_width_px` | `64.0`, its row: none in Adwaita's CSS | `= add(16px, add(part.padding.left, part.padding.right))` | 3 |
| `adwaita` | fills at rest, hover, pressed, checked | flattened over the window | `alpha(surface.foreground, 0.1)`, `0.15`, `0.3` | 1, over the corrected foreground |
| `kde-breeze` | `primary_background`, `primary_text_color` | inherited, the accent | the default button, `{default}` (the catalogue's *Today's fields*) | 3 |
| `kde-breeze` | `toolbar.font` (the reader's `toolBarFont`) | the reader's font | stated only under `variant tool` | 2 (`tool`) |
| `kde-breeze` | `min_height_px` | `32.0`, its row: none, sized to content | `auto` | 3 |
| `kde-breeze` | `min_width_px`, `padding_*` | `80`, `6` (`Button_MinWidth`, `Button_MarginWidth`) | `80 + 2·2 − 2·1.001`, `6 + 2 − 1.001` (`breeze@6.8.2 kstyle/breezestyle.cpp:3555-3564`) | 1 (Breeze's frame inside the box) |
| `kde-breeze` | `border.color`, fills | `kcu_mix` at frame contrast 0.2 | the same expressions | 1 |
| `macos-*` | `hover_background` | `#e0e0e5`, its row: measured, with no value or evidence | no `hover` in the macOS state models | 3 |
| `macos-monterey` | `active_background` | `#d6d6db` / `#4e4e50`, its row: AppKit publishes no pressed fill | `#f0f0f0ff` / `#ffffff5c`, measured | 3 |
| `macos-*` | `icon_text_gap_px` | `4.0`, its row: measured, with no evidence | `2px`, measured | 3 |
| `macos-*` | `min_width_px` | `64.0`, its row: none | `16px`, `34px` | 3 |
| `macos-*` | `font.color`, fills at rest and disabled, padding, `min_height`, shadow | the OS probe's | the facts', the same | 1 |
| `macos-monterey` | `primary_*`, `checked_*`, `disabled_opacity` | inherited | pending (U141, U612) | on the path |
| `ios-13` | the plain button's fields | WebKit's iOS button, `opaqueSecondaryFill` (`html.css:875`) | `@ variant @gray`, UIKit's gray configuration | 2 (`@gray`) |
| `ios-13` | the look of a button with no variant | WebKit's HTML button, the grey fill | UIKit's system button, no fill (`UIButton.h:22-23`) | a correction of the mapping |
| `ios-13` | `hover_background` | `#d1d1d6`, uncommented, beside its line "iOS has no hover state" | `hover` is the iPad's | 3 |
| `ios-13` | `active_background`, `disabled_background` | `#c7c7cc`, `#e5e5ea80`, uncommented | `@gray`: `#7878801f`, `#7878801f` | 3 |
| `ios-13` | `min_width_px`, `min_height_px` | `64.0`, uncommented; `44.0`, the HIG's minimum hit target | `auto`, `auto` | 3 (a hit target is no drawn height) |
| `ios-13` | `icon_text_gap_px` | `6.0`, uncommented | `0px @ variant @gray, with_icon` | 3 |
| `ios-13` | `active_text_color` | `#007aff` (`html.css:881-886`) | `@gray`: `#007affbf` | 4 (R1.3, R1.4) |
| `ios-13` | `font.color`, `disabled_text_color` | WebKit's | the same under `@gray` | 1 |
| `windows-11` | `disabled_background` | `#f9f9f900` / `#33333300`, where its row cites `ControlFillColorDisabled` | `= role.@control_fill_disabled`, `#f9f9f94d` / `#ffffff0b` | 3 |
| `windows-11` | `min_height_px` | `32.0`, its row: 27, derived, no `MinHeight` resource | `0px` | 3 |
| `windows-11` | `min_width_px` | `64.0`, its row: none | `0px` | 3 |
| `windows-11` | `hover_background`, `active_background` | flattened over `SolidBackgroundFillColorBase` | `ControlFillColorSecondary`, `ControlFillColorTertiary` | 1 |
| `windows-11` | `padding_*` | WinUI's 11, 5, 11, 6 | `add(11px, part.stroke.left.width)` and the other sides | 1 (the stroke inside the box) |
| `windows-11` | `icon_text_gap_px` | `8.0`, no line cited | pending (U517) | on the path |
| `material-3` | the plain button's fields | the filled tonal button, `v0_192` tokens | `@ variant tinted`: `secondary_container`, `on_secondary_container`, its state layer | 2 (`tinted`), equal |
| `material-3` | the look of a button with no variant | Material's filled tonal button | Compose's `Button`, the filled one (`Button.kt:77-88`; the tonal one is `FilledTonalButton`, `:439-447`) | a correction of the mapping |
| `material-3` | `disabled_background`, `disabled_text_color` | container `on-surface` at 0.12 (`tokens/versions/v0_192/_md-comp-filled-tonal-button.scss:42-44`) | `alpha(role.@on_surface, 0.1)` (`tokens/versions/latest/sass/_md-comp-button-tonal.scss:23`); the label at 0.38, the same | 4 (R1.6) |
| `material-3` | `padding_vertical_px` | `10.0` (`button/internal/_shared.scss:31-32`) | `8px` (`Button.kt:915`); one line of label draws at the 40 px minimum height either way | 4 (R1.6) |
| `material-3` | `min_width_px` | `64.0` (`button/internal/_shared.scss:91-94`) | `58px` (`Button.kt:1053`) | 4 (R1.6) |
| `material-3` | `checked_background`, `checked_text_color` | `#625b71`, `#ffffff`, the latest tonal button's `selected` tokens (`tokens/versions/latest/sass/_md-comp-button-tonal.scss:79`, `:95`) | `toggle` drawn nowhere at depth; held only under `flavor:expressive` | 5: the proposed loss |

Of 0.7.0's `button` fields, two sourced looks come to depth under the variant that holds them (`material-3`'s filled
tonal button, `tinted`; `ios-13`'s grey fill, `@gray`); `material-3`'s and `ios-13`'s button with no variant draws the
platform's default button (Compose's `Button`, the filled one; UIKit's system button), 0.7.0's look staying under
`tinted` and `@gray`, a correction of 0.7.0's mapping (Compose `Button.kt:77-88` against `FilledTonalButton`,
`:439-447`; UIKit `UIButton.h:22-23`); one look is proposed as lost, `material-3`'s checked text toggle (R21), held
only under the `expressive` flavour, a shared unit's name (Compose Material 3 1.5's Expressive `ToggleButton` is where
it returns, with the flavours unit); the values 0.7.0 stated with no source are corrected by the facts and listed with
R19's invented light variants (R21): the `macos-*` and `ios-13` hover fills, `kde-breeze`'s inherited primary,
`adwaita`'s `#2e3436`, gap and minimum width, `kde-breeze`'s minimum height, `macos-monterey`'s pressed fill, the
`macos-*` gap and minimum widths, `ios-13`'s sizes, gap and pressed and disabled fills, `windows-11`'s disabled fill
and minimum sizes; and the values 0.7.0 took from a source the preset's reference rule does not choose for it are
restated by that rule (R1.6 for `material-3`: the older `v0_192` token set's disabled container, material-web's
component's vertical padding and minimum width where Compose has the button; R1.3 and R1.4 for `ios-13`: WebKit's HTML
button's pressed label against UIKit's). Each is on the maintainer's D6 veto list. The same comparison runs for each
further unit (*M3*'s step 3).

### The units 0.8.0 enables (M3a), in M3's order

Each token group of 0.8.0 enters by S6.1's rule with the first unit whose value reads it (D6): S4.6's groups with
their readers (`window`'s frame `token.params.display`, `input`'s caret `token.params.caret`, `list`'s
`token.params.layout`), S4.8's with the core; S4.3's theme change is the motion unit's (D6, outcome 3: with no engine
in 0.8.0 a theme change is drawn at once, which is 0.8.0's stated behaviour and no interim rule); S4.7's `pointer`,
which 0.7.0's model lacks and the readers unit (now M3b's, outcome 4) was to read first, is the pointer unit's. A
condition key is declared by the presets with the first unit whose rule selects on it, as S12 refuses a declared key
no rule selects on, and the reader's read of it comes with that unit's readers step (ADD-A147). `cond
overlay_scrollbars` (0.7.0's `overlay_mode`) needs no unit, the condition being in the model since M1a and set by the
GNOME and macOS readers since M1b.

The readers unit is M3b's first (D6, outcome 4; ADD-A147): the comparison of all four readers with 0.7.0's at
`2796b26e` found every value 0.7.0 reads and M1b's port does not either replaced or the target of an M3a unit, so
each unit's readers step (*M3*, step 5) reads it. For window and chrome: GNOME's `titlebar-font` (GN-1.19), KDE's
`[WM] activeFont`, macOS's `windowFrameTextColor` and Windows' `SM_CXSMICON` for the title bar's icon (W1-010); for
fields: macOS's `placeholderTextColor`; for indicators: macOS's scroller width (MAC-S07); for containers: macOS's
`headerTextColor` and `gridColor`. KDE's `activeFont` and the four macOS colours have no trace `reader` line yet, which their units' lanes
add; Windows' `SM_CXSMICON` and `SM_CXICON` for `token.icons.sizes` wait on the register's U516: if its facts
state the shell metrics, the read comes with M2's task 0 (the Windows lane's round that closes U516, its reader
change an `implement` task), else no read is made; KDE's
`forceFontDPI` and `xrandr` reads the specification drops (S9.3). Rejected: the readers unit in M3a (208 trace rows
whose reads 0.7.0 does not make, against D13's timing).

1. **window and chrome**: `window`.
2. **overlays**: `dialog`, `popover`, `tooltip`, `menu`.
3. **buttons and toggles**: `button` (enabled by the slice), `link`, `segmented_control`, `switch`, `checkbox`,
   `radio`, `radio_group` (no 0.7.0 field, though 0.7.0's showcases draw a radio group, `basic.radios`; it joins with
   its unit).
4. **fields**: `input`, `text_area`, `combo_box`, `slider`.
5. **indicators**: `spinner`, `progress_bar`, `scrollbar`.
6. **containers**: `scroll_view` (no 0.7.0 field, though 0.7.0's showcases scroll their pages, `basic.page`; the
   slice's staged `token.behavior.scroll` ends here), `list`, `tab`,
   `sidebar`, `toolbar`, `status_bar`, `splitter`, `separator`, `card`, `expander`, `layout`. With `list` the unit
   decides `token.behavior.single_click_activates` (D6, outcome 14; the register's U504 and U555, the token's keys
   under U472, U329 and U609 beside them): which views the token speaks of, how a view is read (its default
   configuration, a documented one that is not the default, its documentation against its code), whether AppKit's
   `NSWorkspace` decides an item's activation, whether the Kirigami flavour reads the style hint or its delegates,
   and the token's form (one bool per platform, a `list` value per variant, or both); `absent` is no option where
   the views exist and activate. The evidence by platform is the record the ledger's U555 row names. 0.7.0's model
   holds no activation value, so the token stays in the parking unit *behaviour* unless the unit shows a value at
   depth reads it (ADD-A147).

30 of the catalogue's 87 typed widgets. Their 15 `Placement` values are deferred to the placement unit (S6.1, as the
slice defers the button's; the M1a design's decision 1): `window.values.{caption_button_menu_placement,
window_menu_placement, title_menu_placement, keyboard_avoidance}`, `dialog.values.{placement, keyboard_avoidance}`,
`popover.values.placement`, `tooltip.values.placement`, `menu.values.{placement, submenu_placement}`,
`button.values.{menu_placement, scaffold_placement}`, `combo_box.values.popup_placement`,
`slider.values.thumb_tooltip_placement`, `layout.values.levitated_placement`. Rejected: the placement unit in 0.8.0;
0.7.0 places nothing, and S13's strategies each need a vector table from their reference (the GTK and Qt containers,
Compose's code, the WinUI host) or measured AppKit and UIKit frames.

**After 0.8.0 (M3b):** the readers unit (ADD-A146's scope: every desktop `reader` line of the traces whose target,
a role, a token, an input, a text slot or a condition, lies in an enabled unit, the catalogue's *readers* list
among them; a line whose target is a widget's part or value, or a condition only a later unit's rule selects on,
waits for that unit's readers step; the iOS and Android lines stay R18's; D6, outcome 4); placement (ending those
deferrals); the 57 typed widgets the model lacks today, in alphabetical
batches of about ten; the 28 keyed components; the units of depth of 0.8.0's entries (content, styles, menus, states
and identities per widget unit; sizes, flavours, increased contrast, window activity, the iPad, accessibility text,
interface level, glass, right-to-left), each model-wide and closed on the measured platforms by a designed sample run
with a composition decision per key (S17's *Sets not yet measured*); the parking units (behaviour, text rendering,
text styles, icon contexts, elevation, parameters, keyed roles, pointer); and motion. A unit no Apple state model
names (flavours) needs no Apple fact and, adding no enum member, is a patch release (D14): the model member it
carries, U753's field on `Decoration`, is not breaking, `Decoration` being `#[non_exhaustive]` (ADD-A147).

### Widgets 0.7.0 styles through stand-ins

Some widgets 0.7.0's connectors and showcases style have no field of their own in 0.7.0's model: the connector
borrows other widgets' or the defaults' fields. The catalogue gives most of them an entry of a later unit (M3b). From
the 0.7.0 contract tables and connector code at `2796b26e`:

| Widget | 0.7.0's stand-in | Its catalogue entry |
|---|---|---|
| a menu bar | iced: `styles::aw::menu`, `iced_aw`'s `menu_bar::Style`, its bar from `menu.background_color` and `menu.border` (`connectors/native-theme-iced/src/styles/aw.rs:137-162`; contract `contract.rs:1610`); the showcases' `chrome.menu_bar` from `window.background_color` and `layout.container_margin_px`, its titles from `menu.*` (`docs/showcase-elements.toml@v0.7.0`) | `menu_bar` (M3b) |
| a number input | the showcases' `basic.number_input.field` from `input.*`; gpui's `NumberInput` frame refined by the connector (`geometry.rs:322-324`) | `stepper` (M3b), its text `input @ context stepper` |
| a code editor | gpui: the code `Editor`'s line height from `defaults.line_height`, its fill from `input.background_color`, its font from `defaults.mono_font` (`geometry.rs:394-408`) | `code_editor` (M3b) |
| a group box | gpui: `group_box`, `group_box_foreground` from the background, secondary and foreground colours (`connectors/native-theme-gpui/src/colors.rs:530-532`), `group_box_content` from `card.*` (`geometry.rs:767-774`) | `group_box` (M3b) |
| a loading placeholder | gpui: `skeleton` from `button.background_color` (`colors.rs:582`, contract `contract.rs:627-631`) | `redaction` (M3b) |
| a chart | gpui: `chart_1`–`chart_5` from the accent (`colors.rs:491-513`), `chart_bullish`, `chart_bearish` from the success and danger colours (`:353-354`), `chart_grid` from `list.grid_color` (`:446`) | `chart` (M3b) |
| a drag image's border | gpui: `drag_border` from the primary colour (`colors.rs:585`) | `drag_image` (M3b) |
| a description list | gpui: `description_list_label` and its foreground from the background, border and muted colours (`colors.rs:535-536`) | none: a gpui-component widget no platform list names; it stays a stand-in |

Not stand-ins, their homes being 0.8.0's: gpui's `accordion`, `accordion_title` and `accordion_hover` (`expander`,
`geometry.rs:785-821`), `table_*` (`list`), `overlay` (`dialog`'s `scrim`), `drop_target` (the `drop_target` state
of 0.8.0's widgets), `title_bar` (`window`); iced's `selection_list`, `pane_grid` and `aw::spinner` (`list`,
`splitter`, `spinner`). egui has none: every `Style` field it writes comes from a leaf of 0.8.0's units
(`connectors/native-theme-egui/mapping.toml@2796b26e`), and egui's own widgets that read those fields (its
`DragValue`'s text style, `src/mapping.md:745`) read them by egui's design.

**Decided by the orchestrator** (2026-11-06), by D13's own reasons (nothing 0.7.0 has is lost; ship sooner): until
its unit is enabled, a connector keeps its 0.7.0 stand-in mapping for such a widget, from the model's enabled roles
and widgets as 0.7.0 does, recorded in its contract table as a stand-in entry (the widget, the fields it reads, the
unit that replaces it), which S21.3's contract test and M5 accept; the widget's unit replaces it with the widget's own
mapping in its connector task. Rejected: adding those widgets to M3a, which costs their whole units against D13's
timing. The spec follows through the ledger's queue (S21.3's stand-in entry). A stand-in entry appears in the
connector task of the unit that enables the last widget it reads (ADD-A146): at M2.1 where the slice holds all it
reads (the loading placeholder's `button` fill, the drag image's and the description list's colours), with fields
for the number input and the code editor, with containers for the menu bar, the group box and the chart; until
then the widget is unmapped as its source widgets are, a state of the build no release holds, every source being
in 0.8.0's units. Rejected: the entry written at M2.1 from what exists and extended by each later unit (a stand-in
drawn from part of its sources, a look 0.7.0 never drew, and an entry several units rewrite).

**Widgets a preset declares unavailable** (D6, outcome 6; ADD-A147). 0.7.0 resolved `tab`, `card`, `expander` and
`status_bar` on every preset; the catalogue declares them, truthfully, unavailable where the platform has no such
control (`tab` on `ios-*`, UITabBar being `bottom_tab_bar`; `card` on `macos-*`, NSBox being `group_box`, and on
`ios-*`; `expander` on `material-3` and `ios-*`, their expandable item a list row; `status_bar` on `adwaita`,
`windows-11`, `ios-*` and `material-3`), and the query answers `NotAvailable` there. The declarations stay. Each
connector's contract lists, per preset, the entry it draws its toolkit's widget from there (S21.3): a stand-in entry
for the widget the catalogue names, until that widget's unit (`bottom_tab_bar` and `group_box`, M3b; `list`'s row
for `expander`); or, where the catalogue names none (`status_bar`, `card` on `ios-*`), the toolkit's own geometry
with every colour and
font from the theme's typed roles, never an unthemed toolkit colour. The contract test checks the list is complete.
The same rule serves a stand-in widget whose unit is enabled on a preset that declares it unavailable. The rows are
written in the containers unit's connector task, which enables the four widgets. 0.7.0's values under those names
(the iOS tab bar's height of 49, the macOS box margins) return with `bottom_tab_bar` and `group_box`. Rejected:
`bottom_tab_bar` and `group_box` moved into 0.8.0 (two more units against D13's timing, and the rule still missing
where the catalogue names no control); each connector deciding alone (against "connectors follow the theme's data").

### 0.7.0's public features the new model keeps

| 0.7.0 feature (at `2796b26e`) | In the new model | Planned in |
|---|---|---|
| the seven platform presets (`presets.rs:58-64`) | the presets from facts; `ios-26` added | M1a (`adwaita`, `macos-tahoe`), **M1b** (the other six), each unit |
| the ten community presets of seven projects (`presets.rs:66-75`) | palettes from the projects' published colours (S14) | **M2.4** |
| `Theme::preset`, `merge`, `resolve`, `resolve_with`, `list_presets`, `list_presets_for_platform` (`model/mod.rs:578`, `:340`, `:487`, `:515`, `:718`, `:741`); `SystemTheme::pick`, `with_overlay` (`lib.rs:735`, `:890`) | `Theme::from_preset`, `add_layer`, `pick`, `resolve` and the query, `PresetId::all`; `PresetId::for_platform` for `list_presets_for_platform` (S16; ADD-A146) | M1a; `PresetId::for_platform` at **M2.5** (S16 complete) |
| the readers: `SystemTheme::from_system`, `from_system_async`, `from_system_with_preset` (`lib.rs:959-1017`); features `kde`, `portal`, `macos`, `windows` and their aliases | `Theme::from_system(None)`, `from_system_async`, `from_system(Some(generation))` and `reload_reader`; `Theme::system()` is the accessor of the reader's state (S16; ADD-A146) | **M1b** (the slice's scope), each unit's readers step; the readers unit after 0.8.0 (M3b; D6, outcome 4) |
| `detect`'s scheme and motion functions (`detect.rs:211-948`), `AccessibilityPreferences` | conditions and `ReaderState` | **M1b** |
| `watch::on_theme_change` (`watch/mod.rs:435`), feature `watch` | the same, ported to S9.5 | **M1b** |
| `icons`: `load_icon`, `load_custom_icon(_in)`, the five loaders, `colorize_monochrome_svg`, `fill_current_color`, `IconSetChoice`, `default_icon_choice`, the freedesktop theme lists (`icons.rs:112-1480`, less `load_icon_indicator`); `IconSet`, `IconRole`, `IconData`, `IconProvider`; features `system-icons`, `material-icons`, `lucide-icons` | kept in place, repointed (the inventory) | M1a's switch |
| `SystemTheme::icon`, `icon_theme_for` (`lib.rs:796`, `:750`) | `ResolvedTheme::icon` and the resolved `token.icons` | **M2.5** |
| `fonts` (`system_face`, `select_face`, `substitute_family(_cached)`; feature `system-fonts`), `rasterize::rasterize_svg` (feature `svg-rasterize`) | kept | M1a's switch |
| `detect::{LinuxDesktop, detect_linux_desktop, invalidate_caches}`, `pipeline::{diagnose_platform_support, DiagnosticEntry, VersionMatch}` | kept (the inventory's F1) | M1a's switch |
| `native-theme-build` | follows the new API | M1a's switch |
| the four connectors, the three showcases | moved to the model | **M2.1**, each unit's connector task, M5 |
| `native-theme-gpui`'s API: `to_theme`, `from_preset`, `from_system`, `apply`, `NativeTheme`, `SystemThemeExt`, the root's accessor functions, the modules `base_layer`, `geometry`, `icons`, `variants`, `widgets`, and `on_theme_change` (`connectors/native-theme-gpui/src/lib.rs:226-1160`) | each item's fate in the connectors' design (ADD-A146) | **M2.1** (the design first), each unit's connector task |
| `native-theme-iced`'s API: `to_theme`, `to_theme_in`, `from_preset`, `from_system`, `SystemThemeExt`, the root's padding, metric and colour helpers and its widget helpers (`switch`, `radio`, `tab_indicator`, `focus_ring`, …), the modules `styles`, `palette`, `spinner`, `icons`, and `theme_changes` (`connectors/native-theme-iced/src/lib.rs:410-2135`) | the same | the same |
| `native-theme-egui`'s API: `to_theme`, `from_preset`, `from_system`, `SystemThemeExt`, `ThemeAtlas` and its `Builder`, `NativeThemeUiExt`, the roles (`Role`, `Surface`, `TextRole`, …), the accessor functions, `ThemeWatcher`, the modules `convert`, `fonts`, `icons` (`connectors/native-theme-egui/src/lib.rs:173-212`) | the same | the same |
| `native-theme-egui-widgets`' API: its widget modules (`combo_box`, `expander`, `fade`, `links`, `parts`, `progress_bar`, `radio_button`, `segmented_control`, `slider`, `spinner`, `switch`; `connectors/native-theme-egui-widgets/src/lib.rs:125-135`) | the same | the same |

Not kept, by R21 and S16 (S19.3): `Theme::from_toml`, `from_file`, `to_toml`, `lint_toml` (custom looks are
application layers, `Layer::from_toml`, M1a); `load_icon_indicator`, `SystemTheme::icon_indicator` and
`AnimatedIcon` (the `spinner` widget, the indicators unit); `ResolutionContext` (its fields' places in the model);
`platform_preset_name` (`PresetId::for_system`, M2.5); the `prelude` module (S16 states none; the inventory's F8) and
`crate::Error`'s theme variants (S1's error types, S16; ADD-A146).

## The order

```text
M0 setup ──► M1a slice core ──► M1b slice complete ──► D6 ──► M2 core ──► M3a 0.8.0's units ──► M4 gate ──► M5 showcases ──► M6 0.8.0
  │              ▲   │                ▲                                     │ each unit closed ──► its connector task         │
  │              │   └─► L facts lanes, verifiers (published from M1a; measured as T delivers) ──► M1b, M3a, M3b              ▼
  └─► spikes (M, G, K first; then I, W, A) ──► T tooling on v0.8.0-tools ──► L                                               M3b later units ──► follow-up releases
      M's button pass, G's vector generator ──► M1a;  K's script, the KDE and Material vector generators ──► M1b
```

## M0 — Setup

M0 closes when its setup tasks close; the spikes continue beside M1a and have their own checklist line.

- **Branches and version.** Cut `v0.8.0-rc1` (D5) and `v0.8.0-tools`. Bump the version to 0.8.0: the workspace
  version, `workspace.dependencies`' `native-theme`, `native-theme-derive` in `native-theme/Cargo.toml`,
  `native-theme-egui` in the egui-widgets manifest, the READMEs' versions (`check_readme_versions.py` requires them
  together), and a `## [0.8.0] - Unreleased` CHANGELOG heading. The visual-asset and compatibility stamp checks, which
  fail once a release's entry is dated, then stay warnings until M6 dates it. Add `PLAN_EXCLUDED` (empty) to
  `check_release.sh`. Open the draft pull requests.
- **The ledger** `docs/todo_v0.7.1_model-completeness-ledger.md`, written by the orchestrator only (the others report
  to it): per unit and platform the status of facts, verification, presets, gate and connectors; the module inventory;
  the warnings of each full run; the spec-change queue; the unmeasured register; the verifiers' agreement and audit
  rates; the spikes' and drift runs' results; the evidence size; the tokens per milestone (the sum of the agents'
  reported usage).
- **The facts checkers early,** so the lanes can start on M1a's first day: `scripts/check_facts.py` (form: keys from
  the catalogue through `spec_catalogue.py`, `when` as S11's, kind and source well formed, the source pinned, the value
  as S2.2's, the generation rule) and `scripts/check_fact_citations.py` (*How values are checked*), after the S17
  amendment below.
- **Spec checkers:** `check_spec_names.py` and `check_spec_entries.py` gain `--self-test` (`check_spec_trace.py` has
  one); the catalogue parser moves into `scripts/spec_catalogue.py`, shared with the generator and the facts checker.
- **The source cache** (D1): `docs/facts/sources.toml`, `scripts/check_sources.py`, and `scripts/update_sources.sh`,
  which fetches each entry at its pin (androidx and AOSP shallow and sparse) and whose `--verify` checks every
  checkout's commit and every snapshot's hash.
- **Re-pin** (before any lane starts): `scripts/repin_citations.py` maps each citation of a pinned source in the core,
  the catalogue, the traces and the gap maps from the commit it was read at (the gap maps record each SHA; `master` for
  most KDE sources, read 2026-10-27), fetched again by its SHA, to the pinned release, by the line mapping between the
  two commits, and lists each citation whose cited text changed or vanished; the orchestrator re-reads and corrects
  those. Not re-pinned: `platform-facts.md` (its rows are lane input the lanes pin themselves, and editing it would
  break today's preset checks before the switch); the rationale (its citations record what each decision was based on,
  dated by their SHAs); `docs/todo.md` (only its `platform-facts.md:LINE` citations get the `@<sha>` rewrite; a lane
  taking a triaged item re-reads its sources at the pin). A citation the script cannot map (M0's inventory, 2026-10-29:
  about 1,100 of 7,729 in scope, mostly the catalogue's bare file names with no commit, names several projects own,
  and branches with no SHA) takes its commit from its row's platform file where that names one source for it;
  otherwise it is listed in the ledger with its row, and the lane that takes the row re-reads it at the pin, as for
  `docs/todo.md`. Re-reading all of them in M0 was rejected: the lanes read every source at the pin anyway, and the
  facts' quote check, not the catalogue's pointer, is what holds a value to its source.
- **The `@<sha>` rewrite** of the gap maps' and `docs/todo.md`'s `platform-facts.md:LINE` citations (*What carries
  over*), as its own pass.
- **The specification's amendments** this plan needs, through *Spec changes*:
  - S17's fact gains `rows` (the gap-map rows it carries); `quote` (`published` and `read`: the source's exact text at
    the citation); `conversion` (a closed list: points to pixels, a fraction's complement, …, applied by the checker);
    `evidence` (`measured`: the evidence file and the path of the value inside it,
    `probe.toml#macos.fonts.sizes.systemFontSize`; for a value read from an image, the committed output of a committed
    analysis script naming its input image and the pixels it reads).
  - Facts stay one directory per platform; a fact for one generation carries `preset:<id>` (S7.1's `Preset`
    condition) in its `when`, and a fact without one cites a source or evidence for each generation its directory
    covers among the presets that exist (at M1a the macOS directory covers macOS 26 only; M1b adds macOS 15's citation
    or evidence to every shared fact, or splits it by `preset:`). Rejected: one directory per preset, which duplicates
    every shared fact.
  - A preset's declarations (availability, variants, sizes, container sizes) are stated in `state-models.toml`, and
    the gate checks each preset's declarations against them.
  - The palettes' published colours in `docs/facts/palettes/<project>.toml`, and a palette run in the gate (S14).
  - S5, S6.1 and S18: the declarations are generated from the catalogue by `scripts/generate_widgets.py`, with a drift
    check until the catalogue is archived, after which the generated code is the source. Rejected: a proc macro
    reading the catalogue (it lies outside the crate and is archived); hand-writing every entry with no drift check.
  - S2.2 and S13: each platform function and each placement strategy has a vector table from its reference; the
    model's own arithmetic (`mul`, `pick`, CSS `color-mix` as `mix` defines it, …) has unit tests from the
    specification's definition.
  - S21.3: a connector's contract table gains `pending` entries, which the contract test accepts; from M2.1, any commit
    that adds a resolved field adds it there for every connector (the generator lists them), and M5's gate requires
    none.
  - S18: the scripts table below, and `check_connector_parity.py` (rewritten at M2.1).
- **Triage:** every open item of `docs/todo.md`, whatever its tag, each of the catalogue's *Open items*, ROADMAP's
  *Later — the connectors* and *Also in the 0.7.x releases* (D11), and the two connector plans
  `docs/todo_gpui-full-theme.md` and `docs/todo_iced-full-theme-geometry.md` get a destination in the ledger: a named
  milestone, unit or lane; settled by the model; carried to a later version with its reason; or not this plan's work.
- **Spikes** on `v0.8.0-tools`, each ending with yes or no and its result in the ledger, up to three at a time within
  the cap (an agent waiting on a CI run does not count), M1a's needs first:
  - **M:** the macOS probe's button pass (hovered, pressed, disabled and checked in a key window, at each size); then
    its other missing passes (a wallpaper and transparency setting under control, the drawn against the read window
    background) and the runner's GPU;
  - **G:** the GNOME harness, and its vector generator for libadwaita's functions (it calls public API such as
    `adw_rgba_to_standalone` and needs no display, so it does not wait on the harness's display question);
  - **K:** the KDE container and the accent script;
  - **I:** the iOS simulator host;
  - **W:** whether microsoft-ui-xaml states the slice's keys and those of ten sample widgets; the WinUI host built, run
    on `windows-11-arm` and reporting a flyout's frame;
  - **A:** the Android host.
- **Gate:** the spec checkers' self-tests and runs green; `check_sources.py` and `update_sources.sh --verify` green;
  `repin_citations.py` reports no unresolved citation among those it maps, and the ledger lists the rest; `scripts/check_ledger.py` (every triaged item has a
  destination) green.

## M1a — The slice's core: `button`, colour roles and the slice's tokens on two presets

- **Presets:** `adwaita` (published) and `macos-tahoe` (measured by spike M's button pass). If spike M fails, the
  button's macOS combinations go to the unmeasured register and `windows-11` (published from microsoft-ui-xaml's
  visual states, spike W brought forward) takes `macos-tahoe`'s place; M1b then closes only after the maintainer has
  decided those entries.
- **Code, beside today's model:** S1; S2's blocks in full with S2.2's grammar, the model's arithmetic and the platform
  functions the two presets' facts use, with their vectors (spike G's generator); S3; S4.1, S4.2 (its `scroll` field staged to the unit holding `scroll_view`, containers, spec ADD-A120), S4.4, S4.5, and
  S4.6's groups the button and the roles use (`token.params.pointer_repeat`, `token.params.color`,
  `token.params.opacity`, …); S5 and S6 for `button`, through `generate_widgets.py` and `ThemeParts` (its `Placement` values deferred to M3's
placement unit, spec ADD-A120); S7; S8 without
  S13; S9.1, S9.2, S9.4; S10; S11; S12 for what the model holds, with its refusal test (an exhaustive `match` over
  `LayerError` and `ProblemKind`, one triggering case per variant, so a new variant does not compile without one);
  S21.2's shape from the start (exhaustive enums, `#[non_exhaustive]` structs), so the connectors are written against
  it; S16's `Theme`, `from_preset`, `set_reader_stand_in`, `set_accent` and the query; S17's gate with its normal and
  perturbed runs and the declarations check. `PresetId` holds the presets written so far. The platform functions live
  in the directory module `next/platform/` (`platform/` after the switch); S13's strategies come in `placement/`
  with M3b's placement unit.
- **Facts tooling:** `scripts/check_function_vectors.py`; `scripts/check_facts_trace.py` (`--enabled` checks the
  enabled units on the presets that exist, `--unit` one unit, no option all); `scripts/generate_platform_facts.py`
  with `--check`, in the gates from the switch; `scripts/import_platform_facts.py` (*What carries over*).
- **The switch** (the last task): as *The transition* says; `native-theme-build` follows the new API (its drift test
  calls `IconSet::from_name`); the preset-validator agent and the two skills rewritten if D9 approves.
- **Gate:** `cargo test -p native-theme` (the facts gate and the doctests included), `check_facts.py`,
  `check_fact_citations.py`, `check_function_vectors.py` and `generate_widgets.py --check` green; after the switch,
  `generate_platform_facts.py --check` green and the full run green, with `PLAN_EXCLUDED` equal to the expected list.

## M1b — The slice complete

- **Presets:** the other six at the slice's scope, `ios-26` created (its `PresetId`, its file, iOS 26 as its
  generation, its `docs/todo.md` entry closed), all from verified facts.
- **Code:** the platform functions the six presets' facts use, each with its vector table (expected among them
  `kde_accent`, `kde_accent_wm`, `kde_effects`, `kde_selection_inactive`, `kcu_mix`, `kcu_tint`, `kcu_shade`,
  `kcu_overlay`, `m3_role` and Qt's colour helpers; the lanes' facts name the full list); S4.8's `token.kde` and
  `token.material` groups; the four readers and their watchers ported to S9.3 and S9.5 at the slice's scope
  (`ReaderState`, the KDE scheme groups and unset keys) and linked again, with the split files' reader functions
  restored and S16's system entry points (`Theme::from_system`, `from_system_async` and `reload_reader`, with the
  accessor `Theme::system()`; ADD-A146); the reader tests return.
- **The accent run:** `Palette::from_port` and `apply_palette` (S14, as far as the run needs; `from_port` and the
  `.colors` conversion it shares with the KDE reader in every build, with no feature, ADD-A142); the KDE schemes and
  accents chosen and pinned (at least one that tints, one with the accent-coloured title bar and a `Header` group, one
  with it and no `Header` group, one lacking a colour group, S17), measured by spike K's script, BreezeClassic also
  with no `AccentColor` as SUSE-Cafe was (run 41158393408's `classic-none` pair, stated by KDE facts round 7, U698
  retired; the no-accent pair S17's *A port's own roles* compares); the run's non-preset
  scheme files read from the source cache, which the facts-gate jobs fetch (S17, ADD-A142).
- **For D6:** the cost, the evidence size and the gate's run time measured and extrapolated.
- **Gate:** as M1a's, over all eight presets, with the KDE accent run; the pull request's CI green on macOS and Windows.

## M2 — The model's core, complete (after D6; the numbered tasks merged in this order, the entries before them as each states)

**Before task 0: the depth 0.8.0 states** (D6, outcomes 2 and 3; the D6 staging design; ADD-A147). One code and tooling
task, after the spec batch that states the depth rule and its form (ADD-A147: R21's amendment, S6.1, S8, S10, S12,
S16, S17, S21's lines, the catalogue's Depth lines and *Today's fields* corrections) and before task 0. It brings the
slice's model, its generators and checkers and the eight bundled presets to the depth 0.7.0's model and readers
distinguish (*The depth 0.8.0 states*). No fact is edited.

*Inputs:* the catalogue's Depth lines and *Today's fields* mappings as ADD-A147 leaves them; the facts at the task's
base; the D6 staging design's tables, whose figures the task reproduces (it does not fit to them: a count that
differs is reported).

*Steps, by file:*
1. `scripts/widget_units.toml`: every unit of depth with the names it holds per entry (and its presets where it is
   limited to some: the iPad unit's `hover` and `focus_visible` on `ios-*`), every parking unit with its paths, an
   `enabled` flag each; `[deferred.button]` gains the values only later names read (the catalogue's Depth line for
   `button`) and the motion unit's part fields (`transitions`, `shown`); `[staged.slice]` gains every token path the
   token rule stages, each to its parking unit or its unit of depth.
2. The predicate of a fact's units (S17) in the four readers of the units file, by one rule: the gate
   (`native-theme/tests/facts_gate/units.rs`, `run.rs`), `scripts/check_facts_trace.py`,
   `scripts/generate_preset.py`; `check_ledger.py` and `verify_facts.py units` by the same predicate.
   `check_facts.py` checks that every name of every state model, every condition a fact names and every token path
   a fact names lies in exactly one place: an entry's Depth line, one unit of depth or one parking unit.
3. `scripts/generate_widgets.py`: reads the Depth lines (`--check`: each line's catalogue part against the *Today's
   fields* selectors that name a target in the entry; no deferral to an enabled unit) and writes `Variant`'s
   members, each widget's `variants` and `content_variants` from the enabled entries' Depth lines and the enabled
   units of depth, the enabled flags of the sizes and flavours units, and the values structs less the deferred
   values (`native-theme/src/widgets/gen_index.rs`, `gen_button.rs`); the tests and gate fixtures that name a
   removed variant rewritten (`native-theme/tests/facts_gate.rs`, `src/query/tests.rs`, `src/toml/tests.rs`,
   `src/widgets/tests.rs`, `src/validate/`).
4. `native-theme/src/tokens.rs` and the hand-written blocks: the staged fields leave their structs
   (`MotionTokens.durations`, `.timings`, `.color_space`; `TextRendering` and `FontFallbacks` from `TextTokens`;
   the nine staged slots from `TextRoles`; `Caret` from `Tokens`; every `Behavior` field but `mnemonics`,
   `mnemonic_when_disabled` and `focus_visible_when`; `IconTokens.mirroring` and `.animated_visuals`;
   `IconStyle.animation`; `FocusIndicator.transition`; `token.theme_change`), with the declarations they read and
   their `DECLARATIONS` rows (`native-theme/src/validate/checks.rs`); first a code search confirms the core's token
   reads and the connectors' reads S4.2 names, so that no path the core reads is staged.
   `ResolvedPartStyle.transitions`' documentation says that no part lists it until the motion unit.
5. The query and the theme (S8, S16): `Unavailable::Later`, answered for an element holding a `later` variant (a
   content variant included) and, while the sizes unit is not enabled, for a size the element takes other than its
   default in force; `Theme::set_flavor` refuses a declared flavour but the default while the flavours unit is not
   enabled, with a problem of its own, and `flavors()` lists the default alone.
6. S12 (`native-theme/src/validate/`): the `later` declaration read; a rule, reference or patch naming a `later`
   variant, or a size or a flavour of a unit not enabled, refused with `ProblemKind::Later`; `checks::unused`
   exempts a flavour of a unit not enabled with its `[declare.flavor.<f>]` sections; the declarations check compares
   `later.variants`, the trimmed exclusive groups, recency sets and keyed `flags` with the state model (S17).
7. `scripts/generate_preset.py`: the state model in force in one place, the `models` dict `declarations()` returns
   (the default flavour's models only; the button's sizes the default in force; no `sizes_by_variant` entry of a
   variant not at depth), which `covering_bases()`, `pending_table()` and `round_trip()` read; the facts of the
   enabled units by the predicate; every mention of a name not at depth left out of a rule (a fact naming it
   positively, a negative entry naming it; exclusive groups and recency sets trimmed to the flags at depth, a group
   of one dropped; a one-of `sizes` list trimmed); `later.variants` written from the state model and the units file;
   `flags` and `[declare] conditions` from the trimmed facts; each keyed namespace's declaration trimmed to the
   entries a base states (a stated base, or a listed path the synthetic route's bases state), a map with none stated
   empty (`{}`).
8. The eight presets regenerated (`generate_preset.py --write`, then `--check`), and their hand-written parts edited
   where the generator reports them: the keyed `[declare]` entries no base states removed; on `ios-*` the rule block
   of `role.global.accent`'s read fact moved to the bases of `colors.<scheme>.global`; on `macos-*` the bases blocks
   of `colors.<scheme>.keyed` removed.
9. `native-theme/src/macos.rs`'s module comment: `role.@selected_control` leaves its list of roles the preset
   states. No reader's reads change.

*Gate:* M2's gate per task, and: `generate_widgets.py --check` and `generate_preset.py --check` on all eight presets
(no mismatch, no unstated field); the self-tests of `check_facts.py`, `check_facts_trace.py`, `check_ledger.py` and
`verify_facts.py`; each regenerated preset loaded in Rust through `check_preset`, and as its positive control a test
that S12's unused check refuses `ios-13` with its `@loading` flag declared and `kde-breeze` with its `@single_click`
setting declared (the form's definition is what keeps them out, so only the loader shows it holds); the facts gate
green over the eight regenerated presets with the predicate in force; `cargo test --workspace` outside
`PLAN_EXCLUDED`.

*Done when* the gate is green and the eight `[pending]` tables hold exactly the listed paths the design gives as
staying: `adwaita` 0, `kde-breeze` 4, `macos-monterey` 94, `macos-tahoe` 77, `ios-13` 221, `ios-26` 227,
`windows-11` 55, `material-3` 9 (687 in all; ledger queue #258). A table that differs is reported with the paths that differ, and the
task does not close until the orchestrator has decided it.

**`docs/platform-facts.md` not committed** (D6, outcome 8; the ledger's queue #170; ADD-A147). One tooling task
(`implement`), early in M2, merged before the next large facts merge (task 0's lanes' rounds). The file leaves the
repository in one commit and is ignored; `check_release.sh`, in every mode, and `ci.yml` run
`generate_platform_facts.py` into a scratch path and its `--self-test` in place of `--check`; each release generates
it at its commit (M6); `check_facts.py` fails a facts file past 40 MB, which is split by key prefix (S17's
*Format*); the local tools that cite the file's lines (the preset validator and the `platform-fact` skill, D9) read
the facts or a copy generated on demand. *Done:* the file absent from the branch and ignored; `check_facts.py
--self-test` passing a made-up facts file at 40 MB and failing one past it; no facts file past 40 MB at the task's
base (the largest, `docs/facts/ios/button.toml`, 34,075,260 bytes at `72dd07b4`); `generate_platform_facts.py
--self-test` and the generation green in `check_release.sh --quick` and in the pull request's CI; neither local
tool reading a committed copy (by search). Rejected: the file committed per platform in smaller files (D6's package
projected, for M3a's units at full depth, about 125 MiB for the iOS file and 104 MiB for the macOS one, each past
GitHub's block); kept as it is (it stands at GitHub's warning and grows with every measured fact).

**The iOS second-tint run** (D6, outcome 12; the register's U706; ADD-A147). The iOS lane's host pass on both
simulators under a second tint, beside M2's tasks, after the staging task, so that what UIKit draws in the tint is a
measured derivation over the accent (R2.6), as the macOS second-accent pass made it on `macos-*`. *Done:* every
tint-following fact at 0.8.0's depth stated as a derivation over the accent and verified, the default tint's
literals gone from them, and U706 closed for them; a value that does not follow the tint stays a literal, with the
run's evidence. It blocks the iOS presets' release, not M2's other tasks.

**The KDE reader's `Xft.dpi` read removed** (D6, outcome 12; the ledger's queue #198; ADD-A147). One code task
(`implement`), beside M2's numbered tasks, since it removes a defect of a built reader (the font DPI applied twice
where the toolkit's scale carries it, R10): the read, its helper, vector table and generator part, its tests and
docs, and S2.2's `qxcb_xft_dpi` entry in the same commit (#198 lists the places). *Done:* none of them left (by
search); `check_function_vectors.py`, the KDE reader's tests and `check_release.sh --quick` green.

**The load path** (D6, outcome 1; the D6 load design; ADD-A147). The load tasks, named as the design names them
("load task 1" is not M2's task 1), and the trigger step run beside M2's numbered tasks, each placed by what must
come before it.
What they build: **A**, trust (a bundled preset whose `[pending]` table is empty, and the layers `Theme` derives from
it alone, checked by the build test and the facts gate through the validating construction and not again at run
time: R2.8, S8, S19.1); **B**, the checks over combinations run only for a layer whose stack, as they read it, the
change altered (S12's *What a change re-checks*), with a thread-local differential switch that re-runs a skipped
change on the full path in the crate's own unit tests; **E1**, one read of a preset per process while a theme holds
it (a `Weak` per preset); **E2**, `WindowState` keeping the resolution it returned (S8). Each diff gets its `review`
before it merges; none changes a preset's value or a fact; each task records the budget's cases its change can move
(*The load budget*, *Who makes records*). The defect the design found in today's checks (two sides of a corner
compared by their stated numbers whatever their unit, `SideLen::same` in `native-theme/src/validate/combos/mod.rs`)
is fixed before task 0 on its own branch, `side-units`.

- *Before the trigger step:* **load task P, the probe** (`implement`). `native-theme/tests/timings.rs`'s
  `bundled_preset_costs` (the ledger's queue #253) merged and extended to every case of *The load budget*: one
  process per pass for line 1; the bytes in a test binary of its own with `peak_alloc` 0.3 as a dev-dependency, one
  test thread (lines 2, 3, 11); line 1 on a preset whose table is not empty as the case `read`, a lower bound, timed
  by a doc-hidden `native_theme::gate` function that returns figures only, never a theme or a stack; lines 4 to 8 on
  the plain theme where the table is empty, else on the synthetic theme, labelled so; the embedded bytes; the held
  bytes per rule for all eight; reference layers for line 12; `scripts/check_cost_budget.py` and
  `scripts/cost_budget.toml`. *Done:* the probe runs the cases it is asked for by name and lists, when asked, every
  case it can measure; it samples the runnable count in a thread of its own and prints each pass's mean beside the
  pass's figure, and each sweep's duration; the script's `--self-test`, on made-up figures, per-pass means and sweep
  start times, passes and fails one case each way of every outcome the four rules give (*The load budget*), and
  leaves `cost_budget.toml` byte for byte unchanged; then five recorded runs (three for the bytes), each case's
  spread, tolerance and first record in the ledger and in `cost_budget.toml`, each case whose record does not meet
  its line under a named miss of its own; the release build's time and a quiet sweep's duration recorded, the
  release run's bound raised where three sweeps take more than half of it. The step is not wired into
  `check_release.sh` before load task 6, so it needs no `PLAN_EXCLUDED` entry.
- **The trigger step** (`worker`, release, on the measuring machine), once the staging task has merged with its
  regenerated presets passing its gate, and before the connectors' design is checked; nothing else waits for it: load
  task P's probe over the eight presets as the staging task leaves them, each trigger of *The load budget*'s
  deferred parts evaluated on them against the lines as they stand. *Done:* the figures and each trigger's outcome in
  the ledger; task P's records on the regenerated presets replaced from rule 3's runs, each case whose record does
  not meet its line under its named miss; each part whose trigger fired named as a task before M3a's first widget
  unit; where line 5, 7 or 11 is missed, its part's decision (the kept rules held by reference, a faster query)
  taken with load task 4a.
- *Before the connectors' design is checked* (task 1's first part; it states who keeps a query's result and how a
  connector resolves): the class-2 lines of the load design and R2.8's sentence, which ADD-A147 applies; and **load
  task 4a, the query and resolve profile** (`worker`, on the `cost-profile` branch, release, on the measuring
  machine). *Done:* where `style`'s 32–53 µs and `resolve`'s 0.23–1.47 ms go on the four profiled presets, by phase,
  traced to raw files; the decisions on load task 5, a faster query and the kept rules held by reference stated from
  it.
- *Before the first connector's code* (task 1's connectors, which are written against these types):
  - **Load task 1, trust and B** (`implement-hard`: B's comparison and the split of `check_stacked` leave judgment,
    and a wrong choice there would pass the tests and be unsound). The stack's checking, `Full` or `Incremental`;
    the validating construction (`Stack::new`, `Stack::synthetic`, `preset_stack`, `synthetic_stack` and the facts
    gate's constructions, `Full`) and the trusted one (`Stack::bundled(id, scheme)` and
    `Stack::set_preset_bundled(id)`, which take a `PresetId` and no text, read `id.source()` through E1's cache and
    are `from_preset`'s and `set_preset`'s alone; a `#[cfg(test)]` twin `Stack::bundled_text`); B; the switch; E1.
    *Done:* the build test (S19.1) through the validating construction, with its items: each preset accepted, each
    trusted form passing the full path's check of its layer, and the stacks `Theme::from_preset` and
    `Theme::set_preset` build equal to the validated ones; its controls, each with the switch off (a fault only the
    checks over combinations find, refused by the build test and accepted by the trusted twin; a listed form with a
    token cycle added, refused; a preset whose inputs read each other in a cycle, refused; a test-only hook that makes
    `Stack::bundled` build a different stack failing the equality); every existing test that relies on
    `preset_stack(None, …)` or `validates(None, …)` refusing, and the gate's tests, unchanged and passing; G1 (the
    runs of the checks over combinations a call makes, a thread-local counter) = 0 with the switch on after
    `from_preset` on each preset whose table is then empty, `pick`, `set_flavor`, `set_contrast` with no palette,
    `set_accent` with no reader (on `kde-breeze` too), `set_preset` to another of them, and the Windows fixture's
    reading set, re-read unchanged and re-read with another accent; G1 ≥ 1 after a structural `add_layer`, a
    `replace_layer` changing one rule's `when`, the KDE fixture's reading set and re-read at the other scheme,
    `set_accent` and a palette over it, `set_contrast` under a KDE port beneath a structural layer, and `set_preset`
    with a structural application layer (the KDE cases on CI's Linux job, where the reader is compiled); negative
    controls, the switch off and on: a `set_preset` whose application layer names a key the new preset does not
    declare, a reading whose inputs form a cycle beneath a layer stating one of them, and `add_layer` of an
    inputs-only layer with an undeclared or ill-typed input (G1 = 0), each refused; the switch on over the crate's
    unit tests, the mutation tests' stacks and the tests that change a validated stack made `Incremental`, their
    expectations unchanged, and a wrongly forced skip failing (its positive control); G2 (the reads of a preset the
    calling thread makes while its own theme holds one, thread-local) at most 1, two themes of one preset holding
    one read, and `from_system`'s path through the KDE and GNOME fixtures reading once; `Theme` asserted `Send` at
    compile time; every statement of the old rule ("every change validates the whole stack") worded as S16's
    sentence: the doc comments of `native-theme/src/theme/mod.rs` and `src/resolve/mod.rs`, the crate's
    documentation in `src/lib.rs`, the build test's own in `src/theme/tests.rs`, `native-theme/README.md`, the
    workspace's `README.md` and the CHANGELOG's 0.8.0 entry; the build test's time in debug and release and the facts gate's time
    (`NT_FACTS_GATE_TIMING=1`) before and after in the ledger, the build test's added items moved into the facts
    gate's per-preset steps where they would pass a limit of *The facts gate's cost*; the records of lines 1 to 4.
  - **Load task 3, the resolution kept** (`implement`): E2; `Resolution.theme` an `Arc<ResolvedTheme>` (S8).
    *Done:* a second `resolve` with nothing changed returns `Unchanged` and the same handle (`Arc::ptr_eq`; G3) and
    fails if it builds one; every `Theme` mutation and every field of `WindowConditions` and `ReaderState` changed
    alone gives a new handle, the fields taken by exhaustive destructuring; the records of lines 3, 5, 6 and 11.
  - **Load task 5, the query cache** (`implement-hard`), only where load task 4a and the connectors' design show a
    connector querying every frame and no repair of the query bringing it near line 8: the cache inside the
    resolution, `Scope` a handle carrying its resolution's identity (S8's class-2 line, written with this task).
    *Done:* every query of the gate's fixture combinations answers the same with the cache and without; each field
    of `Element` and `ElementData` changed alone gives the uncached answer (the key by exhaustive destructuring); a
    list longer than the cache's bound answers correctly; G4 (a repeated query evaluates nothing); the records of
    lines 3, 5, 7, 8 and 11. Where it is not built, line 8 is not binding, its reason that design's ledger row.
- *Before M3a's first widget unit* (at M2's close):
  - **Load task 2, colour values out of the checks' sight** (`implement-hard`: the audit is judgment). *Done:* each
    `Fact` read of the checks (the design's list of thirty-four lines) and every equality on a `Fact`, `Val` or
    `OpKind` under `native-theme/src/validate` annotated in the ledger with the place it reads and why no colour
    place reaches it; a stated colour flattens to `Unknown` (a test); B's compared form sets aside the values read as
    `Unknown`, its cost per op in the probe; every bundled preset's and `src/validate/tests.rs`' problems identical
    before and after; with the switch on, G1 = 0 after a KDE re-read differing only in its accent and results at the
    same scheme, a macOS re-read with another accent and a reading differing only in a string, and G1 ≥ 1 after a
    reading adding a colour statement and one changing a font size; S12's class-2 line (a stated colour read as
    stated); the records of lines 1 to 4.
  - **Load task 4b, `from_system` on the readers' fixtures** (`implement`, its diff reviewed before it merges, since
    the budget step runs its test from then on; its runs release, on the measuring machine, after load tasks 1 and
    2). *Done:* per reader, over its own preset (through a test-only construction that skips the table's refusal
    while the table is pending): the reading's size in flattened ops; the first reading (line 10); a re-read of an
    identical reading, of another accent, of the other scheme and of another font size (line 4); five runs of each,
    their spread, tolerance and first records, each case whose record does not meet its line under its named miss;
    the test added to the budget step's runs; the decision on load task 7 stated from them.
  - **Load task 6, the budget step in the gate** (`implement`): the step enters `check_release.sh`'s full run and
    M3's gate per unit. *Done:* the step's release run green on the branch, so every armed case was compared, every
    case armed or not binding, each case whose record does not meet its line under a named miss with its cause; its
    self-test as in load task P. The project's own parallel lanes keep the runnable count over 10 by themselves, so
    other lanes pause for the release run, or it sweeps again for 30 minutes and fails "machine busy".
  - **Load task 7, the readings proven at build time** (`implement-hard`), only where load task 4b shows a reader's
    first reading over line 10, or a re-read that changes its compared form over line 4: per reader, its maximal
    reading over its own preset validated by the build test, which records every statement whose `Fact` a check
    reads and proves the rest removable, and at run time a reading over a trusted form of its own preset, with no
    application layer or palette, accepted by membership. *Done:* `Fact`'s payload readable only through the
    recording module, its equality included (the `PartialEq` derive off `Fact`; `Val`'s and `OpKind`'s equality and
    `curves.contains` through the module), so a pattern or a `==` on a payload outside it does not compile; the
    proven places per reader; the build test's item for them, failing for a fixture preset that states an
    expression at a proven place (its positive control); G1 = 0 for each reader's maximal fixture reading and each
    with its removable statements left out, and G1 ≥ 1 for a statement outside its proven places, over a preset not
    its own and with an application layer present; the switch over the readers' corpus; the build test's added
    time; S12's class-2 line (the readings proven at build time); the records of lines 4 and 10, and of lines 1
    and 2 under a veto of A.
  - **Each part whose trigger fired** at the trigger step (*The load budget*'s deferred parts), by its agent type
    there. *Done:* its trigger line met in the probe on the presets that fired it, its records made and its named
    miss removed; or the miss that remains named with its cause and its new record, and reported to the maintainer
    with D6's other figures.

0. **What the connectors resolve** (ADD-A146). No bundled preset builds a theme while its `[pending]` table lists a
   path: `Theme::from_preset` refuses it (S17), `from_system` builds through `from_preset`, and the tables are empty
   only at a release (S21.1). So before task 1, and after the staging task above (its gate green; ADD-A147), the
   presets whose table is small are brought to an empty one by their lanes' rounds (`adwaita` 0 paths, `kde-breeze`
   4, `material-3` 9, `windows-11` 55 once the staging task is merged; a row with no route among theirs goes to the
   maintainer first, *How values are checked*), so that at least `adwaita` and `kde-breeze` build
   through `from_preset`. Its first step is code (`implement`; ADD-A147), before the facts of U180 and U505 are
   written: S2.2's operators `ceil` and `if_le` and the font metric `font_descent` (S8's `FontMetrics::descent`,
   S10's data list) in the model's code, with unit tests from S2.2's definitions whose vectors are the exactness
   checks of the D6 register design's `u180_ops_check.py` (QPainter's underline over its grid of inputs in 1/64 px
   and its ties, on both pen paths; WinUI's blink timeout for every blink time from 0 to 10,000 ms; the script's
   three positive controls each failing), and the scripts that read the operators and the metric
   (`check_facts.py`, `check_function_vectors.py`, `verify_facts.py`, `generate_preset.py`). Its second step is
   tooling, before the facts that use it: the labelled mapping's form (S17's `mapping`, S11's comment; D6,
   outcome 5) in `check_facts.py`, `check_fact_citations.py`, `generate_preset.py`, `check_preset_citations.py`
   and the gate's report; then the lanes of task 0's presets write their mappings (`windows-11`'s monospace style,
   U509, and the label on each of their text slots the platform names no style for). The macOS and iOS mappings (the
   status pairings of the catalogue's *global roles*, `status`, U456 and U755; `ios-*`'s `accent_foreground`, U651;
   their text slots) follow in those lanes' rounds after this step, beside M2's tasks (ledger queue #264, #265), and
   task 1 waits for none of them. A preset whose table it empties takes its plain route's records in
   `scripts/cost_budget.toml` if load task P has merged, else load task P records them ("first measured by" this task
   until then), and its synthetic route's cases and line 1's `read` leave the file (*The load budget*, *Who makes records*; ADD-A147). Until a preset's table is empty, a connector's tests reach it
   through the hidden `native_theme::gate`, which gains one function returning the synthetic `Theme` of a bundled
   preset, as S19.1's build test makes it (`from_preset_synthetic` over the preset's synthetic layers); the
   connectors' dev-tests and slice examples (task 1) call it, and no connector's public API names it. A connector test over every preset
   iterates `PresetId::all()` and takes the plain route where it succeeds. *Done:* the four tables empty (`adwaita`,
   `kde-breeze`, `material-3`, `windows-11`), their presets regenerated and `generate_preset.py --check` green; the
   first step's unit tests passing, with `u180_ops_check.py`'s three positive controls failing as they should; the
   second step's checkers' self-tests passing, among them a positive control in which a preset's `# mapping:`
   comment on a value whose fact has no `mapping` is refused. Rejected: tasks 4 and 5 before task 1
   (it gives up the reason the connectors come first, and still no preset would build a theme).
1. **The four connectors on the slice,** preceded by two tasks of their own. First, **the connectors' design**
   (ADD-A146), `docs/todo_v0.7.1_model-completeness-connectors-design.md`, checked as every design document is until
   a pass finds nothing serious, before any connector code:
   - per connector, its 0.7.0 public API item by item with a fate (kept, renamed, replaced by what, removed with its
     reason), as S16 does for `native-theme` (*0.7.0's public features the new model keeps* names each crate's
     items);
   - the 0.8.0 API: how an application gets a part's style, who builds the element tree and the window's
     conditions, where the states and the transitions runtime enter, and what the toolkit's theme object still
     carries; and, for motion, the four points the D6 staging design lists (the state kept per element part, drawing
     from `Drawn`, the request for the next frame, an identity and a frame time in every application-facing call),
     and the contract's deferred entries (ADD-A147);
   - the contract table's format, with its `pending` and stand-in entries, where a stand-in entry first appears
     (*Widgets 0.7.0 styles through stand-ins*), and the side S21.3 adds, what the connector supplies (its flags and
     their order, content variants, element data, root inputs, font metrics, the presences it reports, a row per
     declared keyed component), which the tables hold from the connectors' port on (ADD-A146);
   - one linked example per toolkit, `slice-<toolkit>`, an `[[example]]` of its connector crate (egui-widgets'
     widgets in egui's), which draws the slice's button in its states and which each unit's connector task extends
     with the unit's widgets, kept until M5 links the showcases again; its screenshots are compared at M2.1, and at
     each unit's connector task, with the platform host's captures of the same preset, scheme and state where the
     evidence repository holds them. The showcases are unlinked from M2.1 to M5, and the contract tests alone do
     not show what a connector draws.

   Rejected: the connectors written from S21.3's contract table alone (the specification states the table, not what
   an application calls; the three toolkit crates hold some hundreds of `pub fn` lines with no stated fate, and two
   implementers would build different crates). Before this task's runtime part, the Apple motion trial (D6, outcome
   3; the D6 staging design, 5.5): the alert's presentation and the activity indicator's loop read from the running
   animations on macOS 15 and 26 and iOS 18 and 26, at most two harness rounds per Apple harness (D6's budget for the
   trial, not a figure from the slice's evidence, whose Apple lanes needed several correcting rounds each; the
   fallback is a likely outcome); a motion measured on all four is stated on every preset, one that is not leaves
   0.8.0 on every preset, a loss for the maintainer. Second, if the loop stays, the motion runtime's interface (spec
   S2.7): `Motion`, `Places` (empty), `MotionUpdate`, `Drawn` and `Presence`, with `Motion::update` and
   `Motion::sample`, running the loops a part's style and its widget's `loop` value state and no transition,
   `MotionUpdate.speed_factor` from the resolved token, with a test per clause of S2.7's *A loop*, the connectors
   drawing every part through `Motion::sample` from then on; if the loop leaves, no `Motion` is built but for a
   connector whose design cannot show that the motion unit needs no change of its application-facing API (the D6
   staging design, 5.6). The engines, `Places`' content, `interpolate_color`, Compose's spring, `ComposeShapeMorph` and
   S4.3's theme change are the motion unit's (*After 0.8.0*), `interpolate_color` earlier only with a stated loop that
   interpolates a colour (ADD-A147). Then the connectors, gpui first, then iced, egui and egui-widgets: each one's old mapping deleted;
   its spinner and animated-icon code removed (`AnimatedIcon` is gone; busy indicators return with the indicators
   unit's connector task, drawn from the `spinner` widget); everything the model holds then mapped from the query, or
   listed as what the toolkit cannot draw; a widget 0.7.0 styles through a stand-in kept as a stand-in entry, from
   the enabled roles and widgets, where the slice holds all it reads (*Widgets 0.7.0 styles through stand-ins*); its
   contract test (S21.3); its `slice-<toolkit>` example, with the comparison the design states. The showcases are unlinked from their crates
   (the `[[example]]` entries removed with `autoexamples = false`, the `native-theme-showcase-support` dev-dependency
   removed, gpui's `mod showcase;` removed; files kept, back at M5), so `--all-targets` and the tests no longer build
   them. `check_connector_parity.py` is rewritten to apply its naming and sibling rules over the new resolved types'
   leaves. The steps listed until M2.1 leave `PLAN_EXCLUDED`. The connectors come first so they use the API before the
   rest of S16 is built on it.
2. The model's remaining arithmetic (S2.2), with unit tests. The platform functions come only with the facts that use
   them (M1b, M3), each with its vector table.
3. S12 complete.
4. S14 complete, with what M1b's accent run left of it (task T6b built `apply_palette` for a port alone; ADD-A145): the
   mapping source (`Palette::from_toml`), `Palette::with_port` and `ResolvedTheme::palette_status()`, S12's mapping
   check on every preset and `apply_palette` accepting and suspending a palette under a followed scheme it lacks
   (S12, S14; ADD-A146); then the bundled
   palettes, one for each of the ten schemes 0.7.0 ships of the seven projects, a variant as R19 defines it (four
   for Catppuccin, each with the accent 0.7.0's preset states and that accent's KDE port; S19.1, ADD-A146), as mappings with their labelled
   picks (the guide's own example value where it gives one, else the middle of the stated range; the picks listed in
   the ledger for the maintainer's veto); the KDE ports, with their measured facts added to the accent run; the
   projects' published colours as facts (a palette lane and its verifier); the gate's palette run, by S17's *The
   palette run* (each pick skipped). Over a preset whose widgets read keyed colours, a palette reaches them by S14's
   stated mapping method, or it is not offered over that preset (D6, outcome 7; ADD-A147): per project and preset,
   each keyed colour role the preset's values read that follows nothing the palette states, mapped once to the
   project's vocabulary, the table reviewed by a `review` agent before a palette is bundled over that preset; the
   design of the method is this task's first step. A palette is not offered over a preset whose widget colours the
   method cannot reach, measured literals that follow no role (`macos-*`, `ios-*`): a recorded limit (S14, R19;
   ADD-A147). The build test's trusted forms gain each bundled palette's source over each
   preset it serves, under each contrast condition (S19.1), and line 4's case of a bundled palette applied or removed
   takes its first record ("first measured by" this task until then; *The load budget*; ADD-A147).
   Beside it, `material-3`'s accent-derivation layer (its `m3_role` statements and S8's leave-out, S9.2; its forms
   join the build test's trusted forms, S19.1, ADD-A147), the gate's
   perturbed stand-in stating no `accent` there and the `material-3` accent run (S17; ADD-A134, ADD-A142; ledger queue
   #185), after the Material lane restates the preset's baseline (its roles Compose's sourced literals, its
   `input.accent` the baseline `primary`, U550) and records the run's facts for the accents it chooses.
5. S16 complete; S19.3 inside `native-theme` and in the four connectors (ADD-A146), which M2.1 ported with the free
   `icons` loaders and the resolved `token.icons` and which move here to the items this task builds:
   `PresetId::for_system` and `for_platform`, `ResolvedTheme::icon`, and `PaletteId` from task 4 for their pickers
   and contract loops. The package: `native-theme`'s `exclude = ["tests/"]`, and the packaging step fails where a
   `.crate` passes crates.io's 10 MB limit (*Settled by logic*, *The package*; ADD-A146).
6. S18: `property-registry.toml` generated with its drift test; `check_platform_facts_rows.py` and
   `check_preset_citations.py` rewritten to read the facts (S18); `check_todo_citations.py` rewritten to the model's
   paths; `check_section_refs.py` rewritten to the generated document's sections, or removed through an S22 line if
   nothing cites them any more; their steps and `ci.yml` entries leave `PLAN_EXCLUDED`.
7. S15 (AppKit rendering), in its own module, checked on CI's macOS legs.

Each task that adds a resolved field maps it in the four connectors in the same task, so M2 leaves no `pending` entry.
Token groups that add preset fields enter with the first M3 unit whose value reads them (S6.1; S4.3 with the motion
unit, S4.7 with the pointer unit, both after 0.8.0), and S13's placement with its unit, with their facts and vectors
(ADD-A147).
- **Gate per task:** as *Closing* says (its tests and `check_release.sh --quick`, every third task `--commit`; the
  pull request's CI where it touches macOS or Windows code), its facts gate as *The facts gate's cost* says
  (ADD-A146).
  **Milestone gate:** `cargo test --workspace` (outside `PLAN_EXCLUDED`; a connector's tests reaching a preset as
  task 0 says), `check_function_vectors.py`, the palette run, and no `pending` contract entry.
  **At M2's close** (D6, outcome 10; ADD-A147): a report to the maintainer of M2's cost, the gate's time and the
  load cost; M3a's approval is asked with the first real unit costs, and the maintainer may set caps per milestone
  then. Until that approval no lane writes facts ahead for M3b's units.

## M3 — Units (the per-unit pipeline)

The units, in order: readers (the traces' desktop `reader` lines whose targets are enabled, *The units 0.8.0
enables*; ADD-A146); placement (S13: the types and every strategy with
its vector table, generated by the reference where one runs: the GTK and Qt containers, Compose's code, the WinUI host
for microsoft-ui-xaml's strategies; measured by the probe and the iOS host for AppKit and UIKit, anchor, size and window
in, frame out); window and chrome; overlays; buttons and toggles; fields;
indicators; containers; the typed widgets the model lacks today, in alphabetical batches of about ten entries; the
keyed components; the units of depth, the parking units and motion (*After 0.8.0*; ADD-A147).

D13 splits them. M3a's units keep this order; readers and placement move after them, the first two of M3b (readers
by D6, outcome 4, *The units 0.8.0 enables*; ADD-A147), so 0.8.0's `Placement` values stay deferred until
placement's commit; M3b's units keep this order:

- **M3a, before M4:** 0.8.0's units (*0.8.0's scope*): window and chrome, overlays, buttons and toggles, fields,
  indicators, containers (the token groups entering by S6.1's rule; each unit's readers step reading what 0.7.0
  read for its targets; ADD-A147).
- **M3b, after M6:** readers, placement, the typed-widget batches, the keyed components, in the follow-up releases
  (*After 0.8.0*). The lanes may write their facts ahead, within the usage caps, once the maintainer has approved
  M3a at M2's close; until then none does (D6, outcome 10; ADD-A147). The gate skips them until their unit.
- **Every unit listed** (a code and script task at M3a's start, before its first unit's commit): M2's staging task
  lists in `scripts/widget_units.toml` every unit of depth and every parking unit, each with an `enabled` flag, and
  gives each staged path its parking unit or unit of depth (ADD-A147); this task adds every widget unit of M3's order
  with its entries, prefixes and an `enabled` flag, so that a key in no listed unit is a stray;
  `generate_widgets.py`, `check_facts_trace.py`, `tests/facts_gate/units.rs` and `check_ledger.py` read it, and a unit's commit flips its flag. Done early because it is one format
  change, after which each M3a commit changes one line and the lanes' facts written ahead are classified from the
  start; M4's failures on them wait until M4, since spec batches can still move an entry between units until then.
  The task also (ADD-A146) names each staged path's widget unit as M3 names it (the slice's
  `"token.behavior.scroll" = "scroll_view"` becomes `containers`, there being no `scroll_view` unit), and brings the
  per-unit gate's two new
  checks (below): each register row's kind and round and each lane's last closed round, which the ledger records
  and `check_ledger.py --unit <unit>` reads, and `docs/facts/verified.toml`, which `verify_facts.py` reads in place
  of its built-in baseline. Gate:
  `generate_widgets.py --check` and `--self-test`, `check_facts_trace.py --self-test`, `check_ledger.py --self-test`,
  `verify_facts.py --self-test`, `cargo test -p native-theme`.
- **The load calibration, the second look** (`worker`; D6, outcome 1; the D6 load design; ADD-A147): after M3a's
  first widget unit (window and chrome), the probe over the eight presets as that unit leaves them, and each trigger
  of *The load budget*'s deferred parts evaluated again on its figures, each outcome in the ledger; a part that fires
  there is built before M3a's second unit. Beside the facts gate's calibration (*The facts gate's cost*).

A ledger destination `M3 unit <name>` names the unit in whichever part it runs. Per unit:

1. **Facts** (the lanes, usually done ahead): every key, declaration and state-model combination of the unit at its
   depth on every preset (S17), with the traces' rows carried; what a lane writes beyond it is ahead-work for a unit
   of depth (ADD-A147).
2. **Verification** (*How values are checked*), then the spec corrections the unit found, and then the
   **alignment** (ADD-A146): where an applied class 2 batch renames or adds what the unit's facts state, the lanes
   restate those facts to the batch, and the changed facts are verified again (`verify_facts.py units` lists them),
   before step 4. The slice needed it more than once (its lanes' reconciliation to ADD-A122, their alignment to
   ADD-A125–ADD-A130, the ledger's *Units* and *Unmeasured register*); unstated, its cost and its second
   verification stood in no gate.
3. **The comparison with 0.7.0** (ADD-A147; *The depth 0.8.0 states*): per preset, 0.7.0's value of every field in
   the unit's *Today's fields* (and every other entry's mapping into it) beside what the preset at depth resolves for
   its target, in both schemes; each difference is classed by the D6 staging design's test: equal in the model's
   form; a sourced look the facts hold under a deferred name of the entry's own units (a variant, a state), which
   comes to depth; a value with no source, or from a source the preset's reference rule does not choose for it
   (R1.3, R1.4, R1.6), corrected and listed beside R19's with the rule; a sourced look that no name at depth can hold,
   or that only a shared unit's name holds (a flavour, a size, a condition), proposed as a loss for the maintainer,
   the shared unit named as where it returns. A changed look of the element with no variant, where 0.7.0 mapped it to
   a look that is not the platform's default and the facts hold 0.7.0's look under a variant, is a correction of the
   mapping, listed with the corrections.
4. **Model and presets, in one commit:** the unit enabled in the generator (or its hand-written fields), the platform
   functions it adds with their vectors, and the unit's parts generated into the eight presets (`generate_preset.py
   <id> --write`, their hand-written blocks with them; ADD-A146); the unit's new resolved fields enter each
   connector's contract table as `pending`.
5. **Readers:** the unit's new reads (the traces' `reader` lines), a task of its own (the gate resolves with a
   stand-in, so the readers do not change what it checks), with the pull request's CI on macOS and Windows.
6. **Connectors:** the unit mapped in the four connectors, one task in the series queue, emptying the unit's `pending`
   entries.

**The unit's pending facts** (ADD-A146). A unit's commit (step 4) does not wait for the unmeasured register: S17's
`[pending]` table lets a preset be bundled with pending facts, as the slice was. The register's rows are of two
kinds (*How values are checked*). A measurement to make is the lane's queue: the lane's next round on that platform
measures it, or records why it could not, which makes it a row with no route. A row with no route waits for the
maintainer's decision, and the series queue moves on to the next unit meanwhile. The unit closes when no pending fact
of the unit names a measurement row opened before the lane's last closed round on its platform began, and every row of the unit with no route is decided; so M4 meets stragglers, not every unit's whole
debt. Rejected: the unit waiting for the maintainer's decision on every open row, the rule from before the register
held measurements to make (every unit has such rows, so every M3a unit would wait on decisions that are lane work);
no condition before M4 (every pending fact of M3a would arrive at M4, which holds a tooling task, a drift run and a
gate, and no measuring).
- **Gate per unit:** `cargo test -p native-theme` (the facts gate over the whole model, on CI's shards with the
  listed-paths merge and locally in the per-preset steps, *The facts gate's cost*), `check_facts.py`,
  `check_fact_citations.py`, `check_function_vectors.py`, `check_facts_trace.py --unit <unit>` (each live trace row of
  the unit's targets is in a fact's `rows` or carries a verdict; for placement `--enabled`, since the deferred values
  its commit ends lie in other units' entries, which `--unit placement` does not check), `generate_widgets.py --check`
  and the pull request's CI green; for a unit citing Apple's SDK headers, `os-probe.yml`'s header-citation job green;
  and (ADD-A146) `check_ledger.py --unit <unit>` green (the unit's pending condition, above), and `verify_facts.py
  units` listing no new or changed fact of the enabled units, pending ones apart, since each platform's verified
  commit. `docs/facts/verified.toml` holds that commit per platform, the one its facts were last verified at; the
  orchestrator moves it when every verification unit of a delta has closed. Without it a fact a lane adds or edits
  after its unit's verification reaches the gate and the presets with nothing failing. From M2's load task 6, the
  budget step's unit run (*The load budget*; ADD-A147), which passes a time it could not compare as "not compared";
  a unit whose own change grows a named miss past its record (a widget unit enlarging the macOS presets) records the
  case anew with that cause.

## M4 — The facts gate, complete for 0.8.0's units

- Every unit of M3a enabled; `docs/facts/imported/` emptied (each row converted, or recorded as not a model value with
  its reason); S19.2's staged file removed; uncited legacy evidence deleted from the evidence repository and the
  legacy list emptied (with `imported/` empty no lane still converts a legacy row, and git history keeps the files);
  the accent run over every pinned scheme and accent; the palette run over every bundled palette.
- **The enabled units' checks** (a tooling task before the gate): the facts gate fails on a skipped fact of the
  enabled units and on a fact whose key lies in no unit, and reports the others by unit (*One rule for every model
  field*); `check_ledger.py --final` takes the units from `scripts/widget_units.toml` by the same rule and asks its
  per-fact conditions of the enabled units' facts only (no pending fact or declaration, every measured fact with a
  drift comparison, no open unmeasured-register entry whose key or pending facts lie in the enabled units), fails a
  register entry whose key lies in no unit, and reports the others by unit. A name of a state model, a condition of
  a fact or a staged path in no unit fails as a key in no unit does (ADD-A147).
  - **The gate's two modes** (ADD-A146). The ordinary run, from this task on, fails on what is skipped or refused
    among the enabled units' facts (a key, a `when`, a combination or a variant not held, a value unread) and on a
    fact whose key lies in no unit. The release mode, `FACTS_GATE_RELEASE=1`, which the gate already has (unsharded,
    over every preset), requires in addition no pending fact, listed path or synthetic value among them. Today it
    asserts that nothing at all is skipped, the facts of units not enabled among it, which D13 lets the lanes write
    ahead: the task restates the assertion over the enabled units, the facts of the others reported by unit, and the
    last follow-up release keeps today's (*After 0.8.0*). The full run sets the variable from M4 on, as it calls
    `check_ledger.py --final`.
  - **The drift comparison's tooling** (ADD-A146): `check_fact_citations.py --drift` and its file (*The drift run*),
    which `check_ledger.py --final` reads, and `--final`'s `--staged-file docs/facts/staged/inheritance-bases.toml`
    in the full run's call.
  - The check that the enabled units' facts were verified, as the per-unit gate has it (`verify_facts.py units`
    against `docs/facts/verified.toml`, *M3*; ADD-A146).
- **The drift run** (ADD-A146): each harness's drift job runs once more, on the then-current runner images, every
  pass a measured fact of the enabled units cites, at the same evidence path. `scripts/check_fact_citations.py
  --drift <run folder>` recomputes each such fact's value from the drift run's files by the fact's own evidence
  path, conversion and tolerance, and writes `docs/facts/drift/<release>.tsv` (fact file, key, `when`, value, drift
  value, verdict), which is committed; the drift run's files are not (*How values are checked*: a drift run commits
  evidence only where a fact cites it). `check_ledger.py --final` requires one line per measured fact of the enabled
  units, its verdict `same`, or the fact changed since the run; the ledger records the run's URL and the counts. A
  drifted fact is measured and verified again (citing the new run's committed evidence), or goes to the unmeasured
  register. Rejected: the comparison recorded per fact in the ledger (the slice alone holds 20,731 measured facts
  at M1b's close: no hand-kept table of that size can be kept or read, and no script could check it).
- **Gate:** `FACTS_GATE_RELEASE=1 cargo test -p native-theme --test facts_gate -- --exact facts_gate` green,
  unsharded, on the eight presets (ADD-A146: the release mode, on the thread count the machine's memory holds, *The
  facts gate's cost*), with no fact of the enabled units lacking a resolved key, no state-model combination of the enabled units' names lacking a fact (ADD-A147) and every declaration
  agreeing with its facts; `check_facts_trace.py --enabled` and `check_function_vectors.py` green;
  `check_ledger.py --final` green (`imported/` empty, the staged file gone, the legacy list empty, and over the
  enabled units: every measured fact with a drift comparison, no open register entry, no S17 pending fact);
  `verify_facts.py units` listing no unverified fact of the enabled units. The full
  run calls `check_ledger.py --final` from then on.

## M5 — Showcases (after M4)

`tools/showcase-support` ported first, then the three showcases rebuilt on the query, showing everything 0.8.0's
units hold (S18: the element lists and parity gates follow the connectors), what 0.7.0's showcases show among it;
their busy indicators drawn from the `spinner` widget (S19.3); the widgets 0.7.0 shows through stand-ins (the menu
bar, the number input) shown as their connectors' stand-in entries draw them; the unlinked showcase entries,
dev-dependencies and modules linked again.
- **Gate:** each connector's contract test with no `pending` entry (stand-in entries accepted); every element of
  `docs/showcase-elements.toml` at `v0.7.0` has its counterpart in the new element lists, or the ledger records why
  it goes (S19.3's animated icons); the showcase parity gates; the screenshots regenerated and compared side by side,
  twice (ADD-A146): with each other, per preset, as the showcases' parity rule asks, and, for each widget of
  0.8.0's units, with the platform host's capture under the same preset, scheme and state where the evidence
  repository holds one; a widget with none listed as not compared; the pairs in a generated index (the first shows that the connectors agree, only the second that the look is the platform's);
  `PLAN_EXCLUDED` empty and no `# PLAN_EXCLUDED` mark left in `ci.yml`.

## M6 — Release 0.8.0

- The CHANGELOG's 0.8.0 entry completed and dated; the READMEs, ROADMAP (the follow-up releases and their units,
  D13), `docs/todo.md`'s closed entries,
  `docs/COMPATIBILITY.toml` (`update_compatibility.sh`), `docs/new-os-version-guide.md` (facts per generation, the
  harnesses), the docs.rs build, `generate_assets_release.sh` and the provenance stamps, `docs/platform-facts.md`
  generated at the release's commit (not committed; D6, outcome 8; ADD-A147), the full `check_release.sh`.
- The two superseded connector plans (`docs/todo_gpui-full-theme.md`, `docs/todo_iced-full-theme-geometry.md`) archived
  to `docs/archive/`, which the later units do not need, their triage rows removed with them (`check_ledger.py` reads
  their items only while the files exist), and the links to them repointed in the same commit (ROADMAP's *Later —
  the connectors* links both, and `docs/todo.md` names the gpui plan; ADD-A146); the memory updated;
  `sweep-target.sh` run. The later units need the v0.7.1
  documents, this plan, the ledger and the scripts, so their archiving moves to the last follow-up release (*After
  0.8.0*).
- **Needs the maintainer:** the merge, the tag, the release and the publish.

## After 0.8.0 — M3b and the follow-up releases

- **The units:** M3b's, in M3's order: readers (D6, outcome 4; ADD-A147), placement (its commit ends 0.8.0's
  deferred `Placement` values), the typed
  widgets the model lacks today in alphabetical batches, the keyed components, the units of depth, the parking units
  and motion (*0.8.0's scope*; ADD-A147). Each runs M3's per-unit pipeline and
  its per-unit gate, its connector task included, unchanged.
- **A follow-up release** holds the units chosen when its branch is cut, listed in the ledger, each closed by M3's
  per-unit gate. A release that enables a typed widget, or a unit of depth that adds typed variants (ADD-A147), is
  breaking whatever else it holds (D14), so it takes every
  typed-widget batch closed when its branch is cut, and the breaking releases are as few as the batches allow
  (ADD-A146).
  Its branch is `v<version>-rc1`, cut from `main` after the previous release is merged and named by the number its
  chosen units give by D14 (a typed-widget batch among them: the next minor; none: the next patch); where the
  release's semver checks give another number, the branch is renamed before the release (ADD-A146).
  `v0.8.0-tools` goes on as the tooling branch, its draft pull request retargeted. An emergency fix
  between releases is a patch release from `main`, merged into the follow-up branch, as D11 says for 0.7.x.
- **What a follow-up release requires**, the gates 0.8.0 passed, for its new units and over every enabled unit:
  - M4's gate over the enabled units' facts, with a drift run over every measured one (the runners move between
    releases), no open unmeasured-register entry and no pending fact among them;
  - M5's gate for its units: the contract tests with no `pending` entry, a widget its units hold no longer a
    stand-in entry; its units' widgets in the showcases, with their element lists, parity gates and screenshots;
    `PLAN_EXCLUDED` empty;
  - M6's steps: its CHANGELOG entry, the READMEs, ROADMAP, `docs/todo.md`, `docs/COMPATIBILITY.toml`, the docs.rs
    build, the assets and stamps, the generated `docs/platform-facts.md`, the full `check_release.sh` (its semver
    checks deciding the number by D14), the
    memory, `sweep-target.sh`; the maintainer's merge, tag, release and publish.
- **The last follow-up release** (the one that enables the catalogue's last unit) also runs the gates the plan first set
  for M4: `check_facts_trace.py` with no option (every trace line) and the facts gate with no fact skipped; then the
  v0.7.1 documents, `tools/gap-map/`, this plan, the ledger, and the D6 notes (*D6*, *The D6 notes*; a `worker`
  task; ADD-A147) are archived to `docs/archive/`, the scripts go as the
  table below says, and `check_facts.py` takes its keys from the registry from then on. So do the kept scripts
  that read the catalogue or the units through the deleted ones (ADD-A146): `generate_preset.py`,
  `generate_platform_facts.py` and `verify_facts.py` import `spec_catalogue.py` or `check_facts_trace.py`, directly
  or through `check_facts.py`, and take their keys from the registry as it does; `scripts/widget_units.toml` and
  the gate's unit reader (`tests/facts_gate/units.rs`) go, every unit being enabled.

## The new scripts

"At the end" is the last follow-up release (D13), where M6 had it.

| Script | Made in | In `check_release.sh` | At the end |
|---|---|---|---|
| `spec_catalogue.py`; the three spec checkers | M0 | no (run by hand, S18) | deleted |
| `check_sources.py` | M0 | yes | deleted |
| `repin_citations.py` | M0 | no | deleted |
| `check_ledger.py` | M0 | the full run and `--quick` (`--commit` skips it by name); `--unit <unit>` in each unit's gate from M3a; `--final` from M4 (ADD-A146) | deleted |
| `citations.py` (the citation scanner, and the source cache's and the evidence's roots the other scripts read through it) | M0 | `--self-test` | kept for what the kept scripts import; its scanner of the planning documents goes with them (ADD-A146) |
| `import_platform_facts.py` | M1a | no | deleted at M4 |
| `convert_inheritance_rules.py` (S19.2) | M1a | no | deleted at M4 |
| `generate_widgets.py` | M1a | `--check` | deleted; the generated code is the source |
| `check_facts_trace.py` | M1a | `--enabled` | deleted |
| `verify_facts.py` (the verifiers' units, briefs, comparisons and audit sample) | M1b | `--self-test`; `units` against `docs/facts/verified.toml` in each unit's gate from M3a and at M4 (ADD-A146) | kept, for the verifiers of the next OS version's facts |
| `generate_preset.py` (ADD-A146) | M1a | `<id> --check` for each bundled preset, in every mode; `--self-test` | kept |
| `generate_sf_tracking.py`, `generate_dynamic_type.py` (the platform tables generated from the HIG's snapshots; ADD-A146) | M1a, M1b | `--check`, in every mode (`--commit` skips it, by name, only when the cache is absent); `--self-test` | kept, for the next OS version |
| `check_gate_cfg.py` (no `cfg` on the facts gate's path, so it runs on one configuration; ADD-A146) | M1b | yes, with `--self-test`, in every mode | kept |
| `check_pinned_citations.py` (a line citation of the hand-written `platform-facts.md` names its commit, *What carries over*; ADD-A146) | M1a's switch | yes, with `--self-test` | kept |
| `check_facts.py` | M0 | yes | kept |
| `check_fact_citations.py` | M0 | the full run fails without the cache; `--commit` skips it, by name, only when the cache is absent; the SDK-header citations run in a macOS job of `os-probe.yml` | kept |
| `check_function_vectors.py` | M1a | yes | kept |
| `generate_platform_facts.py` | M1a | `--check`, from the switch; from M2's task *`docs/platform-facts.md` not committed* a generation and `--self-test`, in every mode (D6, outcome 8; ADD-A147) | kept |
| `check_cost_budget.py` and its lines, `cost_budget.toml` (*The load budget*; ADD-A147) | M2's load task P | from load task 6: the release run in the full run; the unit run in each unit's gate (*M3*), on the measuring machine only; `--self-test` | kept, a release gate |
| `update_sources.sh` | M0 | `--verify` in the full run | kept |
| `measure_kde_accent.sh`, the harnesses and hosts under `tools/` | M0 spikes | no (CI) | kept, for the next OS version |

## How values are checked

- **Facts, mechanically** (`check_fact_citations.py`):
  - a `published` or `read` fact's `quote` occurs, whitespace-normalised, in the cited lines of the cited file at its
    pin, and every literal of its `value` (numbers after unit normalisation, colours in any notation, a derivation's
    operands, a text value as a whole string, list entry or font family, spec S17), after its `conversion`, occurs in the quotes of the citations serving each generation (spec S17);
  - a `measured` fact's evidence file is committed, the value at its `evidence` path equals the fact's value after its
    `conversion` within its tolerance (a measured derivation: the form of its passes, which the gate evaluates, spec S17), and the file records the pinned generation and toolkit versions (the exact
    builds listed);
  - every evidence file is cited by a fact, a state-model entry or a vector table (spec S17), except the legacy runs
    until M4. Evidence files are the files under
    `docs/evidence/<harness>/<date>-run-<id>/<image>/` in the evidence repository at its pin (*The evidence
    repository*); READMEs are not evidence, and a run's README stays while any
    file of the run is cited. Spike and drift runs record their result in the ledger with the run's URL and commit
    evidence only where a fact cites it.
- **Facts, independently:** per unit and platform, a verifier that is not the lane's author first derives the unit's
  state model and declarations from the sources, without the lane's, and the two are compared mechanically (a
  combination the lane missed would otherwise never be checked); then, for the agreed combinations, it reports a value
  and a citation for each key, never seeing the lane's, and both are compared mechanically. Each disagreement goes to a
  tie-breaker agent with both citations and the source. An audit sample of the agreed facts (5 % per lane and unit,
  at least ten, as during the slice; kept at D6, outcome 13; ADD-A147) goes to the tie-breaker too, to measure how often a lane and its
  verifier agree on a wrong value. The orchestrator reads the tie-breakers' summaries and every verdict that changes the
  catalogue or the specification. A low agreement or audit rate (below 95 %) starts a search for the systematic cause
  (a wrong pin, a misread format), not a blind re-run. For `measured` facts the verifier checks the evidence and the
  method. A sample alone is not enough: the spec's own trace checks found up to 6.7 % of rows wrong in a pass.
- **Platform algorithms** (S2.2's platform functions such as `m3_role`, `kde_effects`, `adw_standalone`, the Kirigami
  and kirigami-addons functions, `gtk_contrast_pick`, `uikit_prominence`, `sf_tracking`, `kcu_overlay`,
  `kqc_series_color`, S9.4's font scaling; S13's placement strategies): each has a vector table under
  `native-theme/tests/vectors/`, generated on CI by the reference itself at its pin, by one of three routes chosen by
  the lane and recorded:
  1. the reference's public API: Qt through the snapshot's PySide6; KColorScheme and KColorUtils (`kde_effects`,
     `kde_selection_inactive`, `kcu_mix`, `kcu_tint`, `kcu_shade`, `kcu_overlay`) by a small C++ program built in the
     KDE container against the snapshot's Frameworks; material-color-utilities' own code at its commit; libadwaita's
     and GTK's public functions in the GNOME container; Kirigami's QML functions in a QML engine;
  2. its observable behaviour through public API (a swatch's drawn check-mark colour for GTK's file-local `INTENSITY`,
     a scrolled window's adjustment after one synthetic wheel event, an avatar's style class);
  3. the pinned upstream source compiled unchanged in the container with a harness that calls the internal function
     (`gtk_css_shadow_value_get_extents`, KTextEditor's helpers), a CI tool never in the crate.

  Where the reference is closed (AppKit and UIKit placement), the measured frames (anchor, size and window in, frame
  out, each with its evidence) are the strategy's vector table; where a platform publishes a table (Apple's tracking),
  the table itself is the vector table, its rows carrying their citations; where nothing works, the function goes to the unmeasured register.
  `scripts/generate_qt_color_vectors.py`'s Python transcriptions are replaced (a transcription checks a port against
  another port). The model's own arithmetic has unit tests from the specification's definition.
  `check_function_vectors.py` checks that every `pub` or `pub(crate)` function of `platform` and every strategy of
  `placement` has a table and a test (spec S2.2, S13),
  and that every platform function or placement strategy any fact or preset calls is one of them (the model's own
  arithmetic is not one). Without this, a derivation fact and the preset would both be evaluated by the same function,
  and a wrong function would pass the gate.
- **The presets are generated** (ADD-A146) from the verified facts by `generate_preset.py`, whose `--check` fails
  where a bundled preset is not what its facts give; an author writes only its marked blocks (`[preset] generation`,
  the `[declare]` keyed namespaces, the `read` facts' values). So the gate's comparison of a preset with the facts
  proves the generator and the resolution, not the facts: a look the lane never wrote and a literal written where
  the platform derives pass it (R4). What proves the facts is the two checks above, the mechanical one and the
  independent one, and on a measured platform a pass per input that derived values follow.
- **Values not measured yet, and values that cannot be measured** (ADD-A146), each have a row in the unmeasured
  register in the ledger, which the pending facts name (spec S17, *A pending measurement*): the key and
  combination, the attempts and their evidence, and the options. A row is of one of two kinds, which the register
  states in its Kind column (`check_ledger.py` checks it), and from the task at M3a's start with the lane round that
  opened it (*M3*):
  - a **measurement to make** (`b`): a harness or probe pass on a pinned runner, an analysis of committed evidence,
    or a source read. It is the lane's queue, needs no decision, and is closed by the lane's next round, the measured
    or cited fact replacing the pending one;
  - a **decision** (`d`): a value with no route (no pinned source states it and no supported runner can measure
    it), or a choice between a measurement, a model form and a catalogue correction. A choice of model form the
    orchestrator settles from the sources and the project's rules, writes in the row's Decision cell marked "open
    to the maintainer's veto" and lists in the report of the milestone or unit that settled it (D6's in its veto
    list); it stands unless the maintainer vetoes it (D6, outcome 14; ADD-A147); a pure preference, or a value only the maintainer can supply (another setup: real
    hardware, another image), is the maintainer's decision. Either, while open, is a release blocker.

  A row that waits on a class 1 or 2 change (*Spec changes*) is the orchestrator's to apply, whatever its kind.

  Either kind, while open, blocks the release that enables its unit, since R21 releases no unit with a gap (D13);
  *M3* states when a unit's rows are due. A row all of whose pending facts are a later unit's is that unit's
  (ADD-A147). Rejected: every row the maintainer's decision, the rule from when the
  register held only values with no route (since spec ADD-A122 it also holds what is not measured yet: 235 rows at
  M1b's close, most of them, by the D6 package's classing, a pass, an analysis or a source read a lane makes
  without a decision).
- **Evidence** is committed to the evidence repository (GitHub's artifacts expire), images losslessly compressed
  (*The evidence repository*; decided by the maintainer on 2026-11-08, D6's evidence item).

## The load budget

Decided at D6 (2026-11-10, outcome 1; the D6 load design, its section 2; ADD-A147). Per preset, in a release build,
on the measuring machine. Every number is a judgment, proposed by the design and applying until the maintainer
changes it (D6's veto list items 9 to 23); a changed line is an edit of `scripts/cost_budget.toml` with its ledger
row, which the next run comparing the line's cases (at the latest the release run) applies, and the deferred parts'
triggers resting on it are evaluated again there. The rules behind the lines: *per frame*, the theme takes under a
tenth of a 120 Hz frame; *between two frames*, one change, one resolve and every part of a 300-part window asked
again fit one 60 Hz frame; *once*, the load alone fits one 60 Hz frame. Against 0.7.0 these numbers give up a load 21
to 35 times slower, a resolve 43 to 50 times, ten or more times the memory and 6.8 times the bytes.

| # | Line | Budget | What sets it |
|---|---|--:|---|
| 1 | `Theme::from_preset`, a preset's first load in the process | 16 ms | *once* |
| 2 | peak memory during that load, above the level before it | 16 MiB | twice line 3, transiently |
| 3 | memory kept: the theme and one window's resolution | 8 MiB | one full-HD surface (7.9 MiB) |
| 4 | a change every application makes: a reader's re-read accepted, `pick`, `set_flavor`, `set_contrast`, `set_accent`, a bundled palette applied or removed, `set_preset` to a preset a live theme holds | 0.5 ms | *between frames*: 0.5 + 1 + 300 × 0.05 = 16.5 ms |
| 5 | `Theme::resolve`, something changed | 1 ms | the same sum |
| 6 | `Theme::resolve`, nothing changed | 10 µs | *per frame*: 0.01 + 300 × 0.002 = 0.61 ms |
| 7 | a styled query, computed | 50 µs | line 4's sum |
| 8 | a styled query, repeated in one resolution | 2 µs | line 6's sum |
| 9 | bytes a preset adds to a binary | 256 KiB | between the data's compressed size and every binary carrying all eight presets |
| 10 | `Theme::from_system`, the theme's own part (the read of the preset and the acceptance of the reading) | 16 ms | *once* |
| 11 | peak memory during `Theme::resolve`, above the level before it | 8 MiB | no more than line 3 |
| 12 | the cost of adding an application layer that states structure | reported, no line | a line set at the load calibration if the maintainer wants one |

**What gates it.** The work, in `cargo test` on CI's three platforms: G1, the runs of the checks over combinations a
call makes (0, or at least 1, as load task 1 lists); G2, a preset's reads on the calling thread while its own theme
holds one, at most 1; G3, a `resolve` with nothing changed building nothing; G4, if the query cache is built, a
repeated query evaluating nothing. The times and the bytes, by the budget step: `scripts/check_cost_budget.py` over
the probe's output, on the measuring machine only, from load task 6, in a **unit run** (each unit's gate, *M3*) and
the **release run** (`check_release.sh`'s full run), never in `cargo test` or on CI. `scripts/cost_budget.toml`
holds each line's number and, per line, preset and case (the route, plain or synthetic, part of the case), either
its **record** (a figure, its tolerance as a fraction, the ledger row that made it) or why it has none ("first
measured by" a named task, or "not binding" with its ledger row); a case with a record is **armed**. It also holds
the **named misses**, each keyed by one case, with its cause (a task or a section of the D6 load design,
`docs/todo_v0.7.1_model-completeness-d6-load.md`, that removes it) and its ledger row. A figure or a record **meets its line** where it is at most the line. Four rules:

1. **Bound.** A case whose record does not meet its line has a named miss of its own, however close; a case that
   meets its line needs none; a case has at most one (naming one or more causes). A named miss without its cause or
   ledger row, a second one of a case and one keyed by a case the file does not list fail either run as file
   defects. With t the case's tolerance, a figure f fails where f > bound × (1 + t), the bound being the line, or,
   under the case's named miss, its record where that exceeds the line. A miss whose record meets a raised line is
   no error: the line is its bound, and it is printed as one that may go where its figure meets the line. A figure
   over a met line by less than its tolerance passes, printed "over its line, within its tolerance". Under a named
   miss whose record exceeds its line, f < record × (1 − t) prints "record stale: re-record <case>", which fails the
   release run. Every figure is printed beside its line and its record.
2. **Figures.** Bytes are counted by `peak_alloc` 0.3 (a hand-written `GlobalAlloc` is an `unsafe impl`) in a test
   binary of its own run with one test thread, and the preset files' sizes. A time is the lowest median of its
   passes that count, the passes being three sweeps over every case (one process per pass for lines 1 and 10; a
   figure under 1 µs timed over batches of 100 calls, 201 samples); the step sets nice 19 and idle I/O itself. While
   a pass runs the probe samples the runnable count (the first number of `/proc/loadavg`'s fourth field) every
   100 ms in a thread of its own; a pass counts where its mean is at most 10, the step's own threads included, and a
   time counts only where at least two of its passes count. The release run sweeps again over the times with fewer
   than two, starting no sweep later than 30 minutes after the build's end and judging a sweep already running; a
   time still short fails with "machine busy, run again", no verdict on the figures, the bytes still compared. A
   unit run makes the three sweeps alone and prints such a time "not compared"; the bytes are compared either way.
3. **Tolerance.** For a measured figure, twice the spread (the highest over the lowest, less one) of the runs that
   made the record (five for a time, three for bytes, measured as the release run measures), at least 5 %; for an
   exact count (line 9's sizes) 0. The record is the lowest of those runs. A re-record of a case under a named miss
   keeps the old record r where the new lowest lies within r × (1 − t) and r × (1 + t); only a record that changes
   needs a cause. A spread over 10 % is a defect of the probe, fixed before the record is made, so no tolerance
   exceeds 20 %.
4. **The release.** The full run passes only where its release run passed on the release commit (after "machine
   busy", the step alone run again on that commit) and every case is armed or not binding. The file defines the
   cases: the step asks the probe by name for each armed case; every case the probe can measure is in the file,
   armed, "first measured by" a named task or not binding, and one that is not fails either run as "unlisted case";
   a case asked for and not reported fails either run as "not reported". A unit run prints a case with no record,
   asks nothing for it and compares nothing, and passes a time "not compared". No run writes the file: the step
   prints the named misses that may go, and the release's ledger row holds the figures.

The load limit of 10, the 30-minute bound and the 5 % floor are judgments until load task P's runs; where three quiet
sweeps take more than half the bound, the bound is raised with its ledger row. Rejected: the one-minute load average
(it lags, both rising and falling); records written by a run (a named miss would grow by its tolerance at every
passing run); a unit run that waits for a quiet machine (unit gates fail on a machine other projects build on, while
the release run compares every armed case anyway); a bound at k times the line (a named miss unbounded, a met line
passing k times over); a ratchet at five times the record (no release bound to its line); figures reported only (no
regression of a read, a resolve or a query failing).

**Who makes records.** Only a deliberate act with its ledger row changes a record or a named miss: a task that
changes a cost, for each case its change may move by more than its tolerance, judged before the change is written
(load task P every case of lines 1 to 9 and 11; load task 1 lines 1 to 4; load task 2 lines 1 to 4; load task 3
lines 3, 5, 6 and 11; load task 4b line 10 and line 4's re-reads; load task 5 lines 3, 5, 7, 8 and 11; load task 7
lines 4 and 10, and lines 1 and 2 under a veto of R2.8's sentence; a part a trigger brings, the lines it changes);
the trigger step, on the regenerated presets; the load calibration after M3a's first widget unit; M2's fourth task,
for line 4's bundled palette case; the task that empties a preset's table (M2's task 0 for the small ones), for its
plain route; a unit whose own change grows a named miss past its record; a stale record's re-record with the fall's
cause, which the release run requires; and the release task, which gives a named miss with its cause to a case
printed "over its line, within its tolerance" at two consecutive releases. An act that records a case over its line
names its miss, one whose record meets the line removes it. Every line is armed from its first record (lines 1 to 9
and 11 from load task P's, line 10 and line 4's re-reads from load task 4b's, line 4's palette case from M2's fourth
task's); line 8 is not binding where the connectors' design shows no connector querying every frame, and line 6 is
armed whatever the connectors do, guarding E2.

**What it gives.** A regression past a bound fails at the first run that compares it, at the latest at the release;
a case over its line passes only under its own named miss, up to its record and tolerance, and the release run fails
on a stale record. A figure that grows while under its line does not fail; each release's ledger row shows the
growth. A met line stays exceedable within its tolerance, at most 20 %, by design (line 1's 16 ms may read 19.2 ms
at every run): the mark goes into the release's ledger row, and two consecutive releases bring the named miss
above. A unit run on a busy machine says nothing of the times; the release run compares them all.

**The deferred parts and their triggers.** A trigger is a measured miss of a line. Each part below is built only
where its trigger fires: at the trigger step on the presets the staging task regenerates (lines 1, 2, 3, 5, 7, 9
and 11; lines 4, 6, 8 and 10 with the load tasks that bring them), or at the load calibration after M3a's first
widget unit. A part that fires at the trigger step is built before M3a's first widget unit, one that fires at the
calibration before its second. The projection to thirty widgets is printed beside each figure and fires nothing.

- **Sparse patches** (each block's patch boxed in a part's patch, so a rule costs what it states; `implement`, its
  gain measured on one preset first by a `worker`): line 3 missed in the probe's kept bytes, or line 2 after the
  compact form, on a preset whose held bytes per rule are the cause.
- **The compact form** (generated beside the presets and committed), or the text compressed at compile time: line 1
  or line 9 missed. A `worker` spike first (decode time and size on `windows-11` and `material-3`); if it proceeds, a
  design note checked as every design document, then `implement-hard`; the compressed text `implement`.
- **Building by need** (a trusted preset's widgets decoded at their first query; `implement-hard`): line 1 missed
  after the compact form, or its spike decoding no faster than the text's read.
- **The kept rules held by reference** (`Resolution`'s kept rules shared, not cloned; `implement`): line 5 or 11
  missed, and load task 4a's profile showing those clones are the cost.
- **Resolution by need** (a part's rules resolved at its first query, kept per resolution; `implement-hard`): line 5
  or 11 missed after the rules are held by reference.
- **The readings proven at build time** (load task 7): in load task 4b, line 10 missed by a reader's first reading,
  or line 4 by a re-read that changes its compared form.
- **The repairs inside the checks** (the reuse memory extended to every check, the repeated enumerations removed;
  `implement-hard`): the limits of *The facts gate's cost* reached; line 12 set and missed; or, after load task 7, a
  reader's statements left on the checks with line 10 or line 4 still missed.
- **A faster query** (the repair load task 4a's profile names; `implement`): line 7 missed.
- **The query cache** (load task 5): a connector querying every frame and no repair bringing the query near line 8.
- **A feature per preset** (`implement`): line 9 missed for the eight together after the compact form, or the
  maintainer's wish.

Line 12's application layer runs the checks once when added and again whenever something beneath it changes, so
until a line is set such a layer is added before the first frame.

## Spec changes

The orchestrator is the one writer of the specification, the catalogue, the traces and the gap maps. Lanes,
verifiers and implementers report what they find wrong to it, and it records each in the ledger's queue. Three
classes:

1. **Data corrections** (a citation, a line number, a trace row, a catalogue entry's platform detail): applied by the
   orchestrator in batches.
2. **Model changes** (a field, a condition, a function, a rule in S1–S21, a *Today's fields* selector or a Depth line
   of the catalogue, which decides what a release states; ADD-A147): an S22 line, and independent checks of the
   batch until one finds nothing serious, before it is applied (ADD-A146: the maintainer's rule for every
   specification, which the slice's batches followed). Rejected: one check, whatever it finds.
3. **A change to a rationale decision** (R1–R25): the maintainer's. The orchestrator may decide one by the
   maintainer's earlier decisions and the project's rules, and applies it marked at its rule as awaiting the
   maintainer's veto, with its veto list item, as at D6; it stands unless the maintainer vetoes it (ADD-A147).

After each batch the orchestrator runs the spec checkers and `generate_widgets.py --check` and tells the running
tasks; the generator's drift check makes the code follow.

**The documents' checks** (D6, outcome 15; ADD-A147). After the batch that applies D6 (ADD-A147), one full pass of
independent checks over all the documents (the rationale, the specification with its catalogue and traces, this
plan and the ledger); its fixes are checked until a check finds nothing serious, each further pass reading the
documents the fixes changed and what the findings implicate, not the whole set again. From then on each unit's batch
is checked as it is written (class 2, above). Rejected: a full pass before the batch (it would read texts the batch
rewrites); no further full pass (the maintainer's document standard asks for checks until a pass finds nothing
serious). A full pass with its fixes costs about 16 M of usage (D6's package), hence one, then passes by need.

## How the work runs

- **Roles:** an Opus session orchestrates; code tasks run in `implement` agents with a failing test, a compile or a
  fixture as their gate, each reviewed by a fresh reviewer before it closes (G4's precedent); facts lanes run one agent
  per platform and one for the palettes, briefed with the pinned sources, the traces and the facts format; verifiers,
  tie-breakers and clean-room implementers as above; tooling in its own agents on `v0.8.0-tools`. Broad searches go to
  the `locate` agent (Haiku).
- **Briefs:** every agent reads and searches only its worktree, the project, the source cache and the scratchpad
  (verifiers and clean-room implementers less, as above), and never searches the whole filesystem (no `find /`).
- **Failures:** a subagent that fails the same gate twice is not dispatched a third time; the orchestrator takes the
  task over or re-specifies it.
- **Parallel by default:** the lanes, the verifiers, the reviews and the tooling run beside the code, and code tasks
  on disjoint files beside each other in their worktrees (*Where the work happens*); the merges, the commits and
  the gates on `v0.8.0-rc1` stay sequential.
- **Usage** (ADD-A146): as many agents and build slots at once as the maintainer last set, which the ledger records
  (on 2026-11-09 four agents, six where two of them run no `cargo`, and two build slots, the gate and one `-p` build;
  an agent waiting on a CI run does not count). No token cap since 2026-10-30 (D7), and none set at D6 (outcome 10;
  ADD-A147): the lanes work ahead into M3a's units within the agent limit alone, and into M3b's only once the
  maintainer has approved M3a at M2's close, where the maintainer may set caps per milestone.
- **Disk:** `~/.claude/scripts/sweep-target.sh` at each milestone's close and whenever the build folder passes about
  200 GB.
- **Progress** is recorded in the ledger and the project memory.

## Checklist

- [x] D1, D4, D5, D7, D9, D10, D11, D12 decided (2026-10-29)
- [x] `v0.7.1-rc1` merged to `main` (D5; fast-forwarded on the maintainer's request, 2026-10-29)
- [x] M0 setup (2026-10-30)
- [x] Spikes M · G · K · I · W · A (2026-10-30, all yes)
- [x] M1a slice core (2026-11-06)
- [x] M1b slice complete (2026-11-10)
- [ ] D6 checkpoint (its outcomes applied as the work's basis, ADD-A147; the veto list with the maintainer)
- [ ] The full document pass after ADD-A147 (D6, outcome 15)
- [x] D6's evidence item: the evidence repository (the maintainer, 2026-11-08)
- [x] The evidence repository's switch (the tooling task, then *The evidence repository*'s steps; 2026-11-08)
- [x] D13 0.8.0's scope, (B) (2026-11-06)
- [x] D14 the follow-up releases' version numbers, (a) by logic (2026-11-06)
- [ ] M2 model core
- [ ] M3a 0.8.0's units (per unit in the ledger)
- [ ] M4 facts gate complete for 0.8.0's units
- [ ] M5 showcases
- [ ] M6 release 0.8.0
- [ ] M3b later units and the follow-up releases (per release in the ledger)
