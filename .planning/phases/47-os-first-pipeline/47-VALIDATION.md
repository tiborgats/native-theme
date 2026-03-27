# Phase 47: OS-First Pipeline - Validation

## Test Framework

| Property | Value |
|----------|-------|
| Framework | cargo test (built-in) |
| Config file | native-theme/Cargo.toml (dev-dependencies) |
| Quick run command | `cargo test --lib -p native-theme` |
| Full suite command | `cargo test --lib -p native-theme` |

## Requirements -> Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? | Plan |
|--------|----------|-----------|-------------------|-------------|------|
| PIPE-01 | run_pipeline() merges preset + reader, resolves, validates both variants | unit | `cargo test --lib -p native-theme run_pipeline` | Wave 0 (47-01 Task 1) | 47-01 |
| PIPE-01 | run_pipeline() produces both variants even with single-variant reader input | unit | `cargo test --lib -p native-theme run_pipeline_single_variant` | Wave 0 (47-01 Task 1) | 47-01 |
| PIPE-01 | run_pipeline() reader values win over preset values after merge | unit | `cargo test --lib -p native-theme run_pipeline_reader_values_win` | Wave 0 (47-01 Task 1) | 47-01 |
| PIPE-01 | from_system()/from_linux() return SystemTheme via full pipeline | integration | `cargo test --lib -p native-theme run_pipeline_with_preset_as_reader` | Wave 0 (47-01 Task 2) | 47-01 |
| PIPE-02 | platform_preset_name() maps KDE->kde-breeze, GNOME->adwaita on Linux | unit | `cargo test --lib -p native-theme platform_preset` | Wave 0 (47-01 Task 1) | 47-01 |
| PIPE-02 | Each platform preset resolves successfully | unit | Already exists: `presets::tests::all_presets_resolve_validate` | Existing | N/A |
| PIPE-03 | with_overlay() with accent change propagates to primary_bg, checked_bg, slider.fill | unit | `cargo test --lib -p native-theme overlay_accent_propagates` | Wave 0 (47-02 Task 1) | 47-02 |
| PIPE-03 | with_overlay() preserves unrelated fields | unit | `cargo test --lib -p native-theme overlay_preserves` | Wave 0 (47-02 Task 1) | 47-02 |
| PIPE-03 | with_overlay() applies to both light and dark independently | unit | `cargo test --lib -p native-theme overlay_both_variants` | Wave 0 (47-02 Task 1) | 47-02 |
| PIPE-03 | with_overlay() empty overlay produces equivalent result | unit | `cargo test --lib -p native-theme overlay_empty_noop` | Wave 0 (47-02 Task 1) | 47-02 |

## Cross-Cutting Tests

| Behavior | Test Type | Automated Command | File Exists? | Plan |
|----------|-----------|-------------------|-------------|------|
| reader_is_dark() infers dark mode from reader variant presence | unit | `cargo test --lib -p native-theme reader_is_dark` | Wave 0 (47-01 Task 2) | 47-01 |
| SystemTheme::active() returns correct variant based on is_dark | unit | `cargo test --lib -p native-theme system_theme_active` | Wave 0 (47-01 Task 1) | 47-01 |
| SystemTheme::pick() returns requested variant | unit | `cargo test --lib -p native-theme system_theme_pick` | Wave 0 (47-01 Task 1) | 47-01 |
| GNOME double-merge is harmless (preset as reader + pipeline merge) | unit | `cargo test --lib -p native-theme run_pipeline_with_preset_as_reader` | Wave 0 (47-01 Task 2) | 47-01 |

## Sampling Rate

- **Per task commit:** `cargo test --lib -p native-theme`
- **Per wave merge:** `cargo test --lib -p native-theme`
- **Phase gate:** Full suite green + `cargo clippy -p native-theme -- -D warnings` before `/gsd:verify-work`

## Wave 0 Gaps

All tests below are created inline within plan tasks (TDD style: test first, then implementation).

- [ ] Pipeline unit tests: run_pipeline, resolve_variant functions (47-01 Task 1)
- [ ] Platform preset mapping tests (47-01 Task 1)
- [ ] reader_is_dark helper tests (47-01 Task 2)
- [ ] GNOME double-merge harmlessness test (47-01 Task 2)
- [ ] App overlay accent propagation tests (47-02 Task 1)
- [ ] App overlay merge correctness tests (47-02 Task 1)
