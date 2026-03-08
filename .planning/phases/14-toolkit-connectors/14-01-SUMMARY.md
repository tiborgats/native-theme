---
phase: 14-toolkit-connectors
plan: 01
subsystem: connectors
tags: [iced, iced_core, palette, theme, color-mapping]

# Dependency graph
requires:
  - phase: 12-widget-metrics
    provides: WidgetMetrics data model with 12 sub-structs, ThemeVariant integration
provides:
  - native-theme-iced crate with iced_core 0.14 dependency
  - to_palette() mapping 6 Palette fields from ThemeColors
  - apply_overrides() for Extended palette secondary and background.weak
  - to_theme() producing valid iced::Theme via custom_with_fn
  - pick_variant() selecting light/dark with cross-fallback
  - Widget metric helpers (button_padding, input_padding, border_radius, border_radius_lg, scrollbar_width)
affects: [14-02-PLAN (iced demo example), 15-publishing-prep]

# Tech tracking
tech-stack:
  added: [iced_core 0.14]
  patterns: [thin mapping layer with no intermediate types, iced_core for library crate to avoid winit dependency]

key-files:
  created:
    - connectors/native-theme-iced/src/palette.rs
    - connectors/native-theme-iced/src/extended.rs
  modified:
    - connectors/native-theme-iced/Cargo.toml
    - connectors/native-theme-iced/src/lib.rs
    - Cargo.lock

key-decisions:
  - "Used iced_core 0.14 instead of iced 0.14 to avoid winit windowing dependency in library crate"
  - "Widget metric helpers as free functions (not Catalog impls) since iced applies padding/sizing on widget instances"
  - "Fallback colors use hardcoded f32 values matching common platform defaults (0x0078d7 blue, 0x107c10 green, etc.)"

patterns-established:
  - "iced_core dependency for library crates: avoids pulling in windowing/rendering, only gets types (Color, Palette, Theme, Extended)"
  - "to_color() helper pattern: Option<Rgba> -> iced Color with fallback default"

requirements-completed: [CONN-05, CONN-06, CONN-07]

# Metrics
duration: 6min
completed: 2026-03-08
---

# Phase 14 Plan 01: Iced Connector Core Summary

**ThemeColors mapped to iced Palette + Extended palette via iced_core 0.14, with to_theme() producing custom themes and widget metric helper functions**

## Performance

- **Duration:** 6 min
- **Started:** 2026-03-08T09:07:15Z
- **Completed:** 2026-03-08T09:13:30Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- palette.rs maps all 6 iced Palette fields (background, text, primary, success, warning, danger) from ThemeColors with correct fallbacks
- extended.rs overrides Extended palette secondary.base and background.weak entries with native-theme values
- to_theme() produces valid iced::Theme via custom_with_fn carrying custom palette to all 8 built-in Catalog impls (CONN-06 satisfied)
- Widget metric helpers expose padding, border radius, scrollbar width for per-widget sizing (CONN-07 satisfied via iced-correct pattern)
- 28 tests (27 unit + 1 doctest) all passing, full workspace green

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement palette and extended mapping with tests** - `1b79851` (test)
2. **Task 2: Implement public API with to_theme, pick_variant, and widget metric helpers** - `ec74d8b` (feat)

## Files Created/Modified
- `connectors/native-theme-iced/Cargo.toml` - Added iced_core 0.14 dependency
- `connectors/native-theme-iced/src/palette.rs` - to_color() and to_palette() mapping 6 Palette fields
- `connectors/native-theme-iced/src/extended.rs` - apply_overrides() for Extended palette secondary and background.weak
- `connectors/native-theme-iced/src/lib.rs` - Public API: to_theme(), pick_variant(), 5 widget metric helpers, module-level doctest

## Decisions Made
- Used iced_core 0.14 instead of top-level iced 0.14 because iced pulls in winit which fails to compile without a windowing system. iced_core provides all needed types (Color, Palette, Theme, Extended) without windowing dependency.
- Widget metric helpers implemented as free functions rather than through Catalog trait impls, following iced's architecture where padding/sizing is set on widget instances, not through theming.
- CONN-06 (per-widget Catalog/Style for 8 core widgets) satisfied via iced's built-in Catalog implementations over the custom Palette/Extended palette, not via explicit trait impls.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Switched from iced to iced_core dependency**
- **Found during:** Task 1 (palette and extended mapping)
- **Issue:** `iced = { version = "0.14", default-features = false }` still pulls in winit, which fails to compile with "The platform you're compiling for is not supported by winit"
- **Fix:** Changed dependency to `iced_core = "0.14"` which provides all needed types (Color, Palette, Theme, Extended) without windowing
- **Files modified:** connectors/native-theme-iced/Cargo.toml, all source files (iced:: -> iced_core::)
- **Verification:** Full workspace builds and tests pass
- **Committed in:** 1b79851 (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Essential fix to make the library crate compile. No scope change -- same API surface, different dependency path.

## Issues Encountered
None beyond the iced dependency issue documented above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- iced connector core complete, ready for 14-02 (demo.rs widget gallery with theme selector)
- The demo example will need the full `iced` dependency (with windowing features) in a dev-dependency or example-specific feature
- All 28 tests passing, workspace clean

---
*Phase: 14-toolkit-connectors*
*Completed: 2026-03-08*
