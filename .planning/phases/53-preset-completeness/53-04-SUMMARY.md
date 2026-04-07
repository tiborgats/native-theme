---
phase: 53-preset-completeness
plan: 04
subsystem: presets
tags: [toml, community-presets, text-scale, interactive-state-colors, gnome]

# Dependency graph
requires:
  - phase: 52-interactive-state-colors
    provides: "soft_option and inherited state color fields on widget structs"
  - phase: 51-resolve-inheritance
    provides: "text_scale entries in all presets (ratio-derived, to be corrected)"
provides:
  - "6 community presets with GNOME text_scale values (9/400, 11/700, 15/800, 20/800)"
  - "6 community presets with 51 explicit interactive state color fields per variant"
affects: [53-preset-completeness, preset-validation]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Community preset state color derivation: darken 8% hover / 15% active (light), lighten 10% hover / 18% active (dark)"
    - "Scrollbar thumb_hover derived from blend(border, muted, 0.35) not from muted_color alone (wrong_safety_nets)"

key-files:
  created: []
  modified:
    - native-theme/src/presets/nord.toml
    - native-theme/src/presets/dracula.toml
    - native-theme/src/presets/gruvbox.toml
    - native-theme/src/presets/solarized.toml
    - native-theme/src/presets/tokyo-night.toml
    - native-theme/src/presets/one-dark.toml

key-decisions:
  - "GNOME text_scale values used for all 6 community presets (Linux-focused color schemes)"
  - "Consistent derivation pattern across all 6 presets: hover=darken/lighten 8-10%, active=15-18%, disabled=desaturated"
  - "Scrollbar thumb_hover uses blend of border+muted, not muted_color directly (per wrong_safety_nets)"

patterns-established:
  - "Community preset hover derivation: light themes darken, dark themes lighten, consistent percentages"
  - "All state colors pre-computed to solid hex (no alpha overlays in state color fields)"

requirements-completed: [PRESET-02, PRESET-03]

# Metrics
duration: 10min
completed: 2026-04-07
---

# Phase 53 Plan 04: Community Presets Summary

**GNOME text_scale values and 51 palette-derived interactive state colors added to nord, dracula, gruvbox, solarized, tokyo-night, one-dark**

## Performance

- **Duration:** 10 min
- **Started:** 2026-04-07T13:26:59Z
- **Completed:** 2026-04-07T13:37:41Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Replaced ratio-derived text_scale values (11.5/14.0/16.8/28.0) with GNOME Adwaita values (9.0/11.0/15.0/20.0) in all 6 community presets
- Removed explicit line_height entries from text_scale sections (inherited from defaults.line_height=1.2)
- Added 51 interactive state color fields to both light and dark variants of all 6 presets (108 entries per preset total)
- All state colors derived from each preset's own palette using consistent derivation patterns

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix text_scale for all 6 community presets** - `3ca3855` (feat)
2. **Task 2: Add interactive state colors to all 6 community presets** - `0250b3a` (feat)

## Files Created/Modified
- `native-theme/src/presets/nord.toml` - GNOME text_scale + 51 state colors per variant
- `native-theme/src/presets/dracula.toml` - GNOME text_scale + 51 state colors per variant
- `native-theme/src/presets/gruvbox.toml` - GNOME text_scale + 51 state colors per variant
- `native-theme/src/presets/solarized.toml` - GNOME text_scale + 51 state colors per variant
- `native-theme/src/presets/tokyo-night.toml` - GNOME text_scale + 51 state colors per variant
- `native-theme/src/presets/one-dark.toml` - GNOME text_scale + 51 state colors per variant

## Decisions Made
- Used GNOME text_scale values (caption=9/400, section_heading=11/700, dialog_title=15/800, display=20/800) for all 6 community presets since they are Linux-focused desktop color schemes
- Consistent derivation pattern: light themes darken for hover (8%), darken more for active (15%); dark themes lighten (10%/18%)
- Scrollbar thumb_hover derived from blend(border, muted) rather than copying muted_color directly (avoids wrong_safety_nets pattern)
- Solarized light scrollbar uses darken(muted) instead of blend since border=muted in that preset

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- All 6 community presets now have correct text_scale and explicit state colors
- Plan 05 (remaining presets or validation) can proceed
- All presets pass resolve() -> validate() pipeline

## Self-Check: PASSED

- All 6 preset files exist and are modified
- Commit 3ca3855 (Task 1) exists
- Commit 0250b3a (Task 2) exists
- SUMMARY.md created
- All presets pass resolve() -> validate()

---
*Phase: 53-preset-completeness*
*Completed: 2026-04-07*
