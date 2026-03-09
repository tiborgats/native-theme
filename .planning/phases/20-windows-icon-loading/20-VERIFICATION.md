---
phase: 20-windows-icon-loading
verified: 2026-03-09T09:47:11Z
status: passed
score: 4/4 must-haves verified
---

# Phase 20: Windows Icon Loading Verification Report

**Phase Goal:** Windows users get native stock icons and Segoe Fluent Icons font glyphs as RGBA pixels
**Verified:** 2026-03-09T09:47:11Z
**Status:** passed
**Re-verification:** No -- initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | load_windows_icon(role) returns IconData::Rgba for stock icon roles (SIID_ prefixed names) with correct RGBA byte order and straight alpha | VERIFIED | `load_stock_icon` calls `SHGetStockIconInfo` (line 199), `hicon_to_rgba` applies `bgra_to_rgba` + `unpremultiply_alpha` (lines 177-178), returns `IconData::Rgba` (line 180); unit tests `bgra_to_rgba_conversion` and `unpremultiply_correctness` pass |
| 2 | load_windows_icon(role) returns IconData::Rgba for font glyph roles (22 Segoe Fluent Icons codepoints) with white foreground and scaled alpha | VERIFIED | `glyph_codepoint` maps 22 names to PUA codepoints (lines 32-61), `load_glyph_icon` calls `GetGlyphOutlineW(GGO_GRAY8_BITMAP)` (lines 325, 336), `gray8_to_rgba` produces white (255,255,255) with scaled alpha (lines 117-120); unit test `gray8_alpha_scaling` passes; DWORD-aligned pitch calculated (line 358) |
| 3 | When Segoe Fluent Icons font is absent, falls back to Segoe MDL2 Assets; when neither present, falls back to bundled Material SVGs | VERIFIED | `try_create_font` with `GetTextFaceW` verification detects font substitution (lines 256-271); fallback chain: `Segoe Fluent Icons` -> `Segoe MDL2 Assets` via `.or_else()` (lines 289-290); final fallback `bundled_icon_svg(IconSet::Material, role)` (line 411); `fallback_for_unmapped_role` test passes |
| 4 | DialogQuestion loads via LoadIconW(None, IDI_QUESTION) since there is no SIID_QUESTION constant | VERIFIED | `load_idi_icon` calls `LoadIconW(None, IDI_QUESTION)` (line 219), dispatched via `name == "IDI_QUESTION"` check (line 399), no `DestroyIcon` call for shared system resource (documented at line 221); Windows-only test `idi_question_returns_some` present |

**Score:** 4/4 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `native-theme/src/winicons.rs` | Windows icon loader with stock icon and font glyph pipelines, min 150 lines, exports `load_windows_icon` | VERIFIED | 530 lines, exports `pub fn load_windows_icon`, contains both SHGetStockIconInfo and GetGlyphOutlineW pipelines |
| `native-theme/Cargo.toml` | Win32_UI_Shell feature added to windows crate dependency | VERIFIED | `"Win32_UI_Shell"` present at line 33 in windows features list |
| `native-theme/src/lib.rs` | cfg-gated winicons module declaration and re-export | VERIFIED | `pub mod winicons` at line 101 and `pub use winicons::load_windows_icon` at line 118, both with `cfg(all(target_os = "windows", feature = "system-icons"))` |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `winicons.rs` | `crate::icon_name, crate::bundled_icon_svg` | use crate imports for dispatch and fallback | WIRED | `use crate::{bundled_icon_svg, icon_name, IconData, IconRole, IconSet}` at line 11; `icon_name` used at line 394, `bundled_icon_svg` at line 411 |
| `winicons.rs` | `windows::Win32::UI::Shell::SHGetStockIconInfo` | stock icon loading pipeline | WIRED | Import at line 18, called at line 199 with `SHGSI_ICON | SHGSI_LARGEICON` flags |
| `winicons.rs` | `windows::Win32::Graphics::Gdi::GetGlyphOutlineW` | font glyph rendering pipeline | WIRED | Import at line 16, called at lines 325 and 336 with `GGO_GRAY8_BITMAP` |
| `lib.rs` | `winicons.rs` | cfg-gated pub mod + pub use | WIRED | `pub mod winicons` at line 101, `pub use winicons::load_windows_icon` at line 118, both with matching cfg gate |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| PLAT-02 | 20-01 | Windows stock icon loading via SHGetStockIconInfo -> RGBA pixels | SATISFIED | `load_stock_icon` function with `siid_from_name` dispatch table (16 SIID_ constants), `hicon_to_rgba` extraction, BGRA-to-RGBA conversion, unpremultiply |
| PLAT-03 | 20-01 | Windows Segoe Fluent Icons font glyph rendering for action/navigation/window roles | SATISFIED | `load_glyph_icon` function with `glyph_codepoint` table (22 entries), `GetGlyphOutlineW(GGO_GRAY8_BITMAP)` rendering, gray8-to-RGBA conversion, font fallback chain |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| (none) | - | - | - | No anti-patterns detected |

No TODO/FIXME/HACK comments, no placeholder implementations, no empty returns, no console.log stubs.

### Human Verification Required

### 1. Stock Icon Visual Correctness on Windows

**Test:** Run on a Windows machine with `system-icons` + `windows` features enabled, call `load_windows_icon(IconRole::DialogWarning)` and inspect the returned RGBA buffer visually (e.g., write to PNG).
**Expected:** The icon should show the standard Windows warning triangle with correct colors (yellow/orange, not blue-shifted) and clean semi-transparent edges.
**Why human:** BGRA-to-RGBA byte swap correctness and unpremultiply quality can only be verified visually. We verified the algorithms with unit tests, but actual system icon rendering on Windows hardware cannot be tested on Linux.

### 2. Font Glyph Rendering Quality on Windows

**Test:** Run on Windows, call `load_windows_icon(IconRole::ActionCopy)` and inspect the rendered glyph.
**Expected:** The glyph should render as a white icon with smooth anti-aliased edges on transparent background, matching the Segoe Fluent Icons copy glyph.
**Why human:** Font rendering quality, glyph alignment, and alpha smoothness require visual inspection on actual Windows hardware.

### 3. Font Fallback Chain on Windows 10

**Test:** Run on a Windows 10 machine that does not have Segoe Fluent Icons installed, call `load_windows_icon(IconRole::WindowClose)`.
**Expected:** Should return `IconData::Rgba` from Segoe MDL2 Assets (not a garbled glyph from font substitution).
**Why human:** Font availability varies across Windows 10 installations. The `GetTextFaceW` verification logic cannot be tested on Linux.

### Gaps Summary

No gaps found. All four observable truths are verified with supporting code evidence. All artifacts exist, are substantive (530 lines, well above 150 minimum), and are properly wired. Both requirements (PLAT-02, PLAT-03) are satisfied. Compilation succeeds, all 181 tests pass, and clippy reports no warnings.

**Deviation noted:** The plan specified adding `dep:windows` to the `system-icons` feature, but the executor correctly chose not to do this due to a transitive dependency version conflict (`windows-future` v0.2.1 vs v0.3.2). This is a reasonable deviation -- the `windows` Cargo feature already activates `dep:windows`, so Windows users enable both `system-icons` and `windows` features. This was documented in the SUMMARY.

**Commits verified:** `f247a1c` (Task 1: winicons.rs + Cargo.toml) and `e8b78ec` (Task 2: lib.rs wiring) both exist with correct content.

---

_Verified: 2026-03-09T09:47:11Z_
_Verifier: Claude (gsd-verifier)_
