# Gap map — macOS (12–15 Aqua, `macos-monterey`; 26–27 Liquid Glass, `macos-tahoe`)

Model state: `v0.7.1-rc1` at caeddd6b (`docs/property-registry.toml`, `native-theme/src/model/`,
`AccessibilityPreferences` in `native-theme/src/lib.rs:415-437`, reader `native-theme/src/macos.rs`).
Method: `docs/todo_v0.7.1_model-completeness-gap-map.md` §2–§4.

## Conventions of this file

- **Gen** column: `both` = drawn in both generations; `12–15 only` / `26 only`; `differs` = both
  draw it but differently (the difference is named). "26" means 26 and 27 (the macos-tahoe column).
- **Source** column, in §3's order: **R** read at run time (the API or setting), **P** published (URL
  or source file), **M** measure (concrete method). "to verify" marks an API or key whose existence
  or behaviour this pass did not confirm. No value is given anywhere in this file; the few numbers
  quoted are what an existing probe file or `platform-facts.md` line already states, cited.
- APIs confirmed to exist on 2026-10-27 from Apple's documentation JSON
  (`https://developer.apple.com/tutorials/data/documentation/appkit/<symbol>.json`):
  `NSControl.ControlSize.extraLarge`; `NSGlassEffectView` (26.0: `cornerRadius`, `tintColor`,
  `style`, `effectIsInteractive`); `NSGlassEffectContainerView` (26.0: `spacing`);
  `NSButton.BezelStyle.glass`; `NSBackgroundExtensionView` (26.0); `NSView.cornerConfiguration`,
  `NSView.effectiveCornerRadii`, `NSView.prefersCompactControlSizeMetrics` (26);
  `NSScrollEdgeEffectStyle` (26.1: automatic / hard / soft; `preferredScrollEdgeEffectStyle` on
  title-bar accessories); `NSScrollView.refreshController` (27.0); `NSSlider.neutralValue`,
  `NSSlider.tintProminence`, `NSTintProminence`; `NSToolbarItem.style` (plain / prominent),
  `NSToolbarItem.backgroundTintColor`, `NSToolbarItem.badge` / `NSItemBadge`;
  `NSSegmentedControl.borderShape` / `NSControl.BorderShape` (the BorderShape page itself did
  not load: its cases are "to verify"); `NSPopover.hasFullSizeContent`; `NSMenuItem.sectionHeader(title:)`,
  `NSMenuItem.subtitle`, `NSMenuItemBadge` (14.0); `NSTextInsertionIndicator` (14.0: `color`,
  `displayMode`, `automaticModeOptions`, `effectsViewInserter`); `NSEvent.doubleClickInterval`,
  `keyRepeatDelay`, `keyRepeatInterval`; `NSWorkspace.accessibilityDisplayShouldDifferentiateWithoutColor`,
  `…ShouldInvertColors`; `NSAppearance.Name.accessibilityHighContrastAqua` / `…DarkAqua` /
  `…VibrantLight` / `…VibrantDark`, `vibrantLight`, `vibrantDark`; `NSVisualEffectView.Material`
  (titlebar, selection, menu, popover, sidebar, headerView, sheet, windowBackground, hudWindow,
  fullScreenUI, toolTip, contentBackground, underWindowBackground, underPageBackground);
  `NSWindow.ToolbarStyle` (automatic, expanded, preference, unified, unifiedCompact);
  `NSWindow.AnimationBehavior` (default, none, documentWindow, utilityWindow, alertPanel);
  `NSTableView.RowSizeStyle` (default = "the system default layout size: small, medium or large");
  `NSScrollView.scrollerKnobStyle`, `flashScrollers()`, `findBarPosition`, `…ScrollElasticity`;
  `NSBox.titleFont` / `titlePosition` / `boxType`; `NSTabView.tabPosition` / `tabViewBorderType` /
  `tabViewType`; `NSLevelIndicator.fillColor` / `warningFillColor` / `criticalFillColor` /
  `drawsTieredCapacityLevels` / `ratingPlaceholderImage` / `placeholderVisibility`;
  `NSColorWell.colorWellStyle` / `maximumLinearExposure`; `NSSearchField.cancelButtonBounds` /
  `searchButtonBounds` / `searchTextBounds`; `NSDatePicker.datePickerStyle` / `presentsCalendarOverlay`;
  `NSTokenField.TokenStyle`; `NSPathControl.Style`; `NSSegmentedControl.selectedSegmentBezelColor` /
  `trackingMode` / `segmentStyle` / `showsMenuIndicator(forSegment:)`.
- No caret-blink API exists in `NSEvent` (checked). The blink periods are the undocumented user
  defaults `NSTextInsertionPointBlinkPeriodOn` / `…Off` (Chromium reads them:
  https://chromium.googlesource.com/chromium/src/+/main/ui/base/cocoa/defaults_utils.mm); the
  tooltip delay is the undocumented user default `NSInitialToolTipDelay` (community sources only,
  e.g. https://discussions.apple.com/thread/2105713). Both: set only when the user wrote them; the
  default is a measurement.
- **Probe vocabulary** (`tools/os-probe/src/macos.rs`, `.github/workflows/os-probe.yml`, evidence
  under `docs/evidence/os-probe/`):
  - *probe:controls* — the existing offscreen renders (`[macos.controls]`, mini/small/regular/large,
    aqua/darkAqua, `renders/<appearance>/<kind>-regular[@2x].png`). They are drawn in a window that
    is never key (run 40348854007 README), on a transparent backdrop, so alpha outside a bezel is
    its shadow. `extraLarge` is not probed (`probe.toml:606`, run 40348854007, macos-26).
  - *probe:key-window* — the key-window capture (`[macos.key_window]`, `screencapture -x -o -l`);
    `-o` omits the window shadow.
  - *probe:passes* — the existing setting passes: `not-key`, `increase-contrast-on`,
    `reduce-transparency-off`, `tinting-off`, `accent-red`.
  - *new render* — a new control kind or state added to probe:controls; *new key-window element* —
    a control added to the key window (needed wherever the not-key render cannot show the key look).
  - *pressed render* — draw with the cell highlighted (`NSCell.isHighlighted` / `highlight(_:withFrame:in:)`)
    or post a `CGEvent` mouse-down into the key window and capture before mouse-up.
  - *hover capture* — post a `CGEvent` mouse-move into the key window, pump the run loop, capture.
  - *shadow capture* — `screencapture -l` **without** `-o` (or `CGWindowListCreateImage` without
    `kCGWindowImageBoundsIgnoreFraming`) of the window or popup over a flat backdrop window of known
    colour; fit the alpha fall-off per layer (offset, blur, spread, colour).
  - *material capture* — the surface over a known pattern window (calibration stripes) with
    Reduce Transparency off; compare to the same surface over a flat backdrop to separate blur,
    saturation and tint.
  - *CA introspection* — during a state change, read `layer.animationKeys()` and each
    `CAAnimation`'s `duration`, `beginTime`, `timingFunction` (`getControlPoint(at:values:)`) or
    `CASpringAnimation` parameters from the control's layer tree: a read of the motion, not a fit.
  - *recording* — ScreenCaptureKit / `screencapture -v` at the display's refresh rate, curve fitted
    per frame; the method for animations that CA introspection cannot reach (WindowServer-side:
    window open/close, minimise, full screen, menus, sheets).
  - **Environment caveat:** the GitHub runner images report Reduce Motion **on** and Reduce
    Transparency **on** (`probe.toml:465-466`, run 40348854007, macos-14 and macos-26). Every motion
    measurement needs a pass that turns Reduce Motion off (as `reduce-transparency-off` does for
    transparency); every material measurement needs the RT-off pass.

Row format: **ID** · element · what is missing · Gen (12–15 / 26 difference) · source · nearest model field today.

---

## 4.1 Elements the model has, with parts, states or properties it lacks

### Window and its chrome (`[window]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-W01 | Title bar | **Height** (no field). Differs by generation (the key-window capture's `titleBar_height` is 28 on 14, 32 on 26: `probe.toml:3516`, run 40348854007, macos-14 / macos-26) and by toolbar style (no toolbar, unified, unifiedCompact, expanded, preference). | differs | R: `NSWindow.frameRect(forContentRect:styleMask:)` minus the content rect for a titled window; `NSWindow.contentLayoutRect` vs `frame` per `toolbarStyle` (the probe's key window, one per style) | none (`toolbar.bar_height` is the toolbar's) |
| MAC-W02 | Traffic-light buttons | The whole part: diameter, gap between buttons, leading inset, vertical position; fills of close / minimise / zoom at rest, hover (glyphs ×, −, + / full-screen arrows appear on all three when the group is hovered), pressed, in a non-key / inactive window (grey), disabled (e.g. no zoom); glyph colour; the document-edited dot in the close button; their rim. Positions change with the toolbar style on 26. | differs | R geometry: `NSWindow.standardWindowButton(.closeButton/.miniaturizeButton/.zoomButton)?.frame` per toolbar style; M colours: samples in the key-window capture (already pictured in `renders/key_window/aqua.png`), hover capture, pressed render, not-key pass, `isDocumentEdited = true` | none |
| MAC-W03 | Title text | Alignment (centred without toolbar, leading in unified styles), subtitle (`NSWindow.subtitle`, macOS 11+) font and colour, proxy (document) icon size and gap, title truncation, inactive-window subtitle colour. | both | R: `titleBarFontOfSize:` already; subtitle font M (capture with a subtitle); P: `NSWindow.subtitle` docs | `window.title_bar_font`, `inactive_title_bar_text_color` |
| MAC-W04 | Title-bar / toolbar separator | Line colour, width, the `.shadow` variant's shadow, which style `.automatic` picks and when (shown only while content scrolls beneath — to verify). On 26.1+ the separator's role is taken by the scroll edge effect (MAC-X05). | differs | P: `NSWindow.titlebarSeparatorStyle` docs (cited in platform-facts §2.13); M: key-window capture with a scrolled table under the title bar, each style forced | `toolbar.border.*` (all ❓ in platform-facts §2.13) |
| MAC-W05 | Window corner radius | One value cannot carry it: the radius differs by window kind (title-bar only, each toolbar style, utility panel, HUD, sheet, alert, popover window) — on 26 "variable per window style" (platform-facts §1.1.3, third-party measurements only). Corner curve (continuous vs circular) not expressible (MAC-X07). | differs (12–15 one radius; 26 several) | R (26, to verify): `effectiveCornerRadii` of the window's content view / a view with a concentric `cornerConfiguration`; M: alpha mask of the corner in a key-window capture without `-o`, one window per style | `window.border.corner_radius` |
| MAC-W06 | Window outline | Two layers: the outer edge line and (dark appearance) an inner light rim; their colours, widths, opacity; key vs non-key. | differs (to verify on 26) | M: edge pixels of the key-window capture with and without `-o`, aqua and darkAqua, not-key pass | `window.border.color`, `line_width` (one line) |
| MAC-W07 | Window shadow | Every layer (offset, blur, spread, colour) for the key window and the non-key window (they differ), utility panels, HUD; the 26 shadow. | differs (to verify) | M: shadow capture of the probe window, key and not-key | `window.border.shadow_enabled`, `defaults.shadow_color` |
| MAC-W08 | Window background | One colour cannot carry a material: `windowBackgroundColor` is drawn tinted by the wallpaper and differs with Reduce Transparency (platform-facts §2.1.3: the drawn value with RT on); RT-off and tinting values missing. | differs | M: probe:passes `reduce-transparency-off`, `tinting-off` already exist; samples `windowBackground` (`probe.toml:3610`) | `window.background_color`, `defaults.background_color` |
| MAC-W09 | Window tab bar (native tabbing, "Prefer tabs") | The whole part: bar height, fill, selected / unselected tab fill, tab separators, tab title font and colour, close button on hover, "+" (new tab) button, inactive-window look, drag look. | differs (26 to verify) | M: new key-window element — two windows joined with `addTabbedWindow(_:ordered:)`, capture, hover capture | none (`tab.*` is a tab view's) |
| MAC-W10 | Title-bar accessory / unified toolbar background | The toolbar area's fill per toolbar style and its material (`.titlebar`); on 26 the title bar is transparent over content with glass item groups (MAC-TL03). | differs | P: `NSVisualEffectView.Material.titlebar`; M: material capture | `window.title_bar_background`, `toolbar.background_color` |

### Button (`[button]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-B01 | Button bezel styles | One button cannot carry AppKit's styles: push, flexiblePush, smallSquare, toolbar, accessoryBar, accessoryBarAction, badge, circular, helpButton (MAC-N10), disclosure / pushDisclosure (MAC-EX01), glass (26), the deprecated but still drawn recessed / roundRect / inline / texturedRounded / shadowlessSquare, and the borderless button (`isBordered = false`). Each has its own fill, border, radius, height, padding, states. | differs (glass 26 only) | P: `NSButton.BezelStyle` docs; M: new render per style, all sizes, aqua/darkAqua | `button.*` (one style) |
| MAC-B02 | Default (Return) button | State combination: the accent fill shows only in the key window of the active app; a non-key window draws it as a plain button. Default + pressed, default + disabled, default + hover (26). | both | M: new key-window element (a default button) + not-key pass; pressed render | `button.primary_background`, `primary_text_color` |
| MAC-B03 | Primary states | `primary_hover_background`, `primary_active_background`, `primary_disabled_*` do not exist. | both | M: pressed render, hover capture, disabled render of a default button | `button.primary_*`, `active_background` |
| MAC-B04 | Button shadow | 12–15: a soft shadow outside the bezel (platform-facts §1.1.4 NSButton row); its layers (offset, blur, colour) have no field. 26: none per platform-facts §2.3. | 12–15 only | M: the existing push-button renders' alpha outside the bezel (`renders/aqua/push_button-regular@2x.png`, run 40348854007, macos-14) | `button.border.shadow_enabled` |
| MAC-B05 | Bezel paint | 12–15: the bezel's top highlight / bevel and any vertical gradient (to verify on the render); a solid fill cannot carry it. | 12–15 only | M: existing renders, column profile | `button.background_color` |
| MAC-B06 | Destructive button | `NSButton.hasDestructiveAction` (macOS 11+) — whether and how it draws differently (red label in alerts — to verify). | both (to verify) | P: `hasDestructiveAction` docs; M: new render, an NSAlert with a destructive button | `defaults.danger_color` |
| MAC-B07 | Tinted buttons | `bezelColor` / `contentTintColor`; on 26 `tintProminence` (`NSTintProminence`) and the glass button's tint. The model has no secondary-prominence (tinted, not filled) button. | differs | P: `NSButton.bezelColor`, `NSTintProminence` docs; M: new render | `button.primary_background` |
| MAC-B08 | Border shape (26) | `NSControl.BorderShape` (automatic, capsule, rounded rectangle, circle — cases to verify): a capsule button at larger sizes; the automatic shape per control size. | 26 only | P: `NSControl.BorderShape` (page to verify); M: new render per size and shape | `button.border.corner_radius` |
| MAC-B09 | Button image | Icon size inside the button per control size, icon colour per state (template tint: rest, pressed, disabled, on accent), image positions (`imagePosition`), image-only button padding. | both | M: new render with an SF Symbol image, each state | `button.icon_text_gap` only |
| MAC-B10 | Checked toggle button (`pushOnPushOff`) | Checked + pressed, checked + disabled, checked in a non-key window. | both | M: new render (state on, highlighted, disabled), key-window element | `button.checked_*` |

### Text input (`[input]`) and text area (`[text_area]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-I01 | Field bezel styles | Square vs rounded bezel (`NSTextField.BezelStyle`), bordered-not-bezeled, the borderless label field, the field editor inside a table cell. | both | P: `NSTextField.BezelStyle` docs; M: new render per style | `input.*` (one style) |
| MAC-I02 | Read-only state | A non-editable, selectable field's look (bezel, fill, text colour) — no state field. | both (to verify whether it differs from editable) | M: new render `isEditable = false` | none |
| MAC-I03 | Secure field | Bullet glyph, the caps-lock indicator drawn inside `NSSecureTextField`. | both | M: new render; caps lock via `CGEventSourceFlagsState` cannot be faked — screen recording on a runner with a caps-lock key event (to verify feasibility) | none |
| MAC-I04 | Bezel edge paint | 12–15: the field bezel's per-side edge (top edge shading — to verify) and any inner shadow; 26: the 1-pt outer ring "of alpha ≤ 23/255" platform-facts §2.4 measured, which a border colour + width cannot state as an outer ring. | differs | M: existing `text_field-regular@2x.png` renders | `input.border.color`, `line_width` |
| MAC-I05 | Inactive text selection | `unemphasizedSelectedTextBackgroundColor` / `unemphasizedSelectedTextColor` (the selection when the field is not first responder or the window is not key) — read by the probe (`probe.toml:332`, :406) but no field. | both | R: the two NSColor class properties | `input.selection_background`, `selection_text_color` (emphasized only) |
| MAC-I06 | Completion / suggestion UI | The completions list (`NSTextView.complete(_:)`) and the macOS 14 inline prediction ghost text (its colour). | both (inline predictions 14+) | P: `NSTextInputTraits` inline prediction property (to verify name); M: recording in a text view | none |
| MAC-I07 | Text area extras | Ruler (`NSRulerView`) and the find bar are not the field's (MAC-N17, MAC-N18); the text area's default font is `userFont(ofSize:)` (probe `[macos.fonts.named.userFontOfSize]`, `probe.toml:139`), not the system font — no field. | both | R: `NSFont.userFont(ofSize: 0)`, `userFixedPitchFont(ofSize: 0)` | `input.font`, `defaults.mono_font` |
| MAC-I08 | Text area frame | The enclosing `NSScrollView.borderType` frame (none / line / bezel / groove) and its colour. | both | P: `NSScrollView.borderType` docs; M: new render per border type | `text_area.border` (inherits input) |

### Checkbox / radio (`[checkbox]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-C01 | Mixed state | Indeterminate box: the dash glyph, its size and colour, the box fill (accent?). | both | P: `NSControl.StateValue.mixed`; M: new render (`allowsMixedState`, state mixed) | none |
| MAC-C02 | Pressed state | Unchecked + pressed, checked + pressed, radio pressed. | both | M: pressed render | none |
| MAC-C03 | Checked + disabled | No field (platform-facts §2.5 `disabled_background` is ❓ for the checked box); the disabled check-mark colour. | both | M: the existing `checkbox-regular-disabled-on@2x.png` renders (not-key) + key-window element | `checkbox.disabled_background` (unchecked) |
| MAC-C04 | Checked + hover | Field absent. | 26 (12–15 draws no hover — to verify) | M: hover capture | `checkbox.hover_background` (one) |
| MAC-C05 | Box paint | 12–15 dark: the unchecked box is a vertical gradient (platform-facts §2.5 `background_color`); 12–15 light: an outer edge darker at the top and a lighter inner bevel (§2.5 `border.color`). A solid fill + one border colour cannot carry either. | 12–15 only | M: existing renders | `checkbox.background_color`, `border.color` |
| MAC-C06 | Check-mark geometry | Glyph extent / inset inside the box and its path (stroke width exists). | differs | M: existing `checkbox-regular-on@2x.png` renders | `checkbox.check_mark_stroke_width` |
| MAC-C07 | Radio specifics | Radio pressed, radio disabled-on dot colour, radio non-key look; all shared with the checkbox. | both | M: existing radio renders + pressed render | `checkbox.*`, `radio_*` |

### Menu (`[menu]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-MN01 | Highlight vs container | One `border` cannot be both the menu's container (radius, edge) and the item highlight: macOS 11+ draws the hover highlight as a rounded rect inset from the menu's sides (platform-facts §2.6 says "items are rectangular" — to verify). Missing: highlight corner radius, highlight horizontal inset. | differs (26 larger, concentric) | M: new menu capture (pop a menu with `NSMenu.popUp(positioning:at:in:)` from a timer, capture its window via `CGWindowListCreateImage`), hover capture | `menu.border.*`, `popover.border.corner_radius` |
| MAC-MN02 | Menu padding | Content inset above the first and below the last item. | differs | M: menu capture | none |
| MAC-MN03 | Separator item | Separator row height, line inset left / right, line width. | differs | M: menu capture | `menu.separator_color` |
| MAC-MN04 | Key-equivalent column | Shortcut text font and colour at rest / highlighted / disabled, modifier-glyph spacing, alignment, gap to the title. | both | M: menu capture with key equivalents | none |
| MAC-MN05 | State column | On (check), mixed (dash) and off images, their colour per state, the state column's width / title indent (`showsStateColumn`). | both | P: `NSMenu.showsStateColumn`, `NSMenuItem.state`; M: menu capture | none |
| MAC-MN06 | Submenu arrow | Glyph, size, colour per state; the parent item's look while its submenu is open (stays highlighted). | both | M: menu capture with an open submenu | none |
| MAC-MN07 | Section header (14+) | `NSMenuItem.sectionHeader(title:)` font, colour, height. | 14+ and 26 (not 12–13) | P: docs; M: menu capture | none |
| MAC-MN08 | Badges (14+) | `NSMenuItemBadge` (alerts / new items / updates): font, colour, fill. | 14+ and 26 | P: `NSMenuItemBadge` docs; M: menu capture | none |
| MAC-MN09 | Subtitle | `NSMenuItem.subtitle` font and colour. | 14+ (version to verify) and 26 | P: docs; M: menu capture | none |
| MAC-MN10 | Item icon colour | Icon tint at rest / highlighted / disabled; 27 hides item images by default (platform-facts §2.6). | differs | M: menu capture with SF Symbol images | `menu.icon_size` |
| MAC-MN11 | Indentation | `NSMenuItem.indentationLevel` step width. | both | M: menu capture | none |
| MAC-MN12 | Scroll arrows | The up / down scroll areas of a menu taller than the screen. | both | M: menu capture with many items | none |
| MAC-MN13 | Palette menu (14+) | `NSMenu` palette presentation: swatch items, selection ring. | 14+ and 26 | P: `NSMenu.presentationStyle` docs (to verify name); M: capture | none |
| MAC-MN14 | Menu edge and shadow | Container edge line / rim and every shadow layer (MAC-X01); the menu material (MAC-X02). | differs | M: shadow capture of a menu window | `menu.border.*` ("from §2.16") |
| MAC-MN15 | Pop-up menu geometry | A pop-up button's menu opens with the selected item over the button (offset), its font follows the control size. | both | M: capture with a pop-up button's menu open | none |
| MAC-MN16 | Minimum width | The menu's minimum width (`NSMenu.minimumWidth` default). | both | R: `NSMenu().minimumWidth` (to verify default meaning); M | none |

### Tooltip (`[tooltip]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-T01 | Tooltip shadow | Layers (MAC-X01). | differs (to verify) | M: hover capture without `-o` | `tooltip.border.shadow_enabled` |
| MAC-T02 | Tooltip material | The `.toolTip` material — a solid colour cannot carry it. | differs | P: `NSVisualEffectView.Material.toolTip`; M: material capture | `tooltip.background_color` |
| MAC-T03 | Tooltip position | Offset from the pointer / below the element. | both | M: hover capture | none |
| MAC-T04 | Tooltip timings | Re-show delay when moving between items (instant after one is shown — to verify), auto-hide, fade-in/out (MAC-X11). | both | M: recording with timed hovers | `tooltip.show_delay_ms` |
| MAC-T05 | Show delay source | Field exists but its source: R the `NSInitialToolTipDelay` user default when set (undocumented), M the default. | both | R / M | `tooltip.show_delay_ms` |

### Scrollbar (`[scrollbar]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-S01 | Overlay knob hover width | Overlay knob expands on hover (platform-facts §2.8 states idle and hover widths in one row); one `thumb_width` cannot hold both. | differs (26 ❓) | M: new render of an `NSScroller` (overlay) + hover capture | `scrollbar.thumb_width` |
| MAC-S02 | Overlay hover track | The track that appears under an expanded overlay knob: fill and its edge line. | both | M: hover capture | `scrollbar.track_color` (mode-dependent, one value) |
| MAC-S03 | Knob style | `NSScroller.KnobStyle` default / dark / light: the knob's contrasting border colour and width. | both | P: `scrollerKnobStyle` docs; M: new render per style | none |
| MAC-S04 | Knob shape and inset | Knob corner radius (capsule), inset from the track edges across and along the axis. | differs | M: new scroller render | none |
| MAC-S05 | Legacy track edge | The line between content and a legacy track. | both | M: new render, legacy style | none |
| MAC-S06 | Scroll corner | The square where two legacy scrollers meet. | both | M: new render | none |
| MAC-S07 | Scroller per control size | Widths per control size are probed (`probe.toml:472-486`) but the model holds one. | differs | R: `NSScroller.scrollerWidth(for:scrollerStyle:)` | `scrollbar.groove_width` (MAC-X17) |
| MAC-S08 | Pressed knob | Field exists, ❓ in platform-facts. | both | M: pressed capture on a scroller | `scrollbar.thumb_active_color` |

### Slider (`[slider]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-SL01 | Knob shape | 26: a capsule wider than tall (platform-facts §2.9 "one diameter cannot carry it"); missing knob width and height. | 26 only | R: `NSSliderCell.knobRect(flipped:)` (probed) | `slider.thumb_diameter` |
| MAC-SL02 | Knob shadow and rim | 12–15: soft shadow outside the knob (§1.1.4); layers and the knob's rim. | differs | M: existing slider renders' alpha | none |
| MAC-SL03 | Track / fill radius | Corner radius of the track and of the fill's end. | differs | M: existing renders | none |
| MAC-SL04 | Tick marks | Colour, width, shape (line on 12–15, dot on 26 — §2.9), position (`tickMarkPosition` above / below / leading / trailing), gap from track. | differs | R: `rectOfTickMark(at:)` (probed); M: existing `slider_ticks` renders | `slider.tick_mark_length` |
| MAC-SL05 | Knob with ticks | Whether the knob changes shape with tick marks on 12–15 (to verify on `slider_ticks-regular@2x.png`). | 12–15 (to verify) | M: existing render | none |
| MAC-SL06 | Fill vs tick marks (26) | `tintProminence`: "a regular slider tints its track fill, while a slider with tick marks is untinted" — the fill colour depends on tick marks. `neutralValue`: the fill's origin. | 26 only | P: `NSSlider.tintProminence`, `neutralValue` docs | `slider.fill_color` |
| MAC-SL07 | Pressed / dragged knob | Knob while dragged (26: to verify a glass-lens knob); no active state. | differs | M: pressed render, recording | `slider.thumb_hover_color` |
| MAC-SL08 | Circular slider | → MAC-N15. | | | |

### Progress bar (`[progress_bar]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-P01 | Indeterminate bar | Its look (moving segment / stripes) and its animation period and curve. | differs (to verify) | M: new render `isIndeterminate = true` + recording / CA introspection | none |
| MAC-P02 | Radii | Track radius vs fill end radius (border.corner_radius inherits the control radius). | differs | M: existing renders | `progress_bar.border.corner_radius` |
| MAC-P03 | Non-key fill | Fill draws grey in a non-key window (probe README, run 40348854007) — MAC-X16. | both | M: key-window element | `progress_bar.fill_color` |
| MAC-P04 | Value change motion | Whether the fill animates to a new value. | both (to verify) | M: CA introspection | none |

### Tab view (`[tab]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-TB01 | Tab view content box | The bezel box under the tabs: fill, border, radius, padding (`contentRect` vs frame). | both | R: `NSTabView.contentRect` inset; M: new render | none |
| MAC-TB02 | Tab strip geometry | Tabs drawn as a segmented strip centred over and straddling the box's top edge (macOS 11+): the overlap offset. | differs (to verify on 26) | M: new render | `tab.*` |
| MAC-TB03 | Tab variants | `tabPosition` (top / bottom / left / right / none), `tabViewBorderType` (none / line / bezel), `tabViewType`. | both | P: `NSTabView` docs; M: new render per variant | `tab.active_indicator_side` (Top / Bottom only) |
| MAC-TB04 | Tab states | Pressed tab, selected tab in a non-key window. | both | M: pressed render, not-key pass | none |
| MAC-TB05 | Tab metrics per control size | `NSTabView.controlSize`, `font` per size. | both | M: new render per size | MAC-X17 |

### Sidebar (`[sidebar]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-SB01 | Row height and icon size per sidebar size | The user's "Sidebar icon size" (small / medium / large) sets row height and icon size (`NSTableView.RowSizeStyle.default`). Missing: row height per size, icon size per size, font per size (to verify). | both | R: `effectiveRowSizeStyle` and `rowHeight` of a source-list table; the user default `NSTableViewDefaultSizeMode` (to verify); P: platform-facts §1.1.6 sidebar rows (12–15, archived HIG) | `defaults.icon_sizes.small_px`, `list.row_height` |
| MAC-SB02 | Sidebar icon colour | SF Symbol icons tinted (accent) at rest, white on an emphasized selection, unemphasized-selection colour. | both | M: key-window sidebar with rows (the probe's sidebar holds no rows, `probe.toml:3247`, run 40320895749) | none |
| MAC-SB03 | Section header (group row) | Font, colour, height, the hover "Show / Hide" / chevron control. | differs | M: key-window source list with group rows, hover capture | none |
| MAC-SB04 | Selection shape | Selected row corner radius and horizontal inset; merged contiguous selection (per-corner, MAC-X07). | differs | M: key-window source list with a selection | `sidebar.selection_background` |
| MAC-SB05 | Unemphasized selection | Sidebar selection when the outline is not first responder / window not key: fill and text colour — no sidebar field (only `defaults.selection_inactive_background`). | both | R: `unemphasizedSelectedContentBackgroundColor`; M: not-key pass | `defaults.selection_inactive_background` |
| MAC-SB06 | Floating glass sidebar (26) | The inset sidebar panel: margins from the window edges, corner radius (concentric with the window), glass material, shadow, content extending under it (`NSBackgroundExtensionView`). | 26 only | P: `NSBackgroundExtensionView`, `NSView.cornerConfiguration` docs; R: `effectiveCornerRadii`; M: key-window capture with an `NSSplitViewController` sidebar item | `sidebar.background_color`, `sidebar.border.*` |
| MAC-SB07 | Sidebar active / inactive | `NSVisualEffectView.State` followsWindowActiveState: the material changes in a non-key window. | both | M: not-key pass sample `sidebar` (exists: `probe.toml:3595`) | `sidebar.background_color` |
| MAC-SB08 | Nested rows | Disclosure triangles and indentation in a source list (MAC-L10). | both | R: `indentationPerLevel` | none |

### Toolbar (`[toolbar]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-TL01 | Height per toolbar style | unified / unifiedCompact / expanded / preference each have their own height and margins ("reduced margins" for unifiedCompact, docs). | differs | R: `contentLayoutRect` vs frame per style (probe key window) | `toolbar.bar_height`, `border.padding_*` |
| MAC-TL02 | Toolbar item button | Rest / hover / pressed bezel (`isBordered`), icon colour per state, label (icon-and-label display mode) font, colour and gap below the icon, disabled item. | differs | M: key-window toolbar, hover capture, pressed render | `toolbar.font`, `icon_size` |
| MAC-TL03 | Item groups / glass capsules | `NSToolbarItemGroup` segmented look; 26: items grouped in glass capsules, their padding, spacing between capsules, capsule height. | differs (capsules 26 only) | P: `NSGlassEffectContainerView.spacing`; M: key-window toolbar | `toolbar.item_gap` |
| MAC-TL04 | Prominent item (26) | `NSToolbarItem.style = .prominent` ("tints the background"), `backgroundTintColor`. | 26 only | P: docs; M | none |
| MAC-TL05 | Toolbar badges | `NSToolbarItem.badge` / `NSItemBadge`: shape, fill, font. | version to verify (26) | P: docs; M | none |
| MAC-TL06 | Overflow | The overflow chevron button and its menu. | both | M: narrow key window | none |
| MAC-TL07 | Search toolbar item | `NSSearchToolbarItem` collapsed (button) vs expanded field, and its expand motion. | both | M: capture + recording | none |
| MAC-TL08 | Tracking separator | `NSTrackingSeparatorToolbarItem` line aligned with the split view divider. | 12–15 (26 to verify) | M | none |
| MAC-TL09 | Toolbar background | 12–15: title-bar material; 26: transparent over content with the scroll edge effect (MAC-X05). | differs | M: material capture | `toolbar.background_color` |

### Status bar (`[status_bar]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-ST01 | Window bottom bar | macOS's in-window counterpart is the content border (`NSWindow.setContentBorderThickness(_:for: .minY)`): its height, fill, top separator, inactive look. The model's status bar has no height. | differs (to verify on 26) | P: `setContentBorderThickness` docs; M: key-window element | `status_bar.*` (no height) |

### List / table / outline (`[list]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-L01 | Table styles | `NSTableView.Style` automatic / fullWidth / inset / sourceList / plain: horizontal inset per style (the header's column rect starts at x 10 in inset and 0 in fullWidth, `probe.toml:510`, :538, run 40348854007 macos-26), row radius per style. | both | R: `headerRect(ofColumn:)` / `rect(ofRow:)` per style; M: key-window table per style | `list.border.padding_*` |
| MAC-L02 | Selection shape | Selected row corner radius and inset (inset style), merged contiguous rows (per-corner radii, MAC-X07). | both | M: key-window table with a 2-row selection | `list.selection_background` |
| MAC-L03 | Unemphasized selection | List lacks `selection_inactive_background` and an inactive selection text colour (unemphasized rows keep the label colour — to verify). | both | R: `unemphasizedSelectedContentBackgroundColor`; M: not-key pass | `defaults.selection_inactive_background` |
| MAC-L04 | Secondary text on selection | `NSTableCellView.backgroundStyle` `.emphasized` turns secondary labels and icons white-ish: no field. | both | M: key-window table with a secondary label | `list.selection_text_color` |
| MAC-L05 | Header parts | Vertical separators between header cells (visible in `renders/key_window/aqua.png`), the header's bottom line, the sort-indicator glyph (size, colour), header pressed state, header text alignment and padding. | differs | M: key-window table (existing) — add sort descriptor, pressed render | `list.header_*`, `grid_color` |
| MAC-L06 | Grid style | `gridStyleMask` solid vertical / solid horizontal / dashed horizontal: dash pattern and which lines. | both | P: docs; M: new render | `list.grid_color` |
| MAC-L07 | Intercell spacing | `intercellSpacing` (probed `[17, 0]`, `probe.toml:499`) — no field. | both | R: `NSTableView.intercellSpacing` | none |
| MAC-L08 | Row height per row-size style | small / medium / large (MAC-SB01); one `row_height`. | both | R: `rowHeight` after setting `rowSizeStyle` | `list.row_height` |
| MAC-L09 | Group rows | Floating group rows: font, background (`.headerView` material), height. | both | M: key-window table with group rows | none |
| MAC-L10 | Outline parts | Indentation per level (`indentationPerLevel`, probed 13, `probe.toml:591`; platform-facts §2.15 calls it toolkit-internal but it is drawn), disclosure triangle glyph size and colour, its pressed look, its rotation (MAC-X11). | differs (to verify on 26) | R: `NSOutlineView.indentationPerLevel`; M: key-window outline | none |
| MAC-L11 | Drop target | Drop-on row highlight (outlined rounded rect) and drop-above insertion line with its end circle: colour, width, radius. | both | M: `setDropRow(_:dropOperation:)` in the key window, capture | none |
| MAC-L12 | Row actions | `NSTableViewRowAction` (regular / destructive) swipe buttons: fills, font. | both | P: docs; M: recording | none |
| MAC-L13 | Cell editing | The field editor in a cell (fill, focus ring). | both | M: `editColumn(_:row:with:select:)` in the key window | `input.*` |
| MAC-L14 | Table focus | Whether a first-responder table shows a focus ring (`focusRingType`) — MAC-X13. | both | M | `defaults.focus_ring_*` |
| MAC-L15 | Column drag | Dragged column's translucent image. | both | M: recording | none |
| MAC-L16 | Table frame | `NSScrollView.borderType` around the table (platform-facts §2.15 ❓). | both | P/M as MAC-I08 | `list.border.*` |

### Popover (`[popover]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-PO01 | Arrow (anchor) | Width, height, tip shape, offset from the anchor; `hasFullSizeContent` (14+) content under the arrow. | differs | M: new capture of an `NSPopover` shown from the key window | none |
| MAC-PO02 | Material | `.popover` material (MAC-X02). | differs | M: material capture | `popover.background_color` |
| MAC-PO03 | Shadow | Layers (MAC-X01). | differs | M: shadow capture | `popover.border.shadow_enabled` |
| MAC-PO04 | Detached popover | The window a detached popover becomes. | both | M | none |
| MAC-PO05 | Motion | Show / close animation (`animates`) — MAC-X11. | both | M: recording | none |

### Splitter (`[splitter]`) and separator (`[separator]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-SP01 | Divider styles | thin / thick / paneSplitter: three widths (probed: `probe.toml:492-495`) and drawings; one width / colour. | both | R: `dividerThickness` per style | `splitter.divider_width`, `divider_color` |
| MAC-SP02 | Thick divider grip | Whether a grip (dimple) is drawn (to verify). | both | M: new render of each style | none |
| MAC-SP03 | Dragging state | Divider while dragged. | both | M: pressed capture | `splitter.hover_color` |
| MAC-SP04 | Sidebar split (26) | No divider between a floating sidebar and content. | 26 only | M: key-window split view | `splitter.*` |
| MAC-SE01 | Separator | No gap found beyond the per-side / inset rows (MAC-X08); `NSBox` separator type is covered by `[separator]`. | — | — | — |

### Switch (`[switch]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-SW01 | Knob shape | 26: a capsule (platform-facts §2.21: "one diameter cannot carry it"); missing knob width and height. | 26 only | M: existing renders | `switch.thumb_diameter` |
| MAC-SW02 | Knob shadow | Layers of the knob's shadow, separated from its rim (platform-facts §2.21 notes the render does not separate them). | differs | M: shadow fit on existing renders | `switch.thumb_border_*` |
| MAC-SW03 | Pressed | Knob / track while pressed (knob stretch — to verify). | both | M: pressed render, recording | none |
| MAC-SW04 | Non-key on track | Grey on track in a non-key window (probe README) — MAC-X16. | both | M: not-key pass (existing) | `switch.checked_background` |
| MAC-SW05 | Size variants | NSSwitch mini / small / regular / large (probed per size). | both | R: `intrinsicContentSize` per size | MAC-X17 |
| MAC-SW06 | Differentiate without colour | Whether the switch adds on/off glyphs (to verify). | to verify | M: pass with the setting on | none |

### Dialog (`[dialog]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-D01 | Alert layout | `NSAlert` (macOS 11+): centred title and informative text, fixed width, buttons full-width / stacked vs side by side, the suppression checkbox, help button, accessory view. Missing: text alignment, button arrangement rule, alert width, informative-text font (distinct from body — to verify). | differs | R: `NSAlert.layout()` then the window's frame and subviews' frames (no modal run needed); P: HIG Alerts; M: capture of `alert.window` | `dialog.*` |
| MAC-D02 | Critical alert | `NSAlert.Style.critical`: caution badge over the app icon. | both | M: capture | `dialog.icon_size` |
| MAC-D03 | Sheet | Sheet corner radius, material (`.sheet`), shadow, whether the parent dims; 26: floating inset sheet. | differs | M: new capture of a sheet on the key window | `dialog.border.*` |
| MAC-D04 | Motion | `entrance` is an enum without duration / curve; exit and sheet attach / detach motion missing (MAC-X11). | both | M: recording | `dialog.entrance` |

### Spinner (`[spinner]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-SN01 | Fin geometry | Fin count, inner / outer radius, fin width and end rounding, the per-fin alpha ramp (platform-facts §2.23 states one leading alpha). | both (renders byte-identical, §2.23) | M: existing `progress_spinning` renders | `spinner.fill_color`, `stroke_width` |
| MAC-SN02 | Rotation | Stepped rotation period (MAC-X11). | both | M: CA introspection / recording | none |
| MAC-SN03 | Small size | Regular and mini map to `diameter` / `min_diameter`; the small size has no field. | both | R: `sizeToFit` per size (probed) | `spinner.diameter` |
| MAC-SN04 | Determinate spinning | The determinate circular (pie / ring) indicator. | both | M: new render `style spinning`, `isIndeterminate = false` | none |

### Combo box / pop-up (`[combo_box]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-CB01 | Three controls in one section | Pop-up button (double chevron), pull-down (`pullsDown`, single down chevron), `NSComboBox` (editable field + separate arrow button, drawn in an inset rounded box — visible in the existing `combo_box-regular@2x.png` renders). Missing: arrow glyph kind, the combo arrow button's fill / border / radius, the combo field's input look. | differs | M: existing renders (pop-up, combo); new render of a pull-down | `combo_box.*` |
| MAC-CB02 | Arrow area in key window | Whether the pop-up's arrow area is accent-filled in a key window (the not-key render cannot show it — to verify). | 12–15 (to verify) | M: new key-window element | `combo_box.arrow_area_width` |
| MAC-CB03 | Arrow colour | Per state. | both | M | none |
| MAC-CB04 | Open / pressed | The control while its menu is open; `NSComboBox` drop-down list (row height, `numberOfVisibleItems`, scroller, highlight). | both | M: capture with the list open | `popover.*` |
| MAC-CB05 | Borderless pop-up | `isBordered = false` (arrows only, used in toolbars / cells). | both | M: new render | none |
| MAC-CB06 | 26 rest look | The 26 not-key render draws no bezel fill around the pop-up (existing render) — key vs not-key needs the key-window element. | 26 | M | `combo_box.background_color` |

### Segmented control (`[segmented_control]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-SG01 | Separator colour | Line between unselected segments. | differs | M: existing renders | `separator_width` |
| MAC-SG02 | Selected segment shadow | 12–15: the raised selected segment's shadow (key-window capture, macOS 14). | 12–15 only | M: key-window capture without `-o` | none |
| MAC-SG03 | Selected label weight (26) | 26 draws the selected segment accent-filled with a heavier white label (`renders/key_window/aqua.png`, run 40348854007, macos-26); 12–15 neutral raised. `active_text_color` exists, active font weight does not. | differs | M: existing key-window captures | `active_background`, `active_text_color` |
| MAC-SG04 | Styles | automatic, rounded, roundRect, texturedRounded, capsule, texturedSquare, smallSquare, separated (platform-facts §2.25) and the 26 `borderShape`. | differs | P: `NSSegmentedControl.Style`; M: new render per style | one style |
| MAC-SG05 | Tracking modes | selectAny (several selected), momentary (pressed only); `selectedSegmentBezelColor` ("in appearances that support it"). | both | R: `selectedSegmentBezelColor` default; M: new render per mode | `active_background` |
| MAC-SG06 | States | Pressed segment, selected + pressed, disabled segment (`setEnabled(_:forSegment:)`), selected in a non-key window (26 accent → grey?). | both | M: pressed render, not-key pass | `hover_background`, `disabled_opacity` |
| MAC-SG07 | Menu indicator | `showsMenuIndicator(forSegment:)` glyph. | both | M: new render | none |
| MAC-SG08 | Per-corner radii | Only the outer corners of the end segments are round; the selected indicator's own inset radius (MAC-X07). | both | M: existing renders | `border.corner_radius` |
| MAC-SG09 | Selection motion (26) | Sliding selection indicator (MAC-X11). | 26 (to verify) | M: CA introspection | none |

### Card / box (`[card]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-BX01 | Box title | `NSBox.titleFont`, `titlePosition` (noTitle, aboveTop, atTop, belowTop, aboveBottom, atBottom, belowBottom), title colour, title-to-border gap (`titleRect`). | both | R: `NSBox().titleFont`, `titleRect` | none |
| MAC-BX02 | Box types | primary vs custom vs separator, `isTransparent`; the primary box's look is ❓ (fields exist) — needs a render. | both | M: new NSBox render | `card.*` |

### Expander (`[expander]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-EX01 | Two disclosure controls | `.disclosure` (triangle) vs `.pushDisclosure` (a push button with a chevron). | both | P: `NSButton.BezelStyle`; M: new renders | `expander.*` |
| MAC-EX02 | Glyph shape | Triangle vs chevron; the model states no shape (seed: `docs/todo.md:729`). | differs (to verify on 26) | M: new render | `arrow_icon_size`, `arrow_side` |
| MAC-EX03 | Pressed glyph | Highlighted triangle colour. | both | M: pressed render | `arrow_color` |
| MAC-EX04 | Rotation motion | MAC-X11. | both | M | none |

### Link (`[link]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-LK01 | Underline | Underline style (single / thick / pattern), colour, thickness, offset. | both | R: `NSTextView().linkTextAttributes` (the default attributes) | `link.underline_enabled` |
| MAC-LK02 | Cursor | Pointing hand (MAC-X20). | both | R: `linkTextAttributes[.cursor]` | none |
| MAC-LK03 | Focus | Focus ring on a link with Full Keyboard Access (MAC-X13). | both | M | `defaults.focus_ring_*` |

### Defaults, text scale, layout, icons

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-DF01 | Label hierarchy | `tertiaryLabelColor`, `quaternaryLabelColor`, `quinaryLabelColor` (14+; probed `probe.toml:312`) — only `muted_color` (secondary). | both (quinary 14+) | R: NSColor | `defaults.muted_color` |
| MAC-DF02 | Fill ramp | `systemFillColor`, `secondary…`, `tertiary…`, `quaternary…`, `quinarySystemFillColor` (14+; probed `probe.toml:357`); platform-facts §2.3 finds the 26 button drawn in `secondarySystemFillColor`'s value. | 14+ and 26 | R: NSColor | none |
| MAC-DF03 | System palette | `systemYellow`, `Purple`, `Pink`, `Teal`, `Indigo`, `Cyan`, `Mint`, `Brown`, `Gray` (platform-facts §1.1.2 lists them as sourced; the model carries only red / orange / green / blue as status). | both | R: NSColor | `defaults.*_color` (status) |
| MAC-DF04 | Other drawn colours | `underPageBackgroundColor` (area under document pages), `findHighlightColor`, `selectedControlColor`, `secondarySelectedControlColor`, `controlBackgroundColor` vs `textBackgroundColor` distinctions already partly mapped. | both | R: NSColor | `surface_color` etc. |
| MAC-TS01 | Text-scale roles | macOS has 11 text styles (§1.1.1); the model carries 4 roles. Missing: title2, title3, subheadline, callout, footnote, caption2 (body = `defaults.font`, headline = `section_heading`), and each style's emphasized weight (HIG "emphasized" column, §1.1.1). | both | R: `NSFont.preferredFont(forTextStyle:)` (probed); P: HIG typography JSON | `[text_scale]` |
| MAC-TS02 | Named fonts | `labelFontOfSize`, `messageFontOfSize`, `paletteFontOfSize`, `smallSystemFontSize`, `controlContentFontOfSize`, `menuBarFontOfSize` (all probed, `[macos.fonts.named]`) have no fields. | both | R: NSFont | `defaults.font` |
| MAC-LY01 | Layout spacing | Label-to-control gap per control size and inter-button spacing per size (platform-facts §1.1.5: HIG 8/6/5, 12 for push buttons — legacy HIG); 26 window content margins concentric with the window. | differs | P: legacy HIG (archived), current HIG Layout; M | `[layout]` (4 fields) |
| MAC-IC01 | Icon contexts | Button icon, list-row icon, tab icon, toolbar icon per toolbar size mode, status-item icon, sidebar icon per sidebar size — `icon_sizes` has 5 contexts. | both | P: platform-facts §1.1.6; M | `defaults.icon_sizes` |
| MAC-IC02 | SF Symbol configuration | Weight and scale (small / medium / large) per context so a symbol matches adjacent text; rendering mode (monochrome / hierarchical / palette / multicolor). | both | P: HIG SF Symbols; R: `NSImage.SymbolConfiguration` | none |
| MAC-IC03 | Icon appearance (26) | Icon & widget style (Default / Dark / Clear / Tinted) changes the app and folder icons an app shows. | 26 only | R: user default (key to verify) | none |

---

## 4.2 Standard controls the model does not have

| ID | Control | What it draws (all missing) | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-N01 | `NSStepper` | Up / down halves, arrow glyphs, separator, pressed half, disabled half at a limit, size variants, field + stepper pairing gap, autorepeat timing. | differs (existing renders: 26 draws separate rounded halves) | R: metrics already probed (`[macos.controls.*.stepper]`, `probe.toml:1768`); M: existing `stepper-regular@2x.png` renders + pressed render | none (`button`) |
| MAC-N02 | `NSSearchField` | Bezel (capsule on 26 per the existing render), magnifier glyph, cancel button, recents-menu indicator, centred placeholder and its move to leading on focus, focus ring shape; toolbar search item (MAC-TL07). | differs | R: `searchButtonBounds`, `cancelButtonBounds`, `searchTextBounds`; M: existing `search_field` renders, recording for the placeholder motion | `input.*` |
| MAC-N03 | `NSTokenField` | Token fill, border, radius, text, selected token (accent fill), token styles (default / rounded / squared / plainSquared), token menu arrow, spacing. | both | P: `NSTokenField.TokenStyle`; M: new render | none |
| MAC-N04 | `NSDatePicker` | textField / textFieldAndStepper (selected element highlight), clockAndCalendar (month header, navigation arrows, weekday header font, day cells, today marker, selected day, out-of-month days, clock face and hands), the calendar overlay (`presentsCalendarOverlay`). | differs (to verify) | P: `NSDatePicker.Style`; M: new render per style | none |
| MAC-N05 | `NSColorWell` | Styles default / minimal / expanded (macOS 13): swatch frame, active ring, minimal's chevron, expanded's panel button, HDR well (`maximumLinearExposure`). | 13+ styles; differs | P: `NSColorWell.Style`; M: new render per style | none |
| MAC-N06 | `NSLevelIndicator` | continuousCapacity, discreteCapacity (segments, tiered), relevancy bars, rating stars and placeholders; fill / warning / critical colours; tick marks. | both | R: a new instance's `fillColor`, `warningFillColor`, `criticalFillColor`; P: docs; M: new render per style | `progress_bar.fill_color` |
| MAC-N07 | `NSPathControl` | standard / popUp: component chevron separators, component icons, truncation, hover highlight of a component, placeholder. | both | P: `NSPathControl.Style`; M: new render | none |
| MAC-N08 | `NSBrowser` | Columns, column resize handles, branch arrows, title row, selection (emphasized / unemphasized). | both | M: new render | `list.*` |
| MAC-N09 | `NSRuleEditor` / `NSPredicateEditor` | Rows, round +/− buttons, row stripes, nesting indent. | both | M: new render | none |
| MAC-N10 | Help button | `.helpButton`: circle, "?" glyph, sizes, pressed. | differs | P: `NSButton.BezelStyle.helpButton`; M: new render | none |
| MAC-N11 | Badge / circular / inline buttons | `.badge` ("additional information"), `.circular`, deprecated `.inline` count badges in source lists; SwiftUI `.badge()` on list rows. | both | P: bezel-style docs; M: new render | none |
| MAC-N12 | Image view frames | `NSImageView.imageFrameStyle` photo / grayBezel / groove / button. | both | M: new render | none |
| MAC-N13 | Collection view items | `NSCollectionView` item selection highlight (shape, radius, emphasized / unemphasized). | both | M: new key-window element | `list.selection_*` |
| MAC-N14 | HUD panel | `NSPanel` `.hudWindow`: dark translucent material (`.hudWindow`), its title bar and close button, controls drawn in it. | differs | P: material docs; M: new window capture | none |
| MAC-N15 | Circular slider | `sliderType = .circular`: dial, knob dot, ticks. | both | M: new render | `slider.*` |
| MAC-N16 | Utility / floating panel | `.utilityWindow` panels: shorter title bar, smaller traffic lights, title font. | differs | R: frame vs content rect; M: new window capture | `window.*` |
| MAC-N17 | Text ruler | `NSRulerView`: ticks, labels, markers. | both | M: new render | none |
| MAC-N18 | Find bar | `NSTextFinder` incremental find bar in an `NSScrollView` (`findBarPosition`): field, buttons, Done. | both | M: new key-window element | none |
| MAC-N19 | Inspector pane | Trailing inspector split item (14+): background, divider; 26 floating glass inspector. | differs | M: key-window split view with an inspector item | `sidebar.*` |
| MAC-N20 | Menu bar and status items | Bar height, font (`menuBarFontOfSize:`), bold app name, open-menu title highlight, background (wallpaper-tinted translucent; 26 transparent), status-item icon size and spacing. Seeds: `docs/todo.md:1178`, `:3608`. | differs | R: `NSStatusBar.system.thickness`, `NSMenu.menuBarHeight` (0 in the probe without a menu bar, `probe.toml:597` — needs a regular app with a main menu); M: capture of the menu bar | none |
| MAC-N21 | SwiftUI grouped form | `.formStyle(.grouped)` (System Settings look, 13+): inset rounded section cards, row separators, row height, section header / footer fonts. | 13+; differs | P: SwiftUI `GroupedFormStyle` docs; M: needs a SwiftUI host (Swift, not the objc2 probe — to verify feasibility) | `card.*`, `list.*` |
| MAC-N22 | SwiftUI `Gauge` | Linear and circular (accessory) gauges, 13+. | 13+ | P: docs; M as MAC-N21 | none |
| MAC-N23 | SwiftUI tab bar (15+) | `TabView` sidebar-adaptable / toolbar tab bar. | 15+ and 26 | P: docs; M as MAC-N21 | `tab.*` |
| MAC-N24 | Pull-to-refresh (27) | `NSScrollView.refreshController` (27.0): refresh indicator at the scroll top. | 27 only | P: docs; M | `spinner.*` |
| MAC-N25 | System-drawn surfaces | Notification banners and Notification Center, Dock tile badge (`NSDockTile.badgeLabel`), share picker, the standard panels (open / save, colour, font, print). Drawn by the OS or AppKit, not by the app's toolkit; counted here because they are visible platform UI, and the rationale decides whether the model covers them. | differs | M: capture | none |
| MAC-N26 | Touch Bar | `NSTouchBar` items, `NSScrubber`: hardware-specific (Touch Bar Macs). | 12–15; 26 on supported Intel Macs | M: Touch Bar simulator (to verify) | none |

---

## 4.3 Properties that cross widgets

| ID | Property | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-X01 | Shadows / elevation | Per surface and state, every layer's offset, blur, spread, colour, inset or not: window key / non-key, utility panel, HUD, sheet, alert, menu, popover, tooltip, drag image; 12–15 controls (push button, pop-up, selected segment, slider knob, switch knob); 26 glass shadows. | differs | M: shadow capture per surface; existing control renders' alpha for control shadows; P (26, WebKit's own vector controls): `RenderThemeCocoa.mm` as platform-facts cites it | `defaults.shadow_color`, `border.shadow_enabled` |
| MAC-X02 | Materials / vibrancy | Each surface is a material, not a colour: titlebar, selection, menu, popover, sidebar, headerView, sheet, windowBackground, hudWindow, fullScreenUI, toolTip, contentBackground, underWindowBackground, underPageBackground — each with blur radius, saturation, tint overlay (light / dark), blending mode (behindWindow / withinWindow), state (followsWindowActiveState / active / inactive), `isEmphasized`, and its opaque fallback under Reduce Transparency. | both (26 adds glass, MAC-X04) | P: `NSVisualEffectView.Material` docs; M: material capture in the `reduce-transparency-off` pass (exists) over a calibration pattern; `tinting-off` pass (exists) | `*.background_color` |
| MAC-X03 | Vibrant foregrounds | Labels, separators and icons on a material are blended (vibrancy); colours resolved under `vibrantLight` / `vibrantDark` differ from aqua / darkAqua. | both | R: resolve NSColor under `NSAppearance(named: .vibrantLight / .vibrantDark)`; M: blend behaviour from material captures | none |
| MAC-X04 | Liquid Glass | Glass style (regular / clear), tint, corner radius, refraction / lensing, specular rim, adaptive light / dark over content, interactive response (`effectIsInteractive`), merging (`NSGlassEffectContainerView.spacing`), glass morphing; used by `.glass` buttons, toolbar capsules, sidebars, menus, popovers, alerts, sheets; the user's Clear / Tinted setting (26.1, key to verify). | 26 only | P: `NSGlassEffectView` / `NSGlassEffectContainerView` docs, WWDC25 "Build an AppKit app with the new design" (to verify session number); M: material capture, recording | none |
| MAC-X05 | Scroll edge effect | Soft / hard / automatic effect where content scrolls under the toolbar / title bar (`NSScrollEdgeEffectStyle`, 26.1). | 26 only | P: docs; M: key-window capture with scrolled content | `toolbar.border.*` |
| MAC-X06 | Background extension | `NSBackgroundExtensionView`: content mirrored / blurred under a floating sidebar or inspector. | 26 only | P: docs; M | none |
| MAC-X07 | Corner geometry | Per-corner radii (segment ends, merged selections, grouped rows, toolbar groups); concentric radii derived from the container (26 `NSView.cornerConfiguration`); corner curve continuous vs circular (`CALayer.cornerCurve`). | differs (concentric 26 only) | R (26): `effectiveCornerRadii`; P: `CALayer.cornerCurve` docs; M: edge fit on renders | `border.corner_radius`, `corner_radius_lg` |
| MAC-X08 | Borders per side and in layers | Per-side colour / width (header bottom line, toolbar bottom, 12–15 field and checkbox top edges); outer + inner rims and bevel highlights. | both | M: existing renders | `Border.color`, `line_width` |
| MAC-X09 | Gradient fills | 12–15 gradients (dark checkbox box, others to verify). | 12–15 only | M: existing renders | solid colours only |
| MAC-X10 | Colour fidelity | System colours are defined beyond sRGB (P3; the reader converts to sRGB, platform-facts §1.1.2); HDR colours (colour well). The model is 8-bit sRGB RGBA. | both | R: NSColor `colorSpace` before conversion | `Rgba` |
| MAC-X11 | Motion | Duration, curve (or spring), delay and Reduce-Motion variant of: button press / release (26 glass bounce), checkbox / radio toggle, switch knob slide and track cross-fade, segmented selection slide (26), slider knob (26), disclosure-triangle rotation and outline expand / collapse, table row insert / remove (`NSTableView.AnimationOptions`), focus-ring appear, menu open, menu-item selection flash and menu fade-out, submenu open, popover show / close, sheet attach / detach, alert appear, window open / close (`NSWindow.AnimationBehavior` per kind), minimise (Genie / Scale, user setting), full-screen enter / exit, window resize, tab switch, tooltip fade, overlay scroller show / hide / expand / flash (`flashScrollers()`), elastic overscroll and bounce-back, indeterminate progress, spinner rotation, caret blink fade (14+), toolbar search expand, sidebar collapse, find-indicator bounce, failed-drop slide-back, spring-loading flash, glass morphing (26). | differs | R: `NSAnimationContext.duration` default (docs state it), `NSWindow.animationResizeTime(_:)`, `CAMediaTimingFunction` control points; minimise effect user default (Dock `mineffect`, to verify); M: CA introspection for layer-backed controls, recording for WindowServer-side motion — in a pass with Reduce Motion **off** (runners have it on, `probe.toml:466`), and again with it on | `dialog.entrance` (style only) |
| MAC-X12 | Visible timings | Caret blink on / off periods; tooltip delay (MAC-T05); double-click interval; key-repeat delay and interval (visible in fields, steppers); submenu open delay; overlay-scroller hide delay; drag threshold; spring-loading delay; button periodic delay (`NSButtonCell.getPeriodicDelay(_:interval:)`); menu selection-flash count. | both | R: `NSEvent.doubleClickInterval`, `keyRepeatDelay`, `keyRepeatInterval`; `getPeriodicDelay`; user defaults `NSTextInsertionPointBlinkPeriodOn/Off`, `NSInitialToolTipDelay` (undocumented, when set), spring-loading delay key (to verify); drag threshold (no API found — to verify); M: recording for the defaults | `tooltip.show_delay_ms` |
| MAC-X13 | Focus ring | Ring shape follows each control's bezel (ring corner radius per control); ring opacity (the colour carries alpha, `keyboardFocusIndicatorColor` #…7f in the probe); appear animation; which controls show it — text inputs and lists always, buttons / checkboxes / segments / links only with Full Keyboard Access (Keyboard navigation); per-control `focusRingType`; ring on table rows / cells; 26 offset ❓ (platform-facts §2.1.5). | differs | R: Full Keyboard Access — `NSApplication.isFullKeyboardAccessEnabled` (to verify) or user default `AppleKeyboardUIMode` (to verify); M: key-window capture with each control first responder, FKA on | `defaults.focus_ring_color`, `width`, `offset` |
| MAC-X14 | Caret | Width, shape, height relative to the line, blink style (14+ fades), caps-lock indicator (14+, `automaticModeOptions`), dictation effect (`effectsViewInserter`), caret in a text area / list cell editor. | differs (14+ `NSTextInsertionIndicator`) | P: `NSTextInsertionIndicator` docs; R: periods as MAC-X12; M: recording | `input.caret_color` |
| MAC-X15 | Selection | Unemphasized text selection (MAC-I05); unemphasized content-selection text colour (MAC-L03); emphasized secondary text (MAC-L04); `selectedControlColor` in completions; find highlight (`findHighlightColor`) and the find indicator bubble (rounded, shadowed, bouncing) with the dimming of non-matches during incremental find; selection shape in lists (MAC-L02). | both | R: NSColor; M: key-window text view with `NSTextFinder` | `defaults.selection_*`, `text_selection_*` |
| MAC-X16 | Key / non-key window, active / inactive app | Accent-coloured parts turn grey or plain in a non-key window: default button, checked checkbox / radio (to verify), switch on track, slider fill, progress fill, selected segment (26), pop-up arrow area (to verify), selection → unemphasized, sidebar material, traffic lights, toolbar items, title. Also "main but not key" (a panel is key). The model has only the title-bar and selection pair. | both | M: `not-key` pass (exists) + key-window elements for each control | `window.inactive_*`, `selection_inactive_background` |
| MAC-X17 | Control sizes | mini / small / regular / large (11+) / extraLarge (26, enum case confirmed; not probed, `probe.toml:606`): every control's height, padding, radius, indicator size, font size (`systemFontSize(for:)`, probed `probe.toml:20-24`), scroller width, switch size, spinner size; on 26 `prefersCompactControlSizeMetrics` restores pre-26 metrics — a third metric set. | differs | R: probe:controls per size (mini–large exist); add extraLarge and a `prefersCompactControlSizeMetrics = true` pass on 26 | one size per field |
| MAC-X18 | Typography | `Font` lacks: line height per role, letter spacing (SF Pro tracking per size — HIG Typography tracking table), optical size, width (`NSFont.Width`, 13+), system design (rounded / serif / monospaced, `NSFontDescriptor.SystemDesign`), features (monospaced digits — `monospacedDigitSystemFont`, probed `probe.toml:172`), emphasized weights; text antialiasing (font smoothing setting). | both | P: HIG typography JSON (tracking values); R: NSFont / NSFontDescriptor; font smoothing user default `AppleFontSmoothing` (to verify) | `Font` (family, size, weight, style, color) |
| MAC-X19 | Icon tint per state | Template images and SF Symbols take the label colour at rest and change when pressed, disabled, on an emphasized selection, in a sidebar (accent), on accent fills. | both | M: renders with images, each state | `expander.arrow_color` only |
| MAC-X20 | Cursors | Which system cursor each element shows (I-beam text, pointing-hand link, resize for splitters / columns / window edges, drag copy / link, not-allowed) and the accessibility pointer size and colours (12+). | both | R: `linkTextAttributes[.cursor]`, `NSCursor` per control (to verify per control); pointer settings user defaults (to verify) | none |
| MAC-X21 | Drag and drop | Drag image translucency, item-count badge, drop-target highlights (MAC-L11, text drop caret), spring-loading flash. | both | M: recording during a programmatic drag (`beginDraggingSession`) | none |
| MAC-X22 | Increase contrast | Every colour's high-contrast value (the reader resolves aqua / darkAqua only); controls gain outlines, separators darken. Four variants (light, dark, HC light, HC dark) where the model has two. | both | R: resolve NSColor under `accessibilityHighContrastAqua` / `…DarkAqua`; M: `increase-contrast-on` pass (exists) | `AccessibilityPreferences.high_contrast` (flag only) |
| MAC-X23 | Reduce transparency | Opaque values for every material / glass surface and the translucent values with it off (the runners have it on; `reduce-transparency-off` pass exists). | differs | M: both passes | `reduce_transparency` flag |
| MAC-X24 | Differentiate without colour | Flag missing from `AccessibilityPreferences` (platform-facts §1.1.7 sources it; the reader does not read it); what changes visibly (to verify). | both | R: `NSWorkspace.accessibilityDisplayShouldDifferentiateWithoutColor` (probed: `probe.toml:467`, run 40348854007) | none |
| MAC-X25 | Reduce motion variants | Flag exists; each motion's reduced form (cross-fade instead of zoom / slide) has no data (MAC-X11). | both | M: recordings with Reduce Motion on (runner default) | `reduce_motion` flag |
| MAC-X26 | Other accessibility | Invert colours (`accessibilityDisplayShouldInvertColors`; OS-applied, images excluded via `accessibilityIgnoresInvertColors`); larger text (14+ Text size setting, no public API — to verify); "Show toolbar button shapes" (bezels on toolbar items; API / key to verify); pointer size (MAC-X20). | differs | R / M as noted | `text_scaling_factor` |
| MAC-X27 | Appearance-level variants | Wallpaper tinting (`tinting-off` pass exists; user default `AppleReduceDesktopTinting` to verify), multicolour (app) accent, highlight colour (already read via `selectedTextBackgroundColor`). | both | R / M | light / dark only |
| MAC-X28 | Text marks | Spelling (red dotted) and grammar (green dotted) underlines, marked-text (IME) underline, inline predictions (14+), Writing Tools glow (15.1+), text highlight styles (15+, to verify). | differs | M: key-window text view with misspelt text; P: docs (to verify names) | none |
| MAC-X29 | State combinations | Checked + hover / pressed / disabled / non-key, selected + unfocused / disabled, default + non-key / pressed, focused + hover: most have no field (rows above); the model's state set is per-widget and flat. | both | M: as per row | per-widget `*_background` |

---

## Checked and not counted

- In-window status bar: macOS has none (platform-facts §2.14); the bottom bar is MAC-ST01.
- Visited link: macOS draws none (§2.28).
- Separator: no gap beyond MAC-X08.
- Text input error / invalid state: AppKit draws no standard invalid look (to verify; count it if a render shows one).
- Haptic feedback (Force Touch alignment): not visible.
- `NSForm` / `NSMatrix`: deprecated, drawn as the controls they contain.
- `NSStackView` / `NSGridView`: layout only (covered by `[layout]`).

## Counts

- 4.1: 173 rows, 171 gaps (MAC-SE01 records no gap; MAC-SL08 points to MAC-N15).
- 4.2: 26 rows.
- 4.3: 29 rows.
