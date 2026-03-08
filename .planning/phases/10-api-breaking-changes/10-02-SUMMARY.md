---
phase: 10-api-breaking-changes
plan: 02
subsystem: api
tags: [rust, api-design, associated-methods, breaking-change]

# Dependency graph
requires:
  - phase: 10-api-breaking-changes plan 01
    provides: Flat ThemeColors with 36 direct fields
provides:
  - NativeTheme::preset(), ::from_toml(), ::from_file(), ::list_presets(), .to_toml() associated methods
  - Free function exports removed from public API
affects: [10-api-breaking-changes plan 03, downstream consumers]

# Tech tracking
tech-stack:
  added: []
  patterns: [associated method API pattern for NativeTheme]

key-files:
  created: []
  modified:
    - native-theme/src/model/mod.rs
    - native-theme/src/presets.rs
    - native-theme/src/lib.rs
    - native-theme/src/gnome/mod.rs
    - native-theme/README.md
    - native-theme/tests/preset_loading.rs

key-decisions:
  - "Preset functions made pub(crate) rather than removed; impl NativeTheme delegates to them"
  - "to_toml() becomes &self method instead of free function taking &NativeTheme"
  - "from_system() remains a free function (not moved in this plan)"

patterns-established:
  - "Associated method API: NativeTheme::preset('name') instead of native_theme::preset('name')"

requirements-completed: [API-05, API-06]

# Metrics
duration: 3min
completed: 2026-03-08
---

# Phase 10 Plan 02: Move Preset API to NativeTheme Methods Summary

**NativeTheme::preset/from_toml/from_file/list_presets/to_toml associated methods replace free function exports**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-08T05:34:17Z
- **Completed:** 2026-03-08T05:37:00Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Moved 5 preset API functions from free function exports to NativeTheme associated methods
- Made presets.rs functions pub(crate) to prevent direct access from outside the crate
- Updated all internal callers (lib.rs from_linux, gnome/mod.rs from_gnome and tests)
- Updated README examples and integration tests to use new API
- All 91 library tests, 12 integration tests, and 9 doc tests pass

## Task Commits

Each task was committed atomically:

1. **Task 1: Move preset functions to impl NativeTheme and update all callers** - `bba5ae6` (feat)
2. **Task 2: Update README and integration tests for new API** - `2f981c3` (feat)

## Files Created/Modified
- `native-theme/src/model/mod.rs` - Added impl NativeTheme block with 5 associated methods delegating to crate::presets
- `native-theme/src/presets.rs` - Made 5 functions pub(crate), removed doc comments/examples
- `native-theme/src/lib.rs` - Removed free function re-exports, updated from_linux() caller
- `native-theme/src/gnome/mod.rs` - Updated from_gnome() and test helper to use NativeTheme::preset()
- `native-theme/README.md` - Updated all code examples to NativeTheme::method() API
- `native-theme/tests/preset_loading.rs` - Rewrote all imports and calls to use NativeTheme methods

## Decisions Made
- Preset functions made pub(crate) rather than removed entirely; impl NativeTheme delegates to them. This keeps the internal implementation clean while providing the new public API.
- to_toml() takes &self now (instance method) rather than a separate &NativeTheme parameter -- more idiomatic Rust.
- from_system() remains a free function in lib.rs since it's not part of the preset API being moved.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Plan 02 complete. Ready for Plan 03 (remaining API breaking changes if any).
- All public API now uses NativeTheme associated methods for preset operations.
- from_system() still exists as a free function in the crate root.

---
*Phase: 10-api-breaking-changes*
*Completed: 2026-03-08*
