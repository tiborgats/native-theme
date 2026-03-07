# Phase 3: KDE Reader - Validation Architecture

**Phase:** 03-kde-reader
**Source:** 03-RESEARCH.md Validation Architecture section

## Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust built-in test (cargo test) |
| Quick run | `cargo test --features kde` |
| Full suite | `cargo test --all-features` |

## Phase Requirements -> Test Map

| Req ID | Behavior | Test Type | Plan | Task | Automated Command |
|--------|----------|-----------|------|------|-------------------|
| PLAT-01a | from_kde() parses color groups into semantic roles | unit | 03-02 | 1 | `cargo test --features kde kde::colors` |
| PLAT-01b | from_kde() handles missing file gracefully | unit | 03-02 | 2 | `cargo test --features kde kde::tests::missing_file` |
| PLAT-01c | from_kde() handles missing sections gracefully | unit | 03-02 | 1,2 | `cargo test --features kde kde::colors::tests::test_partial` |
| PLAT-01d | from_kde() handles malformed color values | unit | 03-02 | 1 | `cargo test --features kde kde::colors::tests::test_malformed` |
| PLAT-01e | Qt4 (10-field) font string parses correctly | unit | 03-01 | 2 | `cargo test --features kde kde::fonts::tests::test_qt4` |
| PLAT-01f | Qt5/6 (16-field) font string parses correctly | unit | 03-01 | 2 | `cargo test --features kde kde::fonts::tests::test_qt5` |
| PLAT-01g | Feature flag compiles correctly (on/off) | build | 03-01 | 1 | `cargo check && cargo check --features kde` |
| PLAT-01h | Dark/light detection from background luminance | unit | 03-01 | 1 | `cargo test --features kde kde::tests::dark_light` |

## Sampling Rate

- **Per task commit:** `cargo test --features kde`
- **Per wave merge:** `cargo test --all-features`
- **Phase gate:** Full suite green before `/gsd:verify-work`

## Wave 0 Gaps

- [ ] `src/kde/mod.rs` -- module with from_kde(), helpers
- [ ] `src/kde/colors.rs` -- color parsing and 36-role mapping
- [ ] `src/kde/fonts.rs` -- Qt font string parser
- [ ] Cargo.toml feature flag: `kde = ["dep:configparser"]`
- [ ] Test fixtures: embedded kdeglobals strings (dark, light, minimal, malformed)
