# 0.7.1: the KDE fixes from issue #4 — specification

- Rationale: [`../issues/0004_kde-hand-overrides-rationale.md`](../issues/0004_kde-hand-overrides-rationale.md),
  approved by the maintainer on 2026-11-13. This document says what changes, where,
  to which value, with which tests and gates. It decides nothing the rationale did
  not decide, except the small points listed under *Decisions taken while
  specifying*.
- Plan: [`todo_v0.7.1_kde-hand-overrides-plan.md`](todo_v0.7.1_kde-hand-overrides-plan.md).
- Written: 2026-11-14, against `v0.7.1-rc2` at `387fc449`. Every `file:line` is
  that commit's unless it names another version.
- Status: **implemented on 2026-11-14**, on `v0.7.1-rc2` (commits `46e0ed6e` to
  `431673d9`, then the close-out commit that moved this document and the plan
  here from `docs/`); the plan's status line names the two departures. The text
  below is the specification as it was approved: it speaks of the work as still
  to do, and gives this document's path as it was in `docs/`.
- `<scratch>` stands for the working session's scratch folder, which holds the
  downloaded upstream sources and the measurement outputs; what the repository
  needs from it is copied in by Part C.5.

## How to read it

Five parts: four lanes of work and the close-out. Each of the four lanes lists its changes by file, the tests, the
documentation text, the gates that cover it, and the problems its author found
while writing it against the code.

| Part | Rationale | Files it owns |
|---|---|---|
| A — Font DPI | §4.1 | `native-theme/src/` (detect, kde reader's kcmfonts read, watcher, gnome reader's constant), the documentation that names the chain |
| B — KDE values | §4.2–§4.5 | `native-theme/src/kde/colors.rs`, `presets/kde-breeze.toml`, the tests that pin today's values, the connector comments that become false |
| C — Documentation and data | §4.10, §5, §6, §9 | `docs/platform-facts.md`, `docs/inheritance-rules.toml`, `docs/todo.md`, `docs/archive/todo_closed.md`, `docs/issues/0004_kde-union-rationale.md`, `docs/evidence/`, `ROADMAP.md` |
| D — gpui connector | §4.6–§4.9 | `connectors/native-theme-gpui/` (`src/lib.rs`, `src/colors.rs`, `src/geometry.rs`, `README.md`), the upstream request drafts |
| E — Release close-out | §4.10's last item | the sweep of ungated line citations, the asset run, both stamps, the final check |

Every edit has one owner. Three files are edited by more than one part, each
part with its own, different edits: `CHANGELOG.md` (each part's entries),
`native-theme/src/kde/mod.rs` (Part A removes the `kcmfonts` read, Part B changes
tests it names) and `connectors/native-theme-gpui/README.md` (Part A rewords the
font DPI sentence at `:621-623`, Part D edits the rest). There each edit is
applied by its quoted text or its test's name, not by line number, and the plan
fixes the order.

**Lines that must not move.** The gpui showcase's test
`every_colour_claim_is_read_at_the_line_it_cites` opens each colour claim at the
source line it cites. At `387fc449` its claims pin, in
`connectors/native-theme-gpui/src/`: `variants.rs` lines 62–64, `geometry.rs`
lines 230–772 and `colors.rs` lines 281–718; nothing in `lib.rs`. An edit above
or inside one of these ranges keeps the line count, or re-points the claims in
the same change.

## Decisions taken while specifying

They change no result of the rationale; each is the smaller or the safer choice.

1. **A.** The one Linux constant is `LINUX_FONT_DPI = 96.0`; the KDE and GNOME
   readers both use it. `session_type` gets the `portal` feature gate, since the
   GNOME reader is its only reader left and a `kde`-only build would warn.
2. **A, B, D.** `CHANGELOG.md` has no `### Fixed` heading under 0.7.1 yet. The
   first task of the plan that adds a fix entry creates it, after `### Changed`;
   the others add under it.
3. **B.** One new constant in the reader, `SCROLLBAR_IDLE_HANDLE_ALPHA = 0.175`.
   The menu's and the list's hover both reuse the existing `BLEND_VALUE`, as
   Breeze's own code uses one constant for both. The track and the dialog icon
   size are preset values only: no colour scheme changes them.
4. **C.** Three facts rows the rationale does not list are wrong for the same
   reasons as rows it does list, and are corrected with them: line 1127 (calls
   `forceFontDPI / 96` a text scale), line 610 (calls `[Colors:Header]` the
   column header's set) and line 633 (cites the wrong file for
   `PM_FocusFrameHMargin`).
5. **C.** Backlog entries the rationale's §6 missed are closed or folded in:
   *KDE's dialog icon* closes (§4.5 fixes it); *kde-breeze: the Qt Widgets menu
   fill* folds into the new entry 3; the macOS selection entry is
   cross-referenced from the new entry 4.
6. **C.** The evidence folder holds no copy of Breeze's colour scheme files
   (they carry Breeze's licence header). The scripts stay as they ran; the
   folder's README quotes the ten lines the runs appended to each scheme and
   gives the commands that rebuild the three config folders from the scheme
   files at the pinned commit.
7. **C.** Runs made at implementation time (the combo box popup with its text
   colour as a hex value, the Qt font script's output, the showcase's capital
   heights before and after the fix) go into a folder of their own date, with a
   README stating date, commits, commands and environment. A facts cell or a
   closing text that quotes such a number takes it from that folder.
8. **C.** The two new `[wrong_safety_nets]` notes are appended at the end of
   their section: twelve preset comments cite later lines of that file by
   number.
9. **D.** New items in the gpui connector's `colors.rs` go into the gap after
   line 719, before the test module: the showcase's colour claims cite lines
   281–718 of that file and a test fails if they move. `to_hsla` is a public
   three-line wrapper of the private `rgba_to_hsla` at the crate's root, not a
   rename (307 lines in nine files name the private one), and needs an `absent`
   entry for iced and egui in `scripts/connector_parity_exceptions.toml`.
10. **D.** The deferred re-write after the first `apply` stays as it is: the
    write it makes causes the observer delivery that performs the restore, so a
    restore of its own would be redundant and no test could observe its order.
11. **D.** The README's two-preset example becomes one preset applied in both
    modes: with two presets of different names only one variant is stored, and
    the paragraph that follows it would not hold. The README's and `lib.rs`'s
    sentences about an `init` after `apply` are corrected to what it does.

## Corrections to the rationale found while specifying

The rationale is corrected in place for these; none changes a decision.

- §4.1, §6: the `xrandr` backlog entry is `docs/todo.md:1570-1578`; Breeze applies
  the menu margins at `breezestyle.cpp:6053-6054`.
- §4.10: kde-breeze states no menu corner radius, so there is no kept value to
  annotate there; the comment goes on the padding and the icon gap only.
- §4.3: the statement that egui's other readers of the handle colour are "not
  worse" is too kind for one of them; see the first open point below.
- §4.6: the deferred re-write is not changed (decision 10).
- §4.1, §9: where the evidence goes (decisions 6 and 7).

## Open points for the maintainer

1. **egui sliders outside the connector's roles (Part B, problem 1).** egui has
   no colour slot of its own for a scroll bar handle, so the connector's base
   style feeds `scrollbar.thumb_color` into `widgets.inactive.bg_fill`, which an
   unscoped slider's rail and handle also read. With Breeze's handle grey that
   rail drops from 3.7:1 to 1.4:1 against the window in light mode (6.6:1 to
   1.7:1 in dark). A slider inside the connector's Slider role is unaffected.
   The specification keeps the rationale's decision and adds a sentence to the
   egui README; the alternative is to hold `scrollbar.thumb_color` back until
   egui's base style no longer shares that slot.
2. **Two small choices in Part D:** whether the gpui showcase's private colour
   conversion should call the new `to_hsla`, and whether the README's *Core
   concepts* lists it beside `layer_over`. The specification does neither.
3. **Part D's exactness test** walks all 2^24 opaque colours once. Its run time
   is estimated at 0.5 to 2 s per test run in this workspace's profile, not
   measured; no smaller test proves the same. The implementer measures it: up
   to 2 s it stays an ordinary test; above that it is marked `#[ignore]` and
   runs in the full `check_release.sh` only, beside the core's existing ignored
   test, and the maintainer is told the measured time.
4. **The asset run (Part E)** needs the maintainer's desktop unlocked and
   untouched for 4 to 6 minutes, once, near the end.

## Part A — Font DPI

Implements `docs/issues/0004_kde-hand-overrides-rationale.md` §4.1 and its font
DPI line in §4.10 (the CHANGELOG entry) in the code and in the code's
documentation. Worktree `.claude/worktrees/v071`, branch `v0.7.1-rc2` at
`387fc449`. Every line number below is at that commit.

**Owned by Part C, not here** (ownership ruling 1): the facts rows
`docs/platform-facts.md:717-718` and `:1127`, every edit of `docs/todo.md` and
`docs/archive/todo_closed.md` (the two closings, the corrections at `:39-41` and
`:1101-1102`, the `[1.0]` entry of rationale §6 (5)),
`docs/issues/0004_kde-union-rationale.md`, `docs/evidence/` (the X server
re-run after the fix) and `ROADMAP.md`.

**Shared file.** `native-theme/src/kde/mod.rs` is also edited by Part B, whose
changes are found by test name, not by line: this part removes about 60 lines
above them (A22, A25).

**The value and its source.** The font DPI is 72.0 on macOS and 96.0 everywhere
else, every Linux desktop included (rationale §4.1, decision A). The toolkit's
scale factor already carries the display scale: gpui on Wayland from the
compositor (gpui-pre-linux 0.3.12 `src/linux/wayland/window.rs:1218-1222`), on
X11 from `Xft.dpi` / 96 (`src/linux/x11/client.rs:2674-2681`, read in the
registry copy); winit likewise (rationale §4.1). Qt lays a point out at a logical
DPI of 96 on Wayland and X11 (rationale §4.1, qtbase v6.10.1). The parked 0.8.0
work decided the same value for KDE: `docs/facts/kde/tokens.toml:7964-7972` on
`v0.8.0-rc1` (`78ee0efd`), key `token.text.font_dpi`, `value = 96`, citing
`qtbase@6.12.0 src/plugins/platforms/wayland/qwaylandscreen.cpp:150-160`
(`return QDpi(96, 96);` unless `QT_WAYLAND_FORCE_DPI` is set).

**Names this part chooses** (the rationale says "one constant" and names none;
the choice does not change any result): the constant is `LINUX_FONT_DPI` in
`detect.rs`; the KDE reader uses it directly, as the GNOME reader does.

### Changes

#### `native-theme/src/detect.rs`

A1. `SUBPROCESS_TIMEOUT`'s rustdoc, lines 13-14: reworded, still two lines (the
egui connector cites `detect.rs:211` and `:230`, and `fonts.rs:340` cites `:20`;
every edit above line 454 keeps its line count):
`/// Timeout for subprocess commands ([`run_with_timeout`]: gsettings,` /
`/// xfconf-query, fc-match, macOS `defaults`).`

A2. `refine_linux_desktop`'s rustdoc, lines 123-124: reworded, still two lines:
`/// else `Unknown`. The reader, the theme watcher and the icon-theme` /
`/// detection all decide by it ([`DetectionContext::refined_linux_desktop`]),`
(line 125 `/// so they agree. ...` stays).

A3. `invalidate_caches`' rustdoc, lines 215-220: reworded, still six lines:

```
/// Reset all process-wide caches so the next call to [`system_is_dark()`],
/// [`prefers_reduced_motion()`], [`accessibility_preferences()`], or
/// [`system_icon_theme()`] re-queries the OS; a cached failure to detect the
/// icon theme is cleared too. The font DPI is not cached: since 0.7.1 it is a
/// constant per platform, which
/// [`ResolutionContext::from_system()`](crate::ResolutionContext::from_system) uses.
```

A4. Removed: `read_xft_dpi` (454-471), `detect_physical_dpi` (473-486),
`parse_xrandr_dpi` (488-536), test module `xrandr_dpi_tests` (538-593).

A5. Replaced: `GNOME_FONT_DPI` (595-606) and `desktop_font_dpi` (608-615) by one
constant, same place:

```rust
#[cfg(target_os = "linux")]
pub(crate) const LINUX_FONT_DPI: f32 = 96.0;
```

Its rustdoc (exact):

```
/// The font DPI on every Linux desktop: 96. The toolkit's scale factor
/// already carries the display's scale: gpui and winit take it from the
/// compositor on Wayland (gpui-pre-linux 0.3.12
/// `src/linux/wayland/window.rs:1218-1222`; winit 0.30.13
/// `src/platform_impl/linux/wayland/types/wp_fractional_scaling.rs:71-72`) and
/// from `Xft.dpi` / 96 on X11 (gpui-pre-linux `src/linux/x11/client.rs:2674-2681`;
/// winit `src/platform_impl/linux/x11/util/randr.rs:40-53`, `:145-151`), and
/// Plasma writes `Xft.dpi` on Wayland as the largest output's scale × 96, so a
/// font DPI taken from `Xft.dpi` scaled text a second time. Qt lays a point
/// out at a logical DPI of 96 on both (qtbase v6.10.1 `qwaylandscreen.cpp:149-160`,
/// `qhighdpiscaling.cpp:241-256`, `:362-383`); GNOME at a fixed 96
/// (`DPI_FALLBACK`, gnome-settings-daemon 51.0
/// `plugins/xsettings/gsd-xsettings-manager.c:243`, `:583-595`; GTK 4.24.1
/// `gdk/wayland/gdksettings-wayland.c:166-168`), its text-scaling factor
/// carried in
/// [`AccessibilityPreferences::text_scaling_factor`](crate::AccessibilityPreferences::text_scaling_factor)
/// on Wayland and in the toolkit's `Xft.dpi` scale factor on X11
/// (`docs/issues/0004_kde-hand-overrides-rationale.md` §4.1).
```

A6. Replaced: `detect_system_font_dpi` (617-677). Signature unchanged,
`fn detect_system_font_dpi() -> f32`, private, `#[allow(unreachable_code)]`
kept. Behaviour: `return 72.0` under `cfg(target_os = "macos")`;
`return crate::windows::LOGICAL_DPI as f32` under
`cfg(all(target_os = "windows", feature = "windows"))`; `return LINUX_FONT_DPI`
under `cfg(target_os = "linux")`; else `96.0`. It reads nothing and calls
nothing. Rustdoc (exact; code spans, no intra-doc link to the Linux-only
constant, so non-Linux doc builds stay clean):

```
/// The font DPI of the platform, at which a point size becomes logical pixels.
///
/// Used, through [`system_font_dpi`], by
/// [`ResolutionContext::from_system()`](crate::ResolutionContext::from_system)
/// where no OS reader has provided `font_dpi`. A constant per platform since
/// 0.7.1: nothing is read.
///
/// - **Linux**, every desktop: 96.0 (`LINUX_FONT_DPI`)
/// - **macOS**: 72.0 (Apple coordinate system: 1pt = 1px)
/// - **Windows**: 96.0, the Windows reader's `font_dpi`: a point is 96/72
///   logical (effective) pixels at any display scale
/// - **Other**: 96.0
```

A7. `session_type` (679-683): stays, with its cfg narrowed from
`all(target_os = "linux", any(feature = "kde", feature = "portal"))` to
`all(target_os = "linux", feature = "portal")`. After A8 its only caller is the
GNOME reader (`gnome/mod.rs:720`; the `gnome` module is
`cfg(all(target_os = "linux", feature = "portal"))`, `lib.rs:254-255`); with the
old cfg, `--no-default-features --features kde` leaves it dead and
`scripts/check_features.sh` fails on the warning.

A8. Removed: `reads_force_font_dpi` (685-694), `parse_force_font_dpi`
(696-703), `kde_force_font_dpi` (705-719), test module `force_font_dpi_tests`
(1588-1636).

A9. `DetectionContext`'s rustdoc, line 1152: `/// `linux_desktop`, the font DPI) and`
becomes `/// `linux_desktop`) and`.

A10. Field `font_dpi: CacheSlot<f32>` (1171) and its initialiser in `new()`
(1186): removed.

A11. `refined_linux_desktop`'s rustdoc, 1291-1292: `/// The desktop the reader,
the theme watcher, the icon-theme detection` / `/// and the font DPI decide by
([`refine_linux_desktop`]), cached: ...` becomes
`/// The desktop the reader, the theme watcher and the icon-theme detection` /
`/// decide by ([`refine_linux_desktop`]), cached: on an` (rest of the paragraph
unchanged).

A12. `finish_refined_desktop` (1319-1338): the call `self.invalidate_font_dpi();`
(1333) is removed; its rustdoc's last clause (1322-1323) `the values detected
from that desktop are cleared: the font DPI, the` / `icon theme, dark mode and
reduced motion.` becomes `the values detected from that desktop are cleared:
the` / `icon theme, dark mode and reduced motion.`

A13. `refined_linux_desktop_sync`'s rustdoc (1340-1358): line 1346-1347
`... The reader, the theme watcher, the icon-theme` / `/// detection and the font
DPI decide by it, and the icon enums` becomes `... The reader, the theme
watcher and the icon-theme` / `/// detection decide by it, and the icon enums`;
lines 1355-1356 `/// system read then clears the font DPI, icon theme, dark mode
and` / `/// reduced motion detected from this desktop.` become `/// system read
then clears the icon theme, dark mode and reduced` / `/// motion detected from
this desktop.` Behaviour unchanged (native-theme-build's generated code calls
it, `native-theme-build/src/codegen.rs:280`).

A14. `DetectionContext::font_dpi` (1368-1376): stays `pub(crate) fn
font_dpi(&self) -> f32`, body `detect_system_font_dpi()`. Rustdoc (exact):

```
/// The font DPI: 72.0 on macOS, 96.0 everywhere else, every Linux desktop
/// included (`detect_system_font_dpi`). A constant per platform since 0.7.1:
/// nothing is read and nothing is cached.
```

A15. `font_dpi_with` (1378-1382): removed.

A16. `invalidate_font_dpi` (1404-1407): stays `pub fn invalidate_font_dpi(&self)`
with an empty body, not `#[deprecated]` (rationale §4.1: a deprecation warning
in a patch release fails a caller that denies warnings). Rustdoc (exact):

```
/// Does nothing. Since 0.7.1 the font DPI is a constant per platform (72 on
/// macOS, 96 elsewhere), which nothing caches; the method stays so that code
/// calling it still builds.
```

A17. `invalidate_all` (1417-1426): the call `self.invalidate_font_dpi();` (1425)
is removed. Rustdoc `/// Clear all cached values.` stays.

A18. `system_font_dpi` (1446-1451): stays `pub(crate) fn system_font_dpi() ->
f32`, body `system().font_dpi()`; rustdoc `/// The system font DPI, cached
([`DetectionContext::font_dpi`]).` becomes `/// The platform's font DPI
([`DetectionContext::font_dpi`]).`

A19. Tests in `detection_context_tests`, `gnome_font_dpi_tests`: see §Tests.

#### `native-theme/src/gnome/mod.rs`

A20. Line 572: `/// `:145-146`), as gpui-pre-linux 0.3.10 does from `Xft.dpi``
becomes `... as gpui-pre-linux 0.3.12 does from `Xft.dpi``. The cited lines
(`client.rs:2674-2681`, `:2623-2669`) were read in gpui-pre-linux 0.3.12:
2674-2681 is the `Xft.dpi` branch, 2623 starts the `GPUI_X11_SCALE_FACTOR` read
and 2669 is `DpiMode::NotSet => {}`.

A21. Line 603: `font_dpi: Some(crate::detect::GNOME_FONT_DPI),` becomes
`font_dpi: Some(crate::detect::LINUX_FONT_DPI),`. Value unchanged (96.0).

#### `native-theme/src/kde/mod.rs`

A22. Module doc `# Sources` (12-13): the `kcmfonts` item is removed. `# Sources
not watched` (25-26): the `Xft.dpi` / `xrandr` item is removed. The remaining
items are `kdeglobals`, `kdedefaults/kdeglobals`, `org.freedesktop.appearance
accent-color` (Sources) and `$XDG_CONFIG_DIRS/kdeglobals`, the icon theme's
`index.theme` (not watched). `watch::tests::every_listed_source_has_a_table_row`
reads this list (`watch/mod.rs:689-700`), so A30 must land with it.

A23. `from_kde_ini`'s rustdoc (97-98): `/// with the font DPI and the icon
theme's sizes read from the system.` becomes `/// with the icon theme's sizes
read from the system and the font DPI of every Linux desktop, 96.`

A24. `from_kde_ini` (115-116): the comment `// I/O: the cached chain (kcmfonts
forceFontDPI on X11 -> xrdb -> xrandr -> 96.0)` and
`font_dpi: Some(crate::detect::system_font_dpi()),` become
`// No I/O: the font DPI is a constant (detect::LINUX_FONT_DPI)` and
`font_dpi: Some(crate::detect::LINUX_FONT_DPI),`.

A25. Removed: `read_kcmfonts_key` (129-136) and `kcmfonts_key_in` (138-145),
and the tests `force_font_dpi_is_read_from_kcmfonts` (852-863) and
`kcmfonts_keeps_its_keys_past_a_stray_line` (865-888). `read_kconfig`,
`create_kde_parser`, `TempConfigDir` and `temp_config_dir` stay (the kdeglobals
code and tests use them).

A26. Stays: `from_kde_content_pure`'s rustdoc (44-45) and the test
`a_kdeglobals_force_font_dpi_is_not_the_font_dpi` (1747-1758). Both state that
`forceFontDPI` lives in `kcmfonts` and that a kdeglobals key of that name is no
font DPI; that remains true of Plasma 6.7 and of the reader, and the cited facts
row (`docs/platform-facts.md:717`) keeps saying so (Part C's row, C.1). The fixture constant
`BREEZE_DARK_FULL`'s `forceFontDPI=120` (591) stays.

#### `native-theme/src/watch/kde.rs`

A27. `WATCHED_FILES` (12): `&["kdeglobals", "kcmfonts", "kdedefaults/kdeglobals"]`
becomes `&["kdeglobals", "kdedefaults/kdeglobals"]`. Behaviour: a write to
`~/.config/kcmfonts` no longer fires `on_theme_change` on KDE.

#### `native-theme/src/watch/mod.rs`

A28. `ThemeChangeEvent`'s rustdoc, 83: `/// values (the font DPI, dark mode, the
icon theme) are the cached ones.` becomes `/// values (dark mode, the icon
theme) are the cached ones.`

A29. `on_theme_change`'s rustdoc, 381-382 and 394-395, each pair kept at two
lines (the egui connector cites `watch/mod.rs:391-448`):
381-382 become `/// detection values (dark mode, the icon theme, the accessibility` /
`/// preferences) are cached until`; 394-395 become `/// module lists its
sources): on KDE `kdeglobals` and `kdedefaults/kdeglobals`` /
`/// in the config directory; on GNOME and Budgie the`.

A30. Test table `SOURCE_NOTIFICATIONS`: the row `("kde", "kcmfonts", "kcmfonts"),`
(571) is removed.

#### `native-theme/src/resolve/context.rs`

A31. `ResolutionContext::font_dpi`'s rustdoc, the paragraph at 46-61 (from
`/// [`from_system`](Self::from_system) detects it: 72.0 on macOS, 96.0 on` to
`/// 96.0, macOS's 72.0 and Windows' 96.0.`), is replaced by (exact; the first
paragraph, 41-44, and the `///` at 45 stay):

```
/// [`from_system`](Self::from_system) gives the platform's: 72.0 on macOS,
/// where a point is a pixel, and 96.0 everywhere else, every Linux desktop
/// included. Nothing is detected or cached. On Linux the toolkit's scale
/// factor carries the display's scale (from the compositor on Wayland, from
/// `Xft.dpi` on X11), so the font DPI carries none of it; GNOME's
/// text-scaling factor is carried in
/// [`AccessibilityPreferences`](crate::AccessibilityPreferences). When the OS
/// theme is read ([`SystemTheme::from_system`](crate::SystemTheme::from_system)),
/// the reader's own DPI, the same value, replaces it.
///
/// No font DPI is right where the application, not the toolkit, decides the
/// X11 scale (`GPUI_X11_SCALE_FACTOR`, `WINIT_X11_SCALE_FACTOR`); on X11
/// without `Xft.dpi`, where gpui and winit scale the whole interface by the
/// monitor's size (gpui-fast by `GDK_SCALE` where the session sets it) and Qt
/// does not (text and interface stay in proportion, at the toolkit's size);
/// on Plasma 6.7 X11 with a global scale other than 1
/// and a forced font DPI that departs from it, where Qt takes its scale from
/// `QT_SCREEN_SCALE_FACTORS` and gpui and winit from `Xft.dpi`; and for a
/// toolkit that does not scale by `Xft.dpi` on X11 (none of the three
/// connectors'). Such an application builds its own context for the preset
/// and TOML paths ([`Theme::resolve_with`](crate::Theme::resolve_with),
/// [`ThemeMode::into_resolved`](crate::theme::ThemeMode::into_resolved));
/// [`SystemTheme::from_system`](crate::SystemTheme::from_system) takes no font
/// DPI, so on that path it re-derives its sizes from each resolved font's
/// [`defined_size`](crate::theme::ResolvedFontSpec::defined_size) (the text
/// scale's entries carry no `defined_size`, so their sizes cannot be
/// re-derived).
```

The link paths are the public ones: `ResolvedFontSpec` and `ThemeMode` are
re-exported in `pub mod theme` (`lib.rs:308-318`; the crate-root re-export at
`:389-398` is `pub(crate)`), `Theme` at the root; `defined_size` is at
`model/font.rs:279`. `ResolvedTextScaleEntry` (`model/resolved.rs:39-49`) has
`size`, `weight` and `line_height` only.

A32. `from_system`'s rustdoc line 82: `/// - `crate::detect::system_font_dpi`
for `font_dpi` (private)` stays (the function stays).

A32a. `with_font_dpi`'s rustdoc, 98-99 (two lines kept): `/// [`from_system`](Self::from_system)
with a `font_dpi` already known,` / `/// such as the one a platform reader
detected: no DPI detection runs.` become `/// [`from_system`](Self::from_system)
with a `font_dpi` already known,` / `/// such as the one a platform reader
reported.` (Not a grep hit; nothing is detected any more.)

A32b. Stays: the doc example's comment at 30, `/// // Production: auto-detect
from OS.`: `from_system` still detects the icon theme and the button order's
desktop; only its font DPI became a constant.

#### `native-theme/src/model/mod.rs`

A33. `Theme::resolve`'s rustdoc, 455-457 (three lines, kept at three: the egui
connector cites `model/mod.rs:515`, `:529`, `:689`): `/// The font DPI it
detects (on Linux by `xrdb -query`, and `xrandr`` / `/// where `Xft.dpi` is
unset) is cached after the first call until` /
`/// [`invalidate_caches()`](crate::detect::invalidate_caches). To` become

```
    /// Its font DPI is the platform's constant (72 on macOS, 96 elsewhere);
    /// the icon theme it detects is cached after the first call until
    /// [`invalidate_caches()`](crate::detect::invalidate_caches). To
```

#### `native-theme/src/resolve/mod.rs`

A34. `ThemeMode::into_resolved`'s rustdoc, 231: `///   for OS-detected values
(Linux/Windows ≈ 96 DPI, macOS = 72 DPI,` becomes `///   for the platform's
values (96 DPI on Linux and Windows, 72 on macOS,`. (Not a grep hit; the "≈"
says the value is detected, which it no longer is.)

#### `native-theme/src/pipeline.rs`

A35. Comment 35-36 (two lines kept; `docs/todo.md:1092` cites `pipeline.rs:1250-1255`):
`// may have supplied an authoritative font_dpi (e.g. KDE's, from `kcmfonts`' forceFontDPI);` /
`// when it does, the context takes it and no DPI detection runs again.` become
`// may have supplied its font_dpi (each platform reader does: 96 on Linux and` /
`// Windows, 72 on macOS); when it does, the context takes it.`

#### `native-theme/src/lib.rs`

A36. Crate docs 63-64: line 64 `//! hold the font DPI, dark mode and icon theme
the reader uses, and read the` becomes `//! hold the dark mode and icon theme the
reader uses, and read the` (one line; the egui connector cites `lib.rs:416-418`,
`:448-453`, `:468`, `:714`, `:716`).

A37. Readers table, 105: `| KDE Plasma | `kde` | `kdeglobals`, `kcmfonts`; with
`portal`, the portal's accent colour | `kde-breeze` |` becomes
`| KDE Plasma | `kde` | `kdeglobals`; with `portal`, the portal's accent colour | `kde-breeze` |`.

A38. `AccessibilityPreferences::text_scaling_factor`, 425: `gpui-pre-linux
0.3.10` becomes `gpui-pre-linux 0.3.12` (lines verified as in A20).

A39. `ReaderResult::font_dpi`'s doc (516): `/// Font DPI captured at detection
time (None = auto-detect).` becomes `/// The reader's font DPI (None =
`ResolutionContext::from_system()`'s).` `None` takes `from_system()`'s value
(`pipeline.rs:38-44`); the one `None` producer is the preset-as-reader path
(`pipeline.rs:895`). (Not a grep hit; nothing is detected any more.)

#### `native-theme/README.md`, `native-theme/Cargo.toml`, `native-theme/src/presets/README.md`

A40. `README.md:71`: the KDE row becomes the same text as A37.

A41. `Cargo.toml:25`: `# The KDE Plasma reader: kdeglobals and kcmfonts (Linux).`
becomes `# The KDE Plasma reader: kdeglobals (Linux).`

A42. `presets/README.md:123-132`, the `font_dpi` item, becomes (exact):

```
- **`font_dpi`** -- filled by the platform reader: 96 on every Linux desktop,
  KDE's and GNOME's alike, since the toolkit's scale factor already carries
  the display's scale (GNOME's text-scaling factor goes to
  `text_scaling_factor`, not into the DPI; on a GNOME or Budgie X11 session
  the reader reports a `text_scaling_factor` of 1.0 instead: winit and gpui
  take their X11 scale factor from the `Xft.dpi` gnome-settings-daemon
  writes with the factor in it, so the connectors' text scaling would apply
  it twice, as `GnomeReader`'s doc in `gnome/mod.rs` says), macOS's a fixed
  72, and Windows' a fixed 96, since it reads its fonts at 96 DPI so that
  they resolve to logical pixels
```

#### Connectors

A43. `connectors/native-theme-gpui/README.md:621-623`: `The caches hold the
font` / `DPI, dark mode, the icon theme and the accessibility preferences, so a`
becomes `The caches hold` / `dark mode, the icon theme and the accessibility
preferences, so a` (rationale §4.10; Markdown, reflow allowed).

A44. `connectors/native-theme-iced/README.md:723-724`: `The caches hold the
font DPI, dark mode,` / `the icon theme and the accessibility preferences`
becomes `The caches hold dark mode,` / `the icon theme and the accessibility
preferences`.

A45. `connectors/native-theme-gpui/src/watch.rs:21-22` become `/// the reader's
detection values (dark mode, the icon theme, the` / `/// accessibility
preferences) stay cached until the caches are cleared.`

A46. `connectors/native-theme-iced/src/watch.rs:20-22` become `/// `Spinner`:
the reader's detection values (dark mode, the icon theme,` / `/// the
accessibility preferences) stay cached until the caches are` / `/// cleared.`

A47. `connectors/native-theme-gpui/tests/seams.rs:79-84`, the doc of `resolved`:
its cited range `native-theme/src/resolve/context.rs:62-83` moves with A31 and
its claim "read from the desktop" stops being true. New text (exact):

```
/// Resolve a preset without consulting the machine. `ResolutionContext::for_tests`
/// pins the font DPI at 96, so the point sizes the presets declare convert to
/// the same pixels everywhere; `from_preset` would resolve through
/// `ResolutionContext::from_system`, whose DPI is the host platform's (72 on
/// macOS, 96 elsewhere: `ResolutionContext::font_dpi`) and would make the
/// expected height platform-dependent.
```

A48. `connectors/native-theme-gpui/examples/showcase-gpui/tests.rs:1690`: `as
`ResolutionContext::from_system` reads it.` becomes `as
`ResolutionContext::from_system` gives it.` Lines 1680, 1684 (`detect.rs`,
`detect_system_font_dpi`, which stays and still gives 72 on macOS), 1723, 1736
stay.

#### Scripts

A49. `scripts/README.md:752-753`: `and `rustc` only, none of the desktop readers
`gsettings`, `xrdb`, `xrandr`` / `or `xfconf-query`;` become `and `rustc`
only, none of the desktop readers `gsettings` or` / ``xfconf-query`;`.

A50. `scripts/run_in_runner_env.sh:45-46` (comment only; the PATH is built from
the list at `:289`, which has no `xrdb`): `true, and the toolchain's cargo and
rustc. Not gsettings, xrdb, xrandr or` / `xfconf-query, the desktop readers
native_theme::detect asks.` become `true, and the toolchain's cargo and rustc.
Not gsettings or xfconf-query,` / `the desktop readers native_theme::detect
asks.` (each line keeps its `#   ` prefix).

#### Documents

A51. Owned by Part C (C.1, rows 717 and 718).

A52. Owned by Part C (C.4).

A53. Owned by Part C (C.4 (a) 10).

A54. `CHANGELOG.md`: §D1. `ROADMAP.md`: owned by Part C (C.6).

A55. Owned by Part C (C.5, the X server run before and after the fix).

A56. Not this part's edit: Part C carries it for `docs/todo.md` (C.4 (a) 12) and
Part B for the egui connector's `atlas.rs` (§5 item 7); listed here as the cause.
The removals in `detect.rs` (about 270 lines) and `kde/mod.rs` (about 60) move
line citations elsewhere; each is to be re-pointed, after the edit, to the line
of the item its sentence names (several were already a few lines off before
this change):
- for Part C (`docs/todo.md`): `:1036` (`detect.rs:766-771`, `GTK_THEME` in
  `detect_is_dark_inner`), `:1104` (`detect.rs:1355`,
  `refined_linux_desktop_sync`), `:1118` (`detect.rs:1344-1352`), `:1134`
  (`kde/mod.rs:188`, `SmallDefault`), `:1625` (`detect.rs:905`, `:942`),
  `:1638` (`kde/mod.rs:220`, `kde_icon_base_dirs`), `:1649`
  (`detect.rs:1355-1360`), `:2704` (`kde/mod.rs:42-52`, `detect.rs:674-685`);
  and `docs/todo.md:15`, whose example of how to cite an entry names the
  `[1.0]` *Font DPI* item that C.4 moves out (the review proposes the
  `[upstream]` item "An X11 client through Xwayland" as the new example);
- for Part B (egui connector texts): `connectors/native-theme-egui/src/atlas.rs:754`
  (`:765`, `detect_is_dark_inner`).

#### Grep hits that stay

Grep for `Xft`, `xrandr`, `xrdb`, `forceFontDPI`, `kcmfonts`, `font_dpi`,
`font DPI` in the worktree, outside `docs/archive/`,
`docs/todo_v0.7.1_model-completeness-*.md`, the two issue rationales' own
analysis and `CHANGELOG.md` entries before 0.7.1. Every hit not changed above
stays, for the reason given:

| File (lines) | Why it stays |
|---|---|
| `connectors/native-theme-egui/src/accessors.rs:635`, `:637` | GNOME X11: winit's scale is `Xft.dpi` / 96 with the text-scaling factor in it; unchanged by this fix |
| `connectors/native-theme-gpui/tests/seams.rs:23`, `:34`, `:86`, `:97`, `:103-105` | each preset at its platform's DPI; `detect_system_font_dpi` stays and still gives 72 on macOS |
| `docs/new-os-version-guide.md:146`, `docs/platform-facts.md:299`, `:316` | Windows reader's `font_dpi` of 96; unchanged |
| `docs/platform-facts.md:1127` | corrected by Part C (its decision 4); not Part A's |
| `docs/platform-facts.md:1850` | verification log of the `kcmfonts` key's history (Plasma's facts, not native-theme's reading); still true |
| `docs/todo.md:34-35` | KDE's `XftHintStyle` etc., the rendering-preferences entry (its `:39-41` sentence is Part C's, C.4 (a) 8) |
| `docs/todo.md:3991-3992` | `[upstream]` GNOME Xwayland double text scale; GNOME's 96 is unchanged and the entry stays open |
| `native-theme/src/error.rs:128-190`, `:591`, `:600` | `context.font_dpi` validation path; unchanged |
| `native-theme/src/gnome/mod.rs:567-578` (except 572), `:682`, `:686`, `:1447`, `:1460`, `:1467` | gnome-settings-daemon's `Xft.dpi` and the X11 text scale; GNOME behaviour unchanged |
| `native-theme/src/lib.rs:421`, `:504`, `:517`, `:620-621`, `:1254`, `:1293` | GNOME X11 fact; `ReaderResult`/`SystemTheme` field docs (A39) |
| `native-theme/src/macos.rs:601`, `:614`; `native-theme/src/windows.rs` (all hits) | the macOS 72 and Windows 96 readers; unchanged |
| `native-theme/src/model/font.rs:260`, `model/resolved.rs:42`, `:48`, `model/mod.rs:493`, `:506`, `:521` | conversion at the context's `font_dpi`; unchanged |
| `native-theme/src/pipeline.rs` (other hits) | the `font_dpi` field and its validation tests; unchanged |
| `native-theme/src/presets/documented_sizes.rs:917`, `:1128`, `:1154-1159` | Windows `LOGICAL_DPI` and the gate's 96; unchanged |
| `native-theme/src/resolve/context.rs` (other hits), `resolve/mod.rs:187`, `:241`, `:262`, `resolve/tests.rs` (19 hits), `resolve/validate.rs`, `resolve/validate_helpers.rs` | the context field, `for_tests`' 96, `DEFAULT_FONT_DPI`, validation; unchanged |
| `native-theme/tests/fixtures/kde/{breeze-dark,custom-accent,high-dpi,malformed-values}.ini`, `native-theme/tests/reader_kde.rs:18-243` | `forceFontDPI` as a kdeglobals key the reader ignores; still ignored: `from_kde_content_pure` returns the font DPI it is given (`kde/mod.rs:44-60`), so no assertion there changes, and the tests' citation `docs/platform-facts.md:717` still says the key is `kcmfonts`' |
| `native-theme/tests/preset_loading.rs:313` | a test context's 96 |
| `scripts/check_spec_names.py:131`, `:378` | the parked 0.8.0 spec's `TextTokens.font_dpi`; unrelated |
| `CHANGELOG.md` entries of 0.7.0 and earlier (`:84`, `:100`, `:101`, `:205`-`:217`, `:329`, `:330`, `:987`, …) | history |

### Public API check

Every `pub` item of `native-theme` the change touches stays; none is removed or
changes signature:

| Item | After the change |
|---|---|
| `detect::DetectionContext` (struct, all fields private, `new` is `pub(crate)`) | stays; loses a private field (A10); not constructible outside the crate, so no API change |
| `DetectionContext::invalidate_font_dpi(&self)` | stays, does nothing, documented so (A16); not deprecated |
| `DetectionContext::invalidate_all(&self)` | stays; clears every cache (there is no font DPI cache) |
| `DetectionContext::refined_linux_desktop_sync(&self)` (Linux) | stays, behaviour unchanged; rustdoc A13; used by native-theme-build's generated code |
| `detect::invalidate_caches()` | stays; rustdoc A3 |
| `ResolutionContext::font_dpi` (pub field) | stays; rustdoc A31 |
| `ResolutionContext::from_system()` | stays; returns `font_dpi` 72.0 on macOS, 96.0 everywhere else (it was the chain's value on Linux desktops other than GNOME and Budgie) |
| `Theme::resolve()` | stays; rustdoc A33; resolves at the platform's constant |
| `SystemTheme::from_system()`, `from_system_async()` | stay; on KDE the reader reports 96.0 (it was the chain's value) |
| `watch::on_theme_change()`, `watch::ThemeChangeEvent` | stay; rustdoc A28, A29; on KDE a `kcmfonts` write no longer fires |
| `AccessibilityPreferences::text_scaling_factor` | stays; citation label only (A38) |
| `kde::from_kde_content_pure` (`#[doc(hidden)]`) | unchanged |

Removing the private items leaves no public item without a body or meaning (read
in the code): `invalidate_font_dpi` gets an empty body (it only called
`self.font_dpi.invalidate()`); `invalidate_all` and `finish_refined_desktop`
lose a call; `font_dpi()` (pub(crate)) calls `detect_system_font_dpi()`, which no
longer needs `system()`, `session_type`, `kcmfonts` or a sub-process;
`system_font_dpi()` keeps its two callers' contract (`resolve/context.rs:95`
and, before A24, `kde/mod.rs:116`).

`pub(crate)` items other modules use:

- `detect::GNOME_FONT_DPI`: `gnome/mod.rs:603`, replaced (A21).
- `detect::session_type`: `gnome/mod.rs:720`; stays, cfg narrowed (A7).
- `detect::system_font_dpi`: `resolve/context.rs:95` (stays), `kde/mod.rs:116`
  (replaced by the constant, A24).
- `kde::read_kcmfonts_key`: `detect.rs:660` only; removed with its caller.
- `detect::reads_force_font_dpi`, `parse_force_font_dpi`: `detect.rs` only;
  removed.
- `windows::LOGICAL_DPI`: `detect.rs:646`, `presets/documented_sizes.rs:1158`;
  unchanged.

The connectors (`connectors/`) and `native-theme-build` use none of the
`pub(crate)` items (they cannot). Of the touched public items they use
`detect::invalidate_caches` (docs and showcases),
`ResolutionContext::from_system` (`showcase-gpui/tests.rs:1723`) and
`watch::on_theme_change`; native-theme-build's generated code calls
`detect::system().refined_linux_desktop_sync()`. All keep their signatures.

### Tests

**Removed** (every one tests removed code):

- `detect.rs` `xrandr_dpi_tests`: `primary_4k_display`,
  `standard_1080p_display`, `no_primary_falls_back_to_first_connected`,
  `disconnected_only_returns_none`, `missing_physical_dimensions_returns_none`,
  `zero_mm_returns_none`, `empty_output_returns_none`.
- `detect.rs` `force_font_dpi_tests`: `force_font_dpi_is_a_positive_u32`,
  `only_an_x11_session_reads_force_font_dpi`,
  `a_wayland_session_ignores_force_font_dpi`,
  `an_x11_session_takes_a_valid_kcmfonts_key`.
- `detect.rs`: the module `gnome_font_dpi_tests` (2221-2248) is removed whole,
  with its `use super::*;` and its two tests `gnome_desktops_fix_the_font_dpi`
  and `an_unknown_desktop_with_a_gnome_portal_has_gnomes_font_dpi` (an emptied
  module's unused glob import would fail clippy `-D warnings` with `portal`;
  the refinement the second test also exercised stays covered by
  `watch::tests::an_unknown_desktop_with_a_gnome_portal_is_watched_as_gnome`).
- `detect.rs` `detection_context_tests`: `font_dpi_is_cached_until_invalidate_all`,
  `a_font_dpi_detected_across_an_invalidation_is_not_cached`.
- `kde/mod.rs` `tests`: `force_font_dpi_is_read_from_kcmfonts`,
  `kcmfonts_keeps_its_keys_past_a_stray_line`.

**Changed** (`detect.rs` `detection_context_tests` unless named):

- `filled()` (helper): the line `ctx.font_dpi.get_or_fill(|| 120.0);` (1711) goes.
- `invalidate_all_empties_every_slot`: `font_dpi,` leaves the destructuring
  pattern (1731) and `assert!(font_dpi.get().is_none(), "font_dpi");` (1744)
  goes. The pattern names every field, so a leftover slot does not compile.
- `invalidate_is_dark_empties_only_is_dark`: `assert_eq!(ctx.font_dpi.get(),
  Some(120.0));` (1752) becomes `assert_eq!(ctx.reduced_motion.get(),
  Some(true));`.
- `a_portal_answer_that_changes_the_desktop_clears_what_was_read_from_it`:
  lines 1933 and 1950 go; its doc (1923) `/// A font DPI, icon theme, dark mode
  or reduced motion read before the` becomes `/// An icon theme, dark mode or
  reduced motion read before the`.
- `a_portal_answer_that_keeps_the_desktop_keeps_what_was_read`: it checked the
  font DPI only, so it checks the icon theme instead. Body: `ctx.icon_theme_with(||
  Ok("breeze".to_string()))` is `Ok("breeze")`; `ctx.finish_refined_desktop(...)`
  as now (refined `Unknown` + KDE backend, without-portal `Kde`); then
  `ctx.icon_theme_with(|| Ok("Adwaita".to_string())).ok().as_deref() ==
  Some("breeze")`.
- `watch::tests::every_listed_source_has_a_table_row`: code unchanged; it
  checks the new list (A22) against the new table (A30).
- `watch::tests::the_kde_watcher_observes_every_kde_source` (`watch/mod.rs:739-749`):
  gains the reverse direction, so that A27 is checked. After its loop over
  `table_notifications("kde")` it loops over `kde::WATCHED_FILES` and asserts,
  for each `file`, `table_notifications("kde").any(|n| n == *file)`, with the
  message `"the KDE watcher observes `{file}`, which no KDE source row names"`.
  Its doc becomes `/// The KDE watcher observes the file of every KDE row, and
  no other file.` With A27 and A30 both applied the assertion holds
  (`kdeglobals` and `kdedefaults/kdeglobals` are row notifications); with A30
  alone it fails on `kcmfonts`.

**New.** None reads or sets an environment variable or a file: after the fix
nothing reads `XDG_SESSION_TYPE` or `kcmfonts` for the font DPI, and setting a
variable needs `std::env::set_var`, which is `unsafe` in edition 2024
(`Cargo.toml:18`) and denied by `#![deny(unsafe_code)]` (`lib.rs:175`). The
existing KDE and detect tests isolate the same way: by injecting what they read
(`kdeglobals_files_pure`, `refine_desktop(…, || false)`, a fresh
`DetectionContext::new()`), never through the environment. No helper is needed;
the KDE test uses `from_kde_content` (`kde/mod.rs:121-127`, `#[cfg(test)]`), as
the reader tests around it do.

1. `detect.rs`, new module `#[cfg(all(test, target_os = "linux"))] mod
   linux_font_dpi_tests` (`use super::*;`), test
   `the_font_dpi_is_96_on_every_linux_desktop`, four `assert_eq!`s (not
   `assert!`, which on a constant can trip clippy's `assertions_on_constants`):
   `assert_eq!(LINUX_FONT_DPI, 96.0)`, `assert_eq!(detect_system_font_dpi(),
   96.0)`, `assert_eq!(DetectionContext::new().font_dpi(), 96.0)`,
   `assert_eq!(system_font_dpi(), 96.0)` (the process-wide context is safe to
   use: the call reads nothing).
2. `detect.rs` `detection_context_tests`, test
   `invalidate_font_dpi_clears_nothing` (not named by the rationale; it makes
   A16's "does nothing" mechanical): `let ctx = filled();
   ctx.invalidate_font_dpi();` then `is_dark`, `reduced_motion`,
   `reader_accessibility`, `icon_theme` and, on Linux, `linux_desktop` and
   `refined_linux_desktop` are each still `Some` via `.get()`, and
   `ctx.font_dpi()` equals `detect_system_font_dpi()`.
3. `kde/mod.rs` `tests`, test `the_reader_reports_a_font_dpi_of_96` (returns
   `crate::Result<()>`): `let result = from_kde_content(BREEZE_DARK_FULL)?;`
   asserts `result.font_dpi == Some(96.0)` (the fixture's kdeglobals carries
   `forceFontDPI=120`, which stays ignored); then `let sys =
   crate::pipeline::run_pipeline(result, "kde-breeze")?;` and asserts
   `(sys.dark.defaults.font.size - 10.0 * 96.0 / 72.0).abs() < 0.01` (Noto
   Sans 10 pt at 96 DPI; the fixture is dark, so the reader fills `dark`). The
   GNOME test `text_scale_reaches_the_font_size_once` (`gnome/mod.rs:1454-1464`)
   uses the same `run_pipeline` call, which returns `crate::Result<SystemTheme>`
   (`pipeline.rs:23`), so `?` applies.

The GNOME test `text_scale_reaches_the_font_size_once` (asserts `font_dpi ==
Some(96.0)`, `gnome/mod.rs:1460`) and `tests/reader_kde.rs` stay unchanged and
must still pass.

### Documentation text

#### D1. `CHANGELOG.md`, 0.7.1

The entry goes under 0.7.1's `### Fixed`, which the plan creates (ownership
ruling 4). Entry (exact, one physical line):

```
- **native-theme**: on Linux the font DPI is 96 on every desktop, as it already was on GNOME and Budgie, so the display's scale reaches a font size once, through the toolkit's scale factor. The KDE reader and `ResolutionContext::from_system()` took it from KDE's `forceFontDPI` on X11, then `Xft.dpi`, then the monitor's physical DPI from `xrandr`; but gpui and winit already scale by the compositor's scale on Wayland and by `Xft.dpi` on X11, and Plasma writes `Xft.dpi` on Wayland as the display scale × 96, so at a scale of 1.5 a 10 pt font came out at 30 physical pixels where Qt's is 20. Text becomes smaller on a scaled KDE session and on an X11 session that sets `Xft.dpi` above 96; where `Xft.dpi` is unset (Sway and the like, X11 without a settings daemon) it can become larger or smaller, to the size Qt and GTK draw. native-theme no longer runs `xrdb` or `xrandr` or reads `kcmfonts`, the KDE watcher no longer watches `kcmfonts`, and `DetectionContext::invalidate_font_dpi()` has nothing to clear and does nothing.
```

`ROADMAP.md`: owned by Part C (C.6).

#### D2. `docs/platform-facts.md` rows 717 and 718

Owned by Part C (C.1); Part C merges from this part's former rows the
`qhighdpiscaling.cpp:362-383` citation and the Part A review's link-check table.

#### D3. `docs/todo.md` and `docs/archive/todo_closed.md`

Owned by Part C (C.4; C.4 (a) 8 and 9 for `todo.md:39-41` and `:1101-1102`; the
`[1.0]` entry of rationale §6 (5) is among Part C's new backlog entries).

### Gates

`NT_FEATURES` is `kde,portal,system-icons,material-icons,lucide-icons,watch,svg-rasterize,system-fonts`
(`scripts/check_release.sh:420`). `scripts/check_release.sh --commit` runs all of
these except the docs.rs emulation and the semver checks, which only the full
run has (`:1074`, `:1145`).

| Command (check_release.sh line) | Catches |
|---|---|
| `cargo check -p native-theme --all-targets` (427) | a leftover use of a removed item in code built without features (A4-A6, A9-A10, A15); `native-theme` has no default features, so this leg compiles neither `gnome` (A21) nor `kde` (A25) nor the `kde`/`portal` parts of `detect.rs` (A8) |
| `bash scripts/check_features.sh` (443): native-theme with no features, each feature alone, all features; a warning fails | `session_type` dead under `--features kde` alone (A7); a `LINUX_FONT_DPI` or import left unused in some combination |
| `env RUSTFLAGS="-D warnings" cargo check --target x86_64-pc-windows-gnu`/`x86_64-apple-darwin -p native-theme --all-features` (476-477) and `cargo clippy --target … -p native-theme --all-targets --all-features`/`--no-default-features -- -D warnings` (495-497) | a Linux-only item reached on macOS or Windows (A5, A6); skipped with a warning where the target is not installed, so CI's macOS and Windows legs are the backstop |
| `cargo fmt --all` (518) | formatting |
| `cargo clippy -p native-theme --all-targets -- -D warnings` and `… --features "$NT_FEATURES" -- -D warnings` (527-539) | lints in the new and changed code and tests; with `$NT_FEATURES`, a leftover use of a removed item in `gnome`, `kde` or `detect.rs`'s feature-gated code (A8, A21, A25), and an emptied test module's unused import |
| `bash scripts/check_strict_panic.sh -p native-theme --lib --features "$NT_FEATURES"` and `--no-default-features` (573, 575) | panic-prone code in the library |
| `cargo test -p native-theme` (loop at 665-682) and `cargo test -p native-theme --features "$NT_FEATURES"` (683-684) | the new tests (font DPI 96, KDE reader 96 and 13.33 px, `invalidate_font_dpi` a no-op); `invalidate_all_empties_every_slot` (a leftover slot does not compile); `every_listed_source_has_a_table_row` and `the_kde_watcher_observes_every_kde_source` (A22 and A30 must agree, every row's file must be in `WATCHED_FILES`, and every watched file must be a row's: A27); the GNOME reader's 96 |
| `cargo test -p native-theme-gpui --features native,watch` (loop), `cargo test -p native-theme-iced …` (737-739), `cargo test -p native-theme-egui …` (760-762) | connectors unchanged in code; the seams and showcase tests keep their pinned DPIs |
| `env RUSTDOCFLAGS="-D warnings" cargo doc -p native-theme --no-deps` and `… --document-private-items --features "$NT_FEATURES"` (1000-1006) | a rustdoc link to a removed item (`GNOME_FONT_DPI`, `reads_force_font_dpi`, `read_kcmfonts_key`, `invalidate_font_dpi` in docs), or a public item linking a private one (A31, A3) |
| `env RUSTDOCFLAGS="-D warnings" cargo doc -p native-theme-gpui`/`-iced` `--no-deps --document-private-items` (1009), `… -p native-theme-iced --no-deps --all-features` (1015) | A45, A46 |
| `python3 -I scripts/check_docsrs.py native-theme` (1078, full run) | the docs on docs.rs's targets, where a Linux-only link would break |
| cargo-semver-checks (1129-1160, full run) | a `pub` item removed or changed by mistake (`invalidate_font_dpi` must stay) |
| `cargo test -p native-theme --doc`, inside `cargo test` (the README is a doctest, `lib.rs:179-181`) | A40 only if it broke a code block |

Not covered by any gate:

- The comment and Markdown text: A1-A3, A9, A11-A14, A16-A18 wording, A20,
  A23, A28, A29, A31-A39 wording, A40-A50, D1.
- The line-count preservation that keeps other files' citations valid (A1-A3,
  A29, A32a, A33, A35, A36) and the re-pointing handed over in A56.
- The two stamps, which this part's edits stale: `docs/assets/PROVENANCE.toml`
  (it hashes `native-theme/src/`) and `docs/COMPATIBILITY.toml` (it hashes each
  connector's `src/`, `examples/` and `tests/`, which A43-A48 edit). Because
  `CHANGELOG.md` dates the 0.7.1 entry, each is a hard failure in the full
  `scripts/check_release.sh` run (`:1296-1297`, `:1331-1332`); `--commit` and CI
  check neither. The failures are expected until Part E's asset run renews both
  stamps.

### Problems found

1. **`session_type`'s cfg must narrow** (A7). The rationale says it "stays" and
   names no cfg; kept at `any(kde, portal)` it is dead code under `--features
   kde` alone, and `scripts/check_features.sh` fails on the warning.
2. **Citation drift handed over** (A56): removing about 270 lines from
   `detect.rs` and about 60 from `kde/mod.rs` (4 in A22; about 18 for A25's
   functions and 38 for its tests, blank lines included) moves eight live citations in `docs/todo.md` and one in the
   egui connector, and the move of the `[1.0]` *Font DPI* item stales
   `docs/todo.md:15`'s example. Part C carries the re-pointing in
   `docs/todo.md` (C.4 (a) 12), Part B in `atlas.rs` (§5 item 7). No gate checks them.
3. **The rationale's test "the KDE reader's result carries 96"** is satisfiable
   only as a deterministic in-process test (§Tests, new 3): the reader's font
   DPI no longer depends on the session type or `kcmfonts`, and setting either
   through the environment is `unsafe` and denied. The approved rationale says
   so.
4. **Name and placement choices** not made by the rationale (no result
   changes): the constant `LINUX_FONT_DPI`; the KDE reader using it instead of
   `system_font_dpi()`; `invalidate_all` dropping its no-op call; the extra test
   `invalidate_font_dpi_clears_nothing`. `DetectionContext::font_dpi` and
   `system_font_dpi` stay as two stateless wrappers of `detect_system_font_dpi`
   because the rationale lists `font_dpi`'s rustdoc among the texts reworded,
   not among the items removed; folding them into one function is possible and
   changes no result.

## Part B — KDE values: reader, preset, tests

Scope: the code and preset side of rationale §4.2–§4.5
(`docs/issues/0004_kde-hand-overrides-rationale.md`) for the nine KDE fields
`menu.hover_background`, `menu.hover_text_color`, `menu.separator_color`,
`scrollbar.thumb_color`, `scrollbar.track_color`, `list.hover_background`,
`list.header_background`, `list.header_font.color`, `dialog.icon_size`. It also
covers the comments rationale §4.10 asks for on preset values that stay
although their facts rows change (the focus ring, the menu's padding and icon
gap, the primary button), and the citations of corrected facts rows that Part
C's C.2 hands to this part (ruling 3 of review round 1). Paths are relative to
the repository root of branch `v0.7.1-rc2`; line numbers are those of commit
`387fc449`.

Owned elsewhere: Part C owns `docs/platform-facts.md`,
`docs/inheritance-rules.toml`, `docs/todo.md`, `docs/archive/todo_closed.md`,
`docs/evidence/` and `ROADMAP.md`; Part D owns the gpui connector's
`src/geometry.rs`, `src/lib.rs`, `src/colors.rs` and `README.md`, and §4.6–§4.9.
Part A shares `native-theme/src/kde/mod.rs`; this part names the tests it
changes there by name, not by line.

**What this part needs from Part C.** The facts rows are corrected in place,
one line each, so every line number cited here stays valid. Every new or
reworded comment below was checked against Part C's corrected cells in the
joined snapshot `docs/todo_v0.7.1_kde-hand-overrides-spec.md` (C.1): `docs/platform-facts.md:1108-1110` (§2.1.5 focus ring), `:1188`
(§2.3 `primary_background`), `:1254` (§2.6 `separator_color`), `:1260`
(`row_height`), `:1261` (`border.padding_horizontal`), `:1267`
(`icon_text_gap`), `:1269`, `:1270` (§2.6 hover pair), `:1304`, `:1305` (§2.8
track and thumb), `:1428`, `:1429`, `:1435` (§2.15 header text, header fill,
hover), `:1568` (§2.22 `icon_size`). The "kept" comments name two backlog
entries Part C writes under *KDE: what a Qt application under Breeze shows that
native-theme does not (issue #4)*: "KDE values that wait for a connector design"
and "What Breeze draws that the model has no field for". The
`docs/inheritance-rules.toml` notes are Part C's (C.3); no code reads them
except `per_platform_rules_match_safety_nets`
(`native-theme/src/resolve/inheritance.rs:626`), which compares key sets only.

### 1. KDE reader

Files: `native-theme/src/kde/colors.rs` (the derivations), `kde/metrics.rs`
(one comment, §2.1). `kde/mod.rs` has no code change from this part; its tests
change (§3).

**Convention for missing keys** (stated at `colors.rs:444-445`, "Missing INI keys
result in None fields (no hardcoded fallbacks), so a colour the reader cannot
derive keeps the preset's value"): `get_color` (`:7-10`) returns `None` for a
missing group, key or unparsable value; a derived colour combines its inputs
with `Option::zip`, so any missing input gives `None`; the reader merges over
`kde-breeze`, whose stated value then stands. The one default the reader uses is
`frame_contrast` (`:433-438`): a missing or non-numeric `[KDE] frameContrast`
gives `FRAME_CONTRAST_DEFAULT` 0.2 (KColorScheme's own default). The Header
group's cascade to Window (`:650-651`) is KColorScheme's rule for that group
only; the Button set has no such cascade and none is added.

**No function is ported from KDE's sources.** Every derivation below is one call
of a helper `colors.rs` already has — `translucent` (`:22-25`), `mix`
(`:231-233`), `overlay_colors` (`:413-428`), `frame_contrast` (`:433-438`) —
written from the rationale's description of what Breeze paints (§4.2–§4.4),
not from Breeze's code. Checked for all nine: seven are such a call or a direct
key read; `scrollbar.track_color` and `dialog.icon_size` are constants that the
preset states and the reader does not write.

**Constants.**

- `BLEND_VALUE` (`:12-15`, `f32` 0.3, `Metrics::Blend_Value`) is reused for the
  menu's hover and the list's hover: Breeze sets both alphas from
  `Metrics::Blend_Value` (rationale §4.2; facts rows 1269 and 1435 as Part C
  corrects them, `breezestyle.cpp:6021-6029` and `:4871-4875`). Its doc becomes:
  "Breeze's `Metrics::Blend_Value` (breezemetrics.h:176 at breeze v6.7.5), the
  alpha of the selection colour over a checked indicator's fill
  (breezehelper.cpp:40), of the focus colour over a hovered menu item
  (breezestyle.cpp:6021-6029 at breeze f0b1d75) and of the selection colour over
  a hovered item-view row (:4871-4875); docs/platform-facts.md §2.5, §2.6,
  §2.15." No other new alpha constant.
- New `SCROLLBAR_IDLE_HANDLE_ALPHA: f32 = 0.175`, doc: "The alpha of the window
  text over the window in a Breeze scroll-bar handle at rest, the pointer off
  the bar: the handle colour `WindowText` at 0.5 (breezehelper.cpp:279), times
  0.7 off the bar (breezestyle.cpp:6401-6403), filled at half of that over the
  window, `overlayColors(Window, alphaColor(fg, 0.5))` (breezehelper.cpp:1284,
  :349-355; breezestyle.cpp:6405) at breeze f0b1d75, with animations on, their
  default (breeze.kcfg:45-47); docs/platform-facts.md §2.8". One constant, not a
  product of three, because `overlay_colors` takes one alpha; the product
  0.5 × 0.7 × 0.5 in `f32` gives the same 16-bit alpha, 11469.

**The fields, one by one.**

| Field | Today in the reader | Change (inputs: kdeglobals group / key) |
|---|---|---|
| `menu.hover_background` | not written (resolves from `defaults.selection_background`, `colors.rs:477`) | `view_focus.map(\|focus\| translucent(focus, BLEND_VALUE))`, `view_focus` = `[Colors:View] DecorationFocus` |
| `menu.hover_text_color` | not written (resolves from `defaults.selection_text_color`, `:478`) | `window_fg` (`:462`, `[Colors:Window] ForegroundNormal`) |
| `menu.separator_color` | not written (resolves from `defaults.border.color`) | `window_bg.zip(window_fg).map(\|(window, text)\| mix(window, text, frame_contrast(ini)))`; `window_bg` is `[Colors:Window] BackgroundNormal` (`:622`) |
| `scrollbar.thumb_color` | not written (resolves from `defaults.muted_color`) | `window_bg.zip(window_fg).map(\|(window, text)\| overlay_colors(window, text, SCROLLBAR_IDLE_HANDLE_ALPHA))` |
| `scrollbar.track_color` | not written (resolves from `defaults.background_color`) | not written: the preset states `#00000000`, a constant no scheme moves |
| `list.hover_background` | not written (preset's uncited #93cee9 / #2e4a5e) | `selection_bg.map(\|selection\| translucent(selection, BLEND_VALUE))`; `selection_bg` is `[Colors:Selection] BackgroundNormal` (`:564`) |
| `list.header_background` | `colors.rs:702`, `[Colors:Header] BackgroundNormal` | `button_bg` (`:563`, `[Colors:Button] BackgroundNormal`) |
| `list.header_font.color` | `colors.rs:703-705`, `[Colors:Header] ForegroundNormal` | `button_fg` (`:504`, `[Colors:Button] ForegroundNormal`), written with `get_or_insert_default()` only when `Some`, as today |
| `dialog.icon_size` | not written; `kde/metrics.rs` writes only `breezemetrics.h` constants | not written: 64 is KStyle's `KIconLoader::SizeHuge`, not a `breezemetrics.h` constant and not in kdeglobals; the preset states it |

**`view_focus` is bound once.** `get_color(ini, "Colors:View",
"DecorationFocus")` is read four times today (`:465`, `:474`, `:549`, `:556`).
Bind `let view_focus = get_color(ini, "Colors:View", "DecorationFocus");` next to
`window_fg` (`:462`) and use it at those four places and for the menu hover.
This changes no value.

Placement in `populate_colors` (`colors.rs:461-719`):

- Scroll bar block (`:631-640`): after `thumb_active_color` (`:640`), add the
  thumb, with the comment "A handle at rest, the pointer off the bar, is the
  window text at alpha 0.175 over the window (breezehelper.cpp:279, :1284;
  breezestyle.cpp:6401-6405 at breeze f0b1d75; docs/platform-facts.md §2.8); the
  focused view's handle and the pointer-on-the-bar shade have no field. Breeze
  paints no groove, which kde-breeze states as a transparent track."
- Menu block (`:691-694`): the comment becomes "Menu (docs/platform-facts.md
  §2.6): the text is the window text; a hovered item is the focus colour at
  `Blend_Value` lettered in the window text; the separator is `separatorColor()`,
  the window moved towards its text by the frame contrast
  (breezestyle.cpp:5984, :6021-6029, :6155; breezehelper.cpp:329-332 at breeze
  f0b1d75)". Add the three fields after the font colour.
- List block (`:696-705`): `hover_background` after `alternate_row_background`
  (`:701`); `:702-705` read `button_bg` / `button_fg` with the comment "The
  header is the Button set: Breeze fills a `QHeaderView` section with
  `QPalette::Button` (breezestyle.cpp:6827, :6838-6839 at breeze f0b1d75) and Qt
  letters it `ButtonText` (qcommonstyle.cpp:1659-1660), which Breeze does not
  override; neither reads the Header set (docs/platform-facts.md §2.15)." The
  block's first comment, "KDE-02: list fields (Colors:View is the native source
  for list/table content areas)", gains "; a hovered row is the selection at
  `Blend_Value` (breezestyle.cpp:4871-4875)".
- `populate_colors`' doc (`:447-455`): the list of colours "Plasma computes from
  the scheme" gains "the menu's hover and separator, the list's hover and the
  scroll bar's resting handle"; the sentence on the blends on the accent gains
  the list hover ("…the switch's on track and its outline, and the list's hover)
  come from kdeglobals' `[Colors:Selection] BackgroundNormal`"), and a sentence
  is added: "The menu's hover comes from `[Colors:View] DecorationFocus`, so the
  same holds for it."

**Breeze values the derivations give** (checked; each matches rationale §4.2–§4.4,
§9, and Part C's corrected cells):

| Field | Light | Dark | Check |
|---|---|---|---|
| `menu.hover_background` | `Rgba::new(0x3d, 0xae, 0xe9, 0x4d)` | same | `Rgba::from_f32` rounds 0.3 × 255 = 76.5 up to 77 = 0x4d (`color.rs:114-121`); the existing tab hover, 0.2 → 0x33, is asserted at `colors.rs:884` |
| `menu.hover_text_color` | #232629 | #fcfcfc | the Window set's text |
| `menu.separator_color` | #c6c8c9 | #4c4e51 | same `mix(Window, WindowText, frame_contrast)` as `checkbox.unchecked_border_color`, asserted for both Breeze schemes as (198,200,201) / (76,78,81) at `colors.rs:1024`, `:1029` |
| `scrollbar.thumb_color` | #cbcdcd | #46494b | recomputed with `overlay_colors`' exact integer arithmetic (python3): alpha16 = 11469; every alpha16 in 11440..=11499 gives the same two colours |
| `list.hover_background` | `Rgba::new(0x3d, 0xae, 0xe9, 0x4d)` | same | as the menu |
| `list.header_background` | #fcfcfc | #292c30 | `[Colors:Button] BackgroundNormal` (BreezeLight.colors:31, BreezeDark.colors:32) |
| `list.header_font.color` | #232629 | #fcfcfc | `[Colors:Button] ForegroundNormal` (:39, :40) |

### 2. Preset and the other citations of corrected rows

The only KDE preset is `native-theme/src/presets/kde-breeze.toml` (the `-live`
files were removed in 0.7.0; the presets folder holds no other KDE file).
Comment style: the file's own, `# platform-facts.md:<line> (§x.y): <source>`
on the line(s) above the value, a value citation's first line starting with the
citation (`scripts/check_preset_citations.py` checks that the cited row's
property name is the key's, or a key the comment names with `K not stated:`).
"Insert after N" refers to the current line N. Every comment below says only
what the corrected row says or what the code does.

**Light variant.**

1. `[light.menu]` (`:295-301`). Today: no `hover_background`, `hover_text_color`
   or `separator_color` (they inherit #3daee9, #ffffff, #bcc0bf).

   - Replace `:296` (`# row_height not stated: platform-facts.md:1260 (§2.6), (none), sizes to font`) with

     ```toml
     # row_height not stated: platform-facts.md:1260 (§2.6), Breeze sizes an item to its content with a floor, 29 at
     # Noto Sans 10 pt; the model's row_height is a fixed height and has no floor (docs/todo.md, What Breeze draws that
     # the model has no field for)
     ```

   - Insert above `:299` (`icon_text_gap_px = 8.0`, today covered by the `:297`
     `icon_size` citation):

     ```toml
     # platform-facts.md:1267 (§2.6): kept at 8, the reader's MenuItem_TextLeftMargin, although Breeze puts the text 4 px
     # after a 16 px icon and the 8 is the inset before the icon column; it changes together with border.padding_horizontal
     # (docs/todo.md, KDE values that wait for a connector design)
     ```

   - Insert after `:301` (`disabled_text_color = "#a0a1a3"`):

     ```toml
     # platform-facts.md:1269 (§2.6): [Colors:View] DecorationFocus 61,174,233 (BreezeLight.colors:116, breeze v6.7.5) at
     # alpha Blend_Value 0.3, a layer over the menu (breezestyle.cpp:6021-6029, breezemetrics.h:176 at breeze f0b1d75; measured)
     hover_background = "#3daee94d"
     # platform-facts.md:1270 (§2.6): [Colors:Window] ForegroundNormal 35,38,41 (BreezeLight.colors:137, breeze v6.7.5): a hovered
     # item keeps the window text; only a pressed one is HighlightedText (breezestyle.cpp:6155 at breeze f0b1d75; measured)
     hover_text_color = "#232629"
     # platform-facts.md:1254 (§2.6): separatorColor(), mix(Window #eff0f1, WindowText #232629, frameContrast 0.2)
     # (breezestyle.cpp:5984, breezehelper.cpp:329-332 at breeze f0b1d75; measured)
     separator_color = "#c6c8c9"
     ```

2. `[light.menu.border]` (`:304-307`). Keep `:305` as it is
   (`# platform-facts.md:1261-1262 (§2.6): MenuItem_MarginWidth = 4; MenuItem_MarginHeight = 4`,
   still true of the corrected rows: 1261 names `MenuItem_MarginWidth` 4 as the
   inset of the item's contents, 1262 is unchanged) and add after it the two
   uncited lines

   ```toml
   # Kept although, in a menu without icons or checkable items, Breeze puts the text 16 px from the
   # highlight's edge (4 + 8 + 4); see icon_text_gap.
   ```

   No menu corner-radius comment: kde-breeze states no menu radius (decision
   of review round 1).

3. `[light.scrollbar]` (`:225-234`). Today: no `track_color` (inherits #eff0f1)
   and no `thumb_color` (inherits `muted_color` #707d8a). Insert after `:229`
   (`overlay_mode = false`):

   ```toml
   # platform-facts.md:1304 (§2.8): no fill: Breeze's page controls draw nothing and the scroll area paints the bar's area
   # in its viewport's own colour, Base by a list, Window by a plain area (breezestyle.cpp:1490-1495, :1806-1822 at breeze f0b1d75; measured)
   track_color = "#00000000"
   # platform-facts.md:1305 (§2.8): at rest, the pointer off the bar: WindowText #232629 at 0.5 × 0.7, filled at half of that
   # over Window #eff0f1, so WindowText at 0.175 over Window (breezehelper.cpp:279, :349-355, :1284;
   # breezestyle.cpp:6401-6405 at breeze f0b1d75; measured)
   thumb_color = "#cbcdcd"
   ```

4. `[light.list]` header, `:315-316` today
   `# platform-facts.md:1429 (§2.15): [Colors:Header] BackgroundNormal 222,224,226 (BreezeLight.colors:59, breeze v6.7.5)` /
   `header_background = "#dee0e2"`. Replace with

   ```toml
   # platform-facts.md:1429 (§2.15): [Colors:Button] BackgroundNormal 252,252,252 (BreezeLight.colors:31, breeze v6.7.5):
   # Breeze fills a QHeaderView section with QPalette::Button and never reads the Header set for it (breezestyle.cpp:6827,
   # :6838-6839 at breeze f0b1d75; measured)
   header_background = "#fcfcfc"
   ```

5. `[light.list]` hover, `:320` today `hover_background = "#93cee9"`, uncited
   (`:319` is the `row_height not stated` key citation). Replace `:320` with

   ```toml
   # platform-facts.md:1435 (§2.15): [Colors:Selection] BackgroundNormal 61,174,233 (BreezeLight.colors:87, breeze v6.7.5) at
   # alpha Blend_Value 0.3, a focused view's hovered row (breezestyle.cpp:4871-4875 at breeze f0b1d75; measured)
   hover_background = "#3daee94d"
   ```

   `:321` `hover_text_color = "#232629"` stays (it falls under this citation's
   coverage; row 1435 matches `hover_background`, so the check holds).

6. `[light.list.header_font]`, `:336-337` today
   `# platform-facts.md:1428 (§2.15): [Colors:Header] ForegroundNormal 35,38,41 (BreezeLight.colors:67, breeze v6.7.5)` /
   `color = "#232629"`. Replace the comment (value unchanged):

   ```toml
   # platform-facts.md:1428 (§2.15): [Colors:Button] ForegroundNormal 35,38,41 (BreezeLight.colors:39, breeze v6.7.5):
   # Qt letters a QHeaderView section in QPalette::ButtonText (qcommonstyle.cpp:1659-1660; Breeze does not override
   # CE_HeaderLabel; measured)
   ```

7. `[light.dialog]`, `:456` today `icon_size_px =32.0`, uncited (it falls under
   the `:449` `entrance` citation). Replace with

   ```toml
   # platform-facts.md:1568 (§2.22): PM_MessageBoxIconSize, which Breeze leaves to KStyle: KIconLoader::SizeHuge, 64
   # (frameworkintegration src/kstyle/kstyle.cpp:449-451 at v6.31.0; measured 64); QMessageBox and KMessageBox use it
   icon_size_px = 64.0
   ```

8. Kept: the focus ring. Replace `:53`
   (`# platform-facts.md:1108 (§2.1.5): [Colors:View] DecorationFocus 61,174,233 (BreezeLight.colors:116, breeze v6.7.5)`,
   which the corrected row no longer says) with

   ```toml
   # platform-facts.md:1108 (§2.1.5): Breeze's keyboard focus band is [Colors:Selection] BackgroundNormal at alpha 0.3,
   # #3daee94d; kept at [Colors:View] DecorationFocus 61,174,233 (BreezeLight.colors:116, breeze v6.7.5), opaque, because
   # the connectors draw the ring on every focus and gpui-component's one `ring` colour also tints a focused field's
   # border (docs/todo.md, KDE values that wait for a connector design)
   ```

   and insert above `:64` (`focus_ring_width_px = 1.0`), so that the `:62`
   `line_height` citation no longer covers the two:

   ```toml
   # platform-facts.md:1109-1110 (§2.1.5): kept at 1 and 2 although Breeze's band is 2 wide and starts 1 px inside the
   # widget's edge, -1; see focus_ring_color
   ```

9. Kept: the primary button. In `[light.button]`, after `:142`
   (`checked_text_color`) add

   ```toml
   # primary_background / primary_text_color not stated: platform-facts.md:1188-1189 (§2.3), Breeze's default button is
   # mix(Button, Highlight, 0.2), #d6ecf8, under ButtonText #232629; they keep inheriting the accent and its text because
   # gpui's primary tokens and egui's selection fill read them as the accent (docs/todo.md, KDE values that wait for a
   # connector design)
   ```

**Dark variant** — the same edits with the dark values and BreezeDark lines:

| Where (current line) | Today | New |
|---|---|---|
| `[dark.menu]` `:844` | `# row_height not stated: platform-facts.md:1260 (§2.6), (none), sizes to font` | light item 1's `row_height` comment |
| `[dark.menu]` above `:847` `icon_text_gap_px = 8.0` | covered by `:845` | light item 1's `icon_text_gap` comment |
| `[dark.menu]` insert after `:849` | nothing stated | `hover_background = "#3daee94d"` (cite `:1269`, `[Colors:View] DecorationFocus 61,174,233 (BreezeDark.colors:117, breeze v6.7.5)`); `hover_text_color = "#fcfcfc"` (cite `:1270`, `[Colors:Window] ForegroundNormal 252,252,252 (BreezeDark.colors:138, breeze v6.7.5)`); `separator_color = "#4c4e51"` (cite `:1254`, `mix(Window #202326, WindowText #fcfcfc, frameContrast 0.2)`), with light item 1's wording otherwise |
| `[dark.menu.border]` after `:853` | — | light item 2's "Kept although…" line |
| `[dark.scrollbar]` insert after `:776` | nothing stated (inherits #202326, #a1a9b1) | `track_color = "#00000000"` (cite `:1304`); `thumb_color = "#46494b"` (cite `:1305`, "WindowText #fcfcfc … over Window #202326") |
| `[dark.list]` `:863-864` | `[Colors:Header] BackgroundNormal 41,44,48 (BreezeDark.colors:60…)`, `#292c30` | light item 4's comment with `[Colors:Button] BackgroundNormal 41,44,48 (BreezeDark.colors:32, breeze v6.7.5)`; value stays `#292c30` |
| `[dark.list]` `:868` | `hover_background = "#2e4a5e"`, uncited | light item 5's comment with `(BreezeDark.colors:88, breeze v6.7.5)`; `hover_background = "#3daee94d"` |
| `[dark.list.header_font]` `:884` | `[Colors:Header] ForegroundNormal 252,252,252 (BreezeDark.colors:68…)` | light item 6's comment with `[Colors:Button] ForegroundNormal 252,252,252 (BreezeDark.colors:40, breeze v6.7.5)`; value stays `#fcfcfc` |
| `[dark.dialog]` `:1004` | `icon_size_px =32.0` | light item 7's comment, `icon_size_px = 64.0` |
| `[dark.defaults]` `:598`, above `:609` | `:598` cites `DecorationFocus … (BreezeDark.colors:117…)` | light item 8's two comments, with `(BreezeDark.colors:117, breeze v6.7.5)` |
| `[dark.button]` after `:687` | — | light item 9's comment with "#2d4655, under ButtonText #fcfcfc" |

**2.1 Other citations of corrected rows (ruling 3, from Part C's C.2).**

1. `native-theme/src/presets.rs:423-425`, today
   "button.primary_background is derived from accent, except on / Windows, where
   WinUI 3 paints AccentFillColorDefault instead / (docs/platform-facts.md:1188,
   §2.3)". New comment, three lines as today, so no later line of
   `presets.rs` moves (assertion unchanged):

   ```rust
                // button.primary_background is derived from accent, except on Windows
                // (AccentFillColorDefault); KDE's default button is mix(Button, Highlight, 0.2)
                // and waits for the connectors (docs/platform-facts.md:1188, §2.3)
   ```

2. `native-theme/src/presets.rs:469-472`: §3 (with the test it belongs to).

3. `native-theme/src/kde/metrics.rs:78`, today
   `// platform-facts.md:1261-1262 (§2.6)`, and `:83`,
   `variant.menu.icon_text_gap = Some(8.0); // MenuItem_TextLeftMargin`. New
   text (values unchanged). `:78` becomes the six lines

   ```rust
       // platform-facts.md:1261-1262 (§2.6): MenuItem_MarginWidth and
       // MenuItem_MarginHeight, 4; kept although, in a menu without icons or
       // checkable items, Breeze starts the text 16 from the highlight (:1261),
       // and puts a 16 px icon's text 4 after the icon (:1267): the padding and
       // the gap change together (docs/todo.md, KDE values that wait for a
       // connector design)
   ```

   and `:83` becomes the three lines (the comment on its own lines above the
   statement, which loses its trailing comment)

   ```rust
       // Kept, see above: MenuItem_TextLeftMargin is the inset before the icon
       // column, not this gap (platform-facts.md:1267)
       variant.menu.icon_text_gap = Some(8.0);
   ```

   No new line is wider than 77 characters (the file's widest today is 83).
   The two edits add seven lines: lines `:79-82` move down by five, every line
   from `:84` on by seven. The only citation of the file by line outside it and
   outside `docs/archive/`, `docs/issues/0004_kde-union-rationale.md:34`
   (`native-theme/src/kde/metrics.rs:1-9`), lies above the edit and stays true.
   `kde_metrics`' tests (`breeze_metrics_*`) read none of these lines and stay.

4. `native-theme/src/presets/documented_sizes.rs:184-185`, the kde-breeze menu
   row, `row(KdeBreeze, "menu", all(4.0), &[1261, 1262], "MenuItem_MarginWidth = 4; MenuItem_MarginHeight = 4")`
   `.with(&[("row_height", None, 1260)])`. Values and cited lines unchanged (the
   gate checks only that each cited line starts with the row's name,
   `:666-684`, which Part C does not change); the note becomes
   `"MenuItem_MarginWidth = 4; MenuItem_MarginHeight = 4; kept: :1261 gives a 16 px text inset in a menu without icons or checkable items and :1260 a floor of 29, which wait (docs/todo.md)"`.
   The note is printed in a failure message after "platform-facts gives"
   (`:598-608`); the "kept" names why the expected value is not the row's.

5. `native-theme/src/model/widgets/mod.rs:266-268`, `MenuTheme::row_height`'s
   doc, today "Height of a single menu item row. `None` where the platform states
   / no row height: KDE's menu items size to their font / (docs/platform-facts.md
   §2.6)." New:

   ```rust
       /// Height of a single menu item row. `None` where the platform states
       /// no fixed row height: KDE's menu items size to their content with a
       /// floor, which this field cannot carry (docs/platform-facts.md §2.6).
   ```

6. `connectors/native-theme-iced/examples/showcase-iced.rs:15619`, today
   `    // that states no row height (KDE's items size to their font) leaves the`
   (76 characters). Replace that one line with
   `    // that states no row height (KDE's items fit content, with a floor) leaves the`
   (83 characters, still one line). The line
   count does not change, so the citations of later lines of the file
   (`showcase-egui/tests.rs:151`, `:259`, citing `showcase-iced.rs:20316`,
   `:20373`) stay valid.

### 3. Tests that pin today's values or sources

All of these were found by searching `native-theme/src`, `native-theme/tests`,
`connectors/*/src`, `connectors/*/examples`, `connectors/*/tests` for the nine
field names, for today's values (#93cee9, #2e4a5e, #dee0e2, #707d8a, #a1a9b1,
#bcc0bf, #4d545b and their decimal forms), and for `kde-breeze` in the
showcases' and connectors' tests. The connectors' rows, pairs and showcase
claims read the field from the resolved theme, so they follow the value; the
ones that change are in the core.

**Core, must change:**

| Test | Asserts today | Must assert |
|---|---|---|
| `native-theme/src/presets.rs:469-478` | `menu.hover_background.is_some()` exactly for `adwaita` and `windows-11`; comment "the selection on macOS and KDE ([Colors:Selection]), which it inherits; Windows and GNOME paint a hover fill of their own" | `matches!(*name, "adwaita" \| "kde-breeze" \| "windows-11")`; comment, four lines as today (`:469-472`), so the rationale's citation `presets.rs:469-478` stays true: `// menu.hover_background is the selection on macOS, which it` / `// inherits; KDE (DecorationFocus at 0.3, a layer), Windows and` / `// GNOME paint a hover fill of their own (docs/platform-facts.md:1269,` / `// §2.6)`; the `matches!` fits its line (`:475`); the message "stated only where platform-facts does not give it the selection" stays |
| `kde/colors.rs:1334-1338` `test_list_header_background_from_header` | (35,38,41), the fixture's `[Colors:Header]` | rename `test_list_header_background_from_button`; `Some(Rgba::rgb(49, 54, 59))`, the `BREEZE_DARK` fixture's `[Colors:Button] BackgroundNormal` (`:757`) |
| `kde/colors.rs:1340-1347` `test_list_header_font_color_from_header` | (252,252,252) | rename `test_list_header_font_color_from_button`; `Some(Rgba::rgb(239, 240, 241))` (`:759`) |
| `kde/colors.rs:1332` section comment `// === KDE-02: List header from Header group ===` | — | `// === List header from the Button set ===` |
| `kde/mod.rs` `test_list_header_colors` | (35,38,41) / (252,252,252) from `BREEZE_DARK_FULL`'s Header | rename `test_list_header_colors_are_the_button_sets`; (49,54,59) / (239,240,241) (the fixture's `[Colors:Button]`) |
| `kde/mod.rs` `derived_leaves`, `resolved_leaves` | 22 leaves | 29: append, in this order, `menu.hover_background`, `menu.hover_text_color`, `menu.separator_color`, `scrollbar.thumb_color`, `list.hover_background`, `list.header_background`, `list.header_font.color` (mode side: `mode.list.header_font.as_ref().and_then(\|f\| f.color)`; resolved side: `Some(theme.menu.hover_background)` etc.; all seven are required `Rgba` in the resolved theme, `model/widgets/mod.rs:265-285`, `:333-337`, `:616`, `:635`, `:646`); doc "The twenty-nine colour leaves the reader derives from, or reads directly from, the colour scheme." |
| `kde/mod.rs` `the_derived_colours_are_the_presets_under_breeze` | 22 leaves equal and stated | passes with §1 and §2 as written (checked value by value against `BREEZE_LIGHT_SCHEME` / `BREEZE_DARK_SCHEME`); no edit |
| `kde/mod.rs` `an_accent_moves_the_on_track_the_pressed_button_and_the_selected_segment` | 5 leaves | append `"list.hover_background"`; rename `an_accent_moves_the_on_track_the_pressed_button_the_selected_segment_and_the_list_hover`; its doc ends "…the pressed button and the selected segment, and the list's hover; the checkbox and the progress bar follow it as before." |
| `kde/mod.rs` `the_button_set_moves_the_button_slider_and_switch_leaves` | 11 leaves | append `"list.header_background", "list.header_font.color"`; rename `the_button_set_moves_the_button_slider_switch_and_header_leaves` |
| `kde/mod.rs` `the_header_set_moves_the_toolbar` (no doc today) | toolbar's two | list unchanged; add the doc `/// The Header set moves the toolbar's fill and edge, and not the list's header, which is the Button set's.` |
| `kde/mod.rs` `the_window_set_moves_the_popover_edge` | with header: popover edge + 2 handle leaves | with header append `"menu.hover_text_color", "menu.separator_color", "scrollbar.thumb_color"`; without header the same three after the five; doc "The Window set moves the popover's edge, the scroll-bar handles and the menu's hover text and separator, and the toolbar only where the scheme has no Header group to take it from." |
| `kde/mod.rs` `the_view_set_moves_the_card_edge` | card edge | unchanged (no new leaf reads the View set's background or text) |
| `kde/mod.rs` `the_view_decorations_move_the_frames_the_tab_hover_and_the_handle` | hover: 4; focus: `input.focus_border_color` | hover unchanged; focus `["input.focus_border_color", "menu.hover_background"]`; doc "…DecorationFocus a focused field's frame and the menu's hover." |
| `kde/mod.rs` `the_frame_contrast_moves_the_edges` | 5 edges | append `"menu.separator_color"` |
| `native-theme/tests/reader_kde.rs:88-89` | `list.header_background == (35,38,41)` with comment "Header (list)" | `Some(Rgba::rgb(49, 54, 59))`, comment "Header (list): the Button set" (`tests/fixtures/kde/breeze-dark.ini:43-46`) |
| `native-theme/tests/reader_kde.rs:231-232` | "Missing Header section" → `header_background.is_none()` | the fixture has `[Colors:Button]` (the test asserts it at `:219`): `assert_eq!(v.list.header_background, Some(Rgba::rgb(49, 54, 59)))`, comment "The header is the Button set, which this fixture has" |
| `kde/colors.rs:1455-1583` `test_full_mapping_completeness` | each listed leaf is `Some` under `BREEZE_DARK` | add `menu.hover_background`, `menu.hover_text_color`, `menu.separator_color`, `scrollbar.thumb_color`, `list.hover_background` (all `Some` under `BREEZE_DARK`, which has every input) |

Not affected (checked): `scheme_colours_reach_the_combo_box_menu_and_popover`
(`kde/mod.rs`, the separator widget, not the menu's);
`test_popover_and_separator_are_inherited` (`colors.rs:1382-1388`);
`the_menu_font_keeps_the_window_text_colour` (`kde/mod.rs`);
`native-theme/tests/platform_facts_xref.rs` (font, radii, accent, icon set
only); `presets/documented_sizes.rs`'s checks (no dialog icon row; the menu
row's note changes, §2.1, its assertions do not); `resolve/tests.rs` (sets the
fields itself or tests macOS presets); `native-theme/tests/proptest_roundtrip.rs`
(random values).

**Connectors and showcases: nothing pins a kde-breeze value or source of the
nine fields.** The rows that name them compare the emitted token with the
resolved field (gpui `contract.rs:76-87`, `:354-358`, `:380-383`, `:392-395`,
`:410-421`, `:572-583`; iced `contract/rows.rs:1529`, `:1537`, `:2616`,
`:1821-1828`; egui `style/base.rs:526`, `style/states.rs:435-436`,
`install_tests.rs:2718-2721`), and the showcase tests that read them take the
value from the installed theme and run under material-3 or adwaita (gpui
`tests.rs:6421-6480`, `:9878-9935`, `:9955-9975`, `:18700-18740`) or splice it
(iced `showcase-iced.rs:23974-24018`, `:26893-27144`; the menu-title tests of
`menu_bar_style`, `:29035`). They keep passing. One doc comment and one test list
change:

- egui `connectors/native-theme-egui/src/style/base.rs:614-619`, the doc of
  `a_clear_overlay_track_leaves_the_slot_to_the_input_background`, says "A solid
  bar's transparent track (windows-11) and an opaque track (kde-breeze) stay in
  the slot." After the change kde-breeze's solid track is transparent too, and
  the test (`:650-662`) has no opaque-track witness left. New text: "A solid
  bar's transparent track (windows-11, kde-breeze) and an opaque one
  (material-3) stay in the slot."; add `("material-3", ColorMode::Light,
  egui::Theme::Light)` to the list at `:650-654` (material-3 states
  `overlay_mode = false` and no track, so it inherits the opaque window colour).

**Contrast pairs, by hand.** WCAG 2 relative luminance of each channel `c`
(0..1): `c/12.92` if `c ≤ 0.04045` else `((c+0.055)/1.055)^2.4`;
`L = 0.2126 R + 0.7152 G + 0.0722 B`; ratio `(L₁+0.05)/(L₂+0.05)`. A translucent
fill is first laid source-over on its surface with straight alpha
`a = 77/255 = 0.302` (`c = c_fill·a + c_surface·(1−a)`), the text then on the
result: this is `pair_ratio` and `over` in gpui (`contract.rs:681-700`,
`:2409-2412`), iced (`contract.rs:874-900`) and egui (`contract.rs:1500-1505`).
Each test compares the emitted ratio with the native ratio of the same fields;
where the connector emits the field as it is (all pairs below), the two sides
are equal by construction, so the pair passes; the ratios show what changes.

| Pair (test) | Mode | Text on fill over surface, after | Composite fill | L text / L fill | After | Before | Result |
|---|---|---|---|---|---|---|---|
| hovered menu row (gpui `contract.rs:1987-1998`, native `menu.hover_*` over `menu.background_color`, emitted `accent_foreground` / `accent` over `popover`; iced `contract/pairs.rs:503-524`, "menu selected label", over `menu.background_color`) | light | #232629 on #3daee94d over #eff0f1 | #b9dcef | 0.0190 / 0.6778 | 10.54 | 2.49 (#ffffff on #3daee9) | pass (equal sides: on KDE `popover` and `menu.background_color` are both the window #eff0f1 / #202326) |
| same | dark | #fcfcfc on #3daee94d over #202326 | #294d61 | 0.9734 / 0.0663 | 8.80 | 2.43 (#fcfcfc on #3daee9) | pass |
| hovered list row (gpui `:2169-2186`, native `list.hover_text_color` on `list.hover_background` over `list.background_color`, emitted `foreground` on `list_hover` over the same; iced "aw selection list row label", `Hovered`, `contract/pairs.rs:874-888`) | light | #232629 on #3daee94d over #ffffff | #c4e7f8 | 0.0190 / 0.7549 | 11.66 | 8.87 (on #93cee9) | pass (`foreground` = `defaults.text_color` #232629 = the stated `list.hover_text_color`) |
| same | dark | #fcfcfc on #3daee94d over #141618 | #204457 | 0.9734 / 0.0513 | 10.11 | 9.07 (on #2e4a5e) | pass |
| table head text (gpui `:2137-2148`, `header_font.color` on `header_background` over the window) | light | #232629 on #fcfcfc over #eff0f1 | #fcfcfc | 0.0190 / 0.9734 | 14.82 | 11.49 (on #dee0e2) | pass |
| same | dark | #fcfcfc on #292c30 over #202326 | #292c30 | 0.9734 / 0.0249 | 13.67 | 13.67 | pass, unchanged |
| scroller at rest on its rail (iced `contract/pairs.rs:576-611`, indicator pairs, native `thumb_color` on `track_color` over `defaults.background_color`; `styles::scrollable` emits the same two, `styles.rs:1389-1460`) | light | #cbcdcd on #00000000 over #eff0f1 | #eff0f1 | 0.6077 / 0.8702 | 1.40 | 3.69 (#707d8a on #eff0f1) | pass (equal sides), below 3:1, printed |
| same | dark | #46494b on transparent over #202326 | #202326 | 0.0658 / 0.0165 | 1.74 | 6.64 (#a1a9b1) | pass, printed |
| scroller hovered / dragged on its rail (same pairs) | light / dark | #96cfed / #2f6888 over the window | window | — | 1.48 / 2.60 | 1.48 / 2.60 | pass, unchanged (a transparent track over the window is the window) |
| egui "A pressed list cell's text on its pressed fill" (`contract.rs:1909-1928`, `asserted: false`) | light / dark | #232629 / #fcfcfc on #3daee94d over #eff0f1 / #202326 | #b9dcef / #294d61 | — | 10.54 / 8.80 | 8.87 / 9.07 | printed only |

The rationale's figures (§4.4: 2.5 → 10.5 and 2.4 → 8.8) agree. Composites here
are in floating point as the tests compute them; the rationale's Qt composites
(#c4e6f8 / #204357 over `Base`) differ by one level in one channel, which no
test compares. No other asserted pair reads the nine fields: gpui's asserted
`PAIRS` (`contract.rs:1919-2297`), iced's style pairs and egui's T12 pairs were
listed and none else names them.

### 4. New tests

All in `kde/colors.rs`'s test module, which has `populate_fixture` (`:787-793`)
and the scheme-string pattern of `breeze_checkbox_groups` (`:1000-1012`). Add a
helper beside it:

```rust
/// The groups the menu, scroll-bar and list colours read, from BreezeLight.colors
/// and BreezeDark.colors at breeze v6.7.5: Window :129/:130, :137/:138; Button
/// :31/:32, :39/:40; Selection :87/:88; View DecorationFocus :116/:117.
fn breeze_menu_list_groups(window: &str, window_fg: &str, button: &str, button_fg: &str) -> String
```

writing `[Colors:Window]` `BackgroundNormal`/`ForegroundNormal`,
`[Colors:Button]` the same, `"[Colors:Selection]\nBackgroundNormal=61,174,233\n"`
and `"[Colors:View]\nDecorationFocus=61,174,233\n"` (each group's key on the line
after its header, as `breeze_checkbox_groups` writes Selection), no `[KDE]`. Light inputs
("239,240,241", "35,38,41", "252,252,252", "35,38,41"); dark ("32,35,38",
"252,252,252", "41,44,48", "252,252,252").

1. `test_menu_hover_and_separator_are_breezes` — both schemes: `menu.hover_background
   == Some(Rgba::new(0x3d, 0xae, 0xe9, 0x4d))`; `menu.hover_text_color` #232629 /
   #fcfcfc; `menu.separator_color` #c6c8c9 / #4c4e51.
2. `test_scrollbar_thumb_is_breezes` — #cbcdcd / #46494b; and
   `scrollbar.track_color.is_none()` (the reader leaves the track to the preset).
3. `test_list_hover_and_header_are_breezes` — `list.hover_background ==
   Some(Rgba::new(0x3d, 0xae, 0xe9, 0x4d))`; header #fcfcfc / #292c30, header text
   #232629 / #fcfcfc.
4. `a_custom_scheme_moves_the_menu_scrollbar_and_list_colours` — in the pattern of
   `a_custom_scheme_reaches_the_pressed_hovered_and_focused_colours`
   (`:894-937`): call `breeze_menu_list_groups("230,220,200", "90,40,20",
   "200,180,150", "60,20,10")` and then, as that test does, replace by key:
   `.replace("[Colors:Selection]\nBackgroundNormal=61,174,233", "[Colors:Selection]\nBackgroundNormal=200,40,90")`
   and `.replace("DecorationFocus=61,174,233", "DecorationFocus=20,200,90")`
   (each pattern occurs once in the helper's output, so the two keys get
   different colours; a bare `.replace("61,174,233", …)` would set both). Assert
   `menu.hover_background == Some(Rgba::new(20, 200, 90, 0x4d))`,
   `menu.hover_text_color == Some(Rgba::rgb(90, 40, 20))`,
   `menu.separator_color == Some(mix(Rgba::rgb(230,220,200), Rgba::rgb(90,40,20), FRAME_CONTRAST_DEFAULT))`,
   `scrollbar.thumb_color == Some(overlay_colors(Rgba::rgb(230,220,200), Rgba::rgb(90,40,20), SCROLLBAR_IDLE_HANDLE_ALPHA))`,
   `list.hover_background == Some(Rgba::new(200, 40, 90, 0x4d))`, header
   (200,180,150) / (60,20,10); and, for each of the seven, `assert_ne!` against
   the Breeze Light read, so a field that ignored its key fails.
5. `test_menu_separator_follows_frame_contrast` — `breeze_menu_list_groups("0,0,0",
   "200,100,0", "252,252,252", "35,38,41")` plus `[KDE] frameContrast` (the
   Button arguments do not matter here; any parsable values do), as
   `test_checkbox_unchecked_border_follows_frame_contrast` (`:1046-1057`):
   `0.5` → (100,50,0); `2` → (200,100,0) "clamped to 1"; `none` → (40,20,0)
   "the 0.2 default".
6. `test_the_header_ignores_the_header_set` — a scheme with `[Colors:Header]
   BackgroundNormal=1,2,3` / `ForegroundNormal=4,5,6` and `[Colors:Button]`
   (7,8,9) / (10,11,12): header (7,8,9) / (10,11,12), toolbar still (1,2,3).
7. `test_missing_groups_leave_the_new_fields_to_the_preset` — only
   `[Colors:Selection] BackgroundNormal=61,174,233`: `list.hover_background` is
   `Some`, and `menu.hover_background`, `menu.hover_text_color`,
   `menu.separator_color`, `scrollbar.thumb_color`, `list.header_background`,
   `list.header_font` are `None`; then only `[Colors:Window]` with
   `BackgroundNormal` (no text): `menu.separator_color` and
   `scrollbar.thumb_color` are `None`.

`kde/mod.rs`'s moved-leaves tests (§3) are the second check that each field
moves with its own set and no other. `dialog.icon_size` gets no reader test (no
reader change); see §7.

### 5. Connector code and documentation that change with the values

Owned by Part D (§1, §3.2, §4, §6): the doc of `geometry::list_item_selected`
(`connectors/native-theme-gpui/src/geometry.rs:1394-1400`), the gpui README's
`scrollbar_gutter` sentence (`README.md:367`; ruling 2 keeps this part's former
wording: "Where the theme states no track (a transparent `scrollbar.track_color`:
`kde-breeze`, `windows-11`), content that runs under the bar shows through it, a
row's hover fill and text or cells that reach the edge among it; Qt ends the
view at the bar, which this gutter reproduces."), and the comment at
`connectors/native-theme-gpui/src/colors.rs:86-88`, which becomes false ("the
platform's menu hover pair, which is the selection colour on KDE and macOS and a
subtle fill elsewhere").

This part's edits:

1. `connectors/native-theme-iced/src/styles.rs:1241-1244`, `menu`'s doc. Today:
   "A selected row is a row highlight, painted over a panel the widget also
   paints, so it is emitted as given rather than composited: the platform
   measured that fill as it shows, not as a layer." The last clause is false for
   kde-breeze (a 0.3 layer) and windows-11. Replace only line `:1244`,
   `/// composited: the platform measured that fill as it shows, not as a layer.`
   (76 characters), with
   `/// composited: a translucent fill (KDE, Windows) lays itself over the panel.`
   (77 characters). The paragraph keeps its four lines, so no later line of
   `styles.rs` moves (`styles.rs:1261`, which Part C's new backlog entry cites,
   stays valid).

2. `connectors/native-theme-gpui/src/variants.rs:11-14`. Today: "…no theme value
   can make either / button hover the way the platform's buttons do: under KDE
   Breeze the first / turns selection blue and the second a grey that is barely
   distinguishable / from the field, while every ordinary button beside them
   takes Breeze's / `DecorationHover`." Both "selection blue" and the last
   clause are false after this change (kde-breeze's `button.hover_background` is
   the button's own fill, `kde-breeze.toml:129-131`; rationale §5.4).

   The four lines stay four lines, because the gpui showcase's claim gate
   `every_colour_claim_is_read_at_the_line_it_cites`
   (`connectors/native-theme-gpui/src/showcase.rs:2819`) reads six claims at
   `variants.rs:62`, `:63` and `:64` (`info/buttons.rs:121`, `:267`, `:273`,
   `:643`, `:649`; `info/inputs.rs:284`), and the egui showcase cites
   `variants.rs:59-65` (`showcase-egui/chrome.rs:497`, ungated). Line `:11` is
   unchanged; lines `:12-14` become (78-80 characters, as the module's other
   lines, which reach 81):

   ```text
   //! turns the menu's hover tint, the focus colour at 30 % (half that when dark),
   //! and the second a grey barely distinguishable from the field, while ordinary
   //! buttons keep their resting fill, as Breeze's do, marking hover by outline.
   ```

   True against the code: the ghost arm fills `tokens.accent`, at
   `opacity(0.5)` in dark mode (gpui-component 0.8.1 `src/button/button.rs:1205-1212`,
   the version and lines the module already cites at `:6`); kde-breeze's
   `button.hover_background` is the button's resting fill; Breeze's hover changes
   only the pen (`kde/colors.rs:520-523`). No other line of `variants.rs` moves.

3. `connectors/native-theme-gpui/examples/showcase-gpui/info/buttons.rs:618`:
   "…which is the menu selection colour, not a button hover" becomes "…which is
   the menu's hover fill (`menu.hover_background`), not a button hover";
   `info/chrome.rs:1459`: "so the palette follows the platform's menu selection
   rather than a list selection" becomes "so the palette follows the platform's
   menu hover rather than a list selection". No test matches either string
   (searched the gpui showcase, its tests and `scripts/`).

4. `connectors/native-theme-egui/README.md:579-583`: "…so its track stays in the
   slot even when the theme states it transparent, as `windows-11` does: there a
   canvas frame and a progress bar outside its role have no fill." becomes "…as
   `windows-11` and `kde-breeze` do: there a canvas frame and a progress bar
   outside its role have no fill." The base.rs doc comment is in §3.

5. `connectors/native-theme-egui/README.md`, a new paragraph inserted after
   `:583` (the last line of item 4's paragraph), preceded by a blank line of
   its own, before the existing blank line `:584` (`## Semver` is `:585`), so
   it stands as a paragraph of its own:

   ```markdown
   egui paints a scroll bar's handle at rest with `visuals.widgets.inactive.bg_fill`,
   so the base style puts `scrollbar.thumb_color` there; egui has no slot of its own
   for it. The same slot fills an unscoped slider's rail and handle, a check box and a
   radio button. Where the handle is pale, as Breeze's grey is (`kde-breeze`, #cbcdcd /
   #46494b), an unscoped slider's rail all but disappears on the window (1.4:1 /
   1.7:1). A slider inside `Role::Slider` takes `slider.track_color`.
   ```

   Checked against the code: the base style writes the slot from
   `scrollbar.thumb_color` (`src/style/base.rs:376-391`); egui 0.36.2 paints the
   scroll handle with the state's `bg_fill` when `foreground_color` is false
   (`containers/scroll_area.rs:1499-1502`; the base style writes it false,
   `mapping.md:600`), the slider's rail with `widget_visuals.inactive.bg_fill`
   (`widgets/slider.rs:775`) and its handle with the state's `bg_fill`
   (`:813-817`), a radio button's circle with `visuals.bg_fill`
   (`widgets/radio_button.rs:94`) and a check box's frame with it
   (`widget_style.rs:181-182`); `Role::Slider` writes `slider.track_color` as
   the rail (`src/style/roles.rs:553`). The ratios are §3's 1.40 and 1.74 against
   the window colours #eff0f1 / #202326.

6. `native-theme/src/kde/colors.rs`'s comments of §1, and the core comments of
   §2.1.

7. `connectors/native-theme-egui/src/atlas.rs:753-754`, the doc of
   `Builder::os_mode` (`impl<'a> Builder<'a>`, `atlas.rs:673`; handed over by Part A, its item A56). It cites
   "(`native-theme/src/detect.rs:211`)" for `system_is_dark` and
   "(`detect_is_dark_inner`, `:765` on Linux)". Both are correct at `387fc449`
   (`detect.rs:211` is `pub fn system_is_dark() -> bool {`, `:765` is
   `fn detect_is_dark_inner() -> bool {`). Part A removes about 270 lines of
   `detect.rs`; after its change lands, re-point each number to the line that
   then holds the named function: `grep -n "pub fn system_is_dark\|fn detect_is_dark_inner" native-theme/src/detect.rs`
   gives both, and the doc's wording stays as it is. This is done after Part A's
   edit, in the same commit or the next; no gate checks it. Part A's font DPI
   rewordings in READMEs and rustdoc (the iced README and `watch.rs`, the gpui
   README `:621-623`) stay Part A's.

No connector code changes: the readers take the field as it is; the showcases'
menu bar titles (gpui `demo.rs:545-575` `MenuTitle`, iced
`showcase-iced.rs:6518-6545` `menu_bar_style`, egui `chrome.rs:340-350`) fill
with `menu.hover_background` under `menu.hover_text_color` and become the tint
under window text, as rationale §4.2 accepts; their tests (§3) read the field.

### 6. CHANGELOG

Four entries for `CHANGELOG.md`'s `## [0.7.1] - 2026-11-13`, one physical line
each as the file's bullets are, in the style of `:279`. The section heading they
go under (`### Fixed`) is the plan's to create (ruling 4).

```markdown
- **native-theme**: kde-breeze and the KDE reader: a hovered menu item is the one Breeze paints, the focus colour at 30 % (`menu.hover_background`, `[Colors:View] DecorationFocus` at alpha 0.3, #3daee94d) under the window text (`menu.hover_text_color`, #232629 / #fcfcfc), where it was the solid selection colour under white; the menu separator is Breeze's `mix(Window, WindowText, frameContrast)`, #c6c8c9 / #4c4e51, where it was the border colour. The reader derives the three from the colour scheme, so an overlay of `defaults.selection_background` or the portal's accent no longer moves them. Every connector's menus and dropdowns change with them (iced's pick list and combo box, gpui-component's `Select` and `Combobox`), and so do two things Breeze paints otherwise, for want of a field of their own: a menu bar's open or hovered title, which Breeze fills solid under white, and, in the gpui connector, the other readers of gpui-component's `accent`: a toggle's hover and checked fill, a ghost button's hover, the command palette's, the code-action menu's and the touch edit menu's selected rows, a calendar's today and range, and inline code.
- **native-theme**: kde-breeze and the KDE reader: a scroll bar's handle at rest is Breeze's, the window text at alpha 0.175 over the window (#cbcdcd / #46494b), where kde-breeze stated none and it fell back to the muted text colour; the track is transparent, as Breeze paints no groove and the view shows beside the handle, where it was the window colour, a band of another colour on every list. In the gpui connector the bar lies over the content, so content under it now shows through; `geometry::scrollbar_gutter` keeps it clear (the README says so). In the egui connector `visuals.extreme_bg_color` turns transparent with the track, as under `windows-11`: a `Frame::canvas` and a `ProgressBar` outside `Role::ProgressBar` lose their fill; the base style's `widgets.inactive.bg_fill`, which unscoped sliders, check boxes and radio buttons also fill with, turns from the muted text colour to the handle's grey, so an unscoped slider's rail is faint on the window (the README says so).
- **native-theme**: kde-breeze and the KDE reader: a hovered list or table row is Breeze's, the selection colour at 30 % (`list.hover_background`, #3daee94d), laid over the row's fill, where it was #93cee9 / #2e4a5e, an opaque colour with no Breeze source; the column header takes the Button set (`[Colors:Button]`), which Breeze paints a header with, where it read the Header set: in Breeze Light its fill turns from #dee0e2 to #fcfcfc, and its text keeps the same colour from its new source.
- **native-theme**: kde-breeze states `dialog.icon_size` 64, the message-box icon KStyle gives Breeze (`PM_MessageBoxIconSize`, `KIconLoader::SizeHuge`), where it stated 32 without a source. No connector draws with it; an application that reads the resolved `dialog.icon_size` gets 64.
```

`geometry::list_item_selected`'s documentation correction is Part D's CHANGELOG
entry (§6 of Part D).

### 7. Gates

From `scripts/check_release.sh` (full run; `--commit` runs all of these except
the stamp checks of the last row):

| Command (check_release line) | Catches |
|---|---|
| `cargo test -p native-theme --features "$NT_FEATURES"` (`:683-684`; `NT_FEATURES` `:420` includes `kde`) | the reader derivations (§4 tests, literal values); the preset against the reader under both Breeze schemes (`the_derived_colours_are_the_presets_under_breeze`); which set moves which leaf (`moved_leaves` tests); `presets.rs:469-478`; `tests/reader_kde.rs`; `documented_sizes.rs` |
| `cargo test -p native-theme` (`:675`) | `presets.rs` tests (the `kde` module's tests are compiled out without the feature) |
| `cargo test -p native-theme-gpui --features native,watch` (`:667`) and `cargo test -p native-theme-gpui --lib --no-default-features` (`:704-705`) | rows equal their fields, the contrast pairs (§3), `every_colour_leaf_reaches_a_sink_or_is_named`; with `native,watch`, `every_colour_claim_is_read_at_the_line_it_cites` (`src/showcase.rs:2819`), which fails if the `variants.rs` edit (§5 item 2) moves lines `:62-64` |
| `cargo test -p native-theme-iced --no-default-features` (`:736-737`) and `cargo test -p native-theme-iced --features iced_aw,watch,native` (`:738-739`) | iced rows and `style_contrast_never_degrades_the_native_pair` |
| `cargo test -p native-theme-egui --no-default-features` and `--all-features` (`:759-762`) | base-style tests (the extended track test, §3), T12, install tests |
| showcase tests in the runner environment (`:835-853`) | the showcases' menu, title and table tests under their presets |
| `python3 scripts/check_preset_citations.py` (`:980`) | each new or edited preset comment cites a row whose property is the key (it does not compare values) |
| clippy `-D warnings` and `scripts/check_strict_panic.sh -p native-theme --lib --features "$NT_FEATURES"` (`:529-538`, `:572`) | panic-prone or unidiomatic reader code |
| `cargo fmt --all` (`:518`), rustdoc builds (`:1002-1020`) | formatting; broken intra-doc links in edited docs |
| full run only: the two stamp checks, `docs/assets/PROVENANCE.toml` (`scripts/update_provenance.sh`) and `docs/COMPATIBILITY.toml` (`scripts/update_compatibility.sh`, which hashes each connector's `Cargo.toml`, `src/`, `examples/` and `tests/`, all of which this part's connector edits touch) | both go stale with this part's edits. While `CHANGELOG.md` dates the 0.7.1 entry, the full `scripts/check_release.sh` fails on each, hard (`:1296`, `:1331`), until Part E's asset run (release close-out) re-stamps them; `--commit` and CI check neither |

**Not covered by any gate:** the preset's `track_color = "#00000000"` (no reader
writes it and every connector test reads it from the theme, so any value
passes); `dialog.icon_size_px = 64.0` (no reader, no `documented_sizes` row, and
the citation check compares keys, not values); the KDE values in the corrected
facts rows against the preset (no test compares §2.6/§2.8/§2.15/§2.22 KDE cells
with kde-breeze); the wording of every comment, doc comment, README line and
CHANGELOG entry above, including the "kept" comments and the egui README
paragraph; the re-pointed line numbers of §5 item 7; the older line
citations of `kde-breeze.toml` elsewhere (egui `atlas.rs:348`, `icons.rs:805`,
`install_tests.rs:2692`, `pipeline.rs:2309-2413`, `capture_window.sh:351`, citing
`:204`, `:410`, `:473`, `:478`; listed by the round-2 review, not re-read here),
already off before this change and moved further
by §2's inserts, which no gate reads and this part does not re-point (not among
its edits); and the derivations' Qt exactness for non-Breeze schemes (the tests
pin the helpers' arithmetic, not Qt's).

### 8. Problems found

1. **Limit of what was verified:** the reader computes the resting handle with
   one 16-bit alpha (0.175), where Breeze sets the colour's alpha in three
   `setAlphaF` steps. For the two Breeze schemes the 8-bit result is the same for
   any alpha16 in 11440..=11499 (computed), and matches rationale §9's
   measurement; for other schemes a one-level difference in a channel was not
   ruled out (no Qt run was possible here).

2. **Already there before this change, outside the nine fields:** kde-breeze
   states `list.hover_text_color` (#232629 / #fcfcfc, `kde-breeze.toml:321`,
   `:869`) and the reader does not write it, so under a colour scheme whose View
   text differs from Breeze's the hovered list row keeps Breeze's text while the
   hovered menu row (now from `[Colors:Window] ForegroundNormal`) follows the
   scheme. Unstated, it would inherit `list.item_font.color`
   (`docs/inheritance-rules.toml:281`), which the reader writes from the View
   set. Not changed here; it is recorded in Part C's new backlog entry 2
   (Part C adds the sentence). Separately, this part's `kde/colors.rs` edits
   move the `colors.rs` line citations at `docs/todo.md:495` (`:18`, `:23`,
   `:39-43`); Part E's sweep (E.2) re-points them.

3. **Test coverage the change removes:** egui's
   `a_clear_overlay_track_leaves_the_slot_to_the_input_background` loses its only
   opaque solid-track witness when kde-breeze's track turns transparent; §3 adds
   material-3 for it. Without that edit the test still passes and checks less.

4. **egui's unscoped sliders.** The rail of an egui slider outside
   `Role::Slider` drops from 3.69:1 to 1.40:1 (light) and from 6.64:1 to 1.74:1
   (dark) against the window. Every egui widget outside a role scope gets the
   base style (`native-theme-egui/README.md:25-28`), so this is the common case.
   The decision keeps the value and documents it (§5 item 5); rationale §4.3
   now records the loss itself (corrected in place, header *Corrections*), so
   this is no longer an open problem, only the note that the README paragraph
   of §5 item 5 carries it to users.

Nothing found that makes a decision of §4.2–§4.5 unimplementable: every
derivation is one call of an existing helper, every Breeze value recomputes to
the rationale's, and every connector pair on the fields keeps passing.

## Part C — Documentation and data files

Scope: what the implementer changes in `docs/platform-facts.md`, `docs/inheritance-rules.toml`, `docs/todo.md` with `docs/archive/todo_closed.md`, the new `docs/evidence/breeze-offscreen/2026-11-13/` and `docs/evidence/breeze-offscreen/<run-date>/` (with `docs/evidence/README.md`), `ROADMAP.md`, and the two font-DPI sentences of `docs/issues/0004_kde-union-rationale.md`; this part owns every edit of these files (review round 1, ownership 1). The Rust code, the presets, `CHANGELOG.md`, the connectors' READMEs and the upstream requests file are the other parts'; where a change here makes a sentence there false, C.2 names the part that owns it. Every value below is from `docs/issues/0004_kde-hand-overrides-rationale.md` ("the rationale"), a source line opened for this part, or the measurement's saved output. Line numbers in the worktree are at `387fc449` (`v0.7.1-rc2`).

**Order of landing.** The facts rows, the evidence folder and the inheritance notes land in one commit with (or before) the presets' new comments, because a preset comment cites a facts line and the facts cell must say what the comment quotes. The `docs/todo.md` edits that cite new preset keys (`kde-breeze.toml` `[light.scrollbar] thumb_color`) land with or after the preset change, because `scripts/check_todo_citations.py` resolves key paths in the tree it runs on. The readings of `docs/evidence/breeze-offscreen/<run-date>/` (C.5 b), row 717's measured clause and the Font DPI closing text (C.4 a 2) land after Part A's commit, because the after-fix reading needs it, and before Part E's asset run.

### C.1 `docs/platform-facts.md`: the KDE cells

**Columns.** The §1.3.2 and §1.3.3 tables are described at their rows. The §1.3.7 table is `Setting | Source | ✓` (header at lines 715-716): the KDE text is column 2 and the check mark column 3. Every Chapter 2 table touched here is `Property | macOS | Windows | KDE | GNOME | Material` (headers at lines 1048, 1106, 1125, 1175, 1251, 1302, 1419, 1546): the KDE cell is column 4. The implementer replaces that one cell and nothing else on the line.

**Form.** Each row stays one line, so no line number in the file moves (every `platform-facts.md:N` citation elsewhere stays valid). No new cell contains a `|`. Sources are linked as the file already links them: Breeze at `github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/...` (`kstyle/` and `colors/`), KColorScheme at `invent.kde.org/frameworks/kcolorscheme/-/blob/27066d471c93629efee8459d9f69490caa92b7c4/...`, Plasma and frameworkintegration at their tags on invent.kde.org, Qt at a qtbase tag on GitHub. The Qt tags are the rationale's (§2): `v6.11.0` for the item view and style code (`qtableview.cpp`, `qtableview_p.h`, `qabstractitemview.cpp`, `qcommonstyle.cpp`), `v6.10.1` for `qmenu.cpp`, `qfocusframe.cpp` and the font and scale code (`qxcbscreen.cpp`, `qhighdpiscaling.cpp`). A measured value cites the recorded output relatively, `evidence/breeze-offscreen/2026-11-13/out_light.txt#L27`, as the file cites the OS probe (`evidence/os-probe/<run>/<image>/probe.toml#L29`); those line numbers hold only if C.5's cleaning replaces text and deletes no line.

**Checked.** Every source line a new cell cites was opened in the downloaded copy (`scratchpad/issue4/`) and says what the cell says: the Breeze files are the pinned commit's (`breezestyle.cpp` and `breezehelper.cpp` are byte-identical to `breeze-a/pin/` and to `breeze-a/v682/`), KColorScheme is `27066d47`'s (the copy the facts file's existing links read), KStyle's `kstyle.cpp` has the line numbering of frameworkintegration v6.31.0 (its `PM_SmallIconSize` case is at 439-441, as the parked 0.8.0 facts cite it at 6.31.0, `docs/facts/kde/button.toml:10472` on `v0.8.0-rc1`). Every measured value was read in `measure/out_light.txt` / `out_dark.txt` at the cited line. Repository paths: the trees downloaded with the sources confirm `kcm/kcm.cpp` (kscreen), `startkde/startplasma-x11.cpp` and `startkde/plasma-setup-xwayland` (plasma-workspace) and `kded/gtkconfig.cpp` (kde-gtk-config); `kstyle/breezetoolsareamanager.cpp` is the path the facts file already links (its `:201-215` at the pin). Not confirmed by a listing: kwin's `src/` (its tree files list only `src/wayland` and `src/xwayland`) and every qtbase path; the downloaded Qt files carry no version stamp, their tags are the rationale's, and the two scratch copies of `qcommonstyle.cpp` differ (`PM_MessageBoxIconSize` at 4798 in `qt/`, at 4818 in `breeze-a/qt/`): the cells cite `qt/`'s numbering, whose `:2300-2311` also match the facts file's existing link at qtbase `ef55f427`. The implementer opens every kwin and Qt link once before committing. Where a link does not hold at its tag, the line the tag's own source gives is written and the difference is reported to the maintainer before the commit; a statement the tag's source does not make is not written (the rule C.5 (b) states for readings).

**Kinds.** "0.7.1 fix": the preset and the reader change with the cell. "Deferred": the cell states the platform's truth only and says nothing about the preset; kde-breeze keeps a value that now disagrees, and its comment (Part B) says so. "Correction": a row the rationale does not list, wrong for the same reasons as rows it does list (the specification's decision 4); nothing else changes with it.

Rows read and left unchanged: 1262, 1263, 1264, 1266 (menu vertical padding and the menu's own border: true), 1306, 1311 (scroll bar hover and pressed: true), 1379 (`sidebar.hover_background`: its source, Kirigami, was not read; condition 1), 1436 (list hover text: true).

#### Line 610: `[Colors:Header]` (§1.3.2 Colors, `[Colors:Header]`) — correction (specification decision 4)

The table is `Section | What it provides | ✓` (lines 602-603). §4.4 found that Breeze never reads the Header set for a `QHeaderView`; what Breeze does take from it is the tools area of a main window. The ✓ cell (the KF 5.71 commit) stays.

Current "What it provides" cell (column 2):

```text
Table/list column header (KF 5.71+)
```

New "What it provides" cell (column 2):

```text
Header areas: Breeze builds its tools area palette from this set, the `Window` and `WindowText` of a main window's menu bar and top toolbars, where the scheme has Header colours ([breezetoolsareamanager.cpp:201-216, :221-228, :232](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezetoolsareamanager.cpp#L201-L216); [breezestyle.cpp:1322, :1341](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L1322)); Breeze paints a `QHeaderView` with the Button set, not this one (§2.15) (KF 5.71+)
```

#### Line 633: Focus ring margin (§1.3.3 Geometry, "Focus ring margin") — correction (specification decision 4)

The table is `Property | Breeze source constant | Value | ✓` (lines 628-629). The value, 2px, is right; the source named in the ✓ cell is not: `breezemetrics.h` has no such constant (its focus-related ones are `CheckBox_FocusMarginWidth = 2`, `:88`, and `Focus_LightenColorValue`, `:178`).

Current ✓ cell (column 4):

```text
✅ breezemetrics.h
```

New ✓ cell (column 4):

```text
✅ [breezestyle.cpp:737-739](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L737-L739) (`pixelMetric` returns a literal 2; `breezemetrics.h` has no constant for it) · **(measured)** 2 ([out_light.txt:173](evidence/breeze-offscreen/2026-11-13/out_light.txt#L173))
```

#### Line 717: Font DPI override (§1.3.7 Accessibility) — 0.7.1 fix (§4.1)

The **Source** cell (column 2 of this three-column table). Kept: every sentence of today's cell that is still true, in its order; added: what kscreen writes, how Qt on X11 uses `Xft.dpi`, the `QT_SCREEN_SCALE_FACTORS` exception, and that native-theme reads neither.

Current Source cell:

```text
Plasma 6.7: `~/.config/kcmfonts` `[General] forceFontDPI`, UInt, default 0 = unset, "Force font DPI on X11" ([fontsaasettingsbase.kcfg:8-11](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/fonts/fontsaasettingsbase.kcfg#L8-L11)); the fonts KCM shows it on X11 only (`visible: Qt.platform.pluginName === "xcb"`, [main.qml:424-426](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/fonts/ui/main.qml#L424-L426)), and krdb hands it to X11 as `Xft.dpi` only off Wayland ([krdb.cpp:328-341, :423-425](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/krdb/krdb.cpp#L328-L341)); the `plasma6.0-remove-dpi-settings` migration deletes `forceFontDPIWayland` ([kcms/fonts/kconf_update/plasma6.0-remove-dpi-settings.cpp:25](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/fonts/kconf_update/plasma6.0-remove-dpi-settings.cpp#L25)); the key is `kcmfonts`' alone, a file name without `rc`, since KConfig joins a name it is given to the config directory as it stands and adds `rc` only to an empty one ([kconfig.cpp:649, :652-678](https://invent.kde.org/frameworks/kconfig/-/blob/v6.31.0/src/core/kconfig.cpp#L652-L678), KConfig v6.31.0): the KCM binds its DPI setting to that file (`<kcfgfile name="kcmfonts" />`, [fontsaasettingsbase.kcfg:6](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/fonts/fontsaasettingsbase.kcfg#L6)) and krdb reads it there ([krdb.cpp:330, :339-340](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/krdb/krdb.cpp#L330-L340)), while the KCM's kdeglobals settings ([fontssettings.kcfg:6](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/fonts/fontssettings.kcfg#L6)) hold the fonts, not the DPI; no plasma-workspace or plasma-integration v6.7.5 source reads `forceFontDPI` from kdeglobals · Plasma 6.8: no `forceFontDPI` — the KCM's DPI setting and its kcfg entry are gone ([c675077b](https://invent.kde.org/plasma/plasma-workspace/-/commit/c675077b8f7371eb65bfa73c20d86a50828b13fc), first in v6.7.90), and so is krdb, its only reader ([fe367323](https://invent.kde.org/plasma/plasma-workspace/-/commit/fe367323b30bfa239243b2164d75faacb1be15e7)); `Xft.dpi` is `kwinrc [Xwayland] Scale` × 96 ([plasma-setup-xwayland/main.cpp:47-58](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.8.1/startkde/plasma-setup-xwayland/main.cpp#L47-L58))
```

New Source cell:

```text
Plasma 6.7: `~/.config/kcmfonts` `[General] forceFontDPI`, UInt, default 0 = unset, "Force font DPI on X11" ([fontsaasettingsbase.kcfg:8-11](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/fonts/fontsaasettingsbase.kcfg#L8-L11)); the fonts KCM shows it on X11 only (`visible: Qt.platform.pluginName === "xcb"`, [main.qml:424-426](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/fonts/ui/main.qml#L424-L426)), and kscreen writes it as the global scale × 96, 0 at scale 1 ([kcm.cpp:421-482](https://invent.kde.org/plasma/kscreen/-/blob/v6.7.5/kcm/kcm.cpp#L421-L482)); krdb hands it to X11 as `Xft.dpi` only off Wayland ([krdb.cpp:328-341, :423-425](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/krdb/krdb.cpp#L328-L341)), and Qt on X11 takes `Xft.dpi` as the screen's DPI and scales the whole interface by it ÷ 96, its logical DPI staying 96 ([qxcbscreen.cpp:372-373, :865-877](https://github.com/qt/qtbase/blob/v6.10.1/src/plugins/platforms/xcb/qxcbscreen.cpp#L372-L373); [qhighdpiscaling.cpp:241-256, :362-383](https://github.com/qt/qtbase/blob/v6.10.1/src/gui/kernel/qhighdpiscaling.cpp#L241-L256), at the default rounding policy, which passes fractions through, [qguiapplication.cpp:142-143](https://github.com/qt/qtbase/blob/v6.10.1/src/gui/kernel/qguiapplication.cpp#L142-L143)), unless `QT_SCREEN_SCALE_FACTORS`, which Plasma exports from the global scale ([startplasma-x11.cpp:47-53](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/startkde/startplasma-x11.cpp#L47-L53); kscreen writes it at every global scale, 1 included, [kcm.cpp:429-436](https://invent.kde.org/plasma/kscreen/-/blob/v6.7.5/kcm/kcm.cpp#L429-L436)), sets the scale, as Qt lets it for a factor other than 1 ([qhighdpiscaling.cpp:556-557, :603-621](https://github.com/qt/qtbase/blob/v6.10.1/src/gui/kernel/qhighdpiscaling.cpp#L556-L557)): a scale of the whole interface, not of its text alone; the `plasma6.0-remove-dpi-settings` migration deletes `forceFontDPIWayland` ([kcms/fonts/kconf_update/plasma6.0-remove-dpi-settings.cpp:25](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/fonts/kconf_update/plasma6.0-remove-dpi-settings.cpp#L25)); the key is `kcmfonts`' alone, a file name without `rc`, since KConfig joins a name it is given to the config directory as it stands and adds `rc` only to an empty one ([kconfig.cpp:649, :652-678](https://invent.kde.org/frameworks/kconfig/-/blob/v6.31.0/src/core/kconfig.cpp#L652-L678), KConfig v6.31.0): the KCM binds its DPI setting to that file (`<kcfgfile name="kcmfonts" />`, [fontsaasettingsbase.kcfg:6](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/fonts/fontsaasettingsbase.kcfg#L6)) and krdb reads it there ([krdb.cpp:330, :339-340](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/krdb/krdb.cpp#L330-L340)), while the KCM's kdeglobals settings ([fontssettings.kcfg:6](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/fonts/fontssettings.kcfg#L6)) hold the fonts, not the DPI; no plasma-workspace or plasma-integration v6.7.5 source reads `forceFontDPI` from kdeglobals · Plasma 6.8: no `forceFontDPI` — the KCM's DPI setting and its kcfg entry are gone ([c675077b](https://invent.kde.org/plasma/plasma-workspace/-/commit/c675077b8f7371eb65bfa73c20d86a50828b13fc), first in v6.7.90), and so is krdb, its only reader ([fe367323](https://invent.kde.org/plasma/plasma-workspace/-/commit/fe367323b30bfa239243b2164d75faacb1be15e7)); `Xft.dpi` is `kwinrc [Xwayland] Scale` × 96 ([plasma-setup-xwayland/main.cpp:47-58](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.8.1/startkde/plasma-setup-xwayland/main.cpp#L47-L58)), the display scale (Scale factor below) · native-theme reads neither `forceFontDPI` nor `Xft.dpi`: since 0.7.1 its font DPI is 96 on every Linux desktop
```

New ✓ cell (column 3); today it reads:

```text
✅ Plasma 6.7.5 (plasma-workspace `cc1ab796`); ✅ Plasma 6.8.1 (`fb9b679a`)
```

New:

```text
✅ Plasma 6.7.5 (plasma-workspace `cc1ab796`, kscreen v6.7.5); ✅ Plasma 6.8.1 (`fb9b679a`); ✅ Qt 6.10.1
```

With the implementation run's Qt output in hand (C.5 b, `docs/evidence/breeze-offscreen/<run-date>/x11/qtfont-out.txt`), insert into the Source cell, directly before " · Plasma 6.8", the text below. `<run-date>` is that folder's name, `<a>`/`<b>` the lines that hold the readings, and the three ratios and the logical DPI are the ones the file prints, under the rule of C.5 (b) for a reading that differs from the rationale's (which expects 1.25, 1.5, 2 and 96).

```text
; **(measured)** on a private X server with `Xft.dpi` 120, 144 and 192, Qt reports a device pixel ratio of <r120>, <r144> and <r192> and a logical DPI of <dpi> ([x11/qtfont-out.txt:<a>-<b>](evidence/breeze-offscreen/<run-date>/x11/qtfont-out.txt#L<a>-L<b>))
```

#### Line 718: Scale factor (§1.3.7 Accessibility) — 0.7.1 fix (§4.1)

The **Source** cell. Today's "Plasma 6.7 on X11: `forceFontDPI / 96`" presents the value as a scale native-theme may apply; it is the whole interface's scale in Qt.

Current Source cell:

```text
Plasma 6.7 on X11: `forceFontDPI / 96` (from `kcmfonts`; 0 = unset) · Plasma 6.7 on Wayland: **(none)** — the KCM hides the DPI setting ("they should always be using the global scaling system instead", [main.qml:424-426](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/fonts/ui/main.qml#L424-L426)); krdb's `Xft.dpi` there is `kwinrc [Xwayland] Scale` × 96 ([krdb.cpp:333-337](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/krdb/krdb.cpp#L333-L337)), the Xwayland scale, not a text scale · Plasma 6.8: **(none)** — no `forceFontDPI` (Font DPI override above)
```

New Source cell:

```text
**(none)** as a text scale. Plasma 6.7 on X11: krdb sets `Xft.dpi` to `forceFontDPI`, 96 where `kcmfonts` has no key, and removes it where the key is 0 ([krdb.cpp:338-341, :423-445](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/krdb/krdb.cpp#L338-L341)), and Qt takes it as the scale of the whole interface (Font DPI override above) · Plasma on Wayland: krdb in 6.7 and plasma-setup-xwayland in 6.8 write `Xft.dpi` as `kwinrc [Xwayland] Scale` × 96, 96 where the key is absent ([krdb.cpp:333-337](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/krdb/krdb.cpp#L333-L337); [plasma-setup-xwayland/main.cpp:47-58](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.8.1/startkde/plasma-setup-xwayland/main.cpp#L47-L58)); kwin writes that key as the largest output's scale while legacy X11 applications scale themselves (`kdeglobals [KScreen] XwaylandClientsScale`, default true) and deletes it otherwise ([workspace.cpp:613-626](https://invent.kde.org/plasma/kwin/-/blob/v6.7.5/src/workspace.cpp#L613-L626), [main.cpp:317-329](https://invent.kde.org/plasma/kwin/-/blob/v6.7.5/src/main.cpp#L317-L329), v6.7.5; [workspace.cpp:647-660](https://invent.kde.org/plasma/kwin/-/blob/v6.8.2/src/workspace.cpp#L647-L660), [main.cpp:281-310](https://invent.kde.org/plasma/kwin/-/blob/v6.8.2/src/main.cpp#L281-L310), v6.8.2): a copy of the display scale for X11 clients, while Qt's own scale there is the output's, at a logical DPI of 96 ([qwaylandscreen.cpp:149-160](https://github.com/qt/qtbase/blob/v6.10.1/src/plugins/platforms/wayland/qwaylandscreen.cpp#L149-L160); [qwaylandwindow.cpp:1517-1522](https://github.com/qt/qtbase/blob/v6.10.1/src/plugins/platforms/wayland/qwaylandwindow.cpp#L1517-L1522)); Plasma 6.7's fonts KCM hides its DPI setting there ("they should always be using the global scaling system instead", [main.qml:424-426](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/fonts/ui/main.qml#L424-L426)) · native-theme reads neither: gpui and winit take the display scale themselves, the compositor's on Wayland and `Xft.dpi` ÷ 96 on X11 ([rationale §4.1](issues/0004_kde-hand-overrides-rationale.md)), where winit would read XSETTINGS `Xft/DPI` first, which Plasma clears ([gtkconfig.cpp:305](https://invent.kde.org/plasma/kde-gtk-config/-/blob/v6.7.5/kded/gtkconfig.cpp#L305), kde-gtk-config v6.7.5; [:347](https://invent.kde.org/plasma/kde-gtk-config/-/blob/v6.8.2/kded/gtkconfig.cpp#L347), v6.8.2)
```

New ✓ cell (column 3); today it reads `✅ Plasma 6.7.5; ✅ Plasma 6.8.1`. New:

```text
✅ Plasma 6.7.5 (plasma-workspace, kwin); ✅ Plasma 6.8.1 (plasma-workspace); ✅ kwin 6.8.2; ✅ kde-gtk-config 6.7.5, 6.8.2; ✅ qtbase 6.10.1
```

#### Line 1061: `selection_inactive_background` (§2.1.3 `defaults.selection_inactive_background`) — deferred (§5.3)

The preset and the reader keep stating nothing; the cell states the platform.

Current KDE cell:

```text
**(none)** — selection bg unchanged on focus loss
```

New KDE cell:

```text
**(KF6 src)** `KColorUtils::tint([Colors:Window] BackgroundNormal, [Colors:Selection] BackgroundNormal, 0.4)` under the Window set's `ForegroundNormal`: where a scheme sets `[ColorEffects:Inactive] ChangeSelectionColor`, as both Breeze schemes do ([BreezeLight.colors:19](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/colors/BreezeLight.colors#L19); [BreezeDark.colors:20](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/colors/BreezeDark.colors#L20)), KColorScheme builds the Inactive group's selection set from the Window set, its background tinted toward the selection colour ([kcolorscheme.cpp:304-321, :374-377](https://invent.kde.org/frameworks/kcolorscheme/-/blob/27066d471c93629efee8459d9f69490caa92b7c4/src/kcolorscheme.cpp#L304-L321)), which becomes the palette's Inactive `Highlight` and `HighlightedText` ([:676-677](https://invent.kde.org/frameworks/kcolorscheme/-/blob/27066d471c93629efee8459d9f69490caa92b7c4/src/kcolorscheme.cpp#L676-L677)); Breeze fills a selected row from that group in a view without the focus ([breezestyle.cpp:4832-4837, :4868](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L4832-L4837); Qt clears `State_Active` there, [qabstractitemview.cpp:3996-3997](https://github.com/qt/qtbase/blob/v6.11.0/src/widgets/itemviews/qabstractitemview.cpp#L3996-L3997)) — Breeze #c2e0f5 under #232629, Breeze Dark #1b4155 under #fcfcfc **(measured)** ([out_light.txt:2, :116, :128](evidence/breeze-offscreen/2026-11-13/out_light.txt#L2), [out_dark.txt:2, :116, :128](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L2))
```

#### Line 1108: `focus_ring_color` (§2.1.5 `defaults.focus_ring_color`) — deferred (§5.1)

kde-breeze keeps #3daee9.

Current KDE cell:

```text
`[Colors:View] DecorationFocus`
```

New KDE cell:

```text
**(Breeze src)** `[Colors:Selection] BackgroundNormal` at alpha `Blend_Value` 0.3: after a focus change from the keyboard (Tab, Backtab, a shortcut) into a line edit, text edit, spin box, combo box, push or tool button, check box, radio button, slider, dial or group box, Breeze puts a `QFocusFrame` on the widget ([breezestyle.cpp:1648-1674](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L1648-L1674)) and fills its band with `QPalette::Highlight` at 0.3 ([:6776-6779](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6776-L6779); [breezemetrics.h:176](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezemetrics.h#L176)) — Breeze and Breeze Dark #3daee94d. A focus from the pointer draws no band; a focused line edit's own 1 px frame turns `focusColor()`, `[Colors:View] DecorationFocus`, on any focus ([breezestyle.cpp:4006-4008](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L4006-L4008); [breezehelper.cpp:157-158](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L157-L158)), and a button's frame turns `Highlight` on keyboard focus ([breezestyle.cpp:4518](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L4518); [breezehelper.cpp:710-712](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L710-L712))
```

#### Line 1109: `focus_ring_width` (§2.1.5 `defaults.focus_ring_width`) — deferred (§5.1)

kde-breeze keeps 1.

Current KDE cell:

```text
Breeze: 1.001px (stroke); 2px margin
```

New KDE cell:

```text
**(Breeze src)** 2: the band is `PM_FocusFrameHMargin` / `PM_FocusFrameVMargin` wide, 2 ([breezestyle.cpp:737-739](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L737-L739)), the space between two rounded rectangles that far apart, which Breeze fills ([:6642-6651](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6642-L6651), [:6776-6779](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6776-L6779)) **(measured)** `PM_FocusFrameHMargin` 2 ([out_light.txt:173](evidence/breeze-offscreen/2026-11-13/out_light.txt#L173), [out_dark.txt:173](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L173)); the widget's own frame line is `PenWidth::Frame`, 1.001 ([breezemetrics.h:25](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezemetrics.h#L25))
```

#### Line 1110: `focus_ring_offset` (§2.1.5 `defaults.focus_ring_offset`) — deferred (§5.1)

kde-breeze keeps 2. The sign convention (negative = inside the edge) is the one `connectors/native-theme-egui/src/plugin.rs:19` cites this row for.

Current KDE cell:

```text
Breeze: 2px margin (outset)
```

New KDE cell:

```text
**(Breeze src)** −1: the `QFocusFrame` is the widget's rectangle grown by the margin, 2, on each side ([qfocusframe.cpp:56-62](https://github.com/qt/qtbase/blob/v6.10.1/src/widgets/widgets/qfocusframe.cpp#L56-L62)), and for a line edit, text edit, spin box, combo box, push or tool button Breeze fills the band from 1 px inside the widget's rectangle to 1 px outside it (`innerRect.adjust(1, 1, -1, -1)`, the outer edge the inner one grown by 2, [breezestyle.cpp:6642-6651](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6642-L6651)); for a check box, a radio button or a slider it rings the indicator or the handle instead ([:6652-6708](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6652-L6708))
```

#### Line 1127: `text_scaling_factor` (§2.1.7 `text_scaling_factor`) — correction (specification decision 4)

Today's cell calls `forceFontDPI` / 96 a text scale; by §4.1's sources it is the scale of Qt's whole interface, with the `QT_SCREEN_SCALE_FACTORS` exception of line 717.

Current KDE cell:

```text
Plasma 6.7 on X11: `forceFontDPI` / 96 (`kcmfonts`, 0 = unset; [krdb.cpp:338-341](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/krdb/krdb.cpp#L338-L341)) · Plasma 6.7 on Wayland and Plasma 6.8: **(none)** — no Plasma setting (Plasma 6.8 dropped the key and its reader, from v6.7.90 on and still at v6.8.1, [c675077b](https://invent.kde.org/plasma/plasma-workspace/-/commit/c675077b8f7371eb65bfa73c20d86a50828b13fc), [fe367323](https://invent.kde.org/plasma/plasma-workspace/-/commit/fe367323b30bfa239243b2164d75faacb1be15e7))
```

New KDE cell:

```text
**(none)** — no Plasma setting scales text alone. Plasma 6.7 on X11: `forceFontDPI` / 96 (`kcmfonts`, 0 = unset; [krdb.cpp:338-341](https://invent.kde.org/plasma/plasma-workspace/-/blob/v6.7.5/kcms/krdb/krdb.cpp#L338-L341)) is `Xft.dpi` ÷ 96, which Qt applies to the whole interface, not to its text alone, unless a `QT_SCREEN_SCALE_FACTORS` factor other than 1 sets Qt's scale instead (§1.3.7) · Plasma 6.7 on Wayland and Plasma 6.8: no Plasma setting (Plasma 6.8 dropped the key and its reader, from v6.7.90 on and still at v6.8.1, [c675077b](https://invent.kde.org/plasma/plasma-workspace/-/commit/c675077b8f7371eb65bfa73c20d86a50828b13fc), [fe367323](https://invent.kde.org/plasma/plasma-workspace/-/commit/fe367323b30bfa239243b2164d75faacb1be15e7))
```

#### Line 1188: `primary_background` (§2.3 `button.primary_background`) — deferred (§5.2)

The preset keeps inheriting the accent.

Current KDE cell:

```text
← `defaults.accent_color`
```

New KDE cell:

```text
**(Breeze src)** the default button of an active window is filled with `KColorUtils::mix(Button, Highlight, 0.2)` in place of `palette.button()` and outlined in `mix(Highlight, mix(Button, ButtonText, frameContrast), 0.5)` ([breezehelper.cpp:700-702](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L700-L702)) — Breeze #d6ecf8 (outline #87c0dd), Breeze Dark #2d4655 (outline #4882a1) **(measured)** ([out_light.txt:158, :159](evidence/breeze-offscreen/2026-11-13/out_light.txt#L158), [out_dark.txt:158, :159](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L158))
```

#### Line 1189: `primary_text_color` (§2.3 `button.primary_text_color`) — deferred (§5.2)

The preset keeps inheriting `accent_text_color`. `←` names a model field, as the file's other KDE cells do (`← font.color`, line 1195).

Current KDE cell:

```text
← `defaults.accent_text_color`
```

New KDE cell:

```text
← `font.color`: the default button's label is `ButtonText`, as every non-flat button's ([breezestyle.cpp:5350-5362](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L5350-L5362)) — Breeze #232629, Breeze Dark #fcfcfc **(measured)** ([out_light.txt:159](evidence/breeze-offscreen/2026-11-13/out_light.txt#L159), [out_dark.txt:159](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L159))
```

#### Line 1253: `background_color` (§2.6 `menu.background_color`) — deferred (§5.5)

The preset keeps inheriting the window colour.

Current KDE cell:

```text
`[Colors:Window] BackgroundNormal`
```

New KDE cell:

```text
**(Breeze src)** a Qt Widgets menu's panel is `frameBackgroundColor()` = `KColorUtils::mix(Window, Base, Blend_Value 0.3)` (`drawPanelMenuPrimitive`, [breezestyle.cpp:4708](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L4708); [breezehelper.cpp:200-203](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L200-L203); [breezemetrics.h:176](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezemetrics.h#L176)) — Breeze #f4f5f5, Breeze Dark #1c1f22 **(measured)** ([out_light.txt:73](evidence/breeze-offscreen/2026-11-13/out_light.txt#L73), [out_dark.txt:73](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L73)), outlined 1 px in `frameOutlineColor()` ([:4709](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L4709)), #c6c8c9 / #4c4e51 ([out_light.txt:74](evidence/breeze-offscreen/2026-11-13/out_light.txt#L74), [out_dark.txt:74](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L74))
```

#### Line 1254: `separator_color` (§2.6 `menu.separator_color`) — 0.7.1 fix (§4.2)

Current KDE cell:

```text
← `defaults.border.color`
```

New KDE cell:

```text
**(Breeze src)** `separatorColor()` = `KColorUtils::mix(Window, WindowText, frameContrast)` ([breezestyle.cpp:5984](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L5984); [breezehelper.cpp:329-332](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L329-L332)) at the default menu opacity, 100 ([breeze.kcfg:181-183](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breeze.kcfg#L181-L183); below 100 it is `WindowText` at alpha frameContrast, [breezestyle.cpp:5981-5982](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L5981-L5982)) — Breeze #c6c8c9, Breeze Dark #4c4e51 **(measured)** ([out_light.txt:88](evidence/breeze-offscreen/2026-11-13/out_light.txt#L88), [out_dark.txt:88](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L88))
```

#### Line 1260: `row_height` (§2.6 `menu.row_height`) — deferred (§4.2, no field for a floor)

kde-breeze keeps stating no row height.

Current KDE cell:

```text
**(none)** — sizes to font
```

New KDE cell:

```text
**(Breeze src)** content-sized with a floor: the larger of the text's height, the 20 px arrow and check box columns (`MenuButton_IndicatorWidth`, `CheckBox_Size`) and the icon column, plus 1, plus `MenuItem_MarginHeight` 4 above and below ([breezestyle.cpp:3663-3667](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L3663-L3667); [breezemetrics.h:57, :84, :87](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezemetrics.h#L57)) — 29 with Noto Sans 10 pt, whose line is 18 **(measured)** ([out_light.txt:66, :150](evidence/breeze-offscreen/2026-11-13/out_light.txt#L66), [out_dark.txt:66, :150](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L66))
```

#### Line 1261: `border.padding_horizontal` (§2.6 `menu.border.padding_horizontal`) — deferred (§4.2, condition 3)

kde-breeze and the reader keep 4. Line 1262 (`padding_vertical`, `MenuItem_MarginHeight` = 4) is true and stays.

Current KDE cell:

```text
`MenuItem_MarginWidth` = 4
```

New KDE cell:

```text
**(Breeze src)** `MenuItem_MarginWidth` 4 insets an item's contents from the left and right edges of its highlight (`MenuItem_MarginHeight` 4 from the top and bottom, [breezestyle.cpp:6053](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6053); [breezemetrics.h:54, :57](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezemetrics.h#L54)); the text starts `MenuItem_TextLeftMargin` 8 further in ([:6054](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6054)) and, in a menu without icons or checkable items (Qt's `menuHasCheckableItems`, set by any checkable action, a radio item's included, [qmenu.cpp:375, :1614](https://github.com/qt/qtbase/blob/v6.10.1/src/widgets/widgets/qmenu.cpp#L375); Breeze lays a 20 px column for them first, [breezestyle.cpp:6059-6065, :6077](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6059-L6065)), `MenuItem_ExtraLeftMargin` 4 further still ([:6101](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6101); [breezemetrics.h:56, :60](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezemetrics.h#L56)): 16 from the highlight's left edge **(measured)** ([out_light.txt:102](evidence/breeze-offscreen/2026-11-13/out_light.txt#L102), [out_dark.txt:102](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L102)), the ink at 17 over the glyph's left bearing of 1
```

#### Line 1265: `border.corner_radius` (§2.6 `menu.border.corner_radius`) — deferred (§4.2, condition 3)

kde-breeze states no menu corner radius today (no `corner_radius_px` under `[light.menu.border]` or `[dark.menu.border]`); see C.7.

Current KDE cell:

```text
**(none)** — items are rectangular
```

New KDE cell:

```text
**(Breeze src)** 4.5: a hovered item's highlight is a rounded rectangle of `frameRadius(PenWidth::Frame)`, `Frame_FrameRadius` 5 less half the pen taken as an `int`, 1 (`renderFocusRect`, [breezestyle.cpp:6049](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6049); [breezehelper.cpp:384, :400](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L384); [breezehelper.h:345-348](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.h#L345-L348); [breezemetrics.h:25, :42](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezemetrics.h#L25))
```

#### Line 1267: `icon_text_gap` (§2.6 `menu.icon_text_gap`) — deferred (§4.2, condition 3)

kde-breeze and the reader keep 8.

Current KDE cell:

```text
8 **(Breeze src)** icon-text gap
```

New KDE cell:

```text
**(Breeze src)** 4 for a 16 px icon: Qt gives a menu's icon column the widest icon + 4, 20 ([qmenu.cpp:355, :378](https://github.com/qt/qtbase/blob/v6.10.1/src/widgets/widgets/qmenu.cpp#L355)), Breeze centres the icon in it and starts the text `MenuItem_ItemSpacing` 2 + 1 after the column's last pixel ([breezestyle.cpp:6096-6099](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6096-L6099); [breezemetrics.h:58](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezemetrics.h#L58)), (20 − 16) / 2 + 2 = 4 **(measured)** ([out_light.txt:85](evidence/breeze-offscreen/2026-11-13/out_light.txt#L85), [out_dark.txt:85](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L85)), 5 blank columns before the ink over the glyph's left bearing of 1; `MenuItem_TextLeftMargin` 8 is the inset before the icon column ([:6054](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6054)), not this gap
```

#### Line 1269: `hover_background` (§2.6 `menu.hover_background`) — 0.7.1 fix (§4.2)

Current KDE cell:

```text
`[Colors:Selection] BackgroundNormal`
```

New KDE cell:

```text
**(Breeze src)** `[Colors:View] DecorationFocus` at alpha `Blend_Value` 0.3: a hovered item is filled with `focusColor()` made translucent by `alphaColor` ([breezestyle.cpp:6021-6029, :6049](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6021-L6029); [breezehelper.cpp:89, :349-355](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L89); [breezemetrics.h:176](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezemetrics.h#L176)) while `MenuItemDrawStrongFocus` is on, its default ([breeze.kcfg:130-132](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breeze.kcfg#L130-L132)), and outlined 1 px in `mix(focusColor, WindowText, 0.15)` ([breezehelper.cpp:172-175](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L172-L175)), #399acc / #5abaec; only a pressed item takes the opaque focus colour ([breezestyle.cpp:6022, :6025-6027](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6022)) — Breeze and Breeze Dark #3daee94d; over the menu panel #bcdff1 / #264a5e **(measured)** ([out_light.txt:75, :79](evidence/breeze-offscreen/2026-11-13/out_light.txt#L75), [out_dark.txt:75, :79](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L75))
```

#### Line 1270: `hover_text_color` (§2.6 `menu.hover_text_color`) — 0.7.1 fix (§4.2)

Current KDE cell:

```text
`[Colors:Selection] ForegroundNormal`
```

New KDE cell:

```text
**(Breeze src)** `[Colors:Window] ForegroundNormal`: a hovered item keeps `WindowText`; only a pressed item is lettered `HighlightedText` ([breezestyle.cpp:6155](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6155)) — Breeze #232629, Breeze Dark #fcfcfc **(measured)** ([out_light.txt:80](evidence/breeze-offscreen/2026-11-13/out_light.txt#L80), [out_dark.txt:80](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L80))
```

#### Line 1304: `track_color` (§2.8 `scrollbar.track_color`) — 0.7.1 fix (§4.3)

Current KDE cell:

```text
← `defaults.background_color`
```

New KDE cell:

```text
**(Breeze src)** no fill, #00000000: the page controls beside the handle draw nothing (`emptyControl`, [breezestyle.cpp:1490-1495](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L1490-L1495)), and the scroll area paints the bar's area in its viewport's own colour (`eventFilterScrollArea`, [:1806-1822](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L1806-L1822)): `Base` behind a list, a table or a text view, `Window` behind a plain scroll area; a 1 px line of `Text` at alpha frameContrast runs along the bar's inner edge ([:8220-8227](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L8220-L8227)) — **(measured)** the bar's area #ffffff / #141618 in a list and #eff0f1 / #202326 in a scroll area, the line #d3d4d4 / #424445 in a list ([out_light.txt:22, :28, :44](evidence/breeze-offscreen/2026-11-13/out_light.txt#L22), [out_dark.txt:22, :28, :44](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L22))
```

#### Line 1305: `thumb_color` (§2.8 `scrollbar.thumb_color`) — 0.7.1 fix (§4.3)

Lines 1306 (`thumb_hover_color`) and 1311 (`thumb_active_color`) are right and stay.

Current KDE cell:

```text
**(Breeze src)** thumb color
```

New KDE cell:

```text
**(Breeze src)** `WindowText` at alpha 0.175 over `Window`: a resting handle's colour is `WindowText` at 0.5 ([breezehelper.cpp:279](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L279)), times 0.7 while the pointer is off the bar (`0.7 + Blend_Value × grooveOpacity`, [breezestyle.cpp:6401-6403](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6401-L6403), with animations on, their default, [breeze.kcfg:45-47](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breeze.kcfg#L45-L47)), and the handle is filled with that at half alpha over the window (`overlayColors(bg, alphaColor(fg, 0.5))`, [breezehelper.cpp:1284](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L1284), `bg` being `QPalette::Window`, [breezestyle.cpp:6405](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6405); `alphaColor` multiplies, [:349-355](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L349-L355)) — Breeze #cbcdcd, Breeze Dark #46494b **(measured)** in a view without the focus ([out_light.txt:27, :55](evidence/breeze-offscreen/2026-11-13/out_light.txt#L27), [out_dark.txt:27, :55](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L27)); with the pointer on the bar but off the handle the factor is 1, #bcbebf / #57595b (computed); in the view that has the focus the handle is `focusColor()` ([breezehelper.cpp:298-299](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezehelper.cpp#L298-L299)), #b1d9ef / #2a546b **(measured)** ([out_light.txt:16](evidence/breeze-offscreen/2026-11-13/out_light.txt#L16), [out_dark.txt:16](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L16))
```

#### Line 1428: `header_font.color` (§2.15 `list.header_font.color`) — 0.7.1 fix (§4.4)

The value does not change under Breeze; its source does.

Current KDE cell:

```text
`[Colors:Header] ForegroundNormal` — Breeze 35,38,41 #232629, Breeze Dark 252,252,252 #fcfcfc in Breeze v6.7.5 (BreezeLight.colors:67; BreezeDark.colors:68)
```

New KDE cell:

```text
**(Breeze src)** `[Colors:Button] ForegroundNormal`: Qt letters a header section in `ButtonText` (`CE_HeaderLabel`, [qcommonstyle.cpp:1659-1660](https://github.com/qt/qtbase/blob/v6.11.0/src/widgets/styles/qcommonstyle.cpp#L1659-L1660)), which Breeze leaves to its parent style ([breezestyle.cpp:1541-1543](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L1541-L1543)); Breeze never reads the Header set for a `QHeaderView` — Breeze 35,38,41 #232629, Breeze Dark 252,252,252 #fcfcfc ([BreezeLight.colors:39](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/colors/BreezeLight.colors#L39); [BreezeDark.colors:40](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/colors/BreezeDark.colors#L40)) **(measured)** ([out_light.txt:141](evidence/breeze-offscreen/2026-11-13/out_light.txt#L141), [out_dark.txt:141](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L141))
```

#### Line 1429: `header_background` (§2.15 `list.header_background`) — 0.7.1 fix (§4.4)

Light changes from #dee0e2 to #fcfcfc; dark is #292c30 in both sets.

Current KDE cell:

```text
`[Colors:Header] BackgroundNormal` — Breeze 222,224,226 #dee0e2, Breeze Dark 41,44,48 #292c30 in Breeze v6.7.5 (BreezeLight.colors:59; BreezeDark.colors:60)
```

New KDE cell:

```text
**(Breeze src)** `[Colors:Button] BackgroundNormal`: Breeze fills a header section with `QPalette::Button` (`drawHeaderSectionControl`, [breezestyle.cpp:6827, :6838-6839](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6827)), a hovered one with `mix(Button, hoverColor, 0.2)` and a pressed one with `mix(Button, focusColor, 0.2)` ([:6828-6837](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L6828-L6837)) — Breeze 252,252,252 #fcfcfc, Breeze Dark 41,44,48 #292c30 ([BreezeLight.colors:31](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/colors/BreezeLight.colors#L31); [BreezeDark.colors:32](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/colors/BreezeDark.colors#L32)) **(measured)** ([out_light.txt:140](evidence/breeze-offscreen/2026-11-13/out_light.txt#L140), [out_dark.txt:140](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L140))
```

#### Line 1430: `grid_color` (§2.15 `list.grid_color`) — deferred (§5.6)

The preset keeps inheriting `defaults.border.color`.

Current KDE cell:

```text
**(none)** — Qt views use palette pen
```

New KDE cell:

```text
**(KF6 src)** `QPalette::Mid`: a `QTableView` shows its grid by default ([qtableview_p.h:110](https://github.com/qt/qtbase/blob/v6.11.0/src/widgets/itemviews/qtableview_p.h#L110)), 1 px, in the style's `SH_Table_GridLineColor` ([qtableview.cpp:1533-1535](https://github.com/qt/qtbase/blob/v6.11.0/src/widgets/itemviews/qtableview.cpp#L1533-L1535)), which QCommonStyle answers with `Mid` ([qcommonstyle.cpp:5126-5128](https://github.com/qt/qtbase/blob/v6.11.0/src/widgets/styles/qcommonstyle.cpp#L5126-L5128)) and Breeze leaves alone; KColorScheme sets `Mid` to the Window set's `MidShade` at the scheme's `[KDE] contrast` ([kcolorscheme.cpp:685, :526, :609-610](https://invent.kde.org/frameworks/kcolorscheme/-/blob/27066d471c93629efee8459d9f69490caa92b7c4/src/kcolorscheme.cpp#L685)), 4 in both Breeze schemes ([BreezeLight.colors:195](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/colors/BreezeLight.colors#L195); [BreezeDark.colors:202](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/colors/BreezeDark.colors#L202)) — Breeze #c4c8cc, Breeze Dark #1c1f21 **(measured)** ([out_light.txt:132, :133, :135](evidence/breeze-offscreen/2026-11-13/out_light.txt#L132), [out_dark.txt:132, :133, :135](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L132)); a list or tree view draws no line between rows **(measured)** ([out_light.txt:124-125](evidence/breeze-offscreen/2026-11-13/out_light.txt#L124-L125), [tree_dark.txt:5](evidence/breeze-offscreen/2026-11-13/tree_dark.txt#L5))
```

#### Line 1435: `hover_background` (§2.15 `list.hover_background`) — 0.7.1 fix (§4.4)

Line 1436 (`hover_text_color`) is right and stays.

Current KDE cell:

```text
`[Colors:View] DecorationHover` blend
```

New KDE cell:

```text
**(Breeze src)** `[Colors:Selection] BackgroundNormal` at alpha `Blend_Value` 0.3: a hovered row that is not selected is `QPalette::Highlight` with its alpha set to 0.3 (`drawPanelItemViewItemPrimitive`, [breezestyle.cpp:4868, :4871-4875](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L4868-L4875); [breezemetrics.h:176](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezemetrics.h#L176)), outlined 1 px in it at alpha 0.8 ([:4880-4884](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L4880-L4884)), taken from the palette's Inactive group in a view without the focus ([:4832-4837](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L4832-L4837); Qt clears `State_Active` there, [qabstractitemview.cpp:3996-3997](https://github.com/qt/qtbase/blob/v6.11.0/src/widgets/itemviews/qabstractitemview.cpp#L3996-L3997)), whose faded selection makes it fainter (§2.1.3 `selection_inactive_background`); a selected, hovered row is `Highlight.lighter(110)`, #44bfff ([breezestyle.cpp:4876](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.cpp#L4876); [breezemetrics.h:178](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezemetrics.h#L178); measured, [out_light.txt:118](evidence/breeze-offscreen/2026-11-13/out_light.txt#L118)) — Breeze and Breeze Dark #3daee94d; over `Base` #c4e6f8 / #204357 **(measured)** ([out_light.txt:108, :130](evidence/breeze-offscreen/2026-11-13/out_light.txt#L108), [out_dark.txt:108, :130](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L108))
```

#### Line 1568: `icon_size` (§2.22 `dialog.icon_size`) — 0.7.1 fix (§4.5)

Current KDE cell:

```text
**(none)** — per-dialog
```

New KDE cell:

```text
**(KF6 src)** 64: Breeze leaves `PM_MessageBoxIconSize` to its base class, KStyle, where it is built with KStyle ([breezestyle.h:55-59](https://github.com/KDE/breeze/blob/f0b1d7534aa2356d7336241d0c7051522e8a6b68/kstyle/breezestyle.h#L55-L59)), and KStyle returns `KIconLoader::SizeHuge`, 64 ([kstyle.cpp:449-451](https://invent.kde.org/frameworks/frameworkintegration/-/blob/v6.31.0/src/kstyle/kstyle.cpp#L449-L451), frameworkintegration v6.31.0); `QMessageBox` and `KMessageBox` size their icon by it; built without KStyle, Breeze would answer QCommonStyle's 32 ([qcommonstyle.cpp:4798-4806](https://github.com/qt/qtbase/blob/v6.11.0/src/widgets/styles/qcommonstyle.cpp#L4798-L4806)) **(measured)** 64 ([out_light.txt:165](evidence/breeze-offscreen/2026-11-13/out_light.txt#L165), [out_dark.txt:165](evidence/breeze-offscreen/2026-11-13/out_dark.txt#L165))
```

### C.2 Checks on the facts file, and the citations that become false

What each script requires of the rows changed above:

- `scripts/check_platform_facts_rows.py` lists registry fields without a Chapter 2 row; it reads only each row's first cell (the backticked property name) and the `### 2.x` / `#### 2.x.y` headings. C.1 changes no first cell and no heading: no effect.
- `scripts/check_section_refs.py` reads `§N.M` references in `docs/property-registry.toml`, `docs/inheritance-rules.toml`, `docs/showcase-elements.toml` and `docs/new-os-version-guide.md` and requires the section heading to exist. C.3's new notes cite §2.1.5, §2.3, §2.6, §2.8 and §2.15, which exist.
- `scripts/check_preset_citations.py` reads every `platform-facts.md:N` in a preset comment and requires the cited line to be a Chapter 2 row in the stated section whose first-cell name is the value's key (or a key the comment names, `K not stated:`, `no K`, `K inherits X`, ...). It compares names, not cell text, so the corrected cells break no existing preset citation. Part B's new comments on the deferred values (focus ring, primary button, menu padding and icon gap) must keep a citation form the script reads: the comment line starts with the citation (`# platform-facts.md:1108 (§2.1.5): ...`) directly above the value, or names the key (`# icon_text_gap_px ...` is not a form; `# platform-facts.md:1267 (§2.6): ...` above `icon_text_gap_px = 8.0` is).
- `scripts/check_wrong_safety_nets.py` requires that every `[uniform]` rule carrying `# see wrong_safety_nets` has a `[wrong_safety_nets."<key>"]` entry whose `inherits_from` is the rule's source, that every entry whose key is a `[uniform]` rule is marked there, and that each entry has string `inherits_from` and `why_wrong`. It does not read the facts file. C.3 adds two marks with their entries.
- The Rust gate `native-theme/src/presets/documented_sizes.rs` (not a script, but it reads this file through `include_str!`) asserts only that each cited line starts with the expected row name (`facts_line(n).starts_with("| `row_height`")`, `:666-684`); its hard-coded expectations for kde-breeze's menu (`:184-185`: padding 4 on all sides, `row_height` None, cited to `1260`-`1262`, note "MenuItem_MarginWidth = 4; MenuItem_MarginHeight = 4") keep passing, but their note and the gate's message "platform-facts gives" no longer describe line 1261 and line 1260. Part B owns that note (table below).

Every citation of a changed line, found with a scan of `platform-facts.md:<N>` (ranges and `, :K` continuations included) over the worktree outside `docs/archive/`. Citations of these lines that read another platform's cell (macOS, Windows, GNOME, Material: in the presets, `windows.rs`, `resolve/tests.rs:2024`, `docs/todo.md:393`, `:423`, `:1815`) stay true, since those cells do not change. The dated gap-map files (`docs/todo_v0.7.1_model-completeness-gap-map-*.md`, `-seeds.md`) quote the file at a past commit and are not edited. The citations whose meaning becomes false or needs a look, each with the part that owns its new wording (the rulings of review round 1; Part B writes its wording true against the rows of C.1):

| Where | Cites | Owner |
|---|---|---|
| `native-theme/src/presets/kde-breeze.toml:53`, `:598` (and the uncommented `focus_ring_width_px` / `focus_ring_offset_px`, `:64-65`, `:609-610`) | 1108-1110 | Part B (preset comments) |
| `kde-breeze.toml:296`, `:844` | 1260 | Part B |
| `kde-breeze.toml:305`, `:853` (and the uncommented `icon_text_gap_px`, `:299`, `:847`) | 1261-1262, 1267 | Part B (its "kept although ..." comment) |
| `kde-breeze.toml:315`, `:863`; `:336`, `:884` | 1429; 1428 | Part B |
| `native-theme/src/presets.rs:471`; `:423-425` | 1269; 1188 | Part B |
| `native-theme/src/kde/metrics.rs:78-83` | 1261-1262 | Part B |
| `native-theme/src/presets/documented_sizes.rs:184-185` | 1260-1262 | Part B |
| `native-theme/src/model/widgets/mod.rs:266-268` (`menu.row_height`'s rustdoc, "KDE's menu items size to their font", cites §2.6 by section) | §2.6 (1260) | Part B |
| `connectors/native-theme-iced/examples/showcase-iced.rs:15619` ("KDE's items size to their font") | §2.6 (1260) | Part B |
| `native-theme/src/detect.rs:688`, `:697`, `:708`; `native-theme/src/kde/mod.rs:13`, `:45`, `:854`, `:1748` | 717-718 | Part A (font DPI rewordings in any rustdoc or README are Part A's) |
| `native-theme/tests/reader_kde.rs:155` | 717 | Stays as it is: its comment, "forceFontDPI=192 in kdeglobals is no font DPI: Plasma keeps the key in kcmfonts", is still true of row 717, and its assertion does not change (`from_kde_content_pure` returns the font DPI it is given; the test passes `None` and asserts `None`; checked by Part A) |
| `docs/inheritance-rules.toml:587`, `:591` | 1269, 1270 | Part C, C.3 |
| `docs/todo.md:532` | 1061 | Part C, C.4 (c): "Only macOS states it" stops being true of the facts row |
| `docs/todo.md:1146` | 1253 | Part C: the entry is closed into new entry 3 (C.4 a 4), its text archived verbatim |
| `docs/todo.md:1786` | 1265 | Part C, C.4 (c) |

The gpui showcase's "rectangular where menu.border.corner_radius is 0" (`info/chrome.rs:282`, `demo.rs:388`, `info/overlays.rs:503`) is conditional and stays true. `connectors/native-theme-egui/src/plugin.rs:19` cites line 1110 for its sign convention (negative = inside), which the new cell keeps.

### C.3 `docs/inheritance-rules.toml`

Which parts are generated or checked (the file's header, lines 1-48, and the tests in `native-theme/src/resolve/inheritance.rs`): `[border_inheritance]` and `[font_inheritance]` are generated from the widget structs' `#[theme_inherit]` attributes and must match the macro byte for byte (`border_inheritance_toml_matches_macro_emit`, `font_inheritance_toml_matches_macro_emit`); the `[uniform]` keys and sources must match the implemented rules (`uniform_rules_all_accounted_for`, `:681`); the `[per_platform]` keys must match `resolve_safety_nets` (`per_platform_rules_match_safety_nets`, `:626-645`), which checks keys only, not the per-platform values; `[defaults_internal]`, `[no_inheritance]` and `[text_scale_inheritance]` are checked likewise. Nothing below changes a key, a source or a generated section; only comments, a per-platform value and `[wrong_safety_nets]` entries, which `scripts/check_wrong_safety_nets.py` checks (C.2).

- **`:195`, `[uniform]`.** Required by the new entry below (rule 2 of the checker).

  Current:

  ```toml
  "menu.separator_color"                       = "defaults.border.color"
  ```

  New:

  ```toml
  "menu.separator_color"                       = "defaults.border.color"         # see wrong_safety_nets
  ```

- **new, appended at the end of `[wrong_safety_nets]`: after line 651 (`segmented_control.active_background`'s `why_wrong`), with a blank line before it, and before the blank line 652 and the TextScale comment at 653, so that no existing line moves (`ios-13.toml` cites `inheritance-rules.toml:605-625`).** The KDE value is §4.2's; the other platforms' are the cells of line 1254 as they stand.

  New:

  ```toml
  [wrong_safety_nets."menu.separator_color"]
  inherits_from = "defaults.border.color"
  why_wrong     = "KDE's separator is mix(Window, WindowText, frameContrast), #c6c8c9 / #4c4e51 under Breeze, where kde-breeze's defaults.border.color is #bcc0bf / #4d545b; macOS (separatorColor), GNOME (its separator) and Material (divider-color) name a colour of their own; Windows inherits it (platform-facts.md:1254, §2.6)"
  ```

- **`:197`, `:198`.** The rules and their marks stay; the entries change.

- **`:585-587`.**

  Current:

  ```toml
  why_wrong     = "Windows' SubtleFillColorSecondary, GNOME's :hover modelbutton bg and Material's on-surface state layer are not the selection; macOS (selectedContentBackgroundColor) and KDE ([Colors:Selection]) take a selection colour (platform-facts.md:1269, §2.6)"
  ```

  New:

  ```toml
  why_wrong     = "Windows' SubtleFillColorSecondary, GNOME's :hover modelbutton bg, Material's on-surface state layer and KDE's DecorationFocus at alpha 0.3 (#3daee94d, a layer, not the opaque selection) are not the selection; macOS (selectedContentBackgroundColor) takes a selection colour (platform-facts.md:1269, §2.6)"
  ```

- **`:589-591`.**

  Current:

  ```toml
  why_wrong     = "Windows keeps defaults.text_color, GNOME its :hover fg (no change), Material on-surface; macOS (selectedMenuItemTextColor) and KDE ([Colors:Selection] ForegroundNormal) take a selection text colour (platform-facts.md:1270, §2.6)"
  ```

  New:

  ```toml
  why_wrong     = "Windows keeps defaults.text_color, GNOME its :hover fg (no change), Material on-surface, KDE the window text ([Colors:Window] ForegroundNormal); macOS (selectedMenuItemTextColor) takes a selection text colour (platform-facts.md:1270, §2.6)"
  ```

- **`:537-539`.** "only KDE matches" becomes false (§5.5).

  Current:

  ```toml
  why_wrong     = "macOS vibrancy and Win COLOR_MENU differ; only KDE matches"
  ```

  New:

  ```toml
  why_wrong     = "macOS vibrancy and Win COLOR_MENU differ; KDE's Qt Widgets menu panel is mix(Window, Base, 0.3), #f4f5f5 / #1c1f22 under Breeze, four to five levels from the window colour, which kde-breeze keeps until the field's other readers allow (platform-facts.md:1253, §2.6)"
  ```

- **`:205` and `:569-571`.** The rule and its mark at `:205` stay.

  Current:

  ```toml
  why_wrong     = "Scrollbar thumb is a semi-transparent UI control; muted_color is a text color"
  ```

  New:

  ```toml
  why_wrong     = "Scrollbar thumb is a semi-transparent UI control; muted_color is a text color (KDE's is WindowText at 0.175 over the window, #cbcdcd / #46494b under Breeze, platform-facts.md:1305, §2.8)"
  ```

- **`:317-322`, `[per_platform."scrollbar.track_color"]`.** §4.3's decision. `fallback` stays.

  Current:

  ```toml
  kde      = "defaults.background_color"
  ```

  New:

  ```toml
  kde      = "preset"                       # no fill: Breeze paints no groove, the view shows through (platform-facts.md:1304, §2.8)
  ```

- **`:565-567`.**

  Current:

  ```toml
  why_wrong     = "macOS headerTextColor, KDE Header group, GNOME columnview header distinct"
  ```

  New:

  ```toml
  why_wrong     = "macOS headerTextColor, KDE's [Colors:Button] ForegroundNormal (Qt letters a header section in ButtonText; Breeze never reads the Header set for it, platform-facts.md:1428, §2.15) and GNOME's columnview header are distinct"
  ```

- **`:245`, `[uniform]`.** Required by the new entry below.

  Current:

  ```toml
  "list.grid_color"                            = "defaults.border.color"
  ```

  New:

  ```toml
  "list.grid_color"                            = "defaults.border.color"         # see wrong_safety_nets
  ```

- **new, appended directly after the `menu.separator_color` entry above, with a blank line between them.** Only the KDE half is stated: the macOS (`gridColor`), GNOME and Material cells of line 1430 were not compared with `defaults.border.color` for this part.

  New:

  ```toml
  [wrong_safety_nets."list.grid_color"]
  inherits_from = "defaults.border.color"
  why_wrong     = "KDE's table grid is QPalette::Mid, the Window set's MidShade at the scheme's contrast, #c4c8cc / #1c1f21 under Breeze, where kde-breeze's defaults.border.color is #bcc0bf / #4d545b, far brighter in dark mode; kde-breeze keeps the inherited value until the field's other readers allow (platform-facts.md:1430, §2.15)"
  ```

- **`:529-531`, `list.header_background`.** Still true: the Button set's #fcfcfc is not `surface_color` #ffffff.

Left as it is, but recorded in the new entry 3 (C.4 b): `[per_platform."input.caret_color"]` `kde = "defaults.focus_ring_color"` (`:310-315`) gives the caret the ring's colour. The caret's KDE cell (line 1209) is `DecorationFocus`, opaque; the ring's (line 1108, corrected) is #3daee94d. Today both are #3daee9 in kde-breeze, so nothing differs; the day the ring takes its Breeze value, this rule must point elsewhere.

### C.4 `docs/todo.md` and `docs/archive/todo_closed.md`

**Conventions** (the header of `docs/todo.md`, lines 1-21; the header of `docs/archive/todo_closed.md`, lines 1-13; `CONTRIBUTING.md:220`). A closed item moves to `todo_closed.md` verbatim, its box ticked, with a dated line naming the commit that closed it (`  *Closed <date> in <sha>:* ...`, two-space indent, as at `todo_closed.md:2563`). A `###` section whose every item is done moves whole, under its `##` heading; an item from a section that keeps open items goes under "### Closed items from <heading>", at the end of that group if it exists; an item of *Deferred (pre-1.0)* goes under *Post-1.0 / Deferred*. An item only partly done stays in `todo.md` and gains a dated line saying what was done (as `todo.md:443-452`, "*W2-j (2026-10-26):* ..."). `<date>` and `<sha>` below are filled in at commit time. A citation of another entry uses its `###` heading and opening words; a `docs/todo.md:N` citation is valid at the commit that wrote it.

**What `scripts/check_todo_citations.py` requires** of every line written here: a preset is cited by its TOML key path in backticks, `` `[light.menu] icon_text_gap_px` ``, after the preset's file name in the same entry; the table and each key must exist in that preset in the tree the check runs on; a backtick span opening `[light`, `[dark` or `[layout` must close on the same line (the entries below are wrapped so that no backtick span breaks); a preset line number (`kde-breeze.toml:296`) is allowed only with `at <sha>` on its line. The entries below cite no preset line numbers.

#### C.4 (a) Entries that close

1. **`docs/todo.md:1570-1578`**, the whole section *Linux font DPI: the `xrandr` fallback has no facts row*. Delete lines 1570-1579 (heading, item, the blank line after it). In `todo_closed.md`, under `## Core API`, insert before line 910 (the `---` above `## Toolkit Connectors`), after the C.4 (a) 4 section below:

````markdown
### Linux font DPI: the `xrandr` fallback has no facts row

- [x] [v0.7.x] From the kde-breeze validator after M4 T34 (D3, deferred to Step
      8): the font-DPI chain ends with the physical DPI `xrandr` reports
      (`detect_physical_dpi`, `native-theme/src/detect.rs:473-485`, the last
      step of `detect_system_font_dpi`, `:636`), and no
      `docs/platform-facts.md` row says that any desktop uses it for fonts.
      Find a source (Plasma, gnome-settings-daemon) and add the row, or drop
      the step so the chain ends at 96.
  *Closed <date> in <sha>:* the step is gone with the whole chain. On Plasma the
  toolkits already scale by the display (the compositor's scale on Wayland, `Xft.dpi` ÷
  96 on X11), so a font DPI that carried it counted it twice, and `xrandr`'s physical
  DPI added the monitor's density on X11 and logical pixels under Xwayland; since 0.7.1
  the font DPI is 96 on every Linux desktop (issue #4,
  `docs/issues/0004_kde-hand-overrides-rationale.md` §4.1).
````

2. **`docs/todo.md:3989-3990`**, the first item of *Font DPI* under *Deferred (pre-1.0)*. Delete the two lines; the section keeps its `[upstream]` item (`:3991-3992`), which is unaffected. In `todo_closed.md` append to the group *Closed items from Font DPI* (under *Post-1.0 / Deferred*), after line 2177 and before the blank line at 2178:

````markdown
- [x] [1.0] Unverified: do the Linux readers' `font_dpi` sources (`Xft.dpi`, KDE `forceFontDPI`) have the same logical-versus-physical question as Windows under Wayland fractional scaling? The Windows reader now reads its fonts at 96 DPI and reports a `font_dpi` of 96, because the model's sizes are logical pixels (`native-theme/src/windows.rs`, `LOGICAL_DPI`). If a Linux session at, say, 150 % sets `Xft.dpi` to 144 while the toolkit also applies the 1.5 scale factor, a 10pt font would resolve to device pixels and be scaled again. Not checked on a KDE or GNOME Wayland session. M2e (2026-10-11) answered it from source. GNOME: gnome-settings-daemon 51.0 sets `Xft.dpi` to 96 × `text-scaling-factor` × the X11 UI scale (`plugins/xsettings/gsd-xsettings-manager.c:583-595`, `:665-668`, `:797-798`), so it carried the text scale and the display scale; since M2e Step 7 the GNOME reader and `ResolutionContext::from_system()` on GNOME use 96, which applies the factor once on Wayland; X11 and Xwayland clients also get it through the toolkit's `Xft.dpi` scale factor (`docs/archive/todo_closed.md`, *Closed items from Font DPI*, "For M4 (connectors): on GNOME and Budgie X11"; its Xwayland half is the next entry). KDE: Plasma 6.7.5's krdb sets `Xft.dpi` on Wayland to `kwinrc [Xwayland] Scale` × 96 (`kcms/krdb/krdb.cpp:333-337`), and Plasma 6.8 always does (`startkde/plasma-setup-xwayland/main.cpp:47-58` at v6.7.91), so the KDE reader's `Xft.dpi` fallback on Wayland is the display scale: open, for M4, with Plasma 6.8's removal of `forceFontDPI`, which the KDE reader and `detect.rs` read first (ledger M2e-01 to M2e-03).
  M4 (2026-10-12/13): one font-DPI chain in `detect`, cached in `DetectionContext` and shared by the KDE reader (9ed0ca92); `forceFontDPI` is read on X11 only (the plan's ruling) and from `kcmfonts`, the file Plasma opens (49e0067a, b42b1568), so on a Wayland session the chain starts at `Xft.dpi`. The Wayland question above is still unverified; re-check at Plasma 6.8.0 with M2e-01.
  *Closed <date> in <sha>:* answered from source and measured (issue #4,
  `docs/issues/0004_kde-hand-overrides-rationale.md` §4.1): yes, on every Plasma
  session. On Wayland Plasma writes `Xft.dpi` as the largest output's scale × 96 while
  legacy X11 applications scale themselves, the default (kwin v6.7.5
  `src/workspace.cpp:613-626`, v6.8.2 `src/main.cpp:281-310`; plasma-workspace v6.8.2
  `startkde/plasma-setup-xwayland/main.cpp:47-58`), and on Plasma 6.7's X11
  `forceFontDPI` is what kscreen writes as the global scale × 96 (kscreen v6.7.5
  `kcm/kcm.cpp:421-482`), while gpui and winit already scale by the compositor's scale
  or by `Xft.dpi` ÷ 96 and Qt converts points at 96. The chain made a 10 pt font 30
  physical pixels at scale 1.5 where Qt's is 20; on a private X server the egui
  showcase's body capitals were <c144-before> px high at `Xft.dpi` 144 before the fix
  and <c144-after> after it, against <c96> at 96 after it
  (`docs/evidence/breeze-offscreen/<run-date>/x11/capheight-out.txt`). The font DPI is
  96 on every Linux desktop since 0.7.1, and the chain, its cache and the `kcmfonts`
  read are gone. What 96 does not cover (Qt Widgets' whole-pixel font sizes, a font DPI
  for `SystemTheme::from_system()`) is in *KDE: what a Qt application under Breeze shows
  that native-theme does not (issue #4)*, "Font sizes and the KDE button and dialog
  sizes".
````

The `<c96>`, `<c144-before>` and `<c144-after>` are the heights that file records (C.5 b): `<c96>` is the after-fix build's reading at 96, `<c144-before>` and `<c144-after>` the two builds' at 144. The rationale (§4.1, §9) expects 10, 22 and 14 to 15; C.5 (b) states the rule for a reading that differs.

3. **`docs/todo.md:1819-1823`**, *KDE's dialog icon* in *Values the v0.7.1 gap map found against their sources*. The rationale's §6 does not list it, but §4.5 does exactly what it asks (the specification's decision 5). Delete the five lines. In `todo_closed.md`, under `## Core API`, after the section of item 1 above, add the group:

````markdown
### Closed items from Values the v0.7.1 gap map found against their sources

- [x] [v0.7.x] **KDE's dialog icon.** `kde-breeze.toml` `[light.dialog] icon_size_px` and
      `[dark.dialog] icon_size_px` are 32 and `platform-facts.md` §2.22 says "(none) —
      per-dialog", but on Plasma Breeze's base class is KStyle, whose
      `PM_MessageBoxIconSize` is `KIconLoader::SizeHuge`, 64 (KDE/frameworkintegration
      v6.31.0 `src/kstyle/kstyle.cpp:449-451`; gap map K1.224†).
  *Closed <date> in <sha>:* `docs/platform-facts.md:1568` gives 64,
  `PM_MessageBoxIconSize` from KStyle, measured 64 under Breeze 6.8.2
  (`docs/evidence/breeze-offscreen/2026-11-13/`), and kde-breeze states 64 in both modes
  (issue #4, `docs/issues/0004_kde-hand-overrides-rationale.md` §4.5). No connector
  reads the field yet; a helper for it is in *KDE: what a Qt application under Breeze
  shows that native-theme does not (issue #4)*, "Font sizes and the KDE button and
  dialog sizes".
````

4. **`docs/todo.md:1140-1150`**, the whole section *kde-breeze: the Qt Widgets menu fill*. Not in the rationale's §6; it asks for what §5.5 decided, and the new entry 3 (item 3) carries the rest (the specification's decision 5). Delete lines 1140-1151. In `todo_closed.md`, under `## Core API`, before the section of item 1:

````markdown
### kde-breeze: the Qt Widgets menu fill

- [x] [v0.7.x] From M4 T36 and the kde-breeze validator after it (2026-10-13):
      Breeze's Qt Widgets menu is filled with `frameBackgroundColor()`,
      mix(Window, Base, 0.3) (breezestyle.cpp `drawPanelMenuPrimitive`;
      breezehelper.cpp:200-203, as the T36 implementer read them), while
      `docs/platform-facts.md:1253` gives KDE's `menu.background_color` as
      `[Colors:Window] BackgroundNormal`, which qqc2-desktop-style's `Menu.qml`
      uses (`Kirigami.Theme.backgroundColor`). The row depends on which toolkit
      it describes. Cite both at v6.7.5 / v6.31.0 in the facts cell, decide
      which one kde-breeze states, and correct the preset or the cell.
  *Closed <date> in <sha>:* the facts cell gives Breeze's Qt Widgets panel,
  `mix(Window, Base, 0.3)`, #f4f5f5 / #1c1f22, measured
  (`docs/evidence/breeze-offscreen/2026-11-13/`); kde-breeze keeps the window colour,
  because the field's other readers (egui's `window_fill`, iced_aw's menu bar) would
  move away from Breeze (issue #4, `docs/issues/0004_kde-hand-overrides-rationale.md`
  §5.5). The rest, with qqc2-desktop-style's `Menu` fill, which 0.7.1 did not re-read,
  continues in *KDE: what a Qt application under Breeze shows that native-theme does not
  (issue #4)*, "KDE values that wait for a connector design", (3).
````

Entries that close in part (they stay in `docs/todo.md`, each gains a dated line after its last line):

5. **`docs/todo.md:391-452`**, *Presets: the scrollbar thumb colour platform-facts measures*, its KDE half (the W2-j paragraph, `:443-452`). Insert after line 452, as a list item of its own: the entry names `macos-monterey.toml` (`:438`, a dated line citation), and `scripts/check_todo_citations.py` would otherwise look for the new key paths in macos-monterey too, which states no `thumb_color`; a line starting with `- ` starts a new entry for the checker.

````markdown
      - *Issue #4 (0.7.1, <date>, <sha>):* the KDE half is done. §2.8's KDE
        `thumb_color` cell (`docs/platform-facts.md:1305`) gives Breeze's resting
        handle, `WindowText` at alpha 0.175 over the window (the 0.5, the 0.7 while the
        pointer is off the bar, and the half alpha above), #cbcdcd / #46494b, measured
        with Breeze 6.8.2 (`docs/evidence/breeze-offscreen/2026-11-13/`); kde-breeze
        states it (`kde-breeze.toml` `[light.scrollbar] thumb_color`,
        `[dark.scrollbar] thumb_color`) and the reader computes it. The macOS, Windows
        and macos-tahoe parts above stay open.
````

6. **`docs/todo.md:1732-1754`**, the `[v0.7.x]` **KDE.** entry of *Values the v0.7.1 gap map found against their sources*: items (1), (4) and (6) close; (3) and (9) continue in the new entries 4 and 3. Insert after line 1754:

````markdown
      *Issue #4 (0.7.1, <date>, <sha>):* (1), (4) and (6) are fixed in the facts,
      kde-breeze and the reader: the menu's hover is `focusColor` at alpha 0.3,
      #3daee94d, under `[Colors:Window] ForegroundNormal`
      (`docs/platform-facts.md:1269-1270`), the list header the Button set
      (`:1428-1429`), the list's hover `Highlight` at alpha 0.3, #3daee94d (`:1435`),
      each measured (`docs/evidence/breeze-offscreen/2026-11-13/`). (3) and (9) have
      their facts corrected (`:1061`, `:1108-1110`) and continue in *KDE: what a Qt
      application under Breeze shows that native-theme does not (issue #4)*, "The
      selection of a view without the focus" and "KDE values that wait for a connector
      design". (2), (5), (7) and (8) stay open here.
````

7. **`docs/todo.md:2561-2605`**, the `[1.0]` entry "kde-breeze states `button.hover_background = "#93cee9"`", its list half. Insert after line 2605:

````markdown
      *Issue #4 (0.7.1, <date>, <sha>):* the list half is fixed: `list.hover_background`
      is Breeze's `Highlight` at alpha 0.3, #3daee94d in both modes
      (`docs/platform-facts.md:1435`; `kde-breeze.toml` `[light.list] hover_background`,
      `[dark.list] hover_background`), not #93cee9 / #2e4a5e. `sidebar.hover_background`
      keeps #93cee9 / #2e4a5e: its source is Kirigami, which was not read. The
      checkbox's, slider's and combo box's #93cee9 wait for Breeze's hover outlines
      (*KDE: what a Qt application under Breeze shows that native-theme does not
      (issue #4)*, "What Breeze draws that the model has no field for"). The solarized
      and tokyo-night label contrasts stay this entry's.
````

8. **`docs/todo.md:39-41`**, in *Font rendering preferences — read them from the platform*. Current:

````markdown
      We already read `Xft.dpi` for scaling (`detect::read_xft_dpi`,
      `native-theme/src/detect.rs:459`, in the font-DPI chain the KDE reader shares),
      but nothing reads the *rendering*
````

New (lines 42 onward unchanged; the paragraph re-wraps):

````markdown
      Since 0.7.1 native-theme reads no X resource: its font DPI is 96 on every Linux
      desktop, where it read `Xft.dpi` before
      (`docs/issues/0004_kde-hand-overrides-rationale.md` §4.1); and nothing reads the
      *rendering*
````

9. **`docs/todo.md:1101-1102`**, in *Linux: button order and `detect_platform` decide by the raw desktop*. Current: "since 62c13acf the reader, the watcher, the icon theme and the font DPI decide by the refined desktop". New: "since 62c13acf the reader, the watcher and the icon theme decide by the refined desktop (the font DPI did too until 0.7.1 made it 96 on every Linux desktop)". The rest of the entry is unchanged.

10. **`docs/issues/0004_kde-union-rationale.md:29`** and **`:280`** (named in the rationale's §4.1). At `:29`, "; `kcmfonts` for the font DPI" becomes "; until 0.7.1 also `kcmfonts` for the font DPI, which is 96 on every Linux desktop since ([`0004_kde-hand-overrides-rationale.md`](0004_kde-hand-overrides-rationale.md) §4.1)". At `:279-281`, "as it already runs `xrdb` and `xrandr` for the font DPI (`native-theme/src/detect.rs:460`, `:484`)" becomes "as it ran `xrdb` and `xrandr` for the font DPI until 0.7.1 (`native-theme/src/detect.rs:460`, `:484` at 387fc449)" (both lines hold at `387fc449`: `read_xft_dpi`'s `xrdb` call and `detect_physical_dpi`'s `xrandr` call). Each paragraph re-wraps.

11. **`docs/todo.md:15`**, the header's example of how to cite an entry, quotes the opening words "Unverified: do the Linux readers'" of the *Font DPI* item that item 2 above moves to the archive. Those quoted words become "An X11 client through Xwayland", the opening words of the `[upstream]` item that stays in that section (`:3991`); the file and heading of the example stay, and the paragraph re-wraps.

12. **Line citations that Part A's removals move** (its A56; several were already off before this change). After Part A's edits to `native-theme/src/detect.rs` and `native-theme/src/kde/mod.rs` land, each is re-pointed to the line of the item its sentence names, found by searching for that item in the edited file, not by subtracting the removed lines:

| `docs/todo.md` | Cites today | The item its sentence names (search for) | At `387fc449` |
|---|---|---|---|
| `:1036` | `detect.rs:766-771` | the `GTK_THEME` check in `detect_is_dark_inner` (`std::env::var("GTK_THEME")`) | `:768-771` (the function starts at `:765`) |
| `:1104` | `detect.rs:1355` | `pub fn refined_linux_desktop_sync` | `:1361` |
| `:1118` | `detect.rs:1344-1352` | the paragraph of `refined_linux_desktop_sync`'s rustdoc that starts "A system read caches the desktop it refined" (Part A's A13 rewrites the rustdoc above it) | `:1350-1356` |
| `:1134` | `kde/mod.rs:188` | `("SmallDefault", 16.0)` in `KICONTHEME_DEFAULT_SIZES` | `:188` |
| `:1625` | `detect.rs:905`, `:942` | `pub fn prefers_reduced_motion` and `pub fn accessibility_preferences` | `:907`, `:948` |
| `:1638` | `kde/mod.rs:220` | `fn kde_icon_base_dirs(` | `:220` |
| `:1649` | `detect.rs:1355-1360` | `refined_linux_desktop_sync`'s body, whose `kdeglobals_exists` call is the `stat` | `:1361-1366` |
| `:2704` | `kde/mod.rs:42-52`, `src/detect.rs:674-685` | the two readings of reduced motion: `fn reduce_motion` (`[KDE] AnimationDurationFactor`) in `kde/mod.rs` and `fn desktop_reduce_motion` in `detect.rs` | `kde/mod.rs:277-283`, `detect.rs:976-988` |

The citation keeps its form (`native-theme/src/detect.rs:N`); a range keeps covering the same item. `connectors/native-theme-egui/src/atlas.rs:754` (`detect.rs:765`) is Part B's. The re-pointing is done in Part E's sweep of ungated line citations after the last source edit (E.2).

#### C.4 (b) The five new entries

Placement. Entries 2 to 5 go under a new `###` section in `## Core API`, inserted after *KDE: Union, the Application Style that is data (issue #4)* (which ends at line 1704) and before *Dialog: the platform's vertical placement* (line 1705), with a blank line before the next heading. Entry 1 goes under `## Toolkit Connectors` → *native-theme-gpui connector* → *Upstream PR candidates from v0.5.8 (Tier U)*, as its last item, after the item that ends at line 2465 and before `#### native-theme-iced: the same audit` (line 2467). Entry 1 links the upstream requests file, Part D's (ruling 5), which holds the requests' text and their gpui-component and gpui-base citations; the entry repeats none of those line numbers. The citations of GPUI Kit's code in entry 3 are the rationale's (§5.1); those in the connectors' own sources (`colors.rs:203-204`, `:446`, egui `roles.rs:229`, `:287`, `:776-779`, `base.rs:201`, iced `styles/aw.rs:152`, `styles.rs:1261`) were opened for this part; gpui-component's and gpui-pre's 0.9.1 / 0.3.12 sources were not among the downloaded files and were not re-opened.

The new section:

````markdown
### KDE: what a Qt application under Breeze shows that native-theme does not (issue #4)

From issue #4's second comment, `docs/issues/0004_kde-hand-overrides-rationale.md`
(2026-11-13). Breeze lines are at `f0b1d753` (every file cited is byte-identical at
`v6.8.2`), KColorScheme at `27066d47`; "measured" is Breeze 6.8.2 under Qt 6.12,
offscreen, `docs/evidence/breeze-offscreen/2026-11-13/`.

- [ ] [v0.7.x] **What Breeze draws that the model has no field for.** First, as the next
      0.7.x step, "Breeze's hover and focus outlines", with a rationale of its own:
      Breeze shows hover on a button, a combo box, a check box and a slider handle by
      recolouring the outline, never the fill (a non-flat button's pen turns `Highlight`
      on hover, keyboard focus or press, `breezehelper.cpp:710-712`), and the model has
      `hover_border_color` and `focus_border_color` for text inputs only, so on KDE a
      connector's button shows no hover at all. The kde-breeze hover fills that stand in
      for an outline wait for it, since removing them first would remove the only hover
      feedback: `kde-breeze.toml` `[light.checkbox] hover_background`,
      `[light.slider] thumb_hover_color`, `[light.combo_box] hover_background` and their
      dark twins, listed in *Presets: values the M8 T15 platform-facts rows disagree
      with*. The same step takes: a toolbar button's states (hover a 1 px `Highlight`
      outline, pressed `Highlight` at 0.3, checked `ButtonText` at 0.125,
      `breezehelper.cpp:665-712`); a hovered menu item's 1 px outline,
      `mix(focusColor, WindowText, 0.15)` (`breezehelper.cpp:172-175`), #399acc /
      #5abaec (measured); a hovered list row's 1 px outline, `Highlight` at alpha 0.8
      (`breezestyle.cpp:4880-4884`); a menu bar title's fill, the opaque hover or focus
      colour under `HighlightedText` (`drawMenuBarItemControl`,
      `breezestyle.cpp:5841-5948`), which `menu.hover_background` stood in for until
      0.7.1 (*Model: the menu bar has no fill of its own*); a header section's hover and
      pressed fills, `mix(Button, hoverColor, 0.2)` and `mix(Button, focusColor, 0.2)`
      (`breezestyle.cpp:6828-6837`), #d6ecf8 / #2d4655 (computed), and its separators.
      Then the rest, each without a field: the pressed menu item, the opaque focus
      colour under `HighlightedText` (`breezestyle.cpp:6021-6029`, `:6155`); the 4 px
      gap between the menu panel's edge and the highlight (`PM_MenuHMargin` and
      `PM_MenuVMargin` are `MenuItem_HighlightGap` 4, `breezestyle.cpp:649-651`;
      measured); the menu row's minimum height, the larger of the text, the 20 px
      columns and the icon, plus 1, plus 4 above and below
      (`breezestyle.cpp:3663-3667`), 29 at Noto Sans 10 pt (measured;
      `docs/platform-facts.md:1260`), which `menu.row_height`, a fixed height, cannot
      carry; the scroll bar's 1 px line along its inner edge, `Text` at alpha
      frameContrast (`breezestyle.cpp:8220-8227`), #d3d4d4 / #424445 in a list
      (measured); the handle's outline, its colour at alpha frameContrast
      (`breezehelper.cpp:1283`); the handle of the view that has the focus,
      `focusColor()` (`breezehelper.cpp:298-299`), #b1d9ef / #2a546b (measured); the
      handle while the pointer is on the bar but off it, `WindowText` at 0.25 over the
      window, #bcbebf / #57595b (computed); a hovered row in a view without the focus,
      the Inactive `Highlight`, #c2e0f5 / #1b4155, at 0.3 (`breezestyle.cpp:4832-4837`);
      a selected and hovered row, `Highlight.lighter(110)`, #44bfff
      (`breezestyle.cpp:4876`; measured); a list item's padding beyond the highlight's
      margin. The parked 0.8.0 work traces some of these
      (`docs/todo_v0.7.1_model-completeness-spec-trace-kde.md` on `v0.8.0-rc1`: K1.92
      the focused handle, K1.172 the selected and hovered row, K1.177 the header's
      fills). Not a missing field but a gap of the same kind: kde-breeze states
      `list.hover_text_color`, #232629 / #fcfcfc (`kde-breeze.toml`
      `[light.list] hover_text_color`, `[dark.list] hover_text_color`), and the KDE
      reader never writes it (`native-theme/src/kde/colors.rs` writes only the button's
      hover text, `:525`), so under a colour scheme whose View text differs from
      Breeze's a hovered list row keeps Breeze's text colour, while since 0.7.1 the
      hovered menu row follows the scheme's `[Colors:Window] ForegroundNormal`; Breeze
      letters a hovered row in `QPalette::Text`, the View set's `ForegroundNormal`
      (`docs/platform-facts.md:1436`).
- [ ] [v0.7.x] **KDE values that wait for a connector design.** Each facts row below is
      corrected in 0.7.1; kde-breeze and the reader keep their present values, which
      disagree with it, until the readers named can take the platform's value without
      another surface losing. (1) **The focus ring**,
      `docs/platform-facts.md:1108-1110`: Breeze's keyboard-focus `QFocusFrame` is a
      band 2 px wide of `Highlight` at alpha 0.3, #3daee94d in both schemes, from 1 px
      inside a line edit's, spin box's, combo box's or button's rectangle to 1 px
      outside it (offset −1), drawn only after Tab, Backtab or a shortcut; a pointer
      focus only recolours a line edit's own frame (`focusColor()`), and a button's
      frame turns `Highlight` on keyboard focus (`breezestyle.cpp:1648-1674`,
      `:6614-6781`; `PM_FocusFrameHMargin` 2 measured). kde-breeze states #3daee9, width
      1, offset 2 with no source (`kde-breeze.toml`
      `[light.defaults] focus_ring_color, focus_ring_width_px, focus_ring_offset_px`,
      `[dark.defaults] focus_ring_color, focus_ring_width_px, focus_ring_offset_px`). It
      waits because gpui-component's one `ring` colour is both a focused element's
      tinted border and the band, whose alpha it sets itself (`styled.rs:249-327`,
      0.9.1): a translucent ring would make a focused text field's border faint, and
      taking the input's focus border colour for `ring` would change Windows, whose two
      colours differ; and because gpui-component's buttons, inputs and check boxes and
      the iced and egui connectors' rings show on every focus, a click included. GPUI
      can tell the two (`Window::last_input_was_keyboard`, gpui-pre 0.3.12
      `src/window.rs:3641-3644`); whether iced or egui can was not read. A design over
      every platform and three connectors, with *GPUI Kit, from issue #4*'s request (4).
      When the ring takes #3daee94d, `docs/inheritance-rules.toml`'s KDE caret rule
      (`[per_platform."input.caret_color"]`, `kde = "defaults.focus_ring_color"`) needs
      another source: the caret is `DecorationFocus`, opaque
      (`docs/platform-facts.md:1209`). Which part of a control the band surrounds:
      *platform-facts: which part of a control the focus ring surrounds*. Until then an
      application that wants no band on a click turns gpui's ring off (the gpui README's
      overrides section).
      (2) **The default button**, `docs/platform-facts.md:1188-1189`: Breeze fills the
      active window's default button with `mix(Button, Highlight, 0.2)`, #d6ecf8 /
      #2d4655, outlined #87c0dd / #4882a1, under the ordinary `ButtonText`, #232629 /
      #fcfcfc (measured); `button.primary_background` and `primary_text_color` inherit
      the accent pair, #3daee9 under #ffffff / #fcfcfc. It waits because gpui's
      `primary` and `primary_foreground` tokens
      (`connectors/native-theme-gpui/src/colors.rs:203-204`), which gpui-component's
      widgets use as the theme's accent, and egui's selection fill
      (`connectors/native-theme-egui/src/style/roles.rs:229`) read the two fields: one
      button per dialog would gain, and every accent-coloured gpui-component widget and
      every egui selection would turn pale. First the connectors stop reading the
      primary button's fill as the accent. The model has no field for the default
      button's outline.
      (3) **The menu panel's colour**, `docs/platform-facts.md:1253`: Breeze's Qt
      Widgets menu panel is `mix(Window, Base, 0.3)`, #f4f5f5 / #1c1f22 (measured), four
      to five levels from the window colour `menu.background_color` inherits in
      kde-breeze, #eff0f1 / #202326. It waits because egui's base style takes
      `window_fill` from it (`connectors/native-theme-egui/src/style/base.rs:201`),
      which fills every `egui::Window`, popup and tooltip, and iced_aw fills the menu
      bar with it (`connectors/native-theme-iced/src/styles/aw.rs:152`), surfaces Breeze
      paints in the window colour; gpui-component paints a menu in its `popover` token,
      the window colour on KDE since 0.7.0, so gpui has no receiver. Changing the field
      would move more surfaces away from Breeze than toward it. Read and cite
      qqc2-desktop-style's `Menu` fill too, which M4 T36 read as
      `Kirigami.Theme.backgroundColor` (from *kde-breeze: the Qt Widgets menu fill*,
      closed into this item).
      (4) **The table's grid**, `docs/platform-facts.md:1430`: `QPalette::Mid`, the
      Window set's `MidShade` at the scheme's contrast, #c4c8cc / #1c1f21 under Breeze's
      4 (measured; #bdc1c6 / #1c1e21 under KColorScheme's default 7, computed);
      kde-breeze's `list.grid_color` inherits `defaults.border.color`, #bcc0bf /
      #4d545b: close to Qt's in light, far brighter in dark, which the reporter saw. It
      waits because egui's list role draws every separator and frame outline inside a
      list with it (`connectors/native-theme-egui/src/style/roles.rs:776-779`; egui has
      one stroke for both, and the role reads no `list.border.color`) and gpui's
      `chart_grid` token reads it (`connectors/native-theme-gpui/src/colors.rs:446`): a
      list's outline and a chart's grid would all but vanish in dark mode (#1c1f21 on
      #202326). First egui's list role separates the frame from the grid. The derivation
      exists on `v0.8.0-rc1`, written in a clean room and checked against KDE's code
      (`native-theme/src/platform/kde_shade.rs`, with tables from `tools/kde-vectors`),
      to be carried over. Until then an application sets gpui's `table_row_border` to
      #c4c8cc / #1c1f21 (the gpui README's overrides section).
      (5) **The menu item's radius, padding and icon gap**,
      `docs/platform-facts.md:1261`, `:1265`, `:1267`: Breeze rounds a hovered item's
      highlight at 4.5; kde-breeze states no menu corner radius, and
      `menu.border.corner_radius` is read as the item's radius by egui's menu role
      (`connectors/native-theme-egui/src/style/roles.rs:287`) and the showcases, and as
      the panel's by iced (`connectors/native-theme-iced/src/styles.rs:1261`): one
      field, two meanings. In a menu without icons or checkable items (check box or
      radio items) Breeze starts an item's text 16 from the highlight's edge (4 + 8 + 4,
      measured). In a menu with icons it puts a 16 px icon in a column 20 px wide, 12 px
      in where the menu has no checkable items (their column comes first otherwise,
      `breezestyle.cpp:6059-6065`, `:6077`), and starts the text 4 after the icon
      (measured); kde-breeze and the reader state 4 and 8 (`kde-breeze.toml`
      `[light.menu.border] padding_horizontal_px`, `[light.menu] icon_text_gap_px`, and
      dark; `native-theme/src/kde/metrics.rs`). Both are wrong in directions that partly
      cancel: correcting the gap alone moves the text further from where Breeze puts it,
      so the two are one fix, with the question of what `padding_horizontal` means for a
      row with and without an icon.
- [ ] [v0.7.x] **The selection of a view without the focus, KDE and macOS, and its
      missing text colour.** Breeze's schemes set
      `[ColorEffects:Inactive] ChangeSelectionColor`, so a selected row in an unfocused
      view is `KColorUtils::tint(Window, Selection, 0.4)`, #c2e0f5 / #1b4155, lettered
      in the window text, #232629 / #fcfcfc (measured; `docs/platform-facts.md:1061`;
      `kcolorscheme.cpp:304-321`, `:374-377`). The model has
      `defaults.selection_inactive_background` and no text colour for it, and no
      connector draws it (gpui and egui have an accessor only, iced records no
      receiver), so stating it alone would change nothing on screen and would give an
      application that pairs it with `selection_text_color` white text on a pale fill,
      as macos-monterey already does (`macos-monterey.toml`
      `[light.defaults] selection_inactive_background, selection_text_color`: #dcdcdc
      with #ffffff). Add the text colour (registry, inheritance rules, facts, presets,
      the KDE and macOS readers), then state the pair for KDE and draw it in the
      connectors. macOS's text colour is researched in *Selection: the text colour macOS
      pairs with the unemphasised selection*. The KDE derivation exists on `v0.8.0-rc1`,
      written in a clean room and checked against KDE's code
      (`native-theme/src/platform/kde_selection_inactive.rs`, `kcu_tint.rs`, with tables
      from `tools/kde-vectors`), to be carried over. Continues item (3) of *Values the
      v0.7.1 gap map found against their sources*, "**KDE.**".
- [ ] [1.0] **Font sizes and the KDE button and dialog sizes.** (1) Qt Widgets rounds a
      font's pixel size to a whole number before it scales (10 pt is 13 px, qtbase
      v6.10.1 `src/gui/text/qfontdatabase.cpp:2838-2841`; measured 13 under Breeze,
      `docs/evidence/breeze-offscreen/2026-11-13/`), where native-theme gives 13.33;
      read whether Qt Quick rounds too, then decide whether the model follows. (2)
      `SystemTheme::from_system()` takes no font DPI, so an application on a toolkit
      that does not scale by `Xft.dpi` on X11 (none of the three connectors') gets 96
      since 0.7.1 where the chain gave it Qt's text size; it re-derives its font sizes
      from the resolved font's `defined_size`, which the text scale's sizes do not
      carry. A way to pass another font DPI waits for such a consumer. (3) kde-breeze's
      `button.min_height_px = 32` has no citation (`kde-breeze.toml`
      `[light.button] min_height_px`), and `min_width` 80 and the 6 px padding are
      Breeze's values before its 2 px frame margin: a button's size hint is text + 2 ×
      (6 + 2), "OK" at Noto Sans 10 pt measures 84 × 34 (measured), and Breeze draws the
      frame 1 px inside that, a visible box of 82 × 32 with 7 px between the border line
      and the text, where the stated values give 80 × 32 with 6 px; decide what the
      model's padding is measured from. (4) kde-breeze's dialog sizes have no citation
      and the facts give KDE none (`kde-breeze.toml`
      `[light.dialog] min_width_px, max_width_px, min_height_px, max_height_px`). (5) No
      connector reads `dialog.icon_size`, 64 on KDE since 0.7.1
      (`docs/platform-facts.md:1568`): iced records no receiver, and the gpui and egui
      dialog icon helpers read `defaults.icon_sizes.dialog`, KDE's dialog icon group,
      32, which is another size; a helper for the message icon's size. The reporter's 14
      px button padding, 56 px icons, 1.25 line height and 50 % disabled opacity have no
      source in Breeze.
````

Entry 1, under *Upstream PR candidates from v0.5.8 (Tier U)*:

````markdown
- [ ] [upstream] **GPUI Kit, from issue #4**
      (`docs/issues/0004_kde-hand-overrides-rationale.md` §4.6, §4.7, §5.1; 2026-11-13).
      Four requests, drafted for GPUI Kit's tracker in
      `docs/issues/0004_kde-hand-overrides-upstream-requests.md`, with their sources;
      none is posted without the maintainer's approval. (1) A table's scroll bar box
      takes the themed track width, as every other scroll host does: gpui-component
      0.9.1 sizes it with the fixed 16 px `Scrollbar::width()`, so under a theme with a
      wider track (Breeze's is 21 px) the bar in a table is clipped, its thumb off
      centre; gpui-base would have to make the themed width readable. The gpui README's
      limits give the workaround, an overlay with `scrollbar.groove_width` 16. (2) A
      selected cell's overlay is painted behind the cell's content, or gets a token of
      its own: in cell-selection mode `table_active` is laid over a selected cell, so
      the opaque native selection the connector restores hides the cell's text. (3) A
      colour a `ThemeConfig` states is not capped, or the cap can be turned off:
      applying a config caps `selection` at alpha 0.3 and `list_active` and
      `table_active` at 0.2; the connector writes the native colours back after every
      rebuild meanwhile. (4) The band outside a focused element is drawn only for a
      focus from the keyboard, which GPUI can tell (`Window::last_input_was_keyboard`);
      a text input keeps its tinted border on any focus, and a button's and a check
      box's border tint is keyboard-only too, as Breeze draws them
      (`docs/platform-facts.md:1108-1110`). See also *the focus ring's width* above, and
      *KDE: what a Qt application under Breeze shows that native-theme does not
      (issue #4)*, "KDE values that wait for a connector design", (1).
````

#### C.4 (c) Existing entries on the same subjects

Found by searching `docs/todo.md` for `93cee9`, `focus_ring`, `grid_color`, `Union`, `issue #4`, and for `selection_inactive`, `menu.background`, `track_color`, `MessageBoxIcon`, `Xft.dpi`, `forceFontDPI`, `font DPI`, `accent`, `groove`. Each gets a dated line after its last line, unless noted.

- **`:504-512`, *platform-facts: which part of a control the focus ring surrounds*.**

````markdown
      *Issue #4 (2026-11-13):* KDE answered from Breeze `f0b1d753`: the keyboard-focus
      band surrounds a line edit, text edit, spin box, combo box, push or tool button
      whole, from 1 px inside to 1 px outside (`breezestyle.cpp:6642-6651`), and a check
      box's or radio button's indicator or a slider's handle alone (`:6652-6708`); KDE's
      ring values wait in *KDE: what a Qt application under Breeze shows that
      native-theme does not (issue #4)*, "KDE values that wait for a connector design".
````

- **`:530-559`, *Selection: the text colour macOS pairs with the unemphasised selection*.** Its second sentence, "Only macOS states it", stops being true of the facts row cited right after it. Change "Only macOS states it" to "Only macOS's preset and reader state it (since 0.7.1 the KDE cell gives a value too, which kde-breeze does not state)"; the citations that follow stay. Append:

````markdown
      *Issue #4 (2026-11-13):* KDE has the same gap: its unfocused selection is #c2e0f5
      / #1b4155 under the window text (`docs/platform-facts.md:1061`). Adding the text
      colour for both platforms is *KDE: what a Qt application under Breeze shows that
      native-theme does not (issue #4)*, "The selection of a view without the focus";
      this entry keeps the macOS research.
````

- **`:1173-1183`, *Model: the menu bar has no fill of its own*.**

````markdown
      *Issue #4 (0.7.1):* Breeze fills a hovered or open menu bar title with the opaque
      hover or focus colour under `HighlightedText` (`breezestyle.cpp:5841-5948`); the
      connectors drew it from `menu.hover_background` until 0.7.1, which now holds the
      item's 30 % tint, so a title's fill joins *KDE: what a Qt application under Breeze
      shows that native-theme does not (issue #4)*, "What Breeze draws that the model
      has no field for".
````

- **`:1214-1488`, *Presets: values the M8 T15 platform-facts rows disagree with*, its kde-breeze bullet (`:1272-1309`). The line goes after line 1309, the bullet's last, indented eight spaces as the bullet's continuation.**

````markdown
        *Issue #4 (0.7.1):* kde-breeze's `checkbox.hover_background`,
        `slider.thumb_hover_color` and `combo_box.hover_background` wait for Breeze's
        hover outlines, the step proposed after 0.7.1 (*KDE: what a Qt application under
        Breeze shows that native-theme does not (issue #4)*, "What Breeze draws that the
        model has no field for"): removing a hover fill first would remove the only
        hover feedback.
````

- **`:1786-1791`, the gap map's *Menu items are rounded on four platforms*.**

````markdown
      *Issue #4 (0.7.1):* KDE's cell is corrected, 4.5 (`docs/platform-facts.md:1265`);
      kde-breeze's value waits (*KDE: what a Qt application under Breeze shows that
      native-theme does not (issue #4)*, "KDE values that wait for a connector design",
      (5)).
````

- **`:2126-2134`, `[upstream]` a `ghost_hover` / `ghost_hover_foreground` token pair.**

````markdown
      *Issue #4 (0.7.1):* on KDE `accent` is now the menu's 30 % tint under window text
      (#3daee94d), so a hovered ghost button is that tint, not a selection-coloured
      pill; Breeze draws a flat button's hover as an outline
      (`breezehelper.cpp:665-712`).
````

- **`:2204-2211`, `[upstream]` the focus ring's width.**

````markdown
      *Issue #4 (2026-11-13):* see *GPUI Kit, from issue #4* below, request (4),
      keyboard-only bands; KDE's band is 2 px (`docs/platform-facts.md:1109`).
````

- **`:2258-2264`, `[upstream]` `Toggle` needs a token of its own.** Its last sentence, "KDE and macOS are unaffected (both are the selection colour)", becomes false. Append:

````markdown
      *Issue #4 (0.7.1):* KDE is affected now: kde-breeze's menu hover is the 30 % tint
      #3daee94d under window text, so a KDE toggle's hover and checked fill are that
      tint; macOS is still unaffected.
````

- **`:2379-2411`, `[upstream]` internal scrollbars reserve no gutter.**

````markdown
      *Issue #4 (0.7.1):* kde-breeze's track is transparent (#00000000, Breeze paints
      none), so the content under the 21 px bar shows through; the thumb and the bar's
      hit area still lie over it, so the gutter is still needed.
````

- **`:4110-4115`, *egui showcase: the Basic list's contents paint over its frame by fractions of a point*, its (4).**

````markdown
      *Issue #4 (0.7.1):* kde-breeze's track is transparent now, as windows-11's is: its
      +0.66 nub no longer shows.
````

- **`:1691-1704`, *KDE: Union, the Application Style that is data (issue #4)*.** No change: the first comment's subject. The new section's heading names the second comment.

### C.5 The evidence: `docs/evidence/breeze-offscreen/`

Two folders, as `docs/evidence/README.md:6-7` asks ("a later reading is a new run directory"): `2026-11-13/` holds the run the rationale reports, as it ran; `<run-date>/`, named after the day the implementer runs it, holds the readings taken at implementation time. No copy of Breeze's colour scheme files is committed in either: the `kdeglobals` files the runs read are Breeze's `colors/BreezeLight.colors` / `BreezeDark.colors` at the pinned commit plus ten appended lines, and the README gives the commands that rebuild them. The scripts are committed as they ran. Source paths below are in `<scratch>/issue4/`.

#### C.5 (a) `2026-11-13/`

| Goes in as | From | Cleaning |
|---|---|---|
| `README.md` | new, text below | — |
| `measure.py` | `measure/measure.py` | none: no absolute path (`OUT` is the script's own folder, `:21`) |
| `out_light.txt`, `out_dark.txt` | `measure/out_light.txt`, `measure/out_dark.txt` (179 lines each) | lines 8, 19, 30, 41, 53, 56, 61, 64, 89, 105, 107, 109, 111, 113, 115, 117, 119, 122, 127, 129, 131, 136, 142, 144, 154, 157 of each file read `saved /tmp/claude-1000/<session>/scratchpad/issue4/measure/<name>.png size WxH`, a scratch path with the user's home folder in it. Remove it in place, `sed -i 's#/tmp/claude-1000/[^ ]*/measure/##'`, leaving `saved <name>.png size WxH`. **Replace, never delete a line**: the facts cells cite these files by line. Line 29 of each holds Python object addresses (`QScrollArea(0x5597873c79b0) at 0x7f32ccba4400`): per process, not machine-identifying; kept, as the OS probe's evidence keeps its object addresses |
| `tree_check.py`, `tree_light.txt`, `tree_dark.txt` | `measure/` | none |
| `combo.py` | `measure/combo.py` | none (it saves `combo_<tag>.png` to the working folder). Its output was not saved; the README says so |
| `x11/qtfont.py`, `x11/shot.sh`, `x11/capheight.py` | `x11/` | none: no absolute path. `shot.sh` hard-codes display `:97` (`:5`, `:7`, `:8`, `:11`), writes the resource it loads to `x11/res` (`:7`) and reads `x11/cfg` and `x11/empty` (`:8`); the README says so. Their output was not saved |

Not committed: `measure/cfg-light/kdeglobals`, `measure/cfg-dark/kdeglobals`, `x11/cfg/kdeglobals` (the README rebuilds them; the dark file and `x11/cfg/kdeglobals` are identical); every `*.png` (the rationale's §9: "The images are not kept"); `x11/res` (state, not output); `x11/egui-144.log`, `x11/egui-96.png.log` (Mesa's DRI3 warnings about the machine's X server, no result); `measure/probe.py`, `measure/accent_check.py`, `measure/inspect_tree.py` (no saved output, and nothing cites them; `measure.py`'s line 2 repeats `probe.py`'s palette reading); the empty folders `measure/empty-dirs/` and `x11/empty/` (git keeps no empty folder; the README's commands create them).

The rebuild was checked: the pinned `BreezeLight.colors` / `BreezeDark.colors` (`breeze-a/pin/`) with the ten lines appended by the README's `printf` give files whose SHA-256 equals that of the `kdeglobals` the runs read (`af054447…` light, `c7e850c6…` dark, both written in full in the README).

The folder's `README.md`:

````markdown
# Breeze rendered offscreen, 2026-11-13

What the installed Breeze widget style painted when Qt drew real widgets offscreen,
read back pixel by pixel. Recorded for issue #4's second comment
([rationale](../../../issues/0004_kde-hand-overrides-rationale.md) §2, §9); the KDE
cells of `docs/platform-facts.md` cite the outputs by line. Run by hand on a Linux
desktop on 2026-11-13; nothing was drawn on the desktop itself. The images the scripts
write are not kept.

| | |
|---|---|
| Style | the installed Breeze 6.8.2 `breeze` widget style (`out_light.txt:1` records the style's name only) |
| Qt | Qt 6.12 through PySide6 6.12.0, platform `offscreen`, scale 1, logical DPI 96 (`out_light.txt:4`) |
| Platform theme | Plasma's, `QT_QPA_PLATFORMTHEME=kde` |
| Colours and fonts | `cfg-light/kdeglobals` and `cfg-dark/kdeglobals`, rebuilt below: Breeze's colour schemes at breeze `f0b1d753` (the same at `v6.8.2` but for translated names) with Noto Sans fonts and `widgetStyle=breeze` appended |
| Breeze's own settings | their defaults: no `breezerc`, and `XDG_CONFIG_DIRS` names an empty folder, so no system file is read |

No output records the Breeze, Qt and PySide versions: they are the installed packages'
as the rationale (§2) records them. The plasma-integration version was not recorded.

- `measure.py` wrote `out_light.txt` and `out_dark.txt`: scroll bars (a list and a plain
  scroll area, with and without the focus, at rest and hovered), a popup menu with an
  icon, a check box item, a separator and a submenu, a text-only menu, item view rows
  (hovered, selected, selected without the focus, selected and hovered), a table with
  its grid and header, a tree, push buttons, pixel metrics and font metrics. Each line
  names what it samples and where; a `saved <name>.png` line names an image that is not
  kept. The two files have the same lines in the same order.
- `tree_check.py` wrote `tree_light.txt` and `tree_dark.txt`: what a tree view paints
  between and around its rows. The 1 px lines in the dark tree with the focus
  (`out_dark.txt:143`, `tree_dark.txt:3`) are the current item's frame; without the
  focus there is no line between rows (`tree_dark.txt:5`).
- `combo.py` (a combo box's open popup, at rest and with the pointer on its third row)
  and the scripts in `x11/` (Qt's scale and a 10 pt font's size on a private X server
  with `Xft.dpi` set; the egui showcase's text there) ran too, but their printed output
  was not saved. The readings cited from them are in the later run directory beside
  this one, taken with the same scripts (`combo.py` with its text colour printed in hex).

## Rebuilding the configuration

The runs read a private `kdeglobals`: Breeze's colour scheme at the pinned commit with
these ten lines appended (a blank line, then):

```ini
[General]
font=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
menuFont=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
toolBarFont=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
smallestReadableFont=Noto Sans,8,-1,5,400,0,0,0,0,0,0,0,0,0,0,1

[KDE]
widgetStyle=breeze
```

followed by one blank line. `x11/cfg/kdeglobals` is the dark one. To rebuild all three:

```sh
B=https://raw.githubusercontent.com/KDE/breeze/f0b1d7534aa2356d7336241d0c7051522e8a6b68/colors
mkdir -p cfg-light cfg-dark x11/cfg empty x11/empty
curl -fsSL "$B/BreezeLight.colors" > cfg-light/kdeglobals
curl -fsSL "$B/BreezeDark.colors" > cfg-dark/kdeglobals
for f in cfg-light/kdeglobals cfg-dark/kdeglobals; do
  printf '\n[General]\nfont=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\nmenuFont=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\ntoolBarFont=Noto Sans,10,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\nsmallestReadableFont=Noto Sans,8,-1,5,400,0,0,0,0,0,0,0,0,0,0,1\n\n[KDE]\nwidgetStyle=breeze\n\n' >> "$f"
done
cp cfg-dark/kdeglobals x11/cfg/kdeglobals
sha256sum cfg-light/kdeglobals cfg-dark/kdeglobals
# af054447132ed59055ed2cf20acc058c2c6d64e8d6095411085f786a9d6534d7  cfg-light/kdeglobals
# c7e850c6ab7f5f9e2c2f5eea024cff9b3db155c094914c18f9128013e3951de5  cfg-dark/kdeglobals
```

## Repeating the run

From a copy of this folder outside the repository (the scripts write their images
beside themselves or into the working folder), after rebuilding the configuration:

```sh
for s in light dark; do
  export XDG_CONFIG_HOME="$PWD/cfg-$s" XDG_CONFIG_DIRS="$PWD/empty"
  export QT_QPA_PLATFORM=offscreen QT_QPA_PLATFORMTHEME=kde
  python3 measure.py "$s" > "out_$s.txt"
  python3 tree_check.py "$s" > "tree_$s.txt"
done
```

`x11/shot.sh <showcase binary> <Xft.dpi> <out.png>` starts Xvfb on display `:97` (one
1920 × 1080 × 24 screen, `-nolisten tcp`), sets `Xft.dpi` with `xrdb -nocpp -merge`,
runs the showcase with `--theme kde-breeze --variant dark --capture`,
`XDG_SESSION_TYPE=x11`, `x11/cfg` and `x11/empty` and no Wayland variables for 20 s,
and takes a screenshot with ImageMagick's `import`; it writes the resource it loads to
`x11/res`. `x11/capheight.py <png> <x0> <x1> <y0> <y1>` prints the height of the bright
rows in that box. `x11/qtfont.py` prints Qt's device pixel ratio, logical DPI and the
pixel size of a 10 pt Noto Sans.
````

#### C.5 (b) `<run-date>/`: the readings taken at implementation time

The implementer takes three readings, each with the scripts of `2026-11-13/` (copied into the new folder as they are run, so the folder stands alone), and commits them before the facts cells and closing texts that quote them (C.1 row 717, C.4 (a) 2), after Part A's commit. The folder is named after the date of the after-fix run (reading 3's after-fix build); readings taken on other days say their date in the README. A number those texts quote comes from this folder.

**A reading that differs from the rationale's.** For every reading below, the rationale's expected value is given. Where the file's reading differs, the file's number is the one written in the facts cell or closing text, and the difference is reported to the maintainer before the commit. A number the file does not hold is not written.

1. **The combo box popup.** `combo.py` with its text colour printed in hex: lines 19-20 of the 2026-11-13 copy are replaced by the four below (the rest unchanged). The rationale's §9 expects the hovered row's fill #b9dcee / #284c61 and its text #232629 / #fcfcfc (the darkest pixel in light, the lightest in dark).

```python
        cols = [img.pixelColor(x, yy) for x in range(r.left()+4, r.left()+60) for yy in range(r.top()+3, r.bottom()-2)]
        darkest = min(cols, key=lambda c: c.lightness()).name()
        lightest = max(cols, key=lambda c: c.lightness()).name()
        print(tag, "row", row, "rect", r.x(), r.y(), r.width(), r.height(), "fill", bg, "text darkest", darkest, "lightest", lightest, "current" if v.currentIndex().row()==row else "")
```

Run, after rebuilding the configuration as the 2026-11-13 README says:

```sh
mkdir -p empty
for s in light dark; do
  env -i HOME="$HOME" PATH="$PATH" QT_QPA_PLATFORM=offscreen QT_QPA_PLATFORMTHEME=kde \
    XDG_CONFIG_HOME="$PWD/cfg-$s" XDG_CONFIG_DIRS="$PWD/empty" python3 combo.py > "combo_$s.txt"
done
```

2. **Qt on X11.** `x11/qtfont.py` at `Xft.dpi` 96, 120, 144 and 192, its output to `x11/qtfont-out.txt`. It runs under `env -i` with only the variables named, so that no inherited variable changes Qt's scale or DPI: `QT_SCALE_FACTOR` multiplies Qt's scale, a `QT_SCREEN_SCALE_FACTORS` factor takes precedence over the factor from `Xft.dpi` (`qhighdpiscaling.cpp:598-601`), `QT_FONT_DPI` overrides the screen's DPI (`qplatformscreen.cpp:157`, read through `QPlatformScreen::overrideDpi`, `qhighdpiscaling.cpp:252`), and `QT_ENABLE_HIGHDPI_SCALING`, `QT_SCALE_FACTOR_ROUNDING_POLICY` and `QT_USE_PHYSICAL_DPI` change how Qt derives it (`qhighdpiscaling.cpp:24`, `:27`, `:29`).

```sh
mkdir -p x11/empty
Xvfb :97 -screen 0 1920x1080x24 -nolisten tcp -noreset >/dev/null 2>&1 & XP=$!
sleep 1
for d in 96 120 144 192; do
  printf 'Xft.dpi: %s\n' "$d" | DISPLAY=:97 xrdb -nocpp -merge -
  echo "Xft.dpi $d"
  env -i HOME="$HOME" PATH="$PATH" DISPLAY=:97 QT_QPA_PLATFORM=xcb QT_QPA_PLATFORMTHEME=kde XDG_CONFIG_HOME="$PWD/x11/cfg" XDG_CONFIG_DIRS="$PWD/x11/empty" \
    python3 x11/qtfont.py
done > x11/qtfont-out.txt
kill $XP
```

The rationale (§4.1, §9) expects device pixel ratios 1, 1.25, 1.5 and 2, a logical DPI of 96 and 13 px at every value.

3. **The egui showcase's capitals, before and after the fix.** The showcase is the example `showcase-egui` (`connectors/native-theme-egui/Cargo.toml:94-98`), built at `387fc449` (before) and at the commit that lands Part A's change (after). The before binary is built first, at the branch's start (`387fc449`) before any source edit lands, and copied out of `target/` to a place of its own, so that no commit is checked out again later; the after binary is built once Part A's commit is in. Each is built with `cargo build -p native-theme-egui --example showcase-egui --release --all-features` (the binary is `target/release/examples/showcase-egui`). For each build and each `Xft.dpi` of 96 and 144: `env -u WINIT_X11_SCALE_FACTOR XDG_CURRENT_DESKTOP=KDE x11/shot.sh <binary> <dpi> egui-<dpi>-<label>.png`. `shot.sh` passes the caller's environment on except the Wayland variables (`x11/shot.sh:8`): `WINIT_X11_SCALE_FACTOR` would override winit's scale in both builds, and the before build at `387fc449` takes a fixed 96 where the refined desktop is GNOME or Budgie (`native-theme/src/detect.rs:612-614`, `:649-653`), so the desktop is set to KDE. Then `x11/capheight.py` on each image, with this box: `x0..x1` covers only capital letters without descenders or accents (for example H, E, T, L, I) of one label set in the body font (`defaults.font`'s size, not a heading); `y0..y1` covers that label's line with 2 to 4 px to spare above and below, and no other text lies inside the box (`capheight.py` takes each row's brightest pixel across the whole x-range, `x11/capheight.py:5-8`). The 144 screenshot's box covers the same letters of the same label at their own position, and the same label serves both builds. `x11/capheight-out.txt` holds, for each image, the build's commit, the `Xft.dpi`, the label, the letters, the box and the printed line. The images and `shot.sh`'s `*.png.log` files (Mesa's warnings about the X server) are not kept. The rationale expects 10 px at 96 and 22 px at 144 before the fix, 14 to 15 px at 144 after it.

Its `README.md` (fields in angle brackets filled at the run):

````markdown
# Breeze and the font DPI, readings of <run-date>

The readings issue #4's second comment needs that the run of
[`2026-11-13/`](../2026-11-13/) did not save: a combo box popup's colours, Qt's scale on
an X server with `Xft.dpi` set, and the egui showcase's text there before and after
0.7.1 made the font DPI 96 on every Linux desktop. Taken with that run's scripts,
copied here as they ran (`combo.py` prints its text colour in hex), against the
`kdeglobals` files that run's README rebuilds. The images are not kept.

| | |
|---|---|
| Date | <run-date> |
| native-theme | before: `387fc449`; after: `<sha>` (the egui showcase built at each) |
| Versions | Breeze <v>, plasma-integration <v> (the distribution's package query); PySide6 and Qt as printed by `python3 -c "import PySide6, PySide6.QtCore as c; print(PySide6.__version__, c.qVersion())"`: <v>, <v>; Xvfb <v>, xrdb <v>, ImageMagick <v> |
| Environment | from a copy of this folder outside the repository. `combo.py` and `x11/qtfont.py` under `env -i` with `HOME`, `PATH` and only: `QT_QPA_PLATFORM` (`offscreen`, `xcb`), `QT_QPA_PLATFORMTHEME=kde`, `XDG_CONFIG_HOME`, `XDG_CONFIG_DIRS` (an empty folder), and for `qtfont.py` `DISPLAY=:97`; `x11/shot.sh` under `XDG_CURRENT_DESKTOP=KDE` with `WINIT_X11_SCALE_FACTOR` unset (it sets `XDG_SESSION_TYPE=x11`, `x11/cfg`, `x11/empty` and drops the Wayland variables itself). Xvfb on display `:97`, 1920 × 1080 × 24 |

- `combo_light.txt`, `combo_dark.txt`: `combo.py` under `cfg-light` / `cfg-dark`,
  `QT_QPA_PLATFORM=offscreen`, `QT_QPA_PLATFORMTHEME=kde`, `XDG_CONFIG_DIRS` an empty
  folder.
- `x11/qtfont-out.txt`: `x11/qtfont.py` at `Xft.dpi` 96, 120, 144 and 192.
- `x11/capheight-out.txt`: the showcase's capitals at `Xft.dpi` 96 and 144, before and
  after, each line with the build's commit and the box.

<the exact commands of C.5 (b) 1-3, the build commands included, and each reading's date where it is not <run-date>>
````

**`docs/evidence/README.md`.** Its first paragraph (lines 3-9) speaks of workflow runs and runner images. Its first sentence, "Recorded outputs that `docs/platform-facts.md` and the presets cite, kept in the repository because GitHub deletes a workflow run's artifacts after 90 days.", becomes "Recorded outputs that `docs/platform-facts.md` and the presets cite, kept in the repository: GitHub deletes a workflow run's artifacts after 90 days, and a run by hand leaves no record anywhere else." Its second, "Each run directory holds that run's artifacts and a `README.md` of its own that states the runner images and what the run reported.", becomes "Each run directory holds that run's artifacts and a `README.md` of its own that states what produced them (the runner images, or the machine's versions for a run by hand) and what the run reported." The paragraph re-wraps. Appended after the `os-probe/` section (after line 30):

````markdown
## `breeze-offscreen/`

What the installed Breeze style painted when Qt drew its widgets offscreen, read back
from the pixels, and how Qt and the egui showcase size a font on a private X server with
`Xft.dpi` set; runs by hand, one directory per date, with the scripts and their printed
output. Breeze's colour scheme files are not copied; each README says how to rebuild
the configuration the run read.

| Run | Date | Why |
|-----|------|-----|
| [`2026-11-13/`](breeze-offscreen/2026-11-13/) | 2026-11-13 | issue #4's second comment ([rationale](../issues/0004_kde-hand-overrides-rationale.md) §9): the menu's, the list's and the scroll bar's colours, the header, the grid, the default button, the menu's metrics and the message icon in the facts' KDE cells |
| [`<run-date>/`](breeze-offscreen/<run-date>/) | <run-date> | the readings 2026-11-13 did not save: the combo box popup, Qt's scale on X11, and the egui showcase's text before and after the font DPI of 0.7.1 |

A citation links the file and its line (`evidence/breeze-offscreen/2026-11-13/out_light.txt#L27`).
````

### C.6 `ROADMAP.md`

This part owns every `ROADMAP.md` edit (ruling 1); Parts A and D drop theirs, and this text takes in what they had (Part A's: no longer drawing text at the display's scale twice on a scaled KDE session or an X11 session with `Xft.dpi` above 96; Part D's: the README now covers overriding theme values, the focus ring and what only gpui-component can change).

The heading at `ROADMAP.md:85`, "## v0.7.1 — gpui connector on GPUI Kit 0.9.1", no longer covers the section; it becomes "## v0.7.1 — GPUI Kit 0.9.1, and the KDE fixes from issue #4". The **Delivered** paragraph is `:87-94` (it ends "The showcase shows 0.9.1's `SpeechIndicator`."). Append to it, after its last sentence, re-wrapped at the file's width:

````markdown
It also fixes what a KDE application's author found overriding by hand
(issue #4). The font DPI is 96 on every Linux desktop, so a scaled KDE session
and an X11 session with `Xft.dpi` above 96 no longer draw text at the display's
scale twice. A hovered menu item and a hovered list row are tinted as Breeze
tints them, at 30 % under the normal text; the menu separator and the table
header take Breeze's colours, the scroll bar's resting thumb is Breeze's grey
with no track behind it, and the message box icon is 64 px. The gpui connector
restores the three selection colours gpui-component caps after a mode switch,
makes `to_hsla` public, and its README covers overriding theme values, the focus
ring, and what only gpui-component can change. The platform facts for the focus
ring, the default button, the menu panel and the table's grid are corrected too,
and their values wait for a connector design; the selection of an unfocused view
waits for a text colour the model does not have yet. The next step proposed is
Breeze's hover outlines: Breeze shows hover on a button, a combo box, a check
box and a slider handle by recolouring the outline, which the model can state
for text inputs only.
````

Every clause is one of the rationale's decisions (§4.1–§4.6, §4.8, §4.9, §5.1–§5.3, §5.5, §5.6) or its proposal (§5.4). "Tinted as Breeze tints them, at 30 %" covers both: the menu's hover is the focus colour (`DecorationFocus`) at 0.3, the list's the selection colour (`Highlight`) at 0.3.

### C.7 Problems found

1. **The rationale's §6 missed existing entries; the specification's decision 5 settles them.** *KDE's dialog icon* closes (C.4 a 3); *kde-breeze: the Qt Widgets menu fill* folds into new entry 3 (C.4 a 4); the macOS selection entry is cross-referenced from new entry 4 and its "Only macOS states it" corrected (C.4 c). Four sentences the rationale does not name go stale and are handled in C.4 (c): `todo.md:2263-2264`, `:2387`, `:4115`, and the `ghost_hover` entry at `:2126`.
2. **Several `docs/todo.md` citations were stale before this work** (C.4 a 12): `:2704` cites `kde/mod.rs:42-52` and `detect.rs:674-685` for the reduced-motion readings, which are at `kde/mod.rs:277-283` and `detect.rs:976-988` at `387fc449`; `:1036`, `:1104`, `:1625` and `:1649` are two to six lines off. C.4 (a) 12 re-points them by name with Part A's edit.
3. **Unconfirmed repository paths and tags** (C.1, "Checked"): kwin's `src/` and every qtbase path are the repositories' layout as known, not read from a listing; the downloaded Qt files carry no version stamp, and the two scratch copies of `qcommonstyle.cpp` differ, so the rationale's tags (v6.10.1, v6.11.0) are taken on its word. KStyle is linked at tag v6.31.0, whose numbering the downloaded `kstyle.cpp` matches; the facts file's other KStyle link is at `178c522c`, and the rationale names `733073d3`. The implementer opens each of these links once.
4. **`[per_platform."input.caret_color"]` `kde = "defaults.focus_ring_color"`** ties the caret to the ring: harmless while kde-breeze keeps the ring opaque, wrong the day it takes #3daee94d. Recorded in new entry 3 (1); no change now.
5. **The dark tree view shows 1 px lines in the main run** (`out_dark.txt:143`, #23262a in the focused tree). `tree_check.py` shows them to be the current item's frame, present with the focus (`tree_dark.txt:3`) and gone without it (`:5`), so §9's "Lines between rows of a tree or list: none" holds; the light tree shows no such line at the same column (`tree_light.txt:3`), which nothing explains. The grid cell cites the unfocused reading.


## Part D — gpui connector

Implements `docs/issues/0004_kde-hand-overrides-rationale.md` §4.6, §4.7, §4.8
and §4.9, with the lines of §4.10 that concern them (the CHANGELOG entries for
the restore, `to_hsla` and the README additions; `to_hsla`'s rustdoc example;
the selected cell after a mode switch).
Worktree `.claude/worktrees/v071`, branch `v0.7.1-rc2` at `387fc449`. Every
line number below is at that commit; upstream lines are in the registry copies
of gpui-component 0.9.1, gpui-base 0.9.1, gpui-pre 0.3.12 and gpui-pre-linux
0.3.12. Where two parts edit one file (Part A edits the gpui README at
`:621-623`), apply each edit by its quoted text: the line numbers move.

Files this part changes: `connectors/native-theme-gpui/src/colors.rs`,
`src/lib.rs`, `src/geometry.rs`, `README.md`; `scripts/connector_parity_exceptions.toml`;
`CHANGELOG.md` (its own entries only); and one new document,
`docs/issues/0004_kde-hand-overrides-upstream-requests.md`.

**Ownership** (rulings after review round 1). This part owns, in
`connectors/native-theme-gpui/`: `src/geometry.rs` (`list_item_selected`'s doc),
`src/lib.rs`, `src/colors.rs` (the comment at `:86-88` included) and `README.md`
(of the `scrollbar_gutter` row, one sentence: Part B's wording, §3.2); and the
upstream-requests file, which Part C's backlog entry 1 links. Not this part's:
`ROADMAP.md` (owned by Part C, §C.6), the gpui `src/variants.rs` comment (owned
by Part B), the gpui README's font-DPI sentence at `:621-623` (Part A, A43), and
the `### Fixed` heading of the CHANGELOG (the plan assigns it).

**A constraint on `colors.rs`.** The gpui showcase's colour claims cite
`native-theme-gpui/colors.rs` by line, 281 to 718 (for instance
`examples/showcase-gpui/info/theme_map.rs:65`, `"native-theme-gpui/colors.rs:281"`),
and `every_colour_claim_is_read_at_the_line_it_cites` (`src/showcase.rs:2819`)
fails when a cited line no longer names its field. That test pins
`variants.rs` lines 62-64, `geometry.rs` lines 230-772 and `colors.rs` lines
281-718, and nothing in `lib.rs` (measured in the worktree by the coordinator);
the `list_item_selected` edit at `geometry.rs:1394-1400` lies below the pinned
range and may change its line count. So no line is added to or removed from
`colors.rs` above line 720: the new items go between the end of
`assign_base_colors` (`}` at line 719) and `#[cfg(test)]` (line 721), and the
imports they need are already there (`Hsla` at :11; `Theme`, `ThemeToken` at
:12; `ResolvedTheme` at :13). Two places above line 720 are edited in place,
each keeping its line count: `:61` (1.2) and the comment at `:86-88` (1.2).

**Ungated citations of `lib.rs`.** This part's `lib.rs` documentation edits
change line counts (for example "Single-Mode Behavior", lines 77-81, grows from
five lines to eight), so they move four comments in the egui connector that
cite the gpui `lib.rs` by line: `connectors/native-theme-egui/src/atlas.rs:695`
(`lib.rs:345`, `:347`), `plugin.rs:18` (`:354`), `ext.rs:208` (`:459`) and
`ext.rs:412` (`:578`). No gate reads them (two are already off today). Part E's
sweep (E.2), after the last source edit, re-points them by the item each names;
this part does not edit them.

**Rule for the 2^24-iteration test** (test 3 of 1.5). The implementer measures
its run time in the `native,watch` test leg and reports the number. Up to 2 s it
stays an ordinary test. Above 2 s it is marked `#[ignore]` with that reason and
runs in the full `scripts/check_release.sh` only, as the core's ignored
stem-index test does (`check_release.sh:685-700`: skipped under `--commit`, run
as `cargo test -p native-theme --features "$NT_FEATURES" --lib -- --ignored
freedesktop::tests::indexed_lookup_matches_the_unfiltered_lookup_on_installed_themes`
at `:697-699`): a new step beside it, skipped under `--commit` in the same way,
runs `cargo test -p native-theme-gpui --lib -- --ignored
colors::tests::every_8_bit_colour_survives_hsla_to_hex`.

### 1. The restore of the three capped colours (§4.6)

#### 1.1 The data

**The three native colours.** For the stored variant of the styled theme's
current mode (`NativeTheme::variant(NativeTheme::is_dark(cx))`, `lib.rs:975-991`),
the colours `to_theme_color` installs, each through `colors::rgba_to_hsla`
(`colors.rs:20-29`):

| Field | Model field | Where `to_theme_color` assigns it |
|---|---|---|
| `selection` | `input.selection_background` | `colors.rs:219`, `:296` |
| `list_active` | `list.selection_background` | `colors.rs:257`, `:417` |
| `table_active` | `list.selection_background` | `colors.rs:430` (`tc.table_active = tc.list_active`) |

`input.selection_background` inherits `defaults.text_selection_background`,
which inherits `defaults.selection_background` (`docs/inheritance-rules.toml:175`,
`:59`); `list.selection_background` inherits `defaults.selection_background`
(`:242`).

**The caps** (gpui-component 0.9.1 `ThemeColor::apply_config`,
`src/theme/schema.rs:1039-1070`): `list_active` 0.2 (`:1053-1058`),
`table_active` 0.2 (`:1059-1064`), `selection` 0.3 (`:1065-1070`). The closure
at `:1040-1051` sets the field to `color.alpha(color.a.min(max))` and the token
to `ThemeToken::new(that colour, background)`, where `background` is the
config's string parsed with every stop's alpha clamped
(`try_parse_background_clamped`, `src/theme/color.rs:777-791`). `apply_config`
runs from `Theme::edit` on every mode change or reload (`src/theme/mod.rs:332-334`):
`Theme::change` (`:398-408`), `Theme::sync_system_appearance` (`:363-372`), a
`ThemeRegistry` reload (`src/theme/registry.rs:46-70`) and `init` (`mod.rs:36-42`).

**The test "holds upstream's capped native colour".** Let `N` be a native colour
and `cap` its cap. The field holds the cap of `N` when all three hold:

1. `N.a > cap` (with `N.a <= cap` the cap leaves `N` as it is, `:1041-1043`,
   and there is nothing to undo);
2. the plain field (`theme.colors.<field>`, a `gpui::Hsla`) equals `N.alpha(cap)`;
3. the token (`theme.tokens.<field>`, a `gpui_component::theme::ThemeToken`)
   equals `ThemeToken::from(N.alpha(cap))`.

Equality is `Hsla`'s `PartialEq` (gpui-pre `src/color.rs:374-382`: `total_cmp`
on `h`, `s`, `l`, `a`, so bit for bit) and `ThemeToken`'s derived `PartialEq`
(`src/theme/theme_color.rs:10-15`: `color` and `background`, `Background`
deriving `PartialEq`, gpui-pre `src/color.rs:777-786`).

`Hsla::alpha(a)` sets `a.clamp(0., 1.)` and keeps `h`, `s`, `l`
(`src/color.rs:667-674`); upstream's `color.alpha(target)` with
`target = base.min(max)` and `base > max` is `color.alpha(max)`, the same
`f32` literal (`0.3`, `0.2`) the connector passes. The token: for a hex string
the clamped background is `color.alpha(color.a.min(max)).into()`
(`color.rs:778-779`), `From<Hsla> for Background` (gpui-pre `src/color.rs:950-958`:
tag `Solid`, the colour, the rest default), which is exactly what
`ThemeToken::from(colour)` builds (`theme_color.rs:31-38`). So the capped token
is `ThemeToken::from(P.alpha(cap))`, where `P` is the colour upstream parsed
from the config.

**Why `P == N` bit for bit.** The connector's config holds each colour as
`hsla_to_hex(tc.<field>)` (`config.rs:182`, `:204`, `:231`; `hsla_to_hex`,
`colors.rs:59-73`): `Hsla` → `gpui::Rgba` (gpui-pre `src/color.rs:194-221`),
each channel `(v.clamp(0, 1) * 255).round() as u8`, written `#rrggbb` when the
alpha byte is 255 and `#rrggbbaa` otherwise. Upstream parses it with
`try_parse_theme_token` → `try_parse_theme_color` → `try_parse_color`
(`schema.rs:14-19`, `color.rs:693-697`, `:793-799`), which for a `#` string is
`gpui::Rgba::try_from` (gpui-pre `src/color.rs:224-329`: `r = byte as f32 / 255.`,
alpha `0xff` for `#rrggbb`) and then `From<Rgba> for Hsla` (`:677-713`). The
native colour was made by the same two steps from the model's bytes:
`Rgba::to_f32_array` is `byte as f32 / 255.0` (`native-theme/src/color.rs:128-135`),
the clamp at `colors.rs:22-27` is a no-op on those values, then
`From<Rgba> for Hsla`. Same bytes in, same `f32` operations, same `Hsla` out,
provided `hsla_to_hex` gives back the bytes the native colour was made from.
It does: the `Rgba` → `Hsla` → `Rgba` round trip in `f32` lands each channel
within a few units in the last place of `byte / 255`, far inside the ±0.5/255
that `round()` tolerates, and the alpha passes through both conversions
untouched (`a: color.a`). That last step is reasoned, not proven for every
colour; test 3 of 1.5 checks it exhaustively for all 2^24 opaque colours and all
256 alphas, and test 5 checks the whole path through upstream's own parser.
If either test ever fails, the spec's comparison is not exact for that colour:
the restore then leaves the capped value in place (it compares, it never
guesses), and the fix is to compare against
`gpui_component::try_parse_color(&hsla_to_hex(N))` capped instead of
`N.alpha(cap)`; report it rather than loosening the comparison.

#### 1.2 The new items

In `colors.rs`, inserted after line 720 (see the constraint above):

```rust
/// One channel of [`hsla_to_hex`]: `v` clamped to `0.0..=1.0`, times 255,
/// rounded to the nearest byte.
pub(crate) fn channel_byte(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// The alpha gpui-component caps `selection` at whenever it applies a
/// `ThemeConfig` (gpui-component 0.9.1 `src/theme/schema.rs:1065-1070`).
pub(crate) const SELECTION_ALPHA_CAP: f32 = 0.3;

/// The alpha gpui-component caps `list_active` and `table_active` at whenever
/// it applies a `ThemeConfig` (gpui-component 0.9.1
/// `src/theme/schema.rs:1053-1064`).
pub(crate) const ROW_SELECTION_ALPHA_CAP: f32 = 0.2;

/// The native colours of the three `ThemeColor` fields gpui-component caps, as
/// [`to_theme_color`] assigns them: `selection` from
/// `input.selection_background`, `list_active` and `table_active` from
/// `list.selection_background`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CappedNatives {
    selection: Hsla,
    list_active: Hsla,
    table_active: Hsla,
}

/// [`CappedNatives`] of `resolved`.
pub(crate) fn capped_natives(resolved: &ResolvedTheme) -> CappedNatives {
    let row = rgba_to_hsla(resolved.list.selection_background);
    CappedNatives {
        selection: rgba_to_hsla(resolved.input.selection_background),
        list_active: row,
        table_active: row,
    }
}

/// Whether a field and its token hold what gpui-component's cap makes of
/// `native`: `native` at alpha `cap`, and that colour as a solid token. Never
/// where `native`'s alpha is at or below the cap, which leaves it as it is.
fn holds_upstream_cap(color: Hsla, token: ThemeToken, native: Hsla, cap: f32) -> bool {
    if native.a <= cap {
        return false;
    }
    let capped = native.alpha(cap);
    color == capped && token == ThemeToken::from(capped)
}

/// Write the native colour, and its token, into each of the three fields of
/// `to` that holds gpui-component's cap of it ([`holds_upstream_cap`]); any
/// other value is left alone. `true` when any field was written.
pub(crate) fn restore_capped(native: &CappedNatives, to: &mut Theme) -> bool {
    let mut restored = false;
    macro_rules! restore {
        ($($field:ident: $cap:expr),*) => {$(
            if holds_upstream_cap(to.colors.$field, to.tokens.$field, native.$field, $cap) {
                to.colors.$field = native.$field;
                to.tokens.$field = ThemeToken::from(native.$field);
                restored = true;
            }
        )*};
    }
    restore!(
        selection: SELECTION_ALPHA_CAP,
        list_active: ROW_SELECTION_ALPHA_CAP,
        table_active: ROW_SELECTION_ALPHA_CAP
    );
    restored
}
```

In place, in `colors.rs` above line 720 (each keeps its line count):

- `:61`, in `hsla_to_hex`, today
  `    let channel = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;`, becomes
  `    let channel = channel_byte;`, so test 3 of 1.5 tests the rule
  `hsla_to_hex` really uses (round-1 review, finding 12).
- `:86-88`, the comment above `menu_hover_bg`, which becomes false when Part B
  gives KDE's menu hover the focus colour at alpha 0.3 (rationale §4.2). Today:

  ```rust
      // The item highlight gpui-component calls `accent` (menu, list, completion
      // and calendar hover): the platform's menu hover pair, which is the
      // selection colour on KDE and macOS and a subtle fill elsewhere.
  ```

  New (three lines, under 100 columns):

  ```rust
      // The item highlight gpui-component calls `accent` (menu, list, completion
      // and calendar hover): the platform's menu hover pair, which is the selection
      // colour on macOS, a tint of the focus colour on KDE, and a subtle fill elsewhere.
  ```

The natives are read with `rgba_to_hsla` from the same model fields
`to_theme_color` reads, not by calling `to_theme_color` (which builds all 139
fields on every delivery); test 1 of 1.5 holds the two equal over every preset.

In `lib.rs`, a private function after `repair_base_palette` (which ends at
line 1438) and before `install_observer_once`'s doc (line 1440):

```rust
/// Restore `selection`, `list_active` and `table_active` of the styled theme,
/// with their tokens, from the stored variant for the current mode, where
/// each holds what gpui-component's alpha cap made of it; `true` when any was
/// restored.
///
/// gpui-component caps their alpha at 0.3, 0.2 and 0.2 whenever it applies a
/// `ThemeConfig` (gpui-component 0.9.1 `src/theme/schema.rs:1039-1070`, a
/// colour the config states included, `src/theme/color.rs:777-791`), so after
/// every `Theme::change`, `sync_system_appearance` or registry reload they
/// differ from what [`apply`] installed. A field is restored only when it holds
/// exactly that cap of the native colour (`colors::restore_capped`); any other
/// value is the application's and stays. A value the application sets to
/// exactly the capped native colour cannot be told from the cap and is
/// replaced too.
///
/// The write goes through `Theme::update`, which keeps the tokens in step and
/// ends in `sync_base`: gpui-base keeps copies of `selection` in its colour
/// tokens (gpui-component `src/theme/mod.rs:547`) and in the text view's
/// defaults (`src/text/mod.rs:54`), and paints selected text from the first
/// (gpui-base 0.9.1 `src/selectable_text.rs:221-223`). `sync_base` resets the
/// base overrides, so a caller writes them after a restore. A copy is probed
/// first, as in [`repair_base_palette`], because `Theme::update` notifies the
/// base theme's observers even when nothing changed.
fn restore_capped_colors(cx: &mut App) -> bool {
    let Some(nt) = cx.try_global::<NativeTheme>() else {
        return false;
    };
    let is_dark = nt.is_dark(cx);
    let Some(resolved) = nt.variant(is_dark) else {
        return false;
    };
    let native = colors::capped_natives(resolved);
    let capped = cx.try_global::<GpuiTheme>().is_some_and(|styled| {
        let mut probe = styled.clone();
        colors::restore_capped(&native, &mut probe)
    });
    if !capped {
        return false;
    }
    GpuiTheme::update(cx, |styled| colors::restore_capped(&native, styled))
}
```

What each returns: `channel_byte` one byte; `capped_natives` the three colours; `holds_upstream_cap`
whether one field holds the cap; `restore_capped` and `restore_capped_colors`
whether anything was written. No new item is public. `restore_capped_colors`
calls `Theme::update` only after `try_global::<GpuiTheme>()` found the styled
theme (`Theme::update` reaches `cx.global_mut`, which panics on a missing
global, `src/theme/mod.rs:260-262`).

#### 1.3 The write path

**`Theme::update`, not `Theme::global_mut`.**

| | `Theme::global_mut` (`src/theme/mod.rs:252-262`) | `Theme::update` (`:264-299`, `edit` `:307-345`) |
|---|---|---|
| tokens | untouched: the caller writes them | reconciled after the closure: a field changed on `colors` gets the solid token of its new colour, unless the token already names it (`theme_color.rs:373-388`) |
| `sync_base` | not called: gpui-base's `tokens.colors.selection` and `TextViewDefaults` keep the capped `selection` until something else syncs | called (`mod.rs:342`): rebuilds `gpui_base::Theme` (`:481-486`) and the text view's defaults (`:485`, `src/text/mod.rs:69-81`) |
| config | nothing | nothing here: the mode does not change and `reload_mode` is false, so `apply_config` is not run (`:331-335`); the fonts do not change, so they are not resolved again (`:336-341`) |
| notifications | `NotifyGlobalObservers` for the styled `Theme`, which nothing upstream observes (only `ThemeRegistry` and `gpui_base::Theme` are observed: `registry.rs:46`, gpui-base `src/input/base/blink_cursor.rs:101`) | the same, plus one for `gpui_base::Theme` (`set_global`, gpui-pre `src/app.rs:2577-2581`) and one for `TextViewDefaults`, and `refresh_windows` (`mod.rs:343`) |

The restore needs all three copies of `selection` written, so it needs
`sync_base`; `Theme::update` is upstream's documented path for that
("the write path that cannot drift", `mod.rs:264-279`) and is the path the
README tells applications to use. `global_mut` would need the tokens written
by hand, an explicit `Theme::sync_base(cx)` and `cx.refresh_windows()`: the same
effects in more code. `restore_capped` writes the token as well, so the
reconcile keeps it (its colour already equals the field) and the function is
also correct on a theme that is not the global one (the probe, the unit tests).

The `gpui_base::Theme` notification `sync_base` queues is deduplicated with any
other write of that global before the next delivery (`push_effect`,
gpui-pre `src/app.rs:2117-2131`; the pending mark is cleared as a delivery
starts, `:2292-2297`). Calling `Theme::update` on every delivery would loop
for ever; the probe makes the second delivery find nothing to restore.

**Order.** Repair the base palette (`repair_base_palette`, `global_mut`, no base
notification), then restore the three (`Theme::update`, which syncs the base
layer), then `write_base_overrides`, because `sync_base` replaced gpui-base's
scrollbar styles and resize handles with upstream's projection
(`mod.rs:420-461`).

**The observer closure**, replacing its body at `lib.rs:1459-1486`
(`cx.observe_global::<gpui_base::Theme>(|cx| { … })`):

1. `repair_base_palette(cx);` (today :1460-1462, unchanged with its comment)
2. `install_variant_tooltip_delay(cx);` (today :1463-1467, unchanged). It stays
   before the restore so that a delivery that returns at step 3 still installs
   the new mode's delay.
3. New: `if restore_capped_colors(cx) { write_base_overrides(cx, true); return; }`
   with the comment: "Before the branch below: a rebuild merged into the
   notification of this observer's own write caps the three too, and the flag
   alone would swallow it (spec §3.3, limit). The restore ended in `sync_base`,
   which reset the base overrides; this write's notification, merged with the
   sync's, is the one further delivery, and `reapplying` absorbs it."
   `reapplying`: `write_base_overrides(cx, true)` sets it (`lib.rs:1384-1386`).
   A restore needs a stored variant for the current mode, so
   `base_overrides_for` returns `Some` (`:1349-1354`) and the flag is always set
   here. If the flag was already true (this delivery is the observer's own,
   merged with a rebuild), it stays true: exactly one notification is again
   pending, the new write's.
4. The `reapplying` branch, `lib.rs:1468-1484`, unchanged: clear the flag;
   re-write if the handle colours no longer hold the native values; return.
5. `write_base_overrides(cx, true);` (`:1485`, unchanged).

**The deferred re-write stays as it is** (`lib.rs:1488-1494`: the comment and
`write_base_overrides(cx, false); repair_base_palette(cx);`). It runs after the
observer's activation (effects are FIFO, and `install_observer_once` queues it
after `observe_global`'s activation, gpui-pre `src/app.rs:2602-2615`, `:2613`),
and its unmarked write always queues a `gpui_base::Theme` notification when a
restore is possible (a restore needs a stored variant, and then
`base_overrides_for` returns `Some`, `lib.rs:1349-1354`). The active observer
receives that notification, and that delivery performs the restore (step 3)
and a marked write; the next delivery is absorbed. Putting the restore into the
deferred closure, in any order, ends in the same state after the same two
deliveries, so it would add code that no test can observe (round-1 review,
finding 6). The rationale's sentence "the deferred re-write does the same in
the same order" is corrected by the orchestrator, not here.

#### 1.4 The five sequences

Notation, for a stored variant `X` (`L` light, `D` dark): `s_X` =
`rgba_to_hsla(X.input.selection_background)`, `r_X` =
`rgba_to_hsla(X.list.selection_background)`; `ŝ_X` = `s_X.alpha(0.3)`,
`r̂_X` = `r_X.alpha(0.2)`; `T(c)` = `ThemeToken::from(c)`. "Base" is
`gpui_base::Theme::global(cx).tokens.colors.selection`; "text view" is
`gpui_base::text::editor_host::RenderContext::from_app(cx).style().selection()`
(the public reader of the installed `TextViewDefaults` style, gpui-base
`src/text/editor_host.rs:119-134`, `:177-180`, `src/text/style.rs:292-295`;
`TextViewDefaults::style` itself is `pub(crate)`, `src/text/text_view.rs:51-53`).
"Delivery" is a delivery of a `gpui_base::Theme` notification to the active
observer. The tests use macos-tahoe, whose four colours are distinct and opaque:
`s_L` #b3d7ff, `r_L` #0064e1, `s_D` #3f638b, `r_D` #0059d1
(`native-theme/src/presets/macos-tahoe.toml:55`, `:52`, `:468`, `:465`, in that
order: `text_selection_background` and `selection_background` of each variant).

**State after an `apply_system_theme` (or two `apply`s) with mode M current**,
the starting point of (a) to (c). In the update of the first `apply`: the
styled theme is written directly (`lib.rs:1293-1299`), the three are `s_M`,
`r_M`, `r_M` with tokens `T(·)`; the other mode's config is installed
(`:1301-1320`); `sync_base` gives base and text view `s_M`. The flush delivers
the `apply`'s base notification to nobody (the observer activates only at the
end of that flush), then activates the observer, then runs the deferred
re-write: unmarked write, palette unchanged → delivery 1: nothing capped, not
reapplying, marked write → delivery 2: absorbed. Two deliveries, as today.

(a) **`apply_system_theme`, then `Theme::change` to the other mode O.**
`Theme::edit` sets the mode, applies the O config (`mod.rs:332-334`): the
three become `ŝ_O`, `r̂_O`, `r̂_O` with tokens `T(ŝ_O)`, `T(r̂_O)`, `T(r̂_O)`;
the 12 base-palette colours become upstream's constants; `sync_base` gives
base and text view `ŝ_O`. Delivery 1: palette repaired; O's tooltip delay
installed; all three hold the cap → `Theme::update` writes `s_O`, `r_O`, `r_O`
with `T(·)`, `sync_base` gives base and text view `s_O`; marked write; return.
Delivery 2: nothing capped; `reapplying` → cleared; handle colours native →
return. **End:** fields `s_O`, `r_O`, `r_O`; tokens `T(s_O)`, `T(r_O)`,
`T(r_O)`; base `s_O`; text view `s_O`. **Two deliveries.** A change back to M
is the same with M and O exchanged.

(b) **The application writes `table_active` = `r_M.alpha(0.25)` through
`Theme::update`, then the mode switches to O.** The write: reconcile gives the
token `T(r_M.alpha(0.25))`; `sync_base` leaves base and text view at `s_M`.
Delivery 1: `table_active`'s alpha is 0.25, not 0.2, so it does not hold the
cap; `selection`, `list_active` are native → nothing restored; not
reapplying → marked write. Delivery 2: absorbed. After the write: `s_M`,
`r_M`, `r_M.alpha(0.25)`, tokens likewise, base and text view `s_M`; two
deliveries. The switch is (a): the config replaces the application's value
with `r̂_O`, which the observer restores. **End:** `s_O`, `r_O`, `r_O`; tokens
`T(·)` of those; base and text view `s_O`; two deliveries. The application's
0.25 is gone, as the README says (an upstream rebuild replaces colours); an
application that writes it again after the switch, in the same update as the
`Theme::change`, ends with `table_active` = `r_O.alpha(0.25)` (its closure
`t.table_active.alpha(0.25)` turns `r̂_O` into that, and the observer leaves an
alpha of 0.25 alone) and `selection`, `list_active` restored.

(c) **The application writes `selection` = `ŝ_M` (exactly the capped native
colour) through `Theme::update`.** Token `T(ŝ_M)`; `sync_base` gives base and
text view `ŝ_M`. Delivery 1: `selection` holds the cap and `s_M.a` = 1 > 0.3 →
restored to `s_M` through `Theme::update` (base and text view `s_M`); marked
write; return. Delivery 2: absorbed. **End:** `s_M`, `r_M`, `r_M`; tokens
`T(·)`; base and text view `s_M`. **Two deliveries.** The application's value
is replaced; the README says so.

(d) **Only one variant stored, the mode switches to the other.**

- *Dark stored* (`apply` of the dark variant alone), `Theme::change(Light)`:
  the light slot holds `ThemeConfig::default()` (`to_theme`'s
  `GpuiTheme::from`, `lib.rs:337`, gives both slots
  `Rc::new(ThemeConfig::default())`, gpui-component `src/theme/mod.rs:792-793`,
  and `apply_inner` writes the light slot only from a stored light variant,
  `lib.rs:1303-1320`); `apply_config` gives the three upstream's light
  constants as upstream caps them, `U_L` (whatever `ThemeColor::light()` and
  the fallbacks at `schema.rs:977-980`, `:995`, `:1021` give, then
  `:1053-1070`). Delivery 1: no light variant → the palette repair and the
  restore return at once; marked write (`base_overrides_for` falls back,
  `lib.rs:1355-1367`). Delivery 2: absorbed. **End:** the three and their
  tokens are what upstream's switch made (`U_L`; a token there is the default
  token's background scaled by the cap, `schema.rs:1046-1049`, not necessarily
  `T(U_L)`), base and text view `U_L`'s `selection`; two deliveries. `Theme::change(Dark)` then
  applies the stored dark config and is (a): `s_D`, `r_D`, `r_D`, base and
  text view `s_D`, two deliveries.
- *Light stored*, `Theme::change(Dark)`: the dark slot holds
  `ThemeConfig::default()`, whose mode is light (`ThemeMode`'s default,
  `mod.rs:802-819`), so `apply_config` puts that empty config in the *light*
  slot and sets the mode back to light (`schema.rs:1079-1083`, `:1125`; the
  case `apply`'s doc describes, `lib.rs:1099-1114`). The three are `U_L`.
  Delivery 1: the light variant is stored and current → the palette is
  repaired to the native light palette (as today); the three hold `U_L`, not
  `ŝ_L` / `r̂_L` (test 13 of 1.5 asserts the precondition), so nothing is
  restored; marked write. Delivery 2: absorbed. **End:** window light; the
  three, tokens, base and text view `U_L`'s; two deliveries. A later
  `Theme::change(Light)` applies the empty light config again: the same.

(e) **`Theme::sync_system_appearance` in the same update as the first
`apply`.** One update: `apply` (light), `apply` (dark; dark current, both
stored), `GpuiTheme::sync_system_appearance(None, cx)`. On the test platform
`cx.window_appearance()` is `Light` (gpui-pre `src/platform/test/platform.rs:624-626`),
as on Linux before the portal answers (gpui-pre-linux `src/linux/platform.rs:144`),
so it is `Theme::change(Light)`: the three `ŝ_L`, `r̂_L`, `r̂_L`; base and text
view `ŝ_L`. The flush: the base notification (queued by the first `apply`,
deduplicated with the later ones) finds no active observer; the observer
activates; the deferred re-write (unchanged) writes the overrides unmarked and
repairs the palette. Delivery 1: light's tooltip delay installed; the three
hold the cap → restored to `s_L`, `r_L`, `r_L` through `Theme::update` (base and
text view `s_L`); marked write; return. Delivery 2: absorbed. **End:** mode
light; `s_L`, `r_L`, `r_L`; tokens `T(·)`; base and text view `s_L`; two
deliveries.

These counts are read from gpui-pre 0.3.12's effect queue (`push_effect`
`:2117-2131`, `flush_effects` `:2138-2205`, `apply_notify_global_observers_effect`
`:2292-2297`), not run. Tests 6, 10 and 11 of 1.5 assert them; a different count is
reported, not adjusted in the test. The review of round 1 walked (a) to (e) again
and reached the same end states and counts; it also found GPUI Fast 0.1.15's
effect queue the same (`app.rs:2123-2140`, `:2299-2304`, `:2615-2628`).

#### 1.5 Tests

Helpers and patterns reused from `apply_tests` (`lib.rs:2283`): `prefs(bool)`
(:2324-2328), the precondition style of `preset_for_observer_tests`
(:2337-2346), the two-`apply` setup of `apply_installs_configs_for_both_variants`
(:2625-2652), the upstream-constant probe of
`theme_change_resets_private_base_palette_without_repair` (:2457-2478), the
deferred-change trick of `observer_repairs_a_rebuild_merged_into_its_own_notification`
(:2547-2567), the token check of `the_base_palette_repair_keeps_the_tokens_with_the_colours`
(:2484-2539), and `super::tests::system_theme()` (:2018-2024) as in
`apply_system_theme_stores_both_variants` (:2588-2621). Each `cx.update` closes
an effect flush, so an assertion in the next `cx.update` sees the state after
every delivery. A test that returns proves the deliveries end.

New helpers in `apply_tests`:

- `fn tahoe(prefs: &AccessibilityPreferences) -> ((GpuiTheme, ResolvedTheme), (GpuiTheme, ResolvedTheme))`,
  light then dark, from `from_preset("macos-tahoe", false/true, prefs)`, asserting
  as preconditions: the two themes' `theme_name()`s are equal (so both variants
  are stored); in each variant, `to_theme_color(&r, is_dark, false)`'s
  `selection.a > 0.3` and `list_active.a > 0.2` (so the cap changes them); the
  light and dark `selection`s differ, the light and dark `list_active`s differ,
  and `selection` differs from `list_active` in each variant (so a restore from
  the wrong variant or the wrong field fails).
- `fn natives(resolved: &ResolvedTheme, is_dark: bool) -> [Hsla; 3]`:
  `to_theme_color(resolved, is_dark, false)`'s `selection`, `list_active`,
  `table_active`: the colours `apply` installs.
- `fn assert_three(cx: &App, expected: [Hsla; 3], what: &str)`: for each of the
  three fields, the plain field equals the expected colour and the token equals
  `gpui_component::theme::ThemeToken::from(it)`; and
  `gpui_base::Theme::global(cx).tokens.colors.selection` and
  `gpui_base::text::editor_host::RenderContext::from_app(cx).style().selection()`
  equal `expected[0]`. Messages name the field and `what`.
- `fn count_deliveries(cx: &mut App) -> Rc<Cell<usize>>`: registers a second
  `cx.observe_global::<gpui_base::Theme>` that increments the count, detached
  (`Rc` is imported at `lib.rs:285`; `lib.rs` imports no `Cell` today, so
  `use std::cell::Cell;` is added to `apply_tests`' imports, `lib.rs:2284-2286`).
  Registered in an update of its own, before the update it measures,
  because a subscription activates at the end of its update's flush; the count
  is reset to 0 at the start of the measured update.

Tests, in `colors.rs`'s test module (after line 721, so no claim moves):

1. `capped_natives_are_the_colours_to_theme_color_assigns`: for every
   `Theme::list_presets()` entry and both modes (the `resolved_preset` helper,
   `colors.rs:753`), `capped_natives(&r)` equals the three fields of
   `to_theme_color(&r, mode.is_dark(), false)`.
2. `restore_capped_undoes_the_cap_and_nothing_else`: a
   `gpui_component::theme::Theme::from(&to_theme_color(..))` (written out in
   full: in this test module `Theme` is `native_theme::theme::Theme`,
   `colors.rs:726`, which test 1 uses for `Theme::list_presets()`)
   of macos-tahoe's light variant with natives `n = capped_natives(..)`:
   (i) set each field and token to its cap (`n.x.alpha(cap)`, `ThemeToken::from`)
   → `restore_capped` returns `true` and every field and token is native; a
   second call returns `false`; (ii) `table_active` = `n.table_active.alpha(0.25)`
   with its token → `false`, value kept; (iii) `selection`'s field at its cap but
   its token `ThemeToken::from(n.selection)` → `false`, both kept (field and
   token must both hold the cap); (iv) natives built from windows-11's light
   variant, whose `list.selection_background` is #00000009, alpha 9/255 below
   0.2 (`windows-11.toml:371`, asserted as a precondition), with `list_active`
   set to that native at alpha 0.2 → `false`, kept: below the cap there is no
   cap to undo, and that value is the application's.
3. `every_8_bit_colour_survives_hsla_to_hex`: for every `r, g, b` in `0..=255`
   at alpha 255, and every alpha in `0..=255` on one colour, `rgba_to_hsla(Rgba::new(r, g, b, a))`
   converted back to `gpui::Rgba` and each channel through `channel_byte`, the
   rule `hsla_to_hex` uses (1.2), gives back the bytes; no string is built in the
   16,777,216 iterations. Its run time in a debug build is not measured (the
   round-1 review reasons about 0.5–2 s per test leg in this workspace's dev
   profile); the rule in the header above decides whether it is `#[ignore]`d.
4. `a_native_colour_survives_the_config_round_trip`: for every preset, both
   modes and the three natives `n` (taken from `to_theme_color`'s `selection`,
   `list_active`, `table_active`, so the test needs no new item):
   `gpui_component::try_parse_color(&hsla_to_hex(n))`
   (re-exported at gpui-component's root through `pub use theme::*`,
   `src/lib.rs:119`, `src/theme/mod.rs:30`) equals `n`.

Tests in `lib.rs`'s `apply_tests` (after
`the_base_palette_repair_keeps_the_tokens_with_the_colours`, line 2539):

5. `theme_change_caps_the_three_without_the_restore` (the precondition that makes
   the others mean something, and the exactness end to end): no `apply`, so no
   observer. `gpui_component::init(cx)`; set the styled theme's `dark_theme` to
   `Rc::new(config::to_theme_config(&dark, "Test", GpuiThemeMode::Dark, &prefs))`
   of macos-tahoe's dark variant; `GpuiTheme::change(GpuiThemeMode::Dark, None, cx)`.
   Next update: each field equals `n.alpha(cap)` and its token
   `ThemeToken::from(n.alpha(cap))`, bit for bit, and differs from `n`.
6. `a_mode_switch_restores_the_three_capped_colours` (sequence a, and the
   rationale's "both ways"): `apply` light, `apply` dark (dark current); next
   update `assert_three(natives(dark))`; register the counter; then, each in its
   own update, `Theme::change(Light)` → next update `assert_three(natives(light))`
   and count 2; `Theme::change(Dark)` → `assert_three(natives(dark))` and count 2.
7. `apply_system_theme_restores_the_three_after_a_switch`: `let Ok(mut sys) =
   super::tests::system_theme() else { return; }` (Linux always has one; elsewhere
   without `native` the test returns, as :2590-2592 does); replace `sys.light`,
   `sys.dark` with macos-tahoe's resolved variants and `sys.mode` with
   `ColorMode::Dark` (all three are public fields, `native-theme/src/lib.rs:667-671`;
   a `SystemTheme` cannot be built outside its crate, `overlay_source` being
   `pub(crate)`, `:673`); `apply_system_theme(&sys, cx)`; `Theme::change(Light)`
   → `assert_three(natives(light))`; `Theme::change(Dark)` → `assert_three(natives(dark))`.
8. `sync_system_appearance_in_the_first_applys_update_is_restored` (sequence e):
   precondition `cx.window_appearance() == gpui::WindowAppearance::Light`; in one
   update `apply` light, `apply` dark, `GpuiTheme::sync_system_appearance(None, cx)`;
   next update: not dark, `assert_three(natives(light))`. It holds whatever the
   order inside the deferred closure (1.3); it fails today because nothing
   restores.
9. `a_rebuild_merged_into_the_observers_own_notification_is_restored`: `apply`
   light, `apply` dark; next update, as at :2557-2558,
   `let _ = gpui_base::Theme::global_mut(cx); cx.defer(|cx| GpuiTheme::change(GpuiThemeMode::Light, None, cx));`;
   next update `assert_three(natives(light))`. This fails if the restore is
   placed after the `reapplying` branch.
10. `an_applications_own_value_survives_the_delivery_after_its_write`
    (sequence b): both applied, dark current; counter; update:
    `GpuiTheme::update(cx, |t| t.table_active = t.table_active.alpha(0.25))`;
    next update: `table_active` is the dark native at alpha 0.25 with its token,
    `selection` and `list_active` native, count 2; then `Theme::change(Light)`;
    next update `assert_three(natives(light))` (the rebuild replaced the value);
    then, in one update, `Theme::change(Dark)` followed by the same
    `Theme::update`; next update: `table_active` is the dark native at 0.25,
    `selection` and `list_active` native dark.
11. `a_value_equal_to_the_cap_is_replaced` (sequence c): both applied, dark
    current; counter; update: `GpuiTheme::update(cx, |t| t.selection = t.selection.alpha(0.3))`;
    next update `assert_three(natives(dark))`, count 2.
12. `a_mode_without_a_stored_variant_keeps_upstreams_colours_dark_stored`
    (sequence d, first bullet): `apply` the dark variant alone; next update:
    `U_L` is what upstream's switch makes, computed without the observer on a
    copy, `let mut probe = GpuiTheme::global(cx).clone(); let cfg = probe.light_theme.clone(); probe.apply_config(&cfg);`
    (`Theme::apply_config` is public, `schema.rs:1078`; `Theme::change` runs
    exactly that after setting the mode, `mod.rs:318-335`), keeping the three
    fields and their three tokens; then `Theme::change(Light)` in the same
    update; next update: the three fields and tokens equal the probe's, base and
    text view equal the probe's `selection`; `Theme::change(Dark)` →
    `assert_three(natives(dark))`. The tokens are compared with the probe's, not
    with `ThemeToken::from`: without a config string the cap scales the default
    token's background (`schema.rs:1046-1049`), which need not be a solid of the
    capped colour.
13. `a_mode_without_a_stored_variant_keeps_upstreams_colours_light_stored`
    (second bullet): `apply` the light variant alone; next update: the probe of
    12 with `probe.dark_theme` (the empty config, whose mode is light);
    precondition: each probed field differs from the light native at its cap
    (else the test proves nothing); `Theme::change(Dark)`; next update: not dark
    (the empty config is light), the three fields and tokens equal the probe's.

Test in `lib.rs`'s `tests` module, for §4.9: see 2.4.

**Which tests fail on today's code.** Tests 4, 5 and 13 pass today: they are
proofs and guards (4, the round trip through upstream's parser; 5, that
upstream caps exactly `N.alpha(cap)`; 13, that a mode with no stored variant
keeps upstream's colours). Test 3's property, the byte round trip, holds today,
but the test compiles only with `channel_byte`. Every other test fails today:
1, 2 and 2.4 by a compile error (they name `capped_natives`, `restore_capped`,
`to_hsla`);
6, 7, 8, 9, 11 and 12 because the three stay capped after the switch; 10 at its
assertion after `Theme::change(Light)`, for the same reason.

#### 1.6 Documentation that follows

**README.md:17-21** ("What it does", the *Fonts and global geometry* bullet). Today:

> - **Fonts and global geometry**: families, sizes, `radius`, `shadow`,
>   `focus_ring`, `scrollbar_mode`; a `ThemeConfig` per mode so upstream's own
>   `Theme::change` reproduces the native palette in light and dark (the 12
>   base-palette fields `ThemeConfigColors` keeps private are restored by the
>   connector's observer after each switch).

New:

> - **Fonts and global geometry**: families, sizes, `radius`, `shadow`,
>   `focus_ring`, `scrollbar_mode`; a `ThemeConfig` per mode so upstream's own
>   `Theme::change` reproduces the native palette in light and dark. The
>   connector's observer restores after each switch what the config cannot
>   carry: the 12 base-palette fields `ThemeConfigColors` keeps private, and
>   `selection`, `list_active` and `table_active`, whose alpha gpui-component
>   caps at 0.3, 0.2 and 0.2 whenever it applies a config.

**README.md:165-168** (Quick start, `init`). Today:

> Call `gpui_kit::init` (or `gpui_component::init`) **before** `apply`, as
> upstream requires before any component is used. `apply` initialises the styled
> layer itself only when it is absent; an `init` after `apply` would reset the
> theme to upstream's default.

New:

> Call `gpui_kit::init` (or `gpui_component::init`) **before** `apply`, as
> upstream requires before any component is used. `apply` initialises the styled
> layer itself only when it is absent. An `init` after `apply` switches the theme
> to light mode (the stored light variant, or upstream's light constants when
> none is stored) and sets gpui-component's scroll bar mode from the platform
> (`Hover` on Linux), in place of the one `apply` chose from the theme
> (gpui-component `src/theme/mod.rs:36-42`, `:375-388`).

What `init` does, read from the code: `registry::init` installs a new
`ThemeRegistry` and leaves the styled theme's `light_theme` alone
(`src/theme/registry.rs:40-44`); `Theme::change(Light)` then reloads the light
slot because the global exists (`mod.rs:398-408`); `sync_scrollbar_appearance`
sets `Hover` where the platform does not auto-hide scroll bars (`mod.rs:375-382`),
which gpui-pre-linux never does (`src/linux/platform.rs:145`, `:1078-1080`). With
this part's restore the three colours come back native after it.

**README.md:176-188** (the example of "Light and dark under a preset"). Today it
applies `from_preset("catppuccin-latte", false, …)` and then
`from_preset("catppuccin-mocha", true, …)`, whose display names differ
("Catppuccin Latte", "Catppuccin Mocha"), so the second `apply` drops the light
variant (`lib.rs:1268`) and its comment "both variants are stored" is false.
New block (one preset, both modes; adwaita's two variants share the name
"Adwaita", `native-theme/src/presets/adwaita.toml:10`):

```rust,no_run
use native_theme_gpui::native_theme::detect::accessibility_preferences;
use native_theme_gpui::{apply, from_preset};

# fn f(cx: &mut gpui_kit::App) -> native_theme_gpui::Result<()> {
let prefs = accessibility_preferences();
let (light, light_resolved) = from_preset("adwaita", false, &prefs)?;
let (dark, dark_resolved) = from_preset("adwaita", true, &prefs)?;
apply(light, &light_resolved, &prefs, cx);
apply(dark, &dark_resolved, &prefs, cx);   // dark is now current; both variants are stored
# Ok(())
# }
```

**README.md:190-197** ("Light and dark under a preset"). Today:

> Afterwards gpui-component's own `Theme::sync_system_appearance(None, cx)` or
> `Theme::change(mode, None, cx)` switches between the two native palettes: each
> `apply` installs a `ThemeConfig` for every stored variant, and the connector's
> observer restores the base-layer geometry and the 12 base-palette colours the
> config cannot carry after the switch. With only one variant stored, a switch to
> the other mode shows upstream's built-in `ThemeColor::light()` / `dark()`
> constants (the default config carries no colours), so call `apply` once per
> variant.

New:

> Afterwards gpui-component's own `Theme::sync_system_appearance(None, cx)` or
> `Theme::change(mode, None, cx)` switches between the two native palettes: each
> `apply` installs a `ThemeConfig` for every stored variant, and after the switch
> the connector's observer restores what the config cannot carry: the
> base-layer geometry, the 12 base-palette colours, and the native `selection`,
> `list_active` and `table_active`, which gpui-component caps at alpha 0.3, 0.2
> and 0.2 whenever it applies a config (see
> [How re-application works](#how-re-application-works)). The two variants must
> be one theme's: `apply` drops the other mode's stored variant when a theme of
> another display name stored it. With only one variant stored, a switch to the
> other mode shows upstream's built-in `ThemeColor::light()` constants, in light
> mode, those three as upstream caps them (the default config is a light one and
> carries no colours), so call `apply` once per variant.

**README.md:499-508** ("How re-application works", first paragraph). Today:

> gpui-component's `Theme::change`, `sync_system_appearance` and `sync_base`
> rebuild gpui-base's theme with upstream's fixed scrollbar styles. `apply`
> therefore installs, once per `App`, a global observer on `gpui_base::Theme`
> that writes the native scrollbar geometry and resize-handle colours back after
> every such rebuild, and restores the 12 base-palette colours (`red` …
> `cyan_light`) the rebuild reset, because `ThemeConfigColors` keeps those
> private and the variant's config cannot carry them. It absorbs the notification of its own write with a flag
> and repairs the handle colours if another effect rebuilt the base theme in the
> meantime; the scrollbar styles are opaque upstream, so a rebuild that happens
> to leave the handle colours unchanged is repaired at the next one.

New:

> gpui-component's `Theme::change`, `sync_system_appearance` and `sync_base`
> rebuild gpui-base's theme with upstream's fixed scrollbar styles, and a
> rebuild from a config (`Theme::change`, `sync_system_appearance`, a
> `ThemeRegistry` reload) also resets what the config cannot carry. `apply`
> therefore installs, once per `App`, a global observer on `gpui_base::Theme`
> that, on every notification of that theme, and so after every such rebuild:
>
> - restores the 12 base-palette colours (`red` … `cyan_light`), which
>   `ThemeConfigColors` keeps private, so the variant's config cannot carry them;
> - restores `selection`, `list_active` and `table_active`, with their tokens,
>   where each holds what gpui-component's cap made of the native colour.
>   gpui-component 0.9.1 caps their alpha at 0.3, 0.2 and 0.2 whenever it applies
>   a config, a colour the config states included (`src/theme/schema.rs:1039-1070`),
>   so a selection the theme states opaque, as Breeze and macOS draw it, would
>   be a faint wash from the first switch on. The restore goes through
>   `Theme::update`, which also rebuilds gpui-base's copies of `selection`: its
>   colour tokens, which paint selected text, and the text view's defaults;
> - writes the native scrollbar geometry and resize-handle colours back.
>
> It absorbs the notification of its own write with a flag and repairs the
> handle colours if another effect rebuilt the base theme in the meantime; the
> scrollbar styles are opaque upstream, so a rebuild that happens to leave the
> handle colours unchanged is repaired at the next one.

**README.md:524-532** ("Assumptions"). Today:

> Assumptions: the connector is the only code writing `gpui_base::Theme`
> besides gpui-component itself, and the only code writing the styled theme's 12
> base-palette fields after `apply` (a value set there by the application is
> replaced at the next rebuild); and gpui-component's `ThemeRegistry` observer
> replaces …

New (the rest of the paragraph, from "and gpui-component's `ThemeRegistry`
observer replaces", unchanged):

> Assumptions: the connector is the only code writing `gpui_base::Theme`
> besides gpui-component itself, and the only code writing the styled theme's 12
> base-palette fields after `apply` (a value set there by the application does
> not stay: the connector writes them back whenever gpui-base's theme is
> notified, which the application's own `Theme::update` does); a value the
> application gives `selection`,
> `list_active` or `table_active` is its own until the next rebuild replaces it,
> unless it is exactly the capped native colour (the native colour at alpha 0.3
> for `selection`, 0.2 for the other two), which cannot be told from
> gpui-component's cap and is replaced at once; and gpui-component's
> `ThemeRegistry` observer replaces …

**`lib.rs` crate documentation, Quick Start, line 31.** Today
`//!     // Before `apply`: an `init` afterwards resets the theme to upstream's default.`;
new (one line, under 100 columns):
`//!     // Before `apply`: an `init` afterwards switches to light and resets the scroll bar mode.`

**`lib.rs` crate documentation, "Single-Mode Behavior", lines 77-81.** Today:

> To support
> runtime light/dark switching, call [`from_preset()`] once per mode and
> [`apply`] both: each `apply` stores its variant, and gpui-component's
> `Theme::change` / `sync_system_appearance` then switch between the two
> native palettes.

New:

> To support
> runtime light/dark switching, call [`from_preset()`] once per mode and
> [`apply`] both: each `apply` stores its variant, and gpui-component's
> `Theme::change` / `sync_system_appearance` then switch between the two
> native palettes, the observer [`apply`] installs restoring after each switch
> what the installed `ThemeConfig` cannot carry or gpui-component caps: the 12
> base-palette colours and the native `selection`, `list_active` and
> `table_active` ([`apply`], step 8).

**`apply`'s doc, `lib.rs:1099-1100`.** Today "Call `gpui_component::init` (or
`gpui_kit::init`) *before* `apply`: an `init` afterwards resets the theme to
upstream's default." New: "Call `gpui_component::init` (or `gpui_kit::init`)
*before* `apply`: an `init` afterwards switches the theme to light mode (the
stored light variant, or upstream's light constants when none is stored) and
sets gpui-component's scroll bar mode from the platform (`Hover` on Linux), in
place of the one `apply` chose." The paragraph goes on unchanged from "A
single-variant `apply` …".

**`apply`'s step 3, `lib.rs:1069-1075`.** The end of the step, today
"…installs a `ThemeConfig` for the *other* stored variant (if any) under the
same display name, so upstream's `Theme::change` / `sync_system_appearance`
reproduces native colours in either mode;", becomes "…under the same display
name, so upstream's `Theme::change` / `sync_system_appearance` reproduces native
colours in either mode, all but the ones step 8 restores;".

**`apply`'s step 8, `lib.rs:1089-1096`.** New text:

> 8. installs, once per `App`, the observer that, whenever upstream rebuilds
>    the theme (a `Theme::change` or `sync_system_appearance` mode switch among
>    them), restores the 12 base-palette colours a `ThemeConfig` cannot carry
>    (`red` … `cyan_light`, private in `ThemeConfigColors`); restores
>    `selection`, `list_active` and `table_active`, with their tokens, where each
>    holds what gpui-component's alpha cap made of the stored variant's colour
>    (0.3, 0.2 and 0.2, gpui-component src/theme/schema.rs:1039-1070), through
>    `Theme::update`, which also rebuilds gpui-base's copies of `selection`;
>    restores step 5; and installs step 7's delay for the mode then current;
>    together with one deferred repeat for the update that installs it (the
>    subscription activates only at the end of that update's effect flush);

**`apply`'s single-variant paragraph, `lib.rs:1110-1113`.** "…while the base
palette, scrollbars, resize handles, tooltip delay and
[`ActiveNativeTheme::native`] still follow the stored light variant," gains
after "light variant": "(`selection`, `list_active` and `table_active` keep
upstream's constants: they hold no cap of the light variant's colours)".

**`apply_system_theme`, `lib.rs:1142-1146`.** New:

> [`apply`] for the OS theme: installs the theme of the OS colour mode, as
> [`SystemThemeExt::to_gpui_theme`] builds it, and stores both variants
> with a `ThemeConfig` for each, so upstream's
> `Theme::sync_system_appearance` and `Theme::change` reproduce native
> colours in either mode, with what a config cannot carry or gpui-component
> caps restored by [`apply`]'s observer (step 8).
>
> It installs the mode the OS reports ([`SystemTheme::mode`]), so it needs no
> `Theme::sync_system_appearance` after it. Calling that right after it can show
> a dark desktop's application light for its first frames: on Linux GPUI
> reports a light appearance until the desktop portal answers (gpui-pre-linux
> src/linux/platform.rs:144, src/linux/xdg_desktop_portal.rs:49-51). Call
> `sync_system_appearance` on an appearance change, or follow the OS theme with
> `on_theme_change` (feature `watch`).

(`on_theme_change` stays in plain backticks: an intra-doc link to an item behind
`watch` breaks the `-D warnings` docs run without that feature.)

**`install_observer_once`, `lib.rs:1440-1450`.** New:

> Observe `gpui_base::Theme` (§3.3). On every delivery it repairs the base
> palette ([`repair_base_palette`]), installs the current mode's tooltip delay,
> and restores the three colours gpui-component caps ([`restore_capped_colors`]);
> a restore ends in `sync_base`, which resets the base overrides, so it is
> followed by a marked write of them and the delivery ends there. Otherwise the
> delivery that carries the observer's own write is absorbed by `reapplying`,
> and any other one is answered with a marked write. Terminates because
> `global_mut` and `set_global` queue one deduplicated notification
> (gpui-pre 0.3.12 `src/app.rs:2117-2131`, `:2555-2581`), delivered after the
> pending mark is removed (`:2292-2297`): the observer's writes yield exactly one
> further delivery, which finds nothing to restore and is absorbed.
>
> The subscription activates through a deferred effect (`src/app.rs:2602-2615`)
> at the end of the flush that follows this call, so a base-theme write made by
> other code in the same update as the first `apply` (a
> `Theme::sync_system_appearance` right after it) would stand until the next
> rebuild. One deferred re-write, queued after the activation, closes that gap
> (D38): its write is the first notification the active observer receives, and
> that delivery repairs the palette and restores the three as any other does.

(The 0.3.10 line numbers it cites today, `:2107-2109`, `:2275-2279`,
`:2585-2597`, are replaced by 0.3.12's.)

**`repair_base_palette`, `lib.rs:1407-1420`.** Its last sentence, today "After
`apply` the connector is the sole writer of these 12 fields: a value an
application sets on them itself is replaced at the next rebuild." (`:1419-1420`),
becomes "After `apply` the connector is the sole writer of these 12 fields: it
writes them back on every notification of gpui-base's theme, so a value an
application sets on them does not stay (its own `Theme::update` causes such a
notification)." Then appended:

> The three colours gpui-component caps (`selection`, `list_active`,
> `table_active`) are restored beside this repair by [`restore_capped_colors`],
> which, unlike it, writes through `Theme::update`: gpui-base keeps copies of
> `selection` that only `sync_base` refreshes.

### 2. `to_hsla` (§4.9)

#### 2.1 Signature and place

```rust
pub fn to_hsla(rgba: Rgba) -> Hsla
```

Defined in `lib.rs` after `layer_over` (which ends at line 745), with
`#[must_use]`; a free function of the crate root is exported there by
definition. `Rgba` is the root re-export `native_theme::color::Rgba`
(`lib.rs:261`), `Hsla` is gpui's (`lib.rs:281`).

**Wrapped, not renamed.** The body is `colors::rgba_to_hsla(rgba)`.
`rgba_to_hsla` is named on 307 lines in nine source files of the crate (`src/`
including `widgets/`); renaming it touches all of them for no change in
behaviour, and `layer_over` already wraps `colors::over` the same way
(`lib.rs:742-745`). The colour module stays private (rationale §4.9).

#### 2.2 Rustdoc

```rust
/// A colour of the theme as the `Hsla` gpui paints: the conversion the
/// connector maps every colour of a [`ResolvedTheme`] with, so a colour an
/// application converts for a widget of its own is the one the connector would
/// give. Each channel, alpha included, is the byte over 255, through
/// `gpui::Rgba`; nothing is premultiplied or rounded.
///
/// # Examples
///
/// ```
/// use native_theme_gpui::{Rgba, to_hsla};
///
/// // An opaque colour stays opaque, a translucent one keeps its alpha.
/// assert_eq!(to_hsla(Rgba::rgb(0x3d, 0xae, 0xe9)).a, 1.0);
/// assert_eq!(to_hsla(Rgba::new(0, 0, 0, 0x80)).a, 128.0 / 255.0);
/// ```
#[must_use]
pub fn to_hsla(rgba: Rgba) -> Hsla {
    colors::rgba_to_hsla(rgba)
}
```

The example names only `native_theme_gpui`, as `scripts/check_readme_versions.py`
requires of the crate's doc examples, and runs (no `no_run`): it reads nothing
from the system. `Rgba::rgb` and `Rgba::new` are `const fn`s of native-theme
(`native-theme/src/color.rs:64`, `:70`). The second assertion holds bit for bit:
`to_f32_array` computes `0x80 as f32 / 255.0` (`color.rs:128-135`), the literal
`128.0 / 255.0` in an `f32` comparison is the same division, and
`From<Rgba> for Hsla` passes the alpha through (gpui-pre `src/color.rs:710-712`).

#### 2.3 The parity check

`scripts/check_connector_parity.py` holds every crate-root function that takes
no `ResolvedTheme` (class `outside`) to the sibling rule: iced and egui must
have a function of that name with the same signature or be excepted
(`check_connector_parity.py:37-49`). Neither has a root `to_hsla`. Added to
`scripts/connector_parity_exceptions.toml`, after the `layer_over` entries
(`:338-348`, the "composites" block):

```toml
[[exception]]
connector = ["iced", "egui"]
name = "to_hsla"
kind = "absent"
reason = "gpui's conversion of a model colour to the toolkit's, public at the crate root beside `layer_over` (docs/issues/0004_kde-hand-overrides-rationale.md §4.9); iced's is `palette::to_color(Rgba)` and egui's `convert::to_color32(Rgba)`, each in a module of the toolkit's own conversions that the script lists and does not compare"
```

#### 2.4 Test

The doctest above (it runs in the `native,watch` test leg; the GPUI Fast leg
excludes doctests). And in `lib.rs`'s
`tests` module, `to_hsla_gives_the_colours_to_theme_installs`: for macos-tahoe's
two variants, `to_theme(&r, "Test", is_dark, &AccessibilityPreferences::default())`'s
`background`, `selection` and `list_active` equal `to_hsla(r.window.background_color)`,
`to_hsla(r.input.selection_background)` and `to_hsla(r.list.selection_background)`
(the sources at `colors.rs:183`, `:219`, `:257`).

### 3. README additions (§4.8, §4.7)

Rust blocks follow the README's own: fenced `rust,no_run`, items named through
`gpui_kit` (`gpui_kit::component::Theme`) and `native_theme_gpui`, the body
inside a hidden `# fn f(cx: &mut gpui_kit::App) -> native_theme_gpui::Result<()> {`
… `# Ok(())` / `# }`. They compile as doctests of `ReadmeDoctests`
(`lib.rs:274-279`, `cfg(all(doctest, feature = "watch", feature = "widgets"))`),
so in the `native,watch` test leg. `scripts/check_readme_versions.py` requires
every crate they name to be stated in a `toml` block above them; `gpui_kit` and
`native_theme_gpui` are, in the Quick start (`README.md:100-104`). Links are
relative anchors; no HTML.

#### 3.1 Quick start: the start-up caveat and the pointer to geometry

Inserted after `README.md:165-168` (the `init` paragraph, as reworded in 1.6,
ending "…in place of the one `apply` chose from the theme (gpui-component
`src/theme/mod.rs:36-42`, `:375-388`)."), before
`### Light and dark under a preset` (line 170):

> `apply_system_theme` installs the mode the OS reports (`SystemTheme::mode`),
> so do not call gpui-component's `Theme::sync_system_appearance` right after
> it: on Linux GPUI reports a light appearance until the desktop portal has
> answered (gpui-pre-linux `src/linux/platform.rs:144`,
> `src/linux/xdg_desktop_portal.rs:49-51`), so a dark desktop's application
> would turn light for its first frames. Call it when the appearance changes,
> or follow the OS theme with `on_theme_change` (see
> [Runtime theme change](#runtime-theme-change)).
>
> The theme sets the colours and the global font and radius; each widget's own
> native height, padding, radius and font reaches it only through the
> `geometry` builder the application applies to it, as
> [Per-widget geometry](#per-widget-geometry) shows.

#### 3.2 `scrollbar_gutter`'s row (§4.3)

`README.md:367`, the `scrollbar_gutter` row's last cell, today ending "…where
they are, it reserves nothing |", gains one sentence before the closing `|`,
Part B's wording (round-1 ruling), true of every theme without a track:

> Where the theme states no track (a transparent `scrollbar.track_color`:
> `kde-breeze`, `windows-11`), content that runs under the bar shows through it,
> a row's hover fill and text or cells that reach the edge among it; Qt ends the
> view at the bar, which this gutter reproduces.

(gpui's bar is the only one of the three connectors' that lies over the
content: iced's `styles::scrollbar` embeds a non-overlay bar beside it,
`connectors/native-theme-iced/src/styles.rs:1513-1541`.)

#### 3.3 "Limits", a new section after "How re-application works"

Inserted after `README.md:532` (the end of the "Assumptions" paragraph), before
`## Features` (line 534):

````markdown
## Limits

Two things a theme cannot reach in gpui-component 0.9.1; each is a request to
upstream.

**A table's scroll bar is 16 px wide.** `DataTable` sizes the box its scroll
bars sit in with the fixed `Scrollbar::width()`, 16 px (gpui-component
`src/table/state.rs:2743`, `:2762`; gpui-base `src/scrollbar.rs:22`,
`:951-953`), where every other scroll area takes the track width the connector
installs, `scrollbar.groove_width` (21 px under kde-breeze). With a wider
groove, a table's bar is cut to 16 px with its thumb off centre, and its hit
area reaches into the last column by the difference (read from gpui-base's
layout and paint code, `src/scrollbar.rs:1425-1448`, `:1554-1556`,
`:1600-1604`, not run). An application whose tables matter more than its other
scroll areas narrows the groove to 16 px with an overlay, which every scroll
bar then takes:

```rust,no_run
use native_theme_gpui::{SystemTheme, Theme, apply_system_theme};

# fn f(cx: &mut gpui_kit::App) -> native_theme_gpui::Result<()> {
let overlay = Theme::from_toml(r##"
[light.scrollbar]
groove_width_px = 16.0

[dark.scrollbar]
groove_width_px = 16.0
"##)?;
apply_system_theme(&SystemTheme::from_system()?.with_overlay(&overlay)?, cx);
# Ok(())
# }
```

**A selected cell's fill covers its text.** In a `DataTable` with cell
selection on (`cell_selectable`, off by default), gpui-component paints
`table_active` over a selected cell (`src/table/state.rs:2458-2472`), where it
paints it behind a selected row (`:2536-2541`). The connector gives
`table_active` the platform's row selection colour, `list.selection_background`,
which every bundled preset but windows-11 states opaque, so a selected cell's
text is hidden. One token cannot serve both: a translucent one keeps the cell
readable and washes out every selected row. An application that selects cells
writes a translucent `table_active` of its own through `Theme::update`, at an
alpha other than 0.2 (gpui-component's cap, which the connector undoes, see
[How re-application works](#how-re-application-works)), after every call that
replaces it (see [Overriding theme values](#overriding-theme-values)):

```rust,no_run
use gpui_kit::component::Theme;

# fn f(cx: &mut gpui_kit::App) {
Theme::update(cx, |theme| theme.table_active = theme.table_active.alpha(0.25));
# }
```
````

(The outer four-backtick fence above only delimits the inserted text in this
spec; the README gets the content between.)

#### 3.4 "Overriding theme values", a new section after "Limits"

Inserted after the "Limits" section, before `## Features`:

````markdown
## Overriding theme values

An application that wants a value of its own writes it over the installed
theme through gpui-component's `Theme::update`, which brings `Theme::tokens`
and gpui-base's copy of the theme in step with the fields the closure sets
(gpui-component `src/theme/mod.rs:264-299`, `src/theme/theme_color.rs:373-388`);
writing `theme.tokens` as well is not needed.

Three kinds of call replace such a value:

- every `apply`, `apply_system_theme` (an `on_theme_change` closure's
  included) and `apply_accessibility` (where a variant is stored) installs a
  new theme, every field of it but the application's `notification`, `list`, `sheet`, `motion` and
  `cursor_blink` settings;
- every rebuild from a config, `Theme::change`, `Theme::sync_system_appearance`
  and a `ThemeRegistry` reload, replaces the colours, fonts, radii, `shadow`
  and the highlight theme with the config's (`src/theme/schema.rs:1078-1126`);
  `focus_ring` and
  `scrollbar_mode` survive it;
- `gpui_kit::init` (or `gpui_component::init`) after `apply` is such a rebuild,
  to light mode, and also sets `scrollbar_mode` from the platform.

So the application writes its overrides in one function of its own and calls
it after each of those calls (and calls `init` before `apply`, as the Quick
start says):

```rust,no_run
use gpui_kit::App;
use gpui_kit::component::{Theme, ThemeMode};
use native_theme_gpui::{SystemTheme, apply_system_theme};

/// The application's own values, written after every call that replaces them.
fn theme_overrides(cx: &mut App) {
    Theme::update(cx, |theme| {
        theme.focus_ring = false;
        theme.table_active = theme.table_active.alpha(0.25);
    });
}

# fn f(cx: &mut App) -> native_theme_gpui::Result<()> {
apply_system_theme(&SystemTheme::from_system()?, cx);
theme_overrides(cx);

// Later, a switch of the mode:
Theme::change(ThemeMode::Dark, None, cx);
theme_overrides(cx);
# Ok(())
# }
```

A value that is theme data, a colour or size of the platform's that the
`geometry` builders and the other connectors should take too, belongs in a TOML
overlay instead ([Apply user overrides to the OS theme](#apply-user-overrides-to-the-os-theme)):
it reaches both modes and survives all of the above.

The connector keeps some fields at the native value itself: the 12
base-palette colours (`red` … `cyan_light`), which it writes back whenever
gpui-base's theme is notified, so a value of the application's there does not
stay (its own `Theme::update` causes such a notification); and `selection`,
`list_active` and `table_active` where they hold gpui-component's cap of the
native colour (the native colour at alpha 0.3, 0.2 and 0.2). A value the
application sets to exactly that capped colour cannot be told from the cap and
is replaced too; any other value of its own stays until one of the calls above
replaces it.

**The focus ring.** gpui-component draws focus the same way however the focus
came, a click included (`DataTable` and `Carousel` are the exceptions: they
show their focus style only for a focus that did not come from the pointer,
`src/table/data_table.rs:255-261`, `src/carousel/carousel.rs:132-138`): a focused
control's border turns `ring`, and a 3 px band of `ring` at alpha 0.5 lies
outside it (gpui-component `src/styled.rs:11-12`, `:257-262`, `:302-327`).
`theme.focus_ring = false`, as above, turns the band off. What remains: a
focused element with a border keeps its border in `ring`, and one without a
border gets a 1 px line (`:264-299`), in `ring`, or, on a borderless button
that is neither a ghost nor unpadded, in the button's text colour at 60 % of
its alpha (`src/button/button.rs:643-650`). For a
text field that is Breeze's look on any focus; a focused button keeps a tinted
border, which Breeze shows for keyboard focus only. Breeze draws its band, 2 px
of the selection colour at alpha 0.3, only for a focus that came from the
keyboard, so on KDE an application whose users mostly point and click may
prefer the ring off; a keyboard user then loses the band Breeze would draw.
`apply` sets `focus_ring` from the theme again (on where
`defaults.focus_ring_width` is above 0), so write it after each `apply`.
````

Facts behind the text, for review: `apply_config` touches neither `focus_ring`
nor `scrollbar_mode` (`schema.rs:1078-1126`); `apply` keeps the five settings
(`lib.rs:1290-1299`); `to_theme` sets `focus_ring` from `focus_ring_width`
(`lib.rs:357-359`); `FOCUS_RING_WIDTH` 3 px and `FOCUS_RING_OPACITY` 0.5
(`styled.rs:11-12`), `ring.alpha(0.5)` (`:261`), the border tint (`:267-270`),
the 1 px line (`:272-299`), the button's own line colour (`button.rs:21`,
`:643-650`: `FocusLine::Inside(normal_style.fg.opacity(0.6))` for a button that
is neither `no_padding` nor ghost); Breeze's band and its keyboard-only rule are
rationale §5.1 (B `breezestyle.cpp:1632-1681`, `:6614-6781`).

### 4. `geometry::list_item_selected`'s doc comment (§4.2)

Today, `geometry.rs:1394-1400`:

```rust
/// The fill is `list_active` only while gpui-component's
/// `Theme::list.active_highlight` is true (`list_item.rs:258`), its default
/// (gpui-base `list_settings.rs:14`), which this connector leaves alone; an
/// application that turns it off gets `accent`, the menu
/// hover fill (`menu.hover_background`, the `accent` row), under this text
/// colour, which is still `list.selection_text_color` and was chosen for the
/// selection fill, not for that one.
```

Corrected:

```rust
/// The fill is `list_active` only while gpui-component's
/// `Theme::list.active_highlight` is true (gpui-component 0.9.1
/// `list_item.rs:258-259`), its default (gpui-base 0.9.1 `list_settings.rs:15`),
/// which this connector leaves alone. An application that turns it off gets
/// `accent`, the menu hover fill (`menu.hover_background`), lettered
/// `accent_foreground` (`menu.hover_text_color`), both set by upstream after the
/// refinement (0.9.1 `:260-262`), so this builder's colour does not reach that
/// row. Apply it only while `active_highlight` is on.
```

The last sentence is taken from Part B's version of this edit (round-1
ruling: this part owns it). The versions are named inline because the module
header says its citations are verified against gpui-component and gpui-base
0.8.1 (`geometry.rs:102-103`), where `list_settings.rs:15` is the closing `}`
and `list_item.rs:258-262` sets no `accent_foreground`; the header stays (the
rationale's §4.8: the lines this work touches cite 0.9.1). Checked against gpui-component 0.9.1 `src/list/list_item.rs`: refinement at
:209, `if cx.theme().list.active_highlight` at :258, `bg(list_active)` :259,
`bg(accent)` and `text_color(accent_foreground)` :260-262; gpui-base 0.9.1
`src/list_settings.rs:15` `active_highlight: true`. The comment's other
citations (`:257-264`, `:205` → `:209`, `:235-237`) hold for 0.9.1.

### 5. The four upstream requests (§4.7)

Kept in a new file, `docs/issues/0004_kde-hand-overrides-upstream-requests.md`,
beside the rationale whose §4.7 they come from, for the maintainer to send to
GPUI Kit's tracker (<https://github.com/longbridge/gpui-kit/issues>, the
`repository` of gpui-component, gpui-base and gpui-kit 0.9.1). The file says at
its top that nothing in it is posted, and that each is sent only with the
maintainer's approval. Paths are within each crate as published on crates.io.
The file's text:

> **1. `DataTable`'s scroll bars ignore the theme's track width**
>
> gpui-component 0.9.1 `src/table/state.rs:2743` and `:2762` size the box that
> holds a table's vertical and horizontal scroll bar with `Scrollbar::width()`,
> a constant 16 px (gpui-base 0.9.1 `src/scrollbar.rs:22`, `:951-953`). The bar
> itself takes its width from the theme's `ScrollbarStyles` as it lays itself
> out (`src/scrollbar.rs:1390-1395`), so in every other scroll host it is as wide
> as the theme says. With a theme
> whose track is wider than 16 px (KDE Breeze's scroll bar is 21 px), the bar
> in a table is clipped to 16 px and its thumb sits off centre; from reading the
> layout and paint code (`src/scrollbar.rs:1425-1448`, `:1554-1556`,
> `:1600-1604`), not from a run, its hit area also reaches into the last column
> by the difference. Could the table size that box with the themed track width?
> gpui-base's reader of that width, `Scrollbar::themed_track_width`
> (`:955-957`), is `pub(crate)` and serves only the editor's overview ruler
> (`src/input/base/element.rs:320`), so gpui-base would need to make it, or an
> equivalent, readable from gpui-component.
>
> **2. A selected cell's overlay hides the cell's text**
>
> In cell-selection mode (`cell_selectable`), gpui-component 0.9.1 paints
> `table_active`, edged with `table_active_border`, as an absolutely positioned
> child laid over a selected cell's content (`src/table/state.rs:2458-2472`),
> while a selected row gets the same token as its background, behind its content
> (`:2536-2541`). A theme that matches a platform whose row selection is opaque
> (KDE Breeze, macOS) needs an opaque `table_active`, and then a selected cell's
> text is hidden; a translucent one keeps the cell readable but washes out every
> selected row. Could the cell's overlay be painted behind the cell's content,
> or could a selected cell have a token of its own?
>
> **3. Let a theme keep an opaque selection colour**
>
> `ThemeColor::apply_config` caps the alpha of `list_active` and `table_active`
> at 0.2 and of `selection` at 0.3 whenever a `ThemeConfig` is applied
> (gpui-component 0.9.1 `src/theme/schema.rs:1039-1070`), a colour the config
> states included (`try_parse_background_clamped`, `src/theme/color.rs:777-791`),
> on every `Theme::change`, `Theme::sync_system_appearance`, registry reload and
> `init` (`src/theme/mod.rs:332-334`). A theme that states the opaque selection a
> platform draws (KDE Breeze, macOS) cannot keep it across a mode switch;
> native-theme-gpui now writes the colours back after every rebuild. Could the
> cap apply only to a colour the config does not state (a fallback), or could a
> config or the theme turn it off?
>
> **4. Draw the focus band for keyboard focus only**
>
> gpui-component 0.9.1 draws a focused element's 3 px band (`src/styled.rs:250-262`,
> `:302-327`) however the focus came, a click included, and tints a focused
> button's or check box's border the same way. GPUI can tell a keyboard focus
> from a pointer one (`Window::last_input_was_keyboard`, gpui-pre 0.3.12
> `src/window.rs:3641-3644`), and `DataTable` already uses it
> (`src/table/data_table.rs:255-261`), as `Carousel` suppresses its ring after a
> pointer press (`src/carousel/carousel.rs:132-138`). KDE Breeze draws its focus band only for
> a focus that came from the keyboard (Tab, Backtab or a shortcut). Could the
> band be drawn only for keyboard focus, a text input keep its tinted border on
> any focus, and a button's and a check box's border tint be keyboard-only too?
> Hiding the whole focus style on a click, as `DataTable` does, would be wrong
> for a text input, whose focused border should show however it was focused.

The backlog entry that points to these (rationale §6, entry 1, `[upstream]`) is
Part C's, and it links this file (round-1 ruling 5).

### 6. CHANGELOG entries

In `CHANGELOG.md` under `## [0.7.1] - 2026-11-13` (line 8). Each entry is one
physical line, as the file's bullets are (`CHANGELOG.md:12`), with the crate in
bold. Two go under `### Added` (line 10); one goes under `### Fixed`, a heading
this part does not create (the plan assigns it). Exact text, one line each:

Under `### Added`:

```markdown
- **native-theme-gpui**: `to_hsla(Rgba) -> Hsla` at the crate root, the conversion the connector maps every theme colour with, so an application converts a `ResolvedTheme` colour for a widget of its own as the connector does, without a copy of its own (the iced and egui connectors have `palette::to_color` and `convert::to_color32`).
- **native-theme-gpui**: the README says how an application keeps a value of its own over the installed theme (*Overriding theme values*: through `Theme::update`, which keeps `Theme::tokens` in step, so `theme.tokens` need not be written too; which calls replace the value, `gpui_kit::init` after `apply` among them; one function of the application's, called after each of them; a TOML overlay for theme data; the fields the connector keeps), how to turn gpui-component's focus ring off and what still shows focus then, and in *Limits* what only gpui-component can change: a `DataTable`'s scroll bar is 16 px wide whatever the theme's groove width, and a selected cell's fill covers its text. The Quick start points to *Per-widget geometry*, says not to call `Theme::sync_system_appearance` right after `apply_system_theme`, since on Linux GPUI reports a light appearance until the desktop portal answers, and says what an `init` after `apply` does (it switches to light mode and resets the scroll bar mode; the README and the crate docs said it reset the theme to upstream's default); the light-and-dark example applies one preset in both modes, where it applied two themes and so stored only the second; `scrollbar_gutter`'s row says it keeps content out from under a bar whose track is transparent. `geometry::list_item_selected`'s documentation no longer says a selected row keeps `list.selection_text_color` with `active_highlight` off: gpui-component letters that row `accent_foreground` after the refinement, so the helper applies only while `active_highlight` is on.
```

Under `### Fixed`:

```markdown
- **native-theme-gpui**: `selection`, `list_active` and `table_active` keep the theme's colours after a light/dark switch. gpui-component caps their alpha at 0.3, 0.2 and 0.2 whenever it applies a `ThemeConfig` (0.9.1 `src/theme/schema.rs:1039-1070`), whatever colour the config states, so from the first `Theme::change`, `Theme::sync_system_appearance` or `ThemeRegistry` reload on, a text selection and a selected list or table row were a faint wash of the colour `apply` had installed, until the next `apply`. The observer `apply` installs now writes the native colour back, with its token, gpui-base's selection token and the text view's selection, wherever a field holds exactly gpui-component's cap of it; a value the application sets itself stays, unless it is that capped colour. A `DataTable` with cell selection on (`cell_selectable`, off by default) paints `table_active` over a selected cell, so where the platform's row selection is opaque a selected cell's text is hidden after a mode switch too, as it already was before the first switch; in 0.7.0 the cap left the cell readable from the first switch on. The README's *Limits* gives the override that keeps it readable.
```

`ROADMAP.md`: owned by Part C (§C.6), whose sentence covers the restore and
`to_hsla`.

### 7. Gates

From `scripts/check_release.sh`. `--commit` runs every gate below but those
marked "full only" (`check_release.sh:29-34`):

| Gate | Command | Catches |
|---|---|---|
| tests, `native,watch` (:665-667) | `cargo test -p native-theme-gpui --features native,watch` | the restore and its control flow (tests 1, 2, 5-13: wrong field, wrong variant, missing token, gpui-base or text-view copy not synced, restore after the `reapplying` branch, a value of the application's replaced, a non-terminating loop as a hang); the exact round trip (3, 4); `to_hsla` (its doctest, test 2.4); the README's new Rust blocks (`ReadmeDoctests`, `lib.rs:274-279`); a line added above `colors.rs:720`, or the in-place edits at `:61` and `:86-88` changing the line count (`every_colour_claim_is_read_at_the_line_it_cites`, `src/showcase.rs:2819`); a change of `hsla_to_hex`'s byte rule (test 3, through `channel_byte`); the showcase's own tests (`[[example]] test = true`, `Cargo.toml:118-123`) |
| tests, no features (:704-705) | `cargo test -p native-theme-gpui --lib --no-default-features` | the same unit tests without `widgets` and the icon features |
| tests, GPUI Fast (:706-716, full only) | `cargo test -p native-theme-gpui --features gpui-fast,native,watch --all-targets` | the restore on the other core (doctests excluded there) |
| clippy (:527-550) | `cargo clippy -p native-theme-gpui --all-targets --features native,watch -- -D warnings`; `… --features gpui-fast,native,watch …`; `… --no-default-features …` | lints in the new code and tests |
| strict panic (:624-632) | `bash scripts/check_strict_panic.sh -p native-theme-gpui --lib --features native,watch`; the same with `gpui-fast,native,watch`; `-p native-theme-gpui --example showcase-gpui --features native,watch` and its `gpui-fast` twin | a panic-prone call in the new library code |
| panic-pattern scan (:400) | `python3 scripts/check_panic_patterns.py …` | an `unwrap`/`expect`-style pattern outside tests |
| docs (:1000-1010) | `RUSTDOCFLAGS="-D warnings" cargo doc -p native-theme-gpui --no-deps --document-private-items` | a broken intra-doc link in the new docs (`restore_capped_colors`, `repair_base_palette`, `SystemTheme::mode`; `on_theme_change` must stay unlinked) |
| docs.rs emulation (:1073-1080, full only) | `python3 -I scripts/check_docsrs.py native-theme-gpui` | the same under docs.rs's configuration |
| connector parity (:918-921) | `python3 scripts/check_connector_parity.py` (and `--self-test`) | `to_hsla` without its exception, or a stale exception |
| README crates (:922-925) | `python3 scripts/check_readme_versions.py` (and `--self-test`) | a crate a new README block or the `to_hsla` example names that the README does not state |
| examples (:862-872) | `cargo build -p native-theme-gpui --examples` | the showcase still builds |
| showcases in the runner environment (:845-854, Linux) | `scripts/run_in_runner_env.sh test -p native-theme-gpui --example showcase-gpui`; with `--features native,watch`; with `gpui-fast,native,watch` (full only) | the showcase against CI's fonts and desktop |
| semver checks (:1144-…, full only) | `cargo-semver-checks` per published crate | `to_hsla` is additive; nothing public is removed |
| formatting (:518) | `cargo fmt --all` | formatting (CI runs `--check`) |
| release stamps (visual assets :1275-1306, compatibility claims :1309-1336; full only) | `bash scripts/update_provenance.sh check` (the asset stamp), `bash scripts/update_compatibility.sh check` | not a mistake of this part: `docs/assets/PROVENANCE.toml` and `docs/COMPATIBILITY.toml` go stale with the first edit under a connector's `src/`, `examples/` or `tests/`, which this part makes. Because the 0.7.1 CHANGELOG entry is dated, the full run then fails hard on both (`check_release.sh:1296`, `:1331`) until Part E's asset run; `--commit` and CI check neither. Expected red, not a defect to fix here |

No gate covers: the prose of the README additions, the corrected doc comments
(`list_item_selected`, `apply`, `apply_system_theme`, `install_observer_once`,
`repair_base_palette`, the crate documentation, the `colors.rs:86-88`
comment) beyond their links; the order inside the deferred closure, which no
test can observe (1.3), and which this spec leaves unchanged; the
upstream line numbers they cite; the CHANGELOG entries; the four
upstream drafts; and a `ThemeRegistry` reload or a `gpui_kit::init` after `apply`
as such (they reach the restore through the same `Theme::edit` path that the
`Theme::change` tests exercise, `mod.rs:398-408`, `registry.rs:46-70`, but no
test drives them).

### 8. Problems found

1. **`colors.rs` cannot grow above line 720.** Not a problem with a decision, a
   constraint the rationale does not name: the showcase's colour claims cite
   `colors.rs:281-718` and a test reads each cited line. The spec places the new
   items after line 719, edits `:61` and `:86-88` in place with their line
   counts kept, and leaves the module header (lines 8-9, "verified against
   gpui-component 0.8.1") as it is; the new items cite their version inline.
   Rewording the header in place, keeping two lines, would also be safe.
2. **`to_hsla` needs an entry outside the gpui connector.** At the crate root it
   falls under `check_connector_parity.py`'s sibling rule and fails it without
   an `absent` exception for iced and egui (section 2.3). The rationale (§4.9,
   §4.10) does not list `scripts/connector_parity_exceptions.toml`.
3. **The text view's selection has no direct public reader.**
   `TextViewDefaults::style` is `pub(crate)` (gpui-base `src/text/text_view.rs:51-53`).
   The tests read it through `gpui_base::text::editor_host::RenderContext::from_app(cx).style().selection()`,
   which is public and returns the installed defaults' style
   (`src/text/editor_host.rs:119-134`, `:177-180`).
4. **`apply_system_theme` cannot be tested on a preset-built `SystemTheme`.**
   `SystemTheme` has a `pub(crate)` field (`native-theme/src/lib.rs:673`), so a
   test cannot build one; test 7 takes `super::tests::system_theme()` and
   replaces its public `light`, `dark` and `mode`. Where that returns an error
   (macOS and Windows without `native`), the test returns early, as the existing
   `apply_system_theme_stores_both_variants` does; sequences (a) to (e) are
   covered on every platform through two `apply`s, which take the same
   `apply_inner` path.
5. **Not run.** The delivery counts (two per sequence) are read from gpui-pre
   0.3.12's effect queue; the exactness of the hex round trip for colours
   outside the presets is reasoned; the run time of the 2^24-iteration test in a
   debug build is unknown. Tests 3 and 4 (the round trip) and 6, 10 and 11
   (the counts) turn each into a mechanical check; a failure is to be reported,
   not worked around.
6. **Stale upstream line numbers in a doc this part touches.**
   `install_observer_once` cites gpui-pre 0.3.10 lines (`src/app.rs:2107-2109`,
   `:2275-2279`, `:2585-2597`); in 0.3.12 they are `:2555-2563` / `:2117-2131`,
   `:2292-2297`, `:2602-2615`. The new text cites 0.3.12 (rationale §4.8,
   "Citations").
7. **Open, not decided by the rationale; not in this spec:** (a) the gpui
   showcase keeps a private copy of the conversion, `stated` in
   `examples/showcase-gpui/info/mod.rs:453-456`, which `to_hsla` could replace
   with no change of value; (b) the README's *Core concepts* lists `layer_over`
   (`README.md:234-237`) and would list `to_hsla` beside it only if the
   maintainer wants it.
8. **The rationale's sentence on the deferred re-write is not implemented.**
   By the round-1 ruling `lib.rs:1491-1494` stays as it is, because the
   delivery its write causes performs the restore (1.3); the orchestrator
   corrects the rationale's "does the same in the same order".
9. **README example changed beyond the rationale's list.** The "Light and dark
   under a preset" example stored only its second theme (two display names), so
   the paragraph under it could not be true; by the round-1 ruling it now
   applies `adwaita` in both modes (1.6).

## Part E — Release close-out

Parts A, B and D edit `src/`, `examples/` and `tests/` of the core and of all
three connectors. Two release stamps hash those paths and go stale with the
first such edit: `docs/assets/PROVENANCE.toml` (the screenshots and GIFs) and
`docs/COMPATIBILITY.toml` (each connector's verified upstream versions).

### E.1 What the gates do meanwhile

- `scripts/check_release.sh` (full run) checks both stamps
  (`scripts/update_provenance.sh check`, `scripts/update_compatibility.sh check`).
  A stale stamp is a **hard failure** while `CHANGELOG.md` dates the version
  (`check_release.sh:1296`, `:1331`; `CHANGELOG.md:8` is
  `## [0.7.1] - 2026-11-13`). So from the first code commit until E.3 the full
  run has exactly these two failures, and they are expected. Any other failure
  is a defect of the change.
- `scripts/check_release.sh --commit` and CI (`.github/workflows/ci.yml`) check
  neither stamp, so they stay green throughout.
- `.github/workflows/publish.yml:51` refuses a tag while either stamp is stale.
- The 0.7.1 heading keeps a date during the work (the release was already
  prepared under it). E.3 sets it to the day of the close-out commit.

### E.2 Sweep of line citations that no gate reads

The removals and insertions of Parts A, B and D move lines that comments and
backlog entries elsewhere cite by number. No gate reads these citations (the
gated ones are the header's *Lines that must not move*). They are re-pointed
once, after the last source edit has landed, each by the item it names, not by
arithmetic:

- Part A's A56 list (in `docs/todo.md`: Part C, C.4 (a) 12; in the egui
  connector's `atlas.rs`: Part B, §5 item 7);
- the egui connector's citations of the gpui connector's `lib.rs`, which Part
  D's documentation edits move: `connectors/native-theme-egui/src/atlas.rs:695`
  (the two font size lines of `to_theme`), `plugin.rs:18` (the `focus_ring`
  line), `ext.rs:208` and `ext.rs:412` (open each, find the item it names in
  the gpui `lib.rs` after Part D's edits, write that line);
- `docs/todo.md:495`'s citations of `native-theme/src/kde/colors.rs`, which
  Part B's reader edits move, and the `kde/colors.rs:525` that Part C's new
  backlog entry 2 cites.

Where a part says a citation is re-pointed "in the same commit or the next",
this sweep's timing holds: once, after the last source edit. A citation that was
already wrong before this work is corrected in the same sweep. A citation whose
item cannot be found is reported, not guessed.

### E.3 The asset run and the final check

In this order; each step starts only when the one before it is green.

1. `cargo audit` passes with the repository's `.cargo/audit.toml`.
2. `scripts/update_compatibility.sh run` passes (it verifies the three
   connectors against the newest upstream releases; a failure here means an
   upstream release moved and is fixed before anything else). It needs the
   connector sources committed, and it leaves `docs/COMPATIBILITY.toml` and the
   three connector READMEs changed and uncommitted; they stay uncommitted until
   step 7 (the asset script accepts that, and runs the same check again).
3. The full `scripts/check_release.sh` shows only the provenance stamp as
   failing (step 2 renewed the other), and the known `cargo outdated` warning.
4. The branch is pushed and CI on the pull request is green.
5. The three showcases are built in advance with the capture scripts' own
   commands (`--release` for gpui, `--release --features iced_aw` for iced,
   `--release --all-features` for egui; `scripts/capture_window.sh:440`), so
   that the desktop is needed for the capture alone.
6. The maintainer says when the desktop can be left unlocked and untouched for
   the capture (4 to 6 minutes). Then `scripts/generate_assets_release.sh` runs
   once. It requires the branch pushed (origin's branch equal to HEAD) and the
   stamped source paths clean. It triggers the macOS and Windows screenshot
   workflow, captures the Linux screenshots and GIFs, checks their parity,
   writes both stamps and rewrites the connector READMEs' "Verified" lines.
   If it stops, nothing is committed. The likeliest stop is the parity step:
   kde-breeze is now the first preset captured on Linux with a bar that does
   not hide and a transparent track, and the comparator samples the list's
   scroll bar track. A difference is fixed in the showcase that shows it (steps
   2 to 6 then repeat, with another capture window) or, where it is a stated
   difference between the toolkits, listed with its reason in the parity
   exceptions; the maintainer is told either way.
7. One commit holds the assets, both stamps, the READMEs' lines and the 0.7.1
   heading's date (the day of this commit). It is pushed.
8. The full `scripts/check_release.sh` at that commit passes with no failure,
   and CI is green.

The screenshots change with this work (the KDE values of Part B; the gpui
showcase calls `Theme::change`, so Part D's restore shows in it), which is why
they are captured again and not only re-stamped.

Nothing is merged, tagged, published or released by this specification. Those
steps are the maintainer's, after step 8.

