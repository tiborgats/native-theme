---
phase: 78-overlaysource-accessibilitypreferences-font-dpi-relocation
plan: 02
subsystem: model
tags: [overlay, pipeline, memory-optimization, systemtheme]

requires:
  - phase: 78-01
    provides: "AccessibilityPreferences on SystemTheme, font_dpi as explicit parameter"
provides:
  - "OverlaySource struct replacing pre-resolve ThemeMode clones on SystemTheme"
  - "with_overlay() replays merge+resolve pipeline from stored OverlaySource data"
  - "~2KB per-SystemTheme memory reduction (no ThemeMode clone fields)"
affects: [overlay, pipeline, connectors]

tech-stack:
  added: []
  patterns: ["OverlaySource replay pattern for lazy variant reconstruction"]

key-files:
  created: []
  modified:
    - native-theme/src/lib.rs
    - native-theme/src/pipeline.rs

key-decisions:
  - "OverlaySource cloned unchanged on with_overlay -- base reader data and preset don't change when overlay is applied"
  - "unwrap_or on strip_suffix('-live') kept for non-live presets (e.g. user presets or catppuccin-mocha used in tests)"

patterns-established:
  - "OverlaySource replay: store minimal pipeline inputs (reader_output + preset_name + font_dpi) to reconstruct pre-resolve variants on demand"

requirements-completed: [MODEL-02]

duration: 4min
completed: 2026-04-13
---

# Phase 78 Plan 02: OverlaySource Replaces Pre-Resolve Variant Fields Summary

**OverlaySource struct replaces ~2KB ThemeMode clone fields on SystemTheme; with_overlay() replays merge+resolve pipeline from stored reader output and preset name**

## Performance

- **Duration:** 4 min
- **Started:** 2026-04-13T01:10:18Z
- **Completed:** 2026-04-13T01:14:27Z
- **Tasks:** 1
- **Files modified:** 2

## Accomplishments
- Defined OverlaySource struct storing reader_output (Theme), preset_name (String), and font_dpi (Option<f32>)
- Removed light_variant and dark_variant fields from SystemTheme, eliminating ~2KB of pre-resolve ThemeMode clones
- Rewrote with_overlay() to reconstruct pre-resolve variants on demand by replaying the full merge+resolve pipeline
- Updated run_pipeline() to clone reader_output and build OverlaySource instead of cloning variant ThemeModes
- Added round-trip test confirming overlay_source replay produces correct resolved output
- All 8 overlay tests pass including 2 new structural/round-trip tests

## Task Commits

Each task was committed atomically:

1. **Task 1: Define OverlaySource, rewrite with_overlay, remove light_variant/dark_variant** - `180258a` (refactor)

**Plan metadata:** (pending docs commit)

## Files Created/Modified
- `native-theme/src/lib.rs` - OverlaySource struct, SystemTheme field replacement, with_overlay() rewrite, test updates
- `native-theme/src/pipeline.rs` - run_pipeline builds OverlaySource, imports OverlaySource type

## Decisions Made
- OverlaySource is cloned unchanged on with_overlay() because the base reader data and preset name don't change when an overlay is applied -- only the user overlay differs per call
- Used unwrap_or on strip_suffix("-live") for preset names that aren't live presets (e.g. "catppuccin-mocha" used directly in tests)

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 78 complete: AccessibilityPreferences relocated (Plan 01), OverlaySource replaces variant clones (Plan 02)
- Ready for Phase 79 (border split + reader visibility audit)

---
## Self-Check: PASSED

- FOUND: native-theme/src/lib.rs
- FOUND: native-theme/src/pipeline.rs
- FOUND: 78-02-SUMMARY.md
- FOUND: commit 180258a

*Phase: 78-overlaysource-accessibilitypreferences-font-dpi-relocation*
*Completed: 2026-04-13*
