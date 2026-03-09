---
phase: 21-integration-and-connectors
plan: 02
subsystem: icons
tags: [gpui-connector, icon-name, lucide, image-source, bmp-encoder, showcase, icon-set-selector]

requires:
  - phase: 16-icon-data-model
    provides: IconRole, IconData, IconSet, icon_name()
  - phase: 17-bundled-icons
    provides: bundled_icon_svg() with Material + Lucide SVGs
  - phase: 21-integration-and-connectors (plan 01)
    provides: load_icon() dispatch, rasterize_svg()
provides:
  - icon_name() Lucide shortcut mapping 30 IconRole variants to gpui-component IconName
  - to_image_source() converting IconData (SVG + RGBA) to gpui ImageSource
  - Inline BMP encoder for RGBA-to-gpui conversion without external dependency
  - Showcase icon set selector dropdown with native icon display grid
affects: [21-03 iced connector]

tech-stack:
  added: []
  patterns: [inline-bmp-encoding, lucide-shortcut-mapping, icon-set-selector-ui]

key-files:
  created: [connectors/native-theme-gpui/src/icons.rs]
  modified: [connectors/native-theme-gpui/src/lib.rs, connectors/native-theme-gpui/examples/showcase.rs]

key-decisions:
  - "Inline BMP V4 encoder for RGBA-to-gpui conversion (no png crate needed)"
  - "assert!(matches!()) instead of assert_eq!() because gpui-component IconName lacks Debug"
  - "Wildcard arm on IconData match for #[non_exhaustive] forward compat"
  - "30 of 42 roles mapped (not 28 as research estimated) -- gpui-component 0.5 has Window* and more"
  - "Lucide shortcut prioritized over raw img() for roles with gpui-component match"

patterns-established:
  - "BMP V4 encoding: 14-byte file header + 108-byte V4 DIB header + BGRA pixel data"
  - "Icon set selector: pre-load all 42 roles on selection change, store in view state"
  - "Fallback label pattern: is_native_icon_set() check for platform-availability indicator"

requirements-completed: [INTG-03, INTG-05]

duration: 6min
completed: 2026-03-09
---

# Phase 21 Plan 02: gpui Connector Icon Functions & Showcase Icon Display Summary

**icon_name() Lucide shortcut mapping 30 IconRole variants to gpui-component IconName, to_image_source() with inline BMP encoding, and showcase icon set selector dropdown**

## Performance

- **Duration:** 6 min
- **Started:** 2026-03-09T16:17:33Z
- **Completed:** 2026-03-09T16:24:30Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- icon_name() maps 30 of 42 IconRole variants to gpui-component IconName (zero-I/O Lucide shortcut)
- to_image_source() converts IconData::Svg via Image::from_bytes and IconData::Rgba via inline BMP V4 encoder
- 23 unit tests covering mapping specifics, None cases, count validation, and BMP encoding correctness
- Showcase icons tab gains icon set selector dropdown and native icon display grid alongside existing gallery
- All 47 native-theme-gpui tests pass; showcase compiles with material-icons + lucide-icons features

## Task Commits

Each task was committed atomically:

1. **Task 1: Create gpui connector icons.rs with icon_name() and to_image_source()** - `d49f9c4` (feat)
2. **Task 2: Update showcase example with icon set selector and IconData display** - `68fb9f8` (feat)

## Files Created/Modified
- `connectors/native-theme-gpui/src/icons.rs` - icon_name() Lucide shortcut + to_image_source() + inline BMP encoder + 23 tests
- `connectors/native-theme-gpui/src/lib.rs` - Added `pub mod icons;` module declaration
- `connectors/native-theme-gpui/examples/showcase.rs` - Icon set selector dropdown, load_all_icons helper, native icon grid in icons tab

## Decisions Made
- Used inline BMP V4 encoder for RGBA-to-gpui conversion instead of adding `png` crate dependency. BMP with BITMAPV4HEADER supports 32-bit RGBA via channel masks, top-down row order, and sRGB color space -- sufficient for icon display.
- Used `assert!(matches!())` in tests because `gpui_component::IconName` does not derive `Debug`, making `assert_eq!` impossible.
- Added wildcard arm on `IconData` match for `#[non_exhaustive]` forward compatibility (returns empty SVG image for unknown variants).
- Mapped 30 roles (not 28 as plan estimated) because gpui-component 0.5 includes WindowClose/Min/Max/Restore and more Lucide icons than the research anticipated.
- In the showcase, prioritize Lucide shortcut (Icon::new with IconName) for roles that have a gpui-component match, falling back to raw img() with to_image_source for other loaded icons.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed gpui::img() API signature**
- **Found during:** Task 2
- **Issue:** Plan assumed `gpui::img().source(source)` API but gpui 0.2.2 uses `gpui::img(source)` (source is a constructor argument, not a method)
- **Fix:** Changed to `gpui::img(source).w(px(20.0)).h(px(20.0))`
- **Files modified:** connectors/native-theme-gpui/examples/showcase.rs
- **Committed in:** 68fb9f8

**2. [Rule 1 - Bug] Fixed IconName Debug trait absence**
- **Found during:** Task 1 (test compilation)
- **Issue:** `assert_eq!` requires Debug on both sides; gpui_component::IconName doesn't derive Debug
- **Fix:** Changed all icon_name() tests to use `assert!(matches!())` and `.is_none()` patterns
- **Files modified:** connectors/native-theme-gpui/src/icons.rs
- **Committed in:** d49f9c4

**3. [Rule 3 - Blocking] Fixed IconData #[non_exhaustive] match**
- **Found during:** Task 1 (compilation)
- **Issue:** `IconData` is `#[non_exhaustive]` so match requires a wildcard arm
- **Fix:** Added `_ => { ... }` arm returning empty SVG image for unknown variants
- **Files modified:** connectors/native-theme-gpui/src/icons.rs
- **Committed in:** d49f9c4

---

**Total deviations:** 3 auto-fixed (1 bug, 2 blocking)
**Impact on plan:** All fixes necessary for compilation and correctness. No scope creep.

## Issues Encountered
None beyond the auto-fixed items above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- gpui connector icon pipeline complete (icon_name shortcut + to_image_source conversion)
- Showcase demonstrates full icon pipeline with all 5 icon sets
- iced connector (Plan 03) can follow the same pattern for IconData-to-Handle conversion

## Self-Check: PASSED

- FOUND: connectors/native-theme-gpui/src/icons.rs
- FOUND: connectors/native-theme-gpui/src/lib.rs (pub mod icons)
- FOUND: connectors/native-theme-gpui/examples/showcase.rs (icon set selector)
- FOUND: commit d49f9c4
- FOUND: commit 68fb9f8

---
*Phase: 21-integration-and-connectors*
*Completed: 2026-03-09*
