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
      workspace at `1.88.0` (re-measured 2026-09-06, unchanged), the gpui
      connector separately at `1.95.0` (measured 2026-09-06: gpui-pre 0.3.3
      uses `cold_path`, stable since 1.95) and the egui connector at `1.95`.

### native-theme-gpui connector

- [x] Map `WidgetMetrics` → gpui-component per-widget styling — done in
      v0.5.8 for every widget with a reachable seam (`geometry` module,
      `base_layer`); the inner-element remainder is the upstream PR list
      below (v0.5.8 spec §14).

#### Upstream PR candidates from v0.5.8 (Tier U)

- [ ] a styled `Theme` scrollbar-style override honoured by `base_theme()`
      (would make the connector's re-apply observer unnecessary)
- [ ] `Tab` applying its stored `Styled` refinement
- [ ] `Theme.shadow` honoured beyond `Button`; `tokens.shadow` consumed
- [ ] `Size::Size` honoured by `Checkbox` and `Switch`
- [ ] inner geometry exposed: checkbox/radio indicator, switch track and thumb,
      slider track and thumb, separator thickness, resize-handle width, button
      icon gap, input padding, popup-menu items, select arrow, accordion arrow
- [ ] a `PopupMenu` item style hook; a `Button::tooltip` style hook
- [ ] button label text size independent of rem
- [ ] an iterable `IconName::ALL` generated by `icon_named!`

#### Research

- [ ] Scrollbar thumb radius per platform: platform-facts records none, so the
      connector mirrors gpui-component's `radius` for the thumb.

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
