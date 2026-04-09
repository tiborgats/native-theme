---
phase: 66-linux-watchers
plan: 02
subsystem: watch
tags: [dispatch, detect, kde, gnome, budgie, linux, theme-watcher, cfg-gate]

# Dependency graph
requires:
  - phase: 66-linux-watchers-01
    provides: KDE inotify watcher (watch_kde), GNOME D-Bus watcher (watch_gnome)
  - phase: 65-themewatcher-core-api
    provides: ThemeWatcher struct, ThemeChangeEvent enum, on_theme_change stub
provides:
  - Wired on_theme_change() dispatch routing KDE to inotify watcher, GNOME/Budgie to D-Bus watcher
  - Runtime DE detection via detect_linux_de() for backend selection
  - Error::Unsupported for unrecognized DEs and non-Linux platforms
affects: [67-macos-windows-watchers]

# Tech tracking
tech-stack:
  added: []
  patterns: [runtime DE dispatch with cfg-gated match arms, platform-gated test assertions]

key-files:
  created: []
  modified:
    - native-theme/src/watch/mod.rs
    - native-theme/src/watch/kde.rs

key-decisions:
  - "cfg-gated match arms inside on_theme_change: each DE variant gated by its feature (kde, portal) so missing features fall through to Unsupported"
  - "Platform-split test: non-Linux asserts Unsupported with 'not yet implemented'; Linux asserts Ok or Unsupported or Unavailable to handle CI and real DE environments"

patterns-established:
  - "Runtime DE dispatch: detect_linux_de(xdg_current_desktop()) -> match with cfg-gated arms for each backend"
  - "Platform-gated tests: #[cfg(target_os)] to split assertions by platform"

requirements-completed: [WATCH-02, WATCH-03]

# Metrics
duration: 9min
completed: 2026-04-09
---

# Phase 66 Plan 02: Watcher Dispatch Wiring Summary

**Wired on_theme_change() to dispatch KDE/GNOME/Budgie to platform-specific watcher backends via runtime DE detection**

## Performance

- **Duration:** 9 min
- **Started:** 2026-04-09T19:28:33Z
- **Completed:** 2026-04-09T19:37:35Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Replaced on_theme_change() stub with runtime dispatch using detect_linux_de()
- KDE routes to kde::watch_kde (inotify), GNOME/Budgie routes to gnome::watch_gnome (D-Bus portal)
- Unrecognized DEs and non-Linux platforms return descriptive Error::Unsupported
- Removed #[allow(dead_code)] from ThemeWatcher::new() (now used by backends)
- All 5 watch tests pass; no regressions in 616 non-watch tests

## Task Commits

Each task was committed atomically:

1. **Task 1: Wire on_theme_change() dispatch and add cfg-gated module declarations** - `6e6cdf1` (feat)
2. **Task 2: Full test suite and pre-release check** - `800a10d` (fix)

## Files Created/Modified
- `native-theme/src/watch/mod.rs` - Replaced stub on_theme_change() with DE dispatch, updated tests with platform-gated assertions
- `native-theme/src/watch/kde.rs` - Fixed clippy map_or -> is_some_and warning

## Decisions Made
- Used cfg-gated match arms inside on_theme_change() so each DE variant is only compiled when its feature is active (kde, portal). Missing features fall through to the wildcard Unsupported arm.
- Split the on_theme_change test into platform-specific variants: non-Linux asserts exact Unsupported message; Linux accepts Ok, Unsupported, or Unavailable since the result depends on the runtime environment.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed clippy map_or warning in watch/kde.rs**
- **Found during:** Task 2 (clippy check)
- **Issue:** `map_or(false, ...)` in kde.rs filename filter flagged by clippy as `unnecessary_map_or` (new in rustc 1.94)
- **Fix:** Replaced with `is_some_and(...)` per clippy suggestion
- **Files modified:** native-theme/src/watch/kde.rs
- **Verification:** `cargo clippy --features linux,watch` clean for watch code
- **Committed in:** 800a10d (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Minor clippy lint fix in Plan 01 code. No scope creep.

## Issues Encountered
- Pre-existing `gnome::tests::build_gnome_variant_normal_contrast_no_flag` test failure (unrelated to watch code, confirmed by stashing changes)
- Pre-existing `gsettings_get` dead_code clippy warning in detect.rs (documented in Plan 01 summary)
- Pre-existing clippy warnings in gnome/mod.rs and kde/mod.rs from uncommitted working tree changes (out of scope)

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Linux watcher feature complete: on_theme_change() dispatches to KDE (inotify) and GNOME/Budgie (D-Bus portal) backends
- Phase 67 (macOS/Windows watchers) can proceed independently to add platform backends for those platforms
- The wildcard match arm in on_theme_change() ensures graceful Unsupported for DEs without watcher backends

---
*Phase: 66-linux-watchers*
*Completed: 2026-04-09*
