# v0.5.9 — Theme contracts: Specification

Status: Design (2026-09-20); nothing implemented
Companion rationale:
[`todo_v0.5.9_theme-contracts-rationale.md`](todo_v0.5.9_theme-contracts-rationale.md)
(decisions C1–C10)
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
3. `native_theme_iced::to_theme` takes accessibility preferences, and the
   crate forwards the icon features (§4).
4. Both connectors gain a **mapping-contract table**: every slot, the native
   field it must equal, checked over all 32 preset/mode combinations, with a
   coverage tripwire over every slot (§5).
5. Both showcases gain self-tests (§6).
6. Both connectors gain WCAG AA contrast invariants (§7).

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

Verified 2026-09-20 against the published sources.

| Fact | Evidence |
|---|---|
| Every iced widget takes `.style(impl Fn(&Theme, Status) -> Style)` | `iced_widget-0.14.2/src/button.rs:184`, `text_input.rs:278`, `checkbox.rs:236` |
| iced's default `text_input` style reads `placeholder: palette.secondary.base.color` and `selection: palette.primary.weak.color` | `text_input.rs:1769, 1771` |
| The iced connector writes `secondary.base.color = button.background_color` | `connectors/native-theme-iced/src/extended.rs:109` |
| `button::secondary`'s hover reads `palette.secondary.strong.color`, which the connector never writes | `button.rs:620`; `extended.rs:109-119` writes only `.base` |
| The pick-list/menu overlay highlight is `palette.primary.strong` | `overlay/menu.rs:657-658` |
| `to_theme` has no accessibility parameter | `connectors/native-theme-iced/src/lib.rs:113-116` |
| The iced connector declares no `[features]`; the gpui one forwards four | `connectors/native-theme-iced/Cargo.toml`; `connectors/native-theme-gpui/Cargo.toml:26-31` |
| An `[[example]]` target runs its `#[cfg(test)]` tests when the manifest sets `test = true`; the default is `false` | Cargo book, *Configuring a target*; both showcase entries currently omit it (`native-theme-gpui/Cargo.toml:80-81`, `native-theme-iced/Cargo.toml:26-27`) |
| iced 0.14 ships no headless renderer (no `iced_test` in the closure) | `ls ~/.cargo/registry/src/*/iced*` |
| The iced `Style` structs this specification fills | `button.rs:489-497`, `text_input.rs:1715-1725`, `checkbox.rs:530-536`, `toggler.rs:510-522`, `scrollable.rs:2273-2281`, `container.rs:464-472`, `slider.rs:591-593`, `progress_bar.rs:250-254`, `overlay/menu.rs:600-610` |

---

## 2 -- The iced palette: what changes

`src/extended.rs`, `apply_overrides`.

| Line today | Change | Why |
|---|---|---|
| `secondary.base.color = btn_bg` | **removed** | iced reads this slot as placeholder text in three widgets (§1). Leaving iced's generated value restores a readable placeholder; the button surface is delivered by `styles::button` instead. |
| `secondary.base.text = btn_fg` | kept | the slot is text, the value is text |
| — | **add** `secondary.strong.color = button.hover_background` | `button::secondary` reads `.strong` for its hover, and this is the one slot where `.strong` means exactly "hover" |
| `background.weak.color = surface` | kept | the closest single meaning iced has for a subdued panel; the nine readers that want something else are served by `styles::*` |
| the four `ensure_status_contrast` lines | kept | correct as written |

`OverrideColors` gains `btn_hover_bg: Rgba` and loses nothing (`btn_bg` is
still used by `styles::button`, which takes the resolved theme directly, so
the field is dropped from the struct if nothing else reads it — the compiler
decides).

No other palette slot changes. Everything else moves to §3.

---

## 3 -- `native_theme_iced::styles`

A new public module, `src/styles.rs`. Each function takes `&ResolvedTheme`
and returns a closure iced accepts in `.style(..)`. Pure; no global state; the
returned closure ignores the `&Theme` argument, because the values are already
resolved.

```rust
pub fn button(resolved: &ResolvedTheme)
    -> impl Fn(&Theme, button::Status) -> button::Style + use<>;
```

The signature shape is the same for every widget below. Each closure owns the
handful of `Color` values it needs, captured by value, so it is `'static` and
can be stored in a widget.

| Function | iced `Style` fields it fills | Native source |
|---|---|---|
| `button` | `background`, `text_color`, `border`, `shadow` | `button.background_color` / `.font.color` / `.border.*`; `hover_background`, `active_background` per `Status` |
| `button_primary` | as above | `button.primary_background`, `primary_text_color` |
| `text_input` | `background`, `border`, `icon`, `placeholder`, `value`, `selection` | `input.background_color`, `.border.*`, `.placeholder_color`, `.font.color`, `.selection_background`; `hover_border_color` and `focus_border_color` per `Status` |
| `checkbox` | `background`, `icon_color`, `border`, `text_color` | `checkbox.checked_background`, `.check_color`, `.unchecked_border_color`, `.border.*`, `.font.color` |
| `toggler` | `background`, `background_border_*`, `foreground`, `foreground_border_*`, `text_color` | `switch.unchecked_background`, `checked_background`, `thumb_background`, `hover_*` per `Status` |
| `scrollable` | `container`, `vertical_rail`, `horizontal_rail` | `scrollbar.track_color`, `thumb_color`, `thumb_hover_color`, `thumb_active_color`, `groove_width`, `thumb_width`, `min_thumb_length` |
| `menu` | `background`, `border`, `text_color`, `selected_text_color`, `selected_background` | `menu.background_color`, `.border.*`, `.font.color`, `.hover_text_color`, `.hover_background` |
| `container_card` | `background`, `border`, `shadow`, `text_color` | `card.background_color`, `.border.*` |
| `slider` | `rail`, `handle` | `slider.track_color`, `fill_color`, `thumb_color`, `track_height`, `thumb_diameter` |
| `progress_bar` | `background`, `bar`, `border` | `progress_bar.track_color`, `fill_color`, `.border.*` |
| `tooltip` | via `container::Style` | `tooltip.background_color`, `.border.*`, `.font.color` |

Widgets whose native fields the model does not carry (`Tag`, `Badge`,
`Rule` beyond its colour) are not covered; the palette serves them.

Each function's doc comment names the iced default it replaces and the slot
whose meaning it corrects, as `variants::ghost_button` does in the gpui
connector.

---

## 4 -- Accessibility preferences and features (iced)

### 4.1 `to_theme`

```rust
pub fn to_theme(
    resolved: &ResolvedTheme,
    name: &str,
    prefs: &AccessibilityPreferences,
) -> iced_core::theme::Theme
```

`from_preset` and `from_system` gain the same parameter, matching the gpui
connector's signatures. What the preferences change in iced:

| Preference | Effect |
|---|---|
| `text_scaling_factor` | multiplies `font_size()` and `mono_font_size()` (`lib.rs:259, 274`), the two values an iced application sets on its widgets |
| `reduce_transparency` | any colour the connector emits with alpha below 1 is composited against its background and emitted opaque |
| `reduce_motion` | recorded and exposed as `reduce_motion(prefs) -> bool`; iced has no global animation flag, so the application decides |

`AccessibilityPreferences` is re-exported from the crate root, as gpui does.

### 4.2 Features

`connectors/native-theme-iced/Cargo.toml` gains the four icon features and the
same default set as the gpui connector:

```toml
[features]
default = ["material-icons", "lucide-icons", "system-icons", "svg-rasterize"]
material-icons = ["native-theme/material-icons"]
lucide-icons = ["native-theme/lucide-icons"]
system-icons = ["native-theme/system-icons"]
svg-rasterize = ["native-theme/svg-rasterize"]
```

---

## 5 -- Layer 1: the mapping contract

One new file per connector: `src/contract.rs` (gpui) and `src/contract.rs`
(iced), each `#[cfg(test)]` only.

### 5.1 The table

```rust
/// One row of the mapping contract: a toolkit slot, the native field it must
/// equal, and why, if it may legitimately differ anywhere.
struct Row {
    slot: &'static str,
    native: fn(&ResolvedTheme) -> Rgba,
    get: fn(&ThemeColor) -> Hsla,
}
```

The test iterates every row over all 16 presets in both modes and asserts
equality. A row that must differ on some preset carries the reason in a
comment and the preset in an exception list — not a loosened assertion.

Rows are the fields with a native counterpart, including the ones this release
corrected: `accent` ← `menu.hover_background`, `accent_foreground` ←
`menu.hover_text_color`, `sidebar_accent*` ← the sidebar's selection pair,
`secondary_hover` ← `button.hover_background`, `list_hover`, `list_active`,
`selection`, `input`, `primary`, the status colours, the scrollbar colours,
the tab colours.

This replaces the two bespoke tests the sibling release added for `accent`
(C10): the contract table is where that claim belongs.

### 5.2 The coverage tripwire

```rust
#[test]
fn every_theme_color_field_has_a_declared_source() { … }
```

Every field of `ThemeColor` appears in exactly one of three lists:

1. the contract table (§5.1) — a native field;
2. `DERIVED` — with the derivation named (`hover_color(primary)`,
   `light_variant(..)`, a blend);
3. `UPSTREAM_DEFAULT` — with the reason (`transparent`, the 12 private
   palette colours).

The test asserts the three lists partition the 138 fields exactly: no field
missing, none in two lists. A new upstream field therefore fails the build
until someone classifies it, exactly as the `Theme` shape tripwire (sibling
E17) does for shape.

The iced file does the same over the palette slots the connector writes and
over every `styles::*` function's output.

---

## 6 -- Layer 2: showcase self-tests

Both manifests gain `test = true` on the example target.

### 6.1 gpui (`examples/showcase-gpui.rs`, `#[cfg(test)] mod tests`)

| Test | Asserts |
|---|---|
| `every_tab_lays_out` | each of the ten tabs renders on GPUI's test platform and `debug_bounds` finds the tab root; no panic |
| `resizable_groups_have_room_to_drag` | for each resizable group in the file, the container's size exceeds `panels × PANEL_MIN_SIZE` plus its borders. This is the finding of §1.1 stated as a rule |
| `interactive_controls_respond` | for the controls the showcase advertises as interactive, a simulated click produces the effect: the Copy button writes the field's text to the clipboard (`cx.read_from_clipboard()`), the mode switch changes `Theme::mode`, the notification buttons push a notification |

### 6.2 iced (`examples/showcase-iced.rs`, `#[cfg(test)] mod tests`)

iced 0.14 has no headless renderer (§1), so these build and inspect rather
than draw:

| Test | Asserts |
|---|---|
| `view_builds_for_every_tab` | `view()` returns an `Element` for each tab with no panic |
| `every_button_has_a_message` | every `button(..)` the showcase builds carries `.on_press(..)`; an iced button without one renders disabled, which is the iced form of the dead-control bug |
| `styles_cover_every_widget_shown` | every widget the showcase renders is styled with the `styles::*` function for it — no widget is left on the palette default, because the showcase is what the README's screenshots claim the connector achieves (rationale §2.7) |

---

## 7 -- Layer 3: contrast invariants

One test per connector, `#[cfg(test)]`, over all 32 preset/mode combinations.

For every (foreground, background) pair the connector produces — body text on
background, muted text on background, button label on button surface,
placeholder on input background, menu hover text on menu hover background,
list selection text on list selection background, each status foreground on
its status colour, link on background — assert `contrast_ratio >= 4.5`.

Exceptions are listed by name with a reason, not by loosening the threshold:

- disabled text, which is deliberately below AA on every platform;
- any pair where the platform's own two values already fail, which the
  connector must not silently "fix" — those are reported as findings against
  the preset, and belong to `preset-validator`, not here.

The gpui connector already has `contrast_ratio` (`src/colors.rs:55-76`); the
iced connector has its own in `extended.rs`. Neither is re-implemented.

---

## 8 -- Documentation

| File | Change |
|---|---|
| `connectors/native-theme-iced/README.md` | a "Styles" section mirroring gpui's "Flat buttons": what the palette gives automatically, what `styles::*` gives exactly, and the one-line example |
| `connectors/native-theme-iced/src/lib.rs` | crate docs: the accessibility parameter, the feature list, and the two-layer colour story |
| `docs/todo.md` | the nine iced items of the audit move from "not yet done" to done, except the geometry gap, which stays |
| `CHANGELOG.md` | see below |

`CHANGELOG.md`, under `## [Unreleased]`:

- **Breaking Changes → native-theme-iced** — `to_theme`, `from_preset` and
  `from_system` take `&AccessibilityPreferences`, so text scaling and reduced
  transparency reach an iced application for the first time.
- **Breaking Changes → native-theme-iced** — the palette no longer writes the
  platform's button surface into the slot iced reads as placeholder text. On
  Adwaita that placeholder was `#e8e8e8` on a `#fafafb` field, about 1.15:1
  and invisible. Applications that relied on `button::secondary` carrying the
  platform surface call `styles::button` instead.
- **Added** — `native_theme_iced::styles`: per-widget style functions built
  from the resolved theme, for the slots iced's palette cannot carry
  unambiguously.
- **Added** — the four icon features, so an application depending only on
  `native-theme-iced` gets working icon loading.
- **Added** — mapping-contract tests in both connectors: every toolkit slot is
  checked against the native field of the widget that reads it, over all 16
  presets in both modes, with a coverage tripwire so a new slot cannot ship
  without a declared source.
- **Added** — WCAG AA contrast tests over every text-on-background pair both
  connectors produce.
- **Added** — showcase self-tests in both connectors.

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
- [ ] `cargo tree -p native-theme-iced -i native-theme` shows the icon
      features enabled by default.
- [ ] `./pre-release-check.sh` shows no failures.
