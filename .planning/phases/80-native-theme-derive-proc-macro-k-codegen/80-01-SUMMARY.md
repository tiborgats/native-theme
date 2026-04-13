---
phase: 80-native-theme-derive-proc-macro-k-codegen
plan: 01
subsystem: codegen
tags: [proc-macro, derive, syn, quote, widget-theme, code-generation]

# Dependency graph
requires:
  - phase: 79-borderspec-split-and-platform-reader-visibility-audit
    provides: Clean WidgetBorderSpec/DefaultsBorderSpec split for border validation dispatch
  - phase: 62-validate-codegen
    provides: validate_helpers.rs with require/ValidateNested/check_* functions
provides:
  - native-theme-derive proc-macro crate with ThemeWidget derive
  - Automated Resolved struct, FIELD_NAMES, merge, is_empty, validate_widget, check_ranges generation
  - Auto font.size/font.weight range checks for ResolvedFontSpec nested fields
  - ButtonTheme prototype demonstrating full derive equivalence
affects: [80-02 widget migration, future widget additions]

# Tech tracking
tech-stack:
  added: [syn 2, quote 1, proc-macro2 1]
  patterns: [ThemeWidget derive macro, field-level #[theme(...)] attributes, struct-level #[theme_layer(...)]]

key-files:
  created:
    - native-theme-derive/Cargo.toml
    - native-theme-derive/src/lib.rs
    - native-theme-derive/src/parse.rs
    - native-theme-derive/src/gen_structs.rs
    - native-theme-derive/src/gen_merge.rs
    - native-theme-derive/src/gen_validate.rs
    - native-theme-derive/src/gen_ranges.rs
  modified:
    - Cargo.toml
    - native-theme/Cargo.toml
    - native-theme/src/model/widgets/mod.rs

key-decisions:
  - "Field category defaults to 'option' when no #[theme(category = ...)] attribute is present"
  - "ResolvedFontSpec nested fields auto-emit check_positive(size) and check_range_u16(weight, 100, 900) without explicit attributes"
  - "Derive macro does NOT re-emit the Option struct -- user writes serde/Default derives manually, macro only generates Resolved + impls"
  - "inherit_from and border_kind parsed but gated with #[expect(dead_code)] for Plan 02"

patterns-established:
  - "ThemeWidget derive: annotate Option struct with #[derive(ThemeWidget)], field attrs control codegen"
  - "#[theme(check/range)] for range validation, #[theme(nested, resolved_type)] for nested types"
  - "Auto font range checks: any nested field with resolved_type ResolvedFontSpec gets size+weight checks"

requirements-completed: [MODEL-01, VALID-01, BORDER-02]

# Metrics
duration: 11min
completed: 2026-04-13
---

# Phase 80 Plan 01: native-theme-derive proc-macro crate with ButtonTheme prototype

**ThemeWidget derive macro generating Resolved structs, FIELD_NAMES, merge/is_empty, validate_widget, and check_ranges from field attributes -- ButtonTheme prototype confirms exact equivalence with hand-written code**

## Performance

- **Duration:** 11 min
- **Started:** 2026-04-13T09:11:11Z
- **Completed:** 2026-04-13T09:22:45Z
- **Tasks:** 3
- **Files modified:** 11

## Accomplishments
- Created native-theme-derive proc-macro crate with 5 generation modules (parse, gen_structs, gen_merge, gen_validate, gen_ranges)
- Replaced ButtonTheme define_widget_pair! with #[derive(ThemeWidget)] -- generated code is functionally identical
- Auto-generated check_ranges() from field attributes, including automatic font size/weight checks for ResolvedFontSpec nested fields
- All 472 unit tests + all integration tests pass with zero behavior change

## Task Commits

Each task was committed atomically:

1. **Task 1: Create native-theme-derive crate with parsing and struct generation** - `a6c7387` (feat)
2. **Task 2: Add validate_widget and check_ranges generation, prototype on ButtonTheme** - `447efd8` (feat)
3. **Task 3: Verify macro expansion equivalence with cargo expand** - `8aae770` (chore)

## Files Created/Modified
- `native-theme-derive/Cargo.toml` - Proc-macro crate configuration with syn/quote deps
- `native-theme-derive/src/lib.rs` - ThemeWidget derive entry point dispatching to modules
- `native-theme-derive/src/parse.rs` - Field metadata extraction from #[theme(...)] attributes
- `native-theme-derive/src/gen_structs.rs` - Resolved struct and FIELD_NAMES const generation
- `native-theme-derive/src/gen_merge.rs` - merge() and is_empty() method generation
- `native-theme-derive/src/gen_validate.rs` - validate_widget() generation with field dispatch
- `native-theme-derive/src/gen_ranges.rs` - check_ranges() generation from range/check attributes
- `Cargo.toml` - Added native-theme-derive to workspace members
- `native-theme/Cargo.toml` - Added native-theme-derive dependency
- `native-theme/src/model/widgets/mod.rs` - ButtonTheme migrated from define_widget_pair! to derive

## Decisions Made
- Field category defaults to "option" when no explicit attribute present -- most fields are required options
- ResolvedFontSpec nested fields auto-emit font range checks (size positive, weight 100-900) without needing explicit attributes on the widget struct
- The derive macro does NOT re-emit the Option struct or its serde attributes -- users write those manually, the macro only generates the Resolved struct and impl blocks
- inherit_from field and border_kind layer attribute are parsed but marked #[expect(dead_code)] -- they are Plan 02 features

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Clippy collapsible_if and unused import fixes**
- **Found during:** Task 3 (verification)
- **Issue:** Nested if-let chains in extract_option_inner functions triggered clippy::collapsible_if; unused `Meta` import in parse.rs
- **Fix:** Collapsed to let-chains, removed unused import, added #[expect(dead_code)] for future-use fields
- **Files modified:** All 5 native-theme-derive/src/*.rs files
- **Committed in:** 8aae770

---

**Total deviations:** 1 auto-fixed (1 bug fix)
**Impact on plan:** Clippy conformance required for pre-release-check. No scope creep.

## Issues Encountered
- Pre-existing `build_gnome_spec_pure` dead_code warning in gnome/mod.rs causes clippy failure on connector crates (native-theme-gpui, native-theme-iced). This is not introduced by this plan and is logged in deferred-items.md.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- native-theme-derive crate ready for Plan 02: migrating remaining 24 widgets from define_widget_pair! to #[derive(ThemeWidget)]
- border_kind dispatch and inherit_from parsing already in place for Plan 02 usage
- define_widget_pair! macro preserved for 24 remaining widgets -- incremental migration possible

---
*Phase: 80-native-theme-derive-proc-macro-k-codegen*
*Completed: 2026-04-13*
