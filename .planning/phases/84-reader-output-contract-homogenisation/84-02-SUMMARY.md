---
phase: 84-reader-output-contract-homogenisation
plan: 02
subsystem: pipeline
tags: [reader-result, type-safety, reader-output, pipeline, contract-test]

# Dependency graph
requires:
  - phase: 84-reader-output-contract-homogenisation
    plan: 01
    provides: ReaderOutput enum, run_pipeline with 8 params, theme_to_reader_output bridge
provides:
  - ReaderResult struct bundling ReaderOutput + metadata for clean pipeline interface
  - All four platform readers (KDE, GNOME, Windows, macOS) returning ReaderResult natively
  - run_pipeline simplified to 3 params (ReaderResult, preset_name, mode)
  - preset_as_reader helper for fallback paths
  - Contract tests proving Single/Dual variant semantics
affects: [pipeline, readers, overlays, connectors]

# Tech tracking
tech-stack:
  added: []
  patterns: [ReaderResult struct for pipeline entry, preset_as_reader for fallback paths, reader_mode test helper pattern]

key-files:
  created: []
  modified:
    - native-theme/src/lib.rs
    - native-theme/src/pipeline.rs
    - native-theme/src/kde/mod.rs
    - native-theme/src/gnome/mod.rs
    - native-theme/src/windows.rs
    - native-theme/src/macos.rs

key-decisions:
  - "ReaderResult struct replaces 8-param run_pipeline -- bundles ReaderOutput + name + icon_set + layout + font_dpi + accessibility"
  - "from_kde_content_pure unchanged (pub, returns Theme tuple) -- integration tests depend on it"
  - "preset_as_reader helper extracts fallback pattern (10 call sites reduced to single helper)"
  - "Dual variant #[allow(dead_code)] on Linux since macOS code is cfg-gated out"

patterns-established:
  - "ReaderResult: all pipeline entry points return this struct, run_pipeline accepts it"
  - "reader_mode() test helper: extract ThemeMode from ReaderResult for assertions"
  - "preset_as_reader: build ReaderResult from preset for fallback paths without platform reader"

requirements-completed: [READER-01]

# Metrics
duration: 20min
completed: 2026-04-13
---

# Phase 84 Plan 02: Reader Migration to ReaderResult Summary

**All four platform readers return ReaderResult natively; run_pipeline simplified from 8 to 3 params with Single/Dual contract tests**

## Performance

- **Duration:** 20 min
- **Started:** 2026-04-13T19:47:08Z
- **Completed:** 2026-04-13T20:07:08Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Defined `ReaderResult` struct bundling `ReaderOutput` + reader metadata (name, icon_set, layout, font_dpi, accessibility)
- Migrated all four platform readers (KDE, GNOME, Windows, macOS) to return `ReaderResult` natively
- Simplified `run_pipeline` from 8 parameters to 3: `(ReaderResult, preset_name, mode)`
- Removed `theme_to_reader_output` bridge function (Plan 01 temporary)
- Added `preset_as_reader` helper consolidating 10 fallback call sites
- Added two contract tests proving Single fills inactive from preset while Dual uses both reader variants
- All 482 lib tests + 41 integration tests pass, clippy clean

## Task Commits

1. **Task 1: Migrate all readers to ReaderResult; refactor run_pipeline** - `a3cb768` (feat)
2. **Task 2: Add Single/Dual contract tests, cleanup** - `40272ac` (test)

## Files Created/Modified
- `native-theme/src/lib.rs` - Added ReaderResult struct, #[allow(dead_code)] on Dual variant for Linux builds
- `native-theme/src/pipeline.rs` - Simplified run_pipeline to 3 params, added preset_as_reader helper, removed bridge, added contract tests, updated doc comment
- `native-theme/src/kde/mod.rs` - from_kde_content and from_kde return ReaderResult; from_kde_content_pure unchanged (pub)
- `native-theme/src/gnome/mod.rs` - build_gnome_spec_pure, build_theme, from_gnome, from_kde_with_portal all return ReaderResult
- `native-theme/src/windows.rs` - build_theme returns ReaderResult, from_windows returns ReaderResult
- `native-theme/src/macos.rs` - from_macos returns ReaderResult with Dual variant

## Decisions Made
- **ReaderResult struct over tuple:** Bundles all 6 pieces of reader metadata into a single struct, reducing run_pipeline from 8 params to 3. Much cleaner than the intermediate 6-tuple approach mentioned in the plan.
- **from_kde_content_pure unchanged:** Stays `pub`, returns `(Theme, Option<f32>, AccessibilityPreferences)` -- integration tests in `native-theme/tests/reader_kde.rs` depend on it from external crate context.
- **preset_as_reader helper:** Extracted common fallback pattern (build ReaderResult from preset) used by ~10 from_system_inner call sites.
- **#[allow(dead_code)] on Dual:** On Linux, macOS reader code is cfg-gated out so Dual variant appears unused. Allow attribute is correct since the variant IS used on macOS builds.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] #[allow(dead_code)] on Dual variant**
- **Found during:** Task 1 (clippy verification)
- **Issue:** On Linux, ReaderOutput::Dual is only constructed in macOS-gated code, triggering dead_code lint
- **Fix:** Added `#[allow(dead_code)]` on the Dual variant (it IS used on macOS)
- **Files modified:** native-theme/src/lib.rs
- **Committed in:** 40272ac

**2. [Rule 1 - Bug] Fix collapsible_if in KDE from_kde icon cascade**
- **Found during:** Task 2 (pre-release check)
- **Issue:** Nested `if` in from_kde for icon_theme cascade flagged by clippy::collapsible_if
- **Fix:** Collapsed to single `if` with `&&` let chain
- **Files modified:** native-theme/src/kde/mod.rs
- **Committed in:** 40272ac

---

**Total deviations:** 2 auto-fixed (2 bug fixes)
**Impact on plan:** Both auto-fixes necessary for lint compliance. No scope creep.

## Issues Encountered
- Pre-existing dead_code warning for `build_gnome_spec_pure` in gnome/mod.rs when building from iced connector (portal feature not enabled). Not caused by this plan, out of scope. Same issue noted in 84-01-SUMMARY.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 84 complete: ReaderOutput enum + ReaderResult + all readers migrated + contract tests
- All variant-ambiguity comments and bridge functions eliminated
- Pipeline accepts clean type-safe ReaderResult from all readers
- Ready for Phase 85 (data model method and doc cleanup)

---
*Phase: 84-reader-output-contract-homogenisation*
*Completed: 2026-04-13*
