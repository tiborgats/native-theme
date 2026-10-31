# Presentations for ADD-A129, moved out of ADD-A126 (at ac413107)

ADD-A126's checks 9 to 12 drafted the picker flyout's presentation; the coordinator moved it to a batch of its own (every popup's presentation on every platform). This file holds the moved text at ac413107, with its citations, and every check finding on it.

## The moved spec text (S2.7)

```
`PresentationKind::PickerClip` is WinUI's picker flyout clip, `PickerFlyoutThemeTransition`, implemented once to its
source as `transitions::picker_clip(opening: bool, t: f32, height: Px, offset: Px) -> (f32, Px)`, the clip's vertical
scale and translation at the step's eased point `t` (`microsoft-ui-xaml@2.5.1
src/dxaml/xcp/dxaml/lib/LayoutTransition_partial.cpp:580-675`; ADD-A126): a vertical clip of the step's part about its
middle (origin `(0, 0.5)`, `:590`, set at `:634`, `:664`), translated at once by the element's `target_offset` `o`
(`OffsetFromCenter`, `:638`, `:668`), its scale moving between the closed scale `max(r, 2r − 1 + 2|o|/h)` (`h` the
part's height, `OpenedLength`; the second where the clip would pass the part's edge, `:615-627`, `:644-659`) and the
open scale `1 + 2|o|/h` (`:629`, `:649`), `r` the source's own ratio, 0.5 opening and 0.15 closing (`:615`, `:644`).
A presentation step plays out where its part's `shown` turns false at its event's update and plays in otherwise: a
`Fade` to transparent or from it, `PickerClip` from the open scale to the closed or the reverse. The part draws at
full opacity through the open (`:640`), and a `Fade` step beside the close's states its fade (`:670-672`). Its key
frames state their own start at time 0 (`:637-640`, `:667-670`), so a picker reopened while its close plays starts
the open from its own start, as an animation that states its `From` does (`src/dxaml/xcp/core/animation/
animation.cpp:748-758`), not from the drawn clip. `target_offset` is the picker field's (its placement target's)
vertical centre's offset from the popup's centre, down positive (`FlyoutBase_partial.cpp:3062-3079`), which the
connector supplies as declared element data (S8).

Semantics (R9): the query is stateless: the connector compares successive query results per element and part, and
```

Also in the spec: `PresentationKind::PickerClip` in the enum; `ElementData.target_offset` and its place in the `element.` data list; the `shown` rule's clause "and a step on it of a presentation event fired at that update, until the step ends (the picker flyout's close, W2-04)"; the shown hiding sentence's picker example; the interrupted-presentation register row cited at the QtQuick member.

## The moved trace rows

| W1-090 | `list.values.presentations (event @indicator_move, part row_selection_indicator) @ variant sidebar` (NavigationView's move, its key frames' durations the source's: on one depth, both pills offset along the list and scaled from their sizes, the offset holding its start to a third of the run and stepping there, the scale through a stretch at that third on the cubic (0.9, 0.1, 1, 0.2) and back on (0.1, 0.9, 0.2, 1), the scale's centre stepping after its own shorter duration, and the outgoing pill's opacity held to a third and falling to 0 on the second cubic, `src/controls/dev/NavigationView/NavigationView.cpp:2097-2112`, `:2185-2240`, `:1999-2002`; across depths each pill scaling on its axis from 1 to 0 or 0 to 1 about an edge, `:2084-2093`, `:2138-2178`; its form needs key frames within a presentation step, several values at fractions of its duration each on its own curve, a step easing among them, which `PresentationStep` lacks, a class 2 change, register-rows-to-add's row "NavigationView indicator move"; ADD-A126) | rule |
| W2-03 | `date_picker.popup.shown (true) @ variant with_popup; date_picker.popup (absent = true) @ !{open} variant with_popup` (CalendarDatePicker's flyout, drawn at once outside the picker flyout's theme transition, its open and close a `Flyout`'s own, register-rows-to-add's row "CalendarDatePicker flyout transition"; S2.7, S6.3, ADD-A126) | rule |
| W2-04 | `time_picker.popup.shown (false), date_picker.popup.shown (false) @ variant text_field; time_picker.popup.shown @ {open} (true), date_picker.popup.shown @ {open} variant text_field (true); time_picker.values.presentations, date_picker.values.presentations events @popup_open (a step on part `popup`: `PickerClip`, `s_OpenDuration`, CubicBezier(0, 0, 0, 1)) and @popup_close (on part `popup`: `PickerClip`, `s_CloseDuration`, the same curve; and `Fade`, `s_OpacityChangeDuration`, delayed by `s_OpacityChangeBeginTime`, Linear), each playing out as `shown` turns false` (both pickers declaring the element data the step reads under `values`, `data = { values = ["target_offset"] }`, S10; the flyout opens and closes by `PickerFlyoutThemeTransition`, which both pickers' flyouts use, `microsoft-ui-xaml@2.5.1 src/dxaml/phone/lib/DatePickerFlyout_Partial.cpp:83`, `TimePickerFlyout_Partial.cpp:81`, `src/dxaml/xcp/dxaml/lib/FlyoutBase_partial.cpp:1955-1972`, in its load and its unload, `LayoutTransition_partial.cpp:570-578`: the open `:613-641`, the close `:642-673`, the curve `:593`, the durations `PickerFlyoutThemeTransition_Partial.h:17-20`, values in the source, S2.7's `PickerClip`; the popup hidden, not `Absent`, so the close's steps draw it and everything in its box, its looping selectors and its buttons included, until they end; `target_offset` the field's vertical centre against the popup's, `FlyoutBase_partial.cpp:3062-3079`, which the connector supplies; a reopen mid-close starting the open from its own start, the key frames stating it, S2.7; ADD-A126) | rule |
| MAT-N16 | `date_picker.popup.shown (true)` (never hidden by `shown`, S2.7, ADD-A126) | field |
| MAT-N17 | `time_picker.popup.shown (true)` (never hidden by `shown`, S2.7, ADD-A126) | field |

## The moved catalogue text

5449:    `min_height` (MAT-N16), `shown` (`windows-11`: false outside `{open}`, the flyout's open and close its widget's
5463:  Vec<PresentationTransition>`: keyed events `@popup_open` and `@popup_close` (declared, S10), the connector firing
7451:    `date_picker.popup` holds the date picker's, MAT-N16): `fill`, `stroke`, `corners`, `shadow`, `shown`
7472:- **Widget values:** `presentations: Vec<PresentationTransition>`: keyed events `@popup_open` and `@popup_close`
9786:| 134 | `date_picker.popup` and `time_picker.popup` hidden by their `transitions` (row 129's parts' `transitions`, a part entering from transparent) | `shown` on the popup outside `{open}`; keyed events `@popup_open`, `@popup_close` with `PickerClip` and `Fade` steps on `popup` (S2.7, W2-04) | ADD-A126: an `Absent` part becoming drawn appears at once (S2.7), and WinUI's picker flyout opens an
| 134 | `date_picker.popup` and `time_picker.popup` hidden by their `transitions` (row 129's parts' `transitions`, a part entering from transparent) | `shown` on the popup outside `{open}`; keyed events `@popup_open`, `@popup_close` with `PickerClip` and `Fade` steps on `popup` (S2.7, W2-04) | ADD-A126: an `Absent` part becoming drawn appears at once (S2.7), and WinUI's picker flyout opens and closes by its theme transition, a clip and a fade, which a part transition does not state; hidden, the popup keeps its style while its close plays |

## The check findings on it, verbatim

### From check-9a.md

### 3. WinUI's picker flyout animates its close, and it opens by a clip at full opacity: "leaves at once" and "enters from transparent" are both false (T-windows W2-04, W:5431-5433, W:201-202)

- **The text.**
  - The batch changed W2-04 to "it enters from transparent by its transitions and leaves at once".
  - The catalogue says the popup is `Absent` outside `{open}`, "entering from transparent by these and leaving at
    once".
  - W:201-202 names "a picker's popup that only opens animated".
- **The source.**
  - `DatePickerFlyout` and `TimePickerFlyout` set `UsePickerFlyoutTheme(TRUE)` (`src/dxaml/phone/lib/DatePickerFlyout_Partial.cpp:83`,
    `TimePickerFlyout_Partial.cpp:81`). `FlyoutBase` then uses `PickerFlyoutThemeTransition` whenever open and close
    animations are on (`src/dxaml/xcp/dxaml/lib/FlyoutBase_partial.cpp:1955-1972`).
  - `DatePicker::ShowPickerFlyout` opens that flyout (`DatePicker_Partial.cpp:795-825`).
  - The transition takes part in both `Load` and `Unload` (`LayoutTransition_partial.cpp:570-578`).
    - On `Unload` it shrinks the clip to 15% over 167 ms and fades the opacity to 0 over the last 83 ms
      (`:641-675`; `PickerFlyoutThemeTransition_Partial.h:17-20`).
    - On `Load` it grows the clip from half over 250 ms, the opacity 1 from the start (`:612-638`).
- **Fix.**
  - State the flyout's open and close as the presentations they are: `values.presentations` events `Appear` and
    `Disappear` with a clip step, or `shown` with a hidden style, once the model can state a clip.
  - Until then, correct the text and record both motions as pending.
  - Drop W:201's example.

### From check-10a.md

### 4. W2-04's flyout as `Appear`/`Disappear` breaks the catalogue's own event rule, and its close has no part to draw (T-win:454, W:5455-5458, W:7461-7462, W:72-73)

- **The text.**
  - `date_picker.values.presentations` and `time_picker.values.presentations` give "events `Appear`, `Disappear`
    (each a `ClipReveal` step on part `popup`)".
  - The popup is `absent = true @ !{open}` "with no transition of its own, the presentations drawing its open and
    close".
- **Against the catalogue.**
  - W:198-206: "`Appear` and `Disappear` are the element appearing and disappearing … queried with the widget's own
    element, they are the widget's". Here that is the date picker appearing, not its popup opening.
  - W:211-213: a motion the enter rule cannot state "is a keyed event whose steps name the part".
  - The analogous WinUI flyout is a separate element: the combo box's popup is the `menu` widget in
    `context combo_box`, with its own `Appear`/`Disappear` (T-win:376, W1-136; W:1366-1370).
- **Against S2.7.** A part that becomes `Absent` "ends every run at once and draws nothing" (S:1319-1321), and the
  part is not laid out (S6.3, S:2652-2653). Nothing says a presentation step keeps drawing a part after it became
  `NotDrawn`, so the 167 ms close cannot be drawn.
- **The "measurement" is source.** The register row (W:72-73) and the trace send "the clip's origin, its offset from
  the selected item and its scales" to the Windows lane's measurement. The source states all of them
  (`LayoutTransition_partial.cpp:613-673`):
  - clip origin (0, 0.5);
  - the clip translated by the picker's `OffsetFromCenter`;
  - open from 0.5 of the height (more where the offset passes the limit) to `(0.5 + |offset / length|) · 2`, on
    cubic (0, 0, 0, 1), opaque throughout;
  - close from that to 0.15, the opacity linear from 1 to 0 beginning at 84 ms.
  - What is missing is the step's form, since `ClipReveal { from: RevealOrigin }` (S:804-806) cannot state them. That
    is a class 2 change, not a measurement.
- **Fix.**
  - Model the picker's flyout as the combo box's is: an element (its own widget or `menu`-like flyout in
    `context date_picker` / `time_picker`) with its own `Appear`/`Disappear`. Or use declared keyed events
    (`@popup_open`, `@popup_close`) with steps on `popup` and a rule that a closing step draws its part until the
    step ends.
  - Move W:72-73 out of the measurements into a class 2 change of `ClipReveal`, citing the values above.

### From check-10b.md

1. **The picker flyout's open and close are stated as the widget's `Appear` and `Disappear`. Those are the picker
   element appearing, not its popup opening, and a `Disappear` has no style to draw once the popup is `Absent`.**
   - Where:
     - T-win:454 (W2-04: "events Appear, Disappear (each a `ClipReveal` step on part `popup`)");
     - W:5455-5457 and W:7461-7462 (`date_picker` and `time_picker` `presentations`);
     - W:5443, W:7442.
   - Evidence:
     - The catalogue's entry format defines `Appear` and `Disappear` as "the element appearing and disappearing",
       queried with the widget's own element, so "they are the widget's" (W:199-204). It also says "A part shown and
       hidden with a state needs no event" (W:205).
     - The popup opens on the state `{open}`, so these events fire when the date or time picker itself appears or
       disappears (a page shown), not when its popup opens.
     - The rename register decided this occasion twice against `Appear`/`Disappear`:
       - row 121 renamed "`date_picker` `Appear`/`Disappear` … of their popup" to `@popup_show`, `@popup_hide` (W:9761);
       - row 129 then replaced those with "the parts' `transitions`" (W:9769).
       The batch goes back to the form row 121 rejected, and adds no register row.
     - The popup stays `Absent` outside `{open}` (T-win:454). At the close the query answers `NotDrawn` at once, and
       `Motion` "ends every run at once and draws nothing" (S:1320-1323). Nothing states that a presentation step
       draws a part the query no longer styles, so the close has no style to clip or fade.
     - The close also fades the opacity to 0 over its last `s_OpacityChangeDuration` (T-win:454's own text). The row
       states each event as a `ClipReveal` step only, so that fade is not stated.
     - The popup's nested parts (`popup_header`, `divider`, the `picker_wheel` elements) are still queried and drawn
       while the popup is `NotDrawn` (W:184-187; finding 5). That contradicts W:7458's "the popup's entering and
       leaving carry them".
   - Fix:
     - Use the catalogue's own route for "a motion the enter rule cannot state": a keyed event whose steps name the
       part (W:214-219). Declare `@popup_open` and `@popup_close` (row 121's names were `@popup_show` and
       `@popup_hide`) on both pickers, with a `ClipReveal` step, and on the close a `Fade` step delayed to its last
       83 ms. Add a register row for the change.
     - Make the popup `shown = false` outside `{open}` instead of `Absent`. It is an overlay, so it keeps no layout
       place in the field, and its style then exists while the close plays. Alternatively, state in S2.7 that a
       keyed hide event's steps draw the part's last drawn style until they end.
     - State the nested parts `absent = true` under the popup's selector, as W:191-192 already does for the parts a
       substitute replaces. With `shown`, its "nested parts" rule covers them instead.

### From check-11a.md

### 5. `PickerClip`'s citations (S:826-834)

| Cited | Says | Correct |
|---|---|---|
| origin `:590` | `openedLength = 0.0` | `:589` (`splitOrigin`), set at `:634`, `:664` |
| translation `:639`, `:669` | blank lines | `:638`, `:668` |
| open scale `:650` | blank | `:629` (open), `:649` (the close's start) |
| close ratio `:646` | `maxOffset` | `:644` |
| fade `:671-673` | | `:670-672` |

The formulas themselves hold:
- the closed scale is `max(r, 2r − 1 + 2|o|/h)`: `pixelsOff/h·2 + r` with `pixelsOff = rh/2 − (h/2 − |o|)`, taken
  exactly when `|o| > h(1 − r)/2`;
- the open scale is `1 + 2|o|/h`;
- the ratios are 0.5 and 0.15;
- the curve is (0, 0, 0, 1) (`:593`);
- the durations are 250, 167 and 83 ms, and the fade begins at 84 ms
  (`PickerFlyoutThemeTransition_Partial.h:17-20`).

### 6. `ElementData.anchor_offset` defines another quantity than the source's (S:3040-3041, S:829)

- **The text.** "its selected item's offset from its centre, down positive".
- **The source.** `offsetFromCenter = centerOfTarget.Y − (popupOffset + presenterSize.Height / 2)`: the placement
  target's (the picker field's) vertical centre against the flyout's (`FlyoutBase_partial.cpp:3062-3079`).
  - The flyout is placed so that its highlight band, vertically centred in the selectors
    (`themes/generic.xaml:12838-12843`), sits on the field (`DatePickerFlyout_Partial.cpp:243-245`).
  - The two agree only while the placement is not clamped. "Down positive" holds.
- **Also.** The name sits beside `badge.values.anchor_offset_*` (W:4590 and after), a different anchor.
- **Fix.** "the field's (placement target's) vertical centre's offset from the popup's centre, down positive", cited.
  Consider a name like `picker_target_offset`.

### 7. W2-04 leaves three things unstated (T-win:455; W:5447, W:5461; S:1489-1490, S:1503-1504)

- **The base.** `date_picker.popup.shown` is stated only `@ variant text_field`.
  - `windows-11`'s other date picker (CalendarDatePicker, W2-03) has no `shown`.
  - S2.7 (S:1504) and S12 now require every preset to state it on a listed part.
  - The other presets' `shown = true` (W:3360, W:5447, W:7448) are in no trace either.
- **The `Fade` direction.** A `Fade` step's direction is unstated in a keyed event (S:806-809). The close needs
  1 → 0.
- **The clip's reach.** The picker wheels are elements in `context time_picker.popup`, not parts nested in it. S2.7's
  new "a part hidden by `shown` hides the parts nested in it" and `PickerClip`'s "a vertical clip of the step's part"
  say nothing of them. WinUI clips the whole presenter.
- **Fix.**
  - State the base `shown` on each lane.
  - Say a `Fade` in a hiding event runs to transparent.
  - Say the clip and the hiding reach the part's nested parts and the elements in its context.

### From check-11b.md

6. **A `Fade` step's direction in a keyed event is unstated, and `PickerClip`'s fields restate the member.**
   - Where: S:807-810, S:826-834, T-win:455, W:5461-5463.
   - Evidence:
     - `Fade` has no direction. In `Appear` and `Disappear` the event gives it, but in `@popup_close` nothing says it
       fades out.
     - `PickerClip` carries `closing: bool` for exactly this need.
     - Its `closed_ratio` is fixed by the direction (0.5 opening, 0.15 closing, `:615`, `:646`), and the member text
       states both. That is two homes for one constant (R20).
   - Fix: state that a step on a part whose `shown` turns false at the event's update plays out (a `Fade` to 0,
     `PickerClip` open to closed), and plays in otherwise. Then `PickerClip` needs no fields, its ratios being its
     source's.

7. **`shown` statements are missing and over-required.**
   - Where:
     - W:5447 (`date_picker.popup.shown`: "`windows-11`: false outside `{open}`");
     - T-win:455 (stated only `@ variant text_field`);
     - W:3359-3361 ("`true` on every other preset") against W:3529-3531 (the pill absent on `adwaita`,
       `kde-breeze`, `macos-*` and `ios-*`);
     - S:2658 ("an absent part need[s] no fields").
   - Evidence:
     - On `windows-11`, CalendarDatePicker's popup (`with_popup`) has no `shown`, so S:3680-3681 refuses the preset.
     - The catalogue's "false outside `{open}`" has it hidden at rest with no presentation.
     - The pill's "`true` on every other preset" asks four presets to state a field of an absent part.
   - Fix:
     - Give the `with_popup` popup on `windows-11` its own row (`shown = true` and `Absent` outside `{open}`, or the
       flyout's own transition).
     - Write "`true` where the part is drawn" (`material-3` alone for the pill).

8. **`anchor_offset` is unreachable through the declaration route, and `PickerClip` has no named home.**
   - Where: S:3040-3041; S:225-232 (the `element.` data list); S:3082-3085 (data declared on a part or under
     `values`; `data_needs`).
   - Evidence:
     - `date_picker` and `time_picker` declare no `data = { values = ["anchor_offset"] }`, so `data_needs` never asks
       the connector for it. `ElementData`'s zero default then centres the clip on every open.
     - "Implemented once to its source" names no function, unlike `transitions::interpolate_color` or `Motion`.
   - Fix: add `anchor_offset` to the element-data list and declare it under both pickers' `values`. Name the
     evaluator, for example `transitions::picker_clip(step, t, h, o) -> (scale_y, translate_y)`.

### From check-12a.md

### 1. "A picker reopened while its close plays starts the open from its own start" is not what WinUI does, and the cited mechanism excludes key frames (S:836-839, T-win:457, R:849-850, register-rows-to-add "interrupted presentations")

- **The text.** The key frames state their start at time 0, "so a picker reopened while its close plays starts the
  open from its own start, as an animation that states its `From` does (`animation.cpp:748-758`)". R and the register
  row call this "settled from its source".
- **The cited lines.** `dxaml/xcp/core/animation/animation.cpp:745-746` reads "Key-frames don't expose From/To/By, so
  this is unnecessary for them", and then `if (!m_fUsesKeyFrames)`. `:748-758` is the non-key-frame path. The picker
  transition is all key frames (`ThemeGeneratorHelper::RegisterKeyFrame`, `dxaml/xcp/dxaml/lib/ThemeGenerator.cpp:112-160`).
- **What WinUI does with the reopen.**
  1. The closing presenter is moved to unloading storage, and the collection notes "we have left the tree, even
     though we might not yet have run leaveimpl" (`dxaml/xcp/components/Collection/UIElementCollection.cpp:1049-1051`).
     For a popup it says it "will not leave the live tree yet" (`:1086-1092`). Its Leave runs only when the unload
     transition releases it (`RemoveUnloadedElement`, `:685-760`).
  2. `FlyoutBase` clears its open registration only in `OnPresenterUnloaded`
     (`dxaml/xcp/dxaml/lib/FlyoutBase_partial.cpp:2299-2313`), so it does this after the close has played.
  3. While the registration holds, a `ShowAt` of the same flyout at the same target and position is ignored
     ("Calling ShowAt on open FlyoutBase with the same placement target and at the same location. Ignoring.",
     `:1708-1724`; `ShowAtCore` then opens nothing, `:1172-1193`).
  - So the source points to a reopen during the close being dropped: the close finishes, and the picker stays closed.
    It does not point to an open restarted from its start.
- **Fix.**
  - State the reopen from these lines: while `@popup_close` plays, the picker's `{open}` is not set again (the
    connector holds it, or the request is dropped). Cite `UIElementCollection.cpp:1049-1051`, `:1086-1092`,
    `FlyoutBase_partial.cpp:1708-1724` and `:2299-2313`.
  - Drop `animation.cpp:748-758`.
  - If the Unloaded timing is still in doubt, send the case to the "interrupted presentations" register row as a
    measurement, and do not call it settled in R:850.

### 2. The new direction rule decides every `Disappear` step as playing in (S:834-835)

- **The text.** "A presentation step plays out where its part's `shown` turns false at its event's update and plays
  in otherwise".
- **The problem.** The sentence is general, and S2.7 states no other direction rule. It covers three cases where it
  decides the wrong way:
  - a step with `part: None` (the whole element), which has no `shown`;
  - a step on a part that lists no `shown`;
  - a `Disappear` event, where nothing turns `shown` false.

  In all of these the step "plays in" by this sentence. Every preset's `Disappear` fades and slides would therefore
  run backward. Examples are W1-136's `Disappear: ClipReveal`, W:2968's spinner "fade … out on stop", and W:3177.
- **Fix.** "A step in a `Disappear` event, or in a keyed event at whose update its part's `shown` turns false, plays
  out (a `Fade` to transparent, `PickerClip` from the open scale to the closed); every other step plays in."

### From check-12a.md

### 5. `PickerClip`'s citations: two still off (S:829, S:836)

- The origin `(0, 0.5)` is `:589` (`splitOrigin`). `:590` is `openedLength = 0.0`. Check 11a's 5 asked for this and
  it was not changed.
- "Its key frames state their own start at time 0 (`:637-640`, `:667-670`)": the start frames are `:636`, `:638` and
  `:640`, and `:666`, `:668` and `:670`. `:637` and `:667` are the end frames. Cite `:636-640` and `:666-670`.
- The rest now hold:
  - `:634` and `:664` (the origin set);
  - `:638` and `:668` (the translation);
  - `:629` and `:649` (the open scale);
  - `:615` and `:644` (the ratios);
  - `:670-672` (the fade);
  - both scale formulas, and the curve (`:593`).

### 6. `ElementData.target_offset`'s comment names the wrong centre (S:3054-3055)

- **The problem.** "its placement target's vertical centre from its own" reads as the element's own centre. The
  source measures from the popup's centre, `popupOffset + presenterSize.Height / 2` (`FlyoutBase_partial.cpp:3077-3078`),
  as S:836-838 says.
- **Fix.** "the placement target's vertical centre from the popup's, down positive".

### From check-12a.md

### 13. W1-089 and W1-090: one line off, the durations unstated, and the meanwhile outcome unstated (T-win:265, T-win:267)

- **W1-089.** `:2244-2255`: `:2244` is `PlayIndicatorAnimations`'s closing brace. `OnAnimationComplete` is
  `:2246-2255`. `:2130-2131` holds.
- **W1-090's description holds at the source:**
  - the offset is held and stepped at 0.333 (`:2216-2219`);
  - the scale stretches on (0.9, 0.1, 1, 0.2) and returns on (0.1, 0.9, 0.2, 1) (`:2221-2225`, `:1999-2002`);
  - the centre steps (`:2227-2230`);
  - the outgoing fade is held to 0.333 (`:2207-2213`);
  - across depths, the pills scale about an edge (`:2143-2159`).
- **What W1-090 lacks.**
  - It says only "durations the source's". It should name them: 600 ms for the run (`:2149`, `:2173`, `:2211`,
    `:2219`, `:2225`) and 200 ms for the centre (`:2230`).
  - Its ranges stop before the `StartAnimation` calls. Cite `:2185-2243` and `:2138-2182`.
  - In top navigation the cross-depth move scales X, but its code sets the centre's `y` (`:2177-2182`).
- **The row's form.** It now states an event with no step. Say what the model draws meanwhile: `Instant`, so the
  pill is set at once at both ends by W1-089.

### From check-12a.md

### 15. The pickers' `shown` is still missing on two lanes, and the catalogue now misstates `windows-11` (W:5449, W:7451; T-ios:618, :620; T-macos:571)

- **The missing rows.** `ios-*` (C-5) and `macos-*` (MAC-N04) draw `date_picker.popup`, and `ios-*` draws
  `time_picker.popup`. None of their traces states `shown`. S2.7 (S:1504) and S12 then refuse them. The eleventh fix
  added the rows for `material-3` only.
- **The catalogue.** `windows-11`: "false outside `{open}`" now contradicts W2-03. Under `with_popup`, W2-03 states
  `true`, and the popup `Absent` outside `{open}`.
- **Fix.**
  - Add `shown (true)` on C-5 and MAC-N04.
  - Write `windows-11`'s entry per variant.

### From check-12b.md

1. **The new direction rule plays every `Disappear` step "in" unless its part lists `shown`.**
   - Where: S:834-836: "A presentation step plays out where its part's `shown` turns false at its event's update and
     plays in otherwise".
   - Problem:
     - The sentence is stated for every presentation step, not only for keyed events on a `shown` part.
     - A step on the whole element (`part: None`), or on a part that does not list `shown` (`None`), never sees
       `shown` turn false, so by the text it plays in.
   - Evidence:
     - No other sentence in the spec or the catalogue states a step's direction. `grep "plays in|plays out"` finds
       only this one.
     - Dozens of rows state `Disappear` steps on elements with no `shown`. Every one of them would fade in as its
       element disappears:
       - `Q1.24`, `Q1.34`, `Q1.91`, `Q1.105`, `Q1.120`, `K1.46†`, `K-KA-19†` (`Disappear: Fade`);
       - `Q2.52` (`Disappear: Fade, Translate drop and Scale`);
       - `Q1.41` (`Disappear: Fade`), `Q1.58` and `Q1.61` (`Disappear: Slide`).
   - Fix: state the direction by event:
     - an `Appear` step plays in, and a `Disappear` step plays out;
     - a keyed event's step on a `shown` part plays out where that part's `shown` turns false at the event's update,
       and in otherwise;
     - for each kind, "out" moves to the kind's `from` end and "in" moves from it: `Fade` to or from transparent,
       `Slide`/`Scale`/`ClipReveal` to or from their `from`, and `PickerClip` from the open scale to the closed or the
       reverse.

2. **`Motion` cannot report a `shown` part leaving by a presentation step, and no text says who runs presentation
   steps. So the closed picker is not drawn by the text's own API.**
   - Where:
     - S:1401-1402 (`Drawn.shown`: "false: nothing to draw, the part hidden and not leaving");
     - S:1503-1508 (a `shown = false` part drawn "while … a step on it of a presentation event fired at that update"
       runs, and "`Drawn.shown` reports it");
     - S:1424-1446 (`MotionUpdate`);
     - S:826-844 (`transitions::picker_clip`, a pure function).
   - Evidence:
     - `MotionUpdate` carries no presentation event or step, and `Motion` has no presentation method. So at
       `@popup_close`'s update, `Motion` sees `shown` turn false with no transition (W2-04 states none), and
       `sample` returns `Drawn.shown = false`. A connector drawing by `Drawn` hides the popup at once, and the close's
       clip and fade never show.
     - Nothing names a presentation player. `picker_clip(opening, t, …)` needs a step clock and its eased `t`, and a
       reopen mid-close needs the running close's steps cut (finding 9). Neither has a home.
     - So `shown`, `PickerClip` and the reopen cannot be implemented once from the text, which is one of this
       check's questions.
   - Fix: choose one, and state it in S2.7.
     - (a) The connector runs presentation steps. `Drawn.shown` covers transitions only, and the connector draws a
       `shown = false` part while a step of an event fired at its hiding update runs. A new event's steps on a part
       cut that part's running steps (WinUI's `CancelTransitions`).
     - (b) `native_theme::transitions` gains the player: for example, `MotionUpdate.events: &[PresentationEvent]`
       with the element's `presentations`, and `Drawn` gaining the step's opacity, clip and transform.
     - (b) is the "implemented once" form the batch uses everywhere else. (a) is smaller. Either way, weigh them in
       R9's presentation paragraph (R:845-850).

### From check-12b.md

9. **The reopen mid-close cites the non-key-frame branch, and the model never states that the close's steps are
   cut.**
   - Where: S:839-841; T-win:457.
   - Evidence:
     - `animation.cpp:745-758` is under `if (!m_fUsesKeyFrames)`, with the comment "Key-frames don't expose
       From/To/By". `PickerFlyoutThemeTransition` registers key frames.
     - The source of the outcome is `Popup.cpp:478`: `CTransition::CancelTransitions(m_pChild)`, which "kills
       currently running animations" as the popup opens. The load's time-0 key frames (`:637-640`) then set the clip
       and full opacity.
     - The text's "the part draws at full opacity through the open" implies the close's `Fade` stops. Nothing says
       it does.
   - Fix: "an event's steps on a part cut the steps still running on it (`Popup.cpp:478`), so a picker reopened
     mid-close starts the open from its time-0 key frames". Cite `animation.cpp` only for non-key-frame animations.

### From check-12b.md

11. **W1-090's presentation has no step kind and no pending marker. The move's interplay with W1-089's zeros is
    undecided.**
    - Where: T-win:267; S:798-803 (`PresentationStep.kind` is required).
    - Evidence:
      - The row dropped `PresentationKind::Morph` and states no kind, so it is not a stateable value.
      - NavigationView keeps the new pill at opacity 0 until the move completes (`NavigationView.cpp:2244-2255`, the
        row's cite). W1-089's `{selected}` zero sets it to 1 at once, so with a move stated later both pills would
        draw.
    - Fix: state the row pending (`presentations = []` with its queue id, as S22 ADD-A126's "a pending list `[]` with
      its id" has it), and name the interplay in the deferral.

12. **The catalogue's `date_picker.popup.shown` for `windows-11` does not match W2-03.**
    - Where: W:5449 ("`windows-11`: false outside `{open}`") against T-win:443 (W2-03: `shown (true) @ variant
      with_popup`, and `absent = true` outside `{open}`).
    - Fix: "`windows-11`: false outside `{open}` under `text_field` (W2-04); under `with_popup` `true`, the popup
      `Absent` outside `{open}` (W2-03)".

13. **The direction of `PickerClip` and `Fade` is stated twice.** W2-04 (T-win:457) restates "each playing out as
    `shown` turns false", and S:834 states it too.
    - Fix: once finding 1 states the rule, drop the trace's copy (R20, one home).

14. **The presence of elements nested in a hidden `shown` part is unstated.**
    - Where: S:1515-1517; S:1352-1356 and S:1397 (`Presence`).
    - Evidence:
      - The closed picker's wheels and buttons are elements with their own `Motion`.
      - Whether the connector reports them `Hidden`, `Unpainted` or `Removed` decides whether their transitions run
        (`GtkCss` defers only under `Hidden`; Breeze defers under all but `Painted`).
      - For `windows-11`, WinUI unloads the popup's child, so `Removed` or destroyed is the likely answer. WinUI's
        not-drawn behaviour is in the register already.
    - Fix: "elements nested in a `shown` part that rests hidden report `Hidden`", or the WinUI measurement's answer.

## Decisions

- **Withdrawing the dead check.** R:900-904 weighs only "refuse in every layer". It does not weigh:
  - the check on bundled presets only (S19.1's build test);
  - a warning;
  - one spelling, which removes the cause.

  Finding 3 shows that the cause survives the withdrawal.
- **`PickerClip` fieldless, its direction by `shown`.** It is the right reduction (11b's finding 6). The direction rule
  went wider than its case (finding 1).
- **The "exactly when" rule.** It is a good single criterion, but it needs the measurement case and GNOME's marks
  decided by it (finding 4).
- **S2.10's `Absent` clause.** The live-state end was not weighed (finding 7).

## Complexity

The round removed one check (S2.2's clause, S12's bullet, three worked examples and R's argument for it) and
`PickerClip`'s two fields. It added about 1000 words net (word diff: +1664, −664). Most of that is needed:
- MAT-SG1's real entrance;
- W1-107's press shrink;
- the `PickerClip` direction and reopen;
- `target_offset`.

These are longer than their sources need:
- W1-090 (T-win:267) describes NavigationView's key frames in about 130 words for a form it defers. The queue row
  should carry that.
- MAT-C1's new settling digression (T-mat:109) cites five ranges for a difference "under a device pixel".
- S2.10's `Absent` clause (finding 7).
- The S22 row now restates the dead check and its withdrawal in history.

With findings 1, 2, 3 and 7, the batch would be simpler than at a683977e:
- one direction rule, scoped;
- one presentation owner;
- one spelling of state-drawn blocks;
- one definition of the indicator's ends.

### From check-12b.md

5. **Three bundled presets fail S12 as traced: `ios-*` and `macos-*` state no `shown` on the picker popups they draw,
   and `material-3`'s segment check states a base zero that overrides nothing.**
   - Where:
     - S:1513-1514 ("on a part that lists it every preset that draws the part states it");
     - W:5449 and W:7451-7452 ("`true` where another preset draws it");
     - T-ios:618-620 (C-5: `date_picker.popup.{fill,corners,shadow}`, and `time_picker.popup.{…} @ variant
       with_popup`);
     - T-macos:571 (MAC-N04: `date_picker.popup.{fill,corners,shadow,padding} @ variant text_field`);
     - T-mat:444 (MAT-SG1's base `transitions (Opacity, Transform: a zero duration, engine ComposeTransition)`);
     - S:3683-3686 (a stated zero with no delay must override an earlier transition of its groups).
   - Evidence:
     - This round added `shown (true)` for `material-3`'s popups (MAT-N16, MAT-N17) and nothing for the two Apple
       lanes. `grep "popup.shown"` finds rows only in the Material and Windows traces.
     - MAT-SG1's zero is in the check's base. No transition of `Opacity` or `Transform` comes before it in its list,
       in its base or in an earlier rule. Its `{selected}` rule comes after and does not apply where the zero does.
       So S12 refuses it, under "where none can, no transition is stated". It is also unneeded: with no transition in
       the base, the check takes its hidden values at once (S:859-860).
   - Fix:
     - Add `date_picker.popup.shown (true)` and `time_picker.popup.shown (true)` to C-5 and MAC-N04.
     - State whether a `DrawnAs` part needs `shown` (K2.23†, K-KA-20† draw the popup as `menu` and `dialog`). Since
       it is its substitute's element (S:1347), it should not, and S:1513 should say "every preset that draws the
       part itself".
     - Drop MAT-SG1's base zero.

## The register rows moved with it

From `register-rows-to-add.md`, removed there and listed here:
- interrupted presentations (every lane): a presentation started while another runs on the same element (a popover
  dismissed while it appears; Kirigami's `OverlayDrawer` reversed; QML's `reversible` transitions); the picker
  reopened mid-close (12a's 1: WinUI likely drops the reopen, `FlyoutBase_partial.cpp:1708-1724`, `:2299-2313`;
  12b's 9: `Popup.cpp:478` cancels running transitions);
- NavigationView indicator move (`windows-11`): key frames within a presentation step (W1-090);
- CalendarDatePicker flyout transition (`windows-11`): `FlyoutBase_partial.cpp:1971-1975`, the `PopupThemeTransition`;
- full-screen title bar reveal (`windows-11`, W-P3-2†): a revealed condition;
- the pickers' popup `shown` on `ios-*` (C-5) and `macos-*` (MAC-N04), missing at ac413107 (12a's 15, 12b's 5).
