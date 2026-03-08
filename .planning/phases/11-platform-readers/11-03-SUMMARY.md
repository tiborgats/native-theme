---
phase: 11-platform-readers
plan: 03
subsystem: platform-readers
tags: [gnome, kde, portal, dbus, gsettings, fonts, linux, fallback]

# Dependency graph
requires:
  - phase: 10-api-breaking-changes
    provides: ThemeFonts with merge(), NativeTheme::merge(), flat ThemeColors
provides:
  - from_kde_with_portal() async overlay of portal accent on KDE kdeglobals base
  - detect_portal_backend() D-Bus activatable name detection for DE heuristic
  - parse_gnome_font_string() and read_gnome_fonts() for GNOME font reading
  - from_linux() kdeglobals fallback for Unknown DE
affects: [12-connectors, future async consumers]

# Tech tracking
tech-stack:
  added: []
  patterns: [portal-overlay-on-native-base, gsettings-subprocess-font-reading, dbus-activatable-name-detection]

key-files:
  created: []
  modified:
    - native-theme/src/gnome/mod.rs
    - native-theme/src/kde/mod.rs
    - native-theme/src/lib.rs

key-decisions:
  - "Used ashpd re-exported zbus for D-Bus access instead of adding zbus as direct dependency"
  - "detect_portal_backend exposed as pub(crate) async for async consumers rather than integrating into sync detect_linux_de"

patterns-established:
  - "Portal overlay pattern: read native config as base, overlay portal accent via NativeTheme::merge"
  - "Subprocess font reading: gsettings get with graceful fallback on missing binary"

requirements-completed: [PLAT-10, PLAT-11, PLAT-12, PLAT-13]

# Metrics
duration: 3min
completed: 2026-03-08
---

# Phase 11 Plan 03: Linux Reader Enhancement Summary

**KDE+portal accent overlay, D-Bus portal backend detection, GNOME gsettings font reading, and kdeglobals fallback for unknown DEs**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-08T07:05:03Z
- **Completed:** 2026-03-08T07:08:07Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- GNOME font reading via gsettings subprocess with parse_gnome_font_string() handling quoted/unquoted formats, fractional sizes, and edge cases (9 tests)
- KDE+portal overlay function that reads kdeglobals base then overlays portal accent color on accent/selection/focus_ring/primary_background fields
- D-Bus portal backend detection via activatable service names for DE heuristic supplementation
- from_linux() kdeglobals fallback: Unknown DE tries kdeglobals file before Adwaita preset
- All four feature combinations (kde+portal, kde-only, portal-only, none) compile and test cleanly

## Task Commits

Each task was committed atomically:

1. **Task 1: GNOME font reading and parse_gnome_font_string** - `d36c947` (feat)
2. **Task 2: KDE+portal overlay, portal backend detection, from_linux() fallback** - `d2bddd2` (feat)

## Files Created/Modified
- `native-theme/src/gnome/mod.rs` - Added parse_gnome_font_string(), read_gnome_fonts(), from_kde_with_portal(), detect_portal_backend(); updated from_gnome() to merge fonts; added 11 new tests
- `native-theme/src/kde/mod.rs` - Changed from_kde_content from private to pub(crate) for reuse
- `native-theme/src/lib.rs` - Updated from_linux() with kdeglobals fallback for Unknown DE; added from_kde_with_portal re-export; added 2 new tests

## Decisions Made
- Used ashpd's re-exported zbus (ashpd::zbus) for D-Bus access instead of adding zbus as a direct dependency, avoiding version management overhead
- detect_portal_backend() is pub(crate) async rather than integrated into the sync detect_linux_de(), since D-Bus calls require an async runtime -- async consumers can call it when XDG_CURRENT_DESKTOP returns Unknown

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 11 (Platform Readers) is now complete with all 3 plans executed
- All platform readers (macOS, Windows, GNOME, KDE) enhanced with the planned features
- Ready for phase 12 (Connectors)

## Self-Check: PASSED

All files exist, all commits verified.

---
*Phase: 11-platform-readers*
*Completed: 2026-03-08*
