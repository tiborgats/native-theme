# v0.7.0 — GPUI Kit 0.7.3: Specification

**Status:** design, reviewed 2026-10-13, awaiting the maintainer's approval; nothing
implemented. Reasons: [`todo_v0.7.0_gpui-kit-0.7.3-rationale.md`](todo_v0.7.0_gpui-kit-0.7.3-rationale.md)
(L1–L10). Tasks and gates: [`todo_v0.7.0_gpui-kit-0.7.3-plan.md`](todo_v0.7.0_gpui-kit-0.7.3-plan.md).

Abbreviations: `GC` / `GB` = gpui-component / gpui-base 0.7.3 `src/`; `NT` = `native-theme/`;
`CN` = `connectors/native-theme-gpui/`; `SC` = `CN/examples/showcase-gpui/`; `EG` =
`connectors/native-theme-egui/`; `IC` = `connectors/native-theme-iced/`. Our line numbers are
of `v0.7.0-rc1` on 2026-10-13; locate by content.

Source pins for §7–§9 (the platform-facts survey of 2026-10-12): MUX microsoft-ui-xaml
`8463f45162149de0ec3ad7df752596893fe3e13e`; QT qtbase `d865bf9d4877fa09e1fc02e2ad14fb9fd43ef430`
(cross-checked at `v6.10.0`); KIRI kirigami `8319acc7dd1149664147ec1e22c39ae6143ed752`; KWIN
kwin `v6.7.5` (`src/plugins/`); BRZ breeze `v6.7.5`; GTK `4.24.0`; ADW libadwaita `1.10.0`
(`src/`); AX androidx `1e6776853023ebe4266aca2b308305723e3a2d57` (`compose/`); MW material-web
`cbd34a8921915af94d5ef65c2a69eece41d5b4f3`; MDC material-components-android 1.14.0
`66c334b7946dabf33adfe1a2b7cad6bcaa4ea3ad`. In `docs/platform-facts.md` every abbreviated
citation is written out as a full link at its pin, as the file does.

---

## 0 -- Scope and constraints

### 0.1 Deliverables

**G1:** the gpui connector on GPUI Kit 0.7.3 / gpui-pre 0.3.9 (§1); its citations and prose
true for 0.7.3 (§2); `Diff` in the showcase and claims for the sort button and cell outline
(§3); a `gpui-fast` feature and its gates (§4); documents (§5); the OS probe extended (§6), so
its run overlaps G1.
**G2:** the probe run's values (§6); three new theme values with facts, presets and
receivers in all three connectors and their showcases: `tooltip.show_delay_ms` (§7),
`dialog.entrance` (§8), `list.header_height_px` (§9); documents (§10).

### 0.2 Constraints

- No `unwrap` / `expect` / `panic!` / `unreachable!` / slice indexing / unchecked integer
  arithmetic outside tests; no new `unsafe` anywhere except `tools/os-probe/src/{macos,windows}/ffi.rs`
  (the hook's existing exemption).
- No invented values: a preset value cites its `platform-facts.md` row; a row cites a pinned
  primary source or a probe run; an unsourced value stays unset and the toolkit's default
  applies.
- The new fields are `#[theme(category = "soft_option")]`: never required; a preset that
  states none resolves.
- egui's mapping contract (`EG/mapping.toml`) stays exhaustive over model leaves, in every
  commit; gpui's contract covers `ThemeColor` colours and iced's covers iced's `Style` outputs.
  Neither covers model leaves today (M1-T7-03, scheduled for M5/M6); if M5's fix phase lands
  leaf coverage for iced, G2's iced rows go there. Otherwise their new receivers are held by
  tests of their own and by their crate-docs coverage tables.
- Accessor names follow M1-T7-06 as M5's fix phase lands it (named after the model's fields,
  the same set in every connector, `scripts/check_connector_parity.py`): `tooltip_show_delay`
  and `list_header_height` in all three connectors, `dialog_entrance` in gpui, the egui and
  iced gaps recorded the way the parity script records a value a toolkit cannot receive (G2 adds
  that mechanism if M5 leaves none).
- Showcases follow theme data: each showcase applies each new receiver where it draws the
  element.
- No source file cites these three documents by path; they move to `docs/archive/` last.

---

## G1

## 1 -- Floors, lockfile, MSRV, README

- `CN/Cargo.toml`: gpui-pre `0.3.8` → `0.3.9` (normal and dev), gpui-component `0.7.1` →
  `0.7.3`, gpui-base `0.7.1` → `0.7.3`, dev gpui-kit `0.7.1` → `0.7.3`; the comments naming
  versions follow (the `rust-version` comment's `profiler.rs:473` line is unchanged in 0.3.9;
  the `image` comment names both backends, both resolving `image` 0.25.10).
- Lockfile: `cargo update -p gpui-component -p gpui-base -p gpui-kit -p gpui-pre`
  (measured: every `gpui-pre-*` 0.3.8 → 0.3.9, `gpui-kit-assets` 0.7.3; removed, among others,
  `gpui-pre-perf`, `gpui-pre-util-macros`, `gpui-pre-zlog`, `async-compression`, `bzip2`,
  `compression-codecs`, `compression-core`, `core-graphics2`, `core-video`, `erased-serde`;
  no other workspace crate's direct dependency moves).
- MSRV: `cargo +1.95.0 check -p native-theme-gpui --all-features --locked` passes (measured
  2026-10-12, before §4 adds a feature, so this is the gpui-pre build); `rust-version` stays
  `1.95.0`.
- `CN/README.md` floor table and quick start: 0.7.3 / 0.3.9. Gates:
  `compat::the_readme_states_the_manifest_floors`, `compat::the_quick_start_states_the_manifest_floors`.

## 2 -- Citations and prose

### 2.1 Renumbering — digits only

Every `file:line` / `file:a-b` citation in `CN/` (sources, tests, README, showcase),
and `docs/showcase-exceptions.toml` that names a gpui-component,
gpui-base or gpui-pre file changed between the old and new version: the cited line's old text,
located exactly once in the new file, gives the new number; a range keeps its length if its
inside is unchanged, else it goes to the hand-read set (§2.2). Old and
new sources: the upstream tags `v0.7.1` / `v0.7.3`; gpui-pre from the registry (0.3.8 / 0.3.9).
Gates: the removed and added lines of the commit compare equal with their digits stripped; and
a line-text check: for every renumbered citation, the new line (both ends of a range) holds the
text the old line held. The renumbering script lives in the session's scratchpad; its command
goes in the commit body. `docs/todo_gpui-full-theme.md` is not renumbered: its citations are
for gpui-component 0.6.4 (its gap analysis's version), and it only gains an appended 0.7.3 note
(§5).

### 2.2 Read by hand, rewritten — its own commit

- The 16 repeated-text citations, the 4 paths in both crates, the 26 suffix-only matches.
- The tooltip panel's `SHOW_DELAY` claim (`SC/info/feedback.rs`, the line citing
  gpui-base `tooltip.rs`): `TooltipDefaults` (`GB/tooltip.rs:58-107`), 500 ms / 300 ms, a
  trigger's `tooltip_show_delay`; GPUI's own `.tooltip()` defaults to 500 ms and takes a
  per-element `tooltip_show_delay`.
- The TextView panel's heading note (`SC/info/typography.rs`, the "heading sizes"
  `not_themeable`): headings are body × 1.8, 4⁄3, 17⁄15, 1, 14⁄15, 13⁄15, semibold; link
  underline and selection text colour as in rationale L7.
- `SC/info/data.rs`: the claims citing `table/state.rs` re-read at 0.7.3.
- Every claim in the panels of TextView, Dialog, AlertDialog, DataTable, TabBar, ContextMenu
  and Tooltip, re-read at 0.7.3.
- `docs/todo.md`, the entry "A sidebar width and a tooltip delay are missing from the model"
  (`:2239`): its statement that a Button's tooltip delay has no receiver is false since 0.7.3
  (`Button::tooltip_show_delay`); a dated note appended (G2 closes the tooltip half).
- The open gpui items tagged `[upstream]` in `docs/todo.md` and the ROADMAP's upstream-PR list,
  each re-checked at 0.7.3: one that 0.7.3 resolved is marked so (a dated note), one it changed
  is re-cited.
- The version-stating prose: `grep -rn '0\.7\.1\|0\.3\.8' connectors/native-theme-gpui docs/showcase-exceptions.toml docs/todo_gpui-full-theme.md`
  (89 lines on 2026-10-13, 71 of them naming 0.7.1), except the README's
  `<!-- compat:begin -->` … `<!-- compat:end -->` "Verified" line, which only
  `scripts/update_compatibility.sh` writes (`compat::the_verified_line_is_the_stamps`): each
  re-verified at 0.7.3 and its version or date updated; a stated count or measurement
  ("ThemeColor has 139 fields", "86 sites outside `theme/`", "a token 0.7.1 paints nowhere")
  is re-measured; a mention recording history stays.

Gate: `showcase::every_colour_claim_is_read_at_the_line_it_cites` and
`showcase::every_prose_citation_still_exists` pass.

## 3 -- The showcase: `Diff` and two claims

- `SC/support.rs`: `pub(crate) const DIFF_SAMPLE: &str`, one Rust file's unified patch with
  added, removed and context lines. Demonstration data: no style setter receives it.
- `SC/app.rs`: `diff_state: Entity<DiffState>`, created beside `editor_state` with
  `DiffState::new(DiffFile::parse(DIFF_SAMPLE).unwrap_or_default(), cx)`
  (`GC/diff/document.rs:397` returns `Result<Vec<DiffFile>, _>`; `GC/diff/state.rs:252`).
- `SC/demo.rs`: `pub(crate) fn diff(ui, cx, id, state, height) -> Stateful<Div>`:
  `Diff::new(state).h(height)` (`Diff` is `Styled`, applied last, `GC/diff/mod.rs:434-438, 724`),
  `.info(ui, id, info::typography::diff(cx.theme()))`, `.debug_selector(…)`.
- `SC/pages/typography.rs`: a "Diff" section after Markdown, `px(240.0)` tall.
- `SC/info/typography.rs`: `pub fn diff(t: &Theme) -> WidgetInfo` — a colour claim per colour
  it paints at rest, each at its line in `GC/diff/mod.rs`: the frame's `background`,
  `foreground` and `border` (:718, :719, :722); the file header's `muted_foreground` and
  `border` (:1116-1117); the hunk separator's `muted` × 0.35 and `muted_foreground` (:933,
  :935); the sign colours `danger` / `success` (:1576, :1578) and the line fills at × 0.12
  (:1681-1683); line numbers `muted_foreground` (:1301); the incoming conflict side `info`; the add-annotation
button's `primary` / `primary_foreground` and `radius`; and, as other panels do, the hover state
(`list_hover`, :1279-1283) and the selection (`selection`), each at its line (the implementer
records the lines it finds; the colour-claim gate checks them). `.config` for `mono_font_family` /
  `mono_font_size`; `.not_themeable` for the 0.12, 0.35 and 0.3 alphas (upstream literals), the
  syntax colours
  (`highlight_theme`) and the code size (`rem × mono_font_size / 16`, an upstream item).
- `SC/info/data.rs`: claims for the sort button (`secondary_foreground`, `tokens.secondary`,
  `tokens.secondary_active`) and the right-clicked cell outline (`table_active_border` at 0.5),
  at their 0.7.3 lines.
- `SC/tests.rs`: the sample parses to exactly one file; the Typography page renders the debug
  selector `typography-diff`.
- Gate: `python3 scripts/check_widget_coverage.py` reports gpui `missing 0`.

## 4 -- The `gpui-fast` feature

### 4.1 Form A (tried first)

- `CN/Cargo.toml`: `gpui-kit = { version = "0.7.3", default-features = false }` as a normal
  dependency; the dev-dependency gpui-kit gains `"test-support"`; the `gpui-pre` dependency
  stays, because doctests are separate crates and need a dependency named `gpui`; the crate
  roots name GPUI through gpui-kit;
  feature `gpui-fast = ["gpui-kit/gpui-fast", "gpui-component/gpui-fast"]` (gpui-kit forwards
  to gpui-component only weakly, `gpui-component?/gpui-fast`, when its `component` feature is
  off).
- Crate roots (`CN/src/lib.rs`, `CN/tests/seams.rs`, `SC/main.rs`):
  `extern crate gpui_kit as gpui;` unconditionally (upstream's documented alias,
  `crates/kit/src/lib.rs:94, 101-103, 117`).
- Accept form A only if all of these pass, after §4.3's test-helper fix: `cargo check
  -p native-theme-gpui` (library alone) and `cargo test --all-targets` on both backends;
  `cargo check -p native-theme-gpui --lib --features gpui-kit/gpui-fast,gpui-kit/component`
  (the selling point: an application that enables the backend on gpui-kit only; it must pass); the
  `#[gpui::test]` macro resolves; `cargo +1.95.0 check --locked` on both; the packaging check
  `check_release.sh` runs (`cargo package -p native-theme-derive -p native-theme -p
  native-theme-gpui --allow-dirty --target-dir target/package-verify`, which verifies against
  the workspace's native-theme; the crate alone would be verified against crates.io's 0.6.1),
  with and without `--features gpui-fast`. (docs.rs already builds gpui-kit 0.7.3, so the
  platform layer form A adds is buildable there; `check_release.sh`'s docs.rs emulation runs as
  always.) The accepted dependency addition is
  gpui-kit with `gpui-pre-platform` and its platform crates (`cargo tree -p native-theme-gpui
  -e normal` before and after, recorded in the commit body). Otherwise form B.

### 4.2 Form B (measured)

- `[features] gpui-fast = ["dep:gpui-fast", "gpui-component/gpui-fast", "gpui-base/gpui-fast", "gpui-kit/gpui-fast", "gpui-kit/test-support"]`
  (the last two name the dev-dependency gpui-kit; the trial ran `cargo package --no-verify`,
  which accepted the manifest; §4.1's packaging check is the gate).
- `[dependencies] gpui-fast = { package = "gpui-fast", version = "0.1.7", optional = true }`.
- Crate roots: `#[cfg(feature = "gpui-fast")] extern crate gpui_fast as gpui;`.

### 4.3 Common to both forms

- **Test draw helpers** (`SC/tests.rs` `draw`, `CN/src/widgets/tests.rs` `redraw`):
  `window.refresh()` before `window.draw(cx)`, with the reason (rationale §1.2) in a comment,
  repeated in both test modules' headers for future helpers.
- **docs.rs:** `[package.metadata.docs.rs] features = [...]`, every feature except
  `gpui-fast`, replacing `all-features = true`. Gate: a test in `CN/src/compat.rs` that the
  list equals the manifest's features minus `gpui-fast`.
- **Compatibility:** both forms: `scripts/update_compatibility.sh` adds `gpui-fast` and the
  other `gpui-fast-*` crates to `family_of gpui` (:58), and `dependency-canary.yml`'s
  `for c in gpui-pre …` list (:68) gains them, since the backend crates are in the build either
  way. Form B:
  `CN/src/compat.rs` `REQUIRED` gains `"gpui-fast"`, the README's Compatibility table a
  gpui-fast 0.1.7 row. Form A: `REQUIRED` already holds `gpui-kit`; its comment
  ("`gpui-kit` is a dev-dependency", :36) is corrected; no gpui-fast row (gpui-kit chooses the
  version). Both: `gates_gpui` (:233-237) gains a `--features gpui-fast` test gate.
- **CI (`ci.yml`):** clippy `-D warnings` and `scripts/check_strict_panic.sh` for
  `-p native-theme-gpui --features gpui-fast`; `cargo test -p native-theme-gpui --features gpui-fast --all-targets`
  on Linux (doctests excluded: they are not aliased); the MSRV job checks the default features
  and `--features gpui-fast` separately; a new job on `windows-latest` and one on
  `macos-latest` building `cargo build -p native-theme-gpui --example showcase-gpui --features gpui-fast`
  (linking catches what 0.7.3's #3380 fixed, one Windows manifest; gpui-base's and
  gpui-component's `gpui-fast` dependencies already enable `windows-manifest-provided-by-host`).
  No job on push builds the gpui crate on those platforms today; only the dispatch-only
  `screenshots.yml` builds the gpui showcase there. `dependency-canary.yml` and `publish.yml` gain the Linux lines.
- **Local gates:** `scripts/check_release.sh --commit` runs the gpui-fast clippy and
  `scripts/check_strict_panic.sh -p native-theme-gpui --lib --features gpui-fast`; the full mode
  also its tests; `cargo semver-checks` for the gpui crate runs with
  `--default-features`; `scripts/check_features.sh` excludes `gpui-fast` from the gpui crate's
  all-features run (it already builds every feature alone).
- **A real run on GPUI Fast** (the implementing agent runs it on the maintainer's KDE
  session, as the screenshot scripts do): the showcase built with `--features gpui-fast`,
  started, its theme switched (kde-breeze → adwaita → kde-breeze), a screenshot looked at; then
  `--dump-layout` for kde-breeze and adwaita, light and dark, from both builds, compared
  directly (a JSON diff of every element's position and size, which must be equal: both builds
  use the same layout code; `check_showcase_parity.py` compares different toolkits and cannot
  take two gpui dumps). Any difference fixed or explained in the ledger.
- **Lockfile:** `unicode-properties` 0.1.3 (pinned `=0.1.3` by `gpui-fast-web`; also used by
  `rustybuzz` in core's `svg-rasterize` path) accepted; a
  `docs/todo.md` entry to return to 0.1.4 when upstream unpins.
- **Docs:** `CN/README.md` *Features* (seven features, `gpui-fast` off by default), a
  *Backends* section replacing *GPUI as gpui-pre* (what to enable when an application uses
  GPUI Fast), `CN/src/lib.rs` `# Features`.
- **Upstream issue draft** (for the maintainer to file on longbridge's gpui-fast): in test
  mode a partially reused frame carries every previous element's `debug_bounds` forward
  (`retained.rs:1360-1371`); the five showcase absence tests are the repro (the sixth failure,
  the checkbox test, was our harness's missing `cx.notify()`).

## 5 -- G1 documents

- CHANGELOG `[Unreleased]`: *Changed* — the floors; *Added* — the `gpui-fast` feature, the
  showcase's `Diff`.
- `ROADMAP.md`: the v0.7.0 entry, one sentence; the upstream-PR milestone's list gains
  `Diff`'s code size from `mono_font_size` directly, a gpui-fast marker constant (a readable
  backend-mismatch error), and a TextView rule colour separate from `border`.
- `docs/todo_gpui-full-theme.md`: a "gpui-component 0.7.3" note and the Diff row, appended.
- `docs/todo.md` (read first, append or mark): the entry "Markdown headings are sized from a
  fixed 14px" closed (fixed upstream; headings keep TextView's ratios, rationale L7); the
  `unicode-properties` entry; the gpui-fast issue draft.
- `docs/COMPATIBILITY.toml` re-stamped by `scripts/update_compatibility.sh run gpui`.

---

## 6 -- The OS probe, second run (code in G1, values used in G2)

`tools/os-probe` gains, in its output format: on macOS, the header height of a new
`NSTableView` with a header, for its default style (the first run records `automatic`,
effective `inset`) and for `.fullWidth`, per appearance; on Windows, `SPI_GETMOUSEHOVERTIME`
(in `src/windows/ffi.rs`). G1's first task pushes and dispatches `os-probe.yml` (approved by the
maintainer, 2026-10-13); the outputs go to `docs/evidence/os-probe/<date>-run-<id>/` with a README, as the first run's.
The presets take the `.fullWidth` height (a document table); `macos-monterey` takes it only if
macOS 14 and 15 agree; a zero or missing height leaves the preset unset. If the run has not
landed when G2's presets are written, `windows-11`'s delay and the macOS header heights stay
unset and are added when it lands; the run does not block G2.

## G2

**Facts rows shift line numbers.** Inserting rows into `docs/platform-facts.md` §2.7, §2.15
and §2.22 moves every later line, and about 800 `platform-facts.md:N` citations point past the
first insertion (preset TOML comments, `NT/src/presets.rs`, `NT/src/kde/metrics.rs`, the gpui
`contract.rs`, …); `NT/src/presets/documented_sizes.rs` asserts the text at the lines it cites.
So the facts rows and the renumbering land in **one** commit: every `platform-facts.md:N`
citation past an insertion point shifts by the rows inserted before it, and so do
`documented_sizes.rs`'s bare line numbers (about 280) and the `line(n)` checks in
`NT/tests/platform_facts_xref.rs`. Excluded: the finding ledger and `docs/archive/` (append-only
history; their citations are dated). Gates: the digits-only check (§2.1) over every file except
`platform-facts.md` itself (its new rows have no removed partner), the line-text check for every
shifted citation, and `cargo test -p native-theme`.

## 7 -- Tooltip show delay

### 7.1 Model

`show_delay: Option<u16>` on the tooltip struct (`NT/src/model/widgets/mod.rs`), serialised as
`show_delay_ms` (`#[serde(rename = "show_delay_ms")]`, as the size fields carry `_px`),
`soft_option`, milliseconds, no inheritance. `docs/property-registry.toml`:
`show_delay_ms = "u16"` under `[tooltip]`, and its scalar-type note names integer
milliseconds. `docs/inheritance-rules.toml` `[no_inheritance]`: the field with its reason
("stays None where the platform states none").

### 7.2 Facts — a row in `docs/platform-facts.md` §2.7 Tooltip

| Property | macOS | Windows | KDE | GNOME | Material |
|---|---|---|---|---|---|
| `show_delay` | ❓ — undocumented (`NSView.toolTip`, HIG *Offering help*) · macOS 26: ❓ — the same | 2 × `SPI_GETMOUSEHOVERTIME` (the §6 run's value) **(impl)** MUX `dxaml/xcp/dxaml/lib/ToolTipService_Partial.h:17-19`, `ToolTipService_Partial.cpp:1731-1783` | 700 **(impl)** QT `src/gui/kernel/qstylehints_p.h:45` (`m_toolTipWakeUpDelay{700}`), returned for `SH_ToolTip_WakeUpDelay` by `src/widgets/styles/qcommonstyle.cpp:5351-5356`; Qt v6.10.0 `qcommonstyle.cpp:5375-5380` states 700 directly; KIRI `src/platform/units.cpp:45` | 500 **(impl)** GTK `gtk/gtktooltip.c:71-73` (`HOVER_TIMEOUT`) | 0 — shown on pointer enter **(impl)** AX `material3/material3/src/commonMain/kotlin/androidx/compose/material3/internal/BasicTooltip.kt:271-276` |

`ios-13` has no facts column; its preset leaves the field unset with a comment
(`UIToolTipInteraction`, iOS 15+, states no delay).

### 7.3 Presets

`windows-11`: 2 × the §6 run's `SPI_GETMOUSEHOVERTIME` (800 if it reports 400); `kde-breeze`:
700; `adwaita`: 500; `material-3`: 0; every other preset unset. The presets README's
*What each reader supplies* is unchanged (no reader sets it).

### 7.4 Receivers

- **gpui** (`CN/src/lib.rs`, the function that applies the theme to the `App`, beside the
  base-layer install; the value is the variant being applied, `nt.variant(is_dark)`): keep
  `last_written: Option<Duration>` (what the connector last installed) and `app_delay:
  Duration` (initially `TooltipDefaults::global(cx).show_delay()`, `GB/tooltip.rs:58-107`, a
  `Copy` value). On every apply: read the global; if its show delay differs from
  `last_written`, it is the application's, so `app_delay` takes it; install the global with its
  show delay set to the theme's value, or to `app_delay` when the theme states none, and record
  that as `last_written`. The grace period is never touched. This follows
  `ReduceMotionRequest`, which records whether the connector itself made a change. And
  `pub fn tooltip_show_delay(resolved: &ResolvedTheme) -> Option<Duration>` for GPUI's own
  per-element `tooltip_show_delay` (gpui-pre 0.3.9 `src/elements/div.rs:735, 1708`), which the
  showcase's GPUI-native tooltips (the Feedback page's, `SC/demo.rs:5367`; the chrome
  toolbar's) apply when it is `Some`. Rustdoc and README *How re-application works*: the
  install; GPUI's `.tooltip()` defaults to 500 ms unless given the accessor's value. Tests:
  set → installed; unset after set → the application's delay (500 ms when it set none); an
  application that changes the delay between two applies keeps its new value; its grace period
  kept.
- **egui** (`EG/src/style/base.rs`, where the base style is built):
  `style.interaction.tooltip_delay = v / 1000.0` when set (egui 0.36.2 `style.rs:928`), and the
  accessor `tooltip_show_delay(resolved) -> Option<Duration>` (the parity set). In
  `EG/mapping.toml` the leaf gets the row `["tooltip.show_delay_ms"]` with verdict `derived` (ms
  → s) and `sinks = [{ path = "interaction.tooltip_delay" }]` (as `defaults.line_height` has),
  and `EG/src/contract.rs`'s sink table gains its `interaction` arm (it has none); and
  `interaction.tooltip_delay` leaves `[unwritten]`; the shared `[unwritten]` reason "ResolvedTheme
  exposes none of Style::interaction" (:2858) is reworded for the fields that stay
  (`tooltip_grace_time`: no model field). `mapping.md` regenerated; the tests at
  `EG/src/style/base.rs:573`, `EG/src/tripwires.rs:120-129`, the style diff and
  `verdict_totals_match_the_specification` (`EG/src/mapping_tests.rs:438`) updated.
- **All three showcases' leaf readers** strip only `_px` from a TOML key to find the field
  (egui `EG/examples/showcase-egui/info.rs:313-335`, gpui `SC/info/values.rs:61-67`, iced
  `IC/examples/showcase-iced.rs:5244, 5273`): each also strips `_ms`, with a test that
  `tooltip.show_delay_ms` reads its value (it fails before).
- **iced** (`IC/src/lib.rs`): `pub fn tooltip_show_delay(resolved: &ResolvedTheme) -> Option<Duration>`
  for `Tooltip::delay` (iced_widget 0.14.2 `tooltip.rs:126`); the iced showcase's six tooltip
  constructions apply it; the accessor joins the *Widget metrics* row of the coverage table
  (`IC/src/lib.rs:184-196`).
- `docs/showcase-elements.toml`: `tooltip.show_delay_ms` among the leaves of
  `basic.buttons.tooltip.bubble`.

## 8 -- Dialog entrance

### 8.1 Model

`entrance: Option<DialogEntranceStyle>` on the dialog struct, `soft_option`.
`DialogEntranceStyle` lives in `native-theme/src/model/dialog_entrance_style.rs` and follows
`arrow_side.rs` in everything (derives, serde form, no `#[non_exhaustive]`), re-exported from
`native_theme::theme`, with `ScaleFade`, `FadeRise` and, only if §8.2's Material check finds a
source, `SlideDown`. The connectors match it exhaustively. Registry: `entrance = "enum"` with
its variants; `[no_inheritance]` entry.

### 8.2 Facts — a row in §2.22 Dialog

| Property | macOS | Windows | KDE | GNOME | Material |
|---|---|---|---|---|---|
| `entrance` | ❓ — `NSWindow.AnimationBehavior` names styles only; HIG *Sheets* states no motion · macOS 26: ❓ — the same | `scale_fade` (`ContentDialog`: scale 1.05 → 1, 250 ms, with an 83 ms fade) **(impl)** MUX `controls/dev/CommonStyles/ContentDialog_themeresources.xaml:96-114` | `fade_rise` — KDE's in-window dialog, `Kirigami.Dialog`, fades in while rising 2 grid units into place (`y: … + Platform.Units.gridUnit * 2 * (1 - opacity)`, KIRI `src/controls/Dialog.qml:303`, the fade at :306-311). Note: KWin's default-on Scale effect (0.8 → 1, 200 ms; KWIN `scale/package/contents/code/main.js:35-80`, `metadata.json:93`) animates toplevel windows such as a `QDialog`, not an in-window dialog, and Qt's `qdialog.cpp` animates nothing **(impl)** | `scale_fade` (floating `AdwDialog`: fade with scale 0.8 → 1) **(impl)** ADW `adw-floating-sheet.c:22, 34-35, 220-226` | the G2 facts task's result: material-components-android's dialog animation, or Compose's (none: `AlertDialog.kt` defines none, the platform window animation applies); material-web's slide (MW `dialog/internal/animations.ts:52-104`) recorded as a note, not as the value |

### 8.3 Presets

`windows-11`, `adwaita`: `scale_fade`; `kde-breeze`: `fade_rise`; `material-3`: the §8.2 result or
unset; the rest unset.

### 8.4 Receivers

- **gpui** (`CN/src/geometry.rs`, beside `dialog`):
  `pub fn dialog_entrance(n: Native<'_>) -> Option<gpui_component::dialog::DialogEntrance>` —
  `ScaleFade` → `Fade` (gpui has no scale entrance), `FadeRise` → `Fade` (gpui has no rising
  entrance; its `FadeSlide` settles from above), `SlideDown` → `SlideDown`, unset → `None` (the
  caller keeps upstream's default); its rustdoc states both approximations. The showcase applies it to every
  `Dialog` and `AlertDialog` it opens. A test per variant and for unset.
- **egui:** an `unmappable` row, `sub_tag = "egui-limited"`, with an `upstream` text (egui has no
  dialog animation). **iced:** a sentence under the coverage table (`IC/src/lib.rs:184-196`)
  naming the theme values iced has no receiver for, this one among them, and a dated line
  appended to the `docs/todo.md` entry "The iced connector maps no `DialogTheme` field".

## 9 -- List header height

### 9.1 Model

`header_height: Option<f32>` on the list struct (TOML `header_height_px`), logical pixels,
`soft_option`; validation `> 0`. Registry `header_height_px = "f32"`; `[no_inheritance]`
entry. `native-theme/src/presets/documented_sizes.rs` (which gates the list's `row_height`)
gains `list.header_height` in its first table, beside `row_height` (so
`unsourced_presets_state_no_gated_size` covers it), one row per `NativePreset` citing the §2.15
line: adwaita 30, material-3 56, macos-monterey / macos-tahoe the §6 values, kde-breeze and
windows-11 `None`; it fails before Task 10's preset values and passes after. Its module doc
(`documented_sizes.rs:6`) and the presets README's *Gated sizes* (`presets/README.md:25`) name
`list.header_height`.

### 9.2 Facts — a row in §2.15 List / Table

| Property | macOS | Windows | KDE | GNOME | Material |
|---|---|---|---|---|---|
| `header_height` | the §6 run's `.fullWidth` `NSTableHeaderView` height on macOS 14 and 15 (they must agree) **(measured)** · macOS 26: the run's macOS 26 value **(measured)** | **(none)** — WinUI 3 has no column-header control (MUX tree) | **(none)** — content-sized: header font height + 2 × `Header_MarginWidth` 6 (BRZ `kstyle/breezestyle.cpp:3856-3899`, `breezemetrics.h:151-153`); unset, as `row_height_px` is | 30 **(Adwaita CSS)** — `min-height: 24px` + 3 + 3 (ADW `stylesheet/widgets/_column-view.scss:12-35`, `_buttons.scss:18-22`) | 56 **(M3 token)** `md.comp.data-table.header.container.height` (MW `tokens/versions/v0_192/_md-comp-data-table.scss:50`) |

### 9.3 Presets

`adwaita` 30; `material-3` 56; `macos-monterey` the §6 run's macOS 14/15 value,
`macos-tahoe` its macOS 26 value; the rest unset.

### 9.4 Receivers

- **gpui** (`CN/src/geometry.rs`, beside `data_table_size`):
  `pub fn list_header_height(n: Native<'_>) -> Option<Pixels>` (named per M1-T7-06) for
  `DataTable::header_row_height`; the showcase's DataTable (`SC/demo.rs`, the Data page) and the
  Basic page's own table header (`demo::files_table`'s header row) apply it, falling back to
  their row height as today. Tests set and unset.
- **egui:** `pub fn list_header_height(resolved: &ResolvedTheme) -> Option<f32>` (the
  accessor module) for `TableBuilder::header(height, …)` (egui_extras 0.36.2 `table.rs:452`);
  a `derived` row with `tested_by` (as `dialog.button_order`, `EG/mapping.toml:468-470`); the egui
  showcase's tables (`data.rs:48` and the Basic page) apply it, falling
  back to `list.row_height` as today.
- **iced:** `pub fn list_header_height(resolved: &ResolvedTheme) -> Option<f32>`; the iced
  showcase's table header cells apply it, with the same fallback; the accessor joins the
  coverage table's *Widget metrics* row.
- `docs/showcase-elements.toml`: `basic.table.header` adds `list.header_height_px` beside
  `list.row_height_px` (`:1858`; the header falls back to the row height, so it reads both).

## 10 -- G2 documents

- CHANGELOG `[Unreleased]`: *Breaking Changes* — the three widget structs gain fields (they are
  not `#[non_exhaustive]`); *Added* — the fields, their preset values, the receivers
  (gpui's `TooltipDefaults` write and `dialog_entrance`; `tooltip_show_delay` and
  `list_header_height` in all three connectors).
- READMEs: `CN/README.md` *What gets mapped*, *Per-widget geometry* and *How re-application
  works* (the `TooltipDefaults` install); the egui and iced READMEs' accessor lists; the
  presets README's field notes if it lists widget fields.
- `docs/todo_code-review-plan.md`, M2d's step: one line, "`list.header_height_px` 56 and
  `tooltip.show_delay_ms` 0 in `material-3` come from material-web / Compose at the AX pin;
  re-check them against the Compose release M2d pins".
- `ROADMAP.md`, upstream-PR list: a `Scale` dialog entrance; GPUI's `.tooltip()` honouring
  `TooltipDefaults`; separating gpui-base's tooltip hide delay from a reshow window; the
  existing bullet "`Theme.shadow` honoured beyond `Button`" amended to name the dialog's
  literal shadow.
- `docs/todo.md`: the entry "A sidebar width and a tooltip delay are missing from the model"
  — its tooltip half marked done; a live Windows hover-time read as a possible later item (it needs a `SAFETY:` comment, which
  `unsafe_safety_comments.rs` enforces, and a watched `WM_SETTINGCHANGE` /
  `SPI_SETMOUSEHOVERTIME` source)
  (needs the maintainer's approval for its `unsafe`).
- `docs/todo_gpui-full-theme.md`: the tooltip, dialog and list rows gain the receivers.
- `docs/new-os-version-guide.md`: the three values in each platform's *What to look for*.
- The gpui crate-docs coverage table (`list | 7 of 13` and its siblings) updated.
- `docs/COMPATIBILITY.toml` re-stamped for all three connectors by
  `scripts/update_compatibility.sh run`.

---

## 11 -- Acceptance

The full `./scripts/check_release.sh` may show only these expected items: the stale visual
assets (refreshed by the next asset run); the existing `v0.6.1` tag (until the version bump);
the soft semver warnings for native-theme's and native-theme-build's accepted breaks; cargo
audit's advisory warnings as of the run; after G1 only, the stale compatibility claims of the
iced and egui connectors (M4 changed their sources; G2 re-stamps all three); and no gpui soft
warning (every
`run_check_soft` / `run_tests_soft` line for the gpui crate passes, since those only warn).

- G1: the gpui crate's tests pass on both backends; coverage `missing 0`; both showcase gates
  pass; `cargo +1.95.0 check` on both backends; the GPUI Fast run done; `check_release.sh` as
  above; `cargo semver-checks` for the gpui crate (`--default-features`) shows no break beyond
  those CHANGELOG `[Unreleased]` already lists.
- G2: workspace tests pass; egui's mapping contract exhaustive; `preset-validator` reports every
  new value sourced; `cargo semver-checks` for native-theme reports, beyond the breaks
  CHANGELOG `[Unreleased]` already lists, exactly the three struct-field additions;
  `check_release.sh` as above.
- Screenshots that change, for the maintainer's next `generate_assets_release.sh`: the gpui
  Typography page (`Diff`); the table headers of adwaita, material-3 and the macOS presets in
  all three showcases' Basic pages and gpui's Data page.
