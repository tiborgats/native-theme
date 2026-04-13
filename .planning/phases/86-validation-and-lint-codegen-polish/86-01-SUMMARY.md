---
phase: 86-validation-and-lint-codegen-polish
plan: 01
subsystem: validation
tags: [inventory, lint, toml, codegen, derive-macro]

requires:
  - phase: 80-native-theme-derive-proc-macro
    provides: "ThemeWidget derive macro with inventory::submit! for WidgetFieldInfo"
provides:
  - "lint_toml driven by inventory registry instead of hand-maintained string arrays"
  - "Zero-maintenance widget discovery: new #[derive(ThemeWidget)] widgets auto-recognized by linter"
affects: [86-02, validation, lint]

tech-stack:
  added: []
  patterns:
    - "inventory::iter::<WidgetFieldInfo>() for runtime widget discovery in lint_toml"
    - "HashMap<&str, &[&str]> widget registry built once per lint_toml call"

key-files:
  created: []
  modified:
    - native-theme/src/model/mod.rs
    - native-theme/src/resolve/mod.rs

key-decisions:
  - "Pass widget_registry as parameter to lint_variant (not closure capture) for clarity"
  - "STRUCTURAL_KEYS const retains only 'defaults' and 'text_scale' -- all widget names come from inventory"

patterns-established:
  - "Inventory-driven lint: widget names and field names discovered at runtime from derive macro submissions"

requirements-completed: [VALID-03]

duration: 5min
completed: 2026-04-13
---

# Phase 86 Plan 01: Inventory-Driven lint_toml Summary

**lint_toml rewritten to discover widget names and field names from inventory::iter::\<WidgetFieldInfo\>(), eliminating 50+ hand-maintained string literals**

## Performance

- **Duration:** 5 min
- **Started:** 2026-04-13T21:35:35Z
- **Completed:** 2026-04-13T21:41:31Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Replaced 27-entry VARIANT_KEYS const and 25-arm widget_fields() match with inventory-driven HashMap lookup
- Removed #[expect(dead_code)] from WidgetFieldInfo (now actively consumed by lint_toml)
- Added 2 new tests confirming auto-discovery: unknown field rejection and all-registered-widgets recognition
- All 15 lint_toml tests pass (13 existing + 2 new), clippy clean

## Task Commits

Each task was committed atomically:

1. **Task 1: Replace VARIANT_KEYS and widget_fields() with inventory-driven lookup** - `0a95c9f` (feat)
2. **Task 2: Add test verifying inventory auto-discovery** - `7ce1bfd` (test)

## Files Created/Modified
- `native-theme/src/model/mod.rs` - lint_toml rewritten: STRUCTURAL_KEYS const, widget_registry HashMap from inventory, lint_variant accepts registry param; 2 new tests added
- `native-theme/src/resolve/mod.rs` - Removed #[expect(dead_code)] from WidgetFieldInfo struct

## Decisions Made
- Pass widget_registry HashMap as parameter to lint_variant fn (rather than converting to closure) -- simpler, lint_variant is called twice (light/dark)
- STRUCTURAL_KEYS contains only ["defaults", "text_scale"] -- all 25 widget names now come exclusively from inventory

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- Pre-existing stale proc-macro cache caused compilation errors on first build attempt. The derive macro's gen_ranges.rs and validate_helpers.rs had been updated in prior commits (e813e92, 5d63fee) but the compiled macro artifact was cached. Resolved with `cargo clean -p native-theme-derive`.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- VALID-03 satisfied: lint_toml now auto-discovers widgets from inventory
- Ready for 86-02 (VALID-04) which addresses further codegen polish
- Pre-existing connector issues (native-theme-gpui private function, gnome dead_code) are out of scope for this plan

---
*Phase: 86-validation-and-lint-codegen-polish*
*Completed: 2026-04-13*
