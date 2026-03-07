# Phase 8: Documentation - Validation Architecture

**Phase:** 08-documentation
**Extracted from:** 08-RESEARCH.md

## Test Framework

| Property | Value |
|----------|-------|
| Framework | rustdoc doctests (built-in) + cargo test |
| Config file | none (built into rustdoc) |
| Quick run command | `cargo test --doc` |
| Full suite command | `cargo test` |

## Phase Requirements -> Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| DOC-01 | README code examples compile | doctest | `cargo test --doc` | Wave 1: needs `#[cfg(doctest)]` struct in lib.rs |
| DOC-01 | Adapter examples are syntactically correct | manual-only | Visual review (examples marked `ignore` due to external deps) | N/A |
| DOC-01 | Feature flag table matches Cargo.toml | manual-only | Compare README table against Cargo.toml `[features]` | N/A |

## Sampling Rate

- **Per task commit:** `cargo test --doc`
- **Per wave merge:** `cargo test`
- **Phase gate:** Full suite green before verification

## Wave 0 Gaps

- [ ] Add `#[doc = include_str!("../README.md")]` + `#[cfg(doctest)]` struct to `src/lib.rs`
- [ ] Create `README.md` (does not exist yet)
