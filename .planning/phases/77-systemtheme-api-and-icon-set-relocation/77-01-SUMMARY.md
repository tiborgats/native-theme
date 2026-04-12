---
phase: 77-systemtheme-api-and-icon-set-relocation
plan: 01
subsystem: api
tags: [colormode, systemtheme, enum, non-exhaustive, public-api]

# Dependency graph
requires:
  - phase: 76-type-vocabulary-rename-and-crate-root-partition
    provides: "pub mod theme facade, pub(crate) use model re-exports"
provides:
  - "ColorMode enum (Light, Dark) with is_dark() method"
  - "SystemTheme.mode: ColorMode field (replaces is_dark: bool)"
  - "SystemTheme::pick(ColorMode) method (replaces pick(bool) and active())"
  - "Theme::pick_variant(ColorMode) and into_variant(ColorMode)"
affects: [78-overlay-source-accessibility-font-dpi, 82-icon-api-rework, connectors]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "ColorMode enum for light/dark selection instead of bare bool"
    - "is_dark() method on ColorMode for bool-needing contexts"

key-files:
  created: []
  modified:
    - native-theme/src/model/mod.rs
    - native-theme/src/lib.rs
    - native-theme/src/pipeline.rs
    - connectors/native-theme-gpui/src/lib.rs
    - connectors/native-theme-iced/src/lib.rs
    - connectors/native-theme-gpui/examples/showcase-gpui.rs
    - connectors/native-theme-iced/examples/showcase-iced.rs

key-decisions:
  - "ColorMode placed in model/mod.rs (top of file, before ThemeMode), re-exported via pub mod theme and pub(crate) use"
  - "GnomePortalData.is_dark kept as bool (internal D-Bus field, not public API)"
  - "Connector examples renamed local ColorMode to AppColorMode to avoid collision"
  - "Connector from_preset/from_system keep is_dark: bool parameters (gpui/iced facing), convert internally to ColorMode"

patterns-established:
  - "ColorMode enum replaces all bool dark-mode parameters in native-theme public API"
  - "AppColorMode naming for app-level System/Light/Dark enums in examples"

requirements-completed: [MODEL-03]

# Metrics
duration: 17min
completed: 2026-04-13
---

# Phase 77 Plan 01: SystemTheme API ColorMode Migration Summary

**ColorMode enum replaces bool dark-mode parameters: SystemTheme.mode, pick(ColorMode), pick_variant(ColorMode), into_variant(ColorMode) across core crate and both connectors**

## Performance

- **Duration:** 17 min
- **Started:** 2026-04-12T23:02:22Z
- **Completed:** 2026-04-12T23:19:25Z
- **Tasks:** 2
- **Files modified:** 17

## Accomplishments
- Added non_exhaustive ColorMode enum with Light/Dark variants and is_dark() convenience method
- Deleted SystemTheme::active() method, replaced SystemTheme.is_dark: bool with mode: ColorMode
- Migrated pick(), pick_variant(), into_variant() from bool to ColorMode across entire workspace
- Updated all 718+ tests (469 core + 152 gpui + 97 iced) -- zero failures

## Task Commits

Each task was committed atomically:

1. **Task 1: Add ColorMode enum and migrate SystemTheme API** - `7bfbf0c` (feat)
2. **Task 2: Migrate connector crates and examples to ColorMode API** - `f8d0b11` (feat)

## Files Created/Modified
- `native-theme/src/model/mod.rs` - ColorMode enum definition, pick_variant/into_variant signature changes
- `native-theme/src/lib.rs` - SystemTheme.mode field, pick(ColorMode), deleted active(), pub(crate) re-export
- `native-theme/src/pipeline.rs` - run_pipeline(mode: ColorMode), from_linux/from_system_inner/from_system_async_inner
- `native-theme/tests/prelude_smoke.rs` - Updated into_variant calls
- `native-theme/tests/resolve_and_validate.rs` - Updated pick_variant/into_variant calls
- `connectors/native-theme-gpui/src/lib.rs` - SystemThemeExt, from_system, from_preset, ColorMode re-export
- `connectors/native-theme-gpui/src/colors.rs` - Test into_variant calls
- `connectors/native-theme-gpui/src/config.rs` - Test into_variant calls
- `connectors/native-theme-gpui/examples/showcase-gpui.rs` - AppColorMode rename, pick/pick_variant migration
- `connectors/native-theme-iced/src/lib.rs` - SystemThemeExt, from_system, from_preset, ColorMode re-export
- `connectors/native-theme-iced/src/palette.rs` - Test into_variant calls
- `connectors/native-theme-iced/src/extended.rs` - Test into_variant calls
- `connectors/native-theme-iced/tests/integration.rs` - Updated into_variant calls
- `connectors/native-theme-iced/examples/showcase-iced.rs` - AppColorMode rename, pick/pick_variant migration
- `README.md` - Updated API examples
- `native-theme/README.md` - Updated API examples
- `connectors/native-theme-iced/README.md` - Updated API examples

## Decisions Made
- ColorMode placed at top of model/mod.rs, re-exported via `pub mod theme { pub use crate::model::*; }` and `pub(crate) use model::{..., ColorMode, ...}`
- GnomePortalData.is_dark kept as bool -- it's an internal D-Bus field, not part of the public API; conversion to ColorMode happens at the pipeline boundary
- Connector examples (showcase-gpui, showcase-iced) renamed their local `ColorMode` enum to `AppColorMode` to avoid collision with the new `native_theme::theme::ColorMode`
- Connector `from_preset(name, is_dark: bool)` and `to_theme(..., is_dark: bool)` keep their bool parameters (gpui/iced-facing convenience API); they convert internally to ColorMode

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed missing ColorMode imports in connector sub-module tests**
- **Found during:** Task 2 (connector migration)
- **Issue:** `use super::*` in sub-module tests (colors.rs, config.rs, extended.rs, palette.rs) did not bring ColorMode into scope because it was only re-exported at the crate root, not in the sub-modules themselves
- **Fix:** Added `use crate::ColorMode;` to each affected test module
- **Files modified:** colors.rs, config.rs, extended.rs, palette.rs
- **Committed in:** f8d0b11

**2. [Rule 1 - Bug] Fixed remaining pick(is_dark) calls in showcase examples**
- **Found during:** Task 2 (pre-release check)
- **Issue:** Two `system.pick(is_dark)` calls in showcase examples were missed by initial sed passes
- **Fix:** Added bool-to-ColorMode conversion at call sites
- **Files modified:** showcase-gpui.rs, showcase-iced.rs
- **Committed in:** f8d0b11

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 bug)
**Impact on plan:** Both auto-fixes necessary for compilation. No scope creep.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- ColorMode enum is in place for Phase 77-02 (icon_set relocation) to use
- Phase 78 (OverlaySource + AccessibilityPreferences) can build on the new SystemTheme.mode field
- All pre-release checks pass

---
*Phase: 77-systemtheme-api-and-icon-set-relocation*
*Completed: 2026-04-13*
