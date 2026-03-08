---
phase: 11-platform-readers
verified: 2026-03-08T14:45:00Z
status: passed
score: 5/5 must-haves verified
re_verification:
  previous_status: gaps_found
  previous_score: 4/5
  gaps_closed:
    - "D-Bus portal backend detection improves DE heuristic accuracy"
    - "All tests pass under default parallel test execution"
  gaps_remaining: []
  regressions: []
---

# Phase 11: Platform Readers Verification Report

**Phase Goal:** Full desktop platform coverage -- macOS reader completes the 4th platform, Windows and Linux readers enhanced with richer data
**Verified:** 2026-03-08T14:45:00Z
**Status:** passed
**Re-verification:** Yes -- after gap closure (plan 11-04)

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | `NativeTheme::from_macos()` returns a theme with semantic colors (P3-to-sRGB converted), fonts, and both light/dark variants resolved via NSAppearance | VERIFIED | macos.rs: 304 lines unchanged from initial verification. `from_macos()` uses `NSAppearance::performAsCurrentDrawingAppearance` for both Aqua and DarkAqua, `nscolor_to_rgba()` converts via `colorUsingColorSpace(sRGB)`, `read_semantic_colors()` maps all 36 ThemeColors fields, `read_fonts()` reads NSFont system and monospace, `build_theme()` populates both light and dark variants. |
| 2 | `NativeTheme::from_system()` dispatches to `from_macos()` on macOS | VERIFIED | lib.rs lines 177-184: `#[cfg(target_os = "macos")]` block with `#[cfg(feature = "macos")] return crate::macos::from_macos()`. Module declared at line 91, re-export at line 96. |
| 3 | `NativeTheme::from_windows()` returns accent shades, system font, spacing, and DPI-aware geometry; capability checks prevent crashes on older Windows | VERIFIED | windows.rs: 574 lines unchanged from initial verification. `read_accent_shades()` with `.ok()` per-shade fallback, `read_system_font()` via `SystemParametersInfoW(SPI_GETNONCLIENTMETRICS)`, `winui3_spacing()` returns 7 WinUI3 values, `read_geometry_dpi_aware()` uses `GetDpiForSystem()` and `GetSystemMetricsForDpi`. 16 tests. |
| 4 | `from_kde_with_portal()` merges portal accent onto kdeglobals palette; GNOME reader populates font data; `from_linux()` provides kdeglobals fallback | VERIFIED | gnome/mod.rs: `from_kde_with_portal()` at line 208 calls `crate::kde::from_kde()` then overlays portal accent via `apply_accent()` and `base.merge(&overlay)`. `read_gnome_fonts()` at line 71 reads gsettings font-name and monospace-font-name. `parse_gnome_font_string()` at line 49 with 9 tests. lib.rs `from_linux()` at lines 144-153 checks `kdeglobals_path().exists()` for Unknown DE. |
| 5 | D-Bus portal backend detection improves DE heuristic accuracy | VERIFIED | **Gap closed.** `detect_portal_backend()` at gnome/mod.rs line 253 is now called from `from_system_async()` at lib.rs line 236: `if let Some(detected) = crate::gnome::detect_portal_backend().await`. Called within the `LinuxDesktop::Unknown` match arm under `#[cfg(feature = "portal")]`, dispatching to `from_kde_with_portal()` or `from_gnome()` based on detected backend. `cargo check --features kde,portal-tokio` produces zero dead_code warnings. Non-Linux fallback at lines 263-266 delegates to sync `from_system()`. |

**Score:** 5/5 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `native-theme/src/macos.rs` | macOS reader with from_macos(), build_theme(), nscolor_to_rgba(), read_fonts() | VERIFIED | 304 lines, unchanged (regression check) |
| `native-theme/Cargo.toml` | macos feature with objc2/block2 deps + Win32_UI_HiDpi for Windows | VERIFIED | Unchanged (regression check) |
| `native-theme/src/lib.rs` | macos module, from_system() dispatch, from_linux() kdeglobals fallback, from_system_async() with portal detection, ENV_MUTEX | VERIFIED | 418 lines. from_system_async() at line 216 calls detect_portal_backend(). ENV_MUTEX at line 272. 4 env-var tests guarded. |
| `native-theme/src/windows.rs` | Enhanced Windows reader with accent shades, font, spacing, DPI geometry | VERIFIED | 574 lines, unchanged (regression check) |
| `native-theme/src/gnome/mod.rs` | from_kde_with_portal(), read_gnome_fonts(), parse_gnome_font_string(), detect_portal_backend() | VERIFIED | 500 lines. detect_portal_backend() now called from lib.rs from_system_async(). No dead_code warnings. |
| `native-theme/src/kde/mod.rs` | pub(crate) from_kde_content(), ENV_MUTEX guarded tests | VERIFIED | 434 lines. 3 env-var tests guarded with ENV_MUTEX at lines 172, 182, 413. |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| lib.rs from_system_async() | gnome/mod.rs detect_portal_backend() | `crate::gnome::detect_portal_backend().await` | WIRED | Line 236 of lib.rs calls detect_portal_backend() in Unknown DE path under portal feature |
| lib.rs tests | ENV_MUTEX | `crate::ENV_MUTEX.lock().unwrap()` | WIRED | 4 tests in dispatch_tests acquire lock: lines 324, 341, 379, 407 |
| kde/mod.rs tests | ENV_MUTEX | `crate::ENV_MUTEX.lock().unwrap()` | WIRED | 3 tests acquire lock: lines 172, 182, 413 |
| macos.rs | ThemeColors 36 fields | direct field assignment in read_semantic_colors() | WIRED | Unchanged (regression check) |
| lib.rs | macos.rs | from_system() dispatch for cfg(target_os=macos) | WIRED | `crate::macos::from_macos()` at line 180 |
| windows.rs | UIColorType accent shades | read_accent_shades() with .ok() | WIRED | Unchanged (regression check) |
| gnome/mod.rs | kde/mod.rs | from_kde_with_portal() calls kde::from_kde() | WIRED | `crate::kde::from_kde()?` at line 209 |
| lib.rs | kde/mod.rs | from_linux() fallback | WIRED | `crate::kde::from_kde()` at line 149, gated by `kdeglobals_path().exists()` |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|-------------|------------|-------------|--------|----------|
| PLAT-01 | 11-01 | macOS reader reads ~20 NSColor semantic colors with P3-to-sRGB | SATISFIED | Unchanged |
| PLAT-02 | 11-01 | macOS reader resolves both light and dark variants via NSAppearance | SATISFIED | Unchanged |
| PLAT-03 | 11-01 | macOS reader reads NSFont system and monospace fonts | SATISFIED | Unchanged |
| PLAT-04 | 11-01 | macOS reader wired into from_system() dispatch | SATISFIED | Unchanged |
| PLAT-05 | 11-02 | Windows capability checks prevent crashes on older versions | SATISFIED | Unchanged |
| PLAT-06 | 11-02 | Windows reads AccentDark1-3 and AccentLight1-3 | SATISFIED | Unchanged |
| PLAT-07 | 11-02 | Windows reads system font via NONCLIENTMETRICSW | SATISFIED | Unchanged |
| PLAT-08 | 11-02 | Windows populates WinUI3 spacing and primary_foreground | SATISFIED | Unchanged |
| PLAT-09 | 11-02 | Windows uses DPI-aware GetSystemMetricsForDpi | SATISFIED | Unchanged |
| PLAT-10 | 11-03 | from_kde_with_portal() async overlay of portal accent on kdeglobals | SATISFIED | Unchanged |
| PLAT-11 | 11-03, 11-04 | D-Bus portal backend detection for DE heuristic | SATISFIED | **Gap closed.** detect_portal_backend() called from from_system_async() line 236. Zero dead_code warnings. |
| PLAT-12 | 11-03 | GNOME font reading from gsettings | SATISFIED | Unchanged |
| PLAT-13 | 11-03 | from_linux() fallback tries kdeglobals on non-KDE desktops | SATISFIED | Unchanged |

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| (none) | - | - | - | Both previous anti-patterns resolved: dead_code warning eliminated, test race condition fixed |

### Human Verification Required

### 1. macOS Reader Functional Test

**Test:** On a macOS machine, run `cargo test -p native-theme --features macos` and call `from_macos()` to verify live NSColor/NSAppearance reading.
**Expected:** Both light and dark variants populated with non-default color values, font family like "SF Pro" or ".AppleSystemUIFont".
**Why human:** macOS-specific code cannot be tested on Linux; requires macOS hardware or CI.

### 2. Windows Reader Functional Test

**Test:** On a Windows machine, run `cargo test -p native-theme --features windows` and call `from_windows()` to verify live UISettings/SystemParametersInfoW/GetSystemMetricsForDpi reading.
**Expected:** Accent shades populated (at least AccentDark1 and AccentLight1 on Windows 10+), system font "Segoe UI" at a reasonable size, DPI-aware geometry values > 0, WinUI3 spacing populated.
**Why human:** Windows-specific code cannot be tested on Linux; requires Windows hardware or CI.

### 3. D-Bus Portal Backend Detection (Live)

**Test:** On a Linux system with xdg-desktop-portal running, call `from_system_async()` with `XDG_CURRENT_DESKTOP` unset to exercise the portal detection path.
**Expected:** Returns a KDE or GNOME theme based on the running portal backend, rather than falling through to kdeglobals/Adwaita.
**Why human:** Requires running D-Bus session with portal backend installed.

### Gap Closure Summary

Both gaps from the initial verification are now closed:

1. **detect_portal_backend() is no longer dead code.** It is called from the new `from_system_async()` function at lib.rs line 236 within the `LinuxDesktop::Unknown` match arm when the `portal` feature is enabled. `cargo check --features kde,portal-tokio` produces zero dead_code warnings. PLAT-11 is now fully satisfied.

2. **Env var test race conditions are fixed.** `ENV_MUTEX` is defined at lib.rs line 272 as `pub(crate) static` under `#[cfg(test)]`. All 7 tests that manipulate environment variables (4 in lib.rs, 3 in kde/mod.rs) acquire the mutex lock before any `set_var`/`remove_var` calls. All 166 tests pass with default parallel execution (`cargo test -p native-theme --features kde,portal-tokio` -- zero failures, no `--test-threads=1` needed).

No regressions detected in previously-verified truths (1-4). All artifacts remain at expected sizes and all key links are intact.

---

_Verified: 2026-03-08T14:45:00Z_
_Verifier: Claude (gsd-verifier)_
