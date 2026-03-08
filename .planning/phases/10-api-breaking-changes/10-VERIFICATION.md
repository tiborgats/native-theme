---
phase: 10-api-breaking-changes
verified: 2026-03-08T14:30:00Z
status: passed
score: 5/5 must-haves verified
re_verification:
  previous_status: passed
  previous_score: 5/5
  gaps_closed: []
  gaps_remaining: []
  regressions: []
---

# Phase 10: API Breaking Changes Verification Report

**Phase Goal:** Public API refactored to its final v0.2 shape -- flat colors, idiomatic methods, extended geometry -- before any new features build on it
**Verified:** 2026-03-08T14:30:00Z
**Status:** PASSED
**Re-verification:** Yes -- confirming previous pass (no gaps to close)

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | ThemeColors has 36 direct Option<Rgba> fields with no nested sub-structs | VERIFIED | `grep -c 'pub.*Option<Rgba>' colors.rs` returns 36. No CoreColors, ActionColors, StatusColors, InteractiveColors, PanelColors, or ComponentColors types found anywhere in `native-theme/src/`. Struct is flat with 7 logical groups as comments only. |
| 2 | All 17 preset TOML files use flat [light.colors] / [dark.colors] tables and load correctly | VERIFIED | 17 TOML files in `native-theme/src/presets/`. Zero matches for `[light.colors.` or `[dark.colors.` sub-table patterns. Sample file (adwaita.toml) confirmed: flat keys directly under `[light.colors]` and `[dark.colors]`. All 17 parse and round-trip via test suite. |
| 3 | NativeTheme::preset(), ::from_toml(), ::from_file(), ::list_presets(), .to_toml() work; old free functions removed | VERIFIED | All 5 methods present in `impl NativeTheme` block in `model/mod.rs` (lines 119-179), each delegating to `crate::presets::*`. Functions in `presets.rs` are `pub(crate)`. No `pub use presets::` in `lib.rs`. All 5 doc-tests pass. |
| 4 | ThemeGeometry has radius_lg and shadow fields, and presets include values for them | VERIFIED | `geometry.rs` has `radius_lg: Option<f32>` (line 18) and `shadow: Option<bool>` (line 33). All 17 presets contain `radius_lg` (34 occurrences across 17 files, 2 per file for light+dark). All 17 presets contain `shadow` in geometry sections. Integration test `all_presets_have_geometry` asserts `radius_lg.is_some()` and `shadow.is_some()` for both variants of every preset. |
| 5 | All existing tests pass against the new API (updated as needed) | VERIFIED | `cargo test -p native-theme`: 93 unit + 11 merge + 12 preset_loading + 7 serde_roundtrip + 9 doc-tests (3 ignored) = 132 passed, 0 failed. |

**Score:** 5/5 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `native-theme/src/model/colors.rs` | Flat ThemeColors with 36 Option<Rgba> fields | VERIFIED | 36 fields confirmed by grep count, impl_merge with option{} block listing all 36 fields, no sub-structs |
| `native-theme/src/model/geometry.rs` | ThemeGeometry with radius_lg and shadow | VERIFIED | 7 fields total including radius_lg: Option<f32> and shadow: Option<bool>, impl_merge covers all 7, serde round-trip test covers both new fields |
| `native-theme/src/model/mod.rs` | impl NativeTheme with 5 associated methods | VERIFIED | preset, from_toml, from_file, list_presets, to_toml all present with doc examples; each delegates to crate::presets |
| `native-theme/src/presets.rs` | TOML constants and pub(crate) functions | VERIFIED | 17 TOML constants via include_str!, PRESET_NAMES array, 5 pub(crate) functions |
| `native-theme/src/lib.rs` | No free function re-exports | VERIFIED | Only re-exports types (NativeTheme, ThemeColors, etc.); grep for `pub use presets::` or `pub fn (preset\|from_toml\|from_file\|list_presets\|to_toml)` returns no matches |
| `native-theme/src/kde/colors.rs` | KDE reader with flat field access | VERIFIED | Direct `ThemeColors { accent: ..., background: ..., ... }` construction with all 36 fields |
| `native-theme/src/gnome/mod.rs` | GNOME reader using NativeTheme::preset | VERIFIED | `crate::NativeTheme::preset("adwaita")` found at lines 97 and 150 |
| `native-theme/src/presets/adwaita.toml` | Flat color format with radius_lg and shadow | VERIFIED | 144 lines, flat `[light.colors]`/`[dark.colors]` with 36 color keys each, `radius_lg = 14.0` and `shadow = true` in both geometry sections |
| All 17 preset TOML files | Flat format with geometry extensions | VERIFIED | All 17 files confirmed: flat color tables, radius_lg and shadow present in both variants |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `model/colors.rs` | `presets/*.toml` | serde field names match TOML keys | WIRED | All 17 presets parse successfully via `from_toml()` |
| `model/mod.rs` | `presets.rs` | `crate::presets::` delegation | WIRED | All 5 methods call corresponding `crate::presets::` functions |
| `lib.rs` | `model/mod.rs` | `pub use model::NativeTheme` | WIRED | Methods come with the type re-export |
| `gnome/mod.rs` | `model/mod.rs` | `NativeTheme::preset("adwaita")` | WIRED | Found at lines 97 and 150 in gnome/mod.rs |
| `kde/colors.rs` | `model/colors.rs` | ThemeColors flat construction | WIRED | Direct struct literal with all flat fields |
| `model/geometry.rs` | `presets/*.toml` | serde field names | WIRED | radius_lg and shadow keys in TOML match struct field names; round-trip tests pass |
| `tests/preset_loading.rs` | `model/geometry.rs` | geometry field assertions | WIRED | `all_presets_have_geometry` test asserts `radius_lg.is_some()` and `shadow.is_some()` |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|-----------|-------------|--------|----------|
| API-02 | 10-01 | ThemeColors flattened to 36 direct Option<Rgba> fields | SATISFIED | 36 fields in colors.rs, no sub-structs |
| API-03 | 10-01 | All presets updated to flat TOML format | SATISFIED | 17 files with flat [light.colors]/[dark.colors] |
| API-04 | 10-01 | Platform readers updated for flat field access | SATISFIED | KDE, GNOME, Windows readers all use flat fields |
| API-05 | 10-02 | Preset functions moved to impl NativeTheme | SATISFIED | 5 associated methods in model/mod.rs |
| API-06 | 10-02 | Old free functions removed | SATISFIED | No pub use presets::{...} exports in lib.rs |
| API-07 | 10-03 | ThemeGeometry gains radius_lg and shadow | SATISFIED | Both fields present in geometry.rs |
| API-08 | 10-03 | Presets updated with radius_lg and shadow data | SATISFIED | All 17 presets have both fields in both variants |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| (none) | - | - | - | No anti-patterns found |

No TODOs, FIXMEs, placeholders, or stub implementations found in any modified files.

### Human Verification Required

None required. All success criteria are verifiable through automated means (compilation, test suite, grep for structural patterns). The phase is purely a structural refactoring with no visual or runtime behavior changes that would need human testing.

### Gaps Summary

No gaps found. All 5 success criteria are fully satisfied:

1. ThemeColors is a flat struct with exactly 36 Option<Rgba> fields and zero nested sub-structs.
2. All 17 preset TOML files use the flat color format and load correctly with round-trip fidelity.
3. All 5 NativeTheme associated methods work (preset, from_toml, from_file, list_presets, to_toml), and old free function exports are removed.
4. ThemeGeometry has 7 fields including radius_lg and shadow, with all presets populated.
5. Full test suite passes: 132 tests passed, 0 failed.

---

_Verified: 2026-03-08T14:30:00Z_
_Verifier: Claude (gsd-verifier)_
