# Gap map — iOS (13–18 `ios-13`, 26 Liquid Glass `ios-26` unwritten)

Scope: `docs/todo_v0.7.1_model-completeness-gap-map.md` §2 checklist, applied to every element UIKit
(and SwiftUI where it draws differently) draws on iPhone and iPad. Model checked: `docs/property-registry.toml`
at caeddd6b (branch `v0.7.1-rc1`), `native-theme/src/model/`, `AccessibilityPreferences`
(`native-theme/src/lib.rs:415-437`), `IconRole` (`native-theme/src/model/icons.rs:42-139`),
`DialogEntranceStyle`, `AnimatedIcon`, the SF Symbols loader (`native-theme/src/sficons.rs`).
No values are collected here. Every Apple documentation page cited was fetched on 2026-10-27 through its JSON
form (`https://developer.apple.com/tutorials/data/documentation/uikit/<page>.json`; HIG:
`https://developer.apple.com/tutorials/data/design/human-interface-guidelines/<page>.json`). A path written
`uikit/x/y` below means `https://developer.apple.com/documentation/uikit/x/y`. A path written
`HIG <page>` means `https://developer.apple.com/design/human-interface-guidelines/<page>`.

## Conventions

**Source kinds** (§3 order):

- **R**: read at run time. A UIKit API that an iOS reader could query on device. The API's existence was
  checked against Apple's documentation on 2026-10-27 unless the row says *to verify*. native-theme has
  **no iOS reader** (X-RD-1), so until one exists every R row is read by the probe app instead.
- **P**: published. An Apple document (HIG page or API reference) that states the value or the rule.
- **M-q**: measured by probe query. The UIKit probe app in the Xcode simulator on the macOS CI runner
  instantiates the system view and reads its resolved geometry or colour, for example a subview's frame,
  `intrinsicContentSize`, or `layer` properties. No runtime reader would ever do this, so it counts as
  measurement.
- **M-r**: measured by probe render. A screenshot of the control (`xcrun simctl io screenshot`, or the view's
  `drawHierarchy`), then pixel analysis: colours, edges, blur fitted against a known test pattern behind it.
- **M-v**: measured by screen recording at the display refresh rate (`xcrun simctl io recordVideo`). Duration,
  curve or spring are fitted from the frames.

**Generation column** (13–18 / 26):

- *same*: one mechanism in both generations; the values can still differ.
- *26 differs: …*: the generations draw it differently in kind.
- *26 only* / *13–18 only*: the thing exists in one generation only.
- *to verify*: not yet established.

The 26 column also covers the iOS 27.x API additions the documentation already lists:
`UINavigationItem.titleAlignment` (27.2), `UIVerticalBarBehavior` (27.1) and `UIBarButtonItem.axisBehavior`
(27.1). What each of them draws is *to verify*.

**Probe runtimes.** 13–18 values come from the iOS 18 simulator runtime. Drift inside 13–18 is X-VR-1: iOS 15
bar scroll-edge transparency, the iOS 16 edit menu, the iOS 17 caret, selection UI and page-control progress,
and the iPadOS 18 top tab bar. Measuring that drift needs older runtimes, and whether the CI image provides
them is *to verify*.

**Nearest field:** the registry key a connector uses today, or *none*. "Partial" means the field exists but
covers one case only.

**Seeds** (§5) are cited where the gap is already written down. These include `docs/todo.md`'s *ios-26*
entry and the connectors' "the model states no …" notes.

---

## 4.1 Elements the model has: missing parts, states and properties

### defaults / text roles

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest field today |
|---|---|---|---|---|---|
| D-1 | defaults.font (every Font) | Letter spacing (tracking) per point size. SF Pro tracks by size: the HIG gives a size → tracking table for SF Pro, SF Pro Rounded and New York. | same mechanism (the table is the current HIG's; whether it changed in 26 is to verify) | P: HIG typography, "Tracking values — iOS, iPadOS, visionOS" tables | none |
| D-2 | Font | Optical-size variant (SF Pro Text / Display optical sizes) | same | P: HIG typography (states SF Pro); switch point M-r (render a glyph run across sizes); whether `opsz` is readable via Core Text is to verify | `family` (string only) |
| D-3 | Font | System design (default / rounded / serif "New York" / monospaced) and width (compressed / condensed / standard / expanded) | same | R: `uikit/uifontdescriptor/systemdesign`, `uikit/uifont/width` | `family` |
| D-4 | Font (per widget role) | Line height of each widget font. Only `text_scale` entries and the single `defaults.line_height` ratio exist; button, menu, list, nav-title and alert fonts all have their own leading. | same | R: `UIFont.preferredFont(forTextStyle:)` → `lineHeight`, `leading` | `defaults.line_height`, `TextScaleEntry.line_height_*` (partial) |
| D-5 | Font | Text-style binding. Every iOS control font is a Dynamic Type text style (body, headline, footnote, …) that scales by its own non-linear ramp. The model stores a fixed size plus one global factor. | same | R: `preferredFont(forTextStyle:compatibleWith:)`; P: HIG typography Dynamic Type tables | `size_pt` + `AccessibilityPreferences.text_scaling_factor` (partial) |
| D-6 | Font | Leading variants (tight / loose) | same | R: `UIFontDescriptor.SymbolicTraits` tight/loose leading (*to verify* names on iOS) | none |
| D-7 | Font | Emphasized weight per style. The HIG table has an "Emphasized weight" column, used by large titles and Bold Text. | same | P: HIG typography Dynamic Type tables | `weight` (one) |
| D-8 | Font | OpenType features (monospaced digits in timers, steppers and pickers; other features) | same | R: `UIFont.monospacedDigitSystemFont` (*to verify* which system controls use it); M-r | none |
| D-9 | defaults colours | Label hierarchy: `quaternaryLabel` is missing, and `tertiaryLabel` exists only as `disabled_text_color` (semantic conflation) | same API; 26 values to verify | R: `uikit/uicolor/quaternarylabel` (and label…tertiaryLabel); P: HIG color "iOS, iPadOS" foreground table | `text_color`, `muted_color`, `disabled_text_color` (partial) |
| D-10 | defaults colours | Fill hierarchy `systemFill` … `quaternarySystemFill`: four fills that UIKit's control tracks and fields use | same | R: `uikit/uicolor/systemfill` and siblings | none (per-widget colours only) |
| D-11 | defaults colours | Background hierarchy: `tertiarySystemBackground`, plus the three grouped backgrounds (`systemGroupedBackground`, secondary, tertiary) | same | R: `uikit/uicolor/systemgroupedbackground` etc.; P: HIG color (system vs grouped sets) | `background_color`, `surface_color` (partial) |
| D-12 | defaults colours | `separator` (translucent) vs `opaqueSeparator`: two separator colours | same | R: `uikit/uicolor/opaqueseparator` | `defaults.border.color`, `separator.line_color` (one) |
| D-13 | defaults colours | System colour palette beyond the five roles: yellow, mint, teal, cyan, indigo, purple, pink, brown, systemGray…systemGray6. These are drawn by swipe actions, badges, Settings-style row icons, the calendar and charts. | 26 values differ (preset header: "iOS 26 … changed the system colours") | R: `uikit/standard-colors`; P: HIG color "Specifications" (default and increased-contrast, light and dark) | `accent/danger/warning/success/info_color` (partial) |
| D-14 | defaults colours | Elevated level: every background colour has an elevated variant (`userInterfaceLevel = .elevated`), used in sheets, popovers and multitasking, in dark mode | same | R: `resolvedColor(with: UITraitCollection(userInterfaceLevel: .elevated))`, `uikit/uitraitcollection/userinterfacelevel` | none (only light/dark variants) |
| D-15 | defaults colours | Increased-contrast variant of every dynamic colour | same | R: `accessibilityContrast = .high` trait; P: HIG color specification tables (increased contrast columns) | `AccessibilityPreferences.high_contrast` flag only (no data) |
| D-16 | defaults colours | Dimmed tint: while a modal is up, `tintAdjustmentMode = .dimmed` turns every tinted control grey | same | R: `uikit/uiview/tintadjustmentmode-swift.enum`; the dimmed colour M-r | none |
| D-17 | defaults colours | Wide-gamut colour. System colours can be Display P3; the model's colour is 8-bit sRGB RGBA. | same | R: `UIColor` component read in extended sRGB; P: HIG color "Color management" | `Rgba` (sRGB only) |
| D-18 | defaults focus ring | Shape and position of the iPad keyboard focus halo: it follows the view's rounded rect and curve, inside, outside or automatic | 26 to verify | R: `uikit/uifocushaloeffect` (roundedRect cornerRadius curve, `position`); colour and width M-r | `focus_ring_color/width/offset` (partial) |
| D-19 | defaults.icon_sizes | Symbol scale and weight per context. An SF Symbol's size follows a text style plus a scale (small/medium/large), not a px box (see X-IC). | same | R: `uikit/uiimage/symbolscale`, `uikit/uiimage/symbolweight` | `icon_sizes.*_px` (partial) |
| D-20 | defaults.disabled_opacity | Disabled rendering per control: iOS uses state colours (tertiaryLabel, the fill at reduced alpha) or alpha, depending on the control. The ios-26 todo entry already flags the open choice. | to verify per control | M-r per control | `disabled_opacity` + per-widget disabled colours (partial). Seed: docs/todo.md *ios-26* (`defaults.disabled_opacity` 1.0 vs 0.4) |

### text_scale

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| TS-1 | text_scale | 7 of the 11 iOS text styles: Title 2, Title 3, Body (as an entry), Callout, Subheadline, Footnote, Caption 2. The model has caption, section_heading, dialog_title and display. | same set; sizes per generation to verify | P: HIG typography "iOS, iPadOS Dynamic Type sizes"; R: `preferredFont(forTextStyle:)` | `text_scale.*` (4 roles) |
| TS-2 | text_scale | Values per content-size category: xSmall … xxxLarge plus AX1–AX5, every style, size and leading. iOS's ramp is per style and non-linear, so one factor cannot express it. | same | P: HIG typography (one table per category, xSmall … AX5); R: `preferredFont(forTextStyle:compatibleWith: UITraitCollection(preferredContentSizeCategory:))`, `uikit/uicontentsizecategory` | `text_scaling_factor` (partial) |
| TS-3 | text_scale | Bold Text weights per style (`legibilityWeight = .bold`) | same | R: `uikit/uitraitcollection/legibilityweight` + preferredFont | none |
| TS-4 | text_scale | `extraLargeTitle` / `extraLargeTitle2` (iOS 17 text styles); whether iOS draws with them is *to verify* | 17+ | R: `UIFont.TextStyle` (*to verify* availability on iOS) | none |
| TS-5 | text_scale | Tracking per style and size (see D-1) and emphasized weight (see D-7) | same | P: HIG typography | none |

### window

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| W-1 | window (iPad windowed scenes) | Window controls: close / minimize / tile buttons, their glyphs, sizes, colours, hover and pressed states, and the style variants automatic / minimal / unified | 26 only (iPadOS 26) | R: `UIWindowScene.WindowingControlStyle` (iOS 26.0; automatic, minimal, unified); values M-r, M-q | none (the `IconRole::Window*` roles cover glyph identity only) |
| W-2 | window (iPad Stage Manager / iPadOS 26 windows) | Window corner radius, window shadow (every layer), resize grabber at the corner, the inactive window's look | 13–18: Stage Manager (16+); 26 differs (new windowing) | M-r, M-v | `window.border` (radius, shadow bool: partial) |
| W-3 | window (iPadOS 15–18) | Multitasking "…" control at the top centre of a scene | 13–18 only (replaced by W-1 in 26: to verify) | M-r | none |
| W-4 | window | Inactive-scene appearance (`activeAppearance` inactive): sidebar selection, tint and bar look of a non-key scene | same | R: `uikit/uitraitcollection/activeappearance`; colours M-r | `window.inactive_title_bar_*`, `defaults.selection_inactive_background` (partial) |
| W-5 | window | Status bar content style (default / lightContent / darkContent). The system draws the status bar, but the app chooses its style, and it must contrast with the theme's top bar. | same | R: `uikit/uistatusbarstyle` | none |
| W-6 | window (iPhone) | Display corner radius. It is the outer radius that iOS 26 concentric corners derive from. No public API (a private one exists and is excluded). | matters in 26 (concentricity); visible in 13–18 card sheets | M-r per device model | none |
| W-7 | window | Safe-area insets (status bar, home indicator, Dynamic Island): where content may draw | same | R: `UIView.safeAreaInsets` | `layout.window_margin_px` (partial) |
| W-8 | window | Home indicator: system-drawn and adaptive; the app controls only auto-hide. Listed for completeness: nothing themable beyond whether it hides. | same | R: `prefersHomeIndicatorAutoHidden` | none (note only) |
| W-9 | window (iPad title bar) | Scene title and subtitle shown by the system (`UIScene.title` / `subtitle`); whether iPadOS 26 draws a title bar is *to verify* | 26 to verify | R: `UIScene.subtitle`; M-r | `window.title_bar_font` (partial) |

### button (UIButton / UIButton.Configuration / SwiftUI Button)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| B-1 | button | Configuration styles. The model has one plain style plus "primary". UIKit has `plain`, `gray`, `tinted`, `filled`, `borderless`, `bordered`, `borderedTinted` and `borderedProminent`, each with its own fill, foreground and pressed look. | same (15+) | R: `uikit/uibutton/configuration-swift.struct` (style constructors); resolved colours via `background.resolvedBackgroundColor(for:)` (*to verify* it resolves) else M-r | `button.background_color`, `primary_background` (partial) |
| B-2 | button | Glass styles `glass()`, `prominentGlass()`, `clearGlass()`, `prominentClearGlass()`: material fill, tint, the press response | 26 only | R: `configuration-swift.struct/glass()` etc. (iOS 26.0); paint M-r, press M-v | none |
| B-3 | button | Size variants `mini` / `small` / `medium` / `large`: font (text style), content insets per side, corner radius and height for each | same (15+); 26 sizes to verify | R: `configuration-swift.struct/buttonsize`, `setDefaultContentInsets()` then `contentInsets`; font and radius M-q | `border.padding_*`, `font`, `min_height_px` (one size: partial) |
| B-4 | button | Corner style: `fixed` / `dynamic` / `small` / `medium` / `large` / `capsule`. With `dynamic`, the radius depends on size and Dynamic Type. | 26 differs (glass and prominent buttons capsule: to verify) | R: `configuration-swift.struct/cornerstyle-swift.enum`; radius M-q (`background.cornerRadius` after update, *to verify*) | `border.corner_radius_px` (fixed number only) |
| B-5 | button | Pressed state of a plain or text button: 13–18 lowers the label's alpha instead of changing its colour | 26 differs: interactive-glass scale and highlight (B-2) | M-r (alpha), M-v | `active_text_color` (colour only: partial). Seed: iced styles.rs:380 ("no hovered and no pressed primary fill") |
| B-6 | button | Pressed state per style (filled, tinted, gray) and its fade-out | same | M-r, M-v | `active_background` (one style) |
| B-7 | button | Pressed + selected combination (toggle buttons, `changesSelectionAsPrimaryAction`) | same | M-r | `checked_*`, `active_*` (no combination) |
| B-8 | button | Subtitle part: font, colour, and `titlePadding` between title and subtitle | same (15+) | R: `titlePadding`; font and colour M-q | none |
| B-9 | button | Image placement (leading / trailing / top / bottom) and the symbol configuration used for its image | same | R: `imagePlacement`, `preferredSymbolConfigurationForImage` | `icon_text_gap_px` (gap only) |
| B-10 | button | Loading state: `showsActivityIndicator` swaps the image for a spinner, with its own colour | same (15+) | R: `showsActivityIndicator`, `activityIndicatorColorTransformer`; M-r | none |
| B-11 | button | Role variants: destructive (red text), cancel (bold weight in alerts and sheets); SwiftUI roles (26 adds roles: *to verify* `.close`/`.confirm`) | 26 to verify | R: `UIMenuElement.Attributes.destructive`; M-r | `danger_color` (no button role fields) |
| B-12 | button | System button types: close (circle + xmark), info / detail disclosure, contact add. Each has its glyph, fill, size and pressed state. | 26 to verify (close button look) | R: `UIButton.ButtonType`; M-r | none |
| B-13 | button | Font per style: filled and prominent use a heavier weight than plain (*to verify*), and `done` and prominent bar items use semibold | same | M-q | `button.font` (one) |
| B-14 | button | Disabled look per style | same | M-r | `disabled_background/text_color` (one style) |
| B-15 | button | Button Shapes accessibility variant (X-A11-5) | same | M-r | none |
| B-16 | button | Symbol content transition when the image changes (`symbolContentTransition`) | 26 only | R: `configuration-swift.struct/symbolcontenttransition` (iOS 26.0); M-v | none |
| B-17 | button | Pointer hover (iPad): highlight or lift effect, not a colour (X-PH) | same | M-v | `hover_background` (partial) |

### input (UITextField)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| I-1 | input | Border style variants `none` / `line` / `bezel` / `roundedRect`. The default is roundedRect; each draws a different frame. | 26 look to verify | R: `uikit/uitextfield/borderstyle-swift.enum`; M-r | `input.border` (one look) |
| I-2 | input | Clear button part: glyph, colour, size, its trailing inset, and its visibility modes | 26 to verify | R: `clearButtonMode`; M-r | none |
| I-3 | input | Leading and trailing overlay views (icon in field) and their insets | same | R: `leftView/rightView` rects via `leftViewRect(forBounds:)` (M-q) | `border.padding_*` (partial) |
| I-4 | input | Marked text (IME composition): underline and background | same | M-r (probe sets marked text) | none |
| I-5 | input | Spelling and autocorrection underline (colour, style), and the autocorrection bubble/candidate | 17+ differs (inline corrections: to verify) | M-r | none |
| I-6 | input | Inline predictions (iOS 17): ghost text colour | 17+ | M-r | `placeholder_color` (partial?) |
| I-7 | input | `sizingRule` / `adjustsFontSizeToFitWidth` minimum font (visible shrink) | to verify | R: `UITextField.sizingRule`, `minimumFontSize` | none |
| I-8 | input | Caret, selection highlight, selection handles (X-CA, X-SE) | 17 changed | see X-CA, X-SE | `input.caret_color`, `selection_*` (partial) |

### text_area (UITextView)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| TA-1 | text_area | Border style `none` / `roundedRect` (iOS 17), with its radius and colour | 17+ | R: `UITextView.BorderStyle` (iOS 17.0); M-r | `text_area.border` (partial) |
| TA-2 | text_area | Find-and-replace highlights: all matches and the current match (colour, corner, the "bounce" of the current match) | 16+ | R: `uikit/uifindinteraction`; M-r, M-v | none |
| TA-3 | text_area | Text highlight styles (`textHighlightAttributes`, iOS 18) | 18+ | R: `UITextView.textHighlightAttributes` (iOS 18.0); M-r | none |
| TA-4 | text_area | Writing Tools in-text animation (glow/shimmer over rewritten text) | 18.1+ | R: `writingToolsBehavior`; M-v | none |
| TA-5 | text_area | Data-detector link look (`linkTextAttributes` default) | same | R: `linkTextAttributes` | `link.*` (partial) |

### checkbox

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| CB-1 | checkbox | iOS has no UIKit checkbox: `UISwitch.Style.checkbox` is Mac Catalyst only (`uikit/uiswitch`, "Displaying a checkbox in your Mac app built with Mac Catalyst"). Its checkbox roles are the **circular** list multiselect indicator (empty circle / filled tint circle with checkmark) and the list checkmark accessory. The model has no indicator shape (square vs circle) and no "checkmark in filled circle" glyph. | same; 26 to verify | R: `UICellAccessory.multiselect`, `.checkmark` (`uikit/uicellaccessory-swift.struct`); M-r | `checkbox.indicator_width_px`, `radio_*` (partial) |

### menu (UIMenu, context menu, pull-down)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| MN-1 | menu | Background material (blur in 13–18, Liquid Glass in 26) | 26 differs | M-r (X-MT) | `menu.background_color` (colour only) |
| MN-2 | menu | Shadow layers | 26 differs | M-r | `menu.border.shadow_enabled` (bool) |
| MN-3 | menu | Width (min / max / content-fitted) and maximum height with scrolling | to verify | M-q | none |
| MN-4 | menu | Item subtitle (`UIAction.subtitle`, iOS 15): font and colour | same | M-q | none |
| MN-5 | menu | Icon side: iOS menus put the image on the trailing side; whether 26 moves it is *to verify* | 26 to verify | M-r | none (no icon-side field) |
| MN-6 | menu | Checked and mixed states (`UIMenuElement.State` on / mixed): the checkmark and mixed glyphs, their column and colour | same | R: `UIAction.state`; M-r | none |
| MN-7 | menu | Destructive items: red title and red icon | same | R: `UIMenuElement.Attributes.destructive`; M-r | `defaults.danger_color` (no menu field) |
| MN-8 | menu | Touch highlight while the finger drags across items: a grey fill, distinct from pointer hover | same | M-r | `menu.hover_background` (partial: ios-13 states an accent fill) |
| MN-9 | menu | Two separator kinds: the hairline between items, and the thick group separator between inline sections | same | M-r | `menu.separator_color` (one) |
| MN-10 | menu | Menu title header (`UIMenu.title`): font, colour, padding | same | M-q | none |
| MN-11 | menu | Submenus: chevron glyph, inline expansion in place, the expand animation | same (14+) | M-r, M-v | none |
| MN-12 | menu | Element-size rows (`UIMenu.ElementSize` small/medium; `.displayAsPalette`, iOS 17): item tile size, icon size, label under icon | same (16+/17+) | R: `uikit/uimenu/elementsize`, `uikit/uimenu/options-swift.struct/displayaspalette`; P: HIG menus (small = row of four, medium = row of three); M-q | `menu.icon_size_px` (partial) |
| MN-13 | menu | Keyboard-shortcut column (iPad with keyboard) | same | M-r | none |
| MN-14 | menu | Open and close motion: spring scale from the source in 13–18; in 26 the menu morphs out of the button (glass morph) | 26 differs | M-v | none |
| MN-15 | menu | Row height scaling with Dynamic Type | same | M-q at several categories | `menu.row_height_px` (fixed: partial) |
| MN-16 | menu | Context-menu appearance rich vs compact (`UIContextMenuInteraction.appearance`) | same (14+) | R: `menuAppearance` | none |

### tooltip (UIToolTipInteraction, iPad pointer / Catalyst)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| TT-1 | tooltip | Material or fill, corner radius and shadow of the tip | 15+; 26 to verify | M-r (pointer hover in the simulator, *to verify* it works) | `tooltip.background_color`, `border` (partial) |
| TT-2 | tooltip | Hide delay, and the fade-in / fade-out motion | same | M-v | `tooltip.show_delay_ms` only (and its value is unstated, preset comment) |

### scrollbar (scroll indicators)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| SB-1 | scrollbar | Indicator style variants `default` / `black` / `white`: three colour sets, with default adapting to content | same | R: `uikit/uiscrollview/indicatorstyle-swift.enum`; colours M-r | `scrollbar.thumb_color` (one) |
| SB-2 | scrollbar | Inset from the scroll view's edges (distance of the thumb from the edge and the ends) | same | R: `verticalScrollIndicatorInsets` (app-level); system inset M-q | none. Seed: iced styles.rs:1518 ("no gap between the scrollbar and the contents") |
| SB-3 | scrollbar | Auto-hide timing: fade-out delay and fade duration; the flash on appearance | same | R: `flashScrollIndicators()` exists; timing M-v | `overlay_mode` (bool only) |
| SB-4 | scrollbar | Grabbed state: the indicator grows wider when long-pressed and dragged (fast scrolling) | same (13+) | M-v, M-r | `thumb_active_color` (colour only, no width) |
| SB-5 | scrollbar | Thumb corner (capsule) and any outline | same | M-r | none (scrollbar has no border) |
| SB-6 | scrollbar | Indicators with an edge effect under bars | 26 differs (X-MT-7) | M-r | none |

### slider (UISlider)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| SL-1 | slider | Thumb shadow layers (and any thin thumb outline) | 26 differs | M-r | none |
| SL-2 | slider | Thumb shape. Round in 13–18; in 26 a capsule (WebKit draws it as a 24 × 16 capsule per docs/todo.md *ios-26*), so it needs width and height, not a diameter | 26 differs | M-q (`thumbRect(forBounds:trackRect:value:)`), M-r | `thumb_diameter_px` (partial). Seed: docs/todo.md *ios-26* |
| SL-3 | slider | Dragging state: in 26 the thumb becomes a Liquid Glass lens and grows (HIG materials: "sliders and toggles … take on a Liquid Glass appearance … when a person activates it"); 13–18 to verify | 26 differs | P: HIG materials (Liquid Glass); M-v | none. Seed: iced styles.rs:1292 ("no dragged thumb color") |
| SL-4 | slider | Track corner radius (capsule ends) | same | M-r | none |
| SL-5 | slider | Minimum- and maximum-value images: size, colour, gap to the track | same | R: `minimumValueImageRect(forBounds:)` (M-q); colour M-r | none |
| SL-6 | slider | Tick marks (`TrackConfiguration` ticks / numberOfTicks): shape, size, colour, placement relative to the track | 26 only (UIKit ticks) | R: `uikit/uislider/trackconfiguration-swift.struct` (iOS 26.0); M-r | `tick_mark_length_px` (partial) |
| SL-7 | slider | Disabled sub-range (`enabledRange`): the look of the track outside it | 26 only | R: `TrackConfiguration.enabledRange`; M-r | `disabled_track_color` (partial) |
| SL-8 | slider | Thumbless style (`UISlider.Style.thumbless`): track thickness and fill without a thumb | 26 only | R: `uikit/uislider/style` (default, thumbless); M-r | none |
| SL-9 | slider | Animated value change (`setValue(_:animated:)`) and thumb snap to ticks | same / 26 | M-v | none |

### progress_bar (UIProgressView)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| PB-1 | progress_bar | Style variants `default` / `bar` (the toolbar style, with a different track) | same | R: `UIProgressView.progressViewStyle`; M-r | none |
| PB-2 | progress_bar | Rounding of the fill's ends, separate from the track's | 26 differs (preset header: 26 changed the progress bar) | M-r | `progress_bar.border.corner_radius_px` (partial) |
| PB-3 | progress_bar | Animated progress change: duration and curve | same | M-v | none. Seed: gpui widgets/progress.rs:139 ("the model states no motion") |

### tab (UITabBar; iPadOS top tab bar)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| T-1 | tab | Bar background material and the scroll-edge state: 15+ bars are transparent at the scroll edge (`scrollEdgeAppearance`) and material while content is under them | 26 differs (floating glass capsule) | R: `UITabBarAppearance` / `UIBarAppearance.backgroundEffect` after `configureWithDefaultBackground()` (*to verify* it reports the effect); M-r | `tab.bar_background` (colour) |
| T-2 | tab | Top hairline (`shadowColor`) on one side only | 13–18 only (26: no hairline, to verify) | R: `UIBarAppearance.shadowColor` | `tab.border` (all sides) |
| T-3 | tab | Item layouts: stacked (icon over title), inline, compactInline. Each has its own icon size, title font and spacing; plus `stackedItemPositioning`/`Spacing`/`Width` | 26 to verify | R: `UITabBarAppearance` stacked/inline properties; fonts and sizes M-q | `tab.font`, `item_gap_px` (partial) |
| T-4 | tab | Unselected item colour (icon and title) | 26 differs (adaptive monochrome over glass: HIG color "Liquid Glass color") | R: `UITabBarItemAppearance.normal` (defaults likely unset → M-r) | `tab.font.color` (partial) |
| T-5 | tab | Selected icon variant (fill symbol) and selected title weight | to verify | M-r | `active_text_color` (colour only) |
| T-6 | tab | Disabled and focused item states | same | R: `UITabBarItemAppearance` has `disabled`, `focused`; M-r | none |
| T-7 | tab | Badge part (X-BG): fill, text, size, position | 26 to verify | R: `UITabBarItem.badgeColor`, `badgeTextAttributes(for:)`; M-r | none |
| T-8 | tab | Glass selection pill ("lens") behind the selected item, which slides and morphs between items and follows a drag | 26 only | M-r, M-v | `active_indicator_*` (Top/Bottom bar only: cannot express a pill) |
| T-9 | tab | Floating bar geometry: inset from the screen edges, capsule radius, shadow, height | 26 only | M-q, M-r | `tab.min_height_px`, `border` (partial) |
| T-10 | tab | Minimized state (`tabBarMinimizeBehavior`): the size and contents of the minimized bar, and its animation | 26 only | R: `uikit/uitabbarcontroller/tabbarminimizebehavior` (iOS 26.0); M-r, M-v | none |
| T-11 | tab | Search tab as a separate circular button at the trailing end | 26 only | P: HIG tab-bars ("dedicated search tab at the trailing end"), HIG search-fields; M-r | none |
| T-12 | tab | Bottom accessory (4.2 C-21) and its inline vs expanded environments | 26 only | R: `uikit/uitabbarcontroller/bottomaccessory`, `UITabAccessory.Environment` | none |
| T-13 | tab | iPadOS top tab bar (18+, sidebar-adaptable `UITabBarController.Mode.tabSidebar`): floating segmented-like bar plus a sidebar toggle | 18+; 26 differs | R: `uikit/uitabbarcontroller/mode-swift.enum`; P: HIG tab-bars (iPadOS); M-r | `tab.*` (partial) |
| T-14 | tab | Height per orientation and size class (compact landscape vs portrait) | same | M-q | `tab.min_height_px` (one) |
| T-15 | tab | Selection change and hide/show (`setTabBarHidden(_:animated:)`) motion | 26 differs | M-v | none |
| T-16 | tab | Prominent tab (`prominentTabIdentifier`): its distinct look | 26 (to verify) | R: `UITabBarController.prominentTabIdentifier`; M-r | none |

### sidebar (UISplitViewController primary column, `.sidebar` list appearance)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| SD-1 | sidebar | Background effect (`primaryBackgroundStyle = .sidebar`): a material, not a colour | 26 differs (glass panel, "more opaque in larger elements like sidebars", HIG color) | R: `UISplitViewController.BackgroundStyle`; M-r | `sidebar.background_color` |
| SD-2 | sidebar | Selected row as a rounded, inset platter: corner radius, insets, icon tint in the selected row | 26 differs (to verify) | M-r, M-q | `sidebar.selection_background/text_color` (fill only) |
| SD-3 | sidebar | Selected + unfocused selection (grey when focus is elsewhere) | same | M-r | `defaults.selection_inactive_background` (not per sidebar: partial) |
| SD-4 | sidebar | Section header (`sidebarHeader()`): font, colour, and the trailing outline-disclosure chevron | same | R: `UIListContentConfiguration.sidebarHeader().textProperties` | none |
| SD-5 | sidebar | Row image size and tint, plus the accompanied-sidebar cell variants | same | R: `UIListContentConfiguration.sidebarCell()` / `accompaniedSidebarCell()` `imageProperties` | none |
| SD-6 | sidebar | Floating inset glass sidebar: inset from the window edges, corner radius, shadow; content extending beneath (`UIBackgroundExtensionView`) | 26 only | R: `uikit/uibackgroundextensionview` (iOS 26.0); M-r | `sidebar.border` (partial) |
| SD-7 | sidebar | Show/hide and tab-bar↔sidebar morph motion | 18+ / 26 differs | M-v | none |
| SD-8 | sidebar | Row height | same | M-q | none (sidebar has no row height) |

### toolbar (UIToolbar; UINavigationBar items in 4.2)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| TB-1 | toolbar | Material background and the scroll-edge transparent state (as T-1) | 26 differs (no bar background; floating item groups) | R: `UIToolbar.standardAppearance`/`scrollEdgeAppearance`; M-r | `toolbar.background_color` |
| TB-2 | toolbar | One-side hairline (top) | 13–18 only (to verify) | R: `UIBarAppearance.shadowColor` | `toolbar.border` (all sides) |
| TB-3 | toolbar | Compact height (iPhone landscape, `compactAppearance`) | same | M-q | `toolbar.bar_height_px` (one) |
| TB-4 | toolbar | Bar item styles `plain` / `done` / `prominent`: done is bold; prominent is a tinted glass fill in 26 | 26 differs | R: `uikit/uibarbuttonitem/style-swift.enum` (`.prominent`); `UINavigationBarAppearance.doneButtonAppearance`/`prominentButtonAppearance`; M-r | `toolbar.font` (one) |
| TB-5 | toolbar | Grouped glass capsules: items sharing a background, group gap, capsule radius, glass, shadow | 26 only | R: `uikit/uibarbuttonitem/hidessharedbackground`, `sharesBackground` (iOS 26.0); M-r | none |
| TB-6 | toolbar | Item pressed state (alpha dim in 13–18, glass press in 26) | 26 differs | M-v | none |
| TB-7 | toolbar | Item badge (`UIBarButtonItem.Badge`: count / indicator / string) | 26 only | R: `uikit/uibarbuttonitem/badge-swift.struct` (backgroundColor, foregroundColor, font); M-r | none |
| TB-8 | toolbar | Fixed and flexible space widths; padding (`isPaddingRemoved`) | same | M-q | `item_gap_px` (partial) |

### status_bar

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| ST-1 | status_bar | iOS apps have no app status bar; the system status bar is W-5. No model gap; the section has no iOS counterpart. | — | — | — |

### list (UITableView / UICollectionView list, SwiftUI List/Form)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| L-1 | list | Appearance variants `plain` / `grouped` / `insetGrouped` / `sidebar` / `sidebarPlain`, each with its own table background and cell background | same; 26 values to verify | R: `UICollectionLayoutListConfiguration.Appearance`; `UIBackgroundConfiguration.listGroupedCell()` etc. → `resolvedBackgroundColor(for:)` | `list.background_color` (one) |
| L-2 | list | insetGrouped section platter: corner radius, horizontal inset (follows layout margins / readable width) | 26 differs (radius: to verify) | M-q, M-r | `list.border.corner_radius_px` (partial) |
| L-3 | list | Separator inset (leading inset aligned to the text), per-section top/bottom visibility, multi-selection separator colour | same (14.5+) | R: `UITableView.separatorInset`; `uikit/uilistseparatorconfiguration-swift.struct` (insets, color, multipleSelectionColor) | `list.grid_color` (colour only) |
| L-4 | list | Section header per style: plainHeader (sticky, with its own background), groupedHeader, prominentInsetGroupedHeader, extraProminentInsetGroupedHeader. Each has its font, colour and text transform (uppercase in 13–14, *to verify*). | 13–14 vs 15+ differ | R: `UIListContentConfiguration.*Header()` → `textProperties.font/color/transform` | `list.header_font`, `header_background` (one style) |
| L-5 | list | Section footer: font and colour | same | R: `groupedFooter()`/`plainFooter()` textProperties | none. Seed: gpui contract.rs:822, colors.rs:448 ("no footer colour") |
| L-6 | list | Header top padding (`sectionHeaderTopPadding`, 15+) | 15+ | R: `UITableView.sectionHeaderTopPadding` | `list.header_height_px` (partial) |
| L-7 | list | Cell content: secondary text (subtitle / value cell): font, colour, `textToSecondaryText*Padding`, `imageToTextPadding`, `directionalLayoutMargins` | same | R: `UIListContentConfiguration.cell()/subtitleCell()/valueCell()` | `list.item_font`, `row_height_px` (partial) |
| L-8 | list | Cell image: reserved size, tint, corner | same | R: `imageProperties.reservedLayoutSize/tintColor/cornerRadius` | none |
| L-9 | list | Row height is content-sized with a minimum, and scales with Dynamic Type | same | M-q at several categories | `row_height_px` (fixed: partial) |
| L-10 | list | Cell states: highlighted (touch), selected, focused (keyboard: tint fill), editing, swiped, expanded, dragging (lifted), drop target. Combinations too (selected + focused, selected in multi-select). | same | R: `UICellConfigurationState`; colours via `UIBackgroundConfiguration.updated(for: state)` + `resolvedBackgroundColor` (*to verify*); M-r | `selection_background`, `hover_background` (partial). Seed: iced styles.rs:1628 ("no drop target") |
| L-11 | list | Selection fade-out on deselect (duration) | same | M-v | none |
| L-12 | list | Accessories: disclosure indicator, checkmark, detail (info), delete (red minus), insert (green plus), reorder grabber, multiselect circle, outline disclosure, label, pop-up menu. Each has its glyph, size, colour and placement. | same (14+); 26 to verify | R: `uikit/uicellaccessory-swift.struct` options (tintColor etc.); M-r | none |
| L-13 | list | Swipe actions: button fill per style (destructive vs normal), title font, image, full-swipe expansion, layout | 26 differs (to verify) | R: `uikit/uicontextualaction` (`backgroundColor` default, *to verify*); M-r, M-v | none |
| L-14 | list | Edit-mode motion: delete control rotation, indentation, reorder lift | same | M-v | none |
| L-15 | list | Section index (A–Z): font, colour, background, tracking background | same | R: `sectionIndexColor`, `sectionIndexBackgroundColor`, `sectionIndexTrackingBackgroundColor` (defaults nil → M-r); `UIScrollView.indexDisplayMode` | none |
| L-16 | list | Row insert/delete/reload animations (`RowAnimation`: fade, right, left, top, bottom, middle, automatic): durations | same | R: `uikit/uitableview/rowanimation` (kinds); M-v | none |
| L-17 | list | Filler rows of the plain style (empty separators below content) | 15+ | R: `fillerRowHeight` | none |
| L-18 | list | No alternating rows. iOS draws none, but `list.alternate_row_background` is a required leaf (ios-13 states a colour iOS does not draw). The model cannot say "no alternation". | same | P: absence (no UIKit API for alternation) | `alternate_row_background` (required). Seed: docs/todo.md *ios-26* lists it among the 38 unsourced leaves |
| L-19 | list | Accessibility-size layout switch: value cells stack text above secondary text | same | R: `prefersSideBySideTextAndSecondaryText`, `isAccessibilityCategory`; P: HIG typography ("consider using a stacked layout") | none |
| L-20 | list | Expanded/collapsed outline rows (as EX-*) | same | — | `expander.*` |

### popover (UIPopoverPresentationController)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| PO-1 | popover | Arrow part: base width, height, tip rounding, its side | 26 differs (whether 26 popovers keep the arrow is to verify) | R: `permittedArrowDirections`/`arrowDirection`; geometry M-r | none |
| PO-2 | popover | Material (13–18 to verify; glass in 26) | 26 differs | M-r | `popover.background_color` |
| PO-3 | popover | Shadow layers | 26 differs | M-r | `popover.border.shadow_enabled` |
| PO-4 | popover | Screen-edge margin (`popoverLayoutMargins` default) | same | R: `popoverLayoutMargins` | none |
| PO-5 | popover | Present/dismiss motion (fade, or morph from source in 26) | 26 differs | M-v | none |
| PO-6 | popover | Elevated background colours inside (D-14) | same | R (D-14) | none |

### splitter (UISplitViewController columns)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| SP-1 | splitter | Column divider colour and width, and whether the divider is drawn next to a floating sidebar | 26 differs | M-r | `splitter.divider_*` (partial) |
| SP-2 | splitter | User column resizing (a drag handle, its hover/pressed look) | 26 to verify (only `minimum/maximumPrimaryColumnWidth` read so far) | R: `uikit/uisplitviewcontroller` column-width properties; M-r | `splitter.hover_color` (partial). Seed: iced styles.rs:1620 ("picked split … the model states no") |
| SP-3 | splitter | Column show/hide motion | same | M-v | none |

### separator

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| SE-1 | separator | A hairline of exactly one device pixel at any scale (1/displayScale pt). `line_width_px` is logical px and cannot say "one physical pixel". | same | R: `UITraitCollection.displayScale` | `separator.line_width_px` (partial) |
| SE-2 | separator | Translucent `separator` vs `opaqueSeparator` (D-12), and the vibrant separator over materials (X-MT-4) | same | R | `separator.line_color` (one) |

### switch (UISwitch)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| SW-1 | switch | Thumb shadow layers | 26 differs | M-r | none (`thumb_border_*` is the outline only) |
| SW-2 | switch | Pressed state: in 13–18 the thumb stretches wider while held; in 26 it becomes a glass lens and grows (HIG materials) | 26 differs | P: HIG materials (toggles take Liquid Glass when activated); M-v, M-r | none. Seed: iced styles.rs:260 ("no hovered and no pressed checked toggle") |
| SW-3 | switch | Thumb shape: a capsule in 26 (WebKit: 39.857 × 26.571, docs/todo.md *ios-26*), so it needs width and height, plus the thumb's corner radius | 26 differs | M-q (thumb subview frame), M-r | `thumb_diameter_px`, `unchecked_thumb_diameter_px` (partial). Seed: iced lib.rs:926 ("no thumb radius") |
| SW-4 | switch | On/Off Labels: the "I" / "O" glyphs on the track (shape, colour, size, position) | same | R: `uikit/uiaccessibility/isonoffswitchlabelsenabled`; M-r | none |
| SW-5 | switch | Toggle motion: thumb slide (spring) and track colour cross-fade | 26 differs | M-v | none |
| SW-6 | switch | Increase Contrast look (an off-track outline: to verify) | same | M-r with Increase Contrast on | `unchecked_track_border_color` (no variant) |
| SW-7 | switch | Label / title: no font for a switch's label (SwiftUI Toggle row label) | same | M-q | none. Seed: iced styles.rs:982 ("no font for a switch") |

### dialog (UIAlertController `.alert`)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| DL-1 | dialog | Material platter (blur in 13–18, glass in 26) | 26 differs | M-r | `dialog.background_color` |
| DL-2 | dialog | Dimming scrim behind the alert (X-SC) | same | M-r | none |
| DL-3 | dialog | Title and message text alignment (centred in 13–18; leading in 26: to verify) | 26 to verify | M-r | none |
| DL-4 | dialog | Gaps: title↔message, text↔buttons, content insets | 26 differs | M-q | `dialog.border.padding_*` (partial) |
| DL-5 | dialog | Button area. 13–18: full-width cells with hairline separators above and between, and a grey pressed fill. 26: separate capsule buttons with gaps, the default one prominent (to verify). | 26 differs | M-r, M-q | `dialog.button_gap_px`, `button.*` (partial) |
| DL-6 | dialog | Button layout rule: side by side for two, stacked for three or more or for long titles; cancel placement in a stack | same rule (to verify in 26) | M-q (alerts with 2/3 actions and long titles) | `dialog.button_order` (partial) |
| DL-7 | dialog | Button text per role: preferred action and cancel are bold, destructive is red, the others regular in tint | same (26 to verify) | R: `preferredAction`, `UIAlertAction.Style`; M-r | none |
| DL-8 | dialog | Alert button height | 26 differs | M-q | none (`button.min_height_px` is generic) |
| DL-9 | dialog | Alert text fields (`addTextField`): their own small field style | 26 to verify | M-r | none |
| DL-10 | dialog | Entrance motion. 13–18: the alert starts larger and shrinks to size while fading in. The model's `ScaleFade` scales *up* and states no factor, duration or curve. | 26 differs (to verify) | M-v | `dialog.entrance` (enum, partial) |
| DL-11 | dialog | Exit motion (fade) | same | M-v | none |
| DL-12 | dialog | Elevated colours in the platter (D-14) | same | R | none |

### spinner (UIActivityIndicatorView)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| SP-1a | spinner | The iOS indicator is spokes, not a ring: spoke count, spoke length, width and end rounding, and the per-spoke opacity ramp | 26 to verify | M-r (frame captures) | `spinner.stroke_width_px` (ring model: partial) |
| SP-2a | spinner | Animation: step period and stepping (discrete rotation) | same | M-v | none |
| SP-3a | spinner | Size styles `medium` / `large` (two named sizes, not min/max) | same | R: `uikit/uiactivityindicatorview/style-swift.enum`; size M-q (`intrinsicContentSize`) | `diameter_px`, `min_diameter_px` (partial) |
| SP-4a | spinner | Animated icon. The SF Symbols set has no animated spinner (`icons.rs:728-737` returns `None` for SfSymbols), so iOS's spinner has no `AnimatedIcon`. | same | M-r frames | `AnimatedIcon` (none for SfSymbols) |
| SP-5a | spinner | Start/stop fade (`hidesWhenStopped`) | same | M-v | none |

### combo_box (pop-up button: UIButton with menu + `changesSelectionAsPrimaryAction`)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| CB-1 | combo_box | Indicator glyph identity (`.popup` indicator: up/down chevron pair), its colour and weight | same (16+); 26 to verify | R: `uikit/uibutton/configuration-swift.struct/indicator-swift.enum`, `indicatorColorTransformer`; M-r | `arrow_icon_size_px` (size only) |
| CB-2 | combo_box | The button style of the trigger (any of B-1's styles) | same | R | `combo_box.background_color` (one) |
| CB-3 | combo_box | Open state of the trigger, and the opened menu (MN-*) with a checkmark on the current item | same | M-r | none. Seed: iced styles.rs:1182 ("no separate open appearance") |
| CB-4 | combo_box | Text colour: `label` vs tint in 26 | 26 differs | Seed: docs/todo.md *ios-26* (RenderThemeCocoa.mm@de320ba3:3373-3376) | `combo_box.font.color` (values only; listed for the seed) |

### segmented_control (UISegmentedControl)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| SG-1 | segmented_control | The selected "thumb" as a part of its own: inset from the track, its own corner radius, shadow layers, thin outline | 26 differs (capsule track and thumb) | M-r, M-q | `active_background` (fill only), `border.corner_radius_px` (one radius) |
| SG-2 | segmented_control | Font weight per state (the selected segment uses a heavier weight) | same | R: `titleTextAttributes(for:)` (defaults nil → M-q) | `font`, `active_text_color` (colour only) |
| SG-3 | segmented_control | Divider colour, and dividers hidden next to the thumb | same | M-r | `separator_width_px` (width only) |
| SG-4 | segmented_control | Pressed state: the thumb shrinks while held (13–18); a glass lens in 26; pressed unselected segment dims | 26 differs | M-v | none. Seed: iced styles.rs:1927-1928 ("no hover … no pressed") |
| SG-5 | segmented_control | Thumb slide motion, and dragging the thumb across segments | 26 differs | M-v | none |
| SG-6 | segmented_control | Segment image size | same | M-q | none |
| SG-7 | segmented_control | Selected text colour. iOS draws `label` on a light thumb, not white on accent: the field exists but its meaning differs per platform. | same | M-r | `active_text_color` (values only) |

### card

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| CD-1 | card | iOS's card-like surfaces are cell / list backgrounds and SwiftUI GroupBox (4.2 C-32). `UIBackgroundConfiguration` carries cornerRadius, backgroundInsets, strokeColor/Width/Outset, visualEffect and shadowProperties (iOS 18 `UIShadowProperties`: color, offset, opacity, path, radius). The model's card has no stroke outset, no inset, no visual effect and no shadow geometry. | same; 26 concentric | R: `uikit/uibackgroundconfiguration-swift.struct`, `UIShadowProperties` (iOS 18.0) | `card.background_color`, `card.border` (partial) |

### expander (outline disclosure; SwiftUI DisclosureGroup)

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| EX-1 | expander | Glyph identity (chevron) and its angle collapsed vs expanded | same | R: `UICellAccessory.outlineDisclosure` options; M-r | `arrow_icon_size_px`, `arrow_color` (no glyph or angles). Seed: docs/todo.md:729 ("the model states no shape") |
| EX-2 | expander | Expand/collapse motion: chevron rotation and child rows' insertion | same | M-v | none |
| EX-3 | expander | Outline style `.header` (the whole header toggles; header look) vs `.cell` | same | R: `UICellAccessory.OutlineDisclosureOptions.style` | none |
| EX-4 | expander | Chevron tint in header style (tint vs secondary) | to verify | M-r | `arrow_color` (values only) |

### link

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| LK-1 | link | Pressed highlight: a grey rounded box behind the tapped link in a text view | same | M-r | `link.active_text_color` (colour only; no pressed background) |
| LK-2 | link | Long-press preview (context menu preview, 4.2 C-13) | same | — | none |

### layout

| ID | Element | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| LY-1 | layout | Size-class-dependent margins (compact vs regular width) | same | R: `uikit/uiviewcontroller/systemminimumlayoutmargins` | `layout.window_margin_px` (one) |
| LY-2 | layout | Readable content width | same | R: `readableContentGuide` | none |
| LY-3 | layout | System spacing between views, which scales with Dynamic Type | same | M-q (`constraint(equalToSystemSpacingAfter:)` resolved) | `widget_gap_px` (fixed) |
| LY-4 | layout | Spacing scaled with text (`UIFontMetrics.scaledValue(for:)`) | same | R: `uikit/uifontmetrics` | none |

---

## 4.2 Standard controls the model does not have

| ID | Control | What the model would need to carry | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|---|
| C-1 | Navigation bar (UINavigationBar) | Bar heights: standard, large-title, compact (landscape), and with search or prompt. Background per appearance (standard / scrollEdge / compact / compactScrollEdge) with material and hairline. Title font and colour. Large-title font (Large Title, emphasized). Large-title collapse on scroll (motion). Prompt text. Subtitle (`subtitle`, `largeSubtitle`). Title menu chevron (`titleMenuProvider`). Overflow button. | 26 differs: no bar background, items in glass capsules, scroll edge effect; 27.2 `titleAlignment` (to verify) | R: `uikit/uinavigationbar`, `UINavigationBarAppearance` (titleTextAttributes, largeTitleTextAttributes, subtitleTextAttributes), `uikit/uinavigationitem` (subtitle, largeSubtitle, titleAlignment); heights M-q; motion M-v | `toolbar.*`, `window.title_bar_*` (partial) |
| C-2 | Back button | Back chevron glyph (`backIndicatorImage`), back title and its truncation to a generic title, `backButtonDisplayMode`, pressed state, push/pop label transition | 26 differs (glass capsule) | R: `UINavigationItem.backButtonDisplayMode`, `UINavigationBarAppearance.backButtonAppearance`; M-r, M-v | `IconRole::NavBack` (glyph only) |
| C-3 | Search field (UISearchBar / UISearchTextField) | Field fill, corner, height, magnifier glyph and colour, placeholder, clear button. Cancel button with its show/hide motion. Scope bar. Search tokens (fill, text, radius). Bookmark/results buttons. `searchBarStyle` variants default / prominent / minimal. | 26 differs: glass capsule in a toolbar or the tab bar, button that expands into a field above the keyboard (HIG search-fields) | R: `uikit/uisearchbar` (searchBarStyle, scope bar), `uikit/uisearchtextfield` (`tokenBackgroundColor`), `uikit/uisearchtoken`; P: HIG search-fields; M-r, M-v | `input.*` (partial) |
| C-4 | Stepper (UIStepper) | Two-segment body: fill, divider, corner radius, size, plus/minus glyphs and colour, pressed segment, disabled segment at a limit, autorepeat timing | 26 to verify | R: `uikit/uistepper` (autorepeat), size M-q; M-r, M-v | `segmented_control.*` (partial) |
| C-5 | Date picker — compact | Pill buttons (date and time) with a fill and accent text (HIG pickers: "shows the current value in your app’s accent color"), font, pressed state, the popover it opens | 14+; 26 to verify | R: `uikit/uidatepickerstyle` (compact); P: HIG pickers; M-r | `combo_box.*` (partial) |
| C-6 | Date picker — inline / UICalendarView | Month header and navigation chevrons, weekday row, day cells: today, selected (fill shape and text), disabled, out-of-range. Decorations (dot / image / custom) with their size and colour. Month-year wheel toggle. Time row. `fontDesign`. | 14+/16+; 26 to verify | R: `uikit/uicalendarview` (fontDesign, Decoration, DecorationSize); M-r | none |
| C-7 | Picker wheel (UIPickerView, UIDatePicker `.wheels`, countdown) | Row height, visible rows, row font, perspective curvature and fading of off-centre rows, selection band (fill, radius, insets), component gap, momentum | same; 26 to verify | R: `rowSize(forComponent:)`; M-r, M-v | none |
| C-8 | Colour well (UIColorWell) | Ring (multicolour) and inner swatch, sizes, pressed state; the system colour-picker sheet it opens (in-app presentation: to verify whether its drawing is themable) | 14+ | R: `uikit/uicolorwell`; M-r | none |
| C-9 | Page control (UIPageControl) | Dot size and spacing, current vs other colours. Background styles automatic / prominent / minimal: a translucent rounded-rect platter while interacting (HIG page-controls). Shrinking edge dots when overflowing (HIG). Scrubbing state. Timer progress (17+: current indicator elongates and fills). Vertical direction. | 14+/17+; 26 to verify | R: `uikit/uipagecontrol` (pageIndicatorTintColor, backgroundStyle, progress, direction, `size(forNumberOfPages:)`); P: HIG page-controls; M-r, M-v | none |
| C-10 | Refresh control (UIRefreshControl) | Spokes appearing as the pull progresses, trigger distance, tint, title font and colour, the collapse after `endRefreshing` | 26 to verify | R: `uikit/uirefreshcontrol` (tintColor, attributedTitle); M-v | `spinner.*` (partial) |
| C-11 | Action sheet (UIAlertController `.actionSheet`) | 13–18: bottom platters (actions + separate cancel platter), material, radius, separators, title/message fonts, action font and colours (cancel bold, destructive red), screen-edge margins, slide-up motion. On iPad it shows in a popover. | 26 differs (anchored to the source: to verify) | P: HIG action-sheets; M-r, M-v | `dialog.*` (partial) |
| C-12 | Sheet (UISheetPresentationController, page / form sheet) | Corner radius (`preferredCornerRadius` default). Grabber (size, colour, offset). Detents: medium/large heights. Dim per detent (`largestUndimmedDetentIdentifier`). 13–18 stacked-card effect (presenter scales down and rounds, black behind). iPad page/form sheet sizes (`prefersPageSizing`). Edge-attached compact-height variant. Interactive dismiss. Presentation spring. | 26 differs: floating inset glass sheet at partial detents, opaque and edge-attached at large (to verify) | R: `uikit/uisheetpresentationcontroller` (prefersGrabberVisible, preferredCornerRadius, detents, largestUndimmedDetentIdentifier, prefersPageSizing, prefersEdgeAttachedInCompactHeight); P: HIG sheets (grabber, detents); M-q, M-r, M-v | `dialog.*` (partial) |
| C-13 | Context-menu preview (UIContextMenuInteraction) | Lifted preview: scale, corner radius (clipping path), shadow. Blur and dim of everything else (HIG context-menus: "dimming the screen behind the preview and the menu"). Menu placement next to the preview. Lift and dismiss motion. | 26 to verify | R: `uikit/uicontextmenuinteraction`, `uikit/uipreviewparameters`; P: HIG context-menus; M-r, M-v | none |
| C-14 | Edit menu (UIEditMenuInteraction; UIMenuController before 16) | Compact horizontal pill: fill/material, item font, dividers, paging chevrons, pointer arrow toward the selection. Vertical style for pointer/keyboard (HIG edit-menus). | 13–15 callout bar vs 16+ edit menu; 26 glass (to verify) | R: `uikit/uieditmenuinteraction`; P: HIG edit-menus; M-r | `menu.*` (partial) |
| C-15 | Badges | Tab-bar badge (fill, text font/colour, size, dot vs count, position). Bar-button badge (26: count / indicator / string; backgroundColor / foregroundColor / font). SwiftUI list-row `.badge` (trailing secondary text). | 26 adds bar-item badges | R: `UITabBarItem.badgeColor`/`badgeTextAttributes`, `uikit/uibarbuttonitem/badge-swift.struct`; M-r | `defaults.danger_color` (partial) |
| C-16 | Content unavailable view (UIContentUnavailableConfiguration, 17+) | Image size and tint, text and secondary-text fonts and colours, paddings (imageToText, textToSecondaryText, textToButton), buttons, alignment, background; the `.loading()` and `.search()` presets | 17+ | R: `uikit/uicontentunavailableconfiguration-swift.struct` | none |
| C-17 | Paste control (UIPasteControl, 16+) | Secure system paste button: display mode (icon / label / both), corner style, base background and foreground | 16+ | R: `uikit/uipastecontrol` (`UIPasteControl.Configuration`); M-r | `button.*` (partial) |
| C-18 | Scroll edge effect | Soft (blur + fade) vs hard edge under bars; its height, blur and tint | 26 only | R: `uikit/uiscrolledgeeffect` (`style`: automatic / hard / soft, `isHidden`), `UIScrollView.topEdgeEffect` etc.; P: HIG materials ("Scroll edge effects further enhance legibility by blurring and reducing the opacity of background content"); M-r | none |
| C-19 | Background extension | Content mirrored/blurred under a floating sidebar or inspector | 26 only | R: `uikit/uibackgroundextensionview`; M-r | none |
| C-20 | Glass container / glass control | Any custom glass element: regular / clear, tint, interactive, container spacing and merging (X-MT-5…9) | 26 only | R: `uikit/uiglasseffect`, `uikit/uiglasscontainereffect` | none |
| C-21 | Tab-bar bottom accessory (UITabAccessory) | Capsule glass accessory above or inline with the tab bar: height, radius, insets; inline vs expanded environments; transition | 26 only | R: `uikit/uitabaccessory`, `UITraitCollection.tabAccessoryEnvironment`; P: HIG tab-bars; M-r, M-v | none |
| C-22 | iPad sidebar-adaptable tab bar | Top tab bar ↔ sidebar conversion: bar shape, toggle button, morph | 18+; 26 differs | R: `UITabBarController.Mode.tabSidebar`; P: HIG tab-bars (iPadOS); M-r, M-v | `tab.*` (partial) |
| C-23 | Keyboard (system-drawn) and input accessory | The keyboard's appearance is the app's choice (`UIKeyboardAppearance` default / light / dark), so it must match the theme. Show/hide motion (duration and curve). Input-accessory / keyboard-toolbar background. | 26 differs (keyboard look; accessory glass: to verify) | R: `uikit/uikeyboardappearance`; duration and curve R: the keyboard notifications' animation-duration and animation-curve userInfo keys | none |
| C-24 | Drag and drop (UIDragInteraction) | Lift preview (scale, shadow, corner), drag-count badge, drop-target highlight, cancel/drop motion, lift delay | same | R: `uikit/uidraginteraction`, `uikit/uidragpreviewparameters`; M-r, M-v | none. Seed: iced styles.rs:1628 ("no drop target") |
| C-25 | Find navigator (UIFindInteraction) | Find bar above the keyboard: fields, chevrons, result count, options menu; highlights (TA-2) | 16+ | R: `uikit/uifindinteraction`; M-r | none |
| C-26 | Large Content Viewer HUD | Accessibility-size HUD shown on long-press of bar items: material, size, radius, symbol and text | 13+ | R: `UILargeContentViewerInteraction` (*to verify* page); M-r | none |
| C-27 | iPadOS window controls and Stage Manager frame | See W-1, W-2, W-3 (4.1): listed here because the model has no window-control part at all | 16+/26 | — | none |
| C-28 | Navigation title menu / document header | Title with chevron and document-properties header (16+) | 16+ | R: `UINavigationItem.titleMenuProvider`, `documentProperties`; M-r | none |
| C-29 | Split-view display-mode button | Sidebar toggle button glyph and placement | same | R: `displayModeButtonItem`; M-r | none (no `IconRole` for it) |
| C-30 | Gauge (SwiftUI, 16+) | Circular and linear accessory gauges, linear capacity: track, fill, thickness, end caps, gradient | 16+ | M-r | `progress_bar.*` (partial) |
| C-31 | SwiftUI ProgressView circular (determinate) | Whether iOS draws a determinate circular progress, and its geometry | to verify | M-r | `spinner.*` (partial) |
| C-32 | GroupBox (SwiftUI) | Rounded surface, fill, padding, label font | 14+ | M-r | `card.*` (partial) |
| C-33 | Writing Tools panel / glow | The in-text glow animation on app text (TA-4); the panel itself is system-drawn (*to verify* whether it is in-process) | 18.1+ | M-v | none |
| C-34 | Share sheet (UIActivityViewController) | System-drawn, presented as a sheet. The sheet's chrome follows C-12; its content is not themable. Listed so it is not mistaken for a gap. | — | R: `uikit/uiactivityviewcontroller` | n/a |
| C-35 | Toggle as button (SwiftUI `.button` toggle style; HIG toggles "use a button that behaves like a toggle") | Highlight background when on | same | P: HIG toggles; M-r | `button.checked_background` (exists; listed for mapping only) |

---

## 4.3 Properties that cross widgets

### Shadows

| ID | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|
| X-SH-1 | Shadow geometry: offset x/y, blur radius, spread (CALayer has none, but `shadowPath` can enlarge), colour and opacity, and the **number of layers**, per element. Elements: menu, context-menu preview, popover, sheet (26 floating), alert (to verify), segmented thumb, switch thumb, slider thumb, floating tab bar and toolbar glass groups (26), drag lift, pointer lift, Stage Manager windows, 13–18 card sheets. | 26 differs everywhere (glass "adaptive" shadows) | M-q (system views' `layer.shadow*`, *to verify* they are set on accessible layers), M-r (fit against a known background) | `defaults.shadow_color`, `*.border.shadow_enabled` |
| X-SH-2 | UIKit's own shadow model `UIShadowProperties` (color, offset, opacity, path, radius) used by background configurations: its defaults per configuration | 18+ | R: `UIBackgroundConfiguration.shadowProperties` | none |
| X-SH-3 | Shadow differences between light and dark variants and with Increase Contrast | same | M-r | `shadow_color` per variant (colour only) |

### Materials, vibrancy, Liquid Glass

| ID | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|
| X-MT-1 | Blur materials: `systemUltraThinMaterial`, `systemThinMaterial`, `systemMaterial`, `systemThickMaterial`, `systemChromeMaterial` (+ explicit Light/Dark) and the legacy styles. What each draws: blur radius, saturation, tint overlay colour and alpha, per light/dark. Not published numerically. | same (HIG materials: iOS "continue to provide four standard materials" in 26) | R: `uikit/uiblureffect/style` (names only); M-r (render over a calibrated pattern, fit blur and tint) | none (colour alpha only) |
| X-MT-2 | Which element uses which material (bars, menus, alerts, sheets, popovers, sidebars, action sheets) | 26 differs (glass) | R: `UIBarAppearance.backgroundEffect` for bars; M-r for the rest | none |
| X-MT-3 | Reduce Transparency fallback: each material's opaque replacement colour | same; 26 glass "frostier" | R: `isReduceTransparencyEnabled`; M-r with the setting on (whether `simctl` can toggle it is to verify) | `AccessibilityPreferences.reduce_transparency` (flag only) |
| X-MT-4 | Vibrancy: label ×4, fill ×3 and separator vibrancy styles over each material (blend-mode rendering) | same | R: `uikit/uivibrancyeffectstyle`; P: HIG materials (iOS vibrancy levels); M-r | none |
| X-MT-5 | Liquid Glass variants `regular` / `clear`: blur, luminosity adjustment, translucency (HIG materials) | 26 only | R: `uikit/uiglasseffect/style`; P: HIG materials "Liquid Glass"; M-r | none |
| X-MT-6 | Glass tint (`tintColor`) and "stained glass" prominent styling | 26 only | R: `UIGlassEffect.tintColor`; P: HIG color "Liquid Glass color"; M-r | none |
| X-MT-7 | Glass optics: lensing / refraction at edges, specular highlight, adaptive light↔dark flip of small glass elements over content (HIG color), more opacity in large elements (HIG color) | 26 only | P: HIG color, HIG materials (qualitative); M-r over light and dark content | none |
| X-MT-8 | Interactive glass response: press scale, highlight, shimmer | 26 only | R: `UIGlassEffect.isInteractive`; M-v | none |
| X-MT-9 | Glass merging and morphing between nearby elements (`UIGlassContainerEffect.spacing`) | 26 only | R: `uikit/uiglasscontainereffect/spacing` (default value R); M-v | none |
| X-MT-10 | Clear-glass dimming layer over bright content (HIG materials: "consider adding a dark dimming layer of 35% opacity") | 26 only | P: HIG materials (read 2026-10-27) | none |
| X-MT-11 | User preference for the Liquid Glass look (HIG materials: "people choose a preferred look for Liquid Glass in their device’s settings"); its API is not found (to verify) | 26 (26.x) only | P: HIG materials; R: to verify | none |
| X-MT-12 | Increase Contrast glass variant (borders, higher contrast) | 26 only | P: HIG materials (mentions it); M-r | `high_contrast` flag only |
| X-MT-13 | Generation opt-out: `UIDesignRequiresCompatibility` (Info.plist) keeps the pre-26 look on iOS 26, so on iOS 26 either generation can be on screen. A reader must choose `ios-13` vs `ios-26` per app, not per OS version. | 26 | P: `bundleresources/information-property-list/uidesignrequirescompatibility` (page exists, 2026-10-27); R: the host app's Info.plist | none (preset choice only) |

### Corners

| ID | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|
| X-CR-1 | Corner curve: continuous (superellipse-like) vs circular, for every rounded element | same | R: `CALayer.cornerCurve` on system views (M-q) | `corner_radius_px` (circular implied) |
| X-CR-2 | Concentric corners: radius derived from the container (`UICornerRadius.containerConcentric(minimum:)`), capsules with a maximum radius | 26 only | R: `uikit/uicornerconfiguration-swift.struct`, `UIView.effectiveRadius(corner:)` (iOS 26.0) | `corner_radius_px`, `corner_radius_lg_px` (fixed numbers) |
| X-CR-3 | Per-corner radii (sheets: only the top corners when edge-attached; `UICornerConfiguration.corners(topLeftRadius:…)`) | same / 26 API | R: `UICornerConfiguration`; M-r | none (one radius) |
| X-CR-4 | Device display corner radius as the outermost container (W-6) | 26 matters | M-r | none |

### Motion

| ID | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|
| X-MO-1 | No motion model: duration, easing, spring (duration + bounce, or mass / stiffness / damping), delay. The only motion data today is `dialog.entrance` (enum) and spinner frames. | same | — | `dialog.entrance`, `AnimatedIcon` |
| X-MO-2 | UIKit's default spring: the default parameters of `animate(springDuration:bounce:…)` (17+) and `UISpringTimingParameters()` | same | R: `uikit/uiview/animate(springduration:bounce:initialspringvelocity:delay:options:animations:completion:)` (default arguments: to verify they are in the signature), `uikit/uispringtimingparameters` | none |
| X-MO-3 | Navigation push/pop: duration, curve, parallax of the covered view, edge shadow or dim, title cross-fade, large-title collapse | 26 to verify | M-v | none |
| X-MO-4 | Modal transitions: coverVertical, crossDissolve, flipHorizontal, partialCurl, zoom (18+); sheet detent changes | 18 adds zoom | R: `uikit/uimodaltransitionstyle`, `uikit/uiviewcontroller/transition`; M-v | none |
| X-MO-5 | Menu, context menu, popover, alert, action sheet: appear and disappear (MN-14, C-13, PO-5, DL-10/11, C-11) | 26 differs (morph from source) | M-v | `dialog.entrance` (partial) |
| X-MO-6 | Control motions: switch toggle, segmented thumb, slider value, page-control scrub, tab selection (26 lens), tab-bar minimize (26), button press (26 interactive glass; 13–18 highlight fade), list highlight fade, row animations, expand/collapse, refresh | 26 differs | M-v | none |
| X-MO-7 | Keyboard show/hide duration and curve | same | R: keyboard-notification userInfo | none |
| X-MO-8 | Scroll physics: deceleration rate (normal / fast), rubber-band and bounce, paging snap | same | R: `uikit/uiscrollview/decelerationrate-swift.struct` (`normal`, `fast` raw values); bounce M-v | none |
| X-MO-9 | SF Symbol effects: bounce, pulse, variable colour, scale, appear/disappear, replace, wiggle, rotate, breathe; draw on/off in 26 (to verify). Each has timing. | 17+ / 18+ / 26 | R: `symbols/symboleffect`; M-v | none |
| X-MO-10 | Reduce Motion alternatives: cross-fade instead of slide/zoom, no parallax, reduced glass motion; the separate Prefer Cross-Fade Transitions flag | same / 26 | R: `isReduceMotionEnabled`, `uikit/uiaccessibility/preferscrossfadetransitions`; M-v with settings on | `AccessibilityPreferences.reduce_motion` (flag only; no cross-fade flag) |

### Dynamic Type and typography

| ID | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|
| X-DT-1 | Per-style ramps across 12 content-size categories (TS-2), applied to every widget font (D-5) | same | P/R (TS-2) | `text_scaling_factor` |
| X-DT-2 | Control metrics scaling with text: row heights, button heights, bar heights (bars clamp and use the Large Content Viewer instead), paddings, symbol sizes | same | R: `UIFontMetrics`, `maximumContentSizeCategory` (15+); M-q per category | none |
| X-DT-3 | Accessibility-size layout changes (stacking; HIG typography) | same | R: `isAccessibilityCategory`; P: HIG typography | none |
| X-DT-4 | Bold Text (TS-3) for every font and symbol weight | same | R: `isBoldTextEnabled`, `legibilityWeight` | none |
| X-DT-5 | Tracking (D-1), leading (D-4), optical size (D-2) for every role | same | P: HIG typography | none |

### Colour system

| ID | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|
| X-CO-1 | A variant axis beyond light/dark: base vs elevated, normal vs increased contrast, active vs inactive scene. iOS dynamic colours resolve on all of them (D-14, D-15, W-4). | same | R: `UIColor.resolvedColor(with:)` over trait combinations | `light`/`dark` `ThemeMode` only |
| X-CO-2 | Semantic colour inventory: labels ×4, fills ×4, backgrounds ×3 + grouped ×3, separators ×2, placeholder, link, the 12 system colours and 6 greys (D-9…D-13) | 26 values differ | R: `uikit/ui-element-colors`, `uikit/standard-colors`; P: HIG color | defaults (partial) |
| X-CO-3 | Dimmed tint (D-16) | same | R / M-r | none |
| X-CO-4 | Wide gamut (D-17) | same | R | sRGB `Rgba` |
| X-CO-5 | Colour prominence: a `UIColor.Prominence` / `withProminence(_:)` API exists. Its iOS version and what it draws are to verify. | to verify (26+) | R: `uikit/uicolor` ("Working with color prominence"; page for the type not fetched) | none |

### Caret

| ID | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|
| X-CA-1 | Caret width and height (relative to the line), end rounding | 17 changed the caret (to verify which parts) | R: `uikit/uitextinput/caretrect(for:)` (in the probe; also `UITextSelectionDisplayInteraction.cursorView`, 17+) | none (`input.caret_color` only) |
| X-CA-2 | Blink: on/off durations, fade vs hard blink, pause while typing; a non-blinking preference (no `prefersNonBlinkingTextInsertionIndicator` page found, so whether iOS has one is to verify) | same | M-v | none |
| X-CA-3 | Caret accessories: dictation and language indicator, the floating (trackpad-mode) ghost caret | 17+ | M-r, M-v | none |

### Selection

| ID | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|
| X-SE-1 | Text selection highlight colour (tint at a reduced alpha: to verify), corner, fade | 17 changed | R: `uikit/uitextselectiondisplayinteraction` (`highlightView`; colour by M-q); M-r | `defaults.text_selection_background/color`, `input.selection_*` (colour only) |
| X-SE-2 | Selection handles: knob diameter, stem width, colour, drag magnifier/loupe | 17 changed | R: `UITextSelectionDisplayInteraction.handleViews` (M-q); M-r | none |
| X-SE-3 | List and grid selection: grey touch highlight vs tint selection, per context (L-10) | same | M-r | `defaults.selection_*` (partial) |

### Focus and pointer

| ID | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|
| X-FO-1 | Focus halo shape (follows the view's corners and curve), inside/outside position, width, colour (D-18) | same | R: `uikit/uifocushaloeffect`; M-r | `focus_ring_*` (partial) |
| X-FO-2 | Focus "highlight" effect on cells (tint fill) and focus movement animation | same | M-r, M-v | none |
| X-PH-1 | Pointer effects (iPadOS): highlight (platter behind, pointer morphs into it), lift (scale, shadow, specular), hover (tint overlay/underlay, shadow, scale); UIHoverStyle (17+) shapes | same | R: `uikit/uipointereffect-swift.enum`, `uikit/uihoverstyle`; M-v | `*.hover_background` (colour only) |
| X-PH-2 | The pointer itself: shape, size, colour adaptation, morphing; drawn by the system over app content | same | M-r | none (note: system-drawn) |

### Hairlines, scrims, timings

| ID | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|
| X-HL-1 | One-device-pixel lines everywhere (bars, lists, alerts, menus): SE-1 | same | R: `displayScale` | `line_width_px` (logical) |
| X-SC-1 | Scrims and dims: alert dim, sheet dim per detent, context-menu blur + dim, action-sheet dim, the 13–18 card-sheet background, clear-glass dimming (X-MT-10). Colour, opacity and blur. | 26 differs | M-r | none |
| X-BT-1 | Visible timings: long-press duration (context menus, drag lift), touch-highlight delay in scroll views (`delaysContentTouches`), caret blink, scroll-indicator fade, tooltip show/hide, stepper autorepeat, swipe and refresh thresholds | same | R: `uikit/uilongpressgesturerecognizer/minimumpressduration` (default), `UIScrollView.delaysContentTouches` (bool; the delay itself M-v); the rest M-v | `tooltip.show_delay_ms` only |

### Size variants and environment

| ID | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|
| X-SV-1 | Size classes (compact/regular, both axes) and idiom (phone/pad). Bars, margins, tab layout, popover vs sheet and alert width all change. | same | R: `UITraitCollection.horizontalSizeClass/verticalSizeClass/userInterfaceIdiom` | none (no variant axis) |
| X-SV-2 | Control sizes. UIKit has no general control size (`uikit/uicontrol/controlsize` returns 404, so this is to verify). What it has: `UIButton.Configuration.Size` mini/small/medium/large, `UIActivityIndicatorView.Style` medium/large, bar metrics default/compact. SwiftUI `ControlSize` (mini / small / regular / large / extraLarge: availability on iOS to verify). | same | R (above); M-q per size | none |
| X-SV-3 | Layout direction (RTL): mirrored chevrons and glyph placement | same | R: `layoutDirection` | none |

### Accessibility variants (beyond the four flags the model has)

| ID | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|
| X-A11-1 | Increase Contrast **data**: colour variants (D-15), extra borders on controls, glass contrast (X-MT-12) | same / 26 | R: `isDarkerSystemColorsEnabled`, `accessibilityContrast`; P: HIG color tables; M-r | `high_contrast` flag only |
| X-A11-2 | Reduce Transparency data (X-MT-3) | same | as X-MT-3 | `reduce_transparency` flag only |
| X-A11-3 | Reduce Motion data (X-MO-10) | same | as X-MO-10 | `reduce_motion` flag only |
| X-A11-4 | Bold Text: no flag, no data | same | R: `isBoldTextEnabled` | none |
| X-A11-5 | Button Shapes: text buttons gain an underline or background shape (look per control; 26 to verify). No flag, no data. | same | R: `uikit/uiaccessibility/buttonshapesenabled` (deprecated; its replacement is to verify); M-r | none |
| X-A11-6 | On/Off Labels: no flag; switch glyphs (SW-4) | same | R: `isOnOffSwitchLabelsEnabled` | none |
| X-A11-7 | Differentiate Without Color: no flag; which system controls change is to verify | same | R: `uikit/uiaccessibility/shoulddifferentiatewithoutcolor`; M-r | none |
| X-A11-8 | Smart Invert: per-element exemption (`accessibilityIgnoresInvertColors`). The theme cannot mark elements that must not invert. | same | R: `isInvertColorsEnabled`, `uikit/uiview/accessibilityignoresinvertcolors` | none |
| X-A11-9 | Prefer Cross-Fade Transitions: no flag (X-MO-10) | same | R | none |
| X-A11-10 | Content-size **category** (incl. "is accessibility size"), not just a factor (TS-2, X-DT-3) | same | R: `preferredContentSizeCategory` | `text_scaling_factor` (partial) |
| X-A11-11 | Grayscale and Classic Invert: system-wide filters with no app-side paint change. No gap, noted for completeness. | — | R: `isGrayscaleEnabled` | n/a |

### Icons (SF Symbols)

| ID | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|
| X-IC-1 | Rendering mode per element: monochrome, hierarchical, palette, multicolor | same (15+ modes) | R: `uikit/uiimage/symbolconfiguration-swift.class`; M-r | none (icon colour is the caller's) |
| X-IC-2 | Symbol weight (nine) and scale (small/medium/large) per context, and weight matching adjacent text. The loader fixes Regular / Medium (`native-theme/src/sficons.rs:28-40`). | same | R: `symbolweight`, `symbolscale` | `icon_sizes` (size only) |
| X-IC-3 | Text-style-relative symbol size (`SymbolConfiguration(textStyle:)`) that follows Dynamic Type | same | R | `icon_sizes.*_px` (fixed) |
| X-IC-4 | Variant per state: `.fill` for selected tab items (to verify it is automatic), `.circle`, `.slash`; variable value | same | M-r | `IconRole` → one name |
| X-IC-5 | Missing `IconRole`s for glyphs UIKit draws: disclosure chevron, outline chevron, clear (xmark in circle), more (ellipsis), share, checkmark, delete/insert accessories, reorder grabber, pop-up chevrons, info/detail, sidebar toggle, close button. Which symbol UIKit draws for each is to verify by M-q of the accessory's image. | same; 26 glyphs to verify | M-q, M-r | `IconRole` (`icons.rs:42-139`: none of these) |
| X-IC-6 | Symbol effects (X-MO-9) | 17+ | R | none |
| X-IC-7 | iOS icon loading. The SF Symbols loader is AppKit-only (`NSImage`, `sficons.rs:1-40`), so iOS has no loader (`UIImage(systemName:)`). This is a reader gap, not a model gap. | same | R | `SfSymbolsLoader` (macOS only) |

### Reader and versions

| ID | What is missing | 13–18 / 26 | Source | Nearest |
|---|---|---|---|---|
| X-RD-1 | No iOS reader. The readers are macOS, Windows, KDE and GNOME (`native-theme/src/`: `macos.rs`, `windows.rs`, `kde/`, `gnome/`). Every **R** row above needs one: dynamic colours over traits, preferred fonts, the accessibility flags, layout margins. | same | R: UIKit via objc2 (crate choice to verify) | none |
| X-VR-1 | Sub-generation drift inside 13–18 that one `ios-13` preset cannot carry: 13–14 uppercase grouped headers; 15 transparent bars at the scroll edge and button configurations; 16 edit menu; 17 caret, selection and page progress; 18 iPad top tab bar and zoom transition. Plus 26 vs 27.x additions. | — | M per runtime (availability on the CI image to verify) | preset granularity |

---

## Notes

- **Not counted as gaps**, because the model has the field and only the value is open: `tooltip.show_delay_ms`,
  `switch.track_width/height`, `spinner.fill_color`, `button.min_height`, every colour the preset leaves unstated.
  The 38 unsourced `ios-26` leaves (docs/todo.md *ios-26*) are value gaps too; this map lists them only where
  the model is also short (SL-2, SW-3, L-18, D-20, CB-4).
- **App parameters, not theme**: slider `neutralValue`, page count, detents chosen by the app, swipe-action
  titles. The rows record only how the platform *draws* them.
- **Everything marked "to verify" is unestablished.** In particular: what iOS 26 changed in alert layout, the
  popover arrow, menu icon side, the close button and the switch; a Liquid Glass preference API; a non-blinking
  caret preference; UIKit control sizes; SwiftUI `extraLarge`; `UIColor.Prominence`; what the iOS 27.x bar APIs
  draw.
