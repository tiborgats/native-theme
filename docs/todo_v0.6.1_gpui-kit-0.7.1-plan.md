# v0.6.1 — gpui connector on GPUI Kit 0.7.1: Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make native-theme-gpui build and look native on gpui-component / gpui-base / gpui-kit 0.7.1 with gpui-pre 0.3.8, show the widgets 0.7.1 added, and leave every claim true for 0.7.1 — released as v0.6.1, because v0.6.0 no longer compiles for anyone who resolves dependencies fresh.

**Architecture:** Move the floors, the lockfile and the README's floor table together; map the two new `IconName`s to genuine glyphs and end the icon tables in a wildcard so an upstream icon can never again break a released build (the exhaustive list moves into the test module). Restore the platform's font where 0.7.1's size ladder shrank it: a child label for `Button` (`geometry::button_label`, generalising what the showcase's `tool_label` already does by hand), the same text for `Toggle` (`geometry::toggle`), and `Size::Size` for `DataTable` (`geometry::data_table_size`); before/after layout dumps prove the Basic page is back to 0.7.0's. Renumber 436 moved citations in a digits-only commit and rewrite the 17 whose code changed. Show `ColorSelect` and `SpeechWaveform` (driven by a synthetic input, no microphone), except `SpeechButton`.

**Tech Stack:** Rust 2024, gpui-pre 0.3.8 (`#[gpui::test]`, `TestAppContext`), gpui-component / gpui-base / gpui-kit 0.7.1, Python 3.11 scripts.

**Spec:** [`todo_v0.6.1_gpui-kit-0.7.1-spec.md`](todo_v0.6.1_gpui-kit-0.7.1-spec.md) — read it first; it carries every upstream citation, the exact code, and Appendices A and B. Reasons: [`todo_v0.6.1_gpui-kit-0.7.1-rationale.md`](todo_v0.6.1_gpui-kit-0.7.1-rationale.md) (L1–L15).

## Global Constraints

- Floors, verbatim: `gpui = { package = "gpui-pre", version = "0.3.8" }`, `gpui-component = "0.7.1"`, `gpui-base = "0.7.1"`, dev `gpui = { package = "gpui-pre", version = "0.3.8", features = ["test-support"] }`, dev `gpui-kit = { version = "0.7.1", features = ["tree-sitter-rust"] }`. Carets, never `=`. No `speech` feature.
- Connector `rust-version = "1.95.0"` (measured 2026-10-06; do not change).
- Workspace version `0.6.1`; branch `v0.6.1-rc1` from `main`; CHANGELOG lines under a new, undated `## [Unreleased]`.
- Public API: additive only — `geometry::button_label`, `geometry::toggle`, `geometry::data_table_size`. Nothing removed or re-typed.
- No `unwrap` / `expect` / `panic!` / `unreachable!` / slice indexing / `unsafe` / unchecked integer arithmetic outside test code, **including the showcase**. The PreToolUse hook `.claude/hooks/no-runtime-panics.sh` lets an edit containing `.expect(` through only when the edited text contains `#[test]`, `#[cfg(test)]` or `#[allow(clippy::unwrap_used`; `#[gpui::test]` does not match; files under `tests/` are exempt by path.
- No invented values: every colour and size comes from the resolved theme or is an upstream literal cited at its line. The synthetic audio envelope (spec §7.2) is demonstration data and reaches no style setter.
- Never mix icon sets: an unmapped `IconName` returns `None`.
- No source file cites these three design documents by path (they move in Task 10, after the compatibility stamp has hashed the connector's sources).
- Every cargo command runs from the **repository root** with `CARGO_BUILD_JOBS=4`. Never set `CARGO_TARGET_DIR` under `/tmp` (a RAM-backed tmpfs).
- Before every commit: `cargo fmt -p native-theme-gpui` (and `-p native-theme` if touched). Stage files by path; never `git add -A` / `.`. Commit messages carry no `Co-Authored-By` or any AI attribution.
- Shell snippets are bash; run a multi-line one as `bash <<'EOF' … EOF` (the session's shell is fish).
- Never tag, push, publish, create a release, or run `scripts/generate_assets_release.sh`.
- When a step's expected output does not appear, stop and report; do not improvise around it. When cited upstream code no longer says what a comment claims, the claim is the finding: report it, do not reword the comment to fit.

## Review Focus

- **Text scaling other than 1** (accessibility): the label's size is `button.font.size × s` — the unit tests run every case at 1.0, 1.5 and 0.8 (`for_each_case`), the Button seam at 1.0 and 1.5 (Task 3).
- **A theme whose button font is not its body font**: the label follows `button.font`, not the rem — `button_label_follows_the_button_font_not_the_body_font` (Task 3).
- **A theme that states no list row height** (KDE): the DataTable keeps upstream's Medium row — `data_table_size_falls_back_to_upstream_medium` requires both branches to run (Task 5).
- **Stopping while the waveform captures**: the synthetic input's timer ends with the session — `the_synthetic_input_feeds_levels_and_stops_with_the_session` counts delivered blocks after `stop` (Task 7).
- **An `IconName` newer than the tables**: `None`, and the library still builds — proved once by deleting an arm (Task 1 Step 9); the test module is what fails.

## File map

| File | Responsibility in this plan |
|---|---|
| `connectors/native-theme-gpui/Cargo.toml`, `Cargo.lock`, `README.md` (floor table, Quick start) | floors (Task 1) |
| `native-theme/icons/{lucide,material}/{mic,square}.svg` | genuine glyphs (Task 1) |
| `src/icons.rs`, `examples/showcase-gpui/{support.rs,info/icons.rs}`, `README.md` | icon tables, wildcard, list macro, gallery, counts (Task 1) |
| `src/*.rs`, `tests/seams.rs`, `README.md`, `examples/showcase-gpui/**`, `docs/showcase-exceptions.toml`, `ROADMAP.md` | citations and claims (Task 2) |
| `src/geometry.rs`, `tests/seams.rs`, `examples/showcase-gpui/{demo.rs,support.rs,info/buttons.rs,info/leaves.rs}` | Button label (Task 3) |
| `src/geometry.rs`, `tests/seams.rs`, `examples/showcase-gpui/{demo.rs,info/buttons.rs}` | Toggle (Task 4) |
| `src/geometry.rs`, `tests/seams.rs`, `examples/showcase-gpui/{demo.rs,info/data.rs}` | DataTable (Task 5) |
| `examples/showcase-gpui/{app.rs,demo.rs,pages/inputs.rs,info/inputs.rs,tests.rs}` | ColorSelect (Task 6) |
| `examples/showcase-gpui/{speech.rs (new),main.rs,app.rs,demo.rs,pages/inputs.rs,info/inputs.rs,tests.rs}`, `docs/showcase-exceptions.toml` | SpeechWaveform, SpeechButton exception (Task 7) |
| `CHANGELOG.md`, `ROADMAP.md`, `docs/todo.md`, `docs/todo_gpui-full-theme.md`, `README.md`, `Cargo.toml` (four) | notes, roadmap, version (Task 8) |
| `docs/COMPATIBILITY.toml`, README Verified line | stamp (Task 9) |
| `docs/archive/` | archive (Task 10) |
| `target/layout-071/` (not committed) | before/after layout dumps (Tasks 0, 3, 9) |

Paths are relative to `connectors/native-theme-gpui/` when they start with `src/`, `tests/`, `examples/` or `README.md`, and to the repository root otherwise. Line numbers are those of `942302d6`; locate by content.

---

### Task 0: Branch, design documents, baseline dumps

**Files:** none changed in the crate.

**Interfaces:** Produces the branch and `target/layout-071/before/rest/gpui-<preset>-<variant>.json` (six files), the baseline the layout gate compares against.

- [ ] **Step 1: Precondition.** `git status --short` shows nothing staged and no modified tracked file; the only untracked files are the three design documents. Otherwise stop and ask the maintainer.

- [ ] **Step 2: Branch.**

```bash
git switch -c v0.6.1-rc1 main
```

- [ ] **Step 3: Commit the design documents** (Task 10 moves them with `git mv`, which needs them tracked):

```bash
git add docs/todo_v0.6.1_gpui-kit-0.7.1-rationale.md docs/todo_v0.6.1_gpui-kit-0.7.1-spec.md docs/todo_v0.6.1_gpui-kit-0.7.1-plan.md
git commit -m "docs: the gpui connector on GPUI Kit 0.7.1, rationale, specification and plan"
```

- [ ] **Step 4: Baseline dumps (still on 0.7.0).** If `target/layout-071/before/rest/` already holds six JSON files (the design pass took them from `942302d6` on 2026-10-06), keep them. Otherwise, in the desktop session (each run opens the showcase window, writes the dump once the layout settles, and quits):

```bash
PHASE=before bash <<'EOF'
set -e
out="target/layout-071/$PHASE/rest"
mkdir -p "$out"
CARGO_BUILD_JOBS=4 cargo build -p native-theme-gpui --example showcase-gpui --locked
bin=target/debug/examples/showcase-gpui
for row in "kde-breeze dark freedesktop breeze-dark" "kde-breeze light freedesktop breeze" \
           "material dark material -" "material light material -" \
           "catppuccin-mocha dark lucide -" "catppuccin-mocha light lucide -"; do
  set -- $row
  extra=(); [ "$4" != "-" ] && extra=(--icon-theme "$4")
  "$bin" --theme "$1" --variant "$2" --icon-set "$3" "${extra[@]}" --tab basic \
    --dump-layout "$out/gpui-$1-$2.json"
done
ls "$out"
EOF
```

Expected: six JSON files. Nothing is committed.

- [ ] **Step 5: The comparison script** (used by Tasks 3 and 9), saved as `target/layout-071/compare.py` (not committed):

```python
import json, glob, os, sys
bad = 0
for f in sorted(glob.glob("target/layout-071/before/rest/*.json")):
    b = json.load(open(f))["elements"]
    a = json.load(open(f.replace("/before/", "/after/")))["elements"]
    diffs = [k for k in sorted(set(b) | set(a)) if b.get(k) != a.get(k)]
    print(os.path.basename(f), len(b), "elements,", len(diffs), "differ")
    for k in diffs:
        print("  ", k, b.get(k), "->", a.get(k))
    bad += len(diffs)
sys.exit(1 if bad else 0)
```

---

### Task 1: Floors, lockfile, README floors, icons

One task because nothing compiles on 0.7.1 until the icons are in, and the README's floor table is test-checked against the manifest.

**Files:**
- Modify: `Cargo.toml` (connector), `Cargo.lock`, `README.md` (`:47-57`, `:71-80`, `:100`, `:512`, `:537`, `:553`)
- Create: `native-theme/icons/lucide/mic.svg`, `native-theme/icons/lucide/square.svg`, `native-theme/icons/material/mic.svg`, `native-theme/icons/material/square.svg`
- Modify: `src/icons.rs` (three tables, docs, test modules), `examples/showcase-gpui/support.rs` (`GPUI_ICONS`), `examples/showcase-gpui/info/icons.rs:49`

**Interfaces:** Consumes nothing. Produces a stack that compiles with `--all-targets`, and in the icon test module `ALL_ICON_NAMES: &[IconName]` and `variant_name(&IconName) -> &'static str` (both `pub(super)`). Known failing tests after this task: `every_colour_claim_is_read_at_the_line_it_cites` (Task 2) and `a_page_sample_icon_the_set_lacks_is_absent` (Task 3).

- [ ] **Step 1: Manifest and README floors.** Apply spec §2.1 to `connectors/native-theme-gpui/Cargo.toml` and spec §2.4 to its `README.md` (the Required table, the Quick-start line, the `gpui-pre` line, the added sentence). Keep every other line; do not touch the Verified line.

- [ ] **Step 2: Lockfile.**

Run: `cargo update -p gpui-component -p gpui-base -p gpui-kit -p gpui-pre`
Expected: the changes listed in spec §2.2, nothing outside the gpui closure.

- [ ] **Step 3: Verify the failure.**

Run: `CARGO_BUILD_JOBS=4 cargo build -p native-theme-gpui --lib 2>&1 | grep -E "^error"`
Expected: three `error[E0004]: non-exhaustive patterns: ... IconName::Mic ... IconName::Square` (at the three tables).

- [ ] **Step 4: Glyphs.**

```bash
scripts/update_icons.sh add lucide mic
scripts/update_icons.sh add lucide square
scripts/update_icons.sh add material mic
scripts/update_icons.sh add material square
R=$(cargo metadata --format-version 1 | python3 -I -c 'import json,sys; print([p["manifest_path"] for p in json.load(sys.stdin)["packages"] if p["name"]=="gpui-kit-assets"][0].rsplit("/",1)[0])')
cmp native-theme/icons/lucide/mic.svg "$R/assets/icons/mic.svg" && cmp native-theme/icons/lucide/square.svg "$R/assets/icons/square.svg" && echo IDENTICAL
```

Expected: four `… <- https://raw.githubusercontent.com/…` lines, then `IDENTICAL`.

- [ ] **Step 5: Tables.** In `src/icons.rs` add the six arms of spec §3.2 in order, the wildcard arm (with its comment and `#[allow(unreachable_patterns)]`) as the last arm of each of the three `match icon` blocks, and the doc-comment edits of spec §3.2.

- [ ] **Step 6: Tests.** Spec §3.3: turn `ALL_ICON_NAMES` into the `all_icon_names!` invocation (every identifier of today's list, plus `Mic` and `Square`); the count test says `106` with its new comment; `every_none_is_an_allowed_gap`'s two messages name the icon through `variant_name`; add `mic_and_square_map_to_the_sets_own_glyphs` and `mic_and_square_take_standard_names_on_every_desktop`.

- [ ] **Step 7: Showcase gallery and counts.** Spec §3.4 (`GPUI_ICONS`, its doc line, `info/icons.rs:49`) and §3.5 (README icons paragraph and counts).

- [ ] **Step 8: Build and test.**

Run: `CARGO_BUILD_JOBS=4 cargo build -p native-theme-gpui --all-targets --all-features --locked 2>&1 | grep -E "^(warning|error)" ; echo done`
Expected: `done` alone (no warning, no error).

Run: `CARGO_BUILD_JOBS=4 cargo clippy -p native-theme-gpui --all-targets --all-features --locked -- -D warnings 2>&1 | tail -1`
Expected: `Finished` (the wildcard arms and the macro are clean under `-D warnings`).

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --lib icons`
Expected: all pass.

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme --test icon_sources`
Expected: pass.

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --no-fail-fast 2>&1 | grep -E "FAILED|test result"`
Expected: exactly two FAILED — `every_colour_claim_is_read_at_the_line_it_cites`, `a_page_sample_icon_the_set_lacks_is_absent`. In particular the three `compat` tests pass (the README floors moved with the manifest). Anything else: stop and report.

- [ ] **Step 9: Prove the wildcard holds.** Temporarily delete the `IconName::Mic` arm from `lucide_name_for_gpui_icon` only:

Run: `CARGO_BUILD_JOBS=4 cargo build -p native-theme-gpui --lib 2>&1 | grep -cE "^error"`
Expected: `0` (the library still builds — the point of L3).

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --lib -- mic_and_square every_none 2>&1 | grep -E "FAILED|has no Lucide"`
Expected: the Lucide glyph test and the gap test fail, and the gap test's message names `Mic`. Restore the arm.

- [ ] **Step 10: Commit.**

```bash
cargo fmt -p native-theme-gpui
git add connectors/native-theme-gpui/Cargo.toml Cargo.lock native-theme/icons/lucide/mic.svg native-theme/icons/lucide/square.svg native-theme/icons/material/mic.svg native-theme/icons/material/square.svg connectors/native-theme-gpui/src/icons.rs connectors/native-theme-gpui/examples/showcase-gpui/support.rs connectors/native-theme-gpui/examples/showcase-gpui/info/icons.rs connectors/native-theme-gpui/README.md
git commit -m "build(gpui): move to gpui-component, gpui-base and gpui-kit 0.7.1 on gpui-pre 0.3.8; Mic and Square map to genuine Lucide, Material and freedesktop glyphs, and the icon tables return None for an IconName newer than the connector instead of failing to compile, with the exhaustive list kept by the test module"
```

---

### Task 2: Citations and claims

Two commits: a renumbering held to a mechanical gate, then the rewrites, which need reading.

**Files:** `src/*.rs`, `src/widgets/*.rs`, `tests/seams.rs`, `README.md`, `examples/showcase-gpui/**/*.rs`, `docs/showcase-exceptions.toml`, `ROADMAP.md`.

**Interfaces:** Consumes Task 1's build. Produces `every_colour_claim_is_read_at_the_line_it_cites` passing; every later task writes its new citations against 0.7.1 directly.

- [ ] **Step 1: Renumber.** Apply spec Appendix A, Appendix B's shifted entries, and the module-header stack names of spec §9.1 — `:<old>` → `:<new>`, digits only, by hand or with a throwaway script. The appendix line numbers are of `942302d6`; Task 1 moved a few lines in `src/icons.rs`, `README.md`, `support.rs` and `info/icons.rs`, so locate by content. Do not touch the sites of spec §9.2 and §9.3.

- [ ] **Step 2: Gate — only digits changed.**

Run `cargo fmt -p native-theme-gpui` first, then:

```bash
bash <<'EOF'
paths=(connectors/native-theme-gpui docs/showcase-exceptions.toml)
side() { git diff -U0 -- "${paths[@]}" | grep -E "^$1" | grep -vE '^(---|\+\+\+) (a/|b/|/dev/null)' \
         | cut -c2- | sed -E 's/[0-9]+//g' | sort; }
diff <(side -) <(side '\+') && echo DIGITS-ONLY
EOF
```

Expected: `DIGITS-ONLY` (the removed and added lines are the same multiset once digits are stripped). If rustfmt reflowed a line whose citation grew by a digit, the diff shows exactly that line; check it by eye and say so in the commit message. Anything else: stop and report.

- [ ] **Step 3: Commit the renumbering.**

```bash
git add connectors/native-theme-gpui/src connectors/native-theme-gpui/tests connectors/native-theme-gpui/examples connectors/native-theme-gpui/README.md docs/showcase-exceptions.toml
git commit -m "docs(gpui): the upstream citations that only moved between gpui-component 0.7.0 and 0.7.1 are renumbered (436 in the connector, 33 in the showcase exceptions); nothing but line numbers and the stack's version changes"
```

- [ ] **Step 4: Changed code.** Rewrite the sites of spec §9.2 to what the 0.7.1 code does — the focus-ring branches of `info/layout.rs`, the Shimmer dark claim, the checkbox / radio / switch ranges, the Questionnaire exception's citations. Leave `info/leaves.rs:354` and `src/geometry.rs:193` for Task 3.
- [ ] **Step 5: Stale before 0.7.1.** Spec §9.3, each item; for `src/lib.rs:483` fetch gpui-pre-macos 0.3.8 or write "read at 0.3.7" — never guess.
- [ ] **Step 6: Prose that names the stack.** Spec §9.4, including `ROADMAP.md:30, :88, :98`: verified-stack sentences re-verified and moved to 0.7.1 / 0.3.8; history sentences kept; registry-directory paths renamed.
- [ ] **Step 7: Charts.** Spec §8: the draw-in line in the shared `hover` note.
- [ ] **Step 8: Gates.**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --lib every_colour_claim_is_read_at_the_line_it_cites`
Expected: PASS (was 163 failures).

Run: `grep -rn "0\.7\.0\|0\.3\.7" connectors/native-theme-gpui/src connectors/native-theme-gpui/tests connectors/native-theme-gpui/examples connectors/native-theme-gpui/README.md connectors/native-theme-gpui/Cargo.toml ROADMAP.md`
Expected: only the history sentences of spec §9.4 and the README's Verified line (Task 9 writes it). List every remaining hit in the commit message body.

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --no-fail-fast 2>&1 | grep -E "FAILED|test result"`
Expected: only `a_page_sample_icon_the_set_lacks_is_absent` fails.

- [ ] **Step 9: Commit the rewrites.**

```bash
cargo fmt -p native-theme-gpui
git add connectors/native-theme-gpui/src connectors/native-theme-gpui/tests connectors/native-theme-gpui/examples connectors/native-theme-gpui/README.md docs/showcase-exceptions.toml ROADMAP.md
git commit -m "docs(gpui): the claims gpui-component 0.7.1 changed are rewritten to what its code does -- the focus line of a borderless element, the control text ladder, the Shimmer sweep, the chart draw-in -- with the citations found already stale at 0.7.0"
```

---

### Task 3: The Button label

**Files:**
- Modify: `src/geometry.rs` (`button_label`, `button`'s comment, tests), `tests/seams.rs`
- Modify: `examples/showcase-gpui/demo.rs` (`labelled`; `listed_label` and `tool_label` removed; the Button label sites), `support.rs:184-188`, `tests.rs:3724` (comment), `info/buttons.rs:512, :535`, `info/leaves.rs:354`

**Interfaces:**
- Consumes: `with_text`, `relative` (in `geometry.rs`); the showcase's `native_geometry`, `refined`, `elements::label_of`, `elements::record`.
- Produces: `pub fn button_label(n: Native<'_>) -> StyleRefinement` in `native_theme_gpui::geometry`; the showcase's `fn labelled(ui: &Entity<InfoRegistry>, cx: &App, button: Button, id: &str, label: impl Into<SharedString>) -> Button`.

- [ ] **Step 1: Failing unit tests** in `src/geometry.rs`'s test module, on its helpers `for_each_case` (every case × factors 1.0, 1.5, 0.8), `assert_text`, `resolved`, `Native::unscaled`, `abs`:

```rust
    #[test]
    fn button_label_carries_the_button_font() {
        for_each_case(|r, s, n| {
            let out = button_label(n);
            assert_text(&out, &r.button.font, s);
            assert_eq!(out.text.line_height, Some(relative(r.defaults.line_height)));
            assert_eq!(out.text.color, None, "the label takes the button's state colours");
        });
    }

    #[test]
    fn button_label_follows_the_button_font_not_the_body_font() {
        let mut r = resolved("kde-breeze", ColorMode::Light);
        r.button.font.size = r.defaults.font.size + 3.0;
        let out = button_label(Native::unscaled(&r));
        assert_eq!(out.text.font_size, abs(r.button.font.size));
    }
```

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --lib button_label`
Expected: FAIL — `cannot find function button_label`.

- [ ] **Step 2: Implement** spec §4.1 (`button_label`, and the sentence and citation in `button`'s comment).

Run the same command. Expected: 2 passed.

- [ ] **Step 3: Seam test** in `tests/seams.rs`, in the file's form (`Build` functions, `text_probe`, `laid_out_as`, `scaled_by`, `assert_seam`): under every native preset at its DPI, at factors 1.0 and 1.5 — `u` = the width of the text probe as a Medium Button's child with no style, `s` = the same with `geometry::button_label` on the probe, `e` = the probe with `geometry::button_label` in a bare `div()`. `assert_seam(u, s, e, …)` requires `u ≠ e`, so the test cannot pass vacuously.

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --test seams`
Expected: all pass (13 before, 14 now).

- [ ] **Step 4: Showcase.** Spec §4.2: `labelled` takes `cx` and builds the child label with `geometry::button_label`, absorbing `listed_label`; `tool_label` goes and its three callers call `labelled`; every `Button` the showcase labels goes through `labelled`, except `sized_button`; the sentence in `support.rs`.

Run: `grep -n '\.label(' connectors/native-theme-gpui/examples/showcase-gpui/*.rs connectors/native-theme-gpui/examples/showcase-gpui/pages/*.rs`
Expected: no `Button` among the hits but `sized_button`'s and `labelled`'s own fallback (the others are `Tab`, `Toggle`, `Checkbox`, `Radio`, `Switch`, `InputGroupButton`, `CommandGroup`, `Separator`, form fields).

- [ ] **Step 5: Widget Info** — spec §4.3 (`info/leaves.rs:354`, `info/buttons.rs:512, :535`).

- [ ] **Step 6: Suite.**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --no-fail-fast 2>&1 | grep -E "FAILED|test result"`
Expected: no FAILED — `a_page_sample_icon_the_set_lacks_is_absent` passes again, unedited.

- [ ] **Step 7: Layout gate.** Rebuild the showcase and take the after-dumps:

```bash
PHASE=after bash <<'EOF'
set -e
out="target/layout-071/$PHASE/rest"
mkdir -p "$out"
CARGO_BUILD_JOBS=4 cargo build -p native-theme-gpui --example showcase-gpui --locked
bin=target/debug/examples/showcase-gpui
for row in "kde-breeze dark freedesktop breeze-dark" "kde-breeze light freedesktop breeze" \
           "material dark material -" "material light material -" \
           "catppuccin-mocha dark lucide -" "catppuccin-mocha light lucide -"; do
  set -- $row
  extra=(); [ "$4" != "-" ] && extra=(--icon-theme "$4")
  "$bin" --theme "$1" --variant "$2" --icon-set "$3" "${extra[@]}" --tab basic \
    --dump-layout "$out/gpui-$1-$2.json"
done
EOF
python3 -I target/layout-071/compare.py
```

Expected: every file "0 differ", exit 0. (Before this task: 6 / 10 / 12 button and toggle-button elements differ.) A difference is a finding: report it; do not adjust the geometry to make it go away.

- [ ] **Step 8: Commit.**

```bash
cargo fmt -p native-theme-gpui
git add connectors/native-theme-gpui/src/geometry.rs connectors/native-theme-gpui/tests/seams.rs connectors/native-theme-gpui/examples/showcase-gpui
git commit -m "feat(gpui): geometry::button_label gives a Button's child label the platform's button font, which gpui-component 0.7.1 shrank to 0.875 of it at Size::Medium on a .label(); the showcase labels every native Button that way and its Basic page lays out as on 0.7.0"
```

---

### Task 4: Toggle

**Files:** `src/geometry.rs`, `tests/seams.rs`, `examples/showcase-gpui/demo.rs` (`toggle`, `toggle_group`), `examples/showcase-gpui/info/buttons.rs` (`toggle_notes`).

**Interfaces:**
- Consumes: `pub fn button_label(n: Native<'_>) -> StyleRefinement` (Task 3).
- Produces: `pub fn toggle(n: Native<'_>) -> StyleRefinement`.

- [ ] **Step 1: Failing unit test** (same module and helpers as Task 3):

```rust
    #[test]
    fn toggle_is_the_button_labels_text() {
        for_each_case(|_, _, n| {
            let out = toggle(n);
            assert_eq!(out, button_label(n));
            assert_eq!(out.size.height, None);
            assert_eq!(out.padding, Default::default());
            assert_eq!(out.corner_radii, Default::default());
            assert_eq!(out.border_widths, Default::default());
        });
    }
```

(If `StyleRefinement` is not `PartialEq`, compare the `text` fields. Field names per gpui-pre 0.3.8; keep each assertion's meaning.)

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --lib toggle_is`
Expected: FAIL — `cannot find function toggle`.

- [ ] **Step 2: Implement** spec §5.1. Run again: PASS.

- [ ] **Step 3: Seam test** — spec §5.3, as `assert_seam(u, s, e, …)`: `u` = the probe's width as an unrefined Medium `Toggle`'s child (0.7.1's `text_sm`), `s` = with the Toggle refined by `geometry::toggle`, `e` = the probe refined by `geometry::toggle` in a bare `div()`.

- [ ] **Step 4: Showcase** — spec §5.2: both Toggle sites take the builder; `toggle_notes` (and `info::buttons::toggle`, `toggle_group`) take `styled: bool` and are rewritten (the segmented-control sentence goes).

- [ ] **Step 5: Gates.**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --no-fail-fast 2>&1 | grep -E "FAILED|test result"`
Expected: no FAILED.

- [ ] **Step 6: Commit.**

```bash
cargo fmt -p native-theme-gpui
git add connectors/native-theme-gpui/src/geometry.rs connectors/native-theme-gpui/tests/seams.rs connectors/native-theme-gpui/examples/showcase-gpui
git commit -m "feat(gpui): geometry::toggle gives a Toggle the platform's button font, over the text_sm gpui-component 0.7.1 sets on a Medium Toggle"
```

---

### Task 5: DataTable

**Files:** `src/geometry.rs`, `tests/seams.rs`, `examples/showcase-gpui/demo.rs` (`data_table`, `:4021-4041`), `examples/showcase-gpui/info/data.rs` (`data_table`, `:51-75`).

**Interfaces:** Produces `pub fn data_table_size(n: Native<'_>) -> Size`.

- [ ] **Step 1: Failing unit test:**

```rust
    #[test]
    fn data_table_size_falls_back_to_upstream_medium() {
        let mut stated = 0;
        let mut unstated = 0;
        for_each_case(|r, _, n| {
            let expected = match r.list.row_height {
                Some(h) => {
                    stated += 1;
                    px(h)
                }
                None => {
                    unstated += 1;
                    Size::Medium.table_row_height()
                }
            };
            assert!(matches!(data_table_size(n), Size::Size(h) if h == expected));
        });
        assert!(stated > 0 && unstated > 0, "both branches must be exercised");
    }
```

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --lib data_table_size`
Expected: FAIL — `cannot find function data_table_size`.

- [ ] **Step 2: Implement** spec §6.1. Run again: PASS. (If the last assertion fails because no case of the module's `CASES` states a row height, add one that does to this test's own loop — do not weaken the assertion.)

- [ ] **Step 3: Seam test** — spec §6.3: a one-column, one-row `TableDelegate` whose cell is the text probe, in a fixed-size container refined by `geometry::table`; `u` = at `Size::Medium` (0.7.1's `text_sm`), `s` = at `geometry::data_table_size`, `e` = the probe refined by `geometry::table` in a bare `div()`; `assert_seam` on the probe's width.

- [ ] **Step 4: Showcase** — spec §6.2: `with_size`, `geometry::table` on the container, and the three info lines (row height, text, keyboard focus) in place of the "nothing applies it here yet" note.

- [ ] **Step 5: Gates.**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --no-fail-fast 2>&1 | grep -E "FAILED|test result"`
Expected: no FAILED.

- [ ] **Step 6: Commit.**

```bash
cargo fmt -p native-theme-gpui
git add connectors/native-theme-gpui/src/geometry.rs connectors/native-theme-gpui/tests/seams.rs connectors/native-theme-gpui/examples/showcase-gpui
git commit -m "feat(gpui): geometry::data_table_size sizes a DataTable as Size::Size from list.row_height, or upstream's Medium row where none is stated, so its cells inherit the platform's list font instead of 0.7.1's text_sm"
```

---

### Task 6: ColorSelect in the showcase

**Files:** `examples/showcase-gpui/{app.rs,demo.rs,pages/inputs.rs,info/inputs.rs,tests.rs}`.

**Interfaces:** Produces `demo::color_select(ui: &Entity<InfoRegistry>, cx: &App, id: &'static str, state: &Entity<ColorPickerState>) -> Stateful<Div>`, `info::inputs::color_select(t: &Theme, picked: bool) -> WidgetInfo`, `Showcase::color_select_state: Entity<ColorPickerState>` (no default value: the field starts empty).

- [ ] **Step 1: Failing test.** In `a_widgets_own_icons_are_named_gpui_components` (`tests.rs:6191`) add `("ColorSelect", inputs::color_select(&t, false))` to the list and "a ColorSelect's caret" to the doc comment.

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --example showcase-gpui a_widgets_own_icons 2>&1 | grep -E "^error|cannot find"`
Expected: `cannot find function color_select`.

- [ ] **Step 2: Implement** spec §7.1: state, demo function, page section, Widget Info (colour claims at their lines, the "frame" and "own icons" notes).

- [ ] **Step 3: Gates.**

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --no-fail-fast 2>&1 | grep -E "FAILED|test result"`
Expected: no FAILED (the colour-claim gate reads the new claims at their lines; the own-icons test passes).

Run: `python3 scripts/check_widget_coverage.py`
Expected: ColorSelect no longer listed; SpeechButton and SpeechWaveform still are (Task 7).

- [ ] **Step 4: Commit.**

```bash
cargo fmt -p native-theme-gpui
git add connectors/native-theme-gpui/examples/showcase-gpui
git commit -m "feat(showcase-gpui): the Inputs page shows ColorSelect, with the colours its frame takes from the theme, the caret it draws as gpui-component's own, and why no geometry builder reaches the frame"
```

---

### Task 7: SpeechWaveform, SpeechButton exception

**Files:**
- Create: `examples/showcase-gpui/speech.rs`
- Modify: `examples/showcase-gpui/{main.rs,app.rs,demo.rs,pages/inputs.rs,info/inputs.rs,tests.rs}`, `docs/showcase-exceptions.toml`

**Interfaces:**
- Consumes: the showcase's `labelled` (Task 3).
- Produces: `speech::SyntheticAudio { pub(crate) blocks: Rc<Cell<usize>> }` (`Clone + Default`, `impl AudioInput`), `speech::SilentRecognizer` (`impl SpeechRecognizer`), `Showcase::speech_state: Entity<SpeechState>`, `info::inputs::speech_waveform(t: &Theme) -> WidgetInfo`.

- [ ] **Step 1: Failing test** in `tests.rs` — the test of spec §11 (`the_synthetic_input_feeds_levels_and_stops_with_the_session`), copied whole.

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --example showcase-gpui the_synthetic_input 2>&1 | grep -E "^error|unresolved|cannot find"`
Expected: `unresolved import crate::speech`.

- [ ] **Step 2: Implement** `speech.rs` (spec §7.2, adapted to the compiler) and `mod speech;` in `main.rs`.

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --example showcase-gpui the_synthetic_input`
Expected: PASS.

- [ ] **Step 3: Page.** Spec §7.2: `Showcase::speech_state` (no subscription: gpui redraws a window whose draw read a notifying entity), the heading, the Listen / Stop Button (through `labelled`), the `SpeechWaveform`, and `info::inputs::speech_waveform`.

- [ ] **Step 4: Exception** — spec §7.3, in `docs/showcase-exceptions.toml` beside `SidebarToggleButton`. The showcase's sources name `SpeechButton` nowhere.

- [ ] **Step 5: Gates.**

Run: `python3 scripts/check_widget_coverage.py`
Expected: "Every widget is shown or excepted."

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --all-features --no-fail-fast 2>&1 | grep -E "FAILED|test result"`
Expected: no FAILED.

Run: `CARGO_BUILD_JOBS=4 cargo clippy -p native-theme-gpui --all-targets --all-features -- -D warnings 2>&1 | tail -1`
Expected: `Finished`.

- [ ] **Step 6: Commit.**

```bash
cargo fmt -p native-theme-gpui
git add connectors/native-theme-gpui/examples/showcase-gpui docs/showcase-exceptions.toml
git commit -m "feat(showcase-gpui): the Inputs page shows SpeechWaveform, driven by a synthetic level envelope and a recognizer that hears nothing (no microphone, no speech feature); SpeechButton is excepted because it draws gpui-component's own Mic and Square with no setter"
```

---

### Task 8: Project documents and version

**Files:** `CHANGELOG.md`, `ROADMAP.md`, `docs/todo.md`, `docs/todo_gpui-full-theme.md`, `README.md` (connector), `Cargo.toml`, `native-theme/Cargo.toml`, `connectors/native-theme-egui-widgets/Cargo.toml`, `Cargo.lock`.

- [ ] **Step 1: CHANGELOG** — spec §12.1. Undated `## [Unreleased]`, no compare link.
- [ ] **Step 2: `docs/todo.md`** — spec §12.2. Read each item first; tick, append or add — never rewrite an existing item's text.
- [ ] **Step 3: ROADMAP and the gap table** — spec §12.3: the delivered v0.6.1 entry (linking the spec at its `docs/archive/` path), the two planned milestones renumbered to v0.6.2 and v0.6.3, the sharpened and the two added upstream bullets, and the note in `docs/todo_gpui-full-theme.md`.
- [ ] **Step 4: README** — spec §12.4: the geometry table (`button` row, three new rows) and the example at `:194`. Not the Verified line (Task 9).
- [ ] **Step 5: Version.** `0.6.0` → `0.6.1` at `Cargo.toml:14` and `:24`, `native-theme/Cargo.toml:53`, `connectors/native-theme-egui-widgets/Cargo.toml:38`; then `CARGO_BUILD_JOBS=4 cargo check --workspace` so `Cargo.lock` records it.

Run: `grep -n '0\.6\.0' Cargo.toml */Cargo.toml connectors/*/Cargo.toml`
Expected: no output.

Run: `CARGO_BUILD_JOBS=4 cargo test -p native-theme-gpui --lib compat`
Expected: pass (the README still states the manifest's floors).

- [ ] **Step 6: Commit.**

```bash
git add CHANGELOG.md ROADMAP.md docs/todo.md docs/todo_gpui-full-theme.md connectors/native-theme-gpui/README.md Cargo.toml native-theme/Cargo.toml connectors/native-theme-egui-widgets/Cargo.toml Cargo.lock
git commit -m "chore: v0.6.1 version, changelog, roadmap, README and follow-ups for GPUI Kit 0.7.1"
```

---

### Task 9: Verification and hand-over (stops before any release action)

- [ ] **Step 1: Acceptance A1–A7, A10** (spec §13). Paste each result. For A7 re-run Task 3 Step 7's snippet: the later tasks must not have moved the Basic page.
- [ ] **Step 2: Compatibility stamp (A8).**

Run: `scripts/update_compatibility.sh run gpui && scripts/update_compatibility.sh check`
Expected: the gpui stamp names gpui-base, gpui-component, gpui-kit, gpui-kit-assets 0.7.1 and gpui-pre 0.3.8 (or a newer set, if upstream has released one and every gate passed on it); the README's Verified line follows. If `run` fails a gate, stop and report — never hand-edit the stamp. `check` may still report the egui stamp stale (the version bump touched `native-theme-egui-widgets/Cargo.toml`); the release asset run restamps it. Commit:

```bash
git add docs/COMPATIBILITY.toml connectors/native-theme-gpui/README.md
git commit -m "docs(gpui): compatibility stamp for GPUI Kit 0.7.1 and gpui-pre 0.3.8"
```

- [ ] **Step 3: Release check (A9).** `CARGO_BUILD_JOBS=4 ./scripts/check_release.sh`, then `cargo audit`. Expected: no failure; warnings only for the asset stamp (the connector's `src` and `examples` changed) and the egui compatibility stamp. `cargo audit` exits 0.

- [ ] **Step 4: Hand over.** Report test counts, every finding of Task 2 (claims that could not be confirmed), and the maintainer's steps, in order — **do not perform any of them**:
  1. run the showcase on the KDE desktop and look: Basic page buttons and toggle buttons (label size back), Buttons page Toggles, Data page table, Inputs page ColorSelect and the SpeechWaveform (Listen → bars → Stop), Charts page draw-in;
  2. decide whether to file the three upstream asks on `docs/todo.md` (spec §12.2), and whether the roadmap renumbering (v0.6.2, v0.6.3) stands;
  3. push `v0.6.1-rc1` and open a pull request: CI runs on pull requests, not on the branch, and the last release found its CI failures only after `main` had moved;
  4. `./scripts/generate_assets_release.sh` (screenshots, provenance, and all three compatibility stamps), commit;
  5. release commit `chore(release): v0.6.1` (date the CHANGELOG heading, add the compare link); `./scripts/check_release.sh` fully green on it;
  6. fast-forward `main`, push; CI green; dispatch the dependency canary once (it has failed nightly since 2026-10-05 and must pass now); explicit go; `git tag -a v0.6.1`, `git push origin v0.6.1`; crates.io workflow; docs.rs; GitHub release.

---

### Task 10: Archive the design documents

- [ ] **Step 1:**

```bash
git mv docs/todo_v0.6.1_gpui-kit-0.7.1-rationale.md docs/todo_v0.6.1_gpui-kit-0.7.1-spec.md docs/todo_v0.6.1_gpui-kit-0.7.1-plan.md docs/archive/
grep -rn 'todo_v0.6.1_gpui-kit-0.7.1' --include='*.md' --include='*.rs' --include='*.toml' --include='*.sh' . | grep -v '^./docs/archive/\|^./target/'
```

- [ ] **Step 2:** Rewrite prose citations only (ROADMAP's delivered entry already names the archive path; the grep also matches this step's own recorded commands inside the moved files — leave those). Links among the three moved files stay valid; links from them to `../` need one more `../` (`grep -n '](\.\./' docs/archive/todo_v0.6.1_*`).
- [ ] **Step 3:** Re-run the grep: every remaining hit is an `archive/` path or a recorded command. No hit may be under `connectors/native-theme-gpui/{src,examples,tests}` (Global Constraints); `scripts/update_compatibility.sh check` still reports the gpui connector verified. Commit by path: `git commit -m "docs: archive the implemented v0.6.1 gpui design documents"`.

---

## Execution notes (model routing under the maintainer's dispatch policy)

Tasks 1, 3, 4, 5 and 7 have mechanical gates (compile, named failing tests, the layout-dump diff, the coverage script) and complete code: suitable for the `implement` agent (Opus). So is Task 2 Steps 1–3 (the digits-only gate). Task 2 Steps 4–9 (does the changed code still support the claim?), Task 6 (a showcase panel, gated only by the colour-claim, own-icons and coverage checks) and Task 8 (documents) need judgment: orchestrator inline, or `implement` with the instruction to report rather than reword. Task 9 is orchestration and stops before any outward action. Never dispatch a third time after a subagent fails the same gate twice.

Ordering: 0 → 1 → 2 first (everything later cites 0.7.1 directly). Task 4 needs Task 3's `button_label`; Task 7 needs Task 3's `labelled`. Tasks 5 and 6 are independent of the others. No subset is releasable without Tasks 6 and 7: the widget-coverage gate fails until the three new widgets are shown or excepted.
