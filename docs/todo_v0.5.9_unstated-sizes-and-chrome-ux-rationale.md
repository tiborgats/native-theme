# v0.5.9: unstated sizes, and the showcase's chrome UX — rationale

Status: proposed, 2026-09-23.
Spec: `todo_v0.5.9_unstated-sizes-and-chrome-ux-spec.md`.
Plan: `todo_v0.5.9_unstated-sizes-and-chrome-ux-plan.md`.

## 1. What the maintainer saw

The maintainer reported six problems in the gpui showcase, on KDE with the system theme:

1. The status bar's text touches the window's left and right edges.
2. The About dialog's content touches its frame.
3. The Toggle Inspector button (at the toolbar's right end) and the Sidebar toggle (at its left end) are in unintuitive places.
4. The Sidebar's icons overlap.
5. The toolbar's preset, colour-mode and icon-set controls have no labels, and look wrong where they are.
6. The window title carries the preset and mode. It should carry the `native-theme-gpui` version instead.

Items 3–6 are showcase design. Items 1 and 2 are not: they come from the library, and they are a correctness problem.

## 2. Why the status bar and the About dialog have no spacing

`geometry::status_bar` and `geometry::dialog` take a widget's padding from `<widget>.border.padding_{horizontal,vertical}`. On the KDE theme, both resolve to `0.0`:

- **Status bar.** Platform-facts §2.14 has no padding row: nothing documents a status-bar padding, and no preset states one.
- **Dialog.** Platform-facts §2.22 documents each platform's dialog margin:
  - KDE: 10px (`Layout_TopLevelMarginWidth`, breezemetrics.h ✅)
  - Windows: 24px (`ContentDialogPadding` ✅)
  - macOS: about 20px, measured
  - GNOME: 24px horizontally

  Yet no preset states a dialog padding. `adwaita.toml`'s `[light.dialog.border]` holds only a corner radius.

Two things turn these missing values into zero spacing:

- **The resolver** fills a missing widget padding with `0.0` and reports nothing (native-theme `resolve/validate_helpers.rs:283-284`, `:337-338`).
- **The connector** then applies that 0, which overrides the toolkit's own padding. gpui-component's StatusBar, for instance, has `px_2 py_1` (status_bar.rs:89-90).

The same mechanism explains the two findings of the showcase-app plan's Task 7 (`geometry::toolbar`):

- The toolbar padding is 0 on every preset. §2.13 documents KDE 6, GNOME 6, macOS 8 (measured) and Windows 4 left / 0 right.
- KDE's `toolbar.bar_height = 40` has no source. §2.13 says KDE's toolbar "sizes to content".

## 3. The principle

The goal: a native theme should look and feel native, as closely as the data allows. The rule that outranks everything is never to invent a value.

A sizing value that the platform does not state can be handled five ways:

| Option | What the user sees | True? |
|---|---|---|
| Fill in `0.0` (today) | Content touching its frame, a look no platform has | No. `0` is a statement ("no padding") that nothing makes. |
| Fill in a stand-in number (today's KDE `bar_height = 40`) | Something plausible | No. It is an invented value, and it hides the gap for good. |
| Derive it from a related documented value | The related value | Only where platform-facts documents the derivation ("(none) — use §2.20 layout margins"). Among the fields this change touches, only the window's padding has such a note, and no connector applies that padding. |
| Require every theme to state it (a resolution error) | Nothing different for native presets | It forces every user theme and colour-scheme preset to invent numbers its author does not have. |
| Keep it **absent**, so the toolkit's own default stands | The toolkit's own look for that one property | Yes. It says nothing the platform did not say, and the gap stays visible, so it can be counted and closed. |

Only the last option is honest. It is also as close to native as the data allows: a toolkit's default is a considered design value, and a `0` is not.

The principle cuts both ways. **Where the platform does state a value, the native preset must carry it.** A native preset that omits a documented value, as `kde-breeze` omits its dialog margin, has a defect. A gate must catch it, and the resolver must not paper over it.

## 4. Padding is per side

Platform-facts states padding per side: its conventions section defines `padding_horizontal`/`_vertical` as the amount applied to each side. Ten of its padding rows give two different sides:

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

The model has one value per axis, so the presets pick a number for each of these, inconsistently:

| `windows-11.toml` field | Stated | Documented | Choice |
|---|---|---|---|
| tooltip vertical | 7 | 6 top / 8 bottom | an average, which no source gives |
| button vertical | 6 | 5 top / 6 bottom | the bottom side |
| input horizontal | 10 | 10 left / 6 right | the left side |
| input vertical | 5 | 5 top / 6 bottom | the top side |

Leaving those values unstated would be honest, but it would strip most of Windows' documented padding. Keeping them keeps invented values. The only answer that is both true and native is to let the model state each side.

This plan already changes every padding consumer, so that it can handle "not stated". Adding per-side padding in the same change means that code changes once instead of twice.

The TOML stays simple:

- The existing `padding_horizontal_px` and `padding_vertical_px` stay. They are the shorthand for two equal sides, so every existing preset, reader and user theme keeps working.
- `padding_top_px`, `padding_right_px`, `padding_bottom_px` and `padding_left_px` state one side each, for the asymmetric cases.
- A side key wins over its axis key, as CSS's `padding-left` wins over `padding`. So `padding_horizontal_px = 10` with `padding_right_px = 6` reads as "10 left, 6 right".
- The rule is applied at resolution, so it holds whatever order the preset and the reader were merged in.

The resolved model carries four sides, each an `Option<f32>`.

## 5. So the long-term design is

1. **The model can say "not stated", per side.**
   - Each padding side and `toolbar.bar_height` resolves to `Option<f32>`.
   - So does any other sizing field that the audit finds some native platform leaves unstated.
   - `None` means the platform states no value. `Some(0.0)` means it states zero, as KDE's toolbar vertical padding does.
2. **The resolver stops inventing.** A missing value stays `None`.
3. **Connectors apply only what is stated.**
   - A gpui builder refines a side only when it is `Some`. Otherwise the toolkit's own default stands:
     - gpui-component's StatusBar keeps `px_2 py_1`;
     - an application-drawn toolbar with no stated height sizes to its content, which is KDE's native behaviour.
   - Where a builder computes something from a padding (a tooltip's text width, a control's height), it uses the padding actually drawn. That is the stated side, or upstream's own where none is stated.
   - iced's `button_padding` and `input_padding` return a whole `Padding`. For an unstated side they use iced's own public default: `button::DEFAULT_PADDING` (iced_widget 0.14.2 button.rs:462) or `text_input::DEFAULT_PADDING` (text_input.rs:125).
4. **Native presets state everything their platform documents.** A gate checks them against a table of platform-facts values.
5. **A measurement given as a range is not a value.** macOS's combobox "~8–10px (measured)" stays `None` unless a source pins it, in which case platform-facts is corrected. A single measured value such as "~20px (measured)" is stated, and its comment says "measured".
6. **A value given per context is stated for the context the connector's widget matches.** Contexts are things like the input device, a list style or a size mode.
   - For the input device, that is the desktop pointer. Windows' menu, for example, gives "touch: 8 top / 9 bottom, mouse: 4 top / 5 bottom", so the preset states the mouse values.
   - The preset's comment names the context.
   - Where no context clearly matches, as with GNOME's "rich-list" versus "plain list", the audit's ruling decides. If none matches, the value is `None`.

## 6. The platform's values come from two places

A native theme's sizes reach the model from two places:

- its preset TOML (`kde-breeze`, `kde-breeze-live`, …);
- constants in its OS reader (`kde/metrics.rs`, `windows.rs`).

Both must agree with platform-facts, and they already disagree. For example, `kde/metrics.rs:20` gives the button a vertical padding of 5 ("Breeze measured frame+margin"), where §2.3 gives KDE `Button_MarginWidth` = 6.

The audit therefore covers the presets and the readers, and the gate covers the presets. Readers run only on their own OS, so a mismatch in a reader becomes a ruling. Every ruling corrects whichever side its source proves wrong: the preset, the reader, or platform-facts.

The complete fix would be a machine-readable platform-facts, so that every native value is gated, not only the fields this change touches. That is its own project, and it goes into `docs/todo.md`.

## 7. Colour-scheme presets

Catppuccin, Nord, Dracula and the rest are colour schemes, not platforms:

- A sizing value they state stays as it is.
- None of them states a toolbar, dialog or status-bar padding. Those were `0.0` and become `None`, so the toolkit's default replaces a zero nobody chose.

This is a visible change, and a correction.

## 8. The showcase's chrome UX

**Panel toggles belong in the status bar, at the edge of the panel they control.** This is Zed's pattern:

- the left-panel toggle sits at the status bar's left end;
- the inspector toggle sits at its right end;
- each is a small ghost icon button, shown selected while its panel is open, with a tooltip naming the action and its key.

The alternatives lose:

- A toolbar is for acting on the content.
- VS Code puts layout toggles in the title bar. That is far from the panels, and here the title bar already holds the menus.

**The left panel collapses to its icon rail** (the maintainer's decision). **The inspector hides fully**, as it does today.

**Both toggles are our own Buttons, not upstream's `SidebarToggleButton`.** `SidebarToggleButton` draws a hardcoded icon: its Button is built at sidebar/mod.rs:313, and the icon is chosen at :349-360 (`PanelLeftClose`/`PanelLeftOpen`). It would show gpui-component's icon whatever icon set is chosen, and the maintainer's rule forbids mixing icon sets.

- Our buttons use `IconName::PanelLeft` and `IconName::PanelRight` from the chosen set, as the rest of the chrome does.
- The selected state, not a changing icon, shows whether the panel is open.
- `SidebarToggleButton` then appears nowhere, so it goes to `docs/showcase-exceptions.toml` with that reason.

**The theme settings go into the left panel, above the page list, each with a label.** They configure what the whole window shows. A label says what each control is: Theme, Mode, Icon set.

- The Sidebar's own `header` slot (sidebar/mod.rs:276) holds them.
- The icon rail has no room for them, so they are hidden there; expanding the panel brings them back.
- While they are hidden, the command palette still offers the presets and the colour modes, and the Theme menu offers the colour modes (chrome.rs:39-47).

**The toolbar keeps the actions:** Command Palette, Reload Theme and Preferences. It stays because it is where the showcase demonstrates `geometry::toolbar`, the connector's public toolbar builder.

**The title names the application and its version:** `native-theme-gpui 0.5.9 showcase`.

- One string serves both the title-bar label (today `native-theme showcase — {preset} ({mode})`) and the window title the OS shows (today `Native Theme – GPUI Showcase, v…`, main.rs:929).
- The preset and mode stay in the status bar.
- The version leaves the status bar, because the title carries it.

**The Sidebar's overlapping icons** are a layout defect. The fix is to diagnose it against upstream's `SidebarMenuItem` layout and correct the cause.

## 9. Decisions

- **D1.** An unstated sizing value is `None` from the resolved model, through the connector, to the toolkit's default. A computation that needs one uses the value actually drawn.
- **D2.** Native presets state every sizing value platform-facts documents for their platform, and a gate enforces this. The audit covers the OS readers' constants too. A range is not a value. A per-context value is stated for the context the widget matches (the desktop pointer, for the input device), and it is `None` if none matches.
- **D3.** Padding is per side:
  - the axis keys stay, as shorthand for equal sides;
  - a side key wins over its axis key;
  - the resolved model has four optional sides.
- **D4.** Status-bar padding is researched from upstream toolkit sources for all four platforms and recorded in platform-facts §2.14. Where no source states it, it is **(none)**.
- **D5.** The panel toggles live in the status bar. The left one collapses its panel to the rail; the right one hides its panel. Both are our own buttons, with icons from the chosen set.
- **D6.** The theme settings live, labelled, in the Sidebar header, and are hidden in the rail. The toolbar holds the actions.
- **D7.** The title-bar label and the OS window title are both `native-theme-gpui <version> showcase`.
- **D8.** No new public API is added. These existing items change:
  - the resolved border's `padding_horizontal`/`padding_vertical` become `padding`, with four `Option<f32>` sides;
  - `toolbar.bar_height` becomes `Option<f32>`;
  - `native_theme_gpui::dialog_content_padding` returns the dialog's per-side padding;
  - iced's `button_padding` and `input_padding` keep their `Padding` return type.

  These are breaking changes, which are allowed before 1.0, and they get no migration guide.
