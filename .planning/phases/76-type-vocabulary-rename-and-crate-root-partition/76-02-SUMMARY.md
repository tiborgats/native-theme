---
phase: 76-type-vocabulary-rename-and-crate-root-partition
plan: 02
subsystem: api
tags: [module-structure, crate-root, prelude, LAYOUT-01]

# Dependency graph
requires:
  - phase: 76-type-vocabulary-rename-and-crate-root-partition
    plan: 01
    provides: "Renamed types (Theme, ThemeMode, ResolvedTheme, ResolvedDefaults)"
provides:
  - "native_theme::theme:: module facade re-exporting all model types"
  - "native_theme::detect:: public module for detection functions and LinuxDesktop"
  - "native_theme::icons:: public module for icon loading functions"
  - "native_theme::pipeline:: public module for platform_preset_name and diagnose_platform_support"
  - "native_theme::prelude:: with exactly 6 items (Theme, ResolvedTheme, SystemTheme, Rgba, Error, Result)"
  - "tests/prelude_smoke.rs asserting the prelude set"
affects: [77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88]

# Tech tracking
tech-stack:
  added: []
  patterns: ["pub mod theme { pub use crate::model::*; } facade for clean public API", "pub(crate) use for internal backward compatibility while hiding flat re-exports from public API"]

key-files:
  created:
    - native-theme/src/prelude.rs
    - native-theme/tests/prelude_smoke.rs
  modified:
    - native-theme/src/lib.rs
    - native-theme/src/icons.rs
    - native-theme/src/detect.rs
    - connectors/native-theme-gpui/src/lib.rs
    - connectors/native-theme-gpui/src/colors.rs
    - connectors/native-theme-gpui/src/config.rs
    - connectors/native-theme-gpui/src/icons.rs
    - connectors/native-theme-gpui/examples/showcase-gpui.rs
    - connectors/native-theme-iced/src/lib.rs
    - connectors/native-theme-iced/src/extended.rs
    - connectors/native-theme-iced/src/palette.rs
    - connectors/native-theme-iced/src/icons.rs
    - connectors/native-theme-iced/tests/integration.rs
    - connectors/native-theme-iced/examples/showcase-iced.rs
    - native-theme-build/src/codegen.rs
    - native-theme-build/src/schema.rs
    - native-theme-build/src/lib.rs
    - native-theme-build/tests/integration.rs
    - native-theme/README.md
    - native-theme/tests/merge_behavior.rs
    - native-theme/tests/platform_facts_xref.rs
    - native-theme/tests/preset_loading.rs
    - native-theme/tests/serde_roundtrip.rs
    - native-theme/tests/proptest_roundtrip.rs
    - native-theme/tests/resolve_and_validate.rs
    - native-theme/tests/reader_kde.rs

key-decisions:
  - "pub(crate) use re-exports preserve internal crate::Type paths without touching 30+ internal source files"
  - "pub mod theme { pub use crate::model::*; } inline facade avoids creating a new file while providing clean public module"
  - "native-theme-build codegen emits module-qualified paths (theme::, detect::) in generated code"

patterns-established:
  - "Module-qualified public API: native_theme::theme::Type, native_theme::detect::fn, native_theme::icons::fn"
  - "Prelude pattern: exactly 6 items for quick-start usage"

requirements-completed: [LAYOUT-01]

# Metrics
duration: 48min
completed: 2026-04-12
---

# Phase 76 Plan 02: Crate Root Partition Summary

**Replaced ~92 flat pub use re-exports with module-qualified public API (theme::, detect::, icons::, pipeline::) plus a 6-item prelude, updating all connectors, native-theme-build codegen, and 33 files across the workspace**

## Performance

- **Duration:** 48 min
- **Started:** 2026-04-12T17:19:01Z
- **Completed:** 2026-04-12T18:07:16Z
- **Tasks:** 2/2
- **Files modified:** 36

## Accomplishments
- Partitioned the 92-item flat crate root into a clean module hierarchy: `theme::`, `detect::`, `icons::`, `pipeline::`, `watch::`, `prelude::`
- lib.rs now contains only `pub mod` declarations, the `SystemTheme` struct, `Result` type alias, and internal test modules
- Created `prelude.rs` with exactly 6 items (Theme, ResolvedTheme, SystemTheme, Rgba, Error, Result) verified by `prelude_smoke.rs`
- Updated both connector crates (gpui, iced), all examples, all 7 integration tests, and the native-theme-build codegen to use module-qualified import paths
- 710 tests pass across the core crate, all connector tests pass, all build crate tests pass

## Task Commits

Each task was committed atomically:

1. **Task 1: Partition lib.rs re-exports into submodule structure** - `c5f4aec` (refactor)
2. **Task 2: Create prelude module and prelude_smoke.rs test** - `d3f2714` (feat)

## Files Created/Modified

### Created
- `native-theme/src/prelude.rs` - Prelude module with 6 re-exports
- `native-theme/tests/prelude_smoke.rs` - Smoke test asserting prelude contents

### Core crate
- `native-theme/src/lib.rs` - Replaced flat re-exports with pub mod structure + pub(crate) internal re-exports
- `native-theme/src/icons.rs` - Updated doc examples to module-qualified paths
- `native-theme/src/detect.rs` - Updated doc example path
- `native-theme/src/model/mod.rs` - Updated doc example imports
- `native-theme/src/model/icons.rs` - Updated doc example imports
- `native-theme/src/model/animated.rs` - Updated doc example imports
- `native-theme/src/model/bundled.rs` - Updated doc example imports
- `native-theme/src/pipeline.rs` - Updated doc example path
- `native-theme/src/resolve/mod.rs` - Updated doc example import
- `native-theme/src/color.rs` - Updated doc example import
- `native-theme/README.md` - Updated all code examples to module-qualified paths

### Integration tests
- `native-theme/tests/merge_behavior.rs` - use native_theme::theme::* + color::Rgba
- `native-theme/tests/platform_facts_xref.rs` - use native_theme::theme::* + color::Rgba
- `native-theme/tests/preset_loading.rs` - use native_theme::theme::Theme
- `native-theme/tests/serde_roundtrip.rs` - use native_theme::theme::* + color::Rgba
- `native-theme/tests/proptest_roundtrip.rs` - use native_theme::theme::* + color::Rgba
- `native-theme/tests/resolve_and_validate.rs` - use native_theme::theme::* + color::Rgba + error::Error
- `native-theme/tests/reader_kde.rs` - use native_theme::theme::DialogButtonOrder + color::Rgba

### Connector crates
- `connectors/native-theme-gpui/src/lib.rs` - Module-qualified pub use re-exports
- `connectors/native-theme-gpui/src/colors.rs` - native_theme::theme::ResolvedTheme, color::Rgba
- `connectors/native-theme-gpui/src/config.rs` - native_theme::theme::ResolvedTheme, Theme
- `connectors/native-theme-gpui/src/icons.rs` - Module-qualified imports throughout
- `connectors/native-theme-gpui/examples/showcase-gpui.rs` - Module-qualified imports throughout
- `connectors/native-theme-iced/src/lib.rs` - Module-qualified pub use re-exports
- `connectors/native-theme-iced/src/extended.rs` - native_theme::color::Rgba, theme::*
- `connectors/native-theme-iced/src/palette.rs` - native_theme::color::Rgba, theme::*
- `connectors/native-theme-iced/src/icons.rs` - Module-qualified imports
- `connectors/native-theme-iced/tests/integration.rs` - native_theme::theme::Theme
- `connectors/native-theme-iced/examples/showcase-iced.rs` - Module-qualified imports throughout

### Build crate
- `native-theme-build/src/codegen.rs` - Generated code emits theme::, detect:: paths
- `native-theme-build/src/schema.rs` - Updated drift detection imports
- `native-theme-build/src/lib.rs` - Updated test assertion for custom crate path
- `native-theme-build/tests/integration.rs` - Updated generated code assertions

## Decisions Made
- **pub(crate) use pattern:** Used `pub(crate) use` re-exports in lib.rs to maintain backward compatibility for internal `crate::Type` paths across 30+ internal source files without rewriting them all, while removing the public flat re-exports
- **Inline theme facade:** Created `pub mod theme { pub use crate::model::*; }` as an inline module instead of a separate file, since it's a single re-export line
- **native-theme-build codegen update:** Updated code generation templates and qualified path functions to emit `{crate_path}::theme::IconSet`, `{crate_path}::detect::LinuxDesktop`, `{crate_path}::detect::detect_linux_de` instead of flat paths

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated native-theme-build codegen and tests**
- **Found during:** Task 1 (pre-release check)
- **Issue:** native-theme-build crate generates code using flat `native_theme::IconSet`, `native_theme::IconProvider`, `native_theme::LinuxDesktop`, `native_theme::detect_linux_de` paths that no longer exist
- **Fix:** Updated codegen.rs templates, qualified path functions, and all test assertions; updated schema.rs drift detection imports; updated lib.rs and integration test assertions
- **Files modified:** native-theme-build/src/codegen.rs, schema.rs, lib.rs, tests/integration.rs
- **Verification:** All 208 native-theme-build tests pass
- **Committed in:** c5f4aec (Task 1 commit)

**2. [Rule 3 - Blocking] Updated doc examples across model, pipeline, resolve, and color modules**
- **Found during:** Task 1 (doctest failures)
- **Issue:** Doc examples in model/mod.rs, model/icons.rs, model/animated.rs, model/bundled.rs, pipeline.rs, resolve/mod.rs, and color.rs used flat `native_theme::Theme`, `native_theme::Rgba` etc. which are now private
- **Fix:** Updated all doc examples to use module-qualified paths (native_theme::theme::Theme, native_theme::color::Rgba, etc.)
- **Files modified:** 7 files in native-theme/src/
- **Verification:** All 38 doctests pass
- **Committed in:** c5f4aec (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Both auto-fixes necessary for compilation. The native-theme-build update was out of scope per the plan's files_modified list but was required for workspace-wide compilation. No scope creep beyond what was needed.

## Issues Encountered
- Pre-existing `gnome::tests::build_gnome_variant_normal_contrast_no_flag` test failure (system-dependent, reads gsettings on developer machine) -- confirmed pre-existing, not introduced by this plan
- `cargo test --workspace --all-features` fails due to upstream ashpd async-io/tokio feature conflict -- tested each crate individually

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Module structure is clean: theme::, detect::, icons::, pipeline::, watch::, prelude::
- Prelude provides quick-start with 6 items
- All downstream phases can reference types via module-qualified paths
- Phase 76 complete -- ready for Phase 77 (pick(ColorMode) + icon_set relocation)

---
*Phase: 76-type-vocabulary-rename-and-crate-root-partition*
*Completed: 2026-04-12*
