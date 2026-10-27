# Roadmap

Pre-1.0 milestones. Priorities can shift between minor versions — this roadmap
is a snapshot of current direction, not a commitment.

See also:

- [CHANGELOG.md](CHANGELOG.md) — what has already shipped
- [`docs/archive/`](docs/archive/) — detailed design docs and phase notes from completed milestones
- [`docs/todo.md`](docs/todo.md) — the backlog: every open item, tagged by milestone (`[v0.7.0]`, `[v0.7.x]`, `[1.0]`, `[post-1.0]`, `[upstream]`)

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

## v0.6.1 — gpui connector on GPUI Kit 0.7.1

**Delivered:** the gpui connector builds against gpui-component 0.7.1, which added
`IconName::Mic` and `IconName::Square`, and requires gpui-component, gpui-base and
gpui-kit 0.7.1 on gpui-pre 0.3.8. `geometry::button_label`, `geometry::toggle` and
`geometry::data_table_size` give a Button's label, a Toggle's text and a DataTable's
cells the platform's font size, which 0.7.1 shrank to 0.875 of it, and the showcase
shows `ColorSelect` and `SpeechWaveform`.

Detailed design: [`docs/archive/todo_v0.6.1_gpui-kit-0.7.1-spec.md`](docs/archive/todo_v0.6.1_gpui-kit-0.7.1-spec.md).

## v0.7.0 — Whole-codebase review

**Status:** released 2026-10-27 (tag `v0.7.0`).

The release after v0.6.1 is 0.7.0. The review's design phase accepted two
breaking changes ([`docs/archive/todo_v0.7.0_code-review-plan.md`](docs/archive/todo_v0.7.0_code-review-plan.md), D1):

- the `-live` preset tier is removed: the platform readers merge over the
  platform preset, and `Theme::preset("…-live")`, `PlatformPreset::is_live`
  and `PlatformPreset::live_name()` are gone;
- the connectors' `from_system` returns a named result struct instead of a
  tuple, and their accessors take the model's field names.

The rest of the release is the review's fixes: its design decisions, platform
fidelity per OS version, and the code review of every crate, recorded in
[`docs/archive/todo_v0.7.0_code-review-findings.md`](docs/archive/todo_v0.7.0_code-review-findings.md).
Many of them break the public API too: among others, the `material`,
`macos-sonoma` and `ios` presets are `material-3` (decision D6), `macos-monterey`
and `ios-13`, the `model` module is
private and the icon and detection items have one path each (`icons`,
`detect`), `Error::Io` names the file, more types are `#[non_exhaustive]`,
and the gpui and egui connectors run a platform reader only with their
`native` feature (or one of native-theme's `kde`, `portal`, `macos` and
`windows`), as the iced connector does: without one, Linux gives the
platform preset in the OS's light or dark mode and reduced-motion
preference, and macOS and Windows an error. The *Breaking Changes* of
[CHANGELOG.md](CHANGELOG.md)'s *0.7.0* section list every one.

New in it: the `macos-tahoe` preset, macOS 26's Liquid Glass look, and the
platform preset picked by the running OS version's generation
(`SystemTheme::version_match` reports the match); GPUI Kit 0.7.3's elements as
theme data (`tooltip.show_delay_ms`, `dialog.entrance`,
`list.header_height_px`); the selected tab's folder shape
(`tab.active_outward_corner_radius_px`, windows-11's); the switch's track and
thumb outlines; and theme-change integrations in the iced and gpui connectors
(`theme_changes()`, `on_theme_change`).

It also moves the gpui connector to GPUI Kit 0.8.1 on gpui-pre 0.3.10, with a
`gpui-fast` feature that builds it on GPUI Kit's other core, GPUI Fast 0.1.12.

## v0.7.1 — Full theme geometry in the iced connector

v0.5.9 ships most of it, and v0.7.0 adds `styles::button_toggle`,
`pane_grid`, `expander_frame`, `segment_separator`, `dialog`, `toolbar`,
`toolbar_rule` and `application`. `native_theme_iced::styles`
(feature `widgets`, on by default) has 33 style functions, closures built
from the resolved theme. Twenty-one replace iced's own: every `Style` field —
colours, border radius and width included — of the button (neutral, primary,
danger, success, warning and link classes), text input, text editor,
checkbox, radio, toggler, pick list, menu, slider, scrollable, progress bar,
rule, pane grid split lines, tooltip, a card container and the application's
base style (`styles::application`, the window's background).
`styles::checkbox_over` and `toggler_over` are the checkbox and toggler of a
control drawn on a backdrop rather than on the window (a disabled one is
faded as one widget over it), `styles::button_toggle` is a toggle button,
`styles::scrollbar` is the scrollbar's configuration, its widths and
embedding, `styles::segmented_control`, `segment`, `segment_separator`,
`expander` and `expander_frame` style the containers and buttons the showcase
builds those widgets from, and `styles::dialog`, `toolbar` and `toolbar_rule`
style the containers an application draws a dialog and a toolbar in, which
iced 0.14 has no widget for. `styles::aw` (feature `iced_aw`) gives `iced_aw`'s card, menu
bar, tab bar, sidebar, selection list and spinner the same. Padding, which
iced takes through each widget's builder rather than its `Style`, comes from
`button_padding`, `input_padding`, `text_area_padding`, `combo_box_padding`
and, for any other widget, `padding_or`, `padding_inside_border` and
`stated_padding`, each keeping iced's own default for a side the theme does
not state. A `Style` field the model does not carry is read from iced's
default for that widget, and the theme values iced 0.14 and
`iced_aw` 0.14.1 have no receiver for, or carry only in part (and one
emitted field, `styles::aw::card.close_color`), are listed, with the
upstream lines that show it, in the connector's mapping contract.

**What remains** of the plan:

- replacements for the iced classes it lists that still paint from the
  palette: `button::background`, `checkbox::{secondary, success, danger}`,
  `progress_bar::{secondary, success, warning, danger}`,
  `container::{rounded_box, dark, primary, secondary, success, warning,
  danger}`. `container::bordered_box` and `button::subtle` stay iced's on
  purpose: they paint `background.weakest`, a colour iced derives and no
  platform states. `styles::pane_grid` leaves its `hovered_region`, the
  highlight under a pane dragged over another, iced's too: the model states
  no drop target;
- helpers for the sizes iced takes through widget builders
  (`Checkbox::size`, `ProgressBar::girth`, a rule's thickness; a switch's
  and a radio's sizes already come through `switch` and `radio`), which an application reads from the `ResolvedTheme` itself
  today, and a helper that sets iced's `Settings` default font and text size
  from the theme's font;
- shadows: the model carries a shadow colour but no offset or blur, so every
  style keeps iced's own `shadow`.

Detailed design: [`docs/todo_iced-full-theme-geometry.md`](docs/todo_iced-full-theme-geometry.md).

## v0.7.2 — Upstream receivers for the gpui connector

v0.5.8 delivered the connector-side geometry: every widget where
gpui-component 0.6 and later applies the caller's `StyleRefinement` after its own
geometry now takes native heights, paddings, radii, borders and text sizes
through the `geometry` module, and the base layer (scrollbar, resize handle)
takes native values through `base_layer` with automatic re-application. What
remains is geometry on **inner elements the caller's style cannot reach**,
plus the tab properties `Tab`'s render overwrites in the shared style bag,
which is upstream work. v0.7.2 is the set of PRs to gpui-kit, one concern
each, framed as theming flexibility:

- a styled `Theme` scrollbar-style override honoured by `base_theme()`, so
  the connector's observer becomes unnecessary;
- `Tab` keeping the caller's height, radius and text size (its render writes its own into the style bag the caller's setters fill, `tab/tab.rs:958-966` at gpui-component 0.8.1);
- `Theme.shadow` honoured beyond `Button`, and `tokens.shadow` consumed; the
  dialog among them, whose shadow is a literal, two black layers at 10% alpha
  drawn whatever `Theme.shadow` says (`dialog_shadow`,
  `dialog/dialog.rs:343-360` at gpui-component 0.8.1);
- `Size::Size` honoured by `Checkbox` and `Switch`;
- inner geometry exposed: checkbox/radio indicator, switch track and thumb,
  slider track and thumb, separator thickness, resize-handle width, button
  icon gap, popup-menu items, select arrow, accordion arrow;
- a `PopupMenu` item style hook and a `Button::tooltip` style hook;
- a Button label that takes the caller's text size (`button_text_size` on the
  content element shadows it; since 0.7.1 the Medium label is `text_sm`);
- `ColorSelect` applying the caller's refinement to its framed field;
- `DataTable` as `Styled`, or with a text-size setter;
- an iterable `IconName::ALL` generated by `icon_named!`;
- public base-palette fields on `ThemeConfigColors` (`blue` … `yellow_light`,
  `schema.rs:643-677` at gpui-component 0.8.1), so a config can carry the whole palette and the
  connector's palette repair becomes unnecessary;
- `Diff`'s code size taken from `mono_font_size` directly: it is
  `rem × mono_font_size / 16` (`diff/mod.rs:472` at gpui-component 0.8.1), so it
  differs from the theme's monospace size whenever the UI font is not 16 px;
- a TextView rule colour separate from `border`: the horizontal rule paints
  the border colour the tables use (`text/node.rs:4418` at gpui-base 0.8.1);
- a scaling `DialogEntrance`: gpui-component 0.8.1 has `SlideDown`, `Fade`,
  `FadeSlide` and `None` (`dialog/entrance.rs:14-27`), so the connector draws
  `dialog.entrance = "scale_fade"` (Windows, GNOME, Material) as a plain
  `Fade`;
- GPUI's own `.tooltip()` honouring gpui-base's `TooltipDefaults`: it waits
  its own 500 ms (`DEFAULT_TOOLTIP_SHOW_DELAY`, `src/elements/div.rs:53` at
  gpui-pre 0.3.10) unless each element is given a delay;
- gpui-base's tooltip hide delay separate from a reshow window: its grace
  period both keeps a tooltip visible after the pointer leaves and shows
  the next trigger's tooltip at once (`tooltip.rs:226-275` at gpui-base
  0.8.1), where a platform's reshow window does only the second.

Gap analysis with citations: [`docs/todo_gpui-full-theme.md`](docs/todo_gpui-full-theme.md);
limits table: `docs/archive/todo_v0.5.8_gpui-component-0.6-spec.md` §14.

## Also in the 0.7.x releases

[`docs/todo.md`](docs/todo.md) tags 29 open items `[v0.7.x]`, follow-ups of
the v0.7.0 review that a 0.7 patch release takes beside the two milestones
above. Among them: the `ios-26` preset, iOS 26's Liquid Glass look, deferred
while 38 of the values the resolver requires have no iOS 26 source;
material-3's rewrite (the code-review plan's M2d), after which its display
name becomes "Material 3"; the theme watchers' open questions on Windows and
macOS; details of the Linux desktop detection and of the KDE reader; and
showcase follow-ups. The items tagged `[1.0]` are settled before 1.0.

## Beyond v0.7

No milestone targets committed yet. Likely candidates:
- slint, other connectors
- Expanded preset coverage (platform themes for more macOS / Windows versions)
- Additional icon-set bundles if demand emerges
