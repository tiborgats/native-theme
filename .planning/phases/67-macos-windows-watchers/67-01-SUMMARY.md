---
phase: 67-macos-windows-watchers
plan: 01
subsystem: watch
tags: [macos, objc2, cfrunloop, nsdistributednotificationcenter, theme-watcher, platform-shutdown]

# Dependency graph
requires:
  - phase: 65-themewatcher-core-api
    provides: ThemeWatcher struct, on_theme_change() dispatch, ThemeChangeEvent enum
  - phase: 66-linux-watchers
    provides: KDE/GNOME watcher backends, watch module structure
provides:
  - ThemeWatcher with optional platform-specific shutdown handle (Box<dyn FnOnce() + Send>)
  - macOS watcher backend (watch/macos.rs) using CFRunLoop + NSDistributedNotificationCenter
  - Cargo.toml feature flags for macOS watcher (objc2-foundation, objc2-core-foundation) and Windows watcher (windows crate)
  - macOS dispatch arm in on_theme_change()
affects: [67-02-windows-watcher]

# Tech tracking
tech-stack:
  added: [NSDistributedNotificationCenter, CFRunLoop, block2 RcBlock]
  patterns: [platform-shutdown closure in ThemeWatcher Drop, SendableCFRunLoop wrapper for thread-safe CFRunLoop::stop]

key-files:
  created: [native-theme/src/watch/macos.rs]
  modified: [native-theme/Cargo.toml, native-theme/src/watch/mod.rs]

key-decisions:
  - "ThemeWatcher extended with Box<dyn FnOnce() + Send> platform_shutdown field for immediate wakeup on Drop"
  - "CFRunLoop::run() (blocking) + CFRunLoop::stop() from Drop, not run_in_mode timeout loop -- gives instant shutdown"
  - "SendableCFRunLoop newtype wrapper for thread-safe CFRunLoop::stop() since objc2 does not impl Send for CFRunLoop"
  - "Manual Debug impl for ThemeWatcher since Box<dyn FnOnce()> does not implement Debug"

patterns-established:
  - "Platform shutdown pattern: with_platform_shutdown() constructor + closure called before channel drop in Drop impl"
  - "SendableCFRunLoop: thin NonNull wrapper for cross-thread CFRunLoop::stop() with unsafe Send impl"

requirements-completed: []

# Metrics
duration: 8min
completed: 2026-04-09
---

# Phase 67 Plan 01: macOS Watcher Infrastructure Summary

**macOS theme watcher via NSDistributedNotificationCenter with CFRunLoop, plus ThemeWatcher platform-shutdown extension for immediate wakeup on Drop**

## Performance

- **Duration:** 8 min
- **Started:** 2026-04-09T21:09:28Z
- **Completed:** 2026-04-09T21:17:54Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Extended ThemeWatcher with optional platform-specific shutdown handle for immediate event loop wakeup on Drop
- Implemented macOS watcher backend using NSDistributedNotificationCenter + CFRunLoop behind cfg gates
- Added Cargo.toml feature flags for both macOS (objc2-foundation, objc2-core-foundation) and Windows (windows crate) watchers
- Wired macOS dispatch arm in on_theme_change() with proper cfg-gated feature error messages

## Task Commits

Each task was committed atomically:

1. **Task 1: Add Cargo.toml feature flags for macOS and Windows watchers** - `7fe5c2e` (chore)
2. **Task 2: Extend ThemeWatcher with platform shutdown handle and implement macOS watcher** - `0368d2d` (feat)

## Files Created/Modified
- `native-theme/Cargo.toml` - Added NSDistributedNotificationCenter, NSNotification, NSRunLoop, NSOperation, block2 features to objc2-foundation; CFRunLoop, CFDate to objc2-core-foundation; Win32_System_Com, Win32_System_Threading to windows
- `native-theme/src/watch/mod.rs` - Added platform_shutdown field, with_platform_shutdown() constructor, manual Debug impl, macOS module declaration, macOS dispatch arm, updated cfg gates on fallback and test
- `native-theme/src/watch/macos.rs` - New macOS watcher backend: SendableCFRunLoop wrapper, watch_macos() function with CFRunLoop + NSDistributedNotificationCenter observer registration

## Decisions Made
- Used `Box<dyn FnOnce() + Send>` for platform_shutdown rather than a platform-specific enum -- more extensible, each platform provides its own closure
- Used `CFRunLoop::run()` (blocks forever) + `CFRunLoop::stop()` from Drop instead of `run_in_mode()` with timeout loop -- gives instant shutdown without 1-second latency, simpler code
- Created `SendableCFRunLoop` newtype around `NonNull<CFRunLoop>` with unsafe Send impl because objc2 doesn't mark CFRunLoop as Send, but Apple documents CFRunLoopStop as thread-safe
- Oneshot channel from watcher thread to constructor to pass CFRunLoop handle back

## Deviations from Plan
None - plan executed exactly as written.

## Issues Encountered
- Pre-existing test failure in `gnome::tests::build_gnome_variant_normal_contrast_no_flag` -- confirmed unrelated to this plan's changes (fails identically on pre-change code). Not in scope.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- ThemeWatcher platform_shutdown infrastructure is ready for the Windows watcher in Plan 02
- Windows Cargo.toml feature flags (Win32_System_Com, Win32_System_Threading) already added
- on_theme_change() needs a Windows dispatch arm (Plan 02 will add `#[cfg(target_os = "windows")]` block)

---
*Phase: 67-macos-windows-watchers*
*Completed: 2026-04-09*
