---
phase: 49-additive-type-definitions
plan: 01
subsystem: model
tags: [rust, serde, toml, structs, border, font, theme-model]

# Dependency graph
requires: []
provides:
  - BorderSpec and ResolvedBorderSpec structs with 6 fields each
  - FontStyle enum (Normal/Italic/Oblique)
  - FontSpec extended with style and color fields
  - ResolvedFontSpec extended with style and color fields
affects: [49-02, 49-03, 50-atomic-schema-commit, 51-resolution-engine, 53-interactive-state-colors]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "BorderSpec follows existing manual sub-struct pattern (like FontSpec/SpacingSpec)"
    - "FontStyle enum uses serde rename_all lowercase for TOML compatibility"
    - "New Option fields use ..Default::default() for backward-compatible construction"

key-files:
  created:
    - native-theme/src/model/border.rs
  modified:
    - native-theme/src/model/font.rs
    - native-theme/src/model/mod.rs
    - native-theme/src/model/resolved.rs
    - native-theme/src/model/defaults.rs
    - native-theme/src/model/widgets/mod.rs
    - native-theme/src/resolve.rs
    - native-theme/src/lib.rs
    - native-theme/src/macos.rs
    - native-theme/src/windows.rs
    - native-theme/src/kde/fonts.rs
    - native-theme/src/gnome/mod.rs
    - native-theme/src/presets.rs
    - connectors/native-theme-gpui/examples/showcase.rs

key-decisions:
  - "ResolvedFontSpec color uses temporary Rgba::rgb(0,0,0) fallback in require_font -- Phase 51 will wire proper foreground inheritance"
  - "FontStyle derives Default to Normal, keeping ResolvedFontSpec Default derive valid"
  - "All exhaustive FontSpec literals updated to use ..Default::default() for forward compatibility"

patterns-established:
  - "New Option fields on existing specs: add ..Default::default() to all exhaustive construction sites"
  - "BorderSpec pattern: 6 Option fields, FIELD_NAMES const, impl_merge with option clause"

requirements-completed: []

# Metrics
duration: 9min
completed: 2026-04-06
---

# Phase 49 Plan 01: Additive Type Definitions Summary

**BorderSpec/ResolvedBorderSpec with 6 border fields, FontStyle enum with 3 variants, and FontSpec/ResolvedFontSpec extended with style+color -- all backward-compatible**

## Performance

- **Duration:** 9 min
- **Started:** 2026-04-06T22:23:45Z
- **Completed:** 2026-04-06T22:32:47Z
- **Tasks:** 1
- **Files modified:** 14

## Accomplishments
- Created BorderSpec with 6 Option fields (color, corner_radius, line_width, shadow_enabled, padding_horizontal, padding_vertical) and ResolvedBorderSpec with matching concrete fields
- Added FontStyle enum (Normal/Italic/Oblique) with serde lowercase rename and Default derive
- Extended FontSpec with style (Option<FontStyle>) and color (Option<Rgba>) fields, and ResolvedFontSpec with corresponding concrete fields
- Updated all exhaustive FontSpec/ResolvedFontSpec construction sites across 14 files for compile compatibility
- Added 13 new unit tests (6 border, 2 font_style, 5 font_spec extension) -- all 429 existing tests continue to pass

## Task Commits

Each task was committed atomically:

1. **Task 1: Create BorderSpec module and extend FontSpec** - `c67f723` (feat)

## Files Created/Modified
- `native-theme/src/model/border.rs` - New BorderSpec and ResolvedBorderSpec structs with FIELD_NAMES, impl_merge, and 6 unit tests
- `native-theme/src/model/font.rs` - FontStyle enum, FontSpec+ResolvedFontSpec extended with style+color, 5 new tests
- `native-theme/src/model/mod.rs` - Added border module and re-exports for BorderSpec, ResolvedBorderSpec, FontStyle
- `native-theme/src/model/resolved.rs` - Updated sample_font() and sample_defaults() test helpers with style+color
- `native-theme/src/model/defaults.rs` - Added ..Default::default() to exhaustive FontSpec literals
- `native-theme/src/model/widgets/mod.rs` - Updated ResolvedFontSpec test literals, added ..Default::default() to FontSpec literals
- `native-theme/src/resolve.rs` - Updated require_font/require_font_opt with style+color, updated all test FontSpec literals
- `native-theme/src/lib.rs` - Re-exported BorderSpec, ResolvedBorderSpec, FontStyle
- `native-theme/src/macos.rs` - Added ..Default::default() to FontSpec construction
- `native-theme/src/windows.rs` - Added ..Default::default() to FontSpec construction
- `native-theme/src/kde/fonts.rs` - Added ..Default::default() to FontSpec construction
- `native-theme/src/gnome/mod.rs` - Added ..Default::default() to FontSpec construction
- `native-theme/src/presets.rs` - Added ..Default::default() to FontSpec literal in test
- `connectors/native-theme-gpui/examples/showcase.rs` - Updated ResolvedFontSpec fallback literals with style+color

## Decisions Made
- ResolvedFontSpec color field uses `Rgba::rgb(0, 0, 0)` (black) as temporary fallback in require_font/require_font_opt. Phase 51 will wire this properly to foreground color inheritance.
- FontStyle::Normal is the Default variant, which keeps ResolvedFontSpec's `#[derive(Default)]` valid without breaking existing code.
- All exhaustive FontSpec `{ family, size, weight }` literals across the codebase were updated with `..Default::default()` rather than adding explicit `style: None, color: None` -- this is forward-compatible for any future field additions.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- BorderSpec and FontStyle types are available for 49-02 (widget struct integration) and 49-03 (interactive state types)
- All existing tests pass, no regressions
- Pre-existing `.expect()` in lib.rs line 1852 flagged by pre-release-check is unrelated to this plan

---
*Phase: 49-additive-type-definitions*
*Completed: 2026-04-06*
