---
phase: 10-api-breaking-changes
verified: 2026-03-08T06:15:00Z
status: passed
score: 5/5 must-haves verified
---

# Phase 10: API Breaking Changes Verification Report

**Phase Goal:** Public API refactored to its final v0.2 shape -- flat colors, idiomatic methods, extended geometry -- before any new features build on it
**Verified:** 2026-03-08T06:15:00Z
**Status:** PASSED
**Re-verification:** No -- initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | ThemeColors has 36 direct Option<Rgba> fields with no nested sub-structs | VERIFIED | `native-theme/src/model/colors.rs` has exactly 36 `pub ... Option<Rgba>` fields. No CoreColors, ActionColors, StatusColors, InteractiveColors, PanelColors, or ComponentColors types exist anywhere in src/ or tests/. |
| 2 | All 17 preset TOML files use flat [light.colors] / [dark.colors] tables and load correctly | VERIFIED | 17 TOML files exist in `native-theme/src/presets/`. Zero matches for `[light.colors.` or `[dark.colors.` sub-tables. All use flat `[light.colors]` and `[dark.colors]`. All 17 load and round-trip successfully (tested). |
| 3 | NativeTheme::preset("adwaita"), ::from_toml(), ::from_file(), ::list_presets(), and .to_toml() work; old free functions removed | VERIFIED | `impl NativeTheme` block in `native-theme/src/model/mod.rs` contains all 5 methods delegating to `crate::presets::*`. presets.rs functions are `pub(crate)`. lib.rs has no `pub use presets::{...}` line. Doc tests for all 5 methods pass. |
| 4 | ThemeGeometry has radius_lg and shadow fields, and presets include values for them | VERIFIED | `native-theme/src/model/geometry.rs` has `radius_lg: Option<f32>` and `shadow: Option<bool>`. All 17 presets have 2 occurrences each of `radius_lg` and `shadow = true` (34 matches each across all TOML files). Integration test `all_presets_have_geometry` asserts both fields. |
| 5 | All existing tests pass against the new API (updated as needed) | VERIFIED | `cargo test -p native-theme`: 93 unit tests + 11 merge tests + 12 preset loading tests + 7 serde roundtrip tests + 9 doc tests = 132 passed, 0 failed, 3 ignored. |

**Score:** 5/5 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `native-theme/src/model/colors.rs` | Flat ThemeColors with 36 Option<Rgba> fields | VERIFIED | 36 fields, impl_merge with option{}, no sub-structs |
| `native-theme/src/model/geometry.rs` | ThemeGeometry with radius_lg and shadow | VERIFIED | 7 fields total, includes radius_lg: Option<f32> and shadow: Option<bool> |
| `native-theme/src/model/mod.rs` | impl NativeTheme with 5 associated methods | VERIFIED | preset, from_toml, from_file, list_presets, to_toml all present with doc examples |
| `native-theme/src/presets.rs` | TOML constants and pub(crate) functions | VERIFIED | 17 TOML constants, PRESET_NAMES array, 5 pub(crate) functions |
| `native-theme/src/lib.rs` | No free function re-exports | VERIFIED | Only re-exports types (NativeTheme, ThemeColors, etc.), no preset function re-exports |
| `native-theme/src/kde/colors.rs` | KDE reader with flat field access | VERIFIED | ThemeColors constructed with all 36 flat fields directly |
| `native-theme/src/gnome/mod.rs` | GNOME reader using NativeTheme::preset and flat field access | VERIFIED | Uses NativeTheme::preset("adwaita"), flat colors.accent/selection/focus_ring/primary_background access |
| `native-theme/src/windows.rs` | Windows reader with flat field access | VERIFIED | Flat colors.accent/foreground/background/selection/focus_ring/primary_background access |
| `native-theme/src/presets/default.toml` | Flat color format with radius_lg and shadow | VERIFIED | [light.colors]/[dark.colors] sections, radius_lg = 12.0, shadow = true |
| All 17 preset TOML files | Flat format with geometry extensions | VERIFIED | All 17 files use flat color tables, all have radius_lg and shadow in both variants |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `model/colors.rs` | `presets/*.toml` | serde field names match TOML keys | WIRED | Field names (accent, primary_background, etc.) match TOML keys exactly; all 17 presets parse successfully |
| `model/mod.rs` | `presets.rs` | crate::presets:: delegation | WIRED | All 5 methods call crate::presets::preset/from_toml/from_file/list_presets/to_toml |
| `lib.rs` | `model/mod.rs` | NativeTheme re-export | WIRED | `pub use model::NativeTheme` -- methods come with the type |
| `gnome/mod.rs` | `model/mod.rs` | NativeTheme::preset call | WIRED | `crate::NativeTheme::preset("adwaita")` in from_gnome and test helper |
| `kde/colors.rs` | `model/colors.rs` | ThemeColors flat construction | WIRED | Direct `ThemeColors { accent: ..., background: ..., ... }` construction |
| `model/geometry.rs` | `presets/*.toml` | serde field names | WIRED | radius_lg and shadow fields match TOML keys; round-trip tests pass |
| `tests/preset_loading.rs` | `model/geometry.rs` | geometry field assertions | WIRED | Tests assert radius_lg.is_some() and shadow.is_some() for all presets |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|-----------|-------------|--------|----------|
| API-02 | 10-01 | ThemeColors flattened to 36 direct Option<Rgba> fields | SATISFIED | 36 fields in colors.rs, no sub-structs |
| API-03 | 10-01 | All presets updated to flat TOML format | SATISFIED | 17 files with flat [light.colors]/[dark.colors] |
| API-04 | 10-01 | Platform readers updated for flat field access | SATISFIED | KDE, GNOME, Windows readers all use flat fields |
| API-05 | 10-02 | Preset functions moved to impl NativeTheme | SATISFIED | 5 associated methods in model/mod.rs |
| API-06 | 10-02 | Old free functions removed | SATISFIED | No pub use presets::{...} exports |
| API-07 | 10-03 | ThemeGeometry gains radius_lg and shadow | SATISFIED | Both fields present in geometry.rs |
| API-08 | 10-03 | Presets updated with radius_lg and shadow data | SATISFIED | All 17 presets have both fields in both variants |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| (none) | - | - | - | No anti-patterns found |

No TODOs, FIXMEs, placeholders, or stub implementations found in any modified files.

### Human Verification Required

None required. All success criteria are verifiable through automated tests (compilation, test suite, grep for old patterns). The phase is purely a structural refactoring with no visual or runtime behavior changes that would need human testing.

### Gaps Summary

No gaps found. All 5 success criteria are fully satisfied:

1. ThemeColors is a flat struct with exactly 36 Option<Rgba> fields and zero nested sub-structs.
2. All 17 preset TOML files use the flat color format and load correctly with round-trip fidelity.
3. All 5 NativeTheme associated methods work (preset, from_toml, from_file, list_presets, to_toml), and old free function exports are removed.
4. ThemeGeometry has 7 fields including radius_lg and shadow, with all presets populated.
5. Full test suite passes: 132 tests passed, 0 failed.

---

_Verified: 2026-03-08T06:15:00Z_
_Verifier: Claude (gsd-verifier)_
