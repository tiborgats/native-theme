# ADD-A126's inputs: what ADD-A125 removed

The texts ADD-A125 drafted and withdrew, with their citations and the independent checks' findings on them, kept as ADD-A126's starting point (ledger queue #85, #95, #103). Archived with ADD-A126 once it converges.

## Interruption


Removed from `spec-a125` in the commit "spec ADD-A125, the sixth check's fixes; interruption moved to ADD-A126". The texts
were in `docs/todo_v0.7.1_model-completeness-spec.md` (S2.7, S11, S12, S4.3), the rationale (R9), the KDE trace
(K1.17, K1.30, K1.44, K3.31) and the catalogue (`token.theme_change`, adwaita). ADD-A125 now states only: what a running
transition does when a change interrupts it is not yet stated; ADD-A126 states it (ledger queue #103).

## 1. The type

- `StateChange { conditions, interruption: Interruption }` and `pub enum Interruption { FromDrawn, ReverseOrRestart }`
  (earlier `reverses: bool`). `FromDrawn`: the next transition starts from the value drawn. `ReverseOrRestart`: an undo
  enters the next at `1 − p`, any other change starts it from the old style's own value.
- S11's spelling `interruption = "reverse_or_restart"`, default `"from_drawn"`; S12's checks: `ReverseOrRestart`
  refused on a spring (`duration` can be `Absent`), with a non-zero `delay`, or on a kind other than `Interpolate`, on
  both the running and the entered transition.

## 2. The rules and their citations

- **From the drawn value, full duration**: GTK starts a new transition from `gtk_css_style_get_value (source, i)`, the
  drawn value (`gtk@4.24.1 gtk/gtkcssanimatedstyle.c:870-875`); libadwaita's `AdwAnimation` replays from the widget's
  current value (`libadwaita@1.10.0 src/adw-tab.c:129-133`, played only when the target changes, `:126-133`); Compose's
  `animateTo` (state layer `material3-ripple/…/ripple/Ripple.kt:280-322`, `:283` keeps a transition whose interaction is
  unchanged).
- **Undo at `1 − p`** (Breeze's direction turn): `kstyle/animations/breezewidgetstatedata.cpp:24-27` (direction turned,
  no restart while running); `kdecoration/breezedecoration.cpp:278-279` (direction, then `setEasingCurve(isActive ?
  OutCubic : InCubic)`; `:280` starts only if not running); Qt keeps `currentTime` across `setDirection`
  (`qtbase@6.12.0 src/corelib/animation/qabstractanimation.cpp:1149-1184`; reset only when stopped, `:1159-1166`) and
  applies the curve to `currentTime / duration` (`qvariantanimation.cpp:200-202`, `:358-364`). Walks: decoration
  deactivated at 0.5 jumps `1 − OutCubic(0.5) = InCubic(0.5) = 0.125`, reactivated 0.2 D later enters at 0.3 and
  finishes in 0.7 D; line edit returns in 0.5 D, rehovered finishes in 0.7 D. Shadow strength while animating
  `0.5 + m_shadowOpacity * 0.5` (`breezedecoration.cpp:783-808`).
- **Kept unseen where no transition applies** (Breeze's button pen: highlight drawn at once while the engine runs on,
  `breezehelper.cpp:710-711`, `:721-724`; the fill's mix is `:719`).
- **"Any other change" starts from the old style's own value**: claimed from `breezewidgetstateengine.h:47-73` and
  `breezehelper.cpp:146-166`, `:715-725`. Check 6 showed it false: `frameAnimationMode`/`frameOpacity` rank engines by a
  fixed order (enable, focus, hover), not recency; the hover engine runs on `mouseOver && !hasFocus`
  (`breezestyle.cpp:3939-3940`, `:4069-4074`); while the focus engine runs the outline is `mix(mouseOver ? hover : rest,
  focus, opacity)` (`breezehelper.cpp:146-152`), its start end following the live hover flag. The button's fill and pen
  read separate engines (`breezestyle.cpp:4546-4551`); `buttonAnimationMode` (`.h:77-103`) is unused there.
- **Running transition whose end is unchanged** (`ValueChange`): GTK carries it only where the new style's transition
  for the property has `duration + delay` non-zero (`gtkcssanimatedstyle.c:822-826`, `:836-861`, `:840-842`).
- **A change the trigger does not start** (condition, input, layer or theme under `StateChange`): kept progress, ends
  retargeted (Breeze mixes the live palette by its engine opacity, `breezehelper.cpp:715-725`).

## 3. Findings the rules did not cover (check 6a, 6b)

- Breeze's input frame, cases A to C (focus fade with hover leaving or entering): `breezehelper.cpp:146-152`,
  `breezewidgetstateengine.h:47-73`, `breezestyle.cpp:3939-3940`, `:4069-4074`.
- Breeze's tab-bar and header-view engines restart on every change, undo included, with one "previous" slot per bar:
  `breezetabbardata.cpp:20-26`, `:70-90`; `breezeheaderviewdata.cpp:44-66`; `restart()` in `breezeanimation.h:37-44`;
  Qt rewinds a stopped animation, `qabstractanimation.cpp:944-953`. Only `WidgetStateData`, `SpinBoxData` and
  `ScrollBarData` turn the direction. Rows K1.132 (`tab.tab.transitions`), K1.177 (`list.header_section.transitions`),
  the tab's focus line (`breezestyle.cpp:7110-7112`).
- Compose's springs keep the running velocity: `Animatable.animateTo(..., initialVelocity = velocity)`
  (`androidx@e4d933c9 compose/animation/animation-core/src/commonMain/kotlin/androidx/compose/animation/core/Animatable.kt:227-230`);
  the switch thumb (`Switch.kt:180-190`, `FastSpatial`); `TweenSpec` ignores velocity; libadwaita's
  `AdwSpringAnimation` takes velocity from widget code; UIKit's additive animations: to survey.
- Options not yet weighed: the `reverses` bool; an interruption field on `Transition` for both triggers; a platform
  engine semantics implemented once to its source (as `PlatformCurve`); a stated limitation.
- Breeze's duration default 100 ms (`breeze.kcfg:53-55`), bounding any residual.

## Transition start and end, entering and leaving, the marks' draw-on


Removed from `spec-a125` in "spec ADD-A125, the eighth check's fixes; transition start and end moved to ADD-A126".
ADD-A125 now states (S2.7): beyond ADD-A124's rules, when a transition starts and ends and what interrupts it is not
yet stated: ledger queue #85, #95, #103 (orch-docs), ADD-A126 to come. `suspended_by_theme_change: bool` and
`Transition.from` ("held before") are ADD-A124's text again. Interruption's own removal list is
`interruption-removed.md` (same folder); the texts as they stood are in `_snap.md`.

## 1. `Transition.from`: the flags a change clears (38af1c0f)

- Form: a transition with `from` applies to a change that clears one of its flags (held before, not after, after the
  exclusive drop); `transitions::governing(…, before, after)`.
- Evidence: Compose takes its outgoing elevation spec only on going to its default state, `to == null`, `from !=
  null` (`androidx@e4d933c9 …/material3/internal/Elevation.kt:46-62`, `:96-113`); Breeze draws the highlight at once on
  entering a flag and fades only as it turns off (`breeze@6.8.2 kstyle/breezehelper.cpp:710-725`; engines flip on
  their flag, `kstyle/breezestyle.cpp:4547-4549`, `kstyle/animations/breezewidgetstatedata.cpp:12-29`).
- Check 2: the K1.30 window-deactivation example was false (the hovered pen is the Active highlight in every window
  state, `breezehelper.cpp:673`, `:710-711`); no bundled fact changes meaning between the two readings.

## 2. `Transition.trigger` (bdf92e88 to 259cb7ee)

- Type: `TransitionTrigger { ValueChange, StateChange { conditions: Vec<Condition> } }`; S11 `"value_change"`,
  `"state_change"`, `{ state_change = { conditions = […] } }`, left out beside `from`; S12: no theme-change condition
  (`Scheme`, contrast, `Preset(_)`), no `ContainerSize`, no trigger beside `from`; `TransitionChange { before, after,
  conditions }`, `Resolution.conditions_changed`; "applies" by trigger and `from`; a layout move starting only
  `ValueChange` `Geometry` transitions; a state change in a theme change's frame under each kind.
- Citations: `ValueChange`: GTK creates CSS transitions wherever a value differs (`gtk@4.24.1
  gtk/gtkcssanimatedstyle.c:808-880`); QML `Behavior::write` (`qtdeclarative@6.12.0 src/quick/util/qquickbehavior.cpp:361-372`);
  Compose `animate*AsState` (`SingleValueAnimation.kt:57-64`; the target handed every recomposition,
  `animation-core/…/AnimateAsState.kt:430-441`); WinUI brush transitions (`microsoft-ui-xaml@2.5.1
  src/dxaml/xcp/core/core/elements/Border.cpp:42-60`, `ContentPresenter.cpp:150-166`; only between solid colour brushes,
  `components/brushes/WUCBrushManager.cpp:497-509`, `:524-534`); Compose's elevation `LaunchedEffect(target)`
  (`Button.kt:1752`, target from interaction and `enabled`, `:1736-1750`). `StateChange`: Breeze's engines
  (`breezewidgetstatedata.cpp:12-29`; line edit `breezestyle.cpp:4069-4070`; tab bar `breezetabbardata.cpp:70-90`;
  header view `breezeheaderviewdata.cpp:44-66`); the decoration shadow on window activation only
  (`kdecoration/breezedecoration.cpp:241`, `:275-286`; maximise, tiling, shading and settings set it at once,
  `:231-247`, `:389-395`); libadwaita's `AdwAnimation`s (`adw-tab.c:111-133`); Compose's state layer
  (`material3-ripple/…/Ripple.kt:313-322`); WinUI's visual states (`dxaml/lib/Button_Partial.cpp:12-60`; `GoToState`
  into the current state does nothing, `components/vsm/CVisualStateManager2.cpp:103-107`). Measured: Core Animation,
  UIKit, Mica (W3-10).
- Check findings on it: 8a m1 (a scheme change cannot test a trigger: it is a theme change; use a window
  deactivation, `AppInactive`, `BoldText`, `ReducedTransparency`); 8a m2 (WinUI lines above); 8a m5 (R9 wrong under
  `Suspend`); 8a m6, 8b S2 (Compose's elevation `ValueChange` and `StateChange` by its `from` at once); 8b S1 (a
  single `Value<Transition>` states `from = "none"`, so "beside a `from`" refused libadwaita's ring's
  `value_change`); 8b S5 (focus flags in an inactive window undefined: Qt clears the focus widget,
  `qtbase@6.12.0 src/widgets/kernel/qapplication.cpp:1890-1892`, `qwidget.cpp:6494-6506`; GTK unsets FOCUSED,
  FOCUS_WITHIN, FOCUS_VISIBLE, `gtk/gtkwindow.c:6172-6175`, `:5230-5233`, `:5270`; AppKit keeps the first responder in a
  non-key window, hides the ring: measure); 8b m13 (Windows and Apple trace rows); 8b m16 (presentations and the press
  ripple in a `Crossfade` frame undecided); 7b m15 (a layout move).

## 3. `suspended_by_theme_change` (#95) forms tried

- `Option<bool>`, stated only where the theme change can be `Suspend` (c12f8e6e); read at resolution, form checked by
  the gate (d9df68aa); a zero-length `Suspend` for Breeze (d9df68aa); replaced by the trigger, `Suspend` reaching the
  `ValueChange` transitions (bdf92e88). Evidence: libadwaita's provider `* { transition: none; }`
  (`adw-style-manager.c:476-477`, `:237-239`); the tab's close-button `AdwAnimation` (`adw-tab.c:857-862`); GTK carries
  no transition into a style whose one transition's duration and delay sum to zero (`gtkcssanimatedstyle.c:822-826`,
  `:836-861`, `:840-842`); Breeze's engines (above); Kirigami's colour behaviours (`kirigami@6.31.0
  src/controls/NavigationTabButton.qml:83-84`, `:118`, `:136`; `qqc2-desktop-style@6.31.0
  org.kde.desktop/private/SwitchIndicator.qml:87-93`); KWin's Blend Changes out of scope (K-P3-15†).
- Removed with it: `kde-breeze`'s kind `Instant` (from the trigger), S4.3's reach paragraph, the suspension jump
  sentence, the `Crossfade` and `Instant` sentences on a state change in the change's frame (8b m15: the
  `Crossfade` parenthetical "each property is at its new value" decides running transitions too).

## 4. The rest ring (#85)

- Forms: the rest style holds the ring's start geometry (c12f8e6e); the offset arithmetic `width + 4px` (`4px` where
  `$outer`) (d9df68aa); drawn while its transition out runs (bdf92e88); entering and leaving at the rest ring with
  transparent paint (259cb7ee).
- Evidence: `libadwaita@1.10.0 src/stylesheet/_drawing.scss:12-27`, `_common.scss:3-5` (outline colour, width,
  offset over 200 ms).
- Check findings: 7b S1 (a rest ring equal to the focused one loses its fades); 8a S1, 8b S3 (the examples false or
  unmeasured: Breeze's `QFocusFrame` attaches and detaches at once, `breezestyle.cpp:1667-1678`, `drawFocusFrame`
  `:6613-6781`, `facts/kde/tokens.toml:818-822` `token.focus.transition = "absent"`; K3.24 is the focus lines,
  `:5743-5756`, `:7692-7705`; the tab line is drawn only `hasFocus || animated`, `:7110-7115`, and only while
  selected, `:7111`; AppKit's ring unmeasured, U94, U100, U134, U137, U140; the gap map's "focus-ring appear"
  recording, `gap-map-macos.md:475`, `:477`); 8b S4 (a part `Absent` at rest: style after or style before; the group
  box line fades in only, `breezestyle.cpp:7683-7686`; MAC-L14); 8b m7 ("rest style" undefined; the ring's `shadow`
  left at full colour); 8b m8 (`FocusMove`, X-FO-2); 8b m9 (a ring's transition has no property group, K3.24's
  "group Focus"); 8b m10 (K1.129's base transition fades a line Breeze drops); 8b m12 (R12 lacks the decision).

## 5. Trace rows reverted

K1.17's shadow trigger row; K1.30's and K1.44's trigger clauses; K3.31's `token.theme_change (Instant …)`; K-P3-15†'s
`Instant`; GN-3.23's trigger row; GN-3.28's rest-ring row; MAT-X9's `state_change`; MAT-X13's trigger row; W3-10's
trigger measurement; the catalogue's trigger Open item and `token.theme_change` lines.

## 6. Added by check 9 (ADD-A125 now marks these pending ADD-A126)

- S4.3's "a transition already running when a suspension starts finishes" (ADD-A124's) is false on libadwaita: GTK
  returns before the carry-over where the new style's one transition has zero duration and delay
  (`gtk@4.24.1 gtk/gtkcssanimatedstyle.c:822-826`, carry-over `:846-860`), so the running CSS transition jumps. Now
  "not yet stated (#103)", as is R9's copy; the `Crossfade` parenthetical "each property is at its new value under the
  fade" is cut.
- `FocusKind`'s "drawn only while `FocusVisible`" (and R12's "shown in the focus-visible state") decided the ring's
  leaving: libadwaita's ring fades out over 200 ms (`_drawing.scss:13-17`, `_common.scss:3-5`); Breeze's tab line is
  drawn while `hasFocus || animated` (`breezestyle.cpp:7106-7115`), the checkbox label line while its engine animates
  (`:5740-5756`). Now "how it enters and leaves is not yet stated (#85)"; GN-3.28, K1.129, K3.24 marked pending.
- K3.24's `PartStyle.transitions group Focus` names no `PropertyGroup` (`spec-trace-kde.md:1018`); the GNOME fact
  `token.focus.transition` states `properties = []` (`facts/gnome/tokens.toml:1638`), which ADD-A122's rule refuses.
- R9's condition-change example (Breeze's decoration shadow for maximise and tiling) now scoped to activation, which
  changes start which transitions pending (#95).

## 7. Added by check 12 (stated nothing about it in ADD-A125)

- A checkbox in Compose's Material 3 styling branch replaces its indication node on every toggle: the ripple colour
  `indicatorColor(state)` changes with the state (`androidx@e4d933c9 …/material3/Checkbox.kt:510-515`, `:918-922`),
  `ripple(…)` is called each composition (`:554-560`), `RippleNodeFactory.equals` compares `color`
  (`material3/Ripple.kt:718-731`), the clickable undelegates the old node and creates a new one
  (`foundation/Clickable.kt:1717-1719`, `:1762`, `:1874-1896`), which collects interactions only from its attach
  (`material3-ripple/…/ripple/Ripple.kt:228-239`; no replay, `foundation/interaction/InteractionSource.kt:135-140`): the
  press ripple's exit is cut, and no hover or focus layer shows until the next hover or focus event. The default branch's
  constant colour keeps the node. ADD-A125 takes the branch's geometry only; the lifecycle is ADD-A126's (#85, #103).

## 8. Added by check 14 (removed from ADD-A125)

Removed texts, verbatim:

```
a part that becomes drawn enters from transparent with its own
geometry, its whole drawing at zero opacity, as the toolkits fade a whole part by its opacity (Compose's `fadeIn` and
`fadeOut`, a graphics layer's `alpha`, `androidx@e4d933c9
compose/animation/animation/src/commonMain/kotlin/androidx/compose/animation/EnterExitTransition.kt:1546`), its colours
unchanged, and one that becomes absent leaves to transparent likewise (transparent black is a paint's that becomes absent
while its part stays drawn, above; ADD-A125), by the transitions of the style it goes to or comes

----

A platform whose own transparent is another zero-alpha colour (XAML's `#00FFFFFF`,
`microsoft-ui-xaml@2.5.1 src/controls/dev/CommonStyles/Common_themeresources_any.xaml:20`, `:25`) states the paint it
fades to or from as that colour, a stated paint, not `Absent`.

----

(its scale animation, W1-041; absent while unchecked, it enters from transparent by these, S2.7's transparent, on
    `windows-11` the platform's own `#00FFFFFF` as a stated paint once WinUI's arithmetic is measured).
```

Findings on them:
- 14a S1: the "whole part fades by its opacity" rule generalised from `AnimatedVisibility` (`EnterExitTransition.kt:1546`,
  `fadeIn`/`fadeOut` `:328`, `:348`), but Compose's own checkbox mark fades by its colour to `Color.Transparent`
  (`Checkbox.kt:378`) with `animateColorAsState` (`:634`), `FastEffects` out and `DefaultEffects` in (`:1040-1049`),
  in Oklab (`ColorVectorConverter.kt:31-48`), the path snapping out after 100 ms (`:603`, `:1061`): a paint to
  transparent black, not an opacity fade; at mid-fade Oklab darkens the mark where an opacity fade keeps it white.
  Suggested: state the mark drawn with `glyph.fill` transparent outside `{checked}`/`{indeterminate}` on `material-3`.
- 14b m3: which group times a whole part's fade (the catalogue's `Opacity`, `spec-widgets.md:184-186`, pre-batch).
- 14b m4: WinUI's radio dot (W1-041): the `#00FFFFFF` stated-paint clause contradicted the opacity rule; under ADD-A124's
  "enters from transparent" the dot is `Absent` unless WinUI fades its brush, then drawn with that paint stated.
- 14b m8: options not set down: the paint at its own colour and zero alpha; transparent-black paints for a whole part;
  forbidding `Absent` for a faded part (every such part drawn at `opacity` 0).
- XAML's own transparent `#00FFFFFF` (`microsoft-ui-xaml@2.5.1 src/controls/dev/CommonStyles/Common_themeresources_any.xaml:20`,
  `:25`): a fade to it in a straight space goes toward white.

## 9. Added by check 16 (removed from ADD-A125)

- MAT-C8's `checkbox.mark.transform @ {checked} / {indeterminate}; checkbox.mark.transitions (Transform)`, given as
  "the check's shift to the indeterminate dash animated, `:614-629`". Compose applies no transform: it animates
  `checkCenterGravitationShiftFraction` (`androidx@e4d933c9 compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Checkbox.kt:614-629`,
  `DefaultSpatial` both ways from a drawn state, `:596`, snapping from and to `Off`) and `drawCheck` moves the path's
  three points by `lerp` toward the centre line (`:720-724`). The model states the two paths as their states' values
  (MAT-P3-8† `glyph`, MAT-C6 `glyph @ {indeterminate}`); no `Glyph` transition between two `GlyphSource::Path` values
  is defined (S2.4's `morph` covers outlines and shapes only), so the dash is swapped in at once. Options for
  ADD-A126: a `Glyph` transition that interpolates two paths with the same commands point by point (Compose's
  `lerp`), a class 2 change made with this fact; or the shift stated unstated, a ledger row. KDE's mark is timed by
  `Glyph` already (K1.55: `checkbox.mark.glyph.draw_extent … transitions (Glyph)`).
- MAT-C8's `radio.dot.transform @ !{checked} / {checked}` and `Transform` in `radio.dot.transitions`: the dot is
  absent unchecked (Compose draws it only while its animated radius is above zero, `RadioButton.kt:188-190`, the
  target `RadioButtonDotSize / 2` or 0, `:91`, by `FastSpatial`, `:137-142`), so its scale is how it enters and leaves
  (#85), ADD-A126's. The dot's colour keeps `Fill` (`DefaultEffects` while enabled, `:144-147`).
- S2.7's deferral sentence now names a part's entry and exit (a focus ring, a Material check mark or radio dot drawn
  by its fraction or radius), not the ring alone.

## 10. Added by check 17 (removed from ADD-A125)

- S2.7's entering sentence (ADD-A124's): "a part that becomes drawn enters from transparent with its own geometry,
  and one that becomes absent leaves to transparent, by the transitions of the style it goes to or comes from". It
  decided entry for every part `Absent` in a state; it now reads: a part its platform draws on or grows is drawn in
  every state at its hidden values (`Glyph.draw_extent` 0, a zero size or scale), and how a part `Absent` in a state
  enters and leaves is ADD-A126's. The catalogue's general paragraph (`spec-widgets.md`, Presentations) and the
  per-part "it enters from transparent (and leaves to transparent) by these, S2.7" clauses went with it:
  `combo_box.item_selection_indicator` (W1-135), `segmented_control.check` (MAT-SG1), `slider.value_indicator`
  (MAT-S6), `list.row_selection_indicator` (MAT-N13), the pickers' `popup` (W2-04), the check mark, the radio dot;
  and the trace notes of W2-04 and MAT-S6.
- For ADD-A126: the same test (drawn on or grown in place, or drawn nothing at all) may apply to the selection pill
  (WinUI scales it), the segmented check (Material's show and width animation) and the value bubble (its scale).
- The draw-on engines: Breeze's dash pattern toward checked and nothing toward unchecked
  (`breeze@6.8.2 kstyle/breezehelper.cpp:953-960`), its dot's radius both ways (`:1069-1071`); WinUI's accept visual
  by `TrimEnd` on and `TrimStart` off (`AnimatedAcceptVisualSource.cpp:437`, `:470`, `:522-553`), its pressed unchecked
  dot growing (`RadioButton_themeresources.xaml:298-306`, `:376`); Compose's drawn fraction (`Checkbox.kt:595-612`,
  `SnapAnimationDelay` `:1061`) and dot radius (`RadioButton.kt:137-142`). Option for #85: `draw_extent` and
  `draw_start` transitions run as the platform's trims (`TrimEnd` the extent, `TrimStart` the start).

## 11. Added by check 18 (removed from ADD-A125: the marks' draw-on)

ADD-A125 now states the rest looks alone: the check mark `Absent` `@ !{checked,indeterminate}` on every preset, the
radio dot `Absent` `@ !{checked}` (on `windows-11` `@ !{checked,pressed}`, the pressed unchecked dot a rest look).
Removed, for ADD-A126:

- The form check 17 decided: the mark and dot drawn in every state where the platform draws them on or grows them,
  at hidden values (`Glyph.draw_extent` 0, a zero size or scale), the draw-on and growth plain transitions of those
  fields. Rows removed or emptied: S2.7's "a part its platform draws on or grows in place … is drawn in every state";
  the catalogue's checkbox `mark` and radio `dot` per-platform lists; K1.55's `checkbox.mark.glyph.draw_extent @
  !{checked,indeterminate} / {checked} / {indeterminate}; checkbox.mark.transitions (Glyph)` and `radio.dot.transform
  @ !{checked} / {checked}; radio.dot.transitions (Transform)`; W1-039's `checkbox.mark.glyph.draw_extent @
  !{checked,indeterminate}`; W1-037's unchecked mark colours as drawn; MAT-C8's `checkbox.mark.glyph.{draw_extent,fill}
  @ !{checked,indeterminate}; checkbox.mark.transitions (Glyph, Fill)` and `radio.dot.transitions (Geometry, Fill; …)`;
  MAT-C1's `radio.dot.{min_width,min_height} @ !{checked}` and the dot's colour in every unchecked state.
- Check 18b's findings on that form: `draw_extent` is defined for a path only (S2.9), so it has no reading on a Segoe
  code point (WinUI's fallback), Breeze's filled-rectangle dots (`breezehelper.cpp:948-952`, drawn in steps at ⅓, ⅔ and
  1, `:964-974`, with the transparent brush during the run, `:935-936`) or Kirigami's `ThemeIcon` (K-KA-46†); WinUI's
  draw-on was stated twice, as a `Glyph` change and as the `Keyed` AnimatedIcon transition of the `Icon` group, which
  the mark (a `glyph` part) lacks (S2.7); WinUI's draw-off is `TrimStart` 0→1 (a rising `draw_start`), not a falling
  extent; an unscoped `Glyph` transition would draw the check off on uncheck where Breeze draws nothing
  (`breezehelper.cpp:957-959`) and Compose snaps after `SnapAnimationDelay` (`Checkbox.kt:604`, `:1061`); the dot's
  growth was stated in two forms (Breeze's `transform`, Compose's size) though both animate the radius. Options for
  WinUI: (a) the accept visual's check as a stroked `GlyphSource::Path`, `draw_extent` on and `draw_start` off, the
  `Keyed` transition dropped; (b) `Keyed` alone, the mark `Absent` unchecked.
- The per-platform sources: Breeze's check by a dash pattern `path.length() * animation` toward checked, nothing
  toward unchecked (`breeze@6.8.2 kstyle/breezehelper.cpp:954-962`, `kstyle/breezestyle.cpp:4923-4929`), its dot
  `radius * animation` both ways (`breezehelper.cpp:1070-1072`, `breezestyle.cpp:4961-4966`,
  `kstyle/animations/breezewidgetstatedata.cpp:24`); WinUI's accept visual, `TrimEnd` on and `TrimStart` off on its
  stroked path (`microsoft-ui-xaml@2.5.1 src/controls/dev/AnimatedIcon/AnimatedVisuals/AnimatedAcceptVisualSource.cpp:437`,
  `:470`, `:522-553`), drawn while it trims off in the unchecked foregrounds (`CheckBox_themeresources.xaml:65-68`,
  `:153-156`, `:241-244`, `:319-320`, `:344-345`, `:369-370`), its pressed unchecked dot growing from its template
  dimensions (`RadioButton_themeresources.xaml:298-306`, `:376`); Compose's drawn fraction, `DefaultSpatial` from Off
  and a snap after `SnapAnimationDelay` to Off (`Checkbox.kt:595-612`, `:1061`), its mark colour animated in every
  state (`:634`, `:1040-1049`), its check moved to the dash by `lerp` (`:615-631`, `:720-724`), its dot radius by
  `FastSpatial` (`RadioButton.kt:137-142`) and its colour snapped at disabled (`:144-152`); GTK's icon source swapped
  with no transition (`libadwaita@1.10.0 src/stylesheet/widgets/_checks.scss:70`, `:76`, `:80`, `_common.scss:6-7`).
- Still open: AppKit's mark and dot animation (MAC-X11's recording), UIKit's multiselect check (CK-2†), and a checked
  WinUI radio's dot while pressed (two groups set the pressed dot's opacity, `RadioButton_themeresources.xaml:298-300`,
  `:353`): register rows.

## The marks' and dots' representation (rest look and motion)


ADD-A125's twentieth check moved the representation of the check mark and the radio dot to ADD-A126 whole: the
rest look and the motion together. The catalogue and the traces now stand as they did before ADD-A125 touched them,
each affected row marked "the mark's and dot's representation is ADD-A126's, ledger queue #85, ADD-A125". This
section records what ADD-A125 tried, what it removed and what the checks found. Citations are at the pins in
`/run/media/tibi/work/cargo-build/native-theme-sources/`. `AAV` is `microsoft-ui-xaml@2.5.1
src/controls/dev/AnimatedIcon/AnimatedVisuals/AnimatedAcceptVisualSource.cpp`, and `PF` is `docs/platform-facts.md`.

### The forms tried, and why each failed

1. **Drawn in every state at hidden values** (check 17): `Glyph.draw_extent` 0 (or `draw_start`) while unchecked,
   the dot at a zero size or scale, the draw-on and growth plain transitions of those fields.
   - `draw_extent` is defined for a path only (S2.9). It has no reading on a font glyph (WinUI's fallback code
     point), on Breeze's filled-rectangle dots (`breeze@6.8.2 kstyle/breezehelper.cpp:948-952`, drawn in steps at
     ⅓, ⅔ and 1, `:964-974`, with a transparent brush during the run, `:935-936`), or on Kirigami's `ThemeIcon`
     check (K-KA-46†).
   - WinUI's draw-on was stated twice: as a `Glyph` change, and as the `Keyed` AnimatedIcon transition of the
     `Icon` group, which a `glyph` part lacks (S2.7).
   - WinUI's draw-off is `TrimStart` 0→1, a rising `draw_start` (`AAV:470`, `:552`), not a falling extent.
   - An unscoped `Glyph` transition draws the check off on uncheck. Breeze draws nothing toward unchecked
     (`breezehelper.cpp:957-959`), and Compose snaps after `SnapAnimationDelay` (`Checkbox.kt:604`, `:1061`).
   - The dot's growth took two forms (Breeze's `transform`, Compose's size), though both animate the radius.
2. **`Absent` at rest, written last** (checks 18 and 19): the mark `Absent @ !{checked,indeterminate}`, the dot
   `Absent @ !{checked}` (`adwaita` `!{checked,indeterminate}`, `windows-11` `!{checked,pressed}`), each written after
   every other rule on the part, since a later `Style` patch brings an absent part back (S6.3).
   - It holds inside a preset only. Every layer's bases apply, then every layer's rules (S8, `spec.md:2213-2214`), so
     an application or reader layer's ordinary rule (`checkbox.mark.glyph.fill @ {hover}`, `checkbox.mark.* @ cond
     scheme:dark`) draws the base check on every unchecked box. S12 refuses a `Fields` patch over an `Absent`
     block but accepts a part's `Style` patch, so two forms of one rule behave differently (`spec.md:2912-2913`).
   - Options the checks named: (a) an S12 check refusing, in any layer, a rule on a part whose rest-look `Absent`
     came earlier where its states can match; (b) a rest-look `Absent` that a later `Style` patch cannot revive
     unless its states exclude the absent rule's (a new S6.3 rule, against ADD-A123's); (c) document only; and the
     robust form, present at rest and drawn to zero.
   - The rows that would revive the part inside the presets if the order were not kept: W1-109, W2-12, W2-13; MAC-C07's
     `cond window_inactive`, MAC-L17†, MAC-D01; CK-1, CK-2†, L-12.

### The rest looks, as verified (check 19a's walk)

| Preset | Check mark | Radio dot |
|---|---|---|
| `adwaita` | none unchecked; the check and dash icon sources (`libadwaita@1.10.0 src/stylesheet/widgets/_checks.scss:70`, `:80`), swapped with no transition (`_common.scss:6-7`) | none unchecked; the bullet (`:76`); the inconsistent radio's dash (`:79-81`, `gtk@4.24.1 gtk/gtkcheckbutton.c:871-874`) |
| `kde-breeze` | `CheckOff` draws nothing (`breezehelper.cpp:942-943`); `CheckOn` the path; `CheckPartial` three rectangles | `default: break` while off (`:1075-1076`); `RadioOn` an ellipse |
| `windows-11` | the accept visual's Off frames draw nothing; checked, its stroked path | `CheckGlyph` at opacity 0 unchecked (`RadioButton_themeresources.xaml:374`), shown by `Checked` (`:350`); an unchecked radio's `PressedCheckGlyph` while pressed (`:298-306`, `:376`), its border not drawn (a `BorderBrush`, no `BorderThickness`, `:376`); a checked radio's pressed dot set by two groups (`:298-300`, `:353`), to measure |
| `material-3` | none unchecked: the drawn fraction 0 (`Checkbox.kt:597-612`), a zero-length segment Android does not draw (`frameworks-base@android-17.0.0_r1 graphics/java/android/graphics/PathMeasure.java:134-136`) | none unchecked: the radius 0 (`RadioButton.kt:91`), drawn only above 0 (`:188-190`) |
| `macos-*`, `ios-*` | none unchecked, as measured | none unchecked (macOS) |

### The motion sources (for the engines)

- Breeze: the check by a dash pattern `path.length() * animation` toward checked, nothing toward unchecked
  (`breezehelper.cpp:954-962`, `kstyle/breezestyle.cpp:4923-4929`); the dot `radius * animation` both ways
  (`breezehelper.cpp:1070-1072`, `breezestyle.cpp:4961-4966`, `kstyle/animations/breezewidgetstatedata.cpp:24`).
- WinUI: the accept visual, `TrimEnd` on and `TrimStart` off on its stroked path (`AAV:437`, `:470`, `:522-553`),
  drawn while it trims off in the unchecked foregrounds (`CheckBox_themeresources.xaml:65-68`, `:153-156`,
  `:241-244`, set at `:319-320`, `:344-345`, `:369-370`); the pressed unchecked dot growing from its template
  dimensions by `ControlFastAnimationDuration` (`RadioButton_themeresources.xaml:298-306`, `:376`).
- Compose: the drawn fraction, `DefaultSpatial` from Off and a snap after `SnapAnimationDelay` to Off
  (`Checkbox.kt:595-612`, `:1061`); the mark's colour animated in every state, disabled included (`:634`,
  `:1040-1049`); the check moved to the dash by `lerp` (`:615-631`, `:720-724`); the dot's radius by `FastSpatial`
  (`RadioButton.kt:137-142`) and its colour snapped at disabled (`:144-152`).
- GTK: the icon source swapped, no transition.
- AppKit and UIKit: unmeasured (MAC-X11's recording; CK-2†); register rows.

### The mark's colour field

- WinUI's and Compose's marks are stroked open paths: the accept visual sets `StrokeBrush(_themeColor_Foreground)`
  and no fill (`AAV:564`, `:581`), Compose draws `drawPath(…, style = stroke)` (`Checkbox.kt:735`). S2.9 gives a
  stroked path's colour to `GlyphStroke.paint`, with `fill` absent on a stroked-only path (`spec.md:1079`, `:1985`),
  as the spinner does. The rows state the colour on `glyph.fill` (W1-037; MAT-C5, MAT-C6, MAT-C7), which on the
  open polyline "M L L" fills a wedge. Breeze's three dots are filled; its check is stroked. ADD-A126 states each
  mark's colour on the field its form takes.

### WinUI's path data (in the source, not a measurement)

- The check is `Geometry_1`, an open polyline through (−15.172, 0.016), (−5, 10.188), (15.337, −10.337)
  (`AAV:372-385`); the pressed-on tweak, `Geometry_2`, ends at (15.337, −10.281) (`AAV:392-404`); the dash is
  `Geometry_0`, from (−11.75, −0.125) to (11.875, −0.125) (`AAV:358-370`).
- Each is placed at scale 0.7, offset (24, 23) or (24, 24) (`AAV:563`, `:580`), inside a 48×48 visual (`AAV:846`),
  stroked at `StrokeThickness(4.0F)` with round caps and join (`AAV:564-569`, `:581-587`). `AnimatedIcon` scales the
  visual uniformly to its arranged size (`AnimatedIcon.cpp:153-181`).
- The template's code point (`CheckBoxCheckedGlyph`, E73E) is only the `FallbackIconSource`
  (`CheckBox_themeresources.xaml:604-608`), shown where `InsertVisual` fails (`AnimatedIcon.cpp:473-479`,
  `:537-558`). ADD-A125 states the mark as the stroked path (W1-038, W1-039), its path data the value phase's.
- The facts still say "a font glyph, no stroke": `PF:1242` (`check_mark_stroke_width`, Windows column) and
  `docs/facts/imported/windows.toml:2687-2694` (`none_cell`). They are the Windows lane's to correct.

### macOS: AppKit's rendering and the measured parts

- ADD-A125 tried the rendering on `checkbox.box.glyph` and `radio.ring.glyph` (`replaces: [box, mark]`, `[ring,
  dot]`), the parts drawn in every state, so an unchecked control is rendered too (check 19b's serious 2: with the
  rendering on the mark and the mark `Absent` unchecked, the unchecked control is never rendered). It made the mark
  and dot `Absent` in every state on `macos-*` and amended R11. Removed: it stands only with the absent marks.
- Check 20's findings against that form:
  - the dot is measured and can be modelled without tracing: its diameter (`PF:1241`, `radio_dot_diameter`:
    macOS 14–15 6, macOS 26 5, measured) and its colour (`PF:1232`, `indicator_color` white on macOS 14–15, macOS 26 unsourced), shipped today
    (`native-theme/src/presets/macos-tahoe.toml:172-174`, `:585-587`), as a cornered fill (`Full`), as Windows and
    Material model theirs; R11's rejection (a traced outline) covers the mark alone;
  - the mark cannot be modelled without tracing or substituting artwork (R11: AppKit names no image for it), so
    without the rendering (off the main thread, off macOS, a toolkit that cannot composite) a checked and a mixed
    checkbox differ by the box's fill alone: the model's limit, to record where it is (R23); MAC-C01's gap row asked
    for the dash's size and colour by a new render (`todo_v0.7.1_model-completeness-gap-map-macos.md:145`);
  - the options check 20b named: (a) the dot a modelled fill on `macos-*`, the rendering's `replaces` keeping it;
    (b) (a) plus the mixed dash as a measured fill; (c) the absent form with the limit stated.
- `SystemControl` glyphs: ADD-A125 made `Glyph.size` and `fill` may-be-absent on a `SystemControl` source (S6.3).

### The menu item's check glyph

- `menu.check_indicator` is drawn in every state (its column), and its `glyph` is stated only `@ {checked}` /
  `{indeterminate}` (MAC-MN05, W1-050, MN-6, CB-3, GN-1.73, K1.69), so an unchecked item resolves the base glyph.
  ADD-A125 tried `check_indicator.glyph` `Absent` outside the checked states; check 20 found the combo-box half
  undecided (it needs `@ !{checked,indeterminate} not_context combo_box` and `@ !{selected} context combo_box`), no
  trace stating it, macOS's and KDE's selected glyph in the pop-up unstated (K1.245), and the "a later `Style`
  patch" reason wrong for a field (a later `Set` replaces it; a `Fields` patch conflicts). It goes with the marks'
  representation.

### Kept in ADD-A125

- WinUI's accept visual as the mark's source (W1-038, W1-039), its code point WinUI's own fallback, not modelled.
- `adwaita`'s inconsistent radio: `indeterminate` among `radio`'s flags, GN-1.58's dash, the radio rows'
  `!{checked,indeterminate}`; WinUI's radio `[indeterminate, checked]` group (its empty `Indeterminate` state).
- The rendering's cache key without a background style, an open item (S15; MAC-L17†).
