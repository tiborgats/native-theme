# native-theme-egui-widgets — Rationale

Companion to [`todo_egui-widgets-spec.md`](todo_egui-widgets-spec.md).
Target: **egui 0.36.1**. Version milestone: **undecided**.

---

## 0 -- What this document is for

The specification says *what* the crate is. This says *why*, and — more
usefully — why the six alternatives were not chosen, so that none of them has
to be re-argued from scratch in a year.

Where a claim rests on code, the code is cited. Where a claim is a judgement,
it is labelled as one. Where something is unverified, it says so.

---

## 1 -- The problem, stated exactly

The connector is complete and audited: 463 leaves, a true bijection against
`ResolvedTheme`, every verdict adversarially checked. It is also, by its own
ledger, **opt-in**:

> An application that calls `install()` and nothing else gets the 31 effective
> DIRECT leaves plus one elected winner per contested field.
> — `todo_v0.6.0_egui-connector-spec.md` §14.1 item 22

The other 210 SCOPED leaves need somebody at the call site holding the right
`Arc<Style>`. The connector cannot be that somebody: it runs once, at install
time, and never again.

This is not fixable inside the connector. A dedicated sweep enumerated all 162
`Context` methods, all of `plugin.rs`, every `StyleModifier` consumer tree-wide,
every `Ui::new(` call site across seven crates, all 16 public traits, and the
tessellation and persistence surfaces. There is no fourth seam. egui has one
global `Style` with an interaction-state axis and **no widget-type axis**, and
nothing in 0.36.1 changes that.

So the question is not "how do we map more" — the mapping is done — but "who
applies it, and when".

### 1.1 The second problem, which is smaller but real

Some widgets have no egui counterpart at all. `ResolvedSwitchTheme` is 13
leaves of which 8 are UNMAPPABLE, for the simple reason that no switch or
toggle module exists under `egui/src/widgets/`. No amount of style mapping
produces a widget that is not there.

---

## 2 -- Options considered

### Option A: Ship nothing; the connector is enough (rejected)

**The case for it.** The connector already delivers correct colours globally.
Applications that care can scope widgets themselves; the spec documents exactly
how, and `native_scope` exists for the purpose.

**Rejected because** it mistakes a documented workaround for a solution. The
connector's own ledger calls the failure "silent and non-uniform": an
application that forgets to scope gets no error, no warning and no visual cue —
just one elected winner per contested field. Expecting every application author
to thread a `Role` through every call site, correctly, forever, is not a design;
it is a hope. And it leaves §1.1 entirely unaddressed.

### Option B: Put the widgets in the connector crate (rejected)

**Rejected because** the connector's charter forbids it, for reasons that are
still correct:

> Shipping a widget buys a permanent per-release audit obligation —
> interaction, animation, `WidgetInfo`, `AtomLayout` — in a crate whose entire
> remit is theme *mapping*.
> — `todo_v0.6.0_egui-connector-spec.md` §14.3

The obligation is real: egui touches `WidgetInfo` in 22 files. A theme-mapping
crate that also owns interaction semantics has two failure domains and one
version number, and a rendering regression becomes indistinguishable from a
mapping regression.

Note what the charter actually objects to: widgets **in that crate**. A
separate crate satisfies its reasoning exactly rather than evading it — the
mapping crate stays pure, and the widget crate carries its own obligation
openly, with its own version number and its own test suite.

### Option C: A drop-in replacement — shadow egui's API (rejected)

The idea: re-export all of egui, shadow the widget methods, users change one
import and existing code renders natively.

**Rejected because it does not compile into the behaviour it promises.**
`Ui::button` and its 160 siblings are **inherent** methods on `egui::Ui`
(`ui.rs:1847`; 161 inherent `pub fn` in total). Rust resolves inherent methods
before trait methods, so an extension trait offering `button` is *silently
ignored* at every call site. Not a compile error — wrong pixels, no diagnostic.

A design whose failure mode is "looks like it worked" is disqualified on this
project's terms regardless of its other merits.

### Option D: A `Deref`-based wrapper `Ui` (rejected, but closer than it looks)

`struct Ui(egui::Ui)` with `Deref<Target = egui::Ui>` and inherent overrides.
This one deserves more credit than it usually gets: Rust tries inherent methods
on the outer type **before** dereferencing, so an inherent `button` genuinely
would win, and `Deref` passes the other ~140 methods through for free. You
would shadow perhaps 30 methods, not 161.

**Rejected on the builder boundary.** `ScrollArea::show`, `Window::show`,
`Grid::show` and `CollapsingHeader::show` hand the closure a raw
`&mut egui::Ui`, and they are methods on egui's *builders*, not on `Ui`. The
wrapper is therefore lost inside every container unless every container builder
is also wrapped — which is Option E by increments.

Two further objections. Third-party crates take `&mut egui::Ui` and would
render unstyled, producing a *visibly mixed* UI, which is worse than a
uniformly non-native one. And deref-as-inheritance makes it invisible at the
call site which method ran — the same "looks like it worked" failure as
Option C, one step removed.

### Option E: Fork egui's widget layer (rejected)

**Rejected on measured cost.** `widgets/` plus `containers/` is **17,174
lines**; `scroll_area.rs` alone is 1,641 and `text_edit/builder.rs` is 1,440.
Forking means owning interaction, focus order, IME, undo, clipboard,
bidirectional text, animation and accessibility, re-audited every release, in
order to change colours. The ratio of liability to benefit is not close.

### Option F: The upstream PR, and ship no widgets (rejected as a *sole* strategy)

The connector already designs the correct upstream fix: `Style::class_overrides`,
two behaviour-neutral commits, roughly 120 lines, in `widget_style.rs` — a
module that is **byte-identical between egui 0.35.0 and 0.36.1**, which makes it
the best possible PR target because nothing in flight competes with it. If it
lands, SCOPED collapses into DIRECT for every widget egui paints.

**Rejected as the whole answer, not as an idea.** Upstream appetite is
explicitly **UNVERIFIED** — no maintainer has been consulted. Making the
project's headline capability depend on a decision nobody has agreed to is not
a plan. And even if it lands in full, it does nothing for §1.1: it cannot
produce a switch widget that egui does not contain.

### Option G: `[patch.crates-io]` over a minimal fork (adopted — but not as this crate)

Publish the Option F change as a branch and let applications write:

```toml
[patch.crates-io]
egui = { git = "…", branch = "class-overrides" }
```

This is the **only** route that reaches existing code and third-party crates
with no source change, because everything reads the same global `Style`. A
~120-line patch is genuinely rebaseable per release, unlike Option E.

**Adopted, and deliberately assigned elsewhere.** It is the connector's
upstream contribution published as a branch instead of waiting on a merge, so
it belongs to the connector's work, not this crate's. It costs nothing extra:
the patch *is* the PR. If upstream merges it, the fork evaporates and users
delete two lines.

It does not remove the need for this crate — §1.1 again.

### Option H: A companion widget crate, three tiers, `egui::Widget` API (chosen)

Widgets in a separate crate, under distinct names, implementing
`egui::Widget` so they compose through `ui.add()`.

**Why it wins.**

* **The unsound failure mode is structurally excluded.** Because no name
  competes with an inherent method, Option C's silent-shadowing bug cannot
  occur. This is the decisive property.
* **It integrates rather than replaces.** `egui::Widget` (`widgets/mod.rs:63`)
  and `Ui::add` (`ui.rs:1520`) mean our widgets work with `add_sized`
  (`ui.rs:1537`) and `add_enabled` (`ui.rs:1587`) for free, and coexist with
  every egui and third-party widget on the same screen.
* **The cost is bounded and chosen per widget.** The tier model puts the
  expensive widgets permanently out of scope (§2.3 of the spec) instead of
  leaving the boundary to be re-litigated.
* **It solves §1.1**, which nothing else on this list does.

**What it costs**, stated plainly and recorded in spec §13: existing code does
not benefit, and third-party crates do not benefit. Adoption is per call site.
That is the honest price of not being Option C.

---

## 3 -- The decision record

### 3.1 Widgets are `egui::Widget` implementors, not `Ui` methods

Argued in §2 Option C and H. The property that matters is not ergonomics but
**diagnosability**: an extension trait that loses method resolution fails
silently, and a wrong-pixels-no-error failure is the category this project
treats as worst.

A secondary benefit is that it needs no API of its own. `ui.add(..)` is already
how egui users add third-party widgets, so there is nothing new to learn.

### 3.2 No extension trait, ever — not even as sugar

A `ui.native_switch(..)` helper would be additive, compatible and harmless on
the day it shipped. It is still refused, because two spellings for one widget
means every call site carries a permanent question about which is current, and
every future widget must be added twice or the set becomes inconsistent.

This is recorded as a rule rather than a preference so that "it is only one
method" does not reopen it.

### 3.3 Three tiers, with Tier 3 named and closed

The alternative was to judge each widget on its merits when it came up. That
guarantees the boundary drifts toward reimplementation, because each individual
step looks small — `Slider` is only 1,210 lines, `ComboBox` only 487.

Naming Tier 3 and closing it means the expensive cases are decided once, when
nobody is under pressure to ship a particular widget.

### 3.4 The atlas comes from the `Context`, never from a parameter

Every widget calls `ThemeAtlas::from_ctx(ui.ctx())`. The alternative — a
`&ThemeAtlas` parameter — would put the connector's type in every signature,
making the two crates version-locked at the API level rather than merely at the
dependency level, and would push theme plumbing back into application code,
which is the exact burden this crate exists to remove.

### 3.5 No atlas must mean plain egui, never a broken screen

The spec makes this the first implemented behaviour and the first test (T1).

The reasoning is about adoption order, not correctness in the abstract. A user
will inevitably add this crate before wiring up `install()`, or call a widget
from a `Context` where installation failed. If that renders black rectangles or
panics, the crate is judged broken and removed. Falling back to egui's own
appearance is both the safest behaviour and the most honest one: no theme
installed, no theming applied, nothing fabricated.

### 3.6 Font weight is solved by variation axes first, real faces second, never synthesis

The connector cannot express weight because `Style::text_styles` holds
`FontId { size, family }` and nothing else (ledger item 7, with upstream's own
`TODO(emilk): weight (bold), italics`). This crate is not bound by that,
because a hand-painted widget builds its own `LayoutJob`, and `TextFormat`
carries `coords: VariationCoords`, `italics`, `line_height` and more.

The ordering is a quality judgement, and it is deliberate:

1. **Variation axis.** For a variable font, setting `wght` gives the real
   designed weight at any value, from one file, with no duplication. It is the
   only route that is simultaneously genuine, cheap and complete.
2. **A real static face** registered as a separate `FontFamily::Name`. Genuine,
   but costs discovery and memory per face.
3. **Synthesis — prohibited except as a reported fallback.** egui's own
   `TextFormat::italics` shears the glyph quad by a flat 25% of its height
   (`epaint/src/text/text_layout.rs:1173-1177`), which is a fake oblique rather
   than an italic. Algorithmic emboldening is worse still: it distorts stems
   and destroys hinting at UI sizes.

The prohibition is not only aesthetic. Manufacturing a typeface variant the
designer never drew is the same category as fabricating a platform asset, which
this project already refuses. Where synthesis is unavoidable it must emit a
`Note`, so a degraded rendering is observable rather than silent.

### 3.7 The `FontFamily::Name` invariant is a panic guard, not style advice

An unregistered name **panics**:
`panic!("FontFamily::{family:?} is not bound to any fonts")`
(`epaint/src/text/fonts.rs:1031`) and `panic!("No font data found for …")`
(`:1039`). No fallback, no `Result`, no `debug_assert` — it aborts in release.

Under this project's absolute no-panic rule this is the crate's largest single
risk, and it is exactly the risk introduced by §3.6 route 2. Hence the rule
that a `Name` may never be constructed from theme data and used in the same
expression, and hence T3, which makes it mechanical rather than a review
promise.

This is also why the connector's own never-`Name` invariant is *not* simply
inherited: this crate genuinely needs `Name` families, so it must earn them
with a guard instead of banning them.

### 3.8 Accessibility is mandatory, and the mapping is opinionated

A hand-painted widget emits nothing to a screen reader unless it calls
`Response::widget_info` (`response.rs:868`).

`WidgetType` (`lib.rs:623`) has **no `Switch` or `Toggle` variant**, so a
choice was forced. `Switch` reports as `Checkbox` because a switch *is*
semantically a two-state checkbox and `Other` would discard that; `TabBar` and
`SegmentedControl` report as `RadioGroup` because both are exclusive choices
among siblings.

Reporting `Other` everywhere would have been easier and is refused: it is
technically true and practically useless, and a theming project that advertises
respecting accessibility preferences while shipping unusable widgets would be
making a false claim by omission.

### 3.9 Disabled means the theme's colours, not egui's opacity multiply

egui models disabled as one global alpha — `Visuals::disabled_alpha` via
`Ui::disable` → `Painter::multiply_opacity` (`ui.rs:496-501`,
`painter.rs:100-104`) — and has no disabled *colour* at all (ledger item 6).
Every real desktop uses specific greys instead.

So `.enabled(false)` paints the theme's `disabled_background` and
`disabled_text_color` and must not call `Ui::disable`.

`ui.add_enabled(false, w)` is deliberately left alone rather than intercepted:
it applies egui's fade (`ui.rs:1587-1589`), and silently changing the behaviour
of an egui method a user explicitly reached for would be the Option C failure
in a new place. Both spellings exist, they differ, and the rustdoc says how.

### 3.10 Reduced motion is a correctness property

When the atlas reports reduced motion, animation time must be `0.0`. This is
stated as a requirement with a test (T6) rather than left to each widget's
author, because it is exactly the kind of thing that is remembered in the
reference widget and forgotten in the eighth.

### 3.11 No features

The connector ships `default = []` and this crate adds none of its own.
Features here would multiply against the connector's, producing configurations
nobody tests, and no widget is useful only sometimes.

### 3.12 MSRV is the connector's, not the workspace's

`rust-version = "1.95"`, matching egui 0.36.1, and explicitly **not**
`rust-version.workspace = true` (the workspace floor is a measured `1.88.0`).
This crate is strictly downstream of the connector, so it can never require
less.

---

## 4 -- Why the charter is satisfied rather than circumvented

The connector's no-widgets charter (§14.3) exists so that a theme-mapping crate
does not silently acquire a rendering-maintenance obligation. Its test:

1. egui has no widget of that visual identity, **and**
2. a majority of the corresponding struct's leaves are otherwise UNMAPPABLE.

A separate crate honours the *reason* for the charter — the obligation is
carried openly, versioned separately, and tested separately — while the
charter's *test* is retained verbatim as the Tier 1 admission rule.

That retention has an awkward consequence, and the spec records it rather than
smoothing it over: **only `Switch` passes both conditions.** Six
container-shaped widgets — tab bar, toolbar, status bar, sidebar, card,
segmented control — pass condition 1 outright (egui has none of them) but fail
condition 2, because the connector *can* carry most of their leaves into a
`Frame` or a scoped `Style`.

The honest reading is that condition 2 was written to answer a different
question. In the connector it meant "is a widget the only way to reach these
values?" Here the question is "is a widget the only way to reach this
*appearance*?" — and for a tab bar the answer is yes even though its leaves are
individually mappable, because an application must otherwise assemble it from
`Frame` and `Button` and get the composition right.

Spec §15 Q-2 proposes the amended condition. It is left open rather than
decided here because it governs eight of the nine Tier 1 widgets, and a rule
that admits that much work should be approved deliberately rather than inferred
from a rationale document.

---

## 5 -- The long-term argument

### 5.1 When egui bumps a minor

Tier 2 is the exposed surface: it names egui's widgets and their builder
methods. Renames surface as compile errors, which is the good case. The version
policy is inherited from the connector — one egui minor at a time, never a
range.

Tier 1 is far more stable, because it depends only on the primitives:
`allocate_response`, `painter`, `Response`, `WidgetInfo`. Those are the oldest
and least volatile parts of egui's API.

### 5.2 When native-theme grows a widget

Adding a widget to `ResolvedTheme` does not break this crate — it adds a
candidate for Tier 1 admission, judged by §2.4. Nothing in the public API is
keyed to the widget count.

### 5.3 If the upstream change lands

Tier 2 becomes redundant: SCOPED collapses into DIRECT and plain egui widgets
render natively without a wrapper. Tier 2 wrappers would then be thin
pass-throughs and can be deprecated without breaking callers, since they are
functions returning `impl Widget`.

Tier 1 is untouched. It exists for widgets egui does not have, which no styling
change can conjure.

**This asymmetry is the reason the crate is safe to build now.** Its
speculative half degrades gracefully into a no-op; its durable half does not
depend on the speculation at all.

### 5.4 The shape of the bet

The crate bets that egui will keep its `Widget` trait, its painting primitives
and its accessibility model — all of which predate the styling system and none
of which the upstream discussion touches. It does **not** bet on
`widget_style.rs` evolving, on the PR being accepted, or on egui gaining a
widget-type axis.

---

## 6 -- What was deliberately not done

* **No `Link` in Tier 1**, though it is the worst-covered widget (9 of 12
  leaves lost) and a Tier 1 version could track visited URLs in
  `ctx.data_mut()` and fix it. It fails §2.4 condition 1 — egui ships a
  hyperlink — and admitting it would mean the test is decorative. Revisit only
  through Q-2.
* **No text-rendering configuration.** Reading the platform's hinting and
  antialiasing preferences belongs in the core crate and the connector, and is
  tracked in `todo.md`. This crate consumes whatever fonts were registered.
* **No icon loading.** The connector owns it.
* **No layout containers.** A `Row`/`Column` that applied theme spacing would
  overlap egui's own layout API and pull this crate toward Option D.

---

## 7 -- Open questions

Carried from spec §15, not duplicated in detail:

* **Q-1** — `impl Widget` versus named structs for Tier 2 returns.
* **Q-2** — the Tier 1 admission rule. **This is the one that matters**: it
  governs eight of the nine candidate widgets and blocks Phase D.
* **Q-3** — the switch thumb inset and tab underline thickness, which have no
  `ResolvedTheme` source and must not become hardcoded constants.
* **Q-4** — whether the crate survives the upstream change. Answered in §5.3:
  yes, asymmetrically.

One item is **UNVERIFIED** and repeated here so it is not lost: **upstream
egui's appetite for `Style::class_overrides` has never been tested.** Options F
and G both rest on it, and this crate deliberately does not.
