# Deferred Items - Phase 64

## Pre-existing Test Failure

**File:** `native-theme/src/gnome/mod.rs`
**Test:** `gnome::tests::build_gnome_variant_normal_contrast_no_flag`
**Issue:** Test asserts `high_contrast.is_none()` after calling `build_gnome_variant(NoPreference, None, NoPreference, None)`, but on machines where gsettings returns "false" for `org.gnome.desktop.a11y.interface high-contrast`, the gsettings fallback sets `high_contrast = Some(false)`. This is correct production behavior but makes the test environment-dependent.
**Scope:** Pre-existing (verified on original code before Phase 64 changes). Not caused by this phase.
**Suggested fix:** Change assertion to `assert!(!matches!(v.defaults.high_contrast, Some(true)))` or mock gsettings in tests.
