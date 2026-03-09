---
phase: 20-windows-icon-loading
plan: 01
subsystem: icons
tags: [windows, win32, shgetstockiconinfo, getglyphoutlinew, segoe-fluent-icons, rgba, system-icons, gdi]

requires:
  - phase: 16-icon-types
    provides: "IconRole, IconSet, IconData types and icon_name() mapping"
  - phase: 17-bundled-svg-icons
    provides: "bundled_icon_svg() for Material/Lucide SVG fallback"
  - phase: 18-linux-icon-loading
    provides: "Platform icon loader pattern and system-icons feature"
  - phase: 19-macos-icon-loading
    provides: "unpremultiply_alpha pattern and CGBitmapContext rasterization approach"
provides:
  - "load_windows_icon() for Windows icon loading to RGBA pixels via two pipelines"
  - "Stock icon pipeline: SHGetStockIconInfo -> HICON -> GetDIBits -> BGRA-to-RGBA -> unpremultiply"
  - "Font glyph pipeline: GetGlyphOutlineW(GGO_GRAY8_BITMAP) -> gray-to-RGBA with white foreground"
  - "Win32_UI_Shell feature added to windows crate dependency"
affects: [21-icon-connectors]

tech-stack:
  added: []
  patterns: [hicon-to-rgba-extraction, ggo-gray8-bitmap-glyph-rendering, bgra-to-rgba-conversion, font-availability-verification]

key-files:
  created: [native-theme/src/winicons.rs]
  modified: [native-theme/Cargo.toml, native-theme/src/lib.rs]

key-decisions:
  - "Did not add dep:windows to system-icons feature -- windows crate's transitive dependency windows-future has version conflict when activated on Linux; windows feature already activates dep:windows"
  - "White foreground (255,255,255) with alpha from grayscale mask for font glyph icons, matching icon font convention"
  - "DEFAULT_ICON_SIZE = 32 for font glyph rendering, SHGSI_LARGEICON for stock icons"
  - "Font fallback chain: Segoe Fluent Icons -> Segoe MDL2 Assets -> bundled Material SVG"
  - "LoadIconW shared resources (IDI_QUESTION) are NOT destroyed with DestroyIcon"

patterns-established:
  - "Two-pipeline icon dispatch: SIID_ prefix -> stock icons, plain name -> font glyphs, IDI_ -> LoadIconW special case"
  - "GetTextFaceW verification to detect GDI silent font substitution"

requirements-completed: [PLAT-02, PLAT-03]

duration: 9min
completed: 2026-03-09
---

# Phase 20 Plan 01: Windows Icon Loading Summary

**Windows icon loader with dual pipelines -- SHGetStockIconInfo for 18 stock icons and GetGlyphOutlineW for 22 Segoe Fluent Icons font glyphs, with BGRA-to-RGBA conversion, premultiplied-to-straight alpha, and bundled Material SVG fallback**

## Performance

- **Duration:** 9 min
- **Started:** 2026-03-09T09:31:13Z
- **Completed:** 2026-03-09T09:40:56Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Created winicons.rs module (530 lines) with full Windows icon loading pipeline
- Two dispatch pipelines: SHGetStockIconInfo for SIID_ roles, GetGlyphOutlineW for Segoe Fluent Icons glyphs
- BGRA-to-RGBA byte swap, premultiplied-to-straight alpha, GGO_GRAY8_BITMAP gray-to-alpha scaling
- Font availability verification via GetTextFaceW to detect GDI silent substitution
- Fallback chain: Segoe Fluent Icons -> Segoe MDL2 Assets -> bundled Material SVG
- 5 platform-independent inline tests, 4 Windows-only integration tests (cfg-gated)
- All 181 existing tests pass, clippy clean, cfg-gated module compiles out on Linux

## Task Commits

Each task was committed atomically:

1. **Task 1: Add Win32_UI_Shell feature, update system-icons, and create winicons.rs module** - `f247a1c` (feat)
2. **Task 2: Wire winicons module into lib.rs and verify full test suite** - `e8b78ec` (feat)

## Files Created/Modified
- `native-theme/src/winicons.rs` - Windows icon loader with stock icon and font glyph pipelines (530 lines)
- `native-theme/Cargo.toml` - Added Win32_UI_Shell to windows crate features
- `native-theme/src/lib.rs` - Added cfg-gated module declaration and re-export for winicons

## Decisions Made
- Did not add `dep:windows` to `system-icons` feature: the `windows` crate's transitive dependency `windows-future` has a version conflict when activated on Linux via `system-icons`. Since the existing `windows` Cargo feature already activates `dep:windows`, Windows users enable both `system-icons` and `windows` features.
- White foreground (255,255,255) with alpha from grayscale mask for font glyph icons, matching standard icon font convention and allowing callers to tint via multiplication.
- DEFAULT_ICON_SIZE = 32 pixels for font glyph rendering (matching SHGSI_LARGEICON stock icon size).
- LoadIconW shared resources (like IDI_QUESTION) are NOT destroyed with DestroyIcon -- they are system-managed.
- Font fallback chain: Segoe Fluent Icons (Win11) -> Segoe MDL2 Assets (Win10) -> bundled Material SVG.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Removed dep:windows from system-icons feature**
- **Found during:** Task 1 (Cargo.toml update)
- **Issue:** Adding `dep:windows` to `system-icons` activated the `windows` crate on Linux, causing a transitive dependency version conflict in `windows-future` (v0.2.1 vs v0.3.2) that prevented compilation.
- **Fix:** Kept `system-icons` without `dep:windows`. The existing `windows` Cargo feature already activates `dep:windows`. On Windows, users enable both `system-icons` and `windows` features. The research notes identified this same conclusion.
- **Files modified:** native-theme/Cargo.toml
- **Verification:** `cargo check -p native-theme --features system-icons` compiles cleanly
- **Committed in:** f247a1c (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Necessary to avoid cross-platform compilation failure. No functional impact -- the `windows` crate is already separately activated by its own feature flag.

## Issues Encountered

None beyond the documented deviation.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Windows icon loading complete, ready for Phase 21 (icon connectors)
- The system-icons feature now activates Linux (freedesktop-icons), macOS (objc2-core-graphics), and Windows (via separate windows feature) platform loaders
- Phase 21 can use load_windows_icon() alongside load_sf_icon() and load_freedesktop_icon() in the unified load_icon() API

## Self-Check: PASSED

- [x] native-theme/src/winicons.rs exists (530 lines, above 150-line minimum)
- [x] native-theme/Cargo.toml has Win32_UI_Shell feature
- [x] native-theme/src/lib.rs has cfg-gated module and re-export
- [x] Commit f247a1c exists (Task 1)
- [x] Commit e8b78ec exists (Task 2)
- [x] 181 tests pass with icon features
- [x] Clippy clean

---
*Phase: 20-windows-icon-loading*
*Completed: 2026-03-09*
