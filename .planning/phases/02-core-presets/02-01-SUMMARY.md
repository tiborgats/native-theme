---
phase: 02-core-presets
plan: 01
subsystem: presets
tags: [rust, toml, presets, include_str, theme-loading, serialization]

# Dependency graph
requires:
  - "01-02: NativeTheme, ThemeVariant, ThemeColors with 36 color roles, ThemeFonts, ThemeGeometry, ThemeSpacing"
  - "01-01: Rgba hex serde, Error enum with From<toml::de::Error>/From<toml::ser::Error>/From<std::io::Error>"
provides:
  - "Three bundled TOML preset files: default, kde-breeze, adwaita (light + dark each)"
  - "Preset registry API: preset(), list_presets(), from_toml(), from_file(), to_toml()"
  - "Crate-root re-exports of all 5 preset API functions"
affects: [02-02, 03-kde-reader, 04-portal-reader, 05-windows-reader, 06-gnome-reader, 07-extended-presets]

# Tech tracking
tech-stack:
  added: []
  patterns: [include_str-compile-time-embedding, match-based-preset-registry, toml-string-pretty-serialization]

key-files:
  created:
    - src/presets/default.toml
    - src/presets/kde-breeze.toml
    - src/presets/adwaita.toml
    - src/presets.rs
  modified:
    - src/lib.rs

key-decisions:
  - "Pre-computed solid hex for Adwaita alpha colors (foreground #2e3436 from GTK convention, border #d5d5d5 from ~15% opacity black on white)"
  - "Fresh owned NativeTheme per preset() call (no caching) -- callers can freely mutate for merge overlays"
  - "Match statement for 3 presets instead of HashMap -- compile-time exhaustive, zero allocation"

patterns-established:
  - "include_str!() for compile-time TOML embedding from src/presets/*.toml"
  - "Five-function preset API: preset(), list_presets(), from_toml(), from_file(), to_toml()"
  - "PRESET_NAMES const array synchronized with match arms in preset()"

requirements-completed: [PRESET-01, PRESET-02]

# Metrics
duration: 2min
completed: 2026-03-07
---

# Phase 2 Plan 01: Core Presets Summary

**Three bundled theme presets (default/kde-breeze/adwaita) with include_str!() embedding and 5-function preset loading API**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-07T15:50:30Z
- **Completed:** 2026-03-07T15:53:17Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Three complete TOML preset files with all 36 color roles populated for both light and dark variants, plus fonts, geometry, and spacing
- Preset registry module (src/presets.rs) with 5 public API functions: preset(), list_presets(), from_toml(), from_file(), to_toml()
- 12 unit tests covering preset loading, round-trip serialization, error cases, and file I/O
- 6 doc-tests on all public functions, all passing
- All 112 tests passing (88 unit + 18 integration + 6 doc-tests)

## Task Commits

Each task was committed atomically:

1. **Task 1: Create three TOML preset files** - `2e1a985` (feat)
2. **Task 2: Create presets.rs module and wire into lib.rs** - `f471088` (feat)

## Files Created/Modified

- `src/presets/default.toml` - Neutral toolkit-agnostic preset with medium blue accent, both variants
- `src/presets/kde-breeze.toml` - KDE Breeze colors from official BreezeLight/BreezeDark .colors files
- `src/presets/adwaita.toml` - GNOME Adwaita colors from libadwaita CSS variables, alpha pre-computed
- `src/presets.rs` - Preset registry module with include_str!() embedding, 5 public functions, 12 unit tests
- `src/lib.rs` - Added `pub mod presets` and re-exports of all 5 preset API functions

## Decisions Made

- Pre-computed Adwaita alpha/opacity colors to solid hex values (foreground uses traditional GTK #2e3436, border ~#d5d5d5)
- Each preset() call returns a fresh owned NativeTheme (no static caching) so callers can mutate freely for merge
- Match statement for preset lookup rather than HashMap -- compile-time exhaustive for 3 entries, zero allocation

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- All 5 preset API functions available at crate root: native_theme::preset(), list_presets(), from_toml(), from_file(), to_toml()
- Three bundled presets loadable by name with full light/dark variants
- Ready for Plan 02-02 (preset integration tests) to exercise advanced validation
- Established include_str!() pattern ready for Phase 7 (extended presets)

## Self-Check: PASSED

- All 5 source files exist (default.toml, kde-breeze.toml, adwaita.toml, presets.rs, lib.rs)
- SUMMARY.md created at expected path
- All 2 commits verified (2e1a985, f471088)

---
*Phase: 02-core-presets*
*Completed: 2026-03-07*
