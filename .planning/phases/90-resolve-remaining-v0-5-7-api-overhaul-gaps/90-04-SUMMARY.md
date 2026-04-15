---
phase: 90-resolve-remaining-v0-5-7-api-overhaul-gaps
plan: 04
subsystem: api
tags: [error-handling, theme-api, result-type, preset-cache]

requires:
  - phase: 90-01
    provides: "Rgba::new() rename (files already updated)"
  - phase: 90-03
    provides: "ThemeSubscription rename (watch/ and connector examples updated)"
  - phase: 90-05
    provides: "intern_font_family in model/font.rs"
provides:
  - "Error::NoVariant variant for empty themes"
  - "pick_variant/into_variant return Result instead of Option"
  - "Theme::new() deleted (use struct literal with Default)"
  - "Preset cache Parsed type documented"
  - "list_presets_for_platform() Vec return type documented"
affects: [connectors, documentation, api-surface]

tech-stack:
  added: []
  patterns:
    - "Theme construction via struct literal: Theme { name: ..., ..Theme::default() }"
    - "NoVariant error for variant-less themes instead of Option::None"

key-files:
  created: []
  modified:
    - native-theme/src/error.rs
    - native-theme/src/model/mod.rs
    - native-theme/src/presets.rs
    - native-theme/src/resolve/tests.rs
    - native-theme/tests/resolve_and_validate.rs
    - native-theme/tests/merge_behavior.rs
    - native-theme/tests/proptest_roundtrip.rs
    - native-theme/tests/serde_roundtrip.rs
    - native-theme/README.md
    - connectors/native-theme-iced/src/lib.rs
    - connectors/native-theme-iced/examples/showcase-iced.rs
    - connectors/native-theme-iced/README.md
    - connectors/native-theme-gpui/src/lib.rs
    - connectors/native-theme-gpui/examples/showcase-gpui.rs
    - connectors/native-theme-gpui/README.md
    - README.md

key-decisions:
  - "Theme::new() replaced with struct literal + Default, not a builder or new() with different signature"
  - "NoVariant error categorized as ErrorKind::Resolution (theme data issue, not platform issue)"
  - "Connector from_preset functions propagate NoVariant via ? instead of custom error messages"
  - "Showcase examples use .ok() to bridge Result->Option where function returns Option"

patterns-established:
  - "Theme construction: Theme { name: \"...\".into(), ..Theme::default() }"

requirements-completed: []

duration: 10min
completed: 2026-04-15
---

# Phase 90 Plan 04: Theme API Cleanup Summary

**Deleted Theme::new(), changed pick/into_variant from Option to Result<_, NoVariant>, documented preset cache type**

## Performance

- **Duration:** 10 min
- **Started:** 2026-04-15T11:35:43Z
- **Completed:** 2026-04-15T11:45:55Z
- **Tasks:** 2
- **Files modified:** 18

## Accomplishments
- Added Error::NoVariant { mode } variant providing descriptive errors when themes have no variants
- Changed pick_variant/into_variant from Option to Result, eliminating all .ok_or("no variant") patterns
- Deleted Theme::new() constructor, migrated 30+ callers to struct literal with Default
- Documented list_presets_for_platform() Vec return type rationale
- Added doc comment on preset cache Parsed type explaining String error choice (GAP-9)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add NoVariant error, change pick/into_variant to Result, delete Theme::new()** - `15182eb` (feat)
2. **Task 2: Update connector callers and add preset cache comment** - `1aae7b1` (feat)

## Files Created/Modified
- `native-theme/src/error.rs` - Added NoVariant variant, updated kind/Display/source impls
- `native-theme/src/model/mod.rs` - Deleted Theme::new(), changed pick/into_variant signatures, updated tests, added list_presets_for_platform doc note
- `native-theme/src/presets.rs` - Added doc comment on Parsed type alias
- `native-theme/src/resolve/tests.rs` - Migrated Theme::new() callers to struct literal
- `native-theme/tests/resolve_and_validate.rs` - Updated is_none/is_some to is_err/is_ok, migrated Theme::new()
- `native-theme/tests/merge_behavior.rs` - Migrated Theme::new() callers
- `native-theme/tests/proptest_roundtrip.rs` - Migrated Theme::new() caller
- `native-theme/tests/serde_roundtrip.rs` - Migrated Theme::new() callers
- `native-theme/README.md` - Removed .ok_or("no variant") from doc example
- `connectors/native-theme-iced/src/lib.rs` - Removed .ok_or_else() from from_preset, changed Some to Ok in test
- `connectors/native-theme-iced/examples/showcase-iced.rs` - Changed Some/None to Ok/Err, added .ok() bridge
- `connectors/native-theme-iced/README.md` - Removed .ok_or("no variant")
- `connectors/native-theme-gpui/src/lib.rs` - Removed .ok_or_else() from from_preset, updated doc comment
- `connectors/native-theme-gpui/examples/showcase-gpui.rs` - Changed if let Some to if let Ok
- `connectors/native-theme-gpui/README.md` - Removed .ok_or("no variant")
- `README.md` - Simplified .ok_or("no variant").unwrap() to .unwrap()

## Decisions Made
- Theme::new() replaced with struct literal + Default, keeping the Cow<'static, str> name field via .into()
- NoVariant error categorized as ErrorKind::Resolution (theme content issue, not a platform issue)
- Connector from_preset functions now propagate NoVariant directly via ? instead of wrapping in ReaderFailed
- Showcase examples that return Option use .ok() to bridge the new Result return type

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed additional test files not listed in plan**
- **Found during:** Task 1
- **Issue:** Plan listed model/mod.rs, resolve/tests.rs, prelude_smoke.rs, resolve_and_validate.rs as callers, but Theme::new() was also used in serde_roundtrip.rs, proptest_roundtrip.rs, and merge_behavior.rs
- **Fix:** Migrated all additional Theme::new() callers to struct literal syntax
- **Files modified:** native-theme/tests/serde_roundtrip.rs, native-theme/tests/proptest_roundtrip.rs, native-theme/tests/merge_behavior.rs
- **Verification:** All tests pass
- **Committed in:** 15182eb (Task 1 commit)

**2. [Rule 1 - Bug] Fixed workspace README .ok_or patterns**
- **Found during:** Task 2
- **Issue:** Plan listed connector READMEs but workspace-level README.md also had .ok_or("no variant") patterns
- **Fix:** Simplified to .unwrap() (Result already provides unwrap)
- **Files modified:** README.md
- **Verification:** Confirmed no .ok_or("no variant") patterns remain in production code
- **Committed in:** 1aae7b1 (Task 2 commit)

**3. [Rule 1 - Bug] Removed unused `mode` variable in iced from_preset**
- **Found during:** Task 2
- **Issue:** After removing the .ok_or_else() error message, the `mode` local variable was unused
- **Fix:** Removed the dead `let mode = ...` line
- **Files modified:** connectors/native-theme-iced/src/lib.rs
- **Committed in:** 1aae7b1 (Task 2 commit)

---

**Total deviations:** 3 auto-fixed (3 bugs)
**Impact on plan:** All auto-fixes necessary for correctness. No scope creep.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Theme API cleanup (GAP-3, GAP-9) complete
- All connector callers updated for new Result return types
- Ready for remaining Phase 90 plans

---
*Phase: 90-resolve-remaining-v0-5-7-api-overhaul-gaps*
*Completed: 2026-04-15*
