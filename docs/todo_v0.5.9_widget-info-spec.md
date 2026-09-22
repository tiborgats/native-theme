# v0.5.9 — Widget Info: Specification

Status: Design (2026-09-22, revised twice 2026-09-22); nothing implemented
Companion rationale:
[`todo_v0.5.9_widget-info-rationale.md`](todo_v0.5.9_widget-info-rationale.md)
(decisions W1–W9)
Companion plan:
[`todo_v0.5.9_widget-info-plan.md`](todo_v0.5.9_widget-info-plan.md)
Sibling work in the same release:
[`todo_v0.5.9_theme-contracts-spec.md`](todo_v0.5.9_theme-contracts-spec.md).
This specification assumes that work has landed: `geometry` has 37 public
items, `src/showcase.rs` holds the lexical helpers and four tests, and
`docs/showcase-exceptions.toml` exists.

---

## 0 -- Scope

### 0.1 What this delivers

1. Every demo block carries a Widget Info panel, and every widget the toolkit
   offers appears in one (§2).
2. A test asserts that the `geometry::` builders a demo block uses are
   exactly the ones its panel names (§3).
3. Every colour claim carries the upstream location it was read at, and a
   test checks the named field is there; prose claims that already cite a
   symbol have its existence checked (§4).
4. An advisory report lists what each panel omits (§5).
5. The false claims are corrected and the five missing panels written (§6).
6. `check-widget-coverage.py` stops accepting a bare path segment as proof
   that a gpui widget is shown (§7).

### 0.2 Constraints

- No panics, no `unsafe`, no invented values, no hardcoded theme values.
- Every number a test expects comes from the resolved theme, the vendored
  upstream source, or the value under test -- never a literal.
- Every gate ships a discrimination proof (W8): a seeded defect must make it
  fail, and the failure must name the line.
- **`native-theme-gpui` gains no public item.** `geometry` stays at 37. All
  new code is tests, a script, and showcase edits.

### 0.3 Out of scope

Deriving the geometry section from the applied refinement (rationale §3.3,
rejected as disproportionate); a theme-read extractor (rationale §3.5,
rejected); hashing cited upstream files (rationale §3.8 — reduced to the
existence check in §4.6, not dropped); the iced showcase; screenshot
diffing; a `Command` demo unless §6.4 resolves that way.

**The Theme config section is not gated, and that is a decision.** Its
entries are live values (`format!("radius: {}px", t.radius.as_f32())`) which
cannot drift, plus a few that restate an upstream computation — Radio's is
the live defect, printing `radius` where upstream uses `radius * 0.5`
(`radio.rs:215`). Those few are corrected in §6.1 and left to review
afterwards: building a third checker for a handful of derived numbers would
cost more than it protects. §5's report is where a regression here would
surface.

---

## 1 -- Facts this specification rests on

Verified 2026-09-22. `gpui-component` 0.6.4 and 0.6.6 differ only in
`inspector.rs` and `label.rs`; no line below differs between them.

| # | Fact | Read at |
|---|---|---|
| F1 | The panel is built by `widget_tooltip(name, colors, config, not_themeable)` | `showcase-gpui.rs:562-592` |
| F2 | Panels are attached with `.on_hover(self.hover_info(..))`; there are 102 | `showcase-gpui.rs:2709-2719` |
| F3 | A colour claim is a `(role, field, value)` triple; there are 324, and in 317 the field name equals the value expression | measured 2026-09-22 |
| F4 | `src/showcase.rs` is a private, test-only module that reads the showcase with `include_str!` | `src/showcase.rs:24-27` |
| F5 | It provides `public_fns`, `without_comments_or_strings` and `references(haystack, module, name)`, and removed spans keep their newlines so line numbers survive | `src/showcase.rs:33, 55-58, 59, 165` |
| F6 | Upstream reads theme colour as `cx.theme().<field>` or `cx.theme().tokens.<field>` | `button.rs:936-941, 993`; `toggle.rs:155`; `radio.rs:186-188` |
| F7 | `ThemeTokens` is a mechanical 1:1 projection of `ThemeColor`, so `tokens.x` and `x` are the same value | `theme/theme_color.rs:343-371` |
| F8 | `check-widget-coverage.py` matches a widget name "as an identifier or a path segment" | `scripts/check-widget-coverage.py`, Matching |
| F9 | Every one of the 102 panels hangs on its own `tt-` block: **101** as a literal `.id("tt-…")`, and one — the resizable groups — through `.id(group.id)` from the `RESIZABLE_GROUPS` table, whose ids are already `tt-resizable-h`/`tt-resizable-v` (`:459, :480`). Blocks and panels are 1:1 | measured 2026-09-22, then confirmed by the implemented test |
| F10 | The geometry prose that exists is accurate: `input`, `progress` and `tooltip` each match their panel | rationale §1.6 |

**F7 matters**: a panel naming `button_primary` and a widget reading
`tokens.button_primary` agree. §4 treats them as the same token.

**F10 matters**: it is why §3 is one test rather than a derivation project.

---

## 2 -- Layer 1: every demo has a panel

### 2.1 The block convention

A **demo block** runs from a `div().id("tt-<slug>")` to the next one, or to
end of file. By F9 this is already true of **all 102** panels, so no showcase
change is needed to establish the convention — but a block's id reaches
`.id()` in two ways, and the detector must see both:

- as a literal, `div().id("tt-toggle")` — 101 blocks;
- through a `const` table, `.id(group.id)` from `RESIZABLE_GROUPS`
  (`showcase-gpui.rs:5486`, ids at `:459, :480`) — 1 block.

The indirection is a hole unless it is guarded: renaming a table entry to
something that is not a demo would keep the marker and lose the meaning. So a
second test asserts that **every `id: "…"` field in the showcase starts with
`tt-`**. Without it, `demo_block_starts` could count a non-demo as a demo.

**W2**: a builder reaches a demo in one of two ways, and §3's attribution
rule reads both, so **no binding has to move and no exceptions table is
needed**:

```rust
// inside the block — the common case
ListItem::new(..).native(cx, geometry::list_item)

// bound once in the page's render fn, applied in several blocks
let button_style = native_geometry(cx, geometry::button);   // :2776
refined(Button::new("b-link").label("Link").link(), button_style.as_ref())
```

The second form is not rare — 7 of the 17 bindings measured on 2026-09-22 use
`native_geometry(cx, geometry::X)`, which a naive `= geometry::` search misses
entirely. Requiring those to move would mean either duplicating one
computation across ten Button blocks or maintaining a table of which block
uses which binding. §3 reads the binding instead.

A helper that merely applies an already-computed refinement (`refined` `:722`,
`with_gap` `:760`, `with_accordion_title_style` `:784`) needs no treatment: it
names no builder.

### 2.2 The presence test

```rust
/// Spec §2.2: every `tt-` demo block carries a Widget Info panel.
#[test]
fn every_demo_block_has_a_widget_info_panel() { /* … */ }

/// Spec §2.1: an id reaching a demo block through a `const` table is still
/// a `tt-` id.
#[test]
fn every_demo_id_is_a_tt_id() { /* … */ }
```

Block *boundaries* come from the raw `SHOWCASE`, because
`without_comments_or_strings` removes string literals and `.id("tt-` would
not survive it. The *code* check runs on the stripped copy, whose line
numbers still match (F5), so a panel named only inside a comment does not
count as one. A block with no `.on_hover(self.hover_info(` is reported with
its line.

### 2.3 Every widget appears in a panel's block

`check-widget-coverage.py` gains a second question. Today it asks "is this
widget shown?"; it will also ask "is it shown **inside a demo block**?". A
widget rendered only in the application's chrome -- the theme `Select`, the
info-panel `Textarea` -- fails, because a reader can never hover it.

Exceptions go in the existing `docs/showcase-exceptions.toml` under
`[gpui.chrome_only]`, each with a reason, on the terms the file already sets:
an exception is a claim that has to stay true.

---

## 3 -- Layer 2: builders used == builders named

One test, extending machinery that exists (F5).

```rust
/// Spec §3: the `geometry::` builders a demo block uses are exactly the
/// builders its panel names.
#[test]
fn every_demo_names_the_builders_it_applies() { /* … */ }
```

**The attribution rule.** A block *applies* builder `X` when either:

1. `geometry::X` appears in the block's **code** (comments and string
   literals stripped); or
2. a `let <var> = … geometry::X …` binding in the **enclosing `fn`** exists
   and the block references `<var>`.

Rule 2 is what makes W2 free. The enclosing function runs from the preceding
`    fn ` at four-space indent to the next one -- a lexical range, no parsing.
Measured on 2026-09-22 it attributes `geometry::button` to all ten
Button-variant blocks through `button_style` `:2776`, and
`geometry::accordion_title` to the Accordion block through
`accordion_title_style` `:5514`, with no code movement and no declarations.

A block *names* builder `X` when `geometry::X` appears in its **panel text**
-- the string literals, which is where the notes live. This is the opposite
direction from `the_showcase_exercises_every_builder`, which strips literals
because a note about a builder is not a use of it; here a note is exactly what
is being checked.

Require set equality. A builder applied but not named, or named but not
applied, is reported with its block and line.

This catches all seven omissions in rationale §1.5. It does **not** check
that a note describes its builder correctly; F10 measured that risk and found
it does not currently occur, and rationale §3.3 records the fallback if it
ever does.

**Discrimination proof (W8):** add `.native(cx, geometry::popover)` to a
block whose note omits it; the test must fail naming the line.

---

## 4 -- Layer 3: cite the line, and check the line

### 4.1 The shape

A colour claim gains a fourth element: the location it was read at.

```rust
// (role, field, value, cited-at)
("text", "link", t.link, "button.rs:993"),
```

A citation is `<file>:<line>` or `<file>:<from>-<to>`, relative to the
vendored crate's `src/`. A claim about a colour **this connector** produces
cites our own source instead -- Ghost cites `variants.rs:54` -- with the same
syntax and the same check.

**A line, not a symbol** -- although `file.rs, Symbol` is the convention the
prose already uses (§4.6). `ButtonVariant::text_color` (`button.rs:947-996`)
holds `Self::Link => cx.theme().link` at `:993` and
`Self::Text => cx.theme().foreground.opacity(0.9)` at `:994`. A symbol-scoped
check would find `foreground` inside that function and pass Button (Link)'s
false claim -- the very defect this layer exists to catch. Only line
granularity separates variants that share a function.

The panel renders the citation, because a reader who doubts a claim should be
able to check it in one step:

```
Theme colors:
  text: link #2a7ab0 (button.rs:993)
```

Whether that is useful or clutter on a widget with six claims is
rationale §6, question 2 — the maintainer's call. The check does not depend
on it; only the display does.

### 4.2 The test

```rust
/// Spec §4.2: every colour claim's cited line reads the field it names.
#[test]
fn every_colour_claim_is_read_at_the_line_it_cites() { /* … */ }
```

For each claim: resolve the file against the vendored crate's `src/` or this
connector's own source (§4.1), read the cited line or range, and require the
field name to appear in it as an identifier. `tokens.x` satisfies a claim of
`x` (F7).

**Why this is enough, and why nothing heavier is.** Rationale §3.5 measured
the alternatives: a file-level extractor and a call-graph extractor catch the
same defects and both miss Button (Link) and Button (Text), because
`button.rs:1284` reads `muted_foreground` for the *disabled* state. A
citation check catches them exactly, because `:993` reads `link` and not
`foreground`. It is both the cheaper design and the more precise one.

**What it does not do.** It validates the claims a panel makes; it says
nothing about a field the panel never mentions. That is §5's job, and it is
advisory by design.

### 4.3 Staleness, and the cost of line granularity

When upstream moves a line, the cited line stops reading its field and the
test fails, naming the claim. No hashes and no stamp file are needed for
colour claims.

The cost is drift: an insertion earlier in a file moves every later line, so
a claim can fail although nothing about it changed. Measured against the real
bump this is small -- 0.6.4 → 0.6.6 changed two files, and only claims citing
lines after the edit in those files would move.

**The mitigation is a requirement, not a hope.** The failure message must
print what the cited line *now* contains, so distinguishing "the field moved"
from "the claim was wrong" is a look rather than an investigation:

```
Button (Link): claims `link` at button.rs:993
  that line now reads: Self::Text => cx.theme().foreground.opacity(0.9),
```

### 4.4 Locating the crate source

The source directory comes from `cargo metadata --format-version 1` only,
never a registry path or a version literal, exactly as
`check-widget-coverage.py` does. A dependency missing from the metadata is a
hard error, never a skipped check. Because the check needs this at run time
and `src/showcase.rs` is `include_str!`-based, there are two forms: a build
script writes the path to `OUT_DIR` for a `#[test]`, or the check is a script.

**The criterion is not "simpler" but "as strong as its siblings".** The other
two gates run under `cargo test`, so they fail on a developer's machine and
in CI without anyone remembering to invoke them. The `#[test]` form is
therefore preferred. If the build script proves impractical, the script form
is acceptable **only** if it is wired into both `pre-release-check.sh` and
the CI workflow — otherwise this layer would be the one gate that a
contributor can skip. Record the choice, and the reason, in §9.

### 4.5 Discrimination proofs (W8)

1. Restore Toggle's `checked bg` to `secondary_active` with its citation; the
   cited line does not read it — must fail.
2. Change a correct claim's citation to a neighbouring line — must fail.
3. Cite a file that does not exist — must fail with that filename.

### 4.6 Prose claims: the cited symbol must still exist (W9)

§4.2 covers colour claims. It says nothing about the **227 "Not themeable"
entries**, which are prose -- "hardcoded ChevronDown", "2s by default", "the
compositor owns the frame". Dropping the staleness stamp entirely would have
left every one of them unprotected; rationale §3.8 records that mistake and
its correction.

**45 of the 227 already cite a source file.** For those, the check
verifies the file exists and the symbol is declared in it -- **existence
only, never semantics**. A citation resolves against the vendored crate's
`src/` or against this connector's own source, as §4.1 already allows for
colour claims: some prose entries cite `variants.rs` and `geometry.rs`, not
only upstream. A citation that is merely stale in wording is a
human's problem; a citation pointing at something upstream has *deleted* is
one a machine should catch, and this project has already been bitten by
exactly that: `ThemeColor::tiles` was removed in a **patch** release, which
is why the sibling gpui-kit-0.6.4 work exists.

Prose entries without a citation are **not** required to gain one. Many are
statements about an absence ("the model carries no circular-progress
diameter") with no symbol to point at, and forcing a citation would invite
invented ones -- the worst possible outcome under this project's first rule.

**Discrimination proof (W8):** point a prose citation at a symbol that does
not exist; the check must fail naming it.

### 4.7 What none of this covers

Stated plainly so the residual is known rather than assumed:

- The **semantic** content of prose. "2s by default" is checked by a human.
- The **role labels** on colour claims ("checked bg"). A claim can cite the
  right line for the right field under the wrong role (W5).
- The **Theme config** section (§0.3).

§5's report and the version-bump routine are the mitigation for all three.

---

## 5 -- Layer 4: the omission report

`scripts/panel-omissions.py`, advisory (**W6**), not wired to CI as a
failure.

For each panel, it lists theme fields read in the widget's own upstream file
that the panel never names. A human triages: some are states or variants the
demo does not show, some are real gaps.

It is a report and not a gate because making it one would need an exception
per unshown state across ~100 widgets -- exactly the sprawl this design
avoids. It runs during the §6 corrections and at every upstream bump, and
`pre-release-check.sh` prints its count as information, not as a failure.

---

## 6 -- The corrections

### 6.1 The seven false claims

Corrected to rationale §1.3, each with the citation §4.1 now requires. Radio
carries three corrections: the `bg` field, the indicator size, and two
omissions -- the label colour `foreground` (`radio.rs:212`) and the radius,
which is `radius * 0.5` (`:215`) where the panel prints `radius`.

### 6.2 The six Button variants

Renamed to the `button_*` family (rationale §1.4), each citing its line. The
hex does not change. Ghost is **not** renamed: it correctly names the colours
`variants::ghost_button` gives it, and cites `variants.rs:54`.

### 6.3 The seven omitted builders

Each panel gains the note §3 requires. §3 is what keeps them there.

### 6.4 The five missing panels

| Widget | Resolution |
|---|---|
| `Select` | A real demo beside `Combobox`, so the carried-colour difference (`geometry.rs:441-467`) is visible side by side |
| `Textarea` | A real demo in the Input section |
| Native Theme Icons grid | A panel naming the icon theme, the resolved icon size and the fallback rule |
| Animated Icons | A panel naming the frame source and the reduced-motion rule |
| `Command` | **Open question** (rationale §6, question 4). If deferred, a `[gpui.chrome_only]` exception and a follow-up in `docs/todo.md` |

### 6.5 The remaining panels

The ~40 not named above are **unverified, not known-bad**. The citation pass
(§4) reaches every one of them: adding a citation means reading the upstream
line, which is the audit. What it reports is corrected; what it passes is
left alone.

### 6.6 Landing this without a placeholder surviving

The citation pass is large (324 claims). It proceeds page by page, and each
page's commit is green because the test only checks claims that **have** a
citation:

- A claim with no citation is skipped by §4.2 and **counted**.
- During the pass the check *reports* that count; it does not fail on it, so
  every page's commit is green under the standing "run
  `./pre-release-check.sh` before committing" rule.
- The gate is flipped to **fail on a non-zero count** once the pass has
  finished and the count has actually reached zero.

The absence of a citation is itself the to-do marker, counted in plain sight
on every run -- so no placeholder reasons and no temporary exception entries
are needed anywhere. After the flip, a release with an uncited claim is
impossible.

---

## 7 -- The coverage-gate fix

`check-widget-coverage.py`'s gpui matching stops accepting a bare path
segment (F8). A gpui widget counts as shown when it appears as a constructor
or in a toolkit-rooted path:

- `W::new`, or `W::` inside a `gpui_component::` / `gpui_kit::` path, or
- `W` immediately followed by `{` or `(` in expression position, or
- a per-widget pattern for the eight widgets whose docstring already records
  that they are reached through an extension method (`ContextMenu`, `Dialog`,
  `Loading`, `Scrollable`, `Sheet`, `Tab`, `Text`, `WindowBorder`).

**Discrimination proof (W8):** with the fix, `std::process::Command::new`
(`showcase-gpui.rs:8460`) no longer satisfies the `Command` widget, and the
gate reports it missing until §6.4 resolves.

---

## 8 -- Acceptance

1. `./pre-release-check.sh` green, with the new checks counted in its header.
2. `every_demo_block_has_a_widget_info_panel` passes.
3. `every_demo_names_the_builders_it_applies` passes.
4. `every_colour_claim_is_read_at_the_line_it_cites` passes, and the
   **uncited-claim count is zero** (§6.6).
4a. Every prose citation resolves to an existing file and symbol (§4.6).
5. `check-widget-coverage.py` exits 0 — but only after it has first been
   *seen* reporting `Command` missing. That observation is both the
   discrimination proof for §7 and the evidence that §6.4's `Command` row is
   a real gap rather than an assumption, so the tightening and the
   resolution belong to one step.
6. All discrimination proofs fire and name their line: §3, §4.5 (three), §7.
7. `scripts/panel-omissions.py` runs and its count is recorded in the
   CHANGELOG entry, so the residual is a number someone chose, not a number
   nobody looked at.
8. The showcase runs with `XDG_CONFIG_HOME` pointing at an empty directory
   and no panel shows a stale geometry value.
9. CHANGELOG records the corrected claims under `Fixed`, and states that no
   public item was added.

---

## 9 -- As built

Every place the implementation corrected this document.

**§2.1, F9 — the block convention was already complete, and F9 was wrong.**
It recorded Resizable as the one panel without a `tt-` id. It has two:
`RESIZABLE_GROUPS` holds `tt-resizable-h` and `tt-resizable-v`
(`showcase-gpui.rs:459, :480`) and reaches `.id()` as `.id(group.id)`
(`:5486`), which a literal-only detector cannot see. Blocks and panels are
**102 to 102**, not 101 to 102. The detector gained that second marker, and
`every_demo_id_is_a_tt_id` guards it: an id reaching a block through a
`const` must still start with `tt-`, or the marker could come to stand for a
non-demo.

**§2.1 — bindings do not move, and no exceptions file was created.** The
first draft had page-level bindings move into their block or be declared in
`docs/widget-info-exceptions.toml`. Measured, that was wrong work:
`button_style` (`:2776`) serves all ten Button demos and `widget_gap` is
bound separately in six page methods, so moving meant either duplicating a
refinement ten times or maintaining a block-to-binding table. §3's
attribution rule reads the binding instead. **No exceptions file exists.**

**§3 — three corrections, each measured.** A block ends at its own panel's
call, not at the next id: the panel closes the chain, and the next-id rule
let the last block of a method swallow the rest of it, crediting `tt-tabbar`
with the `scrollbar_gutter` on the content scroller beside it (`:8186`,
outside the tab bar's div which closes at `:8175`). A binding inside a block
belongs to that block, because generic names collide — `let content =
native_geometry(cx, geometry::tooltip_content)` otherwise credited every
block mentioning a `content`. And five **ambient** builders are exempt
(`button` and the four `LayoutTheme` spacing helpers): they shape a demo's
scaffolding, not the widget it demonstrates, and each has a panel where it is
the subject. Without these the check reported 38 blocks; with them, exactly
the 7 real omissions.

**§3 — the check is one-directional.** This document required set equality.
That would have deleted `PopupMenu`'s note that `geometry::menu_item` has no
receiver there, and `AlertDialog`'s pointer to the Dialog above. Naming a
builder a demo does *not* apply is how a panel says why it could not, so only
unnamed **application** is a defect.

**§4.4 — the citation check is a script, not a `#[test]`.** The `#[test]`
form needs the vendored source path at run time, which means `cargo metadata`
inside a `build.rs`. `native-theme-gpui` has none, and adding one to a
published crate solely to locate a *dev*-dependency's source would run for
every consumer, for a check that only ever concerns this repository;
`cargo metadata` inside a build script is also recursion-prone. The condition
this document set is met instead: the script is wired into **both**
`pre-release-check.sh` and `.github/workflows/ci.yml`, which already runs
`check-widget-coverage.py` the same way (`ci.yml:121`).

**§6.1 — NumberInput's size claims were corrected in Task 2, not Task 4.**
Adding its `geometry::input` note left "height: set per Size enum" directly
contradicting the line above it. `Size` sets the step buttons' `min_w`
(`number_input.rs:148-150, 179-181`); the field's height comes from
`input.min_height`.

*(Filled in by the implementation, as
[`todo_v0.5.9_theme-contracts-spec.md`](todo_v0.5.9_theme-contracts-spec.md)
§8a is. §4.4's choice is recorded here.)*
