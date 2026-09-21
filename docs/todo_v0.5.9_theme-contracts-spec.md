# v0.5.9 — Theme contracts: Specification

Status: Design (2026-09-20, revised 2026-09-21); nothing implemented
Companion rationale:
[`todo_v0.5.9_theme-contracts-rationale.md`](todo_v0.5.9_theme-contracts-rationale.md)
(decisions C1–C19)
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
| † A hovered `button::secondary` keeps the base label and swaps only the fill for `secondary.strong.color`. Today that label is below 3:1 in 24 of 32 combinations (minimum 1.20). With `secondary.base` fed from the placeholder and `.strong` left generated it is still below 3:1 in 2 (2.54, 2.66); with `.strong` a copy of `.base` it is the idle label, never below 4.16. `secondary.weak` has no reader | `button.rs:613-625`; measured 2026-09-21 |
| † `button.background_color` equals `input.placeholder_color` in none of the 32 combinations, so the corrected row fails on all of them today | measured 2026-09-21 |
| † The iced showcase already renders `radio` (3), `text_editor` (1), `pick_list` (4), `combo_box` (1), `rule` (35) and `button::danger` / `success` / `text` (3 / 2 / 1) | `examples/showcase-iced.rs`, counted 2026-09-21 |
| † `Pair::new(color, text)` is public and picks a readable text colour by iced's own rule; over the 32 combinations the label it gives on `input.placeholder_color` never falls below 4.16:1 | `iced_core-0.14.0/src/theme/palette.rs:440-445`; measured |
| † 25 model fields are `soft_option`: they stay `Option` in `ResolvedTheme`. 19 of them are read by §3.3. None is `None` in any bundled preset, but a live OS reader may produce one | `native-theme-derive/src/gen_structs.rs:42`; `native-theme/src/model/widgets/mod.rs`; measured |
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
| `extended.secondary.base.color = to_color(colors.btn_bg)` and `extended.secondary.base.text = to_color(colors.btn_fg)` | **replaced** by `extended.secondary.base = Pair::new(to_color(colors.placeholder), extended.background.base.text)`, followed by `extended.secondary.strong = extended.secondary.base` | the slot has six readers and three meanings (§1), and the text meaning wins because it is the one a wrong value makes unreadable. Measured, the placeholder foreground becomes exactly the platform's on all 32 combinations. The three fill readers get the placeholder tone, which is what iced's own design gives them, labelled by iced's own readable-text rule. |
| — | **not** `secondary.strong.color = button.hover_background`, and **not** iced's generated `.strong` | the hover keeps the base label (`button.rs:620-623`), so its fill must be one that label was chosen for. The platform's hover is not, and measured, neither is the generated one (2.54 on dracula dark). A copy is: the project's rule for a state with no value of its own (C16). `styles::button` carries the platform's real idle, hover, pressed and label together. |
| — | **not** "leave iced's generated value", which the second draft chose | measured worse than the platform's own pair in 22 of 32 (§1). |
| `background.weak.color = surface`, `background.weak.text = foreground` | kept | overridden as a *pair*, so they stay coherent; the closest single meaning iced has for a subdued panel. The nine readers that want something else are served by `styles::*`. |
| `primary.base.text = accent_fg` | kept | `primary.base.color` comes from the `Palette` itself (`palette.rs:41`), so this pair is native on both sides. |
| the three `ensure_status_contrast(..)` lines for `success` / `danger` / `warning` `.base.text` | **replaced** by the plain native `*_text_color`; `ensure_status_contrast`, `MIN_STATUS_CONTRAST` and their tests are deleted (C19) | measured: of 37 labels below 4.5:1 the function picks the wrong one of white and black in 31, makes 7 worse than the platform's own, and is a no-op on every real platform (rationale §2.13). `contrast_ratio` and `relative_luminance` stay, under `#[cfg(test)]`, for §7 |

`OverrideColors` loses `btn_bg` and `btn_fg`, and the three `*_bg` fields that
only the enforcement read — leaving any of them fails `-D warnings` on
`dead_code` — and gains `placeholder: Rgba`, filled from
`resolved.input.placeholder_color`.

The gpui connector makes the same removal in `src/colors.rs:50-76` and
`assign_status`; `derive::contrast_ratio` is `pub` and stays.

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
closure uses its `&Theme` argument for one thing only — asking iced for its
own default where the native model has no value (§3.2) — because every native
value is already resolved. Each closure owns the handful of `Color` values it needs, captured
by value, so it is `'static` and can be stored in a widget.

### 3.1 Three closure shapes, not one

The first draft said every function had the same signature. It does not
(§1). There are three shapes, and each function's doc comment names which
setter it is passed to.

```rust
// A. with Status, passed to .style(..)
//    button, button_primary, button_danger, button_success, button_warning,
//    button_link, text_input, text_editor, checkbox, radio, toggler,
//    pick_list, scrollable, slider (also serves vertical_slider)
pub fn button(resolved: &ResolvedTheme)
    -> impl Fn(&Theme, button::Status) -> button::Style + use<>;

// B. without Status, passed to .style(..)
//    container_card, progress_bar, rule, tooltip (returns container::Style)
pub fn container_card(resolved: &ResolvedTheme)
    -> impl Fn(&Theme) -> container::Style + use<>;

// C. without Status, passed to .menu_style(..) on PickList / ComboBox.
//    ComboBox takes no .style(..): its field is .input_style(styles::text_input(..))
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

> **A `Style` field the native model does not carry is read, inside the
> closure, from iced's own public default style function for that widget and
> status. It is never written as a literal — not ours, and not a copy of
> iced's.**

```rust
move |theme, status| {
    let iced = toggler::default(theme, status);   // iced's own answer
    toggler::Style {
        background: /* native */,
        // … every field named; the ones with no native source read `iced`:
        border_radius: /* native: switch.track_radius */,
        padding_ratio: /* native: (track_height - thumb_diameter) / (2 * track_height) */,
    }
}
```

The first draft of this rule said "copy iced's value and cite the line". That
would have put `0.1` and a hand-rebuilt `AutoScroll` into `src/`, where the
repository's hook rejects hardcoded values, and it would have gone stale the
day iced retuned a default. Every function needed is public:
`text_input::default` (`:1758`), `toggler::default` (`:561`),
`scrollable::default` (`:2345`), `slider::default` (`:676`),
`checkbox::primary` (`:569`), `progress_bar::primary` (`:287`),
`overlay::menu::default` (`:646`), and `button::Style::default()` /
`container::Style::default()` for the two structs that have one. Construction
stays **exhaustive** — every field is named, so a field added upstream still
fails the build and gets classified by a person.

The fields this release knows have no native source:

| Field | Why the model cannot supply it |
|---|---|
| `button::Style.snap`, `container::Style.snap` | a renderer setting, `cfg!(feature = "crisp")` (`button.rs:517`) — a literal `false` would switch crisp rendering off for a consumer who enabled it |
| `button::Style.shadow`, `container::Style.shadow`, `menu::Style.shadow` | the model has `defaults.shadow_color` and `border.shadow_enabled` but no offset or blur, so an iced `Shadow` cannot be built without inventing geometry |
| `scrollable::Style.gap`, `.auto_scroll`, and every field of its nested `.container` | no native counterpart: `ScrollbarTheme` states the bars, not the scrolled surface. iced's own default leaves the container transparent so the content shows through, and `container::Style` has a `Default`, so the function still names every one of its fields (the compiler would not notice an added one) |
| `slider` `handle.background` in `Status::Dragged` | `SliderTheme` has `thumb_color` and `thumb_hover_color` and no pressed thumb colour; the shape and the rail stay native |
| `toggler::Style`'s four border fields and `.text_color` | `SwitchTheme` carries no border and no font. (`border_radius` and `padding_ratio` **do** have sources — `switch.track_radius`, and the inset `track_height` and `thumb_diameter` state between them — an earlier draft of this row listed both here in error) |
| `slider` `rail.border`, `handle.border_width`, `handle.border_color`; the scrollable rails' `border` | `SliderTheme` and `ScrollbarTheme` carry no border |
| `text_input::Style.icon` | the model has no input-icon colour |
| `container::Style.text_color` for `container_card` | `CardTheme` carries no font; iced's `None` inherits |
| `pick_list::Style.handle_color` | `ComboBoxTheme` has `arrow_icon_size` and `arrow_area_width` but no arrow colour |
| `rule::Style.radius`, `.fill_mode`, `.snap` | `SeparatorTheme` is a colour and a width |

**Soft options (C16).** Nineteen of the native fields below are
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
| `combo_box.hover_background`, `combo_box.disabled_background` | `combo_box.background_color` |
| `tab.hover_background` (§3a) | `tab.background_color` |

**State layers (C17).** Fourteen of these colours are translucent on some
platform. What to emit depends on what the colour *is*, and the platform
documents it (rationale §2.11):

| Kind | Fields | Emitted as |
|---|---|---|
| hover and pressed **of a widget that paints its own fill** | `button.hover_background`, `button.active_background`, `checkbox.hover_background`, `switch.hover_*`, `combo_box.hover_background`, `link.hover_background` | **composited over that widget's idle fill** — the platform layers, iced replaces. For an opaque value this is the identity, so there is no branch |
| idle and disabled fills | `*.background_color`, `*.disabled_background`, `checkbox.unchecked_background` | as given: a translucent disabled fill *replaces* the idle one and lets the window through |
| row highlights and thumbs | `menu.hover_background`, `list.*`, `sidebar.*`, `tab.*`, the scrollbar thumb colours | as given: iced paints them over a panel or rail it also paints, so the layering happens by itself |

Computed for Windows 11 light: the platform's hovered button is `#f3f3f3`; the
raw value in a replacing toolkit gives `#e9e9e9`.

### 3.3 The functions

| Function | Shape | Every `Style` field it fills | Native source |
|---|---|---|---|
| `button` | A | `background`, `text_color`, `border`, `shadow`, `snap` | `button.background_color` / `.font.color` / `.border.*`; `hover_background`, `active_background`, `hover_text_color`, `active_text_color`, `disabled_*` per `Status` |
| `button_primary` | A | as above | idle fill `button.primary_background`, idle label `primary_text_color`, **border from `button.border`**, disabled from `button.disabled_*`. Hover and pressed (fill and label) have no native source — the model has no primary hover, and the neutral `button.hover_background` is an opaque grey on 15 of 16 presets — so they come from iced's `button::primary(theme, status)`, which derives them from the palette's primary. Measured 2026-09-21: `button.primary_background == defaults.accent_color` and `primary_text_color == accent_text_color` in 32 of 32, so that derivation starts from exactly the idle fill; the contract pins the equality |
| `button_danger`, `button_success`, `button_warning` | A | as above | fill `defaults.danger_color` / `success_color` / `warning_color`, label the matching `*_text_color`, **border from `button.border`** — iced's own status buttons hardcode `border::rounded(2)` (`button.rs:739`) and would sit beside a native button with a different radius. Hover and pressed fills have no native source: iced's `button::danger(theme, status)` etc., which derive them from the native status colour already in the palette |
| `button_link` | A | as above | `link.font.color`, `.hover_text_color`, `.active_text_color`, `.disabled_text_color`, `.background_color`, `.hover_background`. Replaces `button::text` |
| `text_input` | A | `background`, `border`, `icon`, `placeholder`, `value`, `selection` | `input.background_color`, `.border.*`, `.placeholder_color`, `.font.color`, `.selection_background`; `hover_border_color` and `focus_border_color` per `Status`; `disabled_text_color` for a disabled `value`; `icon` from §3.2 |
| `text_editor` | A | `background`, `border`, `placeholder`, `value`, `selection` | the `input.*` sources of `text_input`; the struct has no `icon` |
| `checkbox` | A | `background`, `icon_color`, `border`, `text_color` | `checkbox.checked_background`, **`.indicator_color`** (the check mark; there is no `check_color`), `.unchecked_background`, `.unchecked_border_color`, `.border.*`, `.font.color`, and `.disabled_text_color` for a disabled label |
| `radio` | A | `background`, `dot_color`, `border_width`, `border_color`, `text_color` | `CheckboxTheme`, which the model documents as shared by checkbox and radio (`widgets/mod.rs:138-140`): `checked_background` / `unchecked_background` by `is_selected`, `indicator_color`, `border.*` / `unchecked_border_color`, `font.color`. `radio::Status` has no disabled value (iced 0.14), so `disabled_text_color` has no receiver there |
| `toggler` | A | `background`, `background_border_width`, `background_border_color`, `foreground`, `foreground_border_width`, `foreground_border_color`, `text_color`, `border_radius`, `padding_ratio` | `switch.unchecked_background`, `checked_background`, `thumb_background`, `hover_checked_background`, `hover_unchecked_background`, `disabled_*` per `Status`; `border_radius` from `switch.track_radius`; `padding_ratio` = `(track_height − thumb_diameter) / (2 × track_height)`, the unit conversion iced's receiver forces (`toggler.rs:444, 453-454`: `padding = ratio × height`, thumb side `= height − 2 × padding`) — emitted when `track_height > 0` and `0 ≤ thumb_diameter ≤ track_height`, iced's own value otherwise; the border fields and `text_color` from §3.2. Measured over the presets: 0.2 on windows-11, twice iced's 0.1; 0 on kde-breeze, whose thumb is as tall as its track |
| `pick_list` | A | `text_color`, `placeholder_color`, `handle_color`, `background`, `border` | `combo_box.font.color`, `input.placeholder_color`, `combo_box.background_color`, `.border.*`, `hover_background` per `Status`; `handle_color` has no native source — `ComboBoxTheme` carries arrow *sizes* but no arrow colour — so it is iced's own (§3.2) |
| `scrollable` | A | `container`, `vertical_rail`, `horizontal_rail`, `gap`, `auto_scroll` | `scrollbar.track_color` → each rail's `background`; `thumb_color`, `thumb_hover_color`, `thumb_active_color` → the `Scroller` background per `Status`; the last two fields from §3.2 |
| `scrollbar` | D | — (a `Scrollbar`, not a `Style`) | `scrollbar.groove_width` → `.width(..)`, `scrollbar.thumb_width` → `.scroller_width(..)`. `scrollbar.overlay_mode == false` → `.spacing(0)`: iced's only switch between a floating and an embedded scrollbar is that setter (`scrollable.rs:378-383`: an embedded one "will always be displayed, will take layout space, and will not float over the contents"), and the model states no gap, so none is added. `overlay_mode == true` leaves iced's floating default. The gpui connector already honours the same field (`gpui/src/lib.rs:178`). `Scrollbar`'s fields are private with no getters, so the contract compares whole values through its derived `PartialEq` |
| `menu` | C | `background`, `border`, `text_color`, `selected_text_color`, `selected_background`, `shadow` | `menu.background_color`, `.border.*`, `.font.color`, `.hover_text_color`, `.hover_background` |
| `container_card` | B | `text_color`, `background`, `border`, `shadow`, `snap` | `card.background_color`, `.border.*`; `text_color` and `snap` from §3.2 |
| `slider` | A | `rail` (`backgrounds`, `width`, `border`), `handle` (`shape`, `background`, `border_width`, `border_color`) | `slider.fill_color` and `track_color` → `rail.backgrounds`; `track_height` → `rail.width`; `thumb_color`, `thumb_hover_color` → `handle.background`; `thumb_diameter` → `handle.shape` |
| `rule` | B | `color`, `radius`, `fill_mode`, `snap` | `separator.line_color`; the other three have no native source (§3.2). The thickness is the constructor's argument, not a `Style` field: the showcase passes `resolved.separator.line_width` where it writes `rule::horizontal(1)` today |
| `progress_bar` | B | `background`, `bar`, `border` | `progress_bar.track_color`, `fill_color`, `.border.*` |
| `tooltip` | B | a `container::Style`: `text_color`, `background`, `border`, `shadow`, `snap` | `tooltip.background_color`, `.border.*`, `.font.color` |

Twenty items: nineteen style functions and `scrollbar` (C18). The rule for
membership: an `iced_widget` widget with a `Style` **and** a native theme
that models it. `svg` and `text` styles carry a single colour an application
already sets from the resolved theme, and get no function.

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

Six functions, from eight `iced_aw` features. `tabs` is the tabbed container
built on `tab_bar` (its feature literally is `tabs = ["tab_bar"]`), so the two
share one function. `context_menu` needs none: an earlier draft said it shares
`menu`'s styling, and the source says otherwise — `ContextMenu` has its own
`Style` with one field, the backdrop scrim (`style/context_menu.rs:9-22`), its
default class already emits alpha 0, platforms paint no scrim, and the popup's
content is the consumer's own element. All eight feature names were checked
against the published feature table on 2026-09-21.

Covered widgets, each from the native theme that models it:

| `styles::aw::*` | `iced_aw` widget | Native source |
|---|---|---|
| `card` | `Card` | `card.background_color`, `.border.*`; head/body/foot from `card` and `defaults` — `CardTheme` carries only `background_color` and `border`, so the rest comes from `defaults` |
| `menu` | `Menu` / `MenuBar` | `menu.background_color`, `.hover_background`, `.border.*`. `menu_bar::Style` has **no** text colour, so `menu.hover_text_color` and `.font.color` have no receiver here (the items are the consumer's own elements) |
| `tab_bar` | `TabBar`, `Tabs` | `tab.background_color`, `.active_background`, `.active_text_color`, `.hover_background`, `.bar_background` |
| `sidebar` | `Sidebar` | `sidebar.background_color`, `.selection_background`, `.selection_text_color`, `.hover_background` |
| `spinner` | a `Container` wrapping `Spinner` | `spinner.fill_color` → the container's `text_color`. `iced_aw` 0.14.1's `Spinner` has no `Style`, no `Catalog` and no style setter: it paints with the renderer's inherited text colour (`spinner.rs:151`), which a wrapping container supplies. So the function is a `container::Style` (shape B), used as `container(Spinner::new()).style(styles::aw::spinner(&resolved))`. The geometry fields have builder receivers and are the consumer's (there is no `spinner.color`) |
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
`secondary_hover` ← `button.hover_background` (raw — its readers are
transparent-idle), `button_hover` and `button_secondary_hover` ← the same
layer **composited over `button.background_color`**, likewise the two
`*_active` tokens (C17; `colors.rs:325, 329` copy the raw value today), `list_hover`, `list_active`,
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
| `styles_cover_every_widget_shown` | a source-level count over the showcase file (`include_str!`): for each constructor that has a `styles` function — `button(`, `text_input(`, `text_editor(`, `checkbox(`, `radio(`, `toggler(`, `pick_list(`, `scrollable(`, `slider(`, `vertical_slider(`, `progress_bar(`, `tooltip(`, `rule::` — the number of constructor calls equals the number of matching `styles::` calls, and no `button::primary` / `secondary` / `danger` / `success` / `text` or `container::rounded_box` remains. `container(` is excluded: most containers are layout, not cards. The showcase is what the README's screenshots claim the connector achieves (rationale §2.7) |

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

**Run once already, as a probe (2026-09-21).** 448 gpui pairs: five failed.
Three were `ensure_status_contrast` choosing the wrong one of white and black
— removed by C19 — and two are the menu-hover pair on windows-11 dark and
material dark, where upstream paints menus on the `popover` token and the
platform's menu background differs from its popover background (it does in 30
of 32). Those two are **named exceptions** in the gpui test, with that reason;
both remain above 9:1. The probe's 14 pairs are not the final list, so the
implemented test may find more: each is a finding.

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
silent about macOS's green, because — once C19 removes the enforcement that
was a no-op there anyway — we emit exactly what Apple gives.

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
- **Changed → both connectors** — status labels (`success`, `danger`,
  `warning`, and gpui's `info`) are the platform's own colours. The previous
  contrast enforcement chose between white and black by a 0.5 lightness
  threshold, which picked the worse of the two in 31 of the 37 labels it
  touched and made 7 less readable than the platform's own; on every real
  platform it was a no-op.
- **Fixed → native-theme-gpui** — a filled button's hover and pressed colours
  are composited over the button's own fill. Windows 11 gives them as 4 %
  layers, and gpui-component replaces a background where the platform layers
  it, so a hovered button landed on `#e9e9e9` where Windows draws `#f3f3f3`.
  No other preset changes.
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

## 8a -- As built: where the implementation corrected this document

Measured or read from the vendored sources during implementation
(2026-09-21/22). The sections above are left as they were argued; where they
disagree with this list, this list is what was built. Corrections already made
in place (§3.2's no-source table, §3.3's `button_primary`, `toggler`,
`checkbox`, `radio`, `text_input` and `scrollbar` rows, §3a's `menu`,
`spinner` and `context_menu`) are not repeated.

| Section | Said | Built |
|---|---|---|
| §2 | the palette correction is `secondary.base` / `.strong` and the status labels | also `background.base.text` ← `defaults.text_color`: iced's `readable()` replaces a text colour below 6.0:1, so on solarized (both modes) and tokyo-night light the window text was not the platform's. Same class as C19 — an enforcement overriding the platform's pair, iced's rather than ours. `secondary.base.text` stays iced's `readable()` result against the placeholder fill: measured, it is a substitute on 31 of 32 (ios light the exception), and a contract test holds that count |
| §3.1 | three closure shapes | every closure is also `+ Clone`: an `impl Trait` return leaks only auto traits, and `iced_aw::SelectionList::new_with` requires `Clone` |
| §3.3 | — | native geometry with a *builder* receiver is the consumer's, named in each function's doc comment: `Checkbox::size` / `Radio::size` ← `checkbox.indicator_width` (the indicator's side length or diameter, `platform-facts.md:969`), `::spacing` ← `label_gap`, `Toggler::size` ← `switch.track_height`, `ProgressBar::girth` ← `progress_bar.track_height` |
| §5.2 | the iced file "does the same" | palette: every slot the connector writes is found by diffing against `Extended::generate` and must be a row or a `DERIVED` entry; styles: every `Style` leaf is enumerated by exhaustive destructuring (a new upstream field fails to compile) and must be claimed exactly once; and every declared row and pair is proven to have been *checked* (declared == checked), which the gpui file does too |
| §6.1 | three gpui tests | built as specified, against the real view in the real `Root`. They found two showcase defects: the dialog, sheet and notification layers were never mounted (gpui-component's `Root::render` does not mount them), and the mode selector did nothing when no theme could be read. They run in the fallback environment CI has (no desktop at all) as well as on a configured desktop; the mode assertion is relative for that reason |
| §6.2 | "a button with no `on_press` … its click fails" | inaccurate for `iced_test` 0.14: `Simulator::click` errs only on not-found / not-visible. The test asserts the correct, stronger form — the click resolves, no message is emitted, the state is unchanged. `snapshot()` has no failure path either, so `every_tab_renders` catches a panic in view, layout or draw, plus text laid out at 0×0 — which is how it found the Layout tab's `Grid::height` defect |
| §6.2 | `styles_cover_every_widget_shown` counts | it lexes the showcase (comments, strings and character literals stripped), matches each constructor *expression* to its style call — 95 sites — and also requires the secondary styles (`styles::menu` on pick lists and combo boxes, `styles::scrollbar` on scrollables) and a call site for every public `styles::*` / `styles::aw::*` function |
| §6a.1 | 19 of 34 builders unused | 34 of 34 `geometry` builders and the one `variants` builder are used, held by a test that derives the names from the modules' `pub fn` lines |
| §6a.2 | twenty gpui widgets to add | the script measured 19: `DescriptionList` was already shown. `StatusBar`, `TitleBar`, `Combobox` came with the builders (Task 7), the declarative `Table` with `geometry::table` (`DataTable` is not `Styled`), and sixteen in Task 8. `WindowBorder` is not constructed a second time: `Root` installs one (`root.rs:117`, `:605`), and the section reads `window.client_inset()`, which only `WindowBorder::render` writes |
| §6a.3 | five iced modules | six: `grid` was missing too (the first measurement was a substring match that `grid_rows` satisfied). `iced_aw` adds eight widgets behind `#[cfg(feature = "iced_aw")]`; `typed_input` and `custom_layout` are excepted with reasons verified in the source |
| §6a.4 | one matcher | the iced half strips string literals (measured: it costs no iced match); the gpui half cannot, because eight gpui widgets are built through differently named constructors and are identified by their section label — the script's docstring names them |
| §7 | gpui: two named exceptions | built with one rule for every pair — the emitted side is what upstream really renders, the native side is the platform's own pair, the assertion is no-degradation. A pair that is worse somewhere *and* has no `ThemeColor` token to fix it is **reported** with the reason (status bar, title bar, selected row, selected text), and a reported pair that stops being worse fails the test until it is promoted. Pairs over tokens nothing reads in 0.6.4 are marked inert and not counted as coverage; the summary prints how many asserted pairs can bite on today's presets (two) |
| §7 | — | no window-text pair exists in the iced report: the contract row pins `background.base.text` to `defaults.text_color`, so such a pair would be equal by construction |
| §4.2 | — | the showcase example declares `required-features = ["widgets"]`: a default `cargo test` builds and tests it, `--no-default-features` skips it. A self dev-dependency was measured and rejected — feature unification pulled `default` and `iced_aw` into the `--no-default-features` build |

Shipped defects this work found and fixed in the gpui connector, beyond those
the plan named: `link_hover` / `link_active` held a fill and an arithmetic
derivation where upstream reads link *text*; `list_head`, `table_head` and
`table_head_foreground` took the window's defaults where the model states
`list.header_background` / `header_font`; `tab_bar_segmented` ignored
`SegmentedControlTheme`; six `geometry` builders dropped the platform's text
colour (rationale of the narrowed rule: a builder carries it only where it
reaches the text *and* upstream's state colours still win — `checkbox` and
`combobox` therefore carry none, with guard tests).

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
