# v0.6.0-rc1 — gpui connector on GPUI Kit 0.7.0: Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Move `native-theme-gpui` and its showcase to gpui-component / gpui-base / gpui-kit 0.7.0 with gpui-pre 0.3.7, fix what 0.7.0 changed behind the compiler's back, show the widgets it added, and leave every claim the crate and its showcase make true for 0.7.0 — all inside v0.6.0-rc1.

**Architecture:** Move the floors and the lockfile and repair the two API breaks (spinner arc, three new icon names with genuine bundled glyphs). Give the new `ThemeColor::chart_grid` the model's grid colour on both theme paths and make the config tripwire count keys instead of trusting a literal. Put the showcase's action handlers in a `RootPlugin` that wraps the view *and* the overlays gpui-base 0.7.0 now mounts beside it, and make its pointer shield a deferred, top-priority element. Let `geometry::toolbar` leave the height to the content, document which resizables the splitter colours reach, add the `Switch` focus ring, the new showcase sections and the coverage exceptions; re-point every citation and correct every false claim; prove with before/after layout dumps that the shared chrome and Basic page did not move.

**Tech Stack:** Rust 2024, gpui-pre 0.3.7 (`#[gpui::test]`, `VisualTestContext`, `debug_bounds`), gpui-component 0.7.0, gpui-base 0.7.0 (`RootPlugin`, `Toolbar`), gpui-kit 0.7.0 (`open_window`), Python 3.11 scripts.

**Spec:** [`todo_v0.6.0-rc1_gpui-kit-0.7-spec.md`](todo_v0.6.0-rc1_gpui-kit-0.7-spec.md) — read it first; it carries every upstream citation and Appendix A. Reasons: [`todo_v0.6.0-rc1_gpui-kit-0.7-rationale.md`](todo_v0.6.0-rc1_gpui-kit-0.7-rationale.md) (K1–K17).

## Global Constraints

- Floors, verbatim: `gpui = { package = "gpui-pre", version = "0.3.7" }`, `gpui-component = "0.7.0"`, `gpui-base = "0.7.0"`, dev `gpui = { package = "gpui-pre", version = "0.3.7", features = ["test-support"] }`, dev `gpui-kit = { version = "0.7.0", features = ["tree-sitter-rust"] }`. Carets, never `=`.
- Connector `rust-version = "1.95.0"` (measured; do not change).
- Workspace version stays `0.6.0`; branch stays `v0.6.0-rc1`; CHANGELOG lines go under `## [Unreleased]`.
- No public item is added, removed or re-typed.
- No `unwrap` / `expect` / `panic!` / `unreachable!` / slice indexing / `unsafe` outside test code, **including the showcase** (it has none today). The PreToolUse hook `.claude/hooks/no-runtime-panics.sh` lets an edit containing `.expect(` through only when the same edited text contains `#[test]`, `#[cfg(test)]` or `#[allow(clippy::unwrap_used` — `#[gpui::test]` does not match; files under `tests/` are exempt by path.
- No invented values: every colour and size comes from the resolved theme or is cited as upstream's literal.
- Every cargo command runs from the **repository root** with `CARGO_BUILD_JOBS=4` (the gpui stack runs this machine out of memory otherwise). Never set `CARGO_TARGET_DIR` under `/tmp` (a RAM-backed tmpfs).
- Before every commit run `cargo fmt -p native-theme-gpui` (the release check runs `cargo fmt --check`). Stage files by path (`git add <paths>`); never `git add -A` / `.`. Commit messages carry no `Co-Authored-By` or any AI attribution.
- Shell snippets are bash; run a multi-line one as `bash <<'EOF' … EOF` (the session's shell may not be bash).
- Never tag, push, publish, create a release, or run `scripts/generate_assets_release.sh`.
- When a step's expected output does not appear, stop and report; do not improvise around it. When cited upstream code no longer says what a comment claims, the claim is the finding: report it, do not reword the comment to fit.

## Review Focus

- **An action fired from inside any overlay** (palette entry, `secondary-b` / `secondary-,` / `secondary-k` while a dialog or sheet has the focus) must reach the showcase — Task 3 pins the palette and one shortcut; the reviewer also tries the Preferences sheet by hand.
- **A capture taken with an overlay open** must show nothing hovered — Task 3's shield test opens the About dialog; menus and popovers are deferred like dialogs, so the reviewer also hovers an open menu under `--capture` by hand.
- **`Theme::change` after `apply`** must keep the native `chart_grid`, not upstream's `border × 0.6` — Task 2's config test.
- **A toolbar with no stated height (KDE)** must be content-sized, also for gpui-component's new `Toolbar`, whose Small size sets `h_8` — Task 4's test pins `h_auto` in `geometry::toolbar`, and Task 9's before/after dumps (which include `kde-breeze`) show the chrome's row did not move.
- **A preset whose icon theme lacks a new freedesktop name** returns `None`, never another set's glyph — Task 1 keeps `every_none_is_an_allowed_gap` and adds no fallback.

## File map

| File | Responsibility in this plan |
|---|---|
| `connectors/native-theme-gpui/Cargo.toml`, `Cargo.lock` | floors (Task 1) |
| `src/widgets/spinner.rs` | `ArcData::new` (Task 1) |
| `src/icons.rs`, `native-theme/icons/{lucide,material}/` | three `IconName`s, three SVGs (Task 1) |
| `src/colors.rs`, `src/config.rs`, `src/contract.rs`, `src/lib.rs` | `chart_grid`, tripwires, counts (Task 2) |
| `examples/showcase-gpui/{host.rs,app.rs,main.rs,tests.rs}`, `src/showcase.rs` | overlay hosting, deferred shield, `open_window` (Task 3) |
| `src/geometry.rs`, `src/base_layer.rs`, `src/lib.rs`, `README.md`, Settings Widget Info | toolbar height; splitter scope, documented (Task 4) |
| `src/widgets/switch.rs`, `src/widgets/tests.rs`, `README.md` | focus ring (Task 5) |
| `src/*.rs`, `tests/seams.rs`, `README.md` | citations, prose (Task 6) |
| `examples/showcase-gpui/info/*.rs`, `demo.rs`, `support.rs`, `inspector.rs`, `chrome.rs` | Widget Info truth (Task 7) |
| `examples/showcase-gpui/pages/{buttons,inputs,charts}.rs`, `demo.rs`, `info/*.rs`, `docs/showcase-exceptions.toml` | new sections, exceptions (Task 8) |
| `target/layout-070/` (not committed) | before/after layout dumps (Tasks 1, 9) |
| `CHANGELOG.md`, `docs/todo.md`, `docs/COMPATIBILITY.toml`, `README.md` | notes, stamp (Task 10) |
| `docs/archive/` | archive (Task 11) |

Paths below are relative to `connectors/native-theme-gpui/` when they start with `src/`, `tests/`, `examples/` or `README.md`, and to the repository root otherwise. Line numbers are those of `cc2f3b81`; locate by content.

---

### Task 1: Floors, lockfile, and the two API breaks

One task because nothing compiles on the new stack until all three parts are in.

**Files:**
- Modify: `Cargo.toml` (connector), `Cargo.lock`
- Modify: `src/widgets/spinner.rs` (`fn ring`)
- Modify: `src/icons.rs` (`icon_name`, the three `*_name_for_gpui_icon` tables, `ALL_ICON_NAMES`, tests, docs)
- Create: `native-theme/icons/lucide/ban.svg`, `native-theme/icons/lucide/circle-alert.svg`, `native-theme/icons/material/block.svg`
- Modify: `examples/showcase-gpui/app.rs:2197-2199`, `examples/showcase-gpui/tests.rs:624-627`, `examples/showcase-gpui/support.rs` (`GPUI_ICONS`, `role_for_gpui_icon`)

**Interfaces:** Consumes nothing. Produces a stack that compiles with `--all-targets`; the known failing tests are listed in Step 11.

- [ ] **Step 1: Precondition.** `git status --short` must show nothing staged and no modified file under `connectors/native-theme-gpui/` or `docs/COMPATIBILITY.toml`. Today the maintainer's pending asset run (READMEs' Verified lines, `docs/COMPATIBILITY.toml`, `docs/assets/*.png`, `docs/assets/PROVENANCE.toml`) is uncommitted: stop and ask the maintainer to commit or stash it. Do not stash it yourself. (Its provenance stamp hashes the connector's `src` and `examples`, which this plan changes, so the maintainer's next asset run restamps it anyway; Task 10 Step 6.)

- [ ] **Step 1a: Commit the design documents** (Task 11 moves them with `git mv`, which needs them tracked), unless the maintainer already has:

```bash
git add docs/todo_v0.6.0-rc1_gpui-kit-0.7-rationale.md docs/todo_v0.6.0-rc1_gpui-kit-0.7-spec.md docs/todo_v0.6.0-rc1_gpui-kit-0.7-plan.md
git commit -m "docs: the gpui connector on GPUI Kit 0.7.0, rationale, specification and plan"
```

- [ ] **Step 1b: Baseline layout dumps (still on gpui-kit 0.6.6).** Needs the desktop session: each run opens the showcase window, writes the dump once the layout settles and quits (`main.rs:693-697`).

```bash
PHASE=before bash <<'EOF'
set -e
out="target/layout-070/$PHASE"
mkdir -p "$out/rest" "$out/menu"
CARGO_BUILD_JOBS=4 cargo build -p native-theme-gpui --example showcase-gpui --locked
bin=target/debug/examples/showcase-gpui
for row in "kde-breeze dark freedesktop breeze-dark" "kde-breeze light freedesktop breeze" \
           "material dark material -" "material light material -" \
           "catppuccin-mocha dark lucide -" "catppuccin-mocha light lucide -"; do
  set -- $row
  extra=(); [ "$4" != "-" ] && extra=(--icon-theme "$4")
  "$bin" --theme "$1" --variant "$2" --icon-set "$3" "${extra[@]}" --tab basic \
    --dump-layout "$out/rest/gpui-$1-$2.json"
  "$bin" --theme "$1" --variant "$2" --icon-set "$3" "${extra[@]}" --tab basic \
    --open-menu theme --dump-layout "$out/menu/gpui-$1-$2.json"
done
ls "$out"/rest "$out"/menu
EOF
```
Expected: twelve JSON files (the pairings of `scripts/generate_screenshots_gpui.sh`). Nothing is committed.

- [ ] **Step 2: Manifest.** In `connectors/native-theme-gpui/Cargo.toml` replace the `rust-version` comment, the `[dependencies]` gpui / gpui-component / gpui-base block, the version in the `image` comment ("gpui-pre 0.3.6" → "0.3.7"), and the dev `gpui` / `gpui-kit` entries with the text of spec §2. Keep every other line.

- [ ] **Step 3: Lockfile**

```bash
cargo update -p gpui-component -p gpui-base -p gpui-kit
```
Needs the network (or `--offline` when the crates are already in the local registry). Expected: `Locking 27 packages`, 27 package `Updating` lines (gpui-base, gpui-component, gpui-component-macros, gpui-kit, gpui-kit-assets 0.6.6 → 0.7.0; 22 `gpui-pre*` 0.3.6 → 0.3.7) besides cargo's "Updating crates.io index", no `Adding`, no `Removing`.

- [ ] **Step 4: See the gate fail**

Run: `CARGO_BUILD_JOBS=4 cargo check -p native-theme-gpui --lib --locked`
Expected: `error[E0639]` and `error[E0061]` at `src/widgets/spinner.rs:240-241`; `error[E0004]` at `src/icons.rs:158, 290, 439` naming `IconName::Ban`, `IconName::CircleAlert`, `IconName::RefreshCw`.

- [ ] **Step 5: Spinner.** In `src/widgets/spinner.rs`, `fn ring`, replace the `.paint(&ArcData { … }, look.color, None, None, &bounds, window)` call with:

```rust
                        .paint(
                            &ArcData::new(&(), 0, end - start, start * TAU, end * TAU),
                            look.color,
                            &bounds,
                            window,
                        );
```

- [ ] **Step 6: Bundle the three glyphs**

```bash
./scripts/update_icons.sh add lucide ban
./scripts/update_icons.sh add lucide circle-alert
./scripts/update_icons.sh add material block
git status --short native-theme/icons/
```
Needs the network: `add` writes a placeholder and then re-downloads *every* bundled file from its pinned ref, writing the fetched bytes unchanged. Expected: exactly three new files, `native-theme/icons/lucide/ban.svg`, `native-theme/icons/lucide/circle-alert.svg`, `native-theme/icons/material/block.svg`, each containing `<svg`, and no other file under `native-theme/icons/` changed. Then `cmp native-theme/icons/lucide/ban.svg ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/gpui-kit-assets-0.7.0/assets/icons/ban.svg` and the same for `circle-alert.svg` exit 0. Anything else: stop.

- [ ] **Step 7: Tables.** In `src/icons.rs` add one arm per table at the alphabetical position:

```rust
        // lucide_name_for_gpui_icon
        IconName::Ban => "ban",
        IconName::CircleAlert => "circle-alert",
        IconName::RefreshCw => "refresh-cw",
```

```rust
        // material_name_for_gpui_icon
        IconName::Ban => "block",          // exact
        IconName::CircleAlert => "error",  // exact
        IconName::RefreshCw => "refresh",  // close: one circular arrow, Lucide's has two
```

```rust
        // freedesktop_name_for_gpui_icon, "freedesktop standard names" block
        IconName::Ban => "action-unavailable", // exact: circle with a slash (Adwaita, Breeze)
        IconName::CircleAlert => "emblem-important", // close: Adwaita a circle with "!", Breeze a bare "!"
        IconName::RefreshCw => "view-refresh", // exact
```

In `icon_name`, under `// Common Actions`, add `IconRole::ActionRefresh => IconName::RefreshCw,` and rewrite its `# Coverage` doc to the text of spec §3.2 (31 mapped, 11 unmapped, "gpui-component 0.7"). In the docs of all three tables (`src/icons.rs:153, 272, 416`), "101 … 0.6.6" becomes "104 … 0.7.0"; `icon_name`'s catch-all comment "No Lucide equivalent in gpui-component 0.6" says 0.7.

- [ ] **Step 8: Tests in `src/icons.rs`.**
  - `ALL_ICON_NAMES` (sorted by name): insert `IconName::Ban,`, `IconName::CircleAlert,` and `IconName::RefreshCw,` at their alphabetical positions.
  - `all_icon_names_count_matches_gpui_component`: `101` → `104`, and its comment ("… 0.6.6's 101 files") → 0.7.0's 104.
  - `icon_name_maps_exactly_30_roles` → rename to `icon_name_maps_exactly_31_roles`, expected count `31`.
  - `icon_name_data_driven` (its comment "Issue 45 … all 30 Some() mappings" → 31): add

```rust
        assert!(matches!(
            icon_name(IconRole::ActionRefresh),
            Some(IconName::RefreshCw)
        ));
```

- [ ] **Step 8b: The showcase's gallery** (a second hand list the compiler cannot check). In `examples/showcase-gpui/support.rs`, `GPUI_ICONS` (`:539-540`) gains `("Ban", IconName::Ban)`, `("CircleAlert", IconName::CircleAlert)` and `("RefreshCw", IconName::RefreshCw)` at their alphabetical positions, and its doc says "The 104 gpui-component 0.7.0 IconName variants"; `role_for_gpui_icon` (`:503-506`) gains `"RefreshCw" => Some(IconRole::ActionRefresh),` beside the other action arms.

- [ ] **Step 9: Showcase compile.** In `examples/showcase-gpui/app.rs` delete the three lines `.children(Root::render_sheet_layer(window, cx))`, `.children(Root::render_dialog_layer(window, cx))`, `.children(Root::render_notification_layer(window, cx))` (Task 3 restores what they did). In `examples/showcase-gpui/tests.rs` replace both `cx.update(|_w, cx| root.read(cx).notification.read(cx).notifications().len())` with `cx.update(|window, cx| window.notifications(cx).len())`, and rename the now-unused `root` binding of that test to `_root`.

- [ ] **Step 10: Compile everything**

Run: `CARGO_BUILD_JOBS=4 cargo clippy -p native-theme-gpui --all-targets --all-features --locked -- -D warnings`
Expected: one error, `unused import: Root` in `examples/showcase-gpui/app.rs`; remove `Root` from that `use` and re-run: clean.

- [ ] **Step 11: See the known failures**

```bash
CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --no-fail-fast 2>&1 | grep -E '^test .* FAILED$|^test result'
CARGO_BUILD_JOBS=4 cargo test -p native-theme --test icon_sources --locked
```
Expected, lib (exactly these ten FAILED): `colors::tests::theme_color_field_count_tripwire`, `colors::tests::no_theme_color_field_is_left_at_default`, `contract::every_theme_color_field_has_a_declared_source`, `compat::the_readme_states_the_manifest_floors`, `compat::the_quick_start_states_the_manifest_floors`, `showcase::every_theme_color_field_has_one_theme_token`, `showcase::every_widget_reports_itself`, `showcase::the_mirrored_handle_constants_match_gpui_base`, `showcase::every_prose_citation_still_exists`, `showcase::every_colour_claim_is_read_at_the_line_it_cites`. Showcase: `tests::the_palette_switches_page`, `tests::the_palette_installs_a_preset`. `tests/seams.rs`: all pass. `icon_sources`: pass. Every `icons::tests::*` passes. Any other failure: stop and report.

- [ ] **Step 12: Commit**

```bash
cargo fmt -p native-theme-gpui
git add Cargo.lock connectors/native-theme-gpui/Cargo.toml connectors/native-theme-gpui/src/widgets/spinner.rs connectors/native-theme-gpui/src/icons.rs connectors/native-theme-gpui/examples/showcase-gpui/app.rs connectors/native-theme-gpui/examples/showcase-gpui/tests.rs connectors/native-theme-gpui/examples/showcase-gpui/support.rs native-theme/icons/lucide/ban.svg native-theme/icons/lucide/circle-alert.svg native-theme/icons/material/block.svg
git commit -m "build(gpui): move to gpui-component, gpui-base and gpui-kit 0.7.0 on gpui-pre 0.3.7; the spinner builds its arc with ArcData::new, and Ban, CircleAlert and RefreshCw map to genuine Lucide, Material and freedesktop glyphs"
```

---

### Task 2: `chart_grid` from `list.grid_color`, and a config tripwire that counts keys

**Files:**
- Modify: `src/colors.rs` (beside `tc.table_row_border = c.list_grid;`; the two tripwire tests; a new test)
- Modify: `src/config.rs` (export; `theme_config_colors_cover_the_0_6_fields`; a new test; module prose)
- Modify: `src/contract.rs` (a row; its count prose)
- Modify: `src/lib.rs` (doc table counts; `every_theme_field_is_named` comment)
- Modify: `examples/showcase-gpui/demo.rs` (`ThemeToken::ChartGrid`), `examples/showcase-gpui/info/theme_map.rs`, `examples/showcase-gpui/pages/theme_map.rs` (Chart group gains `chart_grid`)

**Interfaces:** Consumes Task 1's build. Produces `ThemeColor::chart_grid == rgba_to_hsla(resolved.list.grid_color)` on the direct path and `ThemeConfigColors::chart_grid == Some(hex)` on the config path, which Task 8's chart section claims.

- [ ] **Step 1: Write the failing tests.** In `src/colors.rs`'s test module (it already imports `Theme`, `ColorMode` and `ResolvedTheme` for `resolved_preset`, `src/colors.rs:711-718`):

```rust
    /// K4: a chart's grid is the model's grid-line colour, on every preset in
    /// both variants (platform-facts §2.15: macOS `gridColor`, Material
    /// `outline-variant`, the border colour elsewhere).
    #[test]
    fn chart_grid_is_the_list_grid_colour() {
        for info in Theme::list_presets() {
            for (mode, is_dark) in [(ColorMode::Light, false), (ColorMode::Dark, true)] {
                let theme = Theme::preset(info.key).expect("a listed preset loads");
                let Ok(variant) = theme.into_variant(mode) else {
                    continue;
                };
                let resolved = variant
                    .into_resolved(&native_theme::ResolutionContext::for_tests())
                    .expect("resolved preset must validate");
                let tc = to_theme_color(&resolved, is_dark, false);
                assert_eq!(
                    tc.chart_grid,
                    rgba_to_hsla(resolved.list.grid_color),
                    "{} {mode:?}: chart_grid is not list.grid_color",
                    info.key
                );
            }
        }
    }
```

In `src/config.rs`'s test module (`hsla_to_hex` is the function behind the module's local `h` closure, `src/config.rs:101`):

```rust
    /// K4: `Theme::change` reinstalls this config, so it must carry the
    /// native grid colour, or upstream's `border × 0.6` fallback
    /// (`theme/schema.rs:928`) replaces it.
    #[test]
    fn the_config_carries_the_native_chart_grid() {
        let resolved = test_resolved();
        let tc = crate::colors::to_theme_color(&resolved, true, false);
        let config = to_theme_config(
            &resolved,
            "Grid",
            GpuiThemeMode::Dark,
            &AccessibilityPreferences::default(),
        );
        assert_eq!(
            config.colors.chart_grid,
            Some(SharedString::from(hsla_to_hex(tc.chart_grid)))
        );
    }
```

If `hsla_to_hex` or `SharedString` is not in the test module's scope, import them the way the module's body does.

- [ ] **Step 2: Run them to see them fail**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --lib chart_grid`
Expected: both FAIL (`chart_grid` is transparent black; the config's is `None`).

- [ ] **Step 3: Implement.** Spec §4.1–§4.3 verbatim (in the `src/colors.rs` comment, cite the inheritance as `docs/inheritance-rules.toml:238` rather than paraphrasing it): the `tc.chart_grid = c.list_grid;` line with its comment in `src/colors.rs`; `colors.chart_grid = h(tc.chart_grid);` after `colors.chart_bearish = …` in `src/config.rs`; the `chart_grid` `Row` after `chart_bearish`'s in `src/contract.rs`.

- [ ] **Step 4: The tripwires.** `theme_color_field_count_tripwire`: comment "ThemeColor has 139 Hsla fields in gpui-component 0.7.0", `field_count, 139`. `no_theme_color_field_is_left_at_default`: `139`. `contract::every_theme_color_field_has_a_declared_source`: its literal `138` (`src/contract.rs:1276`) → `139`. Replace `theme_config_colors_cover_the_0_6_fields` with spec §4.4's `theme_config_colors_export_every_key`: its hand list of 15 `is_some` keys and the `exported == 126` count give way to spec §4.4's check of *which* keys are unset (the 12 `base.*` and `group_box.title.foreground`, nothing else), which asserts every other key exported; keep the `drag_border` `#rrggbbaa` assertion.

- [ ] **Step 5: Prove the new config tripwire can fail.** Comment out `colors.chart_grid = h(tc.chart_grid);`, run `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --lib theme_config_colors_export_every_key`. Expected: FAIL, "ThemeConfigColors keys not exported: [\"chart.grid\"]". Restore the line.

- [ ] **Step 6: Theme Map.** Add a `ChartGrid` variant to the `ThemeToken` enum (`examples/showcase-gpui/demo.rs`, where `ChartBearish` is), and `chart_grid` to the Chart group where `chart_bearish` is listed (`examples/showcase-gpui/info/theme_map.rs:1233` region, `pages/theme_map.rs`). Its value claim cites the connector's own `tc.chart_grid = c.list_grid;` line in `src/colors.rs`, as every Theme Map value claim cites the connector line that writes the field.

- [ ] **Step 7: Prose counts.** `grep -n '138\|139\|126\|127' connectors/native-theme-gpui/src/{colors,config,contract,lib}.rs connectors/native-theme-gpui/README.md` and change each hit that counts `ThemeColor` fields (→ 139), exported config colours (→ 127) or `ThemeConfigColors` keys including the 12 private ones (→ 140, e.g. `src/config.rs:94`). Leave every other number.

- [ ] **Step 8: Green for this task**

Run: `CARGO_BUILD_JOBS=4 cargo clippy -p native-theme-gpui --all-targets --all-features --locked -- -D warnings` (the example must build: Step 6 touched it), then `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --lib`
Expected: clippy clean; the two new tests pass; `theme_color_field_count_tripwire`, `no_theme_color_field_is_left_at_default`, `contract::every_theme_color_field_has_a_declared_source`, `showcase::every_theme_color_field_has_one_theme_token` now pass; the remaining failures are exactly the six `compat::*` / `showcase::*` of Task 1 Step 11 other than the theme-token one.

- [ ] **Step 9: Commit**

```bash
cargo fmt -p native-theme-gpui
git add connectors/native-theme-gpui/src/colors.rs connectors/native-theme-gpui/src/config.rs connectors/native-theme-gpui/src/contract.rs connectors/native-theme-gpui/src/lib.rs connectors/native-theme-gpui/README.md connectors/native-theme-gpui/examples/showcase-gpui/demo.rs connectors/native-theme-gpui/examples/showcase-gpui/info/theme_map.rs connectors/native-theme-gpui/examples/showcase-gpui/pages/theme_map.rs
git commit -m "fix(gpui): chart grids draw in list.grid_color on both theme paths, and the config test counts ThemeConfigColors keys so a new upstream colour cannot go unexported"
```

---

### Task 3: The showcase hosts its actions around the overlays; its pointer shield is deferred

**Files:**
- Create: `examples/showcase-gpui/host.rs`
- Modify: `examples/showcase-gpui/main.rs` (`mod host;`, window opening), `examples/showcase-gpui/app.rs` (`init`, `render`, handler visibility, comments), `examples/showcase-gpui/tests.rs` (`open_with`, two new tests)
- Modify: `src/showcase.rs` (`NOT_WIDGET_CONSTRUCTORS`, `SHOWCASE_FILES`)

**Interfaces:** Consumes Task 1. Produces `host::register(cx: &mut App)`, called by `app::init`; the eight `Showcase::on_*` action handlers become `pub(crate)`; `tests::open` / `open_with` return `(Entity<Showcase>, AnyWindowHandle, VisualTestContext)`.

- [ ] **Step 1: Write the failing tests** in `examples/showcase-gpui/tests.rs`, beside `the_palette_installs_a_preset`:

```rust
/// A shortcut pressed while the palette's query has the focus still reaches
/// the showcase: its handlers wrap the overlays (host.rs), not only the view.
#[gpui::test]
fn a_shortcut_works_while_the_palette_has_the_focus(cx: &mut TestAppContext) {
    let (showcase, _window, mut cx) = open(cx, WINDOW_SIZE);
    let before = read(&mut cx, &showcase, |this, _| this.side_panel_visible);
    press(&mut cx, "secondary-k");
    assert!(a_dialog_is_open(&mut cx), "the palette did not open");
    press(&mut cx, "secondary-b");
    assert_ne!(
        read(&mut cx, &showcase, |this, _| this.side_panel_visible),
        before,
        "Ctrl+B inside the palette did not toggle the side panel"
    );
}

/// The capture shield covers an open dialog too: nothing on it is hovered.
/// A dialog paints deferred (gpui-base dialog.rs, `Dialog::render`), so the
/// shield must be deferred above it, not merely last.
#[gpui::test]
fn the_pointer_shield_covers_an_open_dialog(cx: &mut TestAppContext) {
    let (showcase, _window, mut cx) = open(cx, WINDOW_SIZE);
    without_motion(&mut cx);
    cx.update(|_window, cx| {
        showcase.update(cx, |this, cx| {
            this.pointer_shield = true;
            cx.notify();
        });
    });
    run_menu_item(&mut cx, "Help", "About");
    draw(&mut cx);
    assert!(
        settle_on(&mut cx, &showcase, OVERLAY_ABOUT_NAME).is_none(),
        "under the shield the About dialog is still hovered"
    );
}
```

The second element of `open`'s result is bound as `_window` here; it is still `Entity<Root>` until Step 5 changes `open`, which is fine for these tests.

- [ ] **Step 2: Run them to see them fail**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --example showcase-gpui -- a_shortcut_works_while_the_palette_has_the_focus the_pointer_shield_covers_an_open_dialog the_palette`
Expected: FAIL for the two new tests and the two palette tests (the shield test would fail on 0.6.6 too: the gap is older than 0.7.0).

- [ ] **Step 3: The plugin.** Create `examples/showcase-gpui/host.rs`:

```rust
//! The showcase's actions, hosted around the whole window.
//!
//! gpui-base 0.7.0's `Root` mounts the view and, beside it, one overlay per
//! registered plugin (gpui-base root.rs, `Root::render`); gpui-component's
//! `WindowState` draws dialogs, sheets and notifications there. An action
//! dispatched from inside one of them walks only its own ancestors
//! (gpui-pre window.rs, `dispatch_action`), so handlers on the view's own
//! element never see it. `decorate` wraps the finished surface — view and
//! overlays — so the handlers sit around both.

use gpui::{
    AnyElement, App, Context, InteractiveElement as _, IntoElement, ParentElement as _, Render,
    Styled as _, Window, div,
};
use gpui_base::{Root, RootPlugin};

use crate::app::{
    OpenAbout, OpenCommandPalette, OpenPreferences, ReloadTheme, SetColorMode, SetPreset,
    ShowPage, Showcase, ToggleSidePanel,
};

/// Registered once, after `gpui_kit::init` and before any window opens (a
/// `Root` instantiates the plugins registered when it is created, gpui-base
/// root.rs, `Root::new`), so it wraps outside gpui-component's `WindowState`.
pub(crate) struct ShowcaseHost;

impl Render for ShowcaseHost {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}

macro_rules! forward {
    ($surface:expr, $showcase:expr, $($action:ty => $method:ident),+ $(,)?) => {
        $surface
            $(.on_action({
                let showcase = $showcase.clone();
                move |action: &$action, window: &mut Window, cx: &mut App| {
                    showcase.update(cx, |this, cx| this.$method(action, window, cx));
                }
            }))+
    };
}

impl RootPlugin for ShowcaseHost {
    fn decorate(
        &self,
        surface: AnyElement,
        root: &Root,
        _window: &mut Window,
        _cx: &mut App,
    ) -> impl IntoElement {
        let Ok(showcase) = root.view().clone().downcast::<Showcase>() else {
            return surface;
        };
        forward!(
            div().relative().size_full(),
            showcase,
            ShowPage => on_show_page,
            SetColorMode => on_set_color_mode,
            ReloadTheme => on_reload_theme,
            ToggleSidePanel => on_toggle_side_panel,
            SetPreset => on_set_preset,
            OpenCommandPalette => on_open_command_palette,
            OpenPreferences => on_open_preferences,
            OpenAbout => on_open_about,
        )
        .child(surface)
        .into_any_element()
    }
}

/// Register the host for every window opened after this call.
pub(crate) fn register(cx: &mut App) {
    Root::register_plugin::<ShowcaseHost>(cx, |_, _| ShowcaseHost);
}
```

Adjust the `use crate::app::…` path to where the action types and `Showcase` are defined (grep their definitions), and the action list to exactly the `.on_action(cx.listener(Self::…))` calls in `Showcase::render` (`app.rs:2168-2175`).

- [ ] **Step 4: Wire it.**
  - `main.rs`: `mod host;`.
  - `src/showcase.rs`: add `"host.rs"` to `SHOWCASE_FILES` (otherwise `showcase::the_gates_read_every_showcase_file` fails).
  - `app.rs` `init`: after `cx.on_action(|_: &Quit, cx| quit(cx));` add `crate::host::register(cx);`, and extend its doc: the showcase's actions are handled around the window by `host::ShowcaseHost`.
  - `app.rs`: make the eight `on_*` handlers `pub(crate)`.
  - `app.rs` `Showcase::render`: remove the eight `.on_action(cx.listener(Self::on_…))` calls; keep `.track_focus(&self.focus_handle)`. The `.when(self.pointer_shield, …)` child stays last and wraps its `div` in `gpui::deferred(…).with_priority(usize::MAX)` (spec §5.2); its comment becomes: deferred at the highest priority, so it is the topmost hitbox — dialogs, menus and popovers paint deferred themselves (gpui-base dialog.rs, popup.rs), above any ordinary element, and a deferred element is painted by priority wherever it sits; a popup nested in one of them still lands above it. Rewrite the comment block above the `div()` (it says `Root` does not draw its layers) to: overlays are drawn beside this view by gpui-component's `WindowState` plugin (gpui-base root.rs, `Root::render`); the actions are hosted around both by `host::ShowcaseHost`.
  - `tests.rs`: the notification test's comment "pushes one onto the Root's own layer" → "pushes one onto the window's notification layer".

- [ ] **Step 5: `open_window`.** `main.rs`: replace the `cx.open_window(options, |window, cx| { … cx.new(|cx| Root::new(showcase, window, cx)) })` block and the `showcase_entity` cell with

```rust
            let opened = gpui_kit::open_window(options, cx, |window, cx| {
                cx.new(|cx| {
                    let mut s = Showcase::new(window, cx);
                    apply_cli_args(&mut s, &cli_args, window, cx);
                    s
                })
            });
            let Ok((window_handle, showcase_entity)) = opened else {
```

continuing with the existing failure path (`eprintln!` and exit) unchanged. `window_handle` is now an `AnyWindowHandle`: its four `*window_handle` uses (among them `dump_layout`, `:1482-1489`) become `window_handle`; the `dump_layout` guard and the macOS block lose their `Option` around `showcase_entity`; the `#[cfg(not(target_os = "macos"))] let _ = &showcase_entity;` line and its comment are deleted (`dump_layout` now uses the entity on every platform). `tests.rs` `open_with`: open the same way, `VisualTestContext::from_window(window_handle, cx)`, return `(showcase, window_handle, cx)`; change `open` / `open_with`'s return type to `(Entity<Showcase>, AnyWindowHandle, VisualTestContext)` and import `gpui::AnyWindowHandle`. Call sites binding `_root` need no change. Remove the `Root` (`main.rs`), `RefCell` and `Deref` (`tests.rs`) imports the compiler reports unused; `Rc` and `Root` stay in `tests.rs` (`:148` calls `window.root::<Root>()`). The panic hook blocks an Edit whose new text contains `.expect(` without a test marker, and `open_with` is a plain `fn`: keep its existing `.expect("the window opened")` line out of the edited text (edit the lines above it; the two other `.expect(` lines are deleted, which the hook allows).

- [ ] **Step 6: The constructor exemptions.** In `src/showcase.rs` `NOT_WIDGET_CONSTRUCTORS` delete the three `Root::render_*_layer` entries and the `main.rs` `Root::new` entry.

- [ ] **Step 7: Run the tests**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --example showcase-gpui`
Expected: all pass (176 in the trial), including the four of Step 2, `a_capture_hovers_nothing` and `a_menu_acts_after_the_focused_widget_left_the_page`.
Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --lib -- showcase::every_widget_reports_itself showcase::the_gates_read_every_showcase_file`
Expected: PASS.

- [ ] **Step 8: Prove the priority matters.** Step 2 failing and Step 7 passing already prove the change as a whole. To isolate the shield's `deferred`, replace `deferred(…).with_priority(usize::MAX)` in `app.rs` by the bare `div` (the `deferred` import then goes unused) and re-run `the_pointer_shield_covers_an_open_dialog`: it fails. Restore.

- [ ] **Step 9: Lint and commit**

```bash
CARGO_BUILD_JOBS=4 cargo clippy -p native-theme-gpui --all-targets --all-features --locked -- -D warnings
cargo fmt -p native-theme-gpui
git add connectors/native-theme-gpui/examples/showcase-gpui/host.rs connectors/native-theme-gpui/examples/showcase-gpui/main.rs connectors/native-theme-gpui/examples/showcase-gpui/app.rs connectors/native-theme-gpui/examples/showcase-gpui/tests.rs connectors/native-theme-gpui/src/showcase.rs
git commit -m "fix(showcase-gpui): a RootPlugin hosts the showcase's actions around the overlays gpui-base 0.7.0 mounts beside the view, so palette entries and shortcuts work over dialogs and sheets again; the capture shield is deferred above every top-level overlay, which it never was over dialogs, menus and popovers; the window opens through gpui_kit::open_window"
```

---

### Task 4: `geometry::toolbar` leaves the height to the content; the splitter scope, documented

**Files:**
- Modify: `src/geometry.rs` (`toolbar` and its doc; `toolbar_carries_the_models_toolbar`)
- Modify: `src/base_layer.rs` (`resizable_theme`'s doc), `src/lib.rs:72` (the `splitter` row), `README.md:22-25, 141, 244, 503-504`
- Modify: the `Settings` Widget Info (grep `examples/showcase-gpui/info/` for `Settings`)

**Interfaces:** `geometry::toolbar`'s refinement now carries `size.height = Some(Length::Auto)`.

- [ ] **Step 1: Write the failing assertion.** In `src/geometry.rs`'s `toolbar_carries_the_models_toolbar` (which sweeps every preset in both modes and binds the builder's output as `out`), add `assert_eq!(out.size.height, Some(gpui::Length::Auto), "the toolbar's height is its content's");`. Run `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --lib toolbar_carries_the_models_toolbar` — FAIL.

- [ ] **Step 2: Implement** spec §8.1: in `toolbar`, start the refinement with `.h_auto()` before the optional `min_h(bar_height)`; the doc says the height is the content's, at least `toolbar.bar_height` where stated (citing `docs/platform-facts.md:1351` for KDE and GNOME sizing to content, and gpui-component's `Toolbar` `h_8`, `toolbar.rs:252-256`), and that an application wanting a fixed height sets it after the refinement. Re-run Step 1's test — PASS.

- [ ] **Step 3: Docs**, the text of spec §6 and §9.2's `README.md:244` row:
  - `resizable_theme`'s doc and `README.md:22-25, 503-504`: the colours reach every handle gpui-base draws with its built-in line — groups built with `gpui_base::h_resizable` / `v_resizable` (`gpui_kit::base::h_resizable` / `v_resizable`) — as colours at gpui-base's 1 px width, `splitter.hover_color` only while dragged; not groups built with `gpui_component::h_resizable` / `v_resizable`, `Settings`' divider or the dock's edges, which install gpui-component's own renderer.
  - `src/lib.rs:72` row `splitter`: "divider/hover colours via `base_layer::resizable_theme`, drawn by gpui-base's resizables at their 1 px width (not gpui-component's, `Settings` or dock edges)".
  - `README.md:141`: the same scope in the sentence that lists what `apply` writes.
  - `README.md:244` (`toolbar` row): gpui-component 0.7.0 has a `Toolbar`; `geometry::toolbar` applies to it (the height is the content's); add items with `content()` — `child()` turns buttons into compact ghosts in a 1.5 rem box.
  - The `Settings` Widget Info gains `.not_themeable("divider", "gpui-component's own renderer, a border hairline with a muted_foreground pill, installed inside Settings (setting/settings.rs, Settings; resizable.rs, resize_handle_appearance), which no caller can replace, so splitter.* does not reach it")` — the `file.rs, Symbol` form is the one the prose gate reads.

- [ ] **Step 4: Check and commit**

```bash
CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --lib geometry
RUSTDOCFLAGS="-D warnings" CARGO_BUILD_JOBS=4 cargo doc -p native-theme-gpui --no-deps --all-features --locked
CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --lib showcase::every_prose_citation_still_exists 2>&1 | grep -i 'settings\|resize_handle_appearance'
cargo fmt -p native-theme-gpui
git add connectors/native-theme-gpui/src/geometry.rs connectors/native-theme-gpui/src/base_layer.rs connectors/native-theme-gpui/src/lib.rs connectors/native-theme-gpui/README.md connectors/native-theme-gpui/examples/showcase-gpui/info
git commit -m "fix(gpui): geometry::toolbar leaves the height to the content, so gpui-component 0.7.0's Toolbar does not keep its fixed h_8 where the platform sizes toolbars to content; the splitter colours are documented as reaching gpui-base's resizables only"
```
Expected: the geometry tests pass; the doc build is clean; the prose-citation gate lists neither citation of the `Settings` note (the gate reads them, since they are in its `file.rs, Symbol` form).

---

### Task 5: The connector's `Switch` draws the keyboard focus ring

**Files:**
- Modify: `src/widgets/switch.rs` (`RenderOnce for Switch`, the type's doc)
- Modify: `src/widgets/tests.rs`
- Modify: `examples/showcase-gpui/info/inputs.rs` (Switch entries), `README.md` (Accessibility)

**Interfaces:** None new; behaviour only.

- [ ] **Step 1: Write the test** in `src/widgets/tests.rs`, beside `a_switch_toggles_on_click_and_from_the_keyboard`:

```rust
/// K10: the switch's own focus handle is the base switch's one tab stop. In
/// this harness the switch is the only focusable element, so Tab from it
/// comes back to the same handle; a second tab stop would take the focus.
#[gpui::test]
fn a_switch_keeps_one_tab_stop(cx: &mut TestAppContext) {
    let (cx, value, calls) = toggler(cx, false, true);
    focus_next(cx);
    let first = cx.update(|window, cx| window.focused(cx));
    activate_key(cx, "space");
    assert_eq!((value.get(), calls.get()), (true, 1));
    redraw(cx);
    focus_next(cx);
    assert_eq!(
        cx.update(|window, cx| window.focused(cx)),
        first,
        "Tab from the only switch landed on a second tab stop"
    );
}
```

- [ ] **Step 2: Implement** spec §7: in the native branch of `render`, create the keyed focus handle and `focused` exactly as `src/widgets/checkbox.rs:563-567` does; add `.track_focus(&focus_handle)` to the `BaseSwitch::new(self.id)` chain; add `.when(focused && !self.disabled, |track| track.focus_ring_style(window, cx))` to the `SwitchTrack` chain after its `.bg(…)`. `focus_ring_style` is a `ThemeStyled` method: import `gpui_component::ThemeStyled as _`. Document the ring in `Switch`'s doc comment: "While focused, the track draws the focus ring in `ThemeColor::ring` (`defaults.focus_ring_color`), as gpui-component's switch does (switch.rs:269-272)."

- [ ] **Step 3: Run the widget tests**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --lib widgets`
Expected: all pass, including the new test, `a_switch_toggles_on_click_and_from_the_keyboard` and `a_disabled_switch_is_inert`.

- [ ] **Step 4: Prove the test can fail.** Temporarily give the track a second tab stop of its own — a second keyed handle, `window.use_keyed_state((self.id.clone(), "probe"), cx, |_, cx| cx.focus_handle().tab_stop(true))`, passed to the track's `.track_focus(…)` — and re-run `a_switch_keeps_one_tab_stop`: it fails. Remove it.

- [ ] **Step 5: See the ring.** Run the showcase (`CARGO_BUILD_JOBS=4 cargo run -p native-theme-gpui --example showcase-gpui -- --theme kde-breeze --variant light --tab inputs`, then `--variant dark`), Tab to an *unchecked* switch: the track draws the ring. Report both (the ring is not observable headlessly, and a borderless track's 50 % halo is least visible on an unchecked track, rationale K10).

- [ ] **Step 6: Showcase claim and README.** In `examples/showcase-gpui/info/inputs.rs`, every *enabled* Switch entry gains a focus-ring colour claim, per spec §7: upstream's switch `claim("focus ring", "ring", t.ring, "gpui-component/styled.rs:175-189")`; the connector's (`native_switch(r, …)`, no `Theme` in hand) `claim("focus ring", "focus_ring_color", stated(r.defaults.focus_ring_color), "native-theme-gpui/colors.rs:197")`, in the file's own `claim(…)` form. Run `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --lib showcase::every_colour_claim_is_read_at_the_line_it_cites 2>&1 | grep -i 'focus'` — nothing listed. `README.md` `## Accessibility`: one line — the native `Switch` draws the keyboard focus ring in `defaults.focus_ring_color`.

- [ ] **Step 7: Commit**

```bash
cargo fmt -p native-theme-gpui
git add connectors/native-theme-gpui/src/widgets/switch.rs connectors/native-theme-gpui/src/widgets/tests.rs connectors/native-theme-gpui/examples/showcase-gpui/info/inputs.rs connectors/native-theme-gpui/README.md
git commit -m "feat(gpui): the native Switch draws the keyboard focus ring, as gpui-component 0.7.0's own switch does"
```

---

### Task 6: Library and README citations, and the prose that 0.7.0 made false

**Files:**
- Modify: every connector file named in spec Appendix A (`src/geometry.rs` 89 entries, `src/contract.rs` 39, `src/colors.rs` 20, `tests/seams.rs` 18, `src/lib.rs` 11, `src/base_layer.rs` 7, `src/config.rs` 6, `src/icons.rs` 4, `src/widgets/switch.rs` 3, `src/widgets/mod.rs` 2, `src/showcase.rs` 2, `src/variants.rs` 1, `README.md` 1)
- Modify: `README.md` floors (`:63-66`, `:92`, `:482`), counts (`:41-48`, `:505`, `:520`), the Quick start's window line, the Accessibility row's rem sentence (`:380`)
- Modify: `scripts/check_showcase_parity.py:184-185` (a gpui-pre citation)

**Interfaces:** None. Documentation only; the `compat` tests are the gate for the floors.

- [ ] **Step 1: See the floor tests fail**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --lib compat`
Expected: 2 FAIL (`README.md:63-64` and `:92` state 0.6.6).

- [ ] **Step 2: README floors.** Table `:63-66` → 0.7.0 / 0.7.0 / 0.3.7 / 0.7.0; Quick start `:92` → `gpui-kit = "0.7.0"`; `:482` → `version = "0.3.7"`; `:41-48` → "(104 variants)", audited "against 0.7.0's `default-icons.txt`, which added `ban`, `circle-alert` and `refresh-cw` to 0.6.6's 101" (`:45`'s "0.6.1 … keeps the same 101 variants" is history and stays); `:505` → "(104 variants)"; `:520` → "139-field colour map and a 104-icon gallery"; Quick start: `// open windows as usual` becomes a `gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| cx.new(|_| MyView))` call whose `Result` the example handles without `unwrap` (e.g. `if let Err(error) = … { eprintln!(…) }`), and one sentence under it: a window opened before `gpui_kit::init` gets no `WindowState` — no dialogs, no rem, so text scaling fails silently; `:380`: "`Root` sets the window rem" → "gpui-component's `WindowState` plugin sets the window rem" (spec §9.2). `scripts/check_showcase_parity.py:184-185`: re-read gpui-pre 0.3.7 `window.rs:4665` and cite it. Do **not** touch the `<!-- compat:begin -->` line (Task 10 writes it). Re-run Step 1: PASS.

- [ ] **Step 3: Apply Appendix A.** Work file by file. For each entry: open the connector line, open the *new* upstream line in the 0.7.0 / 0.3.7 source, confirm it says what the connector's prose claims, rewrite the number. An entry marked `CHANGED(…)` or `moved:` is handled in Step 4, not here. A module `//!` header naming the verified stack becomes "gpui-component 0.7.0, gpui-base 0.7.0, gpui-pre 0.3.7".

- [ ] **Step 4: The rewritten citations** — spec §9.2, row by row. Each is re-read in 0.7.0 and the prose rewritten to what 0.7.0 does; where the prose's *claim* no longer holds (e.g. `src/contract.rs:110-112`, `src/lib.rs:915-917`), say what is true now and cite it.

- [ ] **Step 5: Check the work is complete**

```bash
python3 - <<'EOF'
import re, pathlib
root = pathlib.Path("connectors/native-theme-gpui")
text = "\n".join(p.read_text() for p in [*root.glob("src/**/*.rs"), *root.glob("tests/*.rs"), root / "README.md"])
for needle in ["root.rs:582", "`:590`): the installed", "mod.rs:287-289", "mod.rs:283-284",
               "(`:535`, applied", "applied `:156-161`", "resize_handle.rs:12`",
               "(101 variants)", "101-icon gallery", "138-field", "`Root` sets the window rem",
               "open windows as usual"]:
    print(needle, text.count(needle))
EOF
```
Expected: every count 0. (Each needle is a text the connector still holds when Task 6 starts, so each check can fail.)

- [ ] **Step 6: Build, test, commit**

```bash
CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --lib
CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --test seams
cargo fmt -p native-theme-gpui
git add connectors/native-theme-gpui/src connectors/native-theme-gpui/tests connectors/native-theme-gpui/README.md scripts/check_showcase_parity.py
git commit -m "docs(gpui): every upstream citation in the connector and its README is read in gpui-component 0.7.0, gpui-base 0.7.0 and gpui-pre 0.3.7, and the claims 0.7.0 made false say what it does now"
```
Expected: the lib's only failures are the three `showcase::*` gates Task 7 owns (`every_colour_claim_is_read_at_the_line_it_cites`, `every_prose_citation_still_exists`, `the_mirrored_handle_constants_match_gpui_base`); `every_widget_reports_itself` passes since Task 3, the `compat` tests since Step 2.

---

### Task 7: The showcase's Widget Info is true for 0.7.0

**Files:**
- Modify: `examples/showcase-gpui/info/*.rs`, `demo.rs`, `support.rs`, `inspector.rs`, `chrome.rs`, `app.rs:304`, `tests.rs:3113`

**Interfaces:** None. The gates are the proof.

- [ ] **Step 1: See the gates fail**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked --lib showcase:: 2>&1 | grep -E 'FAILED|of [0-9]+ (colour claims|prose citations)'`
Expected: `every_colour_claim_is_read_at_the_line_it_cites` (about 290 of about 710 — the 234 upstream moves plus the Theme Map's claims, which cite `src/colors.rs` lines Task 2 shifted; 291 of 710 in the replay; the gate prints the exact list), `every_prose_citation_still_exists` (1 of 701), `the_mirrored_handle_constants_match_gpui_base`.

- [ ] **Step 2: Colour claims.** For each line the gate prints ("claims `X` at <file>:<old> … that line now reads …"): find where 0.7.0 reads `X` for that widget and state, and cite that line. If 0.7.0 reads a *different* field there, the claim is wrong: rewrite the claim, not just the number. This includes the Bar, Line and Area charts' grid claims, which become `chart_grid` (spec §8.4; Candlestick keeps `border`).

- [ ] **Step 3: Prose and constants.** `info/typography.rs:350`: cite gpui-base `text/node.rs:3324-3334` (`BlockNode::Heading`) for the heading ladder instead of `heading_base_font_size`. `demo.rs:2051, 2056`: cite `resizable/resize_handle.rs:12` and `:13`.

- [ ] **Step 4: What no gate sees** — spec §9.3's list, item by item (Attachment; List/Tree right-click with the `ListRowState` split; Textarea; Popover; Root prose; stale comment lines), and `info/icons.rs:49` ("101 in gpui_component::IconName" → 104). The chart `no_hover` claims are replaced in Task 8; leave them here.

- [ ] **Step 5: Green and commit**

```bash
CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked
cargo fmt -p native-theme-gpui
git add connectors/native-theme-gpui/examples/showcase-gpui
git commit -m "docs(showcase-gpui): Widget Info cites gpui-component 0.7.0 and says what it draws: the Attachment chip, the right-clicked row's outline, the Textarea's size, the WindowState plugin that now sets the rem size and font"
```
Expected: every test of the package passes.

---

### Task 8: New showcase sections and the coverage exceptions

**Files:**
- Modify: `examples/showcase-gpui/demo.rs` (helpers), `pages/buttons.rs`, `pages/inputs.rs`, `app.rs` (two state entities), `info/{buttons,inputs,charts}.rs`, `tests.rs` (one test)
- Modify: `docs/showcase-exceptions.toml` (`[gpui]`, header)

**Interfaces:** New `Showcase` fields: `time_field_state: Entity<TimeFieldState>`, `token_input_state: Entity<InputState>`, built in `Showcase::new` beside `date_picker_state` (`app.rs:1353`). Every new widget is built in a `demo.rs` helper that calls `.info(…)`; built directly in a page it fails `every_widget_reports_itself`.

- [ ] **Step 1: See the coverage gate fail**

Run: `python3 scripts/check_widget_coverage.py`
Expected: gpui `missing 13` (the list of spec §1) and the stale `Text` exception.

- [ ] **Step 2: Buttons page — component Toolbar**, exactly as spec §8.2 specifies it (ids, groups, icons, spacing, glyph fallback, per-button Widget Info `info::buttons::toolbar_item`). The helper uses the same `native_info(…, geometry::toolbar, "toolbar", …)` refinement as `demo::toolbar` (`:907-951`) and the `.info(ui, "buttons-toolbar", …)` wrapper, but none of that row's padding / gap fallbacks and no bottom edge. Its Widget Info entry in `info/buttons.rs`, modelled on its neighbours, states the Small literals (`toolbar.rs:252-256`) that the refinement replaces (`:258`), the groups' spacing, and `child()`'s `ghost().compact()` and 1.5 rem `input_h` wrapper (`toolbar.rs:17-37`, `button.rs:564-566`, `sizing.rs:273`); prose citations in the `file.rs, Symbol` form the prose gate reads. Add the section to `pages/buttons.rs`.

- [ ] **Step 3: Inputs page — TimeField and the token input** (spec §8.3). In `Showcase::new` (validated in the trial):

```rust
        let time_field_state = cx.new(|cx| TimeFieldState::new(window, cx));
        let token_input_state = cx.new(|cx| {
            let mut state = InputState::new(window, cx);
            if let Err(error) =
                state.replace_with_token(InlineToken::new("mention", "@native-theme"), window, cx)
            {
                eprintln!("showcase: the sample token was refused: {error:?}");
            }
            state
        });
```

`app.rs` imports `gpui_component::{input::InlineToken, time_field::TimeFieldState}` and `demo.rs` imports `gpui_component::{input::InputToken, time_field::TimeField}` — each file only what it uses, or clippy's `-D warnings` fails (the paths the trial compiled with; `input::InputState` is already imported). `demo.rs` helpers render `TimeField::new(&state)` and `Input::new(&state).token(|ctx, _, _| InputToken::new(ctx))`, each with a Widget Info entry modelled on `info::inputs::date_picker` (`info/inputs.rs:1108-1135`) carrying spec §8.3's colours and citations (`input_background` / `foreground` cite `input/input.rs:98-106`). In `tests.rs`, `the_token_input_holds_its_token`: open the showcase, read `token_input_state`, and assert exactly one token span whose `.token().text()` is `"@native-theme"` (`InlineTokenSpan::range()` / `.token()`; its fields are private) — a refused token would otherwise show only as an `eprintln!`.

- [ ] **Step 4: Charts page — the hover claims** (spec §8.4): every chart's `no_hover` replaced by the tooltip claims — the surface `popover` citing `gpui-component/styled.rs:197`, the label `muted_foreground`, and the guide line per chart (Line / Area dashed `border`→`foreground`, Bar / Candlestick an 8 % `foreground` band, Pie none) — and the hover-dimming literals. No new chart (rationale K11); the grid claims already moved to `chart_grid` in Task 7.

- [ ] **Step 5: Exceptions** (spec §8.5): remove `Text`; add `Questionnaire` and its seven state-built parts under a comment naming the new seventh reason kind (and add that kind to the file header's list), and `QuestionnaireChoiceDescription` as a composition slot, with the reason texts of spec §8.5; re-read every remaining `[gpui]` entry at gpui-component 0.7.0 and correct its line numbers and `WindowBorder`'s reason; the header's version sentence says 0.7.0.

- [ ] **Step 6: Gates**

```bash
python3 scripts/check_widget_coverage.py
CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked
```
Expected: "Every widget is shown or excepted."; every test passes.

- [ ] **Step 7: Look at it.** `--screenshot` self-captures only on macOS (on Linux the showcase says so and keeps running, `main.rs:1573-1580`), so capture as `scripts/generate_screenshots_gpui.sh` does, with `capture_showcase` from `scripts/capture_window.sh` (KDE Wayland, spectacle):

```bash
bash <<'EOF'
set -e
source scripts/capture_window.sh
mkdir -p target/layout-070
CARGO_BUILD_JOBS=4 cargo build -p native-theme-gpui --example showcase-gpui --locked
for tab in buttons inputs charts; do
  target/debug/examples/showcase-gpui --theme kde-breeze --variant light --icon-set freedesktop \
    --icon-theme breeze --tab "$tab" --capture &
  PID=$!
  sleep 3
  capture_showcase gpui "$PID" "target/layout-070/$tab.png"
  kill -CONT "$PID" 2>/dev/null || true; kill "$PID" 2>/dev/null || true; wait "$PID" 2>/dev/null || true
done
EOF
```
View each PNG with the Read tool: the chart grids are visible, the new widgets sit in their sections without overlap. Report anything that looks wrong; do not adjust sizes by eye.

- [ ] **Step 8: Commit**

```bash
cargo fmt -p native-theme-gpui
git add connectors/native-theme-gpui/examples/showcase-gpui docs/showcase-exceptions.toml
git commit -m "feat(showcase-gpui): the Toolbar and ToolbarGroup, TimeField and an input with an inline token; charts claim their hover tooltip; Questionnaire is excepted until upstream offers an infallible state constructor"
```

---

### Task 9: The shared chrome and the Basic page did not move

**Files:** none committed (the dumps live in `target/layout-070/`).

- [ ] **Step 1: After-dumps.** Run Task 1 Step 1b's snippet with `PHASE=after`.

- [ ] **Step 2: Compare**

```bash
python3 - <<'EOF'
import json, pathlib
base = pathlib.Path("target/layout-070")
def edges(r):
    # R-snap: the parity comparator compares edges rounded to the pixel grid.
    return None if r is None else (round(r["x"]), round(r["y"]), round(r["x"] + r["w"]), round(r["y"] + r["h"]))
diffs = 0
for run in ("rest", "menu"):
    for before in sorted((base / "before" / run).glob("*.json")):
        after = base / "after" / run / before.name
        a = json.loads(before.read_text())["elements"]
        b = json.loads(after.read_text())["elements"]
        for key in sorted(set(a) | set(b)):
            if edges(a.get(key)) != edges(b.get(key)):
                diffs += 1
                print(run, before.name, key, a.get(key), b.get(key))
print("differences:", diffs)
EOF
```
Expected: `differences: 0`. Any difference: stop, find its cause in 0.7.0 (a `chrome.toolbar*` one would point at Task 4's `h_auto`; rationale §1.4(h) lists the upstream changes near the Basic page), and report it with the cause. Only when the maintainer accepts a difference are the three-way comparator (`scripts/check_showcase_parity.py`) and its `--merge --write` regeneration of `[parity]` run, as for any rc1 change.

---

### Task 10: Changelog, todo, compatibility stamp, release check

**Files:**
- Modify: `CHANGELOG.md`, `docs/todo.md`
- Modify (by script only): `docs/COMPATIBILITY.toml`, `connectors/native-theme-gpui/README.md` Verified line

- [ ] **Step 1: CHANGELOG** — spec §10.2's lines under `## [Unreleased]`, in the file's existing sections and style.

- [ ] **Step 2: `docs/todo.md`** — append spec §10.3's entries in the file's format. Append only; never rewrite an existing entry.

- [ ] **Step 3: Commit**

```bash
git add CHANGELOG.md docs/todo.md
git commit -m "docs: changelog and follow-ups for the gpui connector on GPUI Kit 0.7.0"
```

- [ ] **Step 4: Gates**

```bash
CARGO_BUILD_JOBS=4 cargo clippy -p native-theme-gpui --all-targets --all-features --locked -- -D warnings
CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --locked
CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --lib --no-default-features --locked
CARGO_BUILD_JOBS=4 cargo test -p native-theme --test icon_sources --locked
RUSTDOCFLAGS="-D warnings" CARGO_BUILD_JOBS=4 cargo doc -p native-theme-gpui --no-deps --all-features --locked
CARGO_BUILD_JOBS=4 cargo +1.95.0 check -p native-theme-gpui --lib --locked
python3 scripts/check_widget_coverage.py
```
Expected: all clean / green; coverage "Every widget is shown or excepted."

- [ ] **Step 5: Stamp.** `./scripts/update_compatibility.sh run gpui` (needs the network; refuses a dirty tree). Expected: it writes `docs/COMPATIBILITY.toml`'s `[native-theme-gpui]` table with gpui-base / gpui-component / gpui-kit / gpui-kit-assets 0.7.0 and gpui-pre 0.3.7 and rewrites the README's Verified line. Commit exactly those two files:

```bash
git add docs/COMPATIBILITY.toml connectors/native-theme-gpui/README.md
git commit -m "docs(compat): the gpui connector verified against gpui-kit 0.7.0 and gpui-pre 0.3.7"
```

- [ ] **Step 6:** `./scripts/check_release.sh` — passes with two expected warnings while the CHANGELOG says Unreleased (spec §12 item 8): the compatibility stamp and the visual-assets stamp (`docs/assets/PROVENANCE.toml` hashes the connector's `src` and `examples`, which this plan changed). Stop here: screenshots, GIFs and that stamp are the maintainer's `generate_assets_release.sh` run.

---

### Task 11: Archive

- [ ] **Step 1**

```bash
git mv docs/todo_v0.6.0-rc1_gpui-kit-0.7-rationale.md docs/archive/
git mv docs/todo_v0.6.0-rc1_gpui-kit-0.7-spec.md docs/archive/
git mv docs/todo_v0.6.0-rc1_gpui-kit-0.7-plan.md docs/archive/
grep -rn 'todo_v0.6.0-rc1_gpui-kit-0.7' docs/ --include='*.md' | grep -v '^docs/archive/'
```
Expected: the grep prints nothing (fix any link it finds to point into `docs/archive/`). The three documents' links to each other stay valid (they move together).

- [ ] **Step 2: Commit**

```bash
git commit -m "docs: archive the GPUI Kit 0.7.0 rationale, specification and plan, implemented"
```
