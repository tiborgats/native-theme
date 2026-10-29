# v0.7.1 → 0.8.0 — Model completeness: Plan

**Status:** approved 2026-10-29 with the maintainer's decisions D1, D4, D5, D7, D9, D10, D11 and D12 (each marked
below); draft 12. Eleven independent reviews; draft 11 consolidated
their fixes into one text, and draft 12 applies the eleventh review's.

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

**Branches:** `v0.8.0-rc1` for the work, cut from `main` after `v0.7.1-rc1` is merged, or from `v0.7.1-rc1` (D5);
`v0.8.0-tools` for the tooling (*Where the work happens*). Each gets a draft pull request after its first commit, so
every push runs CI (`ci.yml` runs on pull requests, not on branch pushes). Nothing is released before every milestone
is closed (R21; 0.7.x patches, D11).

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
- **Closing.** A code task ends with `scripts/check_release.sh --commit` green on its last commit, and the pull
  request's CI green when it touches macOS or Windows code (the local run checks and lints both targets but runs
  neither's tests). A tooling task or spike, which runs no `cargo` in its worktree, closes on its CI run, and
  `--commit` runs on its merge. A milestone ends with the full `scripts/check_release.sh` green, its documents and its
  part of the CHANGELOG's 0.8.0 entry written, a commit and the branch pushed (standing grant for work branches).

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
  - `kde/` keeps `kdeglobals_path`, `read_kcmfonts_key` and `kdeglobals_icon_theme`; `gnome/` keeps
    `detect_portal_backend`; `windows.rs` its DPI constant;
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
  M1b. Integration tests of unlinked code (`tests/reader_kde.rs`) are deleted and restored with it.
- **Under D10's decision** the existing ports and transcriptions (`gnome/accent.rs`, `kde/colors.rs`'s
  ports, `scripts/generate_qt_color_vectors.py`'s transcriptions and their output `kde/qt_color_vectors.rs`) are
  deleted at the switch and not restored; they are redone by the clean room.

An unlinked module is not compiled, so with the splits the reader features compile and the `--all-features`,
cross-target and per-feature steps keep checking the kept code. Code restored from the switch's parent commit has its
`platform-facts.md` citations rewritten as it is restored (below). The inventory also lists the connectors' showcase
entries unlinked at M2.1.

### One rule for every model field

A widget's part, a component, a token group, a colour-role group or a declared input enters the model in the commit
where every preset that exists then states it from verified facts, or declares its absence as the facts state; a
preset added later states every field the model then holds. So S12 stays strict, and S19.1's build test and the facts
gate run over the whole model at every commit. Generated declarations are staged by the generator's list of enabled
units (`scripts/generate_widgets.py`); hand-written ones (S3's roles, S4's tokens) are written in their unit's commit.
The facts gate skips facts whose key the model does not hold yet (the lanes run ahead); M4 requires every unit
enabled and no such fact left.

Rejected: declaring unwritten widgets `available = false` (the presets would state that the platform does not draw
them, which is false); a lenient validation mode (it hides the gaps the gate exists to find); placeholder preset values
(invented).

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

### What carries over

- **Today's facts.** M1a imports today's `platform-facts.md` rows into `docs/facts/imported/`, keyed where a gap-map
  row and its trace line name the model path; the rows reference their evidence where it lies. It is lane input that
  neither the gate nor the citation check reads: the lanes turn each row into a verified fact like any other, or record
  it as not a model value, so nothing unverified reaches the gate. The `windows-11-arm` evidence stays usable; the
  Windows Server images' values are dropped where they differ. The folder is empty at M4.
- **Today's evidence.** The five runs under `docs/evidence/os-probe/` (about 6,400 files, of which today's rows cite a
  few hundred) are listed by run folder in `check_fact_citations.py` as legacy evidence: their files count as cited
  until M4, when every legacy file no fact cites is deleted (git history keeps it) and the list is emptied. Rejected:
  deleting the uncited files now (a lane converting a row may need another render of the same run); a warning instead
  of a failure (nothing would catch an uncited file after M4).
- **`platform-facts.md`** becomes generated at the switch, from the verified facts only; today's presets and the
  `platform_facts_xref.rs` test go then.
- **Citations of today's `platform-facts.md` by line.** They are rewritten to `platform-facts.md@<sha>:LINE`, `<sha>`
  being the commit `v0.8.0-rc1` is cut from: in M0 for the gap maps and `docs/todo.md`; at the switch for every other
  tracked file (the CHANGELOG included: an `@<sha>` makes a released entry's citation exact without changing its
  meaning), except `docs/archive/`, `docs/evidence/` and the checker scripts that define the format. From the switch, a
  `check_release.sh` step fails on a `platform-facts.md:N` citation without `@` outside those exceptions. The probe's
  strings that cite the document become fact keys in the next spike that changes the probe.
- **Today's inheritance rules** are converted by S19.2's script at M1a into a staged input file that each unit's preset
  step uses and the gate checks; it is removed at M4.

## Where the work happens

Code runs in series on `v0.8.0-rc1`, one implementer at a time (parallel implementers in one tree see each other's
half-written files; separate build folders cost tens of GB each). Everything else runs where it cannot change what a
code task builds or its gate reads (`docs/facts`, the presets, the probe):

- **Facts lanes, preset writers and tie-breakers** each work in their own git worktree of `v0.8.0-rc1` and never run
  `cargo` there, so no build folder is created; they write TOML and run only the Python checkers.
- **Verifiers** work in a scratchpad folder holding a generated brief (the unit's keys, the pins, the catalogue's
  state-model source lines and, for `measured` facts, copies of the cited evidence and the method's script), and
  write their output there. They read only that folder and the source cache, not the project, whose gap maps and
  `platform-facts.md` state values they could copy. That rests on the brief, which is why the quote check and the
  audit sample exist.
- **The orchestrator** compares, brings verified facts and a unit's eight preset parts into `v0.8.0-rc1`, and runs the
  gates there.
- **Tooling** (the probe's passes, the hosts, the harnesses, the vector generators, the workflow jobs) works on
  `v0.8.0-tools` in its own worktree, built and run on CI only (`gh workflow run os-probe.yml --ref v0.8.0-tools -f
  jobs=<names> -f images=<list>`; its draft pull request targets `v0.8.0-rc1`). `v0.8.0-tools` merges `v0.8.0-rc1`
  first; the orchestrator merges it back between code tasks.
- **The project's guards:** M0 checks that the panic and invented-value hooks fire for an agent working in a worktree
  (they are registered by a relative path and `.claude/` is git-ignored); if not, D9 covers making their paths
  absolute.
- Worktrees are removed when their work is merged.

## Decisions

### For the maintainer, at approval

- **D1 — the source cache and the release sets.**
  - **Location (decided 2026-10-29):** `/run/media/tibi/work/cargo-build/native-theme-sources/`, a folder beside the
    workspaces' build folders on the work SSD: inside the maintainer's standing read and write grant for
    `cargo-build`, outside `/home`'s snapshots, and outside every project's own build folder, which is all
    `sweep-target.sh` cleans. `update_sources.sh` can rebuild it from `sources.toml` at any time. Rejected: a git-ignored
    folder inside the project (several GB in `/home`'s snapshots); the session scratchpad (lost at reboot). M0 seeds it
    from this session's scratchpad checkouts whose commit equals the pin.
  - **The release set** per platform: the newest stable at M0 (R1.5), coherent, frozen until 0.8.0 unless the ledger
    records a re-pin. KDE: the Plasma release, KDE Frameworks, Qt, Kirigami and kirigami-addons one dated Arch Linux
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
    DocC JSON; Material, Fluent, Android, KDE and GNOME pages); Apple's HIG has no versions, so its snapshot serves the
    newest generation only. Apple's SDK headers exist only on macOS runners, so their citations are checked by a macOS
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
- **D7 — the budget for M0–M1b** (approved by the maintainer, 2026-10-29): a cap of +15,000k tokens, which D6 revises. An estimate, not a measurement: one
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
- **D11 — 0.7.x releases during the work** (approved by the maintainer, 2026-10-29): no planned 0.7.x release; `docs/todo.md`'s `[v0.7.x]` items
  are triaged into this plan or carried to after 0.8.0, and ROADMAP's *Also in the 0.7.x releases* is updated at M0.
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

### Scheduled

- **D6 — the checkpoint after M1b.** The maintainer approves the scale-up from:
  - the slice's measured cost per widget and platform (facts written, hours, harness rounds, tokens), extrapolated to
    the whole catalogue, and the budget caps per milestone;
  - the evidence's size, extrapolated, and where it lives above a threshold (in the repository; a separate evidence
    repository pinned by commit, recommended above about 500 MB; Git LFS, rejected for its quota and bandwidth costs);
  - the facts gate's run time, the build's time and the docs.rs build, with a budget;
  - whether each unit's preset rules are generated from its verified facts instead of written (the facts carry S11's
    `when` and the platform's order: less work and no drift, against longer presets and the hand-written parts that
    remain, the `read` inputs and S19.2's bases; decided by how closely the slice's presets transcribe their facts);
  - the audit sample's rate, from the slice's measured rate of agreed but wrong facts;
  - the spikes' results and the unmeasured register's open entries.

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
  fails above crates.io's 10 MB limit.

## The order

```text
M0 setup ──► M1a slice core ──► M1b slice complete ──► D6 ──► M2 core ──► M3 units ──► M4 gate complete ──► M5 showcases ──► M6
  │              ▲   │                ▲                                     │ each unit closed ──► its connector task
  │              │   └─► L facts lanes, verifiers (published from M1a; measured as T delivers) ──► M1b, M3
  └─► spikes (M, G, K first; then I, W, A) ──► T tooling on v0.8.0-tools ──► L
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
  functions the two presets' facts use, with their vectors (spike G's generator); S3; S4.1, S4.2, S4.4, S4.5, and
  S4.6's groups the button and the roles use (`token.params.pointer_repeat`, `token.params.color`,
  `token.params.opacity`, …); S5 and S6 for `button`, through `generate_widgets.py` and `ThemeParts`; S7; S8 without
  S13; S9.1, S9.2, S9.4; S10; S11; S12 for what the model holds, with its refusal test (an exhaustive `match` over
  `LayerError` and `ProblemKind`, one triggering case per variant, so a new variant does not compile without one);
  S21.2's shape from the start (exhaustive enums, `#[non_exhaustive]` structs), so the connectors are written against
  it; S16's `Theme`, `from_preset`, `set_reader_stand_in`, `set_accent` and the query; S17's gate with its normal and
  perturbed runs and the declarations check. `PresetId` holds the presets written so far. The platform functions live
  in the directory module `next/platform/` (`platform/` after the switch); S13's strategies come in `placement/` at M3.
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
  restored and S16's system entry points (`Theme::system()`, `from_system`); the reader tests return.
- **The accent run:** `Palette::from_port` and `apply_palette` (S14, as far as the run needs); the KDE schemes and
  accents chosen and pinned (at least one that tints, one with the accent-coloured title bar and a `Header` group, one
  with it and no `Header` group, one lacking a colour group, S17), measured by spike K's script.
- **For D6:** the cost, the evidence size and the gate's run time measured and extrapolated.
- **Gate:** as M1a's, over all eight presets, with the accent run; the pull request's CI green on macOS and Windows.

## M2 — The model's core, complete (after D6; in series)

1. **The four connectors on the slice,** gpui first, then iced, egui and egui-widgets: each one's old mapping deleted;
   its spinner and animated-icon code removed (`AnimatedIcon` is gone; busy indicators return with the indicators
   unit's connector task, drawn from the `spinner` widget); everything the model holds then mapped from the query, or
   listed as what the toolkit cannot draw; its contract test (S21.3). The showcases are unlinked from their crates
   (the `[[example]]` entries removed with `autoexamples = false`, the `native-theme-showcase-support` dev-dependency
   removed, gpui's `mod showcase;` removed; files kept, back at M5), so `--all-targets` and the tests no longer build
   them. `check_connector_parity.py` is rewritten to apply its naming and sibling rules over the new resolved types'
   leaves. The steps listed until M2.1 leave `PLAN_EXCLUDED`. The connectors come first so they use the API before the
   rest of S16 is built on it.
2. The model's remaining arithmetic (S2.2), with unit tests. The platform functions come only with the facts that use
   them (M1b, M3), each with its vector table.
3. S12 complete.
4. S14 complete: one palette per variant each of the seven projects publishes (S19.1), as mappings with their labelled
   picks (the guide's own example value where it gives one, else the middle of the stated range; the picks listed in
   the ledger for the maintainer's veto); the KDE ports, with their measured facts added to the accent run; the
   projects' published colours as facts (a palette lane and its verifier); the gate's palette run (each pick skipped).
5. S16 complete; S19.3 inside `native-theme`.
6. S18: `property-registry.toml` generated with its drift test; `check_platform_facts_rows.py` and
   `check_preset_citations.py` rewritten to read the facts (S18); `check_todo_citations.py` rewritten to the model's
   paths; `check_section_refs.py` rewritten to the generated document's sections, or removed through an S22 line if
   nothing cites them any more; their steps and `ci.yml` entries leave `PLAN_EXCLUDED`.
7. S15 (AppKit rendering), in its own module, checked on CI's macOS legs.

Each task that adds a resolved field maps it in the four connectors in the same task, so M2 leaves no `pending` entry.
Token groups that add preset fields (S4.3, the rest of S4.6, S4.7, the rest of S4.8) and S13's placement are not here:
they enter with their M3 units, with their facts and vectors.
- **Gate per task:** its tests, `check_release.sh --commit` and the pull request's CI. **Milestone gate:**
  `cargo test --workspace` (outside `PLAN_EXCLUDED`), `check_function_vectors.py`, the palette run, and no `pending`
  contract entry.

## M3 — Units (the per-unit pipeline)

The units, in order: tokens (S4's remaining groups) and conditions; placement (S13: the types and every strategy with
its vector table, generated by the reference where one runs: the GTK and Qt containers, Compose's code, the WinUI host
for microsoft-ui-xaml's strategies; measured by the probe and the iOS host for AppKit and UIKit, anchor, size and window
in, frame out); readers (the catalogue's section); window and chrome; overlays; buttons and toggles; fields;
indicators; containers; the typed widgets the model lacks today, in alphabetical batches of about ten entries; the
keyed components. Per unit:

1. **Facts** (the lanes, usually done ahead): every key, declaration and state-model combination of the unit on every
   preset (S17), with the traces' rows carried.
2. **Verification** (*How values are checked*), then the spec corrections the unit found.
3. **Model and presets, in one commit:** the unit enabled in the generator (or its hand-written fields), the platform
   functions it adds with their vectors, and the unit's parts in the eight presets (written in worktrees, or generated
   if D6 so decides); the unit's new resolved fields enter each connector's contract table as `pending`.
4. **Readers:** the unit's new reads (the traces' `reader` lines), a task of its own (the gate resolves with a
   stand-in, so the readers do not change what it checks), with the pull request's CI on macOS and Windows.
5. **Connectors:** the unit mapped in the four connectors, one task in the series queue, emptying the unit's `pending`
   entries.

A unit with an open entry in the unmeasured register waits for the maintainer's decision on it; the series queue moves
on to the next unit meanwhile.
- **Gate per unit:** `cargo test -p native-theme` (the gate over the whole model), `check_facts.py`,
  `check_fact_citations.py`, `check_function_vectors.py`, `check_facts_trace.py --unit <unit>` (each live trace row of
  the unit's targets is in a fact's `rows` or carries a verdict), `generate_widgets.py --check` and the pull request's
  CI green; for a unit citing Apple's SDK headers, `os-probe.yml`'s header-citation job green.

## M4 — The facts gate, complete

- Every unit enabled; `docs/facts/imported/` emptied (each row converted, or recorded as not a model value with its
  reason); S19.2's staged file removed; uncited legacy evidence deleted and the legacy list emptied; the accent run over
  every pinned scheme and accent; the palette run over every bundled palette.
- **The drift run:** every harness runs once more on the then-current runner images, and every measured fact is
  compared with that run; the ledger records the comparison per fact, with the run's URL. A drifted fact is measured
  and verified again (citing the new run's committed evidence), or goes to the unmeasured register.
- **Gate:** `cargo test -p native-theme --test facts_gate` green on the eight presets, with no fact lacking a resolved
  key, no state-model combination lacking a fact and every declaration agreeing with its facts; `check_facts_trace.py`
  and `check_function_vectors.py` over every fact green; `check_ledger.py --final` green (`imported/` empty, the staged
  file gone, the legacy list empty, every measured fact with a drift comparison, the unmeasured register with no open
  entry). The full run calls `check_ledger.py --final` from then on.

## M5 — Showcases (after M4)

`tools/showcase-support` ported first, then the three showcases rebuilt on the query, their element lists and parity
gates following (S18), their busy indicators drawn from the `spinner` widget (S19.3); the unlinked showcase entries,
dev-dependencies and modules linked again.
- **Gate:** each connector's contract test with no `pending` entry; the showcase parity gates; the screenshots
  regenerated and compared side by side with the platforms under the same theme; `PLAN_EXCLUDED` empty and no
  `# PLAN_EXCLUDED` mark left in `ci.yml`.

## M6 — Release 0.8.0

- The CHANGELOG's 0.8.0 entry completed and dated; the READMEs, ROADMAP, `docs/todo.md`'s closed entries,
  `docs/COMPATIBILITY.toml` (`update_compatibility.sh`), `docs/new-os-version-guide.md` (facts per generation, the
  harnesses), the docs.rs build, `generate_assets_release.sh` and the provenance stamps, the full `check_release.sh`.
- The v0.7.1 documents, `tools/gap-map/`, this plan, the ledger and the two superseded connector plans archived to
  `docs/archive/`; the scripts as the table below says; `check_facts.py` takes its keys from the registry from then on;
  the memory updated; `sweep-target.sh` run.
- **Needs the maintainer:** the merge, the tag, the release and the publish.

## The new scripts

| Script | Made in | In `check_release.sh` | At M6 |
|---|---|---|---|
| `spec_catalogue.py`; the three spec checkers | M0 | no (run by hand, S18) | deleted |
| `check_sources.py` | M0 | yes | deleted |
| `repin_citations.py` | M0 | no | deleted |
| `check_ledger.py` | M0 | full run only; `--final` from M4 | deleted |
| `import_platform_facts.py` | M1a | no | deleted at M4 |
| `convert_inheritance_rules.py` (S19.2) | M1a | no | deleted at M4 |
| `generate_widgets.py` | M1a | `--check` | deleted; the generated code is the source |
| `check_facts_trace.py` | M1a | `--enabled` | deleted |
| `check_facts.py` | M0 | yes | kept |
| `check_fact_citations.py` | M0 | the full run fails without the cache; `--commit` skips it, by name, only when the cache is absent; the SDK-header citations run in a macOS job of `os-probe.yml` | kept |
| `check_function_vectors.py` | M1a | yes | kept |
| `generate_platform_facts.py` | M1a | `--check`, from the switch | kept |
| `update_sources.sh` | M0 | `--verify` in the full run | kept |
| `measure_kde_accent.sh`, the harnesses and hosts under `tools/` | M0 spikes | no (CI) | kept, for the next OS version |

## How values are checked

- **Facts, mechanically** (`check_fact_citations.py`):
  - a `published` or `read` fact's `quote` occurs, whitespace-normalised, in the cited lines of the cited file at its
    pin, and every literal of its `value` (numbers after unit normalisation, colours in any notation, a derivation's
    operands), after its `conversion`, occurs in the `quote`;
  - a `measured` fact's evidence file is committed, the value at its `evidence` path equals the fact's value after its
    `conversion` within its tolerance, and the file records the pinned generation and toolkit versions (the exact
    builds listed);
  - every evidence file is cited by a fact, except the legacy runs until M4. Evidence files are the files under
    `docs/evidence/<harness>/<date>-run-<id>/<image>/`; READMEs are not evidence, and a run's README stays while any
    file of the run is cited. Spike and drift runs record their result in the ledger with the run's URL and commit
    evidence only where a fact cites it.
- **Facts, independently:** per unit and platform, a verifier that is not the lane's author first derives the unit's
  state model and declarations from the sources, without the lane's, and the two are compared mechanically (a
  combination the lane missed would otherwise never be checked); then, for the agreed combinations, it reports a value
  and a citation for each key, never seeing the lane's, and both are compared mechanically. Each disagreement goes to a
  tie-breaker agent with both citations and the source. An audit sample of the agreed facts (during the slice 5 % per
  lane and unit, at least ten; D6 sets the rate) goes to the tie-breaker too, to measure how often a lane and its
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
  `check_function_vectors.py` checks that every public function of `platform` and `placement` has a table and a test,
  and that every platform function or placement strategy any fact or preset calls is one of them (the model's own
  arithmetic is not one). Without this, a derivation fact and the preset would both be evaluated by the same function,
  and a wrong function would pass the gate.
- **The presets need no separate author:** the gate compares them with the verified facts mechanically, so who writes
  them does not change what passes.
- **Values that cannot be measured** go to the unmeasured register in the ledger: the key and combination, the
  attempts and their evidence, and the options (another setup such as real hardware or another image, a catalogue
  correction, a release blocker). Each is the maintainer's decision, since R21 allows no partial release.
- **Evidence** is committed (GitHub's artifacts expire), images losslessly compressed; where it lives above a size
  threshold is decided at D6.

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
  fixture as their gate, each reviewed by a fresh reviewer before it closes (G4's precedent); facts lanes run one agent
  per platform and one for the palettes, briefed with the pinned sources, the traces and the facts format; verifiers,
  tie-breakers and clean-room implementers as above; tooling in its own agents on `v0.8.0-tools`. Broad searches go to
  the `locate` agent (Haiku).
- **Briefs:** every agent reads and searches only its worktree, the project, the source cache and the scratchpad
  (verifiers and clean-room implementers less, as above), and never searches the whole filesystem (no `find /`).
- **Failures:** a subagent that fails the same gate twice is not dispatched a third time; the orchestrator takes the
  task over or re-specifies it.
- **Parallel by default:** the lanes, the verifiers, the reviews and the tooling run beside the code (*Where the work
  happens*); builds and commits on `v0.8.0-rc1` stay sequential.
- **Usage:** until D6, at most three agents work at once (an agent waiting on a CI run does not count), within D7's
  cap, the lanes working ahead into M3's units only within that cap; D6 sets the caps per milestone.
- **Disk:** `~/.claude/scripts/sweep-target.sh` at each milestone's close and whenever the build folder passes about
  200 GB.
- **Progress** is recorded in the ledger and the project memory.

## Checklist

- [x] D1, D4, D5, D7, D9, D10, D11, D12 decided (2026-10-29)
- [ ] `v0.7.1-rc1` merged to `main` by the maintainer (D5)
- [ ] M0 setup
- [ ] Spikes M · G · K · I · W · A
- [ ] M1a slice core
- [ ] M1b slice complete
- [ ] D6 checkpoint
- [ ] M2 model core
- [ ] M3 units (per unit in the ledger)
- [ ] M4 facts gate complete
- [ ] M5 showcases
- [ ] M6 release 0.8.0
