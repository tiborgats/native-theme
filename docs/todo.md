# native-theme: TODO

The backlog of native-theme, its connectors and their showcases: one checkbox per
open item, each tagged after its box. `[v0.7.0]`: this cycle does it (the code-review
plan or its M1 ledger schedules it, or it is a small defect in code the review reads);
`[v0.7.x]`: a 0.7 patch release, in ROADMAP.md's order; `[1.0]`: settled before 1.0;
`[post-1.0]`: deferred past 1.0; `[upstream]`: waits on an upstream crate (gpui-kit, iced,
egui, libadwaita, Breeze). Find a milestone's items with
`grep -n '^- \[ \] \[v0.7.0\]' docs/todo.md`. Sections: Core API, Toolkit Connectors,
This cycle (v0.7.0), Post-1.0, Deferred (pre-1.0); items keep their order inside a
section, and a `[v0.7.0]` item that would sit under Deferred (pre-1.0) is under This cycle
(v0.7.0). Closed items move to [`archive/todo_closed.md`](archive/todo_closed.md) as they
close, under the headings they had, each with the commit that closed it. Cite an entry by
its file, its `###` heading and its opening words, quoted, as many as make it unique
under that heading (`docs/todo.md`, *Font DPI*, "Unverified: do the Linux readers'"): a
`docs/todo.md:N` citation is valid at the commit that wrote it
(`git show <commit>:docs/todo.md`), since entries move as they close. A preset
value is cited by its TOML key path, which no preset edit shifts (`adwaita.toml`
`[light.button] active_background`), and a preset line only with the commit it
holds at (`kde-breeze.toml:8` at 2ca74471), as `scripts/check_todo_citations.py`
checks. A bare `:N` continuing a preset's line citation is never written: the script
cannot tell which file a bare number continues, so it does not look for one.

---

## Core API

### Font rendering preferences — read them from the platform

- [ ] [1.0] Research and document the platform font-rendering preferences in
      `docs/platform-facts.md` **first**, cited to authoritative sources, then
      add them to the theme model. Candidates, to be verified rather than
      assumed: fontconfig `hinting` / `hintstyle` / `antialias` / `rgba`
      (`/etc/fonts/`, `~/.config/fontconfig/`), KDE's `XftHintStyle` /
      `XftAntialias` / `XftSubPixel` in `kdeglobals`, GNOME's
      `org.gnome.desktop.interface font-hinting` / `font-antialiasing`,
      Windows ClearType, macOS font smoothing.

      We already read `Xft.dpi` for scaling (`detect::read_xft_dpi`,
      `native-theme/src/detect.rs:459`, in the font-DPI chain the KDE reader shares),
      but nothing reads the *rendering*
      preferences. A Qt or GTK app obeys the user's hinting and antialiasing
      choice; an app built on our connectors silently ignores it. Reading OS
      appearance settings is exactly this crate's remit, so this is a real gap
      rather than a nice-to-have — it is the same shape as reading colours and
      the icon theme.

      Benefits every connector, not just egui: iced and gpui both rasterize
      their own glyphs too.

### Checkbox: a checked checkbox's hover appearance

- [ ] [1.0] Research a checked checkbox's hover appearance on each platform in
      `docs/platform-facts.md` **first**, cited to authoritative sources, then
      add a `checkbox` checked-hover `soft_option` to the theme model.
      `CheckboxTheme` (`native-theme/src/model/widgets/mod.rs:178-249`) has one
      `hover_background` (`:230-232`), with no checked counterpart. All 16
      presets that state colours state it, in both modes (the four `*-live`
      presets, removed since in M1-T4-01, carried geometry only), yet §2.5
      of platform-facts has no hover row at all, so none of those 32 values
      is sourced there, and
      only `windows-11` says what its value is: "Fluent checkbox hover fill"
      (`native-theme/src/presets/windows-11.toml` `[light.checkbox] hover_background`), without saying for
      which box state. Native toolkits do style the checked box under the
      pointer as a state of its own: WinUI3
      gives `CheckBoxCheckBackgroundFillCheckedPointerOver` =
      `AccentFillColorSecondaryBrush` against `AccentFillColorDefaultBrush` at
      rest, in both the `Default` (dark) and `Light` dictionaries
      ([CheckBox_themeresources.xaml:57-58](https://github.com/microsoft/microsoft-ui-xaml/blob/2b8c7757ef2dd57234d5d7c3016d136c59a24bad/controls/dev/CommonStyles/CheckBox_themeresources.xaml#L57-L58),
      [:233-234](https://github.com/microsoft/microsoft-ui-xaml/blob/2b8c7757ef2dd57234d5d7c3016d136c59a24bad/controls/dev/CommonStyles/CheckBox_themeresources.xaml#L233-L234)),
      and libadwaita lays a `color-mix(in srgb, currentColor 10%, transparent)`
      image over a `:checked` check on `:hover`
      ([_checks.scss:52-58](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/1.10.0/src/stylesheet/widgets/_checks.scss#L52-58)).
      KDE Breeze (its source at tree `be6e137e`, which is an l10n commit, so
      the tree is cited, not a change) marks hover on the checked and the
      unchecked box alike, and not with a fill: `Style::drawIndicatorCheckBoxPrimitive` passes
      `mouseOver` to `Helper::renderCheckBox`, which then draws a rounded
      outline in `focusColor(palette)` (a neutral-highlight colour where the
      widget asks for one) with no brush, whatever the check state
      ([breezestyle.cpp:4900-4938](https://invent.kde.org/plasma/breeze/-/blob/be6e137e169e246e67e9a9958d5a4f97bfa69e2e/kstyle/breezestyle.cpp#L4900-4938),
      [breezehelper.cpp:897-908](https://invent.kde.org/plasma/breeze/-/blob/be6e137e169e246e67e9a9958d5a4f97bfa69e2e/kstyle/breezehelper.cpp#L897-908)),
      so what Breeze needs is a hover *border* colour, which `CheckboxTheme`
      has no field for either. Still to research: which Breeze release that
      code shipped in and what `focusColor` resolves to in the Breeze colour
      schemes, whether an AppKit checkbox has a hover state at all, the
      resolved colour of each Fluent brush in light and dark, whether
      `windows-11`'s stated `hover_background` is
      `CheckBoxCheckBackgroundFillUncheckedPointerOver`
      (`ControlAltFillColorTertiaryBrush`, `CheckBox_themeresources.xaml:54`,
      and `CheckBox_themeresources.xaml:230` in `Light`), and which state each of the other presets'
      stated values describes. Until the field exists the
      iced connector (`connectors/native-theme-iced/src/styles.rs:544-547`)
      and the egui connector (its rationale's Q-8, §8) show the plain checked box on
      hover, copying the base state (C16); the new field then becomes the
      value they show, with that copy as its `None` fallback.
      M8 T15 (2026-10-17): §2.5 has a `hover_background` row now, with each
      platform's checked and unchecked hover: WinUI 3
      `CheckBoxCheckBackgroundFillCheckedPointerOver` =
      `AccentFillColorSecondary` (the accent at 0.9) against
      `ControlAltFillColorTertiary` unchecked (CheckBox_themeresources.xaml:234,
      :230 at 8463f45); libadwaita a `currentColor` 10 % layer over a checked
      box and a 20 % ring on an unchecked one; Breeze no fill, a
      `DecorationFocus` ring either way; M3 a state layer around the box;
      macOS ❓. The model field is still the one.
      M8 T21 fix round 1 (2026-10-17): the question whether `windows-11`'s
      stated `hover_background` is `ControlAltFillColorTertiary` is also *M6
      close (2026-10-16): items carried to M7, M8, M9a and the presets*,
      "Presets (windows-11): the checkbox's hover fill", which *Presets:
      values the M8 T15 platform-facts rows disagree with* links, and *Hover
      fills: a layer or a replacement*'s fix round 1 line.

### Tab: WinUI 3's tab separators and its tab bar's bottom rule

- [ ] [1.0] WinUI 3's `TabView` draws two lines around its tabs that the model
      does not state, read at microsoft-ui-xaml 8463f45 (G3 rationale K14 (e),
      (f), `docs/archive/todo_v0.7.0_gpui-kit-0.8.0-rationale.md`). The separator: each
      tab draws a 1px `TabSeparator` at its right edge
      ([TabView.xaml:553](https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/TabView/TabView.xaml#L553)) in `TabViewItemSeparator`
      = `DividerStrokeColorDefaultBrush`
      ([TabView_themeresources.xaml:46](https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/TabView/TabView_themeresources.xaml#L46)),
      `#0F000000` light and `#15FFFFFF` dark
      ([Common_themeresources_any.xaml:257, :53](https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/CommonStyles/Common_themeresources_any.xaml#L257)),
      inset `TabViewItemSeparatorMargin` `0,8,0,8` (TabView_themeresources.xaml:266),
      hidden on the selected tab and on the one left of it
      (`SetTabSeparatorOpacity`, [TabView.cpp:217-231](https://github.com/microsoft/microsoft-ui-xaml/blob/8463f45162149de0ec3ad7df752596893fe3e13e/controls/dev/TabView/TabView.cpp#L217-L231))
      and under the pointer (TabView.xaml:322, :332). The bar's bottom rule: a
      1px line in `TabViewBorderBrush` = `CardStrokeColorDefault`
      (TabView_themeresources.xaml:68, :146) under every tab (`BottomBorderLine`,
      TabView.xaml:546) and beside the tabs, under the strip's header, add
      button and footer (`LeftBottomBorderLine`, `RightBottomBorderLine`,
      :36-37), collapsed under the selected tab (:337).
      Modelling the separator needs a colour, a width, an inset and when it
      hides, in the three connectors (a gpui `TabLook` field, an iced contract
      row with its `contract-leaves.toml` entries, an egui mapping row); the
      rule needs no new field, since its brush is `tab.border.color`'s, but it
      gives `tab.border`, the selected tab's outline, a second meaning (a rule
      under the whole bar) and meets the showcases' own `separator.*` rule under
      their page tabs. The selected tab's outward corners, the same `TabView`'s,
      are `tab.active_outward_corner_radius` (`docs/platform-facts.md` §2.11),
      which leaves both lines out; §2.11's `item_gap` row records the separator
      as one "which no `tab` field models".

### Tab: the border is the selected tab's, and KDE's live tab colours

- [ ] [1.0] `kde-breeze` now states `tab.border.color`, `line_width_px`,
      `corner_radius_px` and `shadow_enabled` (`docs/platform-facts.md:1354-1357`,
      §2.11). Breeze strokes only the selected tab; an unselected tab has no
      pen and rounds 5 on the row's outer ends only (breezehelper.cpp:1488-1507,
      breezestyle.cpp:7248-7253), and Windows' cell (`:1341`) is also
      selected-only. `TabTheme::border` (`native-theme/src/model/widgets/mod.rs`)
      does not say which tab its colour, width and radius belong to: document
      that it is the selected tab's, and check that each connector strokes the
      active tab only. `tab.hover_background` (`#3daee933`) replaces the
      unselected tab's fill rather than tinting it (breezehelper.cpp:1497-1502),
      so it is painted over what lies under the tab, the window.
      gpui-component 0.6.6 paints an idle tab `transparent` in every variant
      (`tab/tab.rs:132, 143, 150, 155, 160`) and nothing reads `tokens.tab`, so
      the gpui connector cannot show `tab.background_color`; its contrast
      contract records `kde-breeze/dark` as an exception for the `tab label`
      pair (`IDLE_TAB_SURFACE`, `connectors/native-theme-gpui/src/contract.rs`).
      On the live path the KDE reader (`native-theme/src/kde/colors.rs`) sets no
      `tab` colour, so the full preset's stock-Breeze values
      (`Window.darker(120)`, `mix(Window, WindowText, 0.2)`) stand for any colour
      scheme; computing them from the user's scheme needs `QColor::darker`
      ([qcolor.cpp:2993-3004](https://github.com/qt/qtbase/blob/ef55f427f2c8b410d34f8a7681020a3000cf6866/src/gui/painting/qcolor.cpp#L2993-L3004))
      and `KColorUtils::mix`
      ([kcolorutils.cpp:144-165](https://invent.kde.org/frameworks/kguiaddons/-/blob/7c766f6f99238c13acb8c50d8391600e964f3b8a/src/colors/kcolorutils.cpp#L144-L165))
      ported to the reader, with `[KDE] frameContrast` read from kdeglobals
      (default 0.2, [kcolorscheme.cpp:529-538](https://invent.kde.org/frameworks/kcolorscheme/-/blob/27066d471c93629efee8459d9f69490caa92b7c4/src/kcolorscheme.cpp#L529-L538)).

### Segmented control: the join and the divider

- [ ] [1.0] Research in `docs/platform-facts.md` **first**, cited to authoritative
      sources, how each platform's segmented control joins its segments —
      one outline around the row, and whether a segment's corners that meet a
      divider are rounded — and the colour of the divider line between
      segments, then add what the platforms state to `SegmentedControlTheme`
      (`native-theme/src/model/widgets/mod.rs:944-976`), which states the
      divider's width, `separator_width` (`docs/platform-facts.md:991`), and
      no colour or join. Until the model states them, the egui widgets crate
      draws its segments `separator_width` apart as separate segments, with no
      divider line (`docs/archive/todo_egui-widgets-spec.md` §4.4); nothing is to be
      invented in the meantime.

### Checkbox: the check mark's glyph and size

- [ ] [1.0] Research each platform's check mark — its glyph, its size inside the
      box and its stroke width — in `docs/platform-facts.md` §2.5 **first**,
      then add to `CheckboxTheme` what the platforms state. The model has no
      field for the mark: `indicator_width` is the box
      (`docs/platform-facts.md:984`), and §2.5 says of the mark only that it
      fills the indicator, except for GNOME's `padding: 3` (`:1237-1238`).
      Each toolkit draws its own: egui a three-point line in an 8 px square
      (`Spacing::icon_width_inner`, `egui/src/style.rs:1467`, read at
      `widget_style.rs:180`; drawn at `widgets/checkbox.rs:149-158`) in the
      widget's `fg_stroke` (`widget_style.rs:188`); Breeze (its source at tree
      `be6e137e`) a three-point path
      at fixed offsets inside the frame, in the text colour at twice the frame
      pen width ([breezehelper.cpp:912-927](https://invent.kde.org/plasma/breeze/-/blob/be6e137e169e246e67e9a9958d5a4f97bfa69e2e/kstyle/breezehelper.cpp#L912-927),
      [:938](https://invent.kde.org/plasma/breeze/-/blob/be6e137e169e246e67e9a9958d5a4f97bfa69e2e/kstyle/breezehelper.cpp#L938)). Still to read: WinUI 3's, libadwaita's and
      AppKit's. Until the model states a mark, the egui connector leaves
      `icon_width_inner` at egui's `8.0` on its base style, and in its
      Checkbox cell writes `indicator_width` less the mean of the two stated
      `checkbox.border.padding` pairs, the mark's inset, where a theme states
      all four sides (`docs/archive/todo_v0.6.0_egui-connector-spec.md` §6.11). Filling that
      field from a number in `docs/platform-facts.md` that no model field
      carries is rejected: the connector would then carry a theme value
      the model does not, which is the hardcoded value the project's rules
      forbid; a platform value reaches a connector only through a model field
      that the readers and presets fill.

### Radio: Material's radio is not the checkbox's shape

- [ ] [1.0] Decision for the maintainer (found 2026-09-28, while adding
      `checkbox.radio_dot_diameter`). Material 3's radio differs from its
      checkbox in size and in shape, which the model's shared `CheckboxTheme`
      cannot state: `md.comp.radio-button.icon-size` is 20px
      ([material-web v2.5.0 `_md-comp-radio-button.scss:34`](https://github.com/material-components/material-web/blob/v2.5.0/tokens/versions/v0_192/_md-comp-radio-button.scss#L34))
      while `md.comp.checkbox.container-size` is 18px (`_md-comp-checkbox.scss:32`),
      and the `material-3` preset's one `indicator_width_px = 18.0` makes the
      radio 18; and a checked Material radio is an unfilled ring and a dot,
      both `md.sys.color.primary` (`radio/internal/_radio.scss:126-127`), not
      a filled accent disc with an on-accent dot. Neither token set has a dot
      size: material-web's SVG draws a 10px dot (`radio.ts:108-119`,
      `r="5"` in a 20px viewBox) and Compose `RadioButtonDotSize = 12.dp`
      drawn at radius 6 − 1 (`RadioButton.kt:320-323`), both implementation
      constants. `docs/platform-facts.md` has no Material column, and
      `native-theme/src/presets/documented_sizes.rs:12-15` keeps the
      `material-3` preset from stating a size no platform-facts row documents,
      so the preset states no `radio_dot_diameter` and each toolkit draws its
      own dot. Carrying Material's radio would need a radio indicator width
      and radio-specific colours (or a ring-style flag) in the model.

### Theme watcher: OS changes it does not report

- [ ] [v0.7.x] **Windows watcher: the window-only sources.** The reader
      reads the non-client fonts (`SPI_GETNONCLIENTMETRICS`), the
      `GetSysColor` colours and `DwmGetColorizationColor` (the latter two
      until T22 drops them), whose changes Windows announces only to
      top-level windows: `WM_SETTINGCHANGE` with the `SPI_SET*` action in
      `wParam`
      (<https://learn.microsoft.com/en-us/windows/win32/winmsg/wm-settingchange>),
      `WM_SYSCOLORCHANGE`
      (<https://learn.microsoft.com/en-us/windows/win32/gdi/wm-syscolorchange>)
      and `WM_DWMCOLORIZATIONCOLORCHANGED`
      (<https://learn.microsoft.com/en-us/windows/win32/dwm/wm-dwmcolorizationcolorchanged>).
      A message-only window "does not receive broadcast messages"
      (<https://learn.microsoft.com/en-us/windows/win32/winmsg/window-features#message-only-windows>).
      Watching them takes a hidden top-level window (not `HWND_MESSAGE`) on
      the watcher thread, whose window procedure posts `WM_THEME_CHANGED` for
      those messages (the pump already coalesces it), destroyed before
      `CoUninitialize` (`native-theme/src/watch/windows.rs`).
      M4 Step 8 (2026-10-13): since 8c133614 the reader reads the `GetSysColor`
      colours under high contrast only and `DwmGetColorizationColor` not at
      all, so outside high contrast the non-client fonts are the source left
      here. High contrast turning on or off is reported by
      `AccessibilitySettings::HighContrastChanged` since 71db4793; whether a
      colour change inside a contrast theme reaches it is unverified.
      G1 Task 5b (2026-10-14): it is not. Outside UWP the event "never
      fires", as it relies on the `CoreWindow` to handle `WM_THEMECHANGED`
      (<https://github.com/microsoft/WindowsAppSDK/blob/main/specs/themes/ThemeSettings.md>),
      and the CI's `windows-latest` refused the registration with "Element
      not found" (0x80070490, run 38478340750). The watcher now subscribes
      where Windows accepts it and starts without it, and
      `SPI_GETHIGHCONTRAST` is listed among the reader's sources not watched:
      high contrast is one of this entry's window-only sources. The same spec
      implements its Win32 replacement, `ThemeSettings`, with a top-level
      window that receives `WM_THEMECHANGED`.
      Correction (2026-10-14, Task 5b review): `ThemeSettings` does not
      create a window; it listens for `WM_THEMECHANGED` on an existing
      top-level window the application passes as a `WindowId` (the spec's
      "Why does this API require a WindowId?": "An alternative option is for
      the API to create and manage its own top-level window, but we decided
      to use an existing window"). The window-creating design the spec cites
      is .NET's `SystemEvents`, the one this entry's hidden top-level window
      follows.
- [ ] [v0.7.x] **macOS watcher: check on a Mac which center posts the
      colour and scroller notifications.** The watcher observes
      `NSSystemColorsDidChangeNotification` and
      `NSPreferredScrollerStyleDidChangeNotification` on the default
      `NSNotificationCenter`, an inference: Apple's pages name no center,
      and no test sees a real AppKit post (the tests post their own). Change
      the accent colour and "Show scroll bars" in System Settings with an
      application running its main run loop, and see that the callback
      fires (`native-theme/src/watch/macos.rs` `registrations`).
- [ ] [v0.7.x] **iced on macOS: the watcher started from iced's executor
      thread.** The iced connector's `theme_changes()` (feature `watch`)
      calls `native_theme::watch::on_theme_change` inside its subscription's
      stream (`connectors/native-theme-iced/src/watch.rs:63-64`), so the
      macOS observer is registered on the executor thread that first runs
      the stream, not on the main thread (M5 T3, 80b9ad13). The watcher's
      notifications are delivered through the process's run loop, usually
      the main thread's, which the application must run
      (`native-theme/src/watch/macos.rs:23-28`; M4 T3), as iced's event loop
      does; the observer has no main-thread requirement of its own (it
      queues the callback on a private `NSOperationQueue`, `:29-33`). So
      delivery is expected but unverified: no test sees a real AppKit post.
      With the check above, run the iced showcase with `--features watch`
      on a Mac in the System colour mode, change the appearance and the
      accent in System Settings, and see the showcase follow (the CI macOS
      leg or the maintainer).


### platform-facts: macOS vertical padding is measured outside the border

- [ ] [1.0] `docs/platform-facts.md` derives macOS `border.padding_vertical` as the
      outer height less the content height, halved — "3 **(measured)**
      (22−16)/2" for the button (`docs/platform-facts.md:1185`), and the same
      shape for the text input (`:1214`), the menu (`:1262`), the tab
      (`:1353`) and the list (`:1434`). Outer less content is border plus padding, yet the outer-box
      rule counts border, padding and content as separate parts
      (`docs/platform-facts.md:901-906`), and a padding is what lies inside the
      border. So each of those cells includes the border's width on its side:
      `macos-monterey` states the button's `padding_top_px = 1.0` and
      `padding_bottom_px = 3.0` (`native-theme/src/presets/macos-monterey.toml`
      `[light.button.border] padding_top_px, padding_bottom_px`,
      `[dark.button.border] padding_top_px, padding_bottom_px`) with a
      `line_width_px = 0.5` border
      (`[light.defaults.border] line_width_px`, `[dark.defaults.border] line_width_px`). Research first, from
      source or measurement with citations, what the 22 and 16 measure — the
      outer box with its border, and the text's line box or its glyph height —
      then correct platform-facts' derivation of each cell and, from it, the
      preset. Until then a connector that adds the border's width to the
      stated padding, as the egui connector does, counts that border twice,
      one border width too much on each side of those macOS controls.
      M2a Step 7 (2026-10-11) took the button's row from the OS probe:
      AppKit's title rect sits 1 below the top and 3 above the bottom of the
      20-tall bezel, measured from its outer (alignment-rect) edge
      (`docs/platform-facts.md:1185`), and the pop-up button's likewise
      (`:1623`). Those are offsets from the outer edge, so the border's
      share is still open on those two rows, as on the text input, menu, tab
      and list rows.
      M2a Step 8 (2026-10-11) took the text input's row from the probe too:
      the title rect sits 2 below the top and 2 above the bottom of the
      21-tall field on macOS 14.8.9 (`docs/platform-facts.md:1214`), from the
      outer edge again, so its border's share is open as the other two's.

### Presets: a spinner stroke width platform-facts does not give

- [ ] [1.0] `docs/platform-facts.md` §2.23 gives the spinner a stroke width on
      Windows only — macOS draws fins, KDE and GNOME rotate an icon — and
      states **(none)** for the other three (`docs/platform-facts.md:1601`),
      yet `macos-monterey`, `kde-breeze` and `adwaita` each state
      `stroke_width_px = 2.0` in both variants
      (`native-theme/src/presets/macos-monterey.toml`,
      `native-theme/src/presets/kde-breeze.toml` and
      `native-theme/src/presets/adwaita.toml`, each `[light.spinner] stroke_width_px`
      and `[dark.spinner] stroke_width_px`). Run the
      `preset-validator` agent over the three presets against §2.23. The
      field is required in `ResolvedSpinnerTheme` — every preset must resolve
      it — so removing the value needs a decision in the model first: make
      `stroke_width` optional, with a connector keeping its toolkit's own
      stroke for `None`, or record in §2.23 a source for a ring's stroke on
      those platforms. The egui widgets crate paints its spinner arc from this
      field where the icon set has no animated indicator
      (`docs/archive/todo_egui-widgets-spec.md` §4.3).
      Found again 2026-09-28 (iced review): §2.23 gives KDE and GNOME no
      `min_diameter` either (**(none)**, `docs/platform-facts.md:1600`), yet
      `kde-breeze` and `adwaita` (and their `-live` twins, until M1-T4-01
      removed them) state `min_diameter_px = 16.0` in both variants.
      `min_diameter` is required in `ResolvedSpinnerTheme` too
      (`native-theme/src/model/widgets/mod.rs`, `SpinnerTheme`, no
      `soft_option`), so the same model decision covers both
      fields; the presets keep the values until it is made.

### Presets: the scrollbar thumb colour platform-facts measures

- [ ] [1.0] `docs/platform-facts.md` §2.8 measures `scrollbar.thumb_color` for
      macOS, `#80808080` (Sonoma), and for Windows, `#c2c2c2`
      (`docs/platform-facts.md:1305`), yet neither `macos-monterey` nor
      `windows-11` states it: their scrollbar sections state
      `thumb_hover_color` and `thumb_active_color` (and macOS the overlay
      `track_color`) but not it
      (`native-theme/src/presets/macos-monterey.toml` and
      `native-theme/src/presets/windows-11.toml`, `[light.scrollbar]` and
      `[dark.scrollbar]`). No
      bundled preset states it, and for KDE and GNOME §2.8 names a source,
      "(Breeze src)" and "(Adwaita CSS)", but no value. The field then
      resolves to `defaults.muted_color` (`docs/inheritance-rules.toml:205`),
      a fallback the same file lists among its wrong safety nets, a text
      colour for "a semi-transparent UI control" (`:559-561`), and the egui
      connector's base style fills unscoped `ScrollArea` handles, checkbox
      and radio boxes and slider rails with it
      (`docs/archive/todo_v0.6.0_egui-connector-spec.md` §5.9). The measurement is one
      value per platform, while the presets' hover colours differ by variant
      on macOS (`#60606080` light, `#a0a0a080` dark,
      `native-theme/src/presets/macos-monterey.toml` `[light.scrollbar] thumb_hover_color`,
      `[dark.scrollbar] thumb_hover_color`) and not on
      Windows (`#a0a0a0`, `native-theme/src/presets/windows-11.toml`
      `[light.scrollbar] thumb_hover_color`, `[dark.scrollbar] thumb_hover_color`):
      state the measured value in the two light variants, find a
      source for each dark one before stating it, and run the
      `preset-validator` agent over both presets against §2.8.
      *M4 T35 (2026-10-13):* windows-11 light states the measured `#c2c2c2`
      (`native-theme/src/presets/windows-11.toml` `[light.scrollbar] thumb_color`);
      dark still has no source (`[dark.scrollbar]` states none). WinUI 3 has one: `ScrollBarThumbFill` and
      `ScrollBarThumbFillPointerOver` are both `ControlStrongFillColorDefault`,
      `#72000000` / `#8BFFFFFF` (ScrollBar_themeresources.xaml:138-139, :26-27;
      Common_themeresources_any.xaml:226, :22, microsoft-ui-xaml 8463f45; now
      beside the measurement in `docs/platform-facts.md:1305-1306`). The
      preset states what WinUI 3 draws elsewhere, but its measured hover and
      pressed thumbs (`#a0a0a0`, `#787878`) are lighter or darker than that
      brush, and WinUI does not change the thumb on hover: decide whether
      windows-11's scroll bar takes the WinUI brushes (rest, hover and
      pressed, both modes) or the measured set, then state the dark thumb.
      *M4 T24's scroll bar validator (2026-10-13):* `macos-tahoe` states no
      light `thumb_color` and no `thumb_hover_color` in either mode
      (`native-theme/src/presets/macos-tahoe.toml` `[light.scrollbar]`; the dark
      `thumb_color` this line cited, `macos-tahoe.toml:576` at 2ca74471, is
      `[dark.slider] thumb_color`, and `[dark.scrollbar]` states neither), so they fall to the safety net here too;
      `macos-monterey`'s `thumb_active_color` (`#50505090` / `#b0b0b090`,
      `native-theme/src/presets/macos-monterey.toml` `[light.scrollbar] thumb_active_color`,
      `[dark.scrollbar] thumb_active_color`) is a soft
      option with no source. Two citations were one line off: the dark scroll
      bar this entry cited as `macos-monterey.toml:640-654` was `:641-655` at 2ca74471, and
      the egui entry's dark `track_color` (`:646`) was `:647`. The validator also
      named `docs/platform-facts.md:1307` and `:181` as "citing probe 15" (its
      words; `:1307` gives the macOS 14.8.9 probe's legacy 15): re-check what
      each should cite now that the reader reads the width live (e6c5c737).

### Fonts: find a system face by family, weight and style

- [ ] [1.0] A name-only lookup beside `system_face`: `native_theme_iced::system_font_family` (`connectors/native-theme-iced/src/lib.rs:1504`) keeps only `SystemFace::family`, yet `system_face` copies the chosen face's whole file into an `Arc<[u8]>` (`native-theme/src/fonts.rs:206-208`; for a `.ttc` face, the whole collection). Measured on 2026-09-27 on a KDE Plasma desktop (release build, database already loaded, its first load 0.88 s): kde-breeze's `Noto Sans` copies 621 572 bytes in 0.10 ms per call and `Hack` 309 408 bytes in 0.08 ms; adwaita's `Adwaita Sans` 879 796 bytes in 0.13 ms and `Adwaita Mono` 1 419 152 bytes in 0.18 ms. The workaround in place is the iced showcase's cache, which calls it, through `native_theme_iced::font`, once per family, weight, style and monospace flag (`font_from_database` and its memo `FONTS`, in `connectors/native-theme-iced/examples/showcase-iced.rs`), and the function's own doc, which says to call it when the theme changes, not per frame. The fix is a spec change for a later version: a `native_theme::fonts::system_face_family(family, weight, style) -> Option<Arc<str>>` over the same `select_face` and the same macOS file route without `with_face_data`, `system_face` built on it plus the copy, and iced's `system_font_family` calling it, after which the showcase's cache can go.
  M8 T7 (2026-10-16): iced's `font(spec)` (6e9c2121) is a second caller that pays the copy on every call, twice the face selection on a miss (the stated family's failed lookup, then the substitute's lookup and copy, since 2fcc26fb), so its doc says to build fonts when the theme changes; with `system_face_family` it would be safe to call per frame, as gpui's `font_family` and `geometry` builders are.
  M9b T12 (2026-10-22): the iced showcase's cache is now `FONTS`, filled by `font_from_database` (d91ff927) once per stated family, weight, style and monospace flag, and `resolved_family` is gone; a fill calls `native_theme_iced::font` and, to tell a face of the stated family from the platform's substitute, `system_face` for the stated family, so it pays two face selections and two copies where the system has the family, and three selections and one copy where it has not, which `system_face_family` would make copy-free.

### Icons: recolour a Breeze `current-color-scheme` stylesheet from the theme, as KIconLoader does

- [ ] [1.0] A Breeze SVG icon carries a `<style id="current-color-scheme">` whose
      `.ColorScheme-*` classes set the `color` its `currentColor` paints take
      (`/usr/share/icons/breeze-dark/actions/16/document-open.svg`:
      `.ColorScheme-Text { color: #fcfcfc; }`). KDE's own loader replaces that
      stylesheet from the palette: KIconLoader colours SVG icons from
      `QGuiApplication::palette()` and follows it
      (`/usr/include/KF6/KIconThemes/kiconloader.h:748-754`, KIconThemes
      6.30.0), and `KIconColors::stylesheet` specifies the `.ColorScheme-Text`,
      `-Background`, `-Highlight`, `-HighlightedText`, `-PositiveText`,
      `-NeutralText`, `-NegativeText` and `-Accent` classes
      (`/usr/include/KF6/KIconThemes/kiconcolors.h:153-161`). native-theme's
      `FreedesktopLoader` hands such an icon on unchanged
      (`native-theme/src/freedesktop.rs:521-524`), so under a KDE colour
      scheme other than Breeze's own every Breeze icon whose stylesheet sets
      its `color` keeps Breeze's stock colours, in all three connectors.
      KIconLoader does this only for an icon theme whose `index.theme` sets
      `FollowsColorScheme=true`, as `breeze` and `breeze-dark` do
      (`/usr/share/icons/breeze/index.theme:120`,
      `/usr/share/icons/breeze-dark/index.theme:120`; `kiconloader.cpp:719-720`,
      `kicontheme.cpp:440`; kiconthemes 6.30.0 source). It replaces the
      `<style id="current-color-scheme">` element's text (`processSvg`,
      `kiconloader.cpp:668-711`) with, in the normal state: Text ←
      `palette.windowText`, Background ← `window`, Highlight ← `highlight`,
      HighlightedText ← `highlightedText`, Accent ← `accent`, and
      Positive/Neutral/NegativeText ←
      `KColorScheme(QPalette::Active, KColorScheme::Window)`'s foregrounds
      (`kiconcolors.cpp:82-100`, `:122-130`). In the selected state Text and
      the three status classes take `highlightedText`, Background and
      HighlightedText take `highlight`, Highlight takes `highlightedText`, and
      Accent is mixed 85 % `accent`, 15 % `highlightedText` (`:114-130`). Map each class to its
      `ResolvedTheme` leaf. The KDE reader takes `text_color` from
      `[Colors:Window] ForegroundNormal` but the status colours from
      `[Colors:View]` (`native-theme/src/kde/colors.rs:18`, `:23`, `:39-43`),
      so check which group Qt's palette fills from. Then `FreedesktopLoader`
      takes those colours, not the one colour `color` takes, which would turn
      all eight classes into one colour
      (`/usr/include/KF6/KIconThemes/kiconcolors.h:40-42`), and rewrites the
      stylesheet for a theme that follows the colour scheme.

### platform-facts: which part of a control the focus ring surrounds

- [ ] [1.0] `docs/platform-facts.md` states each platform's focus-ring colour, width
      and offset (§2.1.5) but not which part of a checkbox, radio button or
      slider the ring surrounds — the indicator or knob alone, or the whole
      control with its label. The egui connector's focus ring surrounds those
      three, and the egui widgets crate's painted `Slider`, around their whole
      response until this is recorded (`docs/archive/todo_egui-widgets-spec.md` §2.5).
      Research it per platform from
      source (Breeze's style, libadwaita's CSS, WinUI's templates, AppKit's
      measured rendering) with citations, then register the shape there.

### Selection: the text colour macOS pairs with the unemphasised selection

- [ ] [1.0] `defaults.selection_inactive_background` is the selection fill of a
      window that has lost focus. Only macOS states it
      (`docs/platform-facts.md:1061`, `unemphasizedSelectedContentBackgroundColor`;
      the reader, `native-theme/src/macos.rs:107`), and `ResolvedTheme` has no
      text colour to pair with it: a toolkit that swapped the fill in on focus
      loss would paint the active `selection_text_color` on it. The egui
      connector grades the leaf UNMAPPABLE `source-side gap` for that reason
      (`docs/archive/todo_v0.6.0_egui-connector-spec.md` §14). Research which text
      colour AppKit draws on the unemphasised selection — `docs/platform-facts.md`
      first, then Apple's documentation, with citations; it is unresearched,
      and no `NSColor` name is assumed here — and add it as a leaf beside
      `selection_inactive_background` (property registry, inheritance rules,
      platform-facts, presets, the macOS reader). Then the egui connector's
      install plugin swaps both, fill and text, while the window is unfocused
      (`ctx.input(|i| i.focused)`).
      A question, not a claim: the macOS reader also writes the unemphasised
      colour into `input.selection_background` (`native-theme/src/macos.rs:490`,
      `:502`), where platform-facts gives an input's selection as
      `← defaults.text_selection_background` (`docs/platform-facts.md:1210`),
      `selectedTextBackgroundColor` on macOS (`:1062`). Neither the reader's
      comments nor the commit that added the assignment (`d87483c8`) say why a text
      field's selection takes the unfocused colour. Is it intended? Fix the
      reader or record the reason.
      Answered (M4 Step 6, 2026-10-12): c1704d2f stopped the reader writing
      `input.selection_background`, and
      `read_macos_leaves_the_text_field_selection_to_inheritance`
      (`native-theme/src/macos.rs:1017`) pins it, so the field inherits
      `defaults.text_selection_background`, as `docs/platform-facts.md:1210`
      gives it. The research task above (the text colour on the unemphasised
      selection) stays open.

### Presets: colours with no reproducible source

- [ ] [1.0] These preset colours are reproduced by no platform computation, and
      stay until a source is found (found 2026-09-28, while checking the
      border opacity):
      kde-breeze `defaults.border.color = "#bcc0bf"` (light) is not
      `mix(Window, WindowText, frameContrast 0.2)` = `#c6c8c9`, the colour
      Breeze draws its frames and separators in (`docs/platform-facts.md`
      §2.11 `border.color`); dark `#4d545b` is not `#4c4e51`. The same
      `#bcc0bf` / `#4d545b` are kde-breeze's `slider.disabled_fill_color`,
      `splitter.hover_color`, `switch.unchecked_background` and (at alpha
      `80`) `switch.disabled_unchecked_background`, and the switch's and
      slider's other disabled colours (`#dee0e2`, `#3daee980`, `#fcfcfc80`;
      dark `#2d3035`) are not KColorScheme's Disabled palette either
      (§2.1.6 `disabled_opacity` gives the palette; Breeze has no
      QtWidgets switch). adwaita `defaults.border.color` `#d5d5d5` / `#4a4a4e`
      is not `currentColor` at `--border-opacity` 15 % over the window,
      `#dcdcde` / `#434347` (window text `RGB(0 0 6 / 80%)` over `#fafafb`,
      white over `#222226`). windows-11 dark `defaults.border.color` and
      `disabled_text_color` `#454545` match no WinUI brush
      (`ControlStrokeColorDefault` dark is `#12FFFFFF`,
      `TextFillColorDisabled` dark `#5DFFFFFF`, Common_themeresources_any.xaml).
      Find each one's source, or replace it with the computed platform
      value, citing the computation.
      *M2b Step 7b (2026-10-11):* windows-11's `disabled_text_color` is now
      `TextFillColorDisabled` as the resource gives it (`#0000005c`, dark
      `#ffffff5d`); its dark `defaults.border.color` is still open.
      *M2b fix round 2 (2026-10-11):* windows-11's `defaults.border.color` is
      now `CardStrokeColorDefault` as the resource gives it, `#0000000f` light
      and `#00000019` dark (Common_themeresources_any.xaml:250, :46,
      microsoft-ui-xaml 8463f45; `docs/platform-facts.md:1055`), so its dark
      `#454545` is closed. `splitter.hover_color` keeps the old `#e5e5e5` /
      `#454545` with no source ❓: WinUI 3 has no draggable divider
      (`docs/platform-facts.md:1468`); it is for M3 with the other soft
      options that have no facts row.
      *M2e Step 7 (2026-10-11):* adwaita's `defaults.border.color` is now the
      computed `#dcdcde` / `#434347`, and its popover outline libadwaita
      1.10.0's `RGB(0 0 0 / 5%)`. Still without a source, each marked ❓ in
      `adwaita.toml`'s light defaults: `text_color` `#2e3436` (libadwaita
      1.10.0 `window_fg_color` is `RGB(0 0 6 / 80%)`, `#323237` over
      `#fafafb`, `_colors.scss:32-33`), `warning_text_color` `#2e3436`
      (`warning_fg_color` `RGB(0 0 0 / 80%)`, `#2e2102` over `#e5a50a`,
      `:19-20`), `muted_color` `#929292` (`.dimmed`, and the deprecated
      `.dim-label`, is the text at `--dim-opacity` 55 %, `#8c8c8f` over the
      window, `_labels.scss:14`,
      `_colors.scss:319`), `link_color` `#1b6acb` (`$link_color` is
      `--accent-color`, `oklab(from #3584e4 min(l, 0.5) a b)`, computed
      `#0461be`, `_colors.scss:146`, `:168-170`, `:265`), `focus_ring_color`
      `#3584e4` (`$focus_border_color`, that accent colour at 50 %, `:307`)
      and `shadow_color` `#00000018` (libadwaita has no one shadow colour;
      `shade_color` is `RGB(0 0 6 / 7%)`, `:79`). The GNOME colour cells of
      `docs/platform-facts.md` §2.1.3-§2.1.5 state no value: give them their
      libadwaita 1.10.0 values first, then the preset. adwaita's dark
      defaults are unchecked, and its explicit copies of the old line colour
      (`#d5d5d5` / `#4a4a4e`: separator, splitter, checkbox) stay until their
      rows are checked; so do kde-breeze light's widget-level `#fcfcfc`
      on-selection colours (`menu.hover_text_color`,
      `segmented_control.active_text_color`: `[Colors:Selection]
      ForegroundNormal` by platform-facts §2.6 and §2.25, `#ffffff` in
      Breeze 6.7).
      *M2e fix round 1 (2026-10-11):* kde-breeze's `separator.line_color`
      (`#c4c8c6` / `#3e4550`, no source) is no longer stated: the separator
      inherits `defaults.border.color` (`docs/platform-facts.md:1476`), so
      the unsourced `#bcc0bf` / `#4d545b` above is now its line colour too.
      kde-breeze light's `button.active_text_color`, the text on the accent
      fill, is `#ffffff` (`[Colors:Selection] ForegroundNormal`,
      BreezeLight.colors:95 at v6.7.5).
      *M2 close (2026-10-11):* ios-13's `slider.disabled_track_color`,
      `#c7c7cc` / `#38383a`, has no source and stands out more than the
      sourced enabled track, opaqueFill `#e4e4e5` / `#2b2b2e`
      (`native-theme/src/presets/ios-13.toml`, `[light.slider]` and
      `[dark.slider]`): darker than it in light, lighter in dark. For M3,
      with the other soft-option colours that have no source.
      *M4 T36 (2026-10-13):* kde-breeze's disabled switch thumb, the
      `#fcfcfc80` above, is the handle's Disabled shade since 74fc414d,
      Disabled `[Colors:Button] BackgroundNormal` (`#f0f0f0` / `#272a2e`,
      `docs/platform-facts.md:1523`). kde-breeze's `defaults.border.color` is
      still open, and two facts cells disagree on it:
      `docs/platform-facts.md:1055` gives KDE's `border.color` as "(preset) —
      derived from background", while `:1476` gives the separator, which
      inherits it, as Breeze's `separatorColor()`, mix(Window, WindowText,
      frameContrast). State the computed mix in the preset and the `:1051` cell
      together.
      *M8 T16 (2026-10-17):* kde-breeze's `switch.unchecked_background` is
      qqc2-desktop-style's off track, #e6e7e7 / #3e4144 (90672c08,
      `docs/platform-facts.md` §2.21). *Fix round 1:* adwaita's
      `focus_ring_color` is the computed `$focus_border_color`, #0461be80 /
      #81d0ff80, which §2.1.5 now gives (b70a5968, cff0bc68).
      M8 T21 fix round 1 (2026-10-17): kde-breeze's
      `slider.disabled_fill_color`, `splitter.hover_color` and the switch's
      disabled colours above are also items of *Presets: values the M8 T15
      platform-facts rows disagree with* (kde-breeze, §2.9, §2.17 and §2.21).

### Disabled: presets that state both mechanisms

- [ ] [1.0] The disabled rule (`docs/platform-facts.md` §2.1.6, 2026-09-28) has
      connectors apply the disabled colours *and* `disabled_opacity`, with
      the data making the one the platform does not use an identity.
      kde-breeze, windows-11, adwaita and material (Material 3 dims by
      colour: `disabled-container-opacity` 0.12 and
      `disabled-label-text-opacity` 0.38 on the parts,
      material-web cbd34a8 `_md-comp-filled-button.scss:42-46`,
      `button/internal/_shared.scss:141-150`) follow it. macos-monterey
      (0.3, and `disabledControlTextColor` colours; PF §2.1.6 says the
      measured ≈0.25–0.3 is the text colour's alpha, "not global opacity",
      and the research found no source for bezel dimming), ios-13 (0.3) and
      the community presets (catppuccin ×4, dracula, gruvbox, nord,
      one-dark, solarized, tokyo-night: 0.5) state both a disabled opacity
      below 1 and disabled colours, so a disabled control is dimmed twice.
      Settle each from its source, then state 1.0 or drop the disabled
      colours.
      M8 T21 (2026-10-17): macos-monterey states `disabled_opacity = 1.0`
      since be69481a (M2a; `macos-monterey.toml` `[light.defaults] disabled_opacity`,
      `[dark.defaults] disabled_opacity`), so of the
      presets above only ios-13 (0.3, `ios-13.toml` `[light.defaults] disabled_opacity`,
      `[dark.defaults] disabled_opacity`) and the ten
      community presets (0.5) still state both.

### KDE: the live reader's disabled text colour

- [ ] [1.0] `native-theme/src/kde/colors.rs` reads `defaults.disabled_text_color`
      from `[Colors:View] ForegroundInactive`; Breeze paints disabled text
      from the palette's Disabled group, which KColorScheme derives with the
      scheme's `[ColorEffects:Disabled]` (`docs/platform-facts.md` §2.1.3,
      §2.1.6: Breeze `#a0a1a3`, Breeze Dark `#686a6c`, which kde-breeze
      states). Port KColorScheme's `StateEffects` (`kcolorscheme.cpp:36-101`
      at 27066d47: Fade by `mix`, Darken in KColorUtils' HCY space) to the
      reader, check it against the computed values, and derive the widgets'
      disabled colours from it as the preset states them (Button, View and
      Window sets, `ForegroundLink`); until then the live reader's
      `defaults.disabled_text_color` is not Breeze's.
      M8 T16c (2026-10-17): the switch's disabled outlines join them,
      `switch.disabled_unchecked_track_border_color` (`handleBorderColor` in
      the Disabled Button set, #e2e2e2 / #34373b) and
      `disabled_checked_track_border_color` (Disabled Selection, #e3e5e7 /
      #1f2124; `docs/platform-facts.md` §2.21), which kde-breeze states and the
      reader leaves to the preset.

### The tab gap, the check mark's line, the expander's structure, the text area (2026-09-28)

`tab.item_gap`, `checkbox.check_mark_stroke_width`, `expander.arrow_side` /
`arrow_gap` / `content_indent` / `frame_enabled` and `[text_area]` are in the
model, the platform presets and the three connectors
(`docs/platform-facts.md` §2.11, §2.5, §2.27, §2.29). Left open:

- [ ] [upstream] **gpui Textarea padding below upstream's editor padding.** gpui-component
      pads a multi-line field's editor by its `Size`'s `input_px` / `input_py`
      in render (`input/input.rs:531-541`), and a `Textarea` offers no size
      (Medium: 10 across, 8 down). `geometry::text_area` pads the root by the
      rest of a stated side; a smaller side — KDE's 5, GNOME's 0 — stays 10 / 8.
      Needs an upstream seam (a size or editor padding on `Textarea`).
- [ ] [1.0] **macOS text area padding.** NSTextView's `lineFragmentPadding` is 5, but
      `textContainerInset`'s default is not stated on its documentation page, so
      the total is not sourced and macos-monterey states none.
- [ ] [1.0] **KDE expander header.** `KCollapsibleGroupBox` puts its 10px arrow in the
      middle of a 20px indicator box at the widget's edge and sizes the header
      as `CT_CheckBox` (text + 2 × 2, at least 20); kde-breeze states
      `expander.header_height_px = 40` and no header padding, while
      platform-facts §2.27 says the KDE height is content-sized. Neither was
      changed in this round.
- [ ] [1.0] **The expander's arrow glyph.** KDE, GNOME and Windows draw a chevron,
      macOS a triangle (§2.27, `arrow_side` row); the model states no shape, so
      the iced showcase and egui draw a filled triangle and gpui a chevron.
- [ ] [1.0] **iced Basic tabs under adwaita.** The iced_aw tab bar clips "Three"
      ("Thr") at `tab.min_width` 64 (seen before this round in the r9 captures).
- [ ] [1.0] **egui Inputs page.** `pages/inputs.rs` computes `input_frame` /
      `text_area_frame` before entering the `Role::Input` scope, so the frame
      reads the outer `Ui`'s widget visuals.
- [ ] [1.0] **Card as a group box** (CORE-PLAN3b item 6): KDE's `QGroupBox` pads 8,
      macOS's `NSBox` 5 — awaits the maintainer's ruling on whether a group box
      is the model's card.

### Progress bar: Breeze's groove, the radii, the shadow

- [ ] [1.0] Found with the progress-bar border (2026-09-28), not changed: Breeze
      fills the groove with the window text at 0.2 × 0.7 over the window
      (`renderProgressBarGroove`, breezehelper.cpp:1204-1219; `#d2d3d5`,
      dark `#3e4144`, computed with Qt 6.11.2 and KF6 KColorUtils) where the
      preset's track inherits `defaults.muted_color`, and rounds it
      `0.5 × ProgressBar_Thickness` = 3 where it inherits
      `defaults.border.corner_radius` 5; libadwaita's trough and fill are
      pills (`border-radius: 99px`, `_scale.scss:1-10`,
      `_progress-bar.scss`); WinUI's track is `ControlStrongStrokeColorDefault`,
      the track's radius 0.5 and the indicator's 1.5
      (ProgressBar_themeresources.xaml:7, :31-32 at 8463f451).
      No preset states `progress_bar.border.shadow_enabled`, so it inherits
      `defaults.border.shadow_enabled` (true) where `docs/platform-facts.md`
      §2.10 gives no shadow on any platform. Correct §2.10 and the presets
      from these sources.
      Windows part done in M2b Step 7 (eff33489): §2.10's track colour and radius
      cite WinUI and windows-11 states them; the shadow is still open.
      macOS part done in M2a Step 7b (be69481a): macos-monterey states no shadow
      for the progress bar (and for the text field, check box, drop-down,
      segmented control and expander), as `docs/platform-facts.md`'s macOS cells give.

### macOS 27 Golden Gate: no preset of its own yet

- [ ] [1.0] When GitHub offers a non-preview macOS 27 runner image (today only the
      `xcode-27` preview, actions/runner-images #14404), run the OS probe on it and
      check, row by row as M2a did for 26, whether 27 changes a value the model
      carries. If it does, build `macos-golden-gate` (D8's key: Apple's codename for
      27) with a row in `pipeline.rs`'s `MACOS_GENERATIONS`; if not, raise
      `macos-tahoe`'s `last_verified` to 27. Until then a Mac on 27 gets
      `macos-tahoe`, and `SystemTheme::version_match` reports `PresetOlder` (plan
      D4). The preview probe already differs from 26.6.2 in the segmented
      control's width (98 against 91), the stepper's height (24 against 26),
      `underPageBackgroundColor` and the text field's render fill
      (`docs/evidence/os-probe/2026-10-11-run-38129819806/os-probe-xcode-27`).

### material-3: the screenshot names and the display name

- [ ] [v0.7.0] At the next `scripts/generate_assets_release.sh` run, the six README
      links to `linux-material-{light,dark}.png` (the gpui, iced and egui
      READMEs) become `linux-material-3-*`, since the screenshot scripts name
      their output after the preset key, and the old PNGs are removed with
      `git rm`. The display name "Material 3" is set in
      the preset's line of the `bundled_presets!` table
      (`native-theme/src/presets.rs`) when M2d rewrites the preset, with
      "Material 3 Expressive" for the second Material preset; the menus say
      "Material" until then. Nothing is renamed before
      (`docs/todo_code-review-plan.md`, M2d Step 15).

### macos-monterey and macos-tahoe: the screenshot names and the display name

- [ ] [v0.7.0] At the next `scripts/generate_assets_release.sh` run (M2a
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

### Hover fills: a layer or a replacement

- [ ] [v0.7.0] The connectors composite `hover_background` and
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
      the other pressed fills have no source yet. For M5–M7: decide whether
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

### windows-11: accent-derived values stated for the default accent

- [ ] [v0.7.0] M4 T35 (2026-10-13) gave windows-11 WinUI 3's brushes where a
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

### ios-26: 38 required fields have no iOS 26 source

- [ ] [v0.7.0] Step 17 of the code-review plan (`docs/todo_code-review-plan.md`,
      M2c) writes `ios-26.toml`, the Liquid Glass look of iOS 26; it is
      deferred (2026-10-11). The resolver requires 59 leaves per variant
      that no inheritance rule fills (`ResolutionIncomplete` otherwise,
      `native-theme/src/resolve/validate.rs`), Step 17 forbids copying
      `ios-13`'s values into them, and 38 of the 59 have no iOS 26 source:
      `defaults.link_color`, `defaults.border.corner_radius_lg_px`,
      `defaults.border.shadow_enabled`, `defaults.icon_sizes.toolbar_px`,
      `small_px`, `large_px`, `dialog_px`, `panel_px`,
      `button.icon_text_gap_px`, `checkbox.label_gap_px`,
      `menu.icon_text_gap_px`, `tooltip.max_width_px`,
      `scrollbar.groove_width_px`, `min_thumb_length_px`, `thumb_width_px`,
      `slider.thumb_diameter_px`, `progress_bar.min_width_px`,
      `tab.min_width_px`, `min_height_px`, `list.alternate_row_background`,
      `splitter.divider_width_px`, `switch.thumb_diameter_px`,
      `track_radius_px`, `dialog.min_width_px`, `max_width_px`,
      `min_height_px`, `max_height_px`, `button_gap_px`, `icon_size_px`,
      `spinner.diameter_px`, `min_diameter_px`, `stroke_width_px`,
      `combo_box.min_width_px`, `arrow_icon_size_px`,
      `segmented_control.segment_height_px`, `separator_width_px`,
      `expander.header_height_px` and `arrow_icon_size_px`. Three of them
      need only a mapping ruling, since WebKit draws them at de320ba3: the
      slider thumb is a 24 × 16 capsule, the switch thumb a 39.857 × 26.571
      capsule, the select's chevron 0.55 em wide. The other 21 leaves have
      a source (9 confirmed, 12 plausible); their values and citations are
      kept in `docs/todo_code-review-findings.md`, `## M2c — iOS`,
      *Steps 16–18*. Three of those carry an open choice:
      `defaults.disabled_opacity` 1.0 (the disabled colours are stated,
      `docs/inheritance-rules.toml:364-371`) or WebKit's 0.4 (html.css:945-947,
      RenderThemeCocoa.mm:1585); `defaults.selection_text_color` white on
      the inherited accent or label on UIKit's grey row selection;
      `button.min_width_px` 44 (the HIG's default control size) or 28 (its
      minimum). Also to re-rule before the file is written: WebKit's iOS 26
      controls draw a plain button's and a select's text in `label`, not
      systemBlue (RenderThemeCocoa.mm@de320ba3:3324-3329, :3229-3232,
      :3373-3376; a UA-sheet colour is not "explicitly set"), so the
      systemBlue `button.font.color`, `button.active_text_color` and
      `combo_box.font.color` M2c gave for iOS 26 are the pre-refresh look.
      The route to the missing values: an iOS Simulator probe on a GitHub
      macOS runner (the `macos-26` image), a dev-only app printing UIKit's
      intrinsic sizes and colours, as M2d Step 3's reference app does; the
      CI dispatch is the maintainer's.
      *M2 final review (2026-10-11), an open question kept from the ledger:*
      `ios-13` cites RenderThemeCocoa.mm@de320ba3:763 (`thumbColor =
      Color::white`) for `switch.thumb_background` and
      `switch.disabled_thumb_color`, "white, disabled too". Is that thumb the
      pre-refresh (iOS ≤ 18) path? The code says it is the classic painter:
      `renderThemePaintSwitchThumb` sends the refreshed look to
      `renderThemePaintLiquidGlassSwitchThumb` when
      `formControlRefreshEnabled()` (:737-742), and the classic painter
      changes nothing for a disabled thumb (:744-797). Not checked: that
      the classic path at de320ba3 is what iOS 18's WebKit drew (the
      pre-refresh tag, 31adf9d1, has `RenderThemeIOS.mm` instead), nor what
      UIKit's `UISwitch` draws. Step 17 cites the refreshed path's
      `liquidGlassSwitchThumbColor` (:475-493) for `ios-26`.

### Model: one `*_text_color` field, two meanings

- [ ] [1.0] For M3: `docs/platform-facts.md` §2.1.4's
      `danger_text_color`, `warning_text_color`, `success_text_color` and
      `info_text_color` are the body text beside a status colour on macOS
      and KDE (footnote ¹) and the text on the status fill on Windows and
      GNOME (footnote ²; libadwaita 1.10.0's `destructive_fg_color`,
      `success_fg_color`, `warning_fg_color` and `accent_fg_color`,
      `_colors.scss:13`, `:17`, `:20`, `adw-style-manager.c:161`, since M2e
      fix round 1). So kde-breeze states the window text (`#232629`) and
      adwaita white (`#ffffff`) for the same field, and gpui maps it to
      `danger_fg`, the text on the fill
      (`connectors/native-theme-gpui/src/colors.rs:182`). Give the model
      one meaning per field (or two fields), then restate the presets.
      M8 T21 (2026-10-17): no longer for M3, which closed on 2026-10-12
      (ledger "## M3") without naming this entry, and no milestone of the
      review is assigned.
      M8 T21 fix round 1 (2026-10-17): [1.0], not [v0.7.0]: a model change
      that no plan schedules fits neither of the header's `[v0.7.0]` clauses.

### Presets: kde-breeze's sidebar colour set

- [ ] [v0.7.0] `docs/platform-facts.md` §2.12 gave KDE's sidebar
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

### Presets: adwaita's list header and slider thumb

- [ ] [v0.7.0] Since M2e fix round 2 adwaita leaves `list.header_background`
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

### OS probe: macOS renders for the sidebar, the title bar and the list header

- [ ] [v0.7.0] `sidebar.background_color` has no source on macOS: the sidebar
      is `NSVisualEffectView.Material.sidebar`, a material with no colour
      (`docs/platform-facts.md:1370`), yet `macos-monterey` and `macos-tahoe`
      state `#e8e8ed` / `#262626`. The OS probe resolves its colours with
      Reduce Transparency on and renders no material. Have it render a
      sidebar-material `NSVisualEffectView` with Reduce Transparency on and
      off, in both appearances, on each macOS image; then state the cell
      from the renders and the presets follow (found in M2a Step 8's
      re-audit, 2026-10-11).
- [ ] [v0.7.0] `window.title_bar_background` and `list.header_background`
      are ≈ `defaults.surface_color` on macOS, "measured", with no render
      cited (`docs/platform-facts.md:1160`, `:1429`). In `macos-monterey`
      dark both inherit `controlBackgroundColor` `#1e1e1e` under a window of
      `#323232`. Render a titled window and an `NSTableView` with a header
      on the probe images, in both appearances, and check both cells against
      the renders (found in M2a Step 8's re-audit, 2026-10-11).

### OS probe: the selected segment in a key window

- [ ] [v0.7.0] `segmented_control.active_background` on macOS is
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

### platform-facts: the Plasma 6.8 rows cite the beta

- [ ] [v0.7.0] `docs/platform-facts.md:717`, `:718` and `:1127` state Plasma
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

### `ResolvedLayoutTheme`: a public type nothing produces

- [ ] [1.0] For the API review before 1.0 (M3 T9 ruling, 2026-10-12):
      `theme::ResolvedLayoutTheme` is generated by the `ThemeWidget` derive for
      `LayoutTheme` and is public, but no function returns it: `Resolved` has no
      layout, `SystemTheme.layout` is the unresolved `LayoutTheme`, and since
      M3-03 (`828d2d2a`) the layout's range check is the hand-written
      `check_layout` (`native-theme/src/resolve/validate_helpers.rs`), so the
      `check = "non_negative"` attributes on `LayoutTheme` feed generated code
      nothing calls. Decide one of: stop generating the type (hide it: an API
      break), or carry a resolved layout in `Resolved`/`SystemTheme` and let the
      generated check replace `check_layout`. Kept unchanged in M3 because no
      break was approved for it.

### macOS: the reader and the SF Symbols loader on macOS 10.15 and earlier

- [ ] [1.0] For the platform readers (M4-44, deferred by M4's triage,
      2026-10-13): the macOS reader calls
      `NSAppearance.performAsCurrentDrawingAppearance(_:)` and the SF Symbols
      loader `NSImage(systemSymbolName:accessibilityDescription:)`, both
      introduced in macOS 11.0 (Apple's documentation JSON, `introducedAt`,
      read 2026-10-13; objc2-app-kit 0.3.2 does not encode availability).
      On macOS 10.15 and earlier the unrecognised selector raises an
      Objective-C exception, which objc2's `msg_send` unwinds (objc2 0.6.4
      `src/macros/mod.rs:983-985`), instead of an `Err`; no GitHub runner is
      older than macOS 14, so this is unreproduced. Since M4 T24 the crate
      README states macOS 11 as the minimum. Gate it at runtime: in
      `read_macos`, `os_version().major >= 11` (or objc2's `available!`)
      else `Error::ReaderFailed`; in `load_sf_icon_by_name`, `None`.

### System icons: `.scale()` after a measurement on macOS and Windows

- [ ] [v0.7.x] From M1-T6-07 (deferred by M4's plan ruling, 2026-10-12): the
      SF Symbols and Segoe Fluent loaders take a size since 34d6d1f7
      (`.size(u32)`, `SfSymbolsLoader::size` and `SegoeFluentLoader::size`,
      `native-theme/src/icons.rs:437`, `:522`), but no display
      scale. The SF Symbols raster comes from
      `CGImageForProposedRect_context_hints` with a null rect
      (`native-theme/src/sficons.rs:52`), which the code's own comment says
      AppKit answers with the full Retina resolution; unmeasured. The Segoe
      glyph is drawn at its em height in pixels
      (`load_windows_icon_by_name` and its doc,
      `native-theme/src/winicons.rs:641-663`), whatever the display's scale.
      M1's decision makes `.scale()` conditional on a measurement: on the
      macOS leg, the raster's pixel size at 2×; on Windows, what a 150 % and a
      200 % display give. Measure on T1's CI legs or a real machine, then
      adopt `.scale(f32)` on both loaders (and the connectors' window scale)
      or record that it is not needed.

### System icons: `SystemTheme::icon` at a size

- [ ] [1.0] From M6-13 (M6 T4): `icons::load_custom_icon(provider, set, color, size)`
      hands `size` to each system set's loader (`FreedesktopLoader::size`,
      `SfSymbolsLoader::size`, `SegoeFluentLoader::size`), but
      `SystemTheme::icon(id, mode, color)` has no size and passes `None`
      (`native-theme/src/lib.rs`, `icons::load_icon_in`), so a system icon
      loaded from the theme comes at the loaders' defaults: a freedesktop
      icon looked up at 24 pixels (a PNG-only one from the theme's directory
      nearest 24), an SF Symbol at 24 points, a Segoe glyph at 32 pixels. M6's
      ruling kept its shape for 0.7.0; a size parameter (or a sized variant)
      is a signature change the maintainer decides with the icon API's
      other 1.0 questions.

### Segoe Fluent icons as SVG outlines

- [ ] [1.0] For M6 (M1-T6-07's Segoe item, deferred by M4's plan ruling,
      2026-10-12): Segoe Fluent glyphs reach the connectors as an RGBA raster
      (`GetGlyphOutlineW` with `GGO_GRAY8_BITMAP`,
      `native-theme/src/winicons.rs:425-433`), so they cannot be recoloured or
      scaled without a new rasterisation. Their outlines (`GGO_NATIVE`, or
      DirectWrite's `GetGlyphRunOutline`) as SVG paths would make Segoe an SVG
      set like the bundled ones, coloured by `colorize_monochrome_svg`. M4's em-box
      raster (d1469e46, aed8b5f1) and `.size(u32)` (34d6d1f7) fixed the visible
      defects; the outline path is a new pipeline, to weigh in M6 with gpui's
      icon cache. SF Symbols have no public vector export, so they stay rasters.
      M8 T21 (2026-10-17): [1.0], no longer for M6, by M6's Step 6 ruling
      (`docs/todo_code-review-findings.md`, "## M6", *Step 6 — fix plan
      (2026-10-15)*: "deferred, tag moved to [1.0]"): M4's em-box raster and
      `.size(u32)` removed the visible defects, M6-13 routes the requested
      size to the Segoe loader, and a `GGO_NATIVE` outline pipeline is
      Windows-only code this host cannot run.

### Validation: a bad `defaults.border` value is reported once per widget

- [ ] [v0.7.x] From M3 T9's review (M3 Step 8 minors, deferred by M4's plan
      ruling): a non-finite or negative `defaults.border` value is reported
      under `defaults.border` (`native-theme/src/resolve/validate_helpers.rs:787-800`)
      and again for every Full-border widget that inherits it
      (`check_widget_border`, `:517`), each under the widget's own key. Error
      noise, not a wrong result. A fix needs stated-versus-inherited knowledge
      the validator does not have (which widget values came from
      `defaults.border`): report an inherited value once, at its source.

### KDE fonts: Qt 6's `styleName`, field 16

- [ ] [1.0] Research (M4 T11 ruling, 2026-10-12): a Qt 6 font string can carry a
      `styleName` in field 16, which the KDE reader's parser ignores
      (`parse_qt_font_with_weight`, `native-theme/src/kde/fonts.rs:65`).
      Unverified: whether a set `styleName` overrides fields 4 and 5 (weight
      and style) in Qt's font matching; read `qfont.cpp` at a pinned Qt tag. If
      it does, a KDE font with a `styleName` renders another weight than the
      reader reports. No user report.

### GNOME: the reader's dark mode does not read `GTK_THEME`

- [ ] [v0.7.x] From M4 T15 (ruled 2026-10-12): `detect::system_is_dark` checks
      `GTK_THEME` first (`:dark` or `-dark`, `native-theme/src/detect.rs:766-771`),
      while the GNOME reader takes its mode from the portal's colour scheme,
      then gsettings' `color-scheme` (`color_scheme_is_dark`,
      `native-theme/src/gnome/mod.rs:497`). A `GTK_THEME=Adwaita:dark` session
      reads dark in `detect` and light in the reader. `GTK_THEME` is a
      developer override, not a GNOME setting. Decide whether the reader follows
      it, or `detect` stops reading it, so the two agree.

### windows-11: the title bar with "Show accent colour on title bars"

- [ ] [v0.7.x] From M4 T22 (ruled 2026-10-13): the Windows reader wrote
      `DwmGetColorizationColor` as `window.title_bar_background` whether or not
      the user turned on *Show accent colour on title bars*; since 8c133614 it
      does not, and outside high contrast the title bar is the preset's Mica
      fallback (bcb1d2c2; `docs/platform-facts.md:1160`, `:374`). With the
      setting on, Windows draws the title bar in an accent colour the theme then
      misses. The probe reads the setting as `ColorPrevalence` under
      `HKCU\Software\Microsoft\Windows\DWM` (0 on the Windows 11 25H2 runner,
      `docs/evidence/os-probe/2026-10-11-run-38129819806/os-probe-windows-11-arm/probe.toml:773`).
      Needed: a source for reading the setting and for the colour Windows draws
      with it on (and its text colour); then the reader writes them only when
      the setting is on, and the watcher reports its change.

### Theme watcher: the trailing debounce has no maximum wait

- [ ] [v0.7.x] From M4 T20's review (2026-10-13): the watchers fire once,
      `DEBOUNCE` (300 ms) after the last relevant event of a burst
      (`native-theme/src/watch/mod.rs:253-264`), with no maximum wait: relevant
      writes that keep arriving less than 300 ms apart never fire. Unrealistic
      for settings writes, so harmless today. Decide whether a burst fires at
      the latest after some maximum wait (a value to rule, there is no
      platform source) or record that none is needed.

### Readerless Linux desktops: text scaling and high contrast are not read

- [ ] [v0.7.x] From M4 T19 (2026-10-13): on a Linux desktop with no reader the
      fallback's accessibility reads reduced motion alone
      (`readerless_accessibility`, `native-theme/src/pipeline.rs:1250-1255`),
      while the GNOME reader also reads the text-scaling factor and high
      contrast. Coherent today: the fallback states the preset's text sizes.
      Decide, per desktop and with a source for each, whether a readerless
      desktop has a text-scaling or high-contrast setting the fallback should
      read.

### Linux: button order and `detect_platform` decide by the raw desktop

- [ ] [v0.7.x] From M4 T16 (2026-10-12): since 62c13acf the reader, the watcher,
      the icon theme and the font DPI decide by the refined desktop
      (`DetectionContext::refined_linux_desktop_sync`,
      `native-theme/src/detect.rs:1355`), but `platform_button_order`
      (`native-theme/src/resolve/inheritance.rs:62-71`) and
      `presets::detect_platform` (`native-theme/src/presets.rs:129-142`) still
      read `XDG_CURRENT_DESKTOP` alone. On an unrecognised desktop refined to
      KDE the button order still agrees, through kde-breeze's stated
      `primary_left`, but `list_presets_for_platform` (`:164`) leaves kde-breeze
      out (pre-existing). Make both read the refined desktop.

### Linux: an application that only resolves presets never gets the portal's answer

- [ ] [v0.7.x] From M4 T16 (2026-10-12; the M3-25 split, documented):
      `refined_linux_desktop_sync` asks no portal and caches nothing until a
      system read (`from_system`, `from_system_async`,
      `AccessibilityPreferences::from_system`) has refined the desktop
      (`native-theme/src/detect.rs:1344-1352`), so an application that resolves
      presets without one gets only the `kdeglobals` check: on an unrecognised
      desktop a KDE portal backend without `kdeglobals` stays `Unknown`. Also,
      `invalidate_linux_desktop()` (`:1406`) clears the desktop but no slot
      derived from it. Decide whether the sync path may ask the portal (through
      `pollster`, bounded as the readers are) or whether the docs stay as they
      are; give `invalidate_linux_desktop` the line its doc needs.

### KDE reader: `switch.thumb_background` and `menu.icon_size` are not read

- [ ] [v0.7.x] From M4 T36 (2026-10-13): kde-breeze states the switch thumb as
      `[Colors:Button] BackgroundNormal` (a7da50c7; `docs/platform-facts.md:1523`)
      and the menu icon size as KIconLoader's Small, 16 (`:1268`), but the KDE
      reader writes neither: a custom colour scheme keeps Breeze's thumb, and
      an icon theme whose `SmallDefault` differs keeps 16 (the reader reads
      `SmallDefault` into `defaults.icon_sizes.small` only,
      `native-theme/src/kde/mod.rs:188`). Write both, citing the facts rows,
      and run the `preset-validator` over kde-breeze.
      M8 T16c (2026-10-17): the reader writes `switch.thumb_background`, the
      Button set's background (`SwitchIndicator.qml:29` at qqc2-desktop-style
      v6.31.0; §2.21). `menu.icon_size` is still not read.

### kde-breeze: the Qt Widgets menu fill

- [ ] [v0.7.x] From M4 T36 and the kde-breeze validator after it (2026-10-13):
      Breeze's Qt Widgets menu is filled with `frameBackgroundColor()`,
      mix(Window, Base, 0.3) (breezestyle.cpp `drawPanelMenuPrimitive`;
      breezehelper.cpp:200-203, as the T36 implementer read them), while
      `docs/platform-facts.md:1253` gives KDE's `menu.background_color` as
      `[Colors:Window] BackgroundNormal`, which qqc2-desktop-style's `Menu.qml`
      uses (`Kirigami.Theme.backgroundColor`). The row depends on which toolkit
      it describes. Cite both at v6.7.5 / v6.31.0 in the facts cell, decide
      which one kde-breeze states, and correct the preset or the cell.

### kde-breeze: the header's "Verified on" date

- [ ] [v0.7.x] From the kde-breeze validator after M4 T36 (2026-10-13):
      `native-theme/src/presets/kde-breeze.toml:8` at 2ca74471 says the cited values were
      verified on 2026-10-11 (M2e), but T34 and T36 changed and cited values on
      2026-10-13 (fd4abc64, a7da50c7, 74fc414d). Note the 2026-10-13 values in
      the header.

### platform-facts: no shadow rows for the toolbar and the list

- [ ] [v0.7.x] From the kde-breeze validator after M4 T36 (2026-10-13):
      `docs/platform-facts.md` has no `shadow_enabled` row for the toolbar and
      the list, so a preset's value for them cites nothing. Add the rows,
      sourced per platform, then state the presets' values with citations.
      M8 T15 (2026-10-17): the rows are in (§2.13 and §2.15
      `border.shadow_enabled`, with `border.color`, `border.corner_radius` and
      `border.line_width`); the presets' half is in *Presets: values the M8
      T15 platform-facts rows disagree with*.

### Model: the menu bar has no fill of its own

- [ ] [1.0] From the M8 T16 review (2026-10-17, finding 10): Breeze fills a
      `QMainWindow`'s tools area, its menu bar and its top toolbars, as one
      area in the Header set's Window colour (`breezestyle.cpp:1341-1347`;
      `breezetoolsareamanager.cpp:118-136`, Breeze f0b1d753), which
      kde-breeze states as `toolbar.background_color` (#dee0e2 / #292c30,
      `docs/platform-facts.md` §2.13). The model has no menu-bar fill, so the
      three showcases draw the menu bar in the window's #eff0f1 / #202326
      above the Header-filled toolbar: a two-tone tools area Breeze does not
      draw. A `menu_bar` fill leaf (registry, inheritance, the three
      connectors' menu bars), or the showcases' chrome taking the toolbar's
      fill for the menu-bar row, would close it.

### Presets: expander hover fills with no source

- [ ] [1.0] From M6 T16b (2026-10-16): `docs/platform-facts.md` §2.27 has no
      `hover_background` row, and these presets state `expander.hover_background`
      in both modes with no citation: adwaita, macos-monterey ("disclosure
      hover -- subtle highlight"), ios-13, material-3 (M3 has no expander,
      §2.27), and the community presets catppuccin-latte, -frappe, -macchiato,
      -mocha, dracula, gruvbox, nord, one-dark, solarized and tokyo-night
      (`adwaita.toml`, `macos-monterey.toml`, `ios-13.toml`, `material-3.toml`,
      `catppuccin-latte.toml`, `catppuccin-frappe.toml`, `catppuccin-macchiato.toml`,
      `catppuccin-mocha.toml`, `dracula.toml`, `gruvbox.toml`, `nord.toml`,
      `one-dark.toml`, `solarized.toml` and `tokyo-night.toml`, each
      `[light.expander] hover_background` and `[dark.expander] hover_background`).
      kde-breeze's was unset in M6 T16b: the
      §2.27 `frame_enabled` row (`docs/platform-facts.md:1686`) says
      `KCollapsibleGroupBox::paintEvent` paints the arrow and the title and
      nothing else. Find each platform's source for a hovered expander header
      (a facts row), or unset the value; the gpui connector's
      `geometry::accordion_hover` draws whatever the preset states.
      M8 T15 (2026-10-17): §2.27 now has a `hover_background` row:
      libadwaita's expander header in a boxed list is a `%boxed_list_row`,
      whose hover is `color-mix(in srgb, currentColor 3%, transparent)` over
      the card (`_lists.scss:352-355`, `:524-526`); WinUI 3 changes the
      chevron button's fill, not the header's; KDE draws none; macOS ❓; M3
      has no expander. The preset values stay for T16 and the entry
      *Presets: values the M8 T15 platform-facts rows disagree with*.

### Presets: values the M8 T15 platform-facts rows disagree with

- [ ] [1.0] From M8 T15 (2026-10-17): the 49 rows T15 added to
      `docs/platform-facts.md` (the state colours of §2.3–§2.28, the toolbar's
      and the list's border, the progress bar's and the sidebar's padding)
      and its re-sourced §2.16 edge cells disagree with these preset values.
      Each item names the preset value, then the row (section, field) that
      gives another; no value was changed (M8-76 ruling: D3, M2's domain).
      - adwaita: `button.active_background` `#c8c8c8` / `#2e2e32`
        (`adwaita.toml` `[light.button] active_background`,
        `[dark.button] active_background`; §2.3: `currentColor` at 30 %, a
        translucent replacement); `input.hover_border_color` and
        `input.focus_border_color` `#3584e4`
        (`[light.input] focus_border_color`, `[dark.input] focus_border_color`;
        `hover_border_color` is stated in neither since cff0bc68, M8 T16 fix
        round 1; §2.4: no border line, and focus is the ring, the accent at
        50 %); `checkbox.hover_background` `#d5d5d5` / `#464649`
        (`[light.checkbox] hover_background`, `[dark.checkbox] hover_background`;
        §2.5: no unchecked fill, a 10 % `currentColor` layer when checked),
        `checkbox.unchecked_background` `#ffffff` / `#303034`
        (`[light.checkbox] unchecked_background`,
        `[dark.checkbox] unchecked_background`; §2.5: transparent),
        `checkbox.unchecked_border_color`
        (`[light.checkbox] unchecked_border_color`,
        `[dark.checkbox] unchecked_border_color`; §2.5: `currentColor` at
        15 %); `scrollbar.thumb_active_color` `#4e4e4e` / `#c0c0c0`
        (`[light.scrollbar] thumb_active_color`,
        `[dark.scrollbar] thumb_active_color`; §2.8: `currentColor` at 60 %);
        `slider.thumb_hover_color` `#ebebeb` / `#3a3a3e`
        (`[light.slider] thumb_hover_color`, `[dark.slider] thumb_hover_color`;
        §2.9: white in both modes); `tab.hover_background`
        (`[light.tab] hover_background`, `[dark.tab] hover_background`; §2.11:
        `currentColor` at 7 %); `splitter.hover_color`
        (`[light.splitter] hover_color`, `[dark.splitter] hover_color`; §2.17:
        no hover state, ← `divider_color`); `switch.hover_checked_background`
        `#2e78cc` (`[light.switch] hover_checked_background`,
        `[dark.switch] hover_checked_background`; §2.21: a 10 %
        accent-foreground layer over the accent),
        `switch.hover_unchecked_background`
        (`[light.switch] hover_unchecked_background`,
        `[dark.switch] hover_unchecked_background`; §2.21: `currentColor` at
        20 %); `combo_box.hover_background`
        (`[light.combo_box] hover_background`,
        `[dark.combo_box] hover_background`; §2.24: `currentColor` at 15 %);
        `segmented_control.hover_background`
        (`[light.segmented_control] hover_background`,
        `[dark.segmented_control] hover_background`; §2.25: `currentColor` at
        7 % over the group's fill); `expander.hover_background`
        (`[light.expander] hover_background`,
        `[dark.expander] hover_background`; §2.27: `currentColor` at 3 %, a
        layer), `expander.arrow_color` `#2e3436` / `#ffffff`
        (`[light.expander] arrow_color`, `[dark.expander] arrow_color`; §2.27:
        `currentColor` at 55 %); `link.hover_text_color` and
        `link.disabled_text_color`, both the rest colour
        (`[light.link] hover_text_color, disabled_text_color`,
        `[dark.link] hover_text_color, disabled_text_color`; §2.28: the
        accent 10 % more saturated and lighter, and the link colour at 50 %). Unstated where the rows give a value: the
        toolbar's border (§2.13: none), the list's (§2.15: none for a plain
        list), the progress bar's padding (§2.10: 0) and the sidebar's
        (§2.12: 6, 6 top / 4 bottom).
      - kde-breeze: `button.active_background` `#3daee9`
        (`kde-breeze.toml` `[light.button] active_background`,
        `[dark.button] active_background`; §2.3: `mix(Button, Highlight,
        0.3)`, #c3e5f6 / #2f5368), `button.active_text_color` `#ffffff` light
        (`[light.button] active_text_color`; §2.3: `ButtonText`, #232629);
        `checkbox.hover_background` `#93cee9`
        (`[light.checkbox] hover_background`, `[dark.checkbox] hover_background`;
        §2.5: no hover fill, a `DecorationFocus` ring);
        `scrollbar.thumb_active_color` `#3daee9`
        (`[light.scrollbar] thumb_active_color`,
        `[dark.scrollbar] thumb_active_color`; §2.8: ← `thumb_hover_color`,
        which the preset states `#93cee9`); `slider.thumb_hover_color`
        `#93cee9` (`[light.slider] thumb_hover_color`,
        `[dark.slider] thumb_hover_color`; §2.9: the `Button` fill, #fcfcfc /
        #292c30, hover being an outline), `slider.disabled_fill_color`,
        `disabled_track_color` and `disabled_thumb_color`
        (`[light.slider] disabled_fill_color, disabled_track_color, disabled_thumb_color`,
        `[dark.slider] disabled_fill_color, disabled_track_color, disabled_thumb_color`;
        §2.9: no fill, the groove #d9dcdd / #2a2b2e, the handle #f0f0f0 /
        #272a2e); `splitter.hover_color`
        (`[light.splitter] hover_color`, `[dark.splitter] hover_color`; §2.17:
        ← `divider_color`); the switch's hover and disabled tracks
        (`[light.switch] hover_checked_background, hover_unchecked_background, disabled_checked_background, disabled_unchecked_background`
        and the same four in `[dark.switch]`; §2.21: hover changes borders
        only; disabled #e6e7e8 / #26292c checked, #e9e9e9 / #2e3135
        unchecked); `combo_box.hover_background` `#93cee9`
        (`[light.combo_box] hover_background`,
        `[dark.combo_box] hover_background`; §2.24: no hover fill);
        `segmented_control.hover_background`
        (`[light.segmented_control] hover_background`,
        `[dark.segmented_control] hover_background`; §2.25: the tab bar's
        proxy, #3daee933); `expander.arrow_color` `#232629` / `#fcfcfc`
        (`[light.expander] arrow_color`, `[dark.expander] arrow_color`; §2.27:
        #424447 / #dbdbdc); `link.disabled_text_color` `#a3cae2` / `#164160`
        (`[light.link] disabled_text_color`, `[dark.link] disabled_text_color`;
        §2.28: Kirigami's `LinkButton` draws the Disabled
        text colour, #a0a1a3 / #686a6c). Inherited where the rows give
        another: the list's border (`defaults.border`, #bcc0bf / #4d545b,
        radius 5, a shadow; §2.15: #c6c8c9 / #4c4e51, radius 4.5, no shadow)
        and the popover's (§2.16: #c2c5c9 / #474c51, 1); `link.active_text_color`
        (§2.28: `Qt.darker`, #14405c / #0e4d79); the progress bar's padding
        (§2.10: 0).
      - windows-11: `button.disabled_background`, `input.disabled_background`
        and `combo_box.disabled_background` `#f9f9f900` / `#33333300`
        (`windows-11.toml` `[light.button] disabled_background`,
        `[light.input] disabled_background`,
        `[light.combo_box] disabled_background`;
        `[dark.button] disabled_background`,
        `[dark.input] disabled_background`,
        `[dark.combo_box] disabled_background`; §2.3, §2.4, §2.24: `ControlFillColorDisabled`, #4DF9F9F9 /
        #0BFFFFFF; `docs/todo.md`, *native-theme-gpui connector*, *Research*,
        "windows-11 light gives `button.disabled_background`"); dark
        `input.hover_border_color` `#00000019`
        (`[dark.input] hover_border_color`; §2.4:
        the rest gradient, `ControlStrokeColorDefault` #12FFFFFF above the
        bottom edge); `checkbox.hover_background` `#0000000a` / `#ffffff0f`
        (`[light.checkbox] hover_background`, `[dark.checkbox] hover_background`;
        §2.5: `ControlAltFillColorTertiary`, #0F000000 /
        #0BFFFFFF, the entry in *M6 close* below); `list.disabled_text_color`
        (`[light.list] disabled_text_color`, `[dark.list] disabled_text_color`;
        §2.15: the rest colour, the row faded to 0.3);
        `segmented_control.hover_background`
        (`[light.segmented_control] hover_background`,
        `[dark.segmented_control] hover_background`; §2.25: no
        mapped control, `SelectorBar` stays transparent). Inherited where the
        rows give another: the toolbar's border (§2.13: none at rest), the
        list's (§2.15: none), the dark popover edge (`defaults.border.color`
        `#00000019`; §2.16: `SurfaceStrokeColorFlyout` #33000000), and the
        progress bar's and the sidebar's padding (§2.10: 0; §2.12: 3 across).
      - material-3: `scrollbar.thumb_active_color`
        (`material-3.toml` `[light.scrollbar] thumb_active_color`,
        `[dark.scrollbar] thumb_active_color`; §2.8: M3 has no scrollbar);
        `list.disabled_text_color` `#1d1b2061` / `#e6e0e961`
        (`[light.list] disabled_text_color`, `[dark.list] disabled_text_color`;
        §2.15: on-surface at 0.3, #1d1b204d / #e6e0e94d);
        `splitter.hover_color` `#6750a4` / `#d0bcff`
        (`[light.splitter] hover_color`, `[dark.splitter] hover_color`; §2.17:
        none in v0_192, the latest drag handle's state layer);
        `expander.hover_background` and `expander.arrow_color`
        (`[light.expander] hover_background, arrow_color`,
        `[dark.expander] hover_background, arrow_color`; §2.27: no expander);
        the link's hover, active and disabled text
        (`[light.link] hover_text_color, active_text_color, disabled_text_color`,
        `[dark.link] hover_text_color, active_text_color, disabled_text_color`;
        §2.28: no link).
        Inherited where the rows give another: the list's border
        (`defaults.border`, outline, radius 12; §2.15: none, 0) and the
        progress bar's and the sidebar's padding (§2.10: 0; §2.12: 12 across).
      - macos-monterey states these fields where the rows give ❓ (no AppKit
        source, and the OS probe renders no state but rest): the button's
        pressed and disabled fills, the input's hover and focus borders and
        disabled fill, the checkbox's hover, disabled and unchecked fills and
        unchecked border, the scrollbar's pressed thumb, the slider's hover
        and disabled colours, the tab's hover, the splitter's hover, the
        switch's hover and disabled tracks and thumb, the combo box's hover
        and disabled fills, the segmented control's hover, and the expander's
        hover and arrow. macos-tahoe states none of them.
      `native-theme/src/presets/documented_sizes.rs` cites §2.10 and §2.12 at
      their headings for the presets whose padding rows give a value they do
      not state yet; T16 states them and cites the rows.
      M8 T15 fix round 1 (2026-10-17, after the independent verification):
      further rows now disagree with presets, for T16. kde-breeze: the
      toolbar's bottom edge, 1 in the Header set's separator colour (#b9bbbd /
      #535659, §2.13, where the preset states `line_width_px = 0`); the
      popover's padding, 12 (§2.16); the slider thumb, `[Colors:Button]`
      #fcfcfc / #292c30 (§2.9); the switch tracks, checked #92cae8 /
      #3e7797 and unchecked #e6e7e7 / #3e4144 (§2.21); the card, Kirigami's
      `Card` (§2.26). windows-11: the tab label, `TextFillColorSecondary`
      (§2.11 `font.color`). adwaita: no border at rest on the button, the
      entry and the dropdown (§2.3, §2.4, §2.24), `$slider_color` for the
      slider thumb (§2.9), `$button_color` behind the toggle group (§2.25).
      The macOS presets: §2.26's cells are ❓ (the box's look is not
      published) with a 5 padding from `NSBox.contentViewMargins`, so the
      inherited card fill is not a wrong safety net; the progress bar's
      padding is 0, and on macOS 26 the unchecked box is `#00000019` /
      `#ffffff19` with no outline (§2.5, §2.10).
      M8 T15 fix round 2 (2026-10-17): two more for T16. kde-breeze
      `button.hover_background` `#93cee9` (`kde-breeze.toml`
      `[light.button] hover_background`, `[dark.button] hover_background`;
      §2.3: no hover fill, ← `background_color`, #fcfcfc / #292c30, hover
      being the `Highlight` outline). adwaita `expander.border`
      (`adwaita.toml` `[light.expander.border] corner_radius_px`,
      `[dark.expander.border] corner_radius_px`, `corner_radius_px = 6.0`, GtkExpander's
      title; §2.27, AdwExpanderRow in a boxed list: 12 at the list's outer
      corners and 0 elsewhere, a 1px `RGB(0 0 6 / 3%)` card outline the
      preset inherits instead, and the card's shadow).
      M8 T15 fix round 3 (2026-10-17): kde-breeze `toolbar.background_color`
      is unstated (`kde-breeze.toml` `[light.toolbar]`, `[dark.toolbar]`),
      so it inherits the window's #eff0f1 / #202326; §2.13 now
      gives the Header set's #dee0e2 / #292c30, the tools area's fill where
      the scheme has Header colours and the decoration's border size is
      automatic, None or NoSides (breezestyle.cpp:1322, :1341-1347).
      M8 T16 (2026-10-17, ac70ed0d..b1423ef6): stated, each checked against its row
      (preset-validator method): kde-breeze's toolbar edge (1, #b9bbbd /
      #535659, no shadow) and fill (#dee0e2 / #292c30: both Breeze schemes
      have Header colours, and `BorderSizeAuto` reads true where kwinrc sets
      nothing, breezehelper.cpp:1913, so the default configuration draws the
      tools area), popover (edge #c2c5c9 / #474c51, padding 12), slider
      thumb, switch rest tracks, button hover and card (Kirigami's `Card`:
      edge #ced1d4 / #3c4146, 1, radius 5, shadow, padding 8; the fill
      inherited); windows-11's tab label and popover edge; adwaita's rest
      borders (0 on the button, the entry and the dropdown), slider thumb,
      toggle-group fill (#e6e6e7 / #38383c, `$button_color` over the window)
      and expander (radius 12, the card's 1px #00000608 outline, the shadow
      kept, as the corrected §2.27 gives it, so M8-77's fifth widget keeps
      `true`); the macOS card padding 5 and macos-tahoe's unchecked box; and
      the toolbar edges of adwaita and windows-11. The progress bar's
      padding, 0, is stated in kde-breeze, adwaita, windows-11 and the macOS
      presets. Still open here: the sidebar padding (adwaita 6, 6 top / 4
      bottom; windows-11 3 across), which moves the gpui showcase's side
      panel and fails its `the_side_panel_holds_the_theme_settings_and_the_inspector`
      and `the_tab_bars_are_inset_by_the_container_margin`, a showcase
      layout question; material-3's progress and sidebar padding (M2d's
      preset); and the state colours above, among them kde-breeze's switch
      hover tracks, which now differ from its rest tracks where §2.21 gives
      them equal.
      M8 T16 fix round 1 (2026-10-17): the KDE reader
      (`native-theme/src/kde/colors.rs`) derives none of kde-breeze's new
      colour values, so under another colour scheme or accent they keep
      Breeze's. Three of them followed the reader before T16 and are frozen
      now: `slider.thumb_color` (it inherited the surface colour),
      `toolbar.background_color` (the window colour) and
      `switch.checked_background` (the accent): M8 T16b's (the T16 review's
      finding 6). Also in fix round 1: the card's padding is 7, not 8, inside
      its border line (Kirigami's 8 is from the outer edge; §2.26 corrected,
      b7f16c0d, 60841b4f); adwaita's text input and drop-down fill with
      `$button_color`, #e6e6e7 / #38383c (§2.4, §2.24 corrected), its input
      states no hover edge and focuses in `$focus_border_color`, #0461be80 /
      #81d0ff80. Still open: the switch track's and handle's outline, which
      qqc2's `SwitchIndicator.qml` strokes (`handleBorderColor`, the
      checked track's `highlightColor`), has no model field (`[switch]`,
      `docs/property-registry.toml`), so #e6e7e7 shows on the window with no
      edge; and adwaita `button.background_color` #e8e8e8 / #3a3a3e is
      uncited, where the same `$button_color` composites to #e6e6e7 /
      #38383c.
      M8 T16b (2026-10-17): the KDE reader derives the eight colour leaves
      from the colour scheme as Breeze, Kirigami and qqc2-desktop-style
      compute them (`button.hover_background`, `slider.thumb_color`, both
      switch tracks, the toolbar's fill and edge, the popover's and the
      card's edges), in Qt's arithmetic, checked against Qt 6.11.2 test
      vectors (`scripts/generate_qt_color_vectors.py`); the toolbar fill's
      kwinrc condition is the next entry.
      M8 T16c (2026-10-17): the switch's outlines are in the model
      (`switch.*_track_border_color`, `track_border_line_width`,
      `thumb_border_color`, `thumb_border_line_width`, soft options), stated
      in kde-breeze, windows-11 and material-3 from §2.21 and drawn by the
      three connectors; the KDE reader derives the rest outlines, the thumb's
      fill and `button.hover_text_color`. kde-breeze's switch hover tracks are
      its rest tracks now, and its track 12 tall with radius 6 (§2.21). Still
      open: its disabled tracks, `#3daee980` / `#bcc0bf80` / `#4d545b80`
      (`kde-breeze.toml`, `[light.switch]`, `[dark.switch]`), where §2.21
      gives #e6e7e8 / #26292c checked and #e9e9e9 / #2e3135 unchecked.
      M8 T16c fix round 1 (2026-10-17): kde-breeze states those four now,
      `disabled_checked_background` #e6e7e8 / #26292c and
      `disabled_unchecked_background` #e9e9e9 / #2e3135
      (`docs/platform-facts.md:1529-1530`); the switch's disabled tracks close.
      M8 final fix (2026-10-17): adwaita `button.hover_background` `#d5d5d5` /
      `#464649` (`adwaita.toml` `[light.button] hover_background`,
      `[dark.button] hover_background`; §2.3 `hover_background`, whose GNOME
      cell this fix gives its value: `$button_hover_color`, `currentColor` at
      15 %, a translucent replacement of the rest fill, #dcdcde / #434347 over
      the window).
      M9a T17 (2026-10-18): the sidebar padding is stated from §2.12's rows
      (`docs/platform-facts.md:1380-1381`): adwaita 6 across, 6 top and 4
      bottom (`adwaita.toml`
      `[light.sidebar.border] padding_horizontal_px, padding_top_px, padding_bottom_px`,
      the same in `[dark.sidebar.border]`); windows-11 3 across
      (`windows-11.toml` `[light.sidebar.border] padding_horizontal_px`,
      `[dark.sidebar.border] padding_horizontal_px`), its vertical ❓ and
      unstated; material-3 12 across, Compose's
      `NavigationDrawerItemDefaults.ItemPadding` (`material-3.toml`
      `[light.sidebar.border] padding_horizontal_px`,
      `[dark.sidebar.border] padding_horizontal_px`), its vertical
      app-defined and unstated. The three sidebar-padding clauses above are
      fixed, and the gpui showcase's two side-panel tests take the panel's
      padding from the theme. The rest of this item stays open.

### KDE reader: the toolbar's Header fill and the window border size

- [ ] [1.0] From M8 T16b (2026-10-17): Breeze fills a main window's tools
      area with the Header set only where the window decoration's border
      size is automatic, None or NoSides (kwinrc `[org.kde.kdecoration2]
      BorderSizeAuto` / `BorderSize`; breezehelper.cpp:1904-1941 at breeze
      f0b1d75; `docs/platform-facts.md` §2.13 `background_color`); otherwise
      the toolbar shows the window colour. The KDE reader
      (`native-theme/src/kde/colors.rs`, `populate_colors`) assumes Plasma's
      default, automatic, and gives `toolbar.background_color` the Header
      colour whatever the user set. Read kwinrc's border size, as Breeze
      does, and give the toolbar the window colour where it is another
      size.

### windows-11: the validator's findings after T35 left open

- [ ] [v0.7.x] From the windows-11 `preset-validator` after M4 T35 (2026-10-13;
      the stopping rule sent everything T35's fix round did not take here):
      - `defaults.shadow_color`: `docs/platform-facts.md:1057` gives Windows
        two layers (the ambient one drawn, 14 % low, 20 % high), and the
        model's one shadow colour serves both the popup and the window
        shadows; unchanged. Decide what the one field states, or split it.
      - `list.alternate_row_background` `#f9f9f9` / `#262626`
        (`native-theme/src/presets/windows-11.toml`
        `[light.list] alternate_row_background`,
        `[dark.list] alternate_row_background`): no WinUI 3
        source (❓, facts :1411); the field stays, as its fallback is the
        wrong kind.
      - Under high contrast the reader writes `input.background_color` as
        `COLOR_BTNFACE` (`native-theme/src/windows.rs:363`), where the facts'
        Win32 cell names `COLOR_WINDOW` (`docs/platform-facts.md:1203`); the
        Win32 cell is not the HC mapping, and the HC writes have no facts rows.
        Give the HC mappings their rows.
      - The link's hover and pressed text have no high-contrast mapping
        (`windows.rs:520-521` writes them from the accent shades outside high
        contrast only); under HC the preset's default-accent values stay.
      - `splitter.hover_color` now equals its rest line (inherits
        `divider_color`): WinUI 3 has no draggable divider (facts :1455).
      Tracked in other entries: the dark scroll thumb (*Presets: the scrollbar
      thumb colour*), the dark selected text and the menu backdrop
      (*windows-11: accent-derived values*), `tab.active_background`
      (*windows-11: what WinUI 3 draws*), `primary_background` (the M5 entry
      above).
      G3 T15 (2026-10-22): `tab.active_background` is WinUI's
      `SolidBackgroundFillColorTertiary`, `#f9f9f9` / `#282828`, and
      *windows-11: what WinUI 3 draws* is closed, in
      `docs/archive/todo_closed.md`.

### Linux font DPI: the `xrandr` fallback has no facts row

- [ ] [v0.7.x] From the kde-breeze validator after M4 T34 (D3, deferred to Step
      8): the font-DPI chain ends with the physical DPI `xrandr` reports
      (`detect_physical_dpi`, `native-theme/src/detect.rs:473-485`, the last
      step of `detect_system_font_dpi`, `:636`), and no
      `docs/platform-facts.md` row says that any desktop uses it for fonts.
      Find a source (Plasma, gnome-settings-daemon) and add the row, or drop
      the step so the chain ends at 96.

### CI: the macOS and Windows legs and the strict-panic steps have not run

- [ ] [v0.7.x] M4 added CI test legs for `macos,watch,system-icons` and
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

### Dependencies: zbus 6 and the three unmaintained-crate advisories

- [ ] [v0.7.x] From M8 Step 8 (2026-10-17, f0f17b95): zbus 6.0.0 merged
      zvariant, zvariant_derive and zbus_names into itself, so the zbus 5
      line's copies, zvariant 5.15.0, zvariant_derive 5.15.0 and zbus_names
      4.3.4, are unmaintained (RUSTSEC-2026-0370, -0371, -0372), and
      `.cargo/audit.toml` passes the three with that reason. native-theme's
      `portal` and `watch` features need zbus 5 for ashpd 0.13.13, the latest
      ashpd, which requires zbus ^5.13 (`native-theme/Cargo.toml`, `ashpd`,
      `zbus`); accesskit_unix, atspi and oo7, in the connectors' winit and
      GPUI stacks, also require zbus ^5 as of 2026-10-17. Move native-theme's
      `zbus` to 6 once ashpd releases on zbus 6, and remove the three
      `.cargo/audit.toml` entries once no dependency in `Cargo.lock` pulls
      zvariant 5, zvariant_derive 5 or zbus_names 4.
      2026-10-17 (M8 Step 8, the full check_release on 3148f231): the
      "Outdated dependencies" check runs no more. `cargo outdated --workspace
      --depth 1` resolves every dependency at its latest release and fails on
      zbus 6.0.0, which has no `blocking-api` feature (native-theme's `watch`
      enables `zbus?/blocking-api`), and with `--exclude zbus` fails next on
      gpui-kit 0.7.3's own graph: gpui-fast-web 0.1.8 pins
      `unicode-properties = "=0.1.3"` while gpui-pre-web 0.3.9 selects 0.1.4.
      check_release.sh reports the failure as a warning, "dependencies not
      checked", and lists nothing. Settle it with the zbus move, or replace the
      check with one that compares each direct dependency's requirement with
      crates.io's newest stable release without resolving the graph.

### detect: `prefers_reduced_motion` and the reader's block are two caches of one read

- [ ] [v0.7.x] From M4's final review (P2-2, 2026-10-13):
      `detect::prefers_reduced_motion()` (`native-theme/src/detect.rs:905`) and
      `accessibility_preferences().reduce_motion` (`:942`) cache one read
      apart; `from_system()` refreshes the second, not the first, so after a
      watched change without `invalidate_caches()` they disagree. T19's
      equality test runs both cold and cannot see it. Fill the reduced-motion
      slot from the reader's block too, or say so in
      `prefers_reduced_motion`'s docs.

### KDE: the icon sizes and the icon lookup search the base dirs in different orders

- [ ] [v0.7.x] From M4's final review (P3-2, 2026-10-13): the KDE reader looks
      for the icon theme's `index.theme` in KIconTheme's order, the data home,
      the data dirs, then `~/.icons` (`kde_icon_base_dirs`,
      `native-theme/src/kde/mod.rs:220`), and the icon lookup in the Icon
      Theme Specification's, `~/.icons` first (`icon_base_dirs_in`,
      `native-theme/src/icons.rs:899`). Both orders are cited; with a copy of
      the theme in both `~/.icons` and `~/.local/share/icons`, the sizes and
      the icons come from different copies. Rare. Decide whether one order
      serves both, or document it.

### detect: `refined_linux_desktop_sync` stats `kdeglobals` on every call before a system read

- [ ] [v0.7.x] From M4's final review (P7-1, 2026-10-13): on an unrecognised
      desktop with nothing cached yet, `refined_linux_desktop_sync`
      (`native-theme/src/detect.rs:1355-1360`) checks that `kdeglobals` exists
      (a `stat`) on every call and caches nothing, and the `icon_name` that
      native-theme-build generates calls it per lookup (64a58f9f), an icon per
      widget per frame in egui. Low impact: a themed application has usually
      made a system read. Cache the portal-less answer in its own slot
      (cleared with the desktop), or document it.

### Links: Kirigami darkens a pressed link

- [ ] [1.0] From M4 T34 (2026-10-13): kde-breeze's link hover and pressed text
      inherit `link.font.color` (fd4abc64), as a Qt rich-text link keeps
      `palette(link)` in every state, but Kirigami's `LinkButton` darkens a
      pressed link, `Qt.darker(linkColor)` (kirigami v6.31.0
      `templates/LinkButton.qml:66`, `:87-93`). The model has no darkening rule,
      so a Kirigami application's pressed link is not modelled. Decide whether
      the model states it (a derived value with that rule) or leaves it to the
      toolkit.
      M8 T21 fix round 1 (2026-10-17): the pressed colour is also an item of
      *Presets: values the M8 T15 platform-facts rows disagree with*
      (kde-breeze, the inherited `link.active_text_color`; §2.28: `Qt.darker`,
      #14405c / #0e4d79).

### KDE: the switch's hover and disabled thumb outlines

- [ ] [1.0] From M8 T16c (2026-10-17): `SwitchTheme`'s outline fields carry
      the states at least two platforms distinguish (rest and disabled, the
      track on and off; `docs/platform-facts.md` §2.21), and three values only
      qqc2-desktop-style distinguishes are not carried:
      - hover and keyboard focus: the off track's and the handle's outline
        turn `hoverColor`, `[Colors:Button] DecorationHover`, #3daee9 in
        both Breeze schemes (`SwitchIndicator.qml:30`, `:50`, `:78` at
        qqc2-desktop-style v6.31.0; `plasmadesktoptheme.cpp:311`; §2.21
        `hover_unchecked_track_border_color`, `hover_thumb_border_color`);
      - disabled: the handle's outline in the Disabled group, #e2e2e2 /
        #34373b (`plasmadesktoptheme.cpp:267-274`; §2.21
        `disabled_thumb_border_color`).
      The connectors draw the rest outline under the pointer and the enabled
      thumb outline on a disabled switch, so kde-breeze's hovered off switch
      keeps #cccfd2 / #4e5359 where Plasma draws #3daee9. Decide whether the
      model gains hover outline fields (and the KDE reader derives them from
      `DecorationHover`) and a disabled thumb outline.

### KDE: Union, the Application Style that is data (issue #4)

- [ ] [upstream] **Read the user's Union style.** KDE's Union styles are folders
      of CSS, so native-theme could follow a KDE Application Style other than
      Breeze (today the KDE reader states Breeze's sizes under every style,
      `src/kde/metrics.rs`). Rationale, and the reply posted on the issue:
      `docs/issues/0004_kde-union-rationale.md`. Start when both hold, checked
      at each Plasma feature release (6.9, 6.10, …): KDE enables Union by
      default (Plasma 6.8 ships it as an opt-in beta, "not quite ready to be
      used by default"), and at least one non-Breeze Union style is published
      (on 2026-10-22 the Union repository's `breeze`, `breeze-rtl` and
      `breeze-mobile` were the only ones, and store.kde.org had none). Then
      write the spec; the rationale's §5 sketches the reader.

---

## Toolkit Connectors

### native-theme-egui connector

- [ ] [1.0] Implement the companion widget crate, `native-theme-egui-widgets`, per
      `docs/archive/todo_egui-widgets-spec.md` (rationale:
      `docs/archive/todo_egui-widgets-rationale.md`): a switch, a slider, a spinner, a
      segmented control and link wrappers. Milestone undecided; it starts after
      the connector, whose API it consumes. When it is scheduled, re-verify the
      spec against the connector as built, then write its plan.
- [ ] [1.0] Map the platform font-rendering preferences (see Core API above) onto
      `Visuals::text_options` (`egui/src/style.rs:1001`) and the faces egui
      draws with. Map `font_hinting` (default `true`,
      `epaint/src/text/mod.rs:61`) from a stated no-hinting preference, and a
      stated light hint style onto each face's `FontTweak::hinting_target`
      (`epaint/src/text/fonts.rs:242`; `SmoothHinting::light`, `:340-348`) on
      the faces `fonts::font_definitions` assembles (connector spec §4.9). The
      other `TextOptions` fields and the two preferences egui cannot express
      are settled in the connector spec (§3.4, §5.10, §14 items 41–42).

- [ ] [upstream] Possible upstream egui issue: a `Button` given `frame_when_inactive(false)`
      changes size with its state. At rest egui lays it out with
      `Frame::new().inner_margin(..)`, no stroke, and hovered or pressed with
      the full frame, whose `bg_stroke` width adds to its outer size
      (`egui/src/widgets/button.rs:364-368`, egui 0.36.2; the full frame's
      margins, `egui/src/widget_style.rs:162-165`, cancel its stroke and
      expansion, the bare margin does not), so it grows on hover by twice
      the resting stroke width less the resting expansion: by twice the
      border's width wherever a style gives the resting entry a border, as
      native-theme-egui's do where the theme's border has a width
      (`BorderEntries`, `connectors/native-theme-egui/src/style/states.rs:38-43`). `Button::selectable` (`:81`), behind `selectable_label` and
      `selectable_value`, takes the same path. The egui showcase draws its
      Ghost buttons in a style scope instead (`demo::ghost`: the full frame in
      every state, the resting fill and border colour transparent), and
      `a_ghost_button_keeps_its_size_when_hovered_and_pressed` checks it.

### native-theme-gpui connector

- [ ] [1.0] Key the icon tables by `gpui_kit_assets::IconName` (gpui-kit 0.6.1:
      `ALL`, `PartialEq`, `Debug`, `Hash`) so the hand audit and the
      101-count tripwire in `icons.rs` become a compile-time check over
      `ALL`; the floor is gpui-component 0.6.6 since v0.5.9, so nothing blocks
      this. Verified 2026-09-09 that the released 0.5.8 connector builds and
      passes its tests unchanged on gpui-kit 0.6.1 with gpui-pre 0.3.3 and
      0.3.4.

- [ ] [1.0] **For M6: two defects in gpui's pressed button fill, shown by
      `macos-tahoe`** (M2a Step 9). macOS 26 draws the push button in a
      translucent fill, `#00000014` / `#ffffff14` (`docs/platform-facts.md:1177`),
      and no source gives its pressed fill, so the connector derives one.
      (1) `tc.button_active = tc.button.blend(tc.secondary_active)`
      (`connectors/native-theme-gpui/src/colors.rs:355`): `Hsla::blend` keeps
      the base's alpha, so a translucent layer over a translucent base is not
      composited (alpha 0.0784 where source-over gives 0.1507). (2)
      `derive::active_color` lightens a near-black base multiplicatively
      (`derive.rs:40-49`), which leaves pure black unchanged: in macos-tahoe
      light the pressed fill equals the idle fill, so pressing shows no
      change. The gpui contract excepts macos-tahoe for both
      (`contract.rs`): `PRESSED_OVER_TRANSLUCENT` fails once (1) is fixed,
      `DERIVED_PRESSED_FILL` once the pressed fill is opaque or stated. The
      hover fill is not affected: `macos-tahoe` states `macos-monterey`'s
      opaque `button.hover_background` (`#e0e0e5` / `#444446`), carried and
      marked "no macOS 26 source" (`docs/platform-facts.md:1192` is ❓; the
      inherited window colour is a wrong safety net,
      `docs/inheritance-rules.toml:167`), so the contract's check that a
      `TRANSLUCENT_BUTTON_FILL` combination's hover layer is opaque holds
      for that stated value.
      (1) fixed (M6 T9, 2026-10-15): `colors::over` composites the pressed
      layer source-over, so macos-tahoe light's `button_active` covers
      0.1507 (`pressed_fill_over_a_translucent_fill_is_source_over`), and
      the contract's `PRESSED_OVER_TRANSLUCENT` exception is gone. (2) stays:
      `DERIVED_PRESSED_FILL` still holds, the pressed fill is derived and
      translucent on both variants.
      M8 T21 fix round 1 (2026-10-17): no longer for M6, which closed on
      2026-10-16; it stays [1.0], with no milestone of the review assigned.

- [ ] [upstream] **`unicode-properties` back to 0.1.4 when gpui-fast-web
      unpins it.** gpui-fast-web 0.1.7, GPUI Fast's wasm platform crate, asks
      for `unicode-properties = "=0.1.3"`, and a lockfile resolves every
      target, so since the connector's `gpui-fast` feature (v0.7.0) the
      workspace's Cargo.lock holds 0.1.3 (it was 0.1.4) for every user of it:
      gpui-pre-web and `rustybuzz`, which the usvg of gpui-pre and of
      gpui-component's resvg depend on (`cargo tree --workspace -i
      unicode-properties --target all`, 2026-10-14; native-theme's own resvg
      has no text support and does not reach it). Once a gpui-fast-web
      release drops the exact pin, `cargo update -p unicode-properties` and
      check the lockfile is back on 0.1.4.
      *GPUI Kit 0.8.0 (2026-10-22, G3 T10):* gpui-fast-web 0.1.10 still pins
      `unicode-properties = "=0.1.3"`, so the lockfile stays on 0.1.3.

- [ ] [upstream] **File the gpui-fast issue: `Window::painted_quads()` omits
      what a rounded clip and a scroll layer paint.** For the maintainer to
      file on longbridge/gpui-fast (an agent files nothing). Under
      `test-support`, gpui-fast 0.1.10's `painted_quads()` (`src/window.rs:2925`)
      returns the rendered scene's own `quads` only. Since 0.1.10 the children
      of a rounded overflow clip are painted into a clip-group `LayerFrame`
      (`tile_size == 0`, `src/fast/rounded_clip_scene.rs:1-10`, `:60-75`), so
      their quads are not among them, and neither are the primitives of a
      composited scroll layer (a tiled `LayerFrame`, in 0.1.7's `Scene` too);
      `painted_underlines()` likewise. Our repro: on GPUI Fast,
      `a_loading_progress_bar_stands_still_under_reduced_motion` read no fill
      (the bar clips its fill to its rounded track). Our tests read
      `painted_scene()` instead (`scene_quads`, and the showcase's
      `scene_underlines`, in `connectors/native-theme-gpui/src/widgets/tests.rs`
      and `connectors/native-theme-gpui/examples/showcase-gpui/tests.rs`),
      descending into the clip-group frames, not the tiled ones (each clip
      group's scene holds a copy of those), so a test that asserts nothing is
      painted over a composited scroll layer still sees none of its content.
      Once `painted_quads()` and `painted_underlines()` include what the clip
      groups and the scroll layers paint, the descent can go. Found
      2026-10-21 (G3 Task 1).

- [ ] [1.0] **The layout dump on GPUI Fast is stale inside a scrolled,
      composited scroll layer.** The showcase's Widget Info registry
      (`connectors/native-theme-gpui/examples/showcase-gpui/info/registry.rs`)
      keeps each element of the list at the bounds it last recorded. Once
      GPUI Fast has seen a scroll area scroll for two frames
      (`PROMOTE_AFTER_SCROLLED_FRAMES`, gpui-fast 0.1.10
      `src/fast/layers/policy.rs:41`, `:286-287`), it composites the area
      while it scrolls: the content is drawn moved by the scroll, and its
      children's prepaint, where the elements record, is skipped
      (`src/fast/layers/paint.rs:233-235`, `src/fast/layers/reuse.rs:39`). So
      `InfoRegistry::layout_drawn`, and with it `--dump-layout`, reports the
      positions of the layer's last repaint for what is inside it, off by the
      scroll since; a `widgets::PartBounds` observer there is not called
      either. Hovers and the inspector's choice are unaffected: gpui moves
      the hitboxes, and the choice compares areas only. No gate reads such a
      dump: the screenshot and dump scripts build the gpui showcase on its
      default core, gpui-pre (`scripts/capture_window.sh`,
      `scripts/generate_screenshots_gpui.sh`), and G3 Task 9's GPUI Fast
      dumps are taken at rest, before any scroll streak. If a GPUI Fast dump
      after a scroll is ever needed, the dump calls `window.refresh()` before
      each reading: a refreshing frame turns the layers off
      (`src/fast/layers/mod.rs:130`) and reuse with them
      (`src/fast/retained.rs:602`), and `DumpSettle` already reads until the
      layout holds. Found 2026-10-21 (G3 Task 1).

- [ ] [1.0] **The gpui showcase keeps little of GPUI Fast's reuse.** Every
      Widget Info record is an update of the `InfoRegistry` entity made
      while the window draws (`elements.rs`, `record`'s and the recorders'
      `ui.update(..)`; `info/registry.rs`, the target's `record_layout` and
      bounds updates), which GPUI Fast counts as a write: a retained subtree
      that read the registry, or recorded into it before a view nested in it
      did, is built again in the next frame (gpui-fast 0.1.10
      `src/fast/dependencies.rs:757-797`, `note_update`; `:646-653`,
      `written_since`). Counted on 2026-10-22 at `98e1e954`, in a test
      window on GPUI Fast (`Window::layout_stats`, `src/fast/stats.rs`,
      with render counters on the showcase's views), with the pointer
      moved to, settled on and nudged within 40 targets of the Basic page
      and 40 of the chrome over the Layout page: 91 frames built 1127 views
      and drew 91 views from the last frame (7.5 %). `Showcase` was built in 89 of
      them, `Inspector` in 90, `MenuBar` in all 91, and on the Basic page
      the 8 `ListRow`s in every frame; gpui-pre drew the same 91 frames and
      built everything in each. Widget Info stays right (rationale K19's
      cost, not a defect). A way back to the reuse: the records kept
      outside the entity, in cells the registry shares with its recorders,
      so that drawing writes nothing gpui tracks, and the frame's end
      notifying only where the frame changed what the registry decides, as
      it does now. Found 2026-10-22 (G3 Task 9).
      G3 T9 review (2026-10-22): two views depend on the entity for their
      rebuilds: `Showcase` records into it while it draws (`app.rs`,
      `elements::record(&self.info_ui, ..)`), and `Inspector` observes it
      and reads what it shows (`inspector.rs`, `cx.observe(&ui, ..)`,
      `InfoRegistry::shown`). With the records outside the entity, the
      `Inspector` still needs the frame's-end notification, through its
      observation, and the `Showcase` is no longer built again for its own
      records.

- [ ] [upstream] **File the gpui-kit issue: on gpui-pre a labelled `Separator`
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
      `docs/todo_code-review-findings.md`, *G3*). The screenshot scripts
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
- [ ] [upstream] **File the gpui-kit issue: gpui-pre clips a rounded box's
      children to a rectangle.** For the maintainer to file on
      longbridge/gpui-kit (an agent files nothing). gpui-pre 0.3.9's content
      mask is a rectangle (`ContentMask`, `src/window.rs:2226-2234`: "only
      rectangular content masks are supported"), so in a box with rounded
      corners that clips both axes a child's square corner shows past the
      curve. gpui-fast 0.1.10 clips such a box's children to its curved
      padding edge (`src/fast/rounded_clip.rs`, from `src/style.rs:743`; only
      where both axes clip, `:68-70`, as CSS does; zed#37480), which 0.1.7
      and 0.1.8 did not. Our repro: on the gpui showcase's Basic page on
      gpui-pre, the selected Table row's fill is square over the Table
      frame's rounded bottom corners (adwaita dark: up to 190 levels at a
      pixel), and so are the Progress fill's start in its track, the List
      frame's right corners, the segmented control's bottom-left under a
      divider and the Expander's bottom-left under its body's text; on GPUI
      Fast each follows the curve (G3 Task 9's captures). The screenshot
      scripts build on gpui-pre, so the README screenshots show the square
      corners. Found 2026-10-22 (G3 Task 9). The issue, ready to paste:

      > **Title:** gpui-pre: a rounded `overflow_hidden` box does not clip
      > its children to its corners
      >
      > gpui-component 0.8.0 on its default core, gpui-pre 0.3.9, Linux.
      >
      > A box with `rounded(..)` and `overflow_hidden()` clips its children
      > to its bounds' rectangle, so a child that fills to the box's edge
      > (a `Table`'s selected last row, a `Progress` fill) shows its square
      > corners past the box's rounded ones. gpui-pre's content mask is a
      > rectangle (`src/window.rs:2226-2234`, "only rectangular content
      > masks are supported").
      >
      > gpui-fast 0.1.10 clips such a box's children to its curved padding
      > edge where both axes clip (`src/fast/rounded_clip.rs`, called from
      > `src/style.rs:743`; zed#37480), so the same tree on the `gpui-fast`
      > feature draws the corners round. Could gpui-pre take the same clip,
      > so both cores draw a rounded box alike?

#### Upstream PR candidates from v0.5.8 (Tier U)

- [ ] [upstream] a `ghost_hover` / `ghost_hover_foreground` token pair separate from
      `accent` / `accent_foreground` (0.6.4 hovers ghost buttons with the
      accent pair, which native themes map to the platform's selection
      colours, so a hovered ghost button becomes a selection-coloured pill;
      `button/button.rs:1125-1132, 1141`)
- [ ] [upstream] a menu-surface token. `PopupMenu` renders with `.popover_style(cx)`, i.e.
      `bg(theme.popover)` (`menu/popup_menu.rs:1476`, `styled.rs:193-199`), and
      there is no seam to override it. native-theme records
      `menu.background_color` separately from `popover.background_color`, and
      the two differ on 30 of the 32 preset/mode combinations (kde-breeze light:
      `#eff0f1` against `#ffffff`), so every menu is painted on the popover's
      surface. Found 2026-09-21 by running the contrast rule.
      *Re-measured 2026-10-11 (17 presets):* 25 of the 34 differ;
      kde-breeze, macos-monterey, macos-tahoe and material-3 in both modes
      and ios-13 light state one colour for both, so the kde-breeze example
      above is out of date.
- [ ] [upstream] a foreground token for the status bar. `StatusBar` labels everything
      with `muted_foreground` (`status_bar.rs:95`) over `tokens.status_bar`;
      measured against the platform's own `status_bar.font.color` on its
      `status_bar.background_color` that is worse on 32 of the 32 preset/mode
      combinations (measured 2026-09-21, 16 presets; nord dark: 1.69 against
      9.25). Since v0.5.9
      `geometry::status_bar` carries the platform's colour — upstream applies
      the caller's refinement after its own text colour, so it wins — but an
      application that does not use the builder gets the muted label. The
      gpui contract reports the token pair on every run.
- [ ] [upstream] foreground tokens for a selected list row and for selected text:
      `ThemeColor` has `list_active` and `selection` fills and no label colour
      for either, so a selected row keeps `foreground` where the platform pairs
      `list.selection_background` with `list.selection_text_color`. Worse than
      the platform on 24 of 34 for the row and 25 of 34 for selected text
      (measured 2026-10-11); reported by the gpui contract, not
      asserted. The title bar is the same shape (no `title_bar_foreground`),
      worse on 13 of 34 (measured 2026-10-11) against the darker end of
      upstream's title-bar gradient (`title_bar.rs:21-35`).
- [ ] [upstream] `Checkbox::label` sets `text_color(foreground)` on the label wrapper
      unconditionally (`checkbox.rs:334-341`), so a platform
      `checkbox.font.color` has no route; and both `Checkbox` and `Combobox`
      apply the caller's refinement *after* their disabled colour
      (`checkbox.rs:251-257`, `combobox.rs:990-997` with
      `input/input.rs:99-103`), so a carried colour would displace it. For
      that reason `geometry::checkbox` and `geometry::combobox` carry no text
      colour; a guard test fails, saying so, the day a preset states one.
      *GPUI Kit 0.8.0 (2026-10-22, G3 T10):* still so; in gpui-component 0.8.0 the label
      wrapper is `checkbox.rs:336-343` and the disabled colour before the
      caller's refinement `:253-259` (the refinement at `:265`);
      `combobox.rs:990-997` and `input/input.rs:99-103` are unchanged.
- [ ] [upstream] `WindowBorder`'s frame colour is a literal (`window_border.rs:152-166`)
      and so is the shadow it draws below it in the same `render`; no theme
      value reaches either.
- [ ] [upstream] a placeholder colour for `Input`. `Input::render` rebuilds the editor
      style from the theme on every frame and hands it `muted_foreground`
      (`input/input.rs:496-499`), which gpui-base paints the placeholder with
      (`input/base/element.rs:1825`); there is no setter. native-theme states
      `input.placeholder_color` from each platform's own placeholder colour
      (`placeholderTextColor`, `[Colors:View] ForegroundInactive`, ...), and
      `docs/inheritance-rules.toml` lists falling back to `muted_color` as a
      wrong safety net -- macOS's placeholder is about half the alpha of its
      secondary label. A `placeholder` token, or a setter on `Input`. Found
      2026-09-22 from a panel that said the placeholder read no theme field.
- [ ] [upstream] the focus ring's width. `focus_ring_style` draws the ring 3px wide at
      half the `ring` colour's alpha from two module consts
      (`styled.rs:11-12`), so the platform's `focus_ring_width` -- modelled,
      and used by the connector only to switch the ring on or off
      (`lib.rs:180`) -- has no receiver. A `Theme::focus_ring_width`.
      M8 T21 fix round 1 (2026-10-17): the connector's own widgets draw the
      same ring: *gpui: the theme-drawn widgets' focus ring*, "The connector's
      `Checkbox`, `Radio`, `Slider` and `Switch`".
- [ ] [v0.7.0] `tokens.tab`, `tokens.list`, `sidebar_primary` and
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
- [ ] [upstream] the segmented `TabBar` has receivers for the track only
      (`tab/tab_bar.rs:391`): the selected segment is filled with
      `tokens.background` and labelled `tab_active_foreground`
      (`tab/tab.rs:247-249`), so `segmented_control.active_background`,
      `.active_text_color`, `.font.color` and `.border.color` reach nothing.
- [ ] [upstream] `Button::icon` discards an `Icon`'s own size
      (`button/button_icon.rs:116-131`); a nested `TitleBar`'s window controls
      are hit-tested by the OS on Windows (`title_bar.rs:220-222`) and
      `on_close_window` is Linux-only (`:99-101`), so a title bar shown inside
      a window closes that window there.
- [ ] [upstream] `Checkbox` and `Radio` draw their unchecked border with `theme.input`
      (`checkbox.rs:238`, `radio.rs:188`), the *text input's* border colour.
      native-theme records `checkbox.unchecked_border_color` separately and the
      two differ on 18 of the 34 preset/mode combinations (re-counted
      2026-10-11) — windows-11 light wants `#00000072` where the input border
      is `#0000000f`. The token is
      named `input`, so the connector feeds it the input border; upstream needs
      a checkbox border token.
- [ ] [1.0] Whether any platform scales a control's corner radius with its control
      size. platform-facts records one `border.corner_radius` per widget on all
      four platforms (§2.3 and the per-widget tables), and records size classes
      only for *dimensions* — NSTextField 22/19/17, NSSwitch 38x22/32x18/26x15,
      NSPopUpButton 21/18/15/24. On macOS the bezel corners are baked into
      CoreUI `.car` artwork with no queryable constant (`:1343`), so a
      size-dependent radius there is unmodelled rather than known to be absent.
      If one exists, `ResolvedBorderSpec` would need a per-size radius and
      `geometry::button` / `input_group_button` would follow it; today both
      restore the widget's single radius, which is what the model carries.
- [ ] [upstream] a control's radius should not shrink with its size: an `XSmall`
      `InputGroupButton` takes `radius / 2` (`input/group.rs:590-593`) and
      `Button` does the same for its small and large roundings
      (`button/button.rs:593-595`), where a platform records one radius per
      widget whatever its size. `geometry::button` and
      `geometry::input_group_button` restore it per call site.
- [ ] [upstream] `Toggle` needs a token of its own for the pressed state: it reads
      `tokens.accent` (`button/toggle.rs:155, 202`), the item highlight of
      menus and lists. Since v0.5.9 the connector feeds that token the
      platform's menu hover pair, which on Adwaita, Windows 11 and Material is
      a subtle fill, while those platforms' `segmented_control.active_background`
      is the accent colour. KDE and macOS are unaffected (both are the
      selection colour).
- [ ] [upstream] `TabVariant::Outline` hovers with `tokens.secondary_hover`, the *button*
      hover (`tab/tab.rs:233` in gpui-component 0.8.0, `:187` in 0.7.3); every
      preset has a different
      `tab.hover_background` (Breeze: `#dee0e2` against the button's
      `#93cee9`). The default `Tab` variant has no hover fill and is unaffected.
      G3 T6 (2026-10-22): GPUI Kit 0.8.0's `TabVariant::Folder` hovers with a
      button token the same way, an inset fill in `tokens.secondary`
      (gpui-component 0.8.0 `tab/tab.rs:1012`), which the connector maps from
      `button.background_color` (native-theme-gpui `src/colors.rs:205`), so
      `tab.hover_background` reaches neither; the gpui showcase's Layout page
      shows the Folder bar, its Widget Info noting the hover so.
- [ ] [upstream] `InputGroupButton` should hover with the platform's button hover, not
      `muted`: `render_in_group` hardcodes `cx.theme().muted` (halved in dark)
      for an in-group addon button (`input/group.rs:544-583`), which is the
      muted-background slot, not a hover colour. Measured on kde-breeze light
      it gives `#dbdcdd`, 20 units from the `#eff0f1` field, while KDE's
      `[Colors:Button] DecorationHover` (our `secondary_hover`) is `#93cee9` at
      57 — so the addon button is grey while every other button in the window
      is blue. `secondary_hover` / `button_secondary_hover` is the token it
      wants.
- [ ] [upstream] a `Dialog::max_h` prop, or letting the caller's `max_h` win: 0.6.4
      clamps the dialog to the viewport after `refine_style`
      (`dialog/dialog.rs:631`), so a themed `dialog.max_height` cannot reach
      it, while the width already has `Dialog::max_w`
- [ ] [upstream] a styled `Theme` scrollbar-style override honoured by `base_theme()`
      (would make the connector's re-apply observer unnecessary)
- [ ] [upstream] a scrollbar thumb inset per axis. gpui-base takes one inset for both
      directions and draws the thumb `2 × inset` shorter than its clamped
      length (gpui-base 0.7.3 `scrollbar.rs:1211-1216`); the connector centres
      the thumb with `(groove_width − thumb_width) / 2` and pays for the two
      insets in `min_length` (`base_layer::scrollbar_styles`), so the drawn
      thumb is at least `scrollbar.min_thumb_length`, but a proportional thumb
      is still `2 × inset` shorter than its share of the track (13 px on
      kde-breeze, 11 px on windows-11). A cross-axis inset alone would centre
      the thumb without shortening it.
      *GPUI Kit 0.8.0 (2026-10-22, G3 T10):* still so at gpui-base 0.8.0: the thumb is
      drawn `2 × inset` shorter (`scrollbar.rs:1225`) than its length, now
      `viewport / content × track` clamped to `min_length` and the track
      (`:1219-1220`).
- [ ] [upstream] `Tab` keeping the caller's height, radius and text size (its render writes its own into the style bag the caller's setters fill, `tab/tab.rs:801-808`)
      *GPUI Kit 0.8.0 (2026-10-22, G3 T10):* still so; the lines are `tab/tab.rs:958-966`
      in gpui-component 0.8.0 (`.h(height)` … `.rounded(radius)`).
- [ ] [upstream] `Theme.shadow` honoured beyond `Button`; `tokens.shadow` consumed
      M8 T21 fix round 1 (2026-10-17), the duplicate rule: the lines of the entry
      "**`Theme::shadow` has one reader.**" (*native-theme-gpui connector*,
      *Follow-ups from the v0.5.9 showcase and contract work*; 6faef5b2):
      **`Theme::shadow` has one reader.** The connector sets it from
      `border.shadow_enabled` (`lib.rs:176`); gpui-component reads it only at
      `button/button.rs:612`, for a `ButtonVariant::Custom` built with
      `.shadow(true)`. The semantic shadow tokens it feeds have no reader
      outside `theme/`. So the platform's shadow preference reaches nothing
      an application builds by default. Found 2026-09-22 from eleven panels
      that listed it as theme config.
- [ ] [upstream] `Size::Size` honoured by `Checkbox` and `Switch`
- [ ] [upstream] a disabled `Button` should read the platform's disabled colours.
      `ButtonVariant::disabled` (`button/button.rs:1273-1284`) derives the
      whole state by multiplying: the variant's own token at 0.15 for the
      fill, `muted_foreground` at 0.5 for the text. native-theme models
      `button.disabled_background`, `button.disabled_text_color` and
      `button.disabled_opacity` (the last inheriting
      `defaults.disabled_opacity`), every platform states them, and upstream
      reads none of the three. Found 2026-09-22 while auditing the showcase's
      "Not themeable" notes, which had said "hardcoded 0.5" -- the wrong
      number for the fill and the wrong story for the text. Since the
      disabled rule of 2026-09-28 (`docs/platform-facts.md` §2.1.6: the
      disabled colours and `disabled_opacity`, one of them an identity in the
      data) the same holds for `Input`, `Select` and every other
      gpui-component control with a disabled state: nothing reaches its
      opacity, so on adwaita (0.5) a disabled gpui-component control is not
      faded as libadwaita fades it. The connector's `widgets::Checkbox`,
      `Radio`, `Switch` and `Slider` apply both.
- [ ] [upstream] the `Link` widget should read `link_hover` and `link_active`. It computes
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
      M9a T15 (2026-10-18, 8524e44d): the underline half is closed, by
      e6b2034e: `geometry::link` removes the resting underline where
      `link.underline_enabled` is false with a zero thickness
      (`text_decoration_0`), a `Some` the refinement carries over upstream's
      `text_decoration_1`, and a Link's hover and press still underline. The
      gpui showcase's Link info now says so (`ios-13.toml`
      `[light.link] underline_enabled`, false). The `link_hover` and
      `link_active` half stays.
- [ ] [upstream] a `PopupMenu` item style hook; a `Button::tooltip` style hook
- [ ] [upstream] button label text size independent of rem
      M8 T21 fix round 1 (2026-10-17): the same label is "PR: Button: let the
      caller's text size reach the label" (*native-theme-gpui connector*,
      *Upstream PRs to gpui-component*).
- [ ] [upstream] public base-palette fields on `ThemeConfigColors` (`red` … `cyan_light`, `schema.rs:657-668`)
- [ ] [upstream] internal scrollbars reserve no gutter where the platform's scrollbars
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
      *GPUI Kit 0.7.3 (2026-10-14):* the Settings page body no longer pads the
      list: each group's row is wrapped in a div carrying that `px_4`, inside
      the list's clip, so focus rings can reach past the group's edges
      (`setting/page.rs:239-243`; the scrollbar at `:255`). The 21px groove
      still covers the row's right edge, so the gutter is still needed.
      M9a T20 (2026-10-18): the showcase's `Tree` rows keep clear of it as
      its List rows do since ee836dc1: each row's `ListItem` takes a right
      margin of the groove's width where the theme's scrollbar is no overlay
      (`demo::list_row_item`, `list_gutter`), a workaround of the
      showcase's, which the rows' Widget Info names; gpui-component 0.7.3
      still lays the Tree's bar over its rows (`tree.rs:119`,
      `vertical_scrollbar`).
      *GPUI Kit 0.8.0 (2026-10-22, G3 T10):* still so. The Settings page body's padded
      row is `setting/page.rs:241-245`, its scrollbar `:257`; the `Tree` lays
      its bar over its rows at `tree.rs:132`, the rows now in a `ScrollBounce`.
- [ ] [upstream] a right-clicked `Tree` row shows the hover text colour without the
      hover fill: gpui-component 0.7.3 marks the row right-clicked after the
      application's `render_item` closure built it
      (`.secondary_selected(entry_state.is_right_clicked())`, `tree.rs:90-92`),
      and hands the closure the selection alone (`tree.rs:89`); `TreeState`
      keeps the right-clicked row in a private field (gpui-base 0.7.3
      `tree.rs:196`, `right_clicked_ix`) with no public reader. A right-clicked
      `ListItem` draws no hover fill (`list/list_item.rs:190`, `:227-233`),
      while the showcase's label, which cannot know, still takes
      `list.hover_text_color` under the pointer (`demo::tree_row`). Upstream: the
      closure given `TreeEntryState`, or a public reader on `TreeState`. Found by
      M9a T18's review (2026-10-18).
      M9a final fix (2026-10-18): the showcase's label cannot know only
      through gpui-component's `Tree`. gpui-base 0.7.3's own `Tree` hands its
      `item` closure the whole `TreeEntryState` (`tree.rs:188`, `RenderItem`,
      built at `:458`), whose `is_right_clicked` is public (`:183`);
      gpui-component's wrapper forwards `is_selected()` alone (`tree.rs:89`),
      so the upstream fix is that one argument. The showcase, which names
      `gpui_base` already (`demo.rs`, `host.rs`), could build its Tree on
      `gpui_base::Tree` instead, at the cost of re-adding what the wrapper
      adds: the disabled, selected and right-clicked marks (gpui-component
      `tree.rs:90-92`), the context menu (`:96`) and the scrollbar (`:119`).
      *GPUI Kit 0.8.0 (2026-10-22, G3 T10):* still so. gpui-component's wrapper hands
      `render_item` the selection alone (`tree.rs:98`) and marks the row after
      it (`:99-101`); its context menu is `:105` and its scrollbar `:132`.
      gpui-base's `Tree` is unchanged at `tree.rs:183`, `:188`, `:196` and
      `:458`, and `list/list_item.rs:190`, `:227-233` are unchanged.
- [ ] [upstream] element hooks on the surfaces upstream builds itself, so an
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
      *GPUI Kit 0.7.3 (2026-10-14):* a `SettingGroup` now renders inside a
      padded div the page's `list` builds for it (`setting/page.rs:237-251`),
      and a `Dialog`'s surface is built at `dialog/dialog.rs:716-802` and then
      wrapped for its entrance animation (`:803-841`); neither offers a hook.
      `sheet.rs` is unchanged.
      *GPUI Kit 0.8.0 (2026-10-22, G3 T10):* still no hook. The padded div round a
      `SettingGroup` is `setting/page.rs:239-253`; a `Dialog`'s surface is
      built at `dialog/dialog.rs:718-804` and wrapped for its entrance at
      `:805-843`; `sheet.rs` and `setting/group.rs` are unchanged.
- [ ] [v0.7.0] the gpui connector reads `AccessibilityPreferences::high_contrast`
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

What is still open on the iced side:

- [ ] [1.0] The iced `geometry` gap stays: the connector has no counterpart of the
      gpui connector's `geometry` module, because iced takes geometry through
      builder methods on each widget. Native values with a builder receiver
      (`Checkbox::size` / `Radio::size` ← `checkbox.indicator_width`,
      `::spacing` ← `label_gap`, `Toggler::size` ← `switch.track_height`,
      `ProgressBar::girth` ← `progress_bar.track_height`, the rule's thickness
      ← `separator.line_width`, `pick_list::Handle::Arrow { size }` ←
      `combo_box.arrow_icon_size`) are named in the style functions' doc
      comments and applied by the showcase; spacing comes from the model's
      `LayoutTheme`, which is public on `Theme` / `SystemTheme` and needs no
      connector API. `scripts/check_widget_coverage.py` keeps *widget*
      coverage visible; nothing mechanical yet lists native geometry fields
      that no iced builder receives.
- [ ] [upstream] Unreachable in iced 0.14 / iced_aw 0.14.1, recorded so they are not
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
- [ ] [upstream] iced_aw 0.14.1 defects found by the showcase self-tests, worth filing:
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
- [ ] [upstream] File upstream iced_aw issue: Card close button ignores the caller's
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

- [ ] [1.0] 174 of 512 text-on-background pairs sit below WCAG AA across the 16
      presets in both modes (measured 2026-09-20, before macos-tahoe; 16
      presets; alpha composited; on 2026-10-11 the gpui contract test
      `no_pair_contrasts_worse_than_the_platforms_own` prints 284 sub-AA lines
      over a larger pair set, 23 asserted pairs and the reported ones × 34
      combinations, so the two counts are not comparable). Most are
      the platform's own choice — macOS ships `success_color = "#34c759"` with
      white text at about 2.2:1 — and the connectors must not override a
      platform. But some look like preset data errors rather than platform
      facts, and `preset-validator` should judge them: `material-3` light and
      dark give `tab.active_text_color` and `tab.active_background` values
      that leave the label on its own fill, and `nord` dark puts
      `input.placeholder_color` at 1.36 against the field. The contrast test
      added in v0.5.9 prints the whole list on every run, so it stays visible.

- [ ] [1.0] kde-breeze states `button.hover_background = "#93cee9"` in both modes
      under a `#fcfcfc` label in dark mode: 1.67:1 (the pressed pair is 2.43
      in light mode). platform-facts records the KDE source as "`[Colors:Button]
      DecorationHover` **blend**" (`platform-facts.md` §2.3, the button's
      `hover_background` row, before M8 T15), and `#93cee9` is
      the raw `DecorationHover`, which Breeze paints as an outline — so the
      preset may be recording the unblended colour as a fill. Found 2026-09-21
      by the iced contrast report; for `preset-validator` to judge. The same
      report shows `solarized` (both modes) and `tokyo-night` light with *idle*
      button labels at 3.6–4.1:1.
      *M2e fix round 1 (2026-10-11):* the same unblended `#93cee9` is
      kde-breeze's `checkbox.hover_background`, `scrollbar.thumb_hover_color`,
      `slider.thumb_hover_color` and `combo_box.hover_background` in both
      modes and `list.hover_background` and `sidebar.hover_background` in
      light; dark's list and sidebar hover is `#2e4a5e`. The facts cells
      give no colour: "`[Colors:View] DecorationHover` blend"
      (`docs/platform-facts.md:1379`, `:1435`), "(Breeze src) thumb hover"
      (`:1306`). `segmented_control.hover_background` and
      `expander.hover_background` (`#dee0e2`, dark `#31363b`, which is no
      BreezeDark colour) have no facts row. Values unchanged until a source
      gives them.
      M8 T21 (2026-10-17): since M8 T15 that row's KDE cell
      (`docs/platform-facts.md:1192`) reads "← `background_color`: no hover
      fill" (breezehelper.cpp:693-707), hover being the `Highlight` outline;
      the button's `#93cee9` is listed in *Presets: values the M8 T15
      platform-facts rows disagree with* ("From M8 T15 (2026-10-17)", fix
      round 2).
      M8 T21 fix round 1 (2026-10-17): the button half is fixed: since
      dc1b81aa (M8 T16) kde-breeze's `button.hover_background` is the rest
      fill, `#fcfcfc` / `#292c30` (`kde-breeze.toml`
      `[light.button] hover_background`, `[dark.button] hover_background`), §2.3's "no
      hover fill". `#93cee9` stays in `checkbox.hover_background`,
      `scrollbar.thumb_hover_color`, `slider.thumb_hover_color` and
      `combo_box.hover_background` in both modes and in light's
      `list.hover_background` and `sidebar.hover_background`; *Presets: values
      the M8 T15 platform-facts rows disagree with* lists the checkbox's, the
      slider's and the combo box's, and the scroll bar's only as the source of
      `thumb_active_color`, so the list and sidebar hovers and the solarized
      and tokyo-night label contrasts are this entry's alone.
      M8 T21 fix round 1 (2026-10-17): the
      `segmented_control.hover_background` `#dee0e2` / `#31363b` above is also
      in *M6 close (2026-10-16): items carried to M7, M8, M9a and the
      presets*, "Presets (kde-breeze), found by M6 T16b's", and in *Presets:
      values the M8 T15 platform-facts rows disagree with* (kde-breeze,
      §2.25).
- [ ] [1.0] `button.hover_text_color` and `button.active_text_color` equal
      `button.font.color` in all 16 presets, so the per-status label rows of
      both contracts cannot tell hover or pressed from idle today. Either the
      platforms really keep the label, or the presets never recorded it.
      *Re-measured 2026-10-11 (17 presets):* the hover label equals the idle
      one in all 34 combinations; the pressed label differs in three:
      kde-breeze light (`#ffffff` against `#232629`) and windows-11 in both
      modes (`#0000009e` against `#000000e4`, dark `#ffffffc5` against
      `#ffffff`).
- [ ] [1.0] The progress bar's fill on its track is below AA on 32 of 34 (the iced
      contract, measured 2026-10-11: macos-tahoe light 1.02, adwaita light
      1.21; material-3 is the exception). It is the platform's own accent-on-muted
      pair, emitted exactly — an indicator, not text — but it is the first
      thing that looks like a bug when the showcase is opened.
- [ ] [1.0] No bundled preset states `segmented_control.background_color`, so it
      inherits the window background — the colour gpui-component fills the
      *selected* segment with. Since v0.5.9 maps the segmented track from that
      field, track and selected segment are one colour until the presets state
      the platform's track.
- [ ] [1.0] kde-breeze states `switch.thumb_diameter == switch.track_height` (18/18),
      so `styles::toggler` emits a thumb inset of zero there. Breeze does draw
      a handle as tall as its groove; confirm against Breeze's metrics.
- [ ] [1.0] Since v0.5.9 the iced connector writes `background.base.text` from
      `defaults.text_color`, where iced's `readable()` used to substitute a
      higher-contrast colour below 6.0:1. That now shows the platform's own
      4.13 (solarized light), 4.75 (solarized dark) and 4.52 (tokyo-night
      light) window text. If those are preset errors rather than the themes'
      own values, the presets are where to correct them.
- [ ] [1.0] How a platform draws a *disabled checked* checkbox is not modelled.
      `platform-facts.md` §2.5 records no disabled indicator colour; the one
      disabled mechanism it records for a checkbox is `disabled_opacity`
      (dim the accent fill and the on-accent mark together). The model instead
      gives one `checkbox.disabled_background` for checked and unchecked alike,
      and `checkbox.disabled_opacity` has **no receiver** in native-theme-iced
      (iced's `checkbox::Style` has no opacity; it would have to be multiplied
      into the emitted alphas). Since v0.5.9 `styles::checkbox` paints the
      disabled mark with `checkbox.disabled_text_color` on that fill — the
      platform's own disabled pair, 1.07–4.02:1 over the 34 combinations (the
      iced contract's report, 2026-10-11), where in v0.5.9 the on-accent mark
      on the neutral fill was worse than the platform's own in 26. Decide
      whether the model should state a disabled *checked* fill (or the
      connector apply `disabled_opacity`), then revisit that arm; it is
      one match arm plus `native_checkbox_mark` in the contract.
      Half settled 2026-09-28 by the disabled rule (`docs/platform-facts.md`
      §2.1.6: a connector applies the disabled colours *and*
      `disabled_opacity`; the data makes the one the platform does not use an
      identity): `styles::checkbox` now multiplies every disabled colour's
      alpha by `checkbox.disabled_opacity`, and where no `disabled_background`
      is stated (adwaita) a disabled box is its enabled self, checked fill and
      on-accent mark included. What stays open is a platform that dims by
      colour and draws a checked box differently from an unchecked one
      (Breeze's Disabled palette has a Selection set, `#e3e5e7` / `#1f2124`,
      computed with KColorScheme 6.30.0): the model's one
      `disabled_background` cannot say so.
- [ ] [1.0] Audit the other `docs/inheritance-rules.toml` rules against
      platform-facts the way `checkbox.indicator_color` was found: the
      second half of `docs/archive/todo_closed.md`, *Closed items from
      Research*, "`checkbox.indicator_color` inherited `defaults.text_color`
      until v0.5.9", which 890edc88 left open.
- [ ] [1.0] windows-11 light gives `button.disabled_background = "#f9f9f900"` — alpha
      zero, so a disabled button has no fill at all. Found 2026-09-21 while
      measuring translucent state colours; for `preset-validator` to judge
      against Fluent's `ControlFillColorDisabled`.
      M8 T21 fix round 1 (2026-10-17), the duplicate rule: the lines of the entry
      "`windows-11` states `checkbox.disabled_background`" (e9281823, which stood
      two items above this one; its "button item below" is this entry):
      `windows-11` states `checkbox.disabled_background = "#f9f9f900"` (alpha
      zero) and `material-3` `#1c1b1f1f`, so on those four combinations a disabled
      box is effectively absent — the same shape as the button item below.
      M8 T21 fix round 1 (2026-10-17), the duplicate rule: the lines of the Core API
      entry *windows-11: the disabled fills are fully transparent* (5cfe6220), whose
      heading goes with it:
      `windows-11.toml` states the `disabled_background` of `button`, `input`,
      `checkbox` and `combo_box` as `#f9f9f900` in light and `#33333300` in
      dark (`[light.button] disabled_background`,
      `[light.input] disabled_background`,
      `[light.checkbox] disabled_background`,
      `[light.combo_box] disabled_background` and the same four in `[dark.button]`,
      `[dark.input]`, `[dark.checkbox]` and `[dark.combo_box]`): alpha `00`, so
      a disabled control shows no fill at all. The button's comment names the
      source, "Fluent ControlFillColorDisabled" (above
      `[light.button] disabled_background`, `[dark.button] disabled_background`),
      and WinUI 3 defines that brush as `#4DF9F9F9` in the
      `Light` dictionary and `#0BFFFFFF` in `Default`, the dark one — ARGB,
      so about 30 % and 4 % opaque, and white rather than `#333333` in dark
      ([Common_themeresources_any.xaml:223](https://github.com/microsoft/microsoft-ui-xaml/blob/2b8c7757ef2dd57234d5d7c3016d136c59a24bad/controls/dev/CommonStyles/Common_themeresources_any.xaml#L223),
      [:19](https://github.com/microsoft/microsoft-ui-xaml/blob/2b8c7757ef2dd57234d5d7c3016d136c59a24bad/controls/dev/CommonStyles/Common_themeresources_any.xaml#L19)). Run the `preset-validator` agent over `windows-11.toml`
      against that source, record the brush in `docs/platform-facts.md`, where
      it is not yet, and correct the eight values.

- [ ] [1.0] Scrollbar thumb radius per platform: platform-facts records none, so the
      connector mirrors gpui-component's `radius` for the thumb.
- [ ] [1.0] gpui-component's `Theme::motion` (`MotionTokens`, eleven fields: four
      durations, three easings, two springs and two `Rems` travel distances,
      `src/theme/motion.rs:8-20`) stays at upstream's default, because
      `ResolvedTheme` has no motion field at all; KDE exposes
      `AnimationDurationFactor` and GNOME `enable-animations`, which
      native-theme reads only as the boolean `reduce_motion`
      (`native-theme/src/kde/mod.rs:42-52`, `src/detect.rs:674-685`). Decide
      whether native-theme should model motion — durations, easings,
      distances — which is a core-type change.
      2026-10-15: since c1208377 (M6 T5, M6-01) `apply` no longer overwrites
      `motion` with upstream's defaults: it copies the installed theme's before
      the write, as upstream's `Theme::change` leaves it (gpui-component 0.7.3
      `src/theme/schema.rs:1064-1108`), so a value the application set
      survives a theme switch; the connector still sets none.
      M8 T21 fix round 1 (2026-10-17), the duplicate rule: the lines of the entry
      "**Model motion.** The big one" (*native-theme-gpui connector*, *Follow-ups
      from the v0.5.9 showcase and contract work*; 923471e9; its "argued above"
      means its place in `docs/todo.md` at b2524822), all but its 2026-10-15 line,
      which is the line of that date above:
      **Model motion.** The big one, argued above: `Theme::motion` is a
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
- [ ] [1.0] Preset font families that are not installed: GPUI resolves a named,
      missing family through its fallback stack on every text run (upstream's
      statement, gpui-component 0.6.4 `src/theme/system_font.rs:3-9`; not
      measured here). Decide whether `to_theme` should fall back to
      `.SystemUIFont` when `cx.text_system().all_font_names()` lacks the
      family, as upstream does for its own defaults.
- [ ] [1.0] Widen the captured screenshot tabs: every capture passes `--tab buttons`
      (`scripts/generate_screenshots_gpui.sh:70`, `screenshots.yml`), so the
      InputGroup, Empty, Carousel, code-editor and Markdown sections never
      appear in an artefact. Since 2026-09-28 every capture passes
      `--tab basic`, the Basic page all three showcases draw alike, so
      that the captures compare; those sections still appear in none.
      Since M8 T12 (7e450822) the tab is passed in one place,
      `capture_showcases` in `scripts/capture_window.sh`.
- [ ] [1.0] Re-verify the upstream `file:line` citations in this file,
      `docs/todo_gpui-full-theme.md` and `ROADMAP.md`, which are still
      0.6.0-era; four are known stale at 0.6.4 (`popup_menu.rs:749` — fixed in
      the gap analysis, `button.rs:360`, `switch.rs:136-146`,
      `checkbox.rs:195-199`).
      M8 T21 fix round 1 (2026-10-17): a mechanical check of such citations is
      *gpui: a citation gate for the connector's doc comments*, "The
      showcase's citations are checked by gates".

#### Follow-ups from the v0.5.9 showcase and contract work

- [ ] [1.0] **Open question for the maintainer, carried out of the archived Widget
      Info rationale (§6, question 2):** the panel displays the citation
      beside the swatch — `text: link #2a7ab0 (button.rs:993)`. It is what
      makes a claim checkable by a reader and not only by a test, but it is
      also four widgets' worth of line numbers in a hover panel. Keep it, or
      keep the citation in the source and show only `text: link #2a7ab0`?
      The other three questions in that section answered themselves: the
      work landed on the v0.5.9 branch, the six Button variants' field names
      were corrected by the citation pass, and `Command` has a real demo.
- [ ] [1.0] **Finish auditing the Widget Info "Not themeable" entries for
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
- [ ] [1.0] Two focus claims hold only while `Theme::focus_ring` is on — the
      Switch panel's ring and the DataTable panel's keyboard-focus line: with
      it off a bordered element's border is tinted instead
      (`styled.rs:269-271`). Every bundled preset turns it on
      (focus_ring_width > 0), so nothing is false today; branch the claims as
      the Carousel panel does if a preset ever states 0.

##### What the audit turned up that is ours to fix

Each of these is a real route the connector or the model does not take. They
are listed separately from the notes because correcting a note only records
the gap — closing it is a change, and each wants its own decision.

- [ ] [1.0] **There is no ambient `font.weight`.** The `geometry::` builders that
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
- [ ] [1.0] **Model the widgets the panels keep apologising for.** Alert, Tag, Kbd
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
- [ ] [v0.7.0] **Carry `defaults.line_height` to the code editor.** `Editor` sets
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
- [ ] [1.0] **The showcase ignores its own `text_scale`.** `ResolvedTextScale` states
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
- [ ] [1.0] **`Size::Size(px)` is an escape hatch, but read the target first.** It is
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
      0.7.1 (2026-10): `button_text_size` is `text_sm` at Medium
      (`sizing.rs:337-343`), so a `.label()` is 0.875 of the platform font;
      `geometry::button_label` on a child label carries `button.font`
      (v0.6.1).
- [ ] [1.0] **A gate the audit broke, and what that says about the others.** Writing
      `(select.rs, Caret::render)` into a note made `Caret` a "shown" widget
      in `check_widget_coverage.py`, because the gpui side read string
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
- [ ] [1.0] **A sidebar width and a tooltip delay are missing from the model.** Both
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
      *GPUI Kit 0.7.3 (2026-10-14):* the Button half is no longer true. A
      Button's tooltip delay has a receiver, `Button::tooltip_show_delay`
      (`button/button.rs:422`), and the overlay's 500ms show delay and 300ms
      grace are no longer module consts but the application-wide
      `TooltipDefaults` (`gpui-base/tooltip.rs:58-108`), which a trigger's own
      delay overrides (`:227-229`). gpui's own `.tooltip()` still defaults to
      500ms (`gpui-pre/elements/div.rs:53`) and takes `tooltip_show_delay`
      (`:1708`). The 0.7.3 work's G2 closes the tooltip half with a model
      value; the sidebar width stays open.
      *G2 (2026-10-14):* the tooltip half is done. `tooltip.show_delay_ms`
      is model data (3ddfe91a), stated by `kde-breeze` 700, `adwaita` 500,
      `windows-11` 800 and `material-3` 0 from `platform-facts.md` §2.7, and
      unset on macOS and iOS, which state none. Its receivers: gpui's `apply`
      installs it as the `TooltipDefaults` show delay and `tooltip_show_delay`
      carries it to GPUI's own `.tooltip()` (0ae7be3c, b95ebe2b), egui's atlas
      writes `interaction.tooltip_delay` (73e8c5ca), iced's
      `tooltip_show_delay` is for `Tooltip::delay` (138c2674). The sidebar
      width stays open.
- [ ] [1.0] **Platform icons inside widgets: how far can it go?** `Icon` takes a path
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
- [ ] [1.0] **`geometry::button` gives every variant a border, and hover takes it
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
- [ ] [v0.7.0] **`geometry::list` could paint the list.** `ThemeColor::list` has no
      reader (the contract says so at `contract.rs:1784`) and a `ListItem`
      paints no idle background, so `list.background_color` reaches nothing.
      On the bundled presets it falls back to the window background
      (`resolve/inheritance.rs:143-144`), so nothing is lost there; on a live
      KDE system it is `[Colors:View] BackgroundNormal` (`kde/colors.rs:91`),
      the view background rather than the window's, and that is dropped. `geometry::list` lands on the element around the rows, which
      is exactly where a fill belongs, and carries only the border
      (`geometry.rs:362-369`). One `.bg()` would carry it. Found 2026-09-22.
- [ ] [v0.7.0] **`Switch::color` is a receiver nothing feeds.** `switch.checked_background`
      is modelled and every preset states it; `ThemeColor` has no field for
      it, which is what `colors.rs`'s Issue 51 note and the contract's
      `NoReceiver` entry record -- but `Switch::color` (`switch.rs:95`) takes
      it per instance. A helper, or a `variants::` function like
      `ghost_button`, would reach every switch an application builds. Found
      2026-09-22.
      M6 T16c (2026-10-16, 12be97dd): with `widgets` the leaf reaches
      `SwitchLook` (the connector's own switch draws it), so the contract's
      `NoReceiver` entry for it exists only without `widgets`.
- [ ] [v0.7.0] **The code editor's background is a fixed colour.** `Theme::editor_background`
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
- [ ] [1.0] **A tab's `min_height` has a receiver nobody uses.** A `Tab` takes the
      caller's style first and then sets its own per-Size `.h()`, text size and
      colours over it (`tab/tab.rs:780-810`), but it never sets `min_h`, and
      the layout honours a minimum over a height. So `tab.min_height` (and
      `min_width`, and a font weight, which the tab sets nowhere either) would
      reach a tab the application builds -- there is simply no `geometry::tab`.
      The showcase builds its tabs from strings, so it would need to build
      `Tab`s to use one. Found 2026-09-22.
- [ ] [1.0] **An empty state's icon outgrows its frame.** `EmptyMedia` is a 2rem
      square (`empty.rs:217`), and the showcase puts
      `defaults.icon_sizes.large` in it -- 32px on most presets and 48px on
      KDE -- which fits only where 2rem reaches the icon, i.e. a 16px body
      font (24px for KDE's 48). `EmptyMedia` applies the caller's refinement
      last, so the frame can be sized to its icon; decide whether the
      showcase should, or use a smaller icon role. Not checked visually.
      M9a T15 (2026-10-18): the gpui showcase's Empty state info prints the
      icon's size from the installed theme's `defaults.icon_sizes.large`
      (`geometry::icon_size_large`), and upstream's one rem where no native
      theme is installed, where it printed "32px, or 48px on KDE"; the frame
      is still 2rem, so the question stays open.
- [ ] [1.0] **The colour gate cannot tell whose line it is.**
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
- [ ] [1.0] **Reference — how a theme colour reaches a gpui-component widget.**
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
- [ ] [1.0] Port `scripts/check_widget_coverage.py` to a `#[test]`, as the Widget
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
      `scripts/generate_gifs_spinners.py` is deliberately **not** included: it is
      release tooling a human runs, not a gate, and nothing can silently
      skip it because its output is the screenshots the asset stamp checks.
- [ ] [1.0] Split the showcases into modules: `showcase-gpui.rs` and
      `showcase-iced.rs` are several thousand lines each after gaining every
      widget and their self-tests. The gpui half is done: the example is
      `examples/showcase-gpui/`, a module tree (ab3f7d6). `showcase-iced.rs`
      remains one file.
- [ ] [1.0] `scripts/check_widget_coverage.py` still accepts weak evidence of
      "shown" on the **iced** side: an import of the module is enough. The
      gpui side no longer does — `shows_gpui` requires a constructor, a call,
      a struct literal or a named extension method, and rejects a name that
      is a segment of somebody else's path. The same tightening for iced
      wants per-module constructor patterns, since an iced widget is a
      function (`text`, `button`) rather than a type. The script also cannot
      see `cfg`, so the `iced_aw` widgets count as shown in a build that
      omits them.
- [ ] [1.0] The repository's PreToolUse hook refuses `panic!` in files under
      `examples/`, although the project's rule exempts examples and tests by
      path; align the hook with the rule.
- [ ] [1.0] Run each showcase once with `XDG_CONFIG_HOME` pointing at an empty
      directory before a release. Both defects the gpui self-tests found (the
      overlay layers never mounted; the mode selector dead when no theme can
      be read) sat on paths a configured desktop never takes.
- [ ] [1.0] The gpui showcase builds two gpui-base widgets outside `demo.rs` and
      `chrome.rs`, against spec §5.3 of the showcase-app spec: the body's
      `h_resizable("body")` and its three `resizable_panel()`s in `app.rs`
      (`Showcase::render`). `every_widget_reports_itself` does not see them,
      because it reads constructors as `Type::function(` (spec §10.1) and
      these are free functions. The handles report themselves
      (`demo::resize_handles`), but the `ResizablePanelGroup` and its panels
      have no Widget Info. Either build the group in a `chrome::` helper that
      reports it, or extend the gate to upstream free functions returning a
      widget, or both.
- [ ] [v0.7.0] **Regenerate the gpui screenshots and review the showcase's visible
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
- [ ] [1.0] **The iced showcase: per-instance Widget Info.** The gpui showcase now
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
      an iced twin. The egui showcase generates its info from its connector's
      manifest (`connectors/native-theme-egui/mapping.toml`, its spec's
      §10.4), which is the question this entry leaves to decide: whether an
      iced info can be generated the same way from the iced contract's rows.
      M9b T3 (2026-10-21, 9f8f89e2, b3d5a8ca): the generation question is
      answered, yes: the iced contract's colour-leaf walk is exported as
      `connectors/native-theme-iced/contract-leaves.toml` (per colour leaf the
      rows it moves, under which statuses and in which bundled combinations),
      and the showcase's Widget Info reads a listed element's colour routes
      from it; the colours the showcase draws itself keep routes by hand, each
      naming the function a test splices. The innermost-instance half stays
      open: iced's widget tree offers no hover walk (`iced_selector` sees only
      `operate` implementors), so the half an iced showcase can give is the
      hovered element's identity (M9b T7), not the innermost instance under
      the pointer.
- [ ] [v0.7.0] **The iced connector reads nothing of the model's toolbar.** The gpui
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

- [ ] [1.0] **Follow-up plan: the other sizing fields a platform leaves
      unstated.** The v0.5.9 unstated-sizes plan
      ([rationale](archive/todo_v0.5.9_unstated-sizes-and-chrome-ux-rationale.md)
      §7) made padding, `toolbar.bar_height` and `toolbar.item_gap` follow
      platform-facts, with a gate. Every other sizing field still carries
      numbers where platform-facts states **(none)**, a range, a preset's own
      choice, or no number at all. Each needs its own connector decision,
      e.g. what a builder does without a minimum height or a dialog bound,
      and `bar_height`'s change to `Option` is the pattern. The plan's audit
      (Table B, 2026-09-23) found these; the values are unchanged since, and
      were the same in each `-live` twin until M1-T4-01 removed them:

      | Field | Platform-facts cell | Stated by |
      |---|---|---|
      | `button.min_width` | macOS, Windows (none); GNOME "none" (§2.3) | macos-monterey, windows-11, adwaita 64 |
      | `button.min_height` | KDE (none), sizes to content (§2.3) | kde-breeze 32. Also Windows: platform-facts "27 (derived)" against windows-11 32 and the reader's 32, a value mismatch |
      | `input.min_height` | KDE (none) (§2.4) | kde-breeze 32 |
      | `menu.row_height` | KDE (none), sizes to font (§2.6) | done 2026-09-24: kde-breeze's 28 removed, and the gate checks the field (D3 of docs/archive/todo_v0.5.9_pre-merge-fixes.md) |
      | `menu.row_height` | Windows per context: touch 31, mouse 23 (§2.6) | done 2026-09-24: windows-11 and the reader state the mouse context's 23, and the gate checks the field (D3 of docs/archive/todo_v0.5.9_pre-merge-fixes.md) |
      | `tooltip.max_width` | KDE "(none) — preset: 300"; GNOME "(none) — preset: 360" (§2.7) | kde-breeze 300, adwaita 360 |
      | `scrollbar.groove_width` | GNOME "slider: 8 + margins", no total; macOS per context, legacy 15 / overlay 16 (§2.8, the macOS 14.8.9 probe) | adwaita 12; macos-monterey 16, the overlay context's, which it states. The macOS reader's fixed 15 (the legacy context's) went in M2a Step 8a; since M4 T24 (e6c5c737) the reader writes AppKit's `scrollerWidth(for: .regular, scrollerStyle:)` for the user's style, with `overlay_mode`. Under the legacy style the merged theme mixes styles: the reader's groove 15 and overlay off beside the preset's overlay thumb 7 and transparent track; no source gives the legacy thumb or track (see `scrollbar.thumb_width`) |
      | `scrollbar.thumb_width` | macOS per context; Windows "↕ `SM_CXVSCROLL` (same)" (§2.8) | macos-monterey 7 (the macOS reader's 7 went in M2a Step 8a); windows-11 6 against the API value. Under the legacy style the macOS theme keeps the overlay thumb 7 and the preset's transparent track beside the reader's groove 15 (M4 T24's preset-validator run): fixing it needs the legacy scroller's knob rectangle in the OS probe and a source for the legacy track colour |
      | `scrollbar.min_thumb_length` | Windows `SM_CYVTHUMB` (§2.8) | windows-11 17 |
      | `slider.tick_mark_length` | GNOME (none), no ticks (§2.9) | adwaita 4 |
      | `progress_bar.min_width` | macOS, Windows, KDE (none) (§2.10) | macos-monterey 100, windows-11 100, kde-breeze 6 (the KDE reader's comment says "Preset provides the value") |
      | `tab.min_width` | macOS (none); GNOME "none" (§2.11) | macos-monterey, adwaita 64 (windows-11 states WinUI's 100 since M2b Step 7, §2.11) |
      | `list.row_height` | KDE (none); GNOME per context, rich list 32 / plain list none (§2.15) | done 2026-09-24: kde-breeze's 28 and adwaita's 34 removed, and the gate checks the field (D3 of docs/archive/todo_v0.5.9_pre-merge-fixes.md) |
      | `splitter.divider_width` | GNOME per context, 1 / 5 (§2.17) | adwaita 1. (The macOS mismatch closed: AppKit's 9 in facts and preset since M2a Step 7, the reader's constant gone in Step 8a) |
      | `layout.widget_gap` | Windows, Material (none) (§2.20) | windows-11 6, material-3 6 |
      | `layout.container_margin` | macOS, Windows, Material (none) (§2.20) | macos-monterey 8, windows-11 6, material-3 6 |
      | `layout.window_margin` | Windows, Material (none) (§2.20) | windows-11 10, material-3 10 |
      | `layout.section_gap` | Windows, KDE, Material (none) (§2.20) | windows-11 18, kde-breeze 18, material-3 18 |
      | `dialog.min_width` / `max_width` / `min_height` / `max_height` | macOS and KDE all (none); GNOME min and max height (none), max width per context 372 / wide 600 (§2.22) | macos-monterey and kde-breeze 320/560/140/600; adwaita min height 140, max height 600 |
      | `dialog.icon_size` | Windows, KDE, GNOME (none) (§2.22) | windows-11, kde-breeze, adwaita 32 |
      | `spinner.min_diameter` | KDE, GNOME (none) (§2.23) | kde-breeze, adwaita 16 |
      | `spinner.stroke_width` | macOS, KDE, GNOME (none) (§2.23) | macos-monterey, kde-breeze, adwaita 2 |
      | `combo_box.min_height` | KDE (none); GNOME a derivation, "← button min-height (24+pad)" (§2.24) | kde-breeze 32, adwaita 34 |
      | `combo_box.min_width` | macOS, KDE, GNOME (none) (§2.24) | macos-monterey, kde-breeze, adwaita 120 |
      | `combo_box.arrow_icon_size` | macOS: a chevron measured 6 × 10, not a square (§2.24) | macos-monterey 17 |
      | `combo_box.arrow_area_width` | macOS range ~16–18; GNOME (none) (§2.24) | done 2026-09-24: both removed, and the gate checks the field (docs/archive/todo_v0.5.9_pre-merge-fixes.md) |
      | `segmented_control.segment_height` | Windows (none); KDE the tab bar as proxy (§2.25). GNOME now gives 28 (`AdwToggleGroup`, `docs/platform-facts.md:1638`, since 0c7bde37), adwaita's value | windows-11 28, adwaita 28, kde-breeze 30 |
      | `segmented_control.separator_width` | Windows, GNOME (none) (§2.25) | windows-11 1, adwaita 1 |
      | `expander.header_height` | macOS, KDE (none) (§2.27) | macos-monterey 40, kde-breeze 40 |
      | `defaults.icon_sizes.small` | macOS range, "sidebar: 16–20pt" (§2.1.8) | macos-monterey 16 |
      | `defaults.icon_sizes.large` | macOS (none) (§2.1.8) | macos-monterey 32 |
      | `defaults.icon_sizes.dialog` | macOS, Windows (none); GNOME "(none) — 48 (GTK3 legacy)" (§2.1.8) | macos-monterey, windows-11, adwaita 22 |
      | `defaults.icon_sizes.panel` | macOS, Windows, GNOME (none) (§2.1.8) | macos-monterey, windows-11, adwaita 20 |
      | `defaults.icon_sizes.toolbar` | macOS per context, "32pt (reg) / 24 (sm)" (§2.1.8) | macos-monterey 24, the small mode rather than the regular default |

      `macos-tahoe` (M2a Step 9) carries `macos-monterey`'s number, marked
      "no macOS 26 source" in the file, in these macOS rows above:
      `button.min_width` 64, `scrollbar.thumb_width` 7,
      `progress_bar.min_width` 100, `tab.min_width` 64, the dialog bounds
      320/560/140/600, `spinner.stroke_width` 2, `combo_box.min_width` 120,
      `combo_box.arrow_icon_size` 17 (the chevron measures 7 × 10.5 on macOS
      26, `docs/platform-facts.md:1619`, not a square either),
      `expander.header_height` 40 and the icon sizes small 16, large 32,
      dialog 22 and panel 20; it makes
      `macos-monterey`'s choice of the small toolbar mode, 24 (the cell's
      macOS 26 tag: "same"). It does not carry `scrollbar.groove_width`:
      macOS 26 reports `scrollerWidth` 17 in both modes
      (`docs/platform-facts.md:1307`), which `macos-tahoe` states; and it
      leaves `layout.container_margin` unstated. It carries the same way,
      wherever else the model requires a number that no source gives for
      macOS 26: `focus_ring_offset` -1, `border.corner_radius_lg` 10 (Tahoe's
      window radii are larger per third-party measurements,
      `docs/platform-facts.md:177`; not adopted, no API gives them),
      `border.line_width` 0.5, `border.shadow_enabled`,
      `button.icon_text_gap` and `menu.icon_text_gap` 4, `tooltip.max_width`
      300, `scrollbar.min_thumb_length` 40, `tab.min_height` 24,
      `segmented_control.separator_width` 1, `expander.arrow_icon_size` 13,
      `dialog.button_gap` 12 and `icon_size` 64, and the link's underline and
      transparent fills. It carries the same way, marked, six values the
      model would otherwise inherit from a safety net known to be the wrong
      kind: `scrollbar.track_color` (transparent), `menu.icon_size` 16, the
      expander's `shadow_enabled` (false), the tooltip's colours,
      `sidebar.background_color` and `button.hover_background` (the inherited
      opaque window colour would hide the translucent button on hover); and
      two that would fall back to a value of another kind: `dialog.title_font`
      (13 pt Bold, against the inherited regular weight) and
      `segmented_control.active_background` (against the inherited accent)
      (`docs/todo_code-review-findings.md`, M2a Steps 8-12).

      The Icons page's Icon Sizes section already marks the unsourced icon
      sizes (panel 20, dialog 22, macOS large 32). Outside sizing, KDE's
      `defaults.border.corner_radius_lg` is "(none) — preset" (§2.1.6) while
      kde-breeze states 8. Two value mismatches the audit noticed and did
      not follow: adwaita's `checkbox.indicator_width` 20 against §2.5's
      "libadwaita CSS: 14" (Ch. 1 gives 20 with padding:
      `docs/platform-facts.md:1233` gives GNOME 14, the CSS `min-width`,
      which is the content box, while `adwaita.toml` states the 20-point box
      that `:980` defines, 14 + 2 · `padding: 3px`, so the table cell should
      read 20; it does since 963bcdd2, which closes this mismatch), and windows-11's
      menu `row_height` 36, which no platform-facts context gave (it states the
      mouse context's 23 since 2026-09-24: the `menu.row_height` row above). Also for
      this plan:
      - **KDE input vertical padding** (the audit's N4): platform-facts'
        §2.4 cell is "3 (measured)", and the gate keeps 3/6/3/6. Breeze's
        `lineEditSizeFromContents` expands by `LineEdit_FrameWidth` = 6 on
        both axes, and QLineEdit's own margins were not read.
      - **iOS.** Platform-facts has no iOS column, so no iOS size is
        sourced: `ios-13.toml`'s `bar_height_px = 44.0` is unsourced, and so is
        every other size in it (the 44 `min_height` of button, input and
        combo box, the 44 `row_height` of menu and list, tooltip
        `max_width` 300, the dialog bounds 270/560/140/600, the layout
        8/8/20/20). The gate does not cover iOS.
      M9a T6 fix round 2 (2026-10-18, 8fbcf9ae): windows-11 states no menu
      `row_height` any more (`windows-11.toml` `[light.menu]`, `[dark.menu]`):
      §2.6's Windows cell is ❓, WinUI 3's `MenuFlyoutItem` being its padding
      round a Segoe UI Variable line box no source states.
- [ ] [1.0] **The colour-scheme presets' sizes have no source.** The eleven
      colour-scheme presets (the four catppuccin, dracula, gruvbox,
      material, nord, one-dark, solarized, tokyo-night) carry sizes that
      were copied rather than sourced: their toolbar's 40 was the height
      the removed generic `default.toml` carried, and material's header
      cites Material 3 for its colours only. v0.5.9 removed only their
      `bar_height_px` (40, material 64). Decide, field by field, what a
      colour-scheme preset's size is: a value with a source, or absent, so
      that the toolkit's default or the application's stands.
- [ ] [1.0] **A machine-readable platform-facts.** The documented-sizes gate
      (`native-theme/src/presets/documented_sizes.rs`) copies each value it
      checks by hand from `docs/platform-facts.md` and cites the line; a
      companion test checks the citation lands on the right row, but not
      that the number is the one written there. With the facts in a
      structured form (per platform, per field, the value, its context and
      its source), every native value — not only padding and the toolbar —
      could be gated, and the Markdown tables generated from it.
      *OS-version axis (2026-10-11, M1-T5-01 in
      `docs/todo_code-review-findings.md`):* a fact is keyed by platform and
      OS version, since a platform's next generation (`macos-tahoe` beside
      `macos-monterey`, two Material presets) differs from the last by version,
      not by cell. Until then a version-tagged cell gives each value to its
      version's preset (platform-facts.md, Conventions) and the gate keys its
      rows by preset; the ruling settles the conflict between a Tahoe value
      in Sonoma's cell and the one-number cell rule. The gate's hand tables
      hold 95 padding rows (19 widgets × 5 presets, 90 of them `row()`
      calls) and 90 field rows; on 2026-10-11, after the `-live` presets
      went, 602 `platform-facts.md:N` citations sit outside `docs/archive/`
      (the review counted 776). The migration (a schema for the seven kinds
      of cell, section by section as M2 verifies them) gets its own
      rationale, spec and plan after M2.
- [ ] [1.0] **WinUI's combobox arrow column: a model extension.** Per-side padding
      is done (v0.5.9). WinUI measures its combobox's right padding, 0, to
      a separate 38px arrow column (platform-facts §2.24), a structure the
      model has no field for, so windows-11 states WinUI's right padding, 0
      (`native-theme/src/presets/windows-11.toml`
      `[light.combo_box.border] padding_right_px`,
      `[dark.combo_box.border] padding_right_px`), and no field
      carries the column. An arrow-column field (or a
      right padding that includes it) would let a connector with such a
      column draw it.
- [ ] [1.0] **Loose ends of the v0.5.9 unstated-sizes plan** (from its reviews):
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

- [ ] [upstream] PR: add `Window::screenshot()` API to gpui — gpui has no public way to
      capture the rendered framebuffer. The underlying blade-graphics backend
      has `copy_texture_to_buffer()` but gpui doesn't expose it. A public
      `screenshot()` method would enable headless CI screenshot capture on all
      platforms (like iced's `--screenshot` flag). Without this, the gpui
      showcase captures itself through OS tools on macOS (`screencapture -l`)
      and Windows (`BitBlt` on its own window), which the screenshots
      workflow runs, and Linux screenshots need external spectacle capture.
      *GPUI Kit 0.8.0 (2026-10-22, G3 T10):* at gpui-pre 0.3.9 and gpui-fast 0.1.10 a window
      renders to an image only through the test-support `render_to_image`
      (gpui-pre `src/window.rs:2871`, gpui-fast `src/window.rs:2915`, both
      `#[cfg(any(test, feature = "test-support"))]`). The showcases capture
      through `tools/showcase-support`'s `capture::FrameCapture` on macOS and
      Windows and through `scripts/capture_window.sh` (spectacle) on Linux,
      so the blade-graphics and `BitBlt` wording above is history. The ask
      stands (handed to G3 by M9b T2).
- [ ] [upstream] PR: let an application drop the SVGs a window has drawn, so the icons
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

- [ ] [upstream] Audit gpui-component widgets for hardcoded values that should be
      theme tokens (padding, icon sizes, corner radii, spacing)
- [ ] [upstream] PR: expose per-widget padding/margin as theme-configurable
- [ ] [upstream] PR: expose checkbox/radio indicator size as theme token
      M8 T21 fix round 1 (2026-10-17), the duplicate rule: the lines of the entry
      "inner geometry exposed" (*Upstream PR candidates from v0.5.8 (Tier U)*;
      5ec4f175), which holds this one whole:
      inner geometry exposed: checkbox/radio indicator, switch track and thumb,
      slider track and thumb, separator thickness, resize-handle width, button
      icon gap, popup-menu items, select arrow, accordion arrow
- [ ] [upstream] PR: expose scrollbar dimensions as theme-configurable
- [ ] [upstream] PR: expose button min-height and icon spacing as theme tokens
      M8 T21 fix round 1 (2026-10-17): the button icon gap is also in "PR:
      expose checkbox/radio indicator size as theme token" above, the
      inner-geometry list merged into it.
- [ ] [upstream] PR: let a scroll container keep the wheel to itself. gpui dispatches
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
      G3 T5 (2026-10-22): gpui-component 0.8.0 wraps each of its scrolling
      widgets in gpui-base's `ScrollBounce` (`src/scroll/scrollable.rs:188`,
      `src/list/list.rs:603`, `src/tree.rs:127`, `src/table/state.rs:2727`,
      `src/diff/mod.rs:557`, `src/message_scroller.rs:383`,
      `src/setting/page.rs:233`, `src/sidebar/mod.rs:455`,
      `src/command/state.rs:1033`), which keeps the wheel: in the capture
      phase the innermost area under the pointer that can scroll claims it
      and stops its propagation (gpui-base 0.8.0 `src/scroll_bounce.rs:248-270`,
      `:556-610`); gpui-base's editor claims it in the capture phase too, and
      scrolls and stops it in the bubble phase, before the page's listener
      (`src/input/base/element.rs:478-495`, `src/input/base/state.rs:2706-2776`).
      So none of the gpui showcase's eight scroll `.occlude()`s remains (the
      DataTable, List, Tree, MessageScroller, Scrollable Area, Settings frame,
      code Editor and Diff), each removed alone behind
      `a_nested_scroller_keeps_the_wheel_to_itself` on both cores. The
      Settings frame was GPUI's own `overflow_y_scroll` div, but the Settings
      fills it, so the frame never scrolled (its dead `overflow_y_scroll` went
      in G3 T6); what does is the Settings' page
      list, wrapped at `setting/page.rs:233`. The occlusions covered whole demo
      boxes, so the page now also scrolls over a box's parts that do not
      scroll (its padding and border, the Settings' heading, sidebar search
      field and resize handle) and over an inner area whose content fits, as
      native toolkits do. Still open: scroll chaining (at its end the inner
      area keeps the wheel, and stretches where bounce is on, on macOS, iOS
      and Android, gpui-base `src/scroll_bounce.rs:170-178`, instead of
      handing over, upstream's test
      `the_innermost_area_takes_the_whole_gesture`, `src/scroll_bounce.rs:970`),
      and GPUI's own `overflow_*_scroll` divs, whose listener still moves
      every scroller under the pointer, so a nested one still needs
      `.occlude()` (the test failed, on both cores, when tried on 2026-10-22
      with the Scrollable Area's viewport made such a div and not occluded).
- [ ] [upstream] Additional PRs as gaps are discovered during connector implementation
- [ ] [upstream] Report: a single-select `Combobox` misses a change after a search.
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
- [ ] [upstream] PR: let a consumer supply the icons gpui-component's widgets build for
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
      Added since: `SpeechButton`'s Mic / Square (`src/speech/button.rs:96-100`,
      0.7.1).
      *GPUI Kit 0.7.3 (2026-10-14):* `Diff` adds its own: ArrowDown, ArrowUp
      and ChevronsUpDown or ChevronDown on a folded hunk's expand buttons
      (`src/diff/mod.rs:993`, `:1000`, `:1006-1010`), ChevronRight or
      ChevronDown on a file's collapse button (`:1194-1198`), and Plus on the
      add-annotation button (`:1546`).
      *GPUI Kit 0.8.0 (2026-10-22, G3 T10):* `Diff`'s icons are at `src/diff/mod.rs:998`,
      `:1005`, `:1011-1015` (the folded hunk's buttons), `:1199-1203` (the
      file's collapse button) and `:1551` (Plus), the same icons.
- [ ] [1.0] Left open by the v0.5.9 showcase-layout work
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
- [ ] [1.0] The gpui showcase's Retina nudge
      (`native_theme_showcase_support::capture::nudge_content_size`, which
      `examples/showcase-gpui/main.rs` calls when the window opens and before
      `--screenshot`'s capture on macOS): two 1 px content-size changes, kept
      because gpui-pre once set its Metal drawable at the logical size on a
      Retina display, which only a real resize corrected. Whether gpui-pre
      0.3.9 still needs it cannot be read from Linux. At the next asset run,
      take a macOS capture without it (the frame check fails if it is still
      needed); drop it if the capture passes.
- [ ] [upstream] PR: Button: let the caller's text size reach the label —
      `button_text_size` on the content element (`button/button.rs:749`)
      shadows a size set on the button; honouring the instance style's text
      size there, or a public `content_style`, would retire the child-label
      pattern of `geometry::button_label`.
- [ ] [upstream] PR: ColorSelect: apply the caller's refinement to the framed field
      (`color_picker.rs:488` lands on the wrapper; the field is
      `render_field`, `:762-813`).
- [ ] [upstream] PR: DataTable: `Styled`, or a text-size setter, so the cell size need
      not go through `Size::Size`.
- [ ] [upstream] a public way to mark an element's accessibility node disabled:
      gpui-base's `A11yDisabledExt` public (it is `pub(crate)`, gpui-base 0.8.0
      `a11y_disabled.rs:28`), or a GPUI `aria_disabled`, which neither gpui-pre 0.3.9
      nor gpui-fast 0.1.10 has. Until then `widgets::Slider` marks its themed root's
      node disabled through a crate-private copy of gpui-base's wrapper
      (`A11yDisabled`, native-theme-gpui `src/widgets/mod.rs`), which can go then
      (G3 T7, 2026-10-22).

### gpui: a tab scrolled into view loses its outer flare

- [ ] [1.0] Found 2026-10-22 in the G3 Task 12 review. Where the theme states `tab.active_outward_corner_radius`, `widgets::TabBar` pads its scrolling row by the radius at both ends, so the end tabs' flares show (`connectors/native-theme-gpui/src/widgets/tab_bar.rs`, `RenderOnce for TabBar`). But a click on a tab the row's edge cuts asks `ScrollHandle::scroll_to_item`, which aligns the tab's own box with the row's edge: gpui-pre 0.3.9 `src/elements/div.rs:4373-4383`, gpui-fast 0.1.10 `:4426-4435`, `scroll_to_active_item`. On an overflowing row, a selected tab revealed that way, the last one included, then shows with its outer flare cut off by the row's clip. Upstream's Folder pads its row the same way (gpui-component 0.8.0 `tab/tab_bar.rs:686-687`) and reveals through the same call. To fix: have the click's reveal keep the radius free beyond the tab, for example by setting the handle's offset from the tab's bounds widened by the radius instead of calling `scroll_to_item`.

### Connectors: a public theme builder that takes the mode

- [ ] [1.0] gpui's `to_theme(&resolved, name, is_dark: bool, ..)` could take a `ColorMode`, the type `Loaded::mode` carries and iced's `to_theme_in` takes (M8 T4 ruling, 2026-10-16): a break outside the maintainer's four M8 answers, which buys nothing while `Loaded` carries the mode; fold it into the 1.0 API pass.

---

## This cycle (v0.7.0)

The `[v0.7.0]` items that sat under *Deferred (pre-1.0)*, in the order they had there
(M8 T21, 2026-10-17).

### iced: dialog is unmapped
- [ ] [v0.7.0] The iced connector maps no `DialogTheme` field, and its contract lists none as unreachable: iced 0.14 has no dialog widget. `iced_aw`'s `Card` could carry the maxima, `Card::max_height` and `Card::max_width` taking an `f32` (`iced_aw-0.14.1/src/widget/card.rs:144`, `:151`); the minima, the button gap and order, the fonts and the border would still have no receiver of their own. Decide whether a `Card` is the dialog's receiver.
  Named in the iced colour-leaf coverage test (M5 T12b, 9c46d834).
  G2 Task 13 (2026-10-14): the new `dialog.entrance` has no receiver either: iced 0.14 and `iced_aw` 0.14.1 have no dialog widget, so nothing animates a dialog's entrance; the crate docs' *Theme Field Coverage* (`connectors/native-theme-iced/src/lib.rs`) names it among the values iced has no receiver for.

### iced: an application style that reads `window.background_color`
- [ ] [v0.7.0] iced paints an application's background from its base style, `theme::Style.background_color` (`iced_core-0.14.0/src/theme.rs:222-230`), which `Base::base` takes from the palette's background (`theme.rs:291-293`, `:328-336`), under this connector `defaults.background_color`; nothing reads `window.background_color`. It inherits `defaults.background_color` (`docs/inheritance-rules.toml:157`) and equals it in all 34 bundled combinations, so nothing differs today (M5 T12a ruling C4). Add a style for `application(..).style(..)` (`iced-0.14.0/src/application.rs:394-399`), a `styles::application(resolved)` whose background is `window.background_color` and whose text colour is the base style's, with its contract row. The colour-leaf coverage test names the leaf for this entry (`connectors/native-theme-iced/src/contract/derived.rs:1353-1365`).

### GNOME reader: title and heading sizes ignore the user's font
- [ ] [v0.7.0] `adwaita.toml`, which the GNOME reader merges over, states fixed sizes for the text scale (`caption` 9pt, `dialog_title` 15pt, `display` 20pt; `[light.text_scale]`, `[dark.text_scale]`) and for `dialog.title_font` (15pt; `[light.dialog.title_font] size_pt`, `[dark.dialog.title_font] size_pt`). libadwaita sizes them as a percentage of the base font (`.caption` 82%, `.title-2` 136%, `.title-1` 181%; `docs/platform-facts.md:746-755`), so these are its sizes at the default 11pt only, and live mode ignores a user who sets another font. The GNOME reader (`native-theme/src/gnome/mod.rs`) reads the font but sets none of them; it should derive them from the font it reads, as the KDE reader derives its headings from the body font (`native-theme/src/kde/fonts.rs`, `HEADING_LEVEL_1_FACTOR` and `HEADING_LEVEL_2_FACTOR`). The preset half is done: the `-live` preset is removed (M1-T4-01, 2026-10-11).

### iced showcase: Widget Info claims a substituted family
- [ ] [v0.7.0] For M9b: the iced and gpui showcases name a family the system lacks by its stated name alone while it is drawn in the platform's substitute. iced: since 2fcc26fb the family the showcase asks for (`resolved_family`, `connectors/native-theme-iced/examples/showcase-iced.rs:12466-12477` at 2fcc26fb; since d91ff927 `font_from_database` in that file, through `native_theme_iced::font`) is `system_font_family`'s, which is the platform's substitute where the system lacks the stated family; `family_label` (`:12376-12386` at 2fcc26fb) returned the stated name alone wherever the drawn family is the one asked for, so `font_row_drawn` says "button.font, family, size and weight" for windows-11's "Segoe UI" drawn in fontconfig's match (Adwaita Sans on one Linux system); before 2fcc26fb the same row said "Segoe UI (not found; drawn in generic sans-serif: …)". gpui: the inspector's Theme tab (`connectors/native-theme-gpui/examples/showcase-gpui/inspector.rs:215-229`) shows the stated `font.family` under `font_family` and notes "drawn as" only for macOS's `.SystemUIFont`, while `Theme::font_family` holds `font_family`'s substitute (`connectors/native-theme-gpui/src/lib.rs:311`, `:747-750`), and its `mono_font_family` row has no note at all. Widget Info and the inspector must name the drawn family in both, e.g. "Segoe UI (not on this system; drawn in the platform's substitute, Adwaita Sans)". egui names it, in another place: its Theme panel's Notes section lists the atlas's `Note::FontFamilyUnavailable { family, substitute }` (`connectors/native-theme-egui/examples/showcase-egui/info.rs:1114-1121`), while its Fonts section just below (`:1129-1135`) prints the stated `font_family` and `mono_font_family` with no note; M9b makes the two sections agree.
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

### gpui: the base-palette repair writes colours without their tokens
- [ ] [v0.7.0] Latent: the observer's base-palette repair (`connectors/native-theme-gpui/src/lib.rs` `repair_base_palette`, `:1370-1387`) writes the 12 base colours (`red` … `cyan_light`) onto `ThemeColor` without their `tokens`. Harmless while no widget reads `tokens.red` … `tokens.cyan_light`; check at every gpui-component bump.
      *GPUI Kit 0.8.0 (2026-10-22, G3 T10):* still harmless: no gpui-component or gpui-base
      0.8.0 source reads `tokens.red` … `tokens.cyan_light` outside `theme/`
      (`grep -rnE 'tokens\.(red|green|blue|yellow|magenta|cyan)'` over both
      crates' `src` finds none).

### gpui: the theme-drawn widgets' focus ring
- [ ] [v0.7.0] The connector's `Checkbox`, `Radio`, `Slider` and `Switch` draw the focus ring through gpui-component's `focus_ring_style`, upstream's 3px at half the ring colour's alpha (`styled.rs:11-12`); the model states `defaults.focus_ring_width` and `focus_ring_offset`, which reach none of them.

### check_widget_coverage.py discovers no chart
- [ ] [v0.7.0] gpui-component's charts derive `IntoPlot`, not `IntoElement`, so `scripts/check_widget_coverage.py` discovers none of them. Teach it `IntoPlot`, then show what it finds — today `RadarChart` and `SankeyChart`, which no showcase shows.

### Presets: material-3's platform-facts citations have drifted
- [ ] [v0.7.0] For M2d (found 2026-10-14 in G2 Task 10's preset audit). About 60 comment lines in each variant of `native-theme/src/presets/material-3.toml` cite a `docs/platform-facts.md:N` line 1–4 lines away from the row they describe: for example, the tooltip's `font.color` cites `:1280` (`font.weight`) where the row is `:1282`, and the list's `header_font` cites `:1425-1426` (the selection rows) where the rows are `:1427-1428`. G2 Task 10 fixed the four pairs beside its new fields (`a440e812`). The values are not affected; only the line numbers are. The other native presets have no such drift: their remaining mismatches are deliberate cross-row citations, such as a disabled colour citing `disabled_opacity` or an unchecked background citing the widget's `background_color`. Method: for each `# platform-facts.md:N` comment, compare the property name of row N (and of each row in a range) with the key on the next line, as `<sub-table>.<key>` without `_px`/`_ms`/`_pt`. M2d re-cites material-3 against the Compose release it pins anyway; a gate doing this check in `check_release.sh` would stop the drift from coming back.
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

### GPUI Kit 0.7.3 (G1, G2): review minors left for M9a, M9b and M9c
Found by the plan's task reviews (2026-10-14) and deferred there; each was checked again at
the plan's close.
M8 T21 (2026-10-17): named for its open items, all the showcases' (it was *GPUI Kit
0.7.3 (G1, G2): review minors left for M6 and M7*); its closed items are under that
name in [`archive/todo_closed.md`](archive/todo_closed.md).
- [ ] [v0.7.0] For M7: the egui showcase's body rows grow past `list.row_height` where an
      auto-sized column wraps (the Data and Theme Map tables; egui_extras `layout.rs:165`);
      `catppuccin-mocha`'s header label (19 px) is taller than its stated header height
      (18 px), pre-existing.
      M8 T21 (2026-10-17): for M9c, not M7: M7's Step 6 ruling sent it to
      M9a as the showcases' milestone; M9's plan gives each showcase its
      own, M9a gpui, M9b iced and M9c egui.

### Showcases: the strict panic lints do not reach the showcase examples

- [ ] [v0.7.0] `scripts/check_release.sh` and `.github/workflows/ci.yml` run
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

### M7 close (2026-10-16), its `[v0.7.0]` entries
The egui connector's and egui widgets' fix phase (M7, `c5ff9271..`, ledger "## M7") left these. Each sits here until its milestone takes it.
- [ ] [v0.7.0] For M9a: the egui showcase's `named_image` reads its freedesktop icon from disk in every pass (`role_image` is cached since M7 T12).
      M8 T21 (2026-10-17): `named_image`: M9c; the parity items are the G1/G2 section's (M9a), not repeated here.

---

## Post-1.0

### Mobile readers
- [ ] [post-1.0] iOS: `from_ios()` via `objc2-ui-kit`
- [ ] [post-1.0] Android: `from_android()` via `jni` + `ndk`, Material You (API 31+)

### Windows reader
- [ ] [post-1.0] **A live Windows hover time for the tooltip delay.**
      (From the Toolkit Connectors entry *A sidebar width and a tooltip delay are
      missing from the model*, whose tooltip half G2 closed.)
      `windows-11` states 800 ms, 2 × the `SPI_GETMOUSEHOVERTIME` the OS probe
      read on its Windows images (400), so a user who changes the hover time
      changes WinUI's tooltip delay and not the theme's. The Windows reader
      could read `SPI_GETMOUSEHOVERTIME` and state twice it.
      That is new `unsafe` FFI, with the `SAFETY:` comment
      `native-theme/tests/unsafe_safety_comments.rs` enforces, so it needs the
      maintainer's approval; and a watched source, since the watcher does not
      receive the `WM_SETTINGCHANGE` with `SPI_SETMOUSEHOVERTIME` that reports
      a change (`native-theme/src/windows.rs`, *Sources not watched*).

---

## Deferred (pre-1.0)

### Font DPI
- [ ] [1.0] Unverified: do the Linux readers' `font_dpi` sources (`Xft.dpi`, KDE `forceFontDPI`) have the same logical-versus-physical question as Windows under Wayland fractional scaling? The Windows reader now reads its fonts at 96 DPI and reports a `font_dpi` of 96, because the model's sizes are logical pixels (`native-theme/src/windows.rs`, `LOGICAL_DPI`). If a Linux session at, say, 150 % sets `Xft.dpi` to 144 while the toolkit also applies the 1.5 scale factor, a 10pt font would resolve to device pixels and be scaled again. Not checked on a KDE or GNOME Wayland session. M2e (2026-10-11) answered it from source. GNOME: gnome-settings-daemon 51.0 sets `Xft.dpi` to 96 × `text-scaling-factor` × the X11 UI scale (`plugins/xsettings/gsd-xsettings-manager.c:583-595`, `:665-668`, `:797-798`), so it carried the text scale and the display scale; since M2e Step 7 the GNOME reader and `ResolutionContext::from_system()` on GNOME use 96, which applies the factor once on Wayland; X11 and Xwayland clients also get it through the toolkit's `Xft.dpi` scale factor (`docs/archive/todo_closed.md`, *Closed items from Font DPI*, "For M4 (connectors): on GNOME and Budgie X11"; its Xwayland half is the next entry). KDE: Plasma 6.7.5's krdb sets `Xft.dpi` on Wayland to `kwinrc [Xwayland] Scale` × 96 (`kcms/krdb/krdb.cpp:333-337`), and Plasma 6.8 always does (`startkde/plasma-setup-xwayland/main.cpp:47-58` at v6.7.91), so the KDE reader's `Xft.dpi` fallback on Wayland is the display scale: open, for M4, with Plasma 6.8's removal of `forceFontDPI`, which the KDE reader and `detect.rs` read first (ledger M2e-01 to M2e-03).
  M4 (2026-10-12/13): one font-DPI chain in `detect`, cached in `DetectionContext` and shared by the KDE reader (9ed0ca92); `forceFontDPI` is read on X11 only (the plan's ruling) and from `kcmfonts`, the file Plasma opens (49e0067a, b42b1568), so on a Wayland session the chain starts at `Xft.dpi`. The Wayland question above is still unverified; re-check at Plasma 6.8.0 with M2e-01.
- [ ] [upstream] An X11 client through Xwayland on a GNOME or Budgie Wayland session draws the text-scaling factor twice: it gets the `Xft.dpi` gnome-settings-daemon writes as 96 × `text-scaling-factor` × the window scale (gnome-settings-daemon 51.0 `plugins/xsettings/gsd-xsettings-manager.c:666-668`, `:742`, `:797-798`), winit 0.30.13 and gpui-pre-linux 0.3.9 take their X11 scale factor from it (`src/platform_impl/linux/x11/util/randr.rs:41-53`, `:145-146`; `src/linux/x11/client.rs:2671-2678`), and the GNOME reader, which sees a `wayland` session, reports the factor, which the connectors apply again. Core cannot see a client's backend, and the connectors' entry points take no window (M8 Step 6 ruling, 2026-10-16). The lengths winit and gpui scale by the factor on X11, which GTK does not, are theirs, not ours. The same holds for an X11 session started with `startx` from a text login: it runs in the text login's session, whose `XDG_SESSION_TYPE` is `tty` (one of pam_systemd's session types, `pam_systemd(8)` `type=`, systemd 262), so the reader reports the factor and the X11 toolkit's `Xft.dpi` scale carries it too. The rule stays `x11` exactly (Step 6 ruling); `DISPLAY` or `WAYLAND_DISPLAY` would guess the backend, not know it. Waits on a way for the reader to learn the backend the client actually uses (Xwayland, `startx`), which no toolkit exposes to a theme reader today.

### docs.rs feature badges
- [ ] [1.0] docs.rs `doc(cfg)` badges: mark each feature-gated item on docs.rs with the feature it needs. It needs nightly's `doc_cfg` (`#![cfg_attr(docsrs, feature(doc_cfg))]` and `--cfg docsrs` in `[package.metadata.docs.rs]`); until then the item's rustdoc names the feature in prose (the v0.5.9 second pre-merge review's Task 6).

### `system_icon_set()` decides where it should report
- [ ] [1.0] `system_icon_set()` returns `IconSet::Material` on platforms other than macOS, iOS, Windows and Linux, documented as a "safe cross-platform fallback" (`system_icon_set` and its doc, `native-theme/src/detect/icon_theme.rs:12`, `:23`): the icon-set counterpart of the icon-theme fallback removed in v0.5.9. By the rule that the library reports and the application decides, it should report there (e.g. `Result<IconSet>` with `Error::PlatformUnsupported`, or `Option<IconSet>`). It decides wrongly where it matters: the BSDs run the same freedesktop desktops as Linux (KDE Plasma, GNOME, XFCE) with freedesktop icon themes, and get the bundled Material set. Ripple: `Resolved::icon_set` and `SystemTheme::icon_set` are required `IconSet` fields filled from it when the theme states none (`Theme::resolve`, `pipeline::run_pipeline`), and `IconSetChoice::effective_icon_set` returns it for `IconSetChoice::System`; each would need a way to carry "none".

### iced: filing variable-font weights is a workaround for cosmic-text 0.15
- [ ] [upstream] iced 0.14 draws text with cosmic-text 0.15, which matches a face only at the weight fontdb filed it at (`font/fallback/mod.rs:279-287`, `:299-303`, `:446-456`), and fontdb 0.23 files a variable font at its OS/2 weight alone (`lib.rs:1034-1037`, `:1161`). The iced showcase therefore files an extra face at each weight a variable face's `wght` axis covers (`register_weight` in `connectors/native-theme-iced/examples/showcase-iced.rs`), and the iced connector's README and *Font Configuration* docs teach consumers the same. cosmic-text 0.19 matches a variable face at any weight its axis covers itself (`variable_weight_match`, `font/system.rs:38-44`, used at `font/fallback/mod.rs:285`, `:301`). When iced moves to cosmic-text 0.19 or later, remove `register_weight` and the recipe, and check whether the nearest-face fallback in `drawable_font` is still needed for families with no face at a weight.

### iced showcase: the macOS system menu goes in after the window opens
- [ ] [v0.7.x] On macOS the iced showcase puts its menus into the system menu bar from `update`, on a message `boot`'s task sends (`system_menu` and `boot` in `connectors/native-theme-iced/examples/showcase-iced.rs`, M9b T17), which an `iced::application` runs once its window is open (`iced_winit-0.14.0/src/lib.rs:108-113`), so possibly after the window's first frame, and winit's default app menu can stand for that frame. Not in `boot`: iced 0.14 builds winit's event loop with winit's default menu on and offers no way to turn it off (`iced_winit-0.14.0/src/lib.rs:79-81`; the egui showcase turns it off through eframe's `event_loop_builder`, `with_default_menu(false)`), and winit installs that menu when the application has finished launching (`winit-0.30.13/src/platform_impl/macos/app_state.rs:139-143`), inside the event loop's run (`iced_winit-0.14.0/src/lib.rs:451`), which `boot` comes before (`:105`), so it would replace a menu `boot` installed. Proposal: run the showcase as an `iced::daemon`. A daemon opens no window of its own (`iced-0.14.0/src/daemon.rs:76-78`), so its boot task runs at once (`iced_winit-0.14.0/src/lib.rs:108-119`); an `update` on its message installs the menus and then opens the window (`window::open` with `main`'s and `capture_window_settings`' settings), `view`, `title` and `theme` take the `window::Id`, and the run exits on the window's close. The menus are then in before any window or frame exists, provided winit's launch handling, which installs its default menu, has run by the time `update` handles the boot task's message: that order is assumed here, not read from the sources, so when the entry is picked up, check that winit's launch handling cannot replace a daemon's menus. Not taken in M9b (controller's ruling, M9b T17 fix round 1): a daemon opens and closes its window itself, which changes the start, the exit and the window settings on every platform, for one frame of winit's menu on macOS that no capture shows.

### macOS: the stated font family "SF Pro" is not a family iced's font database holds
- [ ] [1.0] In the v0.5.9 CI screenshot of the iced showcase (`connectors/native-theme-iced/docs/assets/macos-macos-sonoma-light.png`, captured at `33fea0f7`) the inspector reads "Font: SF Pro (not found; drawn in generic sans-serif: .SF NS)": the family macos-sonoma states (`presets/macos-monterey.toml`, renamed from `macos-sonoma.toml` in 11017186, `[light.defaults.font] family` and `[dark.defaults.font] family`, and the reader's non-macOS testable build, the test fixtures `sample_widget_fonts`, `sample_light_defaults` and `sample_dark_defaults`, `macos.rs:590-661`) is not a family name in fontdb on `macos-latest`, which holds the system UI font as `.SF NS`, so iced reaches it only through cosmic-text's fallback list (`cosmic-text-0.15.0` `font/fallback/macos.rs:30-38`). "SF Pro" is the name `docs/platform-facts.md` §1 (`:63-71`) gives every system font, Apple's name for the typeface, not the name the font file declares. The live reader states `NSFont.familyName()` (`fontspec_from_nsfont`, `macos.rs:297-298`), which on a real Mac is presumably `.AppleSystemUIFont`; unverified, and whether iced (fontdb) or gpui (Core Text) resolve that name is not checked either. To decide: what family name a toolkit resolves the macOS system font by (fontdb family names on `macos-latest`, Core Text's name for `systemFontOfSize:`), whether the preset should state that name, keep "SF Pro" as the documented name with the connectors mapping it, or state none so the toolkit's system default stands; check it on the macOS CI runner rather than by assumption. gpui draws through Core Text, which may resolve "SF Pro" differently: check both connectors.

### gpui: Settings and dock splitters draw upstream's renderer
- [ ] [upstream] gpui-component 0.7.0's `Settings` divider and the dock's edges install gpui-component's own resize-handle renderer internally (`setting/settings.rs:416`, `dock/dock.rs:143`), so the splitter colours `base_layer::resizable_theme` writes do not reach them. Ask upstream for a handle-appearance setter on `Settings` (and the dock), then route the splitter colours through it.
      *GPUI Kit 0.8.0 (2026-10-22, G3 T10):* still so. `Settings` builds its divider with
      gpui-component's `h_resizable` (`setting/settings.rs:416`), which installs
      `resize_handle_appearance` (`resizable.rs:30`); the dock's edge handle is
      gpui-component's `resize_handle` (`dock/dock.rs:144`, `resizable.rs:46`)
      and its splits draw `render_resize_handle` (`dock/dock.rs:58`).

### Popover arrow
- [ ] [1.0] Record per platform whether a popover draws an arrow (`docs/platform-facts.md`, sources first), then model it.

### Showcases: the toolbar's "Reload System Theme" glyph
- [ ] [1.0] All three showcases draw the reload button with rotate-cw; gpui-component 0.7.0 added `IconName::RefreshCw`, the glyph the icon roles map `ActionRefresh` to. Use `refresh-cw` / `refresh` / `view-refresh` (Lucide / Material / freedesktop) in all three.

### gpui: a citation gate for the connector's doc comments
- [ ] [1.0] The showcase's citations are checked by gates (`every_colour_claim_is_read_at_the_line_it_cites`, `every_prose_citation_still_exists`); the connector's own `src/*.rs` doc comments have none, so the 0.7.0 bump re-read about 200 of them by hand. Add a gate that resolves their `file.rs:N` citations against the pinned upstream sources.
      M8 T21 fix round 1 (2026-10-17), the duplicate rule: the lines of the entry "A
      mechanical check of the library's prose `file:line` citations"
      (*native-theme-gpui connector*, *Research*; fb7f7ace):
      A mechanical check of the library's prose `file:line` citations against
      the locked registry sources (the showcase's colour claims have one,
      `every_colour_claim_is_read_at_the_line_it_cites`); the 0.7.1 review
      mapped 1237 of them by script (v0.6.1 spec, Appendix A).
      *GPUI Kit 0.8.0 (2026-10-22, G3 T10):* the 0.8.0 bump renumbered by script 582
      citation chains, 723 digit runs in 33 files of the connector's crate,
      `docs/showcase-exceptions.toml` and `tools/showcase-support/src`
      (Task 2, 901918a1); by hand Task 3b re-pointed 100 of the 188 chains
      the script set aside, 5 more it found outside that list and
      `scripts/connector_parity_exceptions.toml:461` (ea7de303); Task 3
      read 5 of them with the panels whose claims 0.8.0 changed (60ebf586).
      The 0.7.0 bump had re-read about 200 by hand.

### gpui: the splitter renderer into the connector
- [ ] [1.0] The gpui showcase paints its splitters with a renderer of its own that follows `splitter.divider_width`, `divider_color` and `hover_color` (`examples/showcase-gpui/demo.rs`, `resize_handles`); gpui-base's own line is 1px. Move it into the connector as a public renderer an application can install.
      *GPUI Kit 0.8.0 (2026-10-22, G3 T10):* the handle API is public: gpui-base 0.8.0's
      crate root exports the `ResizeHandle` type beside `resize_handle`,
      `ResizeHandleContext` and `ResizeHandleRenderer` (`lib.rs:161-165`,
      `resizable/mod.rs:10-13`), where 0.7.3 re-exported the handle module
      `#[doc(hidden)]` and left `ResizeHandle` out of the root, so a
      connector renderer is installed on public API:
      `ResizablePanelGroup::with_handle_appearance` (`resizable/panel.rs:56`)
      or `ResizeHandle::with_appearance` (`resizable/resize_handle.rs:181`).

### Showcases: a toolbar role and arrow-key roving for the chrome toolbar
- [ ] [1.0] The chrome toolbar of all three showcases is a plain row: no toolbar role for assistive technology and no arrow-key roving between its buttons. gpui-base's `Toolbar` offers both; egui and iced need their own.

### gpui: Questionnaire needs an infallible state constructor
- [ ] [upstream] `QuestionnaireState::new` returns a `Result` (gpui-base 0.7.3 `questionnaire/state.rs:48-51`) while gpui builds entities infallibly, so the showcase cannot hold one without a panic path. Ask upstream for an infallible constructor (or schema validation at a later step), then show `Questionnaire` and drop its nine exceptions (`docs/showcase-exceptions.toml`).
      *GPUI Kit 0.8.0 (2026-10-22, G3 T10):* unchanged at gpui-base 0.8.0
      (`questionnaire/state.rs:48-51`).

### gpui showcase: the Basic tooltip's bubble is never recorded
- [ ] [1.0] The gpui showcase records no `basic.buttons.tooltip.bubble` (`connectors/native-theme-gpui/examples/showcase-gpui/elements.rs` maps `basic-button-tooltip` to the Button and its label only, `:166`, `:362`), so the parity comparator sees the bubble in the iced and egui dumps alone, and no gpui panel shows the bubble's leaves. Hovering the Tooltip button shows the Button's listed panel, which keeps only the info's `not_themeable` notes and drops its `config` lines (`examples/showcase-gpui/inspector.rs:562-565`): where a preset states the bubble's fill, border, padding, `max_width` or (since G2) `show_delay_ms`, the panel says nothing of them. Recording the bubble from the content element `Tooltip::element` builds (`demo::built_tooltip_button`) would give it a dump entry and a listed panel of its own.

### check_showcase_parity.py: the element list's union rule is unchecked
- [ ] [1.0] `docs/showcase-elements.toml` states "A control's leaves are the union of its parts'" (`:36`), but `scripts/check_showcase_parity.py --check-list` (`load_elements`) does not check it, so a leaf added to a part and not to its control passes. A subset check (every `part = true` element's leaves within its control's) would make the rule mechanical.

### M6 close (2026-10-16): items carried to M7, M8, M9a and the presets
The gpui connector's fix phase (M6, `1e60e1c9..`, ledger "## M6") left these. Each sits here until its milestone takes it.
M8 T21 fix round 1 (2026-10-17): its M7, M8 and M9a items are closed, in
[`archive/todo_closed.md`](archive/todo_closed.md) under *Closed items from M6 close
(2026-10-16): items carried to M7, M8, M9a and the presets*; the three `[1.0]` preset
items below stay.
- [ ] [1.0] Presets (kde-breeze), found by M6 T16b's preset-validator run (2026-10-16): `segmented_control.hover_background` states the same uncited pair the expander's hover had (`#dee0e2` / `#31363b`, `kde-breeze.toml` `[light.segmented_control] hover_background`, `[dark.segmented_control] hover_background`). `expander.arrow_color` is uncited. The expander's border inherits `defaults.border` while platform-facts §2.27's KDE cells say none (`frame_enabled = false` must be honoured by every connector). platform-facts.md:1676 names `ItemView_ArrowSize` where :1639 says the arrow is Breeze's `renderArrow` with `ArrowSize`.
      M8 T21 fix round 1 (2026-10-17): its
      `segmented_control.hover_background` and `expander.arrow_color` are also
      items of *Presets: values the M8 T15 platform-facts rows disagree with*
      (kde-breeze, §2.25 and §2.27).
- [ ] [1.0] Presets (windows-11): platform-facts §2.26 states a Fluent card fill (`CardBackgroundFillColorDefault`), and windows-11.toml states no `card.background_color`. Found by M6 T16c's citation check.
      M8 T16 (2026-10-17): the inherited value is the stated token:
      `card.background_color` inherits `defaults.surface_color`
      (`docs/inheritance-rules.toml:274`), which windows-11 states as
      `CardBackgroundFillColorDefault`, #ffffffb3 / #ffffff0d
      (`windows-11.toml` `[light.defaults] surface_color`,
      `[dark.defaults] surface_color`), so the card is filled in the
      platform's colour; what is left is stating it on the card itself.
- [ ] [1.0] Presets (windows-11): the checkbox's hover fill cites no source of its own. Laid over the window, it lands within one 8-bit level of WinUI's `ControlAltFillColorTertiary` (M6 T9's note), but whether the preset means a layer or a replacement is open preset-data work.

### M7 fix phase (2026-10-16)
- [ ] [upstream] epaint 0.36.2 asserts that a glyph fits its font atlas (`TextureAtlas::allocate`, `epaint/src/texture_atlas.rs:226-231`), so a text size whose glyph is wider than `max_texture_side` (at most 16 × 1024, `epaint/src/text/fonts.rs:975`) panics, in release builds too. Ask upstream to skip such a glyph as `allocate_glyph_uncached` already skips one of zero width (`epaint/src/text/font.rs:270`). Until then the egui connector lowers every size it hands egui to `ThemeAtlas::text_size_cap`, derived per pass from `max_texture_side`, `pixels_per_point` and the installed faces' widest `head` box (M7-04, M7 T1); a size the application hands egui itself is its own to clamp.
      Correction (M7 T1 fix round 1): the cap's width is a static face's `head` box and, for a variable or CFF face, the widest of its glyphs' own control boxes at the weight drawn (the `head` box bounds a variable face's default instance only), less the growth TrueType hinting's integer pixels per em allows; the residuals are stated on `ThemeAtlas::text_size_cap`.

### M7 close (2026-10-16): items carried to M8, M9a and the presets
The egui connector's and egui widgets' fix phase (M7, `c5ff9271..`, ledger "## M7") left these. Each sits here until its milestone takes it.
M8 T21 (2026-10-17): its `[v0.7.0]` entries are under *M7 close (2026-10-16), its
`[v0.7.0]` entries* in *This cycle (v0.7.0)*.
- [ ] [1.0] egui text-size cap residuals (M7 T1): the TrueType `ISECT` instruction, the autohinter's CJK and Indic script groups, and hinting settings an application changes itself are unbounded in the hinting source, so they are not covered. Neither is a viewport two immediate levels up during its very first pass, which loses that pass's parts.
