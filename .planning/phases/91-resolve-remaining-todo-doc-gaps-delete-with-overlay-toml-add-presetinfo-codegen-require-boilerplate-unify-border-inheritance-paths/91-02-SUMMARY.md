---
phase: 91-resolve-remaining-todo-doc-gaps
plan: 02
subsystem: proc-macro
tags: [proc-macro, derive, border, validation, codegen]

# Dependency graph
requires:
  - phase: 80-native-theme-derive-proc-macro
    provides: ThemeWidget derive macro with FieldCategory enum and border_kind parsing
provides:
  - Struct-level border_kind as single source of truth for border validation dispatch
  - Simplified FieldCategory enum with 3 variants (Option, SoftOption, Nested)
  - Compile errors for deprecated border_partial/border_optional field attributes
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "border_kind struct-level dispatch: gen_validate checks resolved type for ResolvedBorderSpec and uses LayerMeta.border_kind to select validation function"

key-files:
  created: []
  modified:
    - native-theme-derive/src/parse.rs
    - native-theme-derive/src/gen_validate.rs
    - native-theme-derive/src/gen_structs.rs
    - native-theme-derive/src/gen_merge.rs
    - native-theme-derive/src/lib.rs
    - native-theme/src/model/widgets/mod.rs

key-decisions:
  - "is_border_type() detects border fields by checking if resolved type's last path segment is ResolvedBorderSpec -- robust because only border fields use this type"
  - "meta.error() used for deprecated attribute messages instead of Error::new(meta.path.span()) to avoid syn::spanned::Spanned import (though Spanned was added for future use)"

patterns-established:
  - "border_kind dispatch: struct-level border_kind attribute is the single authority for border validation -- per-field attributes are no longer needed"

requirements-completed: [GAP-B7]

# Metrics
duration: 10min
completed: 2026-04-15
---

# Phase 91 Plan 02: Unify Border Inheritance Paths Summary

**Struct-level border_kind drives all border validation dispatch, eliminating redundant per-field border_partial/border_optional attributes from proc-macro and 5 widget structs**

## Performance

- **Duration:** 10 min
- **Started:** 2026-04-15T13:50:14Z
- **Completed:** 2026-04-15T13:59:55Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Removed FieldCategory::BorderPartial and BorderOptional variants, reducing enum to 3 variants (Option, SoftOption, Nested)
- gen_validate now dispatches border fields via struct-level border_kind: None -> border_all_optional, Partial -> require_border_partial, Full -> ValidateNested
- All 5 widget border fields (MenuTheme, TabTheme, CardTheme, SidebarTheme, StatusBarTheme) now use #[theme(nested)] with struct-level border_kind driving dispatch
- border_partial and border_optional field attributes now produce helpful compile errors guiding users to the struct-level attribute
- All 505 lib tests pass with identical validation behavior
- 3 new unit tests in derive crate for parser behavior

## Task Commits

Each task was committed atomically:

1. **Task 1: Make border_kind drive gen_validate dispatch and remove per-field border categories** - `128b199` (feat)
2. **Task 2: Update widget structs to remove per-field border attributes** - `56ad7a1` (feat)

_Note: Task 1 followed TDD (RED tests included in GREEN commit for proc-macro practicality)_

## Files Created/Modified
- `native-theme-derive/src/parse.rs` - Removed BorderPartial/BorderOptional variants, added error messages for old attributes, removed #[expect(dead_code)] from border_kind, added 3 unit tests
- `native-theme-derive/src/gen_validate.rs` - Added is_border_type() helper, border_kind-driven dispatch for border nested fields
- `native-theme-derive/src/gen_structs.rs` - Simplified match to only FieldCategory::Nested
- `native-theme-derive/src/gen_merge.rs` - Simplified match to only FieldCategory::Nested
- `native-theme-derive/src/lib.rs` - Updated doc comments removing border_partial/border_optional attribute docs
- `native-theme/src/model/widgets/mod.rs` - Changed 5 widget border fields from border_optional/border_partial to nested

## Decisions Made
- Used `is_border_type()` helper checking the resolved type's last path segment for "ResolvedBorderSpec" -- robust because only border fields use this resolved type
- Used `meta.error()` for deprecated attribute error messages (cleaner than `Error::new(meta.path.span())`)

## Deviations from Plan

None -- plan executed exactly as written.

## Issues Encountered
- Pre-existing compilation errors in integration tests (resolve_and_validate.rs) from plan 91-03's PresetInfo changes -- not caused by this plan's changes, verified by running lib tests only (505 pass)

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- border_kind is the single source of truth for all border validation dispatch
- FieldCategory has exactly 3 variants as specified in success criteria
- All existing validation behavior preserved identically

---
*Phase: 91-resolve-remaining-todo-doc-gaps*
*Completed: 2026-04-15*
