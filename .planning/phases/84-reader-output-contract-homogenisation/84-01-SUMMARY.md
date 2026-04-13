---
phase: 84-reader-output-contract-homogenisation
plan: 01
subsystem: pipeline
tags: [enum, type-safety, reader-output, pipeline, overlay]

# Dependency graph
requires:
  - phase: 78-overlaysource-accessibilitypreferences-font-dpi-relocation
    provides: OverlaySource struct with reader_output field, font_dpi param
provides:
  - ReaderOutput enum with Single/Dual variants for type-safe reader contract
  - Updated run_pipeline accepting ReaderOutput instead of raw Theme
  - Updated OverlaySource storing ReaderOutput with name/icon_set/layout metadata
  - theme_to_reader_output bridge for legacy reader tuple returns
affects: [84-02, reader-migration, pipeline]

# Tech tracking
tech-stack:
  added: []
  patterns: [ReaderOutput enum dispatch, Box<ThemeMode> for large enum variants, bridge pattern for incremental migration]

key-files:
  created: []
  modified:
    - native-theme/src/lib.rs
    - native-theme/src/pipeline.rs

key-decisions:
  - "Box<ThemeMode> in ReaderOutput variants to satisfy clippy::large_enum_variant (ThemeMode is ~3KB)"
  - "theme_to_reader_output bridge in pipeline.rs for incremental reader migration (Plan 02 eliminates it)"
  - "run_pipeline expanded to 8 params with #[allow(clippy::too_many_arguments)] -- reader metadata (name, icon_set, layout) passed separately alongside ReaderOutput"
  - "overlay_tests converted to return Result<()> for zero-panic test pattern consistency"

patterns-established:
  - "ReaderOutput::Single/Dual dispatch: match on enum variant instead of checking Option<ThemeMode>.is_some()"
  - "Bridge pattern: theme_to_reader_output converts legacy (Theme, dpi, acc) tuple to ReaderOutput at call site"

requirements-completed: [READER-01]

# Metrics
duration: 11min
completed: 2026-04-13
---

# Phase 84 Plan 01: ReaderOutput Enum and Pipeline Core Summary

**Type-safe ReaderOutput enum with Single/Dual variants replaces fragile is_some() branching in run_pipeline and with_overlay**

## Performance

- **Duration:** 11 min
- **Started:** 2026-04-13T19:31:58Z
- **Completed:** 2026-04-13T19:43:22Z
- **Tasks:** 2 (combined into 1 atomic commit due to tight coupling)
- **Files modified:** 2

## Accomplishments
- Defined `ReaderOutput` enum with `Single { mode: Box<ThemeMode>, is_dark: bool }` and `Dual { light: Box<ThemeMode>, dark: Box<ThemeMode> }` variants
- Rewrote `run_pipeline` to accept `ReaderOutput` and match on variant for type-safe light/dark selection
- Updated `OverlaySource` to store `ReaderOutput` plus reader metadata (name, icon_set, layout)
- Rewrote `with_overlay` to reconstruct Theme via `ReaderOutput::to_theme()` helper
- Removed `reader_is_dark()` helper -- Single variant's `is_dark` field makes it unnecessary
- Added `theme_to_reader_output` bridge for `from_system_inner` call sites (legacy reader returns)
- Replaced 4 `reader_is_dark` tests with 4 `theme_to_reader_output` bridge tests
- All 484 lib tests and 41 integration tests pass, clippy clean

## Task Commits

Tasks 1 and 2 were committed as one atomic unit (neither compiles without the other):

1. **Task 1+2: ReaderOutput enum + pipeline rewrite** - `6d9789d` (feat)

## Files Created/Modified
- `native-theme/src/lib.rs` - ReaderOutput enum, to_theme() helper, updated OverlaySource, updated with_overlay, migrated system_theme_tests and overlay_tests
- `native-theme/src/pipeline.rs` - Updated run_pipeline signature, theme_to_reader_output bridge, updated from_system_inner, migrated dispatch_tests and pipeline_tests

## Decisions Made
- **Box<ThemeMode> everywhere:** ThemeMode is ~3KB, making ReaderOutput ~6KB for Dual without boxing. Boxing both variants brings enum size to ~24 bytes. Applied to Single.mode too since clippy flagged the 3KB vs 16B disparity.
- **Combined commit:** Tasks 1 and 2 cannot compile independently (lib.rs defines ReaderOutput, pipeline.rs consumes it, lib.rs overlay_tests call pipeline::run_pipeline). Combined into one atomic commit.
- **Bridge function:** `theme_to_reader_output` converts legacy `Theme` returns to `ReaderOutput` at each `from_system_inner` call site. This is intentionally temporary -- Plan 02 migrates readers to return `ReaderOutput` directly.
- **overlay_tests -> Result<()>:** Converted all overlay tests from using `.unwrap()` to returning `Result<()>` with `?` for consistency with the project's zero-panic rule.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Box<ThemeMode> for clippy::large_enum_variant**
- **Found during:** Task 1 (ReaderOutput enum definition)
- **Issue:** ThemeMode is ~3KB; unboxed Dual variant would be ~6KB, triggering clippy::large_enum_variant
- **Fix:** Wrapped all ThemeMode fields in Box<> (both Single.mode and Dual.light/dark)
- **Files modified:** native-theme/src/lib.rs, native-theme/src/pipeline.rs
- **Verification:** clippy -D warnings passes clean
- **Committed in:** 6d9789d

**2. [Rule 1 - Bug] #[allow(clippy::too_many_arguments)] on run_pipeline**
- **Found during:** Task 2 (run_pipeline signature expansion)
- **Issue:** Expanding from 5 to 8 params triggered clippy::too_many_arguments
- **Fix:** Added allow attribute; parameter count is intentional and will be reduced when Plan 02 eliminates the bridge
- **Files modified:** native-theme/src/pipeline.rs
- **Verification:** clippy -D warnings passes clean
- **Committed in:** 6d9789d

**3. [Rule 3 - Blocking] Combined Task 1+2 into single commit**
- **Found during:** Task 1 verification
- **Issue:** Task 1 alone doesn't compile -- pipeline.rs still references old OverlaySource shape
- **Fix:** Completed both tasks before committing as one atomic unit
- **Files modified:** both source files
- **Verification:** 484 lib + 41 integration tests pass
- **Committed in:** 6d9789d

---

**Total deviations:** 3 auto-fixed (2 bug fixes, 1 blocking)
**Impact on plan:** All auto-fixes necessary for compilation and lint compliance. No scope creep.

## Issues Encountered
- Pre-existing dead_code warning in gnome/mod.rs (build_gnome_spec_pure) surfaced by connector crate clippy checks -- not caused by this plan, out of scope.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- ReaderOutput enum and pipeline core ready for Plan 02 reader migration
- theme_to_reader_output bridge in place at all from_system_inner call sites
- Plan 02 will make KDE/GNOME/macOS/Windows readers return ReaderOutput directly, eliminating the bridge

---
*Phase: 84-reader-output-contract-homogenisation*
*Completed: 2026-04-13*
