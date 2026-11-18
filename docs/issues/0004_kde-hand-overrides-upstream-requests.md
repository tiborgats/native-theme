# Issue #4, second comment: four requests to GPUI Kit — drafts

- From: [`0004_kde-hand-overrides-rationale.md`](0004_kde-hand-overrides-rationale.md)
  §4.7, for GPUI Kit's tracker (<https://github.com/longbridge/gpui-kit/issues>,
  the `repository` of gpui-component, gpui-base and gpui-kit 0.9.1).
- Status: **drafts, nothing in this file is posted.** Each request is sent only
  with the maintainer's approval.
- Paths are within each crate as published on crates.io.

**1. `DataTable`'s scroll bars ignore the theme's track width**

gpui-component 0.9.1 `src/table/state.rs:2743` and `:2762` size the box that
holds a table's vertical and horizontal scroll bar with `Scrollbar::width()`,
a constant 16 px (gpui-base 0.9.1 `src/scrollbar.rs:22`, `:950-952`). The bar
itself takes its width from the theme's `ScrollbarStyles` as it lays itself
out (`src/scrollbar.rs:1391-1396`), so in every other scroll host it is as wide
as the theme says. With a theme
whose track is wider than 16 px (KDE Breeze's scroll bar is 21 px), the bar
in a table is clipped to 16 px and its thumb sits off centre; from reading the
layout and paint code (`src/scrollbar.rs:1425-1448`, `:1554-1556`,
`:1600-1604`), not from a run, its hit area also reaches into the last column
by the difference. Could the table size that box with the themed track width?
gpui-base's reader of that width, `Scrollbar::themed_track_width`
(`:955-957`), is `pub(crate)` and serves only the editor's overview ruler
(`src/input/base/element.rs:320`), so gpui-base would need to make it, or an
equivalent, readable from gpui-component.

**2. A selected cell's overlay hides the cell's text**

In cell-selection mode (`cell_selectable`), gpui-component 0.9.1 paints
`table_active`, edged with `table_active_border`, as an absolutely positioned
child laid over a selected cell's content (`src/table/state.rs:2458-2472`,
`:2630-2638`), while a selected row gets the same token as its background,
behind its content, as long as `list.active_highlight` is on, its default
(`:2536-2541`; with it off, a selected row is `accent`). A theme that matches
a platform whose row selection is opaque (KDE Breeze, macOS) needs an opaque
`table_active`, and then a selected cell's text is hidden; a translucent one
keeps the cell readable but washes out every selected row. Could the cell's
overlay be painted behind the cell's content, or could a selected cell have a
token of its own?

**3. Let a theme keep an opaque selection colour**

`ThemeColor::apply_config` caps the alpha of `list_active` and `table_active`
at 0.2 and of `selection` at 0.3 whenever a `ThemeConfig` is applied
(gpui-component 0.9.1 `src/theme/schema.rs:1039-1070`), a colour the config
states included (`try_parse_background_clamped`, `src/theme/color.rs:777-791`),
on every `Theme::change`, `Theme::sync_system_appearance`, registry reload and
`init` (`src/theme/mod.rs:332-334`). A theme that states the opaque selection a
platform draws (KDE Breeze, macOS) cannot keep it across a mode switch;
native-theme-gpui now writes the colours back after every rebuild. Could the
cap apply only to a colour the config does not state (a fallback), or could a
config or the theme turn it off?

**4. Draw the focus band for keyboard focus only**

gpui-component 0.9.1 draws a focused element's 3 px band (`src/styled.rs:250-262`,
`:302-327`) however the focus came, a click included, and tints a focused
button's or check box's border the same way. GPUI can tell a keyboard focus
from a pointer one (`Window::last_input_was_keyboard`, gpui-pre 0.3.12
`src/window.rs:3641-3644`), and `DataTable` already uses it
(`src/table/data_table.rs:255-261`), as `Carousel` suppresses its ring after a
pointer press (`src/carousel/carousel.rs:132-138`). KDE Breeze draws its focus band only for
a focus that came from the keyboard (Tab, Backtab or a shortcut). Could the
band be drawn only for keyboard focus, a text input keep its tinted border on
any focus, and a button's and a check box's border tint be keyboard-only too?
Hiding the whole focus style on a click, as `DataTable` does, would be wrong
for a text input, whose focused border should show however it was focused.
The decision would be taken when an element gains the focus, not at each
paint: `last_input_was_keyboard` follows the last event
(`src/window.rs:5873-5881`), so a band drawn from it at paint time would vanish
at the next mouse move, as `DataTable`'s focus style does. Until then
native-theme-gpui sets that switch (`Theme::focus_ring`) per focus for themes
whose platform shows its ring for keyboard focus only, from a plugin of `Root`
at each render of a window (0.7.2): on for a focus that followed a key, off
otherwise. Two rings do not follow `Theme::focus_ring` at all: the `Slider`'s
thumb ring (0.10.0 `src/slider.rs:228-238`), drawn while it has the focus
however it got it, and the `Tab`'s (`src/tab/tab.rs:962-973`), which is already
keyboard-only but through `focus_visible`, the every-paint rule.
