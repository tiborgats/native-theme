# Phase 15: Publishing Prep - Validation

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | cargo test (built-in) |
| Quick run | `cargo test --doc -p native-theme` |
| Full suite | `cargo test -p native-theme && cargo test -p native-theme-iced` |

### Requirement to Test Map

| Req ID | Behavior | Test Type | Command | Automated |
|--------|----------|-----------|---------|-----------|
| PUB-01 | Cargo.toml metadata complete | smoke | `cargo publish --dry-run -p native-theme 2>&1` | yes |
| PUB-02 | License files exist | smoke | `test -f LICENSE-MIT && test -f LICENSE-APACHE && test -f LICENSE-0BSD` | yes |
| PUB-03 | CHANGELOG.md exists with v0.2 | smoke | `grep -q "[0.2.0]" CHANGELOG.md` | yes |
| PUB-04 | Doc examples compile | doctest | `cargo test --doc -p native-theme` | yes |
| PUB-05 | IMPLEMENTATION.md updated | manual | Visual review against source | no |
| PUB-06 | new-os-version-guide.md exists | smoke | `test -f docs/new-os-version-guide.md` | yes |
| PUB-07 | Core crate published | smoke | `cargo publish --dry-run -p native-theme` | yes (dry-run) |
| PUB-08 | Iced connector published | smoke | `cargo publish --dry-run -p native-theme-iced` | yes (dry-run) |

### Sampling Rate

- **Per task:** `cargo test --doc -p native-theme && cargo publish --dry-run -p native-theme`
- **Per wave:** Full dry-run for both crates
- **Phase gate:** Both dry-runs clean, all doc-tests pass, `cargo doc` zero warnings
