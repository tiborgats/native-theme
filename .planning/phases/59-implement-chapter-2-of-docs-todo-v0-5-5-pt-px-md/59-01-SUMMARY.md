---
phase: 59-implement-chapter-2-of-docs-todo-v0-5-5-pt-px-md
plan: 01
subsystem: model
tags: [font-size, type-safety, serde, proxy-struct, pt-px]

# Dependency graph
requires:
  - phase: 58-font-pt-px-dpi-conversion-fix
    provides: font_dpi auto-detection and pipeline propagation
provides:
  - FontSize enum with Pt(f32) and Px(f32) variants
  - FontSpec.size as Option<FontSize> with serde proxy struct
  - TextScaleEntry.size as Option<FontSize> with serde proxy struct
  - Validation converts FontSize to f32 via to_px(dpi)
  - Phase 1.5 (resolve_font_dpi_conversion) deleted from pipeline
affects: [59-02-PLAN (TOML preset renames), 59-03-PLAN (test updates)]

# Tech tracking
tech-stack:
  added: []
  patterns: [serde proxy struct with try_from/into for tagged union serialization]

key-files:
  created: []
  modified:
    - native-theme/src/model/font.rs
    - native-theme/src/model/mod.rs
    - native-theme/src/lib.rs
    - native-theme/src/resolve/inheritance.rs
    - native-theme/src/resolve/mod.rs
    - native-theme/src/resolve/validate.rs
    - native-theme/src/kde/fonts.rs
    - native-theme/src/gnome/mod.rs
    - native-theme/src/windows.rs
    - native-theme/src/macos.rs

key-decisions:
  - "FontSize enum has no Serialize/Deserialize -- serde mapping lives on parent proxy structs (FontSpecRaw, TextScaleEntryRaw)"
  - "TryFrom<FontSpecRaw> rejects both size_pt and size_px set simultaneously with descriptive error string"
  - "line_height stays f32 (layout metric, not font size) -- converted alongside size when unit is points"
  - "Phase 1.5 (resolve_font_dpi_conversion) fully deleted -- pt-to-px conversion moved to validate"

patterns-established:
  - "Serde proxy struct pattern: #[serde(try_from/into)] with Raw suffix for tagged-union field mapping"
  - "FontSize::to_px(dpi) as the single conversion point from typed font sizes to f32 pixels"

requirements-completed: []

# Metrics
duration: 8min
completed: 2026-04-08
---

# Phase 59 Plan 01: Core Type System Refactoring Summary

**FontSize enum with Pt/Px variants replacing ambiguous Option<f32> font sizes, with serde proxy structs mapping to size_pt/size_px TOML keys and validation-time pt-to-px conversion**

## Performance

- **Duration:** 8 min
- **Started:** 2026-04-08T14:07:38Z
- **Completed:** 2026-04-08T14:15:53Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments
- Introduced FontSize enum with compile-time unit safety (Pt/Px variants) and to_px/raw/is_pt methods
- Replaced ambiguous Option<f32> font sizes with Option<FontSize> on FontSpec and TextScaleEntry, with serde proxy structs for TOML size_pt/size_px mapping
- Deleted Phase 1.5 (resolve_font_dpi_conversion) from the resolve pipeline, moving pt-to-px conversion to validate() where it's an explicit type transformation
- Updated all 4 OS readers (KDE, GNOME, Windows, macOS) to wrap font sizes in FontSize::Pt(...)
- Added dpi parameter to require_font/require_font_opt/require_text_scale_entry (26 call sites updated)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add FontSize enum and update FontSpec/TextScaleEntry with proxy structs** - `c87dd59` (feat)
2. **Task 2: Update OS readers, resolve pipeline, and validation** - `a7839a7` (feat)

## Files Created/Modified
- `native-theme/src/model/font.rs` - FontSize enum, FontSpecRaw/TextScaleEntryRaw proxy structs, updated FIELD_NAMES
- `native-theme/src/model/mod.rs` - Re-export FontSize
- `native-theme/src/lib.rs` - Re-export FontSize publicly
- `native-theme/src/resolve/inheritance.rs` - Deleted convert_pt_to_px and resolve_font_dpi_conversion, updated text_scale line_height to use .raw()
- `native-theme/src/resolve/mod.rs` - Removed Phase 1.5 call, updated doc comments
- `native-theme/src/resolve/validate.rs` - Added dpi parameter to font/text_scale validators, FontSize::to_px conversion
- `native-theme/src/kde/fonts.rs` - Wrap parsed size in FontSize::Pt(...)
- `native-theme/src/gnome/mod.rs` - Wrap parsed size in FontSize::Pt(...)
- `native-theme/src/windows.rs` - Wrap parsed size in FontSize::Pt(...), update test constructors
- `native-theme/src/macos.rs` - Wrap parsed size in FontSize::Pt(...), update test constructors

## Decisions Made
- FontSize enum has no Serialize/Deserialize -- serde mapping lives on parent proxy structs to keep TOML keys flat (size_pt/size_px)
- TryFrom returns descriptive error string when both size_pt and size_px are set (threat T-59-01 mitigation)
- line_height stays as bare f32 -- converted alongside its sibling size using same DPI factor when unit is points
- Validation extracts dpi once at top of validate() body, passes to all 26 require_font/require_text_scale_entry calls

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added FontSize to model/mod.rs re-exports**
- **Found during:** Task 1
- **Issue:** FontSize was defined in model::font but not re-exported from model, causing unresolved import in lib.rs
- **Fix:** Added FontSize to the `pub use font::{...}` line in model/mod.rs
- **Files modified:** native-theme/src/model/mod.rs
- **Committed in:** c87dd59 (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Necessary re-export to make the type publicly accessible. No scope creep.

## Issues Encountered
None -- library code compiles cleanly. Test code has expected compilation errors that will be fixed by Plans 02 (TOML renames) and 03 (test updates).

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- FontSize type system is in place and library code compiles
- Plan 02 needs to rename TOML preset keys from `size` to `size_pt`/`size_px`
- Plan 03 needs to update all test constructors and add FontSize-specific tests
- Test compilation will fail until Plans 02 and 03 complete

---
*Phase: 59-implement-chapter-2-of-docs-todo-v0-5-5-pt-px-md*
*Completed: 2026-04-08*
