# v0.5.9 — Theme contracts: the iced mappings and the machinery that proves them: Rationale

Status: Design (2026-09-20, revised 2026-09-21); nothing implemented
Crates: `connectors/native-theme-iced` (mapping fixes, new `styles` module),
`connectors/native-theme-gpui` (contract tests), both showcases
Companion specification:
[`todo_v0.5.9_theme-contracts-spec.md`](todo_v0.5.9_theme-contracts-spec.md)
Companion plan:
[`todo_v0.5.9_theme-contracts-plan.md`](todo_v0.5.9_theme-contracts-plan.md)
Sibling work in the same release: the gpui connector's move to GPUI Kit 0.6.4,
[`todo_v0.5.9_gpui-kit-0.6.4-rationale.md`](todo_v0.5.9_gpui-kit-0.6.4-rationale.md)
(decisions E1–E21). This document numbers its decisions **C1–**, so the two
sets never collide.

---

## 0 -- What this document is for

The v0.5.9 gpui work began as a compatibility release. During its visual check
the maintainer found four defects in four minutes, none of which any test
could have caught. Investigating them exposed a defect *class*, then the same
class in the iced connector, then the fact that nothing in the repository
states what a correct mapping even is.

This document argues what to do about that. The specification says what to
build.

Conventions as in the sibling document: upstream citations name the published
crate sources, every number was measured on the maintainer's machine unless a
source is given, and "all presets" means the 16 presets in both modes — 32
combinations. Measurements are dated 2026-09-20 unless marked 2026-09-21,
which is the revision pass.

---

## 1 -- The evidence

### 1.1 What the maintainer found, and what each one was

| Found by eye | Root cause | Could a machine have caught it? |
|---|---|---|
| The vertical resizable divider would not move | the showcase gave a two-panel group 200 px where gpui-base clamps each panel to 100 px, so both sat at 99 px with no slack | Yes. Proved afterwards with a headless `resize_panel` probe: `[99, 99]` stayed `[100, 99]` for a drag asking 140 px. |
| The `InputGroup` Copy button did nothing | built with `.icon()` and `.label()` and no `.on_click()` | Yes. A click on it changes nothing observable. |
| Its hover was a barely visible grey while every other button hovered blue | upstream hardcodes `theme.muted` for an in-group addon button; `muted` is the subdued-surface colour of `Kbd` and code blocks | Yes. `#dbdcdd` sits 20 units from the `#eff0f1` field where KDE's `button.hover_background` is `#93cee9` at 57. |
| Its corner radius differed from the calendar's day buttons | gpui-component scales radius with control *size*: an `XSmall` in-group button takes `radius / 2` | Yes. The refinement's radius against the widget's native radius. |

Four findings, four machine-checkable properties, none of which needed a
picture. That is the central fact this document is built on: **the visual
check was doing work that belongs in tests**, and doing it unreliably, because
it depends on one person noticing one wrong pixel among thousands.

### 1.2 The defect class

Three of the four, and the `accent` bug the audit then found in the gpui
connector, are the same mistake: **a native field mapped into a toolkit slot
whose meaning is a different one**.

- gpui `accent` is upstream's *item highlight* ("hover background on MenuItem,
  ListItem, etc.", `theme/theme_color.rs:60-61`, and the same doc on the
  serde schema at `theme/schema.rs:254-256`). The connector fed it
  `defaults.accent_color`. On adwaita that painted hovered menu rows
  `#3584e4` where the platform says `#e8e8e8`. Fixed in this release (E20).
- Upstream itself makes the same mistake in the other direction: `Checkbox`
  draws its unchecked border from `theme.input`, the *text input's* border
  colour, and `InputGroupButton` hovers from `muted`. Those are upstream's to
  fix (Tier U).

The class is not exotic. Both connectors translate a rich, per-widget model
into a flat set of toolkit slots, and every such translation is a claim about
meaning that nothing checks.

### 1.3 The same class, unfixed, in the iced connector

The iced connector's `to_theme` output was read against the widget catalogs of
the published `iced_widget-0.14.2` sources on 2026-09-20 — every reader of
every slot the connector writes, found by grepping the catalogs for that slot.
Seven instances:

| iced slot the connector writes | What iced reads it as | What the platform says |
|---|---|---|
| `secondary.base.color` ← `button.background_color` (`extended.rs:109`) | **three different things across six readers**: placeholder text (`text_input.rs:1769`, `text_editor.rs:1476`, `pick_list.rs:910`), the `button::secondary` and `container::secondary` fill (`button.rs:615`, `container.rs:629`), and the `progress_bar::secondary` bar fill (`progress_bar.rs:299`) | `input.placeholder_color`, never read. On adwaita light the placeholder becomes `#e8e8e8` on a `#fafafb` field — about **1.15:1**, invisible. Measured over all 32 combinations on 2026-09-21 it is not an Adwaita problem: today's placeholder sits between **1.06:1 and 1.84:1 on every preset in both modes**, where the platforms' own pairs run from 1.36 to 10.06 |
| `background.weak.color` ← `defaults.surface_color` (`:111`) | scrollbar rail, unchecked switch track, menu panel, closed pick-list, disabled input, rounded box, button hover, rule, several checkbox states — **nine roles across 29 references in ten widget modules** | each has its own field: `scrollbar.track_color`, `switch.unchecked_background`, `menu.background_color`, `input.disabled_background`, `button.hover_background` |
| `primary` ← `defaults.accent_color` (`palette.rs:41`) | the pick-list/menu **item highlight** (`overlay/menu.rs:657-658`) | `menu.hover_background` — the identical defect to gpui's `accent` |
| `primary.weak.color` (derived) | **text selection** (`text_input.rs:1771`) | `input.selection_background`; the connector even has a `selection_color()` helper (`lib.rs:316`) that `to_theme` never calls |
| `primary.strong.color` / `primary.base.color` (derived) | **hovered** and **dragged** scrollbar thumb (`scrollable.rs:2385` and `:2414` respectively — two different slots) | `scrollbar.thumb_hover_color` / `thumb_active_color`, never read |
| `background.strong.color` (derived from the *window background*) | every **border, divider, rail and track** (`text_input.rs:1766`, `checkbox.rs`, `rule.rs:308`, `slider.rs:687`, `progress_bar.rs`) | `defaults.border.color`, `input.border.color`, `checkbox.unchecked_border_color`; the `border_color()` helper (`lib.rs:292`) is never written into the theme |
| only `.base` entries are written (`extended.rs:109-119`) | `.weak` / `.strong` keep values generated from the *unoverridden* palette | a button painted with the platform's surface jumps to an unrelated tone on hover, because `button::secondary`'s hover reads `secondary.strong.color` (`button.rs:620`) and `button.hover_background` is never read |

Two more, of a different kind, and these two are what the repository's own
`connector-parity-checker` reports, because they are public-surface and
feature-table differences rather than slot semantics:

- Nothing in the iced connector takes `AccessibilityPreferences`, so **text
  scaling never reaches an iced application**: `font_size()` and
  `mono_font_size()` (`iced/src/lib.rs:259, 274`) return the platform's size
  unscaled, and `from_system()` (`:177`) reads the preferences and drops them.
  §2.10 works out which preferences iced can receive at all. (An earlier
  draft cited `gpui/src/lib.rs:137-138` as claiming otherwise about the iced
  connector; those lines are about `is_dark`, not accessibility, and the
  citation is withdrawn.)
- The iced connector declares no `[features]`, so a consumer depending on it
  alone gets `native_theme::icons::load_icon` returning `None` for **every**
  icon; gpui forwards the four icon features.

### 1.4 Why iced's case is architecturally worse, and what that forces

gpui-component takes 138 colour fields. iced takes six colours, each expanded
to a `base` / `weak` / `strong` triple, and its widget catalogs read that
triple. There is no hook to replace a catalog: `Catalog for Theme` fixes the
default styles, and `Theme::Custom` carries only a palette.

So in iced a slot with six readers and three meanings **cannot** be made right
for all of them by any palette value. The only exact seam is per-widget: every
iced widget takes `.style(impl Fn(&Theme, Status) -> Style)` (`button.rs:184`,
`text_input.rs:278`, `checkbox.rs:236`, and so on for the rest).

That is the same shape as the gpui connector's answer to a different problem:
`geometry` supplies per-widget refinements because upstream's `Theme` cannot
carry per-widget sizes, and `variants::ghost_button` supplies a button variant
because upstream's tokens cannot carry the platform's flat-button hover. The
iced connector needs the same kind of module for colours.

---

## 2 -- Options considered

### 2.1 Release vehicle

| Option | Verdict |
|---|---|
| **All of it in v0.5.9** | **Chosen by the maintainer (2026-09-20), as a standing rule: every release before v1.0.0 fixes the bugs found during it.** The rule is right in principle — a found bug that ships unfixed is a bug the next reader must rediscover — and it keeps the audit's evidence and its fix in one commit range. |
| Ship v0.5.9 as the gpui compatibility fix, put this in v0.5.10 | Rejected by the maintainer. The argument for it was time-to-fix: `native-theme-gpui` 0.5.8 has been uncompilable on a fresh resolve since 2026-09-18 (sibling §1.2), and the gpui fix is already implemented and green, so every day this release grows is a day new users cannot build. That cost is real and is recorded here; it is the price of the standing rule, and the rule's benefit — no bug outlives the release that found it — was judged larger. |
| Split the crates: release the gpui connector alone | Rejected on the standing decision that the workspace shares one version (v0.5.8 rationale §2.3); the release scripts assume it. |

### 2.2 How to fix the iced mappings

| Option | Verdict |
|---|---|
| Patch more palette slots | Rejected as the whole answer. A slot with several meanings has no single right value; `background.weak.color` is read by the scrollbar rail *and* the switch track *and* the menu panel, which are three different platform colours. Patching harder moves the error around. |
| **Two layers: correct the palette where a slot has one meaning, and add a `styles` module of per-widget style functions for everything else** | **Chosen.** It matches what iced's architecture allows and what the gpui connector already does for sizes. The palette stays the approximate default for an application that styles nothing; `styles::*` is exact for an application that opts in. Each function is a pure builder from `&ResolvedTheme`, like `geometry::*`. |
| Only add `styles`, leave the palette alone | Rejected. It would leave the invisible placeholder (§1.3, 1.15:1) in place for every application that does not opt in, which is an accessibility failure, not a preference. |
| Ship a wrapper widget set (`native_theme_iced::button(..)`) | Rejected. It would duplicate iced's widget API, age badly against it, and force an application to rewrite its view code rather than add one call. |
| Keep the connector on `iced_core` alone and have `styles` return the connector's own data types | Rejected. The `Style` structs live in `iced_widget`, so the module needs that dependency; returning our own types would make every call site convert by hand. The coupling is the same one the gpui connector already accepts with `gpui-component`, it costs a consumer nothing (an iced application has `iced_widget` through `iced`), and the canary reports a breaking change there as it did for gpui-component. |

### 2.2a The `secondary` family: measured, not reasoned

This section has been wrong twice, and both times for the same reason: an
option was chosen by argument and not measured. The measurements below are
from 2026-09-21, over all 32 combinations, with alpha composited.

The readers, verified in `iced_widget-0.14.2`:

| Slot | Every reader |
|---|---|
| `secondary.base.color` | three placeholders, the `button::secondary` fill, the `container::secondary` fill, the `progress_bar::secondary` bar fill — six readers, three meanings |
| `secondary.base.text` | `button::secondary`'s label, `container::secondary`'s text — **both paired with `secondary.base.color`** |
| `secondary.strong.color` | `button::secondary`'s hover, and nothing else (`button.rs:620`) |

No value satisfies a placeholder *and* a button surface *and* a progress-bar
fill, so one meaning has to win. The candidates, and what each does to the
placeholder — the one reader where being wrong makes text unreadable:

| Option | Placeholder contrast, measured | Verdict |
|---|---|---|
| Today: `button.background_color` | 1.06–1.84 on all 32 | The defect. |
| Drop `.base.color`, keep `.base.text`, add `.strong.color = button.hover_background` (first draft) | as the next row | Rejected. A chimera: generated fill, platform label, platform hover. |
| Override none of the family, leave iced's generated value (second draft, which called it "readable by construction") | **worse than the platform's own pair in 22 of 32**, by up to 6.2 — material light falls from 9.11 to 2.91, kde-breeze light from 4.21 to 2.91 | Rejected. Better than today and still a regression against every platform, and it fails this release's own Layer 3. The claim it rested on was never measured. |
| **`secondary.base = Pair::new(input.placeholder_color, text)`** | the foreground is **exactly the platform's on all 32**. The ratio equals the platform's wherever the field and window backgrounds agree (14 of 32); it is lower in 12, by at most 0.68, and higher in 6 | **Chosen.** |
| Keep the family native to the button and fix the placeholder only in `styles::text_input` | 1.06–1.84 for every consumer on `default-features = false` | Rejected: an accessibility failure for anyone who opts out of `styles`. |

Why the chosen option is principled and not merely the best number. iced
itself decided that a placeholder and a secondary fill are the same colour —
a muted mid-tone between background and text. `input.placeholder_color` *is*
the platform's muted mid-tone. So the three text readers get the platform's
value exactly, and the three fill readers get what iced's own design gives
them: the placeholder tone. Their label comes from `Pair::new`
(`iced_core-0.14.0/src/theme/palette.rs:440`), iced's own readable-text rule,
so nothing is invented; measured, that label never falls below 4.16:1.
`.weak` and `.strong` stay iced's generated values, which are neighbours of
the same mid-tone by construction (`palette.rs:531-543`).

The residual 12 of 32 are not a placeholder defect. iced paints a text input
on `background.base.color`, the *window* background, and 18 of the 32
combinations give the field its own `input.background_color`. No palette value
can fix that; `styles::text_input` does, exactly.

`primary` is not affected and keeps `primary.base.text`: its `.base.color`
comes from the `Palette` itself (`palette.rs:41`), so the pair stays native on
both sides. `background.weak` keeps both members for the same reason.

### 2.3 What to automate, and what to leave to a human

The four findings of §1.1 are the specification for this. Ordered by what they
would have caught, cheapest first.

A fifth thing the audit found, which no layer catches because it is not a
defect but an absence: **a showcase that omits a widget hides it from every
check that depends on the showcase.** The gpui showcase exercises 15 of the
connector's 34 geometry builders (re-measured 2026-09-21: still exactly 19
unused) and renders no `StatusBar`, `TitleBar` or `Combobox`, each of which
has a builder shipped for it. Completing both showcases, and keeping them
complete — a test for our own builders, a script for the toolkit's widgets —
is therefore part of this work rather than a later pass (specification §6a).

| Layer | Catches | Why this and not something else |
|---|---|---|
| **1. Mapping contracts** — one data-driven test per connector: every toolkit slot, the native field of the widget that reads it, over all 32 preset/mode combinations, plus a coverage tripwire so a slot cannot be added without a declared source | the whole class of §1.2: `accent`, the iced seven, and the next one | It is the only layer that states what "correct" *means*. Everything else checks consequences. |
| **2. Showcase self-tests** — the showcases gain `test = true` and `#[cfg(test)]` modules that render every tab headlessly, assert each resizable group has drag slack, and click the interactive controls | the two interaction findings | Both were *showcase* bugs, not library bugs, so a library test could not have caught either. The showcase is the only place they live. Both toolkits can do this (§2.5). |
| **3. Theme-wide invariants** — for every text-on-background pair, the connector's output is never less readable than the platform's own values, over all 32 combinations (§2.6) | iced's 1.15:1 placeholder, and any future one | Needs no baseline, no rendering and no human: it is arithmetic over the resolved theme. |
| 4. Screenshot diffing | clipped text, overlap, layout collapse | **Deliberately not in this release** (§4), for a cost that is now measured rather than assumed: `iced_test` supplies the whole mechanism for iced, and gpui has no equivalent, so adopting it means a baseline regime for one toolkit only. See §4. |

### 2.4 How the contract test states a contract

| Option | Verdict |
|---|---|
| Assert each slot in its own `#[test]` | Rejected: 138 tests in gpui, and adding a slot means remembering to add a test. |
| **One table of (slot, native field, presets where they may legitimately differ), iterated over every preset and mode** | **Chosen.** The table *is* the documentation of the mapping, in one place, in the connector's own source. A reader sees the whole contract at once, and a reviewer sees a change to it in the diff. |
| Generate the table from the mapping code | Rejected: it would assert that the code does what the code does. The table's value is that a human wrote down the intent separately. |

The coverage tripwire is what makes it honest: every field of `ThemeColor`
(and every iced palette slot) appears either in the table or in a list of
derived values with the derivation named. A new upstream field fails the
build until someone classifies it — the same mechanism as E17's `Theme` shape
tripwire, applied to meaning rather than shape.

The gpui connector already proves the weaker property that no field is left
unassigned (`colors.rs`, `no_theme_color_field_is_left_at_default`, which
walks all 138 fields through serde). The tripwire adds the missing half: not
"is it set" but "is it set *on purpose, from a named source*".

### 2.5 Why the showcase tests live in the showcase, and what each toolkit can do

An `examples/` target cannot be imported by an integration test. The options
were to move each showcase into its crate as a feature-gated module, or to set
`test = true` on the example target and put `#[cfg(test)]` tests in the file
itself. The second is chosen: it costs one manifest line per showcase, keeps
the showcase a single self-contained file that an application author can read
as an example, and `cargo test` picks it up with no workflow change.

**Both toolkits can render and click headlessly.** The first draft said iced
could not, on the evidence that no `iced_test` crate sat in the local registry.
That evidence was invalid — the crate is absent locally only because no
enabled feature pulls it — and the conclusion was wrong.

`iced_test` 0.14.0, "A library for testing iced applications in headless
mode", was published 2025-12-07, the same day as iced 0.14.0. It gives
`Simulator::{find, click, tap_key, typewrite, point_at, simulate,
into_messages}` and `Simulator::snapshot` → `Snapshot::{matches_image,
matches_hash}`. Verified 2026-09-21 by compiling and running a scratch crate
against the published sources: `click("Bump")` resolved the button by its
label, `into_messages()` returned `[Bump]`, and `snapshot()` rendered with no
window and no display server.

One measured condition: the test's dependency graph must carry a renderer
backend. Without one, every widget measures 0×0 and `click` returns
`TargetNotVisible`; with `iced`'s default features — which the iced showcase's
dev-dependency already has — the button measured 41.9 × 20.8 and the click
landed. The specification records that condition so it is not rediscovered.

So the iced showcase gets the same class of test as the gpui one: click the
control the showcase advertises, assert the message it produces. That is the
iced form of the dead-Copy-button finding, and it is a real interaction test
rather than a structural inspection of the view tree.

### 2.6 Contrast: which pairs, and which threshold

The first draft of this section said "assert AA for every pair the connector
produces, with a small exception list". Measuring it first — which is why it
was measured — showed that wrong twice over.

Of 512 pairs across the 32 combinations, **174 sit below AA**, and they are
not all errors: macOS ships `#34c759` with white text (`macos-sonoma.toml:21`),
about 2.2:1, and that is Apple's own value. Asserting AA would assert that
every platform meets AA, which is false, and satisfying the assertion would
mean inventing values the platform did not give.

A first measurement was also wrong in the other direction: it read translucent
colours as opaque and produced 143 phantom failures, including 1.00 on Windows
11's `#0000000a` menu hover, which is a 4 % overlay meant to composite over the
window.

| Option | Verdict |
|---|---|
| Assert AA everywhere | Rejected: false on real platform data (above). |
| Assert AA with an exception list | Rejected: the list would be ~174 entries of platform data encoded in our tests, and would need editing whenever a preset is retuned. |
| **Assert the connector never makes a pair worse than the platform's own** | **Chosen.** For each pair, compute the ratio from the native fields and the ratio from the connector's output, composite alpha on both sides first, and require ours to be no worse. It catches exactly the defect class it exists for — iced's placeholder is 1.15:1 where the native pair is readable, so our ratio is worse and it fails — and stays silent where we faithfully emit a platform's own poor choice. |

**Where it is asserted, and where it can only be reported.** The rule is an
assertion wherever the connector controls both colours of a pair: the whole of
gpui's `ThemeColor`, and every `styles::*` output. iced's *palette* layer is
different, because iced chooses the background: a text input is painted on
the window background whatever the platform's field colour is (§2.2a). There
the foreground is pinned exactly by a Layer 1 row, and the pair is printed
with both ratios rather than asserted — asserting it would need a tolerance,
and a tolerance is an invented number.

The test also prints every sub-AA pair without failing, so the list stays
visible; whether any is a preset bug rather than a platform fact belongs to
`preset-validator`. The candidates found so far are recorded in
`docs/todo.md` under Research.

### 2.7 What a showcase is allowed to show

A showcase exists to show every widget **in the native theme**. It is also the
screenshot source for the README, so anything it renders is what the project
claims native-theme achieves.

It follows that a showcase never deliberately renders a widget in something
other than the best the connector can deliver. Two things this release nearly
got wrong:

- The iced showcase was going to keep one section styled by the palette alone,
  "so a reader can see what an application gets by default". Rejected: that is
  documentation's job, not the screenshot's. Every widget in the iced showcase
  takes its `styles::*` function.
- The gpui showcase showed upstream's `.ghost()` button beside a
  `variants::ghost_button` one, to make the difference visible. Rejected for
  the same reason, and the comparison had already served its purpose by being
  decided. The `Ghost` entry in "Button Variants" now takes
  `variants::ghost_button` like every other button, and the duplicate is gone.

The distinction that makes this coherent: *flat* is an application's design
choice, not upstream's default — a toolbar button and an in-field action
button are flat on every platform, and platform-facts models one button
appearance with no flat variant (checked 2026-09-20). What the connector owes
a flat button is the platform's state colours, which is exactly what
`variants::ghost_button` gives it. So a flat button in the showcase is native;
a flat button hovering in upstream's item-highlight colour is not.

### 2.8 `iced_aw`

native-theme models `menu`, `card`, `tab`, `sidebar`, `spinner` and a
selection list. iced core has **none** of them, so six of the model's widget
themes reach nothing in an iced application. That is a larger hole than any of
§1.3's seven defects, and `iced_aw` 0.14.1 fills it with widgets that depend
on exactly the `iced_core` and `iced_widget` versions this connector uses.

| Option | Verdict |
|---|---|
| Ignore it; iced core is the connector's remit | Rejected. It would leave six modelled widget themes permanently unreachable, and the project's purpose is native-looking applications, not native-looking *core* widgets. |
| Depend on it unconditionally | Rejected. It is a third-party community crate; an application that does not use it should not pay for it, and native-theme should not tie its release cadence to it. |
| **A non-default `iced_aw` feature carrying `styles::aw::*`** | **Chosen.** Its styling is the same shape as iced's own — plain `Style` struct, `Catalog` with `StyleFn`, `.style(closure)` per widget — so the marginal cost over §2.2 is the mapping itself. Applications opt in; the showcase enables it so the widgets are covered by the completeness rule. |

Only the six widgets native-theme models are covered. `badge`,
`date_picker`, `time_picker`, `color_picker`, `drop_down`, `number_input`,
`slide_bar` and the layout helpers have no native counterpart, and inventing
values for them is forbidden.

### 2.9 Which iced crates the connector covers, and how a consumer narrows it

The connector now spans three iced crates — `iced_core` for the palette,
`iced_widget` for `styles`, `iced_aw` for `styles::aw` — so a consumer should
be able to pay only for what they use.

| Option | Verdict |
|---|---|
| Subtractive features: `no_aw`, `no_widgets`, `core_only` | **Rejected, although this was the shape first proposed.** Cargo features are additive and unified across the entire dependency graph: if any crate in the graph enables `no_widgets`, `styles` disappears for *every* consumer, and the ones who needed it cannot countermand it — feature unification only ever adds. A subtractive feature therefore turns one consumer's narrowing into another's compile error, and the second consumer may not even know the first exists. |
| **Additive features with a `default` set, narrowed by `default-features = false`** | **Chosen.** `widgets` (default) enables `styles`; `iced_aw` enables `styles::aw` and implies `widgets`. "Core only" is `default-features = false`, which is per-consumer and cannot leak. Same intent, correct polarity. |
| One feature per widget group | Rejected: over-engineered for eleven style functions and one builder. |

`iced_aw` is **not** in `default`, for two measured reasons (crates.io,
2026-09-21). It depends unconditionally on `iced_fonts` 0.3.0, whose published
archive is **3.31 MiB** of embedded font data. And its iced support lags: iced
0.14.0 was published **2025-12-07**, and `iced_aw`'s first release supporting
it, 0.14.0, arrived **2026-04-27** — four and a half months later. A
default-on third-party dependency would make this crate unbuildable on a new
iced for that long, which is precisely the failure this release exists to
repair, one crate over.

`widgets` *is* in `default`, because `iced_widget` ships in lockstep with
`iced_core` as part of iced itself, and any real iced application already has
it through `iced`.

### 2.10 What accessibility can reach in iced

The first draft gave `to_theme` an `&AccessibilityPreferences` parameter "like
its gpui sibling". Working out what the parameter would *do* showed it would
do nothing, and that the real receivers are elsewhere.

| Preference | Where it can land in iced | Finding |
|---|---|---|
| `text_scaling_factor` | not in `to_theme`: an iced `Theme` is a palette and carries no font size. It lands in `font_size()` and `mono_font_size()`, the two values an application puts on its widgets | those two take the preferences. `from_system()` must then hand them to the caller, because today it reads them and drops them |
| `reduce_transparency` | would composite a translucent *surface* | **no receiver today.** Measured: none of the eight palette inputs and none of the surface colours `styles::*` emits is translucent in any of the 32 combinations. The 14 fields that are translucent somewhere are all state overlays and scrollbar thumbs, which the platform itself draws translucent |
| `reduce_motion` | iced has no global animation switch | **no receiver.** The application reads the public field. A `reduce_motion(prefs) -> bool` wrapper, which the first draft specified, would return a field of a public struct |

| Option | Verdict |
|---|---|
| `to_theme(resolved, name, prefs)` for symmetry with gpui | Rejected. gpui's `Theme` holds font sizes and an overlay scrim, so its parameter has work to do; iced's would be dead, and a parameter that does nothing is a promise the crate does not keep. Parity is of capability, not of parameter lists. |
| A separate `scaled_font_size(resolved, prefs)`, leaving `font_size(resolved)` | Rejected. It leaves the trap in place: the obvious call silently ignores the user's accessibility setting, which is the defect. |
| **`font_size` and `mono_font_size` take the preferences; `from_system` returns them** | **Chosen.** The right call becomes the only call. The factor is sanitised exactly as gpui does it (`gpui/src/lib.rs:414-417`): used when finite and positive, else 1. |

---

## 3 -- Decision record

| # | Decision |
|---|---|
| C1 | All of this ships in v0.5.9, under the maintainer's standing rule that every pre-1.0 release fixes the bugs found during it (§2.1). The delay to the gpui compatibility fix is accepted and recorded. |
| C2 | The iced connector gains a `styles` module of per-widget style functions built from `&ResolvedTheme`, mirroring gpui's `geometry` and `variants` (§2.2). |
| C3 | The iced palette writes `secondary.base` as `Pair::new(input.placeholder_color, text)` instead of the button surface. Of the slot's three meanings the text one wins, because it is the one where a wrong value is unreadable; measured, the placeholder foreground becomes exactly the platform's on all 32 combinations, where leaving iced's generated value would have been worse than the platform in 22. The platform's button colours are delivered by `styles::button` (§2.2a). |
| C4 | Text scaling reaches iced where iced can receive it: `font_size` and `mono_font_size` take `&AccessibilityPreferences`, and `from_system` returns them. `to_theme`, `from_preset` and `to_iced_theme` are unchanged, because a palette has nothing for the preferences to act on (§2.10). Breaking, documented under Breaking Changes. |
| C5 | The iced connector forwards the four icon features, so an application depending on it alone gets working icons. |
| C6 | Layer 1: a mapping-contract table per connector, iterated over all 32 preset/mode combinations, with a coverage tripwire over every slot (§2.4). |
| C7 | Layer 2: both showcases get `test = true` and self-tests. Both render and click headlessly — gpui on GPUI's test platform, iced with `iced_test`'s `Simulator` (§2.5). |
| C8 | Layer 3: the connector never makes a text-on-background pair less readable than the platform's own values — asserted for gpui and for every `styles::*` output, reported for iced's palette layer, where iced and not the connector chooses the background; sub-AA pairs are printed, not asserted, because 174 of 512 are real platform data (§2.6). |
| C9 | Screenshot diffing is not in this release (§4). |
| C10 | The gpui connector's `accent` fix (sibling E20) is re-stated as a contract-table row rather than two bespoke tests, so it is covered by the same mechanism as everything else. |
| C11 | The iced connector takes `iced_widget` as a normal dependency, because every widget `Style` type lives there and none but `text` is in `iced_core` (§2.2). Those structs derive no `Default` and are not `#[non_exhaustive]`, so `styles::*` constructs them exhaustively and an upstream field addition fails the build. A `Style` field the native model does not carry takes the value iced's own default style function gives it, cited by file and line — never a number of ours (specification §3). |
| C12 | `iced_aw` is covered behind a non-default `iced_aw` feature, for the six widgets native-theme models and iced core lacks (§2.8). |
| C13 | Coverage of the three iced crates is selected by **additive** features — `widgets` in `default`, `iced_aw` opt-in and implying `widgets` — narrowed with `default-features = false`. No subtractive feature, because Cargo unifies features across the graph and one consumer's narrowing would break another's build (§2.9). |
| C14 | Scrollbar *widths* are not a `Style` field in iced and cannot travel through `.style(..)`. `styles::scrollbar` returns a configured `scrollable::Scrollbar` carrying `groove_width` and `thumb_width`; `scrollbar.min_thumb_length` has no receiver in iced 0.14 and is recorded as unreachable rather than approximated (specification §3). |
| C15 | `iced_test` 0.14.0 is a dev-dependency of the iced connector, for the showcase's interaction tests (§2.5). It is dev-only, so no consumer pays for it. |
| C16 | A `soft_option` field is `Option` even after resolution, and `None` means the platform has no distinct appearance in that state. `styles::*` falls back by **copying** the widget's base-state value, never by arithmetic — the rule the egui design already set (`todo_v0.6.0_egui-connector-rationale.md` §3.6). Native alpha is emitted unchanged (specification §3.2). |

---

## 4 -- Deliberately not done, and the trigger to revisit

| Item | Trigger |
|---|---|
| Screenshot diffing with perceptual thresholds | A defect that Layers 1–3 provably cannot express — clipped text, overlapping elements, a collapsed layout. The tooling question is now settled rather than open: `iced_test` supplies `Snapshot::matches_hash` / `matches_image`, a pinned bundled font (`iced_renderer/fira-sans`) and a pinned size (`Simulator::with_size`), so for iced the remaining cost is the baseline regime itself — a stored artefact per tab per preset per mode, re-blessed on every legitimate style change. gpui has no equivalent, so adopting it now would mean that regime for one of two connectors. Revisit when a defect escapes Layers 1–3, or when gpui gains a comparable snapshot API. |
| An iced `geometry` module (menu row height, dialog paddings, control min-heights — 34 builders in gpui, 5 values in iced) | The next iced release. This one fixes colour meaning, not the geometry gap; doing both at once would double a release that is already carrying two bodies of work. The showcase coverage test (specification §6a.4) will keep the gap visible. |
| Upstream fixes for the tokens gpui-component reads wrongly (`Toggle` pressed, the Outline tab hover, the checkbox border, the internally built ghost buttons) | Upstream accepting the token proposals recorded in `docs/todo.md`. |
| A shared contract vocabulary across connectors (one table, two backends) | A third connector. With two, the duplication is smaller than the abstraction. |

---

## 5 -- Open questions for the maintainer

None. §2.2a and §2.10 settled the last two.
