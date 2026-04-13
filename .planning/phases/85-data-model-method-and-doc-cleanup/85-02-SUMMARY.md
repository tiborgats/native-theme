---
phase: 85-data-model-method-and-doc-cleanup
plan: 02
subsystem: model
tags: [font-size, api-naming, documentation, theme-watcher]

# Dependency graph
requires: []
provides:
  - "FontSize::to_logical_px(dpi) method with doctest (replaces to_px)"
  - "ThemeWatcher module-level doc block covering RAII, shutdown, constructor split"
affects: [86-validation-and-lint-codegen-polish, 87-font-family-arc-str]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Method naming: to_logical_px makes DPI parameter asymmetry explicit at call sites"

key-files:
  created: []
  modified:
    - native-theme/src/model/font.rs
    - native-theme/src/resolve/validate_helpers.rs
    - native-theme/src/resolve/tests.rs
    - native-theme/src/watch/mod.rs

key-decisions:
  - "Doctest uses native_theme::theme::FontSize path (FontSize is pub(crate) at crate root, pub via theme module)"
  - "Module example in watch/mod.rs uses ? instead of .expect() for zero-panic compliance"

patterns-established:
  - "to_logical_px naming convention: method names should expose parameter semantics at call site"

requirements-completed: [NAME-03, NAME-02]

# Metrics
duration: 5min
completed: 2026-04-13
---

# Phase 85 Plan 02: FontSize rename and ThemeWatcher documentation Summary

**Renamed FontSize::to_px to to_logical_px with doctest, and documented ThemeWatcher RAII/shutdown/constructor internals**

## Performance

- **Duration:** 5 min
- **Started:** 2026-04-13T20:48:19Z
- **Completed:** 2026-04-13T20:53:31Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Renamed FontSize::to_px to to_logical_px across all callers, doc comments, and test functions (zero remaining to_px references)
- Added doctest on to_logical_px demonstrating both Pt (DPI-dependent) and Px (DPI-ignored) branches
- Expanded watch/mod.rs module doc from 29 lines to 62 lines covering RAII ownership, three-phase shutdown, and constructor split rationale

## Task Commits

Each task was committed atomically:

1. **Task 1: Rename FontSize::to_px to to_logical_px and update all callers** - `54ea6bc` (feat)
2. **Task 2: Document ThemeWatcher internals and constructor split** - `2a31d80` (docs)

## Files Created/Modified
- `native-theme/src/model/font.rs` - Renamed method, updated doc comments, added doctest, renamed test functions
- `native-theme/src/resolve/validate_helpers.rs` - Updated 4 callers and 4 doc comment references
- `native-theme/src/resolve/tests.rs` - Updated comment and test function name referencing to_px
- `native-theme/src/watch/mod.rs` - Expanded module-level doc block with RAII/shutdown/constructor documentation

## Decisions Made
- Doctest uses `native_theme::theme::FontSize` path since FontSize is re-exported via `pub mod theme` but only `pub(crate)` at crate root
- Watch module example changed from `.expect()` to `?` with `Ok::<(), native_theme::Error>(())` return for zero-panic compliance
- Test function `validate_converts_pt_to_px_at_96_dpi` in resolve/tests.rs also renamed (not in plan but required by done criteria: zero to_px matches)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed doctest import path**
- **Found during:** Task 1 (doctest verification)
- **Issue:** Plan specified `use native_theme::FontSize` but FontSize is `pub(crate)` at crate root; only accessible via `native_theme::theme::FontSize`
- **Fix:** Changed import to `use native_theme::theme::FontSize`
- **Files modified:** native-theme/src/model/font.rs
- **Verification:** `cargo test -p native-theme --doc FontSize` passes
- **Committed in:** 54ea6bc (Task 1 commit)

**2. [Rule 1 - Bug] Renamed additional test function in resolve/tests.rs**
- **Found during:** Task 1 (Step 6 grep verification)
- **Issue:** `validate_converts_pt_to_px_at_96_dpi` test function name contained `to_px` substring, violating done criteria
- **Fix:** Renamed to `validate_converts_pt_to_logical_px_at_96_dpi`
- **Files modified:** native-theme/src/resolve/tests.rs
- **Verification:** grep returns zero matches for bare to_px
- **Committed in:** 54ea6bc (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 bug)
**Impact on plan:** Both fixes necessary for correctness. No scope creep.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 85 complete (both plans done): NAME-02, NAME-03, MODEL-04, MODEL-05 all delivered
- Ready for Phase 86 (validation and lint codegen polish)

---
*Phase: 85-data-model-method-and-doc-cleanup*
*Completed: 2026-04-13*
