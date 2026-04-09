---
phase: 62-validate-codegen
plan: 02
subsystem: codegen
tags: [macro_rules, validate, validate_widget, boilerplate-elimination]

# Dependency graph
requires:
  - phase: 62-validate-codegen plan 01
    provides: ValidateNested trait, validate_widget() codegen in define_widget_pair!, border_partial/border_optional categories
provides:
  - Compact validate.rs using generated validate_widget() calls for all 25 widgets
  - Hand-written defaults/text_scale extraction and range checks preserved
  - 5 widgets using correct border categories (border_partial/border_optional)
affects: [validate-codegen-complete, future-widget-additions]

# Tech tracking
tech-stack:
  added: []
  patterns: [validate_widget() dispatch replaces hand-written per-widget extraction, impl_merge! accepts repeated optional_nested blocks]

key-files:
  created: []
  modified:
    - native-theme/src/resolve/validate.rs
    - native-theme/src/model/widgets/mod.rs
    - native-theme/src/lib.rs

key-decisions:
  - "impl_merge! updated from optional $(...)? to $(...)* to support widgets with both optional_nested and border_partial/border_optional blocks"
  - "validate.rs reduced from 2240 to 1266 lines (974 lines removed); 500-line target not achievable with hand-written defaults+range checks but extraction+construction eliminated ~1076 lines"
  - "link.underline path string corrected to link.underline_enabled (matches actual Rust field name)"

patterns-established:
  - "Adding a new widget: define_widget_pair! generates validate_widget() automatically; only range checks need hand-writing in validate.rs"

requirements-completed: []

# Metrics
duration: 10min
completed: 2026-04-09
---

# Phase 62 Plan 02: Validate Widget Extraction Rollout Summary

**All 25 per-variant widgets switched to generated validate_widget() calls, eliminating 974 lines of hand-written extraction and construction boilerplate**

## Performance

- **Duration:** 10 min
- **Started:** 2026-04-09T14:13:21Z
- **Completed:** 2026-04-09T14:23:55Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Replaced ~764 lines of hand-written per-widget field extraction with 25 validate_widget() calls (~30 lines)
- Replaced ~312 lines of hand-written ResolvedThemeVariant construction with compact struct shorthand (~30 lines)
- Updated 5 widget definitions to use correct border categories: sidebar/status_bar (border_partial), menu/tab/card (border_optional)
- Updated all range checks to use resolved struct field access (e.g., `button.min_width` instead of `button_min_width`)
- validate.rs reduced from 2240 to 1266 lines (43% reduction)

## Task Commits

Each task was committed atomically:

1. **Task 1: Update 5 widget definitions with border_partial/border_optional categories** - `6376203` (refactor)
2. **Task 2: Rewrite validate.rs to use validate_widget() calls** - `2a1bd34` (refactor)

## Files Created/Modified
- `native-theme/src/resolve/validate.rs` - Replaced per-widget extraction with validate_widget() calls, updated range checks to use struct fields, compact construction block
- `native-theme/src/model/widgets/mod.rs` - 5 widgets moved to border_partial/border_optional categories, removed dead_code allow comment (now actively used with correct annotation)
- `native-theme/src/lib.rs` - impl_merge! updated to accept repeated optional_nested blocks via `$()*` instead of `$()?`

## Decisions Made
- Updated impl_merge! macro from `$(optional_nested { ... })?` (single optional) to `$(optional_nested { ... })*` (zero or more repeated) to handle widgets with both optional_nested and border_partial/border_optional categories emitting multiple optional_nested blocks
- The 500-line target was not achievable because hand-written defaults extraction (~260 lines), text_scale extraction (~25 lines), and range checks (~400 lines) account for ~685 lines alone. The actual boilerplate sections (extraction + construction) were reduced from ~1076 lines to ~60 lines.
- Path string `"link.underline"` corrected to `"link.underline_enabled"` via macro stringify! -- matches actual Rust field name

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated impl_merge! to accept repeated optional_nested blocks**
- **Found during:** Task 1 (cargo check after border category changes)
- **Issue:** define_widget_pair! emits up to 3 `optional_nested` blocks to impl_merge! (for on, bp, bo fields), but impl_merge! only accepted one via `$()?`
- **Fix:** Changed `$(optional_nested { ... })?` to `$(optional_nested { ... })*` in impl_merge! macro definition (both in merge() and is_empty() expansions)
- **Files modified:** native-theme/src/lib.rs
- **Verification:** cargo check passes for all widget definitions including those with multiple nested block types
- **Committed in:** 6376203 (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (blocking)
**Impact on plan:** Fix necessary for compilation after widget category changes. No scope creep.

## Issues Encountered
- Pre-existing test failure `gnome::tests::build_gnome_variant_normal_contrast_no_flag` (assertion on high_contrast field) -- same as documented in Plan 01, confirmed pre-existing on main
- Pre-existing clippy dead_code warning for `gsettings_get` in detect.rs -- unrelated to changes, confirmed pre-existing

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 62 (validate-codegen) is now complete
- validate.rs is significantly reduced and maintainable; adding a new widget requires only adding range checks (extraction is auto-generated by define_widget_pair!)
- Ready for subsequent phases (63+) which are independent of 62

---
*Phase: 62-validate-codegen*
*Completed: 2026-04-09*
