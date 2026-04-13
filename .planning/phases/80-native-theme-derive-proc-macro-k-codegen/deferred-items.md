# Deferred Items - Phase 80

## Pre-existing Issues (Out of Scope)

1. **gnome/mod.rs:279 `build_gnome_spec_pure` dead_code warning** -- This function is unused when compiling with connector features (native-theme-gpui, native-theme-iced). Pre-dates Phase 80. Causes clippy -D warnings failure on connector crates.
