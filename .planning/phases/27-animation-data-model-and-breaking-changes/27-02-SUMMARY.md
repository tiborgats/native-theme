---
phase: 27-animation-data-model-and-breaking-changes
plan: 02
subsystem: icons
tags: [breaking-change, rename, icon-role, semantic]

# Dependency graph
requires: []
provides:
  - "IconRole::StatusBusy variant replacing StatusLoading across entire workspace"
  - "Zero StatusLoading references in any .rs file"
affects: [28-loading-indicator-api, 32-documentation]

# Tech tracking
tech-stack:
  added: []
  patterns: []

key-files:
  created: []
  modified:
    - native-theme/src/model/icons.rs
    - native-theme/src/model/bundled.rs
    - native-theme/src/winicons.rs
    - connectors/native-theme-gpui/src/icons.rs
    - connectors/native-theme-gpui/examples/showcase.rs

key-decisions:
  - "Renamed StatusLoading to StatusBusy for semantic accuracy -- static icon cannot represent loading"

patterns-established: []

requirements-completed: [BREAK-01, BREAK-02]

# Metrics
duration: 5min
completed: 2026-03-18
---

# Phase 27 Plan 02: Rename StatusLoading to StatusBusy Summary

**Atomic rename of StatusLoading to StatusBusy across 5 files (enum variant, ALL array, 5 icon_name functions, bundled lookups, gpui connector, all tests)**

## Performance

- **Duration:** 5 min
- **Started:** 2026-03-18T05:01:59Z
- **Completed:** 2026-03-18T05:07:39Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- Renamed IconRole::StatusLoading to IconRole::StatusBusy in enum definition and doc comment
- Updated all 5 icon_name mapping functions (SF Symbols, Segoe, Freedesktop, Material, Lucide) -- icon name strings unchanged
- Updated bundled.rs match arms, winicons.rs tests, gpui connector mapping, and showcase example
- Zero StatusLoading occurrences remain in any .rs file; 16 StatusBusy references in icons.rs

## Task Commits

Each task was committed atomically:

1. **Task 1: Rename StatusLoading to StatusBusy in core crate** - `b4c34f7` (feat)
2. **Task 2: Rename StatusLoading to StatusBusy in gpui connector** - `3ecfd29` (feat)

## Files Created/Modified
- `native-theme/src/model/icons.rs` - StatusBusy variant, ALL array, all 5 icon_name functions, all test assertions
- `native-theme/src/model/bundled.rs` - StatusBusy match arms in material_svg() and lucide_svg()
- `native-theme/src/winicons.rs` - Updated test for StatusBusy unmapped role
- `connectors/native-theme-gpui/src/icons.rs` - StatusBusy => IconName::Loader mapping
- `connectors/native-theme-gpui/examples/showcase.rs` - StatusBusy in showcase lookup

## Decisions Made
- Renamed StatusLoading to StatusBusy for semantic accuracy: a static icon cannot represent "loading" -- StatusBusy conveys "system is busy" as a static indicator while animated loading moves to loading_indicator() in Plan 01

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- Pre-existing uncommitted changes from Plan 01 (animated.rs module) caused 2 unrelated test failures in `model::animated::tests` and compilation errors in `cargo check`. These are out-of-scope for this plan and were excluded from commits. The rename itself compiles and passes all tests cleanly.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Breaking change complete: StatusBusy is the canonical variant name going forward
- Plan 01 (AnimatedIcon data model) can proceed independently
- Documentation updates deferred to Phase 32

## Self-Check: PASSED

- All 5 modified files exist on disk
- Both task commits (b4c34f7, 3ecfd29) verified in git log
- Zero StatusLoading in .rs files confirmed
- 225 tests pass, 2 pre-existing animated module failures excluded

---
*Phase: 27-animation-data-model-and-breaking-changes*
*Completed: 2026-03-18*
