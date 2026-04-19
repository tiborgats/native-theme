---
phase: 93-docs-todo-v0-5-7-gaps-md
plan: 05
subsystem: native-theme-derive + native-theme::{model, resolve}
tags: [codegen, inventory, field-names, lint-toml, derive-macro, v057-polish]

# Dependency graph
requires:
  - phase: 93-01
    provides: "model/font.rs and model/border.rs touched previously; Rgba/Default cleanup left the sentinel path stable so further struct-derive additions do not reintroduce Default bounds."
  - phase: 93-03
    provides: "model/mod.rs re-export path narrowed; confirms the `pub(crate) use` visibility conventions that this plan preserves."
  - phase: 93-04
    provides: "Theme.icon_theme Option<Cow<'static, str>> + ThemeDefaults.icon_theme rustdoc update landed; this plan keeps both representations stable while its derive emits icon_theme among ThemeDefaults' 32 field names."
provides:
  - "`native-theme-derive::ThemeFields` proc-macro derive"
  - "`crate::resolve::FieldInfo` inventory-collected struct alongside `WidgetFieldInfo`"
  - "Zero hand-authored `FIELD_NAMES` constants in `native-theme/src/`"
  - "`lint_toml` consumes both widget and non-widget registries via `inventory::iter`"
  - "Renaming a serde field auto-updates the TOML lint entry without source edits"
affects: [phase-93-verification, v0.5.7-changelog, any future struct that wants to be lint_toml-aware]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Dual inventory registries (WidgetFieldInfo + FieldInfo) consumed together in lint_toml"
    - "`#[theme_layer(fields = \"a, b_px, c\")]` explicit-override attribute for serde-proxy structs"
    - "`parse_nested_meta` loop tolerates unknown sub-attributes by optionally consuming their value (Rule-2 fix)"

key-files:
  created: []
  modified:
    - "native-theme-derive/src/lib.rs - new `#[proc_macro_derive(ThemeFields, attributes(theme_layer))]` entry; emits `inventory::submit!(crate::resolve::FieldInfo { ... })`."
    - "native-theme-derive/src/parse.rs - `LayerMeta::explicit_fields` field + parser for `#[theme_layer(fields = \"...\")]`; unknown-serde-subattribute tolerance in `parse_one_field` (Rule 2 fix)."
    - "native-theme/src/resolve/mod.rs - sister `FieldInfo` struct + `inventory::collect!(FieldInfo);`."
    - "native-theme/src/model/font.rs - FontSpec, TextScaleEntry, TextScale derive ThemeFields; FontSpec and TextScaleEntry use explicit-fields attribute (serde proxy); 3 hand-authored `FIELD_NAMES` constants removed."
    - "native-theme/src/model/border.rs - DefaultsBorderSpec, WidgetBorderSpec derive ThemeFields; 2 hand-authored `FIELD_NAMES` constants removed (introspection path picks up the 4 serde renames)."
    - "native-theme/src/model/defaults.rs - ThemeDefaults derives ThemeFields; 32-entry hand-authored `FIELD_NAMES` removed."
    - "native-theme/src/model/icon_sizes.rs - IconSizes derives ThemeFields; 1 hand-authored `FIELD_NAMES` removed."
    - "native-theme/src/model/widgets/mod.rs - LayoutTheme adds ThemeFields alongside ThemeWidget (retains `skip_inventory` for the widget registry)."
    - "native-theme/src/model/mod.rs - `lint_toml` rewritten to look up both widget_registry and struct_registry from inventory; per-test-block `field_info_*_matches_baseline` tests added (8 total)."

key-decisions:
  - "[Phase 93-05] FontSpec and TextScaleEntry use `#[theme_layer(fields = \"...\")]` explicit-override because serde serializes them via private FontSpecRaw / TextScaleEntryRaw proxies whose wire field names (`size_pt`/`size_px`, `line_height_pt`/`line_height_px`) differ from the user-facing struct fields (`size`, `line_height`). The explicit attribute makes the contract visible at the struct level and avoids requiring AST access to sibling types."
  - "[Phase 93-05] LayoutTheme keeps `#[theme_layer(skip_inventory)]` (preventing widget-registry registration -- LayoutTheme is top-level, not per-variant) and additionally derives `ThemeFields` so it is available under its own key in the struct registry. lint_toml's layout-section block looks it up by `get_struct_fields(\"LayoutTheme\")`."
  - "[Phase 93-05] Missing struct registry entries -> silent skip (not error). Matches the pre-existing `continue;` behaviour when a sub-table's type was unrecognised. No new Error variant introduced; no behavioural change to lint_toml for input TOMLs that previously linted silently."
  - "[Phase 93-05] `ThemeFields` emits its inventory submit directly at item level via `inventory::submit!(crate::resolve::FieldInfo { ... });` rather than wrapping in `const _: () = { ... };`. Mirrors the existing widget derive at lib.rs:111 and keeps generated code uniform."
  - "[Phase 93-05] Rule-2 auto-fix: `parse_one_field` in native-theme-derive was calling `parse_nested_meta` on `#[serde(...)]` attributes without consuming values of sub-attributes it did not recognise. On structs with `#[serde(default, skip_serializing_if = \"...\")]` on non-`Option` fields (ThemeDefaults.font, ThemeDefaults.mono_font, ThemeDefaults.border, ThemeDefaults.icon_sizes), this produced a misleading `expected ','` compile error that looked like it originated in `serde_with::skip_serializing_none`. The parser now optionally consumes the value expression of unknown serde sub-attributes."
  - "[Phase 93-05] Gap-doc correction: the §G5 target list in `docs/todo_v0.5.7_gaps.md` names `ResolvedFontSpec` as a migration target, but that struct has no `FIELD_NAMES` constant today (it is an output type consumed by connectors, never linted). ResolvedFontSpec was NOT migrated. The true 7 targets + 1 bonus are: FontSpec, TextScaleEntry, TextScale, DefaultsBorderSpec, WidgetBorderSpec, ThemeDefaults, IconSizes (7) + LayoutTheme (bonus)."

patterns-established:
  - "When a non-widget struct needs its field names to be lint_toml-aware, add `#[derive(ThemeFields)]`. If the struct serializes through a serde proxy, also add `#[theme_layer(fields = \"wire1, wire2, ...\")]`. Hand-authored `FIELD_NAMES` constants are no longer the correct path."
  - "`parse_nested_meta` inside a derive must tolerate unknown sub-attributes by optionally consuming their values; otherwise the next separator parse fails with a cryptic error."

requirements-completed: [G5]

# Metrics
duration: 14m 15s
completed: 2026-04-19
tasks_completed: 2
commits: 3
---

# Phase 93 Plan 05: Drive non-widget FIELD_NAMES from ThemeFields derive Summary

**One-liner:** G5 closed — seven hand-authored `FIELD_NAMES` constants deleted; eight model structs derive the new `ThemeFields` proc-macro; `lint_toml` consumes a unified inventory of widget and non-widget field registries. Serde renames now auto-propagate to the TOML lint without parallel const edits.

## Performance

- **Duration:** 14 min 15 sec
- **Started:** 2026-04-19T14:49:25Z
- **Completed:** 2026-04-19T15:03:40Z
- **Tasks:** 2 (Task 1: derive + registry wiring; Task 2: apply derive + delete constants + rewrite lint_toml)
- **Files modified:** 9
- **Commits:** 3 (feat + test RED + refactor GREEN)

## Accomplishments

- **G5 closed.** `docs/todo_v0.5.7_gaps.md` §G5 Direction B is fully implemented.
- **`ThemeFields` derive** added to `native-theme-derive` with introspection + explicit-override paths. Honours `#[serde(rename = "...")]` on fields by default; accepts a struct-level `#[theme_layer(fields = "...")]` attribute to capture serde-proxy wire formats.
- **`FieldInfo` sister registry** added alongside `WidgetFieldInfo` in `native-theme/src/resolve/mod.rs`. Both registries are populated via `inventory::collect!` and consumed by `lint_toml`.
- **7 hand-authored `FIELD_NAMES` constants deleted** (font.rs x3, border.rs x2, defaults.rs x1 with 32 entries, icon_sizes.rs x1 with 5 entries). LayoutTheme's macro-generated `FIELD_NAMES` is unchanged (still emitted by `ThemeWidget`) but a second `FieldInfo` entry now lives in the struct registry too.
- **`lint_toml` rewritten** in `native-theme/src/model/mod.rs` to look up both registries via `inventory::iter` -- the former free functions `lint_text_scale`/`lint_defaults`/`lint_variant` are now closures that capture the registry HashMaps built once per call.
- **Rule-2 fix in native-theme-derive's `parse_one_field`:** tolerates unknown serde sub-attributes by optionally consuming their values. Without this, `#[derive(ThemeFields)]` on structs that have non-`Option` fields carrying `#[serde(default, skip_serializing_if = "...")]` produced a misleading `expected ','` error pointing at a line not actually at fault.
- **Full native-theme test suite green:** 791 lib tests, 79 integration tests, and 8 new baseline-equality tests all pass. `cargo fmt --all -- --check` clean; `cargo clippy -p native-theme --all-features --lib -- -D warnings` clean.
- **Rename-auto-propagation verified** via the 8 baseline tests (e.g. `defaults_border_spec_field_info_matches_baseline` asserts `"corner_radius_px"` -- the renamed name -- not `"corner_radius"` -- the struct field name).

## Task Commits

1. **Task 1 GREEN:** `4431782 feat(93-05): add ThemeFields derive and FieldInfo inventory registry`
2. **Task 2 RED:** `7ab1c58 test(93-05): add failing baseline tests for ThemeFields inventory`
3. **Task 2 GREEN:** `922ee29 refactor(93-05): drive non-widget FIELD_NAMES from ThemeFields derive; unify lint_toml`

## Files Created/Modified

**native-theme-derive (2 files)**

- `src/lib.rs` — new `#[proc_macro_derive(ThemeFields, attributes(theme_layer))]` and its helper `derive_fields_inner`. 1 doc example added.
- `src/parse.rs` — `LayerMeta::explicit_fields: Option<Vec<String>>` + parser for the `fields = "..."` attribute (with validation: non-empty comma-separated names required). `parse_one_field` gained tolerance for unknown serde sub-attributes. 6 new unit tests covering the parse path.

**native-theme (6 files)**

- `src/resolve/mod.rs` — `pub(crate) struct FieldInfo { struct_name, field_names }` + `inventory::collect!(FieldInfo)`.
- `src/model/font.rs` — FontSpec / TextScaleEntry / TextScale derive ThemeFields; FontSpec + TextScaleEntry use `#[theme_layer(fields = "...")]`; three hand-authored `FIELD_NAMES` constants deleted.
- `src/model/border.rs` — DefaultsBorderSpec / WidgetBorderSpec derive ThemeFields; two hand-authored constants deleted.
- `src/model/defaults.rs` — ThemeDefaults derives ThemeFields; 32-entry hand-authored constant deleted (a short comment now explains its removal).
- `src/model/icon_sizes.rs` — IconSizes derives ThemeFields; 1 hand-authored constant deleted.
- `src/model/widgets/mod.rs` — LayoutTheme adds ThemeFields to its derive list (alongside the existing ThemeWidget + `skip_inventory`).
- `src/model/mod.rs` — lint_toml rewritten; 9 new baseline-equality tests (`font_spec_field_info_matches_baseline`, etc.) and 1 widget-registry regression guard test.

**Planning (1 file, git-ignored)**

- `.planning/phases/93-docs-todo-v0-5-7-gaps-md/deferred-items.md` — appended Plan 05 execution notes about persistent pre-existing issues (not caused by this plan).

## Decisions Made

See `key-decisions` in the frontmatter for the formal list. Highlights:

- **Explicit `fields = "..."` for serde-proxy structs** (FontSpec, TextScaleEntry). Alternative was to teach the derive to parse `#[serde(try_from = "...")]` and fetch the proxy type's fields — this requires AST access to a sibling type, which proc-macros don't have. Explicit declaration keeps the wire contract visible at the struct level.
- **LayoutTheme dual-registers** — keeps `skip_inventory` (correct: not a per-variant widget) and gains `ThemeFields` (so lint_toml can find its fields). Cleaner than removing `skip_inventory` because LayoutTheme genuinely isn't a per-variant widget; its semantics are top-level.
- **Silent skip on missing struct entry** — no `Error::InternalBug` needed, matches pre-existing `continue;` behaviour. Keeps lint_toml's contract unchanged: it NEVER returns an error for unrecognised sub-table types, only for unparseable TOML.
- **Emission style** — `inventory::submit!(...)` at item level, not `const _: () = { ... };`. Matches the existing widget derive at lib.rs:111 for visual consistency.
- **Scope correction** — `ResolvedFontSpec` was on the §G5 target list in the gap doc but has no hand-authored `FIELD_NAMES` today and is never linted (it's an output type). Not migrated. The final migration set is 7 structs + LayoutTheme.

## Deviations from Plan

### Auto-fixed issues

**1. [Rule 2 - Missing critical functionality] `parse_one_field` must tolerate unknown serde sub-attributes**

- **Found during:** Task 2 GREEN build, immediately after applying `ThemeFields` to the 5 model files.
- **Issue:** `parse_nested_meta` on `#[serde(...)]` attributes was only handling `rename`. On fields like `ThemeDefaults.font` which carry `#[serde(default, skip_serializing_if = "FontSpec::is_empty")]`, the `default` flag and the `skip_serializing_if = "..."` key-value pair caused `parse_nested_meta`'s next-separator parse to choke with a misleading error pointing at `serde_with::skip_serializing_none` (the preceding attribute macro) rather than at the actual syntactic issue.
- **Fix:** In the `else` branch of the path dispatch inside `parse_one_field`, optionally consume the sub-attribute's value expression (`if let Ok(v) = meta.value() { let _: syn::Expr = v.parse()?; }`). Flag-style attributes (no `=` tail) continue to be silently accepted.
- **Files modified:** `native-theme-derive/src/parse.rs`.
- **Verification:** Build now green; all 791 lib tests pass.
- **Committed in:** `922ee29` (Task 2 GREEN refactor commit).

**2. [Rule 3 - Blocking] Clippy `collapsible_if` errors in new `lint_toml` closures**

- **Found during:** `./pre-release-check.sh` run after Task 2.
- **Issue:** Two nested `if let ... { if let ... { ... } }` blocks in the new lint_toml rewrite triggered `-D clippy::collapsible-if`.
- **Fix:** Collapsed both into `if let ... && let ... { ... }` form using Rust 2024 let-chains (Rust 1.89+ via edition 2024 support — matches the existing pattern used elsewhere in native-theme/src/).
- **Files modified:** `native-theme/src/model/mod.rs`.
- **Verification:** `cargo clippy -p native-theme --all-features --lib -- -D warnings` clean.
- **Committed in:** `922ee29` (same Task 2 GREEN refactor commit).

**3. [Rule 1 - Bug] Initial emission used `const _: () = { inventory::submit! { ... } };` anonymous-const wrapper**

- **Found during:** Task 2 GREEN build attempt #1.
- **Issue:** The first emission style I tried wrapped the `inventory::submit!` in `const _: () = { ... };` to scope the generated code. Although valid Rust in isolation, it provoked a misleading compile error that pointed at nearby `serde_with::skip_serializing_none` attribute invocations. In retrospect this error was actually caused by issue #1 above (parse_one_field not consuming serde sub-attribute values), not by the emission style — but at the time I didn't have enough signal to distinguish the two, and simplifying the emission path reduced the surface to debug.
- **Fix:** Emit `inventory::submit!(crate::resolve::FieldInfo { ... });` directly at item level, matching the existing widget derive at lib.rs:111. After issue #1 was also fixed, the build went green.
- **Files modified:** `native-theme-derive/src/lib.rs`.
- **Committed in:** `922ee29` (same Task 2 commit; the emission-style simplification and the parse_one_field fix landed together).

**Total deviations:** 3 auto-fixed (1 Rule-1 simplification, 1 Rule-2 critical-functionality gap exposed by the new derive's wider reach, 1 Rule-3 blocking clippy). No Rule-4 architectural decisions triggered.

### Compliance with CLAUDE.md / user memory

- No runtime panics or `.unwrap()` added in library code: the derive uses `?` on `syn::Result`; lint_toml uses `Option::or` / `let-else` / `if let` for all fallible paths.
- No hardcoded theme values: this plan is a pure codegen/metadata refactor -- zero runtime theme fields changed.
- No AI attribution trailers on any commit (honoured user rule).
- All pre-release-check.sh steps that Plan 05 touches (cargo check, cargo fmt, clippy on `-p native-theme --lib`) pass; the `clippy -p native-theme --all-targets` step continues to fail on the pre-existing `bundled_icon_by_name` dead-code item (Plan 93-03 follow-up, documented in deferred-items.md).

## TDD Gate Compliance

Per the plan's `type=tdd` markers on both tasks, the execution followed the RED/GREEN flow where it provided meaningful coverage:

- **Task 1 (`feat` 4431782):** combined RED+GREEN in one commit. The parse-layer tests added alongside the implementation exercise the new `explicit_fields` parser end-to-end. Separating them would have required committing tests that reference compile-time tokens of a struct field (`LayerMeta::explicit_fields`) that did not yet exist. Not materially meaningful for this derive's scope.
- **Task 2 RED (`test` 7ab1c58):** 8 baseline-equality tests + 1 widget-registry regression guard committed BEFORE any struct migration. All 8 failed with `left: None, right: Some([...baseline...])`. Widget guard passed (verified pre-migration).
- **Task 2 GREEN (`refactor` 922ee29):** all 8 baseline tests pass post-migration, plus the 3 auto-fixed deviations.
- **REFACTOR phase:** not invoked separately — the GREEN commit already reads as the refactored final shape (no dead code, no duplicated helpers).

Gate sequence in `git log --oneline`:

```
922ee29 refactor(93-05): drive non-widget FIELD_NAMES from ThemeFields derive; unify lint_toml  ← GREEN
7ab1c58 test(93-05): add failing baseline tests for ThemeFields inventory                      ← RED
4431782 feat(93-05): add ThemeFields derive and FieldInfo inventory registry                    ← Task 1 (combined)
```

## Verification

- `cargo build --all-features` — green (workspace + all features).
- `cargo test -p native-theme --all-features --lib` — 791 passed, 0 failed, 3 ignored.
- `cargo test -p native-theme --all-features --tests` — 79 passed across 8 integration binaries.
- `cargo test -p native-theme --all-features field_info` — 8 baseline-equality tests pass.
- `cargo test -p native-theme --all-features lint_toml` — 16 lint_toml tests pass.
- `cargo test -p native-theme-derive` — 9 parse-layer tests pass (including 6 new `explicit_fields_*` tests).
- `cargo fmt --all -- --check` — clean.
- `cargo clippy -p native-theme --all-features --lib -- -D warnings` — clean.
- `grep -rnE "pub const FIELD_NAMES" native-theme/src/` — empty.
- `grep -rnE "(FontSpec|TextScaleEntry|TextScale|DefaultsBorderSpec|WidgetBorderSpec|ThemeDefaults|IconSizes)::FIELD_NAMES" native-theme/src/` — empty (except one comment citing the removal).
- `grep -rnE "LayoutTheme::FIELD_NAMES" native-theme/src/` — 5 hits, all in the `layout_theme_field_names` unit test exercising the macro-generated constant from `ThemeWidget` (unchanged and expected).

Pre-existing blockers (NOT caused by Plan 05, NOT fixed by Plan 05 per SCOPE BOUNDARY):

- `cargo clippy -p native-theme --all-targets -- -D warnings` fails on `bundled_icon_by_name` dead-code (pre-existing from Plan 93-03).
- `cargo test -p native-theme --all-features --doc` fails on 2 doctests referencing the pre-existing now-private `bundled_icon_svg` / `bundled_icon_by_name` paths.
- `cargo test --workspace --all-features` fails on `naga` upstream compile error (documented in Plan 93-01 deferred-items).

All three are logged in `.planning/phases/93-docs-todo-v0-5-7-gaps-md/deferred-items.md`.

## Known Stubs

None. This plan is a pure metadata refactor — no new runtime fields, no placeholder values.

## Threat Flags

None. The plan's threat register (T-93-05-01 "serde-proxy field-list drift") is mitigated at CI time by the 8 baseline-equality tests which assert bit-for-bit equality between the derive output and the pre-migration hand-authored lists. T-93-05-02 "inventory::iter reentrancy" was accepted by the plan as precedent.

## User Setup Required

None.

## Next Phase Readiness

- **G5 closed.** Phase 93 can now be sealed by its verifier.
- **Established pattern for future lint_toml-aware structs:** when adding a new non-widget struct that needs its fields in the lint_toml vocabulary, add `#[derive(ThemeFields)]` (with `#[theme_layer(fields = "...")]` if serde-proxy). No hand-authored `FIELD_NAMES` array ever again.
- **Widget vs struct boundary:** structs that should register as per-variant widgets use `#[derive(ThemeWidget)]` (populates `WidgetFieldInfo`); top-level structs or plain helpers use `#[derive(ThemeFields)]` (populates `FieldInfo`). LayoutTheme demonstrates the dual-derive pattern for types that straddle both (macro-generated Resolved pair but top-level lint scope).

## Self-Check: PASSED

Files verified (9):

- FOUND: `native-theme-derive/src/lib.rs` (committed in `4431782` and `922ee29`)
- FOUND: `native-theme-derive/src/parse.rs` (committed in `4431782` and `922ee29`)
- FOUND: `native-theme/src/resolve/mod.rs` (committed in `4431782`)
- FOUND: `native-theme/src/model/font.rs` (committed in `922ee29`)
- FOUND: `native-theme/src/model/border.rs` (committed in `922ee29`)
- FOUND: `native-theme/src/model/defaults.rs` (committed in `922ee29`)
- FOUND: `native-theme/src/model/icon_sizes.rs` (committed in `922ee29`)
- FOUND: `native-theme/src/model/widgets/mod.rs` (committed in `922ee29`)
- FOUND: `native-theme/src/model/mod.rs` (committed in `7ab1c58` and `922ee29`)

Commits verified (3):

- FOUND: `4431782 feat(93-05): add ThemeFields derive and FieldInfo inventory registry`
- FOUND: `7ab1c58 test(93-05): add failing baseline tests for ThemeFields inventory`
- FOUND: `922ee29 refactor(93-05): drive non-widget FIELD_NAMES from ThemeFields derive; unify lint_toml`

Plan done-criteria verified:

- `ThemeFields` derive exists and is exported from native-theme-derive — PASS
- `FieldInfo` struct + `inventory::collect!` in place in `resolve/mod.rs` — PASS
- 7 hand-authored `FIELD_NAMES` constants deleted — PASS (`grep -rnE "pub const FIELD_NAMES" native-theme/src/` empty)
- 8 structs (FontSpec, TextScaleEntry, TextScale, DefaultsBorderSpec, WidgetBorderSpec, ThemeDefaults, IconSizes, LayoutTheme) derive `ThemeFields` — PASS
- FontSpec and TextScaleEntry use `#[theme_layer(fields = "...")]` — PASS
- `lint_toml` consumes `inventory::iter::<FieldInfo>()` for all non-widget lookups — PASS
- Baseline-equality tests pass for all 8 structs — PASS (8 tests; confirmed GREEN after migration, were RED before as required)
- Widget registry regression guard passes (≥25 widgets) — PASS
- Full native-theme lib + integration test suite green (791 + 79) — PASS

---

*Phase: 93-docs-todo-v0-5-7-gaps-md*
*Completed: 2026-04-19*
