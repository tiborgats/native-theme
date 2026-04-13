---
phase: 80-native-theme-derive-proc-macro-k-codegen
plan: 02
subsystem: codegen
tags: [proc-macro, derive, syn, quote, widget-theme, inheritance, inventory, code-generation]

# Dependency graph
requires:
  - phase: 80-native-theme-derive-proc-macro-k-codegen
    provides: ThemeWidget derive macro with parsing, struct gen, validate, check_ranges (Plan 01)
  - phase: 79-borderspec-split-and-platform-reader-visibility-audit
    provides: Clean WidgetBorderSpec/DefaultsBorderSpec split for border validation dispatch
provides:
  - All 25 per-variant widgets migrated from define_widget_pair! to #[derive(ThemeWidget)]
  - 67 uniform inheritance rules as #[theme(inherit_from)] attributes
  - Generated resolve_from_defaults() replacing hand-written color inheritance
  - inventory::submit! widget registry for TOML linting
  - 748 net lines of boilerplate eliminated
affects: [Phase 81 feature-matrix cleanup, Phase 86 validation codegen polish]

# Tech tracking
tech-stack:
  added: [inventory 0.3]
  patterns: [attribute-driven inheritance codegen, inventory widget registry, skip_inventory for non-per-variant widgets]

key-files:
  created:
    - native-theme-derive/src/gen_inherit.rs
  modified:
    - native-theme/src/model/widgets/mod.rs
    - native-theme/src/resolve/inheritance.rs
    - native-theme/src/resolve/mod.rs
    - native-theme-derive/src/lib.rs
    - native-theme-derive/src/parse.rs
    - native-theme-derive/src/gen_ranges.rs
    - native-theme-derive/src/gen_validate.rs
    - native-theme/Cargo.toml

key-decisions:
  - "67 inherit_from attributes (exceeds ~55 estimate) -- all uniform rules migrated to attributes"
  - "safety nets stay hand-written in inheritance.rs (6 rules with platform-specific fallbacks)"
  - "widget-to-widget chains stay hand-written (7 rules depend on resolved font/border)"
  - "LayoutTheme and test widgets use #[theme_layer(skip_inventory)] since they are not per-variant"
  - "border_kind set per struct: menu/tab/card=none, sidebar/status_bar=partial, all others=full(default)"

patterns-established:
  - "inherit_from attribute: #[theme(inherit_from = 'defaults.X')] generates resolve_from_defaults()"
  - "skip_inventory: #[theme_layer(skip_inventory)] prevents inventory::submit! for non-widget structs"
  - "resolve_color_inheritance() calls per-widget resolve_from_defaults() instead of inline rules"

requirements-completed: [MODEL-01, VALID-01, VALID-02, BORDER-02]

# Metrics
duration: 10min
completed: 2026-04-13
---

# Phase 80 Plan 02: Complete widget migration to #[derive(ThemeWidget)] with attribute-driven inheritance

**All 25 widgets migrated from define_widget_pair! to derive, 67 inheritance rules as attributes, 748 lines of boilerplate eliminated, zero hand-written check_ranges remain**

## Performance

- **Duration:** 10 min
- **Started:** 2026-04-13T09:25:19Z
- **Completed:** 2026-04-13T09:35:43Z
- **Tasks:** 2
- **Files modified:** 10

## Accomplishments
- Migrated all 25 per-variant widgets + LayoutTheme + 2 test widgets from define_widget_pair! to #[derive(ThemeWidget)]
- Deleted define_widget_pair! macro, __field_name! helper, and all 24 hand-written check_ranges() impl blocks
- Added 67 #[theme(inherit_from)] attributes replacing inline color inheritance rules
- Generated resolve_from_defaults() called from inheritance.rs, replacing ~250 lines of hand-written rules
- Created gen_inherit.rs module for inheritance codegen
- Added inventory::submit! registration for all 25 per-variant widgets
- All 472 unit tests + 12 integration tests + all doc tests pass with zero behavior change

## Task Commits

Each task was committed atomically:

1. **Task 1: Add inheritance codegen and inventory support** - `74ca333` (feat)
2. **Task 2: Migrate all 24 remaining widgets and wire inheritance** - `8722a1b` (feat)

## Files Created/Modified
- `native-theme-derive/src/gen_inherit.rs` - Inheritance codegen from #[theme(inherit_from)] attributes
- `native-theme-derive/src/lib.rs` - Wire gen_inherit, inventory::submit! generation, to_snake_case
- `native-theme-derive/src/parse.rs` - Add skip_inventory to LayerMeta, remove dead_code from inherit_from
- `native-theme-derive/src/gen_ranges.rs` - Add clippy::ptr_arg allow for generated code
- `native-theme-derive/src/gen_validate.rs` - Add clippy::ptr_arg allow for generated code
- `native-theme/src/model/widgets/mod.rs` - All 28 structs migrated to derive, macros deleted, check_ranges deleted
- `native-theme/src/resolve/inheritance.rs` - Comment block, resolve_color_inheritance uses resolve_from_defaults()
- `native-theme/src/resolve/mod.rs` - WidgetFieldInfo struct with inventory::collect!
- `native-theme/Cargo.toml` - inventory 0.3 dependency
- `Cargo.lock` - Updated lockfile

## Decisions Made
- 67 inherit_from attributes total (exceeds ~55 estimate -- the plan's count was approximate; all matching rules were migrated)
- Safety nets (6 rules) stay hand-written because they have platform-specific fallback logic
- Widget-to-widget chains (7 rules) stay hand-written because they depend on resolved font/border
- LayoutTheme uses #[theme_layer(skip_inventory)] since it lives on Theme, not ThemeMode
- Test widgets (TestWidget, DualNestedTestWidget) use skip_inventory and cfg(test)
- border_kind classification: menu/tab/card = "none" (no required border fields), sidebar/status_bar = "partial" (color + line_width only), 20 others = "full" (default)
- Added #[allow(clippy::ptr_arg)] to generated check_ranges/validate_widget impls since helpers push to Vec

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Clippy ptr_arg warning on generated code**
- **Found during:** Task 2 (pre-release-check)
- **Issue:** Generated check_ranges() and validate_widget() use `&mut Vec<T>` which clippy warns about
- **Fix:** Added `#[allow(clippy::ptr_arg)]` to generated impl blocks (Vec is correct since helpers call push())
- **Files modified:** native-theme-derive/src/gen_ranges.rs, native-theme-derive/src/gen_validate.rs
- **Committed in:** 8722a1b

---

**Total deviations:** 1 auto-fixed (1 bug fix)
**Impact on plan:** Clippy conformance required for pre-release-check. No scope creep.

## Issues Encountered
- Pre-existing `build_gnome_spec_pure` dead_code warning in connector crates (gpui, iced) causes clippy failure on those crates. Not introduced by this plan, documented in Plan 01 SUMMARY and deferred-items.md.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 80 complete: all 4 ROADMAP success criteria met
- native-theme-derive crate fully operational with 6 generation modules
- Every widget struct pair is generated from a single annotated source
- Ready for Phase 81 (feature-matrix cleanup) which absorbs all prior changes

---
*Phase: 80-native-theme-derive-proc-macro-k-codegen*
*Completed: 2026-04-13*

## Self-Check: PASSED

All files exist, all commits verified.
