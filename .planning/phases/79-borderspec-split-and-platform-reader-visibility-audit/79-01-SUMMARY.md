---
phase: 79-borderspec-split-and-platform-reader-visibility-audit
plan: 01
subsystem: model
tags: [rust, serde, type-safety, border, theme-resolution]

# Dependency graph
requires:
  - phase: 78-overlaysource-accessibilitypreferences-font-dpi-relocation
    provides: stable model types, AccessibilityPreferences on SystemTheme
provides:
  - DefaultsBorderSpec (6 fields, no padding) for ThemeDefaults
  - WidgetBorderSpec (6 fields, no corner_radius_lg/opacity) for per-widget borders
  - ResolvedBorderSpec unchanged (8 fields, unified output)
  - FIELD_NAMES constants per border type for TOML linting
affects: [80-native-theme-derive-proc-macro, validate, inheritance, presets]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Split spec types enforce field correctness at compile time (defaults vs widget level)"
    - "Resolved output type remains unified even when input types are split"

key-files:
  created: []
  modified:
    - native-theme/src/model/border.rs
    - native-theme/src/model/defaults.rs
    - native-theme/src/model/mod.rs
    - native-theme/src/model/widgets/mod.rs
    - native-theme/src/resolve/validate_helpers.rs
    - native-theme/src/resolve/validate.rs
    - native-theme/src/resolve/inheritance.rs
    - native-theme/src/macos.rs
    - native-theme/src/lib.rs
    - native-theme/src/resolve/tests.rs
    - native-theme/tests/proptest_roundtrip.rs
    - native-theme/tests/merge_behavior.rs

key-decisions:
  - "WidgetBorderSpec sets corner_radius_lg=0.0 and opacity=0.0 in resolved output (defaults-only fields)"
  - "D2 padding-derives-from-presence rule removed -- DefaultsBorderSpec has no padding fields"
  - "ResolvedBorderSpec padding_horizontal/padding_vertical set to 0.0 for defaults-level border"
  - "Proptest strategies split into arb_defaults_border_spec and arb_widget_border_spec"

patterns-established:
  - "Type-level enforcement: split spec types prevent silent field discard across theme levels"

requirements-completed: [BORDER-01]

# Metrics
duration: 16min
completed: 2026-04-13
---

# Phase 79 Plan 01: BorderSpec Split Summary

**Split BorderSpec into DefaultsBorderSpec (6 fields, no padding) and WidgetBorderSpec (6 fields, no corner_radius_lg/opacity) with compile-time field enforcement**

## Performance

- **Duration:** 16 min
- **Started:** 2026-04-13T08:11:46Z
- **Completed:** 2026-04-13T08:28:16Z
- **Tasks:** 2
- **Files modified:** 14

## Accomplishments
- Split unified BorderSpec into two purpose-specific types that enforce correct field sets at the defaults vs widget level
- DefaultsBorderSpec: color, corner_radius, corner_radius_lg, line_width, opacity, shadow_enabled (no padding)
- WidgetBorderSpec: color, corner_radius, line_width, shadow_enabled, padding_horizontal, padding_vertical (no corner_radius_lg, no opacity)
- ResolvedBorderSpec remains unchanged with all 8 fields as the unified resolved output
- All 20 presets parse and validate without changes (TOML field names match new types naturally)
- Removed D2 padding-derives-from-presence inheritance rule (no longer needed)
- Updated proptest strategies, integration tests, and all consumer code

## Task Commits

Each task was committed atomically:

1. **Task 1: Split BorderSpec into DefaultsBorderSpec and WidgetBorderSpec** - `1f00f51` (feat)
2. **Task 2: Update all TOML presets and verify preset validation** - `73785ed` (chore)

## Files Created/Modified
- `native-theme/src/model/border.rs` - New DefaultsBorderSpec and WidgetBorderSpec type definitions
- `native-theme/src/model/defaults.rs` - ThemeDefaults.border typed as DefaultsBorderSpec
- `native-theme/src/model/mod.rs` - Updated pub use exports and FIELD_NAMES linting
- `native-theme/src/model/widgets/mod.rs` - All widget border fields typed as Option<WidgetBorderSpec>
- `native-theme/src/resolve/inheritance.rs` - resolve_border uses split types, D2 padding rule removed
- `native-theme/src/resolve/validate_helpers.rs` - require_border/border_all_optional/require_border_partial accept WidgetBorderSpec
- `native-theme/src/resolve/validate.rs` - Defaults border padding extraction removed, set to 0.0
- `native-theme/src/macos.rs` - Widget defaults use WidgetBorderSpec, test helpers use DefaultsBorderSpec
- `native-theme/src/lib.rs` - pub(crate) use updated with split type names
- `native-theme/src/resolve/tests.rs` - Removed padding field assignments on DefaultsBorderSpec
- `native-theme/tests/proptest_roundtrip.rs` - Split arb_border_spec into two strategies
- `native-theme/tests/merge_behavior.rs` - Updated to DefaultsBorderSpec for ThemeDefaults tests

## Decisions Made
- WidgetBorderSpec resolved output sets corner_radius_lg=0.0 and opacity=0.0 (defaults-only fields not available at widget level)
- D2 padding-derives-from-presence inheritance rule removed entirely since DefaultsBorderSpec has no padding
- ResolvedBorderSpec padding set to 0.0 in defaults-level construction (matches previous behavior where padding was always derived as 0.0)
- Proptest split into arb_defaults_border_spec (6 fields) and arb_widget_border_spec (6 fields) for type coverage

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed widget test using defaults-only fields on WidgetBorderSpec**
- **Found during:** Task 1 (compilation)
- **Issue:** Test in widgets/mod.rs constructed WidgetBorderSpec with corner_radius_lg and opacity fields that no longer exist
- **Fix:** Removed the two defaults-only fields from the test struct literal
- **Files modified:** native-theme/src/model/widgets/mod.rs
- **Verification:** cargo test passes
- **Committed in:** 1f00f51

**2. [Rule 1 - Bug] Fixed resolve/tests.rs setting padding on DefaultsBorderSpec**
- **Found during:** Task 1 (compilation)
- **Issue:** Test set defaults.border.padding_horizontal/padding_vertical which no longer exist on DefaultsBorderSpec
- **Fix:** Removed the two padding field assignments from the test
- **Files modified:** native-theme/src/resolve/tests.rs
- **Verification:** cargo test passes
- **Committed in:** 1f00f51

**3. [Rule 3 - Blocking] Updated proptest and merge_behavior integration tests**
- **Found during:** Task 1 (pre-release check)
- **Issue:** Integration tests in tests/ directory still referenced old unified BorderSpec type
- **Fix:** Split proptest strategy, updated merge_behavior test to use DefaultsBorderSpec
- **Files modified:** native-theme/tests/proptest_roundtrip.rs, native-theme/tests/merge_behavior.rs
- **Verification:** cargo test -p native-theme passes all 581 tests
- **Committed in:** 1f00f51

---

**Total deviations:** 3 auto-fixed (2 bugs, 1 blocking)
**Impact on plan:** All auto-fixes necessary for correctness. The type split intentionally caught these field misuses.

## Issues Encountered
None beyond the expected compilation errors from the type split (all resolved as deviations above).

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- BorderSpec split complete, ready for Phase 80 proc-macro codegen (BORDER-02 depends on clean border target)
- Plan 02 can proceed with reader visibility audit (CLEAN-03, READER-02)

---
*Phase: 79-borderspec-split-and-platform-reader-visibility-audit*
*Completed: 2026-04-13*
