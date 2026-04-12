---
phase: 71-error-restructure-and-validation-split
plan: 02
subsystem: error
tags: [error-handling, validation, range-violation, caller-migration]

requires:
  - phase: 71-error-restructure-and-validation-split
    plan: 01
    provides: 9-variant flat Error enum with ErrorKind, RangeViolation struct
provides:
  - Two-phase validation split (missing fields short-circuit before range checks)
  - All Error construction sites migrated to new variant names
  - Structured RangeViolation data in check_ranges methods
  - BUG-01 and BUG-02 fixed with unit test proof
affects: [72, resolve, validate, pipeline, presets, watch, model, connectors]

tech-stack:
  added: []
  patterns: [two-vec validation split with short-circuit, structured RangeViolation over string errors]

key-files:
  created: []
  modified:
    - native-theme/src/resolve/validate.rs
    - native-theme/src/resolve/validate_helpers.rs
    - native-theme/src/model/widgets/mod.rs
    - native-theme/src/resolve/tests.rs
    - native-theme/src/pipeline.rs
    - native-theme/src/watch/mod.rs
    - native-theme/src/watch/windows.rs
    - native-theme/src/watch/macos.rs
    - native-theme/src/watch/kde.rs
    - native-theme/src/windows.rs
    - native-theme/src/presets.rs
    - native-theme/src/macos.rs
    - native-theme/src/rasterize.rs
    - native-theme/src/model/mod.rs
    - native-theme/src/kde/mod.rs
    - native-theme/tests/resolve_and_validate.rs
    - connectors/native-theme-gpui/src/lib.rs
    - connectors/native-theme-iced/src/lib.rs

key-decisions:
  - "check_positive uses f64::MIN_POSITIVE for min bound in RangeViolation (mathematically correct smallest positive value)"
  - "check_min_max uses min_name for path and max_val for max bound in RangeViolation (the violated constraint is min <= max)"
  - "Connector crates (gpui, iced) migrated to Error::ReaderFailed as part of Rule 3 auto-fix"

patterns-established:
  - "Two-vec validation: missing Vec<String> short-circuits before range_errors Vec<RangeViolation>"
  - "Range checks always produce structured RangeViolation with path, value, min, max"
  - "let-else with return for test error variant matching (avoids panic! in test code)"

requirements-completed: [BUG-01, BUG-02, ERR-02]

duration: 14min
completed: 2026-04-12
---

# Phase 71 Plan 02: Caller Migration and Validation Split Summary

**Two-phase validate() with missing-field short-circuit before range checks, plus full caller migration from old Error variants to Option F**

## Performance

- **Duration:** 14 min
- **Started:** 2026-04-12T12:23:27Z
- **Completed:** 2026-04-12T12:38:01Z
- **Tasks:** 2
- **Files modified:** 18

## Accomplishments
- Restructured validate() into two phases: missing-field collection short-circuits before check_ranges ever runs (BUG-01 fix)
- Converted all 24 check_ranges methods and 6 range-check helpers to produce structured RangeViolation structs instead of strings (BUG-02 fix)
- Migrated every Error construction site across 15 source files and 2 connector crates to new Option F variant names (ERR-02 complete)
- Added 2 new tests proving BUG-01 (short-circuit prevents cross-pollination) and BUG-02 (separate error categories)
- Updated 17 existing test assertions from Error::Resolution to ResolutionIncomplete/ResolutionInvalid
- Crate compiles cleanly with 568 tests passing and zero clippy warnings

## Task Commits

Each task was committed atomically:

1. **Task 1: TDD RED -- failing tests for validation split** - `3185309` (test)
2. **Task 1: GREEN -- restructure validate() two-vec split** - `83e6dc2` (feat)
3. **Task 2: Migrate all remaining callers to new Error variants** - `73d1c68` (feat)
4. **Task 2: Fix connector crates and apply formatting** - `dacc138` (fix)

## Files Created/Modified
- `native-theme/src/resolve/validate.rs` - Two-phase validation: missing short-circuit before range checks
- `native-theme/src/resolve/validate_helpers.rs` - Range-check helpers produce RangeViolation structs
- `native-theme/src/model/widgets/mod.rs` - 24 check_ranges methods take Vec<RangeViolation>
- `native-theme/src/resolve/tests.rs` - 17 test migrations + 2 new BUG-01/BUG-02 proofs
- `native-theme/src/pipeline.rs` - Unsupported -> FeatureDisabled/PlatformUnsupported
- `native-theme/src/watch/mod.rs` - Unsupported -> WatchUnavailable/FeatureDisabled/PlatformUnsupported
- `native-theme/src/watch/windows.rs` - Unavailable -> ReaderFailed
- `native-theme/src/watch/macos.rs` - Unavailable -> ReaderFailed
- `native-theme/src/watch/kde.rs` - Unavailable -> ReaderFailed
- `native-theme/src/windows.rs` - Unavailable -> ReaderFailed
- `native-theme/src/presets.rs` - Unavailable -> UnknownPreset, Format -> ReaderFailed
- `native-theme/src/macos.rs` - Unavailable -> ReaderFailed
- `native-theme/src/rasterize.rs` - Format -> ReaderFailed
- `native-theme/src/model/mod.rs` - Format -> Toml, Unavailable -> UnknownPreset
- `native-theme/src/kde/mod.rs` - Format -> ReaderFailed, Unavailable -> ReaderFailed
- `native-theme/tests/resolve_and_validate.rs` - Unavailable -> UnknownPreset
- `connectors/native-theme-gpui/src/lib.rs` - Format -> ReaderFailed
- `connectors/native-theme-iced/src/lib.rs` - Format -> ReaderFailed

## Decisions Made
- check_positive uses `f64::MIN_POSITIVE` for min bound in RangeViolation -- this is the mathematically correct smallest positive f64 value
- check_min_max uses `min_name` for path and `max_val` for max bound -- the RangeViolation reports the min field that exceeds its corresponding max
- Connector crates migrated as part of this plan despite not being in the plan file list, because pre-release-check.sh caught the breakage

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Migrated resolve/tests.rs in Task 1 instead of Task 2**
- **Found during:** Task 1 (validate restructure)
- **Issue:** The crate cannot compile without fixing ALL Error::Resolution references, but resolve/tests.rs migration was scoped to Task 2
- **Fix:** Migrated all 17 Error::Resolution matches in tests.rs as part of Task 1 to enable compilation and test verification
- **Files modified:** native-theme/src/resolve/tests.rs
- **Verification:** cargo test passes after Task 1 + Task 2 combined
- **Committed in:** 83e6dc2 (Task 1 GREEN commit)

**2. [Rule 3 - Blocking] Migrated connector crates (gpui, iced)**
- **Found during:** Task 2 verification (pre-release-check.sh)
- **Issue:** Connector crates reference Error::Format which no longer exists, causing pre-release check failure
- **Fix:** Migrated connectors/native-theme-gpui/src/lib.rs and connectors/native-theme-iced/src/lib.rs to Error::ReaderFailed
- **Files modified:** connectors/native-theme-gpui/src/lib.rs, connectors/native-theme-iced/src/lib.rs
- **Verification:** pre-release-check.sh passes
- **Committed in:** dacc138

**3. [Rule 3 - Blocking] Migrated kde/mod.rs and watch/kde.rs**
- **Found during:** Task 2 (caller migration)
- **Issue:** These files were not in the plan's caller migration map but contained Error::Format and Error::Unavailable references
- **Fix:** Migrated Error::Format -> ReaderFailed and Error::Unavailable -> ReaderFailed
- **Files modified:** native-theme/src/kde/mod.rs, native-theme/src/watch/kde.rs
- **Verification:** cargo clippy passes with zero warnings
- **Committed in:** 73d1c68

---

**Total deviations:** 3 auto-fixed (3 blocking)
**Impact on plan:** All auto-fixes necessary for compilation. No scope creep -- just additional callers the plan's migration map missed.

## Issues Encountered
None beyond the deviations documented above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Error enum restructure complete: all 9 variants in use, all callers migrated
- BUG-01 fixed: validate() short-circuits on missing before range checks
- BUG-02 fixed: missing fields and range violations are separate error variants
- ERR-02 complete: no old variant names remain in source
- Phase 71 complete -- ready for Phase 72 (CLEAN-02: ENV_MUTEX test simplification)

## Self-Check: PASSED

- FOUND: native-theme/src/resolve/validate.rs
- FOUND: native-theme/src/resolve/validate_helpers.rs
- FOUND: native-theme/src/model/widgets/mod.rs
- FOUND: native-theme/src/resolve/tests.rs
- FOUND: native-theme/src/pipeline.rs
- FOUND: native-theme/src/presets.rs
- FOUND: native-theme/src/model/mod.rs
- FOUND: connectors/native-theme-gpui/src/lib.rs
- FOUND: connectors/native-theme-iced/src/lib.rs
- FOUND: commit 3185309 (TDD RED)
- FOUND: commit 83e6dc2 (Task 1 GREEN)
- FOUND: commit 73d1c68 (Task 2)
- FOUND: commit dacc138 (Task 2 connectors + fmt)

---
*Phase: 71-error-restructure-and-validation-split*
*Completed: 2026-04-12*
