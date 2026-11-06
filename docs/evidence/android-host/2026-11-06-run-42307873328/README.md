# Android host, run 42307873328 (2026-11-06): AH-pass

[The run](https://github.com/tiborgats/native-theme/actions/runs/42307873328): `os-probe.yml`'s android-host job
(dispatched with `-f jobs=probe,winui-host,android-host,kde-render`), green. The harness is `tools/android-host` on
`v0.8.0-tools-hosts-m1b` at `a4b2af0a`: Compose Material 3 1.5.0-beta02 on Compose 1.13.0-beta02, on the emulator
(37.2.13, SwiftShader, 720x1280 at 320 dpi) with the system image `android-37.1;google_apis_ps16k;x86_64` revision 9
(Android 17 QPR1; fingerprint `google/sdk_gphone16k_x86_64/emu64xa16k:17/CP31.260623.012/16064790`), twice from a cold
boot. It is the AH-pass of the 0.8.0 measurement plan. Both runs wrote the same `fonts.toml`, `popup-menu.toml`,
`focus.toml`, focus PNGs and `fonts.xml`; CaptureTest's 50 PNGs are byte-identical to each other's run and are not
kept here (spike A's pass).

- `measure/fonts.toml` (U557, `MeasureTest.fallbackFonts`): 24 sample texts shaped by `TextRunShaper` with
  `Typeface.DEFAULT` (Compose's `FontFamily.Default`) under each sample's locale, and the font file each run of glyphs
  comes from with its collection index and locales: Latin, Greek and Cyrillic from `Roboto-Regular.ttf`; each other
  script from its `NotoSans<Script>` file (Arabic and Persian from `NotoNaskhArabic-Regular.ttf`); Han under ja-JP,
  zh-CN, zh-TW, zh-HK and ko-KR from `NotoSansCJK-Regular.ttc` at indices 0, 2, 3, 3 and 1 (zh-HK takes the zh-Hant-TW
  face); emoji from `NotoColorEmoji.ttf` (`und-Zsye`).
- `system-fonts/fonts.xml`: the device's `/system/etc/fonts.xml`, which names the families and the fallback order.
  `/system/etc/font_fallback.xml` was not readable by the shell user (permission denied) and
  `fonts_customization.xml` does not exist.
- `measure/popup-menu.toml` (U564, `MeasureTest.popupMenuColors`): the two attributes Compose's context menu reads
  (`ContextMenuUi.android.kt`), resolved by the same `obtainStyledAttributes` calls. In the activity's context
  (`androidx.activity.ComponentActivity` of `ui-test-manifest`, theme `android:style/Theme.Material.Light.NoActionBar`,
  the application has no theme) both resolve: `Widget_PopupMenu`'s `colorBackground` `#fffafafa`,
  `TextAppearance_Widget_PopupMenu_Large`'s `textColorPrimary` `#de000000` enabled, `#42000000` disabled. The
  `activity_night` and `application` contexts do not carry the activity's theme (`createConfigurationContext` and the
  application context fall back to the platform's default theme; both read `#ffeeedf6` and `#ff30323b`), so they are
  no dark reading of the activity's theme. The values follow the host activity's theme, so they are this host's.
- `measure/focus.toml`, `measure/focus-*.png` (U554, `MeasureTest.focusVisibility`): a light filled button. A real tap
  (an injected touchscreen event) emits Press and Release and leaves the window in touch mode, the button not focused
  and its pixels those of the initial capture. A Tab key event leaves touch mode, focuses the button (Focus emitted)
  and draws its focus state layer (`focus-2-tab.png` differs from the initial capture in 11538 of 36800 pixels). A
  second tap returns to touch mode and emits Unfocus; its capture equals the initial one.
- `measure/ripple-r<n>-d<ms>.png`, `measure/ripple.toml` (U70, U71, `RippleTest`): the light filled button pressed
  (the interaction a real press emits) in a fresh activity, captured 50, 150, 300, 600 and 1200 ms after the press,
  three presses each; `ripple.toml` records each capture's measured start (within 4 ms of the asked delay). At 600 and
  1200 ms all six captures (both runs) are byte-identical: the ripple has ended and the pressed state layer remains.
  At 50, 150 and 300 ms the three presses differ from each other (run 1: up to 6432 of 36800 pixels, by at most 46
  levels at 50 ms; at most 1 level at 300 ms), and run 2's captures differ from run 1's: the host does not draw one
  press's ripple twice alike, so a comparison against `ripple::shade` needs U71's statistical route or a seeded phase,
  which the host cannot set.

Not kept (only on the run's page, for 90 days): CaptureTest's PNGs and index, run 2's files, the logs, `index.toml`,
`/system/fonts`' listing.
