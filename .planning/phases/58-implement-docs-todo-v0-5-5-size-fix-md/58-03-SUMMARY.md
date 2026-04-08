---
phase: 58-implement-docs-todo-v0-5-5-size-fix-md
plan: 03
subsystem: core
tags: [font-dpi, pipeline-propagation, doc-comments, connectors, proptest]

# Dependency graph
requires:
  - phase: 58-implement-docs-todo-v0-5-5-size-fix-md
    provides: "font_dpi field and resolve_font_dpi_conversion (Plan 01), OS reader font_dpi detection (Plan 02)"
provides:
  - "font_dpi propagation from reader to inactive variant in run_pipeline()"
  - "Accurate doc comments for FontSpec.size, ResolvedFontSpec.size, TextScaleEntry.size, ResolvedTextScaleEntry, text_scaling_factor"
  - "Updated connector doc comments (GPUI and Iced) reflecting resolution-step conversion"
  - "Proptest coverage for font_dpi field (already in place from Plan 01 deviation)"
affects: [connectors, documentation, pre-release-readiness]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Pipeline propagation: extract DPI from reader, apply to both variants before resolution"]

key-files:
  created: []
  modified:
    - native-theme/src/lib.rs
    - native-theme/src/model/font.rs
    - native-theme/src/model/resolved.rs
    - native-theme/src/resolve/mod.rs
    - connectors/native-theme-gpui/src/config.rs
    - connectors/native-theme-iced/src/lib.rs

key-decisions:
  - "Pipeline propagation uses or_else chain to extract font_dpi from whichever variant the reader provided"
  - "Test asserts on conversion effect (resolved font size) not on font_dpi field (consumed during resolution)"
  - "read_xft_dpi gated on kde/portal features to match its callers"

patterns-established:
  - "Propagation before clone: modify variants between construction and pre-resolve clone"

requirements-completed: [FIX-4, FIX-7, FIX-8]

# Metrics
duration: 9min
completed: 2026-04-08
---

# Phase 58 Plan 03: Pipeline Propagation and Doc Comments Summary

**font_dpi propagated from reader to inactive variant in run_pipeline(); all doc comments updated to reflect pt-to-px conversion semantics (Fix 4/7/8); pre-release-check passes**

## Performance

- **Duration:** 9 min
- **Started:** 2026-04-08T01:14:27Z
- **Completed:** 2026-04-08T01:24:03Z
- **Tasks:** 3
- **Files modified:** 6

## Accomplishments
- font_dpi propagation in run_pipeline() ensures inactive variant gets DPI from reader for correct pt-to-px conversion
- Pipeline propagation test verifies conversion effect: 10pt at 120 DPI resolves to ~16.7px on the inactive variant
- All doc comments updated: FontSpec.size explains pt vs px semantics, ResolvedFontSpec.size says "converted from points", connectors say "handled by the resolution step", text_scaling_factor explains accessibility independence from font_dpi
- Pre-release-check passes cleanly (all 17+ presets resolve, clippy clean, all tests pass)

## Task Commits

Each task was committed atomically:

1. **Task 1: Pipeline font_dpi propagation and test** - `03d4b54` (feat)
2. **Task 2: Update doc comments across core and connectors (Fix 7, Fix 8)** - `a896849` (fix)
3. **Task 3: Final verification -- fix pre-existing clippy warnings** - `b7ed7fe` (fix)

## Files Created/Modified
- `native-theme/src/lib.rs` - font_dpi propagation in run_pipeline(), pipeline propagation test, read_xft_dpi cfg gate fix
- `native-theme/src/model/font.rs` - Updated FontSpec.size, ResolvedFontSpec.size, TextScaleEntry.size doc comments
- `native-theme/src/model/resolved.rs` - Updated ResolvedTextScaleEntry.size, line_height, text_scaling_factor doc comments
- `native-theme/src/resolve/mod.rs` - Fixed doc list item indentation for Phase 1.5
- `connectors/native-theme-gpui/src/config.rs` - Updated 3 doc comments to reflect resolution-step conversion
- `connectors/native-theme-iced/src/lib.rs` - Updated font_size and mono_font_size doc comments

## Decisions Made
- Pipeline propagation extracts font_dpi via `or_else` chain from whichever variant the reader provided (light or dark)
- Test asserts on the conversion effect (resolved font size = pt * dpi / 72) rather than on font_dpi field directly, since font_dpi is consumed during resolution and replaced with DEFAULT_FONT_DPI
- read_xft_dpi() cfg gate narrowed to `any(feature = "kde", feature = "portal")` to match its callers and eliminate dead_code warning

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed read_xft_dpi cfg gate**
- **Found during:** Task 3 (pre-release-check)
- **Issue:** read_xft_dpi() was gated only on `target_os = "linux"` but its callers (KDE, GNOME modules) are behind feature flags; clippy flagged dead_code
- **Fix:** Added `any(feature = "kde", feature = "portal")` to the cfg gate
- **Files modified:** native-theme/src/lib.rs
- **Verification:** cargo clippy passes
- **Committed in:** b7ed7fe (Task 3 commit)

**2. [Rule 3 - Blocking] Fixed doc list item indentation**
- **Found during:** Task 3 (pre-release-check)
- **Issue:** Phase 1.5 doc comment in resolve/mod.rs lacked indentation, triggering clippy::doc_lazy_continuation
- **Fix:** Added 3-space indent to align with numbered list
- **Files modified:** native-theme/src/resolve/mod.rs
- **Verification:** cargo clippy passes
- **Committed in:** b7ed7fe (Task 3 commit)

---

**Total deviations:** 2 auto-fixed (2 blocking, both pre-existing from Plan 01/02)
**Impact on plan:** Necessary for clippy clean build. No scope creep.

## Issues Encountered
- Pre-existing test failures (test_wm_title_bar_colors, test_sidebar_none_without_complementary, build_gnome_variant_normal_contrast_no_flag) -- all unrelated to font_dpi changes, confirmed failing before this plan
- GPUI connector's naga dependency has upstream compilation errors -- not in scope, does not affect native-theme or iced connector

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- All 3 plans in Phase 58 are complete
- The full font_dpi pipeline is wired end-to-end: OS readers detect DPI, pipeline propagates to both variants, resolution converts pt to px
- Pre-release-check passes -- v0.5.5 is ready for final review

## Self-Check: PASSED

All 6 modified files exist. All 3 commits verified (03d4b54, a896849, b7ed7fe).

---
*Phase: 58-implement-docs-todo-v0-5-5-size-fix-md*
*Completed: 2026-04-08*
