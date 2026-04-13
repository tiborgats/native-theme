---
phase: 83-detection-cache-layer
plan: 01
subsystem: detect
tags: [api-rename, detect, linux-desktop, xdg]

# Dependency graph
requires: []
provides:
  - "detect_linux_desktop() no-arg convenience replacing detect_linux_de(&str)"
  - "parse_linux_desktop(&str) pub pure parser for testable string parsing"
  - "All callers migrated including native-theme-build codegen and connector examples"
affects: [83-02, detect, pipeline, watch, presets, resolve, icons, codegen]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "No-arg convenience wrapping pub(crate) env reader + pub pure parser"

key-files:
  created: []
  modified:
    - native-theme/src/detect.rs
    - native-theme/src/pipeline.rs
    - native-theme/src/lib.rs
    - native-theme/src/presets.rs
    - native-theme/src/watch/mod.rs
    - native-theme/src/resolve/inheritance.rs
    - native-theme/src/model/icons.rs
    - native-theme-build/src/codegen.rs
    - native-theme-build/src/schema.rs
    - native-theme-build/src/lib.rs
    - native-theme-build/tests/integration.rs
    - connectors/native-theme-gpui/examples/showcase-gpui.rs

key-decisions:
  - "parse_linux_desktop kept pub (not pub(crate)) because native-theme-build tests need external access"

patterns-established:
  - "No-arg detect_linux_desktop() reads env internally; parse_linux_desktop(&str) for pure testable parsing"

requirements-completed: [DETECT-02]

# Metrics
duration: 6min
completed: 2026-04-13
---

# Phase 83 Plan 01: Detect Linux Desktop Rename Summary

**Renamed detect_linux_de to detect_linux_desktop (no-arg) + parse_linux_desktop (pure parser), migrating all 12 files across 3 crates**

## Performance

- **Duration:** 6 min
- **Started:** 2026-04-13T16:36:50Z
- **Completed:** 2026-04-13T16:43:09Z
- **Tasks:** 2
- **Files modified:** 12

## Accomplishments
- Renamed `detect_linux_de(&str)` to `parse_linux_desktop(&str)` as the pure testable parser
- Added `detect_linux_desktop()` no-arg convenience that reads `XDG_CURRENT_DESKTOP` internally
- Eliminated the two-call idiom `detect_linux_de(&xdg_current_desktop())` at all 7 call sites
- Migrated native-theme-build codegen to emit `parse_linux_desktop` in generated code
- Updated connector showcase example
- Fixed 4 stale OnceLock references in comments

## Task Commits

Each task was committed atomically:

1. **Task 1: Rename detect_linux_de and add no-arg detect_linux_desktop** - `9ebf712` (feat)
2. **Task 2: Migrate all callers and update re-exports** - `8350476` (feat)

## Files Created/Modified
- `native-theme/src/detect.rs` - New detect_linux_desktop() no-arg, renamed parse_linux_desktop(), fixed stale comments
- `native-theme/src/pipeline.rs` - Migrated platform_preset_name, from_system_inner, diagnose_platform_support, and all tests
- `native-theme/src/lib.rs` - Updated re-export and test references
- `native-theme/src/presets.rs` - Migrated detect_platform() to use detect_linux_desktop()
- `native-theme/src/watch/mod.rs` - Migrated on_theme_change() dispatch
- `native-theme/src/resolve/inheritance.rs` - Migrated platform_button_order()
- `native-theme/src/model/icons.rs` - Migrated detect_linux_icon_theme()
- `native-theme-build/src/codegen.rs` - Updated generated code to emit parse_linux_desktop
- `native-theme-build/src/schema.rs` - Updated DE table drift test
- `native-theme-build/src/lib.rs` - Updated DE-aware collapse test assertion
- `native-theme-build/tests/integration.rs` - Updated codegen output assertion
- `connectors/native-theme-gpui/examples/showcase-gpui.rs` - Migrated to parse_linux_desktop

## Decisions Made
- Kept `parse_linux_desktop` as `pub` (not `pub(crate)` as plan specified) because `native-theme-build/src/schema.rs` tests import it from an external crate. This is a necessary visibility level.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Migrated native-theme-build codegen and tests**
- **Found during:** Task 2 (pre-release-check.sh revealed compile failures)
- **Issue:** native-theme-build/src/codegen.rs generates code calling detect_linux_de; schema.rs and lib.rs tests also reference the old name
- **Fix:** Updated codegen.rs template string, schema.rs test import/call, lib.rs test assertion, integration.rs test assertion
- **Files modified:** native-theme-build/src/codegen.rs, schema.rs, lib.rs, tests/integration.rs
- **Verification:** `cargo check -p native-theme-build --all-targets` and `cargo test -p native-theme-build` both pass
- **Committed in:** 8350476 (Task 2 commit)

**2. [Rule 3 - Blocking] Migrated connector example**
- **Found during:** Task 2 (grep revealed additional reference in showcase-gpui.rs)
- **Issue:** connectors/native-theme-gpui/examples/showcase-gpui.rs imported and called detect_linux_de
- **Fix:** Updated import and call site to use parse_linux_desktop
- **Files modified:** connectors/native-theme-gpui/examples/showcase-gpui.rs
- **Committed in:** 8350476 (Task 2 commit)

**3. [Rule 3 - Blocking] parse_linux_desktop visibility kept as pub**
- **Found during:** Task 2 (native-theme-build failed to compile with pub(crate))
- **Issue:** Plan specified pub(crate) for parse_linux_desktop, but native-theme-build (external crate) tests import it
- **Fix:** Kept parse_linux_desktop as pub
- **Committed in:** 8350476 (Task 2 commit)

---

**Total deviations:** 3 auto-fixed (3 blocking)
**Impact on plan:** All auto-fixes necessary for cross-crate compilation. No scope creep.

## Issues Encountered
None beyond the deviations documented above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- detect_linux_desktop() is the standard no-arg API for DE detection
- parse_linux_desktop(&str) is the pure parser for tests and string-based calls
- Ready for Plan 02 (detection cache layer)

---
*Phase: 83-detection-cache-layer*
*Completed: 2026-04-13*
