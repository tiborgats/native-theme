---
phase: 65-themewatcher-core-api
plan: 01
subsystem: api
tags: [watch, theme-watcher, notify, feature-gate, raii]

# Dependency graph
requires: []
provides:
  - "ThemeChangeEvent enum (Debug+Clone+PartialEq+Eq, non_exhaustive)"
  - "ThemeWatcher RAII struct with Drop (shutdown signal + thread join)"
  - "on_theme_change() public API stub returning Error::Unsupported"
  - "watch feature flag gating entire module"
  - "notify 8.2 optional dependency under Linux target"
affects: [66-linux-watchers, 67-macos-windows-watchers]

# Tech tracking
tech-stack:
  added: [notify 8.2 (optional, Linux-only)]
  patterns: [cfg-feature-gated module, RAII thread ownership, channel-based shutdown]

key-files:
  created: [native-theme/src/watch/mod.rs]
  modified: [native-theme/Cargo.toml, native-theme/src/lib.rs, Cargo.lock]

key-decisions:
  - "ThemeWatcher gets Debug derive (needed for Result::unwrap_err in tests and general usability)"
  - "ThemeWatcher::new marked #[allow(dead_code)] since no backends consume it yet in Phase 65"
  - "notify added only under Linux target section per research (macOS/Windows use existing platform crates)"

patterns-established:
  - "watch module pattern: cfg(feature = watch) gates mod + re-exports in lib.rs"
  - "ThemeWatcher RAII: Drop drops Sender (channel disconnection = shutdown signal), then joins thread"
  - "ThemeChangeEvent is signal-only: no theme data, callers re-run SystemTheme::from_system()"

requirements-completed: [WATCH-01, WATCH-06]

# Metrics
duration: 3min
completed: 2026-04-09
---

# Phase 65 Plan 01: ThemeWatcher Core API Summary

**watch feature-gated module with ThemeChangeEvent enum, ThemeWatcher RAII struct, and on_theme_change() stub returning Error::Unsupported until platform backends are wired**

## Performance

- **Duration:** 3 min
- **Started:** 2026-04-09T17:42:05Z
- **Completed:** 2026-04-09T17:45:52Z
- **Tasks:** 2
- **Files modified:** 4 (Cargo.toml, lib.rs, watch/mod.rs, Cargo.lock)

## Accomplishments
- Created watch module with ThemeChangeEvent (2 variants, non_exhaustive, Debug+Clone+PartialEq+Eq), ThemeWatcher (RAII with Drop), and on_theme_change() stub
- Added watch feature flag and notify 8.2 optional dependency (Linux-only) to Cargo.toml
- Verified feature gate: notify absent without watch, present with watch; watch symbols absent without feature
- All 458 unit tests + 39 doctests pass with zero regressions

## Task Commits

Each task was committed atomically:

1. **Task 1: Create watch module with ThemeChangeEvent, ThemeWatcher, and on_theme_change() stub** - `b2bc041` (feat)
2. **Task 2: Feature gate verification and full test suite** - `f20bbb5` (chore - rustfmt formatting)

## Files Created/Modified
- `native-theme/src/watch/mod.rs` - New module: ThemeChangeEvent enum, ThemeWatcher struct with Drop, on_theme_change() stub, 5 unit tests
- `native-theme/Cargo.toml` - Added watch = ["dep:notify"] feature flag and notify 8.2 optional Linux dependency
- `native-theme/src/lib.rs` - cfg(feature = "watch") module declaration and re-exports
- `Cargo.lock` - Updated with notify dependency tree

## Decisions Made
- ThemeWatcher gets `#[derive(Debug)]` -- required for `Result::unwrap_err` in tests and improves general usability
- `ThemeWatcher::new` marked `#[allow(dead_code)]` -- constructor is pub(crate) for Phase 66/67 backends, unused in Phase 65
- notify placed only under `[target.'cfg(target_os = "linux")'.dependencies]` per research findings (macOS/Windows use their existing platform crates)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added Debug derive to ThemeWatcher**
- **Found during:** Task 1 (TDD GREEN phase)
- **Issue:** `Result::unwrap_err()` requires `T: Debug`, test compilation failed without Debug on ThemeWatcher
- **Fix:** Added `#[derive(Debug)]` to ThemeWatcher struct
- **Files modified:** native-theme/src/watch/mod.rs
- **Verification:** All 5 watch tests pass
- **Committed in:** b2bc041 (Task 1 commit)

**2. [Rule 3 - Blocking] Added #[allow(dead_code)] to ThemeWatcher::new**
- **Found during:** Task 1 (TDD GREEN phase)
- **Issue:** `pub(crate) fn new` triggers dead_code warning since no backends use it yet
- **Fix:** Added `#[allow(dead_code)]` annotation with comment explaining Phase 66/67 usage
- **Files modified:** native-theme/src/watch/mod.rs
- **Verification:** Clean clippy output (no watch-related warnings)
- **Committed in:** b2bc041 (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Both fixes necessary for compilation. No scope creep.

## Issues Encountered
- pre-release-check.sh fails on clippy `-D warnings` due to pre-existing `gsettings_get` dead_code warning in detect.rs:680. This is NOT caused by Phase 65 changes (confirmed by testing clean HEAD). Logged to deferred-items.md.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- watch module API surface is complete and ready for Phase 66 (Linux watchers) and Phase 67 (macOS/Windows watchers)
- ThemeWatcher::new constructor is pub(crate) for backend implementations to use
- on_theme_change() returns Error::Unsupported -- backends will replace the stub body with platform-specific watcher creation

---
*Phase: 65-themewatcher-core-api*
*Completed: 2026-04-09*
