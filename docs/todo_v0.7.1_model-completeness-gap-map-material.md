# Gap map — Material 3 and Material 3 Expressive (preset `material-3`)

Phase: gap listing only (docs/todo_v0.7.1_model-completeness-gap-map.md §2–§4). No values are
collected here; a number appears only where it was read in the cited line, to identify the token.
The model is `docs/property-registry.toml` at caeddd6b (branch `v0.7.1-rc1`); a row is a gap only
where no field carries the property, a *partial* gap where a field carries part of it.

## Source abbreviations (all read 2026-10-27)

| Abbrev. | Meaning |
|---|---|
| `v0_192/` | material-web `b4de401eb665ec63474f39319a4ba8f2145974cc` (tag `v2.5.0`), `tokens/versions/v0_192/` (the token set the components use; the pin platform-facts.md uses) |
| `latest/` | same commit, `tokens/versions/latest/sass/` (Material 3 design system 34.0.21, the Expressive set) |
| `mw/` | same commit, repository root (component implementation: SCSS and TS) |
| `compose/` | androidx `e4d933c98df7baa7ea2b83eea6e086915961e0ff`, `compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/` |
| `compose-android/` | same commit, `…/material3/src/androidMain/kotlin/androidx/compose/material3/` |
| `m3.io` | m3.material.io (renders with JavaScript only; platform-facts.md §2.20 records it could not be read — every m3.io cite below is **to verify**) |
| *read* | an Android API the (post-1.0, docs/todo.md:3841) Android reader would query at run time |
| *measure-web* | material-web in a headless browser: computed styles, element screenshots, `getAnimations()` traces |
| *measure-compose* | Compose Material 3 on an Android emulator: layout inspector, screenshots, a screen recording at the display rate for motion |

Row format: **ID** · element · what is missing · source kind and location / measurement · nearest model
field today · seed.

---

## 4.1 Elements the model has, with parts, states or properties it lacks

### Window / application chrome (§2.2)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-W1 | window (Android) | System-bar appearance the app window shows: status/navigation bar icon tone (light/dark) and the contrast scrim behind the navigation bar in edge-to-edge. The bars are drawn by the system (SystemUI) over the app's window; the app sets only their appearance, so whether the model carries them is the rationale's scope decision (main map §6.1) | *read*: `WindowInsetsController.getSystemBarsAppearance()` (API 30), `Window.isNavigationBarContrastEnforced()` (API 29) (`a-ref`, as defined in *Pass 3*); *measure-compose* | none | — (review 2026-10-28: API levels settled from `a-ref`; marked system-drawn) |
| MAT-W2 | window | Predictive-back and activity/window transition motion (what moves, duration, easing) | *published*: m3.io motion / predictive back pages (to verify); *measure-compose* (screen recording) | none (`dialog.entrance` is dialog-only) | — |
| MAT-W3 | window ↔ top app bar | The app bar takes the title-bar role; its scrolled state (see MAT-TA1) is the only "window chrome" state Material has; no inactive-window state exists (no gap there) | — (cross-reference) | `window.title_bar_*` | platform-facts.md §2.2 |
| MAT-W4† | window (Android desktop windowing) | The caption bar and frame the system draws round a freeform app window in Android desktop windowing (tablets, external displays): caption height, its fill (taken from the app, or transparent when the app opts in to draw under it), the app chip, the caption buttons (minimise, maximise, close) and their states, the focused vs unfocused look, the window outline and shadow. Drawn by the system's WM Shell, which runs in the SystemUI process (`wms/src/com/android/wm/shell/docs/overview.md:23-24`, `wms/` as defined in *Pass 3*), not by the app; whether the model carries shell-drawn chrome is the rationale's scope decision (main map §6.1) | *read*: `WindowInsets.Type.captionBar()` (API 30), `WindowInsetsController.APPEARANCE_TRANSPARENT_CAPTION_BAR_BACKGROUND` (API 35) (`a-ref`); *published*: developer.android.com desktop-windowing pages (to verify); *measure-compose* on a desktop-windowing emulator image | `window.title_bar_*`, `inactive_title_bar_*`, `border.*` | platform-facts.md §2.2 cells (lines 1158-1169, "(none)") (review 2026-10-28: API levels settled, as *Pass 3*'s *Found on the way*; marked shell-drawn) |
| MAT-W5† | window / display corners | The rounded corners the app's content is clipped to: the display's corner radius per corner on phones and tablets, and a freeform window's corner radius in desktop windowing | *read*: `WindowInsets.getRoundedCorner(int)`, `Display.getRoundedCorner(int)` → `android.view.RoundedCorner` (API 31; to verify); window radius *measure-compose* | `window.border.corner_radius_px` | platform-facts.md §2.2 cell (line 1168) |

### Button (§2.3)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-B1 | button: variants | Five variants, each with its own container colour, label colour, outline and elevation: **elevated**, **filled**, **tonal**, **outlined**, **text**. The model carries tonal (`background_color` …) plus filled's two rest colours | *published*: `v0_192/_md-comp-elevated-button.scss:37-90`, `_md-comp-filled-button.scss:37-90`, `_md-comp-filled-tonal-button.scss:37-99`, `_md-comp-outlined-button.scss:34-85`, `_md-comp-text-button.scss:34-78`; `latest/_md-comp-button-{elevated,filled,tonal,outlined,text}.scss` | `button.*` (tonal), `primary_background`, `primary_text_color` | — |
| MAT-B2 | button: filled (primary) states | Hover / focus / pressed state-layer colour and label colour, disabled container and label colours of the *filled* button (only its rest pair exists) | *published*: `v0_192/_md-comp-filled-button.scss:42-81` | `primary_background`, `primary_text_color` (partial) | — |
| MAT-B3 | button: outlined | Outline colour per state (rest, hover, focus, pressed, disabled + disabled outline opacity), no container | *published*: `v0_192/_md-comp-outlined-button.scss:38-39, :41, :46, :71-74`; `latest/_md-comp-button-outlined.scss:21-123` | `button.border.color` (one colour) | — |
| MAT-B4 | button: text | Text-button padding (its own leading/trailing space), transparent container, label colour per state | *published*: `v0_192/_md-comp-text-button.scss:34-78`; `mw/tokens/_md-comp-text-button.scss:85-86` | none (the model's one padding is the tonal button's) | platform-facts.md §2.3 border.padding_horizontal cell |
| MAT-B5 | button: focused state | Focus state layer (colour + opacity), focus label colour, focus container elevation — no focused background for any button | *published*: `v0_192/_md-comp-filled-tonal-button.scss:47-52` (and the same keys in every button file) | none (only `defaults.focus_ring_*`) | — |
| MAT-B6 | button: per-state elevation | Container elevation per state: rest, hover, focus, pressed, disabled (tonal/filled rise to level1 on hover; elevated rests at level1) | *published*: `v0_192/_md-comp-filled-tonal-button.scss:38, :43, :47, :54, :82`; `_md-comp-elevated-button.scss:38-78` | `button.border.shadow_enabled` (bool, rest only) | — |
| MAT-B7 | button: icon | Icon size, icon colour per state (rest/hover/focus/pressed/disabled) and disabled icon opacity | *published*: `v0_192/_md-comp-filled-tonal-button.scss:89-99` (`with-icon-*`); `latest/_md-comp-button.scss:36, :56, :68, :80, :88` | `button.icon_text_gap_px` only | — |
| MAT-B8 | button: padding with an icon | The icon-side padding differs from the label-only padding (leading 16 with a leading icon) — a padding conditional on content | *published*: `mw/tokens/_md-comp-filled-tonal-button.scss:96-105` | `button.border.padding_*` (one set) | platform-facts.md §2.3 |
| MAT-B9 | button: Expressive sizes | Five sizes (xsmall, small, medium, large, xlarge): container height, icon size, icon–label space, leading/trailing space, outline width, label typescale per size | *published*: `latest/_md-comp-button-{xsmall,small,medium,large,xlarge}.scss:22-48` | `button.min_height_px`, `icon_text_gap_px`, `border.padding_*` (one size) | — |
| MAT-B10 | button: Expressive shape families | Round vs square container shape; a different shape when selected (toggle); a pressed shape | *published*: `latest/_md-comp-button.scss:48-50` (`container.shape.round/.square`), `:102` (`pressed.container.shape`), `:114-116` (`selected.container.shape.*`); per size `latest/_md-comp-button-small.scss:34-46` | `button.border.corner_radius_px` (one radius) | — |
| MAT-B11 | button: shape-morph motion | Corner-size spring on press (damping, stiffness) | *published*: `latest/_md-comp-button.scss:96-98` (`pressed.container.corner-size.motion.spring.damping/stiffness`); per size `latest/_md-comp-button-small.scss:38-40` | none | — |
| MAT-B12 | button: toggle (selected/unselected) | Unselected container colour, selected/unselected × hover/focus/pressed state-layer colour, label colour and icon colour | *published*: `latest/_md-comp-button-filled.scss:78-124`, `_md-comp-button-tonal.scss:22-124`, `_md-comp-button-outlined.scss`, `_md-comp-button-elevated.scss` | `button.checked_background`, `checked_text_color` (rest only) | — |
| MAT-B13 | button: elevation motion | Elevation (shadow) transition duration and easing | *published*: `mw/button/internal/_elevation.scss:18-20` (`transition-duration: 280ms`, `emphasized-easing`) | none | — |
| MAT-B14 | button: press feedback | Ripple (see MAT-X9) | — | none | — |
| MAT-B15 | button: label typography | Label tracking and line height (label-large) — the `Font` struct carries neither | *published*: `v0_192/_md-comp-filled-tonal-button.scss:63, :66` (`label-text-line-height`, `label-text-tracking`) | `button.font` (no tracking / line height) | — |

### Text input (§2.4) and text area (§2.29)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-I1 | input: outline width per state | Focus outline width (thicker than rest), hover outline width, disabled outline width | *published*: `v0_192/_md-comp-outlined-text-field.scss:41, :78, :88, :146` | `input.border.line_width_px` (one width) | platform-facts.md §2.4 border.line_width cell ("focus 2") |
| MAT-I2 | input: disabled outline | Disabled outline colour + opacity | *published*: `v0_192/_md-comp-outlined-text-field.scss:39-40` | `input.border.color` (rest only) | platform-facts.md §2.4 disabled_background cell |
| MAT-I3 | input: floating label | The label part: font at rest and populated (smaller size + line height), colour per state (rest, hover, focus, error, disabled + opacity), padding below it | *published*: `v0_192/_md-comp-outlined-text-field.scss:35-36, :50, :57, :67, :74, :84, :119-142`; `mw/tokens/_md-comp-outlined-field.scss:79-86` (incl. `label-text-padding-bottom`) | none | — |
| MAT-I4 | input: outline notch | The gap cut into the outline around the floated label (label padding) | *published*: `mw/tokens/_md-comp-outlined-field.scss:90` (`outline-label-padding`); `mw/field/internal/_outlined-field.scss:111-141, :216` | none | — |
| MAT-I5 | input: supporting (helper) text | Supporting text under the field: font, colour per state (incl. error, disabled + opacity), top/leading/trailing space | *published*: `v0_192/_md-comp-outlined-text-field.scss:42-44, :54, :63, :71, :79, :89, :147-169`; `mw/tokens/_md-comp-outlined-field.scss:92-99` | none | — |
| MAT-I6 | input: character counter | Trailing supporting text (counter) — same style as supporting text, trailing aligned | *published*: `mw/field/internal/_supporting-text.scss` (to verify lines); *measure-web* | none | — |
| MAT-I7 | input: leading / trailing icons | Icon size, icon colour per state (rest, hover, focus, error, disabled + opacity), the content-side spaces with an icon | *published*: `v0_192/_md-comp-outlined-text-field.scss:37-38, :46-47, :51, :55, :59, :64, :68, :72, :75, :81, :85, :91, :143-144, :171-172`; `mw/tokens/_md-comp-outlined-field.scss:103-104` | none | — |
| MAT-I8 | input: prefix / suffix text | Prefix and suffix colours (and their spacing to the input text) | *published*: `v0_192/_md-comp-outlined-text-field.scss:99, :102`; spacing *measure-web* | none | — |
| MAT-I9 | input: error state | Error outline colour (rest/hover/focus), error caret, error input-text colour, error label, error supporting text, error icons | *published*: `v0_192/_md-comp-outlined-text-field.scss:48-72` | none (`defaults.danger_color` only) | — |
| MAT-I10 | input: hover/focus text and label colours | Hover and focus input-text colours (equal to rest in v0_192 but separate tokens) and label colours | *published*: `v0_192/_md-comp-outlined-text-field.scss:73-91` | `input.font.color` (partial) | — |
| MAT-I11 | input: filled variant | The filled text field: container colour, hover state layer over the container, disabled container colour + opacity, top-only corner shape, the bottom active indicator (colour and height per state: rest, hover, focus, error, disabled) | *published*: `v0_192/_md-comp-filled-text-field.scss:34-112` | `input.background_color`, `border.corner_radius_px` (no per-corner, no indicator) | platform-facts.md §2.4 corner_radius cell ("4 4 0 0") |
| MAT-I12 | input: caret geometry and blink | Caret width and blink timing (material-web leaves both to the browser; Compose draws its own cursor) | *published*: `compose foundation …/text/TextFieldCursor.kt:73, :104` (`DefaultCursorThickness`, value per platform `actual`: Android's is `…/androidMain/…/text/TextFieldCursor.android.kt:22`); blink: Compose foundation cursor animation, switched by `LocalCursorBlinkEnabled` (`TextFieldCursor.kt:49`), the user's interval *read* from version 36.1 (MAT-P3-21†); *measure-compose* | `input.caret_color` only | — (review 2026-10-28: the two "to verify" settled, as *Pass 3*'s *Found on the way*) |
| MAT-I13 | input: selection handles | Android selection handles (the teardrop drag handles) and their colour | *published*: `compose/MaterialTheme.kt:276-281` (`handleColor = primary`); geometry *measure-compose* | none | — |
| MAT-I14 | input: label float motion | Label float/shrink animation, outline-notch opening, placeholder fade: durations and easing | *published*: `mw/field/internal/field.ts:300-301` (150, `EASING.STANDARD`); `mw/field/internal/_content.scss:16-21, :78-79`; `mw/field/internal/_outlined-field.scss:92-93, :158-159, :187-188` (`duration-short3`, `easing-emphasized`) | none | — |
| MAT-I15 | input: label typography | Input-text tracking and line height (body-large) | *published*: `v0_192/_md-comp-outlined-text-field.scss:95, :104` | `input.font` (no tracking / line height) | — |
| MAT-I16 | text area | Multi-line field's minimum rows / height and resize behaviour | *measure-web* (`md-outlined-text-field type="textarea"`) — no token (to verify) | `text_area.border` (padding only) | — |
| MAT-I17† | input: text magnifier | The magnifier shown while the caret or a selection handle is dragged: its size, corner radius, elevation (shadow), zoom and offset above the touch point | *published*: androidx `compose/foundation/foundation/src/androidMain/kotlin/androidx/compose/foundation/Magnifier.android.kt:87-111` (`zoom`, `size`, `cornerRadius`, `elevation`, left to the platform `android.widget.Magnifier` when unspecified); `Magnifier.Builder` defaults (to verify); *measure-compose* | none | — |
| MAT-I18† | input: composition and spell-check marks | The underline drawn under the IME's composing text; spell-check / suggestion underlines and their colours (whether Compose draws any: to verify); material-web leaves both to the browser | *published*: androidx `compose/foundation/foundation/src/commonMain/kotlin/androidx/compose/foundation/text/TextFieldDelegate.kt:425-447` (composition range → `TextDecoration.Underline`); suggestion underlines *measure-compose* (to verify) | none | — |
| MAT-I19† | input: password (secure) field | The obfuscation glyph; whether the last typed character shows briefly, and for how long; the default mode that follows the system's show-passwords setting (a reveal toggle, when shown, is an app-supplied trailing icon: MAT-I7) | *published*: `compose/SecureTextField.kt:128-144` (`textObfuscationMode = TextObfuscationMode.System`, `textObfuscationCharacter`), `:296-312` (outlined), `:390` (`DefaultObfuscationCharacter`); reveal timing in foundation's `BasicSecureTextField` (file to verify); *read*: `Settings.System.TEXT_SHOW_PASSWORD` (to verify); material-web `type="password"` masks with the browser's glyph | none | — |

### Checkbox / radio (§2.5)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-C1 | radio: own colours and shape | Unselected ring colour, selected ring + dot colours (an unfilled `primary` ring and dot, not a filled disc), per state (hover, focus, pressed, disabled + opacity) | *published*: `v0_192/_md-comp-radio-button.scss:28-68`; `mw/radio/internal/_radio.scss:126-127` | `checkbox.*` shared (`checked_background`, `indicator_color`, `radio_*` sizes) | docs/todo.md:235-256 |
| MAT-C2 | checkbox/radio: state-layer halo | The circular halo behind the box: size, shape, and colour + opacity per selected/unselected × hover/focus/pressed | *published*: `v0_192/_md-comp-checkbox.scss:68-77, :85-90, :107-126`; `v0_192/_md-comp-radio-button.scss:36-68`, `:51` (`state-layer-size`) | `checkbox.hover_background` (one colour, no size, no focus/pressed) | platform-facts.md §2.5 hover_background cell |
| MAT-C3 | checkbox: selected × state | Selected container colour and icon (mark) colour per hover/focus/pressed; selected outline widths per state | *published*: `v0_192/_md-comp-checkbox.scss:65-87` | `checkbox.checked_background`, `indicator_color` (rest only) | — |
| MAT-C4 | checkbox: unselected × state | Unselected outline colour and width per hover/focus/pressed | *published*: `v0_192/_md-comp-checkbox.scss:104-126` | `checkbox.unchecked_border_color`, `border.line_width_px` (rest only) | — |
| MAT-C5 | checkbox: error state | Selected error container/icon colours per state, unselected error outline colours, error state layers | *published*: `v0_192/_md-comp-checkbox.scss:33-40, :51-63, :97-104` | none | — |
| MAT-C6 | checkbox: indeterminate | The indeterminate (mixed) mark — a dash — its geometry and colours | *published*: `mw/checkbox/internal/_checkbox.scss:183` (two-rect mark; lines to verify); *measure-web* | none | — |
| MAT-C7 | checkbox: disabled parts | Unselected disabled outline colour/width/opacity, selected disabled mark colour, disabled container opacity (separate from the label) | *published*: `v0_192/_md-comp-checkbox.scss:44-50, :91-95` | `checkbox.disabled_background`, `disabled_text_color` (partial) | — |
| MAT-C8 | checkbox/radio: motion | Mark enter/exit (scale + opacity; durations and emphasized-decelerate / -accelerate easings), radio inner-circle grow, colour fades | *published*: `mw/checkbox/internal/_checkbox.scss:126-144, :210-234`; `mw/radio/internal/_radio.scss:117-132` | none | — |
| MAT-C9 | checkbox/radio: touch target | The 48 touch-target wrapper margin around the 18/20 box | *published*: `mw/checkbox/internal/_checkbox.scss:61-79` | none | — |

### Menu (§2.6)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-M1 | menu item: selected | Selected item container colour, label colour, leading/trailing icon colours | *published*: `v0_192/_md-comp-menu.scss:35-41`; `latest/_md-comp-menu.scss:153-165` | none on `menu` (`defaults.selection_*` are list/sidebar fields) | — |
| MAT-M2 | menu item: focus / pressed | Focus and pressed state-layer colour + opacity, focus/pressed label colours | *published*: `latest/_md-comp-menu.scss:93-101, :141-149` | `menu.hover_*` only | — |
| MAT-M3 | menu item: icons | Leading/trailing icon colour per state, trailing icon size, disabled icon opacity | *published*: `latest/_md-comp-menu.scss:41-53, :169-205` | `menu.icon_size_px` | — |
| MAT-M4 | menu item: secondary texts | Supporting text and trailing supporting text (shortcut) font and colour | *published*: `latest/_md-comp-menus.scss:122-140`; `latest/_md-comp-menus-standard.scss` (`menu-item.supporting-text.*`, `trailing-supporting-text.*`) | none | — |
| MAT-M5 | menu: submenu indicator and timing | Cascading-menu indicator icon colour and size; hover-to-open and hover-to-close delays | *published*: `latest/_md-comp-menu.scss:25, :57`; `mw/menu/internal/submenu/sub-menu.ts:241, :266` (400 ms countdowns) | none | — |
| MAT-M6 | menu: divider | Divider thickness and its vertical margins inside a menu | *published*: `latest/_md-comp-menu.scss:29` (`divider.height`); margins *measure-web* | `menu.separator_color` | — |
| MAT-M7 | menu: container insets vs item padding | The container's top/bottom space as a menu property (the model keeps it on `popover.border`) | *published*: `mw/tokens/_md-comp-menu.scss:19, :24` (`bottom-space`, `top-space`) | `popover.border.padding_*` (partial) | — |
| MAT-M8 | menu: width bounds | Minimum (and maximum) menu width | *published*: `mw/menu/internal/_menu.scss:44` (`min-width: 112px`); maximum: Compose `DropdownMenuItemDefaultMaxWidth` (`compose/Menu.kt:2390-2391`), material-web states none | none | — (review 2026-10-28: the maximum settled, as *Pass 3*'s *Found on the way*) |
| MAT-M9 | menu: open/close motion | Open: height reveal + staggered item fade (full duration, item opacity duration, easing); close: faster, emphasized-accelerate | *published*: `mw/menu/internal/menu.ts:704-867` | none | — |
| MAT-M10 | menu: Expressive menus | Rounded container (`corner-large`), item shapes (rest, selected, first/last child inner corners), groups (gap, group shape, padding), standard vs vibrant colour styles, horizontal and icon-only menus, section labels, 44 item height, item spaces | *published*: `latest/_md-comp-menus.scss:24-140`, `latest/_md-comp-menus-standard.scss:21-187`, `latest/_md-comp-menus-vibrant.scss:21-159` | `menu.border.corner_radius_px`, `row_height_px`, `border.padding_*` (one style) | platform-facts.md §2.6 rows citing `latest/_md-comp-menus.scss` |
| MAT-M11 | menu item: typography | Label tracking and line height | *published*: `latest/_md-comp-menu.scss:125, :133` | `menu.font` (no tracking / line height) | — |

### Tooltip (§2.7)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-T1 | tooltip: rich variant | Rich tooltip: subhead font/colour, supporting text, action (label colour per state, state layers), container colour, elevation, larger corner | *published*: `v0_192/_md-comp-rich-tooltip.scss:37-119`; Compose `compose/Tooltip.kt` (`RichTooltip`) | `tooltip.*` (plain only); `popover.*` reuses its colour | platform-facts.md §2.16 |
| MAT-T2 | tooltip: auto-hide timing | Non-persistent tooltip dismissed after a fixed duration | *published*: `compose/internal/BasicTooltip.kt:426-433, :465` (`TooltipDuration = 1500L`); `compose/Tooltip.kt:1083-1090` | none (`tooltip.show_delay_ms` only) | — |
| MAT-T3 | tooltip: show/hide motion | Fade/scale in and out: durations, easing | *published*: `compose/Tooltip.kt` (animation spec — lines to verify); *measure-compose* | none | — |
| MAT-T4 | tooltip: placement | Spacing between tooltip and anchor; optional caret (pointer) size and shape (the side, flip and clamping against the anchor: MAT-P4-9†) | *published*: `compose/Tooltip.kt:603, :1448` (`SpacingBetweenTooltipAndAnchor`); caret `compose/Tooltip.kt:532` (`TooltipDefaults.caretSize`), `:548-549` (`caretShape()`), `:1343` (`DefaultTooltipCaretShape`) | none | — (review 2026-10-28: caret lines settled; the placement's row named) |
| MAT-T5 | tooltip: minimum height | Plain tooltip minimum height | *published*: `compose/Tooltip.kt:1450-1451` | none (`max_width_px` only) | platform-facts.md §2.7 max_width cell |
| MAT-T6 | tooltip: touch trigger | Long-press timing that shows a tooltip on touch | *read*: `ViewConfiguration.getLongPressTimeout()` | none | — |
| MAT-T7 | tooltip: typography | Supporting text tracking and line height (body-small) | *published*: `v0_192/_md-comp-plain-tooltip.scss:37, :41` | `tooltip.font` | — |

### Scrollbar (§2.8)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-SC1 | scroll indicator (Android) | Fade-out delay and fade duration of the scroll indicator | *read*: `ViewConfiguration.getScrollDefaultDelay()`, `ViewConfiguration.getScrollBarFadeDuration()`; *measure-compose* | `scrollbar.overlay_mode` (bool only) | — |
| MAT-SC2 | overscroll | Android 12+ stretch overscroll (what deforms, how far, its spring back) | *measure-compose* (screen recording); Compose foundation overscroll effect (file to verify) | none | — |
| MAT-SC3† | scrollbar (Compose M3 `nonInteractiveScrollbar`) | Compose Material 3 draws a visual-only scrollbar: thumb colour stated as a colour role at an opacity, thickness, minimum thumb length, maximum thumb length as a fraction of the viewport, a track colour (transparent by default), track insets along and across the axis, fade-in while scrolling and fade-out after a delay (duration and delay), an always-visible option, and its edge (end edge for vertical, bottom for horizontal). No hover or pressed state (visual only). The model has the colours, width, minimum length and overlay flag; it lacks the fade timings, the two insets, the maximum fraction and the colour-at-opacity form | *published*: `compose/Scrollbar.kt:46-117` (the modifier and its parameters), `:119-148` (`NonInteractiveScrollbarDefaults`) | `scrollbar.thumb_color`, `track_color`, `thumb_width_px`, `min_thumb_length_px`, `overlay_mode` (partial) | platform-facts.md §2.8 cells (lines 1304-1311, "M3 has no scrollbar component") |
| MAT-SC4† | overscroll (Android 11 and older) | The glow edge effect drawn before Android 12's stretch (MAT-SC2): its colour, shape, growth with the pull and fade | *read*: theme attribute `android:colorEdgeEffect` (API 21), `EdgeEffect.getColor()` (API 21) (`a-ref`; the reference lists no public `getType()`, so which of the two effects a device draws, this glow or MAT-SC2's stretch, is not read through a public `EdgeEffect` API); Compose foundation's pre-API-31 overscroll (file to verify); *measure-compose* on an API 30 emulator | none | — (review 2026-10-28: `EdgeEffect.getType()` removed, it is not a public API; API levels settled) |

(Corrected by pass 2: Compose Material 3 draws a scrollbar, MAT-SC3†, so the preset's scrollbar values have a Material source that platform-facts.md §2.8 does not cite yet — a value question, not a model gap.)

### Slider (§2.9)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-S1 | slider: handle shape and size | The Expressive handle is a bar (width × height, its own shape), not a disc; its width changes on hover, focus, pressed and disabled | *published*: `latest/_md-comp-slider.scss:36, :48, :56-62, :76, :108, :174` | `slider.thumb_diameter_px` (disc) | platform-facts.md §2.9 thumb_diameter cell |
| MAT-S2 | slider: handle–track gap | The space between the handle and each track half | *published*: `latest/_md-comp-slider.scss:30-34` (`active.handle.leading-space`, `.padding`, `.trailing-space`) | none | — |
| MAT-S3 | slider: track shape | Track corner radius, inner vs outer corner sizes, active vs inactive track shapes and heights | *published*: `v0_192/_md-comp-slider.scss:39, :65`; `latest/_md-comp-slider.scss:40-42, :70, :114-116, :196` | `slider.track_height_px` (no radius) | connectors/native-theme-gpui/examples/showcase-gpui/info/inputs.rs:1446 ("SliderTheme states no rail radius") |
| MAT-S4 | slider: stop indicators | End-of-track stop dots: size, colour (selected/unselected), shape, trailing space, container colour/opacity per active/inactive/disabled | *published*: `latest/_md-comp-slider.scss:38, :52, :68, :82-84, :110, :118, :128-140, :192, :254-258` | none | — |
| MAT-S5 | slider: tick marks | Tick colours and opacities on the active / inactive / disabled track, tick shape | *published*: `v0_192/_md-comp-slider.scss:102-115` | `slider.tick_mark_length_px` only | — |
| MAT-S6 | slider: value indicator | The value label: container colour, height, shape, label font/colour, its distance above the handle; its show/hide motion and reduced-motion behaviour | *published*: `v0_192/_md-comp-slider.scss:66-90`; `latest/_md-comp-slider.scss:72, :86, :264-276`; motion `mw/slider/internal/_slider.scss:91-97, :328-329` | none | — |
| MAT-S7 | slider: state-layer halo | Halo size and colour + opacity for hover, focus, pressed | *published*: `v0_192/_md-comp-slider.scss:49-50, :59-60, :93-96` | `slider.thumb_hover_color` (partial) | — |
| MAT-S8 | slider: focus/pressed colours | Focus and pressed handle colours; focus and pressed active/inactive track colours | *published*: `v0_192/_md-comp-slider.scss:48, :92`; `latest/_md-comp-slider.scss:144-150, :234-238` | none | — |
| MAT-S9 | slider: handle elevation | Handle elevation and shadow colour (rest, disabled), track elevation | *published*: `v0_192/_md-comp-slider.scss:43, :53, :55, :97` | none | — |
| MAT-S10 | slider: overlapping handles | Outline colour and width drawn round a handle that overlaps the other (range slider) | *published*: `v0_192/_md-comp-slider.scss:98-100` | none | docs/todo.md:225-233 (slider thumb outline) |
| MAT-S11 | slider: Expressive sizes | xsmall…xlarge: track height, handle height, track leading/trailing shapes, inset icon size and padding | *published*: `latest/_md-comp-slider-{xsmall,small,medium,large,xlarge}.scss:18-30` | none | — |
| MAT-S12 | slider: layout variants | Range slider, centred slider, vertical slider | *published*: Compose `compose/Slider.kt` (`RangeSlider`, `VerticalSlider`; lines to verify); *measure-compose* | none | — |

### Progress bar (§2.10)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-P1 | linear progress: wavy indicator | Wave amplitude, wavelength (determinate and indeterminate), wavy height | *published*: `latest/_md-comp-progress-indicator-linear.scss:20-26, :56, :64` | none | — |
| MAT-P2 | linear progress: indicator–track gap | Space between the active indicator and the track | *published*: `latest/_md-comp-progress-indicator.scss:21`; `latest/_md-comp-progress-indicator-linear.scss:48, :60` | none | — |
| MAT-P3 | linear progress: stop indicator | End dot: size, colour, shape, trailing space | *published*: `latest/_md-comp-progress-indicator.scss:29, :41-43`; `latest/_md-comp-progress-indicator-linear.scss:28-30, :40-44` | none | — |
| MAT-P4 | linear progress: thickness variants | Default and thick variant (indicator thickness, track thickness, height) | *published*: `latest/_md-comp-progress-indicator-linear.scss:18, :32-52, :62` | `progress_bar.track_height_px` (one) | — |
| MAT-P5 | linear progress: separate indicator shape | Active-indicator shape separate from the track shape | *published*: `v0_192/_md-comp-linear-progress-indicator.scss:30, :41`; `latest/_md-comp-progress-indicator.scss:39, :47` | `progress_bar.border.corner_radius_px` (one) | — |
| MAT-P6 | linear progress: motion | Indeterminate animation (cycle duration, keyframe curves), determinate value transition (duration, easing), four-colour indeterminate colours | *published*: `mw/progress/internal/_linear-progress.scss:28-31, :77-281`; `v0_192/_md-comp-linear-progress-indicator.scss:31-37` (four-color tokens) | none | connectors/native-theme-gpui/src/widgets/progress.rs:139 ("the model states no motion") |
| MAT-P7 | linear progress: buffer | Buffer (secondary progress) bar and its dots | *measure-web* (`md-linear-progress buffer`); no token (to verify) | none | — |

### Tab bar (§2.11)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-TB1 | tabs: secondary variant | Secondary tabs: full-width 2-high indicator, own label/icon colours per state | *published*: `v0_192/_md-comp-secondary-navigation-tab.scss:37-85` | `tab.*` (one style) | platform-facts.md §2.11 active_background cell |
| MAT-TB2 | tabs: indicator shape and length | Indicator corner radii (rounded top corners) and its length rule (label width vs full tab width) | *published*: `v0_192/_md-comp-primary-navigation-tab.scss:39`; `mw/tabs/internal/tab.ts:103-105` | `tab.active_indicator_width_px` (thickness), `_side` | — |
| MAT-TB3 | tabs: focus / pressed / active-hover | Focus and pressed state layers (active and inactive), active-hover state layer colour, focus/pressed label colours | *published*: `v0_192/_md-comp-primary-navigation-tab.scss:41-65, :85-99` | `tab.hover_background`, `hover_text_color` (inactive hover only) | — |
| MAT-TB4 | tabs: icons | Icon size, icon colours active/inactive × states, icon+label tab height, icon–label gap | *published*: `v0_192/_md-comp-primary-navigation-tab.scss:67-83`; `mw/tabs/internal/_tab.scss:89` | none | — |
| MAT-TB5 | tabs: bar divider | Divider line under the tab row: colour and thickness | *published*: `latest/_md-comp-primary-navigation-tab.scss:31, :59` | none (`tab.border` is the tab's own frame) | — |
| MAT-TB6 | tabs: indicator motion | Indicator slide between tabs (duration, easing), reduced-motion variant | *published*: `mw/tabs/internal/tab.ts:152-153, :159-174, :212` | none | — |
| MAT-TB7 | tabs: scrollable edge padding | Leading inset of a scrollable tab row (the overflow itself: MAT-P4-6†) | *published*: Compose `compose/TabRow.kt:987` (`ScrollableTabRowEdgeStartPadding`) | none | — (review 2026-10-28: lines settled; the overflow's row named) |
| MAT-TB8 | tabs: label typography | Tracking and line height (title-small) | *published*: `v0_192/_md-comp-primary-navigation-tab.scss:103, :107` | `tab.font` | — |

### Sidebar = navigation drawer (§2.12)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-SB1 | drawer: active indicator geometry | Indicator shape (pill), height, width | *published*: `v0_192/_md-comp-navigation-drawer.scss:37-41` | `sidebar.selection_background` (colour only) | platform-facts.md §2.12 selection_background cell |
| MAT-SB2 | drawer: per-state colours | Active × hover/focus/pressed state layers, icon and label colours; inactive focus/pressed; focus/pressed opacities | *published*: `v0_192/_md-comp-navigation-drawer.scss:42-64, :70, :95-111, :160` | `sidebar.hover_background` only | — |
| MAT-SB3 | drawer: icons | Icon colours active/inactive (size is `defaults.icon_sizes.panel_px`) | *published*: `v0_192/_md-comp-navigation-drawer.scss:54, :105` | none | — |
| MAT-SB4 | drawer: headline | Section headline font and colour | *published*: `v0_192/_md-comp-navigation-drawer.scss:72-91` | none | — |
| MAT-SB5 | drawer: badge label | Trailing large-badge label font and colour | *published*: `v0_192/_md-comp-navigation-drawer.scss:133-155` | none | — |
| MAT-SB6 | drawer: modal variant | Modal container colour and elevation, scrim colour and opacity | *published*: `v0_192/_md-comp-navigation-drawer.scss:157-159`; `latest/_md-comp-navigation-drawer.scss:36, :150-156` | `sidebar.background_color`, `border.shadow_enabled` | platform-facts.md §2.12 |
| MAT-SB7 | drawer: container geometry | Container width and height; per-corner shape (only the end corners rounded); bottom-sheet drawer shape | *published*: `v0_192/_md-comp-navigation-drawer.scss:66-69` | `sidebar.border.corner_radius_px` (one radius, all corners) | platform-facts.md §2.12 corner_radius cell ("0 16 16 0") |
| MAT-SB8 | drawer: active label weight | A different label weight for the active item | *published*: `v0_192/_md-comp-navigation-drawer.scss:58` | none (`selection_text_color` only) | — |
| MAT-SB9 | drawer: open/close motion | Slide/width transition and scrim fade: duration, easing, delay | *published*: `mw/labs/navigationdrawer/internal/_navigation-drawer-modal.scss:17-19, :73-107`; m3.io (to verify) | none | — |

### Toolbar = top app bar (§2.13)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-TA1 | app bar: on-scroll state | Container colour and elevation when content scrolls under it; the transition | *published*: `v0_192/_md-comp-top-app-bar-small.scss:60-63`; `latest/_md-comp-app-bar.scss` (`on-scroll.container.*`); transition *measure-web* / `mw/labs/gb/components/appbar/app-bar.scss:51-53` (labs) | none | platform-facts.md §2.13 background_color cell |
| MAT-TA2 | app bar: icon colours | Leading and trailing icon colours | *published*: `v0_192/_md-comp-top-app-bar-small.scss:58, :64` | none (`toolbar.icon_size_px` only) | — |
| MAT-TA3 | app bar: size variants | Centre-aligned, medium, large (and Expressive medium/large *flexible*) bars: heights, headline fonts, title alignment, collapse-on-scroll | *published*: `latest/_md-comp-top-app-bar-{small-centered,medium,large}.scss`; `latest/_md-comp-app-bar-{small,medium,large,medium-flexible,large-flexible}.scss:20-39` | `toolbar.bar_height_px`, `font` (one) | — |
| MAT-TA4 | app bar: subtitle | Subtitle font and colour (Expressive) | *published*: `latest/_md-comp-app-bar.scss` (`subtitle.color`), `latest/_md-comp-app-bar-small.scss` (`subtitle.font`) | none | — |
| MAT-TA5 | app bar: avatar | Avatar size and shape | *published*: `latest/_md-comp-app-bar.scss` (`avatar.size`); `latest/_md-comp-top-app-bar-small-centered.scss` (`avatar.shape/size`) | none | — |
| MAT-TA6 | app bar: embedded search | Search field in the app bar: container colour/height/shape, label colour, spaces, on-scroll colour | *published*: `latest/_md-comp-app-bar.scss` (`search.*`), `latest/_md-comp-app-bar-small.scss` (`search.container.*`) | none | — |
| MAT-TA7 | app bar: headline typography | Tracking and line height (title-large) | *published*: `v0_192/_md-comp-top-app-bar-small.scss:40, :43` | `toolbar.font` | — |
| MAT-TA8† | app bar: overflow | Items that do not fit in an app bar row (or column) collapse into an overflow indicator, an icon button that opens a menu: the indicator's icon, its size and the menu it opens | *published*: `compose/AppBarRow.kt:29-54` (`overflowIndicator`); `compose/AppBarColumn.kt:29-46` (the indicator at the bottom of the column) | none | — (review 2026-10-28: AppBarColumn lines settled) |

### Status bar (§2.14)

No Material gap: M3 has no status bar (platform-facts.md §2.14). Android's system status bar is MAT-W1.

### List / table (§2.15)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-L1 | list item: multi-line | Two-line and three-line item heights; supporting text, overline and trailing supporting text fonts and colours | *published*: `v0_192/_md-comp-list.scss:156-250` | `list.row_height_px`, `item_font` (one line) | — |
| MAT-L2 | list item: leading elements | Leading icon size/colour, avatar size/shape/colour and its label font, image size/shape, video sizes/shape | *published*: `v0_192/_md-comp-list.scss:114-155` | none | — |
| MAT-L3 | list item: trailing elements | Trailing icon size/colour, selected/unselected trailing icon colours | *published*: `v0_192/_md-comp-list.scss:192, :222-224, :252` | none | — |
| MAT-L4 | list item: states | Focus, pressed, dragged (incl. dragged elevation) and disabled state layers; per-state label and icon colours; disabled icon opacities | *published*: `v0_192/_md-comp-list.scss:43-89, :182-190` | `list.hover_*`, `disabled_text_color` (partial) | — |
| MAT-L5 | list item: selected × state | Selected container colour; selected × hover/focus/pressed/dragged/disabled state layers, label, overline, supporting text and icon colours; selected-disabled opacities per part | *published*: `latest/_md-comp-list.scss:77-91, :257-339` | `list.selection_*` (rest only) | platform-facts.md §2.15 selection_background cell |
| MAT-L6 | list: dividers | Divider colour, thickness, leading/trailing inset and top/bottom space between items | *published*: `v0_192/_md-comp-list.scss:37-38`; `latest/_md-comp-list.scss:25-35, :125` | `list.grid_color` (data table only, no inset) | — |
| MAT-L7 | list item: element gap | Space between the leading element and the text | *published*: `latest/_md-comp-list.scss:37` (`list-item.between-space`) | none | — |
| MAT-L8 | list: Expressive segmented and shapes | Segmented list: item container colour, gap between items; Expressive item shapes per state (rest, hover, focus, pressed, dragged, disabled, and each selected state) — a shape change on interaction; expressive leading/trailing icon sizes and leading image shape | *published*: `latest/_md-comp-list.scss:63, :101, :121, :139-143, :159, :181, :193, :225, :243, :255, :259-265, :281, :303, :315, :323` | `list.border.corner_radius_px` (one, rest) | — |
| MAT-L9 | list: expandable item | Expanded item container colour; trailing chevron in a container (colour collapsed/expanded), its shape | *published*: `latest/_md-comp-list-expand.scss:21-35` | `expander.*` (partial: no expanded fill, no chevron container) | platform-facts.md §2.27 font cell |
| MAT-L10 | list: reorder | Dragged item container/label/icon colours, drop-zone colour, shadow colour, shape | *published*: `latest/_md-comp-list-reorder.scss:22-44` | none | — |
| MAT-L11 | list: swipe-reveal actions | Revealed action icon-button container/icon/state-layer colours and shapes | *published*: `latest/_md-comp-list-reveal.scss:21-49` | none | — |
| MAT-L12 | data table: header and footer | Header hover headline colour, sorting icon colour; footer container colour/height and supporting text | *published*: `v0_192/_md-comp-data-table.scss:35-47, :63-64` | `list.header_*` (partial; no footer) | connectors/native-theme-gpui/src/colors.rs:448 ("the model states no footer colour") |
| MAT-L13 | data table: rows and frame | Row selected/unselected container colours with hover state layers; table outline colour/width; container shape | *published*: `v0_192/_md-comp-data-table.scss:34, :66-67, :86-96` | `list.selection_background`, `grid_color` (partial) | — |
| MAT-L14 | list item: typography | Label tracking and line height (body-large) | *published*: `v0_192/_md-comp-list.scss:94, :98` | `list.item_font` | — |

### Popover / dropdown (§2.16)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-PO1 | popover: elevation geometry and tint | Level-2 shadow layers and surface-tint layer (see MAT-X3, MAT-X6) | *published*: `latest/_md-comp-menu.scss:63-69` | `popover.border.shadow_enabled` | — |
| MAT-PO2 | popover: enter/exit | Open/close motion (see MAT-M9) | — | none | — |

### Splitter (§2.17)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-SP1 | drag handle: geometry | Handle width and length inside its container width, handle shape | *published*: `latest/_md-comp-drag-handle.scss:23-53` (`width`, `height`, `container.width`, `shape`) | `splitter.divider_width_px` (one width) | platform-facts.md §2.17 |
| MAT-SP2 | drag handle: states | Focus and pressed colours / state layers (hover via a state layer, not a colour change); elevation | *published*: `latest/_md-comp-drag-handle.scss:23-53` (focus/hover/pressed `state-layer.*`, `elevation`) | `splitter.hover_color` (partial) | — |
| MAT-SP3 | adaptive panes | Spacer between panes per window size class | *published*: Compose adaptive `PaneScaffoldDirective.kt:56-74` (platform-facts.md §2.20) | `layout.section_gap_px` (unrelated) | platform-facts.md §2.20 |

### Separator (§2.18)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-D1 | divider: insets | Inset, inset-start, inset-end divider variants and their inset amount | *published*: `mw/divider/internal/_divider.scss:39-46` | `separator.line_*` only | — |

### Text scale (§2.19)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-TS1 | typescale roles | 11 of the 15 roles have no slot: display-large, display-medium, headline-large, headline-medium, title-large, title-small, body-large, body-medium (= defaults.font only by mapping), label-large, label-medium, label-small | *published*: `v0_192/_md-sys-typescale.scss`; `latest/_md-sys-typescale.scss:20-564` | `text_scale.{caption,section_heading,dialog_title,display}` | — |
| MAT-TS2 | typescale: tracking | Letter spacing per role | *published*: `v0_192/_md-sys-typescale.scss:40` (body-large-tracking, and one `*-tracking` key per role) | none (`TextScaleEntry` has size, weight, line height) | — |
| MAT-TS3 | typescale: typeface per role | Brand vs plain typeface (headline/display/title use brand) and the weight names | *published*: `latest/_md-ref-typeface.scss:18-26`; `v0_192/_md-sys-typescale.scss:37` (`body-large-font` = plain) | `defaults.font.family` (one family) | — |
| MAT-TS4 | typescale: emphasized set (Expressive) | 15 emphasized roles (heavier variants) | *published*: `latest/_md-sys-typescale-emphasized.scss:20-552` | none | — |
| MAT-TS5 | typescale: variable axes | Per-role variable-font axes: `crsv`, `fill`, `grad`, `hexp`, `opsz`, `rond`, `slnt`, `wdth`, `wght` | *published*: `latest/_md-sys-typescale.scss:20-42` (body-large; the same per role) | none | — |

### Layout container defaults (§2.20)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-LA1 | layout: window size classes | Margins and pane spacing that change with the window width class (compact … extra-large) and the breakpoints | *published*: m3.io layout pages (to verify); Compose adaptive `PaneScaffoldDirective.kt:56-74`; window size class breakpoints in androidx.window (file to verify) | `layout.*` (one value each) | platform-facts.md §2.20 |

### Switch (§2.21)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-SW1 | switch: pressed handle size | The handle grows while pressed | *published*: `v0_192/_md-comp-switch.scss:51-52` | `switch.thumb_diameter_px`, `unchecked_thumb_diameter_px` | platform-facts.md §2.21 thumb_diameter cell |
| MAT-SW2 | switch: handle colours per state | Selected and unselected handle colours for hover, focus, pressed | *published*: `v0_192/_md-comp-switch.scss:53, :65, :77, :93, :108, :123` | `switch.thumb_background`, `unchecked_thumb_background` (rest) | — |
| MAT-SW3 | switch: handle icons | Selected/unselected icon (check / close) colours per state, icon size, disabled icon colour + opacity, the larger with-icon handle | *published*: `v0_192/_md-comp-switch.scss:33-34, :42-44, :55, :67, :74-76, :79, :95, :110, :120-125, :138-139` | none | — |
| MAT-SW4 | switch: state-layer halo | Halo size, shape and colour + opacity per selected/unselected × hover/focus/pressed | *published*: `v0_192/_md-comp-switch.scss:57-59, :69-71, :81-83, :87-88, :97-99, :112-114, :127-129` | none (`hover_*_background` are track colours) | platform-facts.md §2.21 hover_checked_background cell |
| MAT-SW5 | switch: focus/pressed track | Focus and pressed track colours; focus/pressed track outline colours | *published*: `v0_192/_md-comp-switch.scss:61, :85, :101-103, :131-133` | `switch.hover_*` (hover only) | — |
| MAT-SW6 | switch: disabled handle per selection | Disabled selected and unselected handle colours (and opacities) differ | *published*: `v0_192/_md-comp-switch.scss:31-32, :38-40` | `switch.disabled_thumb_color` (one colour) | platform-facts.md §2.21 disabled_thumb_color cell |
| MAT-SW7 | switch: handle elevation | Handle elevation and shadow colour | *published*: `latest/_md-comp-switch.scss:74, :98, :102` | none | — |
| MAT-SW8 | switch: motion | Handle travel (custom overshoot curve), handle size change, colour fades, icon transitions | *published*: `mw/switch/internal/_handle.scss:24-25, :55-56, :67, :90`; `mw/switch/internal/_track.scss:37`; `mw/switch/internal/_icon.scss:33-34` | none | — |
| MAT-SW9 | switch: focus indicator offset | Switch-specific focus ring offset | *published*: `latest/_md-comp-switch.scss:92-96` | `defaults.focus_ring_offset_px` (global) | — |

### Dialog (§2.22)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-DG1 | dialog: section spacing | Separate headline, content and actions paddings; headline-to-body and body-to-actions gaps | *published*: `mw/dialog/internal/_dialog.scss:111, :192, :216-218` | `dialog.border.padding_*`, `button_gap_px` (partial) | platform-facts.md §2.22 notes ("Dialog internal spacing … not exposed") |
| MAT-DG2 | dialog: icon colour and placement | Hero icon colour; centred headline when an icon is present | *published*: `v0_192/_md-comp-dialog.scss:121`; placement *measure-web* | `dialog.icon_size_px` only | — |
| MAT-DG3 | dialog: actions | Action label colour per state, font, state layers | *published*: `v0_192/_md-comp-dialog.scss:37-71` | none (no text-button variant, MAT-B4) | — |
| MAT-DG4 | dialog: dividers | Dividers above/below scrolling content: colour, height | *published*: `latest/_md-comp-dialog.scss:24, :118` | none | — |
| MAT-DG5 | dialog: scrim | Withdrawn (review 2026-10-28): duplicate of MAT-X23, the same `md.comp.scrim` colour and opacity with the same source; see MAT-X23. Scrim colour and opacity behind the dialog (MAT-X23) | *published*: `latest/_md-comp-scrim.scss:20-22` | none | — (review 2026-10-28: withdrawn, see MAT-X23) |
| MAT-DG6 | dialog: motion | Open (container, content, scrim: durations, easings, delays) and close | *published*: `mw/dialog/internal/animations.ts:57-158` | `dialog.entrance` (enum, no timing, no exit) | connectors/native-theme-gpui/README.md:351 |
| MAT-DG7 | dialog: subhead | Subhead font and colour | *published*: `latest/_md-comp-dialog.scss:82-102` | none | — |
| MAT-DG8 | dialog: window margin | The margin kept to the window edges when the dialog is at its maximum size | *published*: `mw/dialog/internal/_dialog.scss:39-40` (`calc(100% - 48px)`) | `dialog.max_width_px`, `max_height_px` | — |
| MAT-DG9 | dialog: headline typography | Tracking and line height (headline-small, body-medium) | *published*: `v0_192/_md-comp-dialog.scss:78, :81, :101, :105` | `dialog.title_font`, `body_font` | — |
| MAT-DG10† | dialog: action layout | Actions aligned to the end edge; when they do not fit in a row they wrap into a column, and the confirm action, last in the row, comes first in the column; the row gap and the column gap | *published*: `mw/dialog/internal/_dialog.scss:213-219` (`display: flex`, `gap`, `justify-content: flex-end`); `compose/AlertDialog.kt:263-268` (`AlertDialogFlowRow` call), `:378-405` (order flipped between row and column), `:414-417` (`ButtonsMainAxisSpacing`, `ButtonsCrossAxisSpacing`) | `dialog.button_gap_px`, `button_order` (one gap, row order only) | — |

### Spinner / progress ring (§2.23)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-SN1 | circular progress: track | Track colour and thickness (Expressive), gap between indicator and track | *published*: `latest/_md-comp-progress-indicator-circular.scss:42-44`; `latest/_md-comp-progress-indicator.scss:21, :33, :45` | `spinner.fill_color`, `stroke_width_px` | — |
| MAT-SN2 | circular progress: wavy | Wave amplitude, wavelength, with-wave size | *published*: `latest/_md-comp-progress-indicator-circular.scss:20-22, :46` | none | — |
| MAT-SN3 | circular progress: thick variant | Thick size, indicator thickness, track thickness, gap | *published*: `latest/_md-comp-progress-indicator-circular.scss:26-38` | none | — |
| MAT-SN4 | circular progress: cap shape | Active-indicator shape (stroke cap) | *published*: `v0_192/_md-comp-circular-progress-indicator.scss:29` | none | — |
| MAT-SN5 | circular progress: four-colour | Four indicator colours for the indeterminate four-colour variant | *published*: `v0_192/_md-comp-circular-progress-indicator.scss:31-37` | none | — |
| MAT-SN6 | circular progress: motion | Indeterminate arc expand/contract time, cycle, rotation, easing; determinate transition | *published*: `mw/progress/internal/_circular-progress.scss:36-164` | none | — |

### Combo box = select (§2.24)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-CB1 | select: field parts | Everything MAT-I1…I11 lists for the text field: label, notch, supporting text, error, leading icon, focus width, disabled outline, filled variant | *published*: `v0_192/_md-comp-outlined-select.scss:82-261`; `v0_192/_md-comp-filled-select.scss` | `combo_box.*` | — |
| MAT-CB2 | select: arrow colour per state | Trailing (arrow) icon colour rest/hover/focus/error/disabled + opacity | *published*: `v0_192/_md-comp-outlined-select.scss:108-110, :122, :148, :160, :177, :259` | `combo_box.arrow_icon_size_px` only | — |
| MAT-CB3 | select: open state | Open (expanded) state: arrow rotation and its motion; the attached menu's selected item colour and item height | *published*: `v0_192/_md-comp-outlined-select.scss:37-80`; `mw/select/internal/_shared.scss:54-55`; rotation *measure-web* | none | — |

### Segmented control (§2.25)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-SG1 | segmented button: selected check | The check icon a selected segment shows: size and colour per state; its show/width animation | *published*: `v0_192/_md-comp-outlined-segmented-button.scss:71, :77, :85, :91, :113`; `mw/labs/segmentedbutton/internal/_shared.scss:289-356` | none | — |
| MAT-SG2 | segmented button: per-state colours | Selected/unselected × hover/focus/pressed state layers, label and icon colours | *published*: `v0_192/_md-comp-outlined-segmented-button.scss:41-111` | `segmented_control.hover_background` (one) | — |
| MAT-SG3 | segmented button: outer-only corners | Only the outer ends are rounded — per-corner radius | *published*: `v0_192/_md-comp-outlined-segmented-button.scss:93` | `segmented_control.border.corner_radius_px` (all corners) | — |
| MAT-SG4 | segmented button: disabled parts | Disabled outline colour + opacity, disabled icon colour + opacity | *published*: `v0_192/_md-comp-outlined-segmented-button.scss:35-40` | `segmented_control.disabled_opacity` | — |

### Card (§2.26)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-CA1 | card: three variants | Filled, elevated and outlined cards: container colour, outline, elevation each | *published*: `v0_192/_md-comp-filled-card.scss:34-59`, `_md-comp-elevated-card.scss:34-57`, `_md-comp-outlined-card.scss:34-63` | `card.*` (one style) | platform-facts.md §2.26 bracketed cells |
| MAT-CA2 | card: interactive states | Hover/focus/pressed/dragged state layers and per-state elevation; outlined card's per-state outline colour; disabled container colour/opacity, disabled outline | *published*: same files, `:38-59` / `:38-57` / `:38-63` | none | — |
| MAT-CA3 | card: icon | Icon colour and size | *published*: `v0_192/_md-comp-filled-card.scss:55-56` | none | — |

### Expander (§2.27)

| ID | Element | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-EX1 | expander ≈ Expressive expandable list item | See MAT-L9 (expanded fill, chevron container colours, shapes) | *published*: `latest/_md-comp-list-expand.scss:21-35` | `expander.*` | platform-facts.md §2.27 |

### Link (§2.28)

No Material gap: M3 has no link component, but Compose Material 3's `Text` styles the links of rich text, `colorScheme.primary` and an underline (`compose/Text.kt:673-695`; the seed follow-up, 2026-10-28). The model has both fields (`defaults.link_color`, `link.underline_enabled`), so this is a value `platform-facts.md` §2.28 lacks, not a gap.

---

## 4.2 Standard controls the model does not have

Each row is one whole component; "what is missing" lists the parts and property families its token
set carries, every one of which the model lacks.

| ID | Component | What is missing (parts · states · properties) | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-N1 | FAB (small, standard, large; Expressive medium) | container colour, width, height, shape, elevation (rest, hover, focus, pressed, lowered), shadow colour, surface tint; icon colour and size; state layers; focus indicator; colour styles primary / secondary / tertiary / surface and the Expressive *-container styles; branded FAB | *published*: `v0_192/_md-comp-fab-*.scss`; `latest/_md-comp-fab.scss:20-26`, `_md-comp-fab-{small,medium,large}.scss`, `_md-comp-fab-{primary,secondary,tertiary,surface}{,-small,-large}.scss`, `_md-comp-fab-*-container.scss:22-52`, `_md-comp-fab-branded*.scss`; motion `mw/fab/internal/_shared.scss:177-179` | `button.*` | — |
| MAT-N2 | Extended FAB (small, medium, large) | as FAB plus label font/colour, icon–label space, leading/trailing space; lowered elevation; colour styles | *published*: `latest/_md-comp-extended-fab*.scss` (e.g. `_md-comp-extended-fab.scss:21-33`, `_md-comp-extended-fab-primary.scss:25-165`); `v0_192/_md-comp-extended-fab-*.scss` | `button.*` | — |
| MAT-N3 | FAB menu (Expressive) | close button (size, shape, elevation, icon), menu items (height, shape, spaces, label, icon), colours per style, between-spaces; open/close morph | *published*: `latest/_md-comp-fab-menu.scss:22-50`, `_md-comp-fab-menu-{primary,secondary,tertiary}-{close-button,container}.scss` | none | — |
| MAT-N4 | Icon button: standard, filled, tonal, outlined | container colour (none for standard), icon colour and size, outline colour/width, state layers + their size/shape, focus indicator, disabled opacities; **toggle** (selected/unselected) colours | *published*: `v0_192/_md-comp-icon-button.scss`, `_md-comp-filled-icon-button.scss`, `_md-comp-filled-tonal-icon-button.scss`, `_md-comp-outlined-icon-button.scss`; `latest/_md-comp-icon-button.scss:24-190`, `_md-comp-icon-button-{standard,filled,tonal,outlined}.scss` | none (connectors draw flat buttons from `button.*`) | connectors/native-theme-gpui/src/geometry.rs:291 ("the model states no flat-button frame") |
| MAT-N5 | Icon button: Expressive sizes, widths, shapes | xsmall…xlarge container heights, icon sizes; narrow / default / wide leading-trailing spaces; round/square, selected and pressed shapes with corner spring | *published*: `latest/_md-comp-icon-button-{xsmall,small,medium,large,xlarge}.scss:21-51` | none | — |
| MAT-N6 | Button group (standard, connected), Expressive | between-space, container height per size; connected inner-corner size (rest, pressed, selected), container shape; standard group's pressed width multiplier and its spring | *published*: `latest/_md-comp-button-group-standard-{xsmall…xlarge}.scss:20-28`, `_md-comp-button-group-connected-{xsmall…xlarge}.scss:20-30` | `segmented_control.*` (partial analogue) | — |
| MAT-N7 | Split button (Expressive) | leading and trailing button spaces, between-space, inner/outer corner sizes (rest, hovered, pressed, selected), trailing icon size; trailing-button rotation | *published*: `latest/_md-comp-split-button-{xsmall…xlarge}.scss:20-44`; motion `mw/labs/gb/components/splitbutton/split-button.scss:160-161` (labs) | none | — |
| MAT-N8 | Menu buttons (filled, outlined, standard) | a button with trailing dropdown icon: container, label, leading/trailing icon colours and sizes per state, shape | *published*: `latest/_md-comp-filled-menu-button.scss:25-111`, `_md-comp-outlined-menu-button.scss:24-110`, `_md-comp-standard-menu-button.scss:24-96` | `combo_box.*` / `button.*` (partial) | — |
| MAT-N9 | Chips: assist, filter, input, suggestion | height, shape, flat vs elevated containers (colour, elevation, outline colour/opacity/width), label font/colour, leading/trailing icons, avatar (input chip), selected/unselected colours (filter, input), state layers incl. dragged, disabled opacities, focus indicator | *published*: `v0_192/_md-comp-{assist,filter,input,suggestion}-chip.scss`; `latest/_md-comp-assist-chip.scss:25-127`, `_md-comp-filter-chip.scss:25-269`, `_md-comp-input-chip.scss:25-293`, `_md-comp-suggestion-chip.scss:25-127` | none | — |
| MAT-N10 | Badge (small dot, large with label) | colour, size, shape; large badge label font and colour; placement offset on the anchor icon | *published*: `latest/_md-comp-badge.scss:22-46`; offset Compose `compose/Badge.kt` (to verify) | none | — |
| MAT-N11 | Bottom app bar | container colour, height (with and without FAB), shape, elevation, surface tint | *published*: `latest/_md-comp-bottom-app-bar.scss:22-34` | `toolbar.*` (top bar) | platform-facts.md §2.14 |
| MAT-N12 | Navigation bar (+ Expressive horizontal/vertical items) | container colour/height/shape/elevation; active indicator colour/size/shape; icon and label colours per active/inactive × state; label font; badge; item spaces | *published*: `v0_192/_md-comp-navigation-bar.scss`; `latest/_md-comp-navigation-bar.scss:25-143`, `_md-comp-nav-bar.scss:23-67`, `_md-comp-nav-bar-item-{horizontal,vertical}.scss:20-28`; motion `mw/labs/navigationbar/internal/_navigation-bar.scss:66-67` | `tab.*` (partial analogue) | — |
| MAT-N13 | Navigation rail (+ Expressive collapsed, expanded, modal) | container colour/width/shape/elevation; active indicator (size, shape, no-label variant); icon/label colours per state; menu icon; badges; expanded widths, item spaces, header space | *published*: `v0_192/_md-comp-navigation-rail.scss:37-116`; `latest/_md-comp-navigation-rail.scss:24-224`, `_md-comp-nav-rail*.scss` | `sidebar.*` (partial) | platform-facts.md §2.12 background_color cell (rail) |
| MAT-N14 | Search bar | container colour/height/shape/elevation, input text and supporting text fonts/colours, leading/trailing icons, avatar, spaces; Expressive contained search (margins, gaps, spring) | *published*: `latest/_md-comp-search-bar.scss:26-130` | `input.*` (partial) | — |
| MAT-N15 | Search view (docked, full-screen) | container colour, elevation, shapes (docked/full-screen), header height, input/supporting text, divider colour, contained bar/result shapes and gap | *published*: `latest/_md-comp-search-view.scss:23-101` | `popover.*` (partial) | — |
| MAT-N16 | Date picker (docked, modal, input) | container colour/size/shape/elevation; header headline and supporting text; weekday labels; date cell size/shape, selected and today (outline) colours, outside-month opacity; range selection band and headers; year selection cells; menu button; state layers | *published*: `latest/_md-comp-date-picker-docked.scss:24-234`, `_md-comp-date-picker-modal.scss:24-288`, `_md-comp-date-input-modal.scss:23-73` | none | — |
| MAT-N17 | Time picker (dial) and time input | clock dial colour/size, selector handle/track/centre, dial labels; time selector cells (size, shape, colours, font); period selector (AM/PM: size, outline, colours); headline; separator; time input fields (outline, label, supporting text) | *published*: `latest/_md-comp-time-picker.scss:24-250`, `_md-comp-time-input.scss:25-199` | none | — |
| MAT-N18 | Carousel | item container colour/shape/elevation, outline, state layers; item spacing and the masked shape change while scrolling | *published*: `latest/_md-comp-carousel-item.scss:24-78`; Compose `compose/carousel/` (to verify); *measure-compose* | none | — |
| MAT-N19 | Bottom sheet (standard, modal) | container colour, shape (top corners), surface tint, elevation per standard/modal, minimised shape; drag handle colour/size/opacity; focus indicator; drag/settle motion | *published*: `latest/_md-comp-sheet-bottom.scss:23-51`; Compose `compose-android/ModalBottomSheet.android.kt` (motion; to verify) | `popover.*`/`dialog.*` (partial) | connectors/native-theme-gpui/examples/showcase-gpui/chrome.rs:412 ("the model states no sheet") |
| MAT-N20 | Side sheet (standard, modal, detached) | container colour/width/height/shape (docked, modal, detached), divider, headline font/colour, action label and state layers, elevation | *published*: `v0_192/_md-comp-sheet-side.scss:37-91`; `latest/_md-comp-sheet-side.scss:25-89` | `sidebar.*` (partial) | — |
| MAT-N21 | Floating sheet | container colour, elevation, shape, surface tint | *published*: `latest/_md-comp-sheet-floating.scss:22-28` | none | — |
| MAT-N22 | Snackbar | container colour/shape/elevation, single- and two-line heights, supporting text font/colour, action label colour and state layers, close icon colour/size and state layer; show durations (short, long) and enter/exit motion | *published*: `latest/_md-comp-snackbar.scss:24-108`; durations `compose/SnackbarHost.kt:298-306`; motion *measure-compose* | none | — |
| MAT-N23 | Banner (basic, rich; standard, vibrant) | container colour/shape/elevation, heights per line count (mobile/desktop), body/title text, icon/image sizes and shape, action spaces, close button, divider, colours per style | *published*: `latest/_md-comp-banner.scss:24-140`, `_md-comp-banners.scss:20-42`, `_md-comp-banners-basic.scss:21-83`, `_md-comp-banners-rich.scss:22-86` | none | — |
| MAT-N24 | Loading indicator (Expressive) | active indicator colour and size, contained variant (container colour, size, shape), the morphing-shape sequence and its timing | *published*: `latest/_md-comp-loading-indicator.scss:21-37`; shapes and motion Compose `compose/LoadingIndicator.kt`, `MaterialShapes` (to verify paths); *measure-compose* | `spinner.*` (partial) | — |
| MAT-N25 | Toolbars (Expressive): docked, floating, floating with FAB | docked container colour/height/shape/spacing; floating container colour (standard, vibrant), elevation, height/width per orientation, external padding; button/icon/label colours per state; FAB in toolbar sizes; show/hide motion | *published*: `latest/_md-comp-toolbar-docked.scss:21-35`, `_md-comp-toolbar-floating.scss:22-72`, `_md-comp-toolbar-floating-fab.scss:22-50`, `_md-comp-toolbar-standard.scss:22-86`, `_md-comp-toolbar-vibrant.scss:22-86`; motion `mw/labs/gb/components/toolbar/_toolbar-tokens.scss:26-27` (labs) | `toolbar.*` (top app bar) | platform-facts.md §2.13 item_gap cell |
| MAT-N26 | Full-screen dialog | container colour/shape/elevation, header (height, colour, headline font, icon, action label + state layers, on-scroll colour/elevation), action bar, divider | *published*: `latest/_md-comp-full-screen-dialog.scss:24-192` | `dialog.*` (partial) | — |
| MAT-N27 | Autocomplete (filled, outlined) | text field + attached menu (all text-field and menu parts in one token set) | *published*: `latest/_md-comp-filled-autocomplete.scss:24-272`, `_md-comp-outlined-autocomplete.scss:24-268` | `input.*` + `menu.*` (partial) | — |
| MAT-N28 | Pull-to-refresh indicator | indicator container colour, size, elevation, its spinner/loading shape; pull threshold and motion | *published*: Compose `compose/pulltorefresh/PullToRefresh.kt` (to verify); *measure-compose* | none | — |
| MAT-N29 | Scrim (standalone) | Withdrawn (review 2026-10-28): duplicate of MAT-X23, the same `md.comp.scrim` colour and opacity with the same source; see MAT-X23. colour and opacity of the modal backdrop (also MAT-X23) | *published*: `latest/_md-comp-scrim.scss:20-22` | none | — (review 2026-10-28: withdrawn, see MAT-X23) |
| MAT-N30† | Floating text toolbar (text selection actions) | The floating toolbar Android shows over a text selection (cut, copy, paste, select all, and the actions the system adds): container fill, shape, elevation, item font and colours, overflow, its placement against the selection. Compose hands it to the framework's floating `ActionMode`, so the system theme draws it, not M3 tokens | *published*: androidx `compose/ui/ui/src/androidMain/kotlin/androidx/compose/ui/platform/AndroidTextToolbar.android.kt:20-46` (`ActionMode`, `FloatingTextActionModeCallback`); *measure-compose* | none (`menu.*`, `popover.*` nearest) | — |
| MAT-N31† | Scroll field (Expressive wheel picker) | A vertical wheel of values with three items visible: container colour (enabled, disabled) and shape, height, unselected / selected / disabled value colours, the value's text style interpolated between two typescale roles as an item snaps to the centre, the colour and size animation, focus indication through the ripple's focus-ring shape | *published*: `compose/ScrollField.kt:161-282` (the component), `:351-420` (`ScrollFieldDefaults`: height, shape, colours), `:426-460` (`Item`: interpolated text style, motion-scheme animation) | none | — |

---

## 4.3 Properties that cross widgets

| ID | Property | What is missing | Source | Nearest model field | Seed |
|---|---|---|---|---|---|
| MAT-X1 | colour roles | `latest` lists 50 `md.sys.color` tokens; fewer than a third map to a `defaults` colour field. Missing: the tertiary family (tertiary, on-tertiary, tertiary-container, on-tertiary-container); the containers primary-container, on-primary-container, secondary-container, on-secondary-container, error-container, on-error-container; the fixed roles (primary/secondary/tertiary-fixed, -fixed-dim, on-*-fixed, on-*-fixed-variant); surface-dim, surface-bright, the surface-container ladder (lowest, low, container, high, highest); inverse-surface, inverse-on-surface, inverse-primary; outline-variant; scrim; surface-variant; surface-tint; background/on-background | *published*: `latest/_md-sys-color.scss:20-206` (light), `latest/_md-sys-color__dark.scss:20-206`; `v0_192/_md-sys-color.scss:19-135`; Compose `compose/ColorScheme.kt`; *read* on Android 12+ (MAT-X17) | `defaults.*_color` (accent, surface, muted, border, danger …) | platform-facts.md §2.1.3 surface_color cell |
| MAT-X2 | elevation levels | Six levels as distances (level0–level5) and each component's level per state | *published*: `v0_192/_md-sys-elevation.scss:17-22`; `latest/_md-sys-elevation.scss:20-30`; Compose `compose/tokens/ElevationTokens.kt:24-39` | `border.shadow_enabled` (bool) | platform-facts.md §2.1.6 border.shadow_enabled cell |
| MAT-X3 | shadow geometry | Every layer of an elevation shadow: material-web's key shadow (y offset, blur, no spread, opacity) and ambient shadow (y offset, blur, spread, opacity) as functions of the level; Android draws instead a RenderNode ambient + spot shadow from a light source (light position and radius, ambient and spot alpha) | *published*: `mw/elevation/internal/_elevation.scss:63-175` (`:104-105` key, `:173-174` ambient); *read*/published Android: theme attributes `android:ambientShadowAlpha`, `android:spotShadowAlpha` and framework light dimensions (AOSP `frameworks/base/core/res/res/values/dimens.xml` — to verify); *measure-compose* | `defaults.shadow_color` (one colour) | the task statement |
| MAT-X4 | per-state elevation | Hover / focus / pressed / dragged / disabled / lowered elevation per component (buttons, FABs, cards, chips, list items) | *published*: `v0_192/_md-comp-filled-tonal-button.scss:38-82`; `_md-comp-elevated-card.scss:35-55`; `latest/_md-comp-fab-primary.scss` (`lowered.container.elevation`) | none | — |
| MAT-X5 | shadow colour per component | `container-shadow-color` per component (all `shadow` today, but a separate token) | *published*: e.g. `v0_192/_md-comp-filled-tonal-button.scss:40`, `_md-comp-menu.scss:33` | `defaults.shadow_color` (global) | — |
| MAT-X6 | surface tint / tonal elevation | The surface-tint colour and the tint layer an elevated container receives | *published*: `latest/_md-sys-elevation.scss:32` (`surface-tint-color`), `latest/_md-sys-color.scss:202-206`, `*.container.surface-tint-layer.color` in `latest/_md-comp-menu.scss:69` and others; Compose `compose/ColorScheme.kt` (`surfaceColorAtElevation`, to verify) | none | — |
| MAT-X7 | elevation motion | Shadow transition duration and easing when elevation changes | *published*: `mw/button/internal/_elevation.scss:18-20`; `mw/fab/internal/_shared.scss:177-179`; `mw/elevation/internal/_elevation.scss:38-52` (inherits the host's transition) | none | — |
| MAT-X8 | state-layer model | The state layer as data: hover, focus, pressed, dragged (and latest's disabled) opacities, and per component the state-layer colour per state, drawn over any container; the model stores precomposited per-widget colours for hover and pressed only — no focus or dragged state, and no way to recomposite over another container. The two sources disagree on focus/pressed opacity (web 0.12, Compose 0.1) | *published*: `v0_192/_md-sys-state.scss:17-20`; `latest/_md-sys-state.scss:18-26`; Compose `compose/tokens/StateTokens.kt:22-25`, `compose/Ripple.kt:328-338` | `*.hover_background`, `*.active_background` (precomposited, partial) | `material-3.toml` header ("A state layer is its colour at the md.sys.state opacity") |
| MAT-X9 | ripple | The pressed feedback: a circle growing from the touch point (grow time, easing, initial scale, padding, soft edge), minimum press time, touch delay, hover fade, release fade-out | *published*: `mw/ripple/internal/ripple.ts:17-24, :88, :277-293, :370-371`; `mw/ripple/internal/_ripple.scss:67-90`; `mw/tokens/_md-comp-ripple.scss:18-36`; Android RippleDrawable *measure-compose* | none | — |
| MAT-X10 | focus indicator | Inner (inward) offset; ring shape following the component (per-corner shape tokens); the focus animation (grows to an active width then settles; duration and its 25 / 75 % split, emphasized easing) and its reduced-motion form; per-component overrides of colour, thickness, offset | *published*: `mw/tokens/_md-comp-focus-ring.scss:20-52`; `mw/focus/internal/_focus-ring.scss:34-46, :108-110`; `latest/_md-sys-state-focus-indicator.scss:18-22`; per component `latest/_md-comp-button.scss:60-64`, `_md-comp-switch.scss:92-96`, `_md-comp-checkbox.scss:115-119` | `defaults.focus_ring_color`, `_width_px`, `_offset_px` (partial) | — |
| MAT-X11 | shape scale and per-corner shapes | The shape scale's steps (none, extra-small, small, medium, large, large-increased, extra-large, extra-large-increased, extra-extra-large, full); the corner families (`-top`, `-start`, `-end`) that round only some corners; `full` as "half the height at any size" rather than a fixed number | *published*: `v0_192/_md-sys-shape.scss:17-31`; `latest/_md-sys-shape.scss:18-66`; Compose `compose/tokens/ShapeTokens.kt`, `compose/Shapes.kt` | `border.corner_radius_px`, `corner_radius_lg_px` (one radius for all corners, a fixed number) | platform-facts.md §2.1.6 |
| MAT-X12 | shape morphing (Expressive) | Shape changes driven by state (pressed, selected) and their spring; the morphing shape library (loading indicator, FAB menu, icon buttons) | *published*: `latest/_md-comp-button.scss:96-116`, `latest/_md-comp-icon-button-*.scss:21-51`; Compose `MaterialShapes` / `androidx.graphics.shapes` (to verify) | none | — |
| MAT-X13 | motion tokens | 16 duration tokens; easing curves (standard, standard-accelerate/-decelerate, emphasized, emphasized-accelerate/-decelerate, legacy ×3, linear); Expressive springs (default/fast/slow × spatial/effects, damping and stiffness); the standard vs expressive motion scheme | *published*: `v0_192/_md-sys-motion.scss:17-53`; `latest/_md-sys-motion.scss:18-106`; Compose `compose/MotionScheme.kt:56-132`, `compose/tokens/StandardMotionTokens.kt`, `ExpressiveMotionTokens.kt`, `MotionTokens.kt`; `mw/internal/motion/animation.ts:10-20` (the web's emphasized approximation) | `dialog.entrance` (one enum) | — |
| MAT-X14 | transition patterns | Container transform, shared axis, fade through, fade — what moves, durations, easing | *published*: m3.io motion / transitions (to verify); material-components-android transition classes (to verify); *measure-compose* | none | — |
| MAT-X15 | per-component motion | Every widget animation recorded in §4.1 (MAT-B11, B13, C8, I14, M9, S6, P6, TB6, SB9, SW8, DG6, SN6, SG1, CB3) and §4.2 — the model has no motion slot on any widget | see each row | none | — |
| MAT-X16 | typography in `Font` | Letter spacing (tracking), line height per font, variable axes, font features; brand vs plain typeface | *published*: `v0_192/_md-sys-typescale.scss` (`*-tracking`, `*-line-height`); `latest/_md-sys-typescale.scss:20-42`; `latest/_md-ref-typeface.scss:18-26` | `Font` (family, size, weight, style, colour) | — |
| MAT-X17 | dynamic colour | On Android 12+ the scheme is read from the system (wallpaper / user-chosen source colour and style): the whole role set per mode; the API-34 path reads the roles directly; the scheme variant (tonal spot, vibrant, expressive, …) is a user choice | *read*: Compose `compose-android/DynamicTonalPalette.android.kt:34-35, :191-230, :292-293, :404-405` (`dynamicLightColorScheme`, `dynamicDarkColorScheme`); material-color-utilities `DynamicScheme` (to verify) | the preset's fixed colours | docs/todo.md:3841 (Android reader, post-1.0) |
| MAT-X18 | contrast levels | Standard, medium and high contrast schemes (light and dark), each a full set of roles | *published*: `latest/_md-sys-color__medium-contrast.scss:20-206`, `__high-contrast.scss:20-206`, `__dark__medium-contrast.scss:20-206`, `__dark__high-contrast.scss:20-206`; *read*: `UiModeManager.getContrast()` (Android 14) | `AccessibilityPreferences.high_contrast` (flag, no data) | platform-facts.md §2.1.7 high_contrast cell |
| MAT-X19 | font scaling | Android 14 non-linear font scaling (large sizes scale less) and the bold-text weight adjustment | *read*: `Configuration.fontScale`, `FontScaleConverter` (API 34), `Configuration.fontWeightAdjustment` (API 31) | `AccessibilityPreferences.text_scaling_factor` (linear factor only) | — |
| MAT-X20 | reduced motion | What reduced motion changes per animation: material-web drops the focus animation, the slider label transition and the tab indicator slide; Android scales or removes animator durations | *published*: `mw/focus/internal/_focus-ring.scss:108-110`, `mw/slider/internal/_slider.scss:95-97`, `mw/tabs/internal/tab.ts:159-212`; *read*: `Settings.Global.ANIMATOR_DURATION_SCALE`, `TRANSITION_ANIMATION_SCALE` (the "remove animations" setting's effect to verify) | `AccessibilityPreferences.reduce_motion` (flag, no per-animation data) | — |
| MAT-X21 | forced colours (web high contrast) | material-web's forced-colors rules: which parts switch to system colours and gain outlines | *published*: forced-colors blocks in `mw/{button,checkbox,radio,switch,slider,tabs,menu,list,field,textfield,chips,fab,dialog,divider,iconbutton,progress,ripple}/internal/*.scss` (e.g. `mw/slider/internal/forced-colors-styles.scss`, `mw/fab/internal/forced-colors-styles.scss`) | none | — |
| MAT-X22 | density, touch target, size variants | 48 minimum touch target around small controls; the density scale; the Expressive size ladder (xsmall…xlarge) across buttons, icon buttons, button groups, split buttons, sliders, FABs | *published*: `mw/checkbox/internal/_checkbox.scss:61-79`; Compose minimum interactive size `compose/InteractiveComponentSize.kt:47-71` (`minimumInteractiveComponentSize`); density m3.io (to verify); size files in MAT-B9, N5, N6, N7, S11, N1 | none (one size per widget) | gap-map §2.4 ("Material density") (review 2026-10-28: Compose file settled, as Cross-check B, C10) |
| MAT-X23 | scrim | Modal backdrop colour and opacity (dialogs, modal drawer, modal sheets, search view) | *published*: `latest/_md-comp-scrim.scss:20-22`; `latest/_md-comp-navigation-drawer.scss:36, :156` | none | — |
| MAT-X24 | disabled per part | Disabled container, label, icon, outline opacities as separate values per component (list label 0.3 differs from the 0.38 elsewhere); latest's disabled state-layer opacity | *published*: `v0_192/_md-comp-filled-tonal-button.scss:42-46, :89-91`; `v0_192/_md-comp-list.scss:43-57`; `latest/_md-sys-state.scss:18` | `*.disabled_*` colours (no icon/outline colours on most widgets), `disabled_opacity` | platform-facts.md §2.1.6 disabled_opacity cell |
| MAT-X25 | visible timings | Tooltip auto-hide; snackbar short/long durations; submenu open/close delays; ripple touch delay and minimum press; long-press and double-tap timeouts; caret blink; scroll-indicator fade | *published*: `compose/internal/BasicTooltip.kt:465`; `compose/SnackbarHost.kt:304-306`; `mw/menu/internal/submenu/sub-menu.ts:241, :266`; `mw/ripple/internal/ripple.ts:18, :88`; *read*: `ViewConfiguration.getLongPressTimeout()`, `getDoubleTapTimeout()`, `getScrollDefaultDelay()`, `getScrollBarFadeDuration()`; caret blink Compose foundation (to verify) | `tooltip.show_delay_ms` only | — |
| MAT-X26 | caret and selection handles | Caret width and blink; selection handle shape and colour | see MAT-I12, MAT-I13 | `input.caret_color` | — |
| MAT-X27 | icon style | Material Symbols axes (fill, weight, grade, optical size); filled icons for active destinations (nav bar, rail, drawer); per-state icon colours across components | *published*: `mw/tokens/_md-comp-icon.scss:15-16` (`font`, `size`); Material Symbols axes on fonts.google.com/icons (to verify); filled-when-active m3.io navigation pages (to verify) | `icon_set`, `defaults.icon_sizes.*` | — |
| MAT-X28 | open / expanded state | No widget can state an open look: select arrow rotation, menu button, FAB menu, search view, expandable list item | see MAT-CB3, N3, N8, N15, L9 | none | — |
| MAT-X29 | dragged state | Dragged state layer and elevation (cards, chips, list items, drag handle) | *published*: `v0_192/_md-comp-filled-card.scss:43-45`; `v0_192/_md-comp-list.scss:59-69`; `latest/_md-comp-filter-chip.scss` (dragged.*) | none | — |
| MAT-X30 | error state | Error colours across text field, select, checkbox (beyond `defaults.danger_color`) | see MAT-I9, CB1, C5 | `defaults.danger_color` | — |
| MAT-X31 | pointer precision | Compose adjusts for precise pointers (mouse) vs touch | *published*: `compose-android/PrecisionPointer.android.kt` (what it changes: to verify) | none | — |
| MAT-X32† | touch and wheel thresholds | The distance a touch moves before it becomes a scroll or drag (touch slop, paging slop), the double-tap slop, the scroll distance of one mouse-wheel step | *read*: `ViewConfiguration.getScaledTouchSlop()`, `getScaledPagingTouchSlop()`, `getScaledDoubleTapSlop()`, `getScaledVerticalScrollFactor()`; Compose `androidx.compose.ui.platform.ViewConfiguration` (`touchSlop`; to verify) | none | — |
| MAT-X33† | pointer | The cursor each component shows (pointer over interactive controls, text cursor over a field, default when disabled), and Android's pointer appearance settings (size, colour; per API level, to verify) | *published*: `mw/button/internal/_shared.scss:21, :75`; `mw/textfield/internal/_shared.scss:34, :38`; `mw/checkbox/internal/_checkbox.scss:54, :58`; `mw/list/internal/listitem/_list-item.scss:51, :91`; Compose `Modifier.pointerHoverIcon`: links show `PointerIcon.Hand` (androidx `compose/foundation/foundation/src/commonMain/kotlin/androidx/compose/foundation/text/TextLinkScope.kt:237-238`), its use in M3 and foundation text fields to verify; *read*: Android pointer settings (keys to verify) | none | — (review 2026-10-28: the link cursor settled, as *Pass 3*'s *Found on the way*) |
| MAT-X34† | focus indication: when and which style | material-web shows the focus ring only for keyboard focus (`:focus-visible`); Compose has two focus styles, a state-layer opacity (the default) and an inset ring drawn as two strokes (an outer and an inner stroke, each with an inset and a width), the ring taking a shape each component supplies | *published*: `mw/focus/internal/focus-ring.ts:83`, tests `mw/focus/internal/focus-ring_test.ts:163-176`; `compose/Ripple.kt:85-90` (`focusRingShape`), `:342-361` (`OpacityFocusThemeConfiguration`, `InsetFocusRingThemeConfiguration`), `:398-421` (`RippleThemeConfiguration.Focus`) | `defaults.focus_ring_*` (one ring, no visibility rule, no second style) | — |
| MAT-X35† | accessibility: Android display settings | What Android's display accessibility settings change in an M3 app: high-contrast text (an outline or plate drawn round every text run), colour inversion, colour correction, and force dark applied to a light app | *read*: the high-contrast-text, inversion and correction settings (keys to verify); `android:forceDarkAllowed`, `View.setForceDarkAllowed()` (to verify); *measure-compose* | `AccessibilityPreferences` (none of these) | — |
| MAT-X36† | system typefaces | The faces Android supplies where M3 names none or the app keeps the defaults: the platform sans (Compose `FontFamily.Default`) and monospace (`FontFamily.Monospace`); a device can ship faces other than Roboto. material-web falls back to the browser's generic families | *read*: `Typeface.DEFAULT`, `Typeface.MONOSPACE` (family and metrics, by the Android reader; to verify what is exposed); *published*: AOSP `frameworks/base/data/fonts/fonts.xml` (`aosp/` of *Fonts and mirroring*): the `sans-serif` family `:38`, the `monospace` family `:253-254` and its aliases `:256-257` | `defaults.font.family`, `defaults.mono_font` | platform-facts.md §2.1.2 cell (line 1040) (review 2026-10-28: fonts.xml lines settled) |
| MAT-X37† | optical centring | Content inside an asymmetric shape (a split-button half with different inner and outer corners) is moved off the geometric centre by a coefficient of the corner-size difference, within a maximum offset | *published*: `compose/HorizontalCenterOptically.kt:29-46`, `:62`, `:93` (`CenterOpticallyCoefficient`); used in `compose/SplitButton.kt:807, :816, :933, :942` | none | — |
| MAT-X38† | screen-level placement | Where the scaffold puts the FAB (end or centre; its spacing from the edge and above the bottom bar) and the snackbar (above the FAB and the bottom bar) | *published*: `compose/Scaffold.kt:169-242` (placement), `:317` (`FabPosition`), `:368-370` (`FabSpacing`) | none | — |

### Not a gap, recorded for the value phase

- `text_selection_background` / `input.selection_background`: Compose Material 3 *does* state one,
  `primary` at `TextSelectionBackgroundOpacity` (`compose/MaterialTheme.kt:276-287`), against
  platform-facts.md §2.1.3 and §2.4 "(none)". The field exists, so this is a value correction,
  not a model gap.
- Scrollbar values (§2.8) have no Material source; Android's View scroll indicator size is
  *read* (`ViewConfiguration.getScaledScrollBarSize()`), a value source, not a model gap.
  (Review 2026-10-28: the first half is superseded by pass 2: Compose Material 3 states
  scrollbar values, `compose/Scrollbar.kt:119-148`, MAT-SC3† and *Found wrong in pass 1*.)

## Counts

- 4.1: 167 rows (window 3, button 15, input/text area 16, checkbox/radio 9, menu 11, tooltip 7,
  scrollbar 2, slider 12, progress bar 7, tabs 8, sidebar 9, toolbar 7, list/table 14, popover 2,
  splitter 3, separator 1, text scale 5, layout 1, switch 9, dialog 9, spinner 6, combo box 3,
  segmented 4, card 3, expander 1; status bar and link: no gap).
- 4.2: 29 components.
- 4.3: 31 cross-widget properties.
- (Review 2026-10-28: these are pass 1's counts; the rows marked † come after them, and MAT-DG5
  and MAT-N29 are withdrawn as duplicates of MAT-X23.)

## Cross-check (pass 2, 2026-10-28)

This pass checked the map against the five other surveys (every row of the five other platform files), against
the Material column's "(none)" cells of `platform-facts.md` (at caeddd6b), and against two
catalogues of what Material 3 has: the component token files of `latest/` (one file per
component, `tokens/versions/latest/sass/`) and the file list of `compose/` at androidx
`1e6776853023ebe4266aca2b308305723e3a2d57` (read 2026-10-28 through the GitHub contents API).
"cat." below means both catalogues. Rows added in this pass carry a † after their ID. The
androidx foundation and ui files the new rows cite are at the same commit, their paths written
out in full.

### A. Controls other surveys list as missing

Decision: 1 = Material draws it as a standalone control (a §4.2 row); 2 = drawn inside an element
the map already has (rows checked); 3 = Material has no such control.

| Control | Decision | Rows added, or reason and source |
|---|---|---|
| stepper/spin box | 3 | no token file in `latest/`, no Compose component (cat.); the nearest are the scroll field (MAT-N31†) and the time input (MAT-N17) |
| calendar | 2 | drawn only inside the date picker: month grid, today, selection, range band are MAT-N16; no standalone calendar in cat. |
| level indicator/gauge | 3 | none in cat.; the progress indicators (MAT-P*, MAT-SN*) are the nearest |
| menu bar | 3 | none in cat.: Compose's `Menu.kt` is the dropdown menu, material-web has `md-menu` only |
| page control/pager | 3 | no page indicator in cat.; Compose's pager is foundation layout with no M3 indicator, and the carousel (MAT-N18) draws none |
| preference rows/boxed list | 2 | settings rows are list items with trailing controls: MAT-L1–L3; the Expressive segmented list (rounded items with gaps, MAT-L8) is the boxed-list look |
| avatar | 2 | no standalone avatar in cat.; avatars are parts of list items (MAT-L2), the app bar (MAT-TA5), input chips (MAT-N9) and the search bar (MAT-N14), each with its size, shape and colour there |
| breadcrumb/path control | 3 | none in cat. |
| colour button | 3 | none in cat. |
| group box/frame | 3 | none in cat.; cards (MAT-CA1) group content |
| help button | 3 | none in cat. |
| image frame | 3 | none in cat.; carousel items mask images (MAT-N18) |
| status page/placeholder | 3 | none in cat. |
| about dialog | 3 | none in cat. |
| colour picker | 3 | none in cat. (material-web's `catalog/src/components/hct-slider.ts` belongs to its documentation site, not to the components) |
| dial/circular slider | 3 | none in cat.; the time picker's clock dial is part of MAT-N17 |
| find bar | 3 | none in cat.; the search bar and view (MAT-N14, N15) are the nearest |
| navigation drawer/pane | 2 | the navigation drawer is the model's `[sidebar]`: MAT-SB1–SB9; the rail is MAT-N13, the side sheet MAT-N20 |
| password field | 2 | Compose's `SecureTextField` / `OutlinedSecureTextField` are text fields (MAT-I1–I16 apply); what they add: **MAT-I19†** |
| rating | 3 | none in cat. |
| token field | 2 | the token is the input chip (MAT-N9); a field holding chips is app layout, with no token in cat. |
| view switcher | 2 | navigation bar (MAT-N12), tabs (MAT-TB1–TB8), segmented buttons (MAT-SG1–SG4) |
| access key badge | 3 | neither Android nor material-web draws access-key hints; no token in cat. |
| action sheet | 2 | a modal bottom sheet (MAT-N19) holding list items (MAT-L*) |
| back button | 2 | the app bar's navigation icon button: MAT-TA2 (icon colours), MAT-N4 (icon button); predictive-back motion MAT-W2 |
| background extension | 3 | a Liquid Glass feature; M3 has no translucent surface (B, C03) |
| bottom accessory | 3 | an iOS 26 tab-bar accessory; none in cat. |
| collection view | 3 | no grid-item component in cat. (Compose's lazy grids are foundation layout with no M3 styling) |
| column browser | 3 | none in cat. |
| context menu preview | 3 | none in cat.; Compose's `DropdownMenu` shows no lifted preview |
| dock/panel title bar | 3 | none in cat. |
| drag and drop | 2 | M3 draws the dragged state (MAT-X29: state layer and elevation of cards, chips, list items) and the list reorder drop zone (MAT-L10); the drag shadow Android draws across windows is the dragged view's own rendering (review 2026-10-28: settled, `View.DragShadowBuilder` "Creates an image that the system displays during the drag and drop operation", by default with "exactly the same appearance as the given View" (`a-ref`, API 11); the window manager adds only its motion, MAT-P4-8†) |
| edit menu | 1 | **MAT-N30†** (the framework's floating text toolbar) |
| file chooser | 3 | Android's document and photo pickers are system activities another app draws; none in cat. |
| font chooser | 3 | none in cat. |
| glass container | 3 | no translucent material in M3 (B, C03) |
| ink toolbar | 3 | none in cat. |
| inspector/utility panel | 2 | the side sheet (MAT-N20) is M3's supplementary-content panel |
| keyboard | 3 | the soft keyboard is drawn by the input-method app, not the M3 app; an app-side appearance setting is not known (to verify) |
| large content viewer hud | 3 | none in cat. |
| list box | 2 | a list with selectable items: MAT-L4, MAT-L5 |
| media transport controls | 3 | none in cat. |
| paste control | 3 | none in cat.; Android has no in-app paste button |
| picker wheel | 1 | **MAT-N31†** (Compose `ScrollField.kt`; no token file in `latest/`) |
| rubber band | 3 | none in cat.; neither material-web nor Compose M3 has marquee selection |
| rule editor | 3 | none in cat. |
| scroll edge effect | 2 | the scrolled app bar (MAT-TA1) and overscroll (MAT-SC2, **MAT-SC4†**); M3 has no edge blur |
| scrollbar annotation | 3 | none in cat. |
| swipe actions | 2 | the Expressive list reveal (MAT-L11); Compose's `SwipeToDismissBox` (in cat.) draws app-supplied background content — whether it adds M3 colours of its own: to verify |
| teaching tip | 2 | the rich tooltip (MAT-T1) with its optional caret (MAT-T4) |
| text ruler | 3 | none in cat. |
| tool box | 3 | none in cat. |
| tool button | 2 | icon buttons (MAT-N4, N5) in app bars (MAT-TA2) and toolbars (MAT-N25) |
| toolbar flyout | 2 | toolbar overflow: MAT-N25, **MAT-TA8†** |
| touch bar | 3 | none in cat. (Mac hardware) |
| tree view | 3 | none in cat.; the expandable list item (MAT-L9) has one level |
| writing tools | 3 | none in cat. |
| zoom button | 3 | none in cat. |

Tally: 58 controls — decision 1: 2, decision 2: 17, decision 3: 39.

### B. Capabilities

| Capability | Rows added, or why none |
|---|---|
| C01 | MAT-W4†, MAT-I17†, MAT-N30† |
| C02 | no further row — levels, per-state elevation and tint are MAT-X2–X7, X29 and the component rows |
| C03 | no further row — neither token set has a blur, translucency or backdrop token (no match for "blur" or "translucen" in `v0_192/` or `latest/`) and material-web sets no `backdrop-filter`; M3 surfaces are opaque tonal colours, the scrim (MAT-X23) the only see-through layer. Whether any Compose M3 component uses Android's window blur: to verify (none in the files read) |
| C04 | MAT-SC3† (thumb colour as a role at an opacity) |
| C05 | no further row — MAT-X1 lists every `md.sys.color` role; there are no status roles (C, lines 1086–1091) |
| C06 | MAT-X35† (force dark); contrast levels are MAT-X18, and M3 has no inactive-window or elevated colour axis (MAT-W3; tonal surfaces MAT-X6) |
| C07 | MAT-W4†, MAT-N31†, MAT-X33† |
| C08 | MAT-X34† (Compose's focus is a state-layer opacity unless the inset ring is chosen) |
| C09 | MAT-X34† (two focus styles); the variant families are otherwise in MAT-B1, I11, CA1, M10, TB1, N4, N9 |
| C10 | no further row — MAT-X22, X31 and each Expressive size row; X22's Compose file "to verify" is `compose/InteractiveComponentSize.kt:47-71` (`minimumInteractiveComponentSize`) |
| C11 | MAT-W5†, MAT-I17†, MAT-N30†, MAT-N31†, MAT-X37† |
| C12 | MAT-X34† (the two-stroke inset ring) |
| C13 | MAT-W4†, I17†, I18†, I19†, SC3†, SC4†, TA8†, N30†, N31† |
| C14 | MAT-SC3†, SC4†, N31† |
| C15 | MAT-X34† |
| C16 | no further row — caret width and blink are MAT-I12 (the blink can be switched off: `LocalCursorBlinkEnabled`, androidx `compose/foundation/foundation/src/commonMain/kotlin/androidx/compose/foundation/text/TextFieldCursor.kt:49`), the error caret MAT-I9, the handles MAT-I13 |
| C17 | MAT-I17†, MAT-I18†; the text-selection fill is stated (C, line 1062) |
| C18 | MAT-N31†, MAT-X36† |
| C19 | no further row — symbol axes and filled-when-active icons are MAT-X27, per-state icon colours the component rows (MAT-B7, M3, TB4, L3, SB3, SW3, CA3, I7); M3 has no icon theme and no icon state effect beyond the disabled opacity (MAT-X24) |
| C20 | MAT-W4†, MAT-W5† |
| C21 | MAT-I19†, SC3†, X32† |
| C22 | MAT-X33† |
| C23 | MAT-X35† |
| C24 | MAT-N30†, MAT-N31† |
| C25 | MAT-W4†, W5†, I19†, SC4†, X32†, X33†, X35†, X36† |
| C26 | MAT-SC3†, TA8†, DG10†, X37†, X38† |
| C27 | no further row — no gradient token in either set; material-web paints gradients as devices only: the slider's tick dots (`mw/slider/internal/_slider.scss:480-487`, `mw/tokens/_md-comp-slider.scss:69-70`) and the ripple's soft edge, a radial gradient (`mw/ripple/internal/_ripple.scss:70-77`). The second is visible and is MAT-X9's "soft edge"; X9's tag lacks C27 (tags are not rewritten) |
| C28 | no further row — M3's colour roles are sRGB hex values (`latest/_md-sys-color.scss`), dynamic colour yields ARGB integers (MAT-X17); no wide-gamut or HDR role |

### C. "(none)" cells of the Material column

a = Material does not draw it, the cell is right; b = Material draws it — the row says where its
value comes from, or "value correction" where a source states it after all. Lines are
`platform-facts.md` at caeddd6b.

| Line(s) | a/b | Row, or reason |
|---|---|---|
| 1040 | b | MAT-X36† — M3 names no monospace face, but Android supplies one (`FontFamily.Monospace`) |
| 1041–1043 | a | no monospace role; a monospace run takes the size, weight and style of the role it is set in |
| 1058 | b | value correction, no row (review 2026-10-28: was "a"): no link role, but Compose Material 3's `Text` gives every link that states no style `colorScheme.primary` and an underline (`compose/Text.kt:330-332`, `:673-695`), as this file's *Link* note says; `TextLinkStyles.kt:28-30` (androidx `compose/ui/ui-text/src/commonMain/kotlin/androidx/compose/ui/text/`: a null style leaves the link styled as the text around it) is Compose UI's rule, which M3's `Text` overrides; MDC's View `TextView` colours links with `android:textColorLink` — what Theme.Material3 sets there is a value to verify (`link.font.color` exists) |
| 1061 | a | no unfocused-selection state in M3; whether an unfocused Compose field keeps its selection highlight: to verify (*measure-compose*) |
| 1062 | b | value correction, no row: Compose states it, `primary` at `TextSelectionBackgroundOpacity` (`compose/MaterialTheme.kt:276-287`; this file's "Not a gap" note) |
| 1063 | a | `TextSelectionColors` holds only `handleColor` and `backgroundColor` (`compose/MaterialTheme.kt:279-281`): selected text keeps its colour |
| 1086–1091 | a | no warning, success or info roles (`latest/_md-sys-color.scss:20-206`), and no component draws such a state |
| 1116, 1117 | a | no global radius: each component names a shape step (MAT-X11) |
| 1120 | a | the outline roles are opaque |
| 1127 | b | MAT-X19 (*read*: `Configuration.fontScale`) |
| 1128 | b | MAT-X20 (*read*: the animator duration scale) |
| 1129 | b | MAT-X18 (*read*: `UiModeManager.getContrast()`) |
| 1130 | a | M3 has no translucent surface to make opaque (B, C03) |
| 1137, 1138 | a | no small or large icon context; icon sizes are per component (MAT-B7, M3, TB4, L2, N1, N5) |
| 1147 | a | no icon theme; the icon set and its axes are MAT-X27 |
| 1158, 1159 | b | MAT-W4† (a desktop-windowing frame; whether it has an outline: to verify); a full-screen app window has no frame |
| 1160–1167 | b | MAT-W4† (the desktop-windowing caption bar: fill, title, focused and unfocused); a phone app draws its top app bar instead (MAT-W3) |
| 1168 | b | MAT-W5† |
| 1169 | b | MAT-W4† |
| 1170, 1171 | a | no window padding; the system-bar insets are layout read per device, their look is MAT-W1 |
| 1180 | a | tonal and filled buttons have no outline; the outlined button's is MAT-B3 |
| 1210 | b | value correction, no row: as line 1062 |
| 1211 | a | as line 1063 |
| 1220 | a | the outlined field has no container; the filled field's disabled container is MAT-I11 |
| 1228 | a | the label is body text (mdc `textAppearanceBodyMedium`, the cell), no checkbox-specific value |
| 1237, 1238 | a | the mark fills the box; the halo and touch target are MAT-C2, MAT-C9 |
| 1247 | a | transparent, as the cell states |
| 1263 | a | menus have no outline (shadow and tonal fill: MAT-PO1) |
| 1287 | a | no outline |
| 1304, 1305, 1307–1310 | b | the cells are wrong for Compose: `compose/Scrollbar.kt:119-148` states the thumb colour, thickness, minimum length and the fading (overlay) behaviour — a value correction for the fields that exist, and **MAT-SC3†** for what the model lacks; material-web draws no scrollbar of its own |
| 1306, 1311 | a | the Compose scrollbar is visual only (`compose/Scrollbar.kt:47-50`): no hover or active state |
| 1339 | a | no outline |
| 1347 | a | the active tab is not filled; its indicator is MAT-TB2 |
| 1350 | a | fixed tabs share the width; the scrollable minimum is stated in the cell (Compose) |
| 1354 | a | no outline; the row divider is MAT-TB5 |
| 1360 | a | the tabs abut |
| 1366 | a | stated `corner-none` |
| 1375, 1376 | a | no drawer edge line in v0_192; `latest`'s nested `divider-color` is deprecated for the divider component (`latest/_md-comp-navigation-drawer.scss:80-83`) |
| 1391, 1392 | a | stated in `latest` (the cell); v0_192 has none — a token-set choice, not a gap |
| 1393 | a | a fixed height with content centred: no vertical padding exists |
| 1396 | a | no outline or divider; the scrolled state is MAT-TA1 |
| 1404–1415 | a | M3 has no status bar; Android's system status bar is MAT-W1 |
| 1424 | a | no row striping |
| 1438 | a | `comp.list` has no outline token; the data table's outline is MAT-L13 |
| 1457 | a | no outline |
| 1468, 1470 | a | stated in `latest` (the drag handle): MAT-SP1, MAT-SP2 |
| 1503 | a | no layout-manager gap in either token set or in Compose |
| 1504, 1505 | b | MAT-LA1 (margins per window-size class: m3.io to verify, *measure-compose*) |
| 1506 | b | MAT-LA1, MAT-SP3 (the pane spacer per size class) |
| 1535, 1536 | a | the selected track has no outline (`mw/switch/internal/_track.scss:55-84`) |
| 1538–1540 | a | the handle has no outline |
| 1566 | a | no outline |
| 1600 | a | no minimum: one size per variant (MAT-SN3) |
| 1620 | a | no arrow area; the icon and its trailing space are stated |
| 1625 | a | the outlined select's hover recolours the outline (the cell); the filled select's hover state layer is MAT-I11 / MAT-CB1 |
| 1626 | a | no container; the filled select's disabled container is MAT-I11 / MAT-CB1 |
| 1658 | a | filled and elevated cards have no outline; the outlined card's is stated |
| 1673–1688 | a | no expander: `latest/_md-comp-list-expand.scss:21-36` is a token set that no implementation at the pinned commits draws (material-web has no such component; Compose's `ListItem.kt` and `ListItemDefaults.kt` do not use `ExpandedListTokens`); its colours and shapes are MAT-L9 / MAT-EX1 |
| 1693, 1695, 1697–1701 | a | no link component; links inside text: line 1058; a hovered, focused or pressed link style exists only when the app passes one (androidx `compose/foundation/foundation/src/commonMain/kotlin/androidx/compose/foundation/text/TextLinkScope.kt:280-297`) |
| 1694, 1696 | b | value corrections, no row (review 2026-10-28: were "a"): `font.color` and `underline_enabled`, the `primary` colour and underline Compose Material 3's `Text` gives links (line 1058) |

Tally: 125 cells (the 124 of the input list and line 1625) — a: 95, b: 30. Eleven of the b cells are value
corrections, a source states the value after all: lines 1058, 1062, 1210, 1694 and 1696 need no row, the
scrollbar's six (1304, 1305, 1307–1310) come with MAT-SC3† for what the model lacks. (Review
2026-10-28: was a 98, b 27 and eight corrections, before lines 1058, 1694 and 1696 were corrected.)

### Found wrong in pass 1

- **Scrollbar.** Compose Material 3 has a scrollbar at the pinned commit
  (`compose/Scrollbar.kt`, `Modifier.nonInteractiveScrollbar`, `NonInteractiveScrollbarDefaults`),
  against this file's note under §2.8 ("M3 has no scrollbar component") and platform-facts.md
  §2.8's Material cells; MAT-SC3† records the gaps, the stated values belong to the value phase.
- **Missed components** in Compose's file list: the scroll field (MAT-N31†) and the secure text
  field (MAT-I19†); Compose's inset focus-ring style (MAT-X34†) is missing from MAT-X10.
- **Tag:** MAT-X9's ripple soft edge is a radial gradient (C27).
- **A "to verify" settled:** MAT-X22's Compose minimum interactive size is
  `compose/InteractiveComponentSize.kt:47-71`.

## Seed follow-up (2026-10-28)

Rows for the four seed groups no row covered after pass 2 (G12, G19, G24, G27; the main
map's §5). The source abbreviations are defined in
`todo_v0.7.1_model-completeness-gap-map-seeds.md`, *Seed follow-up sources*. CAPS and SRC
are the row's capability tags (main map §4.3) and source kinds.

### Code editor and rich text (G12, G27)

**Code editor: not on this platform.** Neither catalogue the Material file uses has one: the
component token files of `latest/` (app bar … top app bar, no code or text-view component) and the
93 entries of `compose/` at 1e677685 (the text ones are `Text.kt`, `TextField.kt`,
`TextFieldDefaults.kt`, `OutlinedTextField.kt`, `SecureTextField.kt`), both read 2026-10-28 through
the GitHub contents API.

**Rich text / Markdown: not on this platform**, by the same two catalogues: no Markdown component
and no token for code, quotes or list markers. Compose Material 3's `Text(AnnotatedString)` draws
rich text from the app's spans and styles only its **links**: colour `colorScheme.primary` and
underline, for links that state no style, with no hover, focus or pressed style
(`compose/Text.kt:673-695`). The model has both fields (`defaults.link_color`,
`link.underline_enabled`), so it is not a model gap; but the Material file's *Link* section says
"No Material gap: M3 has no link component" and `platform-facts.md` §2.28 states no Material link
values, which this source contradicts: value work for §7, not a row.

---

### Drop zones and dialog placement (G19, G24)

#### G19 — drop zone under a dragged pane, tab or dock

Covered by: MAT-L10 (list reorder: dragged item colours, drop-zone colour, shadow, shape), MAT-SP1 / MAT-SP2
(pane drag handle geometry and states), MAT-X29 (dragged state layer and elevation).

Not on this platform: a pane dragged onto another pane. Compose's adaptive panes are resized (the pane
expansion drag handle) or levitated, never dropped on one another: the adaptive-layout package at
`1e677685` lists no drag-and-drop or drop-target type (`compose/material3/adaptive/adaptive-layout/src/commonMain/kotlin/androidx/compose/material3/adaptive/layout/`:
`AdaptStrategy.kt` … `WindowSizeClassHelper.kt`, `PaneExpansionDraggableModifier.kt`, `DragToResizeState.kt` —
resize only; to verify against the sources' contents). Material tabs have no drag-to-reorder in material-web (no `drag` or `drop` anywhere in `mw/tabs/`, by grep).

#### G24 — where a modal dialog is placed

Covered by: MAT-DG8 (the 48 margin to the window edges at maximum size), MAT-N26 (the full-screen dialog's
look), MAT-N19 (the bottom sheet's look), MAT-N20 (side sheet).

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| MAT-SF01† | 4.1 | [dialog] | Dialog: position | Centred both ways: material-web's host is `position: fixed` with `margin: auto`; Compose's `Dialog` window has `Gravity.CENTER`. | P: `mw/dialog/internal/_dialog.scss:37-45`; `ax/compose/ui/ui/src/androidMain/kotlin/androidx/compose/ui/window/AndroidDialog.android.kt:660`; M: measure-web, measure-compose | none | C26 | P, M |
| MAT-SF02† | 4.1 | [dialog] | Dialog: when it becomes full-screen (the breakpoint) | The rule for using the full-screen dialog (MAT-N26) instead of the basic one on compact windows, and the window-width class where it applies; neither material-web nor Compose switches automatically (to verify). | P: m3.io dialogs and window size classes (to verify, m3.io unreadable without JavaScript, as the Material file records) | none | C26, C10 | P, V |
| MAT-SF03† | 4.2 | bottom sheet | Modal bottom sheet: position on wide windows | Attached to the bottom edge, horizontally centred, at most `SheetMaxWidth` 640 dp wide, so wide windows leave side gaps. MAT-N19 lists the sheet's look, not this. | P: `ax/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/SheetDefaults.kt:536`; `…/ModalBottomSheet.kt:100`, `:159-162` | none | C24, C26 | P |
| MAT-SF04† | 4.3 | placement | Levitated pane (adaptive scaffolds) | A pane with `AdaptStrategy.Levitate` floats above the scaffold "like a pop-up or a sheet (for example, as a bottom sheet or a side sheet)": its alignment (default `Alignment.Center`) and an optional scrim (`LevitatedPaneScrim`). | P: `ax/compose/material3/adaptive/adaptive-layout/src/commonMain/kotlin/androidx/compose/material3/adaptive/layout/AdaptStrategy.kt:77-120` | none | C26, C24 | P |

---

## Relabelled controls (2026-10-28)

Pass 2 asked the platforms about 18 controls under a wrong control name (a tagging
error: GN-2.40–2.46, GN-2.49, K2.7, K2.15–2.17, K2.22, Q2.43, W2-13, W2-14, W2-19,
W2-25), so this check asked again for this platform. Decisions: 1 a new §4.2 row, 2 a
new §4.1 row, 3 a row already covers it (the same job, possibly done another way),
4 the platform has no such thing.

"cat." as in the Material file's Cross-check table A: the component token files of `latest/` (material-web
`cbd34a8921915af94d5ef65c2a69eece41d5b4f3`, `tokens/versions/latest/sass/`, listed again 2026-10-28 through the
GitHub contents API: 195 files) and the file list of `compose/` at androidx
`1e6776853023ebe4266aca2b308305723e3a2d57` (`xcm/compose-ls.json`). System-drawn Android UI follows the file's
convention: what the framework draws inside the app's window counts (MAT-N30†, MAT-I13, MAT-I17†); what another
process draws outside it does not (table A, "keyboard").

| Control | Decision 1–4 | Row added, row covering it, or the source showing it is absent |
|---|---|---|
| keyboard shortcut keycap (GN-2.40) | 3 | MAT-M4 (a menu item's trailing supporting text, the shortcut, its font and colour). Material draws shortcuts only as that text: no keycap component in cat. (no keycap or shortcut token file in `latest/`, no such file in `compose/`) |
| shortcuts dialog (GN-2.41) | 4 | None in cat. Android's keyboard-shortcuts helper (an app supplies its groups through `Activity.onProvideKeyboardShortcuts`, `Activity.requestShowKeyboardShortcuts()`, API 24) is drawn by the system, not by the M3 app (to verify who draws it on current Android versions) — outside the app like the keyboard (table A) |
| text selection handles (touch) (GN-2.42) | 3 | MAT-I13 (handles, `compose/MaterialTheme.kt:276-281` `handleColor`; geometry *measure-compose*), MAT-I17† (magnifier), MAT-X26 (handle shape and colour); the selection's action bar is MAT-N30† |
| emoji chooser (GN-2.43) | 4 | None in cat. The keyboard's emoji panel is drawn by the input-method app (as table A's "keyboard"). androidx `emoji2/emoji2-emojipicker` (`EmojiPickerView`) is a Jetpack View library outside the M3 component set (whether it takes M3 colours: to verify) |
| video play overlay (GN-2.44) | 4 | None in cat.; table A "media transport controls" is 3 for the same reason. The list item's leading video (MAT-L2, `latest/_md-comp-list.scss:57-60, :73-74, :93-96`) states only the video's size, no play glyph |
| editable label (GN-2.45) | 4 | None in cat.: no in-place label editor in `latest/` or `compose/`; editing is the text field (MAT-I*) |
| wrap box (GN-2.46) | 1 | new 4.2 row: Chip set (`md-chip-set`) |
| drag icon (GN-2.49) | 3 | MAT-X29 (dragged state layer and elevation of cards, chips, list items, drag handle), MAT-L10 (dragged list item's colours, shadow colour, shape); the drag shadow Android draws across windows is the app's image, its return and cancel motion MAT-P4-8† (table A, "drag and drop") (review 2026-10-28: table A's "to verify" settled from `View.DragShadowBuilder`, `a-ref`) |
| command link button (K2.7) | 4 | None in cat.: the button token files are elevated, filled, tonal, outlined, text and the Expressive sizes (MAT-B1, MAT-B9); none has a description line |
| title widget (K2.15) | 4 | None in cat.: no title/heading component; page titles are the app bar's headline (MAT-TA3, MAT-TA7) and the typescale roles (MAT-TS1) |
| side tab bar (K2.16) | 4 | None in cat.: the tab files are `primary-navigation-tab` and `secondary-navigation-tab` (`latest/`), and `compose/TabRow.kt` has `PrimaryTabRow`, `SecondaryTabRow`, `PrimaryScrollableTabRow`, `SecondaryScrollableTabRow` (`:150`, `:201`, `:253`, `:324`) and the deprecated `TabRow`, `ScrollableTabRow` (`:1334`, `:1400`), all horizontal rows: the scrollable ones scroll with `horizontalScroll` (`:575`, `:850`), none with `verticalScroll`. The side-edge navigation, the rail (MAT-N13), switches destinations, not tool views (review 2026-10-28: "has only" corrected, the deprecated two added; the scrollable rows settled) |
| splitter collapser (K2.17) | 4 | None in cat. Compose's adaptive panes are resized by the drag handle (MAT-SP1, MAT-SP2; `compose/DragHandle.kt`), whose state sets the first pane's proportion in 0..1 (androidx `compose/material3/adaptive/adaptive-layout/src/commonMain/kotlin/androidx/compose/material3/adaptive/layout/PaneExpansionState.kt:466-470`); the adaptive-layout file list has no collapse button |
| wizard (K2.22) | 4 | None in cat.: no stepper or multi-step component in `latest/` or `compose/` |
| drag handle (Q2.43) | 3 | MAT-SP1, MAT-SP2 (`latest/_md-comp-drag-handle.scss:23-53`, incl. the pressed width, height and shape; Compose `compose/DragHandle.kt`), MAT-N19 (the bottom sheet's handle), MAT-L10 (the reorder list item, `latest/_md-comp-list-reorder.scss:22-45`; the grip glyph is the app's trailing icon, coloured by `trailing-icon.color` at :42-43) |
| grid view items (W2-13) | 4 | None in cat. (table A, "collection view": Compose's lazy grids are foundation layout with no M3 styling); tiles drawn as cards are MAT-CA*, carousel items MAT-N18 |
| items view (W2-14) | 3 | MAT-L4, MAT-L5 (item states, selected × state, disabled opacities), MAT-L3 (trailing element colours, incl. a trailing checkbox's slot); as for table A's "list box". A grid layout of items has none (row above) |
| repeat button (W2-19) | 4 | None in cat.: no auto-repeating button in `latest/` or `compose/` (the button and icon-button files state no repeat delay or interval) |
| radio group (W2-25) | 4 | None in cat.: `latest/` has only `radio-button`, `compose/` only `RadioButton.kt`, and material-web groups radios by `name` with no group element; no group header or spacing exists to carry. The radio itself is MAT-C1, MAT-C2, MAT-C8, MAT-C9 |

Tally: 1 — 1 (Chip set); 2 — 0; 3 — 5 (keycap, text selection handles, drag icon, drag handle, items view); 4 — 12.

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| MAT-RL01† | 4.2 | chip set | Chip set (`md-chip-set`) | The container that lays chips out in rows that wrap: the wrap rule and the gap between chips, one `gap` for both directions (between chips in a row and between rows). MAT-N9 states each chip, not the space between them. Compose has no chip-set component (an app places chips in foundation's `FlowRow` with its own spacing), so what a Compose app shows is the app's choice — to verify against the M3 chips guidance (m3.io, to verify) | *published*: `mw/chips/internal/_chip-set.scss:7-11` (`flex-wrap: wrap`, `gap`), element `mw/chips/chip-set.ts:25`; no token in `latest/` (cat.) | `layout.widget_gap_px` (unrelated, partial) | C24, C26 | P, V |

## Fonts and mirroring (2026-10-28)

A completeness check of the map found two gaps no row covered: the emoji font and
the fallback chain for scripts the UI font lacks (all six platforms), and right-to-left
layout where this file had no general row. The source abbreviations are defined in
`todo_v0.7.1_model-completeness-gap-map-seeds.md`, *Fonts and mirroring sources*.

**Emoji and fallback.** Not covered: MAT-X36† is the platform sans and monospace typefaces from
`fonts.xml`, MAT-X16 tracking, line height, axes and features — neither names the emoji font or the
per-script fallback. material-web has no fallback of its own: its typeface tokens are single names
(`md.ref.typeface.brand` and `.plain`, both `Roboto`, `latest/_md-ref-typeface.scss:18-21`), so in a browser
the emoji and fallback fonts are the host system's (the other five sections) or the browser's; the rows
below are Android's.

**Right-to-left.** No row covers it. The m3.io guidance could not be read (`m3.io`), so what follows is
from the two implementations and Google's icon guide. Beyond mirroring, three things are drawn
differently: icons with a question mark mirror in Arabic and Farsi but not in Hebrew (a per-language
rule, `GFI`), anything that shows time is drawn moving right to left (`GFI`), and material-web's text field
can keep a left-to-right direction inside an RTL layout (`textDirection`, "Useful for RTL languages that
use LTR notation for fractions", `mw/docs/components/text-field.md:502`). No RTL branch was found in either
implementation's circular progress indicator (a search of `mw/progress/internal/_circular-progress.scss`
and `compose/ProgressIndicator.kt`, whose two branches are the linear indicator's), so it is not mirrored;
whether that is the guidance's rule is to verify on m3.io.

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| MAT-FM01† | 4.3 | typography | Emoji font | Android's emoji fonts are the `fonts.xml` families tagged `lang="und-Zsye"`: `NotoColorEmoji.ttf` and `NotoColorEmojiFlags.ttf` (`NotoColorEmojiLegacy.ttf` is listed with `ignore="true"`), placed after the CJK families in `fallback_order.json`. A Compose Material 3 `Text` or `TextField` (Compose UI 1.4 and later) draws emoji through emoji2, which brings "the latest emoji version" down to API 21, so on older devices an M3 app can draw a newer emoji font than the system's; `EmojiSupportMatch.None` turns it off. `fonts.xml` is marked deprecated in favour of the APIs in the next column. The model has no emoji family. | P: `aosp/fonts.xml:1585-1593`, deprecation `:3-21`; `aosp/fallback_order.json:89-96`; `c-emoji`; R: `AFontMatcher_match` with `AFontMatcher_setLocales` on an emoji string (API 29), `ASystemFontIterator_open` / `SystemFonts.getAvailableFonts()` (API 29) (`ndk`); M: *measure-compose*, the sample set in an M3 `Text` on an API ≤ 30 and a current emulator image | none (`defaults.font.family`); related MAT-X36† | C18, C25 | P, R, M |
| MAT-FM02† | 4.3 | typography | Fallback chain per language | "Fonts are chosen based on a match: full BCP-47 language tag including script, then just language, and finally order (the first font containing the glyph)". The CJK families `zh-Hans`, `zh-Hant,zh-Bopo`, `ja` and `ko` are different faces (`index`) of the same `NotoSansCJK-Regular.ttc`, so Han glyphs follow the text's locale; the order of the script families is `fallback_order.json`'s. How a Compose `TextStyle`'s `localeList` reaches the matcher is to verify. The model has one family and no language. | P: `aosp/fonts.xml:24-26`, `:1411-1584`; `aosp/fallback_order.json:1-136`; R: `AFontMatcher_match` per locale (`ndk`); M: *measure-compose*, the sample set per locale | none (`defaults.font.family`) | C18, C25 | P, R, M |
| MAT-FM03† | 4.3 | mirroring | Right-to-left layout | Android reports the layout direction (LTR or RTL); Compose lays out by `LocalLayoutDirection`; material-web follows the CSS `direction` and states its geometry in logical properties, its corner shapes per logical corner (`start-start`, `start-end`, `end-start`, `end-end`), so MAT-X11's `-start` / `-end` corner families swap sides. Components that draw RTL explicitly: the slider (active-track clip, thumb offset, drag direction, and in Compose the track's inside and outside corner radii swap), the linear progress indicator (mirrored as a whole; in Compose the bar and the stop indicator), the tab indicator's offset, the navigation drawer (swipe direction, predictive-back transform origin), the date-range selection highlight, the menu's horizontal offset, the outlined field's resize handle, the search bar's predictive-back offset. Cross-check: MAC-X34†, W3-54†, X-SV-3. | R: `Configuration.getLayoutDirection()` (`a-ref`); P: `mw/internal/controller/is-rtl.ts:15-20`; `mw/slider/internal/_slider.scss:202-208`, `:441-455`, `:517-519`; `mw/progress/internal/_linear-progress.scss:172-174`; `mw/field/internal/_outlined-field.scss:325-329`; `mw/menu/internal/controllers/surfacePositionController.ts:100`; `mw/tabs/internal/_tab.scss:75` (logical corners); `compose/Slider.kt:888-897`, `:2422-2495`; `compose/ProgressIndicator.kt:409-411`, `:919-921`; `compose/TabRow.kt:735-740`; `compose/NavigationDrawer.kt:371-379`, `:917-960`, `:1263`; `compose/DateRangePicker.kt:1071-1076`; `compose/SearchBar.kt:4078`; M: *measure-web* with `dir="rtl"`, *measure-compose* under an RTL locale | none | C26, C11, C25 | R, P, M |
| MAT-FM04† | 4.3 | mirroring | Icons in right-to-left | Which icons mirror. Compose: `Icons.AutoMirrored` ("a subset of Icons that will automatically mirror themselves when displayed in an RTL layout"), used by M3 e.g. for the date picker's month arrows; Android drawables mirror when `autoMirrored`. material-web's `md-icon` never flips; `md-icon-button` flips only with `flip-icon-in-rtl`, checked once ("at startup"). Google's list of icons to mirror includes `arrow_back`, `arrow_forward`, `chevron_left`, `chevron_right`, `undo`, `redo`, `reply`, `send`, `list`, `live_help`; a separate note: icons with a question mark mirror in Arabic and Farsi but not in Hebrew. The model's icon roles that the list covers (`NavBack`, `NavForward`, `ActionUndo`, `ActionRedo`, `Help`) have no mirroring rule. | P: `CAM`; `compose/DatePicker.kt:2511`, `:2519`; `a-ref` (`Drawable.isAutoMirrored()`); `mw/iconbutton/internal/icon-button.ts:60-61`, `:98`, `:226`; `mw/iconbutton/internal/_shared.scss:86-88`; `GFI` ("Which icons should be mirrored for RTL?", the question-mark note); model `native-theme/src/model/icons.rs:78-81`, `:98-101`, `:136-137`; M: *measure-compose* / *measure-web* renders under RTL | `icon_set` (no per-role mirroring) | C19, C26 | P, M |

## Pass 3 (2026-10-28)

The rows the other platforms added after pass 1 (every ID ending in "†": their pass 2, *Seed
follow-up*, *Relabelled controls* and *Fonts and mirroring* sections; 283 rows) were asked of Material here. Decisions: 1 Material draws it and no row
covered it (a row added below), 2 a row covers it (named), 3 Material does not draw it (the
source named). "cat." is the pair of catalogues of the Cross-check's table A (the component token
files of `latest/` and the file list of `compose/`); "outside the app" is the convention of the
*Relabelled controls* section (what another process draws outside the app's window does not
count; what the system draws round or inside it, as MAT-W4†, does). (Review 2026-10-28: that
convention does not hold for every row below. Android's window decorations, desktop windowing,
split screen and starting windows are drawn by the WM Shell, which runs in the SystemUI process
(`wms/src/com/android/wm/shell/docs/overview.md:23-24`), the system bars by SystemUI: another
process. MAT-P3-1†, P3-2†, P3-3†, P3-4† and P3-6† are such shell-drawn UI, round, over or beside
the app's window, and P3-1†'s drag indicator and P3-3†'s divider lie outside it; each row says so.
Whether the model carries shell-drawn UI is the rationale's scope decision (main map §6.1), as for
MAT-W1 and MAT-W4†; the rows keep the facts for it. What another app draws (the keyboard, the
system's own dialogs and pickers) stays without a row.) New abbreviations, read
2026-10-28: `ax/` androidx `e4d933c98df7baa7ea2b83eea6e086915961e0ff`, the full path given;
`fb/` AOSP `platform/frameworks/base` @ `94b4c163b7dfe5ce3607f7bb8456f9573f7de57d` (tag `android-17.0.0_r1`) (the
`aosp/` commit of the seed sources), and `wms/` its `libs/WindowManager/Shell/` (Android's
window decorations, desktop windowing and split screen, drawn by the system round the app's
window); `a-ref` developer.android.com reference pages of the classes named in the row (they
state the API levels quoted).

| Input rows | Decision 1–3 | Row added, row covering it, or the source showing it is absent |
|---|---|---|
| GN-1.278† (back button in the header bar) | 2 | the app bar's navigation icon button: MAT-TA2 (icon colours), MAT-N4 (icon button), MAT-W2 (predictive back), MAT-FM04† (the arrow mirrors in RTL); Material has no back-history menu (none in cat.) |
| GN-1.282† (resize zones), W-11† (window states and their motion), K1.19† (quick-tile outline), MAC-W13† (tiling menu, drag tiling preview), MAC-X32† (tiling and full-screen motions; its momentum part below) | 1 | **MAT-P3-1†** (resize zones, resize veil, drag-to-snap indicator, maximised / snapped / immersive states), **MAT-P3-2†** (the maximise menu with its snap options, the app handle) |
| W-10† (Slide Over, Split View divider), MAC-W12† (full screen, title-bar reveal, Split View divider) | 1 | **MAT-P3-3†** (the split-screen divider), **MAT-P3-4†** (system bars hidden and revealed by a swipe), **MAT-P3-2†** (the app handle over a full-screen app); Android has no Slide Over panel: none among `wms/src/com/android/wm/shell/`'s top-level folders (listed 2026-10-28), whose floating window modes are `bubbles/` and `pip/` (whether a bubble's expanded window is a look this map needs: to verify) (review 2026-10-28: the whole Shell's folder list read) |
| K3.39† (full-screen enter / leave morph) | 1 | **MAT-P3-4†**, and the immersive state of **MAT-P3-1†** |
| MAC-W14† (transparent title bar), MAC-W15† (inactive title text), W1-148† (caption buttons per window kind) | 2 | MAT-W4† (the transparent caption opt-in, focused vs unfocused caption, its buttons) |
| GN-3.71† (window open, close, minimise, maximise animations) | 2 | MAT-W2 (activity and window transitions), MAT-P3-1† (desktop-windowing state transitions) |
| MAC-W11† (document title chevron, rename, Edited mark) | 3 | Android draws no document title in a window's chrome: the desktop-windowing caption holds the app chip and the caption buttons (MAT-W4†; `wms/res/values/dimen.xml:482-485`, caption button regions) |
| W1-149† (window attention flash) | 3 | no request-attention call among Android's window APIs (`Window`, `WindowInsetsController`, `a-ref`; to verify) and none in `wms/` folders read |
| K3.16† (not-responding window recoloured) | 3 | the app's window is not recoloured; Android shows an "app isn't responding" dialog the system draws (outside the app): `AppNotRespondingDialog`, in system_server (`fb/services/core/java/com/android/server/am/AppNotRespondingDialog.java:38`) (review 2026-10-28: the class settled) |
| MAC-X33† (screen safe area round the camera housing) | 1 | **MAT-P3-5†** (display cutout and safe area) |
| K3.66† (application launch feedback) | 1 | **MAT-P3-6†** (the launch splash screen) |
| GN-1.281† (IME pre-edit underline), K3.48† (pre-edit formats and caret), K1.49† (misspelling underline), W1-151† (spell-check and IME underlines) | 2 | MAT-I18† |
| GN-2.58† (password mask glyph), I-9† (secure entry bullets, last-character reveal), K1.48† (mask character and delay) | 2 | MAT-I19†; I-9†'s strong-password look is the autofill highlight, **MAT-P3-7†** (decision 1, below) |
| MAC-I09† (AutoFill button and fill colour) | 1 | **MAT-P3-7†** (the autofilled highlight); Android draws no autofill button inside the field: the suggestions come from the autofill service and the keyboard (outside the app) |
| I-10† (disabled and editing field look, height per text size) | 2 | MAT-I1, MAT-I2, MAT-I10 (focus and disabled outline, colours), MAT-I11 (filled field); the height is `input.min_height_px` (a field the model has), growing with MAT-X19's font scale |
| K1.46† (clear button) | 2 | MAT-I7 (trailing icon: size, colour per state); M3 has no built-in clear button, the app supplies the icon |
| K1.47† (block caret, overwrite caret) | 3 | Compose has one caret: a brush and the Android `DefaultCursorThickness` (`ax/compose/foundation/foundation/src/androidMain/kotlin/androidx/compose/foundation/text/TextFieldCursor.android.kt:22`), no overwrite or mask mode in `ax/…/foundation/text/` (file list, by name); material-web leaves the caret to the browser |
| GN-3.73† (caret hidden while text is selected; drop-position caret) | 1 | **MAT-P3-21†** (when the caret is drawn; its blink setting) |
| W1-150† (description text below the field) | 2 | MAT-I5 (supporting text) |
| K1.58† (check mark path and stroke inside the box) | 1 | **MAT-P3-8†** |
| CK-2† (multiselect circle) | 2 | M3 marks selected list items with a checkbox or a selected fill: MAT-L3 (trailing element), MAT-L5 (selected × state), MAT-C3; no circular multiselect mark in cat. |
| MAC-RL02† (radio group), W-RL01† (in-place label edit), K-RL05† (in-place item editor), RL-2† (inline title rename) | 3 | as *Relabelled controls* "radio group" and "editable label": none in cat. |
| GN-1.285† (labelled menu items never show their icon) | 2 | M3 menu items do show leading and trailing icons: MAT-M3; the leading space with and without an icon is `ax/…/material3/Menu.kt` `MenuArrangement` (`:2327-2370`), part of MAT-M10's item spaces |
| MN-17† (item highlight shape, menu padding, icon–text gap) | 2 | MAT-M10 (Expressive item shapes and spaces), MAT-M7 (container top / bottom space), `menu.icon_text_gap_px` |
| MN-18† (disabled item title, subtitle and icon colours), MAC-MN17† (destructive menu item) | 2 / 3 | MN-18†: 2, MAT-M3 (disabled icon opacity), MAT-M4 (supporting text), `menu.disabled_text_color`. MAC-MN17†: 3, no error or destructive key in `latest/_md-comp-menu*.scss` (grep) and no destructive item in `compose/MenuDefaults.kt` (its colours are `MenuItemColors`) |
| W3-55† (menu drop alignment by handedness), PO-7† (popover content size and arrow gap) | 1 | **MAT-P3-10†** (menu placement: candidates, margins, no handedness); PO-7†'s arrow: M3 menus have none, the rich tooltip's caret is MAT-T4; the menu's width bounds are MAT-M8 (its "maximum to verify": `DropdownMenuItemDefaultMaxWidth`, `ax/…/material3/Menu.kt:2390-2391`) |
| GN-1.286† (touch selection bubble), Q2.53† (touch cut / copy / paste bar) | 2 | MAT-N30† (the floating text toolbar) |
| (the same "edit menu" family, for a mouse) | 1 | **MAT-P3-11†** (the right-click text context menu Compose foundation draws itself); counted under GN-1.286† above, no input row of its own |
| GN-1.283† (tooltip wraps at 50 characters) | 2 | `tooltip.max_width_px` (a field the model has; Compose's plain tooltip maximum width, platform-facts.md §2.7) |
| TT-3† (tooltip placement, gap, re-show) | 2 | MAT-T4 (spacing and caret), **MAT-P4-9†** (the side against the anchor, the flip and the clamping; review 2026-10-28: MAT-T4 holds no placement, the row came in *Pass 4*); no re-show delay: Compose's tooltips share a `MutatorMutex` so one shows at a time (`compose/internal/BasicTooltip.kt:359-368`), with no warm-up state |
| K1.83† (tooltip wake-up, fall-asleep, display time, hide delay) | 2 | `tooltip.show_delay_ms`, MAT-T2 (auto-hide), MAT-T6 (long press), MAT-X25 |
| K1.84† (rich tooltip widget), K2.31† (What's This popup), C-36† and MAC-N36† (TipKit tips) | 2 / 3 | K1.84†, C-36†, MAC-N36†: 2, MAT-T1 (rich tooltip with subhead, text and action), MAT-T4 (caret). K2.31†: 3, Android has no context-help mode and no "?" caption button: the desktop-windowing caption holds the app chip, minimise, maximise and close (`wms/res/layout/desktop_mode_app_header.xml:27-121`), the freeform caption back, minimise, maximise and close (`wms/res/layout/caption_window_decor.xml:24-55`); table A "help button" (none in cat.) (review 2026-10-28: K2.31† was decision 2 under MAT-T1 / MAT-T4, but a rich tooltip is not a help mode) |
| MAC-T06† (expansion tooltip over truncated cells) | 3 | none in cat.; Compose's `Text` ellipsis shows nothing on hover |
| SB-7† (indicator minimum length, overscroll compression, pointer hover) | 2 | MAT-SC3† (minimum thumb length, no hover state), MAT-SC2 (stretch overscroll) |
| W1-152† (auto-hide scroll bar setting) | 2 | MAT-SC1 (the indicator always fades), MAT-SC3† (its always-visible option); Android has no system setting for it (none among `ViewConfiguration`'s scroll methods, `a-ref`) |
| W1-153† (range slider) | 2 | MAT-S12 (range slider), MAT-S10 (overlapping handles) |
| PB-4† (progress label and current-value label), K1.115† (capacity bar) | 3 | M3's progress indicators carry no label: no `label` key in `latest/_md-comp-progress-indicator*.scss` or `_md-comp-loading-indicator.scss` (grep); no capacity bar in cat. |
| MAC-P05† (progress track edge line) | 3 | no outline token in `latest/_md-comp-progress-indicator*.scss`; the track is a fill (MAT-P2, MAT-P5) |
| MAC-SN05† (spinner fade in on start, out on stop) | 3 | material-web's circular indicator has no opacity transition, only the arc and rotation animations (`mw/progress/internal/_circular-progress.scss:107-154`); whether Compose's `CircularProgressIndicator` fades: to verify (*measure-compose*) |
| T-17† (tab customisation), K1.133† (movable tabs), W-SF09† (tab torn out over another tab view), MAC-SF07† (window tab bar as a drop target) | 3 | tabs cannot be moved or customised: no `drag` or `drop` in `mw/tabs/` (seed follow-up G19), and `compose/TabRow.kt` has only fixed or scrollable rows (the four, and two deprecated ones; review 2026-10-28, as *Relabelled controls* "side tab bar"); MAC-SF07†: Android windows have no window tab bar (the desktop-windowing caption holds no tabs, MAT-W4†; none in `wms/src/com/android/wm/shell/windowdecor/`'s file list) |
| MAC-TB06† (tab strip edge, separators, selected fill) | 2 | MAT-TB5 (the divider under the row), MAT-TB1 / TB3 (per-state colours); no separators between tabs (Cross-check C, line 1360) |
| MAC-TB07† (tab overflow and label truncation) | 2 | **MAT-P4-6†** (the scrollable row is the overflow: its scroll to the selected tab), MAT-TB7 (the scrollable row's leading edge padding only); Compose's `Tab.kt` sets no `maxLines` on the label (grep), so truncation is the app's (review 2026-10-28: MAT-TB7 holds the edge padding only; the overflow has its row since *Pass 4*) |
| SD-9† (overlay and displace split behaviours) | 2 | MAT-SB6 (modal drawer over a scrim), MAT-SB9 (open / close motion), MAT-SF04† (levitated pane) |
| SD-10† (primary column width), MAC-SB09† (sidebar widths and collapse) | 1 | **MAT-P3-13†** (adaptive pane count and preferred widths); the drawer's own width is MAT-SB7, the rail's collapse MAT-N13 |
| K1.142† (places panel: section headers, capacity bars, eject icon) | 2 | MAT-SB4 (drawer section headline), MAT-SB5 (trailing badge label), MAT-SB3 (icons); the rest is app content |
| GN-1.279† (tree view), GN-1.280† (icon view), L-21† (outline indentation), K1.184† (column view arrows), K1.185† (tree expand animation) | 3 | table A "tree view", "collection view", "column browser": none in cat.; the expandable list item has one level (MAT-L9) |
| L-22† (table headers, sort indicator), MAC-L18† (highlighted column header) | 2 | MAT-L12 (data-table header hover colour, sorting icon) |
| L-23† (form rows), Q2.55† and Q2.56† (form cards and entries), W2-40† (settings card and expander) | 2 | list items with trailing controls: MAT-L1–L3, MAT-L8 (segmented list), MAT-L6 (dividers), MAT-L9 (expandable item) |
| MAC-L17† (controls in a selected row; disabled selection) | 2 | MAT-L5 (selected × state colours of every part, selected-disabled opacities) |
| MAC-L19† (context-menu target row outline) | 3 | none in cat.; table A "context menu preview" |
| K-RL06† (drag image of item views), W3-49† (system drag image and drop description), GN-3.66† (drop-target highlight) | 2 | MAT-X29 (dragged state layer and elevation), MAT-L10 (reorder drop zone; `drop` appears only in `latest/_md-comp-list-reorder.scss`, grep); the cross-window drag shadow is the app's image (table A, settled), its return and cancel motion MAT-P4-8†; no drop description in cat., and Android draws no operation badge on the shadow (MAT-P4-8†) (review 2026-10-28: table A's "to verify" settled, the *Pass 4* row named) |
| W1-159† (list view marquee), MAC-N31† (marquee rectangle), C-47† (band selection) | 3 | table A "rubber band": none in cat. |
| DL-13† (alert width, scrolling height, width at accessibility sizes) | 2 | `dialog.min_width_px`, `max_width_px` (fields the model has), MAT-DG8 (window margin), MAT-DG4 (dividers round scrolling content) |
| GN-1.284† (alert response order, reversed when stacked) | 2 | MAT-DG10† |
| K1.224† (message box icon), MAC-IC04† (alert app icon size), W1-156† (message box icon, instruction, footer) | 2 | MAT-DG2 (hero icon), `dialog.icon_size_px`, MAT-DG7 (subhead) |
| W1-155† (full-size content dialog), Q2.58† (dialog layer by width) | 2 | MAT-N26 (full-screen dialog), MAT-SF02† (when it is used) |
| GN-SF17†–SF21†, K-SF19†–SF23†, MAC-SF08†–SF10†, SF-7†–SF-9†, W-SF10†, W-SF11†, W-SF13† (dialog and sheet placement) | 2 | MAT-SF01† (centred), MAT-SF02† (full-screen breakpoint), MAT-SF03† (bottom sheet on wide windows), MAT-SF04† (levitated pane); a Compose `Dialog` is its own window, centred by `Gravity.CENTER` (MAT-SF01†) |
| W-SF12† (dialog with the on-screen keyboard) | 2 | MAT-SF01†'s window: by default `decorFitsSystemWindows = true` and the platform's soft-input mode moves it; with `false`, Compose sets adjust-resize (API ≤ 30) or adjust-nothing (API 31+) and the content handles the IME insets (`ax/compose/ui/ui/src/androidMain/kotlin/androidx/compose/ui/window/AndroidDialog.android.kt:106-115`, `:186`, `:846-849`); where the dialog ends up: *measure-compose*, part of SF01†'s M |
| GN-3.72† (parent dimmed under a modal), K3.15† (parent desaturated and darkened) | 2 | MAT-DG5, MAT-X23 (scrim); Android's dim behind a dialog window is `WindowManager.LayoutParams.dimAmount` (API 1) and, since version 37.2, `dimColor` (`a-ref`), a value source for X23 |
| SG-8† (momentary segmented mode) | 3 | `compose/SegmentedButton.kt` has only `SingleChoiceSegmentedButtonRow` and `MultiChoiceSegmentedButtonRow` (`:328`, `:364`); material-web's set is single- or multi-select |
| SG-9† (segment width rule, height per text size, disabled segment) | 1 | **MAT-P3-12†** (segment width rule); disabled parts MAT-SG4; height `segmented_control.segment_height_px` |
| MAC-SG10† (segment image size and tint per state), W1-154† (segmented style variants) | 2 | MAT-SG1, SG2, SG4 (icon size and colours per state); the variants are the connected button group (MAT-N6) and segmented buttons (MAT-SG*) |
| W1-157† (splitter gripper, fills per state) | 2 | MAT-SP1, MAT-SP2 (drag handle) |
| W1-158† (status bar borders) | 3 | M3 has no status bar (§2.14 of this file) |
| K1.260† (link hover colour, underline, hand cursor) | 2 | the *Link* note (§2.28: colour and underline exist in the model); Compose links have a hovered style only when the app passes one, and show `PointerIcon.Hand` (`ax/compose/foundation/foundation/src/commonMain/kotlin/androidx/compose/foundation/text/TextLinkScope.kt:237-238`, `:282-296`): MAT-X33† |
| MAC-B11† (push button minimum width per size) | 2 | `button.min_width_px` (a field), MAT-B9 (Expressive sizes) |
| K-RL07† (auto-repeat while held), GN-3.69† (spin button and trough auto-repeat), K3.76† (spin-box auto-repeat) | 3 | *Relabelled controls* "repeat button": no repeating button and no spin button in cat. (table A "stepper") |
| W-RL02† (command link button) | 3 | *Relabelled controls* "command link button" |
| B-18† (pull-down button), MAC-N29† (menu button) | 2 | MAT-N8 (menu buttons) |
| MAC-N28† (split button) | 2 | MAT-N7 |
| MAC-I10† (text view inset) | 2 | MAT-I16 (text area), `text_area.border` padding |
| MAC-TL10† (fixed and flexible toolbar spaces), K3.38† (dock and toolbar re-layout motion) | 2 | MAT-N25 (toolbar spacing, show / hide motion), MAT-TA3 (collapse on scroll), MAT-TA8† (overflow); none of them moves items as they change: the re-layout motion is **MAT-P4-7†** (the floating toolbar's content entering and leaving on expand / collapse; the app bar's overflow has no animation) (review 2026-10-28: K3.38†'s items-changing motion was not covered by the three rows) |
| MAC-TL11† (selected toolbar item) | 2 | MAT-N4 (icon-button toggle colours), MAT-N25 (button colours per state) |
| MAC-EX05† (disclosure triangle, label gap, indent, header height) | 2 | MAT-L9 / MAT-EX1 (expandable list item: chevron container, shapes) |
| MAC-DF05† (system colours no reader reads), W3-46† (inverse text colour role) | 2 | MAT-X1 (every `md.sys.color` role, inverse ones included), MAT-X17 (read from the system) |
| MAC-TS03† (tight and loose leading variants) | 2 | MAT-X16, MAT-TS1 (one line height per role); no leading variants in `latest/` (its typescale sets are the base and the emphasized, MAT-TS4) |
| MAC-LY02† (form labels beside controls) | 3 | none in cat.: no form layout; a text field's label sits inside it (MAT-I3) |
| MAC-IC05† (symbol effects, per-state variants) | 2 | MAT-X27 (filled icons for active destinations, symbol axes); no symbol effect in cat. |
| MAC-X30† (system rollover / pressed / disabled effects), W3-45† (alpha fills stored flattened) | 2 | MAT-X8 (state layer as data, recomposited), MAT-SC3† (colour at an opacity), MAT-X24 (disabled opacities) |
| MAC-X31† (Full Keyboard Access indicator), X-FO-3† (keyboard focus indicator and keyboard-only rule) | 2 | MAT-X34† (keyboard-only rule, Compose's two focus styles), MAT-X10 |
| MAC-X34† (RTL per part), W3-54† (RTL templates and glyphs), GN-FM03†, K-FM03† | 2 | MAT-FM03†, MAT-FM04† |
| MAC-X35† (compatibility mode draws the older look), MAC-X36† (idioms draw differently), MAC-X37† (versions inside a generation) | 1 | **MAT-P3-19†** (baseline vs Expressive, material-web vs Compose, the library version) |
| MAC-X38† (one-point or one-device-pixel hairlines) | 3 | M3 states thicknesses as fixed dp / px values (`latest/_md-comp-divider.scss:20-21`, `md.comp.divider.thickness`), no device-pixel hairline in cat. |
| MAC-X32†'s momentum part, W3-52† (wheel animation, inertia, overpan, snap) | 1 | **MAT-P3-16†** (fling and its deceleration); overpan is MAT-SC2 / SC4† |
| X-MO-11† (light / dark change transition) | 1 | **MAT-P3-18†** |
| X-BT-2† (double-tap interval, touch slop) | 2 | MAT-X25 (double-tap timeout), MAT-X32† (slops) |
| GN-3.68† (key repeat delay and interval) | 1 | **MAT-P3-14†** |
| GN-3.67† (pointer per element, hidden while typing), K3.65† (cursor per element, auto-hide while typing) | 1 | per element: MAT-X33†; hidden while typing: **MAT-P3-15†** |
| W3-50† (pointer colour, shadow, locator, trails, touch visualisation) | 2 | MAT-X33† (Android's pointer appearance settings, keys to verify); "show taps" is a developer option, not a look an app is given |
| K3.96† (shake to magnify the cursor) | 3 | none among the pointer settings MAT-X33† names (to verify with the keys) |
| GN-3.70† and K3.95† (visual bell) | 2 | **MAT-P4-5†** (the screen flash for notifications, calls and alarms, an accessibility setting: a full-screen overlay system_server's `FlashNotificationsController` draws over every app, Android's counterpart of a visual bell); nothing flashes for an alert sound as such (review 2026-10-28: was decision 3 "no visual bell … to verify"; the flash is drawn, `fb/services/accessibility/java/com/android/server/accessibility/FlashNotificationsController.java:421-443`, and has its row since *Pass 4*) |
| K3.84† (fractional display scale per output) | 1 | **MAT-P3-17†** (display density) |
| W3-47† (monospace font), GN-FM01†, GN-FM02†, FM-1†, FM-2†, K-FM01†, K-FM02†, MAC-FM01†, MAC-FM02†, W-FM01†, W-FM02† | 2 | MAT-X36† (monospace), MAT-FM01† (emoji), MAT-FM02† (fallback per language) |
| W3-48† (access-key underline setting) | 3 | table A "access key badge" |
| W3-51† (disabled icons dimmed) | 2 | MAT-X24 (disabled icon opacity per component) |
| W3-53† (system visual-effect switches) | 2 | MAT-X20 (the animator duration scale) |
| W3-56† (colour filter flag and kind) | 2 | MAT-X35† (colour correction, inversion) |
| RL-3† and MAC-RL01† (flow layout spacing) | 2 | MAT-RL01† (chip set) |
| GN-2.52† and K2.27† (path bar), GN-2.53† and K2.25† (colour picker), W2-42† (colour button), W2-41† (radial gauge) | 3 | table A "breadcrumb/path control", "colour picker", "colour button", "level indicator/gauge": none in cat. |
| GN-2.54†, C-42†, MAC-N30† (media transport controls) | 3 | table A "media transport controls" |
| GN-2.55† (carousel dot and line indicators), C-48† (page curl, page spacing) | 2 / 3 | 2: MAT-N18 (carousel item spacing and masked shape); 3: the carousel draws no indicator (table A "page control"), and there is no page curl (none in cat.) |
| GN-2.56† (entry completion popover), K2.29† (completion popup), C-38† (search suggestions) | 1 | **MAT-P3-9†** (the menu's size rule); the parts are MAT-N27 (autocomplete), MAT-CB3, MAT-N15 (search view suggestions); no completion delay in cat. |
| GN-2.57† (print, page setup, app chooser dialogs) | 3 | composite dialogs: Android's print dialog and share sheet are system activities (outside the app); none in cat. |
| GN-SF01†–SF13†, K-SF01†–SF11† (code editor), SF-1†, MAC-SF01† (invisibles) | 3 | *Seed follow-up*: no code editor in cat.; no text-view invisibles |
| GN-SF14†–SF16† (libpanel drop zones), K-SF18† (dock drop zone) | 3 | *Seed follow-up* G19: panes are resized or levitated, never dropped |
| K-SF12†–SF17†, W-SF01†–SF08†, SF-2†, SF-3†, SF-5†, MAC-SF02†, MAC-SF03†, MAC-SF05† (Markdown and rich text) | 3 | *Seed follow-up*: no Markdown component, no token for code, quotes or list markers |
| SF-4†, MAC-SF04† (links in Markdown text) | 2 | the *Link* note (§2.28) and K1.260†'s row above |
| SF-6†, MAC-SF06† (HTML imported into an attributed string) | 1 | **MAT-P3-20†** (Compose's `AnnotatedString.fromHtml`) |
| GN-RL01†, W-RL03† (wizard) | 3 | *Relabelled controls* "wizard" |
| C-37†, MAC-N33†, MAC-N34† (menu bar) | 3 | table A "menu bar" |
| C-39†, K2.26† (file chooser) | 3 | table A "file chooser" |
| C-40† (ink toolbar) | 3 | table A "ink toolbar"; Android's stylus handwriting in text fields draws no tool picker |
| C-41† (inspector column) | 2 | MAT-N20 (side sheet) |
| C-43† (navigation item styles) | 2 | MAT-TA3 (app bar variants), MAT-TA6 (embedded search) |
| C-44† (document launch view) | 3 | none in cat.: no document browser in `latest/` or `compose/` |
| C-45†, MAC-N37† (chart) | 3 | none in cat.: no chart, axis or series token file in `latest/`, no chart in `compose/` |
| C-46† (activity rings) | 3 | none in cat. |
| C-49† (editing HUD) | 3 | none in cat.; text editing actions are the floating toolbar (MAT-N30†) and the context menu (MAT-P3-11†), no undo HUD |
| MAC-N27† (text input system UI: accent popup, character viewer, IME candidates, Look Up) | 3 | the candidates, accents and emoji panel are the keyboard's (table A "keyboard", outside the app); a lookup goes through the text toolbar's actions (MAT-N30†) |
| K-RL04† (character selector) | 3 | none in cat.; the emoji chooser likewise (*Relabelled controls*) |
| K-RL01† (command bar) | 3 | none in cat.: no command palette in `latest/` or `compose/`; the search view (MAT-N15) is the nearest |
| K-RL02†, RL-1† (shortcuts dialog and HUD), K-RL03† (emoji selector) | 3 | *Relabelled controls* "shortcuts dialog", "emoji chooser" |
| K2.23† (date picker), K2.24† (time picker) | 2 | MAT-N16, MAT-N17 |
| K2.28† (font chooser), K2.30† (find bar), K2.32† (text ruler), Q2.57† and MAC-N32† (about dialog), MAC-N35† (status page) | 3 | table A "font chooser", "find bar", "text ruler", "about dialog", "status page/placeholder" |
| Q2.54† (pull-to-refresh) | 2 | MAT-N28 |
| W2-43† (token / chip) | 2 | MAT-N9 (input chip), MAT-RL01† (chip set) |

Tally: 283 input rows — decision 1: 32, decision 2: 126, decision 3: 125. A row whose parts split
is counted once: MN-18† 2, MAC-MN17† 3, GN-2.55† 2, C-48† 3, and MAC-X32† (two lines) 1. 21 rows added: §4.1 13, §4.2 1, §4.3 7. (Review 2026-10-28: decision 2: 127, decision 3: 124, after the visual-bell line (GN-3.70†, K3.95†) moved from 3 to 2 and K2.31† from 2 to 3; on the split tooltip line K1.84†, C-36†, MAC-N36† count under 2, K2.31† under 3.)

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| MAT-P3-1† | 4.1 | [window] | Window (Android desktop windowing): resizing, snapping, window states | What the system draws round a freeform window besides its caption bar (MAT-W4†): the resize zones along the edges (a width outside the window and one inside it) and at the corners (a larger corner zone for touch), with the cursor they show (MAT-X33†); the veil drawn over the app while it resizes (the app icon on a fill); the indicator that fades in and grows while a window is dragged towards the top or a side (to full screen, to the left or right half, to desktop) and fades out; the maximised, half-snapped, immersive and minimised states and the transitions between them (MAT-W2 names activity transitions only). All of it is drawn by the WM Shell in the SystemUI process, not by the app (*Pass 3* intro); the drag indicator lies outside the app's window, a shell preview like those main map §6.1 leaves to the rationale. | P: `wms/src/com/android/wm/shell/windowdecor/DragResizeWindowGeometry.java:85-109`; `wms/res/values/dimen.xml:573-592` (`desktop_mode_resize_veil_icon_size`, `freeform_edge_handle_outset`, `freeform_edge_handle_inset`, `freeform_resize_handle`, `desktop_mode_corner_resize_large`, `freeform_resize_corner`); `wms/src/com/android/wm/shell/windowdecor/ResizeVeil.kt`; `wms/src/com/android/wm/shell/desktopmode/DesktopModeVisualIndicator.java:67-77` (indicator types), `:292-327` (fade in and out), `:372` (`FULLSCREEN_INDICATOR_DURATION`); transitions in `wms/src/com/android/wm/shell/desktopmode/` (`DesktopImmersiveController.kt`, `ToggleResizeDesktopTaskTransitionHandler.kt`, `minimize/`; to verify); M: *measure-compose* on a desktop-windowing emulator image | `window.border.*` (none of these) | C20, C14, C22, C26 | P, M (review 2026-10-28: marked shell-drawn; scope main map §6.1) |
| MAT-P3-2† | 4.1 | [window] | Window (Android desktop windowing): maximise menu and app handle | The menu the caption's maximise button opens (snap left, snap right, maximise or restore, immersive): its width per number of options, height, corner radius, shadow, and the option buttons' outline radius, stroke and fill per state (pressed, focused, selected, hovered). On a full-screen or split-screen task, the app handle: a small caption drawn over the app's top edge, and the handle menu it opens (rows for app info, windowing modes, new window, screenshot and others: heights, spacing, corner radius, elevation, shadow); on a freeform window the caption's app chip opens the same handle menu. The caption shows the same buttons for every task (only a system flag hides the minimise button); what varies is the maximise menu: its snap options only for a resizeable task, its immersive option only for a task that requests immersive mode. Drawn by the WM Shell in the SystemUI process, not by the app, as popups anchored to the caption or the handle (*Pass 3* intro; scope main map §6.1). | P: `wms/src/com/android/wm/shell/windowdecor/MaximizeMenu.kt:95-99`, `:304-317`, `:702-704`, `:765-768`; `wms/src/com/android/wm/shell/windowdecor/HandleMenu.kt`; `wms/res/values/dimen.xml:458-462` (`desktop_mode_fullscreen_decor_caption_height`, `_width`), `:485-520` (maximise menu), `:523-570` (handle menu); `wms/src/com/android/wm/shell/windowdecor/DesktopModeWindowDecorViewModel.java:904-913` (the handle and the app chip, `open_menu_button`, both open the handle menu); `wms/src/com/android/wm/shell/windowdecor/DesktopModeWindowDecoration.java:966-984` (the maximise menu's width by its option count), `:1242-1244` (immersive and snap options); `wms/src/com/android/wm/shell/windowdecor/viewholder/AppHeaderViewHolder.kt:223`, `:273` (minimise hidden by the `enableMinimizeButton` flag); `wms/res/layout/desktop_mode_app_header.xml:27-121` (app chip, minimise, maximise, close); M: *measure-compose* (desktop-windowing and tablet images) | none | C20, C13, C11, C01, C07 | P, M (review 2026-10-28: marked shell-drawn; review 2026-10-28, pass 4: the app chip's handle menu, the same caption buttons for every task and the maximise menu's per-task options added) |
| MAT-P3-3† | 4.1 | [window] | Window (Android split screen): the divider | Two apps side by side or stacked are separated by a divider the system draws: bar width and colour, the rounded corners it draws into both app windows, the handle (width, height, colour) that grows while touched and shrinks on release (durations and interpolators), the lift elevation while dragged, its touch region (smaller in portrait desktop mode), and the fling to a snap position on release. Drawn by the WM Shell in the SystemUI process between the two apps' windows, outside either, not by an app (*Pass 3* intro; scope main map §6.1). | P: `wms/src/com/android/wm/shell/common/split/DividerView.java:78-79`, `:228-234`, `:397`, `:404`, `:303`, `:386-397`, `:777-778`; `…/common/split/DividerHandleView.java:86-97`, `:169-173`; `…/common/split/DividerRoundedCorner.java:53-56`; `wms/res/values/dimen.xml:86-106`; the colours `docked_divider_handle`, `split_divider_background` in `wms/res/values/colors.xml:21-22`; M: *measure-compose* in split screen | `splitter.*` (an in-app splitter, not this) | C20, C13, C14, C01, C11 | P, M (review 2026-10-28: marked shell-drawn; the colours' file settled) |
| MAT-P3-4† | 4.1 | [window] | Window: system bars hidden (immersive) | An app can hide the status and navigation bars; with `BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE` (API 30) or `BEHAVIOR_DEFAULT` (API 31) a swipe from the edge brings them back over the content for a while: how they slide in and out, how long they stay, what they draw over the app. In desktop windowing an immersive window hides its caption (MAT-P3-1†). MAT-W1 has the bars' look, not this. The app asks for the hiding; the bars and their motion are drawn by SystemUI over the app's window, not by the app (*Pass 3* intro; scope main map §6.1). | R: `WindowInsetsController.hide()`, `setSystemBarsBehavior()` and the two constants (`a-ref`); M: *measure-compose* (screen recording) | none | C20, C14, C21 | R, M (review 2026-10-28: marked system-drawn) |
| MAT-P3-5† | 4.3 | layout | Display cutout and safe area | The camera cutout: its bounding rectangles, path and safe insets per side are readable; the window's cutout mode decides whether it may extend into the cutout area (default `LAYOUT_IN_DISPLAY_CUTOUT_MODE_DEFAULT`; `…_ALWAYS` from API 30); what the system draws where a window does not extend (the system's, not the app's): to verify. An M3 `Scaffold` pads its content by `ScaffoldDefaults.contentWindowInsets`, which on Android are the system bars' insets united with the cutout's. MAT-W5† has the display's rounded corners, not the cutout. | R: `DisplayCutout.getBoundingRects()`, `getSafeInsetTop()` and the other sides (API 28), `getCutoutPath()` (API 31), `WindowInsets.Type.displayCutout()` (API 30), `WindowManager.LayoutParams.layoutInDisplayCutoutMode` (API 28) (`a-ref`); P: `ax/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Scaffold.kt:96`, `:310-312`; `ax/compose/material3/material3/src/androidMain/kotlin/androidx/compose/material3/internal/SystemBarsDefaultInsets.android.kt:25-26` (`systemBars.union(displayCutout)`); M: *measure-compose* on an emulator profile with a cutout | none | C26, C20, C25 | R, P, M (review 2026-10-28: whether the Scaffold's insets include the cutout settled) |
| MAT-P3-6† | 4.1 | [window] | Window: the launch splash screen (Android 12+) | While an app starts, the system draws its starting window: the background colour (from the app's theme, else computed from `windowBackground`), the centred icon (animated or not), a background behind the icon, a branding image at the bottom, and the exit into the app. They are theme attributes, so they follow the app's theme; whether MDC's `Theme.Material3` sets any: to verify. The starting window is drawn by the WM Shell in the SystemUI process from the app's theme, before the app draws, not by the app (*Pass 3* intro; scope main map §6.1). | P: `android.R.attr.windowSplashScreenBackground`, `windowSplashScreenAnimatedIcon`, `windowSplashScreenIconBackgroundColor`, `windowSplashScreenBrandingImage` (API 31), `windowSplashScreenAnimationDuration` (deprecated in API 33) (`a-ref`, `R.attr`); drawn by `wms/src/com/android/wm/shell/startingsurface/SplashscreenContentDrawer.java:526-533` (the four attributes read); M: *measure-compose* (the default look and the exit, screen recording) | none | C20, C13, C14, C25 | P, M, V (review 2026-10-28: marked shell-drawn, the drawing class cited) |
| MAT-P3-7† | 4.1 | [input] | Input: autofilled highlight | A field Android Autofill has filled is marked by a highlight drawn over it: Compose foundation's text fields draw `LocalAutofillHighlightBrush` (by default a solid colour, the Android `actual`); View-based fields draw the theme's `android:autofilledHighlight` drawable (API 26). When it clears: to verify. The model has no autofilled state. The suggestions themselves come from the autofill service and the keyboard (outside the app). | P: `ax/compose/foundation/foundation/src/commonMain/kotlin/androidx/compose/foundation/text/AutofillHighlight.kt:33-37`, `:53`; `ax/compose/foundation/foundation/src/androidMain/kotlin/androidx/compose/foundation/text/AutofillHighlight.android.kt:24-25`; `R.attr.autofilledHighlight` (`a-ref`); M: *measure-compose* | none | C07, C13, C04 | P, M |
| MAT-P3-8† | 4.1 | [checkbox] | Checkbox: the check mark's geometry | The mark is drawn, not a glyph: in Compose a stroked path of three points given as fractions of the box, with a stroke width (a flag, `isCheckboxStylingFixEnabled`, moves the points), the indeterminate dash being the same path pulled to the centre line; in material-web two rectangles rotated into a check (stroke, bottom-left point, the two arms' lengths), the long one alone forming the dash. MAT-C6 asks for the dash, MAT-C3 for colours only. The stroke width is a field the model has, `checkbox.check_mark_stroke_width_px` (platform-facts.md §2.5 states both implementations' value); the gap is the mark's geometry: the points or arm lengths and the dash. | P: `ax/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Checkbox.kt:482` (`StrokeWidth`), `:653`, `:704-736` (`drawCheck`); `mw/checkbox/internal/_checkbox.scss:18-25`, `:180-207`, `:238-265` | `checkbox.check_mark_stroke_width_px` (the stroke width), `indicator_color` (colour) | C13, C12 | P (review 2026-10-28: partial gap, the stroke width is in the model) |
| MAT-P3-9† | 4.1 | [combo_box] | Select and autocomplete: the menu's size and alignment | material-web's select gives its menu at least the field's width (at most that width only with `clamp-menu-width`), anchored at the field's bottom start or end corner (`menu-align`); Compose's exposed dropdown menu takes the anchor's width by default (`matchAnchorWidth`) and limits its height to the window space left minus a margin. MAT-CB3 has the open state's colours and item height, not this. | P: `mw/select/internal/select.ts:138-141`, `:165`, `:480-492`; `mw/select/internal/_shared.scss:72-78`; `ax/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/ExposedDropdownMenu.kt:146`, `:202-213`, `:280-283`; `…/material3/Menu.kt:2324-2325` (`MenuVerticalMargin`) | `combo_box.min_width_px` (the field) | C26, C10 | P |
| MAT-P3-10† | 4.1 | [menu] | Menu: placement against its anchor | Compose places a dropdown menu from ordered candidates (start to the anchor's start, end to the anchor's end, the window's left or right; below the anchor, above it, the window's top or bottom), takes the first that fits inside the window less a horizontal and a vertical margin, and otherwise clamps the last candidate into that area, or centres the menu when it is larger; an x offset is mirrored in RTL; nothing depends on handedness. material-web places by an anchor corner and a menu corner (`anchorCorner`, `menu-corner`) and, when the menu would leave the viewport, flips the corners on the block axis (and likewise on the inline axis), keeping the flip that needs less correction, unless `no-vertical-flip` / `no-horizontal-flip` is set. | P: `ax/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/internal/MenuPosition.kt:79-154` (candidates), `:280-385` (`DropdownMenuPositionProvider`); `…/material3/Menu.kt:2324-2325`, `:2372-2373`; `mw/menu/internal/menu.ts:201`, `:210`; `mw/menu/internal/controllers/surfacePositionController.ts:55-59` (the flip switches), `:304-330` (the block flip), `:345-373` (the inline flip), `:375-380` (the `move` strategy also shifts it back inside) | none | C26 | P (review 2026-10-28: material-web's fitting rule settled) |
| MAT-P3-11† | 4.1 | [menu] | Text context menu (secondary click on text) | A right-click on a text field or a selection opens a dropdown that Compose foundation draws itself, not M3's `DropdownMenu`: its own spec (width bounds, item height, elevation, corner radius, paddings, icon size, divider and its padding, label size, weight, line height and tracking, a disabled alpha), and colours resolved from the platform's popup-menu style (`Widget_PopupMenu`'s `colorBackground`, `TextAppearance_Widget_PopupMenu_Large`'s `textColorPrimary`); M3's `MaterialTheme` provides no replacement. The touch toolbar is MAT-N30†. | P: `ax/compose/foundation/foundation/src/commonMain/kotlin/androidx/compose/foundation/contextmenu/ContextMenuUi.kt:60-79` (`ContextMenuSpec`), `:317`; `ax/compose/foundation/foundation/src/androidMain/kotlin/androidx/compose/foundation/contextmenu/ContextMenuUi.android.kt:32-63`; `ax/compose/foundation/foundation/src/androidMain/kotlin/androidx/compose/foundation/text/contextmenu/internal/PlatformDefaultTextContextMenuProviders.android.kt:59` (`ProvideBothDefaultProviders`), `…/DefaultTextContextMenuDropdownProvider.android.kt:97-166`; `ax/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/MaterialTheme.kt` (no context-menu provider, by grep); M: *measure-compose* with a mouse | `menu.*` (M3's menu, not this one) | C13, C05, C07, C01, C11 | P, M |
| MAT-P3-12† | 4.1 | [segmented_control] | Segmented button: the segments' widths | Segments share the row equally: Compose measures the row at its intrinsic minimum width and gives each segment `weight(1f)` with a minimum width of `ButtonDefaults.MinWidth`; material-web's set is a full-width grid of equal columns (`grid-auto-columns: 1fr`), each segment filling its column. The model has the height only. | P: `ax/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/SegmentedButton.kt:150-157`, `:233-237`, `:328-338`, `:364-373`; `mw/labs/segmentedbuttonset/internal/_shared.scss:15-20` (grid, `1fr` columns, `width: 100%`); `mw/labs/segmentedbutton/internal/_shared.scss:51-57` (the segment, `flex: 1` inside its host) | `segmented_control.segment_height_px` | C26, C10 | P (review 2026-10-28: material-web's equal widths come from the set's grid, citation corrected) |
| MAT-P3-13† | 4.1 | [layout] | Adaptive panes: how many, how wide | Compose's adaptive scaffolds (list-detail, supporting pane) show one, two or three panes side by side by window-width class (`maxHorizontalPartitions`), two stacked panes in tabletop posture or on a tall single-pane window (`maxVerticalPartitions`, with a vertical spacer), a preferred pane width (a larger one at the widest class) and height, and keep panes off a hinge (`excludedBounds`). MAT-SP3 and MAT-LA1 have the horizontal spacer and the margins, not these. | P: `ax/compose/material3/adaptive/adaptive-layout/src/commonMain/kotlin/androidx/compose/material3/adaptive/layout/PaneScaffoldDirective.kt:48-104`, `:127-137`, `:347-348` | `layout.*`, `sidebar.*` (no pane count or width) | C26, C10 | P |
| MAT-P3-14† | 4.3 | timings | Key repeat | The delay before a held key repeats and the interval between repeats (a held key in a text field, an arrow key in a list). material-web leaves it to the host system. | R: `ViewConfiguration.getKeyRepeatTimeout()`, `getKeyRepeatDelay()` (API 12, `a-ref`) | none | C21, C25 | R |
| MAT-P3-15† | 4.3 | pointer | Pointer hidden while typing | Android hides the mouse pointer while the user types on a physical keyboard: on a key press (not a key release or a modifier key alone, and not with a modifier other than Shift or a lock held) while an input-method connection is active, i.e. while a text field has focus, the input system fades the pointer out gradually; the next mouse movement shows it again at once. No user setting is read on that path (the policy offers only `isInputMethodConnectionActive()`). MAT-X33† has the pointer's shape per component, not this. | P: AOSP `platform/frameworks/native` @ `4f463a6b1de9198963dc6aff74154a504ba3f8f6` (`refs/heads/main`, read 2026-10-28), `services/inputflinger/PointerChoreographer.cpp:168-170` (`notifyKey`), `:179-203` (`fadeMouseCursorOnKeyPress`, `Transition::GRADUAL`), `:257-260` (`unfade`, `Transition::IMMEDIATE`, on mouse motion); `services/inputflinger/include/PointerChoreographerPolicyInterface.h:59-63`; introduced by AOSP commit `b3b3f73f1070e751bdd91ff0e8fe8c204344cbc0` ("Hide pointer while typing on a physical keyboard", 2023-07-21); M: *measure-compose* with a mouse and a keyboard (the fade's duration) | none | C22, C21 | P, M (review 2026-10-28: settled from AOSP code; the mirror citation replaced) |
| MAT-P3-16† | 4.3 | motion | Scroll physics: the fling and its deceleration | After a fling, Compose keeps scrolling on Android's spline decay, its deceleration computed from the platform's scroll friction and the density; the minimum and maximum fling velocities start and cap it. Whether a mouse-wheel step animates, and how the pager and carousel settle onto an item (MAT-N18): to verify. Overscroll is MAT-SC2 / SC4†. | R: `ViewConfiguration.getScrollFriction()` (API 1), `getScrollFrictionAmount()` (version 36.1), `getScaledMinimumFlingVelocity()` (API 3), `getScaledMaximumFlingVelocity(int, int, int)` (API 34) (`a-ref`); P: `ax/compose/animation/animation/src/androidMain/kotlin/androidx/compose/animation/SplineBasedFloatDecayAnimationSpec.android.kt:38`, `:41`; `ax/compose/animation/animation/src/commonMain/kotlin/androidx/compose/animation/FlingCalculator.kt:24-56`; M: *measure-compose* (screen recording) | none | C14, C21, C25 | R, P, M |
| MAT-P3-17† | 4.3 | size variants | Display density | Everything M3 states in dp is drawn at the display's density, which differs per display (an external display in desktop windowing) and which the user's display-size setting changes (its key: to verify); material-web's px are CSS px, scaled by the browser's device pixel ratio and zoom. `property-registry.toml` states no display scale. | R: `Configuration.densityDpi` (API 17, `a-ref`), `DisplayMetrics.density` | none (`AccessibilityPreferences.text_scaling_factor` is text only) | C10, C25 | R, V |
| MAT-P3-18† | 4.3 | motion | Switching between light and dark | What the user sees when the system switches theme: the activity takes the `uiMode` configuration change (recreated unless it handles the change itself: to verify); Compose's `MaterialTheme` applies a new colour scheme with no animation of its own (none in `MaterialTheme.kt`, by grep); whether the system cross-fades: to verify. material-web follows the page's CSS. | R: `Configuration.uiMode` (API 8, `a-ref`); P: `ax/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/MaterialTheme.kt` (by grep); M: *measure-compose* (screen recording of the switch) | none | C14 | R, P, M |
| MAT-P3-19† | 4.3 | size variants | Which Material look: generation, implementation, version | Material's look is not one: baseline M3 vs M3 Expressive (the `v0_192/` and `latest/` token sets; Compose's standard vs expressive motion scheme, a `MaterialTheme` parameter, MAT-X13), material-web vs Compose (they differ: MAT-X8's focus and pressed opacities, MAT-SC3†'s scrollbar in Compose only), and the library version an app ships (Compose M3 changes drawing behind flags, e.g. the check mark's points, MAT-P3-8†). The preset is one combination; the model can neither say which nor hold the others. | P: `ax/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/MaterialTheme.kt:91-101`, `:176-181` (`motionScheme`, default `standard()`); `ax/…/material3/ComposeMaterial3Flags.kt`; `ax/…/material3/Checkbox.kt:711-717` | none (one preset) | C09, C25 | P |
| MAT-P3-20† | 4.2 | rich text view | HTML imported into an `AnnotatedString` | Compose's `AnnotatedString.fromHtml` (ui-text) parses with Android's `Html` and keeps its spans as styles: headings h1–h6 as relative sizes, `<tt>` as monospace, bullets with an indentation per level, links (styled by `TextLinkStyles`, the *Link* note), colours, underline, strike-through, sub- and superscript; Android's `<blockquote>` span (`QuoteSpan`) is not among those Compose maps (whether a quote shows any style: to verify by render). The model has no style for any of these. | P: `ax/compose/ui/ui-text/src/androidMain/kotlin/androidx/compose/ui/text/Html.android.kt:87`, `:125-179` (span mapping), `:267-335` (bullets); `fb/core/java/android/text/Html.java:684` (`HEADING_SIZES`), `:827-831`, `:900-904`, `:1030`, `:1040`, `:1053`; M: *measure-compose* | none | C18, C24 | P, M |
| MAT-P3-21† | 4.1 | [input] | Caret: when it is drawn; the blink setting | Compose draws the caret only while the window and the field have focus and the selection is collapsed, so none while text is selected; its width is the Android `actual` `DefaultCursorThickness` (the value MAT-I12 left to verify); from version 36.1 the user sets the blink interval, which an app can read (a range from no blink to a photosensitivity-safe minimum, as the reference states). Whether a drop-position caret is drawn while text is dragged over a field: to verify. | P: `ax/compose/foundation/foundation/src/commonMain/kotlin/androidx/compose/foundation/text/TextFieldCursor.kt:49`, `:59`; `ax/compose/foundation/foundation/src/androidMain/kotlin/androidx/compose/foundation/text/TextFieldCursor.android.kt:22`; R: `ViewConfiguration.getTextCursorBlinkIntervalMillis()` (version 36.1, `a-ref`); M: *measure-compose* | `input.caret_color` | C16, C25 | P, R, M |

### Found on the way

- **"To verify" settled:** MAT-W4†'s API levels: `WindowInsets.Type.captionBar()` is API 30,
  `APPEARANCE_TRANSPARENT_CAPTION_BAR_BACKGROUND` API 35 (`a-ref`). MAT-M8's maximum menu width
  is `DropdownMenuItemDefaultMaxWidth` (`ax/…/material3/Menu.kt:2390-2391`). MAT-I12's
  caret thickness is `TextFieldCursor.android.kt:22`, and its blink is readable from version 36.1
  (MAT-P3-21†). MAT-X33†'s Compose link cursor is `TextLinkScope.kt:237-238`.
- **Pass 2's C03 note** ("whether any Compose M3 component uses Android's window blur"): Compose
  UI's `DialogProperties` carries `blurBehindRadius`, `backgroundBlurRadius` and `scrimAlpha`,
  all unspecified by default (`ax/compose/ui/ui/src/androidMain/kotlin/androidx/compose/ui/window/AndroidDialog.android.kt:136-171`,
  `:186-193`); an app may set them, M3 sets none in the files read.

## Pass 4 (2026-10-28)

The rows pass 3 and the review added were grouped into kinds and each kind put to
every platform; the coverage table is the main map's Appendix B. This section holds
this platform's rows for the kinds it had none for. Sources are cited in full in
each row.

Abbreviations as in the Material file: `compose/`, `mw/`, `latest/`, `ax/`, `fb/` (AOSP `platform/frameworks/base` @ `1cdfff555f4a21f71ccc978290e2e212e2f8b168`), `wms/` (its `libs/WindowManager/Shell/`), `a-ref` (developer.android.com reference pages; the pages new here, `WindowInsetsAnimation`, `PointerIcon`, `BiometricPrompt`, `View.DragShadowBuilder`, `PictureInPictureParams`, `Activity`, read 2026-10-28), "cat." (the 195 token files of `latest/` and the 93 entries of `compose/`, as the file's Cross-check table A). The scope convention is the file's *Pass 3* intro as the review corrected it: what the WM Shell, SystemUI or system_server draws round, over or beside the app's window gets a row marked system-drawn (scope main map §6.1); the system's own dialogs and pickers and what another app draws (the keyboard) get none.


| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| MAT-P4-1† | 4.1 | [window] | Window (Android): picture-in-picture | An activity can enter picture-in-picture (`Activity.enterPictureInPictureMode()`, API 24, deprecated in API 26 for `enterPictureInPictureMode(PictureInPictureParams)`, API 26): its content is shown in a small floating window the system draws round and over it: the window's corner radius and shadow radius; the menu over the content (a background at an alpha, the app's actions, the settings, dismiss and expand buttons, their sizes and padding, a disabled action's alpha, the menu's fade in and out and its auto-hide delays); the dismiss target that appears while the window is dragged (a circle, a gradient at the screen edge); the resize handle and touch edge; the stashed state at a screen edge (the strip left visible); the enter and exit transitions. Drawn by the WM Shell in the SystemUI process, not by the app (*Pass 3* intro; scope main map §6.1). `wms/src/com/android/wm/shell/` has both `pip/` and `pip2/`; which a given Android version runs: to verify. | P: `a-ref` (`Activity.enterPictureInPictureMode()`, `PictureInPictureParams`); `wms/res/values/dimen.xml:18-22` (dismiss circle sizes, dismiss gradient height), `:24-81` (`pip_*`: action sizes and padding, `pip_corner_radius`, resize handle and edge, `pip_stash_offset`, `pip_shadow_radius`); `wms/res/values/colors.xml:38-39`; `wms/src/com/android/wm/shell/pip/phone/PipMenuView.java:85-86` (fade durations), `:103-108` (auto-hide delays, `MENU_BACKGROUND_ALPHA`, `DISABLED_ACTION_ALPHA`), `:133`, `:278-289`, `:367-375` (menu fades), `:491`; M: *measure-compose* (an activity in PiP: screenshots, a screen recording of enter, drag, stash and exit) | none (`window.*` is the app's own window) | C20, C13, C11, C01, C14, C21 | P, M, V (review 2026-10-28: the no-argument call's deprecation in API 26 added, from the `Activity` page) |
| MAT-P4-2† | 4.3 | placement | The app window and the on-screen keyboard | What moves when the keyboard opens over a focused field depends on the window's `softInputMode`: with `SOFT_INPUT_ADJUST_PAN` the window is "panned by the framework to ensure the current input focus is visible"; `SOFT_INPUT_ADJUST_RESIZE` (deprecated in API 30) resizes it; with neither, "the system will try to pick one or the other depending on the contents of the window". An app that fits the insets itself (`Window.setDecorFitsSystemWindows(false)`, the replacement the reference names) gets the keyboard's slide as a `WindowInsetsAnimation` (API 30) and moves its content with it. M3's `Scaffold` pads by the system bars and the cutout, not the keyboard (MAT-P3-5†), so an M3 screen moves only by the pan, the resize or the app's own IME padding; whether a focused Compose text field also scrolls itself into view: to verify. How far and with what motion the content moves: measure. The dialog window: MAT-SF01† (Pass 3, W-SF12†). | R: `WindowManager.LayoutParams.softInputMode`, `SOFT_INPUT_ADJUST_PAN` (API 3), `SOFT_INPUT_ADJUST_RESIZE` (API 3, deprecated API 30), `WindowInsetsAnimation` (API 30) (`a-ref`); P: the `Scaffold.kt` and `SystemBarsDefaultInsets.android.kt` lines MAT-P3-5† cites; M: *measure-compose*, a screen recording of a field in the lower half focused under each mode | none | C26, C14, C25 | R, P, M, V |
| MAT-P4-3† | 4.3 | pointer | Pointer during a drag and drop | When a drag starts from a mouse, the window manager sets the pointer to `PointerIcon.TYPE_GRABBING` for the drag, and sets no other icon (no other `PointerIcon` use in `DragDropController.java` or `DragState.java`, by grep), so the cursor does not show what a drop would do there. `PointerIcon` has `TYPE_COPY`, `TYPE_NO_DROP` and `TYPE_ALIAS` (API 24); whether a drop target can show them during a system drag: to verify. material-web has no drag and drop (no `dragenter` or `dragover` in `mw/`, grep). MAT-X33† has the cursor per component, not this. | P: `fb/services/core/java/com/android/server/wm/DragDropController.java:272-277`; `PointerIcon` (`a-ref`); M: *measure-compose*, a mouse drag over a Compose drop target and outside it | none | C22, C07 | P, M, V |
| MAT-P4-4† | 4.3 | accessibility | Display colour transforms beyond inversion and correction | MAT-X35† has colour inversion and correction. Android's display also takes a night-light colour transform (on or off, its colour temperature, an automatic schedule), a "reduce bright colours" transform with a strength, and a saturation level (0 to 100). Each changes every colour an M3 app shows. They are managed by `ColorDisplayManager` ("Manages the display's color transforms and modes"), which is `@hide` and `@SystemApi`, so an app reader cannot call it; whether their state can be read another way: to verify. Applied by the system to the whole display, not drawn by the app (scope main map §6.1). | P: `fb/core/java/android/hardware/display/ColorDisplayManager.java:44-51` (the class, `@hide`, `@SystemApi`), `:205` (`isNightDisplayActivated`), `:224` (`getNightDisplayColorTemperature`), `:237` (`getNightDisplayAutoMode`: disabled, custom time, twilight, `:96-128`), `:391` (`setSaturationLevel`, 0 to 100), `:457` (`isReduceBrightColorsActivated`), `:478` (`getReduceBrightColorsStrength`); M: *measure-compose* (whether an emulator capture includes the display transform: to verify) | `AccessibilityPreferences` (none of these) | C23, C06, C25 | P, M, V (review 2026-10-28: the automatic schedule's lines added) |
| MAT-P4-5† | 4.3 | accessibility | Screen flash for notifications | With the screen-flash setting on, the system flashes a full-screen overlay over every app: for a notification event twice, with short on and off periods; for a call or an alarm repeatedly until it ends: the overlay's colour with its alpha (the user's choice; a default is stated), its fade in and out (a duration, an accelerate interpolator), the on and off periods and repeats per kind. Android's counterpart of a visual bell, triggered by notifications, calls and alarms rather than an alert sound. Drawn by the accessibility code in system_server as a secure system overlay window over the app, not by the app (scope main map §6.1). The settings are `screen_flash_notification` and `screen_flash_notification_color_global` (`Settings.System`); whether an app can read them: to verify. | P: `fb/services/accessibility/java/com/android/server/accessibility/FlashNotificationsController.java:85-96` (the kinds), `:107-124` (periods, fade, default colour and alpha), `:145-150` (settings keys), `:421-443` (the overlay window, `TYPE_SECURE_SYSTEM_OVERLAY`), `:453-460` (fade), `:492-497` (colour setting), `:699-720` (periods and repeats per kind); R: the two settings (to verify); M: *measure-compose* (a screen recording with the setting on) | `AccessibilityPreferences` (none) | C23, C21, C14, C25 | P, R, M, V |
| MAT-P4-6† | 4.1 | [tab] | Tabs: overflow by scrolling | M3's only overflow for tabs is a scrolling row: no overflow menu, and M3 does not elide labels (Compose's `Tab.kt` sets no `maxLines`, *Pass 3*; material-web's tab is `white-space: nowrap` with no ellipsis). Compose's scrollable rows give each tab a minimum width (already `tab.min_width_px`) and, when the selected tab changes, animate the scroll so that tab sits as near the centre as the row's ends allow, on the motion scheme's default spatial spec. material-web's `md-tabs` scrolls with its scroll bar hidden and smooth scrolling, and brings the active or focused tab into view keeping a margin from the ends (instantly when no tab has focus). MAT-TB7 has the leading edge padding only, MAT-TB6 the indicator's slide. | P: `compose/TabRow.kt:525-533`, `:838-844` (scroll animation spec), `:981` (`ScrollableTabRowMinTabWidth`), `:987` (`ScrollableTabRowEdgeStartPadding`, MAT-TB7's line, now in its row), `:1133-1185` (`ScrollableTabData`: scroll to centre, clamped); `mw/tabs/internal/_tabs.scss:13-15`, `:23-25` (the WebKit scroll bar hidden), `:39` (`flex: 1` on each tab); `mw/tabs/internal/_tab.scss:188-191` (`white-space: nowrap`); `mw/tabs/internal/tabs.ts:135-166` (`scrollToTab`, `scrollMargin`), `:219`, `:286`, `:312` | `tab.min_width_px` (the scrollable row's minimum, the `material-3` preset's 90); no field for the overflow, the scroll or its centring | C26, C14, C10 | P (review 2026-10-28: the minimum tab width is a field the model has, the nearest field says so; material-web's no-ellipsis and hidden WebKit scroll bar cited; MAT-TB7's line moved into MAT-TB7) |
| MAT-P4-7† | 4.2 | toolbars (MAT-N25) | Floating toolbar: content entering and leaving on expand / collapse | The Expressive floating toolbar has an expanded and a collapsed state (the `expanded` parameter, which the app sets; `FloatingToolbarDefaults.floatingToolbarVerticalNestedScroll` sets it from the scroll direction past a distance threshold; the `scrollBehavior` moves the whole bar off screen instead, MAT-N25): when it changes, the leading and trailing content are revealed and hidden by a clip on the motion scheme's fast spatial spec (horizontal toolbar: each grows from its outer end and shrinks towards the main content; vertical toolbar: each grows from and shrinks towards the main content); the shadow elevation animates between the expanded and the collapsed elevation on the same spec (both default to level 0, marked "TODO read from token"); the main content's balancing padding moves on the default effects spec ("Ensures this motion will cause the padding to bounce"). MAT-N25's motion is the bar's show / hide; the app bar's overflow (MAT-TA8†) moves items with no animation (none in `AppBarRow.kt` or `AppBarColumn.kt`, by grep). | P: `compose/FloatingToolbar.kt:175-188` (`expanded`, leading and trailing content), `:636-646` (the two elevations), `:696-699` (`animationSpec()`, `FastSpatial`), `:739-755` (enter and exit transitions), `:1493-1497` (elevation), `:1514-1521`, `:1541-1546`, `:1550-1556` (horizontal), `:1717`, `:1739-1742`, `:1765-1769`, `:1774-1777` (vertical); `:913-937`, `:1148` (`floatingToolbarVerticalNestedScroll`, `ScrollDistanceThreshold`); `:483-492` (`FloatingToolbarScrollBehavior`: the bar exits the screen); M: *measure-compose* (screen recording) | `toolbar.*` (no motion) | C14, C02, C24 | P, M (review 2026-10-28: the content's clip anchors corrected, `:1517-1518`, `:1552-1553`, `:1741-1742`, `:1776-1777`, they do not all lie at the toolbar's edges; what drives `expanded` corrected, the scroll behaviour hides the bar, a nested-scroll modifier sets `expanded`) |
| MAT-P4-8† | 4.3 | drag and drop | Drop outcome on the drag shadow | Android draws no operation badge on the drag shadow: the shadow is the app's image (`View.DragShadowBuilder`: "an image that the system displays during the drag and drop operation", by default "exactly the same appearance as the given View"), and the window manager animates only its position, scale and alpha. What it draws is the outcome: when the drop is not consumed, the shadow flies back to where the drag started while fading to half its alpha, over a duration between a minimum and a maximum scaled by the distance against the display diagonal, decelerating; when the drag is cancelled, it shrinks to nothing and fades out over the minimum duration; when the drag hid its source task, both outcomes only fade the shadow out in place over the minimum duration. No drop-progress indicator was found. Drawn by the window manager on the drag surface over the apps, not by the app (scope main map §6.1). MAT-X29 and MAT-L10 have the dragged item's look inside the app. | P: `fb/services/core/java/com/android/server/wm/DragState.java:83-84` (minimum and maximum duration), `:94-98` (the animated properties), `:147` (interpolator), `:682-715` (`endDragLocked`, `cancelDragLocked`), `:726-769` (return animation), `:771-799` (cancel animation); `View.DragShadowBuilder` (API 11, `a-ref`); M: *measure-compose* (a screen recording of a rejected and a cancelled drag) | none | C14, C07 | P, M (review 2026-10-28: the hidden-source-task variant added, `DragState.java:729-736`, `:773-779`) |
| MAT-P4-9† | 4.1 | [tooltip] | Tooltip: placement against its anchor | Compose places a tooltip against its anchor, not the pointer, through a position provider the app passes to `TooltipBox` (a required parameter). `TooltipDefaults.rememberTooltipPositionProvider` takes a preferred side, `TooltipAnchorPosition` above, below, left, right, start or end (start and end follow the layout direction), and the anchor spacing (MAT-T4): above and below centre the tooltip on the anchor horizontally, left and right centre it vertically; at the window's edge it flips to the opposite side, then it is clamped inside the window. The older provider (deprecated) centres on the anchor horizontally, prefers above, flips below, and aligns to the anchor's left or right edge at the window's sides. material-web has no tooltip component. MAT-T4 has the spacing and the caret (its caret "to verify" settled by these lines and now in its row: `TooltipDefaults.caretSize`, `caretShape()`, `DefaultTooltipCaretShape`). | P: `compose/Tooltip.kt:167-168` (`TooltipBox(positionProvider, …)`), `:683-731` (deprecated provider), `:743-753`, `:807` (`TooltipAnchorPosition`), `:899-1051` (`TooltipPositionProviderImpl`: left / right with flip `:932-978`, above / below with flip `:980-1024`, start / end by direction `:1026-1050`); caret `:532`, `:548-549`, `:1343`; `mw/` has no `tooltip` folder (listing) | `tooltip.*` (no placement) | C26 | P (review 2026-10-28: the deprecated provider aligns to the anchor's left or right edge, not start or end, `:713-721`; its lines start at the `@Deprecated` annotation; MAT-T4's caret lines moved into MAT-T4) |
