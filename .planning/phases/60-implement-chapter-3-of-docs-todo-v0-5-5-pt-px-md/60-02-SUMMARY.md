---
phase: 60-implement-chapter-3-of-docs-todo-v0-5-5-pt-px-md
plan: 02
subsystem: model
tags: [serde, macro, rename, toml, px-suffix]

requires:
  - phase: 60-01
    provides: "FontSpec serde proxy with _px/_pt suffixes"
provides:
  - "define_widget_pair! macro with as rename syntax"
  - "__field_name! helper macro for FIELD_NAMES emission"
  - "All 63 always-pixel fields have serde(rename) with _px suffix"
  - "FIELD_NAMES arrays emit TOML key names (with _px suffix)"
affects: [60-03, 60-04, 60-05]

tech-stack:
  added: []
  patterns: ["__field_name! helper macro for conditional const array emission", "as rename syntax in define_widget_pair! option block"]

key-files:
  created: []
  modified:
    - native-theme/src/model/widgets/mod.rs
    - native-theme/src/model/border.rs
    - native-theme/src/model/icon_sizes.rs
    - native-theme/src/model/defaults.rs

key-decisions:
  - "Used __field_name! helper macro with pub(crate) use for FIELD_NAMES emission in const context"
  - "Optional $(as $opt_rename:literal)? pattern in define_widget_pair! avoids separate macro arms"
  - "Resolved structs never get serde renames (always plain f32 pixel values)"

patterns-established:
  - "as rename syntax: field as \"toml_name\": type in define_widget_pair! option blocks"
  - "__field_name! helper: conditional field name emission for const arrays"

requirements-completed: []

duration: 5min
completed: 2026-04-08
---

# Phase 60 Plan 02: Serde Rename Summary

**Extended define_widget_pair! macro with as-rename syntax and applied _px serde renames to all 63 always-pixel dimensional fields across widgets, BorderSpec, IconSizes, and ThemeDefaults**

## Performance

- **Duration:** 5 min
- **Started:** 2026-04-08T16:57:39Z
- **Completed:** 2026-04-08T17:02:57Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Extended `define_widget_pair!` macro to support `as "toml_name"` rename syntax in option blocks
- Added `__field_name!` helper macro for conditional FIELD_NAMES emission (rename literal vs stringify)
- Applied `_px` serde renames to 51 widget dimensional fields across 20 widget definitions
- Applied `_px` serde renames to 12 manually-defined fields: BorderSpec (5), IconSizes (5), ThemeDefaults (2)
- Updated all FIELD_NAMES arrays to emit TOML key names with `_px` suffix

## Task Commits

Each task was committed atomically:

1. **Task 1: Extend define_widget_pair! macro with rename support** - `a7a99b4` (feat)
2. **Task 2: Add serde renames to manually-defined structs** - `e75ab6b` (feat)

## Files Created/Modified
- `native-theme/src/model/widgets/mod.rs` - __field_name! helper macro, extended define_widget_pair! pattern, 51 widget field renames, test update
- `native-theme/src/model/border.rs` - 5 serde renames on dimensional fields, FIELD_NAMES updated
- `native-theme/src/model/icon_sizes.rs` - 5 serde renames on all fields, FIELD_NAMES updated
- `native-theme/src/model/defaults.rs` - 2 serde renames on focus_ring fields, FIELD_NAMES updated

## Decisions Made
- Used `__field_name!` helper macro with `pub(crate) use` for FIELD_NAMES emission -- macro invocations in const array initializers work because macros expand before const evaluation
- Used optional `$(as $opt_rename:literal)?` pattern in the single macro rule rather than separate arms -- cleaner and avoids code duplication
- Resolved structs never get serde renames -- they are runtime types with plain f32 pixel values, not serialized to TOML

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- Test compilation in `resolve/tests.rs` fails due to pre-existing FontSize type changes from Phase 59 -- these are expected and will be fixed in Plan 04 (test/preset updates)

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- All 63 always-pixel fields now have _px serde renames
- FIELD_NAMES arrays updated for lint_toml compatibility
- Ready for Plan 03 (TOML preset key renames) and Plan 04 (test updates)

## Self-Check: PASSED

- All 4 modified files exist on disk
- Both task commits (a7a99b4, e75ab6b) verified in git log

---
*Phase: 60-implement-chapter-3-of-docs-todo-v0-5-5-pt-px-md*
*Completed: 2026-04-08*
