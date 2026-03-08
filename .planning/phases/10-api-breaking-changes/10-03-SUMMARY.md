---
phase: 10-api-breaking-changes
plan: 03
subsystem: api
tags: [geometry, serde, toml, presets]

requires:
  - phase: 10-api-breaking-changes (plan 02)
    provides: NativeTheme preset API methods
provides:
  - ThemeGeometry with 7 fields (radius, radius_lg, frame_width, disabled_opacity, border_opacity, scroll_width, shadow)
  - All 17 presets with radius_lg and shadow values in both variants
affects: [phase-14-toolkit-connectors]

tech-stack:
  added: []
  patterns: [geometry field addition with preset-wide propagation]

key-files:
  created: []
  modified:
    - native-theme/src/model/geometry.rs
    - native-theme/src/presets/*.toml (all 17)
    - native-theme/tests/preset_loading.rs

key-decisions:
  - "radius_lg values range 8.0-16.0 based on each platform's design language"
  - "shadow is bool (not opacity) since all platforms use drop shadows"

patterns-established:
  - "New geometry fields: add to struct, impl_merge!, all presets, and preset_loading tests"

requirements-completed: [API-07, API-08]

duration: 2min
completed: 2026-03-08
---

# Phase 10 Plan 03: Geometry Fields Summary

**ThemeGeometry extended with radius_lg (Option<f32>) and shadow (Option<bool>), all 17 presets updated with platform-specific values**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-08T05:39:17Z
- **Completed:** 2026-03-08T05:41:25Z
- **Tasks:** 2
- **Files modified:** 19

## Accomplishments
- ThemeGeometry struct extended from 5 to 7 fields with radius_lg and shadow
- impl_merge! updated to include both new fields with full merge semantics
- All 17 preset TOML files updated with radius_lg (8.0-16.0) and shadow (true) in both light and dark geometry sections
- Unit tests added for default-to-none, merge behavior, and TOML round-trip of new fields
- Integration test assertions added to verify all presets include new fields

## Task Commits

Each task was committed atomically:

1. **Task 1: Add radius_lg and shadow to ThemeGeometry** - `07f906a` (feat)
2. **Task 2: Add geometry values to all presets and update tests** - `e24f4d5` (feat)

## Files Created/Modified
- `native-theme/src/model/geometry.rs` - Added radius_lg: Option<f32> and shadow: Option<bool> fields, updated impl_merge!, added 3 new tests
- `native-theme/src/presets/default.toml` - Added radius_lg = 12.0 and shadow = true to both variants
- `native-theme/src/presets/kde-breeze.toml` - Added radius_lg = 8.0 and shadow = true
- `native-theme/src/presets/adwaita.toml` - Added radius_lg = 14.0 and shadow = true
- `native-theme/src/presets/windows-11.toml` - Added radius_lg = 8.0 and shadow = true
- `native-theme/src/presets/macos-sonoma.toml` - Added radius_lg = 10.0 and shadow = true
- `native-theme/src/presets/material.toml` - Added radius_lg = 16.0 and shadow = true
- `native-theme/src/presets/ios.toml` - Added radius_lg = 13.0 and shadow = true
- `native-theme/src/presets/catppuccin-latte.toml` - Added radius_lg = 12.0 and shadow = true
- `native-theme/src/presets/catppuccin-frappe.toml` - Added radius_lg = 12.0 and shadow = true
- `native-theme/src/presets/catppuccin-macchiato.toml` - Added radius_lg = 12.0 and shadow = true
- `native-theme/src/presets/catppuccin-mocha.toml` - Added radius_lg = 12.0 and shadow = true
- `native-theme/src/presets/nord.toml` - Added radius_lg = 12.0 and shadow = true
- `native-theme/src/presets/dracula.toml` - Added radius_lg = 12.0 and shadow = true
- `native-theme/src/presets/gruvbox.toml` - Added radius_lg = 8.0 and shadow = true
- `native-theme/src/presets/solarized.toml` - Added radius_lg = 8.0 and shadow = true
- `native-theme/src/presets/tokyo-night.toml` - Added radius_lg = 12.0 and shadow = true
- `native-theme/src/presets/one-dark.toml` - Added radius_lg = 8.0 and shadow = true
- `native-theme/tests/preset_loading.rs` - Added radius_lg and shadow assertions to all_presets_have_geometry test

## Decisions Made
- radius_lg values chosen per platform design language (8.0 for conservative UIs like KDE/Windows, 14-16 for modern rounded UIs like Adwaita/Material)
- shadow field is boolean (not opacity float) since the question is whether the platform uses shadows, not how opaque they are

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- ThemeGeometry now complete with 7 fields for v0.2
- All presets have full geometry data including radius_lg and shadow
- Ready for remaining Phase 10 plans or Phase 14 toolkit connectors

---
*Phase: 10-api-breaking-changes*
*Completed: 2026-03-08*
