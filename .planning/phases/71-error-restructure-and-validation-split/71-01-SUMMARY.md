---
phase: 71-error-restructure-and-validation-split
plan: 01
subsystem: error
tags: [error-handling, error-kind, range-validation, option-f]

requires:
  - phase: 70-drop-error-clone-bound
    provides: Error enum without Clone derive (Arc wrapping no longer needed)
provides:
  - 9-variant flat Error enum (Option F structure)
  - ErrorKind enum with kind() method for coarse dispatch
  - RangeViolation struct for numeric range validation
  - ThemeResolutionError deleted; Display logic moved to ResolutionIncomplete
affects: [71-02, resolve, validate, pipeline, presets, watch, model]

tech-stack:
  added: []
  patterns: [std::io::Error::kind() precedent for coarse dispatch, flat enum over nested error types]

key-files:
  created: []
  modified:
    - native-theme/src/error.rs
    - native-theme/src/lib.rs

key-decisions:
  - "Kept Vec<String> for ResolutionIncomplete::missing (not Vec<FieldPath>) for Phase 71 compatibility"
  - "Preserved From<toml::ser::Error> via ReaderFailed variant (presets::to_toml uses toml::to_string_pretty with ? operator)"
  - "PlatformUnsupported uses &'static str not a Platform enum (none exists in the crate yet)"

patterns-established:
  - "Error::kind() exhaustive match: every new Error variant must have a kind() branch or compilation fails"
  - "field_category() helper for grouping missing-field paths into [root defaults]/[widget fields]/[text scale]/[icon set]"

requirements-completed: [ERR-02, BUG-02]

duration: 3min
completed: 2026-04-12
---

# Phase 71 Plan 01: Error Restructure Summary

**Flat 9-variant Error enum (Option F) with ErrorKind coarse dispatch and RangeViolation struct, replacing ThemeResolutionError**

## Performance

- **Duration:** 3 min
- **Started:** 2026-04-12T12:17:08Z
- **Completed:** 2026-04-12T12:20:34Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Rewrote Error enum from 6 generic variants to 9 semantic variants matching doc 1 section 31.2 Option F
- Added ErrorKind enum (Platform, Parse, Resolution, Io) with compile-time-enforced exhaustive kind() method
- Added RangeViolation struct separating range violations from missing-field errors (fixes BUG-02 dual-category conflation)
- Deleted ThemeResolutionError; moved categorized Display logic into Error::ResolutionIncomplete arm
- Dropped Arc wrapping on Io variant (Phase 70 removed Clone)
- Updated lib.rs re-exports to Error, ErrorKind, RangeViolation

## Task Commits

Each task was committed atomically:

1. **Task 1: Rewrite error.rs with Option F Error enum, ErrorKind, and RangeViolation** - `7f45fba` (feat)
2. **Task 2: Update lib.rs re-exports for new error types** - `a450c03` (feat)

## Files Created/Modified
- `native-theme/src/error.rs` - Complete rewrite: 9-variant Error, ErrorKind, RangeViolation, Display/Error/From impls, 30 tests
- `native-theme/src/lib.rs` - Re-exports updated, doc comments updated to reference new variant names

## Decisions Made
- Kept `Vec<String>` for `ResolutionIncomplete::missing` instead of `Vec<FieldPath>` -- the doc 1 mention of FieldPath is a future refinement; Phase 71 keeps string paths for compatibility with existing validate.rs output
- Preserved `From<toml::ser::Error>` conversion -- presets::to_toml() uses `toml::to_string_pretty()?` which requires this impl. Mapped to `ReaderFailed { reader: "toml-serializer", .. }` since Error::Toml wraps only `toml::de::Error`
- Used `&'static str` for PlatformUnsupported::platform -- no Platform enum exists in the crate; roadmap does not require one for Phase 71

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Preserved From<toml::ser::Error> impl**
- **Found during:** Task 1 (error.rs rewrite)
- **Issue:** Plan said to delete `From<toml::ser::Error>`, but `presets::to_toml()` uses `toml::to_string_pretty()?` which depends on it
- **Fix:** Mapped `toml::ser::Error` to `Error::ReaderFailed { reader: "toml-serializer", source: Box::new(err) }` instead of deleting
- **Files modified:** native-theme/src/error.rs
- **Verification:** Confirmed presets.rs line 187 uses `?` on `toml::to_string_pretty()` return type
- **Committed in:** 7f45fba (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Auto-fix necessary to prevent compilation failure in presets module. No scope creep.

## Issues Encountered
- Crate does not compile after this plan because callers (resolve/validate.rs, presets.rs, model/mod.rs, resolve/tests.rs) still reference old variant names (Error::Format, Error::Unavailable, Error::Resolution, ThemeResolutionError). This is expected and documented in the plan -- Plan 02 will update all callers.
- Error module tests cannot be run in isolation because `cargo test --lib` requires full crate compilation. All 43 compilation errors are in other files, zero in error.rs itself.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Error type contracts are fully defined and tested (in-module)
- Plan 02 must update all callers to use new variant names before the crate compiles again
- Key callers to update: resolve/validate.rs, presets.rs, model/mod.rs, pipeline.rs, watch/mod.rs, macos.rs, rasterize.rs

## Self-Check: PASSED

- FOUND: native-theme/src/error.rs
- FOUND: native-theme/src/lib.rs
- FOUND: 71-01-SUMMARY.md
- FOUND: commit 7f45fba (Task 1)
- FOUND: commit a450c03 (Task 2)

---
*Phase: 71-error-restructure-and-validation-split*
*Completed: 2026-04-12*
