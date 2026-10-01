# Roadmap

Pre-1.0 milestones. Priorities can shift between minor versions — this roadmap
is a snapshot of current direction, not a commitment.

See also:

- [CHANGELOG.md](CHANGELOG.md) — what has already shipped
- [`docs/archive/`](docs/archive/) — detailed design docs and phase notes from completed milestones

## v0.6.0 — egui connector

**Delivered:** `native-theme-egui`. egui holds one global `Style` per colour scheme, whose
widget appearance varies by interaction state and not by widget type, whereas
`ResolvedTheme` carries a struct per widget; the connector pre-builds a `Style` per widget
role and per appearance variant (resting, selected, disabled) and applies them through
egui's own seams (`UiBuilder::style`, `Ui::set_style`, `StyleModifier`, `Frame`), so
per-widget geometry survives the contested-field collisions. Coverage is `mapping.toml`,
one row per theme value, which the crate's tests hold to `ResolvedTheme` and to every
published `Style`. Targets egui 0.36.2.

Detailed design: [`docs/archive/todo_v0.6.0_egui-connector-spec.md`](docs/archive/todo_v0.6.0_egui-connector-spec.md),
[`docs/archive/todo_v0.6.0_egui-connector-rationale.md`](docs/archive/todo_v0.6.0_egui-connector-rationale.md)
and the implementation plan,
[`docs/archive/todo_v0.6.0_egui-connector-plan.md`](docs/archive/todo_v0.6.0_egui-connector-plan.md).

**Also delivered:** `native-theme-egui-widgets`, egui widgets the connector cannot give
egui's own (a switch, a slider knob, a spinner stroke, a segmented control, links, a radio
dot, a progress-bar outline, an expander), and the gpui connector on gpui-component /
gpui-base 0.7.0 and gpui-pre 0.3.7
([`docs/archive/todo_v0.6.0-rc1_gpui-kit-0.7-spec.md`](docs/archive/todo_v0.6.0-rc1_gpui-kit-0.7-spec.md)).

## v0.6.1 — Full theme geometry in the iced connector

v0.5.9 ships most of it. `native_theme_iced::styles` (feature `widgets`, on by
default) replaces nineteen of iced's style functions with closures built
from the resolved theme: every `Style` field — colours, border radius and
width included — of the button (neutral, primary, danger, success, warning
and link classes), text input, text editor, checkbox, radio, toggler, pick
list, menu, slider, scrollable, progress bar, rule, tooltip and a card
container; its twentieth item, `styles::scrollbar`, is the scrollbar's
configuration, its widths and embedding. `styles::aw` (feature `iced_aw`)
does the same for `iced_aw`'s card, menu bar, tab bar, sidebar, selection
list and spinner, and `styles::segmented_control`, `segment` and `expander`
style the containers and buttons the showcase builds those widgets from. Padding, which iced takes through each widget's builder
rather than its `Style`, comes from `button_padding`, `input_padding`, `text_area_padding`,
`combo_box_padding` and, for any other widget, `padding_or`,
`padding_inside_border` and `stated_padding`, each keeping iced's own
default for a side the theme does not state. A `Style` field the model does
not carry is read from iced's default for that widget, and the twelve theme
values iced 0.14 and `iced_aw` 0.14.1 have no receiver for (and one emitted
field, `styles::aw::card.close_color`) are listed, with
the upstream lines that show it, in the connector's mapping contract.

**What remains** of the plan:

- replacements for the iced classes it lists that still paint from the
  palette: `button::background`, `checkbox::{secondary, success, danger}`,
  `progress_bar::{secondary, success, warning, danger}`,
  `container::{rounded_box, dark, primary, secondary, success, warning,
  danger}` and `pane_grid::default`. `container::bordered_box` and
  `button::subtle` stay iced's on purpose: they paint `background.weakest`,
  a colour iced derives and no platform states;
- helpers for the sizes iced takes through widget builders
  (`Checkbox::size`, `ProgressBar::girth`, a rule's thickness; a switch's
  and a radio's sizes already come through `switch` and `radio`), which an application reads from the `ResolvedTheme` itself
  today, and a helper that sets iced's `Settings` default font and text size
  from the theme's font;
- shadows: the model carries a shadow colour but no offset or blur, so every
  style keeps iced's own `shadow`.

Detailed design: [`docs/todo_iced-full-theme-geometry.md`](docs/todo_iced-full-theme-geometry.md).

## v0.6.2 — Upstream receivers for the gpui connector

v0.5.8 delivered the connector-side geometry: every widget where
gpui-component 0.6 and later applies the caller's `StyleRefinement` after its own
geometry now takes native heights, paddings, radii, borders and text sizes
through the `geometry` module, and the base layer (scrollbar, resize handle)
takes native values through `base_layer` with automatic re-application. What
remains is geometry on **inner elements the caller's style cannot reach**,
plus the tab properties `Tab`'s render overwrites in the shared style bag,
which is upstream work. v0.6.2 is the set of PRs to gpui-kit, one concern
each, framed as theming flexibility:

- a styled `Theme` scrollbar-style override honoured by `base_theme()`, so
  the connector's observer becomes unnecessary;
- `Tab` keeping the caller's height, radius and text size (its render writes its own into the style bag the caller's setters fill, `tab/tab.rs:801-808` at gpui-component 0.7.0);
- `Theme.shadow` honoured beyond `Button`, and `tokens.shadow` consumed;
- `Size::Size` honoured by `Checkbox` and `Switch`;
- inner geometry exposed: checkbox/radio indicator, switch track and thumb,
  slider track and thumb, separator thickness, resize-handle width, button
  icon gap, popup-menu items, select arrow, accordion arrow;
- a `PopupMenu` item style hook and a `Button::tooltip` style hook;
- button label text size independent of rem;
- an iterable `IconName::ALL` generated by `icon_named!`;
- public base-palette fields on `ThemeConfigColors` (`blue` … `yellow_light`,
  `schema.rs:643-677` at gpui-component 0.7.0), so a config can carry the whole palette and the
  connector's palette repair becomes unnecessary.

Gap analysis with citations: [`docs/todo_gpui-full-theme.md`](docs/todo_gpui-full-theme.md);
limits table: `docs/archive/todo_v0.5.8_gpui-component-0.6-spec.md` §14.

## Beyond v0.6

No milestone targets committed yet. Likely candidates:
- slint, other connectors
- Expanded preset coverage (platform themes for more macOS / Windows versions)
- Additional icon-set bundles if demand emerges
