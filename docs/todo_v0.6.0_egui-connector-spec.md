# v0.6.0 — egui Connector: Specification

Status: Pending
Crate: `connectors/native-theme-egui`
Target toolkit: **egui 0.36.2**
Revised 2026-09-25 against egui 0.36.2 and native-theme 0.5.9 (HEAD `2db686ee`).

---

## 0 -- Scope

This document specifies `native-theme-egui`, the third toolkit connector in this
workspace, alongside `connectors/native-theme-iced` and
`connectors/native-theme-gpui`. It is the WHAT and the HOW: an implementer
should need no further design input. The companion rationale document —
[`todo_v0.6.0_egui-connector-rationale.md`](todo_v0.6.0_egui-connector-rationale.md)
— records WHY each decision was taken and which alternatives were rejected, and
the implementation plan,
[`todo_v0.6.0_egui-connector-plan.md`](todo_v0.6.0_egui-connector-plan.md),
the order in which it is built, each step with its gate.

**Target version: egui 0.36.2** — the latest release at the time of this
revision (`egui/Cargo.toml:16`). Every `egui/…`, `epaint/…`, `ecolor/…`,
`emath/…`, `egui_extras/…`, `eframe/…` and `egui_kittest/…` path in this
document is relative to that crate's own root and carries 0.36.2 line numbers.
egui 0.35.0 is never a target and appears only in the rationale, as churn
evidence and in its corrected errors (rationale §2 Option D, §4.2, §5.1, §7). egui 0.36.1 is cited only where its behaviour differs from 0.36.2, and is never a
target either: its `Window` title bar falls back to a different frame (§3.1),
so the requirement is 0.36.2, not 0.36. Every `native-theme/…`, `docs/…` and
`connectors/…` path is relative to this repository's root and carries the line
numbers of `2db686ee` (native-theme 0.5.9).

The version is pinned to one egui minor because the application's
`egui::Style` must be *our* `egui::Style`: cargo cannot unify two
semver-incompatible egui versions, and two copies in one dependency graph
produce `expected egui::Style, found egui::Style`. §12 states the policy.

### 0.1 What this crate is

> `native-theme-egui` gives an egui application an excellent global theme out
> of the box and opt-in per-widget geometry: egui 0.36.2's `Style` has an
> interaction-state axis and provably no widget-type axis, so what one global
> `Style` can hold reaches every widget automatically, and the per-widget
> values reach the screen only where the application asks for them.

That sentence is the crate's headline claim, verbatim, in the README's first
paragraph and in the crate-level rustdoc. It states no counts: §5.7 owns them.
The phrase "full theme geometry" must **not** appear as a claim about what this
crate delivers against egui 0.36.2; the rationale's §1 says what the phrase
could mean and which upstream change would make it true.

### 0.2 What this crate is not

It ships no `egui::Widget`, no image decoder, no font database of its own and
no fork of any egui type. System-font lookup is native-theme's, not this
crate's: with the `system-fonts` feature (on by default, §12.1),
[`fonts::FontPlan::from_system`] asks `native_theme::fonts::system_face` for the
theme's typefaces (§4.9). §14 is the complete list of what is lost and why.

---

## 1 -- Problem statement

The facts the design rests on, each stated once. The measurements behind them
and the argument from them are the rationale's §1.

### 1.1 The two sides

* **native-theme.** §5 has one row per leaf of
  `native_theme::theme::ResolvedTheme` (`native-theme/src/model/resolved.rs:156`
  — `defaults`, `text_scale` and the 25 per-widget structs of
  `resolved.rs:164-212`, flattened through `ResolvedFontSpec`,
  `native-theme/src/model/font.rs:242-273`, `ResolvedDefaultsBorder`,
  `native-theme/src/model/border.rs:212-225`, and `ResolvedWidgetBorder`,
  `border.rs:233-246`, whose `padding` is four `Option<f32>` sides,
  `border.rs:195-204`) and one per field of `LayoutTheme`
  (`native-theme/src/model/widgets/mod.rs:896-913`). An `Option` leaf is one
  leaf whether or not a given theme states it. `LayoutTheme` is not on
  `ResolvedTheme`; it is `Theme::layout` (`native-theme/src/model/mod.rs:269`)
  on the preset path and `SystemTheme::layout` (`native-theme/src/lib.rs:488`)
  on the `from_system()` path, so it is reachable on both. §5.7 counts the
  rows.
* **egui.** `egui::Style` (`egui/src/style.rs:244`) differs between the two
  build profiles — `Style::debug` is `#[cfg(debug_assertions)]`
  (`style.rs:323-324`). Every widget's per-state appearance flows through
  `Visuals::widgets: Widgets` (`style.rs:1030`, struct `:1250`), which has
  exactly five entries — `noninteractive` `:1255`, `inactive` `:1258`,
  `hovered` `:1263`, `active` `:1266`, `open` `:1269` — each a `WidgetVisuals`
  (`style.rs:1290`) with exactly six fields: `bg_fill` `:1295`,
  `weak_bg_fill` `:1300`, `bg_stroke` `:1305`, `corner_radius` `:1308`,
  `fg_stroke` `:1311`, `expansion` `:1319`.

### 1.2 The central asymmetry

`Widgets` is indexed by **interaction state**. It has **no widget-type axis**.
A `Button` and a `TextEdit` in the same `Ui` therefore cannot have different
corner radii through theme data: both resolve to the same
`Visuals.widgets.<state>.corner_radius` — `widget_style.rs:161` for the button,
`widgets/text_edit/builder.rs:744`, `:749`, `:754` for the text edit.
`egui::widget_style` (`pub mod` at `egui/src/lib.rs:426`) does not add one:
`Style::widget_style` (`widget_style.rs:120`), `button_style` (`:146`),
`checkbox_style` (`:174`), `label_style` (`:194`) and `separator_style`
(`:212`) are **inherent methods on `Style`** — no trait, no registration hook —
and the one class any of them reads is the hardcoded `SELECTED_CLASS` of
`button_style` (`:150`, declared `:225`).

`WidgetState` has **four** variants, not five — `Noninteractive`, `Inactive`,
`Hovered`, `Active` (`widget_style.rs:84-90`). `Widgets::state` (`:94-99`) and
`Widgets::style` (`style.rs:1273-1282`) can never return `open`; its only
readers are direct field reads at `containers/window.rs:1427`,
`containers/combo_box.rs:371` and `:450`, `widgets/color_picker.rs:117` and
`egui_extras/src/datepicker/button.rs:143` (`DatePickerButton` while its
calendar is open), plus `menu_style`'s write at `containers/menu.rs:25` and `SubMenuButton::ui`'s
read-then-copy-into-`inactive` at `containers/menu.rs:382-386`.

### 1.3 Contested fields

A field is **contested** when two or more native-theme leaves need different
values in it at the same time. One `Style` holds one value per field, so the
design needs more than one `Style` (§3). §5.11 lists every contested field with
every claimant, and §5.9 names the base owner whose value the global style
carries.

`Style.spacing.interact_size.y` (`style.rs:409`) shows the shape: every leaf
that sizes a control's height claims it (§5.11), and `Checkbox` and
`RadioButton` derive their *whole* minimum size from it through
`Vec2::splat(interact_size.y)` (`widgets/checkbox.rs:85-86`,
`widgets/radio_button.rs:54-55`), so writing it also sets a width floor. It is
**not** the sink for `checkbox.indicator_width`, which is `Spacing::icon_width`
(`widget_style.rs:179`, §5.3). Three of its claimants — `menu.row_height`,
`toolbar.bar_height` and `list.row_height`
(`native-theme/src/model/widgets/mod.rs:210`, `:452`, `:521`) — are
`soft_option` fields, which stay `Option<f32>` after resolution
(`native-theme-derive/src/gen_structs.rs:42`); a `None` claimant writes nothing
(§2, *Unstated sizes*) and so takes no part in that theme's contest.

### 1.4 What the crate promises

Every mappable leaf reaches *some* egui object: the global `Style`, a per-role
`Arc<Style>`, a per-surface `egui::Frame`, the install plugin, or a per-call
input the application passes (§2's route rule). Every other leaf is honestly
reported as lost, in §14, with the upstream change that would close it.

What renders with no application cooperation is the base style — the DIRECT
leaves, the base owners among them (§2, §5.9), and the DERIVED values the base
style carries — plus what the install plugin does every pass with no
application code: the OS colour scheme where the integration reports none, and
the focus ring around the keyboard-focused widget (§3.2). Everything else is
opt-in, one line at the call site: a role scope or a surface frame (§3). No
hook in egui 0.36.2 could make it otherwise (§1.2, §14 item 1).

### 1.5 A second, deeper reach problem

`Area::Prepared::content_ui` builds its `Ui` with a bare `UiBuilder::new()`
(`containers/area.rs:611-629`) — no `.style(..)`. `Ui::new` then falls back to
`ctx.global_style()` (`ui.rs:136`). Everything built on `Area` — `Window`,
`Popup`, `Tooltip`, `Modal`, menus, `ComboBox` popups — reads the **Context**
style and ignores the calling `Ui`'s style entirely. Wrapping a scope around a
`Window::show` has zero effect on that window's frame. The claim is checkable in
one grep: `Ui::new(` has exactly **two** call sites across the seven 0.36.2
egui-family crates (`egui`, `epaint`, `ecolor`, `emath`, `egui_extras`,
`eframe`, `egui_kittest`) — `context.rs:802-808` (the root `Ui`) and
`containers/area.rs:629` — and neither passes a `.style(..)`.

The advice "wrap it in a scope" is therefore false for exactly the containers
where a distinct look is most expected. §3.3 states which seam does reach them.

**What *does* reach them: a scope placed inside the closure.** The closure an
`Area`-based container hands the application receives a `Ui` that **descends
from** the `Area`'s content `Ui` — `Window::show` threads `area_content_ui`
through `Frame::show` → `Resize::show` → `CollapsingState::show_body_unindented`
→ `ScrollArea::show`/`Frame::show` before it calls `add_contents`
(`containers/window.rs:710`, `:719-752`), and `Modal::show` calls
`area.show(ctx, ..)` and then `ui.scope_builder(.., |ui| frame.show(ui, content))`
(`containers/modal.rs:92`, `:104-108`). A child `Ui` inherits its parent's
`Arc<Style>` (`ui.rs:237`), and `Ui::set_style` (`ui.rs:387`, documented
"Changes apply to this `Ui` and its subsequent children", `:384`) replaces it for
that `Ui` and everything after it. So a role scope applied as the **first
statement inside the closure** — `ui.native_set_style(role, variant)` or
`ui.native_scope(role, variant, ..)` (§4.5) — reaches every widget the
application adds there. It does **not** reach the chrome computed before the
closure runs, and it does not reach the parts the container paints itself;
§14 item 2 enumerates the residue. This is application cooperation, not a
fourth seam: one line of application code, the opt-in §1.4 describes.

---

## 2 -- Verdict vocabulary

Used in every mapping row in §5, defined once here.

| Verdict | Exact meaning |
|---|---|
| **DIRECT** | One write to the global `egui::Style` renders the value correctly, and either no other native leaf claims that field or this leaf is its nominated **base owner** (§5.9). Nothing else is required. |
| **SCOPED** | The value is expressible, but only inside a per-role `Arc<Style>` or a per-surface `egui::Frame`: either its egui field is claimed by another native leaf (every colliding claimant is named), or the only object that carries it is a `Frame` the application attaches through a `Surface` (the row names the surface). |
| **DERIVED** | Not 1:1. Either a formula is required (§6); or an input outside `ResolvedTheme` is required (font bytes, a live `Context` — the focused widget's rectangle the install plugin paints the focus ring around, §6.18, is one); or the value reaches its stock widget only as a per-call input at the application's call site (the third kind of route below). The formula and its boundary behaviour are given. |
| **UNMAPPABLE** | This crate takes no **route** (defined below) that carries the value: egui 0.36.2 offers none, or offers one that §5.8 declines (items 7 and 8). Sub-tagged `egui-limited` (no field or API exists, or the one egui offers is declined under §5.8 item 7 or 8; a citation shows the value hardcoded or absent), `widgets-crate` (as `egui-limited`: egui's stock widget has no route, but the companion crate `native-theme-egui-widgets` paints the value in a widget of its own, `docs/todo_egui-widgets-spec.md` — which the route rule below does not count as a route; §5.7 lists these rows), `source-void` (the *native* leaf carries no information — a structural constant, not a theme value), or `source-side gap` (the native leaf carries a real value, but `ResolvedTheme` lacks the companion data egui requires to use it — `shadow_enabled` with no offset, blur or spread, §14 item 10; §5.7 lists these rows). No row uses `source-void` (§5.7); the tag is kept so that a future structural constant is classified, not hidden. |

**The route rule.** A *route* is how a value reaches a **stock** egui widget
or container, and there are exactly three kinds: a `Style` field (in the base
style, a role style or a `StyleModifier`), a [`Surface`]'s `egui::Frame`, or a
per-call input the stock widget itself takes — a §4.7 accessor or a
`ResolvedTheme` leaf feeding the widget's own builder method, or the `Ui` or
`Frame` it is laid out in (`TextEdit::margin`, `RichText::font`,
`Image::fit_to_exact_size`, `Ui::set_max_size`, `Ui::add_space`, …), chosen,
where the leaf depends on the widget's state, from the `Response`
`Context::read_response` returns before the widget is added (§5). Two
things are **not** routes, however faithful the pixels: faking a state the
widget ignores (writing a colour into a state the widget never reads in that
situation), and drawing a shape from scratch with the painter at the
application's call site, which re-implements the widget rather than theming it.
The install plugin's focus ring (§6.18) is the one painted route, and it is one
because it is the connector's, not the application's: it runs every pass for
every focused widget with no application code, as a `Style` field would, and it
overlays the stock widget instead of replacing any of its painting. Every
UNMAPPABLE row is UNMAPPABLE by this rule or by a §5.8 decline, and every
other verdict names its route.

Three conventions that keep the tables honest:

* **One grading: the verdict records what this crate carries.** Where a locked
  decision declines a sink egui could have carried (§5.8), the row is graded by
  the route this crate does use, and its note says "egui could carry this
  through `<sink>`; declined, §5.8" — `list.header_font.size`, which a
  `TextStyle::Name` key could carry, is graded by the route it takes without
  one.
* **A base owner is DIRECT when its one base-style write renders it on its
  own widget, whoever else claims the field.** That is the DIRECT definition
  applied to §5.9's nominations, to a `defaults.*` leaf and a per-widget leaf
  alike: `button.background_color` is written into the base style's
  `inactive.weak_bg_fill` and every unscoped `Button` shows it, exactly as every
  unscoped panel shows `defaults.background_color`. A role scope that writes
  the same value again changes nothing, and the row's «Role» note records it.
  The per-widget leaves that are DIRECT for this reason are exactly these:
  `window.title_bar_background`;
  `menu.background_color`;
  `button.background_color`, `.font.color`, `.border.color`,
  `.border.line_width`, `.border.corner_radius`, `.hover_background`,
  `.hover_text_color`, `.active_background` and `.min_height`; `checkbox.indicator_width` and `.label_gap`;
  `input.background_color`; and `scrollbar.track_color`, `.thumb_color`,
  `.thumb_hover_color` and `.thumb_active_color`. This list lives here only; other sections point to
  it. A base owner whose base write does **not** render it on its own widget
  keeps its verdict and says why in its row — `button.font.size` is SCOPED,
  because its global `text_styles[Button]` feeds `ComboBox` and three other
  readers but not `Button` (§5.3 B3) — and one that needs a formula or font
  bytes stays DERIVED (`button.border.padding.*`, `defaults.font.family`, the
  two shadow gates of §5.9). The contest is not hidden by this: every displaced
  claimant keeps its own verdict and is named in its row, in §5.9 and in §5.11,
  which count claimants, not verdicts.
* **A state layer is a route, not a derivation.** A hover or pressed fill
  composited over its widget's idle fill (C17, §6.1) is written as the colour
  the platform paints — for an opaque layer, the layer itself (§13 **Numeric
  sinks**) — so the composite no more makes a row DERIVED than folding
  `defaults.border.opacity` into a stroke's alpha (§6.13) makes
  `defaults.border.color` DERIVED; the folded-in `defaults.border.opacity` is
  the DERIVED row. The row is graded by its field, as every other row is.
  **A value written as the sum of two leaves is DERIVED.** A padding side with
  the border's `line_width` added — egui strokes inside the padding it is given
  where the platform's padding lies inside the border (§7.2) — is not the
  side's own value, so every border-inclusive padding row is DERIVED, whatever
  its field.

Three facts constrain every row and are stated once:

* **Lengths.** `ResolvedTheme` lengths are logical pixels
  (`docs/platform-facts.md:883-894`). egui `Style` lengths are logical points
  and are **not** scaled by `pixels_per_point` or `zoom_factor`: egui keeps one
  points-to-pixels factor, `zoom_factor * native_pixels_per_point`
  (`context.rs:455`), reports it with every frame's output
  (`FullOutput::pixels_per_point`, `egui/src/data/output.rs:33`), and the
  integration hands it to tessellation and rendering (`Context::tessellate`,
  `egui/src/context.rs:2854-2863`; eframe at
  `eframe/src/native/wgpu_integration.rs:800`), while nothing in
  `egui/src/style.rs` reads it. **The connector must never pre-scale anything for DPI.** One logical
  pixel becomes one point, verbatim. The user's text-scaling preference is a
  different input — an accessibility factor, not a display density — and
  multiplies text sizes only ([`Builder::accessibility`], §8.6).
* **Unstated sizes.** A size the theme does not state stays unstated, and
  `None` is never zero. Each side of a widget's `border.padding` (a
  `ResolvedPadding`, `native-theme/src/model/border.rs:195-204`),
  `menu.row_height`, `toolbar.bar_height`, `toolbar.item_gap`,
  `list.row_height`, `combo_box.arrow_area_width`
  (`native-theme/src/model/widgets/mod.rs:210`, `:452`, `:458`, `:521`, `:745`)
  and every `LayoutTheme` field (`:896-913`) is an `Option<f32>`
  after resolution. For `None` the connector writes nothing, so egui's own
  value stands — for a padding, on that side of the `epaint::Margin` sink
  only; the other sides still take their stated values. `Some(0.0)` is a
  stated zero and is written like any other length, through §7's conversion.
  The connector never substitutes a number of its own for `None`; an
  application that reads one of these leaves itself gets the `Option<f32>`.
* **Light versus dark geometry.** `Theme::default_style()` is
  `Style { visuals: self.default_visuals(), ..Default::default() }`
  (`egui/src/memory/theme.rs:24-29`), so egui's light and dark styles differ
  **only** in `visuals`. Every non-`visuals` value in this document is written
  into both styles, each computed from its own variant's `ResolvedTheme`.

---

## 3 -- Architecture

### 3.1 The layer model

```text
  ┌──────────────────────────────────────────────────────────────────────┐
  │ native_theme                                                         │
  │   SystemTheme / Theme  ──resolve()+validate()──▶  ResolvedTheme × 2   │
  │                                                    (light, dark)     │
  └───────────────────────────────┬──────────────────────────────────────┘
                                  │ &ResolvedTheme, &LayoutTheme,
                                  │ &AccessibilityPreferences, FontPlan
                                  ▼
  ┌──────────────────────────────────────────────────────────────────────┐
  │ native_theme_egui :: COMPILE  (pure, no Context, infallible)         │
  │                                                                      │
  │   convert::*      total f32→u8/i8/Color32/Margin/CornerRadius        │
  │   fonts::*        FontPlan ──▶ egui::FontDefinitions, row height     │
  │   style_patch     the application's own settings, applied last       │
  │   mapping.toml    one audited row per leaf: verdict + declared sinks │
  │                                                                      │
  │              ┌────────────── ThemeAtlas ──────────────┐              │
  │              │  base Style              × 2 themes    │              │
  │              │  Style per (Role, RoleVariant) × 2     │              │
  │              │  Frame per Surface       × 2 themes    │              │
  │              │  focus ring              × 2 themes    │              │
  │              │  2 × ResolvedTheme, FontDefinitions,   │              │
  │              │  prefs, layout                         │              │
  │              │  name, OS colour mode, notes           │              │
  │              │  icon set, icon theme                  │              │
  │              └────────────────────────────────────────┘              │
  └───────────────────────────────┬──────────────────────────────────────┘
                                  │ install(ctx)
                                  ▼
  ┌──────────────────────────────────────────────────────────────────────┐
  │ egui::Context                                                        │
  │   set_fonts               context.rs:2106   (once, before styles)    │
  │   set_style_of(Dark, ..)  context.rs:2250   app's Name keys kept     │
  │   set_style_of(Light, ..) context.rs:2250   app's Name keys kept     │
  │   data_mut                the atlas itself, for from_ctx and the     │
  │                           Ui extension trait                         │
  │   add_plugin              context.rs:2047   the install plugin       │
  │   request_repaint         context.rs:1821   (last)                   │
  │                                                                      │
  │   install plugin, every pass (egui/src/plugin.rs:32-44):             │
  │     input_hook     OS colour scheme where the integration has none   │
  │     on_end_pass    focus ring around the keyboard-focused widget     │
  │     output_hook    OS title bar on the scheme the UI is drawn in     │
  └───────────────────────────────┬──────────────────────────────────────┘
                                  │
                 ┌────────────────┼──────────────────────┐
                 ▼                ▼                      ▼
        ┌────────────────┐ ┌──────────────┐ ┌───────────────────────────┐
        │ UNSCOPED       │ │ SCOPED       │ │ AREA-BASED CONTAINERS     │
        │ every widget   │ │ ui.native_   │ │ Window / Popup / Tooltip  │
        │ that the app   │ │ scope(role,  │ │ Modal / menus / ComboBox  │
        │ never wraps    │ │ variant, ..) │ │ popups                    │
        │                │ │ ui.native_   │ │                           │
        │ the DIRECT     │ │ set_style    │ │ carriers passed           │
        │ leaves, the    │ │              │ │ explicitly, per container │
        │ base owners    │ │ the SCOPED   │ │ (§3.3) — a scope AROUND   │
        │ among them     │ │ leaves of    │ │ them does NOTHING         │
        │ (§2), and the  │ │ NON-Area     │ │ (area.rs:611-629)         │
        │ base style's   │ │ widgets      │ │                           │
        │ DERIVED values │ │              │ │ the remaining SCOPED      │
        │                │ │              │ │ leaves — surfaces         │
        └────────────────┘ └──────────────┘ └───────────────────────────┘
```

The middle and right columns each carry part of the SCOPED leaves; neither
carries all of them, and the third column's carriers are **not
interchangeable** — §3.2 lists which container accepts which carrier, and §3.3
tabulates the result. Two consequences are architectural:

* **`Window` and `Modal` accept no `StyleModifier`.** Their chrome takes
  `surface_frame(..)`, and their **body** takes a role scope applied as the
  first statement inside the closure (§1.5), which reaches the widgets the
  application adds there but not the chrome computed before the closure runs
  (§14 item 2). In egui 0.36.2 a `Window` given a `frame` and no `title_frame`
  paints its title bar with that same frame
  (`window_title_frame = title_frame.unwrap_or(window_frame)`,
  `containers/window.rs:631`), so `Surface::Window` alone already themes the
  title bar and `Surface::WindowTitleBar` is needed only where the title bar
  differs from the window body ([`Surface::Window`] gives the detail). egui
  0.36.1 fell back to `Frame::window(&style)` of the Context style instead
  (`title_frame.unwrap_or_else(|| Frame::window(&style))` at line 630 of its
  `window.rs`), so a window given a surface frame and no title frame got a title
  bar in the global style's window chrome, not the surface's — the reason the
  requirement is 0.36.2 (§0).
* **Panel chrome is a fourth case no column shows.** A `Panel`'s *contents* are
  an ordinary `Ui` and belong to the middle column, but its fill, stroke, radius
  and margins arrive only as an `egui::Frame` value on `Panel::frame`
  (`containers/panel.rs:413`) or `CentralPanel::frame` (`:1206`) — §3.2's fourth
  carrier — and its separator line is painted from the **parent** `Ui`'s style
  (§4.4 gives the recipe).

### 3.2 The three delivery seams, and exactly what each reaches

egui 0.36.2 offers three places a `Style` can be substituted. All three are
used; none is invented.

| # | Seam | API | Reaches |
|---|---|---|---|
| S1 | Global, per colour scheme | `Context::set_style_of(Theme, impl Into<Arc<Style>>)` (`context.rs:2250`) | everything not overridden by S2 or S3 |
| S2 | Child `Ui` | `UiBuilder::style` (`ui_builder.rs:155`, field `:28`) consumed by `Ui::scope_builder` (`ui.rs:2194`) and `Ui::new` (`ui.rs:136`) / `Ui::new_child` (`ui.rs:237`); also `Ui::set_style` (`ui.rs:387`) and `Ui::style_mut` (`ui.rs:380`) | every widget laid out inside that `Ui`, **except** `Area`-based containers |
| S3 | Container-local modifier | `egui::style::StyleModifier` (`style.rs:194`) via `Popup::style` (`containers/popup.rs:417`), `MenuConfig::style` (`containers/menu.rs:107`), `MenuBar::style` (`:241`), `ComboBox::popup_style` (`containers/combo_box.rs:199`) | the whole `Style` inside one popup or menu, including its own frame |

Those four are the only `StyleModifier` acceptors in egui 0.36.2; `Tooltip`
reaches `Popup::style` through its public `popup` field
(`containers/tooltip.rs:9`). `Window` exposes only `frame`
(`containers/window.rs:265`) and `title_frame` (`:272`), `Modal` only `frame`
(`containers/modal.rs:53`), and `Area` has no style builder.

A fourth carrier is not a `Style` at all: an **`egui::Frame` value**, accepted
by `Panel::frame` (`containers/panel.rs:413`), `CentralPanel::frame` (`:1206`),
`Window::frame` (`containers/window.rs:265`), `Window::title_frame` (`:272`),
`Modal::frame` (`containers/modal.rs:53`), `Popup::frame`
(`containers/popup.rs:369`) and `Frame::show` (`containers/frame.rs:404`).
This is the **only** way to theme container margins, because **five** of egui's
eight `Frame` presets hardcode their inner margin and read no `Style::spacing` —
`group`, `side_top_panel`, `central_panel`, `canvas`, and `dark_canvas`, which
delegates to `canvas` (`containers/frame.rs:236-237`) and so inherits its
`.inner_margin(2)`. Only `window` (`:198`), `menu` (`:207`) and `popup` (`:216`)
read `style.spacing`; 5 + 3 = 8.
All eight take a `&Style` and all eight read *something* from it; the column
that matters is the last one:

| preset | decl | inner margin | what it reads from `Style` |
|---|---:|---|---|
| `Frame::group` | `frame.rs:178` | `.inner_margin(6)` `:180` | `widgets.noninteractive` corner radius `:181` and `bg_stroke` `:182` |
| `Frame::side_top_panel` | `:185` | `Margin::symmetric(8, 2)` `:187` | `panel_fill` `:188` |
| `Frame::central_panel` | `:191` | `.inner_margin(8)` `:192` | `panel_fill` `:192` |
| `Frame::window` | `:196` | `style.spacing.window_margin` `:198` | radius `:199`, shadow `:200`, fill `:201`, stroke `:202` |
| `Frame::menu` | `:205` | `style.spacing.menu_margin` `:207` | radius `:208`, shadow `:209`, fill `:210`, stroke `:211` |
| `Frame::popup` | `:214` | `style.spacing.menu_margin` `:216` | radius `:217`, shadow `:218`, fill `:219`, stroke `:220` |
| `Frame::canvas` | `:227` | `.inner_margin(2)` `:229` | `widgets.noninteractive.corner_radius` `:230`, `extreme_bg_color` `:231`, `window_stroke()` `:232` |
| `Frame::dark_canvas` | `:236` | `.inner_margin(2)`, via `canvas` | as `canvas`, then a hardcoded `Color32::from_black_alpha(250)` fill `:237` |

`Frame::menu` and `Frame::popup` read **the same five fields**
(`menu_margin`, `menu_corner_radius`, `popup_shadow`, `window_fill()`,
`window_stroke()`). There is no `menu_fill`, no `popup_fill`, no
`tooltip_fill`, no `menu_stroke`.

**What no carrier can do, the install plugin does.** Three things the look
depends on are not values a `Style` or a `Frame` can hold — the OS colour
scheme, the focus ring and the title bar's scheme — and all three need
something to run every pass; the plugin's three hooks below do them. egui gives a registered `Plugin` exactly those
moments (`egui/src/plugin.rs:13-52`), and `install` registers one (§10.3). It
holds no theme data: each pass it reads the atlas `install` published into
`Context` data, so a re-install needs no second registration, and
`Context::add_plugin` would drop a second plugin of the same type anyway
(`egui/src/context.rs:2045`, `egui/src/plugin.rs:206-210`).

* **`input_hook`** (`egui/src/plugin.rs:38`), called before the input reaches
  the pass (`egui/src/context.rs:965-968`): where `RawInput::system_theme` is
  `None` it becomes the atlas's OS colour mode. winit 0.30.13 reports `None` on
  Linux (`ActiveEventLoop::system_theme`, lines 909–911 of its
  `src/platform_impl/linux/mod.rs`) and never sends `ThemeChanged` there, so without this hook egui would pick its
  `fallback_theme` on every Linux desktop regardless of the OS setting.
* **`on_end_pass`** (`egui/src/plugin.rs:32`): paints the focus ring around the
  widget holding keyboard focus, from `defaults.focus_ring_*`, at the corner
  radius of the innermost role scope recorded around that widget (§4.5) —
  outside any, the root `Ui`'s style's at the end of the pass — or around the
  outline the widget registered with `register_focus_shape` (§4.3); §6.18
  gives the algorithm.
* **`output_hook`** (`egui/src/plugin.rs:44`), called on the pass's output
  after `Context::sync_window_theme` (`egui/src/context.rs:2456`, `:2464`):
  where `input_hook` supplied the OS mode, it turns egui's
  `SetTheme(SystemDefault)` into `SetTheme(os_mode)`, so the title bar the
  window manager or winit draws follows the same scheme (§10.3).

`Spacing::extra_text_line_spacing` is not among them: it depends only on the
face and the body size the atlas installs, so [`Builder::build`] writes it into
every style (§4.3, §6.15).

`on_end_pass`, and with it the focus ring, runs only inside `Context::run_ui`; the input
and output hooks run under every integration; §10.3 (*Where the hooks run*) says which
integrations call it and what one that does not gets.

### 3.3 The reach table

Which carrier reaches which container, from §1.5 and §3.2. "Around" is a scope
wrapped around the container's call; "inside" is one applied as the first
statement inside its closure.

| Container | Chrome | Body | A scope around it |
|---|---|---|---|
| a plain widget (`Button`, `Label`, `Slider`, …) | — | the base style (S1), or the enclosing role scope (S2) | reaches it |
| `Popup`, `Tooltip`, menus, `ComboBox` popup | `role_modifier` (S3), whose style the popup's own frame preset reads; `surface_frame` on `Popup::frame` | `role_modifier` (S3) | **no effect** (`containers/area.rs:611-629`) |
| `Window`, `Modal` | `surface_frame` on `frame` / `title_frame` | a role scope **inside** the closure (S2) | **no effect**; no `StyleModifier` acceptor exists |
| `Panel`, `CentralPanel` | `surface_frame` on `frame`; the separator line from the parent `Ui`'s style (§4.4) | a role scope inside the closure, or the parent's (S2) | reaches the body and the separator line, but a side panel then shrinks the scope's cursor (`containers/panel.rs:862`) while the parent advances past the whole scope (`ui.rs:2213`); §4.4 styles the parent with `native_set_style` instead |
| `Frame::show` (a card) | `surface_frame` | the enclosing `Ui`'s style (S2) | reaches the body |

The install plugin's focus ring reaches scoped and unscoped widgets alike.
Every SCOPED cell is labelled by kind rather than by count; §5.7 owns the
counts.

### 3.4 Why the per-role styles are pre-built

Every `(Role, RoleVariant, egui::Theme)` cell is compiled once, at
`Builder::build`, into an `Arc<egui::Style>`. Handing one to `UiBuilder::style`
moves the `Arc` — `ui.rs:237` is
`let style = style.unwrap_or_else(|| Arc::clone(&self.style));` — so no `Style`
is copied into the child `Ui`. The caller still pays one refcount bump to
produce the `Arc`. Cells with no `Selected` or `Disabled` data share the
`Normal` `Arc` rather than allocating a duplicate. A cell is never mutated
after `build`: everything it carries, the body text's line spacing included
(§6.15), is known there.

`Style` is never built by exhaustive struct literal: `Style::debug` is
`#[cfg(debug_assertions)]` (`style.rs:323-324`), so a literal would fail to
compile in exactly one of the two profiles. Construction always starts from
`egui::Theme::default_style()` (`memory/theme.rs:24-29`) and assigns. **That
construction is the no-hardcoded-values enforcement**: every field the theme
does not supply — every `None` size of §2 included — keeps egui's own value
by construction, and every numeric literal in the mapping code other than the
three kinds §6.17 names and the exemptions it enumerates is a bug.

Two `Visuals` fields make that rule load-bearing rather than tidy, and neither
is written from theme data. `dark_mode` (`style.rs:995`) and `text_options` (`:1001`)
are written only by `Visuals::dark` (`:1500-1504`) and `Visuals::light`
(`:1567-1571`), never from theme data, so **each colour scheme's style must
start from its own `Theme::default_style()`**. Building the light style by
cloning the dark one — an obvious-looking optimisation, since §2 says the two
differ only in `visuals` — would silently carry `dark_mode: true` and the
dark-mode colour-transfer function into the light theme. Both failures are
invisible: `egui_extras`'s syntax highlighter picks its whole palette from
`style.visuals.dark_mode` (`egui_extras/src/syntax_highlighting.rs:243`, `:280`),
and `Context` copies `text_options` off the global style once per pass
(`context.rs:579-582`). Nothing errors; the text just renders wrong.

**What a cell is built from.** Each colour scheme's base style starts from that
scheme's `Theme::default_style()` and takes every value the base style carries:
the base owners of §5.9, §6.1's states from `theme.button`, the text scaling
and reduced motion of [`Builder::accessibility`] and the line spacing of §6.15.
Every `Normal` role cell of that scheme starts from a copy of this base style
and applies only its role's writes; a `Selected` or `Disabled` cell starts from
its role's `Normal` cell and applies only the variant's (§6.2, §6.3). So every
value the base style carries — `panel_fill`, `window_fill`, `selection`,
`hyperlink_color`, `extreme_bg_color`, the warn and error colours, the
`window_*` fields, the line spacing — is present inside every scope unless the
role writes that field itself: the base style is the platform's look, and a
scope changes only what its widget states differently. The `Role::Menu` cells
add one step before the role's writes: egui's own `menu_style`
(`egui/src/containers/menu.rs:22-29`, public at
`egui::containers::menu::menu_style`, `egui/src/lib.rs:394`), the function
egui applies to every menu by default (`MenuConfig::default`,
`egui/src/containers/menu.rs:83`) and which `role_modifier` replaces. It
sets item padding `vec2(2.0, 0.0)`, no item outlines and a transparent resting
item. The cell then writes the item border, radius and padding native-theme
states — `menu.border` is the item's, not the popup's
(`docs/platform-facts.md:1235-1240`) — over the first two (§5.2, §6.1), and the
transparent resting item stands, since native-theme states no resting item
fill, as for any unstated field (§2); without `menu_style` an unstated
item-padding side and every resting item's fill would be the button's from the
base style. [`Builder::style_patch`]
runs last and on each style separately: a cell starts from the base style as it
stands before the patch (§4.3).

**What a `Surface` frame is built from.** The same rule, for chrome. Each
surface's frame starts from the frame egui itself builds for that container,
over the same colour scheme's base style as it stands before the patch —
`Frame::window` for `Surface::Window` and `Surface::WindowTitleBar`
(`egui/src/containers/window.rs:630-631`), `Frame::popup` for `Dialog`,
`Popover` and `Tooltip` (`egui/src/containers/modal.rs:100`;
`egui/src/containers/popup.rs:603`, which `Tooltip::show` reaches through
`popup.show` at `egui/src/containers/tooltip.rs:142`), `Frame::group` for
`Card` (`egui/src/ui.rs:2148`), `Frame::side_top_panel` for `Panel`
(`egui/src/containers/panel.rs:948-950`) and `Frame::central_panel` for
`CentralPanel` (`:1243`) — and then sets the frame fields its surface's rows
state (§5.2, §5.6), each by its row's formula. Every other field keeps the
preset's value, as an unstated field keeps egui's (§2); because each preset
reads the base style (§3.2's table), such a field carries whatever the base
style holds there. So a surface looks like egui's own container wherever the
theme is silent, and passing its frame never changes more than the theme
states.

---

## 4 -- The complete public API

Every signature below is final and unambiguous. Bodies are elided as
`{ /* … */ }`; trait impls with required methods are written `{ /* … */ }` and
never `{}`, which would be `E0046`. An item this section does not show is
crate-private.

The `///` text in these blocks is the rustdoc, copied into the module the
implementation plan places each item in (`atlas.rs`, `roles.rs`, `ext.rs`,
`accessors.rs`, …), where an intra-doc link resolves from that module and not
from the crate root. So a link to an item outside the item's own module carries
an explicit target — ``[`ResolvedTheme`](crate::ResolvedTheme)`` — and a link to
an item behind a feature (`ThemeWatcher`, feature `watch`;
`FontPlan::from_system`, feature `system-fonts`) is written as plain code, so the
documentation builds with any feature set. Every public item, enum variant and
field carries a `///` line, as `#![warn(missing_docs)]` requires (§4.1).

### 4.1 Crate root — attributes, modules, re-exports

```rust,ignore
//! egui toolkit connector for native-theme.
//!
//! Target: **egui 0.36.2**.

#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::panic)]
#![deny(clippy::unreachable)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]

pub mod convert;
pub mod fonts;
pub mod icons;

// ---- toolkit and source-crate re-exports -----------------------------------
// Both are re-exported so a downstream `Cargo.toml` cannot introduce a second
// copy of either crate. Two `egui` versions in one graph produce
// "expected `egui::Style`, found `egui::Style`", which is the single most
// common downstream failure with a toolkit connector.
pub use egui;
pub use native_theme;

// ---- convenience re-exports ------------------------------------------------
// RULE: the crate root re-exports no name that also exists at `egui`'s root.
// Four native-theme names are therefore deliberately NOT here:
//   * `native_theme::color::Rgba`     -> `convert::Rgba`   (egui/src/lib.rs:442, opposite colour space)
//   * `native_theme::theme::IconData` -> `icons::IconData` (egui/src/viewport.rs:183)
//   * `native_theme::theme::Theme`    -> reachable as `native_theme::theme::Theme`
//                                        (egui/src/lib.rs:483 already exports `Theme`)
//   * `native_theme::SystemTheme`     -> reachable as `native_theme::SystemTheme`
//                                        (egui/src/viewport.rs:1036, re-exported to egui's
//                                         root by `viewport::*` at egui/src/lib.rs:493;
//                                         it resolves as `crate::SystemTheme` inside egui at
//                                         context.rs:252 and :2479)
pub use native_theme::error::Error;
pub use native_theme::theme::{
    AnimatedIcon, ColorMode, DialogButtonOrder, FontStyle, IconProvider, IconRole, IconSet,
    LayoutTheme, ResolvedTheme, ThemeMode, TransformAnimation,
};
pub use native_theme::{AccessibilityPreferences, Result};

#[cfg(target_os = "linux")]
pub use native_theme::detect::LinuxDesktop;
```

`FontStyle` is on the list because `FontPlan::face` and
`FontPlan::variable_face` (§4.9) take one, so every application that supplies
fonts needs it, and because it cannot collide: a recursive grep for `FontStyle`
over `egui-0.36.2/src` and `epaint-0.36.2/src` returns **zero** hits.

Dropping `SystemTheme` from the list does **not** change the four §4.7 accessors
that take `&SystemTheme`; a crate-private `use native_theme::SystemTheme;`
resolves them. Do not re-add the `pub` re-export to make them compile — that is
the E0659 ambiguity the rule exists to prevent, for a downstream that globs both
crates.

**The `deny` list is a house convention, not a proof of the no-panic rule**, and
the gap is worth writing down. Measured on rustc/clippy 1.97.1 with exactly the
five original attributes, only slice indexing is rejected: `BTreeMap` and
`HashMap` `Index`, `unreachable!`, `todo!`, `unimplemented!`, `assert!`,
`assert_eq!`, integer overflow, integer division, a `u128 as u64` cast and
`Duration::from_secs_f64` all compile clean. Three of those are lintable and are
denied above — `clippy::unreachable`, `clippy::todo` and
`clippy::unimplemented`, all three verified to fire (re-measured with the full
list above on 1.97.1 and 1.98.1: the same constructs pass and fail). The remainder is banned by
convention and enforced by review: `assert!`, `assert_eq!` and `debug_assert!`
must not appear, and neither may `BTreeMap`/`HashMap` indexing, because
`clippy::indexing_slicing` does not reach `Index` impls outside arrays, slices
and `Vec`. `deny` rather than `forbid`, so an in-crate test can still `#[allow]`
one. §13's **Lints, MSRV, package** test runs this same list.

Explicit non-adoptions, recorded so they are not added later as an oversight:
`clippy::arithmetic_side_effects` and `clippy::integer_division` as crate
attributes, which fire on every integer operation including provably safe ones
and fire on **no** float arithmetic at all (verified) — and float arithmetic is
where §6's formulas live, whose totality §7.2's helpers establish directly; and
`clippy::missing_panics_doc`, a documentation lint rather than a panic-freedom
proof, which §7.5 argues instead. The first two still run on the library:
the pre-release strict set (`pre-release-check.sh:487-503`), to which this
crate is added as the iced connector is, applies `arithmetic_side_effects`,
`integer_division` and `modulo_arithmetic`, so library integer arithmetic uses
`checked_*` / `saturating_*`.

There is **no** `EGUI_VERSION` constant. A hand-maintained version string
cannot be checked against the resolved dependency — `egui = "0.36.2"` accepts
any 0.36.x from 0.36.2 up — and would eventually become a lie. The version policy lives in
`Cargo.toml` and in §12.

### 4.2 `ThemeAtlas` — the handle

```rust,ignore
/// A complete egui theme compiled from native-theme data.
///
/// `Arc`-backed: cloning is one atomic increment, exactly like [`egui::Context`].
/// `Send + Sync + 'static`, so it can be built on a watcher thread and published into
/// [`egui::Context::data_mut`] (`egui/src/context.rs:1033`). This is the one load-bearing
/// auto-trait claim in the crate, so its ground is named: the atlas holds nothing but
/// `Arc<egui::Style>`, `egui::Frame`, [`Note`], [`AccessibilityPreferences`](crate::AccessibilityPreferences), two
/// [`ResolvedTheme`](crate::ResolvedTheme)s, its [`LayoutTheme`](crate::LayoutTheme) (four `Option<f32>`), its name (`String`), the
/// OS [`ColorMode`](crate::ColorMode) (`Option`), the
/// [`IconSet`](crate::IconSet) and the icon-theme name per `egui::Theme` (`Option<String>` each) it carries, the
/// focus ring per `egui::Theme` (§6.18: an `Option` of an `egui::Stroke` and an `f32` offset),
/// and the `egui::FontDefinitions` [`Builder::build`] made from its [`fonts::FontPlan`](crate::fonts::FontPlan) for
/// [`ThemeAtlas::install`] (none when no plan was given), which are `BTreeMap`s of `String`,
/// `FontFamily` (an `Arc<str>` in `Name`) and `Arc<FontData>` (`epaint/src/text/fonts.rs:431-444`,
/// `:74-95`), a `FontData` being a `Cow<'static, [u8]>`, a `u32` and a `FontTweak` of plain
/// values (`:112-122`, `:208-264`) — [`Builder::style_patch`]'s closure
/// is applied at build and not kept; and a `ResolvedTheme` is plain
/// owned data with no interior mutability and no `Rc`
/// (`native-theme/src/model/resolved.rs:156-213`).
///
/// It carries, per `egui::Theme` (Light and Dark):
/// * one base [`egui::Style`],
/// * one [`egui::Style`] per ([`Role`](crate::Role), [`RoleVariant`](crate::RoleVariant)),
/// * one [`egui::Frame`] per [`Surface`](crate::Surface),
/// * one focus ring, or none where the theme states no ring (§6.18),
///
/// plus the two [`ResolvedTheme`](crate::ResolvedTheme)s it was built from. The styles reach widgets through
/// [`ThemeAtlas::install`] (the base styles), [`NativeThemeUiExt`](crate::NativeThemeUiExt) and
/// [`ThemeAtlas::role_modifier`] (the role styles) and [`ThemeAtlas::surface_frame`] (the
/// frames). No raw role or base `Arc<Style>` is handed out: installed raw, it would drop the
/// application's `TextStyle::Name` keys, on which `TextStyle::resolve` panics in release
/// (`egui/src/style.rs:112-120`, §14 item 30), and every seam of §3.3 is covered by those
/// calls.
#[derive(Clone)]
pub struct ThemeAtlas(std::sync::Arc<AtlasInner>);

impl std::fmt::Debug for ThemeAtlas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { /* … */ }
}

impl ThemeAtlas {
    /// Start building from a light and a dark [`ResolvedTheme`](crate::ResolvedTheme).
    ///
    /// Both are required: egui keeps a separate `Style` per `egui::Theme`
    /// (`egui/src/memory/mod.rs:196`, `:200`). Pass the same value twice if only one exists.
    ///
    /// The `#[must_use]` carries a message because [`Builder`] is itself `#[must_use]`; a bare
    /// one is `clippy::double_must_use`, which §13's **Lints, MSRV, package** test runs as an
    /// error.
    #[must_use = "this starts the builder; call `build()` to produce the atlas"]
    pub fn builder<'a>(
        name: &'a str,
        light: &'a ResolvedTheme,
        dark: &'a ResolvedTheme,
    ) -> Builder<'a>;

    /// Display name.
    #[must_use]
    pub fn name(&self) -> &str;

    /// The source [`ResolvedTheme`](crate::ResolvedTheme) for an `egui::Theme` — pass `ctx.theme()` for the scheme
    /// egui is drawing. Total: every atlas holds both.
    #[must_use]
    pub fn resolved_for(&self, theme: egui::Theme) -> &ResolvedTheme;

    /// The accessibility preferences the atlas was built with, applied as
    /// [`Builder::accessibility`] describes, or `AccessibilityPreferences::default()`
    /// (`native-theme/src/lib.rs:262-271`). Hand it to the text-size accessors of §4.7, so a
    /// size read at a call site matches the size the atlas installed.
    #[must_use]
    pub fn accessibility(&self) -> &AccessibilityPreferences;

    /// The layout spacing the atlas was built with, as [`Builder::layout`] took it, or
    /// `LayoutTheme::default()` (`native-theme/src/model/widgets/mod.rs:890-913`, all four
    /// fields `None`). Read `container_margin` and `section_gap` here, the two per-call values
    /// no `Style` field carries (§5.1), so an atlas from [`from_system`](crate::from_system) needs no second
    /// detection of the OS.
    #[must_use]
    pub fn layout(&self) -> &LayoutTheme;

    /// The OS colour mode the atlas carries: `SystemTheme::mode` on the [`from_system`](crate::from_system) /
    /// [`SystemThemeExt::to_egui_atlas`](crate::SystemThemeExt::to_egui_atlas) path, the value given to [`Builder::os_mode`], or
    /// `None` — [`from_preset`](crate::from_preset) and [`to_theme`](crate::to_theme) carry none. The install plugin feeds it to
    /// egui where the integration reports no OS scheme (§3.2, §10.3).
    #[must_use]
    pub fn os_mode(&self) -> Option<ColorMode>;

    /// The theme's icon set, so icon code — a spinner drawing the set's animated indicator
    /// (`native_theme::icons::load_icon_indicator(set)` for a bundled set; for
    /// `IconSet::Freedesktop`, `FreedesktopLoader::load_indicator(atlas.icon_theme(ctx.theme()))`,
    /// because `load_icon_indicator` asks for the system's theme there,
    /// `native-theme/src/icons.rs:508`), an [`icons::IconKey`](crate::icons::IconKey) — follows the theme
    /// with no second input. On the [`from_preset`](crate::from_preset) path it is `Resolved::icon_set`
    /// (`native-theme/src/model/resolved.rs:259`), which native-theme already falls back to
    /// `system_icon_set()` when the preset states none; on the [`from_system`](crate::from_system) /
    /// [`SystemThemeExt::to_egui_atlas`](crate::SystemThemeExt::to_egui_atlas) path it is `SystemTheme::icon_set`
    /// (`native-theme/src/lib.rs:469`); a [`Builder`] atlas takes [`Builder::icon_set`], else
    /// `native_theme::theme::system_icon_set()` (`native-theme/src/model/icons.rs:512`) — the
    /// same fallback native-theme applies.
    #[must_use]
    pub fn icon_set(&self) -> IconSet;

    /// The freedesktop icon-theme name the theme names for one colour scheme, for
    /// [`icons::IconKey::icon_theme`](crate::icons::IconKey::icon_theme); pass `ctx.theme()` for the scheme egui is drawing.
    /// Per scheme because the variants differ — `kde-breeze` names `breeze` for light and
    /// `breeze-dark` for dark (`native-theme/src/presets/kde-breeze.toml:9`, `:317`) — and
    /// egui draws either style whenever the scheme changes, so one name would be wrong for the
    /// other. On the [`from_preset`](crate::from_preset) path it is each variant's `Resolved::icon_theme`
    /// (`native-theme/src/model/resolved.rs:264`, from `Theme::resolve` of that mode); on the
    /// [`from_system`](crate::from_system) / [`SystemThemeExt::to_egui_atlas`](crate::SystemThemeExt::to_egui_atlas) path it is
    /// `SystemTheme::icon_theme_for` of that mode (§9.2); a [`Builder`] atlas takes
    /// [`Builder::icon_theme`] per scheme. `None` where the theme states none for that scheme
    /// and detection failed; nothing is invented.
    #[must_use]
    pub fn icon_theme(&self, theme: egui::Theme) -> Option<&str>;

    /// One role's style in one appearance variant, packaged as an `egui::style::StyleModifier`
    /// (declared at `egui/src/style.rs:194`; **not** re-exported at egui's root —
    /// `egui/src/lib.rs:488` re-exports only
    /// `style::{FontSelection, Spacing, Style, TextStyle, Visuals}`).
    ///
    /// Feed to `Popup::style` (`egui/src/containers/popup.rs:417`), `MenuConfig::style`
    /// (`egui/src/containers/menu.rs:107`), `MenuBar::style` (`:241`) or
    /// `ComboBox::popup_style` (`egui/src/containers/combo_box.rs:199`) — the only four
    /// acceptors in egui 0.36.2 (§3.2); `Tooltip` reaches `Popup::style` through its public
    /// `popup` field (`egui/src/containers/tooltip.rs:9`).
    ///
    /// For menus this is required, not convenient: without it a menu's style is the
    /// `Context`'s passed through egui's `menu_style` (`egui/src/containers/menu.rs:22`),
    /// which overwrites `spacing.button_padding` (`:23`), **four** `bg_stroke`s — `active`
    /// (`:24`), `open` (`:25`), `hovered` (`:26`), `inactive` (`:28`) — and
    /// `widgets.inactive.weak_bg_fill` (`:27`) and carries none of the menu's own values;
    /// the `Role::Menu` cell applies the same function and then those values (§3.4). It is
    /// also the menu's chrome: egui builds every menu's frame from the style
    /// the modifier produces ([`Surface`](crate::Surface) gives the detail), so the fill, stroke, radius,
    /// shadow and margins of a menu are the ones the `Role::Menu` cell inherits from the base
    /// style, and its items' border, radius and padding the cell's own (§5.2).
    ///
    /// The modifier replaces the whole `Style`, like `impl From<Style> for StyleModifier`
    /// (`egui/src/style.rs:211-215`), except that every `TextStyle::Name` key of the style it
    /// receives is carried into the replacement — the closure is handed that style
    /// (`egui/src/style.rs:194`, `:225-229`), so this costs nothing when there is none. Its
    /// closure captures one `Arc<Style>`.
    #[must_use]
    pub fn role_modifier(
        &self,
        theme: egui::Theme,
        role: Role,
        variant: RoleVariant,
    ) -> egui::style::StyleModifier;

    /// The [`egui::Frame`] recipe for one container surface.
    ///
    /// Feed to `Panel::frame` (`egui/src/containers/panel.rs:413`), `CentralPanel::frame`
    /// (`:1206`), `Window::frame` (`egui/src/containers/window.rs:265`), `Window::title_frame`
    /// (`:272`), `Modal::frame` (`egui/src/containers/modal.rs:53`), `Popup::frame`
    /// (`egui/src/containers/popup.rs:369`) or `Frame::show`
    /// (`egui/src/containers/frame.rs:404`).
    ///
    /// This is the **only** way to theme container margins: five of egui's eight `Frame`
    /// presets hardcode their inner margin and read no `Style::spacing` (§3.2).
    #[must_use = "this returns the frame recipe; hand it to a container's `.frame(..)`"]
    pub fn surface_frame(&self, theme: egui::Theme, surface: Surface) -> egui::Frame;

    /// Non-fatal observations made while compiling this atlas: sanitised values, saturated
    /// margins, font requests that could not be met. Inspect or log; never match exhaustively.
    #[must_use]
    pub fn notes(&self) -> &[Note];

    /// Install into an [`egui::Context`]: the atlas's fonts, both colour schemes' base styles
    /// (keeping the application's own `TextStyle::Name` keys), the atlas into `Context` data,
    /// the install plugin, the icon-cache flush and a repaint — §10.3 lists the steps, their
    /// order and what each must not touch. The plugin supplies the OS colour scheme where the
    /// integration reports none, paints the focus ring (§6.18) and keeps the OS title bar on the
    /// scheme the UI is drawn in (§10.3).
    ///
    /// Run it on every application start, even when egui memory is persisted:
    /// `Options::dark_style` and `light_style` are `#[serde(skip)]`
    /// (`egui/src/memory/mod.rs:195`, `:199`). Safe to call at any time, including inside a
    /// pass; the new theme reaches the whole UI on the next pass.
    ///
    /// **The colour-scheme choice stays egui's.** `install` never touches
    /// `Options::theme_preference`, which is serde-persisted (`egui/src/memory/mod.rs:206`
    /// has no `serde(skip)`), so the user's in-app Light/Dark choice survives a restart.
    /// Follow the OS with `ctx.set_theme(egui::ThemePreference::System)`; pin a scheme with
    /// `ctx.set_theme(egui::ThemePreference::Dark)` or `Light`. The trap: `Context::set_theme`
    /// takes `impl Into<ThemePreference>` (`egui/src/context.rs:2170`) and
    /// `From<Theme> for ThemePreference` exists (`egui/src/memory/theme.rs:79-86`), so
    /// `ctx.set_theme(egui::Theme::Dark)` compiles and *pins* dark — it never means "follow".
    /// "The OS" is what the integration reports in `RawInput::system_theme` and, where it
    /// reports `None` (Linux under winit 0.30.13), this atlas's [`ThemeAtlas::os_mode`].
    ///
    /// `Options::fallback_theme` (`egui/src/memory/mod.rs:212`, default `Theme::Dark` at
    /// `:331`) and `Options::sync_window_theme` (`:229`, default `true` at `:333`) are egui's
    /// too and `install` leaves them alone: set them with
    /// `ctx.options_mut(|o| o.fallback_theme = ..)` (`egui/src/context.rs:1135`).
    pub fn install(&self, ctx: &egui::Context);

    /// The atlas most recently published into this `Context` by [`ThemeAtlas::install`], or
    /// `None` when none is (never installed, or removed by [`ThemeAtlas::clear`]).
    #[must_use]
    pub fn from_ctx(ctx: &egui::Context) -> Option<Self>;

    /// Remove the atlas from `Context` data (`IdTypeMap::remove`,
    /// `egui/src/util/id_type_map.rs:572`). The install plugin then finds nothing and does
    /// nothing, and [`NativeThemeUiExt`](crate::NativeThemeUiExt) degrades as it does when nothing was installed. The
    /// `Style`s already written into `Options` are left alone; call
    /// `ctx.set_style_of(t, t.default_style())` (`egui/src/memory/theme.rs:24-29`) for each
    /// `egui::Theme` `t`, and `ctx.set_fonts(egui::FontDefinitions::default())` if the atlas
    /// installed a font plan, to get stock egui back.
    pub fn clear(ctx: &egui::Context);
}
```

### 4.3 `Builder`, `Note`, `register_focus_shape`

```rust,ignore
/// Additive builder for [`ThemeAtlas`]. Every optional input is a method here, so a future
/// optional input never changes an existing signature.
#[must_use = "call `build()` to produce the atlas"]
pub struct Builder<'a> { /* private */ }

impl<'a> Builder<'a> {
    /// Layout spacing. The atlas writes `widget_gap` into `spacing.item_spacing` (§6.16) and
    /// `window_margin` into [`Surface::CentralPanel`](crate::Surface::CentralPanel)'s inner margin; a `None` leaves that egui
    /// value at its stock default. `container_margin` and `section_gap` have no `Style` field;
    /// the application reads them per call from [`ThemeAtlas::layout`], which keeps the
    /// layout given here — a nested container's `Frame::inner_margin` and `Ui::add_space`
    /// (§5.1).
    ///
    /// A separate input because [`LayoutTheme`](crate::LayoutTheme) lives on `native_theme::theme::Theme`
    /// (`native-theme/src/model/mod.rs:269`) and on `SystemTheme`
    /// (`native-theme/src/lib.rs:488`), and **not** on [`ResolvedTheme`](crate::ResolvedTheme)
    /// (`native-theme/src/model/resolved.rs:156-213`). All four fields are `Option<f32>`
    /// (`native-theme/src/model/widgets/mod.rs:896-913`).
    pub fn layout(self, layout: &'a LayoutTheme) -> Self;

    /// Accessibility preferences, from `SystemTheme::accessibility`
    /// (`native-theme/src/lib.rs:490`) or, on the preset path,
    /// `AccessibilityPreferences::from_system()` (`native-theme/src/lib.rs:287`, `:300`).
    /// Applied wherever egui has a sink, as the sibling connectors apply them: gpui scales its
    /// theme's font sizes in `to_theme` (`connectors/native-theme-gpui/src/lib.rs:188`, `:190`)
    /// and forwards reduced motion to gpui (`:760`); iced scales through `font_size` and
    /// `mono_font_size` (`connectors/native-theme-iced/src/lib.rs:438-443`, `:457-462`).
    ///
    /// * **Text scaling.** Every text size the atlas writes — each `Style::text_styles` entry
    ///   and each `Style::override_font_id` size, in the base styles and in every role style —
    ///   is [`scaled_text_size`](crate::scaled_text_size) of the theme's size. No other length is scaled.
    /// * **Reduced motion.** With `reduce_motion` set, every style gets
    ///   `Style::animation_time = 0.0` (`egui/src/style.rs:318`; egui's default `0.2` at
    ///   `:1440`) and `Style::scroll_animation = ScrollAnimation::none()` (`:338`, `:858`).
    ///   `animation_time` is read through `Context::global_style()` — by
    ///   `Context::animate_bool` and `animate_bool_with_easing` (`egui/src/context.rs:3192`,
    ///   `:3208`) and by the `Area` fade-in (`egui/src/containers/area.rs:638`). On egui 0.36.2
    ///   a zero time makes a value animation return its target at once
    ///   (`egui/src/animation_manager.rs:88-93`), and a bool animation's `elapsed / 0.0` is
    ///   non-finite and snaps to its end value (`:56-60`). `scroll_animation` is read from the
    ///   `Ui`'s own style (`egui/src/ui.rs:1401`, `:1443`, `:1492`), which is why the role
    ///   styles carry it too.
    ///
    /// `high_contrast` and `reduce_transparency` are not applied; they are exposed through
    /// [`is_high_contrast`](crate::is_high_contrast) and [`is_reduced_transparency`](crate::is_reduced_transparency). gpui drops its modal scrim under
    /// `reduce_transparency` (`connectors/native-theme-gpui/src/colors.rs:514-523`); egui's
    /// counterpart, the `Modal` backdrop, is no `Style` field but a per-call
    /// `Modal::backdrop_color` (`egui/src/containers/modal.rs:62`, default
    /// `Color32::from_black_alpha(100)` at `:29`), so that choice is the application's at the
    /// call site. Without this call the atlas is built with
    /// `AccessibilityPreferences::default()`: factor `1.0`, motion on.
    pub fn accessibility(self, prefs: &'a AccessibilityPreferences) -> Self;

    /// Font bytes. Without a plan the atlas maps font **sizes** only and leaves the
    /// `Context`'s fonts as they are. For the OS's own typefaces pass
    /// `fonts::FontPlan::from_system`'s plan (feature `system-fonts`), whose lookup notes
    /// then join [`ThemeAtlas::notes`].
    pub fn fonts(self, plan: fonts::FontPlan) -> Self;

    /// The OS colour mode, which the install plugin feeds to egui wherever the integration
    /// reports none — on Linux always, since winit 0.30.13 reports none there (§3.2).
    /// [`from_system`](crate::from_system) and [`SystemThemeExt::to_egui_atlas`](crate::SystemThemeExt::to_egui_atlas) set it from `SystemTheme::mode`.
    /// An atlas built from a preset has none unless given one here; the OS's current mode is
    /// `if native_theme::detect::system_is_dark() { ColorMode::Dark } else { ColorMode::Light }`
    /// (`native-theme/src/detect.rs:141`), which reads `Light` wherever detection fails
    /// (`:139`). Without this call a preset-built atlas leaves the choice to egui's
    /// `Options::fallback_theme` on Linux.
    pub fn os_mode(self, mode: ColorMode) -> Self;

    /// The icon set [`ThemeAtlas::icon_set`] reports. Without this call:
    /// `native_theme::theme::system_icon_set()`. [`from_preset`](crate::from_preset) and [`from_system`](crate::from_system) pass the
    /// theme's own.
    pub fn icon_set(self, set: IconSet) -> Self;

    /// The icon-theme name [`ThemeAtlas::icon_theme`] reports for `theme`; call it once per
    /// scheme. Without this call for a scheme: `None` for that scheme. [`from_preset`](crate::from_preset) and
    /// [`from_system`](crate::from_system) pass each variant's own when it has one.
    pub fn icon_theme(self, theme: egui::Theme, name: &'a str) -> Self;

    /// An application-owned adjustment applied **last** to every style the atlas holds —
    /// both base styles and every role style, after all theme data — for settings that are
    /// the application's, not the theme's (`Style::interaction`, `Style::debug` in a debug build, a
    /// `TextStyle::Name` key of its own, a `Spacing` value it wants fixed). It runs once per
    /// style at [`Builder::build`] and is not kept, so a `ThemeWatcher` rebuild keeps it only
    /// because the application's `rebuild` closure sets it again, as every builder input is
    /// set again. It can override a native value, deliberately: that is the application's
    /// decision, and nothing here second-guesses it. An application that installs fonts of
    /// its own instead of a plan sets `Spacing::extra_text_line_spacing` for them here. A
    /// second call replaces the first.
    pub fn style_patch(self, f: impl Fn(&mut egui::Style) + 'a) -> Self;

    /// Build. Infallible: a [`ResolvedTheme`](crate::ResolvedTheme) is complete by construction and every numeric
    /// conversion in this crate is total (see [`convert`](crate::convert)). Every face of the font plan is
    /// validated here, once, with the parse epaint will make ([`fonts::font_definitions`](crate::fonts::font_definitions));
    /// each rejected face is a [`Note::FontDataInvalid`] in [`ThemeAtlas::notes`], and the
    /// `egui::FontDefinitions` it returns are kept for [`ThemeAtlas::install`] (§10.3).
    ///
    /// Also computed here, per variant, is the body text's `Spacing::extra_text_line_spacing`
    /// (`egui/src/style.rs:424`), written into both base styles, which every role style
    /// starts from (§3.4): the
    /// theme's line box, `defaults.line_height` × the scaled `defaults.font.size`, less the row
    /// height epaint gives the `Proportional` face the atlas installs — the plan's face, or, for
    /// a plan with no face, the head of its base's chain ([`fonts::FontPlan::with_base`](crate::fonts::FontPlan::with_base); egui's
    /// `Ubuntu-Light` for an empty plan) and, with no plan, the head of
    /// `egui::FontDefinitions::default()`'s chain, egui's `Ubuntu-Light`, the face a `Context`
    /// keeps unless the application replaces it ([`Builder::style_patch`]) — at that size,
    /// computed from the face's own metrics exactly as epaint does
    /// (`epaint/src/text/font.rs:397-400`, `:561-565`, `:587`), and floored at egui's `0.0`
    /// (§6.15). The same row height sets the `Role::Slider` cells' `expansion`, which keeps
    /// the slider knob at `slider.thumb_diameter` when the Body row outgrows it (§6.6). The
    /// row height does not depend on `pixels_per_point`
    /// (`epaint/src/text/font.rs:561-565` scales by the font size alone), so no zoom or DPI
    /// change invalidates either value.
    #[must_use = "this builds the styles; it does not install them"]
    pub fn build(self) -> ThemeAtlas;
}

/// A non-fatal observation made while compiling a [`ThemeAtlas`]. Six variants.
///
/// `#[non_exhaustive]` on the *enum* keeps adding a variant non-breaking; it does **not**
/// protect a variant's payload, which stays exhaustively patternable downstream (verified with
/// a two-crate probe on rustc 1.97.1). Since the diagnostics channel is the surface most
/// likely to be enriched, each struct variant carries its own `#[non_exhaustive]`, so a
/// downstream `match` must spell `{ path, .. }` and adding a field stays additive.
/// [`Surface::Panel`](crate::Surface::Panel) and both [`fonts::FontBytes`](crate::fonts::FontBytes) tuple variants are deliberately left
/// exhaustive: `#[non_exhaustive]` on a *tuple* variant makes it unconstructible outside this
/// crate (`E0603`, verified), and constructing them is the documented call shape.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Note {
    /// A theme value was non-finite, or a text size not a positive normal `f32`, and was
    /// replaced by the documented fallback.
    /// `path` is the native-theme field path, e.g. `"button.border.corner_radius"`.
    #[non_exhaustive]
    ValueSanitised {
        /// The native-theme leaf path whose value was replaced (§7.2: the leaf, never the egui sink).
        path: &'static str,
    },
    /// A length saturated at the `i8` bound of an epaint type — in practice
    /// `epaint::Margin`, the only `i8` sink this crate writes from theme data (§6.14 leaves
    /// all shadow geometry to egui). **Real data loss.** `u8` saturation — corner radii — is
    /// benign and is deliberately **not** reported; see §7.2.
    #[non_exhaustive]
    ValueSaturated {
        /// The native-theme leaf path whose length saturated.
        path: &'static str,
    },
    /// The theme asked for a font family for which the plan holds no bytes — or, from
    /// `fonts::FontPlan::from_system`, which the OS has no face of under that name (compared
    /// case-insensitively, §8.2).
    /// egui's own face stays in that family's place.
    #[non_exhaustive]
    FontFamilyUnavailable {
        /// The family the theme asked for.
        family: std::sync::Arc<str>,
    },
    /// The face chosen for a family has no `wght` axis — `FontData::variation_axes()`
    /// (`epaint/src/text/fonts.rs:153-173`) reports none — and either its own weight differs
    /// from the one the theme asked for, or it was registered with
    /// [`fonts::FontPlan::variable_face`](crate::fonts::FontPlan::variable_face). The face renders at its own weight (§8.2).
    #[non_exhaustive]
    FontWeightAxisUnsupported {
        /// The family the face was chosen for.
        family: std::sync::Arc<str>,
    },
    /// A native colour with alpha `0` was written, as given, to a `WidgetVisuals::bg_fill`,
    /// which egui documents as "Must never be `Color32::TRANSPARENT`"
    /// (`egui/src/style.rs:1292-1294`). Nothing is substituted — a substitute would be a
    /// colour no platform stated — so the widget paints no background in that state (§6.4).
    #[non_exhaustive]
    TransparentFill {
        /// The native-theme leaf path of the colour with alpha `0`.
        path: &'static str,
    },
    /// A registered face's bytes failed the parse epaint makes when it loads a face —
    /// `skrifa::FontRef::from_index(data, index)`, the only fallible step of epaint's
    /// `FontFace::new` (`epaint/src/text/font.rs:386-388`), whose failure epaint turns into a
    /// panic in release builds too (`epaint/src/text/fonts.rs:990`) — or they parsed with a zero
    /// `unitsPerEm`, which epaint divides by (§8.2). The face was dropped and
    /// the family falls back to the next face in its chain, egui's own if no other. `family`
    /// is the name the face was registered under.
    #[non_exhaustive]
    FontDataInvalid {
        /// The family name the face was registered under.
        family: std::sync::Arc<str>,
    },
}

/// Tell the install plugin the outline of widget `id`'s focus ring for **this pass**, when
/// it is not a rounded rectangle around the widget's own rect — a switch's track inside a
/// widget that also holds its label, a composite whose focus belongs to one part. A widget
/// whose outline is its rect needs no call: the ring takes the corner radius of the
/// innermost role scope around it ([`NativeThemeUiExt::native_scope`](crate::NativeThemeUiExt::native_scope)), or outside any the
/// radius of the root `Ui`'s style at the end of the pass, normally the base style's (§6.18).
/// `rect` is the shape the ring surrounds (the ring is still offset from it by
/// `defaults.focus_ring_offset`) and `corner_radius` its radius. Call it after the
/// widget's `Response` exists, in the same pass, from outside any other `Context` accessor
/// closure (the lock discipline of §10.3).
///
/// Stored in the viewport's one temporary focus entry (`IdTypeMap::insert_temp`,
/// `egui/src/util/id_type_map.rs:425`), keyed `Id::new(("native-theme-egui/focus",
/// ctx.viewport_id()))` (`egui/src/context.rs:3949`), beside §6.18's scope records and that
/// viewport's `ctx.cumulative_pass_nr()` (`egui/src/context.rs:1781-1793`). The first registration of a new
/// pass empties the entry's lists, and the plugin reads them only in the pass they hold, so
/// a shape registered in an earlier pass — by a widget no longer drawn, or drawn elsewhere —
/// is ignored, and the entry never holds more than one pass's shapes. A `rect` that is not finite (`Rect::is_finite`,
/// `emath/src/rect.rs:524`) is not stored. Needs no atlas: without an install plugin the
/// entry is simply never read.
pub fn register_focus_shape(
    ctx: &egui::Context,
    id: egui::Id,
    rect: egui::Rect,
    corner_radius: egui::CornerRadius,
);
```

### 4.4 `Role`, `RoleVariant`, `Surface`, `PanelSide`

```rust,ignore
/// A widget role: the *content* style for one kind of widget.
///
/// **Exactly one variant per widget field of [`ResolvedTheme`](crate::ResolvedTheme)**
/// (`native-theme/src/model/resolved.rs:164-212`), in declaration order. That is the whole
/// rule. When native-theme grows a widget, this enum grows one variant and `mapping.toml`
/// grows one section; nothing else in this crate's API changes.
///
/// Role names are **native-theme's vocabulary**: each one is a `ResolvedTheme` field name.
/// [`Surface`] is the other axis — container *chrome* — and its names are **attachment
/// points**, not egui type names: egui 0.36.2 has no `Dialog`, `Popover` or `Card` type at all
/// (a recursive grep over `egui-0.36.2/src` returns one hit, `egui/src/viewport.rs:998`, an
/// unrelated window-type hint), and five of `Surface`'s eight names — `Window`, `Dialog`,
/// `Popover`, `Tooltip`, `Card` — are spelled identically to a `Role` variant. The two axes are
/// separate because they track two different moving sides, not because they use different
/// vocabularies.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Role {
    /// The window: `ResolvedTheme::window`.
    Window,
    /// The button: `ResolvedTheme::button`.
    Button,
    /// The input: `ResolvedTheme::input`.
    Input,
    /// The checkbox: `ResolvedTheme::checkbox`.
    Checkbox,
    /// The menu: `ResolvedTheme::menu`.
    Menu,
    /// The tooltip: `ResolvedTheme::tooltip`.
    Tooltip,
    /// The scrollbar: `ResolvedTheme::scrollbar`.
    Scrollbar,
    /// The slider: `ResolvedTheme::slider`.
    Slider,
    /// The progress bar: `ResolvedTheme::progress_bar`.
    ProgressBar,
    /// The tab: `ResolvedTheme::tab`.
    Tab,
    /// The sidebar: `ResolvedTheme::sidebar`.
    Sidebar,
    /// The toolbar: `ResolvedTheme::toolbar`.
    Toolbar,
    /// The status bar: `ResolvedTheme::status_bar`.
    StatusBar,
    /// The list: `ResolvedTheme::list`.
    List,
    /// The popover: `ResolvedTheme::popover`.
    Popover,
    /// The splitter: `ResolvedTheme::splitter`.
    Splitter,
    /// The separator: `ResolvedTheme::separator`.
    Separator,
    /// The switch: `ResolvedTheme::switch`.
    Switch,
    /// The dialog: `ResolvedTheme::dialog`.
    Dialog,
    /// The spinner: `ResolvedTheme::spinner`.
    Spinner,
    /// The combo box: `ResolvedTheme::combo_box`.
    ComboBox,
    /// The segmented control: `ResolvedTheme::segmented_control`.
    SegmentedControl,
    /// The card: `ResolvedTheme::card`.
    Card,
    /// The expander: `ResolvedTheme::expander`.
    Expander,
    /// The link: `ResolvedTheme::link`.
    Link,
}

impl Role {
    /// Every variant known to this build, in declaration order. Provided because the enum is
    /// `#[non_exhaustive]` and callers cannot write their own exhaustive list.
    #[must_use] pub fn all() -> &'static [Self];
    /// The stable identifier used in `mapping.toml` and the generated `src/mapping.md`, e.g.
    /// `"combo_box"`. Equals the corresponding [`ResolvedTheme`](crate::ResolvedTheme) field name.
    #[must_use] pub fn key(self) -> &'static str;
}

/// Which appearance of a [`Role`] to use.
///
/// This is a *value*, not a taxonomy: `RoleVariant::Selected` is passed as data
/// (`ui.native_scope(Role::Checkbox, RoleVariant::from_selected(self.notify), ..)`),
/// so widget state stays where the state already lives and a mis-typed variant cannot silently
/// invert a condition.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum RoleVariant {
    /// Resting appearance.
    #[default]
    Normal,
    /// The checked appearance of a `Checkbox` or `RadioButton`, which takes no selected flag
    /// of its own: `checkbox.checked_background` and `checkbox.border.color` (§6.2). A
    /// button's suggested action, the active tab and segment and the switched-on switch are
    /// carried by their `Normal` cells, which the `Button`'s own `.selected(..)` flag picks
    /// from; their `Selected` cell is the `Normal` one.
    Selected,
    /// The platform's disabled appearance, written into the `inactive` entry with
    /// `Visuals::disabled_alpha` neutralised to `1.0`. See §6.3.
    Disabled,
}

impl RoleVariant {
    /// `Selected` when `selected`, `Normal` otherwise.
    #[must_use] pub const fn from_selected(selected: bool) -> Self;
    /// Every variant known to this build, in declaration order, for the same reason as
    /// [`Role::all`].
    #[must_use] pub fn all() -> &'static [Self];
    /// The stable identifier a `mapping.toml` sink's `variant` names (§13.1): `"normal"`,
    /// `"selected"`, `"disabled"`.
    #[must_use] pub fn key(self) -> &'static str;
}

/// A container surface: the *chrome* (fill, stroke, corner radius, margins, shadow) of one
/// egui container.
///
/// A surface's frame is egui's own default frame for that container, built from the base
/// style of the same colour scheme, with the fields the theme states for that surface set on
/// it; every field the theme does not state keeps the preset's value (§3.4). The preset for
/// each surface is the one [`NativeThemeUiExt::native_frame`](crate::NativeThemeUiExt::native_frame) names.
///
/// The rule: **exactly one variant per container chrome in egui 0.36.2 to which an
/// application can attach an [`egui::Frame`]**. `Popover` and `Tooltip`
/// are two chromes behind one attachment point, `Popup::frame`, fed from different native
/// widgets. Two `Frame` setters are deliberately not surfaces, because they belong to widgets,
/// not containers: `TextEdit::frame` (`egui/src/widgets/text_edit/builder.rs:306`), whose
/// custom frame replaces egui's whole per-state painting — no hover or focus stroke
/// (`:734-735`) — and `AtomLayout::frame` (`egui/src/atomics/atom_layout.rs:112`), the
/// layout engine `Button` and `TextEdit` build on. egui has one `Panel` type
/// (`egui/src/containers/panel.rs:206`) with
/// four constructors (`left` `:249`, `right` `:256`, `top` `:265`, `bottom` `:274`), so the
/// panel case is one variant carrying the side. `egui::SidePanel` and `egui::TopBottomPanel`
/// **do not exist** in 0.36.2 and must never be named in this crate or its docs.
///
/// Menu chrome has no variant, because no menu builder takes a frame: `MenuButton`,
/// `SubMenuButton` and `MenuBar` have no `.frame(..)`, and `MenuConfig` carries no frame field
/// (`egui/src/containers/menu.rs:65-76`). egui builds the frame itself, after applying the
/// menu's `StyleModifier`: a top-level menu is a `Popup::menu` with no frame (`:325-331`), so
/// `Popup::show` applies the modifier and then builds `Frame::popup(ui.style())`
/// (`egui/src/containers/popup.rs:602-603`); a submenu's frame is `Frame::menu(ui.style())` of
/// the parent menu's already-modified `Ui` (`egui/src/containers/menu.rs:432`), handed to
/// `Popup::frame` (`:509`). Both presets read the same five `Style` fields (§3.2), and the
/// base style carries all five for every menu, which the `Role::Menu` cell inherits:
/// `window_fill` from `menu.background_color`, `window_stroke`, `menu_corner_radius` and
/// the `popup_shadow` gate from `defaults.border`, and egui's own `menu_margin` (§5.9).
/// What [`ThemeAtlas::role_modifier`](crate::ThemeAtlas::role_modifier) for `Role::Menu`, fed to `MenuConfig::style`
/// (`:107`), adds is the items' look: `menu.border`'s item border, radius and padding over
/// `menu_style`'s (§5.2, §6.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Surface {
    /// `Window::frame` (`egui/src/containers/window.rs:265`).
    ///
    /// A `Window` given this frame and **no** `title_frame` paints its title bar with the same
    /// frame: `title_frame.unwrap_or(window_frame)` (`egui/src/containers/window.rs:630-631`),
    /// taken before the window's own inner margin is zeroed (`:634-635`). The title bar then
    /// uses this frame's inner margin as its padding (`:1352`) and paints its fill, stroke and
    /// corner radius without the shadow (`:1425`), the fill replaced by
    /// `widgets.open.weak_bg_fill` on the top-most window (`:1426-1428`) and the bottom corners
    /// squared while the window is expanded (`:1429-1432`). egui 0.36.1 fell back to
    /// `Frame::window(&style)` instead, which is one reason this crate requires 0.36.2. Hand
    /// [`Surface::WindowTitleBar`] to `title_frame` to give the title bar its own chrome.
    Window,
    /// `Window::title_frame` (`egui/src/containers/window.rs:272`). Without it the title bar
    /// takes the window's frame; see [`Surface::Window`].
    WindowTitleBar,
    /// `Modal::frame` (`egui/src/containers/modal.rs:53`).
    Dialog,
    /// `Popup::frame` (`egui/src/containers/popup.rs:369`).
    Popover,
    /// The manual tooltip path: `Tooltip::popup` is a public field
    /// (`egui/src/containers/tooltip.rs:9`), so the frame is set in place and the tooltip
    /// still shown through `Tooltip::show` (`:101`), which does the tooltip bookkeeping:
    /// `let mut t = Tooltip::for_enabled(&r); t.popup = t.popup.frame(..); t.show(..)`.
    /// **`for_enabled`** (`:53-59`), which opens the popup only while the enabled widget is
    /// hovered long enough (`should_show_tooltip`, `:57`) — never `for_widget` (`:39`), which
    /// is "Always open (as long as this function is called)" (`:38`) and would pin the tooltip
    /// on screen. `for_disabled` (`:62`) is the same for a disabled widget.
    /// `Response::on_hover_text` does **not** take a frame — see §14 item 3.
    Tooltip,
    /// `Frame::show` (`egui/src/containers/frame.rs:404`) — the frame-taking spelling of
    /// `Ui::group`, which builds `Frame::group(self.style())` itself and takes no frame
    /// (`egui/src/ui.rs:2147-2148`).
    Card,
    /// `Panel::{left,right,top,bottom}(..).frame(..)` (`egui/src/containers/panel.rs:413`).
    Panel(PanelSide),
    /// `CentralPanel::frame` (`egui/src/containers/panel.rs:1206`): `defaults` chrome with
    /// `layout.window_margin` ([`Builder::layout`](crate::Builder::layout)) as its inner margin on all four sides —
    /// the gap between the window's edge and its content, which is what the application's
    /// `CentralPanel` is. Where `window_margin` is `None` (§2, *Unstated sizes*) the inner
    /// margin stays `Frame::central_panel`'s own `8` (`egui/src/containers/frame.rs:192`).
    CentralPanel,
}

/// Which side a `Panel` is anchored to. Exhaustive on purpose: four sides is a closed fact of
/// 2-D screen geometry, not an API taxonomy that can churn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PanelSide {
    /// Anchored to the left edge (`Panel::left`).
    Left,
    /// Anchored to the right edge (`Panel::right`).
    Right,
    /// Anchored to the top edge (`Panel::top`).
    Top,
    /// Anchored to the bottom edge (`Panel::bottom`).
    Bottom,
}

impl Surface {
    /// Every surface known to this build, panel sides expanded — 11 entries.
    #[must_use] pub fn all() -> &'static [Self];
    /// The stable identifier a `mapping.toml` sink's `surface` names (§13.1), one per entry
    /// of [`Surface::all`]: `"window"`, `"window_title_bar"`, `"dialog"`, `"popover"`,
    /// `"tooltip"`, `"card"`, `"panel_left"`, `"panel_right"`, `"panel_top"`,
    /// `"panel_bottom"`, `"central_panel"`.
    #[must_use] pub fn key(self) -> &'static str;
}
```

**Panel-side to native-struct convention** (documented, never inferred):
`PanelSide::Left` and `PanelSide::Right` are fed from `theme.sidebar`,
`PanelSide::Top` from `theme.toolbar`, `PanelSide::Bottom` from
`theme.status_bar`, and `Surface::CentralPanel` from `theme.defaults` plus
`LayoutTheme::window_margin` for its inner margin.

**The panel recipe.** A `Panel` paints its separator line from the **parent**
`Ui`'s style — `widgets.active.fg_stroke` while it is dragged, `hovered.fg_stroke`
while the pointer is on its edge, and otherwise `noninteractive.bg_stroke`
(`egui/src/containers/panel.rs:906`, `:908`, `:911`) — and builds its content
`Ui` as a child of that parent (`:816`), inheriting its style. So the role that
owns the line must be live on the parent **before** `Panel::show`, set with
`native_set_style` rather than a scope around the call: a side panel shrinks the
cursor of the `Ui` it is shown in (`:862`), and a scope's parent then advances
past the whole scope (`egui/src/ui.rs:2213`), which would push the
`CentralPanel` below the side panel. Outer panels come first, as egui lays them
out before the central one:

```rust,ignore
use egui::{CentralPanel, Panel};
// A fixed (non-resizable) panel: its line and its content in its own role.
ui.native_set_style(Role::Toolbar, RoleVariant::Normal);
Panel::top("toolbar")
    .frame(ui.native_frame(Surface::Panel(PanelSide::Top)))
    .show(ui, |ui| {
        // Again inside: the content `Ui` inherits the role, but the focus ring
        // reads a record, and the root's never matches (§6.18).
        ui.native_set_style(Role::Toolbar, RoleVariant::Normal);
        /* toolbar content */
    });
ui.native_set_style(Role::StatusBar, RoleVariant::Normal);
Panel::bottom("status")
    .frame(ui.native_frame(Surface::Panel(PanelSide::Bottom)))
    .show(ui, |ui| {
        ui.native_set_style(Role::StatusBar, RoleVariant::Normal);
        /* status content */
    });
// A resizable panel: its hover and drag line is the splitter's; its content is
// the sidebar's, set as the first statement inside the closure.
ui.native_set_style(Role::Splitter, RoleVariant::Normal);
Panel::left("nav")
    .resizable(true)
    .frame(ui.native_frame(Surface::Panel(PanelSide::Left)))
    .show(ui, |ui| {
        ui.native_set_style(Role::Sidebar, RoleVariant::Normal);
        /* sidebar content */
    });
// The central content takes the base style again: egui's own `Ui::reset_style`
// (`egui/src/ui.rs:392`) restores the Context's style, application `Name` keys included.
ui.reset_style();
CentralPanel::default()
    .frame(ui.native_frame(Surface::CentralPanel))
    .show(ui, |ui| { /* content */ });
```

`Panel::top` and `Panel::bottom` are not resizable by default
(`containers/panel.rs:264-266`, `:273-275`), `Panel::left` and `Panel::right`
are (`:281`, `:296`), and `CentralPanel` derives `Default` (`:1186`).

### 4.5 The `Ui` extension trait

Both extension traits — this one and [`SystemThemeExt`] in §4.6 — are
**sealed**, so only this crate can implement them and adding a method to either
stays additive (§12.2 clause 6):

```rust,ignore
mod sealed {
    pub trait Sealed {}
    impl Sealed for egui::Ui {}
    impl Sealed for native_theme::SystemTheme {}
}
```

```rust,ignore
/// Role scoping and frame recipes on an [`egui::Ui`], from the atlas installed in its
/// `Context` ([`ThemeAtlas::from_ctx`](crate::ThemeAtlas::from_ctx)), in the scheme `self.ctx().theme()` reports
/// (`egui/src/context.rs:2158`). Every method degrades to egui's own behaviour when no atlas
/// is installed; none panics and none returns an `Option`.
///
/// **Invariant, and it is not enforced by the seal:** a method name added here must exist
/// on neither `egui::Ui` nor `egui::Context`. Rust tries the receiver types in order — `Ui`,
/// `&Ui`, `&mut Ui`, then through `Deref` (`egui/src/ui.rs:92-99`) `Context`, `&Context` — and
/// at each step an inherent method beats a trait method, but the first step that has a match
/// wins. So a trait method taking `&mut self` silently rebinds every existing `ui.<name>()`
/// call of a `Context` method of that name, and a trait method taking `&self` every call of
/// an inherent `Ui` method of that name taking `&mut self`; where the inherent `Ui` method is
/// reached first, it shadows the new trait method instead (all three verified with a probe on
/// rustc 1.98.1). §12.2 clause 6 carries the same rule.
pub trait NativeThemeUiExt: sealed::Sealed {
    /// Run `add_contents` in a child `Ui` styled for `role` in `variant`.
    ///
    /// Sugar over `ui.scope_builder(egui::UiBuilder::new().style(..), ..)`
    /// (`egui/src/ui.rs:2194`); a plain `ui.scope(..)` (`egui/src/ui.rs:2186`) when no atlas
    /// is installed. Every `TextStyle::Name` key of `self.style()` that the role style lacks
    /// is carried into the child, so an application's named text styles resolve inside the
    /// scope as outside it (`TextStyle::resolve` would panic on a missing one,
    /// `egui/src/style.rs:112-120`); the role's `Arc` is handed over as is when there is no
    /// such key; a copy is made only when there is one.
    ///
    /// It also records, for this pass, the child `Ui`'s `unique_id` (`egui/src/ui.rs:357-359`)
    /// with the role style's `widgets.active.corner_radius`, so the focus ring around a widget
    /// inside it takes the role's corners: the plugin uses the innermost recorded `Ui` whose
    /// final rect contains the focused widget's `interact_rect` on its layer (§6.18).
    ///
    /// **Does not reach `Area`-based containers** (`Window`, `Popup`, `Tooltip`, `Modal`,
    /// menus, `ComboBox` popups) when wrapped around them: their content `Ui` is built from
    /// the `Context`'s style (§1.5). Popups and menus take [`ThemeAtlas::role_modifier`](crate::ThemeAtlas::role_modifier);
    /// `Window` and `Modal` take [`NativeThemeUiExt::native_frame`] for their chrome and this
    /// scope, or [`NativeThemeUiExt::native_set_style`], as the first statement *inside* their
    /// closure for their body (§1.5, §3.3).
    fn native_scope<R>(
        &mut self,
        role: Role,
        variant: RoleVariant,
        add_contents: impl FnOnce(&mut egui::Ui) -> R,
    ) -> egui::InnerResponse<R>;

    /// Replace this `Ui`'s own style with `role`'s in `variant` for the remainder of the `Ui`
    /// (`Ui::set_style`, `egui/src/ui.rs:387`), carrying its `TextStyle::Name` keys as
    /// [`NativeThemeUiExt::native_scope`] does; nothing changes when no atlas is installed.
    /// Records this `Ui` and the role's corner radius for the focus ring as `native_scope`
    /// does, a later call on the same `Ui` replacing the record (§6.18). On the root `Ui`
    /// that `Context::run_ui` hands over, the ring reads the root's own style at the end of
    /// the pass instead, so a `Ui::reset_style` there (§4.4) is seen. The record covers the
    /// whole `Ui` while the restyle applies from this call on, and on any other `Ui` a style
    /// the application later sets by other means (`Ui::set_style`, `Ui::reset_style`) is
    /// not seen by the ring — §6.18's stated residuals.
    ///
    /// The spelling for a role that must be live on a `Ui` the application does not create:
    /// the body of a `Window` or `Modal` (§1.5), and the parent of a `Panel`, whose separator
    /// line is painted from the parent's style (§4.4).
    fn native_set_style(&mut self, role: Role, variant: RoleVariant);

    /// `surface`'s [`egui::Frame`] ([`ThemeAtlas::surface_frame`](crate::ThemeAtlas::surface_frame)). When nothing is installed
    /// it is the frame egui itself builds for that container from this `Ui`'s style, the
    /// same as passing no frame: `Frame::window` for [`Surface::Window`](crate::Surface::Window) and
    /// [`Surface::WindowTitleBar`](crate::Surface::WindowTitleBar) (`egui/src/containers/window.rs:630-631`),
    /// `Frame::popup` for [`Surface::Dialog`](crate::Surface::Dialog), [`Surface::Popover`](crate::Surface::Popover) and [`Surface::Tooltip`](crate::Surface::Tooltip)
    /// (`egui/src/containers/modal.rs:100`, `egui/src/containers/popup.rs:603`),
    /// `Frame::group` for [`Surface::Card`](crate::Surface::Card) (`egui/src/ui.rs:2147-2148`),
    /// `Frame::side_top_panel` for [`Surface::Panel`](crate::Surface::Panel) (`egui/src/containers/panel.rs:950`)
    /// and `Frame::central_panel` for [`Surface::CentralPanel`](crate::Surface::CentralPanel) (`:1243`).
    #[must_use = "this returns the frame recipe; hand it to a container's `.frame(..)`"]
    fn native_frame(&self, surface: Surface) -> egui::Frame;
}

impl NativeThemeUiExt for egui::Ui { /* … */ }
```

### 4.6 Parity constructors and the `SystemTheme` extension

```rust,ignore
/// Compile an atlas from one resolved variant. Pure mapping; borrows; never touches the OS.
///
/// egui keeps a separate `Style` per `egui::Theme`; with a single variant this installs the
/// same values into both, so an OS light/dark flip changes nothing.
///
/// Built with `AccessibilityPreferences::default()`. Two variants, the user's preferences
/// or any other input go through [`ThemeAtlas::builder`](crate::ThemeAtlas::builder) (`to_theme(r, name)` is
/// `ThemeAtlas::builder(name, r, r).build()`).
#[must_use = "this builds the styles; it does not install them"]
pub fn to_theme(resolved: &ResolvedTheme, name: &str) -> ThemeAtlas;

/// Compile an atlas from a bundled preset, including its [`LayoutTheme`](crate::LayoutTheme)
/// (`native_theme::theme::Theme::layout`, `native-theme/src/model/mod.rs:269`) and, with
/// feature `system-fonts`, the preset's typefaces through
/// `fonts::FontPlan::from_system(&light)` (`light` the resolved `ColorMode::Light`
/// variant), whose notes join [`ThemeAtlas::notes`](crate::ThemeAtlas::notes), as in [`from_system`]. Its
/// [`ThemeAtlas::name`](crate::ThemeAtlas::name) is the preset's `Theme::name` (`native-theme/src/model/mod.rs:257`), as
/// in both siblings (`connectors/native-theme-iced/src/lib.rs:260`,
/// `connectors/native-theme-gpui/src/lib.rs:263`).
/// An application with fonts of its own builds through [`ThemeAtlas::builder`](crate::ThemeAtlas::builder) with
/// `.fonts(FontPlan::from_system(&light).with_base(its_defs))`; this constructor uses
/// egui's default base.
///
/// The atlas carries **both** of the preset's variants — `ColorMode::Light` into the light
/// `Style`, `ColorMode::Dark` into the dark one, each through `Theme::resolve`
/// (`native-theme/src/model/mod.rs:450`), whose `pick_variant` cross-fallback serves a
/// one-variant preset (`:362-368`) and whose resolution is `ThemeMode::resolve_system`'s
/// (`:472`; `native-theme/src/resolve/mod.rs:267-268`) — for
/// the reason [`SystemThemeExt::to_egui_atlas`] gives: under `ThemePreference::System` egui
/// selects between its two styles every pass, so a preset compiled into one scheme would
/// leave the other scheme wrong. This is the one deviation from
/// `native_theme_gpui::from_preset`, which compiles the `is_dark` variant only.
///
/// The icon set is those two `Resolved`s' `icon_set`, a theme-level value they share
/// (`native-theme/src/model/mod.rs:468-470`); the icon theme
/// is each variant's own `Resolved::icon_theme`, `Theme::resolve(ColorMode::Light)`'s for
/// the light scheme and `ColorMode::Dark`'s for the dark one: [`ThemeAtlas::icon_set`](crate::ThemeAtlas::icon_set),
/// [`ThemeAtlas::icon_theme`](crate::ThemeAtlas::icon_theme).
///
/// The atlas carries no OS colour mode ([`ThemeAtlas::os_mode`](crate::ThemeAtlas::os_mode) is `None`), so on Linux,
/// where the integration reports none either, egui falls back to `Options::fallback_theme`.
/// To follow the OS there, build through [`ThemeAtlas::builder`](crate::ThemeAtlas::builder) with [`Builder::os_mode`](crate::Builder::os_mode).
///
/// `is_dark` selects the returned [`ResolvedTheme`](crate::ResolvedTheme); it does not pin egui's colour scheme —
/// that is egui's `ctx.set_theme(..)` ([`ThemeAtlas::install`](crate::ThemeAtlas::install)). It is explicit and never
/// inferred: some presets (`solarized`, `gruvbox`) have ambiguous lightness.
///
/// `prefs` is applied as [`Builder::accessibility`](crate::Builder::accessibility) applies it. Pass
/// `&AccessibilityPreferences::default()` for none, or
/// `&AccessibilityPreferences::from_system()` to honour the OS preferences under a preset —
/// accessibility is orthogonal to the theme choice, which is why
/// `native_theme_gpui::from_preset` takes the same argument
/// (`connectors/native-theme-gpui/src/lib.rs:257-261`).
///
/// # Errors
/// Propagates `Theme::preset` and `Theme::resolve` failures.
#[must_use = "this builds the styles; it does not install them"]
pub fn from_preset(
    name: &str,
    is_dark: bool,
    prefs: &AccessibilityPreferences,
) -> Result<(ThemeAtlas, ResolvedTheme)>;

/// Detect and compile the OS theme, carrying **both** variants, the OS colour mode, the OS
/// accessibility preferences, the OS [`LayoutTheme`](crate::LayoutTheme) and — with feature `system-fonts` — the
/// OS typefaces through `fonts::FontPlan::from_system`, whose notes join
/// [`ThemeAtlas::notes`](crate::ThemeAtlas::notes). Exactly [`SystemThemeExt::to_egui_atlas`] of
/// `SystemTheme::from_system()`. The `bool` is `sys.mode.is_dark()` —
/// the OS preference, not a luminance guess — and the [`ResolvedTheme`](crate::ResolvedTheme) is the variant it
/// selects, as in both siblings' `from_system`.
/// An application with fonts of its own builds through [`ThemeAtlas::builder`](crate::ThemeAtlas::builder) with
/// `.fonts(FontPlan::from_system(&light).with_base(its_defs))`; this constructor uses
/// egui's default base.
///
/// The layout is `SystemTheme::layout` (`native-theme/src/lib.rs:488`), the platform reader's
/// values merged field-wise over the preset's, fed to [`Builder::layout`](crate::Builder::layout); so this path
/// reaches the same spacing fields [`from_preset`] does. The accessibility preferences travel
/// inside the atlas ([`ThemeAtlas::accessibility`](crate::ThemeAtlas::accessibility)), which is why the tuple has no fourth
/// element as `native_theme_iced::from_system`'s does
/// (`connectors/native-theme-iced/src/lib.rs:283-288`).
///
/// # Errors
/// Propagates `SystemTheme::from_system`.
#[must_use = "this builds the styles; it does not install them"]
pub fn from_system() -> Result<(ThemeAtlas, ResolvedTheme, bool)>;

/// Compile a detected [`SystemTheme`](native_theme::SystemTheme). Sealed, like the extension trait of §4.5.
pub trait SystemThemeExt: sealed::Sealed {
    /// Compile an atlas carrying both OS variants, the OS colour mode, the OS accessibility
    /// preferences and the OS layout (`SystemTheme::layout`, `native-theme/src/lib.rs:488`).
    ///
    /// Deviation from `SystemThemeExt::to_iced_theme`
    /// (`connectors/native-theme-iced/src/lib.rs:305`) and `to_gpui_theme`
    /// (`connectors/native-theme-gpui/src/lib.rs:328`), which return one toolkit theme: egui
    /// stores one `Style` per `egui::Theme` and, under `ThemePreference::System`, picks one
    /// every pass from `RawInput::system_theme` (`egui/src/memory/mod.rs:358-359`), so
    /// returning one variant would guarantee a half-themed application. That input is the
    /// live OS scheme on macOS and Windows, where winit 0.30.13 reports one and sends
    /// `ThemeChanged` (egui-winit 0.36.2 `State::on_window_event`, lines 450–451 of its
    /// `src/lib.rs`); on Linux winit reports `None`, and the
    /// scheme egui sees is this atlas's OS mode, supplied by the install plugin (§3.2)
    /// and renewed by the next atlas installed (a `ThemeWatcher` rebuild).
    ///
    /// Equivalent to — and the chain to spell out when adding [`Builder::style_patch`](crate::Builder::style_patch) or a
    /// plan of the application's own —
    /// `ThemeAtlas::builder(&self.name, &self.light, &self.dark).layout(&self.layout)`
    /// `.accessibility(&self.accessibility).os_mode(self.mode).icon_set(self.icon_set)`
    /// `.icon_theme(egui::Theme::Light, light).icon_theme(egui::Theme::Dark, dark)`
    /// `.fonts(plan).build()` (each `icon_theme` only when
    /// `self.icon_theme_for(ColorMode::Light)` or `(ColorMode::Dark)` is `Some`, §9.2), `plan`
    /// being `fonts::FontPlan::from_system(&self.light)` with feature `system-fonts`, and no
    /// `.fonts(..)` call without. An application with fonts
    /// of its own builds through [`ThemeAtlas::builder`](crate::ThemeAtlas::builder) with
    /// `.fonts(FontPlan::from_system(&light).with_base(its_defs))`; this constructor uses
    /// egui's default base.
    #[must_use = "this builds the styles; it does not install them"]
    fn to_egui_atlas(&self) -> ThemeAtlas;
}

impl SystemThemeExt for native_theme::SystemTheme { /* … */ }
```

### 4.7 Free accessors — the values no `Style` field carries

An accessor exists only where it adds meaning to a `ResolvedTheme` leaf that no
`egui::Style` field carries *for that widget* — a conversion to an egui type, the
user's text-scaling factor, a choice between leaves, or the whole-crate font
rule of §8 — or where a sibling connector has the same function (the parity
table below). A leaf that needs none of these has no accessor: the application
reads it from the `ResolvedTheme` it already holds
([`ThemeAtlas::resolved_for`]), and its §5 row names the leaf path.

House shape, matching `connectors/native-theme-iced/src/lib.rs:369-581` and
`connectors/native-theme-gpui/src/lib.rs:369-539`: free functions, `#[must_use]`,
one expression each, no invented defaults, and no arithmetic except the user's
text-scaling factor and a formula a doc comment below states. An accessor returns the
leaf's meaning as the theme states it and clamps nothing: §7's `clamp_length` rule
applies to values this crate writes into a `Style` or a `Frame`, not to values it
hands the application. The exceptions are a returned `egui::FontId`, which
epaint lays out as it is, and `font_size` / `mono_font_size`, which return the
size the atlas installs: each size passes §8.5's rule, a scaled size that is not
a positive normal `f32` giving egui's own size for the `TextStyle` its doc names. The factor is applied by `scaled_text_size` here as in both
siblings (`connectors/native-theme-iced/src/lib.rs:480`,
`connectors/native-theme-gpui/src/lib.rs:444`). Iced's per-side padding helpers just
before that range, `padding_or` and `stated_padding`
(`connectors/native-theme-iced/src/lib.rs:329-339`, `:350-357`), have their
counterpart in [`convert::to_margin`] (§4.8). Adding a free
function is never a breaking change, which is not true of a struct field, an
enum variant or a trait method — which is why the entire non-`Style` surface is
free functions rather than a `Metrics` struct.

Both ranges are narrowed on purpose, and the two exclusions are the point. They
stop short of `to_iced_weight` (`connectors/native-theme-iced/src/lib.rs:603-616`),
a CSS-weight-to-enum table this crate has no use for because egui takes the
weight as a `wght` variation coordinate (§4.9); and they start after
`is_dark_resolved` and its alias `is_dark`
(`connectors/native-theme-gpui/src/lib.rs:353-355`, `:363-365`), whose
`< 0.5` luminance threshold is exactly the guess §4.6 rejects in favour of
`sys.mode.is_dark()`.

```rust,ignore
// --- colours egui has no slot for --------------------------------------------
// `Visuals` has `warn_fg_color` (egui/src/style.rs:1056) and `error_fg_color` (:1059) and
// nothing else in that family. Each is `to_color32` of its `defaults.*` leaf.
/// `defaults.info_color` as an `egui::Color32`.
#[must_use] pub fn info_color(t: &ResolvedTheme) -> egui::Color32;
/// `defaults.info_text_color` as an `egui::Color32`.
#[must_use] pub fn info_text_color(t: &ResolvedTheme) -> egui::Color32;
/// `defaults.warning_text_color` as an `egui::Color32`.
#[must_use] pub fn warning_text_color(t: &ResolvedTheme) -> egui::Color32;
/// `defaults.disabled_text_color` as an `egui::Color32`.
#[must_use] pub fn disabled_text_color(t: &ResolvedTheme) -> egui::Color32;
/// `defaults.selection_inactive_background` as an `egui::Color32`.
#[must_use] pub fn selection_inactive_background(t: &ResolvedTheme) -> egui::Color32;

// --- focus ring: no egui concept at all ---------------------------------------
// egui promotes a keyboard-focused widget to `widgets.active`
// (`egui/src/widget_style.rs:107-109`; `Widgets::style`, `egui/src/style.rs:1273-1282`) and
// paints no ring. The install plugin paints it from these three leaves (§6.18); writing it
// into `widgets.active.bg_stroke` instead would show it on every mouse press too. These
// accessors are for a ring the application draws on a surface of its own.
/// `defaults.focus_ring_color` as an `egui::Color32`.
#[must_use] pub fn focus_ring_color(t: &ResolvedTheme) -> egui::Color32;
/// `defaults.focus_ring_width`, the focus ring's outline width in logical pixels.
#[must_use] pub fn focus_ring_width(t: &ResolvedTheme) -> f32;
/// **May legitimately be negative** (an inset ring: adwaita −2.0, macOS −1.0). Never clamp it
/// to zero (`native-theme/src/resolve/validate_helpers.rs:708-709`).
#[must_use] pub fn focus_ring_offset(t: &ResolvedTheme) -> f32;

// --- typography roles egui's five TextStyles cannot hold -----------------------
/// One of native-theme's four text-scale roles (`native-theme/src/model/resolved.rs:54-63`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TextRole {
    /// Caption / small label text: `text_scale.caption`.
    Caption,
    /// Section heading text: `text_scale.section_heading`.
    SectionHeading,
    /// Dialog title text: `text_scale.dialog_title`.
    DialogTitle,
    /// Large display / hero text: `text_scale.display`.
    Display,
}

/// The `egui::FontId` for a text-scale role, its size [`scaled_text_size`] of the role's size —
/// the size the atlas installs — or, where that is not a positive normal `f32`, egui's own size
/// for `TextStyle::Small` (`Caption`) or `TextStyle::Heading` (the other three, the style
/// `Ui::heading` sets, `egui/src/widget_text.rs:233-235`) (§8.5). `Caption` and
/// `SectionHeading` are also installed
/// into `TextStyle::Small` and `TextStyle::Heading`; `DialogTitle` and `Display` have no
/// **stock** `TextStyle` slot and are reachable only here, through `RichText::font(..)`
/// (`egui/src/widget_text.rs:190-198`).
///
/// A slot could have been minted — `TextStyle::Name(Arc<str>)` exists (`egui/src/style.rs:94`,
/// stored in `Style::text_styles`, `:289`) — so this is a **declined** candidate, not an egui
/// limitation. It is declined because `TextStyle::resolve` panics on a key absent from
/// `text_styles` (`:112-120`) with no `cfg(debug_assertions)` guard, which would make a `Name`
/// key a live panic in release for any `Style` this crate did not build. See §5.8.
#[must_use] pub fn text_role_font(
    t: &ResolvedTheme,
    role: TextRole,
    prefs: &AccessibilityPreferences,
) -> egui::FontId;

/// The absolute line height in logical pixels for a text-scale role
/// (`ResolvedTextScaleEntry::line_height`, `native-theme/src/model/resolved.rs:47`), scaled by
/// the same factor as [`text_role_font`]'s size, so the line box keeps its ratio to the text.
/// Feed to `RichText::line_height(Some(..))` (`egui/src/widget_text.rs:174`) — the only exact
/// per-role mechanism egui has.
#[must_use] pub fn text_role_line_height(
    t: &ResolvedTheme,
    role: TextRole,
    prefs: &AccessibilityPreferences,
) -> f32;

/// `defaults.line_height`, the dimensionless **multiplier**
/// (`native-theme/src/model/resolved.rs:76-77`). Not the same unit as
/// [`text_role_line_height`].
#[must_use] pub fn line_height_multiplier(t: &ResolvedTheme) -> f32;
/// `defaults.font.weight`, a CSS weight.
#[must_use] pub fn font_weight(t: &ResolvedTheme) -> u16;
/// `defaults.mono_font.weight`, a CSS weight.
#[must_use] pub fn mono_font_weight(t: &ResolvedTheme) -> u16;
/// The CSS weight the theme asks for in a given text-scale role — what to pass to
/// `RichText::variation(egui::epaint::text::Tag::new(b"wght"), w as f32)` at a call site.
#[must_use] pub fn text_role_weight(t: &ResolvedTheme, role: TextRole) -> u16;

/// The CSS weight of the font text in `role` is set in — for `RichText::variation(
/// egui::epaint::text::Tag::new(b"wght"), w as f32)` (`egui/src/widget_text.rs:202`) at a
/// call site, because a `FontId` carries no weight (`epaint/src/text/fonts.rs:21-28`) and this
/// crate installs one face per family (§4.9). The font is the widget's own: `font` for the
/// fourteen roles whose widget states one (`Button`, `Input`, `Checkbox`, `Menu`, `Tooltip`,
/// `Tab`, `Sidebar`, `Toolbar`, `StatusBar`, `Popover`, `ComboBox`, `SegmentedControl`,
/// `Expander`, `Link`), `list.item_font` for `List` (its cells), `dialog.body_font` for
/// `Dialog` (its body), `window.title_bar_font` for `Window` (the one font that widget
/// states), and `defaults.font` for the eight roles whose widget states none (`Scrollbar`,
/// `Slider`, `ProgressBar`, `Splitter`, `Separator`, `Switch`, `Spinner`, `Card`), whose
/// text is the defaults' text (`native-theme/src/model/widgets/mod.rs`, the
/// `#[theme_inherit(font = ..)]` attribute of each widget). A font spec that is no role's
/// font — `list.header_font`, `dialog.title_font` — is read from the `ResolvedTheme`
/// (`t.list.header_font.weight`, …) into the same call.
#[must_use] pub fn role_font_weight(t: &ResolvedTheme, role: Role) -> u16;
/// Whether text in `role` needs egui's synthetic slant, `RichText::italics`
/// (`egui/src/widget_text.rs:284`), to show the slant the theme asks for: `true` when the
/// role's font (as [`role_font_weight`] chooses it) has a `style` other than
/// `FontStyle::Normal` (`native-theme/src/model/font.rs:59`) and `defaults.font` — the spec
/// whose face the atlas installs as `FontFamily::Proportional` (§4.9, §8.2) — has
/// `FontStyle::Normal`. When `defaults.font` is itself slanted, the installed face already
/// is, and a second, synthetic slant would double it, so this returns `false`. egui's skew
/// cannot tell `Italic` from `Oblique`, so both map to the same call (§8.4).
#[must_use] pub fn role_font_is_italic(t: &ResolvedTheme, role: Role) -> bool;

/// `window.title_bar_font` as an `egui::FontId`: `FontFamily::Proportional` (the never-`Name`
/// invariant, §4.9) at [`scaled_text_size`] of its size, or egui's own `TextStyle::Heading`
/// size where that is not a positive normal `f32` — the style a `Window` title falls back to
/// (`egui/src/containers/window.rs:1350`, §8.5). For the title atoms of a `Window`,
/// via `RichText::font` (`egui/src/widget_text.rs:190-198`): the title is laid out outside the
/// application's closure (§5.2), so no scope reaches it.
#[must_use] pub fn window_title_bar_font(
    t: &ResolvedTheme,
    prefs: &AccessibilityPreferences,
) -> egui::FontId;
/// The title text colour for an active (`true`) or inactive window: `window.title_bar_font.color`
/// or `window.inactive_title_bar_text_color` (`native-theme/src/model/widgets/mod.rs:25`).
/// Feed to `RichText::color` (`egui/src/widget_text.rs:320`). egui does not tell the title
/// which of the two a window is in (§5.2), so the application passes what it knows.
#[must_use] pub fn window_title_bar_text_color(t: &ResolvedTheme, active: bool) -> egui::Color32;

// --- geometry with no Style sink ----------------------------------------------
/// `input.border.padding` as the margin to hand `TextEdit::margin`
/// (`egui/src/widgets/text_edit/builder.rs:313`), the one place a `TextEdit` takes its inner
/// padding — no `Style` field holds it. Each side the theme states is that side plus the
/// border's line width, because `TextEdit` paints its stroke inside this margin (`:763-767`)
/// where the platform's padding lies inside the border (§7.2); a side the theme leaves
/// `None` keeps egui's own default, `Margin::symmetric(4, 2)`
/// (`egui/src/widgets/text_edit/builder.rs:136`, a crate constant naming that line). Built
/// with [`convert::to_margin`](crate::convert::to_margin), so a side saturates at the `i8` bound of `epaint::Margin`.
#[must_use] pub fn input_margin(t: &ResolvedTheme) -> egui::Margin;
/// The frame a `TextEdit` paints, for `TextEdit::frame`
/// (`egui/src/widgets/text_edit/builder.rs:304-309`), which paints a frame it is handed as
/// given (`:712-713`, `:734-735`): egui's own frame for a mutable text (`:737-750`,
/// `:759-768`), except that a focused field's stroke takes its colour from
/// `input.focus_border_color` where egui's takes `visuals.selection.stroke` (`:742-747`) —
/// the selected text's colour, which the `Role::Input` cell holds (§5.4) — at the width of
/// the state's own `bg_stroke`. Called inside the `Role::Input` scope with the id handed to
/// `TextEdit::id` (`:168`):
/// `TextEdit::singleline(&mut s).id(id).frame(input_frame(ui, id, &t))`. The frame's inner
/// margin is [`input_margin`]; beside a frame, `TextEdit::margin` is unused (`:712-713`).
///
/// Computed from `ui`'s style exactly as egui computes it. The state is
/// `ui.style().interact(&r)` (`egui/src/style.rs:355-357`) of
/// `r = ui.ctx().read_response(id)` (`egui/src/context.rs:1350-1355`) — this pass's
/// interaction, before the field is added — or, where there is none yet, `widgets.active`
/// when focused (below) and `widgets.inactive` otherwise — the entries `Widgets::style` gives
/// a focused response and one neither hovered, pressed nor focused
/// (`egui/src/style.rs:1273-1282`); focused is `Response::has_focus`'s own body
/// (`egui/src/response.rs:349-351`), `ui.ctx().input(|i| i.focused)` and
/// `ui.ctx().memory(|m| m.has_focus(id))`, whether or not `read_response` returned a response
/// (a field focused before its first pass). Focus moved while the field is added — by Tab
/// (`egui/src/context.rs:1269-1270`, `egui/src/memory/mod.rs:659-668`) or by
/// `Memory::request_focus` after this call — shows in the next pass: a key press or a click is
/// an input event, after which egui runs that pass itself (`egui/src/input_state/mod.rs:655-663`,
/// `egui/src/context.rs:469`, `:537-538`); a focus moved with no input event shows in the next
/// pass that runs, which the application can request at once with `Context::request_repaint`
/// (`egui/src/context.rs:1821`).
/// Fill `ui.visuals().text_edit_bg_color()` (`egui/src/widgets/text_edit/builder.rs:739`);
/// corner radius the state's (`:744`, `:749`); stroke the state's `bg_stroke` (`:749`), or,
/// focused, `input.focus_border_color` at the state's own `bg_stroke.width` (not egui's
/// `selection.stroke.width`: a width that changed with the state would change the frame's
/// total margin, and the field's size, on focus) — or the
/// resting `widgets.inactive.bg_stroke` where that `soft_option` is `None`, the border not
/// changing on focus (§5.4); inner margin [`input_margin`] plus `expansion − stroke.width`
/// on every side, outer margin `−expansion` (`:763-767`). Both narrowings to `i8` go through
/// `i8_from_f32_saturating` (§7.2) where egui casts with `as`
/// (`egui/src/widgets/text_edit/builder.rs:765`, `:767`), and the sum is `epaint::Margin`'s
/// saturating `Add` (`epaint/src/margin.rs:123-134`), so the function is total; the two agree
/// bit for bit wherever `expansion` is whole, as it is in every scope but `Role::Slider`'s
/// (§6.6).
///
/// `TextEdit` lays a custom frame out by the frame's own total margin
/// (`egui/src/widgets/text_edit/builder.rs:715`, `:726-729`; `Frame::total_margin`,
/// `egui/src/containers/frame.rs:327-331`), where it lays its stock frame out by the margin
/// alone (`egui/src/widgets/text_edit/builder.rs:713`); the two agree whenever
/// `expansion − stroke.width` is whole. With a half-point stroke (`macos-sonoma`, `ios`,
/// §5.4) the total margin is half a point less than egui's stock one in every state, so the
/// field is one point shorter than egui's own and keeps that size whether focused or not.
#[must_use = "hand the frame to TextEdit::frame"] pub fn input_frame(ui: &egui::Ui, id: egui::Id, t: &ResolvedTheme) -> egui::Frame;
/// The expander's arrow, in `expander.arrow_color`, for `CollapsingHeader::icon`
/// (`egui/src/containers/collapsing_header.rs:480`). egui's own arrow,
/// `egui::collapsing_header::paint_default_icon` (`:336`), fills with the `fg_stroke.color`
/// of the state it is in (`:337`, `:351-355`), which is the header text's colour too. The
/// closure sets that colour to `expander.arrow_color` in all five `widgets` states of a copy
/// of the `Ui`'s style, calls `paint_default_icon` — egui's shape, egui's rotation — and
/// restores the `Ui`'s previous `Arc<Style>` with `Ui::set_style` (`egui/src/ui.rs:387`),
/// because it is handed the header's parent `Ui` (`:592`), whose later widgets must not see
/// the change. Owns one `Color32`, so it is `'static` as `icon` requires.
#[must_use = "hand the closure to CollapsingHeader::icon"] pub fn expander_icon(
    t: &ResolvedTheme,
) -> impl Fn(&mut egui::Ui, f32, &egui::Response) + 'static;
/// `list.header_font` (`native-theme/src/model/widgets/mod.rs:535`) as an `egui::FontId`:
/// `FontFamily::Proportional` (the never-`Name` invariant, §4.9) at [`scaled_text_size`] of
/// its size, or egui's own `TextStyle::Body` size where that is not a positive normal `f32`
/// (§8.5). Feed to `RichText::font(..)`
/// (`egui/src/widget_text.rs:190-198`); the header's colour and weight are
/// `t.list.header_font.color` and `.weight` (§5.4).
#[must_use] pub fn list_header_font(
    t: &ResolvedTheme,
    prefs: &AccessibilityPreferences,
) -> egui::FontId;
/// `dialog.button_order`: the order in which to add a dialog's buttons (§5.2).
#[must_use] pub fn dialog_button_order(t: &ResolvedTheme) -> DialogButtonOrder;
/// `scrollbar.groove_width`, in logical pixels.
#[must_use] pub fn scrollbar_width(t: &ResolvedTheme) -> f32;
/// `defaults.border.corner_radius`, in logical pixels.
#[must_use] pub fn border_radius(t: &ResolvedTheme) -> f32;
/// `defaults.border.corner_radius_lg`, in logical pixels.
#[must_use] pub fn border_radius_lg(t: &ResolvedTheme) -> f32;
/// `defaults.border.color` with `defaults.border.opacity` folded into its alpha —
/// [`convert::to_color32_with_opacity`](crate::convert::to_color32_with_opacity), the fold every border stroke the atlas writes goes through (§6.13).
#[must_use] pub fn border_color(t: &ResolvedTheme) -> egui::Color32;
/// `defaults.disabled_opacity`, the opacity for disabled controls.
#[must_use] pub fn disabled_opacity(t: &ResolvedTheme) -> f32;
/// `defaults.font.family`, the family name the theme states.
#[must_use] pub fn font_family(t: &ResolvedTheme) -> &str;
/// [`scaled_text_size`] of `defaults.font.size` — the `TextStyle::Body` size the atlas
/// installs, or egui's own `TextStyle::Body` size where that is not a positive normal `f32`
/// (§8.5). Same signature as `native_theme_iced::font_size`
/// (`connectors/native-theme-iced/src/lib.rs:438-443`).
#[must_use] pub fn font_size(t: &ResolvedTheme, prefs: &AccessibilityPreferences) -> f32;
/// `defaults.mono_font.family`, the family name the theme states.
#[must_use] pub fn mono_font_family(t: &ResolvedTheme) -> &str;
/// [`scaled_text_size`] of `defaults.mono_font.size` — the `TextStyle::Monospace` size the
/// atlas installs, or egui's own `TextStyle::Monospace` size where that is not a positive
/// normal `f32` (§8.5).
#[must_use] pub fn mono_font_size(t: &ResolvedTheme, prefs: &AccessibilityPreferences) -> f32;

// --- accessibility: `&SystemTheme`, never `&ResolvedTheme` ---------------------
// `AccessibilityPreferences` lives on `SystemTheme` (`native-theme/src/lib.rs:490`) and is
// deliberately absent from `ResolutionContext` (`native-theme/src/resolve/context.rs:20-23`).
// The bare `&SystemTheme` in these four signatures is resolved by a crate-private
// `use native_theme::SystemTheme;`, NOT by a root re-export — §4.1 removes that one on purpose.
/// The atlas already forwards it to egui's own animations ([`Builder::accessibility`](crate::Builder::accessibility)); this
/// is for the application's, e.g. the `reduced_motion` argument of
/// [`icons::animated_frame_index`](crate::icons::animated_frame_index) and [`icons::spin_angle`](crate::icons::spin_angle).
#[must_use] pub fn is_reduced_motion(sys: &SystemTheme) -> bool;
/// `accessibility.high_contrast` of the `SystemTheme`: whether the OS asks for high contrast.
#[must_use] pub fn is_high_contrast(sys: &SystemTheme) -> bool;
/// `accessibility.reduce_transparency` of the `SystemTheme`: whether the OS asks for less transparency.
#[must_use] pub fn is_reduced_transparency(sys: &SystemTheme) -> bool;

/// `accessibility.text_scaling_factor` as the OS reported it (1.0 = no scaling), unfiltered;
/// [`scaled_text_size`] is what ignores a factor that is not finite and positive.
///
/// The atlas applies it to text sizes ([`Builder::accessibility`](crate::Builder::accessibility)). egui has no equivalent of
/// its own: no `egui` or `eframe` 0.36.2 source reads an OS text-scaling preference, and
/// `Context::set_zoom_factor` (`egui/src/context.rs:2337`) is not one — it sets
/// `pixels_per_point = zoom_factor * native_pixels_per_point` (`:455`) and so scales every
/// length, strokes and icons included, where the preference asks for larger text
/// (`native-theme/src/lib.rs:251-253`). Do **not** pre-multiply any `Style` value by
/// `pixels_per_point` or `zoom_factor` (§2, *Lengths*).
#[must_use] pub fn text_scaling_factor(sys: &SystemTheme) -> f32;

/// A text size from the theme times the user's text-scaling factor; a factor that is not
/// finite and positive is ignored. Same signature and semantics as
/// `native_theme_iced::scaled_text_size` (`connectors/native-theme-iced/src/lib.rs:480`) and
/// `native_theme_gpui::scaled_text_size` (`connectors/native-theme-gpui/src/lib.rs:444`).
///
/// For a size no accessor above returns — a widget font read from the `ResolvedTheme`
/// directly (`t.button.font.size`, …) and drawn at a call site. Takes the preferences, not a
/// `&SystemTheme`, so the preset path can pass [`ThemeAtlas::accessibility`](crate::ThemeAtlas::accessibility).
#[must_use] pub fn scaled_text_size(size: f32, prefs: &AccessibilityPreferences) -> f32;
```

**Sibling parity.** Every public item of the two sibling connectors at
`2db686ee`, with its counterpart here or the reason there is none. Crate-root
items are listed one by one — `connectors/native-theme-iced/src/lib.rs:180-616`
and `connectors/native-theme-gpui/src/lib.rs:111-746` — and each public module
as one row. "Style" means the value is written into the atlas's styles or
frames (§5 gives the row), where egui reads it with no accessor; `ui.visuals()`
/ `ui.spacing()` read it back. "Leaf" means the application reads the
`ResolvedTheme` leaf named in the row.

| sibling item | crate | here | reason, where there is no one-to-one counterpart |
|---|---|---|---|
| `to_theme` | both | [`to_theme`] | |
| `from_preset` | both | [`from_preset`] | carries both variants (§4.6) |
| `from_system` | both | [`from_system`] | |
| `SystemThemeExt::to_iced_theme` / `to_gpui_theme` | iced / gpui | [`SystemThemeExt::to_egui_atlas`] | one atlas for both schemes (§4.6) |
| `padding_or`, `stated_padding` | iced | [`convert::to_margin`] | egui's padding sinks are per side or per axis, and an unstated side keeps egui's own; no egui call site takes an all-or-nothing padding, which is what `stated_padding` returns |
| `button_padding` | iced | Style | `button.border.padding` is `Spacing::button_padding` in the base style and the `Role::Button` style (§5.3); iced's button takes padding per call, egui's reads the style |
| `input_padding` | iced | [`input_margin`] | `TextEdit::margin` is per call |
| `scrollbar_width`, `scaled_text_size`, `font_weight`, `mono_font_weight`, `line_height_multiplier`, `disabled_opacity` | both | same names | |
| `border_radius`, `border_radius_lg`, `font_family`, `font_size`, `mono_font_family`, `mono_font_size`, `border_color`, `focus_ring_color`, `info_color` | iced | same names | |
| `link_color` | iced | Style | `defaults.link_color` is `Visuals::hyperlink_color` (§5.1) |
| `selection_color` | iced | Style | `defaults.selection_background` is `Visuals::selection.bg_fill` (§5.1) |
| `info_foreground_color`, `warning_foreground_color` | iced | [`info_text_color`], [`warning_text_color`] | named after their leaves, `defaults.info_text_color` / `warning_text_color` |
| `icon_sizes` | both | [`icons::icon_size`] | one size per [`icons::IconContext`] instead of the struct |
| `to_iced_weight` | iced | [`fonts::weight_coords`] | egui takes a weight as a `wght` coordinate, not an enum (§4.9) |
| `is_dark_resolved`, `is_dark` | gpui | — | a `< 0.5` luminance guess; §4.6 uses `sys.mode.is_dark()` |
| `is_reduced_motion`, `is_high_contrast`, `is_reduced_transparency`, `text_scaling_factor` | gpui | same names | |
| `frame_width` | gpui | Style | `defaults.border.line_width` is the base style's `widgets.noninteractive.bg_stroke.width` and `window_stroke.width`, and `button.border.line_width` its interactive states' (§5.1, §5.9) |
| `border_opacity` | gpui | Style | `defaults.border.opacity` is folded into every border stroke's colour (§5.1, §6.13) and into [`border_color`]; egui has no stroke opacity to hand it to |
| `shadow_enabled` | gpui | Style | `defaults.border.shadow_enabled` gates `window_shadow` / `popup_shadow` (§5.1, §6.14) |
| `text_scale` | gpui | [`text_role_font`], [`text_role_line_height`], [`text_role_weight`] | one accessor per property, keyed by [`TextRole`], each already text-scaled |
| `dialog_button_order` | gpui | same name | |
| `dialog_button_spacing` | gpui | Style | `dialog.button_gap` is `spacing.item_spacing.x` in the `Role::Dialog` style (§5.2) |
| `selection_foreground` | gpui | Style | `defaults.selection_text_color` is `Visuals::selection.stroke.color` (§5.1) |
| `selection_inactive` | gpui | [`selection_inactive_background`] | named after its leaf |
| `disabled_foreground` | gpui | [`disabled_text_color`] | named after its leaf |
| `focus_ring_width`, `focus_ring_offset` | gpui | same names | and painted by the install plugin (§6.18) |
| `NativeTheme` | gpui | [`ThemeAtlas`] | the installed state lives in `Context` data |
| `ActiveNativeTheme` | gpui | [`ThemeAtlas::from_ctx`] | `cx.native_theme()` is `ThemeAtlas::from_ctx(ctx)` |
| `Native<'a>` | gpui | — | it bundles the inputs of gpui's per-element `geometry` builders; egui's geometry is in the role styles and surface frames, so nothing takes such a bundle |
| `apply` | gpui | [`ThemeAtlas::install`] | |
| `apply_system_theme` | gpui | `sys.to_egui_atlas().install(ctx)` | two calls, no helper |
| `apply_accessibility` | gpui | rebuild with [`Builder::accessibility`], then install | the role styles are immutable `Arc`s (§3.4), so the preferences are a build input, not an in-place rescale |
| re-exports `Rgba`, `SystemTheme`, `Theme`, `IconData` at the root | both | [`convert::Rgba`], `native_theme::SystemTheme`, `native_theme::theme::Theme`, [`icons::IconData`] | each name collides with one at egui's root (§4.1) |
| mod `palette` (`to_color`, `to_palette`) | iced | [`convert::to_color32`]; — | egui has no palette type: the colours are compiled into the atlas's `Visuals` |
| mod `styles` (`button`, `button_primary`, `text_input`, `text_editor`, `checkbox`, `radio`, `toggler`, `pick_list`, `menu`, `slider`, `scrollable`, `scrollbar`, `progress_bar`, `rule`, `tooltip`, `container_card`) | iced | [`NativeThemeUiExt::native_scope`], [`NativeThemeUiExt::native_set_style`], [`ThemeAtlas::role_modifier`], [`ThemeAtlas::surface_frame`] | iced styles one widget per call; egui styles a `Ui` (§3). `button_primary` is a `Button::selected(true)` in `Role::Button`'s `Normal` cell (§6.2) |
| `styles::button_danger`, `button_success`, `button_warning` | iced | — (leaf) | no `RoleVariant` for them: native-theme states only the idle fill and label of these classes (`defaults.{danger,success,warning}_color` / `_text_color`), which a stock `Button` takes per call — `Button::fill` (`egui/src/widgets/button.rs:143`) and `RichText::color` — from those leaves, and the base style's `error_fg_color` / `warn_fg_color` (§5.1) |
| `styles::button_link` | iced | `Role::Link` | egui's `Hyperlink` / `Link` read `hyperlink_color`, carried by the role style |
| `styles::aw` | iced | — | it styles `iced_aw` widgets; egui has no such companion crate |
| mod `icons` (`to_image_handle`, `to_svg_handle`, `custom_icon_to_image_handle`, `custom_icon_to_svg_handle`, `into_image_handle`, `into_svg_handle`, `animated_frames_to_svg_handles`, `AnimatedSvgHandles`, `spin_rotation_radians`) | iced | [`icons::to_image_source`], [`icons::to_image`], [`icons::custom_icon_to_image_source`], [`icons::animated_frame_index`], [`icons::spin_angle`] | egui takes one `ImageSource` for both payload kinds, and caches by URI, so there is no by-value `into_*` pair and no pre-built frame vector: a frame is converted when drawn, under the URI its bytes hash to (§9.2); the `color` and `icon_set` arguments are the [`icons::IconKey`]'s tint and set, the tint coloured into the SVG bytes by `native_theme::icons::colorize_monochrome_svg` (§9.2) |
| mod `icons` (`icon_name`, `lucide_name_for_gpui_icon`, `material_name_for_gpui_icon`, `freedesktop_name_for_gpui_icon`) | gpui | — | they translate gpui-component's `IconName`; egui has no icon vocabulary of its own |
| mod `icons` (`to_image_source`, `into_image_source`, `custom_icon_to_image_source`, `bundled_icon_to_image_source`, `bundled_svg_to_image_source`, `animated_frames_to_image_sources`, `AnimatedImageSources`, `with_spin_animation`) | gpui | [`icons::to_image_source`], [`icons::custom_icon_to_image_source`], [`icons::animated_frame_index`], [`icons::spin_angle`] | a bundled icon is an `IconData` like any other, so it takes the same call; `AnimatedImageSources` is on §4.12's list, and a frame is converted when drawn, under the URI its bytes hash to (§9.2). The `color` and `icon_set` arguments are the [`icons::IconKey`]'s tint and set, the tint coloured into the SVG bytes by `native_theme::icons::colorize_monochrome_svg`, gpui's `colorize_svg` algorithm (§9.2); `size` is [`icons::IconKey::size`] |
| mod `geometry` | gpui | the role styles and surface frames, plus [`icons::icon_size`], [`window_title_bar_font`], [`window_title_bar_text_color`] and leaves (`input.min_height`, `dialog.min_width` …) | gpui refines each element per call; egui's geometry reaches widgets through the style. Its four `LayoutTheme` helpers (`widget_gap`, `container_margin`, `window_margin`, `section_gap`) are [`Builder::layout`]'s input: `widget_gap` becomes `spacing.item_spacing` and `window_margin` [`Surface::CentralPanel`]'s inner margin (§5.1); `container_margin` and `section_gap` are per-call values read from [`ThemeAtlas::layout`] (§4.2, §5.1) |
| mod `base_layer` | gpui | the base style, `Role::Splitter` | the scrollbar's widths and colours are the base style's here (§5.9), so every `ScrollArea` has them, and the resize handle's line is `Role::Splitter`'s |
| mod `variants` (`ghost_button`) | gpui | `Button::frame_when_inactive(false)` (`egui/src/widgets/button.rs:178`) in the `Role::Button` scope | egui's stock button already has the ghost shape: no frame at rest, the role's frame on hover |

### 4.8 `mod convert`

Total, panic-free conversions from native-theme values to epaint types: every
function is defined for all `f32` bit patterns and none can panic, allocate or
use `unsafe`. The public surface is the signatures below, with a one-line
contract each; §7.2 holds the single copy of every doc comment and reference
body — they are part of the contract, not an illustration — and the module's
one non-finite rule: a `NaN` or `±∞` theme value is replaced by egui's own value
for the sink it feeds, never by an invented one, and reported as
[`crate::Note::ValueSanitised`].

```rust,ignore
/// native-theme's sRGB, straight-alpha `u8` colour. Re-exported **here and not at the crate
/// root**: `egui::Rgba` (`egui/src/lib.rs:442`) is linear, premultiplied `f32` (§4.1).
pub use native_theme::color::Rgba;

/// `Rgba` -> `egui::Color32`, premultiplying in gamma space with no transfer function.
#[must_use] pub const fn to_color32(c: Rgba) -> egui::Color32;
/// [`to_color32`] with an opacity folded into the alpha — how `defaults.border.opacity`
/// reaches every stroke (§6.13). Never for a `WidgetVisuals::bg_fill`.
#[must_use] pub fn to_color32_with_opacity(c: Rgba, opacity: f32) -> egui::Color32;
/// `v` when finite, else `fallback` — egui's own value for the sink being written.
#[must_use] pub fn finite_or(v: f32, fallback: f32) -> f32;
/// One layout length written into a `Style` or a `Frame`, clamped into
/// `0.0..=LENGTH_CEILING` (§7.2).
#[must_use] pub fn clamp_length(v: f32) -> f32;
/// An opacity clamped into `0.0..=1.0`; mandatory before `Visuals::disabled_alpha`.
#[must_use] pub fn unit_interval(v: f32) -> f32;
/// Saturating, rounding `f32` -> `u8` for `epaint::CornerRadius`, agreeing bit for bit with
/// epaint's own `From<f32> for CornerRadius`.
#[must_use] pub fn u8_from_f32_saturating(v: f32) -> u8;
/// Uniform `CornerRadius` from one length; `base` for a non-finite one.
#[must_use] pub fn to_corner_radius(base: egui::CornerRadius, radius_px: f32) -> egui::CornerRadius;
/// `epaint::Margin` from a resolved padding, side for side; an unstated or non-finite side
/// keeps `base`'s, and a stated one saturates at the `i8` bound, which the call site reports
/// as [`crate::Note::ValueSaturated`].
#[must_use] pub fn to_margin(
    base: egui::Margin,
    padding: &native_theme::theme::ResolvedPadding,
) -> egui::Margin;
/// A stated hover or pressed layer composited over the widget's idle fill (C17, §6.1).
#[must_use] pub fn composite_over(layer: Rgba, idle: Rgba) -> egui::Color32;
/// `epaint::Stroke` from a colour and a line width; the width falls back to `base.width`
/// when non-finite and is never negative.
#[must_use] pub fn to_stroke(base: egui::Stroke, color: Rgba, line_width_px: f32) -> egui::Stroke;
```

Crate-private, specified in §7.2 beside the public ones: `denan` (the
saturating casts' internal `NaN` step), `i8_from_f32_saturating`,
`padding_with_border`, `to_button_padding` and `to_shadow`. They are sink
details of this crate's own writes, with no caller outside it; the one an
application needs, the input padding, is [`input_margin`].

### 4.9 `mod fonts`

Font registration. The module's rustdoc states the three rules §8 argues: a face
enters epaint only as bytes, never by an OS family name (§8.2); this crate never
emits a `FontFamily::Name`, so at most two faces are installed — the one
matching `defaults.font` as `FontFamily::Proportional` and the one matching
`defaults.mono_font` as `FontFamily::Monospace` — and one weight per family
(§8.1, §8.3); and every face is validated with epaint's own parse before epaint
sees it, so epaint's release-mode parse panic is unreachable (§8.2). This crate
reads no file itself: the bytes come from the application ([`FontPlan::face`])
or, with feature `system-fonts`, from native-theme (`FontPlan::from_system`).

```rust,ignore
/// A byte buffer for one font face.
#[derive(Clone)]
#[non_exhaustive]
pub enum FontBytes {
    /// `&'static [u8]`, typically `include_bytes!`. Registered with `FontData::from_static`
    /// (`epaint/src/text/fonts.rs:125`), avoiding the per-`Fonts::new` copy that `Cow::Owned`
    /// incurs (`fonts.rs:391-396`).
    Static(&'static [u8]),
    /// A shared owned buffer. epaint's only constructors are `FontData::from_static`
    /// (`epaint/src/text/fonts.rs:125`) and `FontData::from_owned` (`:133`), so these bytes
    /// are copied into a `Vec<u8>` once when [`font_definitions`] builds the `FontData`, and
    /// again at every `FontsImpl::new` (`:391-396`). The `Arc` serves a [`FontPlan`] reused
    /// across rebuilds and `native_theme::fonts::SystemFace`'s `Arc<[u8]>`; it does **not**
    /// avoid epaint's copies. Prefer [`FontBytes::Static`] for bytes the application embeds.
    Shared(std::sync::Arc<[u8]>),
}

/// Which faces the application has bytes for, and the choice among them.
///
/// The choice is made by `native_theme::fonts::select_face` (§8.2), the pure function
/// native-theme exports without a feature and `native_theme::fonts::system_face` also uses:
/// each registered face is described to it as a `native_theme::fonts::FaceTraits` (its
/// family, weight and style as registered, and width `5`, normal, since a registered face
/// states no width), and it applies CSS Fonts Level 4 §5.1 caseless
/// family matching, never substituting another family, then §5.2's width, style and weight
/// steps. So an application's faces and the system's are matched by one rule. A
/// request with no face of its family is a [`crate::Note::FontFamilyUnavailable`] in
/// [`crate::ThemeAtlas::notes`]; nothing is ever invented. Faces registered beyond the two
/// the theme selects are unused and cost nothing.
#[derive(Clone, Default)]
#[must_use]
pub struct FontPlan { /* private */ }

impl FontPlan {
    /// An empty plan. An atlas built with it installs the plan's base alone (§10.3 step 1).
    pub fn new() -> Self;

    /// Register one face at its own `weight` (a CSS weight in `100..=900`,
    /// `native-theme/src/model/font.rs:267-268`; values outside that range are clamped into
    /// it, never rejected) and `style`. `family` is matched against `ResolvedFontSpec::family`
    /// (`native-theme/src/model/font.rs:244`) by the rule on [`FontPlan`].
    pub fn face(
        self,
        family: &str,
        weight: u16,
        style: crate::FontStyle,
        bytes: FontBytes,
    ) -> Self;

    /// Register one **variable** face, letting this crate set the `wght` axis to the weight the
    /// theme asks for instead of requiring one file per weight. It is described to
    /// `select_face` at the weight asked for, which its axis reaches, so within its family and
    /// style it matches exactly. The coordinate is applied through `FontTweak::coords`
    /// (`epaint/src/text/fonts.rs:250`) with the infallible `Tag::new(b"wght")` (§8.3). A face
    /// registered here that has no `wght` axis (§8.3) renders at its own weight and is a
    /// [`crate::Note::FontWeightAxisUnsupported`].
    pub fn variable_face(
        self,
        family: &str,
        style: crate::FontStyle,
        bytes: FontBytes,
    ) -> Self;

    /// The OS's own faces for the theme's two families — `defaults.font` and
    /// `defaults.mono_font`, each asked for at its `weight` and `style` — from
    /// `native_theme::fonts::system_face` (§8.2), whose `SystemFace` carries the chosen face's
    /// bytes, its index in a collection file, and its own `family`, `weight` and `style` as
    /// fontdb records them; this crate reads the bytes, the weight and the style. Each face is registered at that weight and style — the face's own, not
    /// the ones asked for. A face with a `wght` axis
    /// gets the theme's weight as a coordinate (§8.3); a face whose weight differs from the
    /// theme's and has no `wght` axis renders at its own weight and is a
    /// [`crate::Note::FontWeightAxisUnsupported`]; a family the OS has no face of is a
    /// [`crate::Note::FontFamilyUnavailable`] and keeps egui's own face. The plan carries
    /// these lookup notes, and [`font_definitions`] returns them with its own, so they reach
    /// [`crate::ThemeAtlas::notes`] through [`crate::Builder::build`]. Feature
    /// `system-fonts`, which enables native-theme's own `system-fonts` feature. That the names
    /// the macOS and Windows readers report resolve is verified on those platforms' CI
    /// runners (§8.2, §13).
    ///
    /// No `#[must_use]` of its own: `FontPlan` is `#[must_use]`, and a second one on a function
    /// returning it trips `clippy::double_must_use`.
    #[cfg(feature = "system-fonts")]
    pub fn from_system(t: &ResolvedTheme) -> Self;

    /// The `FontDefinitions` the plan's faces are prepended to — by [`crate::ThemeAtlas::install`]
    /// (§10.3 step 1) and by [`crate::Builder::build`]'s row height (§6.15). Without this call:
    /// `egui::FontDefinitions::default()`, whose emoji fallbacks it keeps (§8.1).
    /// `Context::set_fonts` overwrites the whole definition (`egui/src/context.rs:2105-2106`), so
    /// an application's own fonts — a `FontFamily::Name` family its code names, a script
    /// fallback — survive an install only in this base; an `add_font` still pending at the next
    /// pass is applied on top of them (`egui/src/context.rs:556-574`). A dropped `Name` family
    /// panics in release at the first text that names it (`epaint/src/text/fonts.rs:1025`). The base cannot be
    /// recovered from the `Context` instead: `Context::fonts` panics before the first pass
    /// (`egui/src/context.rs:1097-1106`), and a face re-added with `Context::add_font` after an
    /// install is skipped while the loaded definitions still hold its name (`:2133-2143`).
    pub fn with_base(self, base: egui::FontDefinitions) -> Self;
}

/// Build a `FontDefinitions` that puts the theme's faces at the head of egui's `Proportional`
/// and `Monospace` chains. Pure: needs no `Context`, so it is fully testable and the
/// application may install the result itself. The starting point is the plan's base
/// ([`FontPlan::with_base`]; `FontDefinitions::default()`, with egui's emoji fallbacks,
/// without one, §8.1). If a resulting chain would be empty, the family is left as the base
/// had it.
///
/// **How a chain is reached is part of the contract**, because `FontDefinitions`'s two fields
/// are both `pub` (`epaint/src/text/fonts.rs:435`, `:443`) and a plan may carry a base that
/// binds neither family. Each chain is reached with
/// `families.entry(family).or_default()` — **never** `get_mut` — so the result binds both
/// `FontFamily::Proportional` and `FontFamily::Monospace` even when the base did not, which
/// is what keeps the unbound-family panic (`epaint/src/text/fonts.rs:1025`) unreachable. And
/// every name prepended to a chain is inserted into `font_data` in the same call, which is
/// what keeps the missing-font-data panic (`:1033`) unreachable. Both epaint constructors
/// already bind both families — `default()` at `:534-550` and `empty()` at `:563-564` — so the
/// `or_default` only matters for a hand-built base.
///
/// A face whose bytes fail epaint's parse, or whose `head` states a zero `unitsPerEm` (§8.2),
/// is not added, and the returned `Vec` carries a [`crate::Note::FontDataInvalid`] for it; the
/// family's chain is then left as the base had it. The `Vec` also carries the notes the plan
/// holds from `FontPlan::from_system`. Faces already in the base are the caller's and are
/// not checked.
#[must_use]
pub fn font_definitions(
    theme: &ResolvedTheme,
    plan: &FontPlan,
) -> (egui::FontDefinitions, Vec<crate::Note>);

/// The `wght` coordinate for a CSS weight (100–900), as the `VariationCoords` that egui takes
/// where it sets axes for a whole face or run: `FontTweak::coords`
/// (`epaint/src/text/fonts.rs:250`) of a face an application registers in a `FontDefinitions`
/// of its own rather than through a [`FontPlan`], and `TextFormat::coords`
/// (`epaint/src/text/text_layout_types.rs:505`) of a `LayoutJob` section. The counterpart of
/// iced's `to_iced_weight` (§4.7). Built with the infallible `Tag::new(b"wght")` (§8.3).
#[must_use]
pub fn weight_coords(css_weight: u16) -> egui::epaint::text::VariationCoords;
```

### 4.10 `mod icons`

native-theme icon payloads → egui image sources. §9 is the design: the boundary
is key, URI and `ImageSource`, never decoding (§9.1); the application installs
the loaders with `egui_extras::install_image_loaders` and its `svg` feature
(§9.1); the URI is a hash of the final bytes alone, so it
changes whenever the pixels do (§9.2); and the release-mode assert and the oversize test (§9.3). The lock discipline
every function here follows is §10.3's.

```rust,ignore
/// Re-exported **here and not at the crate root**, because `egui::IconData`
/// (`egui/src/viewport.rs:183`, the window/taskbar icon) already occupies that name and this
/// crate re-exports `egui`.
pub use native_theme::theme::IconData;

/// Which of native-theme's five per-context icon sizes to use
/// (`ResolvedIconSizes`, `native-theme/src/model/resolved.rs:15-26`). egui has no icon-size
/// vocabulary of its own (§9.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum IconContext {
    /// Small icon size for inline use: `defaults.icon_sizes.small`.
    Small,
    /// Icon size for toolbar buttons: `defaults.icon_sizes.toolbar`.
    Toolbar,
    /// Icon size for panel headers: `defaults.icon_sizes.panel`.
    Panel,
    /// Icon size for dialog buttons: `defaults.icon_sizes.dialog`.
    Dialog,
    /// Large icon size for menus and lists: `defaults.icon_sizes.large`.
    Large,
}

/// The icon size for a context, in logical pixels, for
/// `Image::fit_to_exact_size(Vec2::splat(..))` (`egui/src/widgets/image.rs:177`). Do **not**
/// pre-multiply by `Context::pixels_per_point`.
#[must_use]
pub fn icon_size(theme: &ResolvedTheme, context: IconContext) -> f32;

/// The inputs an icon is looked up and coloured by: the role or name, the set, the icon
/// theme, the size and the tint. They decide the bytes, whose hash alone makes the URI
/// (§9.2), and the role or name gives [`to_image`] its alt text. Constructed through the builder below: private
/// fields, because a
/// `#[non_exhaustive]` struct with no constructor cannot be built outside its defining crate
/// at all (`E0639`).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IconKey { /* private */ }

impl IconKey {
    /// Start a key for a role in a set.
    #[must_use] pub fn role(role: IconRole, set: IconSet) -> Self;
    /// Start a key for a named icon.
    #[must_use] pub fn name(name: &str, set: IconSet) -> Self;
    /// The freedesktop icon-theme name the icon was looked up in; given only when there is
    /// one — `if let Some(name) = atlas.icon_theme(ctx.theme()) { key = key.icon_theme(name) }`
    /// ([`crate::ThemeAtlas::icon_theme`], §9.2).
    #[must_use] pub fn icon_theme(self, icon_theme: &str) -> Self;
    /// The requested edge length in logical pixels; freedesktop lookup is size-dependent
    /// (`native-theme/src/icons.rs:137`). Stored as a private `u16` — the type
    /// `native_theme::icons::FreedesktopLoader::size` takes (`native-theme/src/icons.rs:137`)
    /// — computed as `(if points > 0.0 { points.round() } else { 0.0 }) as u16`, which is
    /// total over every `f32`: `NaN`, `-0.0` and every negative fold to the single key `0`,
    /// and the float-to-int `as` cast saturates at `65535` (§7.1). An integer, not an `f32`,
    /// is what lets `IconKey` derive `Eq` and `Hash`. Without this call the key has no size,
    /// and [`custom_icon_to_image_source`]'s freedesktop lookup takes `FreedesktopLoader`'s
    /// own default, 24 (`native-theme/src/icons.rs:126`).
    #[must_use] pub fn size(self, points: f32) -> Self;
    /// The colour a monochrome SVG icon is drawn in. [`to_image_source`] bakes it into the
    /// `IconData::Svg` bytes — with `native_theme::icons::colorize_monochrome_svg` for a
    /// bundled set, by replacing `currentColor` for a freedesktop icon — so the pixels, and
    /// through their hash the URI, carry it (§9.2). Not `egui::Image::tint`, a draw-time
    /// multiply that leaves black black. Unset, a bundled set's bytes are used as they are,
    /// which is right for a full-colour icon, and a freedesktop icon takes the text colour. A
    /// freedesktop icon whose own stylesheet sets its `color`, as nearly every Breeze
    /// `currentColor` icon does, keeps it, tint or not; the tint colours bundled monochrome sets and the
    /// freedesktop icons that set none (§9.2).
    #[must_use] pub fn tint(self, tint: egui::Color32) -> Self;
}

/// The URI of these bytes: `bytes://native-theme/` and the 16 lower-case hex digits of a
/// 64-bit hash of `icon`'s bytes and, for `IconData::Rgba`, its width and height, ending in
/// `.svg` for `IconData::Svg` (§9.2). Pass the final bytes; [`to_image_source`] calls it with
/// the bytes after colouring.
#[must_use]
pub fn uri(icon: &IconData) -> String;

/// Convert a decoded RGBA icon into an `egui::ColorImage`; `None`, never a panic, for a zero
/// side, an overflowing `width * height * 4` or a buffer of another length (§9.3), and for
/// an `IconData` variant this crate does not know (`IconData` is `#[non_exhaustive]`,
/// `native-theme/src/model/icons.rs:292`), and for an `IconData::Svg`, which this crate never
/// decodes (§9.1). The
/// oversize test needs a `Context` and lives in [`to_image_source`] and [`to_image`].
#[must_use]
pub fn to_color_image(icon: &IconData) -> Option<egui::ColorImage>;

/// Build an `egui::ImageSource` for an icon and register its URI for [`forget_icons`]:
/// `IconData::Svg` → `ImageSource::Bytes` under the `.svg` URI of its final bytes — for a
/// bundled set, coloured with the key's tint when it has one ([`IconKey::tint`]); for an
/// `IconSet::Freedesktop` key whose bytes still contain `currentColor` and set no `color`
/// of their own, `currentColor` replaced by the `#rrggbb` of the tint, else of
/// `defaults.text_color` of the installed atlas's `ResolvedTheme` for `ctx.theme()` (the
/// bytes as they are where no atlas is installed); a full-colour icon, and one whose
/// stylesheet sets its `color`, untouched (§9.2). Load a freedesktop icon for it with `FreedesktopLoader::color` in the
/// same colour (§9.2). `IconData::Rgba` →
/// `ImageSource::Texture`, uploaded once via `Context::load_texture`
/// (`egui/src/context.rs:2390`) under the same URI and reused afterwards. `None` for an
/// image with a side above `max_texture_side` — rejected, never resampled (§9.3) — and for
/// an `IconData` variant this crate does not know.
///
/// **Texture ownership is part of the contract.** `egui::TextureHandle` frees its texture on
/// drop (`epaint/src/texture_handle.rs:25-29`) while `ImageSource::Texture(SizedTexture)`
/// owns nothing (`egui/src/widgets/image.rs:586`), so this **stores** the handle in
/// `ctx.data_mut()` keyed by the URI — it is `Clone` (`epaint/src/texture_handle.rs:31-39`)
/// and `Send + Sync`, as `IdTypeMap::insert_temp` requires
/// (`egui/src/util/id_type_map.rs:425`) — and returns
/// `ImageSource::Texture(SizedTexture::from_handle(&handle))` (`egui/src/load.rs:473`). The
/// cache is read in one `ctx.data_mut`, the texture uploaded outside it, and inserted in a
/// second one, because `load_texture` itself takes the `Context` lock (§10.3).
#[must_use]
pub fn to_image_source(
    ctx: &egui::Context,
    key: &IconKey,
    icon: &IconData,
) -> Option<egui::ImageSource<'static>>;

/// [`to_image_source`], with its colouring and its URI, wrapped in an `egui::Image` with
/// `alt_text` from the key's role,
/// `IconRole::name()` (`native-theme/src/model/icons.rs:164`), or the key's name, which feeds
/// `WidgetInfo.label` (`egui/src/widgets/image.rs:406-410`) and is shown on load failure
/// (`:678-703`). `None` where [`to_image_source`] gives
/// `None`, an `IconData` variant this crate does not know included.
#[must_use]
pub fn to_image(
    ctx: &egui::Context,
    key: &IconKey,
    icon: &IconData,
) -> Option<egui::Image<'static>>;

/// Same, for an application-supplied [`IconProvider`]. Loads through native-theme's
/// custom-provider path for the key's set — `FreedesktopLoader::new(provider)` with the key's
/// icon theme and size, and `FreedesktopLoader::color` in the colour [`to_image_source`]
/// uses, for `IconSet::Freedesktop`, else `native_theme::icons::load_icon(provider, set)`
/// (`native-theme/src/icons.rs:487`), each trying `provider.icon_name(set)` then
/// `icon_svg(set)` — then colours and keys the bytes as [`to_image_source`] does, so the
/// provider's bytes are what the URI's hash covers. `None` where the provider has none. Build
/// the key with `IconKey::name(n, set)`, `n` its `icon_name(set)` where it has one; where it
/// has none, a name that describes it, which becomes its alt text — the URI is the hash of
/// the bytes alone (§9.2).
#[must_use]
pub fn custom_icon_to_image_source(
    ctx: &egui::Context,
    provider: &dyn IconProvider,
    key: &IconKey,
) -> Option<egui::ImageSource<'static>>;

/// Drop every icon this crate cached in `ctx`, leaving unrelated application images alone;
/// [`crate::ThemeAtlas::install`] calls it. Two stores, both required: the `bytes://` URIs
/// are released with `Context::forget_image` (`egui/src/context.rs:3764`), which reaches only
/// the loader caches (`:3771-3780`), and the stored `TextureHandle`s are removed from
/// `ctx.data_mut()`, which is what frees their textures — `forget_image` never reaches a
/// handle this crate stored.
pub fn forget_icons(ctx: &egui::Context);

/// The frame index to draw for a frame-animated icon, scheduling one wake-up at the next frame
/// boundary (§9.4). The caller hands that frame's `IconData`, `frames[i]`, to
/// [`to_image_source`] under the icon's one key, and the URI follows the frame's bytes (§9.2).
/// `None`, scheduling nothing, when the icon is not `AnimatedIcon::Frames` or
/// `reduced_motion` is `true` (pass `atlas.accessibility().reduce_motion` or
/// [`crate::is_reduced_motion`]) — draw `AnimatedIcon::first_frame()` then.
#[must_use]
pub fn animated_frame_index(
    ctx: &egui::Context,
    icon: &AnimatedIcon,
    reduced_motion: bool,
) -> Option<usize>;

/// The rotation angle in radians for a spin-animated icon, repainting every frame (§9.4);
/// `None`, scheduling nothing, when the icon is not `AnimatedIcon::Transform` with
/// `TransformAnimation::Spin`, or `reduced_motion` is `true`. Feed to
/// `Image::rotate(angle, Vec2::splat(0.5))` (`egui/src/widgets/image.rs:239-243`), which
/// forces `corner_radius = ZERO` (`:241`).
#[must_use]
pub fn spin_angle(
    ctx: &egui::Context,
    icon: &AnimatedIcon,
    reduced_motion: bool,
) -> Option<f32>;
```

### 4.11 `ThemeWatcher` (feature `watch`)

§10.1 says why egui needs no poll timer and §10.2 gives the threading contract
and what fires the watcher on each platform; these are the signatures.

```rust,ignore
/// Watch for OS theme changes, rebuild the atlas off the UI thread with the application's
/// `rebuild` closure, and wake egui with `ctx.request_repaint()` (§10.2). Installation stays
/// on the UI thread: [`ThemeWatcher::take`] is the hand-off.
#[cfg(feature = "watch")]
#[must_use = "dropping the watcher stops it immediately"]
pub struct ThemeWatcher { /* private */ }

#[cfg(feature = "watch")]
impl ThemeWatcher {
    /// Start watching. `rebuild` runs on the watcher thread after each change event and
    /// returns the atlas to hand over: [`ThemeWatcher::system_rebuild`] for an application on
    /// the OS theme, else a closure that builds exactly what the application built at start-up,
    /// every builder input included, after `native_theme::detect::invalidate_caches()`
    /// (`native-theme/src/detect.rs:155`) when it reads the OS (§10.2).
    ///
    /// Bind the result: dropping it stops the watch and joins the thread
    /// (`native-theme/src/watch/mod.rs:172-187`). `Send` but not `Sync`, like native-theme's
    /// subscription (`native-theme/src/watch/mod.rs:107-109`).
    ///
    /// # Errors
    /// Propagates `native_theme::watch::on_theme_change` (`native-theme/src/watch/mod.rs:217`),
    /// which reports `Error::WatchUnavailable` on desktops and feature sets it cannot watch
    /// (`:234-266`). A `rebuild` error is not returned here; it is kept for
    /// [`ThemeWatcher::last_error`].
    pub fn start(
        ctx: &egui::Context,
        rebuild: impl Fn() -> native_theme::Result<ThemeAtlas> + Send + 'static,
    ) -> native_theme::Result<ThemeWatcher>;

    /// The OS-theme rebuild: `native_theme::detect::invalidate_caches()`, then
    /// [`from_system`](crate::from_system)'s atlas.
    ///
    /// # Errors
    /// Propagates [`from_system`](crate::from_system).
    pub fn system_rebuild() -> native_theme::Result<ThemeAtlas>;

    /// Take the pending atlas, if the OS theme changed since the last call. Non-blocking, and
    /// `None` on the overwhelming majority of frames. Install the result; `install` flushes
    /// the icon cache and requests the repaint (§10.3).
    #[must_use]
    pub fn take(&self) -> Option<ThemeAtlas>;

    /// The most recent `rebuild`'s error, as text — a `String`, not an [`Error`](crate::Error), because
    /// `native_theme::error::Error` is not `Clone` (`native-theme/src/error.rs:58-60`); `None`
    /// again once a later `rebuild` succeeds.
    #[must_use]
    pub fn last_error(&self) -> Option<String>;
}
```

### 4.12 Names that must never appear

`EguiTheme`, `EguiThemeSet`, `StyleSet`, `StyleSheet`, `WidgetRole`, `Metrics`,
`InstallReport`, `FontProvider`, `FontFaces`, `FontRequest`, `FontMatch`,
`BundledFonts`, `AnimatedImageSources`, `widgets::Switch`, `EGUI_VERSION`,
`egui::SidePanel`, `egui::TopBottomPanel`, `egui::StyleModifier`,
`Panel::show_inside`, `CentralPanel::show_inside`, `refresh_line_spacing`,
`themed`, `themed_disabled`, `themed_frame`, `Role::CheckboxOn`,
`Role::CheckboxOff`, `Role::ButtonPrimary`, `metrics::IconSizes`.

Three of these are not merely style preferences but compile errors:
`egui::SidePanel` and `egui::TopBottomPanel` **do not exist** in egui 0.36.2 (a
recursive grep over `egui/src` returns zero hits; the types are `Panel`,
`containers/panel.rs:206`, and `CentralPanel`, `:1187`), and
`egui::StyleModifier` is not re-exported — `egui/src/lib.rs:488` re-exports only
`style::{FontSelection, Spacing, Style, TextStyle, Visuals}`, so the path is
`egui::style::StyleModifier`. `Panel::show_inside` (`containers/panel.rs:428`)
and `CentralPanel::show_inside` (`:1218`) are both
`#[deprecated = "Renamed to \`show\`"]`.

---

## 5 -- The complete mapping

All 482 leaves — the 478 of `ResolvedTheme` and the four of `LayoutTheme` —
one row each, in six groups. Each row carries **one** verdict, graded by §2:
what this crate carries — the `Style` field or `Frame` it writes, or the
per-call input it documents. Where the crate declines a route egui offers, the
row says so and §5.8 lists the decline. A base owner (§5.9) is DIRECT when its
one base-style write renders it on its own widget; §2 names the per-widget rows
this makes DIRECT. The totals are §5.7's alone. Column 2 gives the egui sink
path; an em dash means no sink exists.
`<state>` means the write is repeated across the `Widgets` entries §6.1 writes
the leaf into — its own entry and the entries §6.1 copies from it, in the
order *Which entries a role writes* gives; an entry that takes the leaf only
through a formula or a `None` soft option's fallback is carried in
`mapping.toml` as a `when` sink (§13.1) — and `{5}` names all five entries — `noninteractive`,
`inactive`, `hovered`, `active`, `open` (`style.rs:1250-1270`) — and `{a,b}`
the ones listed. "«scope»" means the write lands in that `Role`'s
`Arc<Style>`, not on the base style; "«Role, Variant»" names one `RoleVariant`
cell of it (§4.4, §6.2, §6.3). "Per call" means the value is handed to egui at
the call site rather than installed.

**What counts as a route.** A leaf is carried wherever a stock egui widget or
container renders it from something it is handed: a `Style` field (DIRECT or
SCOPED), an `egui::Frame` a `Surface` recipe builds (SCOPED), or a per-call
input of the widget itself — a builder method such as `Button::min_size`, a
format in its own `RichText`, a `Ui` sizing call, an `Image` size, a `Frame`
the application lays round it, the icon closure `CollapsingHeader::icon` takes
— which is DERIVED: the value reaches the widget through the application's own
call, read from `ResolvedTheme` or from an accessor of §4.7. Two things are not routes, and leave a leaf
UNMAPPABLE: faking a state the widget ignores (§2); and a shape the
application would draw from scratch with a `Painter` because the widget draws
none (a switch knob, slider ticks) — where the companion widgets crate paints
the value in a widget of its own — such a shape, or a colour or width the stock
widget hardcodes — the row's sub-tag is `widgets-crate` (§2).
A per-call input may follow the widget's own interaction state:
`Context::read_response` hands back a widget's `Response` before the widget is
added, because "widget interaction happens at the start of the pass, using the
widget rects from the previous pass" (`egui/src/context.rs:1350-1355`), so the
application chooses the leaf for the state the widget is in and hands it to the
widget's own builder — a link's hover and pressed text colours (§5.3), a text
field's focused stroke (`input_frame`, §4.7). A state egui does not record at
all, a visited link, is the application's to keep, and its leaf reaches the
widget the same way (§5.3). The one shape the
**connector** paints is the keyboard focus ring: the install plugin draws it at
the end of every pass around the focused widget (§5.1, §5.8 item 1), with no
application code at all, so it is the connector's own route — DERIVED by §2,
because it needs a live `Context` rather than a `Style` write.

`ResolvedFontSpec` (`native-theme/src/model/font.rs:242-273`) expands to
`{family, size, defined_size, weight, style, color}`. `defaults.border` is a
`ResolvedDefaultsBorder` (`native-theme/src/model/border.rs:212-225`) and expands
to `{color, corner_radius, corner_radius_lg, line_width, opacity,
shadow_enabled}`; every widget's `border` is a `ResolvedWidgetBorder`
(`:233-246`) and expands to `{color, corner_radius, line_width, shadow_enabled,
padding.top, padding.right, padding.bottom, padding.left}`. All three are
expanded in every table because their leaves land in different egui fields with
different verdicts. A widget border has no large radius and no opacity, so
`defaults.border.corner_radius_lg` and `defaults.border.opacity`
(`native-theme/src/model/resolved.rs:131`) are the only sources of either; a
`border_kind = "full_lg"` widget already receives the large radius as its own
`border.corner_radius` (§5.2).

**Text sizes carry the user's text-scaling factor.** Every text size the atlas
writes — each `text_styles` entry and each `override_font_id` size, on the base
style and in every role style — and every size an accessor returns
(`text_role_font`, `list_header_font`) is `scaled_text_size(size,
&AccessibilityPreferences)` of the leaf (§4); the `.size` rows say so with
"× text-scaling factor". No other length is scaled. The preferences are not a
`ResolvedTheme` leaf, so they add no row.

**Padding is four optional sides (decision S1).** `ResolvedPadding`
(`native-theme/src/model/border.rs:195-204`) states each side as `Option<f32>`,
and a widget's four `border.padding.*` rows map one-to-one onto the four sides of
that widget's `epaint::Margin` sink. `Some(x)` goes through the saturating
conversion of §7. A side that resolves to `None` is a side the theme does not
state, and it keeps the value egui's own default `Style` for that scheme has on
that side — for a `Frame` built from a contested global, egui's default for
that global, never another leaf's value (`spacing.menu_margin`, which the
popups' frames read, keeps egui's default on the base style too, §5.9). That is `convert::to_margin(base, &padding)` (§4.8), with `base` the
default at that sink.

**Where egui draws the border inside the padding.** A platform's padding lies
inside its border: the outer size is border + padding + content
(`docs/platform-facts.md:900-902`; WinUI's button, 5 + 6 padding plus 2 border,
`:409`). A `Frame` agrees — its stroke lies outside `inner_margin`
(`egui/src/containers/frame.rs:327-331`) — so a surface's padding goes to
`to_margin` as stated. Four widgets do not: `Button` gives its frame the margin
`button_padding + expansion − bg_stroke.width`
(`egui/src/widget_style.rs:163-165`), which the frame's stroke brings back to
`button_padding`; `ComboBox`'s `button_frame` and the collapsing header stroke
`StrokeKind::Inside` within `content ± button_padding`
(`egui/src/containers/combo_box.rs:439-460`,
`egui/src/containers/collapsing_header.rs:531`, `:566-567`); and `TextEdit`
allocates with its margin alone and paints its frame with
`margin + expansion − stroke.width` (`egui/src/widgets/text_edit/builder.rs:713`,
`:763-767`). In all four the stroke is drawn *within* the padding egui is
given, so the outer size is content + 2 · padding whatever the stroke width.
The connector therefore writes, for these widgets' padding rows, the stated
side **plus the role's `border.line_width`** —
`convert::padding_with_border` (§4.8), through `to_button_padding` and
`input_margin` — and the native outer size is exact wherever the theme's
padding excludes the border, as `docs/platform-facts.md:900-902` defines it. One
recorded value does not: macOS's button vertical padding is `(22 − 16) / 2`,
outer height less content (`docs/platform-facts.md:1172`), so it already holds
the border, and on `macos-sonoma` the button comes out one `border.line_width`
per side taller than native until that derivation is corrected at its source
(`docs/todo.md`). Every padding row whose
written value adds the line width is therefore **DERIVED**, whether or not a
mean is also involved: what reaches the egui field is a formula over two
leaves, the side and `border.line_width`, not the side itself (§2). A state
layer is different — §2 grades it by its field, because the composite *is* the
colour the platform paints — while an egui padding that holds the stroke is not
the native padding and has to be computed from it. `expansion` does not
interact: the connector writes it in the `Role::Slider` scope only (§6.6), so
every other scope keeps egui's `0.0` (`egui/src/style.rs:1689`); inside a
slider scope the value box's frame sits `−expansion` inside its allocation,
the cost §6.6 records.
A side the theme leaves `None` keeps egui's own padding unchanged, since egui's
padding already holds egui's own stroke.

**One sink holds one value per axis.** `Spacing::button_padding` is a single
`Vec2` (`egui/src/style.rs:398`), and every widget that reads it applies it
symmetrically: `button_style` turns it into a symmetric margin
(`egui/src/widget_style.rs:163-165`, via `Margin::symmetric` at
`epaint/src/margin.rs:117-119`), `ComboBox` shrinks and grows by it with
`shrink2`/`expand2` (`egui/src/containers/combo_box.rs:439`, `:443`), and
`CollapsingHeader` pads its height by `2.0 * button_padding.y`
(`egui/src/containers/collapsing_header.rs:531`). None of these widgets has a
per-side route to use instead: `Button` offers no margin builder and always
takes its frame's margin from `button_style` (`egui/src/widgets/button.rs:331`,
`:333-356`), and neither `button_frame` nor the collapsing header takes a
margin. On the `button`, `segmented_control`, `tab` and `combo_box` rows, left
and right therefore share `.x` and top and bottom share `.y`; the expander
shares `.y` only, because it reads `.x` on its trailing side alone (§5.6).

**The rule for a shared component: the mean of its two targets.** Per side,
the *target* is the side the theme states, or — for a side it leaves `None`, or
states as a non-finite number — the starting style's own component on that
side — egui's `vec2(4.0, 1.0)` (`egui/src/style.rs:1458`) on the base style, the
base style's inside a role cell (§3.4) — exactly as `to_margin` keeps
egui's side for an unstated side of a four-sided sink. The component written is
the **mean of the two targets**: `(left + right) / 2` for `.x`,
`(top + bottom) / 2` for `.y`, floored at `0.0`, a stated side's target
carrying the border's line width as above — with both sides stated, the mean
of the pair plus the line width. The reason is the goal, the
native look: for two targets `a` and `b` and any one value `v` applied to both
sides, the widget's outer size along that axis is exact only when
`2v = a + b`, and the larger of the two per-side errors,
`max(|a − v|, |b − v|)`, is at least `|a − b| / 2`, with equality only at
`v = (a + b) / 2`. The mean is therefore the one value that keeps the native
outer size exactly *and* has the smallest worst-side error: the content moves
off-centre by `|a − b| / 2` and by nothing else. Every alternative is worse on
both counts — taking either side for both changes the size by `|a − b|`, and
leaving egui's value, which is what an unequal pair would otherwise get, is
wrong on both sides and on the size: `windows-11` states button top 5 / bottom
6 (`native-theme/src/presets/windows-11.toml:98-99`) inside a 1-point border
(`:43`), so the connector writes `5.5 + 1 = 6.5`, half a point off-centre,
where egui's `1.0` on both sides would make the button 11 points shorter than
native (content + 2 against content + 1 + 5 + 6 + 1). When both sides
are unstated the result is egui's own component exactly, and when both are
stated and equal it is that value exactly; neither case loses anything. The
mean is a derivation with a stated formula over two platform values, not an
invented number, so every paired row is **DERIVED** — lossy only in centring,
by `|a − b| / 2` — and the loss is a §14 ledger entry. It is not hypothetical:
besides the button, `windows-11` states combo box left 12 / right 0 and top 5 /
bottom 7 (`:340-343`), written as `6.0 + 1 = 7.0` on both axes, and
`kde-breeze` combo box left 6 / right 0
(`native-theme/src/presets/kde-breeze.toml:295-296`) inside a 1-point border
(`:43`), written as `3.0 + 1 = 4.0`, each from `docs/platform-facts.md:1172`,
`:1552` and `:1557`. One widget adds a rounding of its own that no choice of
`v` can avoid: `Button` narrows `button_padding + expansion − bg_stroke.width`
— the mean itself, once the line width is added and taken away — to whole
points (`egui/src/widget_style.rs:163-165` → `epaint/src/margin.rs:117-119`),
so with `a + b` odd — the `windows-11` button — its height is one point off
native whatever `v` is written, while `ComboBox` and `CollapsingHeader` keep
the `f32` and are exact.

### 5.1 Foundation — `defaults`, `text_scale`, `layout`

`ResolvedDefaults` is `native-theme/src/model/resolved.rs:72-146`: 31 declared
fields, 50 leaves.

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `defaults.font.family` | `text_styles[Body].family` + `FontDefinitions` bytes | DERIVED | egui needs bytes, not a name (`epaint/src/text/fonts.rs:112-122`). §8.2 |
| `defaults.font.size` | `text_styles[Body].size` (`style.rs:289`, `:77`) | DIRECT | base owner; egui default `13.0` (`style.rs:1419`); × text-scaling factor (§4) |
| `defaults.font.defined_size` | — | UNMAPPABLE `egui-limited` | the size in the unit its source stated (`native-theme/src/model/font.rs:266`), kept for showing a size to a person. `FontId` holds one length, `size` "in points" (`epaint/src/text/fonts.rs:22-23`), and one logical pixel is one point (§2), so `.size` already carries everything egui renders; egui has no field for the unit a size was stated in. An application that displays sizes reads this leaf from `ResolvedTheme` |
| `defaults.font.weight` | `FontTweak::coords` `wght` (`epaint/src/text/fonts.rs:250`) | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`; upstream's own `// TODO(emilk)` at `:27`). §8.3 |
| `defaults.font.style` | matched face in the `FontPlan` | DERIVED | no italic anywhere in `Style`. §8.4 |
| `defaults.font.color` | — | UNMAPPABLE `egui-limited` | egui has one non-interactive text colour, `widgets.noninteractive.fg_stroke.color` (`style.rs:1311`), which the base owner `defaults.text_color` takes (§5.9), and `defaults` has no `Role` scope (§4.4), so nothing carries a second one. Rendered correctly whenever it equals `defaults.text_color`, which it copies whenever a theme omits it (`docs/inheritance-rules.toml:56`, enforced at `native-theme/src/resolve/inheritance.rs:109-112`), and the platform facts record the two as one value (`docs/platform-facts.md:1062-1063`): no bundled preset states it in either mode, and no platform reader writes it — the readers write `defaults.text_color` (`native-theme/src/kde/colors.rs:23`, `native-theme/src/windows.rs:451`). Only a theme that sets `color` under `[*.defaults.font]` apart from `text_color` loses it. A per-call colour on every body text would replace the base owner call by call, not route this leaf: no call site is its own |
| `defaults.line_height` | `spacing.extra_text_line_spacing` (`style.rs:424`) | DERIVED | multiplier → additive delta, computed once at build: `Builder::build` derives egui's row height from the face's own metrics exactly as epaint does (`epaint/src/text/font.rs:561-565`, `:587`), per variant, and writes the difference to the native line height, floored at `0.0`, into the base style, which every role style starts from (§3.4). Measured with the KDE and GNOME platform faces the difference is below zero, so egui's `0.0` is written there. §6.15 |
| `defaults.mono_font.family` | `FontDefinitions.families[Monospace]` head | DERIVED | prepend into the existing chain (`epaint/src/text/fonts.rs:534-542`) |
| `defaults.mono_font.size` | `text_styles[Monospace].size` (`style.rs:80`) | DIRECT | the only uncontested text style; × text-scaling factor (§4) |
| `defaults.mono_font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` |
| `defaults.mono_font.weight` | `FontTweak::coords` `wght` on the mono entry | DERIVED | as `font.weight` |
| `defaults.mono_font.style` | matched face | DERIVED | as `font.style` |
| `defaults.mono_font.color` | `RichText::color` (`widget_text.rs:320`) on `RichText::code()`/`monospace()` text (`:239`, `:245-248`), or `TextEdit::text_color` (`widgets/text_edit/builder.rs:251`, used at `:480-483`) on a `TextEdit::code_editor` (`:162`), per call | DERIVED | no `Style` field: `RichText::code` changes the font and the background only (`widget_text.rs:440-441`), and egui's one non-interactive text colour is `defaults.text_color`'s. Monospace text has call sites of its own, where a colour carries exactly this leaf, as `dialog.title_font.color` (§5.2). A copy of `defaults.font.color` whenever a theme omits it (`docs/inheritance-rules.toml:57`), so rendered correctly on every bundled preset as that row is |
| `defaults.background_color` | `visuals.panel_fill` (`style.rs:1072`) + `widgets.noninteractive.bg_fill` and `widgets.noninteractive.weak_bg_fill` | DIRECT | read at `frame.rs:188`, `:192`; the two fills keep egui's own pairing of the non-interactive entry with the panel (`from_gray(27)` in `Widgets::dark`, `style.rs:1684-1685`, as `panel_fill` at `:1532`; `from_gray(248)` in `Widgets::light`, `:1729-1730`, as at `:1591`), which `Visuals::gray_out` tints towards (`:1186`) |
| `defaults.text_color` | `widgets.noninteractive.fg_stroke.color` and `widgets.active.fg_stroke.color` of the base style | DIRECT | base owner of both; doc `style.rs:1254`. The `active` write is egui's strong text: every `RichText::strong()`, `ui.strong()` and default `Spinner` reads `widgets.active.text_color()` (`style.rs:1147-1149`, read at `widget_text.rs:485`), so on the base style that field carries the panel's text colour, not a pressed button's (§5.3 `button.active_text_color`) |
| `defaults.accent_color` | — | UNMAPPABLE `egui-limited` | `Visuals` has no accent field (`style.rs:989-1127`) and no role writes this leaf. Every place a platform paints the accent reaches egui through a leaf that inherits it — `defaults.selection_background`, `defaults.focus_ring_color`, `button.primary_background`, `checkbox.checked_background`, `slider.fill_color`, `progress_bar.fill_color`, `switch.checked_background`, `segmented_control.active_background` (`docs/inheritance-rules.toml:51`, `:52`, `:156`, `:174`, `:195`, `:201`, `:244`, `:256`) — each graded in its own row, as for `defaults.surface_color` |
| `defaults.accent_text_color` | — | UNMAPPABLE `egui-limited` | same shape, through `button.primary_text_color`, `checkbox.indicator_color` and `segmented_control.active_text_color` (`docs/inheritance-rules.toml:157`, `:175`, `:257`) |
| `defaults.surface_color` | — | UNMAPPABLE `egui-limited` | the content-surface token (`controlBackgroundColor`, `CardBackgroundFillColorDefault`, `[Colors:View] BackgroundNormal`, `docs/platform-facts.md:1050`) has no egui field: egui's frame presets fill from `window_fill` (`Frame::window`/`menu`/`popup`, `frame.rs:201`, `:210`, `:219`), `panel_fill` (`:188`, `:192`), `extreme_bg_color` (`Frame::canvas`, `:231`), a literal (`Frame::dark_canvas`, `:236`) or nothing (`Frame::group`, `:178-183`), and every `Surface` frame sets its own surface's fill leaf (§3.4), so no route carries the token itself; `window_fill`'s base owner is `menu.background_color` (§5.9). Nothing native is lost by this row: where a platform paints the token it does so through a widget leaf that inherits it — `window.title_bar_background`, `slider.thumb_color`, `list.header_background`, `switch.thumb_background`, `card.background_color`, and on Windows `popover.background_color` (`docs/inheritance-rules.toml:150`, `:197`, `:231`, `:245`, `:261`, `:321`) — and each of those is graded in its own row |
| `defaults.muted_color` | `visuals.weak_text_color = Some(..)` (`style.rs:1027`) | DIRECT | base owner; makes `weak_text_alpha` dead by design (`style.rs:1020`) |
| `defaults.shadow_color` | `visuals.window_shadow.color` + `popup_shadow.color` (`:1062`, `:1074`) | DIRECT | two sinks, one value, no rival **on the colour** — the `border.shadow_enabled` gates claim the same two `Shadow` fields but not this sub-field (§5.9, and the shadow-counting convention in §5.11) |
| `defaults.link_color` | `visuals.hyperlink_color` (`style.rs:1036`) | DIRECT | base owner (§5.9); sole reader `widgets/hyperlink.rs:47` |
| `defaults.selection_background` | `visuals.selection.bg_fill` (`style.rs:1196`) | DIRECT | base owner of a contested field (§5.11) |
| `defaults.selection_text_color` | `visuals.selection.stroke.color` (`:1199`) | DIRECT | base owner; keep `stroke.width` at egui's `1.0` (`style.rs:1620`) |
| `defaults.selection_inactive_background` | — | UNMAPPABLE `source-side gap` | no unfocused-window styling anywhere in `Visuals` (`style.rs:989-1127`), but a route exists: the install plugin, which already reads the window's focus (`ctx.input(\|i\| i.focused)`, §6.18), could swap `selection.bg_fill` for this leaf while the window is unfocused. `ResolvedTheme` has no text colour to pair with it, so the swap would paint the focused `selection_text_color` on the unemphasised fill — on macOS, the one platform that states it (`unemphasizedSelectedContentBackgroundColor`, `docs/platform-facts.md:1057`; reader `native-theme/src/macos.rs:107`). §14 item 45; `selection_inactive_background()` accessor (§4.7) |
| `defaults.text_selection_background` | — | UNMAPPABLE `egui-limited` | egui has one selection pair for selected widgets and selected text alike (`text_selection/visuals.rs:39-40`). The base style's is `defaults.selection_background`'s (§5.9), which selectable `Label`s use; the `Role::Input` cell holds `input.selection_background`, which inherits this leaf (`docs/inheritance-rules.toml:167`). Lost only where a theme states it apart from both; no bundled preset states it and no reader writes it (`native-theme/src/resolve/inheritance.rs:101-104`) |
| `defaults.text_selection_color` | — | UNMAPPABLE `egui-limited` | same shape, through `input.selection_text_color` (`docs/inheritance-rules.toml:168`; `native-theme/src/resolve/inheritance.rs:105-108`) |
| `defaults.disabled_text_color` | — | UNMAPPABLE `egui-limited` | egui has a disabled *opacity*, not a disabled colour (`ui.rs:497-502`), and `defaults` has no `Role` scope (§4.4), so nothing carries this leaf itself. It reaches the screen through the seven per-widget `disabled_text_color` leaves that inherit it (`docs/inheritance-rules.toml:161`, `:170`, `:177`, `:185`, `:269`, `:278`, `:287`), each in its role's `Disabled` cell (§6.3) |
| `defaults.danger_color` | `visuals.error_fg_color` (`style.rs:1059`) | DIRECT | readers `painter.rs:284`, `context.rs:1184`, `widgets/image.rs:688`; emitted as the platform gives it, with no contrast adjustment (C19) |
| `defaults.danger_text_color` | `RichText::color` (`widget_text.rs:320`) on the text, per call | DERIVED | text beside a status indicator on macOS, KDE and GNOME, contrast text on the status fill on Windows (`docs/platform-facts.md:1089-1098`): either way application text, which a colour in its own format carries exactly — on a status fill, a `Button` or `Frame` the application fills with the status colour (§5 introduction). `Visuals` has no status text colour (`style.rs:1056`, `:1059`) |
| `defaults.warning_color` | `visuals.warn_fg_color` (`style.rs:1056`) | DIRECT | sole reader `egui/src/lib.rs:507`; as `danger_color`, no contrast adjustment (C19) |
| `defaults.warning_text_color` | `RichText::color` (`widget_text.rs:320`) on the text, per call | DERIVED | as `danger_text_color`; `warning_text_color()` (§4.7) |
| `defaults.success_color` | `RichText::color` (`widget_text.rs:320`) on the status text, `Button::fill` (`widgets/button.rs:143`) on a status button, `IconKey::tint` (§4.10) on a status icon, per call | DERIVED | `Visuals` models `warn_fg_color` and `error_fg_color` only (`style.rs:1056`, `:1059`); the application reads this leaf from `ResolvedTheme` and hands it to the text's own format — as it hands egui's own `error_fg_color` through `Ui::colored_label` (`ui.rs:1703`) — or to the button's fill or the icon's tint |
| `defaults.success_text_color` | `RichText::color` (`widget_text.rs:320`) on the text, per call | DERIVED | as `danger_text_color` |
| `defaults.info_color` | `RichText::color` (`widget_text.rs:320`) on the status text, `Button::fill` (`widgets/button.rs:143`) on a status button, `IconKey::tint` (§4.10) on a status icon, per call | DERIVED | as `success_color`, through `info_color()` (§4.7) |
| `defaults.info_text_color` | `RichText::color` (`widget_text.rs:320`) on the text, per call | DERIVED | as `danger_text_color`; `info_text_color()` (§4.7) |
| `defaults.border.color` | `widgets.noninteractive.bg_stroke.color` + `visuals.window_stroke.color` | DIRECT | base owner; egui's universal hairline (doc `style.rs:1686`) |
| `defaults.border.corner_radius` | `widgets.noninteractive.corner_radius` (`style.rs:1308`) | DIRECT | base owner of the non-interactive state — frames, separators, group boxes; the four interactive states take `button.border.corner_radius` (§5.9, §6.1) |
| `defaults.border.corner_radius_lg` | `visuals.menu_corner_radius` (`:1069`) | DIRECT | base owner; the only *live* `corner_radius_lg` in the model. `visuals.window_corner_radius` (`:1061`) belongs to `window.border.corner_radius` instead (§5.2, §5.9), which under `border_kind = "full_lg"` already *is* the large radius |
| `defaults.border.line_width` | `widgets.noninteractive.bg_stroke.width` + `window_stroke.width` | DIRECT | base owner, as `defaults.border.color`; the four interactive states take `button.border.line_width` (§5.9, §6.1); `f32`→`f32`, but `NaN <= 0.0` is false (`epaint/src/stroke.rs:35-37`) |
| `defaults.border.opacity` | folded into the alpha of every border stroke's colour — the strokes a `border.color`, `checkbox.unchecked_border_color` or `input.hover_border_color` leaf fills, on the base style, in every cell and in every `Surface` frame; a separator, grid, divider or menu-separator line keeps its own stated colour | DERIVED | `Stroke` has no alpha channel (`epaint/src/stroke.rs:13-16`); the model states the multiplier for the border colour alone (`docs/platform-facts.md:997`). §6.13 |
| `defaults.border.shadow_enabled` | `visuals.popup_shadow` | DERIVED | boolean gate; geometry stays egui's. The `window_shadow` gate is `window.border.shadow_enabled`'s (§5.9), which inherits this leaf in the model but is resolved on its own. §6.14 |
| `defaults.disabled_opacity` | `visuals.disabled_alpha` (`style.rs:1126`) | DIRECT | base owner; **must** pass `unit_interval()` (§7.4) |
| `defaults.focus_ring_color` | the ring the install plugin paints in `Plugin::on_end_pass` (`plugin.rs:32`) | DERIVED | needs a live `Context`: the ring is drawn round `ctx.memory(\|m\| m.focused())`'s rect in this pass's widget table (`Context::viewport`, `context.rs:3963-3965`; §6.18) on `Context::layer_painter` of that widget's layer (`context.rs:1587`), always, and only while the window has OS focus — the gate egui's own `Response::has_focus` applies (`response.rs:350`). Painted for scoped and unscoped widgets alike. `widgets.active.bg_stroke` is **not** the sink — §5.8 item 1; `focus_ring_color()` accessor |
| `defaults.focus_ring_width` | the plugin's ring stroke width | DERIVED | as `focus_ring_color` |
| `defaults.focus_ring_offset` | the plugin's ring rect, `rect.expand(offset)` | DERIVED | as `focus_ring_color`; a negative offset insets the ring, as the platforms that state one inset mean it (`docs/platform-facts.md:1106`). Never clamp to 0. The ring's corners take the active corner radius of the nearest `native_scope`/`native_set_style` scope that encloses the focused widget, and outside every scope the root `Ui`'s style's at the end of the pass — the base style's `widgets.active.corner_radius`, `button.border.corner_radius` (§5.9), unless the application left a role on the root (§6.18). `register_focus_shape` (§4.3) is only for an outline that is not the scope's rounded rectangle, such as a switch's track |
| `defaults.icon_sizes.toolbar` | `Image::fit_to_exact_size` (`widgets/image.rs:177`), per call | DERIVED | egui has no icon-size vocabulary in `Style`; the `icons::icon_size` accessor feeds the stock `Image`, which renders the size exactly |
| `defaults.icon_sizes.small` | `Image::fit_to_exact_size`, per call | DERIVED | same |
| `defaults.icon_sizes.large` | `Image::fit_to_exact_size`, per call | DERIVED | same |
| `defaults.icon_sizes.dialog` | `Image::fit_to_exact_size`, per call | DERIVED | same |
| `defaults.icon_sizes.panel` | `Image::fit_to_exact_size`, per call | DERIVED | same |

`ResolvedTextScale` is `native-theme/src/model/resolved.rs:54-63` — four roles,
**no `body` role**. `ResolvedTextScaleEntry` is `:37-48`.

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `text_scale.caption.size` | `text_styles[Small].size` (`style.rs:74`) | DIRECT | reachable only via `RichText::small()` (`widget_text.rs:292`); × text-scaling factor (§4) |
| `text_scale.caption.weight` | `text_role_weight()` + `RichText::variation` | DERIVED | one weight per family under §8.3; §5.8 item 3 |
| `text_scale.caption.line_height` | `text_role_line_height()` + `RichText::line_height` (`widget_text.rs:174`) | DERIVED | no per-`TextStyle` line height in `Style`; the accessor (§4.7) feeds the one exact per-call mechanism, the route `.size` and `.weight` take |
| `text_scale.section_heading.size` | `text_styles[Heading].size` (`style.rs:88`) | DIRECT | base owner; readers `widget_text.rs:234`, `window.rs:1313`, `:1350`; × text-scaling factor (§4) |
| `text_scale.section_heading.weight` | `text_role_weight()` + `RichText::variation` | DERIVED | §5.8 item 3 |
| `text_scale.section_heading.line_height` | `text_role_line_height()` + `RichText::line_height` | DERIVED | as above |
| `text_scale.dialog_title.size` | `text_role_font()` + `RichText::font` (`widget_text.rs:190-198`) | DERIVED | egui has one `Heading`; §5.8 item 4; × text-scaling factor (§4) |
| `text_scale.dialog_title.weight` | `text_role_weight()` + `RichText::variation` | DERIVED | §5.8 item 3 |
| `text_scale.dialog_title.line_height` | `text_role_line_height()` + `RichText::line_height` | DERIVED | as above |
| `text_scale.display.size` | `text_role_font()` + `RichText::font` | DERIVED | §5.8 item 4; × text-scaling factor (§4) |
| `text_scale.display.weight` | `text_role_weight()` + `RichText::variation` | DERIVED | §5.8 item 3 |
| `text_scale.display.line_height` | `text_role_line_height()` + `RichText::line_height` | DERIVED | as above |

`LayoutTheme` is `native-theme/src/model/widgets/mod.rs:896-913`: four
`Option<f32>` fields. It lives on `native_theme::theme::Theme`
(`native-theme/src/model/mod.rs:269`) and on `SystemTheme` as `layout`
(`native-theme/src/lib.rs:488`), **not** on `ResolvedTheme`, so it reaches the
connector beside the resolved theme on both the preset path and the
`from_system()` path. A `None` field is one that neither the platform nor the
preset states (`native-theme/src/lib.rs:484-487`); egui's own value stands for
it.

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `layout.widget_gap` | `spacing.item_spacing: Vec2` (`style.rs:392`) | DERIVED | one native number, an asymmetric `Vec2` sink. §6.16. `None` leaves egui's `item_spacing` |
| `layout.container_margin` | `Frame::inner_margin` (`frame.rs:248`) on the container's own `Frame`, per call | DERIVED | no `Style` field: `Frame::group` hardcodes `6` (`frame.rs:180`). Read from [`ThemeAtlas::layout`] (§4.2); a `Frame` the application builds for a nested container carries it on all four sides; no `Surface` recipe exists for it. `None` leaves egui's `6` |
| `layout.window_margin` | `Frame::inner_margin` «Surface::CentralPanel» (`CentralPanel::frame`, `panel.rs:1206`) | SCOPED | the app content area is a `CentralPanel`, whose stock frame hardcodes `8` (`frame.rs:192`); `Surface::CentralPanel` (§4.4) is the frame that replaces it, so it carries this margin, and `None` keeps egui's `8`. `Spacing::window_margin` is a false friend: it feeds only the *floating* `Window` |
| `layout.section_gap` | `Ui::add_space` (`ui.rs:1675`), per call | DERIVED | egui has one general gap among `Spacing`'s 21 fields; sections are spaced with `Ui::add_space`, which renders this value exactly, read from [`ThemeAtlas::layout`] (§4.2). `None` leaves the gap to the application |

### 5.2 Surfaces — window, dialog, popover, card, tooltip, menu

Five of these six are `Area`-based, so `native_scope` does not reach them
(§1.5). Their carriers are `surface_frame(..)` and `role_modifier(..)`. In the
dialog, popover, tooltip and card tables a bare `Frame::…` sink is that
widget's own surface frame: `Surface::Dialog`, `Surface::Popover`,
`Surface::Tooltip`, `Surface::Card`.

`ResolvedWindowTheme` — `native-theme/src/model/widgets/mod.rs:15-32`,
`#[theme_inherit(border_kind = "full_lg", …)]` at `:14`, so
`border.corner_radius` already carries the **large** radius
(`docs/inheritance-rules.toml:97`, applied at
`native-theme/src/resolve/inheritance.rs:208`).

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `window.background_color` | `visuals.window_fill` (`frame.rs:201`) + `panel_fill` (`:188`, `:192`) «Surface::Window» | SCOPED | displaced on `window_fill` by its base owner `menu.background_color` and on `panel_fill` by `defaults.background_color` (§5.9); `Surface::Window` (`Window::frame`) carries it |
| `window.title_bar_background` | `widgets.open.weak_bg_fill` (`window.rs:1427`) | DIRECT | base owner of `open.weak_bg_fill` (§5.9), written on the base style only; the title bar reads it from the **Context** style (§1.5), so no scope is involved. Applied only when the window is the topmost layer (`window.rs:660`, `:1426-1428`) |
| `window.inactive_title_bar_background` | `Frame::fill` on `Window::title_frame` (`window.rs:272`) | SCOPED | no global field exists; requires `Surface::WindowTitleBar`. Without a `title_frame` the title bar takes the window's own frame (`window.rs:631`), so an inactive title bar shows that frame's fill — `window.background_color` under `Surface::Window`, the base `window_fill` (§5.9) without it |
| `window.inactive_title_bar_text_color` | `RichText::color` on the title atoms (`widget_text.rs:320`), per call | DERIVED | `title_ui` never calls `AtomLayout::fallback_text_color`, so without a per-call colour the title is `visuals.text_color()` (`atom_layout.rs:300-301`) in both states. `window_title_bar_text_color(t, false)` (§4.7) returns this leaf, and the application picks the state with the comparison `Window::show` itself makes — `Some(area_layer_id) == ctx.top_layer_id()` (`window.rs:660`; `Context::top_layer_id`, `context.rs:3127-3129`), the window's layer being `LayerId::new(Order::Middle, id)` (`area.rs:182-184`, order `Middle` by default at `:143`) — before it builds the title, so the colour follows the same state the title-bar fill follows (`window.rs:1426-1428`) |
| `window.title_bar_font.family` | — | UNMAPPABLE `egui-limited` | every `FontId` this crate installs or hands out names `Proportional`, which holds `defaults.font`'s face (§8.1, §8.2), so the text renders in `defaults.font.family`: correct whenever this leaf equals it, as on every bundled preset (§5.8 item 7). egui could carry a distinct family through a `TextStyle` key naming a `FontFamily::Name`; declined, §5.8 item 7 |
| `window.title_bar_font.size` | `RichText::font` on the title atoms | DERIVED | `Window::new` takes `impl IntoAtoms` (`window.rs:102`) and an explicit `RichText::font` overrides `AtomLayout::fallback_font(TextStyle::Heading)` (`window.rs:1350`, `atomics/atom_layout.rs:147`, `atomics/atom_kind.rs:135`, resolved at `widget_text.rs:425-436`) — the per-call route §5.1 grades DERIVED for `text_scale.dialog_title.size`. The `FontId` is `window_title_bar_font(t, prefs)` (§4.7), already scaled; `text_role_font()` does not return it, because `TextRole` has only the four `text_scale` roles. The leaf loses the global `text_styles[Heading]` contest to `text_scale.section_heading` (§5.9); × text-scaling factor (§4) |
| `window.title_bar_font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `window.title_bar_font.weight` | `role_font_weight(t, Role::Window)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). §5.8 item 2 |
| `window.title_bar_font.style` | `role_font_is_italic(t, Role::Window)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). §5.8 item 2 |
| `window.title_bar_font.color` | `RichText::color` on the title atoms (`widget_text.rs:320`), per call | DERIVED | `FontId` carries no colour and `title_ui` sets no `fallback_text_color`, so the resting title is `visuals.text_color()` (`atom_layout.rs:300-301`); a colour in the title text's own format outranks that fallback, the same per-call route as `.size`. `window_title_bar_text_color(t, true)` (§4.7) returns it for the active window — the state test is `window.inactive_title_bar_text_color`'s |
| `window.border.color` | `visuals.window_stroke.color` (`frame.rs:202`) «Surface::Window» | SCOPED | displaced on the global by its base owner `defaults.border.color` (§5.9); `Surface::Window` carries it |
| `window.border.corner_radius` | `visuals.window_corner_radius` (`frame.rs:199`) | DIRECT | base owner of that field (§5.9); only consumers are `Frame::window` and the resize corner (`window.rs:1230`). `defaults.border.corner_radius_lg` owns `menu_corner_radius` instead, so the two do not collide |
| `window.border.line_width` | `visuals.window_stroke.width` «Surface::Window» | SCOPED | as `border.color` |
| `window.border.shadow_enabled` | `visuals.window_shadow` (`frame.rs:200`) | DERIVED | §6.14; offset/blur/spread un-driven |
| `window.border.padding.top` | `spacing.window_margin.top` (`frame.rs:198`) | DIRECT | one `Margin` side, `i8` narrowing (§7); a `None` side keeps egui's `Margin::same(6)` side (`style.rs:1456`). `Window` transplants the whole margin onto `ScrollArea::content_margin` (`window.rs:634-635`, `:740`) |
| `window.border.padding.right` | `spacing.window_margin.right` | DIRECT | as `.top` |
| `window.border.padding.bottom` | `spacing.window_margin.bottom` | DIRECT | as `.top` |
| `window.border.padding.left` | `spacing.window_margin.left` | DIRECT | as `.top` |

`ResolvedDialogTheme` — `native-theme/src/model/widgets/mod.rs:652-690`;
egui counterpart `Modal` (`containers/modal.rs:22`), frame
`Frame::popup(ui.style())` (`modal.rs:100`).

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `dialog.background_color` | `Frame::fill` via `Modal::frame` (`modal.rs:53`); global `window_fill` (`frame.rs:219`) | SCOPED | `menu.background_color` owns the global (§5.9); the modal backdrop is the hardcoded `from_black_alpha(100)` (`modal.rs:29`), for which native-theme has no field |
| `dialog.min_width` | `Ui::set_min_width` (`ui.rs:739`) in the `Modal::show` closure, per call | DERIVED | `Area` has no `min_size`; only `default_size` (`containers/area.rs:256`), `default_width` (`:263`), `default_height` (`:270`). The modal's frame is laid round its content `Ui` (`modal.rs:104-108`); the application reads `dialog.min_width` from `ResolvedTheme`, and that less the frame's margins and stroke, floored at `0.0` and passed only when finite, is exact: `Ui::set_min_width` debug-asserts `0.0 <= width` (`ui.rs:739-743`), which a negative or `NaN` width fails |
| `dialog.max_width` | `Ui::set_max_width` (`ui.rs:718`) in the `Modal::show` closure, per call | DERIVED | `spacing.default_area_size` is **not** a sink: `Area` reads it from `ctx.global_style()` (`area.rs:470-476`), so no scope reaches it, and globally it sizes every free `Area` — §5.8 item 6. The application reads `dialog.max_width` from `ResolvedTheme` and passes it less the frame's margins and stroke, floored at `0.0` and only when finite: `Ui::set_max_width` reaches `Layout::next_frame_ignore_wrap` (`placer.rs:233-234`, `layout.rs:655-660`), which debug-asserts a size of at least `0.0` (`layout.rs:584-587`), which a negative or `NaN` width fails |
| `dialog.min_height` | `Ui::set_min_size` (`ui.rs:732`) in the `Modal::show` closure, per call | DERIVED | as `min_width` |
| `dialog.max_height` | `Ui::set_max_size` (`ui.rs:711`) in the `Modal::show` closure, per call | DERIVED | as `max_width` |
| `dialog.button_gap` | `spacing.item_spacing.x` (`style.rs:392`) «Dialog» | SCOPED | modal bodies are ordinary child `Ui`s (`modal.rs:104-108`), so `ui.style_mut()` works there |
| `dialog.icon_size` | `Image::fit_to_exact_size` (`widgets/image.rs:177`), per call | DERIVED | `Spacing::icon_width` is checkbox/radio/arrow geometry; the application reads `dialog.icon_size` from `ResolvedTheme` into the dialog's `Image`, which renders it exactly |
| `dialog.button_order` | `dialog_button_order(t)` (§4.7) + the order the application adds the buttons in the `Modal::show` closure (for `PrimaryRight`, e.g. `Ui::with_layout(Layout::right_to_left(..))`, `ui.rs:2470`, `layout.rs:156`, primary first), per call | DERIVED | `Modal` (`modal.rs:22`) lays out no buttons of its own; the stock `Button`s show the platform's order exactly as they are added |
| `dialog.title_font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `dialog.title_font.size` | `text_styles[Heading].size` «Dialog» | SCOPED | `Surface::Dialog` resolves to an `egui::Frame`, whose six fields carry no text style (`containers/frame.rs:96-141`); the modal title is drawn by the application inside the `Modal::show` closure (`modal.rs:104-108`), the same carrier as `dialog.button_gap` above. Contested by `window.title_bar_font`, `text_scale.section_heading`; × text-scaling factor (§4) |
| `dialog.title_font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `dialog.title_font.weight` | `t.dialog.title_font.weight` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | as the other `*.font.weight` rows, except that the application reads the leaf from `ResolvedTheme`: `role_font_weight(t, Role::Dialog)` (§4.7) answers for `dialog.body_font`, because a `Role` names one font per widget and this widget has two. §5.8 item 2 |
| `dialog.title_font.style` | `t.dialog.title_font.style` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | as `.weight`, and as the other `*.font.style` rows for the shear. §5.8 item 2 |
| `dialog.title_font.color` | `RichText::color` on the title (`widget_text.rs:320`), per call | DERIVED | the modal's one `Style` text-colour slot is taken by `body_font.color`, but the title is application text inside the `Modal::show` closure (`modal.rs:104-108`), so a colour in its own format carries it exactly |
| `dialog.body_font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `dialog.body_font.size` | `text_styles[Body].size` «Dialog» | SCOPED | contested by `popover.font`, `tooltip.font`, `defaults.font`; × text-scaling factor (§4) |
| `dialog.body_font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `dialog.body_font.weight` | `role_font_weight(t, Role::Dialog)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). §5.8 item 2 |
| `dialog.body_font.style` | `role_font_is_italic(t, Role::Dialog)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). §5.8 item 2 |
| `dialog.body_font.color` | `widgets.noninteractive.fg_stroke.color` «Dialog» | SCOPED | via `Visuals::text_color()` (`style.rs:1136-1139`) |
| `dialog.border.color` | `Frame::stroke.color`; global `window_stroke.color` (`frame.rs:220`) | SCOPED | `defaults.border.color` owns the global (§5.9) |
| `dialog.border.corner_radius` | `Frame::corner_radius`; global `menu_corner_radius` (`frame.rs:217`) | SCOPED | `full_lg`, so this is already the large radius |
| `dialog.border.line_width` | `Frame::stroke.width` | SCOPED | same claimants |
| `dialog.border.shadow_enabled` | `Frame::shadow`; global `popup_shadow` (`frame.rs:218`) | DERIVED | §6.14 |
| `dialog.border.padding.top` | `Frame::inner_margin.top`; global `menu_margin.top` (`frame.rs:216`) | SCOPED | one `Margin` side; a `None` side keeps egui's default `menu_margin` side, `Margin::same(6)` (`style.rs:1457`), which the base style keeps too (§5.9) |
| `dialog.border.padding.right` | `Frame::inner_margin.right`; global `menu_margin.right` | SCOPED | as `.top` |
| `dialog.border.padding.bottom` | `Frame::inner_margin.bottom`; global `menu_margin.bottom` | SCOPED | as `.top` |
| `dialog.border.padding.left` | `Frame::inner_margin.left`; global `menu_margin.left` | SCOPED | as `.top` |

`ResolvedPopoverTheme` — `native-theme/src/model/widgets/mod.rs:548-557`;
egui counterpart `Popup`, carriers `Popup::frame` (`containers/popup.rs:369`) and
`Popup::style` (`:417`, applied at `:602` **before** the frame is built at
`:603`, so a modifier does change that popup's own frame).

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `popover.background_color` | `Frame::fill`; global `window_fill` (`frame.rs:219`) | SCOPED | `menu.background_color` owns the global (§5.9) |
| `popover.font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `popover.font.size` | `text_styles[Body].size` «Popover» | SCOPED | contested by `dialog.body_font`, `tooltip.font`, `defaults.font`; × text-scaling factor (§4) |
| `popover.font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `popover.font.weight` | `role_font_weight(t, Role::Popover)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). §5.8 item 2 |
| `popover.font.style` | `role_font_is_italic(t, Role::Popover)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). §5.8 item 2 |
| `popover.font.color` | `widgets.noninteractive.fg_stroke.color` «Popover» | SCOPED | carrier `Popup::style` |
| `popover.border.color` | `Frame::stroke.color`; global `window_stroke.color` | SCOPED | `defaults.border.color` owns the global |
| `popover.border.corner_radius` | `Frame::corner_radius`; global `menu_corner_radius` | SCOPED | `full_lg` |
| `popover.border.line_width` | `Frame::stroke.width` | SCOPED | same claimants |
| `popover.border.shadow_enabled` | `Frame::shadow`; global `popup_shadow` | DERIVED | §6.14 |
| `popover.border.padding.top` | `Frame::inner_margin.top`; global `menu_margin.top` | SCOPED | as `dialog.border.padding.top` |
| `popover.border.padding.right` | `Frame::inner_margin.right`; global `menu_margin.right` | SCOPED | as `.top` |
| `popover.border.padding.bottom` | `Frame::inner_margin.bottom`; global `menu_margin.bottom` | SCOPED | as `.top` |
| `popover.border.padding.left` | `Frame::inner_margin.left`; global `menu_margin.left` | SCOPED | as `.top` |

`ResolvedCardTheme` — `native-theme/src/model/widgets/mod.rs:812-819`,
`#[theme_layer(border_kind = "none")]` at `:811` and **no `#[theme_inherit]`**.
It is the one member of this group that is not `Area`-based (`Ui::group` →
`Frame::group`, `ui.rs:2147-2149`). It is also the one widget missing from the
`check_ranges` dispatch list (`native-theme/src/resolve/validate.rs:168-191`),
so **every numeric card leaf must be treated as possibly zero, negative or
non-finite**.

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `card.background_color` | `Frame::fill` on a connector `Frame` passed to `Frame::show` (`frame.rs:404`) | SCOPED | `Frame::group` never calls `.fill(..)` (`frame.rs:178-183`); `Frame::NONE.fill` is `TRANSPARENT` (`:164`) |
| `card.border.color` | `Frame::stroke.color`; global `widgets.noninteractive.bg_stroke.color` (`frame.rs:182`) | SCOPED | heavily contested hairline |
| `card.border.corner_radius` | `Frame::corner_radius`; global `widgets.noninteractive.corner_radius` (`frame.rs:181`) | SCOPED | also `Frame::canvas` (`:230`) |
| `card.border.line_width` | `Frame::stroke.width`; global `noninteractive.bg_stroke.width` | SCOPED | same claimants |
| `card.border.shadow_enabled` | — | UNMAPPABLE `source-side gap` | `Frame.shadow` exists (`frame.rs:140`, `:303`) but `ResolvedWidgetBorder` supplies no offset/blur/spread, and `Frame::group` has no shadow of egui's own whose geometry §6.14 could keep: its `..base` is `Shadow::NONE` here (`frame.rs:178-183`, `:167`), so a gated `true` would paint nothing. As `sidebar` (§5.6) |
| `card.border.padding.top` | `Frame::inner_margin.top` | SCOPED | `Frame::group` hardcodes `6` (`frame.rs:180`), which a `None` side keeps |
| `card.border.padding.right` | `Frame::inner_margin.right` | SCOPED | as `.top` |
| `card.border.padding.bottom` | `Frame::inner_margin.bottom` | SCOPED | as `.top` |
| `card.border.padding.left` | `Frame::inner_margin.left` | SCOPED | as `.top` |

`ResolvedTooltipTheme` — `native-theme/src/model/widgets/mod.rs:243-257`,
`border_kind = "full"` at `:242` (the ordinary radius, not the large one).
**Every SCOPED row below is reachable only on the manual `Tooltip` path**:
`Response::on_hover_text` is `Tooltip::for_enabled(&self).show(..)`
(`response.rs:665-667`, `:727-735`) and `Tooltip::for_widget`
(`containers/tooltip.rs:39-50`) never calls `.frame(..)` or `.style(..)`, so it
falls through to the global style. `Tooltip::popup` is a public field
(`tooltip.rs:9`), which is the escape hatch.

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `tooltip.background_color` | `Frame::fill` via `Tooltip::popup.frame(..)`; global `window_fill` | SCOPED | `menu.background_color` owns the global (§5.9) |
| `tooltip.max_width` | `spacing.tooltip_width` (`style.rs:448`) | DIRECT | one of three uncontested fields in the surfaces group, with `visuals.window_corner_radius` and `spacing.window_margin`, both owned by `window` (§5.9); readers `tooltip.rs:26`, `:43`, `response.rs:714`, `:731`, `:754` |
| `tooltip.font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `tooltip.font.size` | `text_styles[Body].size` «Tooltip» | SCOPED | tooltip content is a plain `Label` (`response.rs:733`, and `:716` at the pointer); × text-scaling factor (§4) |
| `tooltip.font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `tooltip.font.weight` | `role_font_weight(t, Role::Tooltip)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). §5.8 item 2 |
| `tooltip.font.style` | `role_font_is_italic(t, Role::Tooltip)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). §5.8 item 2 |
| `tooltip.font.color` | `widgets.noninteractive.fg_stroke.color` «Tooltip» | SCOPED | contested by `dialog.body_font.color`, `popover.font.color`, `defaults.text_color` |
| `tooltip.border.color` | `Frame::stroke.color`; global `window_stroke.color` | SCOPED | `defaults.border.color` owns the global |
| `tooltip.border.corner_radius` | `Frame::corner_radius`; global `menu_corner_radius` | SCOPED | ordinary radius |
| `tooltip.border.line_width` | `Frame::stroke.width` | SCOPED | same claimants |
| `tooltip.border.shadow_enabled` | `Frame::shadow`; global `popup_shadow` | DERIVED | §6.14 |
| `tooltip.border.padding.top` | `Frame::inner_margin.top`; global `menu_margin.top` | SCOPED | as `dialog.border.padding.top`. The 4.0 pt anchor gap is hardcoded (`tooltip.rs:30`, `:42`); native-theme has no anchor gap, so nothing is lost there |
| `tooltip.border.padding.right` | `Frame::inner_margin.right`; global `menu_margin.right` | SCOPED | as `.top` |
| `tooltip.border.padding.bottom` | `Frame::inner_margin.bottom`; global `menu_margin.bottom` | SCOPED | as `.top` |
| `tooltip.border.padding.left` | `Frame::inner_margin.left`; global `menu_margin.left` | SCOPED | as `.top` |

`ResolvedMenuTheme` — `native-theme/src/model/widgets/mod.rs:198-234`,
`border_kind = "none"` at `:196` so an omitted `[menu.border]` legitimately
resolves to the placeholder border — transparent colour, zero radius and width,
no shadow, every padding side `None` (`validate_helpers.rs:44-56`, `:279`). **Menu items are
`Button`s** (`MenuButton::ui` is `self.button.ui(ui)`, `menu.rs:322`).
`MenuConfig::default()` hardcodes `style: menu_style.into()` (`menu.rs:83`) and
`MenuBar::default()` the same (`:226`), so without an application-supplied
`MenuConfig::style` / `MenuBar::style`, `menu_style` (`menu.rs:22-29`) discards
six of the connector's writes inside every menu (`:23`–`:28`). The same is true of
`Response::context_menu`, which is `Popup::context_menu(self).show(..)`
(`response.rs:1028-1030`) and takes no style argument at all: it delegates to
`Popup::menu` (`containers/popup.rs:237`), which hardcodes `.style(menu_style)` (`:241`), so
`role_modifier(theme, Role::Menu, RoleVariant::Normal)` has **no supply point** on that
one-line spelling. The
escape hatches are `Popup::context_menu(&response).style(..).show(..)`
(`containers/popup.rs:248`, `:417`, `:508`) and
`MenuButton::from_button(..).config(MenuConfig::new().style(..))` (`menu.rs:309`,
`:302`, `:92`, `:107`).

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `menu.background_color` | `visuals.window_fill` of the base style (base owner, §5.9), which the «Menu» cell inherits (§3.4) | DIRECT | every menu, context menu and `ComboBox` list is a `Frame::menu` or `Frame::popup` (`frame.rs:210`, `:219`) built from its `Ui`'s style — the base style, or a role cell that inherits the base write — so it renders with no scope; `Response::context_menu` offers none (§5.9) |
| `menu.separator_color` | `widgets.noninteractive.bg_stroke.color` (`widget_style.rs:213-217` → `separator.rs:109`) «Menu» | SCOPED | `separator_style` ignores `_classes` (`widget_style.rs:212`) and hardcodes `spacing: 6.0` (`:215`) |
| `menu.row_height` | `spacing.interact_size.y` (`style.rs:409`) «Menu» | SCOPED | contested (§5.11); floor via `widgets/button.rs:308`, exact bar row via `menu.rs:274-275`. `Option<f32>` (S2): `None` writes nothing, and egui's `interact_size.y` stands |
| `menu.icon_text_gap` | `spacing.icon_spacing` (`style.rs:436`) «Menu» | SCOPED | this is the default gap between **all** atoms (`atom_layout.rs:302`) |
| `menu.icon_size` | `Image::fit_to_exact_size` (`widgets/image.rs:177`) on the item's image atom, per call | DERIVED | no menu-item icon size in `Style`; the application reads `menu.icon_size` from `ResolvedTheme` into the `Image` atom of the item's `Button`, which renders it exactly on an item built with `Button::new` (for a menu, `ui.menu_button((image, text), ..)`, `ui.rs:2788`, which builds one through `MenuButton::new` or `SubMenuButton::new`, `containers/menu.rs:296-297`, `:346-347`); `Button::image`/`image_and_text` and the `menu_image_*` helpers clamp the atom to the Body row height (`widgets/button.rs:113`, `:311-313`; §9.4). **Not** `icons::icon_size`, which reads `defaults.icon_sizes` |
| `menu.hover_background` | `widgets.{hovered,active,open}.weak_bg_fill` (`style.rs:1300`) «Menu» | SCOPED | via `button_style` (`widget_style.rs:159`). `open` too: while an item's submenu is open, `SubMenuButton::ui` paints that item with the `open` entry in place of `inactive` (`menu.rs:382-384`), and the platform keeps it highlighted as on hover — WinUI 3 gives `MenuFlyoutSubItemBackgroundSubMenuOpened` the brush of `MenuFlyoutSubItemBackgroundPointerOver` in its `Default` and `Light` dictionaries (microsoft-ui-xaml `258a2e9b`, `controls/dev/CommonStyles/MenuFlyout_themeresources.xaml` lines 16/18 and 182/184, applied by the `SubMenuOpened` state at lines 662-667; the `HighContrast` dictionary differs, lines 99/101, and the model has no open-submenu leaf to carry that). So the `Role::Menu` scope copies `hovered` into `open`. A row highlight, so emitted as given (C17): the resting item is invisible against the menu's own frame — its fill is `TRANSPARENT` (`menu.rs:27`), set by the `menu_style` the «Menu» cell applies (§3.4) — and egui paints the hover fill over that frame |
| `menu.hover_text_color` | `widgets.{hovered,active,open}.fg_stroke.color` (`style.rs:1311`) «Menu» | SCOPED | shared with check-mark and arrow strokes; no text-only slot. `open` as for `menu.hover_background`: WinUI 3's `MenuFlyoutSubItemForegroundSubMenuOpened` equals `…ForegroundPointerOver` (same file, lines 22/24 and 188/190) |
| `menu.disabled_text_color` | `widgets.{noninteractive,inactive}.fg_stroke.color` «Menu, Disabled» | SCOPED | a disabled item is a `Button` in `WidgetState::Inactive` (§6.3), so the `Disabled` cell carries the platform's hue, and a plain label in the menu takes it from `noninteractive`; `Ui::disable`'s fade alone cannot reach it |
| `menu.font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `menu.font.size` | `text_styles[Body].size` «Menu» | SCOPED | menu items are `Button`s, which read `override_font_id` or else `TextStyle::Body`, never `TextStyle::Button` (B3); × text-scaling factor (§4) |
| `menu.font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `menu.font.weight` | `role_font_weight(t, Role::Menu)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). §5.8 item 2 |
| `menu.font.style` | `role_font_is_italic(t, Role::Menu)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). §5.8 item 2 |
| `menu.font.color` | `widgets.{noninteractive,inactive}.fg_stroke.color` «Menu» | SCOPED | items read `inactive`, a plain label in the menu `noninteractive` (§6.1); survives `menu_style`, which touches only `weak_bg_fill` and `bg_stroke` (`menu.rs:27-28`) |
| `menu.border.color` | `widgets.{inactive,hovered,active,open}.bg_stroke.color` «Menu» | SCOPED | the menu *item's* outline: `docs/platform-facts.md:1237` sends the popup's border to §2.16, and `docs/inheritance-rules.toml:71` gives `menu` no border inheritance for that reason. Written over `menu_style`'s `Stroke::NONE` (`menu.rs:24-28`) and read by every item, a `Button` (`widget_style.rs:160`); `border_kind = "none"` → `TRANSPARENT` when the preset omits it (`validate_helpers.rs:285`, or `:44-56` for an omitted `[menu.border]`), as every bundled preset does, which is `menu_style`'s look. The menu's frame keeps the base style's `window_stroke`, `defaults.border.color`'s (§5.9) |
| `menu.border.corner_radius` | `widgets.{inactive,hovered,active,open}.corner_radius` «Menu» | SCOPED | items are rectangular (`docs/platform-facts.md:1239`); `border_kind = "none"` → `0.0` when the preset omits it (`validate_helpers.rs:286`). `menu_style` leaves the radius alone (`menu.rs:22-29`), so without this write every item would take `button.border.corner_radius` from the base style. The menu's frame keeps the base style's `menu_corner_radius`, `defaults.border.corner_radius_lg`'s (§5.9) |
| `menu.border.line_width` | `widgets.{inactive,hovered,active,open}.bg_stroke.width` «Menu» | SCOPED | as `menu.border.color`; `0.0` when omitted (`validate_helpers.rs:287`), `menu_style`'s `Stroke::NONE` width. It also feeds the item padding below (§5 intro) |
| `menu.border.shadow_enabled` | — | UNMAPPABLE `source-side gap` | an item is a `Button`, whose `ButtonStyle.frame` is `..Default::default()` (`widget_style.rs:166`), so the stock item casts none, and the platform's menu shadow is the popup's (`docs/platform-facts.md:1240`, its §2.16), which the base style's `popup_shadow` gate carries from `defaults.border.shadow_enabled` (§5.9); as `button.border.shadow_enabled`, a `Frame` the application lays round an item paints a shadow under it (`frame.rs:140`, `:303`): what is missing is the geometry (§14 item 10) |
| `menu.border.padding.top` | `spacing.button_padding.y` «Menu» | DERIVED | as `button.border.padding.top`, with `menu.border.line_width`; applied after `menu_style`, whose `vec2(2.0, 0.0)` (`egui/src/containers/menu.rs:23`) is the unstated side's target. The item padding of `docs/platform-facts.md:1235-1236` — NSMenuItem 12 / 3, WinUI 3's `MenuFlyoutItem`, KDE's `MenuItem_Margin*`, GNOME's `$menu_padding` — which the four platform presets and their `-live` variants state under `[*.menu.border]`; the other presets state none, so their items keep `menu_style`'s padding |
| `menu.border.padding.right` | `spacing.button_padding.x` «Menu» | DERIVED | as `button.border.padding.right`, with `menu.border.line_width`; the unstated side's target is `menu_style`'s `2.0` |
| `menu.border.padding.bottom` | `spacing.button_padding.y` «Menu» | DERIVED | as `.top` |
| `menu.border.padding.left` | `spacing.button_padding.x` «Menu» | DERIVED | as `.right` |

Only the base owners of global fields — `window`'s and `menu`'s own rows (§5.9)
— and `tooltip.max_width` are DIRECT here; every other surface's chrome is
displaced from those globals, which is why the other surfaces go through
`Surface` frames. The menu's frame is the base style's, and the «Menu» cell
carries its items (§4.4).

### 5.3 Buttons and selection controls

button, link, switch, checkbox, segmented control.

Five structural facts drive every verdict here:

* **B1** — Button fill is `weak_bg_fill` (`widget_style.rs:159`) while checkbox
  fill is `bg_fill` (`:182` → `widgets/checkbox.rs:137`). They are different
  fields, which is a rare piece of good news.
* **B2** — egui has **no disabled visual state**: a disabled widget is painted
  in its `inactive` entry, faded by `disabled_alpha` (§6.3). No `disabled_*`
  colour has a sink on the base style; `RoleVariant::Disabled` carries one
  inside a scope, which is what SCOPED means (§2), so each such row is SCOPED
  on the field its widget actually reads in that state.
* **B3** — `Style::override_font_id` (`style.rs:255`, checked first at
  `:159-162`) is the universal, scope-local font sink. `egui::Button` does
  **not** honour `TextStyle::Button`: `Button::new` sets
  `.fallback_font(TextStyle::Button)` (`widgets/button.rs:49`) and `atom_ui` then
  overwrites it at `:358-360` from `Style::button_style` →
  `Style::widget_style`, which is
  `override_font_id.unwrap_or_else(|| TextStyle::Body.resolve(self))`
  (`widget_style.rs:122`, `:137`). So a `Button` — and every menu item, tab and
  `selectable_label` — reads `override_font_id` or else `TextStyle::Body`,
  never `TextStyle::Button`. `override_font_id` also outranks a text style
  chosen explicitly in `RichText` (`widget_text.rs:426-430`; only an explicit
  size or family, `:431-436`, beats it), so in a scope that writes it
  `ui.heading(..)` renders in that font, not in `TextStyle::Heading`.
* **B4** — `Visuals::override_text_color` (`style.rs:1016`) cleanly separates
  the checkbox *label* from the checkbox *check mark*:
  `Style::checkbox_style` takes the label colour from `ws.text`
  (`widget_style.rs:133-136` → `:187`) and the check-mark stroke from
  `ws.stroke` = `fg_stroke` (`:188`). It is written in the `Role::Checkbox`
  scope **only**, never on the base style. It is also what keeps the label
  readable: `checkbox.indicator_color` inherits `defaults.accent_text_color`,
  the colour of a mark drawn on `checked_background`
  (`docs/inheritance-rules.toml:175`), so without `override_text_color` the
  label would be painted in the mark's colour over the window background — a
  text-on-background pair no platform states (C8).
* **B5** — egui 0.36.2 has **no switch widget and no segmented control**. The
  documented substitutes are one `Button::new(..).selected(checked)`
  (`widgets/button.rs:270`) whose click the application turns into a toggle,
  and a row of `Button::new(..).selected(..)`. Both use `Button::new`, which
  keeps `frame_when_inactive` `true` (`widgets/button.rs:54`), and not
  `Ui::toggle_value` (`ui.rs:1875-1882`) or `Button::selectable`, which set it
  to the selected flag (`widgets/button.rs:78-81`) and so paint nothing at rest
  when unselected (`widgets/button.rs:364-368`) — where a native switch shows its unchecked track and a native
  segmented control its unselected segments.

`ResolvedButtonTheme` — `native-theme/src/model/widgets/mod.rs:41-88`.

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `button.background_color` | `widgets.inactive.weak_bg_fill` of the base style (base owner, §5.9) and `widgets.{inactive,open}.weak_bg_fill` «Button» | DIRECT | `widget_style.rs:159` → `widgets/button.rs:331`, `:365`. The base style's `inactive` entry is `theme.button`'s (§6.1), so every unscoped `Button` renders it; the «Button» scope writes the same value |
| `button.primary_background` | `visuals.selection.bg_fill` «Button» | SCOPED | in the `Normal` cell, which `button_style` applies to a button the application builds with `.selected(true)` (§6.2): `widget_style.rs:150-155` overwrites a selected button's fill and text colour regardless of interaction state (`:159`, `:168`); a write to `widgets.*.weak_bg_fill` would be discarded. `Button::selected` also opts the button into toggle semantics, so assistive technology announces the primary button as a pressed toggle (`widgets/button.rs:263-269`, `response.rs:975-980`) — egui-limited, a §14 ledger entry |
| `button.primary_text_color` | `visuals.selection.stroke.color` «Button» | SCOPED | same branch — `widget_style.rs:154` → `:168`; §6.2 specifies the identical sinks |
| `button.min_width` | `Button::min_size` (`widgets/button.rs:193`), per call | DERIVED | `atom_ui` raises only `min_size.y`, to `interact_size.y` (`widgets/button.rs:307-309`), and `interact_size.x` drives `Grid`, `DragValue` and the colour swatch instead; the builder's `x` is an outer width (`atomics/atom_layout.rs:409`), so `Button::min_size(vec2(min_width, 0.0))` is exact |
| `button.min_height` | `spacing.interact_size.y` of the base style (base owner, §5.9) and «Button» | DIRECT | base owner of a contested field (§5.11). `atom_ui` raises every button's height to it (`widgets/button.rs:307-309`) |
| `button.icon_text_gap` | `spacing.icon_spacing` «Button» | SCOPED | `atom_layout.rs:302`, applied `(n−1)` times at `atom_layout.rs:346-348`, so only a button with an image and a label paints it. Contested (§5.11): the base owner is `checkbox.label_gap` (§5.9), whose widgets always paint the gap |
| `button.disabled_opacity` | `visuals.disabled_alpha` «Button» | SCOPED | contested (§5.11); must pass `unit_interval()` |
| `button.hover_background` | `widgets.hovered.weak_bg_fill` of the base style (base owner, §5.9) and «Button» | DIRECT | contested (§5.11). A state layer (C17): written composited over `button.background_color`, because egui paints one fill per state where the platform layers (§6.1) — the colour the platform paints, so the composite is the route, not a derivation (§2) |
| `button.hover_text_color` | `widgets.hovered.fg_stroke.color` of the base style (base owner, §5.9) and «Button» | DIRECT | see §5.11 |
| `button.active_text_color` | `widgets.active.fg_stroke.color` «Button» | SCOPED | not on the base style: there the field is every `RichText::strong()`'s colour, `widgets.active.text_color()` (`style.rs:1147-1149`, read at `widget_text.rs:485`), and the platform's pressed-button text is its colour on the pressed fill — kde-breeze light's `#fcfcfc` (`native-theme/src/presets/kde-breeze.toml:95`) on the `#eff0f1` panel would make strong text unreadable. The base style writes `defaults.text_color` there (§5.1); the Button cell keeps the platform's exact pressed pair |
| `button.disabled_text_color` | `widgets.{noninteractive,inactive}.fg_stroke.color` and `visuals.selection.stroke.color` «Button, Disabled» | SCOPED | B2: a disabled button lands in `WidgetState::Inactive`, and the `Disabled` cell carries the colour there, and to the plain labels of the scope in `noninteractive` (§6.3); a selected (primary) button takes its text colour from `selection.stroke.color` in every state (`widget_style.rs:153-154`), so the cell writes it there too (§6.2) |
| `button.active_background` | `widgets.active.weak_bg_fill` of the base style (base owner, §5.9) and «Button» | DIRECT | its only claimant. `soft_option`; `None` → `hover_background` (§6.4). A state layer like `hover_background`: composited over `button.background_color` (C17) |
| `button.disabled_background` | `widgets.inactive.weak_bg_fill` and `visuals.selection.bg_fill` «Button, Disabled» | SCOPED | B2; as `disabled_text_color`: a selected (primary) button fills from `selection.bg_fill` in every state (`widget_style.rs:150-152`) |
| `button.font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `button.font.size` | `override_font_id.size` «Button» **and** `text_styles[Button].size` (base owner, §5.9) | SCOPED | B3; two sinks, not one: the base write exists because `ComboBox`, `ProgressBar`, `CollapsingHeader` and `Style::drag_value_text_style` read `TextStyle::Button` even though `Button` itself does not (§8.5); × text-scaling factor (§4). SCOPED although it is the base owner of `text_styles[Button]`: that global write renders on `ComboBox`, `ProgressBar`, `CollapsingHeader` and `DragValue`, but `Button` itself reads `override_font_id` or else `TextStyle::Body` (B3), and only the «Button» scope writes `override_font_id` — no one global write renders it on its own widget (§2) |
| `button.font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `button.font.weight` | `role_font_weight(t, Role::Button)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). §5.8 item 2 |
| `button.font.style` | `role_font_is_italic(t, Role::Button)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). §5.8 item 2 |
| `button.font.color` | `widgets.{inactive,open}.fg_stroke.color` of the base style (base owner, §5.9) and `widgets.{noninteractive,inactive,open}.fg_stroke.color` «Button» | DIRECT | `widget_style.rs:133-136` → `:168` → `widgets/button.rs:361` |
| `button.border.color` | `widgets.{5}.bg_stroke.color` «Button» **and** `widgets.{inactive,hovered,active,open}.bg_stroke.color` of the base style (base owner, §5.9) | DIRECT | stock egui has `inactive.bg_stroke = Stroke::NONE` (`style.rs:1694`); on the base style it is what every unscoped control draws its outline with (§6.1) |
| `button.border.corner_radius` | `widgets.{5}.corner_radius` «Button» **and** `widgets.{inactive,hovered,active,open}.corner_radius` of the base style (base owner, §5.9) | DIRECT | `widget_style.rs:161`; `u8` narrowing (§7); flattens egui's `hovered = same(3)` vs `same(2)` (`style.rs:1704`). The base style's `widgets.active.corner_radius` is also the focus ring's default radius (§6.18) |
| `button.border.line_width` | `widgets.{5}.bg_stroke.width` «Button» **and** `widgets.{inactive,hovered,active,open}.bg_stroke.width` of the base style (base owner, §5.9) | DIRECT | feeds back into layout: inner margin is `button_padding + expansion − bg_stroke.width` (`widget_style.rs:163-165`), so the stroke lies inside `button_padding`, and the padding rows add this width to what they write (§5 intro, `convert::padding_with_border`); the manifest therefore also lists `spacing.button_padding` of the base style and «Button» among its sinks, moved wherever a side of the pair is stated (§13.1) |
| `button.border.shadow_enabled` | — | UNMAPPABLE `source-side gap` | `ButtonStyle.frame` is `..Default::default()` (`widget_style.rs:166`), so the stock button casts none; as `list.border.shadow_enabled`, a `Frame` the application lays round the widget paints a shadow under it (`frame.rs:140`, `:303`): what is missing is the geometry (§14 item 10) |
| `button.border.padding.top` | `spacing.button_padding.y` of the base style (base owner, §5.9) and «Button» | DERIVED | one `Vec2` component shared with `.bottom`, under the pair rule above: the mean of the two targets, a stated side counting with `button.border.line_width` added and a side the theme leaves unstated as egui's `1.0` (`style.rs:1458`) — with both sides stated, `(top + bottom) / 2 + line_width`. Lossy only in centring, by `\|top − bottom\| / 2`. `i8` narrowing; `Button::small()` zeroes both (`widgets/button.rs:339-342`) |
| `button.border.padding.right` | `spacing.button_padding.x` of the base style (base owner, §5.9) and «Button» | DERIVED | one component shared with `.left`, under the pair rule above: the mean of the two targets, a stated side counting with `button.border.line_width` added and a side the theme leaves unstated as egui's `4.0` (`style.rs:1458`) — with both sides stated, `(left + right) / 2 + line_width`. Lossy only in centring, by `\|left − right\| / 2`. Never halved |
| `button.border.padding.bottom` | `spacing.button_padding.y` of the base style (base owner, §5.9) and «Button» | DERIVED | as `.top` |
| `button.border.padding.left` | `spacing.button_padding.x` of the base style (base owner, §5.9) and «Button» | DERIVED | as `.right` |

`ResolvedLinkTheme` — `native-theme/src/model/widgets/mod.rs:858-878`. egui's
`Link` (`widgets/hyperlink.rs:27-74`) reads exactly three style values:
`visuals.hyperlink_color` (`:47`), `visuals.fg_stroke.width` (`:51`) and
`interaction.selectable_labels` (`:56`).

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `link.visited_text_color` | `RichText::color` (`widget_text.rs:320`) on the link text, per call | DERIVED | egui records no visit — `Hyperlink` opens its URL on a click and keeps nothing (`hyperlink.rs:132-142`) — so the application keeps the URLs it has opened and gives a link among them this leaf, read from `ResolvedTheme`. `Link` hands `hyperlink_color` (`hyperlink.rs:47`) to its `TextShape` only as the fallback colour (`:58-63`), which replaces nothing but `Color32::PLACEHOLDER` (`epaint/src/tessellator.rs:2080-2081`), the colour of text with none of its own (`widget_text.rs:423`): an explicit colour in the text's own format "takes precedence over this fallback color" (`epaint/src/shapes/text_shape.rs:47`). The companion widgets crate's `wrap::hyperlink` keeps such a set (`docs/todo_egui-widgets-spec.md` §4.5) |
| `link.underline_enabled` | `RichText::underline` (`widget_text.rs:268`) on the link text, per call | DERIVED | `Link::ui` adds its own underline only on hover or focus (`hyperlink.rs:50-54`), but an underline in the text's own format is laid into the galley (`epaint/src/text/text_layout.rs:1079`) and painted at rest. The application reads `link.underline_enabled` from `ResolvedTheme`; the text also takes its colour per call (`link.font.color`, or the state leaf), since the underline is drawn in the text's own colour, else in `visuals.text_color()` (`widget_text.rs:422`, `:446-448`), not in `hyperlink_color` |
| `link.background_color` | `RichText::background_color` (`widget_text.rs:310`) on the link text, per call | DERIVED | `Link::ui` paints only a `TextShape` (`hyperlink.rs:62-64`) — no fill, no `Frame` — but a background in the text's own format is painted behind the glyphs (`epaint/src/text/text_layout.rs:1070-1071`): the colour is exact, the extent is the text's, since native-theme states no link padding |
| `link.hover_background` | `RichText::background_color` (`widget_text.rs:310`) on the link text while hovered, per call | DERIVED | as `link.background_color`, chosen by the hover state `link.hover_text_color` reads. A state layer (C17): composited over `link.background_color` with `convert::composite_over` (§4.8) |
| `link.hover_text_color` | `RichText::color` (`widget_text.rs:320`) on the link text while hovered, per call | DERIVED | the colour read at `hyperlink.rs:47` is unconditional, but it is only the `TextShape`'s fallback, which an explicit text colour takes precedence over (`link.visited_text_color`). The application reads the link's state before adding it — `Context::read_response` of the id `ui.next_auto_id()` gives (`context.rs:1350-1355`, `ui.rs:889`), the idiom egui's own `Checkbox` uses (`widgets/checkbox.rs:72-73`), and `Response::hovered` (`response.rs:320`) — and passes this leaf, read from `ResolvedTheme`, while hovered (§5 introduction). egui's own hover underline stays in `hyperlink_color` (`hyperlink.rs:50-54`) — a link wrapped over rows answers on its first row only: a wrapped `Label` gives each further row a fresh id (`widgets/label.rs:219-227`, `ui.rs:1272-1273`) and `union` keeps the first (`response.rs:1083`) |
| `link.active_text_color` | `RichText::color` (`widget_text.rs:320`) on the link text while pressed, per call | DERIVED | as `link.hover_text_color`, for `Response::is_pointer_button_down_on` (`response.rs:595`) — a link wrapped over rows answers on its first row only: a wrapped `Label` gives each further row a fresh id (`widgets/label.rs:219-227`, `ui.rs:1272-1273`) and `union` keeps the first (`response.rs:1083`) |
| `link.disabled_text_color` | `visuals.hyperlink_color` «Link, Disabled» | SCOPED | `Link::ui` takes its colour from `hyperlink_color` alone (`hyperlink.rs:47`), so that — not `fg_stroke` — is the field the `Disabled` cell must carry it in; B2 |
| `link.font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `link.font.size` | `override_font_id.size` «Link» | SCOPED | B3; × text-scaling factor (§4) |
| `link.font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `link.font.weight` | `role_font_weight(t, Role::Link)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). §5.8 item 2 |
| `link.font.style` | `role_font_is_italic(t, Role::Link)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). §5.8 item 2 |
| `link.font.color` | `visuals.hyperlink_color` (`style.rs:1036`) «Link» | SCOPED | sole reader `hyperlink.rs:47`. The base owner is `defaults.link_color` (§5.9), to which `docs/inheritance-rules.toml:281` pins this leaf unless a theme sets `color` under `[*.link.font]`; no shipped preset does, and the KDE reader writes both from `[Colors:View] ForegroundLink` (`native-theme/src/kde/colors.rs:29`, `:103-105`), so the contest is vacuous today |

`ResolvedSwitchTheme` — `native-theme/src/model/widgets/mod.rs:599-642`.
No font, no border, and the highest `soft_option` count in `ResolvedTheme`.
Per B5 every row is scoped to the `Button::new(..).selected(checked)` substitute.

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `switch.checked_background` | `visuals.selection.bg_fill` «Switch» | SCOPED | `widget_style.rs:151-152` overwrites a selected button's fill |
| `switch.unchecked_background` | `widgets.{inactive,open}.weak_bg_fill` «Switch» | SCOPED | the substitute's resting frame fill (`widget_style.rs:159` → `widgets/button.rs:365`), painted because `Button::new` keeps `frame_when_inactive` (B5) |
| `switch.thumb_background` | — | UNMAPPABLE `widgets-crate` | no egui widget draws a knob inside a track; the companion crate's `Switch` paints it (`docs/todo_egui-widgets-spec.md` §4.1) |
| `switch.track_width` | `Button::min_size` (`widgets/button.rs:193`) on the substitute, per call | DERIVED | `Button::new(..).selected(checked).min_size(..)` carries the width as an outer width (`atomics/atom_layout.rs:409`); the application reads `switch.track_width` from `ResolvedTheme` |
| `switch.track_height` | `spacing.interact_size.y` «Switch» | SCOPED | the substitute button's whole height |
| `switch.thumb_diameter` | — | UNMAPPABLE `widgets-crate` | no knob; the companion crate's `Switch` paints one (`docs/todo_egui-widgets-spec.md` §4.1) |
| `switch.track_radius` | `widgets.{5}.corner_radius` «Switch» | SCOPED | `widget_style.rs:161` |
| `switch.disabled_opacity` | `visuals.disabled_alpha` «Switch» | SCOPED | `unit_interval()` |
| `switch.hover_checked_background` | `Button::fill` (`widgets/button.rs:143`) on the substitute while hovered and checked, per call | DERIVED | `button_style` substitutes `selection.bg_fill` regardless of state (`widget_style.rs:147`, `:150-152`), but a fill handed to the builder is laid on the frame after it (`widgets/button.rs:346-347`). The application reads the substitute's state before adding it — `Context::read_response` of `ui.next_auto_id()` (`context.rs:1355`, `ui.rs:889`), as `Button` does itself (`widgets/button.rs:325-326`) — and passes this leaf only while hovered and checked. `soft_option`: `None` passes no fill, so `switch.checked_background` stands (§6.4). A state layer (C17): composited over `switch.checked_background` with `convert::composite_over` (§4.8). The companion crate's `Switch` paints it too (`docs/todo_egui-widgets-spec.md` §4.1) |
| `switch.hover_unchecked_background` | `widgets.{hovered,active}.weak_bg_fill` «Switch» | SCOPED | `soft_option`; `None` → `unchecked_background`. A state layer (C17, §6.1): composited over `switch.unchecked_background`, the resting track (row above) |
| `switch.disabled_checked_background` | `visuals.selection.bg_fill` «Switch, Disabled» | SCOPED | a disabled checked substitute is a selected `Button` in `WidgetState::Inactive`, whose fill `button_style` takes from `selection.bg_fill` in every state (`widget_style.rs:150-152`), so the `Disabled` cell carries it |
| `switch.disabled_unchecked_background` | `widgets.inactive.weak_bg_fill` «Switch, Disabled» | SCOPED | a disabled unchecked substitute is an unselected `Button` in `WidgetState::Inactive` (§6.3), filled with that state's `weak_bg_fill` (`widget_style.rs:159`); B2 |
| `switch.disabled_thumb_color` | — | UNMAPPABLE `widgets-crate` | no knob; B2. The companion crate's `Switch` paints it (`docs/todo_egui-widgets-spec.md` §4.1) |

Every leaf is a public `ResolvedTheme` field, so an application that paints its
own switch reads the whole struct from `ResolvedTheme`. The five `soft_option`
colours are `Option` there, because a `None` is the platform stating that the
switch has no distinct appearance in that state (§6.4); substituting a colour
would be an invented value.

`ResolvedCheckboxTheme` — `native-theme/src/model/widgets/mod.rs:146-188`.
Radio buttons share this struct (`:141`); `RadioButton` reads the same fields
(`radio_button.rs:52`, `:94`, `:102`). egui paints `checkbox_frame.fill`
unconditionally (`widgets/checkbox.rs:134-140`) with no checked/unchecked
branch, so the checked-ness axis is carried by `RoleVariant::Selected`.

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `checkbox.background_color` | `widgets.{inactive,open}.bg_fill` «Checkbox, Normal» | SCOPED | used when `unchecked_background` is `None` |
| `checkbox.checked_background` | `widgets.{5}.bg_fill` «Checkbox, Selected» | SCOPED | same egui field, different variant cell; the `hovered` state of that cell holds it too, because native-theme models no checked-hover fill (§6.4) |
| `checkbox.indicator_color` | `widgets.{5}.fg_stroke.color` «Checkbox» | SCOPED | `widget_style.rs:131` → `:188` → `checkbox.rs:151-158`. Inherits `defaults.accent_text_color` when a theme omits it (`docs/inheritance-rules.toml:175`) — the mark sits on `checked_background` — so it is not the label colour; B4 |
| `checkbox.indicator_width` | `spacing.icon_width` (`style.rs:428`) of the base style (base owner, §5.9) and «Checkbox» | DIRECT | base owner of `icon_width`, which every unscoped `Checkbox` reads as its box (`widget_style.rs:179`) and `RadioButton` as its circle (`widgets/radio_button.rs:52`); **not** `icon_width_inner` — egui reads the check mark at `widget_style.rs:180`, and `docs/platform-facts.md:980` defines the field as the box |
| `checkbox.label_gap` | `spacing.icon_spacing` of the base style (base owner, §5.9) and «Checkbox» | DIRECT | inter-atom gap (`atom_layout.rs:302`, `:346-348`) between the box atom a `Checkbox` or `RadioButton` pushes before its label (`widgets/checkbox.rs:92`, `widgets/radio_button.rs:61`), so every unscoped one renders it |
| `checkbox.disabled_opacity` | `visuals.disabled_alpha` «Checkbox» | SCOPED | `unit_interval()` |
| `checkbox.disabled_text_color` | `visuals.override_text_color` and `widgets.inactive.fg_stroke.color` «Checkbox, Disabled» | SCOPED | B4: inside `Role::Checkbox` the label colour is `override_text_color`, which outranks `fg_stroke` for the label (`widget_style.rs:133-136`) and for every plain label in the scope (`style.rs:1136-1139`), so the label takes it there; the check mark strokes `inactive.fg_stroke` (`widget_style.rs:188` → `checkbox.rs:151-158`) and takes it too, since a disabled box shows no accent for `indicator_color` to sit on — iced gives the disabled mark and label the same colour (`connectors/native-theme-iced/src/styles.rs:564-565`) (§6.3) |
| `checkbox.hover_background` | `widgets.{hovered,active}.bg_fill` «Checkbox» | SCOPED | `soft_option`; per-variant fallback in §6.4. A state layer (C17, §6.1): composited over `unchecked_background.unwrap_or(background_color)`, in the `Normal` cell only. It is the unchecked box's hover fill — `windows-11` states it as the "Fluent checkbox hover fill" (`native-theme/src/presets/windows-11.toml:130-131`) — so the hovered `Selected` cell does not read it and copies the idle `Selected` cell (§6.4) |
| `checkbox.disabled_background` | `widgets.inactive.bg_fill` «Checkbox, Disabled» | SCOPED | B2; `checkbox_frame.fill` is the state's `bg_fill` (`widget_style.rs:182`) |
| `checkbox.unchecked_background` | `widgets.{inactive,open}.bg_fill` «Checkbox, Normal» | SCOPED | precedence `unchecked_background.unwrap_or(background_color)` |
| `checkbox.unchecked_border_color` | `widgets.{5}.bg_stroke.color` «Checkbox, Normal» | SCOPED | `widget_style.rs:184` → `checkbox.rs:138`, painted `StrokeKind::Inside` (`:139`); `None` → `border.color` |
| `checkbox.font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `checkbox.font.size` | `override_font_id.size` «Checkbox» | SCOPED | B3; checkbox sets no `fallback_font` (`checkbox.rs:96-100`); × text-scaling factor (§4) |
| `checkbox.font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `checkbox.font.weight` | `role_font_weight(t, Role::Checkbox)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). §5.8 item 2 |
| `checkbox.font.style` | `role_font_is_italic(t, Role::Checkbox)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). §5.8 item 2 |
| `checkbox.font.color` | `visuals.override_text_color` «Checkbox» | SCOPED | B4 — the only exact way to differ from `indicator_color` |
| `checkbox.border.color` | `widgets.{5}.bg_stroke.color` «Checkbox, Selected» | SCOPED | `widget_style.rs:184` |
| `checkbox.border.corner_radius` | `widgets.{5}.corner_radius` «Checkbox» | SCOPED | `widget_style.rs:183` → `checkbox.rs:136` |
| `checkbox.border.line_width` | `widgets.{5}.bg_stroke.width` «Checkbox» | SCOPED | same read site |
| `checkbox.border.shadow_enabled` | — | UNMAPPABLE `source-side gap` | `checkbox_frame` is `..Default::default()` (`widget_style.rs:185`); as `list.border.shadow_enabled`, a `Frame` the application lays round the widget paints a shadow under it (`frame.rs:140`, `:303`): what is missing is the geometry (§14 item 10) |
| `checkbox.border.padding.top` | `spacing.icon_width_inner` «Checkbox» := the cell's `spacing.icon_width` (`checkbox.indicator_width` after its rule) `− (f32::midpoint(left, right) + f32::midpoint(top, bottom))`, floored by `clamp_length` (§6.11) | DERIVED | as `.left`: the mean of both pairs |
| `checkbox.border.padding.right` | `spacing.icon_width_inner` «Checkbox» := the cell's `spacing.icon_width` (`checkbox.indicator_width` after its rule) `− (f32::midpoint(left, right) + f32::midpoint(top, bottom))`, floored by `clamp_length` (§6.11) | DERIVED | as `.left` |
| `checkbox.border.padding.bottom` | `spacing.icon_width_inner` «Checkbox» := the cell's `spacing.icon_width` (`checkbox.indicator_width` after its rule) `− (f32::midpoint(left, right) + f32::midpoint(top, bottom))`, floored by `clamp_length` (§6.11) | DERIVED | as `.left`: the mean of both pairs |
| `checkbox.border.padding.left` | `spacing.icon_width_inner` «Checkbox» := the cell's `spacing.icon_width` (`checkbox.indicator_width` after its rule) `− (f32::midpoint(left, right) + f32::midpoint(top, bottom))`, floored by `clamp_length` (§6.11) | DERIVED | §6.11: the mark's box (`widget_style.rs:180` → `widgets/checkbox.rs:132-133`) and the radio dot's (`radio_button.rs:87`, `:101`); written only where all four sides are stated, else egui's `8.0` stands. The box is square (`widgets/checkbox.rs:133`) and has one inset, so the value subtracted is the mean of the horizontal and the vertical pair's — exact where the pairs agree, as on the one preset that states them (`adwaita`, 3/3/3/3), an approximation where they differ. Both checkbox frames are inert (`Frame::new()` at `widget_style.rs:178`, `checkbox_frame` with `Frame`'s zero margin at `:185`), so the mark's box is the only inset egui has; the mark fills it edge to edge, and the glyph is egui's |

`ResolvedSegmentedControlTheme` — `native-theme/src/model/widgets/mod.rs:773-803`.
Per B5 the documented recipe is a row of `Button::new(label).selected(i == cur)`,
deliberately **not** `Ui::selectable_value`, because `Button::selectable` sets
`frame_when_inactive(false)` (`widgets/button.rs:81`) and would make unselected segments
frameless at rest, whereas `Button::new` leaves it `true` (`widgets/button.rs:54`).
The row is a horizontal `Ui` (`Ui::horizontal`) inside one `Role::SegmentedControl`
scope, whose `Normal` cell also holds the active segment's colours, which each
segment's own `.selected(..)` flag picks (§6.2), so the cell's
`spacing.item_spacing.x` is the gap between two segments
(`egui/src/layout.rs:753`).

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `segmented_control.background_color` | `widgets.{inactive,open}.weak_bg_fill` «SegmentedControl» | SCOPED | requires the `frame_when_inactive == true` idiom |
| `segmented_control.active_background` | `visuals.selection.bg_fill` «SegmentedControl» | SCOPED | `widget_style.rs:150-152` |
| `segmented_control.active_text_color` | `visuals.selection.stroke.color` «SegmentedControl» | SCOPED | `widget_style.rs:153-154` |
| `segmented_control.segment_height` | `spacing.interact_size.y` «SegmentedControl» | SCOPED | contested (§5.11) |
| `segmented_control.separator_width` | `spacing.item_spacing.x` «SegmentedControl» | SCOPED | the gap between two segments of the recipe's horizontal row (`egui/src/layout.rs:753`): `clamp_length(finite_or(w, <base item_spacing.x>))`. egui draws no divider between two `Button`s, so the divider's width is the space the segments leave between them; `.y` stays inherited |
| `segmented_control.disabled_opacity` | `visuals.disabled_alpha` «SegmentedControl» | SCOPED | `unit_interval()` |
| `segmented_control.hover_background` | `widgets.{hovered,active}.weak_bg_fill` «SegmentedControl» | SCOPED | unselected segments only; `soft_option`, `None` → `background_color`. A state layer (C17, §6.1): composited over `segmented_control.background_color` |
| `segmented_control.font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `segmented_control.font.size` | `override_font_id.size` «SegmentedControl» | SCOPED | B3; × text-scaling factor (§4) |
| `segmented_control.font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `segmented_control.font.weight` | `role_font_weight(t, Role::SegmentedControl)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). §5.8 item 2 |
| `segmented_control.font.style` | `role_font_is_italic(t, Role::SegmentedControl)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). §5.8 item 2 |
| `segmented_control.font.color` | `widgets.{5}.fg_stroke.color` «SegmentedControl» | SCOPED | unselected segments only; a selected one paints `selection.stroke` (`widget_style.rs:153-154`) |
| `segmented_control.border.color` | `widgets.{5}.bg_stroke.color` «SegmentedControl» | SCOPED | survives selection: the `SELECTED_CLASS` branch does not touch `bg_stroke` (`widget_style.rs:150-155`) |
| `segmented_control.border.corner_radius` | `widgets.{5}.corner_radius` «SegmentedControl» | SCOPED | uniform only; per-position radii need `Button::corner_radius` per call site (`widgets/button.rs:200-203`) |
| `segmented_control.border.line_width` | `widgets.{5}.bg_stroke.width` «SegmentedControl» | SCOPED | `widget_style.rs:160`; the same state set as `border.color` |
| `segmented_control.border.shadow_enabled` | — | UNMAPPABLE `source-side gap` | `ButtonStyle.frame` is `..Default::default()`; as `list.border.shadow_enabled`, a `Frame` the application lays round the widget paints a shadow under it (`frame.rs:140`, `:303`): what is missing is the geometry (§14 item 10) |
| `segmented_control.border.padding.top` | `spacing.button_padding.y` «SegmentedControl» | DERIVED | as `button.border.padding.top` |
| `segmented_control.border.padding.right` | `spacing.button_padding.x` «SegmentedControl» | DERIVED | as `button.border.padding.right` |
| `segmented_control.border.padding.bottom` | `spacing.button_padding.y` «SegmentedControl» | DERIVED | as `.top` |
| `segmented_control.border.padding.left` | `spacing.button_padding.x` «SegmentedControl» | DERIVED | as `.right` |

### 5.4 Text input, combo box, list

`ResolvedInputTheme` — `native-theme/src/model/widgets/mod.rs:97-137`;
egui counterpart `TextEdit` (`widgets/text_edit/builder.rs`).

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `input.background_color` | `visuals.text_edit_bg_color = Some(..)` (`style.rs:1050`) | DIRECT | `text_edit_bg_color()` falls back to `extreme_bg_color` (`style.rs:1152-1153`), so writing it explicitly takes the input out of that field's contest, which keeps two claimants, `scrollbar.track_color` and `progress_bar.track_color` (§5.11); sole consumer `builder.rs:739`. Base owner of `text_edit_bg_color` (§5.9), whose other claimant is `input.disabled_background` in the `Role::Input` `Disabled` cell (§2) |
| `input.placeholder_color` | `visuals.weak_text_color` «Input» | SCOPED | hint text `builder.rs:632`; rival `defaults.muted_color` via `RichText::weak()` (`widget_text.rs:487`) |
| `input.caret_color` | `visuals.text_cursor.stroke.color` (`style.rs:953`, `:1079`) | DIRECT | sole consumer `text_selection/visuals.rs:268`. Caret width stays egui's `2.0` (`style.rs:971`) — no native caret width exists. The IME composition underlines are **not** recoloured with it: `Visuals::ime_composition` (`style.rs:1033`) is a separate struct whose two strokes egui merely initialises to the same values (`:1641`, `:1655`), and this crate never writes it (§5.10), so a written `caret_color` diverges from the IME underline |
| `input.selection_background` | `visuals.selection.bg_fill` «Input» | SCOPED | `text_selection/visuals.rs:39` |
| `input.selection_text_color` | `visuals.selection.stroke.color` «Input» | SCOPED | `text_selection/visuals.rs:40`. A stock `TextEdit` strokes its focused frame from the same field (`builder.rs:742-747`); `input_frame` strokes it in `input.focus_border_color` instead (§4.7), so the field is the selected text's alone |
| `input.min_height` | `TextEdit::min_size` (`builder.rs:410`), per call | DERIVED | the application reads `input.min_height` from `ResolvedTheme`. `TextEdit` never reads `interact_size`; its height is `(min_inner_height + frame.total_margin().sum().y).at_least(min_size.y)` (`builder.rs:715`, `:521`), so a per-instance `min_size.y` is an exact outer minimum. No `Style` field |
| `input.disabled_opacity` | `visuals.disabled_alpha` «Input» | SCOPED | contested (§5.11); `unit_interval()` |
| `input.disabled_text_color` | `widgets.{noninteractive,inactive}.fg_stroke.color` «Input, Disabled» | SCOPED | `TextEdit` takes its text colour from `inactive` in every state (`builder.rs:480-483`), and a disabled one is `WidgetState::Inactive` (§6.3); `Ui::disable` alone only fades (`ui.rs:497-501`) |
| `input.hover_border_color` | `widgets.{hovered,active}.bg_stroke.color` «Input» | SCOPED | `builder.rs:737` → `:749`; `soft_option`, `None` → `border.color` (no change on hover) |
| `input.focus_border_color` | `input_frame(ui, id, t)` + `TextEdit::frame` (`builder.rs:306`), per call | DERIVED | a focused `TextEdit` strokes its stock frame from `visuals.selection.stroke` (`builder.rs:742-747`), which is already the selected-text colour, so no `Style` field carries this leaf. A frame handed to `TextEdit::frame` is painted as given (`builder.rs:712-713`, `:734-735`), and `input_frame` (§4.7) is egui's own frame with the focused stroke in this colour, chosen by the field's focus, which it reads back before the field is added (§5 introduction). Being per call, it claims no shared field (§5.11). A `soft_option` (`native-theme/src/model/widgets/mod.rs:126-127`): `None` means the border does not change on focus, and `input_frame` then keeps the resting stroke, as iced paints `input.border.color` (`connectors/native-theme-iced/src/styles.rs:380`) |
| `input.disabled_background` | `visuals.text_edit_bg_color` «Input, Disabled» | SCOPED | the fill is `text_edit_bg_color()` (`builder.rs:739`, `style.rs:1152-1154`), not a `WidgetVisuals` fill, so that is the field the `Disabled` cell must carry it in |
| `input.font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `input.font.size` | `text_styles[Body].size` «Input» | SCOPED | `builder.rs:488` → `FontSelection::Default` → `Body` (`style.rs:151-153`); × text-scaling factor (§4) |
| `input.font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `input.font.weight` | `role_font_weight(t, Role::Input)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). The edited text is no `RichText`: it takes the coordinate through `TextEdit::layouter` (`builder.rs:286`) as `TextFormat::coords` (`epaint/src/text/text_layout_types.rs:505`); the hint text through `RichText`. §5.8 item 2 |
| `input.font.style` | `role_font_is_italic(t, Role::Input)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). The edited text takes it through `TextEdit::layouter` (`builder.rs:286`) as `TextFormat::italics` (`epaint/src/text/text_layout_types.rs:507`). §5.8 item 2 |
| `input.font.color` | `widgets.{5}.fg_stroke.color` «Input» | SCOPED | `builder.rs:480-483`; egui deliberately does not use `Style::interact` here (in-source comment at `:482`), so the input's text colour is `inactive` in **every** state |
| `input.border.color` | `widgets.{noninteractive,inactive,open}.bg_stroke.color` «Input» | SCOPED | `builder.rs:749`, `:756`; `hovered` and `active` are `hover_border_color`'s. A `TextEdit::interactive(false)` strokes the `noninteractive` entry (`builder.rs:737`, `style.rs:1273-1274`), and a focused one `selection.stroke` instead (`builder.rs:742-747`), or `input.focus_border_color` in `input_frame` (§4.7) |
| `input.border.corner_radius` | `widgets.{5}.corner_radius` «Input» | SCOPED | `builder.rs:744`, `:749`, `:754`; one radius in every entry, so it does not jump on hover (stock `hovered` is `same(3)`, `style.rs:1704`) |
| `input.border.line_width` | `widgets.{5}.bg_stroke.width` «Input» | SCOPED | same read sites |
| `input.border.shadow_enabled` | — | UNMAPPABLE `source-side gap` | the stock frame never sets `Frame::shadow` (`builder.rs:713`, `:759-769`); a custom `TextEdit::frame` (`builder.rs:306`) could, but `ResolvedWidgetBorder` has no geometry — as `sidebar` — and `input_frame` (§4.7), the custom frame this crate builds, sets none |
| `input.border.padding.top` | `input_margin(t)` + `TextEdit::margin` (`builder.rs:313`) `.top`, per call | DERIVED | the inner margin is the per-instance `TextEdit::margin` (`builder.rs:82`), default `Margin::symmetric(4, 2)` (`:136`), applied at `:713`: a four-sided `Margin`, so each side is its own. The frame's stroke is painted inside that margin (`:763-767`), so `input_margin` (§4.7) is `convert::to_margin` of `convert::padding_with_border(&t.input.border)` over that default — each stated side plus `input.border.line_width` (§5 intro) — and a `None` side keeps egui's. `i8` narrowing: exact where side plus line width is whole; a half-point border (`macos-sonoma`, `native-theme/src/presets/macos-sonoma.toml:43`) rounds each side up half a point. No `Style` field. `visuals.expansion` is rejected as a substitute: symmetric, applied after allocation, and read by nine other widgets. With `input_frame` (§4.7) the margin travels in that frame's inner margin instead, and `TextEdit::margin` is unused (`builder.rs:712-713`) |
| `input.border.padding.right` | `input_margin(t)` + `TextEdit::margin` `.right`, per call | DERIVED | as `.top` |
| `input.border.padding.bottom` | `input_margin(t)` + `TextEdit::margin` `.bottom`, per call | DERIVED | as `.top` |
| `input.border.padding.left` | `input_margin(t)` + `TextEdit::margin` `.left`, per call | DERIVED | as `.top` |

`ResolvedComboBoxTheme` — `native-theme/src/model/widgets/mod.rs:722-764`.

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `combo_box.background_color` | `widgets.{inactive,open}.weak_bg_fill` «ComboBox» | SCOPED | `combo_box.rs:452` → `:460`; the trigger reads `open` while its popup is open (`:449-451`), so both entries carry it (§6.1) |
| `combo_box.min_height` | `spacing.interact_size.y` «ComboBox» | SCOPED | `combo_box.rs:437`, `:444` |
| `combo_box.min_width` | `spacing.combo_width` (`style.rs:418`) | DIRECT | sole reader `combo_box.rs:347`; outer-width semantics match native-theme exactly |
| `combo_box.arrow_icon_size` | `spacing.icon_width` «ComboBox» | SCOPED | `combo_box.rs:342`. The painted chevron is `0.7 × w` by `0.45 × h` inside that box (`:472-476`) — egui's own art, not compensated for |
| `combo_box.arrow_area_width` | `spacing.icon_spacing` «ComboBox» | DERIVED | §6.12. egui has no distinct clickable arrow zone: the whole `outer_rect` senses the click (`:446`). `Option<f32>` (S2): `None` writes nothing, and egui's `icon_spacing` stands |
| `combo_box.disabled_opacity` | `visuals.disabled_alpha` «ComboBox» | SCOPED | `unit_interval()` |
| `combo_box.disabled_text_color` | `widgets.{noninteractive,inactive}.fg_stroke.color` «ComboBox, Disabled» | SCOPED | the label is painted in the state's text colour (`combo_box.rs:389`), and a disabled trigger is `WidgetState::Inactive` (§6.3) |
| `combo_box.hover_background` | `widgets.{hovered,active}.weak_bg_fill` «ComboBox» | SCOPED | `soft_option`; `None` → `background_color`. A state layer (C17, §6.1): composited over `combo_box.background_color` |
| `combo_box.disabled_background` | `widgets.inactive.weak_bg_fill` «ComboBox, Disabled» | SCOPED | `button_frame` fills with the state's `weak_bg_fill` (`combo_box.rs:460`) |
| `combo_box.font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `combo_box.font.size` | `text_styles[Button].size` «ComboBox» | SCOPED | `combo_box.rs:358` selects `TextStyle::Button` explicitly; × text-scaling factor (§4) |
| `combo_box.font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `combo_box.font.weight` | `role_font_weight(t, Role::ComboBox)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). §5.8 item 2 |
| `combo_box.font.style` | `role_font_is_italic(t, Role::ComboBox)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). §5.8 item 2 |
| `combo_box.font.color` | `widgets.{5}.fg_stroke.color` «ComboBox» | SCOPED | `combo_box.rs:389`; this **also** paints the chevron (`:484`), so the arrow cannot differ from the label |
| `combo_box.border.color` | `widgets.{5}.bg_stroke.color` «ComboBox» | SCOPED | `combo_box.rs:461` |
| `combo_box.border.corner_radius` | `widgets.{5}.corner_radius` «ComboBox» | SCOPED | `combo_box.rs:459`. Write `open` too (`:450-451`) — no dispatcher can return it, so it is easy to forget |
| `combo_box.border.line_width` | `widgets.{5}.bg_stroke.width` «ComboBox» | SCOPED | same read site |
| `combo_box.border.shadow_enabled` | — | UNMAPPABLE `source-side gap` | `button_frame` emits a bare `RectShape` (`combo_box.rs:424-470`); as `list.border.shadow_enabled`, a `Frame` the application lays round the widget paints a shadow under it (`frame.rs:140`, `:303`): what is missing is the geometry (§14 item 10) |
| `combo_box.border.padding.top` | `spacing.button_padding.y` «ComboBox» | DERIVED | one component shared with `.bottom`, the mean of the two targets under the pair rule above, a stated side counting with `combo_box.border.line_width` added — with both sides stated, `(top + bottom) / 2 + line_width`. Lossy only in centring: `button_frame` shrinks and expands by it symmetrically (`combo_box.rs:433`, `:439`, `:443`; also `:339`). Here the value stays `f32`, while `Button` narrows the same field to `i8` (`widget_style.rs:163-165` → `epaint/src/margin.rs:117-119`), so a fractional padding renders differently on the two widgets from one `Style` |
| `combo_box.border.padding.right` | `spacing.button_padding.x` «ComboBox» | DERIVED | one component shared with `.left`, the mean of the two targets under the pair rule above, a stated side counting with `combo_box.border.line_width` added — with both sides stated, `(left + right) / 2 + line_width`. Lossy only in centring; same `f32`/`i8` asymmetry |
| `combo_box.border.padding.bottom` | `spacing.button_padding.y` «ComboBox» | DERIVED | as `.top` |
| `combo_box.border.padding.left` | `spacing.button_padding.x` «ComboBox» | DERIVED | as `.right` |

`ResolvedListTheme` — `native-theme/src/model/widgets/mod.rs:499-539`.
Two egui realisations: `Grid` (`egui/src/grid.rs`) and `egui_extras::Table`.
Neither paints a container background.

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `list.background_color` | `Frame::fill` (`frame.rs:258`) on a `Frame` the application lays round the `Grid` or `Table`, per call | DERIVED | neither container paints a background: `Grid` fills a row only with the colour its `with_row_color` closure returns (`grid.rs:255-273`, `:356-362`), and `egui_extras` only stripes, selects and hovers (`egui_extras/src/layout.rs:129-151`). `extreme_bg_color` is rejected: it is the scroll trough, progress trough and canvas fill. No `Surface` recipe exists for a list, so the application builds the `Frame`: `Frame::group(ui.style())` inside the List scope, which brings the scope's `list.border.corner_radius` and `.line_width` (`frame.rs:181-182`), with this fill, the `list.border.padding.*` sides and the `list.border.color` stroke colour set on it per call |
| `list.alternate_row_background` | `visuals.faint_bg_color` (`style.rs:1040`) | DIRECT | both readers are list-shaped (`grid.rs:505-510`, `egui_extras/src/layout.rs:129-134`). egui's own value is *additive* (`from_additive_luminance(5)`, `style.rs:1512`); a native opaque colour will look different from stock, correctly. Painted when the application stripes the list (`Grid::striped`, `TableBuilder::striped`); `Visuals::striped` is never written (§5.10) |
| `list.selection_background` | `visuals.selection.bg_fill` «List» | SCOPED | `egui_extras/src/layout.rs:137-142`; `Grid` has no selection concept |
| `list.selection_text_color` | `visuals.selection.stroke.color` «List» | SCOPED | `egui_extras/src/layout.rs:228-230` |
| `list.header_background` | `Grid::with_row_color` (`grid.rs:356-362`) returning it for row 0, per call | DERIVED | `paint_row` fills each row with that closure's colour (`grid.rs:255-273`). `egui_extras::Table` header cells go through the same `StripLayout::add` with `header`-less flags (`egui_extras/src/table.rs:499-503`, `egui_extras/src/layout.rs:30-38`) and get no fill. In a striped `Grid` the same closure must return the stripe too. `Grid::striped` sets this one closure (`grid.rs:375-383`), so a later `with_row_color` replaces the stripes. Return `list.header_background` for row 0 and `style.visuals.faint_bg_color` for odd rows, as egui's private `striped_row_color` does (`:505-510`) |
| `list.grid_color` | `widgets.noninteractive.bg_stroke.color` «List» | SCOPED | `egui_extras/src/table.rs:897-900`, `egui_extras/src/layout.rs:233-238`. These list lines have no other sink, so the field is this leaf's in the List scope; `list.border.color`, which `Frame::group` would read from the same field (`frame.rs:182`), goes on the application's list `Frame` instead |
| `list.row_height` | `spacing.interact_size.y` «List» | SCOPED | `Grid` only (`grid.rs:447`). `egui_extras::Table` takes row height as a **call argument** (`egui_extras/src/table.rs:976`, `:1027`, `:452`), into which the application passes `list.row_height`, read from `ResolvedTheme`. `Option<f32>` (S2): `None` writes nothing, and egui's `interact_size.y` stands |
| `list.hover_background` | `widgets.{hovered,active}.bg_fill` «List» | SCOPED | `egui_extras/src/layout.rs:145-151`. Note this is `bg_fill`, not `weak_bg_fill`. A row highlight, emitted as given (C17): egui paints it over the row, after any stripe (`:129-134`) |
| `list.hover_text_color` | `RichText::color` (`widget_text.rs:320`) on the hovered row's cell text, per call | DERIVED | `egui_extras` sets `override_text_color` only in the `selected` branch (`egui_extras/src/layout.rs:227-231`) and keeps its hovered-row index to itself (`egui_extras/src/table.rs:770-788`, `:938`), so the application colours the text of the row it knows is hovered with this leaf, read from `ResolvedTheme`: in a `Table`, from the row's `Response` (`TableRow::response`, `egui_extras/src/table.rs:1349`, which panics before the row's first `col`, `:1352`), kept for the next pass — one pass behind; in a `Grid`, from its cells' `Response::hovered` (`response.rs:320`), likewise |
| `list.disabled_text_color` | `widgets.noninteractive.fg_stroke.color` «List, Disabled» | SCOPED | cells are `Label`s, which read `visuals.text_color()` — the `noninteractive` entry — unless they sense clicks (`widgets/label.rs:296-300`), so that is the field the `Disabled` cell must carry it in |
| `list.item_font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `list.item_font.size` | `text_styles[Body].size` «List» | SCOPED | cells are `Label`s; × text-scaling factor (§4) |
| `list.item_font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `list.item_font.weight` | `role_font_weight(t, Role::List)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). §5.8 item 2 |
| `list.item_font.style` | `role_font_is_italic(t, Role::List)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). §5.8 item 2 |
| `list.item_font.color` | `widgets.noninteractive.fg_stroke.color` «List» | SCOPED | `widgets/label.rs:297-300` → `style.rs:1136-1139` |
| `list.header_font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `list.header_font.size` | `list_header_font(t, prefs)` + `RichText::font` (`widget_text.rs:190-198`), per call | DERIVED | the per-call route `text_scale.dialog_title.size` takes (§5.1): the `FontId` is `list_header_font` (§4.7), already scaled, and an explicit size in the header text's own format renders exactly. It loses the global `text_styles[Heading]` to `text_scale.section_heading` (§5.9). egui could carry this through a `TextStyle::Name` key; declined, §5.8 item 5. × text-scaling factor (§4) |
| `list.header_font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `list.header_font.weight` | `t.list.header_font.weight` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | as the other `*.font.weight` rows, except that the application reads the leaf from `ResolvedTheme`: `role_font_weight(t, Role::List)` (§4.7) answers for `list.item_font`, because a `Role` names one font per widget and this widget has two. §5.8 item 2 |
| `list.header_font.style` | `t.list.header_font.style` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | as `.weight`, and as the other `*.font.style` rows for the shear. §5.8 item 2 |
| `list.header_font.color` | `widgets.active.fg_stroke.color` «List», via `Visuals::strong_text_color()` (`style.rs:1147-1149`) | SCOPED | trade-off: any interactive widget in a cell then renders its pressed text in the header colour — over that widget's pressed fill, a pair no platform states (C8). The recommended default is per call: the application reads `list.header_font.color` from `ResolvedTheme` into `RichText::color(..)` |
| `list.border.color` | `Frame::stroke` colour (`frame.rs:116`, `:267`) on the application's list `Frame`, per call | DERIVED | the List scope's `noninteractive.bg_stroke.color` is `list.grid_color`'s: a resizable column's edge line and a row's overline read it (`egui_extras/src/table.rs:846`, `:897-900`; `egui_extras/src/layout.rs:233-238`) and have no other sink, while `Frame::group` reads the same field for its outline (`frame.rs:182`). So the list's outline is the `Frame` the application already lays round the list (`list.background_color`), whose `stroke.color` it sets to this leaf; the width and radius stay the scope's. The two leaves never meet in one field (§5.11) |
| `list.border.corner_radius` | `widgets.noninteractive.corner_radius` «List» | SCOPED | `frame.rs:181`, `:230`; the application's list `Frame` is `Frame::group` built in the List scope (`list.background_color`), so it takes this radius |
| `list.border.line_width` | `widgets.noninteractive.bg_stroke.width` «List» | SCOPED | the application's list `Frame` (`Frame::group` in the List scope, `frame.rs:182`) takes it; the grid lines read the same width, which no other leaf states |
| `list.border.shadow_enabled` | — | UNMAPPABLE `source-side gap` | `Frame::group` sets no shadow; the application's list `Frame` (`list.background_color`) could carry one, but `ResolvedWidgetBorder` has no geometry — as `sidebar` |
| `list.border.padding.top` | `Frame::inner_margin` (`frame.rs:248`) `.top` on the application's list `Frame`, per call | DERIVED | `Frame::group` hardcodes `6` (`frame.rs:180`); the list `Frame` of `list.background_color` carries each side exactly, and a `None` side keeps egui's `6`. `item_spacing` is rejected: it is the inter-cell gap **and** the gap after every widget in every layout |
| `list.border.padding.right` | `Frame::inner_margin` `.right`, per call | DERIVED | as `.top` |
| `list.border.padding.bottom` | `Frame::inner_margin` `.bottom`, per call | DERIVED | as `.top` |
| `list.border.padding.left` | `Frame::inner_margin` `.left`, per call | DERIVED | as `.top` |

### 5.5 Indicators

scrollbar, slider, progress bar, splitter, separator, spinner.

Three structural facts:

* **I1** — the slider **rail is hard-wired to `inactive`**: `slider_ui` paints
  it with `widget_visuals.inactive.bg_fill` and `.inactive.corner_radius`
  (`slider.rs:772`, `:775`) regardless of interaction, while the *handle* uses
  `ui.style().interact(response)` (`:766`), which at rest is also `inactive`.
  Rail and resting handle therefore read the identical `Color32`.
* **I2** — `ProgressBar` and `Spinner` sense `hover()` only
  (`progress_bar.rs:117`, `spinner.rs:68`), so they never consult the five-state
  table — except that `Spinner` reads `widgets.active` directly through
  `Visuals::strong_text_color()` (`spinner.rs:44` → `style.rs:1147-1149`).
* **I3** — `Style::separator_style` ignores both `_classes` and `_state`
  (`widget_style.rs:212`) and hardcodes `spacing: 6.0` (`:215`). A `Separator`
  has exactly one appearance, forever — which matches native-theme supplying
  exactly one.

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `scrollbar.track_color` | `visuals.extreme_bg_color` of the base style (base owner, §5.9) and «Scrollbar» | DIRECT | `scroll_area.rs:1509-1511`; base owner of `extreme_bg_color` after the input is removed by `text_edit_bg_color` |
| `scrollbar.thumb_color` | `widgets.{inactive,open}.bg_fill` of the base style (base owner, §5.9) and «Scrollbar» | DIRECT | `scroll_area.rs:1499-1503`, the resting handle (`:1457-1466`). Every unscoped `ScrollArea` renders it, egui's own inside a `Window` or a `ComboBox` list among them; the «Scrollbar» scope writes the same value. The connector **must** set `spacing.scroll.foreground_color = false` (`style.rs:538`), or the handle colour comes from `fg_stroke.color`, which is also the text colour. The handle's radius is the state's `corner_radius`, the base style's `button.border.corner_radius` outside a scope (§6.8) |
| `scrollbar.thumb_hover_color` | `widgets.hovered.bg_fill` of the base style (base owner, §5.9) and «Scrollbar» | DIRECT | `scroll_area.rs:1451-1461` — pointer inside the handle rect, i.e. exactly "thumb hover". A thumb colour, emitted as given (C17): egui paints the handle over the trough it also paints (`:1506-1512`) |
| `scrollbar.groove_width` | `spacing.scroll.{bar_width,bar_inner_margin,bar_outer_margin}` | DERIVED | §6.5 (D1) |
| `scrollbar.min_thumb_length` | `spacing.scroll.handle_min_length` (`style.rs:515`) | DIRECT | sole reader `scroll_area.rs:1380`; sole claimant |
| `scrollbar.thumb_width` | `spacing.scroll.bar_width` / `floating_width` | DERIVED | §6.5 (D1); trough and handle share one cross-range (`scroll_area.rs:1360-1364` vs `:1388-1398`) |
| `scrollbar.overlay_mode` | `spacing.scroll.floating` + `floating_allocated_width` | DERIVED | §6.5 (D1); one bool drives two fields |
| `scrollbar.thumb_active_color` | `widgets.active.bg_fill` of the base style (base owner, §5.9) and «Scrollbar» | DIRECT | the handle while the bar is pressed (`scroll_area.rs:1458-1459`). `soft_option`; `None` → `thumb_hover_color` (§6.9, D5) |
| `slider.fill_color` | `visuals.selection.bg_fill` «Slider» + `visuals.slider_trailing_fill = true` | SCOPED | `slider.rs:802`, gated at `:781-783`; egui's default for the bool is `false` (`style.rs:1552`), so **without writing it the theme's fill never appears** |
| `slider.track_color` | `widgets.{inactive,open}.bg_fill` «Slider» | SCOPED | I1 |
| `slider.thumb_color` | `widgets.{hovered,active}.bg_fill` «Slider», where `thumb_hover_color` is `None` | SCOPED | the hovered and pressed handle only, through D6's copy (§6.4, §6.9): every full bundled preset states `thumb_hover_color` in both modes, and the live pipeline merges that preset under the reader (`native-theme/src/pipeline.rs:50-52`), so `thumb_hover_color` holds both entries on every bundled theme and this leaf reaches the screen only from a theme that leaves it unstated. The resting handle is lost: I1 makes rail and resting handle one field, and `track_color` wins it because the rail has no alternative sink (§5.11) |
| `slider.track_height` | `spacing.slider_rail_height` (`style.rs:415`) | DIRECT | `slider.rs:770-771`; sole reader, sole claimant |
| `slider.thumb_diameter` | `spacing.interact_size.y` «Slider», and `widgets.{5}.expansion` «Slider» where the Body row is taller than `1.25 · thumb_diameter` — at a text-scaling factor of `1.0`, on no bundled preset (§6.6) | DERIVED | §6.6 (D2): `interact_size.y` from the diameter, and one `expansion` in every state computed at build from the Body row height, `−fg_stroke.width` wherever that row is below `1.25 · thumb_diameter`, so the painted knob — the disc plus the outline tessellated **outside** it (`epaint/src/tessellator.rs:1531`) — is `thumb_diameter` across at every text size. The cost is the value box beside the slider: its `Button` frame reads the same `expansion` (`widget_style.rs:162-165`), so its painted frame is inset by `−expansion` — `fg_stroke.width`, 1 point at egui's `1.0`, where the row is below `1.25 · thumb_diameter` — on every side. |
| `slider.tick_mark_length` | — | UNMAPPABLE `egui-limited` | `slider_ui` emits three shape groups and no ticks (`slider.rs:659-837`) |
| `slider.disabled_opacity` | `visuals.disabled_alpha` «Slider» (`style.rs:1126`) | SCOPED | contested (§5.11), base owner `defaults.disabled_opacity` (§5.9); `ui.rs:497-503`; `unit_interval()` mandatory |
| `slider.thumb_hover_color` | `widgets.{hovered,active}.bg_fill` «Slider» | SCOPED | clean sink inside the slider scope, because the rail is pinned to `inactive`. `None` → `thumb_color`, written as given into both entries (§6.4, §6.9 D6) |
| `slider.disabled_fill_color` | `visuals.selection.bg_fill` «Slider, Disabled» | SCOPED | a disabled slider still paints its trailing fill from `selection.bg_fill` (`slider.rs:799-803`), so the `Disabled` cell writes `disabled_fill_color.unwrap_or(fill_color)` there; B2 |
| `slider.disabled_track_color` | `widgets.inactive.bg_fill` «Slider, Disabled» | SCOPED | the rail is always `inactive.bg_fill` (`slider.rs:775`, I1), so the `Disabled` cell writes `disabled_track_color.unwrap_or(track_color)` there; B2 |
| `slider.disabled_thumb_color` | — | UNMAPPABLE `widgets-crate` | I1: a disabled handle is `WidgetState::Inactive` (§6.3), the same `inactive.bg_fill` as the rail, which `disabled_track_color` holds. The companion crate's `Slider` paints its disabled knob in it (`docs/todo_egui-widgets-spec.md` §4.2) |
| `progress_bar.fill_color` | `visuals.selection.bg_fill` «ProgressBar» | SCOPED | `progress_bar.rs:155-161`; the animation `color_factor` is hardcoded (`:147-153`). A label, when the app sets one, is painted in `selection.stroke.color` (`:197-199`) — in this scope the base style's `defaults.selection_text_color` — over this fill and the track: a text-on-fill pair the connector assembles (C8) |
| `progress_bar.track_color` | `visuals.extreme_bg_color` «ProgressBar» | SCOPED | `progress_bar.rs:139-140`; displaced by `scrollbar.track_color` globally |
| `progress_bar.track_height` | `spacing.interact_size.y` «ProgressBar» | SCOPED | `progress_bar.rs:115-117` — **exact**, not a floor. Semantic loss on Windows, where the groove is 1 inside a 3-high control (`docs/platform-facts.md:1303`) and egui's bar *is* the groove |
| `progress_bar.min_width` | `ProgressBar::desired_width` (`progress_bar.rs:41`), per call | DERIVED | the `96.0` floor is a literal that applies only when no width is given (`progress_bar.rs:113-114`); `desired_width(available.at_least(min_width))` carries the native floor |
| `progress_bar.border.color` | — | UNMAPPABLE `egui-limited` | `ProgressBar::ui` paints only filled rects and a galley — no frame stroke and no `Frame` (`progress_bar.rs:130-204`); the sole `Stroke` in the widget is the indeterminate-animation arc at `:178-179`, inside the `animate && !has_custom_cr` branch opened at `:163`, which is not a border. `docs/platform-facts.md` §2.10 states no progress-bar border (its table has no `border.color` or `border.line_width` row): the resolved value is `defaults.border`'s by inheritance (`docs/inheritance-rules.toml:86-93`); see §5.8 item 8. egui could carry this through `Frame::stroke` on a `Frame` laid round the bar; declined, §5.8 item 8 |
| `progress_bar.border.corner_radius` | `ProgressBar::corner_radius` (`progress_bar.rs:93`), per call | DERIVED | without it the bar is always a pill: the radius falls back to `half_height` (`progress_bar.rs:136-138`, where `half_height = outer_rect.height() / 2.0` at `:137`). A custom radius also switches off the indeterminate arc (`:163`, documented at `:89-91`) |
| `progress_bar.border.line_width` | — | UNMAPPABLE `egui-limited` | no frame stroke; see `progress_bar.border.color`. `docs/platform-facts.md` §2.10 states no progress-bar border (its table has no `border.color` or `border.line_width` row): the resolved value is `defaults.border`'s by inheritance (`docs/inheritance-rules.toml:86-93`); see §5.8 item 8. egui could carry this through `Frame::stroke` on a `Frame` laid round the bar; declined, §5.8 item 8 |
| `progress_bar.border.shadow_enabled` | — | UNMAPPABLE `source-side gap` | no per-widget shadow, and `docs/platform-facts.md:1306` records "no shadow" on all four platforms; as `list.border.shadow_enabled`, a `Frame` the application lays round the widget paints a shadow under it (`frame.rs:140`, `:303`): what is missing is the geometry (§14 item 10) |
| `progress_bar.border.padding.top` | — | UNMAPPABLE `egui-limited` | the bar has no vertical inset; the label is centred by construction (`progress_bar.rs:195`) |
| `progress_bar.border.padding.right` | — | UNMAPPABLE `egui-limited` | the label is placed from the left edge and clipped to the bar (`progress_bar.rs:195-196`, `:201`); nothing reserves a right inset |
| `progress_bar.border.padding.bottom` | — | UNMAPPABLE `egui-limited` | as `.top` |
| `progress_bar.border.padding.left` | `spacing.item_spacing.x` «ProgressBar» | SCOPED | the label's only inset (`progress_bar.rs:195-196`); effective only when the app calls `ProgressBar::text` |
| `splitter.divider_width` | `widgets.noninteractive.bg_stroke.width` + `widgets.{hovered,active}.fg_stroke.width` «Splitter» | SCOPED | `panel.rs:911`, `:908`, `:906`, read from the `Ui` the panel is shown in, so `Role::Splitter` is set on that `Ui` with `native_set_style` for a resizable panel's `Panel::show` (§4.4). All three must be written to keep constant thickness. egui reserves room with `bg_stroke.width.round() as i8` (`panel.rs:962`) + `saturating_add` (`:964-965`), so a width above 127 is clamped to 127 points of reserved space rather than growing further — the `as i8` saturates (§7.1) and `saturating_add` then stops at `i8::MAX` |
| `splitter.divider_color` | `widgets.noninteractive.bg_stroke.color` «Splitter» | SCOPED | `panel.rs:911`; genuinely diverges from `separator.line_color` in shipped presets |
| `splitter.hover_color` | `widgets.{hovered,active}.fg_stroke.color` «Splitter» | SCOPED | `panel.rs:908`, `:1034`; the drag line too (`:906`, `:1032`), which has no native colour of its own (§6.9, D7) |
| `separator.line_color` | `widgets.noninteractive.bg_stroke.color` «Separator» | SCOPED | `widget_style.rs:212-218` → `separator.rs:106-109`, painted `:134-144` |
| `separator.line_width` | `widgets.noninteractive.bg_stroke.width` «Separator» | SCOPED | same `Stroke`; on macOS `splitter.divider_width` is 6.0 against `separator.line_width` 0.5 — a 12× divergence on one field |
| `spinner.fill_color` | `widgets.active.fg_stroke.color` «Spinner» | SCOPED | `spinner.rs:42-44` → `strong_text_color()` (`style.rs:1147-1149`) |
| `spinner.diameter` | `spacing.interact_size.y` «Spinner» | DERIVED | §6.7 (D3); `allocate_exact_size` (`spinner.rs:65-68`) |
| `spinner.min_diameter` | `spacing.interact_size.y` «Spinner» | DERIVED | §6.7 (D3) |
| `spinner.stroke_width` | — | UNMAPPABLE `widgets-crate` | `Stroke::new(3.0, color)` is a literal (`spinner.rs:57-58`); no width builder, no `Style` field. The companion crate's `Spinner` paints its arc at this width where the icon set has no indicator (`docs/todo_egui-widgets-spec.md` §4.3) |

The painted spinner ring is inset from its allocated box —
`radius = (min(w, h) / 2.0) − 2.0` (`spinner.rs:45`) with a 3.0-wide stroke
centred on that path (`:58`) — so the visible outer diameter is
`interact_size.y − 1.0`. **Do not compensate.** Adding `1.0` would bake egui's
`2.0` and `3.0` literals into the connector, and they are exactly the kind of
value that changes without a compile error. Map the allocated size and document
the one-point shortfall.

### 5.6 Chrome — tab, sidebar, toolbar, status bar, expander

`ResolvedTabTheme` — `native-theme/src/model/widgets/mod.rs:373-405`.
egui realisation: a row of `Button::new(label).selected(i == cur)` — per B5 (§5.3),
not `Button::selectable`, whose unselected tab paints no resting frame
(`widgets/button.rs:81`, `:364-368`), so `tab.background_color` would not show. **Native "active tab" means
*selected*, which in egui is the `SELECTED_CLASS` branch
(`widget_style.rs:150-155`) and `Visuals::selection`, not `widgets.active`** —
`widgets.active` is pointer-down-or-keyboard-focus (`style.rs:1276-1278`).
Conflating them is the easiest way to produce a plausible but wrong connector.

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `tab.background_color` | `widgets.{inactive,open}.weak_bg_fill` «Tab» | SCOPED | `widget_style.rs:159` → `widgets/button.rs:365` |
| `tab.active_background` | `visuals.selection.bg_fill` «Tab» | SCOPED | SELECTED_CLASS branch, not `widgets.active`. `Button::selected` (`widgets/button.rs:263-269`) opts the tab into toggle semantics, so assistive technology announces a pressed toggle rather than a selected tab (`widgets/button.rs:263-269`, `response.rs:975-980`) — egui-limited, a §14 ledger entry |
| `tab.active_text_color` | `visuals.selection.stroke.color` «Tab» | SCOPED | `widget_style.rs:153-154`. `Selection::stroke.width` has no native source; stays at egui's `1.0` (`style.rs:1620`) |
| `tab.bar_background` | `visuals.panel_fill` «Tab», read by `Frame::side_top_panel` of a `Panel` shown inside a `Role::Tab` scope, which builds its frame from its parent `Ui` (`panel.rs:708`, `:950`; `frame.rs:185-189`) | SCOPED | `panel_fill` is contested (§5.11) |
| `tab.min_width` | `Button::min_size` (`widgets/button.rs:193`), per call | DERIVED | as `button.min_width`: `Button` raises only the cross axis from `interact_size` (`widgets/button.rs:306-309`), and `interact_size.x` is a `Grid`/`DragValue`/colour-swatch field |
| `tab.min_height` | `spacing.interact_size.y` «Tab» | SCOPED | contested (§5.11); writing only `.y` leaves `.x` inherited (§7.5) |
| `tab.hover_text_color` | `widgets.{hovered,active}.fg_stroke.color` «Tab» | SCOPED | correctly applies to unselected tabs only |
| `tab.hover_background` | `widgets.{hovered,active}.weak_bg_fill` «Tab» | SCOPED | unselected tabs only; `soft_option`, `None` → mirror `tab.background_color` (§6.4). A state layer (C17, §6.1): composited over `tab.background_color`, the resting tab fill, which `Button::new` paints (`widgets/button.rs:54`, `:364`); where that fill is transparent the composite is the layer itself, lying over the tab bar as on the platform. One egui field, one copy — the same shape as `combo_box.hover_background` and `segmented_control.hover_background`, so SCOPED like them |
| `tab.font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `tab.font.size` | `text_styles[Body].size` «Tab» | SCOPED | B3; × text-scaling factor (§4) |
| `tab.font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `tab.font.weight` | `role_font_weight(t, Role::Tab)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). §5.8 item 2 |
| `tab.font.style` | `role_font_is_italic(t, Role::Tab)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). §5.8 item 2 |
| `tab.font.color` | `widgets.{noninteractive,inactive,open}.fg_stroke.color` «Tab» | SCOPED | contested (§5.11) |
| `tab.border.color` | `widgets.{5}.bg_stroke.color` «Tab» | SCOPED | `widget_style.rs:160` |
| `tab.border.corner_radius` | `widgets.{5}.corner_radius` «Tab» | SCOPED | `widget_style.rs:161`; `u8` narrowing |
| `tab.border.line_width` | `widgets.{5}.bg_stroke.width` «Tab» | SCOPED | coupled to padding: a tab is a `Button`, whose stroke lies inside `button_padding` (`widget_style.rs:163-165`), so the padding rows add this width to what they write (§5 intro); every bundled preset resolves it to `0` in both modes (measured 2026-09-25) |
| `tab.border.shadow_enabled` | — | UNMAPPABLE `source-side gap` | `WidgetVisuals` has no shadow field; `ButtonStyle.frame` sets none; as `list.border.shadow_enabled`, a `Frame` the application lays round the widget paints a shadow under it (`frame.rs:140`, `:303`): what is missing is the geometry (§14 item 10) |
| `tab.border.padding.top` | `spacing.button_padding.y` «Tab» | DERIVED | as `button.border.padding.top`, with `tab.border.line_width` as the line width. Each side is realised as `round(p − w) + w` — `p` the written `button_padding.y`, `w` the line width — because `button_style` narrows `p − w` to whole points (`widget_style.rs:163-165` → `epaint/src/margin.rs:117-119`) and the frame's stroke adds `w` back (`containers/frame.rs:327-331`); the realised height is `max(interact_size.y, galley + 2·(round(p − w) + w))` |
| `tab.border.padding.right` | `spacing.button_padding.x` «Tab» | DERIVED | as `button.border.padding.right` |
| `tab.border.padding.bottom` | `spacing.button_padding.y` «Tab» | DERIVED | as `.top` |
| `tab.border.padding.left` | `spacing.button_padding.x` «Tab» | DERIVED | as `.right` |

`ResolvedSidebarTheme` — `native-theme/src/model/widgets/mod.rs:415-434`,
`border_kind = "partial"` (`:413-414`), so only `border.color` and
`border.line_width` inherit; radius and shadow default to `0.0`/`false`
(`validate_helpers.rs:328-333`), and a padding side the theme does not state is
`None` (`:340`). No shipped preset supplies a `[*.sidebar.border]` table, so
today a sidebar's radius is `0.0` and its padding is egui's own.

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `sidebar.background_color` | `Frame::fill` «Surface::Panel(Left\|Right)», and `visuals.panel_fill` «Sidebar» for a fixed side panel | SCOPED | a `Panel` resolves its frame and its separator line from the **parent** `Ui` (`panel.rs:708`, `:947-950`, `:906-911`). A resizable side panel's `Panel::show` runs inside `Role::Splitter`, which owns its resize line, and its content inside `Role::Sidebar` (§4.4), so the `Surface::Panel` frame handed to `Panel::frame` (`panel.rs:413`) carries the fill; a fixed side panel's `Panel::show` runs inside `Role::Sidebar`, whose `panel_fill` also reaches the stock frame |
| `sidebar.selection_background` | `visuals.selection.bg_fill` «Sidebar» | SCOPED | items are `selectable_label`s |
| `sidebar.selection_text_color` | `visuals.selection.stroke.color` «Sidebar» | SCOPED | `widget_style.rs:154` |
| `sidebar.hover_background` | `widgets.{hovered,active}.weak_bg_fill` «Sidebar» | SCOPED | **required**, not a `soft_option` (`native-theme/src/model/widgets/mod.rs:426-427`), so it needs no fallback. A row highlight, emitted as given (C17) |
| `sidebar.font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `sidebar.font.size` | `text_styles[Body].size` «Sidebar» | SCOPED | a sidebar mixes `Label`s and `selectable_label`s, and both resolve `TextStyle::Body` when `override_font_id` is unset (`style.rs:159-166`; B3); × text-scaling factor (§4) |
| `sidebar.font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `sidebar.font.weight` | `role_font_weight(t, Role::Sidebar)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). §5.8 item 2 |
| `sidebar.font.style` | `role_font_is_italic(t, Role::Sidebar)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). §5.8 item 2 |
| `sidebar.font.color` | `widgets.{5}.fg_stroke.color` «Sidebar» | SCOPED | plain labels take `noninteractive`, unselected `selectable_label`s the interactive entries; `ResolvedSidebarTheme` states no hover text colour, so the hovered item keeps it (§6.1) |
| `sidebar.border.color` | `widgets.noninteractive.bg_stroke.color` «Sidebar» | SCOPED | the visible edge of a fixed side panel, whose `Panel::show` runs inside `Role::Sidebar`: the panel separator (`panel.rs:911`). A resizable side panel's edge is its resize line, drawn from the `Role::Splitter` style live on the `Ui` the panel is shown in, in `splitter.*`'s colours (§5.5); there this leaf reaches the separators and group frames inside the sidebar |
| `sidebar.border.corner_radius` | `Frame::corner_radius` «Surface::Panel» | SCOPED | no `Style` field: `Frame::side_top_panel` never calls `.corner_radius(..)` (`frame.rs:185-189`) |
| `sidebar.border.line_width` | `widgets.noninteractive.bg_stroke.width` «Sidebar» | SCOPED | as `sidebar.border.color`: the edge of a fixed side panel only. Separator room is `width.round() as i8` (`panel.rs:962`) + `saturating_add` (`:964-965`) — egui's own saturating precedent; a width above 127 is clamped to 127 points of reserved space |
| `sidebar.border.shadow_enabled` | — | UNMAPPABLE `source-side gap` | `Frame.shadow` exists (`frame.rs:140`, `:303`) but `ResolvedWidgetBorder` supplies no offset/blur/spread; borrowing egui's `blur: 15, offset: [10,20]` would bake an egui constant into a "native" theme |
| `sidebar.border.padding.top` | `Frame::inner_margin.top` «Surface::Panel» | SCOPED | overrides the hardcoded `Margin::symmetric(8, 2)` (`frame.rs:187`), whose side a `None` keeps |
| `sidebar.border.padding.right` | `Frame::inner_margin.right` «Surface::Panel» | SCOPED | as `.top` |
| `sidebar.border.padding.bottom` | `Frame::inner_margin.bottom` «Surface::Panel» | SCOPED | as `.top` |
| `sidebar.border.padding.left` | `Frame::inner_margin.left` «Surface::Panel» | SCOPED | as `.top` |

`ResolvedToolbarTheme` — `native-theme/src/model/widgets/mod.rs:443-469`.

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `toolbar.background_color` | `Frame::fill` «Surface::Panel(Top)», and `visuals.panel_fill` «Toolbar» | SCOPED | contested (§5.11) |
| `toolbar.bar_height` | `spacing.interact_size.y` minus the frame margin «Toolbar» | DERIVED | §6.10 (R-BAR). `Option<f32>` (S2): `None` — the platform's toolbar sizes to its content — writes nothing, and egui's `interact_size.y` stands |
| `toolbar.item_gap` | `spacing.item_spacing.x` «Toolbar» | SCOPED | documented as *horizontal* (`native-theme/src/model/widgets/mod.rs:453`), so only `.x` is written; `.y` has no toolbar source and stays inherited. `Option<f32>` (S2): `None` writes nothing, and egui's `item_spacing.x` stands |
| `toolbar.icon_size` | `Image::fit_to_exact_size` (`widgets/image.rs:177`) on each toolbar item's image atom, per call | DERIVED | no general icon size in `Spacing`'s 21 fields; image atoms carry their own size (`atom_layout.rs:517`, `:569`), so the application reads `toolbar.icon_size` from `ResolvedTheme` into each item's `Image`, which renders it exactly on an item built with `Button::new`; `Button::image` and `image_and_text` clamp the atom to the Body row height (`widgets/button.rs:113`, `:311-313`; §9.4) — the route of `menu.icon_size` (§5.2). **Not** `icons::icon_size`, which reads `defaults.icon_sizes` |
| `toolbar.font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `toolbar.font.size` | `text_styles[Body].size` «Toolbar» | SCOPED | toolbars mix `Button`s and `Label`s, and both resolve `TextStyle::Body` (B3); × text-scaling factor (§4) |
| `toolbar.font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `toolbar.font.weight` | `role_font_weight(t, Role::Toolbar)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). §5.8 item 2 |
| `toolbar.font.style` | `role_font_is_italic(t, Role::Toolbar)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). §5.8 item 2 |
| `toolbar.font.color` | `widgets.{5}.fg_stroke.color` «Toolbar» | SCOPED | plain labels take `noninteractive`, the toolbar's buttons the interactive entries (§6.1) |
| `toolbar.border.color` | `widgets.noninteractive.bg_stroke.color` «Toolbar» | SCOPED | panel separator (`panel.rs:911`): a toolbar is a fixed top panel, whose `Panel::show` runs inside `Role::Toolbar` (§4.4) |
| `toolbar.border.corner_radius` | `Frame::corner_radius` «Surface::Panel(Top)» | SCOPED | not reachable from `Style` |
| `toolbar.border.line_width` | `widgets.noninteractive.bg_stroke.width` «Toolbar» | SCOPED | `panel.rs:911`, `:962-965` |
| `toolbar.border.shadow_enabled` | — | UNMAPPABLE `source-side gap` | as sidebar. `docs/platform-facts.md` §2.13 states no toolbar shadow; the leaf inherits `defaults.border.shadow_enabled` (`docs/inheritance-rules.toml:88`, `:94`) |
| `toolbar.border.padding.top` | `Frame::inner_margin.top` «Surface::Panel(Top)» | SCOPED | overrides `Margin::symmetric(8, 2)` (`frame.rs:187`), whose side a `None` keeps; `.top` + `.bottom` is the term subtracted in R-BAR (§6.10) |
| `toolbar.border.padding.right` | `Frame::inner_margin.right` «Surface::Panel(Top)» | SCOPED | as `.top` |
| `toolbar.border.padding.bottom` | `Frame::inner_margin.bottom` «Surface::Panel(Top)» | SCOPED | as `.top` |
| `toolbar.border.padding.left` | `Frame::inner_margin.left` «Surface::Panel(Top)» | SCOPED | as `.top` |

`ResolvedStatusBarTheme` — `native-theme/src/model/widgets/mod.rs:479-489`.
The only status-bar table a shipped preset declares is `[*.status_bar.border]`
with the four paddings, in `adwaita` and `kde-breeze`
(`native-theme/src/presets/adwaita.toml:220-223`,
`native-theme/src/presets/kde-breeze.toml:215-220`); every other leaf arrives by
inheritance from `defaults`.

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `status_bar.background_color` | `Frame::fill` «Surface::Panel(Bottom)», and `visuals.panel_fill` «StatusBar» | SCOPED | `Panel::bottom` is `.resizable(false)` by construction (`panel.rs:274-276`) |
| `status_bar.font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `status_bar.font.size` | `text_styles[Body].size` «StatusBar» | SCOPED | content is `Label`s; × text-scaling factor (§4) |
| `status_bar.font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `status_bar.font.weight` | `role_font_weight(t, Role::StatusBar)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). §5.8 item 2 |
| `status_bar.font.style` | `role_font_is_italic(t, Role::StatusBar)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). §5.8 item 2 |
| `status_bar.font.color` | `widgets.{5}.fg_stroke.color` «StatusBar» | SCOPED | plain labels take `noninteractive`, a status bar's buttons the interactive entries (§6.1). A status bar's "secondary" text has no native source and inherits egui's `weak_text_alpha` of `0.6` (`style.rs:1506`) unless `defaults.muted_color` supplies `weak_text_color` |
| `status_bar.border.color` | `widgets.noninteractive.bg_stroke.color` «StatusBar» | SCOPED | bottom-panel separator (`panel.rs:911`): a status bar is a fixed bottom panel, whose `Panel::show` runs inside `Role::StatusBar` (§4.4) |
| `status_bar.border.corner_radius` | `Frame::corner_radius` «Surface::Panel(Bottom)» | SCOPED | not reachable from `Style` |
| `status_bar.border.line_width` | `widgets.noninteractive.bg_stroke.width` «StatusBar» | SCOPED | `panel.rs:911`, `:962-965` |
| `status_bar.border.shadow_enabled` | — | UNMAPPABLE `source-side gap` | as sidebar |
| `status_bar.border.padding.top` | `Frame::inner_margin.top` «Surface::Panel(Bottom)» | SCOPED | overrides `Margin::symmetric(8, 2)` (`frame.rs:187`), whose side a `None` keeps. `ResolvedStatusBarTheme` has no height field, so the bar's height stays `interact_size.y + total_margin` (`panel.rs:1072`) |
| `status_bar.border.padding.right` | `Frame::inner_margin.right` «Surface::Panel(Bottom)» | SCOPED | as `.top` |
| `status_bar.border.padding.bottom` | `Frame::inner_margin.bottom` «Surface::Panel(Bottom)» | SCOPED | as `.top` |
| `status_bar.border.padding.left` | `Frame::inner_margin.left` «Surface::Panel(Bottom)» | SCOPED | as `.top` |

`ResolvedExpanderTheme` — `native-theme/src/model/widgets/mod.rs:828-849`;
egui counterpart `CollapsingHeader` (`containers/collapsing_header.rs:361`).
Three things the `Role::Expander` scope must get right, none of which is a
native-theme field:

* `visuals.collapsing_header_frame = true` (`style.rs:1093`, default **`false`**
  at `:1547`) — it gates the header's only background *and* border layer
  (`collapsing_header.rs:561-568`). Without it, `expander.border.*` and
  `expander.hover_background` are painted nowhere. The second layer in the
  source, the `bg_fill` rectangle at `:571-581`, runs only for a `selected` or
  `selectable` header, and `CollapsingHeader` sets both `false`
  (`collapsing_header.rs:405-406`) with no public method that changes them
  (`:396-483`), so it never runs in 0.36.2.
* `widgets.inactive.weak_bg_fill = Color32::TRANSPARENT` — that layer fills the
  header with the state's `weak_bg_fill` (`collapsing_header.rs:565`), and at
  rest the state is `inactive`. A native expander paints no fill at rest, and
  `ResolvedExpanderTheme` has no leaf for one
  (`native-theme/src/model/widgets/mod.rs:828-849`), so the resting fill is
  transparent — which `weak_bg_fill`, unlike `bg_fill`, may be
  (`style.rs:1297-1300`) — and no button colour appears where the platform
  draws none. The hover fill is then the same layer in the `hovered` state.
* `spacing.indent` (`style.rs:404`) is the arrow **column width**
  (`collapsing_header.rs:516`, `:587`). native-theme has no expander indent
  property, so it is left inherited. The connector must **not** bake in egui's
  `18.0` (`style.rs:1459`).

| leaf | egui sink | verdict | note |
|---|---|---|---|
| `expander.header_height` | `spacing.interact_size.y` «Expander» | SCOPED | `collapsing_header.rs:531-533`; a floor, and `.at_least(interact_size)` clamps **both** axes (§7.5) |
| `expander.arrow_icon_size` | `spacing.icon_width_inner` «Expander» | DERIVED | §6.11 (R-ARROW). `Spacing::icon_width` is irrelevant to `CollapsingHeader` — only the standalone `show_button_indented` path reads it (`:93`) |
| `expander.hover_background` | `widgets.{hovered,active}.weak_bg_fill` «Expander» | SCOPED | the frame layer's fill (`collapsing_header.rs:565`); the `bg_fill` overlay never runs (prerequisites above). `soft_option`, `None` → no highlight: the resting value is copied (§6.4). Emitted as given, not composited (C17, §6.1): the resting fill is transparent, so the hover fill has no expander fill to sit on — the row-highlight case |
| `expander.arrow_color` | `expander_icon(t)` + `CollapsingHeader::icon` (`collapsing_header.rs:480`), per call | DERIVED | in the stock path arrow and label share one field **inside one widget**: `paint_default_icon` fills the arrow with the state's `fg_stroke.color` (`:337`, `:353`) and the label is painted in the same state's `text_color()` (`:559`, `:598`), so no scope separates them. The icon closure does: `expander_icon` (§4.7) runs egui's own public `paint_default_icon` (`:336`) with `arrow_color` in the `fg_stroke` it reads, and the label keeps `font.color`, because `show` copied its `visuals` before calling the icon (`:559`, `:592`). `soft_option`: `None` → egui's own icon, in the label colour. Real values: `macos-sonoma.toml:336` gives the arrow `#86868b` against `#1d1d1f` text; `windows-11.toml:371` a semi-transparent `#1a1a1ae0` |
| `expander.font.family` | — | UNMAPPABLE `egui-limited` | as `window.title_bar_font.family` (§5.2); declined, §5.8 item 7 |
| `expander.font.size` | `text_styles[Button].size` «Expander» | SCOPED | `collapsing_header.rs:518-523` selects `TextStyle::Button` explicitly; × text-scaling factor (§4) |
| `expander.font.defined_size` | — | UNMAPPABLE `egui-limited` | as `defaults.font.defined_size` (§5.1) |
| `expander.font.weight` | `role_font_weight(t, Role::Expander)` + `RichText::variation` (`widget_text.rs:200-205`), per call | DERIVED | `FontId` has no weight field (`epaint/src/text/fonts.rs:21-28`); the accessor (§4.7) gives the CSS weight the application passes as the `wght` coordinate, which renders on a face with that axis and is ignored on a static one (§8.3). §5.8 item 2 |
| `expander.font.style` | `role_font_is_italic(t, Role::Expander)` + `RichText::italics` (`widget_text.rs:282-287`), per call | DERIVED | `Style` has no italic; `RichText::italics` is egui's fixed shear of the upright face, not the family's italic face (§8.4), so an italic leaf is approximated in glyph shape. No shipped preset states a style, so every one resolves `Normal` today (§8.4). §5.8 item 2 |
| `expander.font.color` | `widgets.{5}.fg_stroke.color` «Expander» | SCOPED | the label (`collapsing_header.rs:598`), and the arrow too wherever the application passes no `expander_icon` |
| `expander.border.color` | `widgets.{5}.bg_stroke.color` «Expander» | SCOPED | `collapsing_header.rs:566`, only when `collapsing_header_frame` is set |
| `expander.border.corner_radius` | `widgets.{5}.corner_radius` «Expander» | SCOPED | `collapsing_header.rs:564` |
| `expander.border.line_width` | `widgets.{5}.bg_stroke.width` «Expander» | SCOPED | `collapsing_header.rs:566`, painted `StrokeKind::Inside` (`:567`) |
| `expander.border.shadow_enabled` | — | UNMAPPABLE `source-side gap` | a bare `RectShape`, no `Frame` in the path (`collapsing_header.rs:562-568`); as `list.border.shadow_enabled`, a `Frame` the application lays round the widget paints a shadow under it (`frame.rs:140`, `:303`): what is missing is the geometry (§14 item 10) |
| `expander.border.padding.top` | `spacing.button_padding.y` «Expander» | DERIVED | one component shared with `.bottom`, the mean of the two targets under the pair rule above, a stated side counting with `expander.border.line_width` added — with both sides stated, `(top + bottom) / 2 + line_width`. Lossy only in centring: the header is `galley + 2.0 * button_padding.y` high (`collapsing_header.rs:531`) |
| `expander.border.padding.right` | `spacing.button_padding.x` «Expander» | DERIVED | the trailing pad (`collapsing_header.rs:526`); the header reads `button_padding.x` nowhere else, so this side is exact, and DERIVED only because the value written adds the line width (§5 intro). Written as `convert::padding_with_border(&t.expander.border).right` — the side plus the border's line width, as on every `button_padding` row — or, where that is `None`, egui's own `.x`, never `0`; floored at `0.0` by `clamp_length`, because a `NaN` in a `Vec2` trips emath 0.36.2's comparison assert (§7.4); `.y` comes from `convert::to_button_padding` |
| `expander.border.padding.bottom` | `spacing.button_padding.y` «Expander» | DERIVED | as `.top` |
| `expander.border.padding.left` | — | UNMAPPABLE `egui-limited` | the header has no leading padding: the label starts at `Spacing::indent` (`collapsing_header.rs:516`), the arrow column (`:587`), which is not a padding and has no native leaf (see above) |

No chrome row is DIRECT. The three plausible candidates were tested
individually and none survives — two are contested and one needs a formula: `toolbar.item_gap` → `item_spacing.x` is contested by
`dialog.button_gap` and `layout.widget_gap`; `expander.arrow_icon_size` →
`icon_width_inner` has no base owner (§5.9), and its one other claimant,
`checkbox.border.padding.*`, writes it in another scope, but it is nonetheless DERIVED, because the sink needs the 4/3 inversion of
egui's own `0.75` triangle scaling (§6.11); and
`status_bar.background_color` → `panel_fill` is contested by four other widgets. For chrome,
a scoped architecture is not a preference — it is the only architecture that
can carry the data.

### 5.7 Totals

The one set of totals for this mapping. At implementation they are generated
from `mapping.toml` (§13.1), whose rows carry the verdicts of §5.1–§5.6: one
generator writes `connectors/native-theme-egui/src/mapping.md` — the manifest's
rows, these totals and §5.9's table as the manifest records it (each base-style
field, the leaf that owns it unscoped, the leaves that only feed its formula or force
it to a constant, and the leaves that write the same field in a role scope or a
`Surface` frame, a frame's field listed under the `Style` field its egui preset
reads) — which the crate's rustdoc includes, and a
test fails when the committed file differs from what the generator writes. Every
other place in the crate's documents refers here instead of restating them.

| group | leaves | DIRECT | SCOPED | DERIVED | UNMAPPABLE |
|---|---:|---:|---:|---:|---:|
| Foundation (`defaults`, `text_scale`, `layout`) | 66 | 18 | 1 | 37 | 10 |
| Surfaces (window, dialog, popover, card, tooltip, menu) | 108 | 8 | 55 | 31 | 14 |
| Buttons (button, link, switch, checkbox, segmented control) | 99 | 11 | 45 | 29 | 14 |
| Inputs (input, combo box, list) | 78 | 4 | 35 | 28 | 11 |
| Indicators (scrollbar, slider, progress bar, splitter, separator, spinner) | 40 | 6 | 17 | 8 | 9 |
| Chrome (tab, sidebar, toolbar, status bar, expander) | 91 | 0 | 53 | 22 | 16 |
| **TOTAL** | **482** | **47** | **206** | **155** | **74** |

Row check: `18+1+37+10 = 66`; `8+55+31+14 = 108`; `11+45+29+14 = 99`;
`4+35+28+11 = 78`; `6+17+8+9 = 40`; `0+53+22+16 = 91`.
Column check: `47+206+155+74 = 482` and `66+108+99+78+40+91 = 482` — the 478
`ResolvedTheme` leaves and the four `LayoutTheme` fields.

The 74 UNMAPPABLE leaves carry three of the four sub-tags of §2, in the
proportions `15 + 5 + 54 = 74`. No row is `source-void`: every leaf of the model
carries a value of its own.

**15 are `source-side gap`**. Fourteen are `.border.shadow_enabled` leaves:
`sidebar`, `toolbar`, `status_bar`, `card`, `list`, `input`, `button`,
`checkbox`, `segmented_control`, `combo_box`, `progress_bar`, `tab`,
`expander` and `menu` (its items). Each has an `egui::Frame` that could carry
a shadow — a `Surface` frame (`Frame.shadow`, `containers/frame.rs:140`,
`:303`), the application's list `Frame`, a custom `TextEdit::frame`, or a
`Frame` the application lays round the widget — and native-theme has a real boolean, but
`ResolvedWidgetBorder` supplies no offset, blur or spread to go with it (§14
item 10), and none of those frames has a shadow of egui's own whose geometry
§6.14 could keep — neither side alone is at fault. The fifteenth is
`defaults.selection_inactive_background`: the install plugin could swap
`selection.bg_fill` for it while the window is unfocused, but `ResolvedTheme`
has no text colour to pair with the unemphasised fill (§5.1, §14 item 45).

**5 are `widgets-crate`**: `switch.thumb_background`, `thumb_diameter` and
`disabled_thumb_color`, `slider.disabled_thumb_color` and
`spinner.stroke_width`. egui's stock widget has no route for any of them, and
the companion crate's `Switch`, `Slider` and `Spinner` paint all five
(`docs/todo_egui-widgets-spec.md` §4.1–§4.3). `defaults.disabled_text_color` is
not among them, though the companion crate paints it: it paints it only as the
switch label's fallback for a text colour `SwitchTheme` lacks, and the leaf
itself reaches the screen through the seven per-widget leaves that inherit it
(§5.1).

The remaining **54 are `egui-limited`**, in five groups, `21 + 19 + 1 + 6 + 7 =
54`:

* 21 are the `*.defined_size` leaves: each states the unit a size came in, and
  the size itself reaches egui through the `.size` row beside it (§5.1).
* 19 are the per-widget `.family` leaves: each renders correctly whenever it
  equals the default it inherits, which it does on every bundled preset (§5.8
  item 7).
* 1 is `defaults.font.color`, rendered correctly on the same terms (§5.1).
* 6 are inheritance sources with no egui field of their own —
  `defaults.accent_color`, `accent_text_color`, `surface_color`,
  `disabled_text_color`, `text_selection_background` and
  `text_selection_color` — each rendered through the widget leaves that
  inherit it (§5.1).
* 7 are values the stock widget has no place for: `slider.tick_mark_length`
  (the slider draws no ticks, §5.5), `progress_bar.border.color` and
  `.line_width` (a `Frame` laid round the bar could carry them; declined, §5.8
  item 8), `progress_bar.border.padding.{top,right,bottom}` (the bar has no
  such inset, §5.5) and `expander.border.padding.left` (the header has no leading padding,
  §5.6).

### 5.8 Routes this crate declines

In eight places egui offers a route that this crate deliberately does not take.
Each is listed with its leaves, the route declined and the one taken instead;
the rows of §5.1–§5.6 grade the route taken, and every row a decline touches
says so.

| # | Leaves | Route declined | What this crate does instead, and why |
|---|---:|---|---|
| 1 | `defaults.focus_ring_color`, `defaults.focus_ring_width`, `defaults.focus_ring_offset` (3) | `widgets.active.bg_stroke` for `.color` and `.width` | egui makes focus and press the same state — `is_pointer_button_down_on() \|\| has_focus() \|\| clicked()` (`widget_style.rs:107-109`) — so a ring written there would appear on **every mouse press** *and* displace the role's real pressed border. The install plugin paints the ring itself instead, at the end of every pass, round the focused widget, while the window has OS focus (§5.1, §4.3). A button takes focus from keyboard navigation, not from a click: on an egui 0.36.2 `Context` the plugin's ring appears 0 times at rest, 0 times during and after a mouse click, and once after Tab. All three leaves reach the screen, DERIVED because the ring needs a live `Context`; the ring's radius is the nearest enclosing scope's (§5.1, §6.18) |
| 2 | per-widget `*.font.weight` and `*.font.style` — every one of the 19 per-widget `ResolvedFontSpec` slots of §5.2–§5.6 (38) | a registered `FontFamily::Name` per `(family, weight, style)` triple | §8.1 forbids emitting `FontFamily::Name`, for reasons that are structural rather than aesthetic. The route taken is per call, like item 3's: `role_font_weight(t, role)` (§4.7) gives the CSS weight the application passes to `RichText::variation(..)` (`widget_text.rs:200-205`), which renders on a face with a `wght` axis (§8.3), and `role_font_is_italic(t, role)` says when to call `RichText::italics()`, egui's fixed shear rather than the family's italic face (§8.4) — lossy in glyph shape only, and vacuous today because no shipped preset states a style. No preset states a per-widget `font` weight either; the weighted per-widget fonts are the title fonts — adwaita 700 for `window.title_bar_font` and 800 for `dialog.title_font` (`native-theme/src/presets/adwaita.toml:83`, `:277`), macos-sonoma 700 for both (`native-theme/src/presets/macos-sonoma.toml:345`, `:274`) and windows-11 600 for the dialog title (`native-theme/src/presets/windows-11.toml:303`), and the `-live` variants of the first two for the dialog title — so the weight route is live today. A `Role` names one font per widget, so for the second font of `dialog` (`title_font`) and of `list` (`header_font`) the application reads the two leaves from `ResolvedTheme` into the same calls. All 38 rows are DERIVED |
| 3 | `text_scale.{caption,section_heading,dialog_title,display}.weight` (4) | a registered `wght` family per role | Same cause as item 2. `text_role_weight()` tells the application which CSS weight to pass to `RichText::variation`; nothing is installed into any `Style`. DERIVED |
| 4 | `text_scale.dialog_title.size`, `text_scale.display.size` (2) | `TextStyle::Name("nt-dialog-title")` / `Name("nt-display")` keys | `text_role_font()` + `RichText::font(..)` (`widget_text.rs:190-198`). No `TextStyle::Name` key is added, which removes an entire class of panic: `TextStyle::resolve` calls `panic!` on a missing key (`style.rs:112-120`) and sits on the hot path of essentially every widget. A `FontId` handed out by an accessor performs no map lookup and therefore has no panic path. DERIVED |
| 5 | `list.header_font.size` (1) | a `TextStyle::Name("native-theme::list-header")` key | Same reason as item 4: no `Name` key is added. The leaf reaches the header text through `list_header_font()` (§4.7) + `RichText::font(..)`, per call, and loses the global `text_styles[Heading]` to `text_scale.section_heading` (§5.9). DERIVED; with the key it would have been DIRECT, since a size needs no font bytes (§8.2). `list.header_font.family` is item 7's |
| 6 | `dialog.max_width`, `dialog.max_height` (2) | `spacing.default_area_size` | `default_area_size` is read from `ctx.global_style()` (`area.rs:470`), so no scope reaches it, and it is the first-frame size of **every** free `Area` (`area.rs:470-476`; `Area::default_size` defaults to `Vec2::NAN` at `:145`) — window, popup, menu, tooltip — not a dialog constraint; writing it globally would resize all of them from a dialog metric. The application reads the two leaves from `ResolvedTheme` into `Ui::set_max_size` inside the `Modal::show` closure (§5.2). DERIVED |
| 7 | the `.family` of every per-widget `ResolvedFontSpec` slot of §5.2–§5.6 (19) | a `FontFamily::Name` per family (`epaint/src/text/fonts.rs:94`), named by a `TextStyle` key or a `FontId` | §8.1 forbids emitting `FontFamily::Name`, so every `FontId` this crate installs or hands out names `Proportional` or `Monospace` (`epaint/src/text/fonts.rs:79`, `:84`), and the font plan puts `defaults.font`'s face at the head of `Proportional` and `defaults.mono_font`'s at the head of `Monospace` (§8.2). Every per-widget text therefore renders in `defaults.font.family`, which is correct exactly when the leaf equals it — as it does on every bundled preset in both modes: no preset states a per-widget family, and each of the 19 inherits `defaults.font.family` (`docs/inheritance-rules.toml:101-136`). A live reader can state one of its own — KDE's `menuFont`, `toolBarFont` and `activeFont` (`native-theme/src/kde/fonts.rs:135`, `:141`, `:149-150`), Windows' caption, menu and status fonts (`native-theme/src/windows.rs:470-472`), macOS's menu, tooltip and title-bar fonts (`native-theme/src/macos.rs:375-377`), GNOME's title-bar font (`native-theme/src/gnome/mod.rs:250`) — and a family that differs from the default is lost there (§14 item 8). All 19 rows are UNMAPPABLE `egui-limited`: the verdict records what this crate carries (§2) |
| 8 | `progress_bar.border.color`, `progress_bar.border.line_width` (2) | `Frame::stroke` on a `Frame` the application lays round the bar (`frame.rs:116`, `:267`) | nothing: `docs/platform-facts.md` §2.10 states no progress-bar border, so the value is `defaults.border`'s by inheritance (`docs/inheritance-rules.toml:86-93`), not a platform's. Stroked, it would outline the bar on every preset, where WinUI 3 and libadwaita draw no outline outside high contrast (microsoft-ui-xaml `258a2e9b`, `controls/dev/ProgressBar/ProgressBar_themeresources.xaml` lines 5 and 21; libadwaita 1.10.0 `_scale.scss:1-10`, which `_progress-bar.scss`'s `> trough` extends). KDE's Breeze does stroke the groove, with a 1.001 px pen of the window text colour at its frame intensity, which over the window is its frame outline colour (breeze master, `kstyle/breezehelper.cpp` lines 134–139, 144 and 1204–1219, `kstyle/breezestyle.cpp` lines 6302–6303, `kstyle/breezemetrics.h` line 25); native-theme does not record it (`docs/todo.md`). UNMAPPABLE `egui-limited` |

### 5.9 The base-owner table

A contested field always means somebody loses, and **the loser must be nameable
by a reader who never opens `mapping.toml`**. This table is what lets a user
predict what an *unscoped* widget looks like. Its owners and its scope and frame displacements are published in the crate's
rustdoc through the generated `src/mapping.md` (§5.7), derived from the
manifest's sinks; the accessor displacements and the explanations below live
here and in the accessors' rustdoc (§4.7), since an accessor route is a
`tested_by` row with no sink to derive them from (§13.1).

| egui field | base owner (wins globally) | displaced to a `Role` scope, a `Surface` frame or an accessor |
|---|---|---|
| `visuals.panel_fill` (`style.rs:1072`) | `defaults.background_color` | `sidebar`/`toolbar`/`status_bar`/`window` `.background_color`, `tab.bar_background` |
| `visuals.window_fill` (`:1063`) | `menu.background_color` — every menu, context menu and `ComboBox` list is a `Frame::menu`/`Frame::popup` (`frame.rs:210`, `:219`; `containers/menu.rs:432`, `containers/popup.rs:603`), and `Response::context_menu` offers no scope at all (§5.2), while a `Window` takes `Surface::Window`. No shipped preset states it or `window.background_color`, so the two are equal by inheritance (`docs/inheritance-rules.toml:149`, `:180`); the Windows reader states it (`native-theme/src/windows.rs:312`) | `window`/`popover`/`tooltip`/`dialog` `.background_color`, via `Surface` frames (`defaults.surface_color` reaches no egui field, §5.1) |
| `visuals.extreme_bg_color` (`:1045`) | `scrollbar.track_color` | `progress_bar.track_color` (`input.background_color` is removed from the contest by §5.4) |
| `visuals.faint_bg_color` (`:1040`) | `list.alternate_row_background` | — |
| `visuals.widgets.noninteractive.fg_stroke.color` (`:1311`) | `defaults.text_color` | the other claimants (§5.11): the text colours of `dialog`, `popover`, `tooltip` and `list` cells and the `font.color` of ten roles through their state sets (§6.1), `checkbox.indicator_color` through its `{5}` set — inheriting `defaults.accent_text_color`, not `text_color` (`docs/inheritance-rules.toml:175`) — the `Disabled`-cell `disabled_text_color` of `button`, `combo_box`, `input`, `menu` and `list`, `defaults.font.color`, which no scope carries (§5.1), and `dialog.title_font.color`, which goes per call (§5.2); `defaults.mono_font.color` goes to the monospace text's own format, per call (§5.1) |
| `visuals.widgets.noninteractive.{bg_fill,weak_bg_fill}` (`:1295`, `:1300`) | `defaults.background_color` — egui's own pairing of the non-interactive entry with the panel fill (§5.1) | `checkbox.checked_background`, whose `Selected` cell writes `bg_fill` in all five entries (§6.2) |
| `visuals.widgets.{inactive,hovered,active,open}.bg_fill` (`:1295`) | `scrollbar.thumb_color` (`inactive`, and `open` as §6.1 copies it) / `scrollbar.thumb_hover_color` (`hovered`) / `scrollbar.thumb_active_color` (`active`, D5's fallback included, §6.9) — every `ScrollArea` handle reads it (`scroll_area.rs:1457-1466`, `:1499-1503`), the ones egui builds itself in a `Window` and a `ComboBox` list included (`containers/window.rs:738-742`, `containers/combo_box.rs:403`), and the bars are painted by the outer `Ui`, so no `Role::Scrollbar` scope could reach them without restyling the scrolled content; `theme.button` has no `bg_fill` of its own, since a `Button` fills from `weak_bg_fill` (B1) | the other claimants (§5.11): the `checkbox` fills, `slider.track_color` and thumb colours and `list.hover_background`, each in its own scope. Every reader no scope carries takes the base value, where egui's own is its stock grey, commented "checkbox background" (`style.rs:1693`, `:1738`): an unscoped `Checkbox`'s or `RadioButton`'s box, a `Slider`'s rail (`slider.rs:775`) and handle (`:816`, `:830`), the colour button's outline in every state (`widgets/color_picker.rs:116-130`), and `dnd_drop_zone`'s frame fill (`ui.rs:2721`; the showcase's is unscoped, §10.4). Inside `Role::List`, an `egui_extras` `Table`'s own `ScrollArea` (`egui_extras/src/table.rs:739`, shown in the table's `Ui` at `:760`) hovers and presses in `list.hover_background`. The cost: no bundled preset states `scrollbar.thumb_color`, which then resolves to `defaults.muted_color` (`docs/inheritance-rules.toml:191`), a fallback the same file lists as a wrong safety net (`:468-470`). An unscoped checkbox's box takes that colour at rest, and its check mark the button text colour (`check_stroke`, the `fg_stroke`: `widget_style.rs:131`, `:188`; `widgets/checkbox.rs:157`), which falls to about 2.3:1 contrast on kde-breeze dark (`#fcfcfc` on `#a1a9b1`) and 3.1:1 on adwaita dark (`#ffffff` on `#929292`). The remedy for a checkbox is its role scope, `Role::Checkbox` |
| `visuals.widgets.inactive.*` (`:1295-1319`) | `theme.button` (§6.1), its border colour, width and radius included — every field but `bg_fill` (row above) | the other widgets' resting colours; per-field claimant counts in §5.11 |
| `visuals.widgets.hovered.*` | `button.hover_background` (composited over `button.background_color`, C17) / `button.hover_text_color` — every field but `bg_fill` (row above) | the other widgets' hover colours; per-field claimant counts in §5.11 |
| `visuals.widgets.active.*` | `button.active_background` (composited like the hover fill) — every field but `bg_fill` (row above) and `fg_stroke.color`, whose owner is `defaults.text_color`: egui reads that field as `strong_text_color()` (`style.rs:1147-1149`), the colour of all strong text, so the panel's text colour wins it (§5.1, §5.3) | `button.active_text_color` «Button», slider and spinner pressed colours, `list.header_font.color` via `strong_text_color()`, and the hover values of every role that states no pressed one, copied (§6.1) |
| `visuals.widgets.open.weak_bg_fill` (`:1300`) | `window.title_bar_background` | `combo_box.background_color` (the open trigger, `Role::ComboBox`), `menu.hover_background` (the item whose submenu is open, `Role::Menu`), and the resting fills of `button`, `tab`, `segmented_control` and `switch`, copied from `inactive` in their own scopes (§6.1); the colour picker's open button has no native leaf |
| `visuals.selection.bg_fill` (`:1196`) | `defaults.selection_background` | the other claimants (§5.11), including `slider.disabled_fill_color`, `switch.disabled_checked_background` and `button.disabled_background` in `Disabled` cells, `progress_bar.fill_color`, `slider.fill_color`, `tab.active_background`, `switch.checked_background`, `button.primary_background` |
| `visuals.selection.stroke.color` (`:1199`) | `defaults.selection_text_color` | the other claimants (§5.11); `input.focus_border_color` reaches the focused field's stroke per call, through `input_frame`, and claims no shared field (§4.7, §5.4) |
| `visuals.weak_text_color` (`:1027`) | `defaults.muted_color` | `input.placeholder_color` |
| `visuals.hyperlink_color` (`:1036`) | `defaults.link_color`, the token every reader writes (e.g. `native-theme/src/kde/colors.rs:29`, `native-theme/src/macos.rs:108`) | `link.font.color`, which inherits it (`docs/inheritance-rules.toml:281`), and `link.disabled_text_color`, both in `Role::Link` |
| `visuals.error_fg_color` (`:1059`) | `defaults.danger_color` | — |
| `visuals.warn_fg_color` (`:1056`) | `defaults.warning_color` | — |
| `visuals.disabled_alpha` (`:1126`) | `defaults.disabled_opacity` | seven per-widget `disabled_opacity`s, each in its own role scope; a `Disabled` cell that carries the platform's disabled colours sets `1.0` there instead (§6.3) |
| `visuals.widgets.{inactive,hovered,active,open}.corner_radius` (`:1308`) | `button.border.corner_radius` — what every unscoped button, combo box, drag value and selectable draws with (§6.1), and the focus ring's default radius (§6.18) | the other claimants (§5.11): `switch.track_radius` and the `border.corner_radius` of `checkbox`, `segmented_control`, `input`, `combo_box`, `tab`, `expander` (§5.11), and `menu.border.corner_radius` (its items, `Role::Menu`) |
| `visuals.widgets.noninteractive.corner_radius` (`:1308`) | `defaults.border.corner_radius` — frames, separators and group boxes | the other claimants (§5.11): `switch.track_radius` and the `border.corner_radius` of `button`, `checkbox`, `segmented_control`, `input`, `combo_box`, `list`, `card`, `tab`, `expander`, each in its own scope (§5.11) |
| `visuals.window_corner_radius` (`:1061`) | `window.border.corner_radius` — already the *large* radius, because `ResolvedWindowTheme` is `border_kind = "full_lg"` (`native-theme/src/model/widgets/mod.rs:14`) | — (`Frame::window` `frame.rs:199` and the resize corner `window.rs:1230` are its only consumers) |
| `visuals.menu_corner_radius` (`:1069`) | `defaults.border.corner_radius_lg` — also the menu's frame (`menu.border` is the item's, `docs/platform-facts.md:1239`) | `popover`/`dialog`/`tooltip` `border.corner_radius`; vacuous for `popover` and `dialog`, which inherit `corner_radius_lg` by construction (`border_kind = "full_lg"`, `native-theme/src/model/widgets/mod.rs:547`, `:650`), but not for `tooltip` (`"full"`, the ordinary radius), §5.2 |
| `visuals.widgets.{inactive,hovered,active,open}.bg_stroke` (`:1305`) | `button.border.{color,line_width}` — the outline of every unscoped interactive control (§6.1) | the other claimants (§5.11): `checkbox.border.*`, `checkbox.unchecked_border_color`, `segmented_control.border.*`, `input.border.*`, `input.hover_border_color`, `combo_box.border.*`, `tab.border.*`, `expander.border.*` (§5.11), and `menu.border.*` (its items) |
| `visuals.widgets.noninteractive.bg_stroke` (`:1305`) | `defaults.border.{color,line_width}` — frames, separators and group boxes | the other claimants (§5.11) |
| `visuals.window_stroke` (`:1064`) | `defaults.border.{color,line_width}` — also the menu's frame, whose border `docs/platform-facts.md:1237-1238` gives to the popup of its §2.16 | the borders of `window`, `dialog`, `popover` and `tooltip`, via `Surface` frames |
| `visuals.{window,popup}_shadow.color` (`:1062`, `:1074`) | `defaults.shadow_color` | — (geometry stays egui's, §14 item 10) |
| `visuals.window_shadow` on/off gate (§6.14) | `window.border.shadow_enabled` — the window's own frame, as for `window_corner_radius` and `window_margin` | `defaults.border.shadow_enabled`, which it inherits (`docs/inheritance-rules.toml:94`) |
| `visuals.popup_shadow` on/off gate (§6.14) | `defaults.border.shadow_enabled` — a `border_kind = "none"` menu resolves `false` when unstated (`validate_helpers.rs:288`), which would strip every unscoped popup of the shadow the platforms give it | `dialog`/`popover`/`tooltip` `border.shadow_enabled`, via `Surface` frames |
| `visuals.text_cursor.stroke.color` (`:953`) | `input.caret_color` | — (width stays egui's `2.0`; no native caret width exists). The struct's other four fields are also left alone: `preview` `false` (`style.rs:956`, `:972`), `blink` `true` (`:959`, `:973`), `on_duration` and `off_duration` `0.5` (`:962`, `:965`, `:974-975`) — caret preview and blink rate are input behaviour with no native leaf, read at `text_selection/visuals.rs:297-299` and `widgets/text_edit/builder.rs:795` |
| `visuals.slider_trailing_fill` (`:1105`) | `slider.fill_color` forces it `true` | — (`Slider` is its only reader, `slider.rs:783`) |
| `visuals.handle_shape` (`:1110`, enum `HandleShape` at `:1235-1244`) | `HandleShape::Circle`, implied by `slider.thumb_diameter` being a *diameter* | — (`Slider` is its only reader, `slider.rs:663`, `:810`, `:996`) |
| `spacing.interact_size.y` (`:409`) | `button.min_height` | the other claimants (§5.11) — the most contested `Spacing` field in egui |
| `spacing.button_padding` (`:398`) | `button.border.padding.{top,right,bottom,left}` — one `Vec2`, so a side pair shares a component (§5 intro) | the other claimants (§5.11) — `tab`, `expander`, `combo_box`, `segmented_control`, and `menu`, whose item padding the `Role::Menu` cell writes over `menu_style`'s `vec2(2.0, 0.0)` (`menu.rs:23`) |
| `spacing.icon_width` (`:428`) | `checkbox.indicator_width` (§5.3) | `combo_box.arrow_icon_size` |
| `spacing.icon_width_inner` (`:432`) | **nobody — left at egui's `8.0`** | `expander.arrow_icon_size`, in the expander scope only, and `checkbox.border.padding.*`, in the checkbox scope only (§6.11) |
| `spacing.icon_spacing` (`:436`) | `checkbox.label_gap` — every `Checkbox` and `RadioButton` lays out its box beside its label, so it paints the gap (`atom_layout.rs:302`, `:346-348`; the box atom at `widgets/checkbox.rs:92`, `widgets/radio_button.rs:61`), while a text-only `Button` has one atom and none | `button.icon_text_gap`, `menu.icon_text_gap`, `combo_box.arrow_area_width` |
| `spacing.slider_rail_height` (`:415`) | `slider.track_height` | — (exact match, `slider.rs:770`) |
| `spacing.combo_width` (`:418`) | `combo_box.min_width` | — |
| `spacing.combo_height` (`:462`) | **nobody — left at egui's `200.0`** (`style.rs:1473`) | — (no native leaf states a combo-box maximum height) |
| `spacing.tooltip_width` (`:448`) | `tooltip.max_width` | — |
| `spacing.text_edit_width` (`:421`) | **nobody — left at egui's `280.0`** (`style.rs:1464`) | — (no native leaf states a text-edit default width) |
| `spacing.menu_margin` (`:401`) | **nobody — left at egui's `Margin::same(6)`** (`style.rs:1457`): native-theme states no menu-container padding — `docs/platform-facts.md:1235-1236` gives its §2.6 `border.padding` to the item, carried in the `Role::Menu` cell's `button_padding` (§5.2) | `dialog`/`popover`/`tooltip` paddings, via `Surface` frames |
| `spacing.window_margin` (`:395`) | `window.border.padding.{top,right,bottom,left}` | — |
| `spacing.item_spacing` (`:392`) | `layout.widget_gap`, when the supplied `LayoutTheme` states it | `toolbar.item_gap`, `dialog.button_gap`, `progress_bar.border.padding.left`, `segmented_control.separator_width` |
| `spacing.scroll.{bar_width,handle_min_length,floating}` (`:512`, `:515`, `:503`) | `scrollbar.{groove_width,min_thumb_length,overlay_mode}` | — |
| `spacing.scroll.foreground_color` (`:538`) | forced `false` so the handle reads `bg_fill` | — |
| `spacing.scroll.{dormant,active,interact}_{background,handle}_opacity` (`:545`, `:552`, `:559`, `:566`, `:573`, `:580`) | **nobody — left at egui's `ScrollStyle::floating()` values**: background `0.0`/`0.4`/`0.7` and handle `0.0`/`0.6`/`1.0` (`style.rs:649`, `:607-608`, `:650`, `:611-612`) | — (no native leaf describes an auto-hide fade curve; `scrollbar.overlay_mode` is a bool, `docs/platform-facts.md:1001`). They modulate the floating branch only (`scroll_area.rs:1469-1497`, gamma-multiplied at `:1506-1519`), so on a preset with `overlay_mode = true` the four mapped scrollbar colours are painted fully transparent at rest — see §6.5 |
| `spacing.slider_width` (`:412`) | **nobody — left at egui's `100.0`** (`style.rs:1461`) | — (no native leaf states a slider length; egui's own colour picker overrides it to `275.0` inside its popup, `widgets/color_picker.rs:526`, `:532`) |
| `spacing.default_area_size` (`:445`) | **nobody — left at egui's `vec2(600.0, 400.0)`** (§5.8 item 6) | `dialog.max_width` / `max_height`, which the application reads from `ResolvedTheme` (§5.2) |
| `text_styles[Small]` (`:74`) | `text_scale.caption` | — |
| `text_styles[Body]` (`:77`) | `defaults.font` | the other claimants (§5.11), `menu`, `tab`, `sidebar` and `toolbar` among them because their `Button`s read `Body` (B3) |
| `text_styles[Monospace]` (`:80`) | `defaults.mono_font` | — (the only uncontested text style) |
| `text_styles[Button]` (`:85`) | `button.font` — read by `ComboBox` (`combo_box.rs:358`), `ProgressBar` (`progress_bar.rs:193`), `CollapsingHeader` (`collapsing_header.rs:522`) and `DragValue` through `Style::drag_value_text_style` (`style.rs:1434`), but **not** by `Button` itself (B3) | two others, `combo_box.font` and `expander.font` (`menu`, `tab`, `sidebar` and `toolbar` fonts sit on `text_styles[Body]`, and `checkbox` and `segmented_control` fonts on `override_font_id`, §5.3) |
| `text_styles[Heading]` (`:88`) | `text_scale.section_heading` | `text_scale.dialog_title`, `window.title_bar_font`, `dialog.title_font`, `list.header_font` |
| `override_font_id` (`:255`) | **unset on the base style**; written in the `Button`, `Link`, `Checkbox` and `SegmentedControl` scopes, whose rows name it. A scope that holds headings must not write it, because it outranks `RichText::heading` (B3) | — |
| `override_text_color` (`:1016`) | **unset on the base style**; written in the `Role::Checkbox` scope only — `checkbox.font.color` in the `Normal` and `Selected` cells, `checkbox.disabled_text_color` in the `Disabled` cell | — |
| `visuals.text_edit_bg_color` (`:1050`) | `input.background_color` | `input.disabled_background`, in the `Role::Input` `Disabled` cell |

**Text-on-fill pairs (C8).** Most text colour and fill pairs above are the
platform's own: a role scope writes a state's fill and its text colour from the
same widget's leaves, and the base style pairs `defaults.text_color` with
`defaults.background_color` on every panel. The C8 assertion —
never less readable than the platform's own pair — belongs on the pairs the
mapping **assembles** from leaves no platform put together, and every one is
named in its row:

* the title bar: `window.title_bar_background` and
  `window.inactive_title_bar_background` (§5.2) under title text that is
  `visuals.text_color()`, the base `defaults.text_color`, unless the
  application colours the title atoms per call with
  `window_title_bar_text_color` (`window.title_bar_font.color` and
  `window.inactive_title_bar_text_color`, §5.2);
* an unscoped `Window`, menu, popup or tooltip: base `defaults.text_color`
  labels over the base `window_fill`, `menu.background_color` (§5.9), and menu
  items in `button.font.color` over it;
* the checkbox label, kept off the mark's colour only by B4 (§5.3);
* a pressed widget inside a list cell, whose text is `list.header_font.color`
  over its own pressed fill (§5.4);
* a progress-bar label: the base `defaults.selection_text_color` over
  `progress_bar.fill_color` and `progress_bar.track_color` (§5.5);
* a `Switch` substitute's label, which native-theme has no colour for: when
  checked, `Style::button_style` paints a selected button's text in
  `selection.stroke.color` (`egui/src/widget_style.rs:154`), which the
  `Role::Switch` scope does not write, over `switch.checked_background`; when
  unchecked, the inherited `inactive.fg_stroke.color` over
  `switch.unchecked_background` (§5.3).

Hover and pressed fills are compared after the C17 composite (§6.1), which is
the colour the platform paints. Status colours are emitted as given, with no
contrast adjustment (C19, §5.1).

### 5.10 Fields this crate deliberately never writes

| field | decl | why |
|---|---|---|
| `Spacing::menu_width` | `style.rs:453` | **no reader anywhere in egui 0.36.2** — only the destructure at `:1962` and the settings UI at `:2032` |
| `Spacing::menu_spacing` | `:456` | same — `:1963`, `:2037` |
| `Style::compact_menu_style` | `:341` | same — `:1805`, `:1911` |
| `Visuals::clip_rect_margin` | `:1086-1087` | `#[deprecated]`, doc says "Setting it now has no effect" (`:1081-1085`) |
| `Visuals::window_highlight_topmost` | `:1067` | no consumer |
| `Visuals::striped` | `:1100` | gates striping entirely; native-theme has no "are lists striped" boolean, and turning it on because an alternate colour exists would be inventing platform policy |
| `Style::interaction` (all 8 fields) | `:911-945` | input-behaviour policy — hit-test slop, tooltip timing, text-selection policy. `ResolvedTheme` exposes none of it |
| `Visuals::override_text_color` on the **base** style | `:1016` | it forces one colour on *all* text; set globally it destroys every per-state and per-widget text colour, and it would turn a `ProgressBar` label into ordinary body text on an accent fill (`progress_bar.rs:197-199`) |
| `ScrollStyle::fade` (`ScrollFadeStyle`, `strength` + `size`) | `:582`, struct `:784-793` | left at egui's `strength 0.5` / `size 20.0` (`style.rs:795-802`). It is live on every scroll area — `paint_fade_areas_impl` is called unconditionally (`containers/scroll_area.rs:1275`, body `:1564-1568`, gradients `:1581-1593`, `:1596-1608`) — but **no platform fact records a scroll-edge fade**, so any value written here would be invented |
| `Visuals::ime_composition` (both underline strokes) | `:1033`, struct `:1206-1230` | native-theme carries no IME-composition colour, so writing these from `input.caret_color` would be a mapping with no source. egui initialises them from the `TextCursorStyle::stroke` defaults, with upstream's own comment saying so (`style.rs:1638-1665`); readers `text_selection/visuals.rs:168-169` and `widgets/text_edit/builder.rs:873`. The consequence for `input.caret_color` is recorded in §5.4 |
| `ImeComposition::legacy_visuals` | `:1229` | a `winit` workaround, not an appearance choice: it defaults to `cfg!(windows)` (`style.rs:1667-1671`) and switches between two IME rendering strategies (`:1213-1228`). Platform behaviour, not theme data |
| `Visuals::code_bg_color` | `:1053` | the background behind `RichText::code`; left at `from_gray(64)` dark / `from_gray(230)` light (`style.rs:1515`, `:1578`), sole painter `widget_text.rs:440-441`. `ResolvedTheme` has no code-block colour |
| `Visuals::indent_has_left_vline` | `:1096` | left at `true` (`style.rs:1548`). Read at `ui.rs:2260` and drawn at `:2274` with the stroke taken at `:2268` from `widgets.noninteractive.bg_stroke`, which the connector **does** write — so the rule already carries the theme's hairline colour and width. Whether an indented region is ruled at all is layout policy with no native leaf |
| `Spacing::indent_ends_with_horizontal_line` | `:459` | left at `false` (`style.rs:1475`); same read site (`ui.rs:2261`), drawn at `:2280-2281` from the same stroke, and the same reason |
| `Style::override_text_style` | `style.rs:249` | left at `None` (`style.rs:1431`). Read at `widget_text.rs:427`, it forces one `TextStyle` on all text that names none; native-theme has no such leaf, and the four scopes that need one font for everything write `override_font_id` instead (§5.3 B3) |
| `Style::override_text_valign` | `style.rs:260` | left at `Some(Align::Center)` (`style.rs:1432`), read at `ui.rs:612`: vertical placement of mixed text in a row is layout policy with no native leaf |
| `Style::wrap_mode` | `style.rs:306` | left at `None` (`style.rs:1436`), read at `ui.rs:590`: text-wrapping behaviour, not appearance data |
| `Style::number_formatter` | `style.rs:298` | left at egui's `emath::format_with_decimals_in_range` (`style.rs:1435`), read at `widgets/drag_value.rs:552`: number formatting, which native-theme does not model |
| `Style::explanation_tooltips` | `style.rs:329` | left at `false` (`style.rs:1443`); readers `ui.rs:1993`, `widgets/color_picker.rs:522`, `widgets/drag_value.rs:653`. Whether widgets explain themselves in tooltips is application behaviour |
| `Style::url_in_tooltip` | `style.rs:332` | left at `false` (`style.rs:1444`), read at `widgets/hyperlink.rs:144`; application behaviour |
| `Style::always_scroll_the_only_direction` | `style.rs:335` | left at `false` (`style.rs:1445`), read at `containers/scroll_area.rs:1220`; input behaviour |
| `Visuals::resize_corner_size` | `style.rs:1076` | left at `12.0` (`style.rs:1541`); readers `containers/resize.rs:371`, `containers/window.rs:859`. native-theme states no resize-grip size |
| `Visuals::interact_cursor` | `style.rs:1117` | left at `None` (`style.rs:1555`), read at `widgets/button.rs:385`. native-theme has no pointer-cursor leaf |
| `Visuals::image_loading_spinners` | `style.rs:1120` | left at `true` (`style.rs:1557`), read at `widgets/image.rs:673`; application behaviour |
| `Visuals::numeric_color_space` | `style.rs:1123` | left at `GammaByte` (`style.rs:1559`); readers `widgets/color_picker.rs:287`, `:408`. How a colour picker prints numbers is not theme data |
| `Visuals::text_options.font_hinting` | `epaint/src/text/mod.rs:39` | left at `true` (`epaint/src/text/mod.rs:61`), read at `epaint/src/text/font.rs:402`. Hinting is an operating-system text-rendering setting, and native-theme carries no leaf for it |
| `Visuals::text_options.subpixel_binning` | `epaint/src/text/mod.rs:53` | left at `true` (`epaint/src/text/mod.rs:62`), read at `epaint/src/text/font.rs:428`, `:622`: fractional horizontal glyph positions (`epaint/src/text/mod.rs:43-44`), not LCD sub-pixel rendering, and native-theme carries no leaf for it |

Two fields for which native-theme carries no value are nonetheless written,
from `AccessibilityPreferences` rather than from the theme: with
`reduce_motion` set, every style the atlas builds gets
`Style::animation_time = 0.0` (`egui/src/style.rs:318`) and
`Style::scroll_animation = ScrollAnimation::none()` (`:338`, `:858`) (§4);
without it both keep egui's own values (`:1440`, `:1446`). Neither is a
`ResolvedTheme` leaf, so neither adds a row to §5.1–§5.6.

Writing any of the first five is harmless but produces a silent no-op — which
is exactly the "plausible fabrication" failure mode this project forbids. The
nineteen rows after `override_text_color` are the converse case: they have
live readers and affect appearance or behaviour, and are listed so that "in no
bucket" never means "overlooked". Nothing is asserted about whether any of them
is *native* — no platform fact records a scroll-edge fade, an IME underline
colour, a code-block background, an indent rule, a resize-grip size or a text
hinting setting, and none may be invented.

### 5.11 The complete contested-field list

Every egui field claimed by two or more native-theme leaves, with a claimant
count. This is the evidence that the scoped architecture is **necessary, not
tasteful**.

This table is **generated from the sink column of §5.1–§5.6**: a leaf appears
here only where a per-widget row names that field as its sink. A leaf whose
per-widget row is UNMAPPABLE because egui offers no route to that field at all
is *not* a claimant, and neither is a leaf whose route writes no `Style` field:
the three focus-ring leaves, whose declined sink `widgets.active.bg_stroke`
(§5.8 item 1) gives way to the ring the install plugin paints, do not appear in the
`bg_stroke` row. A per-call route (§5 introduction) claims no shared field,
so `dialog.max_width` and `max_height`, whose declined sink is
`spacing.default_area_size` (§5.8 item 6), claim nothing here, and neither does
`expander.arrow_color`, which reaches its arrow through `expander_icon` (§5.6),
nor `list.border.color`, which reaches the list's outline on the application's
list `Frame` (§5.4), nor `input.focus_border_color`, which reaches the focused
field's stroke through `input_frame` (§5.4), nor `link.visited_text_color`,
`.hover_text_color`, `.active_text_color` and `.hover_background`,
`list.hover_text_color` and `defaults.mono_font.color`, which travel in the
text's own format (§5.1, §5.3, §5.4), nor `switch.hover_checked_background`,
handed to the substitute's `Button::fill` (§5.3).
Nor do the 19 per-widget `.family` leaves: every `FontId` names
`Proportional` or `Monospace` (§8.1), and the route that would carry a
family of their own, a `FontFamily::Name` key, is declined (§5.8 item 7), so
`defaults.font` and `defaults.mono_font` are the only families installed and
they hold different keys. Where a per-widget row and this list appear to disagree,
**the per-widget row governs.**

Eight conventions govern the numbers, stated once here instead of row by row:

* **`{…}` expands.** A sink written as a `{…}` state set claims the field in
  *every* state it names, so a `{5}` set reaches all five `Widgets` entries —
  `noninteractive`, `inactive`, `hovered`, `active`, `open` (`style.rs:1250-1270`)
  — and the leaf is a claimant of each per-state row below, not only of the row
  its widget is usually discussed under. Every row below was regenerated
  from the sink column of §5.1–§5.6 with this expansion applied, by a script
  over the tables; each count is exact for those tables.
* **The count is entries, not always leaves.** The claimant count is the number
  of entries in that row's list, and an entry may abbreviate a group:
  `X.border.*` stands for `{color, line_width}` and `X.border.padding.*` for
  the four sides. Where the distinction matters the row states both
  (`spacing.button_padding`, `spacing.menu_margin`,
  `visuals.widgets.<state>.bg_stroke`, `visuals.widgets.<state>.bg_fill`).
* **Shadows count once.** A `*_shadow.color` write and a `*_shadow` on/off gate
  are claims on the same egui field, so `defaults.shadow_color` and the
  `border.shadow_enabled` leaves appear together in one row per shadow field.
* **A `RoleVariant` cell claims like any other.** `checkbox.checked_background`
  in the `Selected` cell and `button.disabled_text_color` in the `Disabled` cell
  claim their fields exactly as a `Normal` write does; such a pair is resolved
  by the variant, so only the intra-widget contest below shares a cell.
* **DERIVED does not remove a claim.** A leaf that needs a formula or font bytes
  still occupies its sink and is counted exactly like a SCOPED one; only a sink
  §5.8 declines drops out, as above.
* **Losing a field does not remove the claim either.** A leaf whose
  route became an accessor plus a per-call mechanism *because* it lost that
  field to its base owner or to a sibling leaf in the same scope (§5.8, §5.9)
  is still counted as a claimant of the field it lost — `dialog.title_font.color`,
  which lost the `Dialog` scope's text colour to `dialog.body_font.color`
  (§5.2) — and so is a leaf left UNMAPPABLE by that loss —
  `defaults.font.color` and the two `defaults.text_selection_*` leaves
  (§5.1) — though its
  sink column is empty: that loss is the contest, and hiding it would make the
  contest disappear from the evidence. `text_styles[Heading]` is the row where
  this matters: `window.title_bar_font` is routed through `RichText::font` on
  the title atoms (§5.2) and `list.header_font` through §5.8 item 5, yet both
  are counted, because both wanted `Heading` and lost it to
  `text_scale.section_heading`.
* **Copied entries are counted from the state sets.** §6.1 copies `inactive`
  into `open` (`hovered` in `Role::Menu`), and `hovered` into `active` where a
  role states no pressed value, and every row's state set names the entries
  those copies reach; a per-state row lists exactly the leaves whose own row
  names that state.
* **A `soft_option` fallback is a route, not a claim.** Where a hover or
  pressed leaf is `None`, §6.4 copies the base-state value into that entry
  (C16). The copy happens only when the other leaf is unstated, so it can
  decide a row's grade (`slider.thumb_color`, §5.5) but it does not make the
  base-state leaf a claimant of the entry it fills.

| egui field | decl | claimants |
|---|---|---|
| `spacing.interact_size.y` | `style.rs:409` | 13: `button.min_height`, `combo_box.min_height`, `menu.row_height`, `tab.min_height`, `list.row_height`, `expander.header_height`, `toolbar.bar_height`, `segmented_control.segment_height`, `switch.track_height`, `slider.thumb_diameter` (via §6.6), `progress_bar.track_height`, `spinner.diameter`, `spinner.min_diameter`. `input.min_height` is **not** among them: `TextEdit` never reads `interact_size` (§5.4) |
| `visuals.widgets.<state>.corner_radius` | `style.rs:1308` | 12: `defaults.border.corner_radius`, `switch.track_radius`, and the `border.corner_radius` of `button`, `checkbox`, `segmented_control`, `input`, `combo_box`, `list`, `card`, `tab`, `expander` and `menu` (its items, §5.2). The other eight widget radii reach `visuals.window_corner_radius` (`window`), `visuals.menu_corner_radius` (`dialog`, `popover`, `tooltip`), an `egui::Frame` (`sidebar`, `toolbar`, `status_bar`) or a per-call builder (`progress_bar`, §5.5) |
| `visuals.widgets.<state>.bg_stroke` (colour and width) | `style.rs:1305` | 20 entries, 35 leaves: `defaults.border.{color,line_width}`, `separator.{line_color,line_width}`, `splitter.{divider_color,divider_width}`, `menu.separator_color`, `list.grid_color`, `list.border.line_width`, `card.border.*`, `sidebar.border.*`, `status_bar.border.*`, `toolbar.border.*`, `tab.border.*`, `expander.border.*`, `button.border.*`, `checkbox.border.*`, `checkbox.unchecked_border_color`, `segmented_control.border.*`, `input.border.*`, `input.hover_border_color`, `combo_box.border.*`, `menu.border.*` (its items, §5.2). `window.border.*` is **not** among them — it reaches `visuals.window_stroke` (§5.2) |
| `visuals.widgets.inactive.fg_stroke.color` | `style.rs:1311` | 16: `tab.font.color`, `expander.font.color`, `sidebar.font.color`, `toolbar.font.color`, `status_bar.font.color`, `button.font.color`, `combo_box.font.color`, `menu.font.color`, `segmented_control.font.color`, `input.font.color`, `checkbox.indicator_color`, which reaches it via a `{5}` state set, and the `Disabled`-cell `disabled_text_color` of `button`, `input`, `menu`, `combo_box` and `checkbox` (its mark, §6.3). `expander.arrow_color` is not among them: `expander_icon` carries it per call (§5.6). `list.item_font.color` reaches `noninteractive.fg_stroke.color` and `link.font.color` reaches `visuals.hyperlink_color`, so neither is a claimant here |
| `visuals.widgets.hovered.weak_bg_fill` | `style.rs:1300` | 8: `tab`, `sidebar`, `expander`, `button`, `menu`, `combo_box`, `segmented_control` hover backgrounds, plus `switch.hover_unchecked_background`. `list.hover_background` and `checkbox.hover_background` claim `hovered.bg_fill` instead (§5.3, §5.4); `link.hover_background` travels in the link text's own format, per call (§5.3) |
| `visuals.widgets.active.weak_bg_fill` | `style.rs:1300` | 8: `button.active_background`, and seven hover fills that §6.1 copies from `hovered` because their roles state no pressed fill: the `hover_background` of `menu`, `tab`, `combo_box`, `segmented_control`, `sidebar` and `expander`, and `switch.hover_unchecked_background` |
| `visuals.selection.bg_fill` | `style.rs:1196` | 14: `defaults.selection_background`, `defaults.text_selection_background`, `input.selection_background`, `list.selection_background`, `sidebar.selection_background`, `tab.active_background`, `segmented_control.active_background`, `switch.checked_background`, `progress_bar.fill_color`, `slider.fill_color`, `button.primary_background`, and the `Disabled`-cell `slider.disabled_fill_color`, `switch.disabled_checked_background` and `button.disabled_background` |
| `visuals.selection.stroke.color` | `style.rs:1199` | 9: `defaults.selection_text_color`, `defaults.text_selection_color`, `input.selection_text_color`, `list.selection_text_color`, `sidebar.selection_text_color`, `tab.active_text_color`, `segmented_control.active_text_color`, `button.primary_text_color`, and the `Disabled`-cell `button.disabled_text_color` |
| `visuals.widgets.noninteractive.bg_stroke` | `style.rs:1305` | 18: `sidebar`, `status_bar`, `toolbar`, `card` `border.{color,line_width}`, `list.border.line_width`, `separator.{line_color,line_width}`, `splitter.{divider_color,divider_width}`, `list.grid_color`, `menu.separator_color`, `defaults.border.*`, plus eight that reach it through a state set: `button.border.*`, `checkbox.border.*`, `checkbox.unchecked_border_color`, `segmented_control.border.*`, `input.border.*`, `combo_box.border.*`, `tab.border.*`, `expander.border.*`. `window.border.*` reaches `visuals.window_stroke` instead (§5.2) |
| `visuals.widgets.noninteractive.fg_stroke.color` | `style.rs:1311` | 23: `defaults.text_color`, `dialog.body_font.color`, `popover.font.color`, `tooltip.font.color`, `list.item_font.color`; the `font.color` of `button`, `combo_box`, `input`, `menu`, `segmented_control`, `tab`, `sidebar`, `toolbar`, `status_bar` and `expander` and `checkbox.indicator_color`, through their state sets (§6.1); the `Disabled`-cell `disabled_text_color` of `button`, `combo_box`, `input`, `menu` and `list`; `defaults.font.color`, UNMAPPABLE because it loses this, egui's one non-interactive text colour, with no scope to carry it (§5.1); and `dialog.title_font.color`, which loses the `Dialog` scope's slot to `dialog.body_font.color` and goes per call (§5.2) |
| `visuals.disabled_alpha` | `style.rs:1126` | 8: `defaults.disabled_opacity` plus the seven per-widget values at `native-theme/src/model/widgets/mod.rs:65`, `:118`, `:166`, `:323`, `:626`, `:748`, `:793` (`button`, `input`, `checkbox`, `slider`, `switch`, `combo_box`, `segmented_control`) |
| `visuals.widgets.hovered.fg_stroke.color` | `style.rs:1311` | 12: `tab`, `button` and `menu` hover text colours, `splitter.hover_color`, which uses the same field for a line, and eight that reach it through a state set, because their roles state no hover text colour (§6.1): `checkbox.indicator_color` (§5.3), the `font.color` of `segmented_control` (§5.3), `combo_box` and `input` (§5.4), `sidebar`, `toolbar`, `status_bar` and `expander` (§5.6). `list.hover_text_color` and `link.hover_text_color` travel in the text's own format, per call (§5.3, §5.4) |
| `spacing.button_padding` | `style.rs:398` | 6 widgets, 23 leaves: the four `border.padding.*` sides of `button`, `tab`, `combo_box`, `segmented_control` and `menu` (its items, §5.2), and `expander.border.padding.{top,right,bottom}`; a pair of sides shares one `Vec2` component (§5 intro). `checkbox.border.padding.*` reaches `spacing.icon_width_inner` instead — both checkbox frames are inert (§5.3, §6.11) |
| `visuals.widgets.inactive.weak_bg_fill` | `style.rs:1300` | 8: `tab`, `button`, `combo_box`, `segmented_control` background colours, `switch.unchecked_background`, and the `Disabled`-cell `button.disabled_background`, `combo_box.disabled_background` and `switch.disabled_unchecked_background`. `link.background_color` travels in the link text's own format instead (§5.3) |
| `visuals.panel_fill` | `style.rs:1072` | 6: `defaults.background_color` (base owner, §5.9), `window`, `sidebar`, `toolbar`, `status_bar` `.background_color`, `tab.bar_background` |
| `visuals.window_fill` | `style.rs:1063` | 5: `menu.background_color` (base owner, §5.9), `window`, `dialog`, `popover`, `tooltip` `.background_color`. `defaults.surface_color` is not among them: no route carries it (§5.1) |
| `visuals.widgets.open.weak_bg_fill` | `style.rs:1300` | 7: `window.title_bar_background` (base owner, §5.9), `menu.hover_background` through its `{hovered,active,open}` state set (§5.2), and the resting fills that §6.1 copies from `inactive` in their own scopes: `button`, `tab`, `segmented_control` and `combo_box` `.background_color` and `switch.unchecked_background` |
| `visuals.widgets.open.fg_stroke.color` | `style.rs:1311` | 11: `menu.hover_text_color` through its `{hovered,active,open}` state set (§5.2), `checkbox.indicator_color` through its `{5}` set, and the `font.color` of `button`, `combo_box`, `input`, `segmented_control`, `tab`, `sidebar`, `toolbar`, `status_bar` and `expander`, copied from `inactive` (§6.1) |
| `visuals.window_stroke` | `style.rs:1064` | 5: `defaults.border.*` plus `window`, `dialog`, `popover`, `tooltip` `border.{color,line_width}`. `menu.border.*` is the items' and claims `widgets.<state>.bg_stroke` instead (§5.2) |
| `visuals.menu_corner_radius` | `style.rs:1069` | 4: `defaults.border.corner_radius_lg`, `dialog`, `popover`, `tooltip` `border.corner_radius` |
| `visuals.popup_shadow` | `style.rs:1074` | 5: `defaults.shadow_color` (the colour write) plus `defaults.border.shadow_enabled`, `dialog`, `popover`, `tooltip` `border.shadow_enabled` (the gates). `menu.border.shadow_enabled` is the items' and UNMAPPABLE (§5.2) |
| `visuals.window_shadow` | `style.rs:1062` | 3: `defaults.shadow_color` (the colour write) plus `defaults.border.shadow_enabled` and `window.border.shadow_enabled` (the gates) |
| `spacing.menu_margin` | `style.rs:401` | 3 widgets, 12 leaves: `dialog`, `popover`, `tooltip` `border.padding.*`, each in its `Surface` frame; the base style keeps egui's value (§5.9) |
| `visuals.widgets.<state>.bg_fill` | `style.rs:1295` | 13 across all states: `defaults.background_color` (`noninteractive`, §5.1), `scrollbar.thumb_color`, `scrollbar.thumb_hover_color` and `scrollbar.thumb_active_color` (the interactive states' base owners, §5.9), `checkbox.background_color`, `checkbox.checked_background`, `checkbox.unchecked_background`, `checkbox.hover_background`, `checkbox.disabled_background`, `slider.track_color`, `slider.thumb_hover_color`, `slider.disabled_track_color`, `list.hover_background`. `slider.thumb_color` reaches the hovered and pressed handle only through D6's fallback, which is not a claim (conventions above). The `hovered` subset is the row below |
| `visuals.widgets.hovered.bg_fill` | `style.rs:1295` | 5: `scrollbar.thumb_hover_color` (base owner, §5.9), `list.hover_background`, `checkbox.hover_background`, `slider.thumb_hover_color` (whose `None` D6 fills from `slider.thumb_color` — a route, not a claim, §6.9), and `checkbox.checked_background`, whose `Selected` cell writes a `{5}` state set. `expander.hover_background` is not among them: the collapsing header's `bg_fill` layer never runs (§5.6) |
| `visuals.widgets.active.fg_stroke.color` | `style.rs:1311` | 15: `defaults.text_color` (base owner, §5.9, as `strong_text_color()`), `spinner.fill_color`, `list.header_font.color` (via `strong_text_color()`), `button.active_text_color` «Button», and eleven that reach it because their row's state set includes `active` — §6.1 copies `hovered` there where a role states no pressed value: `menu.hover_text_color`, `tab.hover_text_color`, `splitter.hover_color` (§6.9 D7), `checkbox.indicator_color`, and the `font.color` of `expander`, `combo_box`, `segmented_control`, `input`, `sidebar`, `toolbar` and `status_bar`. `button.primary_text_color` is **not** among them: its sink is `visuals.selection.stroke.color`, because `button_style` overwrites `fg_stroke` under `SELECTED_CLASS` (§5.3, `widget_style.rs:150-155`) |
| `visuals.extreme_bg_color` | `style.rs:1045` | 2: `scrollbar.track_color`, `progress_bar.track_color`. `input.background_color` is removed from the contest by `visuals.text_edit_bg_color` (§5.4) |
| `visuals.weak_text_color` | `style.rs:1027` | 2: `defaults.muted_color`, `input.placeholder_color` |
| `spacing.icon_width` | `style.rs:428` | 2: `checkbox.indicator_width`, `combo_box.arrow_icon_size` |
| `spacing.icon_width_inner` | `style.rs:432` | 2 entries, 5 leaves: `expander.arrow_icon_size` and `checkbox.border.padding.*`, each in its own scope; the base style keeps egui's value (§5.9, §6.11) |
| `spacing.scroll.bar_width` | `style.rs:512` | 2: `scrollbar.groove_width`, `scrollbar.thumb_width` — one D1 formula (§6.5) writes whichever of the two `overlay_mode` selects and carries the other in the margins or `floating_width`, so this contest has no loser |
| `spacing.icon_spacing` | `style.rs:436` | 4: `checkbox.label_gap` (base owner, §5.9), `button.icon_text_gap`, `menu.icon_text_gap`, `combo_box.arrow_area_width` |
| `spacing.item_spacing` | `style.rs:392` | 5: `layout.widget_gap`, `toolbar.item_gap`, `dialog.button_gap`, `progress_bar.border.padding.left`, `segmented_control.separator_width` |
| `text_styles[Body]` | `style.rs:77` | 11: `defaults.font`, `input.font`, `tooltip.font`, `popover.font`, `sidebar.font`, `status_bar.font`, `list.item_font`, `dialog.body_font`, `toolbar.font`, `menu.font`, `tab.font` — the last four because a `Button` reads `Body` (B3) |
| `text_styles[Button]` | `style.rs:85` | 3: `button.font`, `combo_box.font`, `expander.font`. No `Button` reads this key (B3), so `menu`, `tab`, `sidebar` and `toolbar` fonts claim `text_styles[Body]` instead, and `checkbox.font` and `segmented_control.font` claim `override_font_id` (§5.3) |
| `override_font_id` | `style.rs:255` | 4: `button.font`, `link.font`, `checkbox.font`, `segmented_control.font`, each in its own scope; unset on the base style (§5.9) |
| `text_styles[Heading]` | `style.rs:88` | 5: `text_scale.section_heading`, `text_scale.dialog_title`, `window.title_bar_font`, `dialog.title_font`, `list.header_font` |
| `visuals.hyperlink_color` | `style.rs:1036` | 3: `defaults.link_color` (base owner, §5.9), `link.font.color`, and `link.disabled_text_color` in the `Disabled` cell |
| `visuals.override_text_color` | `style.rs:1016` | 2: `checkbox.font.color`, and `checkbox.disabled_text_color` in the `Disabled` cell; never written on the base style (§5.10) |
| `visuals.text_edit_bg_color` | `style.rs:1050` | 2: `input.background_color` (base owner, §5.9), and `input.disabled_background` in the `Disabled` cell |

**One contest is *intra-widget* and therefore unresolvable by any scoping
mechanism in 0.36.2**, because both claimants live in the same widget:
`slider.track_color` versus `slider.thumb_color` — the rail is hard-wired to
`inactive` (`widgets/slider.rs:772-775`). **Resolution: `track_color` wins
the rail**; the hovered and pressed handle keeps its own colour,
`thumb_hover_color`, or `thumb_color` where that is `None` (§6.9 D6). The
`Disabled` cell repeats the contest, and `disabled_track_color` wins it over
`disabled_thumb_color` for the same reason.

`input.selection_text_color` and `input.focus_border_color` would be a second —
a stock `TextEdit` strokes its focused frame from `visuals.selection.stroke`
(`widgets/text_edit/builder.rs:742-747`), which is the selected text's colour
(`text_selection/visuals.rs:40`) — but `input_frame` strokes the focused field
in `input.focus_border_color` per call (§4.7), so `selection_text_color` holds
the field alone.
`expander.font.color` and `expander.arrow_color` would be a third — egui's
stock arrow and label both take the state's `fg_stroke.color` — but
`expander_icon` paints the arrow in its own colour per call (§5.6), so the
two never meet in one field. `list.border.color` and `list.grid_color` would be
another — `Frame::group`'s outline and the table's lines read one
`noninteractive.bg_stroke` (`frame.rs:182` versus
`egui_extras/src/table.rs:897-900`, `egui_extras/src/layout.rs:233-238`) — but
the border colour goes on the list `Frame` the application builds, per call
(§5.4), so the grid colour holds the field alone.

---

## 6 -- Derivation formulas

Every formula below is total over `f32`: defined for `NaN`, `±∞`, subnormals and
`-0.0`, with no `unwrap`, no panicking index, no `as` cast on an unconstrained
operand, and no division by a runtime value but epaint's own in §6.15, which an
`f32` division cannot make panic and whose result is tested. `denan`,
`finite_or`, `clamp_length`, `unit_interval`, `u8_from_f32_saturating` and
`i8_from_f32_saturating` are §7.2's helpers.

**One non-finite rule, everywhere.** A theme leaf that is `NaN` or `±∞` is
replaced by **egui's own value for the sink it feeds** — the value the starting
`Style` carries there — and never by `0.0` unless `0.0` *is* that value; the
call site emits `Note::ValueSanitised` (§7.2). A finite value is then floored and
capped by the formula's own clamps, and every layout length it produces passes
`clamp_length` (§7.2), which keeps it below the `f32::MAX * GUI_ROUNDING` edge
egui's rounding turns into `+∞` (§6.7). "Unstated → egui's value" (S2) and
"unusable → egui's value" are therefore the same rule.

### 6.1 The five interaction states

egui selects a `WidgetVisuals` with an identical predicate in both dispatchers
(`Widgets::style`, `style.rs:1273-1284`; `Response::widget_state`,
`widget_style.rs:105-115`):

```text
!response.sense.interactive()                                       -> noninteractive  style.rs:1274
response.is_pointer_button_down_on() || has_focus() || clicked()    -> active          style.rs:1276
response.hovered() || response.highlighted()                        -> hovered         style.rs:1279
otherwise                                                           -> inactive        style.rs:1281
```

Three consequences the derivation must respect:

* **`open` is unreachable from both dispatchers.** `WidgetState` has four
  variants (`widget_style.rs:84-90`). Every use of `widgets.open` is a direct
  field read.
* **Keyboard focus is not a state.** `has_focus()` promotes to `active`, the
  same bucket as "pointer held down". The platform's focus ring is therefore
  not a `WidgetVisuals` value at all; the install plugin paints it (§6.18).
* **Disabled is not a state.** `Ui::disable` multiplies painter opacity
  (`ui.rs:497-502`); there is no `disabled` `WidgetVisuals`.

The policy, applied to the base style and to every role scope. A role scope
applies it to the values its role states and to nothing else: a field for
which the role has no value keeps the base style's, in every entry, because
the cell starts from the base style (§3.4). Which entries each role's leaves
reach follows the table and *Which entries a role writes* below it; §5's state
sets are that rule applied, leaf by leaf.

| egui slot | source |
|---|---|
| `noninteractive` | the role's non-interactive colours: `fg_stroke.color` ← the role's `font.color` (or `defaults.text_color` on the base style); `bg_stroke` ← the role's border colour × `defaults.border.opacity` and its `line_width`; `corner_radius` ← the role's `border.corner_radius`; `bg_fill` and `weak_bg_fill` ← `defaults.background_color`, on the base style only; `expansion` unchanged (egui's `0.0` in all ten stock entries: `Widgets::dark` `style.rs:1689`, `:1697`, `:1705`, `:1713`, `:1721`; `Widgets::light` `:1734`, `:1742`, `:1750`, `:1758`, `:1766`) |
| `inactive` | the role's resting interactive colours: `weak_bg_fill` ← the role's `background_color`, `fg_stroke.color` ← the role's `font.color`, border and radius as above. On the base style this comes from `theme.button`, border colour, width and radius included, while the base style's `noninteractive` takes its border and radius from `defaults.border`: the interactive states are what every unscoped control — `Button`, `ComboBox`'s trigger, `DragValue`, a selectable — draws with, and each is a `Button` or paints like one, while `noninteractive` is what frames, separators and group boxes draw with (§5.9). The base style's `bg_fill` in the four interactive entries is the scrollbar thumb's, not the button's — `scrollbar.thumb_color` here, and in `open` as copied — because every unscoped `ScrollArea` handle reads it (`scroll_area.rs:1457-1466`, `:1499-1503`) and a `Button` fills from `weak_bg_fill` (§5.3 B1). Inside `Role::Menu`, `menu.background_color` is the menu's frame (§5.2), not an item's: an item rests transparent, as the cell's `menu_style` leaves it (§3.4), outlined and rounded by `menu.border`, the item's border (§5.2) |
| `hovered` | the role's `hover_*` values; a hover **fill** is a state layer composited over the role's idle fill (see "Hover and pressed fills are state layers" below), a hover text or border colour is written as given; `soft_option` fallbacks per §6.4. Every field the role states no hover value for — `bg_stroke`, `corner_radius`, `fg_stroke.width` — is `inactive`'s, so a hover changes only what the platform states: egui's stock entries would otherwise widen the radius to `3` (`style.rs:1704`, `:1749`) and the foreground stroke to `1.5` (`:1703`, `:1748`), which is egui's look, not the platform's. On the base style, `composite_over(button.hover_background, button.background_color)` → `hovered.weak_bg_fill`, `button.hover_text_color` → `hovered.fg_stroke.color` and `scrollbar.thumb_hover_color` → `hovered.bg_fill`, written as given (below) |
| `active` | the role's `active_*` values where they exist, a pressed fill composited exactly as a hover fill is; every other field is `hovered`'s — all of them where the role states no `active_*` — so egui's stock `active` stroke of `2.0` (`style.rs:1711`, `:1756`) does not appear either. On the base style `bg_fill` is `scrollbar.thumb_active_color`, or `thumb_hover_color` where it is `None` (§6.9 D5), and `fg_stroke.color` is `defaults.text_color`, not `button.active_text_color`: egui reads it as `strong_text_color()` (`style.rs:1147-1149`), so it is the colour of every strong label on the panel, and only the `Role::Button` cell carries the platform's pressed text (§5.3) |
| `open` | a field-wise copy of `inactive` — on the base style the whole entry, in a role scope every value the role writes into `inactive` — since native-theme states no open-state value, with two exceptions. On the base style `window.title_bar_background` → `open.weak_bg_fill`, which egui paints under the top-most window's title bar (`window.rs:1427`). Inside `Role::Menu`, `open` is a field-wise copy of **`hovered`**: `SubMenuButton::ui` swaps `open` in for `inactive` while the item's submenu is open (`containers/menu.rs:382-384`), and the platform keeps that item highlighted exactly as on hover — WinUI 3 gives `MenuFlyoutSubItemBackgroundSubMenuOpened`, `…ForegroundSubMenuOpened` and `…ChevronSubMenuOpened` the same brushes as the `…PointerOver` keys in its `Default` and `Light` dictionaries (microsoft-ui-xaml commit `258a2e9b`, `controls/dev/CommonStyles/MenuFlyout_themeresources.xaml` lines 16/18, 22/24, 27/29 and 182/184, 188/190, 193/195, applied by the `SubMenuOpened` state at lines 662-669); §5's `menu.hover_background` and `menu.hover_text_color` rows carry the two leaves. `WidgetState` cannot select `open`; egui reads it directly for the combo-box trigger and chevron while the popup is open (`combo_box.rs:371`, `:449-451`, painted `:460`), the colour button while its picker is open (`widgets/color_picker.rs:116-117`) and a submenu button while its submenu is open (`containers/menu.rs:382-384`). Outside a scope the combo-box trigger and the submenu button show the title-bar colour as their fill; the colour button strokes its outline with `open.bg_fill` (`widgets/color_picker.rs:127-131`), a copy of the base style's `inactive.bg_fill`, `scrollbar.thumb_color`. Inside the `Role::ComboBox` scope `open` is the combo box's own `inactive`, copied, and inside `Role::Menu` the item's hover highlight |

**Which entries a role writes.** Per field, in this order: `noninteractive`
and `inactive` from the role's leaves as the table says; `hovered` from the
role's hover leaf for that field, or else `inactive`'s; `active` from its
pressed leaf, or else `hovered`'s; `open` a copy of `inactive` — of `hovered`
in `Role::Menu`. The "or else" is decided by the model, not by the preset:
where the model has a hover or pressed leaf for the field, that leaf holds the
entry and a `None` soft option there falls back per §6.4; where it has none,
the copy is unconditional. So a role that states a text colour and no hover
text colour — `input`, `combo_box`, `segmented_control`, `expander`,
`sidebar`, `toolbar`, `status_bar` — writes it into all five entries, and a
role that states a hover fill and no pressed one — `menu`, `tab`, `combo_box`,
`segmented_control`, `switch`, `checkbox`, `expander`, `sidebar`, `list`,
`slider` — writes the hover fill into `active` too. Three kinds of role write
fewer entries than the table names, because the entry is not their widget's:

* **Surface roles** — `Dialog`, `Popover`, `Tooltip` — write their text colour
  into `noninteractive` only. Their body text is `Label`s, painted from
  `Visuals::text_color()` (`widgets/label.rs:296-299`, `style.rs:1136-1139`),
  while the controls inside a dialog, popover or tooltip are ordinary controls
  that keep the base style's interactive entries, and a surface's border and
  fill are its `Surface` frame's (§5.2). `Window` and `Card` write no entry.
* **Container-edge roles** — `Sidebar`, `Toolbar`, `StatusBar`, `List` — write
  their border into `noninteractive` only, where their edge is painted: a
  panel's separator line (`containers/panel.rs:911`) or a group's frame
  (`containers/frame.rs:181-182`). In the interactive entries it would outline
  every button in the panel. A panel role's radius is its `Surface::Panel`
  frame's (§5.6). `List`'s stroke colour there is `list.grid_color`, which its
  table lines read; its border colour goes on the application's list `Frame`
  per call (§5.4). `List` puts its cell text in `noninteractive` alone, because
  its cells are `Label`s, and its header text in `active`, through
  `strong_text_color()` (`style.rs:1147-1149`; §5.4).
* **`Role::Menu`** writes its item border, radius and padding (`menu.border`,
  §5.2) over `menu_style`'s, and nothing into the frame's stroke, radius or
  shadow, which the base style's `defaults.border` values carry, as
  `docs/platform-facts.md:1237-1240` sends them to §2.16. The item border goes
  into the four interactive entries only; its separator colour goes to
  `noninteractive.bg_stroke.color`, which `separator_style` strokes
  (`widget_style.rs:212-217`).

Within one cell a field holds one leaf. The checkbox's
fill field is `bg_fill` and its `fg_stroke` the mark's `indicator_color`
(§5.3); the scrollbar and slider thumbs, the splitter's lines and the
spinner's arc write the fields §5.5 and §6.9 name.

**Why the base style's interactive states come from `theme.button`, and why that
is a stated borrowing rather than a derivation.** `ResolvedDefaults` has 31
fields and **not one of them is a hover or pressed value**
(`native-theme/src/model/resolved.rs:72-146`). Every hover colour in the model
lives on a widget. `Button` is by a wide margin egui's most common interactive
widget: `ui.button` (`ui.rs:1848`), `ui.toggle_value` (`:1875`),
`ui.selectable_label` (`:1929`) and `ui.selectable_value` (`:1939`) all build
one, and so does every menu entry — `MenuButton` and `SubMenuButton` each hold a
`Button` field and construct it from the caller's atoms
(`containers/menu.rs:291`, `:297`, `:338`, `:347`). Borrowing from it is the only
non-inventing choice.

No hover or pressed shade is ever computed: every such formula needs a factor
`ResolvedTheme` does not state.

`active` when the role's `active_*` is a `None` `soft_option` — as
`button.active_background` legitimately is (`widgets/mod.rs:77-78`) — falls back
to `hovered`, i.e. no fill change on press, with the platform's own border still
applied. That is honest; a synthesised pressed shade would not be.

**Hover and pressed fills are state layers (C17).** The platform paints a hover
or pressed colour *over* the widget's own idle fill; egui, like iced and
gpui-component, *replaces* the fill per state — `Style::button_style` hands
`widgets.<state>.weak_bg_fill` straight to the button's frame
(`widget_style.rs:147`, `:159`) and `Style::checkbox_style` hands
`widgets.<state>.bg_fill` to the box (`widget_style.rs:175`, `:182`). Emitting a
translucent layer as it stands would paint it over whatever lies behind the
widget. So a **stated** hover or pressed fill is composited over the idle fill
before it is written, with §7.2's `composite_over`:

```text
hovered.<fill> := composite_over(<role>.hover_background, <idle>)
active.<fill>  := composite_over(button.active_background.unwrap_or(button.hover_background), <idle>)  // Button only
```

| role | `<fill>` | `<idle>` — the fill the layer sits on |
|---|---|---|
| base style and `Role::Button` | `weak_bg_fill` | `button.background_color` |
| `Role::ComboBox` | `weak_bg_fill` | `combo_box.background_color` |
| `Role::SegmentedControl` | `weak_bg_fill` | `segmented_control.background_color` |
| `Role::Tab` | `weak_bg_fill` | `tab.background_color`, the resting tab fill, which a tab drawn as `Button::new(..).selected(..)` paints because `Button::new` keeps `frame_when_inactive` (`widgets/button.rs:54`, `:364`); a transparent resting fill makes the composite the layer itself (`ecolor/src/color32.rs:343-345`), so the hover then lies over the tab bar as on the platform. The selected tab takes its fill from `selection.bg_fill` in every state (`widget_style.rs:150-152`), so no layer is laid on it |
| `Role::Switch` | `weak_bg_fill` | `switch.unchecked_background`, the resting unchecked track, which the substitute `Button::new(..).selected(checked)` paints because `Button::new` keeps `frame_when_inactive` (`widgets/button.rs:54`, `:364`); the layer is `switch.hover_unchecked_background`. A checked substitute takes its fill from `selection.bg_fill` in every state (`widget_style.rs:150-152`), so no layer is laid on it (§6.2); its hovered fill goes per call (§5.3) |
| `Role::Checkbox`, `Normal` cell | `bg_fill` | `checkbox.unchecked_background.unwrap_or(checkbox.background_color)` |
| `Role::Checkbox`, `Selected` cell | `bg_fill` | none: no layer is laid on it. `checkbox.hover_background` is the unchecked box's hover fill, so the hovered `Selected` cell copies the idle one (§6.4; rationale §8, Q-8) |

This is the rule both sibling connectors apply: gpui composites
`button.hover_background` over the button's idle fill with `Hsla::blend`
(`connectors/native-theme-gpui/src/colors.rs:354-355`, reasoned at `:339-353`),
and iced composites its widgets' hover and pressed layers over their idle fills
(`connectors/native-theme-iced/src/styles.rs:126-133` for the button, `:520-522`
for the checkbox, `:802-806` for the combo box). The rule's scope, measured when
it was made: the button's two layers are translucent only on windows-11, and the
checkbox's hover also on material; everywhere else a layer is opaque and
compositing it is the identity
(`docs/archive/todo_v0.5.9_theme-contracts-rationale.md:424-428`).

Written **as given**, never composited: every idle fill; every disabled fill
(§6.3), which replaces the idle fill and lets the window show through; a
`soft_option` fallback (§6.4), which *is* the idle fill, copied (C16); the row
highlights `menu.hover_background`, `list.hover_background` and
`sidebar.hover_background`, which egui paints over a panel it also paints; `expander.hover_background`, because `ExpanderTheme` has
no idle fill of its own (`native-theme/src/model/widgets/mod.rs:828-849`): the
`Role::Expander` scope turns on `collapsing_header_frame`, whose one layer fills
the header with the state's `weak_bg_fill` (`collapsing_header.rs:561-568`), and
makes the resting `inactive.weak_bg_fill` `Color32::TRANSPARENT` — legal for
`weak_bg_fill` (`style.rs:1297-1300`) — so no fill appears where the platform
draws none and the hover fill lies over nothing (compositing over a transparent
fill would be the identity anyway, `ecolor/src/color32.rs:343-345`); and the scrollbar and slider thumb colours of
§6.9, which the thumb itself carries over a rail egui also paints — iced emits
its thumb colours as given for the same reason
(`connectors/native-theme-iced/src/styles.rs:897`, `:963-966`).

**One slot serves framed and frameless readers alike, and it is composited.**
The base style's `hovered.weak_bg_fill` is also what a frameless reader paints
over the backdrop: `Button::selectable(false, ..)` sets `frame_when_inactive(false)`
(`widgets/button.rs:78-83`), so `ui.selectable_label` and `ui.selectable_value`
(`ui.rs:1929`, `:1939`) show no fill at rest and the hovered fill over the panel
(`widgets/button.rs:364-368`), and `menu_style` makes every menu entry's idle fill
`TRANSPARENT` (`containers/menu.rs:27`). gpui keeps a raw copy of the layer for
exactly those readers, which are transparent-idle there too
(`docs/archive/todo_v0.5.9_theme-contracts-rationale.md:436-440`); egui has one
field, so a frameless reader shows the layer over
`button.background_color` rather than over its backdrop. That differs from the
platform only where the layer is translucent, which for the button is
windows-11 alone; inside `Role::Menu` the row highlight is written as given and
the difference does not arise.

### 6.2 `RoleVariant::Selected`

`Checkbox` is the one role with a `Selected` cell. The four roles whose
checked, active or suggested appearance is a `Button`'s — `Button`, `Tab`,
`SegmentedControl` and `Switch` — carry it in the `Normal` cell, and their
`Selected` cell is the `Normal` `Arc`: each substitute passes its state to the
widget itself, `Button::new(..).selected(..)` (§5.3, §5.6), and
`Style::button_style` overwrites `weak_bg_fill`, `bg_fill` and `fg_stroke` from
`visuals.selection.*` for exactly such a button, **regardless of the
interaction state** (`widget_style.rs:147`, `:150-155`). So one scope serves a
whole tab bar, segment row or button row, each button's own flag picks its
look, and the `Normal` cell writes:

```text
visuals.selection.bg_fill      := to_color32(<role>.<selected background>)
visuals.selection.stroke.color := to_color32(<role>.<selected text colour>)
visuals.selection.stroke.width := unchanged   // egui's 1.0, style.rs:1620; no native source
```

with the per-role sources `button.primary_background` / `primary_text_color`,
`tab.active_background` / `active_text_color` and
`segmented_control.active_background` / `active_text_color`. Selected text
inside such a scope takes the same pair, because egui has one `Selection` for
selected widgets and selected text (`text_selection/visuals.rs:39-40`, §5.1).
The `Switch` `Normal` cell writes the same branch's fill, `visuals.selection.bg_fill :=
to_color32(switch.checked_background)`, and only that: `button_style` applies it
exactly when the substitute is checked, in every state (`widget_style.rs:150-152`),
while an unchecked one paints `widgets.<state>.weak_bg_fill` (§6.1).
`ResolvedSwitchTheme` states no text colour
(`native-theme/src/model/widgets/mod.rs:599-642`), so `selection.stroke.color`
stays at the value the base style carries rather than borrowing one.
`Role::Checkbox` is different in kind: `Checkbox` never reads
`selection` — `SELECTED_CLASS` is tested only in `Style::button_style`
(`widget_style.rs:150`) and added only by `Button` (`widgets/button.rs:329`),
while `Style::checkbox_style` fills the box from the state's `bg_fill`
(`widget_style.rs:175`, `:182`) and egui paints `checkbox_frame.fill` with no
checked branch (`widgets/checkbox.rs:134-140`). So the checkbox `Selected` cell
writes `checkbox.checked_background` into `widgets.{5}.bg_fill`, which fills the
box, and `checkbox.border.color` into `widgets.{5}.bg_stroke.color`, the checked
box's outline (`widget_style.rs:184`), in place of the `Normal` cell's
`unchecked_border_color` (§5.3, §6.4); its check mark keeps
`checkbox.indicator_color` from `widgets.{5}.fg_stroke.color` (§5.3).

Roles with no selected data share the `Normal` `Arc` — no allocation, no
duplicate.

**`Selected` and `Disabled` together are not offered as a variant.**
`RoleVariant` is a three-value enum. The only native data for the combination is
`switch.disabled_checked_background` / `disabled_unchecked_background`, and the
switch needs no combined variant: its checked state is the widget's own flag, so
its `Disabled` cell carries both disabled tracks and the flag picks one (§6.3).
No other role has native data for the combination; for a selected `Button`,
`button_style` overwrites the fill and stroke from `selection.*` regardless of
the `inactive` entry. So the `Button` `Disabled` cell also writes the disabled fill
and text colour into `selection.bg_fill` and `selection.stroke.color` (§6.3): a
selected (primary) button in a `Disabled` scope shows the disabled colours, not
the primary ones. `RoleVariant` is `#[non_exhaustive]`, so adding a combined
variant later is additive.

### 6.3 `RoleVariant::Disabled`

A disabled widget lands in `WidgetState::Inactive`: it is still
`sense.interactive()`, because hit testing strips the sense only from its own
copy (below), and every predicate `Response::widget_state` tests after that
(`widget_style.rs:105-115`) is false for it:

* `hovered()` — `Flags::HOVERED` is set only inside `if res.enabled()`
  (`egui/src/context.rs:1496-1500`);
* `clicked()` — `Flags::CLICKED` only under
  `if enabled && sense.senses_click() && …` (`:1525-1526`);
* `is_pointer_button_down_on()` — hit testing strips `Sense::CLICK` and
  `Sense::DRAG` from a disabled widget before any interaction is computed
  (`egui/src/hit_test.rs:131-139`), so it can never become `potential_click_id`
  or `potential_drag_id` (`egui/src/interaction.rs:165`, `:170`);
* `has_focus()` — `interested_in_focus = w.enabled && …`
  (`egui/src/context.rs:1256`), with `mem.surrender_focus(w.id)` at `:1274-1276`;
* and `highlighted()`, which is **not** gated on `enabled`
  (`egui/src/context.rs:1439`, `:1454`) but which egui never sets by itself: its
  only setter is the public `Response::highlight` (`egui/src/response.rs:743-746`),
  which no egui widget calls. An application that highlights a disabled widget
  gets the `hovered` entry.

The `Disabled` cell, which starts from its role's `Normal` cell (§3.4),
therefore writes the platform's disabled colours into the
fields a widget in the `inactive` state paints from, and neutralises the opacity
multiply:

```text
// fill: into the field the role's widget paints its idle fill from
widgets.inactive.weak_bg_fill := to_color32(<role>.disabled_background.unwrap_or(<idle>))   // Button, ComboBox
widgets.inactive.bg_fill      := to_color32(checkbox.disabled_background.unwrap_or(<idle>)) // Checkbox
visuals.text_edit_bg_color    := Some(to_color32(input.disabled_background.unwrap_or(input.background_color)))  // Input
widgets.inactive.bg_fill      := to_color32(slider.disabled_track_color.unwrap_or(slider.track_color))          // Slider rail
visuals.selection.bg_fill     := to_color32(slider.disabled_fill_color.unwrap_or(slider.fill_color))            // Slider trailing fill
widgets.inactive.weak_bg_fill := to_color32(switch.disabled_unchecked_background.unwrap_or(switch.unchecked_background)) // Switch, off
visuals.selection.bg_fill     := to_color32(switch.disabled_checked_background.unwrap_or(switch.checked_background))     // Switch, on
visuals.selection.bg_fill     := to_color32(button.disabled_background.unwrap_or(button.background_color))             // Button, selected (primary)
// text
widgets.inactive.fg_stroke.color       := to_color32(<role>.disabled_text_color)          // Button, ComboBox, Menu, Input; Checkbox's mark
widgets.noninteractive.fg_stroke.color := to_color32(<role>.disabled_text_color)          // Button, ComboBox, Menu, Input, List: labels, cells
visuals.override_text_color            := Some(to_color32(checkbox.disabled_text_color))   // Checkbox: its label, and every label in the scope
visuals.hyperlink_color                := to_color32(link.disabled_text_color)             // Link
visuals.selection.stroke.color         := to_color32(button.disabled_text_color)           // Button, selected (primary)
visuals.disabled_alpha                 := 1.0
```

`<idle>` is the role's idle fill of §6.1's table — `button.background_color`,
`combo_box.background_color`,
`checkbox.unchecked_background.unwrap_or(checkbox.background_color)`. The eight
disabled fill leaves — four `disabled_background`s, `slider.disabled_{track,fill}_color`
and `switch.disabled_{unchecked,checked}_background` — are `soft_option`s, and a
`None` is the platform stating no distinct disabled fill, so it copies the
role's own fill for that part (C16, §6.4), as iced does for the first four
(`connectors/native-theme-iced/src/styles.rs:136`, `:374`, `:524`).

The slider and switch lines follow from where egui paints those widgets. A
disabled `Slider` is `WidgetState::Inactive` (above), and its rail is always
`widgets.inactive.bg_fill` while the trailing fill up to the handle is
`selection.bg_fill` (`widgets/slider.rs:774-775`, `:781-803`; §5.5). The switch's
substitute is a `Button::new(..).selected(checked)` (§5.3): unchecked it fills
from the state's `weak_bg_fill` (`widget_style.rs:159`), checked from
`selection.bg_fill` in every state (`:150-152`), so one cell carries both
disabled tracks and the widget's own flag chooses. What stays lost is the knob:
`slider.disabled_thumb_color` because the handle fills from the same
`inactive.bg_fill` as the rail (`slider.rs:810-816`), and
`switch.disabled_thumb_color` because the substitute has no knob.

A disabled fill is written as given, never composited (§6.1); a fully
transparent one is real data — `windows-11` states `#f9f9f900` (light) and
`#33333300` (dark) for the checkbox
(`native-theme/src/presets/windows-11.toml:133`, `:513`) — so the checkbox
cell writes a transparent `bg_fill` there and emits `Note::TransparentFill`
(§6.4). Each sink is the
one the widget actually reads: `Style::button_style` fills a button from
`weak_bg_fill` (`widget_style.rs:159`) and `button_frame` a combo-box trigger
(`containers/combo_box.rs:460`); `Style::checkbox_style` fills the box from
`bg_fill` (`widget_style.rs:182`); a `TextEdit` fills from
`Visuals::text_edit_bg_color()` and never from `widgets.*.weak_bg_fill`
(`widgets/text_edit/builder.rs:738-739`, `style.rs:1152-1154`), which is where
`input.background_color` already goes (§5.4). The text colour needs four sinks
beyond `inactive.fg_stroke`: a plain `Label` is painted from
`Visuals::text_color()`, i.e. `noninteractive.fg_stroke.color`
(`widgets/label.rs:296-299`, `style.rs:1136-1139`), so without the second line
the text of a disabled scope would render at full strength once the fade is
neutralised — `List` writes that line alone, since its cells are `Label`s and
its `Normal` cell puts its text colour in no other entry (§6.1); the
`Role::Checkbox` scope sets `override_text_color` to
`checkbox.font.color` (§5.3), which wins over `fg_stroke` for the label
(`widget_style.rs:133-136`) while the check mark still strokes `fg_stroke`
(`widgets/checkbox.rs:151-158`) — both take `checkbox.disabled_text_color`,
as in iced (`styles.rs:564-565`), and because `override_text_color` comes first
in `Visuals::text_color()` it is the label colour of every `Label` in that scope
too, so the checkbox cell needs no `noninteractive` line; and `Link::ui` paints
with `visuals.hyperlink_color` alone (`widgets/hyperlink.rs:47`), so the link
cell writes that field only; and a selected (primary) `Button` takes its fill
and text colour from `selection` in every state (`widget_style.rs:150-154`),
so the button cell writes `selection.bg_fill` and `selection.stroke.color`
too (§6.2).

`1.0` is a **multiplicative identity, not a theme value**, and is one of the
three structural constants listed in §6.17. `Ui::disable` (`ui.rs:497-502`)
multiplies painter opacity by `disabled_alpha`; `1.0` makes the multiply the
identity while interaction stays blocked, so the platform's own disabled colour
survives instead of being faded a second time. The multiply also covers what no
colour field reaches — images, icons and borders — so inside a `Disabled` scope
those are no longer faded; native-theme states no disabled border or icon
treatment to put in its place.

The cell writes **15** leaves, each through the field its widget
reads, so §5 records each as SCOPED by §2's definitions: the four `disabled_background` leaves (`button`
`native-theme/src/model/widgets/mod.rs:81`, `input` `:130`, `checkbox` `:175`,
`combo_box` `:757`), `slider.disabled_fill_color` and `disabled_track_color`
(`:329`, `:332`), `switch.disabled_checked_background` and
`disabled_unchecked_background` (`:635`, `:638`), and the seven per-widget
`disabled_text_color` leaves (`button` `:75`, `input` `:121`, `checkbox` `:169`,
`menu` `:227`, `list` `:529`, `combo_box` `:751`, `link` `:874`).
`slider.disabled_thumb_color` and `switch.disabled_thumb_color` (`:335`, `:641`)
stay lost, as above. `defaults.disabled_text_color` has no `Role` scope at all,
because `Role` has exactly one variant per **widget** field of `ResolvedTheme`
and none for `defaults` (§4.4). And all of it applies **only for widgets the
application scopes**. An unscoped disabled widget still gets egui's opacity fade
from `defaults.disabled_opacity`. §14 item 6 records the loss.

Every other role writes none of these leaves, so its `Disabled` cell is the
`Normal` `Arc` and `disabled_alpha` is left at the theme's value.

### 6.4 Soft-option fallbacks

A `None` `soft_option` is **not missing data** — it is the platform asserting
that the widget has no distinct appearance in that state. The correct
derivation is therefore always a **copy**, never a computed tint: no arithmetic,
no division, no narrowing, no `NaN` path.

| leaf | `None` fallback | terminates on |
|---|---|---|
| `button.active_background` | `button.hover_background` | required field (`native-theme/src/model/widgets/mod.rs:67-68`) |
| `tab.hover_background` | `tab.background_color` | required field |
| `expander.hover_background` | the scope's resting `inactive.weak_bg_fill`, `Color32::TRANSPARENT` (§6.1): no highlight, as the platform states | a constant, no fill — which `weak_bg_fill` may hold (`style.rs:1297-1300`) |
| `expander.arrow_color` | egui's own `paint_default_icon`, in the label's colour (§6.19) | egui's code path, no colour |
| `checkbox.hover_background` (Normal cell) | `checkbox.unchecked_background.unwrap_or(checkbox.background_color)` | required field |
| `checkbox.hover_background` (Selected cell) | not read, stated or not: the hovered cell is `checkbox.checked_background`, the idle `Selected` cell copied (rationale §8, Q-8) | required field |
| `checkbox.unchecked_background` | `checkbox.background_color` | required field |
| `checkbox.unchecked_border_color` | `checkbox.border.color` | required field |
| `combo_box.hover_background` | `combo_box.background_color` | required field |
| `segmented_control.hover_background` | `segmented_control.background_color` | required field |
| `switch.hover_unchecked_background` | `switch.unchecked_background` | required field (`native-theme/src/model/widgets/mod.rs:604`) |
| `switch.hover_checked_background` | none passed: `switch.checked_background` stands | required field (`native-theme/src/model/widgets/mod.rs:602`) |
| `input.hover_border_color` | `input.border.color` | required field |
| `scrollbar.thumb_active_color` | `scrollbar.thumb_hover_color` | required field |
| `slider.thumb_hover_color` | `slider.thumb_color` | required field |
| `button.disabled_background`, `combo_box.disabled_background` (`Disabled` cell) | the role's `background_color` | required field |
| `input.disabled_background` (`Disabled` cell) | `input.background_color` | required field |
| `checkbox.disabled_background` (`Disabled` cell) | `checkbox.unchecked_background.unwrap_or(checkbox.background_color)` | required field |
| `slider.disabled_track_color`, `slider.disabled_fill_color` (`Disabled` cell) | `slider.track_color`, `slider.fill_color` | required field |
| `switch.disabled_unchecked_background`, `switch.disabled_checked_background` (`Disabled` cell) | `switch.unchecked_background`, `switch.checked_background` | required field (`native-theme/src/model/widgets/mod.rs:602`, `:604`) |

Every chain but the expander's two ends on a required field after at most two
`unwrap_or` steps, each onto a different leaf, so no chain can loop and none can
end in `None`; the expander's end on egui's own value at once.

A fallback is the idle value itself, so it is written as given and never
composited: compositing the idle fill over itself would change a translucent
idle fill, and C16 asks for a copy. Only a **stated** hover or pressed fill goes
through §6.1's `composite_over`. `button.active_background`'s fallback is
different in kind: it is another *layer*, `button.hover_background`, and is
composited like any stated layer.

One constraint that removes an apparently obvious option:
`WidgetVisuals::bg_fill` is documented **"Must never be
`Color32::TRANSPARENT`"** (`style.rs:1292-1294`), while `weak_bg_fill`
explicitly **"May be `Color32::TRANSPARENT`"** (`:1297-1300`). "Paint nothing"
is therefore expressible only through a `weak_bg_fill`, which is why the
expander's header — the one surface whose native idle state has no fill at all —
is drawn through the `collapsing_header_frame` layer that reads `weak_bg_fill`
(§6.1) rather than through a `bg_fill`. Where a native colour with `a == 0` would reach a
`bg_fill`, the connector emits a `Note` rather than substituting a colour — a
substitution would be an invented value. A §6.1 composite can reach that case only
when the layer and the idle fill both have `a == 0` (`Color32::blend` adds the
layer's alpha to the scaled idle alpha, `ecolor/src/color32.rs:343-345`), so in
practice it concerns the fills written as given. The variant is
**`Note::TransparentFill { path }`**, one of `Note`'s six (§4.3). None of the
others describes the case — `ValueSanitised` is documented as "a theme value was
non-finite, or a text size not a positive normal `f32`, and was replaced by the
documented fallback", and here the value is
finite and nothing is replaced — and letting the fill through unreported would
leave an application to find out from a widget with no background that egui's
documented invariant was broken. The transparent value itself is still what is
written: it is the platform's, and the native look has no better number to
offer. `Note` is `#[non_exhaustive]`, so the variant costs no compatibility.

### 6.5 D1 — scrollbar groove and thumb widths

```text
// only when both widths are finite; otherwise the block writes nothing (Boundaries)
let g = clamp_length(scrollbar.groove_width);           // >= 0.0 and below the rounding edge
let t = clamp_length(scrollbar.thumb_width).min(g);     // thumb never wider than groove
let pad = (g - t) * 0.5;                                // finite and >= 0.0: so are g and t
```

Non-overlay (`scrollbar.overlay_mode == false`):

```text
spacing.scroll.floating         = false;   // style.rs:503
spacing.scroll.foreground_color = false;   // :538  handle reads bg_fill, not fg_stroke (§5.5)
spacing.scroll.bar_width        = t;       // :512
spacing.scroll.bar_inner_margin = pad;     // :518
spacing.scroll.bar_outer_margin = pad;     // :522
```

`ScrollStyle::allocated_width()` is
`bar_inner_margin + bar_width + bar_outer_margin` (`style.rs:656-662`) =
`pad + t + pad` = **`g`**, so the layout reserves exactly the platform groove
width while the painted bar is `t` wide and centred inside it. This is
necessary because the trough rect and the handle rect share an identical
cross-axis range in egui (`scroll_area.rs:1360-1364` versus `:1388-1398`), so a
narrow thumb inside a wide groove is not otherwise expressible.

Overlay (`overlay_mode == true`):

```text
spacing.scroll.floating                 = true;   // :503
spacing.scroll.foreground_color         = false;  // :538  floating()'s default is true (:647)
spacing.scroll.floating_allocated_width = 0.0;    // :535  -> allocated_width() == 0.0
spacing.scroll.floating_width           = t;      // :527  idle thickness
spacing.scroll.bar_width                = g;      // :512  hover thickness and hit target
```

**At rest the overlay bar is invisible, and that is accepted rather than
corrected.** `ScrollStyle`'s six opacity fields exist only to modulate this
branch and are left at `ScrollStyle::floating()`'s values, which is also
`ScrollStyle`'s `Default` (`style.rs:585-589`, `:643-653`): both
`dormant_background_opacity` (`:649`) and `dormant_handle_opacity` (`:650`) are
`0.0`, against `0.4` / `0.7` and `0.6` / `1.0` inherited from `solid()`
(`:607-608`, `:611-612`). Both are gated on `floating` and applied with
`gamma_multiply` (`scroll_area.rs:1469-1497`, `:1506-1519`), so on a preset with
`overlay_mode == true` the scrollbar colours §5.5 maps are painted **fully
transparent** until the pointer enters the scroll area. The non-overlay branch
short-circuits both to `1.0` (`:1483-1484`, `:1495-1497`) and is unaffected.
That is exactly what `overlay_mode` means — auto-hiding rather than persistent
(`docs/platform-facts.md:1001`) — and native-theme carries no fade-curve leaf to
override it with (`overlay_mode` is a bare `bool`,
`native-theme/src/model/widgets/mod.rs:287`), so the six fields are left alone
rather than invented.

The asymmetry between the two branches is forced by egui, not chosen: in the
floating branch `bar_inner_margin` is computed but never used
(`scroll_area.rs:1296` feeds only the `else` arm at `:1356`), the bar's
thickness is `lerp(floating_width ..= bar_width, hover_t)` (`:1347-1351`), and
`allocated_width()` short-circuits to `floating_allocated_width`
(`style.rs:657-658`). No hovered-thumb-width constant is invented:
`groove_width` is the only other value in `ResolvedScrollbarTheme` that
describes the gutter.

**Boundaries.** `g == 0` ⇒ `t == 0`, `pad == 0`: no bar is drawn, and egui never
divides by a bar width. `t > g` ⇒ `t` clamps to `g`, `pad == 0`. A negative
finite width ⇒ `0`, by `clamp_length`'s floor. **`NaN` or `±∞` in either width ⇒
the whole block writes nothing** and `Note::ValueSanitised` names the leaf: the
five `spacing.scroll` fields each branch writes are joint functions of the two
widths and of `overlay_mode`, so egui's
own value for each sink is its whole starting `ScrollStyle` — `floating()`,
`ScrollStyle`'s `Default` (`style.rs:585-589`), in a default `Style` — never a
`0.0` bar. `1e30` ⇒ all three stay finite and the passes complete. A huge solid
groove is the width `allocated_width()` reserves, so it reaches egui's rounding
edge (§6.7): measured on egui 0.36.2 with rustc 1.95 under debug assertions, a
vertical and a two-axis `ScrollArea` in a `Window` completed three passes with
`g` at `clamp_length`'s ceiling (thumb `0`, `6` and the ceiling itself), at
`f32::MAX * GUI_ROUNDING` and one `f32` above it, and tripped with `g` at twice
that value or at `f32::MAX`. The ceiling therefore sits below every trip
measured; the overlay bar reserves `0.0` and completed at `f32::MAX` (§6.7). All
targets are `f32` — no integer narrowing.

**Why the guard is on the inputs and not on `pad`.** Guarding only the result
would still let `groove_width == thumb_width == +∞` reach `g == t == ∞` and make
`pad` the `∞ − ∞` `NaN`, and — worse — it would leave `+∞` in `t` and `g`
themselves, which the overlay branch writes straight into `floating_width` and
`bar_width`, and which the non-overlay branch feeds to
`ScrollStyle::allocated_width` (`style.rs:656-662`). The finiteness test runs on
the two leaves before any arithmetic, which is §7.2's discipline ("`denan` must
run first, every time") in its `±∞`-aware form.

Every shipped preset satisfies `g > t`: KDE 21/8, GNOME 12/8, macOS 16/7,
Windows 17/6 (`native-theme/src/presets/kde-breeze.toml:146`, `:148`;
`adwaita.toml:151-152`; `macos-sonoma.toml:134-135`; `windows-11.toml:153`, `:155`).

### 6.6 D2 — slider thumb diameter

egui's chain, all in `widgets/slider.rs`:

```text
thickness  = ui.text_style_height(&TextStyle::Body).at_least(spacing.interact_size.y)  // :957-959
limit      = rect.height() (horizontal) == thickness                                  // :650-654, :881-884
handle_r   = limit / 2.5                                                              // :885
painted_r  = handle_r + visuals.expansion            [HandleShape::Circle]            // :815
```

so the filled disc's diameter is `0.8 · thickness + 2 · expansion`, and the
outline `fg_stroke` is tessellated **outside** that radius
(`PathStroke::from(stroke).outside()`, `epaint/src/tessellator.rs:1531`), so the
knob as painted is `2 · fg_stroke.width` wider than the disc. native-theme's
`thumb_diameter` is the whole knob, so the outer edge must lie at `d / 2`:
`handle_r + expansion + w = d / 2`, i.e. `expansion = (d − 0.8 · thickness) / 2 − w`.
`thickness` is not the atlas's alone to choose — the `at_least` floor at
`slider.rs:957-959` raises it to the Body row height `h` — but `Builder::build`
knows `h`: it is §6.15's `row`, the row height egui gives the Body text of
this variant, and the `Role::Slider` cell writes no text style, so a slider in
it lays out with exactly that `h` (`Ui::text_style_height`, `ui.rs:633-635`, is
`FontsView::row_height` of the resolved Body `FontId`). So the atlas computes
`thickness` as `slider.rs` will:

```text
let d = slider.thumb_diameter;
spacing.interact_size.y = if d.is_finite() {
    clamp_length(1.25 * d)                           // 0.8 * (1.25 d) == d
} else {
    base_y                                           // egui's own value; base_y: §6.7
};
let w = widgets.inactive.fg_stroke.width;            // every state's width (§6.1): finite, >= 0.0
let e = match row {                                  // §6.15's row: Some only when finite and > 0
    Some(h) if d.is_finite() => {
        let thickness = h.max(spacing.interact_size.y);                     // slider.rs:957-959
        ((clamp_length(d) - 0.8 * thickness) * 0.5).min(0.0) - w
    }
    _ => -w,                                         // no row height, or no finite diameter
};
for state in [noninteractive, inactive, hovered, active, open] {
    widgets[state].expansion = e;                    // disc radius handle_r + e, outline out to d / 2
}
visuals.handle_shape = HandleShape::Circle;          // style.rs:1237
```

Where the floor does not bite, `thickness = 1.25 · d`, the first term is `0`
and `e = −w`: the disc is `handle_r − w` and its outline reaches `handle_r`, so
the painted knob is `2 · handle_r = 0.8 · thickness = d` across, outline
included. Where it bites — a Body row taller than `1.25 · d`, which a
text-scaling factor above `1.0` brings about (§8.6) — the first term is
negative and shrinks the knob back to `d`, so the knob stays at
`slider.thumb_diameter` for every text size while the slider's own height
follows the text, as egui lays it out. The `.min(0.0)` only ever shrinks: a
diameter so large that `clamp_length` capped `interact_size.y` below `1.25 · d`
keeps the knob at `0.8 · thickness`, inside the rect egui allocated. The
**same** `expansion` goes to all five states, or the knob changes size on
hover, and one `w` serves all five because §6.1 gives every state
`inactive`'s `fg_stroke.width` — egui's `1.0` (`style.rs:1695`) unless the role
states a width; egui's stock `1.5` and `2.0` (`:1703`, `:1711`) would otherwise
have grown the knob on hover and press. `expansion` is the right correction term
because the slider's own readers are the two handle-paint sites (`slider.rs:815`,
`:825`) — the rail does not read it (`:774-775`) — and it is an `f32`
(`style.rs:1319`), so a negative value is representable with no narrowing.

**The cost is the value box.** `Slider::show_value` adds a `DragValue`, whose
`Button` frame reads the same `expansion`: `outer_margin` is `−expansion` and
`inner_margin` is `button_padding + expansion − bg_stroke.width`
(`widget_style.rs:162-165`), so its painted frame sits `−e` inside its allocated
rect on every side — `w`, 1 point at egui's `1.0`, wherever the floor does not
bite — while its text stays where it was.
While it is being edited, its `TextEdit` frame does the same, rounded to whole
points (`widgets/text_edit/builder.rs:765`, `:767`). Nothing else in a slider
scope reads `expansion`.

`HandleShape::Circle` is written because native-theme states a *diameter*
(`widgets/mod.rs:313-316`), which presumes a round knob on every platform it
supports; egui's default is `HandleShape::Rect { aspect_ratio: 0.75 }`
(`style.rs:1553`) and that `0.75` has no theme value behind it, so leaving it
would silently narrow the knob by a factor nothing in `ResolvedTheme` justifies.

**Boundaries.** Every step is total: `f32::max` and `f32::min` return the
other operand for a `NaN`, `h` and `interact_size.y` are finite, so the
difference cannot overflow, and `clamp_length` floors a negative `d` at `0`. `d == 0` ⇒ `interact_size.y == 0`; the slider
does not shrink, because the floor keeps `thickness` at `h` — only the scope's
other readers of `interact_size.y`, such as the value box, collapse — and
`e = −0.4 · h − w` makes the disc radius `−w ≤ 0`, so the tessellator draws
neither disc nor outline (`epaint/src/tessellator.rs:1494-1496`): a stated
zero diameter paints no knob. The same holds wherever `d ≤ 2 · w`. `NaN` or
`±∞` ⇒ `base_y`, egui's own `interact_size.y`, with a `Note::ValueSanitised`
(§7.2) — an infinite `interact_size.y` is not harmless (§6.7) — and `e = −w`,
the knob then `0.8 · thickness` across. A finite `d` from about `8.5e36` on,
where `1.25 · d` reaches `f32::MAX * GUI_ROUNDING` or overflows to `+∞`, is
capped by `clamp_length` just below that edge, so a lone slider in a `Window`
no longer trips the assert §6.7 bisected. Where `row` is `None` — an empty
Proportional chain, a face with `units_per_em` `0` (§6.15), or a Body size
whose rounded row is `0.0` or overflows (measured: sizes `1e-6`, `0.01` and
`1e-30` round to `0.0`; `3e37`, `1e38` and `f32::MAX` overflow — all pass
§8.5's test) — `e = −w`, the
value that is exact whenever the floor does not bite.

**Where the floor bites, measured.** At a text-scaling factor of `1.0` it
bites on no platform preset: `1.25 · d` (KDE 20→25, GNOME 20→25, Windows
18→22.5, macOS 21→26.25) exceeds the **measured** Body row heights — 15.34 px
for `kde-breeze`, 16.84 px for `adwaita`, 16.13 px for `windows-11` (each at 96
DPI) and 14.97 px for `macos-sonoma` at the 72 DPI native-theme uses on macOS
(19.94 px at 96 DPI), in both modes — `FontsView::row_height` of
`FontId::proportional` at the resolved `defaults.font.size`, on an egui 0.36.2
`Context` with `egui::FontDefinitions::default()`, rustc 1.98.1, 2026-09-25.
With the platform's own face, which feature `system-fonts` installs by default,
the Linux rows are 18.16 px (`kde-breeze`, Noto Sans) and 17.75 px (`adwaita`,
Adwaita Sans) (rationale §3.12's measurement), still below 25. A larger text-scaling
factor grows `h` with the Body size and not `d`, so the floor bites sooner,
taking the row height as proportional to the size: with egui's default faces
first on `windows-11`, from a factor of about `22.5 / 16.125 ≈ 1.40`; with the
platform's own face first on `kde-breeze`, from about `25 / 18.156 ≈ 1.38`, the
smallest factor measured (`adwaita` from about `25 / 17.75 ≈ 1.41`). The rows of
the Windows and macOS faces are not measured yet (§15). Where the floor bites,
the knob stays at `d` by the formula above.

**Collateral inside a slider scope, documented rather than compensated for.**
The value box also reads `interact_size` (`widgets/drag_value.rs:590`, `:593`,
`:640`), so a slider scope with `interact_size.y = 25` makes the value box 25
points tall.

### 6.7 D3 — spinner size

```text
let d = finite_or(spinner.diameter, base_y);       // base_y: the starting Style's own interact_size.y (egui's 18.0 on a default Style, style.rs:1460)
let m = finite_or(spinner.min_diameter, 0.0);      // egui has no minimum: 0.0 is max's identity on a length
spacing.interact_size.y = clamp_length(d.max(m));
```

`Spinner` allocates `vec2(size, size)` **exactly** (`spinner.rs:65-68`), so it
never shrinks and "minimum rendered size" collapses to a `max`. Each operand
passes `finite_or` **before** the `max`, which is load-bearing: `NAN.max(x) == x`
and `x.max(NAN) == x`, so a `NaN` would otherwise silently pick the other operand
instead of being reported. Each operand's fallback is egui's own value for what
it feeds: the diameter *is* the spinner's size, egui's `interact_size.y`; the
minimum is a floor egui does not have, and the floor egui does not have is `0.0`
— the one place the non-finite rule (§6) yields `0.0`, because `0.0` is egui's
value there.

**Boundaries.** Both zero ⇒ `radius = -2.0` (`spinner.rs:45`) and
`n_points = (-2.0_f32.round() as u32).clamp(8, 128)` — the `as u32` of a
negative float is `0` by Rust's saturating float-to-int rule, then
`.clamp(8, 128)` yields `8`. Each point is
`rect.center() + radius * vec2(cos, sin)` (`spinner.rs:50-54`), so they land on a
circle of radius `2.0` about the centre and `Shape::line` paints a small arc with
the hardcoded `Stroke::new(3.0, ..)` (`:58`). A zero diameter therefore still
shows a ring that no theme value can defeat — consistent with the
`interact_size.y − 1.0` shortfall recorded at the end of §5.5.

The eight points are **not** guaranteed distinct: `spinner.rs:49` is
`end_angle = start_angle + 240°.to_radians() * time.sin()`, so the whole spread
collapses to one point whenever `time.sin()` is `0.0` — exactly so for a host
that supplies `RawInput::time == 0.0`, and to within rounding near every multiple
of π. **Still no panic**, for two independent reasons: `n_points >= 8`, so
`Path::add_open_points`' `assert!(n >= 2, ..)`
(`epaint/src/tessellator.rs:387`) cannot fire; and coincident points give a
zero-length difference vector, which `Vec2::normalized` returns unchanged rather
than dividing by zero (`emath/src/vec2.rs:171-174`). The degenerate frame paints
nothing visible and the next frame recovers.

`±∞` or `NaN` in `diameter` ⇒ `base_y`, the `interact_size.y` egui's own
`Style` has (`18.0`, `style.rs:1460`), and `Note::ValueSanitised` (§7.2); in
`min_diameter` ⇒ no floor, and the same note. No platform states an infinite size, so this is
hostile input, and egui's own value is the one substitute nobody has to invent
— the rule an unstated size already follows (S2). Passing `+∞` through is
**not** harmless, measured on egui 0.36.2 with rustc 1.98.1 under debug
assertions: a `Spinner` or `Slider` whose scope has `interact_size.y = +∞`
trips `debug_assert!(!frame_rect.any_nan(), ..)` in a horizontal layout
(`egui/src/layout.rs:632`), and inside a `Window` the widget rect becomes `NaN`
and trips `debug_assert!(!w.rect.any_nan(), ..)` (`egui/src/context.rs:1254`);
`egui/src/layout.rs:632` also fires in a `Grid` and in a `ui.horizontal` row inside a top
`Panel`; in a plain vertical layout, a `ScrollArea` and a top `Panel`'s own
vertical layout the passes completed. The same run bounds a finite value too,
and the bound is egui's own rounding: `Layout::next_frame` ends in
`frame_rect.round_ui()` (`egui/src/layout.rs:635`), which is
`(x / GUI_ROUNDING).round() * GUI_ROUNDING` with `GUI_ROUNDING = 1.0 / 32.0`
(`emath/src/gui_rounding.rs:18`, `:59`; re-exported as `egui::emath::GUI_ROUNDING`,
`egui/src/lib.rs:438`), so an edge above `f32::MAX * GUI_ROUNDING`
(≈ `1.0633823e37`) rounds to `+∞`, the cursor follows, and the next `Rect::size`
(`emath/src/rect.rs:341`, reached from `egui/src/layout.rs:428`) is `∞ − ∞`.
Bisected, one `Spinner` or `Slider` in a `Window` passes at exactly that value and
trips `context.rs:1254` at the next `f32`, at screen sizes 200, 1280, 10 000 and
`1e30` alike; re-measured at 1280 with rustc 1.95, both also pass at
`clamp_length`'s ceiling, one `f32` below. **No finite bound is safe on every layout path, and none can be**:
the limit is on an edge, the sum of every length laid out before it — two
stacked `Spinner`s in a `Window` trip it from `2^122` each. That is why the
connector caps each length it writes with `clamp_length` (§7.2), strictly below
`f32::MAX * GUI_ROUNDING`, and states sums of lengths as the residual they are.
Both asserts are `debug_assert!`s; a release build completed every pass. T4's
`1e30` is about ten million times below the limit and stays the hostile value
asserted to pass. A negative finite value ⇒ `0.0`, by `clamp_length`'s floor.

### 6.8 D4 — corner radii with no native source

`ResolvedScrollbarTheme` and `ResolvedSliderTheme` have no radius field, but
egui requires one at `scroll_area.rs:1508`, `:1517` and `slider.rs:772`.

```text
let fit = |cap: f32| {                                  // a non-finite radius passes through
    let r = defaults.border.corner_radius;               // untouched: to_corner_radius then keeps
    if r.is_finite() { r.max(0.0).min(cap) } else { r }  // the starting Style's radius (§7.2)
};

// Role::Scrollbar — no cap: epaint clamps to half the painted width, tessellator.rs:549, :638-642
for st in [inactive, hovered, active] {
    widgets[st].corner_radius = to_corner_radius(widgets[st].corner_radius, defaults.border.corner_radius);
}

// Role::Slider (rail) — the rail height the scope carries (§5.5)
widgets.inactive.corner_radius =
    to_corner_radius(widgets.inactive.corner_radius, fit(spacing.slider_rail_height * 0.5));
```

The radius is a `ResolvedTheme` value. The scrollbar's needs no cap: only the
tessellator reads it, and it clamps every radius to half the painted rect's
smaller side (`epaint/src/tessellator.rs:549`, `:638-642`), while a negative
one saturates at `0` in `to_corner_radius` (§7.2). Outside a `Role::Scrollbar`
scope — every unscoped `ScrollArea`, which takes its colours from the base
style (§5.5) — the handle and trough take the base style's radius,
`button.border.corner_radius` (§5.9), clamped by epaint the same way. The
slider keeps its cap because its rail radius is also trailing-fill geometry:
the fill's end is placed at the handle centre plus that radius (`slider.rs:792`,
`:795`). The cap is the width egui paints — the rail is `slider_rail_height`
thick (`slider.rs:770-771`) — read back from the scope's `Spacing`, which §6's
rule has already made finite, so it never inherits a non-finite input. A `NaN` or `±∞` radius leaves each `corner_radius` at the value the
starting `Style` has there (§6's rule) and emits `Note::ValueSanitised`. The `* 0.5` is the geometric
definition of a fully rounded end, not a theme constant — on the rail itself a
radius larger than half the thickness could not be honoured anyway, since
epaint re-clamps in the tessellator (`epaint/src/tessellator.rs:638-642`).

### 6.9 D5, D6, D7 — the remaining indicator fallbacks

```text
// D5  scrollbar pressed thumb
widgets.active.bg_fill = to_color32(
    scrollbar.thumb_active_color.unwrap_or(scrollbar.thumb_hover_color));

// D6  slider hovered and pressed handle
let hover = slider.thumb_hover_color.unwrap_or(slider.thumb_color);
widgets.hovered.bg_fill = to_color32(hover);
widgets.active.bg_fill  = to_color32(hover);      // "active := hovered", §6.1

// D7  splitter, all three lines; the first argument is the stroke the starting Style has there
widgets.noninteractive.bg_stroke = to_stroke(widgets.noninteractive.bg_stroke, splitter.divider_color, splitter.divider_width);
widgets.hovered.fg_stroke        = to_stroke(widgets.hovered.fg_stroke, splitter.hover_color, splitter.divider_width);
widgets.active.fg_stroke         = widgets.hovered.fg_stroke;   // no native drag colour
```

`to_stroke` (§7.2) applies §6's rule: a `NaN` or `±∞` `divider_width` keeps each stroke's own width — egui's `1.0`
for `noninteractive.bg_stroke` (`style.rs:1686`, `:1731`) and §6.1's `inactive` width
for `hovered.fg_stroke` — with the colours still written, and emits
`Note::ValueSanitised`; a negative width is `0.0`.

D6 is the one route by which `slider.thumb_color` reaches the screen: where
`thumb_hover_color` is `None` it fills the hovered and pressed handle, while
the resting handle is the rail's `inactive.bg_fill`, `track_color`'s (I1, §5.5).

D5 and D6 write thumb colours, which §6.1 writes as given rather than
compositing them over `thumb_color`, as iced does: egui paints the thumb over a
rail it also paints, the reason C17 gives for row highlights.

`ResolvedSplitterTheme` has no pressed/dragging colour, and egui uses a third
stroke while dragging (`panel.rs:906`, `:1032`), so `active := hovered` is the
only non-inventing choice. The `0.0 < stroke.width` gate at `panel.rs:915` and
`:1038` means a `divider_width` of `0.0` correctly makes the divider invisible
in all three states.

### 6.10 R-BAR — toolbar bar height

```text
total_v := f32::from(frame.inner_margin.top)  + f32::from(frame.inner_margin.bottom)
         + 2.0 * frame.stroke.width
         + f32::from(frame.outer_margin.top)  + f32::from(frame.outer_margin.bottom)
         + f32::from(i8_from_f32_saturating(widgets.noninteractive.bg_stroke.width))
           // separator room egui reserves while show_separator_line is on, its default (panel.rs:298, :952-965)

spacing.interact_size.y := match toolbar.bar_height {
    Some(h) if h.is_finite() => clamp_length(h - total_v),
    Some(_) => base_y,      // NaN, ±∞: egui's own value (§6); base_y: §6.7
    None    => unchanged,   // S2: no stated height; egui's own sizing stands
}
```

`total_v` is exactly `Frame::total_margin().sum().y` (`frame.rs:327-331`) of
the frame `Panel::resolve_frame` builds (`panel.rs:947-965`, called at `:1071`),
which is the term egui adds back at `panel.rs:1072`
(`interact_size[axis] + frame.total_margin().sum()[axis]`, with `axis == 1` for
Top and Bottom, `panel.rs:85-86`). `frame` is the connector's own toolbar
`Frame`, whose `inner_margin` is `to_margin(base, &toolbar.border.padding)`
(§7.2) with `base` the margin egui's own top panel frame has,
`Margin::symmetric(8, 2)` (`Frame::side_top_panel`, `frame.rs:185-189`) — so a
padding side the theme leaves unstated keeps egui's `2`, and `total_v` counts
whatever the frame really carries. `resolve_frame` then raises that frame's
`outer_margin` on the resize side by `noninteractive.bg_stroke.width.round() as i8`
with `saturating_add` (`panel.rs:961-965`), which the last term adds: the width
of the `Role::Toolbar` cell, `toolbar.border.line_width` (§5.6), and
`i8_from_f32_saturating` rounds it as egui does (§7.2). An application that
calls `Panel::show_separator_line(false)` gets a bar that many points shorter.
Both terms are read from the `Ui` the panel is
shown in — `resolve_frame(parent_ui)`, `outer_size(parent_ui)` (`panel.rs:708`,
`:721`) — so R-BAR takes effect only when the `Panel` is shown inside the
`Role::Toolbar` scope, not when the scope sits inside its contents.

| input | behaviour |
|---|---|
| `None` | `interact_size.y` is not written, so the toolbar scope keeps egui's value (`18.0` in a default `Style`, `style.rs:1460`) and the panel sizes itself by egui's own formula at `panel.rs:1072`. It is what KDE and GNOME state: no toolbar height (`native-theme/src/model/widgets/mod.rs:447-449`) |
| `NaN`, `±∞` | egui's own `interact_size.y` (`18.0` in a default `Style`, `style.rs:1460`), with a `Note::ValueSanitised`, so the panel's formula at `panel.rs:1072` sizes the bar exactly as it would with no stated height. An infinite `interact_size.y` would trip egui's debug asserts (§6.7) |
| negative | `0.0`, by `clamp_length`'s floor |
| `Some(0.0)` | a stated zero: `interact_size.y == 0.0`; widgets fall back to their content height |
| `bar_height < total_v` | `clamp_length`'s floor; never a negative `interact_size.y` |
| huge | finite, so no narrowing and no cast (`i8 → f32` is exact); `clamp_length` caps it below `f32::MAX * GUI_ROUNDING`, where a widget in the scope would trip egui's debug asserts inside a `Window` (§6.7); the panel itself clamps its size to the available rect (`panel.rs:720-722`) and completed at `f32::MAX` |

Every row but `None` describes a stated `Some(h)`.

**Why the guards are here at all.** The guards are load-bearing: range checks
— for a `soft_option` such as `toolbar.bar_height` only on a stated value
(`native-theme-derive/src/gen_ranges.rs:101-106`) — run only inside
`validate()`, which a deserialised or field-mutated `ResolvedTheme` never
enters (§7.1).

**Three residuals, stated because they are real.** A persisted `PanelState`
wins: `outer_size` reads the stored rect first (`panel.rs:1066-1067`) and only
falls back to the formula at `:1068-1073`; the panel stores its realized rect
every frame it is not being dragged (`:882-898`), so the formula sizes the first
frame and the realized size — content plus frame, a `ui.horizontal` row starting
at `interact_size.y` (`ui.rs:2376-2378`) — every frame after, and a user who has
resized the toolbar keeps their size. `clamp_to_range(raw, self.outer_size_range)` (`panel.rs:1074`)
with `Rangef::new(20.0, f32::INFINITY)` for Top and Bottom (`:289`) imposes a
hardcoded 20.0-point floor that no `Style` value can defeat. And
`Panel::exact_size` / `default_size` (`:405`, `:369`) override the formula
entirely; an application that sets either passes the platform's own height,
`toolbar.bar_height` read from `ResolvedTheme`. It is an `Option<f32>` (S2):
`None` says the platform states no height, so the application sets neither and
leaves egui's sizing in place.

### 6.11 R-ARROW — expander arrow size

```text
spacing.icon_width_inner := if expander.arrow_icon_size.is_finite() {
    clamp_length(expander.arrow_icon_size * (4.0 / 3.0))   // a product that overflows to +∞ is capped too
} else {
    unchanged                                              // NaN, ±∞: egui's own value (§6)
}
```

The painted triangle's bounding box is the icon rect scaled by `0.75` —
`Rect::from_center_size(rect.center(), vec2(rect.width(), rect.height()) * 0.75)`
(`collapsing_header.rs:342`) — and that icon rect is the **small** rect from
`Spacing::icon_rectangles`, sized `Vec2::splat(icon_width_inner)`
(`style.rs:477-478`), selected at `collapsing_header.rs:585`. Inverting gives
`arrow_icon_size / 0.75 = arrow_icon_size · 4/3`. The `0.75` is read from egui's
source, not invented.

| input | behaviour |
|---|---|
| `NaN`, `±∞` | `icon_width_inner` keeps the value the starting `Style` has — egui's `8.0` (`style.rs:1467`) — with a `Note::ValueSanitised`. The finiteness test comes first: `clamp_length` would turn a `NaN` into `0.0` and `±∞` into its floor or cap, none of them egui's value |
| negative finite | `0.0`, by `clamp_length`'s floor. That floor is what keeps a negative extent out of `Vec2::splat(icon_width_inner)` (`style.rs:477-478`) and therefore out of `Rect::from_center_size` at `collapsing_header.rs:342` |
| `0.0` | `icon_width_inner == 0.0`; three coincident points tessellate to nothing (`collapsing_header.rs:351`) — no panic |
| finite, from about `8e36` | `clamp_length`'s cap, just below `f32::MAX * GUI_ROUNDING`, including where `x · 4/3` overflows to `+∞`, so nothing infinite reaches `icon_rectangles`. Load-bearing: an infinite `icon_width_inner` trips emath's `Vec2` `NaN`-comparison assert from `epaint/src/tessellator.rs:400` (§7.4), while `f32::MAX` itself completed in vertical, horizontal, `ScrollArea`, `Grid`, top-`Panel` and `Window` layouts, so the lower cap is inside what was measured |

No `as` cast, no `unwrap`, no index, no division by a runtime value.

**Why the guards are here at all.** The guards are load-bearing: range checks
run only inside `validate()`, which a deserialised or field-mutated
`ResolvedTheme` never enters (§7.1).

**Secondary constraint, documented not compensated.** The arrow is re-centred at
`rect.left() + ui.spacing().indent / 2.0` (`collapsing_header.rs:586-588`), so
an arrow wider than `indent` overlaps the label. native-theme has no expander
indent property, so `Spacing::indent` is left inherited; baking in egui's `18.0`
(`style.rs:1459`) is forbidden. `icon_width_inner` also sizes the check mark
(`widget_style.rs:180`) and the radio dot (`radio_button.rs:87`, `:101`); the
`Role::Checkbox` scope replaces the style with its own cell, which holds egui's
`8.0` or the inset below, so the expander's value reaches only a checkbox or radio button laid in the expander scope with no Checkbox
scope of its own.

**The same field in the Checkbox scope: the mark's inset.** `checkbox_style`
hands `icon_width_inner` to the check mark as `check_size`
(`widget_style.rs:180`), and the mark spans a `check_size` square centred in the
`icon_width` box (`widgets/checkbox.rs:132-133`, drawn at `:153-155`), so
`checkbox.border.padding` — the mark's inset in the indicator
(`docs/platform-facts.md` §2.5: Adwaita `check { padding: 3px }`) — is carried
as the box less the mean of the two pairs' insets:

```text
// «Checkbox» only; the base style's icon_width_inner is egui's (§5.9)
// l, r, t, b: checkbox.border.padding.{left, right, top, bottom}
spacing.icon_width_inner := match (l, r, t, b) {
    (Some(l), Some(r), Some(t), Some(b))
        if l.is_finite() && r.is_finite() && t.is_finite() && b.is_finite() =>
        clamp_length(spacing.icon_width - (f32::midpoint(l, r) + f32::midpoint(t, b))),  // the cell's icon_width, after its own rule; §7.2's midpoint, overflow-free per pair
    (Some(_), Some(_), Some(_), Some(_)) => unchanged,   // NaN, ±∞ in a side, all four stated: egui's own value (§6)
    _ => unchanged,   // a side unstated: egui's 8.0 (style.rs:1467) stands
}
```

`spacing.icon_width` is read back from the cell, where it is already
`checkbox.indicator_width` after its own rule (§5.3), so the mark's box and the
indicator it sits in come from one value. Stated leaves, a mean and one
subtraction: no ratio and no invented number. On `adwaita`, the one bundled
preset that states the padding, `indicator_width` 20 and padding 3 on each side
in both variants (`native-theme/src/presets/adwaita.toml:124`, `:135-136`, `:452`,
`:463-464`; `native-theme/src/presets/adwaita-live.toml:70-71`, `:273-274` repeat the
padding) give 14 where egui draws 8. libadwaita's box is a 14-point content box
(`min-width: 14px`, the `-gtk-icon-size` its mark is drawn at) inside `padding: 3px`,
with no CSS border (libadwaita 1.10.0 `_checks.scss:20-26`). So 20 is the whole box,
which is what egui's `icon_width` is, and 14 is the native mark box.
`docs/platform-facts.md:1212` records that 14 content width for GNOME, not the 20
box its `:980` defines. The box is square (`checkbox.rs:133`) and
egui gives it one inset, so the formula takes the mean of the two pairs' —
§5's pair rule applied across the two axes: exact where the pairs agree, as on
`adwaita`, and where they differ each axis is off by half the pairs'
difference, the smallest worst-axis error one inset can have. A `RadioButton` in the scope takes the same inset: its dot's
radius is a third of that box (`radio_button.rs:87`, `:101`).

| input | behaviour |
|---|---|
| a side `None` | `icon_width_inner` keeps the value the starting `Style` has, egui's `8.0`; nothing is reported, since no inset is stated |
| `NaN`, `±∞` in a side, all four stated | egui's `8.0`, with a `Note::ValueSanitised` for that leaf |
| a mean inset larger than the box | `0.0`, by `clamp_length`'s floor, which keeps a negative extent out of `Rect::from_center_size` (`checkbox.rs:132-133`) |
| finite sides whose inset overflows | each pair's midpoint is finite (§7.2), so only their sum can overflow, to `±∞`; `clamp_length` floors or caps the difference, as every formula overflow is (§7.2) |

### 6.12 Combo-box arrow area

egui reserves `icon_spacing + icon_width` for the arrow (`combo_box.rs:355`,
`:360`), so:

```text
spacing.icon_spacing := match combo_box.arrow_area_width {
    Some(w) if w.is_finite() => clamp_length(w - spacing.icon_width),   // the cell's icon_width
    Some(_) => unchanged,   // NaN, ±∞: the base style's gap (§6)
    None    => unchanged,   // S2: no stated width; the base style's gap stands
}
```

`arrow_area_width` is a `soft_option` (`native-theme/src/model/widgets/mod.rs:743-745`),
`None` where the theme states no width — GNOME, macOS, iOS, Material and the
community colour-scheme presets (`:738-742`). Then the `Role::ComboBox` scope
leaves `icon_spacing` at the value it inherits, the base style's `icon_spacing`,
`checkbox.label_gap` (§5.9), and no gap is invented.

`spacing.icon_width` is read back from the cell, where it is already
`combo_box.arrow_icon_size` after its own rule (§5.4) — a finite length — so
the width egui reserves, `icon_spacing + icon_width`, is exactly `w` for any
icon size up to `w`.

**Boundaries** of a stated `Some(w)`. `w` below the cell's `icon_width` ⇒ `0.0`
(arrow flush against the label, never a negative gap), by `clamp_length`'s
floor. A non-finite `w` keeps the base style's `icon_spacing`,
`checkbox.label_gap` (§5.9), with a `Note::ValueSanitised`, never a `0.0` gap. A
difference that overflows is floored by `clamp_length`, as every formula
overflow is (§7.2).

The right frame inset `button_padding.x` (`combo_box.rs:339`, `:443`) is
deliberately not subtracted. Under §5's pair rule it is the mean of the combo
box's left and right padding, each side plus its border's line width
(`padding_with_border`, §7.2): `(left + right) / 2 + line_width`. It is an `f32`,
so nothing is rounded, and the trigger's width beside the text,
`2 · button_padding.x + icon_spacing + icon_width`,
equals the platform's `border + padding.left + padding.right + arrow_area_width + border`
exactly: `7 + 7 + 26 + 12 = 1 + 12 + 0 + 38 + 1` on `windows-11`,
`4 + 4 + 0 + 20 = 1 + 6 + 0 + 20 + 1` on `kde-breeze`
(`native-theme/src/presets/windows-11.toml:328-329`, `:340-341`, `:43`;
`kde-breeze.toml:286-287`, `:295-296`, `:43`). Subtracting it would make the arrow zone
alone exact and the trigger `button_padding.x` narrower than the platform's.

**Truth in advertising.** egui has no separate clickable arrow *zone*: the whole
`outer_rect` senses the click (`combo_box.rs:446`), so `arrow_area_width` only
ever controls the reserved *layout* width.

### 6.13 Border opacity fold

egui has no stroke-alpha multiplier — `Stroke` is `{ width: f32, color: Color32 }`
and nothing else (`epaint/src/stroke.rs:13-16`). Applied once, at conversion
time, on the **straight** colour before the premultiplied conversion:

```text
let a = u8_from_f32_saturating(f32::from(c.a) * unit_interval(finite_or(defaults.border.opacity, 1.0)));
Color32::from_rgba_unmultiplied_const(c.r, c.g, c.b, a)
```

This is `to_color32_with_opacity` (§7.2). A `NaN` or `±∞` opacity is replaced by
`1.0` — egui's own value for this sink, since egui folds no opacity into a
stroke and paints it at its colour's own alpha — with a `Note::ValueSanitised`,
never by `0.0`, which would erase every border. `unit_interval` then clamps a
finite value to `0.0..=1.0`; `f32::from(u8)` is lossless; the product lies in
`0.0..=255.0`, so `u8_from_f32_saturating` never actually saturates.

**The multiplier is always `defaults.border.opacity`.** It is the only border
opacity the model has: `ResolvedDefaultsBorder` carries `opacity`
(`native-theme/src/model/border.rs:222`), and a widget's `ResolvedWidgetBorder`
carries none (`:233-246`), so there is no widget-level value to prefer or to
fold in. `opacity == 0.0` yields
`Color32::TRANSPARENT` — legal for `bg_stroke` and `weak_bg_fill`
(`style.rs:1297-1300`) but **illegal for `bg_fill`** (`:1292-1294`), so the fold
is never applied to a `bg_fill`.

### 6.14 Shadow gate

```text
if <owner>.border.shadow_enabled {
    Shadow { color: to_color32(defaults.shadow_color), ..base }   // base = egui's own
} else {
    Shadow::NONE                                                   // epaint/src/shadow.rs:40-45
}
```

`<owner>` is the widget whose `border.shadow_enabled` row claims that sink in §5
— `window` for `visuals.window_shadow`; `defaults`, `dialog`,
`popover` or `tooltip` for `visuals.popup_shadow` and their own frames
(§5.1, §5.2, the claimant lists in §5.11) — and the whole
block is §7.2's `to_shadow(base, defaults.shadow_color, <owner>.border.shadow_enabled)`.

`base` is egui's own shadow for the scheme — `window_shadow`
`Shadow { offset: [10, 20], blur: 15, spread: 0 }` (`style.rs:1520-1525` dark,
`:1582-1587` light) and `popup_shadow` `{ offset: [6, 10], blur: 8, spread: 0 }`
(`:1534-1539`, `:1593-1598`) — for the `Dialog`, `Popover` and `Tooltip` frames
too, never the base style's `popup_shadow`, which `defaults.border.shadow_enabled`
has already gated. **Only the colour is replaced.** native-theme
supplies no offset, blur or spread anywhere in the model — the only shadow field
of `DefaultsBorderSpec` and `WidgetBorderSpec` is `shadow_enabled: Option<bool>`
(`native-theme/src/model/border.rs:39`, `:89`), resolved to
`shadow_enabled: bool` (`native-theme/src/model/border.rs:224` on
`ResolvedDefaultsBorder`, `:241` on `ResolvedWidgetBorder`) — so populating them would be
invention. `shadow_enabled == false` yields exactly `Shadow::NONE`, never a
transparent-but-blurred shadow.

### 6.15 Line height

native-theme's `defaults.line_height` is a dimensionless **multiplier**
(`native-theme/src/model/resolved.rs:76-77`). egui's
`Spacing::extra_text_line_spacing` (`style.rs:424`, new in 0.36) is an
**absolute additive delta in points**. The conversion needs the row height egui
lays Body text out with, and `Builder::build` computes that row height itself,
from the installed face's bytes, with epaint's own arithmetic — no pass, no
`Context`, no `Fonts`. epaint measures a family by the first face of its chain
(`epaint/src/text/font.rs:697-702`, called from `FontsView::row_height`,
`epaint/src/text/fonts.rs:865-875`), reads that face's metrics once, unscaled and
at the default variation location (`epaint/src/text/font.rs:397-400`), scales
them by `font_size · tweak.scale / units_per_em` (`:561`, `px_scale_factor` at
`:215-218`), rounds ascent, descent and line gap each with `round_ui`
(`:563-565`), and sums them (`:587`). Per variant `t`, with `defs` the
definitions the atlas installs — `fonts::font_definitions` of the atlas's light
`ResolvedTheme` and the plan, or `egui::FontDefinitions::default()` for an atlas built with no plan, one for both variants, because egui holds one `FontDefinitions` per `Context`
(§8.2, §10.3 step 1) — and `prefs` the builder's accessibility preferences:

```text
let s    = scaled_text_size(t.defaults.font.size, prefs);
let size = if s.is_normal() && s > 0.0 { s } else { egui_body }; // the TextStyle::Body size the atlas writes (§8.5, §8.6); egui_body: egui's own
let row  = defs.families.get(&FontFamily::Proportional)
    .and_then(|chain| chain.first())
    .and_then(|name| defs.font_data.get(name))                  // the chain head: the plan's face, else the base's head — egui's Ubuntu-Light for the default base (fonts.rs:546)
    .and_then(|f| skrifa::FontRef::from_index(&f.font, f.index).ok().map(|face| (face, f.tweak.scale)))
    .map(|(face, scale)| {
        let m = face.metrics(Size::unscaled(), LocationRef::default());      // font.rs:397-400
        let k = size * scale / f32::from(m.units_per_em);                   // font.rs:561, :215-218
        (m.ascent * k).round_ui() - (m.descent * k).round_ui() + (m.leading * k).round_ui()   // font.rs:563-565, :587
    })
    .filter(|r| r.is_finite() && *r > 0.0);                     // so `row` is Some only when finite and > 0 (§6.6)
let want = t.defaults.line_height * size;
spacing.extra_text_line_spacing := match row {
    Some(row) if t.defaults.line_height.is_finite() => clamp_length(want - row),
    _ => 0.0,                                                    // egui's own value, style.rs:1465
};
```

The value is written into the variant's base style, and every role cell of that
variant carries it from there (§3.4), so a `native_scope` hands its widgets the
same leading as the unscoped UI, and a `ThemeAtlas` the application holds
carries it too. `round_ui`
is emath's public `GuiRounding` (`emath/src/gui_rounding.rs:58-60`), and
`skrifa` is already a direct dependency at epaint's own version (§8.2).

**Verified, not assumed** — rationale §3.12 records the comparison with a live
`Context`'s `FontsView::row_height`; §13 T5 keeps it true.

**Nothing at run time changes it.** The row height does not depend on
`pixels_per_point`: that enters only the rasterising scale
(`epaint/src/text/font.rs:567-568`), not the row (`:561-565`), and the
measurement gave the same row at `pixels_per_point` 1 and 2 (rationale §3.12). egui's zoom is
a `pixels_per_point` factor (`egui/src/context.rs:455`), so zooming needs no
recomputation. Weight needs none either: `row_height` passes
`VariationCoords::default()` (`epaint/src/text/fonts.rs:870-872`) and the metrics
are read at the default location. The size, the line height, the text-scaling
factor and the face are all `Builder` inputs, so any change to them is a new
atlas, built with its own value. An application that later replaces the fonts
itself with `ctx.set_fonts` measures against a face the atlas never saw; it
registers its face through `FontPlan::face` (§4.9) instead.

**Boundaries.** Every fallback is `0.0`, and here `0.0` *is* egui's own value
for the sink (`style.rs:1465`), so §6's rule holds. A face that fails to parse
never reaches this point — it was dropped before `defs` was built (§8.2) — and
the `from_index` failure path above only keeps the computation total. An empty
Proportional chain is `None`; a `units_per_em` of `0` makes `k` non-finite and
the `filter` drops the row (unreachable once §8.2 drops such a face; kept for
totality): this is §6's one division by a value read at
run time, and it is epaint's own (`epaint/src/text/font.rs:215-218`), kept so
the result is epaint's to the bit. A non-finite `line_height` keeps egui's
`0.0` (§6's rule); a product that overflows is capped or floored by
`clamp_length` (§7.2).

**The floor at `0.0`, and what it costs on the platforms.** `clamp_length` floors
a negative difference at egui's `0.0`. egui documents no negative value — its
own settings slider clamps the field to `0.0..=20.0` (`style.rs:2024`) and the
field's whole doc is "Additional vertical spacing between lines of text."
(`style.rs:423`). Measured with the platform's own face, the difference is
slightly negative on both Linux presets (rationale §3.12), so the floor gives
egui's `0.0` there. The same measurement with the macOS and Windows Body faces is an
*Open verification item*.

**Three losses that must be stated, not buried.** (a) One global `f32` cannot be
simultaneously correct for `Small` (9), `Body` (13), `Button` (13), `Heading`
(18) and `Monospace` (13); the delta is exact for `Body` only. (b) It is read at
exactly two functional sites — `WidgetText::into_galley_impl`'s `Self::Text` arm
(`widget_text.rs:775-776`) and `TextEdit` (`text_edit/builder.rs:489-490`). (c)
Consequently `ui.label("x")` and `ui.button("x")` honour it, but
`ui.label(RichText::new("x"))` does **not**: the `RichText` arm
(`widget_text.rs:791`) delegates to `RichText::into_layout_job` (`:384-392`),
which propagates only `RichText`'s own `line_height` field (`:30`, builder
`:174`) and never reads `style.spacing`. `LayoutJob` and pre-built `Galley`
ignore it too.

The four per-role line heights are exact and lossless through
`text_role_line_height()` + `RichText::line_height(Some(..))`
(`widget_text.rs:174`), which is the only exact per-role mechanism egui has.

### 6.16 `layout.widget_gap`

```text
spacing.item_spacing = match layout.widget_gap {
    Some(g) if g.is_finite() => Vec2::splat(clamp_length(g)),
    _                        => base.spacing.item_spacing,   // egui's vec2(8.0, 3.0)
};
```

`None`, `NaN` or `±∞` keep egui's default — **not** `0.0`, because a zero gap
is a real visual regression and is not what "unspecified" means; a non-finite
gap also emits `Note::ValueSanitised` (§6's rule). A finite gap is floored at
`0.0` and capped by `clamp_length`, as every length is.

**Information the source does not have.** native-theme gives one number; egui
wants an asymmetric `Vec2` whose own default is asymmetric (`8.0` horizontal,
`3.0` vertical, `style.rs:1455`). `Vec2::splat(g)` is the only non-inventing
choice, and it *will* change vertical rhythm relative to stock egui.

### 6.17 The three structural constants

Exactly three *kinds* of numeric literal are legitimate in the mapping itself.
**Every other numeric literal in mapping code is a bug**, with the explicitly
enumerated exemptions below.

| constant | where | why it is not a theme value |
|---|---|---|
| `1.0` | `Visuals::disabled_alpha` in `RoleVariant::Disabled` (§6.3); the non-finite fallback of the border-opacity fold (§6.13) | the multiplicative identity: it neutralises `Painter::multiply_opacity`, and it is the fold egui itself applies to a stroke — none |
| `0.0`, `1.0`, `-128.0`, `127.0`, `127.5`, `-128.5`, `255.0` | the clamp bounds and the saturation predicate inside `convert` (§7.2) | the representable range of the destination type (`u8`, `i8`, an opacity), and the two values `round()` carries outside the `i8` range |
| `0.0` | every `.max(0.0)` floor on a length or gap, including `clamp_length`'s (§6.5–§6.12, §6.15, §6.16), `denan`'s image of `NaN` (§7.2), and the fallbacks where `0.0` is egui's own value — `spinner.min_diameter`'s non-finite one (§6.7) and every fallback of `Spacing::extra_text_line_spacing` (§6.15); the `.min(0.0)` that keeps §6.6's correction a shrink; §6.5's `floating_allocated_width = 0.0`, `ScrollStyle::floating()`'s own (`style.rs:643-653`); and the sign tests against `0.0` (§6.15, §6.18) | a length is never negative, so `0.0` is the lower end of the destination's meaningful range, not a theme value; egui's own settings UI floors `extra_text_line_spacing` at the same `0.0` (`style.rs:2024`) |

The exemptions, in full, because a rule that its own formulas violate is worse
than no rule. None of these is a constant *of this crate*: each is either read
out of egui's own source, or the reciprocal of one that is, or a bare geometric
identity, and each is cited at its use site.

| exempt | where | why |
|---|---|---|
| `0.75` and its reciprocal `4.0 / 3.0` | §6.11 | egui's own triangle scaling, `collapsing_header.rs:342` |
| `2.5`, `0.8` and its reciprocal `1.25` | §6.6 | egui's own handle-radius chain, `slider.rs:885` |
| `0.5` | §6.5, §6.6, §6.8 | the geometric definition of a midpoint or of a fully rounded end |
| `2.0` | §6.10 | a stroke has two sides; the term egui itself adds back at `panel.rs:1072` via `Frame::total_margin` |
| `f32::MAX * GUI_ROUNDING`, taken one `f32` step down | `clamp_length` (§7.2) | egui's own rounding bound: `GUI_ROUNDING` is emath's constant (`emath/src/gui_rounding.rs:18`), and above the product `round_ui` overflows to `+∞` (§6.7) |
| `0.0` and `ScrollAnimation::none()` | `Builder::accessibility` under `reduce_motion` (§4.3) | the platform's reduced-motion preference means "no animation"; `0.0` is the duration egui treats as reaching the end value at once (`egui/src/animation_manager.rs:56-60`, `:88-93`), and `ScrollAnimation::none()` is egui's own constructor for the same thing |

### 6.18 The focus ring

egui has no focus-ring value to write: keyboard focus is folded into `active`
(§6.1), and nothing in `Visuals` means "focus outline". The ring is therefore
**painted**, always, by the install plugin's `on_end_pass(&mut self, ui: &mut Ui)`
(`egui/src/plugin.rs:32`). `on_end_pass` runs after the application's UI in the
same pass (`context.rs:811-813`), so the ring is drawn over the widget it
surrounds.

**Built once, per variant, at `Builder::build`** from the three leaves:

```text
let (c, w, o) = (defaults.focus_ring_color, defaults.focus_ring_width, defaults.focus_ring_offset);
ring := if !w.is_finite() || !o.is_finite() {
    None                                   // egui's own value: no ring; Note::ValueSanitised per leaf
} else if w <= 0.0 {
    None                                   // a stated zero width: no ring, as gpui's `focus_ring = width > 0.0`
} else {
    Some(FocusRing { stroke: Stroke::new(clamp_length(w), to_color32(c)), offset: o })
};
```

`o` is kept signed and unclamped: it is the "gap between element edge and focus
indicator" (`native-theme/src/model/resolved.rs:140-141`), negative meaning
inside the edge — libadwaita's `-2` px and macOS's measured `-1` px
(`docs/platform-facts.md:1106`). gpui gates its ring on the same `> 0.0` test
(`connectors/native-theme-gpui/src/lib.rs:198`).

**Painted every pass**, in this order, doing nothing at the first `None`:

```text
let atlas = ThemeAtlas::from_ctx(ctx)?;                                // ctx.data, every pass
let ring  = atlas.focus_ring(ctx.theme())?;                            // the active variant
if !ctx.input(|i| i.focused) { return; }                               // input_state/mod.rs:315: the window has OS focus
let id    = ctx.memory(|m| m.focused())?;                              // memory/mod.rs:893-895
let wr    = ctx.viewport(|v| v.this_pass.widgets.get(id).copied())?;   // context.rs:3963-3965, widget_rect.rs:127-129
let (edge, radius) = focus_shape(ctx, id)                               // register_focus_shape, this pass (§4.3)
    .unwrap_or((wr.rect, scope_radius(ctx, &wr)                        // the innermost recorded scope (below)
        .unwrap_or(ui.style().visuals.widgets.active.corner_radius))); // `ui`: the root `Ui` on_end_pass receives
let rect  = edge.expand(ring.offset);                                  // emath/src/rect.rs:193-195; negative shrinks
if !rect.is_positive() { return; }                                     // an inset larger than the widget
let grow  = |c: u8| u8_from_f32_saturating(f32::from(c) + ring.offset);
let r     = CornerRadius { nw: grow(radius.nw), ne: grow(radius.ne), sw: grow(radius.sw), se: grow(radius.se) };
ctx.layer_painter(wr.layer_id)                                         // context.rs:1587-1590
   .with_clip_rect(wr.interact_rect.expand((ring.offset + ring.stroke.width).max(0.0)))   // painter.rs:71-75
   .rect_stroke(rect, r, ring.stroke, StrokeKind::Outside);            // painter.rs:406-414
```

* **Geometry.** `StrokeKind::Outside` on the offset rectangle puts the ring's
  inner edge `offset` away from the widget's edge and its outer edge a further
  `width` out — CSS's `outline` + `outline-offset`, which is what the leaf
  describes. epaint grows the outer corner by the stroke width itself
  (`epaint/src/tessellator.rs:1894-1897`), so the corner radius passed is the
  inner edge's: the widget's radius grown by `offset` — the concentric curve,
  floored at `0` and saturated at `255` by `u8_from_f32_saturating`.
* **Radius — the innermost styled scope's.** The ring takes the
  `widgets.active.corner_radius` of the style the focused widget was laid out
  with — the state a focused widget is painted in (§6.1) — so a widget inside a
  role scope gets that role's radius. egui keeps no style per widget, so
  `native_scope` and `native_set_style` each record, for this pass, the
  `unique_id` of the `Ui` they style (`Ui::unique_id`, `ui.rs:357-359`) with
  the role style's `widgets.active.corner_radius`. All of a viewport's records
  live in **one** temporary `ctx.data` entry, keyed
  `Id::new(("native-theme-egui/focus", ctx.viewport_id()))` and holding
  `{ pass_nr, scopes: Vec<(Id, CornerRadius)>, shapes: Vec<(Id, Rect, CornerRadius)> }`
  — these scope records and `register_focus_shape`'s shapes (§4.3). A write
  whose viewport's `cumulative_pass_nr` differs from the entry's `pass_nr`
  empties both lists first, and `on_end_pass` reads the lists only when the
  two agree; the ring walk iterates them. Per viewport because an immediate
  viewport runs a pass nested inside its parent's
  (`eframe/src/native/wgpu_integration.rs:1139`, `:1201`) and
  `cumulative_pass_nr` is counted per viewport
  (`egui/src/context.rs:1781-1793`); one entry rather than one per `Ui` or
  widget because egui never collects a temporary entry — its GC runs only over
  the values it serialises, and skips a temporary one
  (`egui/src/util/id_type_map.rs:712-722`, `:728`) — so per-id entries would
  grow for the life of the process. A later `native_set_style` on the same
  `Ui` replaces its record. The rect is not
  taken at the call: a `Ui`'s `max_rect` still grows with content that
  overflows it (`egui/src/layout.rs:52-55`), so no rect known then bounds what the `Ui`
  will hold. It is read at `on_end_pass` instead. Every `Ui` has an entry in
  this pass's widget table under its `unique_id`, created before anything it
  contains (`ui.rs:170-183`, `:298-312`) and given its final `min_rect` when
  the `Ui` ends (`ui.rs:2889-2893`, `:979-990`, `widget_rect.rs:192`), so
  `scope_radius` reads each recorded `Ui`'s layer and final rect there. Among
  the records on the focused widget's layer whose rect contains the widget's
  `interact_rect` (`Rect::contains_rect`, `emath/src/rect.rs:279-281`; the
  clipped rect, so a widget half scrolled out of a `ScrollArea` inside the scope
  still counts), it takes the innermost: the one whose `Ui` comes last in the
  layer's order (`WidgetRects::order`, `widget_rect.rs:132-134`), since a
  `Ui` is registered before every `Ui` it contains. **The root `Ui` is the one
  exception**: the `Ui` `Context::run_ui` hands to the application ends only
  after the plugins' end-pass hooks have run (`context.rs:811-813`), so during
  `on_end_pass` its entry still holds the `Rect::NOTHING` it was created with
  (`ui.rs:171`), which contains no rect, and a record under its id never
  matches. That root is the `Ui` `on_end_pass` receives, so where no record
  matches the ring takes the corner radius of the root's own style at that
  moment — the base style's `widgets.active.corner_radius`,
  `button.border.corner_radius` (§5.9), unless the application left a role on
  the root with `native_set_style`, and then that role's. §4.4's recipe
  restores the root with `Ui::reset_style` before its `CentralPanel`, so the
  central content takes the base radius. Three stated residuals:
  `native_set_style` restyles a `Ui` only from the call on, while its record
  covers the whole `Ui`; a style the application sets on a `Ui` by other means
  (`Ui::set_style`, `Ui::reset_style`) is seen only on the root, and a child
  that inherited a role left on the root takes the root's final radius unless
  the role is set again inside it — which §4.4's recipe does for every panel,
  so a toolbar's or status bar's widgets take their role's radius; and a
  widget drawn outside its scope's rect (a negative
  margin, a custom `Painter`) gets the next enclosing scope's radius. A widget whose outline is not a rounded rectangle around its
  rect — a switch's track in `native-theme-egui-widgets`
  (`docs/todo_egui-widgets-spec.md`); the connector itself ships no widgets
  (§14.3) — registers that outline with `register_focus_shape(ctx, id, rect,
  corner_radius)` (§4.3), which takes precedence.
* **Layer and clip.** The ring goes on the focused widget's own layer, so a
  popup or window above it covers it as it covers the widget.
  `ctx.layer_painter` clips only to the screen, so the painter is narrowed to
  the widget's `interact_rect` — its rect already cut by its parent's clip, e.g.
  a `ScrollArea` (`widget_rect.rs:32-35`) — grown by the ring's outer extent: a
  fully visible widget's ring is never cut, and a widget scrolled half out of
  view gets no ring over its neighbours. On the cut side the ring may still
  reach up to that extent past the clip edge. WidgetRect coordinates are local to
  the layer (`widget_rect.rs:29-35`), as `layer_painter` draws, so a transformed
  layer carries its ring with it.
* **Why this pass's widget table and not `read_response`.** `Context::read_response`
  (`context.rs:1355-1376`) falls back to the *previous* pass's rect for a
  widget not shown in this one, while focus on a widget that stopped being
  shown is dropped only in `Focus::end_pass` (`memory/mod.rs:630-637`), after
  the plugins have run; the fallback would draw a ring for one frame where the
  widget used to be. `this_pass.widgets` holds exactly the widgets of this pass
  (`context.rs:212`, `ViewportState` is `pub` through `Context::viewport`).

**When it is drawn.** Whenever egui reports a focused widget and the window
has the OS keyboard focus — egui records no focus origin. egui keeps the
focused widget when the window loses focus: `Event::WindowFocused(false)`
clears only the held keys and modifiers (`egui/src/input_state/mod.rs:440-448`),
while `InputState::focused` (`:315`) says whether the window receives key
presses. A ring marks where keyboard input goes, and an unfocused window
receives none, so the ring is drawn only while `ctx.input(|i| i.focused)` and
comes back on the same widget when the window does — egui's own precedent,
since `Response::has_focus` makes exactly that test
(`egui/src/response.rs:350`), and what Qt and GTK do (their sources are in the
rationale). A `Button` gains focus only from the keyboard: `Tab` and
`Shift+Tab` (`memory/mod.rs:596-597`), the arrow keys (`:591-594`) and an
AccessKit focus request; no egui widget but `TextEdit`
(`widgets/text_edit/builder.rs:816`) and `DragValue`
(`widgets/drag_value.rs:667`) calls `request_focus` itself. So a clicked button
shows no ring and a tabbed-to one does, while a clicked text field, which takes
focus to accept typing, shows it. `Escape` clears focus
(`memory/mod.rs:599-602`), and the ring with it.

**How it coexists with egui's own focus look.** egui still promotes the focused
widget to `active` (`widget_style.rs:108`), so a tabbed-to button shows its
pressed fill under the ring — `button.active_background`, or its hover fill
where the platform states none (§6.1) — where a native button shows its resting
fill; no route removes that without faking a state. A focused
`TextEdit` also strokes its frame with `visuals.selection.stroke`
(`widgets/text_edit/builder.rs:742-747`), or with `input.focus_border_color` where
the application hands it `input_frame` (§4.7), so the ring sits outside, or with a
negative offset over, that border. Both are egui's, and both stay.

### 6.19 The expander's arrow colour

`CollapsingHeader::icon` takes any `FnOnce(&mut Ui, f32, &Response) + 'static`
(`collapsing_header.rs:480`) and calls it in place of the default arrow
(`:591-595`). `expander_icon(t)` (§4.7) returns such a closure, built on egui's
own public `paint_default_icon` (`collapsing_header.rs:336`, reached as
`egui::collapsing_header::paint_default_icon` through `containers/mod.rs:7` and
the glob at `egui/src/lib.rs:462-464`) so no shape is re-implemented:

```text
let arrow = t.expander.arrow_color.map(to_color32);   // soft_option
move |ui: &mut Ui, openness: f32, response: &Response| match arrow {
    None => paint_default_icon(ui, openness, response),            // egui's arrow, label colour (§6.4)
    Some(c) => {
        let saved = Arc::clone(ui.style());                         // ui.rs:365-367
        for v in all five of ui.style_mut().visuals.widgets {       // ui.rs:380-382, clone-on-write
            v.fg_stroke.color = c;
        }
        paint_default_icon(ui, openness, response);                 // reads interact(response).fg_stroke.color, :337, :353
        ui.set_style(saved);                                        // ui.rs:387-389
    }
}
```

`paint_default_icon` takes its colour from the `Ui`'s style —
`ui.style().interact(response).fg_stroke.color` (`collapsing_header.rs:337`,
`:353`) — so the closure writes `arrow_color` into every state's `fg_stroke`
of the header's own `Ui` for the one call and puts the style `Arc` back
afterwards. The label is unaffected: `show` copied its `visuals` before calling
the icon (`:559`) and paints the text from that copy afterwards (`:598`). A
child `Ui` would not do: `Ui::new_child` advances the parent's auto-id salt
(`ui.rs:262`) and adds a widget-table entry (`ui.rs:298-312`), and the icon is
painted only while the header is visible (`collapsing_header.rs:558`), so the
ids of the widgets after it would change with scrolling; the swap costs one
`Style` clone per header per pass (`Arc::make_mut`), only where the
theme states an arrow colour.


---

## 7 -- Numeric conversion into epaint's integer-backed types

### 7.1 What is actually at risk

Rust's **float-to-integer `as` cast saturates**: values above the target maximum
clamp to it, values below the minimum clamp to it, and `NaN` becomes `0`. It
never wraps and never panics. That is a language guarantee, so it has no
`file:line` inside these crates, and epaint's own `impl From<f32> for CornerRadius`
relies on it (`Self::same(radius.round() as u8)`,
`epaint/src/corner_radius.rs:42-44`). **Integer-to-integer** `as` casts *do*
wrap, so no intermediate integer step may be introduced.

The real hazards are therefore **silent rounding to whole points**, **silent
saturation at ±127 or 255**, and **`NaN` silently becoming a zero margin** — a
wrong layout rather than a crash, which is worse because it is invisible.

This is not defensive programming. `ResolvedTheme` derives `Deserialize` with
public fields (`native-theme/src/model/resolved.rs:155-156`); of a widget's
`border.*` numbers native-theme range-checks only the stated padding sides
(`native-theme-derive/src/gen_ranges.rs:127-141` emits `check_padding`,
`native-theme/src/resolve/validate_helpers.rs:458-485`) and never a widget's
`corner_radius` or `line_width` (`check_defaults_ranges`,
`validate_helpers.rs:650-842`, covers `defaults.*` and `text_scale.*` only; its
border checks are `defaults.border`'s, `:683-701` and `:720-727`); and `card` is missing from the
`check_ranges` dispatch list altogether (`native-theme/src/resolve/validate.rs:168-191`),
so not even its padding is checked. All of it runs only inside `validate()`. A
negative, `NaN` or `1e30` radius, or padding, can reach the connector in
production.

epaint additionally ships **two disagreeing rounding policies for the same
target type**, which the connector must not inherit:

```text
impl From<f32>       for CornerRadius   Self::same(radius.round() as u8)   corner_radius.rs:42-44
impl From<f32>       for Margin         Self::same(v.round() as _)         margin.rs:108-113
impl From<Vec2>      for Margin         Self::symmetric(v.x.round() as _, v.y.round() as _)  margin.rs:115-120
impl From<MarginF32> for Margin         bare `as _`, NO .round() -> truncates  margin_f32.rs:32-42
```

Two routes from the same float can differ by one point. **The connector uses its
own helpers everywhere and never the `From` impls, and never routes through
`MarginF32`.**

### 7.2 The helpers, in full

Locked. Panic-free, `unsafe`-free, total over `f32`, and every `as` cast is
applied to a value already proven in range. This is the one copy of every
`convert` doc comment and body; §4.8 lists the public signatures and points
here. `pub` items are the module's public surface; `pub(crate)` items are
internal steps that no application needs (`padding_with_border` and
`to_button_padding` are reached through the atlas, `to_shadow` through §6.14,
`denan` and `i8_from_f32_saturating` through the functions that use them).

```rust,ignore
//! Total, panic-free conversions from native-theme values to epaint types.
//!
//! Every function is defined for **all** `f32` bit patterns including `NaN`, `±∞`, subnormals
//! and `-0.0`. None can panic, none allocates, none uses `unsafe`, and every `as` cast is
//! provably exact because the preceding steps constrain the operand to the destination range.
//!
//! # One non-finite rule
//!
//! A theme value that is `NaN` or `±∞` is replaced by **egui's own value for the sink it
//! feeds** — `finite_or(x, <that value>)` — and never by `0.0` unless `0.0` is that value;
//! the call site records a [`crate::Note::ValueSanitised`]. Every sink conversion that can know
//! egui's value takes it as a `base` parameter and applies the rule itself
//! ([`to_corner_radius`], [`to_margin`], [`to_stroke`]). Every layout length that survives
//! the rule then passes [`clamp_length`]; a finite value merely clamped into range is not a
//! substitution and records nothing.
//!
//! This matters because native-theme range-checks a per-widget border's padding sides only
//! (`native-theme-derive/src/gen_ranges.rs:127-141`, `check_padding` at
//! `native-theme/src/resolve/validate_helpers.rs:458-485`), never its `corner_radius` or
//! `line_width`, and [`crate::ResolvedTheme`] derives `Deserialize` with public fields
//! (`native-theme/src/model/resolved.rs:155-156`), which bypasses validation altogether — so a
//! hostile or buggy input really can put a non-finite number in
//! `theme.button.border.corner_radius`, or in a padding side.

/// Re-exported **here and not at the crate root**, because `egui::Rgba`
/// (`egui/src/lib.rs:442` -> `ecolor/src/rgba.rs:3-10`) is *linear f32, premultiplied* while
/// this one is *sRGB u8, straight* (`native-theme/src/color.rs:8-11`, `:43-52`), and this crate
/// re-exports `egui`.
pub use native_theme::color::Rgba;

/// Map `NaN` to `0.0`; leave every other value, including `±∞`, untouched.
///
/// The first step of [`u8_from_f32_saturating`], [`i8_from_f32_saturating`],
/// [`unit_interval`] and [`clamp_length`], which must be total: the one place a `NaN` is
/// folded *inside* a conversion. It is not what a theme `NaN` becomes at a sink — that is
/// [`finite_or`] with egui's own value, and the call site reports that substitution; `denan`
/// itself reports nothing. `f32::clamp` is banned: it *propagates* `NaN` and panics when
/// `min > max` or a bound is `NaN`. Bare `.max()`/`.min()` are order-dependent under `NaN` —
/// `NAN.max(lo).min(hi) == lo` but `NAN.min(hi).max(lo) == hi` — so `denan` runs first, every
/// time.
#[inline]
#[must_use]
pub(crate) const fn denan(v: f32) -> f32 {
    if v.is_nan() { 0.0 } else { v }
}

/// Saturating, rounding `f32` -> `u8`, for `epaint::CornerRadius` (four `u8`,
/// `epaint/src/corner_radius.rs:13-25`) and the alpha byte of [`to_color32_with_opacity`].
/// `epaint::Shadow::{blur, spread}` (`epaint/src/shadow.rs:20`, `:23`) are cited for their
/// representable range only — this crate never writes shadow geometry (§6.14).
///
/// `NaN` -> 0; `v <= 0.0` -> 0; `v >= 255.0` -> 255; otherwise round-half-away-from-zero.
/// `f32::round`, not `round_ties_even`, so a value this crate converts and a value epaint
/// converts through `impl From<f32> for CornerRadius` (`epaint/src/corner_radius.rs:42-44`)
/// agree bit for bit. After `denan` + `max` + `min` the operand is finite and in
/// `0.0..=255.0`, and `.round()` keeps it there, so the cast is exact.
#[inline]
#[must_use]
#[allow(clippy::manual_clamp, reason = "f32::clamp is banned: denan runs first, in a fixed order")]
pub fn u8_from_f32_saturating(v: f32) -> u8 {
    denan(v).max(0.0).min(255.0).round() as u8
}

/// Saturating, rounding `f32` -> `i8`, for `epaint::Margin` (four `i8`,
/// `epaint/src/margin.rs:15-20`) — the only `i8` sink this crate writes from theme data.
/// `epaint::Shadow::offset` (`epaint/src/shadow.rs:15`) is cited for its representable
/// range only; this crate never writes shadow geometry (§6.14).
///
/// `NaN` -> 0, **not** −128, because [`denan`] runs first: `f32::NAN.max(-128.0)` is
/// `-128.0`, so a `NaN` margin would otherwise become the most-negative margin.
/// `v <= -128.0` -> −128; `v >= 127.0` -> 127; otherwise round-half-away-from-zero.
/// Negative values are preserved rather than clamped to zero — egui itself computes a
/// negative outer margin from `expansion` (`egui/src/widget_style.rs:162`,
/// `egui/src/widgets/text_edit/builder.rs:767`).
///
/// Saturation here is a **real** loss of theme data and emits a
/// [`crate::Note::ValueSaturated`] at the call site, which detects it itself (§7.2's
/// emission rule); the helper stays pure and single-valued.
#[inline]
#[must_use]
#[allow(clippy::manual_clamp, reason = "f32::clamp is banned: denan runs first, in a fixed order")]
pub(crate) fn i8_from_f32_saturating(v: f32) -> i8 {
    denan(v).max(-128.0).min(127.0).round() as i8
}

/// Pass a finite `f32` through; substitute `fallback` for `NaN` and `±∞`.
///
/// The crate's one non-finite rule (§6): `fallback` is **egui's own value for the sink** —
/// the value the starting `Style` carries there — so a non-finite theme value leaves egui's
/// look in place instead of inventing one, and it is never `0.0` unless that is egui's
/// value. A substitution made here is **reported**: the call site emits
/// [`crate::Note::ValueSanitised`] for that leaf. `finite_or` already maps `NaN`, so
/// `finite_or(denan(x), fallback)` is never written.
#[inline]
#[must_use]
pub fn finite_or(v: f32, fallback: f32) -> f32 {
    if v.is_finite() { v } else { fallback }
}

/// The largest `f32` strictly below `f32::MAX * GUI_ROUNDING` (≈ `1.0633823e37`).
///
/// `Layout::next_frame` rounds every edge with `round_ui`, `(x / GUI_ROUNDING).round() *
/// GUI_ROUNDING` (`egui/src/layout.rs:635`, `emath/src/gui_rounding.rs:59`), which overflows
/// to `+∞` above this product and trips egui's debug asserts (§6.7); `GUI_ROUNDING` is
/// `1.0 / 32.0`, `pub` at `egui::emath::GUI_ROUNDING` (`emath/src/gui_rounding.rs:18`,
/// re-exported at `emath/src/lib.rs:47` and `egui/src/lib.rs:438`). The product with a power
/// of two is exact, so `next_down` lands on the neighbouring `f32`; float arithmetic and
/// `f32::next_down` are `const` on Rust 1.88.0 already, below the crate's MSRV (§12.4). One
/// step below the product, a lone widget or a solid scrollbar groove inside a `Window`
/// completes its passes (§6.5, §6.7).
const LENGTH_CEILING: f32 = (f32::MAX * egui::emath::GUI_ROUNDING).next_down();

/// Clamp one layout length into `0.0..=LENGTH_CEILING`, total over all `f32`.
///
/// Every length this crate writes into a `Style` or a `Frame` — a `Spacing` field, a stroke
/// width, a `button_padding` component — passes here **after** [`finite_or`] has applied the
/// non-finite rule, so its `NaN` image (`0.0`) is a totality backstop, never the documented
/// fallback. `−∞` floors to `0.0`; `+∞` — a finite theme value the formula's own arithmetic
/// overflowed — caps at the ceiling. A value merely clamped is not sanitised and emits
/// nothing. The cap bounds **one** length: an edge is the sum of every length laid out
/// before it, so two stacked widgets each just under the ceiling can still reach egui's
/// rounding edge; no per-value bound can prevent that, and it stays a stated residual (§6.7).
#[inline]
#[must_use]
#[allow(clippy::manual_clamp, reason = "f32::clamp is banned: denan runs first, in a fixed order")]
pub fn clamp_length(v: f32) -> f32 {
    denan(v).max(0.0).min(LENGTH_CEILING)
}

/// Clamp an opacity into `0.0..=1.0`, total over all `f32`.
///
/// Mandatory before anything reaches `Visuals::disabled_alpha` (`egui/src/style.rs:1126`):
/// `Visuals::disable` (`:1177-1180`) forwards it to `Color32::gamma_multiply`, which carries
/// `debug_assert!(0.0 <= factor && factor.is_finite(), ..)` (`ecolor/src/color32.rs:270-273`);
/// its only caller in egui is `Ui::dnd_drop_zone` (`egui/src/ui.rs:2726-2727`), while
/// `Ui::disable` does not reach it (§7.4). A **non-finite** input is a substitution and is
/// reported as [`crate::Note::ValueSanitised`] at the call site; a finite value merely
/// clamped into range is not, and emits nothing.
#[inline]
#[must_use]
#[allow(clippy::manual_clamp, reason = "f32::clamp is banned: denan runs first, in a fixed order")]
pub fn unit_interval(v: f32) -> f32 {
    denan(v).max(0.0).min(1.0)
}

/// sRGB straight-alpha [`Rgba`] -> `egui::Color32` (sRGB premultiplied,
/// `ecolor/src/color32.rs:31`).
///
/// `const`, allocation-free, total over all 2^32 inputs. It uses
/// `Color32::from_rgba_unmultiplied_const` (`ecolor/src/color32.rs:139`), which premultiplies
/// in **gamma** space, applying no sRGB transfer function: exact at `a == 0` (`:142`) and
/// `a == 255` (`:145`), and at `1..=254` an integer `mul_frac_round(channel, a)` per channel
/// (`:147-151`, `ecolor/src/lib.rs:137-146`) that rounds exactly as the non-`const`
/// `Color32::from_rgba_unmultiplied` does, since that one calls this one
/// (`ecolor/src/color32.rs:133-135`). Do **not** route through `ecolor::Rgba`: feeding it sRGB
/// floats double-encodes gamma (§7.3).
#[inline]
#[must_use]
pub const fn to_color32(c: Rgba) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied_const(c.r, c.g, c.b, c.a)
}

/// [`to_color32`] with an opacity multiplier folded into alpha. See §6.13.
///
/// This is how `defaults.border.opacity` (`native-theme/src/model/border.rs:222`), the
/// model's only border opacity, reaches egui: `epaint::Stroke` is
/// `{ width: f32, color: Color32 }` and nothing else (`epaint/src/stroke.rs:13-16`). A
/// non-finite `opacity` is `1.0`, egui's own value for a stroke (no fold), per the
/// non-finite rule; the call site reports it. Never use the result for
/// `WidgetVisuals::bg_fill`, documented "Must never be `Color32::TRANSPARENT`"
/// (`egui/src/style.rs:1292-1294`); `opacity == 0.0` produces exactly that.
#[inline]
#[must_use]
pub fn to_color32_with_opacity(c: Rgba, opacity: f32) -> egui::Color32 {
    let a = u8_from_f32_saturating(f32::from(c.a) * unit_interval(finite_or(opacity, 1.0)));
    egui::Color32::from_rgba_unmultiplied_const(c.r, c.g, c.b, a)
}

/// Uniform `epaint::CornerRadius` from one logical-pixel length, over the sink's own radius.
///
/// A non-finite `radius_px` keeps `base` — the radius the starting `Style` has in that sink —
/// per the non-finite rule; the call site reports it. Saturation at 255 is provably benign:
/// the tessellator re-clamps a corner radius to half the smaller side
/// (`epaint/src/tessellator.rs:638-642`), so 255 simply reads as "pill". No `Note` is
/// emitted for saturation.
#[inline]
#[must_use]
pub fn to_corner_radius(base: egui::CornerRadius, radius_px: f32) -> egui::CornerRadius {
    if radius_px.is_finite() {
        egui::CornerRadius::same(u8_from_f32_saturating(radius_px))
    } else {
        base
    }
}

/// `epaint::Margin` from a widget's per-side padding, over the sink's own margin.
///
/// `ResolvedPadding` has one `Option<f32>` per side (`native-theme/src/model/border.rs:195-204`),
/// each a **per-side** value (`docs/platform-facts.md:910-926`) — do **not** halve. A stated,
/// finite side (`Some(0.0)` included) is rounded to a whole point, saturating; an unstated
/// side (`None`) and, by the non-finite rule, a `NaN` or `±∞` one keep `base`'s side, where
/// `base` is the value egui's own default `Style` for that scheme gives the sink (S1) —
/// `Spacing::window_margin`'s `Margin::same(6)` (`egui/src/style.rs:1456`), a panel frame's
/// `Margin::symmetric(8, 2)` (`egui/src/containers/frame.rs:187`) — never an invented `0`.
/// `base` is a parameter because only the call site knows the sink, as in iced's
/// `padding_or` (`connectors/native-theme-iced/src/lib.rs:329-339`). The call site reports a
/// non-finite side, and a saturated one.
#[inline]
#[must_use]
pub fn to_margin(base: egui::Margin, padding: &native_theme::theme::ResolvedPadding) -> egui::Margin {
    let side = |stated: Option<f32>, own: i8| {
        stated.filter(|v| v.is_finite()).map_or(own, i8_from_f32_saturating)
    };
    egui::Margin {
        left: side(padding.left, base.left),
        right: side(padding.right, base.right),
        top: side(padding.top, base.top),
        bottom: side(padding.bottom, base.bottom),
    }
}

/// A widget border's padding measured from the outside of its border: each stated, finite
/// side plus the border's line width.
///
/// A platform's padding lies *inside* its border — outer size = border + padding + content
/// (`docs/platform-facts.md:900-902`) — while `Button`, `ComboBox`, `CollapsingHeader` and
/// `TextEdit` draw their stroke *inside* the padding egui gives them: `Button`'s frame margin
/// is `button_padding + expansion − bg_stroke.width` and `Frame` adds the stroke back
/// (`egui/src/widget_style.rs:163-165`, `egui/src/containers/frame.rs:327-331`),
/// `button_frame` strokes `StrokeKind::Inside` within `content ± button_padding`
/// (`egui/src/containers/combo_box.rs:439-460`), the header likewise
/// (`egui/src/containers/collapsing_header.rs:531`, `:566-567`), and `TextEdit` allocates with
/// its margin alone and paints with `margin + expansion − stroke.width`
/// (`egui/src/widgets/text_edit/builder.rs:713`, `:763-767`). Written into those sinks, this
/// padding makes the native outer size exact up to the sinks' whole-point rounding: a
/// `Margin` sink holds `round(side + width)`, and a `Button` realises
/// `round(button_padding − width) + width` per side (`egui/src/widget_style.rs:163-165`,
/// `epaint/src/margin.rs:117-119`), so a fractional padding or width moves an edge by at
/// most half a point. It is exact only where the preset's padding is the platform's
/// inside-the-border padding: `macos-sonoma`'s vertical button padding is derived as outer
/// minus content, `(22 − 16) / 2` (`docs/platform-facts.md:1172`), which already contains
/// the border, so there the button comes out one line width per side taller than the
/// platform's until platform-facts corrects that derivation (`docs/todo.md`).
///
/// The width is `clamp_length(finite_or(line_width, 0.0))`: a non-finite width adds nothing
/// — egui's own padding already holds egui's own stroke, so adding nothing is egui's value
/// there — and a negative one is `0.0`; the call site reports a non-finite width. A finite
/// side plus a finite width can only overflow upwards, and `.min(f32::MAX)` keeps that sum
/// finite, so [`to_margin`] saturates it rather than mistaking it for a non-finite side. A
/// `None` or non-finite side stays `None`, which every consumer resolves to egui's own value.
#[inline]
#[must_use]
pub(crate) fn padding_with_border(
    border: &native_theme::theme::ResolvedWidgetBorder,
) -> native_theme::theme::ResolvedPadding {
    let width = clamp_length(finite_or(border.line_width, 0.0));
    let side = |stated: Option<f32>| {
        stated.filter(|v| v.is_finite()).map(|v| (v + width).min(f32::MAX))
    };
    native_theme::theme::ResolvedPadding {
        top: side(border.padding.top),
        right: side(border.padding.right),
        bottom: side(border.padding.bottom),
        left: side(border.padding.left),
    }
}

/// `Spacing::button_padding` from a widget's resolved border, under §5's pair rule.
///
/// `button_padding` is one `Vec2` (`egui/src/style.rs:398`) that `Button`, `ComboBox` and
/// `CollapsingHeader` apply symmetrically, so left and right share `.x` and top and bottom
/// share `.y`. Each component is the mean of its pair's two targets: a side of
/// [`padding_with_border`] that is `Some` — the stated side plus the border's line width — is
/// its own target, and a `None` side targets `base`'s component — the starting style's: egui's own default
/// (`vec2(4.0, 1.0)`, `style.rs:1458`) on the base style, the base style's in a role cell — exactly as [`to_margin`] keeps `base`'s side.
/// With both sides stated the component is `(a + b) / 2 + line_width`: the one symmetric value
/// that keeps the native outer size and moves the content off-centre by the least,
/// `|a − b| / 2` (§5, §14 item 32). Filtering non-finite sides in [`padding_with_border`] is
/// what keeps a `NaN` out of the `Vec2` (§7.4). `f32::midpoint` (stable since Rust 1.85)
/// never overflows for finite operands, returns an equal pair bit for bit, and equals
/// `(a + b) / 2.0` wherever that is finite. `button_padding` is `f32`, so no integer
/// saturation is needed here; [`clamp_length`] supplies the `0.0` floor (§6.17) and the layout
/// cap. The expander reads `.x` on its trailing side only
/// (`egui/src/containers/collapsing_header.rs:526`), so its call site takes `.y` from this
/// function and writes `.x` from `padding_with_border(border).right` alone (§7.4).
#[inline]
#[must_use]
pub(crate) fn to_button_padding(
    base: egui::Vec2,
    border: &native_theme::theme::ResolvedWidgetBorder,
) -> egui::Vec2 {
    let padding = padding_with_border(border);
    let pair = |a: Option<f32>, b: Option<f32>, own: f32| {
        clamp_length(f32::midpoint(a.unwrap_or(own), b.unwrap_or(own)))
    };
    egui::vec2(
        pair(padding.left, padding.right, base.x),
        pair(padding.top, padding.bottom, base.y),
    )
}

/// A stated hover or pressed fill composited over the widget's idle fill (C17, §6.1).
///
/// Source-over in premultiplied gamma space, `layer + idle · (1 − layer.a)`, done by
/// ecolor's own `Color32::blend` (`ecolor/src/color32.rs:343-345`): an integer
/// `gamma_multiply_u8` (`:289-298`) plus a saturating add, so it is total, panic-free and
/// divides by no runtime value. An opaque layer returns itself exactly and an `a == 0` layer
/// returns the idle fill exactly. Gamma space rather than linear light, as iced's composite
/// argues (`connectors/native-theme-iced/src/styles.rs:74-78`). The result is rounded to
/// `u8` in premultiplied space, so where the layer is translucent it can differ in rounding
/// from a straight-alpha `f32` composite such as iced's `composite_over`.
#[inline]
#[must_use]
pub fn composite_over(layer: Rgba, idle: Rgba) -> egui::Color32 {
    to_color32(idle).blend(to_color32(layer))
}

/// `epaint::Stroke` from a native colour and a line width, over the sink's own stroke.
///
/// The width is `clamp_length(finite_or(line_width_px, base.width))`: a non-finite width keeps
/// `base.width` — the width the starting `Style` has in that sink — per the non-finite rule
/// (the call site reports it), a negative one is `0.0`, and a huge one is capped. That
/// matters because `Stroke::is_empty` is `width <= 0.0 || color == TRANSPARENT`
/// (`epaint/src/stroke.rs:35-37`) and `NaN <= 0.0` is `false`, so a `NaN` width would not be
/// filtered and would reach the tessellator. Only the width falls back; the colour is always
/// the theme's.
#[inline]
#[must_use]
pub fn to_stroke(base: egui::Stroke, color: Rgba, line_width_px: f32) -> egui::Stroke {
    egui::Stroke::new(clamp_length(finite_or(line_width_px, base.width)), to_color32(color))
}

/// `epaint::Shadow` from a native shadow colour, keeping `base`'s geometry. See §6.14.
///
/// native-theme carries **no** shadow geometry — only `shadow_enabled: bool`
/// (`native-theme/src/model/border.rs:224`, `:241`) — while `epaint::Shadow` needs
/// `offset: [i8; 2]` (`epaint/src/shadow.rs:15`), `blur: u8` (`:20`) and `spread: u8` (`:23`).
/// Only the colour is replaced; `enabled == false` yields `Shadow::NONE`
/// (`epaint/src/shadow.rs:40-45`).
#[inline]
#[must_use]
pub(crate) fn to_shadow(base: egui::Shadow, color: Rgba, enabled: bool) -> egui::Shadow {
    if enabled {
        egui::Shadow { color: to_color32(color), ..base }
    } else {
        egui::Shadow::NONE
    }
}
```

**Who emits the `Note`s.** Emission is owned by the **mapping layer**, never by
the helpers. Every helper above is pure, single-valued and takes no path, which
is why `i8_from_f32_saturating`'s doc says "at the call site" rather than
declaring a diagnostics channel of its own. The signatures do not change; the
rule below is what makes `Note::ValueSanitised`, `Note::ValueSaturated` and
`Note::TransparentFill` (§4.3) reachable at all.

* **`Note::ValueSanitised { path }`** — emitted **once per leaf**, at the call
  site, whenever that leaf's input was `NaN` or `±∞` — or, for a text size,
  whenever its scaled size is not a positive normal `f32` (§8.5) — and egui's own
  value was substituted for it (§6's rule): by `finite_or`, by a formula's `is_finite()`
  test, or inside `to_margin`, `padding_with_border`, `to_button_padding`,
  `to_stroke`, `to_corner_radius` or `to_color32_with_opacity`, which the call site detects
  with the same `is_finite()` test on the leaf it passed. A **finite** value
  merely clamped into range — by `unit_interval`, `clamp_length`, or a
  `.max(0.0)` / `.min(..)` in one of §6's formulas — is not sanitisation and
  emits nothing.
* **`Note::ValueSaturated { path }`** — emitted whenever `i8_from_f32_saturating`
  clamped. Because the helper returns a bare value, saturation is detected at the
  call site rather than reported by the helper:

  ```rust,ignore
  let d = denan(v);
  let saturated = d >= 127.5 || d <= -128.5;   // round() leaves -128..=127
  ```

  `u8_from_f32_saturating` and `to_corner_radius` emit **nothing**: corner-radius
  saturation is benign, because the tessellator re-clamps a radius to half the
  smaller side (`epaint/src/tessellator.rs:638-642`), so `255` reads as "pill"
  (§14 item 19). `epaint::Margin` is the only `i8` sink this crate writes from
  theme data, so in practice `ValueSaturated` is a margin note.
* **`path`** is the **leaf** path — the same dotted string `mapping.toml` keys
  its rows with (§13.1), e.g. `"button.border.corner_radius"` — never the egui
  sink path.
* **No producer outside the mapping layer.** §9 emits none: the icon free functions could not, since
  `Note` is compiled into the atlas (§4.3) and surfaces only through
  `ThemeAtlas::notes()` (§4.2), which a free function has no handle on. An
  oversize icon is reported by returning `None` (§9.3), not by a new variant.
* **`Note::TransparentFill { path }`** — emitted **once per leaf**, at the call
  site, whenever a colour with alpha `0` is written, unchanged, into any
  `widgets.<state>.bg_fill` (§6.4). It is never folded into `ValueSanitised`:
  nothing was non-finite and nothing was replaced.

### 7.3 The colour-space trap

`native_theme::color::Rgba` is **straight (non-premultiplied) sRGB `u8`**
(`native-theme/src/color.rs:8-11`, struct `:43-52`). `egui::Color32` is **premultiplied sRGB
`u8`** (`ecolor/src/color32.rs:31`). `egui::Rgba` is a third thing entirely:
**linear `f32`, premultiplied** (`ecolor/src/rgba.rs:3-10`).

Two mistakes are possible and both survive casual testing:

* `Color32::from_rgba_premultiplied(r, g, b, a)` (`ecolor/src/color32.rs:122`)
  stores straight components as if premultiplied, turning `(255, 255, 255, 0)`
  into *additive white*. It is **identical** for opaque colours, so the bug only
  appears on a semi-transparent value, and the model has many:
  `defaults.shadow_color` (`native-theme/src/presets/kde-breeze.toml:15`, `#00000040`), the disabled
  backgrounds (`adwaita.toml:94`, `#e8e8e880`) and windows-11's hover layers
  (`windows-11.toml:85`, `#0000000a`).
* Routing through `egui::Rgba` double-encodes gamma, because
  `native_theme::Rgba::to_f32_array()` divides by 255 with no transfer-function
  change (`native-theme/src/color.rs:126-134`) while `ecolor::Rgba` expects
  linear. The error is worst exactly in the dark tones a dark theme is made of.

`Color32::from_rgba_unmultiplied_const` (`ecolor/src/color32.rs:139`) is the
only correct route. It is `const`, and it premultiplies in **gamma** space,
applying no sRGB transfer function: `a == 255` short-circuits to an exact
`from_rgb` (`:145`) and `a == 0` to `TRANSPARENT` (`:142`), while `1..=254`
computes the integer fixed-point `mul_frac_round(channel, a)` per channel
(`ecolor/src/color32.rs:147-151`, helper at `ecolor/src/lib.rs:137-146`), i.e.
`channel · a / 255` rounded, with no float and no division. The non-`const`
`from_rgba_unmultiplied` simply calls it (`ecolor/src/color32.rs:133-135`), so
the two agree bit for bit by construction. It *does* round, so the **forward** conversion is already lossy at low alpha.
The documented inverse, `Color32::to_srgba_unmultiplied`
(`ecolor/src/color32.rs:248`), is lossy there too. Round-tripping is therefore
**not** exact in either direction, and that must be documented rather than
assumed.

**Never `use egui::Rgba` in the same module as `native_theme::color::Rgba`.**
This is why `Rgba` is re-exported from `convert`, not from the crate root
(§4.1).

### 7.4 The two genuine panic paths

`Visuals::disabled_alpha` (`style.rs:1126`) is forwarded by `Visuals::disable`
(`:1177-1180`) to `Color32::gamma_multiply`, which carries
`debug_assert!(0.0 <= factor && factor.is_finite(), ..)`
(`ecolor/src/color32.rs:270-273`). A negative, `NaN` or infinite value would
therefore **abort every downstream user's `cargo test` and `cargo run`** — far
worse than a wrong pixel. Everything reaching `disabled_alpha` **must** pass
through `unit_interval(finite_or(x, base))`, `base` being the starting `Style`'s
own `disabled_alpha` — egui's `0.5` (`style.rs:1560`) — so a `NaN` or `±∞`
opacity keeps egui's fade (§6's rule) instead of becoming `0.0`, which would make
every disabled widget invisible.

Two precisions that matter for the test that proves this:

* `Ui::disable` (`ui.rs:497-502`) calls `Painter::multiply_opacity`, which is
  `if opacity.is_finite() { self.opacity_factor *= opacity.clamp(0.0, 1.0); }`
  (`painter.rs:100-104`) — non-finite is dropped and the value is clamped, so
  **no assert can fire on that path**.
* The **only** caller of `Visuals::disable` in all of egui is
  `Ui::dnd_drop_zone` (`ui.rs:2726-2727`). A test that only calls `ui.disable()`
  proves nothing; §13 T4(b) exercises `dnd_drop_zone` specifically.

**The second, new in emath 0.36.2.** `Vec2` and `Pos2` no longer derive
`PartialEq`; their hand-written `eq` asserts, in debug builds only, that neither
side has a `NaN` component (`emath/src/vec2.rs:349-359`,
`emath/src/pos2.rs:233-243`); 0.36.1 derived it without the assert. `Style`'s
derived `PartialEq` (`style.rs:241`) reaches every `Vec2` in `Spacing`
(`:382`), so a `NaN` written into `item_spacing`, `button_padding` or
`interact_size` aborts a debug build at the first comparison of two styles —
§13's own tests compare built styles. Every float this crate writes into a
`Vec2` therefore passes the same `denan` / `finite_or` discipline as one
narrowed to an integer: `to_button_padding` filters non-finite sides, and the
expander's `.x`, the only `Vec2` component written straight from a padding side
(§5, `expander.border.padding.right`), is
`padding_with_border(&expander.border).right.map_or(base.x, clamp_length)` —
the side plus the border's line width, like every border-inclusive padding
(§7.2): an unstated or non-finite side keeps egui's `4.0` (`style.rs:1458`),
exactly as `to_button_padding`'s targets do.

### 7.5 Two more silent-failure guards

**`Vec2` fields must be written whole.** `spacing.interact_size` is a `Vec2`
(`style.rs:409`); writing only `.y` leaves `.x` at egui's `40.0` (`:1460`),
which is a `Grid` column floor (`grid.rs:126`, `:446`), the `DragValue` width
(`widgets/drag_value.rs:590`, `:593`, `:640`) and the **exact** colour-swatch
width (`widgets/color_picker.rs:111`). native-theme has no analogue for any of
those, so `.x` is deliberately left inherited — but that decision must be
written down, because `CollapsingHeader` clamps **both** axes
(`collapsing_header.rs:532`) and `Checkbox`/`RadioButton` derive their width
from `.y` through `Vec2::splat` (`checkbox.rs:85-86`, `radio_button.rs:54-55`).

**`text_styles` keys may be overwritten and added, never removed.**
`TextStyle::resolve` calls `panic!` when a key is missing
(`style.rs:112-120`), and it sits on the hot path of essentially every widget
via `FontSelection::resolve` (`:151-153`) and `resolve_with_fallback`
(`:158-172`). The connector overwrites the five built-in keys and adds none of
its own (§5.8 items 4 and 5).

It does, however, **replace the whole map**, and that is not the same thing as
leaving it alone. Every `Style` the connector publishes starts from
`Theme::default_style()` — a base style directly, a role cell through the base
style it starts from (§3.4) — which is `Style::default()` with only the
visuals swapped (`egui/src/memory/theme.rs:24-29`), and `Style::default()` sets
`text_styles: default_text_styles()` (`style.rs:1433`) — exactly the five stock
keys (`:1418-1422`). Published as it stands, it would **remove an application's
own `TextStyle::Name` key**: `set_style_of` assigns `opt.dark_style` /
`light_style` wholesale (`context.rs:2250-2255`), and every role scope replaces a
child `Ui`'s style outright (`ui.rs:237`). The next `resolve` of that key would
then panic — and `style.rs:112-120` carries no `cfg(debug_assertions)`, so it
would fire in **release** as well as debug.

**So every route that publishes a style carries the application's keys across:**

* **`install`**, before each `set_style_of(theme, ..)`, copies into the
  atlas's base style for `theme` every `TextStyle::Name` key of
  `ctx.style_of(theme)` (`context.rs:2221`) — the style it is about to replace —
  that the base style does not have. The atlas's base styles hold no `Name` key
  (§8.5), so every such key is carried with the application's own `FontId`,
  while the five stock keys keep the atlas's values. Dark takes dark's keys and
  light takes light's. When the old style holds no `Name` key, the atlas's own
  `Arc` is installed unchanged; only otherwise is a copy made.
* **`native_scope` and `native_set_style`** merge the `Name` keys of the parent
  `ui.style()` that the role style lacks into the style they hand the child
  `Ui` — the parent carries whatever the application registered, globally or on
  that `Ui` — and clone the role `Arc` only when such a key exists. An
  application without `Name` keys pays nothing; one with them pays one `Style`
  clone per scope per pass.
* **`role_modifier(theme, role, variant)`** replaces the whole style it is handed but
  carries that style's `Name` keys into the replacement: egui hands a
  `StyleModifier`'s closure the style it modifies (`style.rs:194`, `:225-229`).

No other route hands out a role style: the raw per-cell `Arc<Style>`s are
crate-private (§4.2), so no application can install one without the merge.

`Builder::build` still takes no `Context` (§4.3), so seeding `text_styles` there
is impossible; the two routes above are where the application's keys are
visible, and each merges at the moment it replaces a style.

---

## 8 -- Fonts, text scale and line height

### 8.1 The never-`Name` invariant

**This crate never emits a `FontFamily::Name`.** Every `FontId` it produces
names `FontFamily::Proportional` or `FontFamily::Monospace`, and
`fonts::font_definitions` only ever *prepends into those two existing chains*.

This is structural, not stylistic. Two epaint panics fire, in release as in
debug, at the first text layout or row-height query that names the family
(`FontsImpl::font`, `epaint/src/text/fonts.rs:1021-1036`, reached from
`epaint/src/text/text_layout.rs:127`, `:777` and `epaint/src/text/fonts.rs:867`)
— inside the application's own UI code, with no recovery point:

| panic | site | trigger |
|---|---|---|
| `FontFamily::{family:?} is not bound to any fonts` | `epaint/src/text/fonts.rs:1025` | a `Name` in `Style` that `FontDefinitions` does not bind |
| `No font data found for {font_name:?}` | `:1033` | a family listing a key absent from `font_data` |

A theme can reach those two panics because `Style` and `FontDefinitions` land through
**different channels**: `set_style_of` takes effect immediately
(`context.rs:2250`) while `set_fonts` is deferred to the next pass
(`context.rs:2104-2106`). Any design that emits a `Name` must therefore guarantee an
ordering across two channels, and a violated guarantee is a panic in the user's
application. The never-`Name` invariant makes both **unreachable by
construction** for every family this crate names: `Style` and
`FontDefinitions` become independent, and installing one without the other is
harmless for those families; an application's own fonts survive an install
only through `FontPlan::with_base` (§4.9); an `add_font` still pending at the
next pass is applied on top of them (`egui/src/context.rs:556-574`).

Two further consequences:

* The emoji fallback tail — `NotoEmoji-Regular` and `emoji-icon-font` — that
  `FontDefinitions::default()` installs (`epaint/src/text/fonts.rs:534-550`)
  survives. A custom `Name` family would lose it, and emoji would render as the
  replacement glyph. egui ships **no** CJK face at all — its own documentation
  says "The default `egui` fonts only support latin and cyrillic alphabets"
  (`egui/src/context.rs:2101`) — so CJK coverage is the application's problem
  under either design.
* **The price is exactly two families, therefore exactly one weight per
  family.** §8.3.

### 8.2 Families need bytes, not names

The only way a face enters epaint is a byte buffer:
`FontData { font: Cow<'static, [u8]>, .. }` (`epaint/src/text/fonts.rs:112-122`).
The `String` keys in `FontDefinitions` (`:431-444`) are arbitrary caller labels
that nothing looks up in a system font database, and neither epaint nor egui nor
eframe depends directly on `fontdb`, `font-kit`, `fontconfig`, `core-text` or DirectWrite.
The only `load_system_fonts()` in the egui family is
`egui_extras/src/loaders/svg_loader.rs:42`, under feature `svg_text` (`:41`), which feeds **resvg's** fontdb for
text inside SVG images and has zero connection to egui's text layout. (The only
system-font lookup in eframe's default dependency tree is sctk-adwaita 0.10.1's
`fc-match` call, `src/title/ab_glyph_renderer.rs` line 186, reached through winit
0.30.13's default `wayland-csd-adwaita`, which eframe enables with `winit/default`
(`eframe/Cargo.toml:67`). It draws the Wayland client-side title bar, not egui text.)

`ResolvedFontSpec::family` is a *name* (`native-theme/src/model/font.rs:244`).
A name becomes bytes in exactly two ways:

* **The system's own face** — feature `system-fonts`, on by default (§12.1).
  native-theme core gains the same-named feature, with a dependency on `fontdb`
  0.23.x (0.23.0 is already in the workspace lock, `Cargo.lock:2166-2167`) with
  `default-features = false, features = ["fs", "fontconfig", "memmap"]`, fontdb's
  own default set (fontdb 0.23.0 `Cargo.toml` lines 63–68; the fontdb line
  numbers below are all in its `src/lib.rs`): with `memmap` a font file is
  mapped to be described instead of read in full (`load_font_file_impl`, lines
  267–271, which records the face as `Source::File`, lines 233–234), and a
  face's bytes are mapped only when asked for (`Source::with_data`, lines
  912–917). The mapping's `unsafe` is fontdb's own, as every dependency's is,
  not this repository's. Then the function

  ```rust,ignore
  pub fn native_theme::fonts::system_face(
      family: &str,
      weight: u16,
      style: native_theme::theme::FontStyle,   // native-theme/src/model/font.rs:59-66, re-exported via lib.rs:154-155
  ) -> Option<native_theme::fonts::SystemFace>;
  pub struct SystemFace {
      pub data: std::sync::Arc<[u8]>,
      pub index: u32,
      pub family: std::sync::Arc<str>,           // the chosen face's own family: FaceInfo::families' first entry
      pub weight: u16,                           // the chosen face's own weight: fontdb's FaceInfo::weight
      pub style: native_theme::theme::FontStyle, // the chosen face's own style: fontdb's FaceInfo::style
  }
  pub fn native_theme::fonts::is_macos_system_ui_family(family: &str) -> bool;
  ```

  It loads the system font database once per process
  (`Database::load_system_fonts`, line 400) into a crate-private
  `std::sync::OnceLock<fontdb::Database>` in `native_theme::fonts`, which every
  later call shares, so no fontdb type enters the public API. Loading it per
  call would stall every theme build (rationale §3.11 gives the measurement),
  and `FontPlan::from_system` calls `system_face` twice. The cost of the
  shared database: a font installed while the application runs is seen after a
  restart, as with iced's own font system. It describes every face to `select_face` (below) from fontdb's `FaceInfo` — its
  `families` (line 836), `style` (line 846), `weight` (line 849, the OS/2
  `usWeightClass`, lines 1161–1164, so a variable face takes part with its
  default instance's weight) and `stretch` (line 852, as ttf-parser 0.25.1's
  `Width::to_number`, `src/tables/os2.rs` line 102) — and copies the chosen
  face's bytes once into the `Arc<[u8]>` (`Database::with_face_data`, lines
  721–727), a whole collection file for a `.ttc` face, with its `index`, its
  own `weight` and its own `style`, and its own family name as fontdb records
  it: the first entry of `FaceInfo::families`, where "the first family is
  always English US, unless it's missing from the font" (lines 826–827, the
  field at line 836). This crate reads only the bytes; the family is the name
  the sibling connectors need (§8.8).

  **The macOS system UI font is found by its file, not by its name.** A name
  lookup misses it: fontdb on `macos-latest` holds the system UI font as
  `.SF NS`, not as "SF Pro", the name platform-facts §1 gives it
  (`docs/platform-facts.md:59-67`) and `macos-sonoma` states
  (`native-theme/src/presets/macos-sonoma.toml:47`, `:403`) — the v0.5.9 iced
  showcase's macOS capture shows exactly that miss (`docs/todo.md`, *macOS: the
  stated font family "SF Pro" is not a family iced's font database holds*,
  which keeps the open question of what the preset should state). The most
  native answer asks the OS which file its system font is. On macOS, when
  `family` names the system UI font — caselessly equal to "SF Pro", the test
  `is_macos_system_ui_family` makes, or to the family Core Text reports for
  that font, which is the font the live reader
  reads through `NSFont::systemFontOfSize` (`native-theme/src/macos.rs:205-206`,
  `:221`) — `system_face` creates it with `CTFont::new_ui_font_for_language`
  (`CTFontCreateUIFontForLanguage`, `CTFontUIFontType::System`, size `0.0` for
  the type's default, no language; objc2-core-text 0.3.2
  `src/generated/CTFont.rs` line 574), and for an `Italic` or `Oblique` style
  its italic, `CTFont::copy_with_symbolic_traits` (line 662, compiled only with
  the crate's `CTFontTraits` feature, line 660) with a null matrix and
  `CTFontSymbolicTraits::ItalicTrait` as value and mask (the upright font where
  Core Text has none). Both the family name and the file come from
  `CTFont::attribute` (`CTFontCopyAttribute`, line 843), which returns an
  `Option`: `kCTFontFamilyNameAttribute` downcast to a `CFString`, and
  `kCTFontURLAttribute` downcast to a `CFURL` (`CFRetained::downcast`,
  objc2-core-foundation 0.3.2 `src/retained.rs` line 204), whose path is
  `CFURL::to_file_path` (`src/url.rs` line 159). `CTFont::family_name` is not
  used: its binding `expect`s a non-null result (`CTFont.rs` lines 977,
  982-983), a panic path this design does not take. That one file is loaded
  into a fontdb `Database` of its own (`Database::load_font_file`, line 258)
  and `select_face` picks among its faces by width, style and weight, called
  with the family that file itself records — the first entry of its first
  face's `FaceInfo::families` (lines 824–836) — since every face in that file
  is the system UI font, and neither "SF Pro" nor Core Text's family name need
  be a family the file records. Every other family, on
  macOS as elsewhere, goes through the name lookup.
  `is_macos_system_ui_family` is a pure predicate, exported unconditionally
  in `native_theme::fonts` and needing no `fontdb`: `true` exactly when
  `family` is caselessly equal (the `unicase` matching of `select_face`,
  below) to "SF Pro", the name `docs/platform-facts.md:59-67` gives the macOS
  system UI font. `system_face`'s macOS branch uses it beside its Core Text
  family comparison, and the gpui connector uses it alone (§8.8).

  The Core Text calls are `unsafe fn`s, and the two attribute keys are statics
  declared in `extern` blocks (`src/generated/CTFontDescriptor.rs` lines 46-55
  and 79-88), so reading them is `unsafe` too. All of it lives in
  native-theme's `macos` module, which already permits FFI `unsafe`
  (`native-theme/src/macos.rs:7`), behind one safe crate-private
  function that returns the family name Core Text reports for the font and the
  font's file path, or `None`; `system_face` itself stays safe.
  The maintainer approved this `unsafe` explicitly on 2026-09-25 (the
  alternatives are in the rationale); no other `unsafe` is part of this design.
  The dependency is `objc2-core-text` 0.3.2, already in the workspace lock
  through `objc2-app-kit` (`Cargo.lock:5569-5570`), with its `CTFont`,
  `CTFontDescriptor` and `CTFontTraits` features, and `objc2-core-foundation`'s
  `CFURL` feature; both are macOS-only, under `system-fonts`. A `None` from Core
  Text, a failed downcast, or a file that yields no face is a `None` from
  `system_face`, as a family with no face is — nothing is substituted.

  `FontPlan::from_system(t: &ResolvedTheme) -> FontPlan` (§4.9)
  calls it for `defaults.font` and `defaults.mono_font` with each spec's
  `family`, `weight` and `style`, records the face at the weight and style
  `SystemFace` reports, not at the ones asked for, and keeps its lookup notes
  in the plan for `font_definitions` to return; [`from_system`],
  `to_egui_atlas` and [`from_preset`] use it whenever the feature is on (§4.6),
  each on the atlas's light `ResolvedTheme`. That one variant's fonts serve
  both schemes: egui holds one `FontDefinitions` per `Context`, not one per
  `egui::Theme`, `ctx.theme()` before the first pass is
  `Options::fallback_theme` (`egui/src/memory/mod.rs:363-368`), so no choice
  made at install could follow the scheme, and no bundled preset states a
  different `defaults.font` or `defaults.mono_font` family, weight or style
  for its two variants. **UNVERIFIED
  until run on the platform:** that the families the Windows reader reports,
  and macOS's monospace family, are names fontdb records for those faces, and
  that the Core Text route resolves the macOS system UI font under both the
  live reader's name and "SF Pro"; `system_faces_resolve` (§13) runs on the
  screenshot workflow's `macos-latest` and `windows-latest` runners, over the
  reader's theme and the platform's bundled preset (an *Open verification
  item*).
* **The application's own bytes** — `FontPlan::face` and
  `FontPlan::variable_face` (§4.9), e.g. `include_bytes!` fonts, each
  registered with its family and style — a `face` at its own weight, a
  `variable_face` at the weight asked for — which the plan hands to the same
  `select_face`, each described at width `5`, normal: a registered face states
  no width, so the font-width step below keeps every one of them.

**One matcher, in native-theme.** Both routes pick a face with one pure
function that native-theme exports unconditionally — it needs no `fontdb`,
only `unicase`, which native-theme then depends on without a feature:

```rust,ignore
pub fn native_theme::fonts::select_face(
    faces: &[native_theme::fonts::FaceTraits<'_>],
    family: &str,
    weight: u16,
    style: native_theme::theme::FontStyle,
) -> Option<usize>;                              // an index into `faces`
#[derive(Clone, Copy, Debug)]
pub struct FaceTraits<'a> {
    pub families: &'a [&'a str],               // every family name the face records
    pub width: u16,                            // OS/2 usWidthClass, 1..=9; 5 is normal
    pub style: native_theme::theme::FontStyle,
    pub weight: u16,                           // OS/2 usWeightClass, the CSS weight
}
```

It considers only faces one of whose family names equals `family` under CSS
Fonts Module Level 4, §5.1 "Localized name matching": "User agents must match
these names case insensitively, using the "Default Caseless Matching"
algorithm", applied without normalisation — `unicase::UniCase` (2.9.0, already
in the workspace lock, `Cargo.lock:8711-8712`), which implements full Unicode
case folding (line 10 of its `src/lib.rs`) and contains no `unsafe`. **No other
family is ever substituted**: an empty candidate set is `None`. Among the
candidates it applies the three steps of CSS Fonts Module Level 4, §5.2
"Matching font styles" (W3C Working Draft 13 September 2026,
`https://www.w3.org/TR/2026/WD-css-fonts-4-20260913/`), with the model's
`normal` width: **font-width** (normal first, then narrower widths descending,
then wider ascending); **font-style** (`italic`: italic, then oblique, then
normal; `oblique`: oblique, then italic, then normal; `normal`: normal, then
oblique, then italic); and **font-weight**, quoted: "If the desired weight is
inclusively between 400 and 500, weights greater than or equal to the target
weight are checked in ascending order until 500 is hit and checked, followed by
weights less than the target weight in descending order, followed by weights
greater than 500, until a match is found. If the desired weight is less than
400, weights less than or equal to the desired weight are checked in descending
order followed by weights above the desired weight in ascending order until a
match is found. If the desired weight is greater than 500, weights greater than
or equal to the desired weight are checked in ascending order followed by
weights below the desired weight in descending order until a match is found."
fontdb's own `query` is not used: it compares family names with a
case-sensitive `==` (line 667), so a name that differs only in case would miss,
and its weight step follows CSS Fonts Level 3 with a `450` cut-off (lines
1207–1208 and 1278–1336), which differs from Level 4 for an intermediate weight
— asked for `400` from faces `300` and `450`, Level 4 picks `450` and fontdb
`300`. Like `query`, it never falls back past the family it is given (lines
661–677).

At most two faces are installed either way: the one matching `defaults.font` as
`Proportional` and the one matching `defaults.mono_font` as `Monospace`.

**What happens on each failure — egui's own face stays, and a `Note` says why:**

| failure | result |
|---|---|
| feature `system-fonts` off and no plan | the atlas maps font **sizes** (scaled per §8.6) and **colours** only and leaves both families at egui's built-in `Proportional` and `Monospace`: **"right metrics, wrong typeface"**; no `Note`, nothing was asked for |
| no face of that family, or its file cannot be read (`with_face_data` → `None`, fontdb lines 721–727) | that family keeps egui's face; `Note::FontFamilyUnavailable { family }` |
| the bytes do not parse at their index | the face is dropped before epaint sees it and the family keeps egui's face; `Note::FontDataInvalid { family }` (below) |
| the chosen face's own weight differs from the one asked for, and the face has no `wght` axis | the CSS-nearest face renders at its own weight; `Note::FontWeightAxisUnsupported { family }`. The face's own weight is the one `select_face` chose it by: `SystemFace::weight` (fontdb's `FaceInfo::weight`) for a system face, the registered weight for an application face. A face with a `wght` axis gets the asked weight as a coordinate (§8.3) and no `Note`; a face whose own weight is the asked one gets none either |
| a face registered with `FontPlan::variable_face` has no `wght` axis | it is described to `select_face` at the asked weight (§4.9), so it is chosen as an exact match, but nothing can set its weight: it renders at its own and is a `Note::FontWeightAxisUnsupported { family }` |
| the style asked for has no face | CSS's style fallback above picks the nearest style; an `Italic` or `Oblique` request that lands on a `normal` face is not synthesised (§8.4) |

**Validated before epaint sees it.** `FontsImpl::new` parses every registered
face eagerly and **panics** on a parse failure — `.unwrap_or_else(|err| panic!(..))`
(`epaint/src/text/fonts.rs:990`), in release as in debug, from inside
`begin_pass`, with no recovery point. The only fallible step of the parse is
`skrifa::FontRef::from_index(data, index)?` (`epaint/src/text/font.rs:386-388`):
the charmap, outline glyphs and metrics that follow are infallible, and the
hinting instance is `.ok()`-ed (`:402-415`). So the connector makes that same
call first, with a **direct** `skrifa` dependency carrying exactly epaint
0.36.2's requirement, `0.44.0` (`epaint/Cargo.toml:149-150`), and a gate that
`cargo tree -d -p native-theme-egui` lists a single `skrifa` — the check can
then never drift from the parser epaint runs. `Builder::build` validates every
face of its plan, keeps only those that parse and whose `head` states a
non-zero `unitsPerEm` (epaint divides by it, `epaint/src/text/font.rs:215-218`,
and debug-asserts the result, `:181-184`), and records
`Note::FontDataInvalid { family }` for each dropped one; `fonts::font_definitions`
applies the same check and returns the same notes beside the definitions (§4.9),
so a plan handed to it directly can never reach the panic either. Application
bytes and system bytes are checked alike.

* `Context::add_font` is **never** used: it de-duplicates by name only
  (`context.rs:2134-2144`, with upstream's own `TODO` at `:2111`), so
  re-applying a theme with the same font name but different bytes would be
  silently ignored. `install` always uses `set_fonts` (`context.rs:2106`).

The connector itself never reads a file: system bytes come from native-theme's
`system_face`, application bytes from the application.

### 8.3 Weight: one per family

`FontId` is `{ size: f32, family: FontFamily }` — two fields, with upstream's
own `// TODO(emilk): weight (bold), italics, …` at
`epaint/src/text/fonts.rs:27` (struct `:21-28`). There is no weight anywhere in
`Style`, `Visuals`, `Spacing`, `TextStyle` or `FontId`. Family is `FontId`'s
only selector, so more than one weight per family requires
`FontFamily::Name`, which §8.1 forbids.

The connector therefore applies **one** weight per family, as a `wght`
variation coordinate through `FontTweak::coords`
(`epaint/src/text/fonts.rs:250`), with the tag built by `Tag::new(b"wght")`
(an infallible `const fn`: font-types `src/tag.rs` lines 30–32, identical in
0.12.2 through 0.12.5, every release epaint 0.36.2's caret requirement `0.12.2`
admits (`epaint/Cargo.toml:111-112`); epaint re-exports the type
(`epaint/src/text/text_layout_types.rs:11`, glob at `epaint/src/text/mod.rs:18`), so
it is spelled `egui::epaint::text::Tag` and needs no `font-types` dependency) — **never** through
the `[u8; 4]`, `&[u8; 4]` and `&str` `IntoTag` impls, which `expect`
(`epaint/src/text/text_layout_types.rs:396`, `:403`, `:410`) and are banned by
the no-panic rule.

Three honest limits, all silent upstream:

* On a **static** font a `wght` coordinate is ignored. The crate-private
  `fonts::supports_weight_axis` finds out; it returns `false` for
  both "static font" and "unparseable bytes", because
  `FontData::variation_axes` early-returns an empty `Vec` for both
  (`epaint/src/text/fonts.rs:153-173`) and the two are not distinguishable
  through any public epaint API. A face whose own weight differs from the asked
  one and which has no `wght` axis, and a face registered with
  `FontPlan::variable_face` that has none, emit
  `Note::FontWeightAxisUnsupported` (§8.2).
* An out-of-range coordinate is silently clamped to the axis range. epaint hands
  skrifa the face's `tweak.coords` followed by the call's
  (`epaint/src/text/font.rs:577-578`); skrifa 0.44.0, epaint's requirement
  (`epaint/Cargo.toml:149-150`), ignores a tag with no matching axis, clamps to
  the axis range and keeps the last setting per axis (`src/variation.rs` lines 136–140)
  — which is also why a per-call `RichText::variation` wins over the face's `wght`.
* **Bold headings against a regular body are not carried by the theme.**
  `defaults.font.weight`, `defaults.mono_font.weight`, the four
  `text_scale.*.weight` and the 19 per-widget `font.weight` values cannot all
  reach one family. Applications reach per-call weight with
  `RichText::variation(..)` (`widget_text.rs:200-205`), and `text_role_weight()`
  tells them what to ask for.

One convenient property: a `wght` coordinate does **not** change line height.
`FontsView::row_height` passes `VariationCoords::default()`
(`epaint/src/text/fonts.rs:870-872`), so weight can never desync text metrics
from §6.15's line-spacing calculation.

### 8.4 Slant

`grep italic egui/src/style.rs` returns zero hits: there is no italic switch
anywhere in `Style`. Three mechanisms exist and none is a global theme sink:

1. Register an italic **face** — true italic, correct metrics. This is what
   `FontPlan::face(.., FontStyle::Italic, ..)` participates in, and what
   `FontPlan::from_system` fetches when the theme's `style` is `Italic` or
   `Oblique` (§8.2); it is the only mechanism that can distinguish `Italic` from
   `Oblique`, and then only where both faces exist.
2. A `slnt` or `ital` variation coordinate, only on variable fonts carrying the
   axis.
3. `RichText::italics()` (`widget_text.rs:282-287`) → `TextFormat::italics`
   (`epaint/src/text/text_layout_types.rs:507`) → a **hardcoded 0.25 shear of
   the glyph quad** (`epaint/src/text/text_layout.rs:1173-1200`), which ignores
   the font's `italic_angle` and cannot distinguish the two styles. Per call
   site only.

In practice every bundled preset and every live reader resolves `style` to
`Normal`. An `Italic` or `Oblique` request gets a true italic or oblique face
where the system or the plan has one (§8.2's style step); where only an upright
face exists it renders upright — mechanism 3 is per call site and is never
applied to a whole family, so nothing is sheared behind the application's
back.

### 8.5 The five `TextStyle` slots

| `TextStyle` | decl | egui default | driven from |
|---|---|---|---|
| `Small` | `style.rs:74` | `9.0` Proportional (`style.rs:1418`) | `text_scale.caption.size`. Selected in egui only by `RichText::small()` (`widget_text.rs:292`) and `small_raised()` (`:298`), which `WidgetText::small` (`:692`) and `Ui::small` (`ui.rs:1736`) call — **no built-in widget selects it** |
| `Body` | `style.rs:77` | `13.0` Proportional (`style.rs:1419`) | `defaults.font.size`. `FontSelection::Default`'s fallback (`style.rs:151-152`), so it is what a `Label` and a `TextEdit` both get |
| `Monospace` | `style.rs:80` | `13.0` Monospace (`style.rs:1422`) | `defaults.mono_font.size`. The only uncontested slot |
| `Button` | `style.rs:85` | `13.0` Proportional (`style.rs:1420`) | `button.font.size` |
| `Heading` | `style.rs:88` | `18.0` Proportional (`style.rs:1421`) | `text_scale.section_heading.size` |

Every size in the last column is written as `scaled_text_size(size, prefs)`
(§8.6), and so is every per-role `override_font_id` below.

Each written size first passes §6's rule: a scaled size that is not a positive
normal `f32` keeps the starting style's size for that slot (egui's on the base
style — the *egui default* column — and the base style's in a role cell; for
`override_font_id`, no override is written) and emits `Note::ValueSanitised`
for the leaf. epaint debug-asserts a finite, positive scale
(`epaint/src/text/font.rs:181-184`), and a `+∞` size panics in release inside
the rasteriser (measured, egui 0.36.2). Positive means a positive normal `f32`
(`f32::is_normal`): a subnormal size underflows epaint's scale
`size · pixels_per_point / unitsPerEm` (`epaint/src/text/font.rs:567-568`,
`:215-218`) to `0.0` and trips the same assert (measured, egui 0.36.2, debug:
Body `1e-45` tripped it; `1e-42` and `1e-38` passed). A finite size so large that one glyph
outgrows the font atlas panics in release as well: a glyph wider than the
atlas trips an `assert!` (`epaint/src/texture_atlas.rs:226-229`), and one taller
than the room left takes `allocate`'s overflow branch (`:243-249`), after which
its pixels are written past the image by an index
(`epaint/src/text/font.rs:283-288`); at `pixels_per_point` 1 on egui's
default 2048 atlas, 2000 panicked at `epaint/src/text/font.rs:288` and 1500
passed). That bound depends on `max_texture_side` and `pixels_per_point` at run
time, so it is a stated residual (§14 item 44).

**`egui::Button` does not honour `TextStyle::Button`.** `Button::new` sets
`.fallback_font(TextStyle::Button)` (`widgets/button.rs:49`) and `atom_ui` then
overwrites it at `:358-360` with `text_style.font_id` from
`Style::button_style` → `Style::widget_style`, which is
`override_font_id.unwrap_or_else(|| TextStyle::Body.resolve(self))`
(`widget_style.rs:122`, `:137`); `AtomLayout::fallback_font` is a plain setter
(`atomics/atom_layout.rs:147-150`), so the second call wins. `text_styles[Button]`
is still written, because `ComboBox` (`containers/combo_box.rs:358`),
`ProgressBar` (`widgets/progress_bar.rs:193`), `CollapsingHeader`
(`containers/collapsing_header.rs:522`) and `Style::drag_value_text_style`
(`style.rs:1434`) do read it. **Per-widget typography therefore travels on
`Style::override_font_id`** (`style.rs:255`, checked first at `:159-162`),
written in every role scope that has its own font.

`TextStyle::Name(Arc<str>)` (`style.rs:94`) is never emitted (§5.8 items 4 and 5).
`DialogTitle` and `Display` are reachable through `text_role_font()` +
`RichText::font(..)`.

### 8.6 The text-scaling factor

`AccessibilityPreferences::text_scaling_factor` is, in native-theme's own words,
the factor to "Multiply font sizes by … when honoring the user's preference for
larger text" (`native-theme/src/lib.rs:251-253`). `Builder::accessibility`
applies it: every text size the atlas writes — the five `text_styles` slots of
§8.5 and every role scope's `override_font_id`, in the two base styles and in
every role style alike — is `scaled_text_size(size, prefs)`, the size times the
factor when the factor is finite and positive and the size unchanged otherwise.
That is the function, and the degenerate-factor rule, both siblings export
(`connectors/native-theme-iced/src/lib.rs:479-489`,
`connectors/native-theme-gpui/src/lib.rs:443-455`). Geometry is not scaled:
paddings, radii, widths, `interact_size` and icon sizes keep the theme's values,
as in gpui ("Text sizes carry the accessibility text-scaling factor; widths,
paddings, radii and icon sizes do not", `connectors/native-theme-gpui/src/geometry.rs:17-19`).

**Why the `FontId` sizes and not the zoom factor.** egui's zoom factor is
multiplied into `pixels_per_point` (`egui/src/context.rs:455`) — "Make larger to
make everything larger" (`:2317`) — so it scales every length, not text. It is
also the user's own control: with `Options::zoom_with_keyboard`, `true` by
default (`egui/src/memory/mod.rs:242-245`, `:335`), `Context::end_pass` changes
it on Cmd/Ctrl+Plus and Minus (`egui/src/context.rs:2446-2447`), so a connector
that set it would overwrite the user's zoom. `FontId::size` — "Height in points"
(`epaint/src/text/fonts.rs:22-23`) — is egui's only text-only sink.

**What the application scales.** The accessors that return a text size
(`text_role_font`, `text_role_line_height`, `list_header_font`, `font_size`,
`mono_font_size`) take the same `&AccessibilityPreferences` and return the
scaled value (§4.7). A size an
application reads from `ResolvedTheme` itself is the theme's own, unscaled, and
`scaled_text_size` scales it.

**What the factor does not reach.** A height the theme states is geometry and is
not scaled, so where egui takes it as an exact size — the row height an
application passes to `egui_extras::Table` from `list.row_height`, a
`Panel::exact_size` from `toolbar.bar_height` (§6.10) — text scaled above
`1.0` can grow past it; gpui meets the same case by turning a stated height into
a minimum above a factor of `1.0` (`connectors/native-theme-gpui/src/geometry.rs:160-172`),
and in egui that choice is the application's, at the call. Scaled Body text also
raises the slider's `at_least` floor, which §6.6's `expansion`, computed from
the same row height, answers so the knob keeps its diameter, and is what
§6.15's line spacing targets.

### 8.7 `ResolvedFontSpec::defined_size`

`ResolvedFontSpec` carries `defined_size: Option<FontSize>` beside `size`
(`native-theme/src/model/font.rs:249-266`): the unit and number the preset or
platform reader stated, `FontSize::Pt` or `FontSize::Px` (`:79-89`), kept so a
display can show what the source said. `size` is always logical pixels
(`:245-248`). The connector feeds **`size`**, scaled per §8.6, into every
`FontId`, and never `defined_size`:

* epaint's unit is the logical point — "`epaint` uses logical _points_ as its
  coordinate system", related to physical pixels by `pixels_per_point`
  (`epaint/src/lib.rs:12-15`) — which is native-theme's logical pixel one for
  one (§2's lengths rule). A `FontSize::Pt` is a typographic point, and
  resolution has already applied its `dpi / 72` to produce `size`
  (`FontSize::to_logical_px`, `native-theme/src/model/font.rs:115-120`); its bare
  number in `FontId::size` would be wrong by exactly that factor.
* `defined_size` is `None` where the source stated no size (`:263-264`), so it
  could not be the total input a `FontId` needs anyway.

No `Style` field shows a size to a person, so `defined_size` has no sink here;
an application that displays a theme's sizes, as a showcase's Widget Info does,
reads it from `ResolvedTheme` directly.

### 8.8 The sibling connectors

`macos-sonoma` states the macOS system UI font as "SF Pro"
(`native-theme/src/presets/macos-sonoma.toml:47`, `:403`), the platform's
documented name (`docs/platform-facts.md:59-67`), and keeps stating it: no
single name serves the three toolkits — fontdb records the font as `.SF NS`
(§8.2), Core Text resolves it as `.AppleSystemUIFont`, Apple documents "SF
Pro" — so each connector maps the name the way its toolkit's own font system
needs. One toolkit gets bytes, one a name and one an alias:

* **egui gets the face's bytes.** egui has no font system of its own (§8.2),
  so this crate registers `SystemFace::data` through `FontPlan::from_system`
  (§4.9).
* **iced gets a name, from `system_face`.** iced draws through cosmic-text
  over fontdb 0.23, and its own database — the one the iced showcase probes
  through `iced::advanced::graphics::text::font_system()`
  (`connectors/native-theme-iced/examples/showcase-iced.rs:5687`) — holds the
  macOS system UI font under the family its file records, `.SF NS`, not "SF
  Pro" (§8.2). So iced names the face by the family its database records,
  chosen by the same selection egui's bytes come from: one implementation.
  cosmic-text 0.15.0 asks fontdb's `Database::query` for the family
  (`src/font/system.rs` lines 340–348), which takes a face when any entry of
  its `families` equals the name (fontdb 0.23.0 `src/lib.rs` line 667), so
  `SystemFace::family`, the first entry, matches.
  native-theme-iced gains a `system-fonts` feature forwarding native-theme's,
  on by default for the reason §12.1 gives this crate's — the native look is
  not opt-in — and it adds no crate to iced's graph, which already holds
  fontdb 0.23.0 through cosmic-text (§12.1). Behind it:

  ```rust,ignore
  pub fn native_theme_iced::system_font_family(
      spec: &native_theme::theme::ResolvedFontSpec,
  ) -> std::sync::Arc<str>;
  ```

  It returns `native_theme::fonts::system_face(&spec.family, spec.weight,
  spec.style)`'s `family` where that finds a face, else `spec.family`. Its doc
  says that the first call loads the system font database, which every later
  call shares (§8.2), and that each call copies the chosen face's bytes, so it
  is called when the theme changes, not per frame; and that it returns the
  family iced's database holds
  for the face `system_face` chooses — on macOS the system UI font's `.SF NS`
  for "SF Pro". The crate's *Font Configuration* docs
  (`connectors/native-theme-iced/src/lib.rs:90-143`) use it in place of the
  stated family. The iced showcase uses it for its theme fonts: its
  `font_from_database` (`connectors/native-theme-iced/examples/showcase-iced.rs:5680`)
  is given the resolved name, and its inspector keeps showing the stated
  family. `family_label` (`:5593-5603`) shows the stated family only where the
  drawn family equals it, so it compares the drawn family with the resolved
  name instead; macOS then shows "SF Pro" without "not found".
* **gpui gets an alias, not `system_face`.** gpui resolves a family name
  through the platform's own font system — on macOS Core Text, memory fonts
  first, then `system_source.select_family_by_name` (gpui-pre-macos 0.3.6
  `src/text_system.rs` lines 282–289) — and has a documented name for exactly
  this font: `.SystemUIFont`, "used to identify the system UI font, which
  varies based on platform" (gpui-pre 0.3.6 `src/text_system.rs` line 1295),
  which its macOS text system maps to `.AppleSystemUIFont` (gpui-pre-macos
  0.3.6 line 282, through `font_name_with_fallbacks`, gpui-pre 0.3.6 lines
  1420–1430). So in `to_theme` (`connectors/native-theme-gpui/src/lib.rs:184`)
  and the config path (`connectors/native-theme-gpui/src/config.rs:63`), on
  macOS (`cfg!(target_os = "macos")`), a `defaults.font.family` for which
  `is_macos_system_ui_family` holds becomes `.SystemUIFont`; every other
  family, and `mono_font`, stays as stated. Core Text then supplies its own
  system UI font object, the most native route (rationale §3.11 says why not
  its bytes). The gpui showcase's
  inspector shows the stated family
  (`connectors/native-theme-gpui/examples/showcase-gpui/inspector.rs:191`) and
  says it is drawn as the system UI font where it is mapped. A unit test, run
  on every platform, checks the mapping: "SF Pro" and "sf pro" become
  `.SystemUIFont` on macOS and stay as they are elsewhere, and every other
  name stays everywhere.

**Verified on the macOS runner** (§13, §15). iced: the `macos-sonoma`
capture's inspector shows "SF Pro" with no "not found"; a runner test asserts
that `system_font_family` of `macos-sonoma`'s `defaults.font` is a family in
iced's font database (`font_system()`'s `db`), and a second prints, without
asserting, whether the same holds for its `mono_font`, "SF Mono" —
**UNVERIFIED** whether any database holds it under that name. gpui: the gpui showcase's macOS
`--screenshot` run logs whether `cx.text_system().all_font_names()`
(gpui-pre 0.3.6 `src/text_system.rs` line 284) holds the stated mono family
"SF Mono" — **UNVERIFIED**. It is a log line in the application, not a test,
because a `#[gpui::test]` gets gpui's `NoopTextSystem` (gpui-pre 0.3.6
`src/app/test_context.rs` line 131 and `src/platform/test/platform.rs` lines
124–131). If the name is absent, the remedy is `system_face`'s bytes through
`TextSystem::add_fonts` (`src/text_system.rs` line 295), an open item (§15) not
designed here.

---

## 9 -- Icons

### 9.1 Scope

The connector does key and URI construction, `IconData` → `ImageSource` /
`Image` (a monochrome SVG coloured with native-theme's function, §9.2), alt
text, animation scheduling and cache invalidation. It does **no**
decoding, **no** rasterising and **no** loader installation. It does keep
exactly one thing: the `egui::TextureHandle` of each uploaded raster icon, in
`ctx.data_mut()` keyed by the icon URI, because dropping that handle frees the
texture (`epaint/src/texture_handle.rs:25-29`). `icons::forget_icons` releases
them.

egui core ships **zero image decoders**: `Loaders::default()` starts with an
empty image-loader vector (`egui/src/load.rs:625`), so `try_load_image` returns
`LoadError::NoImageLoaders` (`egui/src/context.rs:3866-3868`) until the
application calls `egui_extras::install_image_loaders(&ctx)`
(`egui_extras/src/loaders.rs:58`). The application therefore adds:

```toml
egui_extras = { version = "0.36.2", default-features = false, features = ["svg"] }
```

This is a deliberate, argued deviation from the gpui connector, which enables
`native-theme/svg-rasterize` by default. `egui_extras` 0.36.2's `svg` feature pulls
`resvg 0.45.1` while `native-theme`'s `svg-rasterize` pulls `resvg 0.48.1`
(`native-theme/Cargo.toml:52`) — two semver-incompatible pre-1.0 minors — so enabling both compiles two copies of
resvg, usvg and tiny-skia into one binary. Letting `egui_extras` own
rasterisation also gets DPI-correct re-rasterisation for free
(`Image::load_for_size`, `egui/src/widgets/image.rs:350-355`). In-connector
rasterisation stays an **additive** opt-in, never a silent switch: the
`svg-rasterize` feature only forwards native-theme's, and an application that
wants it calls `native_theme::rasterize::rasterize_svg`
(`native-theme/src/rasterize.rs:39`) and hands the `IconData` to
`icons::to_color_image`.

The connector's default features are `material-icons`, `lucide-icons`,
`system-icons` and `system-fonts` (§12.1), each a passthrough of the
same-named `native-theme` feature (`native-theme/Cargo.toml:32-45`; `system-fonts`
is the one §8.2 adds): the icon sets and the platform's own
typeface are on out of the box, because the native look must not be opt-in.
`svg-rasterize` is the one sibling default left off, for the reason above — it
changes no pixel of the `egui_extras` path and would only add the second
resvg.

### 9.2 What the URI carries

Every egui loader layer caches on the URI **string**
(`egui/src/load/bytes_loader.rs:15-26`,
`egui/src/load/texture_loader.rs:49-59`), so two renderings that shared a URI
would see the first served forever: `DefaultBytesLoader::insert` keeps the first
bytes per URI (`egui/src/load/bytes_loader.rs:15-26`). The URI is therefore
`bytes://native-theme/<hash>`, `<hash>` the 16 lower-case hex digits of a 64-bit
hash of the **final** bytes, after any colouring below —
`std::hash::DefaultHasher::new()` over the bytes and their length and, for an
`IconData::Rgba`, its width and height. Its keys are fixed: std documents a
hasher from `DefaultHasher::new` as the same as every other created through
`new` (Rust std `library/std/src/hash/random.rs`, lines 97–107), where a
`RandomState`'s hasher is keyed afresh per instance, so the hash is stable
within a process, which is all a `Context`'s cache needs. No lookup input — role
or name, set, icon theme, size — enters the URI: those inputs decide the bytes,
and equal bytes are equal pixels, so two keys whose final bytes are equal share
one cache entry correctly; the size need not be named either, since both SVG
caches keep one rendering per size hint under a URI
(`egui_extras/src/loaders/svg_loader.rs:19`, `:71-73`;
`egui/src/load/texture_loader.rs:14-15`, `:49-59`). The URI so changes whenever the
pixels do: a tint, the text colour a freedesktop icon is loaded in, another
frame of an `AnimatedIcon::Frames` (under one URI every frame would draw frame
0 forever), an edited icon file, a provider's differing bytes — none of which
the key has to name. The cost is one hash over the icon's bytes per call, a
few KB for an SVG.

**The icon-theme name is optional, and nothing stands in for a missing one.**
`Resolved::icon_theme` and `SystemTheme::icon_theme` are
`Option<Cow<'static, str>>` (`native-theme/src/model/resolved.rs:264`,
`native-theme/src/lib.rs:483`), and `SystemTheme::icon_theme_for`, which
v0.6.0 adds to native-theme (§4.6), returns an `Option<&str>`, `None` where the TOML states no theme and system
detection fails; `system_icon_theme()` and `detect_icon_theme()` return
`Result<String>` and say why (`native-theme/src/model/icons.rs:542-559`,
`:572`, `:588`). A freedesktop icon comes only from the chosen theme or a theme
in its `Inherits=` chain — never from `hicolor` unless that is the theme, nor
from a loose file or `/usr/share/pixmaps` — and is `None` otherwise
(`native-theme/src/icons.rs:80-86`); with no theme set and detection failing,
`FreedesktopLoader::load` searches no theme at all (`:167-173`, `:187-194`).
The connector keeps that shape. It never detects a theme itself and never turns
an `Err` or a `None` into a name: `IconKey::icon_theme` (§4.10) is called only
with a name the application has, a key built without it names no theme, and an
icon the icon layer did not find stays `None` — no other theme's or
set's icon is substituted.

**A monochrome SVG icon is coloured in its bytes.** `egui_extras` rasterises
with `usvg::Options::default()` (`egui_extras/src/loaders/svg_loader.rs:39`), so
an implicit fill, and a `currentColor` no `color` resolves, draw black, and
`egui::Image::tint` is a multiply (`egui/src/widgets/image.rs:222`) that leaves
black black: a Material or Lucide icon would draw black on a dark scheme. Both siblings recolour the
bytes — iced's `colorize_monochrome_svg`
(`connectors/native-theme-iced/src/icons.rs:277`) and gpui's `colorize_svg`
(`connectors/native-theme-gpui/src/icons.rs:1299`) — and v0.6.0 adds one
function to native-theme, which this crate calls; the siblings switch to it
later (`docs/todo.md`):

```rust,ignore
pub fn native_theme::icons::colorize_monochrome_svg(
    svg: &[u8],
    color: native_theme::color::Rgba,
) -> Vec<u8>;
```

It is gpui's algorithm: `currentColor` and explicit black fills and strokes
(`black`, `#000000`, `#000`) become the colour's `#rrggbb`, and where none of
them occurs a `fill` is injected into a root `<svg>` tag that has none. Bytes
that are not UTF-8 come back unchanged, and the colour's alpha is discarded —
an SVG `fill` or `stroke` attribute takes opaque hex. gpui's unit tests
(`connectors/native-theme-gpui/src/icons.rs:2053-2225`) are copied into it. [`icons::to_image_source`] applies it to a bundled set's `IconData::Svg` bytes when the
key has a tint (`IconKey::tint`, as `Color32::to_srgba_unmultiplied`,
`ecolor/src/color32.rs:248`), so the colour is in the pixels and, through
their hash, in the URI; without a tint a bundled set's bytes are used as they
are, right for a full-colour icon. An `IconData::Rgba` is uploaded as it is.
`egui::Image::tint` stays out of the URI: it is a draw-time multiply that does
not change the texture.

**A freedesktop icon is loaded in the text colour.** `FreedesktopLoader::color`
and `color_opt` (`native-theme/src/icons.rs:143-152`) write the colour into a
GTK-convention symbolic icon's bytes (`native-theme/src/freedesktop.rs:246-248`,
`:516-541`) and leave a `currentColor` (Breeze-convention) icon unchanged
(`:522`), which usvg draws in the `color` the icon's own stylesheet sets —
Breeze's `current-color-scheme` classes, per variant (`breeze`, `breeze-dark`)
— and black only where none is set (usvg 0.45.1, the version `egui_extras`'
`resvg` 0.45.1 pulls, `src/parser/style.rs` lines 170–173). So a freedesktop
icon with no colour of its own is coloured in `c`, the key's tint where it has
one, else `defaults.text_color` of the installed atlas's `ResolvedTheme` for `ctx.theme()`
([`ThemeAtlas::from_ctx`], §4.2), the colour of the text beside it — the gpui
showcase loads its freedesktop icons in its text colour too
(`connectors/native-theme-gpui/examples/showcase-gpui/support.rs:366`). The
application loads the icon with `FreedesktopLoader::color(c)`
(`custom_icon_to_image_source` does so itself), and `to_image_source`, where
the bytes still contain `currentColor` and set no colour of their own, replaces
`currentColor` alone with `c`'s `#rrggbb` (`str::replace`, iced's step,
`connectors/native-theme-iced/src/icons.rs:292-293`), not the whole
`colorize_monochrome_svg`: that is exactly what usvg draws with `color` = `c`,
and the icon's own black stays black. "Sets no colour" means no `color`
property: the name `color` not preceded by a letter, digit, `-` or `_` (so not
`stop-color`, `flood-color`, `lighting-color`, `solid-color`,
`text-decoration-color`, nor Inkscape's `pagecolor`/`bordercolor`), then
optional whitespace and `:` or `=`; the
test is over the whole document, because usvg takes `color` from the nearest
ancestor that sets it (usvg 0.45.1, `src/parser/svgtree/mod.rs` lines 356–362,
`color` being inheritable). A full-colour icon — no `currentColor` and no GTK
foreground placeholder — and a `currentColor` icon whose stylesheet sets its
`color` pass through both untouched: an icon's own colours are never altered;
KDE's own loader colours such an icon's `.ColorScheme-*` classes from the
palette with a stylesheet of its own, in an icon theme whose `index.theme` sets
`FollowsColorScheme=true` as Breeze's does (kiconthemes 6.30.0,
`/usr/include/KF6/KIconThemes/kiconcolors.h` lines 153–161, `kiconloader.h`
lines 738–754, `kicontheme.h` lines 260–268;
`/usr/share/icons/breeze/index.theme` line 120), which native-theme's loader does not yet do (`docs/todo.md`).
On `adwaita`, one icon theme serves both variants
(`native-theme/src/presets/adwaita.toml:5`), so its light and dark icons differ
only in these bytes, and the hash gives them two URIs.

The URI is always `bytes://native-theme/…`. For `IconData::Svg` it always ends
in `.svg`, required by `egui_extras`'s `is_supported`
(`egui_extras/src/loaders/svg_loader.rs:31-33`), which `SvgLoader::load` calls
before it looks at anything else (`:59-62`), and by `DefaultTextureLoader`'s
per-size cache (`egui/src/load/texture_loader.rs:152-154`). In 0.36.2 both are
`egui::load::has_extension(uri, "svg")` (`egui/src/load.rs:315-320`), which
drops any `#…` suffix and compares the extension case-insensitively; 0.36.1 used
`uri.ends_with(".svg")`. The URI this crate writes ends in a lowercase `.svg`
and satisfies both. It holds the prefix, the hex digits and the extension and
nothing taken from a name, so no `#` occurs in it: egui reserves `#` for
animated-image frame indices (`egui/src/widgets/image.rs:892-894`), and
`has_extension` cuts the URI at it (`egui/src/load.rs:316`). For `IconData::Rgba`
the URI has no extension: it names the texture `Context::load_texture` uploads
and the handle's `ctx.data_mut()` entry, and no loader reads it (§4.10).

**That coupling has only partial mechanical protection, and the spelling
matters.** `is_supported` is a bare module-level `fn`, private and *outside* the
`impl SvgLoader` block at `egui_extras/src/loaders/svg_loader.rs:27-29`; the
module `loaders::svg_loader` is public (`egui_extras/src/lib.rs:19`, `loaders.rs:121`, feature `svg`) but the item is
not exported and `SvgLoader` itself is not re-exported at `egui_extras`'s crate
root (`egui_extras/src/lib.rs:25-32` re-exports only `DatePickerButton`, `Size`, `strip::*`,
`table::*` and `install_image_loaders`). So there is **no path**
`egui_extras::SvgLoader::is_supported` to name. What 0.36.2 does expose is the
predicate it delegates to: `egui::load::has_extension` is public
(`egui/src/lib.rs:409` declares `pub mod load`), so a unit test can assert
`has_extension(&uri, "svg")` for every URI this crate produces. That pins
today's predicate only — `is_supported` itself may change without a trace — so
the contract is still checked end to end as well. §13 must therefore carry a
test that installs the loaders and asserts that a URI this crate produced actually
loads — `egui_extras` is already a dev-dependency with its `svg` feature
(§11) and `Context::try_load_image` is public (`egui/src/context.rs:3861`).

### 9.3 The release-mode assert

`ColorImage::from_rgba_unmultiplied` carries an `assert_eq!` that fires in
**release** too (`epaint/src/image.rs:113-120`). `icons::to_color_image`
therefore returns `None` — never panics — when `width == 0`, `height == 0`, when
`width * height * 4` overflows `usize` (checked with `usize::checked_mul`), or
when the product does not equal `data.len()`. `from_rgba_unmultiplied` rather
than `..._premultiplied` is correct because native-theme raster payloads are
straight alpha (`native-theme/src/rasterize.rs:67-69`, `sficons.rs:110`,
`winicons.rs:171-173`, the Windows glyph path's white-plus-alpha pixels
`winicons.rs:106-113`, `:355`, and freedesktop PNGs, returned as the `png`
decoder's samples, `freedesktop.rs:254-257`, `:271-316`).

`Context::load_texture` carries a second `debug_assert!`, and it **cannot be
mitigated by clamping** — there is nothing to clamp. Its signature is
`load_texture(&self, name, image, options)` (`context.rs:2390-2395`): there is no
requested-size parameter. It reads `max_texture_side` out of the input state
itself (`:2398`) and asserts on the `ColorImage`'s **own** dimensions
(`:2399-2406`), which come from `IconData::Rgba`'s baked-in `width` and `height`
(`native-theme/src/model/icons.rs:303-310`). Any `IconData::Rgba` with a side
larger than `max_texture_side` — a 4096-pixel system icon against the `2048`
default a `Context` keeps until the integration raises it
(`input_state/mod.rs:268`, `input_state/mod.rs:349`) is the obvious case — therefore aborts every
downstream `cargo test` and `cargo run` in debug.

The mitigation is therefore an **admissibility test, not a resize**.
`icons::to_image_source` and `icons::to_image` read
`let max = ctx.input(|i| i.max_texture_side);` and return `None` when
`width > max` or `height > max`, exactly as `to_color_image` already returns
`None` for a zero or mismatched buffer — the oversize case joins that list.
**Nothing is downscaled.** Resampling an icon would fabricate pixel data that no
platform source supplies, which this crate forbids; a caller that wants a smaller
icon asks the icon layer for a smaller one. No `Note` variant is added either:
`Note` is compiled into the atlas (§4.3) and surfaces only through
`ThemeAtlas::notes()` (§4.2), which these free functions cannot reach (§7.2).

`load_texture` is documented as *not* immediate-mode safe (`context.rs:2360-2361`), which
is exactly why the URI, not the call site, is the cache key.

### 9.4 Sizes and animation

egui has **no icon-size vocabulary**. The three icon-shaped `Spacing` fields —
`icon_width` (`style.rs:428`), `icon_width_inner` (`:432`), `icon_spacing`
(`:436`) — are *control* geometry: the checkbox box, the check mark, and the
default gap between **all** atoms in **every** `AtomLayout`
(`atomics/atom_layout.rs:302`). Writing an icon size into any of them would
resize every checkbox and radio button from an icon metric. The five
`ResolvedIconSizes` values are therefore exposed through
`icons::icon_size(theme, IconContext)` and fed to
`Image::fit_to_exact_size(Vec2::splat(..))` (`widgets/image.rs:177`) at the call
site. **Do not pre-multiply by `pixels_per_point`.**

Which `IconContext` belongs on which widget is a design opinion, not a source
fact — egui has no convention to appeal to — so the crate exposes the five sizes
and takes no position.

One egui behaviour worth naming: `Button::opt_image_and_text`
(`widgets/button.rs:105`) — and therefore `Button::image` (`:89`) and
`Button::image_and_text` (`:97`) — sets `limit_image_size = true` (`:113`), so an
image atom added through those constructors is clamped to the font height at
`:311-313` whatever the theme asks for. `Button::new` leaves `limit_image_size`
at `false` (`:59`) and does **not** clamp, which upstream's own doc comment at
`:103-104` states explicitly.

`animated_frame_index` schedules exactly **one** wake-up at the next frame
boundary with `Context::request_repaint_after` (`context.rs:1872`) — the same
shape as egui's own `animated_image_frame_index`
(`widgets/image.rs:910-933`) — and is time-derived, so it stays correct after a
dropped frame or a window un-minimise. `spin_angle` has no natural frame
boundary and therefore calls `Context::request_repaint` (`context.rs:1821`)
every frame. Both return `None` and schedule nothing when the caller passes
`reduced_motion` as `true` (§4.10), `animated_frame_index` also when the icon is
not `AnimatedIcon::Frames` and `spin_angle` when it is not `AnimatedIcon::Transform`
with `TransformAnimation::Spin`; the caller then draws `AnimatedIcon::first_frame()`, which is infallible
(`native-theme/src/model/animated.rs:306-311`).

All animation arithmetic is on `u128` with a `NonZeroU32` divisor
(`native-theme/src/model/animated.rs:145`) over a non-empty frame list (`:54-59`):
no division by zero, no overflow — spelled with `checked_*` / `saturating_*`, as
the strict-panic set requires (§4.1).

---

## 10 -- Runtime theme change and repaint

### 10.1 Why egui is different from the two sibling connectors

The iced and gpui showcases poll a flag on a 500 ms timer because their runtimes
offer no cross-thread wake-up. egui does. `egui::Context` is
`Clone + Send + Sync + 'static` (`context.rs:722-723`, with upstream's own
compile-time assertion at `:4269-4272`), and `Context::request_repaint`
(`:1821`) documents that a call from outside the UI thread wakes it, provided
the integration installed a repaint callback — which eframe does on all three
backends (`eframe/src/native/glow_integration.rs:303`,
`eframe/src/native/wgpu_integration.rs:281`, `eframe/src/web/app_runner.rs:146`).
**There is no timer and no interval to tune.**

### 10.2 The threading contract

The watcher thread does the **whole** job, through the closure the
application hands `ThemeWatcher::start(ctx, rebuild)` (§4.11), where `rebuild`
is `impl Fn() -> native_theme::Result<ThemeAtlas> + Send + 'static` — the
bound `native_theme::watch::on_theme_change` asks of its callback
(`native-theme/src/watch/mod.rs:217-218`): on
each OS change it calls `rebuild()` — re-detection, re-resolution and
`ThemeAtlas` construction — stores the atlas for `take()`, then calls
`Context::request_repaint`. A closure that returns `Err` publishes nothing;
its message is what `last_error()` reports. This keeps D-Bus, registry and
`CFRunLoop` work, and the font-file reads of `system-fonts`, off the UI
thread. It is sound because atlas construction needs no `Context` and
`ThemeAtlas` is `Send + Sync + 'static`.

The closure is the application's because only the application knows which
theme it built. One whose atlas is `from_system()`'s passes
`ThemeWatcher::system_rebuild` (§4.11), which first calls
`native_theme::detect::invalidate_caches()`
(`native-theme/src/detect.rs:155`) — the process-wide caches behind
`system_is_dark`, `prefers_reduced_motion` and `system_icon_theme`
(`native-theme/src/detect.rs:145-148`) would otherwise answer with the values
from before the change — and then builds `from_system()`'s atlas afresh: the
accessibility preferences, the OS mode, the layout and, with `system-fonts`,
`FontPlan::from_system`'s plan from the new `SystemTheme`, whose font family
may be the one that changed. One that gave the first atlas builder inputs of
its own (`style_patch`, its own font plan), or runs on a preset or on
`SystemTheme::with_overlay` (`native-theme/src/lib.rs:538`), passes its own
closure, which sets every such input again and calls the same
`invalidate_caches()` before it reads anything from the OS.

**What fires the watcher, per platform.** `ThemeWatcher::start` wraps
`native_theme::watch::on_theme_change` (`native-theme/src/watch/mod.rs:217`),
whose backends observe exactly this, and nothing else:

| Platform | Fires on | Source | Not observed, though the reader reads it |
|---|---|---|---|
| KDE Plasma (`kde`) | a create, modify or remove of `kdeglobals` or `kcmfontsrc` in the directory holding `kdeglobals`; after a fire, relevant events are dropped for 300 ms (a leading-edge throttle, not a trailing debounce) | `native-theme/src/watch/kde.rs:20-32`, `:59-60`, `:73-89` | settings held in any other file |
| GNOME, Budgie (`portal`) | a portal `SettingChanged` in the namespace `org.freedesktop.appearance` | `native-theme/src/watch/gnome.rs:45-47`, `native-theme/src/watch/mod.rs:230-231` | `font-name`, `monospace-font-name`, `text-scaling-factor` and `icon-theme` from `org.gnome.desktop.interface`, and `titlebar-font` from `org.gnome.desktop.wm.preferences` (`native-theme/src/gnome/mod.rs:337-341`, `:371`), and `overlay-scrolling` from `org.gnome.desktop.interface` (`:364`) |
| any other Linux desktop | nothing: `start` returns `Error::WatchUnavailable` | `native-theme/src/watch/mod.rs:234-236` | — |
| macOS (`macos`) | `AppleInterfaceThemeChangedNotification` | `native-theme/src/watch/macos.rs:88-89` | the accent colour (`native-theme/src/macos.rs:57`) and the reduce-motion, increase-contrast and reduce-transparency flags (`native-theme/src/macos.rs:151-155`), wherever their change posts no such notification |
| Windows (`windows`) | `UISettings::ColorValuesChanged`; no other event is subscribed | `native-theme/src/watch/windows.rs:76-77` | the text-scaling factor (`native-theme/src/windows.rs:359`), whose `UISettings::TextScaleFactorChanged` is not subscribed; the non-client fonts, high contrast and client-area animation flag (`native-theme/src/windows.rs:154`, `:369`, `:387`) unless their change raises `ColorValuesChanged`, which is unverified |

The gaps are native-theme's, not this crate's, and are filed in
`docs/todo.md`. One more defect lies in the same module and
matters to every egui application, because an application drops its watcher
when it shuts down, if not before: `ThemeSubscription`'s `Drop` joins
the watcher thread (`native-theme/src/watch/mod.rs:183-185`), and the GNOME
thread waits inside the blocking signal iterator (`native-theme/src/watch/gnome.rs:52-62`)
with no platform wake-up registered (`:65`), so on GNOME the drop returns only
when the next `org.freedesktop.appearance` signal arrives. It is fixed at the
root, in native-theme's GNOME backend, before this crate's watcher is built (the
implementation plan orders it so) — never by detaching the thread in this crate.

**Installation stays on the UI thread.** `Context::set_style_of` takes `&self`
and would compile from the watcher thread, but each `Ui` snapshots its
`Arc<Style>` exactly once (`ui.rs:136`, `:237`) and never re-reads it, so a
mid-pass swap can leave two halves of one frame in two different themes.
`ThemeWatcher::take()` is the hand-off: non-blocking, `None` on the
overwhelming majority of frames.

### 10.3 The install sequence, and what it must not touch

`ThemeAtlas::install(ctx)` is the one install call, and it takes no options.
This list is the one statement of what it does; §4.2 points here. In this
order:

1. **Fonts.** `ctx.set_fonts(..)` (`context.rs:2106`) whenever the atlas was
   built with a font plan (`Builder::fonts`, §4.9), even one in which no face was
   found, so a family the plan has no face for is egui's own again, not the
   previous install's; an atlas built with no plan maps font sizes only and leaves
   the `Context`'s fonts alone.
   An application with fonts of its own builds through `ThemeAtlas::builder(..)`
   with `.fonts(FontPlan::from_system(&light).with_base(its_defs))`;
   `from_preset`, `from_system` and `SystemThemeExt::to_egui_atlas` use egui's
   default base. The argument is the `FontDefinitions` `Builder::build`
   computed with `fonts::font_definitions(t, plan)` and kept, starting
   from the plan's base (`FontPlan::with_base`, §4.9), `t` the atlas's light
   `ResolvedTheme`, the variant `FontPlan::from_system` reads (§8.2): egui
   holds one `FontDefinitions` per `Context`, not one per `egui::Theme`. The
   base defaults to `FontDefinitions::default()`, never `FontDefinitions::empty()`
   (`epaint/src/text/fonts.rs:561`), because `default()` carries the emoji
   fallback tails (`epaint/src/text/fonts.rs:534-550`); `set_fonts`, not
   `add_font`, which de-duplicates by name only. `font_definitions` has already
   dropped every face epaint could not parse (§8), so this step cannot reach
   epaint's release-mode parse panic (`epaint/src/text/fonts.rs:990`).
2. **Both base styles.** `ctx.set_style_of(Theme::Dark, ..)` and
   `set_style_of(Theme::Light, ..)` (`context.rs:2250`), each base style first
   given every `TextStyle::Name` key of the style it replaces,
   `ctx.style_of(theme)` (`context.rs:2221`), that it lacks, so an
   application's own named text styles survive the install (§7.5, §14 item
   30). A stock key is never copied — the atlas's value wins — and the atlas's
   own `Arc` is published unchanged when the old style holds no `Name` key.
   The styles already carry the theme's line spacing, computed at build
   (§6.15).
3. **The atlas published** into `ctx.data_mut()` (`context.rs:1033`), where
   `ThemeAtlas::from_ctx` reads it back, `NativeThemeUiExt` and the plugin find
   it, and `ThemeAtlas::clear(ctx)` removes it.
4. **The install plugin registered** with `ctx.add_plugin`
   (`context.rs:2047`). It holds no theme data of its own: every pass it reads
   what step 3 published, so a re-install with another atlas takes effect with
   no second registration — egui keeps the first plugin of a type and ignores
   the later ones (`context.rs:2045`, `egui/src/plugin.rs:206-210`) — and after
   `ThemeAtlas::clear` it finds nothing and does nothing. Its three hooks are
   described below.
5. `icons::forget_icons(ctx)`, which releases the previous theme's icons: a
   recoloured icon has new bytes and so a URI of its own (§9.2), and egui drops
   a URI's only texture and its bytes only when told to
   (`egui/src/load/texture_loader.rs:125-134`, `egui/src/load/bytes_loader.rs:15-26`),
   so they would otherwise stay for the life of the `Context`. It uses
   `Context::forget_image` (`context.rs:3764`) per URI and removes the stored
   `TextureHandle`s (§4.10), leaving unrelated application images alone.
6. `ctx.request_repaint()` (`context.rs:1821`), last, because the install reaches the
   screen only in a later pass: a `Ui` that already exists keeps the style it
   inherited for the rest of the pass (`ui.rs:237`), and new fonts "become
   active at the start of the next pass" (`context.rs:2104`). Without it an
   install made in response to something that requests no repaint — a
   watcher hand-off read in `logic`, a setting changed from outside — would
   wait for the next input event.

`install` is safe to call at any time, including inside a pass, and it can
never cause the unbound-family panic at `epaint/src/text/fonts.rs:1025` for a
family this crate names, because it never emits a `FontFamily::Name` (§8.1); an
application's own fonts survive an install only through `FontPlan::with_base`
(§4.9); an `add_font` still pending at the next pass is applied on top of them
(`egui/src/context.rs:556-574`). Called inside a pass, it reaches
only the `Ui`s built from the global style afterwards — the `Area`-based
containers (`ui.rs:136`) — and step 6's repaint switches the whole UI on the
next pass.

Three constraints are load-bearing:

* **Both themes, always.** `set_style_of(Theme::Dark, ..)` **and**
  `set_style_of(Theme::Light, ..)` (`context.rs:2250`). Never `set_visuals`
  (`:2280`) or `set_global_style` (`:2200`) — they touch only the *active*
  theme, leaving the other stock. Never `set_visuals_of` (`:2267`) — it writes
  only `visuals` (`:2268`), so the atlas's `spacing` and `text_styles` would
  never reach the context.
* **On every application start.** `Options::dark_style` and `light_style` are
  `#[serde(skip)]` (`memory/mod.rs:195`, `:199`), so a persisted `Memory` never
  restores them.
* **Never `Options::theme_preference`.** That field **is** serde-persisted
  (`memory/mod.rs:206`, no `serde(skip)`), so an install that set it would
  discard the user's in-app Light/Dark choice on every launch. The application
  sets it with egui's own `Context::set_theme` (`context.rs:2170`):
  `ThemePreference::System` follows the OS, `Theme::Light` or `Theme::Dark`
  pins one. Note the trap: `set_theme` takes `impl Into<ThemePreference>` and
  `From<Theme> for ThemePreference` exists (`memory/theme.rs:79-86`), so
  `ctx.set_theme(Theme::Dark)` *pins* dark; following the OS is
  `ctx.set_theme(ThemePreference::System)`. `Options::fallback_theme` and
  `Options::sync_window_theme` are egui's too: the install never writes them,
  and an application that wants other values sets them with
  `ctx.options_mut(..)` (`context.rs:1135`).

**The OS colour scheme on Linux, and the title bar that follows it.** Under
`ThemePreference::System` egui picks the active `Theme` from
`RawInput::system_theme`, falling back to `Options::fallback_theme` (default
`Theme::Dark`) when the integration reports none
(`egui/src/memory/mod.rs:365-367`, `:331`). eframe fills that field from
winit's `ActiveEventLoop::system_theme()`
(`eframe/src/native/wgpu_integration.rs:303`, `eframe/src/native/glow_integration.rs:1317`,
into `egui-winit` 0.36.2 src/lib.rs line 188), and winit 0.30.13 answers `None`
on Linux, X11 and Wayland alike (src/platform_impl/linux/mod.rs lines 909-911,
read in the local cargo registry), and nothing under its
src/platform_impl/linux emits `WindowEvent::ThemeChanged`, the one event on
which `egui-winit` updates the field (src/lib.rs line 451). So on every Linux desktop egui runs dark whatever the OS says. The
plugin's input and output hooks therefore do two things, and on macOS and Windows, where the
integration reports the OS scheme, neither changes anything:

* **The active theme — `input_hook`** (`egui/src/plugin.rs:38`). It sets
  `input.system_theme = Some(os_mode)` — the atlas's `os_mode()`, published by
  step 3 — **only when the field is `None`**. The hook runs before
  `Options::begin_pass` copies the field (`egui/src/context.rs:966-968`,
  `egui/src/memory/mod.rs:358-359`), so the pass that follows uses it. A
  preset-built atlas has no OS mode and fills nothing: an application that
  builds one and still wants it to follow the OS gives the builder
  `Builder::os_mode(ColorMode)`, from `native_theme::detect::system_is_dark()`
  (`native-theme/src/detect.rs:141`). `Options::fallback_theme` is not the
  route: it is serde-persisted like `theme_preference`
  (`egui/src/memory/mod.rs:212`) and, like the hook's value, would change only
  when an atlas is installed again.
* **The title bar — `output_hook`** (`egui/src/plugin.rs:44`). egui keeps the window's theme in step with the
  preference by itself (`Options::sync_window_theme`, default `true`,
  `egui/src/memory/mod.rs:229`, `:333`): at the end of a pass whose preference
  differs from the one last sent, `Context::sync_window_theme` sends
  `ViewportCommand::SetTheme`, and under `ThemePreference::System` it sends
  `SystemTheme::SystemDefault` (`egui/src/context.rs:2474-2497`, called at
  `:2456`). `egui-winit` turns `SystemDefault` into `Window::set_theme(None)`
  (src/lib.rs lines 1911-1915), which winit 0.30.13 handles per backend:

  | Backend | `set_theme(None)` | `set_theme(Some(mode))` |
  |---|---|---|
  | X11 | writes `_GTK_THEME_VARIANT = "dark"` on the window whatever the OS scheme (src/platform_impl/linux/x11/window.rs lines 631-649, the `None` arm at 638), so a window manager that honours the hint draws a dark title bar on a light desktop | writes `"light"` or `"dark"` (lines 636-637) |
  | Wayland, client-side decorations (a compositor that offers none, e.g. Mutter) | reconfigures winit's Adwaita frame with `FrameConfig::auto()` (src/platform_impl/linux/wayland/window/state.rs lines 846-851, 1214-1219), which runs `dbus-send` once for the portal's `color-scheme` with a 100 ms reply timeout and draws dark only for the value `1` (sctk-adwaita 0.10.1, src/config.rs lines 5-24; src/theme.rs lines 24-29) — read at that moment and never again | `FrameConfig::light()` or `dark()` (state.rs lines 1216-1217) |
  | Wayland, server-side decorations (KWin) | nothing visible: winit drops its frame when the compositor decorates (state.rs lines 305-308) and the compositor draws its own title bar | the same |
  | macOS, Windows | the window follows the OS | — (the connector sends none) |

  So the output hook, which runs
  after `sync_window_theme` (`egui/src/context.rs:2456`, `:2464`), does this in
  a pass whose `input_hook` filled `system_theme`, while
  `Options::sync_window_theme` is `true` and the preference is
  `ThemePreference::System`: it rewrites every
  `ViewportCommand::SetTheme(SystemTheme::SystemDefault)` in the pass's
  `FullOutput::viewport_output` (`egui/src/data/output.rs:39`,
  `ViewportOutput::commands`, `egui/src/viewport.rs:1270`) into
  `SetTheme(os_mode)`, and when the atlas's `os_mode` differs from the one it
  last sent for that viewport — a watcher rebuild after the user switched the
  desktop's scheme — it appends `SetTheme(os_mode)` itself, because egui sends
  again only when the *preference* changes (`egui/src/context.rs:2486-2493`). Under a pinned
  preference, with `sync_window_theme` set to `false`, or where the
  integration reported a scheme, the hook changes nothing. The title bar then
  matches the scheme the whole UI is drawn in, on X11 and on client-side
  decorations, and follows it live; the rewrite rather than a command sent
  from `on_end_pass` is what makes it robust, because `sync_window_theme` runs
  after the plugins' end-pass hooks (`egui/src/context.rs:811-813`) and a
  `SystemDefault` it sent later in the same pass would win.

**The focus ring — `on_end_pass`** (`egui/src/plugin.rs:32`). It paints the
ring of §6.18 around the widget holding keyboard focus
(`Memory::focused`, `egui/src/memory/mod.rs:893`), and only while the window
has the OS keyboard focus — `ctx.input(|i| i.focused)`
(`egui/src/input_state/mod.rs:315`), the test egui's own `Response::has_focus`
makes (`egui/src/response.rs:350`). A button takes focus from the keyboard
only, so a click shows no ring and `Tab` does. The ring's corners are the
active corner radius of the innermost role scope around the widget:
`native_scope` and `native_set_style` record their `Ui` with that radius for
the pass, and the plugin takes the innermost recorded `Ui` whose final rect
contains the focused widget's (§6.18) — where none does, the radius of the
root `Ui`'s own style at the end of the pass, the base style's after §4.4's
`Ui::reset_style`. `register_focus_shape` remains for an
outline that is not the widget's own rect, such as a switch's track; an
application that draws its own outline registers it there. The ring is always
on: it paints nothing until a widget has keyboard focus in a focused window.

**Where the hooks run.** egui calls a plugin's begin- and end-pass hooks only
from `Context::run_ui` (`egui/src/context.rs:794`, `:811-813`), which eframe
and egui_kittest drive (`eframe/src/native/epi_integration.rs:288`,
`eframe/src/web/app_runner.rs:284`, `egui_kittest/src/lib.rs:162`, `:285`). An
integration that calls `Context::begin_pass` and `end_pass` itself
(`egui/src/context.rs:962`, `:2443`) reaches only the input and output hooks
(`:966`, `:2464`): it gets the OS scheme and the title bar, and no focus ring.

One residue, glow only: eframe's glow backend picks the clear colour before
the pass, from an `Options::begin_pass` over the raw input the hook has not
yet seen (`eframe/src/native/glow_integration.rs:687-695`), so a glow
application on a light Linux desktop clears to the dark base style's
`panel_fill` wherever nothing paints. eframe's default backend, wgpu, asks for
the clear colour after the pass (`eframe/src/native/wgpu_integration.rs:814`).

**Lock discipline: no `Context` accessor inside a `Context` accessor.**
`egui::Context` is one `Arc<RwLock<ContextImpl>>` (`context.rs:723`) behind
`read` (`:759`) and `write` (`:764`), and the "read-only"-looking accessors are
not all read locks — `Context::input` takes the **write** lock (`:991`), as does
`data_mut` (`:1033`). epaint's `RwLock` is not reentrant and, in debug builds,
panics after ten seconds rather than blocking forever
(`epaint/src/mutex.rs:5`, `:98-106`). No function in this crate may therefore
call a second `Context` accessor from inside the closure of a first one.

This is not hypothetical: it is exactly the shape the icon texture cache invites.
`Context::load_texture` calls `self.input(..)` internally (`context.rs:2398`), so
the natural cache-then-upload spelling —
`ctx.data_mut(|d| d.get_temp_mut_or_insert_with(key, || ctx.load_texture(..)))`
(`egui/src/util/id_type_map.rs:515`) — deadlocks by construction. §9.1's cache
must instead read in one `ctx.data_mut`, **drop the guard**, call `load_texture`
outside any accessor closure, and insert in a second `ctx.data_mut`. No test is
specified for this: a test would hang and then panic by construction, which is
not a useful signal.

### 10.4 The worked example's spelling

**Scope (rationale §8, Q-6).** The showcase exists to show, and to let
a reader check, how close to the platform an egui application looks when this
crate installs the platform's theme. It is therefore the gpui showcase's
application built from egui's own containers, not a gallery: the same chrome
(menu bar, toolbar, a resizable side panel holding the theme settings and the
inspector, page tabs, status bar, command palette, Preferences and About),
per-instance **Widget Info** for every widget on screen, and a palette of
every widget egui 0.36.2 and egui_extras 0.36.2 offer an application. It
carries the C7 self-tests (§13 T11, the chrome and Widget Info ones in §13.2),
is a screenshot source of the release pipeline, is kept complete by the
widget-coverage script, and is built with no connector feature and with all
of them (§11; the implementation plan wires each gate) —
`scripts/check-features.sh` checks each feature alone only for the library
(`cargo check -p … --lib`, `scripts/check-features.sh:54`).
With it, egui is the second showcase with per-instance info; the iced
showcase's own entry stays in `docs/todo.md`. The layout is the gpui
showcase's as the maintainer last shaped it
(`docs/archive/todo_v0.5.9_showcase-layout.md` S1–S4 and S8, amending
`docs/archive/todo_v0.5.9_showcase-app-spec.md` §1–§2); where egui has no
counterpart of a gpui-component widget, the egui spelling is named below.

**The window: the OS draws the frame.** The gpui showcase asks the window
manager for server-side decorations and draws a title bar only where a
compositor refuses (showcase-layout S8). eframe behaves that way on its own:
`NativeOptions::viewport` (`eframe/src/epi.rs:303`) is built with
`ViewportBuilder::with_decorations(true)` (`egui/src/viewport.rs:366`) —
written out rather than left to winit's default, so a test can read it back
from the public `decorations` field (`:307`) — and with a fixed title,
`with_title(WINDOW_TITLE)` (`:355`), where
`WINDOW_TITLE = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"), " showcase")`:
the crate's name and version, as the gpui showcase's `WINDOW_TITLE`
(`connectors/native-theme-gpui/examples/showcase-gpui/main.rs:222-227`); the
preset and mode are in the status bar. eframe 0.36.2 draws through
winit 0.30.13 (`eframe/Cargo.toml:266-267`), whose Wayland window asks the
compositor for server-side decorations whenever decorations are on (winit
0.30.13, src/platform_impl/linux/wayland/window/mod.rs lines 98-107, read in
the local cargo registry), so KWin draws its own frame; where the compositor
answers with client-side decorations — Mutter offers no xdg-decoration
manager (`docs/archive/todo_v0.5.9_showcase-layout.md`, "As built", S8) —
winit draws an Adwaita-styled frame (winit 0.30.13,
src/platform_impl/linux/wayland/window/state.rs lines 47-48 and 278-304)
through its `wayland-csd-adwaita` feature, a winit default (winit 0.30.13
Cargo.toml lines 73-79) that eframe's default `winit/default` keeps on
(`eframe/Cargo.toml:67`). X11, Windows and macOS give the platform's own
frame. The showcase therefore draws **no title bar in any case**: egui has no
title-bar widget, and one painted by the showcase would imitate the platform
instead of being it. On Linux and Windows the menus sit in a menu-bar row at
the top of the window, as KDE and Windows applications place them. **On macOS
they sit in the system menu bar**, as the gpui showcase's do
(`docs/archive/todo_v0.5.9_showcase-layout.md` S8). eframe 0.36.2 has no menu
API of its own (a grep of its `src/` for `NSMenu`, `menubar` and `set_menu`
finds nothing), so the showcase goes through the two seams that exist.
Its `NativeOptions::event_loop_builder` (`eframe/src/epi.rs:351`) calls winit's
`EventLoopBuilderExtMacOS::with_default_menu(false)` (winit 0.30.13,
src/platform/macos.rs lines 429 and 446-449); without it winit installs its
own default application menu when the application finishes launching
(src/platform_impl/macos/app_state.rs lines 139-142, the menu at
src/platform_impl/macos/menu.rs lines 12-85). The app-creation closure, which
eframe runs on the main thread once the event loop exists, builds File, View,
Theme and Help with muda 0.20.0 and installs them with its safe
`Menu::init_for_nsapp` (muda src/items/menu.rs lines 548-552, which calls
`NSApplication::setMainMenu`, src/platform_impl/macos/mod.rs lines 164-168).
A muda `Menu` holds `Rc`s (src/items/menu.rs lines 35-39) and is not `Send`,
so the app struct owns it, which `eframe::App` allows: the trait has no `Send`
bound (`eframe/src/epi.rs:152`). `MenuEvent::set_event_handler` (muda
src/menu_event.rs line 47) sends each clicked item's id down a channel and
calls `ctx.request_repaint()`; `logic` maps the id to the action the
in-window item of the same name runs, so both menus share one set of actions.
Each muda item carries its shortcut as its accelerator, so the menu shows it
in the platform's spelling, and on macOS the egui-side consumption of those
shortcuts is compiled out, so one route owns each key. Under `cfg(test)` the
in-window `MenuBar` is used on every platform: `build_eframe` takes no
`NativeOptions` (§13 T11) and a harness has no system menu bar, so the menus'
actions are tested through it (§13.2), and the muda path is the binary's
alone. **UNVERIFIED** until the macOS runner's capture step runs (§13's runner
checks, §15): that the menu is installed. **UNVERIFIED** until run on a Mac: that AppKit delivers an accelerator to
the menu before winit's view sees the key; *what would verify it*: pressing
Cmd+B once and seeing the side panel toggle exactly once. And `eframe::App::clear_color` defaults to a
hardcoded translucent dark grey (`eframe/src/epi.rs:248-253`) that shows
wherever no panel paints, so the showcase returns the `panel_fill` of the
`Visuals` it is handed — the active base style's
(`eframe/src/native/wgpu_integration.rs:814`) — through
`Color32::to_normalized_gamma_f32`, as the method's documentation asks
(`eframe/src/epi.rs:242-247`); that field carries `defaults.background_color`
(§5.1).

**The chrome, outside in** (egui lays out the outer panels before the central
one). Each element is drawn through the seam it names, so the chrome itself
demonstrates the connector, and each records its Widget Info:

| Element | egui spelling | Seam |
|---|---|---|
| Chrome bar: the menu-bar row above the toolbar row | one `Panel::top` (`containers/panel.rs:265`), shown on the root `Ui` while `Role::Toolbar` is live on it (`ui.native_set_style(Role::Toolbar, RoleVariant::Normal)`, §4.4's recipe), so its separator line and its body take the toolbar's style, and set again as the first statement inside its closure, so the focus ring there takes the toolbar's radius (§6.18); after the three outer panels the root takes the base style again with `Ui::reset_style` (`ui.rs:392`) before the `CentralPanel` | `Surface::Panel(PanelSide::Top)`, fed from `theme.toolbar` (§4.4); `Role::Toolbar`. native-theme models no menu-bar surface, so the menu row sits on the toolbar's, as the gpui menu-bar row takes no fill of its own |
| Menu bar | on Linux and Windows, and on macOS under `cfg(test)` (the system menu bar otherwise, above): `MenuBar::new()` (`containers/menu.rs:232`) with `style` (`:241`) and `config(MenuConfig::new().style(..))` (`:250`, `:107`); File, View, Theme and Help built with `Ui::menu_button` (`ui.rs:2788`); View lists the pages, as the gpui showcase's View menu does (`SubMenuButton` is the Overlays page's) | `Role::Menu` through `ThemeAtlas::role_modifier`, handed to both calls: `MenuBar::style` styles the bar's buttons and `MenuConfig::style` the menus, and without them `menu_style` overwrites padding and strokes (§4.2). Each open menu's own `Ui` (`Ui::response`, `egui/src/ui.rs:944`) is also recorded, as `Role::Menu`: its frame is the `Frame::popup` or `Frame::menu` egui builds from that role's style (`containers/popup.rs:603`, `containers/menu.rs:432`), which carries the menu's chrome (§4.4), and `Ui::menu_button` returns no response for the popup (`ui.rs:2798`) |
| Toolbar | a row in the chrome bar: icon buttons for the command palette, a theme reload and Preferences, each with a tooltip, their icons of the chosen icon theme at `toolbar.icon_size`, each bundled Material or Lucide key tinted `ui.visuals().text_color()` and a system icon theme's icon loaded in the text colour (§9.2), as on the Icons page and in the gpui showcase (`connectors/native-theme-gpui/examples/showcase-gpui/support.rs:366`), the row at least `toolbar.bar_height` tall where the theme states one, both read from the `ResolvedTheme` | `Role::Toolbar`, with `Role::Button` for its buttons and, for their tooltips, `Surface::Tooltip` and `Role::Tooltip` through `Tooltip::for_enabled` (`containers/tooltip.rs:53`), the hover-gated constructor `Response::on_hover_ui` uses (`response.rs:665-666`), its public `popup` given the frame and the style — `Response::on_hover_text` takes neither (§14 item 3) |
| Status bar | `Panel::bottom` (`containers/panel.rs:274`), shown while `Role::StatusBar` is live on the root `Ui` and set again inside its closure, as the chrome bar is: the side-panel toggle, then the environment text — detected desktop, preset and mode, `defaults.font` in its defined unit (§8.7), the text-scaling factor, the accessibility flags that are set — then the title of the shown Widget Info | `Surface::Panel(PanelSide::Bottom)`, fed from `theme.status_bar`; `Role::StatusBar` for its separator line and its body |
| Side panel | `Panel::left` (`:249`) `.resizable(true)` (`:322`) `.default_size(LEFT_PANEL_WIDTH)` (`:369`), shown with `show_collapsible` (`:451`) while `Role::Splitter` is live on the root `Ui`, which the status-bar toggle, View > Toggle Side Panel and Ctrl+B flip; the dragged width survives. Not inside a `native_scope`: a panel moves the cursor of the `Ui` it is shown in (`containers/panel.rs:852-865`), and a scope then advances its parent past the scope's whole rect (`ui.rs:2213`), which for a left or bottom panel leaves the central panel no room | `Surface::Panel(PanelSide::Left)`, fed from `theme.sidebar`; `Role::Splitter` for its separator line and its hover and drag line (`containers/panel.rs:905-911`); its body in `Role::Sidebar`, set with `ui.native_set_style(Role::Sidebar, RoleVariant::Normal)` as the first statement inside the closure (§4.4) |
| … its content | the rows **Theme** (a `ComboBox` of `default` and `Theme::list_presets_for_platform()`, `native-theme/src/model/mod.rs:674`), **Mode** (System, Light and Dark as a segmented control of `Button::new(..).selected(..)`, as the Buttons page draws one; System is `ctx.set_theme(ThemePreference::System)`, the other two `ctx.set_theme(Theme::Light)` and `ctx.set_theme(Theme::Dark)`, egui's own calls, §10.3) and **Icon theme** (a `ComboBox`); a `Separator`; the inspector's tabs **Widget** and **Theme**, drawn as the page tabs are; the inspector's content in a `ScrollArea` | `Role::ComboBox` with its popup through `ComboBox::popup_style` (`containers/combo_box.rs:199`); `Role::SegmentedControl`; `Role::Separator`; `Role::Tab`; the base style for the `ScrollArea`, which carries the scrollbar's colours (§5.9) |
| Content | `CentralPanel` (`containers/panel.rs:1206`, `:1212`): the page tabs, a theme error when one occurred, then the page in a `ScrollArea` | `Surface::CentralPanel`, fed from `theme.defaults`; the tab row in one `ui.native_scope(Role::Tab, RoleVariant::Normal, ..)`, each tab a `Button::new(..).selected(..)` whose own flag picks the active tab's colours (§6.2) — not `Button::selectable`, whose unselected button paints no resting frame (`widgets/button.rs:81`, `:364-368`), so `tab.background_color` would not show; the error in a `Surface::Card` frame, its text in `visuals.error_fg_color`, which carries `defaults.danger_color` (§5.1) |
| Command palette | a `Modal` (`containers/modal.rs:77`), sized within `dialog.min_width`, `min_height`, `max_width` and `max_height`, read from the `ResolvedTheme` where the theme states them: a `TextEdit::singleline` given focus (`Response::request_focus`, `response.rs:377`), then `selectable_label` rows for every page, the presets the Theme row offers and the three modes. Opened by the toolbar, View > Command Palette and `KeyboardShortcut::new(Modifiers::COMMAND, Key::K)` (`data/input/keyboard_shortcut.rs:18`, `data/input/modifiers.rs:115`) consumed with `InputState::consume_shortcut` (`input_state/mod.rs:735`) in `logic`, before any widget reads input, so it is Ctrl+K, and Cmd+K on macOS, and it takes precedence over a focused `TextEdit`'s own Ctrl+K (`widgets/text_edit/builder.rs:1436`). Escape clears the query, then closes: while the query is not empty the showcase consumes Escape itself before it asks `ModalResponse::should_close` (`containers/modal.rs:151`), which would consume it too (`:155-156`) and also answers a click on the backdrop | `Surface::Dialog` (`Modal::frame`, `egui/src/containers/modal.rs:53`), its body in `Role::Dialog`; the rows in `Role::List` |
| Preferences | a `Window` holding the four `AccessibilityPreferences` fields; a change rebuilds the atlas with `Builder::accessibility` (§4.3) and installs it | `Surface::Window` and `Surface::WindowTitleBar` (`containers/window.rs:265`, `:272`), its body in `Role::Window`; its title a `RichText` in `window_title_bar_font` and `window_title_bar_text_color(t, true)` (§4.7, §14 item 2b) |
| About | a `Modal`: the crate's name and version, and a `Hyperlink` to the README's compatibility table; its buttons in `dialog_button_order` (§4.7) | `Surface::Dialog`, `Role::Dialog`, `Role::Link` |

The menus are File (Quit, Ctrl+Q, sending `ViewportCommand::Close`,
`egui/src/viewport.rs:1084`), View (the pages, Toggle Side Panel, Command
Palette), Theme (Reload System Theme; System, Light, Dark; Preferences…,
Ctrl+,) and Help (About), as in the gpui showcase (showcase-app spec §2.2); a
shortcut's label is `Button::shortcut_text` (`widgets/button.rs:225`) filled by
`Context::format_shortcut` (`context.rs:1734`), so it is the platform's
spelling of the key. `LEFT_PANEL_WIDTH`, `WINDOW_SIZE` and `INFO_SETTLE` are
the chrome's only numeric literals, each a named constant whose comment says the model
states no such value — native-theme has no side-panel width, no initial window
size and no hover delay. Every other margin, gap, bar height, icon size and
dialog size is the atlas's, and where the theme states none, egui's own value
stands.

**The atlas the showcase installs, and when it changes.** A preset's atlas is
`from_preset(name, is_dark, &prefs)` (§4.6), and `default`'s is the system
atlas described below, so with feature `system-fonts` every atlas the showcase
builds carries the font plan `FontPlan::from_system` returns for the theme's
light variant (§4.6, §4.9, §8), and text is drawn in the typeface the theme names wherever the system has
that family; a family it lacks is a `Note::FontFamilyUnavailable` and one
whose data epaint would reject a `Note::FontDataInvalid`, both listed in the
Theme tab, with egui's own face in their place. With feature `watch` the
showcase starts one `ThemeWatcher` (§10.2) at start-up — **not** under
`cfg(test)` and **not** under `--screenshot`, where an OS change during a test
or a capture would make its result depend on the desktop it ran on. Its
`rebuild` closure reads what the side panel and Preferences currently select
— `default` or a preset, the mode, the accessibility overrides — from an
`Arc<RwLock<..>>` the UI thread writes on each pick (a poisoned lock is read
through `PoisonError::into_inner`, never unwrapped), so one watcher serves
every selection and is never dropped and restarted, which on GNOME would
block (§10.2); for `default` it returns `ThemeWatcher::system_rebuild()`'s
atlas when Preferences overrides nothing, and otherwise does what that
function does — `invalidate_caches()` first — with `Builder::accessibility`
set to the overrides. `logic`
calls `take()` every pass and installs what it returns, and Theme > Reload
System Theme runs the same closure on the UI thread.

**Widget Info: registration.** A module `demo` holds one helper per palette
entry and `chrome` one per chrome element; pages and the chrome build widgets
only through them. A helper draws its widget and records, in the pass's
registry, the `Response`'s `id`, `interact_rect` and `layer_id` with an
`InstanceInfo`: the widget's kind and variant, the seams it was drawn with — a
`Role` with its `RoleVariant`, a `Surface`, or the base style — the §4.7
accessors and the `ResolvedTheme` leaves it read, and *This instance* notes formatted from the helper's own
arguments (a slider's range, a field's hint), never a claim about the theme.
The helper takes each seam once and both applies and records it, so no widget
is drawn with a seam its info does not name — the egui form of gpui's
`native_info` (showcase-app spec §5.2). The type is not called `WidgetInfo`:
egui exports a `WidgetInfo` of its own, for AccessKit
(`egui/src/data/output.rs:563`, re-exported at `egui/src/lib.rs:471`).

**Widget Info: innermost hovered wins.** At the end of `ui`, among the
registrations of the current pass whose `Response::contains_pointer()` is true
(`egui/src/response.rs:333`), the one with the smallest `interact_rect` area
wins, the rect first moved to global coordinates with
`Context::layer_transform_to_global` (`egui/src/context.rs:3078`) when its
layer has a transform, as a `Scene`'s has; a tie goes to the registration
recorded first. `contains_pointer` is set from the pass's interaction snapshot
(`egui/src/context.rs:1459-1462`), which is egui's hit test's
`contains_pointer` plus its click and drag hits
(`egui/src/interaction.rs:252-255`), and the hit test keeps only the layers the
pointer reaches — the top-most layer whose widget covers the search circle,
and any in front of it (`egui/src/hit_test.rs:120-129`), less every widget a
widget on another layer fully covers (`:145-156`), before it collects the
widgets that contain the pointer (`:160-164`) — so a widget under an open
popup, menu or modal never wins. Paint order is deliberately not the rule.
Every `Ui` enters egui's widget list when it is created (`egui/src/ui.rs:172`,
`:300`; `egui/src/widget_rect.rs:91-92`), but a `Frame` enters its own rect
after its contents: `Frame::show_dyn` ends with `Prepared::end`
(`egui/src/containers/frame.rs:416-418`), which allocates the frame's rect
(`:488`, `:466-467`). The last widget containing the pointer is therefore often
the container around the one the reader points at, and the tie rule exists for
the same reason: a container is recorded after its contents return, so one of
the same size never beats what it holds. Smallest area is the gpui showcase's
rule (showcase-app spec §4.2). Registrations live for one pass, so an instance
no longer drawn — the previous page after a switch, a closed popup — is never
a candidate, although egui's hit test, which runs on the previous pass's rects
(`egui/src/context.rs:485-495`), may still find it.

**Widget Info: showing.** A choice that differs from what is shown replaces it
only after it has stayed the choice for `INFO_SETTLE` (250 ms, the gpui
showcase's value, `connectors/native-theme-gpui/examples/showcase-gpui/info/registry.rs:16`,
measured on `InputState::time`), with
`Context::request_repaint_after` (`egui/src/context.rs:1872`) scheduling the
pass that completes it; leaving every target — the pointer outside the
window, since the chrome and the pages record everything inside it — keeps
what is shown. The side
panel and the status bar are drawn before the central panel, so they show the
previous pass's choice, and a changed choice requests a repaint. The
inspector's content — its rows, swatches and Copy button below the tabs —
records nothing and is a hold zone: while the pointer is inside its rect no
registration is a candidate and the choice stays what it was, so moving the
pointer into it to read or copy leaves the info in place, although the side
panel's own record contains the pointer there; its tabs are chrome and
record. Copy puts the shown info on the clipboard (`Context::copy_text`, `egui/src/context.rs:1686`). Before any hover
the Widget tab reads "Hover any widget to see what the theme sets on it."

**Widget Info: what it says, generated from `mapping.toml`.** The showcase
embeds `connectors/native-theme-egui/mapping.toml` with `include_str!` and
parses it with the `toml` dev-dependency (§11); a parse failure is shown in
the inspector, never a panic. A value is read from `serde_json::to_value` of
the theme `atlas.resolved_for(theme)` returns, at the row's leaf path, walked
as T3 walks it (§13.1); a serialisation error is shown the same way. For each seam of the instance, the Widget tab
lists the manifest's rows:

* for a `Role` R, every row whose leaf starts with `R.key()` and a dot, and
  every other row with a sink carrying `scope = R.key()`;
* for a `Surface` S, every row with a sink carrying `surface = S.key()`, and
  every row of each native widget those rows' leaves belong to — `sidebar`
  for `panel_left`, §4.4's convention, read from the rows rather than written
  a second time;
* for the base style, every row with a sink that carries neither `scope` nor
  `surface` — the base-owner table (§5.9) — and every row under `defaults.`,
  `text_scale.` or `layout.` that carries no sink, `unmappable` or checked
  through `tested_by` (§13.1), which no `Role` key begins, so that what the
  whole application loses, or reaches only per call, is shown with the base
  style.

A `Role` seam lists the base style's rows too: a role cell carries every value
of the base style its role does not write (§3.4).

Each row prints the native leaf; its value under the installed preset and mode
(a colour as hex beside a swatch, a `None` as "not stated — egui's own value
stands"); the verdict; and the egui sinks it writes, with their scope or
surface — for a `tested_by` row, that no `Style` or `Frame` field carries it,
and the test that checks its route. A row whose `exceptions` name the
installed preset prints that reason. An `unmappable` row prints **lost here**, its `sub_tag` and its
`upstream` line: what the platform states that this egui application cannot
show, and the egui change that would show it — the honest half of native
fidelity. Under the rows come the §4.7 accessors and `ResolvedTheme` leaves
the helper read, with their values, then *This instance*. The **Theme** tab shows what no hover target
carries: the atlas's `name()`, `os_mode()` and `accessibility()`, every `Note`
in `notes()`, the fonts in their defined units, the window-frame facts above,
and the manifest's `[unwritten]` table — each egui field this crate leaves at
egui's value, with the reason. Every claim an info makes is therefore a
manifest row that T2 and T3 already hold and T10 holds in all 32 combinations
(§13), so
the info needs no citation gate of its own; what is left to check is which
seams an instance is drawn with, and §13.2 checks that. The residual, stated
so that it is known: an info names what its seams write, not which of those
fields egui reads for that particular widget — a `Label` under the base style
lists every base-owner row. Narrowing it would take a hand-written list, per
widget, of the `Style` fields egui's paint code reads: claims about egui of
exactly the kind this design avoids.

**The palette.** Every item egui 0.36.2 and egui_extras 0.36.2 offer an
application: each type the coverage script's egui rule discovers (its rules are the implementation plan's)
— 14 widget types and 21 container types in egui (`HeaderResponse` among
them, by its `body`), 5 types in egui_extras —
each `Ui` method under ui.rs's four widget headings (69), and egui_extras's
`code_view_ui`: 110 items, measured with a prototype of that rule over the
0.36.2 sources on 2026-09-25. Each is on a page with the seam it
demonstrates, or **none**: native-theme models no counterpart and the item is
drawn with the atlas's base style, egui's own look inside the themed
application — itself the evidence of what the model does not cover. The
coverage script and §13.2's `every_seam_is_recorded` hold the
pages to this table.

| Page (`--tab`) | Items | Seam |
|---|---|---|
| Buttons (`buttons`), 7 | `Button` (text, icon and text, `small`, frameless, disabled, with `shortcut_text`), `ui.button`, `ui.small_button`, `ui.toggle_value` | `Role::Button`, in `Normal` — the suggested action too, which the button paints only with `.selected(true)` (§6.2, §14 item 43) — and `Disabled` |
| | `Button::selectable`, `ui.selectable_label`, `ui.selectable_value` | the base style: frameless at rest, they show the base hover fill (§14 item 31) |
| | a segmented control: a row of `Button::new(..).selected(i == current)`, §5's recipe — not `Button::selectable`, whose unselected segment paints no resting frame (`widgets/button.rs:81`, `:364-368`), so `segmented_control.background_color` would not show | `Role::SegmentedControl`: the row a `ui.horizontal` inside `ui.native_scope(Role::SegmentedControl, RoleVariant::Normal, ..)`, whose `item_spacing.x` is `segmented_control.separator_width` (§5.3) and whose `Normal` cell holds the active segment's colours, which each segment's own flag picks (§6.2). A `Button` is counted once, above, so this entry adds no item |
| | `AtomLayout` | none |
| | a switch: `Button::new(..).selected(checked)`, §5.3's substitute, off and on and disabled | `Role::Switch` through `ui.native_scope(Role::Switch, RoleVariant::Normal, ..)`, or `RoleVariant::Disabled` (§6.3); the checked state is the button's own flag, so the `Selected` cell is not used (§6.2). egui has no switch and this crate ships none (§14.3); this is the route the switch's leaves take (§5.3), and it shows a switch's track in both states but no thumb (§14 item 13). A `Button` is counted once, above, so this entry adds no item |
| Selection (`selection`), 6 | `Checkbox` (unchecked, checked, `indeterminate`, disabled), `ui.checkbox` | `Role::Checkbox`, `Normal`, `Selected` and `Disabled` |
| | `RadioButton`, `ui.radio`, `ui.radio_value` | `Role::Checkbox`: the model's `checkbox` is "Checkbox / radio button" (`native-theme/src/model/resolved.rs:169`) |
| | `ComboBox` (enabled, disabled) | `Role::ComboBox`, `Normal` and `Disabled`, its popup through `popup_style` |
| Inputs (`inputs`), 8 | `TextEdit` (single line, multiline, password, hint, disabled), `ui.text_edit_singleline`, `ui.text_edit_multiline`, `ui.code_editor` | `Role::Input`, `Normal` and `Disabled`. Each `TextEdit` the page builds takes `input_frame`, whose inner margin is `input_margin` (§4.7), so a focused one strokes `input.focus_border_color`; `ui.text_edit_singleline`, `ui.text_edit_multiline` and `ui.code_editor` build their own `TextEdit` (`egui/src/ui.rs:1806`, `:1816`, `:1825`) and keep egui's focused stroke, `selection.stroke`, and the difference is the demonstration |
| | `DragValue`, `ui.drag_angle`, `ui.drag_angle_tau` (`egui/src/ui.rs:1971`, `:1987`, each a `DragValue`) | `Role::Input`: a `DragValue` is a `Button` at rest (`egui/src/widgets/drag_value.rs:636-638`) and a `TextEdit` while typed into (`:585`); native-theme models no drag-to-edit number, and the input field is the look it takes while edited |
| | `egui_extras::DatePickerButton` | none; its calendar opens in an `Area` (`egui_extras/src/datepicker/button.rs:175`), which no role scope reaches (§1.5) |
| Range (`range`), 4 | `Slider` (horizontal, vertical, with its value, `trailing_fill`, disabled) | `Role::Slider`, `Normal` and `Disabled` |
| | `ProgressBar` (with text, animated) | `Role::ProgressBar` |
| | `Spinner`, `ui.spinner` | `Role::Spinner` |
| Text (`text`), 18 | `Label` (wrapped, truncated, selectable), `ui.label`, `ui.colored_label`, `ui.heading`, `ui.monospace`, `ui.code`, `ui.small`, `ui.strong`, `ui.weak` | the base style: the five `TextStyle` slots (§8.5); the text-role accessors (§4.7) state what egui cannot hold, shown by a label per `TextRole` in `text_role_font`, `text_role_line_height` and `text_role_weight` — the counterpart of the gpui showcase's Typography page |
| | `Hyperlink`, `Link` (enabled, disabled), `ui.link`, `ui.hyperlink`, `ui.hyperlink_to` | `Role::Link`, `Normal` and `Disabled` |
| | `Separator` (horizontal, vertical), `ui.separator` | `Role::Separator` |
| | `egui_extras::syntax_highlighting::code_view_ui`, `CodeTheme` with its `ui` editor | none: `CodeTheme::from_style` picks egui_extras's own dark or light colours by `visuals.dark_mode` (`egui_extras/src/syntax_highlighting.rs:237`, `:243-247`), and without the `syntect` feature egui_extras highlights with its own fallback (`:3-4`); only its font is the theme's, the base style's `Monospace` slot (`:238-241`) |
| Colour (`colour`), 8 | `ui.color_edit_button_srgba`, `_hsva`, `_srgb`, `_rgb`, `_srgba_premultiplied`, `_srgba_unmultiplied`, `_rgba_premultiplied`, `_rgba_unmultiplied` (`egui/src/ui.rs:2044-2120`), each opening its picker in `Popup::menu` (`egui/src/widgets/color_picker.rs:528`) | none: native-theme models no colour picker |
| Containers (`containers`), 39 | `Frame` with `native_frame(Surface::Card)`, beside `ui.group` | `Surface::Card`, its body in `Role::Card`; `ui.group` keeps egui's own `Frame::group`, whose margin is a literal (`egui/src/containers/frame.rs:180`), and the difference is the demonstration |
| | `CollapsingHeader`, `ui.collapsing`, `CollapsingState` with a custom header and the `HeaderResponse` its `show_header` returns (`egui/src/containers/collapsing_header.rs:129`, `:141`) | `Role::Expander`: the `CollapsingHeader` given `.icon(expander_icon(&t))` (§4.7), `ui.collapsing` without it — the difference is the demonstration |
| | `ScrollArea` (vertical, both axes, `show_rows`) | the base style, which carries the scrollbar's colours and widths for every `ScrollArea` (§5.5, §5.9). One is wrapped in a scope all the same, so that `Role::Scrollbar` is demonstrated: the `show_rows` one, whose contents are plain `Label`s, inside `ui.native_scope(Role::Scrollbar, RoleVariant::Normal, ..)`. That cell differs from the base style only in §6.8's radius, so the labels are unaffected |
| | `Panel::right` with a `CentralPanel`, nested inside the page | `Surface::Panel(PanelSide::Right)`, the panel side the chrome does not use |
| | `Sides`, `Resize`, `Scene`, `ui.columns`, `ui.dnd_drag_source`, `ui.dnd_drop_zone` (`egui/src/ui.rs:2642`, `:2694`), `ui.with_visual_transform` (`:2746`) | none |
| | the other layout methods: `add`, `add_sized`, `place`, `put`, `add_enabled`, `add_enabled_ui`, `add_visible`, `add_space`, `push_id`, `scope`, `scope_builder`, `scope_dyn`, `indent`, `horizontal`, `horizontal_centered`, `horizontal_top`, `horizontal_wrapped`, `vertical`, `vertical_centered`, `vertical_centered_justified`, `with_layout`, `centered_and_justified`, `end_row`, `set_row_height`, `columns_const` | none: each places other widgets and paints nothing of its own. The pages' own layout uses most of them; one no page uses is excepted with that reason and its line in `docs/showcase-exceptions.toml` |
| Data (`data`), 4 | `egui_extras::TableBuilder` and its `Table`: striped, resizable columns, a selected row | `Role::List`. A table's row height is a call argument no `Style` field reaches, so the helper passes `list.row_height`, read from the `ResolvedTheme`, and where the theme states none, the scope's `interact_size.y`, which then holds egui's own value (§5.4 `list.row_height`); the header takes `list_header_font` (§4.7) and `list.header_font.color` |
| | `Grid` (striped; and one in `RoleVariant::Disabled` with `ui.disable()`) | `Role::List`, `Normal` and `Disabled`, whose `interact_size.y` is `Grid`'s minimum row height (§5.4 `list.row_height`) |
| | `egui_extras::StripBuilder` | none |
| Overlays (`overlays`), 11 | `Window` (collapsible, resizable, scrolling) | `Surface::Window` and `Surface::WindowTitleBar`, its body in `Role::Window`; its title a `RichText` in `window_title_bar_font` and `window_title_bar_text_color(t, true)` (§4.7, §14 item 2b) |
| | `Modal` with a message and two buttons | `Surface::Dialog`, its body in `Role::Dialog`; the buttons in `dialog_button_order` (§4.7), the platform's order |
| | `Popup` opened from a toggle button (`Popup::from_toggle_button_response`, `egui/src/containers/popup.rs:230`), and a context menu (`Popup::context_menu`, `:248`) | `Surface::Popover` with `Role::Popover` through `Popup::style` (`egui/src/containers/popup.rs:417`); the context menu `Role::Menu` through the same call |
| | `Tooltip::for_enabled` (`egui/src/containers/tooltip.rs:53`), beside a `Response::on_hover_text` | `Surface::Tooltip` through the public `Tooltip::popup` (`egui/src/containers/tooltip.rs:9`), with `Role::Tooltip`; the `on_hover_text` one shows what §14 item 3 records |
| | `Area` | none: an `Area` has no style builder (§3.2) |
| | `ui.menu_button`, `ui.menu_image_button`, `ui.menu_image_text_button` (`egui/src/ui.rs:2788`, `:2822`, `:2859`) inside a page `MenuBar`, which gives them their `MenuConfig`; each builds a `MenuButton`, and inside a menu a `SubMenuButton` (`:2793-2797`) with its `SubMenu` (`egui/src/containers/menu.rs:357`); a menu holds a disabled item | `Role::Menu`, `Normal` and `Disabled`. The menus' icons are `ui.menu_button` items given an `(Image, text)` atom tuple, the image at `menu.icon_size` read from the `ResolvedTheme`; `ui.menu_image_button` and `ui.menu_image_text_button` are shown too, their icons at egui's Body row height, the clamp of `Button::image` and `image_and_text` they build (`egui/src/ui.rs:2829`, `:2833`, `:2867`, `:2871`; §9.4) |
| Icons (`icons`), 2 | `Image` through `icons::to_image` (§4.10) for every icon of the chosen icon theme, at each size `defaults.icon_sizes` states, each bundled Material or Lucide key tinted `ui.visuals().text_color()`, so a monochrome icon is drawn in it (§9.2); a system icon theme's icons are loaded in the text colour, a full-colour one left as it is (§9.2), as the gpui showcase loads them (`connectors/native-theme-gpui/examples/showcase-gpui/support.rs:366`); the tint demonstration stays on the bundled keys; `ui.image`; and the loading indicator the chosen icon theme ships — `FreedesktopLoader::load_indicator(Some(<chosen theme>))` (`native-theme/src/icons.rs:246`) for a freedesktop theme, `native_theme::icons::load_icon_indicator(set)` (`:506`) for a bundled set, as the gpui showcase does (`connectors/native-theme-gpui/examples/showcase-gpui/app.rs:504-513`), since `load_icon_indicator(IconSet::Freedesktop)` takes the system's theme (`native-theme/src/icons.rs:508`) and would mix two themes — drawn through `icons::animated_frame_index`, each frame's `IconData` handed to `to_image_source`, under the URI its bytes hash to (§9.2), or rotated by `icons::spin_angle`, with `atlas.accessibility().reduce_motion`, its `first_frame()` when that is set, its key tinted by the same rule as the other icons, as the gpui Icons page's Animated Icons section does (`connectors/native-theme-gpui/examples/showcase-gpui/pages/icons.rs:32-34`) | none: an image carries no theme. The icon theme follows the preset until the user picks one (the rules below) |
| the chrome, 3 | `MenuBar`, `Panel` (top, bottom, left), `CentralPanel` | the chrome table above |
| Theme Map (`theme-map`), 0 | every `mapping.toml` row in a `Table` built with `TableBody::rows` (`egui_extras/src/table.rs:1027`): the leaf, its value now, the verdict, the sinks with their scope or surface, and for an `unmappable` row its `sub_tag` and `upstream`; filtered by verdict with `selectable_value` | `Role::List`. The siblings' Theme Map pages list their toolkit's slots; this one lists the manifest that maps egui's |

Excluded, because they are not in egui or egui_extras: `egui_plot` and every
other crate outside the two. Left out by the coverage script's discovery rule itself, each for its reason: egui's style editors, the `Widget` impls on
`&mut Margin`, `&mut CornerRadius`, `&mut Shadow`, `&mut Stroke`,
`&mut Frame` and `&mut FontTweak` (`egui/src/style.rs:2777`, `:2834`, `:2892`,
`:2937`, `:2956`, `:3171`), and its introspection views
(`egui/src/introspection.rs:77`, `:138`, `:210`) — developer tools that edit
or report the very `Style` this crate installs; egui_extras's image loaders,
which are not widgets (the Icons page uses the SVG one, §9); and
`ClosableTag`, the marker `UiBuilder::closable` sets
(`egui/src/containers/close_tag.rs:12`). Left out as well, each drawn only
with widgets the palette already shows: egui's module-level widget functions
`widgets::{reset_button, reset_button_with, global_theme_preference_switch,
global_theme_preference_buttons}`
(`egui/src/widgets/mod.rs:124`, `:132`, `:144`, `:151`),
`color_picker::{show_color, color_picker_hsva_2d, color_picker_color32,
color_edit_button_*}`
(`egui/src/widgets/color_picker.rs:57`, `:493`, `:510`, `:518-575`),
`gui_zoom::zoom_menu_buttons`
(`egui/src/gui_zoom.rs:72`) and `ThemePreference::radio_buttons`
(`egui/src/memory/theme.rs:90`). egui_extras's widgets carry two
panic paths the showcase must not reach: a `Table` row or `Strip` given more
cells than it has columns panics in a debug build
(`egui_extras/src/table.rs:1287`, `egui_extras/src/strip.rs:176`), and
`TableRow::response` panics before the row's first cell
(`egui_extras/src/table.rs:1352`); the date picker unwraps dates it builds
itself (`egui_extras/src/datepicker/mod.rs:16`, `:19`, `:32`;
`egui_extras/src/datepicker/popup.rs:22`, `:419`, `:456`). The helpers add
exactly one cell per column, never call `response` before a cell, and hand
the date picker a valid `jiff::civil::Date`, leaving its year range at the
default (`egui_extras/src/datepicker/button.rs:104`), from which the picker
builds its dates; T11(a) runs every
page under debug assertions, which is where a miscounted row would fire.

The showcase example must obey these rules. The first six were each derived
from a defect found in a rejected draft; the next three carry over what the
v0.5.9 showcases do, and the last is Widget Info's:

* `eframe::App` in 0.36.2 has an optional `logic(&mut self, ctx, frame)`
  (`eframe/src/epi.rs:167`) and a **required**
  `ui(&mut self, ui: &mut egui::Ui, frame: &mut Frame)` (`:182`). There is no
  `update`.
* Panels are `egui::Panel::{left,right,top,bottom}(id).frame(f).show(ui, ..)`
  (`containers/panel.rs:249`, `:256`, `:265`, `:274`, `:413`, `:422`) and
  `egui::CentralPanel::default().frame(f).show(ui, ..)` (`:1187`, `:1206`,
  `:1212`). `show_inside` is deprecated on both (`:427-428`, `:1217-1218`) and
  must never appear.
* The application struct holds **one** field for theme state: `atlas: ThemeAtlas`.
  It never holds a `ResolvedTheme` beside it — `atlas.resolved_for(ui.ctx().theme())`
  is the reader.
* No `&self` accessor may be live across a closure that captures
  `&mut self.<field>` (`E0502`). Read what is needed into locals **before** the
  closure, or read it through `ui.native_theme()` **inside** the closure.
* No indexing expression anywhere. `ComboBox::show_index` closures use
  `.get(i).copied().unwrap_or(<first>)` — `clippy::indexing_slicing` is
  deny-level in this crate. An example is a crate of its own, which the
  library's crate attributes do not reach, so the example's `main.rs` opens
  with §4.1's `#![forbid(unsafe_code)]` and its seven `#![deny]` lints —
  `clippy::unwrap_used`, `expect_used`, `indexing_slicing`, `panic`,
  `unreachable`, `todo` and `unimplemented`. §4.1's `#![warn(missing_docs)]`
  is not repeated: it concerns a library's public API, and an example has
  none.
* `.into()` is never written on an argument whose parameter is `impl Into<T>`
  when the argument's own type already satisfies the bound (`E0283`) — notably
  `native_theme::icons::load_icon(IconRole::ActionSave, ..)`, which takes
  `impl Into<IconId<'a>>` (`native-theme/src/icons.rs:487`) and for which both
  `IconId<'_>` and `IconRole` satisfy the bound.
* **The command line takes what the pickers offer, and nothing else.** The
  capture pipeline starts every showcase with `--theme`, `--variant`,
  `--icon-set`, `--tab` and `--screenshot`
  (`.github/workflows/screenshots.yml:79-82`), so this one parses the same
  five. Each flag accepts exactly what its picker offers; a value the
  showcase cannot honour — no such preset, a preset of another platform, an
  icon theme that is not installed, no such page — is reported on stderr and
  ignored, and the setting stays what it would have been without the flag.
  That is the iced showcase's `apply_cli_args` and `reported`
  (`connectors/native-theme-iced/examples/showcase-iced.rs:1186-1230`, the
  other-platform message at `:307`), which a self-test there pins
  (`the_command_line_rejects_what_it_cannot_honour`, `:7086`).
* **The icon choice follows the theme until the user picks one.** Installing
  a theme re-derives the icon set and icon theme from it
  (`ThemeAtlas::icon_set` and `icon_theme(ctx.theme())`, §4.2), and a switch
  of colour scheme re-derives the icon theme — `breeze` to `breeze-dark` on
  `kde-breeze` — until the user picks
  an icon theme and again after a pick of the `default` row
  (`icon_choice_follows_preset`, `showcase-iced.rs:899-905`). An icon the
  chosen set or freedesktop theme lacks is shown as absent, never drawn from
  another set.
* **`--screenshot` captures the showcase's own frame.** It sends
  `egui::ViewportCommand::Screenshot` (`egui/src/viewport.rs:1199`) and writes
  the image that comes back in an `egui::Event::Screenshot` as a PNG through
  the `image` dev-dependency (§11). eframe's glow backend pushes that event
  (`eframe/src/native/glow_integration.rs:808-813`); its wgpu backend —
  eframe's default — hands the command to `egui_wgpu`'s painter
  (`eframe/src/native/wgpu_integration.rs:802-811`), which copies the frame
  into a capture texture and reads it back asynchronously (egui-wgpu 0.36.2,
  src/winit.rs lines 634-640 and 749-757; src/capture.rs line 207), and
  eframe pushes the `Event::Screenshot` at the start of a later pass
  (`eframe/src/native/wgpu_integration.rs:694`; egui-wgpu src/winit.rs lines
  775-785). So the event arrives on both backends, on wgpu one or more passes
  after the command; a readback that fails is logged and answered with no
  event (egui-wgpu src/capture.rs lines 208-219). The showcase never uses eframe's internal `__screenshot` feature, whose writer
  panics on failure (`eframe/src/native/glow_integration.rs:1748-1773`).
* **A seam is applied only where it is recorded.** Outside `demo.rs` and
  `chrome.rs` the showcase calls none of the connector's seams —
  `native_scope`, `native_set_style`, `native_frame`, `role_modifier` and
  `surface_frame` — and inside them a helper takes each seam as one parameter
  that it both applies and records. A widget drawn with a seam its Widget Info
  does not name is the defect gpui's `native_info` exists to prevent
  (showcase-app spec §5.2), and §13.2's `every_seam_is_recorded` holds this
  rule.

---

## 11 -- The full `Cargo.toml`

```toml
[package]
name = "native-theme-egui"
version.workspace = true
edition.workspace = true
license.workspace = true
# NOT `rust-version.workspace = true` — egui 0.36.2 declares 1.95, above the
# 1.88.0 workspace floor. See §12.4.
rust-version = "1.95"
repository.workspace = true
homepage.workspace = true
keywords = ["theme", "egui", "gui", "native", "colors"]
categories = ["gui", "config"]
readme = "README.md"
description = "egui toolkit connector for native-theme"

[package.metadata.docs.rs]
all-features = true
targets = [
    "x86_64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "x86_64-pc-windows-msvc",
]

[features]
# The native look is not opt-in (§12.1): everything that serves it without
# application code is on by default. `svg-rasterize` and `watch` stay opt-in.
default = ["material-icons", "lucide-icons", "system-icons", "system-fonts"]
material-icons = ["native-theme/material-icons"]
lucide-icons  = ["native-theme/lucide-icons"]
system-icons  = ["native-theme/system-icons"]
svg-rasterize = ["native-theme/svg-rasterize"]
system-fonts  = ["native-theme/system-fonts"]
watch         = ["native-theme/watch"]

[dependencies]
egui = "0.36.2"
native-theme = { workspace = true }
# `fonts::font_definitions` drops a face epaint could not parse before epaint
# sees it (§8): `FontRef::from_index` is the one fallible step of epaint's
# `FontFace::new` (`epaint/src/text/font.rs:386-388`). `Builder::build` reads
# the face metrics epaint's row height is made of (§6.15). The requirement is
# epaint 0.36.2's own (`epaint/Cargo.toml:149-155`), so cargo resolves one
# `skrifa` for both — checked by `cargo tree -d` (§13 T7).
skrifa = { version = "0.44.0", default-features = false, features = ["std"] }

[target.'cfg(target_os = "linux")'.dependencies]
native-theme = { workspace = true, features = ["linux"] }
[target.'cfg(target_os = "macos")'.dependencies]
native-theme = { workspace = true, features = ["macos"] }
[target.'cfg(target_os = "windows")'.dependencies]
native-theme = { workspace = true, features = ["windows"] }

[dev-dependencies]
eframe = "0.36.2"
# The showcase's palette is every widget egui_extras offers (§10.4): `svg` for
# the icons, `datepicker` for `DatePickerButton`. Not `syntect`: without it
# `code_view_ui` highlights with egui_extras's own fallback.
egui_extras = { version = "0.36.2", default-features = false, features = ["svg", "datepicker"] }
# `DatePickerButton::new` takes a `&mut jiff::civil::Date`
# (`egui_extras/src/datepicker/button.rs:29`), and egui_extras re-exports no
# `jiff`. Its `datepicker` feature turns on the `jiff` 0.2.35 dependency with
# `std`, `tz-system` and `js` (`egui_extras/Cargo.toml:65`, `:127-135`), which
# unifies with this one.
jiff = { version = "0.2.35", default-features = false }
# The showcase's self-tests (§13 T11) drive its real `eframe::App` headlessly
# and click it through AccessKit. `eframe` enables `HarnessBuilder::build_eframe`;
# neither `wgpu` nor `snapshot`: nothing is rasterised and no baseline is kept
# (rationale §8, Q-3).
egui_kittest = { version = "0.36.2", features = ["eframe"] }
# `--screenshot` (§10.4) writes the captured frame as a PNG.
image = { version = "0.25.10", default-features = false, features = ["png"] }
# §13 T2 and T10 parse `mapping.toml`; T3 walks `serde_json::to_value(&resolved)`;
# the showcase's Widget Info does both (§10.4).
# Neither crate is reachable transitively: `native-theme` depends on `toml`
# (`native-theme/Cargo.toml:55`) but re-exports it nowhere, and its own
# `serde_json` is dev-only (`native-theme/Cargo.toml:125`).
serde_json = "1.0.151"
toml = { workspace = true }

[[example]]
name = "showcase-egui"
# A module tree, as the gpui showcase is: `main.rs`, `app.rs`, `chrome.rs`,
# `demo.rs`, `info.rs`, `pages/`, `tests.rs` (§10.4).
path = "examples/showcase-egui/main.rs"
# The showcase's own `#[cfg(test)]` module (§13 T11) then runs under a plain
# `cargo test`, so CI and the dependency canary pick it up unchanged.
test = true

# The showcase's menus in the macOS system menu bar (§10.4): winit for
# `EventLoopBuilderExtMacOS::with_default_menu`, which eframe does not
# re-export, at eframe's own requirement (`eframe/Cargo.toml:266-268`); muda
# for the menu; objc2 and objc2-app-kit, at the versions `native-theme`
# requires (`native-theme/Cargo.toml:58`, `:68`), to read back
# `NSApplication::mainMenu` for the macOS runner's check (§13).
[target.'cfg(target_os = "macos")'.dev-dependencies]
winit = { version = "0.30.13", default-features = false }
# Off: muda's default `libxdo` and `gtk3` are Linux/BSD-only (its `Cargo.toml` lines 45-51,
# 96-107), change nothing on macOS, and would add the gtk-rs 0.18 stack to `Cargo.lock`.
muda = { version = "0.20.0", default-features = false }
objc2 = "0.6.4"
objc2-app-kit = { version = "0.3.2", features = ["NSApplication", "NSMenu", "NSResponder"] }
```

**No `required-features` on the example.** Cargo skips an example whose
required features are off — the iced manifest records the effect for its
`--no-default-features` run (`connectors/native-theme-iced/Cargo.toml:49-55`)
— so a `required-features` list would make every run with a feature off, and
every `--no-default-features` run, skip T11 without a word. The showcase's `watch`,
`system-fonts` and icon sections are `#[cfg(feature = …)]` instead, as the iced showcase's `iced_aw` sections are, and CI, the canary and
the screenshot matrix also build it with `--all-features` (the implementation plan wires both).
Those sections are gated on this crate's own features, which no
dev-dependency can turn on, so the manifest lists no `native-theme`
dev-dependency: a `native-theme` feature the showcase needs reaches it through
one of the six of §12.1.

The workspace `Cargo.toml` gains `"connectors/native-theme-egui"` in
`members` (the workspace members are five today, `Cargo.toml:2-8`). The loops
of `pre-release-check.sh` that discover the workspace's crates through
`cargo metadata` pick the crate up with no edit; the places that name each
connector explicitly do not, and the implementation plan lists them.

**Direct dependencies are `egui`, `native-theme` and `skrifa`, and nothing
else.** egui re-exports `epaint`, `ecolor` and `emath`
(`egui/src/lib.rs:436-438`), so a direct dependency on any of them would only
create a way to end up with two `ecolor`s in one graph. egui re-exports no
`skrifa`, and the font check and the build-time row height (§6.15) have to
run the parser epaint runs and read the metrics it reads, so `skrifa`
is named directly, at epaint's requirement: `0.44.0` with
`default-features = false` (`epaint/Cargo.toml:149-155`). A later `skrifa`
minor would compile a second parser that could disagree with epaint's, so
§13 T7 checks that `cargo tree -p native-theme-egui --target all -d` prints no
`skrifa` line. The check is scoped to this package on purpose: the
workspace's own lock already holds `skrifa` 0.37.0, 0.40.0 and 0.44.0, and
the iced connector's graph alone holds two of them (`cargo tree --offline -d`,
measured 2026-09-25). The font matching is `native-theme`'s, not this
manifest's (§8.2): `native-theme` gains `unicase` 2.9 as an unconditional
dependency for it — 2.9.0 is already in the workspace lock (`Cargo.lock:8711-8712`) — and `system-fonts` adds `fontdb`, and on
macOS `objc2-core-text` (§12.1). `egui_kittest` is a **dev-dependency** only, for the
showcase's self-tests (§13 T11; rationale §8, Q-3), so no consumer pays for it; it
depends on `egui` `0.36.2` with `default-features = false`, which unifies with
this crate's own requirement.

**egui's default features are inherited deliberately.** `egui = "0.36.2"` carries
no `default-features = false`, so `default = ["default_fonts"]`
(`egui/Cargo.toml:62`) stays on, chains to `epaint/default_fonts` (`:63`) and
compiles in the four bundled faces — `Hack`, `NotoEmoji-Regular`, `Ubuntu-Light`
and `emoji-icon-font` (`epaint/src/text/fonts.rs:506-532`). Cargo feature
unification is additive, so a downstream binary that sets
`default-features = false` on **its own** `egui` dependency cannot opt out of
them while this crate is in the graph. That is the intended position, not an
oversight: §8.2's "right metrics, wrong typeface" fallback and §8.1's emoji tail
both assume those faces are present. It costs binary size, and the position is
recorded here so the cost is a decision rather than an accident.

---

## 12 -- Feature flags, semver, egui version policy, MSRV

### 12.1 Features

Six features, every one a 1:1 pure passthrough to a `native-theme` feature
name. No feature name is invented at this layer.

| feature | forwards to | default | effect |
|---|---|---|---|
| `material-icons` | `native-theme/material-icons` | on | bundled Material icon set |
| `lucide-icons` | `native-theme/lucide-icons` | on | bundled Lucide icon set |
| `system-icons` | `native-theme/system-icons` | on | freedesktop / SF Symbols / Windows system icon lookup |
| `system-fonts` | `native-theme/system-fonts` | on | `FontPlan::from_system`, and the system typeface in `from_preset`, `from_system()` and `to_egui_atlas` (§4.6, §8) through `native_theme::fonts::system_face` |
| `svg-rasterize` | `native-theme/svg-rasterize` | off | forwards native-theme's in-process rasteriser, `native_theme::rasterize::rasterize_svg`, whose `IconData` `icons::to_color_image` converts (§9.1); this crate adds no item for it |
| `watch` | `native-theme/watch` | off | enables `ThemeWatcher` |

`default = ["material-icons", "lucide-icons", "system-icons", "system-fonts"]`.
**The native look is not opt-in**, and that decides the default. The icon
features are the iced connector's defaults (`connectors/native-theme-iced/Cargo.toml:23`;
the gpui connector's are the same four, `connectors/native-theme-gpui/Cargo.toml:27`)
less `svg-rasterize`, and with them an application shows the platform's own
icons with no feature to discover; `system-fonts` draws the text in the
typeface the platform names instead of egui's bundled faces (§14 item 8), the
largest single step toward the native look an application otherwise has to
know to ask for. It adds `fontdb` 0.23 to `native-theme`'s graph — the
version the workspace lock already holds through the iced connector's
`cosmic-text` (`cargo tree --offline -i fontdb@0.23.0`, measured 2026-09-25) —
with its `fs`, `fontconfig` and `memmap` features — fontdb 0.23.0's own
default set, `default = ["std", "fs", "memmap", "fontconfig"]`, which §8.2
keeps for its file mapping; fontdb 0.24.0 is the latest release, and taking it would put a
second `fontdb` into the workspace for no gain to the look. On macOS it also
adds `objc2-core-text` 0.3.2, for the system UI font's file (§8.2), already in
the workspace lock through `objc2-app-kit` (`Cargo.lock:5569-5570`).

Two features stay off, each because it adds nothing to the look.
`svg-rasterize` duplicates what the image loader the application installs
already does: `egui_extras`'s `svg` feature brings `resvg` 0.45.1
(`egui_extras/Cargo.toml:150-151`) and `native-theme`'s `svg-rasterize`
`resvg` 0.48.1 (`native-theme/Cargo.toml:52`), two semver-incompatible
minors, so defaulting it on would compile two SVG stacks into every binary
that shows an SVG icon (§9.1). `watch` enables an API the application must
call — `ThemeWatcher::start` and `take` (§4.11) — so a default would only add
a D-Bus, inotify or `CFRunLoop` thread's code to binaries that never start
one. A downstream crate that wants a smaller graph writes
`default-features = false` and names what it keeps.

### 12.2 The semver contract

Six clauses, all of which belong in the README as well as the rustdoc:

1. The crate version equals the workspace version (`Cargo.toml:12`).
2. One egui minor per release line of this crate; an egui minor bump is a
   **breaking change** for this crate.
3. **The numeric contents of a produced `egui::Style` are not covered.** A
   mapping fix — electing a different base owner, correcting a sink, closing a
   `Note` — is never a breaking change. This clause is what keeps every future
   mapping correction possible.
4. Every public enum is `#[non_exhaustive]` except `PanelSide`, so a new `Role`,
   `Surface`, `RoleVariant`, `Note`, `TextRole`, `IconContext` or `FontBytes`
   variant is additive. The attribute on the *enum* does **not** protect a
   variant's payload, so each of `Note`'s six struct variants carries its own
   `#[non_exhaustive]` (§4.3) and enriching one of them stays additive too.
   `Surface::Panel` and both `FontBytes` tuple variants are deliberately left
   exhaustive: on a *tuple* variant the attribute makes the variant
   unconstructible outside this crate, and constructing them is the documented
   call shape.
5. Adding a free accessor is additive.
6. `ThemeAtlas`, `Builder`, `FontPlan`, `IconKey` and — behind feature `watch` —
   `ThemeWatcher` are opaque — no struct this crate defines has a public
   field — and `NativeThemeUiExt` and `SystemThemeExt` are sealed (§4.5,
   §4.6), so adding a method to any of them is additive. For
   `NativeThemeUiExt` that holds only for a name that exists on neither
   `egui::Ui` nor `egui::Context`: an inherent method beats a trait method
   only at the same receiver step, so a new trait method would silently take
   over the calls of a `Context` method of its name reached through `Ui`'s
   `Deref`, or of a `&mut self` `Ui` method when it takes `&self` (§4.5).

Clause 4 exists because **native-theme churn is an axis independent of egui
churn**. Two new native-theme widget structs force two new `Role` variants with
no egui bump at all, breaking every downstream exhaustive `match` and every
struct literal. The version pin buys nothing on that axis. `PanelSide` is
exhaustive on the opposite argument: four sides is a closed fact of 2-D screen
geometry, not an API taxonomy that can churn, and applications benefit
permanently from being able to `match` it. Rationale §8, Q-4 records the decision.

### 12.3 egui version policy

`egui = "0.36.2"` — a caret requirement, i.e. `>=0.36.2, <0.37.0`. It accepts
later 0.36 patch fixes and rejects 0.37. Every egui-family dev-dependency of
§11 (`eframe`, `egui_extras`, `egui_kittest`) states the same `0.36.2`.

**Why the floor is 0.36.2 and not 0.36.1.** This document is verified against
0.36.2, and one 0.36.2 change alters behaviour the design relies on: a
`Window` given a custom `frame` and no `title_frame` now paints its title bar
with that custom frame (`title_frame.unwrap_or(window_frame)`,
`egui/src/containers/window.rs:630-631`), where 0.36.1 fell back to
`Frame::window(&style)`. `Surface::Window` and `Surface::WindowTitleBar`
therefore render differently on 0.36.1, and a floor that admitted it would
admit a look this document never checked.

An exact `=0.36.2` pin is wrong because it would reject patch fixes. A wider
range is impossible because the application's `egui::Style` must be *our*
`egui::Style` and cargo cannot unify two semver-incompatible egui versions. The
same shape as the sibling connectors, which require `iced_core = "0.14"`
(`connectors/native-theme-iced/Cargo.toml:36`) and
`gpui = { package = "gpui-pre", version = "0.3.6" }`
(`connectors/native-theme-gpui/Cargo.toml:40`).

The policy is published in the README's **Compatibility** section — the
required floors in a table, and the dated upstream set the crate was last
verified against — the shape both sibling READMEs have
(`connectors/native-theme-iced/README.md:35-59`), and there is **no**
`EGUI_VERSION` constant: a hand-maintained string cannot be checked against the
resolved dependency and would eventually lie. The README's floors are checked
against this manifest by a test instead, as the siblings' `src/compat.rs` does.

The churn evidence behind this policy is rationale §5.1 and §4.2.

**What the tripwires cannot catch, and the bump step that follows from it.**
Upstream `Style` can drift in three ways and only two of them are mechanical.
A field **added or removed** becomes a compile error naming the field, because
§13 T1 destructures every relevant struct with no `..` rest pattern. A field
whose **type** changes is caught too, but by the assignment site rather than by
T1, so the error names this crate's mapping code and not the upstream change.
A field whose **meaning** changes with no signature change is caught by
**nothing** — and that is not hypothetical: `Spacing::extra_text_line_spacing`
(`style.rs:424`) could be redefined from logical pixels to a multiplier, and
§6.15's derivation would then be silently wrong by a factor of the font size
with every test still green. §13 T1b narrows the third mode by pinning three
default *values* this document reasons from, but it does not close it. So the
egui-bump procedure carries one step no tripwire can supply: **read the
doc-comment diff for every field this crate writes, not only the field list.**

### 12.4 MSRV — workspace 1.88.0, this connector 1.95

egui 0.36.2 declares `edition = "2024"` (`egui/Cargo.toml:13`) and
`rust-version = "1.95"` (`:14`), above the workspace floor, and so do
`epaint`, `ecolor`, `emath`, `egui_extras`, `eframe` and `egui_kittest` 0.36.2,
each at lines 13–14 of its own `Cargo.toml`. **The connector declares
`rust-version = "1.95"` explicitly and does *not* write
`rust-version.workspace = true`** — the precedent is the gpui connector, which
declines to inherit for the same reason and declares `1.95.0`
(`connectors/native-theme-gpui/Cargo.toml:6-10`).

The workspace floor itself, `1.88.0` (the root `Cargo.toml`, line 15), is the measured
maximum of what the dependency graph declares outside the gpui connector's
closure and what our own sources need; the measurement and its history are
rationale §3.18.

The dependencies this plan adds move no floor. `fontdb` 0.23.0
(`rust-version = "1.60"` on the crates.io index), which `system-fonts` adds to
`native-theme`, and `skrifa` 0.44.0 (`"1.85"`), which this crate names, are
already in the workspace graph, through the iced connector's
`cosmic-text` and `swash` (`cargo tree --offline -i`), and so is `unicase`
2.9.0, which `native-theme` gains for font matching and which declares no
`rust-version`; `objc2-core-text` 0.3.2,
which `system-fonts` adds on macOS, declares `rust-version = "1.71"` (line 14
of its `Cargo.toml`), below the `1.88.0` floor; muda 0.20.0 (`"1.90"`),
winit 0.30.13 (`"1.70.0"`), objc2 0.6.4 and objc2-app-kit 0.3.2 (`"1.71"`)
are macOS-only dev-dependencies of this crate, below its `1.95`.

`rust-version` is a per-package key whose workspace inheritance is opt-in, so
declining to inherit is ordinary and supported, not a workaround. Cargo
refuses a toolchain below any compiled package's `rust-version`, dependencies
included, with an error naming that package (measured with cargo 1.88.0
against a path dependency declaring `1.95`), so egui's own `1.95` already
stops such a build; declaring `1.95` here keeps this crate's own metadata true
instead of inheriting a `1.88.0` it cannot meet.

**Still outstanding, and now the load-bearing part.** A measured floor decays the
moment a dependency bumps its own requirement, so the number is only as good as
the job that checks it, and `.github/workflows/ci.yml` still has no such job.
The job:

```yaml
  msrv:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - uses: dtolnay/rust-toolchain@1.88.0
      - run: cargo check --workspace --exclude native-theme-gpui --exclude native-theme-egui --all-features --locked
      - uses: dtolnay/rust-toolchain@1.95.0
      - run: cargo check -p native-theme-egui --all-features --locked
      - name: Install system dependencies (gpui)
        run: sudo apt-get update && sudo apt-get install -y libxcb1-dev libxkbcommon-dev libxkbcommon-x11-dev libfontconfig1-dev libfreetype-dev
      - run: cargo check -p native-theme-gpui --all-features --locked
```

`--locked` on every run is not decoration: the floor was measured against
the committed `Cargo.lock` (rationale §3.18), and without it cargo may resolve a newer dependency
whose own `rust-version` moved, so the job would stop testing the number the
workspace declares. The gpui connector is excluded from the first run because it
declares `1.95.0` and would stop it with a `rust-version` error, and is checked
by the job's last step at its own floor — the one `1.95.0` toolchain serves both
connectors (the `1.95` channel would follow a later 1.95.x and leave the floor), as `docs/todo.md`'s MSRV item requires
(1.95.0 builds its closure, 1.94.0 does not). That step first installs the gpui
system libraries every gpui job installs (`.github/workflows/ci.yml:39-41`),
because the gpui-pre stack needs `fontconfig.pc` even for `cargo check`.

---

## 13 -- Headless testing strategy

Eighteen groups, all headless. T1–T9 and T13–T18 need a bare
`egui::Context` or no `Context` at all; T10–T12 are the three test layers v0.5.9 made binding on
every connector (`docs/archive/todo_v0.5.9_theme-contracts-rationale.md:524-526`,
C6–C8), and only T11 uses `egui_kittest` (rationale §8, Q-3). No snapshot testing and no
screenshot comparison: that is the theme-contracts rationale's deliberately
deferred Layer 4 (C9, `:527`; trigger at `:545`). T1b is a second, value-level tripwire living in T1's
module rather than a group of its own. Three checks that only a macOS or a
Windows machine can make follow the table; they are not headless and are not
counted among the groups.

| T | Name | What it does | Why it is sufficient |
|---|---|---|---|
| T1 | **Compile-time drift tripwires** | Exhaustive destructuring with **no `..` rest pattern** of `Spacing` (21 fields, `style.rs:392-465`), `Visuals` (36 fields, with `#[expect(deprecated)]` for `clip_rect_margin` at `:1086`), `WidgetVisuals` (6, `:1295-1319`), `Selection` (2, `:1196-1199`), `Widgets` (5, `:1255-1269`), `Interaction` (8, `:916-944`), `ScrollStyle` (16, `:503-582`), `TextCursorStyle` (5, `:953-965`), `ImeComposition` (3, `:1208`, `:1211`, `:1229`), `ScrollFadeStyle` (2, `:788`, `:792`), `ScrollAnimation` (2, `:833`, `:836`), `Style` itself (17, `:244-342`), `egui::Frame` (6, `containers/frame.rs:104-140`), and `epaint::{Shadow, Margin, CornerRadius, Stroke, TextOptions, FontId}` (`TextOptions` 4 fields, `epaint/src/text/mod.rs:29-53`; `FontId` 2, `epaint/src/text/fonts.rs:23`, `:26`) | A new upstream field becomes a compile error **naming the field**. Strictly better than a `size_of` assertion. `Style`'s `debug` field exists only under `debug_assertions` (`style.rs:323-324`), so its pattern binds it as `#[cfg(debug_assertions)] debug: _`: a `cfg` on a field pattern is honoured, and the one pattern compiles in both profiles (measured with rustc 1.98.1, 2026-09-25). A destructure binds a nested struct **by name and stops there**, so every type reachable from a destructured one is listed separately or the tripwire has a hole at that level — `Visuals::text_options` (`:1001`), `Visuals::ime_composition` (`:1033`), `ScrollStyle::fade` (`:582`), `Style::scroll_animation` (`:338`), which §4.2's reduced motion writes, and the `FontId`s of `Style::text_styles` and `override_font_id`, whose sizes the atlas writes, are exactly those holes. `egui::Frame` is listed although it is not a `Style` field: it is the public return type of two §4.2 methods, the sink for the whole `Surface` axis and a normative manifest spelling (§13.1), and upstream guards it only with a `size_of` test (`containers/frame.rs:143-154`) — the weaker technique this row exists to replace |
| T1b | **Upstream default-*value* tripwires** | Three assertions in T1's module: `egui::Style::default().spacing.extra_text_line_spacing == 0.0` (`style.rs:1465`), `egui::Style::default().spacing.interact_size.x == 40.0` (`:1460`) and `egui::Visuals::dark().disabled_alpha == 0.5` (`:1560`, field `:1126`) | T1 catches a field that appears or disappears; it cannot catch a *value* that moves under an unchanged signature. These three are the defaults this document reasons **from**: §6.15's floor gives egui's own resting `0.0`, §7.5 leaves `interact_size.x` deliberately inherited at `40.0`, and §7.4's whole panic argument is about `disabled_alpha`. A change of *meaning* with no change of signature or value is still caught by nothing; §12.3 names that residue and the bump step it forces |
| T2 | **`mapping.toml` differential coverage** | For each row with `verdict != unmappable`: splice that one native leaf from preset B into preset A for every pair of the fixed list that differentiates it, in both modes, and splice the row's `probe` when it has one; rebuild with an empty `FontPlan` and `AccessibilityPreferences::default()`, diff via an exhaustive-destructuring `style_diff` — in the base styles, every role cell and every `Surface`'s `Frame` — and assert that no splice moves a sink the row does not declare at a location that carries it — a base-style sink in every cell that does not override the field too, and in every `Surface` frame whose preset reads it where the surface sets no value of its own (§3.4; §13.1, *Inheritance*) — and that every declared sink without a `when` moves under at least one splice (two presets can give one sink the same value by chance: kde-breeze's `button.min_height` and adwaita's `menu.row_height` are both `32`). A `tested_by` row changes nothing anywhere | **No discretionary skip allowance.** The test iterates a fixed list of preset pairs and requires every row to be exercised by at least one pair. A row that no pair differentiates fails with an actionable message and must be given an explicit `probe` value in the manifest. There is no knob a maintainer can raise instead of fixing a mapping. A `when` sink is the one change the test does not require, because another input of its formula can hold it still on a given pair; its value is T10's in all 32 combinations, and a `tested_by` row's route is its named test's |
| T3 | **Converse coverage** | Walk every leaf of `serde_json::to_value(&resolved)` (`ResolvedTheme` derives `Serialize`, `native-theme/src/model/resolved.rs:155`) plus the four `LayoutTheme` leaves, and assert each has exactly one `mapping.toml` row; and that every row that is not `unmappable` carries exactly one of `sinks` and `tested_by`, a `tested_by` naming a clause of this table | Without this, the leaf count §5.7 accounts for is a claim about a document rather than about the code |
| T4 | **Hostile input, with egui's own asserts as the oracle** | **(a)** `ResolvedTheme`'s fields are public, so write `NaN`, `+∞`, `−∞`, `-0.0`, `1e30` and `-1e30` into every `f32` leaf, and into every `Some` of an `Option<f32>` leaf (the four padding sides, and the five optional sizes of rationale §3.23); assert the atlas still builds, every produced `Style` equals itself, and exactly the `Note`s §7.2's emission rule predicts are emitted, and no others. **(b)** Under `cargo test`'s default debug assertions, **explicitly call `Ui::dnd_drop_zone::<A, _>`** with every `Visuals` the all-at-once atlases of (a) produce — those built from a theme whose padding sides and sizes are `NaN` and `±∞` among them — after `egui::DragAndDrop::set_payload` (`egui/src/drag_and_drop.rs:79`) has set a payload of another type `B`. **(c)** With `defaults.font.size` and `button.font.size` each `NaN`, then `+∞`, then the subnormal `1e-45`, install the atlas on a `Context` that keeps egui's default fonts (not T14's `FontDefinitions::empty()`, under which no glyph is laid out and neither panic can fire, `epaint/src/text/fonts.rs:640-648`) and run one pass that lays out a `Label` and, in `ui.native_scope(Role::Button, ..)`, a `Button`: no panic, a `Note::ValueSanitised` for each such leaf, and every `FontId` accessor of §4.7, `font_size` and `mono_font_size` return a positive normal size | (a) is reachable in production, not hypothetical: of a widget border, only the stated padding sides are range-checked (`check_padding`, emitted at `native-theme-derive/src/gen_ranges.rs:127-141`) — its `corner_radius` and `line_width` are not — and `card` is absent from the `check_ranges` dispatch (`native-theme/src/resolve/validate.rs:168-192`). (b): `Ui::dnd_drop_zone` (`ui.rs:2726-2727`) — which calls it only while something it cannot accept is being dragged (`:2702-2704`, `:2724`), hence the foreign payload — is the only path to `Visuals::disable` → `Color32::gamma_multiply`'s `debug_assert!(0.0 <= factor && factor.is_finite())` (`ecolor/src/color32.rs:270-273`). `Ui::disable` does **not** reach it (§7.4), so a test that only calls `ui.disable()` proves nothing. (a)'s self-comparison runs emath 0.36.2's `debug_assert!` against `NaN` in every `Vec2` of `Spacing` (`emath/src/vec2.rs:349-359`, §7.4). (c): building a style lays out no text, so only a pass reaches epaint's `debug_assert!` on the font scale (`epaint/src/text/font.rs:181-184`), which a subnormal size trips too, and the release panic a `+∞` size causes in the rasteriser (§8.5) |
| T5 | **Line spacing at build** | Pure, with one `Context` as the oracle. For every bundled preset in both modes, at a text-scaling factor of `1.0` and of `2.0` — where §6.6's floor bites on all four platform presets, twice their measured Body rows being above every `1.25 · d` — once with no plan (egui's default faces) and once with a plan whose face is egui's bundled `Hack` (`epaint/src/text/fonts.rs:506-532`): every base style and every role cell of the atlas, in every variant, holds one `spacing.extra_text_line_spacing`, and it equals §6.15's value with the row height taken from egui itself — `ctx.fonts_mut(\|f\| f.row_height(&egui::FontId::proportional(s)))` (`epaint/src/text/fonts.rs:865-875`), `s` the Body size the atlas installs, on a `Context` given the same `FontDefinitions` and run for one pass; and every `Role::Slider` cell's `expansion` equals §6.6's with that same row height | §6.15 recomputes egui's row height from the face's metrics as epaint does (`epaint/src/text/font.rs:397-400`, `:561-565`, `:587`), so what can go wrong is a drift from epaint's arithmetic — a rounding, a tweak scale, the wrong face — and egui's own `row_height` is the oracle that catches it. The row height scales with the font size in points, never with the pixels per point (`:561`), so one `pixels_per_point` suffices; the recomputation matched egui in 168 of 168 comparisons — every bundled preset with egui's default faces, and the KDE and GNOME system faces at two scale factors — measured 2026-09-25 |
| T6 | **Fonts** | Pure except for the last clause of (b). **(a)** `fonts::font_definitions` adds no `FontFamily::Name` key, and every `FontId` in every style of an atlas names `FontFamily::Proportional` or `Monospace`, whatever family a leaf states; the emoji fallback tail survives; an empty plan is a no-op; and, in a unit test inside `fonts`, the crate-private `supports_weight_axis` returns `false` for garbage bytes without panicking. **(b)** Given a plan whose face is garbage bytes, a face cut short inside its table directory, a single face addressed with collection index `1`, or a face whose `head` states `unitsPerEm` `0`: the face is absent from the returned `FontDefinitions`, its family's list holds egui's own faces, and exactly one `Note::FontDataInvalid { family }` names it; `ctx.set_fonts` of the result followed by a pass does not panic. **(c)** With feature `system-fonts`, `FontPlan::from_system` over a theme whose `defaults.font` and `defaults.mono_font` name a family no system has (a generated name): the plan holds no face, `font_definitions`' notes hold one `Note::FontFamilyUnavailable` per family, and it leaves `Proportional` and `Monospace` as `FontDefinitions::default()` has them. **(d)** Given a plan holding faces of two families, two weights and both styles, `font_definitions` puts at the head of `Proportional` the face `select_face` picks for `defaults.font`'s family, weight and style, and at the head of `Monospace` the one for `defaults.mono_font`'s, each entry's `FontTweak::coords` holding `wght` at that spec's weight (§8.3). **(e)** A `FontFamily::Name` family in a plan's base (`FontPlan::with_base`) survives `font_definitions`, its chain and its `font_data` intact | (a) proves the never-`Name` invariant **mechanically** rather than by review — the invariant that makes two upstream panics unreachable (§8.1). (b) is the panic A4 closes: epaint parses every face with `skrifa::FontRef::from_index` (`epaint/src/text/font.rs:386-388`) and panics on failure in every build (`epaint/src/text/fonts.rs:990`), so the check must be the same call; garbage, a file cut short inside its table directory and a wrong index are its three ways to fail (read-fonts checks the table directory, not each table's range, so a cut past the directory still parses), and a zero `unitsPerEm` passes it but is a division epaint makes (§8.2). (c) is the "never substitute" rule: a missing family is reported, never replaced by another. (d) is the route of the two default fonts' family, weight and style, which reach no `Style` field. (e): `set_fonts` overwrites the whole definition, and a `Name` family it drops panics in release at the first text that names it (§4.9) |
| T7 | **Lints, MSRV, package** | `clippy -- -D warnings` with `unwrap_used`, `expect_used`, `indexing_slicing`, `panic`, `unreachable`, `todo` and `unimplemented` at deny — the same seven §4.1 declares, and no fewer; `#![forbid(unsafe_code)]`; `cargo check` on 1.95; `cargo package` dry run via `pre-release-check.sh`; the library clean under that script's strict-panic set (`pre-release-check.sh:487-503`), whose `arithmetic_side_effects`, `integer_division` and `modulo_arithmetic` are not crate attributes (§4.1); and `cargo tree -p native-theme-egui --target all -d` printing no `skrifa` line (§11) | House workflow; the MSRV job is what makes `rust-version = "1.95"` non-decorative (§12.4). The list is not a proof of the no-panic rule and must not be read as one: `assert!`, `assert_eq!`, `debug_assert!` and `BTreeMap`/`HashMap` indexing all compile clean under it and are banned by convention and review instead (§4.1) |
| T8 | **Documented-limit regressions** | Three assertions, for the behaviours this crate's own design answers for; the other ledger rows that state a behaviour describe egui alone, and asserting them would test egui. **(a)** §7.5 and §14 item 30: register a `TextStyle::Name` key in both of a `Context`'s styles, call `install`, and assert the key, with its `FontId`, is in `ctx.style_of(Theme::Dark)` and `ctx.style_of(Theme::Light)` (`context.rs:2221`); in a pass, assert that `TextStyle::Name(..).resolve(ui.style())` returns it inside `ui.native_scope(Role::Button, RoleVariant::Normal, ..)`, after `ui.native_set_style(Role::Sidebar, RoleVariant::Normal)`, and inside a `Popup` given `role_modifier(theme, Role::Popover, RoleVariant::Normal)` — every public seam that replaces a style (§4.5). **(b)** §14 item 2b: on `kde-breeze`, whose scrollbar is no overlay (`native-theme/src/presets/kde-breeze.toml:149`), so egui paints it at full opacity (`containers/scroll_area.rs:1484`, `:1496`), show a `Window` with scrolling enabled — the `ScrollArea` branch is gated on `scroll.is_any_scroll_enabled()` (`containers/window.rs:738`) — and content taller than it, the theme's `button.border.corner_radius` first set apart from its `defaults.border.corner_radius` (the fields are public, as in T4); apply a `Role::Scrollbar` scope as the first statement inside the closure; and assert that the handle the window paints (`containers/scroll_area.rs:1515-1519`, a `RectShape` in the window's layer of the pass's shapes) is filled with `scrollbar.thumb_color`, the base style's `widgets.inactive.bg_fill` (§5.9), and rounded with the base style's `widgets.inactive.corner_radius`, not the scope's, having first asserted that the two radii differ. The scrollbar's colours and widths are the base style's inside the scope and out (§5.5, §5.9), so only the `Role::Scrollbar` radius, `defaults.border.corner_radius` (§6.8), where it differs from `button.border.corner_radius`, can tell the two apart. **(c)** §9.3: an `IconData::Rgba` with a side larger than `ctx.input(\|i\| i.max_texture_side)` makes `icons::to_image_source` return `None` rather than reaching `load_texture`'s `debug_assert!` | A ledger row that names a concrete loss is a testable claim, and an untested one quietly becomes false at the next egui bump — the exact failure mode §14 exists to prevent. **(a)** guards a release-mode panic: `TextStyle::resolve` panics on a missing key in every build (`style.rs:112-120`), and each of those seams replaces a whole style, so a seam that forgot the merge would crash an application that names its own text styles; **(b)** is the residue that §1.5's inside-the-closure scope does *not* reach, so it is what stops item 2b being re-described as a total loss or as no loss at all; **(c)** is the one `None` that costs a visible icon, so it must be a deliberate `None` and not an accident |
| T9 | **Icon URI, end to end** | Install `egui_extras::install_image_loaders` on a bare `Context`, build a URI through `icons::uri` for an `IconData::Svg`, and assert `Context::try_load_image` (`egui/src/context.rs:3861`) does **not** answer `Err(LoadError::NoMatchingImageLoader { .. })` for it — that, not `NotSupported`, is what a rejected URI produces: `try_load_image` swallows each loader's `NotSupported` and falls through to `NoMatchingImageLoader` (`:3873-3885`), while its own doc comment says `NotSupported` (`:3853`), so an assertion written from the doc would pass vacuously. **Per-scheme icon theme:** for both values of `is_dark`, `from_preset("kde-breeze", is_dark, ..)`'s atlas reports `icon_theme(egui::Theme::Light) == Some("breeze")` and `icon_theme(egui::Theme::Dark) == Some("breeze-dark")` (`native-theme/src/presets/kde-breeze.toml:9`, `:317`). **Tint:** a Lucide `currentColor` SVG given a tint yields, through `to_image_source`, `ImageSource::Bytes` whose bytes contain the tint's `#rrggbb` and no `currentColor`; without a tint the bytes are the payload's, unchanged. **The URI follows the bytes:** two frames of the bundled Material indicator (`native_theme::icons::load_icon_indicator(IconSet::Material)`, an `AnimatedIcon::Frames`) handed to `to_image_source` under one key give different URIs; the same `IconSet::Freedesktop` key under `Theme::Light` and `Theme::Dark` of an installed two-variant atlas whose `defaults.text_color`s differ yields two URIs, and two sets of bytes each holding its scheme's `#rrggbb` and no `currentColor`, for a `currentColor` test SVG that sets no colour, and one URI, over bytes left unchanged, for a full-colour test SVG; a Breeze-style SVG whose stylesheet sets `color` is handed on unchanged, under one URI; and a provider returning different bytes for the same name yields different URIs. **Provider:** `custom_icon_to_image_source` over a test `IconProvider` that names no icon and returns an SVG from `icon_svg`, under a key in `IconSet::Material`, gives `ImageSource::Bytes` under `icons::uri` of those bytes, holding exactly those bytes. **Converters and animation:** `to_color_image` gives `None` for a zero width, a zero height, `width = height = u32::MAX` (whose `width * height * 4` overflows `usize`), a buffer one byte short and an `IconData::Svg`; `animated_frame_index` and `spin_angle` give `None` under `reduced_motion` and for the other `AnimatedIcon` variant, else an index below the frame count and a finite angle; `forget_icons` after `to_image_source` of an `IconData::Rgba` returns `ctx.tex_manager().read().num_allocated()` (`egui/src/context.rs:2417`, `epaint/src/textures.rs:117`) to its count before the upload | §9.2's `.svg`-suffix contract has no mechanical protection: `is_supported` is a private module-level `fn` (`egui_extras/src/loaders/svg_loader.rs:31-33`) that `SvgLoader::load` calls before it looks at anything else (`:59-62`). `SvgLoader` itself is public by its module path — `egui_extras::loaders::svg_loader::SvgLoader` (`egui_extras/src/lib.rs:19`, `egui_extras/src/loaders.rs:120-121`) — but calling it directly would skip what an application's image goes through; the check goes through `Context::try_load_image` instead, so it also covers the loader's registration by `install_image_loaders` — and §9.2 requires §13 to carry it; a single icon-theme name would give a dark style the light scheme's icons, which no other test sees; a tint left out of the bytes draws a monochrome icon black on a dark scheme, which only the bytes show; frames sharing a URI draw frame 0 forever, since the bytes loader keeps the first bytes per URI (§9.2), as a scheme switch that kept a freedesktop icon's URI would keep the other scheme's colour, and a full-colour icon, or a Breeze icon's stylesheet colour, recoloured would lose its own colours; and a provider's icon is loaded by a path of its own (§4.10), which none of the other clauses reaches; `to_color_image`'s `None` paths stand between a malformed buffer and an `assert_eq!` that fires in release too (§9.3), and a `forget_icons` that missed the stored handles would keep every replaced texture |
| T10 | **Mapping contract (C6)** | Iterate `mapping.toml` itself over every preset `Theme::list_presets()` returns (`native-theme/src/model/mod.rs:653`) — sixteen; the four `-live` presets are merge bases, not selectable — in both modes: the 32 combinations of C6, which this crate's own `#[cfg(test)]` `contract` module iterates as the siblings' do (rationale §8, Q-5). Each mode's resolved theme is checked against what the atlas publishes for the matching `egui::Theme`, each sink read at the location it names (§13.1): the base `Style`, a role cell, or a `Surface`'s `Frame`. For every row that is not `unmappable`, each declared sink must hold the leaf's own value through the row's conversion — the `convert` of §7.2 for `direct` and `scoped` rows, the named formula for `derived` ones, among them `composite_over` for §6.1's hover and pressed state layers (C17), and for a `when` sink the formula of the row that writes it — and a leaf that is `None` (an unstated padding side, one of rationale §3.23's five optional sizes) must leave its sink at the value its location inherits — egui's own `Style` for that `Theme` under the base style, the base style under a role's `Normal` cell (with `menu_style` applied for `Role::Menu`), the `Normal` cell under its `Selected` and `Disabled` cells, and egui's preset frame over the base style under a `Surface` (§3.4; §13.1, *Inheritance*). So §6.11's checkbox inset gives `spacing.icon_width_inner` `14` in `adwaita`'s Checkbox cell in both modes and leaves the base style's value, egui's `8.0`, on every preset that states no checkbox padding; on `adwaita` with `top` and `bottom` first set to `5` (the fields are public, as in T4), where the pairs differ, the mean of both, `20 − (6 + 10) / 2 = 12`; and on `adwaita` with all four sides first set above half of `checkbox.indicator_width`, `clamp_length`'s floor, `0.0`. A row that legitimately differs on some preset lists it under `exceptions` with the reason (§13.1), never a loosened assertion; a `tested_by` row has no sink to read here, and its named test checks its route. **T10b, the slot-side tripwire:** every path `style_diff` can emit, and every `Frame` field of every `Surface`, is a sink of at least one row or a key of the manifest's `[unwritten]` table, which states why egui's own value stands there. **And every produced `Style`** — both base styles and every role cell in every variant, in every combination — **equals itself**, `assert_eq!(*a, *a)` | T2 proves *which* sinks a leaf reaches and T3 that every native leaf has a row; neither proves that a sink holds the *right value* in every combination, the defect class C6 exists for (`docs/archive/todo_v0.5.9_theme-contracts-rationale.md:524`). T10b is T3's converse on egui's side: a `Style` field this crate should write and does not is otherwise invisible, because no native row names it. It needs the third list the siblings did without (theme-contracts spec §5.2), because here egui's own value stands deliberately for real fields — `interact_size.x` (§7.5), the five inert knobs (§14 item 17). The self-comparison is the `NaN` sweep: `Style` derives `PartialEq` (`style.rs:241`) and one `NaN` anywhere makes the value unequal to *itself*, while `Style::number_formatter` (`style.rs:298`), compared by `Arc::ptr_eq` (`:57-62`), always equals itself; T4 feeds the infinite inputs |
| T11 | **Showcase self-tests (C7)** | In the showcase's own `#[cfg(test)]` module (`test = true`, §11), driving its real `eframe::App` through `egui_kittest::Harness::builder().with_theme(..).build_eframe(..)` (`egui_kittest/src/builder.rs:75`, `egui_kittest/src/builder.rs:212`), each test installing a named preset rather than reading the desktop. **(a) `every_page_renders`**: for every page in both `egui::Theme`s, the harness runs the app's `logic` and `ui` (`egui_kittest/src/app_kind.rs:38-42`) until it settles; the pass produced shapes, and they tessellate (`Context::tessellate`, `egui/src/context.rs:2859`) with no panic. **(b) `interactive_controls_respond`**: every control the showcase advertises as interactive is found by its AccessKit label and clicked — `get_by_label(..)` from `kittest`'s `Queryable` (`kittest/src/query.rs:161-172`), which the harness implements (`egui_kittest/src/lib.rs:963-969`), then `Node::click` (`egui_kittest/src/node.rs:61`) — and its effect is asserted on the app through `Harness::state` (`egui_kittest/src/lib.rs:478`): the theme and mode pickers install the theme they name, the icon picker the icon set. **(c)** the command-line and icon-following rules of §10.4, as the iced showcase's own tests pin them (`the_command_line_rejects_what_it_cannot_honour` and `an_icon_choice_that_followed_the_preset_keeps_following_it`, `connectors/native-theme-iced/examples/showcase-iced.rs:7086`, `:7040`). **(d)** the chrome and Widget Info tests of §13.2 | The findings C7 exists for — a dead control, a page that does not lay out — were showcase bugs, not library bugs, so only a test inside the showcase catches them (`docs/archive/todo_v0.5.9_theme-contracts-rationale.md:525`). "Renders" stops before pixels, as rationale §8, Q-3 records: without its `wgpu` or `snapshot` feature `egui_kittest` has no renderer at all (`egui_kittest/src/renderer.rs:37-46`) and `Harness::render` does not exist (`egui_kittest/src/lib.rs:690-691`); layout, painting into shapes and tessellation — everything this crate's styles feed — do run. Three harness facts shape the tests: after the app's creation closure has installed the theme, the harness pins the theme preference (`egui_kittest/src/lib.rs:142`) and overwrites three style fields — cursor blink, scroll animation, `animation_time` (`egui_kittest/src/lib.rs:145-149`) — so no test compares a whole `Style` with the atlas; a single-node label query panics when two nodes match (`kittest/src/query.rs:65-70`), so a label the showcase repeats is queried with `query_all_by_label`; and `Harness::run` panics past its step limit (`egui_kittest/src/lib.rs:356`, default four steps at `egui_kittest/src/builder.rs:39`), so a page that keeps repainting — a spinner — is driven with `run_steps` (`egui_kittest/src/lib.rs:451`) |
| T12 | **Contrast invariant (C8)** | Over T10's 32 combinations, for every text-on-fill pair egui paints — each entry of the test's pair table cites the egui painting site that combines the two, e.g. a `TextEdit`'s text on `Visuals::text_edit_bg_color()`, which falls back to `extreme_bg_color` (`egui/src/widgets/text_edit/builder.rs:739`, `egui/src/style.rs:1152-1153`) — compute the WCAG contrast ratio of the connector's two colours and of the native pair `mapping.toml` names as their sources, compositing any colour with alpha below 1 over its own background first, and require the connector's ratio to be no lower. A pair whose fill is a §6.1 state layer over a translucent layer may fall below the native ratio by `composite_over`'s `u8` rounding alone (§7.2): such a pair is listed in the pair table's `exceptions` with the preset, the mode and that reason, and is not skipped: it is asserted against its rounding floor — the lowest native ratio over the composites whose red, green and blue each differ from the exact composite by at most one `u8` step — and both ratios are printed, with the floor; an excepted pair that no longer falls below the native ratio fails as a stale exception. **Asserted** where both colours come from one native widget's pair, because the connector chose both; **printed with both ratios, not asserted,** where egui pairs a text colour with a fill from another native source; and every pair below AA is printed, never asserted | The rule is no-degradation, not AA: of 512 pairs measured for v0.5.9, 174 sit below AA and are real platform data, and asserting AA would mean inventing values the platform did not give (`docs/archive/todo_v0.5.9_theme-contracts-rationale.md:526`). **No status contrast enforcement anywhere (C19):** `warn_fg_color` and `error_fg_color` (§14 item 15) carry the platform's own colours, printed like any pair whose fill egui chooses. The pair table, straight-alpha compositing and the ratio live in this crate's own `#[cfg(test)]` `contract` module, in the siblings' shape (`connectors/native-theme-iced/src/contract.rs:852-877`; rationale §8, Q-5): each row names the egui painting site and the native pair it is held to. Neither sibling's `contrast_ratio` — each `#[cfg(test)]` since C19 (`connectors/native-theme-gpui/src/derive.rs:60-61`, `connectors/native-theme-iced/src/extended.rs:52-53`) — is reachable from this crate, and neither composites alpha. The arithmetic is straight-alpha: an `egui::Color32` is gamma-space sRGBA with *premultiplied* alpha (`ecolor/src/color32.rs:8`), so it is unmultiplied at the boundary, before any blend |
| T13 | **Accessibility preferences** | Pure; no `Context`. For every bundled preset in both modes, build the atlas with `AccessibilityPreferences::default()` and again with a text-scaling factor other than `1.0` and `reduce_motion` set. **(a)** every `text_styles` size and every `override_font_id` size, in both base styles and every role style, equals `scaled_text_size` of the theme's size; **(b)** `style_diff` between the two atlases reports those sizes, `animation_time` and `scroll_animation`, and — where their values move — the two that follow the Body row height, `spacing.extra_text_line_spacing` (§6.15) and the `Role::Slider` cells' `expansion` (§6.6), and nothing else; **(c)** with `reduce_motion`, every style — role styles included — has `animation_time == 0.0` and `scroll_animation == ScrollAnimation::none()`, and without it both keep egui's defaults; **(d)** `scaled_text_size` returns the size unchanged for a factor of `0.0`, a negative one, `NaN` or `+∞`, as the iced connector's test pins its own (`connectors/native-theme-iced/src/lib.rs:996-1004`) | The atlas applies the preferences egui has a sink for (§4.3, §14 item 27), and each assertion guards one way that silently fails: (b) stops text scaling turning into a zoom of every length; (c) covers the role styles because `scroll_animation` is read from the `Ui`'s own style (`egui/src/ui.rs:1401`), so a scope whose style missed it would still animate; (d) keeps the three connectors' `scaled_text_size` one function in three places, as §4.7 promises |
| T14 | **Install, the plugin and the install sequence** | A bare `egui::Context` with `ctx.set_fonts(egui::FontDefinitions::empty())`, driven with `Context::run_ui`, whose passes call the plugins' begin- and end-pass hooks (`egui/src/context.rs:811-813`), while `begin_pass` and `end_pass` call `input_hook` and `output_hook` (`:966`, `:2464`). **(a) Install (§10.3).** Install, run three passes, and assert that both `Theme`s' base styles survive; then install an atlas of another preset and assert that both are now its; an install of an atlas whose plan found no face, after one that installed a face, leaves `FontFamily::Proportional`'s chain as `FontDefinitions::default()` has it; after `to_image_source` has stored a raster icon's `TextureHandle`, an `install` leaves none in `ctx.data`. **(b) Focus ring (§10.3, §6.18).** One `Button` in a `Window`: the pass's shapes hold no ring at rest and none after a primary click on the button; after a `Key::Tab` press they hold exactly one, on the button's layer (`Response::layer_id`), in `focus_ring_color` at `focus_ring_width` and offset by `focus_ring_offset`, with the base style's active corner radius; a following pass whose input has `RawInput::focused = false` holds none while egui still reports the button focused, and the next pass with `true` holds the ring again; for a `Button` inside a role scope whose active corner radius differs from the base style's (asserted first), the ring's corners follow the scope's, and so they do for a `Button` in a `Window` whose body took that role with `native_set_style`; a `Button` on the root `Ui` after `ui.native_set_style(role, ..)` there takes the role's corners, and after a following `ui.reset_style()` the base style's (§6.18's root rule); and for a widget whose code called `register_focus_shape(ctx, id, rect, corner_radius)` in the pass, the ring follows that rect and radius. **(c) The OS colour scheme (§10.3).** Under `ThemePreference::System`, with an atlas built with `Builder::os_mode(ColorMode::Light)`: input with `system_theme: None` gives `ctx.theme() == Theme::Light` in the first pass, where egui alone would fall back to `Theme::Dark` (`egui/src/memory/mod.rs:331`); input with `Some(Theme::Dark)` gives `Dark`; an atlas with no OS mode leaves egui's fallback. On the title bar, in `FullOutput::viewport_output` (`egui/src/data/output.rs:39`): with `None` in the input the first pass carries `SetTheme(SystemTheme::Light)` and no `SetTheme(SystemTheme::SystemDefault)`, the second carries no `SetTheme`, and after an atlas with `os_mode(ColorMode::Dark)` is installed the next pass carries exactly one `SetTheme(SystemTheme::Dark)`; a pinned preference is checked against an OS mode of the *other* scheme, both ways — after `ctx.set_theme(Theme::Light)` under that `Dark` OS mode egui's own `SetTheme(SystemTheme::Light)` passes through, after `ctx.set_theme(ThemePreference::System)` the pass carries the OS mode again, and after an atlas with `os_mode(ColorMode::Light)` is installed and `ctx.set_theme(Theme::Dark)` egui's own `SetTheme(SystemTheme::Dark)` passes through; with `Some(..)` in the input, egui's `SystemDefault` passes through unchanged; with `Options::sync_window_theme = false` no pass carries a `SetTheme`. **(d) Repaint (§10.3 step 6).** After passes are run until `ctx.has_requested_repaint()` (`egui/src/context.rs:1934`) is `false`, `install` makes it `true` | `FontDefinitions::empty()` binds both built-in families to empty vectors (`epaint/src/text/fonts.rs:561-570`) and `CachedFamily::new` early-returns on an empty list (`:640-648`), so neither the `:1025` nor the `:1033` panic can fire, and it saves a face parse per test — **valid only because of the never-`Name` invariant** (§8.1). Each assertion has the defect it catches: (a) an install that reaches only the active theme, or a second install that does not replace the first, or one that keeps the previous theme's icon textures (§10.3 step 5); (b) a ring painted for `clicked()` as well as for focus, or on the background layer, where a window would cover it, or in a window that does not have the keyboard, or with the base radius inside a scope, or a ring that looks for the root `Ui`'s record in the widget table, where the root's rect is still `Rect::NOTHING` during `on_end_pass` (§6.18); (c) a hook that overwrites a scheme the integration reported, a hook that rewrites a pinned preference's command to the OS mode — invisible when the two are the same scheme, hence the other one, a plugin that keeps reading the first atlas it saw — egui keeps the first plugin of a type and drops every later one (`context.rs:2045`, `egui/src/plugin.rs:206-210`) — and a title bar left to `SystemDefault`, which winit turns into a dark X11 hint on a light desktop (§10.3), and a `SetTheme` sent from `on_end_pass` instead of rewritten, which the first-pass assertion catches because egui's `SystemDefault` comes after it (`egui/src/context.rs:2456`); (d) an install that waits for the next input event |
| T15 | **The watcher's hand-off** | The step the watcher thread runs on each OS change, called directly on a bare `Context` whose repaint callback (`Context::set_request_repaint_callback`, `egui/src/context.rs:1961`) counts: **(a)** with a `rebuild` that returns an atlas of preset P, `take()` returns an atlas named P once and then `None`, `rebuild` ran once, and one repaint was requested; **(b)** with a `rebuild` that returns `Err`, `take()` returns `None` and `last_error()` returns its message; **(c)** `ThemeWatcher::start` on a runner with no desktop returns `Ok` or an error — never a panic — as native-theme's own test accepts (`native-theme/src/watch/mod.rs:313`) | The thread's work is `rebuild` plus a hand-off; the hand-off is the connector's and is tested here. What fires it per platform is native-theme's (§10.2) |
| T16 | **`Builder::style_patch`** | Pure. For every bundled preset in both modes, an atlas built with a patch that sets `Style::explanation_tooltips = true`, which the atlas never writes (§5.10), and one without: `style_diff` between the two reports that field in both base styles and in every role style in every variant, and nothing else; and a patch that writes `visuals.hyperlink_color`, which the atlas does write, leaves the patch's value in every style | The patch is applied last, to every style (§4.3): the first half proves "every", the second "last" |
| T17 | **Numeric sinks** | Pure. `convert::finite_or(x, b)` returns `x` for every finite `x` — `-0.0`, `f32::MIN_POSITIVE`, `±f32::MAX` among them — and `b` for `NaN`, `+∞` and `−∞`; `convert::clamp_length(v)` returns `v` for every `v` from `0.0` up to below `f32::MAX * egui::emath::GUI_ROUNDING` (`emath/src/gui_rounding.rs:18`, re-exported at `emath/src/lib.rs:47`), `0.0` for every negative `v` — `−f32::MAX` and `−∞` among them — and for `NaN`, and, for `f32::MAX` and `+∞`, a value whose `GuiRounding::round_ui` (`emath/src/gui_rounding.rs:58-59`) is finite; `to_margin` and `to_button_padding` given a `NaN` or `±∞` side, and `to_stroke` and `to_corner_radius` given a `NaN` or `±∞` width or radius, return `base`'s value there; `padding_with_border` adds the line width to each stated finite side, adds nothing for a `NaN` or `±∞` width, and keeps a `f32::MAX` side plus a width finite, so `to_button_padding` of a border with line width `1.0` and sides 5 / 6 gives `.y = 6.5`; and `convert::composite_over(layer, idle)` returns `to_color32(layer)` for an opaque `layer` and `to_color32(idle)` for a `layer` with alpha `0`, over every colour T10's 32 combinations produce and every hostile colour of T4 | `round_ui` divides by `GUI_ROUNDING` before it rounds, so a length above `f32::MAX * GUI_ROUNDING` becomes `+∞` on egui's layout paths (§6.7); T4 feeds hostile values through the whole atlas, and this pins the two helpers every sink goes through. The two `composite_over` identities are what §6.1 leans on: on every preset but windows-11 the button's layers are opaque, so compositing must return the layer exactly (C17) |
| T18 | **Accessors and constructors** | Pure except (f) and (h), which run passes on a bare `Context`, and (g), which reads the OS. **(a)** `ThemeAtlas::resolved_for(theme)` returns, for each `egui::Theme`, the `ResolvedTheme` the builder took for it, on an atlas built from two different presets, and `from_preset(n, ..)`'s `name()` equals `Theme::preset(n)`'s `name`; **(b)** `input_margin` over a border whose sides are stated, `None` and `NaN`: a stated side is that side plus `border.line_width`, and a `None` or non-finite side keeps `Margin::symmetric(4, 2)`'s (`egui/src/widgets/text_edit/builder.rs:136`); **(c)** `role_font_weight`, on a theme whose every font spec has its own weight: each `Role` returns the weight of the font §4.7's table names for it — `font` for the fourteen, `list.item_font`, `dialog.body_font`, `window.title_bar_font`, and `defaults.font` for the other eight; **(d)** `role_font_is_italic` is `true` exactly when the role's font is `Italic` or `Oblique` and `defaults.font` is `Normal`, and `false` whenever `defaults.font` is slanted; **(e)** at a text-scaling factor of `1.0` and of `2.0`, `text_role_font`, `text_role_line_height`, `text_role_weight`, `window_title_bar_font`, `window_title_bar_text_color` for `true` and `false`, `list_header_font`, `dialog_button_order` and `icons::icon_size` for each `IconContext` return their leaves as §4.7 states — every text size and line height `scaled_text_size` of its leaf, every `FontId` `Proportional`, and `icons::icon_size`, which takes no preferences, its leaf as stated; **(f)** `expander_icon`'s closure, called in a pass, paints in `expander.arrow_color` and leaves the `Ui` it was handed holding the `Arc<Style>` it held before (`Arc::ptr_eq`); **(g)** `to_egui_atlas` of `SystemTheme::from_system()`'s result, its public `mode`, `accessibility`, `layout` and `icon_set` first set to values the detection did not return, reports each — `os_mode()`, `accessibility()`, `layout.widget_gap` in `spacing.item_spacing` and `layout()` returns the layout it was given, `icon_set()` — with `name()`, `resolved_for` of both themes from `light` and `dark`, and `icon_theme` of both from `icon_theme_for`; with `system-fonts`, its `notes()` hold every `Note` that `fonts::font_definitions(&sys.light, &FontPlan::from_system(&sys.light))` returns; `from_system()` gives the same name, OS mode and `ResolvedTheme`s, or an error exactly when `SystemTheme::from_system()` does; and `to_theme(r, n)` equals `ThemeAtlas::builder(n, r, r).build()` style for style; **(h)** `input_frame`, over passes in the `Role::Input` scope round a `TextEdit` given the same id, before and after `Memory::request_focus` of that id (`egui/src/memory/mod.rs:918`): unfocused, the frame's stroke is the interaction state's `bg_stroke`; focused, `input.focus_border_color` at the state's `bg_stroke.width`, and `widgets.inactive.bg_stroke` with that leaf `None`; in the first pass of a field whose id was given `Memory::request_focus` before the field was first added, the frame is built from `widgets.active`, the entry egui's own frame takes there (`egui/src/style.rs:1276-1278`, `egui/src/widgets/text_edit/builder.rs:737`); and on a theme whose `input.focus_border_color` equals `input.selection_text_color` and whose `input.border.line_width` is `1.0`, egui's selection-stroke width, focused and unfocused, the shapes the field paints equal those of the same `TextEdit` given `.margin(input_margin(t))` and no frame — egui's own frame in every other field | An accessor with logic — a table, a rule, a formula, a restored style — and a constructor's plumbing fail by handing over a plausible wrong leaf, which nothing else notices: the value renders, only not the platform's. (a) is also the route of every per-call row whose application reads the leaf itself, which `tested_by` names (§13.1), and (e)–(f) and (h) the route of the rows whose accessor they call |

**The three runner checks.** They run in the egui legs of the screenshot
workflow, on `macos-latest` and `windows-latest`, because no
Linux runner can make them:

* **`system_faces_resolve`** — an `#[ignore]`d test behind `system-fonts`, run
  there with `--ignored`: `font_definitions` of `FontPlan::from_system`'s plan
  returns no `Note::FontFamilyUnavailable` over `SystemTheme::from_system()`'s resolved
  theme in both modes — the reader's path — nor over the platform's bundled
  preset resolved in both modes — `macos-sonoma` on macOS, whose family is
  "SF Pro" (`native-theme/src/presets/macos-sonoma.toml:47`, `:403`), and
  `windows-11` on Windows. The assertion covers `defaults.font` on both
  platforms and `defaults.mono_font` on Windows; for macOS's `mono_font`, "SF
  Mono", the test prints whether it resolves, without asserting, as the iced
  and gpui runner checks do, since no source states that any font database
  holds that name (§8.8, §15). On macOS it also prints, without asserting, how
  many faces the system UI font's file holds (`SystemFace::data`, the whole file
  for a collection) and the family each records (§15), through skrifa's re-export of
  read-fonts — `skrifa::raw::FileRef::new` over those bytes, `CollectionRef::len` and
  `get`, and `MetadataProvider::localized_strings` for the family (skrifa 0.44.0,
  `src/lib.rs` line 29 and `src/provider.rs` line 30; read-fonts 0.41.0, `src/lib.rs`
  lines 224–233, 271 and 281) — so the print adds no dev-dependency, `skrifa` being a direct one (§8.2). It is the evidence that the names the readers and
  the presets give resolve — by name through fontdb, and on macOS for the
  system UI font by file through Core Text (§8.2) — which nothing else in
  this design establishes. A failure is fixed at its root in native-theme —
  the name a reader reports, or `system_face`'s lookup — never by substituting
  another family.
* **`system_line_spacing`** — an `#[ignore]`d test behind `system-fonts`, run
  the same way, over the same two themes: it prints §6.15's `want − row` for
  the Body face `FontPlan::from_system` loads, and asserts that the atlas's
  built `extra_text_line_spacing` is T5's value for that face. The printed
  values are the macOS and Windows measurement §6.15 records; KDE and GNOME
  were measured on Linux (§6.15).
* **The macOS menu is installed.** Under `--screenshot` on macOS the showcase
  reads back `NSApplication::sharedApplication(mtm).mainMenu()` after
  `Menu::init_for_nsapp` (objc2-app-kit 0.3.2, src/generated/NSApplication.rs
  lines 488 and 703; `numberOfItems`, src/generated/NSMenu.rs line 269, all
  three safe methods) and exits with failure unless the main menu exists and
  its `numberOfItems()` equals the number of top-level menus the showcase
  built — so a menu that failed to install, or winit's default menu left in
  its place, fails the capture step.

**The sibling connectors' macOS font checks** (§8.8) run in the iced and gpui
legs of the same workflow, on `macos-latest`, in the iced showcase's tests and
the gpui showcase's capture run: the iced showcase's `macos-sonoma` capture
shows "SF Pro" in its inspector with no "not found"; an iced showcase runner
test asserts that `system_font_family` of `macos-sonoma`'s `defaults.font` is a
family in iced's font database (`font_system()`'s `db`), and another prints,
without asserting, whether the same holds for its `mono_font` ("SF Mono",
**UNVERIFIED**); and the gpui showcase's macOS `--screenshot` run logs whether
`cx.text_system().all_font_names()` holds the stated mono family "SF Mono"
(**UNVERIFIED**).

`mapping.toml` lives at `connectors/native-theme-egui/mapping.toml`, one row per
native leaf, carrying `verdict ∈ {direct, scoped, derived, unmappable}`, the
declared egui `sinks`, and — for `unmappable` — the upstream change that would
close it. It is **also rendered into the published rustdoc**, not merely kept
repo-local, so a reader who never opens the manifest can still name the loser of
every contest (§5.9).

**One table, read four ways.** C6 asks each connector for a mapping-contract
table; the siblings keep theirs as Rust rows in a `#[cfg(test)]` `contract`
module (`connectors/native-theme-iced/src/contract/rows.rs`). Here
`mapping.toml` *is* that table, and no second one is written: T2 reads it for
which sinks each leaf reaches, T3 for native-side completeness, T10 for the
value each sink holds and, as T10b, for egui-side completeness, and T12 for the
native source of each colour pair. Keeping the contract as data rather than
Rust rows is what lets it also be rendered into the rustdoc. The rest of the
contract — the 32 combinations, straight-alpha compositing and the contrast
ratio — is this crate's own `#[cfg(test)]` `contract` module, in the siblings'
shape (rationale §8, Q-5).

### 13.1 The `mapping.toml` schema

T2 and T3 both compare strings, so the two path spellings are part of the
contract rather than an implementation detail.

* **Leaf path (the table key).** Exactly the dotted path `serde_json` produces
  when walking `serde_json::to_value(&resolved)` — e.g.
  `"button.border.corner_radius"`, `"defaults.icon_sizes.toolbar"`. The four
  `LayoutTheme` leaves are not in that value (§4.3), so they are injected by T3
  under the reserved prefix `layout.`, giving `"layout.widget_gap"` and its three
  siblings. Two value shapes need a rule so that a path does not depend on the
  data. A `null` is a leaf: an unstated padding side is
  `"sidebar.border.padding.left"` whether it holds a number or not. And the walk
  stops at a `FontSize`: `defined_size` serializes externally tagged, as
  `{"Pt": <f32>}` or `{"Px": <f32>}` (`native-theme/src/model/font.rs:78-89`), so
  the key is `"button.font.defined_size"` and never `…defined_size.Pt`. Every
  key in the file other than the `[unwritten]` table must be one of those
  strings (§5.7 counts them), and every one of them must appear exactly once.
* **Sinks (`sinks`).** A list of inline tables `{ path, scope?, variant?,
  surface?, when? }`, one per field the leaf is written into, each naming its
  own location, because one leaf can land in several: a base owner (§5.9) is
  written into the base style **and** its own role's cell, often at different
  paths (`button.border.color` reaches `noninteractive` in «Button» but not on
  the base style, §5.3), and a `sidebar` leaf reaches the frames of both side
  panels. A leaf that feeds a formula lists the formula's outputs as well: a
  border's `line_width` the `spacing.button_padding` its role's padding rows
  write (§5 intro), an idle fill the hovered and pressed fills composited over
  it (§6.1), the leaf a `None` soft option falls back to the sink it then fills
  (§6.4), `slider.thumb_diameter` and `defaults.font.size` the slider's
  `expansion` (§6.6), and `defaults.font.size` the line spacing (§6.15).
* **Sink path (`path`).** Exactly the path that `style_diff` emits for a changed
  field, which is the field chain rooted at `Style` with the `Style.` prefix
  dropped: `visuals.widgets.hovered.weak_bg_fill`, `spacing.button_padding.x`,
  `text_styles[Body].size`. **`style_diff`'s spelling is normative**: the
  manifest never invents a path the helper cannot emit, and the `{5}` and
  `{inactive,hovered,active}` shorthands used in §5's prose are expanded to one
  concrete entry per state in the manifest. The helper splits a `Vec2`, `Stroke`,
  `Margin`, `Shadow` and `FontId` into their fields (a `Vec2` must be split: its
  `PartialEq` debug-asserts on `NaN`, `emath/src/vec2.rs:349-359`), keeps a
  `CornerRadius` one path, and spells a text style `text_styles[<TextStyle>]`.
* **Sink location.** With neither `scope` nor `surface` the sink is a field of
  the base style. `scope = "<Role::key()>"` makes it a field of that role's
  style, in the cell `variant = "<RoleVariant::key()>"` names — allowed only
  beside `scope`, for a `Selected` (§6.2) or `Disabled` (§6.3) cell; absent
  means `Normal`. `surface = "<Surface::key()>"`, never beside `scope`, makes
  it a field of that surface's `egui::Frame`, and the path is spelled against
  `egui::Frame` (`fill`, `stroke.color`, `inner_margin.left`, `corner_radius`,
  `shadow.blur`). T10 and the showcase's Widget Info read each sink at the location
  it names.
* **Inheritance.** A cell is built from another style (§3.4), so a sink is
  also carried by the cells built from its location: a base-style sink by
  every `Normal` cell that does not override the field, and a `Normal` cell's
  field — its own sink or one it carries — by that role's `Selected` and
  `Disabled` cells that do not override it. A cell overrides a field when the
  manifest declares a sink for it at that very cell **whose leaf the theme
  states** — a sink whose leaf is `None` (an unstated padding side, a `None`
  optional length) writes nothing, and the cell carries the value it is built
  from (§5 intro, §6.4) — or when §6 writes it there with no row:
  `disabled_alpha` in a `Disabled` cell (§6.3), the resting `weak_bg_fill` of
  the `Role::Expander` cells and its `open` copy (§6.1), and the fields
  `menu_style` sets in the `Role::Menu` cells (§3.4). Where a field is
  declared only by `when` sinks, or by a stated side beside an unstated one
  (§5's mean of two targets), the carried value can still show through: T2
  allows a change there and does not require one. A `Surface` frame's own
  sinks override its preset's field the same way. A `Surface` frame is
  built the same way, from its container's egui preset over the base style
  (§3.4), so it carries a base-style sink in every frame field its preset
  reads from that field (§3.2's table) and its surface declares no sink for:
  `Surface::CentralPanel`'s `fill` carries `visuals.panel_fill`, and
  `Surface::Window`'s `corner_radius` and `inner_margin` carry
  `visuals.window_corner_radius` and `spacing.window_margin`. T2 expects every location that carries a sink to change under at least
  one of its splices.
* **`when`.** Optional on a sink that another input of its formula can hold
  still: the condition, in words, under which a change of the leaf moves it.
  An opaque state layer makes the composite the layer itself whatever the idle
  fill (`ecolor/src/color32.rs:343-345`); a pair of padding sides both
  unstated keeps the starting style's component whatever the border's
  `line_width` (§5 intro), so a `line_width` row's padding sink carries this `when`
  and a padding side's own sink carries none; §6.15's floor holds the line
  spacing at `0.0`; a stated soft option leaves its fallback unused (§6.4);
  and the slider's `expansion` moves only where the Body row is taller than
  `1.25 · thumb_diameter` (§6.6). T2 accepts a `when` sink changed or
  unchanged; T10 checks its value.
* **`tested_by`.** In place of `sinks`, on a row whose leaf moves no `Style`
  or `Frame` field: a per-call route (§2's third kind of route — an accessor,
  or a `ResolvedTheme` leaf fed to the widget's own builder method), the
  install plugin's focus ring (`defaults.focus_ring_*`), a route through
  `FontDefinitions` and `FontTweak` (the `.family`, `.weight` and `.style` of
  `defaults.font` and `defaults.mono_font`, whose `text_styles` entry holds
  `FontFamily::Proportional` or `Monospace` whatever the leaf, §8.1). The value
  names the §13 test of the route: `"T14(b)"` for the ring; `"T6(d)"` for the
  two default fonts; for a per-call route,
  the T18 clause of the accessor its row names, or `"T18(a)"` where the
  application reads the leaf itself from `ThemeAtlas::resolved_for`. A row that
  is not `unmappable` carries exactly one of `sinks` and `tested_by` (T3), and
  T2 asserts that splicing a `tested_by` leaf changes nothing anywhere.
* **`probe`.** Optional. A literal value T2 splices in when no bundled preset
  pair differentiates the leaf. Its presence is what makes the no-skip rule of
  T2 enforceable.
* **`sub_tag` and `upstream`.** Required exactly when `verdict = "unmappable"`
  and forbidden otherwise. `sub_tag ∈ {egui-limited, widgets-crate,
  source-void, source-side gap}` per §2; `upstream` is the one-line change that would close
  the row, which Widget Info and the Theme Map page print (§10.4).
* **`exceptions`.** Optional, and forbidden on `unmappable` rows: a list of
  `[preset, reason]` pairs naming the presets on which T10 may find the sink
  different from the leaf, each with its reason. It is the only way T10's
  equality is relaxed.
* **`[unwritten]`.** One table outside the rows, keyed by sink path (spelled
  as a sink's `path` is, with a `Frame` field written `"<Surface::key()>.<field>"`),
  each value the reason egui's own value stands for that field. A path may not
  be both a declared sink and a key here; T10b requires every path to be one of
  the two.

```toml
["button.border.color"]
verdict = "direct"
sinks   = [
  { path = "visuals.widgets.inactive.bg_stroke.color" },
  { path = "visuals.widgets.hovered.bg_stroke.color" },
  { path = "visuals.widgets.active.bg_stroke.color" },
  { path = "visuals.widgets.open.bg_stroke.color" },
  { path = "visuals.widgets.noninteractive.bg_stroke.color", scope = "button" },
  { path = "visuals.widgets.inactive.bg_stroke.color", scope = "button" },
  { path = "visuals.widgets.hovered.bg_stroke.color", scope = "button" },
  { path = "visuals.widgets.active.bg_stroke.color", scope = "button" },
  { path = "visuals.widgets.open.bg_stroke.color", scope = "button" },
]

["slider.thumb_diameter"]
verdict = "derived"
sinks   = [
  { path = "spacing.interact_size.y", scope = "slider" },
  { path = "visuals.widgets.inactive.expansion", scope = "slider", when = "Body row > 1.25 · d, §6.6" },
  # … and the same `when` sink for noninteractive, hovered, active and open
]

["menu.font.weight"]
verdict   = "derived"
tested_by = "T18(c)"

["button.primary_background"]
verdict = "scoped"
sinks   = [{ path = "visuals.selection.bg_fill", scope = "button" }]

["checkbox.checked_background"]
verdict = "scoped"
sinks   = [
  { path = "visuals.widgets.noninteractive.bg_fill", scope = "checkbox", variant = "selected" },
  # … and the same sink for inactive, hovered, active and open
]

["sidebar.border.padding.left"]
verdict = "scoped"
sinks   = [
  { path = "inner_margin.left", surface = "panel_left" },
  { path = "inner_margin.left", surface = "panel_right" },
]

["slider.tick_mark_length"]
verdict  = "unmappable"
sub_tag  = "egui-limited"
upstream = "egui: tick marks on Slider"

["layout.widget_gap"]
verdict = "derived"
sinks   = [{ path = "spacing.item_spacing.x" }, { path = "spacing.item_spacing.y" }]
probe   = 11.0
```

### 13.2 The showcase's chrome and Widget Info tests

T11(d), in the showcase's own `#[cfg(test)]` module and driven as T11 is.
`Node::hover` (`egui_kittest/src/node.rs:56`) and `Harness::hover_at`
(`egui_kittest/src/lib.rs:621`) move the pointer, `Harness::key_press_modifiers`
(`:616`) presses a shortcut, and a node is found by role with `query_all`
(`kittest/src/query.rs:149-158`) over `By::role` (`kittest/src/filter.rs:119`).
The harness advances a quarter second per pass by default
(`egui_kittest/src/builder.rs:40`), exactly `INFO_SETTLE`, so one default pass
can already complete a settle and a test that must stay inside it builds with
`with_step_dt` (`:118`); `Harness::run` stops once no *immediate* repaint is
requested (`egui_kittest/src/lib.rs:373-374`), so a settle, which
`request_repaint_after` schedules, is driven with `run_steps`. The harness's
window is 800 × 600 (`egui_kittest/src/builder.rs:34`), so a node below the
fold is brought into view with `Node::scroll_to_me`
(`egui_kittest/src/node.rs:152`) before it is hovered. A node's AccessKit
bounds are its untransformed `Response::rect` (`egui/src/response.rs:912-917`;
egui transforms only the root, by the pixels-per-point,
`egui/src/context.rs:526`), so a widget inside a `Scene` is hovered with
`Harness::hover_at` at its centre mapped through
`Context::layer_transform_to_global` (`egui/src/context.rs:3078`), not with
`Node::hover`. A menu item's AccessKit label is its text and its shortcut
text joined by a space (`egui/src/atomics/atoms.rs:51-62`). Every gate names its
discrimination proof, the seeded defect that must make it fail, as the v0.5.9
showcase gates did (`docs/archive/todo_v0.5.9_widget-info-spec.md` §0.2).

| Test | What it asserts | Discrimination proof |
|---|---|---|
| `the_window_asks_for_the_os_frame` | the `NativeOptions` the showcase's own constructor returns — `build_eframe` takes none, so `main` and the test call the same function — has `viewport.decorations == Some(true)` and the title `WINDOW_TITLE`, which contains `env!("CARGO_PKG_VERSION")` | `with_decorations(false)`; a title without the version |
| `the_chrome_is_where_the_layout_puts_it` | the top panel holds the menu row above the toolbar; the side panel holds the Theme, Mode and Icon theme rows, a separator, then the inspector's tabs; the status bar is the window's bottom; the page tabs sit above the page, and clicking one shows its page | the toolbar drawn above the menu row |
| `the_side_panel_toggle_hides_and_shows_it` | the status-bar toggle, View > Toggle Side Panel and Ctrl+B each hide and show the side panel, and a dragged width survives hiding | a toggle that recreates the panel at `LEFT_PANEL_WIDTH` |
| `the_menus_run_their_actions` | every menu item's action takes effect on the app (`Harness::state`) and every shortcut label is `Context::format_shortcut`'s | one item wired to another's action |
| `the_command_palette_runs_what_it_lists` | Ctrl+K opens it; typing filters the rows; a row switches the page, installs the preset or sets the mode it names; Escape clears the query, then closes | a palette that closes on the first Escape |
| `every_widget_reports_itself` | for every page in both `egui::Theme`s, every AccessKit node outside the inspector's content whose role is one egui gives a widget — the `WidgetType` roles of `egui/src/response.rs:935-957` less `Pane`, `Window`, `RadioGroup` and `Unknown`, found with `By::predicate` (`kittest/src/filter.rs:113`) — scrolled into view, hovered at its centre and settled, shows an info that is the node's own (the record's `Id` gives the node's id through `Id::accesskit_id`, `egui/src/id.rs:103`) or that of a non-container widget whose rect contains the node's — a container being one of the 21 container types of §10.4's palette or a chrome panel; a node egui creates inside a `Window` or a `ScrollArea` — the title bar's collapse and close buttons (`egui/src/containers/window.rs:1377-1388`, `:1466-1470`), its resize handles (`:1085`) and the scroll bars (`egui/src/containers/scroll_area.rs:1334-1336`), whose ids no helper sees — shows the info of the innermost record whose rect contains it, container or not (a `ScrollArea` returns no `Response` to record, `egui/src/containers/scroll_area.rs:89-104`; its bars are AccessKit `ScrollBar` nodes, `:1336`, `egui/src/response.rs:955`) | a `ui.button("probe")` on a page, drawn outside `demo`, bare and again inside a `Surface::Card` frame |
| `no_id_is_recorded_twice` | the registry notes an `Id` recorded twice in one pass, and every pass a test runs asserts that none was | two helpers given one id salt |
| `the_innermost_hovered_target_wins` | the pointer over a button inside the overlays page's `Modal` shows the button, and over the modal's frame beside it `Surface::Dialog`; over a checkbox in a `Surface::Card` frame, the checkbox; over a widget inside the `Scene`, that widget | the rule changed to the largest area, or to the last recorded |
| `instances_are_distinct` | an unchecked and a checked checkbox, and an enabled and a disabled button, show different variants and different values where the theme states different ones | a helper that records a fixed `RoleVariant` |
| `leaving_every_target_keeps_what_is_shown` | moving into the inspector's content, where the side panel's record contains the pointer, and the pointer leaving the window (`Harness::remove_cursor`, `egui_kittest/src/lib.rs:652`) each leave the shown info in place | the inspector's content not held, so the side panel's record wins there |
| `crossing_is_not_hovering` | a target hovered for less than `INFO_SETTLE` never replaces what is shown, and one hovered longer does, with no further input; every pass while the choice waits out `INFO_SETTLE` asks for a repaint no later than `INFO_SETTLE` (the root `ViewportOutput::repaint_delay`, `egui/src/viewport.rs:1278`), since the harness steps whether or not one was asked for | `INFO_SETTLE` set to zero; every `request_repaint_after` call of the choice removed |
| `a_target_no_longer_drawn_never_wins` | after a page switch, and after a popup closes, none of its instances is shown, even where its rect from the previous pass contains the pointer | a registry not cleared between passes |
| `the_info_is_the_manifest` | for every `Role` in each `RoleVariant` (`RoleVariant::all()`), every `Surface` and the base style, over the 32 combinations (`Theme::list_presets()` in both modes, enumerated in the test): the rows an info lists are exactly the ones §10.4's rule picks, computed in the test from the parsed manifest and not through the info module; there is at least one; every value printed is the one at that leaf of `serde_json::to_value` of the installed resolved theme; and every `unmappable` row appears under its role, its surface or the base style, with its `sub_tag` and its `upstream` text. After a preset switch the shown info prints the new preset's values | the `unmappable` rows filtered out; a row printed without `upstream`; the base style's `defaults.` and `text_scale.` rule removed; values cached across installs |
| `every_seam_is_recorded` | lexical, over the showcase's source with comments and string and char literals blanked, as the gpui detector does (`connectors/native-theme-gpui/src/showcase.rs:833`): the rule "A seam is applied only where it is recorded" of §10.4; it finds at least one seam call in `demo.rs` and one in `chrome.rs`, so it cannot pass by reading nothing | a `ui.native_scope(Role::Button, ..)` seeded in a page |
| `every_role_and_surface_is_demonstrated` | across the pages and the chrome, the records cover `Role::all()`, each role in every variant it carries (§6.2, §6.3), and `Surface::all()`; the menus' chrome counts under `Role::Menu`, the seam that carries it (§4.4) | the nested right panel removed from the Containers page |
| `the_chrome_reports_itself` | the menu bar, the toolbar, the side panel, the status bar, both tab rows and the central panel each show their own info, and the status bar names the shown info's title | the toolbar row drawn without its helper |
| `the_showcase_hardcodes_no_style_values` | lexical, over the showcase's source with comments and string and char literals blanked, as the gpui detector does (`connectors/native-theme-gpui/src/showcase.rs:833`) and `every_seam_is_recorded` reads it: no numeric literal is an argument of a call that sets a size, gap, margin, radius, stroke or colour. The calls are exactly: the `Ui` methods `add_space`, `set_min_size`, `set_max_size`, `set_width`, `set_min_width`, `set_max_width`, `set_height`, `set_min_height`, `set_max_height`, `set_width_range`, `set_height_range` (`egui/src/ui.rs:1675`, `:711-822`); the builder methods `default_size`, `default_width`, `default_height`, `min_size`, `max_size`, `exact_size`, `fixed_size`, `min_width`, `max_width`, `min_height`, `max_height`, `desired_width`, `desired_height`, `fit_to_exact_size`, `inner_margin`, `outer_margin`, `corner_radius`, `stroke`, `size` and `spacing`; and the constructors `vec2`, `Vec2::new`, `Vec2::splat`, `Margin::`, `CornerRadius::`, `Stroke::new`, `Color32::from_`, `FontId::new`, `FontId::proportional` and `FontId::monospace`. Exempt are the three named constants of §10.4 and the sites on an `ALLOWED_STYLE_LITERALS` list, keyed by the enclosing `fn` with its reason, for a value that is the datum on display rather than style — the colour a Colour-page editor starts from, the detector's own sample source, the `0.5` of `Vec2::splat(0.5)`, the centre `Image::rotate` turns the spinning indicator about (§4.10) — as the gpui showcase's list is (`connectors/native-theme-gpui/src/showcase.rs:3003`). The detector finds at least one such call, so it cannot pass by reading nothing; the twin of the gpui showcase's gate of the same name (`connectors/native-theme-gpui/src/showcase.rs:3094`) | a `ui.add_space(8.0)` on a page |

**What this replaces in the gpui showcase.** gpui's Widget Info is written by
hand, each colour claim citing the upstream line it was read at, and six
gates keep that prose true (`docs/archive/todo_v0.5.9_showcase-app-spec.md`
§10.2): `every_colour_claim_is_read_at_the_line_it_cites`,
`every_prose_citation_still_exists`, the omission report,
`every_geometry_builder_has_a_note`, `native_info_names_the_builder_it_applies`
and `every_recorded_builder_has_a_note`. None needs an egui twin but the fifth.
A claim here is a `mapping.toml` row, whose sinks T2 proves, whose leaf T3
proves present exactly once, and whose value T10 proves in all 32
combinations; the omission report's question — what a panel leaves out —
cannot arise, because an info lists every row of its seams; and the geometry
notes have no counterpart, because a seam is the manifest's own key.
`every_seam_is_recorded` is the twin of
`native_info_names_the_builder_it_applies`. `every_widget_reports_itself`,
`no_id_is_recorded_twice` and the resolution tests mirror gpui's §10.1 and
§10.3. `the_info_is_the_manifest` and `every_role_and_surface_is_demonstrated`
are new: the first because the info is generated, the second because the
chrome is meant to exercise every seam. Of the
gates the gpui showcase kept from before its Widget Info (showcase-app spec
§10.4), `the_showcase_hardcodes_no_style_values` has its twin above and the
coverage script is the implementation plan's.

---

## 14 -- Limits: what this connector does NOT do

This is the honesty ledger, in full. Every row states what is lost, cites the
evidence, and names the upstream change that would close it. Nothing here is
softened, and nothing here is a promise.

### 14.1 The ledger

| # | What is lost | Evidence | Upstream change that would fix it |
|---|---|---|---|
| 1 | **No widget-type axis in `Style`.** 25 native widget structs share one `Visuals.widgets` with five entries of six fields each | `style.rs:1250-1269`; `WidgetVisuals` `:1295-1319`; `WidgetState` has 4 variants (`widget_style.rs:84-90`); `Style::widget_style`, `button_style`, `checkbox_style`, `label_style` and `separator_style` are inherent methods (`widget_style.rs:120`, `:146`, `:174`, `:194`, `:212`) and `_classes` is ignored at `:120` and `:212` | `Style::class_overrides` — rationale §4.2 |
| 2 | **`Area`-based containers ignore the calling `Ui`'s style.** "Wrap it in a scope" is false for exactly the containers where a distinct look is most expected | `containers/area.rs:611-629` builds with a bare `UiBuilder::new()`; `ui.rs:136` falls back to `ctx.global_style()`. **Connector-side, and it makes the loss partial rather than total:** a role scope applied as the *first statement inside* the container's closure does reach every widget the application adds there (§1.5) — one line of application code, not an upstream change. What stays lost even then is enumerated in item 2b | a `style` on the `UiBuilder` inside `Area::Prepared::content_ui`, or an `Area::style` |
| 2b | **A `Window`'s self-painted parts take the base style regardless.** The irreducible residue of item 2: even with a role scope inside the closure and `surface_frame` / `title_frame` on the chrome, these read the `Area` content `Ui` — or the `Context` — *outside* `add_contents` | the title bar's button size (`containers/window.rs:1307`), the heading row height that sizes those buttons (`:1312-1313`) and its active fill (`:1427`); the window's own `ScrollArea`, which wraps `add_contents` from outside (`:738-742`) — its handles take the base style's thumb colours (§5.9), so only the `Role::Scrollbar` radius, `defaults.border.corner_radius` (§6.8), where it differs from `button.border.corner_radius`, is lost there; the resize corner (`:766-772`, painted at `:850-859`); and the resize grab radii, read straight from `ctx.global_style()` (`:1095-1098`). The title **text** is the exception: `Window::new` takes `impl IntoAtoms` (`:102`) and an explicit `RichText::font` overrides `AtomLayout::fallback_font(TextStyle::Heading)` (`:1350`), because the explicit family and size are applied after the fallback resolves (`widget_text.rs:425-436`) | the same `Area::style` as item 2. Regression-tested by §13 T8(b) |
| 3 | **`Response::on_hover_text` tooltips are unthemable.** `Role::Tooltip` and `Surface::Tooltip` are inert on the idiomatic path | `on_hover_text` shows its tooltip through `Tooltip::for_enabled` (`response.rs:665-666`), which builds on `Tooltip::for_widget` (`containers/tooltip.rs:39-50`, `:53-59`); that reads `response.ctx.global_style()` at `:43`, and neither takes a `Frame` or a `StyleModifier` | a `Tooltip::style` / `Tooltip::frame`, or `Response::on_hover_text_styled`. **Escape hatch that exists today**: `Tooltip::popup` is a public field (`tooltip.rs:9`), so `let mut t = Tooltip::for_enabled(&r); t.popup = t.popup.frame(f).style(m); t.show(..)` works on the manual path — `for_enabled`, not `for_widget`, which is open whenever it is called (`:38-39`) |
| 4 | **`Separator` spacing is hardcoded `6.0`** — the space a separator occupies is not themable at all | `widget_style.rs:212-217`, `spacing: 6.0` at `:215`; overridable only per instance via `Separator::spacing` (`widgets/separator.rs:45`) | `separator_style` reading spacing from the resolved `WidgetStyle` |
| 5 | **A focused widget is drawn in its pressed look, and its ring is a rounded rectangle.** The ring itself reaches the screen: the install plugin paints `defaults.focus_ring_color`, `focus_ring_width` and `focus_ring_offset` (§10.3, §6.18), around the widget's rect with the corners of the role scope around it. An outline of another shape is the widget code's to register (`register_focus_shape`, §4.3) | focus has no widget state of its own: `Response::widget_state` and `Widgets::style` pick `Active`, the pressed look, for `has_focus()` (`widget_style.rs:107-109`, `style.rs:1273-1282`) | a focused state apart from `active`, and a `Visuals::focus_stroke` with an offset that each widget paints in its own shape (rationale §4.2) |
| 6 | **No disabled colour, only a disabled alpha** | `Visuals::disabled_alpha` (`style.rs:1126`), applied by `Ui::disable` → `Painter::multiply_opacity` (`ui.rs:497-502`, `painter.rs:100-104`) | a sixth `Widgets` entry, or a `WidgetState::Disabled`. Partially worked around by `RoleVariant::Disabled` (§6.3) — but **only for widgets the application scopes** |
| 7 | **Font weight and style: one per family.** `defaults.font.weight`, `defaults.mono_font.weight`, 4 `text_scale.*.weight` and 19 per-widget `font.weight`, and the 19 per-widget `font.style` (italic) leaves — bold headings against a regular body are not carried by the theme | `FontId` is `{size, family}` (`epaint/src/text/fonts.rs:21-28`) with upstream's own `TODO(emilk): weight (bold), italics` at `:27`; family is `FontId`'s only selector | a `weight` (and `slant`) field on `FontId`. Until then exposed as `role_font_weight` and `role_font_is_italic` (§4.7) |
| 8 | **A typeface by name only for the two default fonts, and only where the system has that family.** With `system-fonts` (on by default, §12.1) `FontPlan::from_system` loads `defaults.font` and `defaults.mono_font` by family name through `native_theme::fonts::system_face`, the named family only, compared case-insensitively (§8.2); still lost are the 19 per-widget `font.family` leaves wherever one differs from `defaults.font.family` — on no bundled preset, but wherever a live reader states a menu, toolbar, status-bar, tooltip or title-bar family of its own (§5.8 item 7) — a family the system lacks (`Note::FontFamilyUnavailable`, egui's own face in its place), and every family without the feature | `FontData { font: Cow<'static, [u8]>, .. }` (`epaint/src/text/fonts.rs:112-122`); the `String` keys in `FontDefinitions` (`:431-444`) are arbitrary labels; no font database anywhere in egui, epaint or eframe; a per-widget family would need a `FontFamily::Name`, which this crate never emits (§8.1, §5.8 item 7) | a font-discovery layer in eframe, or an epaint hook, and a `FontId` that can name a family without a registered `Name` key |
| 9 | **Line height is additive, single and only partially wired.** `RichText`, `LayoutJob` and pre-built `Galley` ignore it, and the four `text_scale.*.line_height` leaves are lost: one `f32` serves every `TextStyle` | `Spacing::extra_text_line_spacing` (`style.rs:424`) read at exactly two sites: `widget_text.rs:775-776` (the `Text` arm only) and `widgets/text_edit/builder.rs:489-490` | applying it in `RichText::into_layout_job`, or a multiplicative `line_height_factor` |
| 10 | **Shadow geometry is egui's, not the platform's** | native-theme carries only `shadow_enabled: bool` (`native-theme/src/model/border.rs:224`); `epaint::Shadow` needs `offset: [i8;2]` (`shadow.rs:15`), `blur: u8` (`:20`), `spread: u8` (`:23`) | **native-theme-side**: add shadow offset, blur and spread to `DefaultsBorderSpec` and `WidgetBorderSpec` (`native-theme/src/model/border.rs:24`, `:81`) |
| 11 | **`slider.thumb_diameter` has no `Style` sink** | the handle radius is derived locally from the slider's **allocated rect** as `rect.height() / 2.5` (horizontal) or `rect.width() / 2.5` (vertical) — not from the rail rect (`widgets/slider.rs:853-857`, `:880-886`); `HandleShape` (`style.rs:1235-1243`) only scales it. The distinction is load-bearing: §6.6's workaround moves `spacing.interact_size.y` precisely because the radius keys off the widget rect | a `Spacing::slider_handle_radius`. Worked around by §6.6, at the cost of also resizing a `Slider::show_value` `DragValue` |
| 12 | **`spinner.stroke_width`; and the ring is drawn one point short of the size the theme states** (`spinner.diameter` and `min_diameter` are DERIVED onto `interact_size.y`, §5.5, §6.7) | `egui::Spinner` exposes `.size` (`spinner.rs:25`) and `.color` (`:32`) only; its radius inset, point count and `Stroke::new(3.0, ..)` are hardcoded (`:45`, `:46`, `:58`), so the visible outer diameter is the allocated size less `1.0` (§5.5) | a `Spinner::stroke_width` builder plus a `Style` fallback |
| 13 | **No switch widget at all.** 3 of the 13 `ResolvedSwitchTheme` leaves are UNMAPPABLE (§5.3); the rest reach §5.3's substitute, `Button::new(..).selected(checked)` in the `Role::Switch` scope, which paints a track in both states as a switch does, but no thumb, no travel and no switch geometry | no switch or toggle module exists under `egui/src/widgets/`; `Ui::toggle_value` (`ui.rs:1870-1882`) is a `Ui::selectable_label`, i.e. a `Button::selectable`, which frames an unselected button only when it is not inactive (`widgets/button.rs:78-83`, `:364-368`), so at rest it paints no track when off — which is why the substitute is not `toggle_value` | an `egui::Switch` widget. Until then the application reads `switch.*` from the `ResolvedTheme`; the companion crate `native-theme-egui-widgets` plans a `Switch` (`docs/todo_egui-widgets-spec.md`) |
| 14 | **No `Style` field for link states or visits**: every link colour, background and resting underline goes per call (§5.3); egui's own hover underline stays `hyperlink_color` | one `Visuals::hyperlink_color` (`style.rs:1036`) and no visited state; the underline is `Stroke::new(visuals.fg_stroke.width, color)` gated on hover-or-focus (`widgets/hyperlink.rs:50-54`), so an underline at rest is unreachable; the text colour read at `:47` is unconditional; `Link` paints only a `TextShape` (`:62-64`), no fill | a `Visuals::hyperlink_visited_color`, a visited-URL set in `Memory`, and a state-dependent link colour. Until then the link's colours, its resting underline and its backgrounds go per call in the link text's own format (§5.3) |
| 15 | **No `Style` field for success or info, and no status background: per call only** — `defaults.{success,info}_color` and the four `*_text_color`s | `Visuals` models exactly two: `warn_fg_color` (`style.rs:1056`) and `error_fg_color` (`:1059`) | a `Visuals::status` block. On macOS, KDE and GNOME the four `*_text_color`s are provably the body foreground (`docs/platform-facts.md:1089-1098`, `native-theme/src/macos.rs:98-104`), so only Windows loses a genuinely distinct value |
| 16 | **No unfocused-window field in `Visuals`**: the inactive title text goes per call (§5.2); the unfocused selection fill waits on item 45 — `window.inactive_title_bar_text_color`, whose title text is `visuals.text_color()` whatever the focus (`atomics/atom_layout.rs:300-301`), and `defaults.selection_inactive_background` | none of `Visuals`' 36 fields (`style.rs:989-1126`) and none of the five `Widgets` entries (`:1255-1269`) means "the window lost focus"; `widgets.open` means the *active* window title bar (`containers/window.rs:1427`) — the opposite. On macOS the live reader supplies a distinct selection fill (`native-theme/src/macos.rs:76`, `:107`); it is lost for want of its text colour, not of this field — the install plugin reads the window's focus (item 45) | a `Visuals::selection_inactive` consulted when `ctx.input(\|i\| !i.focused)`. Until then the title text colour of either state is exposed as `window_title_bar_text_color` (§4.7), and the connector's own swap of the selection fill waits on item 45 |
| 17 | **Five `Style` knobs are inert**, so the crate must not claim to theme them | `Spacing::menu_width` (`style.rs:453`), `Spacing::menu_spacing` (`:456`), `Style::compact_menu_style` (`:341`) and `Visuals::window_highlight_topmost` (`:1067`) have **no functional reader anywhere in egui 0.36.2** — besides the declaration and the default, each occurs only in the settings UI, which destructures it and offers a widget for it (`:1962`/`:2032`, `:1963`/`:2037`, `:1805`/`:1911`, `:2302`/`:2476`); `Visuals::clip_rect_margin` (`:1086-1087`) is `#[deprecated]` and documented "Setting it now has no effect". §5.10 lists the same five | wire them, or delete them |
| 18 | **Container frames are values, not lookups** — panel, group and canvas inner margins cannot be themed globally, so `layout.container_margin` (the group's `6`) reaches only a `Frame` the application builds, and `layout.window_margin` (the central panel's `8`; `Spacing::window_margin` feeds only a floating `Window`) only a panel shown with `Surface::CentralPanel`'s frame; a stock `CentralPanel` or `Frame::group` keeps egui's value | `Frame::group` `.inner_margin(6)` (`frame.rs:180`), `Frame::side_top_panel` `Margin::symmetric(8, 2)` (`:187`), `Frame::central_panel` `.inner_margin(8)` (`:192`), `Frame::canvas` `.inner_margin(2)` (`:229`) and `Frame::dark_canvas` (`:236-237`, via `canvas`) read no `Style::spacing` | those five presets reading `Spacing`. Worked around by `Surface`-supplied `Frame`s — `Surface::CentralPanel` takes `layout.window_margin` as its inner margin (§4.4) — which only help callers who pass them |
| 19 | **Sub-point precision and large magnitudes are lost at the epaint boundary** | `Margin` is four `i8` (`epaint/src/margin.rs:15-20`); `CornerRadius` four `u8` (`corner_radius.rs:13-25`); `Shadow::offset` `[i8;2]`, `blur`/`spread` `u8` | `MarginF32` / `CornerRadiusF32` in `Style`. Corner-radius saturation is benign (the tessellator re-clamps to half the smaller side, `epaint/src/tessellator.rs:638-642`); **margin saturation is a real loss** and emits `Note::ValueSaturated` |
| 20 | **An intra-widget contest that no scoping mechanism can resolve** | `slider.track_color` vs `slider.thumb_color` — the rail is hard-wired to `inactive` (`widgets/slider.rs:772-775`), so the resting handle shows the rail's colour. The others that stood here are resolved per call: `input.selection_text_color` vs `input.focus_border_color` on `visuals.selection.stroke.color` (`text_selection/visuals.rs:40` vs `widgets/text_edit/builder.rs:742-747`) by `input_frame` (§4.7); `expander.font.color` vs `expander.arrow_color` on one `fg_stroke.color` (`collapsing_header.rs:353` vs `:598`) by `expander_icon` (§4.7, §6.19); and `list.border.color` vs `list.grid_color` on one `noninteractive.bg_stroke` (`frame.rs:182` vs `egui_extras/src/table.rs:897-900`) by the application's list `Frame` (§5.4) | per-widget style structs upstream. Locked resolutions in §5.11 |
| 22 | **Fidelity is opt-in, and the failure is silent and non-uniform.** An application that calls `install()` and nothing else gets the rows §5.7 grades DIRECT — each contested field's base owner wherever its one global write renders it (§2) — and the DERIVED values the base style carries; every SCOPED row waits for a role scope or a surface frame | a `Style` reaches a `Ui` through three seams only — `Options::{dark_style,light_style}` (`memory/mod.rs:196`, `:200`), `UiBuilder::style` with `Ui::set_style` (`ui_builder.rs:28`, `ui.rs:387`), and `StyleModifier` (`style.rs:194`) — and none of them, nor any `Plugin` hook (`plugin.rs:13-52`), is handed a widget's kind; the census that finds no fourth is rationale §3.3. Items 1 and 2 are the two consequences | rationale §4.2's class overrides. Until then the README's first paragraph is §0.1's sentence, never "full theme geometry" |
| 26 | **`egui::Button` does not honour `TextStyle::Button`** | `Button::new` sets `.fallback_font(TextStyle::Button)` (`widgets/button.rs:49`) and `atom_ui` overwrites it at `:358-360` with `Style::button_style`'s font, which is `override_font_id.unwrap_or_else(\|\| TextStyle::Body.resolve(self))` (`widget_style.rs:122`, `:137`, `:148`, `:168`) — a hardcoded `TextStyle::Body` whenever `override_font_id` is `None`, which is egui's default (`style.rs:1430`). The overwrite is not what makes `override_font_id` win: `FontSelection::resolve_with_fallback` consults it ahead of any fallback anyway (`style.rs:158-168`, called at `widget_text.rs:774`) | `Style::button_style` resolving `TextStyle::Button` instead of inheriting `widget_style`'s `Body` fallback — the same shape of one-line fix as row 4's `separator_style`, in the same frozen module rationale §4.2 targets. Recorded because the assumption is natural and wrong, and because `text_styles[Button]` is still written for `ComboBox`, `ProgressBar`, `CollapsingHeader` and `drag_value_text_style` |
| 27 | **`accessibility.reduce_transparency` and `high_contrast` are not applied.** Text scaling and reduced motion are: `Builder::accessibility` scales every text size the atlas writes by `scaled_text_size` and, under `reduce_motion`, sets `animation_time = 0.0` and `scroll_animation = ScrollAnimation::none()` in every style (§4.3; decided by the native-look goal, rationale §8, Q-7) | egui has no `Style` sink for either. Its one translucent overlay a theme would reach, the `Modal` backdrop, is a per-call `Modal::backdrop_color` (`egui/src/containers/modal.rs:62`, default `Color32::from_black_alpha(100)` at `:29`), not a `Style` field — where the gpui connector drops its modal scrim under `reduce_transparency` (`connectors/native-theme-gpui/src/colors.rs:514-523`) | a `Style`-level backdrop colour. Until then both are exposed as `is_reduced_transparency(&SystemTheme)` and `is_high_contrast(&SystemTheme)` (§4.7), and the backdrop is the application's choice at the call site |
| 28 | **No widget is shipped** | — | a deliberate charter, §14.3 |
| 30 | **egui panics on a missing `TextStyle::Name` key, in every build.** Nothing is lost through this crate: `install` copies an application's `Name` keys into each base style (§10.3 step 2), `native_scope` and `native_set_style` merge the keys the parent `Ui` has, and `role_modifier` carries the keys of the style it is handed (§4.5, §7.5); the atlas's raw `Arc<Style>` cells, which carry none, are crate-private. Recorded so that no seam added later forgets the merge | every `Style` the connector builds starts from `Theme::default_style()` (`egui/src/memory/theme.rs:24-29`), whose `text_styles` are exactly the five stock keys (`style.rs:1433`, `:1417-1423`); `set_style_of` assigns outright (`context.rs:2250-2255`) and a child `Ui`'s style replaces its parent's (`ui.rs:237`); `TextStyle::resolve` panics on a missing key with no `cfg(debug_assertions)` (`style.rs:112-120`). §13 T8(a) tests every seam | a `Style::text_styles` lookup that falls back instead of panicking |
| 31 | **A frameless reader's hover sits on the button's idle fill, not on its backdrop.** `ui.selectable_label`, `ui.selectable_value` and menu entries show the base style's hover fill with nothing under it at rest, yet that fill is `button.hover_background` composited over `button.background_color` (§6.1, C17) | egui has one `hovered.weak_bg_fill` for framed and frameless readers: `Button::selectable` frames an unselected button only when not inactive (`egui/src/widgets/button.rs:78-83`, `:364-368`), which is what `Ui::selectable_label` and `Ui::selectable_value` build (`egui/src/ui.rs:1929`, `:1939`), and `menu_style` makes the inactive fill `TRANSPARENT` (`egui/src/containers/menu.rs:27`). gpui keeps a raw copy of the layer for exactly those readers (`docs/archive/todo_v0.5.9_theme-contracts-rationale.md:436-440`); egui has no second slot. It differs from the platform only where the layer is translucent — for the button, windows-11 alone — and not inside `Role::Menu`, whose row highlight is written as given | a hover fill of its own for frameless readers, or `frame_when_inactive(false)` widgets reading an uncomposited layer |
| 32 | **An unequal padding pair reaches `Spacing::button_padding` as its mean — a centring loss of `\|a − b\| / 2`.** `button`, `segmented_control`, `tab` and `combo_box` share `.x` between left and right and `.y` between top and bottom; the expander shares `.y`. §5's pair rule writes the mean of the two targets plus the border's line width (an unstated side targeting egui's own `vec2(4.0, 1.0)` component, `style.rs:1458`), which keeps the native outer size and moves the content off-centre by half the difference: `windows-11` button top 5 / bottom 6 (`native-theme/src/presets/windows-11.toml:98-99`) inside a 1-point border (`:43`) → `6.5`, half a point; combo box left 12 / right 0 → `7.0` and top 5 / bottom 7 → `7.0` (`:340-343`), six points and one; `kde-breeze` combo box left 6 / right 0 (`native-theme/src/presets/kde-breeze.toml:295-296`, border `:43`) → `4.0`, three points. `Button` additionally rounds its margin to whole points (`epaint/src/margin.rs:117-119`), so the `windows-11` button's odd sum leaves its height one point off native under any symmetric value | `button_padding` is a single `Vec2` (`style.rs:398`) that `Button`, `ComboBox` and `CollapsingHeader` apply symmetrically, and none of them offers a per-side margin (`egui/src/widgets/button.rs:331`, `:333-356`) | a `Margin`-typed `button_padding`, or a per-widget margin builder |
| 33 | **No icon size in `Style`** — `defaults.icon_sizes.*` (5), `menu.icon_size`, `toolbar.icon_size` and `dialog.icon_size` | none of `Spacing`'s 21 fields (`style.rs:392-465`) is an icon size — `Spacing::icon_width` is checkbox, radio and arrow geometry — and an image atom carries its own size (`atomics/atom_layout.rs:517`, `:569`) | a `Spacing::icon_size` that the image atoms of buttons and menus fall back to. Until then `icons::icon_size` (§4.10) gives the five `defaults.icon_sizes`, and the application reads `menu.icon_size`, `toolbar.icon_size` and `dialog.icon_size` from the `ResolvedTheme` |
| 34 | **Minimum sizes egui never reads** — `button.min_width`, `tab.min_width`, `input.min_height`, `progress_bar.min_width`, `dialog.min_width` and `dialog.min_height` | `Button` clamps only its cross axis, to `interact_size.y` (`widgets/button.rs:306-309`); a `TextEdit` is as tall as its rows plus its own margin, at least its per-instance `min_size` (`widgets/text_edit/builder.rs:715`, `:521`, `:410`); a `ProgressBar`'s `96.0` floor is a literal (`widgets/progress_bar.rs:113-114`); an `Area`, and so a `Modal`, has `default_size`, `default_width` and `default_height` and no minimum (`containers/area.rs:256`, `:263`, `:270`) | minimum sizes per widget kind in `Spacing` (or per class, rationale §4.2), and an `Area::min_size`. Until then the application reads the six from the `ResolvedTheme` for its own calls |
| 35 | **Widget padding that egui keeps per instance, or not at all** — `input.border.padding.*` (4), `list.border.padding.*` (4), `progress_bar.border.padding.{top,right,bottom}` and `expander.border.padding.left` | a `TextEdit`'s inner margin is the per-instance `TextEdit::margin`, default `Margin::symmetric(4, 2)` (`widgets/text_edit/builder.rs:82`, `:136`, `:313`); `Frame::group` hardcodes `6` (`containers/frame.rs:180`); a `ProgressBar` has no inset (`widgets/progress_bar.rs:130-204`); an expander's label starts at `Spacing::indent` (`containers/collapsing_header.rs:516`) | a `Style`-level `TextEdit` margin, and a per-widget `Margin` (the change item 32 names). The input's four sides are exposed as `input_margin()` (§4.7), the inner margin of `input_frame`. The checkbox's padding is not lost: both checkbox frames are inert (`widget_style.rs:178`, `:181-186`), but it is the mark's inset, carried in `spacing.icon_width_inner` as the mean of its two pairs (§6.11) |
| 36 | **`ProgressBar` has no frame** — `progress_bar.border.color`, `.corner_radius` and `.line_width` | it paints filled rects and a galley, with no stroke and no `Frame` (`widgets/progress_bar.rs:130-204`); its radius falls back to half its height (`:136-138`), and any other radius is per instance (`:92-96`) | a progress-bar style, the route of rationale §4.2's commit 3 |
| 37 | **Widgets cast no shadow** — `*.border.shadow_enabled` on `button`, `checkbox`, `segmented_control`, `input`, `combo_box`, `list`, `tab`, `expander` and `progress_bar` (9). Item 10 is the geometry where egui does draw a shadow; here it draws none | `ButtonStyle.frame` and `checkbox_frame` are `..Default::default()` (`widget_style.rs:166`, `:185`); `WidgetVisuals` has no shadow field (`style.rs:1295-1319`); a `ComboBox`'s button and an expander's header paint bare `RectShape`s (`containers/combo_box.rs:424-470`, `containers/collapsing_header.rs:562-568`); `Frame::group` sets none (`containers/frame.rs:180`) | a shadow on `WidgetVisuals`, painted by the widget frames |
| 38 | **List fills and outline** — `list.background_color`, `list.header_background` and `list.border.color` | `Grid`'s only non-debug fill is its per-row striping (`grid.rs:255-273`), as egui_extras's is (`egui_extras/src/layout.rs:129-151`); header cells take the body cells' path (`egui_extras/src/table.rs:499-503`); neither container draws an outline, and the `Frame::group` round one reads the table lines' own `noninteractive.bg_stroke` (`frame.rs:182`, `egui_extras/src/table.rs:897-900`), so the outline colour is set on the application's list `Frame` per call (§5.4) | a container fill, outline and header fill for `Grid` and `Table` |
| 39 | **No slider tick marks** — `slider.tick_mark_length` | `slider_ui` paints no ticks (`widgets/slider.rs:659-837`) | tick marks on `Slider` |
| 40 | **No per-role text colour, and no section gap** — `window.title_bar_font.color`, `dialog.title_font.color` and `layout.section_gap` | `FontId` carries no colour (`epaint/src/text/fonts.rs:21-28`) and egui has no text colour per role: a window's title is `visuals.text_color()` (`atomics/atom_layout.rs:300-301`), and a modal's one text colour is its body's (§5); `Spacing` has one general gap among its 21 fields, and sections are spaced with `Ui::add_space` | a text colour per `TextStyle`, and a `Spacing::section_gap`. The window title's font and colour are exposed as `window_title_bar_font` and `window_title_bar_text_color` (§4.7), for the `RichText` item 2b says overrides the title's fallback font |
| 41 | **No LCD subpixel text.** A desktop that renders text with per-channel (RGB or BGR) subpixel antialiasing cannot be matched: egui's glyphs are grey-scale coverage | native-theme models no subpixel order (no such leaf in `ResolvedTheme`), and egui has no expression for one either: epaint fills each glyph outline in white and stores one coverage value per texel (`epaint/src/text/font.rs:272-294`), and none of `TextOptions`' four fields — `max_texture_side`, `color_transfer_function`, `font_hinting`, `subpixel_binning` (`epaint/src/text/mod.rs:27-53`) — selects a channel order; `subpixel_binning` is horizontal glyph *positioning* (`:41-52`) | **both sides**: a subpixel-order leaf in native-theme's model, and per-channel glyph coverage in epaint |
| 42 | **No unantialiased text.** A user who turned text antialiasing off on the desktop still gets antialiased text in egui | native-theme models no such preference, and egui has no switch for it: glyph outlines are always filled with coverage (`epaint/src/text/font.rs:272-294`), and `TextOptions` has no antialiasing field (`epaint/src/text/mod.rs:27-53`) | **both sides**: an antialiasing leaf in native-theme's model, and a `TextOptions` switch in epaint |
| 43 | **A primary button, an active tab and an active segment are announced as toggle buttons.** The selected look (§6.2) is painted only for a `Button` that calls `selected(..)`, and the same call tells screen readers the button is a toggle, pressed or not | egui adds `SELECTED_CLASS`, the one thing `Style::button_style`'s selected branch tests, only from `Button::selected` (`widgets/button.rs:329`, `widget_style.rs:150`), and that flag becomes AccessKit's toggled state (`widgets/button.rs:263-269`, `:391-398`; `response.rs:975-980`); a native default button is announced as a plain button, and a native tab as a tab — no `WidgetType` maps to AccessKit's tab role (`response.rs:935-957`) | a way to choose the selected look without toggle semantics, such as a class the button can be given (rationale §4.2's commit 1), and a tab widget type |
| 44 | **A text size can still be too large for the font atlas.** Every text size the atlas writes or a `FontId` accessor, `font_size` or `mono_font_size` returns is a positive normal `f32` (§8.5), but a finite size large enough that one glyph outgrows the font atlas panics in release; nothing this crate knows at build time bounds it | epaint allocates each glyph in its atlas behind an `assert!` on the glyph's width (`epaint/src/texture_atlas.rs:226-229`), and a glyph taller than the room left takes `allocate`'s overflow branch (`:243-249`), after which its pixels are written past the image by an index (`epaint/src/text/font.rs:283-288`); the atlas side is `TextOptions::max_texture_side` (`epaint/src/text/mod.rs:29`, default `2048` at `:59`) and a glyph's pixel size scales with `pixels_per_point`, both run-time values. Measured on egui 0.36.2 at `pixels_per_point` 1: a Body size of 2000 panicked at `epaint/src/text/font.rs:288`, 1500 passed | a glyph too large for the atlas drawn as a missing glyph instead of a panic |
| 45 | **No text colour for an unfocused window's selection** — `defaults.selection_inactive_background` (§5.1, `source-side gap`) | `ResolvedTheme`'s selection text colours — `defaults.selection_text_color`, `defaults.text_selection_color` (`native-theme/src/model/resolved.rs:107`) and the input, sidebar and list ones (`native-theme/src/model/widgets/mod.rs:111`, `:424`, `:509`) — all pair with a focused fill; macOS, the one platform that states an unfocused fill (`unemphasizedSelectedContentBackgroundColor`, `docs/platform-facts.md:1057`; reader `native-theme/src/macos.rs:107`), would have its focused text colour painted on it. egui is not the obstacle: the install plugin already reads the window's focus (`ctx.input(\|i\| i.focused)`, §6.18) | **native-theme-side**: a leaf for the text colour the platform pairs with the unfocused selection fill, researched in `docs/platform-facts.md` first; the install plugin then swaps `selection.bg_fill` and `selection.stroke.color` for the two while the window is unfocused |

Numbers 21, 23, 24, 25 and 29 are retired and not reused: 21 landed with `SystemTheme::layout` (rationale §8, Q-2); 23 and 24 named leaves native-theme no longer has; 25 is closed by `expander_icon` (§4.7, §6.19); 29 by the line spacing computed at build (§6.15).

### 14.2 The upstream contribution

The egui change that would make SCOPED collapse into DIRECT for every widget
egui paints — widgets declaring their kind as a class, and `Style` gaining
class overrides — and the smaller PRs each ledger row names are argued in
rationale §4.2. The connector does not depend on any of them: the design works
against egui 0.36.2 exactly as shipped, and each would be a simplification,
never a prerequisite.

### 14.3 The no-widgets charter

The crate ships no `egui::Widget`. A widget may be added only if **both** hold:
egui 0.36.2 has no widget with that visual identity, **and** a majority of the
corresponding `ResolvedTheme` struct's leaves are otherwise UNMAPPABLE. The
widgets egui lacks are the companion crate `native-theme-egui-widgets`'s
(`docs/todo_egui-widgets-spec.md`); the argument is rationale §3.22.

---

## 15 -- Open verification items

Each item is stated where it applies, with what would verify it; this list
keeps them in one place so none is lost. Everything else this document states
was read at its source or measured.

| Item | Where | What closes it | Run by |
|---|---|---|---|
| The family names the macOS and Windows readers and presets give resolve to a face — by name through fontdb, and the macOS system UI font by its file through Core Text; macOS's `mono_font` is the "SF Mono" row below | §8.2 | `system_faces_resolve` (§13) passing on `macos-latest` and `windows-latest`, over the reader's theme and the platform's bundled preset | the implementation plan's runner-checks task |
| `want − row` with the platform's own Body face on macOS and Windows; KDE and GNOME are measured (§6.15) | §6.15 | `system_line_spacing` (§13) printing it on the same runners, the atlas's value asserted equal to T5's | the implementation plan's runner-checks task |
| Whether the Core Text system UI font's file is a collection whose faces record different families: `system_face` selects within the family the file's first face records, which could exclude the face Core Text chose. If it is, the remedy is undecided: the Core Text path reads only the font's family name and file URL, and no call it makes names the face within the file | §8.2 | the faces and families `system_faces_resolve` (§13) prints on `macos-latest` | the implementation plan's runner-checks task |
| The showcase's menus are installed in the macOS menu bar | §10.4 | the capture step's read-back of `NSApplication::mainMenu` on `macos-latest` (§13's third runner check) | the implementation plan's runner-checks task |
| iced's font database holds a family for `system_font_family` of `macos-sonoma`'s `defaults.font` and of its `mono_font`, "SF Mono", and the iced showcase's inspector shows "SF Pro" with no "not found"; and fontdb resolves "SF Mono" for this crate's `system_face`. If "SF Mono" resolves in neither, the remedy is the system UI font's file route for the monospaced system font, `NSFont::monospacedSystemFontOfSize:weight:` (`native-theme/src/macos.rs:222-223`), which needs the maintainer's approval of new `unsafe`, not designed here | §8.8, §8.2 | the iced showcase's runner tests and its `macos-sonoma` capture, and the line `system_faces_resolve` prints (§13), on `macos-latest` | the implementation plan's runner-checks task |
| gpui's text system lists the stated mono family "SF Mono" (`all_font_names`); if it does not, the remedy is `system_face`'s bytes through `TextSystem::add_fonts`, not designed here | §8.8 | the gpui showcase's macOS capture log (§13) on `macos-latest` | the implementation plan's runner-checks task |
| AppKit hands a menu accelerator to the menu before winit's view sees the key | §10.4 | a person at a Mac pressing Cmd+B once and seeing the side panel toggle exactly once | none: no runner presses keys; it stays open until someone checks it on a Mac |
| Whether a change of Windows' non-client fonts, high contrast or animation flag raises `ColorValuesChanged`, and whether a macOS accent or accessibility change posts `AppleInterfaceThemeChangedNotification` | §10.2 | changing each setting while a `native_theme::watch::on_theme_change` subscription runs | none: native-theme's (`docs/todo.md`, the watcher item); no runner changes OS settings |
| egui's appetite for the upstream change | rationale §4.2 | an issue or discussion on the egui repository | none: the connector depends on no upstream change (§14.2) |

The settled measurements and the decisions of the questions raised during
design (Q-1 to Q-8) are rationale §8.
