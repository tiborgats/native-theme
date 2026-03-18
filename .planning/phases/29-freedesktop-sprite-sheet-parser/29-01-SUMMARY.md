---
phase: 29-freedesktop-sprite-sheet-parser
plan: 01
subsystem: icons
tags: [svg, viewbox, sprite-sheet, freedesktop, animation, linux]

# Dependency graph
requires:
  - phase: 28-bundled-svg-spinner-frames
    provides: adwaita_spinner() bundled fallback and AnimatedIcon types
provides:
  - parse_sprite_sheet() pure function for vertical SVG sprite sheet frame extraction via viewBox rewriting
  - load_freedesktop_spinner() two-pass loader (plain name for sprite sheets, symbolic for single-frame)
  - loading_indicator("freedesktop") runtime-first dispatch with bundled Adwaita fallback
affects: [freedesktop-icons, loading-animations]

# Tech tracking
tech-stack:
  added: []
  patterns: [viewbox-rewriting-sprite-sheets, two-pass-animation-lookup, or-else-fallback-chain]

key-files:
  created: []
  modified:
    - native-theme/src/freedesktop.rs
    - native-theme/src/lib.rs

key-decisions:
  - "String-level viewBox find-and-replace instead of XML parser -- sufficient for attribute rewriting, no new dependency"
  - "80ms frame duration for theme-native sprite sheets (consistent with project's other frame-based spinners)"
  - "Size 22 for animation lookup (Breeze stores animated icons at 22, not 24)"

patterns-established:
  - "viewBox rewriting: extract frames from vertical SVG sprite sheets by computing frame_count = height/width and rewriting viewBox per frame"
  - "Animation-first lookup order: plain name first (sprite sheets in animations/ dirs), symbolic second (single-frame spin)"

requirements-completed: [FD-01, FD-02, FD-03, FD-04]

# Metrics
duration: 3min
completed: 2026-03-18
---

# Phase 29 Plan 01: Freedesktop Sprite Sheet Parser Summary

**Vertical SVG sprite sheet parser with viewBox rewriting, two-pass freedesktop spinner loader, and or_else Adwaita fallback in loading_indicator()**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-18T08:15:27Z
- **Completed:** 2026-03-18T08:18:52Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Implemented parse_sprite_sheet() pure function that splits vertical SVG sprite sheets into individual frames via viewBox attribute rewriting
- Implemented load_freedesktop_spinner() with two-pass lookup: plain "process-working" for sprite sheets (returns Frames), "process-working-symbolic" for single-frame (returns Transform::Spin)
- Wired loading_indicator("freedesktop") to try theme-native sprite sheet first, falling back to bundled Adwaita via or_else chain
- Added 8 unit tests covering sprite sheet parsing edge cases (two-frame, fifteen-frame, single-frame, non-multiple, comma-separated viewBox, content preservation, invalid SVG, no-panic integration)
- Updated existing freedesktop dispatch test to accept theme-dependent results

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement sprite sheet parser and freedesktop spinner loader** - `c8909e5` (feat)
2. **Task 2: Wire loading_indicator freedesktop match arm and update tests** - `5a92ac5` (feat)

## Files Created/Modified
- `native-theme/src/freedesktop.rs` - Added parse_sprite_sheet() and load_freedesktop_spinner() with unit tests
- `native-theme/src/lib.rs` - Updated Freedesktop match arm to use load_freedesktop_spinner().or_else(adwaita); updated and added dispatch tests

## Decisions Made
- String-level viewBox find-and-replace instead of XML parser -- the viewBox attribute has a predictable format, and only one attribute value changes per frame
- 80ms frame duration for theme-native sprite sheets, consistent with the project's Material (83ms) and macOS (83ms) spinners, faster than KDE's 200ms default
- Size 22 for animation icon lookup (Breeze animations live at 22, not 24)

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Freedesktop sprite sheet parsing is complete and tested
- The or_else fallback chain guarantees loading_indicator("freedesktop") always returns Some on Linux with system-icons feature
- Phase 29 is single-plan, so this completes the phase

---
*Phase: 29-freedesktop-sprite-sheet-parser*
*Completed: 2026-03-18*
