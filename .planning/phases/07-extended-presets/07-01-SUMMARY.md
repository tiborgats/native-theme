---
phase: 07-extended-presets
plan: 01
subsystem: presets
tags: [toml, windows-11, macos-sonoma, material-design, ios, fluent-design, platform-themes]

# Dependency graph
requires:
  - phase: 02-core-presets
    provides: "preset() / list_presets() API, TOML deserialization, include_str!() pattern"
provides:
  - "4 platform preset TOML files (windows-11, macos-sonoma, material, ios)"
  - "7-entry preset registry (up from 3)"
affects: [07-02-PLAN, tests/preset_loading]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Platform-specific font families and geometry per preset"]

key-files:
  created:
    - src/presets/windows-11.toml
    - src/presets/macos-sonoma.toml
    - src/presets/material.toml
    - src/presets/ios.toml
  modified:
    - src/presets.rs

key-decisions:
  - "Platform-appropriate spacing values (Windows/Material use 8px base, macOS/iOS use tighter 6/8px base)"
  - "Dark variant status colors adjusted for readability (lighter tones on dark backgrounds)"
  - "Material Design 3 disabled_opacity 0.38 matches official spec"

patterns-established:
  - "Platform presets use native font families (Segoe UI, SF Pro, Roboto)"
  - "Each preset TOML follows exact same section ordering as adwaita.toml"

# Metrics
duration: 2min
completed: 2026-03-07
---

# Phase 7 Plan 1: Platform Presets Summary

**4 platform preset TOML files (Windows 11 Fluent, macOS Sonoma, Material Design 3, iOS) with accurate system colors wired into 7-entry preset registry**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-07T21:46:14Z
- **Completed:** 2026-03-07T21:48:43Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- Created 4 complete platform preset TOML files with all 36 color roles for both light and dark variants
- Updated presets.rs registry from 3 to 7 presets with include_str!(), match arms, and PRESET_NAMES
- All 12 unit tests pass including new assertions for 7-preset count and correct display names

## Task Commits

Each task was committed atomically:

1. **Task 1: Create 4 platform preset TOML files** - `6d8d239` (feat)
2. **Task 2: Wire platform presets into presets.rs registry** - `b9e4128` (feat)

## Files Created/Modified
- `src/presets/windows-11.toml` - Windows 11 Fluent Design light/dark theme (Segoe UI, radius 4)
- `src/presets/macos-sonoma.toml` - macOS Sonoma system colors light/dark (SF Pro, radius 6)
- `src/presets/material.toml` - Material Design 3 baseline light/dark (Roboto, radius 12)
- `src/presets/ios.toml` - iOS system colors light/dark (SF Pro, radius 10)
- `src/presets.rs` - Updated with 4 new constants, match arms, PRESET_NAMES (7 total), updated tests

## Decisions Made
- Platform-appropriate spacing values: Windows/Material use 8px small spacing, macOS/iOS use tighter 6/8px base
- Dark variant status colors use lighter readable tones (e.g., Windows dark danger #ff9999 instead of #c42b1c) for contrast on dark backgrounds
- Material Design 3 disabled_opacity set to 0.38 matching official M3 spec (not 0.5 like other presets)
- iOS geometry frame_width 0.5 for retina hairline borders as specified in plan

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- 4 platform presets in place, ready for Plan 02 to add 11 community presets
- Integration tests (tests/preset_loading.rs) will need count update from 3 to 18 after Plan 02 completes
- All existing iterating tests (core colors, font sizes, loadability) automatically cover the 4 new presets

## Self-Check: PASSED

All 6 files verified present. Both task commits (6d8d239, b9e4128) confirmed in git log.

---
*Phase: 07-extended-presets*
*Completed: 2026-03-07*
