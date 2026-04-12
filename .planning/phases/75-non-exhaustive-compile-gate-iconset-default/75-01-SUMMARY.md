---
phase: 75-non-exhaustive-compile-gate-iconset-default
plan: 01
subsystem: detect
tags: [non_exhaustive, linux-desktop, wayland, hyprland, sway, river, niri, cosmic]

# Dependency graph
requires: []
provides:
  - "LinuxDesktop enum with #[non_exhaustive] and five Wayland compositor variants"
  - "detect_linux_de() maps Hyprland, sway, river, niri, COSMIC to dedicated variants"
  - "Pipeline dispatch (sync and async) handles all new variants with adwaita preset"
affects: [76-type-rename-crate-root, 82-icon-api-rework, 83-detection-cache]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "#[non_exhaustive] on LinuxDesktop forces wildcard arms in external crate matches"
    - "Within-crate matches list all variants explicitly (no wildcard needed)"

key-files:
  created: []
  modified:
    - native-theme/src/detect.rs
    - native-theme/src/pipeline.rs

key-decisions:
  - "Removed unreachable wildcard arms in same-crate matches (non_exhaustive only affects external crates)"
  - "Wayland compositors read icon themes via org.gnome.desktop.interface gsettings (GTK portal interface)"

patterns-established:
  - "LinuxDesktop #[non_exhaustive]: new DE variants are non-breaking additions"
  - "Wayland compositor DEs use adwaita preset and GNOME gsettings interface"

requirements-completed: [LAYOUT-02]

# Metrics
duration: 7min
completed: 2026-04-12
---

# Phase 75 Plan 01: LinuxDesktop non_exhaustive Summary

**#[non_exhaustive] on LinuxDesktop with Hyprland, Sway, River, Niri, CosmicDe variants and full pipeline dispatch**

## Performance

- **Duration:** 7 min
- **Started:** 2026-04-12T15:37:10Z
- **Completed:** 2026-04-12T15:44:55Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Added `#[non_exhaustive]` to `LinuxDesktop` enum for future-proof extensibility
- Added five Wayland compositor variants: Hyprland, Sway, River, Niri, CosmicDe
- Updated `detect_linux_de()` to map XDG_CURRENT_DESKTOP strings to new variants
- Updated `from_linux()` and `from_system_async_inner()` match arms for new variants
- Updated tests: renamed 3 existing tests, added 3 new detection tests (River, Niri, COSMIC colon-separated)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add #[non_exhaustive] and new variants to LinuxDesktop** - `68879cf` (feat)
2. **Task 2: Update pipeline dispatch, watch dispatch, and tests** - `c188095` (feat)

## Files Created/Modified
- `native-theme/src/detect.rs` - Added #[non_exhaustive], five new LinuxDesktop variants, detect_linux_de() mapping
- `native-theme/src/pipeline.rs` - Updated from_linux() and from_system_async_inner() match arms, updated/added 6 detection tests

## Decisions Made
- Removed unreachable `_ =>` wildcard arms in from_linux() and from_system_async_inner(): within the same crate, `#[non_exhaustive]` does not require wildcard arms since the compiler knows all variants. Adding wildcards produced unreachable-pattern warnings.
- model/icons.rs was already updated by the parallel 75-02 plan execution, so no duplicate changes needed.
- watch/mod.rs already had a `_` wildcard arm, which correctly handles new variants without modification.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Removed unreachable wildcard arms in pipeline.rs**
- **Found during:** Task 2
- **Issue:** Plan suggested adding `_ =>` wildcards to from_linux() and from_system_async_inner(), but within the same crate #[non_exhaustive] does not force wildcard arms. All variants are listed explicitly, making `_` unreachable and producing compiler warnings.
- **Fix:** Omitted the wildcard arms since all variants are already covered
- **Files modified:** native-theme/src/pipeline.rs
- **Verification:** `cargo clippy -p native-theme -- -D warnings` passes with zero warnings
- **Committed in:** c188095

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Minor correction to avoid compiler warnings. No scope change.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- LinuxDesktop enum is now non_exhaustive and recognizes five Wayland compositors
- All pipeline dispatch paths handle the new variants correctly
- Ready for Phase 76 (type rename + crate root partition) and Phase 82 (icon API rework)

---
*Phase: 75-non-exhaustive-compile-gate-iconset-default*
*Completed: 2026-04-12*
