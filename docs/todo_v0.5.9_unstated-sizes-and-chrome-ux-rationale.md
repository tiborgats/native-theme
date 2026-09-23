# v0.5.9: unstated sizes, and the showcase's chrome UX — rationale

Status: proposed, 2026-09-23.
Spec: `todo_v0.5.9_unstated-sizes-and-chrome-ux-spec.md`.
Plan: `todo_v0.5.9_unstated-sizes-and-chrome-ux-plan.md`.

## 1. What the maintainer saw

The gpui showcase, on KDE with the system theme:

1. The status bar's text touches the window's left and right edges.
2. The About dialog's content touches its frame.
3. The Toggle Inspector button (at the toolbar's right end) and the Sidebar toggle (at its left end) are in unintuitive places.
4. The Sidebar's icons overlap.
5. The preset, colour-mode and icon-set controls in the toolbar have no labels, and look wrong there.
6. The window title carries the preset and mode. It should carry the `native-theme-gpui` version instead.

Items 3–6 are showcase design. Items 1 and 2 are not. They come from the library, and they are a correctness problem.

## 2. Why the status bar and the About dialog have no spacing

`geometry::status_bar` and `geometry::dialog` set the widget's padding from `<widget>.border.padding_{horizontal,vertical}`. For the KDE theme, both values resolve to `0.0`:

- **The status bar.** No platform documents a status-bar padding: platform-facts §2.14 has no padding row. So no preset states one.
- **The dialog.** Platform-facts §2.22 documents KDE's dialog margin as 10px (`Layout_TopLevelMarginWidth`, breezemetrics.h ✅). Windows is 24px (`ContentDialogPadding` ✅), and macOS about 20px measured. Only `adwaita.toml` states a dialog padding. `kde-breeze.toml` does not.
- **The resolver.** It does not report the missing value. Border padding "defaults to `0.0` and is not extracted via `require()`" (native-theme `resolve/validate_helpers.rs:521`, `:283`, `:337`).
- **The connector.** It then applies that 0, and the 0 overrides the toolkit's own padding: gpui-component's StatusBar has `px_2 py_1`, status_bar.rs:89-90.

The same mechanism is behind two findings Task 7 recorded:

- The toolbar padding is 0 on every preset, where §2.13 documents KDE 6, GNOME 6, macOS 8 (measured) and Windows 4 left / 0 right.
- KDE's `toolbar.bar_height = 40` has no source, where §2.13 says KDE's toolbar "sizes to content".

## 3. The principle

The goal is a native look and feel when a native theme is selected, as close as the data allows. The rule that outranks everything is never to invent a value.

Take a sizing value a platform does not state. There are three things the pipeline can do with it:

| Choice | What the user sees | Is it true? |
|---|---|---|
| Put in `0.0` (today) | The content touches the frame, a look no platform has | No. `0` is a statement ("no padding"), and nothing makes it. |
| Put in a stand-in number (today's KDE `bar_height = 40`) | Something plausible | No. It is an invented value, and it hides the gap for good. |
| Keep it **absent**, so the toolkit's own default stands | The toolkit's look for that one property | Yes. It says nothing the platform did not say, and the gap stays visible, so it can be counted and closed. |

Only the third choice is honest. It is also the closest to native that the data allows: the toolkit's default is a considered design value, which a `0` is not.

The principle cuts both ways. **Where the platform does state a value, the native preset must carry it.** A native preset that omits a documented value, like `kde-breeze`'s dialog margin, is a defect. A gate must catch it, and the resolver must not paper over it.

So the long-term design has four parts:

1. **The model can say "not stated".** Every sizing field that some native platform leaves unstated resolves to `Option<f32>`. `None` means the platform states no value. `0.0` means it states zero, as KDE's toolbar vertical padding does.
2. **The resolver stops inventing.** A missing padding stays `None`. It no longer becomes `0.0`.
3. **Connectors apply only what is stated.** A builder refines a property only when the value is `Some`. Otherwise the toolkit's own default stands. For gpui-component that means the StatusBar keeps `px_2 py_1`, and an application-drawn toolbar with no stated height sizes to its content, which is KDE's native behaviour.
4. **Native presets state everything their platform documents.** A gate checks the native presets' sizing values against a table that cites platform-facts. That catches both a documented value that is missing and a stated value where the platform says **(none)**.

## 4. What the model cannot yet say

Two documented paddings are asymmetric:

- Windows toolbar: 4px left, 0 right.
- GNOME dialog: 32px top, 24px bottom.

The model has one value per axis. Picking either side, or an average, would be an invented value. So each of these stays `None`, the toolkit default stands, and `docs/todo.md` records per-side padding as a model extension. It is not done here, because the fields would have to change for every widget, and nothing else needs it yet.

## 5. Community presets

Catppuccin, Nord, Dracula and the others are colour schemes, not platforms. A sizing value they state stays as it is. A value they omit is now `None`, where it used to be `0.0`, so the toolkit's default replaces a zero that nobody chose. That is a visible change: those presets gain the toolkit's own paddings. It is a correction, not a regression.

## 6. The showcase's chrome UX

**Panel toggles belong in the status bar, at the edge of the panel they control.** This is the Zed pattern:

- a left-panel button at the status bar's left end;
- an inspector button at its right end.

Each is a small ghost icon button. It looks selected while its panel is open, and its tooltip names the action and its key (Ctrl+B, Ctrl+I). A toolbar is for acting on the content, and showing or hiding a panel is not that.

- **The left panel collapses to its icon rail** (the maintainer's decision).
- **The inspector hides fully**, as it does today.

**The left toggle is our own Button, not upstream's `SidebarToggleButton`.** `SidebarToggleButton` draws a hardcoded icon (sidebar/mod.rs:313, `IconName::PanelLeftClose/Open`), so it would show gpui-component's icon whatever set is chosen. That mixes icon sets, which the maintainer's rule forbids. Both toggles take their icon from the chosen set, as the rest of the chrome does. `SidebarToggleButton` then appears nowhere, and it goes to `docs/showcase-exceptions.toml` with that reason.

**The theme settings go in the left panel, above the page list, each with a label.** They configure what the whole window shows, the way a settings column does. A label says what each control is: preset, colour mode, icon set. The Sidebar's own `header` slot (sidebar/mod.rs:276) is where they go. In the icon rail there is no room for them, so they are hidden. The Theme menu and the command palette still reach all three.

**The toolbar keeps the actions:** Command Palette, Reload Theme, Preferences.

**The title bar names the application and its version:** `native-theme-gpui 0.5.9 showcase`. The preset and mode stay in the status bar, and the version leaves the status bar because the title now carries it.

**The Sidebar's overlapping icons** are a layout defect. It will be diagnosed against upstream's `SidebarMenuItem` layout and fixed at its cause.

## 7. Decisions

- **D1.** An unstated sizing value is `None` end to end: resolved model, then connector, then toolkit default.
- **D2.** Native presets state every sizing value platform-facts documents for their platform. A gate enforces it.
- **D3.** Asymmetric documented paddings stay `None`. Per-side padding is a recorded model extension.
- **D4.** Status-bar padding is researched from upstream toolkit sources for all four platforms, and recorded in platform-facts §2.14. Where no source states it, it is **(none)**.
- **D5.** Panel toggles live in the status bar. The left one collapses to the rail, the right one hides the panel. Both are our own buttons with icons from the chosen set.
- **D6.** Theme settings live, labelled, in the Sidebar header, and are hidden in the rail. The toolbar holds the actions.
- **D7.** The title is `native-theme-gpui <version> showcase`.
- **D8.** Nothing here adds public API. The resolved sizing fields change type (`f32` becomes `Option<f32>`). That is a breaking change, allowed before 1.0, and gets no migration guide.
