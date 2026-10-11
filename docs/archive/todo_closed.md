# native-theme: closed backlog items

The items of [`docs/todo.md`](../todo.md) that were done, moved here on 2026-10-11
(M1-T10-01 of the code review) so that the backlog lists open work only. Each is copied
verbatim with its decision text. A `###` section whose every item was done moved whole,
under its `##` heading; a done item from a section that still has open items is under
"Closed items from" and the name of the heading it sat under. The order is the order
they had in `docs/todo.md`. Their `file:line` citations were correct when written and
many have since drifted, as the archive's [README](README.md) warns.

---

## Core API

### `SystemTheme` — expose layout metrics

- [x] Add `pub layout: LayoutTheme` to `SystemTheme` (done in v0.5.8). Approved 2026-08-10; see
      Q-2 in `docs/archive/todo_v0.6.0_egui-connector-rationale.md` §8. Before it, `from_preset` could supply `Spacing::item_spacing` (from
      `layout.widget_gap`) but `from_system` could not, because `SystemTheme`
      had no `layout` field — so on the system path that spacing fell back to
      the toolkit's default. `Spacing::window_margin` never depended on it: the
      egui connector writes it from `window.border.padding`, which every path
      carries, and gives `layout.window_margin` to the central panel's frame
      (`Surface::CentralPanel`, spec §5.1).
      One additive field on a struct that already carries `preset` and
      `icon_theme`; no resolver work, since `Theme::layout` is a plain
      `LayoutTheme` shared across the light and dark variants and all four of
      its fields are `Option<f32>`, so an absent layout costs nothing.
      Benefits the egui, iced and gpui connectors equally.

### Checkbox: KDE's checked checkbox is not a solid accent box

- [x] Re-research KDE's checked checkbox in `docs/platform-facts.md` §2.5.
      Breeze (its source at tree `be6e137e`) paints the checked box as the unchecked one — a rounded rectangle
      filled with the button colour, `palette.button()` — then fills it again
      with the highlight colour made translucent at `highlightBackgroundAlpha`
      (`Metrics::Blend_Value`), outlines it in the highlight colour, and draws
      the check mark in the text colour, `palette.text()`
      ([breezehelper.cpp:847-848](https://invent.kde.org/plasma/breeze/-/blob/be6e137e169e246e67e9a9958d5a4f97bfa69e2e/kstyle/breezehelper.cpp#L847-848),
      [:853-854](https://invent.kde.org/plasma/breeze/-/blob/be6e137e169e246e67e9a9958d5a4f97bfa69e2e/kstyle/breezehelper.cpp#L853-854),
      [:862](https://invent.kde.org/plasma/breeze/-/blob/be6e137e169e246e67e9a9958d5a4f97bfa69e2e/kstyle/breezehelper.cpp#L862),
      [:869-873](https://invent.kde.org/plasma/breeze/-/blob/be6e137e169e246e67e9a9958d5a4f97bfa69e2e/kstyle/breezehelper.cpp#L869-873),
      [:938](https://invent.kde.org/plasma/breeze/-/blob/be6e137e169e246e67e9a9958d5a4f97bfa69e2e/kstyle/breezehelper.cpp#L938); the constant at
      [:40](https://invent.kde.org/plasma/breeze/-/blob/be6e137e169e246e67e9a9958d5a4f97bfa69e2e/kstyle/breezehelper.cpp#L40)).
      §2.5 instead gives KDE `checked_background` ← `defaults.accent_color` and
      `indicator_color` = `[Colors:Selection] ForegroundNormal`
      (`docs/platform-facts.md:1211`, `:1218`), and `kde-breeze` states
      neither, so it shows a solid accent box with an accent-text mark
      (`docs/inheritance-rules.toml:174-175`). Still to research: the value of
      `Metrics::Blend_Value` (`breezemetrics.h`, not read), which Breeze release
      shipped this code, and whether the model states Breeze's box as one
      composited `checked_background` or needs a layer of its own.

      Done 2026-09-28, from breeze v6.7.5 (the release Plasma 6.7 ships):
      `Metrics::Blend_Value` is 0.3
      ([breezemetrics.h:176](https://invent.kde.org/plasma/breeze/-/blob/v6.7.5/kstyle/breezemetrics.h#L176)),
      which `highlightBackgroundAlpha` reads
      ([breezehelper.cpp:40](https://invent.kde.org/plasma/breeze/-/blob/v6.7.5/kstyle/breezehelper.cpp#L40));
      breeze v6.4.0 had a literal 0.33 there
      ([breezehelper.cpp:39](https://invent.kde.org/plasma/breeze/-/blob/v6.4.0/kstyle/breezehelper.cpp#L39)).
      The radio button is drawn the same way (`breezehelper.cpp:995-1024`, its
      dot in `palette.text()` at `:1059`). The model states the box as one
      composited `checked_background` — the button colour under the selection
      colour at alpha 0.3, Breeze #c2e4f6, Breeze Dark #2f5368 — with
      `indicator_color` the View foreground, the checked outline
      (`checkbox.border.color`) the selection colour and
      `unchecked_border_color` Breeze's `separatorColor()`
      (`docs/platform-facts.md:1209`, `:1211`, `:1218`). The kde-breeze preset
      states them, and the KDE reader computes them from `kdeglobals`
      (`native-theme/src/kde/colors.rs`). Not settled: `hover_background`
      (`#93cee9` in the preset, with no source): Breeze draws a hovered
      indicator's outline in the focus colour (`breezehelper.cpp:897-910`,
      `:1043-1057`) and leaves its fill alone, and the model has no hover
      outline colour for the checkbox.

### Closed items from Theme watcher: OS changes it does not report

- [x] **GNOME: dropping the watcher blocks until the next portal signal.**
      The GNOME thread waits inside the blocking `SettingChanged` signal
      iterator and checks the shutdown channel only after a signal arrives
      (`native-theme/src/watch/gnome.rs:52-62`); the subscription is created
      with no platform shutdown to wake it (`:65`), unlike the macOS and
      Windows backends, and `ThemeSubscription`'s `Drop` joins the thread
      (`native-theme/src/watch/mod.rs:183-185`). So dropping a GNOME or Budgie
      watcher — at the latest when the application exits — returns only when
      the next `org.freedesktop.appearance` change arrives. Register a
      platform shutdown that wakes the thread (e.g. closes the D-Bus
      connection's signal stream) so `Drop` returns at once; the egui
      connector's plan, `docs/archive/todo_v0.6.0_egui-connector-plan.md`, does this,
      with a test that a started watcher drops within a bound.

### Closed items from Fonts: find a system face by family, weight and style

- [x] Add the feature `system-fonts` to `native-theme`: an optional `fontdb`
      0.23 dependency (0.23.0 is already in `Cargo.lock`, through cosmic-text)
      with `default-features = false` and the features `fs`, `fontconfig` and
      `memmap`, fontdb's own default set (fontdb 0.23.0 `Cargo.toml`, lines
      63–68) —
      `memmap` because without it loading the system fonts reads every font
      file in full — loaded once per process into a crate-private
      `std::sync::OnceLock` that every call shares, and one public
      function, `native_theme::fonts::system_face(family: &str, weight: u16,
      style: native_theme::theme::FontStyle) -> Option<native_theme::fonts::SystemFace>`,
      where a `SystemFace` carries the face's bytes (`std::sync::Arc<[u8]>`),
      its index in a collection file, and the family (the first of
      `FaceInfo::families`), weight and style fontdb records for the face.
      It compares the family name case-insensitively, by Unicode default
      caseless matching as CSS Fonts Level 4 §5.1 requires (`unicase` 2.9,
      already in `Cargo.lock`), and never returns a face of another family;
      among that family's faces it picks the width, then the style, then the
      weight by the CSS Fonts Level 4 font-matching algorithm (§5.2). That selection is a
      pure function over the candidate faces, exported without the feature
      because it needs no fontdb, so `system_face` and the egui connector's
      `FontPlan` match fonts through the one implementation. egui has no font
      database and needs a font's bytes, so the egui connector's
      `FontPlan::from_system` builds on it for `defaults.font` and
      `defaults.mono_font` (`docs/archive/todo_v0.6.0_egui-connector-spec.md` §4.9,
      §8), and its own `system-fonts` feature, on by default, turns this one
      on. On macOS the system UI font is found by its file instead of its
      name, because fontdb holds it as `.SF NS` (see *macOS: the stated font
      family "SF Pro" is not a family iced's font database holds*, below): a
      `family` equal to "SF Pro" or to Core Text's family name for the system
      font is resolved by asking Core Text for that font
      (`CTFontCreateUIFontForLanguage`) and its file (`kCTFontURLAttribute`),
      through `objc2-core-text` 0.3.2 in the `macos` module, which already
      permits FFI `unsafe` (connector spec §8.2). Unverified until
      run: that the other names the macOS and Windows readers report are names
      fontdb records, and that the Core Text route finds the system font; the
      connector's `system_faces_resolve` runs on the macOS and Windows CI
      runners over the reader's theme and the platform's preset, in the runner
      task of `docs/archive/todo_v0.6.0_egui-connector-plan.md`.
      The iced and gpui connectors build on it in that plan's Tasks 4 and 5.

### `SystemTheme` — the icon theme of both variants

- [x] `SystemTheme::icon_theme` is the active variant's icon-theme name only
      (`native-theme/src/lib.rs:470-483`), yet the variants can name different
      ones (`kde-breeze`: `breeze` and `breeze-dark`,
      `native-theme/src/presets/kde-breeze.toml:9`, `:317`), and a toolkit that
      keeps a style per scheme, as egui does, needs both. Add
      `SystemTheme::icon_theme_for(mode: ColorMode) -> Option<&str>`, the other
      variant's name resolved by the same three tiers
      (`native-theme/src/pipeline.rs:82-93`) and carried through `with_overlay`
      — which the *with_overlay* fix below covers. The egui connector's
      `ThemeAtlas::icon_theme` needs it for the scheme that is not active.

### platform-facts: whether a progress bar draws a border

- [x] `docs/platform-facts.md` §2.10 states no `border.color` or
      `border.line_width` for the progress bar, so both are `defaults.border`'s
      by inheritance (`docs/inheritance-rules.toml:86-93`), and a connector
      stroking them would outline the bar on every preset. The platforms
      differ. KDE's Breeze strokes the groove with a 1.001 px pen of the
      window text colour at its frame intensity, which over the window is its
      frame outline colour (breeze master, `kstyle/breezehelper.cpp` lines
      134–139, 144 and 1204–1219, `kstyle/breezestyle.cpp` lines 6302–6303,
      `kstyle/breezemetrics.h` line 25). WinUI 3's
      `ProgressBarBorderThemeThickness` is 0, and 1 in high contrast
      (microsoft-ui-xaml `258a2e9b`,
      `controls/dev/ProgressBar/ProgressBar_themeresources.xaml` lines 5, 13
      and 21). libadwaita draws none, and an inset 1 px `box-shadow` under
      `prefers-contrast: more` (libadwaita 1.10.0 `_scale.scss:1-10`, which
      `_progress-bar.scss`'s `> trough` extends). macOS's
      `NSProgressIndicator` is unread: a screenshot on `macos-latest` settles
      it. Add the two rows to §2.10 with those sources, and state
      `line_width_px = 0` in the presets of the platforms that draw none.
      Then the egui connector's `Frame::stroke` route is exact on every
      preset, and the decline in its spec's §5.8 item 8 can go
      (`docs/archive/todo_v0.6.0_egui-connector-spec.md` §5.8).
      Done 2026-09-28: §2.10 has the `border.color` and `border.line_width`
      rows (KDE: `WindowText` at alpha 0.2, `#23262933` / `#fcfcfc33`, 1px,
      from Breeze f0b1d75; GNOME and Windows 0; macOS still unread). kde-breeze
      states the colour and the width, adwaita and windows-11 a width of 0,
      and the KDE reader derives the colour from `kdeglobals`. gpui's
      `widgets::ProgressBar` and iced's `styles::progress_bar` stroke what is
      stated; egui draws it through the companion crate's
      `progress_bar::ProgressBar` (egui's `ProgressBar::ui` strokes nothing,
      `progress_bar.rs:130-204`), and the egui ledger grades the two leaves
      `widgets-crate`.

### Border opacity: which lines `defaults.border.opacity` fades

- [x] Decision for the maintainer (found 2026-09-28, while comparing the
      three showcases' Basic pages). The connectors disagree on where
      `defaults.border.opacity` applies. egui folds it into the strokes of
      `defaults.border.color` (the base style's non-interactive and window
      strokes, and `native_theme_egui::border_color`) and into no widget's own
      border colour (`connectors/native-theme-egui/mapping.toml`,
      `["defaults.border.opacity"]`); gpui and iced fold it into nothing, and
      paint every border colour, `defaults.border.color` included, as stated
      (the `border` token, `native_theme_iced::border_color`). So lines drawn
      in `defaults.border.color` are faded in egui and not in gpui or iced:
      kde-breeze states 0.2 (`kde-breeze.toml:44`), which leaves `#bcc0bf`
      faint on `#eff0f1`. The data does not settle it:
      `native-theme/src/model/border.rs:36` says "Border alpha multiplier
      0.0-1.0 (defaults only)", which is where the field is stored, not what
      it applies to (the same note sits on `corner_radius_lg`, `:30`, which
      `docs/platform-facts.md:944` gives to popover, window and dialog
      containers); `docs/platform-facts.md:946` and `:997` say only "applied
      to the border color"; `:1116` gives every platform's value as
      **(preset)** (0.2, 0.14, 0.2, 0.15), with no platform source. The
      archived egui spec folded it into every widget border stroke
      (`docs/archive/todo_v0.6.0_egui-connector-spec.md:2682`, §6.13), and
      the older iced design note into text input, checkbox and pick list
      borders (`docs/todo_iced-full-theme-geometry.md:107`); at 0.2 that makes
      kde-breeze's button border `#d1d1d2` nearly invisible where Breeze draws
      it clearly. Decide: does the multiplier apply to lines drawn in
      `defaults.border.color` (then gpui and iced apply it, and every
      gpui-component frame drawn with the `border` token — dialog, tab,
      accordion, sheet, menu separator — fades with it), to every border
      colour, or to none (then egui stops folding it, and the presets'
      unsourced values could go)? Record the rule where the model documents
      the field, and make the three connectors and their tests follow it.
      Resolved 2026-09-28, from the sources: none. The opacity is the share
      of the text colour that makes a line, already folded into every
      stated border colour, which is the final line colour. KDE's 0.2 is
      `frameIntensityBias()` = `KColorScheme::frameContrast()`, in
      `mix(Window, WindowText, 0.2)` for frames and separators
      ([breezehelper.cpp:142-144](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L142-L144),
      [kcolorscheme.cpp:529-538](https://invent.kde.org/frameworks/kcolorscheme/-/blob/27066d471c93629efee8459d9f69490caa92b7c4/src/kcolorscheme.cpp#L529-L538));
      libadwaita's 0.15 is `--border-opacity` in
      `$border_color: color-mix(in srgb, currentColor var(--border-opacity), transparent)`
      ([_colors.scss:257-264](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/5789add99c79cee0fae624b56706c7c0bea7fb2b/src/stylesheet/_colors.scss#L257-L264));
      WinUI has no multiplier, each stroke role being a colour with its own
      alpha ([Common_themeresources_any.xaml:243-257](https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/Common_themeresources_any.xaml#L243-L257)).
      The rule is in `DefaultsBorderSpec::opacity`
      (`native-theme/src/model/border.rs`) and `docs/platform-facts.md`
      §2.1.6: no connector multiplies any colour by it. egui stopped folding
      it (its `mapping.toml` row is UNMAPPABLE `source-void`); gpui and iced
      folded it into nothing already. The windows-11 (0.14) and macos-sonoma
      (0.2) values have no source; nothing reads them.

### KDE: the single-line input's padding

- [x] `docs/platform-facts.md:1196-1197` gives KDE's single-line input
      padding as `LineEdit_FrameWidth` = 6 horizontal and 3 vertical
      **(measured)**. The sources give 8 and 7: Breeze's line edit frame is
      6 on every side when the field is tall enough (`lineEditContentsRect`,
      [breezestyle.cpp:2228-2251](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L2228-L2251)),
      and QLineEdit adds its own `horizontalMargin = 2`, `verticalMargin = 1`
      ([qlineedit_p.cpp:35-36](https://github.com/qt/qtbase/blob/ef55f427f2c8b410d34f8a7681020a3000cf6866/src/widgets/widgets/qlineedit_p.cpp#L35-L36)).
      Not changed: measure a Breeze `QLineEdit` in a screenshot (text to the
      frame's outer edge, at 1× scale) to settle which PF measured, then
      correct PF and kde-breeze's `input.border`.
      Done 2026-09-28: a real Breeze QLineEdit (`kdialog --inputbox`, nested
      KWin, scale 1.0, Noto Sans 10pt) is 32px tall with a 1px frame line;
      its text starts 8px from the frame's outer left edge and its 18px line
      box 7px below the outer top edge, as the sources give. The model's
      padding lies inside the border line (platform-facts.md:948-949), so
      KDE's is 7 horizontal and 6 vertical: platform-facts §2.4, kde-breeze
      (and -live), the KDE reader and the documented-sizes gate state it.

### material: the slider thumb is the page's colour

- [x] Data question for the maintainer (found 2026-09-28 on the Basic
      pages). `material-3.toml` states no `slider.thumb_color` (`[light.slider]`
      `:133-140`, `[dark.slider]` `:381-388`), so it resolves through
      `docs/inheritance-rules.toml:197` to `defaults.surface_color`, which
      material states as `#fffbfe` (`:13`), the same as its
      `defaults.background_color` (`:11`); dark `#1c1b1e` (`:261`) on
      `#1c1b1f` (`:259`). A toolkit that paints the thumb in that colour
      without an outline draws it invisible: the iced Basic slider under
      material light shows a gap in the track where the thumb is.
      `docs/platform-facts.md` §2.9 (`:1291`) gives `thumb_color ←
      defaults.surface_color` for macOS, Windows, KDE and GNOME, and has no
      Material column; the preset's source line (`material-3.toml:2`,
      "m3.material.io baseline color scheme") gives no slider handle colour,
      and what a Material 3 slider handle takes was not researched. Decide
      whether material states `slider.thumb_color` from a cited Material 3
      source, or the inherited colour stands.
      Resolved 2026-09-28: the Material 3 enabled slider handle is
      `md.sys.color.primary` — `'handle-color': map.get($deps, 'md-sys-color',
      'primary')` ([material-web tokens/versions/v0_192/_md-comp-slider.scss:52](https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/v0_192/_md-comp-slider.scss#L52);
      the M3 token build's `md.comp.slider.handle.color`,
      [tokens/versions/latest/sass/_md-comp-slider.scss:164-165](https://github.com/material-components/material-web/blob/cbd34a8921915af94d5ef65c2a69eece41d5b4f3/tokens/versions/latest/sass/_md-comp-slider.scss#L164-L165)),
      baseline primary40 `#6750a4` light and primary80 `#d0bcff` dark
      (`_md-sys-color.scss:110`, `:51`; `_md-ref-palette.scss:73`, `:77`).
      `material-3.toml` states `slider.thumb_color` so in both variants.

---

## Toolkit Connectors

### Closed items from native-theme-egui connector

- [x] Implement the connector by its plan,
      `docs/archive/todo_v0.6.0_egui-connector-plan.md` — tasks in execution order,
      each with its gate and commit point — per
      `docs/archive/todo_v0.6.0_egui-connector-spec.md` (rationale:
      `docs/archive/todo_v0.6.0_egui-connector-rationale.md`). Targets egui 0.36.2.
      Archiving the three documents is the plan's last task.
- [x] **The egui showcase: the gpui application, with per-instance Widget
      Info.** Part of v0.6.0, not deferred
      (`docs/archive/todo_v0.6.0_egui-connector-spec.md` §10.4, §13.2; plan Tasks 31,
      34–36). Tick when those tasks pass their gates; the iced showcase's
      entry (*The iced showcase: per-instance Widget Info* below) stays open.
- [x] Add an MSRV CI job (spec §12.4; a task of
      `docs/archive/todo_v0.6.0_egui-connector-plan.md`). The workspace floor of `1.88.0`
      was measured on 2026-08-10, but nothing re-checks it: every CI job
      installs `@stable`, there is no `rust-toolchain.toml`, and
      `scripts/check_release.sh` has no MSRV check. The job is the one spec §12.4
      spells out, three `cargo check --all-features --locked` runs: the
      workspace except the gpui and egui connectors at `1.88.0` (re-measured
      2026-09-25, clean; since 2026-09-07 `native-theme` itself uses
      `slice::as_chunks`, stable since 1.88.0, so the floor cannot drop below
      that), the egui connector at `1.95`, and the gpui connector at `1.95.0`
      after installing the gpui system libraries (re-measured 2026-09-19 on
      the 0.6.4 closure: 1.95.0 builds, 1.94.0 fails on `std::hint::cold_path`
      in gpui-pre 0.3.5's `src/profiler.rs`, lines 473 and 494).
- [x] Cross-target warning hygiene (done 2026-09-24). The 5 Windows and 3
      macOS warnings measured on 2026-09-07 are gone (1dc99f82), and CI's
      Windows and macOS test legs now run `cargo check -p native-theme
      --all-features` with warnings denied (`.github/workflows/ci.yml:83-87`,
      b35432a9), as the cross-target section of `scripts/check_release.sh` does.
      Re-measured 2026-09-25 on rustc 1.98.1: `cargo check -p native-theme`
      for `x86_64-pc-windows-msvc` and `x86_64-apple-darwin`, each with and
      without its platform feature, reports no warning.

### Closed items from native-theme-gpui connector

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

### Closed items from Upstream PR candidates from v0.5.8 (Tier U)

- [x] ~~a determinate `ProgressCircle` has no size receiver the theme can
      feed~~ -- wrong, found 2026-09-22: `ProgressCircle` applies the caller's
      refinement after its per-Size arm (`progress/progress_circle.rs:187-194`),
      so a `.size()` reaches it. What is missing is a model value for a
      determinate circle's diameter, which is ours, not upstream's.
- [x] an iterable `IconName::ALL` generated by `icon_named!` — landed
      upstream in gpui-kit 0.6.1 without our PR: `gpui_kit_assets::IconName`
      covers the full Lucide catalog (1830 variants) with `ALL`, `PartialEq`,
      `Debug` and `Hash`; gpui-component's `IconName` stays a 101-variant
      compatibility enum (same set as 0.6.0) that converts into it via
      `From`. The connector-side follow-up is listed above.
- [x] spec v0.5.9 §2.2's action table lacks `SetPreset(key)`, which the
      command palette's preset entries run (Task 12): add it with the
      palette as its one caller (the toolbar's Combobox installs a preset
      through its own `Change` event). Added to the archived spec's §2.2 as
      an *As built* note (Task 25).

### Closed items from native-theme-iced: the same audit (found 2026-09-20, fixed in v0.5.9)

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

### Closed items from Research

- [x] `checkbox.indicator_color` inherited `defaults.text_color` until v0.5.9,
      against `platform-facts.md` §2.5 (on-accent on all four platforms);
      corrected to `defaults.accent_text_color`. The test that ties
      `docs/inheritance-rules.toml` to the proc-macro attributes
      (`uniform_rules_all_accounted_for`) checks that each key is *present* on
      both sides and never compares the inheritance **target**, so the
      published rule file can drift from the code with no test noticing. Add a
      target-level assertion, then audit the other rules against
      platform-facts the way this one was found.
      Done 2026-10-11 (890edc88): `uniform_rules_all_accounted_for` checks
      the 86 rules' sources, 67 against the derive's recorded `inherit_from`,
      4 against the border inventory and 15 against a hand-kept list.
- [x] The showcase's "Text Input" tooltip claims the input background is
      `background`; `Input` paints `Theme::input_background()`, which equals
      `background` only in light mode (gpui-component 0.6.4
      `src/theme/mod.rs:379-385`). Noticed while adding the InputGroup section
      in v0.5.9; correct the tooltip (and check the sibling widgets' claims).
      **Done in the v0.5.9 showcase-app work:** the Input, Textarea,
      NumberInput, OtpInput, Checkbox, Radio, Select and Combobox infos take
      their fill claim from one mode-aware helper (`info/chrome.rs`,
      `input_background`).

### Closed items from What the audit turned up that is ours to fix

- [x] **A `DataTable` can take the platform's row height today.** `Size::Size(px)`
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
      **Done in v0.6.1:** `geometry::data_table_size`. Since gpui-component
      0.7.1 the same `Size::Size` is also what keeps the cells from `text_sm`
      (`sizing.rs:323-333`).
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
- [x] **A `geometry::toggle`.** A `Toggle` applies the caller's refinement
      last (`button/toggle.rs:215`) *and* folds it into its checked style
      (`:207-212`, gpui-base `toggle.rs:91-101`), so `segmented_control`'s
      `segment_height`, padding, font and -- on a checked toggle --
      `active_background` and `active_text_color` all have a receiver. Only
      an unchecked toggle's hover is out of reach (Tier U: it is set with
      `.hover()` on the base element). Found 2026-09-22 from a panel that
      called the whole thing Tier U.
      **Done in v0.6.1:** `geometry::toggle` carries the text alone:
      `button.font`'s size and weight and the platform's line height, over the
      `text_sm` gpui-component 0.7.1 sets on a Medium `Toggle`.
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
- [x] (Three copies, iced, gpui and egui: now `AccessibilityPreferences::text_scale_factor` and `scaled` in the core crate, which all three call; their crate docs say what each flag does.) `text_scale_factor` (finite and positive, else 1) is written twice, once
      per connector; a method on `AccessibilityPreferences` in the core crate
      would state it once.
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

---

## Publishing Prep

- [x] Publish to crates.io (v0.5.8 released 2026-09-07, tag `v0.5.8`)

---

## Post-1.0 / Deferred

### Change notification
Done: the `watch` feature's `on_theme_change()` has a backend on each
platform (`native-theme/src/watch/`).

- [x] Linux portal: `SettingChanged` D-Bus signal, through `zbus::blocking` (`watch/gnome.rs`)
- [x] Linux KDE: `notify` crate watching the directory of `kdeglobals` (`watch/kde.rs`)
- [x] macOS: `AppleInterfaceThemeChangedNotification` observer on the distributed notification center (`watch/macos.rs`)
- [x] Windows: `UISettings.ColorValuesChanged` event (`watch/windows.rs`)

### Live presets
- [x] `windows-11-live.toml` still states fields the Windows reader provides: `focus_ring_width_px` (light :14, dark :216, reader: `SM_CXFOCUSBORDER`) and the scrollbar `groove_width_px` / `min_thumb_length_px` (:80-81, :282-283, reader: `SM_CXVSCROLL` / `SM_CYVTHUMB`). Live presets omit reader-provided fields (`native-theme/src/presets/README.md:34-35`). **Done 2026-10-11 (M1-T4-01):** moot; the `-live` presets are removed, since each stated only its platform preset's values, and the readers merge over the platform preset.

### freedesktop icon lookup falls back to hicolor
- [x] `freedesktop-icons` 0.4.0 looks an icon up in `hicolor` when the theme it is given does not exist (`lookup_in_theme`, `THEMES.get(self.theme).or_else(|| THEMES.get("hicolor"))`, `freedesktop-icons-0.4.0/src/lib.rs:297-299`), and also when the theme and its parents lack the icon (`:328-334`), then in the icon base directories and `/usr/share/pixmaps` (`:335-347`). Every lookup in `native-theme/src/freedesktop.rs` goes through it: the icon lookups (`:44`, `:55`) and the spinner's two passes (`:292-296`, `:318-322`). The freedesktop Icon Theme Specification makes `hicolor` every theme's last parent, while this project never mixes icon sets and returns `None` for an icon the chosen theme lacks. Decide which rule the library follows, and whether a nonexistent theme is an error. The showcases cannot reach this now: they take only an installed theme the icon-theme picker lists. **Decided 2026-09-24, done in v0.5.9:** an icon may come from the chosen theme and the themes its `Inherits=` chain declares, never from `hicolor` (unless it is the theme asked for), a loose file in an icon base directory or `/usr/share/pixmaps`; a theme that is not installed is not an error, every icon of it is `None`. `freedesktop.rs` asks freedesktop-icons for each theme of the chain in the specification's depth-first order and keeps a result only when the file lies in that theme's own directory (`theme_chain`, `first_in_chain`), for every lookup, the spinner's included.

### Icon theme detection falls back to hicolor
- [x] `system_icon_theme()` returns `"hicolor"` when detection fails: every Linux reader ends in it (`native-theme/src/model/icons.rs`: `detect_linux_icon_theme`'s Unknown branch `:621-628`, `detect_kde_icon_theme` `:643`, `:655`, `gsettings_icon_theme` `:669`, `detect_xfce_icon_theme` `:683`, `detect_lxqt_icon_theme` `:693`, `:708`). The freedesktop lookup allows `hicolor` when it is the theme asked for, so a failed detection makes `hicolor` the chosen theme and its application icons count as the UI set; the rule that `hicolor` is never a fallback does not see it, because the fallback happens one step earlier. Question for the maintainer: when detection fails, use the desktop's own default icon theme, or return `None`? The defaults found in the installed files: GNOME's `org.gnome.desktop.interface` `icon-theme` defaults to `'Adwaita'` (`/usr/share/glib-2.0/schemas/org.gnome.desktop.interface.gschema.xml:86-87`). KDE's is `KIconTheme::defaultThemeName()` (kiconthemes 6.30.0 exports it from `/usr/lib/libKF6IconThemes.so.6`); the library holds the string `breeze` (`strings -el`), but that does not show the function returns it, and the kiconthemes source was not checked. The Cinnamon and MATE schemas are not installed here, and XFCE and LXQt were not looked into. **Decided 2026-09-24, done in v0.5.9:** neither; the library reports the failure and the application decides. `system_icon_theme()`, `detect_icon_theme()` and `DetectionContext::icon_theme()` return a `Result` whose error names each source tried and why it gave no name (both attempts on an unrecognised desktop); other platforms than Linux, macOS, iOS and Windows get `Error::PlatformUnsupported`, not `"material"`. The detection cache keeps the failure. `Resolved::icon_theme` and `SystemTheme::icon_theme` are `Option`, `None` where the theme states none and detection fails; `FreedesktopLoader::load` and `load_indicator` without a theme search no theme then, so they return `None`, except that a custom `IconProvider`'s own freedesktop SVG, which depends on no theme, still loads; both showcases show the system icon theme as unavailable, with the reason.
