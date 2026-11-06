# native-theme: closed backlog items

The items of [`docs/todo.md`](../todo.md) that were done. They move here as they close,
so that the backlog lists open work only: the first batch on 2026-10-11 (M1-T10-01 of the
code review), later ones at the M3 and M4 closes and in M8 T21 (2026-10-17). Each is
copied verbatim with its decision text; from M4's close on, a moved item ends with a
dated line naming the commit that closed it. A `###` section whose every item was done
moves whole, under its `##` heading (an item of *Deferred (pre-1.0)* or *Post-1.0* goes
under *Post-1.0 / Deferred*); a done item from a section that still has open items is
under "Closed items from" and the name of the heading it sat under, and a later one
from the same heading joins the end of that group. Within a group the order is the
order they had in `docs/todo.md`. Their `file:line` citations were correct when written
and many have since drifted, as the archive's [README](README.md) warns.

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

- [x] [v0.7.0] `watch::on_theme_change()` fires on fewer changes than the readers
      read, so an application that rebuilds its theme on it — the egui
      connector's `ThemeWatcher` (`docs/archive/todo_v0.6.0_egui-connector-spec.md`)
      — misses these:

      - **GNOME and Budgie**: the watcher subscribes to the portal's
        `SettingChanged` for the `org.freedesktop.appearance` namespace only
        (`native-theme/src/watch/gnome.rs:46`), while the reader takes the
        font, monospace font, text-scaling factor, animations, overlay
        scrolling and icon theme from `org.gnome.desktop.interface`, the title
        bar font from `org.gnome.desktop.wm.preferences` and high contrast from
        `org.gnome.desktop.a11y.interface` (`native-theme/src/gnome/mod.rs:337-371`).
        Changing any of them fires nothing.
      - **macOS**: the watcher observes `AppleInterfaceThemeChangedNotification`
        alone (`native-theme/src/watch/macos.rs:88-89`), while the reader also
        reads the accent colour, `NSColor::controlAccentColor`
        (`native-theme/src/macos.rs:57`), and the reduce-motion, contrast and
        transparency flags (`:151-155`). Whether macOS posts that notification
        on an accent change is unverified; AppKit declares
        `NSSystemColorsDidChangeNotification` and
        `NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification` for these
        (objc2-app-kit 0.3.2, `NSColor.rs:1068`, `NSAccessibility.rs:190`).
      - **Windows**: the watcher subscribes to `UISettings::ColorValuesChanged`
        alone (`native-theme/src/watch/windows.rs:77`), while the reader's
        text-scaling factor (`native-theme/src/windows.rs:359`) has its own
        event, `UISettings::TextScaleFactorChanged` (windows 0.62.2). Whether a
        change of the non-client fonts, high contrast or client-area animation
        (`:154`, `:369`, `:387`) raises `ColorValuesChanged` is unverified.
      - **KDE**: the watcher sees `kdeglobals` and `kcmfontsrc` in the config
        directory, non-recursively (`native-theme/src/watch/kde.rs:29-32`,
        `:51`), while the reader falls back to `kdedefaults/kdeglobals` for the
        icon theme (`native-theme/src/kde/mod.rs:395`), a file in a
        subdirectory the watcher does not see.

      Decide per platform which further notification to observe, keeping one
      user action to one event.
      Fixed by c951c587 (M4 T21, 2026-10-13): GNOME fires on the
      `SettingChanged` of every key the reader reads (the portal's GNOME and
      GTK backends forward the gsettings namespaces), KDE watches
      `kdedefaults/kdeglobals`, macOS observes the system-colour,
      scroller-style and accessibility-display notifications (and the
      appearance one with `DeliverImmediately`), Windows subscribes to
      `TextScaleFactorChanged`; one change fires once (300 ms debounce) on
      every platform. Each reader module lists its sources under
      `# Sources`, and the table test in `native-theme/src/watch/mod.rs`
      checks each against its watcher. Still not watched on Windows: the
      non-client fonts, high contrast and client-area animation, which
      `WM_SETTINGCHANGE` reports only to a top-level window
      (`native-theme/src/windows.rs`, "Sources not watched").
      T21 fix round 1 (2026-10-13): high contrast
      (`AccessibilitySettings::HighContrastChanged`) and client-area
      animation (`UISettings::AnimationsEnabledChanged`, Windows 10 2004 and
      later) are watched too, with no window; the window-only Windows
      sources are the next entry.
      Closed 2026-10-13 (M4 Step 8): the watchers observe every source the
      readers read except the Windows window-only ones, the open entry
      *Windows watcher: the window-only sources* in `docs/todo.md`. The "Still
      not watched on Windows" sentence above describes c951c587; since 71db4793
      high contrast and client-area animation are watched. The KDE file named
      above as `kcmfontsrc` is `kcmfonts`, the file Plasma opens and the
      watcher watches since 49e0067a.

- [x] [v0.7.0] **KDE: the 300 ms throttle keeps the first event of a burst and drops
      the rest.** `last_fire` starts as `None` and the watcher fires on the
      first relevant event, then ignores every relevant event until 300 ms
      have passed (`native-theme/src/watch/kde.rs:59-60`, `:86`); nothing fires
      after the burst. KDE writes its settings in several steps (the watcher's
      own comment names `QSaveFile`'s multi-write pattern, `:14-15`), so a
      consumer that re-reads on the first event can read the files before the
      last write lands — or before `kcmfontsrc` changes, if `kdeglobals` changed
      first — and nothing tells it to read again. Make it a trailing debounce
      (fire once, 300 ms after the last relevant event), or fire again at the
      end of the window when events were dropped in it.
      Fixed by 7988c1ec (M4 T20, 2026-10-13): a trailing debounce, the
      callback fires once, 300 ms after the last relevant event of a burst;
      the loop checks shutdown on every iteration
      (`native-theme/src/watch/kde.rs` `watch_kde_dir`).
      Closed 2026-10-13 (M4 Step 8): fixed by 7988c1ec. The file named above as
      `kcmfontsrc` is `kcmfonts`, watched since 49e0067a. A maximum wait for the
      debounce is the open entry *Theme watcher: the trailing debounce has no
      maximum wait* in `docs/todo.md`.
- [x] [v0.7.0] **For M5–M7: a preset pinned with `from_system_with_preset` is
      lost on the next system re-read** (M2 final review, Minor 5,
      2026-10-11). The connectors' re-reads call `from_system()`, which picks
      the running version's generation: egui's watcher rebuilds through
      `ThemeWatcher::system_rebuild`
      (`connectors/native-theme-egui/src/watch.rs:100`, then
      `SystemTheme::from_system()` at `ext.rs:242`), and gpui's
      `from_system` (`connectors/native-theme-gpui/src/lib.rs:389`) calls it
      too. An application that pinned `macos-monterey` on macOS 26 gets
      `macos-tahoe` after the first theme change. A feature gap, not a bug:
      decide whether a connector's re-read keeps the pinned preset (the
      watcher or the connector remembering the name, or a re-read entry
      point that takes it).
      For iced, settled (M5-03, 2026-10-14): the connector has no re-read of
      its own; `theme_changes()` (feature `watch`) only signals, and the
      README's and crate docs' *Runtime theme change* recipe re-reads with the
      entry point the application used: `from_system()`, or
      `SystemTheme::from_system_with_preset(name)` then `to_iced_theme()` for
      a pinned preset. gpui and egui stay open (M6, M7).
      M5 close (2026-10-14): gpui and egui weigh iced's answer first. Both
      re-read through `from_system()`: gpui's `from_system`
      (`connectors/native-theme-gpui/src/lib.rs:389`) and egui's
      `ThemeWatcher::system_rebuild`
      (`connectors/native-theme-egui/src/watch.rs:98-101`), so a pinned
      preset needs the application's own re-read with
      `from_system_with_preset`, or a name the watcher keeps.
      For gpui, settled as for iced (M6 T3, 2026-10-15):
      `on_theme_change(window, cx, f)` (feature `watch`) only signals, running
      `f` on gpui's foreground, and the README's and crate docs' *Runtime
      theme change* recipe re-reads in `f` with the entry point the
      application used:
      `SystemTheme::from_system()`, or
      `SystemTheme::from_system_with_preset(name)` for a pinned preset, then
      `apply_system_theme`. egui stays open (M7).
      For egui, settled as for iced and gpui (M7 T3, 2026-10-16):
      `ThemeWatcher::start(ctx, rebuild)` (feature `watch`) takes the rebuild,
      and `system_rebuild` stays the OS theme's; the README's *Runtime theme
      change* recipe and `system_rebuild`'s documentation say that an
      application that pinned a preset passes a closure that reads with
      `SystemTheme::from_system_with_preset(name)` again, after
      `invalidate_caches()`, then `to_egui_atlas()`.
  Archived 2026-10-17 (M8 T21): ticked in a829f6ff.

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

### `SystemTheme::from_system_with_preset`: an older generation over the live OS data

- [x] [v0.7.0] Add `SystemTheme::from_system_with_preset(name)` and its async twin
      (M1-T1-04 in `docs/todo_code-review-findings.md`): the platform reader
      runs as in `from_system()`, and its values merge over the named preset
      instead of the running generation's, so an application, or its user,
      can keep `macos-monterey`'s look on macOS 26 with the OS's colours,
      fonts, DPI and accessibility. Today the only way to an older
      generation is `Theme::preset(name)`, which reads nothing from the OS.
      The name is checked against the generation table's rows for the
      running platform (M1-T1-01), not only `PresetInfo.platforms`, and
      `Error::UnknownPreset` answers any other. The rustdoc says what a pin
      gives: the older generation's geometry over the running OS's colours,
      fonts and accessibility (on macOS 26, `NSColor` gives Tahoe's
      colours), a hybrid rather than the old OS's look, reported as
      `PresetOlder` by the version match. The accessibility path needs no
      counterpart: it reads no preset. Implement after M1-T1-01 and
      `macos-tahoe` (M2a Step 9) land; until a platform has a second
      generation there is nothing to choose.
      Done in M2a Step 10a (2026-10-11): `SystemTheme::from_system_with_preset` and
      `from_system_with_preset_async`.

### `SystemTheme::with_overlay` — the overlay's icon theme is never read

- [x] [v0.7.0] `SystemTheme::with_overlay` (`native-theme/src/lib.rs:538`) merges the
      overlay's light and dark variants (`:581-586`) and re-resolves them, but
      builds its result with `icon_theme: self.icon_theme.clone()` (`:603`):
      the base theme's name, whatever the overlay states. Neither the
      overlay's `Theme::icon_theme` (`native-theme/src/model/mod.rs:304`) nor
      its active variant's `ThemeDefaults::icon_theme`
      (`native-theme/src/model/defaults.rs:137`) is read, although
      `from_system` ranks exactly those two above runtime detection
      (`native-theme/src/pipeline.rs:82-92`) and `Theme::merge` lets an
      overlay's `icon_theme` win (`native-theme/src/model/mod.rs:348-350`).
      Expected: an icon theme the overlay states wins, like its other fields —
      the same three tiers applied to the base merged with the overlay — and
      an overlay that states none keeps the base's. The same function keeps
      `icon_set` (`:602`) and `layout` (`:558`, read before the overlay is
      applied) and ignores the overlay's `Theme::icon_set` and `Theme::layout`
      in the same way, so the fix covers all three — and with them the other
      variant's icon theme that `icon_theme_for` (above) returns, resolved by
      the same tiers over the merged theme. Add regression tests
      beside the tier tests in `native-theme/src/pipeline.rs` (`:1265`,
      `:1281`).
      Closed 2026-10-12: `with_overlay` replays the whole theme since bdad0edf
      (M3-41); the overlay's `icon_theme`, `icon_set` and `[layout]` apply;
      regression test in `pipeline.rs`.

### macOS: the scroll bar width for the user's scroller style

- [x] [v0.7.0] For M4 (M2 final review, Minor 4, 2026-10-11): the macOS reader
      reads the scroll bar style live, `NSScroller.preferredScrollerStyle`
      (`native-theme/src/macos.rs:142`, stored as `scrollbar.overlay_mode` at
      `:495-501`), but the groove width is always the preset's constant:
      `macos-monterey` states the overlay style's 16
      (`macos-monterey.toml:189`, `:643`), as ruled in M2a. With "always
      show scroll bars" set, AppKit draws the legacy style, whose width is 15
      on macOS 14.8.9 and 15.7.9 (`docs/platform-facts.md:1280`, the probe's
      `scrollerWidth(for: .regular, scrollerStyle: .legacy)`); on macOS 26
      both styles are 17, which `macos-tahoe` states. The reader could read
      `scrollerWidth(for: .regular, scrollerStyle:)` for the style it reads,
      a live OS value rather than a constant.
      Fixed in M4 T24 (e6c5c737, `fix(macos): the scroll bar's groove width is
      the one AppKit gives the user's scroller style`): the reader writes
      `scrollerWidth(for: .regular, scrollerStyle:)` for the style it reads;
      `scrollbar_for_style_takes_the_styles_width`
      (`native-theme/src/macos.rs`) checks that the overlay style gets the
      overlay width and the legacy style the legacy width.
      Closed 2026-10-13 (M4 Step 8): fixed by e6c5c737, with b41bab4b (the
      reader merges its scroll bar fields). The `macos.rs:142` and `:495-501`
      above predate M4's reader changes (23f8f381). The legacy style's thumb and
      track, for which no source exists, are recorded in `docs/todo.md`'s
      `scrollbar.groove_width` and `scrollbar.thumb_width` rows.

### KDE: the plain-read link test cannot tell the reader from the preset

- [x] [v0.7.0] For M4, the KDE reader (M3 final review, 2026-10-12): the test
      `link_text_resolves_to_foreground_link` (`native-theme/src/kde/mod.rs:935`)
      reads the dark fixture, whose `ForegroundLink` is `#1d99f3`, the same
      value `kde-breeze` states as its dark `defaults.link_color`
      (`native-theme/src/presets/kde-breeze.toml:451`), so it passes whether
      link text comes from the reader or from the preset. Give it a fixture
      whose `ForegroundLink` differs from the preset's (the light fixture has
      no `ForegroundLink`).
      Fixed in M4 T10 (`fix(kde): kdeglobals read as KConfig cascades it`):
      the test reads a fixture whose `ForegroundLink` is `246,116,0`, which
      `kde-breeze` does not state as `link_color`; it fails when the reader
      drops `ForegroundLink`.
      Closed 2026-10-13 (M4 Step 8): fixed by 598959b9.

### kde-breeze: link hover and active colours stated without a source

- [x] [v0.7.0] For M4, the `kde-breeze` preset through the `preset-validator`
      agent (M3 final review, 2026-10-12): `[light.link]` and `[dark.link]`
      state `hover_text_color` and `active_text_color` (`#2980b9` light,
      `native-theme/src/presets/kde-breeze.toml:363-364`; `#1d99f3` dark,
      `:765-766`), Breeze's default blue with no citation, so they never
      follow a KDE colour scheme or an overlay's `defaults.link_color`.
      Find a source for each, or state none so they inherit `link.font.color`
      (`docs/inheritance-rules.toml:290-291`).
      Fixed by fd4abc64: both keys removed, they inherit `link.font.color`;
      a Qt link keeps `palette(link)` hovered and pressed. Kirigami's
      `LinkButton` alone darkens a pressed link, `Qt.darker(linkColor)`
      (kirigami v6.31.0 `templates/LinkButton.qml:66`, `:87-93`), which the
      model does not derive.
      Closed 2026-10-13 (M4 Step 8): fixed by fd4abc64. Kirigami's pressed-link
      darkening is the open entry *Links: Kirigami darkens a pressed link* in
      `docs/todo.md`.

### Validation: an out-of-range `size_pt` is reported with its px value

- [x] [v0.7.0] In the resolve validation code, carried to M4 (M3 final review,
      2026-10-12): `check_font` (`native-theme/src/resolve/validate_helpers.rs:475`)
      names the key the theme stated, `size_pt`, but reports the resolved
      size, converted to px: `size_pt = -3` prints `got -4`. Report the
      value in the stated key's unit, or name the px key.
      Fixed by 443f8141: `check_font` reports the stated points.
      Closed 2026-10-13 (M4 Step 8): fixed by 443f8141; the text-scale range
      check reports a stated size in its unit too since 79a9f2ae.

### Icons: one monochrome-SVG colouriser, in native-theme

- [x] [v0.7.0] The iced and gpui connectors each recolour a monochrome SVG icon with a
      private copy of the same idea — iced's `colorize_monochrome_svg`
      (`connectors/native-theme-iced/src/icons.rs:277`) and gpui's
      `colorize_svg` (`connectors/native-theme-gpui/src/icons.rs:1299`) — and
      the copies differ: iced returns as soon as it has replaced
      `currentColor` (`connectors/native-theme-iced/src/icons.rs:292-293`),
      while gpui goes on to replace the explicit black fills and strokes as
      well (`connectors/native-theme-gpui/src/icons.rs:1312-1333`). The egui
      connector's plan adds the one implementation to native-theme,
      `native_theme::icons::colorize_monochrome_svg`, with gpui's algorithm
      and gpui's unit tests. Switch both siblings to it, so an icon that mixes
      `currentColor` with an explicit black is coloured alike in all three
      connectors, and delete the two copies, moving into native-theme any
      test case of theirs the core's tests lack.
      iced: switched in M5 T8 (d809a666); gpui in M6 T4: each of its eleven
      `colorize_svg` cases is a case of native-theme's `colorize_tests` of
      the same name and input, so none moved.
      egui (M7 T4, 2026-10-16, closed by inspection): `to_image_source`
      colours a bundled set's SVG through core's `colorize_monochrome_svg`,
      as it has since the connector was written, and holds no copy; its
      `custom_icon_to_image_source` now loads through
      `load_custom_icon_in`, which colours with the same function.
  Archived 2026-10-17 (M8 T21): ticked in 0cf938bc.

### Icons: one `currentColor` fill for a freedesktop indicator, in native-theme

- [x] [v0.7.0] For M6/M7 (ruling M5-04, 2026-10-14): core's
      `native_theme::icons::fill_current_color`
      (`native-theme/src/icons.rs:891`; M5 T7, 89ec7f61) replaces a symbolic
      freedesktop icon's `currentColor` with a colour's `#rrggbb` where the
      document sets no `color` of its own, and iced's `Spinner` calls it with
      `defaults.text_color` (`connectors/native-theme-iced/src/spinner.rs:131`).
      gpui and egui keep their own copies of the same substitution: gpui's
      `sets_color` and `paint_svg`
      (`connectors/native-theme-gpui/src/widgets/spinner.rs:132`, `:153`),
      whose colour `indicator_colour` picks (`:173`), and egui's `sets_color`
      and the freedesktop arm of `final_svg_bytes`
      (`connectors/native-theme-egui/src/icons.rs:168`, `:238-247`). Switch
      both to the core function and delete the copies, moving into
      native-theme any test case of theirs its tests lack. The doc of egui's
      `final_svg_bytes` still cites iced's step at
      `connectors/native-theme-iced/src/icons.rs:313-314`
      (`connectors/native-theme-egui/src/icons.rs:234`), a copy M5 T8
      removed.
      gpui done (M6 T8, e7e3f809): the `Spinner` loads through
      `animated_frames_to_image_sources_for`, which fills a freedesktop
      frame through `fill_current_color`; `sets_color` and `paint_svg` are
      gone from `widgets/spinner.rs`.
      egui done (M7 T4, 2026-10-16): `final_svg_bytes`'s freedesktop arm
      calls `fill_current_color` with `load_color`'s colour, its doc no
      longer cites iced's removed step, and `sets_color` is gone; each of
      its test's cases is a case of native-theme's
      `sets_color_reads_only_the_color_property` or
      `fill_current_color_borrows_what_it_leaves_alone` (the Breeze
      `.ColorScheme-Text { color:#232629; }` shape), so none moved.
      `custom_icon_to_image_source` loads through `load_custom_icon_in`,
      which fills a freedesktop icon the same way, and colours nothing
      again.
  Archived 2026-10-17 (M8 T21): ticked in 0cf938bc.

### windows-11: a border opacity with no source

- [x] [v0.7.0] `windows-11.toml` states `defaults.border.opacity = 0.14` in both
      variants (light `:70`, dark `:567`), and `docs/platform-facts.md:1120`
      gives Windows no multiplier: WinUI 3 states each stroke role as a colour
      with its own alpha. M2b-P07 (`docs/todo_code-review-findings.md`) was
      deferred to the *Border opacity* entry, which closed on 2026-09-28
      (`docs/archive/todo_closed.md:181-231`) saying "the windows-11 (0.14) …
      values have no source; nothing reads them", so no open item owned the
      value. For M3: remove the key from both variants if the resolved theme
      can do without it (`ResolvedDefaultsBorder::opacity` is an `f32`,
      `native-theme/src/model/border.rs:232`), or mark it **(preset)** ❓ in the
      preset as the facts row does.
      *Closed by M2b fix round 2 (2026-10-11):* the opacity is
      `CardStrokeColorDefault`'s alpha, already in the stated
      `defaults.border.color` (`model/border.rs`: no connector multiplies by
      it), as macos-monterey's 0.098 is: 15/255 = 0.059 light (`#0000000f`)
      and 25/255 = 0.098 dark (`#00000019`); `docs/platform-facts.md:1120`
      says so. It moves to the archive at the next prune.
  Archived 2026-10-17 (M8 T21): ticked in 9ac11178.

### windows-11: the switch's off-track outline

- [x] [v0.7.0] For M3: WinUI 3 draws the off switch's track with a stroke,
      `ToggleSwitchStrokeOff` = `ControlStrongStrokeColorDefault`, `#72000000`
      light and `#8BFFFFFF` dark (ToggleSwitch_themeresources.xaml:139, :19;
      Common_themeresources_any.xaml:252, :48, microsoft-ui-xaml 8463f45),
      over a nearly clear fill, `ToggleSwitchFillOff` =
      `ControlAltFillColorSecondary`, `#06000000` / `#19000000`
      (`docs/platform-facts.md:1522`). The model has no switch border colour,
      so windows-11's off switch draws as a faint pill with no outline
      (`#00000006` over `#f3f3f3`). Its hover and disabled strokes
      (`ToggleSwitchStrokeOffPointerOver`, `ToggleSwitchStrokeOffDisabled`,
      :140, :142) have no field either. Decide whether the model gains an
      off-track border colour; M2b fix round 2 gave the off track's hover and
      disabled fills their WinUI resources.
      *Closed by M8 T16c (2026-10-17):* `SwitchTheme` carries the track's
      outline (`unchecked_track_border_color`, `checked_track_border_color`,
      their `disabled_*` forms, `track_border_line_width`) and the thumb's
      (`thumb_border_color`, `thumb_border_line_width`), soft options
      (`docs/platform-facts.md` §2.21); windows-11 states the off stroke,
      `#00000072` / `#ffffff8b`, its disabled form `#00000037` / `#ffffff28`,
      and the width 1, and no on-track outline (`ToggleSwitchOnStrokeThickness`
      0). The hover and pressed strokes are the rest colour, which the
      connectors draw in those states. It moves to the archive at the next
      prune.
  Archived 2026-10-17 (M8 T21): ticked in 4cc70dbc.

### For M5: iced's primary hover from `primary_background`, then windows-11 states `AccentFillColorDefault`

- [x] [v0.7.0] For M5 (M4 T35 ruling, 2026-10-13): the iced connector puts
      `defaults.accent_color` in the palette's primary slot
      (`connectors/native-theme-iced/src/palette.rs:43`) and derives the
      primary button's hover and pressed fills from it, so its contract test
      `the_primary_pair_is_the_accent_pair`
      (`connectors/native-theme-iced/src/contract.rs:2739`) requires the idle
      `button.primary_background` to equal the accent. windows-11's static
      `primary_background` therefore inherits the accent, `#0078d4`
      (ee75ad7e; the comment at `native-theme/src/presets/windows-11.toml:139-141`),
      where `docs/platform-facts.md:1188` gives WinUI 3's
      `AccentFillColorDefault`, `#0067c0` / `#4cc2ff` for the default accent.
      The live reader already writes the `AccentFillColorDefault` shades there
      (8ccdbed0), so a live Windows theme already breaks the iced assumption.
      In M5 iced derives the primary hover and pressed fills from
      `primary_background`; then windows-11 states `AccentFillColorDefault`,
      as its `checked_background` does (9284f52b).
      Done in M5 T9: iced's primary hover and pressed fills from
      `primary_background` (07362c9b); windows-11 states
      `AccentFillColorDefault` (cc9e933e).
  Archived 2026-10-17 (M8 T21): ticked in 94bdf0cb.

### Presets: the toolbar's and the popover's edge width is the defaults'

- [x] [v0.7.0] From M6 T16b (2026-10-16): the toolbar and the popover inherit
      `defaults.border` (`docs/inheritance-rules.toml:92-96`,
      `widgets_with_border`), so their resolved `border.line_width` is 1 or
      0.5 in most presets (the toolbar's is 0 only in kde-breeze and
      material-3; the popover's is stated only by adwaita, 1 from libadwaita's
      outline, `adwaita.toml:296`, and by material-3, 0), while
      `docs/platform-facts.md` §2.13 states no toolbar frame and gpui-component
      draws a popover's edge as its own ring inside the shadow (gpui-component
      0.7.3 `styled.rs:186-194`). The card's fill is the same kind of value:
      `card.background_color` inherits `defaults.surface_color`
      (`docs/inheritance-rules.toml:274`), and only material-3 (both modes)
      and adwaita (dark, `#ffffff14`, `adwaita.toml:771`, libadwaita
      `_misc.scss:203`) state one, while §2.26 (`docs/platform-facts.md:1657`) states no card for macOS and
      KDE. Should the inheritance give the toolbar and the popover a 0 width
      and the card no fill, or should their presets state them from a source?
      Until then the gpui connector carries neither `toolbar.border.color`,
      `popover.border.color` nor `card.background_color`; M6 T16c names all
      three in the coverage test.
      M6 T16c fix round 1 (2026-10-16): adwaita also states the popover's
      edge colour, `#0000000d` in both modes from libadwaita's outline
      (`adwaita.toml:296`, `:678`, `docs/platform-facts.md:1457`). A resolved
      theme cannot tell a stated value from an inherited one, so until this
      is settled gpui draws neither adwaita's stated popover edge nor its dark
      card fill, nor material-3's card fill: sourced values not drawn, so the
      item is retagged [v0.7.0], for M8.
      M8 T5 (2026-10-16, da8144f2, 92cd1665, 3e31d86e): the connector half is
      done. gpui carries the three as the theme states them:
      `geometry::group_box_content` fills the card with
      `card.background_color`, `geometry::popover` edges the popover in
      `popover.border.color` at its `line_width`, and `geometry::toolbar`
      draws the bar's bottom edge in `toolbar.border.color` at its
      `line_width`; the coverage test's three entries and its `NoEdgeByData`
      / `NoFillByData` classes are gone. The preset half (T15, T16) and the
      `[wrong_safety_nets]` entries (T17) stay open here.
      M8 T15 (2026-10-17): the facts half. §2.13 has `border.color`,
      `border.corner_radius`, `border.line_width` and `border.shadow_enabled`
      rows: no toolbar frame on KDE (Breeze's `CE_ToolBar` paints nothing),
      GNOME (`.toolbar` sets none) and Material, none at rest on Windows (a
      closed `CommandBar`'s `OpenBorder` is collapsed), ❓ on macOS (the
      title-bar separator, `NSWindow.titlebarSeparatorStyle`, `.automatic`).
      §2.16's macOS, Windows and KDE edge cells no longer read
      `← defaults.border.*`: Windows `SurfaceStrokeColorFlyout`, 1; KDE
      qqc2-desktop-style's Popup, #c2c5c9 / #474c51, 1; macOS ❓. §2.26's and
      §2.13's macOS 26 cells stay ❓: no source gives them.
      M8 T16 (2026-10-17, ac70ed0d..b1423ef6): the presets' half. They follow T15's
      rows: the connectors draw `toolbar.border` as the bar's bottom rule, so
      adwaita, windows-11 (0 at rest, with its rest radius 0) and material-3
      state none, kde-breeze states Breeze's 1px tools-area line in the
      Header set's separator colour (#b9bbbd / #535659), and the macOS
      presets keep the inherited edge (§2.13 macOS ❓). kde-breeze and
      windows-11 state the popover edge §2.16 gives (#c2c5c9 / #474c51;
      `SurfaceStrokeColorFlyout`, #0000000f / #00000033); adwaita states
      its own already. No preset has a transparent card: §2.26 gives KDE
      Kirigami's `Card`, whose fill is kde-breeze's inherited surface colour
      and whose edge, radius, shadow and padding kde-breeze now states, and
      macOS `NSBox`, whose look Apple does not publish (❓), so the macOS
      presets keep the inherited fill and state only the box's 5 of padding.
      The `[wrong_safety_nets]` entries are T17's.
      M8 T17 (2026-10-17): the `[wrong_safety_nets]` entries.
      `docs/inheritance-rules.toml` records `toolbar.border.line_width` (no edge
      on Windows at rest, GNOME and Material, §2.13) and `toolbar.border.color`
      (KDE's Header-set separator); no popover entry, since no §2.16 edge row
      states "no edge" with a source (Windows, KDE and GNOME give a 1px edge,
      macOS is ❓, Material's `(none)` and `0` cite nothing), and no card entry
      (§2.26: macOS `NSBox`, KDE Kirigami's `Card`). The item closes.
  Archived 2026-10-17 (M8 T21): ticked in 7c9fb875.

### Closed items from Presets: values the M8 T15 platform-facts rows disagree with

- [x] [v0.7.0] From the M9a T6 review (2026-10-18,
      `.superpowers/sdd/todo_code-review-plan/m9a-T6-review.md`, *Controller's
      questions*): windows-11's menu rows are 23 px (`windows-11.toml`
      `[light.menu] row_height_px`, `[dark.menu] row_height_px`), from the Windows
      cell of §2.6's `row_height` row (`docs/platform-facts.md:1260`: "mouse/narrow
      mode=23 (14 text + 4+5 pad); no explicit MinHeight resource — derived from
      padding"). That cell counts the 14 px font size as the text's height, where
      the same document gives Windows's line box as Fluent Body 20 px over 14 px
      (`docs/platform-facts.md:1034`, `line_height` 1.43), so WinUI's narrow
      MenuFlyoutItem is about 4 + 20 + 5 = 29 px, and touch mode's 31 (14 + 8 + 9)
      repeats the error. Since M9a T6 the gpui showcase's chrome menu draws a row
      at the stated height by `geometry::menu_item`'s control-height rule, as the
      Overlays page's rows already did, so under windows-11 the 20 px line fills
      the 23 px row and the stated 4 / 5 px padding is lost. The fix: verify the
      §2.6 Windows cell independently and correct it, then the preset (the review
      suggests stating no `row_height`, as a row with no MinHeight sizes to its
      font, which the model encodes as none); the CHANGELOG line for M9-012 names
      windows-11 as its example and follows the change.
      M9a T6 fix round 2 (2026-10-18, 8fbcf9ae): verified independently
      against microsoft-ui-xaml at 8463f451, neither 23 nor 29 has a source:
      `MenuFlyoutItem` has no MinHeight and is its padding, 4+5 by mouse, pen
      or keyboard and 8+9 otherwise, round a TextBlock with no Style and no
      LineHeight, whose line box is Segoe UI Variable's line spacing at 14 px,
      which no source states. §2.6's Windows `row_height` cell is ❓ with those
      sources, windows-11 states no `menu.row_height` (`windows-11.toml`
      `[light.menu]`, `[dark.menu]`), and the M9-012 CHANGELOG line names no
      windows-11 example. The item closes.

### Tab: an active-tab indicator

- [x] [1.0] Research in `docs/platform-facts.md` **first**, cited to authoritative
      sources, which platforms mark the active tab with an indicator line
      (underline or similar) and at what thickness and colour, then add a `tab`
      field for it. `TabTheme` (`native-theme/src/model/widgets/mod.rs:373-405`)
      has no such field, so no connector paints an indicator until the model
      states one — the egui connector marks the active tab only by the
      `selection.*` colours of its `Tab` scope's `Normal` cell
      (`tab.active_background`, `tab.active_text_color`; connector spec §6.2); no thickness is to be invented in the meantime.
      Found 2026-09-28 for KDE: Breeze fills a 3px `QPalette::Highlight`
      strip along the selected North tab's top edge (the bottom edge for
      South tabs), with no pen, rounded `Frame_FrameRadius` = 5 on the
      selected tab's corners — `TabBar_ActiveEffectSize` = 3
      ([breezemetrics.h:137](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezemetrics.h#L137)),
      painted after the tab's frame in `Helper::renderTabBarTab`
      ([breezehelper.cpp:1471-1487](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L1471-L1487));
      recorded in `docs/platform-facts.md:1354` (§2.11). The other three
      platforms are still to be researched before the field is added.
      Closed 2026-10-22 (G3 T11): met by e610750b (2026-09-30), which added
      the three `tab.active_indicator_*` fields, their §2.11 facts rows for
      all five platforms and their receivers.

### windows-11: what WinUI 3 draws, or what Win32 reports

- [x] [v0.7.0] M2b Step 7b (`docs/todo_code-review-findings.md`, M2b) has
      windows-11 state what WinUI 3 draws. Where a Windows cell of
      `docs/platform-facts.md` names a Win32 or `UISettings` API whose value
      differs (the base font size, the window background, text, the button's
      fill and text, the input's text, the tooltip), the cell now gives the
      WinUI brush beside the API, and the preset states the brush. On a live
      system the Windows reader reads the Win32 and `UISettings` values and
      merges them over the preset (`native-theme/src/pipeline.rs:42-43`),
      so a live Windows theme mixes the two. For M3 (the model) and M4 (the
      reader): decide which one the Windows platform theme means, and make
      the reader and the preset agree. Two preset values still follow Win32
      or the window rather than WinUI: `input.background_color` `#ffffff`
      (`COLOR_WINDOW`; WinUI's `TextControlBackground` is
      `ControlFillColorDefault`, `#B3FFFFFF`, TextBox_themeresources.xaml:130)
      and `tab.active_background` `#f3f3f3` / `#202020` (WinUI's
      `TabViewItemHeaderBackgroundSelected` is
      `SolidBackgroundFillColorTertiary`, `#F9F9F9` / `#282828`,
      TabView_themeresources.xaml:7, :85; Common_themeresources_any.xaml:274,
      :70). `button.checked_background` states the accent, `#0078d4`, for
      `AccentFillColorDefault` (`docs/platform-facts.md:1194`), but that brush
      is `SystemAccentColorDark1` light and `SystemAccentColorLight2` dark
      (Common_themeresources_any.xaml:329, :125), `#0067c0` / `#4cc2ff` for
      the default accent on Windows 11 25H2
      (`docs/evidence/os-probe/2026-10-11-run-38129819806/os-probe-windows-11-arm/probe.toml:755`, `:758`).
      *M2b fix round 2 (2026-10-11):* `input.background_color` now states
      `TextControlBackground` = `ControlFillColorDefault` as the resource
      gives it, `#ffffffb3` / `#ffffff0f` (TextBox_themeresources.xaml:130,
      :23), and `docs/platform-facts.md:1203` gives the brush beside
      `COLOR_WINDOW`; the old dark `#2d2d2d` had no source.
      `tab.active_background` and `button.checked_background` are still open.
      *M2 final review (2026-10-11, I1), for M4:* the mix shows in dark mode.
      `apply_sys_colors` (`native-theme/src/windows.rs:268-281`) writes the
      `GetSysColor` values over whichever variant `build_theme` fills
      (`:449-452`), the dark one included: the button's rest fill and text
      become `COLOR_BTNFACE` and `COLOR_BTNTEXT`, `#f0f0f0` and `#000000` on
      the probe's runner
      (`docs/evidence/os-probe/2026-10-11-run-38129819806/os-probe-windows-11-arm/probe.toml:398`,
      `:413`). That runner was in light mode (`AppsUseLightTheme = 1`,
      `:768`); the review reads these colours as not following dark mode,
      which no dark-mode probe has checked yet. Since M2b the dark preset's
      `button.hover_background` is WinUI's opaque, flattened `#323232`
      (`windows-11.toml` `[dark.button] hover_background`), where it was a near-invisible translucent
      layer, so hovering such a light button turns it dark grey. The code
      predates M2; M4 decides with the rest of this entry whether the reader
      writes Win32 colours into the dark variant at all.
      I1 fixed by 8c133614 (M4 T22, 2026-10-13): the reader writes no
      `GetSysColor` colour outside high contrast, in either variant, nor the
      DWM colorization, `UISettings`' foreground and background, the accent
      focus ring and selection or `SM_CXFOCUSBORDER`; outside high contrast
      it writes light or dark, the accent colours, the text scale, the
      non-client fonts and the accessibility flags, and the preset states
      the rest. Under high contrast it maps WinUI 3's HighContrast
      dictionaries (8463f45) to the system colours, cited per field in
      `apply_sys_colors` (`native-theme/src/windows.rs`). Still open here:
      the preset's `tab.active_background` and `button.checked_background`;
      the reader's `button.primary_background` and `checked_background`
      from AccentDark1 / AccentLight1 outside high contrast, against
      `AccentFillColorDefault` = `SystemAccentColorDark1` /
      `SystemAccentColorLight2` (M4-38's related note).
      M4 Step 8 (2026-10-13): the reader's accent fills are
      `AccentFillColorDefault` since 8ccdbed0 (`SystemAccentColorDark1` /
      `Light2` of the user's accent; under high contrast `COLOR_HIGHLIGHT`), so
      the "AccentDark1 / AccentLight1" half above is fixed. The preset states
      `button.checked_background` as `AccentFillColorDefault`, `#0067c0` /
      `#4cc2ff` (9284f52b); its `primary_background` stays on the accent until
      M5 (ee75ad7e; the entry *For M5: iced's primary hover from
      `primary_background`*, now in `docs/archive/todo_closed.md`). Still open: `tab.active_background`, `#f3f3f3` /
      `#202020` (`native-theme/src/presets/windows-11.toml` `[light.tab] active_background`,
      `[dark.tab] active_background`).
      M5 T9: the preset's `primary_background` is `AccentFillColorDefault`
      since cc9e933e.
      Closed 2026-10-22 (G3 T15): the last open item is met by the commit
      `feat(windows-11): the selected tab curves out by 4 at its feet and is
      filled SolidBackgroundFillColorTertiary, …` (G3 T15): the preset's
      `tab.active_background` is `TabViewItemHeaderBackgroundSelected` =
      `SolidBackgroundFillColorTertiary`, `#f9f9f9` / `#282828`
      (TabView_themeresources.xaml:7, :85; Common_themeresources_any.xaml:274,
      :70 at microsoft-ui-xaml 8463f45), and `docs/platform-facts.md:1347`'s
      Windows cell gives the brush in place of "← `defaults.background_color`".

### material-3: the screenshot names and the display name

- [x] [v0.7.0] At the next `scripts/generate_assets_release.sh` run, the six README
      links to `linux-material-{light,dark}.png` (the gpui, iced and egui
      READMEs) become `linux-material-3-*`, since the screenshot scripts name
      their output after the preset key, and the old PNGs are removed with
      `git rm`. The display name "Material 3" is set in
      the preset's line of the `bundled_presets!` table
      (`native-theme/src/presets.rs`) when M2d rewrites the preset, with
      "Material 3 Expressive" for the second Material preset; the menus say
      "Material" until then. Nothing is renamed before
      (`docs/archive/todo_v0.7.0_code-review-plan.md`, M2d Step 15).
      *Closed 2026-10-27 by `b933efe0` (the v0.7.0 release asset run):* the six
      README links name `linux-material-3-{light,dark}.png` and the old PNGs are
      removed. The display-name half waits for M2d, as written; it is the open
      `[v0.7.x]` entry *material-3: the display names "Material 3" and "Material 3
      Expressive"* in `docs/todo.md`, *Core API*.

### macos-monterey and macos-tahoe: the screenshot names and the display name

- [x] [v0.7.0] At the next `scripts/generate_assets_release.sh` run (M2a
      Step 12): the captures are named after the preset key, so the six
      README links to `macos-macos-sonoma-{light,dark}.png`
      (`connectors/native-theme-egui/README.md`, `native-theme-iced/README.md`,
      `native-theme-gpui/README.md`) become `macos-macos-monterey-*`, and the
      old PNGs are removed with `git rm`; `macos-monterey`'s display name and
      TOML `name` become "macOS Monterey" (its line of
      the `bundled_presets!` table in `native-theme/src/presets.rs`,
      `macos-monterey.toml`'s `name`; "macOS Sonoma" until that run, ruled 2026-10-11). The run also
      captures `macos-tahoe` (`.github/workflows/screenshots.yml`), and the
      three connector READMEs show `macos-macos-tahoe-{light,dark}.png` beside
      the Monterey pair (ruled 2026-10-11, Q5); no README links them before
      the files exist. The header comment of
      `scripts/generate_screenshots_gpui.sh` ("macOS Sonoma and Windows 11 are
      captured by CI") then names both macOS presets (`_egui.sh` has no such
      line).
  *The header-comment half done by M8 T12 (2026-10-17, 7e450822):* the
      sentence went (Step 6 ruling, M8-40), so the run leaves that header as
      it is.
      *Closed 2026-10-27:* `macos-monterey`'s display name and TOML `name` are
      "macOS Monterey" (`bd903293`); the release asset run (`b933efe0`) captured
      `macos-macos-monterey-*` and `macos-macos-tahoe-*` on the `macos-26` runner
      (macOS 26.6.2), the three galleries link both pairs and say so, and the
      `macos-macos-sonoma-*` PNGs are removed.

---

### CI: the macOS and Windows legs and the strict-panic steps have not run


- [x] [v0.7.x] M4 added CI test legs for `macos,watch,system-icons` and
      `windows,watch,system-icons` (9ed71649) and strict-panic steps for both
      targets (c373fdb0, `.github/workflows/ci.yml`, *Strict panic lints
      (native-theme, macOS and Windows targets)*, `:82-89`). CI runs on a push to
      `main`, a tag or a pull request (`ci.yml:3-7`), not on a branch push, so
      none of them has run, nor the macOS and Windows runtime tests the M4
      fixes wrote (the watcher, reader and icon tests of T2, T3, T5, T7, T19–T24
      and T26, known-unrun). Run them at the next pull request or push to
      `main`, and fix what fails.
      M8 T21 (2026-10-17): since M8 T11 (60dcffbb) ci.yml has no tag trigger
      of its own (its `on:` block, `ci.yml:6-10`): a `v*` tag runs it inside
      publish.yml, whose `ci-full` job calls it (`workflow_call`) on the
      tag's commit before the upload; a branch push still runs nothing.
  *Closed 2026-10-29 (0.8.0 M0 triage):* they ran: CI run 40962868944 (pull request #6) passed all 39 jobs, the macOS and Windows test legs with their features and the strict-panic steps among them, as the v0.7.0 publish run 40580852144 had.

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
- [x] [v0.7.0] For M7: the base style's `extreme_bg_color` comes from
      `scrollbar.track_color` (`connectors/native-theme-egui/src/mapping.md:432`,
      `connectors/native-theme-egui/src/style/base.rs:202`), which
      `macos-monterey` states transparent, the overlay scroll bar's clear track
      (`native-theme/src/presets/macos-monterey.toml:193`, `:646`). egui 0.36.2
      also fills a `ProgressBar`'s track (`egui/src/widgets/progress_bar.rs:140`)
      and `Frame::canvas` (`egui/src/containers/frame.rs:231`) with it, so on
      macOS a plain `ProgressBar` outside the progress-bar role, and an app's
      canvas or plot frame, lose their fill. The role cell rewrites it from
      `progress_bar.track_color` (`connectors/native-theme-egui/src/style/roles.rs:542`);
      the base mapping does not. Decide which theme colour the base
      `extreme_bg_color` takes.
      `windows-11` states the track transparent too since M2b Step 7b, WinUI's
      `SubtleFillColorTransparent`, `#ffffff00` in light and dark
      (`native-theme/src/presets/windows-11.toml:239`, `:799`;
      `docs/platform-facts.md:1304`), so the same holds there.
      M7 T14 (2026-10-16): egui paints the scroll-bar groove from the slot,
      at the three `spacing.scroll.*_background_opacity` when floating and at
      1 when solid (`egui/src/containers/scroll_area.rs:1485-1511`). Where the
      bar is an overlay one and its stated track fully transparent
      (macos-monterey, macos-tahoe), the base style writes the three
      opacities 0 and `extreme_bg_color` takes `input.background_color`,
      egui's own meaning of the slot (`egui/src/style.rs:1513`); elsewhere it
      keeps `scrollbar.track_color`. windows-11 states its transparent track
      in solid mode, where the groove is painted at opacity 1, so its slot
      stays transparent: the README's *Limits* records it, and mapping.md's
      `scrollbar.track_color` row is DERIVED with the condition.
  Archived 2026-10-17 (M8 T21): ticked in 4fa37c3f.
- [x] [v0.7.0] For M7: the left and right panels fill with
      `sidebar.background_color`
      (`connectors/native-theme-egui/src/style/frames.rs:190-193`), which
      `windows-11` states transparent since M2b Step 7b, WinUI's docked
      `NavigationView` pane, `#f3f3f300` / `#20202000`
      (`native-theme/src/presets/windows-11.toml:379`, `:939`). eframe
      0.36.2's root `Ui` "has no margin or background color"
      (`eframe/src/epi.rs:173`), and its default `clear_color` is
      `rgba(12, 12, 12, 180)` (`epi.rs:248-252`), so an ordinary eframe app
      shows that near-black behind the sidebar, under windows-11 light's
      `#000000e4` text. The egui showcase hides it: its `clear_color` returns
      `panel_fill` (`connectors/native-theme-egui/examples/showcase-egui/app.rs:891-893`).
      Decide whether the connector documents or supplies a `clear_color`, or
      what a transparent panel fill means. For M6: gpui depends on the app's
      root background the same way; gpui-base 0.7.1's `Root` paints none
      (`gpui-base/src/root.rs:281-309`), and the gpui showcase sets one
      (`connectors/native-theme-gpui/examples/showcase-gpui/app.rs:2227`).
      M6 T15 (2026-10-15), the gpui half: a README paragraph under Quick
      start. gpui-base 0.7.3's `Root` paints none (`src/root.rs:280-310`),
      but gpui-component 0.7.3's root plugin, which `gpui_kit::init`
      registers, fills every `gpui_kit::open_window` root with the theme's
      `background` (`window.background_color`; gpui-component
      `src/root.rs:444-452`), so only a view opened with gpui's own
      `cx.open_window` paints its own (`cx.theme().background`, as the
      showcase's root view does, `app.rs:2213`). The egui half stays M7's.
      M7 T13 (2026-10-16), the egui half: a recipe, no API. The panel fill
      is theme data and stays as stated; what shows through it is eframe's
      `App::clear_color`, which only the application sets. The README's
      *Window background* section, the crate docs' and `ThemeAtlas::install`'s
      doc say that eframe paints it behind every panel and that an
      application returns `visuals.panel_fill.to_normalized_gamma_f32()`,
      the theme's `defaults.background_color`, as the showcase does
      (`connectors/native-theme-egui/examples/showcase-egui/app.rs:895-897`).
  Archived 2026-10-17 (M8 T21): ticked in be1fbf49.

- [x] [1.0] Implement the companion widget crate, `native-theme-egui-widgets`, per
      `docs/archive/todo_egui-widgets-spec.md` (rationale:
      `docs/archive/todo_egui-widgets-rationale.md`): a switch, a slider, a spinner, a
      segmented control and link wrappers. Milestone undecided; it starts after
      the connector, whose API it consumes. When it is scheduled, re-verify the
      spec against the connector as built, then write its plan.
  *Closed 2026-10-29 (0.8.0 M0 triage):* done: `connectors/native-theme-egui-widgets` (the switch, slider, spinner, segmented control and link wrappers, with a combo box, expander, progress bar and radio button) shipped in 0.6.0 and 0.7.0.

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
- [x] [upstream] **File the gpui-fast issue: in test mode a partly reused
      frame keeps every bound the last frame drew.** For the maintainer to
      file on longbridge/gpui-fast (an agent files nothing). Under
      `test-support`, gpui-fast 0.1.7's and 0.1.8's `finish_retained_frame`
      (`src/fast/retained.rs:1360-1371`, identical in both) copies all of
      the rendered frame's `debug_bounds` into the next frame whenever any
      subtree was reused, so
      `VisualTestContext::debug_bounds` still finds an element the new frame
      no longer draws. Our repro: five showcase tests that assert an element
      is gone failed on GPUI Fast and pass on gpui-pre (the sixth failure of
      the first GPUI Fast run, `a_checkbox_toggles_on_click_enter_and_space`,
      was our harness: its view's `on_change` changed state without
      `cx.notify()`). Our draw helpers work around it with `window.refresh()`
      before `window.draw(cx)` (`connectors/native-theme-gpui/examples/showcase-gpui/tests.rs`,
      `draw`; `connectors/native-theme-gpui/src/widgets/tests.rs`, `redraw`);
      once a release fixes it, the workaround can go (the helpers' comments
      say why it is there). Found 2026-10-12; reproduced 2026-10-14 at
      `5a719b99` (the `refresh()` taken out of `draw`: exactly these five
      fail, 178 pass). The issue, ready to paste:

      > **Title:** test-support: a partly reused frame keeps the
      > `debug_bounds` of elements it no longer draws
      >
      > gpui-fast 0.1.7 and 0.1.8 (identical at the lines below) with
      > `test-support` (through gpui-kit 0.7.3's `gpui-fast` and
      > `test-support` features), Linux.
      >
      > `finish_retained_frame` (`src/fast/retained.rs:1360-1371`) ends every
      > frame that reused a subtree by copying **all** of the rendered frame's
      > debug bounds into the new one:
      >
      > ```rust
      > #[cfg(any(test, feature = "test-support"))]
      > if window.next_frame.retained.reused_any() {
      >     // Reused subtrees do not paint, and the bounds they would have
      >     // recorded for tests to find them by are last frame's.
      >     for (selector, bounds) in &window.rendered_frame.debug_bounds {
      >         window
      >             .next_frame
      >             .debug_bounds
      >             .entry(selector.clone())
      >             .or_insert(*bounds);
      >     }
      > }
      > ```
      >
      > So once any part of a frame is reused, `VisualTestContext::debug_bounds`
      > returns last frame's bounds for an element the new frame did not draw,
      > and a test that asserts an element is gone (a hidden side panel, a
      > tab's content after a switch, a menu row that moved into the title
      > bar) fails. A reused layer range's own bounds are already carried, moved
      > by its offset, in `src/fast/layers/reuse.rs:214-220`; the copy above
      > also carries every selector outside the reused subtrees. Expected:
      > only the reused subtrees' bounds carry forward, so an element that is
      > no longer drawn has none.
      >
      > Repro: native-theme's gpui showcase
      > (https://github.com/tiborgats/native-theme, `connectors/native-theme-gpui`).
      > Its test helper `draw` (`examples/showcase-gpui/tests.rs`) calls
      > `window.refresh()` before `window.draw(cx)` as a workaround (`refreshing`
      > turns reuse off, `retained.rs:1090`, `splice.rs:302`). Without that
      > line, `cargo test -p native-theme-gpui --features gpui-fast --example showcase-gpui`
      > fails these five, which pass on gpui-pre 0.3.9 and on gpui-fast with the
      > line:
      >
      > - `the_dragged_width_survives_the_toggle`: "View > Toggle Side Panel did
      >   not hide the side panel", left `Some(Bounds { origin: Point { x: 0px,
      >   y: 72px }, size: Size { 330px × 614px } })`, right `None`
      > - `toggling_the_side_panel_mid_drag_is_safe`: "Ctrl+B did not hide the
      >   side panel", left `Some(Bounds { origin: Point { x: 0px, y: 72px },
      >   size: Size { 321px × 614px } })`, right `None`
      > - `the_inspectors_theme_tab_lays_out`: "the Theme tab still shows the
      >   Widget tab's title", left `Some(Bounds { origin: Point { x: 6px,
      >   y: 307px }, size: Size { 128px × 18px } })`, right `None`
      > - `the_status_bar_names_the_hovered_widget`: "the status bar drew None,
      >   and its hover label was laid out at Some(Bounds { origin: Point {
      >   x: 1194px, y: 8975px }, size: Size { 72px × 18px } })" (the frame drew
      >   no label, yet its bounds are found)
      > - `under_client_decorations_the_title_bar_holds_the_menus`: "a menu-bar
      >   row is drawn beside a title bar", left `Some(Bounds { origin: Point {
      >   x: 0px, y: 0px }, size: Size { 1280px × 26px } })`, right `None`

      *Closed by G3 T4 (2026-10-22, c3c1cef9), not filed:* gpui-fast 0.1.9 fixed it
      (its release notes: "remove debug bounds for elements no longer drawn", PR #74;
      the notes list it with #75, which aligns composited scroll content's bounds);
      0.1.10's `finish_retained_frame` (`src/fast/retained.rs:1372-1385`) copies
      no debug bounds forward, and `reuse_paint` copies the reused range's only
      (`src/window.rs:4134-4143`). Both draw helpers call `window.draw(cx)` alone; the five
      tests above pass on GPUI Fast without the `refresh()`. Three other tests had leaned on
      its full frame and no longer do (the controller's ruling, spec §2.3): the checkbox
      harness's `on_change` notifies its view, the loading bar's test delivers the next
      frame through `Window::simulate_next_frame`, and
      `a_target_a_recordless_frame_took_away_does_not_win_when_drawn_again` draws the full
      frame spec §2.4 asks for with its own `window.refresh()`.

- [x] [upstream] **File the gpui-kit issue: on gpui-pre a labelled `Separator`
      draws its label at the line's end.** For the maintainer to file on
      longbridge/gpui-kit (an agent files nothing). gpui-component 0.8.0's
      `Separator` lays its label out in a `justify_center` row, the label
      `mx_auto` and the line an absolute child (`src/separator.rs:79-80`,
      `:136`, `:147`). gpui-pre 0.3.9 takes taffy `0.13.0`, whose
      `distribute_remaining_free_space` gives the free space to the auto
      margins (`src/compute/flexbox.rs:1721`) and then offsets the item by the
      same free space for `justify-content` (`:1750`), so the label moves by
      the free space twice. taffy 0.14.0, which gpui-fast 0.1.10 pins
      (`=0.14.0`), sets the free space to zero once the margins have taken it
      (`:2140`), as CSS does, so GPUI Fast centres the label. gpui-component
      pins `gpui-pre = "=0.3.9"`, so no gpui-pre build can take taffy 0.14.
      Our repro: the gpui showcase's Layout page, at rest on gpui-pre, draws
      "Section Break" at content x 1169–1249 on a 938 px line that starts at
      311, where GPUI Fast draws it at 740–820 (G3 Task 9's captures,
      `docs/archive/todo_v0.7.0_code-review-findings.md`, *G3*). The screenshot scripts
      build the showcase on gpui-pre, so its screenshots show the label at
      the end. Found 2026-10-22 (G3 Task 9). The issue, ready to paste:

      > **Title:** Separator: on gpui-pre the label is drawn at the line's
      > end, not centred
      >
      > gpui-component 0.8.0 on its default core, gpui-pre 0.3.9 (taffy
      > 0.13.0), Linux.
      >
      > `Separator::horizontal().label("Section Break")` draws the label
      > flush with the line's right end. With the `gpui-fast` feature
      > (gpui-fast 0.1.10, taffy `=0.14.0`) the same separator centres it.
      >
      > The separator's row is `justify_center()` and the label `mx_auto()`
      > (`src/separator.rs:136`, `:147`). taffy 0.13.0's
      > `distribute_remaining_free_space` (`src/compute/flexbox.rs`) gives
      > the line's free space to the auto margins (`:1721`) and then, for
      > `justify-content`, offsets the item by the same free space again
      > (`:1750`): on a 938 px line with an 80 px label the label starts
      > 858 px in, where centred it starts 429 px in. taffy 0.14.0 sets the
      > free space to zero once auto margins have taken it (`:2140`), as CSS
      > does.
      >
      > Either change fixes it on gpui-pre: gpui-pre on taffy 0.14, or the
      > label without `mx_auto()`, which `justify_center()` centres on both
      > cores.

      *Closed by G4 T6 (2026-10-27, 26186e33), not filed:* gpui-pre 0.3.10 lays out on
      taffy 0.15.0, whose `distribute_remaining_free_space` leaves no free space for
      `justify-content` once the auto margins have taken it ("The auto margins have
      absorbed all of the free space, leaving none for `justify-content`", taffy
      0.15.0 `src/compute/flexbox.rs:2274-2275`, 0.15.1 `:2320-2321`), so gpui-pre
      centres the label as GPUI Fast does. The showcase test
      `the_labelled_separator_centres_its_label` finds the label's box, the one quad
      inside the Separator filled with `tokens.background` and narrower than it
      (gpui-component 0.8.1 `separator.rs:149`), and holds its centre within a pixel
      of the Separator's on both cores: in the test window, at scale 2, the Separator
      spans x 311–1270 and the label 745–836, both centred at 790.5, on gpui-pre and
      on GPUI Fast. The Linux screenshot scripts capture the Basic page, so none of
      them showed the label.

- [x] [v0.7.0] **Regenerate the gpui screenshots and review the showcase's visible
      changes.** The v0.5.9 showcase-app plan (Tasks 1–25) changed what the
      gpui showcase draws, and nobody has looked at it on screen: the plan's
      tasks lay it out headlessly and never ran
      `scripts/generate_screenshots_gpui.sh`, which is maintainer-run because
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
        (Since 2026-09-27 the scripts' shared helper, `scripts/capture_window.sh`
        (then `capture_size.sh`), fails a capture no larger than the window's
        content, so one without the frame fails the script. Since 2026-09-28 it
        also makes the showcase's
        window the active one before capturing, and fails unless the window's
        content appears pixel for pixel inside the capture.)
      - **The layout.** Two panels: the side panel (`LEFT_PANEL_WIDTH`
        300) holds the Theme, Mode and Icon theme rows, a Separator and the
        inspector; the content panel has a TabBar of the pages above the
        page. The window is 1280 × 720 (1180 × 850 until 2026-09-27). The status bar has one toggle, at
        its left end. The Layout page shows an expanded and a collapsed
        `Sidebar`.

      *GPUI Kit 0.8.1 (2026-10-27, G4):* the screenshots the release asset run
      changes. Every committed gpui, iced and egui screenshot is still
      v0.6.1's (`docs/assets/PROVENANCE.toml`, commit `8db3da0e`), and each
      Linux capture of G4's parity run differed from the committed PNG, so the
      run replaces all of them; material-3's are written as new
      `linux-material-3-*` files, since the preset was renamed and the
      committed ones are still `linux-material-*`. Of what G4 itself changed:
      - The Layout page's labelled Separator now has its label centred on the
        line on gpui-pre as well (gpui-pre 0.3.10 lays out on taffy 0.15).
        No screenshot shows it: every capture path passes `--tab basic`.
      - The parity run moved nothing against the iced and egui showcases but
        six status-bar keys that differed at M9x and agree now: material-3's
        status bar y and height and its toggle's x and y, and
        catppuccin-mocha's toggle x and y.
      - The ColorPicker's hex text, where a channel rounds differently
        (gpui-base 0.8.1 rounds each channel, `color_picker.rs:71`), is on the
        Inputs page, which no screenshot shows either.
      *Closed 2026-10-27 by `b933efe0` (the v0.7.0 release asset run, captured from
      `bcde8141`):* every gpui, iced and egui screenshot regenerated, 12 per toolkit,
      and looked at, each Linux one beside its iced and egui counterpart: the KWin
      frame is in every Linux capture, the title reads "native-theme-gpui 0.7.0
      showcase", the parity step found 887 differences, all excepted. The gpui
      showcase's other ten pages were captured under kde-breeze light and looked at:
      the labelled Separator's label is centred, the Layout page shows the expanded
      and the collapsed `Sidebar` and the TitleBar sample, the masked label masks only
      the secret, the Icons page has its Icon Sizes section, the Textarea keeps its
      90px. What only opens on input (the
      Dialog, the two Sheets, the Popover, the HoverCard, the menus' hover colours)
      was not opened on screen; that look is the open `[v0.7.x]` entry *gpui
      showcase: look at the overlays that open on input*.

- [x] [1.0] **Finish auditing the Widget Info "Not themeable" entries for
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
      (already in "inner geometry exposed", now under *Upstream PRs to
      gpui-component*, "PR: expose checkbox/radio indicator size as theme
      token") and the notes now say so
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
      the platform's body text.
      0.7.1 (2026-10): no longer true of a Medium Button label —
      `button_text_size` is `text_sm` at Medium (`sizing.rs:337-343`); see the
      `Size::Size(px)` item below and `geometry::button_label` (v0.6.1).
      The `Headings` panel listed px figures that
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
  *Closed 2026-10-29 (0.8.0 M0 triage):* done: the item's own eighth pass (2026-09-22) records the audit complete, every "Not themeable" entry, colour claim and config line read against the source that paints it.

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
- [x] [v0.7.0] `tokens.tab`, `tokens.list`, `sidebar_primary` and
      `sidebar_primary_foreground` are read by nothing in 0.6.4 outside
      `theme/` (an idle tab is `transparent`, `tab/tab.rs:132-160`). The
      connector maps them anyway, and the contract marks pairs over them as
      inert so they are not counted as coverage.
      M8 T21 fix round 1 (2026-10-17), the duplicate rule: the lines of the entry
      "`ThemeColor::tab` and `ThemeColor::list_even` are slots nothing paints"
      (479e7284). Tagged [v0.7.0], that entry's tag, not [upstream]: its second
      remedy, the contract recording the slots as write-only, is the connector's
      own:
      `ThemeColor::tab` and `ThemeColor::list_even` are slots nothing paints.
      The connector writes both on every `apply` and both have contract rows
      (`contract.rs:420`, `:354`), but no `theme().tab` is read anywhere in
      gpui-component or gpui-base (declared `theme_color.rs:261`, defaulted
      `schema.rs:996`), and `list_even` is read only as `table_even`'s
      fallback (`schema.rs:1005`) — which never fires, because the connector
      writes `table_even` too (`contract.rs:391`). Every gate is green on
      both: they are written, so no coverage check notices, and they are read
      nowhere, so no widget can disagree with them. Either upstream should be
      asked to read them or the contract should record them as write-only.
      M8 T21 fix round 1 (2026-10-17): the list half overlaps
      "**`geometry::list` could paint the list.**" (*Follow-ups from the
      v0.5.9 showcase and contract work*), which would give
      `list.background_color` a receiver.
  *Closed 2026-10-26 in c77b6f77 (D4):* the second remedy. The contract keeps the five rows, the true mapping, so the day upstream reads a slot it carries the theme's colour, and names the slots in `WRITE_ONLY` (`connectors/native-theme-gpui/src/contract.rs`), each with its gpui-component 0.8.0 evidence (an idle tab `transparent` in every variant, `tab/tab.rs:173`-`:212`; `list` read only as the schema fallback of `list_even`, `list_head` and `table`, `theme/schema.rs:971-972`, `:1006`; `list_even` only as `table_even`'s, `:1009`; the two sidebar slots read nowhere): the coverage tripwire prints them and leaves them out of the coverage it counts, and `every_write_only_slot_is_a_row_and_not_coverage` holds each to one plain row that is not counted. The two list pairs leave `INERT_SURFACES`: their surface is now `geometry::list`'s fill, which is painted. Asking upstream to read the slots is the `[upstream]` entry "gpui-component reads `tokens.tab` …" in its place.
- [x] [v0.7.0] the gpui connector reads `AccessibilityPreferences::high_contrast`
      nowhere but its accessor (`native-theme-gpui` `lib.rs:615-617`,
      `is_high_contrast`): `to_theme` and `apply` build the same theme with it
      set or unset, so the showcase's Preferences toggle changes only the
      stored value and the status bar's flag. Decide what it should change
      (a high-contrast preset, or the contrast rule's thresholds) or say in
      the connector's docs that it is carried and not applied.
      For `adwaita` the platform's rules are known (M2e, 2026-10-11):
      libadwaita 1.10.0 applies high contrast as `prefers-contrast: more`
      (`adw-style-manager.c:270-284`): `--border-opacity` 50 %,
      `--dim-opacity` 90 %, `--disabled-opacity` 40 % (`_colors.scss:323-329`)
      and per-widget rules in 32 stylesheet files. The GNOME reader carries
      the flag (portal `contrast`, else `org.gnome.desktop.a11y.interface
      high-contrast`, in `native-theme/src/gnome/mod.rs`'s
      `accessibility_from_gnome_data` and `build_gnome_variant`);
      no preset or connector applies it.
  *Closed 2026-10-26 in c77b6f77 (D4):* carried, not applied, by design. High contrast belongs in the theme data, which a reader fills with what the platform draws under it. The Windows reader writes the high-contrast system colours (`apply_sys_colors`), the GNOME reader libadwaita's high-contrast focus ring, and the macOS reader's system colours follow Increase Contrast (`docs/evidence/os-probe/2026-10-26-run-40320895749/os-probe-macos-14/increase-contrast-on/probe.toml`). A connector-side transform would invent behaviour no platform states, and on Windows apply it twice; no platform has a high-contrast preset beyond Windows' system colours, which the reader reads live. The three connectors' docs say so in one word-identical row. The rest of libadwaita's rules move to the `[1.0]` entry *The GNOME reader applies libadwaita's high-contrast rules*.

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
- [x] [v0.7.0] Make the layout-dump gate deterministic: `--dump-layout` records the
      Widget Info and status-bar readout of the element under the pointer
      (`chrome.info.*`, `chrome.status_bar.shown`), so a before/after
      comparison differs with where the pointer sat (v0.6.1 had to filter
      those keys); park the pointer outside the window during a dump, or
      leave those keys out of the dump.
  *Closed by M8 T12 (2026-10-17, fc2fa811):* the dump ignores the pointer
      (Step 6 ruling): under `--dump-layout` each showcase keeps the window's
      own pointer off its widgets with its capture shield (gpui's and iced's
      `CliArgs::shields_pointer`, egui's `CliArgs::shields_pointer` read by
      `raw_input_hook`), so the keys stay in the list and the comparator, the
      Widget Info rows hold their hint and `chrome.status_bar.shown` is not
      drawn; `--pointer X,Y` still holds a pointer of its own. Each showcase's
      `a_layout_dump_is_the_same_under_the_pointer` checks it. Two live dumps
      of each showcase are byte-identical. The release asset run now compares
      the three at rest (c20daadc).
  Archived 2026-10-17 (M8 T21): ticked in 70ead29f.

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
- [x] [1.0] **The showcase's pages: the application's own values, as the chrome
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
      *Closed by M9a T11 (2026-10-18, 73ded48d, bc9ed52c):* the pages apply
      `window_margin`, `section_gap` and `widget_gap` as the Basic page does;
      where the theme states none they draw none
      (`docs/todo_code-review-findings.md`, *Step 6 — fix plan, M9a
      (2026-10-17)*, "Ruling: the pages (M9-022)"), so no page constant is
      added; the chrome's §3.1 constants stay. The distances of another role
      the pages keep (the Icons page's icon grids, the room beside the Layout
      page's Carousel, the Theme Map's swatch gaps) are named showcase
      constants, each with its comment and its Widget Info line. windows-11's
      unsourced `widget_gap` stays in the *Follow-up plan* table.
- [x] [v0.7.0] **Carry `defaults.line_height` to the code editor.** `Editor` sets
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
  *Closed 2026-10-26 in c77b6f77 (D4):* `geometry::editor(n)`, `defaults.line_height` and nothing else, over the `relative(1.5)` upstream's `Editor` sets on its `Input` first (gpui-component 0.8.0 `src/input/editor.rs:157`), which refines the caller's style last (`:174`); its font is already the theme's mono font (`:155-156`). The gpui showcase's `demo::editor` applies it and its Widget Info gives the builder's line. `editor_carries_the_platforms_line_height_alone` checks it over every preset and mode, and the showcase's `the_code_editor_is_refined_by_geometry_editor` the Editor it builds.
- [x] [v0.7.0] **`geometry::list` could paint the list.** `ThemeColor::list` has no
      reader (the contract says so at `contract.rs:1784`) and a `ListItem`
      paints no idle background, so `list.background_color` reaches nothing.
      On the bundled presets it falls back to the window background
      (`resolve/inheritance.rs:143-144`), so nothing is lost there; on a live
      KDE system it is `[Colors:View] BackgroundNormal` (`kde/colors.rs:91`),
      the view background rather than the window's, and that is dropped. `geometry::list` lands on the element around the rows, which
      is exactly where a fill belongs, and carries only the border
      (`geometry.rs:362-369`). One `.bg()` would carry it. Found 2026-09-22.
  *Closed 2026-10-26 in c77b6f77 (D4):* `geometry::list` fills the list view with `list.background_color` (`list_paints_the_list_background`, every preset and mode): neither a `List`, a `Tree` nor a `ListItem` at rest paints a fill (gpui-component 0.8.0 `src/list/list_item.rs:225-233`, `:257-265`). The gpui showcase's List and Tree drop the fill they painted by hand, the same colour, and the leaf route names the builder; the contract's two list pairs take the builder's fill as their surface.
- [x] [v0.7.0] **`Switch::color` is a receiver nothing feeds.** `switch.checked_background`
      is modelled and every preset states it; `ThemeColor` has no field for
      it, which is what `colors.rs`'s Issue 51 note and the contract's
      `NoReceiver` entry record -- but `Switch::color` (`switch.rs:95`) takes
      it per instance. A helper, or a `variants::` function like
      `ghost_button`, would reach every switch an application builds. Found
      2026-09-22.
      M6 T16c (2026-10-16, 12be97dd): with `widgets` the leaf reaches
      `SwitchLook` (the connector's own switch draws it), so the contract's
      `NoReceiver` entry for it exists only without `widgets`.
  *Closed 2026-10-26 in c77b6f77 (D4):* `variants::switch_color(n)`, `switch.checked_background` from the `Native` view the geometry builders take, for `Switch::color` (gpui-component 0.8.0 `src/switch.rs:101-106`; the `primary` token otherwise, `:181-184`). The gpui showcase's gpui-component switches use it; the contract's `NoReceiver` entry, which stays without `widgets` because a `Switch` built without `.color` drops the value, and `colors.rs`'s Issue 51 note name it. `switch_color_is_the_platforms_checked_track` checks it over every preset and mode, and that it differs from `primary` somewhere.
- [x] [v0.7.0] **The code editor's background is a fixed colour.** `Theme::editor_background`
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
  *Closed 2026-10-26 in c77b6f77 (D4):* the highlight theme the connector installs, in `to_theme` and in the `ThemeConfig` that `Theme::change` reads, is upstream's default for the mode with `editor_background` = `input.background_color`, the field's fill, as `geometry::input_fill` gives a text field (a code editor is an `Input` in code mode, gpui-component 0.8.0 `src/input/input.rs:720-722`), and `editor_gutter_background` that fill over `window.background_color`, opaque wherever the window is, as gpui-base wants the gutter over the text scrolling under it (`src/input/base/element.rs:3352-3358`; windows-11's field fill is translucent). Clearing `editor_background` would have landed on `input_background()`, the border token mixed toward transparent in dark (`src/theme/mod.rs:485-499`). The current line, the line numbers, the invisibles and the syntax colours stay upstream's scheme: the model states none of them (`config::highlight_style`). `the_code_editor_takes_the_platforms_field_fill` checks both fills over every preset and mode, and the Editor's Widget Info claims them.
- [x] [v0.7.0] **The iced connector reads nothing of the model's toolbar.** The gpui
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
      Named in the iced colour-leaf coverage test (M5 T12b, 9c46d834).
      M9b T19 (2026-10-22): the showcase half is not open, and "the iced
      showcase draws no toolbar" no longer holds: the iced showcase has drawn
      a toolbar row since the v0.5.9 chrome (`toolbar` in
      `connectors/native-theme-iced/examples/showcase-iced.rs`), which reads
      `resolved.toolbar` itself: `icon_size`, `background_color`, the
      `toolbar.border` padding, line width and colour, `item_gap` and
      `bar_height`. The connector half stays open: nothing under
      `connectors/native-theme-iced/src/` reads `resolved.toolbar` yet, and a
      reader is a connector API, which M9b leaves to this entry (M9b's fix
      plan, the ruling on what leaves M9b, item 5).
      M9b final fix (2026-10-22): the showcase's reads include
      `toolbar.font` too, the M9b T19 line left it out: `toolbar` hands
      `font: &t.font` to every item, and the bar's `Surface::Toolbar` takes
      `toolbar.font.color` as its text colour.
  *Closed 2026-10-26 in 2eca456a (D5):* the connector half, in the shape C-01 gave the dialog: `styles::toolbar(&resolved)`, the container a consumer draws its toolbar in (fill `toolbar.background_color`, text `toolbar.font.color`, iced's border, shadow and snap: the edge is the rule under the bar), `styles::toolbar_rule(&resolved)`, that rule in `toolbar.border.color` (the edge gpui's `geometry::toolbar` draws as `border_b`), and the crate-root `toolbar_padding(&resolved, default)`, `toolbar_item_gap`, `toolbar_bar_height` (the outer height, the rule included; `None` where the platform states none, KDE and GNOME, docs/platform-facts.md §2.13) and `toolbar_border_line_width`. Contract rows for the three colours; the toolbar's `UNREAD_COLOURS` entry is gone and `contract-leaves.toml` moves the three leaves to the new rows. The iced showcase's toolbar calls the API (`Surface::Toolbar` and `Line::Toolbar` go) at the same values, so its dump is unchanged; its fallbacks (`layout.container_margin`, `TOOLBAR_PADDING`, `TOOLBAR_GAP`) stay the showcase's. The toolbar font stays the showcase's to hand its items: plan §1 for 3417 gives the connector the colours and the geometry, not the font.

### Closed items from Follow-ups from the v0.5.9 showcase and contract work

- [x] [1.0] **Markdown headings are sized from a fixed 14px.** gpui-base draws a
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
      *Closed by gpui-component 0.7.3 (2026-10-14):* fixed upstream. gpui-base
      0.7.3 has no `heading_base_font_size`: a heading is the body size
      times its level's ratio, 1.8, 4⁄3, 17⁄15, 1, 14⁄15 and 13⁄15, every level
      semibold (`gpui-base text/node.rs:2867-2876, 2883-2888`), and the body
      size follows the rem the connector sets from `defaults.font.size`.
      Headings keep TextView's ratios rather than the platform's
      `text_scale` roles: editors size their headings their own way, not as
      the OS sizes its titles (the maintainer's decision for v0.7.0), and
      gpui-component builds TextView's defaults from the theme itself
      (`text/mod.rs:35-62`), so the connector installs nothing. It moves to
      the archive at the next prune.
  Archived 2026-10-17 (M8 T21): ticked in c0fa1b02.

### Connectors: `native` and `watch` passthrough features in gpui and egui

- [x] [v0.7.0] For M6/M7 (ruling M5-02, 2026-10-14): the iced connector
      forwards native-theme's readers and watcher as two features, both off
      by default as in core, `native = ["native-theme/native"]` and
      `watch = ["native-theme/watch", "dep:iced_futures"]`
      (`connectors/native-theme-iced/Cargo.toml:42-46`; M5 T3, 80b9ad13),
      and its README's Quick start and Features sentence name them. The gpui
      connector has neither: every build turns on native-theme's `linux`,
      `macos` or `windows` through a per-target dependency
      (`connectors/native-theme-gpui/Cargo.toml:81-88`), and `watch` is on
      for its dev-dependencies only (`:96`). The egui connector forwards
      `watch` (`connectors/native-theme-egui/Cargo.toml:33`) but has no
      `native`, and turns the readers on per target the same way (`:46-51`).
      Give both iced's two passthroughs and name them in their READMEs. In
      both, an off-by-default `native` in place of the per-target readers
      means a default build no longer reads the OS, a change their
      CHANGELOGs state.
      gpui done (M6 T3, 2026-10-15): `native = ["native-theme/native"]` and
      `watch = ["native-theme/watch", "dep:futures"]` in place of the
      per-target reader lines, the dev-dependencies' native-theme with
      `native` and `watch`, the README's Quick start, Features table and
      sentence, the CHANGELOG's *Breaking Changes*; `check_release.sh`, CI,
      the crates.io workflow, the canary and `update_compatibility.sh` build
      the connector with `native,watch`. egui stays (M7).
      egui done (M7 T3, 2026-10-16): `native = ["native-theme/native"]` in
      place of the per-target reader lines, beside the `watch` it had; the
      dev-dependencies' native-theme with `native` and `watch`; the widgets
      crate forwards both (M7-33); the README's Quick start, Features table
      and sentence, the widgets README's Features sentence, the CHANGELOG's
      *Breaking Changes*. The egui legs of `check_release.sh`, CI, the
      crates.io workflow, the canary and `update_compatibility.sh` build both
      crates with `--all-features`, and `check_features.sh` each feature
      alone, so no gate line changed.
  Archived 2026-10-17 (M8 T21): ticked in 36620c8e.

### READMEs: the dependency-version check beyond iced

- [x] [v0.7.0] For M6/M7/M8 (ruling M5-20, 2026-10-14): the iced README's
      `toml` blocks are checked against the workspace's minor by
      `the_readme_depends_on_this_release`
      (`connectors/native-theme-iced/src/compat.rs:311`; M5 T11, b0efe9e3),
      so a release cannot publish a README that names the previous minor.
      The same literals stand unchecked in the gpui README
      (`native-theme = "0.6"`, `native-theme-gpui = "0.6"`,
      `connectors/native-theme-gpui/README.md:104-105`), the egui README
      (`native-theme-egui = "0.6"`,
      `connectors/native-theme-egui/README.md:124`), the core README
      (`native-theme = { version = "0.6", features = ["native"] }`,
      `native-theme/README.md:180`) and the build README
      (`native-theme-build = "0.6"`, `native-theme-build/README.md:30`).
      Give each crate the same check: gpui in M6, egui in M7, core and build
      in M8.
      gpui done (M6 T3, 2026-10-15): `the_readme_depends_on_this_release`
      and its parser test in `connectors/native-theme-gpui/src/compat.rs`,
      beside the floor checks; the README's two `toml` blocks (the Quick
      start, *Runtime theme change*) name 0.6 until the 0.7.0 bump, which
      the test then fails until they follow. egui, core and build stay.
      egui (M7 T3, 2026-10-16): no third copy of the check (the M6 T3
      ruling); the egui README's Quick start and the widgets README's new one
      name 0.7, the coming release's minor, and M8's one workspace check (the
      *For M8* entry on the README check) takes both READMEs with core's and
      build's.
  *Closed by M8 T9 (2026-10-17, 9246c7de):* one workspace check,
      `scripts/check_readme_versions.py`, holds every README's `toml` blocks
      (the root's, core's, build's, the three connectors' and egui-widgets')
      to one rule: each `native-theme` and `native-theme-*` dependency names
      the workspace version's `major.minor`, so the release bump moves every
      README at once and the check holds them there (the Step 6 amendment,
      no rule predicting the next version from the CHANGELOG). The egui and
      egui-widgets READMEs name the workspace's 0.6 again, not 0.7 as M7 T3
      wrote them; the gpui, iced, core and build READMEs name 0.6 until the
      0.7.0 bump. `scripts/check_release.sh` (both modes), ci.yml's
      documentation job and publish.yml run it.
      2026-10-17 (M8 T11): publish.yml no longer runs it itself; it calls
      ci.yml, whose documentation job runs it, on the tag's commit.
  Archived 2026-10-17 (M8 T21): ticked in 304544ed.

### Closed items from Connectors: a public theme builder that takes the mode

- [x] [v0.7.0] All three connectors (M5 T4 ruling, 2026-10-14): iced's
      public `to_theme(&resolved, name)` leaves `Extended::is_dark`, which
      iced uses for the window title bar, to iced's lightness test of the
      background, because a `ResolvedTheme` carries no mode
      (`connectors/native-theme-iced/src/lib.rs:317-328`); `from_preset`,
      `from_system` and `SystemThemeExt::to_iced_theme` pass the mode to the
      private `to_theme_in` (`:330-335`). An application that resolves a
      theme itself, as the iced showcase's `load_adwaita_fallback` does
      (`connectors/native-theme-iced/examples/showcase-iced.rs:1152-1168`,
      `to_theme` at `:1162`), gets the guess. Nothing differs today: no
      bundled preset is misjudged by the test (the M3-15 ruling), and
      adwaita's backgrounds agree with their modes. Decide on a public
      builder that takes the mode, which the showcase's fallback would then
      use. In the siblings: gpui's `to_theme` takes `is_dark: bool` already
      (`connectors/native-theme-gpui/src/lib.rs:213-219`), where M1-T7-06's
      `Loaded` carries `mode: ColorMode`; egui's `to_theme` compiles one
      variant into both of egui's styles
      (`connectors/native-theme-egui/src/ext.rs:134-137`), and
      `ThemeAtlas::builder` takes the two variants and `Builder::os_mode`
      (`connectors/native-theme-egui/src/atlas.rs:599`).
  *Closed by M8 T4 (2026-10-16, 8d0e029f):* iced's `to_theme_in(resolved, name, mode: ColorMode)` is public, `to_theme` calls the private `Option` form and its doc points at `to_theme_in`, and the showcase's `load_adwaita_fallback` builds its theme with it in the mode it resolved (`Resolved::mode`). gpui's `to_theme(.., is_dark: bool, ..)` stays (the line below); egui needs nothing (`Builder::os_mode` takes both variants).
  Archived 2026-10-17 (M8 T21): ticked in b1fcb373; its "above", "below" and "next entry" mean `docs/todo.md` at 6980aa05.

### Contracts: the connectors' widgets

- [x] [v0.7.0] A design question for all connectors (M5 T7 ruling,
      2026-10-14): the iced contract covers every palette slot the
      connector writes and every field of a `styles::*` output
      (`connectors/native-theme-iced/src/contract.rs:1-3`), so a widget of
      the connector's own is outside it. iced's `Spinner` draws a
      freedesktop indicator's `currentColor` in `defaults.text_color` and a
      Material or Lucide one in `spinner.fill_color`
      (`connectors/native-theme-iced/src/spinner.rs:126-132`), and only its
      unit test `a_symbolic_freedesktop_indicator_is_drawn_in_the_text_colour`
      (`:359`) checks the first. Decide whether a contract also rows a
      widget's reads, in iced (`Spinner`), gpui (the theme-drawn widgets of
      its `widgets` feature, `connectors/native-theme-gpui/Cargo.toml:32-35`)
      and native-theme-egui-widgets, or a widget's unit tests stay its only
      check.
  *Closed by M8 T7 (2026-10-16, c94f1a1b, c645b2d5, 637f8fef):* a connector's own widget is a sink of a colour-leaf coverage test, with no second contract (Step 6 ruling), in all three. iced: the `Spinner` reads the theme through `spinner::Look::of`, and two accessor rows read it (`spinner.fill_color`, the arc and a bundled set's tint; `defaults.text_color`, a freedesktop indicator's `currentColor`), which `every_colour_leaf_reaches_a_row_or_is_named` requires each leaf to reach and its splice drift check holds to the leaf. gpui: the seven `widgets` Looks were sinks of its coverage test already (M6 T16). egui: the ruling's premise that native-theme-egui-widgets reads only the atlas's cells was wrong; it reads leaves from `resolved_for` too, and since c645b2d5 each widget reads its colours through one `Look` (the spinner, switch, slider, links, progress bar, radio button, segmented control and painted expander; the drop-down reads none; since 637f8fef the spinner's carries `defaults.text_color` to a freedesktop indicator as its icon key's tint, where the connector's `load_color` read it), which `each_widget_draws_in_the_colour_leaves_it_lists` (the crate's `tests.rs`) splices every colour leaf against in all 34 combinations: a widget whose `Look` moves under a leaf its `WIDGET_READS` row does not list fails it, as does a listed leaf that moves it nowhere, and `every_look_is_read_by_the_coverage_test` requires every `Look` the crate defines to be spliced. The one colour read the egui widgets make outside a `Look` is the expander's arrow over egui's own header, drawn by the connector's `expander_icon` and rowed by `connectors/native-theme-egui/mapping.toml:2727-2729` (`expander.arrow_color`, `tested_by = "T18(f)"`). The atlas's cells stay `mapping.toml`'s rows. In all three a read added outside the widget's Look later would not be seen: the widgets still take the whole `ResolvedTheme`.
  Archived 2026-10-17 (M8 T21): ticked in 80c6cf02.

---

## Publishing Prep

- [x] Publish to crates.io (v0.5.8 released 2026-09-07, tag `v0.5.8`)

---

## This cycle (v0.7.0)

### egui showcase: its gpui-component citations miss the code they name

- [x] [v0.7.0] For M9c (G3 Task 3b, 2026-10-21): four `GC/` citations in the egui showcase
      point at other code, at gpui-component 0.7.3 already and at 0.8.0, which the
      workspace builds now. Their 0.8.0 lines, read in the registry's gpui-component 0.8.0:
      - `connectors/native-theme-egui/examples/showcase-egui/chrome.rs`, `page_tabs`' doc,
        the tab bar's page menu: `GC/tab/tab_bar.rs:554-584` (the tab row) is `:699-732`,
        `TabBar::render`'s Ghost `menu` Button, the selected tab's item `checked`.
      - The same file, the doc of the dialog title row: `GC/dialog/dialog.rs:696-701` is
        `:749-756` (the `DialogTitle`) and `:789-804` (the Ghost close button,
        `IconName::Close`).
      - `connectors/native-theme-egui/examples/showcase-egui/demo.rs`, `GHOST_NOTE` and the
        `ghost` scope's doc: `GC/button/button.rs:973, :1063` is `:1008` (`bg_color`'s
        `Ghost` arm, `transparent`) and `:1098` (`border_color`'s).
  *Closed by M9c T1 fix round 1 (2026-10-23, f95b7f23):* the four citations name the
      locked gpui-component 0.8.0 by version and path at the lines above, each read back:
      `page_tabs`' doc `src/tab/tab_bar.rs:699-732`; the dialog title row's
      `src/dialog/dialog.rs:749-756` and `:789-804`; `GHOST_NOTE` and the `ghost` scope's
      doc `src/button/button.rs:1008` and `:1098`.

### Showcases: the strict panic lints do not reach the showcase examples

- [x] [v0.7.0] `scripts/check_release.sh` and `.github/workflows/ci.yml` run
      `scripts/check_strict_panic.sh` on each connector with `--lib`
      (check_release.sh, *strict-panic (native-theme-gpui, native, watch)*),
      so the three showcase examples get clippy's default lints alone, where
      the M9 constraints hold them to the strict ones. M9a's final fix
      (2026-10-18) ran it over the gpui showcase:
      `bash scripts/check_strict_panic.sh -p native-theme-gpui --example showcase-gpui --features native,watch`
      finds 45: 42 `arithmetic_side_effects`, 2 `indexing_slicing` and 1
      `integer_division`, in demo.rs (17), app.rs (6), info/layout.rs (5),
      inspector.rs (4), main.rs (3), pages/data.rs and pages/layout.rs (2
      each), and info/feedback.rs, pages/basic.rs, pages/feedback.rs,
      pages/mod.rs, speech.rs and support.rs (1 each). Some of the arithmetic
      is on gpui's `Pixels`, an `f32` whose operators do not panic, the rest
      on integers and one `Duration` sum (main.rs, `held`). Fix them, then
      add the example's line to check_release.sh's strict-panic section and
      to ci.yml's strict-panic step, beside the connector's. M9b's iced
      showcase (`--example showcase-iced`) and M9c's egui showcase
      (`--example showcase-egui`) need the same run.
      M9a T21 (2026-10-18): the gpui half is closed. `30c49fc1` fixes the
      45 (integer arithmetic saturating or checked, the `Duration` sum
      through `saturating_add`, the integer division through `div_euclid`,
      the two indexings through `Value::get` and a slice pattern, `Pixels`
      arithmetic in `f32` through `f32::from` and `px`, as the connector's
      widgets spell it), and `259a8146` adds the example's line, with
      `native,watch` and with `gpui-fast,native,watch`, to check_release.sh's
      strict-panic section and ci.yml's two gpui strict-panic steps. The
      iced and egui showcases' runs stay open, for M9b and M9c.
      M9b T1 (2026-10-20): the iced half is closed. The run found 40 at
      `c0d6d530` (21 `arithmetic_side_effects`, 17 `indexing_slicing`, 2
      `exit`); `1d57e42d` moved the showcase onto the shared crate, which
      took 19 of them with the code it replaced (the command-line parser,
      the hex parser, the exits through `cli::refuse`), and `f6ed4f75` fixes
      the other 21 (the `const` `Tab::ALL` check by slice pattern, counters
      saturating, the animation frames through a zip, `.get()` and
      `checked_rem`, the grids as slice chunks, the frame check's arithmetic
      saturating). `1bacf203` adds the example's line,
      with `iced_aw,watch,native`, to check_release.sh's strict-panic section
      and ci.yml's iced leg, and splits each connector's step in ci.yml so
      that the library and the showcase run as steps of their own. The egui
      showcase's run stays open, for M9c.
      M9c T1c (2026-10-23): the egui half is closed. The run found 13 at
      `1bea8cf5` with `--all-features` and with the default features, all
      `arithmetic_side_effects`: pages/basic.rs (7), demo.rs (3), chrome.rs
      (2) and pages/icons.rs (1); 12 with `--no-default-features` (the
      semibold registration compiles only with `system-fonts`), 12 for
      x86_64-apple-darwin (the Theme menu's frame is not compiled there) and
      13 for x86_64-pc-windows-msvc. `f183d015` fixes
      them (the 1-based row numbers and the registration count saturating;
      egui's `Rect`, `Margin` and `Vec2` operators spelled in `f32` as epaint
      and emath compute them, so nothing drawn moves), and `3d471f65` adds
      the example's lines, with `--all-features` and `--no-default-features`
      and with `--all-features` for the two targets, to check_release.sh's
      strict-panic section and ci.yml's egui leg, each a step of its own.
  *Closed by M9c T1c (2026-10-23, f183d015, 3d471f65):* the three showcase examples hold
      the strict panic lints, in check_release.sh's strict-panic section and ci.yml's
      gpui, iced and egui legs.

### iced showcase: Widget Info claims a substituted family
- [x] [v0.7.0] For M9b: the iced and gpui showcases name a family the system lacks by its stated name alone while it is drawn in the platform's substitute. iced: since 2fcc26fb the family the showcase asks for (`resolved_family`, `connectors/native-theme-iced/examples/showcase-iced.rs:12466-12477` at 2fcc26fb; since d91ff927 `font_from_database` in that file, through `native_theme_iced::font`) is `system_font_family`'s, which is the platform's substitute where the system lacks the stated family; `family_label` (`:12376-12386` at 2fcc26fb) returned the stated name alone wherever the drawn family is the one asked for, so `font_row_drawn` says "button.font, family, size and weight" for windows-11's "Segoe UI" drawn in fontconfig's match (Adwaita Sans on one Linux system); before 2fcc26fb the same row said "Segoe UI (not found; drawn in generic sans-serif: …)". gpui: the inspector's Theme tab (`connectors/native-theme-gpui/examples/showcase-gpui/inspector.rs:215-229`) shows the stated `font.family` under `font_family` and notes "drawn as" only for macOS's `.SystemUIFont`, while `Theme::font_family` holds `font_family`'s substitute (`connectors/native-theme-gpui/src/lib.rs:311`, `:747-750`), and its `mono_font_family` row has no note at all. Widget Info and the inspector must name the drawn family in both, e.g. "Segoe UI (not on this system; drawn in the platform's substitute, Adwaita Sans)". egui names it, in another place: its Theme panel's Notes section lists the atlas's `Note::FontFamilyUnavailable { family, substitute }` (`connectors/native-theme-egui/examples/showcase-egui/info.rs:1114-1121`), while its Fonts section just below (`:1129-1135`) prints the stated `font_family` and `mono_font_family` with no note; M9b makes the two sections agree.
      M8 T21 (2026-10-17): each half is its showcase's milestone: iced's
      Widget Info M9b's, gpui's inspector M9a's, egui's two Theme panel
      sections M9c's.
      M9a T4 (2026-10-18): gpui's status bar font item and its inspector's
      Theme tab `font_family` and `mono_font_family` rows name the drawn
      family through the shared crate's `status::family_label` (f3e058ee),
      e.g. on one Linux system windows-11's "Segoe UI (not on this system;
      drawn in the platform's substitute, Adwaita Sans)". gpui's listed Widget Info font
      rows still print the stated family: they wait for M9c T10's
      drawn-family argument to `values::leaf_value`. The entry stays open.
      M9b T12 (2026-10-22): iced's own font lines name the drawn family
      through the shared crate's `status::family_label` (d91ff927): the
      font rows (`font_row_drawn`), the Theme fonts lines
      (`widget_tooltip_themed`), the inspector's Theme tab and the Display
      page's font line, and the status bar's family item through
      `status::environment`, e.g. on one Linux system windows-11's "Segoe UI
      (not on this system; drawn in the platform's substitute, Adwaita
      Sans)". gpui's inspector half was M9a's (M9a T4, above). iced's listed
      Widget Info font rows still print the stated family: they wait for
      M9c T10's drawn-family argument to `values::leaf_value`. The entry
      stays open.
  *Closed by M9c T10 (2026-10-23, a02f1dd7, 337e0de0):* the egui showcase names the
      family egui draws (`demo::DrawnFamilies`, from the atlas's
      `Note::FontFamilyUnavailable`: its substitute, or egui's own face where it names
      none) in the status bar's font item (`status::environment_drawn`), the Theme
      tab's Fonts `font_family` and `mono_font_family` rows, which so agree with its
      Notes section, and Widget Info's whole-font and `*.font.family` rows, listed and
      `mapping.toml` alike; the three showcases' listed Widget Info font rows name it
      through the drawn-family lookup the shared crate's `values::leaf_value` and
      `info::ListedInfo::new` now take: gpui's `status::family_label` of the
      connector's `font_family`, iced's label of what it draws (`listed_family`),
      egui's `DrawnFamilies::font_label`; e.g. on one Linux system windows-11's
      "Segoe UI (not on this system; drawn in the platform's substitute, Adwaita
      Sans)". A host-conditioned test in each showcase reads the rows
      (`every_font_label_names_the_drawn_family`,
      `a_listed_font_row_names_the_family_iced_draws`,
      `a_listed_font_row_names_the_family_gpui_draws`).

### gpui showcase: a family the platform names no substitute for is labelled as stated

- [x] [v0.7.0] Found by the M9c T10 review (Minor 2, 2026-10-23): the gpui showcase labels a theme font's family by the family gpui is asked for, `native_theme_gpui::font_family` (`connectors/native-theme-gpui/src/lib.rs:752-761`), in Widget Info's listed font rows (`inspector::listed`'s lookup, `connectors/native-theme-gpui/examples/showcase-gpui/inspector.rs:567` at 79769ea9), the status bar's font item and the inspector's Theme tab (`font_rows`, M9a T4). Where the system lacks the family and the platform names no substitute, `font_family` returns the stated family: on macOS and Windows always (`native_theme::fonts::substitute_family` is `None` there, `native-theme/src/fonts.rs:286-288`, `platform_substitute` `:370-381`), and in a build without `system-fonts` (`connectors/native-theme-gpui/src/lib.rs:779-783`). gpui then draws the first font of its own fallback stack that resolves, `.ZedMono` first (gpui-pre 0.3.9 `src/text_system.rs:255-266`, `resolve_font` `:370-378`), so the three sites name the stated family alone while gpui draws another: adwaita on Windows reads "Adwaita Sans". Name what gpui draws: ask gpui's text system whether the family's `font_id` resolves, and where it does not, label gpui's fallback stack (a form of the shared crate's `status::DrawnFamily`), as the egui and iced showcases name their toolkit's own face.
  *Closed by M9c T10b (2026-10-24, 43b918d4, d1176bde):* the three sites name the family
      gpui's text system draws: `inspector::DrawnFamilies`, built once an install with the
      panel's look, asks `TextSystem::resolve_font` and `get_font_for_id` (gpui-pre 0.3.9
      `src/text_system.rs:370-388`, `:354-362`; gpui-fast 0.1.10 `:372-374`, `:356-364`) of
      each family gpui is asked for, and a family drawn under another is the shared crate's
      new `status::DrawnFamily::FallbackStack`: "Noto Sans (gpui finds no face of it; drawn
      in the first family of gpui's fallback stack it finds, DejaVu Sans)", or, where the
      platform's substitute is not found either, "NoSuchFamily-T10b (not on this system;
      gpui finds no face of the platform's substitute, Noto Sans; drawn in the first family
      of gpui's fallback stack it finds, DejaVu Sans)". A text system that lists no font
      (the test platform's `NoopTextSystem`) is not asked. Two tests run the showcase on a
      text system that holds the named families alone (`HeldFamilies`):
      `a_family_gpui_finds_no_face_of_is_named_by_its_fallback` and
      `a_family_gpui_finds_is_named_as_stated`.

### check_widget_coverage.py discovers no chart

- [x] [v0.7.0] gpui-component's charts derive `IntoPlot`, not `IntoElement`, so `scripts/check_widget_coverage.py` discovers none of them. Teach it `IntoPlot`, then show what it finds — today `RadarChart` and `SankeyChart`, which no showcase shows.
  *Closed 2026-10-26 in d2d4a3af and b1b8f095 (C-07):* the script discovers `#[derive(IntoPlot)]` (d2d4a3af), and RadarChart and SankeyChart are shown (b1b8f095).

### Presets: material-3's platform-facts citations have drifted

- [x] [v0.7.0] For M2d (found 2026-10-14 in G2 Task 10's preset audit). About 60 comment lines in each variant of `native-theme/src/presets/material-3.toml` cite a `docs/platform-facts.md:N` line 1–4 lines away from the row they describe: for example, the tooltip's `font.color` cites `:1280` (`font.weight`) where the row is `:1282`, and the list's `header_font` cites `:1425-1426` (the selection rows) where the rows are `:1427-1428`. G2 Task 10 fixed the four pairs beside its new fields (`a440e812`). The values are not affected; only the line numbers are. The other native presets have no such drift: their remaining mismatches are deliberate cross-row citations, such as a disabled colour citing `disabled_opacity` or an unchecked background citing the widget's `background_color`. Method: for each `# platform-facts.md:N` comment, compare the property name of row N (and of each row in a range) with the key on the next line, as `<sub-table>.<key>` without `_px`/`_ms`/`_pt`. M2d re-cites material-3 against the Compose release it pins anyway; a gate doing this check in `check_release.sh` would stop the drift from coming back.
      M8 T16 (2026-10-17): material-3's four §2.20 layout values
      (`material-3.toml` `[layout] widget_gap_px, container_margin_px, window_margin_px, section_gap_px`:
      6, 6, 10, 18) have no source: the
      Material cells are "(none)" (`docs/platform-facts.md` §2.20), and the
      preset-validator check found none. They join the four `layout.*` rows
      of *Follow-up plan: the other sizing fields a platform leaves
      unstated*; the preset keeps them, as they pad all three showcases'
      chrome.
      G3 T12 (2026-10-22): two pairs re-pointed by content, from the G3 Task 11
      review: the toolbar's `font.size` comments (`[light.toolbar.font]`,
      `[dark.toolbar.font]`) cited `:1384`, §2.13's table rule, and now cite
      `:1386`, the `font.size` row; the toolbar's `border` comments
      (`[light.toolbar.border]`, `[dark.toolbar.border]`) cited `:1394`,
      `background_color`, and now cite `:1397-1399`, the
      `border.corner_radius`, `border.line_width` and `border.shadow_enabled`
      rows. The rest of the entry stays open.
  *Closed 2026-10-26 in 7bea7d81 (C-08):* `scripts/check_preset_citations.py` is the gate (check_release.sh and CI); all 104 drifted material-3 lines are re-pointed by content (the dialog's `body_font.color` among them, which a whole-font row had hidden); the other presets' cross-row citations pass by stated rules, and one windows-11 citation by named exceptions since d296edf7, which re-pointed windows-11's two `active_background` comments to their own row.

### iced: an application style that reads `window.background_color`

- [x] [v0.7.0] iced paints an application's background from its base style, `theme::Style.background_color` (`iced_core-0.14.0/src/theme.rs:222-230`), which `Base::base` takes from the palette's background (`theme.rs:291-293`, `:328-336`), under this connector `defaults.background_color`; nothing reads `window.background_color`. It inherits `defaults.background_color` (`docs/inheritance-rules.toml:157`) and equals it in all 34 bundled combinations, so nothing differs today (M5 T12a ruling C4). Add a style for `application(..).style(..)` (`iced-0.14.0/src/application.rs:394-399`), a `styles::application(resolved)` whose background is `window.background_color` and whose text colour is the base style's, with its contract row. The colour-leaf coverage test names the leaf for this entry (`connectors/native-theme-iced/src/contract/derived.rs:1353-1365`).
  *Closed 2026-10-26 in b3591c62 (C-02):* `styles::application(resolved)`, background `window.background_color`, text colour the base style's (`defaults.text_color`), with two contract rows (`styles::application.background_color`, `.text_color`); the leaf left `UNREAD_COLOURS`, and the iced showcase passes it to `application(..).style(..)`.

### iced: dialog is unmapped

- [x] [v0.7.0] The iced connector maps no `DialogTheme` field, and its contract lists none as unreachable: iced 0.14 has no dialog widget. `iced_aw`'s `Card` could carry the maxima, `Card::max_height` and `Card::max_width` taking an `f32` (`iced_aw-0.14.1/src/widget/card.rs:144`, `:151`); the minima, the button gap and order, the fonts and the border would still have no receiver of their own. Decide whether a `Card` is the dialog's receiver.
  Named in the iced colour-leaf coverage test (M5 T12b, 9c46d834).
  G2 Task 13 (2026-10-14): the new `dialog.entrance` has no receiver either: iced 0.14 and `iced_aw` 0.14.1 have no dialog widget, so nothing animates a dialog's entrance; the crate docs' *Theme Field Coverage* (`connectors/native-theme-iced/src/lib.rs`) names it among the values iced has no receiver for.
  *Closed 2026-10-26 in fad5f7f7 (C-01):* the controller's ruling: an `iced_aw` `Card` is not the dialog's receiver (its head, body and foot take styles of their own; only its maxima match). The dialog is the consumer's `container`: `styles::dialog` (fill, border, body text colour, each with a contract row), `dialog_padding`, `dialog_max_width`, `dialog_max_height`, `dialog_min_width` and `dialog_min_height` (the minima as a floor in the content, with the C-01 review's fix), `dialog_title_font_color` (an accessor row), and `dialog_button_gap` and `dialog_button_order` under the gpui connector's names (the egui connector has `dialog_button_order`); `dialog.entrance` and `.icon_size` are `UNREACHABLE` with their reasons, and the `UNREAD_COLOURS` "dialog" entry is gone. The iced showcase's dialogs use them.

### GNOME reader: title and heading sizes ignore the user's font

- [x] [v0.7.0] `adwaita.toml`, which the GNOME reader merges over, states fixed sizes for the text scale (`caption` 9pt, `dialog_title` 15pt, `display` 20pt; `[light.text_scale]`, `[dark.text_scale]`) and for `dialog.title_font` (15pt; `[light.dialog.title_font] size_pt`, `[dark.dialog.title_font] size_pt`). libadwaita sizes them as a percentage of the base font (`.caption` 82%, `.title-2` 136%, `.title-1` 181%; `docs/platform-facts.md:746-755`), so these are its sizes at the default 11pt only, and live mode ignores a user who sets another font. The GNOME reader (`native-theme/src/gnome/mod.rs`) reads the font but sets none of them; it should derive them from the font it reads, as the KDE reader derives its headings from the body font (`native-theme/src/kde/fonts.rs`, `HEADING_LEVEL_1_FACTOR` and `HEADING_LEVEL_2_FACTOR`). The preset half is done: the `-live` preset is removed (M1-T4-01, 2026-10-11).
  *Closed 2026-10-26 in 67aa7b4b (C-03):* the GNOME reader derives, from the size of the font it reads, `text_scale.caption` (82%, `.caption`), `text_scale.dialog_title` and `dialog.title_font` (136%, `.title-2`) and `text_scale.display` (181%, `.title-1`), the shares libadwaita states (`docs/platform-facts.md:746-755`, :1485-1488, :1561); the weights and `text_scale.section_heading`, which inherits the base size, stay the preset's. At GNOME's default 11pt the sizes round to adwaita.toml's (9, 15, 20, 15pt); the text-scaling factor is reported apart and applied once by the connectors (M2e).

### gpui: the base-palette repair writes colours without their tokens

- [x] [v0.7.0] Latent: the observer's base-palette repair (`connectors/native-theme-gpui/src/lib.rs` `repair_base_palette`, `:1370-1387`) writes the 12 base colours (`red` … `cyan_light`) onto `ThemeColor` without their `tokens`. Harmless while no widget reads `tokens.red` … `tokens.cyan_light`; check at every gpui-component bump.
      *GPUI Kit 0.8.0 (2026-10-22, G3 T10):* still harmless: no gpui-component or gpui-base
      0.8.0 source reads `tokens.red` … `tokens.cyan_light` outside `theme/`
      (`grep -rnE 'tokens\.(red|green|blue|yellow|magenta|cyan)'` over both
      crates' `src` finds none).
  *Closed 2026-10-26 in e92c288a (C-04):* the tokens are on the styled `Theme`, not on `ThemeColor`: `Theme::tokens: ThemeTokens` (gpui-component 0.8.0 `src/theme/mod.rs:154`), whose `red` … `cyan_light` (`src/theme/theme_color.rs:533-544`) a rebuild resets with the colours. `colors::copy_base_palette` now takes the `Theme` and writes each colour's solid token (`From<Hsla> for ThemeToken`, `:30-37`) beside it, and `repair_base_palette` probes a clone of it; `the_base_palette_repair_keeps_the_tokens_with_the_colours` holds all 12 tokens to the native colours after a rebuild (red's upstream token differs, so it proves the write).

### egui: the focus ring inside an Expander's body takes the expander's radius

- [x] [v0.7.x] The companion crate's `Expander` draws its body in the caller's style (round 4 review, item 17 I1), but the connector's install plugin rings a focused widget there with `expander.border.corner_radius`: `native_scope` records the scope's `Ui` with `Role::Expander`'s radius (`connectors/native-theme-egui/src/ext.rs:103-106`), and the ring takes the innermost recorded scope round the widget (`src/plugin.rs:595-613`, `:646-651`); the body's `Ui` records none. Add a public hook to native-theme-egui that sets a style on a `Ui` and records that `Ui`'s radius from it with `replace = true`, as `native_set_style` does for a role's cell, and call it in the `Expander`'s body wrapper (`connectors/native-theme-egui-widgets/src/expander.rs`, `Expander::show`) with the caller's style. Predates the round.
  *Closed 2026-10-26 in 64bd92be (C-05):* `NativeThemeUiExt::native_set_style_with_ring(style)` (a free `set_scope_style` at first; the C-05 review moved it onto the trait) sets `style` on the `Ui` and records its `widgets.active.corner_radius` for the focus ring with `replace = true`, as `native_set_style` does for a role's cell (nothing is recorded without an atlas); `Expander::show`'s body wrapper calls it with the caller's style, and the `Expander` doc (473899a7's) says the ring takes the caller's radius. `a_widget_focused_in_the_body_is_ringed_at_the_callers_radius` holds the body's ring to a plain button's over four presets, where a `Role::Expander` scope rings at another radius (and fails, by mutation, with the old `set_style`).

### gpui: the theme-drawn widgets' focus ring

- [x] [v0.7.0] The connector's `Checkbox`, `Radio`, `Slider` and `Switch` draw the focus ring through gpui-component's `focus_ring_style`, upstream's 3px at half the ring colour's alpha (`styled.rs:11-12`); the model states `defaults.focus_ring_width` and `focus_ring_offset`, which reach none of them.
  *Closed 2026-10-26 in 753cda70 (C-06):* `widgets::FocusRing` draws the model's ring for the connector's `Checkbox`, `Radio`, `Slider` and `Switch` in place of gpui-component's `focus_ring_style` (3 px at half `ThemeColor::ring`'s alpha, gpui-component 0.8.0 `src/styled.rs:11-12`, `:301-327`): `defaults.focus_ring_color` as stated, `focus_ring_width` wide, the outer edge the element's bounds grown by `focus_ring_offset + focus_ring_width` and the corners grown as much, floored at 0, an absolute child against the padding box, as iced's `focus_ring::RingLook::quad` and egui's `paint_focus_ring` draw it; no ring where the width is not positive or a value not finite; where the ring is inset at the edge (`offset == -width`, adwaita) a band inside the edge, or the element's own border in the ring's colour where it has one (the C-06 review's fix). `the_focus_ring_is_the_themes` checks the band's bounds, width, colour and corners for an outside and a straddling ring, the inset tint and the no-ring cases; `a_switch_told_not_to_draws_no_focus_ring` checks the switch's option.

### OS probe: macOS renders for the sidebar, the title bar and the list header

- [x] [v0.7.0] `sidebar.background_color` has no source on macOS: the sidebar
      is `NSVisualEffectView.Material.sidebar`, a material with no colour
      (`docs/platform-facts.md:1370`), yet `macos-monterey` and `macos-tahoe`
      state `#e8e8ed` / `#262626`. The OS probe resolves its colours with
      Reduce Transparency on and renders no material. Have it render a
      sidebar-material `NSVisualEffectView` with Reduce Transparency on and
      off, in both appearances, on each macOS image; then state the cell
      from the renders and the presets follow (found in M2a Step 8's
      re-audit, 2026-10-11).
- [x] [v0.7.0] `window.title_bar_background` and `list.header_background`
      are ≈ `defaults.surface_color` on macOS, "measured", with no render
      cited (`docs/platform-facts.md:1160`, `:1429`). In `macos-monterey`
      dark both inherit `controlBackgroundColor` `#1e1e1e` under a window of
      `#323232`. Render a titled window and an `NSTableView` with a header
      on the probe images, in both appearances, and check both cells against
      the renders (found in M2a Step 8's re-audit, 2026-10-11).
  *Closed 2026-10-26 in 8c219626 (evidence and facts 2245393c, dc1be6bd):*
  - os-probe run 40320895749 (`docs/evidence/os-probe/2026-10-26-run-40320895749/`, D1's
    key-window section) made a titled window key on every macOS image and captured it with a
    sidebar-material `NSVisualEffectView` (state active), an `NSTableView` with its header and a
    segmented control.
  - A calibration strip read exact in every capture.
  - Reduce Transparency was on (the images' setting), with a second pass off. The facts cells
    :1160, :1370, :1429, :1636 and :1637 state the measured values.
  - macos-monterey (macOS 14 and 15, which agree) and macos-tahoe (macOS 26, as 27 but for the
    dark selected segment, #067dff) state them.
  - Run 40324458365 (`docs/evidence/os-probe/2026-10-26-run-40324458365/`) adds the title bar of a
    window that is not key, #ececec / #141414 on macOS 14 and 15 (facts :1166, macos-monterey's
    `inactive_title_bar_background`), and shows wallpaper tinting off drawing every value the same.
  - The selected segment in a key window is not `selectedContentBackgroundColor`: macOS 14 and 15
    draw it raised and neutral (#ffffff / #666666) with the ordinary label colour, and macOS 26
    draws it #0077f9 / #077dff, which no NSColor the probe reads gives.

### OS probe: the selected segment in a key window

- [x] [v0.7.0] `segmented_control.active_background` on macOS is
      `selectedContentBackgroundColor` (`docs/platform-facts.md:1636`), whose
      value macOS 26 reports unchanged (probe 26 `:324`, `:398`), but the OS
      probe renders each control in a borderless window it never orders
      front, so never key (probe `:514`), and there the selected segment is
      not that colour: white, and white at alpha 74/255 in dark, on macOS
      14.8.9 and 15.7.9; black or white at alpha 50/255 on macOS 26
      (`renders/aqua/` and `renders/darkAqua/segmented_control-regular@2x.png`
      under `docs/evidence/os-probe/2026-10-11-run-38129819806/`, measured
      2026-10-11). Have the probe render a segmented control with a selected
      segment in a key window, in both appearances, on each macOS image;
      then the facts cell follows, and `macos-monterey` and `macos-tahoe`
      (which carries monterey's value, marked so) follow the cell (found in
      M2a Steps 8–12, 2026-10-11).
  *Closed 2026-10-26 in 8c219626 (evidence and facts 2245393c, dc1be6bd),* with *OS probe: macOS renders for the sidebar, the title bar and the list header* above, whose closing has the runs: the selected segment in a key window is not `selectedContentBackgroundColor`: macOS 14 and 15 draw it raised and neutral (#ffffff / #666666) with the ordinary label colour (facts :1636, :1637), and macOS 26 draws it #0077f9 / #077dff (#067dff dark on 27), which no NSColor the probe reads gives; macos-monterey and macos-tahoe state them.

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

### Closed items from Font DPI
- [x] [v0.7.0] For M4: an unrecognised `XDG_CURRENT_DESKTOP` with a GNOME portal backend gets the GNOME reader (`native-theme/src/pipeline.rs:631-646`), whose font DPI is 96, but `desktop_font_dpi(LinuxDesktop::Unknown)` is `None` (`native-theme/src/detect.rs:409`), so `ResolutionContext::from_system()` still reads `Xft.dpi` (96 × factor × the X11 UI scale under gnome-settings-daemon) while the connectors apply the factor: the preset path scales text twice there. The preset path should follow the reader's choice of desktop. Found in the M2e Step 7 review (2026-10-11).
  Fixed by 62c13acf (M4 T16, 2026-10-12): `desktop_font_dpi` takes the refined desktop the reader is chosen by (`detect::refine_linux_desktop`: `XDG_CURRENT_DESKTOP`, then the portal backend, then `kdeglobals`), so an unrecognised desktop with a GNOME portal backend gives 96 on the preset path. The preset path is synchronous and does not ask the portal: it uses the desktop the last system read (`from_system`, `from_system_async`, `AccessibilityPreferences::from_system`) refined, until `invalidate_caches()`; an application that resolves presets without such a read gets only the `kdeglobals` check.
  Closed 2026-10-13 (M4 Step 8): fixed by 62c13acf and 7553e77d (a portal answer that changes the desktop clears what was read from it). That an application which only resolves presets never gets the portal's answer is the open entry *Linux: an application that only resolves presets never gets the portal's answer* in `docs/todo.md`.
- [x] [v0.7.0] For M4 (connectors): on GNOME and Budgie X11 the connectors apply a text-scaling factor the toolkit already applied. gnome-settings-daemon 51.0 writes XSETTINGS `Xft/DPI` and the `Xft.dpi` resource as 96 × `text-scaling-factor` × the window scale (`plugins/xsettings/gsd-xsettings-manager.c:666-668`, `:742`, `:797-798`), and winit 0.30.13 (`src/platform_impl/linux/x11/util/randr.rs:41-53`, `:146`) and gpui-pre-linux 0.3.8 (`src/linux/x11/client.rs:2672-2677`) take their X11 scale factor from that DPI over 96. On an X11 session, and for any client through Xwayland, the window scale then carries the factor, and `AccessibilityPreferences::scaled` multiplies text by it again: text at factor² × the UI scale, and every other length scaled by the factor, which GTK does not do. Native Wayland clients are right. The connectors should not apply the factor where winit or gpui already did. Found in the M2e Step 7 review (2026-10-11).
  *Closed by M8 T8 (2026-10-17, d6ca9889):* on an `x11` session (`$XDG_SESSION_TYPE`, `detect::session_type`, now also under the `portal` feature) the GNOME reader reports `text_scaling_factor = 1.0` (`toolkit_scale_carries_text_scale` in `native-theme/src/gnome/mod.rs`, injected through `GnomePortalData::session_type` as the KDE DPI chain injects it), so the connectors apply nothing the toolkit's `Xft.dpi` scale factor carried; on a `wayland` session, or where the session type is not set, the factor is reported and applied once, as before. `AccessibilityPreferences::text_scaling_factor`'s doc says it is the factor the toolkit has not applied. The Xwayland half stays, the next entry.
  Archived 2026-10-17 (M8 T21): ticked in 775b26c0; its "above", "below" and "next entry" mean `docs/todo.md` at 6980aa05.

### Icon-theme detection commands have no timeout
- [x] [v0.7.0] The icon-theme detection runs `gsettings get <schema> icon-theme` (`native-theme/src/model/icons.rs` `gsettings_icon_theme`) and `xfconf-query -c xsettings -p /Net/IconThemeName` (`detect_xfce_icon_theme`) with `Command::output()`, which waits as long as the command runs; a hung D-Bus session blocks the caller (pre-existing). `detect.rs`'s `run_gsettings_with_timeout` (`:180-218`) bounds its own gsettings calls by `SUBPROCESS_TIMEOUT` (2 s), but it runs only `gsettings` and returns `Option<String>`, dropping why a call gave nothing (not found, unsuccessful status, stderr, empty output, timed out) — which the icon-theme error now reports. Needed: a general run-with-timeout helper that takes the program and its arguments and returns `io::Result<Output>`, with a timeout reported as its own reason ("timed out after 2 s"), so `command_icon_theme` keeps naming every failure.
  Fixed by 6c2327d4 (M4 T18, 2026-10-13): `detect::run_with_timeout(program, args, timeout)` returns `io::Result<Output>`, a timeout as `ErrorKind::TimedOut` ("timed out after 2s", which `command_icon_theme` reports as its own reason), and reads stdout and stderr while the command runs; the icon-theme detection's `gsettings` and `xfconf-query`, `fc-match`, macOS `defaults` and the diagnostics' `gsettings --version` run through it (`run_gsettings_with_timeout` had become `gsettings_get` over the same runner).
  Closed 2026-10-13 (M4 Step 8): fixed by 6c2327d4. `gsettings_icon_theme` and `detect_xfce_icon_theme` live in `native-theme/src/detect/icon_theme.rs` since 2c9d969b, and `run_gsettings_with_timeout` is gone: its successor is `gsettings_get` over the same runner.

### gpui: the native Switch lacks 0.7.0's focus options

- [x] [1.0] gpui-component 0.7.0's `Switch` gained `tab_stop`, `tab_index` and `focus_ring(bool)`; the connector's `widgets::Switch` offers none of them.
      Done in M6 T11 (M6-27): `widgets::Switch` has `tab_stop`, `tab_index`, `focus_ring`,
      `accessibility_label` and `Styled`, and `Checkbox` and `Radio` the same but `focus_ring`
      (`a_switch_keeps_its_tab_order`, `a_switch_announces_its_accessibility_label`,
      `the_callers_style_reaches_the_row` in `connectors/native-theme-gpui/src/widgets/tests.rs`).
  Archived 2026-10-17 (M8 T21): ticked in 485628c8.

### gpui: the `Loaded` equality test compares the theme by `is_dark()` only

- [x] [v0.7.0] For M6 (M5 T1 review, Minor): gpui's `from_system_matches_system_theme_ext` (`connectors/native-theme-gpui/src/lib.rs:1549-1561`) compares every other `Loaded` field of `from_system()` with `SystemTheme::to_gpui_theme()`'s, but the gpui `Theme` only through `is_dark()` (`:1555`), where iced's sibling test compares the palette (`connectors/native-theme-iced/src/lib.rs:2208-2221`, `:2220`). Compare a `ThemeColor` field too, such as `background`, so the two tests are equally strong.
  *Closed by M8 T21 (2026-10-17, M8-82):* already done in M6, 4c5410e8 (2026-10-15):
      `from_system_matches_system_theme_ext`
      (`connectors/native-theme-gpui/src/lib.rs:1915-1930`) asserts
      `loaded.theme.background == expected.theme.background` (`:1924`) beside
      `is_dark()` (`:1923`), the `ThemeColor` comparison this entry asks for.
  Archived 2026-10-17 (M8 T21): closed and moved here in the same commit.

### Closed items from GPUI Kit 0.7.3 (G1, G2): review minors left for M6 and M7

- [x] [v0.7.0] For M6: `connectors/native-theme-gpui/src/contract.rs:641` cites our own
      `colors.rs:649-656`, which has moved since; own-file citations were outside G1's scope.
      M6 T16a (2026-10-15): already corrected by M6 T9 (0c69f9bd), which moved the lines
      and changed the citation to `colors.rs:684-691`, where `assign_base_colors` assigns
      `red` … `yellow_light`; checked again at T16a's base, 3b696688.
  Archived 2026-10-17 (M8 T21): ticked in e765ae21.
- [x] [v0.7.0] For M6: no test shows an application's own `TooltipDefaults` delay surviving
      an upstream mode switch to a variant that states no delay (the install path is tested,
      this case is not).
      Done in M6 T5: `the_applications_delay_survives_a_mode_switch_to_a_variant_without_one`
      (`connectors/native-theme-gpui/src/lib.rs`) switches to a variant of the same theme that
      states none, and to the unstored mode of a theme applied in one variant after another
      (M6-02).
  Archived 2026-10-17 (M8 T21): ticked in c1208377.
- [x] [v0.7.0] For M8 (iced): the iced Layout table's header rule sits outside
      `list.header_height` (iced puts `separator_y` between rows) where the Basic table's is
      inside; say so beside the Layout table's header height in `examples/showcase-iced.rs`.
      The tooltip delay test covers the Basic tooltip only (all six share `ShowDelay`), and
      the header test re-derives the fallback without `scaled_text_size` (true at the default
      accessibility settings).
  *Closed by M8 T14 (2026-10-17, ba117968, dcfe5c2d):* the Layout table's header
      comment says the rule is iced's `separator_y`, drawn past the padding below the
      header (iced_widget 0.14.2 `src/table.rs:535-557`), outside `list.header_height`;
      `every_tooltip_waits_the_themes_show_delay` rests the pointer on the chrome's
      tooltip and the Display page's four, whose frames under a 60 s delay and under none
      differ; the header test derives an unstated header through `scaled_text_size` at text
      scales 1 and 1.5.
  Archived 2026-10-17 (M8 T21): ticked in 5e353d5a.
- [x] [v0.7.0] For M8: `winicons.rs`'s `bitmap_bgra` returns on
      `checked_neg()?` (`native-theme/src/winicons.rs:223`) without a `note_failure`;
      unreachable after the bound check above it.
  *Closed by M8 T13 (2026-10-17, 69831323):* the negated height is taken in
      the size check, so its failure (none, for a height `u32` accepts) notes
      the bitmap's size as that check's others do.
  Archived 2026-10-17 (M8 T21): ticked in 39106f15.
- [x] [v0.7.0] For M8's api batch (M6-12 ruling, 2026-10-15): ask the maintainer whether
      `native_theme_gpui::icons::freedesktop_name_for_gpui_icon(icon, de)` goes. It picks a
      name by desktop, where the icon theme in use decides which name exists (Adwaita, with
      the AdwaitaLegacy it inherits, lacks the Breeze name for 48 of the 50 icons the
      conventions name apart); M6 T7 added `freedesktop_name_candidates(icon)` beside it and
      core's `FreedesktopLoader::any(names)`, moved the gpui showcase to them, and the old
      function's docs point there. Removing it is an api-break of a released `pub fn`. The
      iced showcase's `panel_left_name` (`examples/showcase-iced.rs`, beside `PanelLeft`'s
      doc) makes the same desktop-keyed choice for `sidebar-show` / `sidebar-expand-left`.
      M7 T12 (2026-10-16): the iced half is done. `panel_left_name` is gone; the iced
      showcase's chrome loads its freedesktop names through `FreedesktopLoader::any` in the
      one theme it chose (`&["sidebar-expand-left", "sidebar-show"]` for `PanelLeft`), and so
      does the egui showcase's `ChromeIcon` (M7-15, its `gtk_desktop` gone). A fixture test in
      each showcase loads the icon from a GTK-names-only and a Breeze-names-only theme on Sway
      and on GNOME. What stays open here is the gpui function's question for M8.
  *Closed by M8 T5 (2026-10-16, 6c82cf19):* the function is not removed but `#[deprecated(since = "0.7.0")]`, its note pointing at `freedesktop_name_candidates` and `FreedesktopLoader::any`, its own tests under `#[allow(deprecated)]`, a CHANGELOG *Deprecated* line. Its removal is a line of the M8 close-out's maintainer question.
  Archived 2026-10-17 (M8 T21): ticked in c522be20.
- [x] [v0.7.0] For M6: no test clicks the DataTable's sort button, so the showcase's
      `perform_sort` (`examples/showcase-gpui/support.rs`) is untested; upstream gives the
      button `.id(("icon-sort", col_ix))` with `test_support()`. The sort button's claims sit
      on the table-level info (`info/data.rs`), not on the header row's own
      (`data_table_header`).
      M8 T21 (2026-10-17): for M9a, not M6: M6's Step 6 ruling deferred the
      three showcase minors to M9a, the gpui showcase's milestone.
  *Closed by M9a T20 (2026-10-18, be7b6392):* `the_tables_sort_button_sorts_its_rows`
      clicks upstream's `("icon-sort", col_ix)` (gpui-component 0.7.3
      `table/state.rs:1643`) through its cycle, descending, ascending and back to the
      rows as given, and reads the delegate's rows after each click.
- [x] [v0.7.0] For M6: the showcase's `SampleTableDelegate` does not override `move_column`
      while its columns are movable, so dragging a header moves the header and not the data,
      and the sortable Role column then sorts the wrong data (pre-existing).
      M8 T21 (2026-10-17): for M9a, not M6: M6's Step 6 ruling deferred the
      three showcase minors to M9a, the gpui showcase's milestone.
  *Closed by M9a T20 (2026-10-18, be7b6392):* `SampleTableDelegate::move_column`
      moves the column with its cells, in `rows` and in `unsorted`, as
      `table/delegate.rs:120-130` asks; `a_moved_column_keeps_its_data` drags the Role
      header before Name and sorts the moved column by its sort button.
- [x] [v0.7.0] For M6: the gpui showcase evaluates `themed_entrance(cx)` more than once per
      dialog (`examples/showcase-gpui/demo.rs`); `info/chrome.rs`'s `panel_toggle` carries a
      `too_many_arguments` allow.
      M8 T21 (2026-10-17): for M9a, not M6: M6's Step 6 ruling deferred the
      three showcase minors to M9a, the gpui showcase's milestone.
  *Closed by M9a T20 (2026-10-18, 2f2debef):* each dialog's build evaluates
      `themed_entrance(cx)` once and hands it to its info and to `dialog_frame`; a
      `#[cfg(test)]` counter reads one per build of each of the four dialogs
      (`a_dialog_reads_the_themes_entrance_once`). `info::chrome::panel_toggle` takes
      the status bar's `demo::PanelToggle`, and its `too_many_arguments` allow is gone.
- [x] [v0.7.0] For M9b (M8 T14 review, 2026-10-17): above a text scale of 1 the iced
      showcase's table headers keep a stated `list.header_height`, where gpui's grow.
      iced: `basic_table` gives the header `native_theme_iced::list_header_height(resolved)`
      where the theme states one (`connectors/native-theme-iced/examples/showcase-iced.rs:8405-8406`),
      an accessor that returns the stated value and takes no accessibility preferences
      (`connectors/native-theme-iced/src/lib.rs:1176-1178`); the header cell's text box is that
      height less the rule, fixed (`:8380`, `:8390`), while its label is drawn at the scaled
      `list.header_font` size. The Layout table's header cells are fixed the same way
      (`:10361`, `:10368-10370`), and the Basic body rows keep a stated `list.row_height`
      (`:8356-8359`). `the_tables_headers_take_the_themes_header_height` asserts the stated
      height at text scale 1.5 for adwaita and material-3. gpui: `geometry::list_header_height`
      grows a stated header above 1 to at least the scaled `list.item_font` line box, the cell
      padding and the row's bottom border (`connectors/native-theme-gpui/src/geometry.rs:1074`,
      through `table_row_height`, `:1091`), as `data_table_size` grows a row. The risk: where
      the scaled label's line box and the padding no longer fit the stated height, iced's label
      runs past its cell, which does not clip unless asked (iced_widget 0.14.2
      `src/container.rs:102`), and over the header's rule, or is clipped where a parent clips.
      M9b grows the iced headers (and the test's expectation) as gpui's do; M8 leaves the
      behaviour as it is.
  *Closed by M9b T11 (2026-10-21, 11c07f3c):* the iced showcase grows a stated height
      above a text scale of 1 by gpui's rule, through the shared crate's
      `rows::scaled_height` (with its unit tests), which the egui showcase's tables take
      in M9c: the stated height at a factor of 1 or less; above it at least the line box
      of the row's font at its scaled size and `defaults.line_height`, rounded up, plus the
      row's padding and any rule inside it. The Basic table's header (`list.header_height`,
      else `list.row_height`; its `separator.line_width` line inside it) and body rows
      (`list.row_height`), and the Layout table's header (its cells' padding, no rule
      inside) take it. `the_tables_headers_take_the_themes_header_height` expects the
      grown heights at 1.5 over adwaita, material-3, macos-tahoe and catppuccin-mocha,
      computed at test time (adwaita's 30 px header is 32).
- [x] [v0.7.0] For M7: the egui showcase's body rows grow past `list.row_height` where an
      auto-sized column wraps (the Data and Theme Map tables; egui_extras `layout.rs:165`);
      `catppuccin-mocha`'s header label (19 px) is taller than its stated header height
      (18 px), pre-existing.
      M8 T21 (2026-10-17): for M9c, not M7: M7's Step 6 ruling sent it to
      M9a as the showcases' milestone; M9's plan gives each showcase its
      own, M9a gpui, M9b iced and M9c egui.
  *Closed by M9c T12 (2026-10-24, 8fdda5db):* the Data and Theme Map tables' rows are
      exactly their height: `list.row_height` where the theme states it, grown above a
      text scale of 1 by the shared crate's `rows::scaled_height`, else egui's own
      `interact_size.y` or the line box rounded up where taller, and the header likewise
      (`list.header_height`, else a row, in the header font). Each cell is one unwrapped
      line in its font's line box at `defaults.line_height`, an auto-sized column as wide
      as its text, and the last column is clipped (egui_extras 0.36.2 `Column::clip`,
      `src/layout.rs:160-168`, `:218-224`), so a text too long for it is cut short with
      egui's ellipsis and shown whole on hover (egui 0.36.2 `src/widgets/label.rs:285-294`);
      the rows lie with no spacing between them. The second sentence does not hold as
      written: catppuccin-mocha states no `list.header_height` (its two
      `header_height_px = 40.0` keys are `[light.expander]` and `[dark.expander]`), so the
      18 px was egui's own `interact_size.y`, under the 19 px line egui laid its label out in
      on the host where it was measured;
      the header is now 18 px with its label in its 16.8 px line box.
      `the_tables_headers_take_the_themes_header_height` lost its `header.max(content)`
      allowance.
      M9c T12 fix round 1 (2026-10-24, bdd5ffa9): the columns that clip are the Data
      table's last (Platform) and the Theme Map's Value and Sinks, which share the room
      the Leaf and Verdict columns leave (egui_extras 0.36.2 `src/sizing.rs:141-166`), so
      the Theme Map stays inside the central panel, where its Value column took its whole
      text and pushed the Verdict and Sinks columns past the panel at a text scale of 1.5;
      the Data table's Theme column and the Theme Map's Leaf and Verdict columns are as
      wide as their text.
      M9c T12 fix round 2 (2026-10-24, fc64f851): the Theme Map's Value and Sinks columns
      are each at least as wide as their header's text as drawn, and the table lies in a
      horizontal `ScrollArea`, so where the Leaf and Verdict columns and those floors are
      wider than the page (at the text scale 2.25 the Preferences offer, on every bundled
      preset) the rest is reached by scrolling across.

### Closed items from M6 close (2026-10-16): items carried to M7, M8, M9a and the presets

- [x] [v0.7.0] For M6's deferrals (ruling, Step 6): the Segoe Fluent SVG-outline item ("Segoe … to weigh in M6", the `For M6` entry above) moves to [1.0]: M4's em-box raster and `.size(u32)` removed the visible defects, M6-13 routes the requested size to the Segoe loader, and a `GGO_NATIVE` outline pipeline is Windows-only code this host cannot run. The public mode-taking theme builder ("Connectors: a public theme builder that takes the mode") and the widget contracts ("Contracts: the connectors' widgets") go to M8's API batch for all three connectors. The three showcase minors in *GPUI Kit 0.7.3 (G1, G2): review minors left for M6 and M7* (the sort-button test, `move_column`, `themed_entrance`/`panel_toggle`) go to M9a. The `TooltipDefaults` survival item there was closed by M6 T5 (c1208377), and the `contract.rs:641` citation was already right (M6 T9, 0c69f9bd).
  *Closed by M8 T21 (2026-10-17, ffd34ba1):* each part is where the ruling sent it. The
      Segoe entry (`docs/todo.md`, *Segoe Fluent icons as SVG outlines*, "For M6
      (M1-T6-07's Segoe item") is [1.0] (ffd34ba1); the mode-taking builder and the
      widget contracts closed in M8's API batch, M8 T4 (8d0e029f) and M8 T7
      (c94f1a1b, c645b2d5, 637f8fef); the three showcase minors carry M9a
      (ffd34ba1).
  Archived 2026-10-17 (M8 T21): closed and moved here in the same commit; its "above", "below" and "next entry" mean `docs/todo.md` at 6980aa05.
- [x] [v0.7.0] For M8 (parity): iced's `AnimatedSvgHandles::frame_duration_ms` is still a `u32`, while gpui's `AnimatedImageSources::frame_duration_ms` is a `NonZeroU32` since M6 T1 (8212bf00).
  *Closed by M8 T4 (2026-10-16, fdc4b24a):* the field is a `NonZeroU32`, in the same commit as M8-08's `spin_rotation_radians(elapsed, duration_ms: NonZeroU32)`, whose zero guard and callers' `.get()`s are gone (the doc example, the README's spin recipe, the `Spinner`, the showcase's Icons page).
  Archived 2026-10-17 (M8 T21): ticked in b1fcb373.
- [x] [v0.7.0] For M8 (parity): gpui lays `slider.thumb_hover_color` over the thumb as a layer (M6 T9, `colors::over`), while iced uses it as given.
  *Closed by M8 T3 (2026-10-16, 97105ccc):* iced's `styles::slider` composites the hover over the thumb with `styles::composite_over`, as gpui's `SliderLook` does, and its contract row follows; both composites are native-theme's `color::composite_over` since 072855bc. No bundled value moves: every stated thumb hover is opaque.
  Archived 2026-10-17 (M8 T21): ticked in 2b1f73ca.
- [x] [v0.7.0] For M8 (parity): on a text-scaling factor whose product is not a normal positive number, gpui and iced keep the theme's unscaled size (core's `AccessibilityPreferences::scaled`, M6 T13), while egui keeps its stock size with a `Note::ValueSanitised`, as its spec prescribes (d05d64f6). Decide whether the connectors should agree.
  *Closed by M8 T6 (2026-10-16, e4972f32):* they do not unify (Step 6 ruling): both are documented clamps of a pathological factor. egui's `scaled_text_size` keeps its plain product by its spec §8.6, and its doc says why the siblings differ; iced's and gpui's docs name egui's contract; the parity gate records nothing, as the three signatures agree.
  Archived 2026-10-17 (M8 T21): ticked in b3ca8c31.
- [x] [v0.7.0] For M8 (parity): iced's `font_family` returns the default family only. gpui's geometry builders carry each font's family and style since M6's final review (35785f1e).
  *Closed by M8 T7 (2026-10-16, 6e9c2121):* `font_family` stays the defaults' family, its leaf's name (Step 6 ruling); the per-widget parity is `font(spec) -> iced_core::Font` (feature `system-fonts`): the family `system_font_family` gives (since 2fcc26fb the platform's substitute where the system lacks the stated one, as gpui and egui draw it, M8-01), the weight through `to_iced_weight`, the style; it leaks one name per distinct family, kept for the process. The showcase's own leak is M9b's to shrink.
  Archived 2026-10-17 (M8 T21): ticked in 80c6cf02.
- [x] [v0.7.0] For M8: the README dependency-version check exists twice, byte for byte (iced's and gpui's `compat.rs`, `the_readme_depends_on_this_release`). Make it one workspace check over every crate's README, rather than adding egui, core and build copies (M6 T3 ruling). The egui widgets README has a `toml` block too since M7 T3 (M7-34), and the check takes it with the egui README's.
  *Closed by M8 T9 (2026-10-17, 9246c7de):* `scripts/check_readme_versions.py` replaces both copies and their parser tests, and reads every README the workspace has, the egui widgets one among them: every `native-theme*` dependency in a `toml` block names the workspace version's `major.minor`, and each crate's README states its own crate's, in its Quick start where it has one. The entry above records the rule.
  Archived 2026-10-17 (M8 T21): ticked in 304544ed; its "above", "below" and "next entry" mean `docs/todo.md` at 6980aa05.
- [x] [v0.7.0] For M8 (test quality): the icon fixture helper (`icon_fixture.rs`) is copied between the iced and gpui connectors (M6 T4). The gpui DataTable row-growth test re-implements `table_row_height`, and the row formula leaves out the row's 1 px bottom border (M6 T6). The text stays inside either way.
  *Fixture half closed by M8 T2 (2026-10-16, 990ab210):* one `publish = false` crate, `native-theme-test-fixtures` (tools/test-fixtures), replaces the copies; the DataTable half stays open.
  *DataTable half closed by M8 T14 (2026-10-17, b788fc94):* `table_row_height` carries the 1 px bottom border upstream draws inside a row and the header row (`border_b_1`, gpui-component `src/table/state.rs:1899`, `:2063`); the geometry tests call it instead of their copy, and `tests/seams.rs` checks the cell text inside the cell's padding and that border at text scales above 1, which the formula without the border failed (kde-breeze at 2: the text half a pixel into the padding).
  *Correction (M8 T14 fix round 1, 2026-10-17):* the header row's border is the main header's `.h(header_row_height).border_b_1()` at gpui-component `src/table/state.rs:1959-1960`; `:1899` is the left-fixed columns' header.
  Archived 2026-10-17 (M8 T21): ticked in 5e353d5a.
- [x] [v0.7.0] For M8's API batch: whether to remove `freedesktop_name_for_gpui_icon(icon, de)`, now that `freedesktop_name_candidates` and core's `FreedesktopLoader::any` choose by the icon theme (M6 T7).
  *Closed by M8 T5 (2026-10-16, 6c82cf19):* deprecated, not removed, as the entry above records.
  Archived 2026-10-17 (M8 T21): ticked in c522be20; its "above", "below" and "next entry" mean `docs/todo.md` at 6980aa05.

### Closed items from M7 close (2026-10-16): items carried to M8, M9a and the presets

- [x] [v0.7.0] For M8's API batch: a `Ui` form of `role_modifier` (M7-28, deferred: an `Option` against `NativeThemeUiExt`'s contract, ext.rs:15). The egui showcase keeps its own helper.
  *Closed by M8 T6 (2026-10-16, 915c84b7):* a free function, `native_theme_egui::role_modifier(ui, role, variant) -> Option<StyleModifier>`, not a `NativeThemeUiExt` method (no method of it returns an `Option`, and a `StyleModifier::default()` would drop egui's `menu_style`); the showcase's four sites call it and its own helper is gone.
  Archived 2026-10-17 (M8 T21): ticked in b3ca8c31.
- [x] [v0.7.0] For M8 (parity): egui clamps a custom icon's size only at 1 and gives `None` for a size the SF Symbols or Segoe loader refuses. gpui clamps to 1..=512 and returns an icon (M7 T4, M6 T4).
  *Closed by M8 T3 (2026-10-16, c40de308):* the bound is native-theme's `icons::MAX_ICON_SIZE` (512), clamped once in `load_custom_icon`/`load_custom_icon_in` and in `rasterize_svg`; gpui's copy and clamps and egui's `.max(1)` are gone, and iced takes it with M8-02's `size` (T4). A loader that cannot draw the clamped size still returns `None`.
  Archived 2026-10-17 (M8 T21): ticked in 2b1f73ca.
- [x] [v0.7.0] For M8 (test quality): the icon fixture helper now has three copies (`icon_fixture.rs` in the iced, gpui and egui connectors), and the showcases' child-process fixture runner is copied between the egui and iced showcases (about 70 lines, M7 T12). The core test `a_custom_icon_is_looked_up_in_the_icon_theme_given` proves something only on a host whose system theme has `edit-copy`. The `§` tripwires excuse a whole line on a substring match, and the widgets crate's cuts at the first `#[cfg(test)]` text. `t18_g_to_egui_builder_is_to_egui_atlas_before_build` returns early on a headless run.
  *Fixture half closed by M8 T2 (2026-10-16, 990ab210, 32ac77a6):* the three `icon_fixture.rs`, the two showcase runners and the three theme writers are one dev-only crate (tools/test-fixtures), and `a_custom_icon_is_looked_up_in_the_icon_theme_given` reads fixture themes in a child process; the tripwire and headless-return items stay open.
  *Closed by M8 T14 (2026-10-17, af06ca88, 04eace98, 186e3a00, 96604bcd):* `t18_g_to_egui_builder_is_to_egui_atlas_before_build` never returned early on Linux, where a desktop with no reader falls back to the platform preset (native-theme `pipeline.rs`, `linux_fallback`); it and every other connector and showcase test that read the OS theme now expect it on Linux (M8-16), the skip kept off Linux. The `§` tripwires of the egui and egui widgets crates check each `§` of a line, named by the document right before it or continuing a named list or range, so `spec §1 and §2` fails; the widgets crate's source readers leave out a file's inline test module only, by the egui tripwires' rule.
  Archived 2026-10-17 (M8 T21): ticked in 5e353d5a.
- [x] [v0.7.0] For M8 (docs): the CHANGELOG lines that say a default build "no longer reads the OS theme" (gpui, iced, egui) are loose. Without a reader, Linux still reads the light/dark preference and reduced motion (native-theme pipeline.rs, detect.rs).
  *Closed by M8 T20 (2026-10-17, 0743c84d):* the gpui and egui *Breaking Changes* lines say a default build no longer reads the OS theme through a platform reader, and they and the iced `native` line give the platform preset on Linux in the light or dark mode the OS reports and with its reduced-motion preference (`pipeline.rs`'s `linux_fallback`, `detect_is_dark`, `readerless_accessibility`), an error on macOS and Windows.
  Archived 2026-10-17 (M8 T21): ticked in 6980aa05.
- [x] [v0.7.0] For M8 (scripts): `scripts/check_release.sh` runs `cargo fmt --all`, which formats rather than checks, so only CI's `--check` fails on unformatted code (by design; recorded so the two are known to differ).
  *Closed by M8 T1 (2026-10-16, f59bcdc3):* by design, as the script's header now says: the per-commit gate formats what is about to be committed, and CI's `cargo fmt --all --check` refuses what was committed unformatted.
  Archived 2026-10-17 (M8 T21): ticked in 3e169bd6.
- [x] [v0.7.0] For M7's follow-up in M8: the egui plugin's per-viewport focus entry (connectors/native-theme-egui/src/plugin.rs ~:82-84) stays after its viewport closes, the same leak shape M7 T9 fixed for the widgets' parts store.
  *Closed by M8 T6 (2026-10-16, 66053837):* at each pass's `input_hook` the install plugin drops the focus entry of every viewport whose input it saw and that egui no longer has (`Context::cumulative_pass_nr_for` is 0 for it; egui 0.36.2 has no accessor that lists its viewports), the viewports of the passes in progress excepted, and its `last_sent` window theme (fix round 1); at its first root pass it adds the viewports of the entries written before the install, found through `IdTypeMap::temp_keys`, to those it saw (fix round 1); a test opens an immediate viewport, closes it and holds one `FocusEntry` two passes later, where it held two.
  Archived 2026-10-17 (M8 T21): ticked in b3ca8c31.
- [x] [v0.7.0] For M9a: the egui showcase's `named_image` reads its freedesktop icon from disk in every pass (`role_image` is cached since M7 T12).
      M8 T21 (2026-10-17): `named_image`: M9c; the parity items are the G1/G2 section's (M9a), not repeated here.
  *Closed by M9c T13 (2026-10-24, efb1d47e):* the showcase keeps the icons it loads itself in an `IconCache` beside `DemoState`, keyed as `chrome_key` keys an icon (its name, the set, the icon theme, the size, the tint): `named_image`'s freedesktop icons are read from disk once per key, an icon the theme lacks kept as absent, and the Icons page's loading indicator (ledger M9-104) is loaded once per set and icon theme and each frame's image built once per frame and scheme. `App::adopt` clears it, since `ThemeAtlas::install` forgets the connector's icon URIs a kept image names, and so does a change of the icon choice; a scheme switch under a chosen theme keeps both schemes' icons. `the_chromes_named_icons_load_once_per_key` and `the_icons_page_loads_its_indicator_once` count the loads under a fixture theme: once in the passes of opening and eight steps (14 before, one a pass), once more after an install, only the new text colour's keys after a mode switch.
      This was the section's last item; *M7 close (2026-10-16), its `[v0.7.0]` entries* leaves docs/todo.md.

### gpui showcase: two 1px offsets under adwaita's borderless fields

- [x] [v0.7.0] For M9a (M8 T16 fix round 2, 2026-10-17): under adwaita, whose
      button, text input and drop-down have no line at rest
      (`docs/platform-facts.md` §2.3, §2.4, §2.24 `border.line_width` 0), the
      parity comparison at rest (M8 T16's last after-run, both variants)
      finds two gpui positions a pixel off that no exception covers:
      `basic.text_inputs.focused.text.x` gpui 10, iced 9, egui 9 (the
      focused field's text); `basic.buttons.primary.x` gpui 86, iced 85
      (85.18), egui 85 (85.19). gpui-component's focused Input adds
      `border_1()` in the ring colour (gpui-component 0.7.3
      `src/input/input.rs:758-762`) over the 0 the theme states, which moves
      the text a pixel in; the primary button sits after the default one,
      80 wide in gpui and 79.18 / 79.19 in iced / egui. Find what the gpui
      showcase or connector must change so the two land where iced and egui
      put them, or the forced cause with its source.
      M8 T16 fix round 3 (2026-10-17): the focused text's offset is forced:
      upstream's focused `border_1()` is a focus-state style no `geometry`
      refinement reaches, recorded with its ring as the gpui contract's
      `UPSTREAM_LIMITS` entry (`connectors/native-theme-gpui/src/contract.rs`);
      the primary button's 86 stays open here.
      M8 T16 fix round 4 (2026-10-17): "forced" above overstates it. What
      upstream cannot do is draw the theme's inset ring in a focus state;
      the 1px edge, and the text offset with it, is the price of keeping
      upstream's focus display, as `Input::focus_bordered(false)`
      (gpui-component 0.7.3 `src/input/input.rs:311-314`) would drop both the
      edge and the ring, leaving a focused field with no focus display. The
      contract entry and the exception reason now say so; the entry is a
      documented limit, as no test can observe gpui's painted scene.
  *Closed by M9a T20 (2026-10-18, ca454def):* the primary button's 86 comes from the
      default button's width alone: in M8 T16's last after-run's adwaita dumps the
      primary button sits `layout.widget_gap`, 6px, after the default one in all three,
      which is 80 wide in gpui (its 46px label, a shaped run rounded up) and 79.18 /
      79.19 in iced / egui. `adwaita:basic.buttons.primary.x` joins the whole-pixel
      group of `docs/showcase-exceptions.toml` with `adwaita:basic.buttons.default.w`,
      and the focused text's `rest@adwaita:basic.text_inputs.focused.text.x` the
      focused Input's toolkit-limit group, whose reason now names the offset;
      `scripts/check_showcase_parity.py` over those dumps excepts every difference.

### iced showcase: the toggle's general button Widget Info lines
- [x] [1.0] Latent (M5 T5 review, Minor): the iced showcase's Widget Info lines for `button.background_color`, `button.font.color`, `button.hover_background` and `button.hover_text_color` (`connectors/native-theme-iced/examples/showcase-iced.rs:5561-5577`) describe `{class}`'s fill and text colour at rest and Hovered, which under `class = "styles::button_toggle"` (`:5543-5552`) misdescribe the "On" toggle: a checked toggle is its checked pair at rest and under the pointer, with no hover of its own (`connectors/native-theme-iced/src/styles.rs:245-248`). None shows today, since `basic.buttons.toggle_on` lists none of the four leaves (`docs/showcase-elements.toml:845`). A change that adds one gives it a toggle arm, as `checked_background` has (`showcase-iced.rs:5578-5580`).
  *Closed by M9b T3 (2026-10-21, b3d5a8ca):* the toggle's colour lines are read from the
      iced contract's export (`connectors/native-theme-iced/contract-leaves.toml`):
      `basic.buttons.toggle_on` routes through `styles::button_toggle`'s rows
      (`BUTTON_TOGGLE_ROWS`, which carry the checked pair) under the checked statuses
      alone, so a leaf of the plain button's pair names a row only where it draws the
      On toggle on the installed preset (`button.active_background` and
      `.active_text_color` where the theme states no checked pair), and none of them
      can describe a hover the checked toggle does not have.

### iced showcase: the startup fallback outside Linux
- [x] [1.0] When `SystemTheme::from_system()` fails at startup, the iced showcase installs the adwaita preset and selects it (`showcase-iced.rs` `State::starting_with`, `:950`, fallback `load_adwaita_fallback` `:497`), also on macOS and Windows, where adwaita is `platforms: &["linux"]` (`native-theme/src/presets.rs:149`) and the theme picker does not list it. The gpui showcase instead keeps gpui-component's own theme under `default` and shows the error. Decide what a failed OS read falls back to off Linux (the platform's own preset would match the "only native presets" rule).
  *Closed by M9b T6 (2026-10-21, fb0d3ef5; the preset's rule as fix round 1 sets it, c95b9ff8):* a
      failed OS read at startup installs and selects the platform's own preset
      (`platform_fallback_preset`), the one a system read merges over on every platform
      (`native_theme::pipeline::platform_preset_name`, as the egui showcase falls back to):
      `kde-breeze` on KDE, `adwaita` on another Linux desktop, the running version's
      generation on macOS, `windows-11` on Windows; with the OS's accessibility preferences
      (`detect::accessibility_preferences()`).
      `the_startup_fallback_is_the_platform_preset_in_every_mode` computes the preset from
      `platform_preset_name` itself and holds it to a preset the platform offers.

### Hover fills: a layer or a replacement

- [x] [v0.7.0] The connectors composite `hover_background` and
      `active_background` over the widget's rest fill, by rule C17, which
      says "the platform layers where the toolkits replace"
      (`docs/archive/todo_v0.5.9_theme-contracts-rationale.md:535`; egui
      `connectors/native-theme-egui/src/style/states.rs:125`, gpui
      `connectors/native-theme-gpui/src/colors.rs:354`, iced
      `styles::composite_over`, `connectors/native-theme-iced/src/styles.rs:79`).
      WinUI 3 replaces instead: `ButtonBackgroundPointerOver` =
      `ControlFillColorSecondary` takes the place of `ButtonBackground`
      (`docs/platform-facts.md:1192`), and so do `ButtonBackgroundPressed`
      and `ComboBoxBackgroundPointerOver` (Button_themeresources.xaml:130,
      ComboBox_themeresources.xaml:243, microsoft-ui-xaml 8463f45). M2b
      Step 7b (`docs/todo_code-review-findings.md`, M2b) has windows-11
      state the button's and the segmented control's hover flattened over
      the window colour, `#f6f6f6` (dark `#323232`), which composites to
      itself, and the button's pressed fill too, `#f5f5f5` (dark `#272727`).
      Fix round 2 flattens the combo box's hover the same way,
      `ComboBoxBackgroundPointerOver` = `ControlFillColorSecondary`, `#f6f6f6`
      (dark `#323232`), and the switch's off-track hover,
      `ToggleSwitchFillOffPointerOver` = `ControlAltFillColorTertiary`,
      `#e5e5e5` (dark `#2a2a2a`, ToggleSwitch_themeresources.xaml:136, :16);
      The model's only pressed fill is
      `button.active_background`: `tab.active_background` and
      `segmented_control.active_background` are the selected state
      (docs/property-registry.toml:191, :325, :528), and
      `scrollbar.thumb_active_color` is the dragged thumb, which facts §2.8
      sources for Windows. For M5–M7: decide whether
      the model's
      `hover_background` is a layer (C17 stays, and a preset for a replacing
      platform states the flattened result, as windows-11 now does) or a
      replacement (the connectors write it as given, and the presets for
      layering platforms state composites). For M6 as well: gpui's
      `Hsla::blend` keeps the base's alpha (`colors.rs:347-353`), which is
      not source-over once a preset states a translucent
      `button.background_color` (windows-11 does since Step 7b, WinUI's
      `ControlFillColorDefault`).
      iced (M5 T6, 98b522f7): a layer (C17 stays); an unset soft hover is
      the idle fill.
      gpui (M6 T9, 2026-10-15): a layer (C17 stays), composited
      source-over: one `colors::over` (straight alpha in sRGB) in place of
      `Hsla::blend` for the button states and the widgets' hover states, so
      a translucent layer over a translucent fill covers both (the
      macos-tahoe entry under *For M6: two defects*, defect (1)). Over an
      opaque base or under an opaque layer it is `blend` bit for bit; what
      moved is macos-tahoe's derived pressed fill, material-3's checkbox
      hover (a state layer over a transparent box, which `blend` hid) and
      windows-11's checkbox hover (`#0000000a` over `#00000006`, dark
      `#ffffff0f` over `#00000019`: the preset's "Fluent checkbox hover
      fill", which cites no source, now laid over the translucent fill
      where `blend` showed the fill alone). The switch and the
      slider leave an unset hover at rest; the checkbox's is M6-22 (M6 T11).
      gpui (M6 T11, 2026-10-15): the checkbox's and the radio's unset hover
      is the box at rest too (M6-22).
      Fix round 1 (2026-10-15): over the window that composite lands within
      one 8-bit level of `ControlAltFillColorTertiary`, the brush
      *Checkbox: a checked checkbox's hover appearance* names for
      `CheckBoxCheckBackgroundFillUncheckedPointerOver`, laid over the
      window: light 228.0 against 228.7 (`#0F000000` over
      `#f3f3f3`), dark 42.2 against 41.6 (`#0BFFFFFF` over `#202020`); the
      brush's values as `windows-11.toml`'s comments above
      `[light.switch] hover_unchecked_background` and
      `[dark.switch] hover_unchecked_background` cite them
      (Common_themeresources_any.xaml:235, :31 at microsoft-ui-xaml 8463f45).
      The gpui connector also exports the composite, `layer_over`.
      M8 T3 (2026-10-16): both f32 composites are native-theme's `color::composite_over`, and the iced connector exports it too, as `styles::composite_over`.
      egui (M7, docs/todo_code-review-findings.md:10138, ruled in 467d54e6): a layer (C17 stays),
      settled with nothing to change: ecolor's own `Color32::blend` composites source-over
      (`connectors/native-theme-egui/src/convert.rs:330-340`, `composite_over`), right for a
      translucent layer over a translucent fill, and an unset soft hover copies `inactive`, so M5-07's
      and M6-22's twin does not exist.
  *Closed 2026-10-26:* all three connectors keep the hover and pressed fills a layer (C17), and a preset for a replacing platform states the flattened result, as windows-11 does. iced: 98b522f7 (M5 T6, an unset hover is the widget at rest). gpui: 0c69f9bd (M6 T9, source-over in place of `Hsla::blend`, since M8 T3 core's `color::composite_over`). egui: the M7 ruling (docs/todo_code-review-findings.md:10138, 467d54e6), nothing to change. The windows-11 checkbox hover's unsourced value stays the separate `[1.0]` entry (*Checkbox: a checked checkbox's hover appearance*).

### platform-facts: the Plasma 6.8 rows cite the beta

- [x] [v0.7.0] `docs/platform-facts.md:717`, `:718` and `:1127` state Plasma
      6.8 from its Beta 2 (plasma-workspace `v6.7.91`, `40cde89a`): no
      `forceFontDPI`, no krdb, and `Xft.dpi` from the Xwayland scale; the
      `kde-breeze.toml` header says Breeze's values are unchanged at Breeze
      `v6.7.91`. `v6.8.0` was not tagged on 2026-10-11 (`git ls-remote
      https://invent.kde.org/plasma/plasma-workspace.git refs/tags/v6.8.0`;
      due 2026-10-14). Once it is, re-read the cited lines
      (`c675077b` and `fe367323` in the release,
      `startkde/plasma-setup-xwayland/main.cpp:47-58`) and Breeze's colour
      files and kstyle at `v6.8.0`, then re-pin the three rows and the
      header (found in M2e Step 7, 2026-10-11).
  *Closed 2026-10-26 in cb74515d:* re-pinned to Plasma 6.8.1. The cited lines were re-read at plasma-workspace v6.8.1 (`fb9b679a`): `c675077b` and `fe367323` are in the release (first in v6.7.90), no source reads `forceFontDPI` (only the `plasma6.0-remove-dpi-settings` migration names `forceFontDPIWayland`, to delete it), krdb is gone, and `startkde/plasma-setup-xwayland/main.cpp:47-58` is byte-identical to v6.7.91's. Breeze v6.8.1 (`23c317d0`): no kstyle source changed since v6.7.5 (only `CMakeLists.txt`, `breeze.themerc` and `config/breezestyleconfig.json`), and the colour files gained translation lines only. facts :717, :718, :1127 and the appendix's forceFontDPI row (:1850) cite v6.8.1; the `kde-breeze.toml` header says the kstyle and colour values are unchanged at Plasma 6.8.1.

### Presets: adwaita's list header and slider thumb

- [x] [v0.7.0] Since M2e fix round 2 adwaita leaves `list.header_background`
      and `slider.thumb_color` to inheritance (`defaults.surface_color`,
      `#ffffff` / `#1d1d20`). `list.header_background`: the GNOME cell of
      `docs/platform-facts.md:1429` names "columnview header" but no colour;
      at libadwaita 1.10.0 the header's buttons are transparent
      (`_column-view.scss:12-13`) over the column view's `.view` fill,
      `--view-bg-color` (`_column-view.scss:72-76`, `_views.scss:1-4`).
      State that in the cell, then decide whether the preset states it.
      `slider.thumb_color`: the GNOME cell of `docs/platform-facts.md:1319`
      says ← `defaults.surface_color`, but libadwaita 1.10.0 fills the slider
      with `$slider_color` = `color-mix(in srgb, white 80%,
      var(--view-bg-color))` (`_scale.scss:50`, `_colors.scss:288`), a light
      grey on the dark `#1d1d20` where the preset resolves `#1d1d20`.
      Correct the cell, then the preset follows.
      M8 T21 fix round 1 (2026-10-17): the slider half is closed: §2.9's GNOME
      cell gives `$slider_color` since cbab21c4 (M8 T15), and adwaita states
      it since 859bf526 (M8 T16), `#ffffff` / `#d2d2d2`
      (`adwaita.toml` `[light.slider] thumb_color`, `[dark.slider] thumb_color`); *Presets: values the M8 T15
      platform-facts rows disagree with* records it among the values T16
      stated. The list-header half stays.
  *Closed 2026-10-26 in 48df37e9:* the list-header half. facts :1429's GNOME cell states the colour: the header's buttons are transparent (`_column-view.scss:12-13`) over the column view's `.view` fill, `--view-bg-color` (`_column-view.scss:72-76`, `_views.scss:1-4`), #ffffff / #1d1d20 (`_colors.scss:36`, `:99`, libadwaita 1.10.0). `adwaita.toml` states it (`[light.list] header_background`, `[dark.list] header_background`), as the `list.header_background` wrong safety net requires a preset to (docs/inheritance-rules.toml:488-489, :529-531); kde-breeze and windows-11 already did. The resolved value is unchanged: it equals `surface_color`, which the field inherited. The rule's reason now says GNOME equals surface and KDE is distinct. The slider half closed in M8 (cbab21c4, 859bf526).

### windows-11: accent-derived values stated for the default accent

- [x] [v0.7.0] M4 T35 (2026-10-13) gave windows-11 WinUI 3's brushes where a
      value derives from the accent and no inheritance carries it: the text
      box's focused bottom line `SystemAccentColorDark1` / `Light2` (`#0067c0`
      / `#4cc2ff`, `native-theme/src/presets/windows-11.toml`
      `[light.input] focus_border_color`, `[dark.input] focus_border_color`) and
      the on switch's hover `AccentFillColorSecondary`, those shades at 0.9,
      flattened (`#1875c5` / `#48b2e9`, `[light.switch] hover_checked_background`,
      `[dark.switch] hover_checked_background`), each the UISettings
      shade of the default accent `#0078d4` on the OS probe. On a live system
      with another accent they stay the default accent's: the reader writes
      the shades only into the accent and checked buttons' fills
      (`native-theme/src/windows.rs`, `build_theme`). Decide whether the
      reader writes these two from the same shades (cited as the preset
      cites them). The text selection now inherits the accent
      (`TextControlSelectionHighlightColor` = `SystemAccentColor`), so it
      follows the user's. Still without a source: the dark
      `defaults.selection_text_color` (`#ffffff`, `[dark.defaults] selection_text_color`; `COLOR_HIGHLIGHTTEXT`
      was read in light mode only, and WinUI 3 states no selected-text
      foreground) and the menu's background (`[light.menu]`, `[dark.menu]`): WinUI 3 draws the
      menu on a `DesktopAcrylicBackdrop` whose fallback colour Microsoft does
      not publish; the preset states the fallback of the acrylic brush of the
      same name, `#F9F9F9` / `#2C2C2C` (`docs/platform-facts.md:1253`). Under
      high contrast the preset's stated strokes and fills that the reader
      does not map (`input.hover_border_color`, the check box's and switch's
      off colours, the slider's track and disabled colours) keep
      their light or dark values.
  *Closed 2026-10-26 in c7989e7b (facts 56407e2d):* the reader writes `input.focus_border_color` and the four accent fills (`checkbox.checked_background`, `switch.checked_background`, `slider.fill_color`, `progress_bar.fill_color`) from the UISettings shade the accent and checked buttons take, SystemAccentColorDark1 in light and Light2 in dark, and under high contrast the four from Highlight (CheckBox :145, ToggleSwitch :87, ProgressBar :14; the slider's SystemControlHighlightAccentBrush, generic.xaml:2055) and the switch hover from ButtonFace (ToggleSwitch :88), and the focused text box's line from Highlight (generic.xaml:2658, :2055, the TextBox HighContrast dictionary, TextBox_themeresources.xaml:67-110, having no key of its own; the D2 review). WinUI 3 fills every accent control with `AccentFillColorDefault` (CheckBox_themeresources.xaml:233, :57; RadioButton :146, :30; Slider :128, :24; ProgressBar :22, :6; ToggleSwitch :143, :23 at microsoft-ui-xaml 8463f45), not `SystemAccentColor`; windows-11 states the four as #0067c0 / #4cc2ff. The checked switch's hover is the window at alpha 25/255 laid over the track (#f3f3f319 / #20202019), which composites to WinUI's #1875c5 / #48b2e9 for the default accent and follows any other (`the_windows_11_switch_hover_composites_to_winuis`). The four [uniform] rules are `# see wrong_safety_nets` for Windows and KDE (the controller's ruling: the MUST binds the presets of the platforms an entry's why_wrong names; GNOME, macOS, iOS and Material keep inheriting the accent, but for iOS's on switch, which ios-13 states as systemGreen and the switch's why_wrong names). The notes that have no Microsoft source move to the `[1.0]` entry *windows-11: values WinUI 3 states no source for*.

### Presets: kde-breeze's sidebar colour set

- [x] [v0.7.0] `docs/platform-facts.md` §2.12 gave KDE's sidebar
      `[Colors:Complementary]`, which no sidebar uses at KF 6.31.0; since M2e
      fix round 1 the cells (`:1370`, `:1372`) are ❓. A non-modal Kirigami
      drawer takes the `Window` set (kirigami v6.31.0
      `src/templates/OverlayDrawer.qml:244`), a non-modal `GlobalDrawer` the
      `View` set (`src/controls/GlobalDrawer.qml:337`), and KIO's places
      panel (Dolphin, the file dialog) is transparent over the window with
      `WindowText` (kio v6.31.0 `src/filewidgets/kfileplacesview.cpp:863-866`).
      Decide which the model's sidebar is; then the facts cells, kde-breeze's
      `sidebar.background_color` (`#dee0e2`, `[Colors:Header]
      BackgroundNormal`; dark `#272c31`, `[WM] activeBackground`; both
      marked ❓ "no source"), the KDE reader, which reads `[Colors:Complementary]`
      (`native-theme/src/kde/colors.rs:173-176`, M4),
      `docs/inheritance-rules.toml:549` and the `[Colors:Complementary]`
      description at `docs/platform-facts.md:609` follow. The text colour
      is settled either way: `ForegroundNormal` is the same in both sets.
  *Closed 2026-10-26 in 2caad225 (facts and rules 719680cb):* the model's sidebar is KDE's `View` set, as the KDE HIG's sidebar is the non-modal `GlobalDrawer` (develop-kde-org `cc5482a3` content/hig/layout_and_nav.md:75), which Kirigami docks in `View` (GlobalDrawer.qml:337), and System Settings' sidebar sets `colorSet: View` (systemsettings v6.8.1 app/qml/CategoriesPage.qml:19-20); Dolphin's places panel, transparent over the window, is the exception the cell records. kde-breeze states `[Colors:View] BackgroundNormal`, #ffffff / #141618 (BreezeLight.colors:115, BreezeDark.colors:116 at v6.7.5), and the KDE reader reads `[Colors:View]` in place of `[Colors:Complementary]` (`native-theme/src/kde/colors.rs`, `populate_colors`; the entry's :173-176 having drifted), which on Breeze Light had given a dark #2a2e32 sidebar with #fcfcfc text. The facts cells :1370, :1372 and :609 and docs/inheritance-rules.toml:557-559 (the entry's :549) follow.

### macOS: what the OS probe does not render yet

- [x] [1.0] `macos-tahoe` states its selected segment `#0077f9` / `#077dff`
      (`macos-tahoe.toml` `[light.segmented_control] active_background`,
      `[dark.segmented_control] active_background`), a blue the D round's
      OS probe measured in a key window on macOS 26 and 27
      (`docs/platform-facts.md:1636`). It is accent-coloured, but no NSColor
      the probe reads gives it (`controlAccentColor` #007aff,
      `selectedContentBackgroundColor` #0064e1 / #0059d1), and no reader
      writes it. Whether it follows the user's accent is unmeasured: if it
      does, a live macOS 26 desktop with another accent keeps the default
      blue there. A probe pass with another `AppleAccentColor` set before
      the key-window capture would show it. If the segment follows, find
      what it then matches and have the reader write it; if it does not,
      the stated value stands. (D6/D7 review, M3, 2026-10-26.)
  *Closed 2026-10-26 in cd32a249 (facts ae57aceb, 8973a684):* the segment follows the accent. OS probe run 40348854007's red-accent pass (`AppleAccentColor` 0) draws it #db373c / #ff575b on macOS 26 (#da373d / #ff565b on 27), where `controlAccentColor` reads #e0383e / #ff5257, as the default #0077f9 / #077dff sits by #007aff: within 7 levels each time, as macOS 26's on switch and slider value fill are, which the presets leave to the accent; no NSColor the probe reads matches it exactly. macos-tahoe states no `active_background`, so it inherits `defaults.accent_color`, which the macOS reader writes from `controlAccentColor`; macOS 14 and 15 keep their neutral #ffffff / #666666 under the red accent, and macos-monterey states it (`docs/platform-facts.md:1636`).
