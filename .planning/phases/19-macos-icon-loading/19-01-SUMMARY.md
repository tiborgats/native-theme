---
phase: 19-macos-icon-loading
plan: 01
subsystem: icons
tags: [macos, sf-symbols, nsimage, core-graphics, rgba, system-icons]

requires:
  - phase: 16-icon-types
    provides: "IconRole, IconSet, IconData types and icon_name() mapping"
  - phase: 17-bundled-svg-icons
    provides: "bundled_icon_svg() for Material/Lucide SVG fallback"
  - phase: 18-linux-icon-loading
    provides: "Platform icon loader pattern and system-icons feature"
provides:
  - "load_sf_icon() for macOS SF Symbols icon loading to RGBA pixels"
  - "CGBitmapContext rasterization pipeline with unpremultiply pass"
  - "objc2-core-graphics dependency integrated into system-icons feature"
affects: [20-windows-icon-loading, 21-icon-connectors]

tech-stack:
  added: [objc2-core-graphics 0.3]
  patterns: [cgbitmap-rasterization, premultiplied-to-straight-alpha, sf-symbol-configuration]

key-files:
  created: [native-theme/src/sficons.rs]
  modified: [native-theme/Cargo.toml, native-theme/src/lib.rs]

key-decisions:
  - "CGBitmapContext rasterization for guaranteed RGBA pixel format normalization"
  - "Post-processing unpremultiply pass converts premultiplied to straight alpha"
  - "DEFAULT_ICON_SIZE = 24 matching freedesktop default"
  - "Read pixel dimensions from CGImage::width()/height() not NSImage size for Retina correctness"

patterns-established:
  - "SF Symbol loading: NSImage -> NSImageSymbolConfiguration -> CGImage -> CGBitmapContext -> RGBA buffer -> unpremultiply"
  - "Cross-platform system-icons feature activates per-platform deps via cfg-gated optional dependencies"

requirements-completed: [PLAT-01]

duration: 2min
completed: 2026-03-09
---

# Phase 19 Plan 01: macOS SF Symbols Icon Loading Summary

**SF Symbols icon loader with CGBitmapContext rasterization, premultiplied-to-straight alpha conversion, and Material SVG fallback using objc2-core-graphics**

## Performance

- **Duration:** 2 min
- **Started:** 2026-03-09T08:44:39Z
- **Completed:** 2026-03-09T08:47:18Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Created sficons.rs module (192 lines) with full SF Symbols -> RGBA pipeline
- CGBitmapContext rasterization normalizes any internal image format to consistent RGBA output
- Unpremultiply pass converts premultiplied alpha to straight alpha for consumer compatibility
- 4 inline tests covering unpremultiply correctness, icon loading, fallback, dimension validation
- All 181 existing tests pass, clippy clean, cfg-gated module compiles out cleanly on Linux

## Task Commits

Each task was committed atomically:

1. **Task 1: Add objc2-core-graphics dependency, update features, and create sficons.rs module** - `cb2307e` (feat)
2. **Task 2: Wire sficons module into lib.rs and verify full test suite** - `fa1746b` (feat)

## Files Created/Modified
- `native-theme/src/sficons.rs` - macOS SF Symbols icon loader module (192 lines)
- `native-theme/Cargo.toml` - Added objc2-core-graphics dependency, updated system-icons feature and objc2-app-kit features
- `native-theme/src/lib.rs` - Added cfg-gated module declaration and re-export for sficons

## Decisions Made
- CGBitmapContext with PremultipliedLast alpha info for guaranteed RGBA pixel format normalization
- Post-processing unpremultiply pass converts premultiplied alpha to straight alpha (consumers expect straight)
- DEFAULT_ICON_SIZE = 24 pixels matching freedesktop default for consistency
- Read pixel dimensions from CGImage::width()/height() (not NSImage size) for correct Retina pixel counts

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- macOS icon loading complete, ready for Phase 20 (Windows icon loading)
- The system-icons feature now activates both Linux (freedesktop-icons) and macOS (objc2-core-graphics) platform loaders
- Phase 21 (icon connectors) can use load_sf_icon() alongside load_freedesktop_icon() in the unified load_icon() API

## Self-Check: PASSED

- [x] native-theme/src/sficons.rs exists (192 lines, above 60-line minimum)
- [x] native-theme/Cargo.toml has objc2-core-graphics dependency
- [x] native-theme/src/lib.rs has cfg-gated module and re-export
- [x] Commit cb2307e exists (Task 1)
- [x] Commit fa1746b exists (Task 2)
- [x] 181 tests pass with icon features
- [x] Clippy clean

---
*Phase: 19-macos-icon-loading*
*Completed: 2026-03-09*
