---
phase: 93-docs-todo-v0-5-7-gaps-md
plan: 06
subsystem: icons
tags: [icons, doctest, dead-code, pub-crate, v057-polish, gap-closure, iconloader, cfg-attr]

# Dependency graph
requires:
  - phase: 93-03
    provides: "pub(crate) demotion of bundled_icon_svg / bundled_icon_by_name / load_freedesktop_icon_by_name; IconLoader builder established as canonical public replacement"
  - phase: 93-05
    provides: "Phase 93 Plan 05 sits at HEAD; 93-06 runs in wave 4 after all 93-01..05 land. No direct code dependency."
provides:
  - "Doctest on bundled_icon_svg rewritten to use native_theme::icons::IconLoader (public-API replacement) -- compiles and asserts IconLoader returns None for SfSymbols on non-macOS targets"
  - "Doctest on bundled_icon_by_name rewritten identically (uses IconLoader::new(name))"
  - "#[cfg_attr(not(any(feature = material-icons, feature = lucide-icons)), allow(dead_code))] on bundled_icon_by_name -- silences dead_code lint exactly and only when both feature-gated callers at icons.rs:598,603 are cfg'd out"
  - "cargo test -p native-theme --all-features --doc: 50 passed; 0 failed (was 48 passed; 2 failed)"
  - "cargo clippy -p native-theme --all-targets -- -D warnings exits 0 (was failing with dead_code error)"
  - "pre-release-check.sh advances past the clippy step at line 283 (the former failure locus) and through the tests/examples/docs steps -- now fails at the later cargo package step due to a separate pre-existing defect (see Deferred Issues)"
affects: [v057-release-gate, cargo-doc, cargo-test-doc, cargo-clippy, pre-release-check-script]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Doctest-on-pub(crate)-function pattern: rewrite to use the public replacement API (IconLoader here) rather than silently ignore (#[doc(ignore)]) or delete. Maintains a runnable example that demonstrates the correct external pattern."
    - "Conditional dead_code suppression pattern: #[cfg_attr(not(any(feature = ..., feature = ...)), allow(dead_code))] that exactly mirrors the cfg union of the function's callers. Stronger than unconditional #[allow(dead_code)] because it unmasks real dead-code regressions if callers get un-gated later."

key-files:
  created:
    - ".planning/phases/93-docs-todo-v0-5-7-gaps-md/93-06-SUMMARY.md"
  modified:
    - "native-theme/src/model/bundled.rs (three targeted edits: two doctest rewrites + one cfg_attr on bundled_icon_by_name; net +24/-6 lines)"
    - ".planning/phases/93-docs-todo-v0-5-7-gaps-md/deferred-items.md (APPEND-ONLY: 93-06 resolution notes for the three G3 follow-up items + new pre-existing cargo-package failure note)"

key-decisions:
  - "[Phase 93-06]: Doctests on pub(crate) functions rewritten to use IconLoader rather than deleted or annotated with #[doc(ignore)]. The rewrite demonstrates the correct external-caller API, which is exactly what a maintainer reading `cargo doc --document-private-items` should see. Deletion was a valid option-B but loses the runnable example."
  - "[Phase 93-06]: Dead-code suppression tied to the exact cfg predicate `not(any(feature = material-icons, feature = lucide-icons))` rather than unconditional #[allow(dead_code)]. Rationale: if one of the two feature-gated callers is un-gated in the future, the unconditional allow would silently mask real dead code; the conditional form is self-unmasking."
  - "[Phase 93-06]: #[cfg] on the function itself (Option B) rejected. Gating the function would also gate out the unconditional #[cfg(test)] tests `by_name_non_bundled_sets_return_none` and `by_name_unknown_name_returns_none` (bundled.rs:691-702), which call the function and must remain live in the default test build. The allow-attribute approach keeps the function always-compiled and always-testable."
  - "[Phase 93-06]: Deletion + inlining (Option C) rejected. Would lose the 5 internal `bundled.rs:585,679,693,694,695,700,701` test call sites that cover by-name dispatch logic (the two non-feature-gated tests especially, which assert correct return-None behaviour for unknown names and non-bundled sets)."

patterns-established:
  - "Pub(crate) doctest pattern: docstring on a pub(crate) helper whose public replacement is a builder should show that builder directly, not the private function name."
  - "Feature-cfg dead-code suppression: cfg_attr's predicate is the complement (not(any(...))) of the caller's cfg union (any(...))."

# Metrics
duration: "~13 min"
completed: "2026-04-19T18:32:18Z"
---

# Phase 93 Plan 06: G3 Follow-Up — `pub(crate)` Doctest Rewrite + Dead-Code Suppression Summary

**Three targeted edits to one file (`native-theme/src/model/bundled.rs`) closing the phase-93 verifier's two E0603 doctest failures and one dead_code clippy failure — both regressions of Plan 93-03's `pub(crate)` demotion — by rewriting both doctests to use the public `IconLoader` builder and by adding a `cfg_attr` that suppresses `dead_code` exactly when both feature-gated callers are cfg'd out.**

## Performance

- **Duration:** ~13 min
- **Started:** 2026-04-19T18:19:00Z (executor load/context phase)
- **Completed:** 2026-04-19T18:32:18Z
- **Tasks:** 1 (single atomic task per plan design)
- **Files modified:** 1 source file (`bundled.rs`) + 1 deferred-items append + 1 SUMMARY

## Accomplishments

- **Gap 1 closed (doctest E0603 on bundled_icon_svg):** rewritten to `IconLoader::new(IconRole::ActionCopy).set(IconSet::SfSymbols).load()`. Imports updated to `use native_theme::icons::IconLoader;` + `use native_theme::theme::{IconRole, IconSet};`. Compiles and the assertion holds (SfSymbols is not a bundled set on non-macOS targets).
- **Gap 2 closed (doctest E0603 on bundled_icon_by_name):** rewritten to `IconLoader::new("check").set(IconSet::SfSymbols).load()`. Same import pattern.
- **Gap 3 closed (dead_code on bundled_icon_by_name):** added `#[cfg_attr(not(any(feature = "material-icons", feature = "lucide-icons")), allow(dead_code))]` between the existing `#[must_use]` and `#[allow(unreachable_patterns, unused_variables)]` stack and the `pub(crate) fn bundled_icon_by_name` line.
- **Verification matrix (plan's 8-step verify block):** steps 1, 2, 3, 5, 6, 7 pass. Step 4 (`--all-features` clippy) and step 8 (`./pre-release-check.sh` reaches green banner) do NOT pass; both failures are **pre-existing** at parent commit `51c386b` and unrelated to Plan 93-06's scope (see Deferred Issues below).
- **Pre-release script advances 8 steps:** was failing at step 15 ("Running clippy (native-theme)" — line 283). Now passes through clippy + all tests + all examples + all docs steps and fails at step 23 ("Validating packages (core)" — line 321). The former failure locus that Plan 93-06 was chartered to unblock IS unblocked.

## Task Commits

Each task was committed atomically:

1. **Task 93-06.01: Rewrite both demoted-function doctests to use IconLoader + suppress dead_code on bundled_icon_by_name** — `7611d53` (fix)

**Plan metadata:** will be committed as `docs(93-06)` after STATE.md update.

## Files Created/Modified

- `native-theme/src/model/bundled.rs` — three targeted edits:
  - Lines 20-36: `# Examples` block on `bundled_icon_svg` doctest rewritten to use `IconLoader`.
  - Lines 194-208: `# Examples` block on `bundled_icon_by_name` doctest rewritten to use `IconLoader`.
  - Lines 211-214: `#[cfg_attr(not(any(feature = "material-icons", feature = "lucide-icons")), allow(dead_code))]` inserted between `#[allow(unreachable_patterns, unused_variables)]` and `pub(crate) fn bundled_icon_by_name`.
- `.planning/phases/93-docs-todo-v0-5-7-gaps-md/deferred-items.md` — APPEND-ONLY: added "Resolved during Plan 93-06 (G3 follow-up closure)" section closing the three G3 follow-up items, plus a "Logged during Plan 93-06 (G3 follow-up) execution" section documenting a new pre-existing `cargo package` failure.
- `.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-06-SUMMARY.md` — this file.

## Decisions Made

All four key decisions above are recorded in the frontmatter `key-decisions` block for machine scanning. Highlights:

- **Option A chosen for doctest fix (rewrite to `IconLoader`) over Option B (delete).** Preserves runnable example; serves maintainers reading `cargo doc --document-private-items`.
- **Option A chosen for dead-code fix (conditional `cfg_attr`) over Options B (`#[cfg]` on function) and C (delete + inline).** Option B would gate out the unconditional `#[cfg(test)]` tests that still call the function; Option C would lose 7 internal test call sites covering by-name dispatch.
- **Conditional cfg predicate tied to the exact caller union** — `not(any(material-icons, lucide-icons))` — not unconditional `#[allow(dead_code)]`. The conditional form self-unmasks if callers get un-gated later.

## Deviations from Plan

### Auto-fixed Issues

**None.** The plan body was fully prescriptive: three literal edits to one file, exact commit message template, exact doctest replacements. Executor applied them verbatim. No Rule-1/2/3 fixes were required in the code itself.

---

**Total deviations:** 0 code-level auto-fixes.
**Impact on plan:** Plan executed exactly as written at the code level. The only deviation is at the verification-reporting level (two of the plan's 8 verify steps don't pass due to pre-existing defects outside this plan's scope — see Deferred Issues).

## Deferred Issues (Pre-existing, out of Plan 93-06's SCOPE BOUNDARY)

### 1. `cargo clippy -p native-theme --all-targets --all-features -- -D warnings` fails

Plan verify **step 4** is `--all-features` clippy green. It surfaces pre-existing errors in files Plan 93-06 does not touch:

- `native-theme/src/spinners.rs:230,263` — `unwrap()` on `Result` (clippy `unwrap_used`, last touched by Phase 82-01 commit `c748b1b`).
- `native-theme/src/freedesktop.rs:346` — `assert!(false, ...)` (clippy `assertions_on_constants`, last touched by Plan 93-03 commit `7ba2b4c`).
- `native-theme/tests/reader_kde.rs` — 14 `unused_import` / `unused_variable` errors (kde-feature-gated test harness, last touched by Phase 78-01).
- Multiple `unused` / `unreachable_pattern` / range-contains manual-impl errors elsewhere in the lib, all pre-existing.

**Scope-boundary attestation:** `git stash && cargo clippy -p native-theme --all-targets --all-features -- -D warnings` reproduces the same errors without Plan 93-06's edits. Plan 93-06 does NOT touch `spinners.rs`, `freedesktop.rs`, or `reader_kde.rs`.

**Why not auto-fix:** Plan 93-06's `files_modified` frontmatter lists exactly ONE file (`native-theme/src/model/bundled.rs`). Fixing these pre-existing issues would involve touching 3+ unrelated files with distinct rationales; each fix is its own non-trivial task (for example, `freedesktop.rs:346`'s `assert!(false, ...)` is inside a match arm intended to be unreachable during the test — the idiomatic fix is `unreachable!()` but the larger match's ergonomics need careful review).

**Why this is acceptable for the G3 gap:** `./pre-release-check.sh` (the project's actual release-gate per `feedback_run_prerelease_check.md`) runs clippy **WITHOUT** `--all-features` (line 283: `cargo clippy -p "$crate" --all-targets -- -D warnings`). The no-features clippy step IS green post-Plan-93-06. The `--all-features` clippy is an extra verification the plan author included as a belt-and-suspenders check; it is NOT the release gate.

### 2. `./pre-release-check.sh` — fails at step 23 ("Validating packages (core)")

Plan verify **step 8** is the full script reaching its green banner. It does not — but the failure locus has moved from step 15 (the step Plan 93-06 was chartered to unblock, now passing) to step 23 (`cargo package -p native-theme-derive -p native-theme -p native-theme-build --allow-dirty`).

**Error signature:**
```
error[E0432]: unresolved import `native_theme_derive::ThemeFields`
  --> src/model/border.rs:4:5
  (and sibling errors in defaults.rs, font.rs, icon_sizes.rs, widgets/mod.rs)
error: cannot find attribute `theme_layer` in this scope
  --> src/model/font.rs:157:3
  (and sibling on font.rs:262:3)
```

**Root cause (pre-existing at parent commit `51c386b`):** Plan 93-05 (commit `4431782 feat(93-05): add ThemeFields derive and FieldInfo inventory registry`) added the `ThemeFields` proc-macro derive to `native-theme-derive v0.5.7` and the consuming `use native_theme_derive::ThemeFields;` imports + `#[theme_layer(fields = "...")]` attributes to five `native-theme` source files. However, **`native-theme-derive v0.5.7` has not yet been published to crates.io**. `cargo package`'s tarball verification step builds each crate in isolation from the tarball (simulating the published-to-crates.io state), pulling dependencies from the crates.io index rather than the workspace `path = "..."`. The packaged `native-theme-0.5.7.crate` therefore cannot resolve `native_theme_derive::ThemeFields` because only pre-93-05 published versions are available there.

**Scope-boundary attestation (double-confirmed):**
- `git stash && cargo package ... --allow-dirty` (stashing my three edits only) reproduces the same 54 errors.
- `git checkout 51c386b -- . && cargo package ... --allow-dirty` (checking out the parent commit before 93-07 and before 93-06) reproduces the same 54 errors.

Plan 93-06 does NOT touch `native-theme-derive`, does NOT touch any of the five files that import `ThemeFields`, and does NOT touch `bundled.rs` in any way that interacts with this failure (`bundled.rs` has no `ThemeFields` derive usage).

**Why not auto-fix (Rules 1-3):** The fix is to `cargo publish` `native-theme-derive v0.5.7` to crates.io. This is a **release action**, not a code change, and is governed by user memory rule `feedback_never_bypass_checkpoints.md` ("Never publish, push tags, create releases without EXPLICIT user approval"). Publishing is out of scope for any automated plan executor.

**Why not Rule 4 (ask):** Not an architectural decision. The path forward is unambiguous — publish `native-theme-derive v0.5.7`, then `cargo package` will resolve correctly. The only open question is **timing** (the user decides when to cut the release), not **approach**.

**Re-evaluation trigger:** First time the user runs `cargo publish -p native-theme-derive` for v0.5.7, the `Validating packages (core)` step of `./pre-release-check.sh` should transition to green, and the full script should render its success banner.

## Issues Encountered

None. All three edits applied cleanly first-try; no debugging iterations; no hook friction. Parallel plan 93-07 landed a docs-only commit on main (`a6e8d4e`) between my Read and Edit calls, but touched a disjoint file set (`docs/todo_v0.5.7_gaps.md` + `.planning/phases/93-docs-todo-v0-5-7-gaps-md/deferred-items.md`) and did not create any merge concern.

## User Setup Required

None.

## Next Phase Readiness

- **G3 is fully closed.** All three verifier-identified defects (two doctest E0603, one dead_code clippy) are resolved. Phase-93 re-verification will find Gap 1 (truth 6 in VERIFICATION.md) and Gap 2 (truth 15 in VERIFICATION.md) closed.
- **Release gate is one action away from green.** Once `cargo publish -p native-theme-derive` runs (or the user opts to scope the package check to per-crate-excluding-derive for CI purposes, matching Plan 93-07's per-crate-posture framing for naga/workspace), `./pre-release-check.sh` should complete with the success banner.
- **Phase 93 requirements coverage:** G1 (Plan 01), G2 (Plan 02), G3 (Plan 03 + this Plan 06 follow-up), G4 (Plan 04), G5 (Plan 05), G11 (Plan 07) all addressed. Phase-93 verification can re-run to confirm programmatic closure.
- **No blockers for phase 93 closure** beyond the user-approval gate for publishing `native-theme-derive v0.5.7`.

## Self-Check: PASSED

**1. Created files exist:**
- `.planning/phases/93-docs-todo-v0-5-7-gaps-md/93-06-SUMMARY.md` — FOUND (this file).

**2. Modified files have the expected changes:**
- `native-theme/src/model/bundled.rs` — FOUND (lines 20-36, 194-208, 211-214 all contain the expected post-edit content).
- `.planning/phases/93-docs-todo-v0-5-7-gaps-md/deferred-items.md` — FOUND (append-only additions after the 93-07 section).

**3. Commit exists:**
- `7611d53 fix(93-06): close G3 follow-up — rewrite demoted-fn doctests + suppress dead_code` — FOUND in `git log --oneline -1`.
- Commit touched exactly one source file (`native-theme/src/model/bundled.rs`, +24/-6).
- No `Co-Authored-By` trailer in the commit message (verified by `git show HEAD`).

**4. Verification matrix (plan's 8 steps):**
- Step 1 (`cargo build --all-features`): PASS (Finished dev profile in 1.62s).
- Step 2 (`cargo build -p native-theme`, no features): PASS (Finished dev profile in 8.57s — exercises the dead-code cfg_attr).
- Step 3 (`cargo clippy -p native-theme --all-targets -- -D warnings`): PASS (Finished; exit 0; NO "function bundled_icon_by_name is never used").
- Step 4 (`cargo clippy -p native-theme --all-targets --all-features -- -D warnings`): DEFERRED — pre-existing failures in spinners.rs/freedesktop.rs/reader_kde.rs; see Deferred Issues #1.
- Step 5 (`cargo test -p native-theme --all-features --doc`): PASS (`test result: ok. 50 passed; 0 failed; 10 ignored`).
- Step 6 (`cargo test -p native-theme --all-features --lib`): PASS (`test result: ok. 791 passed; 0 failed; 3 ignored`).
- Step 7 (`cargo test -p native-theme --all-features --tests`): PASS (79 passed across 8 test binaries: 12+6+2+12+11+9+19+8).
- Step 8 (`./pre-release-check.sh` reaches green banner): DEFERRED — advances past the target clippy step but fails at a later pre-existing `cargo package` step; see Deferred Issues #2.

6 of 8 steps PASS. The 2 DEFERRED steps are both due to pre-existing defects at HEAD that are outside Plan 93-06's one-file scope and outside the SCOPE BOUNDARY rule. Plan 93-06's chartered deliverable (close G3's three defects) is fully achieved; the new failure locus is a release-sequencing artifact whose fix is a `cargo publish` action requiring explicit user approval.

---

*Phase: 93-docs-todo-v0-5-7-gaps-md*
*Plan: 06*
*Completed: 2026-04-19T18:32:18Z*
