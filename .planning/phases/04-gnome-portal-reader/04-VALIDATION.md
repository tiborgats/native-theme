---
phase: 04-gnome-portal-reader
type: validation
created: 2026-03-07
---

# Phase 4: GNOME Portal Reader - Validation Architecture

## Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust built-in test framework (cargo test) |
| Quick run command | `cargo test --features portal-tokio` |
| Full suite command | `cargo test --all-features` |

## Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Automated Command |
|--------|----------|-----------|-------------------|
| PLAT-02a | from_gnome() returns NativeTheme with accent, scheme, contrast | unit | `cargo test --features portal-tokio gnome::tests` |
| PLAT-02b | build_theme with PreferDark produces dark variant only | unit | `cargo test --features portal-tokio gnome::tests::dark_scheme` |
| PLAT-02c | build_theme with PreferLight produces light variant only | unit | `cargo test --features portal-tokio gnome::tests::light_scheme` |
| PLAT-02d | build_theme with NoPreference defaults to light | unit | `cargo test --features portal-tokio gnome::tests::no_preference` |
| PLAT-02e | Out-of-range accent color treated as None (Adwaita default) | unit | `cargo test --features portal-tokio gnome::tests::accent_out_of_range` |
| PLAT-02f | Valid accent color propagates to accent, selection, focus_ring, primary | unit | `cargo test --features portal-tokio gnome::tests::accent_propagation` |
| PLAT-02g | High contrast adjusts border_opacity and disabled_opacity | unit | `cargo test --features portal-tokio gnome::tests::high_contrast` |
| PLAT-02h | portal feature compiles without tokio when using portal-async-io | integration | `cargo check --features portal-async-io --no-default-features` |
| PLAT-02i | All portal values unavailable -> returns Adwaita defaults | unit | `cargo test --features portal-tokio gnome::tests::all_unavailable` |
| PLAT-02j | Feature flag wiring: portal-tokio enables portal + ashpd/tokio | integration | `cargo check --features portal-tokio` |

## Sampling Rate
- **Per task commit:** `cargo test --features portal-tokio`
- **Per wave merge:** `cargo test --all-features`
- **Phase gate:** Full suite green before `/gsd:verify-work`
