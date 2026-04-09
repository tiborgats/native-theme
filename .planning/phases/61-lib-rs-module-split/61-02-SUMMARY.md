---
phase: 61-lib-rs-module-split
plan: 02
subsystem: refactoring
tags: [rust, module-extraction, code-organization]

# Dependency graph
requires:
  - phase: 61-01
    provides: detect.rs module with OS detection logic, test_util.rs with ENV_MUTEX
provides:
  - pipeline.rs module with theme pipeline orchestration (run_pipeline, from_system_inner, etc.)
  - icons.rs module with icon loading dispatch (load_icon, load_custom_icon, etc.)
  - Clean lib.rs root module (macro, struct, re-exports only -- no functional code)
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns: [pipeline:: delegation pattern for SystemTheme methods, crate:: imports for cross-module test access]

key-files:
  created:
    - native-theme/src/pipeline.rs
    - native-theme/src/icons.rs
  modified:
    - native-theme/src/lib.rs

key-decisions:
  - "Split system_theme_tests: 8 run_pipeline/reader_is_dark tests moved to pipeline.rs, 4 active/pick/platform_preset_name tests + 2 platform_preset_name tests stayed in lib.rs"
  - "overlay_tests helper calls pipeline::run_pipeline directly (pub(crate) access within crate)"
  - "lib.rs at 645 lines including 260 lines of tests and verbose doc comments on SystemTheme methods -- all functional code extracted"

patterns-established:
  - "pipeline:: delegation: SystemTheme::from_system() delegates to pipeline::from_system_inner()"
  - "Conditional icon imports: #[allow(unused_imports)] for cfg-gated icon function dependencies"

requirements-completed: [STRUCT-01]

# Metrics
duration: 21min
completed: 2026-04-09
---

# Phase 61 Plan 02: Pipeline and Icons Extraction Summary

**Extracted pipeline orchestration and icon dispatch from lib.rs into pipeline.rs (857 lines) and icons.rs (617 lines), completing the module split with zero test regression (674/674 tests)**

## Performance

- **Duration:** 21 min
- **Started:** 2026-04-09T12:30:55Z
- **Completed:** 2026-04-09T12:51:55Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Created pipeline.rs with all orchestration functions (run_pipeline, from_linux, from_system_inner, from_system_async_inner, platform_preset_name, diagnose_platform_support, reader_is_dark, linux_preset_for_de) plus dispatch_tests (16 tests) and pipeline_tests (12 tests)
- Created icons.rs with all icon dispatch functions (load_icon, load_icon_from_theme, load_system_icon_by_name, load_custom_icon, loading_indicator, is_freedesktop_theme_available) plus 5 icon test modules (23 tests)
- Trimmed lib.rs to root module: impl_merge! macro, module declarations, re-exports, SystemTheme struct+impl, system_theme_tests (6 tests), overlay_tests (7 tests)
- Zero test count regression: 674 tests before and after extraction
- All 5 feature combinations compile cleanly

## Task Commits

Each task was committed atomically:

1. **Task 1: Extract pipeline.rs and icons.rs from lib.rs** - `1cf534e` (refactor)
2. **Task 2: Full verification -- tests, line counts, feature combinations** - `bda4fcc` (refactor)

## Files Created/Modified
- `native-theme/src/pipeline.rs` - New module: theme pipeline orchestration with run_pipeline, from_system_inner, from_system_async_inner, dispatch_tests, pipeline_tests
- `native-theme/src/icons.rs` - New module: icon loading dispatch with load_icon, load_custom_icon, loading_indicator, and 5 icon test modules
- `native-theme/src/lib.rs` - Trimmed to root module: macro, struct, re-exports, remaining tests
- `native-theme/src/model/icons.rs` - Formatter: line-wrapping on Cinnamon match arm
- `native-theme/src/presets.rs` - Formatter: line-wrapping adjustment
- `native-theme/src/resolve/inheritance.rs` - Formatter: line-wrapping adjustment

## Decisions Made
- Split system_theme_tests between lib.rs (tests for active/pick/platform_preset_name) and pipeline.rs (tests for run_pipeline/reader_is_dark) per research recommendation
- overlay_tests stayed in lib.rs alongside SystemTheme impl, with helper calling pipeline::run_pipeline directly
- Used #[allow(unused_imports)] for icon module imports that are conditionally compiled based on feature flags
- Refactored font_dpi propagation in run_pipeline to use `.or()` idiom instead of explicit if/Some pattern

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed import paths for bundled_icon_by_name and bundled_icon_svg**
- **Found during:** Task 1 (compilation check)
- **Issue:** Plan specified importing from `crate::model::icons::` but these functions are in `crate::model::bundled::` (re-exported at `crate::model::`)
- **Fix:** Changed import to `use crate::model::{bundled_icon_by_name, bundled_icon_svg, ...}`
- **Files modified:** native-theme/src/icons.rs
- **Verification:** Compilation succeeds
- **Committed in:** 1cf534e (Task 1 commit)

**2. [Rule 3 - Blocking] Removed unused ThemeDefaults and ThemeVariant imports from pipeline.rs production code**
- **Found during:** Task 1 (compilation check)
- **Issue:** These types are only used in pipeline_tests, not in production pipeline functions
- **Fix:** Removed from production imports; tests have their own `use crate::model::{ThemeDefaults, ThemeSpec, ThemeVariant}`
- **Files modified:** native-theme/src/pipeline.rs
- **Verification:** Compilation succeeds with no warnings
- **Committed in:** 1cf534e (Task 1 commit)

**3. [Rule 3 - Blocking] Removed unused crate::SystemTheme import in pipeline_tests**
- **Found during:** Task 2 (clippy verification)
- **Issue:** pipeline_tests imported SystemTheme but only used it implicitly through run_pipeline return type
- **Fix:** Removed the unused import
- **Files modified:** native-theme/src/pipeline.rs
- **Verification:** clippy passes for native-theme-modified files
- **Committed in:** bda4fcc (Task 2 commit)

**4. [Rule 3 - Blocking] Added #[allow(unused_imports)] for conditionally-compiled icon imports**
- **Found during:** Task 2 (clippy verification)
- **Issue:** icon_name, system_icon_theme, bundled_icon_by_name, bundled_icon_svg are unused when no icon features enabled
- **Fix:** Added `#[allow(unused_imports)]` on the cfg-dependent import groups
- **Files modified:** native-theme/src/icons.rs
- **Verification:** No warnings with minimal feature set
- **Committed in:** bda4fcc (Task 2 commit)

---

**Total deviations:** 4 auto-fixed (all blocking -- import path corrections)
**Impact on plan:** All fixes necessary for compilation. No scope creep.

## Issues Encountered
- lib.rs line count is 645 (plan estimated ~250, target was <300). The difference is 260 lines of test code (system_theme_tests + overlay_tests) plus ~190 lines of detailed doc comments on SystemTheme methods (from_system has 30+ lines of documentation). All functional production code was successfully extracted -- lib.rs contains only the macro, struct, re-exports, and tests.
- Pre-existing clippy failures in kde/mod.rs (unnecessary_unwrap, manual_range_contains) and spinners.rs (unwrap_used) are out of scope -- not caused by this plan's changes.
- Pre-existing test failure (gnome::tests::build_gnome_variant_normal_contrast_no_flag) unchanged.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 61 (lib.rs Module Split) is complete: detect.rs, pipeline.rs, icons.rs, test_util.rs all extracted
- lib.rs is now a clean root module suitable for LLM-friendly navigation
- All crate::detect::, crate::pipeline::, crate::icons:: path conventions established
- Ready for Phase 62 (derive macro validation) or any subsequent phase

## Self-Check: PASSED

- native-theme/src/pipeline.rs: FOUND
- native-theme/src/icons.rs: FOUND
- Commit 1cf534e: FOUND
- Commit bda4fcc: FOUND
- Test count: 674 (verified)
- Feature combinations: 5/5 compile

---
*Phase: 61-lib-rs-module-split*
*Completed: 2026-04-09*
