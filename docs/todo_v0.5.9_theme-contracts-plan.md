# v0.5.9 — Theme contracts: Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Give the iced connector the same fidelity the gpui connector has, and
replace the visual check with assertions that state what a correct mapping is.

**Architecture:** iced's palette carries six colours where one slot feeds nine
widget roles, so exact native colour needs a per-widget seam: correct the
palette where a slot has one meaning, and add `styles::*` (plus `styles::aw::*`
behind a feature) for the rest. Then three layers of assertions — a mapping
contract with a coverage tripwire, showcase self-tests, and a contrast
invariant — plus complete widget coverage in both showcases, kept complete by a
script.

**Tech Stack:** Rust 2024, `iced_core` / `iced_widget` 0.14, optional
`iced_aw` 0.14, gpui-pre 0.3.5 test-support, `cargo metadata`.

**Spec:** [`todo_v0.5.9_theme-contracts-spec.md`](todo_v0.5.9_theme-contracts-spec.md) — read it first. Reasons: [`todo_v0.5.9_theme-contracts-rationale.md`](todo_v0.5.9_theme-contracts-rationale.md) (C1–C12).

**Sibling work, already implemented on this branch:** the gpui connector's move
to GPUI Kit 0.6.4 and the fixes from the maintainer's visual check
([`todo_v0.5.9_gpui-kit-0.6.4-plan.md`](todo_v0.5.9_gpui-kit-0.6.4-plan.md),
E1–E21). This plan assumes that state and does not repeat it.

## Global Constraints

- No `unwrap` / `expect` / `panic!` / indexing / `unsafe` in non-test code. The
  repository PreToolUse hook enforces it on every Write/Edit under `src/`,
  **including test modules**, and it recognises only `#[test]`,
  `#[cfg(test)]` and `#[allow(clippy::unwrap_used` as test markers —
  `#[gpui::test]` alone is refused. An edit adding `.expect(` under `src/` must
  carry one of those markers in the same edited region. Files under `tests/`
  and `examples/` are exempt by path.
- No invented values. Every expected value in a test comes from the resolved
  theme or from the value under test. A contract row that cannot be satisfied
  is a finding: stop and report it, do not delete the row.
- No hardcoded theme values in the showcases: every colour, size and radius
  comes from the resolved theme or a connector builder.
- Every command runs from the **repository root**; no step changes directory.
- All cargo commands: `CARGO_BUILD_JOBS=4`.
- Verify lints by **exit code**, not by reading the last output line:
  `cargo clippy … >/dev/null 2>&1; echo $?`.
- No `Co-Authored-By` or other AI attribution in commits.
- Never tag, push a tag, publish or create a release. Task 12 stops before it.
- Work continues on branch `v0.5.9-gpui-kit-0.6.4`.
- If a step's expected output does not appear, stop and report.

## File map

| File | Responsibility |
|---|---|
| `connectors/native-theme-iced/Cargo.toml` | `iced_widget`, the icon features, the optional `iced_aw` feature |
| `connectors/native-theme-iced/src/extended.rs` | the palette corrections (spec §2) |
| `connectors/native-theme-iced/src/styles.rs` (new) | per-widget style functions (spec §3) |
| `connectors/native-theme-iced/src/styles/aw.rs` (new) | `iced_aw` style functions (spec §3a) |
| `connectors/native-theme-iced/src/lib.rs` | accessibility preferences, re-exports, crate docs |
| `connectors/native-theme-iced/src/contract.rs` (new) | mapping contract + coverage tripwire + contrast (spec §5, §7) |
| `connectors/native-theme-gpui/src/contract.rs` (new) | the same for gpui |
| both `examples/showcase-*.rs` | complete widget coverage, `styles::*` throughout, self-tests |
| both `Cargo.toml` `[[example]]` | `test = true` |
| `scripts/check-widget-coverage.py` (new), `docs/showcase-exceptions.toml` (new) | toolkit widget coverage |
| `pre-release-check.sh`, `.github/workflows/dependency-canary.yml` | run the coverage script |
| `docs/todo.md`, `CHANGELOG.md`, both `README.md` | notes |

---

### Task 1: The iced contract, failing first

Spec §5, §7. Write the gate before the fix, so every later task has a target.

**Files:** Create `connectors/native-theme-iced/src/contract.rs`; modify `src/lib.rs` (add `#[cfg(test)] mod contract;`).

**Interfaces:** Produces nothing public. Consumes `to_theme`, `palette::to_color`, `native_theme::theme::*`.

- [ ] **Step 1: The row type and the table.** One row per palette slot the connector writes, each naming the native field it must equal, per spec §5.1. Start with the four slots `apply_overrides` touches today plus `primary` and `primary.base.text`.
- [ ] **Step 2: Iterate.** For all 16 presets (`Theme::list_presets()`) in both modes, resolve with `ResolutionContext::for_tests()`, build the theme, and assert each row. Presets are sixteen, not the twenty files: the four `*-live.toml` are geometry-only merge bases (spec §5.1).
- [ ] **Step 3: Run it.**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-iced --lib -- contract`
Expected: **failures**, naming `secondary.base.color` (it holds the button surface where iced reads placeholder text) and any other row the connector gets wrong. Record the exact failure list in the commit message; it is the baseline.

- [ ] **Step 4: The contrast invariant** (spec §7), in the same file. For each pair in spec §7's list, composite any colour with alpha below 1 over its own background, compute the ratio from the **native** fields and from the **connector's output**, and assert the connector's is not worse. Print every pair below 4.5 without failing on it.
- [ ] **Step 5: Run it.** Expected: the placeholder pair fails (1.15:1 against a readable native pair). Any other failure is a finding: report it.
- [ ] **Step 6: Commit** (a failing test is committed deliberately here, since Task 2 lands immediately after; note it in the message).

---

### Task 2: The iced palette corrections

Spec §2.

**Files:** Modify `connectors/native-theme-iced/src/extended.rs`.

- [ ] **Step 1: Remove the placeholder poisoning.** Delete `extended.secondary.base.color = to_color(colors.btn_bg);`. iced reads that slot as placeholder text (`text_input.rs:1769`, `text_editor.rs:1476`, `pick_list.rs:910`); leaving iced's generated value restores a readable placeholder. The button surface moves to `styles::button` in Task 3.
- [ ] **Step 2: Write the hover.** Add `extended.secondary.strong.color = to_color(colors.btn_hover_bg);` and add `btn_hover_bg: Rgba` to `OverrideColors`, filled from `resolved.button.hover_background` in `to_theme`. `secondary.strong.color` is read by exactly one place — `button::secondary`'s hover (`button.rs:620`, verified) — so this slot means hover unambiguously.
- [ ] **Step 3: Drop `btn_bg` if nothing reads it.** The compiler decides; remove the field and its initialiser if it is now dead.
- [ ] **Step 4: Run the contract and contrast tests.**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-iced`
Expected: the placeholder contract row and the placeholder contrast pair now pass. Rows that need `styles::*` still fail; they are Task 3's.

- [ ] **Step 5: Commit.**

---

### Task 3: `native_theme_iced::styles`

Spec §3.

**Files:** Modify `Cargo.toml` (add `iced_widget = "0.14"` to `[dependencies]`); create `src/styles.rs`; modify `src/lib.rs` (`pub mod styles;`).

**Interfaces:** Produces `pub fn <widget>(resolved: &ResolvedTheme) -> impl Fn(&Theme, <widget>::Status) -> <widget>::Style + use<>` for each row of spec §3's table.

- [ ] **Step 1: Add the dependency and its feature.** `iced_widget = { version = "0.14", optional = true }`, plus `widgets = ["dep:iced_widget"]` in `[features]` and `widgets` in `default` (spec §4.2). Gate the module with `#[cfg(feature = "widgets")]`. Confirm it unifies: `cargo tree -p native-theme-iced -i iced_core` shows one `iced_core`.
- [ ] **Step 2: Write one function, `button`, completely**, as the pattern for the rest: capture the handful of `Color` values by value so the closure is `'static`; match on `Status` for the hover and pressed colours; construct `button::Style` **exhaustively**, never `..Default::default()` — those structs derive no `Default` and are not `#[non_exhaustive]`, so an upstream field addition then fails the build (C11).
- [ ] **Step 3: A test for it**, asserting each field equals the native value it claims, over all 32 preset/mode combinations. This is the pattern every other function's test follows.
- [ ] **Step 4: The remaining ten functions** of spec §3's table, one at a time, compiling and testing after each: `button_primary`, `text_input`, `checkbox`, `toggler`, `scrollable`, `menu`, `container_card`, `slider`, `progress_bar`, `tooltip`. Each one's doc comment names the iced default it replaces and the slot whose meaning it corrects.
- [ ] **Step 5: Extend the contract table** with a row per `styles::*` output field, so the mapping is stated in one place.
- [ ] **Step 6: Green.**

```bash
CARGO_BUILD_JOBS=4 cargo test -p native-theme-iced
CARGO_BUILD_JOBS=4 cargo clippy -p native-theme-iced --all-targets --all-features --locked -- -D warnings >/dev/null 2>&1; echo "clippy=$?"
```
Expected: all contract rows and the contrast invariant pass; `clippy=0`.

- [ ] **Step 7: Commit.**

---

### Task 4: Accessibility preferences and the icon features (iced)

Spec §4.

**Files:** `connectors/native-theme-iced/src/lib.rs`, `Cargo.toml`, `examples/showcase-iced.rs`.

- [ ] **Step 1: Signatures.** `to_theme` and `from_preset` take `&AccessibilityPreferences`. `from_system` does **not**: it already reads a `SystemTheme`, which carries `accessibility`, and a parameter would let a caller contradict the system it just asked for — it uses `sys.accessibility`.
- [ ] **Step 2: Effects.** `text_scaling_factor` multiplies `font_size()` and `mono_font_size()`; `reduce_transparency` composites any emitted colour with alpha below 1 over its background and emits it opaque; add `pub fn reduce_motion(prefs: &AccessibilityPreferences) -> bool`.
- [ ] **Step 3: Re-export** `AccessibilityPreferences` from the crate root, as the gpui connector does.
- [ ] **Step 4: Features.** Add the `[features]` table of spec §4.2.
- [ ] **Step 5: Update the showcase** for the new signatures.
- [ ] **Step 6: Verify the icons actually load.**

Run: `CARGO_BUILD_JOBS=4 cargo tree -p native-theme-iced -i native-theme | head -3`
Expected: the four icon features enabled. Then a test asserting `native_theme::icons::load_icon` returns `Some` for a known icon, which fails on today's feature-less manifest.

- [ ] **Step 7: Commit.**

---

### Task 5: `iced_aw`

Spec §3a. The six widgets native-theme models and iced core lacks.

**Files:** `Cargo.toml` (optional dependency + `iced_aw` feature), `src/styles/aw.rs`, `src/lib.rs`.

- [ ] **Step 1: The optional dependency.** `iced_aw = { version = "0.14", optional = true, default-features = false, features = ["card", "menu", "context_menu", "tab_bar", "tabs", "sidebar", "spinner", "selection_list"] }` and `iced_aw = ["widgets", "dep:iced_aw"]` in `[features]` — it implies `widgets` because `iced_aw` depends on `iced_widget ^0.14.2`. **Not** in `default`: it drags in `iced_fonts` (3.3 MB of font data) and its iced support has lagged by months (C13).
- [ ] **Step 2: The module**, `#[cfg(feature = "iced_aw")] pub mod aw;` under `styles`. Six functions per spec §3a's table, same pattern as Task 3 — `iced_aw` styles identically (`.style(impl Fn(&Theme, Status) -> Style)`, `Catalog` with `StyleFn`; `src/widget/card.rs:226`, `src/style/card.rs:75-85`).
- [ ] **Step 3: Tests** per function, as in Task 3 Step 3, gated on the feature.
- [ ] **Step 4: Run.**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-iced --features iced_aw`
Expected: all pass. Also run without the feature, to prove the crate still builds lean.

- [ ] **Step 5: Commit.**

---

### Task 6: The gpui contract and coverage tripwire

Spec §5. The gpui mapping is already correct after the sibling work; this states it.

**Files:** Create `connectors/native-theme-gpui/src/contract.rs`; modify `src/lib.rs`.

- [ ] **Step 1: The contract table** over the `ThemeColor` fields with a native counterpart, including the ones this release corrected (`accent` ← `menu.hover_background`, `accent_foreground` ← `menu.hover_text_color`, `sidebar_accent*` ← the sidebar's selection pair, `secondary_hover` ← `button.hover_background`).
- [ ] **Step 2: Run.** Expected: passes immediately. If a row fails, it is a finding in the mapping: stop and report.
- [ ] **Step 3: The coverage tripwire** (spec §5.2): three lists — contracted, `DERIVED` with the derivation named, `UPSTREAM_DEFAULT` with the reason — asserted to partition the 138 `ThemeColor` fields exactly.
- [ ] **Step 4: Prove it discriminates.** Remove one field name from its list; the test must fail naming that field. Restore.
- [ ] **Step 5: Retire the two bespoke `accent` tests** (C10): `accent_is_the_platforms_menu_hover_pair` and `sidebar_accent_is_the_sidebars_selection_pair` are now table rows. Keep `accent_foreground_comes_from_the_highlight_pairs`, which additionally proves highlight text never falls back to the window foreground.
- [ ] **Step 6: The contrast invariant** for gpui, same rule as Task 1 Step 4.
- [ ] **Step 7: Commit.**

---

### Task 7: The widget-coverage script

Spec §6a.4.

**Files:** Create `scripts/check-widget-coverage.py` and `docs/showcase-exceptions.toml`; modify `pre-release-check.sh` and `.github/workflows/dependency-canary.yml`.

- [ ] **Step 1: The script.** `cargo metadata --format-version 1` gives each dependency's `manifest_path`, and therefore its `src` directory — this is the only reliable way to reach a dependency's source, and it is why this is a script and not a test. For gpui-component: enumerate types implementing `RenderOnce` or `IntoElement`, discarding names matching the harness/sub-part rules of spec §6a.2. For iced_widget: enumerate the widget modules. Compare against the showcase source.
- [ ] **Step 2: The exception file**, `docs/showcase-exceptions.toml`: every widget not shown, with a reason — a test harness, an internal sub-part, a layout wrapper with no visual surface, or (for `iced_aw`) no native counterpart.
- [ ] **Step 3: Run it.** Expected: it lists exactly the widgets Tasks 8 and 9 will add. Record that list; it is those tasks' work order.
- [ ] **Step 4: Wire it in.** `pre-release-check.sh` gains a check; the canary gains a step, so an upstream release that adds a widget is reported the evening it appears.
- [ ] **Step 5: Commit.**

---

### Task 8: Complete the gpui showcase

Spec §6a.1, §6a.2. The largest task; do it in the order the script reports.

**Files:** `connectors/native-theme-gpui/examples/showcase-gpui.rs`.

- [ ] **Step 1: The 19 unused builders.** Every `geometry::*` builder must be referenced at least once. `status_bar`, `title_bar` and `combobox` need the widgets they style — add those first. The five `icon_size_*` and the four layout helpers apply to existing sections.
- [ ] **Step 2: The missing widgets**, one section at a time, compiling after each: `Combobox`, `StatusBar`, `TitleBar`, `Pagination`, `Rating`, `Stepper`, `HoverCard`, `ProgressCircle`, `Marker`, `AlertDialog`, `ShimmerText`, `WindowBorder`, `DescriptionList`, `SidebarToggleButton`, the carousel's `CarouselNext` / `CarouselPrevious`, and the chat components `Bubble`, `Message`, `MessageScroller`, `Attachment`. Constructors are read from the vendored source at implementation time; a widget that cannot be built as described is a finding — report it, do not substitute another.
- [ ] **Step 3: Every colour, size and radius from the theme**, via `.native(cx, geometry::…)` or `cx.theme()`, as the surrounding sections do.
- [ ] **Step 4: The builder-coverage test**, in the connector, using `include_str!("../examples/showcase-gpui.rs")` to assert every `geometry::*` and `variants::*` name appears.
- [ ] **Step 5: Prove it discriminates.** Delete one `geometry::` call; the test must fail. Restore.
- [ ] **Step 6: Run the coverage script.** Expected: gpui clean.
- [ ] **Step 7: Commit.**

---

### Task 9: Complete the iced showcase

Spec §6a.3, and the showcase principle of rationale §2.7.

**Files:** `connectors/native-theme-iced/examples/showcase-iced.rs`, `Cargo.toml` (dev-dependency on `iced_aw` with the feature).

- [ ] **Step 1: `styles::*` on every widget.** No widget is left on the palette default: the showcase is what the README's screenshots claim the connector achieves.
- [ ] **Step 2: The five missing themed modules:** `markdown`, `pane_grid`, `qr_code`, `table`, `canvas`.
- [ ] **Step 3: The six `iced_aw` widgets**, with `styles::aw::*`.
- [ ] **Step 4: Run the coverage script.** Expected: iced clean, with the layout wrappers in the exception file.
- [ ] **Step 5: Commit.**

---

### Task 10: Showcase self-tests

Spec §6.

**Files:** both `Cargo.toml` `[[example]]` entries, both showcase files.

- [ ] **Step 1: `test = true`** on both example targets. Verified mechanism: an example with `test = true` runs its `#[cfg(test)]` tests under a plain `cargo test`, so CI and the canary pick them up with no workflow change.
- [ ] **Step 2: gpui tests** (spec §6.1): `every_tab_lays_out`, `resizable_groups_have_room_to_drag`, `interactive_controls_respond`. The clipboard assertion works: gpui's test platform holds a real in-memory clipboard (`platform/test/platform.rs:671-677`).
- [ ] **Step 3: Prove each discriminates.** Shrink a resizable container below `panels × PANEL_MIN_SIZE`; remove a control's handler. Both must fail. Restore.
- [ ] **Step 4: iced tests** (spec §6.2): `view_builds_for_every_tab`, `every_button_has_a_message` — iced renders a button with no `on_press` as `Status::Disabled` (`button.rs:342`), which is the iced form of the dead control — and `styles_cover_every_widget_shown`.
- [ ] **Step 5: Run both.**

```bash
CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui
CARGO_BUILD_JOBS=4 cargo test -p native-theme-iced --features iced_aw
```
Expected: the example targets appear in the output with their tests passing.

- [ ] **Step 6: Commit.**

---

### Task 11: Documentation

Spec §8.

- [ ] **Step 1: iced README** — a "Styles" section: what the palette gives automatically, what `styles::*` gives exactly, the `iced_aw` feature, and a one-line example.
- [ ] **Step 2: iced crate docs** — the accessibility parameter, the feature list, the two-layer colour story.
- [ ] **Step 3: `docs/todo.md`** — the nine iced audit items move to done; the iced `geometry` gap stays, with the coverage script named as what keeps it visible.
- [ ] **Step 4: `CHANGELOG.md`** — the seven entries of spec §8, under the existing undated `## [Unreleased]` heading. Do not date it: `pre-release-check.sh:593` turns the asset-stamp check hard once it carries a date, and dating is the maintainer's release commit.
- [ ] **Step 5: Commit.**

---

### Task 12: Verification and hand-over (stops before any release action)

- [ ] **Step 1: The full gate.**

```bash
CARGO_BUILD_JOBS=4 cargo test --workspace --all-features
CARGO_BUILD_JOBS=4 cargo clippy --workspace --all-targets --all-features --locked -- -D warnings >/dev/null 2>&1; echo "clippy=$?"
CARGO_BUILD_JOBS=4 cargo test -p native-theme-iced --no-default-features
CARGO_BUILD_JOBS=4 cargo test -p native-theme-iced --features iced_aw
python3 scripts/check-widget-coverage.py
CARGO_BUILD_JOBS=4 ./pre-release-check.sh
cargo audit >/dev/null 2>&1; echo "audit=$?"
```
Expected: all green; `pre-release-check.sh` reports no failures and one warning, the stale asset stamp.

- [ ] **Step 2: Spec §9 acceptance list** — run each item, paste each result into the hand-over message.
- [ ] **Step 3: Report to the maintainer.** Both design document sets are now implemented. Report: test counts per crate, the contract tables' row counts, the contrast baseline (how many pairs sit below AA, and that none degraded), anything Task 8 or 9 could not build, and the remaining maintainer-only steps, which are unchanged from the sibling plan's Task 8 — the visual check, pushing the branch, `./scripts/pre-release.sh`, the release commit, CI, the tag, and archiving **all six** design documents afterwards.

**Do not push the branch, open a PR, tag, or publish.**

---

## Execution notes (model routing under the maintainer's dispatch policy)

Tasks 1–7 and 10 have mechanical gates — a named failing test, a script's
output, a compile — and belong to the `implement` agent. Tasks 8 and 9 are
showcase work whose only gate is "compiles and the coverage script is clean",
but whose content is judgment about how a widget should be demonstrated: the
orchestrator does them inline, as the sibling plan did for its showcase task.
Tasks 11 and 12 are orchestration.
