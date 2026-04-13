# Deferred Items -- Phase 81

## Pre-existing dead_code warning: build_gnome_spec_pure

- **File:** `native-theme/src/gnome/mod.rs:279`
- **Issue:** `build_gnome_spec_pure` is flagged as unused by `dead_code` lint when building with `-D warnings` via connector crates (gpui, iced) which activate the `linux` feature
- **Impact:** pre-release-check.sh reports failure on connector clippy steps
- **Origin:** Pre-existing before Phase 81 -- noted in 81-01-SUMMARY.md
- **Resolution:** Needs `#[expect(dead_code)]` annotation or actual usage wired up; belongs in gnome module cleanup, not feature-matrix plan
