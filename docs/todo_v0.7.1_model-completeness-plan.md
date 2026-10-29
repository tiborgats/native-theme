# v0.7.1 → 0.8.0 — Model completeness: Plan

**Status:** draft 5, 2026-10-29, for the maintainer's approval (revised after four independent reviews).

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
the first push, so that every push runs CI (`ci.yml` runs on pull requests, not on branch pushes). Tooling work runs on
`v0.8.0-tools` (*Where the work happens*). Nothing is released before every milestone below is closed (R21).

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
- **Closing.** Each task ends with `scripts/check_release.sh --commit` green on its last commit, and the pull
  request's CI green when it touches macOS or Windows code (the local run checks and lints both targets but runs
  neither's tests); each milestone ends with the full `scripts/check_release.sh`, with only the expected items the
  ledger lists, its documents updated, a commit and the branch pushed (standing grant for work branches).

## The transition from today's model

- **M1a builds the new model beside today's**, in a public module hidden from the documentation
  (`#[doc(hidden)] pub mod next;`, its presets under `src/next/presets/`; `ThemeParts` beside `ThemeWidget` in
  `native-theme-derive`), one part per task with its tests, so today's crate, connectors and showcases stay green, the
  integration tests (the facts gate is `tests/facts_gate.rs`) reach it, and clippy's `-D warnings` raises no dead
  code. Rejected: a private module (dead-code warnings, and integration tests cannot reach it); a cargo feature `next`
  (every check leg gains and later loses it).
- **The module inventory.** Before the switch, the ledger gives every module of `native-theme` outside today's
  `model/` one fate with its milestone: **ported beside** in M1a (what the slice's API needs: `error.rs` to S1's
  errors, the colour type, the icon loaders and `IconSet` as S2.9 and S16 need them, which `native-theme-build` uses);
  **deleted** at the switch as replaced (`resolve/`, the preset table, `documented_sizes.rs`, `ThemeWidget`,
  `AnimatedIcon`); **unlinked** at the switch (its `mod` line removed, the file kept, linked again by the milestone
  that ports it: the readers, the watchers and `pipeline.rs`'s system entry points at M1b, `spinners.rs` with the
  `spinner` unit). An unlinked module is not compiled, so the reader features compile as empty and the
  `--all-features` and cross-target steps keep passing; only the reader tests leave until M1b.
- **M1a's last task switches:** it moves `next` into place as the public API, deletes and unlinks as the inventory
  says, rewrites the README's examples (they are doctests) and fills `PLAN_EXCLUDED`. No second model is kept after
  M1a. Rejected: deleting today's model first (the crate stays red for most of M1a); one task for all of M1a (too large
  to review or gate); red commits inside M1a (breaks the closing rule).
- **One rule for every model field** (a widget's part, a component, a token group, a colour-role group, a declared
  input): it enters the model in the commit where every preset that exists then states it from verified facts, or
  declares its absence as the facts state; a preset added later states every field the model then holds. So S12 stays
  strict and S19.1's build test and the facts gate run over the whole model at every commit. Generated declarations are
  staged by the generator's list of enabled units (`scripts/generate_widgets.py`); hand-written ones (S3's roles, S4's
  tokens) are written in their unit's commit. The facts gate skips facts whose key the model does not hold yet (the
  lanes run ahead); M4's gate requires every unit enabled and no such fact left.
  - Rejected: declaring unwritten widgets `available = false` (the presets would state that the platform does not draw
    them, which is false); a lenient validation mode (it hides exactly the gaps the gate exists to find); placeholder
    preset values (invented).
- **What cannot build meanwhile is listed, not silently skipped.** One variable, `PLAN_EXCLUDED`, in
  `scripts/check_release.sh` names each excluded step and the milestone that brings it back, and the summary prints it.
  The switch task builds it mechanically: after the switch, every step of the full run that fails is in the list with
  its reason and return milestone, and nothing else is (expected: the connectors' and `tools/showcase-support`'s
  steps, the showcases, widget coverage, connector and showcase parity, the screenshot stamps and configuration
  sections until M2–M5; the reader tests until M1b; the data-document checks, each rewritten or removed in M2). `ci.yml` follows with the same list as matrix entries, steps
  and `--exclude` arguments (the MSRV `--workspace` check). M5's gate is the list empty.
- **Today's files that carry over:** today's `platform-facts.md` rows and evidence become facts (M1a's import; the
  `windows-11-arm` evidence stays, and the Windows Server images' values are dropped where they differ, the two images
  only building afterwards); S19.2's conversion of today's inheritance rules runs at M1a into a staged input file that
  each unit's preset step uses and the gate checks (removed at M4); the ten community preset files go at the switch,
  since the palettes are built from the projects' published colours (M2), not from them.

## Where the work happens

Code runs in series on `v0.8.0-rc1`, one implementer at a time (parallel implementers in one tree see each other's
half-written files; separate build folders cost tens of GB each). Everything else runs elsewhere, so that no parallel
work changes the files a code task builds or its gate reads (`docs/facts`, the presets, the probe):

- **Facts lanes, preset writers, verifiers and tie-breakers** each work in their own git worktree of `v0.8.0-rc1` at
  the last commit and never run `cargo` there, so no build folder is created; they write TOML and run only the Python
  checkers. A verifier's worktree never holds the lane's unverified facts, and its brief limits it to its own worktree,
  the source cache and the scratchpad (no other worktree, no other branch's history); that rests on the brief, which
  is why the audit sample exists. The orchestrator brings verified facts, and a unit's eight preset parts, into
  `v0.8.0-rc1` itself, and runs the gates there.
- **Tooling** (the probe's passes, the hosts, the harnesses, the vector generators, the workflow jobs) works on
  `v0.8.0-tools` in its own worktree, built and run on CI only (`gh workflow run os-probe.yml --ref v0.8.0-tools -f
  jobs=<names>`, and a draft pull request to `v0.8.0-rc1` for CI); the orchestrator merges it between code tasks.
- **The project's guards in worktrees:** M0 checks that the panic and invented-value hooks fire for an agent working
  in a worktree (they are registered by a relative path and `.claude/` is git-ignored); if not, D9 covers making their
  paths absolute.
- Worktrees are removed when their work is merged.

## Decisions

**For the maintainer, at approval:**

- **D1 — the source cache and the release sets.**
  - Location `/run/media/tibi/work/native-theme-sources/` (the work SSD that builds already need mounted, outside
    `/home`'s snapshots), read and written by this work and its agents: it needs a read and write grant. Options
    weighed: a git-ignored folder inside the project (no grant, but every fetch of several GB lands in `/home`'s
    snapshots); the session scratchpad (lost at reboot).
  - The release set per platform, the newest stable at M0 (R1.5), coherent and frozen until 0.8.0 unless the ledger
    records a re-pin: KDE, the Plasma release and the KDE Frameworks, Qt, Kirigami and kirigami-addons that one dated
    Arch Linux Archive snapshot carries (the container D2 measures in; today Plasma 6.8.2's plasma-workspace); GNOME,
    libadwaita, GTK and the gsettings schemas (today 1.10.0 and 4.24.1); Material, material-web's token set (the
    primary reference, R1.6), a Compose release, the Android emulator's system image (API level and revision) and
    material-color-utilities by commit (Compose carries its own colour code, not that library); Windows, the
    microsoft-ui-xaml commit of a Windows App SDK release; Apple, the OS generation (the runners' images
    move within a generation, so each evidence file records its exact build) and the Xcode per generation. A newer patch
    at M0 is fetched instead of today's. Recorded in `docs/facts/sources.toml`, one pin per source and generation.
    Rejected: pinning `master` commits (unreleased code no user runs and no harness can run); re-pinning per fact
    (every lane redoes the mapping and versions mix).
  - The list of sources is generated from the citations in the traces, the gap maps and `platform-facts.md`
    (`scripts/check_sources.py` fails on a cited project with no entry). A web page is pinned as a commit of its public
    source repository where it has one (Microsoft Learn's MicrosoftDocs repositories), else as a dated snapshot with its
    SHA-256 (Apple's pages as their DocC JSON; Material, Fluent, Android, KDE and GNOME pages); Apple's HIG has no
    versions, so its snapshot serves the newest generation only. Apple's SDK headers exist only on macOS runners, so
    their citations are checked by a macOS job of `os-probe.yml` at the pinned Xcode.
  - Approving now lets M0 seed the cache from this session's scratchpad checkouts whose commit equals the pin before a
    reboot clears them; the others are fetched.
- **D4 — `unsafe` and the measurement hosts.**
  - The readers' and watchers' new reads (the traces' `reader` lines on macOS and Windows) grow under their existing
    module exceptions (`macos.rs`, `windows.rs`, `watch/*.rs`), in the same form.
  - The probe's measurements grow under its existing exceptions (`tools/os-probe/src/macos/ffi.rs`, `windows/ffi.rs`).
  - The hosts, none with Rust `unsafe`: iOS, a small Swift app built by Xcode on the runner (rejected: Rust with
    objc2-ui-kit, a third `unsafe` module for a tool); Android, a Kotlin Compose app; KDE, the `qml` runtime with QML
    files (qqc2-desktop-style, Kirigami) and the snapshot's own `pyside6` for Qt Widgets with the Breeze style (pip's
    PySide6 brings its own Qt, which cannot load the snapshot's Breeze plugin); GNOME, a C program modelled on
    libadwaita's `doc/tools/screenshot.c`; WinUI, a small C# Windows App SDK app only if spike W shows values
    microsoft-ui-xaml does not state.
- **D5 — `v0.7.1-rc1` to `main` first** (recommended): the maintainer merges the v0.7.1 documents and scripts (no crate
  change) before `v0.8.0-rc1` is cut, so `main`'s ROADMAP states the next work and the 0.8.0 pull request holds only
  0.8.0's work. The alternative, cutting from `v0.7.1-rc1` and merging both at 0.8.0, also works.
- **D7 — the budget for M0–M1b:** a cap of +15,000k tokens, which D6 revises. The basis is an estimate, not a
  measurement: one independent review of this plan used about 220k tokens; M0–M1b holds six spikes, eight presets'
  slice facts with their verifiers, two implementation milestones with a reviewer per task, and the reviews.
- **D9 — the project's own guards.** `.claude/hooks/no-invented-values.sh` blocks Rust lines with coefficient-like
  literals (`* 2.0`, `0.299 … 0.587 … 0.114`), which a faithful implementation of a platform function contains
  (CAM16, `kde_effects`, Qt's colour helpers); `.claude/agents/preset-validator.md` and the `platform-fact` and
  `add-preset` skills describe today's `platform-facts.md`, inheritance rules and preset format. Recommended: the hook
  exempts the platform functions' module by its path, as it already exempts tests, since `check_function_vectors.py`
  proves that every function there has a vector table
  (rejected: a `// reference:` comment per function, which the hook cannot see in an edit's fragment); the hooks'
  paths made absolute if M0 finds they do not fire in worktrees; the agent and the skills are rewritten at M1a's switch for
  facts and S11's format. The guards are the maintainer's configuration, so this is the maintainer's to approve.
- **D10 — licences.** The crate is MIT OR Apache-2.0 OR 0BSD. The platform functions' references are LGPL
  (`colorsapplicator.cpp` LGPL-2.0-only, `kcolorscheme.cpp` LGPL-2.0-or-later, Qt's `qcolor.cpp`, libadwaita) or
  Apache-2.0 (material-color-utilities); the specification defines several functions only by their reference
  (`adw_standalone`, `m3_role`, `kde_accent`), and today's `gnome/accent.rs` calls itself a port of libadwaita's
  routine. Options: (a) a two-role clean room for the LGPL references: one agent reads the code and writes a
  behavioural description into the catalogue, another implements from the description alone, the vectors prove
  equivalence; (b) a maintained crate that passes the vectors (D8), which moves the question to that crate; (c) a port
  with the reference's notice, which conflicts with offering the crate under MIT or 0BSD alone. Recommended: (b) where
  a crate passes, else (a) for LGPL references; Apache-2.0 references ported with their notice (Apache-2.0 is one of
  the crate's own licences); `gnome/accent.rs` redone by (a); the bundled port files shipped with their licence
  notices; the tools written from what the references' behaviour pins (settings, not code). This states a risk, not a
  legal conclusion; the maintainer decides.

**Scheduled:**

- **D6 — the checkpoint after M1b:** the slice's measured cost per widget and platform (facts written, hours, harness
  rounds, tokens) extrapolated to the whole catalogue; the evidence's size extrapolated, with the choice of where it
  lives above a threshold (in the repository; a separate evidence repository pinned by commit, recommended above about
  500 MB; Git LFS, rejected for its quota and bandwidth costs); the facts gate's run time, with a budget; whether each
  unit's preset rules are generated from its verified facts instead of written (the facts carry S11's `when` and the
  platform's order, so a script could write the rules: less work and no drift, against longer presets and the
  hand-written parts that remain, the `read` inputs and S19.2's bases; decided from how closely the slice's presets
  transcribe their facts); the audit sample's rate, from the slice's measured rate of agreed but wrong facts; the
  spikes' results; the unmeasured register's open entries; the budget caps per milestone. The maintainer approves the
  scale-up.

**Settled by logic (recorded):**

- **D2 — all Linux measurement on CI.** The KDE, GNOME and Android harnesses run on CI's Linux runners in containers
  pinned to the release set, as jobs of `os-probe.yml`; each container is built once from its pinned source and pushed
  to `ghcr.io` (free for public repositories), and the jobs pull it (the Arch Linux Archive is a volunteer service).
  Nothing is installed on this machine, nothing touches the
  desktop, and the versions never drift. Rejected: this machine's Plasma (a rolling distribution moves past the pin
  during the work); local packages or a Flatpak runtime for GNOME (installs, and takes Plasma's portal settings); a
  GNOME OS virtual machine (needed only for what the shell draws, out of scope by R1.1); Paparazzi for Android
  (layoutlib is not the device's renderer).
  - **KDE:** the dated Arch Linux Archive snapshot, under `dbus-run-session`, `QT_QPA_PLATFORM=offscreen`. The accent
    script (`scripts/measure_kde_accent.sh`) uses a fresh config directory per (scheme, accent) pair, since
    `applyScheme()` without an accent takes a stored `AccentColor` (`colorsapplicator.cpp:127-136`), and checks that
    none is stored before a no-accent apply; it applies another scheme first (a fresh config already names Breeze
    Light and the tool then writes nothing; `applyScheme` deletes each colour group before writing it,
    `colorsapplicator.cpp:184`, so nothing of the first remains), then the accent in a second call (the tool takes the
    scheme or `--accent-color`, not both), and checks that kdeglobals changed.
  - **GNOME:** GTK 4 has no offscreen backend, so the harness runs on a headless display (spike G chooses between
    mutter's headless mode and Xvfb), with `GSK_RENDERER` pinned, `GDK_DEBUG=no-portals` (libadwaita's own tests keep
    portal settings out this way), and the colour scheme, contrast, accent and fonts pinned as `screenshot.c` pins
    them; the image is an Arch Linux Archive snapshot carrying the pinned libadwaita and GTK, or both built from their
    tags in the container.
  - **Android:** the emulator with KVM, at the pinned system image.
- **D3 — macOS, iOS and Windows on GitHub's runners.** The repository is public, so hosted runners cost nothing; the
  limits are concurrent macOS jobs and the six-hour job limit. macOS 15 and 26 on `macos-15` and `macos-26`; iOS 18 on
  `macos-15`'s simulator and iOS 26 on `macos-26`'s (checked by spike I); Windows 11 on `windows-11-arm`, which
  `os-probe.yml` already measures (`macos-14` and the two Windows Server images leave its default set once the import
  no longer needs them). Every job is in `os-probe.yml`, dispatched by hand, because `workflow_dispatch` needs the file
  on `main` (`--ref` runs the branch's version); a `jobs` input with an `if:` per job runs only the jobs named, since
  GitHub runs at most five macOS jobs at once. The runners are virtual machines; whether they have a GPU is probed once
  (spike M).
- **D8 — the ports:** a platform function or placement strategy is implemented in the crate (D10), unless a maintained
  Rust crate passes the same vector table at the same pinned version, in which case the crate is used (less code). The
  vectors decide, not a preference.
- **The facts gate reads `docs/facts` at run time:** the facts are a directory of many files, which `include_str!`
  handles badly; the test is excluded from the package, so the packaged crate's tests do not look for files outside
  it.

## The order

```text
M0 setup ──► M1a slice core ──► M1b slice complete ──► D6 ──► M2 core ──► M3 units ──► M4 gate complete ──► M5 showcases ──► M6
  │              ▲   │                ▲                                     │ each unit closed ──► its connector task
  │              │   └─► L facts lanes, verifiers (published from M1a; measured as T delivers) ──► M1b, M3
  └─► spikes (M, G, K first; then I, W, A) ──► T tooling on v0.8.0-tools ──► L
      M's button pass, G's vector generator ──► M1a
```

## M0 — Setup

M0 closes when its setup tasks close; the spikes continue beside M1a.

- **The facts checkers first,** so the lanes can start on M1a's first day: `scripts/check_facts.py` (form: keys from
  the catalogue through `spec_catalogue.py`, `when` as S11's, kind and source well formed, the source pinned, the
  value as S2.2's, the generation rule) and `scripts/check_fact_citations.py` (*How values are checked*), after the S17
  amendment below.

- Cut `v0.8.0-rc1` (D5) and `v0.8.0-tools`; open the draft pull requests; add `PLAN_EXCLUDED` (empty) to
  `check_release.sh`.
- The ledger `docs/todo_v0.7.1_model-completeness-ledger.md`, written by the orchestrator only (the others report to
  it): per unit and platform the status of facts, verification, presets, gate and connectors; the expected items of
  the full check; the spec-change queue; the unmeasured register; the verifiers' agreement and audit rates; the
  evidence size; the tokens per milestone (the sum of the agents' reported usage). This plan's checklist stays at
  milestone level.
- `check_spec_names.py` and `check_spec_entries.py` gain `--self-test` (`check_spec_trace.py` has one); the catalogue
  parser moves into `scripts/spec_catalogue.py`, shared with the generator and the facts checker.
- The source cache (D1): `docs/facts/sources.toml`, `scripts/check_sources.py`, and `scripts/update_sources.sh`, which
  fetches each entry at its pin (androidx and AOSP shallow and sparse) and whose `--verify` checks every checkout's
  commit and every snapshot's hash.
- **Re-pin:** `scripts/repin_citations.py` maps every citation in the traces, the gap maps and `platform-facts.md`
  from the commit it was read at (the gap maps record each SHA, `master` for most KDE sources read 2026-10-27; the
  several Plasma releases `platform-facts.md` cites, v6.4.0 to v6.8.1), fetched again by its SHA, to the pinned
  release, by the line mapping between the two commits, and lists each citation
  whose cited text changed or vanished; the orchestrator re-reads and corrects those. Before any lane starts.
- The specification's amendments this plan needs, through *Spec changes*:
  - S17's fact gains `rows` (the gap-map rows it carries), `quote` (`published` and `read`: the source's exact text at
    the citation), `conversion` (a closed list: points to pixels, a fraction's complement, …, applied by the checker),
    `evidence` (`measured`: the committed evidence file and the path of the value inside it,
    `probe.toml#macos.fonts.sizes.systemFontSize`; for a value read from an image, the committed output of a committed
    analysis script that names its input image and the pixels it reads; the evidence records the OS and toolkit
    builds).
  - Facts stay one directory per platform; a fact for one generation carries `preset:<id>` (S7.1's `Preset`
    condition) in its `when`, and a fact without one cites a source or evidence for each generation its directory
    covers (rejected: one directory per preset, which duplicates every shared fact).
  - A preset's declarations (availability, variants, sizes, container sizes) are stated in `state-models.toml`, and
    the gate checks each preset's declarations against them.
  - The palettes' published colours in `docs/facts/palettes/<project>.toml`, and a palette run in the gate (S14).
  - S5, S6.1 and S18: the declarations are generated from the catalogue by `scripts/generate_widgets.py`, with a drift
    check until the catalogue is archived, after which the generated code is the source (rejected: a proc macro
    reading the catalogue, which lies outside the crate and is archived; hand-writing every entry with no drift
    check).
  - S2.2 and S13: each platform function and each placement strategy has a vector table from its reference; the
    model's own arithmetic (`mul`, `pick`, CSS `color-mix` as `mix` defines it, …) has unit tests from the
    specification's definition.
  - S18: the scripts table below.
- **Triage:** every open item of `docs/todo.md` whose subject is the model, a preset, a reader, the facts, the probe
  or a connector, whatever its tag, and each of the catalogue's *Open items*, gets a destination in the ledger: a
  named milestone, unit or lane, settled by the model, or carried to a later version with its reason.
- **Spikes** on `v0.8.0-tools` (each ends with yes or no and its evidence in the ledger), three at a time, M1a's needs
  first (an agent waiting on a CI run does not count against the cap): M, the macOS probe's button pass (hovered, pressed, disabled and checked in a key window, at each size), then
  its other missing passes (a wallpaper and transparency setting under control, the drawn against the read window
  background) and the runner's GPU; G, the GNOME harness and its vector generator for libadwaita's functions; K, the
  KDE container and accent script; then I, the iOS simulator host; W, whether microsoft-ui-xaml states the slice's
  keys and those of ten sample widgets, and a WinUI host if not; A, the Android host.
- **Gate:** the spec checkers' self-tests and runs green; `check_sources.py` and `update_sources.sh --verify` green;
  `repin_citations.py` reports no unresolved citation; `scripts/check_ledger.py` (every open item counted from
  `docs/todo.md` and the catalogue has a destination) green.

## M1a — The slice's core: `button`, colour roles and the slice's tokens on two presets

- **Presets:** `adwaita` (published) and `macos-tahoe` (measured by spike M's button pass). If spike M fails, the
  button's macOS combinations go to the unmeasured register and `windows-11` (published from microsoft-ui-xaml's
  visual states, spike W brought forward) takes `macos-tahoe`'s place; M1b then closes only after the maintainer has
  decided those entries.
- **Code, beside today's model:** S1; S2's blocks in full with S2.2's grammar, the model's arithmetic and the platform
  functions the two presets' facts use, with their vectors (spike G's generator); S3; S4.1, S4.2, S4.4, S4.5, and
  S4.6's groups the button and the roles use (`token.params.pointer_repeat`, `token.params.color`,
  `token.params.opacity`, …); S5 and S6 for `button`, through `generate_widgets.py` and `ThemeParts`; S7; S8 without
  S13; S9.1, S9.2, S9.4; S10; S11; S12 for what the model holds, with its refusal test (an exhaustive `match` over
  `LayerError` and `Problem`, one triggering case per variant, so a new variant does not compile without one); S16's
  `Theme`, `from_preset`, `set_reader_stand_in`, `set_accent` and the query; S17's gate with its normal and perturbed
  runs and the declarations check. `PresetId` holds the presets written so far.
- **Facts tooling:** `scripts/check_function_vectors.py` (*How values are checked*); `scripts/check_facts_trace.py`
  (`--enabled` checks the enabled units, `--unit` one unit, no option all); `scripts/generate_platform_facts.py` with `--check`;
  `scripts/import_platform_facts.py`, which converts today's `platform-facts.md` rows and their evidence to facts
  wherever the key maps (through the re-pin) and keeps the rest in a legacy appendix of the generated document.
- **The switch** (the last task): as *The transition* says, with the inventory; `native-theme-build` follows the new
  API (its drift test calls `IconSet::from_name`); the preset-validator agent and the two skills rewritten if D9
  approves.
- **Gate:** `cargo test -p native-theme` (the facts gate and the doctests included), `check_facts.py`,
  `check_fact_citations.py`, `check_function_vectors.py`, `generate_platform_facts.py --check` and
  `generate_widgets.py --check` green; after the switch, the full run fails only on `PLAN_EXCLUDED`'s steps.

## M1b — The slice complete

- **Presets:** the other six at the slice's scope, `ios-26` created (its `PresetId`, its file, iOS 26 as its
  generation, its `docs/todo.md` entry closed), all from verified facts.
- **Code:** the platform functions the six presets' facts use, each with its vector table (expected among them
  `kde_accent`, `kde_accent_wm`, `kde_effects`, `kde_selection_inactive`, `m3_role` and Qt's colour helpers; the
  lanes' facts name the full list); S4.8's `token.kde` and `token.material` groups; the four readers and their
  watchers ported to S9.3 and S9.5 at the slice's scope (`ReaderState`, the KDE scheme groups and unset keys) and
  linked again, with S16's system entry points (`Theme::system()`, `from_system`); the reader tests return.
- **The accent run:** `Palette::from_port` and `apply_palette` (S14, as far as the run needs); the KDE schemes and
  accents chosen and pinned (at least one that tints, one with the accent-coloured title bar and a `Header` group, one
  with it and no `Header` group, one lacking a colour group, S17), measured by spike K's script.
- **For D6:** the cost, evidence size and gate run time measured and extrapolated.
- **Gate:** as M1a's, over all eight presets, with the accent run; the pull request's CI green on macOS and Windows.

## M2 — The model's core, complete (after D6; in series)

1. **The four connectors on the slice,** gpui first, then iced, egui and egui-widgets: each one's old mapping
   deleted, its spinner and animated-icon code removed (`AnimatedIcon` is gone; they return with the `spinner` unit's
   connector task), the colour roles and `button` mapped from the query, its contract test (S21.3); their clippy,
   strict panic and test steps leave `PLAN_EXCLUDED` (the showcases stay). They come first so the connectors use the
   API before the rest of S16 is built on it.
2. S2.2's remaining functions: the model's arithmetic with unit tests, the platform functions with vector tables.
3. S12 complete.
4. S14 complete: one palette per variant each of the seven projects publishes (S19.1), as mappings with their labelled
   picks (the guide's own example value where it gives one, else the middle of the stated range; the picks listed in
   the ledger for the maintainer's veto), the KDE ports with their measured facts added to the accent run, the
   projects' published colours as facts (a palette lane and its verifier), the gate's palette run (each pick skipped).
5. S16 complete; S19.3 inside `native-theme`; S21.2 (exhaustive enums,
   `#[non_exhaustive]` structs as S21.2 names them).
6. S18: `property-registry.toml` generated with its drift test; `check_wrong_safety_nets.py` and the
   `inheritance-rules.toml` checks removed; `check_platform_facts_rows.py` and `check_preset_citations.py` rewritten to
   read the facts (S18); `check_todo_citations.py` rewritten to the model's paths; `check_section_refs.py` rewritten to
   the generated document's sections, or removed through an S22 line if nothing cites them any more; their
   `check_release.sh` steps and `ci.yml` entries leave `PLAN_EXCLUDED`.
7. S15 (AppKit rendering), in its own module, checked on CI's macOS legs.

Token groups that add preset fields (S4.3, the rest of S4.6, S4.7, the rest of S4.8) and S13's placement are not
here: they enter with their M3 units, with their facts and vectors.
- **Gate per task:** its tests, `check_release.sh --commit` and the pull request's CI; for the milestone,
  `cargo test --workspace` (outside `PLAN_EXCLUDED`), `check_function_vectors.py` and the palette run green.

## M3 — Units (the per-unit pipeline)

The units, in order: tokens (S4's remaining groups) and conditions; placement (S13: the types, every strategy with
its vector table, generated by the reference where one runs, the GTK and Qt containers, Compose's and
microsoft-ui-xaml's code, and measured by the probe and the iOS host for AppKit and UIKit, anchor, size and window in,
frame out); readers (the catalogue's section); window and chrome; overlays; buttons and toggles; fields; indicators;
containers; the typed widgets the model lacks today, in alphabetical batches of about ten entries; the keyed
components. Per unit:

1. **Facts** (the lanes, usually done ahead): every key, declaration and state-model combination of the unit on every
   preset (S17), with the traces' rows carried.
2. **Verification** (*How values are checked*), then the spec corrections the unit found.
3. **Model and presets, in one commit:** the unit enabled in the generator (or its hand-written fields), the platform
   functions it adds with their vectors, its readers' new reads (the traces' `reader` lines), and the unit's parts in
   the eight presets (written in worktrees, or generated if D6 so decides).
4. **Connectors:** the unit mapped in the four connectors, one task in the series queue, their contract tests
   extended.

A unit with an open entry in the unmeasured register waits for the maintainer's decision on it; the series queue
moves on to the next unit meanwhile.
- **Gate per unit:** `cargo test -p native-theme` (the gate over the whole model), `check_facts.py`,
  `check_fact_citations.py`, `check_function_vectors.py`, `check_facts_trace.py --unit <unit>` (each live trace row of
  the unit's targets is in a fact's `rows` or carries a verdict), `generate_widgets.py --check` and the pull request's
  CI green.

## M4 — The facts gate, complete

- Every unit enabled; the legacy appendix of `platform-facts.md` empty (each row converted or recorded as not a model
  value, with its reason); S19.2's staged file removed; the accent run over every pinned scheme and accent; the palette
  run over every bundled palette; every harness run once more on the then-current runner images and every measured fact
  compared with that run, so a drift within a generation is found before the release.
- **Gate:** `cargo test -p native-theme --test facts_gate` green on the eight presets, with no fact lacking a resolved
  key, no state-model combination lacking a fact and every declaration agreeing with its facts;
  `check_facts_trace.py` without `--unit` green; the unmeasured register has no open entry.

## M5 — Showcases (after M4)

The three showcases rebuilt on the query, their element lists and parity gates following (S18), their busy indicators
drawn from the `spinner` widget (S19.3); the connectors built from `default()` with field assignment (R21) checked complete.
- **Gate:** each connector's contract test; the showcase parity gates; the screenshots regenerated and compared side by
  side with the platforms under the same theme; `PLAN_EXCLUDED` empty and `ci.yml`'s exclusions gone.

## M6 — Release 0.8.0

- The version bump (the workspace version, `workspace.dependencies`, `native-theme-derive`'s path dependency, the
  READMEs' versions), CHANGELOG, the READMEs, ROADMAP, `docs/todo.md`'s closed entries, `docs/COMPATIBILITY.toml`
  (`update_compatibility.sh`), `docs/new-os-version-guide.md` (facts per generation, the harnesses), the docs.rs
  build, `generate_assets_release.sh` and the provenance stamps, the full `check_release.sh`.
- The v0.7.1 documents, `tools/gap-map/`, this plan and the ledger archived to `docs/archive/`; the scripts as the
  table below says; `check_facts.py` takes its keys from the registry from then on; the memory updated;
  `sweep-target.sh` run.
- **Needs the maintainer:** the merge, the tag, the release and the publish.

## The new scripts

| Script | Made in | In `check_release.sh` | At M6 |
|---|---|---|---|
| `spec_catalogue.py`; the three spec checkers | M0 | no (run by hand, S18) | deleted |
| `check_sources.py` (reads the traces and gap maps) | M0 | yes | deleted |
| `repin_citations.py` | M0 | no | deleted |
| `check_ledger.py` | M0 | full run only | deleted |
| `import_platform_facts.py` | M1a | no | deleted at M4 |
| `generate_widgets.py` | M1a | `--check` | deleted; the generated code is the source |
| `check_facts_trace.py` (reads the traces) | M1a | `--enabled` | deleted |
| `check_facts.py` | M0 | yes | kept |
| `check_fact_citations.py` | M0 | the full run fails without the cache; `--commit` reports it skipped by name; the SDK-header citations in a macOS job of `os-probe.yml` | kept |
| `generate_platform_facts.py` (`--check`), `check_function_vectors.py` | M1a | yes | kept |
| `update_sources.sh` | M0 | `--verify` in the full run | kept |
| `measure_kde_accent.sh`, the harnesses and hosts under `tools/` | M0 spikes | no (CI) | kept, for the next OS version |

## How values are checked

- **Facts, mechanically** (`check_fact_citations.py`): a `published` or `read` fact's `quote` occurs,
  whitespace-normalised, in the cited lines of the cited file at its pin, and every literal of its `value` (numbers
  after unit normalisation, colours in any notation, a derivation's operands), after its `conversion`, occurs in the
  `quote`; a `measured` fact's `evidence` file is committed, holds the fact's key, combination and value in the
  harness's keyed record, and records the pinned generation and toolkit versions (the exact builds listed); an
  evidence file no fact cites is reported.
- **Facts, independently:** per unit and platform, a verifier that is not the lane's author receives the keys,
  combinations and the pinned sources, never the lane's citations or values, and reports a value and a citation for
  each; both are compared mechanically. Each disagreement goes to a tie-breaker agent with both citations and the
  source. An audit sample of the agreed facts (during the slice 5 % per lane and unit, at least ten; D6 sets the rate)
  goes to the tie-breaker too, to
  measure how often a lane and its verifier agree on a wrong value. The orchestrator reads the tie-breakers' summaries
  and every verdict that changes the catalogue or the specification. A low agreement or audit rate (below 95 %) starts
  a search for the systematic cause (a wrong pin, a misread format), not a blind re-run. For `measured` facts the
  verifier checks the evidence and the method. A sample alone is not enough: the spec's own trace checks found up to
  6.7 % of rows wrong in a pass.
- **Platform algorithms** (S2.2's platform functions such as `m3_role`, `kde_effects`, `adw_standalone`, the Kirigami
  and kirigami-addons functions, `gtk_contrast_pick`, `uikit_prominence`, `sf_tracking`, `kcu_overlay`,
  `kqc_series_color`, S9.4's font scaling; S13's placement strategies): each has a vector table under
  `native-theme/tests/vectors/`, generated on CI by the reference itself at its pin: Qt through the snapshot's
  PySide6; KColorScheme and KColorUtils (`kde_effects`, `kde_selection_inactive`, `mix`) by a small C++ program built
  in the KDE container against the snapshot's Frameworks; material-color-utilities' own code at its commit;
  libadwaita's and GTK's functions in the GNOME container; Kirigami's QML functions in a QML engine
  (`scripts/generate_qt_color_vectors.py`'s Python transcriptions of `KColorUtils::mix` and Kirigami's
  `linearInterpolation` are replaced, since a transcription checks a port against another port); or measured where the reference
  is closed (AppKit and UIKit placement through the probe and the iOS host); where a platform publishes a table
  (Apple's tracking), the table itself is the facts. The model's own arithmetic has unit tests from the
  specification's definition. `check_function_vectors.py` checks that every function and strategy a preset or fact
  uses has a table and a test. Without this, a derivation fact and the preset would both be evaluated by the same
  function, and a wrong function would pass the gate.
- **The presets need no separate author:** the gate compares them with the verified facts mechanically, so who writes
  them does not change what passes.
- **Values that cannot be measured** go to the unmeasured register in the ledger: the key and combination, the
  attempts and their evidence, and the options (another setup such as real hardware or another image, a catalogue
  correction, a release blocker). Each is the maintainer's decision, since R21 allows no partial release.
- **Evidence** is committed under `docs/evidence/<harness>/<date>-run-<id>/<image>/`, today's layout (GitHub's
  artifacts expire), images losslessly compressed and only those a fact cites; where it lives above a size threshold
  is decided at D6.

## Spec changes

The orchestrator is the one writer of the specification, the catalogue, the traces and the gap maps. Lanes,
verifiers and implementers report what they find wrong to it, and it records each in the ledger's queue. Three
classes:

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
  verifiers and tie-breakers as above; tooling in its own agents on `v0.8.0-tools`. Broad searches go to the `locate`
  agent (Haiku).
- **Briefs:** every agent reads and searches only its worktree, the project, the source cache and the scratchpad, and
  never searches the whole filesystem (no `find /`).
- **Failures:** a subagent that fails the same gate twice is not dispatched a third time; the orchestrator takes the
  task over or re-specifies it.
- **Parallel by default:** the lanes, the verifiers, the reviews and the tooling run beside the code, in their own
  worktrees (*Where the work happens*); builds and commits on `v0.8.0-rc1` stay sequential.
- **Usage:** until D6, at most three agents work at once (an agent waiting on a CI run does not count), within D7's cap; D6 sets the caps per milestone from the
  slice's measured usage.
- **Disk:** `~/.claude/scripts/sweep-target.sh` at each milestone's close and whenever the build folder passes about
  200 GB.
- **Progress** is recorded in the ledger and the project memory.

## Checklist

- [ ] D1, D4, D5, D7, D9, D10 decided
- [ ] M0 setup (spikes M · G · K · I · W · A)
- [ ] M1a slice core
- [ ] M1b slice complete
- [ ] D6 checkpoint
- [ ] M2 model core
- [ ] M3 units (per unit in the ledger)
- [ ] M4 facts gate complete
- [ ] M5 showcases
- [ ] M6 release 0.8.0
