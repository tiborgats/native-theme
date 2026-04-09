---
phase: 67-macos-windows-watchers
verified: 2026-04-09T21:40:00Z
status: human_needed
score: 4/4 must-haves verified
human_verification:
  - test: "Toggle Appearance (Light/Dark) in macOS System Settings while a test binary with on_theme_change() running prints to stdout"
    expected: "ThemeChangeEvent::ColorSchemeChanged is received and printed within ~1 second of toggle"
    why_human: "Cannot run macOS code on the Linux build machine; NSDistributedNotificationCenter delivery and CFRunLoop wakeup require a real macOS host"
  - test: "Change system theme in Windows Settings while a test binary with on_theme_change() running prints to stdout"
    expected: "ThemeChangeEvent::ColorSchemeChanged is received and printed within ~1 second of toggle"
    why_human: "Cannot run Windows code on the Linux build machine; UISettings::ColorValuesChanged and COM STA dispatch require a real Windows host"
  - test: "Drop the ThemeWatcher on macOS and observe the watcher thread exits cleanly (no hang, no panic)"
    expected: "handle.join() returns without blocking after drop; thread is not left running"
    why_human: "Requires macOS host to exercise CFRunLoop::stop() path"
  - test: "Drop the ThemeWatcher on Windows and observe the watcher thread exits cleanly"
    expected: "PostThreadMessageW(WM_QUIT) causes GetMessageW to return FALSE and the thread exits; join() returns promptly"
    why_human: "Requires Windows host to exercise PostThreadMessageW/WM_QUIT path"
---

# Phase 67: macOS and Windows Watchers Verification Report

**Phase Goal:** Theme changes on macOS and Windows trigger ThemeChanged signals through the watcher API
**Verified:** 2026-04-09T21:40:00Z
**Status:** human_needed
**Re-verification:** No - initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | On macOS, toggling Appearance triggers ThemeChanged via NSDistributedNotificationCenter | ✓ VERIFIED (code) / ? HUMAN (runtime) | `watch/macos.rs:89` registers `AppleInterfaceThemeChangedNotification` observer; `CFRunLoop::run()` on background thread; wired to `on_theme_change()` at `mod.rs:208` |
| 2 | On Windows, changing system theme triggers ThemeChanged via UISettings::ColorValuesChanged with COM STA and message pump | ✓ VERIFIED (code) / ? HUMAN (runtime) | `watch/windows.rs:55` CoInitializeEx STA; `windows.rs:79` ColorValuesChanged subscription; `windows.rs:96` GetMessageW/DispatchMessageW pump; wired at `mod.rs:223` |
| 3 | Both watchers run on dedicated background threads without requiring caller event loop | ✓ VERIFIED | Both `watch_macos()` and `watch_windows()` call `std::thread::spawn()` immediately; CFRunLoop::run() / GetMessageW pump runs entirely inside the spawned thread |
| 4 | Dropping ThemeWatcher cleanly stops the background thread on both platforms | ✓ VERIFIED (code) / ? HUMAN (runtime) | `Drop::drop()` calls `platform_shutdown.take()()` first (CFRunLoop::stop / PostThreadMessageW WM_QUIT), then drops channel, then `thread.join()`; ordering at `mod.rs:149-157` |

**Score:** 4/4 truths verified at code level; runtime behavior on macOS/Windows requires human validation

### Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| `native-theme/Cargo.toml` | Feature flags for macOS and Windows watcher dependencies | ✓ VERIFIED | NSDistributedNotificationCenter, NSNotification, NSRunLoop, NSOperation, block2 (objc2-foundation); CFRunLoop, CFDate (objc2-core-foundation); Win32_System_Com, Win32_System_Threading (windows) — all present at lines 47-104 |
| `native-theme/src/watch/macos.rs` | macOS watcher backend using CFRunLoop + NSDistributedNotificationCenter, min 40 lines | ✓ VERIFIED | File exists, 143 lines, contains `watch_macos()`, `SendableCFRunLoop`, full observer registration and CFRunLoop::run() |
| `native-theme/src/watch/windows.rs` | Windows watcher backend using UISettings::ColorValuesChanged with COM STA message pump, min 50 lines | ✓ VERIFIED | File exists, 131 lines, contains `watch_windows()`, COM STA init, GetMessageW/DispatchMessageW pump, PostThreadMessageW shutdown |
| `native-theme/src/watch/mod.rs` | ThemeWatcher with platform shutdown handle, macOS and Windows dispatch arms | ✓ VERIFIED | `platform_shutdown: Option<Box<dyn FnOnce() + Send>>` field present; `with_platform_shutdown()` constructor; `#[cfg(target_os = "macos")]` and `#[cfg(target_os = "windows")]` dispatch arms; both call the respective backends |

### Key Link Verification

| From | To | Via | Status | Details |
|------|----|-----|--------|---------|
| `watch/mod.rs` | `watch/macos.rs` | `on_theme_change()` dispatches to `macos::watch_macos()` | ✓ WIRED | `mod.rs:208`: `return macos::watch_macos(callback)` inside `#[cfg(target_os = "macos")]` + `#[cfg(all(feature = "watch", feature = "macos"))]` |
| `watch/macos.rs` | `watch/mod.rs` | Returns `ThemeWatcher::with_platform_shutdown` | ✓ WIRED | `macos.rs:138-142`: `Ok(super::ThemeWatcher::with_platform_shutdown(...))` |
| `watch/mod.rs` | `watch/windows.rs` | `on_theme_change()` dispatches to `windows::watch_windows()` | ✓ WIRED | `mod.rs:223`: `return windows::watch_windows(callback)` inside `#[cfg(target_os = "windows")]` + `#[cfg(all(feature = "watch", feature = "windows"))]` |
| `watch/windows.rs` | `watch/mod.rs` | Returns `ThemeWatcher::with_platform_shutdown` | ✓ WIRED | `windows.rs:126-130`: `Ok(super::ThemeWatcher::with_platform_shutdown(...))` |

### Data-Flow Trace (Level 4)

Not applicable: these are event-driven watcher modules that register OS callbacks. There is no state variable rendering dynamic data — data flow is callback invocation, not fetch/render.

### Behavioral Spot-Checks

| Behavior | Command | Result | Status |
|----------|---------|--------|--------|
| `cargo check --features watch,kde,portal-tokio` compiles on Linux | `cargo check --manifest-path native-theme/Cargo.toml --features watch,kde,portal-tokio` | `Finished dev profile` (0 errors) | ✓ PASS |
| `cargo test --features watch,kde,portal-tokio` passes on Linux (excl. pre-existing failure) | `cargo test --manifest-path native-theme/Cargo.toml --features watch,kde,portal-tokio` | 621 passed, 1 pre-existing failure (`gnome::tests::build_gnome_variant_normal_contrast_no_flag`) | ✓ PASS (pre-existing failure predates phase 67; phase 67 touched no gnome code) |
| All three phase commits exist in git history | `git log --oneline` | 7fe5c2e, 0368d2d, 290e4d2 present | ✓ PASS |
| macOS dispatch arm calls `macos::watch_macos` | grep in `mod.rs` | Found at line 208 | ✓ PASS |
| Windows dispatch arm calls `windows::watch_windows` | grep in `mod.rs` | Found at line 223 | ✓ PASS |
| Drop calls `platform_shutdown` before `shutdown_tx` drop | inspect `mod.rs:144-159` | `platform_shutdown.take()()` at line 149-151, `shutdown_tx.take()` at line 153 | ✓ PASS |

### Requirements Coverage

No explicit requirement IDs were declared in phase 67 plans. The phase delivers new capability additions (macOS and Windows watcher backends) rather than fulfilling tracked requirements.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| (none) | — | — | — | — |

No TODOs, FIXMEs, placeholders, empty returns, or stub patterns found in either new watcher file.

### Human Verification Required

#### 1. macOS: Notification center fires ThemeChanged on appearance toggle

**Test:** On a macOS machine, build with `cargo build --features watch,macos` and run a small binary that calls `on_theme_change(|e| println!("{e:?}"))`, keeps the watcher alive, then toggle System Settings > Appearance between Light and Dark.
**Expected:** `ColorSchemeChanged` is printed within ~1 second of each toggle.
**Why human:** NSDistributedNotificationCenter delivery and CFRunLoop wakeup require a real macOS host; cannot be exercised on the Linux build machine.

#### 2. Windows: UISettings fires ThemeChanged on system theme change

**Test:** On a Windows machine, build with `cargo build --features watch,windows` and run a small binary that calls `on_theme_change(|e| println!("{e:?}"))`, keeps the watcher alive, then change Settings > Personalization > Colors between Light and Dark.
**Expected:** `ColorSchemeChanged` is printed within ~1 second of each toggle.
**Why human:** UISettings::ColorValuesChanged and COM STA dispatch require a real Windows host.

#### 3. macOS: Watcher thread exits cleanly on Drop

**Test:** On macOS, run the test binary and drop the ThemeWatcher (let it go out of scope or press Ctrl+C). Observe no hang and no crash.
**Expected:** Process exits promptly; `CFRunLoop::stop()` unblocks `CFRunLoop::run()` and `thread.join()` returns.
**Why human:** CFRunLoop::stop cross-thread path requires macOS host.

#### 4. Windows: Watcher thread exits cleanly on Drop

**Test:** On Windows, run the test binary and drop the ThemeWatcher. Observe no hang and no crash.
**Expected:** Process exits promptly; `PostThreadMessageW(WM_QUIT)` causes `GetMessageW` to return FALSE and the pump loop exits; `thread.join()` returns.
**Why human:** Win32 message pump WM_QUIT path requires a Windows host.

### Gaps Summary

No code-level gaps found. All four success criteria from the ROADMAP are implemented:

1. NSDistributedNotificationCenter for AppleInterfaceThemeChangedNotification — present and wired in `watch/macos.rs`
2. UISettings::ColorValuesChanged with COM STA and message pump — present and wired in `watch/windows.rs`
3. Dedicated background threads, no caller event loop required — both backends use `std::thread::spawn()` with self-contained loops
4. Drop cleanly stops background thread — `platform_shutdown` called before channel drop, thread joined in `Drop::drop()`

Runtime verification on actual macOS and Windows hardware is the only remaining step before declaring full goal achievement.

---

_Verified: 2026-04-09T21:40:00Z_
_Verifier: Claude (gsd-verifier)_
