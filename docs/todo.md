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

- [x] **v0.5.9: the showcase as an application** — real chrome (the
      TitleBar as the window's title bar with menus, a toolbar sized by a new
      `geometry::toolbar`, Sidebar navigation, draggable panels, an inspector,
      a status bar) and Widget Info per widget instance, innermost wins.
      Rationale, spec and plan:
      [`todo_v0.5.9_showcase-app-rationale.md`](archive/todo_v0.5.9_showcase-app-rationale.md),
      [`-spec.md`](archive/todo_v0.5.9_showcase-app-spec.md),
      [`-plan.md`](archive/todo_v0.5.9_showcase-app-plan.md). Written 2026-09-22 at
      the maintainer's request, awaiting approval; nothing implemented.
      **Implemented 2026-09-23** (plan Tasks 1–25, from 1e159a7 on the
      v0.5.9 branch, not pushed); the three documents are archived, the spec
      with *As built* notes where the build departed from it.
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
- [x] ~~a determinate `ProgressCircle` has no size receiver the theme can
      feed~~ -- wrong, found 2026-09-22: `ProgressCircle` applies the caller's
      refinement after its per-Size arm (`progress/progress_circle.rs:187-194`),
      so a `.size()` reaches it. What is missing is a model value for a
      determinate circle's diameter, which is ours, not upstream's.
- [ ] a placeholder colour for `Input`. `Input::render` rebuilds the editor
      style from the theme on every frame and hands it `muted_foreground`
      (`input/input.rs:496-499`), which gpui-base paints the placeholder with
      (`input/base/element.rs:1825`); there is no setter. native-theme states
      `input.placeholder_color` from each platform's own placeholder colour
      (`placeholderTextColor`, `[Colors:View] ForegroundInactive`, ...), and
      `docs/inheritance-rules.toml` lists falling back to `muted_color` as a
      wrong safety net -- macOS's placeholder is about half the alpha of its
      secondary label. A `placeholder` token, or a setter on `Input`. Found
      2026-09-22 from a panel that said the placeholder read no theme field.
- [ ] the focus ring's width. `focus_ring_style` draws the ring 3px wide at
      half the `ring` colour's alpha from two module consts
      (`styled.rs:11-12`), so the platform's `focus_ring_width` -- modelled,
      and used by the connector only to switch the ring on or off
      (`lib.rs:180`) -- has no receiver. A `Theme::focus_ring_width`.
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
- [ ] a disabled `Button` should read the platform's disabled colours.
      `ButtonVariant::disabled` (`button/button.rs:1273-1284`) derives the
      whole state by multiplying: the variant's own token at 0.15 for the
      fill, `muted_foreground` at 0.5 for the text. native-theme models
      `button.disabled_background`, `button.disabled_text_color` and
      `button.disabled_opacity` (the last inheriting
      `defaults.disabled_opacity`), every platform states them, and upstream
      reads none of the three. Found 2026-09-22 while auditing the showcase's
      "Not themeable" notes, which had said "hardcoded 0.5" -- the wrong
      number for the fill and the wrong story for the text.
- [ ] the `Link` widget should read `link_hover` and `link_active`. It computes
      both from the one token instead — `link.opacity(0.8)` hovered,
      `link.opacity(0.6)` pressed (`link.rs:80, 85`) — while the two tokens
      exist, the connector writes them from `link.hover_text_color` and
      `link.active_text_color` (`colors.rs:277-278`), and a `Button::link()`
      *does* read them (`button/button.rs:1139, 1215`). So the fix is two
      lines in one widget, and it is the narrowest upstream change this audit
      has found: not a missing token, an inconsistency between two readers of
      the same pair. While there: a `Link` is underlined unconditionally
      (`link.rs:78`) and `link.underline_enabled` is a modelled bool with no
      receiver — a caller's refinement cannot remove a decoration, because
      `refine` only overrides on `Some` and `text_decoration_none` sets `None`
      (`gpui-base/styled.rs:79`, derive-refineable `refine`). Found 2026-09-22.
- [ ] a `PopupMenu` item style hook; a `Button::tooltip` style hook
- [ ] button label text size independent of rem
- [x] an iterable `IconName::ALL` generated by `icon_named!` — landed
      upstream in gpui-kit 0.6.1 without our PR: `gpui_kit_assets::IconName`
      covers the full Lucide catalog (1830 variants) with `ALL`, `PartialEq`,
      `Debug` and `Hash`; gpui-component's `IconName` stays a 101-variant
      compatibility enum (same set as 0.6.0) that converts into it via
      `From`. The connector-side follow-up is listed above.
- [ ] public base-palette fields on `ThemeConfigColors` (`red` … `cyan_light`, `schema.rs:657-668`)
- [ ] internal scrollbars reserve no gutter where the platform's scrollbars
      are not overlays. Each of these attaches `.vertical_scrollbar` or
      `.overflow_y_scrollbar`, which lays a `ScrollbarLayer` over the scroll
      area's right edge (`scroll/scrollable.rs:19-29`): the Settings page body
      (`setting/page.rs:248`, which reserves only its own `px_4`), `Dialog`
      (`dialog/dialog.rs:669`), `Sheet` (`sheet.rs:215`), `PopupMenu`
      (`menu/popup_menu.rs:1504`), `MessageScroller`
      (`message_scroller.rs:394`), `Sidebar` (`sidebar/mod.rs:470`) and `Tree`
      (`tree.rs:111`). kde-breeze's always-visible groove is 21px, so it covers
      content there. The showcase pads each `SettingGroup` by the groove width
      (`geometry::scrollbar_gutter`), which works only because a group applies
      the caller's refinement last (`setting/group.rs:112`); the page body
      takes no refinement.
- [ ] element hooks on the surfaces upstream builds itself, so an
      application can attach an id, a hover handler or a debug selector to
      them (found 2026-09-23 by v0.5.9 Task 12). A `SettingGroup` is not an
      element: it is rendered inside the page's `list` (`setting/page.rs:230-248`),
      its `render` is `pub(crate)` (`setting/group.rs:79`), and its `Styled`
      reaches only the `GroupBox` style (`group_box.rs:143`). A `Dialog`'s
      surface, padding and close button are built around the caller's title
      and content (`dialog/dialog.rs:608-725`), and so are a `Sheet`'s title
      row, padding and close button (`sheet.rs:167-245`). The showcase can
      therefore report a SettingGroup only through its enclosing `Settings`,
      and a Dialog or Sheet only on the title and content it passes in; the
      rest of each surface shows no info, which their notes say.
- [ ] the gpui connector reads `AccessibilityPreferences::high_contrast`
      nowhere but its accessor (`native-theme-gpui` `lib.rs:356-357`,
      `is_high_contrast`): `to_theme` and `apply` build the same theme with it
      set or unset, so the showcase's Preferences toggle changes only the
      stored value and the status bar's flag. Decide what it should change
      (a high-contrast preset, or the contrast rule's thresholds) or say in
      the connector's docs that it is carried and not applied.
- [x] spec v0.5.9 §2.2's action table lacks `SetPreset(key)`, which the
      command palette's preset entries run (Task 12): add it with the
      palette as its one caller (the toolbar's Combobox installs a preset
      through its own `Change` event). Added to the archived spec's §2.2 as
      an *As built* note (Task 25).

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
- [ ] File upstream iced_aw issue: Card close button ignores the caller's
      class. `Card::on_close` styles its button with a closure that reads
      `<Theme as Catalog>::default()` rather than the card's `class`
      (iced_aw 0.14.1 `widget/card.rs:193-206`), and an iced button draws its
      content in its own style's `text_color`, ignoring the `close_color` the
      card hands it at `widget/card.rs:942-948` (iced_widget 0.14.2
      `button.rs:368`, `:401-407`). The default class is `primary`, white
      (`style/card.rs:78-80`, `:141-149`), so `card::Style::close_color` never
      reaches the icon. The connector lists it as `UNREACHABLE` and the iced
      showcase dismisses its card with its own button; both go once upstream
      reads the card's class.

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
- [x] The showcase's "Text Input" tooltip claims the input background is
      `background`; `Input` paints `Theme::input_background()`, which equals
      `background` only in light mode (gpui-component 0.6.4
      `src/theme/mod.rs:379-385`). Noticed while adding the InputGroup section
      in v0.5.9; correct the tooltip (and check the sibling widgets' claims).
      **Done in the v0.5.9 showcase-app work:** the Input, Textarea,
      NumberInput, OtpInput, Checkbox, Radio, Select and Combobox infos take
      their fill claim from one mode-aware helper (`info/chrome.rs`,
      `input_background`).
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

      Forty more checked 2026-09-22, and the largest single finding is a
      **whole category the model is missing**: `Theme::motion` is a public,
      writable `MotionTokens` field (`gpui-component/theme/mod.rs:170`,
      `theme/motion.rs:8-20`: four durations, three easings, two springs, two
      distances) read by at least eighteen call sites — `Checkbox`, `Switch`,
      `Slider`, `Accordion`, `Collapsible`, `Progress`, `ProgressCircle`,
      `TabBar`, `TabPanel`, `Carousel`, the charts and the plot tooltip. The
      connector replaces the whole `Theme` on every `apply` and so overwrites
      `motion` with `Default::default()`, because **native-theme models no
      motion at all** (`model/animated.rs` is animated *icons*: frames and
      transforms, nothing about widget transitions). Four "animation:
      hardcoded" notes were therefore false — Progress, Accordion,
      Collapsible and the Switch's timing all read the theme — and the notes
      now say whose gap it is. Five others are true literals with no token
      behind them (Skeleton, Notification, Sheet, Dialog, Spinner), and the
      Carousel's note already had it right. **Decide whether to model motion**:
      the platforms do state some of it (KDE's `AnimationDurationFactor`,
      Windows' `SPI_GETCLIENTAREAANIMATION`, the existing
      `prefers_reduced_motion()` detect), and a `MotionSpec` would light up
      those eighteen readers at once.

      Also corrected: a `Button Sizes` panel called padding and min-height
      "varies per Size" where upstream copies the caller's style *before* the
      Size arm and re-applies it after (`button/button.rs:576`, `:690`), so a
      refinement wins — that demo simply omits it on purpose, and the note now
      says so. `TabBar`'s padding looked like the same shape
      (`tab/tab_bar.rs:517-518`), and that was wrong for the bar in question:
      the showcase's `TabBar` is an Underline one, whose bar and tabs have no
      horizontal padding at all, and its spacing is a per-Size gap on an inner
      row the refinement never reaches (corrected in the sixth pass below).
      The three `Link` notes became Tier U and an upstream PR candidate above.

      Six more were wrong in the plainest way — they described upstream
      inaccurately. A `Tree`'s "indent per depth level" and "hardcoded
      ChevronRight" are in neither this demo nor upstream: `Tree::new` takes a
      `render_item` closure and `tree.rs` draws no row content at all, so both
      are the application's to draw and this demo draws neither. A
      `DropdownButton`'s arrow is a `Caret` reached through
      `Button::dropdown_caret`, and its colour is the button variant's own
      text colour at 75% — themed, where the note said hardcoded. An `Alert`'s
      variant icon is a default that `Alert::icon` replaces. A `Tooltip`'s
      delay is the application's only for a tooltip set on an element: a
      `Button`'s goes through `Root`'s overlay, whose 500ms is a module const
      (corrected in the sixth pass). A
      `Sidebar`'s 255px is a fallback the caller's own `.w()` displaces. And a
      `DataTable`'s row height has an escape hatch.

      A third batch found the mistake behind several of the others at once:
      **gpui-component sets the rem to `Theme::font_size`** (`root.rs:582`), so
      a `rems()` or a `text_base()` upstream is already the platform's size.
      Six Button panels had called their label size Tier U when it is
      `button_text_size` — `text_xs`/`text_sm`/`text_base`, a fixed ratio of
      the platform's body text. The `Headings` panel listed px figures that
      were its rem ladder at a 16px rem, which no bundled preset produces. A
      `Buttons with Icons` panel listed its icon colour as not themeable when
      it follows the button's own text token, and its icon size is rems too.
      And an `Editor`'s line height is only what the widget sets *first*:
      upstream applies the caller's refinement last and says in a comment that
      this is on purpose. Seven more were true and gained the citation that
      proves it — Breadcrumb, Clipboard, Calendar and DatePicker glyphs built
      inline with no setter, `DescriptionList` spacing, `Empty`'s dashed
      border, and a `ButtonGroup` that has no gap to set because it joins its
      buttons by turning edges off.

      A fourth pass sharpened the three remaining `inner element (Tier U)`
      notes that could be checked cheaply. All three are Tier U, but each was
      hiding half the answer: a `Checkbox`'s indicator and a `Select`'s caret
      are sized in rems and so already follow the platform's font, and the
      caret's *colour* is themed. What is genuinely unreachable is the
      absolute-px field the model states beside each -- `indicator_width`,
      `arrow_icon_size` -- because both consumers fold `Size::Size` into the
      catch-all arm.

      A fifth pass took the "unmodelled widget" bucket, and it collapsed:
      Alert, Tag, Kbd and Breadcrumb are each `Styled` and apply the caller's
      refinement after their own paddings, so "hardcoded" was wrong about
      every one of them -- what is absent is a model, not a receiver. That
      pass put `Badge` in the same list, and the sixth pass found it does not
      belong there: `Badge::render` applies the refinement to the wrapper
      around the badged element (`badge.rs:116`), and the pill is an absolute
      child built after it with its own literals, so nothing reaches it.
      Two outright falsehoods fell out with it: a `Kbd` was said to use a
      monospace font, and `kbd.rs` sets no family at all, so it inherits the
      window's platform UI font; and a `Label`'s "font weights: hardcoded"
      turned out to be the opposite -- nothing sets a weight anywhere, which
      is now its own item above.

      After five passes: sixty-three entries audited, thirty-one wrong. The
      73 left uncited were set aside as "API facts no theme could set" --
      and that label was itself unchecked.

      **A sixth pass (2026-09-22) read all 73: sixteen were wrong, and four more were themed facts filed under the wrong heading.** A
      disabled `Button`'s cursor is the default arrow, not `not-allowed`. The
      `Loading Button` demo showed no spinner at all, because a `Button` draws
      its spinner *in place of its icon* and that one had none (the demo now
      has one). An `Input`'s placeholder is painted with `muted_foreground`,
      a theme field, where the note said it was not -- and the model states
      `input.placeholder_color` from each platform's own placeholder colour, so
      that is Tier U. A `DatePicker` defaults to `%Y/%m/%d`, not `YYYY-MM-DD`.
      An `AvatarGroup` drops the fourth avatar with no marker, because its
      overflow marker is a `⋯` avatar, not a `+N` count, and only
      `.ellipsis()` adds it. `Empty`'s "hardcoded 2rem" is rems and settable.
      Markdown headings are sized from a **fixed 14px**, not from the base
      font. The code `Editor`'s background is a fixed `#0a0a0a`/`#ffffff`,
      because the highlight theme the connector installs sets one and the
      `input_background()` fallback is never reached. A `Popover`'s anchor is
      an `Anchor`, not a `Corner`; a `Sidebar`'s children implement
      `SidebarItem`; a `ContextMenu` needs `InteractiveElement` too; a
      candlestick's colours are `chart_bullish`/`chart_bearish`, not green and
      red. And the **`AppMenuBar` was empty**: it reads gpui-base's
      `GlobalState` menus, which only `set_app_menus` fills, and the showcase
      only called `cx.set_menus` (fixed: it now fills both).

      Fifteen entries in all were themed facts filed under "Not themeable":
      those four -- the icon sizes and paddings the `Attachment`, `Toolbar` and
      `Dialog` demos apply themselves -- the `Dialog`'s footer gap, and the ten
      Button panels' font-weight, carried by `geometry::button` since the
      first pass. They are config lines now.

      Re-reading found three of this audit's own corrections wrong -- the
      `Badge` padding (above), the `Tooltip` delay and the `TabBar` padding --
      and three more cited entries: the `Badge`'s text and size notes, and a
      `SidebarToggleButton` icon called hardcoded that is 1rem.

      **The colour claims have the same failure, and the gate cannot see
      it.** The citation gate checks that the cited line reads the named
      token -- not that the line is on this widget's path, in this demo's
      variant. Ten claims named the wrong token and passed: the navigation
      `TabBar` is an Underline bar, and its "active bg `tab_active`" and "bar
      bg `tab_bar`" are the `Tab` variant's (an Underline bar has no fill and
      marks the active tab with a `primary` underline); a `Calendar` has no
      `popover` fill; a `Collapsible` paints neither the `accordion` fill nor
      the `border` it was given; a `Rating`'s empty star, a `Pagination`'s
      page numbers and a `SidebarToggleButton`'s icon are not `foreground`;
      a `Combobox` row hovers with `accent`, not `list_hover`; and a
      `GroupBox`'s edge is the card's, through `geometry::group_box_content`.
      Nine more had the right token at a line borrowed from another widget,
      and five config lines were wrong (three `radius` lines the builders
      override or the variant ignores, a `Calendar` that is `radius_lg`, and
      a `ProgressCircle` that draws `spinner.diameter` at 75%).

      Tally after six passes: 136 "Not themeable" entries audited, 47
      wrong, 15 misfiled across the section, and three of the corrections
      themselves wrong.

      **A seventh pass (2026-09-22) read the 100 entries that already
      carried a citation when the audit began** and had not changed since
      -- the prose gate held their symbols, nothing held their meaning.
      Seventeen were wrong and nine more were true but hid the resolvable
      half. The wrong ones repeat the earlier shapes. "Hardcoded" said of
      rems a refinement reaches: a `Bubble`'s padding, a `Message`'s slot
      gap, a `Marker`'s row gap, a `TitleBar`'s 34px (settable; the model
      states no title-bar height). Fills called absent that are painted: an
      unchecked `Checkbox`, a `Radio` and an `OtpInput` box are all
      `input_background()`, and a `Carousel` does draw a `ring` focus ring.
      And plain misreadings: a `Radio`'s indicator is a circle, not half the
      theme radius; a `Textarea`'s rows are Input's 1.25rem, not the font's
      line height; an `Alert`'s tints are its colour faded into transparent
      white, not mixed toward white; `sheet.margin_top` is gpui-component's
      own setting, not a native-theme field; gpui *does* have a motion switch
      and `with_animation` honours it; the spacing tokens have a reader (a
      `Dialog`'s viewport margin), they just have no field behind them. Two
      Tier U verdicts were wrong the other way: a `Toggle` folds the caller's
      refinement into its *checked* style, so `segmented_control`'s active
      colours reach a checked toggle, and its size is settable too.

      The colour claims beside them had the same rate. **A named `Avatar`
      takes no theme colour at all** -- fill, initials and edge are OKLCH
      literals hashed from the initials, twelve hues -- so the three claims
      on that panel and one on `Message` described an anonymous avatar the
      demo does not show. An `OtpInput`'s digits are `foreground`, not the
      `secondary_foreground` of a masked asterisk. `MessageScroller`'s jump
      button is refined to `background`/`border`/`foreground` over its
      `secondary` variant. And **the theme's `shadow` flag has one reader**:
      `ButtonVariant::shadow` is `false` for every variant but `Custom`
      (`button/button.rs:1050-1055`), so the "shadow" config line on eleven
      panels -- six buttons, Input, NumberInput, Checkbox, Radio, Slider --
      described nothing on screen.

      Tally: 236 "Not themeable" entries audited, 64 wrong, 15 misfiled.

      **An eighth pass (2026-09-22) read every remaining colour claim
      against the code that paints the demo**, not only the cited line. The
      recurring error is a claim that names upstream's token where a
      `geometry::` builder the demo applies has replaced it: ten builders set
      a corner radius and seven a colour, and they land last. So the
      "border-radius: radius" line on eighteen panels -- ten buttons,
      Textarea, InputGroup, NumberInput, Combobox, Select, Radio, Dialog,
      AlertDialog -- showed a radius the builder overrides, and is gone; a
      Radio's and a Select's label colour is the platform's font colour, not
      the `foreground` the claim named (both now say "upstream"). Four claims
      described a variant the demo does not build: an icon badge, a `Label`'s
      highlights, an outline `Kbd`'s edge, an empty `ColorPicker` swatch (the
      demo's has a value, painted with itself). Three had no reader at all:
      `ThemeColor::list` on List and Tree, and `group_box` on a Settings page,
      whose groups are `GroupBoxVariant::Normal`. And the code `Editor`'s
      edge is the Input frame's `input`, while the `border` it claimed paints
      the indent guides. Fourteen more cited a line from another widget
      (InputGroup, NumberInput, Combobox and DatePicker borrowed `Input`'s;
      DataTable borrowed the declarative `Table`'s header and a column
      selection line for its row selection).

      Tally for the colour claims: 374 read, 30 wrong, 25 cited on a
      borrowed line; for the config lines, 36 wrong (eleven `shadow`, twenty
      radii, five others). The audit of the Widget Info panels is complete:
      every "Not themeable" entry, colour claim and config line has now been
      read against the source that paints it.

##### What the audit turned up that is ours to fix

Each of these is a real route the connector or the model does not take. They
are listed separately from the notes because correcting a note only records
the gap — closing it is a change, and each wants its own decision.

- [ ] **Model motion.** The big one, argued above: `Theme::motion` is a
      writable field of twelve tokens that eighteen-plus widgets read, and
      `apply` overwrites all of it with `Default::default()` because
      native-theme states no motion. KDE has `AnimationDurationFactor`,
      Windows has `SPI_GETCLIENTAREAANIMATION`, and
      `detect::prefers_reduced_motion()` already exists. A `MotionSpec` on the
      theme plus one line in `apply` would light up Checkbox, Switch, Slider,
      Accordion, Collapsible, Progress, ProgressCircle, TabBar, TabPanel,
      Carousel, the charts and the plot tooltip at once. Decide first whether
      a *theme* should carry motion at all, or whether this belongs with the
      accessibility preferences.
- [ ] **A `DataTable` can take the platform's row height today.** `Size::Size(px)`
      returns the pixel value verbatim from `table_row_height`
      (`sizing.rs:57-65`), and `table_cell_padding` has *no* `Size::Size` arm
      (`:67-95`), so it falls to the same Medium edges a default table already
      uses — `DataTable::with_size(Size::Size(px(list.row_height)))` changes
      the row height and nothing else. `list.row_height` is modelled and
      `geometry::list_item` already applies it to `List` and `Tree` rows.
      Caveat worth reading before using `Size::Size` more widely: `smaller()`
      maps it to `v * 0.2` and the private `as_f32` returns the raw pixels
      where the enum arms return 0..3, so anything ordering sizes numerically
      misreads it.
- [ ] **There is no ambient `font.weight`.** The `geometry::` builders that
      carry a font spec already carry the weight with it — `with_text` sets
      `font_weight` alongside the size (`geometry.rs:80-83`), which covers
      `input`, `menu_item`, `list_item`, `tooltip`, `status_bar`,
      `dialog_title`, `dialog_description`, `table`, `checkbox` and the rest,
      plus `button` since v0.5.9. What has no weight is everything *outside*
      a builder-styled element: gpui-component's `Theme` has no font-weight
      field, and `Root::render` sets the family, the rem size and the
      foreground but not a weight (`root.rs:582-596`), so a plain `Label`, a
      heading or any text an application draws in a `div()` renders at gpui's
      default rather than the platform's. **The route is cheap**: `Root` is
      `Styled` and applies the caller's refinement last (`root.rs:574-578`,
      `:596`), and an application constructs it — the showcase does, at
      `Root::new(showcase, window, cx)` — so one `.font_weight()` there
      cascades the platform's body weight to the whole window, builders
      included. Decide whether the connector should offer that as a root
      helper. Found 2026-09-22 from a `Label` panel that called its font
      weight hardcoded when in fact nothing sets one.
- [ ] **Model the widgets the panels keep apologising for.** Alert, Tag, Kbd
      and Breadcrumb all turn out to have the *same* answer: each is `Styled`
      and applies the caller's refinement after its own paddings
      (`alert.rs:204`, `tag.rs:266`, `kbd.rs:253`, `breadcrumb.rs:175`), so
      upstream is not the obstacle — native-theme simply states no such
      widget, so there is nothing to carry. `Badge` is the exception, and an
      earlier version of this item listed it wrongly: its refinement lands on
      the wrapper around the badged element (`badge.rs:116`), and the pill is
      an absolute child built after it, so a badge model would also need an
      upstream receiver. Worth checking
      `platform-facts.md` for what the four platforms state about each before
      deciding; several (a KDE/Adwaita "tag" or "badge") may have no platform
      source at all, which would be the honest reason not to model them.
- [ ] **Carry `defaults.line_height` to the code editor.** `Editor` sets
      `relative(1.5)` and then applies the caller's refinement last, with a
      comment saying that is deliberate so a text style set on the editor
      refines over it (`input/editor.rs:137-143`, `:159`).
      `defaults.line_height` is modelled — 1.4 on the bundled defaults — so
      nothing new has to be modelled: it wants a builder, or a line in an
      existing one.
      **Update (v0.5.9 unstated sizes):** `control_height` is gone, and the
      builders now apply the line height themselves: `geometry::button`,
      `input`, `select`, `combobox`, `menu_item` and `list_item` set
      `defaults.line_height` as the control's line height (`with_height_rule`
      in `geometry.rs`). So an `Input` or `Textarea` refined with
      `geometry::input` takes it over upstream's `1.25rem` rows
      (`input/input.rs:699`, set before the caller's refinement at `:719`).
      The `Editor` is what remains.
- [ ] **The showcase ignores its own `text_scale`.** `ResolvedTextScale` states
      four typographic roles — `caption`, `section_heading`, `dialog_title`,
      `display` — each with a size, a weight and a line height, and
      `native_theme_gpui::text_scale()` is public (`lib.rs:412`). The Headings
      demo draws a six-step rem ladder of its own instead, and there is no
      `geometry::` builder for a text role, so an application that wants the
      platform's scale has to reach past the builders. Four roles do not map
      onto H1–H6, which is the honest reason this is a design question and not
      a bug: decide the mapping (or that there is none) before adding a
      builder. The showcase's "no hardcoded style values" test does not cover
      text sizes, which is why the ladder passed. Note what the ladder is *not*
      doing wrong: **gpui-component sets the rem to `Theme::font_size`**
      (`root.rs:582`), which the connector fills from the platform's
      `font.size`, so every `rems()` and every `text_xs`/`text_sm`/`text_base`
      in the toolkit is already proportional to the platform. What `text_scale`
      would add is the *weight* and *line height* of each role, and sizes that
      are the platform's rather than a ratio of its body text.

      That one line in `root.rs` is worth remembering before writing another
      note: a great many "hardcoded" sizes upstream are rems, and a rem here is
      the platform's font size. Six Button panels called their label size
      Tier U on exactly that mistake.
- [ ] **`Size::Size(px)` is an escape hatch, but read the target first.** It is
      the only way to hand a pixel value to a widget that sizes itself from the
      `Size` enum, and the `geometry::` refinements being applied last means
      its collateral effect on the *outer* box is overwritten anyway — so what
      is left is its effect on inner elements, which is exactly the Tier U set.
      It is not uniform, though, and each target has to be read: `table_row_height`
      returns it verbatim, `table_cell_padding` has no `Size::Size` arm at all,
      a `Button`'s icon takes `v * 0.75` (`button/button.rs:580`), and
      `button_text_size` has no `Size::Size` arm either (`sizing.rs:319-325`),
      so a button's label cannot be sized this way. Nor can a select caret
      (`select.rs:60-63`, `Size::Size` folded into the `_` arm with `Medium`)
      or a checkbox indicator (`checkbox.rs:219-224`, same shape) — which is
      what makes `combo_box.arrow_icon_size` and `checkbox.indicator_width`
      genuinely Tier U rather than merely unapplied. And a `ProgressCircle`
      draws `Size::Size(s)` at `s * 0.75` (`progress/progress_circle.rs:193`)
      where a `Spinner` takes it whole (`icon.rs:182`), so the showcase's
      `geometry::spinner_size` on its indeterminate circle draws 75% of
      `spinner.diameter` -- the circle applies the caller's refinement last,
      so a `.size()` from the model would reach it exactly. Worth a survey of
      every `Size` consumer before leaning on it anywhere.
- [ ] **A gate the audit broke, and what that says about the others.** Writing
      `(select.rs, Caret::render)` into a note made `Caret` a "shown" widget
      in `check-widget-coverage.py`, because the gpui side read string
      literals as code — on the explicit reasoning, written into its own
      docstring, that the tightened match made literals harmless. It did not:
      the audit's citations have exactly the matched shape. Fixed by stripping
      literals on both sides, which then revealed that `Dialog`, `AlertDialog`
      and `Sheet` had been passing on prose mentions too. Worth asking of
      every other gate here: **which of them reads prose as evidence?** The
      builder-coverage and omission tests in `showcase.rs` already separate
      code from notes (`without_comments_or_strings`, `string_literals_only`),
      so they are clean; the citation gate reads only string literals by
      design. This one was the outlier, but nothing checked that.
- [x] **"Not themeable" has become a bucket, and that is the volume complaint.**
      Of its 247 entries, 207 now carry an upstream citation, a Tier U verdict
      or a named model gap — the audit's output. The other **40 are not
      resolvability claims at all**: they are API and demo facts that no theme
      on any platform could set. A `ContextMenu`'s trigger is right-click, a
      `PieChart` takes an `inner_radius`, an `OtpInput` has two groups, a
      `Sheet` can be placed on any of four edges, the chart panels list nine
      such options between them. Every one is true and every one is filed
      under a heading that says the theme cannot set it — which is also true,
      and useless: a reader scanning for *what the native theme did* has to
      step over them.

      The fix is a fourth section, not a deletion — that information is worth
      having, just not under that heading. `widget_tooltip` and
      `widget_tooltip_themed` would take one more slice and `hover_info` a
      sixth argument, across 107 call sites: mechanical, and large enough that
      it wants a decision first. Proposed heading: **"This demo:"**, leaving
      "Not themeable" for what the audit actually produced. Decide before the
      next pass, because every note corrected in the meantime is a note that
      may have to move. Two found while
      auditing: a `Select`'s and a `Combobox`'s caret are painted with
      `muted_foreground` (`select.rs:593`, `combobox.rs:655`), and a
      `DropdownButton`'s with the button variant's own foreground at 75%
      (`button/button.rs:732`). All three are theme reads no panel claims,
      because a claim carries a swatch and these are parts of a widget rather
      than the widget. The omission report does not catch them either — the
      files are cited, so their fields count as named. Decide whether the
      colours section should cover a widget's *parts*, and if so whether
      `muted_foreground` on a caret is even right: the platform states
      `combo_box.font.color` for the trigger, and a dimmed arrow is upstream's
      choice, not the platform's.
      **Closed by the v0.5.9 showcase-app work:** the per-instance
      `WidgetInfo` has a fourth section, *This instance*, for the API and
      demo facts, and *Not themeable* keeps the resolvability verdicts. The
      parts question stays open: a Select's and a Combobox's caret are
      still a *Not themeable* note that says its colour is themed
      (`muted_foreground`), not a colour claim with a swatch.
- [ ] **A sidebar width and a tooltip delay are missing from the model.** Both
      have receivers: `Sidebar` reads the caller's own style width and falls
      back to 255px only when none is set (`sidebar/mod.rs:195-201`), and gpui
      takes a hover delay on the element carrying the tooltip
      (`gpui-pre/elements/div.rs:735`) -- but only on a tooltip set on an
      element: `Button::tooltip` goes through `Root`'s overlay, whose 500ms
      `SHOW_DELAY` is a module const (`gpui-base/tooltip.rs:14`), so for a
      button the delay has no receiver at all. `SidebarTheme` and
      `TooltipTheme` state neither, so both notes were "hardcoded" for
      something the application can partly set already. Check `platform-facts.md` for what the four
      platforms state — a hover delay at least is a documented system setting
      on Windows (`SPI_GETMOUSEHOVERTIME`).
- [ ] **Platform icons inside widgets: how far can it go?** `Icon` takes a path
      or raw SVG bytes (`icon.rs:117-131`), and this connector already
      rasterises the platform's icon theme, so the type is not the obstacle —
      the per-widget setter is. `Alert::icon` exists (`alert.rs:124`), so an
      Alert's variant glyph is a default and not a fixture; `Breadcrumb`
      (`:143`), `Clipboard` (`:88-90`) and `Calendar` (`:107-110`) construct
      theirs inline with no setter, and `Button::dropdown_caret` goes through
      `Caret` (`select.rs:59`). Worth deciding whether the connector should
      offer a "platform glyph" helper at all, and worth an upstream ask for
      setters on the three that lack one. Note the standing rule: never
      substitute across icon themes — a missing icon returns `None`.
- [ ] **`geometry::button` gives every variant a border, and hover takes it
      back.** The builder sets `.border(button.border.line_width)` and
      `.border_color(button.border.color)` (`geometry.rs:132-133`), and Button
      applies it last (`button/button.rs:690`) -- so Primary, Secondary, the
      filled variants, Ghost, Link and Text all get a platform-coloured edge
      upstream draws only on Default and outline buttons. On hover and press
      upstream's own hover style sets `border_color` again
      (`button/button.rs:1136`, `:1213`), and gpui applies a hover style after
      the base refinement (`gpui-pre elements/div.rs:3408-3432`), so the edge
      turns `primary`, `border` or a `button_*` token -- or `transparent` on
      Ghost, Link and Text, where it vanishes. Decide whether the builder
      should set a border only for the variants that have one (a separate
      `geometry::flat_button`, or a width-free variant), and note that the
      hover colour itself is out of reach: a refinement cannot set a hover
      style. Not checked visually. Found 2026-09-22.
- [ ] **`geometry::list` could paint the list.** `ThemeColor::list` has no
      reader (the contract says so at `contract.rs:1784`) and a `ListItem`
      paints no idle background, so `list.background_color` reaches nothing.
      On the bundled presets it falls back to the window background
      (`resolve/inheritance.rs:143-144`), so nothing is lost there; on a live
      KDE system it is `[Colors:View] BackgroundNormal` (`kde/colors.rs:91`),
      the view background rather than the window's, and that is dropped. `geometry::list` lands on the element around the rows, which
      is exactly where a fill belongs, and carries only the border
      (`geometry.rs:362-369`). One `.bg()` would carry it. Found 2026-09-22.
- [ ] **A `geometry::toggle`.** A `Toggle` applies the caller's refinement
      last (`button/toggle.rs:215`) *and* folds it into its checked style
      (`:207-212`, gpui-base `toggle.rs:91-101`), so `segmented_control`'s
      `segment_height`, padding, font and -- on a checked toggle --
      `active_background` and `active_text_color` all have a receiver. Only
      an unchecked toggle's hover is out of reach (Tier U: it is set with
      `.hover()` on the base element). Found 2026-09-22 from a panel that
      called the whole thing Tier U.
- [ ] **`Switch::color` is a receiver nothing feeds.** `switch.checked_background`
      is modelled and every preset states it; `ThemeColor` has no field for
      it, which is what `colors.rs`'s Issue 51 note and the contract's
      `NoReceiver` entry record -- but `Switch::color` (`switch.rs:95`) takes
      it per instance. A helper, or a `variants::` function like
      `ghost_button`, would reach every switch an application builds. Found
      2026-09-22.
- [ ] **`Theme::shadow` has one reader.** The connector sets it from
      `border.shadow_enabled` (`lib.rs:176`); gpui-component reads it only at
      `button/button.rs:612`, for a `ButtonVariant::Custom` built with
      `.shadow(true)`. The semantic shadow tokens it feeds have no reader
      outside `theme/`. So the platform's shadow preference reaches nothing
      an application builds by default. Found 2026-09-22 from eleven panels
      that listed it as theme config.
- [x] **The showcase's animated icons read the platform, not gpui's switch.**
      The frame timer checks `detect::prefers_reduced_motion()`
      (`showcase-gpui.rs:1947`, before the example became a module tree),
      while the spinning icons go through
      `with_animation`, which honours `App::reduce_motion`
      (`gpui-pre elements/animation.rs:74-80`) -- the switch `apply_system_theme`
      forwards the platform's preference into, and one an application can
      also set itself. Reading `cx.reduce_motion()` would make the two agree.
      **Done (d1045eb):** the Icons page and the frame timer both read
      `cx.reduce_motion()`. Under reduced motion the timer still wakes; each
      tick returns without advancing a frame (`app.rs`, the frame timer).
- [x] **The model's toolbar is read by nothing.** `ToolbarTheme` states
      `bar_height`, `item_gap`, `icon_size`, `font`, `border` and
      `background_color`, every bundled preset states the first two (KDE 40px
      and 0, Adwaita 47px and 6, iOS 44px and 8), and the KDE reader fills
      `toolbar.font` from `toolBarFont` -- and no line in either connector
      reads any of them. There is no `geometry::toolbar`, so the showcase's
      application-drawn toolbar borrows the button's control height, the
      generic `widget_gap` (6px on KDE, where the platform says 0), the
      `container_margin` and gpui-component's `tab_bar` colour, and
      `icon_size_toolbar` reads `defaults.icon_sizes.toolbar` rather than
      `toolbar.icon_size` (which inherits it, so they only differ where a
      platform states a toolbar-specific size). A toolbar is the textbook
      application-drawn row, so this is the builder an application needs
      most. Found 2026-09-22.
      **Done (b3dea4f):** `geometry::toolbar` carries `bar_height` (as a
      minimum height), `item_gap`, the `border` padding, `background_color`
      and the `font` size and weight, and `icon_size_toolbar` reads
      `toolbar.icon_size`. The showcase's toolbar is drawn with both. What
      checking the presets against platform-facts §2.13 found is the next
      item.
      **Update (v0.5.9 unstated sizes):** Adwaita's 47px above is the
      headerbar's; GNOME's `.toolbar` row sets no minimum height (platform-facts
      §2.13, corrected per ruling R6). `toolbar.bar_height` is `Option<f32>`
      now, adwaita, kde-breeze and the colour-scheme presets state none, and
      `geometry::toolbar` sets its minimum height only where one is stated.
- [x] **KDE's toolbar height has no source, and no preset states a toolbar
      padding.** Checked 2026-09-22 against platform-facts §2.13 before
      `geometry::toolbar` relied on the presets:
      - `kde-breeze.toml:198`/`:478` and `kde-breeze-live.toml:113`/`:272`
        state `bar_height_px = 40.0`, where §2.13's KDE column says
        **(none)** -- sizes to content (`platform-facts.md:1340`). The value
        has no source. It entered in e3ecee8 (2026-03-27, the per-widget
        preset migration), which replaced KDE's `[widget_metrics.toolbar]`
        -- `item_spacing = 0.0` and `padding = 6.0`, no height -- with a
        `[toolbar]` table that added `height = 40.0`. 40 is the toolbar
        height the removed generic `default.toml` and the community presets
        carry; no comment in the preset, no platform-facts row and no reader
        states it (`kde/metrics.rs:95` sets only `item_gap`). It survived the
        renames in 1b97af1 and 51d44ea, and the v0.5.4 audit saw it and
        passed it as "OK -- reasonable"
        (`docs/archive/v0.5.4_native-theme.md:3474`). A preset has to state
        something, because `ResolvedToolbarTheme::bar_height` is a plain
        `f32`; so `geometry::toolbar` applies it as `min_h`, a floor the row
        may grow past, not a fixed height. Whether the model should let KDE
        leave it unstated is the open question.
      - No preset states `toolbar.border.padding_*`, so it resolves to 0 on
        every preset (a widget border's padding defaults to 0,
        `resolve/validate_helpers.rs:337`) and `geometry::toolbar` pads the
        row by 0. §2.13 states 6 for KDE (`ToolBar_ItemMargin`), 6 for GNOME,
        8 (measured) for macOS and 4 left / 0 right for Windows. Every preset
        had a `toolbar.padding` until 1b97af1 (2026-04-07, the schema rename
        per property-registry.toml) dropped it without carrying it to
        `[toolbar.border] padding_horizontal_px`.
      - The Windows reader sets `toolbar.item_gap = 4.0`
        (`windows.rs:240`, `:296`) where §2.13 and `windows-11.toml:217`
        say 0.
      The rest agrees with §2.13: bar height 38/47/48 (macOS, GNOME, Windows
      compact), item gaps 8/6/0 (macOS, GNOME, Windows) and 0 (KDE), and the
      toolbar icon size, which every preset leaves to
      `defaults.icon_sizes.toolbar` (22 KDE, 16 GNOME, 20 Windows, 24 macOS
      small mode). No preset value was changed.
      **Done (24da5c1, 70d705f), all three findings:**
      - `toolbar.bar_height` is `Option<f32>` (24da5c1), and kde-breeze and
        kde-breeze-live no longer state one (70d705f). Nor do adwaita, whose
        47 was the headerbar's (ruling R6), and the colour-scheme presets,
        whose 40 (material's 64) had no source. `geometry::toolbar` sets its
        minimum height only where one is stated; macos-sonoma's 38 and
        windows-11's 48 remain.
      - An unstated padding side resolves to `None`, not 0, and the native
        presets state §2.13's toolbar padding: KDE 6 and GNOME 6 on every
        side (both cells corrected: Qt pads a toolbar's items on all four
        sides, `qtoolbarlayout.cpp:87-89`; libadwaita's `.toolbar { padding:
        6px }`), macOS 0/8/0/8, Windows 0/0/0/4.
      - The Windows reader's `toolbar.item_gap` is 0.
      The gate `native-theme/src/presets/documented_sizes.rs` now holds the
      presets and the readers to §2.13. Plan:
      [archive/todo_v0.5.9_unstated-sizes-and-chrome-ux-plan.md](archive/todo_v0.5.9_unstated-sizes-and-chrome-ux-plan.md).
- [ ] **The code editor's background is a fixed colour.** `Theme::editor_background`
      returns the highlight theme's `editor.background` and falls back to
      `input_background()` only when that is unset (`theme/mod.rs:389-394`).
      The connector installs `HighlightTheme::default_dark/light()`
      (`lib.rs:192-196`), whose `editor.background` is `#0a0a0a` and `#ffffff`
      (`theme/default-theme.json:314`, `:114`), so the fallback never fires
      and every code editor -- the `Editor` widget and any `Input` in code
      mode (`input/input.rs:640-641`), gutter included -- paints a fixed
      near-black or white whatever the platform. Every `HighlightThemeStyle`
      field is public, so the connector can clone the default and clear
      `editor_background` (letting upstream fall back to the platform's input
      background) or set it, and the same goes for `editor_active_line`, the
      gutter and the line-number colours. The syntax colours themselves are a
      separate question: native-theme models none. Found 2026-09-22.
- [ ] **Markdown headings are sized from a fixed 14px.** gpui-base draws a
      heading at `rems(2.)`..`rems(1.)` resolved against `heading_base_font_size`
      (`gpui-base text/node.rs:2891-2901`), which defaults to `px(14.)`
      (`gpui-base text/style.rs:88`), and gpui-component's
      `base_text_view_style` never sets it (`text/mod.rs:34-61`) -- so an H1 is
      28px on every platform while the body text around it follows the
      platform font. `TextViewStyle::with_heading_base_font_size` and
      `with_heading_font_size` are public; find out whether an application can
      install its own `TextViewDefaults` without upstream overwriting them on
      the next theme change, and then whether the platform's `text_scale`
      roles should drive the heading sizes. Found 2026-09-22.
- [ ] **A tab's `min_height` has a receiver nobody uses.** A `Tab` takes the
      caller's style first and then sets its own per-Size `.h()`, text size and
      colours over it (`tab/tab.rs:780-810`), but it never sets `min_h`, and
      the layout honours a minimum over a height. So `tab.min_height` (and
      `min_width`, and a font weight, which the tab sets nowhere either) would
      reach a tab the application builds -- there is simply no `geometry::tab`.
      The showcase builds its tabs from strings, so it would need to build
      `Tab`s to use one. Found 2026-09-22.
- [ ] **An empty state's icon outgrows its frame.** `EmptyMedia` is a 2rem
      square (`empty.rs:217`), and the showcase puts
      `defaults.icon_sizes.large` in it -- 32px on most presets and 48px on
      KDE -- which fits only where 2rem reaches the icon, i.e. a 16px body
      font (24px for KDE's 48). `EmptyMedia` applies the caller's refinement
      last, so the frame can be sized to its icon; decide whether the
      showcase should, or use a smaller icon role. Not checked visually.
- [ ] **The colour gate cannot tell whose line it is.**
      `every_colour_claim_is_read_at_the_line_it_cites` checks that the cited
      line reads the named token -- not that the line runs for *this* widget,
      in *this* demo's variant. Ten wrong claims passed it (the navigation
      TabBar's two `Tab`-variant fills, a Calendar's non-existent `popover`
      fill, a Collapsible's `accordion` fill and border, and five claims
      borrowed from a `Label`, `ListItem` or `Button` line for widgets that
      paint something else). A mechanical fix is hard -- it
      would need the render path -- but the audit of the remaining claims is
      not, and the rate says it is due: of the ~45 claims this pass read, ten
      named the wrong token.
      **Spike outcome (v0.5.9 showcase-app plan, Task 22):** the painted
      scene is reachable. gpui-pre 0.3.6 exposes the last frame's quads to
      tests (`Window::painted_quads`, `window.rs:2718-2725`, under
      `test-support`, which the connector's dev-dependency enables), and
      three windowed tests compare a painted fill with its claim:
      `a_tags_painted_fill_is_its_bg_claim` (the Primary Tag at rest),
      `a_hovered_tags_info_names_its_painted_fill` (the same Tag at 90%) and
      `a_swatchs_painted_fill_is_its_value_claim` (a Theme Map swatch). The
      gap that remains: those are three instances, not every claim. Every
      other claim is still checked only against the line it cites, and a
      general check would have to find each claim's painted quad —
      `painted_fill` (`tests.rs`) takes the largest opaque quad inside an
      element's bounds, which suits a filled widget and not a text colour,
      an edge or a state the test does not put the widget in.
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
      widget and their self-tests. The gpui half is done: the example is
      `examples/showcase-gpui/`, a module tree (ab3f7d6). `showcase-iced.rs`
      remains one file.
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
- [ ] The gpui showcase builds two gpui-base widgets outside `demo.rs` and
      `chrome.rs`, against spec §5.3 of the showcase-app spec: the body's
      `h_resizable("body")` and its three `resizable_panel()`s in `app.rs`
      (`Showcase::render`). `every_widget_reports_itself` does not see them,
      because it reads constructors as `Type::function(` (spec §10.1) and
      these are free functions. The handles report themselves
      (`demo::resize_handles`), but the `ResizablePanelGroup` and its panels
      have no Widget Info. Either build the group in a `chrome::` helper that
      reports it, or extend the gate to upstream free functions returning a
      widget, or both.
- [ ] **Regenerate the gpui screenshots and review the showcase's visible
      changes.** The v0.5.9 showcase-app plan (Tasks 1–25) changed what the
      gpui showcase draws, and nobody has looked at it on screen: the plan's
      tasks lay it out headlessly and never ran
      `scripts/generate_gpui_screenshots.sh`, which is maintainer-run because
      it drives the desktop. Changes to check, by task:
      - Task 9, the toolbar: the preset Combobox and the icon-set Select have
        no literal width (they take `combo_box.min_width` and grow with their
        content, where both were 260px); the icon buttons are laid out at a
        labelled Button's height, `h_8` (2rem); on kde-breeze the items touch,
        because `toolbar.item_gap` and the toolbar padding are 0 there, and a
        vertical Separator's line overflows its 0px box. *Superseded by the
        unstated-sizes plan below:* the three theme controls moved to the
        Sidebar's header, the toolbar is padded, and the Separator is gone.
      - Task 10: the window is 1380px wide (`NAV_WIDTH` 200 + 880 for the
        page + `INSPECTOR_WIDTH` 300), with the Sidebar, the resizable panels
        and the inspector. *Superseded, then restored:* `NAV_WIDTH` was 205
        while the Mode row was a switch; with the Mode row a Select it is 200
        again, and the window 1380px. *Superseded by the showcase-layout
        plan below:* two panels, and the window 1180px.
      - Task 12: the Preferences sheet is 600px wide (`PREFERENCES_WIDTH`).
      - Task 14: page headings are sized to their text (`self_start`), not
        the page's width.
      - Task 16: List, Tree and DataTable row text takes the theme's list
        font; the alignment of the standalone outgoing Bubble (`ml_auto` in
        its block wrapper).
      - Task 18: the strikethrough sample is no longer also underlined; the
        masked sample masks only the secret, with its caption as page text.
      - Task 19: the Layout page's Settings sidebar is upstream's 250px (the
        showcase's `sidebar_width(140)`, which upstream clamped to 160, is
        gone); GroupBox content is plain text, so it takes the GroupBox's
        colour.
      - Task 20: the Dialog and the two Sheets take upstream's default sizes
        (448px, 350px); the Popover content lost its extra `p_4`; the Open
        Dialog and Click for Menu triggers take `geometry::button`; the
        Dialog's Close button sits at the footer's right; menu rows turn
        their text `accent_foreground` on hover.
      - Task 21: the HoverCard content lost its `container_margin` inset
        inside the card's own padding.
      - Task 22: the Spin card turns (a Transform indicator, drawn as a mask
        in the foreground colour) where it pulsed; choosing gpui-component's
        built-in icon set shows its icons.
      - Task 24: section headings are `text_base` (1rem), where they were
        13px.

      The v0.5.9 unstated-sizes plan
      ([archive](archive/todo_v0.5.9_unstated-sizes-and-chrome-ux-plan.md))
      changed more, again without a look on screen. Sides are top / right /
      bottom / left:
      - **The chrome.** The panel toggles are at the two ends of the status
        bar, which is taller for them (the toggles are `h_6`). The theme
        settings are labelled in the Sidebar's header. The toolbar holds
        Command Palette, Reload Theme and Preferences. The title reads
        `native-theme-gpui <version> showcase`. `NAV_WIDTH` is 200 and the
        window 1380 wide. The Sidebar's icons are `icon_size_small`. The
        Icons page has an Icon Sizes section. The Mode row is a Select
        (System / Light / Dark), like the Theme and Icon set rows; the status
        bar shows the resolved mode. On KDE, `PanelRight` is
        `sidebar-expand-right`. The vertical Separator is no longer shown
        anywhere: widget coverage counts types, not variants. *Superseded
        by the showcase-layout plan below:* the Sidebar, its header, the
        rail, the inspector panel and the second toggle are gone.
      - **Windows:** popover 15/16/17/16; dialog 24; card and group box 12;
        list rows 0/12 (were 4/12); menu rows 4/11/5/11 (were 8/11/8/11);
        tooltip 6/9/8/9; button top 5 (gpui and iced); input 5/6/6/10, its
        right side 10 → 6 (gpui and iced); select and combobox 5/–/7/12, the
        right side upstream's; the toolbar gains a 4px left padding.
      - **GNOME:** popover 8; list rows 2/2/2/2 (were 8/12); the toolbar
        loses its 47 minimum, sizes to its content and gains 6 on every side;
        status bar 6/10; dialog 32/24/24/24, and upstream also uses the top
        and bottom as the gaps between its sections and its content; select
        and combobox gain vertical 5.
      - **KDE:** dialog 10; the toolbar loses its 40 minimum and gains 6 on
        every side; status bar 3/14/2/2 (the right is the size grip's
        empty 14px); select and combobox gain vertical 6;
        the live reader's button 5 → 6.
      - **macOS:** dialog 20; toolbar 0/8/0/8; select and combobox
        horizontal becomes upstream's, with vertical 3; the live reader's
        button 12 → 8; select and combobox are 26px at scale 1 (upstream's
        `h_8`; the stated 21 cannot shrink it).
      - **Live Windows only:** reader button 12 → 11, input 12/12 → 10/6,
        menu 12 → 11, tooltip 8 → 6/9/8/9, toolbar `item_gap` 4 → 0.
      - **The colour-scheme presets** (ten at 40px, material at 64px): the
        toolbar sizes to its content, and the showcase's own toolbar padding
        applies where nothing is stated. Their dialog and status bar draw
        gpui-component's padding, where they drew 0.
      - **Every preset:** the popover and hover card, which drew 0; the
        input, select and combobox now draw their stated padding.
      - **Control heights:** the stated height at text scale 1 (the Windows
        button was 33 for 32; macOS controls at 96 DPI were 27 for 22),
        growing with the platform's line height above 1.
      - **The Textarea** keeps its own 90px, and the single-line padding is
        cleared from it.

      The v0.5.9 showcase-layout plan
      ([archive](archive/todo_v0.5.9_showcase-layout.md)) changed the
      window's shape, again without a look on screen:
      - **The frame.** The window asks for server-side decorations. On KDE,
        KWin grants them, so the title bar, the window controls, the
        corners and the shadow are KWin's decoration (Breeze by default),
        and the menus sit in a menu-bar row above the toolbar. The window
        draws gpui-component's `TitleBar` only where a compositor refuses
        (GNOME's Mutter); under KWin the Layout page shows one as a sample.
        The screenshot scripts capture the active window with
        `spectacle -a -b -n`; check that the captures hold KWin's frame, as
        whether Spectacle includes a window's decoration is its own setting.
      - **The layout.** Two panels: the side panel (`LEFT_PANEL_WIDTH`
        300) holds the Theme, Mode and Icon theme rows, a Separator and the
        inspector; the content panel has a TabBar of the pages above the
        page. The window is 1180 × 850. The status bar has one toggle, at
        its left end. The Layout page shows an expanded and a collapsed
        `Sidebar`.
- [ ] **The iced showcase: per-instance Widget Info.** The gpui showcase now
      builds every widget through a helper that attaches a `WidgetInfo` and
      shows the innermost hovered instance's info in an inspector (the v0.5.9
      showcase-app work, [spec](archive/todo_v0.5.9_showcase-app-spec.md)
      §3–§5); the iced showcase still builds one info string per demo block
      (`widget_tooltip`, `showcase-iced.rs`), with three of the four sections
      `WidgetInfo::to_text` writes and no citation per colour. Rationale D13
      put it out of scope and filed it for later. iced has no chrome widgets
      to promote, so this is the info half only: decide how an iced
      showcase can tell which instance is innermost under the pointer, and
      whether the citation gates in `src/showcase.rs` can be shared or need
      an iced twin.
- [ ] **The iced connector reads nothing of the model's toolbar.** The gpui
      connector gained `geometry::toolbar` in v0.5.9, which carries
      `toolbar.bar_height`, `item_gap`, the `toolbar.border` padding,
      `background_color` and the `toolbar.font` size and weight
      (`connectors/native-theme-gpui/src/geometry.rs`, `toolbar`). The iced
      connector has no counterpart: nothing under
      `connectors/native-theme-iced/src/` reads `resolved.toolbar` (the one
      `toolbar` there is `defaults.icon_sizes.toolbar`, in a test of
      `icon_sizes`), and the iced showcase draws no toolbar. Parity means an
      iced reader of the same fields, and a toolbar row in the iced showcase
      to show it. Found in the final review of the v0.5.9 showcase-app work
      (2026-09-23).

- [ ] **Follow-up plan: the other sizing fields a platform leaves
      unstated.** The v0.5.9 unstated-sizes plan
      ([rationale](archive/todo_v0.5.9_unstated-sizes-and-chrome-ux-rationale.md)
      §7) made padding, `toolbar.bar_height` and `toolbar.item_gap` follow
      platform-facts, with a gate. Every other sizing field still carries
      numbers where platform-facts states **(none)**, a range, a preset's own
      choice, or no number at all. Each needs its own connector decision,
      e.g. what a builder does without a minimum height or a dialog bound,
      and `bar_height`'s change to `Option` is the pattern. The plan's audit
      (Table B, 2026-09-23) found these; the values are unchanged since, and
      are the same in each `-live` twin:

      | Field | Platform-facts cell | Stated by |
      |---|---|---|
      | `button.min_width` | macOS, Windows (none); GNOME "none" (§2.3) | macos-sonoma, windows-11, adwaita 64 |
      | `button.min_height` | KDE (none), sizes to content (§2.3) | kde-breeze 32. Also Windows: platform-facts "27 (derived)" against windows-11 32 and the reader's 32, a value mismatch |
      | `input.min_height` | KDE (none) (§2.4) | kde-breeze 32 |
      | `menu.row_height` | KDE (none), sizes to font (§2.6) | kde-breeze 28 |
      | `menu.row_height` | Windows per context: touch 31, mouse 23 (§2.6) | windows-11 36, which matches neither. The reader reads `SM_CYMENU` (its testable build states 32), which Ch. 1 of platform-facts calls the menu *bar* height, not an item's |
      | `tooltip.max_width` | KDE "(none) — preset: 300"; GNOME "(none) — preset: 360" (§2.7) | kde-breeze 300, adwaita 360 |
      | `scrollbar.groove_width` | GNOME "slider: 8 + margins", no total; macOS per context, legacy 16 / overlay 7 (§2.8) | adwaita 12, macos-sonoma 16. The macOS reader states 15, which matches neither context |
      | `scrollbar.thumb_width` | macOS per context; Windows "↕ `SM_CXVSCROLL` (same)" (§2.8) | macos-sonoma 7 and the reader's 7; windows-11 6 against the API value |
      | `scrollbar.min_thumb_length` | Windows `SM_CYVTHUMB` (§2.8) | windows-11 17; the reader's testable build states 40 |
      | `slider.tick_mark_length` | GNOME (none), no ticks (§2.9) | adwaita 4 |
      | `progress_bar.min_width` | macOS, Windows, KDE (none) (§2.10) | macos-sonoma 100, windows-11 100, kde-breeze 6 (the KDE reader's comment says "Preset provides the value") |
      | `tab.min_width` | macOS, Windows (none); GNOME "none" (§2.11) | macos-sonoma, windows-11, adwaita 64 |
      | `list.row_height` | KDE (none); GNOME per context, rich list 32 / plain list none (§2.15) | kde-breeze 28; adwaita 34, which matches neither context |
      | `splitter.divider_width` | GNOME per context, 1 / 5 (§2.17) | adwaita 1. Value mismatches: the macOS reader's 9 against platform-facts' and the preset's 6; the Windows reader's 4 against platform-facts' and the preset's 1 |
      | `layout.widget_gap` | Windows (none) (§2.20) | windows-11 6 |
      | `layout.container_margin` | macOS, Windows (none) (§2.20) | macos-sonoma 8, windows-11 6 |
      | `layout.window_margin` | Windows (none) (§2.20) | windows-11 10 |
      | `layout.section_gap` | Windows, KDE (none) (§2.20) | windows-11 18, kde-breeze 18 |
      | `dialog.min_width` / `max_width` / `min_height` / `max_height` | macOS and KDE all (none); GNOME min and max height (none), max width per context 372 / wide 600 (§2.22) | macos-sonoma and kde-breeze 320/560/140/600; adwaita min height 140, max height 600 |
      | `dialog.icon_size` | Windows, KDE, GNOME (none) (§2.22) | windows-11, kde-breeze, adwaita 32 |
      | `spinner.min_diameter` | KDE, GNOME (none) (§2.23) | kde-breeze, adwaita 16 |
      | `spinner.stroke_width` | macOS, KDE, GNOME (none) (§2.23) | macos-sonoma, kde-breeze, adwaita 2 |
      | `combo_box.min_height` | KDE (none); GNOME a derivation, "← button min-height (24+pad)" (§2.24) | kde-breeze 32, adwaita 34 |
      | `combo_box.min_width` | macOS, KDE, GNOME (none) (§2.24) | macos-sonoma, kde-breeze, adwaita 120 |
      | `combo_box.arrow_icon_size` | macOS range ~16–18 (§2.24) | macos-sonoma 17 |
      | `combo_box.arrow_area_width` | macOS range ~16–18; GNOME (none) (§2.24) | macos-sonoma 17, adwaita 28 |
      | `segmented_control.segment_height` | Windows, GNOME (none); KDE the tab bar as proxy (§2.25) | windows-11 28, adwaita 28, kde-breeze 30 |
      | `segmented_control.separator_width` | Windows, GNOME (none) (§2.25) | windows-11 1, adwaita 1 |
      | `expander.header_height` | macOS, KDE (none) (§2.27) | macos-sonoma 40, kde-breeze 40 |
      | `defaults.icon_sizes.small` | macOS range, "sidebar: 16–20pt" (§2.1.8) | macos-sonoma 16 |
      | `defaults.icon_sizes.large` | macOS (none) (§2.1.8) | macos-sonoma 32 |
      | `defaults.icon_sizes.dialog` | macOS, Windows (none); GNOME "(none) — 48 (GTK3 legacy)" (§2.1.8) | macos-sonoma, windows-11, adwaita 22 |
      | `defaults.icon_sizes.panel` | macOS, Windows, GNOME (none) (§2.1.8) | macos-sonoma, windows-11, adwaita 20 |
      | `defaults.icon_sizes.toolbar` | macOS per context, "32pt (reg) / 24 (sm)" (§2.1.8) | macos-sonoma 24, the small mode rather than the regular default |

      The Icons page's Icon Sizes section already marks the unsourced icon
      sizes (panel 20, dialog 22, macOS large 32). Outside sizing, KDE's
      `defaults.border.corner_radius_lg` is "(none) — preset" (§2.1.6) while
      kde-breeze states 8. Two value mismatches the audit noticed and did
      not follow: adwaita's `checkbox.indicator_width` 20 against §2.5's
      "libadwaita CSS: 14" (Ch. 1 gives 20 with padding), and windows-11's
      menu `row_height` 36, which no platform-facts context gives. Also for
      this plan:
      - **KDE input vertical padding** (the audit's N4): platform-facts'
        §2.4 cell is "3 (measured)", and the gate keeps 3/6/3/6. Breeze's
        `lineEditSizeFromContents` expands by `LineEdit_FrameWidth` = 6 on
        both axes, and QLineEdit's own margins were not read.
      - **iOS.** Platform-facts has no iOS column, so no iOS size is
        sourced: `ios.toml`'s `bar_height_px = 44.0` is unsourced, and so is
        every other size in it (the 44 `min_height` of button, input and
        combo box, the 44 `row_height` of menu and list, tooltip
        `max_width` 300, the dialog bounds 270/560/140/600, the layout
        8/8/20/20). The gate does not cover iOS.
- [ ] **The colour-scheme presets' sizes have no source.** The eleven
      colour-scheme presets (the four catppuccin, dracula, gruvbox,
      material, nord, one-dark, solarized, tokyo-night) carry sizes that
      were copied rather than sourced: their toolbar's 40 was the height
      the removed generic `default.toml` carried, and material's header
      cites Material 3 for its colours only. v0.5.9 removed only their
      `bar_height_px` (40, material 64). Decide, field by field, what a
      colour-scheme preset's size is: a value with a source, or absent, so
      that the toolkit's default or the application's stands.
- [ ] **The showcase's pages: the application's own values, as the chrome
      has them.** The unstated-sizes spec §3.1 rule — a named showcase
      constant only where the theme states nothing, for an element the
      showcase draws itself, with its comment and its info saying so — is
      applied to the toolbar row and the Sidebar header only. The pages
      still use `with_gap` (`support.rs`), which leaves a gap absent where
      `layout.widget_gap` is unstated, so gpui's own default gap
      applies. Every bundled preset states `widget_gap` today, but windows-11's
      6 is itself unsourced (platform-facts §2.20 says (none), see the
      follow-up plan above), so the question becomes live when that plan
      removes it.
- [ ] **A machine-readable platform-facts.** The documented-sizes gate
      (`native-theme/src/presets/documented_sizes.rs`) copies each value it
      checks by hand from `docs/platform-facts.md` and cites the line; a
      companion test checks the citation lands on the right row, but not
      that the number is the one written there. With the facts in a
      structured form (per platform, per field, the value, its context and
      its source), every native value — not only padding and the toolbar —
      could be gated, and the Markdown tables generated from it.
- [x] **macOS: the Sidebar icon runs into its rail item's padding.** Under
      macos-sonoma at 96 DPI (rem 17.33px) the icon rail's items are 30px
      wide, and gpui-component's `p_2` on every side (sidebar/menu.rs:284)
      leaves about 12.7px, so the 16px icon runs about 1.7px into the
      padding on each side. It still lies inside its item. The padding is
      upstream's; measured in v0.5.9 Task 5. *Re-measured at 72 DPI*, the
      DPI macOS resolves at and the only one the showcase shows macos-sonoma
      at (it is offered on macOS alone): rem 13px, items 34px wide, `p_2`
      6.5px, so 21px remains and the 16px icon fits inside the padding. The
      96 DPI case does not arise.
- [ ] **WinUI's combobox arrow column: a model extension.** Per-side padding
      is done (v0.5.9). WinUI measures its combobox's right padding, 0, to
      a separate 38px arrow column (platform-facts §2.24), a structure the
      model has no field for, so windows-11 leaves the right side unstated
      and gpui's trigger keeps upstream's. An arrow-column field (or a
      right padding that includes it) would let a connector with such a
      column draw it.
- [ ] **Loose ends of the v0.5.9 unstated-sizes plan** (from its reviews):
      - The seams test `input_select_and_combobox_draw_the_stated_padding`
        measures the left side only: top and bottom are invisible under
        `items_center` at a fixed height, and the right side is the suffix
        and caret exception. No test measures the Textarea's drawn inset.
        *Since the final review:* `an_input_without_a_suffix_draws_the_stated_right_padding`
        measures an Input's right side, and `a_textarea_draws_no_padding`
        the Textarea's inset, each as the size the padding adds, since
        neither side has a child to measure it by.
      - The NavItem's icon expression could not be extracted whole into a
        `pub(crate)` helper: `every_widget_reports_itself` has no exemption
        for a part whose caller reports it. The test checks the helper
        NavItem calls (`nav_icon_sized`), and the rail check measures the
        result, so a NavItem that bypassed the helper would be caught by the
        rail check alone.
      - Under the `default` theme, the Icon Sizes cells carry no per-cell
        caveat for unsourced sizes: the showcase keeps `default_label`, not
        the system preset's key. The section's general line still applies.
        *Fixed since the final review:* the showcase keeps the key too
        (`default_preset`), and the cells take the caveat of the preset
        `default` is built on.
      - The status bar carries two debug-selector probes, an
        environment-text wrapper and an empty middle box, so the tests can
        measure its content. Task 6's new status-bar tests allow one device
        pixel, half a logical pixel at the test window's scale factor of 2
        (gpui-pre platform/test/window.rs:129).

#### Upstream PR to gpui

- [ ] PR: add `Window::screenshot()` API to gpui — gpui has no public way to
      capture the rendered framebuffer. The underlying blade-graphics backend
      has `copy_texture_to_buffer()` but gpui doesn't expose it. A public
      `screenshot()` method would enable headless CI screenshot capture on all
      platforms (like iced's `--screenshot` flag). Without this, gpui showcase
      screenshots are Linux-only (via external spectacle capture).
- [ ] PR: let an application drop the SVGs a window has drawn, so the icons
      gpui-component's widgets build for themselves can follow a theme
      switch. Measured in the v0.5.9 pre-merge fixes (Task 6's spike,
      2026-09-24, gpui-pre 0.3.6, gpui-component 0.6.6): gpui-component
      names those icons as asset paths (`icon.rs:30-33`, `:189`;
      `checkbox.rs:211` sets the path on an `svg()` directly), so an
      application's `AssetSource` could answer `icons/<name>.svg` from the
      chosen icon theme. It cannot make a switch show: `Window::paint_svg`
      keys the window's sprite atlas by `RenderSvgParams { path, size }`
      (`window.rs:4823-4833`, `svg_renderer.rs:85-88`), and the atlas
      returns an occupied key without calling the build closure
      (`platform.rs:1483-1490`, `AtlasState::get_or_insert_with`), which is
      the only place `SvgRenderer::render_alpha_mask` asks the asset source
      (`svg_renderer.rs:257`). The atlas is a private field of `Window`
      (`window.rs:1165`); `Window::drop_image` removes only image keys
      (`window.rs:4996-5007`), and only gpui itself clears an atlas, as its
      renderers recover from GPU errors and a lost device (gpui-pre-wgpu
      `wgpu_renderer.rs:1300`, `:2120`; gpui-pre-windows
      `directx_atlas.rs:61`). A test against the `HeadlessAtlas` the test
      platform uses built a path at a size once for two paints. The path
      cannot vary per theme either: each call site builds a fixed
      `IconName`. So after a switch every path and size already drawn keeps
      the old theme's icon while any new size gets the new one -- two icon
      themes in one window. What would do it: a public
      `Window::drop_svg(path)` (or a generation in `RenderSvgParams`) that
      removes the path's tiles at every size, which an application calls
      for `icons/*` after it changes what its asset source answers. The
      rendering side is not the obstacle: an SVG is drawn as an alpha mask
      in the element's text colour (`svg_renderer.rs:231-261`), and
      Breeze's and Adwaita's symbolic icons rasterise to non-empty masks
      through `SvgRenderer` (measured: 318 and 678 covered pixels of 4096).
      Until then the gpui showcase's page samples and chrome draw the chosen
      theme's icons where the showcase builds the icon, and each widget's
      Widget Info says which of its icons are gpui-component's own
      (`info::own_icons`); the icon-provider hook in "Upstream PRs to
      gpui-component" below is the other way in.

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
- [ ] Report: a single-select `Combobox` misses a change after a search.
      It decides that the selection changed by comparing the selection's
      `IndexPath`s before and after `on_will_change`
      (gpui-component 0.6.6 `src/combobox.rs:183-196`, and `:459-468` in
      `handle_item_select`), and the default `on_will_change` stores the
      row in the *filtered* list. Choosing, after a search, an item that
      lands in the row the current selection has — typing "nord" and
      pressing Enter while `default`, also row 0, is selected — emits no
      `Change` and no `Confirm` and leaves the popup open. The gpui
      showcase works around it in `PresetDelegate::on_will_change`
      (`examples/showcase-gpui/support.rs`), which records each chosen preset at
      its row in the unfiltered list. Upstream could compare by value, or
      store unfiltered indices. Found 2026-09-22 (showcase-app plan, Task 9).
- [ ] PR: let a consumer supply the icons gpui-component's widgets build for
      themselves. Several widgets name a Lucide `IconName` as they render,
      with no setter, so an application that draws its own icons from
      another set -- the gpui showcase's chrome follows the icon-set Select
      and draws no icon where the chosen set has none -- still shows
      gpui-component's Lucide in them, mixing two sets. Read in
      gpui-component 0.6.6: `SidebarToggleButton`'s PanelLeftClose /
      PanelLeftOpen (`src/sidebar/mod.rs:351-359`), the `Caret` chevron of a
      `Select`, `Combobox` or dropdown `Button` (`src/select.rs:59`), the
      check mark of a chosen list row (`src/searchable_list/item.rs:42`,
      `src/searchable_list/adapter.rs:130`), an empty list's Inbox
      (`src/select.rs:279`, `src/combobox.rs:261`), the window controls
      (`src/title_bar.rs:148-151`), the close button of a `Dialog`
      (`src/dialog/dialog.rs:698`) and of a `Sheet` (`src/sheet.rs:204`),
      the `Command` palette's search icon and check mark
      (`src/command/state.rs:851`, `:767`), a `PopupMenu`'s check mark,
      external-link and submenu chevron (`src/menu/popup_menu.rs:1167`,
      `:1209`, `:1334`, `:1373`), the `Settings` search field's icon
      (`src/setting/settings.rs:152`) and a `NumberInput`'s minus and plus
      (`src/input/number_input.rs:155`, `:185`). One way in is an optional
      icon-provider hook on the theme that these call sites ask first,
      falling back to today's `IconName`. The showcase's icon-set Select
      names the ones its chrome shows (`info::chrome::icon_set_select`).
      Found in the final review of the v0.5.9 showcase-app work
      (2026-09-23).
- [ ] Left open by the v0.5.9 showcase-layout work
      ([archive](archive/todo_v0.5.9_showcase-layout.md)):
      - `info::title_bar` (`examples/showcase-gpui/info/chrome.rs`) keeps
        Windows and macOS branches for its colours and its "window
        controls" note, but the chrome `TitleBar` is drawn only where
        client-side decorations are granted, and only the Linux backends
        ever report that. Drop the branches, or say why they stay.
      - What Windows and macOS grant was read from gpui-pre 0.3.6's source,
        not run: check there that the window keeps its native title bar and
        that the menus sit in the row (Windows) or the system's menu bar
        (macOS).
      - The menu-bar row has no fill or font of its own: the model has no
        menu-bar section (`menu` in platform-facts §2.6 is the popup).
      - `the_page_tabs_navigate` checks that a click shows the page, not
        which tab is drawn selected.
      - The item above on the resizable group's missing Widget Info names
        three `resizable_panel()`s; there are two now.

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
