---
phase: 58-implement-docs-todo-v0-5-5-size-fix-md
plan: 02
subsystem: os-readers
tags: [font-dpi, kde, gnome, macos, windows, xrdb, accessibility]

# Dependency graph
requires:
  - phase: 58-implement-docs-todo-v0-5-5-size-fix-md
    provides: "font_dpi field on ThemeDefaults (Plan 01)"
provides:
  - "read_xft_dpi() shared Linux utility in lib.rs"
  - "KDE detect_font_dpi() with forceFontDPI/Xft.dpi/96 fallback chain"
  - "GNOME font_dpi detection via Xft.dpi"
  - "macOS font_dpi=72.0 (Apple coordinate identity)"
  - "Windows font_dpi=96.0 (logical coordinate base)"
  - "KDE text_scaling_factor no longer derived from forceFontDPI (Fix 5)"
affects: [58-03-PLAN, resolution-pipeline, font-sizing]

# Tech tracking
tech-stack:
  added: []
  patterns: ["spawn+try_wait polling timeout for subprocess calls (read_xft_dpi mirrors run_gsettings_with_timeout)", "detect_font_dpi() per-module helpers wrapping shared crate::read_xft_dpi()"]

key-files:
  created: []
  modified:
    - native-theme/src/lib.rs
    - native-theme/src/kde/mod.rs
    - native-theme/src/gnome/mod.rs
    - native-theme/src/macos.rs
    - native-theme/src/windows.rs

key-decisions:
  - "read_xft_dpi() placed in lib.rs (not kde/mod.rs) to avoid cross-feature dependency between portal and kde features"
  - "GNOME gets its own detect_font_dpi() wrapper (same pattern as KDE) for consistent fallback chain"
  - "KDE forceFontDPI no longer sets text_scaling_factor (Fix 5 correction: it is a font DPI, not accessibility scale)"

patterns-established:
  - "OS-specific DPI detection with shared utility: crate::read_xft_dpi() callable from any Linux feature module"
  - "detect_font_dpi() per-reader pattern: platform-specific chain with 96.0 fallback"

requirements-completed: [FIX-2, FIX-5]

# Metrics
duration: 8min
completed: 2026-04-08
---

# Phase 58 Plan 02: OS Reader font_dpi Detection Summary

**All four OS readers detect font_dpi: KDE from forceFontDPI/Xft.dpi/96 fallback, GNOME from Xft.dpi/96, macOS=72 (identity), Windows=96; KDE text_scaling_factor no longer incorrectly derived from forceFontDPI**

## Performance

- **Duration:** 8 min
- **Started:** 2026-04-08T01:02:53Z
- **Completed:** 2026-04-08T01:11:03Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- Added read_xft_dpi() shared utility in lib.rs with spawn+try_wait 2-second timeout pattern
- KDE reader: detect_font_dpi() with forceFontDPI -> Xft.dpi -> 96.0 fallback chain; fixed incorrect text_scaling_factor derivation from forceFontDPI
- GNOME reader: detect_font_dpi() via crate::read_xft_dpi() with 96 DPI fallback (no cross-feature dependency)
- macOS reader: font_dpi=72.0 (Apple coordinate system identity)
- Windows reader: font_dpi=96.0 (logical coordinate base)
- Updated KDE tests: font_dpi=120.0 from forceFontDPI, text_scaling_factor=None

## Task Commits

Each task was committed atomically:

1. **Task 1: Add read_xft_dpi() to lib.rs, KDE detect_font_dpi(), fix text_scaling_factor** - `ce40eec` (feat)
2. **Task 2: GNOME, macOS, and Windows readers: add font_dpi detection** - `226091c` (feat)

## Files Created/Modified
- `native-theme/src/lib.rs` - Added read_xft_dpi() shared Linux utility with 2-second timeout
- `native-theme/src/kde/mod.rs` - Added detect_font_dpi(), fixed populate_accessibility (font_dpi not text_scaling_factor), updated tests
- `native-theme/src/gnome/mod.rs` - Added detect_font_dpi() and font_dpi detection via crate::read_xft_dpi()
- `native-theme/src/macos.rs` - Added font_dpi=72.0 in accessibility block
- `native-theme/src/windows.rs` - Added font_dpi=96.0 after accessibility block

## Decisions Made
- read_xft_dpi() placed in lib.rs rather than kde/mod.rs to avoid cross-feature dependency (portal feature does not depend on kde feature)
- GNOME gets its own detect_font_dpi() wrapper function for clean encapsulation, consistent with KDE's pattern
- KDE forceFontDPI is purely a font rendering DPI (Fix 5) -- must NOT derive text_scaling_factor from it

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- Pre-existing test failure: `kde::tests::test_wm_title_bar_colors` fails on title_bar_font.color assertion (confirmed failing before plan changes, not caused by this plan)
- Pre-existing zbus dependency compilation error when building portal-only feature (`--features portal --no-default-features`)
- Pre-existing clippy doc list item warning in resolve/mod.rs line 28 (from Plan 01's Phase 1.5 comment)

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- All four OS readers now populate font_dpi on ThemeDefaults
- Plan 03 (pipeline propagation of font_dpi to inactive variant) can proceed
- Plan 01's resolve_font_dpi_conversion() is already implemented in inheritance.rs, ready for end-to-end pipeline integration

## Self-Check: PASSED

- All 5 modified files exist on disk
- Both task commits verified: ce40eec, 226091c
- read_xft_dpi in lib.rs, detect_font_dpi in kde/mod.rs and gnome/mod.rs, font_dpi in macos.rs and windows.rs

---
*Phase: 58-implement-docs-todo-v0-5-5-size-fix-md*
*Completed: 2026-04-08*
