# v0.5.9 — Widget Info: what the panel claims, and how it stays true: Rationale

Status: Design (2026-09-22, revised twice 2026-09-22); nothing implemented
Companion specification:
[`todo_v0.5.9_widget-info-spec.md`](todo_v0.5.9_widget-info-spec.md)
(decisions W1–W9)
Companion plan:
[`todo_v0.5.9_widget-info-plan.md`](todo_v0.5.9_widget-info-plan.md)
Sibling work in the same release:
[`todo_v0.5.9_theme-contracts-spec.md`](todo_v0.5.9_theme-contracts-spec.md) and
[`todo_v0.5.9_gpui-kit-0.6.4-spec.md`](todo_v0.5.9_gpui-kit-0.6.4-spec.md).
This work assumes both have landed.

---

## 0 -- What this document is for

The gpui showcase's **Widget Info** panel tells the reader which theme field
each widget reads, which configuration values it takes, and what about it no
theme can reach. It is the only place in the project where a human can see a
mapping claim and the pixel it produces at the same time.

An audit on 2026-09-22 read all 102 panels against the vendored
`gpui-component` source. Seven panels state something untrue, seven omit a
`geometry::` builder the widget they describe actually receives, and five
widgets have no panel at all.

Everything below was verified against **both** `gpui-component 0.6.4` (the
floor the manifests declare and `Cargo.lock` pins) and **0.6.6** (what a
consumer's fresh `cargo build` resolves). Only two files differ between those
releases -- `inspector.rs` and `label.rs` -- and the `label.rs` change is a
masked-highlight fix that touches no claim. Every line number below is
correct in both.

---

## 1 -- The evidence

### 1.1 The panel has three sections, and they fail differently

`widget_tooltip` (`examples/showcase-gpui.rs:562-592`) renders four blocks
from the arguments `hover_info` (`:2709-2719`) is given: **Theme colors**
(role, field name, live `Hsla`), **Theme config** (what, live value), **Not
themeable** (what, why) and the theme's fonts.

The live values cannot drift -- they are read from the installed theme at
hover time. What drifts is everything *around* them: which field a widget
reads, which builder shaped it, and what upstream hardcodes.

### 1.2 Two generations of panel

Of the 102 panels, **48** cite a `geometry::` builder or an upstream
`file.rs, Symbol`; **54** cite nothing at all. The split is not random: the
48 were written or rewritten by the theme-contracts work, the 54 predate it.

**All seven false claims are in the 54. Two of the seven omitted builders are
not** -- InputGroup and Tree both cite a builder and still miss a second one
(§1.5). That matters for the argument: carefulness at writing time is not the
variable. A panel written by someone with the builder list in front of them
still lost a builder added later.

The 48 hold up under spot checks. Each of these was recomputed from the
vendored source and matched exactly:

| Claim | Verified at |
|---|---|
| `TITLE_BAR_HEIGHT = 34px` | `title_bar.rs:15` — `px(34.)` |
| Sidebar `255px` / `48px collapsed` | `sidebar/mod.rs:27-28` |
| `PANEL_MIN_SIZE = 100px` | gpui-base `resizable/mod.rs:14` |
| HoverCard `600ms` open / `300ms` close | `hover_card.rs:81, 87` |
| ProgressCircle stroke `15%`, capped `5px` | `progress_circle.rs:85` |
| Bubble `max_w(relative(0.8))`, `px_3`/`py_2` | `bubble.rs:134, 201-202` |
| Message `rems(0.625)`, `min_w_8`, `self_end` | `message.rs:161, 261, 265` |
| Separator `1px` | `separator.rs:81-82` |
| Icon `101 built-in Lucide icons` | generated `icon_name.rs`, 101 entries |

### 1.3 The seven false claims

| Panel | States | `gpui-component` 0.6.4 = 0.6.6 |
|---|---|---|
| Toggle / ToggleGroup `:3266` | checked bg `secondary_active`; checked text `secondary_foreground`; unchecked bg `secondary`; hover `secondary_hover` | `tokens.accent` `toggle.rs:155`; `accent_foreground` `:156`; no fill (only `ToggleVariant::Outline` sets `border` `:197` and `tokens.background` `:198`); `tokens.accent` `:202-203` |
| Button (Link) `:2983` | text `foreground`; hover-text `muted_foreground` | `link` `button.rs:993`; `link_hover` `:1139`. Pressed `link_active` `:1215, 1257` and the underline `:1045` are unlisted |
| Button (Text) `:3004` | text `foreground`; hover-text `muted_foreground` | `foreground.opacity(0.9)` `button.rs:994`; `foreground` `:1140`. Pressed `foreground.opacity(0.7)` `:1216` unlisted |
| Radio `:3604` | bg `background`; "indicator size: hardcoded" | `input.opacity(0.5)` `radio.rs:188`; `rems()` per `Size` `:168-173`, `Radio::with_size` `:127`. Also unlisted: the label colour `foreground` `:212`, and the radius is `radius * 0.5` `:215` where the panel prints `radius` |
| NumberInput `:3487` | "height: set per Size enum" | it applies `geometry::input` at `:3482`, whose height comes from `input.min_height` |
| AlertDialog `:6677` | geometry "as the Dialog above" | the Dialog panel names no geometry at all |
| Switch `:3645` | "size: hardcoded" | `(28×16)` / `(12)` per `Size` `switch.rs:150-155`; `Switch::with_size` `:114` |

Toggle is the worst: **all four** colour claims wrong, "Not themeable" empty.
It is also the one the connector already knew about -- `colors.rs:258`
documents in prose that `Toggle` reads the `accent` token for its pressed
state, and calls the conflict Tier U. The mapping knew; the panel said the
opposite.

### 1.4 The six solid Button variants name the wrong family

Primary, Secondary, Danger, Success, Warning and Info each name `primary` and
its siblings. `Button` reads the **button** family: `tokens.button_primary`
`button.rs:936-941`, `button_primary_foreground` `:954`,
`tokens.button_primary_hover` `:1086-1121`, `tokens.button_primary_active`
`:1170-1206`.

Because `colors.rs:362-365` assigns `tc.button_primary = tc.primary` and its
siblings, **the hex the panel shows is right**. Only the field name is wrong.

Ghost names `secondary_*` and is **correct**, because the showcase builds it
with `variants::ghost_button` (`variants.rs:51-57`), not upstream's
`.ghost()`. That is the one place where naming upstream's field would be the
error.

### 1.5 Seven panels omit a builder their widget receives

| Panel | Applies | Names | Generation |
|---|---|---|---|
| Dialog `:6601` | `dialog` `:6594`, `dialog_title` `:6551`, `dialog_description` `:6576`, `dialog_footer` `:6585`, `dialog_max_width` `:6595` | none | uncited |
| Popover `:6771` | `popover` `:6752` | none | uncited |
| NumberInput `:3487` | `input` `:3482` | none | uncited |
| Form / Field `:6288` | `input` (×2) | none | uncited |
| Sidebar `:6391` | `icon_size_panel` (×4) | none | uncited |
| InputGroup `:3458` | `input_group_button` `:3416` | `input` only | **cited** |
| Tree `:4121` | `list_item` `:4116` | `list` only | **cited** |

Tree's is the sharpest: `geometry::list_item` is one of the seven builders
the theme-contracts work gave a carried text **colour**
(`geometry.rs:179-189`) -- precisely what that release is about.

### 1.6 The geometry prose that *is* written is accurate

This was measured, because it decides how much machinery §3 needs. Three
builders were read against the panels that describe them:

| Builder | Sets | Panel says | Verdict |
|---|---|---|---|
| `input` `geometry.rs:138-148` | `h`, `rounded`, `border`, text | "`input.min_height` (control height), `border.corner_radius`, `line_width`, `input.font`" | accurate |
| `progress` `:369-376` | `h`, `rounded`, `min_w` | "`progress_bar.track_height`, `border.corner_radius`, `min_width`" | accurate |
| `tooltip` `:205-216` | `px`, `py`, `rounded`, coloured text | "`border.padding_*`, `corner_radius`, `tooltip.font` — including its colour" | accurate |

**The failure mode is omission, not misdescription.** A panel that names its
builder describes it correctly; the seven in §1.5 simply do not name it. This
is why §3 is one lexical test and not a rendering-derivation project (§3.3).

### 1.7 The module that reads the showcase already assumes the invariant

`src/showcase.rs:16-18` explains why it strips string literals before looking
for builder uses:

> the showcase names builders in both -- every widget's hover note says which
> builder shaped it -- and a note about a builder is not a use of it.

"Every widget's hover note says which builder shaped it" is stated there as a
fact. In seven panels it is not one. The invariant was written down and never
given a test.

### 1.8 Five widgets have no panel

| Widget | Where it is | Consequence |
|---|---|---|
| `Command` | **nowhere** | A public command-palette widget nobody has seen under a native theme |
| `Select` | app chrome only `:8090, :8096, :8109` | `geometry::select` -- whose carried colour is the documented difference from `geometry::combobox` -- is never demonstrated |
| `Textarea` | the Widget Info panel itself `:1384` | — |
| Native Theme Icons grid | `div().id("native-icons-grid")`, no `on_hover` | The connector's own icon feature has no panel |
| Animated Icons | `render_animated_icons_section` | — |

`Command`'s absence also exposes a gate defect.
`scripts/check-widget-coverage.py` reports it as shown, because its matching
rule accepts a name "as an identifier or a path segment" and
`std::process::Command::new` at `:8460` is one. The gate is satisfied by a
name from the standard library.

### 1.9 What is already sound

Across 324 colour triples, the printed field name matches the value
expression in **317**; the other 7 are deliberate prose labels. There is no
copy-paste class of defect. And the block convention is already complete:
all **102** panels hang on their own `tt-` block -- 101 as a literal
`.id("tt-…")`, and the resizable groups through `.id(group.id)` from a const
table whose ids are already `tt-`-prefixed (`showcase-gpui.rs:5486, :459`).
Blocks and panels are 1:1, so §2 costs no showcase edit at all.

---

## 2 -- Why hand-written panels drift

Three distinct things are written in a panel, with different failure modes:

1. **Which builder shaped this widget.** The answer is in the same file, a
   few lines above. It drifts because adding `.native(cx, geometry::x)` and
   updating the prose are two edits, and the compiler needs only the first.

2. **Which theme field the widget reads.** The answer is in a *dependency's*
   source. It drifts because upstream changes without telling us, and because
   nobody re-reads 102 widgets when bumping a patch release.

3. **What upstream hardcodes.** Also in the dependency, but a statement about
   the *absence* of a field.

(1) is a fact our own code holds, and a lexical test settles it. (2) and (3)
are facts a dependency's code holds. The project already has the pattern for
that: `src/contract.rs` partitions all 138 `ThemeColor` fields across `ROWS`,
`COMPUTED_ROWS` and `DERIVED`, and
`every_theme_color_field_has_a_declared_source` asserts the partition is
exact (`src/contract.rs:49-53`). A field cannot quietly fall out. This work
wants the same property for a panel.

---

## 3 -- Options considered

### 3.1 Rewrite the 54 stale panels by hand, change nothing else

Cheapest now, and it leaves the mechanism intact. §1.2 is the
counter-evidence: two of the seven omitted builders are in panels that *do*
cite their builders. Rejected as the *only* action; kept as a necessary one,
because no gate rewrites prose.

### 3.2 Delete the "Not themeable" section

It drifts worst and cannot be derived. Rejected: it also carries the
project's whole argument. "`progress_bar.track_height` has no receiver, so
the bar's height is upstream's" is what the showcase exists to say. Removing
it would make the panel truthful by making it useless.

### 3.3 Derive the geometry section from the applied `StyleRefinement`

`StyleRefinement` has public `Option` fields with `Debug`
(gpui-pre `src/style.rs:178-180`; the derive copies field visibility,
`gpui-pre-derive-refineable src/derive_refineable.rs:395-399`). A demo could
hand the panel the refinement it applied, and the panel could print
`min-height: 32px` instead of prose.

**Rejected as disproportionate.** §1.6 measured the premise and it does not
hold: the geometry prose that exists is accurate in every case checked. This
would add a public API item, a wrapper type, a macro to keep its two fields
from diverging, length-formatting rules for three length kinds, and edits at
~60 call sites -- to prevent a defect that has not occurred. The defect that
*has* occurred is omission, and §3.4 catches it with one test.

Recorded as a follow-up with a trigger: **if a panel is ever found
misdescribing what its builder sets**, or if showing live values is wanted
for its own sake, this is the design to reach for.

### 3.4 One lexical test: builders used == builders named, per demo block

`src/showcase.rs` already reads the showcase, strips comments and literals,
and matches `module::name` at identifier boundaries (`:33, 59, 165`). It
already asserts every public builder is exercised *somewhere*. Scoping that
question to a demo block is a small extension of machinery that exists, and
it catches all seven omissions in §1.5. Accepted (**W3**).

### 3.5 A Python extractor of each upstream widget's theme reads

Parse the vendored source, collect `cx.theme().<field>` per widget, follow
helpers, compare to what the panel names.

**Rejected as disproportionate *and* ineffective**, which was a surprise.
Measured on the real defects:

- File-level collection catches Toggle (4 claims) and Radio (1), because
  `toggle.rs` reads no `secondary_*` and `radio.rs` no `background`.
- It **misses** Button (Link) and Button (Text), because `button.rs:1284`
  does read `muted_foreground` -- for the *disabled* state.
- A call-graph version misses them too. Separating "Link's hover reads
  `link_hover`" from "disabled reads `muted_foreground`" needs constant
  folding over the `ButtonVariant` match, which is a Rust interpreter.

So the expensive design and the cheap one catch the same defects, and both
fail on the same ones. That is the definition of machinery not worth its
weight.

### 3.6 Cite the line, and check the line

Every colour claim carries the upstream location it was read at, and a test
opens that location and checks the named field appears there.

This is where the evidence pointed. It catches exactly the case both
extractors fail: the panel would have to cite `button.rs:993`, which reads
`Self::Link => cx.theme().link` -- and `foreground` is not there. It needs no
Rust parsing at all, only file-and-line lookup. It handles our own overrides
uniformly: Ghost cites `variants.rs:54`, which reads `secondary_foreground`.
And when upstream shifts a line, the check fails -- which is the staleness
gate of §3.8, obtained for free rather than built.

Accepted (**W4**). The cost is honest and visible: 324 claims each gain a
citation, which is the audit itself, made durable.

**Why a line and not a symbol.** `file.rs, Symbol` is the *existing*
convention -- 45 of the 227 "Not themeable" entries already use it -- so
symbol citation would have been the conservative choice. It does not work
here, and the reason is the motivating case itself: `ButtonVariant::text_color`
(`button.rs:947-996`) holds `Self::Link => cx.theme().link` at `:993` and
`Self::Text => cx.theme().foreground.opacity(0.9)` at `:994`. A symbol-scoped
check asked "does `foreground` appear in `ButtonVariant::text_color`?" would
answer **yes**, and Button (Link)'s false claim would pass. Only line
granularity separates two variants that share a function.

**The cost of line granularity** is drift: an insertion earlier in a file
moves every later line. Measured against the real bump, that cost is small --
0.6.4 → 0.6.6 changed two files, and only claims citing lines after the edit
in those files would move. It is not zero, and the mitigation is that the
failure must print what the cited line *now* says, so the fix is a look
rather than an investigation (spec §4.2).

### 3.7 Generate the colour list instead of checking it

Rejected: the extracted set has no roles. It cannot know `primary` is "the
checked fill", and cannot know a `Slider`'s `slider_thumb` is a default that
`.color()` overrides. A generated list would be true and unreadable.

### 3.8 A separate staleness stamp over cited upstream files

The first design hashed each cited file and reported which changed on a
version bump. **Dropped as designed, but not wholly** -- and the first
revision of this document was wrong to say §3.6 "subsumes" it.

§3.6 covers **colour** claims. It says nothing about the 227 "Not themeable"
entries, which are prose: "hardcoded ChevronDown", "2s by default", "the
compositor owns the frame". Dropping the stamp outright would have left those
with no protection at all.

What is worth keeping is the cheapest part. 45 of the 227 already cite a
source file. A check that the cited **file and symbol still exist** --
not what they say, only that they are there -- costs very little and catches
the failure mode this project has already been bitten by: upstream removed
`ThemeColor::tiles` in a *patch* release, which is the entire reason the
sibling gpui-kit-0.6.4 work exists. A panel citing a symbol that no longer
exists is a panel describing a version nobody runs.

Accepted in that reduced form (**W9**): existence, not semantics. The
semantic content of prose stays a human's job, and §5's report plus the
version-bump routine are the mitigation.

### 3.9 Detecting what a panel *omits*

§3.6 validates the claims a panel makes; it cannot know about a field the
panel never mentions. A simple file-level scan does answer that -- "fields
read in `radio.rs` that the Radio panel never names" -- but only as a
heuristic, because a widget's file reads fields for states and variants the
demo does not show.

Accepted as an **advisory report, not a gate** (**W6**): a script a human
runs during the audit and at version bumps. Making it a gate would require an
exception per unshown state across ~100 widgets, which is the exception-table
sprawl this revision exists to remove.

---

## 4 -- Decision record

| # | Decision | Cost if wrong |
|---|---|---|
| W1 | Ship as a third v0.5.9 workstream, not v0.5.10 | A version number; reversible by renaming three files |
| W2 | A demo block is a `div().id("tt-…")`; a builder reaches it directly **or** through a `let` binding in the enclosing `fn`, and §3 reads both | Nothing moves and nothing is declared — but the attribution rule must be right, so it is measured against the ten Button blocks |
| W3 | Builders are held by **one lexical test**, not by deriving the section | If a panel is later found misdescribing a builder, §3.3 is the fallback |
| W4 | Every colour claim carries `file.rs:line`, and a test checks the field is there | 324 claims gain a citation; upstream line moves surface as failures |
| W5 | The colour section keeps its roles; the test checks the citation, never the role label | A wrong role label is caught by review, not by machine |
| W6 | Omission detection is an **advisory report**, not a gate | A panel can stay incomplete without failing CI |
| W7 | `check-widget-coverage.py` stops accepting a bare path segment for gpui widgets | A per-widget constructor pattern for the eight widgets its docstring already names |
| W8 | Every gate ships a discrimination proof: a seeded defect must fail it, naming the line | — |
| W9 | Prose claims that already cite `file.rs, Symbol` have that symbol's **existence** checked — never its semantics | 45 entries are checked; the other 182 stay unguarded, by choice (§3.8) |

**W1 in detail.** v0.5.9 added `geometry::list`, `tooltip_content`,
`scrollbar_gutter` and the carried text colour on seven builders. Shipping it
with panels that name none of them would ship a UI contradicting its own
release notes. The maintainer may prefer v0.5.10; the work is identical.

**W4 in detail.** The citation is displayed, not hidden: a panel line reads
`text: link #2a7ab0 (button.rs:993)`. That is the point -- a reader who
doubts a claim can check it in one step, which no previous design offered.

**W8 in detail.** A gate that has never been seen to fail is not a gate. Each
one here is proved by seeding a defect and watching it name the line.

---

## 5 -- Deliberately not done, and the trigger to revisit

- **Derived geometry (§3.3).** Trigger: a panel found misdescribing what its
  builder sets, or a wish to show live values.
- **The iced showcase.** Same `widget_tooltip` shape
  (`showcase-iced.rs:1888-1938`), not audited here. Trigger: this lands and
  the gates prove out on gpui.
- **Empirical measurement** -- perturb a theme field, diff the render -- would
  beat reading any source. **Blocked**: `Window::scene` is `pub(crate)`
  (gpui-pre `src/window.rs:985`) and test support exposes no framebuffer.
  Trigger: an upstream accessor. Recorded in `docs/todo.md` as Tier U.
- **Reusing gpui-component's `DivInspector`** (`inspector.rs:72`). It reads
  back a hovered element's *merged final* style, which answers "what is this
  element's style", not the panel's question, "what did the native theme
  contribute". Trigger: none expected.
- **Screenshot diffing of the panel.** Out of scope, as in the sibling spec.

---

## 6 -- Open questions for the maintainer

1. **W1**: v0.5.9 third workstream, or v0.5.10?
2. **W4** displays the citation in the panel (`link #2a7ab0
   (button.rs:993)`). Useful, or is it clutter that belongs in the source
   only?
3. §1.4: the six solid Button variants show a **correct hex** under a
   **wrong field name**. Fix the names now, or leave them until the citation
   pass reaches them anyway?
4. `Command` has no demo at all; one needs a command list and a key binding.
   In this release, or a follow-up with the gate excepting it meanwhile?
