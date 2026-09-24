# Roadmap

Pre-1.0 milestones. Priorities can shift between minor versions — this roadmap
is a snapshot of current direction, not a commitment.

See also:

- [CHANGELOG.md](CHANGELOG.md) — what has already shipped
- [`docs/archive/`](docs/archive/) — detailed design docs and phase notes from completed milestones

## v0.6.0 — egui connector

There is no `native-theme-egui` crate yet. egui is the most structurally
difficult connector target so far: it holds a **single global `Style`**, whose
entire widget appearance is five `WidgetVisuals` states of six fields each,
whereas `ResolvedTheme` carries 25 per-widget structs. Expanded to leaves that
is 463 properties competing for one `Style`, and 100 egui fields are claimed by
two or more native-theme widgets at once — `selection.bg_fill` alone has ten
claimants. A connector that only writes the global `Style` can serve 33 of the
463.

**Planned deliverable:** a `native-theme-egui` crate that pre-builds a `Style`
per widget role and per interaction variant, applied through egui's own seams
(`UiBuilder::style`, `Ui::style_mut`, `Frame`), so per-widget geometry survives
the contested-field collisions. Coverage is enforced by an audited manifest that
a headless differential test checks against `ResolvedTheme`, rather than
asserted in prose. Targets egui 0.36.1.

Detailed design: [`docs/todo_v0.6.0_egui-connector-spec.md`](docs/todo_v0.6.0_egui-connector-spec.md)
and [`docs/todo_v0.6.0_egui-connector-rationale.md`](docs/todo_v0.6.0_egui-connector-rationale.md).

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
list and spinner. Padding, which iced takes through each widget's builder
rather than its `Style`, comes from `button_padding`, `input_padding` and, for
any other widget, `padding_or` and `stated_padding`, each keeping iced's own
default for a side the theme does not state. A `Style` field the model does
not carry is read from iced's default for that widget, and the ten theme
values iced 0.14 and `iced_aw` 0.14.1 have no receiver for are listed, with
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
  (`Checkbox::size`, `Toggler::size`, `ProgressBar::girth`, a rule's
  thickness), which an application reads from the `ResolvedTheme` itself
  today, and a helper that sets iced's `Settings` default font and text size
  from the theme's font;
- shadows: the model carries a shadow colour but no offset or blur, so every
  style keeps iced's own `shadow`.

Detailed design: [`docs/todo_iced-full-theme-geometry.md`](docs/todo_iced-full-theme-geometry.md).

## v0.6.2 — Upstream receivers for the gpui connector

v0.5.8 delivered the connector-side geometry: every widget where
gpui-component 0.6 applies the caller's `StyleRefinement` after its own
geometry now takes native heights, paddings, radii, borders and text sizes
through the `geometry` module, and the base layer (scrollbar, resize handle)
takes native values through `base_layer` with automatic re-application. What
remains is geometry on **inner elements the caller's style cannot reach**,
plus the tab properties `Tab`'s render overwrites in the shared style bag,
which is upstream work. v0.6.2 is the set of PRs to gpui-kit, one concern
each, framed as theming flexibility:

- a styled `Theme` scrollbar-style override honoured by `base_theme()`, so
  the connector's observer becomes unnecessary;
- `Tab` keeping the caller's height, radius and text size (its render writes its own into the style bag the caller's setters fill, `tab/tab.rs:801-808`);
- `Theme.shadow` honoured beyond `Button`, and `tokens.shadow` consumed;
- `Size::Size` honoured by `Checkbox` and `Switch`;
- inner geometry exposed: checkbox/radio indicator, switch track and thumb,
  slider track and thumb, separator thickness, resize-handle width, button
  icon gap, input padding, popup-menu items, select arrow, accordion arrow;
- a `PopupMenu` item style hook and a `Button::tooltip` style hook;
- button label text size independent of rem;
- an iterable `IconName::ALL` generated by `icon_named!`;
- public base-palette fields on `ThemeConfigColors` (`red` … `cyan_light`,
  `schema.rs:657-668`), so a config can carry the whole palette and the
  connector's palette repair becomes unnecessary.

Gap analysis with citations: [`docs/todo_gpui-full-theme.md`](docs/todo_gpui-full-theme.md);
limits table: `docs/archive/todo_v0.5.8_gpui-component-0.6-spec.md` §14.

## Beyond v0.6

No milestone targets committed yet. Likely candidates:
- slint, other connectors
- Expanded preset coverage (platform themes for more macOS / Windows versions)
- Additional icon-set bundles if demand emerges
