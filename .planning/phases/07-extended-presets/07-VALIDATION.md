# Phase 7: Extended Presets - Validation

**Phase:** 07-extended-presets
**Requirements:** PRESET-03, PRESET-04

## Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust built-in test (cargo test) |
| Config file | Cargo.toml (already configured) |
| Quick run command | `cargo test --lib presets` |
| Full suite command | `cargo test` |

## Phase Requirements -> Test Map

| Req ID | Behavior | Test Type | Automated Command | Existing? |
|--------|----------|-----------|-------------------|-----------|
| PRESET-03 | Platform presets parse into NativeTheme with light+dark | integration | `cargo test --test preset_loading` | Yes (auto-covers via list_presets iteration) |
| PRESET-03 | Platform presets have non-empty core colors | integration | `cargo test --test preset_loading` | Yes (auto-covers) |
| PRESET-03 | Platform presets round-trip TOML | integration | `cargo test --test preset_loading` | Yes (auto-covers) |
| PRESET-04 | Community presets parse into NativeTheme with light+dark | integration | `cargo test --test preset_loading` | Yes (auto-covers) |
| PRESET-04 | Community presets have non-empty core colors | integration | `cargo test --test preset_loading` | Yes (auto-covers) |
| PRESET-04 | Community presets round-trip TOML | integration | `cargo test --test preset_loading` | Yes (auto-covers) |
| PRESET-03/04 | list_presets() returns all 18 names | unit+integration | `cargo test presets_returns` | Yes (needs count update 3->18) |
| PRESET-03/04 | Dark backgrounds darker than light | integration | `cargo test dark_backgrounds_are_darker` | Yes (auto-covers) |

## Sampling Rate

- **Per task commit:** `cargo test`
- **Per wave merge:** `cargo test`
- **Phase gate:** Full suite green before `/gsd:verify-work`

## Wave 0 Gaps

- [ ] 15 new TOML files under `src/presets/` -- each ~150 lines, both light+dark variants
- [ ] `src/presets.rs` -- updated with 15 new constants, match arms, PRESET_NAMES entries
- [ ] `tests/preset_loading.rs` -- updated count assertion from 3 to 18
- [ ] `src/presets.rs` unit tests -- updated count and name assertions
