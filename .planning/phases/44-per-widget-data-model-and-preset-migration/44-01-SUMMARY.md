---
phase: 44-per-widget-data-model-and-preset-migration
plan: "01"
subsystem: native-theme/model
tags: [model, macros, foundation-types, serde, merge]
dependency_graph:
  requires: []
  provides:
    - FontSpec with family/size/weight Option fields and impl_merge!
    - TextScaleEntry with size/weight/line_height and impl_merge!
    - TextScale with 4 named entries using optional_nested macro clause
    - IconSizes with 5 Option<f32> context fields and impl_merge!
    - DialogButtonOrder enum with TrailingAffirmative/LeadingAffirmative
    - ResolvedFontSpec concrete struct for post-resolution use
    - define_widget_pair! macro for generating Option/Resolved struct pairs
    - impl_merge! extended with optional_nested clause
  affects:
    - native-theme/src/lib.rs (impl_merge! macro)
    - native-theme/src/model/mod.rs (new module declarations and re-exports)
tech_stack:
  added: []
  patterns:
    - define_widget_pair! uses [OptType, ResType] bracket syntax to avoid Rust ty/path
      fragment restriction (neither ty nor path fragments can precede / token)
    - optional_nested clause handles 4-way match: None+None, Some+None, None+Some, Some+Some
    - TextScale's manual merge/is_empty converted to macro invocation after macro extension
key_files:
  created:
    - native-theme/src/model/font.rs
    - native-theme/src/model/icon_sizes.rs
    - native-theme/src/model/dialog_order.rs
    - native-theme/src/model/widgets/mod.rs
  modified:
    - native-theme/src/lib.rs
    - native-theme/src/model/mod.rs
decisions:
  - define_widget_pair! uses [OptType, ResType] bracket syntax instead of OptType/ResType
    slash syntax because Rust macro_rules ty and path fragment specifiers cannot be
    followed by a forward slash token
  - TextScale initially had manual merge/is_empty matching optional_nested pattern;
    converted to macro after optional_nested was implemented (plan-specified approach)
  - DialogButtonOrder tests use a wrapper struct because TOML cannot serialize bare
    enum variants as top-level document values
metrics:
  duration_minutes: ~15
  completed_date: "2026-03-27"
  tasks_completed: 2
  files_created: 4
  files_modified: 2
  tests_before: 211
  tests_after: 259
  new_tests: 48
---

# Phase 44 Plan 01: Foundation Types and Macros Summary

Foundation types and macros for the per-widget architecture: FontSpec/TextScale/IconSizes/DialogButtonOrder structs with full serde+merge support, ResolvedFontSpec for post-resolution use, optional_nested clause added to impl_merge!, and define_widget_pair! macro for generating paired Option/Resolved struct definitions.

## Tasks Completed

| Task | Name | Commit | Files |
|------|------|--------|-------|
| 1 | Foundation type modules | e802d2f | font.rs, icon_sizes.rs, dialog_order.rs, widgets/mod.rs (stub), model/mod.rs |
| 2 | Extend impl_merge! and define_widget_pair! | 3d17406 | lib.rs, font.rs (TextScale conversion), widgets/mod.rs (full) |

## What Was Built

### New Types

**FontSpec** (`native-theme/src/model/font.rs`): Option-based font specification with `family: Option<String>`, `size: Option<f32>`, `weight: Option<u16>`. Supports TOML round-trip and field-level merge.

**TextScaleEntry** (same file): Typographic role entry with `size`, `weight`, `line_height` all `Option<f32>`. Same derive stack and merge behavior.

**TextScale** (same file): Named text scale with 4 `Option<TextScaleEntry>` fields (`caption`, `section_heading`, `dialog_title`, `display`). Uses `optional_nested` clause introduced in Task 2.

**IconSizes** (`native-theme/src/model/icon_sizes.rs`): Per-context icon sizes: `toolbar`, `small`, `large`, `dialog`, `panel`, all `Option<f32>`. Marked `#[non_exhaustive]`.

**DialogButtonOrder** (`native-theme/src/model/dialog_order.rs`): Enum with `TrailingAffirmative` (Windows/GNOME) and `LeadingAffirmative` (macOS/KDE) variants. Uses `#[serde(rename_all = "snake_case")]`.

**ResolvedFontSpec** (`native-theme/src/model/widgets/mod.rs`): Concrete (non-optional) font spec for post-resolution use: `family: String`, `size: f32`, `weight: u16`.

### Extended Macro

**impl_merge! optional_nested clause** (`native-theme/src/lib.rs`): Third clause added alongside existing `option` and `nested`. For each `Option<T>` field where T has its own `merge()`:
- `None + None` → stays `None`
- `Some + None` → base preserved unchanged
- `None + Some` → base set to `Some(over.clone())`
- `Some + Some` → base inner merged with overlay inner

### New Macro

**define_widget_pair!** (`native-theme/src/model/widgets/mod.rs`): Generates paired Option-based struct (serde-ready with skip_serializing_none) and Resolved struct (plain types) from a single definition. Invokes `impl_merge!` automatically for the Option struct. Syntax:

```rust
define_widget_pair! {
    /// Doc comment
    WidgetTheme / ResolvedWidget {
        option {
            field: Type,
        }
        optional_nested {
            font: [FontSpec, ResolvedFontSpec],
        }
    }
}
```

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] DialogButtonOrder test serialization approach**
- **Found during:** Task 1 (RED phase)
- **Issue:** Tests tried to serialize a bare `DialogButtonOrder` enum as a top-level TOML value, which TOML does not support (`UnsupportedType` error)
- **Fix:** Wrapped enum in a `Wrapper { order: DialogButtonOrder }` struct for serialization tests
- **Files modified:** `native-theme/src/model/dialog_order.rs`
- **Commit:** e802d2f

**2. [Rule 1 - Bug] define_widget_pair! slash separator in optional_nested clause**
- **Found during:** Task 2 (RED phase)
- **Issue:** Rust macro `ty` and `path` fragment specifiers cannot be followed by `/` token. The plan's proposed `OptType / ResType` syntax fails to compile.
- **Fix:** Changed to bracket syntax `[OptType, ResType]` which avoids the restriction since `ty` can be followed by `,` and `]` tokens
- **Files modified:** `native-theme/src/model/widgets/mod.rs`
- **Commit:** 3d17406

## Test Coverage

- Before: 211 tests
- After: 259 tests (+48 new)
- Breakdown:
  - `model::font::tests`: 16 tests (FontSpec 8, TextScaleEntry 3, TextScale 5)
  - `model::icon_sizes::tests`: 6 tests
  - `model::dialog_order::tests`: 6 tests
  - `model::widgets::tests`: 18 tests (ResolvedFontSpec 1, generated structs 10, impl_merge! optional_nested 7)
  - All 211 prior tests preserved (0 regressions)

## Known Stubs

None. All types are fully functional with merge, is_empty, and serde support.

## Self-Check: PASSED

Files verified present:
- native-theme/src/model/font.rs: FOUND
- native-theme/src/model/icon_sizes.rs: FOUND
- native-theme/src/model/dialog_order.rs: FOUND
- native-theme/src/model/widgets/mod.rs: FOUND

Commits verified:
- e802d2f: FOUND
- 3d17406: FOUND
