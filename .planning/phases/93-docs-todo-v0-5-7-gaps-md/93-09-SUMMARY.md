---
phase: 93-docs-todo-v0-5-7-gaps-md
plan: 09
completed: 2026-04-20T00:00:00Z
atomic_commits:
  - 9a7ef3f  # test(93-09): RED regression test for Phase 93-03 IconLoader silent-ignore bug
  - 3132d7f  # refactor(93-09): replace IconLoader with typed-per-set loaders
  - 870253d  # docs(93-09): CHANGELOG + README migration examples
tasks_completed: 4/4
gap_closure: true
api_break: true
requirements_closed: [G3-design-followup]
---

# Plan 93-09 Summary

## Objective

Eliminate the silent-ignore bug class exposed by Phase 93-03 (`.theme()` dropped on `IconId::Name` lookups). Rather than patch the single dispatch path, restructure `IconLoader` from one struct-with-optional-fields into five typed per-set loader structs. Each loader exposes only the methods meaningful for its set; silent no-op bugs become compile errors.

## What Landed

### RED (commit 9a7ef3f)

Failing unit test in `native-theme/src/icons.rs::load_icon_tests::icon_loader_theme_override_honored_for_name_lookup` demonstrating the bug at the native-theme level (previously only visible via the gpui connector's `gnome_names_resolve_in_adwaita` which was soft-gated). Expected-fail test committed to record the bug in git history.

### GREEN (commit 3132d7f) — 12 files, +574/-503

**`native-theme/src/icons.rs`**: Deleted `IconLoader` struct and impl. Added five typed per-set loader structs with only set-appropriate methods:
- `FreedesktopLoader<'a>` — `size`, `color`, `color_opt`, `theme`, `load`, `load_indicator(theme)` (associated fn)
- `SfSymbolsLoader<'a>` — `load` only (no size/theme/indicator)
- `SegoeIconsLoader<'a>` — `load` only
- `MaterialLoader<'a>` — `load`, `load_indicator()` (associated fn)
- `LucideLoader<'a>` — `load`, `load_indicator()` (associated fn)

Added two free helpers for runtime-set dispatch with default options:
- `pub fn load_icon(id, set) -> Option<IconData>`
- `pub fn load_icon_indicator(set) -> Option<AnimatedIcon>`

Deleted orphaned inner helpers (`load_icon_inner`, `load_icon_from_theme_inner`, `load_system_icon_by_name_inner`, `loading_indicator_inner`) — each per-set loader's `load()` owns its dispatch.

**`native-theme/src/freedesktop.rs`**: `load_freedesktop_spinner` signature changed from `fn() -> Option<AnimatedIcon>` to `fn(theme: Option<&str>) -> Option<AnimatedIcon>`. Closes a second latent silent-ignore bug where `.theme()` was dropped for indicator loads even in the OLD API. Removed orphan `load_freedesktop_icon` (role-taking variant now unused; tests migrated to `load_freedesktop_icon_by_name`).

**Core crate**: `lib.rs` re-exports updated, `model/icons.rs` + `model/mod.rs` + `model/bundled.rs` doc comments migrated, `tests/resolve_and_validate.rs` migrated 4 test blocks.

**native-theme-build**: doc example migrated.

**Connectors**: `gpui/src/icons.rs` and `iced/src/icons.rs` migrated (imports, internal `load_custom_via_builder`, test sites). Both showcases (`showcase-gpui.rs`, `showcase-iced.rs`) migrated all call sites to explicit per-set matches — conditional theme/color application now explicit rather than hidden behind silent-drop builder calls. Wildcard `_ => None` arms added to satisfy `IconSet`'s `#[non_exhaustive]` in external crates.

### Docs (commit 870253d)

**CHANGELOG.md**: Append-only subsection in the existing v0.5.7 `### Breaking Changes` block, tagged "supersedes Phase 93-03" with a before/after migration table covering all five loaders + two runtime-dispatch helpers. Old IconLoader entry preserved for audit trail.

**connectors/native-theme-gpui/README.md** and **connectors/native-theme-iced/README.md**: `load_indicator` example migrated from `IconLoader::new(role).set(set).load_indicator()` chain to `MaterialLoader::load_indicator()` associated-function call.

## Final Verification

Full `./pre-release-check.sh` run, line-by-line scan:

```
🎉 All pre-release checks passed successfully!
native-theme v0.5.7 is ready for release.
```

Every test suite in the script reports `test result: ok. N passed; 0 failed`:

| Suite | Count |
|-------|-------|
| native-theme lib (default features) | 544 passed, 0 failed |
| native-theme merge_behavior | 12 |
| native-theme platform_facts_xref | 6 |
| native-theme prelude_smoke | 2 |
| native-theme preset_loading | 12 |
| native-theme proptest_roundtrip | 11 |
| native-theme reader_kde | 0 (kde feature-gated) |
| native-theme resolve_and_validate | 17 |
| native-theme serde_roundtrip | 8 |
| native-theme doctests | 47 |
| native-theme-derive lib | 9 |
| native-theme-build lib | 208 |
| native-theme-build integration | 24 |
| native-theme-build doctests | 3 |
| **native-theme-gpui lib** | **152 passed (was 151 passed + 1 FAILED before 93-09)** |
| native-theme-iced lib | 97 |
| native-theme-iced integration | 5 |
| native-theme-iced doctests | 7 |

Per-crate with `--all-features`:
- `native-theme --all-features --lib`: 794 passed, 0 failed (was 791; +3 for new regression tests).
- `native-theme --all-features --doc`: 50 passed, 0 failed.
- `native-theme --all-features --tests`: 79 passed, 0 failed.
- `cargo clippy -p native-theme --all-targets -- -D warnings`: exit 0.
- `cargo clippy -p native-theme-gpui --all-targets -- -D warnings`: exit 0.
- `cargo clippy -p native-theme-iced --all-targets -- -D warnings`: exit 0.

Zero `⚠` markers for anything this plan was chartered to fix. Two remaining warnings in the full script output are pre-existing and out of scope:
- cargo-audit `unmaintained` warnings on 4 transitive deps (async-std, instant, paste, rustls-pemfile — all come through gpui-component).
- rustdoc `private_intra_doc_links` on `ResolvedTextScaleEntry` at `native-theme/src/model/resolved.rs:35`.

Both are deferred to separate follow-up plans.

## Regression Check

- `gnome_names_resolve_in_adwaita` now PASSES (previously: 151/1 FAILED with 12 missing icons; now: 152/0).
- No other test regressed.
- Zero `unwrap()` or `expect()` added to production source.
- No `unsafe` added.
- No `Co-Authored-By` trailer in any commit.

## Key Decisions

See STATE.md v0.5.7 Decisions section for Phase 93-09 entries.

## Deviations

None. Plan body followed as written.

## Key Files

- `/home/tibi/Rust/native-theme/native-theme/src/icons.rs` (full rewrite of loader surface)
- `/home/tibi/Rust/native-theme/native-theme/src/freedesktop.rs` (spinner signature change)
- `/home/tibi/Rust/native-theme/native-theme/src/lib.rs` (re-exports)
- `/home/tibi/Rust/native-theme/CHANGELOG.md` (v0.5.7 Breaking Changes append)
- `/home/tibi/Rust/native-theme/connectors/native-theme-gpui/src/icons.rs` + examples/showcase-gpui.rs + README.md
- `/home/tibi/Rust/native-theme/connectors/native-theme-iced/src/icons.rs` + examples/showcase-iced.rs + README.md

## Commits

- `9a7ef3f` — test(93-09): RED regression test
- `3132d7f` — refactor(93-09): typed-per-set loaders (12 files, +574/-503)
- `870253d` — docs(93-09): CHANGELOG + READMEs
