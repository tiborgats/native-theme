# v0.5.9: unstated sizes, and the showcase's chrome UX — rationale

Status: implemented (2026-09-23) and archived; see *As built* at the end.
Spec: `todo_v0.5.9_unstated-sizes-and-chrome-ux-spec.md`.
Plan: `todo_v0.5.9_unstated-sizes-and-chrome-ux-plan.md`.

## 1. What the maintainer saw

Six problems showed up in the gpui showcase, on KDE with the system theme:

1. The status bar's text touches the window's left and right edges.
2. The About dialog's content touches its frame.
3. The Toggle Inspector button (right end of the toolbar) and the Sidebar toggle (left end) sit in unintuitive places.
4. The Sidebar's icons overlap.
5. The toolbar's preset, colour-mode and icon-set controls have no labels, and look wrong there.
6. The window title carries the preset and mode. It should carry the `native-theme-gpui` version instead.

Items 3, 5 and 6 are showcase design. Items 1, 2 and 4 are correctness problems. Items 1 and 2 come from the library. Item 4 comes from a wrong icon size (§9).

## 2. Why the status bar and the About dialog have no spacing

`geometry::status_bar` and `geometry::dialog` take a widget's padding from `<widget>.border.padding_{horizontal,vertical}`. For the KDE theme, both widgets' paddings resolve to `0.0`.

**Status bar.** Platform-facts §2.14 has no padding row. Nothing documents a status-bar padding, and no preset states one.

**Dialog.** Platform-facts §2.22 documents a dialog margin for every platform:

| Platform | Dialog margin | Source |
|---|---|---|
| KDE | 10px | `Layout_TopLevelMarginWidth`, confirmed in breezemetrics.h (platform-facts.md:683) |
| Windows | 24px | `ContentDialogPadding` (platform-facts.md:449) |
| macOS | about 20px | measured |
| GNOME | 24px horizontally | |

Yet no preset states a dialog padding. `adwaita.toml`'s `[light.dialog.border]` holds only a corner radius.

**Resolver.** It fills a missing widget padding with `0.0` and reports nothing (native-theme `resolve/validate_helpers.rs:283-284`, `:337-338`).

**Connector.** It then applies that 0, which overrides the toolkit's own padding. gpui-component's StatusBar, for example, has `px_2 py_1` (status_bar.rs:89-90).

The same mechanism is behind two more problems, recorded in the open `docs/todo.md` item "KDE's toolbar height has no source, and no preset states a toolbar padding":

- The toolbar padding resolves to 0 on every preset. §2.13 documents KDE 6, GNOME 6, macOS 8 (measured) and Windows 4 left / 0 right.
- KDE's `toolbar.bar_height = 40` has no source. §2.13 says KDE's toolbar "sizes to content".

## 3. The principle

The goal is for a native theme to look and feel native, as closely as the data allows. The rule that outranks everything is never to invent a value.

When the platform does not state a sizing value, there are five options:

| Option | What the user sees | True? |
|---|---|---|
| Fill in `0.0` (today) | Content touching its frame, a look no platform has | No. `0` is a statement ("no padding") that nothing makes. |
| Fill in a stand-in number (today's KDE `bar_height = 40`) | Something plausible | No. It is an invented value, and it hides the gap for good. |
| Derive it from a related documented value | The related value | Only where platform-facts documents the derivation, e.g. GNOME's combobox padding "← button padding (10px)" (§2.24). Such a value counts as stated (§5, point 5). |
| Require every theme to state it (a resolution error) | Nothing different for native presets | It forces every user theme and colour-scheme preset to invent numbers its author does not have. |
| Keep it **absent**, so the toolkit's own default stands | The toolkit's own look for that one property | Yes. It says nothing the platform did not say, and the gap stays visible, so it can be counted and closed. |

For a value the platform truly does not state, only the last option is honest. It is also as close to native as the data allows: a toolkit's default is a considered design value, and `0` is not.

The principle cuts both ways. **Where the platform does state a value, the native preset must carry it.** A native preset that omits a documented value, as `kde-breeze` omits its dialog margin, is a defect. A gate must catch it, and the resolver must not paper over it.

**Only a number is a stated value.** Platform-facts writes some cells as "(none)" with a remark, for example §2.16's "(none) — content provides own padding" for the popover, on all four platforms alike. Such a remark may describe a real zero, but it cites nothing, so it is not a value, and it is read as unstated. Where a remark does describe a real zero, the fix is to source it and write `0` with its source into platform-facts. The audit proposes such corrections; it never writes a `0` on the strength of a remark.

## 4. Padding is per side

Platform-facts states padding per side. Its conventions define `padding_horizontal`/`_vertical` as the amount applied to each side, and write an asymmetric cell as, for example, "10 left / 6 right". Ten of its padding rows give two different sides:

| Widget | Documented padding |
|---|---|
| Windows button vertical | 5 top / 6 bottom |
| Windows input | 10 left / 6 right; 5 top / 6 bottom |
| Windows tooltip vertical | 6 top / 8 bottom |
| Windows tab horizontal | 8 left / 4 right |
| Windows combobox | 12 left / 0 right; 5 top / 7 bottom |
| Windows toolbar horizontal | 4 left / 0 right |
| Windows menu vertical | 4 top / 5 bottom (mouse); 8 top / 9 bottom (touch) |
| GNOME dialog vertical | 32 top / 24 bottom |

The model has one value per axis, so the presets pick one number for each of these, inconsistently:

| `windows-11.toml` field | Stated | Documented | Choice |
|---|---|---|---|
| tooltip vertical | 7 | 6 top / 8 bottom | an average, which no source gives |
| button vertical | 6 | 5 top / 6 bottom | the bottom value |
| input horizontal | 10 | 10 left / 6 right | the left value |
| input vertical | 5 | 5 top / 6 bottom | the top value |

Leaving those unstated would strip most of Windows' documented padding. Keeping the chosen numbers keeps invented values. The only answer that is both true and native is a model that states each side. The plan already changes every padding consumer, so adding sides in the same change touches that code once.

**The model stores only the four sides.** Unresolved, these are `padding_top`, `padding_right`, `padding_bottom` and `padding_left`, each `Option<f32>`. Merging stays exactly as it is: field by field, with the overlay winning (pipeline.rs:48-52). So a reader's or a user theme's value overrides the preset's, side by side.

**TOML keeps its axis keys as shorthand.** `padding_horizontal_px = 10` is read as left = right = 10, and `padding_vertical_px` likewise sets top and bottom. Every existing preset and user theme keeps working. A table that states an axis key together with one of its sides is a parse error. That leaves no precedence rule to learn and no merge order to get wrong.

## 5. So the long-term design is

1. **The model can say "not stated", per side.**
   - Each padding side and `toolbar.bar_height` resolves to `Option<f32>`.
   - `None` means the platform states no value.
   - `Some(0.0)` means it states zero, as KDE's toolbar vertical padding does.
2. **The resolver stops inventing.** A missing value stays `None`. A stated padding side must be ≥ 0; today no range check covers widget padding.
3. **Connectors apply only what is stated.**
   - A gpui builder refines a side only when that side is `Some`. Otherwise the toolkit's own default stands: gpui-component's StatusBar, for example, keeps `px_2 py_1`.
   - **The input, select and combobox builders apply their stated sides too.** Upstream sets its own padding on those widgets' roots before applying the refinement (input.rs:701 then :719; select.rs:544 then :546; combobox.rs:995 then :997), so a refinement overrides it. The builders' current "Tier U" note (geometry.rs:147), which says the padding cannot be reached, is wrong. Today no preset's input or combobox padding reaches gpui at all. One side has no receiver: WinUI measures the combobox's right padding of 0 up to a separate 38px arrow column (§2.24), which gpui's trigger does not have, so that side stays unstated.
   - **Control heights: the stated height up to the platform's text scale, laid out above it.** At a text scale of 1 or less, a control is exactly the height its theme states. Above 1, the builder sets that height as a minimum and lets layout grow the control around its drawn text and padding. That is how the native toolkits behave: GTK's `min-height` with content-driven growth, and Breeze's heights derived from font metrics. So that the growth follows the platform's metrics rather than gpui-component's, the same builders apply the platform's `defaults.line_height`, which is sourced on all four platforms (platform-facts.md:1019) and today reaches no control builder. `control_height`, which computes a height from the model's padding and gets it wrong at scale 1 on Windows and macOS, goes. The rule is for single-line controls; a multi-line input keeps the height its caller gives it. On KDE the stated heights (32 for the button and input, 28 for a row) are themselves unsourced (§7); the rule applies what the theme states, and the follow-up decides those values.
   - **The tooltip's text width does need a number.** `tooltip_content` subtracts the padding from `max_width`. Where no side is stated, it uses upstream's own tooltip padding (`px_2`, tooltip.rs:123). That is a rem multiple, so it is converted at the font size the connector installs as the rem (root.rs:582).
   - **iced's `button_padding` and `input_padding` return a whole `Padding`.** They fill an unstated side with iced's own public default: `button::DEFAULT_PADDING` (iced_widget 0.14.2 button.rs:461) or `text_input::DEFAULT_PADDING` (text_input.rs:125).
4. **Native themes state everything their platform documents, for the fields this change covers** (§6, §7). A gate checks both paths by which a native theme reaches the model.
5. **Reading platform-facts cells:**

   | Cell | How it is read |
   |---|---|
   | A number, with or without a qualifier such as "(measured)", "(convention)" or a leading "~" | Stated. The comment carries the qualifier. |
   | A range ("~8–10px") | Not stated, unless a source pins it. Then platform-facts is corrected. |
   | A value per context (input device, list style, size mode, with or without a close button) | The context the connector's widget matches is stated, and the comment names it. For the input device that is the desktop pointer, so the Windows menu states its mouse values. Where no context clearly matches, a ruling decides. If none matches, the value is `None`. |
   | A derivation ("← button padding (10px)") | Stated as the derived value, and the comment cites the derivation. |
   | A pointer ("(none) — use §2.20 layout margins") | Not stated here; the field it points to carries the value. |
   | A cell with no number ("(Adwaita CSS) row padding"), or any "(none)" | Not stated. |
6. **An application may choose its own value where the theme states none, and must say so.** The connector never invents. The showcase, though, is an application. An application-drawn element, such as the toolbar row `geometry::toolbar` refines or the Sidebar header's rows, has no toolkit default. Where the theme leaves such a value unstated, the showcase uses its own named constant, and the element's info says it is the showcase's choice. Without that, the colour-scheme presets' toolbar would put its items flush against the window edge, the very defect reported.

## 6. The platform's values come from two places

A native theme reaches the model two ways:

- **Static path:** the user picks a preset, e.g. `kde-breeze`, and its TOML alone gives the sizes.
- **Live path:** the user runs the system theme. The pipeline starts from the full preset, merges the `-live` preset over it, then merges the OS reader's output on top (pipeline.rs:48-52). Wherever a reader sets a size, the reader's value wins.

The readers' sizes are mostly constants:

| Reader | Where its constants are |
|---|---|
| KDE | `kde::metrics::populate_widget_sizing` (kde/metrics.rs) |
| macOS | `macos_widget_defaults` (macos.rs:257) |
| Windows | The WinUI3 constants in `read_widget_sizing`, written twice: once for Windows builds (windows.rs:186) and once as a "non-Windows testable version" (:246) |

The KDE reader also reads some sizes from the system, such as icon sizes from `index.theme` (kde/mod.rs:128). Those are not constants, and the gate does not cover them.

Both paths must agree with platform-facts, and today they do not. On the live path these reader constants win over the presets:

| Reader | States | Platform-facts |
|---|---|---|
| kde/metrics.rs:20 | button vertical padding 5 ("Breeze measured frame+margin") | `Button_MarginWidth` = 6 (§2.3) |
| windows.rs:196-233, :250-290 | 12 horizontal for button, input, tab and menu; tooltip 8 / 8 | 11; 10 left / 6 right; 8 left / 4 right; 11; tooltip 9 and 6 top / 8 bottom |
| windows.rs:240, :296 | `toolbar.item_gap` 4 | 0 (§2.13; `windows-11.toml:217` also says 0) |
| macos.rs:263 | button horizontal padding 12 | ~8 (§2.3; the preset states 8) |

So the gate resolves both paths:

- the static preset;
- full preset, then `-live` preset, then the reader's constants.

Checking a `-live` preset alone would check values a reader may override.

Every run must see every reader's constants, and today that is not possible. The pre-release check runs `cargo test -p native-theme` with no features, and the KDE module only builds on Linux with the `kde` feature (lib.rs:99-100). So the KDE and Windows constants move into functions outside any feature or OS gate. Only the OS reads, `GetSystemMetricsForDpi` and `index.theme`, stay gated.

A mismatch is corrected on whichever side its source proves wrong: the preset, the reader, or platform-facts.

## 7. Scope: what this change leaves for a follow-up

The same principle reaches beyond padding. Platform-facts marks KDE **(none)** for these fields, yet `kde-breeze` states a value for each:

| Field | `kde-breeze` value |
|---|---|
| button and input `min_height` | 32 |
| menu and list `row_height` | 28 |
| dialog minimum and maximum width and height | 320 / 560 / 140 / 600 |
| combobox `min_height` and `min_width` | 32 / 120 |

Tooltip `max_width` is "(none) — preset: 300" (platform-facts.md:1248): a preset's own choice, written into the facts. The macOS `small` icon size is a range, "sidebar: 16–20pt" (:1122), while `macos-sonoma` states 16. And `panel_px = 20` in adwaita, macos-sonoma and windows-11 stands against "(none)" (:1125).

This change leaves them as they are. Each needs its own connector decision, such as what a builder does without a minimum height or without a dialog bound. None of them causes the reported defects. The audit (plan Task 1) lists every such field, and `docs/todo.md` records the list as a follow-up plan.

The colour-scheme presets carry sizes that were copied rather than sourced (see the todo.md toolbar item). This change removes only their `bar_height_px = 40.0`: `toolbar.bar_height` becomes optional here, and 40 is the same unsourced value this change removes from KDE. Sourcing the rest of their sizes is part of the follow-up.

## 8. Visible changes on every preset

These change what native presets draw too, not only the colour-scheme presets:

- The popover and hover card were padded 0 on every preset, because no preset states a popover padding and the resolver filled in 0. They now draw the platform's stated padding where the audit sourced one (GNOME 8, Windows 15/16/17/16, ruling R13), and gpui-component's own `p_3` (popover.rs:284) elsewhere. The showcase's content inside them adds no padding of its own.
- The status bar draws gpui-component's own `px_2 py_1` on every preset until Task 2 states a platform's value. That is the fix for the first reported defect.
- On the colour-scheme presets, the dialog draws gpui-component's own padding instead of a zero nobody chose, and the toolbar gets the showcase's own padding (§5, point 6).
- The input, select and combobox draw their stated padding for the first time.
- Control heights become the stated height at text scale 1. Today `control_height` overshoots it: the Windows button is 33 for a stated 32, and at 96 DPI the macOS button, input and menu row are 27 for a stated 22. The select and combobox keep their minimum height, so nothing turns an unsourced value into an exact height.
- The showcase's Textarea, which `geometry::input` forces to the single-line height today (input.rs:706-709 runs before the refinement at :719), keeps its own 90px.

These are corrections, and the screenshot review covers them.

## 9. The showcase's chrome UX

**Panel toggles belong in the status bar, at the edge of the panel they control.** This is Zed's pattern:

- The left-panel toggle sits at the status bar's left end; the inspector toggle sits at its right end.
- Each is a small ghost icon button, shown selected while its panel is open.
- Its tooltip names the action and its key.

The alternatives lose:

- A toolbar is for acting on the content.
- VS Code puts layout toggles in the title bar. That is far from the panels, and ours already holds the menus.

**The left panel collapses to its icon rail** (the maintainer's decision). **The inspector hides fully**, as it does today.

**Both toggles are our own Buttons, not upstream's `SidebarToggleButton`.** `SidebarToggleButton` draws a hardcoded icon. Its Button is built at sidebar/mod.rs:313, and the icon is chosen at :349-360 (`PanelLeftClose`/`PanelLeftOpen`). So it would show gpui-component's icon whichever icon set is chosen, and the maintainer's rule forbids mixing icon sets.

- Our buttons use `IconName::PanelLeft` and `IconName::PanelRight` from the chosen set, as the rest of the chrome does. The connector's freedesktop map gives KDE `sidebar-expand-left` for `PanelLeft` but `view-right-new` for `PanelRight` (icons.rs:665-691), while Breeze ships `sidebar-expand-right` beside `sidebar-expand-left` (both present in `/usr/share/icons/breeze/actions/22/`). The `PanelRight` mapping is corrected to `sidebar-expand-right`, so the two toggles are a matching pair on KDE.
- The selected state, not a changing icon, shows whether a panel is open.
- `SidebarToggleButton` then appears nowhere, so it goes to `docs/showcase-exceptions.toml`. That file's list of reason kinds gains "it would mix icon sets".

**The theme settings go into the left panel, above the page list, each with a label.** They configure what the whole window shows, and a label says what each control is: Theme, Mode, Icon set.

- The Sidebar's own `header` slot holds them (sidebar/mod.rs:276). That slot already pads its content (`pt_3 px_3`).
- The icon rail has no room for them, so they are hidden there. Expanding the panel brings them back.
- While they are hidden, the command palette still offers the presets and colour modes, and the Theme menu offers the colour modes (chrome.rs:39-47).
- The icon set is reachable only from the expanded panel. That is accepted: the rail is the compact state.

**The toolbar keeps the actions:** Command Palette, Reload Theme and Preferences. It stays because it is where the showcase demonstrates `geometry::toolbar`, the connector's public toolbar builder.

**The title names the application and its version:** `native-theme-gpui 0.5.9 showcase`. One constant serves three places:

- the title-bar label, today `native-theme showcase — {preset} ({mode})` (chrome.rs:59);
- the window title the OS shows, today `Native Theme – GPUI Showcase, v…` (main.rs:929);
- the Windows screenshot capture, which finds the window by that title (main.rs:707-709).

The preset and mode stay in the status bar. The version leaves the status bar, because the title now carries it.

**The Sidebar's icons overlap because of their size.**

- The page icons are sized with `icon_size_panel` (demo.rs:472), which is 48px on KDE (kde-breeze.toml:61). KDE's "Panel" group is the Plasma panel.
- An expanded `SidebarMenuItem` row is `h_7` (sidebar/menu.rs:308): 1.75 rem, which is 28px at a 16px rem and about 23px at KDE's 13.33px font. The rail is 48px wide (sidebar/mod.rs:28). A 48px icon fits neither.
- The sidebar icon size is `icon_size_small`. Platform-facts' `small` row names macOS's value "sidebar" (platform-facts.md:1133), and it is 16 on KDE, Windows and GNOME.

## 10. Decisions

- **D1. Unstated stays unstated.** An unstated sizing value is `None` from the resolved model, through the connector, to the toolkit's default. Only a number in platform-facts is a stated value; a "(none)" with a remark is not. A single-line control is its stated height at a text scale of 1 or less, and is laid out from its drawn text, the platform's line height and its padding above that.
- **D2. The gate.** For padding, `toolbar.bar_height` and `toolbar.item_gap`, native themes state every value platform-facts documents. A gate enforces this on both paths:
  - the static preset;
  - the live merge: full preset, then `-live` preset, then reader constants.

  The reader constants move outside feature and OS gates, so every run sees them. Other sizing fields are a recorded follow-up (§7).
- **D3. Padding is per side.** The model stores only sides. The TOML axis keys are parse-time shorthand, and stating an axis key together with one of its sides is an error. Merging stays per field, with the overlay winning.
- **D4. Status-bar padding** is researched from upstream toolkit sources for all four platforms and recorded in platform-facts §2.14. A platform without a sourced number is **(none)**, with the reason, and stays unstated.
- **D5. Panel toggles** live in the status bar. The left one collapses its panel to the rail; the right one hides its panel. Both are our own buttons, with icons from the chosen set, and KDE's `PanelRight` icon becomes Breeze's `sidebar-expand-right`.
- **D6. Theme settings** live, labelled, in the Sidebar header, and are hidden in the rail. The toolbar holds the actions. Where the theme states no gap or padding for these application-drawn elements, the showcase uses its own named constant and says so.
- **D7. Title.** One constant, `native-theme-gpui <version> showcase`, serves the title-bar label, the OS window title and the screenshot capture's window lookup.
- **D8. The resolved border type is split** into `ResolvedDefaultsBorder` and `ResolvedWidgetBorder`. The shared `ResolvedBorderSpec` makes every widget border carry a `corner_radius_lg` and an `opacity` resolved to an invented `0.0` (validate_helpers.rs:279-281, :333-335). It would also make `defaults.border` carry a padding that is always `None`. No connector reads either widget field.
- **D9. Public API.** These breaking changes are allowed before 1.0, with no migration guide.
  - New: `ResolvedPadding`, `ResolvedDefaultsBorder`, `ResolvedWidgetBorder`, and `WidgetBorderSpec`'s four side fields.
  - Removed: `ResolvedBorderSpec`; `WidgetBorderSpec`'s `padding_horizontal`/`padding_vertical` fields (the TOML keys stay); `geometry::control_height`, which layout replaces; `native_theme_gpui::dialog_content_padding`, which nothing but its own test calls (lib.rs:1420) and which the resolved theme already exposes.
  - Changed: `toolbar.bar_height` becomes `Option<f32>`; `geometry::input_height` returns a `StyleRefinement` carrying the height rule alone, so a caller that wants only the height gets the same rule `geometry::input` applies.
  - Unchanged: iced's `button_padding` and `input_padding` keep their `Padding` return type.

## As built

Implemented 2026-09-23 on branch `v0.5.9-gpui-kit-0.6.6`, commits
`cd76c3e`..`b4d586b`, and archived in the commit after them. The decisions
held. These outcomes differ from the text above; the controller's rulings
behind them are in the SDD ledger.

- **§6, the audit's rulings (R1–R13).** They settled the per-context cells
  and corrected platform-facts from upstream sources. Windows tab 3/8/3/8,
  the "without close button" context (R1). Windows `bar_height` 48: the
  default CommandBar style is Compact, so 64 is not the default (R2). GNOME
  list 2/2/2/2, the plain list (R3). The Windows combobox's right side
  stays unstated (R4). Windows menu 4/11/5/11, the mouse context (R5).
  GNOME's toolbar is `.toolbar`: padding 6 on every side, `item_gap` 6, and
  no `bar_height`, because 47 is the headerbar's (R6). KDE toolbar 6 on
  every side (R7). KDE combobox 6 on every side (R8). No platform states a
  `segmented_control` padding except macOS's measured vertical 3; KDE's
  cells borrow the tab bar as a proxy (R9, R12). Windows expander 0/0/0/16,
  the header context (R10). Windows list 0/12/0/12 (R11). macOS combobox
  and segmented horizontal stay unstated, as ranges (R12). Popover padding
  GNOME 8 and Windows 15/16/17/16, and GNOME checkbox 3 (R13).
- **§5 point 3, heights: fit, not growth.** At text scale 2 a control is at
  least its scale-1 height and its text lies inside it. It need not grow:
  Windows 11's list row, with 0 top and bottom padding (R11), fits its text
  exactly in its 40px at scale 2, and that is correct.
- **D9, `input_height`.** It carries the height rule alone. Above text
  scale 1 a field it alone refines grows around upstream's text size and
  padding, not the platform's, so it equals the refined `Input`'s height
  at scale 1 only (kde-breeze at scale 2: 50px against 44px).
- **macOS select and combobox are 26px at text scale 1**, not the stated
  21. Upstream's trigger is `h_8`, 2 rem at 72 DPI, and `min_h` cannot
  shrink it. The seams test holds the two as an explicit exception;
  `combo_box.min_height` belongs to the follow-up (§7).
- **§9, the Sidebar icons.** `icon_size_panel` lost its only use. The Icons
  page gained an Icon Sizes section: the chosen set's icon at each of the
  five `defaults.icon_sizes` contexts, each reporting its own info, and
  marking a native preset's size where platform-facts §2.1.8 documents none.
- **§9, the Sidebar header (final review).** The Mode row became a Select like its siblings, so the header fits at text scales 1 and 2 on every offered preset (`ios` on macOS included); `NAV_WIDTH` is back to 200 and the window 1380px (the smallest passing width measured is 172). Before that: `NAV_WIDTH` was 205px, up from 200: the
  smallest whole-pixel width at which every control fits (at 204, adwaita
  overflows by 0.83px). The System toggle reads "System", with the resolved
  mode in its tooltip, and the fit test resolves each native preset at its
  own platform's DPI and text scale 1. The window is 1385px wide.
- **D4, the status bar.** KDE 3/0/2/2, QStatusBar's layout for added
  widgets, which is what the showcase's bar holds; GNOME 6/10/6/10;
  Windows and macOS (none).
- **§6, the reader constants.** The KDE constants compile on every target
  as the crate-private `kde_metrics`; the public `native_theme::kde::metrics`
  module, which held no public item, is gone.
- **§5 point 3, iced.** `button_padding` and `input_padding` need the
  `widgets` feature (on by default), because `iced_widget`, which holds
  `DEFAULT_PADDING`, is an optional dependency of it.
