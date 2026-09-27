# v0.6.0 — egui Connector: Rationale

Status: Implemented (2026-09-27, plan Tasks 0–43) and archived; see *As built* at the end.
Crate: `connectors/native-theme-egui`
Target toolkit: **egui 0.36.2**
Companion specification:
[`todo_v0.6.0_egui-connector-spec.md`](todo_v0.6.0_egui-connector-spec.md);
implementation plan:
[`todo_v0.6.0_egui-connector-plan.md`](todo_v0.6.0_egui-connector-plan.md)
Revised: 2026-09-25 — re-verified against egui 0.36.2 and native-theme 0.5.9
(`HEAD` `2db686ee`).

---

## 0 -- What this document is for

The specification is the WHAT and the HOW. This is the WHY. It exists so that a
maintainer opening this repository in three years can answer three questions
without re-deriving anything:

1. Why is the connector shaped like this and not like the two sibling
   connectors?
2. Which alternatives were considered, and what specifically killed each one?
3. Which of the losses are egui's fault, which are native-theme's,
   and which are deliberate refusals — and what would close each?

Everything settled here is settled. A question re-opened without new evidence
from source is a question already answered below.

**Reading contract.** Every factual claim about egui, epaint, ecolor, emath,
eframe, egui_extras or egui_kittest is cited as `path:line` relative to that
crate's own root, at version **0.36.2**. Every claim about this repository is cited relative to the
repository root, at `2db686ee`. egui 0.35.0 and 0.36.1 appear only as churn
evidence and as the reason for the version floor — §2 (Options D and E), §3.10,
§3.17, §4.2, §4.4, §5.1 and §7 — and are never a target; a line number that
belongs to one of them, or to an older commit of this repository, says so in
words and is not a live citation. Anything that could not be established from source is
marked **UNVERIFIED**, together with what would verify it. An honestly marked
hole is correct; a plausible fabrication is a total failure.

**The claim this document defends.** Not "full theme geometry". The headline is
the specification's §0.1 sentence: *an excellent global theme out of the box and
opt-in per-widget geometry*. §1 is the proof that this is the strongest honest
claim available against egui 0.36.2, and §4.2 names the upstream change that
would make the stronger claim true.

---

## 1 -- The core tension, argued from evidence

### 1.1 The two shapes, measured

`native_theme::theme::ResolvedTheme` (`native-theme/src/model/resolved.rs:156`)
is `defaults` + `text_scale` + **25 per-widget structs** in declaration order
(`resolved.rs:164-212`: window, button, input, checkbox, menu, tooltip,
scrollbar, slider, progress_bar, tab, sidebar, toolbar, status_bar, list,
popover, splitter, separator, switch, dialog, spinner, combo_box,
segmented_control, card, expander, link). Flattened to leaf scalars — with
`ResolvedFontSpec` (6 leaves including `defined_size`,
`native-theme/src/model/font.rs:242-273`, 21 instances),
`ResolvedWidgetBorder` (8 leaves: `color`, `corner_radius`, `line_width`,
`shadow_enabled` and the four `ResolvedPadding` sides,
`native-theme/src/model/border.rs:233-246`, sides at `:195-204`, 18 instances)
and `defaults.border`'s `ResolvedDefaultsBorder` (6 leaves, no padding,
`border.rs:212-225`) expanded — that is 478 leaves, plus the four `LayoutTheme`
fields (`native-theme/src/model/widgets/mod.rs:896-913`): **482**. The count is
measured, not derived: a resolved `kde-breeze` light `ResolvedTheme` serialised
and its scalar leaves counted.

`egui::Style` (`egui/src/style.rs:244`) has 17 fields in a debug build and 16 in
release, because `Style::debug` is `#[cfg(debug_assertions)]` (`style.rs:323-324`).
All widget appearance flows through `Visuals::widgets: Widgets`
(`style.rs:1030`, struct `:1250`): **five** entries — `noninteractive` `:1255`,
`inactive` `:1258`, `hovered` `:1263`, `active` `:1266`, `open` `:1269` — each a
`WidgetVisuals` (`style.rs:1290`) with exactly **six** fields: `bg_fill` `:1295`,
`weak_bg_fill` `:1300`, `bg_stroke` `:1305`, `corner_radius` `:1308`,
`fg_stroke` `:1311`, `expansion` `:1319`.

Thirty distinct colour-and-geometry slots for twenty-five widgets. That ratio is
the whole design problem, and no amount of API taste makes it go away.

### 1.2 The asymmetry, stated exactly

`Widgets` is indexed by **interaction state**. It has **no widget-type axis**.
This is not a gap in the documentation; it is observable in the dispatchers.
`Widgets::state` (`egui/src/widget_style.rs:94-99`) takes a `WidgetState` and
nothing else, and `WidgetState` has four variants — `Noninteractive`,
`Inactive`, `Hovered`, `Active` (`widget_style.rs:84-90`). `Response::widget_state`
(`:105-115`) computes that value from the response's sense, pointer, focus and
highlight state alone. Nothing anywhere
in that path knows whether it is styling a button or a text field.

The consequence is concrete and unavoidable: a `Button` and a `TextEdit` in the
same `Ui` cannot have different corner radii from theme data. The button reads
`widget_style.rs:161`; the text edit reads
`widgets/text_edit/builder.rs:744`, `:749`, `:754`. Same field, same value.

### 1.3 The contested-field evidence — the actual proof

A field is **contested** when two or more native-theme leaves need different
values in it at the same time. A contest cannot be resolved by choosing better
defaults, by writing more careful code, or by any refactor internal to the
connector: it is a cardinality fact. The complete list, with every field's
claimants and their count, is specification §5.11; these are the ones that
decide the architecture:

| egui field | decl | worst pair |
|---|---:|---|
| `visuals.widgets.<state>.bg_stroke` | `style.rs:1305` | `separator.line_width` vs `card.border.line_width` |
| `spacing.interact_size.y` | `style.rs:409` | `toolbar.bar_height` vs `button.min_height` |
| `visuals.selection.bg_fill` | `style.rs:1196` | `progress_bar.fill_color` vs `defaults.selection_background` |
| `visuals.widgets.<state>.corner_radius` | `style.rs:1308` | `button.border.corner_radius` vs `checkbox.border.corner_radius` |
| `visuals.widgets.noninteractive.bg_stroke` | `style.rs:1305` | `separator.line_color` vs `list.grid_color` |
| `visuals.widgets.inactive.fg_stroke.color` | `style.rs:1311` | `checkbox.indicator_color` vs `button.font.color` |
| `visuals.widgets.noninteractive.fg_stroke.color` | `style.rs:1311` | `tooltip.font.color` vs `status_bar.font.color` |
| `visuals.widgets.hovered.weak_bg_fill` | `style.rs:1300` | `menu.hover_background` vs `button.hover_background` |
| `visuals.disabled_alpha` | `style.rs:1126` | any two per-widget `disabled_opacity` values |
| `visuals.window_fill` | `style.rs:1063` | `tooltip.background_color` vs `menu.background_color` |
| `visuals.panel_fill` | `style.rs:1072` | `sidebar.background_color` vs `status_bar.background_color` |

One row has a per-call escape hatch, named here so that a reader does not
conclude none exists anywhere: `Button::corner_radius`
(`egui/src/widgets/button.rs:200-203`) overrides the style-derived radius at a
single call site (applied at `:349-350`, over the `Frame` that
`Style::button_style` produced). It does not carry a second claimant *from theme
data* — the application has to pass the number itself — so it changes nothing
about the contest above; it is Option E's residue (§2), not a resolution.

`spacing.interact_size.y` deserves its own sentence, because it is the single
most contested `Spacing` field in egui. It must simultaneously be
`button.min_height`, `combo_box.min_height`, `menu.row_height`,
`tab.min_height`, `list.row_height`, `expander.header_height`,
`toolbar.bar_height`, `segmented_control.segment_height`,
`switch.track_height`, `slider.thumb_diameter` (through the inversion in
specification §6.6), `progress_bar.track_height`, `spinner.diameter` and
`spinner.min_diameter`. On
macOS Sonoma `toolbar.bar_height` is `38.0` and `button.min_height` is `22.0`
(`native-theme/src/presets/macos-sonoma.toml:208`, `:67`). No arithmetic
reconciles them, because there is nothing to reconcile: they are two different
numbers that must both be written into one `f32`.

Three of those claimants — `toolbar.bar_height`, `menu.row_height` and
`list.row_height` — are `Option<f32>` after resolution
(`native-theme/src/model/widgets/mod.rs:452`, `:210`, `:521`), and `None`
means the platform states no such size: KDE Breeze states no toolbar height
and no menu or list row height (`native-theme/src/presets/kde-breeze.toml:204`,
`:180`, `:193` record why). A `None` claimant contests nothing; it leaves the
field to the claimants that are stated, or to egui's own value when none is
(§3.23). The contest is a fact about the presets that *do* state both numbers,
and macOS Sonoma is one.

A separate egui behaviour compounds it: `Checkbox` and `RadioButton` derive
their whole minimum size from that one number through
`Vec2::splat(interact_size.y)` (`egui/src/widgets/checkbox.rs:85-86`,
`egui/src/widgets/radio_button.rs:54-55`), so writing it also sets a width
floor on both. It is not the sink for `checkbox.indicator_width`, which is
`Spacing::icon_width` (§7, row 1).

`visuals.widgets.<state>.corner_radius` has the same shape in radii. It is
claimed by `defaults.border.corner_radius`, `switch.track_radius` and the
`border.corner_radius` of `button`, `checkbox`, `segmented_control`, `input`,
`combo_box`, `list`, `card`, `tab` and `expander`; the other nine widget radii
reach `visuals.window_corner_radius`, `visuals.menu_corner_radius` or an
`egui::Frame` instead, or nothing (specification §5.11).

**This is the proof.** A connector that owns exactly one `egui::Style` can
deliver at most one of each contested pair. It is not a matter of effort. The
only way to carry more than one value is to have more than one `Style` — which
means the architecture must be *some* form of per-scope style substitution.
Everything else in this document follows from that.

### 1.4 The second reach problem, which changes the answer again

Having accepted per-scope styles, the obvious model is "wrap the widget in a
scope". That model is **false for exactly the containers where a distinct look is
most expected**.

`Area::Prepared::content_ui` builds its `Ui` with a bare `UiBuilder::new()` —
no `.style(..)` (`egui/src/containers/area.rs:611-629`) — and `Ui::new` then
falls back to `ctx.global_style()` (`egui/src/ui.rs:136`). Everything built on
`Area` — `Window`, `Popup`, `Tooltip`, `Modal`, menus, `ComboBox` popups — reads
the **Context** style and ignores the calling `Ui` entirely. Wrapping a scope
around a `Window::show` has literally no effect on that window.

This is why the architecture has *three* seams and a fourth non-`Style` carrier
rather than one, and why `Surface` exists beside `Role` (§3.3).

### 1.5 One grading, and why the second one was dropped

Every mapping row carries one verdict: what this crate carries for that leaf
against egui 0.36.2 (specification §2). The totals are specification §5.7's,
generated from `mapping.toml` when it is written, and no other place restates
them: a number copied into several documents drifts, and only the manifest is
held by the tests.

An earlier revision printed two aggregates: a *matrix* one, grading each leaf by
what egui 0.36.2 could express, and an *effective* one, grading it by what this
crate does under its locked decisions. They differed in one leaf,
`list.header_font.size`, which egui could carry through a `TextStyle::Name` key
and this crate carries per call instead, because it adds no such key (§3.11).
The split was dropped because a verdict is read by an application author asking
what reaches the screen, and only the second grading answers that; the first
described a crate nobody ships, and every reader of the numbers had to ask
which of the two was meant. What the matrix grading carried is kept where it
belongs: each declined route is named in the rows it touches — for this leaf,
"egui could carry this through a `TextStyle::Name` key; declined, specification
§5.8" — and §5.8 lists only genuine declines.

Grading by what the crate carries settles two shapes a reader might otherwise
dispute. A base owner — the leaf whose value a contested field's one global
write holds (specification §5.9) — is DIRECT when that write renders it on its
own widget, whoever else claims the field: `button.background_color` in the
base style's `inactive.weak_bg_fill` is shown by every unscoped `Button`,
`scrollbar.thumb_color` in its `inactive.bg_fill` by every unscoped
`ScrollArea`'s handle (`egui/src/containers/scroll_area.rs:1457-1466`,
`:1499-1503`), and `defaults.background_color` by every unscoped panel. The list of such rows
lives in specification §2 only. And a per-widget weight or slant, which no
`Style` field carries, reaches the screen per call — `role_font_weight` or
`role_font_is_italic` handed to `RichText` (§3.11) — and a per-call input the
stock widget takes is a route graded DERIVED (specification §2). Such a route
may need the widget's interaction state read back first — its response from
`Context::read_response`, since egui settles interaction at the start of the
pass from the previous pass's widget rects (`egui/src/context.rs:1350-1355`) —
and what the call hands over is still the stock widget's own builder input;
painting a widget of one's own stays outside the grade, and a row whose value
only the companion widgets crate paints that way is tagged `widgets-crate`
(specification §2).

### 1.6 What "full theme geometry" can and cannot mean here

Three readings, and the crate is honest about which one it delivers
(specification §1.4):

1. **Every mappable leaf reaches some egui object.** Reachable, and delivered:
   a base style, a role style, a surface frame, or a per-call route with the
   accessor or leaf the specification names (its §2 route rule).
2. **Every mappable leaf renders correctly with no application cooperation.**
   Not reachable in 0.36.2. There is no hook that could make it so — §1.2–§1.4
   are the proof, and §4.2 is the upstream change that would create one. What
   an application that calls `install()` and nothing else does get is the
   DIRECT rows — every base owner whose one global write renders it
   (specification §2) — `button.font.size` on the four widgets that read
   `text_styles[Button]` (§4.4), the DERIVED values the base style carries, the
   line spacing among them (§3.12), and what the install plugin adds every pass
   with no application code: the OS colour scheme where the integration reports
   none, and the focus ring around the keyboard-focused widget (§3.5, §3.7).
3. **Every leaf renders correctly.** Not reachable in any egui today: the
   UNMAPPABLE rows have no expression at all, and the `source-side gap` among
   them are native-theme's to close, not egui's (§4.3). The structural zeros
   that were once native-theme's own holes have left the model (§4.3).

iced has a seam that could carry (2): `iced::Theme` is consulted through
per-widget `Catalog` traits, so a wrapper theme implementing all of them could
style every widget with no cooperation. The iced investigation declined that
shape for its coupling to iced internals
(`docs/todo_iced-full-theme-geometry.md:64-78`), and the iced connector
delivers per-widget geometry through opt-in `styles` functions the application
passes to each widget (`connectors/native-theme-iced/src/lib.rs:43-53`) — so it
does not promise (2) either, but it declined a seam that exists. The gpui connector could aim at it only if
gpui-component accepts the per-widget `ThemeConfig` metrics its own
investigation proposes — an unsubmitted draft
(`docs/todo_gpui-full-theme.md:177-197`, status at `:235-236`) — so today
it cannot promise (2) either. egui has no equivalent seam even to propose
against, because `Style` has no widget-type axis at all. Promising (2) here would
be a lie, and the phrase "full theme geometry" is therefore banned from the
crate's own claims and confined to naming the goal in §4.2.

---

## 2 -- Options considered

House convention (`docs/todo_iced-full-theme-geometry.md:21-92`): every
option gets a fair statement of its appeal before the reason it loses.

### Option A: Global `Style` only, no per-widget mechanism (rejected)

Map the theme onto one `egui::Style`, install it with `Context::set_style_of`
(`egui/src/context.rs:2250`), ship the rest as documentation. This is what a
first-cut egui connector looks like, and it is the shape both sibling connectors'
`to_*_theme` entry points have (`connectors/native-theme-iced/src/lib.rs:305`,
`connectors/native-theme-gpui/src/lib.rs:328`).

**Appeal.** Tiny API — one type, one function. Nothing to learn. No scoping
noise at any call site. Impossible to misuse. Every widget in the application is
themed uniformly with zero cooperation, which is precisely the property (2) of
§1.6 that the chosen design cannot promise.

**Rejected because** it can carry exactly one claimant per contested field.
§1.3's table is the arithmetic: only the elected winner of each contested field
survives — its base owner, DIRECT by specification §2 wherever that one write
renders it — and every other claimant is silently dropped —
not because it is hard to carry, but because there is nowhere to put it. The
failure is invisible — the application renders, it just renders `button.hover_background` on
the tabs and `button.min_height` on the toolbar. This is the "plausible
fabrication" failure mode the project's rules forbid, applied to geometry
instead of to prose.

Note carefully what is *not* rejected: Option A **is** the base layer of the
chosen design. The base `Style` is exactly this, and the DIRECT leaves
(specification §2) — each contested field's elected winner among them wherever
its one write renders it — reach the screen through it with no cooperation at all. What is rejected is stopping there.

### Option B: Ship replacement widgets in the connector (rejected)

The connector ships `native_theme_egui::Button`, `::Checkbox`, `::TextEdit`, …,
each reading `ResolvedTheme` directly and painting with `Ui::painter`. This is
the gpui investigation's Option B/F shape
(`docs/todo_gpui-full-theme.md:239-272`, `:371-498`).

**Appeal.** Total fidelity. Every one of the 482 leaves is reachable, because
the connector owns the paint code. No contest exists, because there is no shared
`Style` to contest. It is the only design that could truthfully claim (2) *and*
(3) from §1.6 simultaneously.

**Rejected because** of what a widget costs. egui's `Button` is not a rectangle:
it is `AtomLayout` composition, image auto-sizing clamped to font height
(`egui/src/widgets/button.rs:105`, applied `:311-313`), a `WidgetInfo`
accessibility payload, the `SELECTED_CLASS` interaction with
`Style::button_style` (`widget_style.rs:146`, `:150-155`), and the shortcut-text (`button.rs:225-237`) and
frame-when-inactive (`:364-368`) branches. Reimplementing that
buys a permanent per-release audit obligation on behaviour that has nothing to
do with theming, in a crate whose entire remit is theme *mapping*. Worse, a
replacement widget is only used by applications that switch to it, so the design
purchases its fidelity by giving up the property that made Option A attractive:
plain `ui.button(..)` would go back to being unthemed.

The rejection is recorded permanently as the no-widgets charter (specification
§14.3) with an explicit two-test predicate, so that it does not have to be
re-argued every release. `Switch` shows the predicate deciding: it has no egui
counterpart, so it passes the first test, and it fails the second — with its
substitute drawn as a `Button` carrying `.selected(..)`, which paints the
unchecked track a native switch shows (§3.22), only a minority of its leaves
stay UNMAPPABLE (specification §5.3 grades each). A predicate over counted leaves is
settled by recounting, not by argument.

What is rejected is widgets *in the connector*, not the widgets. Where egui has
no widget with the platform's visual identity, the native look is supplied by a
separate companion crate, `native-theme-egui-widgets`
([`todo_egui-widgets-spec.md`](../todo_egui-widgets-spec.md) §0.1, milestone
undecided), which draws from the connector's `ThemeAtlas` and ships `Switch`
among its widgets. The per-release audit obligation then lives in a crate whose
remit it is, the connector stays a mapper, and an application that never adds
that crate still gets everything this document describes.

### Option C: Fork egui (rejected)

Vendor egui, add the widget-type axis, publish the fork, depend on it.

**Appeal.** Everything becomes possible immediately, with no upstream
negotiation and no waiting.

**Rejected** on three independent grounds, any one of which is sufficient.
First, the connector's whole value is that the application's `egui::Style` is
*our* `egui::Style`; a fork guarantees two incompatible egui crates in one
dependency graph as soon as the application depends on egui itself, and the
compiler then reports `expected egui::Style, found egui::Style`.
Second, a fork must be re-synchronised with every upstream release forever, by
one maintainer. Third, it would make the crate's own eventual upstream
contribution (§4.2) pointless, because a fork is the argument *against* fixing
it upstream.

### Option D: Build on `egui::widget_style` as the extension point (rejected)

`egui::widget_style` (`pub mod` at `egui/src/lib.rs:426`) declares `WidgetStyle`,
`ButtonStyle`, `CheckboxStyle`, `LabelStyle`, `SeparatorStyle`, `Classes`,
`HasClasses` and `WidgetState`, and it is genuinely consumed by
`widgets/button.rs:331`, `widgets/checkbox.rs:83` and `widgets/separator.rs:109`.
It reads like the widget-type axis §1.2 says egui lacks — a per-widget-kind
style resolution layer with a class system attached.

**Appeal.** If it were an extension point, the entire contested-field problem
would dissolve: register a per-class override, get per-widget styling from one
`Style`, and `native_scope` would never need to exist.

**Rejected because it is not an extension point, and it is not evolving into
one.** Four verified facts, each independently fatal:

* `Style::widget_style` (`widget_style.rs:120`), `button_style` (`:146`),
  `checkbox_style` (`:174`), `label_style` (`:194`) and `separator_style`
  (`:212`) are **inherent methods on `Style`**. There is no trait, no
  `dyn StyleEngine`, no registration function — nothing a downstream crate can
  implement or install.
* `widget_style` and `separator_style` **ignore their `_classes` argument
  outright** — the parameter is literally named `_classes` at `:120` and `:212`.
  Only `button_style` reads a class, and only one: the hardcoded
  `SELECTED_CLASS` (`:150`, declared `:225`).
* `separator_style` hardcodes `spacing: 6.0` (`:215`), so even the widget that
  does route through the module has an untuneable geometry constant in it.
* **`egui/src/widget_style.rs` is byte-identical between egui 0.35.0 and
  0.36.1**, verified by `diff`, and 0.36.2 changes exactly one line of it — the
  `Display` impl for `Classes` now names `core::fmt::Display` instead of
  `std::fmt::Display` (`widget_style.rs:259`), part of that patch release's
  crate-wide `std::` → `core::` path churn. From 0.35.0 to 0.36.2 — one minor
  and two patch releases — it did not gain a single line of behaviour.

The last point is the decisive one. A module that changed between releases might
be worth betting on. A frozen module is not — and a connector that shipped
against `Classes` would be shipping against an API that upstream has, so far,
not moved.

The same fact is the strongest possible argument *for* a well-argued upstream
PR, because nothing in flight competes with it. That is §4.2, and the
non-negotiable rider there is that **the connector must not depend on it**.

### Option E: A runtime data table consulted per widget (rejected)

Keep one `Style`, put the 482 mapped values in a table inside the connector, and
have the application ask the table for what it needs at each call site —
`ui.add(egui::Button::new("x").corner_radius(atlas.button_corner_radius()))` and
so on.

**Appeal.** No `Style` cloning, no per-role allocation, complete data available
at every call site, and the table is trivially testable in isolation.

**Rejected because** egui's builders do not accept most of the values: for the
great majority of the 482 leaves there is no per-call setter at all — no
`TextEdit::border_width`, no `ComboBox::padding`. The partial exceptions are
real, and narrow. `Button` takes `fill` (`egui/src/widgets/button.rs:143`),
`stroke` (`:151`), `min_size` (`:193`), `corner_radius` (`:200-203`, applied
over the style-derived radius at `:349-350`) and `gap` (`:277`); `TextEdit`
takes a whole `Frame` (`egui/src/widgets/text_edit/builder.rs:306`), `margin`
(`:313`) and `min_size` (`:410`), which egui 0.36.2 honours in both axes
(`:498`, `:715`; 0.36.1 read only its `x`, and let the available width
override even that). It is why per-position segmented-control radii, which the
model does not state (`docs/todo.md`, *Segmented control: the join and the
divider*), could reach a segment only through `Button::corner_radius` at each
call site (specification §5.3, row `segmented_control.border.corner_radius`). Everything
else egui reads, it reads from `Style`, which is the whole point of §1.2.
A table would therefore be a table of numbers the application must apply with
`Ui::painter` by hand — which is Option B without the widgets, i.e. all of the
work and none of the rendering.

The useful residue of this option is kept, narrowed: the free accessors of
specification §4.7 are this table, restricted to values that no `egui::Style`
field carries **for that widget** and that need more than a field read — a
conversion, the text-scaling factor, a choice between leaves, the crate's font
rule — or that a sibling connector also offers (§3.1). `role_font_weight()`
covers leaves with no sink at all, because `FontId` is a size and a family and
nothing else (`epaint/src/text/fonts.rs:21-28`); `expander_icon()` covers
`expander.arrow_color`, which loses an intra-widget contest to the label on the
one colour both are painted with (`containers/collapsing_header.rs:353`,
`:598`) and returns through the per-call `CollapsingHeader::icon` (`:480`). A
leaf that is already a plain number or colour is read from the `ResolvedTheme`
itself: `list.row_height`, which `egui_extras::Table` takes as a **call
argument** rather than from `Style` (specification §5.4), and
`dialog.max_width`, whose `Style` sink exists and is refused (specification
§5.8). Those are genuinely the application's to pass. That is the difference between a table as the architecture (rejected) and
a table as the honest remainder (adopted).

### Option F: Helpers only — document the mapping, install nothing (rejected)

Publish the 482-row mapping as documentation plus accessors and let the
application build its own `Style`.

**Appeal.** Zero API risk. Nothing to keep in sync with egui except the
documentation.

**Rejected** for the reason the iced investigation rejected the same shape
(`docs/todo_iced-full-theme-geometry.md:80-92`): every application would
duplicate the same boilerplate, and the connector's entire job is to bridge
`native-theme` to the toolkit. Handing back a list of numbers is not a bridge.
It also fails the project's own no-hardcoded-values rule in practice: an
application assembling a `Style` by hand from an accessor list will fill the
gaps with literals.

### Option G: Pre-built per-role styles over egui's own seams, with an audited manifest (chosen)

One `ThemeAtlas` compiles, once, per `egui::Theme`:

* one base `egui::Style` — Option A, and it is what every unwrapped widget gets;
* one `Arc<egui::Style>` per (`Role`, `RoleVariant`) — 25 roles × 3 variants;
* one `egui::Frame` per `Surface` — 11 entries with panel sides expanded;

delivered through the three substitution points egui already has (specification
§3.2), plus `egui::Frame` values as a fourth, non-`Style` carrier. Every one of
the 482 leaves gets a row in a checked-in `mapping.toml` carrying its verdict and
its declared egui sinks, and a differential headless test proves the manifest
describes the code rather than the document (specification §13 T2, T3).

**Why it wins.**

* It is the only option that carries more than one claimant per contested field
  **without** owning any paint code (§1.3 makes multi-`Style` mandatory; Option B
  is the only alternative that achieves it).
* Every mechanism it uses is upstream API that egui uses itself. Nothing is
  invented: `UiBuilder::style` (`ui_builder.rs:155`, field `:28`) consumed by
  `Ui::scope_builder` (`ui.rs:2194`) and `Ui::new_child` (`ui.rs:237`);
  `Ui::set_style` (`ui.rs:387`); `egui::style::StyleModifier` (`style.rs:194`)
  consumed by `Popup::style` (`containers/popup.rs:417`), `MenuConfig::style`
  (`containers/menu.rs:107`), `MenuBar::style` (`:241`) and `ComboBox::popup_style`
  (`containers/combo_box.rs:199`).
* It degrades to Option A exactly, and gracefully: an application that installs
  and scopes nothing gets a correct global theme, and `native_scope` on a
  `Context` with no atlas is a plain `ui.scope(..)` (`ui.rs:2186`) that never
  panics and never returns an `Option`.
* It keeps the two churning sides separate. `Role` tracks `ResolvedTheme`'s
  widget list and nothing else; `Surface` tracks egui's `Frame` attachment points
  and nothing else. §5 is the argument that this is what makes the design
  survivable.

**What it costs, stated up front.** Per-call-site noise where fidelity is wanted
(`ui.native_scope(Role::Input, RoleVariant::Normal, |ui| ..)`); a memory footprint, measured at
about 201 KiB at most per atlas (§8, Q-1); and the honest admission that fidelity is opt-in, which
is ledger item 22 and the reason the README's first paragraph is the sentence in
specification §0.1.

### 2.8 Alternatives inside the chosen shape, and why each lost

These were live proposals during design. Each is recorded with its defeating
fact so it is not re-proposed.

| Rejected | Why |
|---|---|
| Naming the handle `EguiTheme` | `egui::Theme` is a real two-variant type (`egui/src/memory/theme.rs:6`, re-exported `egui/src/lib.rs:483`). The crate would ship `fn resolved_for(&self, theme: egui::Theme)` and `fn surface_frame(&self, theme: egui::Theme, ..)` *on a type called `EguiTheme`*. `ThemeAtlas` names what the value is — a compiled table indexed by two axes — and no signature in the crate is then ambiguous |
| `Surface::{SidePanel, TopPanel, BottomPanel, CentralPanel}` | Its own stated rule was "one variant per egui container", yet three of the four were distinguished only by *native-theme* concepts, and none of those three names is an egui 0.36.2 type (a recursive grep over `egui/src` finds no `SidePanel` or `TopBottomPanel` at all, and `TopPanel` / `BottomPanel` only as `UiKind` variants, `egui/src/ui_stack.rs:25`, `:28`; the types are `Panel`, `containers/panel.rs:206`, with `left`/`right`/`top`/`bottom` at `:249`, `:256`, `:265`, `:274`, and `CentralPanel`, `:1187`). `Surface::Panel(PanelSide)` is one variant for egui's one `Panel` type |
| `Role::CheckboxOn` / `Role::CheckboxOff`, `Role::ButtonPrimary` | Encoding widget *state* in a widget-*type* taxonomy forces `let role = if self.notify { Role::CheckboxOn } else { Role::CheckboxOff };` at every call site, and inverting that condition is silent. `RoleVariant` is a *value* — `RoleVariant::from_selected(self.notify)` — so the state stays where the state already lives |
| Re-exporting `Rgba` at the crate root | The crate re-exports `egui`, and `egui::Rgba` (`egui/src/lib.rs:442` → `ecolor/src/rgba.rs:3-10`) is *linear f32, premultiplied* while `native_theme::color::Rgba` is *sRGB u8, straight* (`native-theme/src/color.rs:42-52`). `native_theme_egui::Rgba` beside `native_theme_egui::egui::Rgba` with opposite colour spaces is a trap that survives casual testing. It lives at `convert::Rgba`, next to the function that consumes it |
| A `Metrics` struct instead of free accessors | Adding a field to a public struct is a change; adding a free function is not. Since the whole non-`Style` surface exists precisely because native-theme keeps growing, it must be the shape that grows without a version bump. This also matches both sibling connectors (`connectors/native-theme-iced/src/lib.rs:369-581`, `connectors/native-theme-gpui/src/lib.rs:369-539`, the ranges specification §4.7 cites); gpui's `Native<'a>` (`:622-627`) is the borrowed *input* of its `geometry` builders, not a struct of values |
| A `StyleSet` struct with 26 public fields | Struct-literal-constructible downstream, so adding a native-theme widget breaks every literal; and `pub const ALL: [Self; 25]` bakes a count into a public type signature |
| Declining `#[non_exhaustive]` crate-wide on the grounds that the egui pin already buys compatibility | Wrong axis. native-theme churn is independent of egui churn: two new widget structs force two new `Role` variants with no egui bump at all. The pin buys nothing there. See §5.2 |
| An `EGUI_VERSION: &str` constant | Cannot be checked against the resolved dependency (`egui = "0.36.2"` accepts any `>=0.36.2, <0.37.0`), so it would eventually become a lie. The version policy lives in `Cargo.toml` and in the README's **Compatibility** section, the shape both sibling connectors' READMEs already have |
| Keeping `egui_kittest` out of `[dev-dependencies]` | The first version of this document declined it because its API had never been read, and every specification §13 test group was expressible with `egui` alone. v0.5.9 made that position untenable: contract C7 requires every showcase to carry self-tests that **render and click headlessly** (`docs/archive/todo_v0.5.9_theme-contracts-rationale.md:525`), and clicking a widget found by its label is exactly what a bare `Context` does not offer. The version-aligned 0.36.2 source is now read (`rust-version = "1.95"`, `egui_kittest/Cargo.toml:14`, depending on `egui` 0.36.2 with `default-features = false`, `:119-121`): `HarnessBuilder::build_eframe` (`egui_kittest/src/builder.rs:212`, behind the `eframe` feature) drives the showcase's real `eframe::App`, queries go through `kittest`'s `Queryable` over the AccessKit tree (`egui_kittest/src/lib.rs:963-969`), `Node::click` (`egui_kittest/src/node.rs:61`) queues the pointer events, and `Harness::run` (`egui_kittest/src/lib.rs:356`) steps until nothing is animating or asking to repaint. AccessKit is an unconditional dependency of egui 0.36.2 (`egui/Cargo.toml:82-83`), so nothing beyond `eframe` needs a feature; `Harness::render` (`egui_kittest/src/lib.rs:690-691`) would need `wgpu` or `snapshot` (`egui_kittest/Cargo.toml:61-74`; only `wgpu` brings in `wgpu` 30, `:156-157`), and neither is enabled. §8 Q-3 carries the reasoning; the exact test code is the specification's |
| A direct `epaint` / `ecolor` / `emath` dependency | egui re-exports all three (`egui/src/lib.rs:436-438`), so a direct dependency only creates a way to end up with two `ecolor`s. `skrifa` is the exception and *is* a direct dependency: epaint does not re-export it, and the connector needs its one fallible call to validate the font bytes it now supplies (§3.11) |
| An `InstallReport` return value from `install` | Diagnostics belong to the *atlas*, which is where the conversions happen, not to installation, which cannot fail. `ThemeAtlas::notes()` is populated once per theme change |
| A `refresh_line_spacing(&Ui) -> bool` two-phase protocol | A protocol on the common path whose failure mode is silent degradation. §3.12 |
| A post-pass retint of the shapes through `Plugin::output_hook` | Recorded because it is the one mechanism a contributor will find, prototype and propose. It is **unsound**, not merely fragile: nothing in the output ties a shape to the widget that produced it. The hook (`egui/src/plugin.rs:44`, dispatched at `:181`) is invoked after `end_pass` and before `tessellate` (`egui/src/context.rs:2461`, `:2463-2464`, `:2859`), and what it can reach is `ClippedShape`, which is `{clip_rect, shape}` (`epaint/src/lib.rs:117-124`) — and none of `Shape`'s twelve variants (`epaint/src/shapes/shape.rs:27-71`) records which widget produced it. `WidgetRect` has seven fields and no widget type (`egui/src/widget_rect.rs:9-49`); the only structure that *does* carry a kind, `WidgetRects::infos` (`:105`), is a private field whose sole writer is `#[cfg(debug_assertions)]` **and** gated on `show_interactive_widgets`, with a release body of `_ = (self, id, make_info);` (`egui/src/context.rs:1572-1584`). `Button` paints at `widgets/button.rs:380` and registers its `WidgetInfo` only afterwards at `:391`, so the attribution does not exist even in a debug build at the moment the shape is created. The one per-widget record the output can hold, `PlatformOutput::accesskit_update` (`egui/src/data/output.rs:176`), is filled only while AccessKit is enabled (`egui/src/context.rs:3701-3702`) and carries roles and bounds, not shapes, so pairing it with shapes would be a geometric guess |

---

## 3 -- The decision record

One subsection per locked decision. Each names what was chosen, what was
rejected, and the fact that decided it.

### 3.1 The handle: opaque, `Arc`-backed, found in the `Context` or not at all

**Chosen.** `ThemeAtlas(Arc<AtlasInner>)`, opaque, `Clone`, `Send + Sync +
'static`, carrying both colour schemes and both source `ResolvedTheme`s.
`install` publishes it into the `Context`, `ThemeAtlas::from_ctx(ctx)` finds it
again as an `Option<ThemeAtlas>`, and `ThemeAtlas::clear(ctx)` removes it.
`resolved_for(theme) -> &ResolvedTheme` is total, because every atlas holds
both variants.

**Rejected: a passthrough atlas behind an infallible context accessor** — the
earlier shape, a `native_theme()` that returned `ThemeAtlas::passthrough()`,
whose styles were `egui::Theme::default_style()`
(`egui/src/memory/theme.rs:24-29`), beside a `native_theme_opt()` and a
`NativeThemeContextExt` trait to carry both. It was meant to spare every frame
body a `let Some(..) else` branch, and it bought less than it cost. It removed
the branch only before installing, scoping or asking for a frame — never before
the free accessors, which need a `ResolvedTheme` a passthrough atlas does not
have, so `resolved_for` stayed an `Option` and the branch came back one line
later. And it put a second kind of atlas, one with no theme in it, into a type
whose every other value is a theme, so every method had to say what it means
there. Where the branch is genuinely avoidable the degradation now lives inside
the call: `NativeThemeUiExt`'s methods do egui's own thing when no atlas is
installed (specification §4.5) and the install plugin does nothing. What is left
is one `Option`, in the one place where "nothing installed" is a real answer.
An associated function says that as well as a context-extension trait did,
adds no trait to seal, and cannot shadow a `Context` method of the same name —
the hazard specification §4.5 guards against for `NativeThemeUiExt`.

**Why opaque.** Clause 6 of the semver contract: adding a method to an opaque
type is additive. An atlas with public fields would make every future mapping
refinement a breaking change.

**Why it carries the `ResolvedTheme`s.** Every free accessor of specification
§4.7 takes a `&ResolvedTheme` except five: four take `&SystemTheme`, because
`AccessibilityPreferences` lives there (`native-theme/src/lib.rs:490`) and is
deliberately absent from `ResolutionContext`
(`native-theme/src/resolve/context.rs:20-23`); and `scaled_text_size` takes only a
size and the preferences, as in both siblings. And most leaves have no accessor
at all: the application reads them from the `ResolvedTheme` (below). If the atlas
did not carry it, every application would hold a second theme object beside the
atlas — and an application struct with both is exactly the shape that produces a
borrow-check failure when a `&self` accessor stays live across a closure
capturing `&mut self.field` (`E0502`). `atlas.resolved_for(ui.ctx().theme())` is
one call, on the object the application already has, and it also removes the
`if t == egui::Theme::Dark { ColorMode::Dark } else { ColorMode::Light }` wart
that an atlas keyed only on `ColorMode` would force at every call site.

**Why most leaves have no accessor.** An earlier revision declared a free
function for nearly every leaf no `Style` field carries — `success_color`,
thirteen `switch_*` functions, `dialog_max_size`, `list_row_height` and the like
— most of them one field read and one colour conversion. Each was an item to
document, test and keep in step with native-theme, and none gave an
application anything it lacked: `ResolvedTheme`'s fields are public, the atlas
hands out the right one, and `convert::to_color32` is the one conversion. An
accessor now exists only where it adds meaning — a conversion to an egui type
(`window_title_bar_font`, `input_margin`), the user's text-scaling factor
(`font_size`, `scaled_text_size`), a choice between leaves
(`window_title_bar_text_color`), the whole-crate font rule (`role_font_weight`,
`role_font_is_italic`, `text_role_font`), a closure egui takes
(`expander_icon`) — or where a sibling connector has the same function, so an
application moving between toolkits finds the same name (specification §4.7).
No native value became unreachable: the §5 row of each leaf names the path the
application reads. Two rules came with the reduction. An accessor returns the
leaf's meaning and clamps nothing: `clamp_length` guards a value written into a
`Style` or a `Frame`, where egui's layout arithmetic would turn a huge length
into `+∞` (§3.9), not a value handed to the application's own arithmetic. And
where an accessor and a painted value describe the same thing they agree:
`border_color` returns the colour with `defaults.border.opacity` folded in, as
every stroke the atlas writes has it (specification §6.13), and
`role_font_is_italic` answers `true` only when the role's slant differs from the
installed Proportional face's, so an italic system face is not slanted twice.

### 3.2 Two axes, two moving sides, and state as a value

**Chosen.** `Role` (25 variants, `#[non_exhaustive]`) named in **native-theme's**
vocabulary, exactly one per widget field of `ResolvedTheme` in declaration order
(`resolved.rs:164-212`). `Surface` (8 variants, `Panel` carrying `PanelSide`)
named for **attachment points**, exactly one per point at which an application can
attach an `egui::Frame`. `RoleVariant` (`Normal`, `Selected`, `Disabled`) as a
*value* parameter, not a taxonomy.

**Why menus have no `Surface`.** No egui menu builder takes a frame: egui builds a
menu's `Frame::popup` or `Frame::menu` from the style its `StyleModifier` produced
(`containers/popup.rs:602-603`, `containers/menu.rs:432`), so the menu's fill,
stroke, radius, shadow and margins are fields of the `Role::Menu` cell, which
`MenuConfig::style` already receives (specification §4.4, §5.2). A `Surface::Menu`
beside it would name a frame nobody can attach — a second name for one carrier,
whose rows could only restate the role's.

**Rejected:** one enum covering both, which is what an earlier proposal shipped.
A single enum mixing native widgets, value splits and egui surfaces tracks both
moving sides at once, so *either* upstream forces a variant change. Two enums,
each tracking exactly one side, is what makes §5.1 and §5.2 mechanical.

**Why the names do not decide anything.** Five of `Surface`'s eight names —
`Window`, `Dialog`, `Popover`, `Tooltip`, `Card` — are spelled like a
`Role` variant, and that is harmless: the two enums are separate because they
track two different moving sides, not because they use different words
(specification §4.4). A `Panel` names a side, not a native struct, so the
panel-side-to-native-struct mapping is written down as a convention rather than
inferred from a name
(`PanelSide::Left`/`Right` ← `theme.sidebar`, `Top` ← `theme.toolbar`,
`Bottom` ← `theme.status_bar`, `Surface::CentralPanel` ← `theme.defaults`, with
`layout.window_margin` as its inner margin).

**Why `PanelSide` is exhaustive** while everything else is not: four sides is a
closed fact of 2-D screen geometry, not an API taxonomy that can churn, and
applications benefit permanently from being able to `match` it. Decided in
§8, Q-4.

### 3.3 Three seams and a `Frame` carrier; why styles are pre-built

**Chosen.** All three of egui's substitution points, plus `egui::Frame` values.

Seam S1 (`Context::set_style_of`, `context.rs:2250`) is the global base. Seam S2
(`UiBuilder::style` / `Ui::set_style`) reaches everything laid out inside a `Ui`
**except** `Area`-based containers. Seam S3 (`egui::style::StyleModifier`) is the
only thing that reaches the *whole* style of a popup, tooltip, menu or
`ComboBox` popup, its own frame included. `Window` and `Modal` accept no
modifier, so their bodies take an S2 scope applied inside the closure, which
reaches only the widgets the application adds there.

"All three" is an exhaustiveness claim, so it carries its own evidence rather
than being asserted: a grep for `StyleModifier` across every vendored egui crate
returns **24** hits, all of them in `egui/src/style.rs` and
`egui/src/containers/{popup,menu,combo_box}.rs`; and `Ui::new` — the one
constructor that can seed a style from nothing — has exactly **two** call sites
in the whole tree (`egui/src/context.rs:802`,
`egui/src/containers/area.rs:629`), neither of which supplies a style, which is
what makes §1.4's universal `Area` statement checkable rather than merely
plausible. Nor is there a fourth seam anywhere else a widget's kind could reach a
style. `Plugin` has six hooks (`egui/src/plugin.rs:13-52`: `setup` `:22`,
`on_begin_pass` `:27`, `on_end_pass` `:32`, `input_hook` `:38`, `output_hook`
`:44`, and a `cfg(debug_assertions)` `on_widget_under_pointer` `:50-51`), and
none is handed a widget's kind. Of the 159 `pub fn` in `Context`'s `impl` blocks,
the eight that mention style — `global_style` (`egui/src/context.rs:2175`),
`global_style_mut` (`:2189`), `set_global_style` (`:2200`), `all_styles_mut`
(`:2213`), `style_of` (`:2221`), `style_mut_of` (`:2237`), `set_style_of`
(`:2250`) and the settings UI's `style_ui` (`:3666`) — are global or per-`Theme`
without exception. This census is recorded because a negative this
load-bearing — specification §14 item 22, fidelity is opt-in — is otherwise
re-argued at every egui bump.

The `Frame` carrier is not a `Style` at all, and it is **the only way to theme
container margins**, because five of egui's eight `Frame` presets hardcode their
inner margin and read no `Style::spacing`:
`Frame::group` hardcodes `.inner_margin(6)` (`containers/frame.rs:180`),
`Frame::side_top_panel` `Margin::symmetric(8, 2)` (`:187`),
`Frame::central_panel` `.inner_margin(8)` (`:192`), `Frame::canvas`
`.inner_margin(2)` (`:229`), and `Frame::dark_canvas` (`:236-237`), which
delegates to `Frame::canvas` and so inherits its `.inner_margin(2)`. The other
three are `window` (`:198`), `menu` (`:207`) and `popup` (`:216`), which do read
`Style::spacing`; 5 + 3 = 8. All eight *do* read something from `Style` — even
`Frame::group` takes its corner radius and stroke from
`widgets.noninteractive` (`:181-182`) — so the precise claim is about
`Style::spacing`, and specification §3.2 tabulates what each one reads.

**Rejected:** using only S2 and telling users to "wrap it in a scope". §1.4 is
why. The specification says so loudly, in the doc comment of the very method that
would tempt someone into it (`NativeThemeUiExt::native_scope`), rather than only
in a limitations section.

**Panels take their role on the parent `Ui`, and the central panel gets the
base style back.** A `Panel` paints its separator line from the parent `Ui`'s
style and builds its content `Ui` as a child of that parent (specification §4.4
cites the lines), so the role that owns the line must be live on the parent
before `Panel::show`. A scope around the call would do that and break the
layout: a side panel shrinks the cursor of the `Ui` it is shown in, and a
scope's parent advances past the whole scope, which would push the
`CentralPanel` below the side panel. `native_set_style` restyles the parent in
place instead, and egui's own `Ui::reset_style` restores the `Context`'s style —
application `Name` keys included — before the `CentralPanel`, so the central
content takes the base style with no call of this crate's. A resizable panel's
line is the one a user drags, so it takes the splitter's role and the content
takes the sidebar's as the first statement inside the closure; a fixed panel's
line and content are both its own role's.

**Why pre-built rather than computed per call.** Handing a prepared
`Arc<Style>` to `UiBuilder::style` **moves** the `Arc` — `ui.rs:237` is
`let style = style.unwrap_or_else(|| Arc::clone(&self.style));` — so no `Style` is
copied into the child `Ui`. The caller still pays one refcount bump to produce
the `Arc`; the claim "not even a refcount bump", which appeared in two rejected
drafts, is false and the documents state only the true half. Building on demand
would instead clone a `Style` (including its `BTreeMap`) at every scope, every
frame, in an immediate-mode loop.

**Why `Style` is never built by struct literal.** `Style::debug` is
`#[cfg(debug_assertions)]` (`style.rs:323-324`), so an exhaustive literal would
fail to compile in exactly one of the two profiles. Construction always starts
from `egui::Theme::default_style()` and assigns. That construction *is* the
no-hardcoded-values enforcement: every field the theme does not supply keeps
egui's own value by construction, so a forgotten mapping degrades to stock egui
rather than to a literal somebody typed.

**Why a role cell starts from the base style, not from `default_style()`.** The
base style is the platform's look: its panel and window fills, selection, link,
warning and error colours, window chrome and line spacing come from
`defaults` and the base owners (specification §5.9). A cell built from egui's
default style would drop all of them inside every scope, so a link in a
dialog body or a warning in a tooltip would show egui's colours where the
unscoped UI shows the platform's — the opposite of the goal. Starting from the
finished base style, a scope changes only what its widget states differently
(specification §3.4). The one field set that must not be inherited is the menu
item's: a `Role::Menu` cell replaces egui's `menu_style`, so it applies that
function first and then writes the item's own border, radius and padding
(`menu.border`, specification §5.2), not the button's. The menu popup's frame
takes nothing from `menu.border`: the platform facts give a menu item no border
or shadow of its own and send the popup's to the popover
(`docs/platform-facts.md:1237-1240`), so the frame keeps the base style's
`defaults.border` values.

**One call shape, and no raw style handed out.** Every role call takes the role
and the variant together — `ui.native_scope(role, variant, f)`,
`ui.native_set_style(role, variant)`, `atlas.role_modifier(theme, role,
variant)` — where an earlier revision had a `native_scope` for `Normal` beside a
`native_scope_variant`. Two spellings of one operation are two things to
document and test for no fidelity gained, and the variant is a value the call
site already has (§3.2). The atlas's raw `Arc<Style>` cells, which an earlier
revision exposed as `base_style`, `role_style` and `role_style_variant`, are
crate-private. Installed raw with `Ui::set_style` or `UiBuilder::style`, a cell
replaces the whole style, so an application's own `TextStyle::Name` keys vanish
and the next `TextStyle::resolve` of one panics, in release
(`egui/src/style.rs:112-120`, specification §14 item 30). The public calls merge
those keys (§3.11), and they cover every seam: S2 through `native_scope` and
`native_set_style`, S3 through `role_modifier`, the `Frame` carrier through
`surface_frame` and `native_frame`, and S1 through `install`. A public raw cell
would add a panic path and no reach.

### 3.4 Install into both colour schemes, always, on every start

**Chosen.** `ctx.set_style_of(egui::Theme::Dark, ..)` **and**
`set_style_of(egui::Theme::Light, ..)` (`context.rs:2250`), unconditionally.

**Rejected:** `set_global_style` (`context.rs:2200`) and `set_visuals`
(`context.rs:2280`) — both
touch only the *active* theme, leaving the other stock, so an OS light/dark flip
would drop the application into unthemed egui. Also rejected: `set_visuals_of`
(`context.rs:2267`), which carries only `Visuals` and so can never deliver `spacing` or
`text_styles` — i.e. every geometry
value the connector exists to deliver.

**Why on every application start.** `Options::dark_style` and `light_style` are
`#[serde(skip)]` (`egui/src/memory/mod.rs:195`, `:199`), so a persisted `Memory`
never restores them. An installer that assumed persistence would work in
development and fail on the second launch of a shipped binary.

This is also why `SystemThemeExt::to_egui_atlas` returns one atlas carrying both
variants, deviating from `to_iced_theme` (`connectors/native-theme-iced/src/lib.rs:305`)
and `to_gpui_theme` (`connectors/native-theme-gpui/src/lib.rs:328`), which each
return a single toolkit theme. egui stores one `Style` per `egui::Theme` and,
under `ThemePreference::System`, picks between them every pass from the colour
scheme the integration reports in `RawInput::system_theme`
(`egui/src/memory/mod.rs:358-359`, `:367`); returning one variant would
guarantee a half-themed application. On Linux the integration reports nothing:
winit 0.30.13's `system_theme()` there is a bare `None` (lines 909–911 of its
`src/platform_impl/linux/mod.rs`), and its Linux backends never emit the
`ThemeChanged` event egui-winit would update from (a grep of
`src/platform_impl/linux` finds none). egui then takes `Options::fallback_theme`,
dark by default (`egui/src/memory/mod.rs:212`), whatever the desktop's scheme —
which is why the install plugin supplies the atlas's `os_mode` (§3.5).

### 3.5 The connector does not own the colour-scheme choice, and `install` takes no options

**Chosen.** One install call, `ThemeAtlas::install(ctx)`, which never touches
`Options::theme_preference`, `fallback_theme` or `sync_window_theme`. Following
the OS or pinning a scheme is egui's own `ctx.set_theme(..)`, documented on
`install` (specification §4.2).

**Rejected:** an install option `theme_preference` defaulting to
`Some(ThemePreference::System)`. `Options::theme_preference` **is**
serde-persisted (`egui/src/memory/mod.rs:206` has no `serde(skip)`), so that
default would silently discard the user's in-app Light/Dark choice on every
launch. A theme installer overwriting a user preference is the worst class of
surprise a library can ship.

**Rejected: an `InstallOptions` struct and an `install_with(ctx, options)`
altogether.** Every option the earlier revision offered either had one right
value for the native look or was egui's already. The focus ring is always on: it
appears only on keyboard focus in a window that has the keyboard (§3.7), where a
platform shows one, and a widget with an outline of another shape registers it
with `register_focus_shape` rather than turning the ring off. The line-spacing
plugin switch went with the plugin (§3.12). The icon-cache flush always runs,
because a theme change gives every recoloured icon new bytes and a new URI
(specification §9.2), and without the flush the previous theme's entries would
stay cached for the life of the `Context` (§3.14). `fallback_theme` and `sync_window_theme` are `Options` fields egui
exposes through `ctx.options_mut` (`egui/src/context.rs:1135`), so options
forwarding them would be a second spelling of an egui API. The two helpers
`follow_os_color_scheme(ctx)` and `pin_color_scheme(ctx, theme)` were one-line
wrappers over `ctx.set_theme(..)` and are gone too; the trap they existed to
document is documented where every application meets it, on `install`:
`Context::set_theme` takes `impl Into<ThemePreference>` (`context.rs:2170`) and
`From<Theme> for ThemePreference` exists (`memory/theme.rs:79-86`), so
`ctx.set_theme(egui::Theme::Dark)` *pins* dark rather than following the OS.
That is an easy mistake to make and an invisible one to debug.

**What the connector does own: the colour scheme egui is told, where nobody
tells it.** That is an input, not a preference. The install plugin's
`input_hook` (`egui/src/plugin.rs:38`) sets `RawInput::system_theme` to the
atlas's `os_mode()` **only when the integration left it `None`** — on Linux,
always (§3.4). The hook runs inside `Context::begin_pass` before egui copies
the value into `Options` (`egui/src/context.rs:965-968`,
`egui/src/memory/mod.rs:358-359`), so `ThemePreference::System` resolves to the
desktop's scheme on that same pass, and on every integration, hand-driven
passes included. Where the integration does report a scheme it wins untouched,
so the hook changes nothing on macOS or Windows.

**Rejected: writing `Options::fallback_theme` from the OS mode.** It feeds the
same `unwrap_or` (`egui/src/memory/mod.rs:367`), but it has no `serde(skip)`
(`:212`): a field the application owns would be rewritten and saved to disk,
where the hook changes only the pass's input. Both are exactly as live as the
installed atlas: on Linux a desktop switch made while the application runs
reaches egui when `ThemeWatcher` rebuilds and installs (§3.14), else at the
next start. Also rejected: setting the preference to the detected scheme,
which is the overwrite this section refuses.

A preset-built atlas has no OS mode — a preset states two schemes, not which
one the desktop is showing — so `Builder::os_mode(ColorMode)` supplies it,
documented with `native_theme::detect::system_is_dark()`
(`native-theme/src/detect.rs:141`) as the value to pass. Without it the hook
does nothing and egui's own behaviour stands.

The window's title bar has to follow the same scheme, or a light application
sits under a dark title bar. With `Options::sync_window_theme` at its default
`true` (`egui/src/memory/mod.rs:333`), egui answers `ThemePreference::System`
with `ViewportCommand::SetTheme(SystemTheme::SystemDefault)`
(`egui/src/context.rs:2474-2497`), egui-winit hands that to winit as `None`
(lines 1911–1915 of egui-winit 0.36.2's `src/lib.rs`), and winit's X11 backend
writes the same `dark` `_GTK_THEME_VARIANT` hint for `None` as for `Dark`
(lines 631–655 of its `src/platform_impl/linux/x11/window.rs`). So under
`System` an X11 title bar asks to be dark whatever the desktop shows. The
connector that tells egui the scheme therefore also tells the window, when the
preference is `System` and the integration reported no scheme: the install
plugin's `output_hook` (`egui/src/plugin.rs:44`) rewrites that `SystemDefault`
into the OS mode (specification §10.3).

### 3.6 The five-state derivation, and why the base states borrow from `theme.button`

**Chosen.** `noninteractive` ← the role's non-interactive colours; `inactive` ←
its resting interactive colours; `hovered` ← its `hover_*`; `active` ← its
`active_*` or, where absent, a field-wise copy of `hovered`; `open` a
field-wise copy of `inactive` except where egui reads it for a distinct look
(specification §6.1). On the *base* style the interactive states come from
`theme.button` — its border colour, width and radius as well as its colours —
and `noninteractive` takes its border and radius from `defaults.border`. A hover or pressed **fill** that sits on the widget's own idle
fill is written composited over that fill, never raw — contract C17, argued
below. Every role fills the entries in that order, so a role that states no
hover or pressed value for a field keeps its resting value there — the
platform stating no change — with two exceptions where the entry is not the
widget's: a dialog, popover or tooltip writes only its text colour, into
`noninteractive`, leaving the controls inside it the base style's states, and
a panel or list writes its border into `noninteractive` only, where its edge
line is painted, so that it does not outline every button in it
(specification §6.1).

**Why `open` is special.** `WidgetState` has four variants
(`widget_style.rs:84-90`), so `Widgets::state` (`:94-99`) and `Widgets::style`
(`style.rs:1273-1282`) can never return `open`. Every use of it in egui is a
direct field read: `containers/window.rs:1427`, `containers/combo_box.rs:371` and
`:450`, `widgets/color_picker.rs:117`, plus `menu_style`'s write at
`containers/menu.rs:25` (`menu_style` is `:22-29` in its entirety), and
`SubMenuButton::ui`'s read-then-copy-into-`inactive` at
`containers/menu.rs:382-386` — which is neither inside `menu_style` nor a write
to `widgets.open`. Outside egui itself, `DatePickerButton` reads it too while
its calendar is open (`egui_extras/src/datepicker/button.rs:143`). So `open` is
not on the interaction-state axis, whatever its place in the struct suggests.

One of those reads decides a cell. While a submenu is open, `SubMenuButton`
paints its parent item from `open` (`containers/menu.rs:382-384`), so inside
`Role::Menu` `open` is a field-wise copy of `hovered`: the parent item of an
open submenu keeps the highlight the pointer gave it. That is the platform's
behaviour, not a guess — WinUI 3 gives a sub-item's `SubMenuOpened` state the
same brush as its `PointerOver` state, `SubtleFillColorSecondaryBrush`, in both
the default and the light dictionaries (`MenuFlyout_themeresources.xaml` in
microsoft-ui-xaml at commit `258a2e9b`, lines 16 and 18, 182 and 184). The
alternative, the copy of `inactive`, would drop the highlight from the item the
user is inside.

**Why `theme.button` and not `defaults`.** `ResolvedDefaults` has 31 fields and
**not one of them is a hover or pressed value** (`resolved.rs:72-146`). Every
hover colour in the model lives on a widget. `Button` is by a wide margin egui's
most common interactive widget: `ui.button` (`ui.rs:1848`), `ui.toggle_value`
(`:1875`), `ui.selectable_label` (`:1929`) and `ui.selectable_value` (`:1939`)
are all `Button`s, and so is every menu entry — `MenuButton` holds a `Button`
(`containers/menu.rs:291`, constructed `:297`) and so does `SubMenuButton`
(`:338`, `:347`). Borrowing from it is the only
non-inventing choice, and the specification calls it a stated borrowing rather
than dressing it up as a derivation. The border and radius follow the same
reasoning: an unscoped `ui.button`, combo-box trigger, `DragValue` or
selectable is drawn from those interactive states (a `DragValue` builds a
`Button`, `widgets/drag_value.rs:636`), so the native look of an unscoped
control is the button's outline, not the generic hairline; `noninteractive`
is what frames, separators and group boxes draw with, and there
`defaults.border` is the platform's own value.

**Why the base style's pressed text is `defaults.text_color`, not
`button.active_text_color`.** egui ties strong text to the pressed state:
`Visuals::strong_text_color()` is `widgets.active.text_color()`
(`egui/src/style.rs:1147-1149`), read by every `RichText::strong()` and
`ui.strong()` (`widget_text.rs:485`) and by the default `Spinner`
(`spinner.rs:44`). Borrowing the button's pressed text there as well painted
every strong label in the colour the platform reserves for text on a pressed
fill — kde-breeze light's `#fcfcfc` (`native-theme/src/presets/kde-breeze.toml:95`)
on its `#eff0f1` panel, where the showcase's captions all but vanished. A label
the user reads is on screen all the time; an unscoped button's pressed state
lasts as long as a click. So the base style writes `defaults.text_color` into
`widgets.active.fg_stroke.color` — the platform's text on the panel — and
`button.active_text_color` moves to the `Role::Button` cell, where the platform's
exact pressed pair holds (specification §5.3, §6.1). Its verdict is SCOPED for
that reason: the one global write that would have made it DIRECT is the one this
decision withdraws.

**Rejected by name, so they are not re-proposed.** Every "obvious" way to
synthesise a hover colour needs a constant that exists nowhere in
`ResolvedTheme`: lightening or darkening by a factor; `Color32::gamma_multiply(k)`
(`ecolor/src/color32.rs:269`, whose `k` outside `0.0..` additionally trips a
`debug_assert` at `:270-273`); `ecolor::tint_color_towards` at some ratio;
blending toward `accent_color` by some weight. A specification that writes
"derive hover by lightening 8 %" has invented a platform value.

**Why hover and pressed fills are composited over the idle fill (C17).** The
platform *layers* a hover or pressed colour over the control's own fill; egui,
like iced and gpui-component, *replaces* the fill per state —
`Style::button_style` paints the frame with the state's `weak_bg_fill`
(`widget_style.rs:159`). A translucent layer written raw is therefore drawn over
whatever lies behind the widget. Windows 11's `button.hover_background` is
`#0000000a`, a 4 % black layer: over the button's `#fdfdfd` the platform shows
`#f3f3f3`, while a replacing toolkit given the raw value shows 4 % black over the
`#f3f3f3` window, `#e9e9e9` (measured in
`docs/archive/todo_v0.5.9_theme-contracts-rationale.md` §2.11, which fixed the
same defect in the gpui connector). Compositing is not the synthesis rejected
above: it takes no constant, only the platform's two colours and the platform's
own alpha, and on an opaque layer it is the identity — so one rule serves every
preset. More presets layer than gpui's four button tokens suggest: windows-11
states `#0000000a` hover layers on its `button`, `checkbox`, `combo_box`,
`segmented_control` and `expander` too (`native-theme/src/presets/windows-11.toml:85`,
`:131`, `:333`, `:353`, `:369`), material states translucent hovers on its
`checkbox`, `combo_box`, `segmented_control` and `expander`
(`native-theme/src/presets/material.toml:112`, `:230`, `:238`, `:248`), and which
of those reach an egui fill is the specification's per-row
business (§6.1). egui ships the operation: `Color32::blend` is `self` scaled by the
layer's remaining alpha plus the layer, on premultiplied bytes with integer
arithmetic (`ecolor/src/color32.rs:343-345`, `:289-298`, `:375-382`). What is
**not** composited, for the reasons that theme-contracts §2.11 and
specification §6.1 give: idle fills; disabled fills (Adwaita's half-transparent
`disabled_background` *replaces* the fill, so compositing it would erase the
disabled look); a `soft_option` fallback, which *is* the idle fill, copied; row
highlights — `menu`, `list`, `sidebar`, `tab` — which are drawn over a panel the
same UI also paints; `expander.hover_background`, because `ExpanderTheme` has no
idle fill; and the scrollbar and slider thumbs, which sit on a rail egui also
paints.

The same principle governs `soft_option` fallbacks, and since v0.5.9 the category
holds two kinds of field that must not be confused. A `soft_option` **colour**
that is `None` is **not** missing data: it is the platform asserting the widget
has no distinct appearance in that state. The correct derivation is therefore
always a **copy** — no arithmetic, no division, no narrowing, no `NaN` path — and
every chain in specification §6.4 terminates on a required field in one step.
v0.5.9 made this rule binding on both sibling connectors as contract C16, citing
this section. A `soft_option` **size** — `menu.row_height`, `toolbar.bar_height`,
`toolbar.item_gap`, `list.row_height`, `combo_box.arrow_area_width`
(`native-theme/src/model/widgets/mod.rs:209`, `:451`, `:457`, `:520`, `:744`) —
that is `None` means something else: the platform states no such size. There is
no base-state value to copy, and nothing may be invented; egui's own behaviour
stands (§3.23).

### 3.7 The focus ring is painted by the connector, never folded into `active`

**Chosen.** The install plugin's `on_end_pass` (`egui/src/plugin.rs:31-32`) asks egui which widget has
keyboard focus (`Memory::focused`, `egui/src/memory/mod.rs:893`), reads that
widget's rect from this pass's widget table (`Context::viewport`,
`egui/src/context.rs:3963-3965`) and strokes a ring around it on the widget's
own layer (`Context::layer_painter`, `:1587`), from
`defaults.focus_ring_color`, `focus_ring_width` and `focus_ring_offset` — a
negative offset draws the ring inside the widget's rect, as the platform does.
One ring, for every focused widget, scoped or not, with no application code and
no switch to turn it off (§3.5).

**Rejected, first: folding the ring into `widgets.active.bg_stroke`,** which two
of the three design proposals wanted and which looks like free fidelity. The
defeating fact: egui makes focus and press the *same* state —
`response.is_pointer_button_down_on() || has_focus() || clicked()` selects
`active` (`widget_style.rs:107-109`, mirrored at `style.rs:1276`). Writing the
ring there paints it on **every mouse press** and simultaneously displaces the
role's real pressed border, and `focus_ring_offset` has no sink there at all.
A wrong ring on every press is worse than no ring.

**Rejected, second: accessors only, the ring left to the application** — this
document's earlier decision. The resolved model carries the ring's colour,
width and offset for every preset, as required fields
(`native-theme/src/model/resolved.rs:137-141`); an application that must draw
the ring itself mostly will not, so the look would lose a
visible, stated element on exactly the path the connector exists to theme, the
unscoped one. Waiting for an upstream `Visuals::focus_stroke` (§4.2) is not an
answer either, because the connector must not depend on upstream change.

**Why the plugin's ring is the right one.** It keys off egui's *focus*, not its
`active` state, so it appears where the platform's does: a button registers
interest in keyboard focus (`egui/src/context.rs:1256-1270`) but a click does
not give it focus. Measured on egui 0.36.2 with the plugin as specified: no ring
at rest, none during or after a mouse click on a button, one after Tab.

**Why this pass's rect.** `Context::read_response` would be the obvious lookup,
but it falls back to the previous pass's rect (`egui/src/context.rs:1355-1376`),
and egui drops the focus of a widget that stopped being shown only after the
plugins have run (`egui/src/memory/mod.rs:630-637`) — so it would draw one
frame of ring where the widget used to be.

**Only while the window has the keyboard.** egui keeps the focused widget
when the window loses OS focus (`egui/src/input_state/mod.rs:440-448` clears
only held keys), so a ring drawn from `Memory::focused` alone would stay on in
a window the user has left. A ring marks where keyboard input goes, and an
unfocused window receives none; egui's own `Response::has_focus` already
requires the window's focus (`egui/src/response.rs:350`). The platforms do the
same. Qt clears the application's focus widget when no window is active (qtbase
v6.8.0, `src/widgets/kernel/qapplication.cpp` lines 1880-1881),
`QWidget::hasFocus` is `QApplication::focusWidget() == this`
(`src/widgets/kernel/qwidget.cpp` line 6489), `QStyleOption::initFrom` sets
`State_HasFocus` from it (`src/widgets/styles/qstyleoption.cpp` lines 146-147),
and Breeze draws no focus frame without that flag (`kstyle/breezestyle.cpp`
line 4104, the file at tree `be6e137e`). GTK, when a window stops being active,
moves focus from the focus widget to none (GTK 4.16.0, `gtk/gtkwindow.c` lines
5919-5922), which unsets `GTK_STATE_FLAG_FOCUSED` and
`GTK_STATE_FLAG_FOCUS_VISIBLE` along the chain (lines 5019-5021, 5059), the
flag `:focus-visible` outlines match. So the plugin paints only while
`InputState::focused` (`egui/src/input_state/mod.rs:315`) is true.

**The ring's shape.** The ring follows the corners of the role scope the
focused widget was laid out in, with no call at the widget; specification §6.18
gives the mechanism, its one exception (the root `Ui`), the shape registration
for an outline that is not the widget's rect, and where the plugin's hooks run.
Three alternatives lost. Asking widget code to register every rounded shape is
a protocol nobody follows for egui's own widgets. A walk from the focused widget
up `WidgetRect::parent_id` to the nearest recorded scope cannot work:
`parent_id` holds the host `Ui`'s `id` (`egui/src/ui.rs:924`, `:303`), while a
`Ui`'s own entry in the widget table is keyed by its `unique_id` (`:302`,
`:174`), and the two differ for every child `Ui` with an automatic id
(`:252-257`), so the chain breaks after one step. And guessing a shape from the
widget's size would invent one. Ledger item 5 therefore keeps only egui's part: a
focused widget drawn in its pressed look, and a ring that is a rounded rectangle
unless a shape is registered.

The record the scopes and registrations leave for the plugin is one temporary
`IdTypeMap` entry per viewport, emptied when the pass number changes
(specification §6.18). egui never drops a temporary entry by itself — its only
collection runs when memory is persisted, and it leaves temporary values out of
the saved copy (`egui/src/util/id_type_map.rs:712-722`, `:728`) — so an entry
per scope id would grow with every id ever scoped.

`focus_ring_offset` is the one value documented as legitimately negative
(adwaita −2.0, macOS −1.0, `native-theme/src/resolve/validate_helpers.rs:708-709`),
which is exactly the kind of value a well-meaning clamp destroys; the ring
geometry takes it as given.

### 3.8 `RoleVariant::Disabled` and the `1.0` identity

**Chosen.** The `Disabled` cell writes the platform's disabled colours into the
field each widget paints from in the **`inactive`** state, and sets
`Visuals::disabled_alpha = 1.0`. For a button or a combo box that field is the
`inactive` entry itself; for the others it is the one the widget actually reads
— `text_edit_bg_color` for an input, `override_text_color` for a checkbox's
label and `inactive.fg_stroke` for its mark, the `noninteractive` foreground
for list cells and plain labels,
`hyperlink_color` for a link, the rail's `inactive.bg_fill` and the fill's
`selection.bg_fill` for a slider, `selection.bg_fill` for the switch substitute
(specification §6.3). A disabled colour written where the widget does not look
is a silent no-op, the failure §3.21 forbids.

**Why `inactive`.** A disabled widget lands in `WidgetState::Inactive`, because
every predicate `Response::widget_state` tests before its fall-through
(`widget_style.rs:105-115`) is false for it: `Flags::HOVERED` is set only inside
`if res.enabled()` (`egui/src/context.rs:1496-1500`), `Flags::CLICKED` only when
`enabled` (`:1525-1526`), hit testing strips `Sense::CLICK` and `Sense::DRAG`
from a disabled widget so it can never be the pressed one
(`egui/src/hit_test.rs:131-139`), and focus is surrendered because
`interested_in_focus = w.enabled && …` (`egui/src/context.rs:1256`, `:1274-1276`).
Nothing in `widget_style.rs` itself tests `enabled`. That is where the
platform's disabled colours have to go.

**Why `1.0`.** `Ui::disable` multiplies painter opacity by `disabled_alpha`
(`ui.rs:497-502`). Writing `1.0` makes the multiply the identity, so the
platform's own disabled colour survives instead of being faded a second time,
while interaction stays blocked. `1.0` is a **multiplicative identity, not a
theme value**, and it is one of the three kinds of numeric literal the mapping
may contain beside the exemptions read from egui's own source (specification
§6.17).

**The rejected objection, recorded because it was raised and answered.** One
proposal declined this on the grounds that it would compound into
`disabled_color.gamma_multiply(disabled_opacity)`. Setting the alpha to the
multiplicative identity is precisely what prevents that compounding.

**What the technique reaches.** The leaves specification §6.3 lists: the
widgets' disabled fills and disabled text colours, the switch's two disabled
tracks among them through the substitute's fills (§3.22), less the three named
below. Each disabled fill falls back to its enabled colour
when the platform states none. The link is reached only because the cell
writes `visuals.hyperlink_color`: `Link::ui` paints the galley with that
colour and nothing else (`egui/src/widgets/hyperlink.rs:47`, used at `:59` and
`:63`), so a `fg_stroke.color` written for it would never show. What stays lost is lost for a
stated reason: `slider.disabled_thumb_color`
(`native-theme/src/model/widgets/mod.rs:335`), because egui fills the
thumb from the same `inactive.bg_fill` as the rail once the slider is disabled
(`egui/src/widgets/slider.rs:775`, `:816`), so the rail wins; the switch's
`disabled_thumb_color` (`native-theme/src/model/widgets/mod.rs:641`), because the substitute has no knob; and
`defaults.disabled_text_color`, which has no `Role` to be scoped by — `defaults`
is not one of the 25 widget fields of `ResolvedTheme`, so there is no
`Role::Defaults` (specification §4.4). The reached rows are SCOPED by
specification §2's definition — a `Style` field delivered through a role scope
— and the route exists **only for widgets the application scopes**, which is
ledger item 6 and is stated as a limit, not sold as a win.

**Why `Selected` and `Disabled` are not combined.** The model states a
disabled-and-selected colour for one widget only, the switch
(`switch.disabled_checked_background`, `native-theme/src/model/widgets/mod.rs:635`),
and that widget needs no combined variant: its substitute takes the checked
state per call, `Button::selected(checked)`, so a `Disabled` switch scope carries
the disabled checked track in `selection.bg_fill` and the disabled unchecked
track in the resting fill (§3.22). For every other role a combined cell would
have nothing to write — and on a selected `Button` the `Disabled` cell's
`inactive` fill never shows anyway, because `Style::button_style` overwrites
`weak_bg_fill`, `bg_fill` and `fg_stroke` from `visuals.selection.*`
**regardless of the interaction state** (`widget_style.rs:147`, `:150-155`).
`RoleVariant` is `#[non_exhaustive]`, so a combined variant is additive if the
model or egui ever gives it something to carry.

### 3.9 Numeric conversion: total, saturating, `denan` first

**Chosen.** Every `f32` → `u8` / `i8` conversion goes through the crate's own
helpers, which are total over all `f32` bit patterns, panic-free, and whose
single `as` cast is applied to a value already proven in range.

**The premise correction that shaped the policy.** Rust's float-to-integer `as`
cast **saturates** — above the maximum it clamps, below the minimum it clamps,
`NaN` becomes `0`. It never wraps and never panics. That is a language
guarantee, so it has no `file:line` inside these crates, and epaint's own
`impl From<f32> for CornerRadius` relies on it
(`Self::same(radius.round() as u8)`, `epaint/src/corner_radius.rs:42-44`). The
project's design brief said such a cast "wraps or truncates"; that is wrong, and
the documents say "saturates and truncates toward zero" instead. **Integer-to-integer**
`as` casts *do* wrap, which is why no intermediate integer step is permitted.

**The real hazards are therefore quieter than a crash**: silent rounding to whole
points, silent saturation at ±127 or 255, and `NaN` silently becoming a zero
margin — a wrong layout rather than a panic, which is worse because it is
invisible. That is why panic-free saturating conversion is still a hard
requirement.

**Rejected:** `f32::clamp` (it *propagates* `NaN` and panics when `min > max`);
bare `.max()`/`.min()` without `denan` first (order-dependent under `NaN` —
`NAN.max(lo).min(hi) == lo` but `NAN.min(hi).max(lo) == hi`); and routing
anything through `MarginF32`, whose `impl From<MarginF32> for Margin`
**truncates** with a bare `as _` and no `.round()` (`margin_f32.rs:32-42`) while
`impl From<f32> for Margin` rounds (`margin.rs:108-113`). epaint ships two
disagreeing rounding policies for the same target type; the connector inherits
neither and uses its own helpers everywhere.

**One rule for a value that is not finite.** A `NaN` or `±∞` leaf is replaced
by *egui's own value for that sink* — `finite_or(x, <egui's value>)` — and
never by `0.0` unless `0.0` is what egui itself has there; `to_margin` and
`to_button_padding` apply it per side, so a non-finite side takes `base`'s side.
It is §3.23's rule for an unstated size, applied to a size that cannot be read:
in both cases the platform has given no usable number, and egui's considered
default is the one value nobody invented. A zero is not neutral — as a margin it
is content touching its frame, the look §3.23 removed. The narrowing helpers stay
total on their own, but no call site relies on their `NaN → 0`.

A text size is held to more: epaint debug-asserts a finite, positive font scale
(`epaint/src/text/font.rs:181-184`), so a scaled size that is not a positive
normal `f32` keeps egui's own size for its slot and emits `Note::ValueSanitised`
(specification §8.5); a subnormal size underflows that scale to `0.0`. A finite
size too large for the font atlas still panics in release; the bound depends on
`max_texture_side` and `pixels_per_point` at run time, so it is a stated
residual (specification §14 item 44), not a clamp to a number no source gives.

**One ceiling for a length.** Every length sink passes through
`convert::clamp_length`, which keeps it below
`f32::MAX * egui::emath::GUI_ROUNDING`. egui snaps layout values to multiples
of `GUI_ROUNDING`, 1/32 (`emath/src/gui_rounding.rs:18`), by dividing by it,
rounding and multiplying back (`:59`), so any finite value above
`f32::MAX / 32` — about `1.0634e37` — becomes `+∞` in the division
(specification §6.7 records the bisection), and a finite theme value would turn
into an infinite layout inside egui. The ceiling is a property of egui's
arithmetic, read from its own constant, not a theme value. A sum of lengths
clamped one by one can still pass it; that residual is stated rather than
closed, because a theme whose sizes are each near `1e37` is hostile input, not a
platform. A formula over finite leaves that overflows — `f32::MAX − (−f32::MAX)`
is `+∞` — needs no guard of its own: the leaves are tested for finiteness, and
the result lands in the same clamp, capped or floored (specification §7.2).

**Why `.round()` and not `round_ties_even`**: so that a value this crate converts
and a value epaint converts through its own `From` impl agree bit for bit.

**Why this is not defensive programming.** `ResolvedTheme` derives `Deserialize`
with public fields (`resolved.rs:155-156`), native-theme range-checks only the
padding sides of a per-widget border — the generator emits `check_padding` for a
nested `ResolvedWidgetBorder` and nothing for its `corner_radius` or
`line_width` (`native-theme-derive/src/gen_ranges.rs:127-141`, the font check
beside it at `:146-147`; `check_defaults_ranges` covers `defaults.*` and `text_scale` only,
`native-theme/src/resolve/validate_helpers.rs:650-842`) — and `card` is missing
from the `check_ranges` dispatch list altogether
(`native-theme/src/resolve/validate.rs:168-191`). A `NaN`, negative or `1e30`
radius can reach the connector in production.

**The first genuine panic path**, and why it justifies a dedicated helper:
`Visuals::disabled_alpha` (`style.rs:1126`) is forwarded by `Visuals::disable`
(`:1177-1180`) to `Color32::gamma_multiply`, which carries
`debug_assert!(0.0 <= factor && factor.is_finite(), ..)`
(`ecolor/src/color32.rs:270-273`). A bad value there would abort every downstream
user's `cargo test` and `cargo run` — far worse than a wrong pixel. Everything
reaching that field passes through `unit_interval()`.

**The second, new in emath 0.36.2.** `Vec2` and `Pos2` no longer derive
`PartialEq`; their hand-written `eq` asserts, in debug builds only, that neither
side has a `NaN` component (`emath/src/vec2.rs:349-359`,
`emath/src/pos2.rs:233-243`).
`Style`'s derived `PartialEq` (`egui/src/style.rs:241`) reaches every `Vec2`
in `Spacing`, and specification §13 T4 and T10 compare every built style with
itself, so a `NaN` written into `item_spacing`, `button_padding` or `interact_size` would abort a
debug build at the first comparison. A float written into a `Vec2` therefore
needs the same `denan` / `finite_or` discipline as one narrowed to an integer
(specification §7).

Two precisions that were wrong in rejected drafts and are corrected here:
`Ui::disable` **cannot** fire that assert, because it calls
`Painter::multiply_opacity`, which is
`if opacity.is_finite() { self.opacity_factor *= opacity.clamp(0.0, 1.0); }`
(`painter.rs:100-104`) — non-finite dropped, value clamped. The **only** caller
of `Visuals::disable` in all of egui is `Ui::dnd_drop_zone` (`ui.rs:2726-2727`),
so the test that proves the invariant must exercise `dnd_drop_zone` specifically
(specification §13 T4).

**What `mod convert` makes public.** The helpers an application needs to write a
value the way the atlas writes it — `Rgba`, `to_color32`,
`to_color32_with_opacity`, `finite_or`, `clamp_length`, `unit_interval`,
`u8_from_f32_saturating`, `to_corner_radius`, `to_margin`, `composite_over` and
`to_stroke` — so a `Frame` or a `RichText` built at a call site from a
`ResolvedTheme` leaf gets the same colour space, the same non-finite rule and the
same saturation as the atlas's own. The helpers only the atlas's own sinks use —
`denan`, `i8_from_f32_saturating`, `padding_with_border`, `to_button_padding`,
`to_shadow` — are crate-private: each is a step inside a public helper or is
bound to a `Style` field the application never writes, and every public helper
is a signature the semver contract freezes. Specification §4.8 lists the public
signatures once, and specification §7.2 gives every doc comment and body once.

### 3.10 Colour space: one route, and it is `const`

**Chosen.** `Color32::from_rgba_unmultiplied_const` (`ecolor/src/color32.rs:139`).
It is `const`, and it premultiplies in **gamma** space, applying no sRGB
transfer function — which is the property that matters, because
`native_theme::color::Rgba` is already gamma-encoded. `a == 255` short-circuits
to an exact `from_rgb` (`:145`) and `a == 0` to `TRANSPARENT` (`:142`); for
`1..=254` it computes `mul_frac_round(channel, a)` per channel (`:147-151`) —
in ecolor 0.36.2 an integer fixed-point `channel · a / 255`, rounded, with no
float at all (`ecolor/src/lib.rs:137-146`). ecolor 0.36.1 computed
`fast_round(channel as f32 * linear_f32_from_linear_u8(a))` there instead, and
the two give the same byte for every one of the 256 × 254 inputs — checked
exhaustively for this revision, against an IEEE `f32` emulation of the old
expression; upstream's own test samples every fourth value
(`ecolor/src/color32.rs:515-522`). The non-`const` `from_rgba_unmultiplied` now
simply calls the `const` one (`:133-135`), where 0.36.1 kept a lookup table of the
float expression, so the two agree by construction. It still rounds, so the
forward conversion is lossy at low alpha in exactly that one place.

**Rejected:** `Color32::from_rgba_premultiplied` (`ecolor/src/color32.rs:122`), which stores straight
components as if premultiplied and turns `(255, 255, 255, 0)` into *additive
white*. It is **identical** for opaque colours, so the bug only appears on the
first semi-transparent value — which in this model is `defaults.shadow_color`,
i.e. it would ship.

**Also rejected:** routing through `egui::Rgba`. `native_theme::Rgba::to_f32_array`
divides by 255 with no transfer-function change (`native-theme/src/color.rs:126-134`)
while `ecolor::Rgba` expects linear (`ecolor/src/rgba.rs:3-10`), so the round trip
double-encodes gamma — worst exactly in the dark tones a dark theme is made of.

Round-tripping is documented as **not** exact **in either direction**: the
forward premultiply rounds at low alpha, and `Color32::to_srgba_unmultiplied`
(`ecolor/src/color32.rs:248`) is lossy there too — its own doc says so
(`:245-246`). That is stated rather than assumed.

### 3.11 Fonts: the system's own typeface, never a `FontFamily::Name`, one weight per family

**Chosen.** Every `FontId` this crate produces names `FontFamily::Proportional`
or `FontFamily::Monospace`, and `fonts::font_definitions` only ever *prepends
into those two existing chains* — with the platform's own typeface where the
system has it (below).

**Rejected:** registering a custom `Name` family per `(family, weight, style)`
triple — which an earlier mapping assumed, and which would have
installed the per-widget weight and slant leaves into the role styles
(specification §5.8).

**Why the invariant wins anyway.** Two epaint panics fire in `FontsImpl::font`
(`epaint/src/text/fonts.rs:1021`), at the first layout of text in the offending
family, with no recovery point:
`FontFamily::{family:?} is not bound to any fonts` (`epaint/src/text/fonts.rs:1025`)
and `No font data found for {font_name:?}` (`:1033`). They exist because `Style`
and `FontDefinitions` land through **different channels**: `set_style_of` takes
effect immediately (`context.rs:2250`) while `set_fonts` is deferred to the next
pass (`context.rs:2104-2106` — `:2103` is a blank doc line). Any design that emits a `Name` must guarantee an
ordering across two channels, and a violated guarantee is a panic in the *user's*
application. The invariant makes both unreachable by construction — no pass
counter, no promotion gate, no documented precondition whose violation panics.

The rejected alternative had a concrete mechanism, a begin-pass gate
(`Plugin::on_begin_pass(&mut Ui)`, `egui/src/plugin.rs:26-27`), and it cannot be
made total. The hook itself could restyle the pass: it is handed the very root
`Ui` that `run_ui` then passes to the application (`egui/src/context.rs:811-812`),
`Ui::set_style` replaces its style (`ui.rs:387`), and every `Area` `Ui` built
later reads `ctx.global_style()` (`ui.rs:136`). But the hook runs only under
`run_ui` — `Context::begin_pass` runs input hooks alone (`context.rs:962-967`) —
so an integration that drives passes by hand would meet an unguarded `Name`. A
per-`(Role, RoleVariant)` `Arc<Style>` already handed to a scope cannot be
recalled. And `set_fonts` *overwrites* the definitions (`context.rs:2105`), so
every application font change would have to re-include the connector's families.
No check the connector could run closes those doors: a style already handed to
a scope keeps its `Name` family, and an application's own `set_fonts` bypasses
any gate — and `fonts.rs:1025` is a plain `panic!`, so it fires in release.

**An application's own fonts survive an install only through the plan.** The
install's own `set_fonts` overwrites too (`context.rs:2105`), so a `Name`
family the application registered would be dropped and its first use would
panic at `fonts.rs:1025`. So a `FontPlan` adds its faces to a base, egui's
`FontDefinitions::default()` unless the application hands its own in with
`FontPlan::with_base` (specification §4.9, §10.3); an `add_font` still pending
at the next pass is applied on top of them (`egui/src/context.rs:556-574`).

**The cost of the invariant is small, and it is carried.** No preset states a
weight or a slant for any widget's own `font`; the per-widget weights the
presets do state are the title fonts — adwaita 700 for `window.title_bar_font`
and 800 for `dialog.title_font` (`native-theme/src/presets/adwaita.toml:83`,
`:277`), macos-sonoma 700 for both
(`native-theme/src/presets/macos-sonoma.toml:345`, `:274`) and windows-11 600
for the dialog title (`native-theme/src/presets/windows-11.toml:303`), and the
`-live` variants of the first two for the dialog title. `role_font_weight(t, role)`
and `role_font_is_italic(t, role)` return each role's weight and slant, and the
call site applies them per text, exactly as `text_role_weight()` does for the
four text-scale roles — so what the invariant withholds from the installed
styles still reaches the screen wherever the application asks.

**An application's own `TextStyle::Name` keys survive installation.** Every
style this crate publishes starts from `egui::Theme::default_style()`, whose
`text_styles` holds egui's five stock keys and nothing else, and a published
style replaces the previous one wholesale — so an application's `Name` keys
would vanish, and the next `TextStyle::resolve` of one panics in release
(specification §14 item 30). Leaving the fix to the application was the
earlier answer and is rejected: it is a panic in an application that did
nothing wrong. Seeding the keys in `Builder::build` is impossible, because the
builder has no `Context`. So `install` copies every `Name` key from
`ctx.style_of(theme)` (`egui/src/context.rs:2221`) into each base style before
publishing it, `native_scope` / `native_set_style` merge any key missing from
the role style out of the parent `ui.style()`, cloning only when there is such
a key, and `role_modifier` carries the keys of the style its closure is handed.
No raw cell is public (§3.3), so no public path installs a style without the
merge.

**Two further wins.** The emoji fallback tail — `NotoEmoji-Regular` and
`emoji-icon-font` — that `FontDefinitions::default()` installs
(`epaint/src/text/fonts.rs:534-550`) survives, which a custom `Name` family keeps only if it copies the tail into
every chain. (There is no CJK tail to preserve: egui ships four faces and its own
documentation says "The default `egui` fonts only support latin and cyrillic
alphabets", `egui/src/context.rs:2101`.) And the headless test
technique of specification §13 T14 — `ctx.set_fonts(egui::FontDefinitions::empty())`,
which saves a face parse per test — is valid **only** because of this invariant.

**The price, stated as a price.** Exactly two families, therefore exactly one
weight per family. `FontId` is `{size, family}` with upstream's own
`// TODO(emilk): weight (bold), italics, …` at `epaint/src/text/fonts.rs:27`
(struct `:21-28`), and family is its only selector. Bold headings against a
regular body are not carried by the installed styles; a call site carries them
with `RichText::variation(Tag::new(b"wght"), ..)` (`egui/src/widget_text.rs:202`)
and `text_role_weight()` or `role_font_weight()`, as `text_role_line_height()`
carries leading. Weight is applied as a `wght` variation coordinate whose tag
is built by the infallible `const fn` `Tag::new(b"wght")`, never through the
`IntoTag` impls, which `expect` (`epaint/src/text/text_layout_types.rs:396`,
`:403`, `:410`) and are banned by the no-panic rule (specification §8.3).

**Why `fonts::supports_weight_axis` is crate-private.** "Is this face actually
variable" has a subtlety: the check returns `false` for *both* "static font"
and "unparseable bytes", because `FontData::variation_axes` early-returns an
empty `Vec` for both (`epaint/src/text/fonts.rs:153-173`) and the two are not
distinguishable through any public epaint API. The crate needs the answer only
to report a face that cannot take the theme's weight
(`Note::FontWeightAxisUnsupported`, specification §8.3), and its own `skrifa`
check (below) tells the two cases apart, removing an unparseable face before
epaint ever sees it. So the function is an internal helper under a unit test,
not a promise to applications.

**Why the typeface is the system's, found by name.** The platform states its
typeface as a *name* — `defaults.font.family`, `defaults.mono_font.family` —
and the only way a face enters epaint is a byte buffer:
`FontData { font: Cow<'static, [u8]>, .. }` (`epaint/src/text/fonts.rs:112-122`).
The `String` keys in `FontDefinitions` (`:431-444`) are arbitrary caller labels
that nothing looks up in a system font database, and neither epaint nor egui
nor eframe depends on `fontdb`, `font-kit`, `fontconfig`, `core-text` or
DirectWrite. (The only `load_system_fonts()` in the tree is
`egui_extras/src/loaders/svg_loader.rs:42`, feeding **resvg's** fontdb for text
inside SVG images — no connection to egui's text layout.) Something has to turn
the name into bytes, or every themed application shows egui's bundled faces:
right metrics, wrong typeface, and the typeface is the most visible thing a
platform look has.

So native-theme gains a `system-fonts` feature and
`native_theme::fonts::system_face(family, weight, style)`, and the connector's
`FontPlan::from_system` uses it for `defaults.font` and `defaults.mono_font` of
the atlas's light `ResolvedTheme`. egui holds one `FontDefinitions` per
`Context`, so one plan serves both variants, and no preset states a different
font family, weight or style for its dark variant (specification §8).
The rules are the project's rules for icons, applied to faces: the family must
be the one named — compared case-insensitively, as CSS Fonts Module Level 4
§5.1 requires of family names, and never another family, as fontdb's `query`
never offers one (lines 661–677 of fontdb 0.23.0's `src/lib.rs`) — and a miss is
reported as `Note::FontFamilyUnavailable` while egui's own font stays, never a
look-alike from another family. Within the family the face is chosen by the
width, style and weight steps, at the model's normal width, of the CSS
font-matching algorithm (CSS Fonts Module
Level 4, §5.2 "Matching font styles"), a published rule rather than a nearness
of our own. The selection is one pure function, `native_theme::fonts::select_face`,
exported without the feature because it needs no font database: `system_face`
runs it over fontdb's faces and the connector's `FontPlan` over the
application's own, so a family, weight and style are matched by one
implementation wherever the bytes come from. `SystemFace` carries the chosen
face's own weight and style as fontdb records them, and the face is registered
at those; the earlier design re-read the weight with `skrifa` and registered the
face at the weight asked for, describing a face as something it is not.
`Note::FontWeightAxisUnsupported` then fires only where it means something: the
chosen face's weight differs from the asked weight and the face has no `wght`
axis to reach it. The alternatives lost on the goal or on the rules:

* *Leave it to the application* — the earlier decision, "right metrics, wrong
  typeface". Rejected: the native look must not be opt-in, and every
  application would show the wrong typeface until it added a font-discovery
  layer of its own.
* *Do the lookup inside the connector.* It works, but the name → face question
  is the platform's, not egui's: the family names come from native-theme's
  readers, and the lookup is tested once, beside them, for any connector whose
  toolkit wants bytes.
* *Which lookup crate.* fontdb 0.23.0 is already in the workspace lock,
  through cosmic-text, so the workspace already builds it; it is used with
  `fs`, `fontconfig` and `memmap`, its own default set (its `Cargo.toml`,
  lines 63–68). Without `memmap` loading the system fonts reads every font
  file in full (its `src/lib.rs`, lines 287–293) — measured with
  `fs` and `fontconfig` alone, in a release build on one Linux machine: 1,129 faces from 1,060 files, 1,045 MB read, 1.02 s cold and
  142 ms warm per load (a second series on the same machine: 183–196 ms warm
  without `memmap`, 26–28 ms with it) — where with it fontdb maps each file to read its
  faces and keeps only the path (lines 233–234, 267–271), and maps the chosen
  face's file again when its bytes are asked for (lines 721–727, 912–917).
  The `unsafe` of those maps is fontdb's own, which its documentation calls
  "inherently unsafe" and answers by not keeping the files open (lines 50–51 of
  its `src/lib.rs`), as every dependency's `unsafe` is
  its own; native-theme's code gains none from it. The database is loaded
  once per process, into a crate-private `std::sync::OnceLock` in
  `native_theme::fonts` that every `system_face` call shares, so a theme
  switch pays no load; a font installed while the application runs is seen
  after a restart, as with iced's own font system (specification §8.2).

**The macOS system font is looked up by its file.** One name is already
known to miss: fontdb on `macos-latest` holds the system UI font as `.SF NS`,
while `macos-sonoma` states "SF Pro", the name `docs/platform-facts.md` §1
gives it, and the v0.5.9 iced showcase's macOS capture shows iced reaching the
font only through a fallback list (`docs/todo.md`, *macOS: the stated font
family "SF Pro" is not a family iced's font database holds*, which keeps the
question of what the preset should state). The alternatives were to map the
name "SF Pro" to `.SF NS` — an undocumented name Apple can change, and a
lookup by a name no theme states — or to leave the family to egui's face,
which gives up the platform's most visible typeface. The most native answer
asks the OS: Core Text names the file of its system font (its
`kCTFontURLAttribute`), so on macOS `system_face` resolves the system UI font
— "SF Pro", or the family Core Text itself reports for it — by that file,
and every other family by name. The Core Text calls are FFI, so they sit in
native-theme's `macos` module, which already permits FFI `unsafe`
(`native-theme/src/macos.rs:7`), behind one safe function (specification
§8.2). Because the project's rule is no `unsafe` without the maintainer's
consent, the choice was put to the maintainer, who approved the Core Text
route on 2026-09-25.

What stays **UNVERIFIED** until it runs on the platform: that the other
family names the macOS and Windows readers report — the Windows families,
macOS's monospace — are names fontdb records, and that the Core Text route
finds the system font under the names a theme gives it. The implementation plan's
runner-checks task runs the lookup on the macOS and Windows CI runners that the screenshot
workflow already uses, over the reader's theme and the platform's bundled
preset; specification §15 lists it among the open verification items.

**The sibling connectors: bytes for egui, a name for iced, gpui's own
alias.** The same macOS font reaches three toolkits whose font systems
differ, so each gets what its own font system resolves (specification §8.8).
egui has no font database and draws only the bytes it is given, so it takes
the face's bytes from `system_face`. iced draws through cosmic-text over
fontdb 0.23, a database of its own — the one the iced showcase probes
(`connectors/native-theme-iced/examples/showcase-iced.rs:5687`) — which files
the system UI font under the name its file records, `.SF NS`, not "SF Pro"; so
iced takes the family of the face `system_face` chooses, as
`SystemFace::family` records it, through
`native_theme_iced::system_font_family`: egui's selection, one
implementation, and no font name written into the connector. gpui resolves a
family through the platform's font system — on macOS Core Text, its memory
fonts first and then the system source (gpui-pre-macos 0.3.6
`src/text_system.rs`, lines 282–289) — and has a name for exactly this font:
`.SystemUIFont`, "used to identify the system UI font, which varies based on
platform" (gpui-pre 0.3.6 `src/text_system.rs`, line 1295), which its macOS text
system asks Core Text for as `.AppleSystemUIFont` through
`font_name_with_fallbacks` (lines 1420–1430). Mapping a stated family that
`fonts::is_macos_system_ui_family` names to that alias lets Core Text supply
its own system UI font, the most native route there is; were Core Text to
resolve "SF Pro" by name as well, the alias still selects the same font, so
the mapping is right either way.

**Rejected:** the bytes for all three — for gpui, the file `system_face`
finds loaded as a memory font through `TextSystem::add_fonts` (gpui-pre 0.3.6
`src/text_system.rs`, line 295) would replace Core Text's own font with a copy of
its file, and load the system font database inside `to_theme`, which has no
`cx` to add fonts through, for nothing gained; changing what `macos-sonoma`
states — no single name serves the three toolkits (fontdb records `.SF NS`,
gpui asks Core Text for `.AppleSystemUIFont`, Apple documents "SF Pro"), and
the preset states the platform's documented name
(`docs/platform-facts.md:59-67`); and a literal "SF Pro" in each connector —
the name test is written once, `fonts::is_macos_system_ui_family`, which
`system_face`'s macOS branch and the gpui connector share. The maintainer's
`docs/todo.md` item on the name (*macOS: the stated font family "SF Pro" is
not a family iced's font database holds*) stays open: these changes take its
option of keeping "SF Pro" as the documented name "with the connectors
mapping it", and whether iced's database and gpui's font names
hold the monospace "SF Mono" is left to the macOS runner (specification §15).

**Why the crate validates a face before epaint sees it.** Supplying the bytes
makes their validity the connector's business. `FontsImpl::new` parses every
registered face eagerly and **panics** on a parse failure
(`epaint/src/text/fonts.rs:990`) from inside `begin_pass`, in release; the only
fallible step of that parse is `skrifa::FontRef::from_index`
(`epaint/src/text/font.rs:386-388`). So `font_definitions` makes the same call
first, drops a face it rejects and reports `Note::FontDataInvalid`. That needs
a direct `skrifa` dependency, because epaint does not re-export it, and it is
declared with the requirement epaint 0.36.2 itself declares, `0.44.0`
(`epaint/Cargo.toml:149-150`), with a gate that `cargo tree -d` shows one
`skrifa`; the drift the earlier text feared is then a red gate at the next egui
minor, not a silent second copy. Rejected: parsing with fontdb's own parser,
which is not the call epaint makes, so a face it accepts could still panic
epaint; and leaving validity to the application, which no longer owns the
bytes.

### 3.12 Line height: computed at build, from the face epaint will use

**Chosen.** `Builder::build` computes egui's Body row height from the installed
face's own metrics, with epaint's arithmetic — the first face of the
Proportional chain, its unscaled metrics at the default variation location,
scaled by `size · tweak.scale / units_per_em`, ascent, descent and line gap each
rounded with `round_ui` and summed (`epaint/src/text/font.rs:397-400`,
`:561-565`, `:587`) — and writes `Spacing::extra_text_line_spacing` as the
theme's line height minus that row, floored at egui's `0.0`, into every base and
role style of each variant (specification §6.15). The slider's `expansion`,
which depends on the same row height (specification §6.6), is computed there
too.

**Why at build and not at run time.** native-theme's `defaults.line_height` is a
dimensionless multiplier (`resolved.rs:76-77`) while egui's field is an additive
delta in points, so the conversion needs the row height egui lays Body text out
with. The earlier design asked egui for it at run time, because
`Context::fonts_mut` panics before the first pass (`context.rs:1114-1122`): a
begin-pass plugin recomputed the value every pass, wrote it into the two base
styles, restyled that pass's root `Ui` and, when the value moved, republished a
patched atlas so the role cells caught up; a public `extra_text_line_spacing`
function served integrations that drive passes by hand. All of that existed to
ask egui a question whose answer is a pure function of inputs the builder
already has. The row depends on the face, the size and the face's tweak scale —
all `Builder` inputs — and not on `pixels_per_point`, which enters only the
rasterising scale (`epaint/src/text/font.rs:567-568`), so egui's zoom needs no
recomputation; nor on the weight, since `row_height` reads the metrics at the
default variation location (`epaint/src/text/fonts.rs:870-872`). Computed at
build, the value is in every style from the first pass — role cells and base
styles alike, in an atlas the application holds, and under an integration that
drives passes by hand, where no pass hook runs (`egui/src/context.rs:962-967`).
The begin-pass hook, the republish, the public function and ledger item 29 —
scoped text keeping egui's `0.0` — went with it.

**Verified, not assumed.** Recomputing epaint's arithmetic outside epaint is
safe only if it agrees with epaint, so it was measured against a live
`Context`'s `FontsView::row_height` on egui 0.36.2 (rustc 1.98.1, 2026-09-25):
168 comparisons, 168 equal — egui's default face at every bundled preset's Body
size and at other sizes, in both modes, and the KDE and GNOME system faces at
`pixels_per_point` 1 and 2. Specification §13 T5 keeps it true, with egui's own
`row_height` as the oracle.

**Rejected: a caller protocol.** A `refresh_line_spacing(&Ui) -> bool` the
application must call fails silently when forgotten: text simply has the wrong
leading and nothing reports it. A value computed at build has no call to forget.

**The `>= 0.0` floor, and what it costs.** A negative
`extra_text_line_spacing` is undocumented upstream: egui clamps only its own
settings slider to `0.0..=20.0` (`style.rs:2024`), the field's whole doc is
"Additional vertical spacing between lines of text." (`style.rs:423`), and no
use site enforces a range. Negative values were run on egui 0.36.2 with rustc
1.98.1 under debug assertions — `-2`, `-8`, `-100` and `-1e6`, three passes
each of multi-line and wrapping labels, a multi-line `TextEdit` and a two-line
button, in a vertical and a horizontal layout, a `ScrollArea` and a `Window` —
with no panic. That shows no crash on those paths, not a guarantee on every
path, so the floor stays. What it costs was measured too. With egui's default
fonts every bundled preset in both modes wants more than egui's row, by `+0.50`
px (`macos-sonoma` at 72 DPI) to `+3.89` px (`windows-11`), so the floor never
bites. With the platform's own face — the default, now that `system-fonts` is on
— the difference is negative on both Linux presets, and tiny: `kde-breeze` with
Noto Sans (noto-fonts 2026.09.01) at 13.333 px wants 18.1333 px against egui's
row of 18.15625 px, −0.0229 px, and `adwaita` with Adwaita Sans (adwaita-fonts
51.0) at 14.667 px wants 17.7467 px against 17.75 px, −0.0033 px, the same at
`pixels_per_point` 1 and 2 and in both modes. The floor turns both into egui's
`0.0`, a line at most 0.023 px looser than the platform's. The same measurement
with the macOS and Windows Body faces is one of the open verification items
of specification §15, run on the CI runners.

**The losses are stated, not buried** (specification §6.15): one global `f32`
is exact for `Body` only, it is read at exactly two sites, and a `RichText`, a
`LayoutJob` or a pre-built `Galley` ignores it (`widget_text.rs:775-776`,
`widgets/text_edit/builder.rs:489-490`, `widget_text.rs:384-392`). The four
per-role line heights *are* exact and lossless, through `text_role_line_height()`
and `RichText::line_height(Some(..))` (`widget_text.rs:174`) — the only exact
per-role mechanism egui has.

### 3.13 Icons: the boundary is decode, not draw

**Chosen.** The connector does key and URI construction, `IconData` →
`ImageSource` / `Image`, the colour of a monochrome SVG icon, alt text,
animation scheduling and cache invalidation.
It does **no** decoding, **no** rasterising, and **no** loader installation.

**Why the boundary sits there.** egui core ships **zero** image decoders:
`Loaders::default()` starts with an empty image-loader vector
(`egui/src/load.rs:625`), so `try_load_image` returns `LoadError::NoImageLoaders`
(`egui/src/context.rs:3866-3868`) until the application calls
`egui_extras::install_image_loaders(&ctx)` (`egui_extras/src/loaders.rs:58`).

**Rejected:** depending on `egui_extras`, and enabling `native-theme/svg-rasterize`
by default the way both sibling connectors do
(`connectors/native-theme-gpui/Cargo.toml:27`,
`connectors/native-theme-iced/Cargo.toml:23`). An SVG icon's route here is
`egui_extras`'s own SVG loader, which the application installs and which
re-rasterises for the display's pixel density at no cost to this crate
(`Image::load_for_size`, `egui/src/widgets/image.rs:350-355`). That loader's
`svg` feature pulls `resvg 0.45.1` (`egui_extras/Cargo.toml:150-151`) while
`native-theme`'s `svg-rasterize` pulls `resvg 0.48.1`
(`native-theme/Cargo.toml:52`) — two semver-incompatible pre-1.0 minors — so
`svg-rasterize` on by default would compile a second copy of resvg, usvg and
tiny-skia into every binary that follows the route, for a rasteriser the route
never calls. `svg-rasterize` stays an **additive** opt-in and a pure forward of
native-theme's feature, for an application that installs no `egui_extras`
loader: it rasterises with `native_theme::rasterize::rasterize_svg`
(`native-theme/src/rasterize.rs:39`) and hands the result to
`icons::to_color_image` — never a silent switch. A dependency on `egui_extras`
would buy nothing either: the connector calls none of its items — it hands egui
a `bytes://…svg` URI that whichever loader the application installs decodes —
so it would only fix the application's loader features for it.

**Why the URI carries everything that can change a pixel.** Every egui loader
layer caches on the URI *string* (`egui/src/load/bytes_loader.rs:15-26`,
`egui/src/load/texture_loader.rs:49-59`), so two renderings sharing a URI would
see the first served forever. `egui::Image::tint` is deliberately excluded: it is
a draw-time multiply that does not change the texture.

The URI is `bytes://native-theme/` and a 64-bit hash of the final bytes — for
a raster icon, its width and height too — and `.svg` for an SVG, nothing
else (specification §9.2), so whatever changes the pixels — a tint, a frame, the load colour, an
edited file, a provider's bytes — changes the URI without the key naming it;
the alternative, one key field per input, needed an `IconKey::frame` and a
caller-chosen unique name per provider icon, and could not see an edited file.
No lookup input enters the URI: equal bytes are equal pixels, so two keys that
yield them sharing one cache entry is correct — egui keeps one rendering per
size of an SVG under a URI (`egui/src/load/texture_loader.rs:49-59`) — and no
name has to be encoded, where a `#` in one would cut the URI short
(`egui/src/load.rs:316`). A freedesktop icon is loaded in the text colour, as
the gpui showcase loads it
(`connectors/native-theme-gpui/examples/showcase-gpui/support.rs:366`); for an
icon that sets no colour, which usvg would otherwise draw black, the connector
replaces `currentColor` alone with that colour's `#rrggbb`, as iced does
(`connectors/native-theme-iced/src/icons.rs:292-293`), which is exactly what
usvg draws with `color` set to it, so the icon's own black stays (specification
§9.2).

**Why a monochrome icon is coloured in its bytes.** egui_extras's SVG loader
parses with `usvg::Options::default()` (`egui_extras/src/loaders/svg_loader.rs:39`)
and draws an implicit fill, or a `currentColor` no `color` resolves, black, and `Image::tint`'s
multiply leaves black black, so a Material or Lucide icon would draw black on a
dark scheme. Both siblings recolour the SVG text instead
(`connectors/native-theme-iced/src/icons.rs:277`,
`connectors/native-theme-gpui/src/icons.rs:1299`), each with its own copy.
The colouring is added to native-theme as one function,
`native_theme::icons::colorize_monochrome_svg`, with the gpui connector's
algorithm and tests; this connector bakes a bundled set's `IconKey::tint` into
the bytes with it (a freedesktop icon's tint only replaces its `currentColor`,
above), so URI and pixels agree (specification §4.10, §9.2), and the siblings switching
to it is a `docs/todo.md` item.

**Why the system sets' glyphs are coloured when they are loaded.** The
screenshots of CI run 36315092575 showed the showcases' toolbar icons vanish on
a light Windows window and on a dark macOS window: native-theme drew every
Segoe Fluent glyph white (the `GGO_GRAY8` mask as `[255, 255, 255, alpha]`,
`native-theme/src/winicons.rs`) and every SF Symbol as a black monochrome
template (`native-theme/src/sficons.rs`), and nothing let the application
choose. An `IconData::Rgba` reaches the connector as pixels, and
`Image::tint`'s multiply cannot turn black into a light colour, so the colour
is chosen where the pixels are made: `SfSymbolsLoader::color` and
`SegoeIconsLoader::color`, like `FreedesktopLoader::color`, in native-theme,
for every connector at once. A full-colour stock icon keeps its own colours.
The Segoe set's four action roles that were shell stock icons —
`ActionSearch`, `ActionSettings`, `ActionDelete`, `ActionPrint`, as
`SIID_FIND`, `SIID_SETTINGS`, `SIID_DELETE`, `SIID_PRINTER` — became the
glyphs `Search`, `Settings`, `Delete` and `Print`, because the other action
roles already are glyphs and those stock icons are full-colour Explorer icons
(on the runner `SIID_SETTINGS` drew a black box and `SIID_FIND` a colour
magnifier) that no load colour reaches.

**Why a missing icon stays missing.** `SystemTheme::icon_theme` is
`Option<Cow<'static, str>>` (`native-theme/src/lib.rs:483`): `None` where the
theme states no icon theme and detection fails, and detection reports why it
failed instead of naming a stand-in (`CHANGELOG.md:18`). A freedesktop icon
comes only from the chosen theme or its `Inherits=` chain; `hicolor` and the
pixmap directories no longer stand in (`CHANGELOG.md:63`). The connector hands
on the absence as it receives it and never substitutes a glyph from another
theme or set — that would put two icon sets in one window, the defect
native-theme itself stopped committing in v0.5.9.

**Why the atlas names the icon set.** `ThemeAtlas::icon_set()` and
`icon_theme(theme)` hand icon-using code the theme's own set and icon theme, so
the code that draws the window's colours and the code that picks its icons read
one source. Looking the set up anywhere else is how one theme's colours end up
beside another theme's icons. The icon theme is asked per colour scheme because
the variants name different ones — `breeze` and `breeze-dark` in `kde-breeze`
(`native-theme/src/presets/kde-breeze.toml:9`, `:317`) — and egui can switch
between its two styles in any pass; a single name would give the dark style the
light scheme's icons. `SystemTheme` records only the active variant's name
today (`native-theme/src/lib.rs:470-483`), so native-theme gains the other,
`SystemTheme::icon_theme_for` (§4.3; specification §9.2).

**Why `IconKey` is a builder with private fields.** A `#[non_exhaustive]` struct
with public fields and no constructor cannot be built outside its defining crate
at all (`E0639`), which would make the whole icon module uncallable downstream —
a real API hole that a rejected draft shipped, and that its own example
demonstrated by failing to compile.

**Why `to_color_image` returns `Option`.** `ColorImage::from_rgba_unmultiplied`
carries an `assert_eq!` that fires in **release** too (`epaint/src/image.rs:113-120`),
so the size arithmetic is checked with `usize::checked_mul` and a mismatch
returns `None` instead of aborting the user's application.

**Why no icon size is written into `Spacing`.** The three icon-shaped `Spacing`
fields — `icon_width` (`style.rs:428`), `icon_width_inner` (`:432`),
`icon_spacing` (`:436`) — are *control* geometry: the checkbox box, the check
mark, and the default gap between **all** atoms in **every** `AtomLayout`
(`atomics/atom_layout.rs:302`). Writing an icon size into any of them would
resize every checkbox and radio button from an icon metric.

### 3.14 Watch and repaint: egui gets a real cross-thread wake

**Chosen.** A `watch` feature;
`ThemeWatcher::start(ctx: &egui::Context, rebuild: impl Fn() -> native_theme::Result<ThemeAtlas> + Send + 'static) -> native_theme::Result<ThemeWatcher>`;
on each OS change the watcher thread calls `rebuild` — re-detection,
re-resolution **and** atlas construction — then `Context::request_repaint`;
installation happens on the UI thread via `take()`. The closure's bound is the
one `native_theme::watch::on_theme_change` asks of its callback, `Fn + Send +
'static` (`native-theme/src/watch/mod.rs:217-218`), with no `Sync`: only the
watcher thread calls it.

**Why the watcher takes a rebuild closure.** An earlier signature took a
`FontPlan` and rebuilt from the system theme alone, which silently served the
wrong theme to an application built on a preset or on
`SystemTheme::with_overlay` (`native-theme/src/lib.rs:538`): the OS change would
arrive, and the watcher would replace the application's theme with a different
one. The closure is the application's own recipe, so the watcher repeats what
the application did. The system-theme rebuild the crate supplies,
`ThemeWatcher::system_rebuild()`, calls
`native_theme::detect::invalidate_caches()` (`native-theme/src/detect.rs:155`)
first, because detection caches the dark flag, reduced motion and the icon
theme process-wide (`:141-157`), and a rebuild that read the cache would
rebuild the old theme.

**What fires it, stated per platform, because a watcher that misses a change
is a stale look nobody reports.** native-theme's watchers
(`native-theme/src/watch/mod.rs:217-277`) fire on: KDE — a change to
`kdeglobals` or `kcmfontsrc` in the configuration directory
(`native-theme/src/watch/kde.rs:29-32`), firing on the first event of a burst
and dropping the rest for 300 ms (`:59-60`, `:86`); GNOME and Budgie — the portal's
`SettingChanged` in the `org.freedesktop.appearance` namespace only
(`native-theme/src/watch/gnome.rs:46`); macOS —
`AppleInterfaceThemeChangedNotification` only
(`native-theme/src/watch/macos.rs:88-89`); Windows — `UISettings`'
`ColorValuesChanged` (`native-theme/src/watch/windows.rs:77`); any other
desktop — `WatchUnavailable`. So a GNOME text-scale, font or icon-theme change
and a Windows text-scaling change reach the application only at its next start,
as does a macOS accent-colour or accessibility change wherever macOS posts no
`AppleInterfaceThemeChangedNotification` for it (unverified; specification
§10.2), and the last write of a KDE burst can go unseen; those gaps are filed
in `docs/todo.md` rather than papered over. One
defect sits under the connector as well: the GNOME watcher checks for shutdown
only after the next portal signal arrives (`native-theme/src/watch/gnome.rs:52-65`)
and registers no call that would wake it, while dropping the subscription joins
its thread (`native-theme/src/watch/mod.rs:183-185`) — so dropping a
`ThemeWatcher` on GNOME blocks until the desktop next changes a setting.
native-theme fixes that in this release (§4.3; specification §10.2) rather than
the connector working around it, because every connector's watcher sits on the
same thread.

**Rejected:** the sibling showcases' 500 ms polling flag, and installing from the
watcher thread.

**Why polling is not needed here.** `egui::Context` is
`Clone + Send + Sync + 'static` (`context.rs:722-723`, with upstream's own
compile-time assertion at `:4268-4272`) and `Context::request_repaint` (`:1821`)
documents that a call from outside the UI thread wakes it, provided the
integration installed a repaint callback — which eframe does on all three
backends (`eframe/src/native/glow_integration.rs:303`,
`eframe/src/native/wgpu_integration.rs:281`, `eframe/src/web/app_runner.rs:146`).
The iced and gpui showcases poll a flag every 500 ms
(`connectors/native-theme-iced/examples/showcase-iced.rs:5937`,
`connectors/native-theme-gpui/examples/showcase-gpui/app.rs:1530`); for iced
that is a choice rather than a limit of its runtime, since `iced_futures` 0.14.0
can deliver the change through `Subscription::run` over a `stream::channel`
(its `src/subscription.rs` line 182 and `src/stream.rs` line 11). Whatever the
siblings do, egui needs no timer: there is no interval to tune.

**Why the whole rebuild happens off-thread.** Atlas construction is pure CPU,
needs no `Context`, and `ThemeAtlas` is `Send + Sync + 'static`, so D-Bus,
registry and `CFRunLoop` work all stays off the UI thread.

**Why installation does not.** `Context::set_style_of` takes `&self` and would
compile from the watcher thread — but each `Ui` snapshots its `Arc<Style>`
exactly once (`ui.rs:136`, `:237`) and never re-reads it, so a mid-pass swap can
leave two halves of one frame in two different themes. `take()` is the hand-off,
and it is non-blocking and `None` on the overwhelming majority of frames.

**Why `install` ends by requesting a repaint.** Within a pass, every
descendant `Ui` keeps the `Arc<Style>` it inherited (`ui.rs:237`), and
`set_fonts` takes effect only at the next begin-pass (`context.rs:2104-2106`),
so an install is fully visible only on the pass after it. egui repaints on
input, not on a style change, so without `ctx.request_repaint()` as its last
step a watcher-driven install in an idle window would stay half-applied until
the user next moved the pointer.

`install` calls `icons::forget_icons(ctx)`, always, so the previous theme's
recoloured icons, whose URIs no pass under the new theme asks for
(specification §9.2), do not stay cached for the life of the `Context`, using `Context::forget_image` (`context.rs:3764`) per URI and removing the
stored `TextureHandle`s (specification §4.10), so unrelated application images
are left alone.

### 3.15 Re-exports: two crates in, four names deliberately out

**Chosen.** `pub use egui;` **and** `pub use native_theme;`, plus convenience
re-exports of the non-colliding native-theme names.

**Why both crates.** Two egui versions in one graph produce
`expected egui::Style, found egui::Style`, which is the single most common
downstream failure with a toolkit connector. Two lines of manifest make it
unrepresentable. This is a documented, argued deviation from both sibling
connectors.

**The rule that follows, and the four names it excludes.** The crate root
re-exports no name that also exists at `egui`'s root:

* `native_theme::color::Rgba` → `convert::Rgba` — opposite colour space to
  `egui::Rgba` (`egui/src/lib.rs:442`), see §3.10;
* `native_theme::theme::IconData` → `icons::IconData` — `egui::IconData`
  (`egui/src/viewport.rs:183`) is the window/taskbar icon;
* `native_theme::theme::Theme` — `egui/src/lib.rs:483` already exports `Theme`;
  it stays reachable as `native_theme::theme::Theme`;
* `native_theme::SystemTheme` — `egui::SystemTheme` (`egui/src/viewport.rs:1036`)
  reaches egui's root through `viewport::*` (`egui/src/lib.rs:493`); it stays
  reachable as `native_theme::SystemTheme`.

Each excluded name is still reachable in one hop from a module whose name says
what it is for. This is the same rule that produced the handle's name (§2.8).

### 3.16 Features: the native look on by default

**Chosen.** Six features, every one a 1:1 pure passthrough to a `native-theme`
feature of the same name — `material-icons`, `lucide-icons`, `system-icons`,
`system-fonts`, `svg-rasterize` and `watch` — with
`default = ["material-icons", "lucide-icons", "system-icons", "system-fonts"]`.

**Why these four are on.** The project's goal is as native a look as possible
when the platform theme is selected, and a look that needs features switched on
is opt-in fidelity by another name. `system-icons` is what finds the platform's
own icons, `system-fonts` its own typeface (§3.11), and the two bundled sets are
the icon sets the presets name where they are not a platform's — `material` for
Material, `lucide` for the rest (`native-theme/src/presets/material.toml:4`,
`native-theme/src/presets/nord.toml:4`). The icon features match the siblings'
defaults: gpui's is
`default = ["material-icons", "lucide-icons", "system-icons", "svg-rasterize"]`
(`connectors/native-theme-gpui/Cargo.toml:27`), and iced's is the same four
plus its own `widgets` coverage feature
(`connectors/native-theme-iced/Cargo.toml:23`); its icon features are on by
default so that `load_icon`
works for an application that depends on the connector alone
(`CHANGELOG.md:38`) — the same reason applies here. `widgets` has no egui
counterpart, because this connector ships no widgets (§3.22); `system-fonts` is
the addition, because egui is the one toolkit here that cannot find a system
face by name (§3.11). The iced connector gains a `system-fonts` feature of its
own in this release, on by default for the same reason, for the family name
its font database holds (§3.11, *The sibling connectors*).

**Why `svg-rasterize` is the one sibling default left off.** Here it is not
what draws an SVG icon: `egui_extras`'s loader is, and turning the feature on
would build a second, semver-incompatible resvg beside that loader's (§3.13).
The native look loses nothing by its absence, and every binary on the
recommended route would pay for it. `watch` stays off because a watcher is a
thread the application should decide to run, and without it the look is still
right at every start.

**Rejected: `default = []`,** this document's earlier decision, argued as "a
theme connector should not force bundled icon blobs or an SVG rasteriser on
every downstream binary". The rasteriser half stands; the other half weighs
binary size against the goal and lets size win. The cost is real and stated —
the icon blobs, and fontdb with fontconfig on Linux — and an application that
must shed it sets `default-features = false` and names what it keeps; the
default serves the look.

No feature name is invented at this layer, so a native-theme feature rename is a
one-line change here and never a semantic divergence.

### 3.17 Semver and the egui version policy

**Chosen.** `egui = "0.36.2"` — a caret requirement, `>=0.36.2, <0.37.0`, with
egui's default features kept on purpose (specification §11) — and `0.36.2` for
every other egui-family crate the connector names. One egui minor per release line; an egui minor bump
is a **breaking change** for this crate. Crate version equals the workspace
version. The same shape as `iced_core = "0.14"`
(`connectors/native-theme-iced/Cargo.toml:36`) and
`gpui = { package = "gpui-pre", version = "0.3.6" }`
(`connectors/native-theme-gpui/Cargo.toml:40`).

**Why the floor is 0.36.2 and not 0.36.1.** Every egui citation in
both documents was re-read against 0.36.2, released 2026-09-08, and one of its
changes moves a surface this crate themes. A `Window` given a custom `frame` and
no `title_frame` now draws its title bar from **that custom frame**
(`title_frame.unwrap_or(window_frame)`, `egui/src/containers/window.rs:630-631`);
0.36.1 drew it from `Frame::window(&style)`, ignoring the custom frame. So
`Surface::Window` and `Surface::WindowTitleBar` render differently on the two
patch releases, and a requirement admitting 0.36.1 would admit a version on
which the documented behaviour of those surfaces is not what the specification
describes. The patch also makes `TextEdit` honour `min_size` in both axes
(§2, Option E). The sibling connectors set floors for the same kind of reason:
gpui's gpui-component requirement is a hard floor because a patch release, 0.6.2, removed a field the
older connector wrote (`connectors/native-theme-gpui/Cargo.toml:41-43`). MSRV
is unaffected — egui 0.36.2 still declares `1.95` (§3.18).

**Rejected:** an exact `=0.36.2` pin (it would reject later patch fixes) and any
wider range (cargo cannot unify two semver-incompatible egui versions, and the
application's `egui::Style` must be *our* `egui::Style`).

**The clause that matters most is clause 3: the numeric contents of a produced
`egui::Style` are not covered.** Electing a different base owner, correcting a
sink, closing a `Note` — none of those is a breaking change. Without that clause
every future mapping correction would need a major version, and the mapping
*will* be corrected: §7 lists the corrections made before a line of code was
written.

**Clause 4 — `#[non_exhaustive]` on every public enum except `PanelSide`** —
exists because native-theme churn is an axis independent of egui churn. §5.2 is
the worked example.

### 3.18 MSRV: a measured workspace floor of 1.88.0, and 1.95 for this connector

egui 0.36.2 declares `edition = "2024"` (`egui/Cargo.toml:13`) and
`rust-version = "1.95"` (`:14`), above the workspace floor.

**Chosen.** The workspace declares `rust-version = "1.88.0"` (`Cargo.toml:15`);
`connectors/native-theme-egui/Cargo.toml` declares `rust-version = "1.95"`
explicitly and does **not** inherit. `rust-version` is a per-package key whose
workspace inheritance is opt-in, so declining to inherit is ordinary and
supported.

**Why `1.88.0`.** It is the maximum of two independent lower bounds — what the
dependency graph demands and what our own sources demand — and both land on the
same number. A third check, compiling against that exact toolchain, confirms it.

*The dependency bound, measured rather than asserted.* `cargo metadata
--offline --all-features --locked` over the committed `Cargo.lock` reports
**1009 packages**, of which **646 declare a `rust-version`** (535 distinct
package names; 734 packages, 472 declaring, under `--filter-platform
x86_64-unknown-linux-gnu`). Outside the gpui connector's closure the highest declared is
**`1.88.0`**. Four packages declare more, and all four are the gpui connector or
reach the graph only through it: `native-theme-gpui` 0.5.9 (`1.95.0`), `oo7`
0.6.0 (`1.92`), `cosmic-text` 0.19.0 and `smol_str` 0.3.6 (`1.89`). At `1.88.0`
sit, among others, `darling` 0.23.0, `image` 0.25.10, `serde_with` 3.22.0,
`time` 0.3.55, `wgpu` 27.0.1 and the other four workspace members, some spelling
it `1.88`. Measured **2026-09-25**, before this crate joins the graph. The first measurement, on
2026-08-10 and before the gpui connector declared `1.95.0`, found 957 packages,
577 declaring (476 names), and `1.88.0` as the highest anywhere.

*The toolchain bound.* Re-measured 2026-09-25: `cargo +1.88.0 check --workspace
--exclude native-theme-gpui --all-features --locked` is clean. On 2026-08-10
every workspace member was also compiled with `cargo +1.88.0 check -p <member>
--all-targets --locked`, clean for all five, tests included; that per-member
`--all-targets` run has not been repeated. The MSRV job checks library targets
only, which is what an MSRV promises a consumer: a dev-dependency is not part of
a downstream build, so one that stops compiling on `1.88.0` is not an MSRV
violation.

*The gpui precedent.* v0.5.8 re-measured the workspace floor and left it at `1.88.0`
(`CHANGELOG.md:112`), and moved the gpui connector to its own
`rust-version = "1.95.0"`, declared and not inherited because the `gpui-pre`
closure requires it (`connectors/native-theme-gpui/Cargo.toml:6-10`, measured
2026-09-19; `CHANGELOG.md:96`). That is exactly the shape chosen here for egui,
so the workspace now carries one precedent for a connector floor above the
workspace's, and the 2026-08-10 "all five on `1.88.0`" result no longer holds
for the gpui connector (its manifest records `1.94.0` failing).

*Our own sources need `1.88.0` too.* An earlier draft of this section recorded
that a sweep for let-chain syntax found none in our own code; **that was
false**, and the trigger built on it is corrected below. Let-chains stabilised
in 1.88 and are available only under edition 2024 (`Cargo.toml:13`); a grep for
`&& let ` across `native-theme`, `native-theme-build`, `native-theme-derive` and
both existing connectors returns **69** occurrences at `2db686ee` (59 when this
section was first measured) — for example
`native-theme/src/detect.rs:247-249`,
`native-theme-derive/src/gen_ranges.rs:178-179`,
`native-theme/src/spinners.rs:78-79` and
`native-theme/src/resolve/inheritance.rs:46`. That grep undercounts, because it
does not see a chain whose second term is a bare boolean, such as
`native-theme/src/model/font.rs:331-332` or
`native-theme-derive/src/gen_ranges.rs:146-147` (the font-spec check beside the
padding check §3.9 cites).

So the floor is **held up from both sides**: it is the lowest value the
dependencies promise to support *and* the lowest value our own code compiles
under. Lowering it would not merely outrun a dependency's promise — it would
fail to build `native-theme` itself.

**Rejected: pinning the workspace to current Rust stable** (`1.97.1` at the time
of writing). This was briefly adopted, on the argument that all six toolchain
installs in `.github/workflows/ci.yml` were `@stable` (seven today, lines 18,
42, 73, 97, 109, 128 and 165, all still `@stable`), there is no
`rust-toolchain.toml`, and `scripts/check_release.sh` has no MSRV check — so any declared number was an untested claim, and one equal to what CI
runs is at least true by construction.

**Why that reasoning was wrong.** The remedy did not match the defect. The fix
for an unverified number is to verify it, not to raise it until verifying it is
vacuous. Pinning to stable purchases honesty by discarding the entire point of
declaring an MSRV: it makes `native-theme` — the crate with the broadest audience
— demand a toolchain none of its dependencies require, excluding every user on
anything older than the newest release, and it moves the floor every six weeks
for no engineering reason. A low floor that is measured and enforced dominates a
high floor that is true only by tautology.

Also rejected: silently inheriting the workspace floor in the connector and
hoping. The connector would then declare `1.88.0`, a toolchain it cannot build
on, which is a false statement in its own manifest; and cargo's error on that
toolchain would name egui (`dep@… requires rustc 1.95`, measured on a two-crate
fixture with `cargo +1.88.0`), pointing the reader at the wrong crate.

**The caveat that now carries the weight.** A measured floor decays the moment
any dependency raises its own requirement, and nothing currently re-checks it.
`1.88.0` is honest as of the measurement date and no longer than that. The MSRV
job in specification §12.4 is therefore not a nicety but the thing that keeps
this decision true; the implementation plan adds it as its first
release-readiness step.

### 3.19 Diagnostics: `Note`, because silent correctness is unobservable

**Chosen.** `ThemeAtlas::notes() -> &[Note]`, populated at build time, with six
`#[non_exhaustive]` variants: `ValueSanitised`, `ValueSaturated`,
`FontFamilyUnavailable` (the platform's family is not installed, §3.11),
`FontDataInvalid` (a face `skrifa` rejects, dropped before epaint parses it,
§3.11), `FontWeightAxisUnsupported` and `TransparentFill` (a native alpha-0
colour written as given to a `bg_fill` that egui documents must never be
transparent, `egui/src/style.rs:1292-1294`).

**Rejected:** silent substitution only. `convert` substituting for `NaN` and
`±∞` is correct behaviour, but it is *unobservable*: the only symptom is a
rendering anomaly nobody can trace. A non-fatal channel makes a sanitised or
saturated value show up in a log, and it costs one `Vec<Note>` on a type
constructed once per theme change.

The distinction between the first two variants is deliberate and load-bearing.
Corner-radius saturation at 255 is **benign** — the tessellator re-clamps to half
the smaller side (`epaint/src/tessellator.rs:638-642`), so 255 simply reads as
"pill" — and emits nothing. Margin saturation at the `i8` bounds (−128 / 127) is a **real** loss of theme
data and emits `ValueSaturated`. A diagnostic channel that cried wolf on the
benign case would be ignored on the real one.

### 3.20 `mapping.toml`, and why the coverage table is a test rather than a claim

**Chosen.** One checked-in row per native leaf, carrying `verdict`, the declared
egui `sinks`, and — for `unmappable` — the upstream change that would close it.
Rendered into the published rustdoc, through the generated `src/mapping.md`
(specification §5.7), not merely kept repo-local.

**Why a manifest at all.** It is the only mechanism in any of the candidate
designs that detects **native-theme** drift. An egui bump is caught by the
compile-time tripwires of specification §13 T1; a native-theme field addition is
caught by nothing else, and would otherwise appear as a leaf that quietly maps
nowhere.

**Why the differential test has no skip allowance.** T2 splices one native leaf
from preset B into preset A, rebuilds, and asserts that the set of changed egui
fields equals the row's declared `sinks` — a base-style sink also in every role
cell that does not override it, since a cell starts from the base style — with
one allowance that is not a skip: a formula output another input can hold still
is marked `when`, and T10 checks its value instead. A row whose leaf reaches its
widget by no `Style` field names the test of its route in `tested_by`, and T2
asserts it changes nothing. A row that no preset pair differentiates
**fails**, with an actionable message, and must be given an explicit probe value.
There is deliberately no discretionary knob a maintainer can raise instead of
fixing a mapping — because the knob would be raised.

**Why the base-owner table is published.** A contested field always means
somebody loses, and the loser must be nameable by a reader who never opens the
manifest. Specification §5.9 is what lets a user predict what an *unscoped*
widget looks like; hiding it in a repo-local file would make the coverage numbers
a claim about a document instead of a description of the code.

### 3.21 Fields deliberately never written

Specification §5.10 lists the `Style` fields never written, each with its
reason. The principle behind the list has two halves. **Writing an inert field is a silent
no-op, which is the "plausible fabrication" failure mode this project forbids**:
the crate would appear to theme something it does not. And writing a live field
that no native leaf states (scroll fade, IME underline, code background, indent
rules, `striped`, `interaction`) would invent a value.

`Spacing::menu_width` (`style.rs:453`), `Spacing::menu_spacing` (`:456`),
`Style::compact_menu_style` (`:341`) and `Visuals::window_highlight_topmost`
(`:1067`) have **no functional reader anywhere in egui 0.36.2**. Each appears
exactly four times in the crate: its declaration, its default (`:1471`, `:1472`,
`:1447`, `:1528`), a destructure inside egui's own settings UI, and the widget
that edits it there (`:1962`/`:2032`, `:1963`/`:2037`, `:1805`/`:1911`,
`:2302`/`:2476`). No paint or layout path consults any of them — which is the
precise claim, and it is stronger than "no occurrences" because the settings-UI
occurrences do exist. `Visuals::clip_rect_margin` (`:1081-1087`) is
`#[deprecated]` and documented "Setting it now has no effect". `Style::interaction` (all eight fields,
`:911-945`) is input-behaviour policy — hit-test slop, tooltip timing,
text-selection policy — and `ResolvedTheme` exposes none of it, so writing it
would mean inventing platform behaviour, not mapping it. `Visuals::striped`
(`:1100`) gates striping entirely, and turning it on because an alternate row
colour exists would be inventing platform policy from the presence of a colour.

`Spacing::default_area_size` (`style.rs:445`) is the sharpest case, because it *looks*
like the dialog-size sink: `dialog.max_width` / `max_height` map onto it
naturally. It is left alone because it is the first-frame size of **every** free
`Area` (`containers/area.rs:470-476`) — window, popup, menu, tooltip — so writing
it would resize all of them from a dialog metric. The application reads
`dialog.max_width` and `max_height` from the `ResolvedTheme` for its own
`Modal` instead (specification §5.2).

`Visuals::override_text_color` (`style.rs:1016`) is never written on the **base** style
for the same class of reason: it forces one colour on *all* text, destroying
every per-state and per-widget text colour, and it would turn a `ProgressBar`
label into ordinary body text on an accent fill (`widgets/progress_bar.rs:197-199`).
It **is** written in the `Role::Checkbox` scope, and only there, because
`Style::checkbox_style` takes the label colour from `ws.text` (which honours it,
`widget_style.rs:133-136` → `:187`) and the check-mark stroke from `ws.stroke`
(= `fg_stroke`, `:188`). That is the only exact way to separate
`checkbox.font.color` from `checkbox.indicator_color`, and they are meant to
differ: `indicator_color` inherits `defaults.accent_text_color`, the colour made
for the accent fill the mark sits on (`native-theme/src/model/widgets/mod.rs:154-155`),
while the label is body text.

**What the application owns: `Builder::style_patch`.** The fields above are
left at egui's values because no platform states them, not because an
application may not want its own — scroll behaviour, `interaction` timing,
`striped`, debug overlays. But every style the atlas publishes starts from
`egui::Theme::default_style()` (§3.3), so a setting the application made with
`ctx.all_styles_mut` before installing is gone after it, gone again after every
watcher rebuild, and never reached the role styles at all. `Builder::style_patch(f)`
is the one door: `f: impl Fn(&mut egui::Style) + 'a` is applied **last** to
every base and role style when the builder builds, and is not kept after it; a
watcher rebuild applies it again because the rebuild closure is the
application's own recipe (§3.14). Rejected: asking the application to reapply its
settings after each install, which is the forgettable protocol §3.12 refuses and
still misses the role cells; and exposing the atlas's styles mutably, which
would break the opaque handle (§3.1). Because it runs last, a patch can
override a native value too; that is documented as deliberate — the
application's explicit choice outranks the theme — and it is the application's
code, never this crate's, that makes it.

### 3.22 The no-widgets charter

Recorded in specification §14.3 as a two-test predicate rather than an opinion:
a widget may be added only if egui 0.36.2 has no widget with that visual identity
**and** a majority of the corresponding `ResolvedTheme` struct's leaves are
otherwise UNMAPPABLE. `Switch` passes the first test and fails the second (§2,
Option B), and it would be declined even if it met both: a shipped widget buys a
permanent per-release audit obligation — interaction, animation, `WidgetInfo`,
`AtomLayout` — in a crate whose remit is theme mapping. The charter bounds this
crate's remit, not the native look. What egui cannot draw natively is the
companion crate `native-theme-egui-widgets`'s to supply — a switch with its
knob, a slider whose knob is not its rail, a spinner at the theme's stroke,
and a segmented control
([`todo_egui-widgets-spec.md`](../todo_egui-widgets-spec.md) §0.1) — and only
that: where a role scope or a surface frame already gives egui's own widget the
native look, that crate adds nothing. Links in their state and visited colours
are not on that list: egui's own `Link` takes them per call, a route this crate
grades DERIVED (specification §5.3), and that crate's link wrappers are only the
ready-made form of that call.

**The switch substitute, and why it is a selected `Button`.** Without a switch,
the connector's `Role::Switch` styles a stand-in, and the stand-in decides how
much of the switch reaches the screen. egui's own on/off control,
`Ui::toggle_value`, is a `selectable_label` (`egui/src/ui.rs:1875-1882`) — a
`Button::selectable`, which sets `frame_when_inactive(selected)`
(`egui/src/widgets/button.rs:78-82`) — so an unchecked toggle paints no frame at
rest (`:364-368`), where a native switch always shows its track.
`Button::new(..).selected(checked)` keeps the frame in both states: the
unchecked track and its disabled form are painted from the scope's resting
fill, and the checked track from `selection.bg_fill`, which `button_style`
substitutes for a selected button (`egui/src/widget_style.rs:150-152`). That is
why `switch.unchecked_background` and `disabled_unchecked_background` are
reachable at all. Rejected: `toggle_value`, which drops the unchecked track;
a `Checkbox`, whose box is not a track; and painting a track and knob with the
painter, which is a widget by another name. The knob stays lost — no egui
widget draws one inside a track.

The charter exists in this form so that the question is **closed**. An opinion
gets re-litigated every release; a predicate over counted leaves is settled by
recounting.

### 3.23 Unstated sizes: a `None` keeps egui's own value

**Chosen.** A widget's resolved padding is four
`Option<f32>` sides (`ResolvedPadding`, `native-theme/src/model/border.rs:195-204`).
Where egui's sink is an `epaint::Margin` — four `i8` sides
(`epaint/src/margin.rs:15-20`): `Spacing::window_margin`, `Spacing::menu_margin`
(`egui/src/style.rs:395`, `:401`) or a `Frame`'s inner margin — each stated side
goes to its own side through the saturating conversion of §3.9, and a `None`
side keeps the value egui's own default `Style` for that scheme has on that
side. `convert::to_margin` takes that base margin as a parameter, the
counterpart of iced's `padding_or` (`connectors/native-theme-iced/src/lib.rs:329-339`),
because only the call site knows which sink, and so which default, it is. The
five optional sizes — `toolbar.bar_height`, `toolbar.item_gap`,
`menu.row_height`, `list.row_height`, `combo_box.arrow_area_width` — follow the
same rule: `None` means egui's own behaviour stands, and an application that
reads one of them from the `ResolvedTheme` gets the `Option<f32>`. `Some(0.0)`
is a stated zero and is written.

**Why.** Until v0.5.9 the resolver turned a size the platform does not state
into `0.0`, and a connector then wrote that zero over the toolkit's own padding:
content touching its frame, a look no platform has. The unstated-sizes work
weighed five answers — a zero, a stand-in number, a derivation, a resolution
error, or leaving the value absent — and only the last says nothing the platform
did not say, while a toolkit's default is a considered design value and `0` is
not (`docs/archive/todo_v0.5.9_unstated-sizes-and-chrome-ux-rationale.md` §3).
Both siblings apply it the same way: gpui's `geometry` builders set only the
padding sides, heights and gaps the theme states (`CHANGELOG.md:27`), and iced's
`button_padding` / `input_padding` fill an unstated side with iced's own default
padding (`CHANGELOG.md:34`). In egui the rule costs almost nothing, because of §3.3: every style starts from
`egui::Theme::default_style()` and assigns, so a field the connector does not
write already holds egui's value. The one place that needs care is a sink
written *per side*, where a stated left and an unstated right must not turn into
a written `0` on the right — hence the `base` parameter.

**Where egui's sink cannot hold four sides.** `Spacing::button_padding` is a
`Vec2` (`egui/src/style.rs:398`), one value per axis added to both sides of it
(`egui/src/widget_style.rs:163-165`), and it is the sink for the button, tab,
combo-box, segmented-control and expander paddings. A platform that states two
different sides of one axis — Windows' button, 5 top and 6 bottom
(`native-theme/src/presets/windows-11.toml:98-99`) — cannot be carried exactly
there, and neither can one stated side of an axis whose other side is `None`.
The goal, the native look, decides what such a sink gets. Give each side a
*target* — the side the platform states, or the starting style's component for a
side it does not (egui's on the base style, the base style's inside a role cell,
spec §3.4), which is the same per-side rule as above — and write one value `v` to
both sides. The widget's outer size along the axis is native only when `v` is
the mean of the two targets, and the worse of the two sides is then off by half
their difference; any other `v` is off by more on one side and changes the size
as well. One more term enters: a platform's padding lies inside its border
(`docs/platform-facts.md:900-902`), while `Button`, `ComboBox`, the collapsing
header and `TextEdit` draw their stroke inside the padding egui gives them
(`egui/src/widget_style.rs:163-165`, `egui/src/containers/frame.rs:327-331`;
`egui/src/containers/combo_box.rs:439-460`), so a stated side's target is the
side plus the border's line width, and only then is the outer size native. So
the mean plus the line width is written: Windows' button, 5 and 6 inside a
1-point border (`native-theme/src/presets/windows-11.toml:43`), gets `6.5` on
both sides, the height one point off native (`Button` rounds its margin to whole
points, `epaint/src/margin.rs:115-120`, and 5 + 6 is odd, so no symmetric value
is exact) and the content half a point off-centre, where leaving egui's `1.0`
would make it eleven points shorter than a Windows button. The mean is a
derivation with a stated formula over two values the platform gave, so it
invents nothing, and the rows that use it are DERIVED, lossy in centring and,
for an odd pair sum, one point of size
(specification §5, §14 item 32). An axis whose two sides are both unstated gets
egui's own value, as every other unstated size does.

### 3.24 The v0.5.9 connector contracts, and where each lands here

v0.5.9 made a set of rules binding on every connector
(`docs/archive/todo_v0.5.9_theme-contracts-rationale.md` §3). This design
predates them; each either holds by its construction already or is adopted.

* **C6 — a mapping-contract table iterated over all 32 preset/mode
  combinations, with a coverage tripwire over every slot.** `mapping.toml` with
  its differential and converse-coverage tests (§3.20) is this crate's table and
  tripwire; C6 fixes the domain the assertions run over.
* **C7 — the showcase has `test = true` and self-tests that render and click
  headlessly.** This is what turns `egui_kittest` from a declined dependency
  into a needed one (§2.8; §8, Q-3).
* **C8 — the connector never makes a text-on-background pair less readable than
  the platform's own values.** In this design a colour is copied or, for a
  state layer, composited exactly as the platform composites it (§3.6); nothing
  substitutes a "readable" colour, the substitution iced's palette layer dropped
  in v0.5.9 (`CHANGELOG.md:33`). C8 is asserted where the connector chooses both
  colours of a pair; a sub-AA pair that is real platform data is reported, not
  asserted.
* **C16 — a `soft_option` colour that is `None` is filled by copying the base
  state, never by arithmetic.** The rule originated in §3.6 of this document.
* **C17 — hover and pressed colours are state layers.** §3.6.
* **C19 — no status-label contrast enforcement.** The status colours reach the
  application as the platform states them. Both siblings removed
  `ensure_status_contrast` in v0.5.9 after measuring that it chose the wrong one
  of white and black in 31 of the 37 labels it touched.

One item of that document's "not done" list names this connector as its
trigger: *a shared contract vocabulary across connectors (one table, two
backends)*, deferred until "a third connector"
(`docs/archive/todo_v0.5.9_theme-contracts-rationale.md:548`). egui is the
third. It is decided in §8, Q-5: each connector keeps its own contract module,
and this one writes its own in the siblings' shape; a shared crate waits for a
shared pair list or a fourth connector.

### 3.25 The showcase: the gpui application, in egui's own containers

**Chosen (specification §10.4; §8, Q-6).** The egui showcase is
the gpui showcase's application — menu bar, toolbar, a resizable side panel
with the theme settings and an inspector, page tabs, status bar, command
palette, Preferences, About — built from egui's own containers, with
per-instance **Widget Info** generated from `mapping.toml`, and a palette of
every widget egui 0.36.2 and egui_extras 0.36.2 offer an application: 110
items, 107 on ten palette pages and three in the chrome itself. With it, egui is the second showcase with per-instance
info; the iced showcase keeps its own `docs/todo.md` entry.

**Rejected: the iced baseline now, the application later.** The previous
revision gave the egui showcase the iced showcase's scope — every widget egui
offers that native-theme models, and the self-tests — and filed the chrome and
the per-instance info in `docs/todo.md`, on two arguments: that the release
gates can check a widget gallery, and that the application layer would need
egui twins of gpui's citation gates and answers to two open questions. The
maintainer rejected it: the gpui showcase has a menu, a status bar and better
Widget Info, and egui covers different widgets from gpui and iced, so its
palette should cover everything egui offers.

**Why the rejection is right, argued from the goal.** The project's overriding
goal is as native a look as possible when the platform theme is selected, and
the showcase is where that look is shown and checked. Four facts decide it.

* **The look in question is an application's.** Menus, toolbars, panels and
  status bars are themed by this crate's seams too — `Role::Menu`,
  `Role::Toolbar`, `Role::StatusBar`, the four `Surface::Panel` sides — and a
  gallery leaves those seams undemonstrated and untested under every preset.
  The chrome is drawn through them, so it is the demonstration.
* **egui can express the chrome.** Every element of the gpui chrome maps onto
  an egui container: the chrome bar and status bar onto `Panel::top` and
  `Panel::bottom`, the side panel onto a resizable, collapsible `Panel::left`,
  the menus onto `MenuBar` with its `StyleModifier`s, the command palette onto
  a `Modal` with a `TextEdit` and a list, Preferences onto a `Window`
  (specification §10.4's chrome table cites each). The one gpui element with no
  egui counterpart, the title bar, is the one the gpui showcase stopped drawing
  itself where the OS will: it asks for server-side decorations
  (`docs/archive/todo_v0.5.9_showcase-layout.md` S8), and eframe's winit asks
  for them by default, so the egui showcase draws none at all. The first open
  question is answered.
* **egui says which instance is innermost.** Each pass, egui's hit test
  records every widget that contains the pointer, on the layers the pointer
  actually reaches, and each widget's `Response::contains_pointer` reports it
  (`egui/src/response.rs:333`, set at `egui/src/context.rs:1459-1462`). Among
  the showcase's own records of that pass, the smallest rect wins — the gpui
  rule. Paint order would not do, because a `Frame` enters its rect after its
  contents (`egui/src/containers/frame.rs:416-418`, `:488`), so the last
  widget under the pointer is often the container. The second open question
  is answered.
* **Generated Widget Info is cheaper and stronger than cited prose.** gpui's
  info is written by hand, and each colour claim is held true by a citation to
  the upstream line it was read at; six gates keep that prose honest
  (`docs/archive/todo_v0.5.9_showcase-app-spec.md` §10.2), and what they still
  cannot check — role labels, semantics — is left to review
  (`docs/archive/todo_v0.5.9_widget-info-spec.md` §4.7). egui's connector
  already keeps a manifest, one row per native leaf with its sinks, its verdict
  and, for a loss, the upstream change that would close it, and T2 and T3
  prove every row's sinks and completeness, and T10 its value in all 32 preset
  and mode combinations. An info that is
  those rows, selected by the seam the widget is drawn with and valued from
  the installed theme, is checked by construction; the only gates it needs
  check which seams an instance is drawn with (specification §13.2). Its
  `unmappable` rows are the honest half of native fidelity: an info says what
  the platform states that egui cannot show here, and what would fix it. The
  third question — share gpui's citation gates or build egui twins — has the
  answer neither.

**On macOS the menus are the system's.** A macOS application's menus live in
the system menu bar, and the gpui showcase keeps them there
(`docs/archive/todo_v0.5.9_showcase-layout.md:109`); an in-window menu row on
macOS would be an imitation of a look the platform draws elsewhere. eframe
offers no route in — a grep of eframe 0.36.2's `src/` for `NSMenu`, `menubar`
and `set_menu` finds nothing — and building items with objc2 directly needs
`unsafe fn`s the no-`unsafe` rule excludes. The showcase therefore takes muda,
the latest release 0.20.0 (rust-version 1.90, under this connector's 1.95), as
a macOS-only dev-dependency with `default-features = false`: its default
`libxdo` and `gtk3` features enable only Linux and BSD dependencies (muda
0.20.0's `Cargo.toml`, `default = ["libxdo", "gtk3"]`, `gtk` 0.18 under
`cfg(any(target_os = "linux", …))`), so they would add the gtk-rs stack to
`Cargo.lock` and change nothing on macOS. `NativeOptions::event_loop_builder`
(`eframe/src/epi.rs:351`) turns winit's default menu off with
`with_default_menu(false)` (line 446 of winit 0.30.13's `src/platform/macos.rs`),
muda's safe `Menu::init_for_nsapp` installs the bar (line 550 of its
`src/items/menu.rs`), and `MenuEvent::set_event_handler` (line 47 of its
`src/menu_event.rs`) feeds a channel and calls `ctx.request_repaint()` so a menu
command wakes an idle window. Linux and Windows keep the in-window `MenuBar`,
as the gpui showcase keeps its menu-bar row there. That the bar is installed is
checked at runtime, not assumed: the showcase's `--screenshot` run reads back
`NSApplication::mainMenu` and fails unless it holds as many items as the
showcase built (specification §13's runner checks), and the implementation plan runs
that capture on the `macos-latest` runner the screenshot workflow already uses.
That its commands arrive by keyboard — that AppKit hands a menu accelerator to
the menu before winit's view sees the key — no runner can show, and stays
UNVERIFIED until run on a Mac (specification §10.4). The §6 entry that recorded the macOS menu bar as
not done is retired by this.

**Why the palette is all of egui, not only what native-theme models.** A
widget with no native counterpart, drawn in the themed application with
egui's defaults, is part of the answer the showcase gives: it shows the reader
exactly where the platform look stops. It also closes the coverage gap the
earlier scope left open, since a gate that knows only the modelled widgets
cannot report a new egui widget at all; the coverage script now discovers
egui's and egui_extras's widgets from their sources, so a new one fails the
gate until it is shown or excepted with a reason (the rules are the implementation
plan's).

**Two smaller choices, both by the look.** The window title is fixed,
`WINDOW_TITLE`, the crate's name and version, as the gpui showcase's is: the
maintainer asked in v0.5.9 for the version in the title, and a title that
changed with the page would be a status line where the platform shows the
application's name. The page tabs and the segmented controls are rows of
`Button::new(..).selected(..)` inside one `Role::Tab` or
`Role::SegmentedControl` scope per row, whose `Normal` cell carries the active
colours in `selection.*`, which egui reads only for a selected button
(`egui/src/widget_style.rs:150-155`); not `Button::selectable`: a selectable button
paints no resting frame while unselected (`egui/src/widgets/button.rs:78-83`,
`:364-368`), so the unselected segment's `segmented_control.background_color`
and the resting tab's `tab.background_color` would never reach the screen,
while `Button::new` keeps the frame in both states. The cost is egui's and is
recorded as ledger item 43: `selected(..)` also makes screen readers announce a
toggle.

**The cost, and why it is accepted.** The showcase is larger than the
connector it demonstrates, in the release that introduces the connector —
the earlier revision's argument. It is paid in example code and tests, not in
the connector's API, and it buys the one thing a smaller showcase could not:
a place where the connector's promise is visible and checked. The residual is
stated in the specification: an info names what its seams write, not which of
those fields egui reads for that particular widget, because narrowing it would
take hand-written claims about egui's paint code — the kind this design
avoids.

---

## 4 -- Why each UNMAPPABLE class is genuinely unmappable

The UNMAPPABLE leaves (specification §5.7 counts them) are not one problem; all
but two have one of six causes, and the distinction matters because the fix for each lives
somewhere different — and **the `source-side gap` leaves are not egui's fault
alone** (§4.3). The rest are `egui-limited`, the `*.defined_size` leaves among
them, or `widgets-crate`: egui's stock widget has no route, and the companion
widgets crate paints the value on a widget of its own (specification §2). The `source-void` leaves the first version counted no longer exist in the
v0.5.9 model (§4.3).

How the rows fall (specification §5.7 lists every group): the `source-side
gap` leaves are §4.3's, the menu items' shadow among them; among the
`widgets-crate` leaves the switch's are Cause 3, `slider.disabled_thumb_color`
Cause 1 (a disabled handle and the rail share `inactive.bg_fill`) and
`spinner.stroke_width` Cause 4; among the `egui-limited`, the
`*.defined_size` leaves are Cause 3, the per-widget families Cause 6, and the
values the stock widget has no place for are Cause 3 where egui models
nothing (`slider.tick_mark_length`) and Cause 4 where its paint code fixes the
value (the progress bar's inset and the expander's leading padding; specification
§5.5, §5.6). Among the `defaults` leaves, which have no `Role` scope,
`accent_color`, `accent_text_color`, `surface_color` and `disabled_text_color`
are Cause 3 (`Visuals` has no accent or surface field, and egui a disabled
opacity, not a colour). `font.color` and the text-selection pair are Cause 1's
intra-widget kind: each shares its one egui field with the `defaults` leaf that
owns it. Each of the seven reaches the screen through the leaf that owns its
field or the leaves that inherit it (specification §5.1). Only the progress
bar's border colour and width have no cause below: a `Frame` laid round the bar
could carry them, and they are declined because no platform-facts row states
that border, and the inherited value would outline the bar where WinUI 3 and
libadwaita draw none (specification §5.8). Causes
2 and 5, and Cause 4's other items, leave no leaf UNMAPPABLE:
they are what a SCOPED or DERIVED row costs.

### 4.1 The six causes

**Cause 1 — no widget-type axis (ledger item 1).** The root cause, proved in
§1.2–§1.3. It does not make leaves UNMAPPABLE by itself — it makes them SCOPED,
which is why so many leaves are in that bucket rather than lost. What it makes
genuinely unmappable are the **intra-widget** contests, where both claimants live
in the same widget and no scoping mechanism can help (ledger item 20): one of
them, the slider's rail and handle, resolved by naming a winner
(specification §5.11). The focused text field's border and its selected text
were a second, and now separate per call: `input_frame` hands `TextEdit` a
frame whose focused stroke takes `input.focus_border_color` (specification
§4.7), so the border claims no shared field. The third a scope could not
separate, the expander's arrow and label, now separates per call (Cause 4), and so does a fourth, the
list's outline and grid lines on one `noninteractive.bg_stroke`: the outline
colour is set on the `Frame` the application lays round the list
(specification §5.4).

**Cause 2 — `Area` containers ignore the calling `Ui` (ledger items 2, 3).**
`Area::Prepared::content_ui` builds with a bare `UiBuilder::new()`
(`containers/area.rs:611-629`). The specific casualty is the *idiomatic* tooltip:
`Tooltip::for_widget` sizes itself from `response.ctx.global_style()` at
`containers/tooltip.rs:43` and accepts neither a `Frame` nor a `StyleModifier`,
so `Response::on_hover_text` — the call everyone writes — is unthemable. There is
an escape hatch today, and it is documented rather than hidden: `Tooltip::popup`
is a **public field** (`tooltip.rs:9`), so
`let mut t = Tooltip::for_enabled(&r); t.popup = t.popup.frame(f).style(m);` followed by
`t.show(..)` works on the manual path — the same field reassignment egui itself
makes inside `Tooltip::for_enabled` (`tooltip.rs:53-58`), and `for_enabled`
rather than `for_widget`, because `for_widget` is "always open" for as long as
it is called (`:38-39`) while `for_enabled` opens on hover over an enabled
widget, as `Response::on_hover_ui` does (`response.rs:665-666`); chaining `.popup.frame(f)`
straight off the constructor yields a bare `Popup` and skips `Tooltip::show`
(`tooltip.rs:101`).

What a scope *around* the call cannot do, a scope **inside the closure** can,
and both documents must say so or ledger item 2 reads as a total loss when it is
partial. `Window::show`, `Modal::show` and `Popup::show` all hand the
application a `Ui` that descends from the `Area`'s content `Ui`
(`containers/window.rs:710`, used at `:719-752`; `containers/modal.rs:104-108`;
`containers/popup.rs:508`, body at `:600-603`),
and `Ui::set_style` (`ui.rs:387`, doc at `:384`: "Changes apply to this `Ui` and
its subsequent children") replaces the style for that `Ui` and everything built
from it (`ui.rs:237`). A role scope applied as the *first statement inside the
closure* therefore reaches every widget the application adds there. It does
**not** reach the chrome computed before the closure runs, nor the parts the
container paints itself; specification §14 item 2b records that residue. The
consequence is a cost reduction, not a reach increase: opt-in fidelity inside a
`Window` costs one line of application code rather than an upstream change — and
one line of application code is still application cooperation, which is exactly
what reading (2) of §1.6 says is unavoidable.

**Cause 3 — the field simply does not exist.** No amount of cleverness invents a
slot. `Visuals` models exactly two status colours, `warn_fg_color`
(`style.rs:1056`) and `error_fg_color` (`:1059`); nothing in `Visuals`' 36
fields (`:989-1126`) or the five `Widgets` entries (`:1255-1269`) means "the
window lost focus", and `widgets.open` means the *active* window title bar
(`containers/window.rs:1427`) — the opposite; and there is one
`Visuals::hyperlink_color` (`style.rs:1036`) and no visited state. No row
waits on these three missing fields: success, info and the four status text
colours reach a stock `Button` or `Label` per call (item 15); the inactive
title text reaches the title per call (specification §5.2), and the unfocused
selection fill waits on a native-theme leaf, not on egui (item 16, §4.3); and
the hover, pressed and visited link colours reach `Link` per call, the
disabled one through its role scope (item 14, specification §5.3). No focus outline exists anywhere in `Visuals`, so the
connector paints one itself (§3.7); item 5 keeps only the pressed look egui gives
a focused widget. No switch
or toggle module exists under `egui/src/widgets/` at all (item 13). Nor does egui record the
unit a size was stated in, so the 21 `*.defined_size` leaves have no sink — and
lose nothing visible, because each size reaches egui through the `.size` leaf
beside it (specification §5.7).

**Cause 4 — the value is hardcoded in paint code.** `separator_style` hardcodes
`spacing: 6.0` (`widget_style.rs:215`), overridable only per instance via
`Separator::spacing` (`widgets/separator.rs:45`) — item 4. Five `Frame` presets
hardcode their inner margins (`containers/frame.rs:180`, `:187`, `:192`, `:229`,
and `:236-237`, where `dark_canvas` delegates to `canvas`) — item 18. `Spinner`'s radius inset, point count and `Stroke::new(3.0, ..)` are
hardcoded (`widgets/spinner.rs:45-46`, `:58`) — item 12. The slider handle's
radius is derived from the allocated rect (`widgets/slider.rs:880-886`) — item
11 — and the tessellator strokes a circle's outline *outside* it
(`epaint/src/tessellator.rs:1531`), so the Slider scope sets `expansion` —
`−w`, `w` the outline width, wherever the Body row does not raise the slider's
thickness — so that the outline's outer edge lies at `d / 2`, and the painted knob's outer
diameter is exactly `slider.thumb_diameter` (specification §6.6). The price is
the slider's value box, drawn in the same scope, whose button frame the same
negative expansion insets by `−expansion` — one point at egui's stroke width
where the floor does not bite (`egui/src/widget_style.rs:162-165`) — a stated cost on a secondary
element, for the knob's true size on the primary one.

`expander.arrow_color` shares a field with the label: `paint_default_icon` fills
the arrow with `visuals.fg_stroke.color` (`containers/collapsing_header.rs:353`)
and the label uses `visuals.text_color()` (`:598`), which *is* the same field —
item 25, no longer a loss. The difference is not hypothetical — `native-theme/src/presets/macos-sonoma.toml:336`
gives the arrow `#86868b` against `#1d1d1f` text, and
`native-theme/src/presets/windows-11.toml:371` uses a semi-transparent
`#1a1a1ae0` — and it is not lost: `expander_icon(t)` returns a closure for the
per-call `CollapsingHeader::icon` (`containers/collapsing_header.rs:480`) that
draws egui's own `paint_default_icon` (`containers/collapsing_header.rs:336`) in
the arrow colour, so only a header built
without it shows the label colour on its arrow. The header's resting fill is
the opposite case: `ResolvedExpanderTheme` states none
(`native-theme/src/model/widgets/mod.rs:828-849`) and egui by default paints
none, so the Expander scope paints none either — with `collapsing_header_frame`
set, egui would fill the resting header with the state's `weak_bg_fill`
(`collapsing_header.rs:561-568`), and a button colour there would be a fill no
platform stated.

**Cause 5 — the type is too narrow (item 19).** `Margin` is four `i8`
(`epaint/src/margin.rs:15-20`), `CornerRadius` four `u8`
(`corner_radius.rs:13-25`), `Shadow::offset` `[i8; 2]` with `blur` and `spread`
`u8` (`shadow.rs:15`, `:20`, `:23`). Sub-point precision and large magnitudes are
lost at the boundary. This is the one cause where the loss is *quantified* rather
than total, and where the connector reports it (`Note::ValueSaturated`).

**Cause 6 — `FontId` has no weight or slant, and egui has no font database
(items 7, 8).** §3.11. Twenty-one family slots, one selector; the connector
supplies the system's face for the two it installs, and a per-call weight and
slant for the rest. The nineteen per-widget families render in the default
one, which is right wherever they equal it — on every bundled preset — and
UNMAPPABLE where a live reader states another (specification §5.8 item 7).

### 4.2 The upstream contribution to egui

Framed per the house guidance for upstream PRs recorded in `docs/todo.md`,
section *Upstream PRs to gpui-component*: **more theming flexibility, not "native platform look"**;
**no API breaking changes**; **one concern per PR**; **concrete benefit shown**;
follow the project's own contribution guide, including disclosure of
AI-generated code.

`egui/src/widget_style.rs` is **byte-identical** between 0.35.0 and 0.36.1,
verified by `diff`, and 0.36.2 changes only one path in it (`std::fmt` →
`core::fmt`, `widget_style.rs:259`, §2 Option D). A frozen module is the best
possible target for a well-argued PR, because nothing in flight competes with it.

* **Commit 1 — widgets declare their kind.** Add `BUTTON_CLASS`,
  `CHECKBOX_CLASS`, `SEPARATOR_CLASS` and `CHECKED_CLASS` beside
  the existing `ROOT_CLASS` (`widget_style.rs:222`) and `SELECTED_CLASS` (`:225`),
  and have `Button`, `Checkbox` and `Separator` add their own class where
  `Button` already adds `SELECTED_CLASS` (`widgets/button.rs:329`) — the three
  widgets that already ask for a class-aware style (`widgets/button.rs:331`,
  `widgets/checkbox.rs:83`, `widgets/separator.rs:109`).
  **Behaviour-neutral**, no signature changes, one concern.
* **Commit 2 — `Style` gains class overrides.**
  `pub class_overrides: Arc<[(ClassName, StyleModifier)]>` (`#[serde(skip)]`,
  default empty), applied to a local copy of `self` for the classes present at
  the top of each class-aware method — `widget_style` (`widget_style.rs:120`)
  and also `button_style` (`:146`) and `checkbox_style` (`:174`), because those
  two read `self.visuals` and `self.spacing` directly as well as through
  `widget_style` (`:147`, `:175`, `:179-180`), so an override applied in
  `widget_style` alone would reach their text and miss their frames.
  **Behaviour-neutral when empty**; the `Arc<[_]>` keeps `Style: Clone` cheap and
  the whole thing `Send + Sync`. One concern, additive field only.
* **Commit 3 — plumb classes through the rest.** `separator_style` stops
  ignoring its classes and hardcoding `spacing: 6.0` (`:212`, `:215`);
  `TextEdit`, `ComboBox`, `Slider` and `CollapsingHeader` gain a `Classes`
  field and route through `widget_style` instead of `Style::interact`
  (`widgets/text_edit/builder.rs:737`, `containers/combo_box.rs:373`, `:452`,
  `widgets/slider.rs:766`, `containers/collapsing_header.rs:337`); `Label`
  routes through `label_style` (`widget_style.rs:194`, which no widget calls in
  0.36.2) with a new `LABEL_CLASS`, instead of `Style::interact`
  (`widgets/label.rs:297`); and `ProgressBar` gains one in place of its direct
  `Visuals` reads (`widgets/progress_bar.rs:135`).

Commits 1 and 2 are small — four `const`s, three `add_class` calls, one field
applied at the top of three methods — and individually behaviour-preserving —
the shape of PR that lands. The concrete benefit is easy to demonstrate: with
them, SCOPED collapses into DIRECT for `Button` and `Checkbox`; with the third,
for `TextEdit`, `ComboBox`, `Slider`, `CollapsingHeader`, `ProgressBar`,
`Separator` and `Label` too, this crate ships its atlas as `class_overrides` on
a single `Style`, `native_scope` becomes optional sugar, and **SCOPED collapses
into DIRECT for every widget egui itself paints**. That is the change that would
make "full theme geometry" a true claim — for widgets. The `Area`-based
containers (§1.4) build their content `Ui` with no class at all
(`containers/area.rs:611-629`) and their `Frame` presets read
`visuals.window_fill` directly (`containers/frame.rs:197-219`), so for surface
leaves the claim also needs those containers to add a class of their own
through the `classes` field `UiBuilder` already carries (`ui_builder.rs:31`).

Smaller, independently landable PRs in the same spirit, one concern each:

| PR | Closes | Shape |
|---|---|---|
| `Area::style` / a `style` on the `UiBuilder` in `Area::Prepared::content_ui` | items 2, 2b | additive builder method |
| `Tooltip::style` / `Tooltip::frame`, or `Response::on_hover_text_styled` | item 3 | additive builder method |
| `Visuals::focus_stroke` + `focus_offset`, painted by `AtomLayout` | item 5 for every widget without `register_focus_shape`: each widget would paint its own ring in its own shape, where the connector's plugin (§3.7) draws a rounded rectangle unless a shape is registered | additive `Visuals` fields, default = today's behaviour |
| a sixth `Widgets` entry, or `WidgetState::Disabled` | item 6 | additive in the sense below; the `WidgetState` form also breaks every exhaustive `match` on it |
| `weight` (and `slant`) on `FontId` | item 7 | the largest of these; upstream already carries the `TODO` (`epaint/src/text/fonts.rs:27`) |
| apply `extra_text_line_spacing` in `RichText::into_layout_job`, or add a multiplicative `line_height_factor` | item 9 | one concern, visible benefit |
| `Spacing::slider_handle_radius`; `Spinner::stroke_width` | items 11, 12 | additive |
| `Visuals::hyperlink_visited_color` + a visited set in `Memory` | item 14 | two concerns; split |
| a `Visuals::status` block | item 15 | additive |
| `Visuals::selection_inactive`, consulted when `ctx.input(\|i\| !i.focused)` | item 16 | additive |
| the five `Frame` presets reading `Spacing` | item 18 | behaviour-changing by definition; frame as "let themes reach panel margins", offer the fields as `Option` |
| `MarginF32` / `CornerRadiusF32` in `Style` | item 19 | large; lowest priority |
| wire or delete `menu_width`, `menu_spacing`, `compact_menu_style`, `window_highlight_topmost` | item 17 | trivially reviewable, good first contribution |
| per-widget style structs for the widgets that hold an intra-widget contest (`Slider`; `TextEdit`'s and `CollapsingHeader`'s are resolved per call, by `input_frame` and `expander_icon`), like `Style::button_style` / `checkbox_style` (`widget_style.rs:146`, `:174`) | item 20 | additive, one widget per PR |
| `Style::button_style` resolving `TextStyle::Button` instead of `widget_style`'s `Body` fallback | item 26 | small; same frozen module as commit 3 |
| a colour slot of its own for `paint_default_icon` | item 25 without the per-call `expander_icon` step (§4.1, Cause 4) | additive |
| a hover fill of its own for frameless readers | item 31 | additive |
| a `Margin`-typed `button_padding`, or a per-widget margin builder | item 32 | changes a field's type |
| an `egui::Switch` widget | item 13 | a new widget |
| a font-discovery layer in eframe, or an epaint hook | item 8 for every egui application, not only this connector's (§3.11) | the largest in scope |

**What "additive" means above.** No type in egui 0.36.2 or epaint 0.36.2 is
`#[non_exhaustive]` — not `Style`, `Spacing`, `Visuals`, `Widgets`, `FontId`
nor `WidgetState` — so every row that adds a field or a variant, commit 2
included, breaks a downstream struct literal or exhaustive pattern: the very
property §5.1's T1 tripwire relies on. "Additive" here means no existing
signature and no default behaviour changes; such a change lands only in an egui
minor, a breaking release for a 0.x crate anyway, which is how
`Spacing::extra_text_line_spacing` arrived between 0.35.0 and 0.36.1 (§5.1).

**Two honesty notes, both non-negotiable and both repeated from the
specification.** No upstream maintainer has been consulted and egui's appetite
for any of this is **UNVERIFIED**; *what would verify it* is an issue or
discussion on the egui repository, which is outside what this design work can
establish. And **the connector must not depend on any of it**: the design works
against egui 0.36.2 exactly as shipped, and every upstream change above would be
a simplification, never a prerequisite.

### 4.3 The changes owed to native-theme itself

The first version of this design listed four native-theme-side items: two
groups of structural zeros, `LayoutTheme` on `SystemTheme`, and shadow
geometry. native-theme has made the first three; shadow geometry is still owed.
v0.6.0 makes three more, each because the connector needs it and each belonging
where the platform's facts are read: the platform's typeface found by name, the
icon theme of both variants, and a GNOME watcher that stops when dropped.

* **The structural zeros are gone — formerly 38 `source-void`
  leaves.** A *source-void* leaf carries no information, so no egui PR can close
  it. Every widget border used to carry a `corner_radius_lg` and an `opacity`
  that the resolver hardwired to `0.0` (36 leaves, 18 widgets × 2; lines 276 and
  278, 330 and 332, and 50 and 52 of `validate_helpers.rs` at `423d396e`), and
  `defaults.border` carried `padding_horizontal` / `padding_vertical` hardcoded
  to `0.0` (2 leaves, lines 584–585 there; item 23). A connector that read
  `theme.window.border.corner_radius_lg` would get square window corners; one
  that folded `theme.button.border.opacity` into a stroke alpha would make
  **every** border in the theme invisible; one that read the defaults padding
  into `Spacing::button_padding` would inject a fabricated zero and flatten every
  `Button`, `ComboBox`, `CollapsingHeader` and `DragValue` at once. v0.5.9 took
  all 38 out of the resolved model — the fix this section asked for:
  `ResolvedWidgetBorder` has no `corner_radius_lg` and no `opacity`
  (`native-theme/src/model/border.rs:233-246`), and `ResolvedDefaultsBorder` has
  no padding (`border.rs:212-225`; `DefaultsBorderSpec` has none by design,
  `:12-18`). What the connector folds into a stroke alpha is therefore
  `defaults.border.opacity`, the one border opacity the model carries
  (`border.rs:222`), and there is no widget-level field left to misread.
* **`LayoutTheme` on `SystemTheme` (item 21) — added in v0.5.8.**
  `SystemTheme.layout` (`native-theme/src/lib.rs:488`) carries the reader's
  layout values merged field-wise over the preset's, so `from_system` supplies
  `Spacing::item_spacing`, from `layout.widget_gap`, and the central panel's
  margin, from `layout.window_margin` through `Surface::CentralPanel`, exactly
  as `from_preset` does. `Spacing::window_margin` never depended on it: it is
  written from `window.border.padding`, which `ResolvedTheme` carries on every
  path. The reasoning that recommended the change stays in §8, Q-2.
* **Shadow geometry (item 10) — still owed.** native-theme carries only
  `shadow_enabled: bool` (`native-theme/src/model/border.rs:224`, `:241`) while
  `epaint::Shadow` needs an offset, a blur and a spread. The connector replaces
  **only the colour** and keeps egui's own geometry; populating
  offset/blur/spread would be invention. Adding them to the border specs is the
  fix. Fourteen leaves are tagged for exactly this — the
  `.border.shadow_enabled` of `sidebar`, `toolbar`, `status_bar`, `card`, `list`,
  `input`, `button`, `menu` (its items), `checkbox`, `segmented_control`, `combo_box`,
  `progress_bar`, `tab` and `expander`, the UNMAPPABLE sub-tag
  **`source-side gap`** of specification §2 — because each has an
  `egui::Frame` that could carry a shadow, its own or one the application lays
  round the widget (`frame.rs:140`, `:303`), and no shadow of egui's own whose
  geometry the connector could keep, so neither side alone is at fault. A
  fifteenth leaf carries the sub-tag for a different missing
  companion: `defaults.selection_inactive_background`, the selection fill of a
  window that has lost focus. The install plugin could swap it in while
  `InputState::focused` is `false` (`egui/src/input_state/mod.rs:312-315`), but
  `ResolvedTheme` states no text colour to pair with it, so the active
  `selection_text_color` would be painted on the unemphasised fill. Adding that
  leaf is native-theme's (`docs/todo.md`); the swap follows it.
* **The system typeface (items 8, 7) — made in v0.6.0.** A theme names its
  typeface, and a toolkit that takes bytes cannot use a name. native-theme gains
  the pure matching function `native_theme::fonts::select_face` — the named
  family only, compared by Unicode default caseless matching through a new
  `unicase` dependency, then the CSS font-matching steps for width, style and weight —
  exported with no feature, and the `system-fonts` feature with
  `native_theme::fonts::system_face`, which runs it over fontdb's faces and, on
  macOS, finds the system UI font by its file through Core Text (§3.11). So
  `defaults.font` and `defaults.mono_font` reach egui as the platform's own
  faces. It belongs in native-theme rather than here because the question —
  which face on this system is the one the platform named — is the platform's,
  answered once beside the readers that produce the names.
* **The icon theme of both variants — made in v0.6.0.** `SystemTheme::icon_theme`
  names the active variant's only (`native-theme/src/lib.rs:470-483`), while
  egui can draw either scheme's style in any pass, so a dark style would get the
  light scheme's icons (§3.13). `SystemTheme::icon_theme_for(mode)` gives both,
  resolved by the same three tiers (`native-theme/src/pipeline.rs:82-93`) and
  carried through `with_overlay`.
* **A GNOME watcher that stops when dropped — fixed in v0.6.0.** Dropping a
  GNOME or Budgie subscription waited for the next portal signal (§3.14). The
  GNOME backend now registers the platform shutdown the macOS and Windows
  backends already have; every connector's watcher sits on that thread, so the
  fix is native-theme's, not a workaround here.

The unstated-sizes change (§3.23) belongs to the same class of fix: a size the
platform does not state is `None` instead of an invented `0.0`, so the model
no longer asks a connector to write a number nobody stated.

### 4.4 Two losses argued in full, the refusal, and what lies outside

Two rows of the ledger need more than their cause; the second
is, since this revision, not a loss but a decision.

**Item 26 — `egui::Button` does not honour `TextStyle::Button`: egui
hardcodes `Body`.** `Button::new` sets `.fallback_font(TextStyle::Button)`
(`widgets/button.rs:49`) and `atom_ui` then overwrites that field at `:358-360`
with `Style::button_style`'s font, which is
`override_font_id.unwrap_or_else(|| TextStyle::Body.resolve(self))`
(`widget_style.rs:122`, `:137`, `:148`, `:168`) — a hardcoded
`TextStyle::Body` whenever `override_font_id` is `None`, which is egui's default
(`style.rs:1430`). `AtomLayout::fallback_font` is a plain setter
(`atomics/atom_layout.rs:147-150`), so the second call wins and every `Button`
renders in `text_styles[Body]`. The overwrite is not what makes
`override_font_id` win: `FontSelection::resolve_with_fallback` consults it ahead
of any fallback anyway (`style.rs:158-168`, called at `widget_text.rs:774`).
The fix is upstream and small — `Style::button_style` resolving
`TextStyle::Button` instead of inheriting `widget_style`'s `Body` fallback
(specification §14 item 26), the same shape of one-line fix as item 4's
`separator_style`, in the module §4.2 already targets. Until then there is
**nothing the connector can fix in `text_styles`** — it is recorded because the
assumption is natural and wrong, and because it decides where per-widget
typography must travel: on `Style::override_font_id` (`style.rs:255`), not on
`text_styles[Button]`. That field outranks every text style a call site asks
for, `RichText::heading` included (`widget_text.rs:426-430`), so it is written
only in the `Button`, `Link`, `Checkbox` and `SegmentedControl` scopes, whose
widgets carry one label each; in a container's scope it would flatten every
heading inside it. The `text_styles[Button]` write is
nevertheless kept, because `ComboBox` (`containers/combo_box.rs:358`),
`ProgressBar` (`widgets/progress_bar.rs:193`), `CollapsingHeader`
(`containers/collapsing_header.rs:522`) and `Style::drag_value_text_style`
(`style.rs:1434`) do read it.

**Item 27 — the accessibility preferences are applied where egui has a sink,
and only there** (decided by the goal: a native application follows the OS
text-scaling factor and reduced motion, so an application on this connector
that ignored them would not look or behave natively; §8, Q-7).
The first version
of this document refused `accessibility.text_scaling_factor` on the grounds that
egui already has a global scale, `Context::set_zoom_factor`
(`egui/src/context.rs:2337`), and that scaling only the text would desync it from
the geometry. Both halves were wrong for this preference:

* `set_zoom_factor` is not a text scale. It multiplies `pixels_per_point`
  (`context.rs:455`; its doc, `:2328-2332`, says "Make larger to make everything
  larger"), so it enlarges every length — margins, rounding, strokes, icons —
  where the platform preference enlarges text.
* Nothing else will apply it. A search of the egui and eframe 0.36.2 sources
  finds no reader of an OS text-scaling or reduced-motion preference, so a
  preference the connector drops is dropped for good. v0.5.8 recorded exactly
  that, in the gpui connector, as a bug and fixed it (`CHANGELOG.md:119`).
* Scaling text sizes and nothing else is what both siblings do, through the
  same `scaled_text_size(size, &AccessibilityPreferences)`
  (`connectors/native-theme-iced/src/lib.rs:480`,
  `connectors/native-theme-gpui/src/lib.rs:444`), whose docs say the factor
  applies to every text size (`connectors/native-theme-iced/src/lib.rs:467`,
  `connectors/native-theme-gpui/src/lib.rs:441`) — gpui's adds "applies to
  every text size alike".

So the atlas scales every text size it writes through `scaled_text_size`, and
under `reduce_motion` gives every style `animation_time = 0.0`
(`egui/src/style.rs:318`) and `ScrollAnimation::none()` (`:338`, `:858`). That
last write is one more reason for the 0.36.2 floor (§3.17): egui 0.36.2 returns
a zero-time value animation's target at once (`egui/src/animation_manager.rs:88-93`),
where 0.36.1 still returned a value interpolated from the old endpoints on the
frame of the change and reset them only afterwards. `reduce_transparency` and
`high_contrast` stay exposed, not applied: egui's one translucent surface that
the preference is about, the `Modal` backdrop, is a per-call
`Modal::backdrop_color` (`egui/src/containers/modal.rs:62`, default
`Color32::from_black_alpha(100)` at `:29`), not a `Style` field, so that choice
is made at the application's call site. `set_zoom_factor` remains the
application's own decision.

And one is a refusal, recorded in the ledger so that it reads as a decision
rather than an oversight: the shipped widget (§3.22, ledger item 28) — a refusal
for this crate only: the switch egui lacks is the companion
`native-theme-egui-widgets` crate's (`docs/todo_egui-widgets-spec.md` §0.1),
whose milestone is undecided (`docs/todo_egui-widgets-spec.md:3-4`). The focus ring, the other refusal of the
earlier revision, is now painted (§3.7).

Two platform behaviours lie outside every mapping, and are named so that their
absence reads as known. **Keyboard mnemonics** — the underlined access key of a
menu item or a label: egui 0.36.2 has none (a grep of its sources for
`mnemonic` finds nothing), and native-theme models none, so there is nothing to
carry and nothing this crate could draw. **Right-to-left text**: egui's
`Layout::right_to_left` (`egui/src/layout.rs:156`) orders widgets, not
characters, and epaint lays text out with no bidirectional reordering — its own
comment waits for the day "RTL/bidi support is added"
(`epaint/src/text/text_layout.rs:1366`) — so a right-to-left locale gets egui's
left-to-right text whatever the theme says.

---

## 5 -- The long-term evolution argument

The design's central claim is that it absorbs churn from **two independent
sides** without either becoming a crisis. Here is the walk-through for each.

### 5.1 When egui bumps a minor

**What breaks.** A renamed or removed `Visuals` / `Spacing` / `WidgetVisuals`
field breaks the mapping code — and it breaks it at **compile time, naming the
field**, because specification §13 T1 destructures those structs exhaustively
with **no `..` rest pattern**. A new field is likewise a compile error naming the
new field, which is exactly the moment to decide whether a native leaf belongs in
it. This is strictly better than a `size_of` assertion, which tells you only that
something changed.

`Style` itself is included, although its `debug` field is
`#[cfg(debug_assertions)]` (`style.rs:323-324`) and the field count differs
between the profiles: the pattern binds it as `#[cfg(debug_assertions)] debug: _`,
and a `cfg` on a field pattern is honoured, so one pattern compiles in both
profiles (measured, specification §13 T1). `Visuals` is included with `#[expect(deprecated)]` for `clip_rect_margin` (`:1086`), which
would otherwise warn.

**What does *not* break, and is the residual risk.** Upstream drift comes in
three modes and the tripwire catches only the first two. (1) A field **added,
removed or renamed** is a compile error naming the field — that is T1's own
claim (specification §13) and it is exact. (2) A field whose **type** changes is
caught only at the sites that assign to it, and only where the value assigned has a fixed type; a site that assigns a
literal (`animation_time = 0.0`) or goes through `.into()` compiles against any
type that accepts it.
(3) A field whose **meaning** changes with no change to its name or type is
caught by **nothing**. This is not hypothetical: if
`Spacing::extra_text_line_spacing` (`style.rs:424`, default `0.0` at `:1465`)
were redefined from an additive delta in points to a multiplier, specification
§6.15's derivation would be wrong by a factor of the font size with every test
still green. The mitigation is a value tripwire beside the shape tripwire —
assert egui's own published defaults, `extra_text_line_spacing == 0.0`
(`:1465`), `interact_size.x == 40.0` (`:1460`),
`Visuals::dark().disabled_alpha == 0.5` (`:1560`) — and reading the **doc-comment diff**, not only the field
list, at every egui bump. The first is specification §13 T1b; both belong to the version-policy procedure of
specification §12.3.

**How much is a semver break for the connector's users.** Clause 2: an egui minor
bump is breaking for this crate, and users move deliberately. Clause 3 means the
*mapping* corrections that accompany the bump are not additionally breaking.

**How much is mechanical.** Almost all of it. The public API names egui types
only as parameter and return types — `Context`, `Ui`, `Theme`, `Style`, `Frame`,
`style::StyleModifier`, `Color32`, `FontId`, `Vec2`, and the epaint value types
in `convert` — and **no public item of this crate is a re-export or a rename of
an egui type**. `Role` is named from `ResolvedTheme`; `Surface` names the
**attachment points** at which an application can hand egui a `Frame`, which is
not the same thing as naming egui's types. Four of the eight have no same-named
egui type: `WindowTitleBar` is `Window::title_frame` (`containers/window.rs:272`),
and `Dialog`, `Popover`, `Card` have no egui item of that name at all; the types behind them
are `Modal` (`containers/modal.rs:16`), `Popup` (`containers/popup.rs:165`) and
`Frame` (`containers/frame.rs:96`), and a recursive grep for
`Dialog|Popover|Card` over `egui/src` returns exactly one hit,
`viewport.rs:998`, an unrelated window-type hint. What survives an upstream
rename is therefore the *shape* of the axis rather than any spelling: `Surface`
tracks where a `Frame` can be attached, so an upstream container rename is a
cosmetic follow-up rather than a compile break. A renamed egui type is a
find-and-replace;
a renamed `Style` field is a compile error with the field's name in it.

**Evidence for the churn rate.** Between 0.35.0 and 0.36.1, `egui/src/style.rs`
changed by **23 lines** — the `<` and `>` lines of `diff`, which equal the `+`
and `-` lines of `diff -u` less its two file headers. The semantic delta is
three items: `Spacing::extra_text_line_spacing` added (`style.rs:424`, default
`0.0` at `:1465`), `Visuals::clip_rect_margin` deprecated (`:1086-1087`), and
`warn_if_rect_changes_id`'s default flipped to `false` (`:1404`). Across 0.36.0
and 0.36.1: one new field, one deprecation, one default flipped. The 0.36.2
patch then changed the same file by 21 lines under the same count (44 from
0.35.0), and every one of them is path churn — `std::fmt` and `std::ops`
becoming `core::fmt` and `core::ops`, and one import regrouped — with no
semantic change to `Style` at all. The new field, notably, is
one this connector *wants* — it is the line-height sink of §3.12 — which is a
small piece of evidence that egui's style surface is drifting toward, not away
from, what a theme connector needs.

### 5.2 When native-theme grows a widget

Say native-theme adds two widget structs and six fields. **No egui bump is
involved at all** — this is the axis the version pin buys nothing on, which is
why clause 4 of the semver contract exists.

**What changes here, in order:**

1. `Role` grows two variants. `Role` is `#[non_exhaustive]`, so no downstream
   `match` breaks; `Role::all()` exists precisely because callers cannot write
   their own exhaustive list.
2. `mapping.toml` grows two sections and six rows. **T3 (converse coverage) fails
   until they are added** — it walks every leaf of
   `serde_json::to_value(&resolved)` (`ResolvedTheme` derives `Serialize`,
   `resolved.rs:155`) and asserts each has exactly one manifest row. A forgotten
   field is a red test, not a silent hole.
3. The role-style builder grows two arms. T4 (hostile input) and T10 (the
   mapping contract, which also compares every built style with itself) cover
   them for free, because both iterate every role cell.
4. T2 (differential coverage) fails until each new row's declared sinks are
   proved by a preset splice, or its `tested_by` names the test of its route.

**What does *not* change:** `Surface`, `RoleVariant`, `PanelSide`, every
extension trait, every conversion helper, the install sequence, the semver
version. Nothing in the public API surface moves except one `#[non_exhaustive]`
enum growing variants, which is additive by construction — and, for a new leaf
no `Style` field carries, at most one new free accessor, where it adds meaning
(§3.1), additive by specification §12.2 clause 5.

This is the property that the rejected `StyleSet`-with-26-public-fields shape did
not have: there, the same change breaks every downstream struct literal and
changes the type of a public `ALL: [Self; 25]` constant.

### 5.3 When `widget_style` matures

Three scenarios, and none of them is a crisis.

**(a) egui adds `class_overrides` as proposed in §4.2.** The connector's atlas is
*already* a table of `Style` values keyed by widget kind — which is exactly the
shape `class_overrides` wants. The change is internal: build the same values, hand
them to one `Style` as class overrides instead of to N `Arc<Style>`s.
`native_scope` keeps working (it becomes optional sugar), `ThemeAtlas` is opaque
so nothing in its shape leaks, and clause 3 means the resulting change in what an
unscoped widget looks like is not a breaking change. The SCOPED leaves of every
widget that paints through `widget_style` become DIRECT and the README's headline
sentence is rewritten upward; the container-carried ones follow only with the
container classes §4.2 names.

**(b) egui adds something similar but differently shaped.** The blast radius is
the same: the atlas is a *table*, and a table can be delivered through whatever
seam exists. The public enums do not encode the seam. This is why `Role` is
defined as "one variant per `ResolvedTheme` widget field" rather than as "one
variant per egui style hook" — the first definition is stable against upstream
redesign, the second is not.

**(c) egui does nothing, for several more releases.** The most likely outcome, on
the evidence: across a minor and two patch releases (0.36.0–0.36.2) the module changed one path
and no behaviour. The connector
keeps working exactly as specified, because it never depended on the module. This
is the entire reason for the non-negotiable rider in §4.2.

### 5.4 The shape of the bet

Summarising the three walk-throughs: the design bets that **egui's `Style` struct
is stable-ish and its `widget_style` module is frozen**, and it structures itself
so that being wrong about either is survivable. If `Style` churns **in shape**,
compile errors name the fields — with §5.1's caveat attached: a change of
*meaning* with no change of name or type names nothing, and only the value
assertions and the doc-comment diff catch it. If `widget_style` unfreezes, the
atlas is already the right data
structure. If native-theme churns, one `#[non_exhaustive]` enum grows and two
tests go red. There is no scenario in the three that requires an architectural
change.

---

## 6 -- What was deliberately not done, and the trigger to revisit

Each item names the trigger that would reopen it. Absent that trigger, the
question is closed.

| Not done | Trigger to revisit |
|---|---|
| Ship any `egui::Widget` | Both charter tests pass *and* the widget's behaviour surface stops moving upstream — i.e. egui gains a stable composition primitive that makes a widget a data declaration rather than a paint routine. Passing the two tests alone is **not** sufficient; `Switch`, the one candidate, fails the second since its substitute reaches the unchecked track (§3.22) |
| Depend on `egui::widget_style::Classes` | egui merges something like §4.2 commit 2 **and** it ships in a released minor. Not a nightly branch, not an accepted issue |
| Emit `FontFamily::Name` | `FontId` gains a weight or slant selector (§4.2), which removes the reason to need extra families in the first place. Emitting `Name` before that would trade two release-mode panics for installing per-widget weights that `role_font_weight` already carries per call (§3.11) |
| Apply `reduce_transparency` or `high_contrast` in the atlas | egui gains a `Style` field for `reduce_transparency`, a `Style` field for the `Modal` backdrop, the one translucent surface it concerns, today a per-call `Modal::backdrop_color` (`egui/src/containers/modal.rs:62`); for `high_contrast`, the theme model carrying a value a platform states for it (the gpui connector also builds nothing differently for it). Text scaling and reduced motion *are* applied (§4.4, item 27); `Context::set_zoom_factor` stays the application's |
| Write `Spacing::default_area_size` | egui gains a per-container default size, or `Modal` gains a size constraint that reads `Style` |
| Write the focus ring into `widgets.active.bg_stroke` | egui separates focus from press in `Response::widget_state` (`widget_style.rs:107-109`), or adds `Visuals::focus_stroke`. Until then the install plugin paints the ring (§3.7), with the corners of the widget's role scope or in the shape a widget registers; either upstream change would let each widget paint it in its own shape unasked |
| Offer `RoleVariant::Selected` and `Disabled` together | `Style::button_style` stops overwriting the selected fill regardless of state (`widget_style.rs:147`, `:150-155`), or egui gains a disabled `WidgetVisuals` |
| Depend on `egui_extras`, or default `svg-rasterize` on | `egui_extras` and `native-theme` converge on one `resvg` major (for `svg-rasterize`). Today they are on 0.45.1 (egui_extras 0.36.2) and 0.48.1 (§3.13, §3.16) |
| Validate font faces through epaint instead of a direct `skrifa` dependency | epaint re-exports `skrifa`, or exposes a fallible face constructor, removing the need to pin a second name to epaint's requirement (§3.11) |
| Enable `egui_kittest`'s `snapshot` or `wgpu` feature | A defect that the showcase's click-and-query tests and the headless specification §13 groups provably cannot express — the trigger theme-contracts set for screenshot diffing (`docs/archive/todo_v0.5.9_theme-contracts-rationale.md:545`). Pixel rendering is the one part of the harness that needs a feature, and it brings in `wgpu` 30 (§8, Q-3) |
| Lower the workspace floor below `1.88.0` | Every dependency that declares a `rust-version` drops below it **and** our own sources stop using let-chains — 69 `&& let ` occurrences today, plus chains whose second term is a bare boolean (§3.18). **Both** conditions, not either. The floor is held up from both sides, so acting on the dependency half alone would break `native-theme`'s own build |
| Raise the workspace floor | A dependency bump forces it, caught by the MSRV job. Raise to the new measured maximum, never to whatever stable happens to be |
| Publish a memory-footprint number for the atlas in the crate's own documentation | Measured now for the design (§8, Q-1: 704 B / 688 B per `Style` in debug / release, the difference `Style::debug`, `#[cfg(debug_assertions)]` at `style.rs:323-324`), but the number is egui 0.36.2's and moves with every egui release, so the crate's docs promise none. Revisit when an application reports the atlas's memory as a problem |
| Install per-widget font **weight** and **slant** into the role styles | `FontId` gains the fields (§4.2). Until then `role_font_weight()` and `role_font_is_italic()` carry them per call, and an application applies the weight with `RichText::variation(..)` (`widget_text.rs:200-205`), on a variable face only (§3.11) |
| Fabricate `Spacing::icon_width_inner` on the base style | A platform reports a check-*mark* size. `docs/platform-facts.md:980` defines `indicator_width` as the **box**, so deriving a mark size from it by ratio would be an invented value. The stated inset is carried instead, in the Checkbox cell only (specification §6.11); on the base style egui's `8.0` stands (`style.rs:1467`) |

Two of these deserve a note about *why the temptation is strong*, because a
future reader will feel it.

**The `icon_width_inner` ratio.** Two of the three design proposals wanted
`icon_width_inner = indicator_width * (8.0 / 14.0)`, taking the ratio from egui's
own defaults (`style.rs:1466-1467`). It is honest arithmetic on honest inputs and
it produces a plausible number — which is exactly the problem. No platform
reports a check-mark size, so the result is a synthesised control dimension
presented as theme data. Leaving egui's value is the truthful option where
the theme states no inset; where it does, `checkbox.border.padding` is the
mark's inset in the box (Adwaita's `check, radio { padding: 3px }`,
`docs/platform-facts.md:1216`), and stated leaves, a mean and a subtraction give
the mark with no invented number (specification §6.11). The
mistake this replaced was worse in the other direction: an earlier rule wrote
`indicator_width` into `icon_width_inner` and left the checkbox **box** pinned to
egui's literal `14.0` on platforms that report 20 (`docs/platform-facts.md:1212`).

**A widget for the switch.** A switch looks like a little paint code, and the
substitute still draws no knob. The charter says no — `Switch` fails its second
test (§3.22) — and nothing is lost to an application that wants a real one:
every `switch.*` leaf is a public field of the `ResolvedTheme` the atlas hands
out, so it can build one with **nothing hardcoded**, which is the property that
actually matters under this project's rules; the companion widgets crate is
where a shipped one belongs.

---

## 7 -- Errors found and corrected during design

Recorded so that a future reader who finds one of these claims in an older note,
a proposal or a matrix knows it was checked and rejected, and does not
"re-correct" the documents back to the wrong value.

| # | Claim in an input | Correct, with evidence |
|---|---|---|
| 1 | `checkbox.indicator_width` maps to `Spacing::icon_width_inner` | It maps to `Spacing::icon_width`. `docs/platform-facts.md:980` defines the leaf as the **box** (`:1212`; `adwaita` states 20, libadwaita's 14-point content plus its 3-point padding, specification §6.11); egui reads the box as `checkbox_size: self.spacing.icon_width` (`widget_style.rs:179`) and the mark as `check_size: self.spacing.icon_width_inner` (`:180`) |
| 2 | `menu_style` overwrites three `bg_stroke`s | **Four**: `active` (`containers/menu.rs:24`), `open` (`:25`), `hovered` (`:26`), `inactive` (`:28`), plus `spacing.button_padding` (`:23`) and `widgets.inactive.weak_bg_fill` (`:27`) |
| 3 | `text_styles[Button]` ← `button.font.size` is a live DIRECT mapping | `egui::Button` never reads it (§4.4). Per-widget typography travels on `override_font_id`; `text_styles[Button]` is still written for its four other readers |
| 4 | `Ui::disable` can fire `Color32::gamma_multiply`'s `debug_assert` | It cannot. `Painter::multiply_opacity` drops non-finite and clamps (`painter.rs:100-104`). The only caller of `Visuals::disable` is `Ui::dnd_drop_zone` (`ui.rs:2726-2727`) |
| 5 | `WidgetState` has five variants, so `widgets.open` is on the interaction-state axis | Four (`widget_style.rs:84-90`). `open` is unreachable from both dispatchers and is only ever read as a direct field |
| 6 | `egui::SidePanel` / `egui::TopBottomPanel` | Do not exist in 0.36.2, 0.36.1 or 0.35.0. `Panel` (`containers/panel.rs:206`) with `left`/`right`/`top`/`bottom` (`:249`, `:256`, `:265`, `:274`), and `CentralPanel` (`:1187`). `show_inside` is `#[deprecated]` on both (`:428`, `:1218`) |
| 7 | `egui::StyleModifier` | Not re-exported at egui's root — `egui/src/lib.rs:488` re-exports only `style::{FontSelection, Spacing, Style, TextStyle, Visuals}`. The path is `egui::style::StyleModifier` |
| 8 | A naive `as u8` / `as i8` cast **wraps** | Float-to-int `as` **saturates**, `NaN → 0`, since Rust 1.45; epaint relies on it (`epaint/src/corner_radius.rs:42-44`). Integer-to-integer casts do wrap. §3.9 |
| 9 | Handing a pre-built `Arc<Style>` to `UiBuilder::style` costs "not even a refcount bump" | The `Arc` is **moved** (`ui.rs:237`), so no `Style` is copied — but the caller pays one refcount bump to produce it. Only the true half is stated |
| 10 | `egui/src/style.rs` changed by "97 diff lines" between 0.35.0 and 0.36.1 | 25 under `diff -u \| grep -c '^[+-]'`, 39 under default `diff` (the 25 include the two `---` / `+++` header lines: 23 changed lines). The semantic description was right; the number was not |
| 11 | `epaint::FontId` is declared at lines 21–24 of `fonts.rs` with the `TODO` at line 24 (quoted as made, against 0.36.1) | Lines 27–34 and 33 in 0.36.1; in 0.36.2, `epaint/src/text/fonts.rs:21-28`, `TODO` at `:27` |
| 12 | `Color32::from_rgba_unmultiplied_const` involves "no float" | In 0.36.1, the version the claim was made against, only `a == 0` and `a == 255` were float-free, and `1..=254` ran `fast_round(channel as f32 * linear_f32_from_linear_u8(a))` per channel. In 0.36.2 the claim has become true: the `1..=254` arm is integer `mul_frac_round(channel, a)` (`ecolor/src/color32.rs:147-151`, helper `ecolor/src/lib.rs:139-146`), with the same result byte for byte (§3.10), and `a == 0` / `a == 255` still short-circuit (`ecolor/src/color32.rs:142`, `:145`). What the correction protected stays true either way: the route premultiplies in gamma space and is the right one, and the forward conversion rounds at low alpha, so the round trip is inexact in **both** directions |
| 13 | `FontDefinitions::default()` installs an emoji **and CJK** fallback tail | There is no CJK face. epaint registers exactly four — Hack, NotoEmoji-Regular, Ubuntu-Light, emoji-icon-font (`epaint/src/text/fonts.rs:506-532`) — and egui's own docs say "The default `egui` fonts only support latin and cyrillic alphabets" (`egui/src/context.rs:2101`) |
| 14 | Four of egui's **seven** `Frame` presets "read no `Style` at all" | Eight presets take a `&Style` and all eight read something from it; `Frame::group` reads `widgets.noninteractive`'s corner radius and stroke (`containers/frame.rs:181-182`). The true proposition is the narrower one the specification already used elsewhere: **five** hardcode their inner margin and read no `Style::spacing` — `group` (`:180`), `side_top_panel` (`:187`), `central_panel` (`:192`), `canvas` (`:229`) and `dark_canvas` (`:236-237`), which delegates to `canvas` and so inherits its `.inner_margin(2)`. Only `window` (`:198`), `menu` (`:207`) and `popup` (`:216`) read `Style::spacing`, and 5 + 3 = 8. |
| 15 | A panel separator wider than 127 "stops reserving space" | It reserves 127. `bg_stroke.width.round() as i8` **saturates** (`containers/panel.rs:962`) and `saturating_add` then clamps at `i8::MAX` (`:964-965`) |
| 16 | `to_image_source` can return an `ImageSource::Texture` without retaining anything | `TextureHandle` frees its texture on drop (`epaint/src/texture_handle.rs:25-29`) and `SizedTexture` owns nothing (`egui/src/load.rs:456-461`; carried by `ImageSource::Texture`, `egui/src/widgets/image.rs:586`), so the handle must be stored in `ctx.data_mut()`. `Context::forget_image` cannot release it either: it touches only the four loader caches (`egui/src/context.rs:3771-3780`) |
| 17 | A write through `ctx.all_styles_mut` reaches every style in use | It reaches the two base styles only (`egui/src/context.rs:2213`); a role cell is a separate `Arc<Style>` a scope already holds. One reason the line spacing is computed at build and written into every style (§3.12) |
| 18 | Two builds of one theme can be compared with `assert_eq!` | Never equal: `Style::number_formatter` compares by `Arc::ptr_eq` (`style.rs:57-62`) and `Style::default` allocates a fresh `Arc` every call (`:1435`). A built style is compared with itself instead — the `NaN` sweep of specification §13 T4 and the contract test of T10 |
| 19 | The line-spacing value can be tested under `FontDefinitions::empty()`, like the install tests | Vacuous there: with no face in the Proportional chain there is no row height, and specification §6.15's fallback is egui's own `0.0`, so the test would pass whatever the formula said. Specification §13 T5 runs with egui's default faces and its bundled `Hack` |

---

## 8 -- Questions raised, and their decisions

Eight questions were raised during design, and all eight are settled; each is
kept here with its reasoning. **Q-2 is done** — implemented in native-theme
v0.5.8. **Q-1 is decided by measurement**, **Q-3** from the 0.36.2 source,
**Q-4, Q-7 and Q-8** by the goal — as native a look as egui can give when the
platform theme is selected — under the project's rules: never invent a value,
no panics, no hardcoded theme values. **Q-5** is decided by the rule that
nothing is built without a reason the look, safety or verifiability gives, and
**Q-6** with the showcase (§3.25). None blocks implementation. What is still
open is specification §15's short list; the measurements that closed the
other open items are at the end of this section.

**Q-1 — Atlas memory footprint. DECIDED by measurement: build eagerly, no
per-cell `OnceLock`.** Measured on 2026-09-25 with rustc and cargo 1.98.1 on
`x86_64-unknown-linux-gnu`, by a throwaway binary depending on
`egui = "=0.36.2"` (default features) under a counting global allocator, run
with `cargo run` and `cargo run --release`:

| quantity | debug | release |
|---|---|---|
| `size_of::<egui::Style>()` | 704 B | 688 B |
| heap a `Style::default()` owns | 648 B in 2 allocations | the same |
| one cell, `Arc::new(style.clone())`, heap in total | 1352 B | 1336 B |
| a default `egui::Context` after one pass, with its default fonts | 347,696 B | 346,678 B |

The 16-byte difference between the two profiles is `Style::debug`, which exists
only under `debug_assertions` (`style.rs:323-324`); a clone allocates 632 B
beyond the `Arc` and the struct, the five-entry `text_styles` map. The atlas
holds at most 25 roles × 3 variants × 2 themes = 150 role cells plus the two
base styles, so even with no structural sharing its styles take at most
152 × 1352 B = 205,504 B, about 201 KiB, in a debug build — 59 % of what egui's
own default `Context` holds after one pass — allocated once, when the atlas is
built, and shared by every clone of the `ThemeAtlas`. Sharing the `Normal` `Arc`
for a role with no `Selected` or `Disabled` data only lowers it. Per-cell
`OnceLock` would save at most the 100 `Selected` and `Disabled` cells,
135,200 B, and would pay for it by building a `Style` inside the first frame
that scopes such a cell, by a synchronisation primitive per cell, and by making
the contract test's self-comparison (specification §13 T10) force every cell
before it can compare them. The saving is smaller than egui's own baseline and
nothing is gained for it, so the tables are built eagerly. The earlier
conditional on "roughly 256 KiB" is withdrawn: it was a threshold no source
states, and the measurement makes it unnecessary. Nothing in the public API
depends on the choice, which is itself an argument for the opaque handle of
§3.1.

**Q-2 — `layout: LayoutTheme` on `SystemTheme`? CLOSED — approved by the
maintainer on 2026-08-10 and implemented in native-theme v0.5.8**
(`SystemTheme.layout`, `native-theme/src/lib.rs:488`); ledger item 21 is
retired (§4.3). The reasoning that carried it still explains the shape: a
one-field additive change to a struct that already carries `preset` and
`icon_theme`, needing no resolver work — `Theme::layout` is a plain
`LayoutTheme` guarded by `skip_serializing_if = "LayoutTheme::is_empty"`
(`native-theme/src/model/mod.rs:268-269`) and shared across the light and dark
variants, and all four of its own fields are `Option<f32>`
(`native-theme/src/model/widgets/mod.rs:896-913`), so an absent layout costs
nothing. It benefits the iced and gpui connectors equally. The atlas takes it on
every path — `from_system`, `SystemThemeExt::to_egui_atlas` and
`Builder::layout` (specification §4.3, §4.6) — so `Spacing::item_spacing`,
which `layout.widget_gap` feeds, follows the platform on the `from_system()`
path too, and the central panel's margin comes from `layout.window_margin`
through `Surface::CentralPanel` and reaches no `Style` field (specification §14
item 18); a `None` field leaves egui's value (§3.23). `Spacing::window_margin`
never depended on the change: it is written from `window.border.padding`, which
`ResolvedTheme` carries on every path.

**Q-3 — `egui_kittest`. DECIDED: a dev-dependency,
`egui_kittest = { version = "0.36.2", features = ["eframe"] }` (specification
§11), used by the showcase's self-tests alone (specification §13 T11).** The
question the first version left open is settled by contract C7 (§3.24): every
showcase carries self-tests that render and click headlessly
(`docs/archive/todo_v0.5.9_theme-contracts-rationale.md:525`), and a bare
`Context` can be fed raw pointer events but offers no way to find a widget by
its label, so every click would be aimed at coordinates the test hardcodes. The
version-aligned 0.36.2 source was read (`egui_kittest/Cargo.toml:14`:
`rust-version = "1.95"`, the same floor as egui; it depends on `egui` 0.36.2
with `default-features = false`, `egui_kittest/Cargo.toml:119-121`, and on
`kittest` 0.4.0, `egui_kittest/Cargo.toml:133-134`, whose own manifest declares
`rust-version = "1.92"`, `kittest/Cargo.toml:14`):

* **Driving the real app.** `HarnessBuilder::build_eframe`
  (`egui_kittest/src/builder.rs:212`, behind the `eframe` feature) makes a fresh
  `Context`, runs the app's creation closure against it
  (`egui_kittest/src/builder.rs:219-226`) and steps the app's own `logic` and
  `ui` every pass (`egui_kittest/src/app_kind.rs:38-42`). `with_theme` picks the
  `egui::Theme` (`egui_kittest/src/builder.rs:75`); the harness's `Context` is a
  public field (`egui_kittest/src/lib.rs:73`).
* **Finding and clicking.** The harness implements `kittest`'s `Queryable` over
  the AccessKit tree (`egui_kittest/src/lib.rs:963-969`), whose `get_by_label`
  family is generated at `kittest/src/query.rs:161-172`; `Node::click` queues a
  primary-button click at the node's centre (`egui_kittest/src/node.rs:61`), and
  `Harness::state` hands back the app (`egui_kittest/src/lib.rs:478`). AccessKit
  is a non-optional dependency of egui 0.36.2 (`egui/Cargo.toml:82`), so none of
  this needs a feature beyond `eframe`.
* **Rendering stops before pixels.** Without the `wgpu` feature the harness has
  no renderer (`egui_kittest/src/renderer.rs:37-46`), and without `wgpu` or
  `snapshot` `Harness::render` does not exist (`egui_kittest/src/lib.rs:690-691`).
  Neither is enabled: `snapshot` compares against stored images, the Layer 4
  v0.5.9 deliberately deferred
  (`docs/archive/todo_v0.5.9_theme-contracts-rationale.md:545`), and no test
  here needs pixels. What the self-tests do run — layout, painting into shapes,
  tessellation — is everything this crate's styles feed. That is the same class
  of test as iced's `Simulator` and gpui's test platform in the sibling
  showcases; iced's harness can also produce pixels, which here would cost the
  `snapshot` feature, and the gpui showcase's tests render on a test platform
  with no snapshot API, which is the reading taken here.
* **Harness behaviour the tests must respect.** After the creation closure the
  harness pins the theme preference (`egui_kittest/src/lib.rs:142`) and
  overwrites cursor blink, scroll animation and `animation_time` in every style
  (`egui_kittest/src/lib.rs:145-149`); `Harness::run` panics past its step limit
  (`egui_kittest/src/lib.rs:356`, default four at
  `egui_kittest/src/builder.rs:39`); a single-node label query panics on two
  matches (`kittest/src/query.rs:65-70`). Specification §13 T11 records how each
  is handled. The crate allows `unwrap` internally (`egui_kittest/src/lib.rs:5`),
  which is harmless for a dev-dependency.

Pixel snapshots stay out until a defect that click-and-query tests provably
cannot express appears (§6), the trigger theme-contracts set for screenshot
diffing.

**Q-4 — `#[non_exhaustive]` on `PanelSide`? DECIDED: it stays exhaustive.** A
rectangle has four sides; that is a closed fact of 2-D screen geometry, not an
API taxonomy that can churn, so no fifth variant can arrive and applications
must be able to `match` it (§3.2). The cost of being wrong would be one breaking
change in a pre-1.0 crate; the ergonomic gain is permanent.

**Q-5 — A shared contract vocabulary across the three connectors. DECIDED: A —
each connector keeps its own contract module; the egui connector writes one in
the siblings' shape.** v0.5.9 deferred "a shared contract vocabulary across
connectors (one table, two backends)" with the trigger **"A third connector"**
(`docs/archive/todo_v0.5.9_theme-contracts-rationale.md:548`). This crate is
that connector, so the question is due. What the three could share is only the
*native* half of a contract: the target half is a gpui `ThemeColor` field, an
iced palette slot or `styles::*` field, and here an `egui::Style` path, and no
row type spans them.

| Option | Cost |
|---|---|
| **A. Each connector its own contract module** (the status quo, plus `mapping.toml`) | The 32-combination loop, straight-alpha compositing and the composited ratio exist once per connector, each in its own colour type |
| **B. Share the native half** — the combination loop, a native text-on-fill pair list, and the compositing and ratio arithmetic, over `native_theme::Rgba` — in a dev-only workspace crate | One more workspace member that is never published; the two v0.5.9 suites rewritten onto it; published connector tarballs whose unit tests no longer build (below) |
| **C. One table, three backends** — a single row set keyed by native leaf, each connector supplying a getter per row | The row set would have to hold every toolkit's exceptions and derivations side by side, and a row one toolkit cannot receive would need a per-toolkit "unreachable" mark; the table stops being readable as any one connector's contract, which is the property the v0.5.9 design chose tables for |

An earlier revision of this document chose B, on the argument that three copies
of the native half could drift into three different truths about the platform,
and one could not. Measurement took the premise away: the siblings have no
shared native half to extract. Their pair lists are not one list: each row pairs
a native triple with the toolkit getter it is compared against — iced's
`ReportPair` (`connectors/native-theme-iced/src/contract.rs:666`, rows from
`connectors/native-theme-iced/src/contract/pairs.rs:11`) and gpui's `Pair`
(`connectors/native-theme-gpui/src/contract.rs:1309`, rows from `:1349`; 128
`native:` closures in the file) — and each connector picks the native triple for
the widget its toolkit paints, which is a per-toolkit decision. What is truly
the same is three small functions — `combinations`, `over` and `pair_ratio`, 46
lines in iced (`connectors/native-theme-iced/src/contract.rs:820-842`,
`:852-870`, `:874-877`) and 65 in gpui
(`connectors/native-theme-gpui/src/contract.rs:974-996`, `:669-706`,
`:1802-1805`) — and sharing them would rewrite the 9,582 lines of shipped
contract tests (`connectors/native-theme-gpui/src/contract.rs` and
`connectors/native-theme-iced/src/contract.rs` with its three submodules, `wc
-l` at `2db686ee`) for no gain in fidelity, safety or verifiability. It would
also cost the published tarballs their tests: measured with cargo 1.98.1 in a
throwaway clone of `2db686ee`, cargo drops a path-only dev-dependency from the
packaged manifest — `cargo package` passes, verification included, and the
normalized `Cargo.toml` names no such crate — so a published connector's own
unit tests would no longer build from its tarball. **DECIDED: A; B and C
rejected**, C for the reason in the table. The egui connector writes its own
`#[cfg(test)]` `contract` module (specification §13 T10, T12) in the siblings'
shape. Revisit when a fourth connector arrives or the connectors come to share a
pair list.

**Q-6 — The showcase's scope. DECIDED in specification §10.4, by the goal.** The
egui showcase is the gpui showcase's application built from egui's own
containers, with per-instance Widget Info generated from `mapping.toml` and a
palette of every widget egui 0.36.2 and egui_extras 0.36.2 offer. §3.25 argues
it: the look in question is an application's, egui can express the chrome, egui
itself says which instance is innermost, and generated Widget Info is cheaper
and stronger than cited prose. The earlier revision's position — the iced
baseline, with the chrome and the info filed for later — was rejected by the
maintainer.

**Q-7 — Applying the accessibility preferences. DECIDED in specification §4.3,
by the goal.** The atlas applies what egui has a sink for, as the siblings do:
every text size it writes is `scaled_text_size` of the theme's, and
`reduce_motion` sets `animation_time = 0.0` and
`scroll_animation = ScrollAnimation::none()` in every style;
`reduce_transparency` and `high_contrast` have no `Style` sink and are only
exposed (specification §14 item 27). A native application follows the OS
text-scaling factor and the reduced-motion preference, so an application on this
connector that ignored them would neither look nor behave natively, and nothing
in egui or eframe 0.36.2 reads either preference, so a preference the atlas
dropped would be dropped for good. §4.4 (item 27) argues it in full, with the
alternative this document specified before — the theme's own sizes and egui's
own motion, left to the application through the accessors — rejected on the
same ground.

**Q-8 — Does a checked checkbox change on hover? DECIDED: the hovered
`Selected` cell copies the idle `Selected` cell.** native-theme models one
`checkbox.hover_background` (`native-theme/src/model/widgets/mod.rs:170-172`),
a `soft_option` with no checked counterpart in `CheckboxTheme` (`:146-188`). All
16 presets that state colours state it, in both modes (the four `-live` presets
carry geometry only), and only `windows-11` says what it is — "Fluent checkbox
hover fill" (`native-theme/src/presets/windows-11.toml:130-131`) — without
saying for which box state; Breeze draws no hover fill at all, but a rounded
outline in its focus colour whatever the check state (Breeze at tree
`be6e137e`, kstyle/breezestyle.cpp lines 4900-4938 and kstyle/breezehelper.cpp
lines 897-908, linked from `docs/todo.md`). Laying that layer over
`checkbox.checked_background` would put a value where the platform shows
something else, which the goal rules out. The checked box's own hover appearance
is a value the model does not carry, and a missing soft value is filled by
copying the base-state value, never by arithmetic (C16, §3.6), so the hovered
`Selected` cell is the idle `Selected` cell (specification §6.1, §6.4). The iced
connector does the same: a hovered checked checkbox shows the plain checked box
(`connectors/native-theme-iced/src/styles.rs:544-547`). The gap is the model's,
and it is filed in `docs/todo.md` (Core API, *Checkbox: a checked checkbox's
hover appearance*): once the platforms' checked-hover appearance is researched
into `docs/platform-facts.md` and modelled as a `checkbox` `soft_option`, the
hovered `Selected` cell takes it and this copy becomes its `None` fallback.

**Items that stood open and are settled.** **The body row heights specification
§6.6 reasons from** are measured (specification §6.6), and at a larger
text-scaling factor the build computes the slider's `expansion` from the row
height, so no measurement is needed for the knob to stay `d` across.
**`want − row` on KDE and GNOME with the platform's own face** is measured
(§3.12; specification §6.15). **A negative `extra_text_line_spacing`** ran without a panic on
every path tried (§3.12). **The transparent `bg_fill`** is reported by a `Note`
variant of its own, `TransparentFill` (§3.19; specification §4.3, §6.4, §7.2).
**eframe's wgpu backend answers the showcase's screenshot command** as the glow
backend does, read in egui-wgpu 0.36.2 (specification §10.4). And **which finite
length is safe on egui's layout paths** is derived rather than measured:
`GuiRounding::round_ui` divides by `GUI_ROUNDING` before it rounds
(`emath/src/gui_rounding.rs:58-59`, the constant `1.0 / 32.0` at `:18`), so a
length above `f32::MAX * GUI_ROUNDING` becomes `+∞`, and `convert::clamp_length`
keeps every length sink below that ceiling (§3.9; specification §6.7, §13 T17).
A *sum* of lengths each below it can still exceed it; that is specification
§6.7's stated residual, not an open question.

## As built

The implementation was ported onto `v0.6.0-rc1` from a task-by-task gated dry
run of the plan, then reviewed over the whole branch by six reviewers and
corrected in four fix passes (`47baf7fd`..`c43bbc07`). Two decisions were
settled differently from this document as first written; both are now in it.

**Strong text is the panel's text colour** (§3.6, `47baf7fd`). The base style's
pressed text, which egui reads as strong text, is `defaults.text_color`, not
`button.active_text_color`; the latter is SCOPED to the `Role::Button` cell,
and the totals are 482 = 47 DIRECT · 206 SCOPED · 155 DERIVED · 74 UNMAPPABLE.

**The system sets' glyphs are coloured when they are loaded** (§3.13,
`8e8de66a`). native-theme's `SfSymbolsLoader` and `SegoeIconsLoader` gained
`color` / `color_opt` (`f6cbb01c`), the Segoe set's `ActionSearch`,
`ActionSettings`, `ActionDelete` and `ActionPrint` became the Fluent glyphs,
the showcases load the glyphs in the text colour (`3db595b5`), and all three
connectors' custom-provider paths load them in the icon's colour (`d36c8081`,
`87a99ccc`, `95df6010`, `99db360e`).

Still open: "SF Mono" resolves in neither iced's font database, fontdb nor
gpui's font names (run 36315092575; specification §15); whether AppKit hands
Cmd+B to the menu before winit sees it needs a person at a Mac; and
gpui-component's widgets cannot show the SF Symbols and Segoe rasters
(`docs/todo.md`, *Platform icons inside widgets*).
