---
phase: 59-implement-chapter-2-of-docs-todo-v0-5-5-pt-px-md
verified: 2026-04-08T15:10:00Z
status: passed
score: 11/11 must-haves verified
re_verification: false
---

# Phase 59: Implement Chapter 2 of docs/todo_v0.5.5_pt-px.md Verification Report

**Phase Goal:** Font sizes use a `FontSize` enum (`Pt(f32)` / `Px(f32)`) with compile-time unit safety, TOML presets use explicit `size_pt` / `size_px` keys, and pt-to-px conversion moves from hidden in-place mutation to type transformation in validate.
**Verified:** 2026-04-08
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | FontSize enum exists with Pt(f32) and Px(f32) variants | VERIFIED | `native-theme/src/model/font.rs` lines 29-37: `pub enum FontSize { Pt(f32), Px(f32) }` with `to_px`, `raw`, `is_pt` methods and `Default` impl |
| 2 | FontSpec.size is Option<FontSize> not Option<f32> | VERIFIED | `font.rs` line 86: `pub size: Option<FontSize>` |
| 3 | TextScaleEntry.size is Option<FontSize> not Option<f32> | VERIFIED | `font.rs` line 184: `pub size: Option<FontSize>` |
| 4 | FontSpec serde uses proxy struct mapping to size_pt/size_px TOML keys | VERIFIED | `#[serde(try_from = "FontSpecRaw", into = "FontSpecRaw")]` on FontSpec; `FontSpecRaw` has `size_pt: Option<f32>` and `size_px: Option<f32>`; `TryFrom` rejects both set simultaneously |
| 5 | TextScaleEntry serde uses proxy struct mapping to size_pt/size_px TOML keys | VERIFIED | `#[serde(try_from = "TextScaleEntryRaw", into = "TextScaleEntryRaw")]` on TextScaleEntry; `TextScaleEntryRaw` has `size_pt`/`size_px`; `TryFrom` with mutual exclusivity check |
| 6 | Phase 1.5 resolve_font_dpi_conversion is deleted from inheritance.rs | VERIFIED | Neither `convert_pt_to_px` nor `resolve_font_dpi_conversion` appear anywhere in `src/` (only a comment reference in `tests.rs` line 246 noting the deletion) |
| 7 | resolve() no longer calls resolve_font_dpi_conversion | VERIFIED | `resolve/mod.rs` `resolve()` method calls only: `resolve_defaults_internal`, `resolve_safety_nets`, `resolve_widgets_from_defaults`, `resolve_widget_to_widget`, icon_set fallback |
| 8 | validate require_font/require_font_opt/require_text_scale_entry accept dpi parameter and call FontSize::to_px(dpi) | VERIFIED | `validate.rs` lines 37-131: all three helpers have `dpi: f32` param; `font.size.map(|fs| fs.to_px(dpi))` pattern used in all three; `dpi` extracted at `validate()` top: `let dpi = self.defaults.font_dpi.unwrap_or(DEFAULT_FONT_DPI)` |
| 9 | OS readers wrap font sizes in FontSize::Pt(...) | VERIFIED | All 4 OS readers confirmed: `kde/fonts.rs` line 59, `gnome/mod.rs` line 97, `windows.rs` line 120, `macos.rs` line 211 all use `crate::model::font::FontSize::Pt(...)` |
| 10 | FontSize is publicly re-exported from lib.rs | VERIFIED | `lib.rs` line 114: `FontSize` in `pub use model::{...}` block |
| 11 | FIELD_NAMES updated to use size_pt/size_px instead of size | VERIFIED | `FontSpec::FIELD_NAMES` = `["family", "size_pt", "size_px", "weight", "style", "color"]`; `TextScaleEntry::FIELD_NAMES` = `["size_pt", "size_px", "weight", "line_height"]` |

**Score:** 11/11 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|---------|--------|---------|
| `native-theme/src/model/font.rs` | FontSize enum, FontSpecRaw proxy, TextScaleEntryRaw proxy | VERIFIED | All three present and substantive with full TryFrom/From impls |
| `native-theme/src/resolve/inheritance.rs` | Inheritance without Phase 1.5 | VERIFIED | No `resolve_font_dpi_conversion` or `convert_pt_to_px`; `resolve_text_scale_entry` uses `font_size.raw()` at line 86 |
| `native-theme/src/resolve/validate.rs` | Validation with dpi parameter on font helpers | VERIFIED | All three helpers have `dpi: f32` param; `to_px(dpi)` called at extraction time |
| `native-theme/src/presets/kde-breeze.toml` | Platform preset with size_pt keys | VERIFIED | 12 occurrences of `size_pt`, 0 of `size_px`, 0 bare `size =` |
| `native-theme/src/presets/dracula.toml` | Community preset with size_px keys | VERIFIED | 12 occurrences of `size_px`, 0 of `size_pt`, 0 bare `size =` |
| `docs/property-registry.toml` | Updated Font and TextScaleEntry structures | VERIFIED | Lines 36-39: `size_pt` and `size_px` in `[_structures.Font]`; lines 68-69 same in `[_structures.TextScaleEntry]` |
| `native-theme/src/model/font.rs` (tests) | FontSize unit tests and serde round-trip tests | VERIFIED | `pt_to_px_at_96_dpi`, `px_ignores_dpi`, `pt_to_px_at_72_dpi_is_identity`, `raw_extracts_value`, `font_size_default_is_px_zero`, plus 7 serde round-trip tests |
| `native-theme/src/resolve/tests.rs` | Rewritten Phase 1.5 tests as validate-based tests | VERIFIED | 6 validate-based tests: `validate_converts_pt_to_px_at_96_dpi`, `validate_px_ignores_dpi`, `validate_pt_at_72_dpi_is_identity`, `validate_text_scale_pt_converted`, `validate_per_widget_font_pt_converted`, `validate_no_dpi_uses_default_96` |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `native-theme/src/resolve/validate.rs` | `FontSize::to_px(dpi)` | `require_font`/`require_font_opt`/`require_text_scale_entry` | WIRED | Pattern `to_px(dpi)` confirmed in lines 44, 74, 106 of validate.rs |
| `native-theme/src/model/font.rs` | serde proxy structs | `#[serde(try_from = "FontSpecRaw", into = "FontSpecRaw")]` | WIRED | Both `TryFrom<FontSpecRaw> for FontSpec` and `From<FontSpec> for FontSpecRaw` implemented |
| `native-theme/src/presets/*.toml` | FontSpecRaw serde | TOML deserialization through proxy struct | WIRED | All 20 presets use `size_pt` or `size_px` which are fields of `FontSpecRaw`/`TextScaleEntryRaw` |
| `native-theme/src/resolve/tests.rs` | `validate()` | FontSize::Pt values resolved through validate | WIRED | Tests use `fully_populated_variant()` + `validate()` to assert pt-to-px conversions |

### Behavioral Spot-Checks

| Behavior | Result | Status |
|----------|--------|--------|
| `cargo test -p native-theme` | 447 lib tests + 12 integration serde + 6 integration other suites — all passed, 0 failed | PASS |
| Specific FontSize unit tests (5 tests) | `pt_to_px_at_96_dpi`, `px_ignores_dpi`, `pt_to_px_at_72_dpi_is_identity`, `raw_extracts_value`, `font_size_default_is_px_zero` — all ok | PASS |
| Specific serde round-trip tests (7 tests) | `fontspec_toml_round_trip_size_pt/px`, `fontspec_toml_rejects_both_pt_and_px`, `fontspec_toml_rejects_bare_size`, `fontspec_toml_no_size_is_valid`, `text_scale_entry_toml_round_trip_size_pt/px` — all ok | PASS |
| Validate DPI conversion tests (6 tests) | All 6 `validate_*` tests in `resolve::tests` — all ok | PASS |
| No bare `size =` in any preset | `grep -rn "^size = " native-theme/src/presets/` returns 0 lines | PASS |
| Phase 1.5 code deleted | `grep -rn "resolve_font_dpi_conversion\|convert_pt_to_px" native-theme/src/` returns 0 non-comment lines | PASS |
| Platform presets use size_pt exclusively | kde-breeze(12), kde-breeze-live(8), adwaita(10), adwaita-live(6), macos-sonoma(10), macos-sonoma-live(8), windows-11(14), windows-11-live(8), ios(12) — all size_px=0 | PASS |
| Community presets use size_px exclusively | All 11 presets have size_px=12 and size_pt=0 | PASS |

### Requirements Coverage

No explicit requirement IDs were referenced in the plans (`requirements: []` in all three). The phase goal maps to the design spec `docs/todo_v0.5.5_pt-px.md` Chapter 2, which is fully covered by the implementation.

### Anti-Patterns Found

None. No TODOs, placeholders, or stub patterns detected in the modified files. The `TryFrom` implementations, `to_px` method, and proxy structs all contain real production logic.

### Human Verification Required

None. All goal behaviors are verifiable through the type system, compilation, and automated tests.

### Gaps Summary

No gaps. All 11 must-have truths are verified, the full test suite (447 unit + integration tests) passes, all 20 TOML presets use self-documenting size keys, Phase 1.5 code is deleted, and FontSize is publicly exported and accessible.

---

_Verified: 2026-04-08_
_Verifier: Claude (gsd-verifier)_
