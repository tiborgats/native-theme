# Widget Info Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the gpui showcase's Widget Info panel true, and keep it true, with three checks and an advisory report — no new public API.

**Architecture:** Three gates, each the smallest thing that catches its defect class. Presence: every demo has a panel, every widget has a demo. Builders: the `geometry::` builders a demo applies are the ones its panel names — one lexical test extending `src/showcase.rs`. Colours: every claim cites the upstream line it was read at, and a test opens that line. A fourth script reports what panels omit, advisory only.

**Tech Stack:** Rust 1.95, gpui-component 0.6.4 (floor; 0.6.6 verified identical for every cited line), Python 3 for the omission report and for `check-widget-coverage.py`, the lexical helpers already in `connectors/native-theme-gpui/src/showcase.rs`. Whether the citation check is a Rust test or a Python script is decided in Task 3 Step 1.

**Spec:** [`todo_v0.5.9_widget-info-spec.md`](todo_v0.5.9_widget-info-spec.md)
**Rationale:** [`todo_v0.5.9_widget-info-rationale.md`](todo_v0.5.9_widget-info-rationale.md)

## Global Constraints

- **NEVER LIE, NEVER INVENT.** Every claim written into a panel is read from the cited source line. A claim you cannot verify is a finding to report, not a sentence to write, and never a citation guessed to make a test pass.
- No runtime panics, no `unsafe`. The repo's PreToolUse hook enforces this on every Write/Edit under `src/`, **including test modules**. It recognises `#[test]`, `#[cfg(test)]` and `#[allow(clippy::unwrap_used` as markers — **`#[gpui::test]` is not recognised**, so such a fn needs `#[allow(clippy::unwrap_used, clippy::expect_used)]` on its module.
- No hardcoded theme values. Every number a test expects comes from the resolved theme, the cited source, or the value under test.
- **No public item is added.** `geometry` stays at 37 public items (spec §0.2).
- Every gate ships a **discrimination proof**: a seeded defect must make it fail, naming the line. A gate never seen to fail is not a gate.
- Verified against gpui-component **0.6.4** (the manifest floor and the `Cargo.lock` pin). 0.6.6 differs only in `inspector.rs` and `label.rs`; if you cite a line in either, re-read it in both.
- Run `./pre-release-check.sh` before committing code changes, with `CARGO_BUILD_JOBS=4`.
- The Bash tool's shell is **fish**: a multi-word flag passed through an unquoted variable arrives as one argv element. Write `--features iced_aw` literally.
- Never `git add -A`. No `Co-Authored-By` or AI-attribution lines in commits.

---

### Task 1: The block convention and the presence test

**Files:**
- Modify: `connectors/native-theme-gpui/src/showcase.rs`
- Modify: `connectors/native-theme-gpui/examples/showcase-gpui.rs` — **only if** Step 3 finds an id that is not `tt-`-prefixed

**Interfaces:**
- Consumes: `without_comments_or_strings` (`src/showcase.rs:59`)
- Produces: `demo_block_starts`, the block boundary Tasks 2 and 3 rely on

- [ ] **Step 1: Write the test**

It does **not** start red: every panel already has its own `tt-` block (spec F9). Step 2 proves it fires.

Boundaries come from the raw `SHOWCASE`, because `without_comments_or_strings` strips string literals and `.id("tt-` would not survive. The code check runs on the stripped copy, whose line numbers still match (F5).

```rust
/// The marker that opens a demo block, and the call that gives it a panel.
const BLOCK_ID: &str = ".id(\"tt-";
const PANEL_CALL: &str = ".on_hover(self.hover_info(";
/// The one demo whose id reaches `.id()` through a `const` table (Step 3).
const INDIRECT_BLOCK_ID: &str = ".id(group.id)";

fn demo_block_starts(source: &str) -> Vec<usize> {
    source
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(BLOCK_ID) || line.contains(INDIRECT_BLOCK_ID))
        .map(|(n, _)| n)
        .collect()
}

/// Spec §2.2: every `tt-` demo block carries a Widget Info panel.
#[test]
fn every_demo_block_has_a_widget_info_panel() { /* boundaries from SHOWCASE,
    body from the stripped copy; report the block's line */ }
```

Assert `!starts.is_empty()` before the loop, or a detector that matches nothing passes vacuously. Slice with `.min(body.len())` on both ends — the hook rejects panic-prone indexing under `src/`.

- [ ] **Step 2: Prove it fires (discrimination proof, W8)**

Delete the `.on_hover(self.hover_info(` from any one `tt-` block, run the test, confirm it fails naming that line, revert.

- [ ] **Step 3: Teach the detector the indirected id, and guard it**

**No showcase change is needed here.** All 102 panels already have their own `tt-` block (spec F9); one of them just reaches `.id()` through a `const`: `render_resizable_group` writes `.id(group.id)` (`:5486`) and `RESIZABLE_GROUPS` already holds `tt-resizable-h`/`tt-resizable-v` (`:459, :480`).

Add `.id(group.id)` as a second block marker, and a second test — `every_demo_id_is_a_tt_id` — asserting every `id: "…"` field in the showcase starts with `tt-`. Without that guard the marker is a hole: renaming a table entry to a non-demo keeps the marker and loses the meaning.

With the marker in place the block count is **102, matching the 102 panels exactly** — a 1:1 that the earlier 101 figure hid.

The two icon sections are deliberately left alone: giving them ids before they have panels would make this commit red. They get ids and panels together, in Task 7.

- [ ] **Step 4: Do *not* move the page-level bindings**

It is tempting to hunt them down — `grep -n "let .*geometry::"` finds 17, of which 7 use the `native_geometry(cx, geometry::X)` form a naive `= geometry::` search misses. **Leave them where they are.** Task 2's attribution rule (spec §3) reads a binding in the enclosing `fn`, so `button_style` `:2776` is correctly attributed to all ten Button blocks and `accordion_title_style` `:5514` to the Accordion block, with no movement and no declarations.

Moving them would mean either duplicating one computation across ten blocks or maintaining a table of which block uses which binding. Neither is worth it, and an earlier draft of this plan specified exactly that before the attribution rule was measured.

- [ ] **Step 5: Run, then commit**

```bash
cargo test -p native-theme-gpui --lib showcase::every_demo
git add connectors/native-theme-gpui/src/showcase.rs
git commit -m "test(showcase): every demo block carries a Widget Info panel"
```

---

### Task 2: Builders used == builders named

**Files:**
- Modify: `connectors/native-theme-gpui/src/showcase.rs`
- Modify: `connectors/native-theme-gpui/examples/showcase-gpui.rs`

**Interfaces:**
- Consumes: the `tt-` blocks from Task 1; `references` (`src/showcase.rs:165`)

- [ ] **Step 1: Write the failing test**

```rust
/// Spec §3: the `geometry::` builders a demo block uses are exactly the
/// builders its panel names.
#[test]
fn every_demo_names_the_builders_it_applies() { /* … */ }
```

Per block, two sets. **Applied**: `geometry::X` in the block's code (comments and literals stripped), *or* a `let <var> = … geometry::X` binding in the enclosing `fn` whose `<var>` the block references — spec §3's attribution rule. **Named**: `geometry::X` in the block's panel text (the literals). Require equality.

The enclosing `fn` runs from the preceding `    fn ` at four-space indent to the next one. Verify the rule before trusting the report: it must attribute `button` to all ten `tt-btn-*` blocks and `accordion_title` to `tt-accordion`. If it does not, the report's mismatches are the rule's fault, not the showcase's.

- [ ] **Step 2: Run it to see it fail**

Expected: FAIL, naming the seven blocks in rationale §1.5 — Dialog, Popover, NumberInput, Form/Field, Sidebar, InputGroup, Tree. If fewer than seven appear, the block scoping is wrong; fix it before writing any note.

- [ ] **Step 3: Write the seven missing notes**

Each names the builder and what it carries, in the form the 48 good panels use. Read the builder in `geometry.rs` before writing the note — do not copy a neighbouring panel's wording.

- [ ] **Step 4: Discrimination proof (W8)**

Add `.native(cx, geometry::popover)` to a block whose note omits it; confirm the failure names the line; revert.

- [ ] **Step 5: Commit**

```bash
git add connectors/native-theme-gpui/src/showcase.rs connectors/native-theme-gpui/examples/showcase-gpui.rs
git commit -m "test(showcase): a demo names exactly the builders it applies"
```

---

### Task 3: The citation check

**Files:**
- Modify: `connectors/native-theme-gpui/src/showcase.rs` (or a script — see Step 1)
- Modify: `connectors/native-theme-gpui/examples/showcase-gpui.rs`
- Modify: `pre-release-check.sh`

**Interfaces:**
- Produces: the `(role, field, value, cited-at)` claim shape every later task fills in

- [ ] **Step 1: Decide where the check runs, and record it**

The check needs the vendored crate's `src/` at run time, from `cargo metadata` only (spec §4.4). Either a build script writes the path to `OUT_DIR` for a `#[test]`, or the check is a Python script called by `pre-release-check.sh`. Pick the simpler one for this repo, implement it, and record the choice in spec §9.

- [ ] **Step 2: Extend the claim shape and render it — its own commit**

`(role, field, value)` becomes `(role, field, value, cited_at)`. `widget_tooltip` renders `text: link #2a7ab0 (button.rs:993)`. An empty citation renders without the parenthetical and is **counted** (Step 4).

**This step touches all 324 claim sites**, because the tuple type changes and every existing 3-tuple stops compiling. Add `""` to each — a purely mechanical pass, no reading of upstream, no judgement. **Commit it on its own**, before the check exists, so that the diff a reviewer reads for the check is the check and not 324 lines of `""`.

The alternative — encoding the citation inside the field string as `"link@button.rs:993"` — avoids the mass edit and allows incremental adoption. It is **rejected**: the theme-contracts work already removed a stringly-typed encoding from `contract.rs` after a typo let rows silently pass, and re-introducing one here would repeat that. One mechanical commit is the cheaper price.

- [ ] **Step 3: Write the check**

For each claim with a citation: resolve `<file>:<line>` or `<file>:<from>-<to>` against the crate source, read it, require the field name to appear as an identifier. `tokens.x` satisfies a claim of `x` (spec F7). A missing file is an error naming the filename.

- [ ] **Step 4: Gate the uncited count**

`pre-release-check.sh` fails when any claim lacks a citation. It starts at 324 and only falls (spec §6.6), so intermediate commits are green and a release with an uncited claim is impossible.

At this task the count is 324 — that is correct. Tasks 4, 6 and 7 drain it. Wire the check now and land Task 3 with the gate **reporting** the count; Task 8 flips it to failing, once the count has actually reached zero.

- [ ] **Step 5: The prose existence check (spec §4.6, W9)**

45 of the 227 "Not themeable" entries already cite a source file. Verify the file exists and the symbol is declared in it — **existence only, never semantics**. This is what catches an upstream *deletion*, the failure this project has already been bitten by (`ThemeColor::tiles`, removed in a patch release).

Entries without a citation are **not** required to gain one. Many state an absence with no symbol to point at, and demanding a citation would invite an invented one.

- [ ] **Step 6: The four discrimination proofs (spec §4.5 and spec §4.6)**

Seed each, confirm the failure names the claim, revert.

- [ ] **Step 7: Commit**

```bash
git add connectors/native-theme-gpui/src/showcase.rs connectors/native-theme-gpui/examples/showcase-gpui.rs pre-release-check.sh
git commit -m "test(showcase): a colour claim is read at the line it cites"
```

---

### Task 4: Correct the seven false claims and the six Button variants

**Files:**
- Modify: `connectors/native-theme-gpui/examples/showcase-gpui.rs`

- [ ] **Step 1: Apply rationale §1.3, with citations**

Toggle's four claims; Button (Link)'s two plus the two it omits; Button (Text)'s two plus one; Radio's `bg`, indicator size, label colour (`radio.rs:212`) and `radius * 0.5` (`:215`); NumberInput's height; AlertDialog's cross-reference; Switch's size.

- [ ] **Step 2: Rename the six Button variant families**

`primary` → `button_primary` and siblings, each citing its line (`button.rs:936-941`, `:954`, `:1086-1121`, `:1170-1206`). The hex does not change.

**Do not touch Ghost.** It correctly names the colours `variants::ghost_button` gives it; cite `variants.rs:54`.

- [ ] **Step 3: Run the citation check — these claims must now pass**

- [ ] **Step 4: Commit**

```bash
git add connectors/native-theme-gpui/examples/showcase-gpui.rs
git commit -m "fix(showcase): the panels state the fields their widgets read"
```

---

### Task 5: The omission report — **after** Task 6, not before

**Files:**
- Modify: `connectors/native-theme-gpui/src/showcase.rs`

This task was ordered before the citation pass, and that was wrong. The
report has to know **which upstream file implements each widget**, and the
plan never said where that mapping comes from. Guessing it from the panel's
name (`Popover` → `popover.rs`) is fuzzy and would make the report's misses
indistinguishable from its mistakes.

Once Task 6 has run, the mapping is free and exact: **the files a panel cites
are the files its widget is implemented in**. So the report becomes a few
lines over machinery that already exists — `panel_claims`, `candidates`,
`mentions` — instead of a new widget-to-file guesser.

- [ ] **Step 1: For each panel, scan the files its own claims cite**

Collect every `cx.theme().<field>` and `cx.theme().tokens.<field>` in those
files; report the ones the panel never names.

- [ ] **Step 2: Print, do not fail (W6)**

A panel legitimately omits fields belonging to states and variants its demo
does not show. Making this a gate would need an exception per unshown state
across ~100 widgets, which is the sprawl this design exists to avoid.

- [ ] **Step 3: Record the count in the CHANGELOG**

So the residual is a number someone chose, not one nobody looked at.

---

### Task 6: The citation pass

**Files:**
- Modify: `connectors/native-theme-gpui/examples/showcase-gpui.rs`

The bulk of the work, and a task of one kind: every remaining claim gains a citation, which means reading the upstream line — the audit itself, made durable. Split from Task 7 because that one adds *new demos*, which is different work with a different review surface.

- [ ] **Step 1: Page by page, cite every remaining claim**

Read the widget's `render`, confirm the field, write the citation. Where the reading contradicts the claim, correct the claim. Triage Task 5's report for the same panel while you are there.

**Commit per page, not per claim.** Each commit lowers the uncited count and is green. A page is a natural review unit; 300 claims in one diff is not.

- [ ] **Step 2: Confirm the uncited count is zero**

If a claim has no line to cite because nothing reads that field, the claim is wrong and the panel is corrected — never a citation chosen to make the check pass.

---

### Task 7: The missing demos and panels

**Files:**
- Modify: `connectors/native-theme-gpui/examples/showcase-gpui.rs`
- Modify: `docs/showcase-exceptions.toml`, `docs/todo.md`

- [ ] **Step 1: A real `Select` demo beside `Combobox`**

So the carried-colour difference (`geometry.rs:441-467`) is visible side by side — the distinction the connector argues for and nothing currently shows.

- [ ] **Step 2: A real `Textarea` demo in the Input section**

- [ ] **Step 3: The two icon sections, with their `tt-` ids**

Deferred from Task 1 so ids and panels land together. Rename `native-icons-grid` to `tt-native-icons`, give the animated section `tt-animated-icons`, then write both panels: the icon theme, the resolved icon size and the fallback rule; the frame source and the reduced-motion rule.

`Command` is deliberately **not** handled here: it is the coverage gate that reveals it is missing, so it is resolved in Task 8 where that failure is observed.

- [ ] **Step 4: Every new claim carries a citation, and the tests stay green**

```bash
git add connectors/native-theme-gpui/examples/showcase-gpui.rs docs/showcase-exceptions.toml docs/todo.md
git commit -m "feat(showcase): Select, Textarea and the icon grids have panels"
```

---

### Task 8: The coverage-gate fix

**Files:**
- Modify: `scripts/check-widget-coverage.py`

- [ ] **Step 1: Tighten gpui matching per spec §7**

- [ ] **Step 2: Discrimination proof (W8) — and the finding it produces**

`std::process::Command::new` must no longer satisfy the `Command` widget:

```
python3 scripts/check-widget-coverage.py
```

Expected: `gpui: Command` reported missing. That is both the proof the fix works **and** a genuine finding — `Command` really is absent from the showcase.

- [ ] **Step 3: Resolve `Command`, so this task commits green**

Per rationale §6, question 4, the maintainer's answer decides: a real demo, or an exception with a reason plus a follow-up in `docs/todo.md`. Do this here, not earlier, because Step 2 is where the need is demonstrated rather than asserted.

- [ ] **Step 4: Update the script's docstring**

Its "Matching" section documents the path-segment rule that caused this. Rewrite it to describe the new rule and why.

- [ ] **Step 5: Flip the uncited-claim gate to failing**

Task 3 Step 4 wired it as a report. The count must be zero before this flip; if it is not, the remaining claims are named and finished first.

- [ ] **Step 6: Commit**

```bash
git add scripts/check-widget-coverage.py pre-release-check.sh docs/showcase-exceptions.toml docs/todo.md
git commit -m "fix(scripts): a std path segment no longer proves a gpui widget is shown"
```

---

### Task 9: CHANGELOG, docs, and archive

**Files:**
- Modify: `CHANGELOG.md`, `docs/todo.md`, `pre-release-check.sh`
- Move: the three `docs/todo_v0.5.9_widget-info-*.md` into `docs/archive/`

- [ ] **Step 1: CHANGELOG**

Under `Fixed`: the seven corrected claims, named. State plainly that **no public item was added**. Record the omission-report count (spec §8, item 7), so the residual is a number someone chose.

- [ ] **Step 2: `docs/todo.md`**

The `Command` follow-up if deferred; the Tier U entry for empirical measurement (rationale §5, blocked on gpui-pre exposing the scene); and the trigger for derived geometry (rationale §3.3).

- [ ] **Step 3: Update the check count in `pre-release-check.sh`'s header**

- [ ] **Step 4: Full gate**

```bash
CARGO_BUILD_JOBS=4 ./pre-release-check.sh
```

- [ ] **Step 5: Fill in spec §9 "As built"**

Every place the implementation corrected the design, including spec §4.4's choice. Do this before moving the files.

- [ ] **Step 6: Move and commit**

```bash
git mv docs/todo_v0.5.9_widget-info-rationale.md docs/todo_v0.5.9_widget-info-spec.md docs/todo_v0.5.9_widget-info-plan.md docs/archive/
git commit -m "docs(archive): the Widget Info design documents are implemented"
```

Archive the other v0.5.9 design documents at the same time, per the standing rule that archiving is the last task of every plan.
