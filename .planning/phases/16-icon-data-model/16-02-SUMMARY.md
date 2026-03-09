---
phase: 16-icon-data-model
plan: 02
subsystem: model
tags: [icons, icon-name, icon-theme, system-icon-set, mapping, presets, toml]

# Dependency graph
requires:
  - "16-01: IconRole, IconData, IconSet enum definitions"
provides:
  - "icon_name() mapping function with 210 match arms (5 sets x 42 roles)"
  - "system_icon_set() runtime OS detection returning IconSet"
  - "ThemeVariant.icon_theme field with serde, merge, is_empty support"
  - "6 native preset TOMLs with icon_theme assignments"
affects: [17-bundled-svgs, 18-platform-loaders, 20-connectors, 21-icon-integration]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Private dispatch functions per icon set (sf_symbols_name, segoe_name, etc.)"
    - "cfg!() runtime detection instead of #[cfg()] compile-time gating for system_icon_set"
    - "#[allow(unreachable_patterns)] for #[non_exhaustive] forward compatibility"

key-files:
  created: []
  modified:
    - "native-theme/src/model/icons.rs"
    - "native-theme/src/model/mod.rs"
    - "native-theme/src/lib.rs"
    - "native-theme/src/presets/windows-11.toml"
    - "native-theme/src/presets/macos-sonoma.toml"
    - "native-theme/src/presets/ios.toml"
    - "native-theme/src/presets/adwaita.toml"
    - "native-theme/src/presets/kde-breeze.toml"
    - "native-theme/src/presets/material.toml"
    - "native-theme/src/macos.rs"
    - "native-theme/src/windows.rs"
    - "native-theme/src/kde/mod.rs"

key-decisions:
  - "Combined macOS + iOS cfg!() branches to avoid clippy if_same_then_else warning"
  - "Used #[allow(unreachable_patterns)] on mapping functions for non_exhaustive forward compat"
  - "icon_theme set on both light and dark variants in native presets (same value)"

patterns-established:
  - "Private fn per icon set: sf_symbols_name(), segoe_name(), freedesktop_name(), material_name(), lucide_name()"
  - "TOML [light] section header before dotted sub-tables for bare fields like icon_theme"

requirements-completed: [ICON-03, ICON-04, ICON-05]

# Metrics
duration: 6min
completed: 2026-03-09
---

# Phase 16 Plan 02: Icon Name Mapping Summary

**icon_name() with 210 match arms mapping 42 IconRoles across 5 icon sets, system_icon_set() OS detection, and ThemeVariant icon_theme field with 6 preset TOML assignments**

## Performance

- **Duration:** 6 min
- **Started:** 2026-03-09T06:42:29Z
- **Completed:** 2026-03-09T06:48:02Z
- **Tasks:** 2 (4 TDD commits)
- **Files modified:** 12

## Accomplishments
- Implemented icon_name() mapping function dispatching to 5 private per-set functions with exact identifiers from docs/native-icons.md
- Implemented system_icon_set() using cfg!() macros for runtime OS detection (macOS/iOS -> SfSymbols, Windows -> SegoeIcons, Linux -> Freedesktop, other -> Material)
- Added icon_theme: Option<String> to ThemeVariant with serde, merge, and is_empty support
- Updated 6 native preset TOMLs with correct icon_theme values; 11 community/default presets correctly deserialize to None
- Re-exported icon_name and system_icon_set at crate root

## Task Commits

Each task was committed atomically (TDD RED+GREEN):

1. **Task 1 RED: Failing tests for icon_name/system_icon_set** - `3c9c7c3` (test)
2. **Task 1 GREEN: Implement icon_name() and system_icon_set()** - `1bdb81b` (feat)
3. **Task 2 RED: Failing tests for icon_theme** - `13d62db` (test)
4. **Task 2 GREEN: Add icon_theme to ThemeVariant + preset TOMLs** - `73c9356` (feat)

## Files Created/Modified
- `native-theme/src/model/icons.rs` - Added icon_name(), system_icon_set(), and 5 private mapping functions with 210 match arms + 31 new tests
- `native-theme/src/model/mod.rs` - Added icon_theme field to ThemeVariant, updated merge() and is_empty() + 6 new tests
- `native-theme/src/lib.rs` - Re-exported icon_name and system_icon_set as free functions
- `native-theme/src/presets/windows-11.toml` - Added icon_theme = "segoe-fluent" to light and dark
- `native-theme/src/presets/macos-sonoma.toml` - Added icon_theme = "sf-symbols" to light and dark
- `native-theme/src/presets/ios.toml` - Added icon_theme = "sf-symbols" to light and dark
- `native-theme/src/presets/adwaita.toml` - Added icon_theme = "freedesktop" to light and dark
- `native-theme/src/presets/kde-breeze.toml` - Added icon_theme = "freedesktop" to light and dark
- `native-theme/src/presets/material.toml` - Added icon_theme = "material" to light and dark
- `native-theme/src/presets.rs` - Added 2 new preset icon_theme tests (native + community)
- `native-theme/src/macos.rs` - Fixed ThemeVariant struct literal (added icon_theme: None)
- `native-theme/src/windows.rs` - Fixed ThemeVariant struct literal (added icon_theme: None)
- `native-theme/src/kde/mod.rs` - Fixed ThemeVariant struct literal (added icon_theme: None)

## Decisions Made
- Combined macOS and iOS cfg!() branches into single `cfg!(any(target_os = "macos", target_os = "ios"))` to avoid clippy if_same_then_else warning
- Used `#[allow(unreachable_patterns)]` on all 6 match functions to suppress warnings for wildcard arms needed for #[non_exhaustive] forward compatibility
- Set icon_theme on both light and dark variants in native presets (same value for both)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed clippy if_same_then_else warning in system_icon_set()**
- **Found during:** Task 1 (GREEN phase)
- **Issue:** Separate `if cfg!(target_os = "macos")` and `if cfg!(target_os = "ios")` branches both returned `IconSet::SfSymbols`, triggering clippy error
- **Fix:** Combined into `if cfg!(any(target_os = "macos", target_os = "ios"))`
- **Files modified:** native-theme/src/model/icons.rs
- **Committed in:** 1bdb81b

**2. [Rule 3 - Blocking] Fixed ThemeVariant struct literals in platform modules**
- **Found during:** Task 2 (GREEN phase)
- **Issue:** macos.rs, windows.rs, and kde/mod.rs constructed ThemeVariant with struct literals missing the new icon_theme field
- **Fix:** Added `icon_theme: None` to all 4 struct literal constructors
- **Files modified:** native-theme/src/macos.rs, native-theme/src/windows.rs, native-theme/src/kde/mod.rs
- **Committed in:** 73c9356

---

**Total deviations:** 2 auto-fixed (1 bug, 1 blocking)
**Impact on plan:** Both fixes necessary for compilation. No scope creep.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 16 (Icon Data Model) is fully complete
- icon_name() and system_icon_set() are available at crate root for all downstream phases
- ThemeVariant.icon_theme enables preset-driven icon set selection
- Ready for Phase 17 (Bundled SVGs) to embed Material/Lucide SVG assets

## Self-Check: PASSED

All 13 files exist, all 4 commits verified.

---
*Phase: 16-icon-data-model*
*Completed: 2026-03-09*
