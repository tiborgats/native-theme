---
phase: 82-icon-api-rework
plan: 02
subsystem: icons
tags: [builder-api, icon-loader, api-consolidation, size-parameter]

# Dependency graph
requires:
  - phase: 82-01
    provides: "IconData::Svg Cow migration, IconRole::name() kebab-case identifiers"
provides:
  - "IconLoader builder struct with new/set/size/color/color_opt/theme/load/load_indicator"
  - "IconId enum (Role/Name/Custom) with From impls for IconRole, &str, &dyn IconProvider"
  - "13 standalone icon-loading functions demoted to private/_inner or deleted"
  - "Size parameter flows through to freedesktop (ICON-02: no hardcoded 24px)"
  - "Platform loaders demoted to pub(crate): freedesktop, sficons, winicons"
  - "All connector code migrated to IconLoader builder"
affects: [connectors, icon-loading, public-api]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "IconLoader builder pattern: new(impl Into<IconId>).set().size().color().theme().load()"
    - "load_custom_via_builder helper preserves ?Sized bound while delegating to IconLoader"

key-files:
  created: []
  modified:
    - native-theme/src/icons.rs
    - native-theme/src/lib.rs
    - native-theme/src/freedesktop.rs
    - native-theme/src/sficons.rs
    - native-theme/src/winicons.rs
    - connectors/native-theme-iced/src/icons.rs
    - connectors/native-theme-iced/examples/showcase-iced.rs
    - connectors/native-theme-gpui/src/icons.rs
    - connectors/native-theme-gpui/examples/showcase-gpui.rs

key-decisions:
  - "IconLoader defaults to system_icon_set() and size 24 -- callers only override what they need"
  - "Connector ?Sized functions use load_custom_via_builder helper instead of IconLoader::new() to avoid unsized-to-trait-object coercion"
  - "is_freedesktop_theme_available stays public (capability probe, not a loader)"
  - "CLI theme override in gpui showcase uses IconLoader::new(name).set(Freedesktop).theme(t) instead of direct load_freedesktop_icon_by_name"

patterns-established:
  - "IconLoader builder: new(id).set(set).size(px).color(rgb).theme(name).load()"
  - "load_custom_via_builder: inline provider method calls + IconLoader for system dispatch"

requirements-completed: [ICON-01, ICON-02]

# Metrics
duration: 10min
completed: 2026-04-13
---

# Phase 82 Plan 02: IconLoader Builder API and Size-24 Hardcode Removal Summary

**13 standalone icon-loading functions collapsed into IconLoader builder with configurable size parameter; all connectors migrated**

## Performance

- **Duration:** 10 min
- **Started:** 2026-04-13T14:49:00Z
- **Completed:** 2026-04-13T14:59:01Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments
- IconLoader builder struct replaces 13 standalone icon-loading functions with a single fluent API
- IconId enum (Role/Name/Custom) with From impls enables ergonomic `IconLoader::new(role)`, `IconLoader::new("name")`, `IconLoader::new(provider)` construction
- Size parameter flows through to freedesktop loader (ICON-02: no hardcoded 24px)
- Platform loaders demoted to pub(crate): load_freedesktop_icon, load_freedesktop_icon_by_name, load_sf_icon, load_sf_icon_by_name, load_windows_icon, load_windows_icon_by_name
- load_custom_icon deleted (logic inlined in IconLoader::load_custom)
- All connector crates (iced, gpui) and showcase examples migrated to builder API
- 5 new builder API tests + all existing tests migrated

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement IconLoader builder and IconId enum, demote old functions** - `f933fbc` (feat)
2. **Task 2: Migrate connector code to IconLoader builder API** - `97c3157` (feat)

## Files Created/Modified
- `native-theme/src/icons.rs` - IconLoader struct, IconId enum, From impls, _inner dispatch functions, migrated + new tests
- `native-theme/src/lib.rs` - Updated pub(crate) re-exports to IconLoader/IconId
- `native-theme/src/freedesktop.rs` - load_freedesktop_icon/load_freedesktop_icon_by_name demoted to pub(crate)
- `native-theme/src/sficons.rs` - load_sf_icon/load_sf_icon_by_name demoted to pub(crate)
- `native-theme/src/winicons.rs` - load_windows_icon/load_windows_icon_by_name demoted to pub(crate)
- `connectors/native-theme-iced/src/icons.rs` - load_custom_icon -> load_custom_via_builder helper using IconLoader
- `connectors/native-theme-iced/examples/showcase-iced.rs` - All icon calls migrated to IconLoader builder
- `connectors/native-theme-gpui/src/icons.rs` - load_custom_icon -> load_custom_via_builder helper using IconLoader
- `connectors/native-theme-gpui/examples/showcase-gpui.rs` - All icon calls migrated to IconLoader builder, load_freedesktop_icon_by_name replaced

## Decisions Made
- IconLoader defaults to `system_icon_set()` and size 24 -- callers only override what they need
- Connector functions with `?Sized` bounds use `load_custom_via_builder` helper that calls provider methods directly then delegates to `IconLoader::new(name)` for system lookup, avoiding the unsized-to-trait-object coercion issue
- `is_freedesktop_theme_available` stays public as a capability probe (not a loader)
- GPUI showcase CLI theme override uses `IconLoader::new(name).set(Freedesktop).theme(t)` instead of direct `load_freedesktop_icon_by_name` (which is now `pub(crate)`)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed collapsible_if clippy lint in IconLoader::load_custom**
- **Found during:** Task 2 verification (pre-release-check.sh)
- **Issue:** Nested `if let` in `load_custom` method triggered `clippy::collapsible_if` lint
- **Fix:** Collapsed to single `if let ... && let Some(data)` expression
- **Files modified:** native-theme/src/icons.rs
- **Verification:** clippy passes with -D warnings
- **Committed in:** 97c3157 (Task 2 commit)

**2. [Rule 3 - Blocking] Added load_custom_via_builder helper for ?Sized connector functions**
- **Found during:** Task 2 (connector compilation)
- **Issue:** Connector functions accept `&(impl IconProvider + ?Sized)` but `IconLoader::new()` requires `Into<IconId>` which needs `&dyn IconProvider` -- unsized types cannot be coerced to trait objects
- **Fix:** Created `load_custom_via_builder` helper that calls `provider.icon_name()`/`provider.icon_svg()` directly and uses `IconLoader::new(name)` for system lookup
- **Files modified:** connectors/native-theme-iced/src/icons.rs, connectors/native-theme-gpui/src/icons.rs
- **Verification:** Both connector crates compile with all examples
- **Committed in:** 97c3157 (Task 2 commit)

---

**Total deviations:** 2 auto-fixed (1 bug, 1 blocking)
**Impact on plan:** Both fixes necessary for compilation. No scope creep.

## Issues Encountered
- Pre-existing `build_gnome_spec_pure` dead_code warning in gnome/mod.rs causes clippy failures when checking connector crates with `-D warnings`; this is Phase 78 Plan 04 residual and out of scope for this plan

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- IconLoader builder API complete with full connector migration
- Phase 82 icon-api-rework is fully complete (Plans 01 and 02 done)
- ICON-01 (unified builder), ICON-02 (size parameter), ICON-03 (Cow), ICON-04 (name()), ICON-06 (IconProvider), ICON-07 (drift guard) all delivered
- All 6 ICON requirements for this phase are satisfied

---
*Phase: 82-icon-api-rework*
*Completed: 2026-04-13*

## Self-Check: PASSED
- All 9 modified files exist on disk
- Both task commits verified (f933fbc, 97c3157)
- SUMMARY.md created successfully
