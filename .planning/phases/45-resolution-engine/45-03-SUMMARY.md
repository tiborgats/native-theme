---
phase: 45-resolution-engine
plan: 03
subsystem: presets
tags: [presets, toml, resolve, validate, enrichment, pipeline]

# Dependency graph
requires:
  - phase: 45-resolution-engine
    plan: 02
    provides: "resolve() and validate() methods on ThemeVariant"
provides:
  - "All 17 bundled presets pass resolve()+validate() for both light and dark variants"
  - "5 new integration tests proving end-to-end resolution pipeline"
  - "Community presets have fonts, mono_font, line_height, accent_foreground, icon_sizes, focus_ring, accessibility defaults"
affects: [46-connector-updates, 47-iced-connector, 48-gpui-connector]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Enriched TOML preset structure with all non-derived fields", "icon_set = freedesktop for community/default presets"]

key-files:
  created: []
  modified:
    - native-theme/src/presets.rs
    - native-theme/src/presets/catppuccin-mocha.toml
    - native-theme/src/presets/catppuccin-frappe.toml
    - native-theme/src/presets/catppuccin-macchiato.toml
    - native-theme/src/presets/catppuccin-latte.toml
    - native-theme/src/presets/nord.toml
    - native-theme/src/presets/dracula.toml
    - native-theme/src/presets/gruvbox.toml
    - native-theme/src/presets/solarized.toml
    - native-theme/src/presets/tokyo-night.toml
    - native-theme/src/presets/one-dark.toml
    - native-theme/src/presets/default.toml
    - native-theme/src/presets/kde-breeze.toml
    - native-theme/src/presets/adwaita.toml
    - native-theme/src/presets/windows-11.toml
    - native-theme/src/presets/macos-sonoma.toml
    - native-theme/src/presets/material.toml
    - native-theme/src/presets/ios.toml

key-decisions:
  - "icon_set = freedesktop added to community and default presets since validate() requires icon_set"
  - "Platform-appropriate dialog button_order: KDE/macOS use leading_affirmative, GNOME/Windows use trailing_affirmative"
  - "Material/iOS switch and segmented_control use platform-specific larger dimensions"

patterns-established:
  - "All presets must provide every non-derived field for resolve()/validate() pipeline"
  - "Community presets use Inter/JetBrains Mono fonts at 14.0 size, weight 400"

requirements-completed: [PRESET-03]

# Metrics
duration: 27min
completed: 2026-03-27
---

# Phase 45 Plan 03: Preset Enrichment Summary

**All 17 bundled presets enriched with 80+ non-derived fields each, passing complete resolve()+validate() pipeline with 5 integration tests**

## Performance

- **Duration:** 27 min
- **Started:** 2026-03-27T09:49:11Z
- **Completed:** 2026-03-27T10:16:24Z
- **Tasks:** 2
- **Files modified:** 18

## Accomplishments
- Enriched all 17 preset TOMLs (10 community + 7 platform) with all non-derived fields required by validate(): fonts, line_height, accent_foreground, icon_sizes, focus_ring geometry, accessibility defaults, and 15+ widget-specific geometry/behavior fields
- Added icon_set = "freedesktop" to community and default presets for pipeline compatibility
- Platform presets enriched with platform-appropriate values (KDE leading_affirmative button order, Material larger switch dimensions, iOS overlay scrollbars, etc.)
- 5 new integration tests proving: all presets resolve+validate, accent-derived propagation, complete ResolvedTheme field verification, font sub-field inheritance, text_scale inheritance with computed line_height
- Total: 312 lib tests passing (was 307)

## Task Commits

Each task was committed atomically:

1. **Task 1: Enrich all 17 preset TOMLs with missing non-derived fields** - `532ff1c` (feat)
2. **Task 2: Add integration tests proving all 17 presets pass resolve()+validate()** - `87d80b6` (test)

## Files Created/Modified
- `native-theme/src/presets.rs` - Updated icon_set test (community presets now freedesktop), added 5 integration tests
- `native-theme/src/presets/catppuccin-mocha.toml` - Enriched with fonts, icon_sizes, switch, dialog, spinner, combo_box, segmented_control, card, expander, link, accessibility defaults
- `native-theme/src/presets/catppuccin-frappe.toml` - Same enrichment pattern
- `native-theme/src/presets/catppuccin-macchiato.toml` - Same enrichment pattern
- `native-theme/src/presets/catppuccin-latte.toml` - Same enrichment pattern
- `native-theme/src/presets/nord.toml` - Same enrichment pattern
- `native-theme/src/presets/dracula.toml` - Same enrichment pattern
- `native-theme/src/presets/gruvbox.toml` - Same enrichment pattern
- `native-theme/src/presets/solarized.toml` - Same enrichment pattern
- `native-theme/src/presets/tokyo-night.toml` - Same enrichment pattern
- `native-theme/src/presets/one-dark.toml` - Same enrichment pattern
- `native-theme/src/presets/default.toml` - Enriched with font weight, icon_sizes, widget geometry, icon_set=freedesktop
- `native-theme/src/presets/kde-breeze.toml` - Enriched with font weight, icon_sizes, new widget sections
- `native-theme/src/presets/adwaita.toml` - Enriched with font weight, icon_sizes, new widget sections
- `native-theme/src/presets/windows-11.toml` - Enriched with font weight, icon_sizes, new widget sections
- `native-theme/src/presets/macos-sonoma.toml` - Enriched with font weight, icon_sizes, new widget sections
- `native-theme/src/presets/material.toml` - Enriched with font weight, icon_sizes, Material-specific widget sizes
- `native-theme/src/presets/ios.toml` - Enriched with font weight, icon_sizes, iOS-specific widget sizes

## Decisions Made
- icon_set = "freedesktop" added to all community presets and the "default" preset: validate() requires icon_set to be present, and freedesktop is the most portable cross-platform default
- Updated existing test `icon_set_community_presets_are_none` to `icon_set_community_and_default_presets_are_freedesktop` to reflect the new requirement
- Platform-specific dialog button_order: KDE and macOS presets use `leading_affirmative` (affirmative button on the left), GNOME/Windows use `trailing_affirmative` (right)
- Material Design preset uses larger switch (52x32 track), segmented_control (40px), and input (56px min_height) matching M3 spec
- iOS preset uses platform-accurate switch dimensions (51x31), overlay scrollbars, and 44px touch targets
- link.underline = false for Material and iOS (these platforms use color-only link styling), true for all others

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed button_order serde value**
- **Found during:** Task 1
- **Issue:** Plan specified `button_order = "TrailingAffirmative"` but serde uses `rename_all = "snake_case"` requiring `trailing_affirmative`
- **Fix:** Used `"trailing_affirmative"` / `"leading_affirmative"` in all preset TOMLs
- **Files modified:** All 17 preset TOML files
- **Verification:** Presets parse without error
- **Committed in:** 532ff1c (Task 1 commit)

**2. [Rule 3 - Blocking] Added icon_set to community/default presets**
- **Found during:** Task 1
- **Issue:** validate() requires `icon_set` field but community presets had None; existing test asserted None
- **Fix:** Added `icon_set = "freedesktop"` to community and default presets; updated test to verify "freedesktop" instead of None
- **Files modified:** All 17 preset TOML files, native-theme/src/presets.rs
- **Verification:** All presets pass resolve()+validate(), updated test passes
- **Committed in:** 532ff1c (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (1 bug, 1 blocking)
**Impact on plan:** Both fixes necessary for pipeline correctness. No scope creep.

## Issues Encountered
- Pre-existing broken integration test `merge_behavior` references old removed types (ThemeColors, ThemeFonts, ThemeGeometry from pre-Phase-44 refactor). Not caused by this plan. Not fixed (out of scope).

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All 17 presets now pass the complete resolve()+validate() pipeline
- Phase 45 (Resolution Engine) is fully complete: resolved types (Plan 01), resolve/validate engine (Plan 02), enriched presets (Plan 03)
- Ready for Phase 46 (connector updates to use ResolvedTheme)

---
*Phase: 45-resolution-engine*
*Completed: 2026-03-27*
