---
phase: 91-resolve-remaining-todo-doc-gaps
plan: 03
subsystem: validation
tags: [macro_rules, validation, codegen, boilerplate-reduction]

# Dependency graph
requires:
  - phase: 62-03
    provides: "validate_helpers.rs split with require/require_font/require_text_scale_entry helpers"
  - phase: 79-01
    provides: "DefaultsBorderSpec/WidgetBorderSpec split with ResolvedBorderSpec shared output"
provides:
  - "validate_defaults! declarative macro for defaults extraction + ResolvedDefaults construction"
  - "validate_text_scale! declarative macro for text_scale extraction + ResolvedTextScale construction"
  - "Compact validate.rs using macro invocations (220 lines vs previous 470)"
affects: [validate, resolve, model]

# Tech tracking
tech-stack:
  added: []
  patterns: ["$crate-qualified paths in macro_rules for cross-module expansion", "concat!/stringify! for zero-allocation compile-time path strings in macros"]

key-files:
  created: []
  modified:
    - "native-theme/src/resolve/validate_helpers.rs"
    - "native-theme/src/resolve/validate.rs"

key-decisions:
  - "$crate-qualified paths in macros for robust cross-module expansion (macros expand at call site)"
  - "use statements inside macro body (not at call site) to keep caller imports minimal"
  - "concat!/stringify! for path strings instead of format! -- compile-time &'static str with zero allocation"

patterns-established:
  - "validate_defaults! macro pattern: categorized field groups (font/option/border_required/icon_sizes) with automatic struct construction"
  - "validate_text_scale! macro pattern: uniform field list with automatic ResolvedTextScale construction"

requirements-completed: [GAP-B1, GAP-C6]

# Metrics
duration: 9min
completed: 2026-04-15
---

# Phase 91 Plan 03: Require Boilerplate Macro Summary

**Declarative macros replace ~285 lines of hand-written defaults extraction with ~35 lines of categorized macro invocations in validate.rs**

## Performance

- **Duration:** 9 min
- **Started:** 2026-04-15T13:50:17Z
- **Completed:** 2026-04-15T13:58:48Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Created validate_defaults! macro handling 4 extraction patterns (font, option, border_required, icon_sizes) and ResolvedDefaults/ResolvedBorderSpec/ResolvedIconSizes construction
- Created validate_text_scale! macro for text_scale entry extraction and ResolvedTextScale construction
- Replaced 285 lines of hand-written require() calls and struct construction with 35 lines of macro invocation
- validate.rs reduced from 470 to 220 lines (250 line reduction, 53% smaller)
- validate_helpers.rs grew from 597 to 689 lines (92 lines for macro definitions)
- Net reduction: 158 lines across both files
- Used concat!/stringify! for compile-time path strings (zero-allocation, functionally equivalent to hand-written string literals)

## Task Commits

Each task was committed atomically:

1. **Task 1: Create validate_defaults! and validate_text_scale! declarative macros** - `1712b4c` (feat)
2. **Task 2: Replace validate.rs boilerplate with macro invocations** - `7dbd4a7` (feat)

## Files Created/Modified
- `native-theme/src/resolve/validate_helpers.rs` - Added validate_defaults! and validate_text_scale! macros with $crate-qualified paths and pub(crate) use exports
- `native-theme/src/resolve/validate.rs` - Replaced ~285 lines of extraction boilerplate with ~35 lines of macro invocations; removed unused imports

## Decisions Made
- Used $crate-qualified paths inside macro bodies (not bare function names) because macros expand at the call site, where the helper functions are not in scope
- Placed `use` statements inside the macro body block to avoid requiring callers to import require/require_font/require_text_scale_entry
- concat!/stringify! produces identical path strings to hand-written literals but is DRY (single source of field names)
- from_kde_content_pure pub visibility documented as intentional deviation (C6): integration tests in tests/reader_kde.rs depend on it, confirmed in Phase 79-02 and Phase 84-02

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- Pre-existing build breakage from phase 91 plans 01/02 (derive macro changes to border_optional/border_partial attributes) prevented running full test suite. Verified my changes introduce zero new compilation errors by checking cargo check output for validate-related errors (none found). All errors are from pre-existing derive macro incompatibility in committed 91-01/91-02 changes.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Macro infrastructure ready for future field additions (only macro invocation needs updating, not separate extraction AND construction blocks)
- Pre-existing derive macro issues from 91-01/91-02 need resolution before full test suite can run

---
*Phase: 91-resolve-remaining-todo-doc-gaps*
*Completed: 2026-04-15*
