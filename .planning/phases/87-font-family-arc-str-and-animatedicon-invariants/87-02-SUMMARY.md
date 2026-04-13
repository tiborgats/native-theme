---
phase: 87-font-family-arc-str-and-animatedicon-invariants
plan: 02
subsystem: model
tags: [arc-str, font, serde-rc, allocation, clone-optimization]

# Dependency graph
requires: []
provides:
  - "FontSpec::family as Option<Arc<str>> (cheap clones via ref-counting)"
  - "ResolvedFontSpec::family as Arc<str> (shared allocation across widgets)"
  - "serde rc feature enabled in workspace for Arc<str> serialize/deserialize"
affects: [connectors, presets, phase-88]

# Tech tracking
tech-stack:
  added: ["serde rc feature"]
  patterns: ["Arc<str> for shared immutable strings", ".as_ref() for Arc<str> to &str comparison"]

key-files:
  created: []
  modified:
    - "Cargo.toml"
    - "native-theme/src/model/font.rs"
    - "native-theme/src/model/resolved.rs"
    - "native-theme/src/model/widgets/mod.rs"
    - "native-theme/src/resolve/tests.rs"
    - "native-theme/src/presets.rs"
    - "native-theme/src/lib.rs"
    - "native-theme/src/windows.rs"
    - "native-theme/src/macos.rs"
    - "native-theme/src/gnome/mod.rs"
    - "native-theme/src/kde/fonts.rs"
    - "native-theme/src/kde/mod.rs"
    - "native-theme/tests/proptest_roundtrip.rs"

key-decisions:
  - "Used .as_ref() for Arc<str> to &str comparison in assertions (Arc<str> has no PartialEq<&str> impl)"
  - "Changed .to_string() to .into() for &str->Arc<str> in platform readers (avoids intermediate String allocation)"
  - "kde/fonts.rs: renamed local var to family_str to avoid shadowing with type change"

patterns-established:
  - "Arc<str> shared strings: use .into() from &str, .as_ref() for &str access"
  - "Serde rc feature: workspace-level feature enables Arc<str> derive support"

requirements-completed: [LAYOUT-04]

# Metrics
duration: 8min
completed: 2026-04-13
---

# Phase 87 Plan 02: FontSpec family Arc<str> Migration Summary

**Migrated FontSpec::family and ResolvedFontSpec::family from String to Arc<str> with serde rc feature, eliminating per-widget string clones via shared reference counting**

## Performance

- **Duration:** 8 min
- **Started:** 2026-04-13T22:28:11Z
- **Completed:** 2026-04-13T22:36:42Z
- **Tasks:** 1
- **Files modified:** 13

## Accomplishments
- FontSpec::family migrated from Option<String> to Option<Arc<str>>
- ResolvedFontSpec::family migrated from String to Arc<str>
- FontSpecRaw serde proxy updated to Option<Arc<str>> with serde rc feature
- Arc pointer-equality test confirms cloning shares allocation (no deep copy)
- All 615 native-theme tests pass (505 lib + 12 integration + 55 proptest + 43 doctests)
- Platform readers (KDE, GNOME, macOS, Windows) updated for Arc<str> construction

## Task Commits

Each task was committed atomically:

1. **Task 1: Enable serde rc feature and migrate FontSpec + ResolvedFontSpec family to Arc<str>** - `bed5168` (feat)

## Files Created/Modified
- `Cargo.toml` - Added serde rc feature to workspace dependencies
- `native-theme/src/model/font.rs` - Core type change: family fields to Arc<str>, added Arc sharing test
- `native-theme/src/model/resolved.rs` - Updated test assertions to use .as_ref()
- `native-theme/src/model/widgets/mod.rs` - Updated test assertions to use .as_ref()
- `native-theme/src/resolve/tests.rs` - Updated family assertions and .to_string() to .into()
- `native-theme/src/presets.rs` - Updated family assertions to use .as_ref()
- `native-theme/src/lib.rs` - Updated overlay test family assertion
- `native-theme/src/windows.rs` - Changed .to_string() to .into(), updated assertions
- `native-theme/src/macos.rs` - Changed .to_string() to .into(), updated assertion
- `native-theme/src/gnome/mod.rs` - Changed family.to_string() to family.into()
- `native-theme/src/kde/fonts.rs` - Changed String construction to &str.into()
- `native-theme/src/kde/mod.rs` - Updated family assertion to use .as_ref()
- `native-theme/tests/proptest_roundtrip.rs` - Added Arc::from() mapping for proptest family strategy

## Decisions Made
- Used `.as_ref()` for `Arc<str>` to `&str` comparison in `assert_eq!` calls because `Arc<str>` does not implement `PartialEq<&str>` in the standard library
- Converted `.to_string()` font family constructors to `.into()` which goes directly from `&str` to `Arc<str>` (avoids unnecessary intermediate `String` allocation)
- In `kde/fonts.rs`, renamed `family` local variable to `family_str` since the trimmed `&str` is now used directly with `.into()` instead of being converted to `String` first

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed compilation errors in files outside plan scope**
- **Found during:** Task 1 (type migration)
- **Issue:** Changing core types caused compilation errors in 10 additional files beyond the 8 listed in the plan (widgets/mod.rs, presets.rs, resolve/tests.rs, lib.rs, windows.rs, macos.rs, gnome/mod.rs, kde/fonts.rs, kde/mod.rs, proptest_roundtrip.rs)
- **Fix:** Updated all direct `family == &str` comparisons to use `.as_ref()`, changed `.to_string()` constructors to `.into()`, added `Arc::from()` mapping in proptest strategy
- **Files modified:** 10 additional files (see list above)
- **Verification:** All 615 tests pass, cargo check succeeds
- **Committed in:** bed5168 (part of task commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Auto-fix was necessary because changing core type definitions causes cascading compilation errors in all consumers. No scope creep -- all changes are mechanical type updates.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Core type migration complete, ready for Plan 03 (AnimatedIcon invariants)
- Connector crates (gpui, iced) will need matching updates when their pre-existing const block issues are resolved

## Self-Check: PASSED

All 13 modified files verified present. Commit bed5168 verified in git log.

---
*Phase: 87-font-family-arc-str-and-animatedicon-invariants*
*Completed: 2026-04-13*
