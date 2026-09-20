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

#### Research

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
