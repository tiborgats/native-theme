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

**Spec:** [`todo_v0.5.9_theme-contracts-spec.md`](todo_v0.5.9_theme-contracts-spec.md) — read it first. Reasons: [`todo_v0.5.9_theme-contracts-rationale.md`](todo_v0.5.9_theme-contracts-rationale.md) (C1–C13).

**Sibling work, already implemented on this branch:** the gpui connector's move
to GPUI Kit 0.6.4 and the fixes from the maintainer's visual check
([`todo_v0.5.9_gpui-kit-0.6.4-plan.md`](todo_v0.5.9_gpui-kit-0.6.4-plan.md),
E1–E21). This plan assumes that state and does not repeat it.

## Global Constraints

- **Every commit is green.** Write the test, watch it fail, implement, watch it
  pass, commit — within one task. Never commit a red test, and never wire a
  gate to a check that does not yet pass: CI, the nightly canary and
  `pre-release-check.sh` all run on this branch's commits.
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
- Three iced configurations must build at every commit touching that connector:
  default, `--no-default-features`, and `--features iced_aw`.
- No `Co-Authored-By` or other AI attribution in commits.
- Never tag, push a tag, publish or create a release. Task 13 stops before it.
- Work continues on branch `v0.5.9-gpui-kit-0.6.4`. The name predates this half
  of the release; it is not renamed mid-flight.
- If a step's expected output does not appear, stop and report.

## File map

| File | Responsibility |
|---|---|
| `connectors/native-theme-iced/Cargo.toml` | the `widgets` and `iced_aw` features, the icon features |
| `connectors/native-theme-iced/src/extended.rs` | the palette corrections (spec §2) |
| `connectors/native-theme-iced/src/styles.rs`, `src/styles/aw.rs` (new) | per-widget style functions (spec §3, §3a) |
| `connectors/native-theme-iced/src/lib.rs` | accessibility preferences, re-exports, crate docs |
| `connectors/native-theme-iced/src/contract.rs` (new) | mapping contract, coverage tripwire, contrast (spec §5, §7) |
| `connectors/native-theme-gpui/src/contract.rs` (new) | the same for gpui |
| both `examples/showcase-*.rs` | complete widget coverage, `styles::*` throughout, self-tests |
| both `Cargo.toml` `[[example]]` | `test = true` |
| `scripts/check-widget-coverage.py`, `docs/showcase-exceptions.toml` (new) | toolkit widget coverage |
| `pre-release-check.sh`, `.github/workflows/dependency-canary.yml` | run the coverage script (Task 12, once it passes) |
| `docs/todo.md`, `CHANGELOG.md`, both `README.md`, the sibling spec §10 | notes and reconciled counts |

---

### Task 1: The iced contract mechanism, and the palette corrections it proves

Spec §2, §5, §7. Test-first, committed green.

**Files:** Create `connectors/native-theme-iced/src/contract.rs`; modify `src/lib.rs` (`#[cfg(test)] mod contract;`), `src/extended.rs`.

**Interfaces:** Produces nothing public. Consumes `to_theme`, `palette::to_color`.

- [ ] **Step 1: The row type and the palette rows.** Per spec §5.1: one row per palette slot the connector writes, each naming the native field it must equal. Iterate `Theme::list_presets()` — sixteen presets, both modes; the four `*-live.toml` are geometry-only merge bases and are not in that list.
- [ ] **Step 2: The contrast invariant** (spec §7) in the same file: composite any colour with alpha below 1 over its own background, compute the ratio from the **native** fields and from the **connector's output**, assert ours is not worse, and print every pair below 4.5 without failing on it.
- [ ] **Step 3: Watch them fail.**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-iced --lib -- contract`
Expected: the `secondary.base.color` row fails (it holds the button surface where iced reads placeholder text), and the placeholder contrast pair fails at about 1.15:1 against a readable native pair. Any *other* failure is a finding: stop and report it.

- [ ] **Step 4: Remove the placeholder poisoning.** Delete `extended.secondary.base.color = to_color(colors.btn_bg);` (`extended.rs:109`). iced reads that slot as placeholder text (`text_input.rs:1769`, `text_editor.rs:1476`, `pick_list.rs:910`); iced's generated value is readable by construction.
- [ ] **Step 5: Write the hover.** Add `extended.secondary.strong.color = to_color(colors.btn_hover_bg);`, and `btn_hover_bg: Rgba` to `OverrideColors`, filled from `resolved.button.hover_background`. That slot is read by exactly one place — `button::secondary`'s hover (`button.rs:620`, verified) — so it means hover unambiguously. Drop `btn_bg` if nothing reads it; the compiler decides.
- [ ] **Step 6: Watch them pass.**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-iced`
Expected: all rows and the contrast invariant pass. Rows for values only `styles::*` can carry arrive in Task 2.

- [ ] **Step 7: Commit.**

---

### Task 2: `native_theme_iced::styles`

Spec §3, §4.2.

**Files:** `Cargo.toml`, `src/styles.rs` (new), `src/lib.rs`, `src/contract.rs`.

**Interfaces:** Produces `pub fn <widget>(resolved: &ResolvedTheme) -> impl Fn(&Theme, <widget>::Status) -> <widget>::Style + use<>` per spec §3's table. This exact signature was compiled against iced 0.14 before the plan was written — `use<>`, exhaustive construction, and acceptance by `.style()` with a `'static` bound.

- [ ] **Step 1: The dependency and its feature.** `iced_widget = { version = "0.14", optional = true }`; `widgets = ["dep:iced_widget"]` in `[features]`, with `widgets` in `default` (spec §4.2). Gate the module `#[cfg(feature = "widgets")]`.
- [ ] **Step 2: `button`, written completely**, as the pattern for the rest: capture the `Color` values by value so the closure is `'static`; match on `Status` for hover and pressed; construct `button::Style` **exhaustively**, never `..Default::default()` — those structs derive no `Default` and are not `#[non_exhaustive]`, so an upstream field addition then fails the build (C11).
- [ ] **Step 3: Its contract rows**, asserting each field equals the native value it claims, over all 32 preset/mode combinations.
- [ ] **Step 4: The remaining ten**, one at a time, compiling and testing after each: `button_primary`, `text_input`, `checkbox`, `toggler`, `scrollable`, `menu`, `container_card`, `slider`, `progress_bar`, `tooltip`. Each doc comment names the iced default it replaces and the slot whose meaning it corrects.
- [ ] **Step 5: Green, in all three configurations.**

```bash
CARGO_BUILD_JOBS=4 cargo test -p native-theme-iced
CARGO_BUILD_JOBS=4 cargo test -p native-theme-iced --no-default-features
CARGO_BUILD_JOBS=4 cargo clippy -p native-theme-iced --all-targets --all-features --locked -- -D warnings >/dev/null 2>&1; echo "clippy=$?"
```

- [ ] **Step 6: Commit.**

---

### Task 3: Accessibility preferences and the icon features (iced)

Spec §4.

**Files:** `src/lib.rs`, `Cargo.toml`, `examples/showcase-iced.rs`.

- [ ] **Step 1: Signatures.** `to_theme` and `from_preset` take `&AccessibilityPreferences`. `from_system` and `SystemThemeExt::to_iced_theme` (`src/lib.rs:191-198`) do **not**: both already read a `SystemTheme`, which carries `accessibility`, and a parameter would let a caller contradict the system it just asked for. They use `sys.accessibility`.
- [ ] **Step 2: Effects.** `text_scaling_factor` multiplies `font_size()` and `mono_font_size()`; `reduce_transparency` composites any emitted colour with alpha below 1 over its background and emits it opaque; add `pub fn reduce_motion(prefs: &AccessibilityPreferences) -> bool`.
- [ ] **Step 3: Re-export** `AccessibilityPreferences` from the crate root, as gpui does.
- [ ] **Step 4: A test that fails first.** Assert `native_theme::icons::load_icon` returns `Some` for a known icon. It fails on today's feature-less manifest.
- [ ] **Step 5: The features** of spec §4.2, then watch the test pass. Confirm with `cargo tree -p native-theme-iced -i native-theme`.
- [ ] **Step 6: Update the showcase** for the new signatures, so the crate and its example stay green together.
- [ ] **Step 7: Commit.**

---

### Task 4: `iced_aw`

Spec §3a. Six widgets native-theme models and iced core lacks.

**Files:** `Cargo.toml`, `src/styles/aw.rs` (new), `src/lib.rs`, `src/contract.rs`.

- [ ] **Step 1: The optional dependency.** `iced_aw = { version = "0.14", optional = true, default-features = false, features = ["card", "menu", "context_menu", "tab_bar", "tabs", "sidebar", "spinner", "selection_list"] }`, and `iced_aw = ["widgets", "dep:iced_aw"]` in `[features]` — it implies `widgets` because `iced_aw` depends on `iced_widget ^0.14.2`. **Not** in `default`: it pulls `iced_fonts` (3.3 MB of font data) and its iced support has lagged by months (C13).
- [ ] **Step 2: The module**, `#[cfg(feature = "iced_aw")]`. Six functions per spec §3a's table — eight features, six functions, because `tabs` builds on `tab_bar` and `context_menu` shares the `menu` styling. Same pattern as Task 2: `iced_aw` styles identically (`.style(impl Fn(&Theme, Status) -> Style)`, `Catalog` with `StyleFn`; `src/widget/card.rs:226`, `src/style/card.rs:75-85`).
- [ ] **Step 3: Contract rows** per function, feature-gated.
- [ ] **Step 4: Green, all three configurations**, including `--features iced_aw`.
- [ ] **Step 5: Commit.**

---

### Task 5: The gpui contract and coverage tripwire

Spec §5. The gpui mapping is already correct; this states it.

**Files:** Create `connectors/native-theme-gpui/src/contract.rs`; modify `src/lib.rs`, `src/colors.rs`.

- [ ] **Step 1: The contract table** over the `ThemeColor` fields with a native counterpart, including the ones this release corrected (`accent` ← `menu.hover_background`, `accent_foreground` ← `menu.hover_text_color`, `sidebar_accent*` ← the sidebar's selection pair, `secondary_hover` ← `button.hover_background`).
- [ ] **Step 2: Run.** Expected: passes immediately. A failing row is a finding in the mapping: stop and report.
- [ ] **Step 3: The coverage tripwire** (spec §5.2): three lists — contracted, `DERIVED` with the derivation named, `UPSTREAM_DEFAULT` with the reason — asserted to partition the 138 `ThemeColor` fields exactly.
- [ ] **Step 4: Prove it discriminates.** Remove one field name from its list; the test must fail naming that field. Restore.
- [ ] **Step 5: Retire the two bespoke `accent` tests** (C10), now table rows: `accent_is_the_platforms_menu_hover_pair` and `sidebar_accent_is_the_sidebars_selection_pair`. Keep `accent_foreground_comes_from_the_highlight_pairs`, which also proves highlight text never falls back to the window foreground.
- [ ] **Step 6: The contrast invariant** for gpui, same rule as Task 1 Step 2.
- [ ] **Step 7: Commit.**

---

### Task 6: The widget-coverage script

Spec §6a.4. Created and run here; **wired into the gates in Task 12**, once Tasks 7–9 have made it pass.

**Files:** Create `scripts/check-widget-coverage.py` and `docs/showcase-exceptions.toml`.

- [ ] **Step 1: The script.** `cargo metadata --format-version 1` gives each dependency's `manifest_path` and therefore its `src` directory — the only reliable way to reach a dependency's source, and why this is a script rather than a test. For gpui-component, enumerate types implementing `RenderOnce` or `IntoElement`, discarding harnesses and sub-parts by spec §6a.2's rules; for `iced_widget` and `iced_aw`, the widget modules. Compare against the showcase source. Exit non-zero on anything neither shown nor excepted.
- [ ] **Step 2: The exception file** with a reason per entry: a test harness, an internal sub-part, a layout wrapper with no visual surface, or no native counterpart.
- [ ] **Step 3: Run it and record the work list.**

Run: `python3 scripts/check-widget-coverage.py`
Expected: it exits non-zero, listing exactly the widgets Tasks 7–9 add. That list is those tasks' work order; paste it into the commit message.

- [ ] **Step 4: Commit** the script and the exception file. Nothing is wired to them yet, so the tree stays green.

---

### Task 7: The gpui showcase — the builders it never exercises

Spec §6a.1. Closes the "19 of 34 builders unused" gap.

**Files:** `connectors/native-theme-gpui/examples/showcase-gpui.rs`; a test in `src/`.

- [ ] **Step 1: The three widgets whose builders are unused:** `StatusBar`, `TitleBar`, `Combobox`, each with `geometry::status_bar`, `geometry::title_bar`, `geometry::combobox`.
- [ ] **Step 2: The value helpers** on existing sections: the five `icon_size_*`, `input_height`, `control_height`, `widget_gap`, `container_margin`, `window_margin`, `section_gap`, and the builders for widgets already shown (`menu_item`, `tooltip`, `table`, `radio`, `dialog_description`).
- [ ] **Step 3: The builder-coverage test**, in the connector, using `include_str!("../examples/showcase-gpui.rs")` to assert every `geometry::*` and `variants::*` name appears at least once.
- [ ] **Step 4: Prove it discriminates.** Delete one `geometry::` call; the test must fail. Restore.
- [ ] **Step 5: Green, and commit.**

---

### Task 8: The gpui showcase — the widgets it never renders

Spec §6a.2. In the order Task 6's script reported, one section at a time, compiling after each.

**Files:** `connectors/native-theme-gpui/examples/showcase-gpui.rs`.

- [ ] **Step 1: Controls and navigation:** `Pagination`, `Rating`, `Stepper`, `SidebarToggleButton`, `CarouselNext` / `CarouselPrevious`.
- [ ] **Step 2: Feedback and overlay:** `ProgressCircle`, `Marker`, `ShimmerText`, `ShimmerGlyphs`, `AlertDialog`, `HoverCard`, `WindowBorder`.
- [ ] **Step 3: Content:** `DescriptionList` / `DescriptionText`, and the chat components `Bubble`, `Message`, `MessageScroller`, `Attachment`.
- [ ] **Step 4:** Constructors are read from the vendored source at implementation time. A widget that cannot be built as described is a **finding**: report it, do not substitute another.
- [ ] **Step 5: Every colour, size and radius from the theme**, via `.native(cx, geometry::…)` or `cx.theme()`, as the surrounding sections do.
- [ ] **Step 6: Run the coverage script.** Expected: the gpui half is clean.
- [ ] **Step 7: Commit.**

---

### Task 9: Complete the iced showcase

Spec §6a.3 and the showcase principle of rationale §2.7.

**Files:** `connectors/native-theme-iced/examples/showcase-iced.rs`, `Cargo.toml` (dev-dependency on `iced_aw` with the feature).

- [ ] **Step 1: `styles::*` on every widget.** No widget stays on the palette default: the showcase is what the README's screenshots claim the connector achieves.
- [ ] **Step 2: The five missing themed modules:** `markdown`, `pane_grid`, `qr_code`, `table`, `canvas`.
- [ ] **Step 3: The six `iced_aw` widgets**, with `styles::aw::*`.
- [ ] **Step 4: Run the coverage script.** Expected: both halves clean.
- [ ] **Step 5: Commit.**

---

### Task 10: Showcase self-tests

Spec §6. Verified mechanism: an example with `test = true` runs its `#[cfg(test)]` tests under a plain `cargo test`, so CI and the canary pick them up with no workflow change.

**Files:** both `Cargo.toml` `[[example]]` entries, both showcase files.

- [ ] **Step 1: `test = true`** on both example targets.
- [ ] **Step 2: gpui tests** (spec §6.1): `every_tab_lays_out`, `resizable_groups_have_room_to_drag`, `interactive_controls_respond`. The clipboard assertion works: gpui's test platform holds a real in-memory clipboard (`platform/test/platform.rs:671-677`).
- [ ] **Step 3: Prove each discriminates.** Shrink a resizable container below `panels × PANEL_MIN_SIZE`; remove a control's handler. Both must fail. Restore.
- [ ] **Step 4: iced tests** (spec §6.2): `view_builds_for_every_tab`, `every_button_has_a_message` — iced renders a button with no `on_press` as `Status::Disabled` (`button.rs:342`), the iced form of the dead control — and `styles_cover_every_widget_shown`.
- [ ] **Step 5: Run both**, confirming the example targets appear in the output.
- [ ] **Step 6: Commit.**

---

### Task 11: Documentation

Spec §8.

- [ ] **Step 1: iced README** — a "Styles" section: what the palette gives automatically, what `styles::*` gives exactly, the feature table of spec §4.2 with its consumer-facing "what to write" rows, and a one-line example.
- [ ] **Step 2: iced crate docs** — the accessibility parameter, the features, the two-layer colour story.
- [ ] **Step 3: `docs/todo.md`** — the nine iced audit items move to done; the iced `geometry` gap stays, with the coverage script named as what keeps it visible.
- [ ] **Step 4: Reconcile the sibling specification.** `todo_v0.5.9_gpui-kit-0.6.4-spec.md` §10 states test counts (200 library, 6 seam) that this work changes, and §0.2 lists the public items added. Update both, so the two documents do not contradict each other when they are archived together.
- [ ] **Step 5: `CHANGELOG.md`** — the seven entries of spec §8, under the existing **undated** `## [Unreleased]` heading. Do not date it: `pre-release-check.sh:593` turns the asset-stamp check hard once it carries a date, and dating is the maintainer's release commit.
- [ ] **Step 6: Commit.**

---

### Task 12: Wire the coverage script into the gates

Now that Tasks 7–9 have made it pass.

- [ ] **Step 1:** `pre-release-check.sh` gains a check that runs the script.
- [ ] **Step 2:** `.github/workflows/dependency-canary.yml` gains a step, so an upstream release that adds a widget is reported the evening it appears rather than at the next release.
- [ ] **Step 3: Run both** paths locally and confirm they pass.
- [ ] **Step 4: Commit.**

---

### Task 13: Verification and hand-over (stops before any release action)

- [ ] **Step 1: The full gate.**

```bash
CARGO_BUILD_JOBS=4 cargo test --workspace --all-features
CARGO_BUILD_JOBS=4 cargo test -p native-theme-iced --no-default-features
CARGO_BUILD_JOBS=4 cargo test -p native-theme-iced --features iced_aw
CARGO_BUILD_JOBS=4 cargo clippy --workspace --all-targets --all-features --locked -- -D warnings >/dev/null 2>&1; echo "clippy=$?"
python3 scripts/check-widget-coverage.py ; echo "coverage=$?"
cargo audit >/dev/null 2>&1; echo "audit=$?"
CARGO_BUILD_JOBS=4 ./pre-release-check.sh
```
Expected: all green; `pre-release-check.sh` reports no failures and one warning, the stale asset stamp.

- [ ] **Step 2: Spec §9 acceptance list** — run each item, paste each result into the hand-over message.
- [ ] **Step 3: Report to the maintainer.** Both design document sets are now implemented. Report: test counts per crate, the contract tables' row counts, the contrast baseline (how many pairs sit below AA, and that none degraded), anything Tasks 8 or 9 could not build, and the remaining maintainer-only steps, unchanged from the sibling plan's Task 8 — the visual check, pushing the branch, `./scripts/pre-release.sh`, the release commit, CI, the tag, and archiving **all six** design documents afterwards.

**Do not push the branch, open a PR, tag, or publish.**

---

## Execution notes (model routing under the maintainer's dispatch policy)

Tasks 1–6, 10 and 12 have mechanical gates — a named failing test, a script's
exit code, a compile — and belong to the `implement` agent. Tasks 7–9 are
showcase work whose only gate is "compiles and the coverage script is clean",
but whose content is judgment about how a widget should be demonstrated: the
orchestrator does them inline, as the sibling plan did for its showcase task.
Tasks 11 and 13 are orchestration.
