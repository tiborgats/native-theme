---
phase: 53-preset-completeness
plan: 03
subsystem: presets
tags: [toml, text-scale, interactive-colors, ios, material, catppuccin, apple-hig, md3, gnome-adwaita]

# Dependency graph
requires:
  - phase: 52-interactive-state-colors
    provides: "Rust struct fields for 51 interactive state colors (soft_option + inherited)"
provides:
  - "Complete iOS preset with Apple HIG text_scale and 49 interactive state color fields"
  - "Complete Material preset with MD3 text_scale and 49 interactive state color fields"
  - "Complete Catppuccin Latte/Frappe/Macchiato/Mocha presets with GNOME text_scale and 49 interactive state color fields each"
affects: [53-04, 53-05, preset-completeness]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Catppuccin palette-to-state-color derivation: Surface1 hover, Surface2 active, accent focus/border"]

key-files:
  created: []
  modified:
    - "native-theme/src/presets/ios.toml"
    - "native-theme/src/presets/material.toml"
    - "native-theme/src/presets/catppuccin-latte.toml"
    - "native-theme/src/presets/catppuccin-frappe.toml"
    - "native-theme/src/presets/catppuccin-macchiato.toml"
    - "native-theme/src/presets/catppuccin-mocha.toml"

key-decisions:
  - "iOS text_scale uses Apple HIG iOS type ramp (caption=12, section_heading=15/700, dialog_title=22, display=34)"
  - "Material text_scale uses MD3 type ramp with MD3 line heights (caption=11/16, section_heading=16/24, dialog_title=24/32, display=36/44)"
  - "Catppuccin text_scale uses GNOME/Adwaita values with no explicit line_height (inherits defaults.line_height=1.2)"
  - "Catppuccin light variants use Latte palette with flavor-specific accent colors for hover/focus border"
  - "Catppuccin dark variants use flavor-specific palette (Frappe/Macchiato/Mocha) for all state colors"

patterns-established:
  - "Catppuccin state color derivation: hover=Surface1/next-step-up, active=Surface2, disabled=50% alpha overlay, unchecked=Base+Surface2 border"
  - "Community preset disabled_text_color mirrors defaults.disabled_text_color from same file"

requirements-completed: [PRESET-02, PRESET-03]

# Metrics
duration: 15min
completed: 2026-04-07
---

# Phase 53 Plan 03: iOS/Material/Catppuccin State Colors Summary

**Apple HIG and MD3 text_scale corrections plus 49 interactive state color fields across 6 presets (ios, material, 4 catppuccin flavors)**

## Performance

- **Duration:** 15 min
- **Started:** 2026-04-07T13:27:19Z
- **Completed:** 2026-04-07T13:42:28Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments
- Replaced ratio-derived text_scale values in all 6 presets with platform-appropriate values (iOS Apple HIG, Material Design 3, GNOME/Adwaita)
- Added 49 interactive state color fields to both light and dark variants of all 6 presets (588 new lines)
- All 4 catppuccin light variants share consistent Latte-based state colors; dark variants use flavor-specific palettes
- All 423 tests pass, pre-release-check.sh green

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix text_scale for ios, material, and all 4 catppuccin presets** - `8e27279` (feat)
2. **Task 2: Add interactive state colors to all 6 presets** - `0d00a51` (feat)

## Files Created/Modified
- `native-theme/src/presets/ios.toml` - Apple HIG text_scale (12/15/22/34), iOS-style state colors (system blue accent, semi-transparent disabled)
- `native-theme/src/presets/material.toml` - MD3 text_scale (11/16/24/36 with MD3 line heights), state layer overlays (8% hover, 12% active)
- `native-theme/src/presets/catppuccin-latte.toml` - GNOME text_scale (9/11/15/20), Latte light + Frappe dark state colors
- `native-theme/src/presets/catppuccin-frappe.toml` - GNOME text_scale, Latte light + Frappe dark state colors (identical to catppuccin-latte)
- `native-theme/src/presets/catppuccin-macchiato.toml` - GNOME text_scale, Latte light + Macchiato dark state colors
- `native-theme/src/presets/catppuccin-mocha.toml` - GNOME text_scale, Latte light + Mocha dark state colors

## Decisions Made
- iOS text_scale: Used Apple HIG iOS-specific sizes (caption=12pt Regular, section_heading=15pt Bold, dialog_title=22pt Regular, display=34pt Regular) rather than copying macOS values
- Material text_scale: Used MD3 type ramp with explicit MD3 line heights (16sp/24sp/32sp/44sp) instead of 1.2x multiplier
- Catppuccin text_scale: Omitted explicit line_height entries to inherit from defaults.line_height (1.2), matching adwaita's sparse text_scale pattern
- Catppuccin light hover/focus border colors use each flavor's accent color (Latte=#1e66f5, Frappe/Latte-dark=#8caaee, Macchiato=#8aadf4, Mocha=#89b4fa)

## Deviations from Plan
None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- 6 of 16 presets now have complete interactive state colors and corrected text_scale
- Plans 04 and 05 cover the remaining 10 community presets (nord, dracula, gruvbox, solarized, tokyo-night, one-dark)

## Self-Check: PASSED

All 7 files found, both commit hashes verified.

---
*Phase: 53-preset-completeness*
*Completed: 2026-04-07*
