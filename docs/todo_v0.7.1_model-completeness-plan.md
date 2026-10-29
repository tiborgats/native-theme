# v0.7.1 → 0.8.0 — Model completeness: Plan

**Status:** draft 2, 2026-10-29, for the maintainer's approval (draft 1 revised after its first independent review).

**Goal:** the model the specification states, implemented; the eight platform presets on six platforms written from
facts and passing the facts gate; the ten community palettes checked against their projects' published colours; the
four connectors and the three showcases moved to the model; released as 0.8.0 (R21, S21).

**Inputs:** the gap map ([`todo_v0.7.1_model-completeness-gap-map.md`](todo_v0.7.1_model-completeness-gap-map.md)),
the rationale ([`…-rationale.md`](todo_v0.7.1_model-completeness-rationale.md), "R…") and the specification (core
[`…-spec.md`](todo_v0.7.1_model-completeness-spec.md), "S…"; catalogue
[`…-spec-widgets.md`](todo_v0.7.1_model-completeness-spec-widgets.md); the six traces). This plan decides the order of
the work, its gates, what runs in parallel and how the work is checked; it decides no part of the model. Where the
work finds the specification wrong, the specification is corrected first (*Spec changes*, below) and the work follows.

**The presets:** `adwaita` (GNOME), `kde-breeze` (KDE), `macos-monterey` (macOS 15, R1.5) and `macos-tahoe` (macOS
26), `ios-13` (iOS 18, R1.5) and `ios-26`, `windows-11`, `material-3` (Android). `ios-26` does not exist yet (its
`docs/todo.md` entry: 38 required fields had no iOS 26 source); S21.1 makes it a release condition, and this plan
creates it.

**Branch:** `v0.8.0-rc1`, cut from `main` once `v0.7.1-rc1` is merged (D5), with a draft pull request to `main` from
the first push, so that every push runs CI (`ci.yml` runs on pull requests, not on branch pushes). Nothing is released
before every milestone below is closed (R21).

## Global constraints

- **No invented value.** A value enters a fact only from a source pinned by commit or version (kind `published`), from
  an API a reader calls (kind `read`, its definition cited), or from a measurement recorded with its method, input and
  output (kind `measured`, with its tolerance) (S17). Every fact is checked for truth twice: mechanically (its quote is
  at its citation, its value in its evidence) and by an independent verifier (*How facts are checked*).
- **No runtime panics.** No `unsafe` except S15's module (approved 2026-10-29) and what D4 approves; every exception is
  module-scoped and every `unsafe` block carries its safety argument, as `sficons.rs` does.
- **Gates are mechanical.** Every task names the command whose result closes it. A visual comparison is kept where the
  project's rules ask for one, recorded with its screenshots, beside the mechanical gate, never instead of it.
- **One writer per file at a time.** Parallel work owns disjoint files; the specification, the catalogue, the traces
  and the ledger have one writer, the orchestrating session.
- **Sources stay pinned** in one cache (D1), so a citation can be re-read by a script and by a verifier.
- **Captures announce themselves** when they take the desktop's focus, and are run and compared by the agent.
- **Closing.** Each task ends with `scripts/check_release.sh --commit` green on its last commit; each milestone ends
  with the full `scripts/check_release.sh`, with only the expected items the ledger lists, its documents updated, a
  commit and the branch pushed (standing grant for work branches).

## The transition from today's model

The new model **replaces** today's in place, at M1a; no second model is kept beside it (keeping one means two derives
and two preset formats maintained for no gain, and the old one is deleted at the end anyway).

- **Widgets enter the model one section at a time** (M3), together with that section's verified facts and the eight
  presets' parts for it. The model therefore never holds a widget a preset lacks, so S12 stays strict, S19.1's build
  test and the facts gate run over the whole model at every commit, and no gate needs a scope filter. The only staging
  device is the generator's list of enabled sections (`scripts/generate_widgets.py`, F-5 below); M4's gate requires it
  to hold every section.
  - Rejected: declaring unwritten widgets `available = false` (the presets would state that the platform does not draw
    them, which is false); a lenient validation mode (it hides exactly the gaps the gate exists to find).
- **What cannot build meanwhile is listed, not silently skipped.** The connectors, the showcases, the examples and the
  steps that check them (clippy, the strict panic check, widget coverage, connector and showcase parity, the screenshot
  stamps) and the data-document checks on `inheritance-rules.toml` and `platform-facts.md` sit in one variable,
  `PLAN_EXCLUDED`, in `scripts/check_release.sh`, printed in its summary; `ci.yml`'s matching jobs carry a condition
  naming the same plan. Entries leave the list as their milestone moves them (the readers at M1b, the examples at M2,
  the data-document checks at M2, the connectors and showcases at M5). M5's gate is the list empty and the conditions
  gone.

## Decisions the maintainer takes (at approval)

- **D1 — the source cache.** Location `/run/media/tibi/work/native-theme-sources/` (the work SSD that builds already
  need mounted, outside `/home`'s snapshots), read and written by this work and its agents: it needs a read and write
  grant. Options weighed: a git-ignored folder inside the project (no grant, but every fetch of several GB lands in
  `/home`'s snapshots); the session scratchpad (lost at reboot). Contents, each pinned in `docs/facts/sources.toml`
  (name, URL, commit or version, licence): libadwaita, GTK, GtkSourceView, libspelling, libpanel; Breeze, Kirigami,
  qqc2-desktop-style, kirigami-addons, KColorScheme, plasma-workspace, KWin, qtbase, KTextEditor, KSyntaxHighlighting,
  KQuickCharts; microsoft-ui-xaml, the Windows Community Toolkit; material-web's tokens, Compose Material 3, compose-ui
  (`FontScaling`), AOSP `frameworks/base` (`FontScaleConverter`, S9.4), material-color-utilities (androidx and AOSP
  fetched shallow and sparse); the community projects (Catppuccin and its KDE port, Dracula and its KDE port, Nord,
  Gruvbox, One Dark, Solarized, Tokyo Night, and the rest of the ten); Apple's HIG and Microsoft Learn pages as
  snapshots (HTML, retrieval date, SHA-256); Apple SDK headers by Xcode version (local only: their licence forbids
  committing them). Approving now lets M0 seed the cache from this session's scratchpad checkouts (Breeze,
  plasma-workspace, KWin, kcolorscheme, Kirigami, libadwaita 1.10.0, GTK 4.24.1, qtbase) before a reboot clears them.
- **D2 — measurement on Linux.**
  - **KDE, on this machine, isolated:** `scripts/measure_kde_accent.sh` runs Plasma's installed
    `plasma-apply-colorscheme` under `dbus-run-session` with `QT_QPA_PLATFORM=offscreen` and temporary
    `XDG_CONFIG_HOME`, `XDG_CONFIG_DIRS` and `XDG_DATA_HOME` in the scratchpad (holding the pinned scheme files). The
    private session bus receives the KWin blend call and the palette broadcast, so the desktop sees neither; the script
    applies the scheme (another scheme first, since a fresh config already names Breeze Light and the tool then writes
    nothing), then the accent in a second call (the tool takes the scheme or `--accent-color`, not both), checks that
    kdeglobals changed, and records the installed plasma-workspace version, refusing when it differs from the pinned
    one; a container on a CI Linux runner at the pinned version is the fallback. Breeze's widget geometry, where its
    sources are silent, is rendered the same way, offscreen. Nothing is installed; approval is needed because it runs
    the system's Plasma tools.
  - **GNOME and Android, on CI (recommended):** a GNOME harness modelled on libadwaita 1.10.0's
    `doc/tools/screenshot.c` (offscreen; `ADW_DEBUG_COLOR_SCHEME`, `ADW_DEBUG_HIGH_CONTRAST`, `ADW_DEBUG_ACCENT_COLOR`
    and the font settings pinned, so no portal settings leak in) in a container with libadwaita 1.10 and GTK 4.24; a
    Compose host app on the Android emulator on CI's Linux runners. Both are mostly `published` platforms, so
    measurement is the exception and CI's round-trip time costs little. Options weighed: local packages or a local
    Flatpak runtime (faster rounds, but installs on this machine and, without pinning, takes Plasma's portal settings);
    a GNOME OS virtual machine (needed only for what the shell draws, out of scope by R1.1); JVM rendering for Android
    (Paparazzi, no emulator; rejected: layoutlib is not the device's renderer, so text and animation differ).
- **D3 — macOS, iOS and Windows on GitHub's runners** (settled by logic, recorded here): the repository is public, so
  hosted runners cost nothing; the limits are concurrent macOS jobs and the six-hour job limit. macOS 15 and 26 on
  `macos-15` and `macos-26`; iOS 18 and 26 on their simulators (that the images carry both runtimes is checked by
  spike I); Windows 11 on `windows-11-arm` (Windows 11's look; `windows-2022` and `windows-2025` are Windows Server
  and only build). Runs are dispatched by hand from `os-probe.yml`, extended rather than joined by new workflow files,
  because `workflow_dispatch` needs the file on `main` (`gh workflow run os-probe.yml --ref v0.8.0-rc1` runs the
  branch's version). The runners are virtual machines; whether they have a GPU is probed once (spike M), and every
  measured fact records its runner image and tolerance.
- **D4 — the measurement hosts and their `unsafe`.**
  - The probe's measurements grow under its existing module exceptions, `tools/os-probe/src/macos/ffi.rs` and
    `windows/ffi.rs` (`#![allow(unsafe_code)]`), in the same form: approval asked for extending them.
  - The iOS host: a small Swift app built by Xcode on the runner (recommended; no Rust `unsafe`, UIKit's own
    language); rejected: Rust with objc2-ui-kit, a third `unsafe` module for a tool.
  - The WinUI host: none unless spike W shows values microsoft-ui-xaml's XAML and resources do not state; then a small
    C# Windows App SDK app (no Rust `unsafe`).
  - The Android host: a Kotlin Compose app (D2).
- **D5 — `v0.7.1-rc1` to `main` first** (recommended): the maintainer merges the v0.7.1 documents and scripts (no crate
  change) before `v0.8.0-rc1` is cut, so `main`'s ROADMAP states the next work and the 0.8.0 pull request holds only
  0.8.0's work. The alternative, cutting from `v0.7.1-rc1` and merging both at 0.8.0, also works.
- **D6 — the checkpoint after M1b** (scheduled, not decided now): the slice's measured cost and its extrapolation,
  the spikes' results, the open entries of the unmeasured register and the usage per milestone; the maintainer
  approves the scale-up and its budget caps.

## The order

```text
M0 setup ──► M1a slice core ──► M1b slice complete ──► D6 ──► M2 core ──► M3 sections ──► M4 gate complete ──► M6
  │              │                                              │                                              ▲
  │              └─► L facts lanes (published from M1a, measured as T delivers) ──► feed M1b, M3 ────────────┤
  └─► spikes ──► T tooling (probe passes, hosts, harnesses) ──► feed L                                        │
                                                                └──► M5 connectors (from M2's API; closes after M4)
```

Code runs in series, one implementer on the tree at a time (parallel implementers in one tree see each other's
half-written files; separate worktrees each cost a whole build folder of tens of GB). The parallelism is in the facts
lanes, the verifiers, the reviews and the tooling, which touch no code the model builds.

## M0 — Setup

- Cut `v0.8.0-rc1` (D5); open the draft pull request; add `PLAN_EXCLUDED` (empty) to `check_release.sh`.
- The ledger `docs/todo_v0.7.1_model-completeness-ledger.md`, owned by the orchestrator: per section and platform
  the status of facts, verification, presets and gate; the expected items of the full check; the spec-change queue;
  the unmeasured register; the usage per milestone. This plan's checklist stays at milestone level.
- The spec checkers (`check_spec_names.py`, `check_spec_entries.py`, `check_spec_trace.py`) gain `--self-test`; the
  catalogue parser moves into `scripts/spec_catalogue.py`, shared with the generator and the facts checker.
- The source cache (D1): `scripts/update_sources.sh` fetches each entry of `docs/facts/sources.toml` at its pin and
  `--verify` checks every checkout's commit and every snapshot's hash; seeded from the scratchpad's checkouts.
- The specification's amendments this plan needs, through *Spec changes*: S17's fact gains `rows` (the gap-map rows it
  carries), `quote` (`published` and `read`: the source's exact text at the citation) and `evidence` (`measured`: the
  committed evidence file); facts stay one directory per platform, a fact for one generation carrying
  `preset:<id>` (S7.1's `Preset` condition) in its `when` (rejected: one directory per preset, which duplicates every
  shared fact); palettes' published colours in `docs/facts/palettes/<project>.toml` and a palette run in the gate
  (S14); the gate reads `docs/facts` at run time, not through `include_str!` (files outside the crate would break
  `cargo package --verify`); S5, S6.1 and S18: the widget declarations are generated from the catalogue by
  `scripts/generate_widgets.py`, with a drift check until the catalogue is archived, after which the generated code
  is the source (rejected: a proc macro reading the catalogue, which lies outside the crate and is archived; hand
  writing 114 entries with no drift check); S18 lists the new scripts.
- **Triage:** each of `docs/todo.md`'s 43 open `[v0.7.x]` items and each of the catalogue's *Open items* gets a
  destination in the ledger: a named milestone or lane, settled by the model, or carried to `[0.8.x]` with its reason.
- **Spikes start** (background; each ends with yes or no and its evidence in the ledger): K, the KDE accent harness
  (D2); G, the GNOME harness (D2); A, the Android host on the CI emulator; I, the iOS simulator host on `macos-15` and
  `macos-26`, with both runtimes present; W, whether microsoft-ui-xaml states what `windows-11` needs, and a WinUI host
  on `windows-11-arm` if not; M, the macOS probe's missing passes (hovered controls, disabled and checked rendered in
  a key window, a wallpaper and transparency setting under control, the drawn against the read window background)
  and the runner's GPU.
- **Gate:** the three spec checkers' self-tests and runs green; `update_sources.sh --verify` green; the triage table
  holds every item (`scripts/check_ledger.py` counts them against `docs/todo.md` and the catalogue).

## M1a — The slice's core: `button`, colour roles and the slice's tokens on `adwaita` and `macos-tahoe`

- **Code** (today's model, derive and preset format removed with their tests; `PLAN_EXCLUDED` gains the connectors,
  showcases, examples, the readers' features and the data-document checks): S1; S2's blocks in full with S2.2's
  grammar and the functions the slice's facts use; S3; S4.1, S4.2, S4.4, S4.5; S5 and S6 for `button`, through the
  generator; S7; S8 without S13; S9.1, S9.2, S9.4; S10; S11; S12 for what the model holds; S16's `Theme`,
  `from_preset`, `set_reader_stand_in`, `set_accent` and the query; S17's gate with its normal and perturbed runs.
  `PresetId` holds the presets written so far (two here, eight from M1b), so the gate never meets an unwritten one.
- **Facts tooling:** `scripts/check_facts.py` (form: keys from the catalogue through `spec_catalogue.py`, `when` as
  S11's, kind and source well formed, the source pinned in `sources.toml`, the value as S2.2's);
  `scripts/check_fact_citations.py` (truth, *How facts are checked*); `scripts/generate_platform_facts.py` with
  `--check`; `scripts/import_platform_facts.py`, which converts today's `platform-facts.md` rows and their evidence to
  facts wherever the key maps and keeps the rest in a legacy appendix of the generated document (M4 empties it).
- **Facts and presets:** `adwaita` (published) and `macos-tahoe` (measured with today's probe, extended for the
  button's states and sizes) for the slice's keys, verified; both presets in S11's format.
- **Gate:** `cargo test -p native-theme` (the facts gate included) green; `check_facts.py`,
  `check_fact_citations.py` and `generate_platform_facts.py --check` green; `generate_widgets.py --check` green.

## M1b — The slice complete

- **Presets:** the other six at the slice's scope, `ios-26` created (its `PresetId`, its file, iOS 26 as its
  generation, its `docs/todo.md` entry closed), all from verified facts.
- **Readers:** the four readers ported to S9.3 and S9.5 at the slice's scope (`ReaderState`, the KDE scheme groups
  and unset keys); their features leave `PLAN_EXCLUDED`.
- **The accent run:** `Palette::from_port` and `apply_palette` (S14, as far as the run needs); the KDE schemes and
  accents chosen and pinned (at least one that tints, one with the accent-coloured title bar and a `Header` group, one
  with it and no `Header` group, one lacking a colour group, S17), measured by spike K's script.
- **A connector on the slice:** the gpui connector's colour roles and its button drawn from the query, compared side by
  side with the platform's own button under the same theme on `kde-breeze` and `adwaita`, the screenshots recorded
  under `docs/evidence/slice/` (proves the query is usable by a connector before M2 fixes the API).
- **Cost:** per widget and platform, the facts written, the hours, the probe rounds and the usage, extrapolated to the
  114 entries × 8 presets (D6).
- **Gate:** as M1a's, over all eight presets, with the accent run.

## M2 — The model's core, complete (after D6; in series)

1. S2.2's remaining functions; S4.3, S4.6–S4.8.
2. S13 (placement).
3. S12 complete; `scripts/check_refusal_tests.py` checks that every `LayerError` and `Problem` variant has a test
   that triggers it.
4. S14 complete: the ten palettes as mappings with their labelled picks (listed in the ledger for the maintainer's
   veto), the KDE ports, their published colours as facts, the gate's palette run (each pick skipped).
5. S16 complete and S19 (S19.2's conversion of today's inheritance rules before `inheritance-rules.toml` goes; S19.3
   inside `native-theme`; the examples move and leave `PLAN_EXCLUDED`); S21.2 (exhaustive enums, `#[non_exhaustive]`
   structs as S21.2 names them).
6. S18: `property-registry.toml` generated with its drift test; `check_wrong_safety_nets.py` removed;
   `check_platform_facts_rows.py` and `check_preset_citations.py` reading the facts; their `check_release.sh` steps
   and `ci.yml` jobs changed; the data-document checks leave `PLAN_EXCLUDED`.
7. S15 (AppKit rendering), owned by one implementer in its own module and checked on CI's macOS legs; it may run beside
   1–6, since it builds only on macOS.

- **Gate per task:** its tests and `check_release.sh --commit`; for the milestone, `cargo test --workspace` (outside
  `PLAN_EXCLUDED`), `check_refusal_tests.py` and the palette run green.

## M3 — Sections (the per-section pipeline)

The units, in order: the catalogue's tokens, conditions, readers and placement section (its facts and reads; its code
is M1's and M2's); window and chrome; overlays; buttons and toggles; fields; indicators; containers; the typed widgets
the model lacks today, in alphabetical batches of about ten entries; the keyed components. Per unit:

1. **Facts** (the lanes, usually done ahead): every key and state-model combination of the unit on every preset
   (S17), with the traces' rows carried.
2. **Verification** (*How facts are checked*), then the spec corrections the unit found.
3. **Model:** the unit enabled in the generator; its readers' new reads (the traces' `reader:` lines).
4. **Presets:** the unit's parts in the eight presets, one owner per preset file.
- **Gate per unit:** `cargo test -p native-theme` (the gate over the whole model), `check_facts.py`,
  `check_fact_citations.py`, `scripts/check_facts_trace.py --unit <unit>` (each live trace row of the unit's targets
  is in a fact's `rows` or carries a verdict) and `generate_widgets.py --check` green.

## M4 — The facts gate, complete

- Every section enabled; the legacy appendix of `platform-facts.md` empty (each row converted or recorded as not a
  model value, with its reason); the accent run over every pinned scheme and accent; the palette run over the ten
  palettes.
- **Gate:** `cargo test -p native-theme --test facts_gate` green on the eight presets with no fact lacking a resolved
  key and no state-model combination lacking a fact; `check_facts_trace.py` without `--unit` green; the unmeasured
  register has no open entry.

## M5 — Connectors and showcases (from M2's API; closes after M4)

The gpui, iced, egui and egui-widgets connectors map the resolved model, each with its contract table and contract
test (S21.3) and built from `default()` with field assignment (R21); `AnimatedIcon`'s users moved (S19.3); the three
showcases rebuilt on the query, their element lists and parity gates following (S18).
- **Gate:** each connector's contract test; the showcase parity gates; the screenshots regenerated and compared side by
  side with the platforms under the same theme; `PLAN_EXCLUDED` empty and `ci.yml`'s conditions gone.

## M6 — Release 0.8.0

- CHANGELOG, the READMEs, ROADMAP, `docs/todo.md`'s closed entries, `COMPATIBILITY.toml` (`update_compatibility.sh`),
  `docs/new-os-version-guide.md` (facts per generation), the docs.rs build, `generate_assets_release.sh` and the
  provenance stamps, the full `check_release.sh`.
- The v0.7.1 documents, this plan and the ledger archived to `docs/archive/`; the spec checkers deleted (S18);
  `check_facts.py` takes its keys from the registry from then on; the memory updated; `sweep-target.sh` run.
- **Needs the maintainer:** the merge, the tag, the release and the publish.

## How facts are checked

- **Mechanically** (`scripts/check_fact_citations.py`, run in `check_release.sh` with the cache present): a
  `published` or `read` fact's `quote` occurs, whitespace-normalised, in the cited lines of the cited file at its pin;
  a `measured` fact's `evidence` file is committed and holds the fact's key, combination and value in the probe's
  keyed record; an evidence file no fact cites is reported.
- **Independently:** per unit and platform, a verifier that is not the lane's author receives the unit's keys and
  combinations and the cited sources, not the values, derives each value itself and compares with the lane's facts;
  each disagreement is settled by the orchestrator reading the source, and recorded. For `measured` facts the verifier
  reads the evidence and the method. A sample is not enough: the spec's own trace checks found 0.4 to 6.7 % of rows
  wrong per pass.
- **The presets need no separate author:** the gate compares them with the verified facts mechanically, so who writes
  them does not change what passes.
- **Values that cannot be measured** go to the unmeasured register in the ledger: the key and combination, the
  attempts and their evidence, and the options (another setup such as real hardware or another image, a catalogue
  correction, a release blocker). Each is the maintainer's decision, since R21 allows no partial release.
- **Evidence** is committed under `docs/evidence/<platform>/<date>-run-<id>/` (GitHub's artifacts expire), images
  losslessly compressed and only those a fact cites; each round reports its size in the ledger.

## Spec changes

The orchestrator is the one writer of the specification, the catalogue and the traces. Lanes, verifiers and
implementers report what they find wrong into the ledger's queue; the orchestrator applies the queue in batches, each
change with its S22 line, then runs the four spec checkers and `generate_widgets.py --check`, and tells the running
tasks; the generator's drift check makes the code follow.

## How the work runs

- **Roles:** an Opus session orchestrates; code tasks run in `implement` agents with a failing test, a compile or a
  fixture as their gate, each reviewed by a fresh reviewer before it closes (G4's precedent); facts lanes run one
  agent per platform, briefed with its sources, its traces and the facts format; verifiers as above; tooling (the
  probe passes, the hosts, the harnesses) in its own agents owning `tools/` and the workflows. Broad searches go to
  the `locate` agent (Haiku).
- **Briefs:** every agent reads and searches only the project, the source cache and the scratchpad, and never searches
  the whole filesystem (no `find /`).
- **Failures:** a subagent that fails the same gate twice is not dispatched a third time; the orchestrator takes the
  task over or re-specifies it.
- **Parallel by default:** the lanes, the verifiers, the reviews and the tooling run beside the code; builds and
  branch writes stay sequential, and only the orchestrator commits.
- **Usage:** the maintainer's subscription has a weekly limit. Until D6, at most three agents run at once; D6 sets the
  caps per milestone from the slice's measured usage.
- **Disk:** `~/.claude/scripts/sweep-target.sh` at each milestone's close and whenever the build folder passes about
  200 GB.
- **Progress** is recorded in the ledger and the project memory.

## Checklist

- [ ] D1–D5 decided
- [ ] M0 setup (spikes K · G · A · I · W · M)
- [ ] M1a slice core
- [ ] M1b slice complete
- [ ] D6 checkpoint
- [ ] M2 model core
- [ ] M3 sections (units in the ledger)
- [ ] M4 facts gate complete
- [ ] M5 connectors and showcases
- [ ] M6 release 0.8.0
