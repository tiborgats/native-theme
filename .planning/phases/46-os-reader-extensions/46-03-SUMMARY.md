---
phase: 46-os-reader-extensions
plan: 03
subsystem: macos-reader
tags: [objc2, nscolor, nsfont, accessibility, scrollbar, text-scale, appkit]

requires:
  - phase: 44-model-restructure
    provides: ThemeVariant per-widget struct pairs, FontSpec, TextScale, ThemeDefaults accessibility fields
  - phase: 45-resolution-engine
    provides: resolve() safety nets that fill input.caret from defaults.foreground
provides:
  - Extended macOS reader with per-widget fonts (menu, tooltip, title bar) with weight extraction
  - Text scale entries from Apple type scale ratios
  - Additional NSColor values for per-widget fields (placeholder, selection_inactive, alternate_row, header_foreground, grid_color)
  - Scrollbar overlay mode from NSScroller.preferredScrollerStyle
  - Accessibility flags (reduce_motion, high_contrast, reduce_transparency, text_scaling_factor)
  - Dialog button order (leading_affirmative) for macOS convention
affects: [47-platform-toml, 48-connector-updates]

tech-stack:
  added: []
  patterns:
    - "fontspec_from_nsfont() + nsfont_weight_to_css() for extracting FontSpec with CSS weight from any NSFont"
    - "PerWidgetColors struct for passing appearance-dependent per-widget colors alongside ThemeDefaults"
    - "compute_text_scale(system_size) for proportional type scale from Apple ratios"
    - "MainThreadMarker::new().and_then() pattern for main-thread-required APIs"

key-files:
  created: []
  modified:
    - native-theme/src/macos.rs
    - native-theme/Cargo.toml

key-decisions:
  - "input.caret intentionally not read: textInsertionPointColor requires macOS 14+; resolve() safety net fills it from defaults.foreground"
  - "Text scale uses proportional computation from system font size rather than NSFontTextStyle API for broader compatibility"
  - "Weight extraction uses NSFontDescriptor traits dictionary (NSFontWeightTrait) for AppKit-to-CSS weight mapping"
  - "read_semantic_colors renamed to read_appearance_colors returning (ThemeDefaults, PerWidgetColors) tuple"
  - "Per-widget colors applied after build_theme() in from_macos() since they are appearance-dependent"

patterns-established:
  - "fontspec_from_nsfont: unified NSFont-to-FontSpec extraction with weight"
  - "PerWidgetColors: appearance-dependent per-widget color struct returned alongside ThemeDefaults"

requirements-completed: [MACOS-01, MACOS-02, MACOS-03, MACOS-04, MACOS-05]

duration: 15min
completed: 2026-03-27
---

# Phase 46 Plan 03: macOS Reader Extensions Summary

**Extended macOS reader with per-widget fonts (menu/tooltip/title bar) with CSS weight extraction, Apple type scale entries, 5 additional NSColor per-widget fields, NSScroller overlay mode, and all 4 accessibility flags**

## Performance

- **Duration:** 15 min
- **Started:** 2026-03-27T11:49:57Z
- **Completed:** 2026-03-27T12:04:28Z
- **Tasks:** 2
- **Files modified:** 2 (native-theme/src/macos.rs, native-theme/Cargo.toml)

## Accomplishments
- Per-widget fonts with weight extraction: nsfont_weight_to_css maps AppKit weight (-1.0..1.0) to CSS (100-900), fontspec_from_nsfont extracts family/size/weight from any NSFont, read_per_widget_fonts reads menu/tooltip/title bar fonts
- Text scale entries computed from Apple's type scale ratios (caption 11pt, subheadline 15pt, title2 22pt, largeTitle 34pt at default 13pt, scaling proportionally)
- Additional NSColor values: placeholderTextColor, unemphasizedSelectedContentBackgroundColor (selection_inactive), alternatingContentBackgroundColors (alternate_row), headerTextColor, gridColor
- Scrollbar overlay mode from NSScroller.preferredScrollerStyle via MainThreadMarker
- Accessibility: reduce_motion, high_contrast, reduce_transparency from NSWorkspace, text_scaling_factor derived from system font size vs 13pt default
- Dialog button_order set to LeadingAffirmative per macOS convention

## Task Commits

Each task was committed atomically:

1. **Task 1: Add per-widget fonts with weight extraction and text scale entries** - `223316f` (feat)
2. **Task 2: Add additional NSColor values, scrollbar overlay mode, and accessibility flags** - `d87483c` (feat)

## Files Created/Modified
- `native-theme/src/macos.rs` - Extended macOS reader with all MACOS-01 through MACOS-05 requirements
- `native-theme/Cargo.toml` - Added NSScroller feature to objc2-app-kit dependencies

## Decisions Made
- **input.caret deferred:** NSColor::textInsertionPointColor requires macOS 14+, so input.caret is intentionally not read by the macOS reader. The resolve() safety net (resolve.rs:167-169) fills input.caret from defaults.foreground, producing correct results for all macOS versions.
- **Text scale uses proportional computation:** Rather than calling NSFont.preferredFontForTextStyle (which has complex binding requirements), compute_text_scale uses Apple's known type scale ratios (11/13, 15/13, 22/13, 34/13) applied to the actual system font size.
- **read_semantic_colors renamed to read_appearance_colors:** Returns a (ThemeDefaults, PerWidgetColors) tuple to capture both defaults-level and per-widget colors in a single appearance block pass.
- **Per-widget colors applied post-build_theme:** Since per-widget colors are appearance-dependent (unlike fonts/text-scale), they are applied to variants after build_theme() returns.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] NSNumber.as_f64() does not exist in objc2-foundation 0.3**
- **Found during:** Task 1 (weight extraction)
- **Issue:** Plan referenced `as_f64()` but the actual API is `doubleValue()` (generated Obj-C binding)
- **Fix:** Used `unsafe { weight_num.doubleValue() }` instead
- **Files modified:** native-theme/src/macos.rs
- **Verification:** Compiles cleanly with `cargo check --features macos`
- **Committed in:** d87483c (Task 2 commit, along with Task 2 changes)

**2. [Rule 3 - Blocking] MainThreadMarker::alloc() is not a constructor**
- **Found during:** Task 2 (scrollbar overlay mode)
- **Issue:** Plan referenced `MainThreadMarker::alloc()` but the constructor is `MainThreadMarker::new()` returning `Option<Self>`
- **Fix:** Used `MainThreadMarker::new().and_then(read_scrollbar_style)` pattern
- **Files modified:** native-theme/src/macos.rs
- **Verification:** Compiles cleanly
- **Committed in:** d87483c

---

**Total deviations:** 2 auto-fixed (2 blocking API mismatches)
**Impact on plan:** Both fixes required for correct API usage. No scope change.

## Issues Encountered
None beyond the API mismatches noted above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All 5 macOS requirements (MACOS-01 through MACOS-05) implemented
- macOS reader now produces sparse ThemeVariant with per-widget fields that the resolution pipeline can fill
- Platform TOML slimming (removing gear fields now provided by the reader) can proceed in Phase 47
- Cannot test on Linux host; compilation-verified only

---
*Phase: 46-os-reader-extensions*
*Completed: 2026-03-27*
