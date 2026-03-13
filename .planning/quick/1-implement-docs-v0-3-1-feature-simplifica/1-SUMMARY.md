---
phase: quick
plan: 01
subsystem: native-theme
tags: [feature-flags, cargo, documentation]
dependency_graph:
  requires: []
  provides: [meta-features, target-gated-deps]
  affects: [native-theme/Cargo.toml, README.md]
tech_stack:
  added: []
  patterns: [target-gated-optional-deps, meta-features]
key_files:
  created: []
  modified:
    - native-theme/Cargo.toml
    - native-theme/src/lib.rs
    - native-theme/src/macos.rs
    - README.md
decisions:
  - "Combine target_os + feature in cfg gates so meta-features compile on all platforms"
  - "Keep portal base feature out of README individual features table"
metrics:
  duration: 5min
  completed: "2026-03-13T11:39:45Z"
---

# Quick Task 1: v0.3.1 Feature Flag Simplification Summary

Target-gated all OS-specific deps (ashpd, configparser, windows) and added native/linux meta-features so users can enable full cross-platform support with a single `features = ["native"]` line.

## What Was Done

### Task 1: Target-gate OS-specific deps and add meta-features (a970981)

Moved three dependencies from `[dependencies]` to target-gated sections:
- `ashpd` and `configparser` to `[target.'cfg(target_os = "linux")'.dependencies]`
- `windows` to `[target.'cfg(target_os = "windows")'.dependencies]`

Added four meta-features to `[features]`:
- `linux` = `["kde", "portal-tokio"]`
- `linux-async-io` = `["kde", "portal-async-io"]`
- `native` = `["linux", "macos", "windows"]`
- `native-async-io` = `["linux-async-io", "macos", "windows"]`

### Task 2: Update README feature documentation (66cc423)

Replaced the flat feature table with a structured section:
- Recommended one-liner using `native`
- Meta-features table (native, native-async-io, linux, linux-async-io)
- Individual features table (without the `portal` base feature)
- "Which Linux DEs are supported?" subsection explaining GNOME/XFCE/etc. coverage
- Target-gating benefit explanation

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Updated cfg gates to combine target_os with feature flag**
- **Found during:** Task 1
- **Issue:** Source code used `#[cfg(feature = "macos")]` and `#[cfg(feature = "windows")]` to gate module imports and declarations. After target-gating the dependencies, enabling these features on non-matching platforms (e.g., `native` on Linux activates `macos` feature) caused compilation failures because the crates don't exist on the wrong platform.
- **Fix:** Changed cfg gates to `#[cfg(all(target_os = "...", feature = "..."))]` in lib.rs (module declarations and re-exports for gnome, kde, macos, windows) and macos.rs (all `#[cfg(feature = "macos")]` guards).
- **Files modified:** `native-theme/src/lib.rs`, `native-theme/src/macos.rs`
- **Commit:** a970981

## Verification Results

All checks passed:
1. `cargo check -p native-theme --features native` -- OK
2. `cargo check -p native-theme --features linux` -- OK
3. `cargo check -p native-theme --features native-async-io` -- OK
4. `cargo check -p native-theme --features kde` -- OK (individual feature still works)
5. `cargo check -p native-theme` -- OK (no features)
6. `cargo test -p native-theme --features native` -- 21 passed, 0 failed

## Self-Check: PASSED

All 4 modified files verified on disk. Both task commits (a970981, 66cc423) verified in git log.
