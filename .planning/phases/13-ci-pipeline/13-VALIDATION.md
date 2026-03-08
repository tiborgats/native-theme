# Phase 13: CI Pipeline - Validation

**Phase Goal:** Automated cross-platform testing catches regressions and API breakage on every push

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Manual verification (CI infrastructure phase) |
| Quick run command | `cargo test -p native-theme --no-default-features` |

### Phase Requirements -> Test Map
| Req ID | Behavior | Test Type | Verification Method |
|--------|----------|-----------|-------------------|
| CI-01 | Workflow runs on 3 OS runners | manual-only | Push to GitHub, verify Actions tab shows Linux/Windows/macOS jobs |
| CI-02 | Feature flag matrix tests each feature | manual-only | Verify 7 matrix entries in ci.yml + green CI runs |
| CI-03 | semver-checks catches API breakage | manual-only | Verify semver job output in Actions tab |
| CI-04 | clippy + fmt run in CI | manual-only | Verify lint jobs green in Actions tab |

### Pre-CI Local Verification
| Check | Command | Expected |
|-------|---------|----------|
| Clippy clean | `RUSTFLAGS=-Dwarnings cargo clippy --workspace --all-targets` | Exit 0 |
| Fmt clean | `cargo fmt --all -- --check` | Exit 0, no output |
| Tests pass | `cargo test -p native-theme --no-default-features` | All tests pass |
| ci.yml exists | `test -f .github/workflows/ci.yml` | True |
| ci.yml structure | `grep -c "jobs:" .github/workflows/ci.yml` | 1 |

### Note
All CI-phase requirements are verified by inspecting the workflow file and observing CI runs after push. There are no unit tests to write for CI configuration.
