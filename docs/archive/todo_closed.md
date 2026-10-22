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
