# native-theme-egui-widgets — Rationale

Companion to [`todo_egui-widgets-spec.md`](todo_egui-widgets-spec.md).
Target: **egui 0.36.2** and the native-theme **v0.5.9** model. Version
milestone: **undecided** — a scheduling choice with no bearing on the look the
crate draws. Re-verify against the connector as built when this crate is
scheduled.

---

## 0 -- What this document is for

The specification says *what* the crate is. This says *why*, and why the
alternatives were not chosen, so that none of them has to be re-argued from
scratch. Where a claim rests on code, the code is cited; where it is a
judgement, it is labelled as one; where something is unverified, it says so.

---

## 1 -- The problem, stated exactly

The connector maps every leaf of `ResolvedTheme` onto egui's one global `Style`
and its role scopes, and says in its own honesty ledger which leaves no
`Style` field can carry. Most of what it cannot carry globally it still
reaches with one call at the call site — a role scope, a `Surface` frame, an
accessor passed to a builder. What is left is a short list of native pixels
that **no** `Style` and no single call can produce, because egui draws them
from a hardcoded value or does not draw them at all, and one, the link's, that
a call reaches only through a choice every call site would otherwise repeat:

| native appearance | why no `Style` reaches it |
|---|---|
| a switch | `egui/src/widgets/` has no switch or toggle module (spec §4.1) |
| a slider knob in its own colour | egui paints the rail and the resting knob from the same `inactive.bg_fill` (`widgets/slider.rs:774-775`, `:813-818`), and all four platform presets state two different colours (spec §4.2) |
| a spinner at the theme's stroke | `egui::Spinner` hardcodes `Stroke::new(3.0, color)` (`widgets/spinner.rs:58`) |
| a link in its hover, pressed, disabled and visited colours | `Link` paints one `hyperlink_color` (`widgets/hyperlink.rs:47`), and egui records no visited state. The connector grades the hover, pressed and visited colours DERIVED (the disabled one is SCOPED), a per-call text colour chosen by this pass's interaction, read before the widget is added (`Context::read_response`, `context.rs:1350-1355`; connector spec §5.3), so what is missing is not a route but that choice and a visit set, which the wrappers make once (spec §4.5) |
| a segmented control | egui has none; the connector's `SegmentedControl` scope carries its colours — the active segment's in the `Normal` cell's `selection.*` — and the divider's width as the row's gap; what no `Style` says is the row itself — one exclusive choice, reported as a radio group (spec §4.4, §2.7) |

This crate is those five, and nothing else.

### 1.1 Why it is only those five

An earlier design of this crate also carried a toolbar, a status bar, a
sidebar, a tab bar, an expander, a list, a dialog button row and a wrapper
around each of egui's own widgets. Each put an egui widget or container inside
the role scope or `Surface` frame the connector already builds, or passed it
one connector accessor. That is one call the application writes itself; the
wrapper added a name, an API and a per-release audit, and no pixel. Under the
admission rule (spec §1.5) the cheapest tier for each of them is no widget at
all, so they are gone (spec §4.6).

What stays is where this crate draws what the connector cannot:
the five of §1, each at the cheapest tier that draws it.

---

## 2 -- Options considered

### Option A: Ship nothing; the connector is enough (rejected)

**Rejected because** of §1: no role scope gives egui a switch, a knob fill
apart from the rail, a spinner stroke, a visited link or a link's state
colours. An application could paint each itself — which is exactly the code
this crate holds once, audited, instead of in every application.

### Option B: Put the widgets in the connector (rejected)

The connector's no-widgets charter forbids it, and its reason still holds: a
widget is a permanent per-release audit obligation — interaction, animation,
`WidgetInfo` — in a crate whose remit is theme *mapping*; two failure domains
under one version number make a rendering regression indistinguishable from a
mapping regression. A separate crate satisfies that reasoning rather than
evading it: the mapping crate stays pure, and this one carries its obligation
openly, with its own version and tests.

### Option C: A drop-in replacement that shadows egui's API (rejected)

`Ui::button` and its siblings are **inherent** methods on `egui::Ui`
(`ui.rs:1848`). Rust resolves inherent methods before trait methods, so an
extension trait offering `button` is *silently ignored* at every call site —
wrong pixels, no diagnostic. A design whose failure mode is "looks like it
worked" is disqualified.

### Option D: A `Deref`-based wrapper `Ui` (rejected)

Rust tries inherent methods on the outer type before dereferencing, so an
inherent `button` on a wrapper would win. It fails on the builder boundary:
`ScrollArea::show`, `Window::show`, `Grid::show` and `CollapsingHeader::show`
hand their closure a raw `&mut egui::Ui`, so the wrapper is lost inside every
container unless every builder is wrapped too. Third-party crates take
`&mut egui::Ui` and would render unstyled beside styled widgets.

### Option E: Fork egui's widget layer (rejected)

Owning interaction, focus order, IME, undo, clipboard, bidirectional text and
accessibility, re-audited every release, to change colours. `scroll_area.rs`
alone is 1,641 lines and `text_edit/builder.rs` 1,457 (spec §1.4).

### Option F: The upstream `class_overrides` change, and no widgets (rejected as the whole answer)

The connector designs that contribution; it would let egui's own widgets take
per-role styles. It would not give egui a switch, a knob fill apart from the
rail, a spinner stroke, a visited link or a segment group, and upstream
appetite for it is **UNVERIFIED**. It is the connector's work and does not
compete with this crate (§7 Q-4).

### Option G: A companion crate, tiered, implementing `egui::Widget` (chosen)

Widgets in a separate crate, under distinct names, implementing
`egui::Widget` so they compose through `ui.add()`
(`egui/src/widgets/mod.rs:63`, `ui.rs:1521`) and work with `add_sized`
(`ui.rs:1538`) and `add_enabled` (`ui.rs:1588`) for free. No name competes
with an inherent method, so Option C's silent failure cannot occur; the tier
model bounds the cost per widget; and it draws the five things of §1, which
nothing else on this list does. What it costs is recorded in spec §7: existing
code and third-party crates do not benefit, and adoption is per call site.

---

## 3 -- The decision record

### 3.1 No extension trait, ever

A `ui.native_switch(..)` helper would be additive and harmless the day it
shipped. It is refused because two spellings for one widget leave every call
site with a permanent question about which is current. Recorded as a rule so
that "it is only one method" does not reopen it.

### 3.2 Four tiers, ordered by liability, with the last one closed

Judging each widget on its merits as it comes up lets the boundary drift toward
reimplementation, because each step looks small. Naming Tier F and closing it
decides the expensive cases once. `Slider` is Tier P, not F: this crate paints
a linear horizontal rail and knob with egui's own interaction semantics
(spec §4.2), not egui's slider with its value field, logarithmic ranges and
smart aim.

### 3.3 The atlas comes from the `Context`

Every widget calls `ThemeAtlas::from_ctx`. A `&ThemeAtlas` parameter would put
the connector's type in every signature and push theme plumbing back into the
application.

### 3.4 No atlas means plain egui, never a broken screen

A user will add this crate before wiring up `install()`, or call a widget where
installation failed. Falling back to egui's own counterpart is the safest and
the most honest behaviour: no theme installed, no theming applied, nothing
fabricated. For a switch that counterpart is egui's `Checkbox`: drawing a
switch shape without the theme's switch sizes would need a proportion no
source states.

### 3.5 Disabled is the `Disabled` scope plus `ui.disable()`

egui models disabled as one opacity multiply, `Visuals::disabled_alpha` via
`Ui::disable` (`ui.rs:497-502`), with no disabled colour. The connector's
`Disabled` cell writes the platform's disabled colours and sets that alpha to
`1.0`. Opening that cell and calling `ui.disable()` inside it gives a widget
egui's own disabled semantics — no hit sense, no focus, `enabled: false` for
assistive technology — and the platform's colours unfaded, with no
reimplementation of either. The alternative, a Tier P widget that never calls
`ui.disable()` and blocks its own input, reimplements what egui already does
and can drift from it.

`ui.add_enabled(false, w)` is deliberately left alone: silently changing an
egui method a user reached for would be the Option C failure in a new place.
It fades on top of the disabled colours, and the rustdoc says so.

### 3.6 The focus ring is the connector's

The connector paints one ring, at the end of each pass, around whichever widget
holds keyboard focus, at the radius of the role scope it sits in. A ring of our
own would double it. The switch registers its track as its outline, because its
focusable part is not its whole response.

### 3.7 Accessibility is mandatory, and the mapping is opinionated

A hand-painted widget emits nothing to a screen reader unless it calls
`Response::widget_info` (`response.rs:869`). `WidgetType` has no switch
(`egui/src/lib.rs:623`), so `Switch` reports `Checkbox` to egui and
`Role::Switch` to AccessKit, through `Context::accesskit_node_builder`
(`context.rs:3684`). A segmented control is an exclusive choice among siblings,
so its row is a radio group and each segment a radio button. Reporting `Other`
would be technically true and practically useless.

### 3.8 An unstated size is egui's, and a hover colour is a layer

A size the theme leaves unstated is one the platform does not document; any
number put there would be invented, and egui's own value is the only one nobody
had to make up — the rule native-theme v0.5.9 made binding on every connector,
and the iced connector's paddings follow it (`button_padding`,
`input_padding`). A hover colour is composited over the idle fill because that
is how the platforms draw it (rule C17 of the v0.5.9 theme contracts); painting
it in place turns a translucent layer into a different colour.

### 3.9 The spinner: the icon set's first, egui's motion second

Where the application's icon set has an animated indicator, the spinner is
that indicator, because a spinner in the icon set's style is what the
application shows everywhere else, and both sibling showcases do the same. The
painted arc is for sets without one. Its motion is egui's, cited as egui's:
native-theme states a spinner's size, colour and stroke, and nothing about its
motion, so egui's is the value nobody had to invent (§3.8). Its radius follows
from the diameter being an outer size and egui's stroke being centred on its
path (spec §4.3).

### 3.10 The segmented control draws only what is stated

`separator_width` is a width; nothing in the model states the divider's colour
or the shape where two segments meet. Painting the divider in the border's
colour, or squaring the inner corners, would each state something no source
does, so neither is drawn, and the control shows as segments `separator_width`
apart (spec §4.4, §7 item 5). The research that would state them is filed in
`docs/todo.md`, as the tab bar's active-tab indicator was before it.

### 3.11 Fonts: nothing to add today

Every text this crate lays out takes the font of the scope it sits in, which
the connector writes. A per-run weight or slant would change a pixel only
where a role's font states a weight or style the installed face does not have;
measured on 2026-09-25, all sixteen presets state `link.font` and
`segmented_control.font` at weight 400, style normal, in both modes — the
weight and style of `defaults.font` — so no route is specified. If a preset
ever states another, the route is the connector's per-call one —
`role_font_weight` as a `wght` coordinate (`TextFormat::coords`,
`epaint/src/text/text_layout_types.rs:505`) and `role_font_is_italic` through
egui's slant — and this crate adopts it then.

### 3.12 No features of its own; the connector's MSRV

Features here would multiply against the connector's, producing
configurations nobody tests. The connector's are forwarded, with its default
set, because a native look must not be opt-in and a default this crate
requested from the connector could not be turned off below it: Cargo unifies a
dependency's features upward. `rust-version = "1.95"` is egui 0.36.2's, not
the workspace's `1.88.0` floor; this crate is downstream of the connector and
can never require less.

---

## 4 -- Why the charter is satisfied rather than circumvented

The connector's no-widgets charter exists so that a theme-mapping crate does
not silently acquire a rendering obligation. Its test: egui has no widget of
that visual identity, **and** most of the native struct's leaves are otherwise
unmappable. A separate crate honours the charter's reason; its test is
replaced, not inherited (spec §1.5), for two defects:

* **It measured need twice and cost never.** Both conditions ask whether a
  widget is wanted; neither asks what owning it costs, which is the reason the
  charter exists.
* **It assumed "our widget" means "we paint every pixel".** A segmented control
  can be ours — our name, our layout, our role choices — while egui's own
  `Button` paints, senses and reports each segment.

Under the replacement, three widgets are painted, each with a citation proving
no cheaper tier draws it (spec §4.1–§4.3); one is composed; two functions wrap;
and everything the connector already delivers with one call is no widget at
all (spec §4.6).

---

## 5 -- The long-term argument

**When egui bumps a minor**, Tier W and Tier C name egui's widgets and builder
methods, so renames surface as compile errors. Tier P depends on the oldest,
least volatile primitives — `allocate_response`, `painter`, `Response`,
`WidgetInfo` — and T7 re-checks each promotion's citation. The version policy
is the connector's: one egui minor at a time.

**When native-theme grows a widget or a field**, it adds a candidate, tiered by
spec §1.5; a divider colour or a joined-outline field turns spec §4.4's "not
drawn" into a drawn pixel.

**If the upstream change lands**, nothing here changes: it reaches none of the
five things of §1. The crate bets only that egui keeps its `Widget` trait, its
painting primitives and its accessibility model, all of which predate the
styling system.

---

## 6 -- What was deliberately not done

* **No painted `Link`.** Every link leaf but a bare link's visited colour and
  the hover underline reaches egui's `Link` per instance, because its
  `hyperlink_color` is only a fallback behind a colour the text carries itself
  (`epaint/src/shapes/text_shape.rs:25`). What stays egui's is spec §7 item 3.
* **No `mark_link_visited`.** The visited set holds what this `Context` saw
  clicked. An application's own history is data this crate has no source for.
* **No text-rendering configuration.** Reading the platform's hinting and
  antialiasing preferences belongs in the core crate and the connector
  (`docs/todo.md`).
* **No icon loading of its own.** The one icon shown here, the spinner's, comes
  from native-theme's loaders and is drawn through the connector.
* **No layout containers.** A `Row` or `Column` that applied theme spacing would
  overlap egui's own layout API and pull this crate toward Option D.

---

## 7 -- Questions, and their decisions

* **Q-1 — `impl Widget` or named structs for the link wrappers? DECIDED:
  `impl Widget`.** The return type has no bearing on the native look. A
  function returning `impl Widget` can later return a named type without
  breaking a caller, who could only ever use it as a `Widget`, while a named
  struct, once public, can never be taken back. The trigger to name one is a
  caller that needs an egui per-instance option the wrapper does not pass
  through, because leaving the wrapper to reach it leaves the native look with
  it.
* **Q-2** (does the charter's two-condition test decide admission) is answered
  by spec §1.5 and §4.
* **Q-3 — Where does the switch thumb inset come from? DECIDED: it is
  derived,** `0.5 * (switch.track_height − switch.thumb_diameter)`, the thumb
  centred on the track's axis, which is the geometry the two platform numbers
  describe; both fields are resolved for every preset, and `0.5` is one of spec
  §2.3's constants. The iced connector derives its toggler's inset the same
  way, expressed as iced's ratio and guarded against a track with no height and
  a thumb taller than its track
  (`connectors/native-theme-iced/src/styles.rs:725-726`); iced needs the guard
  because its fallback is its own ratio, while a widget that paints the thumb
  itself draws an overhanging thumb, which is what those numbers describe. A
  field for a value two existing fields already determine would be the wrong
  direction.
* **Q-4 — Does this crate survive the upstream `class_overrides` PR? DECIDED:
  yes, and it proceeds.** That change would let egui's own widgets take
  per-role styles; it would give egui no switch, no knob fill apart from the
  rail, no spinner stroke, no visited link and no segment grouping, which is
  everything this crate draws. The crate does not depend on the change and
  would not be made redundant by it (§5).

One item is **UNVERIFIED** and repeated so it is not lost: **upstream egui's
appetite for `Style::class_overrides` has never been tested.** Option F rests on
it; this crate deliberately does not.
