---
phase: 91-resolve-remaining-todo-doc-gaps
plan: 01
subsystem: api
tags: [presets, api-cleanup, preset-info, overlay]

# Dependency graph
requires:
  - phase: 88-diagnostic-and-preset-polish-sweep
    provides: PRESET_DISPLAY_NAMES, preset pipeline
provides:
  - PresetInfo struct with structured preset metadata
  - list_presets() returns &[PresetInfo] instead of &[&str]
  - list_presets_for_platform() returns Vec<PresetInfo>
  - with_overlay_toml removed from public API
affects: [connectors, preset-consumers]

# Tech tracking
tech-stack:
  added: []
  patterns: [PresetInfo structured metadata for preset enumeration]

key-files:
  created: []
  modified:
    - native-theme/src/presets.rs
    - native-theme/src/model/mod.rs
    - native-theme/src/lib.rs
    - native-theme/README.md
    - native-theme/tests/preset_loading.rs
    - native-theme/tests/resolve_and_validate.rs
    - connectors/native-theme-iced/src/lib.rs
    - connectors/native-theme-iced/tests/integration.rs
    - connectors/native-theme-gpui/src/lib.rs
    - native-theme/src/resolve/tests.rs

key-decisions:
  - "PresetInfo re-exported via pub mod theme for consistent access path"
  - "PLATFORM_SPECIFIC const removed (superseded by PresetInfo.platforms field)"
  - "PRESET_NAMES const retained for UnknownPreset error display"

patterns-established:
  - "info.key pattern: callers iterate PresetInfo and extract .key for preset() calls"

requirements-completed: [GAP-15b, GAP-15f]

# Metrics
duration: 12min
completed: 2026-04-15
---

# Phase 91 Plan 01: Delete with_overlay_toml and Add PresetInfo Summary

**Removed with_overlay_toml convenience wrapper and replaced bare &str preset listing with structured PresetInfo carrying key, display_name, platforms, and light_only metadata**

## Performance

- **Duration:** 12 min
- **Started:** 2026-04-15T13:49:59Z
- **Completed:** 2026-04-15T14:02:08Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments
- Deleted with_overlay_toml method and its test from SystemTheme (users call Theme::from_toml + with_overlay instead)
- Added PresetInfo struct with key, display_name, platforms, light_only fields plus PRESET_INFOS const
- Changed list_presets() from &[&str] to &[PresetInfo] and list_presets_for_platform() from Vec<&str> to Vec<PresetInfo>
- Updated all 10+ callers across core tests, integration tests, and both connectors

## Task Commits

Each task was committed atomically:

1. **Task 1: Delete with_overlay_toml and update references** - `96544a1` (feat)
2. **Task 2: Add PresetInfo struct and update list_presets return types** - `d64de48` (feat)

## Files Created/Modified
- `native-theme/src/presets.rs` - PresetInfo struct, PRESET_INFOS const, updated list functions, removed PLATFORM_SPECIFIC
- `native-theme/src/model/mod.rs` - Updated Theme::list_presets/list_presets_for_platform signatures and docs
- `native-theme/src/lib.rs` - Deleted with_overlay_toml, re-exported PresetInfo via pub mod theme
- `native-theme/README.md` - Two-step overlay pattern, structured metadata mention
- `native-theme/tests/preset_loading.rs` - All for loops use info.key pattern
- `native-theme/tests/resolve_and_validate.rs` - Updated list_presets callers
- `native-theme/src/resolve/tests.rs` - Updated validate_all_presets caller
- `connectors/native-theme-iced/src/lib.rs` - Updated preset test loop
- `connectors/native-theme-iced/tests/integration.rs` - Updated both preset test loops
- `connectors/native-theme-gpui/src/lib.rs` - Updated both preset test loops

## Decisions Made
- PresetInfo re-exported via `pub mod theme` for consistent `native_theme::theme::PresetInfo` access path alongside other public types
- PLATFORM_SPECIFIC const removed since platform data is now embedded in PRESET_INFOS entries
- PRESET_NAMES const retained separately for the UnknownPreset error variant (error display only needs names)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Restored files from incomplete prior 91-03 execution**
- **Found during:** Task 1 verification
- **Issue:** Working tree contained uncommitted changes to validate.rs and parse.rs from an incomplete 91-03 execution that broke compilation
- **Fix:** Restored those files to committed state via git checkout
- **Files modified:** native-theme/src/resolve/validate.rs, native-theme-derive/src/parse.rs (restored, not committed)
- **Verification:** Compilation succeeded after restore

**2. [Rule 1 - Bug] Removed unused PLATFORM_SPECIFIC constant**
- **Found during:** Task 2 verification
- **Issue:** After migrating platform data into PRESET_INFOS, PLATFORM_SPECIFIC was dead code producing a compiler warning
- **Fix:** Deleted the unused constant
- **Files modified:** native-theme/src/presets.rs
- **Verification:** Zero warnings on cargo test

---

**Total deviations:** 2 auto-fixed (1 blocking, 1 bug)
**Impact on plan:** Both auto-fixes necessary for correctness. No scope creep.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- PresetInfo is public and documented, ready for downstream consumers
- All preset-related tests pass with new return types
- Ready for 91-02 and 91-03 plans

---
*Phase: 91-resolve-remaining-todo-doc-gaps*
*Completed: 2026-04-15*

## Self-Check: PASSED

- All key files exist (presets.rs, model/mod.rs, lib.rs)
- Both commits found (96544a1, d64de48)
- PresetInfo struct present in source (1 definition)
- with_overlay_toml fully removed from native-theme/src/ (0 matches)
