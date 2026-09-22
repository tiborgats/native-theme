# native-theme: TODO

---

## Core API

### `SystemTheme` — expose layout metrics

- [x] Add `pub layout: LayoutTheme` to `SystemTheme` (done in v0.5.8). Approved 2026-08-10; see
      `docs/todo_v0.6.0_egui-connector-spec.md` §16 Q-2 and honesty-ledger item
      21. Today `from_preset` can supply `Spacing::item_spacing` and
      `Spacing::window_margin` but `from_system` cannot, because `SystemTheme`
      has no `layout` field — so the two spacing values a toolkit user reaches
      for first fall back to toolkit defaults on the system path.
      One additive field on a struct that already carries `preset` and
      `icon_theme`; no resolver work, since `Theme::layout` is a plain
      `LayoutTheme` shared across the light and dark variants and all four of
      its fields are `Option<f32>`, so an absent layout costs nothing.
      Benefits the egui, iced and gpui connectors equally.

### Font rendering preferences — read them from the platform

- [ ] Research and document the platform font-rendering preferences in
      `docs/platform-facts.md` **first**, cited to authoritative sources, then
      add them to the theme model. Candidates, to be verified rather than
      assumed: fontconfig `hinting` / `hintstyle` / `antialias` / `rgba`
      (`/etc/fonts/`, `~/.config/fontconfig/`), KDE's `XftHintStyle` /
      `XftAntialias` / `XftSubPixel` in `kdeglobals`, GNOME's
      `org.gnome.desktop.interface font-hinting` / `font-antialiasing`,
      Windows ClearType, macOS font smoothing.

      We already read `Xft.dpi` for scaling (`native-theme/src/kde/mod.rs:159`,
      `:174-175`, via `detect::xft_dpi()`), but nothing reads the *rendering*
      preferences. A Qt or GTK app obeys the user's hinting and antialiasing
      choice; an app built on our connectors silently ignores it. Reading OS
      appearance settings is exactly this crate's remit, so this is a real gap
      rather than a nice-to-have — it is the same shape as reading colours and
      the icon theme.

      Benefits every connector, not just egui: iced and gpui both rasterize
      their own glyphs too.

---

## Toolkit Connectors

### native-theme-egui connector

- [ ] Implement the connector per `docs/todo_v0.6.0_egui-connector-spec.md`
      (rationale: `docs/todo_v0.6.0_egui-connector-rationale.md`). Targets
      egui 0.36.1.
- [ ] Map the platform font-rendering preferences (see Core API above) onto
      `Visuals::text_options` (`egui/src/style.rs:1000`). Only **one** of its
      four fields has a genuine platform source:

      - `font_hinting: bool` — hardcoded `true` by `TextOptions::default()`
        (`epaint/src/text/mod.rs:61`) regardless of the user's setting. This is
        the mappable one: fontconfig `hintnone` → `false`, otherwise `true`.
      - `subpixel_binning` — **not** a platform preference. It renders each
        glyph at up to four fractional horizontal offsets for more even kerning
        (`epaint/src/text/mod.rs:44-53`); it is *sub-pixel positioning*, not
        LCD subpixel rendering. Do not map fontconfig `rgba` onto it. Leave at
        egui's default.
      - `color_transfer_function` — **already correct, do not write it.**
        `Visuals::dark()` and `Visuals::light()` set the right per-mode curve
        (`style.rs:1500`, `:1567`) and the connector inherits it by starting
        each scheme from its own `Theme::default_style()` (spec §3.4).
        Writing it from theme data would fabricate a value.
      - `max_texture_side` — overruled by `RawInput::max_texture_side`
        (`style.rs:997-999`). Never write it.

      Two platform preferences have **no egui expression** and should be
      recorded in the spec's §14 honesty ledger rather than faked: LCD subpixel
      order (`rgba`) — epaint computes one coverage value per pixel, so it is
      grayscale-antialiased only; and `antialias=false` — `HintingTarget`'s own
      docs state egui always renders anti-aliased
      (`epaint/src/text/fonts.rs:306-308`).

      Also check whether `HintingTarget` (`epaint/src/text/fonts.rs:303-314`)
      is reachable globally or only per-font via `FontTweak`; `TextOptions`
      exposes only the `bool`.

- [ ] Add an MSRV CI job (spec §12.4, task 22). The workspace floor of `1.88.0`
      was measured on 2026-08-10, but nothing re-checks it: every CI job
      installs `@stable`, there is no `rust-toolchain.toml`, and
      `pre-release-check.sh` has no MSRV check. The job must cover the
      workspace at `1.88.0` (re-measured 2026-09-06, unchanged; since
      2026-09-07 `native-theme` itself uses `slice::as_chunks`, stable since
      1.88.0, so the floor cannot drop below that), the gpui connector
      separately at `1.95.0` (re-measured 2026-09-19 on the 0.6.4 closure:
      1.95.0 builds, 1.94.0 fails on `std::hint::cold_path` in gpui-pre 0.3.5,
      `src/profiler.rs:473, 494`) and the egui connector at `1.95`.

- [ ] Cross-target warning hygiene. `cargo check -p native-theme --features
      windows --target x86_64-pc-windows-msvc` reports 5 warnings and
      `--features macos --target x86_64-apple-darwin` 3 (measured 2026-09-07
      on rustc 1.98.1: `resolve/inheritance.rs:74` unreachable tail after the
      Windows `return`, `icons.rs:466` unused `theme` off Linux,
      `pipeline.rs:576` `preset_as_reader` used only on Linux,
      `windows.rs:179` `read_frame_width` and `:373` `dwm_color_to_rgba`
      never called, `macos.rs:60` unused `separator_c`). On a Windows target
      without the `windows` feature the whole `windows` module is dead code
      (`lib.rs:165` gates it on `target_os` only; the `not(windows)` twin
      carries `#[allow(dead_code)]`), which is what docs.rs and a Windows
      `native-theme-iced` build compile. None of the sites changed in
      v0.5.8; CI's test jobs do not deny warnings, so nothing fails. The
      MSRV CI job above is the natural place for a cross-target
      `cargo check -D warnings`.

### native-theme-gpui connector

- [x] Map `WidgetMetrics` → gpui-component per-widget styling — done in
      v0.5.8 for every widget with a reachable seam (`geometry` module,
      `base_layer`); the inner-element remainder is the upstream PR list
      below (v0.5.8 spec §14).
- [ ] Key the icon tables by `gpui_kit_assets::IconName` (gpui-kit 0.6.1:
      `ALL`, `PartialEq`, `Debug`, `Hash`) so the hand audit and the
      101-count tripwire in `icons.rs` become a compile-time check over
      `ALL`; the floor is gpui-component 0.6.4 since v0.5.9, so nothing blocks
      this. Verified 2026-09-09 that the released 0.5.8 connector builds and
      passes its tests unchanged on gpui-kit 0.6.1 with gpui-pre 0.3.3 and
      0.3.4.

#### Upstream PR candidates from v0.5.8 (Tier U)

- [ ] a `ghost_hover` / `ghost_hover_foreground` token pair separate from
      `accent` / `accent_foreground` (0.6.4 hovers ghost buttons with the
      accent pair, which native themes map to the platform's selection
      colours, so a hovered ghost button becomes a selection-coloured pill;
      `button/button.rs:1125-1132, 1141`)
- [ ] a menu-surface token. `PopupMenu` renders with `.popover_style(cx)`, i.e.
      `bg(theme.popover)` (`menu/popup_menu.rs:1476`, `styled.rs:193-199`), and
      there is no seam to override it. native-theme records
      `menu.background_color` separately from `popover.background_color`, and
      the two differ on 30 of the 32 preset/mode combinations (kde-breeze light:
      `#eff0f1` against `#ffffff`), so every menu is painted on the popover's
      surface. Found 2026-09-21 by running the contrast rule.
- [ ] a foreground token for the status bar. `StatusBar` labels everything
      with `muted_foreground` (`status_bar.rs:95`) over `tokens.status_bar`;
      measured against the platform's own `status_bar.font.color` on its
      `status_bar.background_color` that is worse on 32 of the 32 preset/mode
      combinations (nord dark: 1.69 against 9.25). Since v0.5.9
      `geometry::status_bar` carries the platform's colour — upstream applies
      the caller's refinement after its own text colour, so it wins — but an
      application that does not use the builder gets the muted label. The
      gpui contract reports the token pair on every run.
- [ ] foreground tokens for a selected list row and for selected text:
      `ThemeColor` has `list_active` and `selection` fills and no label colour
      for either, so a selected row keeps `foreground` where the platform pairs
      `list.selection_background` with `list.selection_text_color`. Worse than
      the platform on 21 of 32 each; reported by the gpui contract, not
      asserted. The title bar is the same shape (no `title_bar_foreground`),
      worse on 12 of 32 against the darker end of upstream's title-bar
      gradient (`title_bar.rs:21-35`).
- [ ] `Checkbox::label` sets `text_color(foreground)` on the label wrapper
      unconditionally (`checkbox.rs:334-341`), so a platform
      `checkbox.font.color` has no route; and both `Checkbox` and `Combobox`
      apply the caller's refinement *after* their disabled colour
      (`checkbox.rs:251-257`, `combobox.rs:990-997` with
      `input/input.rs:99-103`), so a carried colour would displace it. For
      that reason `geometry::checkbox` and `geometry::combobox` carry no text
      colour; a guard test fails, saying so, the day a preset states one.
- [ ] `WindowBorder`'s frame colour is a literal (`window_border.rs:152-166`)
      and so is the shadow it draws below it in the same `render`; no theme
      value reaches either.
- [ ] a determinate `ProgressCircle` has no size receiver the theme can feed;
      only the indeterminate one takes `geometry::spinner_size`.
- [ ] `tokens.tab`, `tokens.list`, `sidebar_primary` and
      `sidebar_primary_foreground` are read by nothing in 0.6.4 outside
      `theme/` (an idle tab is `transparent`, `tab/tab.rs:132-160`). The
      connector maps them anyway, and the contract marks pairs over them as
      inert so they are not counted as coverage.
- [ ] the segmented `TabBar` has receivers for the track only
      (`tab/tab_bar.rs:391`): the selected segment is filled with
      `tokens.background` and labelled `tab_active_foreground`
      (`tab/tab.rs:247-249`), so `segmented_control.active_background`,
      `.active_text_color`, `.font.color` and `.border.color` reach nothing.
- [ ] `Button::icon` discards an `Icon`'s own size
      (`button/button_icon.rs:116-131`); a nested `TitleBar`'s window controls
      are hit-tested by the OS on Windows (`title_bar.rs:220-222`) and
      `on_close_window` is Linux-only (`:99-101`), so a title bar shown inside
      a window closes that window there.
- [ ] `Checkbox` and `Radio` draw their unchecked border with `theme.input`
      (`checkbox.rs:238`, `radio.rs:188`), the *text input's* border colour.
      native-theme records `checkbox.unchecked_border_color` separately and the
      two differ on 12 of the 32 preset/mode combinations — windows-11 light
      wants `#0000005c` where the input border is `#e5e5e5ff`. The token is
      named `input`, so the connector feeds it the input border; upstream needs
      a checkbox border token.
- [ ] Whether any platform scales a control's corner radius with its control
      size. platform-facts records one `border.corner_radius` per widget on all
      four platforms (§2.3 and the per-widget tables), and records size classes
      only for *dimensions* — NSTextField 22/19/17, NSSwitch 38x22/32x18/26x15,
      NSPopUpButton 21/18/15/24. On macOS the bezel corners are baked into
      CoreUI `.car` artwork with no queryable constant (`:1311`), so a
      size-dependent radius there is unmodelled rather than known to be absent.
      If one exists, `ResolvedBorderSpec` would need a per-size radius and
      `geometry::button` / `input_group_button` would follow it; today both
      restore the widget's single radius, which is what the model carries.
- [ ] a control's radius should not shrink with its size: an `XSmall`
      `InputGroupButton` takes `radius / 2` (`input/group.rs:590-593`) and
      `Button` does the same for its small and large roundings
      (`button/button.rs:593-595`), where a platform records one radius per
      widget whatever its size. `geometry::button` and
      `geometry::input_group_button` restore it per call site.
- [ ] `Toggle` needs a token of its own for the pressed state: it reads
      `tokens.accent` (`button/toggle.rs:155, 202`), the item highlight of
      menus and lists. Since v0.5.9 the connector feeds that token the
      platform's menu hover pair, which on Adwaita, Windows 11 and Material is
      a subtle fill, while those platforms' `segmented_control.active_background`
      is the accent colour. KDE and macOS are unaffected (both are the
      selection colour).
- [ ] `TabVariant::Outline` hovers with `tokens.secondary_hover`, the *button*
      hover (`tab/tab.rs:187`); every preset has a different
      `tab.hover_background` (Breeze: `#dee0e2` against the button's
      `#93cee9`). The default `Tab` variant has no hover fill and is unaffected.
- [ ] `InputGroupButton` should hover with the platform's button hover, not
      `muted`: `render_in_group` hardcodes `cx.theme().muted` (halved in dark)
      for an in-group addon button (`input/group.rs:544-583`), which is the
      muted-background slot, not a hover colour. Measured on kde-breeze light
      it gives `#dbdcdd`, 20 units from the `#eff0f1` field, while KDE's
      `[Colors:Button] DecorationHover` (our `secondary_hover`) is `#93cee9` at
      57 — so the addon button is grey while every other button in the window
      is blue. `secondary_hover` / `button_secondary_hover` is the token it
      wants.
- [ ] a `Dialog::max_h` prop, or letting the caller's `max_h` win: 0.6.4
      clamps the dialog to the viewport after `refine_style`
      (`dialog/dialog.rs:631`), so a themed `dialog.max_height` cannot reach
      it, while the width already has `Dialog::max_w`
- [ ] a styled `Theme` scrollbar-style override honoured by `base_theme()`
      (would make the connector's re-apply observer unnecessary)
- [ ] `Tab` keeping the caller's height, radius and text size (its render writes its own into the style bag the caller's setters fill, `tab/tab.rs:801-808`)
- [ ] `Theme.shadow` honoured beyond `Button`; `tokens.shadow` consumed
- [ ] `Size::Size` honoured by `Checkbox` and `Switch`
- [ ] inner geometry exposed: checkbox/radio indicator, switch track and thumb,
      slider track and thumb, separator thickness, resize-handle width, button
      icon gap, input padding, popup-menu items, select arrow, accordion arrow
- [ ] a `PopupMenu` item style hook; a `Button::tooltip` style hook
- [ ] button label text size independent of rem
- [x] an iterable `IconName::ALL` generated by `icon_named!` — landed
      upstream in gpui-kit 0.6.1 without our PR: `gpui_kit_assets::IconName`
      covers the full Lucide catalog (1830 variants) with `ALL`, `PartialEq`,
      `Debug` and `Hash`; gpui-component's `IconName` stays a 101-variant
      compatibility enum (same set as 0.6.0) that converts into it via
      `From`. The connector-side follow-up is listed above.
- [ ] public base-palette fields on `ThemeConfigColors` (`red` … `cyan_light`, `schema.rs:657-668`)

#### native-theme-iced: the same audit (found 2026-09-20, fixed in v0.5.9)

The gpui connector's state tokens were audited against the native field of the
widget that reads them, and `accent` was wrong. The iced connector's output was
then read the same way — every reader of every slot `to_theme` writes, found by
grepping the `iced_widget-0.14.2` catalogs — and the same defect class was
there. The last two items below are the ones the repository's own
`connector-parity-checker` reports, because they are public-surface and
feature-table differences rather than slot semantics. All of it is scheduled
into v0.5.9 by the standing rule that a
pre-1.0 release fixes the bugs found during it; the design is in
`docs/archive/todo_v0.5.9_theme-contracts-{rationale,spec}.md` and the citations stay
here so the work is not re-derived.

All nine are closed in v0.5.9. iced's palette has six colours, so most of them
could not be fixed *in* the palette: the fix is `native_theme_iced::styles`, one
function per widget, and the palette default stays what iced derives. An
application that wants the platform's value for these roles passes the style
function; the showcase does so for every widget it renders, and a source-level
test (`styles_cover_every_widget_shown`) keeps it that way.

- [x] (`styles::menu`: `selected_background` ← `menu.hover_background`.) Menu/pick-list highlight takes the platform accent: `Palette.primary`
      (`palette.rs:41`) feeds `overlay/menu.rs:658` `selected_background`,
      where the platform's field is `menu.hover_background`. Exactly the gpui
      defect. `primary.strong` is also read by focused-input border, radio dot,
      pick-list hover border, checkbox hover fill and scroll-thumb hover, so
      the fix is an explicit `menu::Style`, not an override of the token.
- [x] (Palette: `secondary.base` ← `input.placeholder_color`, labelled by the window text, `secondary.strong` a copy of it; `styles::text_input`, `text_editor` and `pick_list` state the placeholder exactly.) `button.background_color` is written into `secondary.base.color`
      (`extended.rs:109`), which iced reads as *placeholder text*
      (`text_input.rs:1769`, `text_editor.rs:1476`, `pick_list.rs:910`). On
      adwaita light that is `#e8e8e8` on a `#fafafb` field — about 1.15:1, so
      the placeholder is invisible. `input.placeholder_color` exists and is
      never read.
- [x] (`styles::scrollable`, per axis.) Hovered/dragged scrollbar thumb takes the accent
      (`scrollable.rs:2385, 2414`); `scrollbar.thumb_hover_color` and
      `thumb_active_color` exist and are never read.
- [x] (Each role has its own function: `scrollable`, `toggler`, `menu`, `pick_list`, `text_input`, `container_card`, `button`, `rule`, `checkbox`.) `defaults.surface_color` drives nine unrelated roles
      (`extended.rs:111`): scrollbar rail, unchecked switch track, menu panel,
      closed pick-list, disabled input, rounded box, button hover, rule,
      several checkbox states. Each has its own native field. An "off" switch
      is currently indistinguishable from the page on adwaita.
- [x] (`styles::text_input` / `text_editor`: `selection` ← `input.selection_background`.) Text selection takes a 40% accent tint (`text_input.rs:1771`) rather
      than `input.selection_background`; the connector's own
      `selection_color()` helper is never used in `to_theme`.
- [x] (Every style function emits its widget's own `border.*`; `styles::rule` takes `separator.line_color`.) Borders, dividers, rails and tracks come from a lightness deviation of
      the window background, not from `defaults.border.color` /
      `input.border.color` / `checkbox.unchecked_border_color`; the
      connector's `border_color()` helper is never written into the theme.
- [x] (`secondary.strong` is now a copy of `secondary.base`; `styles::button` states hover and pressed from `button.hover_background` / `active_background`, composited over the idle fill.) `apply_overrides` writes only `.base` entries, so `.weak` / `.strong`
      keep values generated from the unoverridden palette: a button painted
      with the platform's surface jumps to an unrelated tone on hover, and
      `button.hover_background` is never read.
- [x] (`font_size` and `mono_font_size` take the preferences and `from_system` returns them; `to_theme` does not, because a palette has nothing for them to act on — reduced transparency and reduced motion have no receiver in iced.) `to_theme` takes no `AccessibilityPreferences` (`iced/src/lib.rs:113`),
      so text scaling, reduced transparency and reduced motion never reach an
      iced application at all.
- [x] (`widgets`, `iced_aw` and the four icon features; the icon features are in `default`.) The iced connector declares no `[features]`, so a consumer depending on
      it alone gets `native_theme::icons::load_icon` returning `None` for every
      icon; the gpui connector forwards the four icon features.

What is still open on the iced side:

- [ ] The iced `geometry` gap stays: the connector has no counterpart of the
      gpui connector's `geometry` module, because iced takes geometry through
      builder methods on each widget. Native values with a builder receiver
      (`Checkbox::size` / `Radio::size` ← `checkbox.indicator_width`,
      `::spacing` ← `label_gap`, `Toggler::size` ← `switch.track_height`,
      `ProgressBar::girth` ← `progress_bar.track_height`, the rule's thickness
      ← `separator.line_width`, `pick_list::Handle::Arrow { size }` ←
      `combo_box.arrow_icon_size`) are named in the style functions' doc
      comments and applied by the showcase; spacing comes from the model's
      `LayoutTheme`, which is public on `Theme` / `SystemTheme` and needs no
      connector API. `scripts/check-widget-coverage.py` keeps *widget*
      coverage visible; nothing mechanical yet lists native geometry fields
      that no iced builder receives.
- [ ] Unreachable in iced 0.14 / iced_aw 0.14.1, recorded so they are not
      re-derived: `scrollbar.min_thumb_length` (iced computes the scroller
      length itself, `scrollable.rs:1998, 2068`; the one entry of the
      contract's `UNREACHABLE` list); `Table` has a `Catalog` and a `Style`
      but no `.style()` / `.class()` setter (`table.rs:149-196`), so its
      separators stay `palette.background.strong`; `splitter.divider_color`
      (a `pane_grid` split line is drawn only while picked,
      `pane_grid.rs:950-955`); `progress_bar.min_width`, `tooltip.max_width`
      and `slider.tick_mark_length` (no receiver at all); a platform font
      *family* (`Family::Name(&'static str)` against the model's `Arc<str>`);
      `tab.min_width` / `min_height` (iced_aw takes fixed lengths, no minimum
      form); `sidebar.border.corner_radius` and `list.border.corner_radius`
      (both widgets hardcode radius 0); `menu.hover_text_color` and
      `menu.font.color` on `MenuBar` (`menu_bar::Style` has no text colour);
      `list.row_height` is reachable only indirectly, as the vertical padding
      of `SelectionList::new_with`.
- [ ] iced_aw 0.14.1 defects found by the showcase self-tests, worth filing:
      `ContextMenu::operate` hands the open popup the underlay's layout
      (`context_menu.rs:176-200`), which panics any `iced_test` selector
      while a menu is open; `SelectionList::operate` reports its rows in
      list-local coordinates (`selection_list/list.rs:280-310`);
      `Tabs::operate` never visits its own tab bar (`tabs.rs:601-621`).
      `Spinner` has no `Style`, `Catalog` or setter — `styles::aw::spinner`
      styles a wrapping container instead.
      `TabBar` and `Sidebar` test the pointer before the selection
      (`tab_bar.rs:589-595`, `sidebar/sidebar.rs:980-986`) and `Status::Hovered`
      carries no selected flag, so a hovered *selected* tab loses its selected
      look and no style function can prevent it (`SelectionList` tests the
      selection first and is fine).

#### Research

- [ ] 174 of 512 text-on-background pairs sit below WCAG AA across the 16
      presets in both modes (measured 2026-09-20, alpha composited). Most are
      the platform's own choice — macOS ships `success_color = "#34c759"` with
      white text at about 2.2:1 — and the connectors must not override a
      platform. But some look like preset data errors rather than platform
      facts, and `preset-validator` should judge them: `material` light and
      dark give `tab.active_text_color` and `tab.active_background` values
      that leave the label on its own fill, and `nord` dark puts
      `input.placeholder_color` at 1.36 against the field. The contrast test
      added in v0.5.9 prints the whole list on every run, so it stays visible.

- [ ] kde-breeze states `button.hover_background = "#93cee9"` in both modes
      under a `#fcfcfc` label in dark mode: 1.67:1 (the pressed pair is 2.43
      in light mode). platform-facts records the KDE source as "`[Colors:Button]
      DecorationHover` **blend**" (`platform-facts.md:1168`), and `#93cee9` is
      the raw `DecorationHover`, which Breeze paints as an outline — so the
      preset may be recording the unblended colour as a fill. Found 2026-09-21
      by the iced contrast report; for `preset-validator` to judge. The same
      report shows `solarized` (both modes) and `tokyo-night` light with *idle*
      button labels at 3.6–4.1:1.
- [ ] `button.hover_text_color` and `button.active_text_color` equal
      `button.font.color` in all 16 presets, so the per-status label rows of
      both contracts cannot tell hover or pressed from idle today. Either the
      platforms really keep the label, or the presets never recorded it.
- [ ] The progress bar's fill on its track is below AA on 32 of 32 (kde-breeze
      dark 1.05, adwaita light 1.21). It is the platform's own accent-on-muted
      pair, emitted exactly — an indicator, not text — but it is the first
      thing that looks like a bug when the showcase is opened.
- [ ] No bundled preset states `segmented_control.background_color`, so it
      inherits the window background — the colour gpui-component fills the
      *selected* segment with. Since v0.5.9 maps the segmented track from that
      field, track and selected segment are one colour until the presets state
      the platform's track.
- [ ] kde-breeze states `switch.thumb_diameter == switch.track_height` (18/18),
      so `styles::toggler` emits a thumb inset of zero there. Breeze does draw
      a handle as tall as its groove; confirm against Breeze's metrics.
- [ ] Since v0.5.9 the iced connector writes `background.base.text` from
      `defaults.text_color`, where iced's `readable()` used to substitute a
      higher-contrast colour below 6.0:1. That now shows the platform's own
      4.13 (solarized light), 4.75 (solarized dark) and 4.52 (tokyo-night
      light) window text. If those are preset errors rather than the themes'
      own values, the presets are where to correct them.
- [ ] How a platform draws a *disabled checked* checkbox is not modelled.
      `platform-facts.md` §2.5 records no disabled indicator colour; the one
      disabled mechanism it records for a checkbox is `disabled_opacity`
      (dim the accent fill and the on-accent mark together). The model instead
      gives one `checkbox.disabled_background` for checked and unchecked alike,
      and `checkbox.disabled_opacity` has **no receiver** in native-theme-iced
      (iced's `checkbox::Style` has no opacity; it would have to be multiplied
      into the emitted alphas). Since v0.5.9 `styles::checkbox` paints the
      disabled mark with `checkbox.disabled_text_color` on that fill — the
      platform's own disabled pair, 1.17–2.71:1 over the 32 combinations, where
      the on-accent mark on the neutral fill was worse than the platform's own
      in 26. Decide whether the model should state a disabled *checked* fill
      (or the connector apply `disabled_opacity`), then revisit that arm; it is
      one match arm plus `native_checkbox_mark` in the contract.
- [ ] `windows-11` states `checkbox.disabled_background = "#f9f9f900"` (alpha
      zero) and `material` `#1c1b1f1f`, so on those four combinations a disabled
      box is effectively absent — the same shape as the button item below.
- [ ] `checkbox.indicator_color` inherited `defaults.text_color` until v0.5.9,
      against `platform-facts.md` §2.5 (on-accent on all four platforms);
      corrected to `defaults.accent_text_color`. The test that ties
      `docs/inheritance-rules.toml` to the proc-macro attributes
      (`uniform_rules_all_accounted_for`) checks that each key is *present* on
      both sides and never compares the inheritance **target**, so the
      published rule file can drift from the code with no test noticing. Add a
      target-level assertion, then audit the other rules against
      platform-facts the way this one was found.
- [ ] windows-11 light gives `button.disabled_background = "#f9f9f900"` — alpha
      zero, so a disabled button has no fill at all. Found 2026-09-21 while
      measuring translucent state colours; for `preset-validator` to judge
      against Fluent's `ControlFillColorDisabled`.

- [ ] Scrollbar thumb radius per platform: platform-facts records none, so the
      connector mirrors gpui-component's `radius` for the thumb.
- [ ] The showcase's "Text Input" tooltip claims the input background is
      `background`; `Input` paints `Theme::input_background()`, which equals
      `background` only in light mode (gpui-component 0.6.4
      `src/theme/mod.rs:379-385`). Noticed while adding the InputGroup section
      in v0.5.9; correct the tooltip (and check the sibling widgets' claims).
- [ ] gpui-component's `Theme::motion` (`MotionTokens`, eleven fields: four
      durations, three easings, two springs and two `Rems` travel distances,
      `src/theme/motion.rs:8-20`) stays at upstream's default, because
      `ResolvedTheme` has no motion field at all; KDE exposes
      `AnimationDurationFactor` and GNOME `enable-animations`, which
      native-theme reads only as the boolean `reduce_motion`
      (`native-theme/src/kde/mod.rs:42-52`, `src/detect.rs:674-685`). Decide
      whether native-theme should model motion — durations, easings,
      distances — which is a core-type change.
- [ ] Preset font families that are not installed: GPUI resolves a named,
      missing family through its fallback stack on every text run (upstream's
      statement, gpui-component 0.6.4 `src/theme/system_font.rs:3-9`; not
      measured here). Decide whether `to_theme` should fall back to
      `.SystemUIFont` when `cx.text_system().all_font_names()` lacks the
      family, as upstream does for its own defaults.
- [ ] Widen the captured screenshot tabs: every capture passes `--tab buttons`
      (`scripts/generate_gpui_screenshots.sh:70`, `screenshots.yml`), so the
      InputGroup, Empty, Carousel, code-editor and Markdown sections never
      appear in an artefact.
- [ ] Re-verify the upstream `file:line` citations in this file,
      `docs/todo_gpui-full-theme.md` and `ROADMAP.md`, which are still
      0.6.0-era; four are known stale at 0.6.4 (`popup_menu.rs:749` — fixed in
      the gap analysis, `button.rs:360`, `switch.rs:136-146`,
      `checkbox.rs:195-199`).

#### Follow-ups from the v0.5.9 showcase and contract work

- [ ] **Open question for the maintainer, carried out of the archived Widget
      Info rationale (§6, question 2):** the panel displays the citation
      beside the swatch — `text: link #2a7ab0 (button.rs:993)`. It is what
      makes a claim checkable by a reader and not only by a test, but it is
      also four widgets' worth of line numbers in a hover panel. Keep it, or
      keep the citation in the source and show only `text: link #2a7ab0`?
      The other three questions in that section answered themselves: the
      work landed on the v0.5.9 branch, the six Button variants' field names
      were corrected by the citation pass, and `Command` has a real demo.
- [ ] **Finish auditing the Widget Info "Not themeable" entries for
      *resolvability*.** 257 entries over 107 panels; 99 cite an upstream
      symbol, 158 do not, and an uncited one is an assertion nobody checked.
      The audit is not "is this sentence accurate" but "could the connector
      do something about it" -- those are the same question, because "not
      themeable" is itself a claim about resolvability.

      Method, one entry at a time: (1) does native-theme model a field for
      this property; (2) does a `geometry::` builder carry it; (3) does
      upstream apply the caller's refinement to the element that would
      receive it, or to an ancestor of it. A "no" at (3) with a "yes" at (1)
      is Tier U, not an absence, and the note should say which.

      Scoped by where the answer can differ: **70 of the 158 are under
      widgets native-theme models no counterpart for at all** (Alert, Tag,
      Badge, Kbd, the charts, Avatar, Breadcrumb, Skeleton, the text
      samples) -- those claims are true, but because of *our* model rather
      than upstream, so the note should say that and the fix is a model
      addition. The other **88 sit under one of the 27 modelled widgets**
      and are where a resolvable gap can hide.

      Three done, and the hit rate justifies the rest: a Button's
      `font-weight` was called hardcoded on ten panels and was simply not
      carried -- fixed, `geometry::button` now applies it. A Separator's
      thickness and a Slider's track height and thumb size are genuinely
      unreachable, but the model carries all three, so they are Tier U
      (already in "inner geometry exposed" above) and the notes now say so
      rather than "hardcoded".

      The largest untouched groups are `padding` (13), `animation` (7),
      `label size` (6) and `icon size` (5).
- [ ] `ThemeColor::tab` and `ThemeColor::list_even` are slots nothing paints.
      The connector writes both on every `apply` and both have contract rows
      (`contract.rs:420`, `:354`), but no `theme().tab` is read anywhere in
      gpui-component or gpui-base (declared `theme_color.rs:261`, defaulted
      `schema.rs:996`), and `list_even` is read only as `table_even`'s
      fallback (`schema.rs:1005`) — which never fires, because the connector
      writes `table_even` too (`contract.rs:391`). Every gate is green on
      both: they are written, so no coverage check notices, and they are read
      nowhere, so no widget can disagree with them. Either upstream should be
      asked to read them or the contract should record them as write-only.
- [ ] **Reference — how a theme colour reaches a gpui-component widget.**
      Eight routes, seven of which defeat a search for the field's own name,
      and the thing the next audit of this kind will need first:
      `cx.theme().field`; `self.tokens.field` inside the `Theme` impl
      (`theme/mod.rs`, the scrollbars); `cx.theme().tokens.<group>` — a
      *token group*, not a colour, so `tokens.button_hover.into()` and
      `tokens.primary_hover.background` both carry `button_hover` and
      `primary_hover` past a search for a flat field;
      `cx.theme().semantic_tokens().colors.field` (`bubble.rs`); a rename in
      the semantic layer (`danger` → `destructive`, `theme/mod.rs:428`); a
      value written across to gpui-base as data (the scrollbar and resizable
      settings); a mode-switching accessor (`input_background()`, which is
      what `input_style` returns and why a "trigger bg" claim cites
      `theme/mod.rs:383` and not `input/input.rs:105`); and plain
      inheritance, where the widget sets nothing and takes `Root`'s.
      Two corollaries, both paid for: several widgets read **nothing** and
      delegate entirely (`clipboard.rs`, `hover_card.rs`,
      `menu/context_menu.rs`, `popover.rs`, `text/text_view.rs`,
      `menu/app_menu_bar.rs`, `dialog/alert_dialog.rs`, `pagination.rs`), so
      the file that names the widget is not the file that paints it; and a
      citation proposed from a crate-wide search was plausible-but-wrong five
      times out of five — it is a lead to read, never an answer to accept.
- [ ] Port `scripts/check-widget-coverage.py` to a `#[test]`, as the Widget
      Info citation check was. A gate that has to be invoked can be skipped;
      one that runs under `cargo test` cannot, and the objection that a test
      cannot reach `cargo metadata` turned out to be false — a test runs
      after the build and can call cargo itself (1007 packages resolved in
      under a second, `src/showcase.rs`). Not a straight copy of that port:
      this one reads **three** toolkits and needs the iced manifest with
      `--features iced_aw`, so it does not belong in the gpui connector's
      tests — it wants splitting per connector or a shared home. It also
      parses TOML, so it needs a `toml` dev-dependency, which the citation
      check did not. It works today, so this is tidying, not a fix.
      `scripts/generate_gifs.py` is deliberately **not** included: it is
      release tooling a human runs, not a gate, and nothing can silently
      skip it because its output is the screenshots the asset stamp checks.
- [ ] Split the showcases into modules: `showcase-gpui.rs` and
      `showcase-iced.rs` are several thousand lines each after gaining every
      widget and their self-tests.
- [ ] `scripts/check-widget-coverage.py` still accepts weak evidence of
      "shown" on the **iced** side: an import of the module is enough. The
      gpui side no longer does — `shows_gpui` requires a constructor, a call,
      a struct literal or a named extension method, and rejects a name that
      is a segment of somebody else's path. The same tightening for iced
      wants per-module constructor patterns, since an iced widget is a
      function (`text`, `button`) rather than a type. The script also cannot
      see `cfg`, so the `iced_aw` widgets count as shown in a build that
      omits them.
- [ ] `text_scale_factor` (finite and positive, else 1) is written twice, once
      per connector; a method on `AccessibilityPreferences` in the core crate
      would state it once.
- [ ] The repository's PreToolUse hook refuses `panic!` in files under
      `examples/`, although the project's rule exempts examples and tests by
      path; align the hook with the rule.
- [ ] Run each showcase once with `XDG_CONFIG_HOME` pointing at an empty
      directory before a release. Both defects the gpui self-tests found (the
      overlay layers never mounted; the mode selector dead when no theme can
      be read) sat on paths a configured desktop never takes.

#### Upstream PR to gpui

- [ ] PR: add `Window::screenshot()` API to gpui — gpui has no public way to
      capture the rendered framebuffer. The underlying blade-graphics backend
      has `copy_texture_to_buffer()` but gpui doesn't expose it. A public
      `screenshot()` method would enable headless CI screenshot capture on all
      platforms (like iced's `--screenshot` flag). Without this, gpui showcase
      screenshots are Linux-only (via external spectacle capture).

#### Upstream PRs to gpui-component

Where the connector needs customization hooks that gpui-component doesn't
expose, submit PRs to gpui-component upstream. Guidelines for acceptance:

- **Frame as "more theming flexibility"** — not "native platform look."
  The maintainers follow shadcn/ui + Apple HIG + Fluent design philosophy;
  they'll accept exposing knobs, not changing defaults.
- **No API breaking changes.** Add new builder methods, new optional theme
  tokens, or new style parameters — never change existing signatures or
  defaults.
- **One concern per PR.** Each PR should expose one category of
  customization (e.g., "allow custom checkbox indicator size via theme
  token" or "expose button padding as configurable").
- **Provide concrete benefit.** Show how the change enables theming use
  cases (screenshots of before/after with different themes help).
- **Follow their CONTRIBUTING.md.** AI-generated code must be disclosed
  and human-reviewed. Default cursor for buttons (not pointer). Medium
  sizes as default.

Checklist of likely needed PRs (discover exact gaps during connector work):

- [ ] Audit gpui-component widgets for hardcoded values that should be
      theme tokens (padding, icon sizes, corner radii, spacing)
- [ ] PR: expose per-widget padding/margin as theme-configurable
- [ ] PR: expose checkbox/radio indicator size as theme token
- [ ] PR: expose scrollbar dimensions as theme-configurable
- [ ] PR: expose button min-height and icon spacing as theme tokens
- [ ] PR: let a scroll container keep the wheel to itself. gpui dispatches
      `ScrollWheelEvent` in the bubble phase to every scroller whose hitbox is
      under the pointer and stops at none of them (gpui-pre-0.3.5
      `src/elements/div.rs`, `Interactivity::paint_scroll_listener`, and
      `HitboxId::should_handle_scroll`, `src/window.rs:810-812`), so a wheel
      turned inside a nested widget scrolls the page behind it by the same
      delta. None of gpui-component's scrolling widgets takes itself out of
      that list: `List`/`ListState` (`src/list/list.rs`, `render_items`),
      `Tree` (`src/tree.rs`, `RenderOnce for Tree`) and the `Scrollable`
      wrapper (`src/scroll/scrollable.rs`, `RenderOnce for Scrollable<E>`)
      render plain `div()`s with no `occlude`, which upstream uses only for
      overlays (`src/popover.rs:282`, `src/dialog/dialog.rs:572`). A consumer
      can work around it — `.occlude()` on the element that holds the
      scroller, which is what the gpui showcase now does — at the price of
      blocking hover and tooltips for everything behind that box; a consumer
      cannot get scroll chaining (the page moving on once the inner scroller
      reaches its end), because nothing reports that end to the ancestor.
- [ ] Additional PRs as gaps are discovered during connector implementation

---

## Publishing Prep

- [ ] Publish to crates.io

---

## Post-1.0 / Deferred

### Change notification
Ship without it. Users can poll `from_system()` or use their toolkit's
appearance observer. Add when there's demand.

- [ ] Linux portal: `SettingChanged` D-Bus signal via ashpd stream
- [ ] Linux KDE: `notify` crate file watching (`watch` feature)
- [ ] macOS: ObjC notification observers
- [ ] Windows: `UISettings.ColorValuesChanged` event

### Mobile readers
- [ ] iOS: `from_ios()` via `objc2-ui-kit`
- [ ] Android: `from_android()` via `jni` + `ndk`, Material You (API 31+)
