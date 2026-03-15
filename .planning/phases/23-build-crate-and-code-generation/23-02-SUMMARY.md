---
phase: 23-build-crate-and-code-generation
plan: 02
subsystem: icons
tags: [build-crate, validation, toml, build-time-errors]

# Dependency graph
requires:
  - phase: 23-build-crate-and-code-generation
    plan: 01
    provides: "MasterConfig, MappingValue, ThemeMapping, BuildError, KNOWN_THEMES"
provides:
  - "validate_themes function for theme name validation"
  - "validate_mapping function for role/mapping consistency (VAL-01, VAL-03, VAL-04)"
  - "validate_svgs function for SVG file existence checks (VAL-02)"
  - "check_orphan_svgs function for unreferenced SVG detection (VAL-05)"
  - "validate_no_duplicate_roles function for cross-file role collision detection (VAL-06)"
affects: [23-04]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Pure validation functions returning Vec<BuildError> for composable error collection", "HashMap-based seen tracking for cross-file duplicate detection"]

key-files:
  created:
    - "native-theme-build/src/validate.rs"
  modified:
    - "native-theme-build/src/lib.rs"

key-decisions:
  - "All validation functions are pure: take data, return errors/warnings, no side effects"
  - "check_orphan_svgs returns Vec<String> warnings (not BuildErrors) since orphans are non-fatal"
  - "validate_svgs uses default_name() to resolve SVG file path for both Simple and DeAware values"

patterns-established:
  - "Pure validation pattern: functions take parsed data and return structured error vectors for composable pipeline usage"
  - "Orphan detection pattern: collect referenced stems into BTreeSet, compare against read_dir results"

requirements-completed: [VAL-01, VAL-02, VAL-03, VAL-04, VAL-05, VAL-06]

# Metrics
duration: 3min
completed: 2026-03-15
---

# Phase 23 Plan 02: TOML Validation Functions Summary

**Pure validation functions for TOML icon configs covering missing roles, unknown roles, missing defaults, missing SVGs, orphan SVGs, and duplicate roles across files**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-15T22:02:44Z
- **Completed:** 2026-03-15T22:06:09Z
- **Tasks:** 1 (TDD: RED + GREEN + fix)
- **Files modified:** 2

## Accomplishments
- Implemented 5 validation functions covering all 6 VAL requirements
- 17 new tests covering every validation scenario with dedicated tests per requirement
- All functions are pure and composable for Plan 04's validation pipeline
- SVG validation uses filesystem checks; orphan detection uses read_dir scanning

## Task Commits

Each task was committed atomically:

1. **Task 1 (RED): Failing tests for validation functions** - `1133888` (test)
2. **Task 1 (GREEN): Implement validation functions** - `c8179d9` (feat)
3. **Task 1 (fix): Restore mod validate in lib.rs** - `10a9c72` (fix)

## Files Created/Modified
- `native-theme-build/src/validate.rs` - All 5 validation functions + 17 tests
- `native-theme-build/src/lib.rs` - Added mod validate declaration

## Decisions Made
- All validation functions are pure: they take parsed data and return error/warning vectors with no side effects (no println!, no process::exit)
- check_orphan_svgs returns Vec<String> rather than Vec<BuildError> since orphan SVGs are warnings, not hard errors
- validate_svgs uses MappingValue::default_name() to resolve the expected SVG filename for both Simple and DeAware mapping entries

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Restored mod validate declaration after parallel plan overwrote lib.rs**
- **Found during:** Task 1 (post-GREEN verification)
- **Issue:** Parallel plan 23-03 modified lib.rs, replacing mod validate with mod codegen
- **Fix:** Added mod validate back alongside mod codegen
- **Files modified:** native-theme-build/src/lib.rs
- **Verification:** All 55 tests pass (16 original + 17 validation + 22 codegen)
- **Committed in:** 10a9c72

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Necessary fix due to parallel execution race condition. No scope creep.

## Issues Encountered

Parallel plan 23-03 was executing concurrently and modified lib.rs, removing the `mod validate;` declaration that this plan added. Resolved by adding both `mod codegen;` and `mod validate;` declarations.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- All 5 validation functions ready for Plan 04's validation pipeline
- Functions are pure and composable: take parsed config data, return structured errors
- check_orphan_svgs returns warnings (strings) for non-fatal reporting
- validate_svgs and check_orphan_svgs handle filesystem I/O for bundled theme validation

## Self-Check: PASSED

All artifacts verified:
- native-theme-build/src/validate.rs exists on disk
- native-theme-build/src/lib.rs contains mod validate
- Commit 1133888 found in git log
- Commit c8179d9 found in git log
- Commit 10a9c72 found in git log
- 55 tests pass via `cargo test -p native-theme-build`

---
*Phase: 23-build-crate-and-code-generation*
*Completed: 2026-03-15*
