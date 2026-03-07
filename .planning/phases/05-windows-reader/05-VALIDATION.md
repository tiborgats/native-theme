---
phase: 05-windows-reader
type: validation
---

# Phase 5: Windows Reader - Validation Architecture

## Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust built-in test framework (cargo test) |
| Config file | Cargo.toml (existing) |
| Quick run command | `cargo check --target x86_64-pc-windows-gnu --features windows` |
| Full suite command | `cargo test --all-features` |

## Phase Requirements -> Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| PLAT-04 | from_windows() returns NativeTheme with accent, fg, bg, geometry | unit | `cargo check --target x86_64-pc-windows-gnu --features windows` | No -- Wave 0 |
| PLAT-04 | Dark mode detection from foreground luminance | unit | `cargo check --target x86_64-pc-windows-gnu --features windows` | No -- Wave 0 |
| PLAT-04 | Graceful degradation: UISettings unavailable returns Error::Unavailable | unit | `cargo check --target x86_64-pc-windows-gnu --features windows` | No -- Wave 0 |
| PLAT-04 | Only active variant (light or dark) populated | unit | `cargo check --target x86_64-pc-windows-gnu --features windows` | No -- Wave 0 |
| PLAT-04 | Feature flag isolation: "windows" feature only pulls minimal crate features | compile | `cargo check --target x86_64-pc-windows-gnu --features windows` | No -- Wave 0 |

Note: Unit tests are inside `#[cfg(feature = "windows")]` module and can only execute on Windows.
On Linux, compilation checks via cross-compilation (`--target x86_64-pc-windows-gnu`) verify correctness.

## Sampling Rate

- **Per task commit:** `cargo check --target x86_64-pc-windows-gnu --features windows` (cross-compile check on Linux)
- **Per wave merge:** `cargo test --all-features` (existing tests) + cross-compile check
- **Phase gate:** Full suite green before `/gsd:verify-work`

## Wave 0 Gaps

- [ ] `src/windows.rs` -- the entire module (new file)
- [ ] Unit tests for `is_dark_mode()`, `win_color_to_rgba()`, `build_theme()` testable core
- [ ] Integration test for `from_windows()` (requires Windows environment)
