# v0.5.8 — gpui connector on gpui-component 0.6: Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

Status: Implemented on branch `v0.5.8-gpui-component-0.6` (2026-09-06); release pending the maintainer's testing and approval
Date: 2026-09-05

**Goal:** Move `native-theme-gpui` onto gpui-component 0.6.0 / gpui-base 0.6.0 / GPUI as `gpui-pre` 0.3.x and deliver everything the v0.5.8 specification lists: the complete 139-field colour mapping, accessibility as a first-class input, automatic re-application of the native base-layer overrides, per-widget geometry builders, honest `Option` icon tables on refreshed bundles, two additive `native-theme` APIs, a dependency refresh and two measured MSRV floors.

**Architecture:** The connector keeps its three mapping stages (`colors.rs` → `config.rs` → `to_theme`) and gains an installation layer in `lib.rs` (`apply`, a `NativeTheme` global, a `gpui_base::Theme` observer) plus two pure modules: `base_layer.rs` (scrollbar geometry and resize-handle colours written onto gpui-base) and `geometry.rs` (`StyleRefinement` builders applied with `refine_style`). `native-theme` gains `SystemTheme.layout`, `AccessibilityPreferences::from_system()`, a `build.rs` that generates the icon name tables from the bundle directories, and refreshed bundles with a provenance manifest and refresh script.

**Tech Stack:** Rust 2024 edition; `gpui = { package = "gpui-pre", version = "0.3.3" }`; `gpui-component = "0.6.0"`; `gpui-base = "0.6.0"`; `gpui-kit = "0.6.0"` (showcase only); `#[gpui::test]` with `TestAppContext` for headless `App` tests; `serde_json` for the field-completeness test; bash + python3 (≥3.11, `tomllib`) for the icon refresh script.

**Spec:** [`docs/todo_v0.5.8_gpui-component-0.6-spec.md`](todo_v0.5.8_gpui-component-0.6-spec.md)
**Rationale:** [`docs/todo_v0.5.8_gpui-component-0.6-rationale.md`](todo_v0.5.8_gpui-component-0.6-rationale.md)

The plan argues from the spec; executors read both. Section numbers below (§) refer to the spec unless prefixed "rationale".

## Global Constraints

- **No fork, no patch, no overwrite** of gpui-pre, gpui-base, gpui-component or gpui-kit. Every value enters through a public API of the pinned versions (§0.2).
- **Pinned requirements** (§4.1): `gpui = { package = "gpui-pre", version = "0.3.3" }`, `gpui-component = "0.6.0"`, `gpui-base = "0.6.0"`, dev: `gpui = { package = "gpui-pre", version = "0.3.3", features = ["test-support"] }`, `gpui-kit = "0.6.0"`.
- **No invented values** (§3.5): every numeric or colour literal in connector code is (a) a `ResolvedTheme` or `AccessibilityPreferences` field, (b) one of the derivations in §8.2 / §9.4, or (c) a clamp guard (`max(0.0)`, `> 0.0`). The pre-tool-use hook blocks anything else.
- **No panics, no unsafe** (project rule). The connector keeps `#![forbid(unsafe_code)]`, `#![deny(clippy::unwrap_used)]`, `#![deny(clippy::expect_used)]`; only `#[cfg(test)]` modules carry `#[allow(clippy::unwrap_used, clippy::expect_used)]`. `build.rs` returns `Result<(), Box<dyn Error>>` instead of panicking. No `Theme::global` / `global_mut` call without the `has_global` guard; `try_global` everywhere else (§8.1, D29).
- **Dependency policy** (§4.3): latest stable or a recorded reason. Requirement strings for this release: serde `1.0.229`, serde_with `3.22.0`, toml `1.1.5`, serde_json `1.0.151`, arc-swap `1.9.2`, async-trait `0.1.92`, ashpd `0.13.13`, configparser `3.2.0`, zbus `5.19.0`, quote `1.0.47`, proc-macro2 `1.0.107`, pollster `1.0`, syn `3.0.5` (gated on the derive crate's tests; fallback `2.0.119` with a recorded reason), resvg `0.48.1`.
- **MSRV** (§4.4): workspace floor re-measured after the refresh (measured in Task 1: `1.88.0`, unchanged; the predicted 1.89.0 was wrong, see spec §4.4); connector gets its own `rust-version`, provisional `1.95`, measured in Task 13. Both are *measured*, never assumed.
- **Breaking changes** are allowed (pre-1.0) and are exactly the rows of §7.1; everything else is additive. No migration guide (project rule).
- **Icon rules**: tables return `None` where a set has no equivalent, never a substitute (§10.1); bundles hold only genuine upstream files under upstream names (§10.2); confidence labels `exact` / `close` / `approximate` on every freedesktop and Material row (rationale §2.21).
- **Test feature set** for native-theme, as in `publish.yml`: `--features material-icons,lucide-icons,system-icons,svg-rasterize`.
- **Citations** in code comments follow the spec's convention: `crate version path:line` (e.g. `gpui-component 0.6.0 src/button/button.rs:650`).
- **Commits**: conventional-commit subjects as in `git log` (`feat(gpui)!:`, `chore(deps):`, `docs:`); **no AI attribution trailers** (project rule, memory `feedback_no_coauthored_by`). Run `./pre-release-check.sh` before the release commit.
- **Release gate**: tag, push tag, publish only on the maintainer's explicit approval (Task 16). Nothing in this plan authorises it.

## Execution notes

- **Model routing** (maintainer's instruction of 2026-09-05, consistent with `~/.claude/CLAUDE.md`): precisely specified, mechanical tasks run on **Opus 5** through the `implement` agent (`subagent_type: "implement"`, never a bare dispatch or a fork); tasks that need further planning, naming, documentation wording, API discovery or visual judgment run on **Fable 5.1** inline. Each task header carries its assignment. An Opus task that fails its gate twice is not retried a third time: Fable takes it over or re-specifies, and the escalation (cause: spec ambiguity vs under-thinking) is logged in the cascatura memory file `opus_gate_failures.md`.

| Task | Model | Why |
|------|-------|-----|
| 1 Dependency refresh, MSRV | Opus 5 | version strings and toolchain commands are given; the syn-3 fallback rule is explicit |
| 2 `SystemTheme.layout` | Opus 5 | test and edits given verbatim |
| 3 `AccessibilityPreferences::from_system()` | Opus 5 | code given verbatim |
| 4 Generated icon tables | Opus 5 | `build.rs` given verbatim |
| 5 Icon bundles | Opus 5 for steps 1–7 and 9–12; **Fable 5.1 for step 8** (glyph check) and for reading the refreshed-file diff before the commit | the visual comparison and any name change are judgment |
| 6 Connector port + icon tables | Opus 5 | every rename and every table arm is given; the gate is the compile |
| 7 Colour mapping | Opus 5 | field list and test given verbatim |
| 8 Accessibility API | Opus 5 | signatures and bodies given; documentation wording is deferred to Task 15 |
| 9 `base_layer` | Opus 5 | module given verbatim |
| 10 `NativeTheme`, `apply`, observer | **Fable 5.1** | effect-ordering semantics; a failing observer test needs reasoning, not retries |
| 11 `geometry` | Opus 5 | module and tests given verbatim; escalate to Fable if a preset precondition assertion fails |
| 12 Showcase | **Fable 5.1** | 31+ compile errors need API discovery; placement of builders, tooltip wording and the visual smoke run are judgment |
| 13 Connector MSRV | Opus 5 | one loop |
| 14 CI / publish | Opus 5; **Fable 5.1** if the apt-list check surfaces a new library | YAML edits are mechanical, a new system library is not |
| 15 Docs | **Fable 5.1** | wording, README structure, roadmap rewrite |
| 16 Release | **Fable 5.1** | approval gate, irreversible actions |
- **Intermediate red states, and where they are allowed**: after Task 5 the 0.5.1 connector's icon tests may fail until Task 6 (the bundle names it used are gone); from Task 6 until Task 12 the showcase example does not compile, so connector gates use `cargo test -p native-theme-gpui --lib` (a plain `cargo test` also builds examples). No task leaves `native-theme`, `native-theme-derive`, `native-theme-build` or `native-theme-iced` red.
- Run every command from the repository root. Where a step says "Expected: FAIL", run it and read the failure before implementing; the failure text is part of the gate.
- **Repository hooks** (`.claude/settings.json`, PreToolUse on Write/Edit) check every edit of a `.rs` file outside `tests/`: `no-runtime-panics.sh` rejects an edit whose text contains `.unwrap()`, `.expect(`, `panic!(` or `unsafe` unless that same text also contains `#[cfg(test)]`, `#[test]` or `#[allow(clippy::unwrap_used` (a bare `#[gpui::test]` does not count), so write each test module in one edit that includes its header; `no-invented-values.sh` rejects `unwrap_or(<number>)`, `map_or(<number>` and multiplications by `0.6x`–`0.8x`, `1.1x`–`1.3x`, `1.8x` or `2.0`, in test modules too, which is why every scaling test in this plan uses the factor `1.5`. Files under `tests/`, `.md`, `.toml`, `.sh` and `.py` are exempt.
- Each task ends with one commit. Steps inside a task that say "Run" are the mechanical checks; do not skip them.

## File structure

| File | Action | Responsibility |
|------|--------|----------------|
| `Cargo.toml`, `native-theme/Cargo.toml`, `native-theme-derive/Cargo.toml`, `native-theme-build/Cargo.toml`, `Cargo.lock` | modify | dependency refresh, workspace MSRV |
| `native-theme/src/lib.rs` | modify | `SystemTheme.layout`; `AccessibilityPreferences::from_system()` |
| `native-theme/src/pipeline.rs` | modify | `layout` in `run_pipeline`; `accessibility_from_system_inner` |
| `native-theme/build.rs` | create | generates `lucide_svg_by_name` / `material_svg_by_name` from the icon directories |
| `native-theme/src/model/bundled.rs` | modify | `include!` the generated tables; three role-table paths `trash-2.svg` → `trash.svg`; tests |
| `native-theme/icons/SOURCES.toml` | create | provenance manifest: one rule per set, one per-file exception per set |
| `native-theme/icons/lucide/*.svg`, `native-theme/icons/material/*.svg` | modify | ten duplicates deleted, `trash-2` → `trash`, `star_border` removed, 28 files added, both sets refreshed |
| `native-theme/tests/icon_sources.rs` | create | manifest coverage tests |
| `scripts/refresh-icons.sh` | create | re-downloads every file from the manifest; `add` sub-command |
| `scripts/README.md` | modify | documents the refresh script |
| `connectors/native-theme-gpui/Cargo.toml` | modify | 0.6.0 stack, own `rust-version`, dev-deps |
| `connectors/native-theme-gpui/src/lib.rs` | modify | `to_theme` / `from_preset` signatures, text scaling, `focus_ring`, `scrollbar_mode`; `NativeTheme`, `ActiveNativeTheme`, `Native`, `apply`, `apply_system_theme`, `apply_accessibility`, observer |
| `connectors/native-theme-gpui/src/colors.rs` | modify | 139-field mapping, `assign_buttons`, completeness test |
| `connectors/native-theme-gpui/src/config.rs` | modify | 34 new hex exports, scaled font sizes, `prefs` parameter |
| `connectors/native-theme-gpui/src/icons.rs` | modify | `Option` tables, 15 new arms, real Lucide names, `Github`, `ALL_ICON_NAMES` 101 |
| `connectors/native-theme-gpui/src/base_layer.rs` | create | `ScrollbarGeometry`, `scrollbar_geometry`, `scrollbar_styles`, `resizable_theme`, `apply_overrides` |
| `connectors/native-theme-gpui/src/geometry.rs` | create | per-widget builders, `Size` helpers, `control_height`, layout accessors |
| `connectors/native-theme-gpui/examples/showcase-gpui.rs` | modify | port to gpui-kit; `apply`; geometry builders; 139-field colour map |
| `connectors/native-theme-gpui/README.md`, `CHANGELOG.md`, `ROADMAP.md`, `docs/todo.md`, `docs/todo_gpui-full-theme.md`, `docs/todo_v0.6.0_egui-connector-{spec,rationale}.md`, `docs/archive/v0.5.7_gaps.md` | modify | documentation (§13) |
| `.github/workflows/publish.yml`, `.github/workflows/ci.yml` | modify | remove G11 soft gates; apt list re-verified |

---

### Task 1: Dependency refresh and workspace MSRV re-measurement (§4.3, §4.4; rationale §2.14, §2.16)

**Model:** Opus 5 (`implement` agent)

**Files:**
- Modify: `Cargo.toml` (`[workspace.package] rust-version`, `[workspace.dependencies]`)
- Modify: `native-theme/Cargo.toml`, `native-theme-derive/Cargo.toml`, `native-theme-build/Cargo.toml`
- Regenerate: `Cargo.lock`

**Interfaces:**
- Consumes: nothing.
- Produces: the lock every later gate runs on; the measured workspace `rust-version`.

- [ ] **Step 1: Bump every requirement string to the §1.5 versions**

```bash
sed -i 's/serde = { version = "1.0.228"/serde = { version = "1.0.229"/; s/serde_with = "3.18.0"/serde_with = "3.22.0"/; s/^toml = "1.1.2"/toml = "1.1.5"/' Cargo.toml
sed -i 's/arc-swap = "1.9.1"/arc-swap = "1.9.2"/; s/async-trait = "0.1.89"/async-trait = "0.1.92"/; s/resvg = { version = "0.47"/resvg = { version = "0.48.1"/; s/ashpd = { version = "0.13.10"/ashpd = { version = "0.13.13"/; s/configparser = { version = "3.1.0"/configparser = { version = "3.2.0"/; s/^pollster = "0.4"/pollster = "1.0"/; s/zbus = { version = "5.14"/zbus = { version = "5.19.0"/; s/serde_json = "1.0.149"/serde_json = "1.0.151"/' native-theme/Cargo.toml
sed -i 's/syn = { version = "2.0.117"/syn = { version = "3.0.5"/; s/quote = "1.0.45"/quote = "1.0.47"/; s/proc-macro2 = "1.0.106"/proc-macro2 = "1.0.107"/' native-theme-derive/Cargo.toml
sed -i 's/^toml = "1.1.2"/toml = "1.1.5"/; s/serde = { version = "1.0.228"/serde = { version = "1.0.229"/' native-theme-build/Cargo.toml
grep -n 'pollster\|resvg\|syn =\|serde_with\|^toml\|zbus\|ashpd\|configparser\|arc-swap\|async-trait\|serde_json\|quote\|proc-macro2\|serde = ' Cargo.toml native-theme/Cargo.toml native-theme-derive/Cargo.toml native-theme-build/Cargo.toml
```

Expected: every listed crate shows the new requirement; `pollster = "1.0"` appears twice in `native-theme/Cargo.toml` (dependencies and dev-dependencies).

- [ ] **Step 2: Regenerate the lock**

```bash
cargo update
git diff --stat Cargo.lock
```

Expected: the lock changes; `grep -A1 'name = "resvg"' Cargo.lock` shows `0.48.1`, `grep -A1 'name = "syn"' Cargo.lock` shows a `3.0.x` entry (a `2.x` entry may remain for transitive users; that is fine).

- [ ] **Step 3: syn 3 gate**

```bash
cargo test -p native-theme-derive
```

Expected: PASS. If it fails on `Type::BareFn` (renamed `Type::FnPtr`) or a `*Modifiers` struct, apply the rename in `native-theme-derive/src` and re-run. If it cannot be made green with mechanical edits, revert to `syn = { version = "2.0.119", features = ["full", "extra-traits"] }`, run `cargo update -p syn`, and record the reason in rationale §2.14 (the `syn` row) and in the CHANGELOG `Changed` entry for the dependency refresh.

- [ ] **Step 4: Workspace tests on the new lock**

```bash
cargo test -p native-theme-derive
cargo test -p native-theme-build
cargo test -p native-theme --features material-icons,lucide-icons,system-icons,svg-rasterize
cargo test -p native-theme-iced
cargo test -p native-theme-gpui
```

Expected: the four workspace members PASS; the resvg 0.48 rasterisation change is covered by `rasterize_produces_non_empty_pixels` (`native-theme/src/rasterize.rs:130`). The connector is still on the 0.5.1 stack and should pass too; if the refreshed lock resolves a naga / codespan-reporting pair that stack cannot compile (the G11 class of failure), record the error in the commit body and continue: Task 6 replaces the stack.

- [ ] **Step 5: Measure the workspace floor**

Candidate from the declared floors in the lock:

```bash
cargo metadata --format-version 1 --locked \
  | jq -r '[.packages[] | select(.rust_version != null) | .rust_version] | unique | sort_by(split(".") | map(tonumber)) | last'
```

Expected: `1.89.0` or `1.89` (from `font-types`, pulled by resvg 0.48.1). Then the authoritative per-member check, following commit `0319942`:

```bash
rustup toolchain install 1.89.0 --profile minimal
cargo +1.89.0 check -p native-theme-derive --all-targets --locked
cargo +1.89.0 check -p native-theme-build --all-targets --locked
cargo +1.89.0 check -p native-theme --all-targets --locked --features material-icons,lucide-icons,system-icons,svg-rasterize,linux,watch
cargo +1.89.0 check -p native-theme-iced --all-targets --locked
```

Expected: all four succeed. If one fails with "package `X` requires rustc N", install `N`, repeat from the top with `N`; the first version that passes all four is the floor.

- [ ] **Step 6: Declare the measured floor**

In `Cargo.toml` set `rust-version = "<measured>"` under `[workspace.package]` (expected `"1.89.0"`). Do not touch `connectors/native-theme-gpui/Cargo.toml` yet (Task 6 gives it its own floor).

Then, because the workspace uses `resolver = "3"` and `cargo update` resolves against the lowest workspace `rust-version` (MSRV-aware resolution), run `cargo update` once more: with the floor raised from 1.88.0 the resolver may now pick releases it held back. If `git diff --stat Cargo.lock` shows a change, repeat Step 5 on the new lock; the floor cannot drop, and if it rises, declare the new value and run this step again until the lock is stable.

- [ ] **Step 7: Commit**

```bash
git add Cargo.toml Cargo.lock native-theme/Cargo.toml native-theme-derive/Cargo.toml native-theme-build/Cargo.toml
git commit -m "chore(deps): refresh every external requirement to its latest release; workspace MSRV re-measured

pollster 1.0 (only change: FutureExt for IntoFuture), syn 3.0.5 (derive crate
tests pass unchanged), resvg 0.48.1 (font-types raises the declared floor to
1.89). Workspace rust-version measured per member with cargo +1.89.0 check
--locked, as in 0319942."
```

(If syn stayed on 2.0.119, say so in the body with the failing error.)

---

### Task 2: `SystemTheme.layout` (§11.1; rationale §2.17)

**Model:** Opus 5 (`implement` agent)

**Files:**
- Modify: `native-theme/src/lib.rs` (struct `SystemTheme` ~line 369; `with_overlay` ~line 470-535; three test literals at ~640, ~680, ~720)
- Modify: `native-theme/src/pipeline.rs` (`run_pipeline`, literal at ~line 130; tests module)
- Modify: `docs/todo.md` (close the item at lines 7-19)

**Interfaces:**
- Consumes: `Theme::merge` (already merges `layout` field-wise, `model/mod.rs:338`), `LayoutTheme` (`model/widgets/mod.rs:884`, derives `Clone, Debug, Default, PartialEq`).
- Produces: `pub layout: LayoutTheme` on `native_theme::SystemTheme`; the connector's layout accessors (Task 11) and the showcase read it.

- [ ] **Step 1: Write the failing test** in the existing `#[cfg(test)]` module of `native-theme/src/pipeline.rs` (the one that already calls `run_pipeline`):

```rust
    // Add to `mod pipeline_tests` (pipeline.rs ~line 985, `#[cfg(test)]`, not Linux-gated).
    /// §11.1: `SystemTheme.layout` is the reader's layout merged field-wise over
    /// the preset's. On the preset-only path the reader *is* the full preset, so
    /// the result equals full ⊕ live ⊕ full.
    #[test]
    fn system_theme_layout_is_the_merged_preset_layout() -> crate::Result<()> {
        let reader = preset_as_reader("adwaita", crate::ColorMode::Light)?;
        let sys = run_pipeline(reader, "adwaita-live", crate::ColorMode::Light)?;

        let mut expected = Theme::preset("adwaita")?.layout;
        expected.merge(&Theme::preset("adwaita-live")?.layout);
        expected.merge(&Theme::preset("adwaita")?.layout);

        assert_eq!(sys.layout, expected);
        assert!(
            sys.layout.widget_gap.is_some(),
            "adwaita defines all four layout keys (spec §1.3)"
        );
        Ok(())
    }
```

- [ ] **Step 2: Run it to verify it fails**

```bash
cargo test -p native-theme system_theme_layout_is_the_merged_preset_layout
```

Expected: FAIL to compile with `no field `layout` on type `SystemTheme``.

- [ ] **Step 3: Add the field and fill it on both construction paths**

In `native-theme/src/lib.rs`, inside `pub struct SystemTheme`, after `pub icon_theme: Cow<'static, str>,`:

```rust
    /// Layout spacing shared by both variants: the platform reader's values
    /// merged field-wise over the preset's, the same precedence the pipeline
    /// uses for colours. `None` in a field means neither the platform nor the
    /// preset specifies it (platform-facts §2.20); nothing is invented.
    pub layout: LayoutTheme,
```

In `native-theme/src/pipeline.rs` `run_pipeline`, immediately before the comment `// Match on ReaderOutput for type-safe variant selection:` add

```rust
    // Shared across variants; read before the variants are moved out of `merged`.
    let layout = merged.layout.clone();
```

and add `layout,` to the `Ok(SystemTheme { ... })` literal (after `icon_theme,`).

In `native-theme/src/lib.rs` `with_overlay`, immediately before `let (mut light, mut dark) = match &src.reader_output {` add the same two lines, and add `layout,` to its `Ok(SystemTheme { ... })` literal.

In the three test literals in `native-theme/src/lib.rs` (`let st = SystemTheme { ... }` at ~640, ~680, ~720) add `layout: LayoutTheme::default(),`.

- [ ] **Step 4: Run the tests**

```bash
cargo test -p native-theme --features material-icons,lucide-icons,system-icons,svg-rasterize
```

Expected: PASS, including the new test.

- [ ] **Step 5: Close the todo item**

In `docs/todo.md` change line 9 from `- [ ] Add \`pub layout: LayoutTheme\` to \`SystemTheme\`.` to `- [x] Add \`pub layout: LayoutTheme\` to \`SystemTheme\` (done in v0.5.8).` and leave the explanatory lines.

- [ ] **Step 6: Commit**

```bash
git add native-theme/src/lib.rs native-theme/src/pipeline.rs docs/todo.md
git commit -m "feat(native-theme): expose the merged layout on SystemTheme

Approved 2026-08-10. Field-wise merge of the reader's LayoutTheme over the
preset's, computed where the pipeline already merges both (pipeline.rs) and
on the with_overlay replay."
```

---

### Task 3: `AccessibilityPreferences::from_system()` (§11.2; rationale §2.17)

**Model:** Opus 5 (`implement` agent)

**Files:**
- Modify: `native-theme/src/pipeline.rs` (new `accessibility_from_system_inner`)
- Modify: `native-theme/src/lib.rs` (`impl AccessibilityPreferences`; test)

**Interfaces:**
- Consumes: `pipeline::select_reader()` (`pipeline.rs:~612`, currently private `async fn`), `crate::reader::ThemeReader::read`, `crate::detect::detect_reduced_motion()` (`detect.rs:662`, the uncached variant).
- Produces: `pub fn AccessibilityPreferences::from_system() -> AccessibilityPreferences`; the connector's preset path and README use it.

Resolution of an ambiguity in §11.2 ("on every platform `reduce_motion` comes from `detect::detect_reduced_motion()`"): the reader's value is kept where a reader supplies one (KDE's `AnimationDurationFactor`, GNOME's portal), and `detect_reduced_motion()` is OR-ed in, as a floor for the paths that leave `reduce_motion` at its default: non-KDE/GNOME Linux, and macOS or Windows builds without their reader feature (with the feature on, both readers fill all four fields — `macos.rs:513-528`, `windows.rs:393-398`; the brief's first wording said they do not, rationale error 55). Nothing is ever turned *off* by the fallback. The **uncached** `detect_reduced_motion()` is used, not `prefers_reduced_motion()`: the latter caches its first answer in a process-wide `OnceLock` (`detect.rs:654-656, 856-857`), while `from_system()` re-reads the platform reader on every call and is what a caller polls before `apply_accessibility`; mixing a cached and an uncached source would freeze the fallback at its first value.

- [ ] **Step 1: Write the failing test** in the `#[cfg(test)]` module of `native-theme/src/lib.rs`:

```rust
    /// §11.2: `from_system()` is an extraction of the reader path, so it must
    /// agree with `SystemTheme::from_system()` wherever both succeed, and it
    /// must never return a non-finite or non-positive text scale.
    #[test]
    fn accessibility_preferences_from_system_is_consistent_with_system_theme() {
        let prefs = AccessibilityPreferences::from_system();
        assert!(prefs.text_scaling_factor.is_finite());
        assert!(prefs.text_scaling_factor > 0.0);

        // CI has no desktop; only compare when the full pipeline also works.
        if let Ok(sys) = SystemTheme::from_system() {
            assert_eq!(prefs.text_scaling_factor, sys.accessibility.text_scaling_factor);
            assert_eq!(prefs.high_contrast, sys.accessibility.high_contrast);
            assert_eq!(prefs.reduce_transparency, sys.accessibility.reduce_transparency);
            // reduce_motion may additionally be true via detect::detect_reduced_motion().
            assert!(prefs.reduce_motion || !sys.accessibility.reduce_motion);
        }
    }
```

- [ ] **Step 2: Run it to verify it fails**

```bash
cargo test -p native-theme --features linux accessibility_preferences_from_system_is_consistent
```

Expected: FAIL to compile with `no function or associated item named `from_system` found for struct `AccessibilityPreferences``.

- [ ] **Step 3: Implement**

In `native-theme/src/pipeline.rs` add, directly after `from_system_inner` (same module as the private `select_reader`, so no visibility change):

```rust
/// Reader-only extraction for [`crate::AccessibilityPreferences::from_system`]:
/// runs the same platform reader `from_system_inner` would run and returns its
/// accessibility block without merging or resolving a theme. Where no reader
/// is available (or it fails) the defaults are used. `reduce_motion` is OR-ed
/// with [`crate::detect::detect_reduced_motion`], which covers macOS and
/// Windows builds without their reader feature; with it on, both readers
/// fill all four fields, and OR-ing their `true` is a no-op (spec §11.2).
pub(crate) async fn accessibility_from_system_inner() -> crate::AccessibilityPreferences {
    let mut prefs = match select_reader().await {
        Some((reader, _preset)) => match reader.read().await {
            Ok(result) => result.accessibility,
            Err(_) => crate::AccessibilityPreferences::default(),
        },
        None => crate::AccessibilityPreferences::default(),
    };
    if !prefs.reduce_motion {
        prefs.reduce_motion = crate::detect::detect_reduced_motion();
    }
    prefs
}
```

In `native-theme/src/lib.rs`, after `impl Default for AccessibilityPreferences { ... }`:

```rust
impl AccessibilityPreferences {
    /// Read the OS accessibility preferences without resolving a theme.
    ///
    /// Runs the same platform reader [`SystemTheme::from_system`] runs (KDE
    /// `kdeglobals`, GNOME portal + gsettings) and takes its accessibility
    /// block; `reduce_motion` is additionally read through
    /// [`crate::detect::detect_reduced_motion`] on every platform. Fields no
    /// reader supplies keep their defaults. Never fails: with no reader or a
    /// failing reader the defaults are returned.
    ///
    /// Use it on the preset path, where accessibility is orthogonal to the
    /// theme choice (a user with large text wants it under a preset too).
    #[must_use]
    #[cfg(target_os = "linux")]
    pub fn from_system() -> Self {
        pollster::block_on(pipeline::accessibility_from_system_inner())
    }

    /// Read the OS accessibility preferences without resolving a theme (non-Linux).
    ///
    /// The inner future has no `.await` points off Linux, so a noop-waker
    /// single poll suffices, as in [`SystemTheme::from_system`].
    #[must_use]
    #[cfg(not(target_os = "linux"))]
    pub fn from_system() -> Self {
        let waker = std::task::Waker::noop();
        let mut cx = std::task::Context::from_waker(&waker);
        let mut fut = std::pin::pin!(pipeline::accessibility_from_system_inner());
        match fut.as_mut().poll(&mut cx) {
            std::task::Poll::Ready(prefs) => prefs,
            std::task::Poll::Pending => Self::default(),
        }
    }
}
```

- [ ] **Step 4: Run the tests**

```bash
cargo test -p native-theme --features material-icons,lucide-icons,system-icons,svg-rasterize,linux
cargo test -p native-theme   # no features: the non-reader path must compile and return defaults
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add native-theme/src/lib.rs native-theme/src/pipeline.rs
git commit -m "feat(native-theme): AccessibilityPreferences::from_system()

An extraction of the reader path, not new detection: the same KDE/GNOME
reader fills the struct, reduce_motion is OR-ed with
detect::detect_reduced_motion() as a floor for paths whose reader leaves it
at the default."
```

---

### Task 4: Generated icon name tables (§10.5 item 3; rationale §2.15)

**Model:** Opus 5 (`implement` agent)

Behaviour-preserving: the two hand-written `match` tables in `bundled.rs` become build-script output from the directory listings. The four files the hand-written Lucide table never listed (`scan`, `grip`, `arrow-up-narrow-wide`, `arrow-down-wide-narrow`) become resolvable, which is the failing test.

**Files:**
- Create: `native-theme/build.rs`
- Modify: `native-theme/src/model/bundled.rs` (replace `lucide_svg_by_name` and `material_svg_by_name` bodies at ~lines 222-449 with `include!`s; add tests)

**Interfaces:**
- Consumes: `icons/lucide/*.svg`, `icons/material/*.svg`; `CARGO_MANIFEST_DIR`, `OUT_DIR`.
- Produces: `fn lucide_svg_by_name(name: &str) -> Option<&'static [u8]>` and `fn material_svg_by_name(name: &str) -> Option<&'static [u8]>` with identical signatures to today, generated into `$OUT_DIR/lucide_svg_by_name.rs` / `$OUT_DIR/material_svg_by_name.rs`.

- [ ] **Step 1: Write the failing tests** in the `mod tests` of `native-theme/src/model/bundled.rs`:

```rust
    fn bundled_files(dir: &str) -> Vec<String> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("icons").join(dir);
        let mut names: Vec<String> = std::fs::read_dir(&path)
            .expect("icon directory exists")
            .map(|entry| entry.expect("readable entry").file_name().to_string_lossy().into_owned())
            .filter_map(|file| file.strip_suffix(".svg").map(str::to_owned))
            .collect();
        names.sort();
        assert!(!names.is_empty(), "icons/{dir} must not be empty");
        names
    }

    /// §10.5: the by-name table is generated from the directory, so every
    /// bundled file resolves and the table cannot drift.
    #[test]
    #[cfg(feature = "lucide-icons")]
    fn generated_lucide_table_covers_every_bundled_file() {
        for name in bundled_files("lucide") {
            let svg = bundled_icon_by_name(&name, IconSet::Lucide)
                .unwrap_or_else(|| panic!("Lucide table misses bundled file {name}.svg"));
            assert!(std::str::from_utf8(svg).expect("UTF-8").contains("<svg"));
        }
    }

    #[test]
    #[cfg(feature = "material-icons")]
    fn generated_material_table_covers_every_bundled_file() {
        for name in bundled_files("material") {
            let svg = bundled_icon_by_name(&name, IconSet::Material)
                .unwrap_or_else(|| panic!("Material table misses bundled file {name}.svg"));
            assert!(std::str::from_utf8(svg).expect("UTF-8").contains("<svg"));
        }
    }
```

- [ ] **Step 2: Run them to verify they fail**

```bash
cargo test -p native-theme --features lucide-icons,material-icons generated_
```

Expected: both FAIL. The Lucide test on `Lucide table misses bundled file arrow-down-wide-narrow.svg` (the first of the four missing names in sorted order); the Material test on `content_cut.svg`, the first of the eleven role-table-only files the hand-written Material table lacks (`content_cut`, `content_paste`, `edit`, `error`, `help`, `home`, `lock`, `print`, `refresh`, `save`, `shield`). Both failures are the point of this task.

- [ ] **Step 3: Write `native-theme/build.rs`**

```rust
//! Generates the by-name SVG tables for the bundled icon sets from the
//! directory listings under `icons/`, so the tables cannot drift from the
//! files (v0.5.8 spec §10.5). No panics: every error is returned to Cargo.

use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let out_dir = std::env::var("OUT_DIR")?;

    for (set, dir, fn_name) in [
        ("Lucide", "lucide", "lucide_svg_by_name"),
        ("Material", "material", "material_svg_by_name"),
    ] {
        let icons_dir = Path::new(&manifest_dir).join("icons").join(dir);
        // A directory path makes Cargo re-run this script when entries are
        // added, removed or modified.
        println!("cargo:rerun-if-changed={}", icons_dir.display());

        let mut names: Vec<String> = fs::read_dir(&icons_dir)?
            .filter_map(Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .filter_map(|file| file.strip_suffix(".svg").map(str::to_owned))
            .collect();
        names.sort();

        let mut out = fs::File::create(Path::new(&out_dir).join(format!("{fn_name}.rs")))?;
        writeln!(out, "// Generated by build.rs from `icons/{dir}/*.svg` ({set}). Do not edit.")?;
        writeln!(out, "fn {fn_name}(name: &str) -> Option<&'static [u8]> {{")?;
        writeln!(out, "    match name {{")?;
        for name in &names {
            writeln!(
                out,
                "        {name:?} => Some(include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/icons/{dir}/{name}.svg\"))),"
            )?;
        }
        writeln!(out, "        _ => None,")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;
    }
    Ok(())
}
```

- [ ] **Step 4: Replace the hand-written tables**

In `native-theme/src/model/bundled.rs` delete the whole `fn lucide_svg_by_name` and `fn material_svg_by_name` definitions (including their `#[cfg(feature = ...)]` attributes) and put in their place:

```rust
#[cfg(feature = "lucide-icons")]
include!(concat!(env!("OUT_DIR"), "/lucide_svg_by_name.rs"));

#[cfg(feature = "material-icons")]
include!(concat!(env!("OUT_DIR"), "/material_svg_by_name.rs"));
```

Keep `bundled_icon_by_name` and the role tables (`material_svg`, `lucide_svg`) exactly as they are.

- [ ] **Step 5: Run the tests**

```bash
cargo test -p native-theme --features material-icons,lucide-icons,system-icons,svg-rasterize
cargo test -p native-theme            # no icon features: the include!s are cfg-gated
cargo package -p native-theme --allow-dirty --no-verify --list | grep -c '^icons/'      # expected: every icon file (Task 5 changes the count)
cargo package -p native-theme --allow-dirty --no-verify --list | grep -c '^build.rs$'   # expected 1
```

Expected: all PASS (the existing hand-written `lucide_by_name_covers_gpui_icons` / `material_by_name_covers_gpui_icons` still pass here; Task 5 retires them); the package listing includes every icon file and `build.rs`.

- [ ] **Step 6: Commit**

```bash
git add native-theme/build.rs native-theme/src/model/bundled.rs
git commit -m "refactor(native-theme): generate the icon by-name tables from the bundle directories

Closes the 99-of-103 and 76-of-87 gaps by construction; the directory is
the single source of truth."
```

---

### Task 5: Icon bundles: manifest, refresh script, upstream names, Lucide 1.41.0, Material HEAD, 28 new files (§10.2–10.5; rationale §2.10, §2.11, §2.15, §2.21)

**Model:** Opus 5 (`implement` agent) for steps 1–7 and 9–12; Fable 5.1 inline for step 8 and the pre-commit diff review

**Files:**
- Create: `native-theme/icons/SOURCES.toml`, `native-theme/tests/icon_sources.rs`, `scripts/refresh-icons.sh`
- Delete: ten Lucide duplicates, `native-theme/icons/material/star_border.svg`
- Rename: `native-theme/icons/lucide/trash-2.svg` → `trash.svg`
- Add: 14 Lucide + 14 Material files
- Modify: `native-theme/src/model/bundled.rs` (three role-table paths; tests), `scripts/README.md`

**Interfaces:**
- Consumes: the generated tables from Task 4.
- Produces: bundle names the connector's tables (Task 6) return: Lucide `x`, `minus`, `scan`, `grip`, `arrow-up-narrow-wide`, `arrow-down-wide-narrow`, `maximize`, `minimize-2`, `trash`, `battery`, `battery-charging`, `battery-full`, `battery-low`, `battery-medium`, `battery-warning`, `cpu`, `file-text`, `hard-drive`, `memory-stick`, `network`, `pause`, `play`, `rotate-cw`; Material `battery_0_bar`, `battery_charging_full`, `battery_full`, `battery_2_bar`, `battery_4_bar`, `battery_alert`, `memory`, `hard_drive`, `memory_alt`, `lan`, `pause`, `play_arrow`, `rotate_right`, `star_fill1`. Old names `close`, `dash`, `inspect`, `resize-corner`, `sort-ascending`, `sort-descending`, `window-close`, `window-maximize`, `window-minimize`, `window-restore`, `trash-2`, `star_border` stop resolving (§7.1).

After this task the 0.5.1 connector's icon tests may fail (they still return the old names); Task 6 fixes them. Do not run `cargo test -p native-theme-gpui` as a gate here.

- [ ] **Step 1: Write the failing manifest tests** as `native-theme/tests/icon_sources.rs`:

```rust
//! Every bundled SVG must be traceable through `icons/SOURCES.toml`
//! (v0.5.8 spec §10.5): one `[[set]]` rule per directory, one `[[file]]`
//! entry per exception, and nothing else under `icons/`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize)]
struct Manifest {
    set: Vec<SetRule>,
    #[serde(default)]
    file: Vec<FileRule>,
}

#[derive(Deserialize)]
struct SetRule {
    name: String,
    dir: String,
    repository: String,
    #[serde(rename = "ref")]
    git_ref: String,
    path: String,
    license: String,
}

#[derive(Deserialize)]
struct FileRule {
    set: String,
    file: String,
    #[serde(rename = "ref")]
    git_ref: String,
    path: String,
    reason: String,
}

fn icons_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("icons")
}

fn manifest() -> Manifest {
    let text = std::fs::read_to_string(icons_root().join("SOURCES.toml"))
        .expect("native-theme/icons/SOURCES.toml exists");
    toml::from_str(&text).expect("SOURCES.toml parses")
}

fn svg_names(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .expect("set directory")
        .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

#[test]
fn every_directory_under_icons_has_a_set_rule_and_holds_only_svgs() {
    let m = manifest();
    let dirs: Vec<&str> = m.set.iter().map(|s| s.dir.as_str()).collect();
    let mut total = 0;
    for entry in std::fs::read_dir(icons_root()).expect("icons dir") {
        let entry = entry.expect("entry");
        if !entry.path().is_dir() {
            continue; // LICENSE-*.txt and SOURCES.toml
        }
        let dir = entry.file_name().to_string_lossy().into_owned();
        assert!(dirs.contains(&dir.as_str()), "icons/{dir} has no [[set]] rule");
        for name in svg_names(&entry.path()) {
            assert!(name.ends_with(".svg"), "unexpected non-SVG file icons/{dir}/{name}");
            total += 1;
        }
    }
    assert!(total > 0);
}

#[test]
fn set_rules_are_complete() {
    for s in manifest().set {
        assert!(icons_root().join(&s.dir).is_dir(), "icons/{} missing", s.dir);
        assert!(icons_root().join(&s.license).is_file(), "{} missing", s.license);
        assert!(s.repository.starts_with("https://github.com/"), "{}", s.repository);
        assert!(s.path.contains("{name}"), "set {} path has no {{name}} placeholder", s.name);
        assert!(!s.git_ref.is_empty(), "set {} has no ref", s.name);
    }
}

#[test]
fn per_file_exceptions_point_at_existing_files() {
    let m = manifest();
    for f in &m.file {
        let set = m
            .set
            .iter()
            .find(|s| s.name == f.set)
            .unwrap_or_else(|| panic!("exception {} names unknown set {}", f.file, f.set));
        assert!(
            icons_root().join(&set.dir).join(&f.file).is_file(),
            "exception file icons/{}/{} missing",
            set.dir,
            f.file
        );
        assert!(!f.reason.is_empty(), "{} has no reason", f.file);
        assert!(!f.git_ref.is_empty(), "{} has no ref", f.file);
        assert!(f.path.ends_with(".svg"), "{} path is not an svg", f.file);
    }
}

#[test]
fn file_names_follow_each_sets_convention() {
    // Lucide: kebab-case; Material Symbols: snake_case. These are the names
    // the refresh script substitutes into the upstream path.
    for s in manifest().set {
        let allowed: fn(char) -> bool = match s.name.as_str() {
            "lucide" => |c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-',
            "material" => |c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_',
            other => panic!("unknown set {other}"),
        };
        for name in svg_names(&icons_root().join(&s.dir)) {
            let stem = name.strip_suffix(".svg").expect("svg");
            assert!(
                stem.chars().all(allowed),
                "icons/{}/{name} violates the {} naming convention",
                s.dir,
                s.name
            );
        }
    }
}

#[test]
fn retired_names_are_gone() {
    for gone in [
        "lucide/close.svg",
        "lucide/window-close.svg",
        "lucide/dash.svg",
        "lucide/window-minimize.svg",
        "lucide/inspect.svg",
        "lucide/resize-corner.svg",
        "lucide/sort-ascending.svg",
        "lucide/sort-descending.svg",
        "lucide/window-maximize.svg",
        "lucide/window-restore.svg",
        "lucide/trash-2.svg",
        "material/star_border.svg",
    ] {
        assert!(!icons_root().join(gone).exists(), "{gone} should have been removed");
    }
    for present in ["lucide/trash.svg", "lucide/github.svg", "material/star_fill1.svg"] {
        assert!(icons_root().join(present).is_file(), "{present} should exist");
    }
}
```

- [ ] **Step 2: Run them to verify they fail**

```bash
cargo test -p native-theme --test icon_sources
```

Expected: FAIL in `manifest()` with `native-theme/icons/SOURCES.toml exists`, and `retired_names_are_gone` fails on `lucide/close.svg should have been removed`.

- [ ] **Step 3: Delete the ten duplicates, rename `trash-2`, remove `star_border`**

The ten are byte-identical to files already present under Lucide's names (verified 2026-09-05 with `cmp`; spec §1.3):

```bash
L=native-theme/icons/lucide
for pair in close:x window-close:x dash:minus window-minimize:minus inspect:scan resize-corner:grip sort-ascending:arrow-up-narrow-wide sort-descending:arrow-down-wide-narrow window-maximize:maximize window-restore:minimize-2; do
  cmp "$L/${pair%%:*}.svg" "$L/${pair##*:}.svg" || { echo "NOT identical: $pair"; exit 1; }
done
git rm "$L"/close.svg "$L"/window-close.svg "$L"/dash.svg "$L"/window-minimize.svg "$L"/inspect.svg "$L"/resize-corner.svg "$L"/sort-ascending.svg "$L"/sort-descending.svg "$L"/window-maximize.svg "$L"/window-restore.svg
git mv "$L"/trash-2.svg "$L"/trash.svg
git rm native-theme/icons/material/star_border.svg
```

Expected: every `cmp` silent; `ls native-theme/icons/lucide | wc -l` = 93; `ls native-theme/icons/material | wc -l` = 86.

- [ ] **Step 4: Point the three role-table paths at `trash.svg`**

In `native-theme/src/model/bundled.rs` (`lucide_svg`), change the three `include_bytes!("../../icons/lucide/trash-2.svg")` (roles `ActionDelete`, `TrashEmpty`, `TrashFull`) to `include_bytes!("../../icons/lucide/trash.svg")` and the trailing comment `// reuse trash-2` to `// reuse trash`.

- [ ] **Step 5: Write `native-theme/icons/SOURCES.toml`**

```toml
# Provenance of every SVG under native-theme/icons/ (v0.5.8 spec §10.5).
#
# Each directory is covered by exactly one [[set]] rule: repository, git ref,
# and the upstream path pattern with {name} = the file stem. A [[file]] entry
# overrides the ref and path for one file. `scripts/refresh-icons.sh`
# re-downloads everything from this manifest; native-theme/tests/icon_sources.rs
# checks that every directory and exception is described here.

[[set]]
name = "lucide"
dir = "lucide"
repository = "https://github.com/lucide-icons/lucide"
ref = "1.41.0"
path = "icons/{name}.svg"
license = "LICENSE-LUCIDE.txt"

[[set]]
name = "material"
dir = "material"
# Material Symbols, Outlined style, 24px, default weight, no fill. The ref is
# the full commit SHA (the spec and rationale cite its short form 0cbb08816df0):
# raw.githubusercontent.com resolves the short form today, but only the full
# form is the documented stable address.
repository = "https://github.com/google/material-design-icons"
ref = "0cbb08816df07faaae3dca060d4ebb10b66c214f"
path = "symbols/web/{name}/materialsymbolsoutlined/{name}_24px.svg"
license = "LICENSE-MATERIAL.txt"

[[file]]
set = "lucide"
file = "github.svg"
ref = "0.577.0"
path = "icons/github.svg"
reason = "Lucide removed every brand icon in 1.x (commit aa8f74eb) and offers no replacement; the file stays under Lucide's ISC licence from the last tag that carries it"

[[file]]
set = "material"
file = "star_fill1.svg"
ref = "0cbb08816df07faaae3dca060d4ebb10b66c214f"
path = "symbols/web/star/materialsymbolsoutlined/star_fill1_24px.svg"
reason = "filled variant of star; the upstream stem carries _fill1 before _24px, so the set pattern does not apply"
```

- [ ] **Step 6: Write `scripts/refresh-icons.sh`**

```bash
#!/usr/bin/env bash
# Re-download every bundled icon from native-theme/icons/SOURCES.toml, or add
# a new one:  scripts/refresh-icons.sh            (refresh all)
#             scripts/refresh-icons.sh add lucide battery
#             scripts/refresh-icons.sh add material battery_0_bar
# Requires python3 >= 3.11 (tomllib) and network access. Run from anywhere.
set -euo pipefail
cd "$(dirname "$0")/.."
MANIFEST=native-theme/icons/SOURCES.toml
command -v python3 >/dev/null || { echo "python3 is required" >&2; exit 1; }

if [ "${1:-}" = "add" ]; then
  [ $# -eq 3 ] || { echo "usage: $0 add <set> <name>" >&2; exit 1; }
  dir=$(python3 -c 'import sys,tomllib;m=tomllib.load(open(sys.argv[1],"rb"));print(next(s["dir"] for s in m["set"] if s["name"]==sys.argv[2]))' "$MANIFEST" "$2")
  : > "native-theme/icons/$dir/$3.svg"   # an empty file makes the refresh below fetch it
fi

python3 - "$MANIFEST" <<'PY'
import pathlib, sys, tomllib, urllib.request

manifest = pathlib.Path(sys.argv[1])
root = manifest.parent
data = tomllib.loads(manifest.read_text())
exceptions = {(f["set"], f["file"]): f for f in data.get("file", [])}

def raw(repo: str, ref: str, path: str) -> str:
    return repo.replace("https://github.com/", "https://raw.githubusercontent.com/", 1) + f"/{ref}/{path}"

failures = 0
for s in data["set"]:
    for svg in sorted((root / s["dir"]).glob("*.svg")):
        exc = exceptions.get((s["name"], svg.name))
        url = raw(s["repository"], exc["ref"], exc["path"]) if exc else raw(s["repository"], s["ref"], s["path"].replace("{name}", svg.stem))
        try:
            with urllib.request.urlopen(url) as r:
                body = r.read()
        except Exception as e:  # noqa: BLE001 - report and continue
            print(f"FAILED {svg}: {url}: {e}", file=sys.stderr)
            failures += 1
            continue
        if b"<svg" not in body:
            print(f"FAILED {svg}: {url}: not an SVG", file=sys.stderr)
            failures += 1
            continue
        svg.write_bytes(body)
        print(f"{svg} <- {url}")
sys.exit(1 if failures else 0)
PY
```

Then `chmod +x scripts/refresh-icons.sh`.

- [ ] **Step 7: Refresh both sets and add the 28 files**

```bash
# Pre-create the 28 new files empty, then one refresh fetches everything
# (the script's `add` sub-command does the same for a single file).
for n in battery battery-charging battery-full battery-low battery-medium battery-warning cpu file-text hard-drive memory-stick network pause play rotate-cw; do : > "native-theme/icons/lucide/$n.svg"; done
for n in battery_0_bar battery_charging_full battery_full battery_2_bar battery_4_bar battery_alert memory hard_drive memory_alt lan pause play_arrow rotate_right star_fill1; do : > "native-theme/icons/material/$n.svg"; done
./scripts/refresh-icons.sh
ls native-theme/icons/lucide | wc -l; ls native-theme/icons/material | wc -l
git status --short native-theme/icons | grep -c '^A\|^??'
git diff --stat -- native-theme/icons/lucide/github.svg
```

Expected: exit 0 on every run; 107 Lucide files, 100 Material files; 28 added files; `github.svg` byte-identical or whitespace-only different (pinned to 0.577.0). Modified files are the refreshed ones (including `material/warning.svg` and `material/info.svg`, spec §16 Q4 — expected).

Every downloaded file must contain `<svg` (the script checks and exits non-zero otherwise).

- [ ] **Step 8: Visual check of the `close` and `approximate` Material rows** (spec §10.3; judgment step — **done ahead of the task by the controller on 2026-09-06**: rendered side by side, five rows hold and `MemoryStick` moved from `sd_card` to `memory_alt`, already reflected in Step 7's list, spec §10.3 and rationale §2.21/error 56. The implementer only confirms the fetched `memory_alt.svg` renders as a pinned module; no other change expected.) Reference table as checked:

| gpui-kit glyph (`~/.cargo/registry/src/*/gpui-kit-assets-0.6.0/assets/icons/`) | Material candidate |
|---|---|
| `battery.svg` | `battery_0_bar.svg` |
| `battery-low.svg` | `battery_2_bar.svg` |
| `battery-medium.svg` | `battery_4_bar.svg` |
| `cpu.svg` | `memory.svg` (alternative to inspect: `developer_board`) |
| `memory-stick.svg` | `memory_alt.svg` (chosen; `sd_card` rejected as a flash card) |
| `network.svg` | `lan.svg` (alternative: `hub`) |

Record the outcome (kept or changed) in the commit body.

- [ ] **Step 9: Retire the hand-written coverage tests, add the negative test**

In `native-theme/src/model/bundled.rs` `mod tests` delete `lucide_by_name_covers_gpui_icons` and `material_by_name_covers_gpui_icons` (the generated-table tests from Task 4 cover the directories) and add:

```rust
    /// §7.1: the gpui-component-style names and the two retired files no longer
    /// resolve; callers get `None`, never a substitute.
    #[test]
    #[cfg(all(feature = "lucide-icons", feature = "material-icons"))]
    fn retired_bundle_names_return_none() {
        for name in [
            "close", "dash", "inspect", "resize-corner", "sort-ascending", "sort-descending",
            "window-close", "window-maximize", "window-minimize", "window-restore", "trash-2",
        ] {
            assert!(bundled_icon_by_name(name, IconSet::Lucide).is_none(), "{name} should be gone");
        }
        assert!(bundled_icon_by_name("star_border", IconSet::Material).is_none());
        assert!(bundled_icon_by_name("trash", IconSet::Lucide).is_some());
        assert!(bundled_icon_by_name("star_fill1", IconSet::Material).is_some());
    }
```

- [ ] **Step 10: Run the gate**

```bash
cargo test -p native-theme --features material-icons,lucide-icons,system-icons,svg-rasterize
cargo test -p native-theme --test icon_sources
```

Expected: PASS (role coverage, size budgets, generated-table coverage, manifest tests, retired names).

- [ ] **Step 11: Document the script**

Append to `scripts/README.md`:

````markdown
## refresh-icons.sh

Re-downloads every bundled SVG under `native-theme/icons/` from the
provenance manifest `native-theme/icons/SOURCES.toml` (one rule per set, one
per-file exception per set), so a refresh or an addition is reproducible.

```sh
./scripts/refresh-icons.sh                      # refresh every file to the manifest refs
./scripts/refresh-icons.sh add lucide battery   # add a Lucide icon by its upstream name
./scripts/refresh-icons.sh add material lan     # add a Material Symbols icon
```

Requires Python 3.11+ (for `tomllib`) and network access. After a run,
`cargo test -p native-theme --test icon_sources` checks the manifest and
`cargo build -p native-theme` regenerates the by-name tables from the
directories (`native-theme/build.rs`).
````

- [ ] **Step 12: Commit**

```bash
git add native-theme/icons native-theme/src/model/bundled.rs native-theme/tests/icon_sources.rs scripts/refresh-icons.sh scripts/README.md
git commit -m "feat(native-theme)!: icon bundles under upstream names, Lucide 1.41.0, Material HEAD, provenance manifest

Ten Lucide files stored under gpui-component's names were byte-identical
duplicates of files already present under Lucide's names and are deleted;
trash-2 -> trash (Lucide 1.x canonical name, identical glyph); star_border
(a duplicate of star that backed StarOff) removed. 14 + 14 files added for
the gpui-component 0.6.0 icons. icons/SOURCES.toml records every source;
scripts/refresh-icons.sh reproduces the bundle. Breaking for LucideLoader /
MaterialLoader callers using the old names (they now get None)."
```

---

### Task 6: Connector on the 0.6.0 stack: manifest, mechanical first pass, `Option` icon tables with the 15 new variants (§4.1, §5.1, §5.2, §10.1–10.4, §10.6)

**Model:** Opus 5 (`implement` agent)

**Files:**
- Modify: `connectors/native-theme-gpui/Cargo.toml` (replace whole file)
- Modify: `connectors/native-theme-gpui/src/lib.rs` (`ScrollbarShow` → `ScrollbarMode`, lines 96 and 147-152; test `scrollbar_show_from_overlay_mode`)
- Modify: `connectors/native-theme-gpui/src/colors.rs` (lines 285-286, 378-380, tripwire 869-877)
- Modify: `connectors/native-theme-gpui/src/config.rs` (lines 84, 151-152)
- Modify: `connectors/native-theme-gpui/src/icons.rs` (three tables, `bundled_icon_to_image_source`, `ALL_ICON_NAMES`, tests)

**Interfaces:**
- Consumes: Task 5's bundle names.
- Produces: `pub fn lucide_name_for_gpui_icon(icon: IconName) -> Option<&'static str>`, `pub fn material_name_for_gpui_icon(icon: IconName) -> Option<&'static str>`, `pub fn freedesktop_name_for_gpui_icon(icon: IconName, de: LinuxDesktop) -> Option<&'static str>` (Linux); a compiling library on gpui-component 0.6.0 for Tasks 7–11.

Gate progression (the probe of 2026-09-05 measured these counts, §1.4): 14 errors after the manifest → 3 `E0004` after the renames → 0 after the tables.

- [ ] **Step 1: Replace `connectors/native-theme-gpui/Cargo.toml`**

```toml
[package]
name = "native-theme-gpui"
version.workspace = true
edition.workspace = true
license.workspace = true
# NOT inherited: the gpui-pre closure declares a higher floor than the
# workspace. Set to the lowest toolchain that compiles the library (spec §4.4);
# provisional until Task 13 measures it.
rust-version = "1.95"
repository.workspace = true
homepage.workspace = true
keywords = ["theme", "gpui", "gui", "native", "colors"]
categories = ["gui", "config"]
readme = "README.md"
description = "gpui toolkit connector for native-theme"

[package.metadata.docs.rs]
# No `targets` list (spec §4.1, D40): every platform-gated public item of this
# crate is Linux-gated and appears on docs.rs's default target; the other two
# targets would render the same page minus those items at the cost of the
# first-ever cross-target docs build of gpui-pre's macOS and Windows platform
# crates.
all-features = true

[features]
default = ["material-icons", "lucide-icons", "system-icons", "svg-rasterize"]
material-icons = ["native-theme/material-icons"]
lucide-icons = ["native-theme/lucide-icons"]
system-icons = ["native-theme/system-icons"]
svg-rasterize = ["native-theme/svg-rasterize"]

[dependencies]
# GPUI under the package name gpui-component 0.6.0 uses, so both edges unify.
# gpui-component does not re-export gpui, so the connector names it. Caret:
# an exact pin in a library would conflict with any consumer whose other
# dependencies want a later snapshot.
gpui = { package = "gpui-pre", version = "0.3.3" }
gpui-component = "0.6.0"
# For gpui_base::Theme, ScrollbarTheme, ResizableTheme (not re-exported).
gpui-base = "0.6.0"
native-theme = { workspace = true }

[target.'cfg(target_os = "linux")'.dependencies]
native-theme = { workspace = true, features = ["linux"] }

[target.'cfg(target_os = "macos")'.dependencies]
native-theme = { workspace = true, features = ["macos"] }

[target.'cfg(target_os = "windows")'.dependencies]
native-theme = { workspace = true, features = ["windows"] }

[dev-dependencies]
# Headless App for the apply/observer tests (#[gpui::test], TestAppContext).
gpui = { package = "gpui-pre", version = "0.3.3", features = ["test-support"] }
# The showcase uses the facade an application would: application(), init(),
# and the Lucide assets under gpui_kit::assets.
gpui-kit = "0.6.0"
native-theme = { workspace = true, features = ["watch"] }

[target.'cfg(target_os = "macos")'.dev-dependencies]
objc2 = "0.6.4"

[target.'cfg(target_os = "windows")'.dev-dependencies]
image = { version = "0.25.10", default-features = false, features = ["png"] }
windows = { version = "0.62.2", default-features = false, features = [
    "Win32_Foundation",
    "Win32_Graphics_Dwm",
    "Win32_Graphics_Gdi",
    "Win32_UI_WindowsAndMessaging",
] }

[[example]]
name = "showcase-gpui"
```

- [ ] **Step 2: Resolve and count the first-pass errors**

```bash
cargo update
git diff Cargo.lock | grep '^[-+]name' | sort | uniq -c | head -40
cargo check -p native-theme-gpui --lib 2>&1 | grep -c '^error'
```

Expected: the lock diff adds the gpui-pre / gpui-base / gpui-component / gpui-kit closures and drops the gpui 0.2.2 / gpui-component 0.5.1 ones, nothing else (Task 1 already put every other crate at its latest release, so `cargo update` is idempotent for them); the error count is about `14` (the probe measured 14: the rows of §5.1; a later gpui-pre patch may add or remove one). If the lock diff shows other crates, that is the same latest-release policy applied to releases since Task 1. Cargo's MSRV-aware resolver (`resolver = "3"`) resolves against the workspace's lowest `rust-version` (1.88.0 after Task 1; Task 1 measured the floor unchanged), so a crate in the gpui-pre closure whose requirement admits a 1.89-compatible release resolves to that release rather than the newest; expected, and the compile is the gate.

- [ ] **Step 3: Apply the §5.1 renames**

`src/lib.rs`:
- line 96: `use gpui_component::scroll::ScrollbarShow;` → `use gpui_component::scroll::ScrollbarMode;`
- lines 147-152:

```rust
    // §8.3: Scrolling (overlay, auto-hide) when the platform draws overlay
    // scrollbars, Always otherwise.
    theme.scrollbar_mode = if resolved.scrollbar.overlay_mode {
        ScrollbarMode::Scrolling
    } else {
        ScrollbarMode::Always
    };
```

- test `scrollbar_show_from_overlay_mode` → rename to `scrollbar_mode_from_overlay_mode`:

```rust
    #[test]
    fn scrollbar_mode_from_overlay_mode() {
        let resolved = test_resolved();
        let theme = to_theme(&resolved, "Scroll", true, false);
        let expected = if resolved.scrollbar.overlay_mode {
            ScrollbarMode::Scrolling
        } else {
            ScrollbarMode::Always
        };
        assert_eq!(theme.scrollbar_mode, expected);
    }
```

`src/colors.rs`:
- `tc.bullish = c.success;` / `tc.bearish = c.danger;` → `tc.chart_bullish = c.success;` / `tc.chart_bearish = c.danger;`
- delete the three lines `// Accordion hover: 8% accent tint over background.`, `// Matches upstream apply_config fallback pattern (Issue 60).`, `tc.accordion_hover = c.bg.blend(c.accent.opacity(0.08));`
- tripwire: `field_count, 108,` → `field_count, 139,` and the doc line above it accordingly (`ThemeColor` has 139 `Hsla` fields in gpui-component 0.6.0).
- if any `coverage_*` test names `bullish`, `bearish` or `accordion_hover`, rename to `chart_bullish` / `chart_bearish` or drop the line.

`src/config.rs`:
- delete `colors.accordion_hover = h(tc.accordion_hover);`
- `colors.bullish = h(tc.bullish);` / `colors.bearish = h(tc.bearish);` → `colors.chart_bullish = h(tc.chart_bullish);` / `colors.chart_bearish = h(tc.chart_bearish);`

`src/icons.rs`: `IconName::GitHub` → `IconName::Github` at lines 179, 289, 673 and in `ALL_ICON_NAMES` (line ~1283); the showcase's `("GitHub", IconName::GitHub)` is Task 12.

- [ ] **Step 4: Verify exactly the three non-exhaustive errors remain**

```bash
cargo check -p native-theme-gpui --lib 2>&1 | grep '^error' | sort | uniq -c
```

Expected: one line, `3 error[E0004]: non-exhaustive patterns: ...` (the 15 missing variants in the three tables), nothing else.

- [ ] **Step 5: Convert the three tables to `Option` and add the 15 arms**

`lucide_name_for_gpui_icon`: signature `-> Option<&'static str>`; body `Some(match icon { ... })`. Change these existing arms to the real Lucide names (§10.2): `IconName::Close => "x"`, `IconName::Dash => "minus"`, `IconName::Inspector => "scan"`, `IconName::ResizeCorner => "grip"`, `IconName::SortAscending => "arrow-up-narrow-wide"`, `IconName::SortDescending => "arrow-down-wide-narrow"`, `IconName::WindowClose => "x"`, `IconName::WindowMaximize => "maximize"`, `IconName::WindowMinimize => "minus"`, `IconName::WindowRestore => "minimize-2"`. Add, in alphabetical position:

```rust
        IconName::Battery => "battery",
        IconName::BatteryCharging => "battery-charging",
        IconName::BatteryFull => "battery-full",
        IconName::BatteryLow => "battery-low",
        IconName::BatteryMedium => "battery-medium",
        IconName::BatteryWarning => "battery-warning",
        IconName::Cpu => "cpu",
        IconName::FileText => "file-text",
        IconName::HardDrive => "hard-drive",
        IconName::MemoryStick => "memory-stick",
        IconName::Network => "network",
        IconName::Pause => "pause",
        IconName::Play => "play",
        IconName::RotateCw => "rotate-cw",
        // Lucide ships no filled star (star-fill / star-filled absent at 1.41.0);
        // gpui-kit's star-fill.svg is Lucide's `star` with fill="currentColor"
        // added, which a bundled Lucide file cannot express, and the hollow
        // `star` would make Star and StarFill indistinguishable (spec §10.2).
        IconName::StarFill => return None,
```

Doc comment: replace "Covers all 86 gpui-component `IconName` variants." with "Returns `None` where Lucide has no equivalent (today only `StarFill`, spec §10.2); every `Some` is Lucide's own file name (`LucideLoader::new(name)` resolves it)."

`material_name_for_gpui_icon`: signature `-> Option<&'static str>`; body `Some(match icon { ... })`; change `IconName::ALargeSmall => "font_size",` to `IconName::ALargeSmall => "format_size",` (Task 5 stored the file under its upstream name); change `IconName::StarOff => "star_border",` to

```rust
        // Material Symbols has no star-off glyph; the duplicate hollow star that
        // used to back this variant was removed (spec §10.3). None, no substitute.
        IconName::StarOff => return None,
```

and add (§10.3, confidence in the comment):

```rust
        IconName::Battery => "battery_0_bar",            // close
        IconName::BatteryCharging => "battery_charging_full", // exact
        IconName::BatteryFull => "battery_full",         // exact
        IconName::BatteryLow => "battery_2_bar",         // close: one of three bars ↔ two of six
        IconName::BatteryMedium => "battery_4_bar",      // close: two of three ↔ four of six
        IconName::BatteryWarning => "battery_alert",     // exact
        IconName::Cpu => "memory",                       // close
        IconName::FileText => "description",             // exact
        IconName::HardDrive => "hard_drive",             // exact
        IconName::MemoryStick => "memory_alt",           // close: a RAM module with pins, like Lucide's
        IconName::Network => "lan",                      // close
        IconName::Pause => "pause",                      // exact
        IconName::Play => "play_arrow",                  // exact
        IconName::RotateCw => "rotate_right",            // exact
        IconName::StarFill => "star_fill1",              // exact
```

Doc comment: "Returns `None` where Material Symbols has no equivalent (today only `StarOff`); every `Some` is a bundled Material Symbols Outlined 24px file."

`freedesktop_name_for_gpui_icon`: signature `-> Option<&'static str>`; body `Some(match icon { ... })`; add to the "standard names" group:

```rust
        IconName::Battery => "battery",                  // exact
        IconName::FileText => "text-x-generic",          // exact
        IconName::HardDrive => "drive-harddisk",         // exact
        IconName::Network => "network-workgroup",        // close
        IconName::Pause => "media-playback-pause",       // exact
        IconName::Play => "media-playback-start",        // exact
        IconName::RotateCw => "object-rotate-right",     // exact
        IconName::StarFill => "starred",                 // exact: the filled "starred" state
```

and change the existing arm `IconName::Star => "starred",                // exact` to

```rust
        IconName::Star => "non-starred",                 // close: the hollow star, the "not starred" state; `starred` is StarFill's
```

(§10.4; `IconName::StarOff => "non-starred"` stays: the state it means). Then to the "KDE and GNOME differ" group (§10.4):

```rust
        IconName::BatteryCharging => {
            if is_gtk { "battery-full-charging" } else { "battery-100-charging" }
        } // close
        IconName::BatteryFull => {
            if is_gtk { "battery-full" } else { "battery-100" }
        } // exact
        IconName::BatteryLow => {
            if is_gtk { "battery-low" } else { "battery-020" }
        } // close
        IconName::BatteryMedium => {
            if is_gtk { "battery-good" } else { "battery-050" }
        } // close
        IconName::BatteryWarning => {
            if is_gtk { "battery-caution" } else { "battery-010" }
        } // close (GNOME) / approximate (KDE: near-empty level, no alert icon)
        IconName::Cpu => {
            if is_gtk { "computer" } else { "cpu" }
        } // approximate (GNOME) / exact (KDE)
        IconName::MemoryStick => {
            if is_gtk { "media-flash" } else { "memory" }
        } // approximate (GNOME) / exact (KDE: devices/64/memory.svg is a RAM module)
```

`bundled_icon_to_image_source`: the name lookup becomes

```rust
    let name = match icon_set {
        native_theme::theme::IconSet::Lucide => lucide_name_for_gpui_icon(icon)?,
        native_theme::theme::IconSet::Material => material_name_for_gpui_icon(icon)?,
        _ => return None,
    };
```

Module doc table (lines 7-9): `&str` → `Option<&str>`; "gpui-component 0.5" (lines 64, 128) → "gpui-component 0.6"; "Covers all 86" (lines 236, 360) → "Covers all 101 gpui-component 0.6.0 `IconName` variants" where the sentence survives.

- [ ] **Step 6: `ALL_ICON_NAMES` and the tests**

Add the 15 variants to `ALL_ICON_NAMES` in alphabetical position (`Battery`, `BatteryCharging`, `BatteryFull`, `BatteryLow`, `BatteryMedium`, `BatteryWarning`, `Cpu`, `FileText`, `HardDrive`, `MemoryStick`, `Network`, `Pause`, `Play`, `RotateCw`, `StarFill`); tripwire `86` → `101`. Replace `all_icons_have_lucide_mapping` and `all_icons_have_material_mapping` with:

```rust
    fn same_variant(a: &IconName, b: &IconName) -> bool {
        // IconName derives neither PartialEq nor Debug (icon_named! emits Clone only).
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }

    /// Variants a set legitimately lacks (spec §10.1). Every `None` a table
    /// returns must be listed here with its reason; a missing mapping cannot
    /// hide as an intentional one.
    const LUCIDE_NONE_ALLOWED: &[(IconName, &str)] = &[(
        IconName::StarFill,
        "Lucide has no filled star; gpui-kit's star-fill.svg is Lucide's star with fill added",
    )];
    const MATERIAL_NONE_ALLOWED: &[(IconName, &str)] =
        &[(IconName::StarOff, "Material Symbols has no star-off glyph")];

    #[test]
    fn every_none_is_an_allowed_gap() {
        for icon in ALL_ICON_NAMES {
            if lucide_name_for_gpui_icon(icon.clone()).is_none() {
                assert!(
                    LUCIDE_NONE_ALLOWED.iter().any(|(a, _)| same_variant(a, icon)),
                    "an IconName has no Lucide mapping and is not in LUCIDE_NONE_ALLOWED"
                );
            }
            if material_name_for_gpui_icon(icon.clone()).is_none() {
                assert!(
                    MATERIAL_NONE_ALLOWED.iter().any(|(a, _)| same_variant(a, icon)),
                    "an IconName has no Material mapping and is not in MATERIAL_NONE_ALLOWED"
                );
            }
        }
    }

    /// §10.1: a table may only name a file that is actually bundled.
    #[test]
    fn every_some_resolves_in_its_bundle() {
        use native_theme::theme::IconSet;
        for icon in ALL_ICON_NAMES {
            if let Some(name) = lucide_name_for_gpui_icon(icon.clone()) {
                assert!(
                    bundled_icon_to_image_source(icon.clone(), IconSet::Lucide, None, None).is_some(),
                    "Lucide name {name} is not bundled"
                );
            }
            if let Some(name) = material_name_for_gpui_icon(icon.clone()) {
                assert!(
                    bundled_icon_to_image_source(icon.clone(), IconSet::Material, None, None).is_some(),
                    "Material name {name} is not bundled"
                );
            }
        }
    }

    #[test]
    fn lucide_table_returns_lucide_names_for_the_former_gpui_names() {
        assert_eq!(lucide_name_for_gpui_icon(IconName::Close), Some("x"));
        assert_eq!(lucide_name_for_gpui_icon(IconName::WindowClose), Some("x"));
        assert_eq!(lucide_name_for_gpui_icon(IconName::Dash), Some("minus"));
        assert_eq!(lucide_name_for_gpui_icon(IconName::WindowMinimize), Some("minus"));
        assert_eq!(lucide_name_for_gpui_icon(IconName::Inspector), Some("scan"));
        assert_eq!(lucide_name_for_gpui_icon(IconName::ResizeCorner), Some("grip"));
        assert_eq!(lucide_name_for_gpui_icon(IconName::SortAscending), Some("arrow-up-narrow-wide"));
        assert_eq!(lucide_name_for_gpui_icon(IconName::SortDescending), Some("arrow-down-wide-narrow"));
        assert_eq!(lucide_name_for_gpui_icon(IconName::WindowMaximize), Some("maximize"));
        assert_eq!(lucide_name_for_gpui_icon(IconName::WindowRestore), Some("minimize-2"));
        assert_eq!(lucide_name_for_gpui_icon(IconName::StarFill), None);
        assert_eq!(material_name_for_gpui_icon(IconName::StarOff), None);
    }
```

In `freedesktop_mapping_tests`: rename `all_86_gpui_icons_have_mapping_on_kde` / `_on_gnome` to `every_gpui_icon_has_a_freedesktop_name_on_kde` / `_on_gnome` asserting `fd_name.is_some_and(|n| !n.is_empty())`; in `eye_differs_by_de`, `freedesktop_standard_ignores_de`, `xfce_uses_gnome_names` wrap the expected strings in `Some(..)`; in `all_kde_names_resolve_in_breeze` and `gnome_names_resolve_in_adwaita` replace `let fd_name = ...;` with `let Some(fd_name) = freedesktop_name_for_gpui_icon(name.clone(), ...) else { continue };`. Any other existing assertion on a specific old Lucide name takes the new name from Step 5, and any assertion of `Star` → `"starred"` takes `"non-starred"`. Add to `freedesktop_mapping_tests`:

```rust
    /// §10.4: the two star states must not share a glyph.
    #[test]
    fn star_states_have_distinct_freedesktop_names() {
        for de in [LinuxDesktop::Kde, LinuxDesktop::Gnome] {
            assert_eq!(freedesktop_name_for_gpui_icon(IconName::Star, de), Some("non-starred"));
            assert_eq!(freedesktop_name_for_gpui_icon(IconName::StarFill, de), Some("starred"));
        }
    }
```

- [ ] **Step 7: Run the gate**

```bash
cargo check -p native-theme-gpui --lib 2>&1 | grep -c '^error'    # expected 0
cargo test -p native-theme-gpui --lib
```

Expected: 0 errors; all library tests PASS, including `all_kde_names_resolve_in_breeze` (this machine runs KDE with Breeze) and `gnome_names_resolve_in_adwaita` (Adwaita installed), which are the gate for the 15 freedesktop names (§10.4). Do not run `cargo test -p native-theme-gpui` without `--lib`: the showcase does not compile until Task 12.

- [ ] **Step 8: Commit**

```bash
git add Cargo.lock connectors/native-theme-gpui/Cargo.toml connectors/native-theme-gpui/src
git commit -m "feat(gpui)!: move the connector to gpui-component 0.6.0 / gpui-base 0.6.0 / gpui-pre 0.3

Mechanical first pass (ScrollbarShow -> ScrollbarMode, chart_bullish/bearish,
accordion_hover gone, IconName::Github), ThemeColor tripwire 139. The three
icon tables return Option<&'static str>, cover the 15 new IconName variants,
and the Lucide table returns Lucide's own names. StarFill has no Lucide
equivalent and StarOff no Material one, both return None; the freedesktop
table maps Star to non-starred so the two star states differ. The showcase
example is ported in a later commit."
```

---

### Task 7: The complete `ThemeColor` mapping (§6, §5.3; rationale §2.4, §2.25)

**Model:** Opus 5 (`implement` agent)

**Files:**
- Modify: `connectors/native-theme-gpui/src/colors.rs` (`to_theme_color`, new `assign_buttons`, `assign_list_table`, `assign_misc`, tests)
- Modify: `connectors/native-theme-gpui/src/config.rs` (`theme_color_to_config_colors`, tests)
- Modify: `connectors/native-theme-gpui/Cargo.toml` (`[dev-dependencies] serde_json = "1.0.151"`)

**Interfaces:**
- Consumes: `ThemeColor` 0.6.0 fields (`gpui-component 0.6.0 src/theme/theme_color.rs:59-342`); `ThemeColor: Serialize` (`:58`), `Hsla: Serialize + PartialEq` (gpui-pre `src/color.rs:374, 725`).
- Produces: every one of the 139 fields assigned; `ThemeConfigColors` with all 34 new hex exports.

- [ ] **Step 1: Write the failing completeness test** in `colors.rs` `mod tests` (add `serde_json = "1.0.151"` under `[dev-dependencies]` first):

```rust
    /// §6.4: `ThemeColor: Default` is transparent black, so an unassigned
    /// field is invisible at compile time and in most screenshots. Compare
    /// every field with the zero value through serde, which sees all of them.
    #[test]
    fn no_theme_color_field_is_left_at_default() {
        let zero = serde_json::to_value(Hsla::default()).expect("Hsla serialises");
        for (resolved, is_dark) in [(test_resolved(), true), (test_resolved_light(), false)] {
            let tc = to_theme_color(&resolved, is_dark, false);
            let value = serde_json::to_value(tc).expect("ThemeColor serialises");
            let fields = value.as_object().expect("ThemeColor serialises as an object");
            assert_eq!(fields.len(), 139, "serde sees a different field count than the tripwire");
            let unassigned: Vec<&String> =
                fields.iter().filter(|(_, v)| **v == zero).map(|(k, _)| k).collect();
            assert!(
                unassigned.is_empty(),
                "fields left at transparent black (is_dark = {is_dark}): {unassigned:?}"
            );
        }
    }
```

- [ ] **Step 2: Run it to verify it fails**

```bash
cargo test -p native-theme-gpui --lib no_theme_color_field_is_left_at_default
```

Expected: FAIL listing the 28 `button_*` fields plus `status_bar`, `status_bar_border`, `table_foot`, `table_foot_foreground` (32 names; `chart_bullish` / `chart_bearish` were assigned in Task 6).

- [ ] **Step 3: Assign the 32 fields**

In `to_theme_color`, after `assign_status(&mut tc, &c, is_dark);` insert `assign_buttons(&mut tc);` (it copies from fields the three preceding calls set). Add the function:

```rust
/// The 28 `button_*` fields gpui-component 0.6.0 reads for `Button`
/// (`src/button/button.rs:884-949`) take the values the semantic fields their
/// variant used in 0.5.1 (`0.5.1 src/button/button.rs:630-635, 924-929`), so a
/// native theme's solid button surfaces render as before (spec §6.1). Nothing
/// new is read; upstream's alternative is a tinted house style (rationale §2.4).
fn assign_buttons(tc: &mut ThemeColor) {
    // Ordinary push button = secondary; native themes do not distinguish a
    // "default" from a "secondary" button, so both groups take the same values.
    tc.button = tc.secondary;
    tc.button_hover = tc.secondary_hover;
    tc.button_active = tc.secondary_active;
    tc.button_foreground = tc.secondary_foreground;
    tc.button_secondary = tc.secondary;
    tc.button_secondary_hover = tc.secondary_hover;
    tc.button_secondary_active = tc.secondary_active;
    tc.button_secondary_foreground = tc.secondary_foreground;

    tc.button_primary = tc.primary;
    tc.button_primary_hover = tc.primary_hover;
    tc.button_primary_active = tc.primary_active;
    tc.button_primary_foreground = tc.primary_foreground;

    tc.button_danger = tc.danger;
    tc.button_danger_hover = tc.danger_hover;
    tc.button_danger_active = tc.danger_active;
    tc.button_danger_foreground = tc.danger_foreground;

    tc.button_info = tc.info;
    tc.button_info_hover = tc.info_hover;
    tc.button_info_active = tc.info_active;
    tc.button_info_foreground = tc.info_foreground;

    tc.button_success = tc.success;
    tc.button_success_hover = tc.success_hover;
    tc.button_success_active = tc.success_active;
    tc.button_success_foreground = tc.success_foreground;

    tc.button_warning = tc.warning;
    tc.button_warning_hover = tc.warning_hover;
    tc.button_warning_active = tc.warning_active;
    tc.button_warning_foreground = tc.warning_foreground;
}
```

In `assign_list_table`, after `tc.table_row_border = c.border;`:

```rust
    // Derivation (spec §6.2): the footer mirrors the header; ListTheme has no
    // footer field and transparent black is not a colour.
    tc.table_foot = tc.table_head;
    tc.table_foot_foreground = tc.table_head_foreground;
```

In `assign_misc`, after the `tc.drop_target = ...;` line:

```rust
    // Status bar: direct sources (spec §6.2).
    tc.status_bar = rgba_to_hsla(resolved.status_bar.background_color);
    tc.status_bar_border = rgba_to_hsla(resolved.status_bar.border.color);
```

Update the module doc (`colors.rs:1-6`) and the `to_theme_color` doc (`:133`) from 108 to 139 fields.

- [ ] **Step 3b: `hsla_to_hex` keeps alpha (D36)**

`ThemeConfigColors` holds hex strings. gpui's `Rgba::try_from(&str)` accepts `#rrggbbaa` (gpui-pre 0.3.3 `src/color.rs:224-262`) and gpui-component's `try_parse_color` delegates to it for `#` strings (`src/theme/color.rs:677-680`); without alpha in the export, `overlay`, `drag_border` and `drop_target` turn opaque after `Theme::change`. Replace `hsla_to_hex` in `colors.rs` (and its doc comment "Alpha is discarded"):

```rust
/// Convert an `Hsla` colour to a hex string: `#rrggbb` when opaque, `#rrggbbaa`
/// when the alpha is below 1 (gpui parses both, gpui-pre 0.3.3
/// `src/color.rs:224-262`). Alpha is quantised to 8 bits like the channels.
pub(crate) fn hsla_to_hex(c: Hsla) -> String {
    let rgba: gpui::Rgba = c.into();
    let channel = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    let (r, g, b, a) = (channel(rgba.r), channel(rgba.g), channel(rgba.b), channel(rgba.a));
    if a == u8::MAX {
        format!("#{r:02x}{g:02x}{b:02x}")
    } else {
        format!("#{r:02x}{g:02x}{b:02x}{a:02x}")
    }
}
```

Test, next to `hsla_to_hex_roundtrip` (which stays: opaque colours are still seven characters):

```rust
    /// D36: alpha below 1 survives the config round trip as `#rrggbbaa`.
    #[test]
    fn hsla_to_hex_keeps_alpha_below_one() {
        let translucent = Hsla { h: 0.0, s: 0.0, l: 0.0, a: 0.65 };
        let hex = hsla_to_hex(translucent);
        assert_eq!(hex, "#000000a6", "0.65 × 255 rounds to 166 = a6");
        let back = gpui::Rgba::try_from(hex.as_str()).expect("gpui parses #rrggbbaa");
        assert!((back.a - 0.65).abs() < 1.0 / 255.0);
        assert_eq!(hsla_to_hex(Hsla { a: 1.0, ..translucent }), "#000000");
    }
```

- [ ] **Step 4: Export the 34 new or renamed fields as hex in `config.rs`**

In `theme_color_to_config_colors`, after `colors.window_border = h(tc.window_border);` add one `colors.<field> = h(tc.<field>);` line for each of: `button`, `button_hover`, `button_active`, `button_foreground`, `button_secondary`, `button_secondary_hover`, `button_secondary_active`, `button_secondary_foreground`, `button_primary`, `button_primary_hover`, `button_primary_active`, `button_primary_foreground`, `button_danger`, `button_danger_hover`, `button_danger_active`, `button_danger_foreground`, `button_info`, `button_info_hover`, `button_info_active`, `button_info_foreground`, `button_success`, `button_success_hover`, `button_success_active`, `button_success_foreground`, `button_warning`, `button_warning_hover`, `button_warning_active`, `button_warning_foreground`, `status_bar`, `status_bar_border`, `table_foot`, `table_foot_foreground` (`chart_bullish` / `chart_bearish` are already there from Task 6). Update the doc comments from 108 to 139 fields and the note that the 12 private base colours plus `group_box_title_foreground` stay `None`.

Add the test:

```rust
    /// §5.3: the config copy carries every new field, so `Theme::change`
    /// reproduces the solid button surfaces instead of upstream's tint.
    #[test]
    fn theme_config_colors_cover_the_0_6_fields() {
        let resolved = test_resolved();
        let config = to_theme_config(&resolved, "New", GpuiThemeMode::Dark);
        let c = &config.colors;
        for (name, value) in [
            ("button", &c.button),
            ("button_foreground", &c.button_foreground),
            ("button_primary", &c.button_primary),
            ("button_primary_foreground", &c.button_primary_foreground),
            ("button_secondary_active", &c.button_secondary_active),
            ("button_danger_hover", &c.button_danger_hover),
            ("button_info_active", &c.button_info_active),
            ("button_success_foreground", &c.button_success_foreground),
            ("button_warning", &c.button_warning),
            ("chart_bullish", &c.chart_bullish),
            ("chart_bearish", &c.chart_bearish),
            ("status_bar", &c.status_bar),
            ("status_bar_border", &c.status_bar_border),
            ("table_foot", &c.table_foot),
            ("table_foot_foreground", &c.table_foot_foreground),
        ] {
            assert!(value.is_some(), "config colour {name} not exported");
        }
        // D36: drag_border is primary at alpha 0.65, so its export carries alpha.
        assert_eq!(
            c.drag_border.as_deref().map(str::len),
            Some(9),
            "translucent colours are exported as #rrggbbaa"
        );
    }
```

(The test names a representative from every group; the completeness of the 28 `button_*` lines is checked by reading the list above against `ThemeConfigColors`'s field list, `gpui-component 0.6.0 src/theme/schema.rs:249`.)

- [ ] **Step 5: Run the gate**

```bash
cargo test -p native-theme-gpui --lib
```

Expected: PASS, including `no_theme_color_field_is_left_at_default`, `theme_color_field_count_tripwire` (139) and the config test.

- [ ] **Step 6: Commit**

```bash
git add connectors/native-theme-gpui/Cargo.toml Cargo.lock connectors/native-theme-gpui/src/colors.rs connectors/native-theme-gpui/src/config.rs
git commit -m "feat(gpui): map all 139 ThemeColor fields; button_* take the 0.5.1 semantic values

28 button_* fields copy secondary/primary/status; table_foot mirrors
table_head; status_bar from StatusBarTheme. A serde-based test fails on any
field left at transparent black."
```

---

### Task 8: Accessibility as an input: `to_theme` / `from_preset` signatures, text scaling, `focus_ring` (§7, §3.4, §8.3; rationale §2.12)

**Model:** Opus 5 (`implement` agent)

**Files:**
- Modify: `connectors/native-theme-gpui/src/lib.rs` (`to_theme`, `from_preset`, `from_system`, `SystemThemeExt`, crate docs, tests)
- Modify: `connectors/native-theme-gpui/src/config.rs` (`to_theme_config` takes `prefs`)

**Interfaces:**
- Consumes: `native_theme::AccessibilityPreferences { text_scaling_factor: f32, reduce_motion: bool, high_contrast: bool, reduce_transparency: bool }` (`native-theme/src/lib.rs:230`); `Theme.focus_ring: bool`, `Theme.font_size`, `Theme.mono_font_size` (gpui-component 0.6.0 `src/theme/mod.rs:87-146`).
- Produces (breaking, §7.1):
  - `pub fn to_theme(resolved: &ResolvedTheme, name: &str, is_dark: bool, prefs: &AccessibilityPreferences) -> GpuiTheme`
  - `pub fn from_preset(name: &str, is_dark: bool, prefs: &AccessibilityPreferences) -> Result<(GpuiTheme, ResolvedTheme)>`
  - `pub use native_theme::AccessibilityPreferences;`
  - `pub(crate) fn text_scale_factor(prefs: &AccessibilityPreferences) -> f32` (used by Task 11)
  - `pub fn config::to_theme_config(resolved, name, mode, prefs: &AccessibilityPreferences) -> ThemeConfig`

- [ ] **Step 1: Write the failing tests** in `lib.rs` `mod tests`:

```rust
    fn scaled(factor: f32) -> AccessibilityPreferences {
        AccessibilityPreferences {
            text_scaling_factor: factor,
            ..AccessibilityPreferences::default()
        }
    }

    /// §3.4: font sizes carry the factor; the config copies too, so
    /// `Theme::change` reproduces them.
    #[test]
    fn to_theme_scales_font_sizes_by_the_text_scaling_factor() {
        let resolved = test_resolved();
        let theme = to_theme(&resolved, "Scaled", true, &scaled(1.5));
        assert_eq!(theme.font_size, px(resolved.defaults.font.size * 1.5));
        assert_eq!(theme.mono_font_size, px(resolved.defaults.mono_font.size * 1.5));
        assert_eq!(theme.dark_theme.font_size, Some(resolved.defaults.font.size * 1.5));
        assert_eq!(theme.dark_theme.mono_font_size, Some(resolved.defaults.mono_font.size * 1.5));
    }

    /// §7.2: a non-finite or non-positive factor means "no scaling".
    #[test]
    fn to_theme_ignores_a_degenerate_text_scaling_factor() {
        let resolved = test_resolved();
        for factor in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            let theme = to_theme(&resolved, "Degenerate", true, &scaled(factor));
            assert_eq!(theme.font_size, px(resolved.defaults.font.size), "factor {factor}");
        }
    }

    /// §8.3: only the flag has a receiver; a zero-width ring is not drawn.
    #[test]
    fn focus_ring_follows_focus_ring_width() {
        let resolved = test_resolved();
        let theme = to_theme(&resolved, "Ring", true, &AccessibilityPreferences::default());
        assert_eq!(theme.focus_ring, resolved.defaults.focus_ring_width > 0.0);
    }

    #[test]
    fn from_preset_takes_preferences() {
        let (theme, resolved) =
            from_preset("catppuccin-latte", false, &scaled(1.5)).expect("preset should load");
        assert_eq!(theme.font_size, px(resolved.defaults.font.size * 1.5));
    }
```

and in `config.rs` `mod tests` (add `use gpui_component::highlighter::HighlightTheme;`):

```rust
    /// D41: the config carries upstream's default highlighter style for its
    /// mode, so `Theme::change` to this mode switches code highlighting too.
    #[test]
    fn to_theme_config_carries_the_default_highlight_style_for_its_mode() {
        let resolved = test_resolved();
        let prefs = AccessibilityPreferences::default();
        let dark = to_theme_config(&resolved, "H", GpuiThemeMode::Dark, &prefs);
        let light = to_theme_config(&resolved, "H", GpuiThemeMode::Light, &prefs);
        assert_eq!(dark.highlight.as_ref(), Some(&HighlightTheme::default_dark().style));
        assert_eq!(light.highlight.as_ref(), Some(&HighlightTheme::default_light().style));
    }
```

- [ ] **Step 2: Run them to verify they fail**

```bash
cargo test -p native-theme-gpui --lib to_theme_scales
```

Expected: FAIL to compile (`to_theme` takes a `bool`, `from_preset` takes two arguments).

- [ ] **Step 3: Change the signatures and implement scaling**

In `lib.rs`, next to the other re-exports: `pub use native_theme::AccessibilityPreferences;`. Add:

```rust
/// Text-scaling multiplier from the preferences: the factor when it is finite
/// and positive, else `1.0` (spec §7.2). Named `text_scale_factor` because
/// `text_scale(&ResolvedTheme)`, the public typography-scale accessor at
/// `lib.rs:367`, already owns the shorter name.
pub(crate) fn text_scale_factor(prefs: &AccessibilityPreferences) -> f32 {
    let s = prefs.text_scaling_factor;
    if s.is_finite() && s > 0.0 { s } else { 1.0 }
}
```

`to_theme` becomes:

```rust
#[must_use = "this returns the theme; it does not apply it"]
pub fn to_theme(
    resolved: &ResolvedTheme,
    name: &str,
    is_dark: bool,
    prefs: &AccessibilityPreferences,
) -> GpuiTheme {
    let s = text_scale_factor(prefs);
    let theme_color = colors::to_theme_color(resolved, is_dark, prefs.reduce_transparency);
    let mode = if is_dark { GpuiThemeMode::Dark } else { GpuiThemeMode::Light };
    let d = &resolved.defaults;

    let mut theme = GpuiTheme::from(&theme_color);
    theme.mode = mode;
    theme.font_family = SharedString::from(d.font.family.clone());
    // §3.4: Root sets the window rem to font_size (gpui-component 0.6.0
    // src/root.rs:579), so scaling these two sizes scales every rem-relative
    // size in gpui-component, as the platform toolkit scales its own text.
    theme.font_size = px(d.font.size * s);
    theme.mono_font_family = SharedString::from(d.mono_font.family.clone());
    theme.mono_font_size = px(d.mono_font.size * s);
    theme.radius = px(d.border.corner_radius.max(0.0));
    theme.radius_lg = px(d.border.corner_radius_lg.max(0.0));
    theme.shadow = d.border.shadow_enabled;
    // §8.3: the ring is drawn only when the theme gives it a width; its width
    // and offset have no receiver (derived from the element's border upstream).
    theme.focus_ring = d.focus_ring_width > 0.0;
    theme.scrollbar_mode = if resolved.scrollbar.overlay_mode {
        ScrollbarMode::Scrolling
    } else {
        ScrollbarMode::Always
    };
    theme.highlight_theme = if is_dark {
        gpui_component::highlighter::HighlightTheme::default_dark()
    } else {
        gpui_component::highlighter::HighlightTheme::default_light()
    };

    let config: Rc<_> = Rc::new(config::to_theme_config(resolved, name, mode, prefs));
    if mode == GpuiThemeMode::Dark {
        theme.dark_theme = config;
    } else {
        theme.light_theme = config;
    }
    theme
}
```

Keep the existing comments about `Theme.transparent` and `highlight_theme`. `from_preset`:

```rust
#[must_use = "this returns the theme; it does not apply it"]
pub fn from_preset(
    name: &str,
    is_dark: bool,
    prefs: &AccessibilityPreferences,
) -> Result<(GpuiTheme, ResolvedTheme)> {
    let spec = Theme::preset(name)?;
    let display_name = spec.name.clone();
    let variant = spec.into_variant(if is_dark { ColorMode::Dark } else { ColorMode::Light })?;
    let resolved = variant.resolve_system()?;
    let theme = to_theme(&resolved, &display_name, is_dark, prefs);
    Ok((theme, resolved))
}
```

Its doc gains: "Pass `&AccessibilityPreferences::default()` for no scaling, or `&AccessibilityPreferences::from_system()` to honour the OS preferences under a preset (spec §7.1)."

`from_system`: replace `let reduce_transparency = sys.accessibility.reduce_transparency;` and the call with `let theme = to_theme(&resolved, &name, is_dark, &sys.accessibility);` (`SystemTheme` has no `Drop`, so partial moves are fine: `let accessibility = sys.accessibility;` next to `let name = sys.name;`, then `to_theme(&resolved, &name, is_dark, &accessibility)`). `SystemThemeExt::to_gpui_theme`: `to_theme(self.pick(self.mode), &self.name, self.mode.is_dark(), &self.accessibility)`.

`config.rs`:

```rust
pub fn to_theme_config(
    resolved: &ResolvedTheme,
    name: &str,
    mode: GpuiThemeMode,
    prefs: &native_theme::AccessibilityPreferences,
) -> ThemeConfig {
    let d = &resolved.defaults;
    let is_dark = mode.is_dark();
    let s = crate::text_scale_factor(prefs);
    let radius = d.border.corner_radius.max(0.0).round() as usize;
    let radius_lg = d.border.corner_radius_lg.max(0.0).round() as usize;
    let tc = to_theme_color(resolved, is_dark, prefs.reduce_transparency);
    let colors = theme_color_to_config_colors(&tc);
    // D41: upstream's own default highlighter style for this mode. Theme::change
    // installs a config's highlight as highlight_theme only when it is Some
    // (gpui-component 0.6.0 src/theme/schema.rs:1066-1073) and otherwise keeps
    // the previous mode's; to_theme sets Theme.highlight_theme to this same
    // default directly, so both paths agree.
    let highlight = if is_dark {
        gpui_component::highlighter::HighlightTheme::default_dark()
    } else {
        gpui_component::highlighter::HighlightTheme::default_light()
    };
    ThemeConfig {
        name: SharedString::from(name.to_string()),
        mode,
        font_family: Some(SharedString::from(d.font.family.clone())),
        // Scaled (§3.4, §5.3) so Theme::change reproduces the scaled sizes.
        font_size: Some(d.font.size * s),
        mono_font_family: Some(SharedString::from(d.mono_font.family.clone())),
        mono_font_size: Some(d.mono_font.size * s),
        radius: Some(radius),
        radius_lg: Some(radius_lg),
        shadow: Some(d.border.shadow_enabled),
        colors,
        highlight: Some(highlight.style.clone()),
        ..ThemeConfig::default()
    }
}
```

(Delete the existing comments saying `highlight` is left at `None`, `config.rs:24-27` and `:56-58`; keep the `is_default` comment.)

- [ ] **Step 4: Update every call site and doc example**

Tests in `lib.rs`, `colors.rs` (none call `to_theme`), `config.rs` (`to_theme_config(.., &AccessibilityPreferences::default())`), and the `lib.rs` tests: `to_theme(&resolved, "..", true, false)` → `to_theme(&resolved, "..", true, &AccessibilityPreferences::default())`; `from_preset(name, dark)` → `from_preset(name, dark, &AccessibilityPreferences::default())`; `from_system_matches_manual_path` passes `&sys.accessibility`. The `ignore` doc examples at the top of `lib.rs` and on `to_theme` / `from_preset` show the new signatures (mechanical: same calls, new arguments). The crate-level "Theme Field Coverage" table and the "Why the gap" paragraph are rewritten in Task 15 (documentation wording, Fable); leave them untouched here.

- [ ] **Step 5: Run the gate**

```bash
cargo test -p native-theme-gpui --lib
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add connectors/native-theme-gpui/src/lib.rs connectors/native-theme-gpui/src/config.rs
git commit -m "feat(gpui)!: to_theme and from_preset take &AccessibilityPreferences; text scaling reaches rem

The connector passed only reduce_transparency and silently dropped the
platform's text-scaling factor. font_size and mono_font_size (and their
ThemeConfig copies) now carry it; Root sets rem to font_size, so every
rem-relative size in gpui-component scales. focus_ring = focus_ring_width > 0."
```

---

### Task 9: `base_layer`: scrollbar geometry and resize-handle colours for gpui-base (§8.2, §8.3; rationale §2.20, §5.1, §5.2)

**Model:** Opus 5 (`implement` agent)

**Files:**
- Create: `connectors/native-theme-gpui/src/base_layer.rs`
- Modify: `connectors/native-theme-gpui/src/lib.rs` (`pub mod base_layer;`)

**Interfaces:**
- Consumes: `ResolvedTheme.scrollbar { track_color, thumb_color, thumb_hover_color, thumb_active_color: Option<Rgba> (soft option, unset in the *-live presets), groove_width, min_thumb_length, thumb_width, overlay_mode }`, `.splitter { divider_color, hover_color }`, `.defaults.border { corner_radius, color }`; gpui-base 0.6.0 `ScrollbarStyles` / `ScrollbarTrackStyle` / `ScrollbarThumbStyle` builders (`src/scrollbar.rs:589-700`, re-exported by `gpui_component::scroll`), `gpui_base::{Theme, ResizableTheme}` (`src/theme.rs:17-112`, exported at the crate root `src/lib.rs:173`).
- Produces:
  - `pub struct ScrollbarGeometry { track_width, thumb_width, thumb_inset, thumb_radius, min_thumb_length: Pixels; track, track_active_border, thumb, thumb_hover, thumb_active: Hsla }` deriving `Debug, Clone, PartialEq`
  - `pub fn scrollbar_geometry(resolved: &ResolvedTheme) -> ScrollbarGeometry`
  - `pub fn scrollbar_styles(g: &ScrollbarGeometry) -> ScrollbarStyles`
  - `pub fn resizable_theme(resolved: &ResolvedTheme) -> ResizableTheme`
  - `pub fn apply_overrides(g: &ScrollbarGeometry, r: ResizableTheme, cx: &mut App)`

- [ ] **Step 1: Write the failing tests** (create the file with only the test module first, and add `pub mod base_layer;` to `lib.rs`):

```rust
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::colors::rgba_to_hsla;
    use crate::{ColorMode, ResolvedTheme, Theme};

    fn resolved(preset: &str, mode: ColorMode) -> ResolvedTheme {
        Theme::preset(preset)
            .expect("preset")
            .into_variant(mode)
            .expect("variant")
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .expect("resolves")
    }

    /// §8.2: every receiver value is a theme field or the centring derivation.
    #[test]
    fn scrollbar_geometry_matches_theme_fields() {
        for (preset, mode) in [("catppuccin-mocha", ColorMode::Dark), ("catppuccin-latte", ColorMode::Light)] {
            let r = resolved(preset, mode);
            let sb = &r.scrollbar;
            let g = scrollbar_geometry(&r);
            assert_eq!(g.track_width, px(sb.groove_width));
            assert_eq!(g.thumb_width, px(sb.thumb_width));
            assert_eq!(g.thumb_inset, px(((sb.groove_width - sb.thumb_width) / 2.0).max(0.0)));
            assert_eq!(g.thumb_radius, px(r.defaults.border.corner_radius.max(0.0)));
            assert_eq!(g.min_thumb_length, px(sb.min_thumb_length));
            assert_eq!(g.track, rgba_to_hsla(sb.track_color));
            assert_eq!(g.track_active_border, rgba_to_hsla(r.defaults.border.color));
            assert_eq!(g.thumb, rgba_to_hsla(sb.thumb_color));
            assert_eq!(g.thumb_hover, rgba_to_hsla(sb.thumb_hover_color));
            assert_eq!(
                g.thumb_active,
                rgba_to_hsla(sb.thumb_active_color.unwrap_or(sb.thumb_hover_color))
            );
        }
    }

    /// `thumb_active_color` is a soft option (unset in the `*-live` presets); the
    /// active thumb then takes the hover colour, upstream's own choice for that slot.
    #[test]
    fn thumb_active_falls_back_to_hover_when_unset() {
        let mut r = resolved("catppuccin-mocha", ColorMode::Dark);
        r.scrollbar.thumb_active_color = None;
        let g = scrollbar_geometry(&r);
        assert_eq!(g.thumb_active, g.thumb_hover);
    }

    /// The inset derivation clamps at zero when the thumb fills the groove.
    #[test]
    fn thumb_inset_is_clamped_when_thumb_fills_the_groove() {
        let mut r = resolved("catppuccin-mocha", ColorMode::Dark);
        r.scrollbar.thumb_width = r.scrollbar.groove_width + 4.0;
        assert_eq!(scrollbar_geometry(&r).thumb_inset, px(0.0));
        r.scrollbar.thumb_width = r.scrollbar.groove_width;
        assert_eq!(scrollbar_geometry(&r).thumb_inset, px(0.0));
    }

    /// §8.3: upstream's two slots take the splitter colours.
    #[test]
    fn resizable_theme_uses_splitter_colours() {
        let r = resolved("catppuccin-mocha", ColorMode::Dark);
        let t = resizable_theme(&r);
        assert_eq!(t.handle, Some(rgba_to_hsla(r.splitter.divider_color)));
        assert_eq!(t.active_handle, Some(rgba_to_hsla(r.splitter.hover_color)));
    }

    /// `ScrollbarStyles` is opaque (private fields, `Clone + Default` only);
    /// the conversion is one setter per field, so the test only proves it builds.
    #[test]
    fn scrollbar_styles_builds() {
        let r = resolved("catppuccin-mocha", ColorMode::Dark);
        let _styles: ScrollbarStyles = scrollbar_styles(&scrollbar_geometry(&r));
    }
}
```

- [ ] **Step 2: Run them to verify they fail**

```bash
cargo test -p native-theme-gpui --lib base_layer
```

Expected: FAIL to compile (`scrollbar_geometry` etc. not found).

- [ ] **Step 3: Implement the module** (above the test module):

```rust
//! Native values for the gpui-base layer, which paints scrollbars and resize
//! handles without going through gpui-component (spec §8.2, §8.3).
//!
//! gpui-component's `Theme::change`, `sync_system_appearance` and `sync_base`
//! rebuild `gpui_base::Theme` with fixed scrollbar styles (gpui-component 0.6.0
//! `src/theme/mod.rs:268-300`), so these values are written after every such
//! rebuild by the observer `apply` installs (`crate::apply`, spec §3.3).

use gpui::{App, Hsla, Pixels, px};
use gpui_base::{ResizableTheme, Theme as BaseTheme};
use gpui_component::scroll::ScrollbarStyles;
use native_theme::theme::ResolvedTheme;

use crate::colors::rgba_to_hsla;

/// The scrollbar values written onto gpui-base, computed from a
/// [`ResolvedTheme`].
///
/// Exists because gpui-base's style structs have private fields and derive
/// only `Clone, Default` (gpui-base 0.6.0 `src/scrollbar.rs:589, 616, 655`),
/// so the geometry is computed into this inspectable struct, tested, and then
/// converted with one setter per field by [`scrollbar_styles`].
#[derive(Debug, Clone, PartialEq)]
pub struct ScrollbarGeometry {
    /// `scrollbar.groove_width`, applied to all three track states.
    pub track_width: Pixels,
    /// `scrollbar.thumb_width`, applied to all three thumb states.
    pub thumb_width: Pixels,
    /// `((groove_width − thumb_width) / 2).max(0)`: centres the thumb, which
    /// gpui-base anchors `inset` from the track's outer edge
    /// (gpui-base 0.6.0 `src/scrollbar.rs:1391-1400`).
    pub thumb_inset: Pixels,
    /// `defaults.border.corner_radius.max(0)`, mirroring upstream's projection
    /// (gpui-component 0.6.0 `src/theme/mod.rs:284-293`); the theme has no
    /// scrollbar radius and platform-facts records none.
    pub thumb_radius: Pixels,
    /// `scrollbar.min_thumb_length`.
    pub min_thumb_length: Pixels,
    /// `scrollbar.track_color`, all three track states (upstream uses one colour).
    pub track: Hsla,
    /// `defaults.border.color` for the active track border, mirroring upstream
    /// (`src/theme/mod.rs:283`).
    pub track_active_border: Hsla,
    /// `scrollbar.thumb_color`.
    pub thumb: Hsla,
    /// `scrollbar.thumb_hover_color`.
    pub thumb_hover: Hsla,
    /// `scrollbar.thumb_active_color`, or `thumb_hover_color` when the theme
    /// leaves it unset (a soft option, `None` in the `*-live` presets); upstream
    /// itself puts the hover colour in the active slot (`theme/mod.rs:291-295`).
    pub thumb_active: Hsla,
}

/// Compute the scrollbar values (spec §8.2).
#[must_use]
pub fn scrollbar_geometry(resolved: &ResolvedTheme) -> ScrollbarGeometry {
    let sb = &resolved.scrollbar;
    let d = &resolved.defaults;
    ScrollbarGeometry {
        track_width: px(sb.groove_width),
        thumb_width: px(sb.thumb_width),
        thumb_inset: px(((sb.groove_width - sb.thumb_width) / 2.0).max(0.0)),
        thumb_radius: px(d.border.corner_radius.max(0.0)),
        min_thumb_length: px(sb.min_thumb_length),
        track: rgba_to_hsla(sb.track_color),
        track_active_border: rgba_to_hsla(d.border.color),
        thumb: rgba_to_hsla(sb.thumb_color),
        thumb_hover: rgba_to_hsla(sb.thumb_hover_color),
        thumb_active: rgba_to_hsla(sb.thumb_active_color.unwrap_or(sb.thumb_hover_color)),
    }
}

/// One setter per field onto gpui-base's builders
/// (gpui-base 0.6.0 `src/scrollbar.rs:597-646`).
#[must_use]
pub fn scrollbar_styles(g: &ScrollbarGeometry) -> ScrollbarStyles {
    ScrollbarStyles::default()
        .track(|s| s.bg(g.track).width(g.track_width))
        .track_hover(|s| s.bg(g.track).width(g.track_width))
        .track_active(|s| {
            s.bg(g.track)
                .border_color(g.track_active_border)
                .width(g.track_width)
        })
        .thumb(|s| {
            s.bg(g.thumb)
                .width(g.thumb_width)
                .inset(g.thumb_inset)
                .radius(g.thumb_radius)
                .min_length(g.min_thumb_length)
        })
        .thumb_hover(|s| {
            s.bg(g.thumb_hover)
                .width(g.thumb_width)
                .inset(g.thumb_inset)
                .radius(g.thumb_radius)
                .min_length(g.min_thumb_length)
        })
        .thumb_active(|s| {
            s.bg(g.thumb_active)
                .width(g.thumb_width)
                .inset(g.thumb_inset)
                .radius(g.thumb_radius)
                .min_length(g.min_thumb_length)
        })
}

/// Resize-handle colours from the splitter (spec §8.3). Upstream projects
/// `border` / `drag_border` into the same two slots
/// (gpui-component 0.6.0 `src/theme/mod.rs:296-298`); the handle width is a
/// constant upstream (gpui-base `src/resizable/resize_handle.rs:12`), Tier U.
#[must_use]
pub fn resizable_theme(resolved: &ResolvedTheme) -> ResizableTheme {
    ResizableTheme {
        handle: Some(rgba_to_hsla(resolved.splitter.divider_color)),
        active_handle: Some(rgba_to_hsla(resolved.splitter.hover_color)),
    }
}

/// Write both onto `gpui_base::Theme`, keeping the scrollbar `mode()` and
/// `motion()` upstream projected. Never panics: `gpui_base::Theme::global_mut`
/// creates a default when the global is absent (gpui-base 0.6.0
/// `src/theme.rs:31-36`).
///
/// Public so an application that writes the base theme itself can restore the
/// native values; the observer `crate::apply` installs calls the same function.
pub fn apply_overrides(g: &ScrollbarGeometry, r: ResizableTheme, cx: &mut App) {
    let styles = scrollbar_styles(g);
    let base = BaseTheme::global_mut(cx);
    base.scrollbar = base.scrollbar.clone().with_styles(styles);
    base.resizable = r;
}
```

- [ ] **Step 4: Run the gate**

```bash
cargo test -p native-theme-gpui --lib base_layer
cargo clippy -p native-theme-gpui --lib -- -D warnings
```

Expected: PASS; no clippy warnings.

- [ ] **Step 5: Commit**

```bash
git add connectors/native-theme-gpui/src/base_layer.rs connectors/native-theme-gpui/src/lib.rs
git commit -m "feat(gpui): base_layer module: native scrollbar geometry and resize-handle colours for gpui-base"
```

---

### Task 10: `NativeTheme`, `apply`, `apply_system_theme`, `apply_accessibility`, the re-apply observer (§8.1, §3.3; rationale §2.7, §2.8, §2.13, §2.22, D29)

**Model:** Fable 5.1 inline

**Files:**
- Modify: `connectors/native-theme-gpui/src/lib.rs`

**Interfaces:**
- Consumes: `base_layer::{scrollbar_geometry, resizable_theme, apply_overrides, ScrollbarGeometry}` (Task 9); `gpui::{App, Global}`; `App::{has_global, try_global, global, global_mut, default_global, observe_global, defer, set_reduce_motion, reduce_motion, refresh_windows}` (gpui-pre 0.3.3 `src/app.rs:1059-1074, 2001-2099`); `gpui_component::init` (`src/lib.rs:128`); `gpui_component::theme::Theme::{global, global_mut, is_dark, sync_base, change}` (`src/theme/mod.rs:172-183, 237-260, 321-326`); `gpui_base::Theme::{global, global_mut}`.
- Produces:

```rust
#[derive(Default)]
pub struct NativeTheme { /* private */ }
impl gpui::Global for NativeTheme {}
impl NativeTheme {
    pub fn resolved(&self, cx: &App) -> Option<&ResolvedTheme>;
    pub fn accessibility(&self) -> &AccessibilityPreferences;
    pub fn native(&self, cx: &App) -> Option<Native<'_>>;
}
pub trait ActiveNativeTheme { fn native_theme(&self) -> Option<&NativeTheme>; }
impl ActiveNativeTheme for App {}
#[derive(Clone, Copy)]
pub struct Native<'a> { pub resolved: &'a ResolvedTheme, pub accessibility: &'a AccessibilityPreferences }
impl<'a> Native<'a> { pub fn unscaled(resolved: &'a ResolvedTheme) -> Self; }
pub fn apply(theme: GpuiTheme, resolved: &ResolvedTheme, prefs: &AccessibilityPreferences, cx: &mut App);
pub fn apply_system_theme(sys: &SystemTheme, cx: &mut App);
pub fn apply_accessibility(prefs: &AccessibilityPreferences, cx: &mut App);
```

**Deviation from the spec as first written (now corrected in §3.3 / §8.1 and rationale error 40):** `apply` does *not* set `reapplying`. `observe_global` activates the subscription through a deferred effect (gpui-pre `src/app.rs:2087-2099`), and effects run in order, so the notification `apply` queues is delivered before the observer is active; a flag set by `apply` would stay set and swallow the next upstream rebuild. The observer therefore re-applies once on its first delivery (idempotent) and sets the flag only for its own write. The test runs `Theme::change` twice to catch a stale flag.

- [ ] **Step 1: Write the failing tests** in a new `#[cfg(test)] mod apply_tests` at the end of `lib.rs`:

```rust
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod apply_tests {
    use super::*;
    use gpui::TestAppContext;
    use gpui_component::scroll::ScrollbarMode;

    fn prefs(reduce_motion: bool) -> AccessibilityPreferences {
        AccessibilityPreferences { reduce_motion, ..AccessibilityPreferences::default() }
    }

    /// The dark catppuccin preset, with the precondition every observer test
    /// relies on: the native *active* handle colour (`splitter.hover_color`)
    /// differs from `drag_border`, which upstream's projection writes there.
    /// `handle` cannot serve as the observable: every preset inherits
    /// `splitter.divider_color` from `defaults.border.color`
    /// (`docs/inheritance-rules.toml:238`), which is also what upstream writes,
    /// so that slot holds the same value whichever side wrote it.
    fn preset_for_observer_tests(prefs: &AccessibilityPreferences) -> (GpuiTheme, ResolvedTheme) {
        let (theme, resolved) =
            from_preset("catppuccin-mocha", true, prefs).expect("preset should load");
        assert_ne!(
            colors::rgba_to_hsla(resolved.splitter.hover_color),
            theme.drag_border,
            "precondition: the native active-handle colour must differ from upstream's drag_border"
        );
        (theme, resolved)
    }

    /// A preset whose light and dark variants resolve to different backgrounds.
    fn preset_with_two_variants(
        prefs: &AccessibilityPreferences,
    ) -> ((GpuiTheme, ResolvedTheme), (GpuiTheme, ResolvedTheme)) {
        for info in Theme::list_presets() {
            if let (Ok(dark), Ok(light)) =
                (from_preset(info.key, true, prefs), from_preset(info.key, false, prefs))
                && dark.1.defaults.background_color != light.1.defaults.background_color
            {
                return (dark, light);
            }
        }
        panic!("no preset with distinct light and dark variants");
    }

    /// §12: after `apply`, every receiver holds the native value; after upstream
    /// rebuilds the base layer, the observer restores it; and the test returning
    /// proves the observer terminates. Two changes catch a stale `reapplying`.
    #[gpui::test]
    fn apply_installs_and_survives_theme_change(cx: &mut TestAppContext) {
        let prefs = prefs(true);
        let (theme, resolved) = preset_for_observer_tests(&prefs);
        let handle = colors::rgba_to_hsla(resolved.splitter.divider_color);
        let active = colors::rgba_to_hsla(resolved.splitter.hover_color);
        let expected_mode = if resolved.scrollbar.overlay_mode {
            ScrollbarMode::Scrolling
        } else {
            ScrollbarMode::Always
        };

        // No gpui_component::init here: apply must initialise the styled layer
        // itself when it is absent (D29) and must not panic.
        cx.update(|cx| apply(theme, &resolved, &prefs, cx));

        cx.update(|cx| {
            assert!(GpuiTheme::global(cx).is_dark());
            let base = gpui_base::Theme::global(cx);
            assert_eq!(base.scrollbar.mode(), expected_mode);
            assert_eq!(base.resizable.handle, Some(handle));
            assert_eq!(base.resizable.active_handle, Some(active));
            assert!(cx.reduce_motion());
            assert!(cx.native_theme().and_then(|t| t.resolved(cx)).is_some());
            // Upstream's projection writes `drag_border` into active_handle; the
            // helper's precondition makes the restore assertions below meaningful.
            assert_ne!(GpuiTheme::global(cx).drag_border, active);
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx); // rebuilds gpui_base::Theme
        });
        cx.update(|cx| {
            assert_eq!(
                gpui_base::Theme::global(cx).resizable.active_handle,
                Some(active),
                "observer restored the native value after the first rebuild"
            );
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
        });
        cx.update(|cx| {
            assert_eq!(
                gpui_base::Theme::global(cx).resizable.active_handle,
                Some(active),
                "and after the second rebuild (no stale reapplying flag)"
            );
        });
    }

    /// §3.3, D38: the observer activates at the end of the effect flush that
    /// installs it, so a rebuild in the *same* update as the first `apply` (an
    /// application calling `Theme::sync_system_appearance` right after it) is
    /// delivered while the observer is inactive. The deferred re-write in
    /// `install_observer_once` closes that gap; without it this test fails.
    #[gpui::test]
    fn apply_then_change_in_the_same_update_keeps_overrides(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) = preset_for_observer_tests(&prefs);
        let active = colors::rgba_to_hsla(resolved.splitter.hover_color);
        cx.update(|cx| {
            apply(theme, &resolved, &prefs, cx);
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx); // same update: observer not yet active
        });
        cx.update(|cx| {
            assert_eq!(
                gpui_base::Theme::global(cx).resizable.active_handle,
                Some(active),
                "the deferred re-write restored the overrides after a same-update rebuild"
            );
        });
    }

    /// The `reapplying` flag cannot tell the observer's own notification from a
    /// rebuild another effect performs before that notification is delivered:
    /// GPUI merges the two (spec §3.3, limit). The observer therefore repairs
    /// the handle colours when they no longer hold the native values (D42).
    #[gpui::test]
    fn observer_repairs_a_rebuild_merged_into_its_own_notification(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) = preset_for_observer_tests(&prefs);
        let active = colors::rgba_to_hsla(resolved.splitter.hover_color);
        cx.update(|cx| apply(theme, &resolved, &prefs, cx)); // observer active after this flush
        cx.update(|cx| {
            // Queue: [N_base (this write), Defer(change)]. The observer answers
            // N_base with a marked write whose notification lands after the
            // deferred change; the change's own notification is deduplicated.
            let _ = gpui_base::Theme::global_mut(cx);
            cx.defer(|cx| GpuiTheme::change(GpuiThemeMode::Dark, None, cx));
        });
        cx.update(|cx| {
            assert_eq!(
                gpui_base::Theme::global(cx).resizable.active_handle,
                Some(active),
                "the observer repaired the rebuild that its own notification had absorbed"
            );
        });
    }

    /// §3.3 step 2: with no stored variant for the new mode, colours come from
    /// the styled theme (upstream's own projection) and nothing panics.
    #[gpui::test]
    fn apply_without_stored_variant_falls_back(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) = preset_for_observer_tests(&prefs); // stores dark only
        cx.update(|cx| apply(theme, &resolved, &prefs, cx));
        cx.update(|cx| GpuiTheme::change(GpuiThemeMode::Light, None, cx));
        cx.update(|cx| {
            let nt = cx.native_theme().expect("installed by apply");
            assert!(nt.resolved(cx).is_none(), "no light variant was stored");
            assert!(nt.native(cx).is_none());
            let styled = GpuiTheme::global(cx);
            let base = gpui_base::Theme::global(cx);
            assert_eq!(base.resizable.handle, Some(styled.border));
            assert_eq!(base.resizable.active_handle, Some(styled.drag_border));
        });
    }

    #[gpui::test]
    fn apply_system_theme_stores_both_variants(cx: &mut TestAppContext) {
        let Ok(sys) = SystemTheme::from_system() else { return }; // CI has no desktop
        cx.update(|cx| apply_system_theme(&sys, cx));
        cx.update(|cx| {
            let nt = cx.native_theme().expect("installed");
            assert!(nt.resolved(cx).is_some());
            GpuiTheme::change(
                if sys.mode.is_dark() { GpuiThemeMode::Light } else { GpuiThemeMode::Dark },
                None,
                cx,
            );
        });
        cx.update(|cx| {
            let nt = cx.native_theme().expect("installed");
            assert!(nt.resolved(cx).is_some(), "the other variant is stored too");
            // D34: the other mode's palette is the native one, not the registry default.
            let other = if sys.mode.is_dark() { &sys.light } else { &sys.dark };
            assert_eq!(
                colors::hsla_to_hex(GpuiTheme::global(cx).background),
                colors::hsla_to_hex(colors::rgba_to_hsla(other.defaults.background_color))
            );
        });
    }

    /// D34: once both variants are applied, upstream's `Theme::change` reproduces
    /// the native palette of either mode through the installed `ThemeConfig`.
    #[gpui::test]
    fn apply_installs_configs_for_both_variants(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let ((dark_theme, dark), (light_theme, light)) = preset_with_two_variants(&prefs);
        cx.update(|cx| {
            apply(dark_theme, &dark, &prefs, cx);
            apply(light_theme, &light, &prefs, cx); // light is current; the dark config must survive
            GpuiTheme::change(GpuiThemeMode::Dark, None, cx);
        });
        cx.update(|cx| {
            let styled = GpuiTheme::global(cx);
            assert!(styled.is_dark());
            // Hex comparison: the config round trip is exact for 8-bit colours.
            assert_eq!(
                colors::hsla_to_hex(styled.background),
                colors::hsla_to_hex(colors::rgba_to_hsla(dark.defaults.background_color))
            );
            assert_eq!(
                colors::hsla_to_hex(styled.button_primary),
                colors::hsla_to_hex(colors::rgba_to_hsla(dark.button.primary_background)),
                "button_* fields survive the config round trip"
            );
            // D41: the config carries the mode's default highlighter style, so the
            // switch to dark also switched code highlighting.
            assert_eq!(styled.highlight_theme.appearance, GpuiThemeMode::Dark);
            assert!(cx.native_theme().and_then(|t| t.resolved(cx)).is_some());
        });
    }

    /// D35: a runtime preference change rebuilds the styled theme from the
    /// stored variant, so scaling reaches `font_size` and its config copy.
    #[gpui::test]
    fn apply_accessibility_rescales_from_the_stored_variant(cx: &mut TestAppContext) {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) =
            from_preset("catppuccin-mocha", true, &prefs).expect("preset should load");
        cx.update(|cx| apply(theme, &resolved, &prefs, cx));
        let scaled = AccessibilityPreferences {
            text_scaling_factor: 1.5,
            reduce_motion: true,
            ..AccessibilityPreferences::default()
        };
        cx.update(|cx| apply_accessibility(&scaled, cx));
        cx.update(|cx| {
            let styled = GpuiTheme::global(cx);
            assert_eq!(styled.font_size, px(resolved.defaults.font.size * 1.5));
            assert_eq!(styled.dark_theme.font_size, Some(resolved.defaults.font.size * 1.5));
            assert!(cx.reduce_motion());
            let nt = cx.native_theme().expect("installed");
            assert_eq!(nt.accessibility().text_scaling_factor, 1.5);
            assert!(nt.resolved(cx).is_some(), "the stored variant survives the rebuild");
        });
    }

    #[gpui::test]
    fn apply_accessibility_forwards_reduce_motion(cx: &mut TestAppContext) {
        cx.update(|cx| {
            apply_accessibility(&prefs(true), cx); // works without a NativeTheme
            assert!(cx.reduce_motion());
            apply_accessibility(&prefs(false), cx);
            assert!(!cx.reduce_motion());
        });
    }

    #[test]
    fn native_unscaled_has_factor_one() {
        let (_, resolved) = from_preset("catppuccin-mocha", true, &AccessibilityPreferences::default()).unwrap();
        let n = Native::unscaled(&resolved);
        assert_eq!(text_scale_factor(n.accessibility), 1.0);
        assert!(!n.accessibility.reduce_motion);
    }
}
```

- [ ] **Step 2: Run them to verify they fail**

```bash
cargo test -p native-theme-gpui --lib apply_tests
```

Expected: FAIL to compile (`apply`, `NativeTheme`, `native_theme` not found).

- [ ] **Step 3: Implement**

Add imports at the top of `lib.rs`: `use gpui::{App, Global, SharedString, px};` (replacing the existing `use gpui::{SharedString, px};`), and `pub mod base_layer;` (Task 9) stays. Add after the helper functions:

```rust
// ---------------------------------------------------------------------------
// Installation: NativeTheme global, apply family, base-layer observer (§8.1, §3.3)
// ---------------------------------------------------------------------------

/// The native theme installed by [`apply`]; one per `App`, read with
/// [`ActiveNativeTheme::native_theme`].
///
/// Stores both resolved variants (when known) because upstream's
/// `Theme::change` switches modes without going through the connector, and the
/// re-apply observer must then find the variant of the mode upstream switched
/// to (rationale §2.22). Every field's default is the right initial state, so
/// `Default` is derived (a hand-written impl would trip clippy's
/// `derivable_impls`).
#[derive(Default)]
pub struct NativeTheme {
    light: Option<ResolvedTheme>,
    dark: Option<ResolvedTheme>,
    accessibility: AccessibilityPreferences,
    /// Set by the observer around its own write of `gpui_base::Theme`, so it
    /// can tell that notification from an upstream rebuild (§3.3).
    reapplying: bool,
    observer_installed: bool,
    /// Mode `apply` last installed; the fallback when the styled theme global
    /// is absent (D29).
    last_is_dark: bool,
}

impl Global for NativeTheme {}

impl NativeTheme {
    fn is_dark(&self, cx: &App) -> bool {
        cx.try_global::<GpuiTheme>()
            .map(GpuiTheme::is_dark)
            .unwrap_or(self.last_is_dark)
    }

    fn variant(&self, is_dark: bool) -> Option<&ResolvedTheme> {
        if is_dark { self.dark.as_ref() } else { self.light.as_ref() }
    }

    /// The stored variant for the styled theme's current mode, if any.
    #[must_use]
    pub fn resolved(&self, cx: &App) -> Option<&ResolvedTheme> {
        self.variant(self.is_dark(cx))
    }

    /// The preferences `apply` / `apply_accessibility` last installed.
    #[must_use]
    pub fn accessibility(&self) -> &AccessibilityPreferences {
        &self.accessibility
    }

    /// Borrowed view for the `geometry` builders, if a variant is stored for
    /// the current mode.
    #[must_use]
    pub fn native(&self, cx: &App) -> Option<Native<'_>> {
        self.resolved(cx).map(|resolved| Native { resolved, accessibility: &self.accessibility })
    }
}

/// `cx.native_theme()`, mirroring gpui-component's `cx.theme()`.
pub trait ActiveNativeTheme {
    /// The installed [`NativeTheme`], or `None` before [`apply`] ran.
    fn native_theme(&self) -> Option<&NativeTheme>;
}

impl ActiveNativeTheme for App {
    fn native_theme(&self) -> Option<&NativeTheme> {
        self.try_global::<NativeTheme>()
    }
}

/// Borrowed inputs of every `geometry` builder (spec §9.1).
#[derive(Clone, Copy)]
pub struct Native<'a> {
    /// The resolved theme the values come from.
    pub resolved: &'a ResolvedTheme,
    /// Accessibility preferences; only `text_scaling_factor` affects geometry.
    pub accessibility: &'a AccessibilityPreferences,
}

static UNSCALED: AccessibilityPreferences = AccessibilityPreferences {
    text_scaling_factor: 1.0,
    reduce_motion: false,
    high_contrast: false,
    reduce_transparency: false,
};

impl<'a> Native<'a> {
    /// Unscaled, no reductions: for the preset path and tests.
    #[must_use]
    pub fn unscaled(resolved: &'a ResolvedTheme) -> Self {
        Self { resolved, accessibility: &UNSCALED }
    }
}

/// Install `theme` as gpui-component's global theme with the native base-layer
/// overrides, and keep them installed across upstream rebuilds (spec §8.1, §3.3).
///
/// 1. stores `resolved` under the theme's mode and `prefs` in [`NativeTheme`];
/// 2. initialises gpui-component if its theme global is absent (upstream
///    requires `gpui_component::init` before any component use; calling it
///    here only when the global is missing means it runs at most once);
/// 3. writes the styled theme, then installs a `ThemeConfig` for the *other*
///    stored variant (if any) under the same display name, so upstream's
///    `Theme::change` / `sync_system_appearance` reproduces native colours in
///    either mode (D34);
/// 4. projects into gpui-base (`sync_base`);
/// 5. writes the native scrollbar geometry/colours and resize-handle colours
///    onto gpui-base ([`base_layer::apply_overrides`]);
/// 6. forwards `prefs.reduce_motion` to GPUI;
/// 7. installs, once per `App`, the observer that restores step 5 whenever
///    upstream rebuilds the base theme, together with one deferred repeat of
///    step 5 for the update that installs it (the subscription activates only
///    at the end of that update's effect flush, D38);
/// 8. refreshes every window so the change paints at once (D37).
///
/// `theme` is moved into the global; `resolved` is cloned once.
pub fn apply(theme: GpuiTheme, resolved: &ResolvedTheme, prefs: &AccessibilityPreferences, cx: &mut App) {
    let is_dark = theme.is_dark();
    let (light, dark) = if is_dark { (None, Some(resolved)) } else { (Some(resolved), None) };
    apply_inner(theme, light, dark, prefs, cx);
}

/// [`SystemThemeExt::to_gpui_theme`] for the OS mode, storing both variants
/// and installing both `ThemeConfig`s, then [`apply`]; upstream's
/// `Theme::sync_system_appearance` then reproduces native colours in either mode.
pub fn apply_system_theme(sys: &SystemTheme, cx: &mut App) {
    let theme = sys.to_gpui_theme();
    apply_inner(theme, Some(&sys.light), Some(&sys.dark), &sys.accessibility, cx);
}

/// Apply a runtime change of the accessibility preferences (a portal signal,
/// a settings toggle). When a variant is stored for the current mode, the
/// styled theme is rebuilt from it with `prefs` and re-installed through the
/// [`apply`] path, so text scaling and transparency take effect and both
/// configs are refreshed (D35); the stored variants are kept. Without a stored
/// variant only `reduce_motion` is forwarded and the preferences are stored.
pub fn apply_accessibility(prefs: &AccessibilityPreferences, cx: &mut App) {
    let rebuilt = cx.try_global::<NativeTheme>().and_then(|nt| {
        let is_dark = nt.is_dark(cx);
        let resolved = nt.variant(is_dark)?;
        let name = cx.try_global::<GpuiTheme>()?.theme_name().clone();
        Some(to_theme(resolved, &name, is_dark, prefs))
    });
    match rebuilt {
        // `None, None` keeps the stored variants; apply_inner stores `prefs`.
        Some(theme) => apply_inner(theme, None, None, prefs, cx),
        None => {
            if cx.has_global::<NativeTheme>() {
                cx.global_mut::<NativeTheme>().accessibility = prefs.clone();
            }
            cx.set_reduce_motion(prefs.reduce_motion);
        }
    }
}

fn apply_inner(
    theme: GpuiTheme,
    light: Option<&ResolvedTheme>,
    dark: Option<&ResolvedTheme>,
    prefs: &AccessibilityPreferences,
    cx: &mut App,
) {
    let is_dark = theme.is_dark();
    // The display name of the config `to_theme` built for this mode; the other
    // variant's config takes the same name.
    let name: SharedString = theme.theme_name().clone();
    {
        let nt = cx.default_global::<NativeTheme>();
        if let Some(light) = light {
            nt.light = Some(light.clone());
        }
        if let Some(dark) = dark {
            nt.dark = Some(dark.clone());
        }
        nt.accessibility = prefs.clone();
        nt.last_is_dark = is_dark;
    }

    // D29: the styled accessors panic on a missing global; init creates it.
    if !cx.has_global::<GpuiTheme>() {
        gpui_component::init(cx);
    }
    *GpuiTheme::global_mut(cx) = theme;

    // D34: the other mode's config from its stored variant, so Theme::change
    // reproduces the native palette instead of the registry default.
    let other_config = cx.try_global::<NativeTheme>().and_then(|nt| {
        nt.variant(!is_dark).map(|other| {
            let mode = if is_dark { GpuiThemeMode::Light } else { GpuiThemeMode::Dark };
            Rc::new(config::to_theme_config(other, &name, mode, prefs))
        })
    });
    if let Some(cfg) = other_config {
        let styled = GpuiTheme::global_mut(cx);
        if is_dark {
            styled.light_theme = cfg;
        } else {
            styled.dark_theme = cfg;
        }
    }

    GpuiTheme::sync_base(cx);
    // `false`: this write's notification may be delivered before the observer
    // is active (§3.3), so it must not be marked as the observer's own.
    write_base_overrides(cx, false);
    cx.set_reduce_motion(prefs.reduce_motion);
    install_observer_once(cx);
    // D37: paint now. A change from a timer, portal signal or menu action must
    // not wait for the next input event; upstream refreshes only the window
    // passed to Theme::change (gpui-pre 0.3.3 src/app.rs:1074).
    cx.refresh_windows();
}

/// The values to write onto gpui-base for `is_dark` (spec §3.3 step 2):
/// the stored variant when there is one; otherwise geometry from the other
/// variant and colours from the styled theme, which is exactly upstream's own
/// projection (`scrollbar`, `scrollbar_thumb`, `scrollbar_thumb_hover` with
/// active = hover; `border` / `drag_border` for the handles).
fn base_overrides_for(
    nt: &NativeTheme,
    is_dark: bool,
    styled: Option<&GpuiTheme>,
) -> Option<(base_layer::ScrollbarGeometry, gpui_base::ResizableTheme)> {
    if let Some(resolved) = nt.variant(is_dark) {
        return Some((base_layer::scrollbar_geometry(resolved), base_layer::resizable_theme(resolved)));
    }
    let other = nt.variant(!is_dark)?;
    let mut geometry = base_layer::scrollbar_geometry(other);
    let mut resizable = base_layer::resizable_theme(other);
    if let Some(styled) = styled {
        geometry.track = styled.scrollbar;
        geometry.track_active_border = styled.border;
        geometry.thumb = styled.scrollbar_thumb;
        geometry.thumb_hover = styled.scrollbar_thumb_hover;
        geometry.thumb_active = styled.scrollbar_thumb_hover;
        resizable = gpui_base::ResizableTheme {
            handle: Some(styled.border),
            active_handle: Some(styled.drag_border),
        };
    }
    Some((geometry, resizable))
}

/// Compute and write the overrides for the current mode. `mark` flags the
/// write as the observer's own so the observer ignores its notification
/// (§3.3); only the observer passes `true`, because a flag set by `apply`
/// could be delivered before the observer is active and would then never be
/// cleared (rationale errors 40, 42).
fn write_base_overrides(cx: &mut App, mark: bool) {
    let Some(nt) = cx.try_global::<NativeTheme>() else { return };
    let is_dark = nt.is_dark(cx);
    let Some((geometry, resizable)) = base_overrides_for(nt, is_dark, cx.try_global::<GpuiTheme>()) else {
        return;
    };
    if mark {
        cx.global_mut::<NativeTheme>().reapplying = true;
    }
    base_layer::apply_overrides(&geometry, resizable, cx);
}

/// Whether gpui-base's resize-handle colours are the ones the connector would
/// write for the current mode. `ResizableTheme` derives no `PartialEq`, so both
/// fields are compared. `true` when nothing is stored (nothing to repair).
fn handles_hold_native_values(cx: &App) -> bool {
    let Some(nt) = cx.try_global::<NativeTheme>() else {
        return true;
    };
    let Some((_, expected)) = base_overrides_for(nt, nt.is_dark(cx), cx.try_global::<GpuiTheme>())
    else {
        return true;
    };
    let current = gpui_base::Theme::global(cx).resizable;
    current.handle == expected.handle && current.active_handle == expected.active_handle
}

/// Observe `gpui_base::Theme` (§3.3). Terminates because `global_mut` queues one
/// deduplicated notification (gpui-pre 0.3.3 `src/app.rs:1662-1664`), delivered
/// after the pending mark is removed (`:1817-1821`): the observer's own write
/// yields exactly one further delivery, absorbed by `reapplying`.
///
/// The subscription activates through a deferred effect (`src/app.rs:2087-2099`)
/// at the end of the flush that follows this call, so a base-theme write made by
/// other code in the same update as the first `apply` (a
/// `Theme::sync_system_appearance` right after it) would stand until the next
/// rebuild. One deferred re-write, queued after the activation, closes that gap
/// (D38); it is the first notification the active observer receives.
fn install_observer_once(cx: &mut App) {
    if cx.try_global::<NativeTheme>().is_none_or(|nt| nt.observer_installed) {
        return;
    }
    cx.global_mut::<NativeTheme>().observer_installed = true;
    cx.observe_global::<gpui_base::Theme>(|cx| {
        let Some(nt) = cx.try_global::<NativeTheme>() else { return };
        if nt.reapplying {
            cx.global_mut::<NativeTheme>().reapplying = false;
            // A rebuild another effect performs between this observer's write
            // and the delivery of its notification is merged into that
            // notification by GPUI's per-type deduplication, so the flag alone
            // would swallow it (spec §3.3, limit). The handle colours are
            // comparable and upstream's rebuild changes them, so repair when
            // they no longer hold the native values; the scrollbar styles are
            // opaque and cannot be checked (D42).
            if !handles_hold_native_values(cx) {
                write_base_overrides(cx, true);
            }
            return;
        }
        write_base_overrides(cx, true);
    })
    .detach();
    // Effects are FIFO, so this runs after the activation above and after any
    // same-update write. `false`: the active observer re-applies once on its
    // delivery and absorbs its own write, like any other rebuild (§3.3).
    cx.defer(|cx| write_base_overrides(cx, false));
}
```

Notes for the implementer: `Option::is_none_or` is stable since Rust 1.82 (below both floors). `cx.global_mut::<NativeTheme>()` is only called after `has_global` / `try_global` established the global exists, and `default_global` creates it, so no accessor can panic. The `let ... else` on `try_global` followed by `cx.try_global::<GpuiTheme>()` are two shared borrows; the mutable borrow starts only after both values are computed.

Add a unit test of the pure fallback next to the other `apply_tests` (no `App` needed):

```rust
    #[test]
    fn base_overrides_fallback_takes_geometry_from_the_other_variant_and_colours_from_styled() {
        let prefs = AccessibilityPreferences::default();
        let (theme, resolved) = from_preset("catppuccin-mocha", true, &prefs).unwrap();
        let nt = NativeTheme { dark: Some(resolved.clone()), ..NativeTheme::default() };
        let (g, r) = base_overrides_for(&nt, false, Some(&theme)).expect("falls back to the dark variant");
        let dark = base_layer::scrollbar_geometry(&resolved);
        assert_eq!(g.track_width, dark.track_width);
        assert_eq!(g.thumb_inset, dark.thumb_inset);
        assert_eq!(g.track, theme.scrollbar);
        assert_eq!(g.thumb_active, theme.scrollbar_thumb_hover);
        assert_eq!(r.handle, Some(theme.border));
        assert!(base_overrides_for(&NativeTheme::default(), false, Some(&theme)).is_none());
    }
```

- [ ] **Step 4: Run the gate**

```bash
cargo test -p native-theme-gpui --lib apply_tests
cargo test -p native-theme-gpui --lib
cargo clippy -p native-theme-gpui --lib --tests -- -D warnings
```

Expected: PASS; the observer tests return (a loop would hang the test binary, which is the intended failure mode); no clippy warnings.

- [ ] **Step 5: Commit**

```bash
git add connectors/native-theme-gpui/src/lib.rs
git commit -m "feat(gpui): apply/apply_system_theme/apply_accessibility, NativeTheme global, base-layer re-apply observer

Theme::change/sync_base rebuild gpui_base::Theme with fixed scrollbar
styles; a global observer restores the native overrides, absorbing its own
notification with a flag that only its own write sets."
```

---

### Task 11: `geometry`: per-widget `StyleRefinement` builders, `Size` helpers, layout accessors (§9; rationale §2.5, §2.6, §2.23, §5.3)

**Model:** Opus 5 (`implement` agent); Fable 5.1 if a precondition assertion fails

**Files:**
- Create: `connectors/native-theme-gpui/src/geometry.rs`
- Modify: `connectors/native-theme-gpui/src/lib.rs` (`pub mod geometry;`)

**Interfaces:**
- Consumes: `Native<'_>` and `text_scale_factor` (Tasks 8, 10); `gpui::{StyleRefinement, Styled, FontWeight, Pixels, px}`; `gpui_component::Size` (`src/sizing.rs:6-13`); `Styled` setters generated by gpui-pre-macros 0.3.3 `src/styles.rs`: `h`, `min_w`, `min_h`, `max_w`, `max_h` (`impl Into<Length>`), `px`, `py`, `gap`, `gap_x` (`impl Into<DefiniteLength>`), `rounded`, `border` (`impl Into<AbsoluteLength>`), `border_color(impl Into<Hsla>)`, `text_size(impl Into<AbsoluteLength>)`, `font_weight(FontWeight)` (gpui-pre `src/styled.rs:522, 538`); `native_theme::theme::{LayoutTheme, ResolvedBorderSpec, ResolvedFontSpec}`.
- Produces: `pub fn <widget>(n: Native<'_>) -> StyleRefinement` for `button`, `input`, `menu_item`, `list_item`, `tooltip`, `popover`, `status_bar`, `dialog`, `dialog_footer`, `dialog_title`, `dialog_description`, `table`, `progress`, `group_box_content`, `accordion_title`, `checkbox`, `radio`, `select`, `combobox`, `title_bar`; `pub fn control_height(theme_height: f32, font: &ResolvedFontSpec, border: &ResolvedBorderSpec, n: Native<'_>) -> Pixels`; `pub fn spinner_size(n) -> Size`, `icon_size_toolbar/_small/_large/_dialog/_panel(n) -> Size`; `pub fn dialog_max_width(n) -> Pixels`, `pub fn input_height(n) -> Pixels`; `pub fn widget_gap / container_margin / window_margin / section_gap(layout: &LayoutTheme) -> Option<Pixels>`.

Reading a refinement in tests: `StyleRefinement` is the `Refineable` derivation of `Style` (gpui-pre `src/style.rs:178-180`), so `size.height: Option<Length>`, `min_size.width / max_size.width / min_size.height / max_size.height: Option<Length>`, `padding.{left,right,top,bottom}: Option<DefiniteLength>`, `corner_radii.top_left: Option<AbsoluteLength>`, `border_widths.top: Option<AbsoluteLength>`, `border_color: Option<Hsla>`, `gap.{width,height}: Option<DefiniteLength>`, `text.font_size: Option<AbsoluteLength>`, `text.font_weight: Option<FontWeight>`. `Pixels` converts into all three length types (`src/geometry.rs:3315, 3585, 3755`).

- [ ] **Step 1: Write the failing tests** (create the file with `pub mod geometry;` in `lib.rs` and only this test module):

```rust
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::colors::rgba_to_hsla;
    use crate::{ColorMode, ResolvedTheme, Theme};
    use gpui::{AbsoluteLength, DefiniteLength, Length};
    use native_theme::AccessibilityPreferences;

    fn resolved(preset: &str, mode: ColorMode) -> ResolvedTheme {
        Theme::preset(preset)
            .expect("preset")
            .into_variant(mode)
            .expect("variant")
            .into_resolved(&native_theme::ResolutionContext::for_tests())
            .expect("resolves")
    }
    fn scaled(factor: f32) -> AccessibilityPreferences {
        AccessibilityPreferences { text_scaling_factor: factor, ..AccessibilityPreferences::default() }
    }
    fn len(v: f32) -> Option<Length> { Some(px(v).into()) }
    fn def(v: f32) -> Option<DefiniteLength> { Some(px(v).into()) }
    fn abs(v: f32) -> Option<AbsoluteLength> { Some(px(v).into()) }

    const CASES: &[(&str, ColorMode)] =
        &[("catppuccin-mocha", ColorMode::Dark), ("catppuccin-latte", ColorMode::Light)];
    const FACTORS: &[f32] = &[1.0, 1.5];

    /// Runs `check` for both presets and both factors with a `Native` view.
    fn for_each_case(mut check: impl FnMut(&ResolvedTheme, f32, Native<'_>)) {
        for (preset, mode) in CASES {
            let r = resolved(preset, *mode);
            for &s in FACTORS {
                let prefs = scaled(s);
                check(&r, s, Native { resolved: &r, accessibility: &prefs });
            }
        }
    }

    fn assert_text(out: &StyleRefinement, font: &ResolvedFontSpec, s: f32) {
        assert_eq!(out.text.font_size, abs(font.size * s));
        assert_eq!(out.text.font_weight, Some(FontWeight(f32::from(font.weight))));
    }

    #[test]
    fn button_refinement_matches_theme_values() {
        for_each_case(|r, _s, n| {
            let b = &r.button;
            let out = button(n);
            assert_eq!(out.size.height, Some(control_height(b.min_height, &b.font, &b.border, n).into()));
            assert_eq!(out.min_size.width, len(b.min_width));
            assert_eq!(out.padding.left, def(b.border.padding_horizontal));
            assert_eq!(out.padding.right, def(b.border.padding_horizontal));
            assert_eq!(out.padding.top, def(b.border.padding_vertical));
            assert_eq!(out.padding.bottom, def(b.border.padding_vertical));
            assert_eq!(out.corner_radii.top_left, abs(b.border.corner_radius.max(0.0)));
            assert_eq!(out.border_widths.top, abs(b.border.line_width));
            assert_eq!(out.border_color, Some(rgba_to_hsla(b.border.color)));
            assert_eq!(out.text.font_size, None, "button label size is inner (Tier U), not set here");
        });
    }

    #[test]
    fn input_refinement_matches_theme_values() {
        for_each_case(|r, s, n| {
            let i = &r.input;
            let out = input(n);
            assert_eq!(out.size.height, Some(control_height(i.min_height, &i.font, &i.border, n).into()));
            assert_eq!(out.corner_radii.top_left, abs(i.border.corner_radius.max(0.0)));
            assert_eq!(out.border_widths.top, abs(i.border.line_width));
            assert_text(&out, &i.font, s);
            assert_eq!(input_height(n), control_height(i.min_height, &i.font, &i.border, n));
        });
    }

    #[test]
    fn menu_and_list_items_match_theme_values() {
        for_each_case(|r, s, n| {
            let m = &r.menu;
            let out = menu_item(n);
            assert_eq!(out.size.height, Some(control_height(m.row_height, &m.font, &m.border, n).into()));
            assert_eq!(out.padding.left, def(m.border.padding_horizontal));
            assert_eq!(out.padding.top, def(m.border.padding_vertical));
            assert_eq!(out.gap.width, def(m.icon_text_gap));
            assert_eq!(out.gap.height, None, "gap_x sets the column gap only");
            assert_text(&out, &m.font, s);

            let l = &r.list;
            let out = list_item(n);
            assert_eq!(out.size.height, Some(control_height(l.row_height, &l.item_font, &l.border, n).into()));
            assert_eq!(out.padding.left, def(l.border.padding_horizontal));
            assert_eq!(out.padding.top, def(l.border.padding_vertical));
            assert_text(&out, &l.item_font, s);
        });
    }

    #[test]
    fn tooltip_popover_status_bar_match_theme_values() {
        for_each_case(|r, s, n| {
            let t = &r.tooltip;
            let out = tooltip(n);
            assert_eq!(out.max_size.width, len(t.max_width));
            assert_eq!(out.padding.left, def(t.border.padding_horizontal));
            assert_eq!(out.padding.top, def(t.border.padding_vertical));
            assert_eq!(out.corner_radii.top_left, abs(t.border.corner_radius.max(0.0)));
            assert_text(&out, &t.font, s);

            let p = &r.popover;
            let out = popover(n);
            assert_eq!(out.padding.left, def(p.border.padding_horizontal));
            assert_eq!(out.padding.top, def(p.border.padding_vertical));
            assert_eq!(out.corner_radii.top_left, abs(p.border.corner_radius.max(0.0)));

            let sb = &r.status_bar;
            let out = status_bar(n);
            assert_eq!(out.padding.left, def(sb.border.padding_horizontal));
            assert_eq!(out.padding.top, def(sb.border.padding_vertical));
            assert_text(&out, &sb.font, s);
        });
    }

    #[test]
    fn dialog_family_matches_theme_values() {
        for_each_case(|r, s, n| {
            let d = &r.dialog;
            let out = dialog(n);
            assert_eq!(out.padding.left, def(d.border.padding_horizontal));
            assert_eq!(out.padding.top, def(d.border.padding_vertical));
            assert_eq!(out.min_size.height, len(d.min_height));
            assert_eq!(out.max_size.height, len(d.max_height));
            assert_eq!(dialog_footer(n).gap.width, def(d.button_gap));
            assert_eq!(dialog_footer(n).gap.height, def(d.button_gap));
            assert_text(&dialog_title(n), &d.title_font, s);
            assert_text(&dialog_description(n), &d.body_font, s);
            assert_eq!(dialog_max_width(n), px(d.max_width));
            assert_text(&table(n), &r.list.item_font, s);
            assert_text(&title_bar(n), &r.window.title_bar_font, s);
        });
    }

    #[test]
    fn progress_group_box_accordion_match_theme_values() {
        for_each_case(|r, _s, n| {
            let p = &r.progress_bar;
            let out = progress(n);
            assert_eq!(out.size.height, len(p.track_height));
            assert_eq!(out.corner_radii.top_left, abs(p.border.corner_radius.max(0.0)));
            assert_eq!(out.min_size.width, len(p.min_width));

            let c = &r.card;
            let out = group_box_content(n);
            assert_eq!(out.padding.left, def(c.border.padding_horizontal));
            assert_eq!(out.padding.top, def(c.border.padding_vertical));
            assert_eq!(out.corner_radii.top_left, abs(c.border.corner_radius.max(0.0)));
            assert_eq!(out.border_widths.top, abs(c.border.line_width));
            assert_eq!(out.border_color, Some(rgba_to_hsla(c.border.color)));

            assert_eq!(accordion_title(n).size.height, len(r.expander.header_height));
        });
    }

    #[test]
    fn checkbox_radio_select_match_theme_values() {
        for_each_case(|r, s, n| {
            let c = &r.checkbox;
            let out = checkbox(n);
            assert_eq!(out.gap.width, def(c.label_gap));
            assert_text(&out, &c.font, s);
            // platform-facts §2.5: radio metrics are the checkbox's.
            assert_eq!(radio(n).gap.width, def(c.label_gap));
            assert_text(&radio(n), &c.font, s);

            let cb = &r.combo_box;
            let out = select(n);
            assert_eq!(out.min_size.height, Some(control_height(cb.min_height, &cb.font, &cb.border, n).into()));
            assert_eq!(out.min_size.width, len(cb.min_width));
            assert_eq!(out.corner_radii.top_left, abs(cb.border.corner_radius.max(0.0)));
            assert_text(&out, &cb.font, s);
            assert_eq!(combobox(n).min_size.width, len(cb.min_width));
        });
    }

    #[test]
    fn size_helpers_use_theme_values() {
        for_each_case(|r, _s, n| {
            assert_eq!(spinner_size(n), Size::Size(px(r.spinner.diameter)));
            let is = &r.defaults.icon_sizes;
            assert_eq!(icon_size_toolbar(n), Size::Size(px(is.toolbar)));
            assert_eq!(icon_size_small(n), Size::Size(px(is.small)));
            assert_eq!(icon_size_large(n), Size::Size(px(is.large)));
            assert_eq!(icon_size_dialog(n), Size::Size(px(is.dialog)));
            assert_eq!(icon_size_panel(n), Size::Size(px(is.panel)));
        });
    }

    #[test]
    fn layout_accessors_pass_through_option() {
        let layout = Theme::preset("kde-breeze").expect("preset").layout;
        assert_eq!(widget_gap(&layout), layout.widget_gap.map(px));
        assert_eq!(container_margin(&layout), layout.container_margin.map(px));
        assert_eq!(window_margin(&layout), layout.window_margin.map(px));
        assert_eq!(section_gap(&layout), layout.section_gap.map(px));
        assert!(widget_gap(&layout).is_some(), "static presets define all four keys (§1.3)");
        assert_eq!(widget_gap(&LayoutTheme::default()), None);
    }

    /// §9.4, rationale §2.23: at s = 1 the platform's own height wins.
    #[test]
    fn control_height_returns_theme_height_when_text_fits() {
        for preset in ["kde-breeze", "adwaita"] {
            let r = resolved(preset, ColorMode::Light);
            let prefs = scaled(1.0);
            let n = Native { resolved: &r, accessibility: &prefs };
            let b = &r.button;
            let text = (b.font.size * r.defaults.line_height).ceil() + 2.0 * b.border.padding_vertical;
            assert!(text <= b.min_height, "{preset}: precondition, text {text} must fit in {}", b.min_height);
            assert_eq!(control_height(b.min_height, &b.font, &b.border, n), px(b.min_height));
        }
    }

    /// §9.4: at s = 1.5 scaled text no longer fits and the height grows.
    #[test]
    fn control_height_grows_when_scaled_text_does_not_fit() {
        for preset in ["kde-breeze", "adwaita"] {
            let r = resolved(preset, ColorMode::Light);
            let prefs = scaled(1.5);
            let n = Native { resolved: &r, accessibility: &prefs };
            let b = &r.button;
            let text = (b.font.size * 1.5 * r.defaults.line_height).ceil() + 2.0 * b.border.padding_vertical;
            assert!(text > b.min_height, "{preset}: precondition, scaled text {text} must exceed {}", b.min_height);
            assert_eq!(control_height(b.min_height, &b.font, &b.border, n), px(text));
        }
    }

    #[test]
    fn unscaled_view_scales_nothing() {
        let r = resolved("catppuccin-mocha", ColorMode::Dark);
        let n = Native::unscaled(&r);
        assert_eq!(input(n).text.font_size, abs(r.input.font.size));
    }
}
```

- [ ] **Step 2: Run them to verify they fail**

```bash
cargo test -p native-theme-gpui --lib geometry
```

Expected: FAIL to compile (`button`, `control_height`, … not found).

- [ ] **Step 3: Implement the module** (above the test module):

```rust
//! Per-widget geometry for gpui-component 0.6.0 widgets (spec §9).
//!
//! Every builder is a pure function of a [`Native`] view and returns a
//! [`StyleRefinement`] the application applies with
//! `gpui_component::StyledExt::refine_style`:
//!
//! ```ignore
//! use gpui_component::StyledExt;
//! use native_theme_gpui::{ActiveNativeTheme, geometry};
//!
//! let n = cx.native_theme().and_then(|t| t.native(cx));
//! let button = Button::new("save").label("Save");
//! let button = match n { Some(n) => button.refine_style(&geometry::button(n)), None => button };
//! ```
//!
//! Each widget applies the caller's refinement after its own geometry at the
//! cited upstream line, so the values here win. Text sizes carry the
//! accessibility text-scaling factor; widths, paddings, radii and icon sizes
//! do not (spec §3.4). Every value is a `ResolvedTheme` field or one of the
//! two derivations in spec §9.4 (`scaled_text_size`, [`control_height`]).

use gpui::{FontWeight, Pixels, StyleRefinement, Styled, px};
use gpui_component::Size;
use native_theme::theme::{LayoutTheme, ResolvedBorderSpec, ResolvedFontSpec};

use crate::colors::rgba_to_hsla;
use crate::{Native, text_scale_factor};

/// `font.size × s` in pixels (spec §9.4).
fn scaled_text_size(font: &ResolvedFontSpec, n: Native<'_>) -> Pixels {
    px(font.size * text_scale_factor(n.accessibility))
}

/// CSS weight (100–900) as GPUI's `FontWeight`.
fn weight_of(font: &ResolvedFontSpec) -> FontWeight {
    FontWeight(f32::from(font.weight))
}

/// Text size and weight from a font spec.
fn with_text(r: StyleRefinement, font: &ResolvedFontSpec, n: Native<'_>) -> StyleRefinement {
    r.text_size(scaled_text_size(font, n)).font_weight(weight_of(font))
}

/// Control height (spec §9.4, rationale §5.3):
/// `max(theme_height, ceil(font.size × s × defaults.line_height) + 2 × padding_vertical)`.
///
/// At `s = 1` a platform's declared height already accommodates its text, so
/// this returns the theme's own value; it grows only when scaled text would be
/// clipped by gpui-component's fixed heights.
#[must_use]
pub fn control_height(
    theme_height: f32,
    font: &ResolvedFontSpec,
    border: &ResolvedBorderSpec,
    n: Native<'_>,
) -> Pixels {
    let text = (font.size * text_scale_factor(n.accessibility) * n.resolved.defaults.line_height).ceil();
    px(theme_height.max(text + 2.0 * border.padding_vertical))
}

/// `Button` (gpui-component 0.6.0 `src/button/button.rs:587-624` → refined at `:650`).
/// The label's text size is set on an inner element (`:658-666`), Tier U.
#[must_use]
pub fn button(n: Native<'_>) -> StyleRefinement {
    let b = &n.resolved.button;
    StyleRefinement::default()
        .h(control_height(b.min_height, &b.font, &b.border, n))
        .min_w(px(b.min_width))
        .px(px(b.border.padding_horizontal))
        .py(px(b.border.padding_vertical))
        .rounded(px(b.border.corner_radius.max(0.0)))
        .border(px(b.border.line_width))
        .border_color(rgba_to_hsla(b.border.color))
}

/// `Input` root (`src/input/input.rs:572-582` → `:587`); padding is inner, Tier U.
#[must_use]
pub fn input(n: Native<'_>) -> StyleRefinement {
    let i = &n.resolved.input;
    with_text(
        StyleRefinement::default()
            .h(control_height(i.min_height, &i.font, &i.border, n))
            .rounded(px(i.border.corner_radius.max(0.0)))
            .border(px(i.border.line_width)),
        &i.font,
        n,
    )
}

/// Application-built `MenuItem` (`src/menu/menu_item.rs:101-103` → `:109`).
#[must_use]
pub fn menu_item(n: Native<'_>) -> StyleRefinement {
    let m = &n.resolved.menu;
    with_text(
        StyleRefinement::default()
            .h(control_height(m.row_height, &m.font, &m.border, n))
            .px(px(m.border.padding_horizontal))
            .py(px(m.border.padding_vertical))
            .gap_x(px(m.icon_text_gap)),
        &m.font,
        n,
    )
}

/// `ListItem` (`src/list/list_item.rs:188-190` → `:196`).
#[must_use]
pub fn list_item(n: Native<'_>) -> StyleRefinement {
    let l = &n.resolved.list;
    with_text(
        StyleRefinement::default()
            .h(control_height(l.row_height, &l.item_font, &l.border, n))
            .px(px(l.border.padding_horizontal))
            .py(px(l.border.padding_vertical)),
        &l.item_font,
        n,
    )
}

/// Application-built `Tooltip::new` (`src/tooltip.rs:120-125` → `:126`).
#[must_use]
pub fn tooltip(n: Native<'_>) -> StyleRefinement {
    let t = &n.resolved.tooltip;
    with_text(
        StyleRefinement::default()
            .max_w(px(t.max_width))
            .px(px(t.border.padding_horizontal))
            .py(px(t.border.padding_vertical))
            .rounded(px(t.border.corner_radius.max(0.0))),
        &t.font,
        n,
    )
}

/// `Popover` (`src/popover.rs:284` → `:312`).
#[must_use]
pub fn popover(n: Native<'_>) -> StyleRefinement {
    let p = &n.resolved.popover;
    StyleRefinement::default()
        .px(px(p.border.padding_horizontal))
        .py(px(p.border.padding_vertical))
        .rounded(px(p.border.corner_radius.max(0.0)))
}

/// `StatusBar` (`src/status_bar.rs:87-89` → `:95`).
#[must_use]
pub fn status_bar(n: Native<'_>) -> StyleRefinement {
    let s = &n.resolved.status_bar;
    with_text(
        StyleRefinement::default()
            .px(px(s.border.padding_horizontal))
            .py(px(s.border.padding_vertical)),
        &s.font,
        n,
    )
}

/// `Dialog` (`src/dialog/dialog.rs:502-512, 578` → `:582`); width through
/// [`dialog_max_width`] and `Dialog::max_w`.
#[must_use]
pub fn dialog(n: Native<'_>) -> StyleRefinement {
    let d = &n.resolved.dialog;
    StyleRefinement::default()
        .px(px(d.border.padding_horizontal))
        .py(px(d.border.padding_vertical))
        .min_h(px(d.min_height))
        .max_h(px(d.max_height))
}

/// `DialogFooter` (`src/dialog/footer.rs:47` → `:51`).
#[must_use]
pub fn dialog_footer(n: Native<'_>) -> StyleRefinement {
    StyleRefinement::default().gap(px(n.resolved.dialog.button_gap))
}

/// `DialogTitle` (`src/dialog/title.rs:42-43` → `:45`).
#[must_use]
pub fn dialog_title(n: Native<'_>) -> StyleRefinement {
    with_text(StyleRefinement::default(), &n.resolved.dialog.title_font, n)
}

/// `DialogDescription` (`src/dialog/description.rs:49` → `:51`).
#[must_use]
pub fn dialog_description(n: Native<'_>) -> StyleRefinement {
    with_text(StyleRefinement::default(), &n.resolved.dialog.body_font, n)
}

/// Declarative `Table` (`src/table/table.rs:111` → `:114`); rows are inner, Tier U.
#[must_use]
pub fn table(n: Native<'_>) -> StyleRefinement {
    with_text(StyleRefinement::default(), &n.resolved.list.item_font, n)
}

/// `Progress` (`src/progress/progress.rs:128-129` → `:130`); the fill copies
/// the caller's radii (`:92-94, 137, 147`), so height and radius are exact.
#[must_use]
pub fn progress(n: Native<'_>) -> StyleRefinement {
    let p = &n.resolved.progress_bar;
    StyleRefinement::default()
        .h(px(p.track_height))
        .rounded(px(p.border.corner_radius.max(0.0)))
        .min_w(px(p.min_width))
}

/// For `GroupBox::content_style` (`src/group_box.rs:105`, applied `:156-161`).
#[must_use]
pub fn group_box_content(n: Native<'_>) -> StyleRefinement {
    let c = &n.resolved.card;
    StyleRefinement::default()
        .px(px(c.border.padding_horizontal))
        .py(px(c.border.padding_vertical))
        .rounded(px(c.border.corner_radius.max(0.0)))
        .border(px(c.border.line_width))
        .border_color(rgba_to_hsla(c.border.color))
}

/// For `AccordionItem::title_style` (`src/accordion.rs:219`, applied `:300-306`).
#[must_use]
pub fn accordion_title(n: Native<'_>) -> StyleRefinement {
    StyleRefinement::default().h(px(n.resolved.expander.header_height))
}

/// `Checkbox` (`src/checkbox.rs:256` → `:271`); the indicator is inner, Tier U.
#[must_use]
pub fn checkbox(n: Native<'_>) -> StyleRefinement {
    let c = &n.resolved.checkbox;
    with_text(StyleRefinement::default().gap(px(c.label_gap)), &c.font, n)
}

/// `Radio` (`src/radio.rs:196` → `:211`). platform-facts §2.5 defines radio
/// metrics as the checkbox's with a circular indicator, so this is a fact,
/// not a substitution (rationale D18).
#[must_use]
pub fn radio(n: Native<'_>) -> StyleRefinement {
    checkbox(n)
}

/// `Select` (`src/select.rs:479-486` → `:490`); the arrow is inner, Tier U.
#[must_use]
pub fn select(n: Native<'_>) -> StyleRefinement {
    let c = &n.resolved.combo_box;
    with_text(
        StyleRefinement::default()
            .min_h(control_height(c.min_height, &c.font, &c.border, n))
            .min_w(px(c.min_width))
            .rounded(px(c.border.corner_radius.max(0.0))),
        &c.font,
        n,
    )
}

/// `Combobox` (`src/combobox.rs:981-988` → `:992`): same sources as [`select`].
#[must_use]
pub fn combobox(n: Native<'_>) -> StyleRefinement {
    select(n)
}

/// `TitleBar` (`src/title_bar.rs:334` → `:342`); the height has no theme field.
#[must_use]
pub fn title_bar(n: Native<'_>) -> StyleRefinement {
    with_text(StyleRefinement::default(), &n.resolved.window.title_bar_font, n)
}

// --- Size helpers (spec §9.3) -------------------------------------------------

/// `Spinner::with_size` (`src/spinner.rs:53-65` → `Icon`, `src/icon.rs:160, 189`).
#[must_use]
pub fn spinner_size(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.spinner.diameter))
}

/// `Icon::with_size` for toolbar icons (`defaults.icon_sizes.toolbar`).
#[must_use]
pub fn icon_size_toolbar(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.defaults.icon_sizes.toolbar))
}

/// `Icon::with_size` for small icons (`defaults.icon_sizes.small`).
#[must_use]
pub fn icon_size_small(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.defaults.icon_sizes.small))
}

/// `Icon::with_size` for large icons (`defaults.icon_sizes.large`).
#[must_use]
pub fn icon_size_large(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.defaults.icon_sizes.large))
}

/// `Icon::with_size` for dialog icons (`defaults.icon_sizes.dialog`).
#[must_use]
pub fn icon_size_dialog(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.defaults.icon_sizes.dialog))
}

/// `Icon::with_size` for panel icons (`defaults.icon_sizes.panel`).
#[must_use]
pub fn icon_size_panel(n: Native<'_>) -> Size {
    Size::Size(px(n.resolved.defaults.icon_sizes.panel))
}

// --- Builder helpers (spec §9.3) ----------------------------------------------

/// For `Dialog::max_w` (`src/dialog/dialog.rs:393`).
#[must_use]
pub fn dialog_max_width(n: Native<'_>) -> Pixels {
    px(n.resolved.dialog.max_width)
}

/// For `Input::h` (`src/input/input.rs:232`): the same control height [`input`] sets.
#[must_use]
pub fn input_height(n: Native<'_>) -> Pixels {
    let i = &n.resolved.input;
    control_height(i.min_height, &i.font, &i.border, n)
}

// --- Layout accessors (spec §9.5) ---------------------------------------------
// The input is `Theme::layout` on the preset path and `SystemTheme.layout` on
// the system path. No gpui-component widget reads the gpui-base spacing
// tokens, so there is no receiver to map these into; `None` means the
// platform specifies nothing (platform-facts §2.20).

/// Space between adjacent widgets.
#[must_use]
pub fn widget_gap(layout: &LayoutTheme) -> Option<Pixels> {
    layout.widget_gap.map(px)
}

/// Padding inside containers.
#[must_use]
pub fn container_margin(layout: &LayoutTheme) -> Option<Pixels> {
    layout.container_margin.map(px)
}

/// Padding inside the main window.
#[must_use]
pub fn window_margin(layout: &LayoutTheme) -> Option<Pixels> {
    layout.window_margin.map(px)
}

/// Space between major content sections.
#[must_use]
pub fn section_gap(layout: &LayoutTheme) -> Option<Pixels> {
    layout.section_gap.map(px)
}
```

- [ ] **Step 4: Run the gate**

```bash
cargo test -p native-theme-gpui --lib geometry
cargo test -p native-theme-gpui --lib
cargo clippy -p native-theme-gpui --lib --tests -- -D warnings
cargo doc -p native-theme-gpui --no-deps 2>&1 | grep -c warning   # expected 0
```

Expected: PASS; both `control_height` branch tests pass (their preconditions hold for kde-breeze and adwaita at 96 dpi: 31 < 32 and 40 > 32; 28 < 34 and 37 > 34, rationale §2.23); no warnings.

- [ ] **Step 5: Commit**

```bash
git add connectors/native-theme-gpui/src/geometry.rs connectors/native-theme-gpui/src/lib.rs
git commit -m "feat(gpui): geometry module: per-widget StyleRefinement builders, Size helpers, layout accessors

Pure functions over Native { resolved, accessibility }; text sizes carry the
text-scaling factor; control heights grow only when scaled text no longer
fits (max(theme height, ceil(text) + 2 * padding))."
```

---

### Task 12: Showcase port to gpui-kit, `apply`, geometry builders, 139-field colour map (§5.4)

**Model:** Fable 5.1 inline

**Files:**
- Modify: `connectors/native-theme-gpui/examples/showcase-gpui.rs` (6044 lines)

**Interfaces:**
- Consumes: everything Tasks 6–11 produced; gpui-kit 0.6.0 facade (`gpui_kit::application()`, `gpui_kit::init`, `gpui_kit::assets::Assets`, `src/lib.rs:86-144`).
- Produces: a compiling, running example; the gate for `cargo test -p native-theme-gpui` without `--lib`.

The gate is the compile: `cargo check -p native-theme-gpui --example showcase-gpui`. The probe measured 31 first-pass errors; the table below is the documented part, the rest is driven by the compiler output. Work in the order given so each pass removes a class of errors.

- [ ] **Step 1: Documented renames (release notes, gpui-kit README)**

| Old (line) | New |
|---|---|
| `use gpui_component::divider::Divider` (53); `Divider::horizontal()` / `horizontal_dashed()` (3649-3651, 4953, 4961, 5357-5372) | `separator::Separator`, `Separator::horizontal()` / `horizontal_dashed()` |
| `table::Table` (stateful) | `table::DataTable`; `Column`, `TableDelegate`, `TableState` unchanged; `TableDelegate::column` takes `Column` by value (834) |
| `gpui_component_assets::Assets` (5845) | `gpui_kit::assets::Assets` |
| `Application::new()` (5844); `gpui_component::init(cx)` (5847) | `gpui_kit::application()`; `gpui_kit::init(cx)` |
| `InputState::new(..).auto_grow(4, 30)` + `Input::new(&state)` (1663) | `TextareaState::new(..).auto_grow(4, 30)` + `Textarea::new(&state)` |
| `Dialog::new(window, cx)` + `.confirm(..)` (4143) | `Dialog::new(cx)` with `DialogHeader` / `DialogTitle` / `DialogDescription` / `DialogFooter` |
| `theme.scrollbar_show` (1870, 1881, 3739) | `theme.scrollbar_mode` |
| `theme.accordion_hover` (3784, 3830, 5210) | remove the swatch |
| `theme.bullish` / `theme.bearish` (4598-4599, 5236-5237) | `theme.chart_bullish` / `theme.chart_bearish` |
| `theme.list` used as a colour | `theme.colors.list` (`Theme.list` is `ListSettings` now) |
| `IconName::GitHub` (474) | `IconName::Github` |
| `PixelsExt` import (39) | remove (no longer exported) |
| `gpui::Timer` (36; uses at 1130, 1806, 5992, 5994, 6013) | `cx.background_executor().timer(Duration)` inside the async blocks |
| `gpui::Menu { .. }` literals (1549, 1561, 1574, 1584) | add the new `disabled: false` field |
| `AppMenuBar::new(window, cx)` (1592) | one argument, per the 0.6.0 signature (`cargo doc -p gpui-component --open` → `menu::AppMenuBar::new`) |
| `Progress::new()` (2996, 3003, 3010) | `Progress::new(<id>)` per the 0.6.0 signature |
| `Sidebar::left()` (3978) | two arguments per the 0.6.0 signature |
| `BarChart::x(..)` (4478) | removed upstream; drop the call |

- [ ] **Step 2: Replace the three `*Theme::global_mut(cx) = theme;` sites with the install API**

- Line ~1312-1329 (system path at startup): replace the `SystemTheme::from_system()` → `to_theme(..)` → `*Theme::global_mut(cx) = theme;` sequence with `native_theme_gpui::apply_system_theme(&sys, cx);` and keep `sys.pick(sys.mode).clone()` for the sidebar's `resolved` state; keep the `from_preset` fallback but call it with `&AccessibilityPreferences::from_system()` and then `native_theme_gpui::apply(theme, &resolved, &prefs, cx)`.
- Line ~1691-1710 (re-detect on click): `apply_system_theme(&sys, cx)`.
- Line ~1745-1746 (preset switch): `let prefs = AccessibilityPreferences::from_system(); let theme = to_theme(&r.variant, name, self.is_dark, &prefs); native_theme_gpui::apply(theme, &r.variant, &prefs, cx);`.

- [ ] **Step 3: Use the geometry builders where the showcase renders the widgets they cover**

Add `use gpui_component::StyledExt;` and `use native_theme_gpui::{ActiveNativeTheme, geometry};`. In the widget gallery, obtain `let n = cx.native_theme().and_then(|t| t.native(cx));` at the top of each render function that builds these widgets and apply (wrap in `if let Some(n)`; keep the widget unrefined when `None`):

- `Button::new(..)` in the Buttons section → `.refine_style(&geometry::button(n))`;
- `Input::new(&state)` → `geometry::input(n)`; `Textarea` unchanged;
- `Checkbox` / `Radio` → `geometry::checkbox(n)` / `geometry::radio(n)`;
- `Progress::new(..)` → `geometry::progress(n)`;
- `Tooltip` built by the showcase → `geometry::tooltip(n)`;
- `Dialog::new(cx)` → `.refine_style(&geometry::dialog(n)).max_w(geometry::dialog_max_width(n))`, `DialogFooter` → `geometry::dialog_footer(n)`, `DialogTitle` / `DialogDescription` → their builders;
- `GroupBox::new()` → `.content_style(geometry::group_box_content(n))`;
- `AccordionItem` → `.title_style(geometry::accordion_title(n))`;
- `Select` / `Combobox` → `geometry::select(n)` / `geometry::combobox(n)`;
- `Spinner::new()` → `.with_size(geometry::spinner_size(n))`;
- the status bar → `geometry::status_bar(n)`; the title bar → `geometry::title_bar(n)`;
- the "layout" row of the Metrics tab shows `geometry::widget_gap(&layout)` etc. from `SystemTheme.layout` (system path) or `Theme::preset(name).layout` (preset path), printing `None` as "platform specifies nothing".

Add one tooltip line per refined widget naming the `ResolvedTheme` fields the builder reads (the showcase's existing documentation style).

Borrow note: `Native<'_>` borrows `cx` immutably for as long as it lives, so a render function that also needs `cx` mutably (`cx.listener`, `cx.new`, `cx.notify`) computes the refinements it needs into locals first (`let button_style = n.map(geometry::button);` — `StyleRefinement` is owned) and lets `n` go out of scope before the mutable uses; `Native` is `Copy`, so passing it to several builders in a row costs nothing.

- [ ] **Step 4: Colour Map tab: 139 fields**

Add swatches for the 34 new or renamed fields (28 `button_*`, `chart_bullish`, `chart_bearish`, `status_bar`, `status_bar_border`, `table_foot`, `table_foot_foreground`) with their `native-theme` source noted as in §6.2 (e.g. `button_primary ← button.primary_background via primary`). Remove the `accordion_hover` swatch. Update the header comment (line 21: "108-field" → "139-field"), the icon gallery count (435, 4871, 4983: "86" → "101") and add the 15 new `IconName`s to the gallery list at line ~474.

- [ ] **Step 5: Compile and run**

```bash
cargo check -p native-theme-gpui --example showcase-gpui 2>&1 | grep -c '^error'   # iterate until 0
cargo clippy -p native-theme-gpui --all-targets -- -D warnings
cargo test -p native-theme-gpui                                                    # builds the example too
cargo run -p native-theme-gpui --example showcase-gpui
```

Expected: 0 errors; clippy clean; all tests pass; the showcase opens. Visual smoke (judgment step): switch presets and light/dark in the sidebar, confirm the scrollbar keeps its native width and colours after every switch (the observer), confirm buttons show the preset's border and height, confirm the Color Map shows 139 swatches with no transparent-black entries, confirm the Icons tab shows 100 icons in Lucide (StarFill empty, labelled "no Lucide equivalent") and 100 in Material (StarOff empty, labelled "no Material equivalent").

- [ ] **Step 6: Commit**

```bash
git add connectors/native-theme-gpui/examples/showcase-gpui.rs
git commit -m "feat(gpui): port the showcase to gpui-kit 0.6.0; use apply and the geometry builders; 139-field colour map"
```

---

### Task 13: Connector MSRV measurement (§4.4)

**Model:** Opus 5 (`implement` agent)

**Files:**
- Modify: `connectors/native-theme-gpui/Cargo.toml` (`rust-version`)

- [ ] **Step 1: Find the lowest toolchain that compiles the library on the final lock**

```bash
for v in 1.92.0 1.93.0 1.94.0 1.95.0; do
  rustup toolchain install "$v" --profile minimal >/dev/null
  if cargo +"$v" check -p native-theme-gpui --lib --locked >/dev/null 2>&1; then echo "floor=$v"; break; fi
  echo "$v: no"
done
```

Expected: the 0.6.0 closure declares 1.92 as its highest floor and the library compiled on 1.95 in the probe; the first passing version is the floor.

- [ ] **Step 2: Declare it**

Set `rust-version = "<floor>"` in `connectors/native-theme-gpui/Cargo.toml` (replace the provisional `"1.95"`), keep the comment. Confirm the workspace floor still holds for the other members: `cargo +<workspace floor> check -p native-theme --all-targets --locked --features material-icons,lucide-icons,system-icons,svg-rasterize,linux,watch`.

- [ ] **Step 3: Commit**

```bash
git add connectors/native-theme-gpui/Cargo.toml
git commit -m "chore(gpui): declare the measured connector MSRV"
```

---

### Task 14: CI and publish workflows (§5.5, D28)

**Model:** Opus 5 (`implement` agent); Fable 5.1 if the apt-list check surfaces a new library

**Files:**
- Modify: `.github/workflows/publish.yml` (lines 38-44, 58-60, 71-75, 156-175), `.github/workflows/ci.yml` (lines 93-95 if needed)

- [ ] **Step 1: Confirm G11's cause is gone on the new lock**

```bash
cargo tree -p native-theme-gpui -i naga --depth 0; cargo tree -p native-theme-gpui -i codespan-reporting --depth 0
cargo check --workspace --all-targets
```

Expected: `naga v29.x`, `codespan-reporting v0.13.x`; the workspace check succeeds (the incompatibility G11 recorded was naga 27.0.3 vs codespan-reporting 0.12.0).

- [ ] **Step 2: Remove the soft gates**

In `publish.yml`: delete `continue-on-error: true` from the four connector steps (clippy, test, documentation, publish), drop "(soft)" from their names, and delete the three G11 comments (`:38-39`, the "soft-gated for G11" note in the dependency-order comment at `:90`, and `:156-158`). If Step 1 failed instead, keep the gates and replace the G11 comment with the new cause.

- [ ] **Step 3: Re-verify the apt list against what the Linux platform crate links**

The gpui-pre platform crates have no build scripts of their own that probe system libraries (only `gpui-pre-0.3.3/build.rs` exists, and it handles the Windows manifest). The native libraries come from three dependencies of `gpui-pre-linux` (its `Cargo.toml`): `xkbcommon` (links `libxkbcommon`, `:281-284`), `x11rb` with `allow-unsafe-code` (links `libxcb`, `:258-263`) and `wayland-backend` with `client_system` **and** `dlopen` (`:218-224`), which loads `libwayland-client` at run time and needs no development package at build time. List the `-sys` crates in the connector's tree to confirm nothing new joined:

```bash
cargo tree -p native-theme-gpui --prefix none -e normal,build | grep -oE '^[a-z0-9_-]+-sys ' | sort -u
```

Expected: every `-sys` crate is one whose library the existing line `libxcb1-dev libxkbcommon-dev libxkbcommon-x11-dev` (plus what the default Ubuntu runner ships) provides, or one that dlopens. If a new one appears (e.g. `fontconfig-sys`, `freetype-sys`), add its `-dev` package to `ci.yml:95`; the CI run on the PR is the gate.

- [ ] **Step 4: `screenshots.yml`**

No edit unless the macOS/Windows build of the example fails on the `test-support` dev feature (§5.5); the workflow run is the check. If it fails, move the `test-support` dev-dependency under `[target.'cfg(target_os = "linux")'.dev-dependencies]`.

- [ ] **Step 5: Commit**

```bash
git add .github/workflows/publish.yml .github/workflows/ci.yml
git commit -m "ci: gpui connector hard-gated again (G11's naga/codespan conflict is gone on the 0.6 stack)"
```

---

### Task 15: Documentation, changelog, roadmap, todo (§13)

**Model:** Fable 5.1 inline

**Files:**
- Modify: `connectors/native-theme-gpui/README.md`, `connectors/native-theme-gpui/src/lib.rs` (crate-level docs, Step 1b), `CHANGELOG.md`, `ROADMAP.md`, `docs/todo.md`, `docs/todo_gpui-full-theme.md`, `docs/todo_v0.6.0_egui-connector-rationale.md:1079`, `docs/todo_v0.6.0_egui-connector-spec.md:4717`, `docs/archive/v0.5.7_gaps.md` (§G11), `docs/todo_v0.5.8_gpui-component-0.6-spec.md` (Status), `docs/todo_v0.5.8_gpui-component-0.6-rationale.md` (Status), this plan (Status)

- [ ] **Step 1: Connector README (§13.1)**

Rewrite the sections: "What it does" (139 fields, geometry builders, base layer, accessibility); "Quick start" with

```rust,ignore
gpui_kit::application().run(|cx| {
    gpui_kit::init(cx);
    match native_theme::SystemTheme::from_system() {
        Ok(sys) => native_theme_gpui::apply_system_theme(&sys, cx),
        Err(_) => {
            let prefs = native_theme_gpui::AccessibilityPreferences::from_system();
            if let Ok((theme, resolved)) = native_theme_gpui::from_preset("adwaita", false, &prefs) {
                native_theme_gpui::apply(theme, &resolved, &prefs, cx);
            }
        }
    }
    // ...
});
```

State that `gpui_kit::init` (or `gpui_component::init`) runs before `apply`, as upstream requires: `apply` initialises the styled layer only when it is absent, and an `init` after `apply` would reset the theme. Add the recipe "Light and dark under a preset": `from_preset(name, false, &prefs)` + `apply`, then `from_preset(name, true, &prefs)` + `apply`; afterwards `Theme::sync_system_appearance(None, cx)` or `Theme::change` switches between the two native palettes (D34). List the GPUI types the connector's public API exposes (spec §4.2), so readers know what a gpui-pre patch bump can touch. "Core concepts" with the new signatures (`apply_accessibility` rebuilds from the stored variant; `Theme::change` reproduces native colours in both modes once both variants are applied); new sections "Per-widget geometry" (the `refine_style` idiom and §9.2's table of builders with the fields each reads), "How re-application works" (§3.3 in plain terms; the single-writer assumption; the registry-name limit: a same-named theme loaded through `ThemeRegistry` replaces the connector's colours on a registry change, spec §3.3), "Accessibility" (§7.2 table), "GPUI as `gpui-pre`" (§1.1 in plain terms: what the package is, that patch bumps track Zed `main`, that pinning `gpui-pre = "=0.3.N"` in an application is the way to freeze it), a compatibility line "gpui-component 0.6.x · gpui-base 0.6.x · gpui-pre 0.3.x · MSRV <measured>". "What gets mapped": 139 fields; icons: 101 variants, Lucide names are Lucide's own, Material `StarOff` → none. Keep relative image paths (project rule).

- [ ] **Step 1b: Crate-level docs in `connectors/native-theme-gpui/src/lib.rs`**

Rewrite the "Theme Field Coverage" table: "108 color fields" → "139 color fields"; `button` row → all 28 `button_*` fields plus `primary*` / `secondary*`; add `status_bar` (2 of 3) and `table_foot` (2, mirrors `table_head`); replace the "Why the gap" paragraph with a "Per-widget geometry" paragraph pointing at the `geometry` module and `apply`, and a "Limits" sentence pointing at spec §14 for what stays upstream work. Check with `cargo doc -p native-theme-gpui --no-deps` (no warnings).

- [ ] **Step 2: CHANGELOG `[0.5.8]` (§13.2)**

Under the existing `## [0.5.8] - Unreleased` add, keeping the docs.rs entry:

```markdown
### Breaking Changes

#### native-theme-gpui

- Moved to **gpui-component 0.6.0**, **gpui-base 0.6.0** and GPUI published as **`gpui-pre` 0.3.x**; the crate is type-incompatible with applications on gpui-component 0.5 / gpui 0.2.
- `to_theme(resolved, name, is_dark, reduce_transparency: bool)` → `to_theme(resolved, name, is_dark, prefs: &AccessibilityPreferences)`.
- `from_preset(name, is_dark)` → `from_preset(name, is_dark, prefs: &AccessibilityPreferences)`.
- `lucide_name_for_gpui_icon`, `material_name_for_gpui_icon`, `freedesktop_name_for_gpui_icon` return `Option<&'static str>`; `StarFill` has no Lucide equivalent and `StarOff` no Material one, both return `None`. The freedesktop table maps `Star` to `non-starred` (was `starred`, the filled star), so `Star` and `StarFill` render as distinct states. The Lucide table returns Lucide's own names (`Close` → `x`, `Dash` → `minus`, `Inspector` → `scan`, `ResizeCorner` → `grip`, `SortAscending` → `arrow-up-narrow-wide`, `SortDescending` → `arrow-down-wide-narrow`, `WindowMaximize` → `maximize`, `WindowRestore` → `minimize-2`).
- `ScrollbarShow` → `ScrollbarMode`; `IconName::GitHub` → `IconName::Github` (upstream renames).
- The crate declares its own `rust-version` (<measured>), higher than the workspace's, because the gpui-pre closure requires it.

#### native-theme

- Bundled icon names follow upstream: the Lucide bundle no longer contains `close`, `dash`, `inspect`, `resize-corner`, `sort-ascending`, `sort-descending`, `window-close`, `window-maximize`, `window-minimize`, `window-restore` (byte-identical duplicates of `x`, `minus`, `scan`, `grip`, `arrow-up-narrow-wide`, `arrow-down-wide-narrow`, `maximize`, `minimize-2`) or `trash-2` (now `trash`); the Material bundle no longer contains `star_border` (a duplicate of `star`) and stores `font_size` under its upstream name `format_size`. `LucideLoader::new` / `MaterialLoader::new` with those names return `None`.

### Added

- **native-theme-gpui**: `apply`, `apply_system_theme` (a `ThemeConfig` is installed for every stored variant, so upstream's `Theme::change` reproduces native colours in both modes), `apply_accessibility` (rebuilds the styled theme from the stored variant at runtime); `NativeTheme` global with `cx.native_theme()` (`ActiveNativeTheme`); `Native` view; `base_layer` module (native scrollbar geometry/colours and resize-handle colours written onto gpui-base, restored automatically after upstream rebuilds the base theme); `geometry` module (per-widget `StyleRefinement` builders for Button, Input, MenuItem, ListItem, Tooltip, Popover, StatusBar, Dialog family, Table, Progress, GroupBox content, Accordion title, Checkbox, Radio, Select, Combobox, TitleBar; `Size` helpers; layout accessors); text scaling through `Theme.font_size` (rem); reduce-motion forwarded to GPUI; `focus_ring` from `focus_ring_width`; the 15 new `IconName` variants covered in all three tables (`StarFill` has no Lucide equivalent and `StarOff` no Material one; both return `None`).
- **native-theme**: `SystemTheme.layout: LayoutTheme`; `AccessibilityPreferences::from_system()`; 28 bundled SVGs (14 Lucide, 14 Material); `icons/SOURCES.toml` provenance manifest; `scripts/refresh-icons.sh`; by-name icon tables generated by `build.rs` from the bundle directories.

### Changed

- **native-theme-gpui**: `ThemeColor` mapping 108 → 139 fields; the 28 `button_*` fields take the semantic values their variant used in 0.5.1 (solid native surfaces, not upstream's tint); `table_foot*` mirror `table_head*`; `status_bar*` from `StatusBarTheme`. Showcase on gpui-kit 0.6.0.
- Dependency refresh: serde 1.0.229, serde_with 3.22.0, toml 1.1.5, serde_json 1.0.151, arc-swap 1.9.2, async-trait 0.1.92, ashpd 0.13.13, configparser 3.2.0, zbus 5.19.0, quote 1.0.47, proc-macro2 1.0.107, pollster 1.0, syn 3.0.5, resvg 0.48.1. Workspace MSRV re-measured: <measured> (font-types via resvg 0.48).

### Fixed

- Icon bundle provenance recorded; Lucide refreshed to 1.41.0 (`github.svg` kept from 0.577.0, the last tag with brand icons); Material Symbols refreshed to upstream `0cbb08816df0`; duplicate `star_border.svg` removed; the old Lucide `delete.svg` was a trash-can glyph and is now Lucide's real `delete` (the backspace key gpui-kit's own icon draws).
- The connector honoured only `reduce_transparency`; the platform's text-scaling factor and reduce-motion preference were dropped.
- The `ThemeConfig` hex export dropped alpha, so `overlay`, `drag_border` and `drop_target` turned opaque after `Theme::change`; translucent colours are now exported as `#rrggbbaa`.
- `publish.yml`: the gpui connector is hard-gated again (the naga/codespan-reporting conflict G11 recorded does not exist on the 0.6 stack).
```

Under `### Changed` add one more bullet: "- **native-theme-gpui**: `[package.metadata.docs.rs]` keeps `all-features = true` and declares no `targets`: every platform-gated public item of the crate is Linux-gated and appears on the default target (spec §1.3, D40)." Then amend the existing unreleased entry at `CHANGELOG.md:12` from "on `native-theme`, `native-theme-gpui`, and `native-theme-iced`" to "on `native-theme` and `native-theme-iced`".

Fill `<measured>` from Tasks 1 and 13. (If syn stayed on 2.0.119, say so here.)

- [ ] **Step 3: ROADMAP (§13.3)**

Rewrite `## v0.6.2` (line 50-64): "108-field" → "139-field"; the connector-side geometry ships in v0.5.8; v0.6.2 becomes the upstream-only list from spec §14, each row phrased as a PR to gpui-kit (spec §13.5: a styled `Theme` scrollbar-style override honoured by `base_theme()`; `Tab` applying its stored `Styled` refinement; `Theme.shadow` honoured beyond `Button` / `tokens.shadow` consumed; `Size::Size` honoured by Checkbox and Switch; inner geometry: checkbox/radio indicator, switch, slider, separator thickness, resize-handle width, button icon gap, input padding, popup-menu items, select arrow, accordion arrow; a `PopupMenu` item style hook; a `Button::tooltip` style hook; button label text size independent of rem; an iterable `IconName::ALL` generated by `icon_named!`).

- [ ] **Step 4: `docs/todo.md` (§13.5)**

Under "native-theme-gpui connector": mark `Map WidgetMetrics → gpui-component per-widget styling` done for the reachable set (v0.5.8) and list the Tier U items as upstream PR candidates (the list in Step 3); add two research items under a new "Research" heading: "scrollbar thumb radius per platform (platform-facts has none; the connector mirrors gpui-component's `radius`)" and (no second research item: the macOS and Windows readers already fill all four accessibility fields, spec §1.3).

- [ ] **Step 5: `docs/todo_gpui-full-theme.md` (§13.4)**

Add a "v0.5.8" column to the gap table: for each widget row, "delivered (R)" / "delivered (S)" / "delivered (B)" / "upstream (citation)" per spec §9.2 and §14; replace the paragraph "gpui-component's `Theme` / `ThemeConfig` has no fields to receive them" with the seam model of spec §2 (the refinement seam exists on every widget in §9.2).

- [ ] **Step 6: Stale references (§13.7) and cross-references (§13.6)**

- `docs/todo_v0.6.0_egui-connector-rationale.md:1079` and `docs/todo_v0.6.0_egui-connector-spec.md:4717`: `gpui = "0.2.2"` → `gpui = { package = "gpui-pre", version = "0.3.3" }`.
- `docs/archive/v0.5.7_gaps.md` §G11 (line 569): append "Closed in v0.5.8: the 0.6 stack resolves naga 29.0.4 / codespan-reporting 0.13.1; the soft gates were removed."
- `README.md:7` (MSRV badge), `CONTRIBUTING.md:8` and the MSRV CI item in `docs/todo.md:83-87`: the workspace floor measured in Task 1; the todo item also names the gpui connector's floor from Task 13 next to the egui connector's.
- Grep and fix: `grep -rn "108 \|108-field\|86 gpui\|all 86\|gpui-component 0.5\b" --include='*.md' --include='*.rs' . | grep -v docs/archive | grep -v CHANGELOG.md:879` must return nothing.
- Set `Status: Done (v0.5.8)` in the spec, the rationale and this plan.

- [ ] **Step 7: Gate**

```bash
./pre-release-check.sh
```

Expected: every check green (fmt, clippy, panic lint, package). Also check every relative link in the connector README resolves (`grep -o '](\([^)]*\))' connectors/native-theme-gpui/README.md`).

- [ ] **Step 8: Commit**

```bash
git add -A -- '*.md'
git add connectors/native-theme-gpui/src/lib.rs   # Step 1b: crate-level docs
git commit -m "docs(v0.5.8): README, CHANGELOG, ROADMAP and todo for the gpui-component 0.6 connector"
```

---

### Task 16: Release (§15 steps 16–17) — requires the maintainer's explicit approval

**Model:** Fable 5.1 inline

**Files:**
- Modify: `CHANGELOG.md` (date)

- [ ] **Step 1: Prepare**

Set `## [0.5.8] - <today>` and change the compare link at the end of `CHANGELOG.md` from `[0.5.8]: https://github.com/tiborgats/native-theme/compare/v0.5.7...HEAD` to `.../compare/v0.5.7...v0.5.8` (as `v0.5.7` did for its own link, `CHANGELOG.md:947` at that tag). Run `./pre-release-check.sh` once more, then the docs build the way docs.rs will run it (default target; D40):

```bash
DOCS_RS=1 cargo doc -p native-theme-gpui --no-deps --all-features 2>&1 | grep -c warning   # expected 0
```

Commit as `chore(release): v0.5.8`; `git status` must be clean.

- [ ] **Step 2: STOP.** Ask the maintainer for explicit approval to tag and publish. Nothing below runs without it (project rule `feedback_never_bypass_checkpoints`).

- [ ] **Step 3: Tag and publish (only after approval)**

Create the tag `v0.5.8` on the release commit, push `main`, then push that one tag by name (project rule: never push all tags at once, memory `feedback_push_specific_tags`). The publish workflow runs on the tag and publishes in dependency order (derive → build → native-theme → iced → gpui).

- [ ] **Step 4: Post-publish check**

Confirm the docs.rs build of `native-theme-gpui` succeeds on the new stack (`https://docs.rs/crate/native-theme-gpui/0.5.8/builds`, default target, as 0.5.7 and the gpui-pre stack itself build). If it fails, propose the fix as a patch release; never rewrite the published tag.

---

## Deviations from the specification recorded by this plan

1. **Task order.** The bundles (Task 5) come before the connector manifest (Task 6), and the manifest, mechanical first pass and icon tables are one task, so no commit leaves the connector library uncompilable. Now reflected in spec §15 and rationale §2.24.
2. **Ten duplicates, not renames.** Spec §10.2 and rationale §2.15 were corrected on 2026-09-05 (rationale error 39): the ten gpui-named Lucide files are byte-identical to files already present, so Task 5 deletes them; Lucide ends at 107 files, Material at 100.
3. **`reapplying` is set only by the observer** (spec §3.3 / §8.1 corrected, rationale error 40): the subscription activates after the effects `apply` queues, so a flag set in `apply` would go stale. Task 10's test runs `Theme::change` twice.
4. **Fallback for the resize handle** when no variant is stored for the new mode: `border` / `drag_border` from the styled theme, upstream's own projection; now stated in spec §3.3 step 2.
5. **`from_system()` reduce-motion rule** (§11.2 ambiguity): the reader's value where the reader supplies one, OR-ed with `detect::detect_reduced_motion()`; never turned off by the fallback.
6. **First-install gap** (D38, rationale error 46): the observer's subscription activates at the end of the flush that installs it, so `install_observer_once` also queues one deferred re-write of the overrides; the test `apply_then_change_in_the_same_update_keeps_overrides` runs `apply` and `Theme::change` in one update. Now in spec §3.3 and §12.
7. **Star states** (D39, rationale errors 48–49): Lucide `StarFill` returns `None` (Lucide has no filled star; the hollow `star` would stand for two states); the freedesktop table maps `Star` to `non-starred` and `StarFill` to `starred`. Now in spec §10.1, §10.2, §10.4.
8. **Connector docs.rs targets dropped** (D40, rationale errors 50–51) and **MSRV-aware re-update** (spec §4.3): the connector's `[package.metadata.docs.rs]` keeps `all-features` and declares no `targets`, because its only platform-gated public items are Linux-gated and the other two targets would add nothing but the first cross-target build of gpui-pre's platform crates; this reverses commit `ce0fdee` for this one crate and amends its unreleased CHANGELOG entry (Task 15). Task 1 re-runs `cargo update` after declaring the floor because `resolver = "3"` resolves against it.
9. **Highlighter style in the config** (D41, rationale error 52): `to_theme_config` sets `highlight` to upstream's default style for its mode, otherwise `Theme::change` to the other mode keeps the previous mode's code highlighting. **Observer tests assert `active_handle`** (rationale error 53): every preset inherits both splitter colours from the border colour, so `handle` holds the same value whether upstream or the connector wrote it. **`from_system()` uses the uncached `detect_reduced_motion()`** (rationale error 54).

Design changes made during the review of 2026-09-05 and written into the spec (§8.1) and rationale (D34, D35, errors 41–43): `apply` installs a `ThemeConfig` for every stored variant, so `Theme::change` reproduces native colours in both modes; `apply_accessibility` rebuilds the styled theme from the stored variant; the re-apply helper takes a `mark` flag so `apply`'s own write never sets `reapplying`. From the third review (D36, D37, errors 44–45): the config hex export keeps alpha as `#rrggbbaa`, and `apply` ends with `refresh_windows`.

## Self-review against the spec

- §0.1 items 1–10 → Tasks 6 (1, 2), 7 (3), 8 (4), 10 (5), 11 (6), 5+6 (7), 2+3 (8), 1+13 (9), 14+15 (10).
- §4 → Tasks 1, 6, 13. §5.1–5.3 → Tasks 6, 7, 8. §5.4 → Task 12. §5.5 → Task 14.
- §6 → Task 7. §7 → Task 8 (and Task 10 for `reduce_motion`). §8 → Tasks 9, 10. §9 → Task 11. §10 → Tasks 4, 5, 6. §11 → Tasks 2, 3. §12 → tests in Tasks 5–11, MSRV in 1 and 13, `pre-release-check.sh` in 15, screenshots in 12/14. §13 → Task 15. §15 → this plan. §16 open questions: Q1 (`dialog_width`) not offered (unchanged decision); Q2/Q3 (iced parity) out of scope, flagged for v0.6.1; Q4 (`warning.svg`, `info.svg`) resolved by the refresh in Task 5.
- Names used across tasks: `text_scale_factor` (Task 8) used by Task 11; `Native`, `NativeTheme`, `ActiveNativeTheme`, `apply*` (Task 10) used by Tasks 11, 12; `base_layer::{ScrollbarGeometry, scrollbar_geometry, resizable_theme, apply_overrides}` (Task 9) used by Task 10; `AccessibilityPreferences::from_system` (Task 3) used by Task 12 and the README; `SystemTheme.layout` (Task 2) used by Task 12.
