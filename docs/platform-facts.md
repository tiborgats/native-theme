# Platform Theme Facts & Cross-Platform Property Mapping

---

### Conventions

#### Value sources

Every value in this document has a deterministic source. When an OS
provides no direct API for a property, the value is measured from rendered
controls and the source is annotated:

| Source tag          | Meaning                                                    |
|---------------------|------------------------------------------------------------|
| **(API)**           | System API, function, or queryable constant                |
| **(HIG)**           | Apple Human Interface Guidelines documentation             |
| **(Fluent)**        | Microsoft Fluent Design System / WinUI3 XAML resources     |
| **(Breeze src)**    | KDE Breeze style engine source (`breezemetrics.h`, etc.)   |
| **(Adwaita CSS)**   | GNOME libadwaita stylesheet values                         |
| **(measured)**      | Pixel-measured from rendered controls; OS version noted     |
| **(font metrics)**  | Computed from OpenType font metric tables (sTypo/hhea)     |
| **(WebKit)**        | Inferred from WebKit's native look implementation          |
| **(cross-project)** | No platform API; value from cross-project consensus (Qt, Firefox, etc.) |
| **(preset)**        | Value from our bundled preset TOML (originally measured)   |
| **(none)**          | OS has no such concept; preset must supply a value         |

Properties annotated **(none)** are absent from the platform entirely —
no API, no guideline, no rendered control to measure. The preset supplies
a default for these.

#### Table symbols

| Symbol | Meaning                                                        |
|--------|----------------------------------------------------------------|
| `→`    | "returns / extract field" — the API on the left yields the value on the right. Example: `+systemFontOfSize:` → family means calling `+systemFontOfSize:` and reading its `family` property. |
| `←`    | "inherits from" — no widget-specific value; the property is taken from the referenced global default. Example: `← defaults.radius` means use the global default radius. |
| L / D  | Light variant / Dark variant hex values.                       |
| ⚙      | Value can be changed by the user via a system setting (theme, accent, font choice, etc.). Inherited (`←`) properties are not marked — follow the chain to the source. |
| ↕      | Value scales with the system DPI / display scaling factor.     |
| ✅     | Verified and correct — confirmed against authoritative sources.|
| ❓     | Uncertain — low confidence; measured or inferred, no authoritative source found. |
| ❌     | Incorrect — disproved; see inline note for the correct information. |

---

## Chapter 1: What OSes Provide

Pure facts — every API, setting, and value available from each platform.
No abstractions, no invented structures.

### 1.1 macOS

#### 1.1.1 Fonts

**NSFont role-based class methods** (each returns family, pointSize, weight):

| Class method                              | Role             | Default result              |   |
|-------------------------------------------|------------------|-----------------------------|---|
| `+systemFontOfSize:`                      | Body text        | SF Pro, 13pt, Regular (400) | ✅ |
| `+boldSystemFontOfSize:`                  | Bold body        | SF Pro, 13pt, Bold (700)    | ✅ |
| `+monospacedSystemFontOfSize:weight:`     | Monospace        | SF Mono, 13pt, Regular (400)| ✅ |
| `+titleBarFontOfSize:`                    | Window title bar | SF Pro, 13pt, Bold (700)    | ✅ |
| `+menuFontOfSize:`                        | Menu items       | SF Pro, 13pt, Regular (400) | ✅ |
| `+menuBarFontOfSize:`                     | Menu bar labels  | SF Pro, 13pt, Regular (400) | ❓ weight undocumented; no evidence of SemiBold; likely Regular like other font methods |
| `+toolTipsFontOfSize:`                    | Tooltip text     | SF Pro, 11pt, Regular (400) | ❓ Apple API docs do not state default size; however, Leopard-era HIG states "The small system font (11 point) is the default font for help tags" (Apple's term for tooltips), strongly supporting 11pt; Cocotron defaults to 10pt; GNUstep defaults to 12pt; open-source impls disagree with each other and with the HIG |
| `+paletteFontOfSize:`                     | Tool palettes    | SF Pro, 12pt, Regular (400) | ❓ size undocumented by Apple; both Cocotron and GNUstep default to 12pt — good corroboration but not authoritative |
| `+controlContentFontOfSize:`              | Buttons/controls | SF Pro, 13pt, Regular (400) | ✅ |

**NSFont size class properties:**

| Property             | Value |   |
|----------------------|-------|---|
| `+systemFontSize`    | 13pt  | ✅ |
| `+smallSystemFontSize`| 11pt | ✅ |
| `+labelFontSize`     | 10pt  | ✅ |

**NSFont.TextStyle** (macOS 11+; sizes are fixed — macOS does **not** support Dynamic Type):

| TextStyle      | Default size | Line height | Weight       |   |
|----------------|-------------|-------------|--------------|---|
| `.largeTitle`  | 26pt        | 32pt        | Regular (400)| ✅ Apple HIG JSON confirms 26pt; third-party impls (ViewKit 24pt, shaps80 30pt) were pre-macOS-11 approximations |
| `.title1`      | 22pt        | 26pt        | Regular (400)| ✅ Apple HIG |
| `.title2`      | 17pt        | 22pt        | Regular (400)| ✅ Apple HIG |
| `.title3`      | 15pt        | 20pt        | Regular (400)| ✅ Apple HIG |
| `.headline`    | 13pt        | 16pt        | Bold (700)   | ✅ Apple HIG JSON confirms Bold; emphasized weight is Heavy. WWDC 2020 Session 10175 discusses `.body`+bold→SemiBold, not `.headline` itself. iOS `.headline` is SemiBold, but macOS differs. |
| `.subheadline` | 11pt        | 14pt        | Regular (400)| ✅ Apple HIG |
| `.body`        | 13pt        | 16pt        | Regular (400)| ✅ Apple HIG; WWDC 2020 confirms |
| `.callout`     | 12pt        | 15pt        | Regular (400)| ✅ Apple HIG |
| `.footnote`    | 10pt        | 13pt        | Regular (400)| ✅ Apple HIG |
| `.caption1`    | 10pt        | 13pt        | Regular (400)| ✅ Apple HIG JSON confirms 10pt Regular; emphasized weight is Medium. Third-party impls (ViewKit 9pt, shaps80 8pt) were pre-macOS-11 approximations. |
| `.caption2`    | 10pt        | 13pt        | Medium (500) | ✅ Apple HIG JSON confirms 10pt Medium (500); emphasized weight is SemiBold. Same size as caption1, differentiated by weight. Third-party impls (ViewKit 8pt, shaps80 7pt) were wrong. |

**Font weight** is obtained from `NSFontDescriptor` traits dictionary. ✅

#### 1.1.2 Colors

**NSColor semantic class methods** (each returns a color that adapts to
light/dark appearance):

| NSColor method                        | What it provides                           |   |
|---------------------------------------|--------------------------------------------|---|
| `controlAccentColor`                  | System accent color (macOS 10.14+)         | ✅ |
| `windowBackgroundColor`               | Window background                          | ✅ |
| `labelColor`                          | Primary text                               | ✅ |
| `secondaryLabelColor`                 | Secondary/muted text                       | ✅ |
| `tertiaryLabelColor`                  | Tertiary text                              | ✅ |
| `quaternaryLabelColor`                | Quaternary text                            | ✅ |
| `controlColor`                        | Button/control background                  | ✅ catalog color (NSColorType.catalog) — must convert via `colorUsingColorSpace:` before reading RGB |
| `controlBackgroundColor`              | Content area background (lists, text views)| ✅ |
| `controlTextColor`                    | Button/control text                        | ✅ |
| `disabledControlTextColor`            | Disabled control text                      | ✅ |
| `selectedContentBackgroundColor`      | Selection background (key window)          | ✅ |
| `unemphasizedSelectedContentBackgroundColor` | Selection background (non-key window) | ✅ |
| `selectedTextColor`                   | Selected text foreground                   | ✅ |
| `alternateSelectedControlTextColor`   | Text on accent-colored selection           | ✅ |
| `separatorColor`                      | Separator/border lines                     | ✅ |
| `gridColor`                           | Table grid lines                           | ✅ |
| `linkColor`                           | Hyperlink text                             | ✅ |
| `placeholderTextColor`                | Input placeholder text                     | ✅ |
| `keyboardFocusIndicatorColor`         | Focus ring around focused controls         | ✅ |
| `underPageBackgroundColor`            | Under-page/sidebar background              | ✅ |
| `windowFrameTextColor`                | Window title bar text                      | ✅ |
| `textBackgroundColor`                 | Text input background                      | ✅ |
| `textColor`                           | Text input foreground                      | ✅ |
| `headerTextColor`                     | Table/list column header text              | ✅ |
| `shadowColor`                         | Shadow color                               | ✅ |
| `highlightColor`                      | Highlight overlay                          | ✅ |
| `findHighlightColor`                  | Find/search match highlight                | ✅ |
| `systemRedColor`                      | Error/danger semantic color                | ✅ |
| `systemOrangeColor`                   | Warning semantic color                     | ✅ |
| `systemGreenColor`                    | Success semantic color                     | ✅ |
| `systemBlueColor`                     | Info semantic color                        | ✅ |
| `systemYellowColor`                   | Caution semantic color                     | ✅ |
| `systemPurpleColor`                   | Purple semantic color                      | ✅ |
| `systemPinkColor`                     | Pink semantic color                        | ✅ |
| `systemTealColor`                     | Teal semantic color                        | ✅ |
| `systemIndigoColor`                   | Indigo semantic color                      | ✅ |
| `systemCyanColor`                     | Cyan semantic color                        | ✅ |
| `systemMintColor`                     | Mint semantic color                        | ✅ |
| `systemBrownColor`                    | Brown semantic color                       | ✅ |
| `systemGrayColor`                     | Neutral gray                               | ✅ |
| `alternatingContentBackgroundColors`  | Array of alternating row colors            | ✅ |

Colors are resolved per-appearance via
`NSAppearance.performAsCurrentDrawingAppearance`. ✅
P3 colors are converted to sRGB via `colorUsingColorSpace:`. ✅

Most system colors date to macOS 10.10+. ✅ Later additions:
`systemTealColor` (macOS 10.12); ✅ Apple SDK headers: `API_AVAILABLE(macos(10.12))`; class-dump confirms symbol in AppKit binary pre-10.15; WWDC 2019 Session 210 called it "new" because the header declaration first shipped in the 10.15 SDK, but the runtime symbol existed since 10.12
`systemMintColor` (macOS 10.12); ✅ same pattern as `systemTealColor` — runtime symbol present in AppKit 1504 (macOS 10.12) per [w0lfschild class-dump](https://github.com/w0lfschild/macOS_headers/blob/master/macOS/Frameworks/AppKit/1504.82.104/NSColor.h); absent from AppKit 1348 (macOS 10.10); SDK header first appeared in macOS 12.0 SDK with `API_AVAILABLE(macos(10.12))`; Apple docs JSON `introducedAt: "10.12"` is correct (not a metadata bug — matches the class-dump evidence)
`systemIndigoColor` (macOS 10.15); ✅ per WWDC 2019 Session 210 and SDK headers `API_AVAILABLE(macos(10.15))`
`systemCyanColor` (macOS 12); ✅ Apple docs JSON correctly shows 12.0; no class-dump evidence of pre-12 existence (unlike teal/mint)

**Text insertion point color:**

| API                                   | What it provides                           |   |
|---------------------------------------|--------------------------------------------|---|
| `NSTextView.insertionPointColor`      | Per-view caret color (instance property; defaults to `controlTextColor`) | ✅ very old API |
| `NSColor.textInsertionPointColor`     | System text insertion point color (type property; macOS 14+) | ✅ Apple docs JSON: `introducedAt: "14.0"`, `roleHeading: "Type Property"` |
| `NSTextInsertionIndicator`            | System caret view (macOS 14+; follows accent color by default) | ✅ |

macOS 14 changed the caret to match the system accent color and exposed
`NSColor.textInsertionPointColor` as a new type property.

#### 1.1.3 Geometry

macOS has **no system APIs** for corner radius, border width, or
spacing. These values come from AppKit intrinsic control sizes and
Apple HIG documentation:

| Property              | Source                                  | Value         |   |
|-----------------------|-----------------------------------------|---------------|---|
| Window corner radius  | macOS window manager **(measured)**      | 10px          | ✅ multiple sources confirm through Sequoia; macOS Tahoe (26) uses variable radii per window style — 16pt (title-bar-only) confirmed by [macos-corner-fix](https://github.com/m4rkw/macos-corner-fix); toolbar window radii: sources disagree on exact values — [Zed discussion #38233](https://github.com/zed-industries/zed/discussions/38233) reports ~26pt (from WWDC25 Session 310 screenshot), [Podfeet/Steve Harris measurement](https://www.podfeet.com/blog/2025/10/rounded-screenshots-shell-script/) measured 50px at 2× = 25pt, [VS Code PR #270236](https://github.com/microsoft/vscode/pull/270236) suggests 20pt (compact toolbar) and 24pt (standard toolbar); [lapcatsoftware](https://lapcatsoftware.com/articles/2026/3/1.html) describes variable radii qualitatively (toolbar > titlebar-only) without exact values; [alt-tab-macos #4985](https://github.com/lwouis/alt-tab-macos/issues/4985) notes "4 or 5" distinct radii — system may have more tiers than documented here; no public API exists (WebKit reads them dynamically via private `_cornerConfiguration` SPI) |
| Control corner radius | AppKit intrinsic rendering **(measured)**| 5px           | ✅ WebKit [`RenderThemeMac.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/rendering/mac/RenderThemeMac.mm): `baseBorderRadius = 5` for styled popup buttons; consistent with measured push button radius |
| Frame/border width    | AppKit rendering **(measured)**          | 0.5px         | ❓ measured, no Apple docs |
| Scrollbar width       | NSScroller legacy style **(API)**       | 16px          | ✅ confirmed 16px by [developer measurement](https://gist.github.com/martynchamberlin/6aaf8a45b36907e9f1e21a28889f6b0a) and multiple corroborating sources; `scrollerWidth(for:scrollerStyle:)` returns this dynamically for regular control size with legacy style |
| Scrollbar width       | NSScroller overlay style **(measured)** | ~7px (idle thumb) | ✅ Gecko [`ScrollbarDrawingCocoa.cpp`](https://searchfox.org/mozilla-central/source/widget/ScrollbarDrawingCocoa.cpp): overlay non-hovered thumb = 7px, hovered = 11px; Chromium [`native_theme_mac.mm`](https://github.com/chromium/chromium/blob/master/ui/native_theme/native_theme_mac.mm): `GetThumbMinSize()` = 6px; two engines agree on ~6–7px; `scrollerWidth(for:scrollerStyle:)` returns 0 for `.overlay` since overlay scrollbars don't consume layout space |
| Focus ring width      | AppKit rendering **(measured)**          | 3px           | ✅ confirmed via WebKit SPI `UIFocusRingStyle.borderThickness = 3`; Mozilla Bug 53927 (Mac OS 9 era, 2px) is obsolete; modern macOS focus ring is a diffuse glow — 3px is the settled border thickness, visual extent is larger |
| Focus ring offset     | AppKit rendering **(measured)**          | -1px (inset)  | ❓ measured, no Apple docs; WebKit [`RenderThemeCocoa.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/rendering/cocoa/RenderThemeCocoa.mm) notes "macOS controls have never honored outline offset" — focus ring drawn via `drawFocusRingMask()` with no public offset parameter |
| Disabled opacity      | AppKit disabled state **(measured)**     | ≈0.25–0.3     | ❓ no global opacity; `disabledControlTextColor` alpha ≈0.25 ([measured](https://gist.github.com/andrejilderda/8677c565cddc969e6aae7df48622d47c): 0.247 in both L/D); overall visual effect ≈0.3 |
| Drop shadows          | compositor-managed                      | yes           | ✅ |

Scrollbar mode depends on user preference (System Preferences →
General → Show scroll bars) and input device (trackpad → overlay,
mouse → legacy).

#### 1.1.4 Widget Metrics

From AppKit intrinsic content sizes (not directly queryable as numbers,
measured from rendered controls):

| Widget           | Property            | Value              |   |
|------------------|---------------------|--------------------|---|
| NSButton         | intrinsic height    | 22px (regular size)| ✅ well-corroborated |
| NSButton         | horizontal padding  | ~8px               | ❓ The legacy HIG 12px is inter-button *spacing*, not internal padding. Gecko [`nsNativeThemeCocoa.mm`](https://searchfox.org/mozilla-central/source/widget/cocoa/nsNativeThemeCocoa.mm) `pushButtonSettings` margins `IntMargin{0,5,2,5}` are *external* drawing-rect inflation (for focus rings/chrome), not content padding; Gecko's actual CSS content padding for `<button>` is `padding-inline: 4px` ([`forms.css`](https://searchfox.org/mozilla-central/source/layout/style/res/forms.css)). WebKit [`RenderThemeMac.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/rendering/mac/RenderThemeMac.mm) `controlPadding(PushButton)` = **8px** horizontal (comment: "Just use 8px. AppKit wants to use 11px for mini buttons, but that padding is just too large for real-world Web sites"); WebKit `cellOutsets` `{5,7,7,7}` are also *external* outsets. Native NSButton bezel internal padding is not directly queryable; best browser-engine evidence points to **~8px** (WebKit) as the closest approximation of the native value |
| NSTextField      | intrinsic height    | 22px               | ✅ WebKit `RenderThemeMac.mm` search field sizes: regular=22px, small=19px, mini=17px |
| NSTextField      | horizontal padding  | 4px                | ❓ measured |
| NSButton (switch)| checkbox indicator  | 14px               | ❓ WebKit [`ToggleButtonMac.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/platform/graphics/mac/controls/ToggleButtonMac.mm): regular=14px; Gecko `nsNativeThemeCocoa.mm`: native=16px — disagreement between engines |
| NSButton (switch)| label spacing       | 4px                | ❓ measured |
| NSSlider         | track height        | 5px                | ✅ WebKit [`SliderTrackMac.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/platform/graphics/mac/controls/SliderTrackMac.mm): `sliderTrackWidth = 5`; previously listed as 4px (measured) |
| NSSlider         | thumb diameter      | 21px               | ❓ measured; note: WebKit `RenderThemeMac.mm` uses `sliderThumbThickness = 17` but with FIXME "should be obtained from AppKit via `knobThickness`" — actual AppKit value may differ |
| NSSlider         | tick mark length    | 8px                | ❓ measured |
| NSProgressIndicator | bar height       | 6px (visual track) | ❓ measured; control frame: WebKit [`ProgressBarMac.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/platform/graphics/mac/controls/ProgressBarMac.mm) cell=20px (regular); `NSProgressIndicatorPreferredThickness`=14px (deprecated, Apple notes "do not accurately represent the geometry"); visual bar rendered by CoreUI is thinner than frame |
| NSScroller       | track width (legacy)| 16px               | ✅ confirmed by [developer measurement](https://gist.github.com/martynchamberlin/6aaf8a45b36907e9f1e21a28889f6b0a) and `scrollerWidth(for:scrollerStyle:)` |
| NSScroller       | thumb width (overlay)| ~7px (idle)        | ✅ Gecko [`ScrollbarDrawingCocoa.cpp`](https://searchfox.org/mozilla-central/source/widget/ScrollbarDrawingCocoa.cpp): overlay non-hovered thumb thickness = 7px (8px base − 1px overlay reduction), hovered = 11px; Chromium [`native_theme_mac.mm`](https://github.com/chromium/chromium/blob/master/ui/native_theme/native_theme_mac.mm): `GetThumbMinSize()` = 6px minimum width; WebKit delegates to native `NSScrollerImp` (no hardcoded value) |
| NSTabView        | tab height          | 24px               | ❓ measured |
| NSTabView        | tab horizontal pad  | 12px               | ❓ measured |
| NSMenuItem       | item height         | 22px               | ❓ measured, plausible |
| NSMenuItem       | horizontal padding  | 12px               | ❓ measured; Chromium [`menu_config.cc`](https://chromium.googlesource.com/chromium/src/+/refs/heads/main/ui/views/controls/menu/menu_config.cc) corroborates `item_horizontal_border_padding = 12` |
| NSToolTipManager | tooltip padding     | 4px                | ❓ measured |
| NSTableView      | row height          | 24px (macOS 11+)   | ✅ changed from 17pt in Big Sur; confirmed by [lapcatsoftware](https://lapcatsoftware.com/articles/BSAppKit.html) and AppKit Release Notes for macOS 11 |
| NSTableView      | cell horizontal pad | 4px                | ❓ measured |
| NSToolbar        | bar height          | 38px               | ❓ measured; varies by config |
| NSToolbar        | item spacing        | 8px                | ✅ HIG: "8 pixels between toolbar controls" |
| NSSplitView      | thick divider       | 6px                | ✅ GNUstep [`NSSplitView.m`](https://github.com/gnustep/libs-gui/blob/master/Source/NSSplitView.m): thick/paneSplitter=6pt, thin=1pt; default style is thick; CocoaDev confirms |
| NSSwitch         | intrinsic size      | 38 × 22px          | ✅ WebKit `RenderThemeMac.mm`: regular={38,22}, small={32,18}, mini={26,15} |
| NSSwitch         | thumb diameter      | ~18px               | ❓ WebKit [`SwitchThumbMac.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/platform/graphics/mac/controls/SwitchThumbMac.mm): thumb bounding box = track height (22px for regular); visual CoreUI knob ~18px inside that box |
| NSSwitch         | track radius        | half height (pill)  | ✅ |
| NSProgressIndicator | spinning regular | 32px diameter       | ✅ `sizeToFit` with `NSControlSizeRegular`; multiple sources confirm 32/16/10 by control size |
| NSProgressIndicator | spinning small   | 16px diameter       | ✅ `sizeToFit` with `NSControlSizeSmall` |
| NSPopUpButton    | intrinsic height    | 21px (regular size) | ✅ WebKit `RenderThemeMac.mm`: regular=21px, small=18px, mini=15px, large=24px; previously listed as 26px which was the right-padding (arrow area), not height |
| NSPopUpButton    | arrow area width    | ~16–18px            | ❓ measured visible indicator; note: WebKit total right-padding is 26px (includes arrow + surrounding space) |
| NSSegmentedControl | segment height    | 24px (regular size) | ❓ measured |
| NSSegmentedControl | separator width   | 1px                 | ❓ measured |
| NSDisclosureButton | triangle size     | ~13px               | ❓ measured visible triangle; Gecko `nsNativeThemeCocoa.mm`: cell=21×21px — visible triangle is a subset of the cell |

#### 1.1.5 Layout Spacing

Apple HIG defines specific spacing values per context (not a system
API — design documentation only):

| Context                          | Recommended spacing |   |
|----------------------------------|---------------------|---|
| Between related controls         | 8pt                 | ❓ oversimplified: HIG says 12px for regular push buttons, 8px for mini/icon |
| Between unrelated groups         | 20pt                | ❓ 20pt is documented as superview-edge margin, not specifically inter-group |
| Label to its associated control  | 8pt                 | ✅ HIG: 8px for regular, 6px small, 5px mini |
| Content margin (window edge)     | 20pt                | ✅ Auto Layout Guide confirms |
| Compact spacing (toolbar items)  | 8pt                 | ✅ HIG: "8 pixels between toolbar controls" |

Interface Builder's "standard spacing" constraint corresponds to 8pt. ✅ Auto Layout Guide confirms
NSStackView default spacing is 8pt. ✅ Apple docs: "default value is 8.0 points"

#### 1.1.6 Icon Sizes

macOS has no per-context icon size constants. Sizes come from
container conventions and SF Symbols automatic sizing:

| Context                 | Source                        | Size    |   |
|-------------------------|-------------------------------|---------|---|
| Toolbar (regular mode)  | `NSToolbar` convention        | 32pt    | ✅ NSToolbar.SizeMode docs (deprecated) |
| Toolbar (small mode)    | `NSToolbar` convention        | 24pt    | ✅ NSToolbar.SizeMode docs (deprecated) |
| Sidebar (small)         | Apple HIG sidebar metrics     | 16×16px (row: 24pt) | ✅ Apple HIG Sidebars page (macOS, 2022 archived); metrics table removed from current HIG ~2024 |
| Sidebar (medium)        | Apple HIG sidebar metrics     | 20×20px (row: 28pt) | ✅ same source; pre-Big Sur was 18pt (legacy CoreTypes.bundle sizes) |
| Sidebar (large)         | Apple HIG sidebar metrics     | 24×24px (row: 32pt) | ✅ same source; pre-Big Sur was 32pt |
| Menu item               | SF Symbols in menus           | ~13pt   | ❓ inferred from system font size |
| Menu bar extra          | Status item convention        | 16pt    | ❓ community best-practice (Bjango), not official |

SF Symbols sizes are automatic when placed in native containers
(`NSToolbarItem`, sidebar). Manual sizing via
`NSImage.SymbolConfiguration(pointSize:weight:scale:)`.

#### 1.1.7 Accessibility

| Setting                            | API                                                          |   |
|------------------------------------|--------------------------------------------------------------|---|
| Text styles                        | `NSFont.preferredFont(forTextStyle:)` returns role-based fonts | ✅ macOS 11+; sizes are fixed — macOS does not support Dynamic Type (WWDC 2020 confirms) |
| Reduce motion                      | `NSWorkspace.accessibilityDisplayShouldReduceMotion`         | ✅ macOS 10.12 |
| Reduce transparency                | `NSWorkspace.accessibilityDisplayShouldReduceTransparency`   | ✅ macOS 10.10 |
| Increase contrast                  | `NSWorkspace.accessibilityDisplayShouldIncreaseContrast`     | ✅ macOS 10.10 |
| Differentiate without color        | `NSWorkspace.accessibilityDisplayShouldDifferentiateWithoutColor` | ✅ macOS 10.10 |

---

### 1.2 Windows

#### 1.2.1 Fonts

**NONCLIENTMETRICSW** (from `SystemParametersInfoW(SPI_GETNONCLIENTMETRICS)`): ✅

Five separate LOGFONTW entries, each with `lfFaceName` (family),
`lfHeight` (size in logical units), `lfWeight` (weight 0–1000): ✅

| Field            | Role              | Typical default             |   |
|------------------|-------------------|-----------------------------|---|
| `lfMessageFont`  | Body/dialog text  | ⚙ Segoe UI, lfHeight=-12, 400 | ✅ face+size: [Win32 UX Guide](https://learn.microsoft.com/en-us/windows/win32/uxguide/vis-fonts) documents "9 pt. Segoe UI" as default; weight 400 is empirical (not documented) |
| `lfCaptionFont`  | Window title bar  | ⚙ Segoe UI, lfHeight=-12, 400 (Win10) / 700 (Win11) | ✅ face+size: same UX Guide source; weight varies at runtime — Win10 returns 400, Win11 returns 700 (Bold) per [Microsoft Q&A](https://learn.microsoft.com/en-us/answers/questions/5489781/title-bar-text-boldness-in-windows-11) |
| `lfSmCaptionFont`| Tool window title | ⚙ Segoe UI, lfHeight=-12, 400 | ✅ face+size: same UX Guide source; weight 400 empirical |
| `lfMenuFont`     | Menu items        | ⚙ Segoe UI, lfHeight=-12, 400 | ✅ face+size: same UX Guide source; weight 400 empirical |
| `lfStatusFont`   | Status bar text   | ⚙ Segoe UI, lfHeight=-12, 400 | ✅ face+size: same UX Guide source; weight 400 empirical |

Note: Win11 uses Segoe UI Variable internally in WinUI3/XAML controls,
but the Win32 `SystemParametersInfoW` API always returns "Segoe UI".

Size conversion: `points = abs(lfHeight) * 72 / dpi` ↕. ✅ derived from LOGFONTW docs
At 96 DPI: abs(-12) * 72 / 96 = 9pt. ✅

**WinUI3 Fluent Design type ramp** (design guidelines, not system API):

| Ramp name  | Size  | Weight       | Line height |   |
|------------|-------|--------------|-------------|---|
| Caption    | 12px  | Regular (400)| 16px        | ✅ |
| Body       | 14px  | Regular (400)| 20px        | ✅ |
| BodyStrong | 14px  | SemiBold(600)| 20px        | ✅ |
| BodyLarge  | 18px  | Regular (400)| 24px        | ✅ |
| BodyLargeStrong | 18px | SemiBold(600)| 24px   | ✅ |
| Subtitle   | 20px  | SemiBold(600)| 28px        | ✅ |
| Title      | 28px  | SemiBold(600)| 36px        | ✅ |
| TitleLarge | 40px  | SemiBold(600)| 52px        | ✅ |
| Display    | 68px  | SemiBold(600)| 92px        | ✅ |

All confirmed via MS Typography docs. All sizes are in effective pixels
(epx), which equal physical pixels at 100% scaling (96 DPI).

There is **no system monospace font setting** on Windows. ✅ Applications
choose their own (typically Consolas or Cascadia Mono).

#### 1.2.2 Colors

**UISettings (WinRT):** ✅

| Method / Value        | What it provides               |   |
|-----------------------|--------------------------------|---|
| `GetColorValue(Foreground)` | System foreground color  | ✅ |
| `GetColorValue(Background)` | System background color  | ✅ |
| `GetColorValue(Accent)`     | Accent color             | ✅ |
| `GetColorValue(AccentDark1/2/3)` | Darker accent shades | ✅ |
| `GetColorValue(AccentLight1/2/3)`| Lighter accent shades| ✅ |
| `GetColorValue(Complement)`     | Complement (not supported, do not use) | ✅ exists but docs say "Do not use" |

**GetSysColor (Win32):** ✅

| Constant             | What it provides                    |   |
|----------------------|-------------------------------------|---|
| `COLOR_WINDOW`       | Text input / view background        | ✅ |
| `COLOR_WINDOWTEXT`   | Text input / view foreground        | ✅ |
| `COLOR_BTNFACE`      | Button background                   | ✅ |
| `COLOR_BTNTEXT`      | Button foreground                   | ✅ |
| `COLOR_HIGHLIGHT`    | Selection background                | ✅ |
| `COLOR_HIGHLIGHTTEXT`| Selection foreground                | ✅ |
| `COLOR_GRAYTEXT`     | Disabled text                       | ✅ |
| `COLOR_MENU`         | Menu background                     | ✅ |
| `COLOR_MENUTEXT`     | Menu foreground                     | ✅ |
| `COLOR_SCROLLBAR`    | Scrollbar track                     | ✅ |
| `COLOR_INFOBK`       | Tooltip background                  | ✅ |
| `COLOR_INFOTEXT`     | Tooltip foreground                  | ✅ |
| `COLOR_ACTIVECAPTION`| Active title bar background         | ✅ |
| `COLOR_CAPTIONTEXT`  | Active title bar text               | ✅ |
| `COLOR_INACTIVECAPTION`| Inactive title bar background     | ✅ |
| `COLOR_INACTIVECAPTIONTEXT`| Inactive title bar text        | ✅ |
| `COLOR_3DSHADOW`     | 3D shadow edge                      | ✅ |
| `COLOR_3DHIGHLIGHT`  | 3D highlight edge                   | ✅ |
| `COLOR_HOTLIGHT`     | Hot-tracked / hyperlink color       | ✅ |

On Windows 10+, only `COLOR_WINDOW`, `COLOR_WINDOWTEXT`, `COLOR_HIGHLIGHT`,
`COLOR_HIGHLIGHTTEXT`, `COLOR_3DFACE`, `COLOR_GRAYTEXT`, `COLOR_BTNTEXT`,
and `COLOR_HOTLIGHT` are supported. ✅ MSDN confirms (8 constants). Note:
`COLOR_3DFACE` (value 15) is not marked "not supported", but its alias
`COLOR_BTNFACE` (same value 15) is — a documentation inconsistency.
The remaining constants listed above (`COLOR_MENU`,
`COLOR_MENUTEXT`, `COLOR_SCROLLBAR`, `COLOR_INFOBK`, `COLOR_INFOTEXT`,
`COLOR_ACTIVECAPTION`, `COLOR_CAPTIONTEXT`, `COLOR_INACTIVECAPTION`,
`COLOR_INACTIVECAPTIONTEXT`, `COLOR_3DSHADOW`, `COLOR_3DHIGHLIGHT`) are
annotated "not supported" by MSDN — they still return values but may not
reflect the actual system appearance.

**DWM:**

| Function                  | What it provides                   |   |
|---------------------------|------------------------------------|---|
| `DwmGetColorizationColor` | Window chrome / title bar color    | ✅ |

#### 1.2.3 Geometry

**GetSystemMetricsForDpi (Win32):**

| Metric           | What it provides              | Typical at 96 DPI |   |
|------------------|-------------------------------|--------------------|---|
| `SM_CXBORDER`    | Border width                  | ↕ 1px              | ✅ MSDN NONCLIENTMETRICSW: "iBorderWidth: The thickness of the sizing border… The default is 1 pixel" |
| `SM_CYBORDER`    | Border height                 | ↕ 1px              | ✅ same source |
| `SM_CXVSCROLL`   | Vertical scrollbar width      | ↕ 17px             | ✅ confirmed via .NET `SystemParameters.VerticalScrollBarWidth` docs and multiple measurements |
| `SM_CYHSCROLL`   | Horizontal scrollbar height   | ↕ 17px             | ✅ same |
| `SM_CYVTHUMB`    | Scrollbar thumb height        | ↕ 17px             | ❓ no explicit MSDN default; derivable from registry `ScrollHeight` default=-255 → -255/-15=17px |
| `SM_CYMENU`      | Menu bar height               | ↕ 20px             | ✅ registry default `MenuHeight`=-285 = 19px (`iMenuHeight`); SM_CYMENU adds +1 border pixel = 20px (confirmed via Wine source: `iMenuHeight + 1`). MSDN: "height of a single-line menu bar… not the height of a menu item" — Win32 dropdown menu items derive height from font + padding, not this metric |
| `SM_CXFOCUSBORDER`| Focus border width           | ↕ 1px              | ✅ confirmed by ReactOS and Wine default values |
| `SM_CYFOCUSBORDER`| Focus border height          | ↕ 1px              | ✅ same |
| `SM_CXSMICON`    | Small icon width              | ↕ 16px             | ✅ standard Windows icon size |
| `SM_CYSMICON`    | Small icon height             | ↕ 16px             | ✅ same |

**GetDpiForSystem:** Returns system DPI (96 = 100% scaling). ✅

**Hardcoded WinUI3 Fluent values** (not queryable, design guidelines):

| Property          | Value |   |
|-------------------|-------|---|
| Control radius    | 4px   | ✅ MS Geometry docs: ControlCornerRadius=4px |
| Overlay radius    | 8px   | ✅ MS Geometry docs: OverlayCornerRadius=8px |
| Shadow            | yes   | ✅ |
| Disabled opacity  | per-control; `ListViewItemDisabledThemeOpacity`=0.3 | ❓ no global disabled opacity; WinUI3 uses per-control `*Disabled` color brushes; 0.55 was legacy Win8.x/UWP — current WinUI3 value is 0.3 |

WinUI3 focus visual primary border is 2px ✅ (differs from Win32
`SM_CXFOCUSBORDER` = 1px). Secondary border is 1px inside. ✅

#### 1.2.4 Widget Metrics

**From system metrics + WinUI3 Fluent defaults:**

| Widget          | Property            | Source                        | Value    |   |
|-----------------|---------------------|-------------------------------|----------|---|
| Button          | min height          | WinUI3 default                | ~27px (effective) | ❓ no `ButtonMinHeight` resource; effective=14px text + 5+6 padding + 2 border = 27px; `ContentDialogButtonHeight=32` is dialog-specific |
| Button          | horizontal padding  | WinUI3 default                | 11px     | ✅ ButtonPadding=11,5,11,6 |
| Button          | vertical padding    | WinUI3 default                | 5px top, 6px bottom | ✅ same |
| Button          | icon spacing        | WinUI3 default                | 8px      | ❓ hardcoded `Margin="8,0,0,0"` in DropDownButton template; no named XAML resource; consistent with Fluent 2 `size80` spacing |
| CheckBox        | indicator size      | WinUI3 default                | 20px     | ✅ CheckBoxSize=20 |
| CheckBox        | label spacing       | WinUI3 default                | 8px      | ✅ CheckBoxPadding first value=8 |
| TextBox         | min height          | WinUI3 default                | 32px     | ✅ `TextControlThemeMinHeight=32` in generic.xaml |
| TextBox         | horizontal padding  | WinUI3 default                | 10px left, 6px right | ✅ TextControlThemePadding=10,5,6,6 (asymmetric: right is smaller due to delete button column) |
| TextBox         | vertical padding    | WinUI3 default                | 5px top, 6px bottom | ✅ TextControlThemePadding=10,5,6,6 |
| Scrollbar (Win32) | width             | `SM_CXVSCROLL` (DPI-aware)    | ↕ 17px   | ✅ see §1.2.3 |
| Scrollbar (Win32) | thumb height     | `SM_CYVTHUMB` (DPI-aware)     | ↕ 17px   | ❓ see §1.2.3 |
| Scrollbar (WinUI3)| collapsed width  | ScrollBar template (inline)   | ~2px     | ❓ XAML template inline value; expands on pointer proximity |
| Scrollbar (WinUI3)| expanded width   | ScrollBar template (inline)   | ~6px     | ❓ XAML template inline value; no named theme resource |
| Slider          | track height        | WinUI3 default                | 4px      | ✅ SliderTrackThemeHeight=4 |
| Slider          | thumb size          | WinUI3 default                | 18px     | ✅ SliderHorizontalThumbWidth/Height=18 |
| Slider          | tick length         | WinUI3 default                | 4px      | ✅ `SliderOutsideTickBarThemeHeight=4` |
| ProgressBar     | min height (control)| WinUI3 default                | 3px      | ✅ ProgressBarMinHeight=3 |
| ProgressBar     | track height        | WinUI3 default                | 1px      | ✅ ProgressBarTrackHeight=1 |
| TabView         | min height          | WinUI3 default                | 32px     | ✅ TabViewItemMinHeight=32 |
| TabView         | horizontal padding  | WinUI3 default                | 8px left, 4px right | ✅ `TabViewItemHeaderPadding=8,3,4,3` (8/8 without close button) |
| TabView         | vertical padding    | WinUI3 default                | 3px      | ✅ same source |
| Menu item       | height              | Win32: font-derived; WinUI3: padding-derived | ↕ ~20px (Win32) | ❓ Win32 formula (from [Wine `menu.c`](https://github.com/wine-mirror/wine/blob/master/dlls/user32/menu.c) / [ReactOS `menu.c`](https://github.com/nicknisi/reactos/blob/master/win32ss/user/ntuser/menu.c)): `max(text_height + 2, char_height + 4)` → at 96 DPI with Segoe UI 9pt (cell ~16px): max(18, 20) = 20px. `SM_CYMENU` (20px) is menu *bar* height, not item height — the match is coincidental. WinUI3: touch ~31px (`MenuFlyoutItemThemePadding=11,8,11,9` + 14px text), narrow ~23px (`MenuFlyoutItemThemePaddingNarrow=11,4,11,5`) |
| Menu item       | horizontal padding  | WinUI3 default                | 11px     | ✅ `MenuFlyoutItemThemePadding=11,8,11,9` (+ 4px outer `MenuFlyoutItemMargin`) |
| Menu item       | icon spacing        | WinUI3 default                | 12px     | ✅ icon placeholder=28px minus 16px icon = 12px gap |
| ToolTip         | padding             | WinUI3 default                | 9px horiz, 6/8px vert | ✅ ToolTipBorderPadding=9,6,9,8 |
| ToolTip         | max width           | WinUI3 default                | 320px    | ✅ MaxWidth=320 |
| ListView        | item height         | WinUI3 default                | 40px     | ✅ ListViewItemMinHeight=40 |
| ListView        | horizontal padding  | WinUI3 default                | 12px     | ✅ `Padding="12,0,12,0"` in Grid-based style (first style uses 16/12) |
| ListView        | vertical padding    | WinUI3 default                | 0px      | ✅ vertical space from `MinHeight=40`, not padding |
| CommandBar      | height (default)    | WinUI3 default                | 64px     | ✅ AppBarThemeMinHeight=64 |
| CommandBar      | height (compact)    | WinUI3 default                | 48px     | ✅ AppBarThemeCompactHeight=48 |
| CommandBar      | item spacing        | WinUI3 default                | 0px      | ✅ StackPanel has no Spacing; visual separation from AppBarButton inner margins (2,6,2,6) |
| CommandBar      | padding             | WinUI3 default                | 4px left only | ✅ `Padding="4,0,0,0"` |
| ToggleSwitch    | track width         | WinUI3 default                | 40px     | ✅ OuterBorder Width=40 |
| ToggleSwitch    | track height        | WinUI3 default                | 20px     | ✅ OuterBorder Height=20 |
| ToggleSwitch    | thumb size (rest)   | WinUI3 default                | 12px     | ✅ 12×12 |
| ToggleSwitch    | thumb size (hover)  | WinUI3 default                | 14px     | ✅ 14×14 |
| ToggleSwitch    | track radius        | WinUI3 default                | 10px (pill) | ✅ |
| ContentDialog   | min/max width       | WinUI3 default                | 320–548px| ✅ XAML confirmed |
| ContentDialog   | min/max height      | WinUI3 default                | 184–756px| ✅ XAML confirmed |
| ContentDialog   | content padding     | WinUI3 default                | 24px     | ✅ ContentDialogPadding=24 |
| ContentDialog   | button spacing      | WinUI3 default                | 8px      | ✅ ContentDialogButtonSpacing=8 |
| ContentDialog   | title font          | WinUI3 default                | 20px SemiBold | ✅ |
| ContentDialog   | corner radius       | WinUI3 default                | 8px      | ✅ OverlayCornerRadius |
| ProgressRing    | default size        | WinUI3 default                | 32×32px  | ✅ Width/Height=32 |
| ProgressRing    | min size            | WinUI3 default                | 16×16px  | ✅ XAML template `MinWidth/MinHeight=16` |
| ProgressRing    | stroke width        | WinUI3 default                | 4px      | ✅ ProgressRingStrokeThickness=4 |
| ComboBox        | min height          | WinUI3 default                | 32px     | ✅ ComboBox_themeresources.xaml |
| ComboBox        | min width           | WinUI3 default                | 64px     | ✅ `ComboBoxThemeMinWidth=64` |
| ComboBox        | padding             | WinUI3 default                | 12,5,0,7 | ✅ ComboBox_themeresources.xaml |
| ComboBox        | arrow glyph size    | WinUI3 default                | 12px     | ✅ glyph min-width/min-height=12 |
| ComboBox        | arrow area width    | WinUI3 default                | 38px     | ✅ ColumnDefinition Width=38 |
| Expander        | header min height   | WinUI3 default                | 48px     | ✅ `ExpanderMinHeight=48` |
| Expander        | chevron button size | WinUI3 default                | 32×32px  | ✅ `ExpanderChevronButtonSize=32` |
| Expander        | chevron glyph size  | WinUI3 default                | 12px     | ✅ `ExpanderChevronGlyphSize=12` |
| Expander        | content padding     | WinUI3 default                | 16px     | ✅ `ExpanderContentPadding=16` |
| HyperlinkButton | padding             | WinUI3 default                | 11,5,11,6 | ✅ inherits `ButtonPadding` |
| HyperlinkButton | background          | WinUI3 default                | transparent | ✅ |

#### 1.2.5 Layout Spacing

**WinUI3 Fluent spacing tokens** (design guidelines, not system API;
Fluent 2 uses numeric names `sizeNone`..`size320` for the code-implemented subset; the full design ramp extends to `size560`): ✅

| Token name | Value |   |
|------------|-------|---|
| None       | 0px   | ✅ |
| XXSmall    | 2px   | ✅ |
| XSmall     | 4px   | ✅ |
| sNudge     | 6px   | ✅ |
| Small      | 8px   | ✅ |
| mNudge     | 10px  | ✅ |
| Medium     | 12px  | ✅ |
| Large      | 16px  | ✅ |
| XLarge     | 20px  | ✅ |
| XXLarge    | 24px  | ✅ |
| XXXLarge   | 32px  | ✅ |

All pixel values confirmed via FluentUI spacings.ts. Token names are informal shorthand from the internal code keys (`xxs`, `xs`, `s`, etc.) — Fluent 2 design system uses `sizeNone`..`size320` (code subset) or up to `size560` (full ramp, 17 tokens); code exports use `spacingHorizontalXXS` etc.

**What these tokens are for**: This is a value palette for WinUI3
control template authors — a menu of recommended spacing values to
pick from when defining padding, margins, and gaps inside XAML
templates. Individual controls pick specific values from this ramp
(and often use off-ramp values like 11px, 9px, 3px that don't land
on any token). The tokens are not a system API, not user-configurable,
and not exposed at runtime.

**Why we don't implement this ramp**: Every spacing value that matters
is already captured as a direct per-widget field — `button.padding_horizontal`
= 11px (from `ButtonPadding`), `dialog.button_spacing` = 8px (from
`ContentDialogButtonSpacing`), `menu.icon_spacing` = 12px, etc. (see
§2.3–2.28). The abstract ramp adds no information beyond what the
per-widget fields already provide. Windows has no layout container
defaults either — `StackPanel.Spacing` defaults to 0 — so unlike KDE
(§1.3.5) there are no global layout constants to capture in §2.20.

#### 1.2.6 Icon Sizes

**GetSystemMetrics (Win32):**

| Metric           | What it provides    | Typical at 96 DPI |   |
|------------------|---------------------|--------------------|---|
| `SM_CXICON`      | Large icon width    | ↕ 32px             | ✅ |
| `SM_CYICON`      | Large icon height   | ↕ 32px             | ✅ |
| `SM_CXSMICON`    | Small icon width    | ↕ 16px             | ✅ |
| `SM_CYSMICON`    | Small icon height   | ↕ 16px             | ✅ |

**Shell image lists** (`SHGetImageList`):

| Constant          | Default size |   |
|-------------------|-------------|---|
| `SHIL_SMALL`      | 16px        | ✅ |
| `SHIL_LARGE`      | 32px        | ✅ |
| `SHIL_EXTRALARGE` | 48px        | ✅ |
| `SHIL_JUMBO`      | 256px       | ✅ |

**WinUI3 Fluent icon contexts** (Segoe Fluent Icons):

| Context              | Typical size |   |
|----------------------|-------------|---|
| `AppBarButton` icon  | 20px        | ✅ MS docs say 20×20; `AppBarButtonContentHeight=16` is from legacy v1 (Reveal) styles only — current WinUI3 uses 20px |
| `NavigationViewItem` | 16px        | ✅ `NavigationViewItemOnLeftIconBoxHeight=16` |

#### 1.2.7 Accessibility

| Setting              | API                                        |   |
|----------------------|--------------------------------------------|---|
| Display scale factor | `GetDpiForSystem()` / 96 (96 = 100%)       | ✅ |
| Text scale factor    | `UISettings.TextScaleFactor` (WinRT, 1.0–2.25) | ✅ MSDN confirms range 1.0–2.25 |
| High contrast mode   | `SystemParametersInfoW(SPI_GETHIGHCONTRAST)`| ✅ |
| Reduce motion        | `SystemParametersInfoW(SPI_GETCLIENTAREAANIMATION)` | ✅ |

Display scaling (DPI) affects all UI uniformly. Text scaling is an
independent accessibility setting (Settings → Accessibility → Text size)
that only enlarges text.

---

### 1.3 KDE

#### 1.3.1 Fonts

**kdeglobals `[General]` section** — `QFont::toString()` format: ✅
`family,pointSizeF,pixelSize,styleHint,weight,style,underline,strikeOut,fixedPitch,...`

Field 0 = family, field 1 = point size, field 4 = weight. ✅
Qt6 adds extra fixed fields (capitalization, letterSpacingType, letterSpacing,
wordSpacing, stretch, styleStrategy, font style, font features, variable axes)
after field 9. Qt6 6.4–6.10 produces 16 fixed fields + an optional
styleName (17th). Qt6 6.11+ (released 2026-03-23) always emits
styleName and adds features/variableAxes counts, producing a minimum of 19
fields. Parser should handle variable field counts gracefully.

Weight scale differs between Qt versions:
- **Qt5**: 0–99 (Normal=50, DemiBold=63, Bold=75, Black=87) ✅ Black (87) is the highest named constant; max accepted value is 99
- **Qt6**: 1–1000 (Normal=400, DemiBold=600, Bold=700, Black=900) ✅ named constants span 100–900 but range accepts 1–1000

Parser must detect which scale is in use and normalize to CSS 100–900.

| Key                    | Role              | Typical Breeze default                      |   |
|------------------------|-------------------|---------------------------------------------|---|
| `font`                 | Body text         | ⚙ Noto Sans, 10pt, 400                     | ✅ kfontsettingsdata.cpp |
| `fixed`                | Monospace         | ⚙ Hack, 10pt, 400                          | ✅ kfontsettingsdata.cpp |
| `smallestReadableFont` | Smallest text     | ⚙ Noto Sans, 8pt, 400                      | ✅ kfontsettingsdata.cpp |
| `toolBarFont`          | Toolbar labels    | ⚙ Noto Sans, 10pt, 400 (can be smaller)    | ✅ |
| `menuFont`             | Menu items        | ⚙ Noto Sans, 10pt, 400                     | ✅ |
| `taskbarFont`          | Taskbar/panel     | ⚙ Noto Sans, 10pt, 400                     | ✅ in kfontsettingsdata.cpp (not in kcfg GUI) |

**`[WM]` section font:**

| Key                    | Role              | Typical Breeze default                      |   |
|------------------------|-------------------|---------------------------------------------|---|
| `activeFont`           | Window title bar  | ⚙ Noto Sans, 10pt, 400 (Normal)            | ✅ |

#### 1.3.2 Colors

**kdeglobals color groups** — each group has these possible keys: ✅ verified in kcolorscheme.cpp

```
BackgroundNormal, BackgroundAlternate,
ForegroundNormal, ForegroundInactive, ForegroundActive,
ForegroundLink, ForegroundVisited,
ForegroundNegative, ForegroundNeutral, ForegroundPositive,
DecorationFocus, DecorationHover
```

Values are `R,G,B` (three comma-separated u8 values). ✅

| Section                   | What it provides                            |   |
|---------------------------|---------------------------------------------|---|
| `[Colors:Window]`         | Window/dialog backgrounds and foregrounds   | ✅ |
| `[Colors:View]`           | Editable content areas (inputs, list views) | ✅ |
| `[Colors:Button]`         | Button backgrounds and foregrounds          | ✅ |
| `[Colors:Selection]`      | Selection backgrounds and foregrounds       | ✅ |
| `[Colors:Tooltip]`        | Tooltip backgrounds and foregrounds         | ✅ |
| `[Colors:Complementary]`  | Complementary areas (e.g. dark sidebar)     | ✅ |
| `[Colors:Header]`         | Table/list column header (KF 5.71+)         | ✅ commit fce11e205c (2020-05-20) landed between v5.70.0 and v5.71.0 tags |

**`[WM]` section** (window manager / title bar): ✅ verified in BreezeLight/BreezeDark.colors

| Key                   | What it provides              |   |
|-----------------------|-------------------------------|---|
| `activeBackground`    | Active title bar background   | ✅ |
| `activeForeground`    | Active title bar foreground   | ✅ |
| `inactiveBackground`  | Inactive title bar background | ✅ |
| `inactiveForeground`  | Inactive title bar foreground | ✅ |
| `activeBlend`         | Active blend color            | ✅ |
| `inactiveBlend`       | Inactive blend color          | ✅ |

#### 1.3.3 Geometry

KDE has **no geometry settings in kdeglobals**. All geometry values
come from the Breeze style engine source code:

| Property              | Breeze source constant          | Value |   |
|-----------------------|---------------------------------|-------|---|
| Control corner radius | `Frame_FrameRadius`             | 5px   | ✅ breezemetrics.h |
| Frame/border width    | `PenWidth::Frame`               | 1.001px | ✅ breezemetrics.h |
| Scrollbar groove width| `ScrollBar_Extend`              | 21px  | ✅ breezemetrics.h |
| Focus ring margin     | `PM_FocusFrameHMargin`          | 2px   | ✅ breezemetrics.h |
| Disabled state        | `ColorEffects:Disabled` palette blending | (no single opacity) | ✅ |
| Drop shadows          | yes (KWin compositor)           |       | ✅ |

#### 1.3.4 Widget Metrics

From Breeze style engine source code (`breezehelper.cpp`,
`breezemetrics.h`):

All breezemetrics.h constants verified against source:

| Constant name              | Widget / property          | Value |   |
|----------------------------|----------------------------|-------|---|
| `Button_MinWidth`          | Button min width           | 80px  | ✅ |
| `Button_MarginWidth`       | Button horizontal padding  | 6px   | ✅ |
| `Button_ItemSpacing`       | Button icon-to-label gap   | 4px   | ✅ |
| `CheckBox_Size`            | Checkbox indicator size    | 20px  | ✅ |
| `CheckBox_ItemSpacing`     | Checkbox label spacing     | 4px   | ✅ |
| `LineEdit_FrameWidth`      | Input horizontal padding   | 6px   | ✅ breezemetrics.h |
| `ScrollBar_Extend`         | Scrollbar groove width     | 21px  | ✅ |
| `ScrollBar_SliderWidth`    | Scrollbar thumb width      | 8px   | ✅ |
| `ScrollBar_MinSliderHeight`| Scrollbar min thumb height | 20px  | ✅ |
| `Slider_GrooveThickness`   | Slider track height        | 6px   | ✅ |
| `Slider_ControlThickness`  | Slider thumb size          | 20px  | ✅ |
| `Slider_TickLength`        | Slider tick mark length    | 8px   | ✅ |
| `ProgressBar_Thickness`    | Progress bar height        | 6px   | ✅ |
| `ProgressBar_BusyIndicatorSize` | Busy indicator size   | 14px  | ✅ |
| `TabBar_TabMinWidth`       | Tab min width              | 80px  | ✅ |
| `TabBar_TabMinHeight`      | Tab min height             | 30px  | ✅ |
| `TabBar_TabMarginWidth`    | Tab horizontal padding     | 8px   | ✅ |
| `TabBar_TabMarginHeight`   | Tab vertical padding       | 4px   | ✅ |
| `MenuItem_MarginWidth`     | Menu item horizontal pad   | 4px (was 5 through v6.5.2) | ✅ current=4; changed in v6.5.3 cycle |
| `MenuItem_MarginHeight`    | Menu item vertical padding | 4px (was 3→5→4) | ✅ current=4; was 3 (≤v6.5.2), 5 (v6.5.3), 4 (v6.5.4+) |
| `MenuItem_TextLeftMargin`  | Menu item text left margin | 8px (new in 6.5.3+) | ✅ commit 35967f0a (2025-11-17), shipped in v6.5.3 |
| `ToolTip_FrameWidth`       | Tooltip padding            | 3px   | ✅ |
| `ItemView_ItemMarginLeft/Right` | List item horizontal pad | 2px   | ✅ breezemetrics.h |
| `ItemView_ItemMarginTop/Bottom` | List item vertical padding | 1px | ✅ breezemetrics.h |
| `ToolBar_ItemSpacing`      | Toolbar item spacing       | 0px   | ✅ |
| `ToolBar_ItemMargin`       | Toolbar item margin        | 6px   | ✅ |
| `Splitter_SplitterWidth`   | Splitter width             | 1px   | ✅ |
| `ComboBox_FrameWidth`      | ComboBox padding           | 6px   | ✅ breezemetrics.h |
| `MenuButton_IndicatorWidth`| ComboBox arrow area width  | 20px  | ✅ breezemetrics.h |
| `GroupBox_TitleMarginWidth` | GroupBox title margin       | 4px   | ✅ breezemetrics.h |
| `ItemView_ArrowSize`       | Tree/disclosure arrow size | 10px  | ✅ breezemetrics.h (`ArrowSize=10`) |
| (QQC2 Switch)              | Track size (font-derived)  | ~36 × 18px | ✅ `implicitWidth=height*2`, `height=fontMetrics.height`≈18px at default font |
| (QQC2 Switch)              | Handle diameter            | ~18px | ✅ `= fontMetrics.height` |
| (QQC2 BusyIndicator)       | Spinner size               | 36px  | ✅ `Kirigami.Units.gridUnit*2` = 36px at default |

#### 1.3.5 Layout Spacing

From Breeze source code:

| Constant name              | What it provides              | Value |   |
|----------------------------|-------------------------------|-------|---|
| `Layout_TopLevelMarginWidth`| Window/dialog content margin | 10px  | ✅ breezemetrics.h |
| `Layout_ChildMarginWidth`  | Nested container margin       | 6px   | ✅ breezemetrics.h |
| `Layout_DefaultSpacing`    | Default gap between widgets   | 6px   | ✅ breezemetrics.h |

There is **no abstract spacing scale** in KDE. These are specific
layout constants. ✅

#### 1.3.6 Icon Sizes

The active icon theme name is read from `kdeglobals [Icons] Theme`
(default: `breeze`). ✅

**`KIconLoader` groups** — sizes come from the icon theme's own
`index.theme` (`DesktopDefault`, `ToolbarDefault`, etc. in `[Icon Theme]`
section), **not** from `kdeglobals`. C++ fallbacks in `kicontheme.cpp`
are used only when the icon theme omits a key:

| Group / icon theme key     | C++ fallback | Breeze default |   |
|----------------------------|-------------|----------------|---|
| `Desktop` `DesktopDefault` | 32px        | 48px           | ✅ Breeze index.theme overrides C++ fallback |
| `Toolbar` `ToolbarDefault` | ⚙ 22px     | 22px           | ✅ |
| `MainToolbar` `MainToolbarDefault` | ⚙ 22px | 22px       | ✅ |
| `Small` `SmallDefault`     | ⚙ 16px     | 16px           | ✅ |
| `Panel` `PanelDefault`     | ⚙ 48px     | 48px           | ✅ Breeze index.theme matches C++ fallback (was 32 until KF5 v5.34.0, changed to 48 circa 2017) |
| `Dialog` `DialogDefault`   | ⚙ 32px     | 32px           | ✅ |

#### 1.3.7 Accessibility

| Setting              | Source                                       |   |
|----------------------|----------------------------------------------|---|
| Font DPI override    | `~/.config/kcmfontsrc` `[General] forceFontDPI` (Plasma 6 fonts KCM hides this on Wayland via `visible: Qt.platform.pluginName === "xcb"` in `main.qml`; visible on X11 only; `plasma6.0-remove-dpi-settings.cpp` migration deletes `forceFontDPIWayland` on upgrade; config key still works if set manually) | ✅ Plasma 6: UI visible on X11 only, hidden on Wayland |
| Scale factor         | `forceFontDPI / 96` (from `kcmfontsrc`)      | ✅ |
| Animation factor     | `kdeglobals [KDE] AnimationDurationFactor` (0 = disabled) | ✅ kwin.kcfg `<min>0</min>`; 0 is the intended "disabled" semantic |

---

### 1.4 GNOME

#### 1.4.1 Fonts

**gsettings keys:**

| Schema.Key                                          | Role         | Default (GNOME 48+)       | Pre-48 default     |   |
|-----------------------------------------------------|--------------|---------------------------|---------------------|---|
| `org.gnome.desktop.interface font-name`             | Body text    | ⚙ Adwaita Sans 11         | Cantarell 11        | ✅ |
| `org.gnome.desktop.interface document-font-name`    | Document text| ⚙ Adwaita Sans 11         | Cantarell 11        | ✅ |
| `org.gnome.desktop.interface monospace-font-name`   | Monospace    | ⚙ Adwaita Mono 11         | Source Code Pro 10  | ✅ |
| `org.gnome.desktop.wm.preferences titlebar-font`    | Title bar    | ⚙ Adwaita Sans Bold 11    | Cantarell Bold 11   | ✅ |

Font strings use Pango format: `[FAMILY-LIST] [STYLE-OPTIONS] SIZE [VARIATIONS] [FEATURES]` ✅
(e.g., "Cantarell Bold 11" → family=Cantarell, weight=Bold, size=11pt).
Style options can include weight, style (Italic), variant, stretch, gravity.
Optional `VARIATIONS` (e.g. `@wght=200`) and `FEATURES` (e.g. `#tnum=1`) segments are supported in modern Pango.

**libadwaita CSS type scale classes** (sizes are percentage-based,
shown here at default 11pt base):

All verified from libadwaita `src/stylesheet/widgets/_labels.scss`:

| CSS class         | CSS `font-size` | Computed size | Weight |   |
|-------------------|-----------------|---------------|--------|---|
| `.caption`        | 82%             | ≈ 9pt         | 400    | ✅ |
| `.caption-heading`| 82%             | ≈ 9pt         | 700    | ✅ |
| `.body`           | (inherited)     | (base font)   | 400    | ✅ |
| `.heading`        | (inherited)     | (base font)   | 700    | ✅ |
| `.title-4`        | 118%            | ≈ 13pt        | 700    | ✅ |
| `.title-3`        | 136%            | ≈ 15pt        | 700    | ✅ |
| `.title-2`        | 136%            | ≈ 15pt        | 800    | ✅ |
| `.title-1`        | 181%            | ≈ 20pt        | 800    | ✅ |

`.title-2` and `.title-3` intentionally share the same font-size (136%)
and are differentiated only by weight (800 vs 700). ✅

#### 1.4.2 Colors

**D-Bus portal (org.freedesktop.appearance):**

| Key            | What it provides                                            |   |
|----------------|-------------------------------------------------------------|---|
| `color-scheme` | Dark/light preference (0=no-preference, 1=prefer-dark, 2=prefer-light) | ✅ |
| `accent-color` | User-chosen accent hue (RGB doubles, out-of-range = unset)  | ✅ |
| `contrast`     | Contrast preference (0=normal, 1=high)                      | ✅ |
| `reduced-motion`| Motion preference (0=no-preference, 1=reduce)              | ✅ |

**libadwaita CSS** defines all other colors. The `adwaita` preset is
measured from these CSS values. GNOME provides no per-color system APIs
beyond the accent — everything comes from the CSS theme.

#### 1.4.3 Geometry

All geometry comes from **libadwaita CSS** (not system APIs):

| Property          | CSS source                     | Value  |   |
|-------------------|--------------------------------|--------|---|
| Control radius    | `$button_radius`               | 9px    | ✅ _common.scss |
| Card radius       | `$card_radius`                 | 12px   | ✅ _common.scss |
| Window/dialog radius | `$button_radius + 6`        | 15px   | ✅ for windows; AdwAlertDialog uses `$alert_radius: 18px` instead (see §1.4.4) |
| Frame/border width| libadwaita CSS `border-width`  | 1px    | ✅ |
| Focus ring width  | libadwaita CSS `outline-width` | 2px    | ✅ focus-ring mixin |
| Focus ring offset | libadwaita CSS `outline-offset`| -2px (inset) | ✅ `$offset: -$width` |
| Disabled opacity  | `--disabled-opacity`           | 0.5 (CSS: `50%`) | ✅ _colors.scss |
| Drop shadows      | libadwaita CSS `box-shadow`    | yes    | ✅ |

#### 1.4.4 Widget Metrics

All from **libadwaita CSS** (not system APIs):

| Widget          | Property           | Value         |   |
|-----------------|--------------------|---------------|---|
| Button          | CSS min-height     | 24px (34px with padding) | ✅ _buttons.scss |
| Entry (input)   | CSS min-height     | 34px          | ✅ _entries.scss |
| CheckButton     | indicator size     | 14px (20px with padding) | ✅ _checks.scss |
| Scale (slider)  | trough min-height  | 10px          | ✅ _scale.scss |
| Scale           | thumb diameter     | 20px          | ✅ _scale.scss |
| ProgressBar     | bar height         | 8px           | ✅ _progress-bar.scss |
| Notebook (tab)  | tab min height     | 30px          | ✅ _notebook.scss |
| Scrollbar       | slider/thumb width | 8px           | ✅ _scrolling.scss |
| Tooltip         | padding            | 6px vert / 10px horiz | ✅ _tooltip.scss |
| GtkSwitch       | thumb size         | 20 × 20px     | ✅ |
| GtkSwitch       | padding (track)    | 3px           | ✅ |
| GtkSwitch       | track radius       | 14px (pill)   | ✅ |
| GtkSwitch       | total track size   | ~46 × 26px (derived) | ✅ derived checks out |
| GtkSpinner      | default size       | 16 × 16px     | ✅ gtkspinner.c DEFAULT_SIZE=16 |
| GtkDropDown     | arrow size         | 16 × 16px     | ✅ _dropdowns.scss `min-height/min-width: 16px` |
| GtkDropDown     | box spacing        | 6px           | ✅ _dropdowns.scss `border-spacing: 6px` |
| AdwAlertDialog  | preferred width    | 300sp         | ✅ adw-alert-dialog.c |
| AdwAlertDialog  | max width          | 372sp (wide: 600sp) | ✅ adw-alert-dialog.c |
| AdwAlertDialog  | button spacing     | 12px          | ✅ _message-dialog.scss `.response-area { border-spacing: 12px }` |
| AdwAlertDialog  | message padding    | 24px sides, 32px top | ✅ _message-dialog.scss `.message-area` padding values |
| AdwAlertDialog  | button padding     | 24px (top: 12px) | ✅ _message-dialog.scss `.response-area { padding: 24px; padding-top: 12px }` |
| AdwAlertDialog  | border radius      | 18px (`$alert_radius`)  | ✅ confirmed in _message-dialog.scss; distinct from `$dialog_radius` (15px) |
| GtkExpander     | arrow size         | 16 × 16px     | ✅ _expanders.scss `min-width/min-height: 16px` |
| AdwExpanderRow  | header min-height  | 50px          | ✅ _lists.scss |
| Card (`.card`)  | border radius      | 12px          | ✅ = $card_radius |
| Button          | padding            | 5px 10px      | ✅ _buttons.scss |
| Entry (input)   | horizontal padding | 9px           | ✅ _entries.scss `padding-left: 9px; padding-right: 9px` |
| Menu item       | min-height         | 32px          | ✅ _menus.scss `modelbutton { min-height: 32px }` |
| Menu item       | padding            | 0 12px        | ✅ _menus.scss `padding: 0 $menu_padding`; `$menu_padding=12` from _common.scss |
| Notebook (tab)  | tab padding        | 3px 12px      | ✅ _notebook.scss `padding: 3px 12px` |
| Headerbar       | min-height         | 47px          | ✅ _header-bar.scss |

#### 1.4.5 Layout Spacing

libadwaita CSS defines specific per-widget margins and padding. There
is **no abstract spacing scale**. Specific values are set per CSS class.

#### 1.4.6 Icon Sizes

The active icon theme name is read from `org.gnome.desktop.interface
icon-theme` (default: `Adwaita`). ✅

GTK4 has **three** `GtkIconSize` enum values. Actual pixel sizes
come from theme CSS via `-gtk-icon-size`:

| `GtkIconSize`         | CSS class       | Adwaita default |   |
|-----------------------|-----------------|-----------------|---|
| `GTK_ICON_SIZE_INHERIT`| (parent)       | (inherited)     | ✅ |
| `GTK_ICON_SIZE_NORMAL`| `.normal-icons`  | 16px            | ✅ |
| `GTK_ICON_SIZE_LARGE` | `.large-icons`   | 32px            | ✅ |

Symbolic icons are designed at 16×16 SVG and rendered at 16, 32,
64, or 128px.

#### 1.4.7 Accessibility

| Setting              | Source                                            |   |
|----------------------|---------------------------------------------------|---|
| Text scaling factor  | `org.gnome.desktop.interface text-scaling-factor`  | ✅ |
| High contrast        | `org.gnome.desktop.a11y.interface high-contrast`   | ✅ |
| Reduce motion        | gsettings `enable-animations` / GtkSettings `gtk-enable-animations` / Portal `reduced-motion` | ✅ |
| Contrast preference  | Portal `org.freedesktop.appearance` `contrast`     | ✅ |

---

## Chapter 2: Cross-Platform Property Mapping

Maps OS-specific APIs from Chapter 1 to unified per-widget properties.
Every visible style property of every widget is listed — including
properties whose value is inherited from a global default (`←`).

Each platform cell shows **where the default value comes from**: a
named API/constant, a CSS class, a measured value, `←` a global
property name, or `**(none)**` if the platform has no such concept.

#### Property naming conventions

Every property name is self-describing. The suffix/pattern tells you
the type and meaning:

**Colors and fills:**
- `*_color` — a color value (e.g. `border.color`, `caret_color`, `line_color`)
- `*_background` — a background fill color (e.g. `hover_background`, `checked_background`, `selection_background`). The base background (`background_color`) uses `_color` suffix for consistency with other global colors.
- `*_text_color` — a text rendering color for a specific state or context, kept outside the font struct because it overrides only the color, not the typeface (e.g. `active_text_color`, `disabled_text_color`, `hover_text_color`, `accent_text_color`)

**Typography:**
- `font` — a typeface struct: family + size + weight + style + **color**. Every text needs both a typeface and a color to render, so color is part of the font struct. When a widget inherits `font ← defaults.font`, it inherits the color too.
- `font.family`, `font.size`, `font.weight`, `font.style`, `font.color` — individual font sub-properties, shown when at least one platform has a widget-specific value. `font.style` is a string enum: `Normal` (upright), `Italic`, or `Oblique`. `font.color` is the default-state text rendering color for this font.

**Measurement rules:**

All **dimension and spacing** values are in **logical pixels**
(scale-independent) unless the property definition explicitly states a
different type. Non-pixel property types: ratios (`line_height`,
`disabled_opacity`, `border.opacity` — dimensionless multipliers),
booleans (`border.shadow_enabled`, `overlay_mode`, `underline_enabled`),
enums/strings (`button_order`, `font.style`, `icon_set`, `icon_theme`),
and colors (`*_color`, `*_background`). Unit suffixes in cells
("px", "pt", bare number) are informal annotations reflecting the
platform's native terminology — all dimension values are logical
pixels regardless of suffix. macOS "pt" = AppKit points = logical
pixels; Windows "px" / "epx" = effective pixels = logical pixels;
bare numbers are also logical pixels. Two general rules eliminate
ambiguity for every dimension and spacing property:

1. **Outer-box rule for dimensions:** `min_width`, `max_width`,
   `min_height`, `max_height`, `row_height`, `bar_height`,
   `segment_height`, `header_height`, and any other *height/width of a
   widget or element* measure the **outer bounding box** — from the
   outside of the border on one side to the outside of the border on
   the other side (border + padding + content). Drop shadows, focus
   rings, and any other visual effects that extend beyond the border
   edge are **not** included. When a platform's source value is a
   content-area measurement rather than outer-box, the cell shows the
   source value first and annotates the outer-box equivalent in
   parentheses — e.g. "CSS min-height: 24 (outer-box: 34)". The
   canonical property value is always the **outer-box** number.

2. **Per-side rule for padding:** `border.padding_horizontal` and
   `border.padding_vertical` are always **per-side** values — the
   amount applied to EACH side independently.
   `border.padding_horizontal: 10` means 10 px on the left AND 10 px
   on the right (20 px total horizontal gap). When a platform has
   asymmetric padding (different left vs right, or different top vs
   bottom), the cell shows both values (e.g. "10 left / 6 right").

3. **GNOME "sp" unit:** Some GNOME/libadwaita values are suffixed
   "sp" (e.g. "300sp"). This is libadwaita's scale-independent pixel
   unit — it equals logical pixels at the default text scale factor
   (1.0) and scales proportionally with
   `org.gnome.desktop.interface text-scaling-factor`. For our
   purposes, **sp = logical pixels** (the text-scaling-factor is
   applied globally by the toolkit, not per-property by us).

**Border struct** (`border.*` sub-properties):

The border struct groups all frame/boundary properties of a widget.
Like `font`, it can appear as a single inherited row
(`border = ← defaults.border`) or expanded into sub-properties when
values differ per platform.

- `border.line_width` — stroke thickness of the border outline.
- `border.corner_radius` — corner rounding radius. `border.corner_radius_lg` is the larger variant used by popover/window/dialog containers.
- `border.color` — color of the border outline.
- `border.opacity` — opacity multiplier applied to the border color.
- `border.shadow_enabled` — whether the widget casts a drop shadow.
- `border.padding_horizontal` — per-side left/right space between the inner border edge and the widget's content (text, icon).
- `border.padding_vertical` — per-side top/bottom space between the inner border edge and the widget's content.

`defaults.border` provides: `line_width`, `corner_radius`,
`corner_radius_lg`, `color`, `opacity`, `shadow_enabled`. Padding has
no global default — it is always widget-specific.

**Content gaps and layout margins:**
- `icon_text_gap` — horizontal distance between an icon and the adjacent text label inside the widget.
- `label_gap` — distance between an indicator (checkbox/radio box) and its text label.
- `item_gap` — distance between adjacent child items in a container (toolbar items, etc.).
- `button_gap` — distance between adjacent action buttons (e.g. OK / Cancel in a dialog).
- `widget_gap` — default distance between sibling widgets in a layout container.
- `section_gap` — vertical distance between content sections.
- `container_margin` — default margin inside a nested layout container.
- `window_margin` — default margin inside a top-level window layout.

**Widget dimensions:**
- `min_width`, `max_width` — minimum/maximum outer width of the widget (see outer-box rule).
- `min_height`, `max_height` — minimum/maximum outer height of the widget (see outer-box rule).
- `row_height` — height of a single item row (menu item, list row).
- `bar_height` — total height of a toolbar bar.
- `track_height` — height of a slider or progress bar track groove.
- `track_width` — width of a switch track.
- `track_radius` — corner radius of the switch track. A value of half the `track_height` produces a pill/capsule shape.
- `thumb_diameter` — diameter of the circular slider/switch thumb knob.
- `thumb_width` — width of the scrollbar thumb element.
- `min_thumb_length` — minimum length of the scrollbar thumb along the scroll axis.
- `groove_width` — total width of the scrollbar groove (track area + margins).
- `divider_width` — width of the splitter divider handle area.
- `line_width` — stroke thickness of a separator line.
- `indicator_width` — side length of the checkbox indicator (square) or diameter of the radio indicator (circle).
- `arrow_icon_size` — size (width = height) of a dropdown arrow icon.
- `arrow_area_width` — total width of the clickable dropdown arrow area including its surrounding padding.
- `stroke_width` — stroke thickness of the spinner ring arc.
- `diameter` — default diameter of the spinner ring.
- `min_diameter` — minimum allowed spinner diameter.
- `segment_height` — height of each segment button in a segmented control.
- `separator_width` — width of the divider line between segments.
- `header_height` — height of an expander header row.
- `tick_mark_length` — length of slider tick marks, measured perpendicular to the track.

**Other dimensions:**
- `disabled_opacity` — opacity multiplier (0.0–1.0) applied to the entire widget when disabled.
- `icon_size` — display size (width = height) of icons within the widget.

**Dimensionless ratios:**
- `line_height` — ratio of line box height to font size (e.g. 1.19 means a 13px font produces ~15.5px line boxes). Typically derived from font metrics (sTypoAscender + |sTypoDescender| + sTypoLineGap) / unitsPerEm; Windows uses the Fluent type ramp value instead (Body 20px / 14px). **Not** in pixels — multiply by `font.size` to get the pixel line height.
- `border.opacity` — 0.0–1.0 multiplier applied to the border color, not pixels.

**Booleans and enums:**
- `border.shadow_enabled` — whether the widget renders a drop shadow.
- `overlay_mode` — boolean: `true` if the scrollbar uses overlay (auto-hiding) mode, `false` if persistent/legacy. Platform cells show how to query this (e.g. `NSScroller.preferredScrollerStyle`).
- `underline_enabled` — whether link text is underlined.
- `button_order` — platform convention for dialog button arrangement (primary left vs right).
- `icon_set` — which icon set to use.
- `icon_theme` — which icon theme to use.

**Named fonts (for widgets with multiple text areas):**
- `body_font` — typeface for the primary body text (e.g. dialog message body). Includes `body_font.color`.
- `title_font.*` — typeface sub-properties for the title/heading text. Includes `title_font.color`.
- `title_bar_font.*` — typeface sub-properties for the window title bar text. Includes `title_bar_font.color`.
- `item_font` — typeface for list/table row content text. Includes `item_font.color`.
- `header_font` — typeface for list/table column header text. Includes `header_font.color`.

**Text scale roles (§2.19 only):**
- `caption`, `section_heading`, `dialog_title`, `display` — these are
  content role names, not widget properties. Each maps a semantic text
  role to per-platform type ramp entries (size + weight).

### 2.1 Global Defaults

#### 2.1.1 Base Font

| Property       | macOS                               | Windows                                | KDE                        | GNOME                       |
|----------------|-------------------------------------|----------------------------------------|----------------------------|-----------------------------|
| `family`       | `+systemFontOfSize:` → family       | `lfMessageFont.lfFaceName`          | `[General] font` field 0 | `font-name` gsetting      |
| `size`         | `+systemFontOfSize:` → pointSize    | ↕ `abs(lfMessageFont.lfHeight)*72/dpi` | `[General] font` field 1 | `font-name` gsetting → size |
| `weight`       | `NSFontDescriptor` traits           | `lfMessageFont.lfWeight`            | `[General] font` field 4 | `font-name` gsetting → wt |
| `style`        | `NSFontDescriptor` traits → Normal  | `lfMessageFont.lfItalic` (0 = Normal) | `[General] font` style field | `font-name` gsetting → style |
| `color`        | `labelColor`                     | `UISettings(Foreground)`            | `[Colors:Window] ForegroundNormal` | **(Adwaita CSS)** body `color` |
| `line_height`  | 1.19 **(font metrics)** SF Pro sTypo (ascender+\|descender\|+lineGap)/UPM=(1950+494+0)/2048; macOS HIG specifies per-style line heights (e.g. body 13/16=1.23, headline 13/16=1.23) but these are design guidelines, not API values — the font metrics yield 1.19 | 1.43 **(Fluent)** Body 20px/14px      | 1.36 **(font metrics)** Noto Sans sTypo (ascender+\|descender\|+lineGap)/UPM=(1069+293+0)/1000 (Roboto-compatible metrics; lineGap=0) | ✅ Cantarell (pre-48): 1.2 **(font metrics)** — `USE_TYPO_METRICS` (fsSelection bit 7) is **not set**, so FreeType/HarfBuzz falls back to Win metrics: usWinAscent=983 (=739+244, lineGap folded), usWinDescent=217 → (983+217)/1000=1.2 (all three metric sets — sTypo (739+217+244)/1000, Win (983+217)/1000, hhea (983+217)/1000 — agree on 1.2); Adwaita Sans (GNOME 48+)=1.21 **(font metrics)** from Inter metrics: (1984+494+0)/2048 (`USE_TYPO_METRICS` IS set, lineGap=0) |

#### 2.1.2 Monospace Font

| Property       | macOS                               | Windows               | KDE                        | GNOME                            |
|----------------|-------------------------------------|-----------------------|----------------------------|----------------------------------|
| `family`       | `+monospacedSystemFont...` → family | **(none)** — preset: Cascadia Mono    | `[General] fixed` field 0 | `monospace-font-name` gsetting |
| `size`         | `+monospacedSystemFont...` → ptSize | **(none)** — preset: 14px             | `[General] fixed` field 1 | `monospace-font-name` → size   |
| `weight`       | `NSFontDescriptor` traits           | **(none)** — preset: 400              | `[General] fixed` field 4 | `monospace-font-name` → weight |
| `style`        | `NSFontDescriptor` traits → Normal  | **(none)** — preset: Normal           | `[General] fixed` style field | `monospace-font-name` → style |
| `color`        | ← `defaults.font.color`          | ← `defaults.font.color`              | ← `defaults.font.color`          | ← `defaults.font.color`          |

#### 2.1.3 Base Colors

| Property              | macOS                               | Windows                                             | KDE                                    | GNOME                      |
|-----------------------|-------------------------------------|------------------------------------------------------|----------------------------------------|----------------------------|
| `background_color`          | `windowBackgroundColor`          | `UISettings(Background)`                          | `[Colors:Window] BackgroundNormal`  | **(Adwaita CSS)**          |
| `text_color`          | `labelColor`                     | `UISettings(Foreground)`                          | `[Colors:Window] ForegroundNormal`  | **(Adwaita CSS)**          |
| `accent_color`              | `controlAccentColor`             | `UISettings(Accent)`                              | `[General] AccentColor` (propagated to `DecorationFocus`) | Portal `accent-color`   |
| `accent_text_color`   | `alternateSelectedControlTextColor` | **(Fluent)** `TextOnAccentFillColorPrimary` (L #ffffff D #000000) | `[Colors:Selection] ForegroundNormal` | **(Adwaita CSS)**        |
| `surface_color`             | `controlBackgroundColor`         | **(Fluent)** CardBackgroundFillColorDefault           | `[Colors:View] BackgroundNormal`    | **(Adwaita CSS)**          |
| `border.color`              | `separatorColor`                 | **(Fluent)** CardStrokeColorDefault                  | **(preset)** — derived from background | **(Adwaita CSS)**          |
| `muted_color`               | `secondaryLabelColor`            | **(Fluent)** TextFillColorSecondary                  | `[Colors:Window] ForegroundInactive`| **(Adwaita CSS)**          |
| `shadow_color`              | `shadowColor`                    | **(Fluent)** two-layer per elevation (from [Fluent 2 spec](https://fluent2.microsoft.design/elevation)): low L=14%/14% D=28%/14%; high L=24%/20% D=28%/20% (note: FluentUI React web tokens use different opacities) | **(none)** — preset: #00000040/#60     | **(Adwaita CSS)**          |
| `link_color`                | `linkColor`                      | **(Fluent)** HyperlinkForeground                     | `[Colors:View] ForegroundLink`      | **(Adwaita CSS)**          |
| `selection_background`           | `selectedContentBackgroundColor` | `COLOR_HIGHLIGHT`                                 | `[Colors:Selection] BackgroundNormal`| **(Adwaita CSS)**         |
| `selection_text_color`| `alternateSelectedControlTextColor` | `COLOR_HIGHLIGHTTEXT`                             | `[Colors:Selection] ForegroundNormal`| **(Adwaita CSS)**         |
| `selection_inactive_background`  | `unemphasizedSelectedContentBackgroundColor` | **(none)** — reduced emphasis / `COLOR_BTNFACE` | **(none)** — selection bg unchanged on focus loss | **(none)** — `:backdrop` CSS state handles this |
| `text_selection_background`      | `selectedTextBackgroundColor`    | ← `defaults.selection_background`                 | ← `defaults.selection_background`    | ← `defaults.selection_background` |
| `text_selection_color` | `selectedTextColor`              | ← `defaults.selection_text_color`                 | ← `defaults.selection_text_color`    | ← `defaults.selection_text_color` |
| `disabled_text_color` | `disabledControlTextColor`       | **(Fluent)** TextFillColorDisabled                   | `[Colors:View] ForegroundInactive`  | **(Adwaita CSS)**          |

`defaults.text_color` and `defaults.font.color` are the same value
(both resolve to the primary text color). `defaults.text_color` is kept
here as a standalone color token so that state-override properties
(`hover_text_color`, `disabled_text_color`, `active_text_color`, etc.)
have a clear reference. Per-widget base text colors use `font.color`.

macOS provides separate APIs for content/row selection
(`selectedContentBackgroundColor` + `alternateSelectedControlTextColor`)
and text-range selection (`selectedTextBackgroundColor` +
`selectedTextColor`). Other platforms use the same colors for both
contexts. `text_selection_*` tokens expose this distinction — widgets
with editable text (§2.4) inherit `text_selection_*`, while list/sidebar
widgets (§2.12, §2.15) inherit `selection_*`.

#### 2.1.4 Status Colors

| Property              | macOS                | Windows                                                | KDE                                     | GNOME              |
|-----------------------|----------------------|--------------------------------------------------------|-----------------------------------------|--------------------|
| `danger_color`              | `systemRedColor`  | ✅ **(Fluent)** SystemFillColorCritical L #c42b1c D #ff99a4 | `[Colors:View] ForegroundNegative`   | **(Adwaita CSS)**  |
| `danger_text_color`   | `labelColor` ¹    | **(Fluent)** L #ffffff D #1a1a1a ² — no dedicated WinUI3 resource | `[Colors:Window] ForegroundNormal` ¹ | **(Adwaita CSS)** ¹ |
| `warning_color`             | `systemOrangeColor` | ✅ **(Fluent)** SystemFillColorCaution L #9d5d00 D #fce100 | `[Colors:View] ForegroundNeutral`  | **(Adwaita CSS)**  |
| `warning_text_color`  | `labelColor` ¹    | **(Fluent)** L #1a1a1a D #1a1a1a ² — no dedicated WinUI3 resource | `[Colors:Window] ForegroundNormal` ¹ | **(Adwaita CSS)** ¹ |
| `success_color`             | `systemGreenColor` | ✅ **(Fluent)** SystemFillColorSuccess L #0f7b0f D #6ccb5f | `[Colors:View] ForegroundPositive`  | **(Adwaita CSS)**  |
| `success_text_color`  | `labelColor` ¹    | **(Fluent)** L #ffffff D #1a1a1a ² — no dedicated WinUI3 resource | `[Colors:Window] ForegroundNormal` ¹ | **(Adwaita CSS)** ¹ |
| `info_color`                | `systemBlueColor` | **(Fluent)** SystemFillColorAttention (accent-derived)    | `[Colors:View] ForegroundActive`     | **(Adwaita CSS)**  |
| `info_text_color`     | `labelColor` ¹    | **(Fluent)** L #ffffff D #1a1a1a ² — no dedicated WinUI3 resource | `[Colors:Window] ForegroundNormal` ¹ | **(Adwaita CSS)** ¹ |

**Status foreground semantic mismatch:** The `*_text_color` rows mix two
different concepts across platforms. ¹ macOS, KDE, and GNOME provide the
**normal body foreground** — suitable as text color *alongside* a status
indicator (e.g. error-message text next to a red icon), **not** as text
*on* a status-colored background. ² Windows provides a **contrast
foreground for text on the status background** (white-on-dark-red in light
mode, near-black-on-light-pink in dark mode). No platform has a dedicated
"text-on-status-background" API; consumers must pick the interpretation
that matches their use case and derive the other (e.g. ensure contrast
against the `danger_color` color if using it as a fill).

#### 2.1.5 Focus Ring

| Property  | macOS                         | Windows                                           | KDE                             | GNOME                   |
|-----------|-------------------------------|---------------------------------------------------|---------------------------------|-------------------------|
| `focus_ring_color`   | `keyboardFocusIndicatorColor` | `UISettings(Accent)` (same as accent)          | `[Colors:View] DecorationFocus` | Adwaita `@accent_color` |
| `focus_ring_width`   | 3px **(measured)**            | Win32 `SM_CXFOCUSBORDER` ↕ =1px / Fluent visual=2px | Breeze: 1.001px (stroke); 2px margin | libadwaita CSS: 2px     |
| `focus_ring_offset`  | -1px **(measured)** (inset)   | Fluent: 0px default margin (outset)               | Breeze: 2px margin (outset)     | libadwaita CSS: -2px (inset) |

#### 2.1.6 Global Geometry

| Property           | macOS          | Windows                   | KDE            | GNOME            |
|--------------------|----------------|---------------------------|----------------|------------------|
| `border.corner_radius`           | 5px **(measured)** | Fluent: 4px               | Breeze: 5px    | Adwaita: 9px     |
| `border.corner_radius_lg`        | 10px **(measured)**| Fluent: 8px               | **(none)** — preset | Adwaita: 15px |
| `border.line_width` | 0.5px **(measured)** | ↕ `SM_CXBORDER` (DPI-aware) | Breeze: 1.001px | Adwaita: 1px     |
| `disabled_opacity` | ≈0.25–0.3 **(measured)** (via text color alpha, not global opacity) | Fluent: per-control (≈0.3) | **(none)** — palette blending | Adwaita: 0.5 |
| `border.opacity`   | 0.2 **(preset)** | 0.14 **(preset)**       | 0.2 **(preset)** | 0.15 **(preset)**|
| `border.shadow_enabled`   | yes            | yes                       | yes            | yes              |

#### 2.1.7 Accessibility

| Property              | macOS                                                    | Windows                                  | KDE                          | GNOME                           |
|-----------------------|----------------------------------------------------------|------------------------------------------|------------------------------|----------------------------------|
| `text_scaling_factor` | Accessibility text size pref (macOS 14+; **very limited** — affects only a few Apple apps; `preferredFont(forTextStyle:)` still returns fixed sizes; not comparable to other platforms' system-wide text scaling) | `UISettings.TextScaleFactor` (text-only) + DPI / 96 (display) | `forceFontDPI` / 96       | `text-scaling-factor` gsetting |
| `reduce_motion`       | `accessibilityDisplayShouldReduceMotion`                  | `SPI_GETCLIENTAREAANIMATION` (Bool)      | `AnimationDurationFactor` = 0 | gsettings `enable-animations` (Bool)  |
| `high_contrast`       | `accessibilityDisplayShouldIncreaseContrast`              | `SPI_GETHIGHCONTRAST` (struct w/ flags)  | **(none)**                   | `a11y.interface high-contrast`   |
| `reduce_transparency` | `accessibilityDisplayShouldReduceTransparency`            | **(none)** — high contrast disables it   | **(none)**                   | **(none)**                       |

#### 2.1.8 Icon Sizes

| Context          | macOS               | Windows                    | KDE                        | GNOME                   |
|------------------|----------------------|----------------------------|----------------------------|-------------------------|
| `toolbar`        | 32pt (reg) / 24 (sm) | Fluent AppBarButton: 20    | `MainToolbar`: 22       | `GTK_ICON_SIZE_NORMAL`: 16 |
| `small`          | sidebar: 16–20pt     | ↕ `SM_CXSMICON`: 16       | `Small`: 16             | `GTK_ICON_SIZE_NORMAL`: 16 |
| `large`          | **(none)**           | ↕ `SM_CXICON`: 32         | `Desktop`: 48 (Breeze default) | `GTK_ICON_SIZE_LARGE`: 32  |
| `dialog`         | **(none)**           | **(none)**                 | `Dialog`: 32            | **(none)** — 48 (GTK3 legacy) |
| `panel`          | **(none)**           | **(none)**                 | `Panel`: 48 (Breeze default = C++ fallback)   | **(none)**              |

---

### 2.2 Window / Application Chrome

| Property                 | macOS                                         | Windows                                         | KDE                              | GNOME                                            |
|--------------------------|-----------------------------------------------|--------------------------------------------------|----------------------------------|--------------------------------------------------|
| `background_color`             | ← `defaults.background_color`                      | ← `defaults.background_color`                         | ← `defaults.background_color`         | ← `defaults.background_color`                         |
| `border.color`                 | ← `defaults.border.color`                           | ← `defaults.border.color` (Win10+: `COLOR_ACTIVEBORDER` unsupported) | `[WM]` decoration theme colors | **(Adwaita CSS)** window border                 |
| `border.line_width`  | ← `defaults.border.line_width`     | ← `defaults.border.line_width`   | ← `defaults.border.line_width`            | ← `defaults.border.line_width`     |
| `title_bar_background`   | **(measured)** ≈ `defaults.surface_color`         | `DwmGetColorizationColor`                     | `[WM] activeBackground`       | libadwaita `headerbar` bg                        |
| `title_bar_font.family`  | `+titleBarFontOfSize:` → family               | `lfCaptionFont.lfFaceName`                    | `[WM] activeFont` field 0     | `titlebar-font` gsetting → family             |
| `title_bar_font.size`    | `+titleBarFontOfSize:` → pointSize            | ↕ `abs(lfCaptionFont.lfHeight)*72/dpi`        | `[WM] activeFont` field 1     | `titlebar-font` gsetting → size               |
| `title_bar_font.weight`  | `+titleBarFontOfSize:` → Bold (700)            | `lfCaptionFont.lfWeight` (varies; see §1.2.1) | `[WM] activeFont` field 4     | `titlebar-font` gsetting → weight (typically 700)|
| `title_bar_font.style`   | `+titleBarFontOfSize:` → Normal               | `lfCaptionFont.lfItalic` (0 = Normal)         | `[WM] activeFont` style field | `titlebar-font` gsetting → style              |
| `title_bar_font.color`   | `windowFrameTextColor`                        | `COLOR_CAPTIONTEXT`                           | `[WM] activeForeground`       | libadwaita `headerbar` fg                     |
| `inactive_title_bar_background`  | **(none)** — system-managed dimming            | `COLOR_INACTIVECAPTION`                       | `[WM] inactiveBackground`     | **(none)** — `:backdrop` CSS state               |
| `inactive_title_bar_text_color`  | **(none)** — system-managed                    | `COLOR_INACTIVECAPTIONTEXT`                   | `[WM] inactiveForeground`     | **(none)** — `:backdrop` CSS state               |
| `border.corner_radius`                 | 10px **(measured)** through Sequoia (compositor-controlled, not inherited — happens to equal `defaults.border.corner_radius_lg`); Tahoe (26): variable per window style, no public API (see appendix) | ← `defaults.border.corner_radius_lg`                          | ← `defaults.border.corner_radius_lg`         | ← `defaults.border.corner_radius_lg`                          |
| `border.shadow_enabled`                 | ← `defaults.border.shadow_enabled`                   | ← `defaults.border.shadow_enabled`                     | ← `defaults.border.shadow_enabled`    | ← `defaults.border.shadow_enabled`                     |
| `border.padding_horizontal` | **(none)** — use §2.20 layout margins | **(none)** — use §2.20 layout margins | **(none)** — use §2.20 layout margins | **(none)** — use §2.20 layout margins |
| `border.padding_vertical` | **(none)** — use §2.20 layout margins | **(none)** — use §2.20 layout margins | **(none)** — use §2.20 layout margins | **(none)** — use §2.20 layout margins |

### 2.3 Button

| Property            | macOS                         | Windows                     | KDE                                  | GNOME                         |
|---------------------|-------------------------------|-----------------------------|--------------------------------------|-------------------------------|
| `background_color`        | `controlColor`             | `COLOR_BTNFACE`          | `[Colors:Button] BackgroundNormal` | libadwaita `.button` bg      |
| `font`              | ← `defaults.font`            | ← `defaults.font`          | ← `defaults.font`                   | ← `defaults.font`            |
| `font.color`        | `controlTextColor`         | `COLOR_BTNTEXT`          | `[Colors:Button] ForegroundNormal` | libadwaita `.button` fg      |
| `border.color`            | ← `defaults.border.color`          | ← `defaults.border.color`        | ← `defaults.border.color`                 | ← `defaults.border.color`          |
| `border.line_width`      | ← `defaults.border.line_width`     | ← `defaults.border.line_width`   | ← `defaults.border.line_width`            | ← `defaults.border.line_width`     |
| `min_width`         | **(none)** — sizes to content | **(none)** — sizes to content | `Button_MinWidth` = 80              | **(Adwaita CSS)**: none       |
| `min_height`        | NSButton intrinsic: 22        | WinUI3: 27 (derived: 14 text + 5+6 pad + 2 border; no explicit MinHeight resource) | **(none)** — sizes to content        | **(Adwaita CSS)**: CSS min-height: 24 (outer-box: 34) |
| `border.padding_horizontal`| NSButton: ~8 **(WebKit)**     | WinUI3: 11                  | `Button_MarginWidth` = 6            | **(Adwaita CSS)**: 10         |
| `border.padding_vertical`  | 3 **(measured)** (22−16)/2    | WinUI3: 5 top / 6 bottom   | 5 **(measured)** Breeze frame+margin | **(Adwaita CSS)**: 5          |
| `border.corner_radius`            | ← `defaults.border.corner_radius`          | ← `defaults.border.corner_radius`        | ← `defaults.border.corner_radius`                 | ← `defaults.border.corner_radius`          |
| `icon_text_gap`      | 4 **(measured)** AppKit       | WinUI3: 8                   | `Button_ItemSpacing` = 4            | **(Adwaita CSS)**: 8          |
| `primary_background`        | ← `defaults.accent_color`          | ← `defaults.accent_color`        | ← `defaults.accent_color`                 | ← `defaults.accent_color`          |
| `primary_text_color`        | ← `defaults.accent_text_color`| ← `defaults.accent_text_color`| ← `defaults.accent_text_color`   | ← `defaults.accent_text_color`|
| `disabled_opacity`  | ← `defaults.disabled_opacity`| ← `defaults.disabled_opacity`| ← `defaults.disabled_opacity`     | ← `defaults.disabled_opacity`|
| `border.shadow_enabled`            | ← `defaults.border.shadow_enabled`  | ← `defaults.border.shadow_enabled`| ← `defaults.border.shadow_enabled`         | ← `defaults.border.shadow_enabled`  |
| `hover_background`  | **(measured)** subtle CoreUI highlight | **(Fluent)** `SubtleFillColorSecondary` overlay | `[Colors:Button] DecorationHover` blend | **(Adwaita CSS)** `.button:hover` bg |
| `hover_text_color`  | ← `font.color` (no change)         | ← `font.color` (no change) | ← `font.color` (no change)              | ← `font.color` (no change)          |

### 2.4 Text Input

| Property              | macOS                            | Windows               | KDE                                  | GNOME                         |
|-----------------------|----------------------------------|-----------------------|--------------------------------------|-------------------------------|
| `background_color`          | `textBackgroundColor`         | `COLOR_WINDOW`     | `[Colors:View] BackgroundNormal`  | libadwaita `.entry` bg        |
| `font`                | ← `defaults.font`               | ← `defaults.font`    | ← `defaults.font`                   | ← `defaults.font`            |
| `font.color`          | `textColor`                   | `COLOR_WINDOWTEXT` | `[Colors:View] ForegroundNormal`  | libadwaita `.entry` fg        |
| `border.color`              | ← `defaults.border.color`             | ← `defaults.border.color`  | ← `defaults.border.color`                 | ← `defaults.border.color`          |
| `border.line_width`  | ← `defaults.border.line_width`     | ← `defaults.border.line_width`   | ← `defaults.border.line_width`            | ← `defaults.border.line_width`     |
| `placeholder_color`         | `placeholderTextColor`        | **(Fluent)** TextPlaceholderColor | `[Colors:View] ForegroundInactive` | libadwaita `.dim-label`      |
| `caret_color`               | `textInsertionPointColor` (macOS 14+; pre-14: `controlTextColor` via `NSTextView.insertionPointColor`) | `foreground` (system default) | `[Colors:View] DecorationFocus`   | libadwaita `@accent_color`   |
| `selection_background`           | ← `defaults.text_selection_background`     | ← `defaults.text_selection_background`| ← `defaults.text_selection_background`         | ← `defaults.text_selection_background`  |
| `selection_text_color`| ← `defaults.text_selection_color`| ← `defaults.text_selection_color`| ← `defaults.text_selection_color`| ← `defaults.text_selection_color`|
| `min_height`          | NSTextField intrinsic: 22        | WinUI3 TextBox: 32    | **(none)** — sizes to content        | **(Adwaita CSS)**: 34         |
| `border.padding_horizontal`  | NSTextField: 4                   | WinUI3: 10 left / 6 right | `LineEdit_FrameWidth` = 6            | **(Adwaita CSS)**: 9          |
| `border.padding_vertical`    | 3 **(measured)** (22−16)/2       | WinUI3: 5 top / 6 bottom | 3 **(measured)** Breeze frame        | **(Adwaita CSS)**: 0 (CSS sets no vertical padding; visual whitespace comes from `min-height: 34` centering the text) |
| `border.corner_radius`              | ← `defaults.border.corner_radius`             | ← `defaults.border.corner_radius`  | ← `defaults.border.corner_radius`                 | ← `defaults.border.corner_radius`          |
| `border.shadow_enabled` | **(none)** — no shadow | **(none)** — no shadow | **(none)** — no shadow | **(none)** — no shadow |
| `disabled_opacity`  | ← `defaults.disabled_opacity`| ← `defaults.disabled_opacity`| ← `defaults.disabled_opacity`     | ← `defaults.disabled_opacity`|

### 2.5 Checkbox / Radio Button

| Property        | macOS                     | Windows                                      | KDE                                   | GNOME                    |
|-----------------|---------------------------|----------------------------------------------|---------------------------------------|--------------------------|
| `background_color`    | **(measured)** white       | **(Fluent)** `ControlAltFillColorSecondary`  | `[Colors:Button] BackgroundNormal` | **(Adwaita CSS)** check bg|
| `font`               | ← `defaults.font`          | ← `defaults.font`          | ← `defaults.font`           | ← `defaults.font`           |
| `font.color`          | ← `defaults.font.color`    | ← `defaults.font.color`    | ← `defaults.font.color`     | ← `defaults.font.color`     |
| `border.color`        | **(measured)** gray outline| **(Fluent)** `ControlStrongStrokeColorDefault`| ← `defaults.border.color`                 | **(Adwaita CSS)** check border|
| `border.line_width`  | ← `defaults.border.line_width`     | ← `defaults.border.line_width`   | ← `defaults.border.line_width`            | ← `defaults.border.line_width`     |
| `indicator_color`| white (#ffffff)           | **(Fluent)** `TextOnAccentFillColorPrimary`  | `[Colors:Selection] ForegroundNormal`| **(Adwaita CSS)** white |
| `indicator_width`| NSButton checkbox: 14     | WinUI3 CheckBox: 20                          | `CheckBox_Size` = 20                 | libadwaita CSS: 14       |
| `label_gap`       | AppKit: 4                 | WinUI3: 8           | `CheckBox_ItemSpacing` = 4       | **(Adwaita CSS)**: 8     |
| `border.corner_radius`        | ← `defaults.border.corner_radius`      | ← `defaults.border.corner_radius`| ← `defaults.border.corner_radius`              | ← `defaults.border.corner_radius`     |
| `border.shadow_enabled` | **(none)** — no shadow | **(none)** — no shadow | **(none)** — no shadow | **(none)** — no shadow |
| `border.padding_horizontal` | **(none)** — checkmark fills indicator | **(none)** — checkmark fills indicator | **(none)** — checkmark fills indicator | **(none)** — checkmark fills indicator |
| `border.padding_vertical` | **(none)** — checkmark fills indicator | **(none)** — checkmark fills indicator | **(none)** — checkmark fills indicator | **(none)** — checkmark fills indicator |
| `checked_background`   | ← `defaults.accent_color`      | ← `defaults.accent_color`    | ← `defaults.accent_color`         | ← `defaults.accent_color`    |
| `disabled_opacity`  | ← `defaults.disabled_opacity`| ← `defaults.disabled_opacity`| ← `defaults.disabled_opacity`     | ← `defaults.disabled_opacity`|

Radio buttons use the same colors but with circular `border.corner_radius`.

### 2.6 Menu

| Property            | macOS                          | Windows                              | KDE                                    | GNOME                       |
|---------------------|--------------------------------|--------------------------------------|----------------------------------------|-----------------------------|
| `background_color`        | **(measured)** ≈ `defaults.background_color` (vibrancy) | `COLOR_MENU`          | `[Colors:Window] BackgroundNormal`  | libadwaita `popover.menu` bg|
| `separator_color`         | **(measured)** = `separatorColor`               | ← `defaults.border.color`        | ← `defaults.border.color`                   | **(Adwaita CSS)** separator |
| `font.family`       | `+menuFontOfSize:` → family   | `lfMenuFont.lfFaceName`           | `[General] menuFont` field 0        | ← `defaults.font`          |
| `font.size`         | `+menuFontOfSize:` → pointSize| ↕ `abs(lfMenuFont.lfHeight)*72/dpi` | `[General] menuFont` field 1      | ← `defaults.font`          |
| `font.weight`       | `+menuFontOfSize:` → weight   | `lfMenuFont.lfWeight`             | `[General] menuFont` field 4        | ← `defaults.font`          |
| `font.style`        | `+menuFontOfSize:` → Normal   | `lfMenuFont.lfItalic` (0 = Normal)| `[General] menuFont` style field    | ← `defaults.font`          |
| `font.color`        | **(measured)** = `labelColor`  | `COLOR_MENUTEXT`                  | `[Colors:Window] ForegroundNormal`  | libadwaita `popover.menu` fg|
| `row_height`       | NSMenuItem: 22                 | WinUI3: touch mode=31 (14 text + 8+9 pad), mouse/narrow mode=23 (14 text + 4+5 pad); no explicit MinHeight resource — derived from padding | **(none)** — sizes to font             | **(Adwaita CSS)**: 32       |
| `border.padding_horizontal`| NSMenuItem: 12                 | WinUI3: 11                           | `MenuItem_MarginWidth` = 4             | **(Adwaita CSS)**: 12 (`$menu_padding`) |
| `border.padding_vertical`  | 3 **(measured)** (22−16)/2     | **(Fluent)** touch: 8 top / 9 bottom, mouse: 4 top / 5 bottom (MenuFlyoutItem padding) | `MenuItem_MarginHeight` = 4            | **(Adwaita CSS)**: 0 (vertical space from min-height) |
| `border.color` | **(none)** — popup border from §2.16 | **(none)** — popup border from §2.16 | **(none)** — popup border from §2.16 | **(none)** — popup border from §2.16 |
| `border.line_width` | **(none)** — popup border from §2.16 | **(none)** — popup border from §2.16 | **(none)** — popup border from §2.16 | **(none)** — popup border from §2.16 |
| `border.corner_radius` | **(none)** — items are rectangular | **(none)** — items are rectangular | **(none)** — items are rectangular | **(none)** — items are rectangular |
| `border.shadow_enabled` | **(none)** — popup shadow from §2.16 | **(none)** — popup shadow from §2.16 | **(none)** — popup shadow from §2.16 | **(none)** — popup shadow from §2.16 |
| `icon_text_gap`      | 4 **(measured)** AppKit layout | WinUI3: 12                           | 8 **(Breeze src)** icon-text gap       | **(Adwaita CSS)**: 8        |
| `icon_size`         | 16 **(cross-project)** — no Apple API constant; Qt/Firefox/Ladybird/Godot all use 16×16 for NSMenuItem images | ↕ `SM_CXSMICON`: 16                 | `Small`: 16                         | `GTK_ICON_SIZE_NORMAL`: 16  |
| `hover_background`  | `selectedContentBackgroundColor` | **(Fluent)** `SubtleFillColorSecondary` | `[Colors:Selection] BackgroundNormal` | **(Adwaita CSS)** `:hover` modelbutton bg |
| `hover_text_color`  | `selectedMenuItemTextColor` (white) | ← `defaults.text_color` (no change) | `[Colors:Selection] ForegroundNormal` | **(Adwaita CSS)** `:hover` fg (no change) |
| `disabled_text_color`| `disabledControlTextColor` | **(Fluent)** `TextFillColorDisabled` | `[Colors:Window] ForegroundInactive` | **(Adwaita CSS)** `:disabled` fg |

### 2.7 Tooltip

| Property      | macOS                                   | Windows             | KDE                                 | GNOME                   |
|---------------|-----------------------------------------|---------------------|--------------------------------------|-------------------------|
| `background_color`  | **(preset)** L #2c2c2e D #3a3a3c       | `COLOR_INFOBK`   | `[Colors:Tooltip] BackgroundNormal` | libadwaita `.tooltip` bg|
| `font.family` | `+toolTipsFontOfSize:` → family        | ← `defaults.font`  | ← `defaults.font`                   | ← `defaults.font`      |
| `font.size`   | `+toolTipsFontOfSize:` → ptSize        | ← `defaults.font`  | ← `defaults.font`                   | ← `defaults.font`      |
| `font.weight` | `+toolTipsFontOfSize:` → weight        | ← `defaults.font`  | ← `defaults.font`                   | ← `defaults.font`      |
| `font.style`  | `+toolTipsFontOfSize:` → Normal        | ← `defaults.font`  | ← `defaults.font`                   | ← `defaults.font`      |
| `font.color`  | **(preset)** #ffffff (both variants)      | `COLOR_INFOTEXT`  | `[Colors:Tooltip] ForegroundNormal` | libadwaita `.tooltip` fg|
| `border.padding_horizontal` | NSToolTipManager: 4               | WinUI3: 9            | `ToolTip_FrameWidth` = 3            | **(Adwaita CSS)**: 10       |
| `border.padding_vertical`   | NSToolTipManager: 4               | WinUI3: 6 top / 8 bottom (`ToolTipBorderPadding=9,6,9,8`) | `ToolTip_FrameWidth` = 3            | **(Adwaita CSS)**: 6        |
| `max_width`   | 300 **(measured)** macOS Sonoma         | WinUI3: 320         | **(none)** — preset: 300             | **(none)** — preset: 360 |
| `border.corner_radius`      | ← `defaults.border.corner_radius`                    | ← `defaults.border.corner_radius`| ← `defaults.border.corner_radius`                  | ← `defaults.border.corner_radius`    |
| `border.color`      | **(measured)** subtle frame             | **(Fluent)** `ToolTipBorderBrush`| ← `defaults.border.color`           | **(Adwaita CSS)** `.tooltip` border|
| `border.line_width`  | ← `defaults.border.line_width`     | ← `defaults.border.line_width`   | ← `defaults.border.line_width`            | ← `defaults.border.line_width`     |
| `border.shadow_enabled`      | yes (system popup shadow)               | yes (elevation)     | yes (KWin compositor)                 | **(Adwaita CSS)** box-shadow|

`max_width` measures the tooltip's **outer bounding box** width in
logical pixels (per the outer-box rule) — from the outside of the
border on one side to the outside of the border on the other side.
It does **not** measure text character count. Text that exceeds this
width wraps to multiple lines. macOS value is measured from rendered
tooltips; WinUI3 value is a XAML `MaxWidth` property; KDE and GNOME
have no platform limit — preset values are our defaults.

### 2.8 Scrollbar

| Property          | macOS                              | Windows                   | KDE                         | GNOME                      |
|-------------------|------------------------------------|---------------------------|-----------------------------|----------------------------|
| `track_color`           | mode-dependent: overlay=transparent, legacy=light gray **(measured)** | transparent               | ← `defaults.background_color`   | **(Adwaita CSS)** scrollbar|
| `thumb_color`           | `#80808080` **(measured)** Sonoma  | `#c2c2c2` **(measured)**  | **(Breeze src)** thumb color| **(Adwaita CSS)** scrollbar|
| `thumb_hover_color`     | `#60606080` **(measured)** Sonoma  | `#a0a0a0` **(measured)**  | **(Breeze src)** thumb hover| **(Adwaita CSS)** :hover   |
| `groove_width`           | mode-dependent: legacy=16 (persistent), overlay=7 (auto-hiding); see `overlay_mode` | ↕ `SM_CXVSCROLL` (DPI-aware)| `ScrollBar_Extend` = 21  | slider: 8 + margins        |
| `min_thumb_length`| 40 **(measured)** legacy mode      | ↕ `SM_CYVTHUMB` (DPI-aware) | `ScrollBar_MinSliderHeight` = 20 | **(Adwaita CSS)**: 40 |
| `thumb_width`    | mode-dependent: legacy=16 (fills groove), overlay=7 (idle) / 11 (hover) | ↕ `SM_CXVSCROLL` (same)    | `ScrollBar_SliderWidth` = 8| **(Adwaita CSS)**: 8      |
| `overlay_mode`    | `NSScroller.preferredScrollerStyle` (.overlay/.legacy) | **(none)** — always persistent | **(none)** — always persistent | gsettings `overlay-scrolling` / `gtk-overlay-scrolling` |

### 2.9 Slider

| Property       | macOS              | Windows         | KDE                           | GNOME                  |
|----------------|--------------------|-----------------|-------------------------------|------------------------|
| `fill_color`         | ← `defaults.accent_color`| ← `defaults.accent_color`| ← `defaults.accent_color`       | ← `defaults.accent_color`   |
| `track_color`        | ← `defaults.muted_color` | ← `defaults.muted_color` | ← `defaults.muted_color`        | ← `defaults.muted_color`    |
| `thumb_color`        | ← `defaults.surface_color`| ← `defaults.surface_color`| ← `defaults.surface_color`     | ← `defaults.surface_color`  |
| `track_height` | NSSlider: 5        | WinUI3: 4       | `Slider_GrooveThickness` = 6 | libadwaita `.scale`: 10 |
| `thumb_diameter`   | NSSlider knob: 21  | WinUI3: 18      | `Slider_ControlThickness` = 20| libadwaita: 20        |
| `tick_mark_length`  | NSSlider: 8        | WinUI3: 4       | `Slider_TickLength` = 8      | **(none)** — no ticks  |
| `disabled_opacity`  | ← `defaults.disabled_opacity`| ← `defaults.disabled_opacity`| ← `defaults.disabled_opacity`     | ← `defaults.disabled_opacity`|

### 2.10 Progress Bar

| Property    | macOS                 | Windows             | KDE                         | GNOME                        |
|-------------|-----------------------|---------------------|-----------------------------|------------------------------|
| `fill_color`      | ← `defaults.accent_color`  | ← `defaults.accent_color` | ← `defaults.accent_color`        | ← `defaults.accent_color`         |
| `track_color`     | ← `defaults.muted_color`   | ← `defaults.muted_color`  | ← `defaults.muted_color`         | ← `defaults.muted_color`          |
| `track_height`    | NSProgressIndicator: 6| WinUI3: visual track groove=1 (`ProgressBarTrackHeight`); minimum control height=3 (`ProgressBarMinHeight`) — `track_height` is the groove: 1 | `ProgressBar_Thickness` = 6| libadwaita `.progressbar`: 8 |
| `min_width` | **(none)** — no minimum | **(none)** — no minimum | **(none)** — no minimum     | **(Adwaita CSS)**: 80       |
| `border.corner_radius`    | ← `defaults.border.corner_radius`  | ← `defaults.border.corner_radius`| ← `defaults.border.corner_radius`         | ← `defaults.border.corner_radius`         |
| `border.shadow_enabled` | **(none)** — no shadow | **(none)** — no shadow | **(none)** — no shadow | **(none)** — no shadow |

### 2.11 Tab Bar

| Property            | macOS               | Windows             | KDE                         | GNOME                |
|---------------------|---------------------|---------------------|-----------------------------|----------------------|
| `background_color`        | ← `defaults.background_color` | ← `defaults.background_color`| ← `defaults.background_color` | ← `defaults.background_color` |
| `active_background` | ← `defaults.background_color` | ← `defaults.background_color`| ← `defaults.background_color` | ← `defaults.background_color` |
| `active_text_color` | ← `defaults.text_color` | ← `defaults.text_color`| ← `defaults.text_color` | ← `defaults.text_color` |
| `bar_background`    | ← `defaults.background_color` | ← `defaults.background_color`| ← `defaults.background_color` | ← `defaults.background_color` |
| `min_width`         | **(none)** — sizes to label | **(none)** — sizes to label | `TabBar_TabMinWidth` = 80  | **(Adwaita CSS)**: none |
| `min_height`        | NSTabView: 24       | WinUI3: 32          | `TabBar_TabMinHeight` = 30 | **(Adwaita CSS)**: 30  |
| `border.padding_horizontal`| NSTabView: 12       | WinUI3: 8 left / 4 right | `TabBar_TabMarginWidth` = 8| **(Adwaita CSS)**: 12  |
| `border.padding_vertical`  | 4 **(measured)** (24−16)/2 | WinUI3: 3      | `TabBar_TabMarginHeight` = 4| **(Adwaita CSS)**: 3 (CSS `padding: 3px 12px`; visual 8px from min-height: 30 centering) |
| `border.color` | **(none)** — CoreUI bezel is a multi-color composite, no single extractable color | **(Fluent)** selected: `CardStrokeColorDefault` gradient (1px top/sides); unselected: transparent | **(Breeze src)** `KColorUtils::mix(bg, WindowText)` blended stroke | Notebook: none per-tab (header has 1px `$border_color`); AdwTabBar: none (high-contrast only) |
| `border.line_width` | **(none)** — CoreUI bezel is a multi-stroke composite, no single line width | 1 (`TabViewItemBorderThickness`; selected only: `TabViewSelectedItemBorderThickness=1,1,1,0`) | `PenWidth::Frame` = 1.001 | Notebook: 0; AdwTabBar: 0 (high-contrast: 1) |
| `border.corner_radius` | **(none)** — CoreUI bezel corners baked into `.car` assets, no queryable constant | 8 top (`OverlayCornerRadius` via `TopCornerRadiusFilterConverter` → 8,8,0,0) | ~4.5 (`Frame_FrameRadius=5` minus half `PenWidth::Frame`; top corners only) | Notebook: 0; AdwTabBar: 9 (`$button_radius`) |
| `border.shadow_enabled` | no — bezel creates visual depth via edge shading, not a drop shadow | no (`TabViewShadowDepth=16` resource exists but is unused in template) | no | no |
| `font`              | ← `defaults.font`    | ← `defaults.font`    | ← `defaults.font`          | ← `defaults.font`     |
| `font.color`        | ← `defaults.font.color` | ← `defaults.font.color` | ← `defaults.font.color`  | ← `defaults.font.color` |

### 2.12 Sidebar

| Property     | macOS                      | Windows                | KDE                                      | GNOME                   |
|--------------|----------------------------|------------------------|------------------------------------------|-------------------------|
| `background_color` | `underPageBackgroundColor` | **(Fluent)** NavigationView pane bg | `[Colors:Complementary] BackgroundNormal`| libadwaita `.sidebar` bg|
| `font`               | ← `defaults.font`          | ← `defaults.font`          | ← `defaults.font`           | ← `defaults.font`           |
| `font.color` | ← `defaults.font.color`   | ← `defaults.font.color`| `[Colors:Complementary] ForegroundNormal`| libadwaita `.sidebar` fg|
| `selection_background`  | ← `defaults.selection_background` | ← `defaults.selection_background` | ← `defaults.selection_background` | ← `defaults.selection_background` |
| `selection_text_color`  | ← `defaults.selection_text_color` | ← `defaults.selection_text_color` | ← `defaults.selection_text_color` | ← `defaults.selection_text_color` |
| `border.color`     | ← `defaults.border.color` | ← `defaults.border.color` | ← `defaults.border.color` | ← `defaults.border.color` |
| `border.line_width`| ← `defaults.border.line_width` | ← `defaults.border.line_width` | ← `defaults.border.line_width` | ← `defaults.border.line_width` |
| `border.corner_radius`  | **(none)** — rectangular pane | **(none)** — rectangular pane | **(none)** — rectangular pane | **(none)** — rectangular pane |
| `border.shadow_enabled` | **(none)** — no shadow | **(none)** — no shadow | **(none)** — no shadow | **(none)** — no shadow |
| `hover_background`    | `selectedContentBackgroundColor` (reduced opacity) | **(Fluent)** `SubtleFillColorSecondary` | `[Colors:View] DecorationHover` blend | **(Adwaita CSS)** row `:hover` bg |

### 2.13 Toolbar

| Property       | macOS                 | Windows            | KDE                               | GNOME                 |
|----------------|-----------------------|--------------------|------------------------------------|----------------------|
| `font.family`  | ← `defaults.font`    | ← `defaults.font` | `[General] toolBarFont` field 0 | ← `defaults.font`  |
| `font.size`    | ← `defaults.font`    | ← `defaults.font` | `[General] toolBarFont` field 1 | ← `defaults.font`  |
| `font.weight`  | ← `defaults.font`    | ← `defaults.font` | `[General] toolBarFont` field 4 | ← `defaults.font`  |
| `font.style`   | ← `defaults.font`    | ← `defaults.font` | `[General] toolBarFont` style   | ← `defaults.font`  |
| `font.color`   | ← `defaults.font.color`   | ← `defaults.font.color`   | ← `defaults.font.color`          | ← `defaults.font.color` |
| `bar_height`       | NSToolbar: 38         | WinUI3 CommandBar: default=64, compact mode=48 (`AppBarThemeCompactHeight`) | **(none)** — sizes to content  | **(Adwaita CSS)**: 47|
| `item_gap` | AppKit: 8             | WinUI3: 0 (visual gap from AppBarButton margins) | `ToolBar_ItemSpacing` = 0         | **(Adwaita CSS)**: 6 |
| `border.padding_horizontal` | 8 **(measured)** NSToolbar | WinUI3: 4 left / 0 right | `ToolBar_ItemMargin` = 6   | **(Adwaita CSS)**: 6 |
| `border.padding_vertical`  | 0                         | WinUI3: 0                      | 0                          | 0                    |
| `background_color`   | ← `defaults.background_color`   | ← `defaults.background_color`   | ← `defaults.background_color`          | ← `defaults.background_color` |
| `icon_size`    | default=32, small mode=24 (`NSToolbar.SizeMode`, deprecated) — see §2.1.8 toolbar | ↕ 20 — see §2.1.8 toolbar | 22 — see §2.1.8 toolbar | 16 — see §2.1.8 toolbar |

### 2.14 Status Bar

| Property      | macOS              | Windows                               | KDE                | GNOME              |
|---------------|--------------------|---------------------------------------|--------------------|--------------------|
| `font.family` | ← `defaults.font` | `lfStatusFont.lfFaceName`          | ← `defaults.font` | ← `defaults.font` |
| `font.size`   | ← `defaults.font` | ↕ `abs(lfStatusFont.lfHeight)*72/dpi` | ← `defaults.font` | ← `defaults.font` |
| `font.weight` | ← `defaults.font` | `lfStatusFont.lfWeight`            | ← `defaults.font` | ← `defaults.font` |
| `font.style`  | ← `defaults.font` | `lfStatusFont.lfItalic` (0 = Normal) | ← `defaults.font` | ← `defaults.font` |
| `font.color`  | ← `defaults.font.color` | ← `defaults.font.color`        | ← `defaults.font.color` | ← `defaults.font.color` |
| `background_color`  | ← `defaults.background_color` | ← `defaults.background_color`        | ← `defaults.background_color` | ← `defaults.background_color` |
| `border.color`      | ← `defaults.border.color` | ← `defaults.border.color` | ← `defaults.border.color` | ← `defaults.border.color` |
| `border.line_width` | ← `defaults.border.line_width` | ← `defaults.border.line_width` | ← `defaults.border.line_width` | ← `defaults.border.line_width` |
| `border.corner_radius` | **(none)** — rectangular bar | **(none)** — rectangular bar | **(none)** — rectangular bar | **(none)** — rectangular bar |
| `border.shadow_enabled` | **(none)** — no shadow | **(none)** — no shadow | **(none)** — no shadow | **(none)** — no shadow |

### 2.15 List / Table

| Property              | macOS                                  | Windows                 | KDE                                   | GNOME                       |
|-----------------------|----------------------------------------|-------------------------|----------------------------------------|-----------------------------|
| `background_color`          | ← `defaults.background_color`               | ← `defaults.background_color`| `[Colors:View] BackgroundNormal`   | libadwaita `.list` bg       |
| `item_font`           | ← `defaults.font`                     | ← `defaults.font`    | ← `defaults.font`                     | ← `defaults.font`          |
| `item_font.color`     | ← `defaults.font.color`               | ← `defaults.font.color`| `[Colors:View] ForegroundNormal`   | libadwaita `.list` fg       |
| `alternate_row_background`       | `alternatingContentBackgroundColors[1]` | **(Fluent)** preset L #f9f9f9 D #262626 | `[Colors:View] BackgroundAlternate` | **(Adwaita CSS)** even row |
| `selection_background`           | ← `defaults.selection_background`                | ← `defaults.selection_background` | ← `defaults.selection_background`                | ← `defaults.selection_background`     |
| `selection_text_color`| ← `defaults.selection_text_color`      | ← `defaults.selection_text_color`| ← `defaults.selection_text_color`| ← `defaults.selection_text_color`|
| `header_font`         | ← `defaults.font`                     | ← `defaults.font`    | ← `defaults.font`                     | ← `defaults.font`          |
| `header_font.color`   | `headerTextColor`                   | ← `defaults.font.color`| `[Colors:Header] ForegroundNormal` | **(Adwaita CSS)** columnview header|
| `header_background`   | **(measured)** ≈ `defaults.surface_color`  | **(Fluent)** ≈ `defaults.background_color` | `[Colors:Header] BackgroundNormal` | **(Adwaita CSS)** columnview header|
| `grid_color`          | `gridColor` (§1.1.2)               | **(none)** — uses border color | **(none)** — Qt views use palette pen | **(none)** — columnview uses CSS separator |
| `row_height`         | NSTableView row: 24                    | WinUI3 ListView: 40    | **(none)** — sizes to content          | **(Adwaita CSS)**: rich-list=32 (`.rich-list` row min-height), plain list=content-driven (no min-height) |
| `border.padding_horizontal`  | NSTableView: 4                         | WinUI3: 12             | 2                                      | **(Adwaita CSS)**: rich-list=12, plain list=2 |
| `border.padding_vertical`    | 4 **(measured)** (24−16)/2             | WinUI3: 0 (height from MinHeight=40)  | 1                                      | **(Adwaita CSS)**: rich-list=8 (`padding: 8px 12px`), plain list=2 (`padding: 2px`) |
| `hover_background`    | `selectedContentBackgroundColor` (reduced opacity) | **(Fluent)** `SubtleFillColorSecondary` | `[Colors:View] DecorationHover` blend | **(Adwaita CSS)** row `:hover` bg |

Tree views share List/Table properties (row height, padding, selection,
hover) plus tree-specific layout: per-level indentation width and
expand/collapse arrow column. These are toolkit-internal (Qt
`QTreeView::indentation()` default=20, GTK `GtkTreeExpander` arrow=16px,
AppKit `NSOutlineView.indentationPerLevel` default=16) and not exposed
as theme properties — they inherit the list row styling above.

### 2.16 Popover / Dropdown

| Property     | macOS                    | Windows                 | KDE                     | GNOME                    |
|--------------|--------------------------|-------------------------|-------------------------|--------------------------|
| `background_color` | ← `defaults.background_color` | **(Fluent)** Flyout bg = `defaults.surface_color` | ← `defaults.background_color`| libadwaita `.popover` bg|
| `font`               | ← `defaults.font`          | ← `defaults.font`          | ← `defaults.font`           | ← `defaults.font`           |
| `font.color` | ← `defaults.font.color` | ← `defaults.font.color`| ← `defaults.font.color` | libadwaita `.popover` fg|
| `border.color`     | ← `defaults.border.color`     | ← `defaults.border.color`    | ← `defaults.border.color`    | ← `defaults.border.color`    |
| `border.line_width`  | ← `defaults.border.line_width`     | ← `defaults.border.line_width`   | ← `defaults.border.line_width`            | ← `defaults.border.line_width`     |
| `border.corner_radius`     | ← `defaults.border.corner_radius_lg`  | ← `defaults.border.corner_radius_lg` | ← `defaults.border.corner_radius_lg` | ← `defaults.border.corner_radius_lg` |
| `border.shadow_enabled`     | yes (system popup shadow)| yes (Flyout elevation)  | yes (KWin compositor)   | **(Adwaita CSS)** box-shadow|
| `border.padding_horizontal` | **(none)** — content provides own padding | **(none)** — content provides own padding | **(none)** — content provides own padding | **(none)** — content provides own padding |
| `border.padding_vertical` | **(none)** — content provides own padding | **(none)** — content provides own padding | **(none)** — content provides own padding | **(none)** — content provides own padding |

### 2.17 Splitter

| Property | macOS                  | Windows           | KDE                | GNOME                   |
|----------|------------------------|-------------------|--------------------|-------------------------|
| `divider_width`  | NSSplitView divider: 6 | **(Fluent)** SplitView pane border: 1 (WinUI3 source; no draggable divider control) | Breeze splitter: 1 | **(Adwaita CSS)** paned: 1 (default) / 5 (wide) |
| `divider_color`  | `separatorColor`    | ← `defaults.border.color`| ← `defaults.border.color`| **(Adwaita CSS)** paned separator|

### 2.18 Separator

| Property | macOS            | Windows              | KDE                  | GNOME                      |
|----------|------------------|----------------------|----------------------|----------------------------|
| `line_color`  | `separatorColor` | ← `defaults.border.color` | ← `defaults.border.color` | libadwaita `.separator` CSS|
| `line_width` | ← `defaults.border.line_width` | ← `defaults.border.line_width` | ← `defaults.border.line_width` | ← `defaults.border.line_width` |

### 2.19 Text Scale

Maps platform type ramp entries into unified content roles.

| Role              | What it is                               | macOS                   | Windows Fluent      | KDE (Kirigami heading)        | GNOME libadwaita     |
|-------------------|------------------------------------------|-------------------------|---------------------|-------------------------------|----------------------|
| `caption`         | Smallest readable (footnotes, timestamps)| `.caption1`: 10pt, 400 | Caption: 12epx, 400 (=9pt @96dpi) | `smallestReadableFont` field 1| `.caption`: ≈9pt, 400 |
| `section_heading` | Section divider (settings group header)  | `.headline`: 13pt, **700** | Subtitle: 20epx, **600** (=15pt @96dpi) | Level 2: body × 1.20 ([Heading.qml](https://invent.kde.org/frameworks/kirigami/-/blob/master/src/controls/Heading.qml))  | `.heading`: 11pt, **700**|
| `dialog_title`    | Dialog/page title (sheet header)         | `.title1`: 22pt, 400 | Title: 28epx, **600** (=21pt @96dpi) | Level 1: body × 1.35 ([Heading.qml](https://invent.kde.org/frameworks/kirigami/-/blob/master/src/controls/Heading.qml))        | `.title-2`: ≈15pt, **800**|
| `display`         | Large hero text (onboarding, banners)    | `.largeTitle`: 26pt, 400| Display: 68epx, **600** (=51pt @96dpi) | **(none)** — no equivalent | `.title-1`: ≈20pt, **800**|

### 2.20 Layout Container Defaults

Default spacing for toolkit layout containers (`QLayout`, `NSStackView`,
`GtkBox`, `StackPanel`). These are the values a layout manager uses when
the developer does not specify explicit spacing. None of these are
user-configurable settings — they are compile-time constants (KDE
`breezemetrics.h`), design guidelines (macOS HIG), or hardcoded CSS
(GNOME). Windows has no layout container defaults; `StackPanel.Spacing`
defaults to 0 and apps pick from the Fluent token ramp (§1.2.5)
themselves.

| Property           | macOS HIG            | Windows Fluent                | KDE Breeze                        | GNOME libadwaita       |
|--------------------|----------------------|-------------------------------|-----------------------------------|------------------------|
| `widget_gap`       | 8 **(HIG)**          | **(none)** — app chooses from Fluent ramp | `Layout_DefaultSpacing` = 6       | 6 **(measured)**       |
| `container_margin` | **(none)** — not specified | **(none)**                    | `Layout_ChildMarginWidth` = 6     | 12 **(measured)**      |
| `window_margin`    | 20 **(HIG)**         | **(none)**                    | `Layout_TopLevelMarginWidth` = 10 | 12 **(measured)**      |
| `section_gap`      | 20 **(HIG)**         | **(none)**                    | **(none)** — not specified        | 18 **(measured)**      |

These are platform layout-manager defaults — what the toolkit uses when
the developer does not specify explicit spacing. Same pattern as
per-widget spacing fields like `dialog.border.padding_horizontal` or
`toolbar.item_gap`.

### 2.21 Switch / Toggle

| Property          | macOS                    | Windows                       | KDE                              | GNOME                         |
|-------------------|--------------------------|-------------------------------|----------------------------------|-------------------------------|
| `track_width`     | 38px                    | WinUI3: 40                    | QQC2: ~36 (font-derived)        | ~46px (derived: 2×thumb+pad) |
| `track_height`    | 22px                    | WinUI3: 20                    | QQC2: ~18 (font-derived)        | ~26px (20+2×3 padding)       |
| `thumb_diameter`      | ~18px **(measured)**     | WinUI3: 12 (rest) / 14 (hover)| QQC2: ~18 (= track height)      | 20px                          |
| `track_radius`    | 11 (= `track_height`/2, pill) | 10 (= `track_height`/2, pill) | ~9 (= `track_height`/2, pill)    | 14 (> `track_height`/2, pill) |
| `checked_background`      | ← `defaults.accent_color`     | ← `defaults.accent_color`          | ← `defaults.accent_color`             | ← `defaults.accent_color`          |
| `unchecked_background`    | **(measured)** track bg  | **(Fluent)** ToggleSwitchFillOff | **(preset)** trough color     | Adwaita `$trough_color`      |
| `thumb_background`        | **(measured)** white     | **(Fluent)** ToggleSwitchKnob | **(preset)** slider color        | Adwaita `$slider_color`      |
| `disabled_opacity`  | ← `defaults.disabled_opacity`| ← `defaults.disabled_opacity`| ← `defaults.disabled_opacity`     | ← `defaults.disabled_opacity`|

macOS NSSwitch introduced in 10.15. KDE has no QWidget toggle — only
QQC2/Kirigami `Switch` with font-metric-derived sizing.

### 2.22 Dialog

| Property              | macOS                         | Windows                           | KDE                               | GNOME                              |
|-----------------------|-------------------------------|-----------------------------------|------------------------------------|-------------------------------------|
| `background_color`          | ← `defaults.background_color`      | **(Fluent)** `ContentDialogBackground` | ← `defaults.background_color`      | **(Adwaita CSS)** `messagedialog` bg|
| `body_font`          | ← `defaults.font`          | ← `defaults.font`          | ← `defaults.font`           | ← `defaults.font`           |
| `body_font.color`    | ← `defaults.font.color`    | ← `defaults.font.color`    | ← `defaults.font.color`     | ← `defaults.font.color`     |
| `border.shadow_enabled`              | yes (sheet overlay)           | yes (ContentDialog smoke layer + elevation) | yes (KWin compositor)       | **(Adwaita CSS)** box-shadow       |
| `min_width`           | **(none)** — AppKit-managed   | WinUI3 ContentDialog: 320         | **(none)** — sizes to content      | AdwAlertDialog: 300sp              |
| `max_width`           | **(none)** — AppKit-managed   | WinUI3 ContentDialog: 548         | **(none)** — sizes to content      | AdwAlertDialog: 372sp (wide: 600sp)|
| `min_height`          | **(none)** — AppKit-managed   | WinUI3 ContentDialog: 184         | **(none)** — sizes to content      | **(none)**                         |
| `max_height`          | **(none)** — AppKit-managed   | WinUI3 ContentDialog: 756         | **(none)** — sizes to content      | **(none)**                         |
| `border.padding_horizontal` | ~20px **(measured)**    | WinUI3: 24                        | `Layout_TopLevelMarginWidth` = 10  | 24px                               |
| `border.padding_vertical`  | ~20px **(measured)**    | WinUI3: 24                        | `Layout_TopLevelMarginWidth` = 10  | 32 top / 24 bottom (`.message-area` padding-top=32; `.response-area` padding-top=12, padding-bottom=24) |
| `button_gap`      | ~12px **(measured)**          | WinUI3: 8                         | `Layout_DefaultSpacing` = 6        | 12px                               |
| `button_order`        | primary rightmost             | primary leftmost                  | OK left of Cancel (right-aligned group; Help/Reset left-aligned) | cancel left, affirmative right     |
| `title_font.family`   | ← `defaults.font`            | ← `defaults.font` (Segoe UI)     | ← `defaults.font`                 | ← `defaults.font`                 |
| `title_font.size`     | ✅ 13pt **(HIG)** — "emphasized system font" = `boldSystemFontOfSize:0` → `systemFontSize` (13pt); corroborated by Cocotron `NSAlert.m` | 20px (ContentDialog template)     | ← `defaults.font`                 | 136% of base ≈15pt (`.title-2`)   |
| `title_font.weight`   | ✅ Bold (700) **(HIG)** — "emphasized system font" = `boldSystemFontOfSize:0`; corroborated by Cocotron `NSAlert.m` | SemiBold (600)                    | ← `defaults.font`                 | 800 (ExtraBold, `.title-2`)       |
| `title_font.style`    | Normal                        | Normal                            | ← `defaults.font`                 | Normal                             |
| `title_font.color`    | ← `defaults.font.color`      | ← `defaults.font.color`          | ← `defaults.font.color`           | ← `defaults.font.color`           |
| `border.corner_radius`              | ← `defaults.border.corner_radius_lg`       | 8px (OverlayCornerRadius) ✅      | ← `defaults.border.corner_radius_lg`            | 18px (`$alert_radius`) — distinct from window radius (15px) |
| `border.color` | ← `defaults.border.color` | ← `defaults.border.color` | ← `defaults.border.color` | **(Adwaita CSS)** dialog border |
| `border.line_width` | ← `defaults.border.line_width` | ← `defaults.border.line_width` | ← `defaults.border.line_width` | ← `defaults.border.line_width` |
| `icon_size`           | 64px (app icon)               | **(none)** — no default icon      | **(none)** — per-dialog            | **(none)** — no default icon       |

Dialog dimensions (`min_width`, `max_width`, `min_height`, `max_height`)
measure the **dialog surface** — the visible dialog box from its outer
border edge to outer border edge. This includes the title area,
`border.padding_*`, body text area, and button row. It does **not**
include the drop shadow, the background overlay (smoke layer), the
desktop window frame, or the desktop title bar. macOS sheets are
fully AppKit-managed and expose no dimension constraints.

Button order convention differs significantly across platforms.
macOS primary action = rightmost. Windows primary = leftmost. KDE:
Help/Reset left-aligned, then stretch, then OK/Apply/Cancel right-aligned
(OK left of Cancel). GNOME: cancel left, affirmative right.

Dialog internal spacing (title-to-body, body-to-buttons) is baked into
each platform's dialog template, not exposed as a single adjustable gap.
macOS: AppKit-managed (not queryable). Windows: ContentDialog XAML
template controls layout. KDE: `Layout_DefaultSpacing` = 6 between
elements. GNOME: `.message-area` padding-top=32, `.response-area`
padding-top=12 / padding-bottom=24 (per `_message-dialog.scss`).

### 2.23 Spinner / Progress Ring

| Property      | macOS                          | Windows                  | KDE                          | GNOME                     |
|---------------|--------------------------------|--------------------------|------------------------------|---------------------------|
| `diameter`    | 32px regular, 16px small       | WinUI3 ProgressRing: 32  | QQC2 BusyIndicator: 36      | GtkSpinner: 16            |
| `min_diameter`    | 10px (mini)                    | WinUI3: 16               | **(none)**                   | **(none)**                |
| `stroke_width`| **(none)** — fin-based         | WinUI3: 4                | **(none)** — icon-based      | **(none)** — icon-based   |
| `fill_color`        | system gray                    | ← `defaults.accent_color`     | ← `defaults.text_color`     | ← `defaults.text_color`  |

macOS uses radiating fins, not a stroke ring. KDE and GNOME use a
rotating `process-working-symbolic` icon.

### 2.24 ComboBox / Dropdown Trigger

| Property            | macOS                    | Windows               | KDE                             | GNOME                        |
|---------------------|--------------------------|-----------------------|---------------------------------|------------------------------|
| `background_color`        | `controlColor`        | `COLOR_BTNFACE`    | `[Colors:Button] BackgroundNormal` | libadwaita button bg    |
| `font`              | ← `defaults.font`       | ← `defaults.font`    | ← `defaults.font`              | ← `defaults.font`           |
| `font.color`        | `controlTextColor`    | `COLOR_BTNTEXT`    | `[Colors:Button] ForegroundNormal` | libadwaita button fg    |
| `border.color`            | ← `defaults.border.color`     | ← `defaults.border.color`  | ← `defaults.border.color`            | ← `defaults.border.color`         |
| `border.line_width`  | ← `defaults.border.line_width`     | ← `defaults.border.line_width`   | ← `defaults.border.line_width`            | ← `defaults.border.line_width`     |
| `min_height`        | NSPopUpButton: 21        | WinUI3 ComboBox: 32   | **(none)** — sizes to content   | ← button min-height (24+pad)|
| `min_width`         | **(none)** — sizes to content | WinUI3: 64         | **(none)** — sizes to content   | **(none)** — sizes to content|
| `border.padding_horizontal`| ~8–10px **(measured)**   | WinUI3: 12 left / 0 right (arrow area adjacent) | `ComboBox_FrameWidth` = 6      | ← button padding (10px)     |
| `arrow_icon_size`        | ~16–18px **(measured)**  | WinUI3 glyph: 12      | `MenuButton_IndicatorWidth` = 20| 16px (pan-down-symbolic)    |
| `arrow_area_width`  | ~16–18px **(measured)**  | WinUI3: 38             | 20px                            | **(none)** — inline icon     |
| `border.corner_radius`            | ← `defaults.border.corner_radius`     | ← `defaults.border.corner_radius`   | ← `defaults.border.corner_radius`            | ← `defaults.border.corner_radius`         |
| `border.shadow_enabled` | **(none)** — no shadow | **(none)** — no shadow | **(none)** — no shadow | **(none)** — no shadow |
| `border.padding_vertical` | ~3px **(measured)** | WinUI3: 5 top / 7 bottom | **(none)** — sizes to content | ← button (5px) |
| `disabled_opacity`  | ← `defaults.disabled_opacity`| ← `defaults.disabled_opacity`| ← `defaults.disabled_opacity`     | ← `defaults.disabled_opacity`|

### 2.25 Segmented Control

| Property          | macOS                         | Windows        | KDE                      | GNOME              |
|-------------------|-------------------------------|----------------|--------------------------|---------------------|
| `background_color`      | NSSegmentedControl bg       | **(none)**     | ← `defaults.background_color`                       | **(none)** |
| `font`               | ← `defaults.font`          | ← `defaults.font`          | ← `defaults.font`           | ← `defaults.font`           |
| `font.color`      | `controlTextColor`          | **(none)**     | ← `defaults.font.color`                       | **(none)** |
| `active_background`| `selectedContentBackgroundColor` | **(none)** | `[Colors:Selection] BackgroundNormal`       | **(none)** |
| `active_text_color`| `alternateSelectedControlTextColor` | **(none)** | `[Colors:Selection] ForegroundNormal`    | **(none)** |
| `segment_height`  | NSSegmentedControl: 24        | **(none)**     | `TabBar_TabMinHeight` = 30 (tab bar as proxy) | **(none)** |
| `separator_width` | 1px                           | **(none)**     | `TabBar_TabOverlap` = 1  | **(none)**          |
| `border.padding_horizontal` | ~8–10px **(measured)**     | **(none)**     | `TabBar_TabMarginWidth` = 8 | **(none)**       |
| `border.corner_radius`          | ← `defaults.border.corner_radius`          | **(none)**     | ← `defaults.border.corner_radius`     | **(none)**          |
| `border.color` | NSSegmentedControl border | **(none)** | ← `defaults.border.color` | **(none)** |
| `border.line_width` | ← `defaults.border.line_width` | **(none)** | ← `defaults.border.line_width` | **(none)** |
| `border.shadow_enabled` | **(none)** — no shadow | **(none)** | **(none)** — no shadow | **(none)** |
| `border.padding_vertical` | ~3px **(measured)** | **(none)** | `TabBar_TabMarginHeight` = 4 | **(none)** |
| `disabled_opacity`  | ← `defaults.disabled_opacity`| **(none)** | ← `defaults.disabled_opacity`     | **(none)** |

macOS is the only platform with a first-class segmented control.
Available styles: `.automatic`, `.rounded`, `.roundRect`, `.texturedRounded`,
`.capsule`, `.texturedSquare`, `.smallSquare`, `.separated`.

### 2.26 Card / Container

| Property     | macOS          | Windows                                    | KDE            | GNOME                     |
|--------------|----------------|--------------------------------------------|----------------|---------------------------|
| `background_color` | **(none)**     | **(Fluent)** CardBackgroundFillColorDefault | **(none)**     | `var(--card-bg-color)`    |
| `border.color`     | **(none)**     | **(Fluent)** CardStrokeColorDefault        | **(none)**     | `var(--card-shade-color)` |
| `border.line_width`  | **(none)**     | 1px                                     | **(none)**     | 1px (CSS)                |
| `border.corner_radius`     | **(none)**     | 8px (OverlayCornerRadius)                  | **(none)**     | `$card_radius` = 12px    |
| `border.shadow_enabled`     | **(none)**     | **(none)** — border only                   | **(none)**     | Adwaita box-shadow        |
| `border.padding_horizontal` | **(none)** | 12px (convention)                       | **(none)**     | **(none)** — app-defined  |
| `border.padding_vertical`  | **(none)** | 12px (convention)                       | **(none)**     | **(none)** — app-defined  |

macOS and KDE have no native card component. WinUI3 has card color
resources but no Card control (open proposal #6543). GNOME defines
`.card` CSS class used by `list.boxed-list`.

### 2.27 Expander / Disclosure

| Property          | macOS                       | Windows                  | KDE                          | GNOME                        |
|-------------------|-----------------------------|--------------------------|------------------------------|------------------------------|
| `font`               | ← `defaults.font`          | ← `defaults.font`          | ← `defaults.font`           | ← `defaults.font`           |
| `font.color`          | ← `defaults.font.color`    | ← `defaults.font.color`    | ← `defaults.font.color`     | ← `defaults.font.color`     |
| `header_height`   | **(none)** — content-sized  | WinUI3 Expander: 48      | **(none)** — content-sized   | AdwExpanderRow: 50           |
| `arrow_icon_size`      | ~13px **(measured)**        | WinUI3 chevron glyph: 12 | `ItemView_ArrowSize` = 10    | 16px (pan-end-symbolic)      |
| `border.padding_horizontal` | **(none)** — app-defined | WinUI3: 16            | **(none)** — app-defined     | **(Adwaita CSS)** row padding|
| `border.padding_vertical`  | **(none)** — app-defined | WinUI3: 16            | **(none)** — app-defined     | **(Adwaita CSS)** row padding|
| `border.corner_radius`          | **(none)**                  | ← `defaults.border.corner_radius`     | `Frame_FrameRadius` = 5      | 6px (expander title)         |
| `border.color` | **(none)** | ← `defaults.border.color` | **(none)** | **(Adwaita CSS)** expander border |
| `border.line_width` | **(none)** | ← `defaults.border.line_width` | **(none)** | ← `defaults.border.line_width` |
| `border.shadow_enabled` | **(none)** — no shadow | **(none)** — no shadow | **(none)** — no shadow | **(none)** — no shadow |

macOS uses `NSDisclosureButton` bezel style (triangle). KDE has no
dedicated expander — `QGroupBox` with a checkbox is the closest.

### 2.28 Link

| Property      | macOS                    | Windows                            | KDE                              | GNOME                         |
|---------------|--------------------------|------------------------------------|----------------------------------|-------------------------------|
| `font`        | ← `defaults.font`    | ← `defaults.font`               | ← `defaults.font`            | ← `defaults.font`           |
| `font.color`  | `linkColor`           | **(Fluent)** AccentTextFillColor   | `ForegroundLink`              | `var(--accent-color)`         |
| `visited_text_color`     | **(none)** — same as link| **(none)** — same as link          | `ForegroundVisited`           | Adwaita 80% mix accent+fg    |
| `underline_enabled`   | yes                      | **(none)** — no underline by default| yes (Kirigami LinkButton)       | yes                           |
| `background_color`  | **(none)** — inline      | **(Fluent)** transparent (HyperlinkButton) | **(none)** — inline      | **(none)** — inline           |
| `hover_background`    | **(none)**               | **(Fluent)** SubtleFillColorSecondary | **(none)**                   | **(none)**                    |

Windows `HyperlinkButton` is a full button control with hover/press
states. Other platforms style links as inline text with underline.

---

## Appendix: Verification Sources (2026-03-24)

Every value in this document was cross-checked against internet sources.
Values marked ✅ are confirmed, ❓ uncertain/unverifiable.
Below are the authoritative sources used, organized by platform.

### macOS

| What was verified | Source |
|---|---|
| NSFont class methods exist and roles are correct | [NSFont — Apple Developer Documentation](https://developer.apple.com/documentation/appkit/nsfont) |
| `systemFontSize`=13, `smallSystemFontSize`=11, `labelFontSize`=10 | [Monkeybread NSFont reference](https://www.monkeybreadsoftware.net/cocoa-nsfontmbs-shared-method.shtml) — explicitly states the three values |
| `.body`=13pt confirmed | [WWDC 2020 Session 10175 "The details of UI typography"](https://developer.apple.com/videos/play/wwdc2020/10175/) — "13 pt" for body on macOS |
| All TextStyle sizes and weights | ✅ [Apple HIG Typography Specifications](https://developer.apple.com/design/human-interface-guidelines/typography) — macOS built-in text styles table accessible via HIG JSON API (`developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json`). All sizes confirmed: `.largeTitle`=26pt, `.caption1`=10pt, `.caption2`=10pt. Key weight corrections: `.headline` is **Bold (700)** on macOS (not SemiBold — iOS differs); `.caption2` is **Medium (500)** (not Regular). WWDC 2020 Session 10175 discusses `.body`+bold→SemiBold, not `.headline` itself. Third-party implementations (ViewKit, shaps80) predate the macOS 11 TextStyle API and have inaccurate values. |
| macOS does not support Dynamic Type | [WWDC 2020 Session 10175](https://developer.apple.com/videos/play/wwdc2020/10175/) — explicitly states "Although there is no Dynamic Type support" for macOS |
| `menuBarFontOfSize:` weight | ❓ API exists ([Apple docs](https://developer.apple.com/documentation/appkit/nsfont/menubarfont(ofsize:))) but weight is not documented; no evidence of SemiBold — likely Regular like other font methods; needs verification on Mac hardware |
| All NSColor semantic methods exist | Individual Apple doc pages — e.g. [controlAccentColor](https://developer.apple.com/documentation/appkit/nscolor/3000782-controlaccentcolor) (macOS 10.14, introduced with Dark Mode in [WWDC 2018 Session 210](https://asciiwwdc.com/2018/sessions/210)), [labelColor](https://developer.apple.com/documentation/appkit/nscolor/1534657-labelcolor), etc. |
| Text insertion point APIs: `NSTextView.insertionPointColor` (old), `NSColor.textInsertionPointColor` (macOS 14+), `NSTextInsertionIndicator` (macOS 14+) | ✅ `NSColor.textInsertionPointColor` is a type property introduced in macOS 14.0 per [Apple docs JSON](https://developer.apple.com/tutorials/data/documentation/appkit/nscolor/textinsertionpointcolor.json) (`"introducedAt": "14.0"`, `"roleHeading": "Type Property"`). Note: [martinhoeller NSColor catalog dump (14.4)](https://gist.github.com/martinhoeller/38509f37d42814526a9aecbb24928f46) does not list it because it only catalogs `NSColorType.catalog` entries, not all class properties. `NSTextInsertionIndicator`: [Apple docs](https://developer.apple.com/documentation/appkit/nstextinsertionindicator) |
| `systemTealColor` = macOS 10.12 | Apple SDK headers: `API_AVAILABLE(macos(10.12))` in [NSColor.h (10.15 SDK)](https://github.com/phracker/MacOSX-SDKs/blob/master/MacOSX10.15.sdk/System/Library/Frameworks/AppKit.framework/Versions/C/Headers/NSColor.h); class-dump from [w0lfschild/macOS_headers](https://github.com/w0lfschild/macOS_headers/blob/master/macOS/Frameworks/AppKit/1643.10.101/NSColor.h) confirms runtime symbol pre-10.15. WWDC 2019 said "new" because the header declaration first shipped in 10.15 SDK, but runtime symbol existed since 10.12. |
| `systemIndigoColor` = macOS 10.15 | Same SDK headers: `API_AVAILABLE(macos(10.15))`. [WWDC 2019 Session 210](https://developer.apple.com/videos/play/wwdc2019/210/?time=754) introduces indigo as genuinely new. |
| `systemCyanColor` = macOS 12 | ✅ [Apple docs](https://developer.apple.com/documentation/appkit/nscolor/systemcyan) confirms macOS 12.0; Apple docs JSON correctly shows `"introducedAt":"12.0"`; no class-dump evidence of pre-12 existence (unlike teal/mint which existed at runtime since 10.12) |
| `systemMintColor` = macOS 10.12 | ✅ Same pattern as `systemTealColor`: runtime symbol present in [AppKit 1504 class-dump](https://github.com/w0lfschild/macOS_headers/blob/master/macOS/Frameworks/AppKit/1504.82.104/NSColor.h) (macOS 10.12); absent from [AppKit 1348](https://github.com/w0lfschild/macOS_headers/blob/master/macOS/Frameworks/AppKit/1348.17/NSColor.h) (macOS 10.10). SDK header first appeared in macOS 12.0 SDK ([codeworkshop diff](http://codeworkshop.net/objc-diff/sdkdiffs/macos/12.0/AppKit.html)) with `API_AVAILABLE(macos(10.12))`. Apple docs JSON `introducedAt: "10.12"` is correct, not a bug. `@available(macOS 12.0, *)` guards in Swift code are overly conservative. |
| `performAsCurrentDrawingAppearance` | [Apple docs](https://developer.apple.com/documentation/appkit/nsappearance/3674525-performascurrentdrawingappearance) — macOS 11.0 |
| `colorUsingColorSpace:` for P3→sRGB | [Apple docs](https://developer.apple.com/documentation/appkit/nscolor/usingcolorspace(_:)) |
| Window corner radius = 10px | Multiple community sources confirm 10pt through Sequoia. macOS Tahoe (26) uses variable radii per window style: [macos-corner-fix](https://github.com/m4rkw/macos-corner-fix) confirms 16pt (title-bar-only); toolbar window radii: sources disagree — [Zed discussion #38233](https://github.com/zed-industries/zed/discussions/38233) reports ~26pt (from WWDC25 screenshot); [Podfeet/Steve Harris](https://www.podfeet.com/blog/2025/10/rounded-screenshots-shell-script/) measured 50px at 2× = ~25pt; [VS Code PR #270236](https://github.com/microsoft/vscode/pull/270236) suggests 20pt (compact) / 24pt (standard); [lapcatsoftware](https://lapcatsoftware.com/articles/2026/3/1.html) confirms variable radii qualitatively; [alt-tab-macos #4985](https://github.com/lwouis/alt-tab-macos/issues/4985) notes "4 or 5" distinct radii; [WebKit commit 643493b](https://github.com/WebKit/WebKit/commit/643493bea2f9824959ebb9824bfb011aedf7498c) reads radii dynamically via private `_cornerConfiguration` SPI (macOS 26.1+); no public API exists. |
| NSTableView rowHeight = 24pt (macOS 11+) | ✅ Changed from 17pt in Big Sur. [lapcatsoftware "BS AppKit notes"](https://lapcatsoftware.com/articles/BSAppKit.html) and [AppKit Release Notes for macOS 11](https://developer.apple.com/documentation/macos-release-notes/appkit-release-notes-for-macos-11) confirm. |
| Control corner radius = 5px | ✅ WebKit [`RenderThemeMac.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/rendering/mac/RenderThemeMac.mm): `const int baseBorderRadius = 5` for styled popup/menu-list buttons |
| NSTextField intrinsic height = 22px | ✅ WebKit `RenderThemeMac.mm` search field sizes: regular=22, small=19, mini=17, large=30 |
| NSSwitch intrinsic size = 38×22, thumb ~18px | ✅ WebKit `RenderThemeMac.mm`: `switchSizes()` = {38,22} regular, {32,18} small, {26,15} mini. Thumb: WebKit [`SwitchThumbMac.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/platform/graphics/mac/controls/SwitchThumbMac.mm): bounding box = track height (22px); visual CoreUI knob ~18px inside. |
| NSSplitView divider = 6px | ✅ GNUstep [`NSSplitView.m`](https://github.com/gnustep/libs-gui/blob/master/Source/NSSplitView.m): thick/paneSplitter=6pt, thin=1pt; default is thick. [CocoaDev SplitViewBasics](https://cocoadev.github.io/SplitViewBasics/) also confirms. |
| NSPopUpButton intrinsic height = 21px | ✅ WebKit `RenderThemeMac.mm`: `popupButtonSizes()` = {0,21} regular, {0,18} small, {0,15} mini, {0,24} large. Previously listed as 26px — that value is the right-padding (arrow area width), not height. |
| Spinning progress 32/16px | ✅ `NSProgressIndicator.sizeToFit` by `controlSize`: regular=32, small=16, mini=10. [Apple sizeToFit docs](https://developer.apple.com/documentation/appkit/nsprogressindicator/1501144-sizetofit) |
| NSSlider track height = 5px | ✅ WebKit [`SliderTrackMac.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/platform/graphics/mac/controls/SliderTrackMac.mm): `sliderTrackWidth = 5` (previously listed as 4px) |
| NSSlider thumb = 21px (measured) vs WebKit 17px | ❓ WebKit `RenderThemeMac.mm`: `sliderThumbThickness = 17` with FIXME "should be obtained from AppKit via `knobThickness`"; WebKit acknowledges its value may be wrong. [`knobThickness`](https://developer.apple.com/documentation/appkit/nsslider/1532909-knobthickness) is deprecated since macOS 10.9 and per WebKit "returns an incorrect value." 21px measured from native AppKit rendering is plausible but no Apple constant exists. |
| NSButton (checkbox) indicator = 14px | ❓ WebKit [`ToggleButtonMac.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/platform/graphics/mac/controls/ToggleButtonMac.mm): regular={14,14}; Gecko [`nsNativeThemeCocoa.mm`](https://github.com/mozilla/gecko-dev/blob/master/widget/cocoa/nsNativeThemeCocoa.mm): native={16,16} — engines disagree |
| NSDisclosureButton cell = 21×21px | Gecko `nsNativeThemeCocoa.mm`: `kDisclosureButtonSize = {21, 21}` — visible triangle (~13px measured) is a subset of the cell |
| Overlay scrollbar thumb ~7px (idle) | ✅ Gecko [`ScrollbarDrawingCocoa.cpp`](https://searchfox.org/mozilla-central/source/widget/ScrollbarDrawingCocoa.cpp): overlay non-hovered thumb = 7px (8px base − 1px overlay adjustment), hovered = 11px, within a 16px overlay track; Chromium [`native_theme_mac.mm`](https://github.com/chromium/chromium/blob/master/ui/native_theme/native_theme_mac.mm): `GetThumbMinSize()` = {6,18} (6px minimum thumb width); Chromium [`overlay_scrollbar_constants.h`](https://github.com/chromium/chromium/blob/master/ui/native_theme/overlay_scrollbar_constants.h): `kOverlayScrollbarThumbWidthPressed` = 10px, idle scale = 0.4; WebKit [`ScrollbarThemeMac.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/platform/mac/ScrollbarThemeMac.mm) delegates to native `NSScrollerImp` (dynamic, no hardcoded value). Two engines agree on ~6–7px idle thumb width. |
| Other geometry/widget metrics (0.5px, padding values, etc.) | ❓ Apple does not publish these. All are measured values. Legacy scrollbar width = ✅ 16px confirmed by [developer measurement](https://gist.github.com/martynchamberlin/6aaf8a45b36907e9f1e21a28889f6b0a) and `scrollerWidth(for:scrollerStyle:)`. Disabled state uses `disabledControlTextColor` (alpha ≈0.247, confirmed across macOS Catalina–Monterey by [andrejilderda gist](https://gist.github.com/andrejilderda/8677c565cddc969e6aae7df48622d47c) and [zrzka gist](https://gist.github.com/zrzka/7836c8339e0141601aa4a02a3f2e04c6)), not a global opacity multiplier. |
| Focus ring width = 3px | ✅ Confirmed via WebKit SPI: `UIFocusRingStyle.borderThickness = 3` ([WebKit outline-style:auto commit](https://github.com/WebKit/WebKit/commit/c3770c7b04d216f822e3a4308c43b01ec0e7afed)); [Mozilla Bug 53927](https://bugzilla.mozilla.org/show_bug.cgi?id=53927) (Mac OS 9 era, 2px) is obsolete. Modern focus ring is a diffuse glow — 3px is the settled border thickness. |
| NSButton height = 22px | Well-corroborated — multiple developer discussions confirm "22px is the right height for a clickable control" |
| NSButton horizontal padding ~8px | ❓ WebKit [`RenderThemeMac.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/rendering/mac/RenderThemeMac.mm) `controlPadding(PushButton)` = 8px horizontal; comment says "AppKit wants to use 11px for mini buttons." Gecko `nsNativeThemeCocoa.mm` `pushButtonSettings` margins `{0,5,2,5}` are *external* outsets (focus ring/chrome), not content padding; Gecko CSS `<button>` uses `padding-inline: 4px`. Legacy HIG 12px is inter-button *spacing*. Native bezel internal padding is not directly queryable. |
| NSStackView default spacing = 8pt | [Apple docs NSStackView.spacing](https://developer.apple.com/documentation/appkit/nsstackview/spacing) — "default value is 8.0 points" |
| IB standard spacing = 8pt between siblings, 20pt to superview | [Auto Layout Guide](https://developer.apple.com/library/archive/documentation/UserExperience/Conceptual/AutolayoutPG/WorkingwithSimpleConstraints.html) |
| Label-to-control = 8pt (regular) | Legacy Apple HIG — 8px regular, 6px small, 5px mini |
| Toolbar regular=32, small=24 | [NSToolbar.SizeMode docs](https://developer.apple.com/documentation/appkit/nstoolbar/sizemode) (deprecated) |
| Sidebar icon sizes 16/20/24 (macOS 11+) | ✅ Apple HIG Sidebars page (macOS section, archived 2022 via Wayback Machine) documented full metrics table: Small=16×16px (row 24pt), Medium=20×20px (row 28pt), Large=24×24px (row 32pt). Table removed from current HIG ~2024. Pre-Big Sur legacy sizes were 16/18/32 (from CoreTypes.bundle). |
| Menu bar extra icon = 16pt | [Bjango guide](https://bjango.com/articles/designingmenubarextras/) — community best-practice, not official |
| Accessibility APIs | All confirmed: [reduceMotion](https://developer.apple.com/documentation/appkit/nsworkspace/accessibilitydisplayshouldreducemotion) (10.12), [reduceTransparency](https://developer.apple.com/documentation/appkit/nsworkspace/accessibilitydisplayshouldreducetransparency) (10.10), [increaseContrast](https://developer.apple.com/documentation/appkit/nsworkspace/accessibilitydisplayshouldincreasecontrast) (10.10), [differentiateWithoutColor](https://developer.apple.com/documentation/appkit/nsworkspace/accessibilitydisplayshoulddifferentiatewithoutcolor) (10.10) |
| `NSFont.preferredFont(forTextStyle:)` does NOT auto-scale | ✅ WWDC 2020 Session 10175 explicitly states macOS has no Dynamic Type; sizes are fixed. macOS 14 (Sonoma) added limited "Text Size" in Accessibility settings (few Apple apps only), but `preferredFont(forTextStyle:)` still returns fixed sizes. |
| NSScroller `.overlay`/`.legacy` | [NSScroller.preferredScrollerStyle](https://developer.apple.com/documentation/appkit/nsscroller/preferredscrollerstyle) — macOS 10.7 |
| NSSwitch introduced macOS 10.15 | [NSSwitch docs](https://developer.apple.com/documentation/appkit/nsswitch) + WWDC 2019 Session 210 |
| "Between related controls = 8pt" oversimplified | Legacy HIG specifies 12px for regular push buttons, 8px only for mini/icon buttons |
| `+toolTipsFontOfSize:` default size | ❓ [Apple API docs](https://developer.apple.com/documentation/appkit/nsfont/1527704-tooltipsfontofsize) do not state default size; [Leopard-era Apple HIG](https://leopard-adc.pepas.com/documentation/UserExperience/Conceptual/AppleHIGuidelines/XHIGText/XHIGText.html) states "The small system font (11 point) is the default font for help tags" (Apple's term for tooltips), strongly supporting 11pt; [Cocotron NSFont.m](https://github.com/berkus/cocotron/blob/master/AppKit/NSFont.m) defaults to 10pt; [GNUstep NSFont.m](https://github.com/gnustep/libs-gui/blob/master/Source/NSFont.m) defaults to 12pt; open-source impls disagree with each other and with the HIG |
| NSProgressIndicator visual bar vs control frame | ❓ bar height 6px is the visual track; `NSProgressIndicatorPreferredThickness`=14px deprecated (Apple: "do not accurately represent the geometry"); WebKit [`ProgressBarMac.mm`](https://github.com/WebKit/WebKit/blob/main/Source/WebCore/platform/graphics/mac/controls/ProgressBarMac.mm) cell sizes: regular=20px, small=12px, mini=12px (frame height, not visual bar); Chromium `LayoutThemeMac.mm` agrees (20, 12, 12); [GRProgressIndicator](https://github.com/insidegui/GRProgressIndicator) confirms visual bar is drawn smaller than frame. Visual track height is rendered by CoreUI — no engine exposes the exact value. |

### Windows

| What was verified | Source |
|---|---|
| NONCLIENTMETRICSW struct and retrieval | [MSDN NONCLIENTMETRICSW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nonclientmetricsw) |
| Five LOGFONTW fields | Same source — lfCaptionFont, lfSmCaptionFont, lfMenuFont, lfStatusFont, lfMessageFont |
| Default font values (Segoe UI, -12, weights) | ✅ (face+size) / ❓ (weights) — [Win32 UX Guide](https://learn.microsoft.com/en-us/windows/win32/uxguide/vis-fonts) documents "9 pt. Segoe UI" as default for all UI text; also confirmed by [VS docs](https://learn.microsoft.com/en-us/visualstudio/extensibility/ux-guidelines/fonts-and-formatting-for-visual-studio?view=visualstudio-2022): "default… 9 pt Segoe UI." UX Guide was written for Windows 7 but values are unchanged. Weights are not documented — 400 is empirical; caption weight varies: 400 on Win10, **700 (Bold) on Win11** per [Microsoft Q&A](https://learn.microsoft.com/en-us/answers/questions/5489781/title-bar-text-boldness-in-windows-11). Win32 API returns "Segoe UI" even on Win11 per [Mozilla Bug 1732404](https://bugzilla.mozilla.org/show_bug.cgi?id=1732404) and [VS Code #156766](https://github.com/microsoft/vscode/issues/156766) (Segoe UI Variable is WinUI3/XAML-internal). |
| lfHeight→points formula | [MSDN LOGFONTW](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-logfontw) — inverse formula documented |
| WinUI3 type ramp (all 9 entries incl. BodyLargeStrong) | [MS Typography in Windows](https://learn.microsoft.com/en-us/windows/apps/design/signature-experiences/typography); BodyLargeStrong confirmed in [TextBlock_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/CommonStyles/TextBlock_themeresources.xaml) |
| UISettings GetColorValue enum values | [UIColorType Enum](https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uicolortype) — Complement exists but "Do not use" |
| GetSysColor constants | [GetSysColor function](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor) |
| Win10+ supported COLOR_ constants (8 total) | Same source — explicitly marks unsupported ones; `COLOR_3DFACE` (value 15) is not marked unsupported but its alias `COLOR_BTNFACE` (same value) is — documentation inconsistency |
| DwmGetColorizationColor | [MSDN DwmGetColorizationColor](https://learn.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmgetcolorizationcolor) |
| SM_CXBORDER=1 | ✅ [NONCLIENTMETRICSW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-nonclientmetricsw) explicitly documents "iBorderWidth… The default is 1 pixel" |
| SM_CXVSCROLL=17, SM_CYHSCROLL=17 | ✅ Confirmed via .NET [SystemParameters.VerticalScrollBarWidth](https://learn.microsoft.com/en-us/dotnet/api/system.windows.systemparameters.verticalscrollbarwidth) docs and multiple measurements |
| SM_CXFOCUSBORDER=1, SM_CYFOCUSBORDER=1 | ✅ Confirmed by ReactOS (`win32ss/user/ntuser/metric.c`) and Wine (`dlls/win32u/sysparams.c`) default values |
| SM_CYMENU=20 | ✅ Registry default `MenuHeight`=-285 = 19px (per [Winaero](https://winaero.com/how-to-change-menu-row-height-in-windows-10-windows-8-1-and-windows-8/) and [MS Windows Registry Guide](https://flylib.com/)); Wine source (`dlls/win32u/sysparams.c`) confirms SM_CYMENU = `iMenuHeight + 1` = 20px (the +1 is the menu bar bottom border). SM_CYMENUSIZE = `iMenuHeight` = 19px (no border). MSDN: "the height of a single-line menu bar… not the height of a menu item" |
| SM_CYVTHUMB=17 | ❓ [GetSystemMetrics](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics) does not document default; derivable from registry `HKCU\Control Panel\Desktop\WindowMetrics\ScrollHeight` default=-255 → -255/-15=17px; consistent with SM_CXVSCROLL=17. Note: [Mozilla Bug 502292](https://bugzilla.mozilla.org/show_bug.cgi?id=502292) reports 15px minimum at true 96 DPI and 17px at 110% — the discrepancy may reflect DPI differences or Firefox-specific measurement. |
| ControlCornerRadius=4px, OverlayCornerRadius=8px | [MS Geometry in Windows 11](https://learn.microsoft.com/en-us/windows/apps/design/signature-experiences/geometry) |
| FocusVisualPrimaryThickness=2px | [FrameworkElement.FocusVisualPrimaryThickness](https://learn.microsoft.com/en-us/uwp/api/windows.ui.xaml.frameworkelement.focusvisualprimarythickness) |
| Button padding=11,5,11,6 | [Button_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/CommonStyles/Button_themeresources.xaml) |
| CheckBox size=20, spacing=8 | [CheckBox_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/CommonStyles/CheckBox_themeresources.xaml) |
| TextBox padding = 10,5,6,6 (asymmetric horizontal: 10 left, 6 right) | WinUI3 `TextControlThemePadding=10,5,6,6` per [Common_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/CommonStyles/Common_themeresources.xaml); right padding is intentionally smaller due to adjacent delete/clear button column (Width=30, collapsed by default) in TextBox template |
| Slider track=4, thumb=18, tick=4 | [Slider_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/CommonStyles/Slider_themeresources.xaml) — `SliderOutsideTickBarThemeHeight=4` |
| MenuFlyoutItem padding=11,8,11,9; icon placeholder=28px | [MenuFlyout_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/CommonStyles/MenuFlyout_themeresources.xaml) |
| ProgressBar min=3, track=1 | [ProgressBar_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/ProgressBar/ProgressBar_themeresources.xaml) |
| TabView min height=32, padding=8,3,4,3 | [TabView_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/TabView/TabView_themeresources.xaml) — `TabViewItemHeaderPadding` |
| ToolTip padding=9,6,9,8; maxWidth=320 | [ToolTip_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/CommonStyles/ToolTip_themeresources.xaml) |
| ListView item height=40 | [ListViewItem_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/CommonStyles/ListViewItem_themeresources.xaml) |
| ToggleSwitch 40×20, thumb 12/14 | [ToggleSwitch_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/CommonStyles/ToggleSwitch_themeresources.xaml) |
| ContentDialog 320-548 × 184-756, padding=24, button spacing=8, title=20px SemiBold | [ContentDialog_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/CommonStyles/ContentDialog_themeresources.xaml) |
| CommandBar 64/48, item spacing=0 (StackPanel), padding=4,0,0,0 | [CommandBar_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/CommonStyles/CommandBar_themeresources.xaml) |
| ProgressRing 32×32, stroke=4, min=16×16 | [ProgressRing_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/ProgressRing/ProgressRing_themeresources.xaml) and ProgressRing.xaml template (`MinWidth/MinHeight=16`) |
| Spacing token pixel values | [FluentUI spacings.ts](https://github.com/microsoft/fluentui/blob/master/packages/tokens/src/global/spacings.ts) |
| Spacing token names (XXSmall, sNudge etc.) | Informal shorthand. Fluent 2 uses `size20..size320` per [Fluent 2 Layout](https://fluent2.microsoft.design/layout). Code uses `spacingHorizontalXXS` etc. |
| SM_CXICON=32, SM_CXSMICON=16 | Standard Windows icon sizes, universally recognized |
| SHIL_SMALL=16, LARGE=32, EXTRALARGE=48, JUMBO=256 | [SHGetImageList](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shgetimagelist) |
| TextScaleFactor range 1.0–2.25 | [UISettings.TextScaleFactor](https://learn.microsoft.com/en-us/uwp/api/windows.ui.viewmanagement.uisettings.textscalefactor) |
| SPI_GETHIGHCONTRAST, SPI_GETCLIENTAREAANIMATION | [SystemParametersInfoW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-systemparametersinfow) |
| SystemFillColorCritical L=#c42b1c D=#ff99a4 | [Common_themeresources_any.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/CommonStyles/Common_themeresources_any.xaml) |
| SystemFillColorCaution L=#9d5d00 D=#fce100 | Same source |
| SystemFillColorSuccess L=#0f7b0f D=#6ccb5f | Same source |
| Status foreground colors (#ffffff/#1a1a1a) | No dedicated WinUI3 resource — these are conventional contrast values, not theme resources |
| Shadow: Fluent 2 two-layer elevation system | Per-elevation opacities: low L=14%/14%, D=28%/14%; high L=24%/20%, D=28%/20%. Per [Fluent 2 Elevation](https://fluent2.microsoft.design/elevation). Note: FluentUI React web tokens ([lightColor.ts](https://github.com/microsoft/fluentui/blob/master/packages/tokens/src/alias/lightColor.ts), [darkColor.ts](https://github.com/microsoft/fluentui/blob/master/packages/tokens/src/alias/darkColor.ts)) use different opacities (e.g. dark normal=24%/28%, dark darker=40%/48%) — values here follow the XAML/native design spec. |
| §2.19 Windows Fluent values use epx | Fluent defines Caption=12epx, Subtitle=20epx, Title=28epx, Display=68epx. Table now shows epx with pt equivalent at 96dpi in parentheses. |
| ComboBox min height=32, width=64, padding=12,5,0,7, arrow glyph=12, arrow area=38 | [ComboBox_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/ComboBox/ComboBox_themeresources.xaml) |
| Expander header=48, chevron button=32, glyph=12, content padding=16 | [Expander_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/Expander/Expander_themeresources.xaml) |
| HyperlinkButton padding=11,5,11,6 (inherits ButtonPadding) | [HyperlinkButton_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/CommonStyles/HyperlinkButton_themeresources.xaml) |
| Button has no MinHeight resource; effective ~27px | No `ButtonMinHeight` setter in [Button_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/CommonStyles/Button_themeresources.xaml) or generic.xaml. Effective = 14px (`ControlContentThemeFontSize`) + 5+6 padding + 2 border = 27px. `ContentDialogButtonHeight=32` is dialog-specific. |
| TextControlThemeMinHeight=32 confirmed | [generic.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/dxaml/xcp/dxaml/themes/generic.xaml) — `<x:Double x:Key="TextControlThemeMinHeight">32</x:Double>` |
| Button icon spacing 8px from hardcoded Margin | [DropDownButton.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/DropDownButton/DropDownButton.xaml) — `Margin="8,0,0,0"` on chevron icon; no named XAML resource |
| ListViewItemDisabledThemeOpacity: 0.3 (current), 0.55 (legacy) | [ListViewItem_themeresources.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/CommonStyles/ListViewItem_themeresources.xaml) =0.3; C++ fallback in [`ListViewBaseItemChrome.h`](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/dxaml/xcp/core/inc/ListViewBaseItemChrome.h): `GetDefaultDisabledOpacity()` returns 0.3 for rounded chrome, 0.55 for legacy non-rounded; [Win 8.x docs](https://learn.microsoft.com/en-us/previous-versions/windows/apps/jj709921(v=win.10)) confirm 0.55 as original UWP value |
| TextOnAccentFillColorPrimary L=#ffffff D=#000000 | [Common_themeresources_any.xaml](https://github.com/microsoft/microsoft-ui-xaml/blob/main/src/controls/dev/CommonStyles/Common_themeresources_any.xaml) — Light dict: #FFFFFF, Default (Dark) dict: #000000 |
| Spacing ramp: sizeNone..size320 (code); full Fluent 2 ramp extends to size560 (17 tokens) | [Fluent 2 Layout](https://fluent2.microsoft.design/layout); [FluentUI spacings.ts](https://github.com/microsoft/fluentui/blob/master/packages/tokens/src/global/spacings.ts) implements 11 tokens (sizeNone..size320) |

### KDE

| What was verified | Source |
|---|---|
| kdeglobals font keys and defaults | [kfontsettingsdata.cpp](https://github.com/KDE/plasma-integration/blob/master/qt6/src/platformtheme/kfontsettingsdata.cpp) and [fontssettings.kcfg](https://github.com/KDE/plasma-workspace/blob/master/kcms/fonts/fontssettings.kcfg) |
| QFont::toString() field layout | [Qt6 qfont.cpp](https://github.com/qt/qtbase/blob/dev/src/gui/text/qfont.cpp) — Qt6 6.4–6.10: 16 fixed fields + optional styleName (17th); Qt6 6.11+ (released 2026-03-23): minimum 19 fields (styleName always emitted + features/variableAxes counts). Parser should handle variable field counts. |
| Qt5 weights: Normal=50, DemiBold=63, Bold=75, Black=87 | [Qt5 qfont.h](https://github.com/qt/qtbase/blob/5.15/src/gui/text/qfont.h) — range is 0-99 (Black=87 is highest named constant, but values up to 99 are accepted) |
| Qt6 weights: Normal=400, DemiBold=600, Bold=700, Black=900 (range 1–1000) | [Qt6 qfont.h](https://github.com/qt/qtbase/blob/dev/src/gui/text/qfont.h) — named constants span 100–900 but the type accepts 1–1000 |
| Color group keys (all 12) and 7 sections | [kcolorscheme.cpp](https://github.com/KDE/kcolorscheme/blob/master/src/kcolorscheme.cpp) lines 252-341 |
| [WM] 6 keys | Verified in [BreezeLight.colors](https://invent.kde.org/plasma/breeze/-/raw/master/colors/BreezeLight.colors) and [BreezeDark.colors](https://invent.kde.org/plasma/breeze/-/raw/master/colors/BreezeDark.colors) |
| [Colors:Header] version KF 5.71 | ✅ Commit [fce11e205c](https://invent.kde.org/frameworks/kcolorscheme/-/commit/fce11e205c9cdd4e569a506c007eec2262b8d35d) (2020-05-20) landed between v5.70.0 and v5.71.0 tags. No `\since` annotation in header, but git history confirms. |
| **All breezemetrics.h constants** (incl. ComboBox_FrameWidth, MenuButton_IndicatorWidth, GroupBox_TitleMarginWidth, ItemView_ArrowSize, LineEdit_FrameWidth, ItemView margins) | [breezemetrics.h](https://github.com/KDE/breeze/blob/master/kstyle/breezemetrics.h) — every value confirmed exactly |
| QQC2 Switch/BusyIndicator dimensions (font-derived) | [SwitchIndicator.qml](https://invent.kde.org/plasma/qqc2-breeze-style/-/blob/master/style/impl/SwitchIndicator.qml), [Units.qml](https://invent.kde.org/plasma/qqc2-breeze-style/-/blob/master/style/impl/Units.qml), [BusyIndicator.qml](https://invent.kde.org/plasma/qqc2-breeze-style/-/blob/master/style/qtquickcontrols/BusyIndicator.qml) |
| MenuItem_TextLeftMargin=8 (v6.5.3+) | Commit [35967f0a](https://invent.kde.org/plasma/breeze/-/commit/35967f0a3c3d) (2025-11-17), shipped between v6.5.2 and v6.5.3 tags |
| Layout_TopLevelMarginWidth=10, ChildMarginWidth=6, DefaultSpacing=6 | Same source |
| Icon sizes come from icon theme's index.theme, not kdeglobals | [kicontheme.cpp](https://github.com/KDE/kiconthemes/blob/master/src/kicontheme.cpp) lines 160-167 and 468-473 — C++ fallbacks used only when theme omits a key. Breeze sets DesktopDefault=48 (C++ fallback=32), PanelDefault=48 (matches C++ fallback; was 32 until KF5 v5.34.0) |
| MenuItem_MarginHeight history: 3→5→4 | Commit [35967f0a](https://invent.kde.org/plasma/breeze/-/commit/35967f0a) (2025-11-17) changed 3→5; commit [2cd5b37d](https://invent.kde.org/plasma/breeze/-/commit/2cd5b37d) (2025-11-19) changed 5→4 |
| forceFontDPI in kcmfontsrc (KConfig appends "rc") | Historically X11-only (Plasma 5 guarded UI with `#if HAVE_X11`); [commit f97930a](https://github.com/KDE/plasma-desktop/commit/f97930a8cc3b620a2b780ebf0df685ba36188cfa) removed X11 guard; [issue #62](https://invent.kde.org/plasma/plasma-desktop/-/issues/62) approved removing for Wayland. In Plasma 6: [fonts KCM main.qml](https://github.com/KDE/plasma-workspace/blob/master/kcms/fonts/ui/main.qml) line 427 hides UI on Wayland (`visible: Qt.platform.pluginName === "xcb"`); [plasma6.0-remove-dpi-settings.cpp](https://github.com/KDE/plasma-workspace/blob/master/kcms/fonts/kconf_update/plasma6.0-remove-dpi-settings.cpp) deletes `forceFontDPIWayland` on upgrade. Config key still works if set manually. |
| AnimationDurationFactor in kdeglobals [KDE], 0=disabled | ✅ Confirmed per [kwin.kcfg](https://invent.kde.org/plasma/kwin/-/blob/master/src/kwin.kcfg) (`<min>0</min>`); 0 yields `std::max(defaultTime * 0, 1.)` = 1ms per [effect.cpp](https://github.com/KDE/kwin/blob/master/src/effect/effect.cpp) — effectively instant, not literally zero; [Phabricator D28651](https://phabricator.kde.org/D28651), [bug 431259](https://bugs.kde.org/show_bug.cgi?id=431259) |
| Breeze PanelDefault=48 (matches C++ fallback) | [breeze-icons commonthemeinfo.theme.in](https://github.com/KDE/breeze-icons/blob/master/icons/commonthemeinfo.theme.in) — `PanelDefault=48`; C++ fallback in [kicontheme.cpp](https://github.com/KDE/kiconthemes/blob/master/src/kicontheme.cpp) is also 48. Was 32 until KF5 v5.34.0 (~2017), changed to 48 in later versions. |
| Dialog button spacing = Layout_DefaultSpacing = 6 | QDialogButtonBox uses `PM_LayoutHorizontalSpacing` → Breeze returns `Layout_DefaultSpacing` = 6 per [breezemetrics.h](https://github.com/KDE/breeze/blob/master/kstyle/breezemetrics.h). `Button_ItemSpacing`=4 is icon-to-label gap inside a single button, not inter-button spacing. |
| `[General] AccentColor` propagates to `DecorationFocus` | [colorsapplicator.cpp](https://invent.kde.org/plasma/plasma-workspace/-/blob/master/kcms/colors/colorsapplicator.cpp) — reads `AccentColor` from `[General]`; applies to `ForegroundActive`, `ForegroundLink`, `DecorationFocus`, `DecorationHover` across color groups |

### GNOME

| What was verified | Source |
|---|---|
| gsettings font keys and GNOME 48+ defaults | [gsettings-desktop-schemas](https://github.com/GNOME/gsettings-desktop-schemas/blob/master/schemas/org.gnome.desktop.interface.gschema.xml.in) — commit 067cb4b changed to Adwaita Sans; all font keys default to size 11 |
| Pre-48 defaults (Cantarell 11, Source Code Pro 10) | Same repo, parent commit |
| titlebar-font key | [org.gnome.desktop.wm.preferences schema](https://github.com/GNOME/gsettings-desktop-schemas/blob/master/schemas/org.gnome.desktop.wm.preferences.gschema.xml.in) |
| All 8 libadwaita type scale classes (percentages and weights) | [libadwaita src/stylesheet/widgets/_labels.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_labels.scss) |
| D-Bus portal color-scheme, accent-color, contrast, reduced-motion | [XDG Desktop Portal Settings spec](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Settings.html) |
| $button_radius=9px, $card_radius=12px | [libadwaita src/stylesheet/_common.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/_common.scss) |
| --window-radius = $button_radius+6 = 15px | Same source |
| AdwAlertDialog radius = 18px ($alert_radius), not $dialog_radius (15px) | [libadwaita src/stylesheet/widgets/_message-dialog.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_message-dialog.scss) — `$alert_radius: 18px` |
| --disabled-opacity: 50% | [libadwaita src/stylesheet/_colors.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/_colors.scss) |
| Focus ring: 2px outline-width, -2px offset | focus-ring mixin in _drawing.scss — `@mixin focus-ring($width: 2px)`, `$offset: -$width` |
| Button min-height=24px, padding=5px 10px | [libadwaita _buttons.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_buttons.scss) |
| Entry min-height=34px | [libadwaita _entries.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_entries.scss) |
| CheckButton indicator=14px, padding=3px | [libadwaita _checks.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_checks.scss) |
| Scale trough=10px, thumb=20px | [libadwaita _scale.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_scale.scss) |
| ProgressBar=8px | [libadwaita _progress-bar.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_progress-bar.scss) |
| Notebook tab=30px | [libadwaita _notebook.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_notebook.scss) |
| Scrollbar slider=8px | [libadwaita _scrolling.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_scrolling.scss) — `$_slider_width: 8px` |
| Tooltip padding=6px 10px | [libadwaita _tooltip.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_tooltip.scss) |
| GtkSwitch thumb=20×20, track radius=14px, total ~46×26px | libadwaita switch SCSS + derived calculation |
| GtkSpinner=16×16 | [GTK4 gtkspinner.c](https://gitlab.gnome.org/GNOME/gtk/-/blob/main/gtk/gtkspinner.c) — `#define DEFAULT_SIZE 16` |
| AdwAlertDialog 300/372/600sp | [adw-alert-dialog.c source](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/adw-alert-dialog.c) |
| AdwExpanderRow header=50px | [libadwaita _lists.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_lists.scss) — `min-height: 50px` |
| GtkDropDown arrow=16×16, box spacing=6px | [libadwaita _dropdowns.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_dropdowns.scss) |
| GtkExpander arrow=16×16 | [libadwaita _expanders.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_expanders.scss) |
| AdwAlertDialog spacing (button=12px, message=24px/32px, response=24px/12px) | [libadwaita _message-dialog.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_message-dialog.scss) |
| Headerbar min-height=47px | [libadwaita _header-bar.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_header-bar.scss) |
| overlay-scrolling gsettings + gtk-overlay-scrolling | [org.gnome.desktop.interface schema](https://gitlab.gnome.org/GNOME/gsettings-desktop-schemas/-/blob/master/schemas/org.gnome.desktop.interface.gschema.xml.in) and [gtksettings.c](https://gitlab.gnome.org/GNOME/gtk/-/blob/main/gtk/gtksettings.c) |
| Card radius=12px | = $card_radius in _common.scss |
| GTK4 has three GtkIconSize values: INHERIT(0), NORMAL(1), LARGE(2) | [GtkIconSize enum docs](https://docs.gtk.org/gtk4/enum.IconSize.html) |
| Icon pixel sizes (16px, 32px) are theme-defined | Adwaita CSS: `.normal-icons { -gtk-icon-size: 16px }`, `.large-icons { -gtk-icon-size: 32px }` |
| text-scaling-factor, high-contrast, enable-animations | [gsettings-desktop-schemas](https://github.com/GNOME/gsettings-desktop-schemas) |
| Portal contrast and reduced-motion preferences | [XDG Desktop Portal Settings spec](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Settings.html) |
| Entry padding: 9px horizontal, no explicit vertical | [libadwaita _entries.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_entries.scss) — `padding-left: 9px; padding-right: 9px;` no vertical padding set; vertical space from `min-height: 34px` |
| Menu item padding: 0 12px ($menu_padding=12), min-height=32 | [libadwaita _menus.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/widgets/_menus.scss) — `popover.menu modelbutton { padding: 0 $menu_padding; min-height: 32px }`. `$menu_padding=12` and `$menu_margin=6` from [_common.scss](https://gitlab.gnome.org/GNOME/libadwaita/-/blob/main/src/stylesheet/_common.scss) |
| Pango font format includes optional VARIATIONS and FEATURES | [Pango FontDescription.from_string](https://docs.gtk.org/Pango/type_func.FontDescription.from_string.html) — format: `[FAMILY-LIST] [STYLE-OPTIONS] SIZE [VARIATIONS] [FEATURES]` |

### Cross-Platform / Font Metrics

| What was verified | Source |
|---|---|
| macOS text style sizes and weights (all 11 styles) | [Apple HIG Typography JSON](https://developer.apple.com/tutorials/data/design/human-interface-guidelines/typography.json) — macOS built-in text styles table. Key confirmations: `.headline`=13pt **Bold**, `.caption1`=10pt Regular, `.caption2`=10pt **Medium (500)**. Per-style line heights also documented (e.g. body 13/16, headline 13/16). |
| Noto Sans sTypo metrics: ascender=1069, descender=293, UPM=1000 | [Google Fonts Noto contribution guidelines](https://github.com/notofonts/noto-source/blob/main/FONT_CONTRIBUTION.md) — fully shaped text must fit within (1069, -293). Confirmed Roboto-compatible metrics. |
| Cantarell metrics: ascender=739, descender=217, lineGap=244, UPM=1000; hhea: 983/−217/0 | [Cantarell-Regular.ufo/fontinfo.plist](https://gitlab.gnome.org/GNOME/cantarell-fonts/-/blob/master/src/Cantarell-Regular.ufo/fontinfo.plist) — ascender/descender/UPM from UFO source; sTypoLineGap=244 confirmed from compiled font binary (`Cantarell-VF.otf` v0.311) via fontTools inspection. `USE_TYPO_METRICS` (fsSelection bit 7) is NOT set (`fsSelection=0x0040`); hhea table: hheaAscender=983 (=739+244, lineGap folded into ascender), hheaDescender=−217, hheaLineGap=0. Both metric sets yield the same 1.2 total: sTypo (739+217+244)/1000=1.2, hhea (983+217)/1000=1.2. Win metrics (usWinAscent=983, usWinDescent=217) also match. |
| Inter (Adwaita Sans basis) metrics: typoAscender=1984, typoDescender=-494, lineGap=0, UPM=2048 | [Inter fontinfo.json](https://github.com/rsms/inter/blob/master/docs/_data/fontinfo.json) — yields (1984+494)/2048=1.2099≈1.21. `USE_TYPO_METRICS` IS set in Inter/Adwaita Sans (fsSelection bit 7). |
| SF Pro metrics: ascender=1950, descender=494, lineGap=0, UPM=2048 | SF Pro is proprietary (not on GitHub); values confirmed by font file inspection with fontTools/FontForge from [Apple's download](https://developer.apple.com/fonts/). Ratio (1950+494)/2048=1.19. |
| GetSysColor Win10+ supported constants (8 total) | [MSDN GetSysColor](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsyscolor) — confirmed: COLOR_WINDOW, COLOR_WINDOWTEXT, COLOR_HIGHLIGHT, COLOR_HIGHLIGHTTEXT, COLOR_3DFACE, COLOR_GRAYTEXT, COLOR_BTNTEXT, COLOR_HOTLIGHT. COLOR_3DFACE (value 15) is not marked "not supported" but its alias COLOR_BTNFACE (same value 15) is — confirmed documentation inconsistency. |
| SM_CYMENU = menu BAR height, not menu item | [MSDN GetSystemMetrics](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics) — "The height of a single-line menu bar, in pixels." |
| Win32 popup menu item height = font-derived ~20px | Formula from [Wine `menu.c`](https://github.com/wine-mirror/wine/blob/master/dlls/user32/menu.c) `MENU_CalcItemSize`: `max(text_height + 2, char_height + 4)`. At 96 DPI with Segoe UI 9pt (cell height ~16px): max(18, 20) = 20px. The 20px result coincidentally equals SM_CYMENU but is derived from a different formula. |
| macOS Tahoe = macOS 26, confirmed name | [Wikipedia](https://en.wikipedia.org/wiki/MacOS_Tahoe), [MacRumors](https://www.macrumors.com/roundup/macos-26/) — announced WWDC 2025, released September 15, 2025 |
| Dialog button order: macOS primary rightmost | ✅ Apple HIG: "A button that initiates an action is furthest to the right, Cancel to its left." [Thomas Tempelmann analysis](https://www.tempel.org/DialogButtonPlacement) |
| Dialog button order: Windows primary leftmost | ✅ [MS Command Buttons guideline](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/bb246415(v=vs.85)): OK first, then Cancel, then Apply |
| Dialog button order: KDE OK→Apply→Cancel | ✅ Qt source [qplatformdialoghelper.cpp](https://codebrowser.dev/qt5/qtbase/src/gui/kernel/qplatformdialoghelper.cpp.html): KdeLayout horizontal = Help, Reset, [Stretch], Yes, No, Action, **Accept**, Alternate, **Apply**, Destructive, **Reject** |
| Dialog button order: GNOME cancel left, affirmative right | ✅ [GNOME HIG dialogs](https://developer.gnome.org/hig/patterns/feedback/dialogs.html): "cancel button appears first, before the affirmative button" |
| Noto Sans lineGap=0 | ✅ [FONT_CONTRIBUTION.md](https://github.com/notofonts/noto-source/blob/main/FONT_CONTRIBUTION.md): "Roboto Regular's metrics translated for 1000em" — sTypoAscender=1069, sTypoDescender=-293, sTypoLineGap=0 |
| Inter metrics confirmed | ✅ [Inter fontinfo.json](https://github.com/rsms/inter/blob/master/docs/_data/fontinfo.json): sTypoAscender=1984, sTypoDescender=-494, sTypoLineGap=0, UPM=2048 |
| WinUI3 Card control still open proposal | ✅ [Issue #6543](https://github.com/microsoft/microsoft-ui-xaml/issues/6543) still open (verified 2026-03-24) |
| NSSegmentedControl.Style 8 cases | ✅ [Apple docs](https://developer.apple.com/documentation/appkit/nssegmentedcontrol/style) + [mackuba.eu guide](https://mackuba.eu/2014/10/06/a-guide-to-nsbutton-styles/): automatic, rounded, roundRect, texturedRounded, capsule, texturedSquare, smallSquare, separated |
