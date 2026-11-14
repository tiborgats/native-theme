# Issue #4, second comment: what a KDE application overrode by hand — rationale

- Issue: <https://github.com/tiborgats/native-theme/issues/4>. The first comment
  asked for KDE Union; its rationale is
  [`0004_kde-union-rationale.md`](0004_kde-union-rationale.md). This document is
  about the reporter's second comment, of 2026-10-30
  (<https://github.com/tiborgats/native-theme/issues/4#issuecomment-6531802306>),
  which answers the first reply's question and lists what their application
  overrode by hand to match the Qt version of the same application under Breeze.
- Written: 2026-11-13, on `v0.7.1-rc2`.
- Status: **implemented on 2026-11-14** for 0.7.1, on `v0.7.1-rc2`, by the plan
  [`../archive/todo_v0.7.1_kde-hand-overrides-plan.md`](../archive/todo_v0.7.1_kde-hand-overrides-plan.md).
  The maintainer approved this document on 2026-11-13, after three review
  rounds, one confirming check and a comparison with the parked 0.8.0 work. The
  specification is
  [`../archive/todo_v0.7.1_kde-hand-overrides-spec.md`](../archive/todo_v0.7.1_kde-hand-overrides-spec.md);
  it lists the few corrections made here since the approval. The reply in §8 is
  a draft and is not posted.

## 1. The report

The reporter's application (`protonctx` v1.3.4,
<https://github.com/l-raider/protonctx/blob/v1.3.4/src/system_theme.rs>) used
native-theme 0.6.0 and native-theme-gpui 0.6.0 on GPUI Kit 0.7.0 (its
`Cargo.lock`). It has since moved to Slint. The comment lists, in four groups:

- **A, before `apply_system_theme`:** the font size re-derived at 96 DPI ("font
  size was scaled twice"); the scroll bar's groove set to 16 px, its thumb width
  to 8 and minimum length to 20, its track to the window background, its thumb
  to the window text at 50 % alpha, hover and pressed to the selection colour.
- **B, after `apply_system_theme`:** scroll bar mode `Always`; `focus_ring`
  off; `muted` = the view background; `popover` = the window background;
  `selection` made opaque; `accent` and `table_hover` = the selection colour at
  30 % alpha; `table_row_border` = the alternate row colour; each of these set
  on `theme.tokens` too; the scroll bar styles written again.
- **C, typed into widgets:** button sizes and state colours, toolbar buttons, a
  table sort header, dialog text, icon and window sizes.
- **D, other:** `SystemTheme::from_system()` run on a thread with a time-out,
  and dark or light for the fallback taken from `detect::system_is_dark()`; a
  private copy of the connector's colour conversion; a colour viewer.

§7 gives the outcome for every item.

## 2. How the facts were established

Everything below was read on 2026-11-13. A value is from a source line, from a
computation on source values, or from a measurement; what was only reasoned says
so.

- **Breeze** at commit `f0b1d7534aa2356d7336241d0c7051522e8a6b68` of
  `github.com/KDE/breeze`, the commit `docs/platform-facts.md` already pins
  ("B" below, `kstyle/` paths). Every file cited is byte-identical at tag
  `v6.8.2`, the Plasma version the maintainer runs; the two colour schemes
  differ there only in translated names.
- **KColorScheme** `27066d47`, **KColorUtils** (kguiaddons) `7c766f6f`, the
  commits the facts file cites; **KStyle** (frameworkintegration) at `733073d3`;
  **Qt** qtbase `v6.10.1` for the font and scale code of §4.1 and for `qmenu.cpp`,
  `v6.11.0` for the item view and style code of §4.4; **Plasma**
  plasma-workspace `v6.8.2` and `v6.7.5`, kwin `v6.8.2` and `v6.7.5`, kscreen
  `v6.7.5`.
- **Toolkits**, as locked: gpui-pre 0.3.12 and gpui-pre-linux 0.3.12,
  gpui-fast-linux 0.1.15, winit 0.30.13, gpui-component 0.9.1, gpui-base 0.9.1.
- **Colours** are computed with Qt's arithmetic (16-bit `QColor` channels,
  `KColorUtils::mix`, the raster blend `kde/colors.rs` already has as
  `overlay_colors`). The method reproduces the hovered scroll bar handle the
  facts file gives by formula and the preset states, #96cfed / #2f6888, and the
  pressed and checked button colours.
- **Measured colours and sizes.** The installed Breeze 6.8.2 style was rendered
  offscreen by Qt 6.12 (PySide6 6.12.0) with Plasma's platform theme, from a
  private `kdeglobals` holding the Breeze Light or the Breeze Dark scheme and
  Noto Sans 10 pt, with Breeze's own settings at their defaults. Every value
  this document changes was measured this way, and most of those it only
  records; §9 holds the table. Each computed value agreed with the pixel Qt
  painted, within one level of one channel in two antialiased lines. Three
  recorded shades are computed only: the scroll bar handle with the pointer on
  the bar, the header's hover and pressed fill, and the hover over an alternate
  row.
- **The parked 0.8.0 work** (branch `v0.8.0-rc1`) read much of the same Breeze
  code for its model: `docs/facts/kde/`, the KDE trace of its specification
  (`docs/todo_v0.7.1_model-completeness-spec-trace-kde.md`), and
  `tools/kde-vectors`, which compiles KDE's and Qt's own colour functions and
  writes their results as tables. This document was written without it and
  compared with it afterwards. On the points compared the two agree on what
  Breeze does (its vector tables' numbers were not compared with §9's): the font
  DPI (§4.1), the combo box popup taking the menu item's fill (trace row
  K1.245), a selected and hovered row lightened (K1.172), the unfocused
  selection's derivation (K1.173), a header's hover and pressed fills (K1.177),
  the focused view's scroll bar handle (K1.92) and the message icon's size
  (K1.224). The menu's and the scroll bar's colour values were not fact lanes
  of the 0.8.0 work, so there is nothing to compare them with there.
- **Measured font sizes.** §4.1's X11 rows were measured on a private X server
  (Xvfb with `Xft.dpi` set through `xrdb`): Qt's scale and font size at three
  DPI values, and the egui showcase's text at 96 and 144. Its Wayland rows are
  computed from the cited sources; measuring them needs a nested compositor with
  a Qt window and a toolkit window in it, which was not set up.

Breeze's palette roles, used throughout (`kcolorscheme.cpp:670-689`): `Window`
/ `WindowText` are `[Colors:Window]` `BackgroundNormal` / `ForegroundNormal`;
`Base` / `Text` are `[Colors:View]`'s; `Button` / `ButtonText` are
`[Colors:Button]`'s; `Highlight` / `HighlightedText` are `[Colors:Selection]`'s.
Breeze's `focusColor()` and `hoverColor()` are `[Colors:View] DecorationFocus`
and `DecorationHover` (`breezehelper.cpp:89-90`). In both Breeze schemes the
three of `Highlight`, `DecorationFocus` and `DecorationHover` are #3daee9, which
is why a wrong role went unnoticed in several rows.

## 3. What goes into 0.7.1 and what does not

Three conditions put a fix into 0.7.1:

1. the right value is read from the platform's source, and measured where a
   measurement is possible;
2. the model already has the field that carries it;
3. every reader of that field, in the core, the three connectors and the
   showcases, was found, and the new value makes none of them worse in what the
   field is documented to mean. A reader that borrows the field for something
   the model has no field for is named with what it loses, and the fix goes in
   only where the gain of the field's own readers clearly outweighs that loss;
   each such weighing is stated where it is made (§4.1, §4.2, §4.3, §4.6,
   §5.2, §5.5, §5.6).

What happens to a defect that fails one:

- **Condition 1** (the value was not read): nothing changes, not even the facts
  row; the defect keeps or gets a backlog entry.
- **Condition 2** (a new field is needed): each new field touches the model, the
  registry and inheritance rules, the presets' tests, the readers, three
  connectors with their contract tables, and the showcases' claims. That is a
  step of its own, and 0.7.x grows one small step at a time (`ROADMAP.md`).
- **Condition 3** (a reader would get worse): the connector needs a design
  first.

A defect that fails condition 2 or 3 is corrected in `docs/platform-facts.md`
at once, so the facts are true, and gets a backlog entry with what was verified
here, so nothing has to be read twice.

## 4. Decisions that change 0.7.1

### 4.1 Font size: the font DPI is 96 on every Linux desktop

**The defect.** The model's sizes are logical pixels: the toolkit multiplies
them by its scale factor. A point size becomes logical pixels at the "font DPI"
(`ResolutionContext::font_dpi`). On Linux the KDE reader, and
`ResolutionContext::from_system()` on every desktop but GNOME and Budgie, take
it from a chain (`native-theme/src/detect.rs:638-676`): KDE's `forceFontDPI` on
an X11 session, then `Xft.dpi` (`xrdb -query`), then the monitor's physical DPI
(`xrandr`), then 96. The connectors apply no DPI or scale of their own to a
size, only the text-scaling factor (`prefs.scaled`), so the font DPI is the one
place the display's scale can enter twice.

**Facts.**

- gpui takes its scale factor on Wayland from the compositor
  (gpui-pre-linux `src/linux/wayland/window.rs:1218-1222`) and on X11 from the
  `Xft.dpi` resource divided by 96 (`src/linux/x11/client.rs:2674-2681`);
  without it on X11 it uses a factor from the monitor's physical size
  (`:2684-2687`). gpui-fast, the connector's `gpui-fast` feature, puts the
  session's `GDK_SCALE` between the two (gpui-fast-linux
  `src/linux/x11/client.rs:2713-2727`). winit, which iced and egui use, takes
  the compositor's scale on Wayland
  (`src/platform_impl/linux/wayland/types/wp_fractional_scaling.rs:71-72`) and
  on X11 the XSETTINGS `Xft/DPI`, then the `Xft.dpi` resource, then the physical
  size (`x11/util/randr.rs:40-53`, `:145-151`). Plasma clears the XSETTINGS
  value (kde-gtk-config `gtkconfig.cpp:347` at v6.8.2), so on Plasma both read
  the resource.
- Plasma writes `Xft.dpi` on Wayland as the largest display scale × 96 when
  legacy applications scale themselves, which is the default (plasma-workspace
  v6.8.2 `startkde/plasma-setup-xwayland/main.cpp:47-58`; kwin v6.8.2
  `src/main.cpp:281-310`; the largest output's scale, kwin v6.7.5
  `src/workspace.cpp:613-626`; plasma-workspace v6.7.5
  `kcms/krdb/krdb.cpp:333-337`). So on Wayland `Xft.dpi` is a copy of the
  display scale. On X11 it is `forceFontDPI`, which kscreen writes as the global
  scale × 96 (`krdb.cpp:338-341`; kscreen v6.7.5 `kcm/kcm.cpp:421-482`).
  plasma-workspace v6.8.2 has no X11 session start-up, so the X11 rows below are
  Plasma 6.7's.
- Qt converts a point size at 96 DPI and multiplies by its scale: on Wayland the
  logical DPI is 96 and the device pixel ratio is the output's scale (qtbase
  `qwaylandscreen.cpp:149-160`, `qwaylandwindow.cpp:1517-1522`); on X11 the
  screen's DPI is `Xft.dpi` (`qxcbscreen.cpp:372-373`, `:865-877`), Qt's scale
  is that DPI divided by 96 and the logical DPI it then uses is 96 again
  (`qhighdpiscaling.cpp:241-256`, `:362-383`; the default rounding policy
  passes fractions through, `qguiapplication.cpp:142-143`). Where the X server
  states no DPI, Qt uses 96 and not the monitor's physical DPI ("We don't want
  to return physical DPI here", `qxcbscreen.cpp:871-876`).

A 10 pt font, in physical pixels. The Qt column is before Qt Widgets' rounding
of the font's pixel size to a whole number (below); the last two rows' native
toolkit is GTK 3.

| Session | `Xft.dpi` | Native toolkit | native-theme app today | with 96 |
|---|---|---|---|---|
| Plasma Wayland, scale 1 | 96 | 13.33 | 13.33 | 13.33 |
| Plasma Wayland, scale 1.5 | 144 | 20.00 | **30.00** | 20.00 |
| Plasma Wayland, scale 2 | 192 | 26.67 | **53.33** | 26.67 |
| Plasma Wayland, outputs at 1 and 2, window on the first | 192 | 13.33 | **26.67** | 13.33 |
| Plasma 6.7 X11, global scale 1, `forceFontDPI` 120 | 120 | 16.67 | **20.83** | 16.67 |
| Plasma 6.7 X11, global scale 2 | 192 | 26.67 | **53.33** | 26.67 |
| XFCE X11, DPI 120 | 120 | 16.67 | **20.83** | 16.67 |
| Sway, scale 2, `Xft.dpi` unset, no GNOME portal backend: the chain falls to `xrandr`, 81 on a 600 × 340 mm 4K monitor | unset | 26.67 | **22.49** | 26.67 |

**Measured** on a private X server (§9): with `Xft.dpi` 144, 120 and 192, Qt
reports a device pixel ratio of 1.5, 1.25 and 2 and a logical DPI of 96, and a
10 pt Noto Sans is 13 logical pixels, so 19.5, 16.25 and 26 physical ones. The
egui showcase, with the kde-breeze preset, draws its body text with capitals 10
px high at `Xft.dpi` 96 and 22 px high at 144: the window grew by 1.5, the text
by 2.2, and labels no longer fit their widgets.

The reporter wrote "a scaled screen" and named both the Wayland scale and
`Xft.dpi / 96` on X11; which session and scale they ran is not known. The
maintainer's desktop is the first row, which is why the defect was never seen
here. The `xrandr` fallback is wrong in either direction: on X11 it adds the
monitor's density to a scale the toolkit already took from it, and under
Xwayland it reports logical pixels per millimetre.

**Options.**

- **A. 96 on every Linux desktop.** The toolkit's scale factor already carries
  the display scale and, on X11, `Xft.dpi`; the font DPI must not carry them a
  second time. Right in every row above. It is the decision GNOME already has
  (`detect.rs`, `GNOME_FONT_DPI`) and Windows has (`windows.rs`, `LOGICAL_DPI`),
  extended to the rest. It is also what the parked 0.8.0 specification decided
  for its model: no reader reports a font DPI, and KDE's is the constant 96,
  from Qt's Wayland screen (`docs/facts/kde/tokens.toml`, `token.text.font_dpi`,
  on `v0.8.0-rc1`). It removes code: two sub-process calls (`xrdb`, `xrandr`)
  and a file read (`kcmfonts`) at the first theme read, and the cache that held
  their answer.
- **B. 96 on Wayland, the chain on X11.** Leaves the three X11 rows wrong.
- **C. Keep the chain and let each connector divide by its toolkit's scale.**
  The theme is built before a window exists, a process has one font size and
  several monitors, and three connectors would each carry the correction.
- **D. Keep the chain and document it.** Leaves the defect.
- **E. A with a way to give `SystemTheme::from_system()` another font DPI.** For
  the one case A makes worse (the last bullet below). An API for a consumer
  that is not known to exist; recorded in the backlog and not built.

**Decision: A.** The font DPI is 72 on macOS and 96 everywhere else.

**Where no font DPI is right** (stated in the documentation, not solved):

- an override of the toolkit's scale on X11 (`GPUI_X11_SCALE_FACTOR`,
  `WINIT_X11_SCALE_FACTOR`): the application owns the scale then, as
  `AccessibilityPreferences::text_scaling_factor` already says for GNOME;
- X11 without `Xft.dpi`: gpui and winit scale the whole interface by the
  monitor's physical size (gpui-fast by `GDK_SCALE` where the session sets it),
  which Qt does not; text and interface stay in proportion to each other, at the
  toolkit's size;
- Plasma 6.7 on X11 with a global scale other than 1 and a forced font DPI that
  departs from it: Qt then takes its scale from `QT_SCREEN_SCALE_FACTORS`, which
  Plasma exports from the global scale (`startplasma-x11.cpp:47-53` at v6.7.5;
  `qhighdpiscaling.cpp:597-623`; a factor of 1 in it is ignored, `:557`, which
  is why the table's `forceFontDPI` 120 row, at global scale 1, holds), and gpui
  and winit take theirs from `Xft.dpi`;
- a toolkit that does not scale by `Xft.dpi` on X11 (none of the three
  connectors'): today the chain gives its application Qt's text size on X11, and
  with 96 it does not. `SystemTheme::from_system()` takes no font DPI, so such
  an application re-derives its font sizes from the resolved font's
  `defined_size`; the text scale's sizes carry no such field. Only the preset
  and TOML paths take a `ResolutionContext` with a `font_dpi` of the caller's.

A residue that 96 does not remove: Qt Widgets rounds a font's pixel size to a
whole number before scaling (10 pt is 13 px, `qfontdatabase.cpp:2838-2841`;
measured), native-theme gives 13.33. Backlog, with the question whether Qt
Quick rounds too.

**What changes.**

- `native-theme/src/detect.rs`: the chain and its cache go: `read_xft_dpi`,
  `detect_physical_dpi`, `parse_xrandr_dpi`, `kde_force_font_dpi`,
  `reads_force_font_dpi`, `parse_force_font_dpi`, `GNOME_FONT_DPI` and
  `desktop_font_dpi` (one constant serves every Linux desktop), the `font_dpi`
  cache slot with `font_dpi_with` and the invalidation in
  `finish_refined_desktop`, and their tests. `session_type` stays (the GNOME
  reader reads it). No public item is removed:
  `DetectionContext::invalidate_font_dpi` stays, does nothing, and says the font
  DPI is a constant per platform since 0.7.1. It is not marked deprecated: a
  deprecation warning in a patch release fails the build of a caller that
  denies warnings.
- `native-theme/src/kde/mod.rs`: `read_kcmfonts_key` and `kcmfonts_key_in` go
  with their tests; the module's list of sources loses `kcmfonts` and its note
  on `Xft.dpi` and `xrandr`. `native-theme/src/watch/kde.rs` stops watching
  `kcmfonts`, and the watcher's source table and `on_theme_change`'s
  documentation in `watch/mod.rs` follow.
- Documentation that names the chain or the cache: `ResolutionContext::font_dpi`
  (`resolve/context.rs`), `Theme::resolve` (`model/mod.rs:455-456`),
  `DetectionContext`'s, `font_dpi`'s, `refined_linux_desktop_sync`'s and
  `invalidate_caches`' rustdoc in `detect.rs`, the comments at `pipeline.rs:35`
  and `kde/mod.rs:115`, the readers table in `native-theme/README.md:71` and its
  copy in `src/lib.rs`, `native-theme/Cargo.toml:25`,
  `native-theme/src/presets/README.md:123-124`, the connectors' mentions of the
  cached font DPI (`connectors/native-theme-iced/README.md`, the gpui and iced
  `watch.rs`), and `scripts/README.md` and `scripts/run_in_runner_env.sh`, which
  name `xrdb` and `xrandr` among the desktop readers.
- `native-theme/src/lib.rs` (`text_scaling_factor`) and
  `native-theme/src/gnome/mod.rs:572-574`: the gpui-pre-linux citation's
  version label becomes 0.3.12 (the lines are the same). The GNOME reader's own
  use of the removed constant (`gnome/mod.rs:603`) takes the one Linux constant.
- `docs/platform-facts.md` §1.3.7: the two rows say what `forceFontDPI` and
  `Xft.dpi` are on each session (the Xwayland scale exists only in the default
  mode and is the largest output's; on X11 Qt uses the value as a scale for the
  whole interface) and that native-theme reads neither.
- `docs/todo.md`: the font DPI question (`:3989-3990`) and the `xrandr`
  fallback's entry (`:1570-1578`) close into `docs/archive/todo_closed.md`; the
  sentence at `:39-41` that says `Xft.dpi` is read is corrected.
- Also named in passing and corrected: the crate documentation's "detection
  caches, which hold the font DPI" (`src/lib.rs:62-64`), the list of sub-process
  commands at `detect.rs:13-14`, `docs/todo.md:1102`, and the first rationale's
  two mentions (`0004_kde-union-rationale.md:29`, `:280`).
- Tests: the font DPI the detection returns is 96 on Linux, and the KDE reader's
  result carries 96. No test sets a session type or a `kcmfonts` file: nothing
  reads them after the fix, and the crate forbids the unsafe call that setting
  an environment variable needs.
- After the fix the private X server run is repeated: the showcase's capitals at
  `Xft.dpi` 144 are 14 to 15 px high (about 1.5 × the run at 96), not 22. Its
  output, and the Qt script's, are kept as evidence (§9).

### 4.2 Menus: the hovered item and the separator

**Facts** (`Style::drawMenuItemControl`, B `breezestyle.cpp:5951-6188`).

- A hovered item is filled with `focusColor` at alpha 0.3 and outlined 1 px in
  `mix(focusColor, WindowText, 0.15)`, with corners rounded at 4.5; its text
  stays `WindowText` (`:6025-6029`, `:6049`, `:6155`; `Blend_Value = 0.3`,
  `breezemetrics.h:176`). Only a *pressed* item is the opaque focus colour with
  `HighlightedText`. `menuItemDrawStrongFocus`, which turns the fill on, is true
  by default (`breeze.kcfg:130-132`).
- The separator is `mix(Window, WindowText, frameContrast)` (`:5984`,
  `breezehelper.cpp:329-332`).

| Field | Facts file and model today | Breeze | Light | Dark |
|---|---|---|---|---|
| `menu.hover_background` | `[Colors:Selection] BackgroundNormal`, opaque #3daee9 | `[Colors:View] DecorationFocus` at alpha 0.3 | #3daee94d | #3daee94d |
| `menu.hover_text_color` | `[Colors:Selection] ForegroundNormal`, #ffffff / #fcfcfc | `[Colors:Window] ForegroundNormal` | #232629 | #fcfcfc |
| `menu.separator_color` | ← `defaults.border.color`, #bcc0bf / #4d545b | `mix(Window, WindowText, frameContrast)` | #c6c8c9 | #4c4e51 |

Over Breeze's menu panel the hover fill is #bcdff1 / #264a5e.

**Options for the hover fill.**

- **A. The translucent colour, #3daee94d.** What Breeze paints, and the form the
  model already uses where a platform paints a layer (`tab.hover_background =
  "#3daee933"` in this preset, the Windows preset's menu hover). It adapts to
  the surface under it, also when an application overlays another menu
  background.
- **B. The opaque composite over Breeze's panel, #bcdff1 / #264a5e.** The same
  pixels on any surface. It goes stale when the surface is another colour, and
  it is not what the platform paints.
- **C. Keep the solid selection colour.** Right for a pressed item and for a
  menu bar title; wrong for every hovered item of every menu and dropdown,
  which is the reported defect.

On the panels the connectors paint, the two differ by three levels per channel
at most (#b9dcef against #bcdff1 in light, #294d61 against #264a5e in dark):
iced and egui paint `menu.background_color`, gpui paints its `popover` token,
and on KDE all three are the window colour (§5.5 says why the panel's own
colour waits). **Decision: A**, with the text colour and the separator as the
table gives them. The KDE reader derives the three from the colour scheme with
helpers it has (`translucent`, `mix`, `frame_contrast`); the preset states the
Breeze values with their citations.

**Readers** (condition 3).

- *Menu items*, in all three connectors and showcases: iced's `styles/aw.rs`
  (`path`, "a row highlight painted over a panel the widget also paints"),
  egui's menu role (`style/roles.rs:275`), gpui's `accent` token as a menu row's
  background (`menu/menu_item.rs:134-139`). They become Breeze's hovered item.
- *Combo box dropdowns*: iced's pick-list and combo-box overlay
  (`styles.rs:1232-1259`) and gpui-component's `Select` and `Combobox` rows
  (`searchable_list/item.rs:113-122`) read the field, the latter through
  `accent`. Breeze paints the row under the pointer in a `QComboBox`'s popup
  with this same tint under window text (measured: #b9dcee / #284c61 on the
  window colour, §9; the 0.8.0 trace reads the same from the code, row K1.245),
  so they gain as the menus do.
- With the preset and the reader stating the menu's hover, an overlay of
  `defaults.selection_background` or the portal's accent no longer moves it, as
  with the reader's other colours derived from `kdeglobals`.
- *The separator*: egui's menu role (`style/roles.rs:294`) and the showcases'
  menu separators. They become Breeze's.
- *Menu bar titles* borrow the field, and lose. Breeze fills a hovered or open
  menu bar title with the opaque hover or focus colour under `HighlightedText`
  (`drawMenuBarItemControl`, B `:5841-5948`), which is what the field holds
  today. iced_aw styles the bar's open title with the same `path`
  (`styles/aw.rs:145`, `:158`), the egui showcase's open title reads the field
  (`showcase-egui/chrome.rs:342`), and gpui-component's menu bar titles are
  ghost buttons whose hover is `accent`. They turn into the 30 % tint under
  window text. The weighing: the field is documented as the menu *item's* hover
  and the model has none for a menu bar title; every menu and every dropdown
  has items, a menu bar has a handful of titles and many applications have no
  menu bar; and the tint under window text stays readable on a title. The loss
  is accepted, and the title's fill joins the backlog entry for §5.4.
- *gpui's other `accent` readers.* `accent` is upstream's general item
  highlight, which the connector gives the menu's hover because, as its comment
  counts, the menu family has seven readers to the toggle's one
  (`colors.rs`, `assign_core`). The others turn from
  solid selection blue under selection text to the tint under window text: a
  toggle's hover and checked fill (`button/toggle.rs:163`, `:216-223`), a ghost
  button's hover (`button/button.rs:1211`, at half the alpha in dark mode), the
  command palette's, the code-action menu's and the touch edit menu's selected
  rows, a calendar's today and range, inline code's background. For the buttons
  Breeze has no solid fill (a flat button's hover is an outline and its pressed
  fill `Highlight` at 0.3, `breezehelper.cpp:665-712`; a checked one is
  `ButtonText` at 0.125), so they are no further from Breeze than today. The
  popup rows are menus in kind and take the menu's look. The calendar and the
  command palette have no Breeze counterpart that was read; they change from a
  solid to a tinted highlight, readable either way.
- *A selected row with `active_highlight` off* (the setting is on by default): a
  list row is filled with `accent` and lettered `accent_foreground`, which
  upstream sets after the caller's style (`list/list_item.rs:209`, `:257-263`);
  a table row is only filled with it and keeps the table's text colour
  (`table/state.rs:2536-2541`). Either way it is the window text on the tint,
  readable. The
  connector's `geometry::list_item_selected` has no effect on that row; its
  documentation says the row keeps `list.selection_text_color` there
  (`geometry.rs:1394-1401`), which is wrong and is corrected.
- *The showcases' own menu bar titles* (the gpui showcase's `MenuTitle`, the
  iced showcase's `menu_bar_style`) fill with the field too, with the same
  judgement as the menu bar titles above; their tests follow.
- One comment becomes false and is reworded: iced's pick-list overlay says the
  platform "measured that fill as it shows, not as a layer"
  (`connectors/native-theme-iced/src/styles.rs:1242-1245`).

**Not in 0.7.1.**

- *No field:* the hovered item's outline (#399acc / #5abaec); the pressed item;
  menu bar titles; the 4 px gap between the panel's edge and the highlight; the
  row's minimum height (the facts say "sizes to font"; Breeze adds a floor,
  `max(font height, 20) + 9`, and a row measures 29 px at Noto Sans 10 pt).
- *Condition 3:* the item's corner radius. The facts say "items are
  rectangular"; Breeze rounds the highlight at 4.5 (`breezehelper.cpp:384`,
  `:400`). `menu.border.corner_radius` is read as the item's radius by egui's
  menu role (`style/roles.rs:287`) and the showcases, and as the panel's by iced
  (`styles.rs:1261`), whose Breeze value is 5 with compositing. One field, two
  meanings.
- *Condition 3:* the item's insets. The model states a 4 px horizontal padding
  (`MenuItem_MarginWidth`) and an 8 px gap between icon and text
  (`MenuItem_TextLeftMargin`). Breeze insets the text of a text-only menu by 4 +
  8 + 4 = 16 px from the highlight's edge (`MenuItem_MarginWidth`,
  `MenuItem_TextLeftMargin`, `MenuItem_ExtraLeftMargin`, B `:6053-6054`,
  `:6101`; measured 16), puts the icon in a column 20 px wide that starts 12 px
  in (Qt `qmenu.cpp:378`, `icone + 4`), and starts the text 2 px after that
  column (`:6097`), which is 4 px after a 16 px icon (measured 4). Both stated
  values are wrong, in directions that partly cancel: correcting the gap alone
  would move the text further from where Breeze puts it. The two belong to one
  fix, with the question of what `padding_horizontal` means for a row with and
  without an icon.
- `menu.border.*` otherwise stays (the model gives a menu no border of its own;
  Breeze's QMenu outline is #c6c8c9 / #4c4e51 where the popover's, from
  qqc2-desktop-style, is #c2c5c9 / #474c51).

### 4.3 Scroll bars: the thumb at rest, and no track

**Facts** (`drawScrollBarSliderControl`, B `breezestyle.cpp:6339-6407`;
`scrollBarHandleColor`, `breezehelper.cpp:277-303`; `renderScrollBarHandle`,
`:1274-1286`).

- At rest the handle's colour is `WindowText` at alpha 0.5, times 0.7 while the
  pointer is off the bar (`:6401-6403`, animations on by default,
  `breeze.kcfg:45-47`), and it is filled at half of that over `Window`
  (`overlayColors(bg, alphaColor(fg, 0.5))`; `alphaColor` multiplies,
  `breezehelper.cpp:349-355`). That is `WindowText` at alpha 0.175 over
  `Window`: **#cbcdcd / #46494b**. With the pointer on the bar but not on the
  handle the factor is 1: #bcbebf / #57595b. While the scrolled view has the
  focus the handle is the focus colour instead: #b1d9ef / #2a546b.
- Breeze paints no groove: the page controls are empty (`:1490-1495`), and the
  scroll area fills the bar's area with its viewport's own colour
  (`eventFilterScrollArea`, `:1806-1822`): `Base` behind a list or a text view,
  `Window` behind a plain scroll area. A 1 px line of `Text` at alpha
  `frameContrast` runs along the bar's inner edge (`:8220-8227`).
- Width 21, handle 8, minimum length 20, never hidden: as the facts say.

| Field | Today | Breeze | Light | Dark |
|---|---|---|---|---|
| `scrollbar.thumb_color` | nothing stated: inherits `defaults.muted_color`, a text colour, #707d8a / #a1a9b1 | `WindowText` at 0.175 over `Window` | #cbcdcd | #46494b |
| `scrollbar.track_color` | ← `defaults.background_color`, #eff0f1 / #202326 | no fill | #00000000 | #00000000 |

**The thumb.** The model has one resting thumb colour for every bar, and Breeze
has two: the grey of a view without the focus, and the focus colour of the one
view the user last clicked into. The grey is the one to state: at any moment
all bars but one show it, and a toolkit that painted every bar in the focus
colour would mark them all as focused. The reader computes it with
`overlay_colors(window, window text, 0.175)` and the preset states the Breeze
values; the focused view's and the pointer-on-the-bar shades have no field.

**The track: options.**

- **A. Transparent.** What Breeze paints. The view shows through, so the bar's
  area is the list's colour on a list and the window's on a window, as in Qt.
  The Windows preset already states a transparent track on a bar that does not
  hide (`windows-11.toml`, `track_color = "#ffffff00"`), so every connector
  already handles one.
- **B. Keep the window colour.** Right on a window-coloured scroll area, wrong
  on every list, table and text view, where today a window-coloured band lies
  over the view's colour (#202326 on #141618 in Breeze Dark).
- **C. The view colour.** The reverse error of B.

**Decision: A.** The preset states `track_color = "#00000000"`;
`inheritance-rules.toml`'s per-platform note for KDE becomes `preset`.

**Readers** (condition 3).

- gpui: the `scrollbar` token and the base layer's track style
  (`base_layer.rs`). gpui-component's bar lies over the content
  (`geometry.rs:1184-1191`), so with no track the content under the bar shows: a
  row's hover fill, and text or cells that reach the edge. In Qt the view ends
  at the bar. An application reserves the bar's width where it wants Qt's layout
  (the connector's `scrollbar_gutter`); the gpui README says so.
- iced: the scrollable's rail. For a bar that does not hide, the connector's
  `styles::scrollbar` already embeds it beside the content
  (`styles.rs:1513-1541`), so the transparent rail shows the container behind
  it, as in Qt.
- egui: the groove is painted from `extreme_bg_color`, which the base style
  takes from the track (`style/base.rs:209-213`); it becomes transparent, as
  under the Windows preset. Two other readers of that slot lose their fill, as
  they already do under the Windows preset: `Frame::canvas` becomes see-through
  and a progress bar outside the connector's roles loses its track. A text field
  is not affected (`text_edit_bg_color` is set from the input's fill, `:216`).
- egui also fills `widgets.inactive.bg_fill` from `thumb_color`
  (`style/base.rs:376-391`), which a slider's rail and a check box outside the
  connector's roles read too. They turn from a text colour to the handle's grey.
  A check box gets nearer Breeze's. A slider outside the connector's Slider role
  loses contrast: its rail goes from 3.7:1 to 1.4:1 against the window in light
  mode and from 6.6:1 to 1.7:1 in dark (found while specifying; the
  specification lists it as an open point for the maintainer). A slider inside
  the role is not affected.

**Not in 0.7.1** (no field): the line at the bar's inner edge; the handle's
outline (its colour at alpha `frameContrast`); the handle of a focused view; the
handle while the pointer is on the bar.

### 4.4 Lists and tables: the hovered row and the header

**Facts** (`drawPanelItemViewItemPrimitive`, B `breezestyle.cpp:4746-4897`;
`drawHeaderSectionControl`, `:6803-6893`).

- A hovered row is `Highlight` at alpha 0.3 (`:4868-4875`), not the hover
  decoration, with a 1 px outline of `Highlight` at alpha 0.8. In a view without
  the focus Breeze takes the colour from the palette's Inactive group
  (`:4832-4837`; Qt clears `State_Active` there, `qabstractitemview.cpp:3996-3997`),
  where Breeze's schemes fade the selection colour (§5.3), so the hover is
  fainter: #c2e0f5 / #1b4155 at alpha 0.3.
- A header section is filled with `Button` and lettered `ButtonText` (`:6827`,
  `:6839`; Qt `qcommonstyle.cpp:1659-1660`). Breeze never reads the Header
  colour set for a `QHeaderView`.

| Field | Today | Breeze | Light | Dark |
|---|---|---|---|---|
| `list.hover_background` | #93cee9 / #2e4a5e, uncited (#93cee9 is KColorScheme's built-in default hover colour, `kcolorscheme.cpp:215`, which no Breeze scheme uses) | `[Colors:Selection] BackgroundNormal` at alpha 0.3 | #3daee94d | #3daee94d |
| `list.header_background` | `[Colors:Header] BackgroundNormal`, #dee0e2 / #292c30 | `[Colors:Button] BackgroundNormal` | #fcfcfc | #292c30 |
| `list.header_font.color` | `[Colors:Header] ForegroundNormal` | `[Colors:Button] ForegroundNormal` | #232629 | #fcfcfc |

Over a row the hover fill is #c4e6f8 / #204357, over an alternate row #bee0f2 /
#264a5e.

**The hovered row.** The field carries the hover of a view that has the focus;
the unfocused view's fainter one has no field. Translucent (#3daee94d) or the
composite over `Base`: the translucent colour is what Breeze paints and is right
over an alternate row where the toolkit lays the hover over the stripe.
**Decision: translucent**, derived by the reader as `translucent(selection
background, 0.3)`.

**The header.** Two options: the Button set, which Breeze paints a
`QHeaderView` with (measured), or the Header set the reader reads today. The
reference for the list's other fields is the Qt Widgets view, so the header
follows it: the Button set, in the facts, the preset and the reader
(`kde/colors.rs:702-705`). The backlog's KDE entry had already found this
(item 4). Whether a Qt Quick table header paints differently was not read. In
Breeze Dark the two sets are equal; in Breeze Light the header turns from
#dee0e2 to #fcfcfc.

**Readers** (condition 3). `list.hover_background`: gpui's `list_hover` and
`table_hover` tokens, the row's own background over the list's fill (a hovered
alternate row loses its stripe there, so it composes over the list colour: one
shade off Breeze, nearer than today), which gpui-component's diff view and
Markdown tables read too; iced's selection list (`styles/aw.rs:420`), emitted as
given over the list fill it paints; egui's list role (`style/roles.rs:799`),
painted over the row, for a hovered and a pressed cell. The header: gpui's
`list_head`, `table_head` and `table_head_foreground`, egui's list role (its
strong text colour, `roles.rs:810`), and the iced and egui showcases' tables.
The header's text colour does not change under Breeze (#232629 / #fcfcfc in
both sets); only its source does. None gets worse. The gpui and iced contrast
pairs on these fields and on the menu's compare like with like and keep passing
(the hovered menu row's text contrast rises from 2.5 to 10.5 in light and from
2.4 to 8.8 in dark).

**Not in 0.7.1.**

- *No field:* the hovered row's outline; the hover in a view without the focus;
  a selected and hovered row (#44bfff); the header's hover and pressed fills
  (#d6ecf8 / #2d4655) and its separators; a list item's padding beyond the
  highlight's margin.
- *Other reasons:* the grid, §5.6; the selection of a view without the focus,
  §5.3.
- *Condition 1:* `sidebar.hover_background` carries the same uncited #93cee9 /
  #2e4a5e. Its source is Kirigami, not Breeze's item views, and was not read; it
  stays in the backlog entry that already lists it.

### 4.5 Dialogs: the message icon is 64 px

Breeze does not set `PM_MessageBoxIconSize`; KStyle, which Breeze is built on,
returns `KIconLoader::SizeHuge`, 64 (`src/kstyle/kstyle.cpp:449-451`).
`QMessageBox` and `KMessageBox` both use it. The facts row says "(none) —
per-dialog", and the preset states `dialog.icon_size_px = 32` without a
citation. The installed Breeze answers 64 (§9); a Breeze built without KStyle
would answer Qt's 32, and no distribution's build was checked beyond this one.

**Decision:** the row and `dialog.icon_size_px` become 64. No connector reads
this field yet: iced records it as having no receiver, and the gpui and egui
dialog icon helpers read `defaults.icon_sizes.dialog`, KDE's dialog icon group,
which is 32 and is another thing. So the fix corrects the datum and changes
nothing a connector draws; an application that reads the resolved
`dialog.icon_size` itself gets 64 where it got 32. A connector helper for the
message icon's size is a backlog entry (§6). The reporter's 56 has no source.

### 4.6 gpui connector: three colours are capped after a mode switch

**The defect.** gpui-component caps the alpha of `selection` at 0.3 and of
`list_active` and `table_active` at 0.2 whenever it applies a `ThemeConfig`
(0.9.1 `src/theme/schema.rs:1039-1070`), which every `Theme::change`,
`Theme::sync_system_appearance`, registry reload and `gpui_component::init` does
(`src/theme/mod.rs`, `Theme::edit`, `:332-334`). No value in a config avoids it:
the config's own colour string is clamped too (`color.rs:777-791`). The
connector's `apply` writes the theme directly, so right after it the three hold
the native colours; after the first switch they are capped and nothing restores
them. The README and `lib.rs` say a switch "reproduces the native palette". No
test checks the three fields after a switch.

The reporter saw selections as "a faint wash". Their application reads the theme
once and never switches the mode, so which rebuild capped it is not known. A
`gpui_component::init` after `apply` would explain both this and the scroll bars
they found hidden: it applies the installed light config again through
`Theme::change`, which caps the three, and sets the scroll bar mode to `Hover`
on Linux (gpui-component `src/theme/mod.rs:36-42`; reasoned from 0.9.1's code,
not reproduced on the versions the reporter ran).

**Options.**

- **A. Restore the three native colours after every upstream rebuild.** The
  connector's observer already repairs what a config cannot carry: the 12
  base-palette colours and the base layer's geometry (`lib.rs`,
  `repair_base_palette`). The documented promise becomes true, and the first
  install and every later state agree.
- **B. Cap them at the first install too.** Agreement the other way: a selected
  row is a 20 % wash on every platform. The connector decided against that
  (`colors.rs`, "list_active should not have alpha forced to 0.2"), and it is
  what the reporter overrode.
- **C. Document the cap.** Leaves the first install and the later state
  different.
- **D. Ask upstream not to cap a colour the config states.** Right, and not
  available in 0.9.1; it goes into the upstream notes of §4.7 beside A.

**Decision: A**, with these rules.

- *Only upstream's own cap is undone.* A field is restored when it holds exactly
  what the cap makes of the native colour (the native colour with its alpha
  lowered to the cap, and the token that colour as a solid fill), and the native
  alpha is above the cap. The comparison is exact: the connector's configs hold
  hex strings, and upstream's parse gives back the bytes the native colour was
  made from. Any other value is the application's own (§4.8) and stays. A value
  the application sets to exactly the capped native colour cannot be told from
  the cap and is replaced too; the README says so.
- *All of `selection`'s copies.* gpui-component copies `selection` into
  gpui-base's colour tokens and into the text view's defaults when it syncs the
  base layer (`src/theme/mod.rs:547`, `src/text/mod.rs:54`); gpui-base paints
  selected text from the first (`selectable_text.rs:223`). So a restore is
  followed by a sync of the base layer, and only then by the connector's base
  overrides, which a sync resets.
- *Where in the observer.* On every delivery, before the branch that absorbs the
  observer's own write (a rebuild merged into that notification would otherwise
  be skipped, `lib.rs:1471-1484`): repair the palette, then check and restore
  the three. If anything was restored: sync the base layer, write the base
  overrides marked as the observer's own, and return. The deferred re-write
  after the first `apply` (`lib.rs:1491-1494`) is not changed: the write it
  makes causes a delivery to the observer, which performs the restore. The
  delivery that follows a restore finds nothing capped, so the sequence ends
  after two deliveries, as today.
- *A mode with no stored variant.* A switch to a mode that was never stored
  applies upstream's default config; the three then hold upstream's own
  constants, which are not a capped native colour, and stay.
- *Tests:* apply a theme, switch the mode both ways through `Theme::change`, and
  check the three fields, their tokens, gpui-base's `selection` token and the
  text view's selection against the native colours; the same after a
  `sync_system_appearance` in the same update as the first `apply`; a value the
  application wrote through `Theme::update` is still there after the delivery
  that follows its write; a value equal to the capped native colour is replaced;
  the deliveries end.
- The places that describe the observer as restoring the 12 colours gain the
  three: in the README lines 19-21, "Light and dark under a preset" (`:190-197`),
  "How re-application works" and "Assumptions"; in `lib.rs` the crate
  documentation, `apply`'s step list, `apply_system_theme`,
  `install_observer_once` and `repair_base_palette`.

**One limit, documented.** gpui-component paints `table_active` *over* a
selected cell in cell-selection mode (`table/state.rs:2463-2472`; off by
default, `cell_selectable`), where it paints it *behind* a selected row. An
opaque native selection colour therefore hides a selected cell's text. That is
so today at the first install. After a mode switch it is new: in 0.7.0 the cap
left the cell readable from then on, and the restore makes it opaque again. The
changelog entry says so. A translucent
`table_active` would keep the cell readable and wash out every selected row; one
token cannot serve both. The README names the limit and the override for an
application that selects cells: a translucent `table_active` through
`Theme::update`, at an alpha other than upstream's 0.2 so that the rule above
leaves it alone, written again after every rebuild (§4.8).

### 4.7 gpui connector: what only upstream can change

**A table's scroll bar is 16 px wide.** gpui-component sizes a table's scroll
bar container with the fixed 16 px `Scrollbar::width()` (`table/state.rs:2743`,
`:2762`), while every other scroll host uses the themed track width, which the
connector sets to `scrollbar.groove_width`, 21 on KDE. In a table the bar is
therefore clipped to 16 px with the thumb off centre, and its hit area still
reaches 5 px into the last column (read from the layout and paint code,
`gpui-base src/scrollbar.rs:1285-1290`, `:1425-1448`, `:1554-1556`,
`:1600-1604`; not run). This is what the reporter worked around with a 16 px
groove.

**Options.**

- **A. Tell upstream, document the limit.** The table's container should use the
  themed width, which gpui-base has to make readable for it
  (`Scrollbar::themed_track_width` is crate-private, `scrollbar.rs:955`).
- **B. Cap the connector's track width at 16.** Makes every scroll bar on KDE 5
  px narrower than the theme says, to repair one host.
- **C. Nothing.**

**Decision: A.** The gpui README gains a "Limits" section after "How
re-application works", which names it with the reporter's workaround (an overlay
that sets `scrollbar.groove_width` to 16).

**The upstream notes.** Four requests, drafted for the maintainer to send to
GPUI Kit's tracker; none is posted without approval:

1. the table's scroll bar container takes the themed track width, and gpui-base
   exposes that width;
2. a selected cell's overlay is painted behind the cell's content, or has a
   token of its own (§4.6);
3. a colour a `ThemeConfig` states is not capped, or the cap can be turned off
   (§4.6);
4. the band outside a focused element is drawn only for a focus that came from
   the keyboard, which GPUI can tell (`Window::last_input_was_keyboard()`, used
   by gpui-component's data table, `table/data_table.rs:255-261`); a text
   input's tinted border stays on any focus; a button's and a check box's border
   tint is keyboard-only too (§5.1). The data table hides its whole focus style
   on a click, which would be wrong for a text input.

### 4.8 gpui connector: documentation

- **Overrides.** Nothing says how an application keeps a value of its own. The
  README gains a section that says:
  - write through `Theme::update`, which keeps `tokens` in step (writing
    `theme.tokens` as well, as the reporter did, is not needed; gpui-component
    `theme_color.rs:373-388`);
  - what removes an override: every `apply`, `apply_system_theme`,
    `apply_accessibility` and theme-watcher re-apply replaces the whole theme,
    and every upstream rebuild from a config (`Theme::change`,
    `sync_system_appearance`, a registry reload) replaces its colours, fonts and
    radii; `focus_ring` and `scrollbar_mode` survive the second kind;
  - so an application writes its overrides again after each of those calls, in
    one function of its own;
  - a value that is theme data belongs in the TOML overlay the README already
    shows, which reaches both modes and survives all of it;
  - the fields the connector itself keeps at the native value: the 12
    base-palette colours, and the three of §4.6 while they hold upstream's cap.
- **The focus ring.** The same section shows turning upstream's ring off
  (`theme.focus_ring = false`) and says what remains: every focused element with
  a border keeps the border tinted in the ring colour, and one without a border
  gets a 1 px line (gpui-component `styled.rs:257-299`). For a text field that
  is Breeze's look on any focus; a focused button keeps a tinted border that
  Breeze shows only for keyboard focus. It says when an application on KDE wants
  this (§5.1).
- **The limits.** The table's 16 px scroll bar (§4.7) and the selected cell
  (§4.6) are the new "Limits" section's two entries.
- **Per-widget helpers.** The reporter typed button sizes by hand because they
  did not know the `geometry` module. The Quick start points to the README's
  *Per-widget geometry* section in one sentence.
- **`list_item_selected`** (§4.2): its documentation is corrected.
- **`Theme::sync_system_appearance` at start-up.** On Linux GPUI reports a light
  appearance until the desktop portal answers (gpui-pre-linux
  `src/linux/platform.rs:144`, `xdg_desktop_portal.rs:49-51`), so calling it
  right after `apply_system_theme` can switch a dark desktop's application to
  light for the first frames. `apply_system_theme` already installs the mode the
  OS reports (`SystemTheme::mode`); the docs say to call
  `sync_system_appearance` only on an appearance change, or to use the
  connector's `on_theme_change`.
- **Citations.** The lines this work touches cite 0.9.1.

### 4.9 gpui connector: a public colour conversion

The reporter copied the connector's private `rgba_to_hsla` to convert model
colours for their own widgets. The iced and egui connectors have public
conversions in a module (`palette::to_color`, `convert::to_color32`).
**Decision:** `native_theme_gpui::to_hsla(Rgba) -> Hsla`, the existing function
made public at the crate's root, where the gpui connector's other free helpers
(`layer_over`, `focus_ring_width`) already are; its colour module holds the
whole token mapping and stays private.

### 4.10 What accompanies the fixes

- `CHANGELOG.md`, under 0.7.1: one entry per fix of §4.1–§4.6 and §4.9, each
  saying what was wrong and what a user sees change, and one for the gpui
  README's additions of §4.8. `to_hsla` has rustdoc with an example. §4.1's
  documentation list also covers the gpui README's mention of the cached font
  DPI (`README.md:621-623`). §4.1's entry says that text
  becomes smaller on a scaled KDE session and on an X11 session that sets
  `Xft.dpi` above 96, and that where `Xft.dpi` is unset (Sway and the like, X11
  without a settings daemon) it can become larger or smaller, to the size Qt and
  GTK draw.
- `docs/platform-facts.md`: every row this document found wrong is corrected in
  place (a row is one line, so no cited line number moves), those of §5
  included. `docs/inheritance-rules.toml`'s notes on `menu.hover_background`,
  `menu.hover_text_color`, `menu.separator_color`, `scrollbar.thumb_color`,
  `scrollbar.track_color` and `list.header_font.color` follow.
- Tests and tables that pin today's values and change with them:
  `native-theme/src/presets.rs:469-478` (which presets state
  `menu.hover_background`); the two header tests in `kde/colors.rs`; in
  `kde/mod.rs` the list of derived leaves, the tests of which colour set moves
  which leaf, and the test that the reader's colours under Breeze are the
  preset's; the connectors' contract tables and the showcases' claims wherever a
  value or its source is named.
- The changelog entries name what gets further from Breeze or changes for an
  application: menu bar titles (§4.2), gpui's toggles, ghost buttons and other
  `accent` readers (§4.2), content under gpui's scroll bars (§4.3), a selected
  cell after a mode switch (§4.6), `dialog.icon_size` for an application that
  reads it (§4.5).
- A preset value that stays although its facts row is corrected gets a comment
  saying that it disagrees with the row and why it waits: the focus ring's three
  values (§5.1), the menu's padding and icon gap (§4.2; the preset states no
  menu radius), the primary button's colours (§5.2).
- `ROADMAP.md`'s 0.7.1 section: its "Delivered" paragraph names these fixes.
- No function is ported from KDE's sources: every derivation uses a helper the
  reader already has, and is written from this document's description of what
  Breeze paints, not from Breeze's code.
- The measurement's evidence (§9).
- The release's assets are captured again afterwards: the KDE screenshots
  change, and the provenance stamp covers the sources.

## 5. Decided, and not in 0.7.1

### 5.1 The focus ring

**Facts.** Breeze shows focus in two ways. It recolours the widget's own frame:
a line edit's on any focus (`focusColor`), a button's on keyboard focus
(`Highlight`), each 1 px. And, for a focus that came from the keyboard (Tab,
Backtab or a shortcut) only, it adds a `QFocusFrame`: a band 2 px wide of
`Highlight` at alpha 0.3, from 1 px inside the widget's rectangle to 1 px
outside it (B `breezestyle.cpp:1632-1681`, `:6614-6781`; `PM_FocusFrameHMargin`
= 2, `:737-739`, measured). The facts rows and the preset say 1 px, 2 px away,
in the opaque focus colour, with no source: no Breeze code draws that.

**Why not now.** In the model's terms the ring is #3daee94d, width 2, offset −1.

- gpui-component has one `ring` colour for two jobs, the tinted border of a
  focused element and the band outside it, whose alpha it sets itself
  (`styled.rs:249-327`). A translucent ring colour would turn a focused text
  field's border from Breeze's solid blue to a faint one. Giving `ring` another
  source, the input's focus border colour, would change Windows, whose ring and
  input focus colours differ.
- gpui-component's buttons, inputs and check boxes, and the iced and egui
  connectors' rings, are drawn on every focus, a click included, where Breeze
  draws the ring for keyboard focus only. GPUI itself can tell the two apart
  (`Window::last_input_was_keyboard`, gpui-pre `src/window.rs:3641-3644`), and
  one gpui-component widget uses it; whether iced or egui can was not read.

That is a design question over every platform and three connectors, with an
upstream request in it (§4.7, request 4): condition 3 fails.

**Decision.** The facts rows are corrected now. The preset keeps its values; its
comment above them says they disagree with the corrected rows and why, and the
backlog entry carries the questions above. The connector does not turn
upstream's ring off by itself on KDE: no datum in the theme says "keyboard
only", and off would also remove the ring for a keyboard user, who sees it in
Breeze. Until the design, an application on KDE that prefers no band on a click
turns the ring off (§4.8 says what remains); the reporter's override was a sound
choice.

### 5.2 The default button

Breeze fills the default button of an active window with `mix(Button,
Highlight, 0.2)` (#d6ecf8 / #2d4655, measured) under the ordinary button text
(`breezehelper.cpp:700-702`). The facts say "← accent", and
`button.primary_background` inherits the opaque #3daee9 under white. The two
fields cannot simply take the tint and the button text: gpui's `primary` and
`primary_foreground` tokens (`colors.rs:203-204`), which gpui-component's
widgets use as the theme's accent, and egui's selection fill
(`style/roles.rs:229`) read them. The weighing: one button per dialog would
gain; every accent-coloured widget of gpui-component and every selection in
egui would turn pale. Condition 3 fails. The facts rows are corrected; the
backlog entry asks for the connectors to stop reading the primary button's fill
as the accent first.

### 5.3 The selection of a view without focus

Breeze's schemes set `ChangeSelectionColor`, so a selected row in an unfocused
view is `tint(Window, Selection, 0.4)` (#c2e0f5 / #1b4155, measured) under the
*window* text colour. The facts say the selection does not change. The model has
`defaults.selection_inactive_background` and no text colour to go with it. No
connector draws the field today (gpui and egui have an accessor only; iced
records no receiver), so the value alone would change nothing on screen and
would give
an application that pairs it with `selection_text_color` white text on a pale
fill. The macOS preset already has that pairing (#dcdcdc with white). Condition
2 fails for the text colour, and nothing on screen would change: the facts row
is corrected, and the backlog entry names KDE and macOS with the missing field.
The derivation itself is not a cost: the 0.8.0 branch has it written in a clean
room and checked against KDE's own code (`kde_selection_inactive`, `kcu_tint`,
with their tables from `tools/kde-vectors`), to be carried over when the field
gets its text colour.

### 5.4 Outlines: how Breeze shows hover and focus

Breeze signals hover on a button, a combo box, a check box and a slider handle
by recolouring the outline, never the fill. The model carries
`hover_border_color` and `focus_border_color` for text inputs only. So on KDE a
connector's button shows no hover at all (its hover fill is, correctly, the rest
fill), and the reporter typed the hover border by hand. The same holds for the
hovered menu item's and list row's outlines, a menu bar title's fill, the
header's hover and pressed fills and a toolbar button's states (hover: a 1 px
`Highlight` outline; pressed: `Highlight` at 0.3; checked: `ButtonText` at
0.125). This is the largest gap the report shows, and it is the model's: it is
proposed as the next 0.7.x step, "Breeze's hover and focus outlines", with its
own rationale. The preset values the backlog already lists as contradicting the
facts for want of these fields (`checkbox.hover_background`,
`slider.thumb_hover_color`, `combo_box.hover_background`) wait for it: removing
a hover fill before the outline exists would remove the only hover feedback.

### 5.5 The menu panel's colour

Breeze's menu panel is `mix(Window, Base, 0.3)` (#f4f5f5 / #1c1f22, measured;
`drawPanelMenuPrimitive`, B `:4708`, `breezehelper.cpp:200-203`), four to five
levels from the window colour the facts and the model give it.
`menu.background_color` has readers that are not menus: egui's base style takes
its `window_fill` from it (`style/base.rs:201`), which fills every
`egui::Window`, popup and tooltip, and iced_aw fills the menu *bar* with it
(`styles/aw.rs:152`); Breeze paints those in the window colour. gpui has no
receiver for the field at all: gpui-component paints a menu in its `popover`
token (`contract.rs:1282-1289`), which is the window colour on KDE since 0.7.0.
Changing the field would move more surfaces away from Breeze than toward it, by
a difference that is barely visible. Condition 3 fails; the facts row is
corrected; backlog.

### 5.6 The table's grid

`QTableView` draws its grid by default, 1 px, in `QPalette::Mid` (Qt
`qtableview.cpp:1533-1535`, `qcommonstyle.cpp:5126-5128`; Breeze does not
override it), which is KColorScheme's `MidShade` of the window colour at the
scheme's `[KDE] contrast` (`kcolorscheme.cpp:685`, `:526`): #c4c8cc / #1c1f21
under Breeze's 4 (measured), #bdc1c6 / #1c1e21 under KColorScheme's default of
7. Tree and list views draw no line between rows. The facts say "(none)", and
`list.grid_color` inherits the border colour, #bcc0bf / #4d545b. In light mode
that is close to Qt's grid; in dark mode it is far brighter than Qt's, which is
what the reporter saw.

The field has two readers that are not a table's grid. egui's list role draws
every separator and every frame outline inside a list with it
(`style/roles.rs:776-779`; egui has one stroke for both and the role reads no
`list.border.color`), and gpui's `chart_grid` token reads it (`colors.rs:446`).
With Breeze Dark's grid colour, a list's own outline and a chart's grid would
all but vanish on the window (#1c1f21 on #202326), where Breeze frames a list in
a clearly visible line. The weighing: a dark table's grid would gain; every
list's frame in egui and every chart's grid in gpui would be lost in dark mode,
which is worse than a grid that is too bright. Condition 3 fails. The facts row
is corrected; the backlog entry asks first for egui's list role to separate the
frame from the grid. The derivation from a colour scheme is not a cost:
`KColorScheme`'s shade is written and checked on the 0.8.0 branch (`kde_shade`,
as in §5.3), to be carried over with the fix. Until then an application sets gpui's
`table_row_border` itself (§4.8), to Qt's #c4c8cc in light and #1c1f21 in dark.
The reporter's choice was the alternate row colour. Breeze's is #1d1f22 in dark,
close to Qt's grid, and #f7f7f7 in light, all but invisible on white. The
#2a2e32 they quote for Breeze Dark is what the kde-breeze preset stated before
0.7.0 (`kde-breeze.toml:627` at v0.6.0), not Breeze's value
(`BreezeDark.colors:115`, `BackgroundAlternate=29,31,34`).

### 5.7 Smaller points recorded in the backlog

- `button.min_height_px = 32` has no citation, and `min_width` 80 and the 6 px
  padding are Breeze's values before its 2 px frame margin. A button's size hint
  is text + 2 × (6 + 2): "OK" at Noto Sans 10 pt measures 84 × 34, and Breeze
  draws the frame 1 px inside that, so the visible box is 82 × 32 with 7 px
  between the border line and the text. The stated values give a box 80 × 32
  with 6 px: 2 px narrower at the minimum, the height equal. Small, and a
  question of what the model's padding is measured from.
- The preset's dialog minimum and maximum sizes are uncited, and the facts give
  KDE none.
- The reporter's 14 px button padding, 56 px icons, 1.25 line height and 50 %
  disabled opacity have no source in Breeze (6 px in the model, 7 visible; 64;
  the font's own; dimmer colours, no opacity).

## 6. Backlog entries this work writes

1. `[upstream]` GPUI Kit: the four requests of §4.7.
2. `[v0.7.x]` KDE: what Breeze draws that the model has no field for, from issue
   #4. §5.4's outlines first; then the "no field" lists of §4.2, §4.3 and §4.4,
   the menu row's minimum height among them; each with its source and values.
3. `[v0.7.x]` KDE values that wait for a connector design, with the corrected
   facts against the preset's present values: the focus ring (§5.1), the default
   button (§5.2), the menu panel's colour (§5.5), the table's grid (§5.6), the
   menu item's radius and its padding and icon gap (§4.2).
4. `[v0.7.x]` The selection of a view without the focus, KDE and macOS, with its
   missing text colour (§5.3).
5. `[1.0]` Qt's whole-pixel font sizes and a font DPI for
   `SystemTheme::from_system()` (§4.1); the button's sizes and the dialog's
   (§5.7); a connector helper for the message icon's size, `dialog.icon_size`
   (§4.5).

Closed by this work: `docs/todo.md:3989-3990` and `:1570-1578` (the font DPI);
the KDE half of the scroll bar thumb's entry at `:391`; in the `[v0.7.x]` KDE
entry at `:1732-1754`, which already recorded several of these defects, items
(1) the menu's hover, (4) the header and (6) the list's hover; and the list
half of the uncited hover colours' entry at `:2561`. That KDE entry's items (3)
the inactive selection and (9) the focus ring are not written twice: entries 4
and 3 above extend them with what was verified here.

## 7. The report, item by item

| # | The reporter's override | Outcome |
|---|---|---|
| A1 | fonts re-derived at 96 DPI | **fixed in 0.7.1** (§4.1) |
| A2 | groove 16 px | upstream's table; documented, request drafted (§4.7) |
| A2 | thumb width 8, minimum length 20 | already the model's values |
| A2 | thumb = window text at 50 % | **fixed**: Breeze's rest colour, alpha 0.175 over the window (§4.3) |
| A2 | track = window background | **changed** to no track, as Breeze paints (§4.3) |
| A2 | thumb hover / pressed = selection | already right since 0.7.0: the hover decoration at half alpha |
| B3 | scroll bar mode `Always` | the connector sets `Always` for Breeze, then and now. Not reproduced; the one path found to another mode is an `init` of gpui-component after `apply`, which the docs warn against |
| B4 | `focus_ring = false` | a sound choice for now; documented (§4.8); the ring's values wait for a design (§5.1) |
| B5 | `muted` = view background | the application's own use of a token; the view's colour is the theme's `list` and `table`. No change |
| B6 | `popover` = window background | popups take the window colour since 0.7.0. Breeze's menu panel is a slightly different mix, which waits (§5.5) |
| B7 | `selection` opaque | **fixed**: restored after every rebuild (§4.6) |
| B8 | `accent`, `table_hover` = selection at 30 % | **fixed**: the menu's and the list's hover are that tint (§4.2, §4.4) |
| B9 | `table_row_border` = alternate row colour | today's grid is too bright in dark mode. The right colour is known (Qt's `Mid`, #c4c8cc / #1c1f21); the field has other readers that would suffer, so it waits, and an override to those two colours is what matches Qt (§5.6) |
| B10 | tokens set as well | not needed through `Theme::update`; documented (§4.8) |
| B11 | scroll bar styles written again | the observer and its deferred re-write have done this since 0.6.0 |
| C | button height, width, padding, border | sizes are in `geometry::button`, now pointed to (§4.8); the hover and focus border wait for §5.4 |
| C | pressed button fill | the model's since 0.7.0 |
| C | disabled at 50 % opacity | not Breeze's: it dims by colour |
| C | toolbar buttons, sort header | no fields (§5.4) |
| C | dialog text, 56 px icons, window sizes | text is the general font; the message icon is 64 (§4.5); Breeze states no window size |
| D | `from_system()` with a time-out | fixed in 0.7.0: the portal reads end within 2 s |
| D | dark or light from `system_is_dark()` | right; the `sync_system_appearance` caveat is documented (§4.8) |
| D | private `rgba_to_hsla` | **`to_hsla` is public** (§4.9) |
| D | colour viewer | the showcase's widget info lists each widget's colours with their sources; no change |

## 8. Reply (draft, not posted)

> Thank you, this was very useful, and I am glad you wrote it down even though
> your application has moved to Slint. I checked every point against Breeze's
> source code, and then against what Qt really paints by rendering Breeze's
> widgets and reading the pixels. Several points were real bugs in native-theme.
> Version 0.7.1 fixes these:
>
> - **Text too big on a scaled screen.** You were right, and it was not only
>   GPUI: native-theme used the display's DPI to turn point sizes into pixels,
>   and the GUI library then applied the display scale a second time. It now
>   uses 96 on every Linux desktop, as Qt does.
> - **Menu and list hover.** Breeze paints the hovered row with the highlight
>   colour at 30 %, under normal text. For menus native-theme said "solid
>   highlight", for lists a blue that had no source. Your 30 % was right for
>   both, and both are fixed.
> - **Scroll bar.** The thumb at rest had no KDE value and fell back to a text
>   colour. It is now what Breeze computes: the window's text colour, mostly
>   transparent, over the window colour. Breeze paints no track at all, so
>   native-theme now gives none, and the list's own colour shows behind the bar.
>   (In GPUI the bar lies on top of the content; for your own scroll areas the
>   connector's `scrollbar_gutter` keeps the content out from under it.)
> - **Table header.** Found while checking your list: it used the wrong group of
>   colours from the colour scheme, and in the light theme that made it the
>   wrong colour. Fixed.
> - **Selections turning faint.** gpui-component makes three selection colours
>   partly transparent (down to 20–30 %) whenever the theme is switched or
>   reloaded. The connector now puts the native colours back each time.
> - **`rgba_to_hsla`.** There is now a public `native_theme_gpui::to_hsla`.
>
> Fixed earlier, in 0.7.0: the portal call now gives up after 2 seconds.
>
> With 0.7.1 (which is built on GPUI Kit 0.9.1) you could delete these
> overrides: the font size, the scroll bar's colours, thumb width and minimum
> length, `popover`, `selection`, `accent`, `table_hover`, the writes to
> `theme.tokens` (`Theme::update` keeps them in step by itself there) and the
> second write of the scroll bar styles (the connector does that itself). Still
> yours to keep: the 16 px groove (until gpui-component's table is fixed),
> `focus_ring`, `muted` (that one is your application's own choice) and
> `table_row_border`. The scroll bar mode override is not needed as long as
> `gpui_kit::init` runs before `apply_system_theme`.
>
> Some things are not fixed yet:
>
> - **The 16 px scroll bar in tables** is a bug in gpui-component's table, which
>   ignores the theme's width. I am reporting it there.
> - **Table lines.** You were right that they are too bright in the dark theme.
>   Qt draws a table's grid in #1c1f21 there (and #c4c8cc in the light theme,
>   which is close to what native-theme gives). I know the right colours now,
>   but the same value is also used for a list's frame in another GUI library,
>   where the dark one would make the frame disappear, so I have to separate
>   those two first. Until then, setting `table_row_border` yourself is right;
>   those two colours match Qt. (You used the alternate row colour. Breeze
>   Dark's is #1d1f22, which is nearly Qt's grid colour; the #2a2e32 you saw was
>   a wrong value in native-theme's own Breeze preset before 0.7.0. In the light
>   theme the alternate row colour is almost invisible as a grid.)
> - **The focus ring.** Breeze draws a ring only when you move with the Tab key.
>   With the mouse it only recolours a text field's border. gpui-component draws
>   its ring however the focus came. GPUI itself can tell keyboard from mouse,
>   so I will ask gpui-component to use that. Until then, turning the ring off,
>   as you did, is a good choice on KDE; the README now shows how and says what
>   stays visible.
> - **Hover borders on buttons.** Breeze shows hover by recolouring a button's
>   border, not its fill. native-theme has no value for that yet, which is why
>   you had to type it yourself. It is at the top of my list.
> - **Menus.** In GPUI a menu has the window's colour (since 0.7.0, so you no
>   longer need to set `popover`). Breeze's real menu colour is very slightly
>   different (a little toward the list colour). That difference stays for now.
>
> I could not reproduce the hidden scroll bars: the connector asks for
> always-visible bars on Breeze, then and now. One thing that would hide them is
> calling `gpui_kit::init` after `apply_system_theme`: it applies the theme
> again in light mode and makes the scroll bars appear only under the pointer.
> It would also explain the faint selection.
>
> A few values you typed do not come from Breeze: a button's side padding is 6
> px in native-theme (about 7 px on screen), not 14; a disabled button is drawn
> with dimmer colours, not at 50 % opacity; and the message box icon is 64 px.
> The button sizes are already in native-theme: `geometry::button` applies them
> for you (the README's "Per-widget geometry" section).
>
> The rest of your list, briefly. Toolbar buttons and the pressed sort header:
> native-theme has no values for their states yet; they belong to the same work
> as the hover borders. Dialog text is the general font, and Breeze sets no
> dialog window size. Taking dark or light from `system_is_dark()` for the
> fallback was right. And for looking at every colour of a theme, the showcase
> example shows each widget's colours with where they come from.

The sentences "I am reporting it there", "I will ask gpui-component" and "at the
top of my list" are the maintainer's to confirm before posting.

## 9. Measurement

**Breeze's painting.** Breeze 6.8.2 under Qt 6.12, offscreen, as §2 describes;
light / dark. "Grab" is a real widget's `grab()`; "option" is a hand-built
`QStyleOption` drawn into an image. The palette Qt reported was the scheme's
(`Window` #eff0f1 / #202326, `Mid` #c4c8cc / #1c1f21, Inactive `Highlight`
#c2e0f5 / #1b4155).

| What | Measured | Computed in §4–§5 | How |
|---|---|---|---|
| Scroll bar handle at rest, view without the focus | #cbcdcd / #46494b | the same | grab and option |
| Scroll bar handle, view with the focus | #b1d9ef / #2a546b | the same (§4.3, no field) | grab |
| Scroll bar handle hovered | #96cfed / #2f6888 | the same (the preset's) | option and a real hover |
| Bar's area beside the handle, in a list | #ffffff / #141618 | `Base`: no track | grab |
| The same, in a plain scroll area | #eff0f1 / #202326 | `Window`: no track | grab |
| Line at the bar's inner edge, in a list | #d3d4d4 / #424445 | `Text` at 0.2 over `Base`: #d3d4d4 / #424446 | grab |
| Bar thickness, handle width, minimum length | 21, 8, 20 | the same | grab, `pixelMetric` |
| Menu panel | #f4f5f5 / #1c1f22 | the same (§5.5) | grab |
| Hovered menu item, fill | #bcdff1 / #264a5e | #3daee94d over the panel | grab |
| Hovered menu item, text | #232629 / #fcfcfc | the window text | grab |
| Hovered menu item, outline | #399acc / #5abaec | the same | grab |
| Menu separator and panel border | #c6c8c9 / #4c4e51 | the same | grab |
| Menu row height (text only) | 29 | `max(18, 20) + 9` | grab |
| Menu text inset from the highlight (text only) | 16 | 4 + 8 + 4 | grab |
| Menu icon (16 px) to text | 4 | (20 − 16) / 2 + 2 | grab |
| Combo box popup, row under the pointer: fill, text | #b9dcee / #284c61 on #eff0f1 / #202326; #232629 / #fcfcfc | #3daee94d over the window colour, window text (§4.2) | grab of the open popup, real hover |
| Hovered list row, focused view | #c4e6f8 / #204357 | #3daee94d over `Base` | option and a real hover |
| Selected row, focused view | #3daee9, text #ffffff / #fcfcfc | the same | option and grab |
| Selected row, window not active | #c2e0f5 / #1b4155, text #232629 / #fcfcfc | the same (§5.3) | option and grab |
| Selected and hovered row | #44bfff | the same | option |
| Table grid | 1 px, #c4c8cc / #1c1f21 | `Mid` (§5.6) | grab, `styleHint` |
| Lines between rows of a tree or list | none | none | grab |
| Header section, fill and text | #fcfcfc / #292c30, #232629 / #fcfcfc | the Button set | grab |
| Default button, fill / border / label | #d6ecf8 / #2d4655, #87c0dd / #4882a1, #232629 / #fcfcfc | the same (§5.2) | grab |
| Plain button, fill / border | #fcfcfc / #292c30, #d1d1d2 / #535659 | the same (the preset's) | grab |
| `QPushButton("OK")` size hint; font height | 84 × 34; 18 | text + 16, minimum 84 | `sizeHint` |
| `PM_MessageBoxIconSize` | 64 | 64 | `pixelMetric` |
| `PM_FocusFrameHMargin` | 2 | 2 (§5.1) | `pixelMetric` |
| `SH_ScrollBar_Transient` | 0 | never hidden | `styleHint` |

**Font sizes on X11.** A private X server (Xvfb, 1920 × 1080) with `Xft.dpi` set
by `xrdb`; nothing on the real desktop.

| `Xft.dpi` | Qt: device pixel ratio, logical DPI | Qt: 10 pt Noto Sans, logical → physical px | egui showcase (kde-breeze preset), height of the body text's capitals |
|---|---|---|---|
| 96 | 1, 96 | 13 → 13 | 10 px |
| 120 | 1.25, 96 | 13 → 16.25 | not captured |
| 144 | 1.5, 96 | 13 → 19.5 | **22 px** today (14 to 15 expected after §4.1) |
| 192 | 2, 96 | 13 → 26 | not captured |

The measuring scripts and their printed output go into the repository with the
fixes, as `docs/evidence/breeze-offscreen/2026-11-13/`, beside the OS probe's
evidence; the corrected facts rows cite it. Breeze's colour scheme files are not
copied there: the folder's README says how the runs' `kdeglobals` files are
rebuilt from them. The runs repeated when the fixes are implemented (the combo
box popup, the Qt font sizes, the showcase's text before and after §4.1) go
into a folder of their own date. The images are not kept.
