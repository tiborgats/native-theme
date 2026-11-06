# v0.7.1 → 0.8.0 — The switch (M1a task 17): the audit and its work list

Read-only audit of `/home/tibi/Rust/native-theme` at `5be7b7fe` (v0.8.0-rc1), 2026-11-06. Inputs: the plan
(*The transition*, :53-176; M1a, :445-469; D9, :237-251), the M1a design (§1, task 17's row :117), the inventory
(drafted at `8483901f`, F1–F16), the ledger (*Module inventory* :70, *`PLAN_EXCLUDED`* :78), `scripts/check_release.sh`,
`.github/workflows/ci.yml`. Nothing was built or run; where a conclusion depends on a run, it says "verify".
Line numbers are at `5be7b7fe`; `lib.rs` has drifted +2 lines since the inventory (the `next` declaration), so the
inventory's lib.rs refs are 2 low (`resolve` :294, `theme` :310-323, `spinners` :300, `reader` :286, `watch` :302,
`prelude` :304, `Result` :359, `AccessibilityPreferences` :417, `ReaderResult` :509, `OverlaySource` :610,
`SystemTheme` :639), and its `check_release.sh` refs are stale (the wrong-safety-nets steps are at :1192-1195, the
mentions at :1129, :1134, :1172-1177).

---

## 1. The inventory against today's tree

### 1.1 Items missing from the inventory, with a proposed fate

| # | Item | Evidence | Proposed fate (plan's rules) |
|---|---|---|---|
| M1 | `tests/facts_gate.rs`, `tests/facts_gate/{facts,model,run,units}.rs`, `tests/fixtures/facts_gate/` (432 files) | facts_gate.rs:22-23, run.rs:10-15, facts.rs:6, model.rs:8 use `native_theme::next::…` | kept; repointed `native_theme::next::` → `native_theme::` at the move; `gate` stays `#[doc(hidden)] pub` |
| M2 | `tests/next_layers.rs`, `tests/fixtures/next/{layers/,timing-rows.txt}` | next_layers.rs:11, :15 | kept; renamed `tests/layers.rs`, `tests/fixtures/layers/` (the `next` name means nothing after the switch); `--quick`'s step (check_release.sh:486) follows |
| M3 | `tests/platform_vectors.rs`, `tests/vectors/*.tsv` (4) | platform_vectors.rs:9-13 | kept; repointed (`native_theme::platform::…`, still `#[doc(hidden)]`) |
| M4 | `native-theme-derive`: `ThemeFields` (lib.rs:296), `gen_inherit/gen_merge/gen_ranges/gen_structs/gen_validate.rs`, `parse.rs`, `util.rs` but `widget_name`, the `ThemeWidget` test module (lib.rs:382), README, Cargo `description` | gen_ranges.rs:22-149 and gen_validate.rs:57-135 name `crate::resolve::validate_helpers`, `crate::color::Rgba`, `crate::model::…`; `ThemeFields`' only users are model/ and resolve/ | deleted with `ThemeWidget` (the plan names only `ThemeWidget`); `util::widget_name` stays (parts/widget.rs:240); `parts/*` repointed (block.rs 13 sites, read.rs 8, widget.rs 5, resolve.rs 4, walk.rs 3, lib.rs:19 doc); README and description rewritten for `ThemeParts` alone |
| M5 | `native-theme/Cargo.toml`: `[lints.rust] unexpected_cfgs` for `nt_next_docs` (:70-73); the licence comment (:5 `src/next/ripple.rs`); dependencies `inventory`, `serde_with`, `async-trait`, `pollster` | `inventory` only in model/ and resolve/; `serde_with` only in model/; `async-trait` in reader.rs, macos.rs, windows.rs, pipeline.rs, kde/, gnome/ (all reader part); `pollster` in lib.rs:471/:960/:1012, detect.rs:1011/:1221 (reader part) | lints entry removed with the cfg; comment path → `src/ripple.rs`; the four dependencies removed (no lint fires on an unused dependency, so nothing else catches them; M1b adds back what its port uses); `serde_with` leaves `[workspace.dependencies]` too (no other member uses it) |
| M6 | F14's premise: `proptest` | used by `next/validate/outcomes_tests.rs`; regressions in the git-ignored `native-theme/proptest-regressions/next/validate/` | F14 resolves to **kept**: the `proptest` dev-dependency and `proptest.toml` stay; the local regressions folder moves to `proptest-regressions/validate/` with the module (ignored by git, so a local `mv`, or its seeds are lost) |
| M7 | `native-theme/NOTICE:1`, `native-theme/README.md:282`, `README.md:126` | name `src/next/ripple.rs` | path → `src/ripple.rs` at the move |
| M8 | lib.rs's crate documentation (:1-173) | four code blocks, two `no_run` doctests (:20, :42, :67) on `SystemTheme`, `Theme::preset` | rewritten at the switch with the README (it is doctested like the README; the inventory names only the README) |
| M9 | lib.rs's internal re-exports (:364-404) | next uses `crate::IconSet`/`crate::IconRole` (tokens.rs:38, toml/model.rs:11-12, blocks/icon.rs:13, validate/walk.rs:34, resolve/evaluate.rs:23); rasterize.rs:6, sficons.rs:11, winicons.rs:21, freedesktop.rs:15 use `crate::IconData` | the icon re-exports stay (`pub` or `pub(crate)` by Q3); every other `pub(crate) use` goes with its item |
| M10 | The destination of `model/icons.rs` and `model/bundled.rs` | inventory: "moved whole out of `model/`", no path; `src/icons.rs` already exists | `icons.rs` → `icons/mod.rs`, `model/icons.rs` → `icons/model.rs`, `model/bundled.rs` → `icons/bundled.rs`; icons.rs:11-16 already re-exports these types, so the loaders' imports become `super::` |
| M11 | `pub mod kde` (lib.rs:270-271) after the split | its one `pub` item, `from_kde_content_pure` (kde/mod.rs:48), goes with the reader part; the rest is `pub(crate)` | `pub(crate) mod kde` until M1b (0.7.0 made the platform modules private because an empty public module shows as an empty docs.rs page) |
| M12 | `next/presets/README.md` | does not exist (F15 assumes it) | written before the switch (17b), then replaces `src/presets/README.md` |
| M13 | `scripts/import_platform_facts.py`, `scripts/convert_inheritance_rules.py` | they read the live files: import :82 `FACTS_MD = docs/platform-facts.md` ("the file there is the file in the tree", :19); convert :91 `RULES = docs/inheritance-rules.toml`, :94 `TODAY_PRESETS` | both inputs vanish or change at the switch, so `--check` then fails (neither is gated, plan :597-598). Q7 |
| M14 | `check_release.sh` steps the plan's script table puts in the gates but the full run lacks | `check_function_vectors.py` runs only in `--quick` (:499); `check_facts_trace.py --enabled` runs nowhere (plan :599 says `--enabled`); `generate_platform_facts.py --check` joins "from the switch" (plan :463, :604) | the Data documents section gains `check_function_vectors.py` and its `--self-test`, `check_facts_trace.py --enabled` and its `--self-test`, and (at 17e) `generate_platform_facts.py --check` with its `--self-test`; verify each passes before it joins |
| M15 | `check_release.sh` steps and texts naming `next` | :46-47, :81-83 (help), :486, :488 (`--doc src/next/`), :489-490 and :1307-1318 (the `nt_next_docs` step), the preset globs `native-theme/src/next/presets/*.toml` at :493 and :1262 | the `nt_next_docs` step goes (both modes); the `--quick` steps are renamed; the globs follow the move (a glob that matches nothing runs the loop once on the literal pattern, which fails) |
| M16 | Scripts with `next` paths | generate_widgets.py:87 (`OUT_DIR`) and :101-108, :547-652 (it **emits** `crate::next::…` imports into `gen_*.rs`); generate_preset.py:225; generate_sf_tracking.py:49; check_facts.py:304; check_preset_citations.py:116; check_facts_trace.py:9 (docstring); check_function_vectors.py:135-139 (already both paths); scripts/README.md:730, :964, :1171, :1199, :1231; tools/motion-vectors/generate.py:24 | repointed (17b makes them path-agnostic, 17d drops the `next` form) |
| M17 | `key!` (next/key.rs:126-131) | `#[cfg_attr(not(nt_next_docs), doc(hidden))] #[macro_export]`, expands to `$crate::next::Key` | the `cfg_attr` goes; `$crate::Key` |
| M18 | Doc examples inside `next` | 9 sites of `native_theme::next::` (key.rs:20, :57; inputs.rs:25; expr/parse.rs:113, :117; …) | repointed at the move |
| M19 | `native-theme-build`'s other uses | README.md:85 (`# use native_theme::theme::IconSet;`, a doctest through lib.rs:169); the string assertions on generated text (codegen.rs:480-1221; lib.rs:2746; tests/integration.rs:99-134, :442-528) | follow the new `IconSet` path (Q3) |
| M20 | `native-theme-build`'s generated `_` arms under S21.2 | codegen.rs:291 emits `_ => None` after every `(role, set)` arm; :253-255 emit `_ => Some(default)` after the `LinuxDesktop` arms | once `IconSet` and `LinuxDesktop` are exhaustive (S21.2), a mapping that covers every pair makes `_` unreachable: `unreachable_patterns` in the user's crate (the compile test denies `unused`, integration.rs:829). Emit each `_` arm only where the arms leave a case uncovered |
| M21 | S21.2 on the kept modules | `#[non_exhaustive]` enums: `IconId` (icons.rs:112), `IconSetChoice` (:1314), `IconRole` (model/icons.rs:42), `IconData` (:297), `IconSet` (:440), `LinuxDesktop` (detect.rs:26), `Error` (error.rs:62), `DiagnosticEntry` (pipeline.rs:173), `VersionMatch` (:363); public structs without it: the five loaders (icons.rs:201, :413, :499, :582, :641), `RgbaData`, `DetectionContext`, `FaceTraits`, `SystemFace` | the enums made exhaustive at the switch (S21.2 lists "every other enum of today's API that stays"; `ThemeChangeEvent` with `watch` at M1b); each struct checked against S1's exceptions and given `#[non_exhaustive]` where it is not one |
| M22 | The CI widget-coverage step's home | ci.yml:358-363 runs it in the iced leg of `test-connectors`, which the switch excludes | the step is not excluded (§2.5), so it moves to a kept job (the `parity` job, which has Python; `cargo metadata` needs only the runner's cargo) |

### 1.2 Stale fates

| # | Inventory row | What the code says | Corrected fate |
|---|---|---|---|
| S1 | `color.rs`: "Stay: `composite_over` (S16)" | today's `composite_over([f32;4], [f32;4])` (color.rs:169) has no caller inside `native-theme`; its callers are connectors (egui contract.rs:1476). S16's kept `color::composite_over` is the `Color` one already in next/color.rs:179 | today's `composite_over` **deleted**; the merged `color` module holds next's. `unpremultiply_alpha` (:329) and `tint_monochrome` (:359), `pub(crate)`, move into the merged `color.rs` (or `icons/`, whose code alone calls them: rasterize.rs:100, sficons.rs:132-133, winicons.rs:174, :525) |
| S2 | `detect.rs` / F2: `detect_icon_theme` "becomes `pub(crate)`" | no caller but the two re-exports (detect.rs:11, lib.rs:374); `pub(crate)` with no caller is dead code under clippy `-D warnings` | `detect_icon_theme` **deleted** (S16 removes it); `system_icon_set` and `system_icon_theme` `pub(crate)` (callers icons.rs:319, :1359, :1389) |
| S3 | `detect.rs`: `DetectionContext::icon_theme` among "Stay" | `pub fn` (detect.rs:1264); S16 removes "`DetectionContext::icon_theme`" from the API | stays, `pub(crate)` |
| S4 | `detect.rs` reader part, as listed | unnamed but reader-only by their callers: `gsettings_get` (:440; callers :790, :1018, gnome/mod.rs:387), `color_scheme_is_dark` (:727), `gnome_reduce_motion` (:958), `session_type` (:681), `reads_force_font_dpi` (:692), `parse_force_font_dpi` (:700), `GNOME_FONT_DPI` (:606), `system_font_dpi` (:1449), `invalidate_is_dark`/`_reduced_motion`/`_accessibility_preferences`/`_font_dpi` | reader part (the rule "a split keeps only what linked code calls" already sends them there; listed so the split is checkable) |
| S5 | `pipeline.rs`: `macos_major` in the reader part | kept `macos_generation` calls it (pipeline.rs:593-595) | **stays** |
| S6 | `pipeline.rs`: `DiagnosticEntry` (`OsGeneration.preset` → `PresetId`), `macos_/windows_generation` "returning `PresetId`" | at the switch `PresetId` holds `Adwaita` and `MacosTahoe` only (next/presets/mod.rs:41-46); the tables name `macos-monterey` (:512) and `windows-11` (:532), which have no `PresetId` until M1b | the tables, `preset_for_version` and `OsGeneration.preset` keep `&'static str` (`PlatformPreset` goes, its `name` returned directly) until M1b, which repoints them to `PresetId` when all eight exist; Q2 |
| S7 | `pipeline.rs`: `GenerationTable` stays | its `keys` field and `row`/`rows` (:467-505) are read only by `pinned_pipeline` (:1228-1231, reader part) and tests | `keys`, `row`, `rows` and their tests go with the reader part (a field never read is dead code) |
| S8 | `model/icons.rs`: "`IconData`, `RgbaData`, `IconProvider` and `icon_name` as they are" | `IconData` is `#[non_exhaustive]` (model/icons.rs:296) | `IconData` exhaustive (S21.2); the others as they are |
| S9 | `error.rs`: "ported beside (S1's errors), then split" | S1's errors are in next/error.rs; today's `Error` is `#[non_exhaustive]` (error.rs:61) | today's remnant (`Error` with three variants, `reader_names::{ICON_THEME, SVG_RASTERIZER}`) merges into next's `error.rs`; `Error` exhaustive (S16, S21.2) |
| S10 | F14 open ("unless the new model's tests use them") | they do (M6) | kept |
| S11 | F10: `IconSet` and `IconRole` to the crate root | not wrong, but it splits the icon types 0.7.0 gathered in `icons` (CHANGELOG 0.7.0: `IconRole`, `IconData`, `IconProvider`, `icon_name` "are in `icons`") and native-theme-build's generated code names `{crate}::icons::IconProvider` (codegen.rs:113, :141) | Q3 |

Rows confirmed against the code (callers re-read): `fonts.rs` (only model use is `crate::theme::FontStyle`, fonts.rs:15; next's `FontStyle` has the same three variants, next/blocks/text.rs:46), `freedesktop.rs` (the spinner loader and freedesktop.rs:20's import), `icons.rs` (`Rgba` in the loaders' `color`/`color_opt`, :283-290, :444-451, :530-537), `kde/mod.rs` (kept: `kdeglobals_path` callers detect.rs:172, pipeline.rs:781; `kdeglobals_icon_theme` icon_theme.rs:321), `macos.rs` (`os_version` :268, `system_ui_font` :405, both already feature-gated), `windows.rs`, the D10 deletions, `gnome/` unlinked whole (F4), `timings.rs` split (F11: `bundled_icons`, `freedesktop_icons`, `freedesktop_xpm_only_name` stay), `resolve_and_validate.rs`'s two SVG tests (:346, :366), the Cargo features, native-theme-build's codegen/schema refs (all still at the inventory's lines), the M2.1 unlinks.

### 1.3 `next/` at the switch

**Name clashes with today's crate root** (each needs today's module gone or merged in the same commit as the move):

| next | today | Resolution |
|---|---|---|
| `pub mod color` | `pub mod color` (lib.rs:246) | merged (S1 above) |
| `mod error` | `pub mod error` (:249) | merged (S9); private, its types re-exported at the root as next's are (the 0.7.0 one-path rule) |
| `mod presets` | `pub mod presets` (:281) + `presets.rs` + `presets/` | today's deleted; `next/presets/` takes `src/presets/` |
| `mod resolve` | `pub mod resolve` (:294) | today's deleted |
| `mod theme` | inline `pub mod theme {…}` (:310-323) | today's deleted |
| `mod toml` | the `toml` crate | no clash: a local module shadows an extern crate only in the scope that declares it (lib.rs, which uses no `toml::`); other modules' `toml::` keep reaching the crate, as they do inside `next` today; the derive names `crate::next::toml::read` explicitly (read.rs:14) |
| `pub mod placement` (a file, S13's data types) | — | none; note the D9 hook pattern `*/src/placement/*` (no-invented-values.sh:46) covers a directory, which M3's strategies create |
| `mod platform` | — | none; the D9 pattern `*/src/platform/*` is already there |
| re-exported names (`Theme`, `ResolvedTheme`, `FontSize`, `FontStyle`, `Color`, `Widget`, the glob `widgets::*`) | `theme::{Theme, ResolvedTheme, FontSize, FontStyle}`, lib.rs:407 | today's deleted; `fonts.rs` repointed to next's `FontStyle` |

**What the move changes.** `git mv native-theme/src/next/* native-theme/src/`, next/mod.rs's body into lib.rs (its four
inline modules `corners`, `shapes`, `transitions`, `timing` and the re-exports); `crate::next::` → `crate::` (738
occurrences in 62 files inside `next`, the derive's 33 sites, generate_widgets.py's emitted imports, then the
`gen_*.rs` regenerated so `--check` holds); `$crate::next::Key`; the 9 doctest paths; the tests (M1-M3); the scripts
(M16).

**`#[doc(hidden)]`.** Removed: lib.rs:278's `cfg_attr` on `next` (the module itself disappears) and key.rs:126's. With
them go the `nt_next_docs` cfg (Cargo.toml:70-73) and its step (check_release.sh:489-490, :1307-1318). Kept hidden:
`gate` and `platform` (next/mod.rs:12, :18: "public for their vector tests", not S16 API), `from_preset_synthetic`
(theme/mod.rs:47, S16 writes `#[doc(hidden)]`) and the two hidden query items (query/theme.rs:111, :116).
Risk to fix, not exclude: `docs, private items (native-theme, Linux features)` (check_release.sh:1299) documents
`next`'s private items for the first time (`--document-private-items` skips hidden modules), so a broken intra-doc
link in a private item's docs, invisible to the `nt_next_docs` run (public items only), fails it.

---

## 2. The expected `PLAN_EXCLUDED` list, by exact step

### 2.1 Mechanism changes the list needs (in 17b, before the switch)

1. **The MSRV workspace step** (check_release.sh:452-454, ci.yml:273) checks every member but gpui and the egui
   crates, so it includes `native-theme-iced` and `native-theme-showcase-support`, which break. Excluding the step by
   name would drop native-theme's `--all-features` MSRV check. Proposal: `msrv_checks` adds `--exclude <crate>` for
   each crate whose `check (<crate>)` key is listed (as the feature step does with its keys, :646-666); ci.yml:273
   gains the same two `--exclude`s, each marked.
2. **`--quick` runs listed steps.** `plan_excluded_step` skips only under `COMMIT_MODE` (:296); `--quick` calls
   `msrv_checks`, so every task would compile the gpui and egui crates at 1.95.0 and print warnings. Proposal: skip
   under `QUICK_MODE` too.
3. **Several marks in one ci.yml `run:`.** A mark is one per line and needs no `#` before it (:535). Where one
   command carries several excluded crates (ci.yml:260's `check_features.sh`, :273's `--exclude`s, :450's
   `cargo doc --workspace`), build the arguments in a bash array, one line per crate, each line ending in its mark:
   `skip+=(--skip native-theme-gpui)  # PLAN_EXCLUDED: feature combinations (native-theme-gpui)`. A commented-out
   matrix entry takes its mark on the whole-line comment above it (a mark after a `#`-commented entry is not read).
4. **Every key needs at least one mark,** also where CI has no counterpart (the check is set equality, :530-545): the
   runner-environment, `docs.rs`, `package` and `check (<crate>)` keys get whole-line marks in the nearest job.
   Choose each mark's key from the same milestone as the CI entry it disables, so that entry returns when its key
   leaves the list (example: `gpui-fast-build` builds `--example showcase-gpui` until M5, so it is marked with an M5
   key, not an M2.1 one).
5. **Keep ci.yml:275's `dtolnay/rust-toolchain@1.95.0` line** (live or commented): `CONNECTOR_TOOLCHAIN` is read
   from it (check_release.sh:449-450), and without it the keys below become `MSRV +, …`.

Keys are the descriptions as printed: `$MSRV_TOOLCHAIN` is `+1.88.0`, `$CONNECTOR_TOOLCHAIN` `+1.95.0`; a toolchain
bump renames four keys.

### 2.2 Until M2.1 (59 steps; check_release.sh line)

- `check (native-theme-gpui)`, `check (native-theme-iced)`, `check (native-theme-egui)`, `check (native-theme-egui-widgets)` — :628
- `MSRV +1.95.0, -D warnings (the egui crates, all features)` — :457
- `MSRV +1.95.0, -D warnings (native-theme-gpui)` — :460
- `MSRV +1.95.0, -D warnings (native-theme-gpui, native, watch)` — :462
- `MSRV +1.95.0, -D warnings (native-theme-gpui, gpui-fast, native, watch)` — :465
- `feature combinations (native-theme-gpui)`, `… (native-theme-iced)`, `… (native-theme-egui)`, `… (native-theme-egui-widgets)` — :664 (the run over the rest becomes `feature combinations (the other crates)`, :662)
- `native-theme-egui, all targets (x86_64-apple-darwin)` — :702
- `clippy (native-theme-gpui, native, watch)` — :752; `clippy (native-theme-iced)`, `clippy (native-theme-egui)`, `clippy (native-theme-egui-widgets)` — :755
- `clippy (native-theme-gpui, gpui-fast, native, watch)` — :766; `clippy (native-theme-gpui, no features)` — :772
- `strict-panic (native-theme-iced, iced_aw, watch, native)` — :838
- `strict-panic (native-theme-gpui, native, watch)` — :847; `strict-panic (native-theme-gpui, gpui-fast, native, watch)` — :849
- `strict-panic (native-theme-egui)` — :856; `strict-panic (native-theme-egui-widgets)` — :864
- `test (native-theme-gpui, native, watch)` — :890; `test (native-theme-iced)`, `test (native-theme-egui)`, `test (native-theme-egui-widgets)` — :898
- `test (native-theme-gpui, no features)` — :927; `test (native-theme-gpui, gpui-fast, native, watch)` — :937 (full run only)
- `test (native-theme-iced, no features)` — :959; `test (native-theme-iced, iced_aw, watch, native)` — :961
- `clippy (native-theme-iced, iced_aw, watch, native)` — :963; `clippy (native-theme-iced, the showcase's required features)` — :968
- `test (native-theme-egui, no features)` — :982; `test (native-theme-egui, all features)` — :984; `clippy (native-theme-egui, all features)` — :986
- `test (native-theme-egui-widgets, no features)` — :992; `test (native-theme-egui-widgets, all features)` — :994; `clippy (native-theme-egui-widgets, all features)` — :996
- `examples (native-theme-gpui)`, `examples (native-theme-iced)`, `examples (native-theme-egui)` — :1095 (Q1: the plan groups them with M5)
- `connector parity: names and signatures (iced · gpui · egui)` — :1141 (it parses model/resolved.rs etc., check_connector_parity.py:139-142)
- `docs, private items (native-theme-gpui)`, `… (native-theme-iced)`, `… (native-theme-egui)`, `… (native-theme-egui-widgets)` — :1303
- `docs (native-theme-iced, all features)` — :1321; `docs (native-theme-egui, all features)` — :1325; `docs (native-theme-egui-widgets, all features)` — :1327
- `docs.rs (native-theme-gpui)`, `docs.rs (native-theme-iced)`, `docs.rs (native-theme-egui)`, `docs.rs (native-theme-egui-widgets)` — :1385 (full only)
- `package (native-theme-iced)` :1425, `package (native-theme-gpui)` :1427, `package (native-theme-egui)` :1429, `package (native-theme-egui-widgets)` :1431 (full only)
- Q4: `semver-checks (native-theme-iced|gpui|egui|egui-widgets)` — :1469 (soft)

### 2.3 Until M2.6 (4, or 3 by Q5)

- `data documents: every registry field has a platform-facts row` — :1188 (fails once 17e generates `platform-facts.md`)
- `data documents: § references name platform-facts sections` — :1196 (reads the deleted `docs/inheritance-rules.toml`, check_section_refs.py:41)
- `data documents: docs/todo.md cites preset values by key path` — :1200 (today's presets gone)
- `data documents: preset comments cite their platform-facts rows` — :1204 (Q5: repoint instead)

Their four `--self-test` steps run synthetic inputs (check_section_refs.py:90-120, check_todo_citations.py:132-175,
the other two in a temporary folder) and stay unlisted.

### 2.4 Until M5 (31 steps)

- `check (native-theme-showcase-support)` — :628 (tools/showcase-support uses `Rgba`, `theme::*`, `AccessibilityPreferences`, `ResolutionContext`: info.rs:8-9, rows.rs:6-8, status.rs:8-9, values.rs:16-17)
- `feature combinations (native-theme-showcase-support)` — :664
- `clippy, showcase-support (x86_64-pc-windows-msvc)`, `clippy, showcase-support (x86_64-apple-darwin)` — :728
- `clippy, iced showcase (x86_64-pc-windows-msvc)`, `clippy, iced showcase (x86_64-apple-darwin)` — :733
- `clippy (native-theme-showcase-support)` — :755
- `strict-panic (native-theme-showcase-support, x86_64-apple-darwin)`, `… (…, x86_64-pc-windows-msvc)` — :819
- `strict-panic (showcase-iced, iced_aw, x86_64-apple-darwin)`, `… (…, x86_64-pc-windows-msvc)` — :824
- `strict-panic (showcase-egui, all features, x86_64-apple-darwin)`, `… (…, x86_64-pc-windows-msvc)` — :829
- `strict-panic (showcase-iced, iced_aw, watch, native)` :842, `strict-panic (showcase-iced, default features)` :844
- `strict-panic (showcase-gpui, native, watch)` :852, `strict-panic (showcase-gpui, gpui-fast, native, watch)` :854
- `strict-panic (showcase-egui, all features)` :860, `strict-panic (showcase-egui, no features)` :862
- `strict-panic (native-theme-showcase-support)` — :876
- `test (native-theme-showcase-support)` — :898
- `test (egui showcase, runner environment)` :1058, `test (egui showcase, no features, runner environment)` :1060, `test (egui showcase, all features, runner environment)` :1062, `test (iced showcase, runner environment)` :1064, `test (iced showcase, iced_aw, watch, native, runner environment)` :1066, `test (gpui showcase, runner environment)` :1068, `test (gpui showcase, native, watch, runner environment)` :1070, `test (gpui showcase, gpui-fast, native, watch, runner environment)` :1076 (full only)
- `showcase parity: element list and exceptions` — :1163 (its `[parity]` names presets the switch deletes, check_showcase_parity.py:551)
- `docs, private items (native-theme-showcase-support)` — :1303

The cross-target keys exist only where the target is installed; elsewhere the full run warns "names no step".

### 2.5 Named by the plan but expected to pass at the switch (not listed; listing them warns "passes, remove it")

- `widget coverage (gpui · iced_widget · iced_aw · egui · egui_ui · egui_extras)` (:1137) and its self-test: source
  reads and `cargo metadata` only (check_widget_coverage.py:24-35, :165-176), no build. Q1.
- `one skrifa (native-theme-egui)` (:988): `cargo tree`, no build.
- `connector parity: self-test` (:1143), `showcase parity: comparator self-test` (:1165), the four data-document
  self-tests: synthetic inputs.
- `docs.rs (native-theme-showcase-support)`: check_docsrs.py:101-104 skips a `publish = false` crate.
- Removed, not listed: `data documents: [wrong_safety_nets] and its [uniform] marks` and its self-test (:1192-1195),
  `docs and missing docs (native-theme's next module)` (:1317; `--quick` :489).

### 2.6 ci.yml marks (line → key)

| Line(s) | Entry | Key(s) marked |
|---|---|---|
| 37-40 | clippy matrix entries gpui, iced, egui, egui widgets (whole-line marks above the commented entries) | `clippy (native-theme-gpui, native, watch)`, `clippy (native-theme-iced)`, `clippy (native-theme-egui)`, `clippy (native-theme-egui-widgets)`; also the four `check (<connector>)` keys (CI has no `cargo check` leg) |
| 53-58 | gpui leg's clippy steps | `clippy (native-theme-gpui, gpui-fast, native, watch)`, `clippy (native-theme-gpui, no features)` |
| 59-64 | iced leg's clippy steps | `clippy (native-theme-iced, iced_aw, watch, native)`, `clippy (native-theme-iced, the showcase's required features)` |
| 65-70 | egui and egui-widgets all-features clippy | `clippy (native-theme-egui, all features)`, `clippy (native-theme-egui-widgets, all features)` |
| 79 (line in the native-theme leg) | `cargo clippy -p native-theme-showcase-support` | `clippy (native-theme-showcase-support)`; `check (native-theme-showcase-support)` |
| 95 | strict-panic showcase-support | `strict-panic (native-theme-showcase-support)` |
| 108, 109 | strict-panic showcase-support per target | `strict-panic (native-theme-showcase-support, x86_64-apple-darwin)`, `… (…, x86_64-pc-windows-msvc)` |
| 122 (inside the target loop) | clippy showcase-support per target | `clippy, showcase-support (x86_64-apple-darwin)`, `clippy, showcase-support (x86_64-pc-windows-msvc)` |
| 131-133 | strict-panic iced connector | `strict-panic (native-theme-iced, iced_aw, watch, native)` |
| 134-142 | strict-panic iced showcase (two steps) | `strict-panic (showcase-iced, iced_aw, watch, native)`, `strict-panic (showcase-iced, default features)` |
| 148-154 | strict-panic iced showcase, both targets | `strict-panic (showcase-iced, iced_aw, x86_64-apple-darwin)`, `… x86_64-pc-windows-msvc)`, and the two `clippy, iced showcase (<target>)` keys (no CI counterpart) |
| 156-167 | gpui strict-panic (four steps) | `strict-panic (native-theme-gpui, native, watch)`, `strict-panic (showcase-gpui, native, watch)`, `strict-panic (native-theme-gpui, gpui-fast, native, watch)`, `strict-panic (showcase-gpui, gpui-fast, native, watch)` |
| 168-178 | egui strict-panic (three steps) | `strict-panic (native-theme-egui)`, `strict-panic (showcase-egui, all features)`, `strict-panic (showcase-egui, no features)` |
| 184-190 | egui showcase both targets | `strict-panic (showcase-egui, all features, x86_64-apple-darwin)`, `… x86_64-pc-windows-msvc)` |
| 194-198 | egui macOS check | `native-theme-egui, all targets (x86_64-apple-darwin)` |
| 199-201 | egui widgets strict-panic | `strict-panic (native-theme-egui-widgets)` |
| 260 | `check_features.sh` (array of `--skip`) | the five `feature combinations (<crate>)` keys |
| 273 | MSRV `--workspace` (two new `--exclude`s) | `check (native-theme-iced)`, `check (native-theme-showcase-support)` |
| 276, 281, 282, 283 | MSRV connector checks | the four `MSRV +1.95.0, …` keys |
| 296 | `cargo test -p native-theme-showcase-support` | `test (native-theme-showcase-support)` |
| 306-309 | test-connectors matrix (whole job) | `test (native-theme-iced)`, `test (native-theme-gpui, native, watch)`, `test (native-theme-egui)`, `test (native-theme-egui-widgets)`, the three `examples (<crate>)` keys (Q1); and the eight runner-environment keys as whole-line comments that disable nothing (CI tests the showcases in these legs, on the runner itself), so they stay when M2.1 restores the legs |
| 320-352 | its configuration steps | `test (native-theme-iced, no features)`, `test (native-theme-iced, iced_aw, watch, native)`, `test (native-theme-egui, no features)`, `test (native-theme-egui, all features)`, `test (native-theme-egui-widgets, no features)`, `test (native-theme-egui-widgets, all features)`, `test (native-theme-gpui, no features)`, `test (native-theme-gpui, gpui-fast, native, watch)` |
| 358-363 | widget coverage | not excluded: moved to the `parity` job (M22) |
| 368-373 | connector parity | `connector parity: names and signatures (iced · gpui · egui)` |
| 380-394 | gpui-fast-build job | `strict-panic (showcase-gpui, gpui-fast, native, watch)` (an M5 key, rule 4) |
| 410 | showcase parity check-list | `showcase parity: element list and exceptions` |
| 417, 421, 423, 425 | data documents | the four M2.6 keys (or three, Q5) |
| 419-420 | wrong safety nets | deleted |
| 450 | `cargo doc --workspace` (array of `--exclude`s) | `docs, private items (<crate>)` for the four connectors and showcase-support |
| 456 | the per-crate doc loop | drop the four connectors; same keys |
| 461, 464, 465 | all-features docs | `docs (native-theme-iced, all features)`, `docs (native-theme-egui, all features)`, `docs (native-theme-egui-widgets, all features)` |
| doc job (whole-line comments) | no CI counterpart (publish.yml's) | the four `docs.rs (<connector>)` and four `package (<connector>)` keys (and the semver keys by Q4) |

Other workflows: `dependency-canary.yml` and `docs.yml` run on `main` only; `publish.yml` on a tag; `screenshots.yml`
on dispatch (must not be dispatched on v0.8.0-rc1 before M5); `os-probe.yml` builds only the probe. None needs marks.

### 2.7 Other steps at the switch

| Step | Expected | Why |
|---|---|---|
| README doctests (`test (native-theme)`, :898) | pass once rewritten | every example in native-theme/README.md and lib.rs's crate docs names removed API |
| native-theme-build's drift test (`test (native-theme-build)`) | pass once repointed | schema.rs:236, :258-263 name `theme::IconSet`; schema.rs:293-345 read `pub enum LinuxDesktop {` and `parse_linux_desktop`'s arms from `native-theme/src/detect.rs` as text, so the split must leave both in that file |
| `test (native-theme-build)`'s compile test | pass; M20 needed for user crates | integration.rs:747-830 builds the generated code against `../native-theme` under `deny(unused)` |
| `semver-checks (native-theme)`, `(native-theme-build)` | pass | 0.8.0 against 0.7.0 is a breaking-allowed bump in 0.x; the connectors' fail to build, soft (Q4) |
| `docs.rs (native-theme)`, `(native-theme-build)` | pass, verify | all features on three targets; the unlinked readers are not compiled |
| `package (core: derive · native-theme · build)` | pass, verify | no `include` list; the moved presets stay under `src/` |
| Visual assets, compatibility claims | warnings (not failures) while CHANGELOG says Unreleased | the stamps hash `native-theme/src`; the spinner GIFs go |
| `feature combinations (the other crates)` | pass, risk | each native-theme feature alone after the splits; `portal` and `watch` become inert (no warning for an unused optional dependency); dead code in a split file under one feature is the risk (also for the cross-target `no default features` clippy, :719) |
| `docs, private items (native-theme, Linux features)` | pass, risk | §1.3 |
| `outdated`, `audit` | as today | resolution only |

---

## 3. The switch's work list

**Split it.** One commit cannot be smaller than 17c below, but most of the work need not be in that commit. The cut
points keep every commit's `--commit` run green: today's crate builds until 17c; 17c and 17d cannot swap (the five
module clashes need today's modules gone before `next`'s move); 17e touches no Rust.

**17b — preparation** (today's crate unchanged; the scripts may be done in a worktree without `cargo`):
1. check_release.sh: §2.1's mechanisms 1-2; the Data documents section gains `check_function_vectors.py`,
   `check_facts_trace.py --enabled` and their self-tests (M14), each verified green first.
2. Scripts path-agnostic (M16): generate_widgets.py chooses `crate::next::` or `crate::` and its `OUT_DIR` by whether
   `native-theme/src/next/mod.rs` exists; generate_preset.py, generate_sf_tracking.py and check_facts.py choose their
   one folder the same way, never both (until 17c `src/presets/` holds today's presets, which `check_facts.py` must
   not take for the new model's existing presets, check_facts.py:278); check_preset_citations.py reads today's
   folder by its citation rule and the new model's by the definition rule, wherever each lies, an absent folder read as empty. Gate: every `--check`
   byte-equal.
3. F13: `assemble_theme_switching_gif` into its own script; generate_gifs_theme_switching.sh:52, :67, :82 call it.
4. F15: write `next/presets/README.md` (M12).
5. Q7: import_platform_facts.py and convert_inheritance_rules.py read their inputs at `fc6109d1`.
6. Optional: `icons/` directory (M10), today's `theme` and `model` paths re-exporting.

**17c — delete, split, unlink** (`next` stays at `crate::next`, still hidden):
1. The inventory's deletions, splits and unlinks, with §1.2's corrections; D10's deletions (`gnome/accent.rs`,
   `kde/qt_color_vectors.rs`, kde/colors.rs's ports, `generate_qt_color_vectors.py` with scripts/README.md:179-194 and
   docs/todo.md's mention).
2. Kept code repointed to `crate::next::{Color, FontStyle}`; S21.2 on the kept enums and structs (M21); `pub(crate) mod
   kde` (M11).
3. native-theme-derive: `ThemeWidget`, `ThemeFields` and their generators (M4); native-theme-build: the `IconSet`
   path (Q3), its tests' expected strings and README:85 (M19), the `_` arms (M20).
4. Tests: deleted per the inventory; `timings.rs` split; the two SVG tests into `icon_sources.rs` or a new
   `tests/icons.rs`; `platform_facts_xref.rs` deleted.
5. Cargo: the four dependencies (M5).
6. The README's and lib.rs's crate-doc examples that no longer compile removed (the new text comes in 17d, against the
   final paths); the spinner GIFs (README.md:92-98, native-theme/README.md:186-187,
   `native-theme/docs/assets/spinner-*.gif`), `generate_gifs_spinners.py` with update_provenance.sh:69,
   generate_assets_release.sh:62-64 and :127-131, generate_assets_local.sh:12-13.
7. `docs/inheritance-rules.toml`, `check_wrong_safety_nets.py`, their steps (check_release.sh:1192-1195, ci.yml:419-420)
   and the mentions (:1129, :1134, :1172-1177; CONTRIBUTING.md:77, :219).
8. `PLAN_EXCLUDED` filled with §2.2-2.4; ci.yml marked per §2.6; widget coverage moved (M22); the ledger's table.
   Gate: `--commit` green; "PLAN_EXCLUDED and ci.yml's marks agree".

**17d — move into place:** §1.3's move; `#[doc(hidden)]` and `nt_next_docs` removed; color.rs and error.rs merged;
tests renamed (M2); scripts drop the `next` form; ripple paths (M5, M7); README, lib.rs's crate docs and
native-theme-build's README written against the final API: native-theme/README.md's Quick start (:31-39),
*Load a bundled preset* (:95-102), *Layer user overrides* (:109-122) as `Layer::from_toml` + `add_layer`, *Apply an
overlay to the OS theme* (:131-144) and *Cheap dark-mode check* (:148-158) removed (S16), *Watch* (:169-180) removed
until M1b, *Icon sets* (:196-209) kept with its path, the spinner recipe (:211-225) removed; its prose (:5-10, :44-91,
:240-262: two presets, no reader yet, the reader features inert); root README.md :10-14, :69-76. Gate: `--commit`
green, `generate_widgets.py --check`, `generate_preset.py <id> --check`.

**17e — the documents** (no Rust): `@fc6109d1` on every surviving `platform-facts.md:N` citation (151 in 31 files at
`5be7b7fe` once 17c's deletions are subtracted: native-theme's reader files, connectors, tools/os-probe/src/windows.rs,
`docs/facts/staged/inheritance-bases.toml` (42, its converter emitting the `@` form), the ledger (2), CHANGELOG.md:290;
exceptions `docs/archive/`, `docs/evidence/`, the checker scripts); the new check step (fails on
`platform-facts.md:N` without `@`), in check_release.sh and ci.yml's `parity` job; `docs/platform-facts.md` replaced
by `generate_platform_facts.py` output (its default `--out` becomes `docs/platform-facts.md`) and the `--check` step
(M14); CONTRIBUTING.md :217-219, :242; `docs/new-os-version-guide.md` (Q9); scripts/README.md (the removed scripts,
~:648 and :179-194, the moved paths); the CHANGELOG's 0.8.0 entry (Breaking: native-theme's API per S16,
native-theme-derive's derives, native-theme-build's generated paths, the exhaustive enums; Removed: the community
presets, `prelude`, `AnimatedIcon` and the spinner loaders, `Theme::from_toml`/`from_file`; Development: the
removed scripts, the generated `platform-facts.md`, `PLAN_EXCLUDED`). Gate (task 17's): the full `check_release.sh`
green with `PLAN_EXCLUDED` equal to the ledger's list, `generate_platform_facts.py --check` green.

**Outside the commits:**
- **D9 is decided** (plan :237-251: "the recommendations approved, 2026-10-29"; :700), so the M1a line's "if D9
  approves" (:466) holds: `.claude/agents/preset-validator.md` (audits a new-format preset against its facts and
  `generate_preset.py --check`), `.claude/skills/platform-fact/SKILL.md` (looks a key up in
  `docs/facts/<platform>/*.toml`, the generated document as an index) and `.claude/skills/add-preset/SKILL.md` (lane
  facts, `generate_preset.py --write`, the `PresetId` entry, the gate) are rewritten at 17e. `.claude/` is git-ignored
  (.gitignore:23): local edits, no commit. The hooks need nothing: no-invented-values.sh:46 already exempts
  `*/src/platform/*`.
- The memory: MEMORY.md's *Source-of-truth documents* entry and `reference_platform_facts.md` (they name
  `platform-facts.md`, `inheritance-rules.toml`) become stale at 17e.
- The ledger: `PLAN_EXCLUDED` table (step, until, since, reason), the inventory marked applied with §1's corrections,
  the *Full runs* row, the triage rows of any `docs/todo.md` item the switch closes (Q8).

---

## 4. Open questions

**Q1 — The plan's expected list against the plan's own rule.** The rule removes a listed step that passes; the plan's
text lists the widget-coverage step and the whole egui configurations section, and puts every showcase-building step
at M5.
- (a) Follow the text: the full run warns "passes, remove it" at the switch, and the gate "the list equals the
  expected list" contradicts the rule.
- (b) List only the steps that fail (§2.5), and the `examples (<connector>)` steps until M2.1, where M2.1's unlinking
  (`autoexamples = false`) leaves `cargo build --examples` nothing to build (verify that cargo accepts `--examples`
  with no example target).
- Recommended: (b), recorded in the ledger as a correction of the plan's expectation.

**Q2 — The generation tables until M1b** (S6).
- (a) Keep `&'static str` preset names until M1b.
- (b) Remove `OsGeneration` from `diagnose_platform_support` until M1b.
- (c) Add `PresetId` variants with no preset. This breaks *One rule* and `every_bundled_preset_validates`.
- Recommended: (a). The API stays whole, and M1b's repoint is one type change.

**Q3 — `IconSet`'s and `IconRole`'s public path** (F10).
- (a) The crate root (F10).
- (b) `native_theme::icons`, beside `IconData`, `IconProvider` and the loaders, as 0.7.0 put the icon types there.
  `IconSet` moves from `theme`, `IconRole` stays, and the generated code's `{crate}::icons::IconProvider` stays
  consistent.
- Recommended: (b), revising F10. One module for all icon types, and native-theme-build's paths become
  `{crate}::icons::IconSet`.

**Q4 — The connectors' `semver-checks`.** They are soft, so they fail as warnings.
- (a) List them (until M2.1), so the warning names its milestone.
- (b) Leave them unlisted, each warning read and recorded at every full run.
- Recommended: (a).

**Q5 — `check_preset_citations.py`.** It also checks the new presets' S9.4 and S4.5 definition comments
(check_preset_citations.py:95-103, :117-121).
- (a) Exclude it until M2.6, losing that check for M1b–M2.6.
- (b) At 17d, read only `native-theme/src/presets/` (today's presets and their `platform-facts.md:N` comments are
  gone), so it passes and is not listed. M2.6 then rewrites it to read the facts.
- Recommended: (b).

**Q6 — Unused dependencies** (M5).
- (a) Remove `inventory`, `serde_with`, `async-trait` and `pollster` at 17c, and let M1b add back what its port uses.
- (b) Keep them until M1b.
- Recommended: (a). Nothing would catch them later.

**Q7 — The two one-off converters** (M13).
- (a) Read the inputs at the pin (`git show fc6109d1:…`), so `--check` stays reproducible until M4 deletes them.
- (b) Freeze their outputs and mark the scripts unrunnable.
- Recommended: (a).

**Q8 — `docs/todo.md` l.1101** (ledger :519, destination M1a: "`detect_platform` becomes
`PresetId::for_platform`/`for_system`"). `for_system` needs every platform's `PresetId`, so the item cannot close
at the switch.
- Recommended: re-destine it to M1b, together with `PresetId::display_name` (S16), which no M1a code needs.

**Q9 — `docs/new-os-version-guide.md`** describes today's process (hand-edited presets, `documented_sizes.rs`,
`platform-facts.md` rows), which is false after the switch.
- (a) Rewrite it at 17e to the facts process (lane facts, `generate_preset.py`, the gate).
- (b) Replace it with a short pointer to that process, and rewrite it fully at M6.
- Recommended: (a). The project's rule holds docs to the code's quality bar, and the process exists at the switch.

**Q10 — The root README between the switch and M2.1.** Its connector Quick start names connectors that do not build
until M2.1.
- (a) Rewrite native-theme's parts at 17d, and leave the connector paragraphs to M2.1, which rewrites the connectors'
  READMEs.
- (b) Rewrite it all now.
- Recommended: (a). Nothing in it is doctested, and M2.1 owns the connector text.

**Q11 — The sub-task split.** Should task 17 run as 17b–17e (§3), each with its reviewer and `--commit` gate, the full
run only at 17e?
- Recommended: yes. 17c is the one large, atomic commit; 17d is mechanical and checkable by the generators' `--check`
  and the 738-site rewrite.

## Decisions (the orchestrator, 2026-11-06)

Each open question above is decided by the plan's rules, as recommended:

- **Q1 (b).** The expected `PLAN_EXCLUDED` list holds only the steps that fail at the switch (§2.2–2.4), plus the `examples (<connector>)` steps until M2.1. The plan's rule removes a listed step that passes, so the steps of §2.5 are not listed. This corrects the plan's text (*What cannot build meanwhile*), whose list named steps that pass; the gate "the list equals the expected list" is read against this list.
- **Q2 (a).** The generation tables keep `&'static str` preset names until M1b. `PresetId` holds only the presets that exist (*One rule for every model field*), so it cannot name macos-monterey or windows-11 before their presets land.
- **Q3 (b).** `IconSet` and `IconRole` live in `native_theme::icons`, beside `IconData`, `IconProvider` and the loaders. This revises F10: one module holds every icon type, and native-theme-build's generated paths become `{crate}::icons::IconSet`, consistent with its `{crate}::icons::IconProvider`.
- **Q4 (a).** The connectors' `semver-checks` steps are listed until M2.1, so their warnings name their milestone.
- **Q5 (b).** `check_preset_citations.py` stays unlisted: from 17d it reads today's folder by its citation rule while that folder exists, and the new presets by the definition rule. M2.6 rewrites it to read the facts.
- **Q6 (a).** `inventory`, `serde_with`, `async-trait` and `pollster` are removed at 17c. Unlinked code does not compile, so nothing would flag them later; M1b adds back what its ports use.
- **Q7 (a).** `import_platform_facts.py` and `convert_inheritance_rules.py` read their inputs at `fc6109d1`, so their `--check` stays reproducible until M4 deletes them.
- **Q8.** `docs/todo.md` l.1101 (`detect_platform` → `PresetId::for_platform`/`for_system`) moves to M1b, with `PresetId::display_name`: `for_system` needs every platform's `PresetId`.
- **Q9 (a).** `docs/new-os-version-guide.md` is rewritten at 17e for the facts process, which exists at the switch. The whole repository is the product, and a guide describing a removed process is false.
- **Q10 (a).** 17d rewrites the root README's native-theme parts. Its connector paragraphs stay until M2.1, which rewrites the connectors and their READMEs; none of them is doctested.
- **Q11.** Task 17 runs as 17b, 17c, 17d and 17e (§3). Each gets its own `--commit` gate and an independent review; the full `check_release.sh` runs at 17e, as task 17's gate. 17c is the one atomic commit.

§1's missing items and stale fates are part of the inventory from now on. Where §1.2 and the inventory's F1–F16 disagree, §1.2 and Q3 hold.
