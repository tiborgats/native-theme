---
phase: 61-lib-rs-module-split
plan: 01
subsystem: refactoring
tags: [rust, module-extraction, code-organization]

# Dependency graph
requires: []
provides:
  - detect.rs module with all OS detection logic (dark mode, DPI, reduced motion, desktop env)
  - test_util.rs with shared ENV_MUTEX for test serialization
  - Narrower accessor pattern (gsettings_get, xft_dpi, physical_dpi, system_font_dpi)
affects: [61-02-PLAN (pipeline.rs extraction uses detect:: paths and test_util::ENV_MUTEX)]

# Tech tracking
tech-stack:
  added: []
  patterns: [module-private-with-accessors, crate::detect:: path convention]

key-files:
  created:
    - native-theme/src/detect.rs
    - native-theme/src/test_util.rs
  modified:
    - native-theme/src/lib.rs
    - native-theme/src/gnome/mod.rs
    - native-theme/src/kde/mod.rs
    - native-theme/src/resolve/mod.rs
    - native-theme/src/resolve/inheritance.rs
    - native-theme/src/presets.rs
    - native-theme/src/model/icons.rs

key-decisions:
  - "Made run_gsettings_with_timeout, read_xft_dpi, detect_physical_dpi, detect_system_font_dpi private with narrower pub(crate) accessors"
  - "xdg_current_desktop stays pub(crate) in detect module, not re-exported at crate root"

patterns-established:
  - "Module-private + accessor pattern: private implementation functions with thin pub(crate) accessor wrappers"
  - "crate::detect:: path convention for all OS detection calls from other modules"

requirements-completed: [STRUCT-01]

# Metrics
duration: 9min
completed: 2026-04-09
---

# Phase 61 Plan 01: Detection Logic Extraction Summary

**Extracted 673 lines of OS detection logic from lib.rs into detect.rs with narrower accessor functions and zero test regression (674/674 tests)**

## Performance

- **Duration:** 9 min
- **Started:** 2026-04-09T12:18:09Z
- **Completed:** 2026-04-09T12:27:38Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments
- Created detect.rs (738 lines) with all dark mode, DPI, reduced motion, and desktop environment detection
- Created test_util.rs with shared ENV_MUTEX for cross-module test serialization
- Updated 7 caller files (gnome, kde, resolve, presets, model/icons, lib.rs) to use crate::detect:: paths
- Zero test count regression: 674 tests before and after extraction

## Task Commits

Each task was committed atomically:

1. **Task 1: Create detect.rs and test_util.rs with accessor pattern** - `8334e4a` (refactor)
2. **Task 2: Update cross-module callers and verify all tests pass** - `991d2b9` (refactor)

## Files Created/Modified
- `native-theme/src/detect.rs` - New module: LinuxDesktop enum, dark mode detection, DPI sensing, reduced motion, accessor functions, xrandr_dpi_tests, reduced_motion_tests
- `native-theme/src/test_util.rs` - New module: shared ENV_MUTEX for test env var serialization
- `native-theme/src/lib.rs` - Removed 673 lines of detection code, added mod declarations and re-exports
- `native-theme/src/gnome/mod.rs` - Updated to use detect::gsettings_get, detect::xft_dpi, detect::physical_dpi
- `native-theme/src/kde/mod.rs` - Updated to use detect::xft_dpi, detect::physical_dpi, test_util::ENV_MUTEX
- `native-theme/src/resolve/mod.rs` - Updated to use detect::system_font_dpi
- `native-theme/src/resolve/inheritance.rs` - Updated to use detect::detect_linux_de, detect::xdg_current_desktop, detect::LinuxDesktop
- `native-theme/src/presets.rs` - Updated to use detect:: paths for DE detection
- `native-theme/src/model/icons.rs` - Updated to use detect:: paths for DE detection and LinuxDesktop enum

## Decisions Made
- Made former pub(crate) functions private in detect.rs with narrower accessor wrappers -- cleaner module boundaries per CONTEXT.md locked decision
- xdg_current_desktop kept as pub(crate) in detect module rather than re-exported at crate root -- only needed internally

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Removed duplicate xrandr_dpi_tests module in detect.rs**
- **Found during:** Task 2 (compilation verification)
- **Issue:** The bash extraction script included xrandr_dpi_tests twice -- once in the bulk 179-850 range and once appended separately
- **Fix:** Removed the duplicate module definition
- **Files modified:** native-theme/src/detect.rs
- **Verification:** Compilation succeeds, test count unchanged at 674
- **Committed in:** 991d2b9 (Task 2 commit)

**2. [Rule 3 - Blocking] Updated lib.rs internal xdg_current_desktop references**
- **Found during:** Task 1 (not explicitly in plan)
- **Issue:** lib.rs production code called xdg_current_desktop() directly, but it moved to detect module and is not re-exported
- **Fix:** Updated 3 call sites in lib.rs to use detect::xdg_current_desktop()
- **Files modified:** native-theme/src/lib.rs
- **Verification:** Compilation succeeds across all feature combinations
- **Committed in:** 8334e4a (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (1 bug, 1 blocking)
**Impact on plan:** Both fixes necessary for correctness. No scope creep.

## Issues Encountered
- Pre-existing gnome test failure (build_gnome_variant_normal_contrast_no_flag) unrelated to this extraction -- documented, not addressed

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- detect.rs and test_util.rs established, ready for Plan 02 (pipeline.rs extraction)
- All crate::detect:: path conventions in place for Plan 02 callers
- test_util::ENV_MUTEX ready for dispatch_tests to reference when moved to pipeline.rs

## Self-Check: PASSED

- All created files verified to exist on disk
- All commit hashes (8334e4a, 991d2b9) verified in git log
- Test count verified: 674 (no regression)
- Compilation verified across 4 feature combinations

---
*Phase: 61-lib-rs-module-split*
*Completed: 2026-04-09*
