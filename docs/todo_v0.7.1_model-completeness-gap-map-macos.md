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
  `NSSegmentedControl.borderShape` / `NSControl.BorderShape` (the BorderShape page did not load
  then; it loaded in the review on 2026-10-28 as `appkit/nscontrol/bordershape.json`: macOS 26.0,
  cases automatic, capsule, circle, roundedRectangle); `NSPopover.hasFullSizeContent`; `NSMenuItem.sectionHeader(title:)`,
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
    its shadow. `extraLarge` is not probed (`probe.toml:604`, run 40348854007, macos-26; review 2026-10-28: was :606).
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
| MAC-W09 | Window tab bar (native tabbing, "Prefer tabs") | The whole part: bar height, fill, selected / unselected tab fill, tab separators, tab title font and colour, close button on hover, "+" (new tab) button, inactive-window look, drag look. What the bar draws when its tabs no longer fit: MAC-P4-5†. | differs (26 to verify) | M: new key-window element — two windows joined with `addTabbedWindow(_:ordered:)`, capture, hover capture | none (`tab.*` is a tab view's) (review 2026-10-28: pointer to MAC-P4-5† for overflow) |
| MAC-W10 | Title-bar accessory / unified toolbar background | The toolbar area's fill per toolbar style and its material (`.titlebar`); on 26 the title bar is transparent over content with glass item groups (MAC-TL03). | differs | P: `NSVisualEffectView.Material.titlebar`; M: material capture | `window.title_bar_background`, `toolbar.background_color` |
| MAC-W11† | Document title menu | A document window (`representedURL`, `isDocumentEdited`): the chevron shown beside the title on hover, the popover it opens (rename and location — contents to verify), the "Edited" mark in the title. | both (26 to verify) | P: `NSWindow.representedURL` docs; M: key-window capture of a document window, hover capture over the title, the popover captured as MAC-PO01 | `window.title_bar_font` |
| MAC-W12† | Full-screen window | The full-screen state: the window's corners and edge in it, the title bar and toolbar hidden and revealed when the pointer reaches the top edge (the reveal's background — `.fullScreenUI` material), the menu bar coming in with it, the divider between two windows in a full-screen Split View. | both (26 to verify) | P: HIG Going full screen; `NSVisualEffectView.Material.fullScreenUI`; M: key-window capture after `toggleFullScreen(_:)`, hover capture at the top edge, recording (MAC-X32) | none |
| MAC-W13† | Window tiling | The zoom button's hover menu (full screen and tile left / right on 12–14; tiling layouts on 15+ — item names to verify), the 15+ drag-to-edge tiling preview, the gaps between tiled windows (the "Tiled windows have margins" setting), the tiled window's corners. | differs (15+ layouts and drag tiling; 12–14 menu only; 26 to verify) | P: Apple Support, Mac User Guide "Tile windows" (page to verify); R: the margins setting (`com.apple.WindowManager` key to verify); M: hover capture over the zoom button, recording of a drag to the screen edge | none |
| MAC-W14† | Title-bar variants | `titlebarAppearsTransparent` (10.10: the title bar draws no background; content under it with `fullSizeContentView`), `titleVisibility` hidden (traffic lights only), a titled window without a toolbar vs with one: each changes the fill, separator and title placement the rows above assume. | both | P: `NSWindow.titlebarAppearsTransparent`, `titleVisibility` docs (read 2026-10-28); M: key-window capture per variant | `window.title_bar_background` |
| MAC-W15† | Inactive title text colour | `inactive_title_bar_text_color` is (none) on 12–15 and ❓ on 26 (platform-facts.md@fc6109d1:1167), yet a non-key window draws its title differently from a key one; the reader reads only `windowFrameTextColor` (`native-theme/src/macos.rs:196`). | both | M: the `not-key` pass of the key-window capture (exists), samples on the title | `window.inactive_title_bar_text_color` |

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
| MAC-B08 | Border shape (26) | `NSControl.BorderShape` (automatic, capsule, roundedRectangle, circle; macOS 26.0): a capsule button at larger sizes; the automatic shape per control size. | 26 only | P: `NSControl.BorderShape` (`appkit/nscontrol/bordershape.json`, read 2026-10-28); M: new render per size and shape | `button.border.corner_radius` (review 2026-10-28: the cases and the page settled, no longer "to verify") |
| MAC-B09 | Button image | Icon size inside the button per control size, icon colour per state (template tint: rest, pressed, disabled, on accent), image positions (`imagePosition`), image-only button padding. | both | M: new render with an SF Symbol image, each state | `button.icon_text_gap` only |
| MAC-B10 | Checked toggle button (`pushOnPushOff`) | Checked + pressed, checked + disabled, checked in a non-key window. | both | M: new render (state on, highlighted, disabled), key-window element | `button.checked_*` |
| MAC-B11† | Minimum width | `min_width` is (none), "sizes to content" (platform-facts.md@fc6109d1:1182), yet a push button with a short title has a drawn minimum (bezel ends and padding) per control size; whether AppKit imposes a wider one: to verify. | differs (to verify) | R: `intrinsicContentSize` of a push button with an empty and a one-character title, per control size; M: existing renders | `button.min_width` |

### Text input (`[input]`) and text area (`[text_area]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-I01 | Field bezel styles | Square vs rounded bezel (`NSTextField.BezelStyle`), bordered-not-bezeled, the borderless label field, the field editor inside a table cell. | both | P: `NSTextField.BezelStyle` docs; M: new render per style | `input.*` (one style) |
| MAC-I02 | Read-only state | A non-editable, selectable field's look (bezel, fill, text colour) — no state field. | both (to verify whether it differs from editable) | M: new render `isEditable = false` | none |
| MAC-I03 | Secure field | Bullet glyph, the caps-lock indicator drawn inside `NSSecureTextField`. | both | M: new render; caps lock via `CGEventSourceFlagsState` cannot be faked — screen recording on a runner with a caps-lock key event (to verify feasibility) | none |
| MAC-I04 | Bezel edge paint | 12–15: the field bezel's per-side edge (top edge shading — to verify) and any inner shadow; 26: the 1-pt outer ring "of alpha ≤ 23/255" platform-facts §2.4 measured, which a border colour + width cannot state as an outer ring. | differs | M: existing `text_field-regular@2x.png` renders | `input.border.color`, `line_width` |
| MAC-I05 | Inactive text selection | `unemphasizedSelectedTextBackgroundColor` / `unemphasizedSelectedTextColor` (the selection when the field is not first responder or the window is not key) — read by the probe (`probe.toml:332`, :406) but no field. | both | R: the two NSColor class properties | `input.selection_background`, `selection_text_color` (emphasized only) |
| MAC-I06 | Completion / suggestion UI | The completions list (`NSTextView.complete(_:)`) and the macOS 14 inline prediction ghost text (its colour). | both (inline predictions 14+) | P: `NSTextInputTraits.inlinePredictionType` (macOS 14.0); M: recording in a text view | none (review 2026-10-28: the property's name settled) |
| MAC-I07 | Text area extras | Ruler (`NSRulerView`) and the find bar are not the field's (MAC-N17, MAC-N18); the text area's default font is `userFont(ofSize:)` (probe `[macos.fonts.named.userFontOfSize]`, `probe.toml:139`), not the system font — no field. | both | R: `NSFont.userFont(ofSize: 0)`, `userFixedPitchFont(ofSize: 0)` | `input.font`, `defaults.mono_font` |
| MAC-I08 | Text area frame | The enclosing `NSScrollView.borderType` frame (none / line / bezel / groove) and its colour. | both | P: `NSScrollView.borderType` docs; M: new render per border type | `text_area.border` (inherits input) |
| MAC-I09† | AutoFill look | A field with a `contentType` (`NSTextContentType`) can show the AutoFill / Passwords button inside the field and a tinted fill once filled or for a suggested strong password: glyph, fill and text colours (behaviour per version to verify). | to verify | P: `NSTextField.contentType`, `NSTextContentType` docs; M: key-window capture of a password field with a new-password content type (feasibility on a runner to verify) | none |
| MAC-I10† | Text area insets | `text_area.border.padding_*` are (none) (platform-facts.md@fc6109d1:1714-1715): the drawn inset is `lineFragmentPadding` (its default is published there) plus `textContainerInset`, whose default no document states. | both | R: `NSTextView.scrollableTextView()` (10.14): the text view's `textContainerInset` and `textContainer.lineFragmentPadding` | `text_area.border.padding_*` |

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
| MAC-MN09 | Subtitle | `NSMenuItem.subtitle` font and colour ("displayed below the standard title"). | 14.4+ and 26 | P: `appkit/nsmenuitem/subtitle` (macOS 14.4, read 2026-10-28); M: menu capture | none (review 2026-10-28: version settled, 14.4) |
| MAC-MN10 | Item icon colour | Icon tint at rest / highlighted / disabled; 27 hides item images by default (platform-facts §2.6). | differs | M: menu capture with SF Symbol images | `menu.icon_size` |
| MAC-MN11 | Indentation | `NSMenuItem.indentationLevel` step width. | both | M: menu capture | none |
| MAC-MN12 | Scroll arrows | The up / down scroll areas of a menu taller than the screen. | both | M: menu capture with many items | none |
| MAC-MN13 | Palette menu (14+) | `NSMenu` palette presentation: swatch items, selection ring. | 14+ and 26 | P: `NSMenu.presentationStyle` (macOS 14.0), `NSMenu.palette(colors:titles:template:onSelectionChange:)` docs (read 2026-10-28); M: capture | none (review 2026-10-28: the property's name settled) |
| MAC-MN14 | Menu edge and shadow | Container edge line / rim and every shadow layer (MAC-X01); the menu material (MAC-X02). | differs | M: shadow capture of a menu window | `menu.border.*` ("from §2.16") |
| MAC-MN15 | Pop-up menu geometry | A pop-up button's menu opens with the selected item over the button (offset), its font follows the control size. The pull-down menu's placement and width, and the combo box list's: MAC-P4-8†. | both | M: capture with a pop-up button's menu open | none (review 2026-10-28: pointer to MAC-P4-8†) |
| MAC-MN16 | Minimum width | The menu's drawn minimum width: `NSMenu.minimumWidth` defaults to 0 ("The menu will not draw smaller than its minimum width", 10.6), so AppKit imposes none through it and the narrowest menu's width comes from its content and insets. | both | P: `appkit/nsmenu/minimumwidth` (read 2026-10-28); M: menu capture of a menu with one short item | none (review 2026-10-28: the default settled, 0; source moved from R to P + M) |
| MAC-MN17† | Destructive item | SwiftUI `Button(role: .destructive)` in a `Menu` or context menu: whether macOS draws the item differently (text colour) — to verify. | to verify | M: SwiftUI host (as MAC-N21) with a destructive menu item, menu capture | `defaults.danger_color` |

### Tooltip (`[tooltip]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-T01 | Tooltip shadow | Layers (MAC-X01). | differs (to verify) | M: hover capture without `-o` | `tooltip.border.shadow_enabled` |
| MAC-T02 | Tooltip material | The `.toolTip` material — a solid colour cannot carry it. | differs | P: `NSVisualEffectView.Material.toolTip`; M: material capture | `tooltip.background_color` |
| MAC-T03 | Tooltip position | Offset from the pointer / below the element; where the tooltip goes when that place would cross a screen edge (flipped, shifted inward: to measure). | both | M: hover capture, also with the pointer near each screen edge | none (review 2026-10-28: the screen edge added) |
| MAC-T04 | Tooltip timings | Re-show delay when moving between items (instant after one is shown — to verify), auto-hide, fade-in/out (MAC-X11). | both | M: recording with timed hovers | `tooltip.show_delay_ms` |
| MAC-T05 | Show delay source | Field exists but its source: R the `NSInitialToolTipDelay` user default when set (undocumented), M the default. | both | R / M | `tooltip.show_delay_ms` |
| MAC-T06† | Expansion tooltip | A second tooltip kind: the full text of a truncated cell or field drawn in a frame over it on hover (`NSControl.allowsExpansionToolTips`, 10.8; `tableView(_:shouldShowCellExpansionFor:row:)`, 10.5): fill, border, shadow, text colour, delay. | both | P: the two docs (read 2026-10-28); M: hover capture over a truncated cell in the key window | `tooltip.*` |

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
| MAC-P03 | Non-key fill | Fill draws grey in a non-key window (the not-key `progress_bar-regular@2x.png` renders, run 40348854007, macos-14 and macos-26; the README states the renders draw the not-key look but names no progress bar) — MAC-X16. | both | M: key-window element | `progress_bar.fill_color` (review 2026-10-28: citation, the README does not name the progress bar) |
| MAC-P04 | Value change motion | Whether the fill animates to a new value. | both (to verify) | M: CA introspection | none |
| MAC-P05† | Track edge | `progress_bar.border.color` / `line_width` are (none), "not read" (platform-facts.md@fc6109d1:1339-1340): whether the track draws an edge line, its colour and width. | differs (to verify) | M: existing `progress_bar-regular@2x.png` renders, edge profile | `progress_bar.border.*` |

### Tab view (`[tab]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-TB01 | Tab view content box | The bezel box under the tabs: fill, border, radius, padding (`contentRect` vs frame). | both | R: `NSTabView.contentRect` inset; M: new render | none |
| MAC-TB02 | Tab strip geometry | Tabs drawn as a segmented strip centred over and straddling the box's top edge (macOS 11+): the overlap offset. | differs (to verify on 26) | M: new render | `tab.*` |
| MAC-TB03 | Tab variants | `tabPosition` (top / bottom / left / right / none), `tabViewBorderType` (none / line / bezel), `tabViewType`. | both | P: `NSTabView` docs; M: new render per variant | `tab.active_indicator_side` (Top / Bottom only) |
| MAC-TB04 | Tab states | Pressed tab, selected tab in a non-key window. | both | M: pressed render, not-key pass | none |
| MAC-TB05 | Tab metrics per control size | `NSTabView.controlSize`, `font` per size. | both | M: new render per size | MAC-X17 |
| MAC-TB06† | Tab strip paint | `tab.border.*` are (none), a CoreUI composite (platform-facts.md@fc6109d1:1354-1356); the strip is drawn like a segmented control (MAC-TB02), so its outer edge, the separators between unselected tabs, the selected tab's fill and label colour per generation are values to measure, not absent. | differs | M: new `NSTabView` render, aqua / darkAqua; key-window element for the selected tab | `tab.*` |
| MAC-TB07† | Tab overflow | What the strip draws when the tabs do not fit: truncated labels (`NSTabViewItem.sizeOfLabel(_:)`, to verify), hidden tabs or an overflow control (to verify). | both (to verify) | M: new render of a narrow tab view with many tabs | none |

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
| MAC-SB09† | Sidebar width | The sidebar's default, minimum and maximum width and its collapse rule; `[sidebar]` has no width. | differs (26 floating sidebar to verify) | R: `minimumThickness` / `maximumThickness` (10.11) and the frame of a new `NSSplitViewItem(sidebarWithViewController:)` in the key window | none |

### Toolbar (`[toolbar]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-TL01 | Height per toolbar style | unified / unifiedCompact / expanded / preference each have their own height and margins ("reduced margins" for unifiedCompact, docs). | differs | R: `contentLayoutRect` vs frame per style (probe key window) | `toolbar.bar_height`, `border.padding_*` |
| MAC-TL02 | Toolbar item button | Rest / hover / pressed bezel (`isBordered`), icon colour per state, label (icon-and-label display mode) font, colour and gap below the icon, disabled item. | differs | M: key-window toolbar, hover capture, pressed render | `toolbar.font`, `icon_size` |
| MAC-TL03 | Item groups / glass capsules | `NSToolbarItemGroup` segmented look; 26: items grouped in glass capsules, their padding, spacing between capsules, capsule height. | differs (capsules 26 only) | P: `NSGlassEffectContainerView.spacing`; M: key-window toolbar | `toolbar.item_gap` |
| MAC-TL04 | Prominent item (26) | `NSToolbarItem.style = .prominent` ("tints the background"), `backgroundTintColor`. | 26 only | P: docs; M | none |
| MAC-TL05 | Toolbar badges | `NSToolbarItem.badge` / `NSItemBadge`: shape, fill, font. | 26 only | P: `appkit/nstoolbaritem/badge` (macOS 26.0, read 2026-10-28); M | none (review 2026-10-28: version settled, 26.0) |
| MAC-TL06 | Overflow | The overflow chevron button and its menu. | both | M: narrow key window | none |
| MAC-TL07 | Search toolbar item | `NSSearchToolbarItem` collapsed (button) vs expanded field, and its expand motion. | both | M: capture + recording | none |
| MAC-TL08 | Tracking separator | `NSTrackingSeparatorToolbarItem` line aligned with the split view divider. | 12–15 (26 to verify) | M | none |
| MAC-TL09 | Toolbar background | 12–15: title-bar material; 26: transparent over content with the scroll edge effect (MAC-X05). | differs | M: material capture | `toolbar.background_color` |
| MAC-TL10† | Space items | Width of the fixed space item (`NSToolbarItem.Identifier.space`) and the flexible space's minimum. | differs (26 glass groups to verify) | M: key-window toolbar with space items, item frames | `toolbar.item_gap` |
| MAC-TL11† | Selectable items | A toolbar with selectable items (`NSToolbar.selectedItemIdentifier`; the Settings-window pattern: `NSTabViewController.TabStyle.toolbar`, toolbar style `.preference`): the selected item's background, icon tint and label colour, its non-key look. | differs | P: `selectedItemIdentifier`, `NSTabViewController.TabStyle` docs (read 2026-10-28); M: key-window toolbar with a selected item, not-key pass | none |

### Status bar (`[status_bar]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-ST01 | Window bottom bar | macOS's in-window counterpart is the content border (`NSWindow.setContentBorderThickness(_:for: .minY)`): its height, fill, top separator, inactive look. The model's status bar has no height. | differs (to verify on 26) | P: `setContentBorderThickness` docs; M: key-window element | `status_bar.*` (no height) |

### List / table / outline (`[list]`)

| ID | Element | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-L01 | Table styles | `NSTableView.Style` automatic / fullWidth / inset / sourceList / plain: horizontal inset per style (the header's column rect starts at x 10 in inset and 0 in fullWidth, `probe.toml:511`, :539, run 40348854007 macos-26), row radius per style. | both | R: `headerRect(ofColumn:)` / `rect(ofRow:)` per style; M: key-window table per style | `list.border.padding_*` (review 2026-10-28: lines :510, :538 were the header frame, the column rect is one line below) |
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
| MAC-L17† | Selection in special contexts | Controls inside an emphasized selected row (checkbox, button, pop-up, progress — `NSTableCellView.backgroundStyle`) and the selected row of a disabled table: what changes — to verify. | both (to verify) | M: key-window table with controls in a selected row; the same table disabled | `list.selection_*` |
| MAC-L18† | Header extras | The highlighted column's header (`highlightedTableColumn`) and the corner view right of the header above the vertical scroller (`cornerView`): fill and separators; whether 12+ draws the highlight: to verify. | both | P: the two docs (read 2026-10-28); M: key-window table with a highlighted column and a legacy scroller | `list.header_*` |
| MAC-L19† | Context-menu target | The row (also a sidebar row or collection item) whose context menu is open is drawn with an outline, not a fill (`clickedRow`): colour, width, radius, inset. | both (26 to verify) | M: secondary click in the key window (`CGEvent`), capture while the menu is open | `list.selection_background` |

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
| MAC-SN05† | Start / stop | Whether the spinner fades in on start and out on stop (`isDisplayedWhenStopped`). | both (to verify) | M: recording / CA introspection | none |

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
| MAC-SG02 | Selected segment shadow | 12–15: the raised selected segment's shadow (key-window capture, macOS 14). | 12–15 only | M: key-window capture without `-o` | `segmented_control.border.shadow_enabled` (the control's, a flag; no layers) (review 2026-10-28: was "none"; the C table's line 1644 names this field) |
| MAC-SG03 | Selected label weight (26) | 26 draws the selected segment accent-filled with a heavier white label (`renders/key_window/aqua.png`, run 40348854007, macos-26); 12–15 neutral raised. `active_text_color` exists, active font weight does not. | differs | M: existing key-window captures | `active_background`, `active_text_color` |
| MAC-SG04 | Styles | automatic, rounded, roundRect, texturedRounded, capsule, texturedSquare, smallSquare, separated (platform-facts §2.25) and the 26 `borderShape`. | differs | P: `NSSegmentedControl.Style`; M: new render per style | one style |
| MAC-SG05 | Tracking modes | selectAny (several selected), momentary (pressed only); `selectedSegmentBezelColor` ("in appearances that support it"). | both | R: `selectedSegmentBezelColor` default; M: new render per mode | `active_background` |
| MAC-SG06 | States | Pressed segment, selected + pressed, disabled segment (`setEnabled(_:forSegment:)`), selected in a non-key window (26 accent → grey?). | both | M: pressed render, not-key pass | `hover_background`, `disabled_opacity` |
| MAC-SG07 | Menu indicator | `showsMenuIndicator(forSegment:)` glyph. | both | M: new render | none |
| MAC-SG08 | Per-corner radii | Only the outer corners of the end segments are round; the selected indicator's own inset radius (MAC-X07). | both | M: existing renders | `border.corner_radius` |
| MAC-SG09 | Selection motion (26) | Sliding selection indicator (MAC-X11). | 26 (to verify) | M: CA introspection | none |
| MAC-SG10† | Segment images | Image size per control size, `imageScaling`, the image tint at rest, selected (on the 26 accent fill: to verify), pressed and disabled. | differs | P: `NSSegmentedControl.setImage(_:forSegment:)`, `setImageScaling(_:forSegment:)` docs; M: new render with SF Symbol segments | none |

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
| MAC-EX05† | Disclosure geometry | The triangle button's size per control size, its gap to the label (`arrow_gap`, (none) at platform-facts.md@fc6109d1:1684), the indent of the disclosed content (`content_indent`, :1685; SwiftUI `DisclosureGroup`), the header height (`header_height`, :1675). | both | R: `intrinsicContentSize` of a `.disclosure` button per control size; M: SwiftUI `DisclosureGroup` capture (host as MAC-N21) | `expander.arrow_gap`, `content_indent`, `header_height` |

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
| MAC-TS02 | Named fonts | `labelFontOfSize`, `messageFontOfSize`, `paletteFontOfSize`, `smallSystemFontSize`, `controlContentFontOfSize`, `menuBarFontOfSize` (all probed, `[macos.fonts.named]`; `smallSystemFontSize` under `[macos.fonts.sizes]`, `probe.toml:17`) have no fields. | both | R: NSFont | `defaults.font` (review 2026-10-28: `smallSystemFontSize` is probed under `[macos.fonts.sizes]`, not `[macos.fonts.named]`) |
| MAC-LY01 | Layout spacing | Label-to-control gap per control size and inter-button spacing per size (platform-facts §1.1.5: HIG 8/6/5, 12 for push buttons — legacy HIG); 26 window content margins concentric with the window. | differs | P: legacy HIG (archived), current HIG Layout; M | `[layout]` (4 fields) |
| MAC-IC01 | Icon contexts | Button icon, list-row icon, tab icon, toolbar icon per toolbar size mode, status-item icon, sidebar icon per sidebar size — `icon_sizes` has 5 contexts. | both | P: platform-facts §1.1.6; M | `defaults.icon_sizes` |
| MAC-IC02 | SF Symbol configuration | Weight and scale (small / medium / large) per context so a symbol matches adjacent text; rendering mode (monochrome / hierarchical / palette / multicolor). | both | P: HIG SF Symbols; R: `NSImage.SymbolConfiguration` | none |
| MAC-IC03 | Icon appearance (26) | Icon & widget style (Default / Dark / Clear / Tinted) changes the app and folder icons an app shows. | 26 only | R: user default (key to verify) | none |
| MAC-DF05† | Unread UI element colours | AppKit UI element colours neither the reader (24 `NSColor` class properties, `native-theme/src/macos.rs`) nor a row reads: `textColor`, `textBackgroundColor`, `controlColor`, `controlTextColor`, `selectedControlTextColor`, `selectedMenuItemTextColor`, `highlightColor`, and the `currentControlTint` setting (`shadowColor` is left out on purpose, macos.rs:157). Five already have a model field in `platform-facts.md`, which the preset carries and the reader does not read (reader work, not a model gap): `textBackgroundColor` → `input.background_color` (:1203, also named in MAC-DF04), `textColor` → `input.font.color` (:1205), `controlColor` → `button.background_color` and `combo_box.background_color` on 12–15 (:1177, :1611), `controlTextColor` → `button.font.color` (:1179), `selectedMenuItemTextColor` → `menu.hover_text_color` (:1270). Which model field `selectedControlTextColor`, `highlightColor` and `currentControlTint` belong to is the rationale's question. | both | R: NSColor (AppKit "UI Element Colors", read 2026-10-28) | `input.*`, `menu.*`, `defaults.*` (review 2026-10-28: five of the eight are mapped to existing fields in platform-facts; the model gap is the other three) |
| MAC-TS03† | Leading variants | Tight and loose leading (`NSFontDescriptor.SymbolicTraits.tightLeading` / `.looseLeading`; SwiftUI `Font.leading(_:)`): the line height a style takes in each. | both | R: a font descriptor with the trait, its font's line height; P: docs (read 2026-10-28) | `[text_scale]` (one line height) |
| MAC-LY02† | Form label alignment | Labels trailing-aligned in a column beside leading-aligned controls (SwiftUI `ColumnsFormStyle`, 13.0: "a trailing aligned column of labels next to a leading aligned column of values"), label-to-control baseline alignment, the label gap per control size (MAC-LY01). | both | P: `ColumnsFormStyle` docs (read 2026-10-28); M: SwiftUI host (as MAC-N21) | `[layout]` |
| MAC-IC04† | Alert icon size | The `dialog` icon-size context is (none) (platform-facts.md@fc6109d1:1139), yet `NSAlert` draws the app icon (and the caution badge, MAC-D02); the widget field `dialog.icon_size_px` states 64 for 12–15 with no source cited and ❓ for 26 (platform-facts.md@fc6109d1:1568). The size is a value to read or measure, not a missing field. | both (26 to verify) | R: the icon view's frame after `NSAlert.layout()` | `dialog.icon_size_px`, `defaults.icon_sizes.dialog_px` (review 2026-10-28: "a size no document states" contradicted platform-facts.md@fc6109d1:1568's unsourced 64; the existing `dialog.icon_size_px` added as nearest field) |
| MAC-IC05† | Symbol effects and transitions | SF Symbol effects and content transitions that controls and apps play (`NSImageView.addSymbolEffect(_:options:animated:)`, `setSymbolImage(_:contentTransition:options:)`, 14.0); the symbol variant (fill / outline) a control picks per state (to verify). | 14+ and 26 | P: `NSImageView` docs (read 2026-10-28); M: recording | none |

---

## 4.2 Standard controls the model does not have

| ID | Control | What it draws (all missing) | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-N01 | `NSStepper` | Up / down halves, arrow glyphs, separator, pressed half, disabled half at a limit, size variants, field + stepper pairing gap, autorepeat timing. | differs (existing renders: 26 draws one translucent rounded body with an inset line between the halves; 12–15 an opaque bezel) | R: metrics already probed (`[macos.controls.*.stepper]`, `probe.toml:1768`); M: existing `stepper-regular@2x.png` renders + pressed render | none (`button`) (review 2026-10-28: the 26 render, run 40348854007, shows one body with a separator line, not separate halves) |
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
| MAC-N20 | Menu bar and status items | Bar height, font (`menuBarFontOfSize:`), bold app name, open-menu title highlight, background (wallpaper-tinted translucent; 26 transparent), status-item icon size and spacing; the font at the Accessibility "Menu bar size" setting (Apple Support *Change Display settings for accessibility on Mac* (`https://support.apple.com/guide/mac-help/unac089/26/mac`, 26 edition, read 2026-10-28): "Set the font size in the menu bar to Large, or back to Default"; 12–15: to verify). Seeds: `docs/todo.md:1178`, `:3707` ("The menu-bar row has no fill or font of its own"; `:3608` at caeddd6b). | differs | R: `NSStatusBar.system.thickness`, `NSMenu.menuBarHeight` (0 in the probe without a menu bar, `probe.toml:597` — needs a regular app with a main menu); M: capture of the menu bar | none (review 2026-10-28: the second seed moved to todo.md:3707 since caeddd6b; the "Menu bar size" setting added) |
| MAC-N21 | SwiftUI grouped form | `.formStyle(.grouped)` (System Settings look, 13+): inset rounded section cards, row separators, row height, section header / footer fonts. | 13+; differs | P: SwiftUI `GroupedFormStyle` docs; M: needs a SwiftUI host (Swift, not the objc2 probe — to verify feasibility) | `card.*`, `list.*` |
| MAC-N22 | SwiftUI `Gauge` | Linear and circular (accessory) gauges, 13+. | 13+ | P: docs; M as MAC-N21 | none |
| MAC-N23 | SwiftUI tab bar (15+) | `TabView` sidebar-adaptable / toolbar tab bar. | 15+ and 26 | P: docs; M as MAC-N21 | `tab.*` |
| MAC-N24 | Pull-to-refresh (27) | `NSScrollView.refreshController` (27.0): refresh indicator at the scroll top. | 27 only | P: docs; M | `spinner.*` |
| MAC-N25 | System-drawn surfaces | Notification banners and Notification Center, Dock tile badge (`NSDockTile.badgeLabel`), share picker, the standard panels (open / save, colour, font, print). Drawn by the OS or AppKit, not by the app's toolkit; counted here because they are visible platform UI, and the rationale decides whether the model covers them. | differs | M: capture | none |
| MAC-N26 | Touch Bar | `NSTouchBar` items, `NSScrubber`: hardware-specific (Touch Bar Macs). | 12–15; 26 on supported Intel Macs | M: Touch Bar simulator (to verify) | none |
| MAC-N27† | Text input system UI | Drawn by the OS in or beside any text field: the press-and-hold accent popup, the Character Viewer / emoji picker, the input-method candidate window, the Look Up popover with its highlighted word, the dictation and caps-lock indicators (MAC-X14). Counted as MAC-N25 is: visible platform UI the app's toolkit does not draw. | differs (to verify per generation) | M: key-window text field, key events (`CGEvent`; feasibility of each on a runner to verify), capture | none |
| MAC-N28† | Split button | `NSComboButton` (13.0) style `.split` ("separates the button's title and image from the menu indicator"): the two parts, the divider, the chevron, each part's states; SwiftUI `Menu(content:label:primaryAction:)` (12.0), drawn as a split button (to verify). | 13+ (SwiftUI 12+); differs | P: `NSComboButton`, `NSComboButton.Style` docs (read 2026-10-28); M: new render per style and size | `button.*` |
| MAC-N29† | Menu button | `NSComboButton` `.unified` ("unifies the button's title and image with the menu indicator"), `NSMenuToolbarItem` (10.15) with its indicator, the pull-down `NSPopUpButton` (MAC-CB01): indicator glyph, its gap to the title, the pressed and open states. | 13+ (toolbar item 10.15+); differs | P: docs (read 2026-10-28); M: new render, key-window toolbar | `combo_box.*` |
| MAC-N30† | `AVPlayerView` controls | AVKit's transport controls, `AVPlayerViewControlsStyle` inline ("a bar along the view's bottom edge"), floating ("a floating window over the video content"), minimal, default: bar material, play / pause, scrubber, time labels, volume, the full-screen, sharing and frame-stepping buttons, auto-hide timing. | differs (26 to verify) | P: `AVPlayerView`, `AVPlayerViewControlsStyle` docs (read 2026-10-28); M: capture of an `AVPlayerView` playing a local file, recording for auto-hide | none |
| MAC-N31† | Rubber-band selection | The marquee drawn while drag-selecting in an icon or collection view: fill, border, radius (Finder draws one; whether `NSCollectionView` does by default: to verify). | both (to verify) | M: drag in a key-window `NSCollectionView` (`CGEvent`), capture | none |
| MAC-N32† | Standard About panel | `orderFrontStandardAboutPanel(options:)`: app icon size, application-name font, version line, credits text view, copyright line, their gaps, the panel width. | both (26 to verify) | P: `NSApplication.AboutPanelOptionKey` docs (read 2026-10-28); R: the panel's subview frames and fonts once open; M: capture | `dialog.*` |
| MAC-N33† | Help-menu search | The search field the OS adds to the Help menu, its results (menu items, help topics) and the pointer it draws at a found menu item (`registerUserInterfaceItemSearchHandler(_:)`, 10.6). | both (to verify) | P: docs (read 2026-10-28); M: capture of the Help menu with typed text (the regular-app setup of MAC-N20) | `menu.*` |
| MAC-N34† | Menu-bar extra window | SwiftUI `MenuBarExtra` (13.0) with `WindowMenuBarExtraStyle` ("renders its contents in a popover-like window"): material, radius, shadow, gap to the menu bar; 27's `NSStatusItemExpandedInterfaceSession` (what it draws: to verify). | differs (13+; 27 expanded interface) | P: `MenuBarExtra`, `WindowMenuBarExtraStyle`, `NSStatusItemExpandedInterfaceSession` docs (read 2026-10-28); M: capture | `popover.*` |
| MAC-N35† | `ContentUnavailableView` | SwiftUI (14.0) empty-content view: image size and colour, title font, description font and colour, actions, spacing; its search variant. | 14+ and 26 | P: docs (read 2026-10-28); M: SwiftUI host (as MAC-N21) | none |
| MAC-N36† | TipKit tips | `TipView` (inline) and popover tips (14.0): container fill, radius, image, title and message fonts, close button, action buttons, the popover arrow. | 14+ and 26 | P: TipKit, `TipView` docs (read 2026-10-28); M: SwiftUI host | `popover.*` |
| MAC-N37† | Swift Charts defaults | Swift Charts (13.0) draws with system defaults: axis and grid-line colours and widths, axis-label font, default series colours, legend font and symbols, selection marks. A HIG component (Charts); the rationale decides whether the model covers it. | 13+ | P: Swift Charts docs (read 2026-10-28), HIG Charts; M: SwiftUI host | none |

---

## 4.3 Properties that cross widgets

| ID | Property | What is missing | Gen | Source | Nearest field |
|---|---|---|---|---|---|
| MAC-X01 | Shadows / elevation | Per surface and state, every layer's offset, blur, spread, colour, inset or not: window key / non-key, utility panel, HUD, sheet, alert, menu, popover, tooltip, drag image; 12–15 controls (push button, pop-up, selected segment, slider knob, switch knob); 26 glass shadows. | differs | M: shadow capture per surface; existing control renders' alpha for control shadows; P (26, WebKit's own vector controls): `RenderThemeCocoa.mm` as platform-facts cites it | `defaults.shadow_color`, `border.shadow_enabled` |
| MAC-X02 | Materials / vibrancy | Each surface is a material, not a colour: titlebar, selection, menu, popover, sidebar, headerView, sheet, windowBackground, hudWindow, fullScreenUI, toolTip, contentBackground, underWindowBackground, underPageBackground — each with blur radius, saturation, tint overlay (light / dark), blending mode (behindWindow / withinWindow), state (followsWindowActiveState / active / inactive), `isEmphasized`, and its opaque fallback under Reduce Transparency. | both (26 adds glass, MAC-X04) | P: `NSVisualEffectView.Material` docs; M: material capture in the `reduce-transparency-off` pass (exists) over a calibration pattern; `tinting-off` pass (exists) | `*.background_color` |
| MAC-X03 | Vibrant foregrounds | Labels, separators and icons on a material are blended (vibrancy); colours resolved under `vibrantLight` / `vibrantDark` differ from aqua / darkAqua. | both | R: resolve NSColor under `NSAppearance(named: .vibrantLight / .vibrantDark)`; M: blend behaviour from material captures | none |
| MAC-X04 | Liquid Glass | Glass style (regular / clear), tint, corner radius, refraction / lensing, specular rim, adaptive light / dark over content, interactive response (`effectIsInteractive`), merging (`NSGlassEffectContainerView.spacing`), glass morphing; used by `.glass` buttons, toolbar capsules, sidebars, menus, popovers, alerts, sheets; the user's Clear / Tinted setting (26.1, key to verify). | 26 only | P: `NSGlassEffectView` / `NSGlassEffectContainerView` docs, WWDC25 session 310, "Build an AppKit app with the new design" (`developer.apple.com/videos/play/wwdc2025/310/`, read 2026-10-28); M: material capture, recording | none (review 2026-10-28: session number settled) |
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
| MAC-X17 | Control sizes | mini / small / regular / large (11+) / extraLarge (26, enum case confirmed; not probed, `probe.toml:604`): every control's height, padding, radius, indicator size, font size (`systemFontSize(for:)`, probed `probe.toml:20-24`), scroller width, switch size, spinner size; on 26 `prefersCompactControlSizeMetrics` restores pre-26 metrics — a third metric set. | differs | R: probe:controls per size (mini–large exist); add extraLarge and a `prefersCompactControlSizeMetrics = true` pass on 26 | one size per field (review 2026-10-28: `probe.toml:606` corrected to :604) |
| MAC-X18 | Typography | `Font` lacks: line height (the four `[text_scale]` roles carry one, `line_height_pt` / `_px`, and `defaults.line_height` is a factor; a widget's `Font` has none), letter spacing (SF Pro tracking per size — HIG Typography tracking table), optical size, width (`NSFont.Width`, 13+), system design (rounded / serif / monospaced, `NSFontDescriptor.SystemDesign`), features (monospaced digits — `monospacedDigitSystemFont`, probed `probe.toml:172`), emphasized weights; text antialiasing (font smoothing setting). | both | P: HIG typography JSON (tracking values); R: NSFont / NSFontDescriptor; font smoothing user default `AppleFontSmoothing` (to verify) | `Font` (family, size, weight, style, color); `TextScaleEntry.line_height_*` (review 2026-10-28: "line height per role" was a partial false gap, the text-scale roles carry one) |
| MAC-X19 | Icon tint per state | Template images and SF Symbols take the label colour at rest and change when pressed, disabled, on an emphasized selection, in a sidebar (accent), on accent fills. | both | M: renders with images, each state | `expander.arrow_color` only |
| MAC-X20 | Cursors | Which system cursor each element shows (I-beam text, pointing-hand link, resize for splitters / columns / window edges, drag copy / link, not-allowed) and the accessibility pointer size and colours (12+). | both | R: `linkTextAttributes[.cursor]`, `NSCursor` per control (to verify per control); pointer settings user defaults (to verify) | none |
| MAC-X21 | Drag and drop | Drag image translucency, item-count badge, drop-target highlights (MAC-L11, text drop caret), spring-loading flash. | both | M: recording during a programmatic drag (`beginDraggingSession`) | none |
| MAC-X22 | Increase contrast | Every colour's high-contrast value (the reader resolves aqua / darkAqua only); controls gain outlines, separators darken. Four variants (light, dark, HC light, HC dark) where the model has two. | both | R: resolve NSColor under `accessibilityHighContrastAqua` / `…DarkAqua`; M: `increase-contrast-on` pass (exists) | `AccessibilityPreferences.high_contrast` (flag only) |
| MAC-X23 | Reduce transparency | Opaque values for every material / glass surface and the translucent values with it off (the runners have it on; `reduce-transparency-off` pass exists). | differs | M: both passes | `reduce_transparency` flag |
| MAC-X24 | Differentiate without colour | Flag missing from `AccessibilityPreferences` (platform-facts §1.1.7 sources it; the reader does not read it); what changes visibly (to verify). | both | R: `NSWorkspace.accessibilityDisplayShouldDifferentiateWithoutColor` (probed: `probe.toml:467`, run 40348854007) | none |
| MAC-X25 | Reduce motion variants | Flag exists; each motion's reduced form (cross-fade instead of zoom / slide) has no data (MAC-X11). | both | M: recordings with Reduce Motion on (runner default) | `reduce_motion` flag |
| MAC-X26 | Other accessibility | Invert colours (`accessibilityDisplayShouldInvertColors`; OS-applied, images excluded via `accessibilityIgnoresInvertColors`); larger text (14+ Text size setting, no public API — to verify); "Show toolbar button shapes" (bezels on toolbar items; API / key to verify); pointer size (MAC-X20). The "Invert colors mode" (Classic / Smart), colour filters, display contrast and Night Shift: MAC-P4-3†. | differs | R / M as noted | `text_scaling_factor` (review 2026-10-28: pointer to MAC-P4-3† for the invert mode and the whole-screen filters) |
| MAC-X27 | Appearance-level variants | Wallpaper tinting (`tinting-off` pass exists; user default `AppleReduceDesktopTinting` to verify), multicolour (app) accent, highlight colour (already read via `selectedTextBackgroundColor`). | both | R / M | light / dark only |
| MAC-X28 | Text marks | Spelling (red dotted) and grammar (green dotted) underlines, marked-text (IME) underline, inline predictions (14+; the same ghost text as MAC-I06), Writing Tools glow (15.1+), text highlight styles (15+, to verify). | differs | M: key-window text view with misspelt text; P: docs (to verify names) | none (review 2026-10-28: inline predictions cross-referenced to MAC-I06, which holds the same property) |
| MAC-X29 | State combinations | Checked + hover / pressed / disabled / non-key, selected + unfocused / disabled, default + non-key / pressed, focused + hover: most have no field (rows above); the model's state set is per-widget and flat. | both | M: as per row | per-widget `*_background` |
| MAC-X30† | System effects | `NSColor.withSystemEffect(_:)` (10.14) derives any colour's `rollover`, `pressed`, `deepPressed` and `disabled` forms (`NSColor.SystemEffect`) — AppKit's own hover / pressed / disabled derivation, readable for each read colour; the reader reads none and the model has no rule for it. | both | R: `withSystemEffect` per effect (docs read 2026-10-28) | per-widget `hover_*`, `active_*`, `disabled_opacity` |
| MAC-X31† | Accessibility display options not in MAC-X22–X26 | The Full Keyboard Access focus indicator of the Accessibility settings (a highlight the OS draws around the focused element, with contrast and colour options — names to verify), distinct from AppKit's focus ring (MAC-X13); "Show window title icons" (proxy icons always shown, MAC-W03; the name is that of Apple Support *Change Display settings for accessibility on Mac* (`https://support.apple.com/guide/mac-help/unac089/26/mac`, 26 edition, read 2026-10-28): "Show a window's icon (if available) in the window's title bar"). | both (to verify per version) | R: user defaults (keys to verify); M: a pass with each setting on | `AccessibilityPreferences` (review 2026-10-28: the setting's name confirmed) |
| MAC-X32† | Motions MAC-X11 does not list | Traffic-light glyphs appearing on hover, the full-screen title-bar reveal (MAC-W12), window tiling (MAC-W13), the light / dark appearance switch, scroll momentum and its deceleration, SwiftUI `NavigationStack` push / pop, window-tab open / close / reorder (MAC-W09), symbol effects (MAC-IC05): whether each animates, then duration and curve. The accent or highlight colour change: MAC-P4-9†; a smooth mouse-wheel step: MAC-P4-4†. | differs | M: recording; CA introspection where layer-backed (in the Reduce Motion off pass) | none (review 2026-10-28: pointers to MAC-P4-9†, MAC-P4-4†) |
| MAC-X33† | Screen safe area | On displays with a camera housing (12.0+): `NSScreen.safeAreaInsets`, `auxiliaryTopLeftArea` / `auxiliaryTopRightArea`, the menu bar beside the housing (its height there: to verify), full-screen content kept below it, the `NSPrefersDisplaySafeAreaCompatibilityMode` Info.plist key. | 12+ (hardware-dependent) | R: the `NSScreen` properties (docs read 2026-10-28); the runners have no such display (values from a maintainer's machine — to verify) | none |
| MAC-X34† | Right-to-left layout | `NSApplication.userInterfaceLayoutDirection` / `NSView.userInterfaceLayoutDirection`: which parts mirror in a right-to-left language (control parts, sidebar side, disclosure triangles, toolbar, title bar, traffic lights) — to verify per part. | both | R: the two properties (docs read 2026-10-28); M: a probe pass in a right-to-left language (`-AppleLanguages`, to verify) | none |
| MAC-X35† | Design compatibility mode | `UIDesignRequiresCompatibility` (Info.plist, macOS 26.0: the app is displayed "as it looks when built against previous versions of the SDKs"; ignored when built for macOS 27 or later): on 26 an app may draw the pre-26 look while the reader assumes the running OS's. Related: `prefersCompactControlSizeMetrics` (MAC-X17). | 26 only | P: docs (read 2026-10-28); R: the app's own Info.plist key | none |
| MAC-X36† | Toolkit layers | What an app draws depends on its toolkit: AppKit; SwiftUI (mostly AppKit controls, some its own: MAC-N21–N23, N35–N37); Mac Catalyst in the iPad or the Mac idiom (what each scales and redraws: to verify). The model has one macOS look. | both | P: Mac Catalyst docs (to verify); M: a Catalyst probe (feasibility to verify) | none |
| MAC-X37† | Versions inside a generation | The 12–15 preset is one look while rows differ by version (13: colour-well styles MAC-N05, `NSComboButton` MAC-N28; 14: menu section headers and badges MAC-MN07–MN08, quinary colours MAC-DF01, `NSTextInsertionIndicator` MAC-X14; 15: tiling layouts MAC-W13; 15.1: Writing Tools MAC-X28), and 26 and 27 differ (27: menu item images MAC-MN10, `NSRefreshController` MAC-N24, MAC-N34). | differs | R: the OS version (the reader's `os_version()`) | one preset per generation |
| MAC-X38† | Hairlines | Whether separators, grid lines and outlines are one point or one device pixel on a 2x display (to verify). | both (to verify) | M: the probe's `-regular.png` and `-regular@2x.png` renders of the same control, compared | `Border.line_width` |

---

## Checked and not counted

- In-window status bar: macOS has none (platform-facts §2.14); the bottom bar is MAC-ST01.
- Visited link: macOS draws none (§2.28).
- Separator: no gap beyond MAC-X08.
- Text input error / invalid state: AppKit draws no standard invalid look (to verify; count it if a render shows one).
- Haptic feedback (Force Touch alignment): not visible.
- `NSForm` / `NSMatrix`: deprecated, drawn as the controls they contain.
- `NSStackView` / `NSGridView`: layout only (covered by `[layout]`).

## Counts (pass 1)

- 4.1: 173 rows, 171 gaps (MAC-SE01 records no gap; MAC-SL08 points to MAC-N15).
- 4.2: 26 rows.
- 4.3: 29 rows.

## Cross-check (pass 2, 2026-10-28)

Pass 2 checked this file against the other five surveys (`all-tags.tsv`) and the macOS "(none)"
cells of `platform-facts.md`. Its rows carry a "†" and sit at the end of their tables: §4.1
27 rows, §4.2 11 rows, §4.3 9 rows, 47 in all. APIs it cites were confirmed on 2026-10-28 from
Apple's documentation JSON (as at the top of this file). The control catalogue it uses for
decision 3 is AppKit's "Views and Controls" topic page
(`https://developer.apple.com/tutorials/data/documentation/appkit/views-and-controls.json`, read
2026-10-28) together with the HIG component pages it names.

### A. Controls other surveys list

Decision: 1 = macOS draws it as a standalone control (a §4.2 row); 2 = macOS draws it inside
an element that has rows; 3 = macOS has no such control.

| Control | Decision | Rows, or reason and source |
|---|---|---|
| icon button | 2 | Image-only buttons are bezel styles and borderless buttons (MAC-B01, MAC-B09); toolbar items (MAC-TL02); repeat timing (MAC-X12, `getPeriodicDelay`). |
| navigation/app bar | 2 | Title bar and toolbar: MAC-W01–W04, MAC-W10, MAC-TL01–TL09; added MAC-W11†, MAC-W14†; navigation push motion in MAC-X32†. |
| banner/inline message | 3 | No inline message control in the AppKit catalogue; notifications are OS-drawn (MAC-N25). |
| calendar | 2 | `NSDatePicker` graphical style (MAC-N04; HIG Pickers, macOS: "two styles … textual and graphical"). |
| page control/pager | 3 | HIG Page controls: "Not supported in macOS." `NSPageController` drives swipe navigation and draws no indicator. |
| sheet/bottom sheet | 2 | Window sheets: MAC-D03; macOS has no bottom sheet. |
| avatar | 3 | No avatar control in the AppKit catalogue or SwiftUI. |
| button group/toggle group | 2 | `NSSegmentedControl` select-any (MAC-SG05); SwiftUI `ControlGroup` (12.0) in toolbars groups items like MAC-TL03 (its drawing: to verify). |
| chip | 2 | Input chips are `NSTokenField` tokens (MAC-N03); filter and action chips have no AppKit control (catalogue). |
| group box/frame | 2 | `NSBox` (MAC-BX01, MAC-BX02); SwiftUI `GroupBox` draws a box (to verify against `NSBox`). |
| split button | 1 | MAC-N28† (`NSComboButton.Style.split`, 13.0). |
| status page/placeholder | 1 | MAC-N35† (`ContentUnavailableView`, 14.0). |
| about dialog | 1 | MAC-N32† (standard About panel). |
| carousel | 3 | No carousel in the AppKit catalogue; `NSPageController` draws no chrome. |
| colour picker | 2 | The colour panel is an OS standard panel (MAC-N25); the colour well and its 13+ expanded popover (MAC-N05). |
| fab | 3 | No floating action button in the AppKit catalogue or the HIG Buttons page. |
| menu button | 1 | MAC-N29† (`NSComboButton` unified, `NSMenuToolbarItem`); the pull-down pop-up is MAC-CB01. |
| navigation drawer/pane | 2 | The sidebar: MAC-SB01–SB08; added MAC-SB09† (width). |
| navigation rail | 3 | No rail control; the nearest are the sidebar and the Settings-style toolbar (MAC-TL11†). |
| password field | 2 | `NSSecureTextField` (MAC-I03); added MAC-I09† (AutoFill look). |
| rating | 2 | `NSLevelIndicator` rating style (MAC-N06; HIG Rating indicators: "No additional considerations for macOS"). |
| time picker | 2 | `NSDatePicker` time elements, text field with stepper and clock face (MAC-N04). |
| view switcher | 2 | A segmented control in the toolbar (MAC-SG01–SG10), the Settings-style toolbar (MAC-TL11†), window tabs (MAC-W09). |
| access key badge | 3 | macOS draws no access-key or mnemonic marks; keyboard navigation shows focus (MAC-X13, MAC-X31†). Source: AppKit catalogue (no such element); HIG Keyboards page to verify. |
| action sheet | 2 | SwiftUI `confirmationDialog` (13.0; HIG Action sheets: "No additional considerations for macOS") presents as an alert or sheet (to verify): MAC-D01, MAC-D03. |
| autocomplete | 2 | `NSComboBox` inline completion (MAC-CB01), text completions (MAC-I06), search suggestions (MAC-P3-7†; MAC-N02 holds only the search field's recents menu). (review 2026-10-28: suggestions pointed to MAC-P3-7†, as MAC-N02's row and MAC-P3-7† state) |
| back button | 2 | A toolbar navigation item (MAC-TL02; SwiftUI `NavigationStack` back item: to verify). |
| background extension | 2 | MAC-X06. |
| bottom accessory | 3 | iOS tab-bar accessory; macOS has no bottom tab bar (availability of the SwiftUI API on macOS: to verify). |
| context menu preview | 3 | HIG Context menus: the macOS section adds no preview; macOS context menus are menus (MAC-MN01–MN17†); the target outline is MAC-L19†. |
| dock/panel title bar | 3 | No dockable panes in the AppKit catalogue; floating panels have utility title bars (MAC-N16). |
| drag and drop | 2 | MAC-X21, MAC-L11. |
| edit menu | 2 | HIG Edit menus, macOS: editing commands are in a context menu and the menu bar's Edit menu — menu rows (MAC-MN01–MN17†, MAC-N20). |
| file chooser | 2 | Open and save panels are OS standard panels (MAC-N25). |
| font chooser | 2 | The font panel is an OS standard panel (MAC-N25). |
| full-screen dialog | 3 | No full-screen dialog; a window in full screen is MAC-W12†. |
| glass container | 2 | MAC-X04 (`NSGlassEffectView`, `NSGlassEffectContainerView`). |
| ink toolbar | 3 | No ink toolbar in the AppKit catalogue (a PencilKit tool picker on macOS: to verify). |
| keyboard | 3 | HIG Virtual keyboards: "Not supported in macOS." The OS's text input UI is MAC-N27†. |
| large content viewer hud | 3 | No such app-level element; the OS's Zoom "Hover Text" overlay is an accessibility feature of the OS (to verify). |
| list box | 2 | `NSTableView` (MAC-L01–L19†). |
| loading indicator | 2 | `NSProgressIndicator` spinning style (MAC-SN01–SN05†). |
| media transport controls | 1 | MAC-N30† (`AVPlayerView`). |
| paste control | 2 | SwiftUI `PasteButton` (macOS 10.15+) — a push button (MAC-B01; its drawing: to verify). |
| picker wheel | 3 | HIG Pickers, macOS: date pickers are textual and graphical only. |
| rubber band | 1 | MAC-N31†. |
| scrim | 3 | No modal scrim on macOS (whether a sheet dims its parent is open in MAC-D03). |
| scroll edge effect | 2 | MAC-X05. |
| scrollbar annotation | 3 | No such element in the AppKit catalogue. |
| swipe actions | 2 | MAC-L12. |
| teaching tip | 1 | MAC-N36† (TipKit, 14.0). |
| tool box | 3 | No tool box in the AppKit catalogue (tab view and disclosure instead). |
| tool button | 3 | GNOME's row is touch selection handles and a magnifier: macOS has no touch text selection. |
| toolbar flyout | 3 | No command-bar flyout in the AppKit catalogue; overflow is MAC-TL06. |
| tree view | 2 | `NSOutlineView` (MAC-L10, MAC-SB08); grids are collection views (MAC-N13). |
| writing tools | 2 | The Writing Tools glow (MAC-X28); its panel is OS-drawn (MAC-N25). |
| zoom button | 3 | No semantic-zoom control in the AppKit catalogue. |

Tally over the 57 controls: 1 — 7 (rows MAC-N28†–N32†, N35†, N36†); 2 — 29; 3 — 21.

### B. Capabilities

| Capability | Rows added, or "no further row" |
|---|---|
| C01 Shadow geometry | No further row: MAC-X01 lists every surface; the new surfaces name theirs (MAC-T06†, MAC-N34†). |
| C02 Elevation | No further row: macOS has no named elevation ramp; shadows are per surface (MAC-X01). |
| C03 Materials | MAC-N30† (player bar), MAC-N34†; the rest is MAC-X02–X06. |
| C04 Colour composition | No further row: the reader keeps `NSColor` alpha (`native-theme/src/macos.rs:48`); blending over materials is MAC-X03. |
| C05 Colour roles | MAC-DF05†. |
| C06 Variant axes | No further row: non-key, increased contrast and vibrant appearances are MAC-X16, X22, X03; iOS's elevated level and dimmed tint have no AppKit counterpart found. |
| C07 States | MAC-L17†, MAC-L19†, MAC-X30†. |
| C08 State layer | MAC-X30† (AppKit's own hover / pressed / disabled derivation). |
| C09 Variants | MAC-W14†, MAC-N28†, MAC-N29†, MAC-X35†, MAC-X36†. |
| C10 Size variants | MAC-B11†, MAC-SB09†; control sizes and compact metrics are MAC-X17; macOS has no touch sizing. |
| C11 Shape | MAC-L19†; the rest is MAC-X07. |
| C12 Borders | MAC-TB06†, MAC-P05†, MAC-X38†. |
| C13 Parts | MAC-W11†, MAC-T06†, MAC-TB06†, MAC-TB07†, MAC-L18†, MAC-SG10†, MAC-EX05†, MAC-IC04†. |
| C14 Motion | MAC-X32†, MAC-SN05†, MAC-IC05†. |
| C15 Focus | MAC-X31†. |
| C16 Caret | No further row: MAC-X14 and MAC-X12 hold width, shape and blink; macOS has no overwrite caret. |
| C17 Selection | MAC-L19†, MAC-N31†. |
| C18 Typography | MAC-TS03†. |
| C19 Icons | MAC-IC04†, MAC-IC05†, MAC-SG10†. |
| C20 Window chrome | MAC-W11†–W15†. |
| C21 Timings | MAC-T06†; the rest is MAC-X12. |
| C22 Pointer | No further row: MAC-X20 holds cursors and the pointer size and colours; macOS has no pointer hover effects like iPadOS. |
| C23 Accessibility | MAC-X31†; colour filters, display contrast and reduce white point are applied by the OS to the whole screen (like invert, MAC-X26) and get no row. Pass 4 gave colour filters, display contrast, the invert mode and Night Shift a row: MAC-P4-3†; reduce white point is not on AS-display's 26 edition (AS-display: see *Pass 4*) (review 2026-10-28: pointer to MAC-P4-3†). |
| C24 Controls | MAC-N27†–N37†. |
| C25 Reader coverage | MAC-DF05†, MAC-X30†, MAC-X33†, MAC-X35†, MAC-X37†. |
| C26 Layout | MAC-LY02†, MAC-TL10†, MAC-SB09†, MAC-X33†, MAC-X34†. |
| C27 Gradient paint | No further row: MAC-X09 and the 12–15 bezel rows (MAC-B05, MAC-C05). |
| C28 Colour precision | No further row: MAC-X10 (P3, HDR) and the HDR colour well (MAC-N05). |

### C. The macOS "(none)" cells of `platform-facts.md`

a = macOS does not draw it, the cell is right; b = macOS draws it and no document states it.

| platform-facts line | Property | a/b | Row, or reason |
|---|---|---|---|
| 1138 | icon `large` | a | No AppKit "large icon" context; the contexts macOS draws are MAC-IC01. |
| 1139 | icon `dialog` | b | MAC-IC04†. |
| 1140 | icon `panel` | a | No panel in an app; the Dock is OS-drawn (MAC-N25), status-item icons are MAC-N20. |
| 1147 | `icon_theme` | a | SF Symbols, no theme to choose (the cell's own source). |
| 1167 | `inactive_title_bar_text_color` | b | MAC-W15†. |
| 1170, 1171 | window `border.padding_*` | a | The window draws no padding; content margins are layout (MAC-LY01). |
| 1182 | button `min_width` | b | MAC-B11†. |
| 1194, 1195 | button `checked_background`, `checked_text_color` | b | MAC-B10 (its new render, state on). |
| 1216 | input `border.shadow_enabled` | a | No drop shadow; the 26 outer ring is MAC-I04. |
| 1236 | checkbox `border.shadow_enabled` | a | The 26 render draws nothing outside the frame (the cell); the 12–15 claim is checked on the existing render's alpha (MAC-X01's method). |
| 1237, 1238 | checkbox `border.padding_*` | b | MAC-C06 (mark inset in the box). |
| 1242 | `check_mark_stroke_width` | b | MAC-C06. |
| 1263, 1264 | menu `border.color`, `line_width` | b | MAC-MN14. |
| 1265 | menu `border.corner_radius` | b | MAC-MN01 (the cell's "items are rectangular" is what MAC-MN01 doubts). |
| 1266 | menu `border.shadow_enabled` | b | MAC-MN14, MAC-X01. |
| 1336 | progress `min_width` | a | AppKit imposes none. |
| 1338 | progress `border.shadow_enabled` | a | Measured none on 26 (the cell); none on 12–15. |
| 1339, 1340 | progress `border.color`, `line_width` | b | MAC-P05†. |
| 1350 | tab `min_width` | a | Sizes to its label; truncation is MAC-TB07†. |
| 1354–1356 | tab `border.*` | b | MAC-TB06†. |
| 1360 | tab `item_gap` | a | Tabs are joined in one strip; their separators are MAC-TB06†. |
| 1361–1363 | tab `active_indicator_*` | a | No indicator line; the selected tab is filled (MAC-TB06†). |
| 1377, 1378 | sidebar `border.corner_radius`, `shadow_enabled` | a (12–15) / b (26) | 12–15: a rectangular pane; 26: the floating sidebar, MAC-SB06. |
| 1412–1415 | status bar `border.*` | a | No in-window status bar (the cell's sources); the bottom bar is MAC-ST01. |
| 1461, 1462 | popover `border.padding_*` | a | Content-defined; content under the arrow is MAC-PO01. |
| 1504 | `container_margin` | b | MAC-LY01. |
| 1552–1555 | dialog `min_*` / `max_*` | b | MAC-D01 (alert width and frame from `NSAlert.layout()`). |
| 1601 | spinner `stroke_width` | a | Fins, not a stroke (MAC-SN01). |
| 1617 | combo `min_width` | a | Sizes to its widest item. |
| 1622 | combo `border.shadow_enabled` | b | MAC-X01 lists a 12–15 pop-up shadow while the cell says none for 12–15. Settled on the existing 14 render (run 40348854007, `renders/aqua/pop_up_button-regular@2x.png`): below the bezel the centre column's alpha falls off in the same sequence as the push button's (`push_button-regular@2x.png`), whose fall-off platform-facts §1.1.4 calls a soft shadow; the cell's 12–15 "no shadow" is a value for §7. (review 2026-10-28: "to verify" settled) |
| 1644 | segmented `border.shadow_enabled` | b | MAC-SG02: the selected segment's 12–15 shadow shows in the key window, which the not-key render cannot show. |
| 1675 | expander `header_height` | b | MAC-EX05†. |
| 1677, 1678 | expander `border.padding_*` | a | App-defined. |
| 1679–1682 | expander `border.*` | a | The disclosure control draws no frame (26: to verify). |
| 1684 | `arrow_gap` | b | MAC-EX05†. |
| 1685 | `content_indent` | b | MAC-EX05†. |
| 1686 | `frame_enabled` | a | No frame (as 1679–1682). |
| 1695 | `visited_text_color` | a | macOS draws no visited link (§2.28, "Checked and not counted"). |
| 1697 | link `background_color` | a | Inline text, no fill. |
| 1698 | link `hover_background` | a | No hover fill; the pointer changes (MAC-LK02). |
| 1714, 1715 | text area `border.padding_*` | b | MAC-I10†. |

Tally over the 63 lines: a 32, b 29, and 2 split by generation (1377, 1378: a on 12–15, b on 26).

## Seed follow-up (2026-10-28)

Rows for the four seed groups no row covered after pass 2 (G12, G19, G24, G27; the main
map's §5). The source abbreviations are defined in
`todo_v0.7.1_model-completeness-gap-map-seeds.md`, *Seed follow-up sources*. CAPS and SRC
are the row's capability tags (main map §4.3) and source kinds.

### Code editor and rich text (G12, G27)

**Code editor: not on this platform.** AppKit has no code-editor control and ships no syntax
scheme: Apple's own WWDC26 session on TextKit builds line numbers and collapsible sections as **app
code** on a `UITextView`/`NSTextView` subclass ("we added line numbers to a UITextView with just a
few lines of code"; the numbers are drawn by the app with `number.draw(at:withAttributes:)`), and
the APIs it introduces ("Starting with our 2027 releases, UITextView and NSTextView now conform to
NSTextViewportLayoutControllerDelegate") only let the app draw its own (WWDC26-370). The ruler
(`NSRulerView`) is MAC-N17; the find highlight, the find-indicator bubble and the dimming of
non-matches are MAC-X15. What AppKit itself draws for a code view is only the invisibles:

| ID | § | Model element | Element | Gen | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|---|
| MAC-SF01† | 4.1 | `[text_area]` | Invisibles (TextKit 1) | both | `NSLayoutManager.showsInvisibleCharacters` substitutes "visible glyphs for whitespace and other typically invisible characters": which glyph for space, tab, line break and no-break space, and their colour. Whether a TextKit 2 text view (the default since macOS 13, per Apple's TextKit 2 docs, to verify) draws them at all is to verify | P: Apple `appkit/nslayoutmanager/showsinvisiblecharacters` (macOS 10.7+); M: probe render of an `NSTextView` forced to TextKit 1 with the flag on, light and dark | none | C13, C17 | P, M, V |

**Rich text / Markdown.**

| ID | § | Model element | Element | Gen | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|---|
| MAC-SF02† | 4.2 | rich text view | SwiftUI `Text` with Markdown: which roles it draws | both (macOS 12+) | `Text` styles inline Markdown only: "It doesn't support line breaks, soft breaks, or any style of paragraph- or block-based formatting like lists, block quotes, code blocks, or tables"; so on macOS the list markers, quotes, code blocks, tables and headings of the seed have **no system look**, while emphasis, strong, strikethrough, links and inline code do | P: Apple `swiftui/text/init(_:tablename:bundle:comment:)`; parsing: `foundation/attributedstring/init(markdown:options:baseurl:)-52n3u` (macOS 12.0) | none | C24 | P |
| MAC-SF03† | 4.2 | rich text view | Inline code in `Text` (`InlinePresentationIntent.code`, "a code voice presentation") | both | The look of code voice: font (monospaced design or a fixed-pitch family), size relative to the run, colour, any fill. Apple states the intent, not its rendering | P: Apple `foundation/inlinepresentationintent/code` (macOS 12.0); M: SwiftUI probe render of `Text("a `code` b")`, light/dark, key/non-key | `defaults.mono_font` (a family, not the inline role) | C18, C24 | P, M |
| MAC-SF04† | 4.2 | rich text view | Links in `Text` Markdown | both | Link colour (accent/tint or `linkColor`, to verify), underline or none, pressed look; whether it follows `.tint` | M: SwiftUI probe render of `Text("[a](https://example.com)")` with and without `.tint`, key/non-key | `defaults.link_color`, `link.underline_enabled` | C07, C24 | M, V |
| MAC-SF05† | 4.1 | `[text_area]` | `NSTextView` showing an `AttributedString(markdown:)` | both | Whether AppKit renders the presentation intents (emphasis, strong, code, links) it is given or ignores them; if it renders them, with which fonts and colours. Apple documents only the parsing | P: Apple `foundation/instantiating-attributed-strings-with-markdown-syntax`; M: probe render; V | none | C18 | M, V |
| MAC-SF06† | 4.2 | rich text view | HTML imported into an attributed string (`NSAttributedString.DocumentType.html`) | both | The default look the importer gives `h1`–`h6`, `code`/`pre`, `blockquote`, list markers, tables and links (WebKit's defaults, applied in process); Apple discourages the synchronous importer. Scope (an app-chosen import path, not a control) is for the rationale | P: Apple `foundation/nsattributedstring/documenttype/html` (macOS 10.0+); M: probe render of a fixed HTML sample | none | C18, C24 | P, M |

macOS 26 adds a rich-text `TextEditor(text: Binding<AttributedString>)` (26.0) whose unstyled runs
take "the font and/or foreground color inherited from the environment" (Apple
`swiftui/texteditor/init(text:selection:)`): no role look of its own, no row.

### Drop zones and dialog placement (G19, G24)

#### G19 — drop zone under a dragged pane, tab or dock

Covered by: MAC-L11 (drop-on row and insertion line in lists and outlines), MAC-X21 (drag image, count badge,
drop-target highlights, spring-loading flash), MAC-W13† (the 15+ drag-to-edge tiling preview; drawn by the
window server, not the app, so §6.2's scope question), MAC-SP03 (a split-view divider while dragged).

Not on this platform: a pane or dock dragged onto another pane. AppKit has no docking control:
`NSSplitView` / `NSSplitViewController` resize their arranged panes and offer no drag-to-rearrange (to verify
against the AppKit class index); SwiftUI's `NavigationSplitView` and `HSplitView` likewise (to verify).

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| MAC-SF07† | 4.2 | window tab bar | Window tab bar as a drop target | While a window tab, or a whole window, is dragged over another window's tab bar: the insertion feedback (the tabs parting to open a gap, or a highlight; which one is to verify), its motion, and the look of the bar accepting a merge. MAC-W09 lists only the dragged tab's own look. **Gen:** both draw it; 26 to verify. | M: two windows joined with `addTabbedWindow(_:ordered:)`, a programmatic or recorded drag of a third window's tab over the bar, screen recording at the display rate | none (`tab.*` is a tab view's) | C24, C17, C07, C14 | M, V |

#### G24 — where a modal dialog is placed

Covered by: MAC-D01 (alert layout, not its position), MAC-D03 (sheet radius, material, shadow, parent dim; 26's
floating inset sheet as a look). Neither states *where* the sheet or the alert sits.

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| MAC-SF08† | 4.1 | [dialog] | Sheet: attachment point | Where a sheet (`beginSheet(_:completionHandler:)`, `NSAlert.beginSheetModal(for:)`, the open/save panels as sheets) sits on its window: the documented default attaches the sheet's top edge at the top of the content view ("the default `origin.y` value is the height of the content view and the default `origin.x` value is 0"); whether it then sits below the title bar only or below the toolbar, per `NSWindow.ToolbarStyle` and with a full-size content view, and whether it is centred horizontally, are to measure. The app can move it per window; the HIG says the macOS sheet "floats on top of its parent window". **Gen:** differs: 12–15 attach the sheet to the window's top; 26's floating inset sheet (MAC-D03) has an inset from the top and sides to measure. | P: Apple `appkit/nswindowdelegate/window(_:willpositionsheet:using:)` (discussion, read 2026-10-28); HIG *Sheets* §macOS (read 2026-10-27); R: the delegate's `rect` argument gives the default per window; M: sheet frame vs window frame on the probe window, without toolbar and with each toolbar style, 12–15 and 26 | none | C26, C25 | P, R, M |
| MAC-SF09† | 4.1 | [dialog] | App-modal alert / panel position on screen | Where an app-modal alert or panel opens when it is not a sheet (`NSAlert.runModal()`, `NSApplication.runModal(for:)`): `NSWindow.center()` places a window "exactly in the center horizontally and somewhat above center vertically", and the same page says `runModal(for:)` invokes it for panels; the "somewhat above" offset is stated by no document; whether `NSAlert.runModal()` goes through `center()`: to verify. **Gen:** to verify on 26. | P: Apple `appkit/nswindow/center()` (read 2026-10-28), `appkit/nsalert/runmodal()`; M: alert window frame vs the screen's visible frame after `runModal` starts (probe, timer-dismissed) | none | C26 | P, M, V |
| MAC-SF10† | 4.1 | [dialog] | SwiftUI alert, confirmation dialog and sheet on macOS | Whether SwiftUI's `.alert`, `.confirmationDialog` and `.sheet` present as window sheets (and so take the sheet row above) or as separate windows centred like `center()`; `presentationSizing` (15+) for sheets. **Gen:** both; 26 to verify. | P: SwiftUI documentation for the three modifiers (to verify); M: probe with a SwiftUI window | none | C26 | M, V |

---

## Relabelled controls (2026-10-28)

Pass 2 asked the platforms about 18 controls under a wrong control name (a tagging
error: GN-2.40–2.46, GN-2.49, K2.7, K2.15–2.17, K2.22, Q2.43, W2-13, W2-14, W2-19,
W2-25), so this check asked again for this platform. Decisions: 1 a new §4.2 row, 2 a
new §4.1 row, 3 a row already covers it (the same job, possibly done another way),
4 the platform has no such thing.

Apple pages read 2026-10-28 through their JSON form (`https://developer.apple.com/tutorials/data/documentation/<path>.json`), as the macOS file does; "AppKit catalogue" = the "Views and Controls" topic page the macOS file's Cross-check uses (`appkit/views-and-controls`); HIG pages at `https://developer.apple.com/design/human-interface-guidelines/<page>` (read 2026-10-27; review 2026-10-28: was a pointer to a local cache).

| Control | Decision 1–4 | Row added, row covering it, or the source showing it is absent |
|---|---|---|
| keyboard shortcut keycap (GN-2.40) | 3 | MAC-MN04 (the menu's key-equivalent column: shortcut text, modifier glyphs, colours per state); the Help-menu search that finds commands is MAC-N33†. macOS draws shortcuts only as menu text: no keycap view in the AppKit catalogue |
| shortcuts dialog (GN-2.41) | 4 | No view or panel that lists an app's shortcuts in the AppKit catalogue; macOS shows shortcuts beside each menu item (MAC-MN04) and finds commands through Help-menu search (MAC-N33†) |
| text selection handles (GN-2.42) | 4 | macOS has no touch text selection: the system selection UI `UITextSelectionDisplayInteraction` lists iOS, iPadOS, Mac Catalyst, tvOS and visionOS, not macOS (`uikit/uitextselectiondisplayinteraction`), and the AppKit catalogue has no handle or loupe. What a Mac Catalyst app draws is MAC-X36† |
| emoji chooser (GN-2.43) | 3 | MAC-N27† (the Character Viewer / emoji picker the OS draws beside a text field) |
| video play overlay (GN-2.44) | 3 | MAC-N30† (`AVPlayerView` controls: play / pause, the inline bar and the floating controls over the video). Whether `AVPlayerView` also draws a centred play image while paused: to verify in MAC-N30†'s capture |
| editable label (GN-2.45) | 3 | MAC-I01 (the borderless label field; the field editor inside a table cell), MAC-L13 (cell editing: the field editor's fill and focus ring); renaming from the title bar is MAC-W11† |
| wrap box (GN-2.46) | 2 | new 4.1 row: Flow layout spacing. Tokens wrapping inside a token field are MAC-N03 |
| drag icon (GN-2.49) | 3 | MAC-X21 (drag image translucency, item-count badge); the dragged column's image MAC-L15. The multi-item formation (`NSDraggingSession.draggingFormation`, `appkit/nsdraggingsession/draggingformation`) is not named in MAC-X21: it belongs to that row's recording |
| command link button (K2.7) | 4 | No title-plus-description button: the `NSButton.BezelStyle` cases (`appkit/nsbutton/bezelstyle-swift.enum`, listed in MAC-B01) and the AppKit catalogue have none |
| title widget (K2.15) | 3 | MAC-TS01 (the title text styles the model lacks: title2, title3, …); a settings section header is MAC-N21. macOS has no title component with an icon and a message-coloured comment (AppKit catalogue) |
| side tab bar (K2.16) | 3 | MAC-TB03 (`NSTabView.tabPosition` left / right, `tabViewType`). Tool-view toggle strips along a window edge (Xcode's) are app-drawn, not in the AppKit catalogue |
| splitter collapser (K2.17) | 3 | HIG Sidebars: "in macOS, you can include a show/hide button or add Show Sidebar and Hide Sidebar commands to your app's View menu"; the button is the toolbar item `NSToolbarItem.Identifier.toggleSidebar` (10.11) / `toggleInspector` (14.0) (`appkit/nstoolbaritem/identifier/togglesidebar`, `…/toggleinspector`), drawn as a toolbar item: MAC-TL02. `NSSplitView` draws no button on its divider (MAC-SP01); a pane collapses by dragging (`NSSplitViewItem.canCollapse`) |
| wizard (K2.22) | 4 | No wizard or assistant class in the AppKit catalogue. HIG Sheets places multi-step flows (Back, Cancel, Done) in a sheet, with button placement stated for iOS and iPadOS only; on macOS such a flow is a sheet (MAC-D03) with push buttons (MAC-B01) |
| drag handle (Q2.43) | 4 | No reorder grip in AppKit: the reorder control `UITableViewCell.showsReorderControl` lists iOS, iPadOS, Mac Catalyst, tvOS and visionOS, not macOS (`uikit/uitableviewcell/showsreordercontrol`); `NSTableView` rows are dragged by their body, with the drop line of MAC-L11. HIG Sheets, macOS: "a sheet is a cardlike view with rounded corners that floats on top of its parent window" — no grabber |
| grid view items (W2-13) | 3 | MAC-N13 (`NSCollectionView` item selection highlight); band selection MAC-N31†. AppKit has no check-box corner on items. Doubt: `NSCollectionViewItem.isSelected` states no default drawing, so whether the item draws any selection itself is to verify in MAC-N13 |
| items view (W2-14) | 3 | MAC-L02 (selection shape and inset of table rows), MAC-L03 (unemphasized selection), MAC-N13 (collection items) |
| repeat button (W2-19) | 3 | MAC-X12 (button periodic delay, `NSButtonCell.getPeriodicDelay(_:interval:)`; `NSButton.setPeriodicDelay(_:interval:)` "for a continuous button", `appkit/nsbutton/setperiodicdelay(_:interval:)`), MAC-N01 (stepper autorepeat); the button itself is drawn as a push button (MAC-B01) |
| radio group (W2-25) | 2 | new 4.1 row: Radio group |

Tally: 1 — 0; 2 — 2 (wrap box, radio group); 3 — 11; 4 — 5.

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| MAC-RL01† | 4.1 | `[layout]` | Flow layout spacing | The spacing a flow layout uses when the app sets none: `NSCollectionViewFlowLayout.minimumInteritemSpacing` (between items in a row) and `minimumLineSpacing` (between rows), whose defaults Apple publishes on each property's page; a flow of tags or chips laid out by it draws these gaps. The model has one `widget_gap_px` for every gap. A built-in SwiftUI flow container was not found (custom `Layout`, 13.0): to verify | P: `appkit/nscollectionviewflowlayout/minimuminteritemspacing`, `…/minimumlinespacing` (read 2026-10-28) | `layout.widget_gap_px` · generations: both (the pages are unversioned; 26 to verify) | C26 | P |
| MAC-RL02† | 4.1 | `[checkbox]` | Radio group | A group of radios: the gap between radios stacked vertically (the default) and placed horizontally (`horizontalRadioGroupLayout()`), the group label's placement beside the column of radios (in a form: MAC-LY02†), per control size. SwiftUI's `RadioGroupPickerStyle` (10.15, "presents the options as a group of radio buttons") draws the group itself; AppKit radios are separate `NSButton`s sharing an action, placed by the app (`NSMatrix`, its `intercellSpacing`, is legacy, "discouraged … in macOS 10.8 and later"). HIG Toggles: "Use consistent spacing when you display radio buttons horizontally" | P: `swiftui/radiogrouppickerstyle`, `swiftui/view/horizontalradiogrouplayout()`, `appkit/nsmatrix` (read 2026-10-28), HIG Toggles "Radio buttons"; M: a SwiftUI host (as MAC-N21) with a vertical and a horizontal radio-group picker per control size, gaps measured on the capture | `checkbox.label_gap_px`, `layout.widget_gap_px` · generations: both (to verify) | C13, C26, C10 | P, M |

## Fonts and mirroring (2026-10-28)

A completeness check of the map found two gaps no row covered: the emoji font and
the fallback chain for scripts the UI font lacks (all six platforms), and right-to-left
layout where this file had no general row. The source abbreviations are defined in
`todo_v0.7.1_model-completeness-gap-map-seeds.md`, *Fonts and mirroring sources*.

Not covered by any row (see the top). The two rows differ nowhere between 12–15 and 26 in mechanism:
the same Core Text API and the same emoji font name serve both; whether a cascade list's members differ
between versions is what the read returns per version. `NSAdaptiveImageGlyph` (macOS 15.0, "an emoji-like
image that can appear in attributed text", `CT`) is user content placed in text, not a platform value,
so it gets no row.

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| MAC-FM01† | 4.3 | typography | Emoji font | The colour-emoji font text is drawn in: Apple Color Emoji, a system font on macOS (`ASF`), reached through Core Text's fallback, not named by the app. The model has no emoji family, so a connector draws emoji in whatever its toolkit bundles. Both generations (same font name; the glyph set per OS version is the font's). | R: `CTFontCreateForString` / `CTFontCreateForStringWithLanguage` on an emoji string returns the font Core Text substitutes (`CT`); P: `ASF`; M: OS probe, the sample set in an `NSTextField`, the font of each `CTRun` | none (`defaults.font.family` is one name) | C18, C25 | R, P, M |
| MAC-FM02† | 4.3 | typography | Fallback chain per language | The ordered fallback list for characters the UI font lacks (CJK, Arabic, Hebrew, Indic, Thai, symbols), which depends on the user's preferred languages: Han characters take a Japanese, Simplified Chinese, Traditional Chinese or Korean font by language. The model has one family and no language. Both generations (same API; members per version as read). | R: `CTFontCopyDefaultCascadeListForLanguages(font, languages)` (macOS 10.8), `NSFontDescriptor.AttributeName.cascadeList` (default "the system default cascading list for user's locale") (`CT`); M: OS probe, the sample set under each preferred language (`-AppleLanguages`, to verify as in MAC-X34†) | none (`defaults.font.family`, `defaults.mono_font.family`: one name each) | C18, C25 | R, M |

## Pass 3 (2026-10-28)

Pass 3 asked macOS each of the 249 rows the other five platforms added after pass 1 (IDs ending
in "†"; pass 2, *Seed follow-up*, *Relabelled controls*, *Fonts and mirroring*). Decisions: 1 macOS
draws it and no row covered it, a row is added below; 2 a row of this file (or a model field the
reader already fills) covers it; 3 macOS does not draw it, with the source. A group with two
decisions names which part takes which. Pass 2's decisions are cited, not repeated. Apple pages were
read on 2026-10-28 through their JSON form, as at the top of this file. New abbreviation: **AS** =
Apple Support, *Mac User Guide*, read 2026-10-28: AS-appearance
`https://support.apple.com/guide/mac-help/mchlp1225` (versions 12.0 and 26: "Click in the scroll bar
to"), AS-pointer `…/mac-help/mchlp2920` (12.0, 15.0 and 26: "Shake mouse pointer to locate", pointer
size, outline and fill colour), AS-audio `…/mac-help/unac611` (12.0 and 26: "Flash the screen when an
alert sound occurs"), AS-dock `…/mac-help/mchlp1119` (26: "Animate opening applications").

| Input rows | Decision 1–3 | Row added, row covering it, or the source showing it is absent |
|---|---|---|
| GN-1.282† | 1 | MAC-P3-1† (resize zone). |
| MAT-W4†, W1-148† | 2 | MAC-W01–W07 (title bar, traffic lights with their states, title, separator, corners, outline, shadow); the button set per window kind: MAC-W02 (a disabled zoom button), MAC-N16 (utility panel), MAC-N14 (HUD). |
| MAT-W5† | 2; 3 | Window radius per window kind: MAC-W05. Display corners: hardware; `NSScreen`'s topics have no corner radius (the only corner-related ones are `auxiliaryTopLeftArea` / `auxiliaryTopRightArea`, MAC-X33†). |
| W-10† | 2; 3 | The divider between two full-screen windows in Split View: MAC-W12†. Slide Over: none; HIG *Multitasking*: "On Mac, multitasking is the default experience … switching between windows". |
| W-11†, GN-3.71†, K3.39† | 2 | MAC-W12† (full screen), MAC-W13† (tiling), MAC-X11 (open / close, minimise, full-screen enter / exit, resize), MAC-X32†. |
| K1.19† | 2 | MAC-W13† (the 15+ drag-to-edge tiling preview). |
| GN-1.278†, C-43† | 2 | Back button: a toolbar item (MAC-TL02; pass 2, "back button"), its push / pop motion MAC-X32†. Navigation item styles: title placement MAC-W03, title-bar variants MAC-W14†; SwiftUI `ToolbarRole` exists on macOS 13.0, but its page gives only an iPadOS effect ("a browser will automatically leading align the title … in iPadOS"): its macOS effect is to verify in MAC-W03's captures. |
| GN-3.72†, K3.15† | 2 | MAC-D03 ("whether the parent dims"). |
| K3.16†, K3.66†, W1-149† | 1 | MAC-P3-15† (Dock bounce for attention and launch, spinning wait cursor). |
| W3-53† | 2; 1 | Reduce Motion MAC-X25; the minimise effect MAC-X11; the launch bounce MAC-P3-15†. |
| GN-1.281†, K3.48†, MAT-I18†, W1-151†, K1.49† | 2 | MAC-X28 (spelling and grammar underlines, the marked-text underline); the candidate window MAC-N27†. A run-time source for MAC-X28's marked text: `NSTextView.markedTextAttributes` ("Text color, background color, and underline are the only supported attributes for marked text"). |
| GN-2.58†, I-9†, K1.48†, MAT-I19† | 2 | MAC-I03 (bullet glyph), MAC-I09† (strong-password look). `NSSecureTextFieldCell.echosBullets` (default true) is AppKit's only echo setting: no last-character reveal, reveal delay or show-passwords setting is documented. |
| I-10† | 2 | Model `input.disabled_background`, `disabled_text_color`; read-only MAC-I02; the focused field MAC-X13; height per control size MAC-X17; larger text MAC-X26. |
| K1.46† | 2 | MAC-N02 (the search field's cancel button); `NSTextField`'s topics have no clear button. |
| K1.47† | 3 | No overwrite caret (pass 2, C16); `NSTextField`'s topics have no input mask. |
| W1-150† | 3 | `NSTextField`'s topics have placeholder strings and no description part below the field; a grouped form's section footer is MAC-N21. |
| MAT-I17†, GN-1.286†, Q2.53†, MAT-N30† | 3; 1 | No touch text selection, magnifier or selection toolbar: *Relabelled controls*, "text selection handles"; editing commands are a context menu and the Edit menu (HIG *Edit menus*, macOS; pass 2, "edit menu"). The Writing Tools affordance a 15.4+ text field can show: MAC-P3-8†. |
| GN-3.73† | 1 | MAC-P3-11† (caret visibility); the drop caret is named in MAC-X21. |
| K1.58† | 2 | MAC-C06. |
| CK-2† | 3 | `UITableView.allowsMultipleSelectionDuringEditing` ("check marks appear next to selected rows in editing mode") lists iOS, iPadOS, Mac Catalyst, tvOS and visionOS, not macOS; `NSTableView` marks selection by its highlight (MAC-L02). |
| GN-1.285†, MN-17†, MN-18† | 2 | MAC-MN10 (item icons; 27 hides them by default), MAC-MN01 (highlight shape), MAC-MN02 (menu padding), model `menu.icon_text_gap_px`, `menu.disabled_text_color`; disabled subtitle and shortcut colours MAC-MN09, MAC-MN04. |
| B-18† | 2 | MAC-N29† (menu buttons, the pull-down indicator), MAC-CB01, MAC-MN15 (the menu's position at the control). |
| GN-1.283† | 2 | Model `tooltip.max_width_px`, a pixel width (platform-facts.md@fc6109d1:1285: measured on Sonoma, 14, and stated for the 12–15 column; ❓ on 26). (review 2026-10-28: the cell's measurement is Sonoma's, not 12–15's) |
| K1.83†, TT-3† | 2 | MAC-T03 (position), MAC-T04 (re-show, auto-hide, fade), MAC-T05 (show delay), `tooltip.max_width_px`. |
| K1.84† | 3 | `NSView.toolTip`: "The text for the view's tooltip", a string; the expansion tooltip is MAC-T06†. |
| SB-7†, MAT-SC3† | 1; 2 | MAC-P3-3† (overlay knob minimum length, its shortening in an overscroll). Fade delay and motion MAC-X12, MAC-X11; knob inset MAC-S04; hover MAC-S01, MAC-S02. |
| MAT-SC4† | 2 | MAC-X11 (elastic overscroll and bounce-back). |
| W1-152† | 2 | Read: `scrollbar.overlay_mode` comes from `NSScroller.preferredScrollerStyle` (`native-theme/src/macos.rs:243-248`). |
| GN-3.69†, K3.76† | 1; 2 | Track paging and its repeat: MAC-P3-2†. Stepper repeat MAC-N01, MAC-X12 (`getPeriodicDelay`). |
| MAT-X32†, W3-52† | 1; 2 | Scroll distances: MAC-P3-17†. Momentum MAC-X32†, overscroll MAC-X11, drag threshold MAC-X12. |
| X-BT-2† | 2 | MAC-X12 (double-click interval, drag threshold). |
| PB-4† | 1 | MAC-P3-4† (SwiftUI progress labels). |
| K1.115† | 2 | MAC-N06 (`NSLevelIndicator` capacity styles). |
| K1.133† | 2 | Window tabs: MAC-W09, MAC-SF07† (drop target), MAC-X32† (reorder motion), MAC-X12 (spring-loading delay). |
| T-17† | 2 | MAC-N23 (SwiftUI tab bar); `TabViewCustomization` (macOS 15.0): "On macOS, a default interaction is provided for reordering sections but not for controlling the visibility of individual tabs"; the drag's insertion line MAC-L11. |
| W-SF09† | 2 | MAC-SF07†. |
| SD-9† | 2 | Sidebar collapse motion MAC-X11; the 26 floating sidebar MAC-SB06. SwiftUI `NavigationSplitViewStyle.prominentDetail` (macOS 13.0: "attempts to maintain the size of the detail content when hiding or showing the leading columns"): whether macOS then draws the sidebar over the content with a shadow is to verify in MAC-X11's recording. |
| SD-10† | 2 | MAC-SB09†. |
| K1.142† | 2 | Section headers MAC-SB03, rows MAC-SB01; the open / save panels' sidebar MAC-N25. Capacity bars and eject buttons are Finder's, not a standard sidebar's. |
| GN-1.279† | 2 | Grid lines MAC-L06, indentation and disclosure MAC-L10, drop lines MAC-L11, source-list nesting MAC-SB08. Type-to-select: `NSTableView.allowsTypeSelect` (10.5, default true) lets "the user … type characters to select rows"; the page describes a selection, no search field. |
| GN-1.280† | 2 | MAC-N13 (item selection), MAC-RL01† (flow-layout spacing), MAC-X21 (drop highlights); the inter-item gap indicator (`NSCollectionView.elementKindInterItemGapIndicator`, 10.11): whether a default one is drawn is to verify in MAC-X21. |
| L-21† | 2 | MAC-L10. |
| L-22† | 2 | MAC-L05 (header parts, sort indicator); the compact-width collapse is iOS's. |
| L-23†, Q2.55†, Q2.56†, W2-40† | 2 | MAC-N21 (grouped form), MAC-LY02† (form label alignment), MAC-EX05† (disclosure rows). |
| K1.184† | 2 | MAC-N08 (`NSBrowser` branch arrows). |
| K1.185† | 2 | MAC-X11 (outline expand / collapse). |
| K-RL05†, W-RL01† | 2 | MAC-L13 (cell editing), MAC-I01 (field editor in a cell). |
| K-RL06†, W3-49† | 2 | MAC-X21 (drag image, count badge), MAC-L15 (column drag), MAC-X20 (drag copy / link cursors). |
| W1-159†, C-47† | 2 | MAC-N31†. |
| GN-3.66† | 2 | MAC-X21, MAC-L11. |
| PO-7† | 2 | MAC-PO01 (arrow and offset from the anchor); the content size is the app's (`NSPopover.contentSize`). |
| GN-1.284†, MAT-DG10† | 2 | MAC-D01 (button arrangement). `NSAlert.addButton(withTitle:)`: "Buttons are placed starting near the right side of the alert and going toward the left side (for languages that read left to right)". |
| DL-13† | 2 | MAC-D01 (alert width; whether long informative text scrolls: to verify there), MAC-X26 (larger text). |
| K1.224†, W1-156† | 2 | MAC-D01, MAC-D02 (critical badge), MAC-IC04† (alert icon size). |
| GN-SF17†–SF21†, K-SF19†–SF22†, MAT-SF01†, SF-7†, SF-8†, SF-9†, W-SF10†, W-SF11†, W-SF13† | 2 | MAC-SF08† (sheet attachment), MAC-SF09† (app-modal position, `center()`), MAC-SF10† (SwiftUI presentations, `presentationSizing`), MAC-D03. |
| K-SF23†, MAT-SF03† | 3 | No bottom sheet (pass 2, "sheet/bottom sheet"); window sheets are MAC-D03, MAC-SF08†. |
| Q2.58†, W1-155†, MAT-SF02† | 3 | No full-screen dialog (pass 2, "full-screen dialog"). |
| W-SF12† | 3 | HIG *Virtual keyboards*: "Not supported in macOS" (pass 2, "keyboard"). |
| GN-2.57† (composite dialogs) | 2; 3 | Print and page setup: the standard panels of MAC-N25 (it names print; `NSPageLayout` is the page-setup panel of the same kind). App chooser: no such panel in AppKit (`appkit/nspanel`, read 2026-10-28: "Inherited By" lists `NSColorPanel`, `NSFontPanel`, `NSSavePanel` only); Apple Support *Choose an app to open a file on Mac* (`https://support.apple.com/guide/mac-help/mh35597/mac`, read 2026-10-28) names only menus: "choose File > Open With, then choose an app", and the Get Info window's "Open with" pop-up menu (look: MAC-MN rows, MAC-MN15) (review 2026-10-28: the "to verify" settled). |
| GN-RL01†, W-RL03† | 3 | No wizard (*Relabelled controls*, "wizard"). |
| RL-1†, K-RL02† | 3 | No shortcuts dialog or HUD (*Relabelled controls*, "shortcuts dialog"); shortcuts are menu text (MAC-MN04). |
| K-RL07† | 2 | MAC-X12 (`NSButtonCell.getPeriodicDelay(_:interval:)`, the repeat of a continuous button); the stepper's repeat MAC-N01. |
| W-RL02† | 3 | No title-plus-description button (*Relabelled controls*, "command link button"). |
| SG-8† | 2 | MAC-SG05 (momentary mode), MAC-SG06 (pressed segment). |
| SG-9† | 1; 2 | Segment width rule: MAC-P3-5†. Height per control size MAC-X17; disabled segment MAC-SG06. |
| W1-154† | 2 | MAC-SG04 (styles). |
| W1-153† | 3 | `NSSlider` holds one value (`init(value:minValue:maxValue:target:action:)`, one knob); no range slider in the AppKit catalogue. |
| W1-157† | 2 | MAC-SP01–SP03. |
| W1-158† | 2 | MAC-ST01. |
| K1.260† | 2 | MAC-LK01 (underline), MAC-LK02 (cursor); no hover fill (pass 2, C, line 1698). |
| MAT-TA8† | 2 | MAC-TL06. |
| K3.38† | 1 | MAC-P3-6† (toolbar customization and item re-layout motion). |
| GN-2.52†, K2.27† | 2 | MAC-N07 (`NSPathControl`); the open / save panels MAC-N25. |
| GN-2.53†, K2.25†, W2-42† | 2 | The colour panel MAC-N25; the colour well and its 13+ styles MAC-N05. |
| GN-2.54†, C-42† | 2; 1 | MAC-N30† (`AVPlayerView` controls). The AirPlay route button and the Picture in Picture window: MAC-P3-9†. |
| GN-2.55†, C-48† | 3; 1 | No carousel indicator (pass 2, "carousel", "page control"). The page transitions `NSPageController` animates: MAC-P3-10†. |
| GN-2.56†, K2.29†, C-38† | 2; 1 | Completions MAC-I06, the combo box list MAC-CB04 (row count `numberOfVisibleItems`). Suggestion menus of a text or search field (AppKit 15.0, SwiftUI 13.0): MAC-P3-7†. |
| K2.23†, K2.24† | 2 | MAC-N04 (`NSDatePicker`: calendar overlay, today marker, time elements). |
| K2.26†, C-39† | 2; 3 | Open / save panels MAC-N25. `SFSafariViewController` lists iOS, iPadOS, Mac Catalyst and visionOS, not macOS. |
| K2.28† | 2 | The font panel, MAC-N25. |
| K2.30† | 2 | MAC-N18 (find bar), MAC-X15 (find highlight, find indicator). |
| K2.31†, C-36† | 2 | MAC-N36† (TipKit); help tags are the tooltips, MAC-T01–T06†. |
| K2.32† | 2 | MAC-N17 (`NSRulerView`). |
| Q2.54† | 2 | MAC-N24 (27). |
| Q2.57† | 2 | MAC-N32†. |
| MAT-N31† | 3 | HIG *Pickers*, macOS: textual and graphical styles only (pass 2, "picker wheel"). |
| MAT-RL01† | 2 | MAC-RL01† (flow-layout spacing), MAC-N03 (tokens). |
| W2-41† | 2 | MAC-N22 (SwiftUI `Gauge`, circular). |
| W2-43† | 2 | MAC-N03. |
| C-37† | 2 | MAC-N20. |
| C-40† | 3 | `PKToolPicker` lists iOS, iPadOS, Mac Catalyst and visionOS, and "doesn't display in Mac apps built with Mac Catalyst". This settles pass 2's "ink toolbar … to verify". |
| C-41† | 2 | MAC-N19. |
| C-44† (document launch view) | 3 | `DocumentGroupLaunchScene` lists iOS, iPadOS, Mac Catalyst and visionOS, not macOS. |
| C-45† (chart) | 2 | MAC-N37†. |
| C-46† (activity rings) | 3 | `HKActivityRingView` lists iOS, iPadOS, Mac Catalyst and visionOS, not macOS. |
| C-49† (editing HUD) | 3 | `UIResponder.editingInteractionConfiguration` lists iOS, iPadOS, Mac Catalyst, tvOS and visionOS, not macOS; what a Catalyst app draws is MAC-X36†. |
| K-RL01† (command bar) | 2 | MAC-N33† (Help-menu search, which finds menu commands); keycaps are menu text, MAC-MN04 (*Relabelled controls*, "keyboard shortcut keycap"). |
| K-RL03†, K-RL04† (character selector) | 2 | MAC-N27† (the Character Viewer / emoji picker). |
| text input system UI (brief's list) | 2 | MAC-N27†. |
| window tab bar (brief's list) | 2 | MAC-W09, MAC-SF07†. |
| RL-2† | 2 | MAC-W11† (renaming from the document title menu). |
| RL-3† | 2 | MAC-RL01†. |
| GN-SF01†, GN-SF02†, GN-SF04†, GN-SF05†, GN-SF07†, GN-SF09†–SF12†, K-SF01†–SF05†, K-SF07†, K-SF09†, K-SF10† | 3 | No code-editor control or syntax scheme in AppKit (*Seed follow-up*, "Code editor: not on this platform", WWDC26-370: line numbers are app code). |
| GN-SF03† | 2 | MAC-X14 (caret); whether AppKit draws a second caret in mixed-direction text is to verify there. |
| GN-SF06†, K-SF06† | 2; 3 | Invisibles MAC-SF01†; indentation guides: none (as the code editor). |
| GN-SF08†, K-SF08† | 2 | MAC-X15 (find highlight, find indicator). |
| GN-SF13† | 2 | MAC-I06 (completions list). |
| K-SF11† | 2; 3 | Spelling underline MAC-X28; template placeholders: none (as the code editor). |
| GN-SF14†–SF16†, K-SF18† | 3 | No docking control (*Seed follow-up*, G19, "Not on this platform"). |
| K-SF12†–SF17†, W-SF01†–SF08†, SF-1†–SF-6† | 2 | MAC-SF01†–SF06†: SwiftUI `Text` draws inline Markdown only, so headings, quotes, lists, tables and rules have no system look (MAC-SF02†); inline code MAC-SF03†; links MAC-SF04†; `NSTextView` MAC-SF05†; the HTML importer's look MAC-SF06†; the text view's frame (W-SF08†) MAC-I08. |
| GN-3.67†, K3.65†, MAT-X33† | 2; 1 | Cursor per element, pointer size and colours: MAC-X20. Pointer hidden while typing: MAC-P3-12†. |
| GN-3.68† | 2 | MAC-X12 (`keyRepeatDelay`, `keyRepeatInterval`). |
| GN-3.70†, K3.95† | 1 | MAC-P3-13† (screen flash for alert sounds). |
| K3.96†, W3-50† | 1; 2; 3 | Shake to locate: MAC-P3-14†. Pointer size, outline and fill colours: MAC-X20. Trails, shadow and touch visualisation: AS-pointer lists no such option. |
| K3.84† | 1 | MAC-P3-16† (display scale). |
| X-MO-11† | 2 | MAC-X32† (the light / dark appearance switch). |
| X-FO-3†, MAT-X34† | 2 | MAC-X13 (focus ring, the Full Keyboard Access rule), MAC-X31† (the Accessibility focus indicator). |
| MAT-X35†, W3-56† | 2; 3 | Increase contrast MAC-X22, invert colours MAC-X26. Colour filters and correction: applied by the OS to the whole screen, no row (pass 2, C23); pass 4 gave them one, MAC-P4-3† (review 2026-10-28: pointer to MAC-P4-3†). |
| MAT-X36†, W3-47† | 2 | Read: `defaults.mono_font` from `NSFont.monospacedSystemFont` (`native-theme/src/macos.rs:365`), the system font into `defaults.font`; the text area's user fonts MAC-I07. |
| MAT-X37† | 3 | HIG *Icons* puts optical centring into the icon asset ("you can optically center the icon by geometrically centering the asset" that includes the adjustment as padding): a property of the image, not a layout rule of the system. |
| MAT-X38† | 3 | No floating action button and no snackbar (pass 2, "fab", "banner/inline message"). |
| MAT-SF04† | 2 | The 26 floating sidebar and inspector: MAC-SB06, MAC-N19. |
| W3-45† | 2 | MAC-DF02 (the alpha fill ramp); the reader keeps `NSColor` alpha (pass 2, C04). |
| W3-46† | 2 | Read: `alternateSelectedControlTextColor` (`native-theme/src/macos.rs:119`); the colours not read are MAC-DF05†. |
| W3-48† | 3 | No access-key marks (pass 2, "access key badge"). |
| W3-51† | 2 | MAC-X19 (icon tint per state, disabled). |
| W3-54†, GN-FM03†, K-FM03†, MAT-FM03†, MAT-FM04† | 2 | MAC-X34† (which parts mirror, the symbols included). |
| W3-55† | 3 | No handedness setting found: a menu opens where `NSMenu.popUp(positioning:at:in:)` places it (MAC-MN15); the absence of a system setting is to verify. |
| GN-FM01†, GN-FM02†, FM-1†, FM-2†, K-FM01†, K-FM02†, MAT-FM01†, MAT-FM02†, W-FM01†, W-FM02† | 2 | MAC-FM01† (emoji font), MAC-FM02† (fallback chain). |

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| MAC-P3-1† | 4.1 | `[window]` | Resize zone | The band where the pointer resizes a window: its width along each edge and at each corner, whether it reaches outside the visible frame into the shadow, and which resize cursor shows where (the cursor kinds are MAC-X20). No Apple page states it. **Gen:** both; whether 26 differs is to verify. | M: in the probe's key window, `CGEvent` mouse moves at stepped offsets inside and outside each edge and corner, reading `NSCursor.currentSystem` (10.6, deprecated in 27.0; page read 2026-10-28) at each, or a press-and-drag at each offset and the window frame compared | none | C20, C22, C26 | M, V |
| MAC-P3-2† | 4.1 | `[scrollbar]` | Track click and its repeat | The user's "Click in the scroll bar to": "Jump to the next page" or "Jump to the spot that's clicked" (AS-appearance): what a click in the track does and, while the track is held, the first delay and interval of the paging repeat. **Gen:** both (the setting is on the 12.0 and the 26 page). | P: AS-appearance; R: the setting's user default (key to verify); M: recording of a held click in a legacy scroller's track | none | C21, C25 | P, R, M, V |
| MAC-P3-3† | 4.1 | `[scrollbar]` | Overlay knob length | `min_thumb_length` is "40 (measured) legacy mode" and ❓ on 26 (platform-facts.md@fc6109d1:1308): the overlay knob's minimum is not stated, and the overlay knob shortens while content rubber-bands past its end (to verify) — a length one field cannot hold. **Gen:** both; 26 to verify. | M: new `NSScroller` render with a very long document, overlay and legacy; recording of an elastic overscroll | `scrollbar.min_thumb_length_px` | C10, C14 | M, V |
| MAC-P3-4† | 4.1 | `[progress_bar]` | SwiftUI progress labels | SwiftUI `ProgressView(value:total:label:currentValueLabel:)` (macOS 11.0) draws a label and a current-value label with the bar: their fonts, colours, placement and gaps; `NSProgressIndicator` has no label. Whether the `.circular` style draws them too: to verify. **Gen:** both (11+). | P: `swiftui/progressview`, `…/init(value:total:label:currentvaluelabel:)` (read 2026-10-28); M: SwiftUI host (as MAC-N21), capture per style | none | C13, C18, C26 | P, M |
| MAC-P3-5† | 4.1 | `[segmented_control]` | Segment width rule | `NSSegmentedControl.segmentDistribution` (10.13): `fit` ("sized to fit their contents"), `fill` ("extra space being distributed equally among them. Default value."), `fillEqually`, `fillProportionally`; the padding a segment adds to its content, and how segments compress when the control is too narrow (`activeCompressionOptions`). The model has no width rule. **Gen:** both. | P: `appkit/nssegmentedcontrol/distribution` (read 2026-10-28); M: segment frames in a new render per distribution | none | C26, C10 | P, M |
| MAC-P3-6† | 4.1 | `[toolbar]` | Customization palette | `NSToolbar.allowsUserCustomization` (default false) enables "Customize Toolbar…", which "lets people change the items on the toolbar, rearrange their positions, and change the toolbar's display mode": the palette (item grid, default set, display-mode choice), the items parting while one is dragged in, and the re-layout motion when items are added, removed or move to the overflow (MAC-TL06). **Gen:** differs (26 glass groups, MAC-TL03; to verify). | P: `appkit/nstoolbar/allowsusercustomization`, `…/runcustomizationpalette(_:)` (read 2026-10-28); M: `runCustomizationPalette(_:)` on the probe's key window, capture; recording | none | C13, C14, C26 | P, M |
| MAC-P3-7† | 4.1 | `[input]` | Suggestion menus | AppKit text-field suggestions (macOS 15.0: `NSTextField.suggestionsDelegate`; `NSSuggestionItem`, "the items that appear in suggestion menus"; `NSSuggestionItemSection`, "a section of suggestions items in a suggestions menu") and SwiftUI `searchSuggestions(_:)` (macOS 13.0: "presents the suggestion views as a list of choices"): placement under the field, width, section-title font, item font, highlight, and whether they are drawn as menus (MAC-MN01–MN17†). MAC-N02 lists only the search field's recents menu. **Gen:** 13+ (SwiftUI), 15+ (AppKit); 26 to verify. | P: the four pages (read 2026-10-28); M: key-window field with a suggestions delegate, typed text (`CGEvent`), capture | `menu.*` | C13, C26 | P, M |
| MAC-P3-8† | 4.1 | `[input]` | Writing Tools affordance | `NSTextField.allowsWritingToolsAffordance` (macOS 15.4; its page states no behaviour): what the affordance draws, where (beside the selection or the field) and when — to verify. MAC-X28 holds the Writing Tools glow, MAC-N25 its panel. **Gen:** 15.4+ and 26 (needs Apple Intelligence; feasibility on a runner to verify). | P: `appkit/nstextfield/allowswritingtoolsaffordance` (read 2026-10-28); M: key-window text field with selected text, capture | none | C13, C19 | P, M, V |
| MAC-P3-9† | 4.2 | AirPlay route picker | `AVRoutePickerView`; Picture in Picture | `AVRoutePickerView` (macOS 10.15) "represents a button that users tap to stream audio/video content"; then "the system presents a popover that displays all of the nearby AirPlay devices": the button's glyph, its colour per state (`routePickerButtonColor(for:)`, `activeTintColor`), its border (`isRoutePickerButtonBordered`), the device popover. Also the system's Picture in Picture window for an `AVPlayerView` (`allowsPictureInPicturePlayback`, 10.15, default false), which MAC-N30† does not list. **Gen:** both (10.15+); 26 to verify. | P: `avkit/avroutepickerview`, `avkit/avplayerview/allowspictureinpictureplayback` (read 2026-10-28); R: `routePickerButtonColor(for:)` per state on a new view; M: render of the button; the popover and the PiP window captured (feasibility on a runner to verify) | none | C24, C13, C07 | P, R, M, V |
| MAC-P3-10† | 4.3 | motion | Page transitions | `NSPageController` (10.8) animates between pages: `transitionStyle` `.stackHistory` (the default: "Pages animate out to the right to reveal the previous page"), `.stackBook` ("Pages animate out to the left to reveal the next page"), `.horizontalStrip`: duration, curve, the look of the stacked pages (a shadow under the top page: to verify). Pass 2 counted its chrome (none, "carousel"); this is its motion. **Gen:** both. | P: `appkit/nspagecontroller/transitionstyle-swift.property`, `…/transitionstyle-swift.enum` (read 2026-10-28); M: recording of a programmatic `navigateForward(_:)` (whether it animates without a swipe: to verify) in the Reduce Motion off pass | none | C14, C01 | P, M, V |
| MAC-P3-11† | 4.3 | caret | Caret visibility | When the insertion point is drawn: not while text is selected, not in a view that is not first responder, not in a non-key window (each to verify), and the drop caret during a text drag (named in MAC-X21). `NSTextView.shouldDrawInsertionPoint` exists but states no rule. **Gen:** both; with 14+'s `NSTextInsertionIndicator` (MAC-X14) to verify. | P: `appkit/nstextview/shoulddrawinsertionpoint` (read 2026-10-28); M: key-window text view with a selection, with another first responder, and in the not-key pass | `input.caret_color` | C16 | P, M, V |
| MAC-P3-12† | 4.3 | pointer | Pointer hidden while typing | Whether the pointer hides while a person types and returns on a mouse move: AppKit's `NSCursor.setHiddenUntilMouseMoves(_:)` (10.0) does it; which controls call it (the text view: to verify). **Gen:** both. | P: `appkit/nscursor/sethiddenuntilmousemoves(_:)` (read 2026-10-28); M: typed text in the key window, a capture that includes the cursor (`screencapture -C`) | none | C22, C21 | P, M, V |
| MAC-P3-13† | 4.3 | accessibility | Screen flash for alert sounds | "Flash the screen when an alert sound occurs" (AS-audio): the OS flashes the screen on an alert sound — the flash's colour, extent and timing. Drawn by the OS, counted as MAC-N25 is. **Gen:** both (on the 12.0 and the 26 page). | P: AS-audio; R: the setting's user default (key to verify); M: recording after `NSSound.beep()` with the setting on | `AccessibilityPreferences` | C23, C21, C25 | P, R, M, V |
| MAC-P3-14† | 4.3 | accessibility | Shake to locate | "Shake mouse pointer to locate: … make the pointer larger when you quickly move your finger on the trackpad or quickly move the mouse" (AS-pointer): the enlarged pointer's size and its grow and shrink motion. The same page's pointer size and colours are MAC-X20. **Gen:** both (on the 12.0, 15.0 and 26 pages). | P: AS-pointer; R: the setting's user default (key to verify); M: recording of fast `CGEvent` pointer moves with the setting on (whether synthetic moves trigger it: to verify) | `AccessibilityPreferences` | C22, C23, C14, C25 | P, R, M, V |
| MAC-P3-15† | 4.3 | window chrome | Attention and busy feedback | Drawn by the Dock and the window server, not the app (as MAC-N25): the Dock icon bounce for `NSApplication.requestUserAttention(_:)` (critical: "The dock icon will bounce until either the app becomes active or the request is canceled"; informational: "The dock icon will bounce for one second"), the launch bounce ("Animate opening applications: Make icons bounce when you open apps", AS-dock), and the spinning wait cursor over an app that stops handling events (the delay before it shows: to measure). A counterpart to KWin's desaturated not-responding window was not found (to verify). **Gen:** both (26 to verify). | P: `appkit/nsapplication/requestuserattentiontype/criticalrequest`, `…/informationalrequest` (read 2026-10-28), AS-dock; M: recording of the Dock after the call from an inactive probe app, and of a probe app that blocks its main thread | none | C14, C20, C21, C22 | P, M |
| MAC-P3-16† | 4.3 | size variants | Display scale | `NSScreen.backingScaleFactor` (10.7): "the number of backing store pixels corresponding to each linear unit in screen space on this screen", per screen, so it changes when a window moves to another screen. The reader reads none; the device-pixel width of hairlines (MAC-X38†) and the pixel snapping of edges depend on it. Whether a scaled display resolution changes the factor or only the framebuffer: to verify. **Gen:** both. | R: `NSScreen.backingScaleFactor` (read 2026-10-28) | none | C10, C25 | R, V |
| MAC-P3-17† | 4.3 | timings | Scroll distances | The distance one scroll step moves: `NSScrollView.verticalLineScroll` / `horizontalLineScroll` (documented for clicks on the scroll arrows; whether a wheel step without precise deltas uses it: to verify), `verticalPageScroll` ("the amount of the document view kept visible when scrolling vertically page by page"); the user's scrolling-speed setting as applied to event deltas by the OS (to verify). Momentum is MAC-X32†. **Gen:** both. | R: the properties' values on a new `NSScrollView` (pages read 2026-10-28); M: `CGEvent` scroll-wheel events in line units on the probe's scroll view, the content offset read after each | none | C21, C25 | R, M, V |

Tally over the 249 input rows (a row in a group with two or three decisions counts once, under its
first): 1 — 19; 2 — 178; 3 — 52. Rows added: §4.1 8 (MAC-P3-1†–P3-8†), §4.2 1 (MAC-P3-9†), §4.3 8
(MAC-P3-10†–P3-17†), 17 in all. The brief's own list: chart 2 (MAC-N37†), activity rings 3, character
selector 2 (MAC-N27†), command bar 2 (MAC-N33†), composite dialogs 2 / 3, document launch view 3,
editing HUD 3, text input system UI 2 (MAC-N27†), window tab bar 2 (MAC-W09, MAC-SF07†).

## Pass 4 (2026-10-28)

The rows pass 3 and the review added were grouped into kinds and each kind put to
every platform; the coverage table is the main map's Appendix B. This section holds
this platform's rows for the kinds it had none for. Sources are cited in full in
each row.

Apple pages read 2026-10-28 through their JSON form (`https://developer.apple.com/tutorials/data/documentation/<path>.json`); HIG pages at `https://developer.apple.com/design/human-interface-guidelines/<page>`; the WebKit file at commit 1716af91a8cf9585adba395faeb98bc72e8bd1e9 (review 2026-10-28: was a pointer to a local download folder). Abbreviations as in the macOS file (AS = Apple Support *Mac User Guide*; *Relabelled controls*, *Seed follow-up*, pass 2 / pass 3 decision tables). New: **AS-display** = `https://support.apple.com/guide/mac-help/unac089/26/mac` ("Change Display settings for accessibility on Mac", 26 edition, read 2026-10-28); **AS-nightshift** = `https://support.apple.com/guide/mac-help/mchl97bc676d/mac` ("Shift to warmer colors on your Mac", current edition, read 2026-10-28).

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| MAC-P4-1† | 4.1 | `[window]` | Title pop-up menu | A Command-click or Control-click on a window's title can open "the title pop-up menu" (`NSWindowDelegate.window(_:shouldPopUpDocumentPathMenu:)`, 10.5: "Asks the delegate whether the window displays the title pop-up menu in response to a Command-click or Control-click on its title"). Missing: its items (the represented document's path, by the method's name: to verify), their icons, the menu's position against the title; it is drawn as an `NSMenu` (look: MAC-MN rows). Window commands are in the menu bar's Window menu (MAC-N20) and the zoom button's menu (MAC-W13†); MAC-W11† has the hover chevron and its popover, not this menu. **Gen:** both (10.5+); 26 to verify. | P: `appkit/nswindowdelegate/window(_:shouldpopupdocumentpathmenu:)` (read 2026-10-28); M: a key-window document window (`representedURL` set), a `CGEvent` Command-click on the title, capture of the menu | `menu.*` | C20, C13, C26 | P, M, V |
| MAC-P4-2† | 4.3 | window chrome | Behind an authentication prompt | The system's authentication dialog for `LAContext.evaluatePolicy(_:localizedReason:reply:)` (the app's name appears "in macOS, in the title of the dialog") and for Authorization Services rights (`AuthorizationFlags.interactionAllowed`: "the Security Server is permitted to interact with the user"): whether the screen or the asking window is dimmed or desaturated while it is up, and the dialog's own look, which no row has (OS-drawn, counted as MAC-N25 is). No Apple page read states either. **Gen:** to verify on 12–15 and 26. | P: `localauthentication/lacontext/evaluatepolicy(_:localizedreason:reply:)` (macOS 10.10), `security/authorizationflags/interactionallowed` (macOS 10.0) (read 2026-10-28); M: recording while a probe app calls `evaluatePolicy` with a password policy (whether a runner shows the prompt: to verify) | none | C06, C20, C14 | P, M, V |
| MAC-P4-3† | 4.3 | accessibility | Screen colour filters, display contrast, Night Shift | Applied by the OS to the whole screen, over every colour the model states: "Color filters" ("Adjust display colors using the selected filter type"; filter types "Grayscale, Red/Green (for protanopia), Green/Red (for deuteranopia)…", an "Intensity" slider, and for Color Tint a "Tint" colour well), "Display contrast" ("Drag the slider to increase or decrease the screen contrast"), "Invert colors mode" ("everywhere (Classic), or everywhere except images and video (Smart)"; MAC-X26 has the invert flag, not the mode) (AS-display); Night Shift, which can "make your display colors warmer at a particular time of day or at sunset", on a custom schedule, sunset to sunrise or manually (AS-nightshift; its colour temperature is on a page not read). Pass 2 (C23) gave these no row while invert has one (MAC-X26). A run-time read for any of them: none found among the pages read (to verify). **Gen:** both as far as read (the 26 edition of AS-display, the current edition of AS-nightshift); the 12–15 editions to verify. | P: AS-display, AS-nightshift (read 2026-10-28); R: keys to verify; M: probe captures with each filter on, compared to the plain capture (setting them on a runner: to verify) | `AccessibilityPreferences` (none of these) | C23, C06, C25 | P, M, V |
| MAC-P4-4† | 4.3 | motion | Smooth wheel step | Whether a discrete (line-unit) mouse-wheel step scrolls at once or animates: the undocumented user default `NSScrollAnimationEnabled`, which WebKit reads as its scroll-animator default on macOS; its default value, and the step's duration and curve when on, are stated nowhere read. Momentum after a trackpad flick is MAC-X32†, elastic overscroll MAC-X11, the step's distance MAC-P3-17†. **Gen:** both (per version: to verify). | R: user default `NSScrollAnimationEnabled` (undocumented; read by WebKit@26695efdefc6076049ec50713a2c4e54b90b4d47 `Source/WebKit/Shared/Cocoa/WebPreferencesDefaultValuesCocoa.mm:92-97`); M: `CGEvent` scroll-wheel events in line units on the probe's scroll view, the content offset sampled per frame (recording or CA introspection), in the Reduce Motion off pass | none | C14, C21, C25 | R, M, V |
| MAC-P4-5† | 4.1 | `[window]` | Window tab bar overflow | MAC-W09 lists the native window tab bar's parts, not what it draws when its tabs no longer fit the window's width (narrowed tabs, truncated titles, scrolling or an overflow control: to verify); a window takes further tabs through `addTabbedWindow(_:ordered:)` without a stated limit. **Gen:** to verify on both. | P: `appkit/nswindow/addtabbedwindow(_:ordered:)` (macOS 10.12, read 2026-10-28; the page states no limit); M: the probe window joined with many tabbed windows, captures at stepped window widths | none (`tab.*` is a tab view's) | C13, C26, C20 | P, M, V (review 2026-10-28: the page behind "without a stated limit" cited, P added) |
| MAC-P4-6† | 4.3 | timings | Double-click distance | AppKit states the double-click interval (`NSEvent.doubleClickInterval`, MAC-X12) and no distance: whether a second click far from the first still counts as a double-click, and from how far it does not, is stated nowhere read (whether any distance applies: to verify). The click count reaches the app as the event's click state (`CGEventField.mouseEventClickState`: "A click state of 2 represents a double-click"); whether a posted `CGEvent` gets that state from the window server or keeps the one its poster set (to verify) decides whether a runner can measure it. **Gen:** both (to verify). | P: `coregraphics/cgeventfield/mouseeventclickstate` (read 2026-10-28); M: two clicks within `doubleClickInterval` at stepped distances, the second event's click state read in the probe | none | C21, C25 | P, M, V |
| MAC-P4-7† | 4.3 | icons | Symbols per writing direction and per script | SF Symbols give "variants for the RTL context and localized symbols for Arabic and Hebrew, among other languages", and "different versions of the signature, rich-text, and I-beam pointer symbols for use with Latin, Hebrew, and Arabic text" (HIG *Right to left*); variants for "Latin, Arabic, Hebrew, Hindi, Thai, Chinese, Japanese, Korean, Cyrillic, Devanagari, and several Indic numeral systems" that "adapt automatically when the device language changes" (HIG *SF Symbols*); both pages state "No additional considerations for … macOS". The model maps an icon role to one symbol name, with no direction or script variant; MAC-X34† asks which parts mirror, not which symbol variant is drawn. **Gen:** same mechanism; the variant set grows with SF Symbols versions (per version: to verify). | P: HIG *Right to left* (`https://developer.apple.com/design/human-interface-guidelines/right-to-left`, read 2026-10-28), HIG *SF Symbols* (`https://developer.apple.com/design/human-interface-guidelines/sf-symbols`, read 2026-10-28: "SF Symbols provides many variants for specific languages and writing systems", and under *Platform considerations* "No additional considerations for iOS, iPadOS, macOS, tvOS, visionOS, or watchOS."); M: probe renders of direction- and script-sensitive symbols under a right-to-left and a Hindi preferred language (`-AppleLanguages`, to verify as in MAC-X34†) | `IconRole` (one name) | C19, C26 | P, M (review 2026-10-28: HIG *SF Symbols* cited by URL and quoted words, not a local cache) |
| MAC-P4-8† | 4.1 | `[combo_box]`, `[menu]` | Pull-down and combo list placement and width | `NSPopUpButton.preferredEdge` is "The edge of the button on which to display the menu when screen space is constrained"; "For pull-down menus, the default behavior is to position the menu under the button"; "For most pop-up menus, the `NSPopUpButton` object attempts to show the selected item directly over the button" (the pop-up case is MAC-MN15). Missing: the pull-down menu's offset under the button, the menu's width against the button's, where it goes when the preferred edge does not fit, and the `NSComboBox` list's placement and width against its field (MAC-CB04 has the list's rows). **Gen:** both; 26 to verify. | P: `appkit/nspopupbutton/preferrededge`, `appkit/nspopupbutton/pullsdown` (read 2026-10-28); M: captures with a pull-down menu and a combo box list open, mid-screen and near the screen's bottom edge | none | C26 | P, M |
| MAC-P4-9† | 4.3 | motion | Accent and highlight colour change | MAC-X32† asks whether the light / dark switch animates; the accent or highlight colour change is not named. AppKit posts `NSColor.systemColorsDidChangeNotification` ("Sent when the system colors have changed, such as through a system control panel interface"); whether the accent-coloured parts (default button, selection, switch, slider fill, focus ring) change at once or cross-fade: to verify. **Gen:** both (to verify). | P: `appkit/nscolor/systemcolorsdidchangenotification` (read 2026-10-28); M: recording of the key window while the accent colour changes (how a runner changes it while the probe runs: to verify; the `accent-red` pass sets it before launch) | none | C14 | P, M, V |
| MAC-P4-10† | 4.1 | `[menu]` | Submenu safe zone | When the pointer leaves an item whose submenu is open, "a triangular region between the current pointer position and the sub-menu bottom corner is defined"; "the user has about 1 second to move the pointer inside the triangle", after which a submenu the pointer has not reached closes; leaving the triangle closes it at once (a community description of Mac OS X in 2005). MAC-X12 has the submenu open delay only. Missing: the zone's shape and the time on current versions. **Gen:** to measure on 12–15 and 26. | P (community): Michel Fortin, "Mac Sub-Menus", 2005-06-12, `https://michelf.ca/blog/2005/mac-sub-menus/` (read 2026-10-28); M: a menu with an open submenu (MAC-MN06's capture), `CGEvent` pointer moves inside and outside the triangle with timed pauses, recording | none | C21, C26 | P, M, V |
| MAC-P4-11† | 4.2 | context help | Context-sensitive help window (`NSHelpManager`) | An app associates help with an object (`NSHelpManager.setContextHelp(_:for:)`: "When the application enters context-sensitive help mode, if `object` is clicked, `help` appears in the context-sensitive help window"); `showContextHelp(for:locationHint:)` "Displays the context-sensitive help for a given object at or near the point on the screen specified by a given point". Missing: the help window's fill, border, shadow, text font and colour, margins, its position against the point; the pointer while context-help mode is active (`isContextHelpModeActive`, to verify). Distinct from tooltips (MAC-T01–T06†), the help button (MAC-N10) and TipKit (MAC-N36†). **Gen:** both as far as read (the pages list macOS without deprecation); 26 to verify. | P: `appkit/nshelpmanager`, `…/setcontexthelp(_:for:)`, `…/showcontexthelp(for:locationhint:)`, `…/iscontexthelpmodeactive` (read 2026-10-28); M: probe sets context help on a control and calls `showContextHelp(for:locationHint:)`, capture | `tooltip.*` (nearest) | C24, C13, C26 | P, M, V |
| MAC-P4-12† | 4.2 | image cropper | ImageKit crop (`IKPictureTaker`, `IKImageView`) | `IKPictureTaker` (macOS 10.4) "represents a panel that allows users to choose images by browsing the file system. The picture taker panel provides an Open Recent menu, supports image cropping, and supports taking snapshots"; `IKImageView` (10.5) supports "rotating, zooming, and cropping", with the crop tool `IKToolModeCrop` set through `currentToolMode`. Missing: the crop rectangle's outline and handles, the shade outside it, their states, and the picture taker panel's layout. **Gen:** both as far as read (no deprecation on the pages); 26 to verify. | P: `quartz/ikpicturetaker`, `quartz/ikimageview`, `quartz/ikimageview/currenttoolmode`, `quartz/iktoolmodecrop`, `quartz/ikpicturetaker/begin(withdelegate:didend:contextinfo:)` (read 2026-10-28); M: new render of an `IKImageView` in the crop tool mode with a selection, capture of the picture taker panel | none (`card.*` nearest for the frame) | C24, C13, C04, C07 | P, M, V |
| MAC-P4-13† | 4.3 | states | Placeholder (redacted) content | SwiftUI `redacted(reason:)` with `.placeholder` (macOS 11.0): "Text and images will be automatically masked to appear as generic placeholders, though maintaining their original size and shape. Use this to create a placeholder UI without directly exposing placeholder data to users" — the look of content not loaded yet, list rows included. Missing: the mask's fill colour (light / dark), its corner radius, its shape per line of text, any animation. **Gen:** both (11+). | P: `swiftui/view/redacted(reason:)`, `swiftui/redactionreasons/placeholder` (read 2026-10-28); M: SwiftUI host (as MAC-N21), a list with redacted rows, capture light and dark | none | C07, C13, C04 | P, M |
