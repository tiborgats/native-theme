---
phase: 62-validate-codegen
plan: 03
subsystem: codegen
tags: [refactor, validate, validate_helpers, check_ranges, module-split]

# Dependency graph
requires:
  - phase: 62-validate-codegen plan 02
    provides: validate.rs with generated validate_widget() calls for all 25 widgets, 1266 lines
provides:
  - validate_helpers.rs module with all helper functions, range-check utilities, ValidateNested trait, and check_defaults_ranges()
  - check_ranges() method on 24 Resolved* widget structs
  - validate.rs under 500 lines (490), focused on orchestration
affects: [future-widget-additions, validate-maintenance]

# Tech tracking
tech-stack:
  added: []
  patterns: [per-widget check_ranges() methods co-located with struct definitions, validate_helpers module for shared validation utilities]

key-files:
  created:
    - native-theme/src/resolve/validate_helpers.rs
  modified:
    - native-theme/src/resolve/validate.rs
    - native-theme/src/resolve/mod.rs
    - native-theme/src/model/widgets/mod.rs

key-decisions:
  - "Construct defaults/text_scale structs before range checks to enable check_defaults_ranges() and compact final construction"
  - "check_ranges() methods use format!(\"{prefix}.field\") to match existing error path strings exactly"
  - "validate_helpers module is pub(crate) for access from both validate.rs and macro-generated code in widgets/mod.rs"

patterns-established:
  - "Per-widget range checks: each Resolved* struct owns its check_ranges() method next to its definition"
  - "validate_helpers: shared validation utilities (require, require_font, check_*, ValidateNested) in dedicated module"

requirements-completed: [STRUCT-04]

# Metrics
duration: 10min
completed: 2026-04-09
---

# Phase 62 Plan 03: Gap Closure -- validate.rs Under 500 Lines Summary

**Extracted helpers and range checks from validate.rs (1317 to 490 lines) into validate_helpers.rs module and per-widget check_ranges() methods**

## Performance

- **Duration:** 10 min
- **Started:** 2026-04-09T15:23:05Z
- **Completed:** 2026-04-09T15:33:10Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Created validate_helpers.rs (490 lines) with all helper functions, range-check utilities, ValidateNested trait and impls, and check_defaults_ranges() function
- Added check_ranges() methods to 24 Resolved* widget structs in widgets/mod.rs, each owning its range validation logic
- Reduced validate.rs from 1,317 to 490 lines (63% reduction, under 500-line target)
- Restructured defaults/text_scale construction to happen before range checks, enabling both check_defaults_ranges() call and compact final ResolvedThemeVariant construction
- Updated macro-generated code to reference validate_helpers instead of validate module

## Task Commits

Each task was committed atomically:

1. **Task 1: Extract helpers and defaults range checks to validate_helpers.rs** - `77aff6e` (refactor)
2. **Task 2: Move per-widget range checks to check_ranges() methods** - `f6f567c` (refactor)

## Files Created/Modified
- `native-theme/src/resolve/validate_helpers.rs` - New module with all helper functions (require, require_font, require_font_opt, require_text_scale_entry, require_border, border_all_optional, require_border_partial), range-check utilities (check_non_negative, check_positive, check_range_f32, check_range_u16, check_min_max), ValidateNested trait and FontSpec/BorderSpec impls, check_defaults_ranges() function, and DEFAULT_FONT_DPI constant
- `native-theme/src/resolve/validate.rs` - Reduced from 1,317 to 490 lines; now contains only orchestration (defaults extraction, widget dispatch, check_ranges calls, construction)
- `native-theme/src/resolve/mod.rs` - Added `pub(crate) mod validate_helpers;`
- `native-theme/src/model/widgets/mod.rs` - Updated macro template to reference validate_helpers; added check_ranges() impl blocks for 24 Resolved* widget structs

## Decisions Made
- Constructed defaults/text_scale structs before range checks rather than duplicating construction -- saves ~50 lines and eliminates the double-build pattern the plan noted as option (a)
- Used `format!("{prefix}.field_name")` in check_ranges() methods to produce identical error path strings as the original inline checks
- Made validate_helpers pub(crate) (same visibility as validate) since macro-generated code in widgets/mod.rs needs direct access to require, ValidateNested, require_border_partial, and border_all_optional

## Deviations from Plan

None -- plan executed exactly as written.

## Issues Encountered
- Pre-existing test failure `gnome::tests::build_gnome_variant_normal_contrast_no_flag` (assertion on high_contrast field) -- confirmed pre-existing on clean main, documented in Plans 01 and 02

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 62 (validate-codegen) gap closure complete: validate.rs is now 490 lines (target was <500)
- Adding a new widget: define_widget_pair! generates validate_widget() automatically; only a check_ranges() impl block needs hand-writing
- validate_helpers.rs provides shared utilities for any future validation needs
- Ready for subsequent phases (63+) which are independent of 62

---
*Phase: 62-validate-codegen*
*Completed: 2026-04-09*
