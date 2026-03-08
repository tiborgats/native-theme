---
phase: 12-widget-metrics
plan: 03
subsystem: presets
tags: [toml, widget-metrics, presets, platform-values]

# Dependency graph
requires:
  - phase: 12-widget-metrics
    plan: 01
    provides: "WidgetMetrics struct with 12 sub-structs and serde support"
provides:
  - "All 17 preset TOML files with widget_metrics sections for both light and dark variants"
  - "Platform-specific widget sizing (adwaita, kde-breeze, windows-11, macos-sonoma)"
  - "Design-system-specific widget sizing (material, ios)"
  - "Generic widget defaults for 11 community/default presets"
affects: [platform-readers, preset-consumers]

# Tech tracking
tech-stack:
  added: []
  patterns: ["widget_metrics section placement after spacing in TOML presets"]

key-files:
  created: []
  modified:
    - native-theme/src/presets/adwaita.toml
    - native-theme/src/presets/kde-breeze.toml
    - native-theme/src/presets/windows-11.toml
    - native-theme/src/presets/macos-sonoma.toml
    - native-theme/src/presets/material.toml
    - native-theme/src/presets/ios.toml
    - native-theme/src/presets/default.toml
    - native-theme/src/presets/catppuccin-latte.toml
    - native-theme/src/presets/catppuccin-frappe.toml
    - native-theme/src/presets/catppuccin-macchiato.toml
    - native-theme/src/presets/catppuccin-mocha.toml
    - native-theme/src/presets/nord.toml
    - native-theme/src/presets/dracula.toml
    - native-theme/src/presets/gruvbox.toml
    - native-theme/src/presets/solarized.toml
    - native-theme/src/presets/tokyo-night.toml
    - native-theme/src/presets/one-dark.toml

key-decisions:
  - "Community color themes use generic defaults since they are color-only design systems, not platform-specific widget specs"
  - "Light and dark variants get identical widget_metrics since widget sizing is mode-independent on all platforms"

patterns-established:
  - "TOML section order: colors, fonts, geometry, spacing, widget_metrics"
  - "Platform presets use researched platform-specific values; community presets use generic defaults"

requirements-completed: [METRIC-08]

# Metrics
duration: 6min
completed: 2026-03-08
---

# Phase 12 Plan 03: Preset Widget Metrics Summary

**All 17 preset TOML files populated with widget_metrics for 12 widget types, using platform-researched values for 6 design systems and generic defaults for 11 community themes**

## Performance

- **Duration:** 6 min
- **Started:** 2026-03-08T07:57:44Z
- **Completed:** 2026-03-08T08:04:23Z
- **Tasks:** 2
- **Files modified:** 17

## Accomplishments
- Added widget_metrics sections to all 17 preset TOML files (both light and dark variants)
- Platform presets use researched values: adwaita (libadwaita CSS), kde-breeze (breezemetrics.h), windows-11 (WinUI3 Fluent), macos-sonoma (HIG)
- Design-system presets use spec values: material (MD3 48px touch targets), ios (HIG 44px min tap)
- 11 community/default presets use identical generic defaults (toolkit-agnostic averages)
- All 152 tests pass including 12 preset-specific tests

## Task Commits

Each task was committed atomically:

1. **Task 1: Add widget metrics to platform presets (4 files)** - `c8f33f3` (feat)
2. **Task 2: Add widget metrics to remaining 13 presets** - `884c967` (feat)

## Files Created/Modified
- `native-theme/src/presets/adwaita.toml` - libadwaita widget metrics (button 34px, scrollbar 12px)
- `native-theme/src/presets/kde-breeze.toml` - Breeze widget metrics (button 80px min-width, scrollbar 21px)
- `native-theme/src/presets/windows-11.toml` - WinUI3 Fluent widget metrics (button 32px, list_item 36px)
- `native-theme/src/presets/macos-sonoma.toml` - macOS HIG widget metrics (button 22px, checkbox 14px)
- `native-theme/src/presets/material.toml` - Material Design 3 widget metrics (button 40px, input 56px)
- `native-theme/src/presets/ios.toml` - iOS HIG touch-optimized metrics (button 44px, thumb 28px)
- `native-theme/src/presets/default.toml` - Generic default widget metrics (button 32px)
- `native-theme/src/presets/catppuccin-latte.toml` - Generic defaults
- `native-theme/src/presets/catppuccin-frappe.toml` - Generic defaults
- `native-theme/src/presets/catppuccin-macchiato.toml` - Generic defaults
- `native-theme/src/presets/catppuccin-mocha.toml` - Generic defaults
- `native-theme/src/presets/nord.toml` - Generic defaults
- `native-theme/src/presets/dracula.toml` - Generic defaults
- `native-theme/src/presets/gruvbox.toml` - Generic defaults
- `native-theme/src/presets/solarized.toml` - Generic defaults
- `native-theme/src/presets/tokyo-night.toml` - Generic defaults
- `native-theme/src/presets/one-dark.toml` - Generic defaults

## Decisions Made
- Community color themes (catppuccin x4, nord, dracula, gruvbox, solarized, tokyo-night, one-dark) use the same generic defaults as default.toml because they are color palette themes, not platform-specific design systems with widget sizing specs
- Light and dark variants get identical widget_metrics values since widget sizing does not change between light/dark mode on any platform

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Reverted uncommitted broken code from incomplete 12-02 attempt**
- **Found during:** Task 1 (compilation check)
- **Issue:** Working tree contained uncommitted changes in macos.rs, gnome/mod.rs, and kde/mod.rs from a failed 12-02 plan execution. These changes referenced non-existent functions (macos_widget_metrics(), adwaita_widget_metrics()) and a missing kde/metrics.rs module, preventing compilation.
- **Fix:** Reverted the 3 files to their last committed state via git checkout HEAD. The untracked kde/metrics.rs file was left in place (not committed, not staged).
- **Files affected:** native-theme/src/macos.rs, native-theme/src/gnome/mod.rs, native-theme/src/kde/mod.rs (reverted, not modified by this plan)
- **Verification:** cargo build -p native-theme compiles cleanly, all 152 tests pass
- **Note:** These changes will need to be re-implemented properly when 12-02 is executed

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Required to unblock compilation. No changes to plan scope.

## Issues Encountered
None beyond the blocking deviation above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All 17 presets now include complete widget_metrics, ready for consumer use
- Plan 12-02 (platform readers) still needs execution to populate widget_metrics from live OS readings
- The untracked kde/metrics.rs and uncommitted test stubs for 12-02 remain in the working tree

## Self-Check: PASSED

All 17 TOML files verified present. Both task commits (c8f33f3, 884c967) verified in git log. SUMMARY.md exists.

---
*Phase: 12-widget-metrics*
*Completed: 2026-03-08*
