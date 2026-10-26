# OS probe, run 40348854007 (2026-10-26)

What each GitHub runner image's OS reported to `tools/os-probe` (`native-theme-os-probe`),
dispatched by hand on `v0.7.0-rc1` at `d14ce711` through `.github/workflows/os-probe.yml`
(https://github.com/tiborgats/native-theme/actions/runs/40348854007), on 2026-10-26. Kept here
because GitHub deletes run artifacts after 90 days, and `docs/platform-facts.md` cites them. The
run adds to the previous one ([`2026-10-26-run-40324458365/`](../2026-10-26-run-40324458365/);
[`2026-10-26-run-40320895749/`](../2026-10-26-run-40320895749/) describes the key-window section,
the calibration strip and the passes):
- disabled and enabled switches and sliders in the key window;
- disabled renders of the push button, checkbox, radio, switch and slider;
- a pass with a red accent colour.

The runner images are the previous run's: the table in `2026-10-26-run-40320895749/README.md`
gives each image's OS and version.

## What is the same as before

Every value the previous run reported is the same here, in every pass and on all seven images,
apart from two things:
- the pattern colours' object addresses (`scrubberTexturedBackgroundColor`), as always;
- the activation timing of macOS 15's Reduce Transparency, Increase Contrast, tinting-off and
  not-key passes: the attempt and step counts under `[macos.key_window.activation]` differ, and
  key status does not.

Every render the previous run made is byte-identical here (1632 files), apart from the 48
key-window captures, which now hold the new controls.

## What is new

**Disabled and enabled controls in the key window** (`[macos.key_window]`, `method_controls`).
Between the segmented control and the table: a row of disabled controls (`setEnabled: NO`) and a
row of the same controls enabled, each an `NSSwitch` on, an `NSSwitch` off and an `NSSlider` at
0.5. Their frames, the slider's bar and knob rects and the sample points are recorded
(`switch_on_frame_xywh` …, `slider_barRect_xywh`, `slider_knobRect_xywh`, `sample_points_xy`).
The samples are under `[macos.key_window.<appearance>.samples]`: a switch's track and knob, a
slider's fill, track and knob (`probe.toml:3620-3685` in light, `:3814-3879` in dark, on all
four images and in the accent-red pass).

On macOS 26 and xcode-27 an `NSSwitch` is 54 × 24 pt (`switch_on_frame_xywh`), so the controls,
placed at the fixed x 200, 250 and 300, overlap by 4 pt each: the off switch covers the on
switch's right end, and the off switch's right end runs under the slider bar's left end. Every
sample lies clear of the overlaps, on the part it names: the on switch's track at x 209 and knob
at x 242, the off switch's knob at x 262 and track at x 295, the slider's fill at x 320, knob at
x 340 and track at x 360 (`sample_points_xy`, in the window; checked against the captures). On
macOS 14 and 15 (42 × 25 pt) nothing overlaps.

A disabled `NSSlider` draws no accent fill on any image: `slider_disabled_fill` is the grey bar
left of the knob, not a faded accent.

**Disabled renders** (`[macos.controls]`, `method_disabled`): at the regular size the push
button, checkbox, radio, switch and slider rendered disabled, each at its enabled render's size
(`renders/<appearance>/<kind>-regular-disabled[-on|-off][@2x].png`), in every pass. These
renders come from a window that is never key, as the enabled ones do: the probe's offscreen
renders draw the not-key look, so an on switch and a slider's value are grey there.

**`accent-red/`**: the main pass with the accent colour red (the global default
`AppleAccentColor` 0).
- `settings.txt`: the image has no value (absent); the pass writes 0 (exit 0) and reads back 0.
  Afterwards it restores the image's value, deleting the key (exit 0), and the read-back finds it
  absent.
- The window is key and the application active at each capture, as in the main pass.
- Eight NSColors change, the same on all four images: `controlAccentColor`,
  `textInsertionPointColor` and `NSTextView_insertionPointColor` #007aff → #e0383e (light) and
  #ff5257 (dark); `selectedContentBackgroundColor` and `alternateSelectedControlColor` #0064e1 /
  #0059d1 → #c4262b / #d13539; `selectedTextBackgroundColor` and `selectedControlColor` #b3d7ff /
  #3f638b → #f6c4c5 / #8b5759; `keyboardFocusIndicatorColor` #0067f47f / #1aa9ff7f → #d321277f /
  #ff7a807f (light `probe.toml:324`, `:329`, `:331`, `:336`, `:339`, `:362`, `:376`, `:379`;
  dark 74 lines below each).
- In the key window, the on switch, the disabled on switch and the slider's value fill follow the
  accent on all four images; on macOS 26 and xcode-27 so do the selected segment and, in dark,
  the on switch's knob, through which the track shows. Every other sample draws as in the main
  pass.
- Every offscreen render is byte-identical to the main pass's: the not-key look has no accent.

**Calibration.** All 504 sRGB patches read exact: 9 per capture × 56 captures (4 images × 7
passes × 2 appearances). 38 of the 112 system-colour readings differ from the NSColor the probe
reads: the previous run's 34, and 4 in the accent-red pass on macOS 14 and 15,
`windowBackgroundColor` drawn #f6f6f6 / #282828 where it reads #ececec / #323232.

## The disabled switch and slider

Key-window samples, light / dark, main pass (`raw`). macOS 14 and 15 agree, except where noted;
so do macOS 26 and xcode-27.

| Sample | macOS 14 (15) | macOS 26 (27) |
|--------|---------------|---------------|
| window background | #f6f6f6 / #282828 | #ffffff / #1e1e1e |
| on switch, track | #1485ff / #1674d9 (15: #1485fe / #1664d9) | #0076f7 / #077cfd (27 dark #067cfc) |
| disabled on switch, track | #84befb / #1f4e81 (15 dark #1f4681) | #8ec0f9 / #164880 (27: #4b9df5 / #1162ba) |
| off switch, track | #dddddd / #3e3e3e | #e3e3e3 / #343434 (27 dark #333333) |
| disabled off switch, track | #efefef / #2e2e2e | #efefef / #292929 |
| switch knob, enabled off | #ffffff / #c9c9c9 | #ffffff / #e1e1e1 |
| switch knob, disabled off | #fbfbfb / #5e5e5e | #ffffff / #dedede |
| slider fill | #0a82ff / #177be5 | #0076f7 / #067cfd (27 dark #067cfc) |
| slider track | #eaeaea / #3e3e3e | #e4e4e4 / #343434 (27 dark #333333) |
| slider knob | #ffffff / #949494 | #ffffff / #dedede |
| disabled slider fill | #efefef / #333333 | #dfdfdf / #393939 (27: #d9d9d9 / #3d3d3d) |
| disabled slider track | #efefef / #333333 | #e3e3e3 / #343434 (27 dark #333333) |
| disabled slider knob | #fbfbfb / #434343 | #ffffff / #dedede |

Against the enabled controls:
- A disabled on switch draws the enabled on track at a lower alpha over the window, in the
  accent, red as well as blue. The alpha is 0.50 on macOS 14 and 15 in every channel that
  differs from the window by more than 40 levels, about 0.45 on macOS 26 (0.44–0.47) and about
  0.71 on xcode-27 (0.70–0.75).
- On macOS 14 and 15 a disabled slider draws no value fill: the bar has the track's colour on
  both sides of the knob. On macOS 26 the bar left of the knob is a denser grey than the
  track, darker in light and lighter in dark.
- On macOS 26 a disabled knob, and a disabled slider's track, draw as the enabled ones.

The offscreen renders (`renders/<appearance>/switch-regular-disabled-off@2x.png`, …) give the
alpha of the neutral parts, uniform along the track away from the knob:

| Part | macOS 14, 15 | macOS 26, 27 |
|------|--------------|--------------|
| switch off track, enabled | #0000001a / #ffffff1a | #00000019 / #ffffff19 |
| switch off track, disabled | #00000007 / #ffffff07 | #0000000d / #ffffff0d |
| switch knob, enabled | #ffffff / #ffffffbf | #ffffff / #ffffffdd |
| switch knob, disabled | #ffffff80 / #ffffff40 | #ffffff / #ffffffdb |
| slider fill, disabled | #00000007 / #ffffff0d | #0000002a / #ffffff2a |
| slider track, enabled | #0000000d / #ffffff1a | #00000019 / #ffffff19 |
| slider track, disabled | #00000007 / #ffffff0d | #00000019 / #ffffff19 |
| slider knob, enabled | #ffffff / #ffffff80 | #ffffff / #ffffffd9 |
| slider knob, disabled | #ffffff80 / #ffffff20 | #ffffff / #ffffffd9 |

On macOS 14 and 15 each of these, laid over the key window's background, gives the key-window
sample within one level. On macOS 26 the dark parts do too, apart from the disabled slider fill.
There, the light tracks draw 2 or 3 levels darker in the key window (the off tracks #e3e3e3 and
#efefef against #e6e6e6 and #f2f2f2), and the disabled slider fill draws lighter: #dfdfdf /
#393939 in the key window, #d3d3d3 / #424242 in the not-key pass's window, where the render
gives #d5d5d5 / #434343.

The switch and slider renders of macOS 14 and 15 are identical pixel for pixel; those of macOS 26
and xcode-27 agree at every point sampled above.

## The disabled push button, checkbox and radio

The renders only: the key window has none of these controls. Uniform fills, light / dark, of the
@2x renders (macOS 14 and 15 identical; macOS 26 and xcode-27 agree at these points):

| Part | macOS 14, 15: enabled → disabled | macOS 26, 27: enabled → disabled |
|------|----------------------------------|----------------------------------|
| push button bezel | #ffffff / #ffffff40 → #ffffff80 / #ffffff20 | #00000014 / #ffffff14 → #0000000a / #ffffff0a |
| unchecked checkbox and radio | #ffffff / #ffffff37 → #ffffff80 / #ffffff1b | #00000019 / #ffffff19 → #0000000d / #ffffff0d |
| checkbox label, densest pixel | #000000d9 / #ffffffd9 → #00000040 / #ffffff40 | #000000d3 / #ffffffd7 → #00000041 / #ffffff3f (antialiased text) |

- A disabled push button's bezel, and its shadow on macOS 14 and 15, are the enabled ones at half
  their alpha; its label is `disabledControlTextColor` over the faded bezel.
- A disabled unchecked box, fill and edge, is the enabled one at half its alpha on macOS 14 and
  15, and at 13/255 against 25/255 on macOS 26.
- A checked box or radio draws the not-key look here, neutral, with no accent; in a key window it
  is the accent, which no capture of this run shows disabled.
- The switch's disabled renders keep the enabled ones' kind of rim on macOS 14 and 15, fainter
  (the off track's 7/255 rising to 12/255 and 20/255 near the edge), and the grey pixel around the
  knob at about half its alpha; on macOS 26 neither has an edge.

Whether a disabled control draws differently in a key window: in the probe's key and not-key
windows every neutral part draws the same (the unselected segment, the switch's off track and
knob, the slider's track and knob, enabled and disabled), and only the accent parts differ, apart
from macOS 26's disabled slider fill. The neutral fills above are taken as the key window's on
that ground, not by a capture.
