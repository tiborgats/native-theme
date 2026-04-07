---
phase: 51-resolution-engine-overhaul
plan: 01
subsystem: presets
tags: [toml, text-scale, typography, presets]

# Dependency graph
requires:
  - phase: 50-atomic-schema-commit
    provides: renamed text_scale fields in all structs
provides:
  - All 20 presets have explicit text_scale entries (caption, section_heading, dialog_title, display)
  - Phase 51 can safely remove ratio-based text_scale computation from resolve.rs
affects: [51-02, 51-03, resolve-engine]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "text_scale entries use preset's own font.size as base, not hardcoded"

key-files:
  created: []
  modified:
    - native-theme/src/presets/catppuccin-mocha.toml
    - native-theme/src/presets/catppuccin-latte.toml
    - native-theme/src/presets/catppuccin-frappe.toml
    - native-theme/src/presets/catppuccin-macchiato.toml
    - native-theme/src/presets/dracula.toml
    - native-theme/src/presets/gruvbox.toml
    - native-theme/src/presets/nord.toml
    - native-theme/src/presets/one-dark.toml
    - native-theme/src/presets/solarized.toml
    - native-theme/src/presets/tokyo-night.toml
    - native-theme/src/presets/ios.toml
    - native-theme/src/presets/kde-breeze.toml
    - native-theme/src/presets/kde-breeze-live.toml
    - native-theme/src/presets/macos-sonoma-live.toml
    - native-theme/src/presets/material.toml

key-decisions:
  - "Used exact resolve_text_scale_entry() ratios (0.82/1.0/1.2/2.0) to compute sizes from each preset's own font.size"
  - "Live presets use platform defaults for font.size (KDE: 10.0, macOS: 13.0) since OS reader normally provides it"
  - "Values rounded to 1 decimal place for TOML readability"

patterns-established:
  - "text_scale values are derived from preset's font.size * ratio, line_height from defaults.line_height * entry.size"

requirements-completed: []

# Metrics
duration: 5min
completed: 2026-04-07
---

# Phase 51 Plan 01: Explicit Text Scale Entries Summary

**Added explicit text_scale entries (caption/section_heading/dialog_title/display) to all 15 community presets using resolve_text_scale_entry() ratios**

## Performance

- **Duration:** 5 min
- **Started:** 2026-04-07T07:57:52Z
- **Completed:** 2026-04-07T08:03:02Z
- **Tasks:** 2
- **Files modified:** 15

## Accomplishments
- All 15 community presets now have explicit text_scale entries for both light and dark variants
- Values mathematically derived from each preset's own font.size and line_height using the existing resolve_text_scale_entry() ratios
- All 20 presets pass validate_all_presets_pass_range_checks
- Phase 51 can now safely remove ratio-based text_scale computation from resolve.rs

## Task Commits

Each task was committed atomically:

1. **Task 1: Add text_scale entries to 8 community presets (batch 1)** - `5c9a9e3` (feat)
2. **Task 2: Add text_scale entries to 7 remaining community presets (batch 2)** - `f445a19` (feat)

## Files Created/Modified
- `native-theme/src/presets/catppuccin-mocha.toml` - text_scale for font.size=14.0, line_height=1.2
- `native-theme/src/presets/catppuccin-latte.toml` - text_scale for font.size=14.0, line_height=1.2
- `native-theme/src/presets/catppuccin-frappe.toml` - text_scale for font.size=14.0, line_height=1.2
- `native-theme/src/presets/catppuccin-macchiato.toml` - text_scale for font.size=14.0, line_height=1.2
- `native-theme/src/presets/dracula.toml` - text_scale for font.size=14.0, line_height=1.2
- `native-theme/src/presets/gruvbox.toml` - text_scale for font.size=14.0, line_height=1.2
- `native-theme/src/presets/nord.toml` - text_scale for font.size=14.0, line_height=1.2
- `native-theme/src/presets/one-dark.toml` - text_scale for font.size=14.0, line_height=1.2
- `native-theme/src/presets/solarized.toml` - text_scale for font.size=14.0, line_height=1.2
- `native-theme/src/presets/tokyo-night.toml` - text_scale for font.size=14.0, line_height=1.2
- `native-theme/src/presets/material.toml` - text_scale for font.size=14.0, line_height=1.2
- `native-theme/src/presets/ios.toml` - text_scale for font.size=17.0, line_height=1.2
- `native-theme/src/presets/kde-breeze.toml` - text_scale for font.size=10.0, line_height=1.36
- `native-theme/src/presets/kde-breeze-live.toml` - text_scale for font.size=10.0 (KDE default), line_height=1.36
- `native-theme/src/presets/macos-sonoma-live.toml` - text_scale for font.size=13.0 (macOS default), line_height=1.19

## Decisions Made
- Used exact ratios from resolve_text_scale_entry(): caption=0.82x, section_heading=1.0x, dialog_title=1.2x, display=2.0x
- For live presets without font.size, used platform defaults: KDE=10.0, macOS=13.0
- Rounded all computed values to 1 decimal place for TOML readability
- Weight for caption uses body weight (400); weight for heading/dialog_title/display uses 700

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All 20 presets have explicit text_scale entries -- resolve_text_scale() ratio computation can be safely removed in Plan 51-02
- No preset relies on ratio-based computation anymore

---
*Phase: 51-resolution-engine-overhaul*
*Completed: 2026-04-07*
