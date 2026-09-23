# v0.5.9: unstated sizes, and the showcase's chrome UX — rationale

Status: proposed, 2026-09-23.
Spec: `todo_v0.5.9_unstated-sizes-and-chrome-ux-spec.md`.
Plan: `todo_v0.5.9_unstated-sizes-and-chrome-ux-plan.md`.

## 1. What the maintainer saw

These observations are from the gpui showcase, on KDE, with the system theme:

1. The status bar's text touches the window's left and right edges.
2. The About dialog's content touches its frame.
3. The Toggle Inspector button (at the right end of the toolbar) and the Sidebar toggle (at its left end) are hard to find where they are.
4. The Sidebar's icons overlap.
5. The toolbar's preset, colour-mode and icon-set controls have no labels, and look wrong there.
6. The window title carries the preset and mode. It should carry the `native-theme-gpui` version instead.

Items 3–6 are showcase design. Items 1 and 2 are not: they come from the library, and they are a correctness problem.

## 2. Why the status bar and the About dialog have no spacing

`geometry::status_bar` and `geometry::dialog` set the widget's padding from `<widget>.border.padding_{horizontal,vertical}`. For the KDE theme both values resolve to `0.0`:

- **Status bar:** platform-facts §2.14 has no padding row, so nothing documents a status-bar padding, and no preset states one.
- **Dialog:** platform-facts §2.22 documents KDE's dialog margin as 10px (`Layout_TopLevelMarginWidth`, breezemetrics.h ✅). It also gives Windows 24px (`ContentDialogPadding` ✅), macOS about 20px (measured) and GNOME 24px horizontally. No preset states a dialog padding: `adwaita.toml`'s `[light.dialog.border]` holds only a corner radius.
- **The resolver** fills a missing widget padding with `0.0` and reports nothing (native-theme `resolve/validate_helpers.rs:283-284`, `:337-338`).
- **The connector** then applies that 0, and it overrides the toolkit's own padding. gpui-component's StatusBar, for example, has `px_2 py_1` (status_bar.rs:89-90).

The same mechanism explains two findings from Task 7:

- The toolbar padding is 0 on every preset. §2.13 documents KDE 6, GNOME 6, macOS 8 (measured), and Windows 4 left / 0 right.
- KDE's `toolbar.bar_height = 40` has no source. §2.13 says KDE's toolbar "sizes to content".

## 3. The principle

The goal: when a native theme is selected, look and feel native, as closely as the data allows. The rule that outranks everything is never to invent a value.

For a sizing value the platform does not state, the pipeline has four options:

| Option | What the user sees | True? |
|---|---|---|
| Fill in `0.0` (today) | Content touching its frame, a look no platform has | No. `0` is a statement ("no padding") that nothing makes. |
| Fill in a stand-in number (today's KDE `bar_height = 40`) | Something plausible | No. It is an invented value, and it hides the gap for good. |
| Require every theme to state it (a resolution error when it's missing) | Nothing different for native presets | Forces every user theme and colour-scheme preset to invent the numbers its author does not have. |
| Keep it **absent**, so the toolkit's own default stands | The toolkit's own look for that one property | Yes. It says nothing the platform did not say, and the gap stays visible, so it can be counted and closed. |

Only the last option is honest. It is also the closest to native that the data allows, because a toolkit's default is a considered design value and a `0` is not.

The principle cuts both ways. **Where the platform does state a value, the native preset must carry it.** A native preset that omits a documented value, as `kde-breeze` omits its dialog margin, has a defect. A gate must catch it; the resolver must not paper over it.

So the long-term design is:

1. **The model can say "not stated".** A resolved sizing field that some native platform leaves unstated becomes `Option<f32>`. `None` means the platform states no value. `0.0` means it states zero, as KDE's toolbar vertical padding does.
2. **The resolver stops inventing.** A missing value stays `None`.
3. **Connectors apply only what is stated.** A builder refines a property only when the value is `Some`, so otherwise the toolkit's own default stands:
   - gpui-component's StatusBar keeps `px_2 py_1`;
   - an application-drawn toolbar with no stated height sizes to its content, which is KDE's native behaviour.

   Where a builder computes something from a padding (a tooltip's text width, a control's height), it uses the padding that is actually drawn. That is the stated one, or upstream's own where none is stated, read at upstream's line.
4. **Native presets state everything their platform documents.** A gate checks the resolved native presets against a table of platform-facts values.

## 4. The platform's values come from two places

A native theme's sizes reach the model two ways:

- from its preset TOML (`kde-breeze`, `kde-breeze-live`, and so on);
- from constants in its OS reader (`kde/metrics.rs`, `windows.rs`).

Both must agree with platform-facts, and they already disagree in places. For example, `kde/metrics.rs:20` gives the button a vertical padding of 5 ("Breeze measured"), while §2.3 gives KDE `Button_MarginWidth` = 6 for it.

The audit (plan Task 1) therefore covers both the presets and the readers. The gate covers the presets. The readers are exercised only on their own OS, and a mismatch in them becomes a ruling the audit records.

## 5. What the model cannot yet say

Two documented paddings are asymmetric:

- Windows toolbar: 4 left, 0 right.
- GNOME dialog: 32 top, 24 bottom.

The model has one value per axis. Picking one side, or an average, would invent a value. So these stay `None`, the toolkit's default stands, and `docs/todo.md` records per-side padding as a model extension. It is not done here: it would change the fields of every widget, and nothing else needs it yet.

## 6. Colour-scheme presets

Catppuccin, Nord, Dracula and the rest are colour schemes, not platforms:

- A sizing value they state stays as it is.
- None of them states a toolbar, dialog or status-bar padding. Those were `0.0` and become `None`, so the toolkit's default replaces a zero nobody chose.

That is a visible change, and a correction.

## 7. The showcase's chrome UX

**The panel toggles move to the status bar, at the edge of the panel they control.** This is Zed's pattern:

- the left-panel toggle sits at the status bar's left end;
- the inspector toggle sits at its right end;
- each is a small ghost icon button, shown selected while its panel is open, with a tooltip naming the action and its key.

The alternatives were weaker:

- The toolbar is for acting on the content.
- VS Code puts layout toggles in the title bar. That places them far from the panels, and the title bar here already holds the menus.

**The left panel collapses to its icon rail** (the maintainer's decision). **The inspector hides fully**, as it does today.

**Both toggles are our own Buttons, not upstream's `SidebarToggleButton`.** `SidebarToggleButton` draws a hardcoded icon (its Button is built at sidebar/mod.rs:313, and the icon is chosen at :349-360: `PanelLeftClose`/`PanelLeftOpen`), so it would show gpui-component's icon whatever set is chosen, and the maintainer's rule forbids mixing icon sets. Our buttons use `IconName::PanelLeft` and `IconName::PanelRight`, taken from the chosen set like the rest of the chrome. The selected state, not a changing icon, shows whether the panel is open. `SidebarToggleButton` then appears nowhere, so it goes to `docs/showcase-exceptions.toml` with that reason.

**The theme settings go into the left panel, above the page list, each with a label.** They configure what the whole window shows, and a label says what each control is: Theme, Mode, Icon set. The Sidebar's own `header` slot (sidebar/mod.rs:276) holds them. The icon rail has no room for them, so they are hidden there, and expanding the panel brings them back. Meanwhile the command palette still offers the presets and the colour modes, and the Theme menu offers the colour modes (chrome.rs:39-47).

**The toolbar keeps the actions:** Command Palette, Reload Theme, Preferences.

**The title names the application and its version:** `native-theme-gpui 0.5.9 showcase`. The same string is used in two places:

- the title-bar label, which today reads `native-theme showcase — {preset} ({mode})`;
- the window title the OS shows, which today reads `Native Theme – GPUI Showcase, v…` (main.rs:929).

The preset and mode stay in the status bar. The version leaves the status bar, because the title now carries it.

**The Sidebar's overlapping icons** are a layout defect. The fix starts by diagnosing it against upstream's `SidebarMenuItem` layout, then fixes the cause.

## 8. Decisions

- **D1.** An unstated sizing value is `None` all the way through: resolved model, connector, toolkit default. A computation that needs such a value uses the value that is actually drawn.
- **D2.** Native presets state every sizing value that platform-facts documents for their platform, and a gate enforces this. The audit covers the OS readers' constants too.
- **D3.** Asymmetric documented paddings stay `None`. Per-side padding is recorded as a model extension.
- **D4.** Status-bar padding is researched from upstream toolkit sources for all four platforms and recorded in platform-facts §2.14. Where no source states it, it is **(none)**.
- **D5.** Panel toggles live in the status bar. The left one collapses the panel to its rail; the right one hides its panel. Both are our own buttons with icons from the chosen set.
- **D6.** Theme settings live, labelled, in the Sidebar header, and are hidden in the rail. The toolbar holds the actions.
- **D7.** The title-bar label and the OS window title are both `native-theme-gpui <version> showcase`.
- **D8.** Nothing here adds public API. Resolved sizing fields change from `f32` to `Option<f32>`, and so do the return types of `native_theme_gpui::dialog_content_padding` and of iced's `button_padding` and `input_padding`. These are breaking changes, allowed before 1.0, with no migration guide.
