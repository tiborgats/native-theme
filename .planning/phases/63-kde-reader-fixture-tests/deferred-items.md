# Deferred Items - Phase 63

## Pre-existing Issues (Out of Scope)

### 1. `gsettings_get` dead code warning in detect.rs:680

- **Found during:** 63-02 Task 2 pre-release check
- **File:** native-theme/src/detect.rs:680
- **Issue:** `pub(crate) fn gsettings_get` is flagged as never used by `-D dead-code`
- **Impact:** Causes `cargo clippy -p native-theme --all-targets -- -D warnings` to fail
- **Not caused by:** Phase 63 changes (confirmed by stashing changes and re-running clippy)
- **Resolution:** Needs `#[allow(dead_code)]` annotation or actual usage added in a future phase
