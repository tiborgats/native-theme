---
phase: 88-diagnostic-and-preset-polish-sweep
plan: 01
subsystem: api
tags: [diagnostics, pipeline, structured-types, display-impl]

# Dependency graph
requires:
  - phase: 79-border-split-and-reader-audit
    provides: clean border target for pipeline types
provides:
  - DiagnosticEntry enum with name/status/detail accessors (ROADMAP SC-1)
  - PlatformPreset struct separating user-facing name from internal -live suffix
  - Display impls reproducing old string format for backward compatibility
affects: [88-02, connectors, showcase-gpui]

# Tech tracking
tech-stack:
  added: []
  patterns: ["structured return types with Display for migration ease", "live_name() method encapsulates -live suffix convention"]

key-files:
  created: []
  modified:
    - native-theme/src/pipeline.rs
    - native-theme/src/lib.rs
    - connectors/native-theme-gpui/examples/showcase-gpui.rs
    - .planning/ROADMAP.md

key-decisions:
  - "DiagnosticEntry::DesktopEnv variant gated with #[cfg(target_os = linux)] to avoid exposing LinuxDesktop on non-Linux"
  - "PlatformPreset.live_name() returns String not &str since it builds the -live suffix dynamically"
  - "ROADMAP SC-5 scoped to user-facing return values -- -live must exist in source for internal use"
  - "DiagnosticEntry feature labels use short names (KDE, Portal) for name() accessor consistency"

patterns-established:
  - "Structured return types with Display impl: new typed returns include Display that reproduces old string format for zero-churn migration"

requirements-completed: [POLISH-01, POLISH-02]

# Metrics
duration: 8min
completed: 2026-04-14
---

# Phase 88 Plan 01: DiagnosticEntry and PlatformPreset Summary

**Typed diagnostic and preset return types replacing raw strings, with Display impls for backward-compatible printing and name/status/detail accessors for programmatic inspection**

## Performance

- **Duration:** 8 min
- **Started:** 2026-04-14T00:05:08Z
- **Completed:** 2026-04-14T00:13:31Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- `diagnose_platform_support()` now returns `Vec<DiagnosticEntry>` with 11 typed variants covering platform, env vars, tools, configs, and features
- `platform_preset_name()` now returns `PlatformPreset { name, is_live }` -- the `-live` suffix no longer leaks into user-facing strings
- DiagnosticEntry accessors `name()`, `status()`, `detail()` satisfy ROADMAP SC-1 field contract
- Showcase callers use `preset.name` directly, eliminating the `strip_suffix("-live")` workaround

## Task Commits

Each task was committed atomically:

1. **Task 1: Define DiagnosticEntry and PlatformPreset types, rewrite both functions** - `24abe71` (feat)
2. **Task 2: Update showcase callers and fix ROADMAP SC-5 scope** - `4eaabaf` (feat)

## Files Created/Modified
- `native-theme/src/pipeline.rs` - DiagnosticEntry enum, PlatformPreset struct, rewritten functions, updated internal callers
- `native-theme/src/lib.rs` - Added pub re-exports for DiagnosticEntry and PlatformPreset, updated tests
- `connectors/native-theme-gpui/examples/showcase-gpui.rs` - Replaced strip_suffix workaround with preset.name
- `.planning/ROADMAP.md` - SC-5 scoped to user-facing return values

## Decisions Made
- DiagnosticEntry::DesktopEnv variant is `#[cfg(target_os = "linux")]` gated since LinuxDesktop only exists on Linux
- PlatformPreset.live_name() returns String (not &str) because it dynamically appends "-live"
- ROADMAP SC-5 updated: the old grep-based criterion was impossible since "-live" must exist in source code for internal use; scoped to user-facing return values
- Diagnostic feature labels use short names ("KDE", "Portal") rather than full descriptions for consistent name() output

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Added doc comments to DiagnosticEntry struct-like variant fields**
- **Found during:** Task 1 (clippy check)
- **Issue:** `#[deny(missing_docs)]` lint failed on named fields in enum variants (12 undocumented fields)
- **Fix:** Added doc comments to all named fields in EnvVar, ToolAvailable, ToolMissing, ConfigFound, ConfigMissing, FeatureDisabled variants
- **Files modified:** native-theme/src/pipeline.rs
- **Verification:** `cargo clippy -p native-theme --all-targets -- -D warnings` passes clean
- **Committed in:** 24abe71 (Task 1 commit)

**2. [Rule 1 - Bug] Converted dispatch_tests to return Result for zero-panic compliance**
- **Found during:** Task 1 (test updates)
- **Issue:** Pre-commit hook blocks .unwrap()/.expect() even in test code; tests using linux_preset_for_de needed updating
- **Fix:** Changed 3 test functions to return `crate::Result<()>` with `?` operator
- **Files modified:** native-theme/src/pipeline.rs
- **Verification:** All 28 pipeline tests pass
- **Committed in:** 24abe71 (Task 1 commit)

---

**Total deviations:** 2 auto-fixed (2 bug fixes)
**Impact on plan:** Both auto-fixes necessary for lint compliance and hook compliance. No scope creep.

## Issues Encountered
- Pre-existing `build_gnome_spec_pure` dead_code warning in gnome/mod.rs -- out of scope, not caused by this plan's changes

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- DiagnosticEntry and PlatformPreset types are stable and re-exported
- Plan 02 (Cow migration for preset name/icon_theme) can proceed independently
- Showcase and all connectors compile cleanly

---
*Phase: 88-diagnostic-and-preset-polish-sweep*
*Completed: 2026-04-14*
