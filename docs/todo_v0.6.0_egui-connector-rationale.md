# v0.6.0 — egui Connector: Rationale

Status: Pending
Crate: `connectors/native-theme-egui`
Target toolkit: **egui 0.36.1**
Companion specification:
[`todo_v0.6.0_egui-connector-spec.md`](todo_v0.6.0_egui-connector-spec.md)

---

## 0 -- What this document is for

The specification is the WHAT and the HOW. This is the WHY. It exists so that a
maintainer opening this repository in three years can answer three questions
without re-deriving anything:

1. Why is the connector shaped like this and not like the two sibling
   connectors?
2. Which alternatives were considered, and what specifically killed each one?
3. Which of the 150 losses are egui's fault, which are native-theme's, and which
   are deliberate refusals — and what would close each?

Everything settled here is settled. A question re-opened without new evidence
from source is a question already answered below.

**Reading contract.** Every factual claim about egui, epaint, ecolor, emath,
eframe or egui_extras is cited as `path:line` relative to that crate's own root,
at version **0.36.1**. Every claim about this repository is cited relative to the
repository root. egui 0.35.0 appears only as churn evidence — §2 (Option D), §4.2,
§5.1 and §7 — and is never a target. Anything that could not be established from source is
marked **UNVERIFIED**, together with what would verify it. An honestly marked
hole is correct; a plausible fabrication is a total failure.

**The claim this document defends.** Not "full theme geometry". The headline is
the specification's §0.1 sentence: *an excellent global theme out of the box and
opt-in per-widget geometry*. §1 is the proof that this is the strongest honest
claim available against egui 0.36.1, and §4.2 names the upstream change that
would make the stronger claim true.

---

## 1 -- The core tension, argued from evidence

### 1.1 The two shapes, measured

`native_theme::theme::ResolvedTheme` (`native-theme/src/model/resolved.rs:155`)
is `defaults` + `text_scale` + **25 per-widget structs** in declaration order
(`resolved.rs:163-211`: window, button, input, checkbox, menu, tooltip,
scrollbar, slider, progress_bar, tab, sidebar, toolbar, status_bar, list,
popover, splitter, separator, switch, dialog, spinner, combo_box,
segmented_control, card, expander, link). Flattened to leaf scalars — with
`ResolvedFontSpec` (5 leaves, `native-theme/src/model/font.rs:239-251`) and
`ResolvedBorderSpec` (8 leaves, `native-theme/src/model/border.rs:94-111`)
expanded — that is 459 leaves, plus the four `LayoutTheme` fields
(`native-theme/src/model/widgets/mod.rs:884-901`): **463**.

`egui::Style` (`egui/src/style.rs:243`) has 17 fields in a debug build and 16 in
release, because `Style::debug` is `#[cfg(debug_assertions)]` (`style.rs:322-323`).
All widget appearance flows through `Visuals::widgets: Widgets`
(`style.rs:1029`, struct `:1249`): **five** entries — `noninteractive` `:1254`,
`inactive` `:1257`, `hovered` `:1262`, `active` `:1265`, `open` `:1268` — each a
`WidgetVisuals` (`style.rs:1289`) with exactly **six** fields: `bg_fill` `:1294`,
`weak_bg_fill` `:1299`, `bg_stroke` `:1304`, `corner_radius` `:1307`,
`fg_stroke` `:1310`, `expansion` `:1318`.

Thirty distinct colour-and-geometry slots for twenty-five widgets. That ratio is
the whole design problem, and no amount of API taste makes it go away.

### 1.2 The asymmetry, stated exactly

`Widgets` is indexed by **interaction state**. It has **no widget-type axis**.
This is not a gap in the documentation; it is observable in the dispatchers.
`Widgets::state` (`egui/src/widget_style.rs:94-99`) takes a `WidgetState` and
nothing else, and `WidgetState` has four variants — `Noninteractive`,
`Inactive`, `Hovered`, `Active` (`widget_style.rs:84-90`). `Response::widget_state`
(`:105-115`) computes that value from pointer and focus alone. Nothing anywhere
in that path knows whether it is styling a button or a text field.

The consequence is concrete and unavoidable: a `Button` and a `TextEdit` in the
same `Ui` cannot have different corner radii from theme data. The button reads
`widget_style.rs:161`; the text edit reads
`widgets/text_edit/builder.rs:727`, `:732`, `:737`. Same field, same value.

### 1.3 The contested-field evidence — the actual proof

A field is **contested** when two or more native-theme leaves need different
values in it at the same time. A contest cannot be resolved by choosing better
defaults, by writing more careful code, or by any refactor internal to the
connector: it is a cardinality fact. The complete list is specification §5.11;
these are the ones that decide the architecture:

| egui field | decl | claimants | worst pair |
|---|---:|---:|---|
| `text_styles[*].family` collectively | `epaint/src/text/fonts.rs:32` | 21 | any two per-widget `font.family` values |
| `visuals.widgets.<state>.bg_stroke` | `style.rs:1304` | 19 | `separator.line_width` vs `card.border.line_width` |
| `spacing.interact_size.y` | `style.rs:408` | 13 | `toolbar.bar_height` vs `button.min_height` |
| `visuals.selection.bg_fill` | `style.rs:1195` | 12 | `progress_bar.fill_color` vs `defaults.selection_background` |
| `visuals.widgets.<state>.corner_radius` | `style.rs:1307` | 11 | `button.border.corner_radius` vs `checkbox.border.corner_radius` |
| `visuals.widgets.noninteractive.bg_stroke` | `style.rs:1304` | 11 | `separator.line_color` vs `list.grid_color` |
| `visuals.widgets.inactive.fg_stroke.color` | `style.rs:1310` | 11 | `expander.font.color` vs `expander.arrow_color` |
| `visuals.widgets.noninteractive.fg_stroke.color` | `style.rs:1310` | 12 | `tooltip.font.color` vs `status_bar.font.color` |
| `visuals.widgets.hovered.weak_bg_fill` | `style.rs:1299` | 8 | `menu.hover_background` vs `button.hover_background` |
| `visuals.disabled_alpha` | `style.rs:1125` | 8 | any two per-widget `disabled_opacity` values |
| `visuals.window_fill` | `style.rs:1062` | 6 | `tooltip.background_color` vs `menu.background_color` |
| `visuals.panel_fill` | `style.rs:1071` | 6 | `sidebar.background_color` vs `status_bar.background_color` |

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
KDE Breeze `toolbar.bar_height` is `40.0` and `button.min_height` is `32.0`
(`native-theme/src/presets/kde-breeze.toml:198`, `:90`). No arithmetic
reconciles them, because there is nothing to reconcile: they are two different
numbers that must both be written into one `f32`.

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
falls back to `ctx.global_style()` (`egui/src/ui.rs:135`). Everything built on
`Area` — `Window`, `Popup`, `Tooltip`, `Modal`, menus, `ComboBox` popups — reads
the **Context** style and ignores the calling `Ui` entirely. Wrapping a scope
around a `Window::show` has literally no effect on that window.

This is why the architecture has *three* seams and a fourth non-`Style` carrier
rather than one, and why `Surface` exists beside `Role` (§3.3).

### 1.5 Two aggregates, and why both are printed

Every aggregate quoted in this document is the **matrix aggregate** —
**DIRECT 32 · SCOPED 209 · DERIVED 72 · UNMAPPABLE 150** — unless it says
otherwise, because that is what the audited `mapping.toml` rows carry and what
the verdict column of every mapping table means: *what egui 0.36.1 can express*,
not what this crate chooses to do.

Specification §5.8 also prints the **effective** aggregate under the locked
decisions — **31 · 210 · 54 · 168** — which differs in exactly two places:
`list.header_font.size` moves DIRECT → SCOPED because no `TextStyle::Name` key
is ever added (§3.11 is the reason; `list.header_font.family` is DERIVED either
way, because a family is bytes and not a name), and eighteen per-widget
`font.weight` / `font.style` leaves move DERIVED → UNMAPPABLE because the
never-`Name` invariant forbids the mechanism the inputs and chrome matrices
assumed. Both numbers are published, so neither is a surprise, and the
difference is itself an argument: the six other decisions that override a
matrix verdict change the *mechanism*
without changing the bucket, because the value still reaches the application
through a documented accessor plus a per-call egui mechanism.

### 1.6 What "full theme geometry" can and cannot mean here

Three readings, and the crate is honest about which one it delivers
(specification §1.4):

1. **Every mappable leaf reaches some egui object.** Reachable, and delivered.
2. **Every mappable leaf renders correctly with no application cooperation.**
   Not reachable in 0.36.1. There is no hook that could make it so — §1.2–§1.4
   are the proof, and §4.2 is the upstream change that would create one.
3. **Every leaf renders correctly.** Not reachable in any egui today; 150 leaves
   have no expression at all, and 38 of those are native-theme's own holes.

The iced connector could promise (2) because `iced::Theme` is consulted through
per-widget `Catalog` traits. The gpui connector could aim at it only if
gpui-component accepts the per-widget `ThemeConfig` metrics its own
investigation proposes — an unsubmitted draft
(`docs/todo_v0.6.2_gpui-full-theme.md:163-183`, status at `:221-222`) — so today
it cannot promise (2) either. egui has no equivalent seam even to propose
against, because `Style` has no widget-type axis at all. Promising (2) here would
be a lie, and the phrase "full theme geometry" is therefore banned from the
crate's own claims and confined to naming the goal in §4.2.

---

## 2 -- Options considered

House convention (`docs/todo_v0.6.1_iced-full-theme-geometry.md:21-92`): every
option gets a fair statement of its appeal before the reason it loses.

### Option A: Global `Style` only, no per-widget mechanism (rejected)

Map the theme onto one `egui::Style`, install it with `Context::set_style_of`
(`egui/src/context.rs:2247`), ship the rest as documentation. This is what a
first-cut egui connector looks like, and it is the shape both sibling connectors'
`to_*_theme` entry points have (`connectors/native-theme-iced/src/lib.rs:197`,
`connectors/native-theme-gpui/src/lib.rs:263`).

**Appeal.** Tiny API — one type, one function. Nothing to learn. No scoping
noise at any call site. Impossible to misuse. Every widget in the application is
themed uniformly with zero cooperation, which is precisely the property (2) of
§1.6 that the chosen design cannot promise.

**Rejected because** it can carry exactly one claimant per contested field.
§1.3's table is the arithmetic: of the 209 SCOPED leaves only the elected winner
of each contested field survives, and every other claimant is silently dropped —
not because it is hard to carry, but because there is nowhere to put it. The
failure is invisible — the application renders, it just renders `menu.hover_background` on
the tabs and `button.min_height` on the toolbar. This is the "plausible
fabrication" failure mode the project's rules forbid, applied to geometry
instead of to prose.

Note carefully what is *not* rejected: Option A **is** the base layer of the
chosen design. The base `Style` is exactly this, and the 31 **effective** DIRECT
leaves (§1.5) plus one elected winner per contested field reach the screen
through it with no cooperation at all. What is rejected is stopping there.

### Option B: Ship replacement widgets in the connector (rejected)

The connector ships `native_theme_egui::Button`, `::Checkbox`, `::TextEdit`, …,
each reading `ResolvedTheme` directly and painting with `Ui::painter`. This is
the gpui investigation's Option B/F shape
(`docs/todo_v0.6.2_gpui-full-theme.md:225-258`, `:357-484`).

**Appeal.** Total fidelity. Every one of the 463 leaves is reachable, because
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
re-argued every release. `Switch` passes both tests today — 13 leaves, 8
UNMAPPABLE, no egui counterpart (specification §5.3) — and is still declined,
which is the strongest possible demonstration that the charter is a rule and not
a preference.

### Option C: Fork egui (rejected)

Vendor egui, add the widget-type axis, publish the fork, depend on it.

**Appeal.** Everything becomes possible immediately, with no upstream
negotiation and no waiting.

**Rejected** on three independent grounds, any one of which is sufficient.
First, the connector's whole value is that the application's `egui::Style` is
*our* `egui::Style`; a fork guarantees two incompatible egui crates in one
dependency graph, which is the single most common downstream failure mode with a
toolkit connector and produces `expected egui::Style, found egui::Style`.
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
  0.36.1**, verified by `diff`. It did not gain a single line across a release.

The last point is the decisive one. A module that changed between releases might
be worth betting on. A frozen module is not — and a connector that shipped
against `Classes` would be shipping against an API that upstream has, so far,
not moved.

The same fact is the strongest possible argument *for* a well-argued upstream
PR, because nothing in flight competes with it. That is §4.2, and the
non-negotiable rider there is that **the connector must not depend on it**.

### Option E: A runtime data table consulted per widget (rejected)

Keep one `Style`, put the 463 mapped values in a table inside the connector, and
have the application ask the table for what it needs at each call site —
`ui.add(egui::Button::new("x").corner_radius(atlas.button_corner_radius()))` and
so on.

**Appeal.** No `Style` cloning, no per-role allocation, complete data available
at every call site, and the table is trivially testable in isolation.

**Rejected because** egui's builders do not accept most of the values: for the
great majority of the 463 leaves there is no per-call setter at all — no
`TextEdit::border_width`, no `ComboBox::padding`. The partial exceptions are
real, and narrow. `Button` takes `fill` (`egui/src/widgets/button.rs:143`),
`stroke` (`:151`), `min_size` (`:193`), `corner_radius` (`:200-203`, applied
over the style-derived radius at `:349-350`) and `gap` (`:277`); `TextEdit`
takes a whole `Frame` (`egui/src/widgets/text_edit/builder.rs:306`) plus
`margin` (`:313`). That is exactly why the specification routes per-position
segmented-control radii through `Button::corner_radius` at the call site
(specification §5.3, row `segmented_control.border.corner_radius`). Everything
else egui reads, it reads from `Style`, which is the whole point of §1.2.
A table would therefore be a table of numbers the application must apply with
`Ui::painter` by hand — which is Option B without the widgets, i.e. all of the
work and none of the rendering.

The useful residue of this option is kept: the 62 free accessors of
specification §4.7 are exactly this table, restricted to the values that no
`egui::Style` field carries **for that widget** — whether because no sink exists
at all, because the leaf lost a contest to another claimant, or because a sink
exists and writing it was deliberately declined. All three shapes are present:
`focus_ring_color()` covers a DERIVED leaf whose sink is real and refused
(specification §5.8 item 1, row at §5.1); `dialog_max_size()` covers a SCOPED
leaf whose sink is real and refused (§5.8 item 6); `list_row_height()` exists
because `egui_extras::Table` takes the row height as a **call argument** rather
than from `Style` (specification §5.4). Those are genuinely the application's to
paint. That is the difference between a table as the architecture (rejected) and
a table as the honest remainder (adopted).

### Option F: Helpers only — document the mapping, install nothing (rejected)

Publish the 463-row mapping as documentation plus accessors and let the
application build its own `Style`.

**Appeal.** Zero API risk. Nothing to keep in sync with egui except the
documentation.

**Rejected** for the reason the iced investigation rejected the same shape
(`docs/todo_v0.6.1_iced-full-theme-geometry.md:80-92`): every application would
duplicate the same boilerplate, and the connector's entire job is to bridge
`native-theme` to the toolkit. Handing back a list of numbers is not a bridge.
It also fails the project's own no-hardcoded-values rule in practice: an
application assembling a `Style` by hand from an accessor list will fill the
gaps with literals.

### Option G: Pre-built per-role styles over egui's own seams, with an audited manifest (chosen)

One `ThemeAtlas` compiles, once, per `egui::Theme`:

* one base `egui::Style` — Option A, and it is what every unwrapped widget gets;
* one `Arc<egui::Style>` per (`Role`, `RoleVariant`) — 25 roles × 3 variants;
* one `egui::Frame` per `Surface` — 12 entries with panel sides expanded;

delivered through the three substitution points egui already has (specification
§3.2), plus `egui::Frame` values as a fourth, non-`Style` carrier. Every one of
the 463 leaves gets a row in a checked-in `mapping.toml` carrying its verdict and
its declared egui sinks, and a differential headless test proves the manifest
describes the code rather than the document (specification §13 T3, T4).

**Why it wins.**

* It is the only option that carries more than one claimant per contested field
  **without** owning any paint code (§1.3 makes multi-`Style` mandatory; Option B
  is the only alternative that achieves it).
* Every mechanism it uses is upstream API that egui uses itself. Nothing is
  invented: `UiBuilder::style` (`ui_builder.rs:155`, field `:28`) consumed by
  `Ui::scope_builder` (`ui.rs:2193`) and `Ui::new_child` (`ui.rs:236`);
  `Ui::set_style` (`ui.rs:386`); `egui::style::StyleModifier` (`style.rs:193`)
  consumed by `Popup::style` (`containers/popup.rs:417`), `MenuConfig::style`
  (`containers/menu.rs:107`), `MenuBar::style` (`:241`) and `ComboBox::popup_style`
  (`containers/combo_box.rs:199`).
* It degrades to Option A exactly, and gracefully: an application that installs
  and scopes nothing gets a correct global theme, and `native_scope` on a
  `Context` with no atlas is a plain `ui.scope(..)` (`ui.rs:2185`) that never
  panics and never returns an `Option`.
* It keeps the two churning sides separate. `Role` tracks `ResolvedTheme`'s
  widget list and nothing else; `Surface` tracks egui's `Frame` attachment points
  and nothing else. §5 is the argument that this is what makes the design
  survivable.

**What it costs, stated up front.** Per-call-site noise where fidelity is wanted
(`ui.native_scope(Role::Input, |ui| ..)`); a memory footprint that is bounded but
not measured (§8, Q-1); and the honest admission that fidelity is opt-in, which
is ledger item 22 and the reason the README's first paragraph is the sentence in
specification §0.1.

### 2.8 Alternatives inside the chosen shape, and why each lost

These were live proposals during design. Each is recorded with its defeating
fact so it is not re-proposed.

| Rejected | Why |
|---|---|
| Naming the handle `EguiTheme` | `egui::Theme` is a real two-variant type (`egui/src/memory/theme.rs:6`, re-exported `egui/src/lib.rs:483`). The crate would ship `fn base_style(&self, theme: egui::Theme)` *on a type called `EguiTheme`*, and a `for_theme(&self, theme: egui::Theme) -> &EguiTheme`. `ThemeAtlas` names what the value is — a compiled table indexed by two axes — and no signature in the crate is then ambiguous |
| `Surface::{SidePanel, TopPanel, BottomPanel, CentralPanel}` | Its own stated rule was "one variant per egui container", yet three of the four were distinguished only by *native-theme* concepts, and two recycled the names of types that **do not exist** in egui 0.36.1 (a recursive grep over `egui/src` returns zero hits; the types are `Panel`, `containers/panel.rs:206`, with `left`/`right`/`top`/`bottom` at `:249`, `:256`, `:265`, `:274`, and `CentralPanel`, `:1187`). `Surface::Panel(PanelSide)` is one variant for egui's one `Panel` type |
| `Role::CheckboxOn` / `Role::CheckboxOff`, `Role::ButtonPrimary` | Encoding widget *state* in a widget-*type* taxonomy forces `let role = if self.notify { Role::CheckboxOn } else { Role::CheckboxOff };` at every call site, and inverting that condition is silent. `RoleVariant` is a *value* — `RoleVariant::from_selected(self.notify)` — so the state stays where the state already lives |
| Re-exporting `Rgba` at the crate root | The crate re-exports `egui`, and `egui::Rgba` (`egui/src/lib.rs:442` → `ecolor/src/rgba.rs:3-10`) is *linear f32, premultiplied* while `native_theme::color::Rgba` is *sRGB u8, straight* (`native-theme/src/color.rs:42-52`). `native_theme_egui::Rgba` beside `native_theme_egui::egui::Rgba` with opposite colour spaces is a trap that survives casual testing. It lives at `convert::Rgba`, next to the function that consumes it |
| A `Metrics` struct instead of free accessors | Adding a field to a public struct is a change; adding a free function is not. Since the whole non-`Style` surface exists precisely because native-theme keeps growing, it must be the shape that grows without a version bump. This also matches both sibling connectors (`connectors/native-theme-iced/src/lib.rs:212-393`, `connectors/native-theme-gpui/src/lib.rs:288-458`) |
| A `StyleSet` struct with 26 public fields | Struct-literal-constructible downstream, so adding a native-theme widget breaks every literal; and `pub const ALL: [Self; 25]` bakes a count into a public type signature |
| Declining `#[non_exhaustive]` crate-wide on the grounds that the egui pin already buys compatibility | Wrong axis. native-theme churn is independent of egui churn: two new widget structs force two new `Role` variants with no egui bump at all. The pin buys nothing there. See §5.2 |
| An `EGUI_VERSION: &str` constant | Cannot be checked against the resolved dependency (`egui = "0.36.1"` accepts any 0.36.x), so it would eventually become a lie. The version policy lives in `Cargo.toml` and in a README table |
| `egui_kittest` in `[dev-dependencies]` | A version-aligned `0.36.1` does exist (`rust-version = "1.95"`, verified against the crates.io sparse index), but its **API is UNVERIFIED** — the crate was never vendored or read, so no test here is written against it. All eleven test groups are expressible without it, and a dependency that buys nothing is still a dependency to track across every egui bump. *What would verify it*: vendoring `egui_kittest` 0.36.1 and reading its harness API |
| A direct `epaint` / `ecolor` / `emath` / `skrifa` dependency | egui re-exports the first three (`egui/src/lib.rs:436-438`), so a direct dependency only creates a way to end up with two `ecolor`s. epaint does not re-export `skrifa`, so a `skrifa` dependency here could drift from epaint's at any egui minor |
| An `InstallReport` return value from `install` | Diagnostics belong to the *atlas*, which is where the conversions happen, not to installation, which cannot fail. `ThemeAtlas::notes()` is populated once per theme change |
| A `refresh_line_spacing(&Ui) -> bool` two-phase protocol | A protocol on the common path whose failure mode is silent degradation. §3.12 |
| A post-pass retint of the shapes through `Plugin::output_hook` | Recorded because it is the one mechanism a contributor will find, prototype and propose. It is **unsound**, not merely fragile: nothing in the output carries widget identity. The hook (`egui/src/plugin.rs:44`, dispatched at `:181`) is invoked after `end_pass` and before `tessellate` (`egui/src/context.rs:2458`, `:2460-2461`, `:2856`), and what it can reach is `ClippedShape`, which is `{clip_rect, shape}` (`epaint/src/lib.rs:117-124`) — and none of `Shape`'s twelve variants (`epaint/src/shapes/shape.rs:27-71`) records which widget produced it. `WidgetRect` has seven fields and no widget type (`egui/src/widget_rect.rs:9-49`); the only structure that *does* carry a kind, `WidgetRects::infos` (`:105`), is a private field whose sole writer is `#[cfg(debug_assertions)]` **and** gated on `show_interactive_widgets`, with a release body of `_ = (self, id, make_info);` (`egui/src/context.rs:1569-1581`). `Button` paints at `widgets/button.rs:380` and registers its `WidgetInfo` only afterwards at `:391`, so the attribution does not exist even in a debug build at the moment the shape is created |

---

## 3 -- The decision record

One subsection per locked decision. Each names what was chosen, what was
rejected, and the fact that decided it.

### 3.1 The handle: opaque, `Arc`-backed, infallible to obtain

**Chosen.** `ThemeAtlas(Arc<AtlasInner>)`, opaque, `Clone`, `Send + Sync +
'static`, carrying both colour schemes and both source `ResolvedTheme`s, plus
`ThemeAtlas::passthrough()` whose styles are `egui::Theme::default_style()`
(`egui/src/memory/theme.rs:24-29`).

**Rejected:** returning `Option<ThemeAtlas>` from the context accessor as the
only path. That forces every frame body to open with a degraded-UI branch —
`let Some(theme) = ui.ctx().native_theme() else { .. return; };` — in an
immediate-mode application, i.e. in the hottest, most-written code in the whole
program. `native_theme()` is infallible and `native_theme_opt()` exists for the
applications where "nothing installed" is genuinely an error.

**What the infallible handle does *not* buy, stated so the claim stays true.**
It removes the branch before *installing, scoping or asking for a frame*:
`base_style`, `role_style`, `role_style_variant`, `role_modifier` and
`surface_frame` are total on a passthrough atlas. It does not remove the
`Option` in front of the free accessors, which need a `&ResolvedTheme` that a
passthrough atlas does not have — `resolved_for` still returns `None` there, and
`#![deny(clippy::unwrap_used)]` means the application must handle it. The
recommended shape is one `let Some(t) = atlas.resolved_for(ui.ctx().theme())
else { .. };` at the top of the frame body, not one per call site.

**Why opaque.** Clause 6 of the semver contract: adding a method to an opaque
type is additive. An atlas with public fields would make every future mapping
refinement a breaking change.

**Why it carries the `ResolvedTheme`s.** Fifty-eight of the 62 free accessors of
specification §4.7 take a `&ResolvedTheme`; the remaining four take
`&SystemTheme`, because `AccessibilityPreferences` lives there
(`native-theme/src/lib.rs:422`) and is deliberately absent from
`ResolutionContext` (`native-theme/src/resolve/context.rs:20-23`). If the atlas did not carry them, every application would
hold a second theme object beside the atlas — and an application struct with both
is exactly the shape that produces a borrow-check failure when a `&self`
accessor stays live across a closure capturing `&mut self.field` (`E0502`).
`atlas.resolved_for(ui.ctx().theme())` is one call, on the object the
application already has, and it also removes the five-line
`if t == egui::Theme::Dark { ColorMode::Dark } else { ColorMode::Light }` wart
that an atlas keyed only on `ColorMode` would force at every call site.

### 3.2 Two axes, two vocabularies, and state as a value

**Chosen.** `Role` (25 variants, `#[non_exhaustive]`) named in **native-theme's**
vocabulary, exactly one per widget field of `ResolvedTheme` in declaration order
(`resolved.rs:163-211`). `Surface` (9 variants, `Panel` carrying `PanelSide`)
named in **egui's** vocabulary, exactly one per point at which an application can
attach an `egui::Frame`. `RoleVariant` (`Normal`, `Selected`, `Disabled`) as a
*value* parameter, not a taxonomy.

**Rejected:** one enum covering both, which is what an earlier proposal shipped.
A single enum mixing native widgets, value splits and egui surfaces tracks both
moving sides at once, so *either* upstream forces a variant change. Two enums,
each tracking exactly one side, is what makes §5.1 and §5.2 mechanical.

**Why the vocabularies are deliberately different.** It is tempting to spell them
alike — `Surface::Sidebar` beside `Role::Sidebar`. Resisting that is the point:
the mismatch is a permanent, visible reminder that the two enums answer to two
different authorities. The panel-side-to-native-struct mapping is therefore
written down as a convention rather than inferred from a name
(`PanelSide::Left`/`Right` ← `theme.sidebar`, `Top` ← `theme.toolbar`,
`Bottom` ← `theme.status_bar`, `Surface::CentralPanel` ← `theme.defaults`).

**Why `PanelSide` is exhaustive** while everything else is not: four sides is a
closed fact of 2-D screen geometry, not an API taxonomy that can churn, and
applications benefit permanently from being able to `match` it. Recorded as a
judgement call in specification §16 Q-4.

### 3.3 Three seams and a `Frame` carrier; why styles are pre-built

**Chosen.** All three of egui's substitution points, plus `egui::Frame` values.

Seam S1 (`Context::set_style_of`, `context.rs:2247`) is the global base. Seam S2
(`UiBuilder::style` / `Ui::set_style`) reaches everything laid out inside a `Ui`
**except** `Area`-based containers. Seam S3 (`egui::style::StyleModifier`) is the
only thing that reaches `Area`-based containers' *bodies*.

"All three" is an exhaustiveness claim, so it carries its own evidence rather
than being asserted: a grep for `StyleModifier` across every vendored egui crate
returns **24** hits, all of them in `egui/src/style.rs` and
`egui/src/containers/{popup,menu,combo_box}.rs`; and `Ui::new` — the one
constructor that can seed a style from nothing — has exactly **two** call sites
in the whole tree (`egui/src/context.rs:801`,
`egui/src/containers/area.rs:629`), neither of which supplies a style, which is
what makes §1.4's universal `Area` statement checkable rather than merely
plausible.

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
would tempt someone into it (`ThemeAtlas::role_style_variant`), rather than only
in a limitations section.

**Why pre-built rather than computed per call.** Handing a prepared
`Arc<Style>` to `UiBuilder::style` **moves** the `Arc` — `ui.rs:236` is
`let style = style.unwrap_or_else(|| Arc::clone(&self.style));` — so no `Style` is
copied into the child `Ui`. The caller still pays one refcount bump to produce
the `Arc`; the claim "not even a refcount bump", which appeared in two rejected
drafts, is false and the documents state only the true half. Building on demand
would instead clone a `Style` (including its `BTreeMap`) at every scope, every
frame, in an immediate-mode loop.

**Why `Style` is never built by struct literal.** `Style::debug` is
`#[cfg(debug_assertions)]` (`style.rs:322-323`), so an exhaustive literal would
fail to compile in exactly one of the two profiles. Construction always starts
from `egui::Theme::default_style()` and assigns. That construction *is* the
no-hardcoded-values enforcement: every field the theme does not supply keeps
egui's own value by construction, so a forgotten mapping degrades to stock egui
rather than to a literal somebody typed.

### 3.4 Install into both colour schemes, always, on every start

**Chosen.** `ctx.set_style_of(egui::Theme::Dark, ..)` **and**
`set_style_of(egui::Theme::Light, ..)` (`context.rs:2247`), unconditionally.

**Rejected:** `set_global_style` (`:2197`) and `set_visuals` (`:2277`) — both
touch only the *active* theme, leaving the other stock, so an OS light/dark flip
would drop the application into unthemed egui. Also rejected: `set_visuals_of`
(`:2264`), which would discard `spacing` and `text_styles` — i.e. every geometry
value the connector exists to deliver.

**Why on every application start.** `Options::dark_style` and `light_style` are
`#[serde(skip)]` (`egui/src/memory/mod.rs:195`, `:199`), so a persisted `Memory`
never restores them. An installer that assumed persistence would work in
development and fail on the second launch of a shipped binary.

This is also why `SystemThemeExt::to_egui_atlas` returns one atlas carrying both
variants, deviating from `to_iced_theme` (`connectors/native-theme-iced/src/lib.rs:197`)
and `to_gpui_theme` (`connectors/native-theme-gpui/src/lib.rs:263`), which each
return a single toolkit theme. egui stores one `Style` per `egui::Theme` and
flips between them from OS input every pass; returning one variant would
guarantee a half-themed application.

### 3.5 The connector does not own `Options::theme_preference`

**Chosen.** `install_with` never touches it. Two explicit free functions instead:
`follow_os_color_scheme(ctx)` and `pin_color_scheme(ctx, theme)`.

**Rejected:** an `InstallOptions::theme_preference` defaulting to
`Some(ThemePreference::System)`. `Options::theme_preference` **is**
serde-persisted (`egui/src/memory/mod.rs:206` has no `serde(skip)`), so that
default would silently discard the user's in-app Light/Dark choice on every
launch. A theme installer overwriting a user preference is the worst class of
surprise a library can ship.

`follow_os_color_scheme` also exists to document a real trap:
`Context::set_theme` takes `impl Into<ThemePreference>` (`context.rs:2167`) and
`From<Theme> for ThemePreference` exists (`memory/theme.rs:79-86`), so
`ctx.set_theme(egui::Theme::Dark)` *pins* dark rather than following the OS.
That is an easy mistake to make and an invisible one to debug.

### 3.6 The five-state derivation, and why the base states borrow from `theme.button`

**Chosen.** `noninteractive` ← the role's non-interactive colours; `inactive` ←
its resting interactive colours; `hovered` ← its `hover_*`; `active` ← its
`active_*` or, where absent, a field-wise copy of `hovered`; `open` written
**only** where egui actually reads it. On the *base* style the interactive three
come from `theme.button`.

**Why `open` is special.** `WidgetState` has four variants
(`widget_style.rs:84-90`), so `Widgets::state` (`:94-99`) and `Widgets::style`
(`style.rs:1272-1281`) can never return `open`. Every use of it in egui is a
direct field read: `containers/window.rs:1427`, `containers/combo_box.rs:371` and
`:450`, `widgets/color_picker.rs:117`, plus `menu_style`'s write at
`containers/menu.rs:25` (`menu_style` is `:22-29` in its entirety), and
`SubMenuButton::ui`'s read-then-copy-into-`inactive` at
`containers/menu.rs:382-386` — which is neither inside `menu_style` nor a write
to `widgets.open`. The mapping matrices
treated `open` as part of the interaction-state axis; it is not, and the
specification corrects them.

**Why `theme.button` and not `defaults`.** `ResolvedDefaults` has 31 fields and
**not one of them is a hover or pressed value** (`resolved.rs:71-145`). Every
hover colour in the model lives on a widget. `Button` is by a wide margin egui's
most common interactive widget: `ui.button` (`ui.rs:1847`), `ui.toggle_value`
(`:1874`), `ui.selectable_label` (`:1928`) and `ui.selectable_value` (`:1938`)
are all `Button`s, and so is every menu entry — `MenuButton` holds a `Button`
(`containers/menu.rs:291`, constructed `:297`) and so does `SubMenuButton`
(`:338`, `:347`). Borrowing from it is the only
non-inventing choice, and the specification calls it a stated borrowing rather
than dressing it up as a derivation.

**Rejected by name, so they are not re-proposed.** Every "obvious" way to
synthesise a hover colour needs a constant that exists nowhere in
`ResolvedTheme`: lightening or darkening by a factor; `Color32::gamma_multiply(k)`
(`ecolor/src/color32.rs:294`, whose `k` outside `0.0..` additionally trips a
`debug_assert` at `:295-298`); `ecolor::tint_color_towards` at some ratio;
blending toward `accent_color` by some weight. A specification that writes
"derive hover by lightening 8 %" has invented a platform value.

The same principle governs `soft_option` fallbacks: a `None` there is **not**
missing data, it is the platform asserting the widget has no distinct appearance
in that state. The correct derivation is therefore always a **copy** — no
arithmetic, no division, no narrowing, no `NaN` path — and every chain in
specification §6.4 terminates on a required field in one step.

### 3.7 The focus ring is deliberately not written

**Chosen.** `focus_ring_color`, `focus_ring_width` and `focus_ring_offset` are
accessors only. The ring is not folded into `widgets.active.bg_stroke`.

**Rejected:** folding it in, which two of the three design proposals wanted and
which looks like free fidelity.

**The defeating fact.** egui makes focus and press the *same* state:
`response.is_pointer_button_down_on() || has_focus() || clicked()` selects
`active` (`widget_style.rs:107-109`, mirrored at `style.rs:1275`). Writing the
ring there therefore paints a focus ring on **every mouse press**, and
simultaneously displaces the role's real pressed border. A wrong ring on every
press is worse than no ring, and `focus_ring_offset` had no sink either way —
so the "fidelity" would have been two-thirds of a wrong feature.

`focus_ring_offset` is additionally the one accessor documented as legitimately
negative (adwaita −2.0, macOS −1.0, `native-theme/src/resolve/validate_helpers.rs:676-677`),
which is exactly the kind of value a well-meaning clamp destroys.

### 3.8 `RoleVariant::Disabled` and the `1.0` identity

**Chosen.** The `Disabled` cell writes the platform's `disabled_background` and
`disabled_text_color` into the **`inactive`** entry and sets
`Visuals::disabled_alpha = 1.0`.

**Why `inactive`.** A disabled widget lands in `WidgetState::Inactive`, because
every predicate `Response::widget_state` tests before its fall-through
(`widget_style.rs:105-115`) is false for it: `Flags::HOVERED` is set only inside
`if res.enabled()` (`egui/src/context.rs:1493-1497`), `Flags::CLICKED` only when
`enabled` (`:1522-1523`), hit testing strips `Sense::CLICK` and `Sense::DRAG`
from a disabled widget so it can never be the pressed one
(`egui/src/hit_test.rs:128-136`), and focus is surrendered because
`interested_in_focus = w.enabled && …` (`egui/src/context.rs:1253`, `:1271-1273`).
Nothing in `widget_style.rs` itself tests `enabled`. That is where the
platform's disabled colours have to go.

**Why `1.0`.** `Ui::disable` multiplies painter opacity by `disabled_alpha`
(`ui.rs:496-501`). Writing `1.0` makes the multiply the identity, so the
platform's own disabled colour survives instead of being faded a second time,
while interaction stays blocked. `1.0` is a **multiplicative identity, not a
theme value**, and it is one of exactly three numeric literals the whole mapping
is allowed to contain (specification §6.17).

**The rejected objection, recorded because it was raised and answered.** One
proposal declined this on the grounds that it would compound into
`disabled_color.gamma_multiply(disabled_opacity)`. Setting the alpha to the
multiplicative identity is precisely what prevents that compounding.

**What the technique reaches, counted rather than estimated.** Specification
§6.3's `Disabled` cell writes exactly two native leaves per role —
`<role>.disabled_background` and `<role>.disabled_text_color` — so of the
**eighteen** UNMAPPABLE disabled-colour rows in specification §5 it reaches
**eleven**: the four `disabled_background` leaves (button, input, checkbox,
combo_box — `native-theme/src/model/widgets/mod.rs:81`, `:130`, `:175`, `:745`)
and the seven per-widget `disabled_text_color` leaves (button, input, checkbox,
menu, list, combo_box, link — `:75`, `:121`, `:169`, `:225`, `:521`, `:739`,
`:862`). One of those eleven is **inert**: `Link::ui` reads
`visuals.hyperlink_color` unconditionally and paints the galley with it
(`egui/src/widgets/hyperlink.rs:47`, used at `:58` and `:62`), so a
`Role::Link` scope's `fg_stroke.color` never shows. **Ten** are effective. The
other seven stay lost for the reasons their own rows already give:
`switch.disabled_checked_background` / `disabled_unchecked_background` /
`disabled_thumb_color` (`:627`, `:630`, `:633`) and `slider.disabled_fill_color`
/ `disabled_track_color` / `disabled_thumb_color` (`:327`, `:330`, `:333`) are
not field shapes §6.3 writes at all, and `defaults.disabled_text_color` has no
`Role` to be scoped by — `defaults` is not one of the 25 widget fields of
`ResolvedTheme`, so there is no `Role::Defaults` (specification §4.4). Their
matrix verdict deliberately stays UNMAPPABLE, because the route exists **only
for widgets the application scopes**, which is ledger item 6 and is stated as a
limit, not sold as a win.

**Why `Selected` and `Disabled` cannot be combined.** The only native data for
the combination is `switch.disabled_checked_background` /
`disabled_unchecked_background`, and egui has no switch widget at all; and for a
selected `Button`, `Style::button_style` overwrites `weak_bg_fill`, `bg_fill` and
`fg_stroke` from `visuals.selection.*` **regardless of the interaction state**
(`widget_style.rs:147`, `:150-155`), so the combination is not expressible.
`RoleVariant` is `#[non_exhaustive]`, so adding it later is additive if egui ever
makes it expressible.

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

**Why `.round()` and not `round_ties_even`**: so that a value this crate converts
and a value epaint converts through its own `From` impl agree bit for bit.

**Why this is not defensive programming.** `ResolvedTheme` derives `Deserialize`
with public fields (`resolved.rs:154-155`), native-theme does not range-check
per-widget `border.*` numbers (`native-theme-derive/src/gen_ranges.rs:117-118`;
`check_defaults_ranges` covers `defaults.*` only,
`native-theme/src/resolve/validate_helpers.rs:652-710`), and `card` is missing
from the `check_ranges` dispatch list altogether
(`native-theme/src/resolve/validate.rs:168-191`). A `NaN`, negative or `1e30`
radius can reach the connector in production.

**The one genuine panic path**, and why it justifies a dedicated helper:
`Visuals::disabled_alpha` (`style.rs:1125`) is forwarded by `Visuals::disable`
(`:1176-1179`) to `Color32::gamma_multiply`, which carries
`debug_assert!(0.0 <= factor && factor.is_finite(), ..)`
(`ecolor/src/color32.rs:295-298`). A bad value there would abort every downstream
user's `cargo test` and `cargo run` — far worse than a wrong pixel. Everything
reaching that field passes through `unit_interval()`.

Two precisions that were wrong in rejected drafts and are corrected here:
`Ui::disable` **cannot** fire that assert, because it calls
`Painter::multiply_opacity`, which is
`if opacity.is_finite() { self.opacity_factor *= opacity.clamp(0.0, 1.0); }`
(`painter.rs:100-104`) — non-finite dropped, value clamped. The **only** caller
of `Visuals::disable` in all of egui is `Ui::dnd_drop_zone` (`ui.rs:2725-2726`),
so the test that proves the invariant must exercise `dnd_drop_zone` specifically
(specification §13 T6).

### 3.10 Colour space: one route, and it is `const`

**Chosen.** `Color32::from_rgba_unmultiplied_const` (`ecolor/src/color32.rs:164`).
It is `const`, and it premultiplies in **gamma** space, applying no sRGB
transfer function — which is the property that matters, because
`native_theme::color::Rgba` is already gamma-encoded. `a == 255` short-circuits
to an exact `from_rgb` (`:170`) and `a == 0` to `TRANSPARENT` (`:167`); for
`1..=254` it computes `fast_round(channel as f32 * linear_f32_from_linear_u8(a))`
per channel (`:172-176`, helpers at `ecolor/src/lib.rs:108` and `:133`). Floats
*do* enter there and it *does* round — the identical expression epaint's
non-`const` `from_rgba_unmultiplied` tabulates (`:142-156`), so the two agree bit
for bit and the forward conversion is lossy at low alpha in exactly that one
place.

**Rejected:** `Color32::from_rgba_premultiplied` (`:122`), which stores straight
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
(`:270`) is lossy there too. That is stated rather than assumed.

### 3.11 Fonts: never emit a `FontFamily::Name`; one weight per family

**Chosen.** Every `FontId` this crate produces names `FontFamily::Proportional`
or `FontFamily::Monospace`, and `fonts::font_definitions` only ever *prepends
into those two existing chains*.

**Rejected:** registering a custom `Name` family per `(family, weight, style)`
triple — which two of the six mapping matrices assumed, and which would have
carried 18 more leaves (specification §5.8 item 2).

**Why the invariant wins anyway.** Two epaint panics fire from inside
`Context::begin_pass` with no recovery point:
`FontFamily::{family:?} is not bound to any fonts` (`epaint/src/text/fonts.rs:1031`)
and `No font data found for {font_name:?}` (`:1039`). They exist because `Style`
and `FontDefinitions` land through **different channels**: `set_style_of` takes
effect immediately (`context.rs:2247`) while `set_fonts` is deferred to the next
pass (`context.rs:2101-2103` — `:2100` is a blank doc line). Any design that emits a `Name` must guarantee an
ordering across two channels, and a violated guarantee is a panic in the *user's*
application. The invariant makes both unreachable by construction — no pass
counter, no promotion gate, no documented precondition whose violation panics.

The rejected alternative had a concrete mechanism, and that mechanism does not
work: `Plugin::on_begin_pass(&mut Ui)` (`egui/src/plugin.rs:26-27`) receives a
`Ui` that has already snapshotted its `Arc<Style>` (`ui.rs:135`), so it cannot
retroactively restyle the pass it runs in.

**Two further wins.** The emoji fallback tail — `NotoEmoji-Regular` and
`emoji-icon-font` — that `FontDefinitions::default()` installs
(`epaint/src/text/fonts.rs:540-556`) survives, which a custom `Name` family would
lose. (There is no CJK tail to preserve: egui ships four faces and its own
documentation says "The default `egui` fonts only support latin and cyrillic
alphabets", `egui/src/context.rs:2098`.) And the headless test
technique of specification §13 T7 — `ctx.set_fonts(egui::FontDefinitions::empty())`,
which saves a face parse per test — is valid **only** because of this invariant.

**The price, stated as a price.** Exactly two families, therefore exactly one
weight per family. `FontId` is `{size, family}` with upstream's own
`// TODO(emilk): weight (bold), italics, …` at `epaint/src/text/fonts.rs:33`
(struct `:27-34`), and family is its only selector. Bold headings against a
regular body are not carried by the theme. Weight is applied as a `wght`
variation coordinate through `FontTweak::coords` (`fonts.rs:256`), with the tag
built by `Tag::new(b"wght")` (`font-types/src/tag.rs:30-32`, an infallible
`const fn`) and **never** through the `&str` or `[u8; 4]` `IntoTag` impls, which
`expect` (`epaint/src/text/text_layout_types.rs:396`, `:403`, `:410`) and are
banned by the no-panic rule.

**Why `fonts::supports_weight_axis` is a public function** rather than a sentence
in a doc comment: "is this face actually variable" is the question every
application will ask, and the honest answer has a subtlety worth making testable
— it returns `false` for *both* "static font" and "unparseable bytes", because
`FontData::variation_axes` early-returns an empty `Vec` for both
(`epaint/src/text/fonts.rs:159-179`) and the two are not distinguishable through
any public epaint API.

**Why the crate never reads a file and never parses a face.** `FontsImpl::new`
parses every registered face eagerly and **panics** on a parse failure
(`epaint/src/text/fonts.rs:996`) from inside `begin_pass`. A pre-flight check
would need the exact `skrifa` call epaint makes (`epaint/src/text/font.rs:387-388`),
and epaint does not re-export `skrifa`. The application owns the bytes and owns
their validity.

**Why family names cannot be resolved at all.** The only way a face enters epaint
is a byte buffer: `FontData { font: Cow<'static, [u8]>, .. }`
(`epaint/src/text/fonts.rs:118-128`). The `String` keys in `FontDefinitions`
(`:437-450`) are arbitrary caller labels that nothing looks up in a system font
database, and neither epaint nor egui nor eframe depends on `fontdb`, `font-kit`,
`fontconfig`, `core-text` or DirectWrite. (The only `load_system_fonts()` in the
tree is `egui_extras/src/loaders/svg_loader.rs:41`, feeding **resvg's** fontdb for
text inside SVG images — no connection to egui's text layout.) Choosing a font
discovery crate would add a dependency whose failure modes and licence terms
belong to the application, so the honest out-of-the-box description is **"right
metrics, wrong typeface"**, and the crate says so.

### 3.12 Line height: a begin-pass plugin, not a protocol

**Chosen.** `Spacing::extra_text_line_spacing` is recomputed every pass by an
`egui::Plugin` registered by `install_with`; the pure formula is public as
`extra_text_line_spacing(&egui::Ui, &ResolvedTheme)`.

**Rejected:** computing it at install time, and a
`refresh_line_spacing(&Ui) -> bool` the caller must remember to call.

**Why not at install time.** `Context::fonts_mut` **panics** before the first
pass (`context.rs:1113-1121`, `expect("No fonts available until first call to
Context::run()")`). native-theme's `defaults.line_height` is a dimensionless
multiplier (`resolved.rs:75-76`) while egui's field is an absolute additive delta
in points, so the conversion needs the loaded font's row height — a runtime
property. Taking `&egui::Ui` is a **strong convention rather than a type-level
proof**: `Ui::new` is public (`egui/src/ui.rs:108`), so a caller can build one
outside a pass, and there is no fallible font accessor to fall back on —
`Context::fonts` carries the same `expect` as `fonts_mut`
(`context.rs:1096-1106`). The begin-pass plugin path that specification §6.15
actually uses *is* airtight: `run_ui_dyn` builds the root `Ui` and invokes
`on_begin_pass` only from inside `run_dyn` (`context.rs:798-818`, plugin call at
`:810`), which runs after `Fonts` has been constructed (`:590`). The public
helper therefore documents the panic rather than denying it.

**Why not a caller protocol.** A two-phase protocol on the common path fails
silently when forgotten: text simply has the wrong leading and nothing reports
it. Plugin registration is idempotent — "a plugin of the same type can only be
added once" (`context.rs:2041-2042`) — so installing twice registers one plugin,
and running every pass means it self-corrects on font and zoom changes. The cost
is at most one pass of egui's `0.0` at start-up, and there is no API the caller
can forget.

**What the plugin can and cannot reach, because the boundary is not obvious.**
It writes `ctx.all_styles_mut(..)` (`context.rs:2210`), i.e. the two **base**
styles. It cannot reach the atlas's per-`(Role, RoleVariant)` `Arc<Style>`
cells: those are compiled at `Builder::build`, where there is no pass and
`Context::fonts_mut` would panic, and a published `Arc<Style>` cannot be patched
afterwards. So a `Label` or `TextEdit` inside `native_scope` keeps egui's `0.0`
while unscoped text gets the theme's leading. That is ledger item 29, recorded
as a loss rather than smoothed over; the connector-side cure — republishing a
patched atlas from the plugin when the computed value changes, which costs one
rebuild per font or zoom change and none per frame — is recorded with it.

**The three losses are stated, not buried.** One global `f32` cannot be
simultaneously correct for `Small`, `Body`, `Button`, `Heading` and `Monospace`;
the delta is exact for `Body` only. It is read at exactly two functional sites —
`WidgetText::into_galley_impl`'s `Self::Text` arm (`widget_text.rs:775-776`) and
`TextEdit` (`widgets/text_edit/builder.rs:473-474`). Consequently `ui.label("x")`
honours it but `ui.label(RichText::new("x"))` does not, because the `RichText`
arm (`:791`) delegates to `RichText::into_layout_job` (`:384-392`), which
propagates only `RichText`'s own `line_height` and never reads `style.spacing`.

The four per-role line heights *are* exact and lossless, through
`text_role_line_height()` + `RichText::line_height(Some(..))` (`widget_text.rs:174`)
— the only exact per-role mechanism egui has.

**One UNVERIFIED item lives here** and is carried through to the open questions:
whether a **negative** `extra_text_line_spacing` is safe. It is undocumented and
unasserted in 0.36.1 — egui clamps only its own settings slider to `0.0..=20.0`
(`style.rs:2023`), and no use site enforces a range. *What would verify it*: a
documented range or a `debug_assert` on the field. Until then the connector
clamps to `>= 0.0` itself.

### 3.13 Icons: the boundary is decode, not draw

**Chosen.** The connector does key and URI construction, `IconData` →
`ImageSource` / `Image`, alt text, animation scheduling and cache invalidation.
It does **no** decoding, **no** rasterising, and **no** loader installation.

**Why the boundary sits there.** egui core ships **zero** image decoders:
`Loaders::default()` starts with an empty image-loader vector
(`egui/src/load.rs:613`), so `try_load_image` returns `LoadError::NoImageLoaders`
(`egui/src/context.rs:3863-3865`) until the application calls
`egui_extras::install_image_loaders(&ctx)` (`egui_extras/src/loaders.rs:58`).

**Rejected:** depending on `egui_extras`, and enabling `native-theme/svg-rasterize`
by default the way the gpui connector does
(`connectors/native-theme-gpui/Cargo.toml:23`). `egui_extras`'s `svg` feature
pulls `resvg 0.45.1` while `native-theme`'s `svg-rasterize` pulls `resvg 0.47` —
two semver-incompatible pre-1.0 minors — so enabling both compiles two copies of
resvg, usvg and tiny-skia into one binary. Letting `egui_extras` own
rasterisation additionally gets DPI-correct re-rasterisation for free
(`Image::load_for_size`, `egui/src/widgets/image.rs:349-354`). In-connector
rasterisation remains an **additive** opt-in under `svg-rasterize`, never a
silent switch.

**Why the URI carries everything that can change a pixel.** Every egui loader
layer caches on the URI *string* (`egui/src/load/bytes_loader.rs:15-26`,
`egui/src/load/texture_loader.rs:49-59`), so two renderings sharing a URI would
see the first served forever. `egui::Image::tint` is deliberately excluded: it is
a draw-time multiply that does not change the texture.

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
fields — `icon_width` (`style.rs:427`), `icon_width_inner` (`:431`),
`icon_spacing` (`:435`) — are *control* geometry: the checkbox box, the check
mark, and the default gap between **all** atoms in **every** `AtomLayout`
(`atomics/atom_layout.rs:302`). Writing an icon size into any of them would
resize every checkbox and radio button from an icon metric.

### 3.14 Watch and repaint: egui gets a real cross-thread wake

**Chosen.** A `watch` feature; `ThemeWatcher::start(&Context, FontPlan)`; the
watcher thread does re-detection, re-resolution **and** atlas construction, then
calls `Context::request_repaint`; installation happens on the UI thread via
`take()`.

**Rejected:** the sibling showcases' 500 ms polling flag, and installing from the
watcher thread.

**Why polling is not needed here.** `egui::Context` is
`Clone + Send + Sync + 'static` (`context.rs:721-722`, with upstream's own
compile-time assertion at `:4266-4269`) and `Context::request_repaint` (`:1818`)
documents that a call from outside the UI thread wakes it, provided the
integration installed a repaint callback — which eframe does on all three
backends. The iced and gpui connectors poll because their runtimes offer no such
wake-up, not because polling is good. There is no timer and no interval to tune.

**Why the whole rebuild happens off-thread.** Atlas construction is pure CPU,
needs no `Context`, and `ThemeAtlas` is `Send + Sync + 'static`, so D-Bus,
registry and `CFRunLoop` work all stays off the UI thread.

**Why installation does not.** `Context::set_style_of` takes `&self` and would
compile from the watcher thread — but each `Ui` snapshots its `Arc<Style>`
exactly once (`ui.rs:135`, `:236`) and never re-reads it, so a mid-pass swap can
leave two halves of one frame in two different themes. `take()` is the hand-off,
and it is non-blocking and `None` on the overwhelming majority of frames.

`install_with` calls `icons::forget_icons(ctx)` by default so a theme change
re-renders recoloured icons rather than serving the previous theme's cached
textures, using `Context::forget_image` (`context.rs:3761`) per URI so unrelated
application images are left alone.

### 3.15 Re-exports: two crates in, three names deliberately out

**Chosen.** `pub use egui;` **and** `pub use native_theme;`, plus convenience
re-exports of the non-colliding native-theme names.

**Why both crates.** Two egui versions in one graph produce
`expected egui::Style, found egui::Style`, which is the single most common
downstream failure with a toolkit connector. Two lines of manifest make it
unrepresentable. This is a documented, argued deviation from both sibling
connectors.

**The rule that follows, and the three names it excludes.** The crate root
re-exports no name that also exists at `egui`'s root:

* `native_theme::color::Rgba` → `convert::Rgba` — opposite colour space to
  `egui::Rgba` (`egui/src/lib.rs:442`), see §3.10;
* `native_theme::theme::IconData` → `icons::IconData` — `egui::IconData`
  (`egui/src/viewport.rs:183`) is the window/taskbar icon;
* `native_theme::theme::Theme` — `egui/src/lib.rs:483` already exports `Theme`;
  it stays reachable as `native_theme::theme::Theme`.

Each excluded name is still reachable in one hop from a module whose name says
what it is for. This is the same rule that produced the handle's name (§2.8).

### 3.16 Features: `default = []`

**Chosen.** Five features, every one a 1:1 pure passthrough to a `native-theme`
feature name, with an empty default.

**Rejected:** the gpui connector's
`default = ["material-icons", "lucide-icons", "system-icons", "svg-rasterize"]`
(`connectors/native-theme-gpui/Cargo.toml:23`). A theme connector must not force
bundled icon blobs or an SVG rasteriser on every downstream binary; the iced
connector, which has no features at all
(`connectors/native-theme-iced/Cargo.toml`), is the closer precedent. §3.13 is
the additional, egui-specific reason `svg-rasterize` must never be on by default
here.

No feature name is invented at this layer, so a native-theme feature rename is a
one-line change here and never a semantic divergence.

### 3.17 Semver and the egui version policy

**Chosen.** `egui = "0.36.1"` — a caret requirement, `>=0.36.1, <0.37.0`. One
egui minor per release line; an egui minor bump is a **breaking change** for this
crate. Crate version equals the workspace version. The same shape as
`iced_core = "0.14"` (`connectors/native-theme-iced/Cargo.toml:24`) and
`gpui = "0.2.2"` (`connectors/native-theme-gpui/Cargo.toml:30`).

**Rejected:** an exact `=0.36.1` pin (it would reject 0.36.2 patch fixes) and any
wider range (cargo cannot unify two semver-incompatible egui versions, and the
application's `egui::Style` must be *our* `egui::Style`).

**The clause that matters most is clause 3: the numeric contents of a produced
`egui::Style` are not covered.** Electing a different base owner, correcting a
sink, closing a `Note` — none of those is a breaking change. Without that clause
every future mapping correction would need a major version, and the mapping
*will* be corrected: §7 lists eleven corrections made before a line of code was
written.

**Clause 4 — `#[non_exhaustive]` on every public enum except `PanelSide`** —
exists because native-theme churn is an axis independent of egui churn. §5.2 is
the worked example.

### 3.18 MSRV: a measured workspace floor of 1.88.0, and 1.95 for this connector

egui 0.36.1 declares `edition = "2024"` (`egui/Cargo.toml:13`) and
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
--offline --all-features` over the committed `Cargo.lock` reports **957
packages**, of which **577 declare a `rust-version`** (476 distinct package
names). The highest declared anywhere is **`1.88.0`**. Every package sitting at
that maximum is named here so the figure can be re-derived: `darling`,
`darling_core` and `darling_macro` 0.23.0; `image` 0.25.10; `time` 0.3.47 with
`time-core` 0.1.8 and `time-macros` 0.2.27; `home` 0.5.12; `iced` and
`iced_program` 0.14.0; `serde_with` and `serde_with_macros` 3.18.0; `wgpu`
27.0.1; and this workspace's own five members. Six of them — `home`, `iced`,
`iced_program`, `serde_with`, `serde_with_macros` and `wgpu` — spell it `1.88`
rather than `1.88.0`. Measured **2026-08-10**.

*The toolchain bound.* On the same date every workspace member was compiled
against that exact toolchain — `cargo +1.88.0 check -p <member> --all-targets
--locked` was clean for all five, tests included.

*Our own sources need `1.88.0` too.* An earlier draft of this section recorded
that a sweep for let-chain syntax found none in our own code; **that was
false**, and the trigger built on it is corrected below. Let-chains stabilised
in 1.88 and are available only under edition 2024 (`Cargo.toml:13`); a grep for
`&& let ` across `native-theme`, `native-theme-build`, `native-theme-derive` and
both existing connectors returns **59** occurrences — for example
`native-theme/src/detect.rs:245-247`,
`native-theme-derive/src/gen_ranges.rs:149-150`,
`native-theme/src/spinners.rs:78-79` and
`native-theme/src/resolve/inheritance.rs:46`. That grep undercounts, because it
does not see a chain whose second term is a bare boolean, such as
`native-theme/src/model/font.rs:309-310` or
`native-theme-derive/src/gen_ranges.rs:117-118` (the same lines §3.9 cites for
the missing range checks).

So the floor is **held up from both sides**: it is the lowest value the
dependencies promise to support *and* the lowest value our own code compiles
under. Lowering it would not merely outrun a dependency's promise — it would
fail to build `native-theme` itself.

**Rejected: pinning the workspace to current Rust stable** (`1.97.1` at the time
of writing). This was briefly adopted, on the argument that all six toolchain
installs in `.github/workflows/ci.yml` are `@stable` (lines 18, 39, 67, 77, 96,
108), there is no `rust-toolchain.toml`, and `pre-release-check.sh` has no MSRV
check — so any declared number was an untested claim, and one equal to what CI
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
hoping, which produces a confusing compile failure inside egui rather than a
clear `rust-version`-too-low error naming the package.

**The caveat that now carries the weight.** A measured floor decays the moment
any dependency raises its own requirement, and nothing currently re-checks it.
`1.88.0` is honest as of the measurement date and no longer than that. The MSRV
job in specification §12.4 is therefore not a nicety but the thing that keeps
this decision true; it remains task 22 of the implementation list.

### 3.19 Diagnostics: `Note`, because silent correctness is unobservable

**Chosen.** `ThemeAtlas::notes() -> &[Note]`, populated at build time, with four
`#[non_exhaustive]` variants: `ValueSanitised`, `ValueSaturated`,
`FontFamilyUnavailable`, `FontWeightAxisUnsupported`.

**Rejected:** silent substitution only. `convert` substituting for `NaN` and
`±∞` is correct behaviour, but it is *unobservable*: the only symptom is a
rendering anomaly nobody can trace. A non-fatal channel makes a sanitised or
saturated value show up in a log, and it costs one `Vec<Note>` on a type
constructed once per theme change.

The distinction between the first two variants is deliberate and load-bearing.
Corner-radius saturation at 255 is **benign** — the tessellator re-clamps to half
the smaller side (`epaint/src/tessellator.rs:638-642`), so 255 simply reads as
"pill" — and emits nothing. Margin saturation at ±127 is a **real** loss of theme
data and emits `ValueSaturated`. A diagnostic channel that cried wolf on the
benign case would be ignored on the real one.

### 3.20 `mapping.toml`, and why the coverage table is a test rather than a claim

**Chosen.** One checked-in row per native leaf, carrying `verdict`, the declared
egui `sinks`, and — for `unmappable` — the upstream change that would close it.
Rendered into the published rustdoc, not merely kept repo-local.

**Why a manifest at all.** It is the only mechanism in any of the candidate
designs that detects **native-theme** drift. An egui bump is caught by the
compile-time tripwires of specification §13 T2; a native-theme field addition is
caught by nothing else, and would otherwise appear as a leaf that quietly maps
nowhere.

**Why the differential test has no skip allowance.** T3 splices one native leaf
from preset B into preset A, rebuilds, and asserts that the set of changed egui
fields equals the row's declared `sinks`. A row that no preset pair differentiates
**fails**, with an actionable message, and must be given an explicit probe value.
There is deliberately no discretionary knob a maintainer can raise instead of
fixing a mapping — because the knob would be raised.

**Why the base-owner table is published.** A contested field always means
somebody loses, and the loser must be nameable by a reader who never opens the
manifest. Specification §5.9 is what lets a user predict what an *unscoped*
widget looks like; hiding it in a repo-local file would make the coverage numbers
a claim about a document instead of a description of the code.

### 3.21 Fields deliberately never written

Ten `Style` fields are listed in specification §5.10 as never written. The
principle behind the list: **writing an inert field is a silent no-op, which is
the "plausible fabrication" failure mode this project forbids** — the crate would
appear to theme something it does not.

`Spacing::menu_width` (`style.rs:452`), `Spacing::menu_spacing` (`:455`),
`Style::compact_menu_style` (`:340`) and `Visuals::window_highlight_topmost`
(`:1066`) have **no functional reader anywhere in egui 0.36.1**. Each appears
exactly four times in the crate: its declaration, its default (`:1470`, `:1471`,
`:1446`, `:1527`), a destructure inside egui's own settings UI, and the widget
that edits it there (`:1961`/`:2031`, `:1962`/`:2036`, `:1804`/`:1910`,
`:2301`/`:2475`). No paint or layout path consults any of them — which is the
precise claim, and it is stronger than "no occurrences" because the settings-UI
occurrences do exist. `Visuals::clip_rect_margin` (`:1085-1086`) is
`#[deprecated]` and documented "Setting it now has no effect". `Style::interaction` (all eight fields,
`:910-944`) is input-behaviour policy — hit-test slop, tooltip timing,
text-selection policy — and `ResolvedTheme` exposes none of it, so writing it
would mean inventing platform behaviour, not mapping it. `Visuals::striped`
(`:1099`) gates striping entirely, and turning it on because an alternate row
colour exists would be inventing platform policy from the presence of a colour.

`Spacing::default_area_size` (`:444`) is the sharpest case, because it *looks*
like the dialog-size sink: `dialog.max_width` / `max_height` map onto it
naturally. It is left alone because it is the first-frame size of **every** free
`Area` (`containers/area.rs:470-476`) — window, popup, menu, tooltip — so writing
it would resize all of them from a dialog metric. `dialog_max_size()` is an
accessor instead.

`Visuals::override_text_color` (`:1015`) is never written on the **base** style
for the same class of reason: it forces one colour on *all* text, destroying
every per-state and per-widget text colour, and it would turn a `ProgressBar`
label into ordinary body text on an accent fill (`widgets/progress_bar.rs:197-199`).
It **is** written in the `Role::Checkbox` scope, and only there, because
`Style::checkbox_style` takes the label colour from `ws.text` (which honours it,
`widget_style.rs:133-136` → `:187`) and the check-mark stroke from `ws.stroke`
(= `fg_stroke`, `:188`). That is the only exact way to separate
`checkbox.font.color` from `checkbox.indicator_color`.

### 3.22 The no-widgets charter

Recorded in specification §14.3 as a two-test predicate rather than an opinion:
a widget may be added only if egui 0.36.1 has no widget with that visual identity
**and** a majority of the corresponding `ResolvedTheme` struct's leaves are
otherwise UNMAPPABLE. `Switch` passes both today and is still declined (§2,
Option B).

The charter exists in this form so that the question is **closed**. An opinion
gets re-litigated every release; a predicate with a worked negative example does
not.

---

## 4 -- Why each UNMAPPABLE class is genuinely unmappable

150 leaves have no expression. They are not one problem; they are six, and the
distinction matters because the fix for each lives somewhere different — and
**41 of the 150 are not egui's fault alone** (§4.3): 38 are `source-void` and 3
are `source-side gap`. The remaining 109 are `egui-limited`
(specification §5.7).

### 4.1 The six causes

**Cause 1 — no widget-type axis (ledger item 1).** The root cause, proved in
§1.2–§1.3. It does not make leaves UNMAPPABLE by itself — it makes them SCOPED,
which is why 209 leaves are in that bucket rather than lost. What it makes
genuinely unmappable are the **intra-widget** contests, where both claimants live
in the same widget and no scoping mechanism can help (ledger item 20): three of
them, resolved by naming a winner and exposing or dropping the loser
(specification §5.11).

**Cause 2 — `Area` containers ignore the calling `Ui` (ledger items 2, 3).**
`Area::Prepared::content_ui` builds with a bare `UiBuilder::new()`
(`containers/area.rs:611-629`). The specific casualty is the *idiomatic* tooltip:
`Tooltip::for_widget` reads `response.ctx.global_style()` at
`containers/tooltip.rs:43` and accepts neither a `Frame` nor a `StyleModifier`,
so `Response::on_hover_text` — the call everyone writes — is unthemable. There is
an escape hatch today, and it is documented rather than hidden: `Tooltip::popup`
is a **public field** (`tooltip.rs:9`), so
`Tooltip::for_widget(&r).popup.frame(f).style(m)` works on the manual path.

What a scope *around* the call cannot do, a scope **inside the closure** can,
and both documents must say so or ledger item 2 reads as a total loss when it is
partial. `Window::show`, `Modal::show` and `Popup::show` all hand the
application a `Ui` that descends from the `Area`'s content `Ui`
(`containers/window.rs:710`, used at `:719-752`; `containers/modal.rs:104-108`;
`containers/popup.rs:508`, body at `:600-603`),
and `Ui::set_style` (`ui.rs:386`, doc at `:383`: "Changes apply to this `Ui` and
its subsequent children") replaces the style for that `Ui` and everything built
from it (`ui.rs:236`). A role scope applied as the *first statement inside the
closure* therefore reaches every widget the application adds there. It does
**not** reach the chrome computed before the closure runs, nor the parts the
container paints itself; specification §14 item 2 records that residue. The
consequence is a cost reduction, not a reach increase: opt-in fidelity inside a
`Window` costs one line of application code rather than an upstream change — and
one line of application code is still application cooperation, which is exactly
what reading (2) of §1.6 says is unavoidable.

**Cause 3 — the field simply does not exist.** No amount of cleverness invents a
slot. `Visuals` models exactly two status colours, `warn_fg_color`
(`style.rs:1055`) and `error_fg_color` (`:1058`), so success and info have
nowhere to go (item 15). Nothing in `Visuals`' 36 fields (`:988-1125`) or the
five `Widgets` entries (`:1254-1268`) means "the window lost focus", and
`widgets.open` means the *active* window title bar (`containers/window.rs:1427`)
— the opposite (item 16). There is one `Visuals::hyperlink_color` (`:1035`) and
no visited state (item 14). No focus outline exists anywhere (item 5). No switch
or toggle module exists under `egui/src/widgets/` at all (item 13).

**Cause 4 — the value is hardcoded in paint code.** `separator_style` hardcodes
`spacing: 6.0` (`widget_style.rs:215`), overridable only per instance via
`Separator::spacing` (`widgets/separator.rs:45`) — item 4. Five `Frame` presets
hardcode their inner margins (`containers/frame.rs:180`, `:187`, `:192`, `:229`,
and `:236-237`, where `dark_canvas` delegates to `canvas`) — item 18. `Spinner`'s radius inset, point count and `Stroke::new(3.0, ..)` are
hardcoded (`widgets/spinner.rs:45`, `:58`) — item 12. `expander.arrow_color` is
dropped because `paint_default_icon` fills the arrow with
`visuals.fg_stroke.color` (`containers/collapsing_header.rs:353`) and the label
uses `visuals.text_color()` (`:598`), which *is* the same field — item 25, and it
is not hypothetical: `macos-sonoma.toml:308` gives the arrow `#86868b` against
`#1d1d1f` text, and `windows-11.toml:329` uses a semi-transparent `#1a1a1ae0`.

**Cause 5 — the type is too narrow (item 19).** `Margin` is four `i8`
(`epaint/src/margin.rs:15-20`), `CornerRadius` four `u8`
(`corner_radius.rs:13-25`), `Shadow::offset` `[i8; 2]` with `blur` and `spread`
`u8` (`shadow.rs:15`, `:20`, `:23`). Sub-point precision and large magnitudes are
lost at the boundary. This is the one cause where the loss is *quantified* rather
than total, and where the connector reports it (`Note::ValueSaturated`).

**Cause 6 — `FontId` has no weight or slant, and egui has no font database
(items 7, 8).** §3.11. Twenty-one family slots, one selector.

### 4.2 The upstream contribution to egui

Framed per the house guidance for upstream PRs recorded in
`docs/todo.md:51-67`: **more theming flexibility, not "native platform look"**;
**no API breaking changes**; **one concern per PR**; **concrete benefit shown**;
follow the project's own contribution guide, including disclosure of
AI-generated code.

`egui/src/widget_style.rs` is **byte-identical** between 0.35.0 and 0.36.1,
verified by `diff`. A frozen module is the best possible target for a well-argued
PR, because nothing in flight competes with it.

* **Commit 1 — widgets declare their kind.** Add `BUTTON_CLASS`,
  `CHECKBOX_CLASS`, `SEPARATOR_CLASS`, `LABEL_CLASS` and `CHECKED_CLASS` beside
  the existing `ROOT_CLASS` (`widget_style.rs:222`) and `SELECTED_CLASS` (`:225`),
  and have `Button`, `Checkbox` and `Separator` add their own class where
  `Button` already adds `SELECTED_CLASS` (`widgets/button.rs:329`).
  **Behaviour-neutral**, no signature changes, one concern.
* **Commit 2 — `Style` gains class overrides.**
  `pub class_overrides: Arc<[(ClassName, StyleModifier)]>` (`#[serde(skip)]`,
  default empty), applied at the top of `Style::widget_style`
  (`widget_style.rs:120`) to a local copy of `self` for the classes present.
  **Behaviour-neutral when empty**; the `Arc<[_]>` keeps `Style: Clone` cheap and
  the whole thing `Send + Sync`. One concern, additive field only.
* **Commit 3 — plumb classes through the rest.** `separator_style` stops
  hardcoding `spacing: 6.0` (`:215`); `TextEdit`, `ComboBox`, `Slider`,
  `CollapsingHeader` and `ProgressBar` gain a `Classes` field and route through
  `widget_style` instead of `Style::interact`.

Commits 1 and 2 are perhaps 120 lines and are individually behaviour-preserving —
the shape of PR that lands. The concrete benefit is easy to demonstrate: with
them, this crate ships its atlas as `class_overrides` on a single `Style`,
`native_scope` becomes optional sugar, and **SCOPED collapses into DIRECT for
every widget egui itself paints**. That is the change that would make "full theme
geometry" a true claim.

Smaller, independently landable PRs in the same spirit, one concern each:

| PR | Closes | Shape |
|---|---|---|
| `Area::style` / a `style` on the `UiBuilder` in `Area::Prepared::content_ui` | items 2, 3 | additive builder method |
| `Tooltip::style` / `Tooltip::frame`, or `Response::on_hover_text_styled` | item 3 | additive builder method |
| `Visuals::focus_stroke` + `focus_offset`, painted by `AtomLayout` | item 5 | additive `Visuals` fields, default = today's behaviour |
| a sixth `Widgets` entry, or `WidgetState::Disabled` | item 6 | additive; `WidgetState` is not `#[non_exhaustive]` today, so this one is **not** breaking-change-free and needs care |
| `weight` (and `slant`) on `FontId` | item 7 | the largest of these; upstream already carries the `TODO` (`epaint/src/text/fonts.rs:33`) |
| apply `extra_text_line_spacing` in `RichText::into_layout_job`, or add a multiplicative `line_height_factor` | item 9 | one concern, visible benefit |
| `Spacing::slider_handle_radius`; `Spinner::stroke_width` | items 11, 12 | additive |
| `Visuals::hyperlink_visited_color` + a visited set in `Memory` | item 14 | two concerns; split |
| a `Visuals::status` block | item 15 | additive |
| `Visuals::selection_inactive`, consulted when `ctx.input(\|i\| !i.focused)` | item 16 | additive |
| the five `Frame` presets reading `Spacing` | item 18 | behaviour-changing by definition; frame as "let themes reach panel margins", offer the fields as `Option` |
| `MarginF32` / `CornerRadiusF32` in `Style` | item 19 | large; lowest priority |
| wire or delete `menu_width`, `menu_spacing`, `compact_menu_style`, `window_highlight_topmost` | item 17 | trivially reviewable, good first contribution |

**Two honesty notes, both non-negotiable and both repeated from the
specification.** No upstream maintainer has been consulted and egui's appetite
for any of this is **UNVERIFIED**; *what would verify it* is an issue or
discussion on the egui repository, which is outside what this design work can
establish. And **the connector must not depend on any of it**: the design works
against egui 0.36.1 exactly as shipped, and every upstream change above would be
a simplification, never a prerequisite.

### 4.3 The changes owed to native-theme itself

Thirty-eight of the 150 UNMAPPABLE leaves are **`source-void`** — the *native*
leaf carries no information. These are not egui's fault and no egui PR closes
them.

* **36 leaves: widget-level `border.corner_radius_lg` and `border.opacity`**
  (18 widgets × 2). The resolver hardwires both to the constant `0.0` —
  `native-theme/src/resolve/validate_helpers.rs:276` and `:278` for the
  `BorderKind::None` arm, `:330` and `:332` for `Full`/`Partial`, `:50` and `:52`
  in the absent-border sentinel — and the function's own doc says so at
  `:258-259`. A connector that read `theme.window.border.corner_radius_lg` would
  get square window corners; one that folded `theme.button.border.opacity` into a
  stroke alpha would make **every** border in the theme invisible. The fix is
  native-theme-side: propagate `defaults.border.{corner_radius_lg, opacity}`, or
  remove the two fields from the widget-level `ResolvedBorderSpec`. Until then
  the prohibition is documented **on the conversion function itself**
  (`convert::to_color32_with_opacity`), not only in prose, because that is where
  the mistake would be made.
* **2 leaves: `defaults.border.padding_horizontal` / `padding_vertical`**
  (item 23). `DefaultsBorderSpec` has no padding fields by design
  (`native-theme/src/model/border.rs:12-18`) and the resolver hardcodes both to
  `0.0` (`validate_helpers.rs:584-585`). Reading them into
  `Spacing::button_padding` would inject a fabricated zero and flatten every
  `Button`, `ComboBox`, `CollapsingHeader` and `DragValue` at once.

Two further native-theme-side items are losses without being `source-void`:

* **Shadow geometry (item 10).** native-theme carries only `shadow_enabled: bool`
  (`native-theme/src/model/border.rs:106`) while `epaint::Shadow` needs an offset,
  a blur and a spread. The connector replaces **only the colour** and keeps
  egui's own geometry; populating offset/blur/spread would be invention. Adding
  them to `BorderSpec` is the fix. Three leaves are tagged for exactly this —
  `sidebar`, `toolbar` and `status_bar` `.border.shadow_enabled`, the third
  UNMAPPABLE sub-tag **`source-side gap`** of specification §2 — because there
  the only carrier is a connector-supplied `egui::Frame`, which has nowhere to
  fall back to. Those three are what make the split 38 + 3 + 109 rather than
  38 + 112.
* **`LayoutTheme` is unreachable from `SystemTheme` (item 21).** `LayoutTheme`
  lives on `native_theme::theme::Theme` (`native-theme/src/model/mod.rs:266`),
  not on `ResolvedTheme` (`resolved.rs:155-212`), and `SystemTheme`
  (`native-theme/src/lib.rs:369-423`) has no `layout` field either. So
  `from_preset` can supply `Spacing::item_spacing` and `Spacing::window_margin`
  and `from_system` cannot — and those are the two spacing fields an egui user
  looks at first. Reasoning in specification §16 Q-2, where the change is now
  **approved and scheduled**; the egui connector ships correctly without it, and
  does.

### 4.4 Losses with no fix anywhere, and losses that are refusals

Two rows of the ledger are neither egui's fault nor native-theme's.

**Item 26 — `egui::Button` does not honour `TextStyle::Button`.** `Button::new`
sets `.fallback_font(TextStyle::Button)` (`widgets/button.rs:49`) and `atom_ui`
then overwrites that field at `:358-360` with `text_style.font_id` from
`Style::button_style` → `Style::widget_style`, which is
`override_font_id.unwrap_or_else(|| TextStyle::Body.resolve(self))`
(`widget_style.rs:122`, `:137`); `AtomLayout::fallback_font` is a plain setter
(`atomics/atom_layout.rs:147-150`), so the second call wins. There is **nothing
to fix** — this is `override_font_id` working as designed. It is recorded because
the assumption is natural and wrong, and because it changes where per-widget
typography must travel: on `Style::override_font_id` (`style.rs:254`, checked
first at `:158-161`), not on `text_styles[Button]`. The `text_styles[Button]`
write is nevertheless kept, because `ComboBox` (`containers/combo_box.rs:358`),
`ProgressBar` (`widgets/progress_bar.rs:193`), `CollapsingHeader`
(`containers/collapsing_header.rs:522`) and `Style::drag_value_text_style`
(`style.rs:1433`) do read it.

**Item 27 — `accessibility.text_scaling_factor` is never applied.** Scaling only
`Style::text_styles` would desync text from every geometry field, and egui
already has a global scale (`Context::set_zoom_factor`, `context.rs:2334`) which
is the application's decision, not a theme connector's. Exposed as
`text_scaling_factor(&SystemTheme)`.

And two are refusals, recorded in the ledger so that they read as decisions
rather than oversights: the focus ring (§3.7, ledger item 5 and specification
§5.8 item 1) and the shipped widget (§3.22, ledger item 28).

---

## 5 -- The long-term evolution argument

The design's central claim is that it absorbs churn from **two independent
sides** without either becoming a crisis. Here is the walk-through for each.

### 5.1 When egui bumps a minor

**What breaks.** A renamed or removed `Visuals` / `Spacing` / `WidgetVisuals`
field breaks the mapping code — and it breaks it at **compile time, naming the
field**, because specification §13 T2 destructures those structs exhaustively
with **no `..` rest pattern**. A new field is likewise a compile error naming the
new field, which is exactly the moment to decide whether a native leaf belongs in
it. This is strictly better than a `size_of` assertion, which tells you only that
something changed.

`Style` itself is deliberately **excluded** from the tripwires: its `debug` field
is `#[cfg(debug_assertions)]` (`style.rs:322-323`), so the field count differs
between profiles and the tripwire would fail in one of them. `Visuals` is
included with `#[expect(deprecated)]` for `clip_rect_margin` (`:1085`), which
would otherwise warn.

**What does *not* break, and is the residual risk.** Upstream drift comes in
three modes and the tripwire catches only the first two. (1) A field **added,
removed or renamed** is a compile error naming the field — that is T2's own
claim (specification §13) and it is exact. (2) A field whose **type** changes is
caught only at the sites that assign to it, and only when the new type is not
convertible from the old one; a widened numeric type would compile silently.
(3) A field whose **meaning** changes with no change to its name or type is
caught by **nothing**. This is not hypothetical: if
`Spacing::extra_text_line_spacing` (`style.rs:423`, default `0.0` at `:1464`)
were redefined from an additive delta in points to a multiplier, specification
§6.15's derivation would be wrong by a factor of the font size with every test
still green. The mitigation is a value tripwire beside the shape tripwire —
assert egui's own published defaults, `extra_text_line_spacing == 0.0`
(`:1464`), `interact_size.x == 40.0` (`:1459`), `Visuals::dark().disabled_alpha
== 0.5` (`:1559`) — and reading the **doc-comment diff**, not only the field
list, at every egui bump. Both belong to the version-policy procedure of
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
not the same thing as naming egui's types. Three of the nine — `Dialog`,
`Popover`, `Card` — have no same-named egui type at all; the types behind them
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
changed by **25 lines** under `diff -u | grep -c '^[+-]'` (39 under default
`diff`). The semantic delta is three items: `Spacing::extra_text_line_spacing`
added (`style.rs:423`, default `0.0` at `:1464`), `Visuals::clip_rect_margin`
deprecated (`:1085-1086`), and `warn_if_rect_changes_id`'s default flipped to
`false`. One release, one new field, one deprecation. The new field, notably, is
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
2. `mapping.toml` grows two sections and six rows. **T4 (converse coverage) fails
   until they are added** — it walks every leaf of
   `serde_json::to_value(&resolved)` (`ResolvedTheme` derives `Serialize`,
   `resolved.rs:154`) and asserts each has exactly one manifest row. A forgotten
   field is a red test, not a silent hole.
3. The role-style builder grows two arms. T1 (determinism) and T5 (hostile input)
   cover them for free, because both iterate `Role::all()`.
4. T3 (differential coverage) fails until each new row's declared sinks are
   proved by a preset splice.

**What does *not* change:** `Surface`, `RoleVariant`, `PanelSide`, every
extension trait, every conversion helper, the install sequence, the semver
version. Nothing in the public API surface moves except one `#[non_exhaustive]`
enum growing variants, which is additive by construction.

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
unscoped widget looks like is not a breaking change. The 209 SCOPED leaves become
DIRECT and the README's headline sentence is rewritten upward.

**(b) egui adds something similar but differently shaped.** The blast radius is
the same: the atlas is a *table*, and a table can be delivered through whatever
seam exists. The public enums do not encode the seam. This is why `Role` is
defined as "one variant per `ResolvedTheme` widget field" rather than as "one
variant per egui style hook" — the first definition is stable against upstream
redesign, the second is not.

**(c) egui does nothing, for several more releases.** The most likely outcome, on
the evidence: the module did not change at all across a release. The connector
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
| Ship any `egui::Widget` | Both charter tests pass *and* the widget's behaviour surface stops moving upstream — i.e. egui gains a stable composition primitive that makes a widget a data declaration rather than a paint routine. `Switch` passing the tests is **not** sufficient; it already does |
| Depend on `egui::widget_style::Classes` | egui merges something like §4.2 commit 2 **and** it ships in a released minor. Not a nightly branch, not an accepted issue |
| Emit `FontFamily::Name` | `FontId` gains a weight or slant selector (§4.2), which removes the reason to need extra families in the first place. Emitting `Name` before that would trade two structural panics for eighteen leaves |
| Apply `accessibility.text_scaling_factor` | Never, at this layer. It is `Context::set_zoom_factor`'s job and the application's decision |
| Write `Spacing::default_area_size` | egui gains a per-container default size, or `Modal` gains a size constraint that reads `Style` |
| Write the focus ring into `widgets.active.bg_stroke` | egui separates focus from press in `Response::widget_state` (`widget_style.rs:107-109`), or adds `Visuals::focus_stroke` |
| Offer `RoleVariant::Selected` and `Disabled` together | `Style::button_style` stops overwriting the selected fill regardless of state (`widget_style.rs:147`, `:150-155`), or egui gains a disabled `WidgetVisuals` |
| Depend on `egui_extras`, or default `svg-rasterize` on | `egui_extras` and `native-theme` converge on one `resvg` major. Today they are on 0.45.1 and 0.47 |
| Take a direct `skrifa` dependency for font pre-flight validation | epaint re-exports `skrifa`, removing the version-skew risk |
| Add `egui_kittest` | Its API is read and understood (existence at a version-aligned `0.36.1` is verified; the **API is UNVERIFIED**, §2.8) *and* a concrete gap in the nine headless groups is identified |
| Lower the workspace floor below `1.88.0` | Every dependency that declares a `rust-version` drops below it **and** our own sources stop using let-chains — 59 `&& let ` occurrences today, plus chains whose second term is a bare boolean (§3.18). **Both** conditions, not either. The floor is held up from both sides, so acting on the dependency half alone would break `native-theme`'s own build |
| Raise the workspace floor | A dependency bump forces it, caught by the MSRV job. Raise to the new measured maximum, never to whatever stable happens to be |
| Publish a memory-footprint number for the atlas | `size_of::<egui::Style>()` is measured in both profiles. It **cannot** be stated today because `Style::debug` is `#[cfg(debug_assertions)]` (`style.rs:322-323`); no such claim appears in either document |
| Carry per-widget font **weight** and **slant** (18 leaves, specification §5.8 item 2) | `FontId` gains the fields (§4.2). Until then applications reach per-call weight with `RichText::variation(..)` (`widget_text.rs:200-205`) and `text_role_weight()` tells them what to ask for |
| Fabricate `Spacing::icon_width_inner` on the base style | A platform reports a check-*mark* size. `docs/platform-facts.md:969` defines `indicator_width` as the **box**, so deriving a mark size from it by ratio would be an invented value; the field stays at egui's `8.0` (`style.rs:1466`) |

Two of these deserve a note about *why the temptation is strong*, because a
future reader will feel it.

**The `icon_width_inner` ratio.** Two of the three design proposals wanted
`icon_width_inner = indicator_width * (8.0 / 14.0)`, taking the ratio from egui's
own defaults (`style.rs:1465-1466`). It is honest arithmetic on honest inputs and
it produces a plausible number — which is exactly the problem. No platform
reports a check-mark size, so the result is a synthesised control dimension
presented as theme data. Leaving egui's value is the truthful option, and the
mistake this replaced was worse in the other direction: an earlier rule wrote
`indicator_width` into `icon_width_inner` and left the checkbox **box** pinned to
egui's literal `14.0` on platforms that report 20 (`docs/platform-facts.md:1201`).

**A widget for the switch.** `ResolvedSwitchTheme` is 13 leaves of which 8 are
UNMAPPABLE, and a switch is perhaps forty lines of paint code. The charter still
says no, and the compensating measure is that all thirteen leaves are exposed —
one accessor per leaf — so an application that wants a switch can build one with
**nothing hardcoded**, which is the property that actually matters under this
project's rules.

---

## 7 -- Errors found and corrected during design

Recorded so that a future reader who finds one of these claims in an older note,
a proposal or a matrix knows it was checked and rejected, and does not
"re-correct" the documents back to the wrong value.

| # | Claim in an input | Correct, with evidence |
|---|---|---|
| 1 | `checkbox.indicator_width` maps to `Spacing::icon_width_inner` | It maps to `Spacing::icon_width`. `docs/platform-facts.md:969` defines the leaf as the **box** (14/20/20/14 at `:1201`); egui reads the box as `checkbox_size: self.spacing.icon_width` (`widget_style.rs:179`) and the mark as `check_size: self.spacing.icon_width_inner` (`:180`) |
| 2 | `menu_style` overwrites three `bg_stroke`s | **Four**: `active` (`containers/menu.rs:24`), `open` (`:25`), `hovered` (`:26`), `inactive` (`:28`), plus `spacing.button_padding` (`:23`) and `widgets.inactive.weak_bg_fill` (`:27`) |
| 3 | `text_styles[Button]` ← `button.font.size` is a live DIRECT mapping | `egui::Button` never reads it (§4.4). Per-widget typography travels on `override_font_id`; `text_styles[Button]` is still written for its four other readers |
| 4 | `Ui::disable` can fire `Color32::gamma_multiply`'s `debug_assert` | It cannot. `Painter::multiply_opacity` drops non-finite and clamps (`painter.rs:100-104`). The only caller of `Visuals::disable` is `Ui::dnd_drop_zone` (`ui.rs:2725-2726`) |
| 5 | `WidgetState` has five variants, so `widgets.open` is on the interaction-state axis | Four (`widget_style.rs:84-90`). `open` is unreachable from both dispatchers and is only ever read as a direct field |
| 6 | `egui::SidePanel` / `egui::TopBottomPanel` | Do not exist in 0.36.1 (or 0.35.0). `Panel` (`containers/panel.rs:206`) with `left`/`right`/`top`/`bottom` (`:249`, `:256`, `:265`, `:274`), and `CentralPanel` (`:1187`). `show_inside` is `#[deprecated]` on both (`:428`, `:1218`) |
| 7 | `egui::StyleModifier` | Not re-exported at egui's root — `egui/src/lib.rs:488` re-exports only `style::{FontSelection, Spacing, Style, TextStyle, Visuals}`. The path is `egui::style::StyleModifier` |
| 8 | A naive `as u8` / `as i8` cast **wraps** | Float-to-int `as` **saturates**, `NaN → 0`, since Rust 1.45; epaint relies on it (`epaint/src/corner_radius.rs:42-44`). Integer-to-integer casts do wrap. §3.9 |
| 9 | Handing a pre-built `Arc<Style>` to `UiBuilder::style` costs "not even a refcount bump" | The `Arc` is **moved** (`ui.rs:236`), so no `Style` is copied — but the caller pays one refcount bump to produce it. Only the true half is stated |
| 10 | `egui/src/style.rs` changed by "97 diff lines" between 0.35.0 and 0.36.1 | 25 under `diff -u \| grep -c '^[+-]'`, 39 under default `diff`. The semantic description was right; the number was not |
| 11 | `epaint::FontId` is declared at `fonts.rs:21-24` with the `TODO` at `:24` | `epaint/src/text/fonts.rs:27-34`, `TODO` at `:33` |

Three further corrections were made to the specification while writing this
document, and are listed here because they are the same class of defect:

* Five internal section cross-references pointed one section off (§5.10 for the
  base-owner table, §5.9 for the override list, §6.16 for the structural
  constants twice, §6.17 for `layout.widget_gap`). Corrected to §5.9, §5.8, §6.17
  and §6.16 respectively.
* The specification claimed "twelve `switch_*` accessors" while §4.7 declared
  seven, and simultaneously claimed the **complete** `ResolvedSwitchTheme` was
  reachable through them. Both cannot be true, and the completeness claim is what
  the no-widgets charter rests on: with seven accessors an application building a
  switch would have to hardcode its hover and disabled colours, violating this
  project's no-hardcoded-values rule. Resolved in favour of the charter by
  declaring the six missing accessors — `switch_disabled_opacity` plus the five
  `soft_option` colours (`native-theme/src/model/widgets/mod.rs:621`, `:624`,
  `:627`, `:630`, `:633`), returned as `Option<egui::Color32>` because a `None`
  there is the platform stating there is no distinct appearance, which is
  information and not a hole. `ResolvedSwitchTheme` now has **exactly one
  accessor per leaf**, 13 for 13.
* The free-accessor total was stated as "40-odd" in the implementation task list;
  the declared list contains **62** functions.

A later adversarial review of both finished documents found a further round of
defects. The eight that change a **published number or a stated fact about
egui** are tabulated first, because older notes, the six mapping matrices and
the locked decision record all still carry the superseded values, and a future
reader must not "re-correct" the documents back to them:

| # | Superseded claim | Correct, with evidence |
|---|---|---|
| 12 | Aggregate `DIRECT 34 · SCOPED 217 · DERIVED 65 · UNMAPPABLE 147` | **`32 · 209 · 72 · 150`.** Three widget-level `border.opacity` leaves (`input`, `combo_box`, `list`) were classed DERIVED "folded into the stroke alpha", which is the one operation the rest of both documents forbids: the resolver hardwires every widget-level opacity to `0.0` (`native-theme/src/resolve/validate_helpers.rs:278`, `:332`, `:52`), so folding one in makes every border invisible. They are `UNMAPPABLE source-void` like the other fifteen. Separately, `slider.disabled_opacity` was DIRECT on `visuals.disabled_alpha` while its seven siblings were SCOPED on the same field. Those two causes alone give `33 · 218 · 62 · 150`; the remaining movement to the figure at left is the later uniform `*.font.family` DERIVED rule (§1.5; specification §2, §8.2), which moved ten leaves SCOPED → DERIVED (the nine per-widget `*.font.family` leaves plus `window.title_bar_font.size`, regraded with them) and one DIRECT → DERIVED (`list.header_font.family`), together with `tab.hover_background` DERIVED → SCOPED |
| 13 | The 147 UNMAPPABLE leaves split 38 `source-void` / 109 `egui-limited` | The tables carry a **third** sub-tag, `source-side gap`, on three rows. The honest split of the 150 is `38 + 3 + 109`, and specification §2 now defines all three |
| 14 | `Color32::from_rgba_unmultiplied_const` involves "no float" | Only `a == 0` and `a == 255` are float-free (`ecolor/src/color32.rs:167`, `:170`). For `1..=254` it runs `fast_round(channel as f32 * linear_f32_from_linear_u8(a))` per channel (`:172-176`). The route is still the right one — it premultiplies in gamma space — but the forward conversion is lossy at low alpha, so the round trip is inexact in **both** directions |
| 15 | `FontDefinitions::default()` installs an emoji **and CJK** fallback tail | There is no CJK face. epaint registers exactly four — Hack, NotoEmoji-Regular, Ubuntu-Light, emoji-icon-font (`epaint/src/text/fonts.rs:512-538`) — and egui's own docs say "The default `egui` fonts only support latin and cyrillic alphabets" (`egui/src/context.rs:2098`) |
| 16 | Four of egui's **seven** `Frame` presets "read no `Style` at all" | Eight presets take a `&Style` and all eight read something from it; `Frame::group` reads `widgets.noninteractive`'s corner radius and stroke (`containers/frame.rs:181-182`). The true proposition is the narrower one the specification already used elsewhere: **five** hardcode their inner margin and read no `Style::spacing` — `group` (`:180`), `side_top_panel` (`:187`), `central_panel` (`:192`), `canvas` (`:229`) and `dark_canvas` (`:236-237`), which delegates to `canvas` and so inherits its `.inner_margin(2)`. Only `window` (`:198`), `menu` (`:207`) and `popup` (`:216`) read `Style::spacing`, and 5 + 3 = 8. This row itself first recorded **four**, omitting `dark_canvas`; five is the counted value |
| 17 | A panel separator wider than 127 "stops reserving space" | It reserves 127. `bg_stroke.width.round() as i8` **saturates** (`containers/panel.rs:962`) and `saturating_add` then clamps at `i8::MAX` (`:964-965`) |
| 18 | `to_image_source` can return an `ImageSource::Texture` without retaining anything | `TextureHandle` frees its texture on drop (`epaint/src/texture_handle.rs:25-29`) and `SizedTexture` owns nothing (`egui/src/widgets/image.rs:585`), so the handle must be stored in `ctx.data_mut()`. `Context::forget_image` cannot release it either: it touches only the four loader caches (`egui/src/context.rs:3768-3777`) |
| 19 | The begin-pass plugin "keeps `extra_text_line_spacing` up to date" everywhere | It reaches the two base styles only (`ctx.all_styles_mut`, `egui/src/context.rs:2210`). The atlas's per-`Role` `Arc<Style>` cells are frozen at build time, so scoped text keeps egui's `0.0`. Recorded as ledger item 29 rather than smoothed over |

Nine smaller citation slips were corrected in the same pass and are not tabulated
individually: `epaint/src/text/fonts.rs:997` → `:996` (three places),
`egui/src/load.rs:614` → `:613` (three places),
`widgets/text_edit/builder.rs:728`/`:731`/`:735-739` → `:727`/`:732`/`:737`,
`epaint/src/margin.rs:105-109`/`:110-115` → `:108-113`/`:115-120`,
`epaint/src/shadow.rs:38-43` → `:40-45`, `Visuals::handle_shape` `style.rs:1234`
(the enum) → `:1109` (the field), `Button::shortcut_text` `button.rs:81` →
`:225-237`, `epaint/src/text/fonts.rs:645-653` → `:646-654`, and the
`WidgetState::Inactive` gating, which is in `egui/src/context.rs` and
`egui/src/hit_test.rs` and **not** in `widget_style.rs:112-114`.

The same review closed eight gaps in the *specification's own* surface, none of
which was a wrong claim about egui but each of which would have left an
implementer inventing a decision:

* Two base-owner rows named native leaves that **do not exist** —
  `combo_box.max_height` and `input.min_width`. `Spacing::combo_height` and
  `Spacing::text_edit_width` now say "nobody" and cite egui's own default.
* Six accessors were promised in prose but never declared: `list_row_height`,
  `list_header_font`, `list_header_color`, `toolbar_bar_height`,
  `menu_icon_size`, `toolbar_icon_size`. Declaring them is what makes the
  specification's §1.4 promise — "either exposed as a plain accessor or honestly reported as
  lost" — true, and it moved the accessor count from 56 to 62.
* `FontPlan::diagnostics` was unobservable by construction: every entry point
  takes the plan **by value** and nothing hands it back, so it could only ever
  return an empty slice. Deleted; `ThemeAtlas::notes()` already carries the
  information.
* `IconKey` derived `Eq` and `Hash` while taking an `f32` size, which does not
  compile. The size is now a `u16` — the type
  `native_theme::icons::FreedesktopLoader::size` takes — folded through a total
  expression that makes the URI injective over the key.
* The three extension traits were unsealed, so adding a method to any of them
  would have been a major-version break. They are sealed, and specification
  §12.2 clause 6 now covers them.
* Specification §6.17's "exactly three legitimate literals" rule condemned the
  crate's own formulas — `1.25`, `4.0 / 3.0`, `2.0`, `f32::MAX`. The exemptions are now
  enumerated in a table with the egui source each is read from.
* `mapping.toml` was named but not specified, while two tests compare its
  strings. Specification §13.1 now gives the schema by example and declares
  `style_diff`'s path spelling normative.
* Two of the nine test groups then defined could not do their job. T1's
  `assert_eq!(*a, *b)` across two builds can never pass, because
  `Style::number_formatter` compares by `Arc::ptr_eq` (`style.rs:56-61`) and
  `Style::default` allocates a fresh `Arc` every call (`:1434`); and T7's
  line-spacing assertion was vacuous under `FontDefinitions::empty()`, where
  `FontsView::row_height` returns `0.0` and specification §6.15's guard therefore
  reproduces egui's own default. Both are restated in specification §13.

---

## 8 -- Open questions carried forward

These are the specification's §16 questions, restated with the reasoning behind
each recommendation. None blocks implementation.

**Q-1 — Atlas memory footprint.** The atlas holds at most 25 roles × 3 variants
× 2 themes = 150 `Arc<Style>` cells, of which only the structurally distinct ones
are separate allocations (a role with no `Selected` or `Disabled` data shares the
`Normal` `Arc`), plus a five-node `BTreeMap` per `Style`.
`size_of::<egui::Style>()` was **not measured** and cannot be stated: it differs
between debug and release (`style.rs:322-323`). *What would verify it*: a
`#[test]` printing it in both profiles.
**Recommendation: build eagerly with structural sharing.** If the measured total
exceeds roughly 256 KiB, convert the `Selected` and `Disabled` tables to per-cell
`OnceLock`, which keeps `ThemeAtlas: Send + Sync`. Nothing in the public API
changes either way, so this is a tuning question and not a design risk — which is
itself an argument for the opaque handle of §3.1.

**Q-2 — `layout: LayoutTheme` on `SystemTheme`?** **DECIDED — approved by the
maintainer, 2026-08-10.** Scheduled for a follow-up native-theme release and
tracked in `docs/todo.md`; it is no longer an open question. It is a one-field additive change to a struct
that already carries `preset` and `icon_theme`, and it needs no resolver work:
`Theme::layout` is a plain `LayoutTheme` guarded by
`skip_serializing_if = "LayoutTheme::is_empty"`
(`native-theme/src/model/mod.rs:265-266`) and shared across the light and dark
variants, and all four of its own fields are `Option<f32>`
(`native-theme/src/model/widgets/mod.rs:884-901`), so an absent layout costs
nothing. It benefits the iced and gpui connectors equally. The egui connector must ship correctly without it, and does —
which is why this is a recommendation rather than a dependency.

**Q-3 — `egui_kittest`.** The crate publishes a version-aligned `0.36.1` with
`rust-version = "1.95"`, matching egui 0.36.1 (verified against the crates.io
sparse index). Its **API is UNVERIFIED** — never vendored, never read — so it
appears nowhere in the manifest or the test plan and no claim is made about what
it could test. *What would verify it*: vendoring `egui_kittest` 0.36.1 and
reading its harness API.
**Recommendation: ship the eleven headless groups**, which need nothing beyond
`egui` itself. Since a version-aligned harness demonstrably exists, the trigger
for revisiting is concrete rather than speculative: evaluate it if a §13 group
proves unable to observe a regression that matters — snapshot-level rendering
differences being the likeliest such gap.

**Q-4 — `#[non_exhaustive]` on `PanelSide`?** **Recommendation: keep it
exhaustive.** The cost of being wrong is one breaking change in a pre-1.0 crate;
the ergonomic gain — applications can `match` it — is permanent.

**And one UNVERIFIED item that is not a question but must not be lost**: whether
a **negative** `Spacing::extra_text_line_spacing` is safe (§3.12). Undocumented
and unasserted in 0.36.1; egui clamps only its own settings slider to
`0.0..=20.0` (`style.rs:2023`). *What would verify it*: a documented range or an
assertion on the field in a future egui release. Until then the connector clamps
to `>= 0.0` itself, which is the conservative direction: too much leading is
ugly, negative leading may overlap glyphs.
