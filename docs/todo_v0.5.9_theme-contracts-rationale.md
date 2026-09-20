# v0.5.9 — Theme contracts: the iced mappings and the machinery that proves them: Rationale

Status: Design (2026-09-20); nothing implemented
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
crate sources, every number was measured on 2026-09-20 on the maintainer's
machine unless a source is given, and "all presets" means the 16 presets in
both modes — 32 combinations.

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
  ListItem, etc.", `theme/schema.rs:254-255`). The connector fed it
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

The repository's own `connector-parity-checker` was run against the iced
connector on 2026-09-20. Seven instances, verified against the published
`iced_widget-0.14.2` sources:

| iced slot the connector writes | What iced reads it as | What the platform says |
|---|---|---|
| `secondary.base.color` ← `button.background_color` (`extended.rs:109`) | **placeholder text** (`text_input.rs:1769`, `text_editor.rs:1476`, `pick_list.rs:910`) and the `button::secondary` fill | `input.placeholder_color`, never read. On adwaita light the placeholder becomes `#e8e8e8` on a `#fafafb` field — about **1.15:1**, invisible |
| `background.weak.color` ← `defaults.surface_color` (`:111`) | scrollbar rail, unchecked switch track, menu panel, closed pick-list, disabled input, rounded box, button hover, rule, several checkbox states — **nine roles** | each has its own field: `scrollbar.track_color`, `switch.unchecked_background`, `menu.background_color`, `input.disabled_background`, `button.hover_background` |
| `primary` ← `defaults.accent_color` (`palette.rs:41`) | the pick-list/menu **item highlight** (`overlay/menu.rs:658`) | `menu.hover_background` — the identical defect to gpui's `accent` |
| `primary.weak.color` (derived) | **text selection** (`text_input.rs:1771`) | `input.selection_background`; the connector even has a `selection_color()` helper that `to_theme` never calls |
| `primary.strong.color` (derived) | hovered/dragged **scrollbar thumb** (`scrollable.rs:2385, 2414`) | `scrollbar.thumb_hover_color` / `thumb_active_color`, never read |
| `background.strong.color` (derived from the *window background*) | every **border, divider, rail and track** (`text_input.rs:1766`, `checkbox.rs`, `rule.rs:308`, `slider.rs:687`, `progress_bar.rs`) | `defaults.border.color`, `input.border.color`, `checkbox.unchecked_border_color`; the `border_color()` helper is never written into the theme |
| only `.base` entries are written (`extended.rs:109-119`) | `.weak` / `.strong` keep values generated from the *unoverridden* palette | a button painted with the platform's surface jumps to an unrelated tone on hover, because `button::secondary`'s hover reads `secondary.strong.color` (`button.rs:620`) and `button.hover_background` is never read |

Two more, of different kinds:

- `to_theme` takes no `AccessibilityPreferences` at all (`iced/src/lib.rs:113`),
  so **text scaling, reduced transparency and reduced motion never reach an
  iced application**. The gpui connector's own documentation already claims
  otherwise about its sibling (`gpui/src/lib.rs:137-138`).
- The iced connector declares no `[features]`, so a consumer depending on it
  alone gets `native_theme::icons::load_icon` returning `None` for **every**
  icon; gpui forwards the four icon features.

### 1.4 Why iced's case is architecturally worse, and what that forces

gpui-component takes 138 colour fields. iced takes six colours, each expanded
to a `base` / `weak` / `strong` triple, and its widget catalogs read that
triple. There is no hook to replace a catalog: `Catalog for Theme` fixes the
default styles, and `Theme::Custom` carries only a palette.

So in iced a slot with nine readers **cannot** be made right for all nine by
any palette value. The only exact seam is per-widget: every iced widget takes
`.style(impl Fn(&Theme, Status) -> Style)` (`button.rs:184`,
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
| Patch more palette slots | Rejected as the whole answer. A slot with nine readers has no single right value; `background.weak.color` is read by the scrollbar rail *and* the switch track *and* the menu panel, which are three different platform colours. Patching harder moves the error around. |
| **Two layers: correct the palette where a slot has one meaning, and add a `styles` module of per-widget style functions for everything else** | **Chosen.** It matches what iced's architecture allows and what the gpui connector already does for sizes. The palette stays the approximate default for an application that styles nothing; `styles::*` is exact for an application that opts in. Each function is a pure builder from `&ResolvedTheme`, like `geometry::*`. |
| Only add `styles`, leave the palette alone | Rejected. It would leave the invisible placeholder (§1.3, 1.15:1) in place for every application that does not opt in, which is an accessibility failure, not a preference. |
| Ship a wrapper widget set (`native_theme_iced::button(..)`) | Rejected. It would duplicate iced's widget API, age badly against it, and force an application to rewrite its view code rather than add one call. |

The palette changes that follow from this are exactly those where the slot has
a single meaning: stop writing a button *surface* into a slot iced reads as
placeholder *text*, and write `.strong` where `.strong` means "hover" for a
slot we do override. The rest moves to `styles`.

### 2.3 What to automate, and what to leave to a human

The four findings of §1.1 are the specification for this. Ordered by what they
would have caught, cheapest first.

| Layer | Catches | Why this and not something else |
|---|---|---|
| **1. Mapping contracts** — one data-driven test per connector: every toolkit slot, the native field of the widget that reads it, over all 32 preset/mode combinations, plus a coverage tripwire so a slot cannot be added without a declared source | the whole class of §1.2: `accent`, the iced seven, and the next one | It is the only layer that states what "correct" *means*. Everything else checks consequences. |
| **2. Showcase self-tests** — the showcases gain `test = true` and `#[cfg(test)]` modules that render every tab headlessly, assert each resizable group has drag slack, and click the interactive controls | the two interaction findings | Both were *showcase* bugs, not library bugs, so a library test could not have caught either. The showcase is the only place they live. |
| **3. Theme-wide invariants** — WCAG contrast for every text-on-background pair the theme produces, over all 32 combinations | iced's 1.15:1 placeholder, and any future one | Needs no baseline, no rendering and no human: it is arithmetic over the resolved theme. |
| 4. Screenshot diffing | clipped text, overlap, layout collapse | **Deliberately not in this release** (§4). It is the only layer that needs pinned fonts, fixed DPI, per-platform baselines and a perceptual threshold, and it is the one that fails for reasons unrelated to the change. Layers 1–3 make it mostly unnecessary. |

### 2.4 How the contract test states a contract

| Option | Verdict |
|---|---|
| Assert each slot in its own `#[test]` | Rejected: 138 tests in gpui, and adding a slot means remembering to add a test. |
| **One table of (slot, native field, presets where they may legitimately differ), iterated over every preset and mode** | **Chosen.** The table *is* the documentation of the mapping, in one place, in the connector's own source. A reader sees the whole contract at once, and a reviewer sees a change to it in the diff. |
| Generate the table from the mapping code | Rejected: it would assert that the code does what the code does. The table's value is that a human wrote down the intent separately. |

The coverage tripwire is what makes it honest: every field of `ThemeColor`
(and every iced palette slot) appears either in the table, in a list of
derived values with the derivation named, or in a list of deliberate
upstream-defaults with the reason. A new upstream field fails the build until
someone classifies it — the same mechanism as E17's `Theme` shape tripwire,
applied to meaning rather than shape.

### 2.5 Why the showcase tests live in the showcase

An `examples/` target cannot be imported by an integration test. The options
were to move each showcase into its crate as a feature-gated module, or to set
`test = true` on the example target and put `#[cfg(test)]` tests in the file
itself. The second is chosen: it costs one manifest line per showcase, keeps
the showcase a single self-contained file that an application author can read
as an example, and `cargo test` picks it up with no workflow change.

What this can and cannot do differs by toolkit, and the specification says so:
gpui renders headlessly on its test platform, so a tab can be laid out and a
control clicked; iced has no headless renderer in 0.14, so its showcase tests
can build the view tree and assert its shape but cannot draw it.

### 2.6 Contrast: which pairs, and which threshold

WCAG AA is 4.5:1 for normal text and 3:1 for large text. The connector already
has `contrast_ratio` and uses it for status foregrounds
(`colors.rs:55-76`). The options were to assert AA everywhere, or to assert it
only for pairs a platform actually controls.

Chosen: assert AA for every pair the *connector produces*, and record a small
list of exceptions with a reason — a disabled control is deliberately below
AA, and a platform's own choice is not ours to override. The test's job is to
catch a pair we *derived* wrongly, not to grade the platform.

---

## 3 -- Decision record

| # | Decision |
|---|---|
| C1 | All of this ships in v0.5.9, under the maintainer's standing rule that every pre-1.0 release fixes the bugs found during it (§2.1). The delay to the gpui compatibility fix is accepted and recorded. |
| C2 | The iced connector gains a `styles` module of per-widget style functions built from `&ResolvedTheme`, mirroring gpui's `geometry` and `variants` (§2.2). |
| C3 | The iced palette stops writing a button surface into `secondary.base.color`, which iced reads as placeholder text, and writes `.strong` where `.strong` means hover (§2.2). |
| C4 | `native_theme_iced::to_theme` takes `&AccessibilityPreferences`, like its gpui sibling; a breaking signature change, documented under Breaking Changes. |
| C5 | The iced connector forwards the four icon features, so an application depending on it alone gets working icons. |
| C6 | Layer 1: a mapping-contract table per connector, iterated over all 32 preset/mode combinations, with a coverage tripwire over every slot (§2.4). |
| C7 | Layer 2: both showcases get `test = true` and self-tests; gpui renders and clicks, iced builds and inspects (§2.5). |
| C8 | Layer 3: WCAG AA contrast over every pair the connector produces, with a listed exception set (§2.6). |
| C9 | Screenshot diffing is not in this release (§4). |
| C10 | The gpui connector's `accent` fix (sibling E20) is re-stated as a contract-table row rather than two bespoke tests, so it is covered by the same mechanism as everything else. |

---

## 4 -- Deliberately not done, and the trigger to revisit

| Item | Trigger |
|---|---|
| Screenshot diffing with perceptual thresholds | A defect that Layers 1–3 provably cannot express — clipped text, overlapping elements, a collapsed layout. Needs pinned fonts, fixed DPI and per-platform baselines first. |
| An iced `geometry` module (menu row height, dialog paddings, control min-heights — 34 builders in gpui, 5 values in iced) | The next iced release. This one fixes colour meaning, not the geometry gap; doing both at once would double a release that is already carrying two bodies of work. |
| Upstream fixes for the tokens gpui-component reads wrongly (`Toggle` pressed, the Outline tab hover, the checkbox border, the internally built ghost buttons) | Upstream accepting the token proposals recorded in `docs/todo.md`. |
| A shared contract vocabulary across connectors (one table, two backends) | A third connector. With two, the duplication is smaller than the abstraction. |

---

## 5 -- Open questions for the maintainer

1. §2.2 gives an application two ways to get native colours in iced: the
   palette (automatic, approximate) and `styles::*` (explicit, exact). The
   showcase should demonstrate the explicit path, since that is what the
   connector recommends — but that makes the iced showcase stop being a
   demonstration of what an application gets *by default*. The gpui showcase
   resolved the same tension by showing upstream's default and documenting the
   remedy. Proposal: iced's showcase uses `styles::*` throughout and keeps one
   section styled by the palette alone, labelled, so both are visible.
