---
phase: 48-connector-migration
plan: 01
subsystem: connector
tags: [gpui, resolved-theme, per-widget, migration]

# Dependency graph
requires:
  - phase: 45-resolution-engine
    provides: ResolvedTheme type and resolve()+validate() pipeline
  - phase: 44-per-widget-types
    provides: Per-widget Resolved structs (ResolvedScrollbar, ResolvedSlider, etc.)
provides:
  - gpui connector accepting &ResolvedTheme instead of &ThemeVariant
  - Per-widget field mapping (scrollbar.thumb, slider.fill, switch.unchecked_bg, etc.)
  - Working gpui showcase using resolve+validate pipeline
affects: [48-02 iced connector migration, 48-03 connector cleanup]

# Tech tracking
tech-stack:
  added: []
  patterns: [resolve+validate pipeline for connector input, per-widget direct field access]

key-files:
  modified:
    - connectors/native-theme-gpui/src/colors.rs
    - connectors/native-theme-gpui/src/config.rs
    - connectors/native-theme-gpui/src/lib.rs
    - connectors/native-theme-gpui/examples/showcase.rs

key-decisions:
  - "ResolvedFontSpec sizes used directly (no pt-to-px conversion) -- sizes are already logical pixels"
  - "Per-widget resolved fields used for scrollbar, slider, switch, progress_bar, title_bar, caret, tab colors"
  - "Showcase stores original_font and original_mono_font as ResolvedFontSpec instead of removed ThemeFonts"

patterns-established:
  - "Connector resolve pipeline: pick_variant() -> clone() -> resolve() -> validate() -> to_theme(&resolved)"
  - "Direct field access: resolved.scrollbar.thumb not derived from accent"

requirements-completed: [CONN-01, CONN-03]

# Metrics
duration: 7min
completed: 2026-03-27
---

# Phase 48 Plan 01: GPUI Connector Migration Summary

**Migrated gpui connector from ThemeVariant (Option-heavy) to ResolvedTheme (guaranteed-populated) with per-widget field mapping for scrollbar, slider, switch, progress bar, title bar, caret, and tab colors**

## Performance

- **Duration:** 7 min
- **Started:** 2026-03-27T14:49:27Z
- **Completed:** 2026-03-27T14:57:06Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Rewrote colors.rs, config.rs, lib.rs to accept &ResolvedTheme -- eliminated all Option fallback handling and ~10 default color functions
- Mapped per-widget resolved fields directly (scrollbar.thumb, slider.fill, switch.unchecked_bg, progress_bar.fill, window.title_bar_background, input.caret, tab colors) instead of deriving from base colors
- Updated gpui showcase (5467-line example) to use resolve+validate pipeline and ResolvedFontSpec
- All 57 lib tests pass, showcase compiles with zero errors

## Task Commits

Each task was committed atomically:

1. **Task 1: Rewrite gpui connector source for ResolvedTheme** - `0e9d177` (feat)
2. **Task 2: Update gpui showcase example for ResolvedTheme API** - `d5a4c7e` (feat)

## Files Created/Modified
- `connectors/native-theme-gpui/src/colors.rs` - ThemeColor mapping from ResolvedTheme with per-widget fields
- `connectors/native-theme-gpui/src/config.rs` - ThemeConfig mapping from ResolvedTheme defaults
- `connectors/native-theme-gpui/src/lib.rs` - to_theme() entry point accepting &ResolvedTheme
- `connectors/native-theme-gpui/examples/showcase.rs` - Working showcase using resolve+validate pipeline

## Decisions Made
- Used ResolvedFontSpec sizes directly without pt-to-px conversion (sizes are already in logical pixels after resolution)
- Per-widget resolved fields used for all available widget-specific colors instead of deriving from base colors
- Showcase stores two ResolvedFontSpec values (font + mono_font) instead of the removed ThemeFonts type
- Kept deprecated pick_variant() function in lib.rs since it operates on NativeTheme (unchanged type)

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Known Stubs
None - all data sources are wired to resolved theme fields.

## Next Phase Readiness
- gpui connector fully migrated, ready for iced connector migration (48-02)
- Pattern established: pick_variant -> clone -> resolve -> validate -> to_theme(&resolved)
- Per-widget mapping pattern ready to replicate for iced connector

## Self-Check: PASSED

- All 4 modified files verified present on disk
- Both task commits (0e9d177, d5a4c7e) verified in git log
- 57/57 lib tests pass
- Showcase compiles with zero errors
- Zero unwrap_or/unwrap_or_default in target source files

---
*Phase: 48-connector-migration*
*Completed: 2026-03-27*
