---
phase: 67-macos-windows-watchers
plan: 02
subsystem: watch
tags: [windows, com-sta, uisettings, colorvalueschanged, message-pump, theme-watcher]

# Dependency graph
requires:
  - phase: 67-macos-windows-watchers
    plan: 01
    provides: ThemeWatcher with platform_shutdown handle, Windows Cargo.toml feature flags
  - phase: 65-themewatcher-core-api
    provides: ThemeWatcher struct, on_theme_change() dispatch, ThemeChangeEvent enum
provides:
  - Windows watcher backend (watch/windows.rs) using UISettings::ColorValuesChanged with COM STA message pump
  - Windows dispatch arm in on_theme_change() with cfg-gated feature error
affects: []

# Tech tracking
tech-stack:
  added: [UISettings ColorValuesChanged, COM STA, GetMessageW/DispatchMessageW message pump, PostThreadMessageW WM_QUIT]
  patterns: [Windows COM STA watcher thread with message pump, thread ID channel for cross-thread PostThreadMessageW shutdown]

key-files:
  created: [native-theme/src/watch/windows.rs]
  modified: [native-theme/src/watch/mod.rs, native-theme/src/watch/macos.rs]

key-decisions:
  - "GetCurrentThreadId sent via oneshot channel for PostThreadMessageW(WM_QUIT) shutdown from any thread"
  - "COM STA (COINIT_APARTMENTTHREADED) required for WinRT UISettings on background thread"
  - "GetMessageW/DispatchMessageW pump breaks on WM_QUIT (returns FALSE), enabling clean shutdown"

patterns-established:
  - "Windows watcher pattern: COM STA init + UISettings event subscription + message pump + PostThreadMessageW shutdown"

requirements-completed: []

# Metrics
duration: 3min
completed: 2026-04-09
---

# Phase 67 Plan 02: Windows Watcher Backend Summary

**Windows theme change watcher via UISettings::ColorValuesChanged with COM STA message pump and PostThreadMessageW(WM_QUIT) shutdown**

## Performance

- **Duration:** 3 min
- **Started:** 2026-04-09T21:19:53Z
- **Completed:** 2026-04-09T21:23:29Z
- **Tasks:** 1
- **Files modified:** 3

## Accomplishments
- Implemented Windows watcher backend using UISettings::ColorValuesChanged behind cfg gates
- COM STA initialization on dedicated background thread with GetMessageW/DispatchMessageW message pump
- Shutdown via PostThreadMessageW(WM_QUIT) through the platform_shutdown handle established in Plan 01
- Wired Windows dispatch arm in on_theme_change() with proper cfg-gated feature error messages

## Task Commits

Each task was committed atomically:

1. **Task 1: Create Windows watcher backend and wire dispatch** - `290e4d2` (feat)

## Files Created/Modified
- `native-theme/src/watch/windows.rs` - New Windows watcher backend: COM STA init, UISettings::ColorValuesChanged subscription, GetMessageW/DispatchMessageW message pump, PostThreadMessageW shutdown
- `native-theme/src/watch/mod.rs` - Added Windows module declaration and cfg-gated dispatch arm in on_theme_change()
- `native-theme/src/watch/macos.rs` - Formatting fixes from cargo fmt

## Decisions Made
- Used GetCurrentThreadId sent via mpsc oneshot channel so the platform_shutdown closure can call PostThreadMessageW from any thread
- COM initialized as STA (COINIT_APARTMENTTHREADED) -- required for WinRT UISettings to function on a background thread
- GetMessageW returns FALSE on WM_QUIT, naturally breaking the message pump loop for clean shutdown
- CoUninitialize called on all exit paths (success, UISettings::new failure, ColorValuesChanged subscription failure)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] cargo fmt formatting fixes in macos.rs**
- **Found during:** Task 1 (pre-release-check.sh run)
- **Issue:** macos.rs had minor formatting inconsistencies (import line wrapping, trailing spaces in comments, map_err chain formatting)
- **Fix:** cargo fmt auto-applied as part of pre-release-check.sh
- **Files modified:** native-theme/src/watch/macos.rs
- **Committed in:** 290e4d2 (part of task commit)

---

**Total deviations:** 1 auto-fixed (1 formatting)
**Impact on plan:** Trivial formatting fix, no scope creep.

## Issues Encountered
- Pre-existing test failure in `gnome::tests::build_gnome_variant_normal_contrast_no_flag` -- confirmed unrelated to this plan's changes (same failure documented in Plan 01 summary). Not in scope.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All macOS and Windows watcher backends are complete
- Phase 67 (macos-windows-watchers) is fully implemented
- Both platforms use the platform_shutdown pattern for immediate watcher shutdown on Drop

## Self-Check: PASSED

- FOUND: native-theme/src/watch/windows.rs
- FOUND: 67-02-SUMMARY.md
- FOUND: commit 290e4d2

---
*Phase: 67-macos-windows-watchers*
*Completed: 2026-04-09*
