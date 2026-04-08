---
phase: 59-implement-chapter-2-of-docs-todo-v0-5-5-pt-px-md
plan: 02
subsystem: schema
tags: [toml, presets, font-size, size_pt, size_px, property-registry]

# Dependency graph
requires:
  - phase: 59-01
    provides: FontSize enum and proxy struct serde (FontSpecRaw, TextScaleEntryRaw) that deserialize size_pt/size_px keys
provides:
  - All 20 TOML presets use self-documenting size_pt or size_px keys
  - property-registry.toml documents the new Font and TextScaleEntry fields
affects: [59-03]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Platform presets use size_pt (typographic points); community presets use size_px (logical pixels)"

key-files:
  created: []
  modified:
    - native-theme/src/presets/kde-breeze.toml
    - native-theme/src/presets/kde-breeze-live.toml
    - native-theme/src/presets/adwaita.toml
    - native-theme/src/presets/adwaita-live.toml
    - native-theme/src/presets/macos-sonoma.toml
    - native-theme/src/presets/macos-sonoma-live.toml
    - native-theme/src/presets/windows-11.toml
    - native-theme/src/presets/windows-11-live.toml
    - native-theme/src/presets/ios.toml
    - native-theme/src/presets/catppuccin-mocha.toml
    - native-theme/src/presets/catppuccin-frappe.toml
    - native-theme/src/presets/catppuccin-latte.toml
    - native-theme/src/presets/catppuccin-macchiato.toml
    - native-theme/src/presets/one-dark.toml
    - native-theme/src/presets/nord.toml
    - native-theme/src/presets/dracula.toml
    - native-theme/src/presets/gruvbox.toml
    - native-theme/src/presets/solarized.toml
    - native-theme/src/presets/tokyo-night.toml
    - native-theme/src/presets/material.toml
    - docs/property-registry.toml

key-decisions:
  - "Windows unit comments removed since _pt suffix is self-documenting"

patterns-established:
  - "size_pt for platform presets (OS-reported typographic points)"
  - "size_px for community presets (hand-authored logical pixels)"

requirements-completed: []

# Metrics
duration: 3min
completed: 2026-04-08
---

# Phase 59 Plan 02: TOML Preset Font Size Key Rename Summary

**Renamed all font size keys from bare `size` to `size_pt` (9 platform presets) or `size_px` (11 community presets), making units self-documenting**

## Performance

- **Duration:** 3 min
- **Started:** 2026-04-08T14:18:58Z
- **Completed:** 2026-04-08T14:22:20Z
- **Tasks:** 2
- **Files modified:** 21

## Accomplishments
- Renamed 88 font size keys across 9 platform presets from `size` to `size_pt`
- Renamed 132 font size keys across 11 community presets from `size` to `size_px`
- Updated property-registry.toml Font and TextScaleEntry structures with size_pt/size_px documentation
- Removed redundant unit comments from Windows 11 presets (suffix is self-documenting)

## Task Commits

Each task was committed atomically:

1. **Task 1: Rename font size keys in all 20 TOML presets** - `7c31a79` (feat)
2. **Task 2: Update property-registry.toml documentation** - `5623315` (docs)

## Files Created/Modified
- `native-theme/src/presets/*.toml` (20 files) - Renamed `size =` to `size_pt =` or `size_px =` in font and text_scale sections
- `docs/property-registry.toml` - Updated Font and TextScaleEntry structures with size_pt/size_px fields and documentation

## Decisions Made
- Windows unit comments ("Font sizes are in typographic points...") removed since the `_pt` suffix makes the unit self-documenting

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All 20 presets now use self-documenting size keys compatible with the FontSpecRaw/TextScaleEntryRaw proxy structs from Plan 01
- Plan 03 (test updates and compilation verification) can proceed to validate the full pipeline

## Self-Check: PASSED

All files exist. All commits verified (7c31a79, 5623315).

---
*Phase: 59-implement-chapter-2-of-docs-todo-v0-5-5-pt-px-md*
*Completed: 2026-04-08*
