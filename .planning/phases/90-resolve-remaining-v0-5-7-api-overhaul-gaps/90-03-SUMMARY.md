---
phase: 90-resolve-remaining-v0-5-7-api-overhaul-gaps
plan: 03
subsystem: watch
tags: [rename, raii, constructor, theme-subscription]

# Dependency graph
requires:
  - phase: 90-01
    provides: Rgba API polish (wave 1 prerequisite)
provides:
  - ThemeSubscription type (renamed from ThemeWatcher)
  - Single pub(crate) constructor with optional platform shutdown closure
  - Send + !Sync documentation on RAII guard
affects: [connectors, watch-module, public-api]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Single constructor with Option<Box<dyn FnOnce() + Send>> for platform-specific shutdown"

key-files:
  created: []
  modified:
    - native-theme/src/watch/mod.rs
    - native-theme/src/watch/kde.rs
    - native-theme/src/watch/gnome.rs
    - native-theme/src/watch/macos.rs
    - native-theme/src/watch/windows.rs
    - connectors/native-theme-gpui/examples/showcase-gpui.rs
    - connectors/native-theme-iced/examples/showcase-iced.rs

key-decisions:
  - "ThemeWatcherTick local enum variant in iced showcase kept as-is (user-defined message name, not library type)"

patterns-established:
  - "ThemeSubscription: single constructor pattern with Option<platform_shutdown> replaces constructor split"

requirements-completed: []

# Metrics
duration: 6min
completed: 2026-04-15
---

# Phase 90 Plan 03: ThemeWatcher to ThemeSubscription Rename Summary

**Renamed ThemeWatcher to ThemeSubscription and collapsed two constructors into one with optional platform shutdown closure**

## Performance

- **Duration:** 6 min
- **Started:** 2026-04-15T11:17:35Z
- **Completed:** 2026-04-15T11:23:43Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments
- Renamed `ThemeWatcher` to `ThemeSubscription` across all watch module files (mod.rs, kde.rs, gnome.rs, macos.rs, windows.rs)
- Collapsed `new()` and `with_platform_shutdown()` into single `new(tx, handle, Option<platform_shutdown>)` constructor
- Added `Send + !Sync` documentation note on the struct
- Updated connector showcase examples (gpui, iced) to use new type name
- Zero `ThemeWatcher` references remain in `native-theme/src/` or connector source

## Task Commits

Each task was committed atomically:

1. **Task 1: Rename ThemeWatcher to ThemeSubscription with single constructor** - `be595be` (feat)
2. **Task 2: Update connector examples and external references** - `fcfab43` (feat)

## Files Created/Modified
- `native-theme/src/watch/mod.rs` - Renamed struct, collapsed constructors, updated Debug/Drop/tests, added Send+!Sync doc
- `native-theme/src/watch/kde.rs` - Updated return type and constructor call (passes None for platform_shutdown)
- `native-theme/src/watch/gnome.rs` - Updated return type, constructor call, and doc comment
- `native-theme/src/watch/macos.rs` - Updated return type, constructor call wraps closure in Some(), doc comments
- `native-theme/src/watch/windows.rs` - Updated return type, constructor call wraps closure in Some(), doc comments
- `connectors/native-theme-gpui/examples/showcase-gpui.rs` - ThemeWatcher -> ThemeSubscription in type and doc
- `connectors/native-theme-iced/examples/showcase-iced.rs` - ThemeWatcher -> ThemeSubscription in type and doc

## Decisions Made
- ThemeWatcherTick local enum variant in iced showcase Message kept as-is -- it is a user-defined message name, not the library type

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- ThemeSubscription rename complete, all callers updated
- Connector examples compile cleanly
- Ready for remaining phase 90 plans

---
*Phase: 90-resolve-remaining-v0-5-7-api-overhaul-gaps*
*Completed: 2026-04-15*
