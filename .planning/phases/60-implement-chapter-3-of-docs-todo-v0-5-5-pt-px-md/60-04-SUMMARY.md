---
phase: 60-implement-chapter-3-of-docs-todo-v0-5-5-pt-px-md
plan: 04
subsystem: model
tags: [tests, FontSize, line_height, serde, property-registry, lint_toml]

# Dependency graph
requires:
  - phase: 60-01
    provides: "TextScaleEntry.line_height as Option<FontSize>"
  - phase: 60-02
    provides: "serde renames with _px suffix on all 63 dimensional fields"
provides:
  - "All tests compile and pass with FontSize line_height"
  - "4 deferred tests enabled: round-trip, from_toml_with_base, lint_toml_preset, lint_toml_all_presets"
  - "property-registry.toml field names match TOML key names"
affects: [60-05]

# Tech tracking
tech-stack:
  added: []
  patterns: ["lint_toml tests use preset() + to_toml() round-trip for source verification"]

key-files:
  created: []
  modified:
    - native-theme/src/model/font.rs
    - native-theme/src/resolve/tests.rs
    - native-theme/src/model/mod.rs
    - native-theme/tests/proptest_roundtrip.rs
    - docs/property-registry.toml

key-decisions:
  - "lint_toml tests use preset() + to_toml() round-trip rather than include_str! (mod.rs not in presets directory)"
  - "Proptest line_height uses same FontSize variant as size for TryFrom unit consistency"

patterns-established:
  - "Preset lint verification via round-trip: load preset -> serialize -> lint"

requirements-completed: []

# Metrics
duration: 10min
completed: 2026-04-08
---

# Phase 60 Plan 04: Test Fixes and Property Registry Update Summary

**Fixed all 18 test compilation errors from FontSize type changes, enabled 4 deferred tests, and updated property-registry.toml with _px field naming convention**

## Performance

- **Duration:** 10 min
- **Started:** 2026-04-08T17:04:59Z
- **Completed:** 2026-04-08T17:15:37Z
- **Tasks:** 3
- **Files modified:** 5

## Accomplishments
- All 18 type mismatch errors fixed: line_height changed from bare f32 to FontSize::Px/Pt in 3 test files
- 4 deferred tests implemented: native_theme_serde_toml_round_trip, from_toml_with_base_merges_colors_onto_preset, lint_toml_preset_has_no_warnings, lint_toml_all_presets_clean
- property-registry.toml updated with _px suffixes on all 63+ dimensional fields matching TOML key convention
- Proptest strategy updated to generate FontSize for line_height matching size unit

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix model test compilation errors** - `4a4f199` (fix)
2. **Task 2: Fix resolve/tests.rs and enable deferred lint tests** - `c662c3e` (fix)
3. **Task 3: Update property-registry.toml field names** - `e04c695` (docs)

## Files Created/Modified
- `native-theme/src/model/font.rs` - 11 line_height values wrapped in FontSize::Px/Pt in test module
- `native-theme/src/resolve/tests.rs` - 6 line_height values wrapped in FontSize::Pt/Px in test constructors and assertions
- `native-theme/src/model/mod.rs` - 4 deferred tests implemented (round-trip, merge, lint single, lint all)
- `native-theme/tests/proptest_roundtrip.rs` - line_height strategy generates FontSize matching size unit
- `docs/property-registry.toml` - All dimensional fields renamed with _px suffix, TextScaleEntry gains line_height_pt/line_height_px

## Decisions Made
- lint_toml tests use preset() + to_toml() round-trip rather than include_str! because mod.rs is not in the presets directory
- Proptest line_height uses same FontSize variant (Pt/Px) as size to satisfy TryFrom unit consistency requirement

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All tests compile with 0 errors
- lint_toml tests are compile-checked; full pass verification deferred to Plan 05
- property-registry.toml is in sync with TOML key naming convention
- Ready for Plan 05 final verification

## Self-Check: PASSED

All 5 modified files exist. All 3 task commits verified (4a4f199, c662c3e, e04c695).

---
*Phase: 60-implement-chapter-3-of-docs-todo-v0-5-5-pt-px-md*
*Completed: 2026-04-08*
