---
phase: 85-data-model-method-and-doc-cleanup
plan: 01
subsystem: api
tags: [rustdoc, doc-hidden, api-cleanup, theme-model]

# Dependency graph
requires: []
provides:
  - "#[doc(hidden)] on ThemeMode resolve/validate intermediate methods"
  - "from_toml_with_base removed from codebase"
  - "Error hint updated to reference Theme::preset() + Theme::merge()"
affects: [86-validation-and-lint-codegen-polish, 88-diagnostic-and-preset-polish-sweep]

# Tech tracking
tech-stack:
  added: []
  patterns: ["#[doc(hidden)] pub for internal-but-tested methods"]

key-files:
  created: []
  modified:
    - native-theme/src/resolve/mod.rs
    - native-theme/src/resolve/validate.rs
    - native-theme/src/model/mod.rs
    - native-theme/src/error.rs
    - native-theme/tests/resolve_and_validate.rs

key-decisions:
  - "Kept all doc comments on hidden methods intact for crate-developer readability"

patterns-established:
  - "#[doc(hidden)] pub pattern: hide from rustdoc while preserving integration test access"

requirements-completed: [MODEL-04, MODEL-05]

# Metrics
duration: 4min
completed: 2026-04-13
---

# Phase 85 Plan 01: API Doc Cleanup Summary

**Hide ThemeMode resolve/validate intermediates from rustdoc via #[doc(hidden)]; remove from_toml_with_base convenience method and update error hints**

## Performance

- **Duration:** 4 min
- **Started:** 2026-04-13T20:48:16Z
- **Completed:** 2026-04-13T20:52:17Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- Five ThemeMode methods (resolve, resolve_platform_defaults, resolve_all, validate, validate_with_dpi) hidden from rustdoc; into_resolved() remains the sole visible API
- from_toml_with_base method and its 3 unit tests removed from model/mod.rs
- Error hint for ResolutionIncomplete now directs users to Theme::preset(name) + Theme::merge()
- Integration tests rewritten to use preset+merge pattern instead of from_toml_with_base

## Task Commits

Each task was committed atomically:

1. **Task 1: Add #[doc(hidden)] to ThemeMode resolve/validate intermediates** - `70dc750` (feat)
2. **Task 2: Remove from_toml_with_base and update error hint** - `5809831` (feat)

## Files Created/Modified
- `native-theme/src/resolve/mod.rs` - Added #[doc(hidden)] to resolve(), resolve_platform_defaults(), resolve_all()
- `native-theme/src/resolve/validate.rs` - Added #[doc(hidden)] to validate(), validate_with_dpi()
- `native-theme/src/model/mod.rs` - Removed from_toml_with_base method and 3 unit tests
- `native-theme/src/error.rs` - Updated hint to reference preset()+merge(); updated test assertion
- `native-theme/tests/resolve_and_validate.rs` - Rewrote 2 tests to use preset+merge; updated module doc

## Decisions Made
- Kept all doc comments on hidden methods intact -- useful for crate developers reading source even though suppressed from rustdoc

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- Pre-existing doctest failure in font.rs (FontSize::to_logical_px) from uncommitted changes in unrelated files. Out of scope, logged as known pre-existing issue.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Plan 02 (NAME-02, NAME-03) can proceed independently
- Rustdoc API surface is now cleaner for all downstream documentation work

---
*Phase: 85-data-model-method-and-doc-cleanup*
*Completed: 2026-04-13*
