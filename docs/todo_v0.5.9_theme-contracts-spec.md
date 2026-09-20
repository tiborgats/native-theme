# v0.5.9 — Theme contracts: Specification

Status: Design (2026-09-20, revised 2026-09-21); nothing implemented
Companion rationale:
[`todo_v0.5.9_theme-contracts-rationale.md`](todo_v0.5.9_theme-contracts-rationale.md)
(decisions C1–C16)
Companion plan:
[`todo_v0.5.9_theme-contracts-plan.md`](todo_v0.5.9_theme-contracts-plan.md)
Sibling work in the same release:
[`todo_v0.5.9_gpui-kit-0.6.4-spec.md`](todo_v0.5.9_gpui-kit-0.6.4-spec.md).
Where the two touch the same file, the gpui-kit work lands first; this
specification assumes its state.

---

## 0 -- Scope

### 0.1 What this delivers

1. The iced connector stops writing native values into iced slots that mean
   something else, where the slot has one meaning (§2).
2. The iced connector gains `styles`, a module of per-widget style functions,
   for the slots that have several meanings (§3).
3. Text scaling reaches an iced application, through the two font-size
   helpers, and the crate forwards the icon features (§4).
4. Both connectors gain a **mapping-contract table**: every slot, the native
   field it must equal, checked over all 32 preset/mode combinations, with a
   coverage tripwire over every slot (§5).
5. Both showcases gain self-tests (§6) and show **every** widget their
   toolkit offers an application, kept complete by a test for our own surface
   and a script for the toolkit's (§6a).
6. Both connectors gain a contrast invariant: the connector never makes a
   text-on-background pair worse than the platform's own (§7).
7. The iced connector optionally covers `iced_aw`, which supplies six widgets
   native-theme models and iced core does not have (§3a).

### 0.2 Constraints

- No panics, no `unsafe`, no invented values, no hardcoded theme values, as in
  the sibling specification §0.2.
- Every number a test expects comes from the resolved theme or from the value
  under test, never from a literal.
- A contract row that cannot be satisfied is a **finding**, not a row to
  delete: stop and report it.

### 0.3 Out of scope

Screenshot diffing; an iced `geometry` module; upstream token proposals
(rationale §4).

---

## 1 -- Facts this specification rests on

Verified 2026-09-20 against the published sources; the rows marked † were
corrected on 2026-09-21, when each was re-checked against the same sources.

| Fact | Evidence |
|---|---|
| † Most iced widgets take `.style(impl Fn(&Theme, Status) -> Style)`, but **not all**: `container`, `progress_bar` and `tooltip` take `impl Fn(&Theme) -> Style` with no `Status`, and the menu overlay is set with `.menu_style(..)` on `PickList` / `ComboBox` rather than `.style(..)` | `button.rs:184`, `text_input.rs:278`, `checkbox.rs:236`, `toggler.rs:234`, `scrollable.rs:243`, `slider.rs:195` vs `container.rs:214`, `progress_bar.rs:112`, `tooltip.rs:139-141`, `pick_list.rs:303-305`, `combo_box.rs:308`, `overlay/menu.rs:631` (`StyleFn = Box<dyn Fn(&Theme) -> Style>`) |
| iced's default `text_input` style reads `placeholder: palette.secondary.base.color` and `selection: palette.primary.weak.color` | `text_input.rs:1769, 1771` |
| The iced connector writes `secondary.base.color = button.background_color` | `connectors/native-theme-iced/src/extended.rs:109` |
| † `secondary.base.color` has **six** readers with **three** meanings: placeholder text, the `button::secondary` and `container::secondary` fill, and the `progress_bar::secondary` bar fill | `text_input.rs:1769`, `text_editor.rs:1476`, `pick_list.rs:910`, `button.rs:615`, `container.rs:629`, `progress_bar.rs:299` |
| `button::secondary`'s hover reads `palette.secondary.strong.color`, which the connector never writes, and which nothing else reads | `button.rs:620`; `extended.rs:109-119` writes only `.base` |
| The pick-list/menu overlay highlight is `palette.primary.strong` | `overlay/menu.rs:657-658` |
| Nothing in the iced connector takes accessibility preferences: `font_size` and `mono_font_size` return the platform size unscaled, and `from_system` drops the `SystemTheme` that carries them | `connectors/native-theme-iced/src/lib.rs:259, 274, 177-189` |
| † An iced `Theme` carries a palette and no font size, so text scaling cannot land in `to_theme` | `iced_core-0.14.0/src/theme.rs`, `Theme::custom_with_fn` |
| † Nothing the palette reads is translucent: `background`, `text`, `accent`, the three status colours, `surface` and `accent_text` are opaque in all 32 combinations. Of the colours `styles::*` emits, 14 fields are translucent somewhere, and every one is a state overlay or a scrollbar thumb — no surface | measured 2026-09-21 |
| † Today's placeholder contrast is 1.06–1.84 on **all 32** combinations. iced's *generated* `secondary.base.color` would be worse than the platform's own pair in 22 of 32 (material light 9.11 → 2.91). `input.placeholder_color` in that slot is exactly native in foreground on all 32; its ratio is lower than native in 12, by at most 0.68, solely because iced paints a text input on the window background, which differs from `input.background_color` in 18 of 32 | measured 2026-09-21, alpha composited |
| † `Pair::new(color, text)` is public and picks a readable text colour by iced's own rule; over the 32 combinations the label it gives on `input.placeholder_color` never falls below 4.16:1 | `iced_core-0.14.0/src/theme/palette.rs:440-445`; measured |
| † 25 model fields are `soft_option`: they stay `Option` in `ResolvedTheme`. 17 of them are read by §3.3. None is `None` in any bundled preset, but a live OS reader may produce one | `native-theme-derive/src/gen_structs.rs:42`; `native-theme/src/model/widgets/mod.rs`; measured |
| † **No `#[test]` can see the missing icon features.** `cargo test` builds with the dev-dependencies, which already enable `material-icons`, `lucide-icons` and `system-icons` on `native-theme`. Only the no-dev feature tree shows the defect | `cargo tree -p native-theme-iced -e no-dev,features -i native-theme` lists only `default` today |
| The iced connector declares no `[features]`; the gpui one forwards four | `connectors/native-theme-iced/Cargo.toml`; `connectors/native-theme-gpui/Cargo.toml:26-31` |
| An `[[example]]` target runs its `#[cfg(test)]` tests when the manifest sets `test = true`; the default is `false` | Cargo book, *Configuring a target*; both showcase entries currently omit it (`native-theme-gpui/Cargo.toml:80-81`, `native-theme-iced/Cargo.toml:26-27`) |
| † **iced 0.14 does ship a headless renderer and interaction simulator.** `iced_test` 0.14.0 — "A library for testing iced applications in headless mode" — was published 2025-12-07, the same day as iced 0.14.0, and is reachable as a plain dev-dependency or through iced's `tester` feature | crates.io metadata; `iced_test-0.14.0/src/simulator.rs:45-253`; `iced-0.14.0/Cargo.toml:101`. The first draft claimed the opposite on the strength of `ls ~/.cargo/registry/src/*/iced*`, which only shows what a local build has pulled |
| † The simulator needs a renderer backend in the test's dependency graph. Without one every widget measures 0×0 and `click` returns `TargetNotVisible`; with iced's default features the same button measured 41.904 × 20.8 and the click produced its message | measured 2026-09-21 in a scratch crate against the published sources |
| † The iced `Style` structs this specification fills, with their **full** field ranges | `button.rs:487-498`, `text_input.rs:1713-1726`, `checkbox.rs:528-537`, `toggler.rs:508-529`, `scrollable.rs:2271-2282`, `container.rs:462-473`, `slider.rs:589-594`, `progress_bar.rs:248-255`, `overlay/menu.rs:598-611` |
| † Seven of those nine have no `Default`; **`button::Style` and `container::Style` do** | `impl Default for Style` at `button.rs:510-520` and `container.rs:475-485`; the other seven carry only `#[derive(Debug, Clone, Copy, PartialEq)]` |
| † Scrollbar *widths* are not in `scrollable::Style`. They live on the `Scrollbar` value passed to `.direction(..)` | `scrollable.rs:322-328` (private fields), `:355` `width`, `:367` `scroller_width` |
| † iced has no minimum-thumb-length setting: the scroller length is `(bounds * ratio).max(2.0)`, hardcoded | `scrollable.rs:2068` |
| `Theme::list_presets()` returns exactly the 16 user-selectable presets; the four `*-live.toml` are internal geometry-only merge bases | `native-theme/src/model/mod.rs:614` (its doctest asserts `len() == 16`), `native-theme/src/presets.rs:51` |
| `ThemeColor` has 138 fields, and the gpui connector already proves every one is assigned | `gpui-component-0.6.4/src/theme/theme_color.rs:59`; `connectors/native-theme-gpui/src/colors.rs`, `theme_color_field_count_tripwire` and `no_theme_color_field_is_left_at_default` |
| `gpui_base::PANEL_MIN_SIZE` is public, so a showcase test can name it | `gpui-base-0.6.4/src/lib.rs:147` |

---

## 2 -- The iced palette: what changes

`src/extended.rs`, `apply_overrides`. One slot changes its source (rationale
§2.2a, C3).

| Line today | Change | Why |
|---|---|---|
| `extended.secondary.base.color = to_color(colors.btn_bg)` and `extended.secondary.base.text = to_color(colors.btn_fg)` | **replaced** by `extended.secondary.base = Pair::new(to_color(colors.placeholder), extended.background.base.text)` | the slot has six readers and three meanings (§1), and the text meaning wins because it is the one a wrong value makes unreadable. Measured, the placeholder foreground becomes exactly the platform's on all 32 combinations. The three fill readers get the placeholder tone, which is what iced's own design gives them, labelled by iced's own readable-text rule. |
| — | **not** `secondary.strong.color = button.hover_background` | that slot does mean "hover" unambiguously (`button.rs:620`), but its base is no longer the button's, and a control that idles in one colour family and hovers into another is the incoherence this release removes. `styles::button` carries idle, hover, pressed and label together. |
| — | **not** "leave iced's generated value", which the second draft chose | measured worse than the platform's own pair in 22 of 32 (§1). |
| `background.weak.color = surface`, `background.weak.text = foreground` | kept | overridden as a *pair*, so they stay coherent; the closest single meaning iced has for a subdued panel. The nine readers that want something else are served by `styles::*`. |
| `primary.base.text = accent_fg` | kept | `primary.base.color` comes from the `Palette` itself (`palette.rs:41`), so this pair is native on both sides. |
| the four `ensure_status_contrast` lines | kept | correct as written |

`OverrideColors` loses `btn_bg` and `btn_fg`, which nothing reads any more
(leaving them fails `-D warnings` on `dead_code`), and gains
`placeholder: Rgba`, filled from `resolved.input.placeholder_color`.

No other palette slot changes. Everything else moves to §3.

---

## 3 -- `native_theme_iced::styles`

**This module needs a dependency the connector does not have.** The crate
depends on `iced_core` only; every `Style` struct below lives in
`iced_widget`, so `iced_widget = "0.14"` joins `[dependencies]`. That is the
same shape as the gpui connector depending on `gpui-component` in order to
name `ThemeColor`, and it costs a consumer nothing — an iced application
already has `iced_widget` through `iced`. What it does cost is version
coupling: the connector now tracks `iced_widget`, and the nightly canary will
report a breaking change there the same way it reported gpui-component's.

A new public module, `src/styles.rs`. Each function takes `&ResolvedTheme`
and returns a closure iced accepts. Pure; no global state; the returned
closure ignores the `&Theme` argument, because the values are already
resolved. Each closure owns the handful of `Color` values it needs, captured
by value, so it is `'static` and can be stored in a widget.

### 3.1 Three closure shapes, not one

The first draft said every function had the same signature. It does not
(§1). There are three shapes, and each function's doc comment names which
setter it is passed to.

```rust
// A. with Status, passed to .style(..)
//    button, button_primary, text_input, checkbox, toggler, scrollable, slider
pub fn button(resolved: &ResolvedTheme)
    -> impl Fn(&Theme, button::Status) -> button::Style + use<>;

// B. without Status, passed to .style(..)
//    container_card, progress_bar, tooltip (tooltip returns container::Style)
pub fn container_card(resolved: &ResolvedTheme)
    -> impl Fn(&Theme) -> container::Style + use<>;

// C. without Status, passed to .menu_style(..) on PickList / ComboBox
pub fn menu(resolved: &ResolvedTheme)
    -> impl Fn(&Theme) -> menu::Style + use<>;

// D. not a closure at all: a configured widget value (C14)
pub fn scrollbar(resolved: &ResolvedTheme) -> scrollable::Scrollbar;
```

Shape A was compiled against iced 0.14 before this specification was written:
`use<>`, exhaustive construction, and acceptance by `.style()` under a
`'static` bound.

### 3.2 Exhaustive construction, and fields the model does not carry

Every function constructs its `Style` **exhaustively**, never with
`..Default::default()`. For the seven structs that have no `Default` the
compiler enforces this, so an upstream field addition fails the build — the
sibling release's E17 guarantee, for free. `button::Style` and
`container::Style` **do** have `Default` (§1), so for those two the compiler
cannot enforce it; the rule is ours, and §5's contract rows assert every
field of those two by name so an added field is still noticed.

Some `Style` fields have no native counterpart at all. They are not invented
and they are not guessed:

> **A `Style` field the native model does not carry takes the value iced's own
> default style function gives it for that widget and status, cited by file
> and line in a comment beside it.**

The ones this release meets, with their sources:

| Field | Value, from iced | Citation |
|---|---|---|
| `button::Style.snap`, `container::Style.snap` | `Style::default().snap`, which is `cfg!(feature = "crisp")` — **written as `Style::default().snap`, not as a literal**, so a consumer who enables `crisp` keeps it | `button.rs:517`, `container.rs:482` |
| `scrollable::Style.gap` | `None` | `scrollable.rs:2375` |
| `scrollable::Style.auto_scroll` | iced's `AutoScroll`, constructed as iced does from the palette | `scrollable.rs:2357-2368` |
| `toggler::Style.border_radius` | `None` (perfectly round) | `toggler.rs:610` |
| `toggler::Style.padding_ratio` | `0.1` | `toggler.rs:611` |
| `menu::Style.shadow` | `Shadow::default()` | `overlay/menu.rs:659` |
| `text_input::Style.icon` | iced's own choice, `palette.background.weak.text` — the model has no input-icon colour | `text_input.rs:1768` |
| `container::Style.text_color` for `container_card` | `None`, which inherits — `CardTheme` carries no font | `container.rs:478` |

**Soft options (C16).** Seventeen of the native fields below are
`soft_option`: `Option` even after resolution, where `None` is the platform
stating the widget has no distinct appearance in that state. The fallback is
always a **copy** of the widget's base-state value — never arithmetic, never
`unwrap` — the rule the egui design already set. Each chain ends on a required
field in one step:

| Soft option | `None` → |
|---|---|
| `button.active_background` | `button.hover_background` |
| `button.disabled_background` | `button.background_color` |
| `input.hover_border_color`, `input.focus_border_color` | `input.border.color` |
| `input.disabled_background` | `input.background_color` |
| `checkbox.hover_background`, `checkbox.unchecked_background`, `checkbox.disabled_background` | `checkbox.background_color` |
| `checkbox.unchecked_border_color` | `checkbox.border.color` |
| `scrollbar.thumb_active_color` | `scrollbar.thumb_hover_color` |
| `slider.thumb_hover_color` | `slider.thumb_color` |
| `switch.hover_checked_background`, `switch.disabled_checked_background` | `switch.checked_background` |
| `switch.hover_unchecked_background`, `switch.disabled_unchecked_background` | `switch.unchecked_background` |
| `switch.disabled_thumb_color` | `switch.thumb_background` |
| `tab.hover_background` (§3a) | `tab.background_color` |

**Alpha is emitted unchanged.** Fourteen of these colours are translucent on
some platform (Windows 11's hover overlays, macOS's scrollbar thumbs). They
are state overlays the platform itself draws translucent over whatever lies
beneath, and iced draws a `Style` background the same way, so `styles::*`
passes the alpha through. Compositing belongs to §7's *measurement*, not to
what the connector emits.

### 3.3 The functions

| Function | Shape | Every `Style` field it fills | Native source |
|---|---|---|---|
| `button` | A | `background`, `text_color`, `border`, `shadow`, `snap` | `button.background_color` / `.font.color` / `.border.*`; `hover_background`, `active_background`, `hover_text_color`, `active_text_color`, `disabled_*` per `Status` |
| `button_primary` | A | as above | `button.primary_background`, `primary_text_color` |
| `text_input` | A | `background`, `border`, `icon`, `placeholder`, `value`, `selection` | `input.background_color`, `.border.*`, `.placeholder_color`, `.font.color`, `.selection_background`; `hover_border_color` and `focus_border_color` per `Status`; `icon` from §3.2 |
| `checkbox` | A | `background`, `icon_color`, `border`, `text_color` | `checkbox.checked_background`, **`.indicator_color`** (the check mark; there is no `check_color`), `.unchecked_background`, `.unchecked_border_color`, `.border.*`, `.font.color` |
| `toggler` | A | `background`, `background_border_width`, `background_border_color`, `foreground`, `foreground_border_width`, `foreground_border_color`, `text_color`, `border_radius`, `padding_ratio` | `switch.unchecked_background`, `checked_background`, `thumb_background`, `hover_checked_background`, `hover_unchecked_background`, `disabled_*` per `Status`; the last two fields from §3.2 |
| `scrollable` | A | `container`, `vertical_rail`, `horizontal_rail`, `gap`, `auto_scroll` | `scrollbar.track_color` → each rail's `background`; `thumb_color`, `thumb_hover_color`, `thumb_active_color` → the `Scroller` background per `Status`; the last two fields from §3.2 |
| `scrollbar` | D | — (a `Scrollbar`, not a `Style`) | `scrollbar.groove_width` → `.width(..)`, `scrollbar.thumb_width` → `.scroller_width(..)` |
| `menu` | C | `background`, `border`, `text_color`, `selected_text_color`, `selected_background`, `shadow` | `menu.background_color`, `.border.*`, `.font.color`, `.hover_text_color`, `.hover_background` |
| `container_card` | B | `text_color`, `background`, `border`, `shadow`, `snap` | `card.background_color`, `.border.*`; `text_color` and `snap` from §3.2 |
| `slider` | A | `rail` (`backgrounds`, `width`, `border`), `handle` (`shape`, `background`, `border_width`, `border_color`) | `slider.fill_color` and `track_color` → `rail.backgrounds`; `track_height` → `rail.width`; `thumb_color`, `thumb_hover_color` → `handle.background`; `thumb_diameter` → `handle.shape` |
| `progress_bar` | B | `background`, `bar`, `border` | `progress_bar.track_color`, `fill_color`, `.border.*` |
| `tooltip` | B | a `container::Style`: `text_color`, `background`, `border`, `shadow`, `snap` | `tooltip.background_color`, `.border.*`, `.font.color` |

Twelve items: eleven style functions and `scrollbar`.

**One native value has no receiver in iced 0.14 and is not approximated:**
`scrollbar.min_thumb_length`. iced computes the scroller length as
`(bounds * ratio).max(2.0)` with no setting (§1). It is recorded here, in
`docs/todo.md`, and in the contract file's unreachable list, rather than
mapped onto something it is not.

Widgets whose native fields the model does not carry (`Tag`, `Badge`,
`Rule` beyond its colour) are not covered; the palette serves them.

Each function's doc comment names the iced default it replaces, the slot whose
meaning it corrects, and the setter it is passed to — as
`variants::ghost_button` does in the gpui connector.

---

## 3a -- `iced_aw`: the widgets iced core does not have

native-theme models `menu`, `card`, `tab`, `sidebar`, `spinner` and a list
whose selection semantics match a selection list. **iced core has none of
them**, so those six widget themes have no receiver in iced at all today —
the largest remaining hole in "make an iced application look native", and
wider than any of the seven mapping defects.

`iced_aw` 0.14.1 (published 2026-04-27) supplies exactly those, and depends on
`iced_core ^0.14.0` and `iced_widget ^0.14.2` — the versions this connector
uses, so it unifies with no version work (crates.io dependency metadata,
re-checked 2026-09-21). Its styling is the same shape as iced's own: a plain
`Style` struct of public fields, a `Catalog` trait whose `Class` is a
`StyleFn`, and a style setter on each widget. So `styles::aw::*` is the same
pattern as §3 — including §3.1's rule that the exact closure shape is read
from the widget at implementation time, not assumed.

**Optional, because it is a third-party crate.** An application that does not
use `iced_aw` must not pay for it, and native-theme must not tie its release
cadence to a community crate. Two measured costs: `iced_aw` depends
unconditionally on `iced_fonts` 0.3.0, whose published archive is **3.31 MiB**
of embedded font data, and its iced support lags — iced 0.14.0 was published
2025-12-07 and `iced_aw` 0.14.0 arrived 2026-04-27, four and a half months
later. A default-on third-party dependency would make this crate unbuildable
on a new iced for that long, which is the failure mode the whole release
exists to repair.

It is therefore off by default, and it implies `widgets`, because `iced_aw`
itself depends on `iced_widget ^0.14.2`. See §4.2 for the full feature table.

Six widgets, from eight `iced_aw` features: `tabs` is the tabbed container
built on `tab_bar` (its feature literally is `tabs = ["tab_bar"]`), and
`context_menu` is the same `menu` styling applied to a right-click overlay, so
each pair shares one `styles::aw::*` function. All eight feature names were
checked against the published feature table on 2026-09-21.

Covered widgets, each from the native theme that models it:

| `styles::aw::*` | `iced_aw` widget | Native source |
|---|---|---|
| `card` | `Card` | `card.background_color`, `.border.*`; head/body/foot from `card` and `defaults` — `CardTheme` carries only `background_color` and `border`, so the rest comes from `defaults` |
| `menu` | `Menu`, `ContextMenu` | `menu.background_color`, `.hover_background`, `.hover_text_color`, `.border.*`, `.font.color` |
| `tab_bar` | `TabBar`, `Tabs` | `tab.background_color`, `.active_background`, `.active_text_color`, `.hover_background`, `.bar_background` |
| `sidebar` | `Sidebar` | `sidebar.background_color`, `.selection_background`, `.selection_text_color`, `.hover_background` |
| `spinner` | `Spinner` | `spinner.diameter`, `.min_diameter`, `.stroke_width`, `.fill_color` (there is no `spinner.color`) |
| `selection_list` | `SelectionList` | `list.background_color`, `.selection_background`, `.selection_text_color`, `.hover_background`, `.row_height` |

Not covered, because native-theme models no equivalent and inventing one is
forbidden: `badge`, `date_picker`, `time_picker`, `color_picker`, `drop_down`,
`number_input`, `slide_bar`, `wrap`, `quad`, `labeled_frame`. They are listed
in the showcase exception file (§6a.4) with that reason.

The iced showcase enables the feature as a dev-dependency and renders all six,
so they are covered by §6a's completeness rule like any other widget.

---

## 4 -- Accessibility preferences and features (iced)

### 4.1 Text scaling

```rust
pub fn font_size(resolved: &ResolvedTheme, prefs: &AccessibilityPreferences) -> f32;
pub fn mono_font_size(resolved: &ResolvedTheme, prefs: &AccessibilityPreferences) -> f32;

pub fn from_system() -> Result<(Theme, ResolvedTheme, bool, AccessibilityPreferences)>;
```

Each font size is multiplied by `prefs.text_scaling_factor` when that is
finite and positive, else by 1 — the same sanitising the gpui connector does
(`gpui/src/lib.rs:414-417`). `from_system` returns the preferences it used to
drop, because a caller now needs them for `font_size`.
`AccessibilityPreferences` is re-exported from the crate root, as gpui does.

**`to_theme`, `from_preset` and `SystemThemeExt::to_iced_theme` do not
change.** The first draft gave them a preferences parameter for symmetry with
gpui; it would have been dead (rationale §2.10):

| Preference | In iced |
|---|---|
| `text_scaling_factor` | the two functions above. An iced `Theme` is a palette and has no font size to scale |
| `reduce_transparency` | **no receiver today**: nothing the palette reads and no surface `styles::*` emits is translucent in any of the 32 combinations (§1). Revisit when a preset or a live reader yields a translucent surface |
| `reduce_motion` | **no receiver**: iced has no global animation switch. The application reads the public field; the connector adds no wrapper |

The call sites are few: `font_size(` and `mono_font_size(` in the showcase
(`showcase-iced.rs:2365, 2370`), the crate's own tests, and the README;
`from_system()` in the README and the crate docs.

### 4.2 Features

`connectors/native-theme-iced/Cargo.toml` gains the four icon features and the
same default set as the gpui connector:

```toml
[features]
default = ["widgets", "material-icons", "lucide-icons", "system-icons", "svg-rasterize"]

# Which iced crates this connector covers. Additive, never subtractive.
widgets = ["dep:iced_widget"]            # enables `styles`
iced_aw = ["widgets", "dep:iced_aw"]     # enables `styles::aw`

material-icons = ["native-theme/material-icons"]
lucide-icons = ["native-theme/lucide-icons"]
system-icons = ["native-theme/system-icons"]
svg-rasterize = ["native-theme/svg-rasterize"]
```

| A consumer who wants | Writes |
|---|---|
| everything iced itself offers (the default) | nothing |
| the `iced_aw` widgets too | `features = ["iced_aw"]` |
| the palette only, no `iced_widget` | `default-features = false` |
| the palette plus icons, no `iced_widget` | `default-features = false, features = ["lucide-icons"]` |

The features are **positive**: each adds coverage. There is deliberately no
`no_aw`, `no_widgets` or `core_only`, because Cargo unifies features across
the whole dependency graph and a subtractive feature enabled anywhere would
silently remove `styles` from every other consumer, with no way for them to
countermand it. `default-features = false` is the supported way to narrow, and
it is per-consumer (rationale §2.9).

`styles` is `#[cfg(feature = "widgets")]`, `styles::aw` is
`#[cfg(feature = "iced_aw")]`, and each module's contract and contrast tests
carry the same gate. `[package.metadata.docs.rs] all-features = true`, as in
the gpui connector, so docs.rs shows the whole surface.

---

## 5 -- Layer 1: the mapping contract

One new file per connector: `connectors/native-theme-gpui/src/contract.rs` and
`connectors/native-theme-iced/src/contract.rs`, each `#[cfg(test)]` only.

### 5.1 The table

The two connectors have different target types, so each declares its own row
type. Neither is shared; with two connectors the duplication is smaller than
the abstraction (rationale §4).

```rust
// gpui
/// One row of the mapping contract: a toolkit slot, the native field it must
/// equal, and the presets where it may legitimately differ.
struct Row {
    slot: &'static str,
    native: fn(&ResolvedTheme) -> Rgba,
    get: fn(&ThemeColor) -> Hsla,
    /// Preset keys where this row does not hold, each with its reason.
    exceptions: &'static [(&'static str, &'static str)],
}

// iced: the target is a palette slot or a `styles::*` output, not a struct
// field, so the getter takes both the theme and the resolved values.
struct Row {
    slot: &'static str,
    native: fn(&ResolvedTheme) -> Rgba,
    get: fn(&Theme, &ResolvedTheme) -> Color,
    exceptions: &'static [(&'static str, &'static str)],
}
```

The test iterates every row over all 16 presets in both modes and asserts
equality. Sixteen, not the twenty preset TOMLs in `native-theme/src/presets/`:
the four `*-live.toml` are geometry-only merge bases for the OS-first
pipeline, carry no colours, and are not user-selectable
(`native-theme/src/presets.rs:51`). They are excluded by iterating
`Theme::list_presets()`, which returns exactly the sixteen (§1). A row that
must differ on some preset carries the reason in `exceptions` — not a
loosened assertion.

Rows are the fields with a native counterpart, including the ones this release
corrected: `accent` ← `menu.hover_background`, `accent_foreground` ←
`menu.hover_text_color`, `sidebar_accent*` ← the sidebar's selection pair,
`secondary_hover` ← `button.hover_background`, `list_hover`, `list_active`,
`selection`, `input`, `primary`, the status colours, the scrollbar colours,
the tab colours, and the four base-palette colours the connector maps
directly (`red` ← `danger`, `green` ← `success`, `blue` ← `info`, `yellow` ←
`warning`; `colors.rs:601-607`).

For the iced connector, the palette rows include `secondary.base.color` ←
`input.placeholder_color` — the row that would have caught the defect this
work began with — and rows cover every `styles::*` field of §3.3 — including
every field of `button::Style` and `container::Style` by name, because those
two have a `Default` and the compiler will not notice an added field (§3.2).

This replaces the two bespoke tests the sibling release added for `accent`
(C10): the contract table is where that claim belongs.

### 5.2 The coverage tripwire

```rust
#[test]
fn every_theme_color_field_has_a_declared_source() { … }
```

Every field of `ThemeColor` appears in exactly one of **two** lists:

1. the contract table (§5.1) — equal to a named native field;
2. `DERIVED` — with the derivation named (`hover_color(primary)`,
   `light_variant(bg, danger, is_dark)`, `magenta` as a fixed hue carrying
   the accent's saturation and lightness, a blend).

The test asserts the two lists partition the 138 fields exactly: no field
missing, none in both. A new upstream field therefore fails the build until
someone classifies it, exactly as the `Theme` shape tripwire (sibling E17)
does for shape.

**There is no third `UPSTREAM_DEFAULT` list.** The first draft specified one,
for "`transparent` and the 12 private palette colours". Both halves were
wrong: `colors.rs:601-628` assigns all twelve base-palette fields from native
values, and `no_theme_color_field_is_left_at_default` (`colors.rs`) already
proves, through serde over all 138 fields in both modes, that nothing is left
at upstream's default. A list of fields we leave to upstream would be empty,
and writing reasons into it would state something untrue. If a future upstream
field genuinely has no native source, the right move is to add the third list
*then*, with the real reason.

The iced file does the same over the palette slots the connector writes and
over every `styles::*` function's output, plus a third short list it does
need: `UNREACHABLE`, for a native value iced has no receiver for, with the
evidence — today exactly one entry, `scrollbar.min_thumb_length` (§3.3).

---

## 6 -- Layer 2: showcase self-tests

Both manifests gain `test = true` on the example target.

### 6.1 gpui (`examples/showcase-gpui.rs`, `#[cfg(test)] mod tests`)

| Test | Asserts |
|---|---|
| `every_tab_lays_out` | each of the ten tabs renders on GPUI's test platform and `debug_bounds` finds the tab root; no panic |
| `resizable_groups_have_room_to_drag` | for each resizable group in the file, the container's size exceeds `panels × gpui_base::PANEL_MIN_SIZE` plus its borders. This is the finding of rationale §1.1 stated as a rule |
| `interactive_controls_respond` | for the controls the showcase advertises as interactive, a simulated click produces the effect: the Copy button writes the field's text to the clipboard (`cx.read_from_clipboard()`; the test platform holds a real in-memory clipboard, `gpui-pre-0.3.5/src/platform/test/platform.rs:671-677`), the mode switch changes `Theme::mode`, the notification buttons push a notification |

### 6.2 iced (`examples/showcase-iced.rs`, `#[cfg(test)] mod tests`)

iced 0.14 **does** render headlessly (§1), so these are real interaction
tests, not structural inspections. `iced_test = "0.14"` joins the connector's
`[dev-dependencies]` (C15); the showcase's existing `iced` dev-dependency
already carries a renderer backend, which §1 records as the condition.

| Test | Asserts |
|---|---|
| `every_tab_renders` | for each tab, `Simulator::with_size(..., view())` then `snapshot(&theme)` returns `Ok` — the interface lays out and draws with no panic. The snapshot is **not** compared to a baseline; that is Layer 4, deliberately out of scope (rationale §4) |
| `interactive_controls_respond` | for each control the showcase advertises, `ui.click(<its label>)` succeeds and `into_messages()` contains the message it should send. A button built without `on_press` renders as `Status::Disabled` (`button.rs:342`) and its click fails — the iced form of the dead-Copy-button finding |
| `styles_cover_every_widget_shown` | every widget the showcase renders is styled with the `styles::*` function for it — no widget is left on the palette default, because the showcase is what the README's screenshots claim the connector achieves (rationale §2.7) |

If `Simulator` cannot initialise a renderer in CI, that is a **finding**: report
it with the error, do not silently downgrade the tests to view-tree inspection.

---

## 6a -- Complete widget coverage

A showcase that omits a widget is a widget nobody has ever seen under a native
theme. Measured 2026-09-20 and re-measured 2026-09-21, both omit a great deal,
and the gpui connector omits its own work.

### 6a.1 gpui: builders the showcase never exercises

**19 of the 34 `geometry` builders are never used in the showcase**
(re-measured 2026-09-21: still exactly these), so more than half of the
connector's own public geometry has never been looked at:

`control_height`, `menu_item`, `tooltip`, `status_bar`, `dialog_description`,
`table`, `radio`, `combobox`, `title_bar`, `icon_size_toolbar`,
`icon_size_small`, `icon_size_large`, `icon_size_dialog`, `icon_size_panel`,
`input_height`, `widget_gap`, `container_margin`, `window_margin`,
`section_gap`.

### 6a.2 gpui: widgets the showcase never renders

Constructible widgets absent from the showcase, filtered from
gpui-component 0.6.4's `RenderOnce` / `IntoElement` implementations by
discarding test harnesses and sub-parts. Every name below was checked against
`pub struct` in the vendored 0.6.4 source on 2026-09-21:

`AlertDialog`, `Attachment`, `Bubble`, `Combobox`, `DescriptionList`,
`HoverCard`, `Marker`, `Message`, `MessageScroller`, `Pagination`,
`ProgressCircle`, `Rating`, `ShimmerText`, `SidebarToggleButton`,
`StatusBar`, `Stepper`, `TitleBar`, `WindowBorder`, `CarouselNext`,
`CarouselPrevious`.

Twenty, not the twenty-one the first draft listed. Two names in it were
wrong: **`DescriptionText` is not a widget** — it is the label type
`DescriptionList::new` takes (`description_list.rs:81`), exactly the kind of
sub-part the filter is meant to discard — and **`ShimmerGlyphs` does not
exist**; `shimmer.rs` declares `ShimmerSpread`, `ShimmerStyle` and
`ShimmerText` and nothing else.

`Combobox` is generic over a `SearchableListDelegate` (`combobox.rs:749`), so
showing it needs a delegate; that is work, not a blocker.

`StatusBar`, `TitleBar` and `Combobox` are the ones that matter most: the
connector ships a geometry builder for each and the showcase renders none of
them. The v0.5.9 sibling rationale §2.11 deferred `Stepper`, `Rating`,
`Pagination`, `HoverCard`, the command palette and the chat components to "a
later showcase pass"; the maintainer's standing rule supersedes that, and they
are in this release.

### 6a.3 iced: widget modules the showcase never renders

`iced_widget` 0.14.2 has 41 source modules under `src/`; discounting `lib`,
`helpers` and `action`, which declare no widget, 38 remain. **Fifteen are
absent** from the showcase (measured 2026-09-21 by name reference):

`canvas`, `float`, `keyed`, `lazy`, `markdown`, `overlay`, `pane_grid`,
`pin`, `qr_code`, `responsive`, `sensor`, `shader`, `stack`, `table`,
`themer`.

Fifteen, not the fourteen the first draft listed: **`shader` was missing from
it entirely**, so the coverage script would have failed with no exception
entry to explain it.

The themed ones — `markdown`, `pane_grid`, `qr_code`, `table`, `canvas` — are
added. **Four of those five are feature-gated in `iced_widget`**, so the
showcase's `iced` dev-dependency must add `canvas`, `markdown` and `qr_code`
to its feature list (`iced-0.14.0/Cargo.toml`: `canvas`, `markdown`,
`qr_code`); `pane_grid` and `table` are unconditional.

The rest are layout and utility wrappers with no visual surface of their own
(`keyed`, `lazy`, `responsive`, `stack`, `pin`, `float`, `overlay`, `sensor`,
`themer`) or need a GPU pipeline an application supplies (`shader`), and are
listed as exceptions in §6a.4 rather than shown.

### 6a.4 Keeping coverage complete

Two mechanisms, because a unit test cannot do both.

**Our own surface: a test.** A test inside the connector reads the showcase
with `include_str!("../examples/showcase-gpui.rs")` — a path known at compile
time — and asserts every `geometry::*` builder and every `variants::*`
function is referenced at least once. A builder nobody demonstrates is a
builder nobody has verified, which is how `geometry::dialog` carried a wrong
radius and `geometry::menu_item` a wrong doc comment for two releases. The
same test for iced covers `styles::*` and `styles::aw::*`.

**The toolkit's surface: a script.** A test *cannot* enumerate the widgets a
dependency offers — it cannot locate that dependency's source, and
gpui-component exposes no list of its widgets to match against. The first
draft of this section claimed otherwise; it was wrong.

`scripts/check-widget-coverage.py` does it instead: `cargo metadata
--format-version 1` gives the exact on-disk source path of `gpui-component`,
`iced_widget` and `iced_aw`, from which the script enumerates the
constructible widgets (types implementing `RenderOnce` or `IntoElement` for
gpui, the source modules for iced), discards test harnesses and internal
sub-parts by the rules in §6a.2, and compares against the showcase. Widgets
not shown must appear in `docs/showcase-exceptions.toml` with a reason;
anything else fails the script.

It runs in two places: `pre-release-check.sh`, so a release cannot ship an
unshown widget, and the nightly dependency canary, so an upstream release that
adds a widget is reported the evening it appears rather than at the next
release.

---

## 7 -- Layer 3: contrast invariants

One test per connector, `#[cfg(test)]`, over all 32 preset/mode combinations.

**The rule is no-degradation, not AA.** Measured 2026-09-20 over 512 pairs:
174 sit below WCAG AA, and they are not all errors. macOS genuinely ships
`success_color = "#34c759"` with white text (`macos-sonoma.toml:21-22`), about
2.2:1; Apple uses it. A connector that asserted AA would be asserting that
every platform meets AA, which is false, and "fixing" it would mean inventing
values the platform did not give — which the repository forbids.

So for each pair the test computes the ratio the *native fields* give and the
ratio the *connector's output* gives, and asserts the connector's is not
worse. Both sides composite any colour with alpha below 1 over its own
background before measuring; skipping that was the first draft's error and
produced 143 phantom failures, including a 1.00 on Windows 11's `#0000000a`
menu hover.

**Asserted where the connector controls both colours, reported where it does
not.** The assertion covers gpui's `ThemeColor` and every `styles::*` output.
iced's *palette* pairs are printed with both ratios and not asserted: iced
paints a text input on the window background whatever the platform's field
colour is, so with an exactly native placeholder the ratio is still lower
than native in 12 of 32, by at most 0.68 (§1). Asserting that would need a
tolerance, and a tolerance is an invented number; the foreground is pinned
exactly by its §5 row instead.

This is exactly the rule that catches the defect it exists for: iced's
placeholder is `#e8e8e8` on `#fafafb` at 1.15:1, where the native pair —
`input.placeholder_color` on `input.background_color` — is comfortably
readable. Our ratio is worse than the platform's, so it fails. And it stays
silent about macOS's green, because there we emit exactly what Apple gives.

The test additionally *prints* every pair below AA without failing on it, so
the list stays visible. Whether any of those is a preset bug rather than a
platform fact is `preset-validator`'s question, not this test's; the ones that
look like data errors are recorded in `docs/todo.md` under Research.

Neither contrast helper is re-implemented. The gpui connector's lives in
`src/derive.rs` — `pub fn contrast_ratio(a: Hsla, b: Hsla) -> f32` at
`derive.rs:54`, with `relative_luminance` at `:62`; `colors.rs` already
imports it (`colors.rs:15`). The iced connector has its own,
`extended.rs:59`, with `relative_luminance` at `:43`. What both lack is alpha
compositing, so the tests composite before calling them.

---

## 8 -- Documentation

| File | Change |
|---|---|
| `connectors/native-theme-iced/README.md` | a "Styles" section mirroring gpui's "Flat buttons": what the palette gives automatically, what `styles::*` gives exactly, the three closure shapes of §3.1 and which setter each goes to, and the feature table of §4.2. Its `font_size` and `from_system` examples take the new shapes |
| `connectors/native-theme-iced/src/lib.rs` | crate docs: text scaling and what the other two preferences cannot reach, the feature list, and the two-layer colour story |
| `docs/todo.md` | the nine iced items of the audit move from "not yet done" to done, except the geometry gap, which stays; `scrollbar.min_thumb_length` is added as unreachable in iced 0.14 |
| `CHANGELOG.md` | see below |

`CHANGELOG.md`, under `## [Unreleased]`:

- **Breaking Changes → native-theme-iced** — `font_size` and `mono_font_size`
  take `&AccessibilityPreferences`, and `from_system` returns them as a fourth
  element, so the user's text-scaling setting reaches an iced application for
  the first time.
- **Breaking Changes → native-theme-iced** — iced's `secondary.base` colour
  now carries the platform's placeholder colour instead of its button
  surface. iced reads that slot as placeholder text in three widgets, and the
  placeholder was between 1.06:1 and 1.84:1 — invisible — on every preset in
  both modes. Applications that relied on `button::secondary` carrying the
  platform surface call `styles::button`, which carries idle, hover, pressed
  and label together.
- **Added** — `native_theme_iced::styles`: per-widget style functions built
  from the resolved theme, for the slots iced's palette cannot carry
  unambiguously.
- **Added** — the four icon features, so an application depending only on
  `native-theme-iced` gets working icon loading.
- **Added** — mapping-contract tests in both connectors: every toolkit slot is
  checked against the native field of the widget that reads it, over all 16
  presets in both modes, with a coverage tripwire so a new slot cannot ship
  without a declared source.
- **Added** — a contrast invariant in both connectors: for every
  text-on-background pair, the connector's output is never less readable than
  the platform's own values. Measured over all 32 preset/mode combinations.
  It is not a WCAG assertion: 174 of 512 pairs sit below AA on real platform
  data, macOS's `#34c759` with white text among them.
- **Added** — showcase self-tests in both connectors, both of which render and
  click headlessly.
- **Added** — both showcases now render every widget their toolkit offers an
  application, including everything gpui-kit 0.6.2 and 0.6.4 introduced, with
  a coverage script, run by the release check and the nightly canary, that
  fails when an upstream release adds a widget nobody shows. The gpui showcase
  previously exercised 15 of the connector's 34 geometry builders and rendered
  no `StatusBar`, `TitleBar` or `Combobox`.

---

## 9 -- Acceptance

- [ ] `cargo test --workspace --all-features` passes, including the contract
      tables, both coverage tripwires, the contrast tests and both showcases'
      self-tests.
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
      clean.
- [ ] Deleting any one row from either contract table makes its coverage
      tripwire fail (run once, per connector, as the negative control).
- [ ] Pointing any one contract row at the field it used to read makes the
      contract test fail on a named preset (run once, per connector).
- [ ] `grep -rn 'placeholder' connectors/native-theme-iced/src/` shows the
      placeholder fed from `input.placeholder_color`, not from a button field.
- [ ] `cargo tree -p native-theme-iced -e no-dev,features -i native-theme`
      lists `material-icons`, `lucide-icons`, `system-icons` and
      `svg-rasterize`. The `no-dev` edge filter is essential: with
      dev-dependencies in the graph the features are already on, which is how
      the defect stayed invisible to every test.
- [ ] `scripts/check-widget-coverage.py` passes for both connectors, and
      removing one widget from a showcase makes it fail (the negative control).
- [ ] The builder-coverage test passes, and deleting one `geometry::` call
      from the showcase makes it fail.
- [ ] `cargo test -p native-theme-iced --features iced_aw` passes, and the
      iced showcase renders the six `iced_aw` widgets (§3a).
- [ ] The iced showcase's `interactive_controls_respond` fails when one
      control's `on_press` is removed (the negative control for §6.2).
- [ ] `./pre-release-check.sh` shows no failures.
