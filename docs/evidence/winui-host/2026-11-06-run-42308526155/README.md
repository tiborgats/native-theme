# WinUI host, run 42308526155 (2026-11-06): WP-reads, WH-defaults, WH-motion, WH-captures, all-hosts

[The run](https://github.com/tiborgats/native-theme/actions/runs/42308526155): `os-probe.yml`'s winui-host job on
`windows-11-arm` (dispatched with `-f jobs=winui-host -f images='["windows-11-arm"]'`), green. The host is
`tools/winui-host` on `v0.8.0-tools-hosts-m1b` at `7bad463f` (Windows App SDK 2.5.1, `Microsoft.UI.Xaml.dll`
3.2.3.2609, unpackaged and self-contained, .NET 10.0.14, ARM64), on the runner image windows-11-vs2026-arm64
20261101.203.1 (Windows 11 25H2, 10.0.26200; 1024x768 at 96 DPI, light theme). `run.ps1` ran it twice and once more
with the text size at 150 %. Kept here: run 1's `frames.toml` and files, and the text-scale run's. Run 2's reads equal
run 1's in every table but the recordings' frame counts and one callback count (`rendering_callbacks_minimised`);
the earlier dispatch [42307873328](https://github.com/tiborgats/native-theme/actions/runs/42307873328) read the same
values for every table it had (its `frames.toml` repeated three `[defaults]` keys, so it is not kept).

The recordings (`motion-<name>.png`, the distinct frames side by side; `motion-<name>.csv`, every frame's time from the
pass's start, its index among the distinct frames, its centre pixel and mean colour; the `[[motion]]` table with the
screen rectangle, the actions and their times) are GDI screen captures on a worker thread: about one frame every 40
to 50 ms on this runner, so they give values and their order, not single-frame timing.

**WP-reads** (U505, U508, U510; the probe's half is `docs/evidence/os-probe/2026-11-06-run-42307873328/`):
- `[system]`: `GetCaretBlinkTime` 530, `GetDoubleClickTime` 500, `UISettings.TextScaleFactor` 1.
- `[directwrite]`: the shared factory's default and monitor rendering parameters: gamma 1.8, enhanced contrast 0.5,
  ClearType level 1, pixel geometry RGB, rendering mode DEFAULT, grayscale enhanced contrast 1, grid fit DEFAULT.
- `[[font_fallback]]`: DirectWrite's system font fallback (`IDWriteFontFallback::MapCharacters`) for 24 samples under
  their locales, from no base family and from Segoe UI, Segoe UI Variable and Segoe UI Variable Text: Latin, Greek,
  Cyrillic, Arabic, Hebrew, Armenian and Georgian map to Segoe UI; Devanagari, Bengali, Tamil and Sinhala to Nirmala UI;
  Thai, Lao and Khmer to Leelawadee UI; Myanmar to Myanmar Text; Ethiopic to Ebrima; Han to Yu Gothic UI (ja-JP),
  Microsoft YaHei UI (zh-CN), Microsoft JhengHei UI (zh-TW, zh-HK), Malgun Gothic (ko-KR); emoji to Segoe UI Emoji;
  ✓ to Segoe UI Symbol. Segoe UI Variable and Segoe UI Variable Text, as base families, map nothing to themselves.
- `[[text]]`, `[[text_capture]]`, `text-default.png`: TextBlocks of the default style (Segoe UI Variable, 400) at 10,
  12, 14, 16, 18, 20, 28, 40 and 68 px in black on white, their measured sizes, and their pixels on the screen: 19740
  grey and 0 coloured pixels, greyscale antialiasing.
- `text-scale/frames.toml`, `text-scale/text-text_scale.png`: the same at `TextScaleFactor` 150
  (`UISettings.TextScaleFactor` 1.5): the heights grow by 1.59 at 10 px and 1.05 at 68 px (13.30 to 21.11, 90.45 to
  94.79), not by one factor; again 0 coloured pixels.

**WH-defaults** (U1): `[defaults]` (the accent button's padding 11,5,11,6, border 1, `FocusVisualMargin` -3, label
Segoe UI Variable 14 px 400); `[dialog]` and `[[dialog.button]]` with `screen-dialog-window.png` (a ContentDialog with
`DefaultButton` Primary: its PrimaryButton takes `AccentButtonStyle`, padding 11,5,11,6, border 1, focus margin -3,
label 14 px 400); `[[app_bar]]`, `[[part]]` and `screen-app_bar-<pass>-window.png` (AppBarButton and
AppBarToggleButton standalone and in a CommandBar with labels below and to the right: padding 0, borders 0 and 1, focus
margin -3, and each named template part's bounds, margins and paddings).

**WH-motion**:
- U50 (`scroll_into_view_*`): focus moved to a field below a ScrollViewer's view jumps in one step (`view_events`:
  two ViewChanging events and the final ViewChanged within 16 ms, no inertial step); `StartBringIntoView` with
  `AnimationDesired` and `ChangeView` with animation run about 480 ms of inertial steps (`view_events`,
  `rendering_offsets`).
- U157 (`color_space_*`): brush transitions of 1000 ms between colours of different alpha over white, and black to
  white.
- U183 (`theme_change_idle`, `theme_change_running`): a `{ThemeResource}` brush's colour moves over a transition when
  the theme switches, and a running explicit transition after a switch.
- U185 (`presence_collapse`, `presence_animations_off`): collapsed and shown mid-transition; client-area animations
  turned off mid-transition (`UISettings.AnimationsEnabled` then false; the next transition still runs).
- U188 (`list_reselect`), U193 (`checkbox_first_frame`), U162 (`radio_pressed_checked`, `radio_pressed_unchecked`).
- U501 (`[gesture_recognizer_*]`): every property of a new GestureRecognizer reads 0 or its type's default
  (`HoldStartDelay` 00:00:00, `HoldRadius` 0): these are unset fields, not the system's thresholds, so U501 stays
  open.
- U502 (`mouse_vanish`): with Mouse Vanish on and a key typed into the focused field, `GetCursorInfo` reported the
  cursor showing throughout: the injected input did not hide it, so U502 stays open.

**WH-captures**: U507 (`caret_ltr`, `caret_mixed`, `caret_mixed_end`: the caret's blink over 2.4 s, at a direction
boundary one caret); U511 (`flyout_shadow_white`, `flyout_shadow_black`: a MenuFlyout's frame with 48 px around it
over white, where its shadow shows, and over black); U513 (`drop`, `screen-drop-window.png`: an item dragged over a
ListView that accepts it, `drag_over_event` true, with the drag visual).

**all-hosts**: U187 (`minimise`: the window minimised 1 s during a transition, `Window.Visible` false, the rendering
callbacks kept coming, then the frames after the restore); U190 (`deactivate`: a keyboard-focused button keeps
`FocusState` Keyboard and the focus while another window is active; after the host's window is activated again its
state reads Pointer), with `deactivate_mica` (a Mica backdrop, `MicaController.IsSupported()` true, one colour
#f3f3f3 throughout the deactivation).

Not kept (only on the run's page, for 90 days): run 2, the full-screen captures, spike W's popup PNGs, `desktop.toml`,
the logs.
