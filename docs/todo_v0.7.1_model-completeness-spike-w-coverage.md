# Spike W — does microsoft-ui-xaml state `windows-11`'s slice keys? (coverage report)

Pin: `microsoft-ui-xaml@2.5.1` = tag `winui3/release/2.5.1`, commit `ba3a8d59e6de4e2b86ca4f61c78bc26ccf129afd`
(source cache `/run/media/tibi/work/cargo-build/native-theme-sources/microsoft-ui-xaml@2.5.1/`). Read 2026-10-30.
Paths below are relative to its `src/`. Abbreviations in chains: `X_tr.xaml` = `controls/dev/CommonStyles/X_themeresources.xaml`
(or `controls/dev/<Dir>/X_themeresources.xaml` where a `Dir/` prefix is shown); `Cany.xaml` =
`controls/dev/CommonStyles/Common_themeresources_any.xaml`; `generic.xaml` = `dxaml/xcp/dxaml/themes/generic.xaml`.
Themes: `L` = the `Light` dictionary, `D` = the `Default` dictionary (WinUI's dark), `HC` = `HighContrast`.

**Verdict (question 1): yes, with a measured remainder.** The XAML states, directly or through a followable resource
chain, 897 of the 1,018 keys counted here (88 %); 51 of those end at the OS accent colour (`SystemAccentColor*`, a
reader input, not a literal). 121 keys are not stated by the XAML: they live in C++ (focus thickness, repeat timing,
theme-animation curves, placement, tooltip delays), in compiled AnimatedVisuals, in OS readers (accent ramp, caret,
behaviour, high-contrast colours) or in rendered metrics (line height, focus-ring corners, the Acrylic result). Every
not-stated key has a route below; the WinUI host closes the measured ones.

## 1. Which XAML is authoritative at 2.5.1 (perf2026, v2.5, framework generic.xaml)

- **`*_themeresources_perf2026.xaml` / `generic_perf2026.xaml` are opt-in, off by default.** `controls/dev/dll/XamlControlsResources.cpp:70-91`
  loads `themeresources_perf2026.xaml` only when `AreDefaultStyleOptimizationsEnabled()` (`:28-39`), i.e.
  `XamlOptionalChanges::IsChangeEnabled(XamlChangeId::DefaultStyleOptimizations)`; the framework side is
  `dxaml/xcp/dxaml/lib/DefaultStyles.cpp:540-553` (`Styles_perf2026.xbf`, `ThemeResources_perf2026.xbf` at `:369`, `:474`).
  The bit starts cleared: `dxaml/xcp/components/runtimeEnabledFeatures/OptionalChangeState.cpp:9`
  (`g_enabledChanges = 0`), set only by an app calling `XamlOptionalChanges.EnableChange` before `Application.Start`
  (`dxaml/xcp/tools/XCPTypesAutoGen/XamlOM/Model/Microsoft.UI.Xaml.Settings.cs:21-31`). Nothing in
  `WindowsAppSDK@2.5.1` enables it (grep: no hit). **Authoritative: the plain `*_themeresources.xaml`.**
- **The two sets carry the same values for the button.** `Button_themeresources.xaml` and its `_perf2026` twin are
  identical in lines 1-160 (every resource definition; `diff` empty), and differ only in the visual states' form
  (`Storyboard`/`ObjectAnimationUsingKeyFrames` with `KeyTime="0"` vs `VisualState.Setters`). The keyed resources of
  ToggleButton, CheckBox, Slider, TextBox, ToggleSwitch, MenuFlyout and RepeatButton are also identical across the sets.
  The shipped package (parent's cross-check: `Microsoft.WindowsAppSDK.WinUI` 2.3.9's merged
  `lib/native/Microsoft.UI/Themes/generic.xaml`) carries the Storyboard form, i.e. the plain set; e.g. it has
  `ButtonPadding` 11,5,11,6 at its line 8463 and `ControlCornerRadius` 4,4,4,4 at 2345, as the pin.
- **`TextBlock_themeresources.xaml` vs `_v2.5.xaml`:** `CommonStyles.vcxitems:81-88` tags the first
  `UseVisualStyle Latest`, the second `V2dot5`; both are merged into the shipped dictionary (the shipped file holds the
  Latest ramp at its lines 27552-27558 and the v2.5 `BodyStrongTextBlockFontSize` after it). Cite the Latest file;
  which visual style an app gets is selected at build time (property not found in the pin's targets; to confirm on the
  host by reading `CaptionTextBlockFontSize` at run time).
- **The framework's own `generic.xaml` is not the source for Fluent control colours.** It holds the pre-Fluent
  defaults (`ButtonBackground` → `SystemControlBackgroundBaseLowBrush`, generic.xaml:352; `ButtonPadding` 8,4,8,5 at
  :5923; `ButtonBorderThemeThickness` 2 at :122). `XamlControlsResources` (in every WinUI 3 app template) overrides
  them with `controls/dev/CommonStyles`. Cite `controls/dev`; generic.xaml only for what `controls/dev` does not
  define: `ContentControlThemeFontFamily` = `XamlAutoFontFamily` (:14), `ControlContentThemeFontSize` 14 (:36),
  `SymbolThemeFontFamily` "Segoe Fluent Icons,Segoe MDL2 Assets" (:20), and the `SystemControl*` brushes the
  HighContrast dictionaries reference.

## 2. Classification rules

- **stated**: the key's own resource (or template/style setter) is a literal at file:line.
- **followable**: one or more `StaticResource`/`ThemeResource` hops end at a literal; the chain is given. Where the
  last hop is `SystemAccentColor`, `SystemAccentColorLight1-3`/`Dark1-3`, the value is an expression over the OS accent
  (the brush's step and opacity are stated, the colour is read: `dxaml/xcp/components/theming/FrameworkTheming.cpp:169,243`
  → `theminginterop/SystemThemingInterop.cpp:282-283`, `UISettings` `UIColorType_AccentLight2` etc.); counted as
  followable and tallied separately ("→read").
- **not stated**: no XAML resource or literal; the route is named.
- Classification uses L and D. Every HighContrast chain ends at a `SystemColor*Color` (Window, WindowText, ButtonFace,
  ButtonText, Highlight, HighlightText, Hotlight, GrayText) that the XAML never defines (the D/L dictionaries even hold
  the placeholder `#FF00FF`, Cany.xaml:178-185, 406-413): HC values are a reader's (W3-35), on every key.
- Unit of count: one key = one catalogue `part.field` under one state/variant (for colours, one WinUI brush resource
  per state — the WinUI resource naming is exactly `<Control><Part><Property><State>`). Resource keys are enumerated
  from each control's `Default` dictionary and its theme-independent resources (styles, templates and the deprecated
  `*ThemeBrush` keys excluded); the hand-listed rows are the catalogue keys no resource carries, found by reading the
  templates. The full per-resource chains (L, D, HC with file:line per hop) are in the appendix.

## 3. `button` (windows-11)

Catalogue (`spec-widgets.md` `### button`): sizes none, densities none on `windows-11`; exclusive group
`[disabled, pressed, hover]`; variants on windows-11 the typed `primary` (AccentButtonStyle), `flat`
(SubtleButtonStyle), `toggle` (ToggleButton, with checked/indeterminate), `menu` (DropDownButton), `tool`
(AppBarButton / AppBarToggleButton), content `icon_only`/`with_icon`, keyed `@repeat` (RepeatButton) and the nested
identities.

### 3.1 State × variant colour keys (from resources; every chain in the appendix)

| Variant (resource set) | rest / hover / pressed / disabled: `body.fill`, `label.text.fill` (= `icon.icon.fill`), `body.stroke` | class |
|---|---|---|
| base (`Button_tr.xaml:30-41` D, `:128-139` L) | `ButtonBackground*` → `ControlFillColor{Default,Secondary,Tertiary,Disabled}Brush` → D `#0FFFFFFF`/`#15FFFFFF`/`#08FFFFFF`/`#0BFFFFFF` (Cany.xaml:15,16,17,19), L `#B3FFFFFF`/`#80F9F9F9`/`#4DF9F9F9`/`#4DF9F9F9` (Cany.xaml:219,220,221,223); `ButtonForeground*` → `TextFillColor{Primary,Primary,Secondary,Disabled}`; `ButtonBorderBrush{,PointerOver}` → `ControlElevationBorderBrush` (gradient, below), `{Pressed,Disabled}` → `ControlStrokeColorDefaultBrush` (D `#12FFFFFF` Cany.xaml:39, L `#0F000000` :243) | 12 followable |
| `primary` (`Button_tr.xaml:5-16`, `:103-114`) | `AccentButtonBackground*` → `AccentFillColor{Default,Secondary,Tertiary}Brush` = D `SystemAccentColorLight2` at opacity 1 / 0.9 / 0.8 (Cany.xaml:125-127), L `SystemAccentColorDark1` 1 / 0.9 / 0.8 (:329-331); disabled `AccentFillColorDisabled` D `#28FFFFFF` (:38), L `#37000000` (:242); foreground `TextOnAccentFillColor{Primary,Primary,Secondary,Disabled}`; border `AccentControlElevationBorderBrush` (rest, hover), `ControlFillColorTransparentBrush` (pressed, disabled) | 9 followable + 3 →read |
| `flat` (`Button_tr.xaml:17-28`, `:115-126`) | `SubtleFillColor{Transparent,Secondary,Tertiary,Transparent}` (D :25-27, L :229-231), stroke the same brushes | 12 followable |
| `toggle` (`ToggleButton_tr.xaml:7-42` D, `:123-158` L) | unchecked ×4, checked ×4, indeterminate ×4, for fill / foreground / stroke = 36; checked rest/hover/pressed fill → accent (→read) | 33 followable + 3 →read (+ `ToggleButtonBorderThemeThickness` 1, `ToggleButtonCheckedStateBackgroundSizing` OuterBorderEdge, stated) |
| `menu` (`DropDownButton/DropDownButton_tr.xaml:5-7`, `:10-12`) | body as base; `menu_indicator.glyph.fill` rest/hover/pressed `DropDownButtonForegroundSecondary*` → `TextFillColorSecondary`/`Tertiary`; disabled: no own resource (inherits the button's Foreground) | 3 followable |
| `tool` (`AppBarButton_tr.xaml`) | 25 colour keys (fill/fg/stroke ×4, accelerator text ×4, sub-menu-opened ×4, sub-item chevron ×5), 18 geometry keys stated (`AppBarButtonContentViewboxMargin` 12,16,0,10 :110, `AppBarButtonTextLabelMargin` 2,0,2,8 :115, `AppBarButtonTextLabelOnRightMargin` 8,16,12,10 :116, `AppBarButtonInnerBorderMargin` 2,6,2,6 :117, glyph `&#xE974;` :101, …) | 25 followable, 18 stated |
| `tool`+`toggle` (`AppBarToggleButton_tr.xaml`) | 58 keys incl. checked ×4 and the overflow check glyph | 48 followable + 3 →read, 7 stated |
| `@repeat` (`RepeatButton_tr.xaml`) | fill / fg / stroke ×4 + thickness | 12 followable, 1 stated |

Resource keys in the six sets: 193 = 30 stated + 154 followable + 9 followable→read.

The gradients: `ControlElevationBorderBrush` D (Cany.xaml:186-191): `MappingMode="Absolute" StartPoint="0,0" EndPoint="0,3"`, stops 0.33 `ControlStrokeColorSecondary` (D `#18FFFFFF`, :40), 1.0 `ControlStrokeColorDefault` (D `#12FFFFFF`, :39); L (:382-390) the same plus `ScaleTransform ScaleY="-1" CenterY="0.5"` (flipped), stops L `#29000000` (:244), `#0F000000` (:243). `AccentControlElevationBorderBrush` (D :198-206, L :397-405) is flipped in both themes, stops `ControlStrokeColorOnAccentSecondary` (D `#23000000` :42, L `#66000000` :246) and `…OnAccentDefault` (`#14FFFFFF`, :41/:245). How `RelativeTransform` with `CenterY="0.5"` maps an `Absolute` 3 px gradient on a 32 px tall box is the brush's runtime behaviour: the paint is followable, its rendered placement is the host's to confirm (one capture).

### 3.2 The button keys no resource carries (hand-listed from the templates)

| # | Key | Class | Evidence / route |
|---|---|---|---|
| 1 | `body.corners` | followable | `CornerRadius="{ThemeResource ControlCornerRadius}"` Button_tr.xaml:168 → 4,4,4,4 (`CornerRadius_themeresources.xaml:5,9,13`) |
| 2-4 | `body.min_width`, `body.min_height`, `body.max_width` | stated (none: auto) | `DefaultButtonStyle` sets no Min/Max size (Button_tr.xaml:154-168). The rendered height is padding 5+6 + border 1+1 + the label's line height, which is not stated (row 19) |
| 5-8 | `body.margin`, `body.shadow`, `body.opacity`, `body.transform` | stated (absent) | the template (Button_tr.xaml:171-231) has no margin, `Shadow`/`Translation`, opacity change or transform in any state. W3-32's press tilt (`PointerDownThemeAnimation`) is not in this template; host capture confirms nothing else applies it |
| 9 | `body.focus.offset` | stated | `FocusVisualMargin` -3, Button_tr.xaml:167 (Subtle :310) |
| 10 | `body.focus` widths (2 outer, 1 inner) | not stated in XAML | C++ defaults `DefaultFocusVisualPrimaryThickness = {2,2,2,2}`, `…Secondary… = {1,1,1,1}` (`dxaml/xcp/components/DependencyObject/DependencyProperty.cpp:24-25`, used :402-408); a pinned-source citation of C++, or measure |
| 11 | `body.focus` colours | followable | `SystemControlFocusVisualPrimaryBrush` → `FocusStrokeColorOuterBrush` (CommonStyles/Common_themeresources.xaml:15/:29 → Cany.xaml:144/:348 → D `#FFFFFF` :54, L `#E4000000` :258); secondary → `FocusStrokeColorInner` D `#B3000000` :55, L `#B3FFFFFF` :259 |
| 12 | `body.focus` corners | not stated | computed by the framework from the control's `CornerRadius` and the margin; measure (host capture with keyboard focus) |
| 13 | `body.cursor` | not stated | no `ProtectedCursor` in XAML; C++ behaviour (W3-29); host reads `ProtectedCursor`/observes |
| 14 | `body.transitions` (Fill) duration | stated | `BrushTransition Duration="0:0:0.083"` Button_tr.xaml:174 (accent :246, subtle :317; ToggleButton_tr.xaml:200) |
| 15 | `body.transitions` curve | not stated | `BrushTransition` has no easing in XAML (composition default); measure |
| 16 | `label.text` family | not stated (alias) | `ContentControlThemeFontFamily` (Button_tr.xaml:163) → `XamlAutoFontFamily` (generic.xaml:14), an alias resolved in C++ (`dxaml/xcp/core/inc/FontAndScriptServices.h:30`); the face and its optical size: Learn *Typography* or host |
| 17 | `label.text` size | followable | `ControlContentThemeFontSize` (Button_tr.xaml:165) → 14 (generic.xaml:36) |
| 18 | `label.text` weight | stated | `Normal`, Button_tr.xaml:164 |
| 19 | `label.text` line height | not stated | font metrics (no `LineHeight`); Learn *Typography* ramp or host measurement |
| 20-21 | `label.padding`, `label.transform` | stated (absent) | ContentPresenter carries only the control's Padding (Button_tr.xaml:172) |
| 22 | `icon.icon` size | not stated | a Button's icon is app content; host/Learn |
| 23 | `icon.icon.fill` | followable | inherits `Foreground` (rows of 3.1) |
| 24 | `icon.transitions` | not stated | `AnimatedIcon.State` PointerOver/Pressed/Normal set at Button_tr.xaml:192, 208, 225, but the frames and timing are compiled AnimatedVisuals (C++), not XAML; measure |
| 25 | `menu_indicator.glyph` | stated | `AnimatedChevronDownSmallVisualSource`, fallback `FontIconSource FontSize="8"` `Glyph="&#xE96E;"` in `SymbolThemeFontFamily` (`DropDownButton/DropDownButton.xaml:93-96`) |
| 26 | `menu_indicator.min_width` | stated | `Width="12" Height="12"` DropDownButton.xaml:93 |
| 27 | `menu_indicator.transitions` | not stated | AnimatedIcon frames (as 24) |
| 28 | `values.menu_indicator_gap` | stated | `Margin="8,0,0,0"` DropDownButton.xaml:93 |
| 29 | `values.icon_text_gap` | not stated | Button has no icon+label layout (content); `tool`'s gaps are the AppBarButton margins above; host/Learn for a plain button |
| 30 | `values.menu_placement` | not stated | the Flyout's placement is C++ (no `Placement` in DropDownButton's XAML or .cpp); **the WinUI host measures it** |
| 31 | `values.label_position` | stated | AppBarButton's `LabelOnRight`/`LabelCollapsed`/`Overflow` visual states (AppBarButton_tr.xaml) |
| 32-33 | `values.repeat_delay`, `repeat_interval` | not stated in XAML | **C++ defaults Delay 500, Interval 33** (`DependencyProperty.cpp:714-720`); no XAML setter in CommonStyles. This **contradicts W2-19's "250 for each" (Learn)**; the host can read `new RepeatButton().Delay/Interval` to settle it |
| 34 | `values.focus_visible_when` | not stated | C++ focus logic / Learn |
| 35-38 | `primary`: `body.padding`, stroke width, label font (size/family/weight), focus offset | not stated (ambiguous) | `AccentButtonStyle` (Button_tr.xaml:235-240) is not `BasedOn` `DefaultButtonStyle` and sets none of them; they come from the framework's default style layer (generic.xaml:5984-5997), whose `{StaticResource ButtonPadding}` / `{ThemeResource ButtonBorderThemeThickness}` could resolve to generic.xaml's 8,4,8,5 (:5923) / 2 (:122) or to XamlControlsResources' 11,5,11,6 / 1 depending on lookup scope. The XAML does not settle which; **measure the accent button's frame on the host** |
| 39-40 | stroke gradient geometry, base and primary | followable | 3.1 above |

Hand-listed: 40 = 16 stated + 6 followable + 18 not stated.

Also noted: `AccentButtonForegroundDisabled` refers to `TextOnAccentFillColorDisabled` (Button_tr.xaml:12, :110), a **`Color`** (Cany.xaml:14 `#87FFFFFF` D, :218 `#FFFFFF` L), not the `…Brush`; the value is followable, but whether the framework converts a Color resource to the Foreground brush (or the disabled accent label falls back) is runtime behaviour: one host capture of a disabled accent button settles it.

Nested identities (`close`, `back`, `previous`, `next`, `overflow`, `add`, `remove`, `clear_field`, `reveal`, `pane_toggle`, media, caption, `route`, and keyed `@query`, `@accept`, `@dismiss`, `@first`, `@last`, `@page_number`, `@zoom_out`) are rules in their host's context; their resource sets live with the hosts (spot-checked present: `TabViewCloseButtonStyle`, `TabViewButtonStyle` TabView/TabView.xaml; `PaneToggleButtonStyle` NavigationView/TitleBar `_themeresources`; `NavigationBackButtonNormalStyle` NavigationView/NavigationBackButton.xaml; `DeleteButtonStyle` TextBox_tr.xaml/AutoSuggestBox; `RevealButtonStyle` PasswordBox_tr.xaml; `PagerControlNumberPanelButtonStyle` PagerControl; `DateTimePickerFlyoutButtonStyle` DatePicker_tr.xaml; `InfoBarCloseButtonStyle` InfoBar; `WindowCaptionButton`, `ZoomOutButton` generic.xaml; `CastButton` MediaTransportControls_tr.xaml). They are not counted here: each belongs to its host widget's lane, which reads the same kind of chains.

## 4. Colour roles (S3) and the slice's tokens (S4.1, S4.2, S4.4, S4.5, S4.6, focus)

### 4.1 Roles (windows-11's sets: `Window`, `Selection`; global; keyed tiers)

| Role | WinUI resource (the trace's mapping) | Class |
|---|---|---|
| `window.background` | `ApplicationPageBackgroundThemeBrush` → `SolidBackgroundFillColorBase` (Common_themeresources.xaml:13/:27 → Cany.xaml:68 D `#202020`, :272 L `#F3F3F3`) | followable |
| `window.background_alternate`, `foreground`, `foreground_inactive`, `negative`, `neutral`, `positive`, `decoration_focus`, `border`, `disabled.foreground` | `SolidBackgroundFillColorSecondary`, `TextFillColorPrimary`, `TextFillColorTertiary`, `SystemFillColorCritical`, `…Caution`, `…Success`, `FocusStrokeColorOuter`, `ControlStrokeColorDefault`, `TextFillColorDisabled` (Cany.xaml D :69, :5, :7, :78, :77, :76, :54, :39, :8; L :273, :209, :211, :282, :281, :280, :258, :243, :212) | 9 stated |
| `window.link` | `AccentTextFillColorPrimaryBrush` → D `SystemAccentColorLight3` (Cany.xaml:93), L `SystemAccentColorDark2` (:297) | followable→read |
| `selection.background` / `selection.foreground` | `AccentFillColorSelectedTextBackgroundBrush` → `SystemAccentColor` (:124/:328) / `TextOnAccentFillColorSelectedText` `#FFFFFF` (:11/:215) | →read / stated |
| `global.accent` | `SystemAccentColor`: not in XAML | not stated (reader `UISettings`) |
| `global.accent_foreground` | `TextOnAccentFillColorPrimary` D `#000000` :12, L `#FFFFFF` :216 | stated |
| `global.accent_standalone` | `AccentTextFillColorPrimaryBrush` (as link) | →read |
| `global.accent_ramp` ×6 | `SystemAccentColorLight1-3`, `Dark1-3`: not in XAML | 6 not stated (reader) |
| `global.status.*.fill` ×5 | `SystemFillColorSuccess/Caution/Critical/Neutral` (D :76-79, L :280-283) stated; `SystemFillColorAttentionBrush` → D `SystemAccentColorLight2` :165, L `SystemAccentColor` :369 | 4 stated, 1 →read |
| `global.status.*.background` ×5 | `SystemFillColor{Attention,Success,Caution,Critical,Neutral}Background` (D :81-85, L :285-289) | 5 stated |
| `global.status.*.background_solid` ×5 | `SystemFillColorSolidAttentionBackground`, `…SolidNeutralBackground` (D :86-87, L :290-291) stated; success/caution/critical have no solid resource | 2 stated, 3 not stated (none in WinUI: the preset states them absent, a fact from absence to confirm on Learn) |
| `global.selection_text_background` | `TextControlSelectionHighlightColor` → `AccentFillColorSelectedTextBackgroundBrush` (TextBox_tr.xaml:39/:146) | →read |
| `global.selection_text_foreground` | none (W3-25: WinUI keeps the text colour) | not stated (host capture) |
| `global.selection_text_background_unfocused` | `SelectionHighlightColorWhenNotFocused` has no XAML default | not stated (host reads the property) |
| `global.scrim` | `SmokeFillColorDefault` `#4D000000` (Cany.xaml:59, :263) | stated |
| `global.drop_highlight` | no resource | not stated (host) |
| keyed tiers `role.@…` | the 83 `Color` literals per theme (Cany.xaml:5-87 D, :209-291 L) | 83 stated |
| keyed accent tiers | the 8 brushes over `SystemAccentColor*` (Cany.xaml:93-95, 124-127, 165 D; 297-299, 328-331, 369 L) | 8 →read |
| keyed gradients | `ControlElevationBorderBrush`, `CircleElevationBorderBrush`, `AccentControlElevationBorderBrush` (:186-206, :382-405) | 3 followable |
| keyed high-contrast system colours ×8 (W3-35) | `SystemColor{Window,WindowText,ButtonFace,ButtonText,Highlight,HighlightText,Hotlight,GrayText}Color`: not in XAML | 8 not stated (reader) |

Roles: 144 = 106 stated + 4 followable + 13 followable→read + 21 not stated.

### 4.2 Tokens

| Token | Class | Evidence / route |
|---|---|---|
| `motion.durations.control_normal`, `control_fast`, `control_fast_after`, `control_faster` | 4 stated | `00:00:00.250`, `.167`, `.168`, `.083` (Cany.xaml:603-606) |
| `motion.timings.control_fast_out_slow_in` | stated | `ControlFastOutSlowInKeySpline` `0,0,0,1` (Cany.xaml:602) |
| `motion.timings.fast_out_slow_in`, `slow_out_fast_in` | 2 not stated | Learn *Timing and easing* |
| `motion.durations.connected`, `timings.connected` | 2 not stated | Learn / measure |
| `motion.speed_factor` | not stated | reader (`SPI_GETCLIENTAREAANIMATION`) |
| theme animations used by the samples (`FadeIn/Out`, `Reposition`, `SplitOpen/Close`, `DragItem/DragOver/DropTargetItem`) | (counted per widget) | parameters from uxtheme animation metrics (`ThemeGenerator::AddTimelinesForThemeAnimation(TAS_FADEIN, TA_FADEIN_SHOWN …)`, `dxaml/xcp/dxaml/lib/ThemeAnimations.cpp:75`, :95); `SplitOpenThemeAnimation::s_OpenDuration` (C++ constant, :681); measure |
| `caret.color`, `width`, `blink_on`, `blink_off`, `blink_timeout` | 5 not stated | readers (W3-24); colour: C++ default |
| `behavior.double_click_time`, `double_click_distance`, `drag_threshold`, `hover_open_delay`, `scroll_lines_per_step`, `key_repeat.delay`, `key_repeat.interval` | 7 not stated | readers (W3-41); the tooltip's use of `SPI_GETMOUSEHOVERTIME` with a 400 ms fallback is C++ (`ToolTipService_Partial.h:18`, `.cpp:1756-1764`) |
| `behavior.focus_visible_when`, `behavior.mnemonics` | 2 not stated | C++ / Learn |
| `focus` primary and secondary colours | 2 followable | as 3.2 row 11 |
| `focus` primary width, secondary width, default margin | 3 not stated in XAML | C++ `DependencyProperty.cpp:24-25, 402-412` |
| `text.keyed.{caption, body, body_strong, body_large, body_large_strong, subtitle, title, title_large, display}` size ×9 | 9 followable | style setter → `…TextBlockFontSize` 12, 14, (Base) 14, 18, 18, 20, 28, 40, 68 (TextBlock_themeresources.xaml:3-9, styles :10-51) |
| … weight ×9 | 3 stated, 6 followable | `Normal` set on caption :21, body :24, body_large :28; the rest inherit `SemiBold` from `BaseTextBlockStyle` :13 |
| … line height ×9 | 9 not stated | `LineStackingStrategy="MaxHeight"` (:16), no `LineHeight`: Learn *Typography in Windows* or measure |
| `text` family | not stated | `XamlAutoFontFamily` (TextBlock_themeresources.xaml:11), alias resolved in C++ |
| `text.rendering` | not stated | reader / WinUI's grayscale AA (W3-22) |
| `icons.set` | stated | `SymbolThemeFontFamily` "Segoe Fluent Icons,Segoe MDL2 Assets" (generic.xaml:20) |
| `icons.sizes.toolbar` | followable | `AppBarButtonContentHeight` 16 (`Materials/Reveal/RevealBrush_themeresources.xaml:563`, used at AppBarButton_tr.xaml:366) |
| `icons.sizes.small`, `large`, `dialog`, `panel`, `icons.mirroring` | 5 not stated | readers (`SM_CXSMICON` …) / Learn |
| `params` (S4.6) | — | windows-11 declares no group the button uses: `opacity` is unused (WinUI disables by colour), and RepeatButton's timing is the widget's own value (W2-19), not `pointer_repeat` |

Tokens: 65 = 9 stated + 18 followable + 38 not stated.

## 5. Ten sample widgets

Catalogue names used: `checkbox`, `switch` (ToggleSwitch), `slider`, `input` (TextBox), `combo_box`, `menu` (MenuFlyout and its items), `tooltip`, `progress_bar`, `tab` (TabViewItem), `banner` (InfoBar). None was absent. Resource rows: appendix.

| Widget (resources) | Resource keys (stated / followable / →read) | Hand-listed keys no resource carries | Class |
|---|---|---|---|
| `checkbox` (CheckBox_tr.xaml) | 81 (9 / 60 / 12): box fill, stroke, glyph fill ×(unchecked, checked, indeterminate)×4 states, `CheckBoxSize`, `CheckBoxGlyphSize`, `CheckBoxHeight`, `CheckBoxMinWidth`, `CheckBoxPadding`, `CheckBoxFocusVisualMargin` (:275), glyphs | `label.text` size (followable, `ControlContentThemeFontSize`); `mark.transitions` (AnimatedIcon frames, 15 refs: compiled), label family, label line height, focus widths (C++), focus corners | 1 followable, 5 not |
| `switch` (ToggleSwitch_tr.xaml) | 39 (6 / 27 / 6) | track 40×20, radius 10 (`OuterBorder` :507); thumb size per state 12 (:510-515), hover 14, pressed 17×14 (storyboards :232-321; `KeyTime` `ControlFasterAnimationDuration`, spline `ControlFastOutSlowInKeySpline`), disabled (:358-367, `ControlNormalAnimationDuration`); knob offset X 20 (:445) — 8 stated; the knob's on/off move is `RepositionThemeAnimation` (:376-425: uxtheme, measure), label line height, focus widths, focus corners — 4 not | 8 stated, 4 not |
| `slider` (Slider_tr.xaml) | 40 (18 / 16 / 6), incl. `SliderTrackThemeHeight`, thumb sizes, `SliderInnerThumbWidth/Height`, corner radii | `thumb_inner.transform` per state (no storyboard durations in the template; the inner thumb's hover/press scale: measure), value-fill transition, header line height, focus widths, focus corners | 5 not |
| `input` (TextBox_tr.xaml) | 31 (4 / 26 / 1), incl. selection highlight (→read), button colours | `body.padding` `TextControlThemePadding` 10,5,6,6 and border `TextControlBorderThemeThickness` 1 / focused 1,1,1,2 (CommonStyles/Common_themeresources.xaml:10-12) — 3 stated; caret colour/width (C++/reader), text line height — 3 not | 3 stated, 3 not |
| `combo_box` (ComboBox/ComboBox_tr.xaml) | 97 (30 / 66 / 1) | popup open/close (`SplitOpen/CloseThemeAnimation`: C++ `s_OpenDuration`), popup placement over the selected item (C++; host), arrow AnimatedIcon frames, label line height, focus widths, focus corners | 6 not |
| `menu` (MenuFlyout_tr.xaml) | 84 (19 / 65 / 0): presenter and item colours ×states, item padding, icon, accelerator text, sub-item chevron | box corners `OverlayCornerRadius` 8 (CornerRadius_themeresources.xaml:6) — followable; open animation (C++ popup transition), placement (C++; **the host's MenuFlyout frame**), submenu delay (C++), the presenter's shadow (C++ `ThemeShadow`), item line height — 5 not | 1 followable, 5 not |
| `tooltip` (ToolTip_tr.xaml) | 9 (4 / 5 / 0): font 12 (:5/:29), border 1, padding 9,6,9,8 (:76), max width 320 (:77), background `ToolTipBackgroundBrush` → `AcrylicInAppFillColorDefaultBrush` (Tint `#FCFCFC` 0.0, Fallback `#F9F9F9`, TintLuminosityOpacity 0.85, `Materials/Acrylic/AcrylicBrush_themeresources.xaml:96`; restated in C++ `XamlControlsResources.cpp:121-123`) | corners `ControlCornerRadius` (:52) followable; show delay (C++ over `SPI_GETMOUSEHOVERTIME`), reshow window 200 ms (C++ `ToolTipService_Partial.h:17`), fade in/out (`FadeIn/OutThemeAnimation` :61, :66 → uxtheme), placement (C++; **the host's ToolTip frame**), shadow (C++), the Acrylic's rendered result, line height — 7 not | 1 followable, 7 not |
| `progress_bar` (ProgressBar/ProgressBar_tr.xaml) | 10 (5 / 4 / 1): min height 3 (:29), track height 1 (:30), radii 1.5 / 0.5 (:31-32), paused `SystemFillColorCaution`, error `…Critical`, value fill → accent | indeterminate loop keyframes (ProgressBar/ProgressBar.xaml, 26 `KeyTime`/`Duration` literals) — stated; the determinate fill's move (`RepositionThemeAnimation`) — not | 1 stated, 1 not |
| `tab` (TabView/TabView_tr.xaml, `TabViewItem*`, `TabViewButton*`, `TabViewClose*`, `TabViewSelected*`) | 83 (31 / 52 / 0) | add / reorder / drag (`DragItem`/`DragOver`/`DropTargetItem` theme animations in TabView.xaml: uxtheme), tear-out visuals (W-SF09†), label line height, focus widths, focus corners | 5 not |
| `banner` (InfoBar/InfoBar_tr.xaml) | 43 (27 / 14 / 2): severity backgrounds and icon colours, icon glyphs and size, padding, margins | title/message line height, the actions' wrap threshold (C++ InfoBar code), focus | 3 not |

## 6. Counts

| Scope | Keys | Stated | Followable (of which →read) | Not stated |
|---|---|---|---|---|
| `button` (6 resource sets + 40 hand-listed) | 233 | 46 | 169 (9) | 18 |
| Colour roles | 144 | 106 | 17 (13) | 21 |
| Slice tokens | 65 | 9 | 18 (0) | 38 |
| `checkbox` | 87 | 9 | 73 (12) | 5 |
| `switch` | 51 | 14 | 33 (6) | 4 |
| `slider` | 45 | 18 | 22 (6) | 5 |
| `input` | 37 | 7 | 27 (1) | 3 |
| `combo_box` | 103 | 30 | 67 (1) | 6 |
| `menu` | 90 | 19 | 66 (0) | 5 |
| `tooltip` | 17 | 4 | 6 (0) | 7 |
| `progress_bar` | 12 | 6 | 5 (1) | 1 |
| `tab` | 88 | 31 | 52 (0) | 5 |
| `banner` | 46 | 27 | 16 (2) | 3 |
| **Samples (10)** | **576** | **165** | **367 (29)** | **44** |
| **Total** | **1,018** | **326** | **571 (51)** | **121** |

Caveat on the unit: the resource rows count every WinUI brush/size resource of the control as one catalogue key
(its naming is one property under one state); a lane that maps several resources to one key, or a catalogue key to
two resources, will count differently. The hand-listed rows are what reading the templates found beyond the
resources, not an exhaustive walk of each catalogue entry's field list for the ten samples (the button's is
exhaustive for its own, non-nested variants).

## 7. What the facts lanes must know

1. Cite `controls/dev/**/<Control>_themeresources.xaml` and `CommonStyles/Common_themeresources_any.xaml`, never the
   `_perf2026` twins (opt-in, off by default, same values) nor the framework's `dxaml/.../themes/generic.xaml` for
   Fluent colours (pre-Fluent defaults overridden by XamlControlsResources). generic.xaml only for
   `XamlAutoFontFamily`, `ControlContentThemeFontSize` 14, `SymbolThemeFontFamily` and the HC `SystemControl*` brushes.
2. The gap map's citations are at MUX 8463f45; every line moves at the pin (e.g. `ControlElevationBorderBrush` is at
   Cany.xaml:186 D and :382 L at the pin). Re-locate, don't copy.
3. `D` (`Default`) is WinUI's dark dictionary. Light's elevation gradient is flipped by `ScaleY="-1"`, dark's is not;
   the accent gradient is flipped in both.
4. Accent-derived values are expressions over the OS accent (step + opacity stated): state them as such, with the
   accent ramp from the reader; HC values are all reader values.
5. The model's "disabled opacity" does not exist on WinUI: every disabled look is a colour.
6. C++-only facts at the pin: focus thicknesses 2/1 (`DependencyProperty.cpp:24-25`); RepeatButton Delay **500** /
   Interval **33** (`:714-720`, contradicting W2-19's 250/250: settle on the host); tooltip reshow 200 ms and hover
   fallback 400 ms (`ToolTipService_Partial.h:17-18`); theme-animation timings come from uxtheme (measure).
7. Not decidable from XAML, for the host: AccentButtonStyle's padding/border width/font (no `BasedOn`); the disabled
   accent label (a `Color` where a brush is expected); the elevation gradient's rendered placement; focus-ring
   corners; line heights; MenuFlyout/ToolTip/ComboBox placement (S13); AnimatedIcon motion; BrushTransition easing.
8. AppBarButton's toolbar icon size 16 is defined in `Materials/Reveal/RevealBrush_themeresources.xaml:563`, an
   unexpected home: cite it there.

## Appendix — every resource key's chain (L, D, HC; file:line per hop)

Generated from the pinned files by a resolver that follows `StaticResource`/`ThemeResource` per theme dictionary
(first definition in `controls/dev/CommonStyles`, then other `controls/dev` files, then generic.xaml). "(runtime)"
marks a key no XAML defines.

### Button_themeresources.xaml

| Key | Class | L / D | HC |
|---|---|---|---|
| `AccentButtonBackground` | accent | L: SystemAccentColorDark1 (runtime) [Button_tr.xaml:103 > Cany.xaml:329]; D: SystemAccentColorLight2 (runtime) [Button_tr.xaml:5 > Cany.xaml:125] | HC: SystemColorHighlightColor (runtime) [Button_tr.xaml:54 > generic.xaml:2055] |
| `AccentButtonBackgroundPointerOver` | accent | L: SystemAccentColorDark1 (runtime) [Button_tr.xaml:104 > Cany.xaml:330]; D: SystemAccentColorLight2 (runtime) [Button_tr.xaml:6 > Cany.xaml:126] | HC: SystemColorButtonTextColor (runtime) [Button_tr.xaml:55 > generic.xaml:2036] |
| `AccentButtonBackgroundPressed` | accent | L: SystemAccentColorDark1 (runtime) [Button_tr.xaml:105 > Cany.xaml:331]; D: SystemAccentColorLight2 (runtime) [Button_tr.xaml:7 > Cany.xaml:127] | HC: SystemColorButtonFaceColor (runtime) [Button_tr.xaml:56 > generic.xaml:2013] |
| `AccentButtonBackgroundDisabled` | followable | L: #37000000 [Button_tr.xaml:106 > Cany.xaml:332 > Cany.xaml:242]; D: #28FFFFFF [Button_tr.xaml:8 > Cany.xaml:128 > Cany.xaml:38] | HC: SystemColorButtonFaceColor (runtime) [Button_tr.xaml:57 > generic.xaml:2010] |
| `AccentButtonForeground` | followable | L: #FFFFFF [Button_tr.xaml:107 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [Button_tr.xaml:9 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorHighlightTextColor (runtime) [Button_tr.xaml:58 > generic.xaml:2056] |
| `AccentButtonForegroundPointerOver` | followable | L: #FFFFFF [Button_tr.xaml:108 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [Button_tr.xaml:10 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorButtonFaceColor (runtime) [Button_tr.xaml:59 > generic.xaml:2020] |
| `AccentButtonForegroundPressed` | followable | L: #B3FFFFFF [Button_tr.xaml:109 > Cany.xaml:303 > Cany.xaml:217]; D: #80000000 [Button_tr.xaml:11 > Cany.xaml:99 > Cany.xaml:13] | HC: SystemColorHighlightColor (runtime) [Button_tr.xaml:60 > generic.xaml:2069] |
| `AccentButtonForegroundDisabled` | followable | L: #FFFFFF [Button_tr.xaml:110 > Cany.xaml:218]; D: #87FFFFFF [Button_tr.xaml:12 > Cany.xaml:14] | HC: SystemColorGrayTextColor (runtime) [Button_tr.xaml:61 > generic.xaml:2026] |
| `AccentButtonBorderBrush` | followable | L: gradient AccentControlElevationBorderBrush [Button_tr.xaml:111 > Cany.xaml:397]; D: gradient AccentControlElevationBorderBrush [Button_tr.xaml:13 > Cany.xaml:198] | HC: SystemColorHighlightColor (runtime) [Button_tr.xaml:62 > generic.xaml:2055] |
| `AccentButtonBorderBrushPointerOver` | followable | L: gradient AccentControlElevationBorderBrush [Button_tr.xaml:112 > Cany.xaml:397]; D: gradient AccentControlElevationBorderBrush [Button_tr.xaml:14 > Cany.xaml:198] | HC: SystemColorHighlightColor (runtime) [Button_tr.xaml:63 > generic.xaml:2073] |
| `AccentButtonBorderBrushPressed` | followable | L: #00FFFFFF [Button_tr.xaml:113 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [Button_tr.xaml:15 > Cany.xaml:106 > Cany.xaml:20] | HC: SystemColorHighlightColor (runtime) [Button_tr.xaml:64 > generic.xaml:2082] |
| `AccentButtonBorderBrushDisabled` | followable | L: #00FFFFFF [Button_tr.xaml:114 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [Button_tr.xaml:16 > Cany.xaml:106 > Cany.xaml:20] | HC: SystemColorGrayTextColor (runtime) [Button_tr.xaml:65 > generic.xaml:2032] |
| `SubtleButtonBackground` | followable | L: #00FFFFFF [Button_tr.xaml:115 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [Button_tr.xaml:17 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorButtonFaceColor (runtime) [Button_tr.xaml:66 > generic.xaml:2010] |
| `SubtleButtonBackgroundPointerOver` | followable | L: #09000000 [Button_tr.xaml:116 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [Button_tr.xaml:18 > Cany.xaml:112 > Cany.xaml:26] | HC: SystemColorHighlightTextColor (runtime) [Button_tr.xaml:67 > Cany.xaml:514] |
| `SubtleButtonBackgroundPressed` | followable | L: #06000000 [Button_tr.xaml:117 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [Button_tr.xaml:19 > Cany.xaml:113 > Cany.xaml:27] | HC: SystemColorHighlightTextColor (runtime) [Button_tr.xaml:68 > Cany.xaml:514] |
| `SubtleButtonBackgroundDisabled` | followable | L: #00FFFFFF [Button_tr.xaml:118 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [Button_tr.xaml:20 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorButtonFaceColor (runtime) [Button_tr.xaml:69 > generic.xaml:2010] |
| `SubtleButtonForeground` | followable | L: #E4000000 [Button_tr.xaml:119 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [Button_tr.xaml:21 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [Button_tr.xaml:70 > Cany.xaml:512] |
| `SubtleButtonForegroundPointerOver` | followable | L: #E4000000 [Button_tr.xaml:120 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [Button_tr.xaml:22 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightColor (runtime) [Button_tr.xaml:71 > generic.xaml:2069] |
| `SubtleButtonForegroundPressed` | followable | L: #9E000000 [Button_tr.xaml:121 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [Button_tr.xaml:23 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightColor (runtime) [Button_tr.xaml:72 > generic.xaml:2069] |
| `SubtleButtonForegroundDisabled` | followable | L: #5C000000 [Button_tr.xaml:122 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [Button_tr.xaml:24 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [Button_tr.xaml:73 > generic.xaml:2026] |
| `SubtleButtonBorderBrush` | followable | L: #00FFFFFF [Button_tr.xaml:123 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [Button_tr.xaml:25 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorButtonTextColor (runtime) [Button_tr.xaml:74 > generic.xaml:2054] |
| `SubtleButtonBorderBrushPointerOver` | followable | L: #09000000 [Button_tr.xaml:124 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [Button_tr.xaml:26 > Cany.xaml:112 > Cany.xaml:26] | HC: SystemColorHighlightColor (runtime) [Button_tr.xaml:75 > Cany.xaml:513] |
| `SubtleButtonBorderBrushPressed` | followable | L: #06000000 [Button_tr.xaml:125 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [Button_tr.xaml:27 > Cany.xaml:113 > Cany.xaml:27] | HC: SystemColorHighlightTextColor (runtime) [Button_tr.xaml:76 > Cany.xaml:514] |
| `SubtleButtonBorderBrushDisabled` | followable | L: #00FFFFFF [Button_tr.xaml:126 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [Button_tr.xaml:28 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorGrayTextColor (runtime) [Button_tr.xaml:77 > generic.xaml:2032] |
| `ButtonBorderThemeThickness` | stated | L: 1 [Button_tr.xaml:127]; D: 1 [Button_tr.xaml:29] | HC: 1 [Button_tr.xaml:90] |
| `ButtonBackground` | followable | L: #B3FFFFFF [Button_tr.xaml:128 > Cany.xaml:305 > Cany.xaml:219]; D: #0FFFFFFF [Button_tr.xaml:30 > Cany.xaml:101 > Cany.xaml:15] | HC: SystemColorButtonFaceColor (runtime) [Button_tr.xaml:78 > generic.xaml:2010] |
| `ButtonBackgroundPointerOver` | followable | L: #80F9F9F9 [Button_tr.xaml:129 > Cany.xaml:306 > Cany.xaml:220]; D: #15FFFFFF [Button_tr.xaml:31 > Cany.xaml:102 > Cany.xaml:16] | HC: SystemColorHighlightTextColor (runtime) [Button_tr.xaml:79 > Cany.xaml:514] |
| `ButtonBackgroundPressed` | followable | L: #4DF9F9F9 [Button_tr.xaml:130 > Cany.xaml:307 > Cany.xaml:221]; D: #08FFFFFF [Button_tr.xaml:32 > Cany.xaml:103 > Cany.xaml:17] | HC: SystemColorHighlightTextColor (runtime) [Button_tr.xaml:80 > Cany.xaml:514] |
| `ButtonBackgroundDisabled` | followable | L: #4DF9F9F9 [Button_tr.xaml:131 > Cany.xaml:309 > Cany.xaml:223]; D: #0BFFFFFF [Button_tr.xaml:33 > Cany.xaml:105 > Cany.xaml:19] | HC: SystemColorButtonFaceColor (runtime) [Button_tr.xaml:81 > generic.xaml:2010] |
| `ButtonForeground` | followable | L: #E4000000 [Button_tr.xaml:132 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [Button_tr.xaml:34 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [Button_tr.xaml:82 > Cany.xaml:512] |
| `ButtonForegroundPointerOver` | followable | L: #E4000000 [Button_tr.xaml:133 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [Button_tr.xaml:35 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightColor (runtime) [Button_tr.xaml:83 > generic.xaml:2069] |
| `ButtonForegroundPressed` | followable | L: #9E000000 [Button_tr.xaml:134 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [Button_tr.xaml:36 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightColor (runtime) [Button_tr.xaml:84 > generic.xaml:2069] |
| `ButtonForegroundDisabled` | followable | L: #5C000000 [Button_tr.xaml:135 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [Button_tr.xaml:37 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [Button_tr.xaml:85 > generic.xaml:2026] |
| `ButtonBorderBrush` | followable | L: gradient ControlElevationBorderBrush [Button_tr.xaml:136 > Cany.xaml:382]; D: gradient ControlElevationBorderBrush [Button_tr.xaml:38 > Cany.xaml:186] | HC: SystemColorButtonTextColor (runtime) [Button_tr.xaml:86 > generic.xaml:2054] |
| `ButtonBorderBrushPointerOver` | followable | L: gradient ControlElevationBorderBrush [Button_tr.xaml:137 > Cany.xaml:382]; D: gradient ControlElevationBorderBrush [Button_tr.xaml:39 > Cany.xaml:186] | HC: SystemColorHighlightColor (runtime) [Button_tr.xaml:87 > Cany.xaml:513] |
| `ButtonBorderBrushPressed` | followable | L: #0F000000 [Button_tr.xaml:138 > Cany.xaml:333 > Cany.xaml:243]; D: #12FFFFFF [Button_tr.xaml:40 > Cany.xaml:129 > Cany.xaml:39] | HC: SystemColorHighlightTextColor (runtime) [Button_tr.xaml:88 > Cany.xaml:514] |
| `ButtonBorderBrushDisabled` | followable | L: #0F000000 [Button_tr.xaml:139 > Cany.xaml:333 > Cany.xaml:243]; D: #12FFFFFF [Button_tr.xaml:41 > Cany.xaml:129 > Cany.xaml:39] | HC: SystemColorGrayTextColor (runtime) [Button_tr.xaml:89 > generic.xaml:2032] |
| `ButtonPadding` | stated | L=D: 11,5,11,6 [Button_tr.xaml:152] | HC: 11,5,11,6 [Button_tr.xaml:152] |

Counts: CommonStyles/Button_themeresources.xaml {'accent': 3, 'followable': 33, 'stated': 2}

### ToggleButton_themeresources.xaml

| Key | Class | L / D | HC |
|---|---|---|---|
| `ToggleButtonBorderThemeThickness` | stated | L: 1 [ToggleButton_tr.xaml:121]; D: 1 [ToggleButton_tr.xaml:5] | HC: 1 [ToggleButton_tr.xaml:63] |
| `ToggleButtonCheckedStateBackgroundSizing` | stated | L: OuterBorderEdge [ToggleButton_tr.xaml:122]; D: OuterBorderEdge [ToggleButton_tr.xaml:6] | HC: OuterBorderEdge [ToggleButton_tr.xaml:64] |
| `ToggleButtonBackground` | followable | L: #B3FFFFFF [ToggleButton_tr.xaml:123 > Cany.xaml:305 > Cany.xaml:219]; D: #0FFFFFFF [ToggleButton_tr.xaml:7 > Cany.xaml:101 > Cany.xaml:15] | HC: SystemColorButtonFaceColor (runtime) [ToggleButton_tr.xaml:65 > generic.xaml:2010] |
| `ToggleButtonBackgroundPointerOver` | followable | L: #80F9F9F9 [ToggleButton_tr.xaml:124 > Cany.xaml:306 > Cany.xaml:220]; D: #15FFFFFF [ToggleButton_tr.xaml:8 > Cany.xaml:102 > Cany.xaml:16] | HC: SystemColorHighlightTextColor (runtime) [ToggleButton_tr.xaml:66 > Cany.xaml:514] |
| `ToggleButtonBackgroundPressed` | followable | L: #4DF9F9F9 [ToggleButton_tr.xaml:125 > Cany.xaml:307 > Cany.xaml:221]; D: #08FFFFFF [ToggleButton_tr.xaml:9 > Cany.xaml:103 > Cany.xaml:17] | HC: SystemColorHighlightTextColor (runtime) [ToggleButton_tr.xaml:67 > Cany.xaml:514] |
| `ToggleButtonBackgroundDisabled` | followable | L: #4DF9F9F9 [ToggleButton_tr.xaml:126 > Cany.xaml:309 > Cany.xaml:223]; D: #0BFFFFFF [ToggleButton_tr.xaml:10 > Cany.xaml:105 > Cany.xaml:19] | HC: SystemColorButtonFaceColor (runtime) [ToggleButton_tr.xaml:68 > generic.xaml:2010] |
| `ToggleButtonBackgroundChecked` | accent | L: SystemAccentColorDark1 (runtime) [ToggleButton_tr.xaml:127 > Cany.xaml:329]; D: SystemAccentColorLight2 (runtime) [ToggleButton_tr.xaml:11 > Cany.xaml:125] | HC: SystemColorHighlightColor (runtime) [ToggleButton_tr.xaml:69 > generic.xaml:2055] |
| `ToggleButtonBackgroundCheckedPointerOver` | accent | L: SystemAccentColorDark1 (runtime) [ToggleButton_tr.xaml:128 > Cany.xaml:330]; D: SystemAccentColorLight2 (runtime) [ToggleButton_tr.xaml:12 > Cany.xaml:126] | HC: SystemColorButtonTextColor (runtime) [ToggleButton_tr.xaml:70 > Cany.xaml:512] |
| `ToggleButtonBackgroundCheckedPressed` | accent | L: SystemAccentColorDark1 (runtime) [ToggleButton_tr.xaml:129 > Cany.xaml:331]; D: SystemAccentColorLight2 (runtime) [ToggleButton_tr.xaml:13 > Cany.xaml:127] | HC: SystemColorHighlightTextColor (runtime) [ToggleButton_tr.xaml:71 > Cany.xaml:514] |
| `ToggleButtonBackgroundCheckedDisabled` | followable | L: #37000000 [ToggleButton_tr.xaml:130 > Cany.xaml:332 > Cany.xaml:242]; D: #28FFFFFF [ToggleButton_tr.xaml:14 > Cany.xaml:128 > Cany.xaml:38] | HC: SystemColorButtonFaceColor (runtime) [ToggleButton_tr.xaml:72 > generic.xaml:2010] |
| `ToggleButtonBackgroundIndeterminate` | followable | L: #B3FFFFFF [ToggleButton_tr.xaml:131 > Cany.xaml:305 > Cany.xaml:219]; D: #0FFFFFFF [ToggleButton_tr.xaml:15 > Cany.xaml:101 > Cany.xaml:15] | HC: SystemColorButtonFaceColor (runtime) [ToggleButton_tr.xaml:73 > generic.xaml:2010] |
| `ToggleButtonBackgroundIndeterminatePointerOver` | followable | L: #80F9F9F9 [ToggleButton_tr.xaml:132 > Cany.xaml:306 > Cany.xaml:220]; D: #15FFFFFF [ToggleButton_tr.xaml:16 > Cany.xaml:102 > Cany.xaml:16] | HC: SystemColorButtonFaceColor (runtime) [ToggleButton_tr.xaml:74 > generic.xaml:2010] |
| `ToggleButtonBackgroundIndeterminatePressed` | followable | L: #4DF9F9F9 [ToggleButton_tr.xaml:133 > Cany.xaml:307 > Cany.xaml:221]; D: #08FFFFFF [ToggleButton_tr.xaml:17 > Cany.xaml:103 > Cany.xaml:17] | HC: SystemColorButtonFaceColor (runtime) [ToggleButton_tr.xaml:75 > generic.xaml:2013] |
| `ToggleButtonBackgroundIndeterminateDisabled` | followable | L: #4DF9F9F9 [ToggleButton_tr.xaml:134 > Cany.xaml:309 > Cany.xaml:223]; D: #0BFFFFFF [ToggleButton_tr.xaml:18 > Cany.xaml:105 > Cany.xaml:19] | HC: SystemColorButtonFaceColor (runtime) [ToggleButton_tr.xaml:76 > generic.xaml:2010] |
| `ToggleButtonForeground` | followable | L: #E4000000 [ToggleButton_tr.xaml:135 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ToggleButton_tr.xaml:19 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [ToggleButton_tr.xaml:77 > generic.xaml:2039] |
| `ToggleButtonForegroundPointerOver` | followable | L: #E4000000 [ToggleButton_tr.xaml:136 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ToggleButton_tr.xaml:20 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightColor (runtime) [ToggleButton_tr.xaml:78 > generic.xaml:2069] |
| `ToggleButtonForegroundPressed` | followable | L: #9E000000 [ToggleButton_tr.xaml:137 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [ToggleButton_tr.xaml:21 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightColor (runtime) [ToggleButton_tr.xaml:79 > generic.xaml:2069] |
| `ToggleButtonForegroundDisabled` | followable | L: #5C000000 [ToggleButton_tr.xaml:138 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [ToggleButton_tr.xaml:22 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [ToggleButton_tr.xaml:80 > generic.xaml:2026] |
| `ToggleButtonForegroundChecked` | followable | L: #FFFFFF [ToggleButton_tr.xaml:139 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [ToggleButton_tr.xaml:23 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorHighlightTextColor (runtime) [ToggleButton_tr.xaml:81 > generic.xaml:2067] |
| `ToggleButtonForegroundCheckedPointerOver` | followable | L: #FFFFFF [ToggleButton_tr.xaml:140 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [ToggleButton_tr.xaml:24 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorButtonFaceColor (runtime) [ToggleButton_tr.xaml:82 > Cany.xaml:511] |
| `ToggleButtonForegroundCheckedPressed` | followable | L: #B3FFFFFF [ToggleButton_tr.xaml:141 > Cany.xaml:303 > Cany.xaml:217]; D: #80000000 [ToggleButton_tr.xaml:25 > Cany.xaml:99 > Cany.xaml:13] | HC: SystemColorHighlightColor (runtime) [ToggleButton_tr.xaml:83 > Cany.xaml:513] |
| `ToggleButtonForegroundCheckedDisabled` | followable | L: #FFFFFF [ToggleButton_tr.xaml:142 > Cany.xaml:218]; D: #87FFFFFF [ToggleButton_tr.xaml:26 > Cany.xaml:14] | HC: SystemColorGrayTextColor (runtime) [ToggleButton_tr.xaml:84 > generic.xaml:2026] |
| `ToggleButtonForegroundIndeterminate` | followable | L: #E4000000 [ToggleButton_tr.xaml:143 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ToggleButton_tr.xaml:27 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [ToggleButton_tr.xaml:85 > generic.xaml:2039] |
| `ToggleButtonForegroundIndeterminatePointerOver` | followable | L: #E4000000 [ToggleButton_tr.xaml:144 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ToggleButton_tr.xaml:28 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightColor (runtime) [ToggleButton_tr.xaml:86 > generic.xaml:2069] |
| `ToggleButtonForegroundIndeterminatePressed` | followable | L: #9E000000 [ToggleButton_tr.xaml:145 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [ToggleButton_tr.xaml:29 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightColor (runtime) [ToggleButton_tr.xaml:87 > generic.xaml:2069] |
| `ToggleButtonForegroundIndeterminateDisabled` | followable | L: #5C000000 [ToggleButton_tr.xaml:146 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [ToggleButton_tr.xaml:30 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [ToggleButton_tr.xaml:88 > generic.xaml:2026] |
| `ToggleButtonBorderBrush` | followable | L: gradient ControlElevationBorderBrush [ToggleButton_tr.xaml:147 > Cany.xaml:382]; D: gradient ControlElevationBorderBrush [ToggleButton_tr.xaml:31 > Cany.xaml:186] | HC: SystemColorButtonTextColor (runtime) [ToggleButton_tr.xaml:89 > generic.xaml:2054] |
| `ToggleButtonBorderBrushPointerOver` | followable | L: gradient ControlElevationBorderBrush [ToggleButton_tr.xaml:148 > Cany.xaml:382]; D: gradient ControlElevationBorderBrush [ToggleButton_tr.xaml:32 > Cany.xaml:186] | HC: SystemColorHighlightColor (runtime) [ToggleButton_tr.xaml:90 > generic.xaml:2073] |
| `ToggleButtonBorderBrushPressed` | followable | L: #0F000000 [ToggleButton_tr.xaml:149 > Cany.xaml:333 > Cany.xaml:243]; D: #12FFFFFF [ToggleButton_tr.xaml:33 > Cany.xaml:129 > Cany.xaml:39] | HC: SystemColorHighlightTextColor (runtime) [ToggleButton_tr.xaml:91 > Cany.xaml:514] |
| `ToggleButtonBorderBrushDisabled` | followable | L: #0F000000 [ToggleButton_tr.xaml:150 > Cany.xaml:333 > Cany.xaml:243]; D: #12FFFFFF [ToggleButton_tr.xaml:34 > Cany.xaml:129 > Cany.xaml:39] | HC: SystemColorGrayTextColor (runtime) [ToggleButton_tr.xaml:92 > generic.xaml:2032] |
| `ToggleButtonBorderBrushChecked` | followable | L: gradient AccentControlElevationBorderBrush [ToggleButton_tr.xaml:151 > Cany.xaml:397]; D: gradient AccentControlElevationBorderBrush [ToggleButton_tr.xaml:35 > Cany.xaml:198] | HC: SystemColorHighlightTextColor (runtime) [ToggleButton_tr.xaml:93 > generic.xaml:2068] |
| `ToggleButtonBorderBrushCheckedPointerOver` | followable | L: gradient AccentControlElevationBorderBrush [ToggleButton_tr.xaml:152 > Cany.xaml:397]; D: gradient AccentControlElevationBorderBrush [ToggleButton_tr.xaml:36 > Cany.xaml:198] | HC: SystemColorButtonTextColor (runtime) [ToggleButton_tr.xaml:94 > Cany.xaml:512] |
| `ToggleButtonBorderBrushCheckedPressed` | followable | L: #00FFFFFF [ToggleButton_tr.xaml:153 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [ToggleButton_tr.xaml:37 > Cany.xaml:106 > Cany.xaml:20] | HC: SystemColorHighlightTextColor (runtime) [ToggleButton_tr.xaml:95 > Cany.xaml:514] |
| `ToggleButtonBorderBrushCheckedDisabled` | followable | L: #00FFFFFF [ToggleButton_tr.xaml:154 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [ToggleButton_tr.xaml:38 > Cany.xaml:106 > Cany.xaml:20] | HC: SystemColorGrayTextColor (runtime) [ToggleButton_tr.xaml:96 > generic.xaml:2032] |
| `ToggleButtonBorderBrushIndeterminate` | followable | L: gradient ControlElevationBorderBrush [ToggleButton_tr.xaml:155 > Cany.xaml:382]; D: gradient ControlElevationBorderBrush [ToggleButton_tr.xaml:39 > Cany.xaml:186] | HC: SystemColorButtonTextColor (runtime) [ToggleButton_tr.xaml:97 > generic.xaml:2054] |
| `ToggleButtonBorderBrushIndeterminatePointerOver` | followable | L: gradient ControlElevationBorderBrush [ToggleButton_tr.xaml:156 > Cany.xaml:382]; D: gradient ControlElevationBorderBrush [ToggleButton_tr.xaml:40 > Cany.xaml:186] | HC: SystemColorHighlightColor (runtime) [ToggleButton_tr.xaml:98 > generic.xaml:2073] |
| `ToggleButtonBorderBrushIndeterminatePressed` | followable | L: #0F000000 [ToggleButton_tr.xaml:157 > Cany.xaml:333 > Cany.xaml:243]; D: #12FFFFFF [ToggleButton_tr.xaml:41 > Cany.xaml:129 > Cany.xaml:39] | HC: SystemColorHighlightColor (runtime) [ToggleButton_tr.xaml:99 > generic.xaml:2082] |
| `ToggleButtonBorderBrushIndeterminateDisabled` | followable | L: #0F000000 [ToggleButton_tr.xaml:158 > Cany.xaml:333 > Cany.xaml:243]; D: #12FFFFFF [ToggleButton_tr.xaml:42 > Cany.xaml:129 > Cany.xaml:39] | HC: SystemColorGrayTextColor (runtime) [ToggleButton_tr.xaml:100 > generic.xaml:2032] |

Counts: CommonStyles/ToggleButton_themeresources.xaml {'stated': 2, 'followable': 33, 'accent': 3}

### AppBarButton_themeresources.xaml

| Key | Class | L / D | HC |
|---|---|---|---|
| `AppBarButtonBackground` | followable | L: #00FFFFFF [AppBarButton_tr.xaml:75 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [AppBarButton_tr.xaml:5 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [AppBarButton_tr.xaml:40 > Common_tr.xaml:34] |
| `AppBarButtonBackgroundPointerOver` | followable | L: #09000000 [AppBarButton_tr.xaml:76 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [AppBarButton_tr.xaml:6 > Cany.xaml:112 > Cany.xaml:26] | HC: SystemColorHighlightColor (runtime) [AppBarButton_tr.xaml:41 > generic.xaml:2080] |
| `AppBarButtonBackgroundPressed` | followable | L: #06000000 [AppBarButton_tr.xaml:77 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [AppBarButton_tr.xaml:7 > Cany.xaml:113 > Cany.xaml:27] | HC: SystemColorHighlightColor (runtime) [AppBarButton_tr.xaml:42 > generic.xaml:2079] |
| `AppBarButtonBackgroundDisabled` | followable | L: #00FFFFFF [AppBarButton_tr.xaml:78 > Cany.xaml:318 > Cany.xaml:232]; D: #00FFFFFF [AppBarButton_tr.xaml:8 > Cany.xaml:114 > Cany.xaml:28] | HC: Transparent [AppBarButton_tr.xaml:43 > Common_tr.xaml:34] |
| `AppBarButtonForeground` | followable | L: #E4000000 [AppBarButton_tr.xaml:79 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [AppBarButton_tr.xaml:9 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [AppBarButton_tr.xaml:44 > generic.xaml:2039] |
| `AppBarButtonForegroundPointerOver` | followable | L: #E4000000 [AppBarButton_tr.xaml:80 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [AppBarButton_tr.xaml:10 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [AppBarButton_tr.xaml:45 > generic.xaml:2058] |
| `AppBarButtonForegroundPressed` | followable | L: #9E000000 [AppBarButton_tr.xaml:81 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [AppBarButton_tr.xaml:11 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [AppBarButton_tr.xaml:46 > generic.xaml:2058] |
| `AppBarButtonForegroundDisabled` | followable | L: #5C000000 [AppBarButton_tr.xaml:82 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [AppBarButton_tr.xaml:12 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [AppBarButton_tr.xaml:47 > generic.xaml:2026] |
| `AppBarButtonBorderBrush` | followable | L: #00FFFFFF [AppBarButton_tr.xaml:83 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [AppBarButton_tr.xaml:13 > Cany.xaml:106 > Cany.xaml:20] | HC: SystemColorButtonTextColor (runtime) [AppBarButton_tr.xaml:48 > generic.xaml:2054] |
| `AppBarButtonBorderBrushPointerOver` | followable | L: #00FFFFFF [AppBarButton_tr.xaml:84 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [AppBarButton_tr.xaml:14 > Cany.xaml:106 > Cany.xaml:20] | HC: SystemColorHighlightColor (runtime) [AppBarButton_tr.xaml:49 > generic.xaml:2082] |
| `AppBarButtonBorderBrushPressed` | followable | L: #00FFFFFF [AppBarButton_tr.xaml:85 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [AppBarButton_tr.xaml:15 > Cany.xaml:106 > Cany.xaml:20] | HC: SystemColorHighlightColor (runtime) [AppBarButton_tr.xaml:50 > generic.xaml:2082] |
| `AppBarButtonBorderBrushDisabled` | followable | L: #00FFFFFF [AppBarButton_tr.xaml:86 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [AppBarButton_tr.xaml:16 > Cany.xaml:106 > Cany.xaml:20] | HC: SystemColorGrayTextColor (runtime) [AppBarButton_tr.xaml:51 > generic.xaml:2032] |
| `AppBarButtonKeyboardAcceleratorTextForeground` | followable | L: #9E000000 [AppBarButton_tr.xaml:87 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [AppBarButton_tr.xaml:17 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorButtonTextColor (runtime) [AppBarButton_tr.xaml:52 > generic.xaml:2041] |
| `AppBarButtonKeyboardAcceleratorTextForegroundPointerOver` | followable | L: #9E000000 [AppBarButton_tr.xaml:88 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [AppBarButton_tr.xaml:18 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [AppBarButton_tr.xaml:53 > generic.xaml:2060] |
| `AppBarButtonKeyboardAcceleratorTextForegroundPressed` | followable | L: #72000000 [AppBarButton_tr.xaml:89 > Cany.xaml:294 > Cany.xaml:211]; D: #87FFFFFF [AppBarButton_tr.xaml:19 > Cany.xaml:90 > Cany.xaml:7] | HC: SystemColorHighlightTextColor (runtime) [AppBarButton_tr.xaml:54 > generic.xaml:2060] |
| `AppBarButtonKeyboardAcceleratorTextForegroundDisabled` | followable | L: #5C000000 [AppBarButton_tr.xaml:90 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [AppBarButton_tr.xaml:20 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [AppBarButton_tr.xaml:55 > generic.xaml:2026] |
| `AppBarButtonBackgroundSubMenuOpened` | followable | L: #09000000 [AppBarButton_tr.xaml:91 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [AppBarButton_tr.xaml:21 > Cany.xaml:112 > Cany.xaml:26] | HC: SystemColorHighlightColor (runtime) [AppBarButton_tr.xaml:56 > generic.xaml:2077] |
| `AppBarButtonForegroundSubMenuOpened` | followable | L: #E4000000 [AppBarButton_tr.xaml:92 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [AppBarButton_tr.xaml:22 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [AppBarButton_tr.xaml:57 > generic.xaml:2058] |
| `AppBarButtonKeyboardAcceleratorTextForegroundSubMenuOpened` | followable | L: #9E000000 [AppBarButton_tr.xaml:93 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [AppBarButton_tr.xaml:23 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [AppBarButton_tr.xaml:58 > generic.xaml:2060] |
| `AppBarButtonBorderBrushSubMenuOpened` | followable | L: #00FFFFFF [AppBarButton_tr.xaml:94 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [AppBarButton_tr.xaml:24 > Cany.xaml:106 > Cany.xaml:20] | HC: Transparent [AppBarButton_tr.xaml:59 > Common_tr.xaml:34] |
| `AppBarButtonSubItemChevronForeground` | followable | L: #9E000000 [AppBarButton_tr.xaml:95 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [AppBarButton_tr.xaml:25 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorButtonTextColor (runtime) [AppBarButton_tr.xaml:60 > generic.xaml:2042] |
| `AppBarButtonSubItemChevronForegroundPointerOver` | followable | L: #9E000000 [AppBarButton_tr.xaml:96 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [AppBarButton_tr.xaml:26 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [AppBarButton_tr.xaml:61 > generic.xaml:2058] |
| `AppBarButtonSubItemChevronForegroundPressed` | followable | L: #72000000 [AppBarButton_tr.xaml:97 > Cany.xaml:294 > Cany.xaml:211]; D: #87FFFFFF [AppBarButton_tr.xaml:27 > Cany.xaml:90 > Cany.xaml:7] | HC: SystemColorHighlightTextColor (runtime) [AppBarButton_tr.xaml:62 > generic.xaml:2058] |
| `AppBarButtonSubItemChevronForegroundSubMenuOpened` | followable | L: #9E000000 [AppBarButton_tr.xaml:98 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [AppBarButton_tr.xaml:28 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [AppBarButton_tr.xaml:63 > generic.xaml:2058] |
| `AppBarButtonSubItemChevronForegroundDisabled` | followable | L: #5C000000 [AppBarButton_tr.xaml:99 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [AppBarButton_tr.xaml:29 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [AppBarButton_tr.xaml:64 > generic.xaml:2026] |
| `AppBarButtonHasFlyoutChevronVisibility` | stated | L: Visible [AppBarButton_tr.xaml:100]; D: Visible [AppBarButton_tr.xaml:30] | HC: Visible [AppBarButton_tr.xaml:65] |
| `AppBarButtonFlyoutGlyph` | stated | L: &#xE974; [AppBarButton_tr.xaml:101]; D: &#xE974; [AppBarButton_tr.xaml:31] | HC: &#xE974; [AppBarButton_tr.xaml:66] |
| `AppBarButtonOverflowFlyoutGlyph` | stated | L: &#xE974; [AppBarButton_tr.xaml:102]; D: &#xE974; [AppBarButton_tr.xaml:32] | HC: &#xE974; [AppBarButton_tr.xaml:67] |
| `AppBarButtonSubItemChevronFontSize` | stated | L: 8 [AppBarButton_tr.xaml:103]; D: 8 [AppBarButton_tr.xaml:33] | HC: 8 [AppBarButton_tr.xaml:68] |
| `AppBarButtonSecondarySubItemChevronFontSize` | stated | L: 12 [AppBarButton_tr.xaml:104]; D: 12 [AppBarButton_tr.xaml:34] | HC: 12 [AppBarButton_tr.xaml:69] |
| `AppBarButtonSubItemChevronMargin` | stated | L: -23,20,12,0 [AppBarButton_tr.xaml:105]; D: -23,20,12,0 [AppBarButton_tr.xaml:35] | HC: -23,20,12,0 [AppBarButton_tr.xaml:70] |
| `AppBarButtonSubItemChevronLabelOnRightMargin` | stated | L: -7,20,12,0 [AppBarButton_tr.xaml:106]; D: -7,20,12,0 [AppBarButton_tr.xaml:36] | HC: -7,20,12,0 [AppBarButton_tr.xaml:71] |
| `AppBarButtonSecondarySubItemChevronMargin` | stated | L: 0,0,16,0 [AppBarButton_tr.xaml:107]; D: 0,0,16,0 [AppBarButton_tr.xaml:37] | HC: 0,0,16,0 [AppBarButton_tr.xaml:72] |
| `AppBarButtonContentViewboxMargin` | stated | L=D: 12,16,0,10 [AppBarButton_tr.xaml:110] | HC: 12,16,0,10 [AppBarButton_tr.xaml:110] |
| `AppBarButtonContentViewboxCompactMargin` | stated | L=D: 0,12,0,12 [AppBarButton_tr.xaml:111] | HC: 0,12,0,12 [AppBarButton_tr.xaml:111] |
| `AppBarButtonContentViewboxCollapsedMargin` | stated | L=D: 0,16,0,2 [AppBarButton_tr.xaml:112] | HC: 0,16,0,2 [AppBarButton_tr.xaml:112] |
| `AppBarButtonOverflowTextTouchMargin` | stated | L=D: 0,9,0,12 [AppBarButton_tr.xaml:113] | HC: 0,9,0,12 [AppBarButton_tr.xaml:113] |
| `AppBarButtonOverflowTextLabelPadding` | stated | L=D: 0,5,0,8 [AppBarButton_tr.xaml:114] | HC: 0,5,0,8 [AppBarButton_tr.xaml:114] |
| `AppBarButtonTextLabelMargin` | stated | L=D: 2,0,2,8 [AppBarButton_tr.xaml:115] | HC: 2,0,2,8 [AppBarButton_tr.xaml:115] |
| `AppBarButtonTextLabelOnRightMargin` | stated | L=D: 8,16,12,10 [AppBarButton_tr.xaml:116] | HC: 8,16,12,10 [AppBarButton_tr.xaml:116] |
| `AppBarButtonInnerBorderMargin` | stated | L=D: 2,6,2,6 [AppBarButton_tr.xaml:117] | HC: 2,6,2,6 [AppBarButton_tr.xaml:117] |
| `AppBarButtonInnerBorderCompactMargin` | stated | L=D: 2,6,2,22 [AppBarButton_tr.xaml:118] | HC: 2,6,2,22 [AppBarButton_tr.xaml:118] |
| `AppBarButtonInnerBorderOverflowMargin` | stated | L=D: 4,0,4,0 [AppBarButton_tr.xaml:119] | HC: 4,0,4,0 [AppBarButton_tr.xaml:119] |

Counts: CommonStyles/AppBarButton_themeresources.xaml {'followable': 25, 'stated': 18}

### AppBarToggleButton_themeresources.xaml

| Key | Class | L / D | HC |
|---|---|---|---|
| `AppBarToggleButtonBackground` | followable | L: #00FFFFFF [AppBarToggleButton_tr.xaml:135 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [AppBarToggleButton_tr.xaml:5 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorButtonFaceColor (runtime) [AppBarToggleButton_tr.xaml:70 > Cany.xaml:511] |
| `AppBarToggleButtonBackgroundPointerOver` | followable | L: #09000000 [AppBarToggleButton_tr.xaml:136 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [AppBarToggleButton_tr.xaml:6 > Cany.xaml:112 > Cany.xaml:26] | HC: SystemColorHighlightColor (runtime) [AppBarToggleButton_tr.xaml:71 > Cany.xaml:513] |
| `AppBarToggleButtonBackgroundPressed` | followable | L: #06000000 [AppBarToggleButton_tr.xaml:137 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [AppBarToggleButton_tr.xaml:7 > Cany.xaml:113 > Cany.xaml:27] | HC: SystemColorHighlightColor (runtime) [AppBarToggleButton_tr.xaml:72 > Cany.xaml:513] |
| `AppBarToggleButtonBackgroundDisabled` | followable | L: #00FFFFFF [AppBarToggleButton_tr.xaml:138 > Cany.xaml:318 > Cany.xaml:232]; D: #00FFFFFF [AppBarToggleButton_tr.xaml:8 > Cany.xaml:114 > Cany.xaml:28] | HC: SystemColorWindowColor (runtime) [AppBarToggleButton_tr.xaml:73 > Cany.xaml:510] |
| `AppBarToggleButtonBackgroundChecked` | accent | L: SystemAccentColorDark1 (runtime) [AppBarToggleButton_tr.xaml:139 > Cany.xaml:329]; D: SystemAccentColorLight2 (runtime) [AppBarToggleButton_tr.xaml:9 > Cany.xaml:125] | HC: SystemColorHighlightColor (runtime) [AppBarToggleButton_tr.xaml:74 > Cany.xaml:513] |
| `AppBarToggleButtonBackgroundCheckedPointerOver` | accent | L: SystemAccentColorDark1 (runtime) [AppBarToggleButton_tr.xaml:140 > Cany.xaml:330]; D: SystemAccentColorLight2 (runtime) [AppBarToggleButton_tr.xaml:10 > Cany.xaml:126] | HC: SystemColorHighlightTextColor (runtime) [AppBarToggleButton_tr.xaml:75 > Cany.xaml:514] |
| `AppBarToggleButtonBackgroundCheckedPressed` | accent | L: SystemAccentColorDark1 (runtime) [AppBarToggleButton_tr.xaml:141 > Cany.xaml:331]; D: SystemAccentColorLight2 (runtime) [AppBarToggleButton_tr.xaml:11 > Cany.xaml:127] | HC: SystemColorHighlightTextColor (runtime) [AppBarToggleButton_tr.xaml:76 > Cany.xaml:514] |
| `AppBarToggleButtonBackgroundCheckedDisabled` | followable | L: #37000000 [AppBarToggleButton_tr.xaml:142 > Cany.xaml:332 > Cany.xaml:242]; D: #28FFFFFF [AppBarToggleButton_tr.xaml:12 > Cany.xaml:128 > Cany.xaml:38] | HC: SystemColorGrayTextColor (runtime) [AppBarToggleButton_tr.xaml:77 > Cany.xaml:516] |
| `AppBarToggleButtonBackgroundHighLightOverlay` | followable | L: #00FFFFFF [AppBarToggleButton_tr.xaml:143 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [AppBarToggleButton_tr.xaml:13 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [AppBarToggleButton_tr.xaml:78 > Common_tr.xaml:34] |
| `AppBarToggleButtonBackgroundHighLightOverlayPointerOver` | followable | L: #09000000 [AppBarToggleButton_tr.xaml:144 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [AppBarToggleButton_tr.xaml:14 > Cany.xaml:112 > Cany.xaml:26] | HC: SystemColorHighlightColor (runtime) [AppBarToggleButton_tr.xaml:79 > generic.xaml:2080] |
| `AppBarToggleButtonBackgroundHighLightOverlayPressed` | followable | L: #06000000 [AppBarToggleButton_tr.xaml:145 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [AppBarToggleButton_tr.xaml:15 > Cany.xaml:113 > Cany.xaml:27] | HC: SystemColorHighlightColor (runtime) [AppBarToggleButton_tr.xaml:80 > generic.xaml:2079] |
| `AppBarToggleButtonBackgroundHighLightOverlayCheckedPointerOver` | followable | L: #09000000 [AppBarToggleButton_tr.xaml:146 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [AppBarToggleButton_tr.xaml:16 > Cany.xaml:112 > Cany.xaml:26] | HC: SystemColorHighlightColor (runtime) [AppBarToggleButton_tr.xaml:81 > generic.xaml:2080] |
| `AppBarToggleButtonBackgroundHighLightOverlayCheckedPressed` | followable | L: #06000000 [AppBarToggleButton_tr.xaml:147 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [AppBarToggleButton_tr.xaml:17 > Cany.xaml:113 > Cany.xaml:27] | HC: SystemColorHighlightColor (runtime) [AppBarToggleButton_tr.xaml:82 > generic.xaml:2079] |
| `AppBarToggleButtonForeground` | followable | L: #E4000000 [AppBarToggleButton_tr.xaml:148 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [AppBarToggleButton_tr.xaml:18 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [AppBarToggleButton_tr.xaml:83 > Cany.xaml:512] |
| `AppBarToggleButtonForegroundPointerOver` | followable | L: #E4000000 [AppBarToggleButton_tr.xaml:149 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [AppBarToggleButton_tr.xaml:19 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [AppBarToggleButton_tr.xaml:84 > Cany.xaml:514] |
| `AppBarToggleButtonForegroundPressed` | followable | L: #9E000000 [AppBarToggleButton_tr.xaml:150 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [AppBarToggleButton_tr.xaml:20 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [AppBarToggleButton_tr.xaml:85 > Cany.xaml:514] |
| `AppBarToggleButtonForegroundDisabled` | followable | L: #5C000000 [AppBarToggleButton_tr.xaml:151 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [AppBarToggleButton_tr.xaml:21 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [AppBarToggleButton_tr.xaml:86 > Cany.xaml:516] |
| `AppBarToggleButtonForegroundChecked` | followable | L: #FFFFFF [AppBarToggleButton_tr.xaml:152 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [AppBarToggleButton_tr.xaml:22 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorHighlightTextColor (runtime) [AppBarToggleButton_tr.xaml:87 > Cany.xaml:514] |
| `AppBarToggleButtonForegroundCheckedPointerOver` | followable | L: #FFFFFF [AppBarToggleButton_tr.xaml:153 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [AppBarToggleButton_tr.xaml:23 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorHighlightColor (runtime) [AppBarToggleButton_tr.xaml:88 > Cany.xaml:513] |
| `AppBarToggleButtonForegroundCheckedPressed` | followable | L: #B3FFFFFF [AppBarToggleButton_tr.xaml:154 > Cany.xaml:303 > Cany.xaml:217]; D: #80000000 [AppBarToggleButton_tr.xaml:24 > Cany.xaml:99 > Cany.xaml:13] | HC: SystemColorHighlightColor (runtime) [AppBarToggleButton_tr.xaml:89 > Cany.xaml:513] |
| `AppBarToggleButtonForegroundCheckedDisabled` | followable | L: #FFFFFF [AppBarToggleButton_tr.xaml:155 > Cany.xaml:218]; D: #87FFFFFF [AppBarToggleButton_tr.xaml:25 > Cany.xaml:14] | HC: SystemColorWindowColor (runtime) [AppBarToggleButton_tr.xaml:90 > Cany.xaml:510] |
| `AppBarToggleButtonBorderBrush` | followable | L: #00FFFFFF [AppBarToggleButton_tr.xaml:156 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [AppBarToggleButton_tr.xaml:26 > Cany.xaml:106 > Cany.xaml:20] | HC: SystemColorButtonTextColor (runtime) [AppBarToggleButton_tr.xaml:91 > Cany.xaml:512] |
| `AppBarToggleButtonBorderBrushPointerOver` | followable | L: #00FFFFFF [AppBarToggleButton_tr.xaml:157 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [AppBarToggleButton_tr.xaml:27 > Cany.xaml:106 > Cany.xaml:20] | HC: SystemColorButtonTextColor (runtime) [AppBarToggleButton_tr.xaml:92 > Cany.xaml:512] |
| `AppBarToggleButtonBorderBrushPressed` | followable | L: #00FFFFFF [AppBarToggleButton_tr.xaml:158 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [AppBarToggleButton_tr.xaml:28 > Cany.xaml:106 > Cany.xaml:20] | HC: SystemColorButtonTextColor (runtime) [AppBarToggleButton_tr.xaml:93 > Cany.xaml:512] |
| `AppBarToggleButtonBorderBrushDisabled` | followable | L: #00FFFFFF [AppBarToggleButton_tr.xaml:159 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [AppBarToggleButton_tr.xaml:29 > Cany.xaml:106 > Cany.xaml:20] | HC: SystemColorGrayTextColor (runtime) [AppBarToggleButton_tr.xaml:94 > Cany.xaml:516] |
| `AppBarToggleButtonBorderBrushChecked` | followable | L: gradient AccentControlElevationBorderBrush [AppBarToggleButton_tr.xaml:160 > Cany.xaml:397]; D: gradient AccentControlElevationBorderBrush [AppBarToggleButton_tr.xaml:30 > Cany.xaml:198] | HC: SystemColorHighlightColor (runtime) [AppBarToggleButton_tr.xaml:95 > Cany.xaml:513] |
| `AppBarToggleButtonBorderBrushCheckedPointerOver` | followable | L: gradient AccentControlElevationBorderBrush [AppBarToggleButton_tr.xaml:161 > Cany.xaml:397]; D: gradient AccentControlElevationBorderBrush [AppBarToggleButton_tr.xaml:31 > Cany.xaml:198] | HC: SystemColorHighlightColor (runtime) [AppBarToggleButton_tr.xaml:96 > Cany.xaml:513] |
| `AppBarToggleButtonBorderBrushCheckedPressed` | followable | L: #00FFFFFF [AppBarToggleButton_tr.xaml:162 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [AppBarToggleButton_tr.xaml:32 > Cany.xaml:106 > Cany.xaml:20] | HC: SystemColorHighlightColor (runtime) [AppBarToggleButton_tr.xaml:97 > Cany.xaml:513] |
| `AppBarToggleButtonBorderBrushCheckedDisabled` | followable | L: #00FFFFFF [AppBarToggleButton_tr.xaml:163 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [AppBarToggleButton_tr.xaml:33 > Cany.xaml:106 > Cany.xaml:20] | HC: SystemColorGrayTextColor (runtime) [AppBarToggleButton_tr.xaml:98 > Cany.xaml:516] |
| `AppBarToggleButtonCheckGlyphForeground` | followable | L: #E4000000 [AppBarToggleButton_tr.xaml:164 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [AppBarToggleButton_tr.xaml:34 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [AppBarToggleButton_tr.xaml:99 > Cany.xaml:512] |
| `AppBarToggleButtonCheckGlyphForegroundPointerOver` | followable | L: #E4000000 [AppBarToggleButton_tr.xaml:165 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [AppBarToggleButton_tr.xaml:35 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [AppBarToggleButton_tr.xaml:100 > Cany.xaml:514] |
| `AppBarToggleButtonCheckGlyphForegroundPressed` | followable | L: #9E000000 [AppBarToggleButton_tr.xaml:166 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [AppBarToggleButton_tr.xaml:36 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [AppBarToggleButton_tr.xaml:101 > Cany.xaml:514] |
| `AppBarToggleButtonCheckGlyphForegroundDisabled` | followable | L: #5C000000 [AppBarToggleButton_tr.xaml:167 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [AppBarToggleButton_tr.xaml:37 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [AppBarToggleButton_tr.xaml:102 > Cany.xaml:516] |
| `AppBarToggleButtonCheckGlyphForegroundChecked` | followable | L: #FFFFFF [AppBarToggleButton_tr.xaml:168 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [AppBarToggleButton_tr.xaml:38 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorHighlightTextColor (runtime) [AppBarToggleButton_tr.xaml:103 > Cany.xaml:514] |
| `AppBarToggleButtonCheckGlyphForegroundCheckedPointerOver` | followable | L: #FFFFFF [AppBarToggleButton_tr.xaml:169 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [AppBarToggleButton_tr.xaml:39 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorHighlightColor (runtime) [AppBarToggleButton_tr.xaml:104 > Cany.xaml:513] |
| `AppBarToggleButtonCheckGlyphForegroundCheckedPressed` | followable | L: #B3FFFFFF [AppBarToggleButton_tr.xaml:170 > Cany.xaml:303 > Cany.xaml:217]; D: #80000000 [AppBarToggleButton_tr.xaml:40 > Cany.xaml:99 > Cany.xaml:13] | HC: SystemColorHighlightColor (runtime) [AppBarToggleButton_tr.xaml:105 > Cany.xaml:513] |
| `AppBarToggleButtonCheckGlyphForegroundCheckedDisabled` | followable | L: #FFFFFF [AppBarToggleButton_tr.xaml:171 > Cany.xaml:218]; D: #87FFFFFF [AppBarToggleButton_tr.xaml:41 > Cany.xaml:14] | HC: SystemColorWindowColor (runtime) [AppBarToggleButton_tr.xaml:106 > Cany.xaml:510] |
| `AppBarToggleButtonOverflowLabelForegroundPointerOver` | followable | L: #E4000000 [AppBarToggleButton_tr.xaml:172 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [AppBarToggleButton_tr.xaml:42 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [AppBarToggleButton_tr.xaml:107 > generic.xaml:2058] |
| `AppBarToggleButtonOverflowLabelForegroundPressed` | followable | L: #9E000000 [AppBarToggleButton_tr.xaml:173 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [AppBarToggleButton_tr.xaml:43 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [AppBarToggleButton_tr.xaml:108 > generic.xaml:2058] |
| `AppBarToggleButtonOverflowLabelForegroundDisabled` | followable | L: #5C000000 [AppBarToggleButton_tr.xaml:174 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [AppBarToggleButton_tr.xaml:44 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [AppBarToggleButton_tr.xaml:109 > generic.xaml:2026] |
| `AppBarToggleButtonOverflowLabelForegroundCheckedPointerOver` | followable | L: #E4000000 [AppBarToggleButton_tr.xaml:175 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [AppBarToggleButton_tr.xaml:45 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [AppBarToggleButton_tr.xaml:110 > generic.xaml:2058] |
| `AppBarToggleButtonOverflowLabelForegroundCheckedPressed` | followable | L: #9E000000 [AppBarToggleButton_tr.xaml:176 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [AppBarToggleButton_tr.xaml:46 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [AppBarToggleButton_tr.xaml:111 > generic.xaml:2058] |
| `AppBarToggleButtonOverflowLabelForegroundCheckedDisabled` | followable | L: #5C000000 [AppBarToggleButton_tr.xaml:177 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [AppBarToggleButton_tr.xaml:47 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [AppBarToggleButton_tr.xaml:112 > generic.xaml:2026] |
| `AppBarToggleButtonKeyboardAcceleratorTextForeground` | followable | L: #9E000000 [AppBarToggleButton_tr.xaml:190 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [AppBarToggleButton_tr.xaml:60 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorButtonTextColor (runtime) [AppBarToggleButton_tr.xaml:125 > generic.xaml:2041] |
| `AppBarToggleButtonKeyboardAcceleratorTextForegroundPointerOver` | followable | L: #9E000000 [AppBarToggleButton_tr.xaml:191 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [AppBarToggleButton_tr.xaml:61 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [AppBarToggleButton_tr.xaml:126 > generic.xaml:2060] |
| `AppBarToggleButtonKeyboardAcceleratorTextForegroundPressed` | followable | L: #72000000 [AppBarToggleButton_tr.xaml:192 > Cany.xaml:294 > Cany.xaml:211]; D: #87FFFFFF [AppBarToggleButton_tr.xaml:62 > Cany.xaml:90 > Cany.xaml:7] | HC: SystemColorHighlightTextColor (runtime) [AppBarToggleButton_tr.xaml:127 > generic.xaml:2060] |
| `AppBarToggleButtonKeyboardAcceleratorTextForegroundDisabled` | followable | L: #5C000000 [AppBarToggleButton_tr.xaml:193 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [AppBarToggleButton_tr.xaml:63 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [AppBarToggleButton_tr.xaml:128 > generic.xaml:2026] |
| `AppBarToggleButtonKeyboardAcceleratorTextForegroundChecked` | followable | L: #9E000000 [AppBarToggleButton_tr.xaml:194 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [AppBarToggleButton_tr.xaml:64 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorButtonTextColor (runtime) [AppBarToggleButton_tr.xaml:129 > generic.xaml:2041] |
| `AppBarToggleButtonKeyboardAcceleratorTextForegroundCheckedPointerOver` | followable | L: #9E000000 [AppBarToggleButton_tr.xaml:195 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [AppBarToggleButton_tr.xaml:65 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [AppBarToggleButton_tr.xaml:130 > generic.xaml:2060] |
| `AppBarToggleButtonKeyboardAcceleratorTextForegroundCheckedPressed` | followable | L: #72000000 [AppBarToggleButton_tr.xaml:196 > Cany.xaml:294 > Cany.xaml:211]; D: #87FFFFFF [AppBarToggleButton_tr.xaml:66 > Cany.xaml:90 > Cany.xaml:7] | HC: SystemColorHighlightTextColor (runtime) [AppBarToggleButton_tr.xaml:131 > generic.xaml:2060] |
| `AppBarToggleButtonKeyboardAcceleratorTextForegroundCheckedDisabled` | followable | L: #5C000000 [AppBarToggleButton_tr.xaml:197 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [AppBarToggleButton_tr.xaml:67 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [AppBarToggleButton_tr.xaml:132 > generic.xaml:2026] |
| `AppBarToggleButtonBorderThemeThickness` | stated | L=D: 1 [AppBarToggleButton_tr.xaml:200] | HC: 1 [AppBarToggleButton_tr.xaml:200] |
| `AppBarToggleButtonOverflowTextTouchMargin` | stated | L=D: 0,9,0,12 [AppBarToggleButton_tr.xaml:201] | HC: 0,9,0,12 [AppBarToggleButton_tr.xaml:201] |
| `AppBarToggleButtonOverflowCheckTouchMargin` | stated | L=D: 12,10,12,10 [AppBarToggleButton_tr.xaml:202] | HC: 12,10,12,10 [AppBarToggleButton_tr.xaml:202] |
| `AppBarToggleButtonOverflowCheckMargin` | stated | L=D: 12,4,12,4 [AppBarToggleButton_tr.xaml:203] | HC: 12,4,12,4 [AppBarToggleButton_tr.xaml:203] |
| `AppBarToggleButtonTextLabelMargin` | stated | L=D: 2,0,2,8 [AppBarToggleButton_tr.xaml:204] | HC: 2,0,2,8 [AppBarToggleButton_tr.xaml:204] |
| `AppBarToggleButtonTextLabelOnRightMargin` | stated | L=D: 8,16,12,10 [AppBarToggleButton_tr.xaml:205] | HC: 8,16,12,10 [AppBarToggleButton_tr.xaml:205] |
| `AppBarToggleButtonOverflowTextLabelPadding` | stated | L=D: 0,5,0,8 [AppBarToggleButton_tr.xaml:206] | HC: 0,5,0,8 [AppBarToggleButton_tr.xaml:206] |

Counts: CommonStyles/AppBarToggleButton_themeresources.xaml {'followable': 48, 'accent': 3, 'stated': 7}

### DropDownButton_themeresources.xaml

| Key | Class | L / D | HC |
|---|---|---|---|
| `DropDownButtonForegroundSecondary` | followable | L: #9E000000 [DropDownButton/DropDownButton_tr.xaml:10 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [DropDownButton/DropDownButton_tr.xaml:5 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorButtonTextColor (runtime) [DropDownButton/DropDownButton_tr.xaml:15 > Cany.xaml:512] |
| `DropDownButtonForegroundSecondaryPointerOver` | followable | L: #72000000 [DropDownButton/DropDownButton_tr.xaml:11 > Cany.xaml:294 > Cany.xaml:211]; D: #87FFFFFF [DropDownButton/DropDownButton_tr.xaml:6 > Cany.xaml:90 > Cany.xaml:7] | HC: SystemColorHighlightColor (runtime) [DropDownButton/DropDownButton_tr.xaml:16 > Cany.xaml:513] |
| `DropDownButtonForegroundSecondaryPressed` | followable | L: #72000000 [DropDownButton/DropDownButton_tr.xaml:12 > Cany.xaml:294 > Cany.xaml:211]; D: #87FFFFFF [DropDownButton/DropDownButton_tr.xaml:7 > Cany.xaml:90 > Cany.xaml:7] | HC: SystemColorHighlightColor (runtime) [DropDownButton/DropDownButton_tr.xaml:17 > Cany.xaml:513] |

Counts: DropDownButton/DropDownButton_themeresources.xaml {'followable': 3}

### RepeatButton_themeresources.xaml

| Key | Class | L / D | HC |
|---|---|---|---|
| `RepeatButtonBorderThemeThickness` | stated | L: 1 [RepeatButton_tr.xaml:53]; D: 1 [RepeatButton_tr.xaml:5] | HC: 1 [RepeatButton_tr.xaml:41] |
| `RepeatButtonBackground` | followable | L: #B3FFFFFF [RepeatButton_tr.xaml:54 > Cany.xaml:305 > Cany.xaml:219]; D: #0FFFFFFF [RepeatButton_tr.xaml:6 > Cany.xaml:101 > Cany.xaml:15] | HC: SystemColorButtonFaceColor (runtime) [RepeatButton_tr.xaml:29 > generic.xaml:2010] |
| `RepeatButtonBackgroundPointerOver` | followable | L: #80F9F9F9 [RepeatButton_tr.xaml:55 > Cany.xaml:306 > Cany.xaml:220]; D: #15FFFFFF [RepeatButton_tr.xaml:7 > Cany.xaml:102 > Cany.xaml:16] | HC: SystemColorButtonFaceColor (runtime) [RepeatButton_tr.xaml:30 > generic.xaml:2010] |
| `RepeatButtonBackgroundPressed` | followable | L: #4DF9F9F9 [RepeatButton_tr.xaml:56 > Cany.xaml:307 > Cany.xaml:221]; D: #08FFFFFF [RepeatButton_tr.xaml:8 > Cany.xaml:103 > Cany.xaml:17] | HC: SystemColorButtonFaceColor (runtime) [RepeatButton_tr.xaml:31 > generic.xaml:2013] |
| `RepeatButtonBackgroundDisabled` | followable | L: #4DF9F9F9 [RepeatButton_tr.xaml:57 > Cany.xaml:309 > Cany.xaml:223]; D: #0BFFFFFF [RepeatButton_tr.xaml:9 > Cany.xaml:105 > Cany.xaml:19] | HC: SystemColorButtonFaceColor (runtime) [RepeatButton_tr.xaml:32 > generic.xaml:2010] |
| `RepeatButtonForeground` | followable | L: #E4000000 [RepeatButton_tr.xaml:58 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [RepeatButton_tr.xaml:10 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [RepeatButton_tr.xaml:33 > generic.xaml:2039] |
| `RepeatButtonForegroundPointerOver` | followable | L: #E4000000 [RepeatButton_tr.xaml:59 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [RepeatButton_tr.xaml:11 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightColor (runtime) [RepeatButton_tr.xaml:34 > generic.xaml:2069] |
| `RepeatButtonForegroundPressed` | followable | L: #9E000000 [RepeatButton_tr.xaml:60 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [RepeatButton_tr.xaml:12 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightColor (runtime) [RepeatButton_tr.xaml:35 > generic.xaml:2069] |
| `RepeatButtonForegroundDisabled` | followable | L: #5C000000 [RepeatButton_tr.xaml:61 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [RepeatButton_tr.xaml:13 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [RepeatButton_tr.xaml:36 > generic.xaml:2026] |
| `RepeatButtonBorderBrush` | followable | L: gradient ControlElevationBorderBrush [RepeatButton_tr.xaml:62 > Cany.xaml:382]; D: gradient ControlElevationBorderBrush [RepeatButton_tr.xaml:14 > Cany.xaml:186] | HC: SystemColorButtonTextColor (runtime) [RepeatButton_tr.xaml:37 > generic.xaml:2054] |
| `RepeatButtonBorderBrushPointerOver` | followable | L: gradient ControlElevationBorderBrush [RepeatButton_tr.xaml:63 > Cany.xaml:382]; D: gradient ControlElevationBorderBrush [RepeatButton_tr.xaml:15 > Cany.xaml:186] | HC: SystemColorHighlightColor (runtime) [RepeatButton_tr.xaml:38 > generic.xaml:2073] |
| `RepeatButtonBorderBrushPressed` | followable | L: #0F000000 [RepeatButton_tr.xaml:64 > Cany.xaml:333 > Cany.xaml:243]; D: #12FFFFFF [RepeatButton_tr.xaml:16 > Cany.xaml:129 > Cany.xaml:39] | HC: SystemColorHighlightColor (runtime) [RepeatButton_tr.xaml:39 > generic.xaml:2082] |
| `RepeatButtonBorderBrushDisabled` | followable | L: #0F000000 [RepeatButton_tr.xaml:65 > Cany.xaml:333 > Cany.xaml:243]; D: #12FFFFFF [RepeatButton_tr.xaml:17 > Cany.xaml:129 > Cany.xaml:39] | HC: SystemColorGrayTextColor (runtime) [RepeatButton_tr.xaml:40 > generic.xaml:2032] |

Counts: CommonStyles/RepeatButton_themeresources.xaml {'stated': 1, 'followable': 12}

### CheckBox_themeresources.xaml

| Key | Class | L / D | HC |
|---|---|---|---|
| `CheckBoxForegroundUnchecked` | followable | L: #E4000000 [CheckBox_tr.xaml:181 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [CheckBox_tr.xaml:5 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [CheckBox_tr.xaml:93 > Cany.xaml:512] |
| `CheckBoxForegroundUncheckedPointerOver` | followable | L: #E4000000 [CheckBox_tr.xaml:182 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [CheckBox_tr.xaml:6 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [CheckBox_tr.xaml:94 > Cany.xaml:512] |
| `CheckBoxForegroundUncheckedPressed` | followable | L: #E4000000 [CheckBox_tr.xaml:183 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [CheckBox_tr.xaml:7 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [CheckBox_tr.xaml:95 > Cany.xaml:512] |
| `CheckBoxForegroundUncheckedDisabled` | followable | L: #5C000000 [CheckBox_tr.xaml:184 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [CheckBox_tr.xaml:8 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [CheckBox_tr.xaml:96 > Cany.xaml:516] |
| `CheckBoxForegroundChecked` | followable | L: #E4000000 [CheckBox_tr.xaml:185 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [CheckBox_tr.xaml:9 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [CheckBox_tr.xaml:97 > Cany.xaml:512] |
| `CheckBoxForegroundCheckedPointerOver` | followable | L: #E4000000 [CheckBox_tr.xaml:186 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [CheckBox_tr.xaml:10 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [CheckBox_tr.xaml:98 > Cany.xaml:512] |
| `CheckBoxForegroundCheckedPressed` | followable | L: #E4000000 [CheckBox_tr.xaml:187 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [CheckBox_tr.xaml:11 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [CheckBox_tr.xaml:99 > Cany.xaml:512] |
| `CheckBoxForegroundCheckedDisabled` | followable | L: #5C000000 [CheckBox_tr.xaml:188 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [CheckBox_tr.xaml:12 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [CheckBox_tr.xaml:100 > Cany.xaml:516] |
| `CheckBoxForegroundIndeterminate` | followable | L: #E4000000 [CheckBox_tr.xaml:189 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [CheckBox_tr.xaml:13 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorWindowTextColor (runtime) [CheckBox_tr.xaml:101 > Cany.xaml:509] |
| `CheckBoxForegroundIndeterminatePointerOver` | followable | L: #E4000000 [CheckBox_tr.xaml:190 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [CheckBox_tr.xaml:14 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorWindowTextColor (runtime) [CheckBox_tr.xaml:102 > Cany.xaml:509] |
| `CheckBoxForegroundIndeterminatePressed` | followable | L: #E4000000 [CheckBox_tr.xaml:191 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [CheckBox_tr.xaml:15 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorWindowTextColor (runtime) [CheckBox_tr.xaml:103 > Cany.xaml:509] |
| `CheckBoxForegroundIndeterminateDisabled` | followable | L: #5C000000 [CheckBox_tr.xaml:192 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [CheckBox_tr.xaml:16 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [CheckBox_tr.xaml:104 > Cany.xaml:516] |
| `CheckBoxBackgroundUnchecked` | followable | L: #00FFFFFF [CheckBox_tr.xaml:193 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:17 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:105 > Cany.xaml:510] |
| `CheckBoxBackgroundUncheckedPointerOver` | followable | L: #00FFFFFF [CheckBox_tr.xaml:194 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:18 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:106 > Cany.xaml:510] |
| `CheckBoxBackgroundUncheckedPressed` | followable | L: #00FFFFFF [CheckBox_tr.xaml:195 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:19 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:107 > Cany.xaml:510] |
| `CheckBoxBackgroundUncheckedDisabled` | followable | L: #00FFFFFF [CheckBox_tr.xaml:196 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:20 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:108 > Cany.xaml:510] |
| `CheckBoxBackgroundChecked` | followable | L: #00FFFFFF [CheckBox_tr.xaml:197 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:21 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:109 > Cany.xaml:510] |
| `CheckBoxBackgroundCheckedPointerOver` | followable | L: #00FFFFFF [CheckBox_tr.xaml:198 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:22 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:110 > Cany.xaml:510] |
| `CheckBoxBackgroundCheckedPressed` | followable | L: #00FFFFFF [CheckBox_tr.xaml:199 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:23 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:111 > Cany.xaml:510] |
| `CheckBoxBackgroundCheckedDisabled` | followable | L: #00FFFFFF [CheckBox_tr.xaml:200 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:24 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:112 > Cany.xaml:510] |
| `CheckBoxBackgroundIndeterminate` | followable | L: #00FFFFFF [CheckBox_tr.xaml:201 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:25 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:113 > Cany.xaml:510] |
| `CheckBoxBackgroundIndeterminatePointerOver` | followable | L: #00FFFFFF [CheckBox_tr.xaml:202 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:26 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:114 > Cany.xaml:510] |
| `CheckBoxBackgroundIndeterminatePressed` | followable | L: #00FFFFFF [CheckBox_tr.xaml:203 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:27 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:115 > Cany.xaml:510] |
| `CheckBoxBackgroundIndeterminateDisabled` | followable | L: #00FFFFFF [CheckBox_tr.xaml:204 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:28 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:116 > Cany.xaml:510] |
| `CheckBoxBorderBrushUnchecked` | followable | L: #00FFFFFF [CheckBox_tr.xaml:205 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:29 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:117 > Cany.xaml:510] |
| `CheckBoxBorderBrushUncheckedPointerOver` | followable | L: #00FFFFFF [CheckBox_tr.xaml:206 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:30 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:118 > Cany.xaml:510] |
| `CheckBoxBorderBrushUncheckedPressed` | followable | L: #00FFFFFF [CheckBox_tr.xaml:207 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:31 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:119 > Cany.xaml:510] |
| `CheckBoxBorderBrushUncheckedDisabled` | followable | L: #00FFFFFF [CheckBox_tr.xaml:208 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:32 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:120 > Cany.xaml:510] |
| `CheckBoxBorderBrushChecked` | followable | L: #00FFFFFF [CheckBox_tr.xaml:209 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:33 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:121 > Cany.xaml:510] |
| `CheckBoxBorderBrushCheckedPointerOver` | followable | L: #00FFFFFF [CheckBox_tr.xaml:210 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:34 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:122 > Cany.xaml:510] |
| `CheckBoxBorderBrushCheckedPressed` | followable | L: #00FFFFFF [CheckBox_tr.xaml:211 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:35 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:123 > Cany.xaml:510] |
| `CheckBoxBorderBrushCheckedDisabled` | followable | L: #00FFFFFF [CheckBox_tr.xaml:212 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:36 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:124 > Cany.xaml:510] |
| `CheckBoxBorderBrushIndeterminate` | followable | L: #00FFFFFF [CheckBox_tr.xaml:213 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:37 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:125 > Cany.xaml:510] |
| `CheckBoxBorderBrushIndeterminatePointerOver` | followable | L: #00FFFFFF [CheckBox_tr.xaml:214 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:38 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:126 > Cany.xaml:510] |
| `CheckBoxBorderBrushIndeterminatePressed` | followable | L: #00FFFFFF [CheckBox_tr.xaml:215 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:39 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:127 > Cany.xaml:510] |
| `CheckBoxBorderBrushIndeterminateDisabled` | followable | L: #00FFFFFF [CheckBox_tr.xaml:216 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [CheckBox_tr.xaml:40 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:128 > Cany.xaml:510] |
| `CheckBoxCheckBackgroundStrokeUnchecked` | followable | L: #72000000 [CheckBox_tr.xaml:217 > Cany.xaml:342 > Cany.xaml:252]; D: #8BFFFFFF [CheckBox_tr.xaml:41 > Cany.xaml:138 > Cany.xaml:48] | HC: SystemColorButtonTextColor (runtime) [CheckBox_tr.xaml:129 > Cany.xaml:512] |
| `CheckBoxCheckBackgroundStrokeUncheckedPointerOver` | followable | L: #72000000 [CheckBox_tr.xaml:218 > Cany.xaml:342 > Cany.xaml:252]; D: #8BFFFFFF [CheckBox_tr.xaml:42 > Cany.xaml:138 > Cany.xaml:48] | HC: SystemColorHighlightColor (runtime) [CheckBox_tr.xaml:130 > Cany.xaml:513] |
| `CheckBoxCheckBackgroundStrokeUncheckedPressed` | followable | L: #37000000 [CheckBox_tr.xaml:219 > Cany.xaml:343 > Cany.xaml:253]; D: #28FFFFFF [CheckBox_tr.xaml:43 > Cany.xaml:139 > Cany.xaml:49] | HC: SystemColorHighlightColor (runtime) [CheckBox_tr.xaml:131 > Cany.xaml:513] |
| `CheckBoxCheckBackgroundStrokeUncheckedDisabled` | followable | L: #37000000 [CheckBox_tr.xaml:220 > Cany.xaml:343 > Cany.xaml:253]; D: #28FFFFFF [CheckBox_tr.xaml:44 > Cany.xaml:139 > Cany.xaml:49] | HC: SystemColorGrayTextColor (runtime) [CheckBox_tr.xaml:132 > Cany.xaml:516] |
| `CheckBoxCheckBackgroundStrokeChecked` | accent | L: SystemAccentColorDark1 (runtime) [CheckBox_tr.xaml:221 > Cany.xaml:329]; D: SystemAccentColorLight2 (runtime) [CheckBox_tr.xaml:45 > Cany.xaml:125] | HC: SystemColorHighlightColor (runtime) [CheckBox_tr.xaml:133 > Cany.xaml:513] |
| `CheckBoxCheckBackgroundStrokeCheckedPointerOver` | accent | L: SystemAccentColorDark1 (runtime) [CheckBox_tr.xaml:222 > Cany.xaml:330]; D: SystemAccentColorLight2 (runtime) [CheckBox_tr.xaml:46 > Cany.xaml:126] | HC: SystemColorButtonTextColor (runtime) [CheckBox_tr.xaml:134 > Cany.xaml:512] |
| `CheckBoxCheckBackgroundStrokeCheckedPressed` | accent | L: SystemAccentColorDark1 (runtime) [CheckBox_tr.xaml:223 > Cany.xaml:331]; D: SystemAccentColorLight2 (runtime) [CheckBox_tr.xaml:47 > Cany.xaml:127] | HC: SystemColorButtonFaceColor (runtime) [CheckBox_tr.xaml:135 > Cany.xaml:511] |
| `CheckBoxCheckBackgroundStrokeCheckedDisabled` | followable | L: #37000000 [CheckBox_tr.xaml:224 > Cany.xaml:343 > Cany.xaml:253]; D: #28FFFFFF [CheckBox_tr.xaml:48 > Cany.xaml:139 > Cany.xaml:49] | HC: SystemColorGrayTextColor (runtime) [CheckBox_tr.xaml:136 > Cany.xaml:516] |
| `CheckBoxCheckBackgroundStrokeIndeterminate` | accent | L: SystemAccentColorDark1 (runtime) [CheckBox_tr.xaml:225 > Cany.xaml:329]; D: SystemAccentColorLight2 (runtime) [CheckBox_tr.xaml:49 > Cany.xaml:125] | HC: SystemColorHighlightColor (runtime) [CheckBox_tr.xaml:137 > Cany.xaml:513] |
| `CheckBoxCheckBackgroundStrokeIndeterminatePointerOver` | accent | L: SystemAccentColorDark1 (runtime) [CheckBox_tr.xaml:226 > Cany.xaml:330]; D: SystemAccentColorLight2 (runtime) [CheckBox_tr.xaml:50 > Cany.xaml:126] | HC: SystemColorHighlightColor (runtime) [CheckBox_tr.xaml:138 > Cany.xaml:513] |
| `CheckBoxCheckBackgroundStrokeIndeterminatePressed` | accent | L: SystemAccentColorDark1 (runtime) [CheckBox_tr.xaml:227 > Cany.xaml:331]; D: SystemAccentColorLight2 (runtime) [CheckBox_tr.xaml:51 > Cany.xaml:127] | HC: SystemColorHighlightColor (runtime) [CheckBox_tr.xaml:139 > Cany.xaml:513] |
| `CheckBoxCheckBackgroundStrokeIndeterminateDisabled` | followable | L: #37000000 [CheckBox_tr.xaml:228 > Cany.xaml:343 > Cany.xaml:253]; D: #28FFFFFF [CheckBox_tr.xaml:52 > Cany.xaml:139 > Cany.xaml:49] | HC: SystemColorGrayTextColor (runtime) [CheckBox_tr.xaml:140 > Cany.xaml:516] |
| `CheckBoxCheckBackgroundFillUnchecked` | followable | L: #06000000 [CheckBox_tr.xaml:229 > Cany.xaml:320 > Cany.xaml:234]; D: #19000000 [CheckBox_tr.xaml:53 > Cany.xaml:116 > Cany.xaml:30] | HC: SystemColorButtonFaceColor (runtime) [CheckBox_tr.xaml:141 > Cany.xaml:511] |
| `CheckBoxCheckBackgroundFillUncheckedPointerOver` | followable | L: #0F000000 [CheckBox_tr.xaml:230 > Cany.xaml:321 > Cany.xaml:235]; D: #0BFFFFFF [CheckBox_tr.xaml:54 > Cany.xaml:117 > Cany.xaml:31] | HC: SystemColorHighlightTextColor (runtime) [CheckBox_tr.xaml:142 > Cany.xaml:514] |
| `CheckBoxCheckBackgroundFillUncheckedPressed` | followable | L: #18000000 [CheckBox_tr.xaml:231 > Cany.xaml:322 > Cany.xaml:236]; D: #12FFFFFF [CheckBox_tr.xaml:55 > Cany.xaml:118 > Cany.xaml:32] | HC: SystemColorHighlightColor (runtime) [CheckBox_tr.xaml:143 > Cany.xaml:513] |
| `CheckBoxCheckBackgroundFillUncheckedDisabled` | followable | L: #00FFFFFF [CheckBox_tr.xaml:232 > Cany.xaml:323 > Cany.xaml:237]; D: #00FFFFFF [CheckBox_tr.xaml:56 > Cany.xaml:119 > Cany.xaml:33] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:144 > Cany.xaml:510] |
| `CheckBoxCheckBackgroundFillChecked` | accent | L: SystemAccentColorDark1 (runtime) [CheckBox_tr.xaml:233 > Cany.xaml:329]; D: SystemAccentColorLight2 (runtime) [CheckBox_tr.xaml:57 > Cany.xaml:125] | HC: SystemColorHighlightColor (runtime) [CheckBox_tr.xaml:145 > Cany.xaml:513] |
| `CheckBoxCheckBackgroundFillCheckedPointerOver` | accent | L: SystemAccentColorDark1 (runtime) [CheckBox_tr.xaml:234 > Cany.xaml:330]; D: SystemAccentColorLight2 (runtime) [CheckBox_tr.xaml:58 > Cany.xaml:126] | HC: SystemColorButtonTextColor (runtime) [CheckBox_tr.xaml:146 > Cany.xaml:512] |
| `CheckBoxCheckBackgroundFillCheckedPressed` | accent | L: SystemAccentColorDark1 (runtime) [CheckBox_tr.xaml:235 > Cany.xaml:331]; D: SystemAccentColorLight2 (runtime) [CheckBox_tr.xaml:59 > Cany.xaml:127] | HC: SystemColorButtonFaceColor (runtime) [CheckBox_tr.xaml:147 > Cany.xaml:511] |
| `CheckBoxCheckBackgroundFillCheckedDisabled` | followable | L: #37000000 [CheckBox_tr.xaml:236 > Cany.xaml:332 > Cany.xaml:242]; D: #28FFFFFF [CheckBox_tr.xaml:60 > Cany.xaml:128 > Cany.xaml:38] | HC: SystemColorGrayTextColor (runtime) [CheckBox_tr.xaml:148 > Cany.xaml:516] |
| `CheckBoxCheckBackgroundFillIndeterminate` | accent | L: SystemAccentColorDark1 (runtime) [CheckBox_tr.xaml:237 > Cany.xaml:329]; D: SystemAccentColorLight2 (runtime) [CheckBox_tr.xaml:61 > Cany.xaml:125] | HC: SystemColorHighlightColor (runtime) [CheckBox_tr.xaml:149 > Cany.xaml:513] |
| `CheckBoxCheckBackgroundFillIndeterminatePointerOver` | accent | L: SystemAccentColorDark1 (runtime) [CheckBox_tr.xaml:238 > Cany.xaml:330]; D: SystemAccentColorLight2 (runtime) [CheckBox_tr.xaml:62 > Cany.xaml:126] | HC: SystemColorHighlightTextColor (runtime) [CheckBox_tr.xaml:150 > Cany.xaml:514] |
| `CheckBoxCheckBackgroundFillIndeterminatePressed` | accent | L: SystemAccentColorDark1 (runtime) [CheckBox_tr.xaml:239 > Cany.xaml:331]; D: SystemAccentColorLight2 (runtime) [CheckBox_tr.xaml:63 > Cany.xaml:127] | HC: SystemColorHighlightColor (runtime) [CheckBox_tr.xaml:151 > Cany.xaml:513] |
| `CheckBoxCheckBackgroundFillIndeterminateDisabled` | followable | L: #37000000 [CheckBox_tr.xaml:240 > Cany.xaml:332 > Cany.xaml:242]; D: #28FFFFFF [CheckBox_tr.xaml:64 > Cany.xaml:128 > Cany.xaml:38] | HC: SystemColorGrayTextColor (runtime) [CheckBox_tr.xaml:152 > Cany.xaml:516] |
| `CheckBoxCheckGlyphForegroundUnchecked` | followable | L: #FFFFFF [CheckBox_tr.xaml:241 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [CheckBox_tr.xaml:65 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorButtonFaceColor (runtime) [CheckBox_tr.xaml:153 > Cany.xaml:511] |
| `CheckBoxCheckGlyphForegroundUncheckedPointerOver` | followable | L: #FFFFFF [CheckBox_tr.xaml:242 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [CheckBox_tr.xaml:66 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorHighlightColor (runtime) [CheckBox_tr.xaml:154 > Cany.xaml:513] |
| `CheckBoxCheckGlyphForegroundUncheckedPressed` | followable | L: #FFFFFF [CheckBox_tr.xaml:243 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [CheckBox_tr.xaml:67 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorHighlightTextColor (runtime) [CheckBox_tr.xaml:155 > Cany.xaml:514] |
| `CheckBoxCheckGlyphForegroundUncheckedDisabled` | followable | L: #FFFFFF [CheckBox_tr.xaml:244 > Cany.xaml:304 > Cany.xaml:218]; D: #87FFFFFF [CheckBox_tr.xaml:68 > Cany.xaml:100 > Cany.xaml:14] | HC: SystemColorGrayTextColor (runtime) [CheckBox_tr.xaml:156 > Cany.xaml:516] |
| `CheckBoxCheckGlyphForegroundChecked` | followable | L: #FFFFFF [CheckBox_tr.xaml:245 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [CheckBox_tr.xaml:69 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorHighlightTextColor (runtime) [CheckBox_tr.xaml:157 > Cany.xaml:514] |
| `CheckBoxCheckGlyphForegroundCheckedPointerOver` | followable | L: #FFFFFF [CheckBox_tr.xaml:246 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [CheckBox_tr.xaml:70 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorButtonFaceColor (runtime) [CheckBox_tr.xaml:158 > Cany.xaml:511] |
| `CheckBoxCheckGlyphForegroundCheckedPressed` | followable | L: #B3FFFFFF [CheckBox_tr.xaml:247 > Cany.xaml:303 > Cany.xaml:217]; D: #80000000 [CheckBox_tr.xaml:71 > Cany.xaml:99 > Cany.xaml:13] | HC: SystemColorButtonTextColor (runtime) [CheckBox_tr.xaml:159 > Cany.xaml:512] |
| `CheckBoxCheckGlyphForegroundCheckedDisabled` | followable | L: #FFFFFF [CheckBox_tr.xaml:248 > Cany.xaml:304 > Cany.xaml:218]; D: #87FFFFFF [CheckBox_tr.xaml:72 > Cany.xaml:100 > Cany.xaml:14] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:160 > Cany.xaml:510] |
| `CheckBoxCheckGlyphForegroundIndeterminate` | followable | L: #FFFFFF [CheckBox_tr.xaml:249 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [CheckBox_tr.xaml:73 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorHighlightTextColor (runtime) [CheckBox_tr.xaml:161 > Cany.xaml:514] |
| `CheckBoxCheckGlyphForegroundIndeterminatePointerOver` | followable | L: #FFFFFF [CheckBox_tr.xaml:250 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [CheckBox_tr.xaml:74 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorHighlightColor (runtime) [CheckBox_tr.xaml:162 > Cany.xaml:513] |
| `CheckBoxCheckGlyphForegroundIndeterminatePressed` | followable | L: #B3FFFFFF [CheckBox_tr.xaml:251 > Cany.xaml:303 > Cany.xaml:217]; D: #80000000 [CheckBox_tr.xaml:75 > Cany.xaml:99 > Cany.xaml:13] | HC: SystemColorHighlightTextColor (runtime) [CheckBox_tr.xaml:163 > Cany.xaml:514] |
| `CheckBoxCheckGlyphForegroundIndeterminateDisabled` | followable | L: #FFFFFF [CheckBox_tr.xaml:252 > Cany.xaml:304 > Cany.xaml:218]; D: #87FFFFFF [CheckBox_tr.xaml:76 > Cany.xaml:100 > Cany.xaml:14] | HC: SystemColorWindowColor (runtime) [CheckBox_tr.xaml:164 > Cany.xaml:510] |
| `CheckBoxBorderThickness` | stated | L=D: 1 [CheckBox_tr.xaml:269] | HC: 1 [CheckBox_tr.xaml:269] |
| `CheckBoxSize` | stated | L=D: 20 [CheckBox_tr.xaml:270] | HC: 20 [CheckBox_tr.xaml:270] |
| `CheckBoxGlyphSize` | stated | L=D: 12 [CheckBox_tr.xaml:271] | HC: 12 [CheckBox_tr.xaml:271] |
| `CheckBoxHeight` | stated | L=D: 32 [CheckBox_tr.xaml:272] | HC: 32 [CheckBox_tr.xaml:272] |
| `CheckBoxMinWidth` | stated | L=D: 120 [CheckBox_tr.xaml:273] | HC: 120 [CheckBox_tr.xaml:273] |
| `CheckBoxPadding` | stated | L=D: 8,5,0,0 [CheckBox_tr.xaml:274] | HC: 8,5,0,0 [CheckBox_tr.xaml:274] |
| `CheckBoxFocusVisualMargin` | stated | L=D: -7,-3,-7,-3 [CheckBox_tr.xaml:275] | HC: -7,-3,-7,-3 [CheckBox_tr.xaml:275] |
| `CheckBoxCheckedGlyph` | stated | L=D: &#xE73E; [CheckBox_tr.xaml:276] | HC: &#xE73E; [CheckBox_tr.xaml:276] |
| `CheckBoxIndeterminateGlyph` | stated | L=D: &#xE9AE; [CheckBox_tr.xaml:277] | HC: &#xE9AE; [CheckBox_tr.xaml:277] |

Counts: CommonStyles/CheckBox_themeresources.xaml {'followable': 60, 'accent': 12, 'stated': 9}

### ToggleSwitch_themeresources.xaml

| Key | Class | L / D | HC |
|---|---|---|---|
| `ToggleSwitchOnStrokeThickness` | stated | L: 0 [ToggleSwitch_tr.xaml:125]; D: 0 [ToggleSwitch_tr.xaml:5] | HC: 1 [ToggleSwitch_tr.xaml:65] |
| `ToggleSwitchOuterBorderStrokeThickness` | stated | L: 1 [ToggleSwitch_tr.xaml:126]; D: 1 [ToggleSwitch_tr.xaml:6] | HC: 1 [ToggleSwitch_tr.xaml:66] |
| `ToggleSwitchContentForeground` | followable | L: #E4000000 [ToggleSwitch_tr.xaml:127 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ToggleSwitch_tr.xaml:7 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorWindowTextColor (runtime) [ToggleSwitch_tr.xaml:67 > Cany.xaml:509] |
| `ToggleSwitchContentForegroundDisabled` | followable | L: #5C000000 [ToggleSwitch_tr.xaml:128 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [ToggleSwitch_tr.xaml:8 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [ToggleSwitch_tr.xaml:68 > Cany.xaml:516] |
| `ToggleSwitchHeaderForeground` | followable | L: #E4000000 [ToggleSwitch_tr.xaml:129 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ToggleSwitch_tr.xaml:9 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorWindowTextColor (runtime) [ToggleSwitch_tr.xaml:69 > Cany.xaml:509] |
| `ToggleSwitchHeaderForegroundDisabled` | followable | L: #5C000000 [ToggleSwitch_tr.xaml:130 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [ToggleSwitch_tr.xaml:10 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [ToggleSwitch_tr.xaml:70 > Cany.xaml:516] |
| `ToggleSwitchContainerBackground` | followable | L: #00FFFFFF [ToggleSwitch_tr.xaml:131 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [ToggleSwitch_tr.xaml:11 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [ToggleSwitch_tr.xaml:71 > Common_tr.xaml:34] |
| `ToggleSwitchContainerBackgroundPointerOver` | followable | L: #00FFFFFF [ToggleSwitch_tr.xaml:132 > Cany.xaml:229]; D: #00FFFFFF [ToggleSwitch_tr.xaml:12 > Cany.xaml:25] | HC: #00000000 [ToggleSwitch_tr.xaml:72 > Common_tr.xaml:33] |
| `ToggleSwitchContainerBackgroundPressed` | followable | L: #00FFFFFF [ToggleSwitch_tr.xaml:133 > Cany.xaml:229]; D: #00FFFFFF [ToggleSwitch_tr.xaml:13 > Cany.xaml:25] | HC: #00000000 [ToggleSwitch_tr.xaml:73 > Common_tr.xaml:33] |
| `ToggleSwitchContainerBackgroundDisabled` | followable | L: #00FFFFFF [ToggleSwitch_tr.xaml:134 > Cany.xaml:229]; D: #00FFFFFF [ToggleSwitch_tr.xaml:14 > Cany.xaml:25] | HC: #00000000 [ToggleSwitch_tr.xaml:74 > Common_tr.xaml:33] |
| `ToggleSwitchFillOff` | followable | L: #06000000 [ToggleSwitch_tr.xaml:135 > Cany.xaml:320 > Cany.xaml:234]; D: #19000000 [ToggleSwitch_tr.xaml:15 > Cany.xaml:116 > Cany.xaml:30] | HC: SystemColorButtonFaceColor (runtime) [ToggleSwitch_tr.xaml:75 > Cany.xaml:511] |
| `ToggleSwitchFillOffPointerOver` | followable | L: #0F000000 [ToggleSwitch_tr.xaml:136 > Cany.xaml:235]; D: #0BFFFFFF [ToggleSwitch_tr.xaml:16 > Cany.xaml:31] | HC: SystemColorHighlightTextColor (runtime) [ToggleSwitch_tr.xaml:76] |
| `ToggleSwitchFillOffPressed` | followable | L: #18000000 [ToggleSwitch_tr.xaml:137 > Cany.xaml:236]; D: #12FFFFFF [ToggleSwitch_tr.xaml:17 > Cany.xaml:32] | HC: SystemColorHighlightTextColor (runtime) [ToggleSwitch_tr.xaml:77] |
| `ToggleSwitchFillOffDisabled` | followable | L: #00FFFFFF [ToggleSwitch_tr.xaml:138 > Cany.xaml:237]; D: #00FFFFFF [ToggleSwitch_tr.xaml:18 > Cany.xaml:33] | HC: SystemColorWindowColor (runtime) [ToggleSwitch_tr.xaml:78] |
| `ToggleSwitchStrokeOff` | followable | L: #72000000 [ToggleSwitch_tr.xaml:139 > Cany.xaml:342 > Cany.xaml:252]; D: #8BFFFFFF [ToggleSwitch_tr.xaml:19 > Cany.xaml:138 > Cany.xaml:48] | HC: SystemColorButtonTextColor (runtime) [ToggleSwitch_tr.xaml:79 > Cany.xaml:512] |
| `ToggleSwitchStrokeOffPointerOver` | followable | L: #72000000 [ToggleSwitch_tr.xaml:140 > Cany.xaml:252]; D: #8BFFFFFF [ToggleSwitch_tr.xaml:20 > Cany.xaml:48] | HC: SystemColorHighlightColor (runtime) [ToggleSwitch_tr.xaml:80] |
| `ToggleSwitchStrokeOffPressed` | followable | L: #72000000 [ToggleSwitch_tr.xaml:141 > Cany.xaml:252]; D: #8BFFFFFF [ToggleSwitch_tr.xaml:21 > Cany.xaml:48] | HC: SystemColorHighlightColor (runtime) [ToggleSwitch_tr.xaml:81] |
| `ToggleSwitchStrokeOffDisabled` | followable | L: #37000000 [ToggleSwitch_tr.xaml:142 > Cany.xaml:253]; D: #28FFFFFF [ToggleSwitch_tr.xaml:22 > Cany.xaml:49] | HC: SystemColorGrayTextColor (runtime) [ToggleSwitch_tr.xaml:82] |
| `ToggleSwitchFillOn` | accent | L: SystemAccentColorDark1 (runtime) [ToggleSwitch_tr.xaml:143 > Cany.xaml:329]; D: SystemAccentColorLight2 (runtime) [ToggleSwitch_tr.xaml:23 > Cany.xaml:125] | HC: SystemColorHighlightColor (runtime) [ToggleSwitch_tr.xaml:87 > Cany.xaml:513] |
| `ToggleSwitchFillOnPointerOver` | accent | L: SystemAccentColorDark1 (runtime) [ToggleSwitch_tr.xaml:144 > Cany.xaml:330]; D: SystemAccentColorLight2 (runtime) [ToggleSwitch_tr.xaml:24 > Cany.xaml:126] | HC: SystemColorButtonFaceColor (runtime) [ToggleSwitch_tr.xaml:88 > Cany.xaml:511] |
| `ToggleSwitchFillOnPressed` | accent | L: SystemAccentColorDark1 (runtime) [ToggleSwitch_tr.xaml:145 > Cany.xaml:331]; D: SystemAccentColorLight2 (runtime) [ToggleSwitch_tr.xaml:25 > Cany.xaml:127] | HC: SystemColorButtonFaceColor (runtime) [ToggleSwitch_tr.xaml:89 > Cany.xaml:511] |
| `ToggleSwitchFillOnDisabled` | followable | L: #37000000 [ToggleSwitch_tr.xaml:146 > Cany.xaml:332 > Cany.xaml:242]; D: #28FFFFFF [ToggleSwitch_tr.xaml:26 > Cany.xaml:128 > Cany.xaml:38] | HC: SystemColorWindowColor (runtime) [ToggleSwitch_tr.xaml:90 > Cany.xaml:510] |
| `ToggleSwitchStrokeOn` | accent | L: SystemAccentColorDark1 (runtime) [ToggleSwitch_tr.xaml:147 > Cany.xaml:329]; D: SystemAccentColorLight2 (runtime) [ToggleSwitch_tr.xaml:27 > Cany.xaml:125] | HC: SystemColorHighlightColor (runtime) [ToggleSwitch_tr.xaml:91 > Cany.xaml:513] |
| `ToggleSwitchStrokeOnPointerOver` | accent | L: SystemAccentColorDark1 (runtime) [ToggleSwitch_tr.xaml:148 > Cany.xaml:330]; D: SystemAccentColorLight2 (runtime) [ToggleSwitch_tr.xaml:28 > Cany.xaml:126] | HC: SystemColorButtonTextColor (runtime) [ToggleSwitch_tr.xaml:92 > Cany.xaml:512] |
| `ToggleSwitchStrokeOnPressed` | accent | L: SystemAccentColorDark1 (runtime) [ToggleSwitch_tr.xaml:149 > Cany.xaml:331]; D: SystemAccentColorLight2 (runtime) [ToggleSwitch_tr.xaml:29 > Cany.xaml:127] | HC: SystemColorButtonTextColor (runtime) [ToggleSwitch_tr.xaml:93 > Cany.xaml:512] |
| `ToggleSwitchStrokeOnDisabled` | followable | L: #37000000 [ToggleSwitch_tr.xaml:150 > Cany.xaml:332 > Cany.xaml:242]; D: #28FFFFFF [ToggleSwitch_tr.xaml:30 > Cany.xaml:128 > Cany.xaml:38] | HC: SystemColorGrayTextColor (runtime) [ToggleSwitch_tr.xaml:94 > Cany.xaml:516] |
| `ToggleSwitchKnobFillOff` | followable | L: #9E000000 [ToggleSwitch_tr.xaml:151 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [ToggleSwitch_tr.xaml:31 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorButtonTextColor (runtime) [ToggleSwitch_tr.xaml:83 > Cany.xaml:512] |
| `ToggleSwitchKnobFillOffPointerOver` | followable | L: #9E000000 [ToggleSwitch_tr.xaml:152 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [ToggleSwitch_tr.xaml:32 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightColor (runtime) [ToggleSwitch_tr.xaml:84 > Cany.xaml:513] |
| `ToggleSwitchKnobFillOffPressed` | followable | L: #9E000000 [ToggleSwitch_tr.xaml:153 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [ToggleSwitch_tr.xaml:33 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightColor (runtime) [ToggleSwitch_tr.xaml:85 > Cany.xaml:513] |
| `ToggleSwitchKnobFillOffDisabled` | followable | L: #5C000000 [ToggleSwitch_tr.xaml:154 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [ToggleSwitch_tr.xaml:34 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [ToggleSwitch_tr.xaml:86 > Cany.xaml:516] |
| `ToggleSwitchKnobFillOn` | followable | L: #FFFFFF [ToggleSwitch_tr.xaml:155 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [ToggleSwitch_tr.xaml:35 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorHighlightTextColor (runtime) [ToggleSwitch_tr.xaml:95 > Cany.xaml:514] |
| `ToggleSwitchKnobFillOnPointerOver` | followable | L: #FFFFFF [ToggleSwitch_tr.xaml:156 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [ToggleSwitch_tr.xaml:36 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorButtonTextColor (runtime) [ToggleSwitch_tr.xaml:96 > Cany.xaml:512] |
| `ToggleSwitchKnobFillOnPressed` | followable | L: #FFFFFF [ToggleSwitch_tr.xaml:157 > Cany.xaml:302 > Cany.xaml:216]; D: #000000 [ToggleSwitch_tr.xaml:37 > Cany.xaml:98 > Cany.xaml:12] | HC: SystemColorButtonTextColor (runtime) [ToggleSwitch_tr.xaml:97 > Cany.xaml:512] |
| `ToggleSwitchKnobFillOnDisabled` | followable | L: #FFFFFF [ToggleSwitch_tr.xaml:158 > Cany.xaml:304 > Cany.xaml:218]; D: #87FFFFFF [ToggleSwitch_tr.xaml:38 > Cany.xaml:100 > Cany.xaml:14] | HC: SystemColorGrayTextColor (runtime) [ToggleSwitch_tr.xaml:98 > Cany.xaml:516] |
| `ToggleSwitchKnobStrokeOn` | followable | L: gradient CircleElevationBorderBrush [ToggleSwitch_tr.xaml:159 > Cany.xaml:391]; D: gradient CircleElevationBorderBrush [ToggleSwitch_tr.xaml:39 > Cany.xaml:192] | HC: Transparent [ToggleSwitch_tr.xaml:99 > Common_tr.xaml:34] |
| `ToggleSwitchTopHeaderMargin` | stated | L=D: 0,0,0,4 [ToggleSwitch_tr.xaml:185] | HC: 0,0,0,4 [ToggleSwitch_tr.xaml:185] |
| `ToggleSwitchPreContentMargin` | stated | L=D: 10 [ToggleSwitch_tr.xaml:186] | HC: 10 [ToggleSwitch_tr.xaml:186] |
| `ToggleSwitchPostContentMargin` | stated | L=D: 10 [ToggleSwitch_tr.xaml:187] | HC: 10 [ToggleSwitch_tr.xaml:187] |
| `ToggleSwitchThemeMinWidth` | stated | L=D: 154 [ToggleSwitch_tr.xaml:188] | HC: 154 [ToggleSwitch_tr.xaml:188] |

Counts: CommonStyles/ToggleSwitch_themeresources.xaml {'stated': 6, 'followable': 27, 'accent': 6}

### Slider_themeresources.xaml

| Key | Class | L / D | HC |
|---|---|---|---|
| `SliderOutsideTickBarThemeHeight` | stated | L: 4 [Slider_tr.xaml:109]; D: 4 [Slider_tr.xaml:5] | HC: 4 [Slider_tr.xaml:80] |
| `SliderTrackThemeHeight` | stated | L: 4 [Slider_tr.xaml:110]; D: 4 [Slider_tr.xaml:6] | HC: 2 [Slider_tr.xaml:81] |
| `SliderBorderThemeThickness` | stated | L: 0 [Slider_tr.xaml:111]; D: 0 [Slider_tr.xaml:7] | HC: 1 [Slider_tr.xaml:82] |
| `SliderHeaderThemeMargin` | stated | L: 0,0,0,4 [Slider_tr.xaml:112]; D: 0,0,0,4 [Slider_tr.xaml:8] | HC: 0,0,0,4 [Slider_tr.xaml:83] |
| `SliderHeaderThemeFontWeight` | stated | L: Normal [Slider_tr.xaml:113]; D: Normal [Slider_tr.xaml:9] | HC: Normal [Slider_tr.xaml:84] |
| `SliderContainerBackgroundPointerOver` | followable | L: #00FFFFFF [Slider_tr.xaml:115 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [Slider_tr.xaml:10 > Cany.xaml:106 > Cany.xaml:20] | HC: Transparent [Slider_tr.xaml:58 > Common_tr.xaml:34] |
| `SliderContainerBackgroundPressed` | followable | L: #00FFFFFF [Slider_tr.xaml:116 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [Slider_tr.xaml:11 > Cany.xaml:106 > Cany.xaml:20] | HC: Transparent [Slider_tr.xaml:59 > Common_tr.xaml:34] |
| `SliderContainerBackgroundDisabled` | followable | L: #00FFFFFF [Slider_tr.xaml:117 > Cany.xaml:310 > Cany.xaml:224]; D: #00FFFFFF [Slider_tr.xaml:12 > Cany.xaml:106 > Cany.xaml:20] | HC: Transparent [Slider_tr.xaml:60 > Common_tr.xaml:34] |
| `SliderThumbBackground` | accent | L: SystemAccentColorDark1 (runtime) [Slider_tr.xaml:118 > Cany.xaml:329]; D: SystemAccentColorLight2 (runtime) [Slider_tr.xaml:14 > Cany.xaml:125] | HC: SystemColorButtonTextColor (runtime) [Slider_tr.xaml:61 > generic.xaml:2036] |
| `SliderThumbBackgroundPointerOver` | accent | L: SystemAccentColorDark1 (runtime) [Slider_tr.xaml:119 > Cany.xaml:330]; D: SystemAccentColorLight2 (runtime) [Slider_tr.xaml:15 > Cany.xaml:126] | HC: SystemColorHighlightColor (runtime) [Slider_tr.xaml:62 > Cany.xaml:513] |
| `SliderThumbBackgroundPressed` | accent | L: SystemAccentColorDark1 (runtime) [Slider_tr.xaml:120 > Cany.xaml:331]; D: SystemAccentColorLight2 (runtime) [Slider_tr.xaml:16 > Cany.xaml:127] | HC: SystemColorHotlightColor (runtime) [Slider_tr.xaml:63 > Cany.xaml:515] |
| `SliderThumbBackgroundDisabled` | followable | L: #37000000 [Slider_tr.xaml:121 > Cany.xaml:332 > Cany.xaml:242]; D: #28FFFFFF [Slider_tr.xaml:17 > Cany.xaml:128 > Cany.xaml:38] | HC: SystemColorGrayTextColor (runtime) [Slider_tr.xaml:64 > generic.xaml:2027] |
| `SliderThumbBorderBrush` | followable | L: gradient ControlElevationBorderBrush [Slider_tr.xaml:122 > Cany.xaml:382]; D: gradient ControlElevationBorderBrush [Slider_tr.xaml:18 > Cany.xaml:186] | HC: SystemColorButtonTextColor (runtime) [Slider_tr.xaml:65 > generic.xaml:2036] |
| `SliderOuterThumbBackground` | followable | L: #FFFFFF [Slider_tr.xaml:123 > Cany.xaml:314 > Cany.xaml:228]; D: #454545 [Slider_tr.xaml:19 > Cany.xaml:110 > Cany.xaml:24] | HC: SystemColorButtonTextColor (runtime) [Slider_tr.xaml:66 > generic.xaml:2036] |
| `SliderTrackFill` | followable | L: #72000000 [Slider_tr.xaml:124 > Cany.xaml:312 > Cany.xaml:226]; D: #8BFFFFFF [Slider_tr.xaml:20 > Cany.xaml:108 > Cany.xaml:22] | HC: SystemColorButtonTextColor (runtime) [Slider_tr.xaml:67 > generic.xaml:2043] |
| `SliderTrackFillPointerOver` | followable | L: #72000000 [Slider_tr.xaml:125 > Cany.xaml:312 > Cany.xaml:226]; D: #8BFFFFFF [Slider_tr.xaml:21 > Cany.xaml:108 > Cany.xaml:22] | HC: SystemColorButtonTextColor (runtime) [Slider_tr.xaml:68 > generic.xaml:2041] |
| `SliderTrackFillPressed` | followable | L: #72000000 [Slider_tr.xaml:126 > Cany.xaml:312 > Cany.xaml:226]; D: #8BFFFFFF [Slider_tr.xaml:22 > Cany.xaml:108 > Cany.xaml:22] | HC: SystemColorButtonTextColor (runtime) [Slider_tr.xaml:69 > generic.xaml:2043] |
| `SliderTrackFillDisabled` | followable | L: #51000000 [Slider_tr.xaml:127 > Cany.xaml:313 > Cany.xaml:227]; D: #3FFFFFFF [Slider_tr.xaml:23 > Cany.xaml:109 > Cany.xaml:23] | HC: SystemColorGrayTextColor (runtime) [Slider_tr.xaml:70 > generic.xaml:2027] |
| `SliderTrackValueFill` | accent | L: SystemAccentColorDark1 (runtime) [Slider_tr.xaml:128 > Cany.xaml:329]; D: SystemAccentColorLight2 (runtime) [Slider_tr.xaml:24 > Cany.xaml:125] | HC: SystemColorHighlightColor (runtime) [Slider_tr.xaml:71 > generic.xaml:2055] |
| `SliderTrackValueFillPointerOver` | accent | L: SystemAccentColorDark1 (runtime) [Slider_tr.xaml:129 > Cany.xaml:330]; D: SystemAccentColorLight2 (runtime) [Slider_tr.xaml:25 > Cany.xaml:126] | HC: SystemColorHighlightColor (runtime) [Slider_tr.xaml:72 > generic.xaml:2055] |
| `SliderTrackValueFillPressed` | accent | L: SystemAccentColorDark1 (runtime) [Slider_tr.xaml:130 > Cany.xaml:331]; D: SystemAccentColorLight2 (runtime) [Slider_tr.xaml:26 > Cany.xaml:127] | HC: SystemColorHighlightColor (runtime) [Slider_tr.xaml:73 > generic.xaml:2055] |
| `SliderTrackValueFillDisabled` | followable | L: #37000000 [Slider_tr.xaml:131 > Cany.xaml:332 > Cany.xaml:242]; D: #28FFFFFF [Slider_tr.xaml:27 > Cany.xaml:128 > Cany.xaml:38] | HC: SystemColorGrayTextColor (runtime) [Slider_tr.xaml:74 > generic.xaml:2027] |
| `SliderHeaderForeground` | followable | L: #E4000000 [Slider_tr.xaml:132 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [Slider_tr.xaml:28 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [Slider_tr.xaml:75 > generic.xaml:2039] |
| `SliderHeaderForegroundDisabled` | followable | L: #5C000000 [Slider_tr.xaml:133 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [Slider_tr.xaml:29 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [Slider_tr.xaml:76 > generic.xaml:2026] |
| `SliderTickBarFill` | followable | L: #72000000 [Slider_tr.xaml:134 > Cany.xaml:312 > Cany.xaml:226]; D: #8BFFFFFF [Slider_tr.xaml:30 > Cany.xaml:108 > Cany.xaml:22] | HC: SystemColorButtonTextColor (runtime) [Slider_tr.xaml:77 > generic.xaml:2043] |
| `SliderTickBarFillDisabled` | followable | L: #51000000 [Slider_tr.xaml:135 > Cany.xaml:313 > Cany.xaml:227]; D: #3FFFFFFF [Slider_tr.xaml:31 > Cany.xaml:109 > Cany.xaml:23] | HC: SystemColorGrayTextColor (runtime) [Slider_tr.xaml:78] |
| `SliderInlineTickBarFill` | followable | L: #FFFFFF [Slider_tr.xaml:136 > Cany.xaml:311 > Cany.xaml:225]; D: #B31E1E1E [Slider_tr.xaml:32 > Cany.xaml:107 > Cany.xaml:21] | HC: SystemColorButtonFaceColor (runtime) [Slider_tr.xaml:79 > generic.xaml:2005] |
| `SliderTopHeaderMargin` | stated | L=D: 0,0,0,4 [Slider_tr.xaml:161] | HC: 0,0,0,4 [Slider_tr.xaml:161] |
| `SliderTrackCornerRadius` | stated | L=D: 2 [Slider_tr.xaml:162] | HC: 2 [Slider_tr.xaml:162] |
| `SliderThumbCornerRadius` | stated | L=D: 10 [Slider_tr.xaml:163] | HC: 10 [Slider_tr.xaml:163] |
| `SliderPreContentMargin` | stated | L=D: 14 [Slider_tr.xaml:164] | HC: 14 [Slider_tr.xaml:164] |
| `SliderPostContentMargin` | stated | L=D: 14 [Slider_tr.xaml:165] | HC: 14 [Slider_tr.xaml:165] |
| `SliderInnerThumbHeight` | stated | L=D: 12 [Slider_tr.xaml:166] | HC: 12 [Slider_tr.xaml:166] |
| `SliderHorizontalHeight` | stated | L=D: 32 [Slider_tr.xaml:167] | HC: 32 [Slider_tr.xaml:167] |
| `SliderVerticalWidth` | stated | L=D: 32 [Slider_tr.xaml:168] | HC: 32 [Slider_tr.xaml:168] |
| `SliderHorizontalThumbWidth` | stated | L=D: 18 [Slider_tr.xaml:169] | HC: 18 [Slider_tr.xaml:169] |
| `SliderHorizontalThumbHeight` | stated | L=D: 18 [Slider_tr.xaml:170] | HC: 18 [Slider_tr.xaml:170] |
| `SliderVerticalThumbWidth` | stated | L=D: 18 [Slider_tr.xaml:171] | HC: 18 [Slider_tr.xaml:171] |
| `SliderVerticalThumbHeight` | stated | L=D: 18 [Slider_tr.xaml:172] | HC: 18 [Slider_tr.xaml:172] |
| `SliderInnerThumbWidth` | stated | L=D: 12 [Slider_tr.xaml:173] | HC: 12 [Slider_tr.xaml:173] |

Counts: CommonStyles/Slider_themeresources.xaml {'stated': 18, 'followable': 16, 'accent': 6}

### TextBox_themeresources.xaml

| Key | Class | L / D | HC |
|---|---|---|---|
| `TemporaryTextFillColorDisabled` | stated | L: #5C010101 [TextBox_tr.xaml:129]; D: #5DFEFEFE [TextBox_tr.xaml:22] | HC: #5C010101 [TextBox_tr.xaml:85] |
| `TextControlBackground` | followable | L: #B3FFFFFF [TextBox_tr.xaml:130 > Cany.xaml:305 > Cany.xaml:219]; D: #0FFFFFFF [TextBox_tr.xaml:23 > Cany.xaml:101 > Cany.xaml:15] | HC: TextControlBackground (runtime) [] |
| `TextControlBackgroundPointerOver` | followable | L: #80F9F9F9 [TextBox_tr.xaml:131 > Cany.xaml:306 > Cany.xaml:220]; D: #15FFFFFF [TextBox_tr.xaml:24 > Cany.xaml:102 > Cany.xaml:16] | HC: TextControlBackgroundPointerOver (runtime) [] |
| `TextControlBackgroundFocused` | followable | L: #FFFFFF [TextBox_tr.xaml:132 > Cany.xaml:311 > Cany.xaml:225]; D: #B31E1E1E [TextBox_tr.xaml:25 > Cany.xaml:107 > Cany.xaml:21] | HC: SystemColorButtonFaceColor (runtime) [TextBox_tr.xaml:86 > generic.xaml:2005] |
| `TextControlBackgroundDisabled` | followable | L: #4DF9F9F9 [TextBox_tr.xaml:133 > Cany.xaml:309 > Cany.xaml:223]; D: #0BFFFFFF [TextBox_tr.xaml:26 > Cany.xaml:105 > Cany.xaml:19] | HC: TextControlBackgroundDisabled (runtime) [] |
| `TextControlBorderBrush` | followable | L: gradient TextControlElevationBorderBrush [TextBox_tr.xaml:134 > TextBox_tr.xaml:155]; D: gradient TextControlElevationBorderBrush [TextBox_tr.xaml:27 > TextBox_tr.xaml:48] | HC: SystemColorButtonTextColor (runtime) [TextBox_tr.xaml:87 > generic.xaml:2041] |
| `TextControlBorderBrushPointerOver` | followable | L: gradient TextControlElevationBorderBrush [TextBox_tr.xaml:135 > TextBox_tr.xaml:155]; D: gradient TextControlElevationBorderBrush [TextBox_tr.xaml:28 > TextBox_tr.xaml:48] | HC: SystemColorHighlightColor (runtime) [TextBox_tr.xaml:88 > generic.xaml:2072] |
| `TextControlBorderBrushFocused` | followable | L: gradient TextControlElevationBorderFocusedBrush [TextBox_tr.xaml:136 > TextBox_tr.xaml:164]; D: gradient TextControlElevationBorderFocusedBrush [TextBox_tr.xaml:29 > TextBox_tr.xaml:57] | HC: TextControlBorderBrushFocused (runtime) [] |
| `TextControlBorderBrushDisabled` | followable | L: #0F000000 [TextBox_tr.xaml:137 > Cany.xaml:333 > Cany.xaml:243]; D: #12FFFFFF [TextBox_tr.xaml:30 > Cany.xaml:129 > Cany.xaml:39] | HC: TextControlBorderBrushDisabled (runtime) [] |
| `TextControlForeground` | followable | L: #E4000000 [TextBox_tr.xaml:138 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [TextBox_tr.xaml:31 > Cany.xaml:88 > Cany.xaml:5] | HC: TextControlForeground (runtime) [] |
| `TextControlForegroundPointerOver` | followable | L: #E4000000 [TextBox_tr.xaml:139 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [TextBox_tr.xaml:32 > Cany.xaml:88 > Cany.xaml:5] | HC: TextControlForegroundPointerOver (runtime) [] |
| `TextControlForegroundFocused` | followable | L: #E4000000 [TextBox_tr.xaml:140 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [TextBox_tr.xaml:33 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [TextBox_tr.xaml:90 > generic.xaml:2039] |
| `TextControlForegroundDisabled` | followable | L: #5C010101 [TextBox_tr.xaml:141 > TextBox_tr.xaml:129]; D: #5DFEFEFE [TextBox_tr.xaml:34 > TextBox_tr.xaml:22] | HC: TextControlForegroundDisabled (runtime) [] |
| `TextControlPlaceholderForeground` | followable | L: #9E000000 [TextBox_tr.xaml:142 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [TextBox_tr.xaml:35 > Cany.xaml:89 > Cany.xaml:6] | HC: TextControlPlaceholderForeground (runtime) [] |
| `TextControlPlaceholderForegroundPointerOver` | followable | L: #9E000000 [TextBox_tr.xaml:143 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [TextBox_tr.xaml:36 > Cany.xaml:89 > Cany.xaml:6] | HC: TextControlPlaceholderForegroundPointerOver (runtime) [] |
| `TextControlPlaceholderForegroundFocused` | followable | L: #9E000000 [TextBox_tr.xaml:144 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [TextBox_tr.xaml:37 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorButtonTextColor (runtime) [TextBox_tr.xaml:91 > generic.xaml:2043] |
| `TextControlPlaceholderForegroundDisabled` | followable | L: #5C000000 [TextBox_tr.xaml:145 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [TextBox_tr.xaml:38 > Cany.xaml:91 > Cany.xaml:8] | HC: TextControlPlaceholderForegroundDisabled (runtime) [] |
| `TextControlSelectionHighlightColor` | accent | L: SystemAccentColor (runtime) [TextBox_tr.xaml:146 > Cany.xaml:328]; D: SystemAccentColor (runtime) [TextBox_tr.xaml:39 > Cany.xaml:124] | HC: TextControlSelectionHighlightColor (runtime) [] |
| `TextControlButtonBackgroundPointerOver` | followable | L: #09000000 [TextBox_tr.xaml:147 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [TextBox_tr.xaml:40 > Cany.xaml:112 > Cany.xaml:26] | HC: TextControlButtonBackgroundPointerOver (runtime) [] |
| `TextControlButtonBackgroundPressed` | followable | L: #06000000 [TextBox_tr.xaml:148 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [TextBox_tr.xaml:41 > Cany.xaml:113 > Cany.xaml:27] | HC: TextControlButtonBackgroundPressed (runtime) [] |
| `TextControlButtonBorderBrush` | followable | L: #00FFFFFF [TextBox_tr.xaml:149 > Cany.xaml:224]; D: #00FFFFFF [TextBox_tr.xaml:42 > Cany.xaml:20] | HC: TextControlButtonBorderBrush (runtime) [] |
| `TextControlButtonBorderBrushPointerOver` | followable | L: #00FFFFFF [TextBox_tr.xaml:150 > Cany.xaml:224]; D: #00FFFFFF [TextBox_tr.xaml:43 > Cany.xaml:20] | HC: TextControlButtonBorderBrushPointerOver (runtime) [] |
| `TextControlButtonBorderBrushPressed` | followable | L: #00FFFFFF [TextBox_tr.xaml:151 > Cany.xaml:224]; D: #00FFFFFF [TextBox_tr.xaml:44 > Cany.xaml:20] | HC: TextControlButtonBorderBrushPressed (runtime) [] |
| `TextControlButtonForeground` | followable | L: #9E000000 [TextBox_tr.xaml:152 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [TextBox_tr.xaml:45 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorButtonTextColor (runtime) [TextBox_tr.xaml:89 > generic.xaml:2042] |
| `TextControlButtonForegroundPointerOver` | followable | L: #9E000000 [TextBox_tr.xaml:153 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [TextBox_tr.xaml:46 > Cany.xaml:89 > Cany.xaml:6] | HC: TextControlButtonForegroundPointerOver (runtime) [] |
| `TextControlButtonForegroundPressed` | followable | L: #72000000 [TextBox_tr.xaml:154 > Cany.xaml:294 > Cany.xaml:211]; D: #87FFFFFF [TextBox_tr.xaml:47 > Cany.xaml:90 > Cany.xaml:7] | HC: TextControlButtonForegroundPressed (runtime) [] |
| `TextControlElevationBorderBrush` | followable | L: gradient TextControlElevationBorderBrush [TextBox_tr.xaml:155]; D: gradient TextControlElevationBorderBrush [TextBox_tr.xaml:48] | HC: gradient TextControlElevationBorderBrush [TextBox_tr.xaml:92] |
| `TextControlElevationBorderFocusedBrush` | followable | L: gradient TextControlElevationBorderFocusedBrush [TextBox_tr.xaml:164]; D: gradient TextControlElevationBorderFocusedBrush [TextBox_tr.xaml:57] | HC: gradient TextControlElevationBorderFocusedBrush [TextBox_tr.xaml:101] |
| `TextBoxTopHeaderMargin` | stated | L=D: 0,0,0,8 [TextBox_tr.xaml:175] | HC: 0,0,0,8 [TextBox_tr.xaml:175] |
| `TextBoxInnerButtonMargin` | stated | L=D: 0,4,4,4 [TextBox_tr.xaml:176] | HC: 0,4,4,4 [TextBox_tr.xaml:176] |
| `TextBoxIconFontSize` | stated | L=D: 12 [TextBox_tr.xaml:177] | HC: 12 [TextBox_tr.xaml:177] |

Counts: CommonStyles/TextBox_themeresources.xaml {'stated': 4, 'followable': 26, 'accent': 1}

### ComboBox_themeresources.xaml

| Key | Class | L / D | HC |
|---|---|---|---|
| `ComboBoxItemForeground` | followable | L: #E4000000 [ComboBox/ComboBox_tr.xaml:215 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ComboBox/ComboBox_tr.xaml:5 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [ComboBox/ComboBox_tr.xaml:110 > generic.xaml:2039] |
| `ComboBoxItemForegroundPressed` | followable | L: #9E000000 [ComboBox/ComboBox_tr.xaml:216 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [ComboBox/ComboBox_tr.xaml:6 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [ComboBox/ComboBox_tr.xaml:111 > generic.xaml:2058] |
| `ComboBoxItemForegroundPointerOver` | followable | L: #E4000000 [ComboBox/ComboBox_tr.xaml:217 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ComboBox/ComboBox_tr.xaml:7 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [ComboBox/ComboBox_tr.xaml:112 > generic.xaml:2058] |
| `ComboBoxItemForegroundDisabled` | followable | L: #5C000000 [ComboBox/ComboBox_tr.xaml:218 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [ComboBox/ComboBox_tr.xaml:8 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [ComboBox/ComboBox_tr.xaml:113 > generic.xaml:2026] |
| `ComboBoxItemForegroundSelected` | followable | L: #E4000000 [ComboBox/ComboBox_tr.xaml:219 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ComboBox/ComboBox_tr.xaml:9 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [ComboBox/ComboBox_tr.xaml:114 > generic.xaml:2058] |
| `ComboBoxItemForegroundSelectedUnfocused` | followable | L: #E4000000 [ComboBox/ComboBox_tr.xaml:220 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ComboBox/ComboBox_tr.xaml:10 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [ComboBox/ComboBox_tr.xaml:115 > generic.xaml:2058] |
| `ComboBoxItemForegroundSelectedPressed` | followable | L: #9E000000 [ComboBox/ComboBox_tr.xaml:221 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [ComboBox/ComboBox_tr.xaml:11 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [ComboBox/ComboBox_tr.xaml:116 > generic.xaml:2058] |
| `ComboBoxItemForegroundSelectedPointerOver` | followable | L: #E4000000 [ComboBox/ComboBox_tr.xaml:222 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ComboBox/ComboBox_tr.xaml:12 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [ComboBox/ComboBox_tr.xaml:117 > generic.xaml:2058] |
| `ComboBoxItemForegroundSelectedDisabled` | followable | L: #5C000000 [ComboBox/ComboBox_tr.xaml:223 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [ComboBox/ComboBox_tr.xaml:13 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [ComboBox/ComboBox_tr.xaml:118 > generic.xaml:2026] |
| `ComboBoxItemBackground` | followable | L: #00FFFFFF [ComboBox/ComboBox_tr.xaml:224 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [ComboBox/ComboBox_tr.xaml:14 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [ComboBox/ComboBox_tr.xaml:119 > Common_tr.xaml:34] |
| `ComboBoxItemBackgroundPressed` | followable | L: #06000000 [ComboBox/ComboBox_tr.xaml:225 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [ComboBox/ComboBox_tr.xaml:15 > Cany.xaml:113 > Cany.xaml:27] | HC: SystemColorHighlightColor (runtime) [ComboBox/ComboBox_tr.xaml:120 > generic.xaml:2079] |
| `ComboBoxItemBackgroundPointerOver` | followable | L: #09000000 [ComboBox/ComboBox_tr.xaml:226 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [ComboBox/ComboBox_tr.xaml:16 > Cany.xaml:112 > Cany.xaml:26] | HC: SystemColorHighlightColor (runtime) [ComboBox/ComboBox_tr.xaml:121 > generic.xaml:2080] |
| `ComboBoxItemBackgroundDisabled` | followable | L: #00FFFFFF [ComboBox/ComboBox_tr.xaml:227 > Cany.xaml:318 > Cany.xaml:232]; D: #00FFFFFF [ComboBox/ComboBox_tr.xaml:17 > Cany.xaml:114 > Cany.xaml:28] | HC: Transparent [ComboBox/ComboBox_tr.xaml:122 > Common_tr.xaml:34] |
| `ComboBoxItemBackgroundSelected` | followable | L: #09000000 [ComboBox/ComboBox_tr.xaml:228 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [ComboBox/ComboBox_tr.xaml:18 > Cany.xaml:112 > Cany.xaml:26] | HC: SystemColorHighlightColor (runtime) [ComboBox/ComboBox_tr.xaml:123 > generic.xaml:2077] |
| `ComboBoxItemBackgroundSelectedUnfocused` | followable | L: #06000000 [ComboBox/ComboBox_tr.xaml:229 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [ComboBox/ComboBox_tr.xaml:19 > Cany.xaml:113 > Cany.xaml:27] | HC: SystemColorHighlightColor (runtime) [ComboBox/ComboBox_tr.xaml:124 > generic.xaml:2077] |
| `ComboBoxItemBackgroundSelectedPressed` | followable | L: #09000000 [ComboBox/ComboBox_tr.xaml:230 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [ComboBox/ComboBox_tr.xaml:20 > Cany.xaml:112 > Cany.xaml:26] | HC: SystemColorHighlightColor (runtime) [ComboBox/ComboBox_tr.xaml:125 > generic.xaml:2076] |
| `ComboBoxItemBackgroundSelectedPointerOver` | followable | L: #06000000 [ComboBox/ComboBox_tr.xaml:231 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [ComboBox/ComboBox_tr.xaml:21 > Cany.xaml:113 > Cany.xaml:27] | HC: SystemColorHighlightColor (runtime) [ComboBox/ComboBox_tr.xaml:126 > generic.xaml:2078] |
| `ComboBoxItemBackgroundSelectedDisabled` | followable | L: #09000000 [ComboBox/ComboBox_tr.xaml:232 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [ComboBox/ComboBox_tr.xaml:22 > Cany.xaml:112 > Cany.xaml:26] | HC: Transparent [ComboBox/ComboBox_tr.xaml:127 > Common_tr.xaml:34] |
| `ComboBoxItemBorderBrush` | followable | L: #00FFFFFF [ComboBox/ComboBox_tr.xaml:233 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [ComboBox/ComboBox_tr.xaml:23 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [ComboBox/ComboBox_tr.xaml:128 > Common_tr.xaml:34] |
| `ComboBoxItemBorderBrushPressed` | followable | L: #06000000 [ComboBox/ComboBox_tr.xaml:234 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [ComboBox/ComboBox_tr.xaml:24 > Cany.xaml:113 > Cany.xaml:27] | HC: Transparent [ComboBox/ComboBox_tr.xaml:129 > Common_tr.xaml:34] |
| `ComboBoxItemBorderBrushPointerOver` | followable | L: #09000000 [ComboBox/ComboBox_tr.xaml:235 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [ComboBox/ComboBox_tr.xaml:25 > Cany.xaml:112 > Cany.xaml:26] | HC: Transparent [ComboBox/ComboBox_tr.xaml:130 > Common_tr.xaml:34] |
| `ComboBoxItemBorderBrushDisabled` | followable | L: #00FFFFFF [ComboBox/ComboBox_tr.xaml:236 > Cany.xaml:318 > Cany.xaml:232]; D: #00FFFFFF [ComboBox/ComboBox_tr.xaml:26 > Cany.xaml:114 > Cany.xaml:28] | HC: Transparent [ComboBox/ComboBox_tr.xaml:131 > Common_tr.xaml:34] |
| `ComboBoxItemBorderBrushSelected` | followable | L: #09000000 [ComboBox/ComboBox_tr.xaml:237 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [ComboBox/ComboBox_tr.xaml:27 > Cany.xaml:112 > Cany.xaml:26] | HC: Transparent [ComboBox/ComboBox_tr.xaml:132 > Common_tr.xaml:34] |
| `ComboBoxItemBorderBrushSelectedUnfocused` | followable | L: #00FFFFFF [ComboBox/ComboBox_tr.xaml:238 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [ComboBox/ComboBox_tr.xaml:28 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [ComboBox/ComboBox_tr.xaml:133 > Common_tr.xaml:34] |
| `ComboBoxItemBorderBrushSelectedPressed` | followable | L: #09000000 [ComboBox/ComboBox_tr.xaml:239 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [ComboBox/ComboBox_tr.xaml:29 > Cany.xaml:112 > Cany.xaml:26] | HC: Transparent [ComboBox/ComboBox_tr.xaml:134 > Common_tr.xaml:34] |
| `ComboBoxItemBorderBrushSelectedPointerOver` | followable | L: #06000000 [ComboBox/ComboBox_tr.xaml:240 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [ComboBox/ComboBox_tr.xaml:30 > Cany.xaml:113 > Cany.xaml:27] | HC: Transparent [ComboBox/ComboBox_tr.xaml:135 > Common_tr.xaml:34] |
| `ComboBoxItemBorderBrushSelectedDisabled` | followable | L: #09000000 [ComboBox/ComboBox_tr.xaml:241 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [ComboBox/ComboBox_tr.xaml:31 > Cany.xaml:112 > Cany.xaml:26] | HC: Transparent [ComboBox/ComboBox_tr.xaml:136 > Common_tr.xaml:34] |
| `ComboBoxBackground` | followable | L: #B3FFFFFF [ComboBox/ComboBox_tr.xaml:242 > Cany.xaml:305 > Cany.xaml:219]; D: #0FFFFFFF [ComboBox/ComboBox_tr.xaml:32 > Cany.xaml:101 > Cany.xaml:15] | HC: SystemColorButtonFaceColor (runtime) [ComboBox/ComboBox_tr.xaml:137 > generic.xaml:2008] |
| `ComboBoxBackgroundPointerOver` | followable | L: #80F9F9F9 [ComboBox/ComboBox_tr.xaml:243 > Cany.xaml:306 > Cany.xaml:220]; D: #15FFFFFF [ComboBox/ComboBox_tr.xaml:33 > Cany.xaml:102 > Cany.xaml:16] | HC: SystemColorWindowColor (runtime) [ComboBox/ComboBox_tr.xaml:138 > generic.xaml:2087] |
| `ComboBoxBackgroundPressed` | followable | L: #4DF9F9F9 [ComboBox/ComboBox_tr.xaml:244 > Cany.xaml:307 > Cany.xaml:221]; D: #08FFFFFF [ComboBox/ComboBox_tr.xaml:34 > Cany.xaml:103 > Cany.xaml:17] | HC: SystemColorButtonFaceColor (runtime) [ComboBox/ComboBox_tr.xaml:139 > generic.xaml:2022] |
| `ComboBoxBackgroundDisabled` | followable | L: #4DF9F9F9 [ComboBox/ComboBox_tr.xaml:245 > Cany.xaml:309 > Cany.xaml:223]; D: #0BFFFFFF [ComboBox/ComboBox_tr.xaml:35 > Cany.xaml:105 > Cany.xaml:19] | HC: SystemColorButtonFaceColor (runtime) [ComboBox/ComboBox_tr.xaml:140 > generic.xaml:2010] |
| `ComboBoxBackgroundUnfocused` | followable | L: #B3FFFFFF [ComboBox/ComboBox_tr.xaml:246 > Cany.xaml:305 > Cany.xaml:219]; D: #0FFFFFFF [ComboBox/ComboBox_tr.xaml:36 > Cany.xaml:101 > Cany.xaml:15] | HC: SystemColorHighlightColor (runtime) [ComboBox/ComboBox_tr.xaml:141 > generic.xaml:2077] |
| `ComboBoxBackgroundFocused` | followable | L: #B3FFFFFF [ComboBox/ComboBox_tr.xaml:247 > Cany.xaml:305 > Cany.xaml:219]; D: #0FFFFFFF [ComboBox/ComboBox_tr.xaml:37 > Cany.xaml:101 > Cany.xaml:15] | HC: SystemColorHighlightColor (runtime) [ComboBox/ComboBox_tr.xaml:142 > ComboBox/ComboBox_tr.xaml:141 > generic.xaml:2077] |
| `ComboBoxBackgroundBorderBrushFocused` | followable | L: #E4000000 [ComboBox/ComboBox_tr.xaml:248 > Cany.xaml:348 > Cany.xaml:258]; D: #FFFFFF [ComboBox/ComboBox_tr.xaml:38 > Cany.xaml:144 > Cany.xaml:54] | HC: SystemColorHighlightColor (runtime) [ComboBox/ComboBox_tr.xaml:143 > generic.xaml:2082] |
| `ComboBoxBackgroundBorderBrushUnfocused` | followable | L: #0F000000 [ComboBox/ComboBox_tr.xaml:249 > Cany.xaml:333 > Cany.xaml:243]; D: #12FFFFFF [ComboBox/ComboBox_tr.xaml:39 > Cany.xaml:129 > Cany.xaml:39] | HC: SystemColorHighlightColor (runtime) [ComboBox/ComboBox_tr.xaml:144 > generic.xaml:2073] |
| `ComboBoxForeground` | followable | L: #E4000000 [ComboBox/ComboBox_tr.xaml:250 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ComboBox/ComboBox_tr.xaml:40 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [ComboBox/ComboBox_tr.xaml:145 > generic.xaml:2039] |
| `ComboBoxForegroundPointerOver` | followable | L: #E4000000 [ComboBox/ComboBox_tr.xaml:251 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ComboBox/ComboBox_tr.xaml:41 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [ComboBox/ComboBox_tr.xaml:146 > generic.xaml:2039] |
| `ComboBoxForegroundPressed` | followable | L: #9E000000 [ComboBox/ComboBox_tr.xaml:252 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [ComboBox/ComboBox_tr.xaml:42 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorButtonTextColor (runtime) [ComboBox/ComboBox_tr.xaml:147 > generic.xaml:2039] |
| `ComboBoxForegroundDisabled` | followable | L: #5C000000 [ComboBox/ComboBox_tr.xaml:253 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [ComboBox/ComboBox_tr.xaml:43 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [ComboBox/ComboBox_tr.xaml:148 > generic.xaml:2026] |
| `ComboBoxForegroundFocused` | followable | L: #E4000000 [ComboBox/ComboBox_tr.xaml:254 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ComboBox/ComboBox_tr.xaml:44 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [ComboBox/ComboBox_tr.xaml:149 > generic.xaml:2039] |
| `ComboBoxForegroundFocusedPressed` | followable | L: #E4000000 [ComboBox/ComboBox_tr.xaml:255 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ComboBox/ComboBox_tr.xaml:45 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [ComboBox/ComboBox_tr.xaml:150 > generic.xaml:2039] |
| `ComboBoxHeaderForeground` | followable | L: #E4000000 [ComboBox/ComboBox_tr.xaml:256 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ComboBox/ComboBox_tr.xaml:46 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [ComboBox/ComboBox_tr.xaml:151 > generic.xaml:2039] |
| `ComboBoxHeaderForegroundDisabled` | followable | L: #5C000000 [ComboBox/ComboBox_tr.xaml:257 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [ComboBox/ComboBox_tr.xaml:47 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [ComboBox/ComboBox_tr.xaml:152 > generic.xaml:2026] |
| `ComboBoxPlaceHolderForeground` | followable | L: #9E000000 [ComboBox/ComboBox_tr.xaml:258 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [ComboBox/ComboBox_tr.xaml:48 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorWindowTextColor (runtime) [ComboBox/ComboBox_tr.xaml:153 > generic.xaml:2097] |
| `ComboBoxPlaceHolderForegroundPointerOver` | followable | L: #9E000000 [ComboBox/ComboBox_tr.xaml:259 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [ComboBox/ComboBox_tr.xaml:49 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorWindowTextColor (runtime) [ComboBox/ComboBox_tr.xaml:154 > generic.xaml:2097] |
| `ComboBoxPlaceHolderForegroundPressed` | followable | L: #72000000 [ComboBox/ComboBox_tr.xaml:260 > Cany.xaml:294 > Cany.xaml:211]; D: #87FFFFFF [ComboBox/ComboBox_tr.xaml:50 > Cany.xaml:90 > Cany.xaml:7] | HC: SystemColorWindowTextColor (runtime) [ComboBox/ComboBox_tr.xaml:155 > generic.xaml:2097] |
| `ComboBoxPlaceHolderForegroundDisabled` | followable | L: #5C000000 [ComboBox/ComboBox_tr.xaml:261 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [ComboBox/ComboBox_tr.xaml:51 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [ComboBox/ComboBox_tr.xaml:156 > generic.xaml:2028] |
| `ComboBoxPlaceHolderForegroundFocused` | followable | L: #9E000000 [ComboBox/ComboBox_tr.xaml:262 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [ComboBox/ComboBox_tr.xaml:52 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorWindowTextColor (runtime) [ComboBox/ComboBox_tr.xaml:157 > generic.xaml:2097] |
| `ComboBoxPlaceHolderForegroundFocusedPressed` | followable | L: #9E000000 [ComboBox/ComboBox_tr.xaml:263 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [ComboBox/ComboBox_tr.xaml:53 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorWindowTextColor (runtime) [ComboBox/ComboBox_tr.xaml:158 > generic.xaml:2097] |
| `ComboBoxBorderBrush` | followable | L: gradient ControlElevationBorderBrush [ComboBox/ComboBox_tr.xaml:264 > Cany.xaml:382]; D: gradient ControlElevationBorderBrush [ComboBox/ComboBox_tr.xaml:54 > Cany.xaml:186] | HC: SystemColorButtonTextColor (runtime) [ComboBox/ComboBox_tr.xaml:159 > generic.xaml:2041] |
| `ComboBoxBorderBrushPointerOver` | followable | L: gradient ControlElevationBorderBrush [ComboBox/ComboBox_tr.xaml:265 > Cany.xaml:382]; D: gradient ControlElevationBorderBrush [ComboBox/ComboBox_tr.xaml:55 > Cany.xaml:186] | HC: SystemColorHighlightColor (runtime) [ComboBox/ComboBox_tr.xaml:160 > generic.xaml:2072] |
| `ComboBoxBorderBrushPressed` | followable | L: #0F000000 [ComboBox/ComboBox_tr.xaml:266 > Cany.xaml:333 > Cany.xaml:243]; D: #12FFFFFF [ComboBox/ComboBox_tr.xaml:56 > Cany.xaml:129 > Cany.xaml:39] | HC: SystemColorHighlightColor (runtime) [ComboBox/ComboBox_tr.xaml:161 > generic.xaml:2071] |
| `ComboBoxBorderBrushDisabled` | followable | L: #0F000000 [ComboBox/ComboBox_tr.xaml:267 > Cany.xaml:333 > Cany.xaml:243]; D: #12FFFFFF [ComboBox/ComboBox_tr.xaml:57 > Cany.xaml:129 > Cany.xaml:39] | HC: SystemColorGrayTextColor (runtime) [ComboBox/ComboBox_tr.xaml:162 > generic.xaml:2025] |
| `ComboBoxDropDownGlyphForeground` | followable | L: #9E000000 [ComboBox/ComboBox_tr.xaml:268 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [ComboBox/ComboBox_tr.xaml:58 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorButtonTextColor (runtime) [ComboBox/ComboBox_tr.xaml:163 > generic.xaml:2042] |
| `ComboBoxDropDownGlyphForegroundDisabled` | followable | L: #5C000000 [ComboBox/ComboBox_tr.xaml:269 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [ComboBox/ComboBox_tr.xaml:59 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [ComboBox/ComboBox_tr.xaml:164 > generic.xaml:2026] |
| `ComboBoxDropDownGlyphForegroundFocused` | followable | L: #9E000000 [ComboBox/ComboBox_tr.xaml:270 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [ComboBox/ComboBox_tr.xaml:60 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorButtonTextColor (runtime) [ComboBox/ComboBox_tr.xaml:165 > generic.xaml:2039] |
| `ComboBoxDropDownGlyphForegroundFocusedPressed` | followable | L: #CC000000 [ComboBox/ComboBox_tr.xaml:271 > generic.xaml:4210 > generic.xaml:4135]; D: #CCFFFFFF [ComboBox/ComboBox_tr.xaml:61 > generic.xaml:285 > generic.xaml:210] | HC: SystemColorButtonTextColor (runtime) [ComboBox/ComboBox_tr.xaml:166 > generic.xaml:2039] |
| `ComboBoxDropDownForeground` | followable | L: #E4000000 [ComboBox/ComboBox_tr.xaml:272 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ComboBox/ComboBox_tr.xaml:62 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [ComboBox/ComboBox_tr.xaml:167 > generic.xaml:2039] |
| `ComboBoxDropDownBackground` | followable | L: #FCFCFC opacity 0.0 [ComboBox/ComboBox_tr.xaml:273 > Materials/Acrylic/AcrylicBrush_tr.xaml:96]; D: #2C2C2C opacity 0.15 [ComboBox/ComboBox_tr.xaml:63 > Materials/Acrylic/AcrylicBrush_tr.xaml:44] | HC: SystemColorButtonFaceColor (runtime) [ComboBox/ComboBox_tr.xaml:168 > generic.xaml:2019] |
| `ComboBoxDropDownBorderBrush` | followable | L: #0F000000 [ComboBox/ComboBox_tr.xaml:274 > Cany.xaml:345 > Cany.xaml:255]; D: #33000000 [ComboBox/ComboBox_tr.xaml:64 > Cany.xaml:141 > Cany.xaml:51] | HC: SystemColorButtonTextColor (runtime) [ComboBox/ComboBox_tr.xaml:169 > generic.xaml:2045] |
| `ComboBoxLightDismissOverlayBackground` | followable | L: #99FFFFFF [ComboBox/ComboBox_tr.xaml:275 > generic.xaml:4238 > generic.xaml:4129]; D: #99000000 [ComboBox/ComboBox_tr.xaml:65 > generic.xaml:313 > generic.xaml:204] | HC: SystemColorWindowColor (runtime) [ComboBox/ComboBox_tr.xaml:170 > generic.xaml:2089] |
| `ComboBoxDropDownBackgroundPointerOver` | followable | L: #09000000 [ComboBox/ComboBox_tr.xaml:311 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [ComboBox/ComboBox_tr.xaml:101 > Cany.xaml:112 > Cany.xaml:26] | HC: SystemColorWindowColor (runtime) [ComboBox/ComboBox_tr.xaml:206 > generic.xaml:2087] |
| `ComboBoxDropDownBackgroundPointerPressed` | followable | L: #06000000 [ComboBox/ComboBox_tr.xaml:312 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [ComboBox/ComboBox_tr.xaml:102 > Cany.xaml:113 > Cany.xaml:27] | HC: SystemColorButtonFaceColor (runtime) [ComboBox/ComboBox_tr.xaml:207 > generic.xaml:2022] |
| `ComboBoxFocusedDropDownBackgroundPointerOver` | followable | L: #4DF9F9F9 [ComboBox/ComboBox_tr.xaml:313 > Cany.xaml:307 > Cany.xaml:221]; D: #08FFFFFF [ComboBox/ComboBox_tr.xaml:103 > Cany.xaml:103 > Cany.xaml:17] | HC: SystemColorWindowColor (runtime) [ComboBox/ComboBox_tr.xaml:208 > generic.xaml:2087] |
| `ComboBoxFocusedDropDownBackgroundPointerPressed` | followable | L: #18000000 [ComboBox/ComboBox_tr.xaml:314 > Cany.xaml:322 > Cany.xaml:236]; D: #12FFFFFF [ComboBox/ComboBox_tr.xaml:104 > Cany.xaml:118 > Cany.xaml:32] | HC: SystemColorButtonFaceColor (runtime) [ComboBox/ComboBox_tr.xaml:209 > generic.xaml:2022] |
| `ComboBoxEditableDropDownGlyphForeground` | followable | L: #9E000000 [ComboBox/ComboBox_tr.xaml:315 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [ComboBox/ComboBox_tr.xaml:105 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorButtonTextColor (runtime) [ComboBox/ComboBox_tr.xaml:210 > generic.xaml:2042] |
| `ComboBoxItemPillFillBrush` | accent | L: SystemAccentColorDark1 (runtime) [ComboBox/ComboBox_tr.xaml:316 > Cany.xaml:329]; D: SystemAccentColorLight2 (runtime) [ComboBox/ComboBox_tr.xaml:106 > Cany.xaml:125] | HC: SystemColorHighlightColor (runtime) [ComboBox/ComboBox_tr.xaml:211 > generic.xaml:2077] |
| `ComboBoxDropdownBorderPadding` | stated | L: 0 [ComboBox/ComboBox_tr.xaml:317]; D: 0 [ComboBox/ComboBox_tr.xaml:107] | HC: 0 [ComboBox/ComboBox_tr.xaml:212] |
| `ComboBoxArrowThemeFontSize` | stated | L=D: 21 [ComboBox/ComboBox_tr.xaml:320] | HC: 21 [ComboBox/ComboBox_tr.xaml:320] |
| `ComboBoxThemeMinWidth` | stated | L=D: 64 [ComboBox/ComboBox_tr.xaml:321] | HC: 64 [ComboBox/ComboBox_tr.xaml:321] |
| `ComboBoxPopupThemeMinWidth` | stated | L=D: 80 [ComboBox/ComboBox_tr.xaml:322] | HC: 80 [ComboBox/ComboBox_tr.xaml:322] |
| `ComboBoxPopupThemeTouchMinWidth` | stated | L=D: 240 [ComboBox/ComboBox_tr.xaml:323] | HC: 240 [ComboBox/ComboBox_tr.xaml:323] |
| `ComboBoxItemPillHeight` | stated | L=D: 16 [ComboBox/ComboBox_tr.xaml:324] | HC: 16 [ComboBox/ComboBox_tr.xaml:324] |
| `ComboBoxItemPillWidth` | stated | L=D: 3 [ComboBox/ComboBox_tr.xaml:325] | HC: 3 [ComboBox/ComboBox_tr.xaml:325] |
| `ComboBoxItemPillMinScale` | stated | L=D: 0.625 [ComboBox/ComboBox_tr.xaml:326] | HC: 0.625 [ComboBox/ComboBox_tr.xaml:326] |
| `ComboBoxMinHeight` | stated | L=D: 32 [ComboBox/ComboBox_tr.xaml:327] | HC: 32 [ComboBox/ComboBox_tr.xaml:327] |
| `ComboBoxPopupMaxNumberOfItems` | stated | L=D: 15 [ComboBox/ComboBox_tr.xaml:328] | HC: 15 [ComboBox/ComboBox_tr.xaml:328] |
| `ComboBoxPopupMaxNumberOfItemsThatCanBeShownOnOneSide` | stated | L=D: 7 [ComboBox/ComboBox_tr.xaml:329] | HC: 7 [ComboBox/ComboBox_tr.xaml:329] |
| `ComboBoxItemScaleAnimationDuration` | stated | L=D: 00:00:00.167 [ComboBox/ComboBox_tr.xaml:330] | HC: 00:00:00.167 [ComboBox/ComboBox_tr.xaml:330] |
| `ComboBoxBorderThemeThickness` | stated | L=D: 1 [ComboBox/ComboBox_tr.xaml:331] | HC: 1 [ComboBox/ComboBox_tr.xaml:331] |
| `ComboBoxDropdownBorderThickness` | stated | L=D: 1 [ComboBox/ComboBox_tr.xaml:332] | HC: 1 [ComboBox/ComboBox_tr.xaml:332] |
| `ComboBoxHeaderThemeMargin` | stated | L=D: 0,0,0,4 [ComboBox/ComboBox_tr.xaml:333] | HC: 0,0,0,4 [ComboBox/ComboBox_tr.xaml:333] |
| `ComboBoxPopupBorderThemeThickness` | stated | L=D: 2 [ComboBox/ComboBox_tr.xaml:334] | HC: 2 [ComboBox/ComboBox_tr.xaml:334] |
| `ComboBoxItemThemePadding` | stated | L=D: 11,5,11,7 [ComboBox/ComboBox_tr.xaml:335] | HC: 11,5,11,7 [ComboBox/ComboBox_tr.xaml:335] |
| `ComboBoxItemThemeTouchPadding` | stated | L=D: 11,11,11,13 [ComboBox/ComboBox_tr.xaml:336] | HC: 11,11,11,13 [ComboBox/ComboBox_tr.xaml:336] |
| `ComboBoxItemThemeGameControllerPadding` | stated | L=D: 11,11,11,13 [ComboBox/ComboBox_tr.xaml:337] | HC: 11,11,11,13 [ComboBox/ComboBox_tr.xaml:337] |
| `ComboBoxBackgroundBorderThicknessFocused` | stated | L=D: 2 [ComboBox/ComboBox_tr.xaml:338] | HC: 2 [ComboBox/ComboBox_tr.xaml:338] |
| `ComboBoxDropdownContentMargin` | stated | L=D: 0,4 [ComboBox/ComboBox_tr.xaml:339] | HC: 0,4 [ComboBox/ComboBox_tr.xaml:339] |
| `ComboBoxTopHeaderMargin` | stated | L=D: 0,0,0,8 [ComboBox/ComboBox_tr.xaml:340] | HC: 0,0,0,8 [ComboBox/ComboBox_tr.xaml:340] |
| `ComboBoxPadding` | stated | L=D: 12,5,0,7 [ComboBox/ComboBox_tr.xaml:341] | HC: 12,5,0,7 [ComboBox/ComboBox_tr.xaml:341] |
| `ComboBoxEditableTextPadding` | stated | L=D: 11,5,38,6 [ComboBox/ComboBox_tr.xaml:342] | HC: 11,5,38,6 [ComboBox/ComboBox_tr.xaml:342] |
| `ComboBoxHiglightBorderCornerRadius` | stated | L=D: 7 [ComboBox/ComboBox_tr.xaml:343] | HC: 7 [ComboBox/ComboBox_tr.xaml:343] |
| `ComboBoxDropDownButtonBackgroundCornerRadius` | stated | L=D: 4 [ComboBox/ComboBox_tr.xaml:344] | HC: 4 [ComboBox/ComboBox_tr.xaml:344] |
| `ComboBoxItemCornerRadius` | stated | L=D: 3 [ComboBox/ComboBox_tr.xaml:345] | HC: 3 [ComboBox/ComboBox_tr.xaml:345] |
| `ComboBoxItemPillCornerRadius` | stated | L=D: 1.5 [ComboBox/ComboBox_tr.xaml:346] | HC: 1.5 [ComboBox/ComboBox_tr.xaml:346] |
| `ComboBoxHeaderThemeFontWeight` | stated | L=D: Normal [ComboBox/ComboBox_tr.xaml:347] | HC: Normal [ComboBox/ComboBox_tr.xaml:347] |
| `ComboBoxPlaceholderTextThemeFontWeight` | stated | L=D: SemiLight [ComboBox/ComboBox_tr.xaml:348] | HC: SemiLight [ComboBox/ComboBox_tr.xaml:348] |

Counts: ComboBox/ComboBox_themeresources.xaml {'followable': 66, 'accent': 1, 'stated': 30}

### MenuFlyout_themeresources.xaml

| Key | Class | L / D | HC |
|---|---|---|---|
| `MenuFlyoutSeparatorBackground` | followable | L: #0F000000 [MenuFlyout_tr.xaml:171 > Cany.xaml:347 > Cany.xaml:257]; D: #15FFFFFF [MenuFlyout_tr.xaml:5 > Cany.xaml:143 > Cany.xaml:53] | HC: SystemColorWindowTextColor (runtime) [MenuFlyout_tr.xaml:88 > Cany.xaml:509] |
| `MenuFlyoutItemBackground` | followable | L: #00FFFFFF [MenuFlyout_tr.xaml:172 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [MenuFlyout_tr.xaml:6 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [MenuFlyout_tr.xaml:89 > Common_tr.xaml:34] |
| `MenuFlyoutItemBackgroundPointerOver` | followable | L: #09000000 [MenuFlyout_tr.xaml:173 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [MenuFlyout_tr.xaml:7 > Cany.xaml:112 > Cany.xaml:26] | HC: SystemColorHighlightColor (runtime) [MenuFlyout_tr.xaml:90 > Cany.xaml:513] |
| `MenuFlyoutItemBackgroundPressed` | followable | L: #06000000 [MenuFlyout_tr.xaml:174 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [MenuFlyout_tr.xaml:8 > Cany.xaml:113 > Cany.xaml:27] | HC: SystemColorHighlightColor (runtime) [MenuFlyout_tr.xaml:91 > Cany.xaml:513] |
| `MenuFlyoutItemBackgroundDisabled` | followable | L: #00FFFFFF [MenuFlyout_tr.xaml:175 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [MenuFlyout_tr.xaml:9 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [MenuFlyout_tr.xaml:92 > Common_tr.xaml:34] |
| `MenuFlyoutItemBackgroundBrush` | followable | L: #00FFFFFF [MenuFlyout_tr.xaml:176 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [MenuFlyout_tr.xaml:10 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [MenuFlyout_tr.xaml:93 > Common_tr.xaml:34] |
| `MenuFlyoutItemForeground` | followable | L: #E4000000 [MenuFlyout_tr.xaml:177 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [MenuFlyout_tr.xaml:11 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [MenuFlyout_tr.xaml:94 > Cany.xaml:512] |
| `MenuFlyoutItemForegroundPointerOver` | followable | L: #E4000000 [MenuFlyout_tr.xaml:178 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [MenuFlyout_tr.xaml:12 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [MenuFlyout_tr.xaml:95 > Cany.xaml:514] |
| `MenuFlyoutItemForegroundPressed` | followable | L: #E4000000 [MenuFlyout_tr.xaml:179 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [MenuFlyout_tr.xaml:13 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [MenuFlyout_tr.xaml:96 > Cany.xaml:514] |
| `MenuFlyoutItemForegroundDisabled` | followable | L: #5C000000 [MenuFlyout_tr.xaml:180 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [MenuFlyout_tr.xaml:14 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [MenuFlyout_tr.xaml:97 > Cany.xaml:516] |
| `MenuFlyoutSubItemBackground` | followable | L: #00FFFFFF [MenuFlyout_tr.xaml:181 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [MenuFlyout_tr.xaml:15 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [MenuFlyout_tr.xaml:98 > Common_tr.xaml:34] |
| `MenuFlyoutSubItemBackgroundPointerOver` | followable | L: #09000000 [MenuFlyout_tr.xaml:182 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [MenuFlyout_tr.xaml:16 > Cany.xaml:112 > Cany.xaml:26] | HC: SystemColorHighlightColor (runtime) [MenuFlyout_tr.xaml:99 > Cany.xaml:513] |
| `MenuFlyoutSubItemBackgroundPressed` | followable | L: #06000000 [MenuFlyout_tr.xaml:183 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [MenuFlyout_tr.xaml:17 > Cany.xaml:113 > Cany.xaml:27] | HC: SystemColorHighlightColor (runtime) [MenuFlyout_tr.xaml:100 > Cany.xaml:513] |
| `MenuFlyoutSubItemBackgroundSubMenuOpened` | followable | L: #09000000 [MenuFlyout_tr.xaml:184 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [MenuFlyout_tr.xaml:18 > Cany.xaml:112 > Cany.xaml:26] | HC: Transparent [MenuFlyout_tr.xaml:101 > Common_tr.xaml:34] |
| `MenuFlyoutSubItemBackgroundDisabled` | followable | L: #00FFFFFF [MenuFlyout_tr.xaml:185 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [MenuFlyout_tr.xaml:19 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [MenuFlyout_tr.xaml:102 > Common_tr.xaml:34] |
| `MenuFlyoutSubItemBackgroundBrush` | followable | L: #00FFFFFF [MenuFlyout_tr.xaml:186 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [MenuFlyout_tr.xaml:20 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [MenuFlyout_tr.xaml:103 > Common_tr.xaml:34] |
| `MenuFlyoutSubItemForeground` | followable | L: #E4000000 [MenuFlyout_tr.xaml:187 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [MenuFlyout_tr.xaml:21 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [MenuFlyout_tr.xaml:104 > Cany.xaml:512] |
| `MenuFlyoutSubItemForegroundPointerOver` | followable | L: #E4000000 [MenuFlyout_tr.xaml:188 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [MenuFlyout_tr.xaml:22 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [MenuFlyout_tr.xaml:105 > Cany.xaml:514] |
| `MenuFlyoutSubItemForegroundPressed` | followable | L: #E4000000 [MenuFlyout_tr.xaml:189 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [MenuFlyout_tr.xaml:23 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [MenuFlyout_tr.xaml:106 > Cany.xaml:514] |
| `MenuFlyoutSubItemForegroundSubMenuOpened` | followable | L: #E4000000 [MenuFlyout_tr.xaml:190 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [MenuFlyout_tr.xaml:24 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightColor (runtime) [MenuFlyout_tr.xaml:107 > Cany.xaml:513] |
| `MenuFlyoutSubItemForegroundDisabled` | followable | L: #5C000000 [MenuFlyout_tr.xaml:191 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [MenuFlyout_tr.xaml:25 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [MenuFlyout_tr.xaml:108 > Cany.xaml:516] |
| `MenuFlyoutSubItemChevron` | followable | L: #9E000000 [MenuFlyout_tr.xaml:192 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [MenuFlyout_tr.xaml:26 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorButtonTextColor (runtime) [MenuFlyout_tr.xaml:109 > Cany.xaml:512] |
| `MenuFlyoutSubItemChevronPointerOver` | followable | L: #9E000000 [MenuFlyout_tr.xaml:193 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [MenuFlyout_tr.xaml:27 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [MenuFlyout_tr.xaml:110 > Cany.xaml:514] |
| `MenuFlyoutSubItemChevronPressed` | followable | L: #72000000 [MenuFlyout_tr.xaml:194 > Cany.xaml:294 > Cany.xaml:211]; D: #87FFFFFF [MenuFlyout_tr.xaml:28 > Cany.xaml:90 > Cany.xaml:7] | HC: SystemColorHighlightTextColor (runtime) [MenuFlyout_tr.xaml:111 > Cany.xaml:514] |
| `MenuFlyoutSubItemChevronSubMenuOpened` | followable | L: #9E000000 [MenuFlyout_tr.xaml:195 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [MenuFlyout_tr.xaml:29 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightColor (runtime) [MenuFlyout_tr.xaml:112 > Cany.xaml:513] |
| `MenuFlyoutSubItemChevronDisabled` | followable | L: #5C000000 [MenuFlyout_tr.xaml:196 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [MenuFlyout_tr.xaml:30 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [MenuFlyout_tr.xaml:113 > Cany.xaml:516] |
| `MenuFlyoutItemKeyboardAcceleratorTextForeground` | followable | L: #9E000000 [MenuFlyout_tr.xaml:197 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [MenuFlyout_tr.xaml:31 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorWindowTextColor (runtime) [MenuFlyout_tr.xaml:114 > Cany.xaml:509] |
| `MenuFlyoutItemKeyboardAcceleratorTextForegroundPointerOver` | followable | L: #9E000000 [MenuFlyout_tr.xaml:198 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [MenuFlyout_tr.xaml:32 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [MenuFlyout_tr.xaml:115 > Cany.xaml:514] |
| `MenuFlyoutItemKeyboardAcceleratorTextForegroundPressed` | followable | L: #9E000000 [MenuFlyout_tr.xaml:199 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [MenuFlyout_tr.xaml:33 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [MenuFlyout_tr.xaml:116 > Cany.xaml:514] |
| `MenuFlyoutItemKeyboardAcceleratorTextForegroundDisabled` | followable | L: #5C000000 [MenuFlyout_tr.xaml:200 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [MenuFlyout_tr.xaml:34 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [MenuFlyout_tr.xaml:117 > Cany.xaml:516] |
| `ToggleMenuFlyoutItemKeyboardAcceleratorTextForeground` | followable | L: #9E000000 [MenuFlyout_tr.xaml:204 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [MenuFlyout_tr.xaml:35 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorWindowTextColor (runtime) [MenuFlyout_tr.xaml:118 > Cany.xaml:509] |
| `ToggleMenuFlyoutItemKeyboardAcceleratorTextForegroundPointerOver` | followable | L: #9E000000 [MenuFlyout_tr.xaml:205 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [MenuFlyout_tr.xaml:36 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [MenuFlyout_tr.xaml:119 > Cany.xaml:514] |
| `ToggleMenuFlyoutItemKeyboardAcceleratorTextForegroundPressed` | followable | L: #9E000000 [MenuFlyout_tr.xaml:206 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [MenuFlyout_tr.xaml:37 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [MenuFlyout_tr.xaml:120 > Cany.xaml:514] |
| `ToggleMenuFlyoutItemKeyboardAcceleratorTextForegroundDisabled` | followable | L: #5C000000 [MenuFlyout_tr.xaml:207 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [MenuFlyout_tr.xaml:38 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [MenuFlyout_tr.xaml:121 > Cany.xaml:516] |
| `SplitMenuFlyoutItemButtonDividerBrush` | followable | L: #0F000000 [MenuFlyout_tr.xaml:201 > Cany.xaml:347 > Cany.xaml:257]; D: #15FFFFFF [MenuFlyout_tr.xaml:39 > Cany.xaml:143 > Cany.xaml:53] | HC: SystemColorWindowTextColor (runtime) [MenuFlyout_tr.xaml:122 > Cany.xaml:509] |
| `MenuFlyoutPresenterBackground` | followable | L: #00000000 [MenuFlyout_tr.xaml:202 > Materials/Acrylic/AcrylicBrush_tr.xaml:105]; D: #00000000 [MenuFlyout_tr.xaml:40 > Materials/Acrylic/AcrylicBrush_tr.xaml:53] | HC: SystemColorWindowColor (runtime) [MenuFlyout_tr.xaml:123 > Cany.xaml:510] |
| `MenuFlyoutPresenterBorderBrush` | followable | L: #0F000000 [MenuFlyout_tr.xaml:203 > Cany.xaml:345 > Cany.xaml:255]; D: #33000000 [MenuFlyout_tr.xaml:41 > Cany.xaml:141 > Cany.xaml:51] | HC: SystemColorWindowTextColor (runtime) [MenuFlyout_tr.xaml:124 > Cany.xaml:509] |
| `MenuFlyoutPresenterBorderThemeThickness` | stated | L: 1 [MenuFlyout_tr.xaml:208]; D: 1 [MenuFlyout_tr.xaml:42] | HC: 2 [MenuFlyout_tr.xaml:125] |
| `MenuFlyoutItemTextTrimming` | stated | L: Clip [MenuFlyout_tr.xaml:209]; D: Clip [MenuFlyout_tr.xaml:43] | HC: Clip [MenuFlyout_tr.xaml:126] |
| `MenuFlyoutItemBorderThickness` | stated | L: 0 [MenuFlyout_tr.xaml:210]; D: 0 [MenuFlyout_tr.xaml:44] | HC: 0 [MenuFlyout_tr.xaml:127] |
| `MenuFlyoutSubItemBorderThickness` | stated | L: 0 [MenuFlyout_tr.xaml:211]; D: 0 [MenuFlyout_tr.xaml:45] | HC: 0 [MenuFlyout_tr.xaml:128] |
| `MenuFlyoutItemPlaceholderThemeThickness` | stated | L: 28,0,0,0 [MenuFlyout_tr.xaml:212]; D: 28,0,0,0 [MenuFlyout_tr.xaml:46] | HC: 28,0,0,0 [MenuFlyout_tr.xaml:129] |
| `MenuFlyoutItemDoublePlaceholderThemeThickness` | stated | L: 56,0,0,0 [MenuFlyout_tr.xaml:213]; D: 56,0,0,0 [MenuFlyout_tr.xaml:47] | HC: 56,0,0,0 [MenuFlyout_tr.xaml:130] |
| `MenuFlyoutItemRevealBorderThickness` | stated | L: 1 [MenuFlyout_tr.xaml:214]; D: 1 [MenuFlyout_tr.xaml:48] | HC: 1 [MenuFlyout_tr.xaml:131] |
| `ToggleMenuFlyoutItemRevealBorderThickness` | stated | L: 1 [MenuFlyout_tr.xaml:215]; D: 1 [MenuFlyout_tr.xaml:49] | HC: 1 [MenuFlyout_tr.xaml:132] |
| `MenuFlyoutSubItemRevealBorderThickness` | stated | L: 1 [MenuFlyout_tr.xaml:216]; D: 1 [MenuFlyout_tr.xaml:50] | HC: 1 [MenuFlyout_tr.xaml:133] |
| `MenuFlyoutLightDismissOverlayBackground` | followable | L: #99FFFFFF [MenuFlyout_tr.xaml:225 > generic.xaml:4238 > generic.xaml:4129]; D: #99000000 [MenuFlyout_tr.xaml:59 > generic.xaml:313 > generic.xaml:204] | HC: SystemColorWindowColor (runtime) [MenuFlyout_tr.xaml:142 > generic.xaml:2089] |
| `MenuFlyoutItemRevealBackground` | followable | L: Transparent [MenuFlyout_tr.xaml:226 > Materials/Reveal/RevealBrush_tr.xaml:219]; D: Transparent [MenuFlyout_tr.xaml:60 > Materials/Reveal/RevealBrush_tr.xaml:34] | HC: Transparent [MenuFlyout_tr.xaml:143 > Materials/Reveal/RevealBrush_tr.xaml:403] |
| `MenuFlyoutItemRevealBackgroundPointerOver` | followable | L: <media:SolidColorBrush x:Key="SystemControlHighlightListLowRevealBackgroundBrush" Color="{ [MenuFlyout_tr.xaml:227 > Materials/Reveal/RevealBrush_tr.xaml:225]; D: <media:SolidColorBrush x:Key="SystemControlHighlightListLowRevealBackgroundBrush" Color="{ [MenuFlyout_tr.xaml:61 > Materials/Reveal/RevealBrush_tr.xaml:40] | HC: SystemColorHighlightColor (runtime) [MenuFlyout_tr.xaml:144 > Materials/Reveal/RevealBrush_tr.xaml:409] |
| `MenuFlyoutItemRevealBackgroundPressed` | followable | L: <media:SolidColorBrush x:Key="SystemControlHighlightListMediumRevealBackgroundBrush" Color [MenuFlyout_tr.xaml:228 > Materials/Reveal/RevealBrush_tr.xaml:224]; D: <media:SolidColorBrush x:Key="SystemControlHighlightListMediumRevealBackgroundBrush" Color [MenuFlyout_tr.xaml:62 > Materials/Reveal/RevealBrush_tr.xaml:39] | HC: SystemColorHighlightColor (runtime) [MenuFlyout_tr.xaml:145 > Materials/Reveal/RevealBrush_tr.xaml:408] |
| `MenuFlyoutItemRevealBackgroundDisabled` | followable | L: #00FFFFFF [MenuFlyout_tr.xaml:229 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [MenuFlyout_tr.xaml:63 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [MenuFlyout_tr.xaml:146 > Common_tr.xaml:34] |
| `MenuFlyoutItemRevealBorderBrush` | followable | L: #00FFFFFF [MenuFlyout_tr.xaml:230 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [MenuFlyout_tr.xaml:64 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [MenuFlyout_tr.xaml:147 > Common_tr.xaml:34] |
| `MenuFlyoutItemRevealBorderBrushPressed` | followable | L: Transparent [MenuFlyout_tr.xaml:231 > Materials/Reveal/RevealBrush_tr.xaml:244]; D: Transparent [MenuFlyout_tr.xaml:65 > Materials/Reveal/RevealBrush_tr.xaml:59] | HC: Transparent [MenuFlyout_tr.xaml:148 > Materials/Reveal/RevealBrush_tr.xaml:428] |
| `MenuFlyoutItemRevealBorderBrushPointerOver` | followable | L: Transparent [MenuFlyout_tr.xaml:232 > Materials/Reveal/RevealBrush_tr.xaml:244]; D: Transparent [MenuFlyout_tr.xaml:66 > Materials/Reveal/RevealBrush_tr.xaml:59] | HC: Transparent [MenuFlyout_tr.xaml:149 > Materials/Reveal/RevealBrush_tr.xaml:428] |
| `MenuFlyoutItemRevealBorderBrushDisabled` | followable | L: #00FFFFFF [MenuFlyout_tr.xaml:233 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [MenuFlyout_tr.xaml:67 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [MenuFlyout_tr.xaml:150 > Common_tr.xaml:34] |
| `ToggleMenuFlyoutItemRevealBackground` | followable | L: Transparent [MenuFlyout_tr.xaml:234 > Materials/Reveal/RevealBrush_tr.xaml:219]; D: Transparent [MenuFlyout_tr.xaml:68 > Materials/Reveal/RevealBrush_tr.xaml:34] | HC: Transparent [MenuFlyout_tr.xaml:151 > Materials/Reveal/RevealBrush_tr.xaml:403] |
| `ToggleMenuFlyoutItemRevealBackgroundPointerOver` | followable | L: <media:SolidColorBrush x:Key="SystemControlHighlightListLowRevealBackgroundBrush" Color="{ [MenuFlyout_tr.xaml:235 > Materials/Reveal/RevealBrush_tr.xaml:225]; D: <media:SolidColorBrush x:Key="SystemControlHighlightListLowRevealBackgroundBrush" Color="{ [MenuFlyout_tr.xaml:69 > Materials/Reveal/RevealBrush_tr.xaml:40] | HC: SystemColorHighlightColor (runtime) [MenuFlyout_tr.xaml:152 > Materials/Reveal/RevealBrush_tr.xaml:409] |
| `ToggleMenuFlyoutItemRevealBackgroundPressed` | followable | L: <media:SolidColorBrush x:Key="SystemControlHighlightListMediumRevealBackgroundBrush" Color [MenuFlyout_tr.xaml:236 > Materials/Reveal/RevealBrush_tr.xaml:224]; D: <media:SolidColorBrush x:Key="SystemControlHighlightListMediumRevealBackgroundBrush" Color [MenuFlyout_tr.xaml:70 > Materials/Reveal/RevealBrush_tr.xaml:39] | HC: SystemColorHighlightColor (runtime) [MenuFlyout_tr.xaml:153 > Materials/Reveal/RevealBrush_tr.xaml:408] |
| `ToggleMenuFlyoutItemRevealBackgroundDisabled` | followable | L: #00FFFFFF [MenuFlyout_tr.xaml:237 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [MenuFlyout_tr.xaml:71 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [MenuFlyout_tr.xaml:154 > Common_tr.xaml:34] |
| `ToggleMenuFlyoutItemRevealBorderBrush` | followable | L: #00FFFFFF [MenuFlyout_tr.xaml:238 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [MenuFlyout_tr.xaml:72 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [MenuFlyout_tr.xaml:155 > Common_tr.xaml:34] |
| `ToggleMenuFlyoutItemRevealBorderBrushPressed` | followable | L: Transparent [MenuFlyout_tr.xaml:239 > Materials/Reveal/RevealBrush_tr.xaml:244]; D: Transparent [MenuFlyout_tr.xaml:73 > Materials/Reveal/RevealBrush_tr.xaml:59] | HC: Transparent [MenuFlyout_tr.xaml:156 > Materials/Reveal/RevealBrush_tr.xaml:428] |
| `ToggleMenuFlyoutItemRevealBorderBrushPointerOver` | followable | L: Transparent [MenuFlyout_tr.xaml:240 > Materials/Reveal/RevealBrush_tr.xaml:244]; D: Transparent [MenuFlyout_tr.xaml:74 > Materials/Reveal/RevealBrush_tr.xaml:59] | HC: Transparent [MenuFlyout_tr.xaml:157 > Materials/Reveal/RevealBrush_tr.xaml:428] |
| `ToggleMenuFlyoutItemRevealBorderBrushDisabled` | followable | L: #00FFFFFF [MenuFlyout_tr.xaml:241 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [MenuFlyout_tr.xaml:75 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [MenuFlyout_tr.xaml:158 > Common_tr.xaml:34] |
| `MenuFlyoutSubItemRevealBackground` | followable | L: Transparent [MenuFlyout_tr.xaml:242 > Materials/Reveal/RevealBrush_tr.xaml:219]; D: Transparent [MenuFlyout_tr.xaml:76 > Materials/Reveal/RevealBrush_tr.xaml:34] | HC: Transparent [MenuFlyout_tr.xaml:159 > Materials/Reveal/RevealBrush_tr.xaml:403] |
| `MenuFlyoutSubItemRevealBackgroundPointerOver` | followable | L: <media:SolidColorBrush x:Key="SystemControlHighlightListLowRevealBackgroundBrush" Color="{ [MenuFlyout_tr.xaml:243 > Materials/Reveal/RevealBrush_tr.xaml:225]; D: <media:SolidColorBrush x:Key="SystemControlHighlightListLowRevealBackgroundBrush" Color="{ [MenuFlyout_tr.xaml:77 > Materials/Reveal/RevealBrush_tr.xaml:40] | HC: SystemColorHighlightColor (runtime) [MenuFlyout_tr.xaml:160 > Materials/Reveal/RevealBrush_tr.xaml:409] |
| `MenuFlyoutSubItemRevealBackgroundPressed` | followable | L: <media:SolidColorBrush x:Key="SystemControlHighlightAccentRevealBackgroundBrush" Color="{T [MenuFlyout_tr.xaml:244 > Materials/Reveal/RevealBrush_tr.xaml:221]; D: <media:SolidColorBrush x:Key="SystemControlHighlightAccentRevealBackgroundBrush" Color="{T [MenuFlyout_tr.xaml:78 > Materials/Reveal/RevealBrush_tr.xaml:36] | HC: SystemColorHighlightColor (runtime) [MenuFlyout_tr.xaml:161 > Materials/Reveal/RevealBrush_tr.xaml:405] |
| `MenuFlyoutSubItemRevealBackgroundSubMenuOpened` | followable | L: <media:SolidColorBrush x:Key="SystemControlHighlightListLowRevealBackgroundBrush" Color="{ [MenuFlyout_tr.xaml:245 > Materials/Reveal/RevealBrush_tr.xaml:225]; D: <media:SolidColorBrush x:Key="SystemControlHighlightListLowRevealBackgroundBrush" Color="{ [MenuFlyout_tr.xaml:79 > Materials/Reveal/RevealBrush_tr.xaml:40] | HC: SystemColorHighlightColor (runtime) [MenuFlyout_tr.xaml:162 > Materials/Reveal/RevealBrush_tr.xaml:409] |
| `MenuFlyoutSubItemRevealBackgroundDisabled` | followable | L: #00FFFFFF [MenuFlyout_tr.xaml:246 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [MenuFlyout_tr.xaml:80 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [MenuFlyout_tr.xaml:163 > Common_tr.xaml:34] |
| `MenuFlyoutSubItemRevealBorderBrush` | followable | L: #00FFFFFF [MenuFlyout_tr.xaml:247 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [MenuFlyout_tr.xaml:81 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [MenuFlyout_tr.xaml:164 > Common_tr.xaml:34] |
| `MenuFlyoutSubItemRevealBorderBrushPressed` | followable | L: Transparent [MenuFlyout_tr.xaml:248 > Materials/Reveal/RevealBrush_tr.xaml:244]; D: Transparent [MenuFlyout_tr.xaml:82 > Materials/Reveal/RevealBrush_tr.xaml:59] | HC: Transparent [MenuFlyout_tr.xaml:165 > Materials/Reveal/RevealBrush_tr.xaml:428] |
| `MenuFlyoutSubItemRevealBorderBrushPointerOver` | followable | L: Transparent [MenuFlyout_tr.xaml:249 > Materials/Reveal/RevealBrush_tr.xaml:244]; D: Transparent [MenuFlyout_tr.xaml:83 > Materials/Reveal/RevealBrush_tr.xaml:59] | HC: Transparent [MenuFlyout_tr.xaml:166 > Materials/Reveal/RevealBrush_tr.xaml:428] |
| `MenuFlyoutSubItemRevealBorderBrushSubMenuOpened` | followable | L: Transparent [MenuFlyout_tr.xaml:250 > Materials/Reveal/RevealBrush_tr.xaml:244]; D: Transparent [MenuFlyout_tr.xaml:84 > Materials/Reveal/RevealBrush_tr.xaml:59] | HC: Transparent [MenuFlyout_tr.xaml:167 > Materials/Reveal/RevealBrush_tr.xaml:428] |
| `MenuFlyoutSubItemRevealBorderBrushDisabled` | followable | L: #00FFFFFF [MenuFlyout_tr.xaml:251 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [MenuFlyout_tr.xaml:85 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [MenuFlyout_tr.xaml:168 > Common_tr.xaml:34] |
| `MenuFlyoutSeparatorHeight` | stated | L=D: 1 [MenuFlyout_tr.xaml:254] | HC: 1 [MenuFlyout_tr.xaml:254] |
| `MenuFlyoutPresenterThemePadding` | stated | L=D: 0,2,0,2 [MenuFlyout_tr.xaml:255] | HC: 0,2,0,2 [MenuFlyout_tr.xaml:255] |
| `MenuFlyoutThemeMinHeight` | stated | L=D: 32 [MenuFlyout_tr.xaml:256] | HC: 32 [MenuFlyout_tr.xaml:256] |
| `MenuFlyoutItemChevronMargin` | stated | L=D: 24,0,0,-1 [MenuFlyout_tr.xaml:257] | HC: 24,0,0,-1 [MenuFlyout_tr.xaml:257] |
| `MenuFlyoutSeparatorThemePadding` | stated | L=D: -4,1,-4,1 [MenuFlyout_tr.xaml:258] | HC: -4,1,-4,1 [MenuFlyout_tr.xaml:258] |
| `MenuFlyoutItemMargin` | stated | L=D: 4,2,4,2 [MenuFlyout_tr.xaml:259] | HC: 4,2,4,2 [MenuFlyout_tr.xaml:259] |
| `MenuFlyoutItemThemePadding` | stated | L=D: 11,8,11,9 [MenuFlyout_tr.xaml:260] | HC: 11,8,11,9 [MenuFlyout_tr.xaml:260] |
| `MenuFlyoutItemThemePaddingNarrow` | stated | L=D: 11,4,11,5 [MenuFlyout_tr.xaml:261] | HC: 11,4,11,5 [MenuFlyout_tr.xaml:261] |
| `SplitMenuFlyoutItemSeparatorHeight` | stated | L=D: 18 [MenuFlyout_tr.xaml:262] | HC: 18 [MenuFlyout_tr.xaml:262] |
| `SplitMenuFlyoutItemChevronButtonWidth` | stated | L=D: 38 [MenuFlyout_tr.xaml:263] | HC: 38 [MenuFlyout_tr.xaml:263] |
| `MenuFlyoutSystemBackdrop` | followable | L: <DesktopAcrylicBackdrop x:Key="AcrylicBackgroundFillColorDefaultBackdrop" /> [MenuFlyout_tr.xaml:264 > Materials/Acrylic/AcrylicBrush_tr.xaml:106]; D: <DesktopAcrylicBackdrop x:Key="AcrylicBackgroundFillColorDefaultBackdrop" /> [MenuFlyout_tr.xaml:264 > Materials/Acrylic/AcrylicBrush_tr.xaml:54] | HC: <DesktopAcrylicBackdrop x:Key="AcrylicBackgroundFillColorDefaultBackdrop" /> [MenuFlyout_tr.xaml:264 > Materials/Acrylic/AcrylicBrush_tr.xaml:158] |

Counts: CommonStyles/MenuFlyout_themeresources.xaml {'followable': 65, 'stated': 19}

### ToolTip_themeresources.xaml

| Key | Class | L / D | HC |
|---|---|---|---|
| `ToolTipContentThemeFontSize` | stated | L: 12 [ToolTip_tr.xaml:29]; D: 12 [ToolTip_tr.xaml:5] | HC: 12 [ToolTip_tr.xaml:17] |
| `ToolTipBorderThemeThickness` | stated | L: 1 [ToolTip_tr.xaml:30]; D: 1 [ToolTip_tr.xaml:6] | HC: 1 [ToolTip_tr.xaml:18] |
| `ToolTipForeground` | followable | L: #E4000000 [ToolTip_tr.xaml:31 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ToolTip_tr.xaml:7 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [ToolTip_tr.xaml:19 > generic.xaml:2039] |
| `ToolTipBackground` | followable | L: #FFF2F2F2 [ToolTip_tr.xaml:32 > generic.xaml:4168 > generic.xaml:4147]; D: #FF2B2B2B [ToolTip_tr.xaml:8 > generic.xaml:243 > generic.xaml:222] | HC: SystemColorButtonFaceColor (runtime) [ToolTip_tr.xaml:20 > generic.xaml:2019] |
| `ToolTipBorderBrush` | followable | L: #0F000000 [ToolTip_tr.xaml:33 > Cany.xaml:345 > Cany.xaml:255]; D: #33000000 [ToolTip_tr.xaml:9 > Cany.xaml:141 > Cany.xaml:51] | HC: SystemColorWindowTextColor (runtime) [ToolTip_tr.xaml:21 > Cany.xaml:509] |
| `ToolTipForegroundBrush` | followable | L: #E4000000 [ToolTip_tr.xaml:37 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [ToolTip_tr.xaml:13 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorWindowTextColor (runtime) [ToolTip_tr.xaml:25 > Cany.xaml:509] |
| `ToolTipBackgroundBrush` | followable | L: #FCFCFC opacity 0.0 [ToolTip_tr.xaml:38 > Materials/Acrylic/AcrylicBrush_tr.xaml:96]; D: #2C2C2C opacity 0.15 [ToolTip_tr.xaml:14 > Materials/Acrylic/AcrylicBrush_tr.xaml:44] | HC: SystemColorWindowColor (runtime) [ToolTip_tr.xaml:26 > Cany.xaml:510] |
| `ToolTipBorderPadding` | stated | L=D: 9,6,9,8 [ToolTip_tr.xaml:76] | HC: 9,6,9,8 [ToolTip_tr.xaml:76] |
| `ToolTipMaxWidth` | stated | L=D: 320 [ToolTip_tr.xaml:77] | HC: 320 [ToolTip_tr.xaml:77] |

Counts: CommonStyles/ToolTip_themeresources.xaml {'stated': 4, 'followable': 5}

### ProgressBar_themeresources.xaml

| Key | Class | L / D | HC |
|---|---|---|---|
| `ProgressBarBorderThemeThickness` | stated | L: 0 [ProgressBar/ProgressBar_tr.xaml:21]; D: 0 [ProgressBar/ProgressBar_tr.xaml:5] | HC: 1 [ProgressBar/ProgressBar_tr.xaml:13] |
| `ProgressBarForeground` | accent | L: SystemAccentColorDark1 (runtime) [ProgressBar/ProgressBar_tr.xaml:22 > Cany.xaml:329]; D: SystemAccentColorLight2 (runtime) [ProgressBar/ProgressBar_tr.xaml:6 > Cany.xaml:125] | HC: SystemColorHighlightColor (runtime) [ProgressBar/ProgressBar_tr.xaml:14] |
| `ProgressBarBackground` | followable | L: #72000000 [ProgressBar/ProgressBar_tr.xaml:23 > Cany.xaml:252]; D: #8BFFFFFF [ProgressBar/ProgressBar_tr.xaml:7 > Cany.xaml:48] | HC: SystemColorWindowColor (runtime) [ProgressBar/ProgressBar_tr.xaml:15] |
| `ProgressBarBorderBrush` | followable | L: #0F000000 [ProgressBar/ProgressBar_tr.xaml:24 > Cany.xaml:333 > Cany.xaml:243]; D: #12FFFFFF [ProgressBar/ProgressBar_tr.xaml:8 > Cany.xaml:129 > Cany.xaml:39] | HC: SystemColorWindowTextColor (runtime) [ProgressBar/ProgressBar_tr.xaml:16] |
| `ProgressBarPausedForegroundColor` | followable | L: #9D5D00 [ProgressBar/ProgressBar_tr.xaml:25 > Cany.xaml:281]; D: #FCE100 [ProgressBar/ProgressBar_tr.xaml:9 > Cany.xaml:77] | HC: SystemColorWindowTextColor (runtime) [ProgressBar/ProgressBar_tr.xaml:17] |
| `ProgressBarErrorForegroundColor` | followable | L: #C42B1C [ProgressBar/ProgressBar_tr.xaml:26 > Cany.xaml:282]; D: #FF99A4 [ProgressBar/ProgressBar_tr.xaml:10 > Cany.xaml:78] | HC: SystemColorHotlightColor (runtime) [ProgressBar/ProgressBar_tr.xaml:18] |
| `ProgressBarMinHeight` | stated | L=D: 3 [ProgressBar/ProgressBar_tr.xaml:29] | HC: 3 [ProgressBar/ProgressBar_tr.xaml:29] |
| `ProgressBarTrackHeight` | stated | L=D: 1 [ProgressBar/ProgressBar_tr.xaml:30] | HC: 1 [ProgressBar/ProgressBar_tr.xaml:30] |
| `ProgressBarCornerRadius` | stated | L=D: 1.5 [ProgressBar/ProgressBar_tr.xaml:31] | HC: 1.5 [ProgressBar/ProgressBar_tr.xaml:31] |
| `ProgressBarTrackCornerRadius` | stated | L=D: 0.5 [ProgressBar/ProgressBar_tr.xaml:32] | HC: 0.5 [ProgressBar/ProgressBar_tr.xaml:32] |

Counts: ProgressBar/ProgressBar_themeresources.xaml {'stated': 5, 'accent': 1, 'followable': 4}

### TabView_themeresources.xaml

| Key | Class | L / D | HC |
|---|---|---|---|
| `TabViewItemHeaderBackground` | followable | L: #00000000 [TabView/TabView_tr.xaml:6 > Cany.xaml:361 > Cany.xaml:271]; D: #00FFFFFF [TabView/TabView_tr.xaml:84 > Cany.xaml:157 > Cany.xaml:67] | HC: SystemColorButtonFaceColor (runtime) [TabView/TabView_tr.xaml:162 > Cany.xaml:511] |
| `TabViewItemHeaderBackgroundSelected` | followable | L: #F9F9F9 [TabView/TabView_tr.xaml:7 > Cany.xaml:364 > Cany.xaml:274]; D: #282828 [TabView/TabView_tr.xaml:85 > Cany.xaml:160 > Cany.xaml:70] | HC: SystemColorWindowColor (runtime) [TabView/TabView_tr.xaml:163 > Cany.xaml:510] |
| `TabViewItemHeaderDragBackground` | followable | L: #F9F9F9 [TabView/TabView_tr.xaml:8 > Cany.xaml:364 > Cany.xaml:274]; D: #282828 [TabView/TabView_tr.xaml:86 > Cany.xaml:160 > Cany.xaml:70] | HC: SystemColorWindowColor (runtime) [TabView/TabView_tr.xaml:164 > Cany.xaml:510] |
| `TabViewItemHeaderBackgroundPointerOver` | followable | L: #0A000000 [TabView/TabView_tr.xaml:9 > Cany.xaml:359 > Cany.xaml:269]; D: #0FFFFFFF [TabView/TabView_tr.xaml:87 > Cany.xaml:155 > Cany.xaml:65] | HC: SystemColorHighlightColor (runtime) [TabView/TabView_tr.xaml:165 > Cany.xaml:513] |
| `TabViewItemHeaderBackgroundPressed` | followable | L: #B3FFFFFF [TabView/TabView_tr.xaml:10 > Cany.xaml:358 > Cany.xaml:268]; D: #733A3A3A [TabView/TabView_tr.xaml:88 > Cany.xaml:154 > Cany.xaml:64] | HC: SystemColorHighlightColor (runtime) [TabView/TabView_tr.xaml:166 > Cany.xaml:513] |
| `TabViewItemHeaderBackgroundDisabled` | followable | L: #00000000 [TabView/TabView_tr.xaml:11 > Cany.xaml:361 > Cany.xaml:271]; D: #00FFFFFF [TabView/TabView_tr.xaml:89 > Cany.xaml:157 > Cany.xaml:67] | HC: SystemColorWindowColor (runtime) [TabView/TabView_tr.xaml:167 > Cany.xaml:510] |
| `TabViewItemHeaderForeground` | followable | L: #9E000000 [TabView/TabView_tr.xaml:12 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [TabView/TabView_tr.xaml:90 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorWindowTextColor (runtime) [TabView/TabView_tr.xaml:168 > Cany.xaml:509] |
| `TabViewItemHeaderForegroundPressed` | followable | L: #72000000 [TabView/TabView_tr.xaml:13 > Cany.xaml:294 > Cany.xaml:211]; D: #87FFFFFF [TabView/TabView_tr.xaml:91 > Cany.xaml:90 > Cany.xaml:7] | HC: SystemColorHighlightTextColor (runtime) [TabView/TabView_tr.xaml:169 > Cany.xaml:514] |
| `TabViewItemHeaderForegroundSelected` | followable | L: #E4000000 [TabView/TabView_tr.xaml:14 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [TabView/TabView_tr.xaml:92 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightColor (runtime) [TabView/TabView_tr.xaml:170 > Cany.xaml:513] |
| `TabViewItemHeaderForegroundPointerOver` | followable | L: #9E000000 [TabView/TabView_tr.xaml:15 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [TabView/TabView_tr.xaml:93 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [TabView/TabView_tr.xaml:171 > Cany.xaml:514] |
| `TabViewItemHeaderForegroundDisabled` | followable | L: #5C000000 [TabView/TabView_tr.xaml:16 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [TabView/TabView_tr.xaml:94 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [TabView/TabView_tr.xaml:172 > Cany.xaml:516] |
| `TabViewItemIconForeground` | followable | L: #9E000000 [TabView/TabView_tr.xaml:17 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [TabView/TabView_tr.xaml:95 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorWindowTextColor (runtime) [TabView/TabView_tr.xaml:173 > Cany.xaml:509] |
| `TabViewItemIconForegroundPressed` | followable | L: #72000000 [TabView/TabView_tr.xaml:18 > Cany.xaml:294 > Cany.xaml:211]; D: #87FFFFFF [TabView/TabView_tr.xaml:96 > Cany.xaml:90 > Cany.xaml:7] | HC: SystemColorHighlightTextColor (runtime) [TabView/TabView_tr.xaml:174 > Cany.xaml:514] |
| `TabViewItemIconForegroundSelected` | followable | L: #E4000000 [TabView/TabView_tr.xaml:19 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [TabView/TabView_tr.xaml:97 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightColor (runtime) [TabView/TabView_tr.xaml:175 > Cany.xaml:513] |
| `TabViewItemIconForegroundPointerOver` | followable | L: #9E000000 [TabView/TabView_tr.xaml:20 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [TabView/TabView_tr.xaml:98 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [TabView/TabView_tr.xaml:176 > Cany.xaml:514] |
| `TabViewItemIconForegroundDisabled` | followable | L: #5C000000 [TabView/TabView_tr.xaml:21 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [TabView/TabView_tr.xaml:99 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [TabView/TabView_tr.xaml:177 > Cany.xaml:516] |
| `TabViewButtonBackground` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:22 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:100 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorButtonFaceColor (runtime) [TabView/TabView_tr.xaml:178 > Cany.xaml:511] |
| `TabViewButtonBackgroundPressed` | followable | L: #06000000 [TabView/TabView_tr.xaml:23 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [TabView/TabView_tr.xaml:101 > Cany.xaml:113 > Cany.xaml:27] | HC: SystemColorHighlightColor (runtime) [TabView/TabView_tr.xaml:179 > Cany.xaml:513] |
| `TabViewButtonBackgroundPointerOver` | followable | L: #09000000 [TabView/TabView_tr.xaml:24 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [TabView/TabView_tr.xaml:102 > Cany.xaml:112 > Cany.xaml:26] | HC: SystemColorHighlightColor (runtime) [TabView/TabView_tr.xaml:180 > Cany.xaml:513] |
| `TabViewButtonBackgroundDisabled` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:25 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:103 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [TabView/TabView_tr.xaml:181 > Cany.xaml:510] |
| `TabViewButtonForeground` | followable | L: #E4000000 [TabView/TabView_tr.xaml:26 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [TabView/TabView_tr.xaml:104 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [TabView/TabView_tr.xaml:182 > Cany.xaml:512] |
| `TabViewButtonForegroundPressed` | followable | L: #9E000000 [TabView/TabView_tr.xaml:27 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [TabView/TabView_tr.xaml:105 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorHighlightTextColor (runtime) [TabView/TabView_tr.xaml:183 > Cany.xaml:514] |
| `TabViewButtonForegroundPointerOver` | followable | L: #E4000000 [TabView/TabView_tr.xaml:28 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [TabView/TabView_tr.xaml:106 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [TabView/TabView_tr.xaml:184 > Cany.xaml:514] |
| `TabViewButtonForegroundDisabled` | followable | L: #5C000000 [TabView/TabView_tr.xaml:29 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [TabView/TabView_tr.xaml:107 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [TabView/TabView_tr.xaml:185 > Cany.xaml:516] |
| `TabViewButtonBorderBrush` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:30 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:108 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorButtonTextColor (runtime) [TabView/TabView_tr.xaml:186 > Cany.xaml:512] |
| `TabViewButtonBorderBrushPressed` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:31 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:109 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorHighlightColor (runtime) [TabView/TabView_tr.xaml:187 > Cany.xaml:513] |
| `TabViewButtonBorderBrushPointerOver` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:32 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:110 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorHighlightColor (runtime) [TabView/TabView_tr.xaml:188 > Cany.xaml:513] |
| `TabViewButtonBorderBrushDisabled` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:33 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:111 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorGrayTextColor (runtime) [TabView/TabView_tr.xaml:189 > Cany.xaml:516] |
| `TabViewItemSeparator` | followable | L: #0F000000 [TabView/TabView_tr.xaml:46 > Cany.xaml:347 > Cany.xaml:257]; D: #15FFFFFF [TabView/TabView_tr.xaml:124 > Cany.xaml:143 > Cany.xaml:53] | HC: SystemColorWindowTextColor (runtime) [TabView/TabView_tr.xaml:202 > Cany.xaml:509] |
| `TabViewItemHeaderCloseButtonBackground` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:47 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:125 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorButtonFaceColor (runtime) [TabView/TabView_tr.xaml:203 > Cany.xaml:511] |
| `TabViewItemHeaderCloseButtonBackgroundPressed` | followable | L: #06000000 [TabView/TabView_tr.xaml:48 > Cany.xaml:317 > Cany.xaml:231]; D: #0AFFFFFF [TabView/TabView_tr.xaml:126 > Cany.xaml:113 > Cany.xaml:27] | HC: SystemColorButtonFaceColor (runtime) [TabView/TabView_tr.xaml:204 > Cany.xaml:511] |
| `TabViewItemHeaderCloseButtonBackgroundPointerOver` | followable | L: #09000000 [TabView/TabView_tr.xaml:49 > Cany.xaml:316 > Cany.xaml:230]; D: #0FFFFFFF [TabView/TabView_tr.xaml:127 > Cany.xaml:112 > Cany.xaml:26] | HC: SystemColorButtonFaceColor (runtime) [TabView/TabView_tr.xaml:205 > Cany.xaml:511] |
| `TabViewItemHeaderPressedCloseButtonBackground` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:50 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:128 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [TabView/TabView_tr.xaml:206 > Cany.xaml:439] |
| `TabViewItemHeaderPointerOverCloseButtonBackground` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:51 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:129 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [TabView/TabView_tr.xaml:207 > Cany.xaml:439] |
| `TabViewItemHeaderSelectedCloseButtonBackground` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:52 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:130 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [TabView/TabView_tr.xaml:208 > Cany.xaml:439] |
| `TabViewItemHeaderDisabledCloseButtonBackground` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:53 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:131 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorWindowColor (runtime) [TabView/TabView_tr.xaml:209 > Cany.xaml:510] |
| `TabViewItemHeaderCloseButtonForeground` | followable | L: #E4000000 [TabView/TabView_tr.xaml:54 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [TabView/TabView_tr.xaml:132 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [TabView/TabView_tr.xaml:210 > Cany.xaml:512] |
| `TabViewItemHeaderCloseButtonForegroundPressed` | followable | L: #9E000000 [TabView/TabView_tr.xaml:55 > Cany.xaml:293 > Cany.xaml:210]; D: #C5FFFFFF [TabView/TabView_tr.xaml:133 > Cany.xaml:89 > Cany.xaml:6] | HC: SystemColorButtonTextColor (runtime) [TabView/TabView_tr.xaml:211 > Cany.xaml:512] |
| `TabViewItemHeaderCloseButtonForegroundPointerOver` | followable | L: #E4000000 [TabView/TabView_tr.xaml:56 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [TabView/TabView_tr.xaml:134 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorButtonTextColor (runtime) [TabView/TabView_tr.xaml:212 > Cany.xaml:512] |
| `TabViewItemHeaderPressedCloseButtonForeground` | followable | L: #E4000000 [TabView/TabView_tr.xaml:57 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [TabView/TabView_tr.xaml:135 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [TabView/TabView_tr.xaml:213 > Cany.xaml:514] |
| `TabViewItemHeaderPointerOverCloseButtonForeground` | followable | L: #E4000000 [TabView/TabView_tr.xaml:58 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [TabView/TabView_tr.xaml:136 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightTextColor (runtime) [TabView/TabView_tr.xaml:214 > Cany.xaml:514] |
| `TabViewItemHeaderSelectedCloseButtonForeground` | followable | L: #E4000000 [TabView/TabView_tr.xaml:59 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [TabView/TabView_tr.xaml:137 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorHighlightColor (runtime) [TabView/TabView_tr.xaml:215 > Cany.xaml:513] |
| `TabViewItemHeaderDisabledCloseButtonForeground` | followable | L: #5C000000 [TabView/TabView_tr.xaml:60 > Cany.xaml:295 > Cany.xaml:212]; D: #5DFFFFFF [TabView/TabView_tr.xaml:138 > Cany.xaml:91 > Cany.xaml:8] | HC: SystemColorGrayTextColor (runtime) [TabView/TabView_tr.xaml:216 > Cany.xaml:516] |
| `TabViewItemHeaderCloseButtonBorderBrush` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:61 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:139 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [TabView/TabView_tr.xaml:217 > Cany.xaml:439] |
| `TabViewItemHeaderCloseButtonBorderBrushPointerOver` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:62 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:140 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorButtonTextColor (runtime) [TabView/TabView_tr.xaml:218 > Cany.xaml:512] |
| `TabViewItemHeaderCloseButtonBorderBrushPressed` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:63 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:141 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorButtonTextColor (runtime) [TabView/TabView_tr.xaml:219 > Cany.xaml:512] |
| `TabViewItemHeaderCloseButtonBorderBrushSelected` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:64 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:142 > Cany.xaml:111 > Cany.xaml:25] | HC: SystemColorButtonTextColor (runtime) [TabView/TabView_tr.xaml:220 > Cany.xaml:512] |
| `TabViewItemHeaderCloseButtonBorderBrushDisabled` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:65 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:143 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [TabView/TabView_tr.xaml:221 > Cany.xaml:439] |
| `TabViewButtonBackgroundActiveTab` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:66 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:144 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [TabView/TabView_tr.xaml:222 > Cany.xaml:439] |
| `TabViewButtonForegroundActiveTab` | followable | L: #72000000 [TabView/TabView_tr.xaml:67 > Cany.xaml:312 > Cany.xaml:226]; D: #8BFFFFFF [TabView/TabView_tr.xaml:145 > Cany.xaml:108 > Cany.xaml:22] | HC: SystemColorButtonFaceColor (runtime) [TabView/TabView_tr.xaml:223 > generic.xaml:2011] |
| `TabViewItemBorderBrush` | followable | L: #00FFFFFF [TabView/TabView_tr.xaml:69 > Cany.xaml:315 > Cany.xaml:229]; D: #00FFFFFF [TabView/TabView_tr.xaml:147 > Cany.xaml:111 > Cany.xaml:25] | HC: Transparent [TabView/TabView_tr.xaml:225 > Cany.xaml:439] |
| `TabViewButtonBorderThickness` | stated | L: 0 [TabView/TabView_tr.xaml:70]; D: 0 [TabView/TabView_tr.xaml:148] | HC: 1 [TabView/TabView_tr.xaml:226] |
| `TabViewItemHeaderCloseButtonBorderThickness` | stated | L: 0 [TabView/TabView_tr.xaml:71]; D: 0 [TabView/TabView_tr.xaml:149] | HC: 1 [TabView/TabView_tr.xaml:227] |
| `TabViewSelectedItemBorderBrush` | followable | L: gradient TabViewSelectedItemBorderBrush [TabView/TabView_tr.xaml:72]; D: gradient TabViewSelectedItemBorderBrush [TabView/TabView_tr.xaml:150] | HC: gradient TabViewSelectedItemBorderBrush [TabView/TabView_tr.xaml:228] |
| `TabViewItemHeaderPadding` | stated | L=D: 8,3,4,3 [TabView/TabView_tr.xaml:240] | HC: 8,3,4,3 [TabView/TabView_tr.xaml:240] |
| `TabViewSelectedItemHeaderPadding` | stated | L=D: 9,3,5,4 [TabView/TabView_tr.xaml:241] | HC: 9,3,5,4 [TabView/TabView_tr.xaml:241] |
| `TabViewItemMinHeight` | stated | L=D: 32 [TabView/TabView_tr.xaml:242] | HC: 32 [TabView/TabView_tr.xaml:242] |
| `TabViewItemMaxWidth` | stated | L=D: 240 [TabView/TabView_tr.xaml:243] | HC: 240 [TabView/TabView_tr.xaml:243] |
| `TabViewItemMinWidth` | stated | L=D: 100 [TabView/TabView_tr.xaml:244] | HC: 100 [TabView/TabView_tr.xaml:244] |
| `TabViewItemHeaderFontSize` | stated | L=D: 12 [TabView/TabView_tr.xaml:245] | HC: 12 [TabView/TabView_tr.xaml:245] |
| `TabViewItemHeaderIconSize` | stated | L=D: 16 [TabView/TabView_tr.xaml:246] | HC: 16 [TabView/TabView_tr.xaml:246] |
| `TabViewItemHeaderIconMargin` | stated | L=D: 0,0,10,0 [TabView/TabView_tr.xaml:247] | HC: 0,0,10,0 [TabView/TabView_tr.xaml:247] |
| `TabViewItemHeaderCloseButtonHeight` | stated | L=D: 24 [TabView/TabView_tr.xaml:248] | HC: 24 [TabView/TabView_tr.xaml:248] |
| `TabViewItemHeaderCloseButtonWidth` | stated | L=D: 32 [TabView/TabView_tr.xaml:249] | HC: 32 [TabView/TabView_tr.xaml:249] |
| `TabViewItemHeaderCloseButtonSize` | stated | L=D: 16 [TabView/TabView_tr.xaml:250] | HC: 16 [TabView/TabView_tr.xaml:250] |
| `TabViewItemHeaderCloseFontSize` | stated | L=D: 12 [TabView/TabView_tr.xaml:251] | HC: 12 [TabView/TabView_tr.xaml:251] |
| `TabViewItemHeaderCloseMargin` | stated | L=D: 4,0,0,0 [TabView/TabView_tr.xaml:252] | HC: 4,0,0,0 [TabView/TabView_tr.xaml:252] |
| `TabViewItemHeaderPaddingWithCloseButton` | stated | L=D: 8,3,4,3 [TabView/TabView_tr.xaml:253] | HC: 8,3,4,3 [TabView/TabView_tr.xaml:253] |
| `TabViewItemHeaderPaddingWithoutCloseButton` | stated | L=D: 8,3,8,3 [TabView/TabView_tr.xaml:254] | HC: 8,3,8,3 [TabView/TabView_tr.xaml:254] |
| `TabViewItemScrollButtonWidth` | stated | L=D: 32 [TabView/TabView_tr.xaml:255] | HC: 32 [TabView/TabView_tr.xaml:255] |
| `TabViewItemScrollButtonHeight` | stated | L=D: 24 [TabView/TabView_tr.xaml:256] | HC: 24 [TabView/TabView_tr.xaml:256] |
| `TabViewItemScrollButonFontSize` | stated | L=D: 8 [TabView/TabView_tr.xaml:257] | HC: 8 [TabView/TabView_tr.xaml:257] |
| `TabViewItemScrollButtonPadding` | stated | L=D: 7,3,7,3 [TabView/TabView_tr.xaml:258] | HC: 7,3,7,3 [TabView/TabView_tr.xaml:258] |
| `TabViewItemLeftScrollButtonContainerPadding` | stated | L=D: 8,0,3,3 [TabView/TabView_tr.xaml:259] | HC: 8,0,3,3 [TabView/TabView_tr.xaml:259] |
| `TabViewItemRightScrollButtonContainerPadding` | stated | L=D: 3,0,8,3 [TabView/TabView_tr.xaml:260] | HC: 3,0,8,3 [TabView/TabView_tr.xaml:260] |
| `TabViewItemAddButtonWidth` | stated | L=D: 32 [TabView/TabView_tr.xaml:261] | HC: 32 [TabView/TabView_tr.xaml:261] |
| `TabViewItemAddButtonHeight` | stated | L=D: 24 [TabView/TabView_tr.xaml:262] | HC: 24 [TabView/TabView_tr.xaml:262] |
| `TabViewItemAddButtonFontSize` | stated | L=D: 12 [TabView/TabView_tr.xaml:263] | HC: 12 [TabView/TabView_tr.xaml:263] |
| `TabViewItemAddButtonContainerPadding` | stated | L=D: 3,0,0,3 [TabView/TabView_tr.xaml:264] | HC: 3,0,0,3 [TabView/TabView_tr.xaml:264] |
| `TabViewItemSeparatorMargin` | stated | L=D: 0,8,0,8 [TabView/TabView_tr.xaml:266] | HC: 0,8,0,8 [TabView/TabView_tr.xaml:266] |
| `TabViewItemBorderThickness` | stated | L=D: 1 [TabView/TabView_tr.xaml:267] | HC: 1 [TabView/TabView_tr.xaml:267] |
| `TabViewSelectedItemBorderThickness` | stated | L=D: 1,1,1,0 [TabView/TabView_tr.xaml:268] | HC: 1,1,1,0 [TabView/TabView_tr.xaml:268] |
| `TabViewSelectedItemHeaderMargin` | stated | L=D: -1,0,-1,1 [TabView/TabView_tr.xaml:269] | HC: -1,0,-1,1 [TabView/TabView_tr.xaml:269] |

Counts: TabView/TabView_themeresources.xaml {'followable': 52, 'stated': 31}

### InfoBar_themeresources.xaml

| Key | Class | L / D | HC |
|---|---|---|---|
| `InfoBarErrorSeverityBackgroundBrush` | followable | L: #FDE7E9 [InfoBar/InfoBar_tr.xaml:5 > Cany.xaml:378 > Cany.xaml:288]; D: #442726 [InfoBar/InfoBar_tr.xaml:24 > Cany.xaml:174 > Cany.xaml:84] | HC: SystemColorWindowColor (runtime) [InfoBar/InfoBar_tr.xaml:43 > Cany.xaml:510] |
| `InfoBarWarningSeverityBackgroundBrush` | followable | L: #FFF4CE [InfoBar/InfoBar_tr.xaml:6 > Cany.xaml:377 > Cany.xaml:287]; D: #433519 [InfoBar/InfoBar_tr.xaml:25 > Cany.xaml:173 > Cany.xaml:83] | HC: SystemColorWindowColor (runtime) [InfoBar/InfoBar_tr.xaml:44 > Cany.xaml:510] |
| `InfoBarSuccessSeverityBackgroundBrush` | followable | L: #DFF6DD [InfoBar/InfoBar_tr.xaml:7 > Cany.xaml:376 > Cany.xaml:286]; D: #393D1B [InfoBar/InfoBar_tr.xaml:26 > Cany.xaml:172 > Cany.xaml:82] | HC: SystemColorWindowColor (runtime) [InfoBar/InfoBar_tr.xaml:45 > Cany.xaml:510] |
| `InfoBarInformationalSeverityBackgroundBrush` | followable | L: #80F6F6F6 [InfoBar/InfoBar_tr.xaml:8 > Cany.xaml:375 > Cany.xaml:285]; D: #08FFFFFF [InfoBar/InfoBar_tr.xaml:27 > Cany.xaml:171 > Cany.xaml:81] | HC: SystemColorWindowColor (runtime) [InfoBar/InfoBar_tr.xaml:46 > Cany.xaml:510] |
| `InfoBarErrorSeverityIconBackground` | followable | L: #C42B1C [InfoBar/InfoBar_tr.xaml:9 > Cany.xaml:372 > Cany.xaml:282]; D: #FF99A4 [InfoBar/InfoBar_tr.xaml:28 > Cany.xaml:168 > Cany.xaml:78] | HC: SystemColorHighlightColor (runtime) [InfoBar/InfoBar_tr.xaml:47 > Cany.xaml:513] |
| `InfoBarWarningSeverityIconBackground` | followable | L: #9D5D00 [InfoBar/InfoBar_tr.xaml:10 > Cany.xaml:371 > Cany.xaml:281]; D: #FCE100 [InfoBar/InfoBar_tr.xaml:29 > Cany.xaml:167 > Cany.xaml:77] | HC: SystemColorHighlightColor (runtime) [InfoBar/InfoBar_tr.xaml:48 > Cany.xaml:513] |
| `InfoBarSuccessSeverityIconBackground` | followable | L: #0F7B0F [InfoBar/InfoBar_tr.xaml:11 > Cany.xaml:370 > Cany.xaml:280]; D: #6CCB5F [InfoBar/InfoBar_tr.xaml:30 > Cany.xaml:166 > Cany.xaml:76] | HC: SystemColorHighlightColor (runtime) [InfoBar/InfoBar_tr.xaml:49 > Cany.xaml:513] |
| `InfoBarInformationalSeverityIconBackground` | accent | L: SystemAccentColor (runtime) [InfoBar/InfoBar_tr.xaml:12 > Cany.xaml:369]; D: SystemAccentColorLight2 (runtime) [InfoBar/InfoBar_tr.xaml:31 > Cany.xaml:165] | HC: SystemColorHighlightColor (runtime) [InfoBar/InfoBar_tr.xaml:50 > Cany.xaml:513] |
| `InfoBarErrorSeverityIconForeground` | followable | L: #FFFFFF [InfoBar/InfoBar_tr.xaml:13 > Cany.xaml:296 > Cany.xaml:213]; D: #E4000000 [InfoBar/InfoBar_tr.xaml:32 > Cany.xaml:92 > Cany.xaml:9] | HC: SystemColorHighlightTextColor (runtime) [InfoBar/InfoBar_tr.xaml:51 > Cany.xaml:514] |
| `InfoBarWarningSeverityIconForeground` | followable | L: #FFFFFF [InfoBar/InfoBar_tr.xaml:14 > Cany.xaml:296 > Cany.xaml:213]; D: #E4000000 [InfoBar/InfoBar_tr.xaml:33 > Cany.xaml:92 > Cany.xaml:9] | HC: SystemColorHighlightTextColor (runtime) [InfoBar/InfoBar_tr.xaml:52 > Cany.xaml:514] |
| `InfoBarSuccessSeverityIconForeground` | followable | L: #FFFFFF [InfoBar/InfoBar_tr.xaml:15 > Cany.xaml:296 > Cany.xaml:213]; D: #E4000000 [InfoBar/InfoBar_tr.xaml:34 > Cany.xaml:92 > Cany.xaml:9] | HC: SystemColorHighlightTextColor (runtime) [InfoBar/InfoBar_tr.xaml:53 > Cany.xaml:514] |
| `InfoBarInformationalSeverityIconForeground` | followable | L: #FFFFFF [InfoBar/InfoBar_tr.xaml:16 > Cany.xaml:296 > Cany.xaml:213]; D: #E4000000 [InfoBar/InfoBar_tr.xaml:35 > Cany.xaml:92 > Cany.xaml:9] | HC: SystemColorHighlightTextColor (runtime) [InfoBar/InfoBar_tr.xaml:54 > Cany.xaml:514] |
| `InfoBarTitleForeground` | followable | L: #E4000000 [InfoBar/InfoBar_tr.xaml:17 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [InfoBar/InfoBar_tr.xaml:36 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorWindowTextColor (runtime) [InfoBar/InfoBar_tr.xaml:55 > Cany.xaml:509] |
| `InfoBarMessageForeground` | followable | L: #E4000000 [InfoBar/InfoBar_tr.xaml:18 > Cany.xaml:292 > Cany.xaml:209]; D: #FFFFFF [InfoBar/InfoBar_tr.xaml:37 > Cany.xaml:88 > Cany.xaml:5] | HC: SystemColorWindowTextColor (runtime) [InfoBar/InfoBar_tr.xaml:56 > Cany.xaml:509] |
| `InfoBarHyperlinkButtonForeground` | accent | L: SystemAccentColorDark2 (runtime) [InfoBar/InfoBar_tr.xaml:19 > Cany.xaml:297]; D: SystemAccentColorLight3 (runtime) [InfoBar/InfoBar_tr.xaml:38 > Cany.xaml:93] | HC: SystemColorHotlightColor (runtime) [InfoBar/InfoBar_tr.xaml:57 > Cany.xaml:515] |
| `InfoBarBorderBrush` | followable | L: #0F000000 [InfoBar/InfoBar_tr.xaml:20 > Cany.xaml:340 > Cany.xaml:250]; D: #19000000 [InfoBar/InfoBar_tr.xaml:39 > Cany.xaml:136 > Cany.xaml:46] | HC: SystemColorWindowTextColor (runtime) [InfoBar/InfoBar_tr.xaml:58 > Materials/Acrylic/AcrylicBrush_tr.xaml:146] |
| `InfoBarBorderThickness` | stated | L: 1 [InfoBar/InfoBar_tr.xaml:21]; D: 1 [InfoBar/InfoBar_tr.xaml:40] | HC: 2 [InfoBar/InfoBar_tr.xaml:59] |
| `InfoBarTitleFontSize` | stated | L=D: 14 [InfoBar/InfoBar_tr.xaml:62] | HC: 14 [InfoBar/InfoBar_tr.xaml:62] |
| `InfoBarTitleFontWeight` | stated | L=D: SemiBold [InfoBar/InfoBar_tr.xaml:63] | HC: SemiBold [InfoBar/InfoBar_tr.xaml:63] |
| `InfoBarMessageFontSize` | stated | L=D: 14 [InfoBar/InfoBar_tr.xaml:64] | HC: 14 [InfoBar/InfoBar_tr.xaml:64] |
| `InfoBarMessageFontWeight` | stated | L=D: Normal [InfoBar/InfoBar_tr.xaml:65] | HC: Normal [InfoBar/InfoBar_tr.xaml:65] |
| `InfoBarMinHeight` | stated | L=D: 48 [InfoBar/InfoBar_tr.xaml:66] | HC: 48 [InfoBar/InfoBar_tr.xaml:66] |
| `InfoBarCloseButtonSize` | stated | L=D: 38 [InfoBar/InfoBar_tr.xaml:67] | HC: 38 [InfoBar/InfoBar_tr.xaml:67] |
| `InfoBarCloseButtonGlyphSize` | stated | L=D: 16 [InfoBar/InfoBar_tr.xaml:68] | HC: 16 [InfoBar/InfoBar_tr.xaml:68] |
| `InfoBarHyperlinkButtonMargin` | stated | L=D: -12,0,0,0 [InfoBar/InfoBar_tr.xaml:69] | HC: -12,0,0,0 [InfoBar/InfoBar_tr.xaml:69] |
| `InfoBarIconBackgroundGlyph` | stated | L=D: &#xF136; [InfoBar/InfoBar_tr.xaml:70] | HC: &#xF136; [InfoBar/InfoBar_tr.xaml:70] |
| `InfoBarInformationalIconGlyph` | stated | L=D: &#xF13F; [InfoBar/InfoBar_tr.xaml:71] | HC: &#xF13F; [InfoBar/InfoBar_tr.xaml:71] |
| `InfoBarErrorIconGlyph` | stated | L=D: &#xF13D; [InfoBar/InfoBar_tr.xaml:72] | HC: &#xF13D; [InfoBar/InfoBar_tr.xaml:72] |
| `InfoBarWarningIconGlyph` | stated | L=D: &#xF13C; [InfoBar/InfoBar_tr.xaml:73] | HC: &#xF13C; [InfoBar/InfoBar_tr.xaml:73] |
| `InfoBarSuccessIconGlyph` | stated | L=D: &#xF13E; [InfoBar/InfoBar_tr.xaml:74] | HC: &#xF13E; [InfoBar/InfoBar_tr.xaml:74] |
| `InfoBarContentRootPadding` | stated | L=D: 16,0,0,0 [InfoBar/InfoBar_tr.xaml:75] | HC: 16,0,0,0 [InfoBar/InfoBar_tr.xaml:75] |
| `InfoBarIconMargin` | stated | L=D: 0,16,14,16 [InfoBar/InfoBar_tr.xaml:76] | HC: 0,16,14,16 [InfoBar/InfoBar_tr.xaml:76] |
| `InfoBarIconFontSize` | stated | L=D: 16 [InfoBar/InfoBar_tr.xaml:77] | HC: 16 [InfoBar/InfoBar_tr.xaml:77] |
| `InfoBarPanelMargin` | stated | L=D: 0,0,16,0 [InfoBar/InfoBar_tr.xaml:78] | HC: 0,0,16,0 [InfoBar/InfoBar_tr.xaml:78] |
| `InfoBarPanelHorizontalOrientationPadding` | stated | L=D: 0,0,0,0 [InfoBar/InfoBar_tr.xaml:79] | HC: 0,0,0,0 [InfoBar/InfoBar_tr.xaml:79] |
| `InfoBarPanelVerticalOrientationPadding` | stated | L=D: 0,14,0,18 [InfoBar/InfoBar_tr.xaml:80] | HC: 0,14,0,18 [InfoBar/InfoBar_tr.xaml:80] |
| `InfoBarTitleHorizontalOrientationMargin` | stated | L=D: 0,14,0,0 [InfoBar/InfoBar_tr.xaml:81] | HC: 0,14,0,0 [InfoBar/InfoBar_tr.xaml:81] |
| `InfoBarTitleVerticalOrientationMargin` | stated | L=D: 0,14,0,0 [InfoBar/InfoBar_tr.xaml:82] | HC: 0,14,0,0 [InfoBar/InfoBar_tr.xaml:82] |
| `InfoBarMessageHorizontalOrientationMargin` | stated | L=D: 12,14,0,0 [InfoBar/InfoBar_tr.xaml:83] | HC: 12,14,0,0 [InfoBar/InfoBar_tr.xaml:83] |
| `InfoBarMessageVerticalOrientationMargin` | stated | L=D: 0,4,0,0 [InfoBar/InfoBar_tr.xaml:84] | HC: 0,4,0,0 [InfoBar/InfoBar_tr.xaml:84] |
| `InfoBarActionHorizontalOrientationMargin` | stated | L=D: 16,8,0,0 [InfoBar/InfoBar_tr.xaml:85] | HC: 16,8,0,0 [InfoBar/InfoBar_tr.xaml:85] |
| `InfoBarActionVerticalOrientationMargin` | stated | L=D: 0,12,0,0 [InfoBar/InfoBar_tr.xaml:86] | HC: 0,12,0,0 [InfoBar/InfoBar_tr.xaml:86] |
| `InfoBarCloseButtonSymbol` | stated | L=D: Cancel [InfoBar/InfoBar_tr.xaml:87] | HC: Cancel [InfoBar/InfoBar_tr.xaml:87] |

Counts: InfoBar/InfoBar_themeresources.xaml {'followable': 14, 'accent': 2, 'stated': 27}
