# Deferred Items - Phase 65

## Pre-existing Issues (Out of Scope)

1. **`gsettings_get` dead_code warning in detect.rs:680** - `pub(crate) fn gsettings_get` is never used, causing clippy `-D warnings` to fail in pre-release-check.sh. This is pre-existing (confirmed by testing on clean HEAD before Phase 65 changes). Not caused by watch module addition.
