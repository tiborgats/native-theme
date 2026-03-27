---
phase: 47-os-first-pipeline
plan: 02
subsystem: api
tags: [system-theme, overlay, toml, resolve, accent-propagation]

# Dependency graph
requires:
  - phase: 47-os-first-pipeline
    provides: SystemTheme with pre-resolve ThemeVariant fields, resolve_variant(), run_pipeline()
  - phase: 45-resolution-engine
    provides: ThemeVariant::resolve() and validate() producing ResolvedTheme
  - phase: 44-data-model-widget-structs
    provides: NativeTheme, ThemeVariant, merge(), NativeTheme::from_toml()
provides:
  - SystemTheme::with_overlay() merging app overlays onto pre-resolve variants with re-resolution
  - SystemTheme::with_overlay_toml() convenience for TOML string overlays
affects: [48-connectors]

# Tech tracking
tech-stack:
  added: []
  patterns: [pre-resolve-overlay-then-resolve, toml-overlay-api]

key-files:
  created: []
  modified:
    - native-theme/src/lib.rs

key-decisions:
  - "Overlay merges onto pre-resolve ThemeVariant (not ResolvedTheme) to avoid double-resolve idempotency issue"
  - "with_overlay() consumes self (move semantics) returning new SystemTheme -- immutable overlay pattern"
  - "Removed dead_code allow on light_variant/dark_variant since overlay now uses them"

patterns-established:
  - "Pre-resolve overlay: clone pre-resolve variant, merge overlay, resolve, validate -- ensures source field propagation"
  - "TOML convenience: with_overlay_toml wraps from_toml + with_overlay for ergonomic app API"

requirements-completed: [PIPE-03]

# Metrics
duration: 4min
completed: 2026-03-27
---

# Phase 47 Plan 02: App TOML Overlay Summary

**SystemTheme::with_overlay() merges app customizations onto pre-resolve ThemeVariant and re-resolves, so changing accent propagates to button.primary_bg, checkbox.checked_bg, slider.fill, and all accent-derived fields**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-27T14:07:51Z
- **Completed:** 2026-03-27T14:11:36Z
- **Tasks:** 1 (TDD: RED + GREEN)
- **Files modified:** 1

## Accomplishments
- with_overlay() method on SystemTheme clones pre-resolve variants, merges overlay, re-resolves and validates both light and dark
- with_overlay_toml() convenience parses TOML string and delegates to with_overlay()
- 6 new tests: accent propagation (5 widget fields verified), unrelated field preservation, empty noop, both variants independently, font family override, TOML convenience
- Removed dead_code allow on pre-resolve variant fields (now actively used)

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement SystemTheme::with_overlay() + accent propagation tests** (TDD)
   - `9bf1afd` (test: add failing tests for SystemTheme overlay)
   - `81479fb` (feat: implement SystemTheme::with_overlay() and with_overlay_toml())

## Files Created/Modified
- `native-theme/src/lib.rs` - Added with_overlay(), with_overlay_toml() methods; removed dead_code allow on variant fields; added 6 overlay tests

## Decisions Made
- Overlay merges onto pre-resolve ThemeVariant (not ResolvedTheme) to avoid the idempotency issue where resolve() won't re-derive fields already filled by a previous pass
- with_overlay() uses move semantics (consumes self) returning a new SystemTheme for immutable overlay pattern
- Removed dead_code allow annotations on light_variant/dark_variant since overlay now uses them

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

Minor: The TOML convenience test initially failed because NativeTheme::from_toml requires a `name` field (not optional in serde). Fixed by adding `name = "overlay"` to the test TOML string.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Phase 47 is complete: SystemTheme provides full OS-first pipeline with overlay support
- Phase 48 (connectors) can consume &ResolvedTheme from SystemTheme.active() or SystemTheme.pick()
- App developers can customize themes via with_overlay() or with_overlay_toml() before passing to connectors
- All 374 tests pass (368 pre-existing + 6 new)

## Self-Check: PASSED

- All created/modified files exist
- All commit hashes verified: 9bf1afd, 81479fb

---
*Phase: 47-os-first-pipeline*
*Completed: 2026-03-27*
