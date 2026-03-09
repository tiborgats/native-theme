---
phase: 21-integration-and-connectors
plan: 01
subsystem: icons
tags: [load_icon, rasterize, resvg, svg, dispatch, icon-pipeline]

requires:
  - phase: 16-icon-data-model
    provides: IconRole, IconData, IconSet, icon_name(), system_icon_set()
  - phase: 17-bundled-icons
    provides: bundled_icon_svg() with Material + Lucide SVGs
  - phase: 18-freedesktop-icons
    provides: load_freedesktop_icon() Linux loader
  - phase: 19-sf-symbols
    provides: load_sf_icon() macOS loader
  - phase: 20-windows-icons
    provides: load_windows_icon() Windows loader
provides:
  - load_icon() dispatch function selecting loader by theme string
  - rasterize_svg() SVG-to-RGBA conversion behind svg-rasterize feature
  - svg-rasterize Cargo feature with resvg 0.47 dependency
affects: [21-02 gpui connector, 21-03 iced connector]

tech-stack:
  added: [resvg 0.47 (optional, behind svg-rasterize feature)]
  patterns: [dispatch-with-fallback, cfg-gated feature modules, unpremultiply alpha]

key-files:
  created: [native-theme/src/rasterize.rs]
  modified: [native-theme/src/lib.rs, native-theme/Cargo.toml]

key-decisions:
  - "Access usvg/tiny_skia through resvg re-exports (not separate deps)"
  - "Centering offset for non-square SVG aspect ratios in rasterize_svg"
  - "#[allow(clippy::needless_return)] on load_icon for cfg-block early return pattern"
  - "#[allow(unused_variables)] on load_icon for no-feature compilation"

patterns-established:
  - "Dispatch pattern: IconSet::from_name -> match with cfg gates -> wildcard fallback"
  - "SVG rasterize pattern: usvg parse -> scale+center transform -> resvg render -> unpremultiply"

requirements-completed: [INTG-01, INTG-02]

duration: 3min
completed: 2026-03-09
---

# Phase 21 Plan 01: Load Icon Dispatch & SVG Rasterize Summary

**load_icon() dispatch routing to platform loaders and bundled sets with rasterize_svg() SVG-to-RGBA conversion via resvg**

## Performance

- **Duration:** 3 min
- **Started:** 2026-03-09T16:11:02Z
- **Completed:** 2026-03-09T16:14:02Z
- **Tasks:** 1
- **Files modified:** 3 (+ Cargo.lock)

## Accomplishments
- load_icon() dispatches to freedesktop/sf-symbols/segoe/material/lucide with theme string parsing and fallback chain
- rasterize_svg() converts SVG bytes to IconData::Rgba with uniform scaling, centering, and straight alpha
- 12 new tests covering dispatch for all 42 roles and rasterization edge cases
- All 188 unit tests pass with material-icons,lucide-icons,svg-rasterize features
- Clean clippy with all features and clean no-feature build

## Task Commits

Each task was committed atomically:

1. **Task 1: Add load_icon() dispatch and rasterize_svg() module** - `3bf9409` (feat)

## Files Created/Modified
- `native-theme/src/rasterize.rs` - SVG-to-RGBA rasterization module (resvg-backed, feature-gated)
- `native-theme/src/lib.rs` - load_icon() dispatch function, module declaration, re-exports
- `native-theme/Cargo.toml` - resvg optional dependency, svg-rasterize feature

## Decisions Made
- Access usvg and tiny_skia through resvg re-exports rather than adding them as separate dependencies
- Center icon with offset calculation when SVG aspect ratio doesn't match target square
- Use #[allow(clippy::needless_return)] for the cfg-block early return pattern (same pattern as from_system)
- Use #[allow(unused_variables)] for no-feature compilation (same pattern as bundled_icon_svg)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed usvg/tiny_skia import paths**
- **Found during:** Task 1
- **Issue:** Plan used bare `usvg::` and `tiny_skia::` paths but these crates are re-exported through resvg
- **Fix:** Added `use resvg::tiny_skia;` and `use resvg::usvg;` imports
- **Files modified:** native-theme/src/rasterize.rs
- **Committed in:** 3bf9409

**2. [Rule 1 - Bug] Fixed clippy needless_return warning**
- **Found during:** Task 1 (clippy verification)
- **Issue:** `return` in cfg-gated block needed for correctness but flagged by clippy
- **Fix:** Added `#[allow(clippy::needless_return)]` attribute
- **Files modified:** native-theme/src/lib.rs
- **Committed in:** 3bf9409

**3. [Rule 1 - Bug] Fixed unused_variables warning in no-feature build**
- **Found during:** Task 1 (no-feature check)
- **Issue:** `role` parameter unused when no icon features enabled
- **Fix:** Added `#[allow(unused_variables)]` attribute (matches bundled.rs pattern)
- **Files modified:** native-theme/src/lib.rs
- **Committed in:** 3bf9409

---

**Total deviations:** 3 auto-fixed (2 bugs, 1 blocking)
**Impact on plan:** All fixes necessary for clean compilation. No scope creep.

## Issues Encountered
None beyond the auto-fixed items above.

## Next Phase Readiness
- load_icon() and rasterize_svg() ready for connector consumption
- gpui connector (Plan 02) can use load_icon() + IconData conversion
- iced connector (Plan 03) can use load_icon() + IconData conversion

## Self-Check: PASSED

- FOUND: native-theme/src/rasterize.rs
- FOUND: native-theme/src/lib.rs (load_icon function)
- FOUND: native-theme/Cargo.toml (svg-rasterize feature)
- FOUND: commit 3bf9409

---
*Phase: 21-integration-and-connectors*
*Completed: 2026-03-09*
