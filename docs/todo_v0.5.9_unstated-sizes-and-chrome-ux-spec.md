# v0.5.9: unstated sizes, and the showcase's chrome UX — spec

Rationale: `todo_v0.5.9_unstated-sizes-and-chrome-ux-rationale.md` (decisions D1–D8).
Plan: `todo_v0.5.9_unstated-sizes-and-chrome-ux-plan.md`.

## 1. Model (native-theme)

### 1.1 Which fields may be unstated

A resolved sizing field becomes `Option<f32>` if either of these is true:
- for at least one of the four native platforms, platform-facts marks it **(none)** or gives no row;
- the platform's documented value is asymmetric (D3).

The following fields qualify now:
- `padding_horizontal` and `padding_vertical` of `ResolvedBorderSpec`. §2.2, §2.5, §2.16, §2.26 and §2.27 mark them (none), and §2.14 has no row. `ResolvedBorderSpec` also serves `defaults.border`, whose unresolved spec has no padding at all (model/border.rs:12-18). There they become `None`.
- `toolbar.bar_height`. KDE's is "(none), sizes to content" (§2.13).

The audit (plan Task 1) lists every other sizing field that some native platform leaves unstated while a native preset or reader states a value for it. The controller rules on each; a field joins only if a native platform really leaves it unstated.

### 1.2 Semantics

- `None` means the platform states no value. `Some(0.0)` means it states zero.
- TOML is unchanged: an absent key means `None`.
- The resolver drops its fallbacks to `0.0`: `unwrap_or_default` at `resolve/validate_helpers.rs:283-284` and `:337-338`, and the literal `0.0` for the defaults border at `:587-588`.
- Validation and every internal consumer handle `None` without substituting a number. Test fixtures, such as `model/resolved.rs:297`, change type along with the fields.
- Stale documentation is removed:
  - The `defaults.border.padding_*` rule at `docs/inheritance-rules.toml:58-59` ("0.0 when border.line_width or corner_radius present") describes a heuristic that model/border.rs:14-18 says is gone.
  - The two entries mirroring it in the `implemented_targets` test list (`resolve/inheritance.rs:350-351`) go too.

### 1.3 Native presets state what their platform documents (D2)

The native presets are `kde-breeze`, `adwaita`, `macos-sonoma` and `windows-11`, each with its `-live` twin and in both variants. Each one states every sizing value platform-facts documents for its platform, as the documented number, provided the value is symmetric per axis (D3). A value platform-facts marks "(measured)" is stated, and its TOML comment says "measured".

Values known now:

| Platform (presets) | Field | Value | Source |
|---|---|---|---|
| KDE (kde-breeze, -live) | `dialog.border.padding_horizontal` / `_vertical` | 10 / 10 | §2.22, `Layout_TopLevelMarginWidth` ✅ |
| KDE | `toolbar.border.padding_horizontal` / `_vertical` | 6 / 0 | §2.13, `ToolBar_ItemMargin` |
| KDE | `toolbar.bar_height` | absent | §2.13, "(none), sizes to content" |
| GNOME (adwaita, -live) | `toolbar.border.padding_horizontal` / `_vertical` | 6 / 0 | §2.13 |
| GNOME | `dialog.border.padding_horizontal` / `_vertical` | 24 / absent | §2.22; the vertical value is 32 top / 24 bottom, so asymmetric (D3) |
| macOS (macos-sonoma, -live) | `toolbar.border.padding_horizontal` / `_vertical` | 8 / 0 | §2.13, measured |
| macOS | `dialog.border.padding_horizontal` / `_vertical` | 20 / 20 | §2.22, measured |
| Windows (windows-11, -live) | `toolbar.border.padding_horizontal` / `_vertical` | absent / 0 | §2.13; the horizontal value is 4 left / 0 right, so asymmetric (D3) |
| Windows | `dialog.border.padding_horizontal` / `_vertical` | 24 / 24 | §2.22, `ContentDialogPadding` ✅ |

Before Task 4 writes anything, the audit adds every other mismatch it finds, and the controller rules on it:
- a documented value that is missing;
- a value stated where the facts say (none);
- a value that differs from the documented one.

A mismatch in an OS reader's constant (`kde/metrics.rs`, `windows.rs`, `macos.rs`, the GNOME reader) is ruled the same way. Every ruling corrects whichever side its source proves wrong: the preset, the reader, or platform-facts itself. An example is `kde/metrics.rs:20`, which gives the button a vertical padding of 5 ("Breeze measured"), where §2.3 gives 6.

### 1.4 Gate

A test in `native-theme` holds one table row per (platform, widget.field) for every field in §1.1. Each row gives the expected `Some(v)` or `None` and the platform-facts line it comes from. For each row, the test reads the value stated in both variants of that platform's two presets, and asserts it. It reads the parsed preset rather than resolving it: the `-live` presets are geometry-only merge bases that never resolve on their own (presets.rs:51), and these fields have no inheritance (`inheritance-rules.toml:66`, `:386`), so the stated value is the one that resolves.

Discrimination proof: remove `kde-breeze`'s dialog padding. The test must fail, naming the preset, the variant, the field and the citation.

### 1.5 Status-bar padding (D4)

Research each platform's native status bar from upstream sources:

- **KDE:** Qt `QStatusBar` together with Breeze.
- **GNOME:** the GTK version and theme sources that platform-facts' GNOME column uses.
- **Windows:** the Win32 status bar and WinUI.
- **macOS:** AppKit, which has no status bar widget.

Append a `border.padding_*` row to platform-facts §2.14, citing each source. A platform with no stated value is **(none)**, with the reason. Where a platform states a symmetric value, the gate gets that row and the native preset states the value.

## 2. Connectors

### 2.1 gpui (`native-theme-gpui`)

- Every `geometry::*` builder that sets a padding sets it only when the value is `Some`.
- `geometry::toolbar` sets `min_h` only when `bar_height` is `Some`. It refines an application's own row, which has no toolkit default, so an unstated padding or height leaves that row to the application. The showcase adds none of its own: on Windows, whose horizontal toolbar padding is asymmetric (D3), the toolbar's items reach its edges.
- A computation from a padding uses the padding that is actually drawn:
  - the stated value when there is one;
  - otherwise upstream's own padding for that widget at that size, read at its line and mirrored as a named constant with the citation, the way `HANDLE_PADDING` is.

  This covers `control_height`'s `2 × padding_vertical` and `tooltip_content`'s width.
- `dialog_content_padding` (lib.rs:~470) returns `Option<f32>`.
- The builders' docs, the tests in `geometry.rs` (both `Some` and `None`), the seams test, the README builder table, and the showcase's `GEOMETRY_NOTES` and infos that describe padding or `bar_height` all follow.

### 2.2 iced (`native-theme-iced`)

- `button_padding` and `input_padding` (lib.rs:275, :286) return `Option<Padding>`. `None` means the application keeps iced's own default.
- Their docs say so, and the iced showcase follows.

## 3. Showcase chrome (gpui example)

### 3.1 Status bar (D5)

The status bar reads, left to right:

- the **left-panel toggle**;
- the environment text;
- the title of the shown info;
- the **inspector toggle**.

The version leaves the status bar (D7).

Each toggle:

- is built by a `demo::` helper and reports itself;
- is a ghost icon Button with `IconName::PanelLeft` or `IconName::PanelRight`, taken from the chosen set through `chrome_icon`. Where the set lacks it, the toggle shows its tooltip text as a label, as toolbar buttons already do;
- is `selected` while its panel is open (for the left toggle, that means expanded rather than collapsed to the rail);
- dispatches `ToggleSidebar` or `ToggleInspector`;
- has a tooltip naming the action and its key: Ctrl+B or Ctrl+I.

`ToggleSidebar` still collapses the Sidebar to its icon rail.

### 3.2 Toolbar (D6)

- The toolbar holds Command Palette, Reload Theme and Preferences.
- The preset Combobox, the colour-mode ToggleGroup, the icon-set Select, the SidebarToggleButton and the inspector button leave it.
- `SidebarToggleButton` goes into `docs/showcase-exceptions.toml` with the reason from rationale §7.

### 3.3 Sidebar header (D6)

`Sidebar::header` (sidebar/mod.rs:276) holds three rows, each a label above its control:

- **Theme**: the preset Combobox.
- **Mode**: the colour-mode ToggleGroup.
- **Icon set**: the icon-set Select.

Rules for the header:

- Each label is built by `demo::label` and reports itself.
- The controls take the panel's width.
- The spacing comes from `geometry::widget_gap` and `container_margin`.
- At `NAV_WIDTH` every control fits without overflowing, and a test asserts this. If they do not fit, `NAV_WIDTH` grows to what they need, and it stays a named constant whose comment says why.
- The header is not rendered while the Sidebar is collapsed to the rail.

### 3.4 Title (D7)

A single constant holds `concat!(CARGO_PKG_NAME, " ", CARGO_PKG_VERSION, " showcase")`. Both the title bar's label and `window.set_window_title` (main.rs:929) use it.

### 3.5 Sidebar icon overlap

A Sidebar item's icon and label must not intersect, whether the Sidebar is expanded or collapsed.

A windowed test measures their bounds for every page item under two icon sets: gpui's built-in set, and one freedesktop set where one is installed. It asserts that they don't overlap. It must fail before the fix.

The fix addresses the cause the diagnosis finds.

### 3.6 Info and tests

Every moved or new chrome element carries true info under the existing rules:

- claims cite the lines that were read;
- painted values are shown;
- notes hold under reduced motion;
- ids are unique.

The toolbar, chrome-bar hover, status-bar and preset/mode/icon-set tests follow the controls to their new places. `the_toolbar_is_the_models_toolbar` checks `min_h` only where `bar_height` is stated.

## 4. Docs

- CHANGELOG `[Unreleased]` records:
  - the resolver no longer inventing `0.0`;
  - the `Option` sizing fields;
  - the preset and reader values that were corrected;
  - the changed return types;
  - the showcase's chrome changes.
- `docs/todo.md` gets these appended:
  - per-side padding (D3);
  - any audit row left open;
  - a screenshot-review item for the changed chrome and for the colour-scheme presets' new toolkit-default paddings.
- The three documents are archived when done.
