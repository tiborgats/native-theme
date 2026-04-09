---
phase: 66-linux-watchers
plan: 01
subsystem: watch
tags: [inotify, dbus, zbus, notify, kde, gnome, linux, theme-watcher]

# Dependency graph
requires:
  - phase: 65-themewatcher-core-api
    provides: ThemeWatcher struct, ThemeChangeEvent enum, on_theme_change stub
provides:
  - KDE inotify watcher backend (watch_kde) with parent-directory watching and debounce
  - GNOME D-Bus portal watcher backend (watch_gnome) via zbus::blocking
affects: [66-02 (watcher dispatch wiring), 67-macos-windows-watchers]

# Tech tracking
tech-stack:
  added: [zbus/blocking-api feature for D-Bus signal subscription]
  patterns: [parent-directory watching with filename filter, Instant-based debounce, blocking D-Bus signal iteration]

key-files:
  created:
    - native-theme/src/watch/kde.rs
    - native-theme/src/watch/gnome.rs
  modified:
    - native-theme/src/watch/mod.rs
    - native-theme/Cargo.toml

key-decisions:
  - "Added zbus as direct dep with blocking-api feature (ashpd does not enable it)"
  - "watch feature gates both notify and zbus deps"

patterns-established:
  - "Parent-directory inotify watching: watch config dir NonRecursive, filter by filename to survive QSaveFile atomic renames"
  - "Instant-based debounce: 300ms window with initial offset for immediate first event"
  - "Blocking D-Bus signals: ashpd::zbus::blocking::Proxy with receive_signal_with_args for sync signal iteration"

requirements-completed: [WATCH-02, WATCH-03]

# Metrics
duration: 8min
completed: 2026-04-09
---

# Phase 66 Plan 01: Linux Watchers Summary

**KDE inotify watcher with parent-dir watching and 300ms debounce, GNOME D-Bus portal watcher via zbus::blocking signal subscription**

## Performance

- **Duration:** 8 min
- **Started:** 2026-04-09T19:16:40Z
- **Completed:** 2026-04-09T19:25:36Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- KDE watcher watches parent directory of kdeglobals NonRecursive, filters by filename (kdeglobals, kcmfontsrc), debounces at 300ms
- GNOME watcher subscribes to SettingChanged D-Bus signal filtered to org.freedesktop.appearance namespace using zbus::blocking
- Both watchers return ThemeWatcher via RAII contract, handle shutdown via channel disconnection + try_recv
- No unwrap/expect/unsafe anywhere in either file

## Task Commits

Each task was committed atomically:

1. **Task 1: Create KDE inotify watcher with parent-directory watching and debounce** - `eb4c279` (feat)
2. **Task 2: Create GNOME D-Bus portal watcher with zbus::blocking** - `14f64ca` (feat)

## Files Created/Modified
- `native-theme/src/watch/kde.rs` - KDE inotify watcher with parent-dir watching, filename filter, 300ms debounce
- `native-theme/src/watch/gnome.rs` - GNOME D-Bus portal watcher using zbus::blocking signal subscription
- `native-theme/src/watch/mod.rs` - Added cfg-gated module declarations for kde and gnome watchers
- `native-theme/Cargo.toml` - Added zbus dep with blocking-api feature, gated by watch feature

## Decisions Made
- Added zbus as a direct dependency with `blocking-api` feature because ashpd's zbus re-export does not enable the blocking module. The `watch` feature now gates `dep:zbus` alongside `dep:notify`.
- Used `ashpd::zbus::blocking` import path (via ashpd re-export) rather than adding a separate zbus import, keeping the dependency relationship clear.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added zbus direct dependency with blocking-api feature**
- **Found during:** Task 2 (GNOME D-Bus watcher)
- **Issue:** `ashpd::zbus::blocking` module was gated behind `zbus/blocking-api` feature which ashpd does not enable. Compilation failed with "could not find `blocking` in `zbus`".
- **Fix:** Added `zbus = { version = "5.13", optional = true, default-features = false, features = ["blocking-api"] }` to linux target deps, gated by `watch` feature.
- **Files modified:** native-theme/Cargo.toml
- **Verification:** `cargo check -p native-theme --features linux,watch` compiles cleanly
- **Committed in:** 14f64ca (Task 2 commit)

**2. [Rule 3 - Blocking] Created gnome.rs stub for cargo fmt compatibility**
- **Found during:** Task 1 commit verification
- **Issue:** `cargo fmt` parses all `mod` declarations regardless of `#[cfg]` gates. The `mod gnome;` declaration in mod.rs caused fmt to fail because gnome.rs didn't exist yet.
- **Fix:** Created minimal gnome.rs stub (module doc comment only) alongside Task 1, replaced with full implementation in Task 2.
- **Files modified:** native-theme/src/watch/gnome.rs
- **Verification:** `cargo fmt --check` passes
- **Committed in:** eb4c279 (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Both fixes necessary for compilation. No scope creep.

## Issues Encountered
- Pre-existing `dead_code` clippy warning on `gsettings_get` in detect.rs (unrelated to this plan, confirmed by stashing changes and re-running clippy on clean state). Not fixed per scope boundary rules.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Both watcher backends ready for dispatch wiring in Plan 02
- Plan 02 will wire on_theme_change() to dispatch to watch_kde/watch_gnome based on detect_linux_de()
- The `#[allow(dead_code)]` annotations on watch_kde and watch_gnome will be removed when Plan 02 wires the dispatch

---
*Phase: 66-linux-watchers*
*Completed: 2026-04-09*
