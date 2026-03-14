# Phase 01: v0.3.2 Quality Improvements - Validation

## Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in (`#[test]`, cargo test) |
| Config file | Cargo workspace (Cargo.toml) |
| Quick run command | `cargo test -p native-theme` |
| Full suite command | `cargo test --workspace` |

## Phase Requirements to Test Map
| Issue | Behavior | Test Type | Automated Command | Coverage |
|-------|----------|-----------|-------------------|----------|
| 1 - Caching | system_icon_theme() returns cached value | unit | `cargo test -p native-theme --lib` | Existing tests; caching is transparent |
| 1 - Caching | system_is_dark() returns cached value | unit | `cargo test -p native-theme --lib` | Transparent behavior |
| 2 - pick_variant | NativeTheme::pick_variant works correctly | unit | `cargo test -p native-theme pick_variant` | New tests (5 behaviors) |
| 2 - pick_variant | Deprecated free functions still compile | unit | `cargo test -p native-theme-gpui --lib && cargo test -p native-theme-iced --lib` | Existing tests |
| 3 - colorize_svg | Renamed function compiles and works | unit | `cargo test -p native-theme-iced colorize` | Existing tests with updated names |
| 4 - Dead wrappers | active_color uses direct trait calls | unit | `cargo test -p native-theme-gpui active_color` | Existing tests |
| 5 - to_theme | to_theme produces valid theme | unit | `cargo test -p native-theme-gpui to_theme` | Existing test |
| 6 - #[must_use] | Compilation succeeds with no new warnings | build | `cargo check --workspace` | Compile-time |
| 7 - Script | Script runs without python3 | manual | `bash -n pre-release-check.sh` | Syntax validation |

## Sampling Rate
- **Per task commit:** `cargo test -p <affected-crate> --lib`
- **Per wave merge:** `cargo test --workspace`
- **Phase gate:** `cargo test --workspace && cargo clippy --workspace -- -D warnings`

## Wave 0 Gaps
- [ ] Add tests for `NativeTheme::pick_variant()` (5 behaviors: dark pref, light pref, dark fallback, light fallback, empty)
- [ ] Verify `#[must_use]` doesn't cause warnings in existing test code
