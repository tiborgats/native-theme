---
phase: 07-extended-presets
plan: 02
subsystem: presets
tags: [toml, catppuccin, nord, dracula, gruvbox, solarized, tokyo-night, one-dark, color-scheme]

# Dependency graph
requires:
  - phase: 07-extended-presets plan 01
    provides: Platform preset TOML files and 7-entry registry
  - phase: 02-core-presets
    provides: Preset registry pattern (include_str, match, TOML deserialization)
provides:
  - 10 community color scheme preset TOML files (Catppuccin 4, Nord, Dracula, Gruvbox, Solarized, Tokyo Night, One Dark)
  - 17-entry preset registry with full test coverage
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Community preset light/dark pairing: Latte base for all Catppuccin light variants"
    - "Flavor-specific accents on shared Latte base tones for derived light variants"

key-files:
  created:
    - src/presets/catppuccin-latte.toml
    - src/presets/catppuccin-frappe.toml
    - src/presets/catppuccin-macchiato.toml
    - src/presets/catppuccin-mocha.toml
    - src/presets/nord.toml
    - src/presets/dracula.toml
    - src/presets/gruvbox.toml
    - src/presets/solarized.toml
    - src/presets/tokyo-night.toml
    - src/presets/one-dark.toml
  modified:
    - src/presets.rs
    - tests/preset_loading.rs

key-decisions:
  - "17 total presets (not 18): plan had arithmetic error counting 10 community themes as 11; corrected to 7 existing + 10 new = 17"
  - "Catppuccin derived light variants use Latte base tones with flavor-specific accent colors"
  - "Community presets use generic fonts (sans-serif/monospace) since they are not platform-specific"

patterns-established:
  - "Community preset TOML structure: ~151 lines per file, all 36 color roles for both variants"

requirements-completed: [PRESET-04]

# Metrics
duration: 4min
completed: 2026-03-07
---

# Phase 7 Plan 2: Community Presets Summary

**10 community color scheme presets (Catppuccin 4 flavors, Nord, Dracula, Gruvbox, Solarized, Tokyo Night, One Dark) as bundled TOML files with 17-entry registry and full test coverage**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-07T21:51:12Z
- **Completed:** 2026-03-07T21:55:27Z
- **Tasks:** 2
- **Files modified:** 12

## Accomplishments
- Created 10 community TOML preset files with complete light+dark variants (all 36 color roles, fonts, geometry, spacing)
- Wired all 10 community presets into the preset registry (include_str, match arms, PRESET_NAMES)
- Updated all test counts and assertions from 7 to 17 -- all 134 tests pass

## Task Commits

Each task was committed atomically:

1. **Task 1: Create 10 community preset TOML files** - `726f82d` (feat)
2. **Task 2: Wire community presets into registry and update all tests** - `f4c5906` (feat)

## Files Created/Modified
- `src/presets/catppuccin-latte.toml` - Catppuccin Latte: Latte light + Frappe-derived dark
- `src/presets/catppuccin-frappe.toml` - Catppuccin Frappe: Latte-base light + Frappe dark
- `src/presets/catppuccin-macchiato.toml` - Catppuccin Macchiato: Latte-base light + Macchiato dark
- `src/presets/catppuccin-mocha.toml` - Catppuccin Mocha: Latte-base light + Mocha dark
- `src/presets/nord.toml` - Nord: Snow Storm light + Polar Night dark
- `src/presets/dracula.toml` - Dracula: Alucard light + Classic Dracula dark
- `src/presets/gruvbox.toml` - Gruvbox: Official light + dark medium contrast
- `src/presets/solarized.toml` - Solarized: Official Solarized Light + Dark
- `src/presets/tokyo-night.toml` - Tokyo Night: Day light + Night dark
- `src/presets/one-dark.toml` - One Dark: One Light + One Dark
- `src/presets.rs` - Updated with 10 new include_str constants, match arms, 17-entry PRESET_NAMES
- `tests/preset_loading.rs` - Updated count assertion to 17, expanded name checks to all 17 presets

## Decisions Made
- Plan arithmetic error corrected: plan said "11 community presets" and "18 total" but only enumerated 10 community themes (4 Catppuccin + Nord + Dracula + Gruvbox + Solarized + Tokyo Night + One Dark = 10). Actual total: 7 + 10 = 17.
- Each Catppuccin flavor uses Latte base tones for its light variant with the flavor's own accent colors, preserving each flavor's unique character.
- Community presets use generic font families (sans-serif, monospace) since they are not tied to any platform.
- Standard community geometry: radius 8.0, frame_width 1.0, disabled_opacity 0.5, border_opacity 0.15, scroll_width 8.0.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Corrected preset count from 18 to 17**
- **Found during:** Task 1 (before creating files)
- **Issue:** Plan consistently said "11 community presets" and "18 total" but only enumerated 10 community themes. 7 existing + 10 new = 17, not 18.
- **Fix:** Used actual count of 17 in all test assertions and documentation
- **Files modified:** src/presets.rs, tests/preset_loading.rs
- **Verification:** All 134 tests pass with count 17
- **Committed in:** f4c5906 (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 bug in plan arithmetic)
**Impact on plan:** Corrected off-by-one error in planned test assertions. No scope change.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 7 (Extended Presets) is complete with 17 total presets covering core, platform, and community themes
- Ready for Phase 8 or final verification

## Self-Check: PASSED

All 10 community TOML files verified present. Both task commits (726f82d, f4c5906) verified in git log. SUMMARY.md exists.

---
*Phase: 07-extended-presets*
*Completed: 2026-03-07*
