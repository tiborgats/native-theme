# v0.7.0 — GPUI Kit 0.7.3: Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development
> (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use
> checkbox (`- [ ]`) syntax for tracking.

**Goal:** G1 — native-theme-gpui on GPUI Kit 0.7.3 with a `gpui-fast` feature and `Diff` in the
showcase; G2 — the tooltip show delay, the dialog entrance and the list header height as theme
values with receivers in all three connectors.

**Architecture:** G1 changes the gpui connector and the OS probe (native-theme only if M4's first
CI run fails): floors, citations, the showcase, the backend feature. G2 goes core-first (facts, model, presets), then
each connector's receiver, its contract or tests, and its showcase.

**Tech stack:** Rust 2024; workspace MSRV 1.88.0, the gpui and egui connectors 1.95.0;
gpui-component / gpui-base / gpui-kit 0.7.3, gpui-pre 0.3.9, gpui-fast 0.1.7; egui 0.36.2;
iced 0.14.

**Spec:** [`todo_v0.7.0_gpui-kit-0.7.3-spec.md`](todo_v0.7.0_gpui-kit-0.7.3-spec.md); reasons:
[`todo_v0.7.0_gpui-kit-0.7.3-rationale.md`](todo_v0.7.0_gpui-kit-0.7.3-rationale.md).

**When:** on `v0.7.0-rc1`, after M4's stop point B (`df86348c`), in the order G1 → M5's fix
phase → G2 → M6, M7 (rationale L10). M5's review phase is at stop point A in session
`native-theme-b5` (`308c988d`, its tree clean); its fix phase starts when G1 has closed. An Opus session
orchestrates; code tasks run in `implement` agents (Opus); a fresh reviewer per task. Approved
by the maintainer on 2026-10-13: G2's API breaks; the pushes and the probe dispatch during G1
(the orchestrating session does them).

## Global Constraints

- No `unwrap` / `expect` / `panic!` / `unreachable!` / slice indexing / unchecked integer
  arithmetic outside tests; no new `unsafe` outside `tools/os-probe/src/{macos,windows}/ffi.rs`.
- NEVER invent a value: preset value ← facts row ← pinned primary source or probe run;
  unsourced stays unset.
- The new fields are `soft_option`; egui's mapping contract stays exhaustive in every commit;
  each showcase applies each new receiver.
- Commits: explicit paths (`git commit -- <paths>`, never `-a` or `add -A`: another session
  shares the tree); author the maintainer; no AI attribution trailer; built in the project's
  `target/`. Before editing the ledger or the review plan, re-read it.
- **Gates per commit:** `./scripts/check_release.sh --commit` green. One exception, stated in
  advance: G1's Tasks 2–5 form one unit, because the bump itself turns three gates red. Task
  2's commit leaves the two showcase citation gates and the coverage gate red; Task 3's leaves
  the citation gates red only for the claims Task 4 rewrites; Task 4's leaves only the coverage
  gate red; Task 5's makes the branch green again. Every other commit is green.
- Pushes (approved: the pushes during G1): G1 Task 1 and G1 Task 8 push `v0.7.0-rc1` to draft
  PR #3, and so does any G1 fix of a CI failure. G2 pushes nothing without the maintainer. No
  upstream issue is filed by an agent.

## Review Focus

1. **An application enabling `gpui-kit/gpui-fast`** — form A: the connector builds with no
   action (Task 6's `--features gpui-kit/gpui-fast,gpui-kit/component` check); form B: the README
   says to enable the connector's feature; without it the build fails with the 365 type errors.
2. **A preset or overlay without the new fields**: it resolves, and every receiver keeps the
   toolkit's or the application's value (gpui's tooltip delay: the application's, spec §7.4;
   egui's, iced's, SlideDown, the body row height). (Tasks 10–14, a test each.)
3. **A theme switch from one stating `show_delay_ms` to one that does not**: gpui's
   `TooltipDefaults` goes back to the application's delay (installed before the connector's first
   write, or set by the application since; 500 ms if it never set one).
   (Task 11.)
4. **Odd overlay values**: `header_height_px = 0` or negative fails validation naming the TOML
   key (the generated range checks use `toml_key()`); `show_delay_ms` outside `u16` fails to
   load. (Task 10.)
5. **GPUI Fast at run time**: the showcase on GPUI Fast lays out exactly like the gpui-pre build
   and survives theme switches. (Task 7.)
6. **An application that installed its own `TooltipDefaults`** (delay or grace period): a theme
   apply keeps its grace period, and a theme without a delay keeps its delay. (Task 11.)

---

## Task 0: Register the milestones (at approval)

- [ ] `docs/todo_code-review-plan.md` (re-read first): G1 and G2 in the milestone table after
  M4; a decision line D10 pointing at these three documents and stating the order G1 → M5's fix
  phase → G2 → M6, M7; the *Resuming* rule names G1 and G2.
- [ ] `docs/todo_code-review-findings.md` (re-read first): *G1* and *G2* rows in the status
  table and their sections.
- **Commit:** `docs(review): GPUI Kit 0.7.3 as milestones G1 and G2 after M4` (with the three
  documents).

## G1

### Task 1: Probe code, first push, probe run

- [ ] Spec §6's probe code in `tools/os-probe`; clippy clean on Linux and both cross targets.
  **Commit:** `feat(os-probe): the macOS table header height and the Windows hover time`.
- [ ] Push `v0.7.0-rc1` to draft PR #3 and dispatch `os-probe.yml` on it. The PR's CI runs M4's
  macOS and Windows tests for the first time (`docs/todo.md`, the M4 entry on unrun platform
  tests). Record both runs in the ledger.
- [ ] If an M4 platform test fails, a G1 task fixes it (native-theme code, its own review),
  before Task 8 closes G1. The probe run does not block G1 or G2 (spec §6).

### Task 2: Floors, lockfile, README floors

- [ ] Spec §1. The two `compat::` README tests fail after the manifest edit and pass after the
  README edit; `cargo check -p native-theme-gpui --all-targets --all-features` clean;
  `cargo +1.95.0 check -p native-theme-gpui --all-features --locked` passes.
- **Commit:** `build(gpui): move to gpui-component, gpui-base and gpui-kit 0.7.3 on gpui-pre 0.3.9`.
  Leaves red: the two showcase citation gates, the coverage gate (Tasks 3–5).

### Task 3: Renumber the moved citations

- [ ] Spec §2.1. Gates: the digits-only check prints nothing; the line-text check (every
  renumbered citation's new line(s) hold the old line(s)' text) passes; the colour-claim gate's
  failures drop to the hand-read set.
- **Commit:** `docs(gpui): the citations that only moved between GPUI Kit 0.7.1 and 0.7.3 and gpui-pre 0.3.8 and 0.3.9 are renumbered`.

### Task 4: Read, rewrite, re-measure

- [ ] Spec §2.2. Gate: both showcase citation gates pass.
- **Commit:** `docs(gpui): the claims and prose GPUI Kit 0.7.3 changed are rewritten to what its code does`.

### Task 5: `Diff` and the DataTable claims

- [ ] Spec §3, the two tests first (they fail: no sample, no widget).
- Gate: coverage `missing 0`; the colour-claim gate passes; `check_release.sh --commit` green.
- **Commit:** `feat(showcase-gpui): the Typography page shows Diff, and the DataTable panel claims the sort button and the right-clicked cell outline`.

### Task 6: The `gpui-fast` feature

- [ ] **Step 1:** The draw-helper fix (spec §4.3) first; harmless on gpui-pre (the default
  build's tests still pass).
- [ ] **Step 2:** Form A (spec §4.1) and its acceptance list. If any item fails, revert to the
  Step 1 state and do form B (§4.2), recording why in the commit body. With either form,
  `cargo test -p native-theme-gpui --features gpui-fast --all-targets --no-fail-fast` passes.
- [ ] **Step 3:** The rest of spec §4.3: docs.rs list and its compat test (fails before the
  list, passes after); `REQUIRED`, the README Compatibility row and `update_compatibility.sh`;
  CI jobs; `check_release.sh`, `check_features.sh`, `cargo semver-checks --default-features`;
  the README and crate-docs sections.
- **Gate:** clippy and the strict panic lints on both backends; `cargo +1.95.0 check` on both;
  the packaging check of spec §4.1, with and without `--features gpui-fast`.
- **Commit:** `feat(gpui): a gpui-fast feature, so the connector builds on GPUI Kit's GPUI Fast backend`.

### Task 7: GPUI Fast at run time

- [ ] Spec §4.3 *A real run* (the agent runs it on the maintainer's session): the theme
  switches, a screenshot looked at, the two builds' layout dumps compared (equal positions and
  sizes; any difference fixed or explained in the ledger). **Commit:** the ledger's note,
  `docs(review): the showcase on GPUI Fast`.

### Task 8: G1 documents and close

- [ ] Spec §5; `scripts/update_compatibility.sh run gpui`; the full `./scripts/check_release.sh`
  (spec §11's expected items only); `cargo semver-checks -p native-theme-gpui --baseline-version 0.6.1 --default-features`,
  then `rm -rf target/semver-checks`; the packaging check with `--features gpui-fast` again.
  **Commit:** `docs(gpui): GPUI Kit 0.7.3 and the gpui-fast backend in the changelog, roadmap and todo list; the compatibility stamp`.
- [ ] Push `v0.7.0-rc1`; PR #3's CI green, the new gpui-fast jobs on Linux, macOS and Windows
  included (their first run). A failure is fixed in G1.
- [ ] The ledger's G1 status (done); tell session `native-theme-b5` that G1 has closed (M5's
  fix phase may start); `~/.claude/scripts/sweep-target.sh /home/tibi/Rust/native-theme`.

## M5's fix phase (session `native-theme-b5`, not this plan)

It lands M1-T7-06 (accessors named after model fields; `scripts/check_connector_parity.py`)
and possibly M1-T7-03 (leaf coverage); G2 follows the rules it leaves (spec §0.2).

## G2

### Task 9: Facts

- [ ] The probe run's outputs, if landed, in `docs/evidence/os-probe/<date>-run-<id>/` with their
  README (spec §6).
- [ ] The two checks of spec §8.2 (KDE: `Kirigami.Dialog`'s entrance at the KIRI pin; Material:
  MDC's dialog animation at the MDC pin and Compose's `AlertDialog.kt` at AX), recorded in the
  spec's row.
- [ ] The three facts rows (spec §7.2, §8.2, §9.2) in `docs/platform-facts.md`, citations as
  full links at their pins, **and in the same commit** the renumbering of spec *Facts rows
  shift line numbers*. Gates: the digits-only and line-text checks; `cargo test -p native-theme`
  (`documented_sizes` and `platform_facts_xref` included).
- **Commits:** `docs(evidence): the OS probe's second run` (if landed);
  `docs(platform-facts): the tooltip show delay, the dialog entrance and the list header height, with the line citations that follow them`.

### Task 10: The three fields in core

- [ ] Tests first: each preset resolves to spec §7.3 / §8.3 / §9.3's values, unset where
  unset; the `documented_sizes.rs` row for `list.header_height` (spec §9.1) fails;
  `header_height_px = 0` and `-1` fail validation naming `list.header_height_px`;
  `show_delay_ms = 70000` fails to load (a `u16` overflow; the error's wording is the TOML
  parser's).
- [ ] Model fields and enum, registry lines, `[no_inheritance]` entries, validation, preset
  values with their facts citations. Each commit also adds the field's `EG/mapping.toml` row
  (final, or `unmappable` with an interim reason Task 12 replaces), regenerates `EG/src/mapping.md`
  (`mapping_md_is_current`), and updates egui's `verdict_totals_match_the_specification` and the
  leaf count (`EG/src/mapping_tests.rs:236-241`), so egui's mapping tests stay green.
- Gate: native-theme's tests (the registry key-set test included); `preset-validator` clean for
  the new values; cross-target clippy.
- **Commits:** one per field: `feat(native-theme): the tooltip show delay as theme data`;
  `feat(native-theme): the dialog entrance as theme data`;
  `feat(native-theme): the list header height as theme data`.

### Task 11: gpui receivers

- [ ] Spec §7.4, §8.4, §9.4 (gpui): the `TooltipDefaults` write (with the restore of the
  application's value), the accessors and builders under the names M5's parity rule requires;
  the showcase applies them (its GPUI-native tooltips, every Dialog / AlertDialog, the DataTable
  and the Basic page's header); tests set, unset, set-then-unset, the application's values kept
  (Review Focus 2, 3, 6); the crate-docs coverage table; `HOW` lines in `SC/info/leaves.rs` for
  the new leaves (`every_listed_leaf_has_a_route`); and, in the same commit, the two panel texts
  this makes false (`SC/info/feedback.rs:750`, `SC/info/chrome.rs:981`, "which the showcase does
  not set").
- **Commit:** `feat(gpui): the tooltip show delay, the dialog entrance and the DataTable header height from the theme`.

### Task 12: egui receivers

- [ ] Spec §7.4, §8.4, §9.4 (egui): the tooltip delay, the header height accessor, the dialog
  row unreachable; `mapping.toml`, `mapping.md`, the tests; the showcase's tables; the parity
  check passes.
- **Commit:** `feat(egui): the tooltip show delay and the list header height from the theme`.

### Task 13: iced receivers

- [ ] Spec §7.4, §9.4 (iced): the two accessors; the showcase's six tooltips and its table
  header; the coverage table and its unreachable sentence; the parity check passes.
- **Commit:** `feat(iced): the tooltip show delay and the list header height from the theme`.

### Task 14: Shared element list and panels

- [ ] Spec §7.4 and §9.4's showcase items: `docs/showcase-elements.toml`; the three showcases'
  leaf readers take `_ms` (with their tests); the gpui panels claim the three values; the
  remaining texts that say the model states no hover delay (`SC/info/feedback.rs:754`,
  `SC/info/registry.rs:14-15`) are rewritten and the egui showcase's citation of `registry.rs`
  (`EG/examples/showcase-egui/main.rs:44-46`) follows; `scripts/check_showcase_parity.py
  --check-list` passes.
- **Commit:** `docs(showcase): the element list and Widget Info name the new theme values`.

### Task 15: G2 documents, late probe values, gates, archive

- [ ] If the probe run landed after Task 10, its values (`windows-11`'s delay, the macOS header
  heights) with their facts cells, in one commit with its tests.
- [ ] Spec §10; `scripts/update_compatibility.sh run` (all three connectors); the full
  `./scripts/check_release.sh`; `cargo semver-checks` for native-theme (spec §11) and the
  connectors (gpui with `--default-features`), then `rm -rf target/semver-checks`.
- [ ] The three documents to `docs/archive/` with an *As built* note at the top of the spec; the
  archive index row; the review plan's D10 links and the ledger's G1 / G2 links pointed at the
  archive; the ledger's G2 status;
  the memory; `~/.claude/scripts/sweep-target.sh`.
- [ ] **Maintainer:** the next `generate_assets_release.sh` run refreshes the screenshots.
- **Commits:** `docs: the tooltip show delay, the dialog entrance and the list header height in the changelog, READMEs and roadmap`;
  `docs: archive the implemented GPUI Kit 0.7.3 design documents`.
