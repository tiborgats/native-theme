---
phase: 93
plan: 01
subsystem: native-theme::{color, model::{border, font, resolved}, resolve::validate_helpers, native-theme-derive::gen_validate}
tags: [rgba, default, resolved-structs, v057-polish, validate-helpers, derive-macro]
requirements: [G1]
dependency_graph:
  requires: []
  provides:
    - "Rgba has no Default impl"
    - "ResolvedBorderSpec has no Default derive"
    - "ResolvedFontSpec has no Default derive"
    - "ResolvedTextScaleEntry has no Default derive"
    - "validate_helpers::require is Default-bound-free"
  affects:
    - "any future code reading a missing-field placeholder must use the helpers `resolved_*_spec_sentinel` or an explicit struct literal"
tech_stack:
  added: []
  patterns:
    - "caller-supplied fallback sentinel (explicit parameter instead of Default bound)"
    - "Option<T> grouped by type at macro call sites (option_color / option_f32 / border_required with per-field expressions)"
    - "derive-macro per-type fallback mapping with compile_error! safety net for unknown types"
key_files:
  created: []
  modified:
    - "native-theme/src/color.rs"
    - "native-theme/src/model/border.rs"
    - "native-theme/src/model/font.rs"
    - "native-theme/src/model/resolved.rs"
    - "native-theme/src/resolve/validate_helpers.rs"
    - "native-theme/src/resolve/validate.rs"
    - "native-theme/src/resolve/tests.rs"
    - "native-theme/tests/merge_behavior.rs"
    - "native-theme-derive/src/gen_validate.rs"
decisions:
  - "[Phase 93-01] `require<T: Clone>` gains an explicit `fallback: T` parameter instead of bounding `T: Default`. Call sites (validate_helpers, validate_defaults!, gen_validate) supply zero-value sentinels (Rgba::TRANSPARENT, 0.0_f32, 0u16, false, Arc::<str>::from(\"\"), FontStyle::Normal) explicitly."
  - "[Phase 93-01] validate_defaults! macro splits the former `option` group into `option_color` (Rgba fields) and `option_f32` (geometry/opacity fields) so the macro can thread the correct sentinel expression without a type-dispatching trait. `border_required` now takes `field: fallback_expr` pairs."
  - "[Phase 93-01] native-theme-derive::gen_validate emits per-field fallbacks in generated `validate_widget()` bodies via `fallback_for_ty`, which maps inner types (Rgba/f32/u16/bool/Arc/String/DialogButtonOrder) to their zero-value sentinel. Unknown types emit a `compile_error!` naming the offending field."
  - "[Phase 93-01] The `impl Default for Rgba` that Phase 90-01 kept as a manual impl is deleted. `Rgba` is now `Clone + Copy + Debug + PartialEq + Eq + Hash`; no Default. Callers use `Rgba::TRANSPARENT`, `Rgba::BLACK`, `Rgba::WHITE`, `Rgba::rgb`, or `Rgba::new`."
  - "[Phase 93-01] `ResolvedBorderSpec`, `ResolvedFontSpec`, `ResolvedTextScaleEntry` no longer derive Default. The validate-path sentinel construction lives in `resolve::validate_helpers::resolved_{border_spec,font_spec}_sentinel` and inline in `require_text_scale_entry`. 26 generated `Resolved*Theme` widget structs already lacked Default (ThemeWidget derive omits it)."
  - "[Phase 93-01] Integration test `trait_assertions_default_clone_debug` updated: `Rgba` now asserts `Clone + Debug` only (no Default). All other default-bearing types retain the full Default+Clone+Debug assertion."
metrics:
  duration: "~60 min"
  completed: 2026-04-19
  tasks_completed: 2
  commits: 3
---

# Phase 93 Plan 01: Remove `Rgba::Default` and break the validate_helpers Default-bound chain — Summary

**One-liner:** G1 closed — `Rgba`, `ResolvedBorderSpec`, `ResolvedFontSpec`, and `ResolvedTextScaleEntry` no longer implement/derive Default; `require<T: Clone>` takes an explicit fallback parameter, and the ThemeWidget derive emits per-field sentinels.

## Scope

Source gap: `docs/todo_v0.5.7_gaps.md` §G1 "Remove `Rgba`'s `Default` implementation (P1)".

Phase 90-01 accepted a manual `impl Default for Rgba` as a principled deviation because 30+ downstream Resolved structs transitively required the bound via `require<T: Clone + Default>` in `validate_helpers.rs`. The six-agent audit on 2026-04-19 found that only 3 hand-written Resolved structs actually derived Default (not ~30 as the gap doc's language implied), and the real blocker was the bound in `require`. Breaking that bound lets the Default impl go.

## Tasks completed

### Task 1 — Rewrite `validate_helpers::require` to drop the `T: Default` bound

Commit: `266d4d2 feat(93-01): break Default-bound chain in validate_helpers::require`

RED commit: `0da8da1 test(93-01): add failing tests for require() without Default bound` (5 regression tests against the new signature)

Touches `native-theme/src/resolve/validate_helpers.rs`, `validate.rs`, `tests.rs`, and `native-theme-derive/src/gen_validate.rs`.

- Changed `require<T: Clone + Default>(field, path, missing) -> T` to `require<T: Clone>(field, path, missing, fallback: T) -> T`.
- Added helpers `empty_arc_str`, `resolved_font_spec_sentinel`, `resolved_border_spec_sentinel` that construct zero values via struct literal (no `::default()` calls).
- `require_font`, `require_font_opt`, `require_text_scale_entry`, `validate_border` all supply explicit fallback expressions.
- `validate_defaults!` macro splits the former monomorphic `option` group into `option_color` (Rgba) and `option_f32` (geometry) so the macro can thread the correct sentinel. `border_required` takes `field: fallback_expr` pairs.
- `validate.rs` call site migrated to the new macro syntax.
- `gen_validate.rs` emits `#fallback` per-field in generated `require()` calls, with `fallback_for_ty` mapping inner types (`Rgba`, `f32`, `u16`, `bool`, `Arc` (`Arc<str>`), `String`, `DialogButtonOrder`) to their zero-value sentinel. Unknown types produce a `compile_error!` that names the offending field.

### Task 2 — Remove `impl Default for Rgba` and the three hand-written Resolved-leaf Default derives

Commit: `e66cd7b fix(93-01): remove Rgba Default impl and three Resolved-leaf Default derives`

Touches `native-theme/src/color.rs`, `model/border.rs`, `model/font.rs`, `model/resolved.rs`, and `tests/merge_behavior.rs`.

- `Rgba` loses `impl Default`. Callers use constants (`TRANSPARENT`/`BLACK`/`WHITE`) or constructors (`rgb`/`new`/`from_f32`/`from_str`).
- `ResolvedBorderSpec` drops `Default` from its derive list. The former `resolved_border_spec_default` unit test is replaced with a field-name compile guard (`resolved_border_spec_struct_literal_compiles`).
- `ResolvedFontSpec` drops `Default` from its derive list.
- `ResolvedTextScaleEntry` drops `Default` from its derive list.
- Integration test `trait_assertions_default_clone_debug` in `tests/merge_behavior.rs`: `Rgba` asserts `Clone + Debug` only; Theme / ThemeMode / ThemeDefaults / FontSpec keep the full Default+Clone+Debug assertion.

## Verification

- `cargo build --all-features` — green.
- `cargo test --package native-theme --lib --all-features` — 776 passed, 0 failed, 3 ignored.
- `cargo test --package native-theme --tests --all-features` — 79 passed, 0 failed.
- `grep -rE "Rgba::default|ResolvedBorderSpec::default|ResolvedFontSpec::default|ResolvedTextScaleEntry::default" native-theme/ connectors/` — only comments referencing the old behaviour (no actual calls).
- `grep -rE "impl Default for (Rgba|ResolvedBorderSpec|ResolvedFontSpec|ResolvedTextScaleEntry)" native-theme/` — only comments.

## Gap-doc correction

The §G1 "Concrete plan" step 1 asked to "remove `Default` from the `derive` list on all `Resolved*` widget structs generated by `ThemeWidget`". Inspection of `native-theme-derive/src/gen_structs.rs:29` shows the macro already emits only `#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]` — no Default, for the 26 widget resolved structs. The Default derives that actually needed removal are the three hand-written leaves (`ResolvedBorderSpec`, `ResolvedFontSpec`, `ResolvedTextScaleEntry`), not ~30. The real blocker was the `require<T: Clone + Default>` bound in `validate_helpers.rs`. Both conditions are now fixed.

## Deviations from plan

### Auto-fixed issues

**1. [Rule 3 - Blocking] Transient merge-conflict markers in `native-theme-derive/src/gen_ranges.rs` + `parse.rs`**
- **Found during:** initial baseline build after RED commit.
- **Issue:** `git status` reported "modificados por ambos" for two files that contained `<<<<<<<`/`=======`/`>>>>>>>` markers from a prior aborted stash pop. Cargo refused to compile.
- **Fix:** the markers disappeared after the auto-rustfmt/formatter hook re-ran on an unrelated edit. No manual intervention needed; the index was already clean.
- **Files modified:** none (self-resolved).
- **Commit:** none — resolved before any Task 1 commit.

**2. [Rule 2 - Missing critical functionality] `String` and `DialogButtonOrder` inner-type mapping in gen_validate**
- **Found during:** Task 1 GREEN — first test-suite build after adding `fallback_for_ty`.
- **Issue:** The initial mapping covered Rgba/f32/u16/bool/Arc but the widget test fixture (`TestWidget`) has an `Option<String>` field, and `DialogTheme::button_order` is `Option<DialogButtonOrder>`. Both produced `compile_error!` from the safety net.
- **Fix:** Added branches for `String` (`String::new()`) and `DialogButtonOrder` (`::PrimaryRight` — matches the type's own `#[default]` variant).
- **Files modified:** `native-theme-derive/src/gen_validate.rs`.
- **Commit:** folded into `266d4d2`.

**3. [Rule 3 - Blocking] `cargo test --workspace` fails on `naga` crate**
- **Found during:** broad test-suite verification.
- **Issue:** `naga v27.0.3` (pulled via gpui-component) fails to compile with `WriteColor` trait-bound errors on upstream code paths; unrelated to native-theme.
- **Fix:** scoped tests to `--package native-theme`, which covers all G1 deliverables. Logged to `deferred-items.md` for future phase follow-up.
- **Files modified:** none.
- **Commit:** none.

**4. [Rule 3 - Blocking] Two pre-existing doctest failures on `bundled_icon_svg`/`bundled_icon_by_name`**
- **Found during:** `cargo test --package native-theme --all-features` (doctest pass).
- **Issue:** Plan 93-03 (commit `7ba2b4c`) demoted these helpers to `pub(crate)` but their doctest examples still reference the public path.
- **Fix:** out of scope for G1. Logged to `deferred-items.md`. Does not affect the Rgba/Default changes.
- **Files modified:** none.
- **Commit:** none.

**5. [Rule 3 - Blocking] Pre-release-check.sh clippy failure on `bundled_icon_by_name` dead code**
- **Found during:** running `./pre-release-check.sh` after Task 2.
- **Issue:** After G3's demotion, `bundled_icon_by_name` has zero internal callers; `clippy -D warnings` flags it as dead code.
- **Fix:** out of scope for G1. Verified the failure is identical with my work stashed (pre-existing from 93-03). Logged to `deferred-items.md`.
- **Files modified:** none.
- **Commit:** none.

No architectural (Rule 4) decisions required.

No authentication gates.

## Known Stubs

None — the Rgba / Resolved-leaf sentinels are explicit zero values in crate-private helpers that are only observable via `ResolutionIncomplete` error branches, never surfaced in public API.

## Threat Flags

None. The plan's threat register (T-93-01-01 "fallback source", T-93-01-02 "TRANSPARENT sentinel information disclosure") both carry `disposition: accept` — fallback is caller-supplied at codegen time (not user input), and sentinel values are observationally identical to the pre-change `Default::default()` output.

## Self-Check: PASSED

- FOUND: native-theme/src/color.rs (committed in e66cd7b)
- FOUND: native-theme/src/model/border.rs (committed in e66cd7b)
- FOUND: native-theme/src/model/font.rs (committed in e66cd7b)
- FOUND: native-theme/src/model/resolved.rs (committed in e66cd7b)
- FOUND: native-theme/src/resolve/validate_helpers.rs (committed in 266d4d2)
- FOUND: native-theme/src/resolve/validate.rs (committed in 266d4d2)
- FOUND: native-theme/src/resolve/tests.rs (committed in 0da8da1)
- FOUND: native-theme-derive/src/gen_validate.rs (committed in 266d4d2)
- FOUND: native-theme/tests/merge_behavior.rs (committed in e66cd7b)
- FOUND commit 0da8da1 (RED tests)
- FOUND commit 266d4d2 (Task 1 GREEN)
- FOUND commit e66cd7b (Task 2)
