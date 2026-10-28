# Gap map — KDE Plasma 6 / Breeze (`kde-breeze`)

Model at caeddd6b (`docs/property-registry.toml`, `native-theme/src/kde/`). Method: `docs/todo_v0.7.1_model-completeness-gap-map.md` §2–§4. This file collects no values: where a cell quotes a constant or a factor, it was read at the cited line, and it is there only to say *what* the gap is.

## Sources and abbreviations

All sources are the KDE GitHub mirrors, shallow clones of `master` read on 2026-10-27. **Before values are collected, every citation must be re-pinned to the release tags the rest of `platform-facts.md` uses (Breeze/KWin v6.7.x, KF 6.31.0), because line numbers move.**

| Abbrev. | File | Repository @ commit |
|---|---|---|
| `st` | `kstyle/breezestyle.cpp` | KDE/breeze @ `75561b7f32f3121603a0f2c8ea929b20a0d6c311` |
| `hp` / `hp.h` | `kstyle/breezehelper.cpp` / `.h` | KDE/breeze @ `75561b7f` |
| `mt` | `kstyle/breezemetrics.h` | KDE/breeze @ `75561b7f` |
| `kcfg` | `kstyle/breeze.kcfg` (breezerc, groups `[Common]`, `[Style]`) | KDE/breeze @ `75561b7f` |
| `shd` | `kstyle/breezeshadowhelper.cpp` | KDE/breeze @ `75561b7f` |
| `anim` | `kstyle/animations/breezeanimations.cpp` (and siblings) | KDE/breeze @ `75561b7f` |
| `tam` | `kstyle/breezetoolsareamanager.cpp` | KDE/breeze @ `75561b7f` |
| `deco` / `deco.h` | `kdecoration/breezedecoration.cpp` / `breeze.h` | KDE/breeze @ `75561b7f` |
| `dbtn` | `kdecoration/breezebutton.cpp` | KDE/breeze @ `75561b7f` |
| `dkcfg` | `kdecoration/breezesettingsdata.kcfg` (breezerc `[Common]`, `[Windeco]`) | KDE/breeze @ `75561b7f` |
| `colors` | `colors/BreezeLight.colors`, `BreezeDark.colors` | KDE/breeze @ `75561b7f` |
| `kcs` | `src/kcolorscheme.cpp` | KDE/kcolorscheme @ `8bb948c4e768800e456b7e8afe0938233a3348a9` |
| `khint` | `qt6/src/platformtheme/khintssettings.cpp` | KDE/plasma-integration @ `46eddde9d38cf907786e567dd4a942140bb1f797` |
| `kfont` | `qt6/src/platformtheme/kfontsettingsdata.cpp` | KDE/plasma-integration @ `46eddde9` |
| `kmw` | `src/kmessagewidget.cpp` | KDE/kwidgetsaddons @ `17cca25bc8ca08063329c9432b2f96a03230bff5` |
| `kwa` | `src/<file>` | KDE/kwidgetsaddons @ `17cca25b` |
| `kie` / `kit` | `src/kiconeffect.cpp` / `src/kicontheme.cpp` | KDE/kiconthemes @ `60bdbcdd55a5b8c35e6f546e0cdfa5ea483e6fb8` |
| `ktb` | `src/ktoolbar.cpp` | KDE/kxmlgui @ `b5aa1026102a2eb332a07ca629e785535d49f8a7` |
| `kwin` | `src/<path>` | KDE/kwin @ `625e3132326c9ff181a9af342be59fd92f05d8fa` |
| `kirigami` | `src/<path>` | KDE/kirigami @ `81ada72b021fe7e34ec4589c9a0a66dc5cf4dbe8` |
| `qqc2` | `<path>` | KDE/qqc2-desktop-style @ `305eb2c44fdc3eb38eebea3b0e6b493cf12d1e5e` |

Source kinds: **read** (a key the reader can query), **pub** (published source), **measure** (on the maintainer's Plasma 6 desktop: a Qt Widgets probe app and a Kirigami/QQC2 probe app, captured with Spectacle / KWin's ScreenShot2 D-Bus interface; animations by screen recording at the display refresh rate, or by Qt's animation trace). "Model today" names the nearest field; *partial* means a field exists but cannot hold what Breeze draws.

Recurring Breeze facts the rows lean on (so they need not be repeated): `focusColor` = `[Colors:View] DecorationFocus`, `hoverColor` = `[Colors:View] DecorationHover` (`hp.h:64-73`); `separatorColor` = mix(Window, WindowText, frameContrast) (`hp:329-332`); `frameBackgroundColor` = mix(Window, Base, `Blend_Value`) (`hp:200-203`); `arrowColor` = mix(text role, its background, `arrowShade` 0.15) (`hp:38`, `:206-219`); the neutral-highlight variant is any widget carrying the `_kde_highlight_neutral` property (`st:8756-8772`; `breezepropertynames.cpp:18`); `hasHighlightNeutral` recolours frames to `[Colors:View] ForegroundNeutral`.

---

## 4.1 Elements the model has: missing parts, states, properties

### Window and its decoration (Breeze KDecoration, KWin)

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.1 | window · title bar | Title bar **height**: max(title-font height, decoration button size) + top and bottom margins | pub `deco:398-418`, `deco:724-728`; margins `deco.h:32-35` (`TitleBar_TopMargin`, `_BottomMargin`, in units of `smallSpacing`) | none |
| K1.2 | window · title bar | Title bar **side padding** and caption insets (`TitleBar_SideMargin` × `smallSpacing`) | pub `deco.h:34`; `deco:514-575`, `deco:730-780` | none |
| K1.3 | window · title | **Title alignment** (Left / Center / CenterFullWidth / Right; default CenterFullWidth with fallback to left/right when it collides with buttons) and middle **elision** of the caption | read breezerc `[Windeco] TitleAlignment` (`dkcfg:70-78`); pub `deco:730-780`, elision `deco:694-697` | none |
| K1.4 | window · buttons (part) | The **decoration buttons** as a part: size (enum Tiny…VeryLarge = gridUnit × 1 / 1.5 / 2 / 2.5 / 3.5), spacing (`TitleBar_ButtonSpacing`), Spacer width (half a button) | read breezerc `[Windeco] ButtonSize` (`dkcfg:81-90`); pub `deco:705-722`, `deco.h:35`, `dbtn:390-408` | none |
| K1.5 | window · buttons | **Button layout**: which buttons, on which side, in which order (menu, on-all-desktops, exclude-from-capture left; help, minimize, maximize, close right by default) | read kwinrc `[org.kde.kdecoration2] ButtonsOnLeft`, `ButtonsOnRight` (`kwin src/decorations/settings.cpp:140-150`) | none |
| K1.6 | window · button glyphs | The **glyphs**: close ✕, maximize/restore chevrons, minimize, keep-above/below, shade, on-all-desktops pin, context help, application menu, exclude-from-capture; drawn in an 18×18 box of a 20-unit grid, stroke `PenWidth::Symbol` (1.01) × scale, round cap; the window-menu button shows the window icon recoloured to the title colour | pub `dbtn:140-303`, `deco.h:54`; menu icon `dbtn:110-127` | none (no icon role for decoration glyphs) |
| K1.7 | window · button states | Per-state paint: rest = glyph in title text colour, no fill; **hover** = filled circle in title text colour, glyph in title bar colour; **close hover** = circle in `Warning` foreground (Header-set `ForegroundNegative`), `.lighter()` when the window is active; **pressed** = mix(title bar, title text, 0.3), close pressed = red `.darker()`; **checked** toggles (keep above/below, shade, exclude-from-capture) filled; `OutlineCloseButton` option draws the close circle always | pub `dbtn:306-387`; red source `kwin src/decorations/decorationpalette.cpp:111-112`; read breezerc `[Common] OutlineCloseButton` (`dkcfg:34-36`) | none |
| K1.8 | window · button motion | Hover fade (`QEasingCurve::InOutQuad`), duration taken from the decoration's own animation, which the source sets to 0 (so no animation at this commit) — the model cannot state "instant" either | pub `dbtn:24-35`, `dbtn:407`, `deco:383-386` | none |
| K1.9 | window · outline | The **window outline** (a 1-device-pixel line around the window, outside the frame): on/off, colour = mix(Frame role of the active/inactive group, palette text, frameContrast), hidden when maximized, per-corner radii | read breezerc `[Common] OutlineEnabled` (`dkcfg:38-40`); pub `deco:474-497` | partial: `window.border.color` / `line_width` (no on/off, no inactive colour, no "outside" semantics) |
| K1.10 | window · corners | **Per-corner radius** and its rules: top corners rounded unless tiled/maximized; bottom corners rounded only with borders or `RoundedCorners`; radius = `Frame_FrameRadius` 2.5 × `smallSpacing` (2 on Wayland) | read breezerc `[Common] RoundedCorners` (`dkcfg:42-44`); pub `deco:440-473`, `deco:870-876`, `deco.h:29` | partial: `window.border.corner_radius` (one value, no maximized/tiled state) |
| K1.11 | window · frame borders | **Border (frame) width** by size enum (None, NoSides, Tiny, Normal, Large, VeryLarge, Huge, VeryHuge, Oversized = multiples of `smallSpacing`), per-window exceptions, bottom-only for NoSides | read kwinrc `[org.kde.kdecoration2] BorderSize`, `BorderSizeAuto` (`kwin src/decorations/settings.cpp:164-174`); breezerc `[Windeco] BorderSize` (`dkcfg:54-67`); pub `deco:300-360` | none (`window.border.line_width` is the outline) |
| K1.12 | window · frame colour | Frame fill colour, active and inactive (KDecoration `ColorRole::Frame`) | pub `deco:584-600`; `kwin src/decorations/decorationpalette.cpp:86-95` | partial: `window.border.color` (no inactive variant) |
| K1.13 | window · title bar colours (source) | KWin takes title bar, frame and title text colours from **`[Colors:Header]`** (active) and **`[Colors:Header][Inactive]`** (inactive) whenever the scheme has a Header group; `[WM]` keys are only the legacy fallback. The reader reads `[WM]` | read kdeglobals `[Colors:Header]`, `[Colors:Header][Inactive]`; pub `kwin src/decorations/decorationpalette.cpp:126-160` | the fields exist (`title_bar_background`, `inactive_title_bar_*`); the **reader's source** is the gap |
| K1.14 | window · title bar gradient | Optional title bar gradient (`lighter(120)` at the top to the colour at 0.8), active window only | read breezerc `[Windeco] DrawBackgroundGradient` (`dkcfg:98-100`); pub `deco:647-657` | none |
| K1.15 | window · options | `HideTitleBar` (title bar merged into the frame), `DrawBorderOnMaximizedWindows` | read breezerc `[Windeco]` (`dkcfg:93-105`) | none |
| K1.16 | window · states | **Maximized / tiled / shaded** states change corners, borders, outline, shadow | pub `deco:440-497`, `deco:633-690` | none |
| K1.17 | window · activation motion | Active↔inactive **shadow** transition: duration `AnimationDurationFactor` × 100 ms, `OutCubic` on activation, `InCubic` on deactivation; title colour cross-fade exists with duration 0 | pub `deco:185-200`, `deco:275-297`, `deco:383-389` | none |
| K1.18 | window · inactive shadow | The inactive window's shadow is the active one at half strength | pub `deco:783-809` (`createShadowObject(0.5)`) | none (see K3.1) |
| K1.19† | window · quick-tile outline | The **quick-tile / snap preview** KWin shows while a window is dragged to a screen edge: a frame from the Plasma desktop theme's SVG `widgets/translucentbackground` (not from the colour scheme), its borders dropped on the edges that touch the maximize area, moving from the window's geometry to the tile's with x, y, width and height animated Kirigami `longDuration`, `InOutCubic` | read kwinrc `[Outline] QmlPath` (`kwin src/outline.cpp:150-152`); pub `kwin src/qml/outline/plasma/outline.qml:16`, `:65-108`, `:110-125`; the SVG is libplasma's Breeze desktop theme (not read, to verify) | none |

### Button (QPushButton; QQC2 Button paints through Breeze)

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.20 | button · border per state | **Border colour per state**: hover, keyboard focus and pressed turn the pen `Highlight`; checked pen = mix(Button, ButtonText, frameContrast); default button pen = mix(Highlight, frame colour, 0.5) | pub `hp:647-741` (`:685-712`) | partial: `button.border.color` (rest only) |
| K1.21 | button · keyboard focus | **Keyboard-only focus state** (`State_KeyboardFocusChange`, no focus proxy) distinct from pointer focus; it drives the highlighted pen and the focus frame (K3.20) | pub `st:4516-4540` | none |
| K1.22 | button · default | **Default (dialog-default) button**: fill mix(Button, Highlight, 0.2), pen as K1.20, only while the window is active | pub `hp:700-703` | partial: `primary_background` / `primary_text_color` (mapped to the accent; no border, no inactive-window rule) |
| K1.23 | button · drop shadow | **Shadow layer**: one 1.001 px stroke, black at alpha 0.125, its rect extended half a pixel down, on rest, enabled, non-flat, non-checked buttons of the **active** window only | pub `hp:727-732`, `hp:1646-1660`, `hp.h:88-91` | partial: `border.shadow_enabled` (no geometry, colour, or state rule) |
| K1.24 | button · flat variant | **Flat button** (`QPushButton::setFlat`, also auto-raise tool buttons): transparent at rest; pressed `Highlight` α `Blend_Value`; checked `ButtonText` α 0.125 + frame pen; default-flat `Highlight` α 0.125; hover pen `Highlight`; label in `WindowText` | pub `hp:678-695`; text role `st:5327-5360` | none |
| K1.25 | button · neutral variant | Neutral-highlight variant: fill mix(Button, NeutralText, `Blend_Value`) when checked, pen `NeutralText` | pub `hp:690-707` | none |
| K1.26 | button · round variant | **Round button** (`_kde_round_button`): radius = half the larger side | pub `hp:727`, `:737`; `st:4529-4535` | none |
| K1.27 | button · menu indicator | **Menu button** (`HasMenu`): down arrow in a `MenuButton_IndicatorWidth` box at the trailing edge, `arrowColor(text role)`, label shortened by `Button_ItemSpacing` | pub `st:5370-5380`; `mt:84` | none |
| K1.28 | button · icon | Button **icon size** (`PM_SmallIconSize`, or the button's own) and whether icons show at all on push buttons | read kdeglobals `[KDE] ShowIconsOnPushButtons` (`khint:116`, `:281`); pub `st:5340-5380` | partial: `defaults.icon_sizes.small`; no visibility flag |
| K1.29 | button · state combos | checked+hover (checked fill, highlight pen), checked+pressed (pressed wins), default+hover — combinations the per-state fields cannot express | pub `hp:678-741` | none |
| K1.30 | button · motion | Fill (pressed) and pen (hover/focus) **cross-fades** via the widget-state engine (see K3.31) | pub `st:4539-4543`; `hp:713-724` | none |

### Text input (QLineEdit) and text area (QTextEdit)

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.40 | input · neutral/warning state | **Neutral highlight** (validation warning) outline `ForegroundNeutral`, `lighter(150)` on hover/focus | pub `st:4083-4085`, `st:4131-4133` | none (no invalid/warning state) |
| K1.41 | input · frameless variant | **Frameless** line edit (frame off, or too short for frame + font): only the `Base` fill | pub `st:4062-4068` | none |
| K1.42 | input · per-side borders | **Per-side borders** (`_breeze_borders_sides`: only chosen edges drawn, hover/focus outline inside) | pub `st:3982-4058`; frames `st:3950-3956`, `hp:424-473` | partial: `Border` has one colour/width for all four sides |
| K1.43 | input · trailing actions | Built-in **action icons** inside the field: clear button (`setClearButtonEnabled`), KPasswordLineEdit reveal toggle; icon, size, colour per state | pub qtbase `qlineedit_p.cpp` (side widgets; to verify lines), `kwa kpasswordlineedit.cpp` (to verify lines); measure: probe app | none |
| K1.44 | input · hover↔focus motion | Outline cross-fade hover→focus (input-widget engine) | pub `st:4073-4079`, `hp:142-169` | none (K3.31) |
| K1.45 | input · inactive-window selection | Selected text in an inactive window uses the Inactive palette (see K3.40) | pub `kcs:303-321` | partial: `defaults.selection_inactive_background` only |
| K1.46† | input · clear button | Qt's built-in **clear button** and other side icons (K1.43 is the part): icon size `PM_LineEditIconSize` = `PM_SmallIconSize` (Breeze does not override it), button box icon + 6 by icon + 2, icon drawn in the Disabled, Normal or Active (pressed) icon mode; a **160 ms opacity fade** (Qt's default easing) as the text appears or is cleared; icon `SP_LineEditClearButton` (theme icon name to verify) | pub qtbase v6.10.1 `qlineedit_p.cpp:353-366`, `:418-441`, `:494-510`; `qcommonstyle.cpp:4797-4799` | none |
| K1.47† | input · block caret | With an **input mask** and no selection, the line edit's caret is a **blinking block**: the character under the cursor drawn in `Window` on `Text`; QTextEdit/QPlainTextEdit in **overwrite mode** widen the caret by the next character's advance (a space's at the line end) | pub qtbase v6.10.1 `qwidgetlinecontrol.cpp:619-625`; `qwidgettextcontrol.cpp:1425-1445` | partial: `input.caret_color` (one thin caret) |
| K1.48† | input · password masking | Masked text: **mask character** U+25CF and **mask delay** 0 ms (the last typed character is never shown), Qt's defaults, since neither Breeze nor the KDE platform theme sets the hints; `KPasswordLineEdit`'s trailing **reveal action** (theme icon `visibility`, a bundled SVG as fallback) shows only while the field has text (Quick: Q1.19) | pub qtbase v6.10.1 `qcommonstyle.cpp:5152-5160`, `qplatformtheme.cpp:617-620`; no `PasswordMask*` hint in `khint:86-133`; `kwa kpasswordlineedit.cpp:40-58` | none |
| K1.49† | input · misspelling underline | Spell-checked text (`KTextEdit` through Sonnet's highlighter): misspelled words get `QTextCharFormat::SpellCheckUnderline`, whose style is the platform hint `SpellCheckUnderlineStyle` (Qt's default `WaveUnderline`; neither Breeze nor the KDE platform theme sets it), coloured **`Qt::red`** unless the app sets a colour: a hard-coded colour, not a scheme role (Quick: Q1.13 uses `negativeTextColor`) | pub KDE/sonnet @ `5442e3b4` `src/ui/highlighter.cpp:89`, `:401-402`; qtbase v6.10.1 `qplatformtheme.cpp:658-659` | none (`defaults.danger_color` is not this colour) |

Checked, no gap: a QTextEdit's frame hover/focus outline is Breeze's frame-shadow overlay (`kstyle/breezeframeshadow.cpp`; `st:3964-3970`) in the line edit's colours, so `text_area` adds nothing beyond K1.40–K1.45.

### Checkbox / radio button

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.50 | checkbox · indeterminate | **Partially checked** state: three 2×2 dots in `Text` | pub `hp:948-952`; `st:4920-4926` | none |
| K1.51 | checkbox · pressed | **Pressed** (sunken) fill: `Button.darker(110)` (`radioCheckSunkenDarkeningFactor`) | pub `hp:39`, `hp:862`, `hp:1012` | none |
| K1.52 | checkbox · hover ring | **Hover ring**: a 1 px ring inside the box in `focusColor` (neutral variant: `NeutralText.lighter()`), fading in | pub `hp:897-907`; radio `hp:1046-1057` | partial: `hover_background` (Breeze adds no fill) |
| K1.53 | checkbox · visual box vs cell | The drawn box is inset 2 px inside the `CheckBox_Size` cell; corner radius `CheckBox_Radius` = `Frame_FrameRadius` − 1 | pub `hp:840-857`; `mt:87-90` | partial: `indicator_width` is the cell; no inset field (radius is data) |
| K1.54 | checkbox · state combos | checked+hover, checked+pressed, partial+hover, disabled+checked (Disabled-group highlight pen) | pub `hp:831-981` | partial: `checked_background`, `disabled_*` only |
| K1.55 | checkbox · motion | **Check animation**: the mark is drawn on along its path (dash pattern × progress); partial dots appear in thirds; the radio dot grows from the centre; the tint fades in; duration `AnimationsDuration` | pub `hp:869-871`, `hp:955-972`, `hp:1069-1075`; `st:4926-4935` | none |
| K1.56 | checkbox · neutral variant | Neutral highlight: pen and tint `NeutralText` | pub `hp:846-856`, `hp:995-1006` | none |
| K1.57 | checkbox · focus | Keyboard **focus frame** around the indicator (square or circular, see K3.20) | pub `st:6655-6698` | partial: `defaults.focus_ring_*` |
| K1.58† | checkbox · mark geometry | The **check mark** does not fill the box: a two-segment path from (box left + 4, centre), down to (centre x − 1, box bottom − 5), up to (centre x + 4.5, box top + 5.5), stroked `Text` at 2 × `PenWidth::Frame` (2.002) with a mitre join; the partial state's three 2 × 2 dots sit 4 px apart about the centre — insets the facts' "(none) — checkmark fills indicator" padding cells leave out | pub `hp:894-895`, `hp:913-933`, `hp:938-940`; `mt:25` (`PenWidth::Frame`) | none (`checkbox.border.padding_*` facts: none) |

### Menu (QMenu; QQC2 Menu/MenuItem paint through Breeze)

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.60 | menu · hovered item | Hovered item = **`focusColor` at α `Blend_Value` fill + 1 px outline** `focusOutlineColor` = mix(focus, WindowText, 0.15), rounded by `frameRadius`; text stays `WindowText` | pub `st:6024-6047`, `st:6159-6160`; `hp:172-175`; `hp:369-405` | partial: `hover_background`, `hover_text_color` (no item border, no item radius; see also D1) |
| K1.61 | menu · pressed item | **Pressed** item: solid `focusColor` fill, `HighlightedText` text, icon in Selected mode, submenu arrow `HighlightedText` | pub `st:6027-6031`, `st:6111-6120`, `st:6134-6160` | none |
| K1.62 | menu · weak-focus variant | `MenuItemDrawStrongFocus` = false: a **focus line** under the text (hover colour, pressed focus colour) instead of the fill | read breezerc `[Style] MenuItemDrawStrongFocus` (`kcfg:130-132`); pub `st:6177-6185`, `hp:407-422` | none |
| K1.63 | menu · highlight inset | The highlight is inset from the popup edge by `PM_MenuHMargin`/`VMargin` = `MenuItem_HighlightGap` | pub `st:649-651`; `mt:55` | none (`menu.border.padding_*` is the item's content padding) |
| K1.64 | menu · popup frame | Popup **corner radius** `Frame_FrameRadius` when compositing, square without an alpha channel; **seamless edges** where the menu meets its menu bar/combo | pub `hp:546-611`, `hp:1943-1955`; `st:4700-4732` | partial: `menu.border.corner_radius` = none, `popover.border` |
| K1.65 | menu · translucency | **Menu opacity** 0–100 %: background alpha, outline `WindowText` α 0.25, separators `WindowText` α frameContrast, KWin **blur behind** | read breezerc `[Style] MenuOpacity` (`kcfg:181-183`); pub `st:4718-4724`, `st:5986-5990`, `st:420-421`; `kstyle/breezeblurhelper.cpp:78-95` | none |
| K1.66 | menu · section header | **Section** item (`QMenu::addSection`): bold menu font, `WindowText` α 0.7, separator continuing after the text | pub `st:5994-6016`; size `st:3686` | none |
| K1.67 | menu · shortcut text | **Accelerator** column: right-aligned, opacity 0.7, `MenuItem_AcceleratorSpace` gap | pub `st:6162-6172`; `mt:59` | none |
| K1.68 | menu · submenu arrow | Submenu arrow: `ArrowSize` in a `MenuButton_IndicatorWidth` box, `arrowColor(WindowText)` | pub `st:6125-6140`; `mt:37`, `mt:84` | none |
| K1.69 | menu · check/radio items | Checkable items draw Breeze's checkbox/radio (`CheckBox_Size`) in a leading column, `MenuItem_TextLeftMargin` | pub `st:6064-6085`; `mt:60` | none |
| K1.70 | menu · icon column | Icon column width (max icon width), `MenuItem_ItemSpacing`, `MenuItem_ExtraLeftMargin` when icons are hidden; global **show icons in menus** setting | read kdeglobals `[KDE] ShowIconsInMenuItems` (`khint:405-411`); pub `st:6087-6110`; `mt:56-58` | partial: `menu.icon_size`, `icon_text_gap` |
| K1.71 | menu · separator geometry | Separator inset (`MenuItem_MarginWidth` each side) and item height | pub `st:5982-5992`; size `st:3607-3718` | partial: `separator_color` only |
| K1.72 | menu · scroll arrows | Scrollable-menu arrows (`SH_Menu_Scrollable` true) | pub `st:927-928` (Qt draws the scroller; measure) | none |
| K1.73 | menu · timing | **Submenu open delay** `SH_Menu_SubMenuPopupDelay` | pub `st:931-932` | none |
| K1.74 | menu · shadow / motion | Popup shadow (K3.2) and fade (K3.34) | — | partial: `border.shadow_enabled` |

### Tooltip

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.80 | tooltip · timings | **Fall-asleep delay** (re-show without delay when moving between items) and **display duration** | pub qtbase `qcommonstyle.cpp` `SH_ToolTip_FallAsleepDelay`, `qtooltip.cpp` hide timer (to verify lines; Breeze does not override them, `st:885-981`) | partial: `show_delay_ms` only |
| K1.81 | tooltip · shadow / fade | Breeze registers tooltip windows with its shadow helper; KWin fades them | pub `st:4734-4749`; see K3.2, K3.34 | partial: `border.shadow_enabled` |
| K1.82 | tooltip · square variant | Without compositing the frame is square (`renderMenuFrame(hasAlpha=false)`) | pub `st:4744-4748`, `hp:592-608` | none |
| K1.83† | tooltip · Qt timings and wrapping | The values K1.80 left to verify: wake-up delay 700 ms, fall-asleep delay 2000 ms (Breeze overrides neither); **display time** 10 000 ms + 40 ms per character beyond 100; hidden 300 ms after the pointer leaves; plain text **wraps only when wider than the screen**, rich text word-wraps, no other maximum width (Quick: gridUnit × 14, Q3.29) | pub qtbase v6.10.1 `qcommonstyle.cpp:5367-5372`; `qtooltip.cpp:175-178`, `:206-210`, `:258` | partial: `tooltip.show_delay_ms`; `max_width` (facts: none) |
| K1.84† | tooltip · rich tooltip widget | `KToolTipWidget` (Dolphin's file tooltips): any widget as content in a `PE_PanelTipLabel` frame on a translucent window, placed centred below the item (else above, else beside it) with a `PM_ToolTipLabelFrameWidth` gap, **hide delay 500 ms** | pub `kwa ktooltipwidget.cpp:48-54`, `:106-150`, `:231-241`; `st:734` | none |

### Scrollbar

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.90 | scrollbar · thumb border | Thumb **outline**: stroke `transparentize(fg, frameContrast)`, fill overlay(bg, fg α 0.5); pill radius = half of min(width, `ScrollBar_SliderWidth`) | pub `hp:1274-1286` | none (`thumb_color` only; no thumb radius) |
| K1.91 | scrollbar · groove-hover state | Thumb alpha 0.7 at rest, rising to full while the pointer is **over the scrollbar** (not the thumb) | pub `st:6366-6372`, `st:6405-6408` | none |
| K1.92 | scrollbar · focused view | Thumb turns **`focusColor`** while the scrolled view has focus | pub `hp:277-303`; `st:6392-6397` | none |
| K1.93 | scrollbar · separator | A 1 px **line on the content side** (`Text` α frameContrast) | pub `st:8218-8237` | none |
| K1.94 | scrollbar · arrow buttons | Optional **arrow buttons** (none / single / double at either end), `ScrollBar_SingleButtonHeight`, arrow colours per state | read breezerc `[Style] ScrollBarAddLineButtons`, `ScrollBarSubLineButtons` (`kcfg:73-79`); pub `st:2178-2207`, `st:6415-6572`, `st:8465-8527`; `mt:100-102` | none |
| K1.95 | scrollbar · motion | Hover/focus and groove-opacity fades (scrollbar engine) | pub `st:6360-6408`; `anim` | none |
| K1.96 | scrollbar · corner | The corner square between two scrollbars (viewport background) | pub `st:4681-4698` | none |

### Slider

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.100 | slider · thumb border | Handle **outline per state**: rest mix(Button, ButtonText, frameContrast), hover `hoverColor`, focus `focusColor`, neutral `NeutralText.lighter(150)` | pub `hp:248-275`; `st:8124-8137` | none (fills only) |
| K1.101 | slider · thumb shadow | Handle **shadow** (ellipse, black α 0.125, 1 px, rotated 45°), removed while pressed | pub `hp:1169-1202`, `hp:1663-1690`; `st:8134` | none |
| K1.102 | slider · pressed | **Pressed** handle state (no shadow) | pub `hp:1179-1181` | none |
| K1.103 | slider · ticks | Tick **colours** (`separatorColor` after the value, `Highlight` up to it), tick **margin** `Slider_TickMarginWidth`, ticks on/off option | read breezerc `[Style] SliderDrawTickMarks` (`kcfg:102-104`); pub `st:7991-8068`; `mt:123-124` | partial: `tick_mark_length` |
| K1.104 | slider · groove radius | Groove corner radius (half the groove thickness) | pub `hp:1081-1098` | none |
| K1.105 | slider · neutral variant | Neutral highlight: value fill and ticks `NeutralText` | pub `st:8048-8050`, `st:8082-8084` | none |
| K1.106 | slider · focus / motion | Circular focus frame around the handle (K3.20); hover/focus fades | pub `hp:1155-1167`; `st:8118-8125` | partial: `defaults.focus_ring_*` |

### Progress bar

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.110 | progress · busy state | **Indeterminate** bar: stripes of `ProgressBar_BusyIndicatorSize` alternating `Highlight` and mix(Highlight, Window, 0.7), moved by a step animation of `ProgressBarBusyStepDuration`; on/off | read breezerc `[Style] ProgressBarAnimated`, `ProgressBarBusyStepDuration` (`kcfg:63-70`); pub `st:6196-6270`, `hp:1222-1270`; `animations/breezebusyindicatorengine.cpp:66-100` | none |
| K1.111 | progress · fill border | The value fill's own **outline** `transparentize(fg, frameContrast)` over a fill of overlay(bg, fg α 0.7) | pub `hp:1204-1218` | partial: `progress_bar.border` is the groove's |
| K1.112 | progress · selected | Inside a **selected** item the fill is `HighlightedText` | pub `st:6294-6296` | none |
| K1.113 | progress · label | **Text label** (percentage): colour `Text`/`HighlightedText`, `ProgressBar_ItemSpacing`, alignment | pub `st:6313-6343`, `st:2386-2417`; `mt:117` | none |
| K1.114 | progress · minimum fill | A non-zero value always shows at least `ProgressBar_Thickness` of fill | pub `st:6249-6266` | none |
| K1.115† | progress · capacity bar | `KCapacityBar` (free-space bars in Dolphin and file dialogs): Breeze's progress bar at a fixed **bar height 12**, its text **below** the bar (`DrawTextOutline`, the default) or inside it (`DrawTextInline`, round margin 6); the Places panel draws its own capacity bars (K1.142†) | pub `kwa kcapacitybar.cpp:19-20`, `:34`, `:42`, `:155-198`, `:202-205` | partial: `progress_bar.*` (no thickness variant, no text-below layout) |

### Tab bar (QTabBar / QTabWidget)

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.120 | tab · vertical tabs | **West/East** tab bars: the indicator strip on the left/right edge | pub `hp:1470-1490` | partial: `active_indicator_side` (Top/Bottom only) |
| K1.121 | tab · unselected radius | **Unselected** tabs are filled with rounded corners (`Frame_FrameRadius`) on the bar side | pub `hp:1505-1507` | partial: `tab.border.corner_radius` |
| K1.122 | tab · pane vs document mode | Selected tab fill: `frameBackgroundColor` (attached to a pane) vs `Window` (document mode) | pub `hp:1459-1463` | partial: `active_background` (one value) |
| K1.123 | tab · static variant | **Static tab bar** (document mode, expanding, not closable, not movable): min height `TabBar_StaticTabMinHeight`, unselected `Base.darker(102)`, hover `hoverColor` α frameContrast, selected `Highlight` α frameContrast + strip, 8 px-inset separators between unselected tabs, no overlap | pub `hp:1299-1430`; `st:7227`, `st:3820`; `mt:133` | none |
| K1.124 | tab · close button | **Close button** glyph: rest mix(Window, text, 0.5); hover filled circle mix(Window, NegativeText, 0.5) with inverted glyph; pressed 0.7; disabled 0.2; size `PM_SmallIconSize` | pub `st:5058-5100`, `st:8597-8670`; `hp:1581-1645` | none |
| K1.125 | tab · scroll buttons | **Overflow scroll buttons** on a filled panel | pub `st:4626-4679` | none |
| K1.126 | tab · tear indicator | Drag **tear** line (`WindowText` α 0.2) | pub `st:5103-5160` | none |
| K1.127 | tab · base line | The **tab-bar base** line under the row | pub `st:4332-4380` | partial: `tab.border` |
| K1.128 | tab · pane | The **tab-widget pane**: rounded frame filled `frameBackgroundColor`, corners cut where the bar touches; `TabWidget_MarginWidth` | pub `st:4229-4330`; `hp:784-812`; `mt:140` | none |
| K1.129 | tab · focus | Keyboard focus = **focus line** under the selected tab's label, faded in | pub `st:7021-7178` (`:7108-7170`) | none |
| K1.130 | tab · icon & spacing | Tab **icon size** and icon-text gap `TabBar_TabItemSpacing` | pub `st:7021-7100`; `mt:134` | none |
| K1.131 | tab · alignment | **Centered tabs** option | read breezerc `[Style] TabBarDrawCenteredTabs` (`kcfg:118-120`); pub `st:951-952` | none |
| K1.132 | tab · disabled / motion | Disabled tab text; hover fade (tab-bar engine) | pub `hp:1494-1503`; `st:7180-7200` | none |
| K1.133† | tab · move and drag-hover | **Movable tabs** slide to their new place in 250 ms `InOutQuad` (less for short moves), hard-coded in Qt; dragging something over a tab bar **switches to the tab after 500 ms** (`SH_TabBar_ChangeCurrentDelay`; Breeze does not override it) | pub qtbase v6.10.1 `qtabbar_p.h:30`, `:145`; `qtabbar.cpp:2122`, `:2339-2340`; `qcommonstyle.cpp:5345-5346` | none |

### Sidebar (Qt Widgets side panels; Kirigami GlobalDrawer in the Quick survey)

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.140 | sidebar · edge line | **Side-panel frame** (`_kde_side_panel_view`): only the inner edge, `Inactive Highlight`, `Active Highlight` when the view has focus; `SidePanelDrawFrame` option draws the full frame | read breezerc `[Style] SidePanelDrawFrame` (`kcfg:126-128`); pub `st:3958-3963`, `hp:184-198`, `hp:504-544` | partial: `sidebar.border` (all sides, no focus colour) |
| K1.141 | sidebar · item margin | `SidePanel_ItemMarginWidth` around side-panel items | pub `mt:166` (use site to verify) | none |
| K1.142† | sidebar · places panel | KIO's **Places panel** (file dialogs, Dolphin): **section headers** in mix(WindowText, Base, 60) of the active/inactive group; the delegate's icon size (initially 48, the view sets it, to verify); a per-device **capacity bar** under the name, height iconSize ÷ 8, fully rounded, track `WindowText` α 0.2, fill `Highlight` (`HighlightedText` on a selected row of the active window), `[Colors:View] ForegroundNegative` above 95 % used; a trailing **eject** action icon; hidden places at opacity 0.6; sections appearing and disappearing by an opacity animation | pub KDE/kio @ `eec01328` `src/filewidgets/kfileplacesview.cpp:64`, `:70-74`, `:118-138`, `:164-210`, `:239-285`, `:395-441`, `:690-709`, `:728-736` | partial: `sidebar.*` (no section header, capacity bar or action icon) |

### Toolbar (QToolBar, tool buttons, the tools area)

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.150 | toolbar · inactive window | Tools-area (menu bar + top toolbars) fill from the **inactive** Header group (`[Colors:Header][Inactive]`) | read kdeglobals `[Colors:Header][Inactive]` (`colors` BreezeLight `:71`); pub `st:1346`, `tam:199-232` | none |
| K1.151 | toolbar · bottom edge | Only the **bottom** separator line of the tools area; dialogs with a tool bar get the same, others a top line | pub `st:1324-1398` | partial: `toolbar.border` (all sides) |
| K1.152 | toolbar · tool button states | **Auto-raise tool button**: transparent at rest, hover pen `Highlight`, pressed `Highlight` α `Blend_Value`, checked `ButtonText` α 0.125 + pen, keyboard-focus pen | pub `st:4577-4624`, `hp:678-695` | none |
| K1.153 | toolbar · button style | **Tool button style** (icon only / text beside / text under / text only), separately for main and other toolbars | read kdeglobals `[Toolbar style] ToolButtonStyle`, `ToolButtonStyleOtherToolbars` (`khint:85-86`, `:414-424`; `ktb:598-630`) | none |
| K1.154 | toolbar · tool button geometry | `ToolButton_MarginWidth`, `ToolButton_ItemSpacing` (icon–text, beside or under) | pub `mt:79-80`; `st:5526-5685`, `st:3573-3600` | none |
| K1.155 | toolbar · menu arrows | Tool button **menu indicator** styles: split sub-control with a separator line, inline small arrow, inline large arrow (`ToolButton_InlineIndicatorWidth`); arrow colours by state | pub `st:213-250`, `st:4992-5056`, `st:7719-7816`, `st:4431-4470`; `mt:81` | none |
| K1.156 | toolbar · separators & handle | Toolbar **separator** (`ToolBar_SeparatorWidth`, `separatorColor`, on/off option) and **handle** (two lines, `ToolBar_HandleWidth`, `ToolBar_HandleExtent`) | read breezerc `[Style] ToolBarDrawItemSeparator` (`kcfg:92-94`); pub `st:5162-5227`; `mt:105-112` | none |
| K1.157 | toolbar · extension button | **Overflow** (extension) button icon | pub `st:2133-2135`, `st:8528-8596`; `mt:109` | none |
| K1.158 | toolbar · icon sizes | **Main** toolbar vs **other** toolbars icon sizes (`MainToolbar` and `Toolbar` KIconLoader groups) | read the icon theme's `index.theme` `MainToolbarDefault`, `ToolbarDefault` (`kit:465-473`); `khint:88` | partial: `toolbar.icon_size`, `icon_sizes.toolbar` (one value) |

### Status bar

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.160 | status bar · edge | Only a **top** separator line, and only for a QMainWindow's status bar | pub `st:5312-5325` | partial: `status_bar.border` (all sides) |

### List / table / tree (QAbstractItemView)

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.170 | list · selection shape | Selection **corner radius** and **inset** (item margins: left/right, top/bottom, larger first-row top), rounding only on a row's first and last cell; tables square | pub `st:4751-4900`; `hp:1692-1731`, `hp:1759-1800`; `mt:157-165` | none |
| K1.171 | list · selection outline | Selection/hover **outline**: the fill colour at α 1.0 (selected) or 0.8 (hover) | pub `st:4886-4893` | none |
| K1.172 | list · selected+hover | **Selected + hover** = `Highlight.lighter(110)` | pub `st:4878-4882`; `mt:178` | none |
| K1.173 | list · inactive window | Selection in an **inactive window** (Inactive group: Window tinted with Selection, Window text) | pub `st:4835-4842`; `kcs:303-321`, `kcs:386-389` | partial: `selection_inactive_background` (no text colour; facts mark it "none", D3) |
| K1.174 | list · disabled selection | Selection in a **disabled** view (Disabled group) | pub `st:4835-4842`; `kcs:316-319` | none |
| K1.175 | list · current-item focus | **Focus rectangle** on the current, unselected item: rounded outline `Highlight.lighter(110)`; on/off option | read breezerc `[Style] ViewDrawFocusIndicator` (`kcfg:97-99`); pub `st:4092-4170`, `st:2209` | none |
| K1.176 | list · tree branches | **Expander arrows** (`ItemView_ArrowSize`, `arrowColor(Text)`, `hoverColor` on hover), **branch lines** mix(Base, Text, 0.25) with on/off option, per-level **indentation** | read breezerc `[Style] ViewDrawTreeBranchLines` (`kcfg:107-109`); pub `st:5229-5299`; indentation qtbase `qtreeview.cpp` (to verify) | none (facts call it toolkit-internal) |
| K1.177 | list · header states | Header section **hover** mix(Button, hoverColor, 0.2), **pressed/sorted** mix(Button, focusColor, 0.2), hover fade | pub `st:6808-6845` | partial: `header_background` |
| K1.178 | list · header lines | Header **separators**: bottom line and lines between sections, `WindowText` α frameContrast; corner button | pub `st:6846-6900` | none |
| K1.179 | list · sort indicator | **Sort arrow** (`Header_ArrowSize`, `arrowColor(ButtonText)`), inverted direction option; `Header_MarginWidth`, `Header_ItemSpacing` | read breezerc `[Style] ViewInvertSortIndicator` (`kcfg:113-115`); pub `st:4481-4514`, `st:2418-2456`; `mt:151-153` | partial: `header_height` (unset), `header_font` |
| K1.180 | list · icon view | **Icon mode** (QListView IconMode, Dolphin): icon above text, `PM_IconViewIconSize`, same selection shape | pub `st:4751-4900`; measure: probe app | none |
| K1.181 | list · item icon size | Item **icon size** in lists/trees | measure: probe app (`PM_SmallIconSize`/`PM_ListViewIconSize`, Breeze defers to Qt `st:653-656`) | none |
| K1.182 | list · drop indicator | **Drag-and-drop** drop indicator (not overridden by Breeze, so Qt's) | measure: probe app; pub qtbase `qcommonstyle.cpp` `PE_IndicatorItemViewItemDrop` (to verify) | none |
| K1.183 | list · sort column shade | `shadeSortColumn` colour-scheme flag | read kdeglobals `[General] shadeSortColumn` (`colors` BreezeLight `:197`); its reader is not in these repos (to verify) | none |
| K1.184† | list · column view | `QColumnView` (Miller columns, a column browser): every column a list; an item with children ends in a **triangle** (`PE_IndicatorColumnViewArrow`) that Breeze leaves to QCommonStyle: filled `Mid` with a `Dark` edge, `HighlightedText` (or `WindowText`) when selected — shade roles (K3.53); Kirigami's column layout is Q1.93–Q1.94 | pub qtbase v6.10.1 `qcommonstyle.cpp:610-659`; Breeze has no `PE_IndicatorColumnViewArrow` case (`st`, by search) | none |
| K1.185† | list · tree expand animation | QTreeView's **animated expand/collapse** (when the app turns it on) is enabled whenever the style's `SH_Widget_Animation_Duration` is above 0, which Breeze sets to `AnimationsDuration` (0 with animations off); the slide eases `InOutQuad`, its duration is QVariantAnimation's default (Qt sets none; to verify) | pub qtbase v6.10.1 `qtreeview.cpp:3131`, `qtreeview_p.h:81-87`; `st:942-943` | none |

### Popover / dropdown

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.190 | popover · shadow | Popups get Breeze's two-layer shadow (K3.2) | pub `shd:33-60` | partial: `border.shadow_enabled` |
| K1.191 | popover · square variant | Square frame without compositing | pub `hp:546-611` | none |

### Splitter, separator

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.200 | splitter · dock separators | Dock-widget resize handles drawn as `separatorColor` fills | pub `st:5301-5310` | partial: `splitter.divider_color` (data) |
| K1.201 | separator · vertical/horizontal | `KMultiTabBar` side separators and Breeze's generic separator (`renderSeparator`, half-pixel centred) | pub `st:1400-1422`; `hp:814-829` | partial: `separator.*` |

### Layout

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.210 | layout · forms | **Form layout** conventions: labels right-aligned, form left/top-aligned, fields grow, no row wrapping | pub `st:955-962` | none |
| K1.211 | layout · group box title | Group box title margin and the space a title takes (see K2.8) | pub `st:2811-2925`; `mt:71` | none |

### Dialog

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.220 | dialog · button box | `QDialogButtonBox::KdeLayout`: the full role order (Help/Reset left, stretch, then the rest) is more than PrimaryLeft/Right | pub `khint:115`; qtbase `qdialogbuttonbox.cpp` layout table (to verify lines) | partial: `button_order` |
| K1.221 | dialog · button icons | Dialog buttons show **icons** (setting) | read kdeglobals `[KDE] ShowIconsOnPushButtons` (`khint:116`); pub `st:944-945` | none |
| K1.222 | dialog · header line | A dialog with a menu/tool bar gets a Header-filled tools area; others a **top separator line** | pub `st:1359-1398` | none |
| K1.223 | dialog · window entrance | A Qt dialog is a window: its entrance is KWin's scale/fade (K3.33), which `entrance` cannot describe (no duration, scale, curve) | pub `kwin src/plugins/scale/package/contents/code/main.js:35-39`, `:105-155` | partial: `dialog.entrance` |
| K1.224† | dialog · message box | `KMessageBox` / `KMessageDialog`, and `QMessageBox` routed through the KDE platform theme's message-dialog helper: a **standard icon** (`SP_MessageBoxQuestion` / `Warning` / `Information` / `Critical`, from the icon theme) at `PM_MessageBoxIconSize`, Qt's default 32 DPI-scaled (Breeze does not override it); text, optional details and a "don't ask again" check box | pub `kwa kmessagebox.cpp:138-147`, `:208`; `kwa kmessagedialog.cpp:247-258`, `:276`; qtbase v6.10.1 `qcommonstyle.cpp:4818-4826`; plasma-integration @ `46eddde9` `qt6/src/platformtheme/kdeplatformtheme.cpp:273-276`, `:335-336` | partial: `dialog.icon_size` (facts: none) |

### Spinner (busy indicator)

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.230 | spinner · rotation | `KBusyIndicatorWidget`: `process-working` icon rotated, period 2000 ms | pub `kwa kbusyindicatorwidget.cpp:23`, `:103` | partial: icon-level `TransformAnimation::Spin`; no `spinner` period |

### Combo box

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.240 | combo · editable variant | **Editable** combo: line-edit frame (`Base`, input hover/focus outline), arrow `arrowColor(WindowText)` → `hoverColor` over the arrow, animated | pub `st:7818-7910` (frame `:7841-7856`, arrow `:7889-7910`) | none |
| K1.241 | combo · flat variant | **Frameless** combo | pub `st:7843-7850`, `st:7954-7960` | none |
| K1.242 | combo · open/pressed | **Pressed/open** state (button frame down fill mix(Button, Highlight, `Blend_Value`)) | pub `st:7856-7871`; `hp:693-705` | none |
| K1.243 | combo · border per state | Pen `Highlight` on hover/focus/pressed | pub `hp:710-712` | partial: `border.color` |
| K1.244 | combo · shadow | A non-flat, non-editable combo is a button frame and draws the **button shadow** (K1.23) — facts say "no shadow" (D5) | pub `st:7856-7871`; `hp:729-732` | partial: `border.shadow_enabled` |
| K1.245 | combo · popup | The **popup list**: menu-frame container, item rows (item-view or menu delegate), shadow, seamless edge, selected item | pub `st:379`, `st:430-440`, `st:1716`, `st:1792`, `st:4100` | none |
| K1.246 | combo · arrow colour | Arrow colour per state (disabled, hover, focus) | pub `st:7880-7910` | none |
| K1.247 | combo · neutral variant | Neutral highlight | pub `st:7831` | none |

### Expander (KCollapsibleGroupBox)

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.250 | expander · arrow hover | Arrow turns `hoverColor` on hover | pub `st:4424-4427` | partial: `arrow_color` |
| K1.251 | expander · motion | Expand/collapse **height animation**, 500 ms `QTimeLine` (Qt's default curve) | pub `kwa kcollapsiblegroupbox.cpp:48-58` | none |

### Link

Checked, no gap: QLabel rich-text links use the palette's `Link`/`LinkVisited` with no hover change (measure to confirm); the Quick `LinkButton`/`UrlButton` gaps are Q1.154–Q1.156.

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.260† | link · KUrlLabel | `KUrlLabel` (clickable URL labels in KF dialogs): text in `Link`, underlined; on hover ("glow", on by default) the text turns **`BrightText`**, a hover colour no other link uses; an optional "float" mode underlines only on hover; an optional alternate pixmap on hover; pointing-hand cursor | pub `kwa kurllabel.cpp:21-29`, `:78`, `:255-290` | partial: `link.hover_text_color` (one value) |

### Text scale

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| K1.270 | text_scale · heading levels | `KTitleWidget` **levels 1–4** (factors of the app font: 1.35, 1.20, 1.15, 1.10) and Kirigami `Heading` levels 1–5 (Q1.3) | pub `kwa ktitlewidget.cpp:30-55` | partial: `section_heading`, `dialog_title` |
| K1.271 | text_scale · family | `smallestReadableFont` is a **full font** (family, style), not a size | read kdeglobals `[General] smallestReadableFont` (`kfont:53`) | partial: `TextScaleEntry` has no family/style |
| K1.272 | text_scale · derived fonts | Derived fonts Breeze makes: flat group box title (bold, +1 pt), command-link title (bold), menu section (bold) | pub `st:2867-2868`, `st:7604-7608`, `st:2039-2041`, `st:5999-6001` | none |

### Qt Quick: Kirigami and qqc2-desktop-style

Rows from the Quick survey (`kirigami@81ada72b`, `qqc2-desktop-style@305eb2c4`, full paths cited). Where qqc2 hands a part to Breeze through `StyleItem` (`elementType`), the Breeze rows above hold the gap; the Quick row only records what QML adds.

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| Q1.1 | defaults (icon sizes) | Kirigami's six named icon steps small 16 / smallMedium 22 / medium 32 / large 48 / huge 64 / enormous 128, plus `sizeForLabels` (font height rounded down to 16/22/32/48); the model has five role-named contexts, no smallMedium/medium/huge/enormous/label-sized step | `kirigami@81ada72b src/platform/units.cpp:297-356` | `defaults.icon_sizes.{toolbar,small,large,dialog,panel}_px` (partial) |
| Q1.2 | defaults (fonts) | `Theme.smallFont` (QFontDatabase::SmallestReadableFont, own family + size) used by Badge, subtitles | `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:151`, `:199` | `text_scale.caption` (size/weight only, no family) |
| Q1.3 | text_scale (Heading) | five Heading levels as factors of the default font size: L1 ×1.35, L2 ×1.20, L3 ×1.15, L4 ×1.10, L5 ×1.0; the model has four named roles, no level ramp | `kirigami@81ada72b src/controls/Heading.qml:14-33` | `text_scale.{caption,section_heading,dialog_title,display}` (partial) |
| Q1.4 | text_scale (Heading) | Heading `Type`: Primary = DemiBold weight, Secondary = opacity 0.75, Normal | `kirigami@81ada72b src/controls/Heading.qml:35-37`; `src/templates/Heading.qml:66-70` | `text_scale_entry.weight` (weight only; no secondary-opacity role) |
| Q1.5 | text_scale (Heading use) | which level each surface uses: dialog title L1 (default), page title L1, card banner L1, OverlaySheet header L2, InlineViewHeader L2, FormLayout section L3 / field label L5, GlobalDrawer section L4, ListSectionHeader L5 Bold | `kirigami@81ada72b src/templates/Heading.qml:59`; `src/controls/private/BannerImage.qml:202`; `src/templates/OverlaySheet.qml:74-75`; `src/controls/InlineViewHeader.qml:73`; `src/layouts/FormLayout.qml:427-429`; `src/controls/GlobalDrawer.qml:481-482`; `src/controls/ListSectionHeader.qml:93-99` | `text_scale.*`, `dialog.title_font` (partial) |
| Q1.6 | window | default window size gridUnit×55 by gridUnit×40 (mobile 30×45) | `kirigami@81ada72b src/controls/AbstractApplicationWindow.qml:279-280` | none |
| Q1.7 | window | `wideScreen` breakpoint width ≥ gridUnit×60 (changes layout of drawers/columns) | `kirigami@81ada72b src/controls/AbstractApplicationWindow.qml:114` | none |
| Q1.8 | button (qqc2 Button) | painted by Breeze, `elementType: "button"`; QML passes `flat`, `on` (checkable&&checked), `hasFocus = activeFocus \|\| highlighted`, `activeControl "default"` for the default button, `menu` arrow flag, icon colour = textColor | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Button.qml:47-66` | `button.*` (Breeze side out of scope; flat/default/menu-arrow variants have no fields) |
| Q1.9 | button (qqc2 ToolButton) | painted by Breeze, `elementType: "toolbutton"` (`CC_ToolButton`); colour set switches Window when flat / Button when raised; `hasFocus = visualFocus \|\| (!flat && pressed) \|\| highlighted`; toolButtonStyle (icon-only / text-beside / text-under) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ToolButton.qml:18-19`, `:53-94`; `plugin/kquickstyleitem.cpp:1772-1773` | `toolbar.*`, `button.*` (no flat tool-button colour set) |
| Q1.10 | button (labels) | mnemonic underline appears only while Alt is held | `kirigami@81ada72b src/primitives/mnemonicattached.cpp:185-212`, `:349` | none |
| Q1.11 | input (TextField/TextArea/SearchField) | frame painted by Breeze, `elementType: "edit"` (`PE_PanelLineEdit`) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/TextField.qml:137-144`; `TextArea.qml:156-165`; `SearchField.qml:117-124` | `input.border` (Breeze side out of scope) |
| Q1.12 | input | default implicit width 200 (TextField); TextArea background implicit 200×22 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/TextField.qml:32`; `TextArea.qml:159-160` | `input.min_height_px` (no width) |
| Q1.13 | input (TextArea) | spell-check misspelling colour = negativeTextColor | `qqc2-desktop-style@305eb2c4 org.kde.desktop/TextArea.qml:94` | `defaults.danger_color` (no misspelling role) |
| Q1.14 | input (touch) | tablet-mode text cursor and selection handles: 3 px bar and round handle of gridUnit/1.5, colour tint(highlight, white 0.4) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/private/MobileCursor.qml:16-43` | `input.caret_color` (desktop caret only) |
| Q1.15 | input (qqc2 SearchField) | leading search icon: size sizeForLabels, colour disabledTextColor, left margin smallSpacing×2, text inset smallSpacing×3; trailing clear ToolButton (sizeForLabels icon) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/SearchField.qml:36-41`, `:60-96` | none |
| Q1.16 | input (qqc2 SearchField popup) | suggestion popup: View set, radius cornerRadius, 1 px border mix(bg,text,frameContrast), shadow x 0 / y 2 / size 8 / black 0.3, margins mediumSpacing, 1 px gap below field, rows padded mediumSpacing | `qqc2-desktop-style@305eb2c4 org.kde.desktop/SearchField.qml:45-58`, `:126-190` | `popover.*` (no shadow geometry) |
| Q1.17 | input (Kirigami SearchField) | search icon colour = placeholderTextColor, size sizeForLabels; search fires after humanMoment (delaySearch) else shortDuration | `kirigami@81ada72b src/controls/SearchField.qml:76-84`, `:114-115` | `input.placeholder_color` (icon role and debounce timing missing) |
| Q1.18 | input (ActionTextField) | inline action ToolButtons (icon sizeForLabels, spacing smallSpacing, 1 px inset); left/right text padding animates longDuration InOutQuad when actions appear; focus-shortcut tooltip | `kirigami@81ada72b src/controls/ActionTextField.qml:104-121`, `:132-159`, `:160-201` | none |
| Q1.19 | input (PasswordField) | reveal toggle icons `password-show-on` / `password-show-off` | `kirigami@81ada72b src/controls/PasswordField.qml:48` | none |
| Q1.20 | checkbox (CheckBox/RadioButton) | indicator painted by Breeze, `elementType: "checkbox"` / `"radiobutton"`; label spacing from pixelMetric `checkboxlabelspacing` / `radiobuttonlabelspacing`; `partiallyChecked` state passed | `qqc2-desktop-style@305eb2c4 org.kde.desktop/private/CheckIndicator.qml:15-42`; `CheckBox.qml:26`; `RadioButton.qml:26` | `checkbox.*` (no indeterminate field) |
| Q1.21 | checkbox | keyboard focus is drawn as a focus rect around the label text (not the box): `elementType: "focusrect"`, x = label start − smallSpacing/2, width = paintedWidth + smallSpacing, top/bottom = label padding − 1 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/CheckBox.qml:92-106`; `RadioButton.qml:93-107` | `defaults.focus_ring_*` (no placement) |
| Q1.22 | checkbox (label) | disabled label opacity 0.6 (applied to text only) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/CheckBox.qml:81`; `RadioButton.qml:85` | `checkbox.disabled_text_color` / `disabled_opacity` (partial) |
| Q1.23 | menu | menu panel shadow: x 0, y 2, size 8, black alpha 0.3 (single layer) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Menu.qml:136-139` | `menu.border.shadow_enabled`, `defaults.shadow_color` |
| Q1.24 | menu | open/close: opacity fade, shortDuration, InOutQuad | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Menu.qml:108-126` | none |
| Q1.25 | menu | panel paddings come from Breeze pixelMetric `menuhmargin` / `menuvmargin`; minimum width gridUnit×8 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Menu.qml:29-30`, `:130` | `menu.border.padding_*` (min width missing) |
| Q1.26 | menu (MenuItem) | highlighted item: fill focusColor at alpha 0.3, 1 px border focusColor, radius cornerRadius (item has its own border + radius) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/MenuItem.qml:121-128` | `menu.hover_background` (no hover border, no item radius) |
| Q1.27 | menu (MenuItem) | pressed item: solid focusColor fill, text highlightedTextColor, icon `selected` | `qqc2-desktop-style@305eb2c4 org.kde.desktop/MenuItem.qml:63`, `:74`, `:124` | none (no pressed fields) |
| Q1.28 | menu (MenuItem) | item padding smallSpacing horizontal, vertical 4 (8 with transient touch input) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/MenuItem.qml:33-34` | `menu.row_height_px` (padding / touch variant missing) |
| Q1.29 | menu (MenuItem) | icon size small (smallMedium with touch input); reserved check-indicator column when any item is checkable; reserved icon column when any item has an icon | `qqc2-desktop-style@305eb2c4 org.kde.desktop/MenuItem.qml:54-66`; `Menu.qml:87-106` | `menu.icon_size_px` (touch variant, column reservation missing) |
| Q1.30 | menu (MenuItem) | shortcut text right-aligned in the label's colour; submenu arrow icon `go-next-symbolic` at size small | `qqc2-desktop-style@305eb2c4 org.kde.desktop/MenuItem.qml:80-95`, `:101-108` | none |
| Q1.31 | menu (MenuItem check) | checkable items draw Breeze `checkbox`/`radiobutton` indicator (exclusive group → radio) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/MenuItem.qml:110-119`; `private/CheckIndicator.qml:25-28` | `checkbox.*` |
| Q1.32 | menu (MenuSeparator) | vertical padding round(smallSpacing/2), line = Kirigami.Separator (width gridUnit×8) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/MenuSeparator.qml:26-33` | `menu.separator_color` (padding missing) |
| Q1.33 | tooltip | shadow x 0, y 4, size 16, black alpha 0.2 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ToolTip.qml:107-110` | `tooltip.border.shadow_enabled`, `defaults.shadow_color` |
| Q1.34 | tooltip | fade in/out longDuration OutCubic | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ToolTip.qml:50-68` | none |
| Q1.35 | tooltip | placement: horizontally centred on the parent, 3 px above it; margins 6, padding 6 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ToolTip.qml:20-21`, `:32-33` | `tooltip.border.padding_*` (placement/gap missing) |
| Q1.36 | tooltip | never times out while hovered (timeout -1); in tablet mode shown on press after mousePressAndHoldInterval | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ToolTip.qml:35-46` | `tooltip.show_delay_ms` (touch delay / timeout missing) |
| Q1.37 | tooltip | colour set switches to Complementary when the parent is Complementary (else Tooltip set) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ToolTip.qml:16-18` | `tooltip.background_color` (single variant) |
| Q1.38 | tooltip (border) | 1 px border = mix(tooltip bg, text, frameContrast) (derived, not stored) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ToolTip.qml:112-115` | `tooltip.border.color` |
| Q1.39 | scrollbar | interactive bar painted by Breeze, `elementType: "scrollbar"` (`CC_ScrollBar`); groove insets from `subControlRect("groove")`; page-click behaviour from styleHint `scrollToClickPosition` | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ScrollBar.qml:181-241`, `:210-220` | `scrollbar.*` |
| Q1.40 | scrollbar | hover cross-fade between the inactive and the hovered Breeze rendering, longDuration InOutQuad | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ScrollBar.qml:243-293` | none |
| Q1.41 | scrollbar (touch overlay) | non-interactive overlay thumb when transient touch input: colour textColor at opacity 0.3, thickness max(2, smallSpacing × state), fully rounded, slides in from the edge, hides after 1000 ms, hide animation longDuration InOutQuad | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ScrollBar.qml:24`, `:37-91` | `scrollbar.overlay_mode` (bool only; geometry, colour, timing missing) |
| Q1.42 | scrollbar | arrow/page auto-repeat interval 150 ms | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ScrollBar.qml:165-180` | none |
| Q1.43 | scrollbar (ScrollView) | framed background painted by Breeze `elementType: "frame"` only when `StyleHints.showFramedBackground` | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ScrollView.qml:35-45` | none |
| Q1.44 | slider | painted by Breeze, `elementType: "slider"` (`CC_Slider`); handle size from `subControlRect("handle")`; tick marks only when `StyleHints.tickMarkStepSize` set | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Slider.qml:35-53`; `private/DefaultSliderHandle.qml:16-29` | `slider.*` |
| Q1.45 | slider (RangeSlider) | RangeSlider is QML-drawn, not Breeze, so it looks different from Slider: handles 18×18 circle, track 6 px thick colour textColor alpha 0.3, fill highlightColor, padding 6; the model has one slider look | `qqc2-desktop-style@305eb2c4 org.kde.desktop/RangeSlider.qml:23-88` | `slider.*` (single variant) |
| Q1.46 | slider (RangeSlider) | handle border mix(bg,text,frameContrast), highlightColor when focused | `qqc2-desktop-style@305eb2c4 org.kde.desktop/RangeSlider.qml:32`, `:52` | none (no thumb border / focus fields) |
| Q1.47 | slider (RangeSlider) | handle drop shadow: offset x 1 y 1, black alpha 0.15, no blur | `qqc2-desktop-style@305eb2c4 org.kde.desktop/RangeSlider.qml:34-42`, `:54-62` | none |
| Q1.48 | progress_bar | painted by Breeze, `elementType: "progressbar"`; indeterminate passes maximum 0 (Breeze busy animation) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ProgressBar.qml:32-40` | `progress_bar.*` |
| Q1.49 | progress_bar | vertical insets largeSpacing above and below the bar | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ProgressBar.qml:25-26` | none |
| Q1.50 | tab (TabBar/TabButton) | tabs painted by Breeze, `elementType: "tab"` with tabpos/selectedpos/orientation; `"tabframe"` StyleItem invisible (metrics only); tab overlap from pixelMetric `tabOverlap` | `qqc2-desktop-style@305eb2c4 org.kde.desktop/TabButton.qml:49-106`; `TabBar.qml:38`, `:50-58` | `tab.*` |
| Q1.51 | tab | focus frame drawn only for keyboard focus reasons | `qqc2-desktop-style@305eb2c4 plugin/kquickstyleitem.cpp:1775-1779` | `defaults.focus_ring_*` (no keyboard-only rule) |
| Q1.52 | tab | TabBar uses Window colour set; minimum bar width gridUnit×6; tab min width text + gridUnit×2, height fallback gridUnit×2 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/TabBar.qml:18-24`; `TabButton.qml:20-21` | `tab.min_width_px`, `tab.min_height_px` (partial) |
| Q1.53 | sidebar (Kirigami OverlayDrawer/GlobalDrawer) | colour set follows modality: template uses View when modal / Window when not; GlobalDrawer reverses it (Window modal / View inline) | `kirigami@81ada72b src/templates/OverlayDrawer.qml:244`; `src/controls/GlobalDrawer.qml:337` | `sidebar.background_color` (one value) |
| Q1.54 | sidebar | inline drawer: 1 px separator on the inner edge in Header colour set; modal: no separator | `kirigami@81ada72b src/controls/OverlayDrawer.qml:39-56` | `sidebar.border` (colour set dependency missing) |
| Q1.55 | sidebar | "segmented" separator: the part beside the global toolbar is drawn separately with top/bottom gaps largeSpacing, margins animated longDuration InOutQuad | `kirigami@81ada72b src/controls/OverlayDrawer.qml:58-131` | none |
| Q1.56 | sidebar (modal) | edge shadow: gradient strip of width cornerRadius, stops black 0.25 @0, 0.1 @0.20, 0.02 @0.35, transparent @1.0; fades longDuration InOutQuad | `kirigami@81ada72b src/controls/private/EdgeShadow.qml:22-59`; `src/controls/OverlayDrawer.qml:133-151` | `sidebar.border.shadow_enabled` (bool only) |
| Q1.57 | sidebar (modal) | dim colour black alpha 0.35 | `kirigami@81ada72b src/templates/OverlayDrawer.qml:265-266` | none |
| Q1.58 | sidebar | open: longDuration OutQuad; close: longDuration InQuad; snap-back longDuration × position | `kirigami@81ada72b src/templates/OverlayDrawer.qml:277-318`, `:431-439` | none |
| Q1.59 | sidebar (collapse) | collapsed width iconSizes.medium + padding; collapse/expand width animation longDuration InOutQuad; labels fade longDuration/2 InOutQuad | `kirigami@81ada72b src/templates/OverlayDrawer.qml:92`, `:450-490`; `src/controls/private/GlobalDrawerActionItem.qml:88-92`, `:138-142` | none |
| Q1.60 | sidebar | size bounds: side drawers min gridUnit×8, max min(gridUnit×25, 80 % window); top/bottom min gridUnit×4, max min(gridUnit×15, 50 %); padding smallSpacing; resize grip smallSpacing×2 | `kirigami@81ada72b src/templates/OverlayDrawer.qml:193-228`, `:251`, `:493-500` | none |
| Q1.61 | sidebar (handle) | floating drawer handle: raised ToolButton, icon smallMedium, box smallMedium + smallSpacing×2, offset smallSpacing from the drawer; slide in/out longDuration OutQuad/InQuad; bottom margin animates shortDuration InOutQuad | `kirigami@81ada72b src/templates/private/DrawerHandle.qml:36-44`, `:118-127`, `:180-204`; `src/templates/OverlayDrawer.qml:115-142` | none |
| Q1.62 | sidebar (GlobalDrawer items) | item icon smallMedium, icon-label gap largeSpacing; expandable (parent) items font ×1.30; disabled label opacity 0.75; submenu arrow iconSizes.small at opacity 0.75; selected/checked → highlightedTextColor | `kirigami@81ada72b src/controls/private/GlobalDrawerActionItem.qml:40-112`, `:148` | `sidebar.font`, `sidebar.selection_*` (partial) |
| Q1.63 | sidebar (GlobalDrawer header) | title Heading (level 1) + title icon, spacing largeSpacing; section headers Heading level 4 with smallMedium icon | `kirigami@81ada72b src/controls/GlobalDrawer.qml:470-485`, `:574-588` | none |
| Q1.64 | sidebar (GlobalDrawer submenu) | sub-level push/pop: x slide + opacity, veryLongDuration OutCubic; content fades longDuration InOutQuad | `kirigami@81ada72b src/controls/GlobalDrawer.qml:567-571`, `:600-604`, `:650-655`, `:675-715`, `:736-741` | none |
| Q1.65 | sidebar (collapsed) | "Open Sidebar" tooltip with toolTipDelay on the collapsed rail | `kirigami@81ada72b src/controls/GlobalDrawer.qml:773-775` | none |
| Q1.66 | sidebar (ContextDrawer) | width gridUnit×20; header ToolBar whose height = global toolbar height, Heading with largeSpacing side margins; items left padding gridUnit; expandable font ×1.30; disabled opacity 0.75 | `kirigami@81ada72b src/controls/ContextDrawer.qml:134-183`; `src/controls/private/ContextDrawerActionItem.qml:36-40` | none |
| Q1.67 | sidebar (qqc2 Drawer) | 1 px separator in Header colour set on the inner edge, 1 px padding on that edge; enter/exit SmoothedAnimation velocity 5 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Drawer.qml:25-54` | `sidebar.border` (partial) |
| Q1.68 | toolbar (qqc2 ToolBar) | colour set by position: Header set as header, Window set as footer; separator line at the bottom (header) or top (footer) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ToolBar.qml:25-28`; `private/DefaultToolBarBackground.qml:18-41` | `toolbar.background_color`, `toolbar.border` (position variants missing) |
| Q1.69 | toolbar | padding smallSpacing; background implicit height 40 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ToolBar.qml:22`; `private/DefaultToolBarBackground.qml:18` | `toolbar.bar_height_px`, `toolbar.border.padding_*` |
| Q1.70 | toolbar (ToolSeparator) | vertical separator 1 px × (40 − 2×smallSpacing) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ToolSeparator.qml:20-24` | `separator.line_width_px` (length missing) |
| Q1.71 | toolbar (Kirigami page header) | desktop header style: Header colour set, non-inheriting; separator under the header unless the page header is a ToolBar | `kirigami@81ada72b src/styles/org.kde.desktop/AbstractApplicationHeader.qml:16-29` | `toolbar.*` |
| Q1.72 | toolbar (global toolbar) | page title = Heading level 1 elided; title left padding gridUnit; header paddings mediumSpacing, item spacing smallSpacing; preferred height 40 + 6 | `kirigami@81ada72b src/controls/private/DefaultPageTitleDelegate.qml:28-35`; `src/controls/private/globaltoolbar/PageRowGlobalToolBarStyleGroup.qml:50-55`; `src/controls/private/globaltoolbar/ToolBarPageHeader.qml:38-52` | `toolbar.bar_height_px`, `toolbar.font` (title font / title inset missing) |
| Q1.73 | toolbar (global toolbar) | leading separator in Header set before the title when columns scroll; back/forward icon-only buttons (`go-previous-symbolic`/`go-next-symbolic`) | `kirigami@81ada72b src/controls/private/globaltoolbar/ToolBarPageHeader.qml:54-62`; `src/controls/private/globaltoolbar/NavigationButtons.qml:87-118` | none |
| Q1.74 | toolbar (header motion) | header height animation longDuration InOutQuad; separator fade longDuration InOutQuad when controls hide; header max height gridUnit×3 + paddings | `kirigami@81ada72b src/templates/AbstractApplicationHeader.qml:65-72`, `:159-164`, `:185-199` | none |
| Q1.75 | toolbar (ActionToolBar overflow) | overflow button: icon `overflow-menu`, icon-only, no menu arrow; menu opens below (above when the bar is a footer) | `kirigami@81ada72b src/controls/ActionToolBar.qml:129`, `:203-220` | none |
| Q1.76 | toolbar (ActionToolBar) | default `flat: true`, display TextBesideIcon, separators are ToolSeparator; action tooltip appends the shortcut | `kirigami@81ada72b src/controls/ActionToolBar.qml:57`, `:74`, `:169-201`; `src/controls/private/PrivateActionToolButton.qml:100-130` | `toolbar.item_gap_px` (flat default / tooltip text rule missing) |
| Q1.77 | list (ItemDelegate family) | row background painted by Breeze: `elementType: "itemrow"` (alternate rows) + `"item"` (hover/selected) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/private/DefaultListItemBackground.qml:33-59` | `list.*` |
| Q1.78 | list | keyboard focus shows the hover look (hover = hovered or visual focus); pressed (`down`) shows the selected look | `qqc2-desktop-style@305eb2c4 org.kde.desktop/private/DefaultListItemBackground.qml:20`, `:31`, `:54-55` | `list.hover_background`, `list.selection_background` (focus/pressed mapping missing) |
| Q1.79 | list | row padding mediumSpacing + smallSpacing, bottom smallSpacing except last row, spacing mediumSpacing; background inset = padding − smallSpacing (rows drawn as separated bubbles; no inset in TableView) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ItemDelegate.qml:28-51` | `list.row_height_px` (padding / inset missing) |
| Q1.80 | list | row icon size smallMedium (medium for TextUnderIcon) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ItemDelegate.qml:39-41` | none |
| Q1.81 | list | text colour: highlighted or down → highlightedTextColor; disabled → disabledTextColor | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ItemDelegate.qml:82-84` | `list.selection_text_color`, `list.disabled_text_color` (pressed state missing) |
| Q1.82 | list | elided rows show a tooltip with the full text (toolTipDelay; press-and-hold in tablet mode) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ItemDelegate.qml:43-45` | none |
| Q1.83 | list (Check/Radio/SwitchDelegate) | leading Breeze `checkbox`/`radiobutton` indicator or QML Switch indicator; text turns highlightedTextColor only while pressed-and-unchecked (CheckDelegate also when highlighted) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/CheckDelegate.qml:80`, `:89-96`; `RadioDelegate.qml:80`, `:90`; `SwitchDelegate.qml:79`, `:88-93` | none |
| Q1.84 | list (SwipeDelegate) | swipe transition: SmoothedAnimation velocity longDuration/80, InOutCubic; background hidden while swiped | `qqc2-desktop-style@305eb2c4 org.kde.desktop/SwipeDelegate.qml:43-48`, `:87-90` | none |
| Q1.85 | list (TreeViewDelegate) | expand/branch indicator painted by Breeze `elementType: "itembranchindicator"` + StylePrivate.ItemBranchIndicators lines; indentation from pixelMetric `treeviewindentation` (fallback 20); margins/spacing smallSpacing | `qqc2-desktop-style@305eb2c4 org.kde.desktop/TreeViewDelegate.qml:22-36`, `:58-95` | `expander.content_indent_px`, `expander.arrow_*` (partial) |
| Q1.86 | list (HeaderView) | header sections painted by Breeze `elementType: "header"` with headerpos/sort arrow; an extra empty header section fills the trailing space | `qqc2-desktop-style@305eb2c4 org.kde.desktop/HorizontalHeaderView.qml:25-81`; `VerticalHeaderView.qml:25-73` | `list.header_*` |
| Q1.87 | list (Kirigami subtitle delegates) | subtitle colour mix(title colour, bg, 0.25) (highlightedText when selected), subtitle font smallFont; icon medium when a subtitle exists else smallMedium; icon gap mediumSpacing | `kirigami@81ada72b src/delegates/TitleSubtitle.qml:63-79`; `src/delegates/IconTitleSubtitle.qml:142`, `:186` | `list.item_font` (no secondary-text role) |
| Q1.88 | list (SwipeListItem) | trailing action buttons fade in on hover, veryShortDuration InOutQuad; drag handle smallMedium icon `overflow-menu-left/right` | `kirigami@81ada72b src/controls/SwipeListItem.qml:309-318`, `:327`, `:387-390` | none |
| Q1.89 | list (SwipeListItem) | revealed action area colour darker(bg, 1.05), pressed darker(bg, 1.1), with EdgeShadow on top and leading edges; snap animation longDuration InOutQuad | `kirigami@81ada72b src/controls/SwipeListItem.qml:446-474`, `:613-616` | none |
| Q1.90 | popover (qqc2 Popup) | shadow x 0, y 4, size 8, black alpha 0.3; padding 12 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Popup.qml:24`, `:46-57` | `popover.border.shadow_enabled`, `popover.border.padding_*` |
| Q1.91 | popover | fade in/out longDuration InOutQuad | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Popup.qml:26-44` | none |
| Q1.92 | splitter (qqc2 SplitView) | handle painted by Breeze `elementType: "splitter"`; hit area extended 6 px each side, 12 px while hovered | `qqc2-desktop-style@305eb2c4 org.kde.desktop/SplitView.qml:23-46` | `splitter.*` |
| Q1.93 | splitter (Kirigami ColumnView) | column separators are Kirigami.Separator in Header colour set; resize grip ±smallSpacing; minimum column gridUnit×8 | `kirigami@81ada72b src/layouts/ColumnViewSeparator.qml:51-67`, `:116-131` | `splitter.divider_color` (colour-set dependency missing) |
| Q1.94 | splitter (Kirigami ColumnView) | default column width gridUnit×20; column scroll animation veryLongDuration OutExpo | `kirigami@81ada72b src/layouts/columnview.cpp:461-462`, `:1985`, `:1993`; `src/controls/PageRow.qml:122` | none |
| Q1.95 | separator | two weights: Normal = mix(bg, text, frameContrast), Light = mix(bg, text, frameContrast/2) | `kirigami@81ada72b src/primitives/Separator.qml:30-48`, `:65-69`; `src/platform/platformtheme.cpp:762-766` | `separator.line_color` (one weight) |
| Q1.96 | separator | colour depends on the colour set it sits in (Header-set separators in toolbars, drawers, columns; Window elsewhere) | `kirigami@81ada72b src/primitives/Separator.qml:65-69`; `src/layouts/ColumnViewSeparator.qml:57-58`; `src/controls/OverlayDrawer.qml:54-55` | `separator.line_color` (single value) |
| Q1.97 | layout | Kirigami spacing ladder smallSpacing 4 / mediumSpacing 6 / largeSpacing 8 and the gridUnit 18 size unit (most metrics are multiples of them) | `kirigami@81ada72b src/platform/units.cpp:36-39` | `layout.{widget_gap,container_margin,window_margin,section_gap}_px` (partial) |
| Q1.98 | layout (Pane/Page) | Pane padding largeSpacing; qqc2 Page none; Kirigami Page padding gridUnit; Control padding from pixelMetric `layout*margin` | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Pane.qml:16`; `Control.qml:21-24`; `kirigami@81ada72b src/controls/Page.qml:33-37` | `layout.container_margin_px` (partial) |
| Q1.99 | switch | track is not full height: inset floor(h/6) top and bottom of a gridUnit-high, 2×gridUnit-wide indicator; handle is the full height | `qqc2-desktop-style@305eb2c4 org.kde.desktop/private/SwitchIndicator.qml:14-15`, `:38-51`, `:65-76` | `switch.track_*`, `switch.thumb_diameter_px` (expressible as numbers; derivation lost) |
| Q1.100 | switch | fill overlay: highlightColor alpha 0.5 from the track start to the handle's right edge (also present, under the handle, when unchecked) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/private/SwitchIndicator.qml:53-63` | `switch.checked_background` (partial-fill geometry missing) |
| Q1.101 | switch | handle border: hoverColor when hovered or visually focused, else mix(bg, text, frameContrast); track border = highlightColor when checked else the handle border colour | `qqc2-desktop-style@305eb2c4 org.kde.desktop/private/SwitchIndicator.qml:27-31`, `:50`, `:78` | `switch.thumb_border_color`, `switch.*_track_border_color` (hover/focus border missing) |
| Q1.102 | switch | handle slide SmoothedAnimation shortDuration (disabled while pressed); handle colour ColorAnimation shortDuration InCubic | `qqc2-desktop-style@305eb2c4 org.kde.desktop/private/SwitchIndicator.qml:80-93` | none |
| Q1.103 | switch | label gap smallSpacing; control padding 1; disabled label opacity 0.6 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Switch.qml:25-26`, `:72` | none (no switch label gap) |
| Q1.104 | dialog (qqc2 Dialog) | dim black alpha 0.3, opacity animated longDuration InOutQuad | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Dialog.qml:35-46` | none |
| Q1.105 | dialog | enter/exit opacity fade longDuration InOutQuad | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Dialog.qml:48-66` | `dialog.entrance` (enum only; duration/easing missing) |
| Q1.106 | dialog | shadow size = radius×2, black alpha 0.3, y offset 1 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Dialog.qml:77-81` | `dialog.border.shadow_enabled` |
| Q1.107 | dialog | native-window variant (`popupType` ≠ Item): radius 0, border 0, no header | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Dialog.qml:75`, `:84`, `:90` | none |
| Q1.108 | dialog | content padding gridUnit (0 when the content is a ScrollView); margins from window gridUnit | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Dialog.qml:31-32` | `dialog.border.padding_*` (window margin missing) |
| Q1.109 | dialog (footer) | DialogButtonBox: padding largeSpacing, right alignment, row height 32; separator above it only when the content scrolls | `qqc2-desktop-style@305eb2c4 org.kde.desktop/DialogButtonBox.qml:24-26`, `:45`; `Dialog.qml:94-108` | `dialog.button_gap_px`, `dialog.button_order` (padding/separator rule missing) |
| Q1.110 | dialog (footer) | standard buttons get icons (dialog-ok, dialog-cancel, …) when Breeze styleHint `dialogButtonsHaveIcons` is true | `qqc2-desktop-style@305eb2c4 org.kde.desktop/DialogButtonBox.qml:58-91` | none |
| Q1.111 | dialog (Kirigami Dialog) | entrance = opacity fade plus a rise of gridUnit×2 × (1 − opacity), longDuration InOutQuad | `kirigami@81ada72b src/dialogs/Dialog.qml:302-310` | `dialog.entrance` = FadeRise (distance/duration/easing missing) |
| Q1.112 | dialog (Kirigami Dialog) | dim black 0.3 (longDuration InOutQuad); shadow size radius×2, black 0.3, y 1; border 1 px mix(bg,text,frameContrast); View colour set | `kirigami@81ada72b src/dialogs/Dialog.qml:314-341` | `dialog.background_color`, `dialog.border` (dim, shadow geometry missing) |
| Q1.113 | dialog (Kirigami Dialog) | max size = window − largeSpacing×2 each axis | `kirigami@81ada72b src/dialogs/Dialog.qml:148`, `:156` | `dialog.max_width_px`, `dialog.max_height_px` (relative rule missing) |
| Q1.114 | dialog (header) | header padding largeSpacing; separator under the header only when the content scrolls; title Heading level 1, elided with tooltip; close button `window-close-symbolic` (swaps to `window-close` on hover) | `kirigami@81ada72b src/dialogs/DialogHeader.qml:63-78`; `src/dialogs/DialogHeaderTopContent.qml:80-109` | `dialog.title_font` (separator rule, close button missing) |
| Q1.115 | dialog (footer) | footer padding largeSpacing; empty-footer buffer height smallSpacing/2; footer separator when content scrolls; `flatFooterButtons` option | `kirigami@81ada72b src/dialogs/Dialog.qml:220`, `:421-424`, `:470`, `:483-491` | `dialog.button_gap_px` (partial) |
| Q1.116 | dialog (PromptDialog) | type icons `data-success` / `data-warning` / `data-error` / `data-information` at iconSizes.huge; content padding largeSpacing; title Heading + subtitle | `kirigami@81ada72b src/dialogs/PromptDialog.qml:83-89`, `:115-135`, `:140-151`, `:166-199` | `dialog.icon_size_px` (type-to-icon mapping missing) |
| Q1.117 | dialog (MenuDialog) | preferred width gridUnit×20; rows height gridUnit×2, icon gridUnit, horizontal padding largeSpacing + smallSpacing | `kirigami@81ada72b src/dialogs/MenuDialog.qml:90-124` | none |
| Q1.118 | dialog (SearchDialog) | size min(gridUnit×35, …) × min(gridUnit×20, …); frameless search field with smallSpacing margins; separator under it | `kirigami@81ada72b src/dialogs/SearchDialog.qml:159-160`, `:186-188`, `:232-233` | none |
| Q1.119 | spinner (qqc2 BusyIndicator) | spinner is the icon `process-working-symbolic` rotated, not a stroked arc; rotation period 2000 ms, deliberately not scaled by animation speed | `qqc2-desktop-style@305eb2c4 org.kde.desktop/BusyIndicator.qml:75-101` | `spinner.stroke_width_px`, `spinner.fill_color` (icon-based spinner missing) |
| Q1.120 | spinner | default size gridUnit×2; fade in/out shortDuration OutCubic; animation stops when longDuration ≤ 1 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/BusyIndicator.qml:35-56` | `spinner.diameter_px` (fade, stop rule missing) |
| Q1.121 | spinner (Kirigami PullDownIndicator) | pull-to-refresh arc: stroke width smallSpacing, stroke highlightColor | `kirigami@81ada72b src/controls/private/PullDownIndicator.qml:85-108` | `spinner.stroke_width_px`, `spinner.fill_color` |
| Q1.122 | combo_box | painted by Breeze `elementType: "combobox"` (`CC_ComboBox` + `CE_ComboBoxLabel`) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ComboBox.qml:121-139`; `plugin/kquickstyleitem.cpp:1830-1837` | `combo_box.*` |
| Q1.123 | combo_box | editable variant uses the View colour set, non-editable the Button set | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ComboBox.qml:19-20` | `combo_box.background_color` (single variant) |
| Q1.124 | combo_box | padding 5, editable trailing padding 24; content min width 75 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ComboBox.qml:33-35`, `:46-53` | `combo_box.border.padding_*`, `combo_box.arrow_area_width_px` |
| Q1.125 | combo_box (popup) | popup is the qqc2 Menu (so Menu's shadow/fade/item highlight rows apply), delegate MenuItem | `qqc2-desktop-style@305eb2c4 org.kde.desktop/ComboBox.qml:37-42`, `:141-154` | `menu.*` |
| Q1.126 | card (Kirigami AbstractCard/Card) | hover background tint(bg, highlight, 0.1); pressed/checked background tint(bg, highlight, 0.3) | `kirigami@81ada72b src/controls/private/DefaultCardBackground.qml:42-82` | `card.background_color` (no hover/pressed/checked) |
| Q1.127 | card | shadow size gridUnit, black alpha 0.05, y offset 2 | `kirigami@81ada72b src/controls/private/DefaultCardBackground.qml:90-94` | `card.border.shadow_enabled` |
| Q1.128 | card | second "lift" layer: a rectangle behind the card, offset down round(smallSpacing/4), colour darker(bg at alpha 0.6, 1.1); hidden while pressed with click feedback | `kirigami@81ada72b src/controls/private/DefaultCardBackground.qml:97-109` | none |
| Q1.129 | card | border 1 px mix(bg, text, frameContrast), radius cornerRadius, View colour set | `kirigami@81ada72b src/controls/private/DefaultCardBackground.qml:65-89`; `src/templates/AbstractCard.qml:92-93` | `card.border` (colour-set dependency missing) |
| Q1.130 | card | padding largeSpacing; gap largeSpacing between header/content/footer | `kirigami@81ada72b src/templates/AbstractCard.qml:96`, `:122-123` | `card.border.padding_*` (inner gap missing) |
| Q1.131 | card (banner) | banner image with title: title Heading level 1, white on images, over a black alpha 0.5 band 1.5× the title height; title icon min size iconSizes.large; paddings smallSpacing×2 (with icon) or largeSpacing; optional CheckBox | `kirigami@81ada72b src/controls/private/BannerImage.qml:37`, `:94-97`, `:159-221` | none |
| Q1.132 | card (footer) | footer ActionToolBar in Footer position | `kirigami@81ada72b src/controls/Card.qml:124-128` | none |
| Q1.133 | card (layouts) | CardsLayout: max 2 columns, column width gridUnit×12–×20, spacing largeSpacing; CardsListView: spacing and side margins largeSpacing×2 | `kirigami@81ada72b src/controls/CardsLayout.qml:40-73`; `src/controls/CardsListView.qml:34-37` | none |
| Q1.134 | link (LinkButton) | pressed colour Qt.darker(linkColor); disabled colour textColor (not a disabled-text role) | `kirigami@81ada72b src/templates/LinkButton.qml:59-93` | `link.active_text_color`, `link.disabled_text_color` (expressible as values; derivation lost) |
| Q1.135 | link | keyboard focus shown by bold text | `kirigami@81ada72b src/controls/LinkButton.qml:11` | none |
| Q1.136 | link (UrlButton) | external-link icon `open-link-symbolic` (fallback `link-symbolic`) at sizeForLabels after the text, gap smallSpacing, in the link colour; URL tooltip after toolTipDelay | `kirigami@81ada72b src/controls/UrlButton.qml:56-79`, `:95-97` | none |

---

## 4.2 Standard controls the model does not have

| ID | Control | What it draws that the model cannot hold | Source | Model today |
|---|---|---|---|---|
| K2.1 | **Menu bar** (QMenuBar; QQC2 MenuBar) | Items with `MenuBarItem_MarginWidth`/`MarginHeight`; hover = rounded `hoverColor` fill, pressed = `focusColor`, text `HighlightedText`; weak-focus variant = focus line; Header-set background in the tools area, inactive Header group | pub `st:5846-5955`; `mt:93-94`; `tam:199-232` | none (menu covers popups only) |
| K2.2 | **Tool button** (QToolButton, standalone and in toolbars) | All of K1.152–K1.157: auto-raise states, styles, menu arrow kinds, geometry | see K1.152–K1.157 | none |
| K2.3 | **Spin box** (QSpinBox, QDoubleSpinBox, QDateTimeEdit) | Line-edit frame; **up/down arrows** in a `SpinBox_ArrowButtonWidth` column, `arrowColor(Text)` → `hoverColor` per arrow, animated; disabled arrow; flat variant | pub `st:7938-7975`, `st:8368-8415`, `st:3028-3088`, `st:3382-3427`; `mt:66-68` | none |
| K2.4 | **Calendar / date picker** (QCalendarWidget, KDatePicker, KDateComboBox popup) | Navigation bar (month/year buttons, arrows), weekday header, week numbers, today/selected cells, weekend colours, other-month days | measure: probe app; pub qtbase `qcalendarwidget.cpp`, `kwa kdatepicker.cpp` (to verify lines) | none |
| K2.5 | **Dial** (QDial; QQC2 Dial) | Groove ring mix(Window, WindowText, 0.2), value arc `Highlight`, handle = slider handle with shadow, `Slider_ControlThickness`; hover on the handle | pub `st:8144-8217`, `st:3219-3257`; `hp:1100-1128` | none |
| K2.6 | **LCD number** (QLCDNumber) | Segment digits in palette colours, Outline/Filled/Flat segment styles, frame | measure: probe app; pub qtbase `qlcdnumber.cpp` (to verify) | none |
| K2.7 | **Command link button** (QCommandLinkButton) | Button frame + icon + **bold title** + wrapped description, `Button_ItemSpacing` | pub `st:1982-2061` | none |
| K2.8 | **Group box** (QGroupBox) | Rounded frame filled `frameBackgroundColor` + outline; title (normal) or **flat** variant (no frame, bold title +1 pt); **checkable** title; focus line under the title, faded | pub `st:4199-4227`, `st:7596-7718`, `st:2811-2925`; `mt:71` | none (`card` is Kirigami's) |
| K2.9 | **Tool box** (QToolBox) | Tab rows with icon (`ToolBox_TabItemSpacing`, `ToolBox_TabMarginWidth`, `ToolBox_TabMinWidth`); selected tab outlined in `focusColor` with a tab-shaped path; hover outline; hover fade | pub `st:7385-7510`, `st:2772-2804`; `hp:743-782`; `mt:143-145`; `animations/breezetoolboxengine.cpp` | none |
| K2.10 | **Dock widget title** (QDockWidget) | Title text (elided), optional frame (`DockWidgetDrawFrame`), float/close buttons as frameless glyphs (`SP_DockWidgetCloseButton`), separator | read breezerc `[Style] DockWidgetDrawFrame` (`kcfg:122-124`); pub `st:7512-7586`, `st:968-969`, `st:2125-2131` | none |
| K2.11 | **MDI sub-window** (QMdiSubWindow) | Title bar from `[WM]` colours (Breeze style reads `[WM]`, `hp:87-117`), top-rounded, inactive outline, active `Highlight` separator line, glyph buttons (min/max/restore/close), `TitleBar_MarginWidth`; MDI window shadow | pub `st:8241-8366`; `hp:1581-1645`; `kstyle/breezemdiwindowshadow.cpp:39-60`; `mt:120` | none |
| K2.12 | **Rubber band** (QRubberBand) | Fill `Highlight` α 0.2, outline `lighten(Highlight)`, radius `Frame_FrameRadius`; filled where its parent is an item view (or its viewport), a `QGraphicsView` or a `QMainWindow` (the dock drop zone, K-SF18†), outline-only (the inside masked out) elsewhere (`st:893-917`; corrected 2026-10-28) | pub `st:6789-6806`, `st:892-922` | none |
| K2.13 | **Inline message** (KMessageWidget) | Four types (positive, information, warning, error) with colours **hard-coded** from the default scheme; fill = 20 % of the type colour over the parent window; **inline**: 2 px border, radius 4 × 0.6; **header/footer** positions: flat fill + 1 px line; icon (`PM_ToolBarIconSize`); close button; show/hide **500 ms** `QTimeLine`, opacity = progress² | pub `kmw:28`, `kmw:60`, `kmw:162-190`, `kmw:357-393`, `kmw:537` | none |
| K2.14 | **Page view / page dialog** (KPageView, KPageDialog: List, Tree, Tabbed, FlatList faces) | Side list with large icons (`PM_IconViewIconSize`), page **title header** (KTitleWidget) with margins, separator line, search field container margins | pub `kwa kpageview.cpp:160-220`, `kwa kpageview_p.cpp:399-460` | none |
| K2.15 | **Title widget** (KTitleWidget) | Levelled title font (K1.270), comment line coloured by message type, icon | pub `kwa ktitlewidget.cpp:30-80` | partial: `text_scale` |
| K2.16 | **Side tab bar** (KMultiTabBar: Kate, KDevelop) | Vertical tab buttons, side separator line | pub `st:1400-1422`; `kwa kmultitabbar.cpp` (to verify lines) | none |
| K2.17 | **Splitter collapser** (KSplitterCollapserButton) | Overlay arrow button with an opacity fade (`TIMELINE_DURATION`) | pub `kwa ksplittercollapserbutton.cpp:184` (value to verify) | none |
| K2.18 | **LED** (KLed) | On/off colours, round/rectangular, raised/sunken/flat looks | pub `kwa kled.cpp` (to verify lines); measure | none |
| K2.19 | **Rating** (KRatingWidget) | Star icons, hover preview, half steps | pub `kwa kratingwidget.cpp` (to verify); measure | none |
| K2.20 | **Colour button** (KColorButton) | Swatch inside a push button frame | pub `kwa kcolorbutton.cpp` (to verify); measure | none |
| K2.21 | **Framed panel / scroll area** (QFrame StyledPanel, QAbstractScrollArea) | `Base` fill + rounded outline with hover/focus outline (input-widget engine, frame-shadow overlay); Box/Panel/WinPanel shapes use Qt's shade roles (K3.53) | pub `st:3924-3980`, `st:6573-6617`; `kstyle/breezeframeshadow.cpp` | partial: `list.border`, `card` |
| K2.22 | **Wizard** (QWizard) | Qt's wizard style on Linux (classic): title/subtitle area, banner, button row | measure: probe app; pub qtbase `qwizard.cpp` (to verify; Breeze does not override `SH_WizardStyle`) | none |
| K2.23† | **Date field and picker** (KDateComboBox, KDatePickerPopup, QDateEdit with a calendar popup) | `KDateComboBox` is a combo whose popup is a **menu** holding the `KDatePicker` (K2.4) and word items (Next Week, Today, Yesterday, No Date) between separators; `QDateTimeEdit` with `calendarPopup` paints as a **combo box** (`CC_ComboBox`), without it as a spin box (K2.3). The month grid (`KDateTable`): weekday header **bold** on `Window`, non-working days' header text **`Qt::darkRed`** and Sunday numbers `Qt::darkRed` (both hard-coded, dropped under a high-contrast preference), **today bold in `LinkVisited`**, selected `Highlight`/`HighlightedText`, other-month days Disabled `Text`, out-of-range days on the Disabled background, hover and selection through Breeze's item-view selection (`PE_PanelItemViewItem`, K1.170), a 1 px line under the header in the foreground colour | pub `kwa kdatecombobox.cpp:64`, `:114-125`; `kwa kdatepickerpopup.cpp:86-122`; `kwa kdatetable.cpp:279-292`, `:309-315`, `:343-374`, `:376-380`, `:382-413`; qtbase v6.10.1 `qdatetimeedit.cpp:2449-2470` | none |
| K2.24† | **Time field** (KTimeComboBox, QTimeEdit, KDateTimeEdit) | `KTimeComboBox` is an **editable** combo (K1.240) listing times, sized to its contents, that answers an invalid entry with an error message box (K1.224†) when `WarnOnInvalid` is set; `QTimeEdit` is a spin box (K2.3); `KDateTimeEdit` pairs the date and time fields | pub `kwa ktimecombobox.cpp:236-261`, `:265-270`; `kwa kdatetimeedit.cpp` (layout; to verify lines) | none |
| K2.25† | **Colour dialog** (QColorDialog through the KDE platform theme; KColorCombo) | The KDE platform theme claims the colour dialog but shows Qt's own widget dialog: a hue–saturation field with a black 20 × 2 / 2 × 20 cross-hair, a value strip framed by `qDrawShadePanel`, basic and custom colour **wells** (cells framed with `PE_Frame`, focus by `PE_FrameFocusRect`), per-channel spin boxes, a hex field, screen colour picking; `KColorCombo` lists colours in a combo; `KColorButton` is K2.20 | pub plasma-integration @ `46eddde9` `qt6/src/platformtheme/kdeplatformtheme.cpp:273-276`, `:337-348`; `qt6/src/platformtheme/kdeplatformcolordialoghelper.cpp:12`, `:38`; qtbase v6.10.1 `qcolordialog.cpp:322-358`, `:819-841`, `:943-955`; `kwa kcolorcombo.cpp` (to verify lines) | none |
| K2.26† | **File dialog** (KFileWidget) | `QFileDialog` under Plasma is KIO's `KFileWidget` (or the portal's dialog when the portal is used): the Places panel (K1.142†), the URL navigator (K2.27†), the file view in icon, compact or details mode (item-view rows K1.170–K1.185†), a preview pane, an **icon-size slider**, location and filter combos, the button box | pub plasma-integration @ `46eddde9` `qt6/src/platformtheme/kdeplatformtheme.cpp:330-334`; `qt6/src/platformtheme/kdeplatformfiledialoghelper.cpp:42`, `:66-72`; KDE/kio @ `eec01328` `src/filewidgets/kfilewidget.cpp:259-261`, `:298` (layout lines to verify); the portal's dialog: xdg-desktop-portal-kde (not read, to verify) | none |
| K2.27† | **URL navigator / breadcrumb** (KUrlNavigator, Qt Widgets) | One flat button per folder, padding 5; text in `Text` at alpha 255 for the current location, 128 for the others, 96 when not hovered; the current (last) crumb **bold**; hover, drag-over or an open popup draw a **flat button panel** (`PE_PanelButtonCommand`); **separator arrows** (`PE_IndicatorArrowRight`/`Left`, hover tracked per arrow) open a sub-folder menu; clipped names **fade out through a linear gradient** (text colour to background over the last 20 %); an editable-path mode and a places selector at the start (Quick breadcrumb: Q2.44) | pub KDE/kio @ `eec01328` `src/filewidgets/kurlnavigatorbuttonbase.cpp:89-117`; `src/filewidgets/kurlnavigatorbutton.cpp:44`, `:124-141`, `:171-257`; `src/filewidgets/kurlnavigator.cpp` (editable mode, places selector; lines to verify) | none |
| K2.28† | **Font chooser** (KFontRequester, KFontChooser) | `KFontRequester`: a **sunken styled-panel label** showing the font's name in that font, its frame forced on Breeze (`_breeze_force_frame`), beside a push button with the `document-edit` icon; `KFontChooser` / `KFontChooserDialog`: family, style and size lists, a size spin box, optional per-attribute check boxes, a preview text field at least one line high with `PM_LayoutVerticalSpacing` above it; Qt's `QFontDialog` is not replaced (the KDE platform theme returns no font-dialog helper), so it stays Qt's widget dialog | pub `kwa kfontrequester.cpp:55-63`; `kwa kfontchooser.cpp:156-168`, `:176-261`; plasma-integration @ `46eddde9` `qt6/src/platformtheme/kdeplatformtheme.cpp:349-351` | none |
| K2.29† | **Completion popup** (KCompletionBox for KLineEdit / KComboBox) | The default completion mode is a popup: a frameless list window under the field with a **1 px plain box frame**, as wide as the field, at most **15 rows** tall, vertical scroll bar as needed (a tooltip-type window on Wayland); the "auto" modes instead append the completion as selected text in the field (Quick: Q1.16) | pub KDE/kcompletion @ `2bc74a5b` `src/kcompletionbox.cpp:42-53`, `:350-365`; `src/klineedit.cpp:104-106`, `:964` | none |
| K2.30† | **Find bar** (KTextEditor search bar: Kate, KWrite, KDevelop) | The pattern field's background turns `PositiveBackground` on a match (also after wrapping), `NegativeBackground` on none, `NeutralBackground` for a neutral result (KColorScheme roles, K3.52); the status label `NegativeText` ("Not found"); matches and replacements in the text are highlighted in the editor theme's **search** and **replace** highlight colours (the KSyntaxHighlighting theme, not the colour scheme) | pub KDE/ktexteditor @ `df8e3ad8` `src/search/katesearchbar.cpp:336-390`, `:1658-1660` | none |
| K2.31† | **What's This popup** (QWhatsThis: Shift+F1, the title-bar help button) | A popup filled `ToolTipBase` with a 1 px `ToolTipText` frame and a second 1 px frame in the shade role `Dark` (K3.53) inside it, rich text inset 12 px left and right and 8 px top and bottom, placed by the pointer; Qt's hand-drawn hatched 6 px shadow in `Shadow` is drawn only when the platform's `DropShadow` hint is true, which the KDE platform theme does not set (whether Breeze's popup shadow, K3.2, applies: to verify) | pub qtbase v6.10.1 `qwhatsthis.cpp:127-136`, `:185-186`, `:246-282`, `:559-578`; `qplatformtheme.cpp:650-651` | none |
| K2.32† | **Ruler** (KRuler) | A ruler 20 px thick, marks from its end inwards: end mark 14 px, big ¾ of it, medium ½, little ½ of medium, base ½ of little; labels at 8 pt; an optional pointer triangle filled with the background colour; marks in the painter's default pen (the widget's foreground role, to verify) | pub `kwa kruler.cpp:17-54`, `:106-121`, `:314`, `:531-653` | none |

Checked, no gap: Breeze draws no size grip (`CE_SizeGrip` is `emptyControl`, `st:1511-1512`).

### Qt Quick: Kirigami and qqc2-desktop-style

Whole controls from the Quick survey (`kirigami@81ada72b`, `qqc2-desktop-style@305eb2c4`, full paths cited). Where qqc2 hands a part to Breeze through `StyleItem` (`elementType`), the Breeze rows above hold the gap; the Quick row only records what QML adds.

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| Q2.1 | SpinBox / DoubleSpinBox | whole control; painted by Breeze `elementType: "spinbox"` (`CC_SpinBox`), up/down/edit rects from `subControlRect`; QML: View colour set, padding 6, hover per button via `value` bits | `qqc2-desktop-style@305eb2c4 org.kde.desktop/SpinBox.qml:15-156`; `DoubleSpinBox.qml` (same structure) | `input.*` |
| Q2.2 | Dial | whole control; painted by Breeze `elementType: "dial"` (`CC_Dial`); implicit 128×128 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Dial.qml:13-32` | none |
| Q2.3 | DelayButton | whole control; Breeze `elementType: "delaybutton"`: `PE_PanelButtonTool` plus a highlight-tinted copy at opacity 0.5 clipped to the progress fraction; progress transition duration delay × (1 − progress) pressing, × 0.3 × progress releasing | `qqc2-desktop-style@305eb2c4 org.kde.desktop/DelayButton.qml:48-96`; `plugin/kquickstyleitem.cpp:1903-1925` | `button.*` |
| Q2.4 | RoundButton | whole control, QML-drawn: implicit gridUnit + 12 square, padding 6, spacing 4, icon smallMedium, radius = control radius | `qqc2-desktop-style@305eb2c4 org.kde.desktop/RoundButton.qml:32-36`, `:81-83` | `button.*` |
| Q2.5 | RoundButton | colours: normal backgroundColor; checked/down tint(textColor, border alpha 0.8); flat transparent; border mix(button, buttonText, frameContrast), flat → highlightColor, width 0 when flat and idle | `qqc2-desktop-style@305eb2c4 org.kde.desktop/RoundButton.qml:77-95` | `button.*_background` (partial) |
| Q2.6 | RoundButton | gloss gradient overlay white alpha 0.13 → black alpha 0.03 (non-flat) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/RoundButton.qml:97-106` | none |
| Q2.7 | RoundButton | hard drop shadow 1 px down, black alpha 0.15 (hidden when down/checked/flat); pressed/checked content shifts 1,1 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/RoundButton.qml:27-30`, `:107-118` | none |
| Q2.8 | RoundButton | colour set switches to Selection when focused or highlighted (non-flat) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/RoundButton.qml:16-17` | none |
| Q2.9 | MenuBar / MenuBarItem | whole control: bar in Header colour set; item padding smallSpacing vertical / largeSpacing horizontal, height gridUnit + 2×smallSpacing, min width 40 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/MenuBar.qml:15-34`; `MenuBarItem.qml:22-25`, `:49-51` | `menu.*` |
| Q2.10 | MenuBarItem | highlight: highlightColor at opacity 0.7 when down/highlighted, fading shortDuration; text highlightedTextColor while hovered and not pressed | `qqc2-desktop-style@305eb2c4 org.kde.desktop/MenuBarItem.qml:42`, `:52-60` | `menu.hover_background`, `menu.hover_text_color` |
| Q2.11 | PageIndicator | whole control: dots of largeSpacing diameter in textColor, opacity 1 current / 0.67 pressed / 0.33 other, spacing & padding largeSpacing, opacity anim shortDuration | `qqc2-desktop-style@305eb2c4 org.kde.desktop/PageIndicator.qml:21-41` | none |
| Q2.12 | Frame | transparent box, 1 px border mix(bg,text,frameContrast), radius cornerRadius, padding 6 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Frame.qml:21-27` | `card.border` |
| Q2.13 | GroupBox | 1 px border mix(bg,text,frameContrast) with no radius, title label (default font, textColor) above content, padding 6 | `qqc2-desktop-style@305eb2c4 org.kde.desktop/GroupBox.qml:22-40` | `card.border`, `expander.*` |
| Q2.14 | StackView (page transitions) | push/pop/replace: x slide by half the width + opacity, longDuration OutCubic | `qqc2-desktop-style@305eb2c4 org.kde.desktop/StackView.qml:16-105` | none |
| Q2.15 | PageRow layers (Kirigami) | layer push: opacity InOutCubic + y from height/2 OutCubic, longDuration; pop: opacity InOutCubic + y to height/2 InCubic; replace similar | `kirigami@81ada72b src/controls/PageRow.qml:761-843` | none |
| Q2.16 | InlineMessage | whole control; 1 px border in type colour: Information = activeTextColor, Positive = positiveTextColor, Warning = neutralTextColor, Error = negativeTextColor | `kirigami@81ada72b src/controls/InlineMessage.qml:23-31` | `defaults.{info,success,warning,danger}_color` |
| Q2.17 | InlineMessage | fill = backgroundColor with the type colour on top at opacity 0.20; inner radius = outer × 0.60 | `kirigami@81ada72b src/controls/InlineMessage.qml:35-59` | none |
| Q2.18 | InlineMessage | position variants: Inline (all borders, radius cornerRadius), Header (no top border, radius 0), Footer (no bottom border, radius 0) | `kirigami@81ada72b src/controls/InlineMessage.qml:33`, `:39-44`; `src/templates/InlineMessage.qml:113-137` | none |
| Q2.19 | InlineMessage | padding smallSpacing (+ border); icon smallMedium, default `emblem-information/-success/-warning/-error`; text left margin largeSpacing; icon top-aligned when multi-line | `kirigami@81ada72b src/controls/InlineMessage.qml:18-21`; `src/templates/InlineMessage.qml:220`, `:292-344`, `:366-368` | none |
| Q2.20 | InlineMessage | actions in a non-flat ActionToolBar, moved below the text when they do not fit; optional close button `dialog-close` | `kirigami@81ada72b src/templates/InlineMessage.qml:276-290`, `:405-452` | none |
| Q2.21 | InlineMessage | reveal: height longDuration, opacity shortDuration, content opacity shortDuration×2 | `kirigami@81ada72b src/templates/InlineMessage.qml:225-274` | none |
| Q2.22 | Chip | whole control; background default backgroundColor, checked highlight alpha 0.2, pressed highlight alpha 0.3 | `kirigami@81ada72b src/controls/private/DefaultChipBackground.qml:13-25`, `:50-59` | none |
| Q2.23 | Chip | border 1 px: default mix(bg,text,frameContrast), hovered hoverColor, checked highlight alpha 0.9, pressed highlight alpha 0.7; radius cornerRadius | `kirigami@81ada72b src/controls/private/DefaultChipBackground.qml:30-48`, `:60-73` | none |
| Q2.24 | Chip | icon small, spacing/padding smallSpacing (+smallSpacing when no icon); remove button `edit-delete-remove` at sizeForLabels; interactive/closable variants | `kirigami@81ada72b src/controls/Chip.qml:23-51`; `src/templates/Chip.qml:74-88` | none |
| Q2.25 | Badge | whole control; types Information/Positive/Warning/Error → border activeText/positiveText/neutralText/negativeText; background tint(bg, border, 0.2) | `kirigami@81ada72b src/controls/Badge.qml:48-66`; `src/templates/Badge.qml:67-72` | `defaults.*_color` status colours |
| Q2.26 | Badge | text colour mix(textColor — swapped to black/white when its brightness equals the background's — , bg, 0.17); custom colour border mix(custom, text, 0.5) | `kirigami@81ada72b src/controls/Badge.qml:48-51`, `:67-81` | none |
| Q2.27 | Badge | font smallFont bold; pill radius height/2 when single-line else cornerRadius; padding smallSpacing (+smallSpacing horizontally for pills); icon sizeForLabels | `kirigami@81ada72b src/controls/Badge.qml:19-31`, `:85-90`; `src/templates/Badge.qml:92-96` | none |
| Q2.28 | Avatar | whole control; default size iconSizes.large; colour from a fixed 12-colour palette hashed from the name | `kirigami@81ada72b src/templates/Avatar.qml:82-85`; `src/primitives/nameutils.cpp:85-97` | none |
| Q2.29 | Avatar | circle background tint(bg, avatar colour, 0.07), 1 px border in the avatar colour; initials in the avatar colour (fit to size); fallback icon `user` inset largeSpacing; image clipped to a circle; padding 1 | `kirigami@81ada72b src/controls/Avatar.qml:16-75`; `src/templates/Avatar.qml:48-50` | none |
| Q2.30 | PlaceholderMessage | whole control; icon round(iconSizes.huge × 1.5), textColor; Heading Primary (level 1); explanation label; action button top margin gridUnit; spacing largeSpacing | `kirigami@81ada72b src/controls/PlaceholderMessage.qml:197-201`, `:242`, `:259-319` | none |
| Q2.31 | PlaceholderMessage | Informational type dims icon, heading and explanation to opacity 0.75 (Actionable stays 1) | `kirigami@81ada72b src/controls/PlaceholderMessage.qml:132-154`, `:261`, `:284`, `:299` | none |
| Q2.32 | LoadingPlaceholder | "Loading…" placeholder with an indeterminate ProgressBar, max width gridUnit×20 | `kirigami@81ada72b src/controls/LoadingPlaceholder.qml:59-69` | `progress_bar.*` |
| Q2.33 | NavigationTabBar | whole control (bottom/top navigation bar): ToolBar background; content max width gridUnit×5 × max(n, 5), centred; equal-width buttons | `kirigami@81ada72b src/templates/NavigationTabBar.qml:180-277`; `src/controls/NavigationTabBar.qml:17-25` | `tab.*` |
| Q2.34 | NavigationTabButton | button: Window colour set, TextUnderIcon, padding/spacing/insets smallSpacing, background height gridUnit×3 (0 when TextBesideIcon); icon smallMedium (small beside text); text-only label font ×1.20 | `kirigami@81ada72b src/controls/NavigationTabButton.qml:35-58`, `:132-134` | `tab.min_height_px`, `tab.font` (partial) |
| Q2.35 | NavigationTabButton | states: pressed highlight alpha 0.3, hovered/checked highlight alpha 0.2, border highlightColor (1 px) when checked/down, radius cornerRadius; colour Behaviors shortDuration | `kirigami@81ada72b src/controls/NavigationTabButton.qml:20-26`, `:73-85`, `:118`, `:136` | `tab.active_background`, `tab.hover_background` (partial) |
| Q2.36 | NavigationTabButton | focus outline: 3 px ring at −2 px outside the button, highlight alpha 0.5, radius cornerRadius | `kirigami@81ada72b src/controls/NavigationTabButton.qml:61-71` | `defaults.focus_ring_*` (partial) |
| Q2.37 | OverlaySheet | whole control (a centred sheet: the template at this commit has no bottom-anchored branch, K-SF23†; corrected 2026-10-28); background = card background (DefaultCardBackground) | `kirigami@81ada72b src/controls/OverlaySheet.qml:11-15` | `dialog.*`, `card.*` |
| Q2.38 | OverlaySheet | dim black alpha 0.5 (opacity anim longDuration InOutQuad); header Heading level 2 with largeSpacing padding; header/footer separators; footer padding smallSpacing | `kirigami@81ada72b src/templates/OverlaySheet.qml:74-75`, `:100-113`, `:277-286`, `:313`, `:360-367` | none |
| Q2.39 | OverlaySheet | mobile drag-handle bar gridUnit×3 × gridUnit/4, textColor at opacity 0.4, fully rounded | `kirigami@81ada72b src/templates/OverlaySheet.qml:264-274` | none |
| Q2.40 | OverlaySheet | enter/exit: opacity + vertical translate gridUnit×5, longDuration InOutQuad; top inset ≥ gridUnit×3; swipe-to-close threshold gridUnit×5 | `kirigami@81ada72b src/templates/OverlaySheet.qml:128`, `:243`, `:415-450` | none |
| Q2.41 | ListSectionHeader | whole control: Heading level 5 Primary forced Bold at opacity 0.75, optional smallMedium icon, then a separator line filling the rest of the row; top padding largeSpacing + smallSpacing, spacing largeSpacing; no hover/background | `kirigami@81ada72b src/controls/ListSectionHeader.qml:60-108` | `text_scale.section_heading`, `separator.line_color` |
| Q2.42 | InlineViewHeader | whole control: background tint(View bg, text alpha 0.03), separator at the bottom (header) or top (footer); padding smallSpacing top/bottom/right, largeSpacing left; Heading level 2; actions right-aligned | `kirigami@81ada72b src/controls/InlineViewHeader.qml:27-90` | `list.header_*` |
| Q2.43 | ListItemDragHandle | whole control: icon `handle-sort` smallMedium at opacity 0.6, 1.0 while hovered/pressed; drop animation longDuration InOutQuad; autoscroll step gridUnit every 50 ms | `kirigami@81ada72b src/controls/ListItemDragHandle.qml:130-131`, `:159-165`, `:233-263` | none |
| Q2.44 | Breadcrumb (global toolbar, Breadcrumb style) | whole control: Header-set separator with largeSpacing top/bottom margins; crumb hover highlight alpha 0.1, radius cornerRadius; `go-next-symbolic` arrows at iconSizes.small; crumb Heading opacity 1 current / 0.4 others; scroll animation longDuration InOutQuad | `kirigami@81ada72b src/controls/private/globaltoolbar/BreadcrumbControl.qml:39-44`, `:123-127`, `:174-202` | none |
| Q2.45 | FormLayout | whole layout: row spacing smallSpacing, column spacing largeSpacing; narrow width cap gridUnit×32; wide/narrow switch at the wide implicit width | `kirigami@81ada72b src/layouts/FormLayout.qml:75`, `:138-148` | `layout.widget_gap_px` (partial) |
| Q2.46 | FormLayout | label alignment: wide mode right-aligned and top-aligned (or labelAlignment), text vertically centred; narrow mode left/bottom above the field | `kirigami@81ada72b src/layouts/FormLayout.qml:270-300` | none |
| Q2.47 | FormLayout | section titles Heading level 3 Primary, centred, extra space above largeSpacing×2; field labels Heading level 5 Normal; empty label height smallSpacing; narrow-mode label top margin largeSpacing×2 | `kirigami@81ada72b src/layouts/FormLayout.qml:427-469` | `text_scale.section_heading`, `layout.section_gap_px` (partial) |
| Q2.48 | ContextualHelpButton | whole control: icon-only ToolButton `help-contextual-symbolic`; click toggles a tooltip with no timeout (delay 0 on click, toolTipDelay on hover); WhatsThis cursor | `kirigami@81ada72b src/controls/ContextualHelpButton.qml:58-103` | `tooltip.*`, `button.*` |
| Q2.49 | PassiveNotification (showPassiveNotification toast) | whole control: Complementary colour set, background at opacity 0.9, radius cornerRadius, shadow size gridUnit/2, black alpha 0.4, y offset 2 | `kirigami@81ada72b src/controls/private/PassiveNotificationsManager.qml:91`, `:261-272` | none |
| Q2.50 | PassiveNotification | padding largeSpacing, content spacing mediumSpacing, stack spacing smallSpacing, bottom margin largeSpacing; width ≤ min(gridUnit×25, window/1.5); stack height ≤ gridUnit×10 | `kirigami@81ada72b src/controls/private/PassiveNotificationsManager.qml:26-31`, `:85`, `:101-109`, `:214-225` | none |
| Q2.51 | PassiveNotification | timeouts: short 4000 ms, default 7000 ms, long 12000 ms | `kirigami@81ada72b src/controls/private/PassiveNotificationsManager.qml:41-48` | none |
| Q2.52 | PassiveNotification | add: opacity + rise gridUnit×3, longDuration OutCubic; remove: opacity + drop gridUnit×3 + scale to 0, longDuration InCubic; displaced longDuration InOutCubic | `kirigami@81ada72b src/controls/private/PassiveNotificationsManager.qml:117-178` | none |
| Q2.53† | Touch text actions (qqc2 private MobileTextActionsToolBar) | in touch use (no hover) a text field or area shows a floating **Cut / Copy / Paste** bar: a qqc2 Popup of icon-only ToolButtons `edit-cut-symbolic`, `edit-copy-symbolic`, `edit-paste-symbolic`, centred on the selection start, above it, or gridUnit×1.5 below the selection end when there is no room; Cut and Copy hidden in password fields; the desktop right-click menu is a normal Menu (`TextFieldContextMenu.qml`, [menu] rows) | `qqc2-desktop-style@305eb2c4 org.kde.desktop/private/MobileTextActionsToolBarImpl.qml:15-87`; `TextArea.qml:43-56` | none |
| Q2.54† | Pull-to-refresh (Kirigami ScrollablePage `supportsRefreshing`, PullDownIndicator) | a strip gridUnit×4 high above the content; while pulling, the arc of Q1.121 sweeps 360° × pull progress, inset smallSpacing in the BusyIndicator's box; a full pull triggers after `refreshDelay` 500 ms; while refreshing the qqc2 BusyIndicator (Q1.119) spins and the content moves down by the strip height, longDuration InOutQuad | `kirigami@81ada72b src/controls/private/PullDownIndicator.qml:52`, `:67`, `:75-113`, `:116-121`, `:125-166`; `src/controls/ScrollablePage.qml:88-95`, `:314-319` | none |
| Q2.55† | Kirigami Forms, cards style (`org.kde.kirigami.forms`: FormGroup, FormEntry) | the boxed-list form: a **group** = optional title (Heading level 5, DemiBold, left inset cornerRadius, + largeSpacing when the card bleeds to the window edge) over an AbstractCard (Q1.126–Q1.130) holding the entries with no gap, min width gridUnit×30, extending largeSpacing past both sides in narrow windows; an **entry** = padding largeSpacing + smallSpacing (largeSpacing between two entries), a label column inside the card before the field (an inline label above it when the form is collapsed), leading and trailing items, a subtitle in disabledTextColor, min width gridUnit×20 + 2 × largeSpacing; a clickable entry's **hover** fill textColor at opacity 0.05, **pressed** 0.1, only the first and last entries' outer corners rounded cornerRadius, fading longDuration InOutQuad | `kirigami@81ada72b src/forms/cards/FormGroup.qml:21-58`; `src/forms/cards/FormEntry.qml:19`, `:29-37`, `:82-90`, `:120-195`, `:198-213` | none |
| Q2.56† | Kirigami Forms: separators, actions, flat style | cards: an entry **separator** = Kirigami Separator at opacity 0.5 inset largeSpacing each side, hidden (longDuration InOutQuad) while a clickable neighbour is hovered; an **action** entry = leading icon smallMedium + label + trailing `go-next-symbolic` at iconSizes.small. Flat style: no card; groups split by a Separator with largeSpacing margins and the form spacing (largeSpacing + smallSpacing) above and below, group title Heading level 3 Primary centred, entries smallSpacing apart, separators empty largeSpacing gaps, actions push buttons. Both: the form collapses (labels above fields) when its wide width does not fit, then capped at gridUnit×30 | `kirigami@81ada72b src/forms/cards/FormSeparator.qml:21-46`; `src/forms/cards/FormAction.qml:17-31`; `src/forms/templates/FormAction.qml:64-77`; `src/forms/flat/FormGroup.qml:30-72`; `src/forms/flat/FormSeparator.qml:15-18`; `src/forms/flat/FormAction.qml:16-29`; `src/forms/templates/Form.qml:71-128` | none |
| Q2.57† | About page (Kirigami AboutItem / AboutPage) | built from the cards forms: a header entry with the application icon at iconSizes.huge (largeSpacing after it, at most a third of the width), the name and version as a Heading, the short description as Heading level 3 Secondary; sections as form groups split by FormSeparators; author entries with an Avatar (Q2.28–Q2.29) at iconSizes.medium | `kirigami@81ada72b src/controls/AboutItem.qml:127-135`, `:185-213`, `:255-290` | none |
| Q2.58† | Dialog layer (Kirigami `PageRow.pushDialogLayer`) | on mobile, a window wider than gridUnit×40 gets a modal in-window dialog inset gridUnit×2.5 on every side, header gridUnit×1.6 high with an elided Heading (largeSpacing left margin) and a close icon at smallMedium (`window-close-symbolic`, `window-close` on hover); a narrower window pushes the page as a full-window layer (Q2.15); on the desktop a separate window, default gridUnit×50 by gridUnit×30 | `kirigami@81ada72b src/controls/PageRow.qml:264-280`, `:315-320`, `:341-346`; `src/controls/private/MobileDialogLayer.qml:12-75` | none |

---

## 4.3 Properties that cross widgets

### Shadows and elevation

| ID | Property | What is missing | Source | Model today |
|---|---|---|---|---|
| K3.1 | **Window shadow** (decoration) | Two layers, each with offset, blur radius, opacity, plus a composite offset, per size step (None/Small/Medium/Large/VeryLarge); strength (25–255) and colour; box radius = window radius + 0.5; `Shadow_Overlap`; inactive = half strength (K1.18) | read breezerc `[Common] ShadowSize`, `ShadowStrength`, `ShadowColor` (`dkcfg:11-31`); pub `deco:41-110`, `deco:811-868`, `deco.h:38`; `libbreezecommon/breezeboxshadowrenderer.cpp` (blur model) | partial: `defaults.shadow_color`, `border.shadow_enabled` |
| K3.2 | **Popup shadow** (menus, tooltips, dock widgets, floating toolbars, combo popups) | Breeze style's own two-layer table per size step (different from the window's), same breezerc keys, `Shadow_Overlap`, frame radius; which window kinds get it | read breezerc `[Common] ShadowSize`, `ShadowStrength`, `ShadowColor` (`kcfg:11-32`); pub `shd:33-60`, `shd:180-230`, `shd:259-320`; `mt:172` | partial: as K3.1 |
| K3.3 | MDI window shadow | Same table as K3.2, laid round sub-windows | pub `kstyle/breezemdiwindowshadow.cpp:39-60` | none |
| K3.4 | **Control micro-shadows** | Button/combo bottom shadow and slider/dial handle shadow (black α 0.125 1 px strokes), state-gated (K1.23, K1.101) | pub `hp:1646-1690`, `hp.h:88-91` | partial: `shadow_enabled` |
| K3.5 | Kirigami shadows | `ShadowedRectangle` shadows (cards, drawers, sheets), one layer each with its own geometry | Q3.20, Q3.21 | partial |

### Translucency and blur

| ID | Property | What is missing | Source | Model today |
|---|---|---|---|---|
| K3.10 | Menu translucency | K1.65 (MenuOpacity + blur) | read breezerc `[Style] MenuOpacity` | none |
| K3.11 | **KWin blur** | Blur strength (iterations/offset table), noise strength, saturation of the blurred backdrop | read kwinrc `[Effect-blur] BlurStrength`, `NoiseStrength`, `Saturation` (`kwin src/plugins/blur/blur.kcfg:7-16`); pub `kwin src/plugins/blur/blur.cpp:192-256` | none |
| K3.12 | Inactive-window dimming | Dim-inactive effect (off by default): strength, which windows | read kwinrc `[Effect-diminactive] Strength`, … (`kwin src/plugins/diminactive/diminactive.kcfg:7-24`); default-off `metadata.json:56` | none |
| K3.13 | Window translucency effect | KWin translucency effect (inactive/moving/menu opacity) | read kwinrc `[Effect-translucency]` (to verify; plugin not read) | none |
| K3.14 | Effect enablement | Whether blur, scale, fading popups, squash, maximize, sliding popups run (all default-on at this commit; fade, glide, magic lamp, dim-inactive default-off) | read kwinrc `[Plugins] <id>Enabled` (key pattern to verify); pub `kwin src/plugins/*/metadata.json` `EnabledByDefault` | none |
| K3.15† | **Modal dialog's parent** | KWin's dialog-parent effect (on by default): while a modal dialog is open, its parent window is drawn at **saturation 0.4, brightness 0.6**, animated over `animationTime(300)` (so scaled by the animation factor) and back | read kwinrc `[Plugins]` enablement (K3.14); pub `kwin src/plugins/dialogparent/package/contents/code/main.js:13`, `:38-60`; default-on `kwin src/plugins/dialogparent/package/metadata.json:113` | none |
| K3.16† | **Not-responding window** | KWin's frozen-app effect (on by default): a window that stops responding is drawn at **saturation 0.1, brightness 1.5**, fading in over `animationTime(1500)` and back over `animationTime(250)` when it answers | read kwinrc `[Plugins]` enablement (K3.14); pub `kwin src/plugins/frozenapp/package/contents/code/main.js:13-17`, `:41-53`, `:73-81`; default-on `kwin src/plugins/frozenapp/package/metadata.json:113` | none |

### Motion

| ID | Property | What is missing | Source | Model today |
|---|---|---|---|---|
| K3.30 | **Global animation speed** | `AnimationDurationFactor` is a continuous factor (0 = off), applied by Breeze (×100 ms), KWin, Kirigami units, the decoration | read kdeglobals `[KDE] AnimationDurationFactor` (`kwin src/kwin.kcfg:313-315`); pub `st:2075-2092`, `qqc2 kirigami-plasmadesktop-integration/animationspeedprovider.cpp:41-47` | partial: `AccessibilityPreferences.reduce_motion` (bool) |
| K3.31 | **Breeze state animations** | Duration `AnimationsDuration` (overridden by the factor), linear progress **quantised into `AnimationSteps` steps**, on/off; engines: widget state (buttons, check/radio, slider, group box: hover, focus, pressed), input frames, scrollbar, dial, combo, tool button, spin box, tab bar, header view, tool box, **enable/disable cross-fade** (`disabledPalette`), stacked-widget fade (off by default) | read breezerc `[Style] AnimationsEnabled`, `AnimationSteps`, `AnimationsDuration`, `StackedWidgetTransitionsEnabled` (`kcfg:45-60`); pub `anim:50-82`, `anim:90-195`, `animations/breezeanimationdata.h:42-77`, `animations/breezeanimation.h:12-25`; `hp:335-347` | none |
| K3.32 | Decoration motion | K1.8, K1.17 | — | none |
| K3.33 | **Window open/close** | KWin scale effect: scale from/to `InScale`/`OutScale` with opacity, `OutCubic` in, `InCubic` out, duration × factor | read kwinrc `[Effect-scale] Duration`, `InScale`, `OutScale` (`kwin src/plugins/scale/package/contents/config/main.xml:8-15`); pub `…/scale/package/contents/code/main.js:35-39`, `:105-155` | partial: `dialog.entrance` |
| K3.34 | **Popup fade** (menus, tooltips, combo popups) | Fading popups: in 150 ms linear, out 4 × that with `OutQuart`, × factor | pub `kwin src/plugins/fadingpopups/package/contents/code/main.js:23-57`, `:62-104` | none |
| K3.35 | Minimize / maximize / Plasma popups | Squash (250 ms, `InCubic`/`OutCubic`), maximize (250 ms `OutCubic`), sliding popups (200 ms, configurable) | read kwinrc `[Effect-slidingpopups] SlideInTime`, `SlideOutTime` (`slidingpopups.kcfg:8-13`); pub `squash/…/main.js:12-14`, `:51`, `:122`; `maximize/…/main.js:25`, `:57`; `slidingpopups.cpp:96-107` | none |
| K3.36 | **Kirigami/Plasma units** | `veryShortDuration` / `shortDuration` / `longDuration` / `veryLongDuration` = longDuration ÷ 4, ÷ 2, × 1, × 2, longDuration from plasmarc × factor; `humanMoment`, `toolTipDelay` | read plasmarc `[Units] longDuration` (`qqc2 kirigami-plasmadesktop-integration/plasmadesktopunits.cpp:38-55`); pub `kirigami src/platform/units.cpp:36-47` | none |
| K3.37 | KF widget timelines | KMessageWidget and KCollapsibleGroupBox 500 ms `QTimeLine` (Qt's default curve; to verify which) | pub `kmw:60`, `kwa kcollapsiblegroupbox.cpp:48` | none |
| K3.38† | **Qt's style-timed animations** | Qt Widgets animations timed by the style hint `SH_Widget_Animation_Duration`, which Breeze sets to `AnimationsDuration` (0 with animations off): the **dock-widget and toolbar re-layout** of a QMainWindow (`QWidgetAnimator`, geometry slide `InOutQuad`) and the tree-view expand (K1.185†); Qt's fixed timings Breeze does not touch: the line-edit side-icon fade 160 ms (K1.46†), the tab move 250 ms (K1.133†) | pub `st:942-943`; qtbase v6.10.1 `qwidgetanimator.cpp:54-62` | none |
| K3.39† | **Fullscreen transition** | KWin's fullscreen effect (on by default): entering or leaving fullscreen morphs the window's size and position with a cross-fade from the previous frame, `animationTime(250)`, `OutCubic` | pub `kwin src/plugins/fullscreen/package/contents/code/main.js:25`, `:66-110`; default-on `kwin src/plugins/fullscreen/package/metadata.json:102` | none |

### Focus

| ID | Property | What is missing | Source | Model today |
|---|---|---|---|---|
| K3.20 | **Keyboard focus frame** (QFocusFrame) | Shown **only after keyboard focus** (Tab, Backtab, shortcut) and only for line edits, text edits, spin boxes, combos, push/tool buttons, checkboxes, radios, sliders, dials, group boxes; a 2 px ring (`PM_FocusFrameH/VMargin`) **filled** `Highlight` α `Blend_Value`; inner radius `Frame_FrameRadius`, outer + margin; circular for radios and slider handles | pub `st:1640-1685`, `st:6619-6788`, `st:737-740` | partial: `focus_ring_color` / `width` / `offset` (no fill-vs-stroke, no alpha rule, no keyboard-only policy, no shape/radius) |
| K3.21 | **Focus line** | 1 px line 2 px under text: tab labels, group box titles, weak-focus menus and menu bar items | pub `hp:407-422` | none |
| K3.22 | Fallback focus rect | `PE_FrameFocusRect` elsewhere = a 1 px underline `Highlight.lighter(110)`; item views get a rounded rect | pub `st:4092-4170` | none |
| K3.23 | Focus colour per widget | Frames, arrows, slider/scrollbar thumbs take `focusColor` on focus (rows above) | pub `hp:142-169`, `hp:221-303` | partial: `input.focus_border_color` only |
| K3.24 | Focus fade | Focus outline/line fades with the state engines | pub `st:7105-7170`, `st:7688-7710` | none |

### Text caret, selection, mnemonics, text rendering

| ID | Property | What is missing | Source | Model today |
|---|---|---|---|---|
| K3.40 | **Inactive-window selection** | `[ColorEffects:Inactive] ChangeSelectionColor` → Selection in the Inactive group = Window background tinted with the Selection background (0.4), text = Window text; applies to item views and text | read kdeglobals `[ColorEffects:Inactive] ChangeSelectionColor`, `Enable` (`colors` BreezeLight `:18-27`); pub `kcs:303-321`, `kcs:386-389` | partial: `selection_inactive_background` (no text colour; reader does not compute it) |
| K3.41 | **Caret blink** | Blink period; 0 = no blink; clamped 200–2000 | read kdeglobals `[KDE] CursorBlinkRate` (`khint:79-80`, `:383-384`) | none |
| K3.42 | **Caret width** | `PM_TextCursorWidth` (Breeze does not override; Qt's) | pub qtbase `qcommonstyle.cpp` (to verify line); Breeze `st:645-890` has no case | none |
| K3.43 | Caret while selecting | Whether the caret blinks with a selection (`SH_BlinkCursorWhenTextSelected`) | pub qtbase `qcommonstyle.cpp` (to verify) | none |
| K3.44 | **Mnemonic underlines** | Never / on Alt (auto) / always | read breezerc `[Style] MnemonicsMode` (`kcfg:82-89`); pub `st:2165`, `kstyle/breezemnemonics.cpp:20-75` | none |
| K3.45 | **Font rendering** | Antialiasing, hinting style, sub-pixel order | read kdeglobals `[General] XftAntialias`, `XftHintStyle`, `XftSubPixel` (keys to verify in plasma-workspace `kcms/fonts`) | none |
| K3.46 | Font string extras | QFont string fields beyond family/size/weight/style: capitalization, letter/word spacing, stretch, style name, features, variable axes | read kdeglobals font strings (`docs/platform-facts.md` §1.3.1) | partial: `Font` (family, size, weight, style, colour) |
| K3.47 | Extra font roles | `taskbarFont` (panel) | read kdeglobals `[General] taskbarFont` (`kfont:52`) | none |
| K3.48† | **Input-method pre-edit** | Text being composed by an input method is drawn inline with the text formats the input method's attributes carry, with its own caret position; what those formats are on Plasma (Wayland text-input-v3 carries no styling) is to verify in qtwayland's input context; measure: probe app with fcitx5 / IBus | pub qtbase v6.10.1 `qwidgetlinecontrol.cpp:633-638` (pre-edit caret); qtwayland `qwaylandinputcontext.cpp` (not read, to verify); measure | none |

### Colour scheme roles and effects

| ID | Property | What is missing | Source | Model today |
|---|---|---|---|---|
| K3.50 | **Disabled effects** | `[ColorEffects:Disabled]` intensity/colour/contrast effects and amounts, applied by KColorScheme to every disabled colour; the reader neither reads them nor derives disabled colours (it takes `[Colors:View] ForegroundInactive` for `disabled_text_color`, `kde/colors.rs:493`) | read kdeglobals `[ColorEffects:Disabled] IntensityEffect`, `IntensityAmount`, `ColorEffect`, `ColorAmount`, `Color`, `ContrastEffect`, `ContrastAmount` (`kcs:21-56`, `kcs:377-390`) | partial: per-widget `disabled_*` (preset data only) |
| K3.51 | **Inactive-window palette** | `[ColorEffects:Inactive]` (when `Enable`, e.g. desaturate inactive windows) and per-set `[Colors:X][Inactive]` subgroups (Breeze ships one for Header) → a whole Inactive group | read kdeglobals `[ColorEffects:Inactive] *`, `[Colors:*][Inactive]` (`kcs:346-352`, `kcs:377-390`) | none |
| K3.52 | **Status backgrounds** | Computed `ActiveBackground`, `LinkBackground`, `VisitedBackground`, `NegativeBackground`, `NeutralBackground`, `PositiveBackground` = tint(normal background, the foreground role) per set | pub `kcs:392-404` (from read `[Colors:*] Foreground*`) | none (`danger_color` etc. are foregrounds) |
| K3.53 | **Shade roles** | `Light`, `Midlight`, `Mid`, `Dark`, `Shadow` computed from Window with `[KDE] contrast` (used by Qt for 3-D frames, table grid, LCD) | read kdeglobals `[KDE] contrast` (`kcs:519-527`; Breeze sets it, `colors` BreezeLight `:199-200`); pub `kcs:556-610`, `kcs:683-687` | none (`contrast` not read; `list.grid_color` none) |
| K3.54 | **Per-set roles** | Sets the model flattens or omits: **Complementary** (all roles), Tooltip/Header/Button/Selection sets' `ForegroundActive`, `ForegroundInactive`, `ForegroundLink`, `ForegroundVisited`, `ForegroundNegative/Neutral/Positive`, `BackgroundAlternate`, `DecorationFocus/Hover` | read kdeglobals `[Colors:*]` (`kcs:250-278`; `colors`) | none / partial (`defaults.*` take the View/Window values) |
| K3.55 | Header colours | Header set for title bars (K1.13), tools area (K1.150), inactive Header | read kdeglobals `[Colors:Header]`, `[Colors:Header][Inactive]` | partial: `list.header_*`, `toolbar.background_color` |

### Icons and cursor

| ID | Property | What is missing | Source | Model today |
|---|---|---|---|---|
| K3.60 | **Icon state effects** | Disabled = to-gray + semi-transparent; active (hover) for Desktop/Panel = to-gamma 0.7; default none (hard-coded in KF6) | pub `kie:57-91`, `kie:144-230` | none |
| K3.61 | **Symbolic icon recolouring** | Icons that follow the colour scheme (`FollowsColorScheme`) are recoloured from the palette (text, highlight, negative/neutral/positive…) per icon mode | pub `kit:440`; `hp:1881-1902` | none |
| K3.62 | Selected/pressed icon mode | Icons in selected rows and pressed menu items use `QIcon::Selected` (`HighlightedText` recolour) | pub `st:5893-5905`, `st:6111-6120` | none |
| K3.63 | Icon size groups | `MainToolbar` vs `Toolbar`; list, icon-view, tab, button icon sizes | read icon theme `index.theme` `*Default` (`kit:465-473`) | partial: `IconSizes` (5 contexts) |
| K3.64 | **Mouse cursor** | Cursor theme and size | read kcminputrc `[Mouse] cursorTheme`, `cursorSize` (`khint:495-504`) | none |
| K3.65† | **Cursor per element** | The cursor shape the toolkits set per element (drawn from the theme of K3.64): pointing hand over links (KUrlLabel, Kirigami LinkButton, links in labels and form subtitles), I-beam over selectable labels, split cursor on Kirigami column separators, open/closed hand on list drag handles, What's-This cursor on the contextual help button, resize cursors on drawer edges; `KLineEdit` **auto-hides the pointer** (KCursor, delay 5000 ms) | pub `kwa kurllabel.cpp:78`; `kwa kcursor.cpp:147`, `:174`; KDE/kcompletion @ `2bc74a5b` `src/klineedit.cpp:101`; `kirigami@81ada72b src/templates/LinkButton.qml:135`, `src/controls/SelectableLabel.qml:266`, `src/layouts/ColumnViewSeparator.qml:81`, `src/controls/ListItemDragHandle.qml:144`, `src/controls/ContextualHelpButton.qml:101`, `src/templates/OverlayDrawer.qml:501`, `src/forms/cards/FormEntry.qml:193`; `qqc2-desktop-style@305eb2c4 org.kde.desktop/Label.qml:23` | none |
| K3.66† | **Launch feedback** | KWin's startup feedback (on by default) while an application starts: its icon **bouncing** by the cursor (500 ms per bounce, the default), or blinking through black, dark grey, light grey and white (5 frames × 100 ms), or a passive busy cursor; for at most `Timeout` 5 s | read klaunchrc `[FeedbackStyle] BusyCursor`, `[BusyCursorSettings] Timeout`, `Bouncing`, `Blinking` (`kwin src/plugins/startupfeedback/startupfeedback.cpp:116-135`); pub `…/startupfeedback.cpp:45-57`, `:152-166`; default-on `…/startupfeedback/metadata.json:56` | none |

### Visible timings and behaviour

| ID | Property | What is missing | Source | Model today |
|---|---|---|---|---|
| K3.70 | Double-click interval | `DoubleClickInterval` | read kdeglobals `[KDE] DoubleClickInterval` (`khint:81`, `:386-387`) | none |
| K3.71 | Drag threshold | `StartDragDist`, `StartDragTime` | read kdeglobals `[KDE] StartDragDist`, `StartDragTime` (`khint:82-83`) | none |
| K3.72 | Wheel step | `WheelScrollLines` | read kdeglobals `[KDE] WheelScrollLines` (`khint:126-128`) | none |
| K3.73 | Single-click activation | `SingleClick` (item views open on one click; hover feedback in file views) | read kdeglobals `[KDE] SingleClick` (`khint:90`) | none |
| K3.74 | Submenu delay | K1.73 | pub `st:931-932` | none |
| K3.75 | Tooltip timings | K1.80; Kirigami `toolTipDelay`, `humanMoment` (K3.36) | — | partial |
| K3.76† | Auto-repeat and press-and-hold | Spin-box arrows **auto-repeat**: first repeat after 500 ms, then every 150 ms (mouse) or 75 ms (key), Qt's values (Breeze sets none of these hints); press-and-hold 800 ms (Qt's default; touch tooltips, Q1.36); drag-hover tab switch 500 ms (K1.133†); Quick scroll-bar repeat is Q1.42 | pub qtbase v6.10.1 `qcommonstyle.cpp:5173-5182`; `qplatformtheme.cpp:667-668`; no such case in `st:885-981` | none |

### Size variants

| ID | Property | What is missing | Source | Model today |
|---|---|---|---|---|
| K3.80 | Decoration sizes | Button size and border size enums (K1.4, K1.11) | read breezerc, kwinrc | none |
| K3.81 | Shadow size | K3.1/K3.2 size enum | read breezerc `[Common] ShadowSize` | none |
| K3.82 | Toolbar variants | Tool button style and main/other icon sizes (K1.153, K1.158) | read kdeglobals | partial |
| K3.83 | Touch / tablet mode | Kirigami `Settings.tabletMode` / `isMobile` size changes | Q3.31; read the `org.kde.KWin.TabletModeManager` D-Bus property (to verify) | none |
| K3.84† | Display scale (Wayland) | On Wayland the **scale** is per output and may be fractional, stored by KWin in `kwinoutputconfig.json` (`scale`) and sent to clients by the compositor; no kdeglobals key holds it, so `platform-facts.md` §1.3.7 has "(none)" for Wayland | read `~/.config/kwinoutputconfig.json` `scale` (`kwin src/outputconfigurationstore.cpp:1158`, `:1248`, `:1598`) | none |

### Accessibility variants

| ID | Property | What is missing | Source | Model today |
|---|---|---|---|---|
| K3.90 | Reduced motion degree | K3.30 (factor, not a bool) | read kdeglobals `[KDE] AnimationDurationFactor` | partial |
| K3.91 | Increased contrast | Plasma has no toggle; contrast comes from the scheme and from `[KDE] frameContrast` (read) and `[KDE] contrast` (not read, K3.53) | read kdeglobals `[KDE] contrast` | partial: `AccessibilityPreferences.high_contrast` = none |
| K3.92 | Reduced transparency | No toggle; MenuOpacity and the blur effect are the transparency (K3.10, K3.11) | read breezerc, kwinrc | partial: `reduce_transparency` = none |
| K3.93 | Colour filters | KWin colour-blindness correction / invert / zoom effects change every pixel | read kwinrc `[Plugins]` (to verify names) | none |
| K3.94 | Cursor size | K3.64 | read kcminputrc | none |
| K3.95† | **Visual bell** | KWin's system-bell effect (on by default; the visible bell itself is off by default): on a bell the window flashes in `VisibleBellColor` (default red) or inverted (`VisibleBellInvert`), for `VisibleBellPause` 500 ms, at most once per 700 ms | read kaccessrc `[Bell] VisibleBell`, `VisibleBellColor`, `VisibleBellInvert`, `VisibleBellPause` (`kwin src/plugins/systembell/systembell.cpp:126-135`); pub `…/systembell.cpp:110-111`, `:143-159`, `…/systembell/shaders/color.frag:3-9`; default-on `…/systembell/metadata.json:52` | none |
| K3.96† | **Shake cursor** | KWin's shake-cursor effect (on by default): shaking the pointer magnifies the cursor ×3 (`Magnification`, + `OverMagnification` 1 per further shake), 200 ms `InOutCubic`, deliberately not scaled by the animation factor | read kwinrc `[Effect-shakecursor] TimeInterval`, `Sensitivity`, `Magnification`, `OverMagnification` (`kwin src/plugins/shakecursor/shakecursorconfig.kcfg:7-21`); pub `…/shakecursor.cpp:82-106`; default-on `…/shakecursor/metadata.json:53` | none |

### Qt Quick: Kirigami and qqc2-desktop-style

Cross-widget rows from the Quick survey (`kirigami@81ada72b`, `qqc2-desktop-style@305eb2c4`, full paths cited). Where qqc2 hands a part to Breeze through `StyleItem` (`elementType`), the Breeze rows above hold the gap; the Quick row only records what QML adds.

| ID | Element | What is missing | Source | Model today |
|---|---|---|---|---|
| Q3.1 | Kirigami.Theme colour sets | seven colour sets View, Window, Button, Selection, Tooltip, Complementary, Header, each a full role palette; a widget's colours come from the set of the region it sits in (`colorSet` + `inherit`) | `kirigami@81ada72b src/platform/platformtheme.h:67`, `:103`, `:473-483`; `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:86-108` | per-widget fixed fields (no colour-set concept) |
| Q3.2 | colour-set assignment per control | Button set: Button, CheckIndicator, SwitchIndicator, Slider; View: TextField, TextArea, SearchField, SpinBox, editable ComboBox, ScrollView, cards, dialogs, TreeView rows; Header: ToolBar header, MenuBar, page header, column/drawer separators; Window: TabBar, footer ToolBar, flat ToolButton; Tooltip/Complementary: ToolTip; Complementary: passive notifications; Selection: focused RoundButton | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Button.qml:17-18`; `ToolButton.qml:18-19`; `TextField.qml:19-20`; `ComboBox.qml:19-20`; `ToolBar.qml:27-28`; `MenuBar.qml:15-16`; `TabBar.qml:18-19`; `ToolTip.qml:16-18`; `RoundButton.qml:16-17`; `kirigami@81ada72b src/templates/AbstractCard.qml:92-93`; `src/controls/private/PassiveNotificationsManager.qml:91` | none |
| Q3.3 | colour group (inactive window) | whole palette switches to the Inactive colour group when the window is not active | `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:267-284` | `defaults.selection_inactive_background`, `window.inactive_title_bar_*` (partial) |
| Q3.4 | colour group (disabled) | a disabled item takes the Disabled colour group of its colour set (all roles) | `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:273-275` | `defaults.disabled_text_color`, per-widget `disabled_*` (partial) |
| Q3.5 | Theme.activeTextColor | role (KColorScheme ActiveText); used as the Information colour by InlineMessage and Badge | `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:292`; `kirigami@81ada72b src/controls/InlineMessage.qml:30`; `src/controls/Badge.qml:61` | `defaults.info_color` (partial) |
| Q3.6 | Theme.activeBackgroundColor | role (KColorScheme ActiveBackground) | `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:303`; `kirigami@81ada72b src/platform/platformtheme.h:270-280` | none |
| Q3.7 | Theme.linkBackgroundColor | role (KColorScheme LinkBackground) | `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:304`; `kirigami@81ada72b src/platform/platformtheme.h:282-293` | `link.background_color` (partial) |
| Q3.8 | Theme.visitedLinkBackgroundColor | role (KColorScheme VisitedBackground) | `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:305` | none |
| Q3.9 | Theme.negative/neutral/positiveBackgroundColor | tinted status backgrounds (KColorScheme Negative/Neutral/PositiveBackground), separate from the status text colours | `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:306-308` | `defaults.{danger,warning,success}_color` (one colour per status) |
| Q3.10 | Theme.negative/neutral/positiveTextColor | status foregrounds per colour set (used for InlineMessage/Badge borders, misspellings) | `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:295-297` | `defaults.{danger,warning,success}_color` (not per colour set) |
| Q3.11 | Theme.hoverColor | decoration colour per colour set (KColorScheme HoverColor), used as a border/outline colour (chip hover border, switch handle border) | `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:311`; `kirigami@81ada72b src/controls/private/DefaultChipBackground.qml:48`; `qqc2-desktop-style@305eb2c4 org.kde.desktop/private/SwitchIndicator.qml:30` | per-widget `hover_*` backgrounds (no hover decoration colour) |
| Q3.12 | Theme.focusColor | decoration colour per colour set (KColorScheme FocusColor); used as menu item highlight fill/border | `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:312`; `org.kde.desktop/MenuItem.qml:124-125` | `defaults.focus_ring_color` (single, not per set) |
| Q3.13 | Theme.alternateBackgroundColor + StyleHints.useAlternateBackgroundColor | alternate background per colour set; alternating rows only when the view opts in | `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:301`; `org.kde.desktop/private/DefaultListItemBackground.qml:21-28` | `list.alternate_row_background` (list only) |
| Q3.14 | frameContrast / lightFrameContrast | contrast factor for frames and separators (default 0.2; Plasma: read kdeglobals `[KDE] frameContrast`, clamped 0–1, `kcs:529-539`; the reader already reads it, `native-theme/src/kde/colors.rs:433`, but stores only finished colours); light = half | `kirigami@81ada72b src/platform/platformtheme.cpp:68`, `:757-766`; `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:313` | none |
| Q3.15 | derived frame colour | frames, popups, menus, tooltips, dialogs, cards, chips, separators, switch/range-slider handles use border = mix(set background, set text, frameContrast), computed in the colour set they sit in | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Frame.qml:25`; `Menu.qml:133`; `Popup.qml:50`; `ToolTip.qml:115`; `kirigami@81ada72b src/primitives/Separator.qml:65-69`; `src/controls/private/DefaultCardBackground.qml:70` | per-widget `border.color` (stored values; no derivation rule) |
| Q3.16 | derived state tints | hover/pressed/checked fills are tints of the highlight over the background at fixed alphas (cards 0.1/0.3, chips 0.2/0.3, nav tabs 0.2/0.3; menu item = focusColor at alpha 0.3) | `kirigami@81ada72b src/controls/private/DefaultCardBackground.qml:48-58`; `src/controls/private/DefaultChipBackground.qml:19-25`; `src/controls/NavigationTabButton.qml:23-24`; `qqc2-desktop-style@305eb2c4 org.kde.desktop/MenuItem.qml:124` | per-widget `hover_background` etc. (values only) |
| Q3.17 | icon recolouring | symbolic/mask icons are tinted with textColor, or highlightedTextColor when `selected`; Plasma recolours theme icons with KIconColors(text colour) | `kirigami@81ada72b src/primitives/icon.cpp:224`, `:593`; `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:245-258` | none |
| Q3.18 | icon states | active icon: highlight amount 0.7 (gamma lift); disabled icon: desaturated and at opacity 0.5 | `kirigami@81ada72b src/primitives/icon.cpp:229-230`; `src/primitives/shaders/icon.frag:32`, `:42`, `:46` | `defaults.disabled_opacity` (icon desaturation missing) |
| Q3.19 | icon source change | cross-fade between old and new icon, longDuration InOutCubic | `kirigami@81ada72b src/primitives/icon.cpp:58-69`, `:360-367` | none |
| Q3.20 | shadow API (ShadowedRectangle) | one shadow per surface with `size` (blur), `xOffset`, `yOffset`, `color`; per-corner radii (`corners.*Radius`); border width/colour; no spread, one layer | `kirigami@81ada72b src/primitives/shadowedrectangle.h:22-97`, `:153-236`; `src/primitives/shaders/shadowedrectangle.frag:24-45` | `border.shadow_enabled` + `defaults.shadow_color` |
| Q3.21 | shadow values per surface | menu (0,2,8, 0.3), popup (0,4,8, 0.3), tooltip (0,4,16, 0.2), qqc2 & Kirigami dialog (size radius×2, y 1, 0.3), search popup (0,2,8, 0.3), card (size gridUnit, y 2, 0.05) + lift layer, passive notification (size gridUnit/2, y 2, 0.4), RoundButton/RangeSlider hard 1 px shadows 0.15, drawer/swipe EdgeShadow gradient | rows in Tables A/B (Menu.qml:136-139, Popup.qml:53-56, ToolTip.qml:107-110, Dialog.qml:77-81, DefaultCardBackground.qml:90-109, PassiveNotificationsManager.qml:264-268, EdgeShadow.qml:42-59) | `defaults.shadow_color` (one colour, no geometry) |
| Q3.22 | modal dim | scrim colour/alpha differs by surface: dialogs black 0.3, modal drawers 0.35, OverlaySheet 0.5; dim fades longDuration InOutQuad | `qqc2-desktop-style@305eb2c4 org.kde.desktop/Dialog.qml:35-46`; `kirigami@81ada72b src/dialogs/Dialog.qml:314-321`; `src/templates/OverlayDrawer.qml:265-266`; `src/templates/OverlaySheet.qml:100-107` | none |
| Q3.23 | Units durations | four animation durations veryShort/short/long/veryLong (base 50/100/200/400 ms); every Kirigami/qqc2 animation picks one | `kirigami@81ada72b src/platform/units.cpp:40-43` | none |
| Q3.24 | Plasma duration derivation | Plasma: longDuration read from plasmarc [Units] longDuration (default 200) × kdeglobals [KDE] AnimationDurationFactor, min 1; veryShort = long/4, short = long/2, veryLong = long×2 | read: plasmarc `[Units] longDuration`, kdeglobals `[KDE] AnimationDurationFactor` — `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktopunits.cpp:20`, `:41-54`; `kirigami-plasmadesktop-integration/animationspeedprovider.cpp:40-47` | `AccessibilityPreferences.reduce_motion` (bool; continuous factor missing) |
| Q3.25 | animation-off behaviour | factor 0 → durations 1 ms; many Behaviors disable themselves when the duration is 0; BusyIndicator stops when longDuration ≤ 1 | `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktopunits.cpp:47-49`; `org.kde.desktop/BusyIndicator.qml:35-38`; `private/SwitchIndicator.qml:81`, `:88` | `AccessibilityPreferences.reduce_motion` (partial) |
| Q3.26 | easing curves | easings in use: InOutQuad (menus, popups, dialogs, drawers' secondary motion, scrollbars), OutCubic (tooltips, StackView, notifications), InCubic (switch colour, removals), InOutCubic (icons, PageRow, swipe), OutQuad/InQuad (drawer open/close), OutExpo (ColumnView scroll), SmoothedAnimation by velocity (qqc2 Drawer velocity 5, SwipeDelegate) | cited per row in Tables A/B; `kirigami@81ada72b src/layouts/columnview.cpp:462`; `qqc2-desktop-style@305eb2c4 org.kde.desktop/Drawer.qml:45-54` | none |
| Q3.27 | humanMoment | 2000 ms "too long" threshold, not scaled by animation speed (search debounce, delayed spinners) | `kirigami@81ada72b src/platform/units.cpp:44`; `src/platform/units.h:240-305` | none |
| Q3.28 | toolTipDelay usage | every tooltip (qqc2 ToolTip, item delegates, drawer handle, nav tabs, action fields) uses Units.toolTipDelay (700); tablet mode uses mousePressAndHoldInterval instead | `kirigami@81ada72b src/platform/units.cpp:45`; `qqc2-desktop-style@305eb2c4 org.kde.desktop/ToolTip.qml:44` | `tooltip.show_delay_ms` (touch variant missing) |
| Q3.29 | gridUnit | base size unit 18 that most widget sizes multiply (drawer widths, dialog sizes, column widths, switch height, BusyIndicator, tooltip max width ×14) | `kirigami@81ada72b src/platform/units.cpp:36`; `qqc2-desktop-style@305eb2c4 org.kde.desktop/ToolTip.qml:78` | none |
| Q3.30 | focus indication | no single focus ring: Breeze `focusrect` around checkbox/radio labels, Breeze-internal focus for buttons/fields/tabs (StyleItem sets State_KeyboardFocusChange only for keyboard focus reasons; tabs drop HasFocus otherwise), NavigationTabButton 3 px outline at −2 px (highlight 0.5), LinkButton bold text, list rows reuse the hover look, Switch handle border turns hoverColor | `qqc2-desktop-style@305eb2c4 org.kde.desktop/private/FocusRect.qml:12-14`; `CheckBox.qml:92-106`; `plugin/kquickstyleitem.cpp:884-889`, `:1775-1779`; `kirigami@81ada72b src/controls/NavigationTabButton.qml:61-71`; `src/controls/LinkButton.qml:11`; `qqc2-desktop-style@305eb2c4 org.kde.desktop/private/DefaultListItemBackground.qml:54` | `defaults.focus_ring_{color,width_px,offset_px}` (one ring style) |
| Q3.31 | touch / tablet variants | `Settings.tabletMode` / `hasTransientTouchInput` / `isMobile` change visuals: bigger menu rows and icons, overlay scrollbars, tooltips on press, no hover on text fields, selection handles, drawer handles, sheet drag bar | `qqc2-desktop-style@305eb2c4 org.kde.desktop/MenuItem.qml:34`, `:64`; `ScrollBar.qml:24`; `ToolTip.qml:39-44`; `TextField.qml:46`; `kirigami@81ada72b src/templates/OverlaySheet.qml:264-274` | none |
| Q3.32 | text rendering | NativeTextRendering at device pixel ratio 1, QtTextRendering otherwise | `qqc2-desktop-style@305eb2c4 kirigami-plasmadesktop-integration/plasmadesktoptheme.cpp:58-62` | none |
| Q3.33 | disabled text opacity | QML labels of check/radio/switch dim to 0.6; combo/search editable text 0.3; drawer items 0.75 — three different disabled opacities | `qqc2-desktop-style@305eb2c4 org.kde.desktop/CheckBox.qml:81`; `ComboBox.qml:85`; `SearchField.qml:114`; `kirigami@81ada72b src/controls/private/ContextDrawerActionItem.qml:40` | per-widget `disabled_opacity` (combo/check have it; switch label, drawer items do not) |
| Q3.34 | Breeze-delegated parts (scope note) | elementTypes painted by Breeze through `StyleItem`: `button`, `toolbutton`, `delaybutton`, `checkbox`, `radiobutton`, `combobox`, `spinbox`, `slider`, `dial`, `progressbar`, `scrollbar`, `edit`, `frame`, `tab`, `tabframe` (metrics only), `header`, `item`, `itemrow`, `itembranchindicator`, `splitter`, `focusrect`; metrics/hints read: `checkboxlabelspacing`, `radiobuttonlabelspacing`, `indicatorwidth`, `exclusiveindicatorwidth`, `menuhmargin`, `menuvmargin`, `treeviewindentation`, `tabOverlap`, `layout{top,left,right,bottom}margin`, styleHints `scrollToClickPosition`, `dialogButtonsHaveIcons` | `qqc2-desktop-style@305eb2c4 plugin/kquickstyleitem.cpp:1586-1656`, `:1739-1925` | out of scope (Breeze QStyle survey) |

Checked in the Quick survey with no gap: qqc2 `Label`, Kirigami `SelectableLabel`, qqc2 `Container`/`Control`; `Units.toolTipDelay` and `Units.cornerRadius` map onto `tooltip.show_delay_ms` and `defaults.border.corner_radius_px` (only their touch and derived uses are rows).

---

## Data and source discrepancies noticed (not model gaps)

These are fields the model has whose KDE source in `platform-facts.md` or the reader disagrees with the Breeze source read above; each needs checking before values are collected.

| ID | Field | Discrepancy | Source |
|---|---|---|---|
| D1 | `menu.hover_background`, `menu.hover_text_color` | Facts: `[Colors:Selection]` background/foreground. Breeze: `focusColor` (View `DecorationFocus`) at α `Blend_Value` + outline, text stays `WindowText` (the solid colour is the *pressed* item) | `st:6024-6047`, `st:6159-6160` |
| D2 | `window.title_bar_background`, `inactive_*` | Reader reads `[WM]`; KWin uses `[Colors:Header]` / `[Colors:Header][Inactive]` when the scheme has them (K1.13) | `kwin src/decorations/decorationpalette.cpp:126-160` |
| D3 | `defaults.selection_inactive_background` | Facts: "(none) — unchanged on focus loss". KColorScheme (Breeze default `ChangeSelectionColor=true`) tints the Window background with the Selection colour for the Inactive group | `kcs:303-321`, `kcs:386-389`; `colors` BreezeLight `:18-19` |
| D4 | `list.header_background` | Reader: `[Colors:Header]`. Breeze's header section fills `QPalette::Button` | `st:6832-6845` |
| D5 | `combo_box.border.shadow_enabled` | Facts: "no shadow". A non-editable combo is a button frame with the button's micro-shadow | `st:7856-7871`; `hp:729-732` |
| D6 | `list.hover_background` | Facts: "DecorationHover blend". Breeze: `Highlight` at α `Blend_Value` | `st:4875-4880` |
| D7 | `defaults.disabled_text_color` | Reader: `[Colors:View] ForegroundInactive`; facts: Disabled Window `ForegroundNormal` via `[ColorEffects:Disabled]` | `native-theme/src/kde/colors.rs:493`; `kcs:21-56` |
| D8 | `input.caret_color` | Facts: `DecorationFocus`. Qt draws the line-edit caret in the palette's text colour (to verify in qtbase `qwidgetlinecontrol.cpp`) | measure: probe app |
| D9 | `defaults.focus_ring_*` | Facts: 1.001 px stroke, 2 px margin. Breeze's focus frame is a 2 px *filled* ring at `Highlight` α 0.3 (K3.20) | `st:6779-6786` |

---

## Cross-check (pass 2, 2026-10-28)

Pass 2 checked this map against the other five surveys (`docs/todo_v0.7.1_model-completeness-gap-map.md` §6.1–§6.3): the controls they list that this file did not (A), every capability C01–C28 (B), and every "(none)" cell of the KDE column of `platform-facts.md` (C). New rows carry a "†" after their ID: 15 in §4.1, 16 in §4.2, 11 in §4.3.

Sources read in this pass beyond the table at the top. The KDE clones are the same commits; KWin's sparse clone gained `src/outline.cpp`, `src/outputconfigurationstore.cpp`, `src/inputmethod.cpp`, `src/qml/outline/` and the plugins `dialogparent`, `frozenapp`, `fullscreen`, `systembell`, `shakecursor`, `startupfeedback` (and, read for enablement only, `sheet`, `invert`, `colorblindnesscorrection`, `translucency`, `slidingnotifications`, `blendchanges`, `screenedge`, `highlightwindow`, `zoom`, `slideback`), all at `625e3132`. Fetched files, read 2026-10-28: qtbase at tag `v6.10.1` (`src/widgets/…`, `src/gui/kernel/qplatformtheme.cpp`, `qplatformintegration.cpp`), cited as "qtbase v6.10.1 `<file>:<line>`"; KDE/kio @ `eec0132888fe2a6ede4b88c8b9689564a88d19fb`, KDE/kcompletion @ `2bc74a5bf8a04faa8f44815fb65dc17f0eb1208d`, KDE/ktexteditor @ `df8e3ad8b6fcbdb5278696640e9cdfe7507cb65a`, KDE/sonnet @ `5442e3b4cac09675133f345ff8ccecdfba9d58d9`, KDE/xdg-desktop-portal-kde @ `936476a53424f11b08c3ee0e290de0154220b531` (the `master` heads that day). They are re-pinned with the rest (§6.4).

### A. Controls other surveys list

Decision: 1 = KDE draws it as a standalone control (row in §4.2); 2 = KDE draws it inside an element this map already covers (rows named; rows added where they missed something); 3 = KDE has no such control.

| Control | Decision | Rows added, or reason and source |
|---|---|---|
| search field | 2 | Quick: Q1.15–Q1.17 (qqc2 and Kirigami SearchField, its popup); Qt Widgets has no search-field class (a QLineEdit with a clear button: K1.43, K1.46†) — `kwa src/` widget list, qtbase |
| date picker | 1 | K2.4 (the calendar) and K2.23† (KDateComboBox, KDatePickerPopup, QDateEdit's calendar popup, KDateTable cell colours) |
| pull-to-refresh | 1 | Q2.54† (Kirigami ScrollablePage's PullDownIndicator); its arc is Q1.121 |
| split button | 2 | K1.155 (QToolButton `MenuButtonPopup`: split sub-control with a separator line); Qt and KF have no split push button |
| carousel | 3 | No carousel in KF6: none in the `kwa src/` widget list, Kirigami's `src/controls/`, or qqc2-desktop-style's control list (`qqc2-desktop-style@305eb2c4 CMakeLists.txt:110-168`); the nearest part, PageIndicator, is Q2.11 |
| colour picker | 1 | K2.25† (QColorDialog shown through the KDE platform theme; KColorCombo); K2.20 is the colour button |
| find bar | 1 | K2.30† (KTextEditor's search bar: match / no-match field backgrounds, search and replace highlights) |
| menu button | 2 | K1.27 (QPushButton with a menu), Q1.8 (qqc2 `menu` arrow flag), K1.155 (tool-button menu arrows) |
| navigation drawer/pane | 2 | Q1.53–Q1.66 (Kirigami OverlayDrawer, GlobalDrawer incl. collapsed rail, ContextDrawer), Q1.67 (qqc2 Drawer) |
| password field | 2 | K1.43, Q1.19; added K1.48† (mask character and delay, KPasswordLineEdit's reveal action) |
| time picker | 1 | K2.24† (KTimeComboBox, QTimeEdit, KDateTimeEdit) |
| token field | 3 | No token field in KF6 (`kwa src/` widget list, Kirigami `src/controls/`); tags are drawn as Kirigami Chips (Q2.22–Q2.24) |
| view switcher | 2 | Q2.33–Q2.36 (Kirigami NavigationTabBar), K2.14 (KPageView's Tabbed and List faces) |
| access key badge | 3 | KDE marks access keys by underlined mnemonics, not badges: K3.44 (Breeze `MnemonicsMode`), Q1.10 (Kirigami mnemonics, shown while Alt is held) |
| action sheet | 2 | Q1.117 (Kirigami MenuDialog: a list of actions in a dialog) |
| autocomplete | 1 | K2.29† (KCompletionBox, inline completion); Quick: Q1.16 (SearchField popup) |
| back button | 2 | Q1.73 (Kirigami global toolbar's back / forward buttons) |
| background extension | 3 | Liquid Glass content mirroring under a floating sidebar; Breeze and Kirigami sidebars are opaque panels (K1.140, Q1.53–Q1.54) |
| bottom accessory | 3 | iOS 26 tab-bar accessory; Kirigami's bottom bar (NavigationTabBar, Q2.33) has no accessory slot (`kirigami@81ada72b src/templates/NavigationTabBar.qml`) |
| collection view | 2 | K1.180 (QListView icon mode), Q1.133 (Kirigami CardsLayout / CardsListView) |
| column browser | 2 | Q1.93–Q1.94 (Kirigami ColumnView); added K1.184† (QColumnView's arrows) |
| context menu preview | 3 | iOS's lifted preview; KDE context menus are plain menus ([menu] rows; `qqc2-desktop-style@305eb2c4 org.kde.desktop/private/TextFieldContextMenu.qml`) |
| drag and drop | 2 | K1.182 (drop indicator), K1.126 (tab tear line), Q2.43 (drag handle, drop animation), K3.71 (drag threshold), K1.133† (drag-hover tab switch), K3.65† (hand cursors) |
| edit menu | 1 | Q2.53† (the touch Cut / Copy / Paste bar); the desktop text context menu is a menu |
| file chooser | 1 | K2.26† (KFileWidget), with K1.142† (Places panel) and K2.27† (URL navigator) |
| font chooser | 1 | K2.28† (KFontRequester, KFontChooser; QFontDialog stays Qt's) |
| full-screen dialog | 1 | Q2.58† (Kirigami `pushDialogLayer`: full-window layer on narrow mobile windows, inset dialog on wide ones, a window on the desktop); Q2.15 (layer transitions) |
| glass container | 3 | Liquid Glass; KDE has no glass material — its translucency is MenuOpacity and KWin blur (K3.10–K3.11) |
| ink toolbar | 3 | No inking control in Qt Widgets, KF6 or Kirigami (`kwa src/`, Kirigami `src/controls/` lists) |
| inspector/utility panel | 2 | K2.10 (dock widgets, Qt Widgets' tool panels), Q1.66 (Kirigami ContextDrawer) |
| keyboard | 3 | The on-screen keyboard is a separate input-method client KWin starts (`kwin src/inputmethod.cpp:867-900`); applications do not draw it |
| large content viewer hud | 3 | iOS accessibility HUD; Plasma has no counterpart (zoom / magnifier are KWin effects, K3.93) |
| list box | 2 | [list] rows K1.170–K1.185†, Q1.77–Q1.83 |
| loading indicator | 2 | K1.230, Q1.119–Q1.120 (icon spinners), Q2.32 (LoadingPlaceholder), K1.110 (busy progress bar) |
| media transport controls | 3 | No media-transport control in Qt Widgets, KF6 or Kirigami (same lists); Plasma's media controller is a shell applet |
| paste control | 3 | iOS's privacy paste button; Qt pastes without a permission control |
| picker wheel | 3 | qqc2-desktop-style ships no Tumbler (`CMakeLists.txt:110-168`), so a QtQuick Tumbler falls back to Qt's own style; Qt Widgets and KF6 have no wheel picker |
| rule editor | 3 | No predicate/rule editor in KF6 (`kwa src/` list) |
| scrim | 2 | Q3.22 (dim per surface), Q1.57, Q1.104, Q1.112, Q2.38 |
| scroll edge effect | 3 | No edge blur or fade in Breeze or Kirigami; the nearest is the separator that appears when content scrolls under a dialog header or footer (Q1.109, Q1.114) |
| scrollbar annotation | 3 | Breeze's `CC_ScrollBar` draws no marks (K1.90–K1.97); KTextEditor's own scroll-bar markers are editor-internal, outside the toolkit |
| swipe actions | 2 | Q1.84 (qqc2 SwipeDelegate), Q1.88–Q1.89 (Kirigami SwipeListItem) |
| teaching tip | 1 | K2.31† (QWhatsThis popup) and K1.84† (KToolTipWidget rich tooltip); Q2.48 (Kirigami ContextualHelpButton) |
| text ruler | 1 | K2.32† (KRuler) |
| tool button | 2 | K2.2 (whole control, pointing to K1.152–K1.157). The cross-check list names GN-2.42, which is GTK's text selection handles and magnifier; KDE's are Q1.14 (qqc2 MobileCursor) |
| toolbar flyout | 2 | K1.157, Q1.75 (toolbar overflow menus), Q2.53† (touch text actions bar) |
| touch bar | 3 | Mac hardware; no counterpart |
| tree view | 2 | K1.176 (branches, arrows, indentation), Q1.85; added K1.185† (expand animation) |
| writing tools | 3 | Apple Intelligence; no counterpart in KF6 |
| zoom button | 3 | WinUI SemanticZoom; no semantic-zoom control in Qt Widgets, KF6 or Kirigami (same lists) |

Tally: 1 — 12 controls (rows K2.23†–K2.32†, Q2.53†, Q2.54†, Q2.58†); 2 — 19; 3 — 19. Q2.55†–Q2.57† (Kirigami Forms, About page) were found on the way, under controls KDE already had rows for (preference rows / boxed list, about dialog).

### B. Capabilities

| Capability | Rows added, or "no further row" and why |
|---|---|
| C01 shadow geometry | K2.31† (QWhatsThis's hand-drawn shadow, off under KDE). Every other KDE surface with a shadow has its row (K3.1–K3.5, Q3.20–Q3.21) |
| C02 elevation | No further row: Breeze and Kirigami have no elevation scale; each surface states its own shadow (K3.1–K3.5, Q3.21) |
| C03 materials | No further row beyond K1.19† (the quick-tile outline uses the Plasma theme's translucent SVG): KDE's translucency is MenuOpacity and KWin blur (K1.65, K3.10–K3.14); the decoration fills the frame and title bar with the scheme's colours as given, with no material or blur of its own (`deco:584-606`) |
| C04 colour composition | K1.142†, K1.184†, K2.27†, K2.31†, Q2.55†, Q2.56† |
| C05 colour roles | K1.49† (hard-coded misspelling red), K2.23† (hard-coded `darkRed`, `LinkVisited` as today), K2.30† (status backgrounds in use), K1.260† (`BrightText` as hover) |
| C06 variant axes | K3.15† (modal dialog's parent dimmed), K3.16† (not-responding window); K2.23† (high-contrast branch) |
| C07 states | K1.46†, K1.133†, K1.142†, K1.260†, K2.23†, K2.27†, K2.30†, Q2.55†, Q2.56† |
| C08 state layers | No further row: KDE's hover / pressed fills are fixed alpha tints, already Q3.16 and the per-widget rows; Q2.55†'s hover / pressed opacities are tagged C04, C07 |
| C09 variants | K1.115† (capacity bar), Q2.56† (cards vs flat forms) |
| C10 size variants | K3.84† (per-output display scale); Q2.58† (dialog layer by window width) |
| C11 shape | Q2.55† (first and last entries' outer corners only), K1.58† |
| C12 borders | K2.29† (1 px plain box frame), K2.31† (double frame) |
| C13 parts | K1.46†, K1.48†, K1.58†, K1.115†, K1.142†, K1.184†, K1.224†, all new §4.2 rows |
| C14 motion | K1.19†, K1.46†, K1.133†, K1.185†, K3.15†, K3.16†, K3.38†, K3.39†, K3.96†, Q2.54†, Q2.55†, Q2.56† |
| C15 focus | No further row: K3.20–K3.24, Q3.30 and the per-widget focus rows cover every focus indication found; the new controls draw focus through those |
| C16 caret | K1.47† (block caret, overwrite caret), K3.48† (pre-edit caret). K3.42's and K3.43's "to verify" now have their source: caret width = the platform hint `TextCursorWidth`, Qt's default 1 (the KDE platform theme does not set it), `qcommonstyle.cpp:4828-4830`, `qplatformtheme.cpp:648-649`; blink while text is selected = 1 off macOS, `qcommonstyle.cpp:5138-5143` |
| C17 selection, highlights | K1.49† (misspelling underline), K2.30† (find highlight), K3.48† (pre-edit) |
| C18 typography | No further row beyond the fonts inside new rows (K2.23† bold header and today, K2.32† 8 pt labels, K2.27† bold crumb): every kdeglobals font role is read (`native-theme/src/kde/fonts.rs:157-204`) or in K3.47 |
| C19 icons | K1.46† (icon modes of side icons), K1.48†, K1.224† (message-box standard icons), Q2.53† |
| C20 window chrome | K1.19† (quick-tile outline), K3.15†, K3.16†, K3.39† |
| C21 timings | K1.83† (tooltip wake-up, fall-asleep, display time), K1.84† (500 ms hide delay), K1.133†, K3.76† (auto-repeat, press-and-hold), K3.66†, Q2.54† (refresh delay) |
| C22 pointer | K3.65† (cursor per element, auto-hide), K3.66† (launch feedback), K3.96† (shake cursor) |
| C23 accessibility | K3.95† (visual bell), K3.96† (shake cursor). Plasma has no contrast preference to read: the KDE portal's appearance keys are `color-scheme`, `accent-color` and `reduced-motion` only (KDE/xdg-desktop-portal-kde @ `936476a5` `src/settings.cpp:211-213`), so the high-contrast branches in KF widgets (`kwa highcontrasthelper.cpp:15-29`) do not run on Plasma — K3.91 stands |
| C24 whole controls | The 16 new §4.2 rows (A) |
| C25 reader coverage | K1.19†, K3.15†, K3.16†, K3.66†, K3.84†, K3.95†, K3.96† |
| C26 layout | K1.83†, K1.84†, K2.27†, K2.29†, Q2.53†, Q2.55†, Q2.56†, Q2.58† |
| C27 gradients | K2.27† (clipped crumb names fade out through a gradient) |
| C28 colour precision | No further row: KDE colour schemes store 8-bit `R,G,B` integers (`colors` BreezeLight `:31`) and KColorScheme reads them as such; no wide-gamut or HDR colour role exists to carry |

### C. "(none)" cells of the KDE column

(a) KDE does not draw it, the cell is right; (b) KDE draws it and no document states it, the row named holds it (or "†" marks a row added here).

| platform-facts line | a/b | Row, or reason |
|---|---|---|
| 718 Scale factor (Wayland) | b | K3.84† (per-output scale in `kwinoutputconfig.json`) |
| 1057 `shadow_color` | b | K3.1, K3.2 (breezerc `ShadowColor`, read) |
| 1061 `selection_inactive_background` | b | K3.40, K1.173 (D3: the cell is wrong for Breeze's default `ChangeSelectionColor`) |
| 1117 `border.corner_radius_lg` | a | One radius step: Breeze `Frame_FrameRadius` (`mt:42`), Kirigami `cornerRadius` (`kirigami@81ada72b src/platform/units.cpp:46`); the window's larger corner is K1.10 |
| 1127 `text_scaling_factor` (Wayland) | a | No text-only scale on Wayland; the font sizes are the text size (read) |
| 1129 `high_contrast` | a | No contrast setting (B, C23; K3.91) |
| 1130 `reduce_transparency` | a | No setting; transparency is MenuOpacity and the blur effect (K3.92) |
| 1170 window `border.padding_horizontal` | a | The window has no padding; content margins are layout rows (K1.210, Q1.98) |
| 1171 window `border.padding_vertical` | a | As 1170 |
| 1183 button `min_height` | a | No minimum; height = content + `Button_MarginWidth` and frame metrics |
| 1212 input `min_height` | a | No minimum on Qt Widgets; the Quick TextArea background's implicit size is Q1.12 |
| 1216 input `border.shadow_enabled` | a | `renderFrame` draws no shadow; Breeze's only control shadows are the button frame and slider / dial handles (`hp:728-732`, `hp:1178-1180`) |
| 1236 checkbox `border.shadow_enabled` | a | As 1216 (`renderCheckBox*`, `hp:831-981`) |
| 1237 checkbox `border.padding_horizontal` | b | K1.58† (mark inset in the box), K1.53 (box inset in the cell) |
| 1238 checkbox `border.padding_vertical` | b | K1.58†, K1.53 |
| 1260 menu `row_height` | a | Content-sized; the derivation is K1.71 (`st:3607-3718`); Quick item padding Q1.28 |
| 1263 menu `border.color` | b | K1.64, K1.65 (the menu frame's outline, translucent variant) |
| 1264 menu `border.line_width` | b | K1.64, K1.65 |
| 1265 menu `border.corner_radius` | b | K1.60, Q1.26: Breeze's highlighted item **is** rounded (`frameRadius`), so the cell's "items are rectangular" is wrong for KDE |
| 1266 menu `border.shadow_enabled` | b | K3.2, K1.74, Q1.23 |
| 1285 tooltip `max_width` | b | K1.83† (Qt: wraps only at the screen width), Q3.29 (Quick: gridUnit × 14) |
| 1310 scrollbar `overlay_mode` | b | Desktop: persistent (the cell is right); touch: Q1.41 (overlay thumb) |
| 1336 progress `min_width` | a | No minimum |
| 1338 progress `border.shadow_enabled` | a | `renderProgressBarGroove` draws no shadow (`hp:1204-1218`) |
| 1366 tab `active_outward_corner_radius` | a | One rounded rectangle, no outward corners (the cell's own source) |
| 1377 sidebar `border.corner_radius` | a | Side panels are rectangular (`hp:504-544`); Kirigami drawers too |
| 1378 sidebar `border.shadow_enabled` | b | Modal Kirigami drawers cast an edge shadow: Q1.56; inline panels none |
| 1390 toolbar `bar_height` | b | Qt Widgets content-sized; Quick implicit 40 (Q1.69) and the global toolbar (Q1.72) |
| 1397 toolbar `border.corner_radius` | a | `CE_ToolBar` paints nothing |
| 1399 toolbar `border.shadow_enabled` | a | As 1397 |
| 1412 status bar `border.corner_radius` | a | Rectangular bar, only a top line (K1.160) |
| 1413 status bar `border.shadow_enabled` | a | As 1412 |
| 1430 list `grid_color` | b | Qt's grid pen is the shade role `Mid` (`SH_Table_GridLineColor`, qtbase v6.10.1 `qcommonstyle.cpp:5145-5150`), computed from `[KDE] contrast`: K3.53 |
| 1431 list `row_height` | a | Content-sized (item margins K1.170); Quick padding Q1.79 |
| 1432 list `header_height` | a | Content-sized from metrics the cell cites; K1.179 |
| 1441 list `border.shadow_enabled` | a | The cell's own source |
| 1488 text `display` | a | Kirigami's largest heading is level 1 (Q1.3); no display style |
| 1506 layout `section_gap` | b | Q2.47 (FormLayout: largeSpacing × 2 above section titles), Q2.56† (Kirigami Forms: largeSpacing + smallSpacing between groups) |
| 1552 dialog `min_width` | a | Content-sized (Kirigami desktop dialog windows have a minimum width, Q2.58†) |
| 1553 dialog `max_width` | b | Q1.113 (Kirigami Dialog: window − largeSpacing × 2) |
| 1554 dialog `min_height` | a | Content-sized |
| 1555 dialog `max_height` | b | Q1.113 |
| 1568 dialog `icon_size` | b | K1.224† (message box: `PM_MessageBoxIconSize`), Q1.116 (PromptDialog: iconSizes.huge) |
| 1600 spinner `min_diameter` | a | No minimum; the spinner is an icon (K1.230, Q1.119–Q1.120) |
| 1601 spinner `stroke_width` | a | Icon-based, no stroke; the only stroked arc is the pull-to-refresh indicator (Q1.121, Q2.54†) |
| 1616 combo `min_height` | a | Content-sized |
| 1617 combo `min_width` | b | Quick: content min width 75 (Q1.124); Qt Widgets content-sized |
| 1622 combo `border.shadow_enabled` | b | K1.244 (D5): a non-editable combo draws the button's shadow, so the cell is wrong |
| 1644 segmented `border.shadow_enabled` | a | KDE has no segmented control |
| 1675 expander `header_height` | a | Content-sized |
| 1677 expander `border.padding_horizontal` | a | KCollapsibleGroupBox has no header padding; its content is indented by `PM_IndicatorWidth` and set below the header (`kwa kcollapsiblegroupbox.cpp:314`) |
| 1678 expander `border.padding_vertical` | a | As 1677 |
| 1680 expander `border.color` | a | No frame (`kwa kcollapsiblegroupbox.cpp:153-182`) |
| 1681 expander `border.line_width` | a | As 1680 |
| 1682 expander `border.shadow_enabled` | a | As 1680 |
| 1687 expander `hover_background` | a | The cell's own source |
| 1697 link `background_color` | a | Inline text; KUrlLabel too changes only the text colour (K1.260†) |
| 1698 link `hover_background` | a | As 1697 |

Tally: a — 38; b — 20 (2 rows added for them: K1.58†, K3.84†; K1.83† and K1.224†, added for A/B, also answer cells).

### Found wrong in pass 1 (not rewritten here)

- `platform-facts.md:1265` (menu items rectangular) and `:1622` (combo, no shadow) disagree with Breeze's source (K1.60, K1.244 / D5) — for `docs/todo.md`'s list of values against their sources.
- The Link note above cites "Q1.154–Q1.156" for the Quick link rows; they are Q1.134–Q1.136.
- In `tags-kde.tsv`, several §4.2 rows sit under a control name that is not theirs, which hid them from the cross-check: K2.7 (command link) under "button group/toggle group", K2.15 (KTitleWidget) under "status page/placeholder", K2.16 (KMultiTabBar) under "navigation rail", K2.17 (splitter collapser) under "button group/toggle group", K2.22 (wizard) under "about dialog", Q2.43 (drag handle) under "icon button"; K2.2 (tool button) has no tag line at all, so "tool button" came out as missing for KDE.
- The cross-check list's "tool button" entry for GNOME is GN-2.42, which is GTK's text selection handles and magnifier.
- The Kirigami Forms module (`src/forms/`, cards and flat styles: KDE's boxed list) and the KIO file widgets were not surveyed in pass 1 (Q2.55†–Q2.57†, K1.142†, K2.26†–K2.27†). kirigami-addons (FormCard, date and time pickers), which many KDE applications use, is outside this map's scope as written; whether it belongs is a question for the rationale.

## Seed follow-up (2026-10-28)

Rows for the four seed groups no row covered after pass 2 (G12, G19, G24, G27; the main
map's §5). The source abbreviations are defined in
`todo_v0.7.1_model-completeness-gap-map-seeds.md`, *Seed follow-up sources*. CAPS and SRC
are the row's capability tags (main map §4.3) and source kinds.

### Code editor and rich text (G12, G27)

**Code editor.** KTextEditor (KF 6.31.0, the editor part of Kate, KWrite and KDevelop) draws with a
KSyntaxHighlighting theme. Breeze ships two, *Breeze Light* and *Breeze Dark*
(`ksh/data/themes/breeze-light.theme`, `breeze-dark.theme`). The search bar and its highlights are
K2.30†; the misspelling underline K1.49†.

| ID | § | Model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| K-SF01† | 4.2 | code editor | Which theme draws | "Auto Color Theme Selection" is on by default; the editor then takes the KSyntaxHighlighting theme whose `BackgroundColor` equals the palette's `Base` (and, among those, whose `TextSelection` equals `Highlight`), else Breeze Dark when `Base`'s lightness < 128, else Breeze Light. With Breeze's colour schemes this lands on the Breeze themes; with any other scheme on Breeze Light/Dark unless a theme matches its colours | P: `kte/src/utils/kateconfig.cpp:691`, `:893-904`; `ksh/src/lib/repository.cpp:176-210`; R: `[Colors:View] BackgroundNormal`, `[Colors:Selection] BackgroundNormal` (kdeglobals) decide the match | none | C05, C25 | R, P |
| K-SF02† | 4.2 | code editor | Editor fill and selection | `BackgroundColor`, `TextSelection` of the editor theme (not the colour scheme's) | P: `ksh/data/themes/breeze-light.theme:447`, `:466`; `breeze-dark.theme:441-470`; `kte/src/utils/kateconfig.cpp:908`, `:911` | `input.background_color`, `defaults.text_selection_background` (scheme values, not the theme's) | C05, C17, C24 | P |
| K-SF03† | 4.2 | code editor | Current line | `CurrentLine` fill across the line; on a line carrying marks it is mixed 90/10 with the marks' colours | P: `breeze-light.theme:450`; `kte/src/render/katerenderer.cpp:185-237`; `kte/src/utils/kateconfig.cpp:914` | none | C04, C17, C24 | P |
| K-SF04† | 4.2 | code editor | Gutter (icon border, line numbers) | `IconBorder` fill, `LineNumbers`, `CurrentLineNumber`, `Separator` between gutter and text, `CodeFolding` (folding markers / range highlight), mark colours `MarkBookmark`, `MarkBreakpointActive`, `…Reached`, `…Disabled`, `MarkExecution`, `MarkWarning`, `MarkError` | P: `breeze-light.theme:448`, `:451`, `:453-461`, `:467`; `kte/src/view/kateviewhelpers.cpp:2049-2135`; `kte/src/utils/kateconfig.cpp:929-941`, `:959-963` | none | C05, C13, C24 | P |
| K-SF05† | 4.2 | code editor | Modified / saved line markers | `ModifiedLines`, `SavedLines` bars in the gutter, recoloured in HSV against the gutter fill (saturation 255 and 100, value from the background) | P: `breeze-light.theme:462`, `:464`; `kte/src/view/kateviewhelpers.cpp:595-602`; `kte/src/utils/kateconfig.cpp:947`, `:950` | none | C04, C13, C24 | P |
| K-SF06† | 4.2 | code editor | Invisibles and indentation guides | `TabMarker` colour for the tab chevrons (pen max(1, space width / 10)), the space dots (round, size from the "marker size" setting) and the no-break-space mark; `IndentationLine` for indentation guides | P: `breeze-light.theme:452`, `:469`; `kte/src/render/katerenderer.cpp:241-300`, `:391`, `:1168` | none | C13, C24 | P |
| K-SF07† | 4.2 | code editor | Bracket matching | `BracketMatching` fill of the matched pair; an optional animation (off by default) | P: `breeze-light.theme:449`; `kte/src/utils/kateconfig.cpp:784`, `:917` | none | C14, C17, C24 | P |
| K-SF08† | 4.2 | code editor | Search and replace highlights | `SearchHighlight`, `ReplaceHighlight`: the theme location of the two colours K2.30† names | P: `breeze-light.theme:463`, `:465`; `kte/src/utils/kateconfig.cpp:953-956` | none | C17, C24 | P |
| K-SF09† | 4.2 | code editor | Right margin (static word-wrap marker) | `WordWrapMarker` line at the wrap column, and the dynamic-wrap indent area filled with it in `Qt::Dense4Pattern` | P: `breeze-light.theme:474`; `kte/src/render/katerenderer.cpp:865`, `:901-902` | none | C13, C24 | P |
| K-SF10† | 4.2 | code editor | Syntax roles | 31 default text styles (`Normal`, `Keyword`, `Function`, `Variable`, `ControlFlow`, `Operator`, `BuiltIn`, `Extension`, `Preprocessor`, `Attribute`, `Char`, `SpecialChar`, `String`, `VerbatimString`, `SpecialString`, `Import`, `DataType`, `DecVal`, `BaseN`, `Float`, `Constant`, `Comment`, `Documentation`, `Annotation`, `CommentVar`, `RegionMarker`, `Information`, `Warning`, `Alert`, `Error`, `Others`), each with text colour, **selected** text colour, optional background, bold, italic, underline, strike-through; plus per-language `custom-styles` | P: `ksh/src/lib/theme.h:163-195`; `breeze-light.theme:11-148`, `:149-445`; `breeze-dark.theme:11-440` | none | C05, C18, C24 | P |
| K-SF11† | 4.2 | code editor | Spell-check and template placeholders | `SpellChecking` underline colour (K1.49† draws it in other widgets from the scheme); `TemplateBackground`, `TemplatePlaceholder`, `TemplateFocusedPlaceholder`, `TemplateReadOnlyPlaceholder` | P: `breeze-light.theme:468`, `:470-473`; `kte/src/utils/kateconfig.cpp:944`, `:966-972` | none | C05, C17, C24 | P |

**Rich text / Markdown.** Kirigami has no Markdown component (`grep -ri markdown` over
`kirigami@81ada72b src/` finds none); KDE applications render Markdown through Qt's
`QTextDocument` (Qt Widgets `Qt::MarkdownText`, Qt Quick `Text.MarkdownText`), whose look is Qt's,
from the palette KDE sets. qqc2-desktop-style's `Label` sets only `linkColor:
Kirigami.Theme.linkColor` (`qqc2@305eb2c4 org.kde.desktop/Label.qml:27`).

| ID | § | Model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| K-SF12† | 4.2 | rich text view | Headings H1–H6 | Bold, and a font-size adjustment of `4 − level` (+3 … −2), which maps to the factors 0.7, 0.8, 1.0, 1.2, 1.5, 2, 2.4 of the base font | P: `qt/qtextmarkdownimporter.cpp:231-238`; `qt/qtextformat.cpp:295-305` | `text_scale.*` (four roles, not this ladder) | C18, C24 | P |
| K-SF13† | 4.2 | rich text view | Inline code and code blocks | Font `QFontDatabase::systemFont(FixedFont)` (KDE's fixed font), no fill, no border; code blocks also lose the paragraph margins, and fenced ones get non-breaking lines | P: `qt/qtextmarkdownimporter.cpp:115`, `:453-456`, `:638-648` | `defaults.mono_font` (font only; "no fill" is not expressible) | C18, C24 | P |
| K-SF14† | 4.2 | rich text view | Block quote | Left margin 40 px per depth, right margin 40 px; no bar and no fill (`qtextdocumentlayout.cpp` reads no `BlockQuoteLevel`) | P: `qt/qtextmarkdownimporter.cpp:34-37`, `:633-636`; `qt/qtextdocumentlayout.cpp` (no reference) | none | C24, C26 | P |
| K-SF15† | 4.2 | rich text view | List markers and task boxes | Disc, circle, square of size line spacing / 3, one space advance before the text, in the run's colour or the palette's `Text`; task items a square with a 2 px cross when checked | P: `qt/qtextdocumentlayout.cpp:2144-2229`; `qt/qtextmarkdownimporter.cpp:248-264` | none | C13, C24 | P |
| K-SF16† | 4.2 | rich text view | Tables | Border 1, collapsed, cell padding 4, border brush `Qt::darkGray` (a constant, not the palette), style Outset; header cells bold | P: `qt/qtextformat.cpp:3259-3266`, `:2953-2957`; `qt/qtextmarkdownimporter.cpp:316-319` | none | C12, C24 | P |
| K-SF17† | 4.2 | rich text view | Horizontal rule, paragraphs | Rule a 1-unit line in the palette's Inactive `WindowText`; paragraph top/bottom margin point size × 2/3 | P: `qt/qtextmarkdownimporter.cpp:181`, `:334-339`, `:645-648`; `qt/qtextdocumentlayout.cpp:2080-2092` | `separator.*` (colour only) | C04, C24, C26 | P |

Links in this view take the palette's `Link` (`qt/qtextmarkdownimporter.cpp:434-442`), which is
`defaults.link_color`: no row. Whether the anchor is underlined is to verify (the importer sets no
underline).

### Drop zones and dialog placement (G19, G24)

#### G19 — drop zone under a dragged pane, tab or dock

Covered by: K2.12 (Breeze's `QRubberBand` paint: fill, outline, radius, the mask rule — but see the row below
for the case K2.12's wording leaves out), K3.38† (the dock re-layout slide of a QMainWindow, timed by
`SH_Widget_Animation_Duration`), K1.126 (a QTabBar's tear line), K1.133† (tab move and drag-hover switch),
K1.182 (item-view drop indicator), K1.19† (KWin's quick-tile preview), K2.10 (dock title).

Application code that draws its own drop zone, so not a platform value:
- **Kate** (view spaces and tool-view sidebars): a tab dropped on another view space shows a `QRubberBand`
  over the whole view space and moves the document there (no edge zones, no split by drop):
  `kate/kateviewspace.cpp:676-682`, `:694-708`; the tool-view sidebar shows a `QRubberBand` parented to the
  main window and, for reordering, a second one forced to 4 px: `kate/katemdi.cpp:595-596`, `:1248-1282`.
  Their paint is Breeze's (K2.12, and the row below); the 4 px is Kate's.
- **Konsole** (split terminals): the half of the target terminal nearest the pointer is filled with a fixed
  `QColor(100, 100, 100, 127)`, not a palette colour: `konsole/terminalDisplay/TerminalDisplay.cpp:374-387`,
  `:807-818`. Nothing for the model to carry.
- **KDDockWidgets** (Kdenlive requires it: `find_package(KDDockWidgets-qt6 2.4.0 REQUIRED)`,
  `kdenlive/CMakeLists.txt:133`; Kdenlive keeps the default indicator type, `kdenlive/src/mainwindow.cpp:155-167`):
  the default `DropIndicatorType::Classic` (`kddw/core/ViewFactory.cpp:17`) draws the library's own PNG arrows
  (`kddw/qtwidgets/views/ClassicIndicatorsWindow.cpp:32`, `:148-149`, `:333-334`; `src/img/classic_indicators/`)
  and a `QRubberBand` (`kddw/qtwidgets/views/RubberBand.cpp:17-19`), whose paint is Breeze's (K2.12). The PNGs
  are the library's artwork, not platform data. Whether KDDockWidgets is in scope with kirigami-addons is §6.2's
  question.
- Kirigami / Qt Quick: no docking control at the pinned commits.

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| K-SF18† | 4.3 | drop target | QMainWindow dock drop zone (gap indicator) | While a `QDockWidget` is dragged over a main window, Qt opens a gap in the dock layout (slid open with `AnimatedDocks`, on by default) and, once the slide ends, shows a `QRubberBand::Rectangle` named `qt_rubberband` over the gap, or over the hovered floating dock group. Breeze paints that rubber band **filled** (`Highlight` α 0.2, lighter outline) when its parent is a `QMainWindow` (and a `QGraphicsView` or an item view), and as a **1 px outline only** elsewhere, so over a floating dock group window; K2.12 states the outline-only rule for "outside item views" and leaves out the main-window and graphics-view exceptions. Missing: the drop-zone paint as its own role (fill, outline, radius, the filled-or-outline rule). | P: `qt/widgets/qmainwindowlayout.cpp:2974-3016` (`updateGapIndicator`), `:2711` (default `AnimatedDocks \| AllowTabbedDocks`); `st:893-917` (`SH_RubberBand_Mask`), `st:6789-6806` (paint, as K2.12); M: Qt Widgets probe, dock dragged over the main window and over a floating group | none | C17, C07, C12, C04 | P, M |

#### G24 — where a modal dialog is placed

Covered by: Q1.113 (Kirigami Dialog's maximum size), Q2.40 (OverlaySheet's top inset ≥ gridUnit×3 and its
motion), Q2.58† (the mobile dialog layer: inset gridUnit×2.5 on windows wider than gridUnit×40, full-window
layer when narrower — the only window-size breakpoint found), K1.223 (KWin's entrance), K3.15† (the parent
dimmed).

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| K-SF19† | 4.1 | [dialog] | Qt Widgets dialog: Qt's placement | On first show a `QDialog` centres itself on its parent's window, both axes (the frame estimated from visible top-levels, or 10 × 40 when that fails), clamped into the screen's available geometry; with no parent widget, on its transient parent window; with neither, in the middle of the screen (the cursor's screen when there are several). Qt skips this when the platform theme's `WindowAutoPlacement` hint is true; Qt's default is false, and KDE's platform theme does not set it (no match in plasma-integration @ `46eddde9`). On Wayland a client cannot position its top-level window (xdg-shell has no position request: to verify), so KWin's rule (next row) applies; which wins on X11: to verify. | P: `qt/dialogs/qdialog.cpp:855-860` (called from `showEvent`), `:867-958` (`adjustPosition`); qtbase v6.10.1 `qplatformtheme.cpp:96-98`, `:625-626` | none | C26 | P, V |
| K-SF20† | 4.1 | [dialog] | Qt Widgets dialog: KWin's placement | KWin places a dialog, and any transient with a surface, through `placeDialog` → `placeOnMainWindow`: centred on its main window's frame and kept inside the work area; when it has no main window on the current desktop, or two or more, it falls back to the global policy, `kwinrc [Windows] Placement`, whose default is `Centered` (`Maximizing` in a build without decorations). | R: kwinrc `[Windows] Placement` (`kwin/kwin.kcfg:87`, `:117-136`); P: `kwin/placement.cpp:33-55`, `:339-355`, `:400-402`, `:451-510` | none | C26, C25 | R, P |
| K-SF21† | 4.1 | [dialog] | Kirigami Dialog (and PromptDialog, MenuDialog, SearchDialog): position | Centred both ways in `applicationWindow().overlay`, the whole window; `y` is computed from `implicitHeight` and carries the entrance rise (Q1.111). | P: `kirigami@81ada72b src/dialogs/Dialog.qml:297-303` | none | C26 | P |
| K-SF22† | 4.1 | [dialog] | qqc2-desktop-style Dialog: position | Parented to `QQC2.Overlay.overlay` and `anchors.centerIn` it. | P: `qqc2-desktop-style@305eb2c4 org.kde.desktop/Dialog.qml:18-19` | none | C26 | P |
| K-SF23† | 4.2 | OverlaySheet | OverlaySheet: position | Horizontally centred in its parent; vertically centred, but never above the larger of its visual parent's top (the page it covers) and gridUnit×3. Q2.37 calls it "bottom-sheet/centred sheet"; the template at this commit has no bottom-anchored branch in `x` / `y` (whether a mobile form anchors it to the bottom elsewhere: to verify). Q2.40 holds the gridUnit×3 floor only. | P: `kirigami@81ada72b src/templates/OverlaySheet.qml:121-129` | none | C24, C26 | P, V |

---

## Relabelled controls (2026-10-28)

Pass 2 asked the platforms about 18 controls under a wrong control name (a tagging
error: GN-2.40–2.46, GN-2.49, K2.7, K2.15–2.17, K2.22, Q2.43, W2-13, W2-14, W2-19,
W2-25), so this check asked again for this platform. Decisions: 1 a new §4.2 row, 2 a
new §4.1 row, 3 a row already covers it (the same job, possibly done another way),
4 the platform has no such thing.

Sources new in this check, besides the KDE file's table (all read 2026-10-28): `kconfigwidgets@939320a3` = KDE/kconfigwidgets @ `939320a3371d92ebb7a87a8a955cc0e7fae5580f`; `kxmlgui@b5aa1026` = KDE/kxmlgui @ `b5aa1026102a2eb332a07ca629e785535d49f8a7` (the file's `ktb` commit); `kitemviews@41adfcaf` = KDE/kitemviews @ `41adfcaf64031030f84f929401a4a0856a87c9d1`; `plasma-desktop@a8a72a58` = KDE/plasma-desktop @ `a8a72a58d57ccdd7c11d29de4de71a921df8ffdb`; qtbase and qtdeclarative at tag v6.10.1 (the version the file's pass-2 rows cite).

| Control | Decision 1–4 | Row added, row covering it, or the source showing it is absent |
|---|---|---|
| keyboard shortcut keycap (GN-2.40) | 1 | new 4.2 row: Command bar and its shortcut keycaps (KCommandBar). KCommandBar, opened from every KXmlGuiWindow (`kxmlgui@b5aa1026 src/kxmlguiwindow.cpp:176-189`), draws each key of a shortcut as a keycap. Elsewhere KDE writes shortcuts as text: menus K1.67, Q1.30; toolbar tooltips Q1.76 |
| shortcuts dialog (GN-2.41) | 1 | new 4.2 row: Shortcuts dialog (KShortcutsDialog / KShortcutsEditor). It is the "Configure Keyboard Shortcuts…" editor of every KXmlGui application (`kxmlgui@b5aa1026 src/kxmlguiwindow.cpp:334`), not a read-only help overlay. Its tree, buttons and search line are standard widgets that existing rows cover. The new row holds what they do not: the in-row extender and the key-sequence button while it records |
| text selection handles (touch) (GN-2.42) | 3 | Q1.14 (qqc2 MobileCursor: 3 px bar and round handle, tablet mode only). The touch Cut / Copy / Paste bar is Q2.53†. KDE has no magnifier: neither qqc2-desktop-style nor Kirigami `src/` contains a magnifier or loupe (search for "magnif", "loupe"). Qt Widgets text controls draw no touch handles |
| emoji chooser (GN-2.43) | 1 | new 4.2 row: Emoji selector (plasma-emojier). Plasma's own picker, bound to Meta+. (`plasma-desktop@a8a72a58 emojier/app/org.kde.plasma.emojier.desktop:162`), built from Kirigami. Also new 4.2 row: Character selector (KCharSelect), the Unicode character table of KWidgetsAddons |
| video play overlay (GN-2.44) | 4 | KDE has no video or media control. There is none in the `kwa src/` widget list, Kirigami `src/controls/` or qqc2-desktop-style's control list (`qqc2-desktop-style@305eb2c4 CMakeLists.txt:110-168`). This agrees with the KDE file's table A, "media transport controls" (decision 3 there) |
| editable label (inline rename) (GN-2.45) | 2 | new 4.1 row: [list] In-place item editor. Kirigami has no editable label: `src/controls/` has only the read-only `SelectableLabel.qml`. Qt Widgets edits in place through the item views' delegate editor |
| wrap box (flow layout of chips/tags) (GN-2.46) | 4 | KDE has no wrap or flow container. Kirigami's `src/layouts/` holds ColumnView, FormLayout, HeaderFooterLayout, Padding, SizeGroup and ToolBarLayout, and none of them wraps. The `kwa src/` widget list has no flow layout. Chips (Q2.22–Q2.24) are placed by an app's own QtQuick `Flow`, which no KDE style sets |
| drag icon (GN-2.49) | 2 | new 4.1 row: [list] Drag image of item views. Related rows: drop indicator K1.182, tab tear line K1.126, drag cursors K3.65†, drag threshold K3.71. Kirigami's ListItemDragHandle (Q2.43) moves the row itself and has no drag image |
| grid view items (W2-13) | 3 | K1.180 (QListView IconMode: icon above text, the same selection shape as K1.170–K1.175). Kirigami grids: CardsLayout / CardsListView Q1.133. A QtQuick GridView of qqc2 ItemDelegates takes the row look of Q1.77–Q1.81. KDE draws no selection check box in a tile corner (Breeze `st:4751-4900` paints the selection only) |
| items view (W2-14) | 3 | The selection visual of a generic item container: K1.170–K1.175 (Qt Widgets item views) and Q1.77–Q1.81 (qqc2 ItemDelegate) |
| repeat button (W2-19) | 2 | new 4.1 row: [button] Auto-repeat while held. The button's look is the [button] rows; K3.76† (spin box) and Q1.42 (Quick scroll bar) cover only their own repeats, not a button's |
| radio group (W2-25) | 3 | QButtonGroup draws nothing. A labelled group is QGroupBox (K2.8, its title space K1.211) or qqc2 GroupBox (Q2.13). Form rows are K1.210 and Q2.45–Q2.47 (FormLayout row and column spacing, label alignment). The gap between radios is the layout spacing (`layout.widget_gap_px`). The radio itself is the [checkbox] rows K1.50–K1.57 and Q1.20–Q1.22 |

Tally: 1 — 3 controls (4 rows); 2 — 3; 3 — 4; 4 — 2.

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| K-RL01† | 4.2 | command bar | Command bar and its shortcut keycaps (KCommandBar) | The whole control: a sunken styled-panel frame (frame forced on Breeze), a frameless search field with a leading `search` icon and a clear button, a tree of actions, and a drop shadow (`QGraphicsDropShadowEffect` in the palette's `Dark`, offset 0, 4, blur radius 48). Size: preferred width = width of the space it opens in ÷ 2.4, height = that height ÷ 2, minimum 500 × 250. Placed top-centre, 6 px below the top of that space. **Keycaps** in the shortcut column: a shortcut is split into key and separator segments ("Ctrl", "+", "A", ", " …). Each key is a rounded rect of radius 3.0 filled with the palette's `Window`, with a 1 px downward copy of itself in `Shadow` beneath it as its shadow. Key text is `ButtonText`; separators are in the row's text colour (`HighlightedText` when selected). Horizontal margin = `PM_FocusFrameHMargin` + 2 per side; height = the font's line spacing | P: `kconfigwidgets@939320a3 src/kcommandbar.cpp:299-387` (keycap paint), `:417-440` (segments), `:473-476` (margin), `:158-168` (text colour), `:572-605` (frame, shadow, field, delegates), `:712-731` (size, position) | none | C24, C01, C05, C11, C26 | P |
| K-RL02† | 4.2 | shortcuts dialog | Shortcuts dialog (KShortcutsDialog / KShortcutsEditor) | The parts no standard-widget row covers. (1) The editor tree's **in-row extender**: clicking a row opens an editor widget below it inside the same row. A leading 16 × 16 indicator (the style's `PE_IndicatorArrowRight` / `Left` in RTL when closed, `PE_IndicatorArrowDown` when open) sits on the item panel (`PE_PanelItemViewItem`). (2) The **key-sequence button** (KKeySequenceWidget): a push button with the `configure` icon showing the shortcut in native text, padded by a space on each side ("None" when empty). While it records it is held **pressed** (`setDown`) and reads "Input" or the partial sequence followed by " …". Beside it is a "Clear" tool button with `edit-clear-locationbar-rtl` / `-ltr`. The dialog's layout and the rest of its look: measure | P: `kxmlgui@b5aa1026 src/kshortcutseditordelegate.cpp:30-48`, `src/kkeysequencewidget.cpp:31`, `:142-157`, `:373-407`; `kitemviews@41adfcaf src/kextendableitemdelegate.cpp:196-218`, `:250-279`; M: probe app opens Configure Keyboard Shortcuts | none | C24, C07, C13, C19 | P, M |
| K-RL03† | 4.2 | emoji selector | Emoji selector (plasma-emojier) | Plasma's emoji picker window (Meta+.). A Kirigami ApplicationWindow, default gridUnit × 27 by × 25, with a collapsible, non-modal GlobalDrawer of categories and a search field in the page header. **Emoji grid**: cell size gridUnit × 3 by default, minimum gridUnit × 1.5, maximum 2.75 × the default; Ctrl+wheel zooms in steps of 0.25 × the default; the column count fills the width. Each cell is a qqc2 ItemDelegate (hover and current look: Q1.77). The glyph is in font family `emoji` at round(cell height × 0.6) px, centred. A **variant button** (`view-more-horizontal-symbolic`, small icon) shows at the cell's top-right corner on hover or focus for emoji with skin-tone variants and opens a Kirigami.Dialog with a grid of the variants. Tooltip: the emoji's name after `toolTipDelay` | P: `plasma-desktop@a8a72a58 emojier/app/ui/Emojier.qml:25-26`, `:85-92`; `emojier/app/ui/CategoryPage.qml:227-241`, `:311-353`, `:402-447`; `emojier/app/org.kde.plasma.emojier.desktop:162` | none | C24, C18, C26 | P |
| K-RL04† | 4.2 | character selector | Character selector (KCharSelect) | KWidgetsAddons' Unicode character table: a QTableView with both headers hidden. Each cell holds one character, centred, in the chosen font. The cell is filled `Base` when that font has the glyph and it is printable, `Button` otherwise and in empty cells; the table's background is `Base`. Cell width comes from the font's metrics: the widest glyph, clamped to 2–8 × x-height and at least the font height. Cell height is computed from the font metrics too (`:299-307`). The other controls are standard: font combo, size spin box, block combos, search line, history buttons | P: `kwa kcharselect.cpp:115-131`, `:255-310`, `:1089-1121` | none | C24, C26 | P |
| K-RL05† | 4.1 | [list] | In-place item editor | Qt Widgets edits an item view cell in place (rename in file and list views) with Qt's `QExpandingLineEdit`. It is framed only if the style's `SH_ItemView_DrawDelegateFrame` says so: Qt's default is 0, and Breeze does not override it. So in Breeze the editor is a **frameless** field (Base fill only, K1.41). It sits on the cell's text rectangle (`SE_ItemViewItemText`) and **widens with its content**. The model has no in-place editing look for a list cell | P: qtbase v6.10.1 `qitemeditorfactory.cpp:261-262`, `:509-556`; `qstyleditemdelegate.cpp:472-487`; `qcommonstyle.cpp:5338-5340`; Breeze has no `SH_ItemView_DrawDelegateFrame` case (`st`, by search) | `input.background_color` (fill only) | C12, C13, C26 | P |
| K-RL06† | 4.1 | [list] | Drag image of item views | A drag from a Qt Widgets item view carries an image of the dragged items. Each item is repainted by its delegate in the **selected** state (`State_Selected` added) on a transparent pixmap at the window's device-pixel ratio, with no fade or outline added. The pointer holds the image where the press was (hot spot = press point). Breeze adds nothing. How the compositor shows the drag icon (opacity, a shadow) is to verify | P: qtbase v6.10.1 `qabstractitemview.cpp:3876-3890`, `:4726-4751`; M: probe app drag, captured under KWin | none (`list.selection_*` paint the items) | C07, C26 | P, M, V |
| K-RL07† | 4.1 | [button] | Auto-repeat while held | A button with `autoRepeat` fires again while it is held: the first repeat after `autoRepeatDelay` 300 ms, then every `autoRepeatInterval` 100 ms. These are Qt's defaults in Qt Widgets and Qt Quick (QQC2 `AbstractButton`); Breeze and qqc2-desktop-style set neither. Users see it on QTabBar's scroll arrows (K1.125), which Qt makes auto-repeating, and on any app button that turns it on. K3.76† (spin box, 500 / 150 / 75 ms) and Q1.42 (Quick scroll bar) are other values | P: qtbase v6.10.1 `qabstractbutton.cpp:31-32`, `:149-150`, `:652`, `:684`; `qtabbar.cpp:374`, `:380`; qtdeclarative v6.10.1 `src/quicktemplates/qquickabstractbutton_p_p.h:91-92`, `qquickabstractbutton.cpp:287`, `:294`; qqc2-desktop-style sets no `autoRepeat` (search of `org.kde.desktop/`) | none | C21 | P |

## Fonts and mirroring (2026-10-28)

A completeness check of the map found two gaps no row covered: the emoji font and
the fallback chain for scripts the UI font lacks (all six platforms), and right-to-left
layout where this file had no general row. The source abbreviations are defined in
`todo_v0.7.1_model-completeness-gap-map-seeds.md`, *Fonts and mirroring sources*.

Not covered by any row (K3.45 is antialiasing and hinting, K3.46 the QFont string's extra fields). Qt
Widgets and Qt Quick (Kirigami) share Qt's font database, so one pair of rows serves both. No Plasma
setting for the emoji family was found: whether plasma-workspace's fonts KCM writes any fontconfig rule
beyond rendering (K3.45) is to verify.

| ID | section | model element | Element | What is missing | Source | Nearest model field | CAPS | SRC |
|---|---|---|---|---|---|---|---|---|
| K-FM01† | 4.3 | typography | Emoji font | For an emoji run Qt asks fontconfig for a colour font with the weak family `emoji` and requires a colour font; fontconfig resolves `emoji` through its prefer list, Noto Color Emoji first, then Apple Color Emoji, Segoe UI Emoji and others, the monochrome Noto Emoji last, and tags emoji with the language `und-zsye`. Which of them a system has is the distribution's choice; an application can override it (`QFontDatabase::setApplicationEmojiFontFamilies`). The model has no emoji family. | P: `qt/unix/qfontconfigdatabase.cpp:826-832`, `qt/qfontdatabase.cpp:2570-2629`, `:2942-2965`; `fc/60-generic.conf:9-47`, `fc/45-generic.conf:8-77`; R: `fc-match emoji` (the reader already runs `fc-match`, `native-theme/src/fonts.rs:283-307`; the colour property in the pattern is to verify); M: Qt probe app, the sample set, the family of each `QGlyphRun` | none (`defaults.font.family`) | C18, C25 | P, R, M |
| K-FM02† | 4.3 | typography | Fallback chain per language | Qt's fallback list for a family is fontconfig's sorted match (`FcFontSort`) for that family with a language: the script's own language where Qt has one, and for Common and Han text the system's default language, so Han characters take a Chinese font under `LANG=zh_CN` and a Japanese one under `LANG=ja` (Qt's own comment). The per-language preferences are fontconfig's (`65-nonlatin.conf`) and the distribution's. The model has one family and no language. | P: `qt/unix/qfontconfigdatabase.cpp:810-872` (Han comment `:848-852`); `fc/65-nonlatin.conf` (per-language families, e.g. `:27-39`, `:103-109`); R: `fc-match -s '<family>:lang=<tag>'` per language (pattern syntax per the fontconfig user's guide, to verify); M: Qt probe app, the sample set under each `LANG` | none (`defaults.font.family`) | C18, C25 | P, R, M |
| K-FM03† | 4.3 | mirroring | Right-to-left layout (outside the brief: no such row exists) | Under a right-to-left language Qt Widgets and Qt Quick mirror: Breeze branches on `layoutDirection()` / `option->direction` in its style code (63 places; what each draws differently is to verify), Kirigami mirrors through QML `LayoutMirroring` (drawers, page row, header, overlay sheet). Which parts mirror and which do not (KWin's title-bar button order included): to verify per part. Cross-check: MAC-X34†, W3-54†, X-SV-3. | R: `QGuiApplication::layoutDirection()` (the app's; how the reader learns the session's language direction: to verify); P: `st:454-458`, `:1043`, `:1760`, `:2321`, `:2355-2359` (63 direction branches in the file); `kirigami@81ada72b src/controls/GlobalDrawer.qml`, `src/controls/PageRow.qml`, `src/templates/OverlaySheet.qml` (34 `LayoutMirroring` uses under `src/`); M: Qt and Kirigami probe apps under an RTL language (Qt's `-reverse` option, to verify) | none | C26, C19, C25 | R, P, M |
