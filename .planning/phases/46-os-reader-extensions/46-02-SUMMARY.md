---
phase: 46-os-reader-extensions
plan: 02
subsystem: platform-reader
tags: [gnome, portal, gsettings, fontspec, accessibility, text-scale, ashpd]

requires:
  - phase: 44-model-restructure
    provides: ThemeVariant per-widget model with FontSpec, TextScale, ThemeDefaults accessibility fields
  - phase: 45-resolution-engine
    provides: resolve() pipeline and validate() that consume sparse ThemeVariant

provides:
  - GNOME reader producing sparse ThemeVariant with OS-only fields
  - parse_gnome_font_to_fontspec with weight extraction from Pango font strings
  - build_gnome_variant function for testable OS field population
  - Text scale computation from base font size via CSS multipliers
  - Accessibility flags from gsettings (text_scaling_factor, reduce_motion, overlay_mode)
  - Icon set from icon-theme gsetting
  - Portal Contrast::High mapped to high_contrast flag

affects: [46-os-reader-extensions, 47-pipeline-integration, 48-connector-mapping]

tech-stack:
  added: []
  patterns: [sparse-variant-population, gsettings-reader-helper, pango-weight-extraction]

key-files:
  created: []
  modified: [native-theme/src/gnome/mod.rs]

key-decisions:
  - "apply_accent targets defaults.accent/selection/focus_ring_color (3 fields, not 4 -- removed primary_background which doesn't exist in new model)"
  - "Weight extraction uses suffix-matching on WEIGHT_MODIFIERS table with longest-match-first ordering"
  - "Text scale section_heading left as None (GNOME CSS doesn't have a direct equivalent)"
  - "build_gnome_variant is synchronous (gsettings reads are sync; portal data passed in as args)"
  - "read_gsetting helper centralizes gsettings command pattern (strips quotes, checks success)"

patterns-established:
  - "Sparse variant pattern: build_*_variant() returns ThemeVariant::default() with only OS-readable fields set"
  - "Font weight extraction from platform font strings into CSS 100-900 scale"
  - "Text scale computation from base font size using platform-specific multipliers"

requirements-completed: [GNOME-01, GNOME-02, GNOME-03, GNOME-04, GNOME-05]

duration: 10min
completed: 2026-03-27
---

# Phase 46 Plan 02: GNOME Reader Rewrite Summary

**GNOME reader rewritten for per-widget ThemeVariant with Pango font weight extraction, CSS text scale multipliers, gsettings accessibility flags, icon_set, and portal contrast mapping**

## Performance

- **Duration:** 10 min
- **Started:** 2026-03-27T11:48:52Z
- **Completed:** 2026-03-27T11:59:42Z
- **Tasks:** 1
- **Files modified:** 1

## Accomplishments
- Rewrote gnome/mod.rs to target per-widget ThemeVariant model (removed all ThemeFonts/ThemeColors/ThemeGeometry/WidgetMetrics references)
- Added parse_gnome_font_to_fontspec with weight extraction supporting Bold, Light, Semi-Bold, Heavy, Thin, Extra-Bold, Ultra-Light, Medium, Black, and all Pango weight variants
- Added build_gnome_variant producing sparse ThemeVariant with fonts, text_scale, accessibility, icon_set, and dialog button_order
- 46 GNOME tests passing including font parsing, text scale computation, portal color conversion, accent propagation, high contrast mapping, and build_theme merge correctness

## Task Commits

Each task was committed atomically:

1. **Task 1: Rewrite GNOME reader to produce sparse ThemeVariant with fonts, text_scale, accessibility, and icon_set** - `4916f28` (feat)

## Files Created/Modified
- `native-theme/src/gnome/mod.rs` - Complete rewrite: removed old model types, added build_gnome_variant, parse_gnome_font_to_fontspec, compute_text_scale, read_gsetting helper, updated apply_accent/build_theme/from_gnome/from_kde_with_portal

## Decisions Made
- apply_accent now sets 3 fields (accent, selection, focus_ring_color) instead of the old 4 (primary_background removed -- doesn't exist in new model)
- Weight modifier extraction uses longest-match-first ordering in WEIGHT_MODIFIERS table to avoid "Light" matching before "Ultra-Light"
- Text scale section_heading left as None since GNOME CSS doesn't define a direct equivalent
- read_gsetting helper strips single quotes from gsettings output (gsettings wraps string values in single quotes)
- Adwaita preset's high_contrast = false is preserved when contrast is NoPreference (merge semantics: None doesn't override Some(false))

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed test assertion for normal contrast after merge**
- **Found during:** Task 1 (test verification)
- **Issue:** Test expected high_contrast to be None after build_theme, but Adwaita preset explicitly sets high_contrast = false, and the OS variant's None doesn't override it during merge
- **Fix:** Changed assertion from `is_none()` to `== Some(false)` to match actual merge semantics
- **Files modified:** native-theme/src/gnome/mod.rs
- **Verification:** All 46 GNOME tests pass
- **Committed in:** 4916f28

---

**Total deviations:** 1 auto-fixed (1 bug fix in test expectation)
**Impact on plan:** Test corrected to match actual merge semantics. No scope creep.

## Issues Encountered
- File was reverted by external process during initial write; re-wrote successfully on second attempt

## Known Stubs
None - all OS-readable fields are wired to gsettings reads or portal data.

## Next Phase Readiness
- GNOME reader complete and tested, ready for integration in Phase 47 pipeline
- Other readers (KDE, macOS, Windows) still need Phase 46 rewrites (plans 01, 03, 04)

---
*Phase: 46-os-reader-extensions*
*Completed: 2026-03-27*
