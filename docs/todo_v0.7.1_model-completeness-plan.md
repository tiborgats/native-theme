# v0.7.1 → 0.8.0 — Model completeness: Plan

**Status:** draft 3, 2026-10-29, for the maintainer's approval (revised after two independent reviews).

**Goal:** the model the specification states, implemented; the eight platform presets on six platforms written from
facts and passing the facts gate; the bundled palettes checked against their projects' published colours; the four
connectors and the three showcases moved to the model; released as 0.8.0 (R21, S21).

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

**Branch:** `v0.8.0-rc1`, cut from `main` once `v0.7.1-rc1` is merged (D5), with a draft pull request to `main` from
the first push, so that every push runs CI (`ci.yml` runs on pull requests, not on branch pushes). Nothing is released
before every milestone below is closed (R21).

## Global constraints

- **No invented value.** A value enters a fact only from a source pinned to its platform's release set (kind
  `published`), from an API a reader calls (kind `read`, its definition cited), or from a measurement recorded with
  its method, input, output and toolkit versions (kind `measured`, with its tolerance) (S17). Every fact is checked
  mechanically and by an independent verifier; every platform function the model implements is checked against its
  reference implementation (*How values are checked*). A preset's declarations (availability, variants, sizes,
  container sizes) are facts too.
- **No runtime panics.** `unsafe` only inside the existing module exceptions (`native-theme`'s `macos.rs`,
  `windows.rs`, `sficons.rs`, `winicons.rs`, `watch/macos.rs`, `watch/windows.rs`; the probe's `macos/ffi.rs` and
  `windows/ffi.rs`), S15's new module (approved 2026-10-29) and what D4 approves; every `unsafe` block carries its
  safety argument, as `sficons.rs` does.
- **Gates are mechanical.** Every task names the command whose result closes it. A visual comparison is kept where the
  project's rules ask for one, recorded with its screenshots, beside the mechanical gate, never instead of it.
- **One writer per file at a time.** Parallel work owns disjoint files; the specification, the catalogue, the traces,
  the gap maps and the ledger have one writer, the orchestrating session.
- **Captures announce themselves** when they take the desktop's focus, and are run and compared by the agent.
- **Closing.** Each task ends with `scripts/check_release.sh --commit` green on its last commit (and the pull
  request's CI green when it touches the macOS or Windows code, which the local run does not build); each milestone
  ends with the full `scripts/check_release.sh`, with only the expected items the ledger lists, its documents updated,
  a commit and the branch pushed (standing grant for work branches).

## The transition from today's model

The new model **replaces** today's in place, at M1a; no second model is kept beside it (keeping one means two derives
and two preset formats maintained for no gain, and the old one is deleted at the end anyway).

- **One rule for every model field** (a widget's part, a component, a token group, a colour-role group, a declared
  input): it enters the model in the commit where all eight presets state it from verified facts, or declare its
  absence as the facts state. So S12 stays strict, S19.1's build test and the facts gate run over the whole model at
  every commit, and no gate needs a scope filter. Generated declarations are staged by the generator's list of enabled
  units (`scripts/generate_widgets.py`); hand-written ones (S3's roles, S4's tokens) are written in their unit's
  commit. M4's gate requires every unit enabled.
  - Rejected: declaring unwritten widgets `available = false` (the presets would state that the platform does not draw
    them, which is false); a lenient validation mode (it hides exactly the gaps the gate exists to find); placeholder
    preset values (invented).
- **What cannot build meanwhile is listed, not silently skipped.** One variable, `PLAN_EXCLUDED`, in
  `scripts/check_release.sh` names each excluded step and the milestone that brings it back, and the summary prints it;
  `ci.yml`'s matching jobs carry a condition naming the same plan. At M1a it holds: the four connectors (clippy, the
  strict panic check, tests, docs, package, examples), `tools/showcase-support`, the showcases, widget coverage,
  connector and showcase parity, the screenshot stamps (until M5); the readers' and watchers' features (until M1b);
  `native-theme`'s examples (until M2); the data-document checks, which check today's registry, presets and
  `platform-facts.md` sections (`check_platform_facts_rows.py`, `check_preset_citations.py`, `check_todo_citations.py`,
  `check_section_refs.py`, `check_wrong_safety_nets.py` and the `inheritance-rules.toml` checks; each rewritten or
  removed in M2). M5's gate is the list empty and the conditions gone.
- **Today's files that carry over:** today's `platform-facts.md` rows and evidence become facts (M1a's import); S19.2's
  conversion of today's inheritance rules runs at M1a into a staged input file that each unit's preset step uses and
  the gate checks (removed at M4); the ten community preset files are removed at M1a, since the palettes are built from
  the projects' published colours (M2), not from them.

## Decisions

**For the maintainer, at approval:**

- **D1 — the source cache and the release sets.**
  - Location `/run/media/tibi/work/native-theme-sources/` (the work SSD that builds already need mounted, outside
    `/home`'s snapshots), read and written by this work and its agents: it needs a read and write grant. Options
    weighed: a git-ignored folder inside the project (no grant, but every fetch of several GB lands in `/home`'s
    snapshots); the session scratchpad (lost at reboot).
  - The release set: per platform, the newest stable release at M0 (R1.5) of a coherent set (Plasma 6.8.x's Breeze,
    KWin, KDecoration and plasma-workspace with the KDE Frameworks and Qt it requires; libadwaita 1.10.0 and GTK 4.24.1
    with their gsettings schemas; a Compose BOM with its material-color-utilities and AOSP tags; a microsoft-ui-xaml
    commit; an Xcode and SDK per Apple generation), frozen until 0.8.0 unless the ledger records a re-pin. Recorded in
    `docs/facts/sources.toml`, one pin per source and generation. Rejected: pinning `master` commits (unreleased code no
    user runs and no measurement host can run); re-pinning per fact (every lane redoes the mapping and versions mix).
  - The list of sources is generated from the citations in the traces, the gap maps and `platform-facts.md`
    (`scripts/check_sources.py` fails on a cited project with no entry), so it is not kept by hand. Apple's HIG and API
    pages are snapshotted as their DocC JSON (date, SHA-256); Microsoft Learn's pages are pinned as commits of their
    public MicrosoftDocs repositories; Apple's SDK headers exist only on macOS runners, so their citations are checked
    by a macOS CI job at the pinned Xcode.
  - Approving now lets M0 seed the cache from this session's scratchpad checkouts whose commit equals the pin
    (plasma-workspace v6.8.2, libadwaita 1.10.0, GTK 4.24.1) before a reboot clears them; the others are fetched.
- **D4 — `unsafe` and the measurement hosts.**
  - The readers' and watchers' new reads (the traces' `reader` lines: 75 on macOS, 42 on Windows) grow under their
    existing module exceptions (`macos.rs`, `windows.rs`, `watch/*.rs`), in the same form.
  - The probe's measurements grow under its existing exceptions (`tools/os-probe/src/macos/ffi.rs`, `windows/ffi.rs`).
  - The hosts, none with Rust `unsafe`: iOS, a small Swift app built by Xcode on the runner (rejected: Rust with
    objc2-ui-kit, a third `unsafe` module for a tool); Android, a Kotlin Compose app; KDE, the `qml` runtime with QML
    files (qqc2-desktop-style, Kirigami) and PySide6 for Qt Widgets with the Breeze style (the project already pins
    PySide6 for its colour vectors); GNOME, a C program modelled on libadwaita's `doc/tools/screenshot.c`; WinUI, a
    small C# Windows App SDK app only if spike W shows values microsoft-ui-xaml does not state.
- **D5 — `v0.7.1-rc1` to `main` first** (recommended): the maintainer merges the v0.7.1 documents and scripts (no crate
  change) before `v0.8.0-rc1` is cut, so `main`'s ROADMAP states the next work and the 0.8.0 pull request holds only
  0.8.0's work. The alternative, cutting from `v0.7.1-rc1` and merging both at 0.8.0, also works.
- **D7 — the budget for M0–M1b:** a cap of +15,000k tokens, which D6 revises. The basis is an estimate, not a
  measurement: one independent review of this plan used about 200k tokens; M0–M1b holds six spikes, eight presets'
  slice facts with their verifiers, two implementation milestones with a reviewer per task, and the reviews.

**Scheduled:**

- **D6 — the checkpoint after M1b:** the slice's measured cost per widget and platform (facts written, hours, probe
  rounds, tokens) extrapolated to the whole catalogue; the evidence's size extrapolated, with the choice of where it
  lives above a threshold (in the repository; a separate evidence repository pinned by commit, recommended above about
  500 MB; Git LFS, rejected for its quota and bandwidth costs); the facts gate's run time, with a budget; the spikes'
  results; the unmeasured register's open entries; the budget caps per milestone. The maintainer approves the
  scale-up.

**Settled by logic (recorded):**

- **D2 — all measurement on CI.** The KDE, GNOME and Android harnesses run on CI's Linux runners in containers pinned to
  the release set: KDE from a dated Arch Linux Archive snapshot carrying the pinned Plasma, under `dbus-run-session`,
  offscreen; GNOME offscreen with libadwaita's debug settings and fonts pinned as `screenshot.c` pins them, so no
  portal setting leaks in; Android on the emulator with KVM. Nothing is installed on this machine, nothing touches the
  desktop, and the versions never drift. Rejected: this machine's Plasma (a rolling distribution moves past the pin
  during the work); local packages or a Flatpak runtime for GNOME (installs, and takes Plasma's portal settings unless
  pinned); a GNOME OS virtual machine (needed only for what the shell draws, out of scope by R1.1); Paparazzi for
  Android (layoutlib is not the device's renderer).
  - The KDE accent script (`scripts/measure_kde_accent.sh`) applies another scheme first (a fresh config already names
    Breeze Light and the tool then writes nothing; `applyScheme` deletes each colour group before writing it,
    `colorsapplicator.cpp:184`, so nothing of the first scheme remains), then the accent in a second call (the tool
    takes the scheme or `--accent-color`, not both), and checks that kdeglobals changed.
- **D3 — macOS, iOS and Windows on GitHub's runners.** The repository is public, so hosted runners cost nothing; the
  limits are concurrent macOS jobs and the six-hour job limit. macOS 15 and 26 on `macos-15` and `macos-26`; iOS 18 on
  `macos-15`'s simulator and iOS 26 on `macos-26`'s (checked by spike I); Windows 11 on `windows-11-arm` (Windows 11's
  look). Today's evidence from `windows-2022` and `windows-2025` (Windows Server) is re-measured once on
  `windows-11-arm` and compared; the two only build afterwards. Every job is added to `os-probe.yml` and dispatched by
  hand (`gh workflow run os-probe.yml --ref v0.8.0-rc1` runs the branch's version), because `workflow_dispatch` needs
  the file on `main`. The runners are virtual machines; whether they have a GPU is probed once (spike M).
- **D8 — `m3_role` and the other ports:** a platform function is a port of its reference implementation, unless a
  maintained Rust crate passes the same vector table at the same pinned version (*How values are checked*), in which
  case the crate is used (less code). The vectors decide, not a preference.
- **The facts gate reads `docs/facts` at run time:** the facts are a directory of many files, which `include_str!`
  handles badly; the test is excluded from the package, so the packaged crate's tests do not look for files outside
  it.

## The order

```text
M0 setup ──► M1a slice core ──► M1b slice complete ──► D6 ──► M2 core ──► M3 units ──► M4 gate complete ──► M5 showcases ──► M6
  │  ▲           ▲   │                ▲                         │   (each unit closed ──► its connector task)
  │  │           │   └─► L facts lanes and verifiers (published from M1a; measured as T delivers) ──► M1b, M3
  └─► spikes (K, I, M first) ──► T tooling (probe passes, hosts, harnesses, reference vectors) ──► L
      M's button pass ──► M1a
```

Code runs in series, one implementer on the tree at a time (parallel implementers in one tree see each other's
half-written files; separate worktrees each cost a whole build folder of tens of GB); connector tasks join the same
queue. The parallelism is in the facts lanes, the verifiers, the reviews and the tooling, which touch no code the model
builds.

## M0 — Setup

- Cut `v0.8.0-rc1` (D5); open the draft pull request; add `PLAN_EXCLUDED` (empty) to `check_release.sh`.
- The ledger `docs/todo_v0.7.1_model-completeness-ledger.md`, owned by the orchestrator: per unit and platform the
  status of facts, verification, presets, gate and connectors; the expected items of the full check; the spec-change
  queue; the unmeasured register; the verifiers' agreement rates; the evidence size; the tokens per milestone (the sum
  of the agents' reported usage). This plan's checklist stays at milestone level.
- `check_spec_names.py` and `check_spec_entries.py` gain `--self-test` (`check_spec_trace.py` has one); the catalogue
  parser moves into `scripts/spec_catalogue.py`, shared with the generator and the facts checker.
- The source cache (D1): `docs/facts/sources.toml`, `scripts/check_sources.py`, and `scripts/update_sources.sh`, which
  fetches each entry at its pin (androidx and AOSP shallow and sparse) and whose `--verify` checks every checkout's
  commit and every snapshot's hash.
- **Re-pin:** `scripts/repin_citations.py` maps every citation in the traces, the gap maps and `platform-facts.md`
  from the commit it was read at (`master` for most KDE sources, read 2026-10-27; Breeze v6.7.5 in
  `platform-facts.md`) to the pinned release, by the line mapping between the two commits, and lists each citation
  whose cited text changed or vanished; the orchestrator re-reads and corrects those. Before any lane starts.
- The specification's amendments this plan needs, through *Spec changes*:
  - S17's fact gains `rows` (the gap-map rows it carries), `quote` (`published` and `read`: the source's exact text at
    the citation), `conversion` (a closed list: points to pixels, a fraction's complement, …, applied by the checker),
    `evidence` (`measured`: the committed evidence file, which records the toolkit versions).
  - Facts stay one directory per platform; a fact for one generation carries `preset:<id>` (S7.1's `Preset`
    condition) in its `when`, and a fact without one cites a source or evidence for each generation its directory
    covers (rejected: one directory per preset, which duplicates every shared fact).
  - A preset's declarations (availability, variants, sizes, container sizes) are stated in `state-models.toml`, and
    the gate checks each preset's declarations against them.
  - The palettes' published colours in `docs/facts/palettes/<project>.toml`, and a palette run in the gate (S14).
  - S5, S6.1 and S18: the declarations are generated from the catalogue by `scripts/generate_widgets.py`, with a drift
    check until the catalogue is archived, after which the generated code is the source (rejected: a proc macro
    reading the catalogue, which lies outside the crate and is archived; hand-writing 114 entries with no drift
    check).
  - S2.2: each platform function has a vector table from its reference implementation.
  - S18: the scripts table below.
- **Triage:** every open item of `docs/todo.md` whose subject is the model, a preset, a reader, the facts, the probe
  or a connector, whatever its tag, and each of the catalogue's *Open items*, gets a destination in the ledger: a
  named milestone, unit or lane, settled by the model, or carried to a later version with its reason.
- **Spikes** (each ends with yes or no and its evidence in the ledger), by risk, three at a time: K, the KDE accent
  harness in its container (D2); I, the iOS simulator host; M, the macOS probe's button pass (hovered, pressed,
  disabled and checked in a key window, at each size) first, then its other missing passes (a wallpaper and
  transparency setting under control, the drawn against the read window background) and the runner's GPU; then W,
  whether microsoft-ui-xaml states the slice's keys and those of ten sample widgets, and a WinUI host if not; G, the
  GNOME harness; A, the Android host.
- **Gate:** the spec checkers' self-tests and runs green; `check_sources.py` and `update_sources.sh --verify` green;
  `repin_citations.py` reports no unresolved citation; `scripts/check_ledger.py` (every open item counted from
  `docs/todo.md` and the catalogue has a destination) green.

## M1a — The slice's core: `button`, colour roles and the slice's tokens on two presets

- **Presets:** `adwaita` (published) and `macos-tahoe` (measured by spike M's button pass). If that pass is not ready,
  `windows-11` (published from microsoft-ui-xaml's visual states) takes `macos-tahoe`'s place, which moves to M1b.
- **Code** (today's model, `native-theme-derive`'s derive, the preset format and their tests replaced; `PLAN_EXCLUDED`
  filled): S1; S2's blocks in full with S2.2's grammar and the functions the two presets' facts use; S3; S4.1, S4.2,
  S4.4, S4.5, and S4.6's groups the button and the roles use (`token.params.pointer_repeat`, `token.params.color`,
  `token.params.opacity`, …); S5 and S6 for `button`, through the generator and the new derive; S7; S8 without S13;
  S9.1, S9.2, S9.4; S10; S11; S12 for what the model holds; S16's `Theme`, `from_preset`, `set_reader_stand_in`,
  `set_accent` and the query; S17's gate with its normal and perturbed runs and the declarations check. `PresetId`
  holds the presets written so far. `native-theme-build` follows if S2.9 or S16 changes `IconProvider`. The README's
  examples are rewritten on the slice's API (they are doctests).
- **Facts tooling:** `scripts/check_facts.py` (form: keys from the catalogue through `spec_catalogue.py`, `when` as
  S11's, kind and source well formed, the source pinned, the value as S2.2's, the generation rule);
  `scripts/check_fact_citations.py` (*How values are checked*); `scripts/generate_platform_facts.py` with `--check`;
  `scripts/import_platform_facts.py`, which converts today's `platform-facts.md` rows and their evidence to facts
  wherever the key maps (through the re-pin) and keeps the rest in a legacy appendix of the generated document.
- **Gate:** `cargo test -p native-theme` (the facts gate and the doctests included), `check_facts.py`,
  `check_fact_citations.py`, `check_function_vectors.py`, `generate_platform_facts.py --check` and
  `generate_widgets.py --check` green.

## M1b — The slice complete

- **Presets:** the other six at the slice's scope, `ios-26` created (its `PresetId`, its file, iOS 26 as its
  generation, its `docs/todo.md` entry closed), all from verified facts.
- **Code:** the platform functions the six presets' facts use, each with its vector table (expected among them
  `kde_accent`, `kde_accent_wm`, `kde_effects`, `kde_selection_inactive`, `m3_role` and Qt's colour helpers; the
  lanes' facts name the full list at M1a's end); S4.8's `token.kde` and `token.material` groups; the four readers
  ported to S9.3 and S9.5 at the slice's scope (`ReaderState`, the KDE scheme groups and unset keys), whose features
  leave `PLAN_EXCLUDED`.
- **The accent run:** `Palette::from_port` and `apply_palette` (S14, as far as the run needs); the KDE schemes and
  accents chosen and pinned (at least one that tints, one with the accent-coloured title bar and a `Header` group, one
  with it and no `Header` group, one lacking a colour group, S17), measured by spike K's script.
- **For D6:** the cost, evidence size and gate run time measured and extrapolated.
- **Gate:** as M1a's, over all eight presets, with the accent run; the pull request's CI green on macOS and Windows.

## M2 — The model's core, complete (after D6; in series)

1. **The gpui connector on the slice:** its old mapping deleted, the colour roles and `button` mapped from the query,
   its contract test (S21.3); its clippy, strict panic and test steps leave `PLAN_EXCLUDED` (its showcase stays). It
   comes first so a connector uses the API before the rest of S16 is built on it.
2. S2.2's remaining functions, each with its vector table.
3. S13 (placement).
4. S12 complete; `scripts/check_refusal_tests.py` checks that every `LayerError` and `Problem` variant has a test that
   triggers it.
5. S14 complete: one palette per variant each of the seven projects publishes (S19.1), as mappings with their labelled
   picks (the guide's own example value where it gives one, else the middle of the stated range; the picks listed in
   the ledger for the maintainer's veto), the KDE ports, the projects' published colours as facts (a palette lane and
   its verifier), the gate's palette run (each pick skipped).
6. S16 complete; S19.3 inside `native-theme`; the examples move and leave `PLAN_EXCLUDED`; S21.2 (exhaustive enums,
   `#[non_exhaustive]` structs as S21.2 names them).
7. S18: `property-registry.toml` generated with its drift test; `check_wrong_safety_nets.py` and the
   `inheritance-rules.toml` checks removed; `check_platform_facts_rows.py` and `check_preset_citations.py` rewritten to
   read the facts (S18); `check_todo_citations.py` rewritten to the model's paths; `check_section_refs.py` rewritten to
   the generated document's sections, or removed through an S22 line if nothing cites them any more; their
   `check_release.sh` steps and `ci.yml` jobs changed, leaving `PLAN_EXCLUDED`.
8. S15 (AppKit rendering), in its own module, checked on CI's macOS legs; it may run beside 2–7, since it builds only on
   macOS.

Token groups that add preset fields (S4.3, the rest of S4.6, S4.7, the rest of S4.8) are not here: they enter with the
tokens unit of M3, with their facts.
- **Gate per task:** its tests, `check_release.sh --commit` and the pull request's CI; for the milestone,
  `cargo test --workspace` (outside `PLAN_EXCLUDED`), `check_refusal_tests.py`, `check_function_vectors.py` and the
  palette run green.

## M3 — Units (the per-unit pipeline)

The units, in order: tokens (S4's remaining groups), conditions, readers and placement (the catalogue's section);
window and chrome; overlays; buttons and toggles; fields; indicators; containers; the typed widgets the model lacks
today, in alphabetical batches of about ten entries; the keyed components. Per unit:

1. **Facts** (the lanes, usually done ahead): every key, declaration and state-model combination of the unit on every
   preset (S17), with the traces' rows carried.
2. **Verification** (*How values are checked*), then the spec corrections the unit found.
3. **Model and presets, in one commit:** the unit enabled in the generator (or its hand-written fields), the platform
   functions it adds with their vectors, its readers' new reads (the traces' `reader` lines), and the unit's parts in
   the eight presets (one owner per preset file while they are written).
4. **Connectors:** the unit mapped in the four connectors, one task in the series queue, their contract tests
   extended (the iced, egui and egui-widgets connectors leave `PLAN_EXCLUDED` at their first unit).
- **Gate per unit:** `cargo test -p native-theme` (the gate over the whole model), `check_facts.py`,
  `check_fact_citations.py`, `check_function_vectors.py`, `scripts/check_facts_trace.py --unit <unit>` (each live
  trace row of the unit's targets is in a fact's `rows` or carries a verdict), `generate_widgets.py --check` and the
  pull request's CI green.

## M4 — The facts gate, complete

- Every unit enabled; the legacy appendix of `platform-facts.md` empty (each row converted or recorded as not a model
  value, with its reason); S19.2's staged file removed; the accent run over every pinned scheme and accent; the palette
  run over every bundled palette.
- **Gate:** `cargo test -p native-theme --test facts_gate` green on the eight presets, with no fact lacking a resolved
  key, no state-model combination lacking a fact and every declaration agreeing with its facts;
  `check_facts_trace.py` without `--unit` green; the unmeasured register has no open entry.

## M5 — Showcases (after M4)

The three showcases rebuilt on the query, their element lists and parity gates following (S18); `AnimatedIcon`'s users
moved (S19.3); the connectors built from `default()` with field assignment (R21) checked complete.
- **Gate:** each connector's contract test; the showcase parity gates; the screenshots regenerated and compared side by
  side with the platforms under the same theme; `PLAN_EXCLUDED` empty and `ci.yml`'s conditions gone.

## M6 — Release 0.8.0

- The version bump (the workspace version, `workspace.dependencies`, `native-theme-derive`'s path dependency, the
  READMEs' versions), CHANGELOG, the READMEs, ROADMAP, `docs/todo.md`'s closed entries, `COMPATIBILITY.toml`
  (`update_compatibility.sh`), `docs/new-os-version-guide.md` (facts per generation, the harnesses), the docs.rs build,
  `generate_assets_release.sh` and the provenance stamps, the full `check_release.sh`.
- The v0.7.1 documents, `tools/gap-map/`, this plan and the ledger archived to `docs/archive/`; the scripts as the
  table below says; `check_facts.py` takes its keys from the registry from then on; the memory updated;
  `sweep-target.sh` run.
- **Needs the maintainer:** the merge, the tag, the release and the publish.

## The new scripts

| Script | In `check_release.sh` | At M6 |
|---|---|---|
| `spec_catalogue.py`, `check_spec_names.py`, `check_spec_entries.py`, `check_spec_trace.py` | no (run by hand, S18) | deleted |
| `check_sources.py` (reads the traces and gap maps) | yes | deleted |
| `repin_citations.py` | no (M0) | deleted |
| `import_platform_facts.py` | no (M1a) | deleted at M4 |
| `generate_widgets.py` | `--check` | deleted; the generated code is the source |
| `check_facts_trace.py` (reads the traces) | yes | deleted |
| `check_ledger.py` | yes | deleted |
| `check_facts.py`, `generate_platform_facts.py` (`--check`), `check_refusal_tests.py`, `check_function_vectors.py` | yes | kept |
| `check_fact_citations.py` | the full run fails without the cache; `--commit` reports it skipped by name | kept |
| `update_sources.sh` | `--verify` in the full run | kept |
| `measure_kde_accent.sh` and the harnesses under `tools/` | no (CI) | kept, for the next OS version |

## How values are checked

- **Facts, mechanically** (`check_fact_citations.py`): a `published` or `read` fact's `quote` occurs,
  whitespace-normalised, in the cited lines of the cited file at its pin, and every literal of its `value` (numbers
  after unit normalisation, colours in any notation, a derivation's operands), after its `conversion`, occurs in the
  `quote`; a `measured` fact's `evidence` file is committed, holds the fact's key, combination and value in the
  harness's keyed record, and records toolkit versions equal to the pins; an evidence file no fact cites is reported.
- **Facts, independently:** per unit and platform, a verifier that is not the lane's author receives the keys,
  combinations and the pinned sources, never the lane's citations or values, and reports a value and a citation for
  each; both are compared mechanically. Each disagreement goes to a tie-breaker agent with both citations and the
  source, and the orchestrator reads the verdicts. Agreement rates per lane go to the ledger; a lane below 97 % is
  verified again. For `measured` facts the verifier checks the evidence and the method. A sample is not enough: the
  spec's own trace checks found up to 6.7 % of rows wrong in a pass.
- **Platform functions** (`m3_role`, `kde_effects`, the Kirigami and kirigami-addons functions, `gtk_contrast_pick`,
  `uikit_prominence`, `sf_tracking`, `kcu_overlay`, `kqc_series_color`, S9.4's font scaling, …): each has a vector
  table under `native-theme/tests/vectors/`, generated by the reference implementation at its pin
  (`scripts/generate_qt_color_vectors.py` is the pattern: Qt through PySide6, material-color-utilities through its
  published package, Kirigami's QML functions in a QML engine; generators that need an install run on CI) or, where the
  reference cannot run, transcribed line for line from it; where a platform publishes a table (Apple's tracking), the
  table itself is the facts. `check_function_vectors.py` checks that every function a preset or fact uses has a table
  and a test. Without this, a derivation fact and the preset would both be evaluated by the same function, and a wrong
  function would pass the gate.
- **The presets need no separate author:** the gate compares them with the verified facts mechanically, so who writes
  them does not change what passes.
- **Values that cannot be measured** go to the unmeasured register in the ledger: the key and combination, the
  attempts and their evidence, and the options (another setup such as real hardware or another image, a catalogue
  correction, a release blocker). Each is the maintainer's decision, since R21 allows no partial release.
- **Evidence** is committed under `docs/evidence/<platform>/<date>-run-<id>/` (GitHub's artifacts expire), images
  losslessly compressed and only those a fact cites; where it lives above a size threshold is decided at D6.

## Spec changes

The orchestrator is the one writer of the specification, the catalogue, the traces and the gap maps. Lanes,
verifiers and implementers report what they find wrong into the ledger's queue. Three classes:

1. **Data corrections** (a citation, a line number, a trace row, a catalogue entry's platform detail): applied by the
   orchestrator in batches.
2. **Model changes** (a field, a condition, a function, a rule in S1–S21): an S22 line, and one independent check of the
   batch before it is applied.
3. **A change to a rationale decision** (R1–R25): the maintainer decides.

After each batch the orchestrator runs the spec checkers and `generate_widgets.py --check` and tells the running
tasks; the generator's drift check makes the code follow.

## How the work runs

- **Roles:** an Opus session orchestrates; code tasks run in `implement` agents with a failing test, a compile or a
  fixture as their gate, each reviewed by a fresh reviewer before it closes (G4's precedent); facts lanes run one
  agent per platform and one for the palettes, briefed with the pinned sources, the traces and the facts format;
  verifiers and tie-breakers as above; tooling (the probe passes, the hosts, the harnesses, the vector generators) in
  its own agents owning `tools/` and the workflows. Broad searches go to the `locate` agent (Haiku).
- **Briefs:** every agent reads and searches only the project, the source cache and the scratchpad, and never searches
  the whole filesystem (no `find /`).
- **Failures:** a subagent that fails the same gate twice is not dispatched a third time; the orchestrator takes the
  task over or re-specifies it.
- **Parallel by default:** the lanes, the verifiers, the reviews and the tooling run beside the code; builds and
  branch writes stay sequential, and only the orchestrator commits.
- **Usage:** until D6, at most three agents run at once, within D7's cap; D6 sets the caps per milestone from the
  slice's measured usage.
- **Disk:** `~/.claude/scripts/sweep-target.sh` at each milestone's close and whenever the build folder passes about
  200 GB.
- **Progress** is recorded in the ledger and the project memory.

## Checklist

- [ ] D1, D4, D5, D7 decided
- [ ] M0 setup (spikes K · I · M · W · G · A)
- [ ] M1a slice core
- [ ] M1b slice complete
- [ ] D6 checkpoint
- [ ] M2 model core
- [ ] M3 units (per unit in the ledger)
- [ ] M4 facts gate complete
- [ ] M5 showcases
- [ ] M6 release 0.8.0
