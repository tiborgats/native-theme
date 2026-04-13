---
phase: 82-icon-api-rework
plan: 01
subsystem: icons
tags: [cow, zero-copy, icon-api, kebab-case, drift-guard]

# Dependency graph
requires: []
provides:
  - "IconData::Svg stores Cow<'static, [u8]> -- zero-copy for bundled icons"
  - "IconProvider::icon_svg returns Option<Cow<'static, [u8]>>"
  - "IconRole::name() returns kebab-case identifier for all 42 variants"
  - "IconData::bytes() accessor for variant-agnostic byte access"
  - "IconSet drift-guard test catches missing from_name entries"
  - "native-theme-build codegen emits Cow<'static, [u8]> return type"
affects: [82-02, icon-loading, connectors]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Cow::Borrowed for bundled static data, Cow::Owned for runtime-loaded data"
    - "IconRole::name() kebab-case stable identifiers"

key-files:
  created: []
  modified:
    - native-theme/src/model/icons.rs
    - native-theme/src/model/bundled.rs
    - native-theme/src/model/animated.rs
    - native-theme/src/icons.rs
    - native-theme/src/freedesktop.rs
    - native-theme/src/spinners.rs
    - native-theme-build/src/codegen.rs
    - connectors/native-theme-iced/src/icons.rs
    - connectors/native-theme-gpui/src/icons.rs
    - connectors/native-theme-gpui/examples/showcase-gpui.rs

key-decisions:
  - "bundled_icon_svg stays returning Option<&'static [u8]> -- Cow wrapping at call site avoids churn in 400-line match blocks"
  - "IconRole::name() uses explicit match (not derive macro) for compile-time guaranteed kebab-case strings"
  - "iced connector uses cow.to_vec()/cow.into_owned() for from_memory() compatibility"

patterns-established:
  - "Cow::Borrowed for bundled icon paths, Cow::Owned for freedesktop/runtime icon paths"
  - "IconRole::name() kebab-case naming: category-variant (e.g. action-save, dialog-warning)"

requirements-completed: [ICON-03, ICON-04, ICON-06, ICON-07]

# Metrics
duration: 13min
completed: 2026-04-13
---

# Phase 82 Plan 01: Icon Model Cow Migration and IconRole Stability Summary

**IconData::Svg migrated to Cow<'static, [u8]> eliminating .to_vec() copies on bundled loads; IconRole::name() returns stable kebab-case identifiers with Display delegation**

## Performance

- **Duration:** 13 min
- **Started:** 2026-04-13T14:33:30Z
- **Completed:** 2026-04-13T14:46:33Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments
- IconData::Svg now stores Cow<'static, [u8]> -- bundled icons use Cow::Borrowed for zero-allocation access
- IconProvider::icon_svg trait method returns Option<Cow<'static, [u8]>> instead of Option<&'static [u8]>
- IconRole::name() provides stable kebab-case identifiers for all 42 variants
- Display for IconRole delegates to name() (human-readable output instead of Debug format)
- IconData::bytes() accessor provides variant-agnostic &[u8] access
- Drift-guard test ensures IconSet::from_name/name round-trips for all variants
- native-theme-build codegen emits Cow return type for generated IconProvider impls
- All connector crates (iced, gpui) updated and compile clean

## Task Commits

Each task was committed atomically:

1. **Task 1: Migrate IconData::Svg and IconProvider::icon_svg to Cow, add IconRole::name()** - `9a78b5f` (feat)
2. **Task 2: Propagate Cow migration to icons.rs, freedesktop, spinners, codegen, connectors** - `c748b1b` (feat)

## Files Created/Modified
- `native-theme/src/model/icons.rs` - IconData::Svg Cow, IconProvider::icon_svg Cow, IconRole::name(), Display, drift-guard test
- `native-theme/src/model/bundled.rs` - Updated doc comment for Cow::Borrowed guidance
- `native-theme/src/model/animated.rs` - Doc examples and tests updated for Cow
- `native-theme/src/icons.rs` - load_icon/load_system_icon_by_name use Cow::Borrowed, load_custom_icon passes Cow through
- `native-theme/src/freedesktop.rs` - Runtime SVG loads wrapped in Cow::Owned
- `native-theme/src/spinners.rs` - Pre-rotated frames use Cow::Owned
- `native-theme-build/src/codegen.rs` - Generated icon_svg returns Cow<'static, [u8]> with Cow::Borrowed wrapping
- `connectors/native-theme-iced/src/icons.rs` - to_svg_handle/into_svg_handle handle Cow, test providers updated
- `connectors/native-theme-gpui/src/icons.rs` - Test providers and data updated for Cow
- `connectors/native-theme-gpui/examples/showcase-gpui.rs` - Bundled icon data uses Cow::Borrowed

## Decisions Made
- bundled_icon_svg/bundled_icon_by_name retain their `Option<&'static [u8]>` return type; Cow wrapping happens at call sites to avoid changing 400+ line match blocks
- IconRole::name() uses an explicit match block returning &'static str for compile-time kebab-case guarantees
- iced connector uses cow.to_vec()/cow.into_owned() because iced_core::svg::Handle::from_memory requires Vec<u8>

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Added Cow import to IconProvider doc example**
- **Found during:** Task 2 verification
- **Issue:** Doc example for IconProvider trait missing `use std::borrow::Cow;` import, causing doctest failure
- **Fix:** Added the import to the doc example
- **Files modified:** native-theme/src/model/icons.rs
- **Verification:** All doctests pass
- **Committed in:** c748b1b (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 missing critical)
**Impact on plan:** Minor doc fix necessary for doctest compilation. No scope creep.

## Issues Encountered
- Pre-existing `build_gnome_spec_pure` dead_code warning in gnome/mod.rs causes clippy failures when checking connector crates with `-D warnings`; this is Phase 78 Plan 04 residual and out of scope for this plan

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Cow migration complete; Plan 02 (ICON-01, ICON-02: IconRole serde and From<&str>) can build on the stable name() foundation
- All connector crates compile clean with the new Cow types
- bundled_icon_load_produces_cow_borrowed test confirms zero-allocation bundled path

---
*Phase: 82-icon-api-rework*
*Completed: 2026-04-13*
