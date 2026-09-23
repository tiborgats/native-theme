# v0.5.9: unstated sizes, and the showcase's chrome UX — spec

Rationale: `todo_v0.5.9_unstated-sizes-and-chrome-ux-rationale.md` (decisions D1–D8).
Plan: `todo_v0.5.9_unstated-sizes-and-chrome-ux-plan.md`.

## 1. Model (native-theme)

### 1.1 Padding is per side (D3)

**Unresolved** (`WidgetBorderSpec`, model/border.rs).

- `padding_horizontal` and `padding_vertical` (TOML `padding_horizontal_px`, `padding_vertical_px`) stay. Each is shorthand for two equal sides.
- Four new fields, each `Option<f32>`, state one side:
  - `padding_top` (TOML `padding_top_px`)
  - `padding_right` (TOML `padding_right_px`)
  - `padding_bottom` (TOML `padding_bottom_px`)
  - `padding_left` (TOML `padding_left_px`)
- The overlay list (border.rs:81) gains them.
- `docs/property-registry.toml`'s `Border` structure gains the four keys.
- Platform-facts' conventions paragraph on padding says how an asymmetric row maps to them.

**Resolved.** `ResolvedBorderSpec` replaces `padding_horizontal` and `padding_vertical` with `padding: ResolvedPadding`. `ResolvedPadding` has four fields, `top`, `right`, `bottom` and `left`, each `Option<f32>`.

**Rule.** A side resolves to its side key if that is present, otherwise to its axis key, otherwise to `None`. One function implements the rule, and both the resolver and the gate (§1.5) use it. It is applied at resolution, so the result does not depend on the order in which the preset and the reader were merged.

**`defaults.border`.** It shares `ResolvedBorderSpec`, but its unresolved spec has no padding (model/border.rs:12-18). Its sides are all `None`.

### 1.2 Other fields that may be unstated

A resolved sizing field becomes `Option<f32>` when at least one of the four native platforms leaves it unstated, meaning platform-facts marks it **(none)** or gives no row for it:

- `toolbar.bar_height` qualifies now: KDE's entry is "(none), sizes to content" (§2.13).
- The audit (plan Task 1) lists every other sizing field that some native platform leaves unstated while a native preset or reader states a value for it. List `row_height` on KDE is one: "(none), sizes to content" (§2.15).
- The controller rules on each one. A field joins only if a native platform really leaves it unstated.

### 1.3 Semantics

- `None` means the platform states no value. `Some(0.0)` means it states zero.
- An absent TOML key is `None`.
- The resolver drops its fallbacks to `0.0`:
  - `unwrap_or_default` at `resolve/validate_helpers.rs:283-284` and `:337-338`;
  - the literal `0.0` for the defaults border at `:587-588`.
- Validation and every internal consumer handle `None` without substituting a number. Test fixtures such as `model/resolved.rs:297` change type with the fields.
- Stale rules go:
  - the `defaults.border.padding_*` entries at `docs/inheritance-rules.toml:58-59`, whose heuristic model/border.rs:14-18 says was removed;
  - their two mirrors in the `implemented_targets` test list, `resolve/inheritance.rs:350-351`.

### 1.4 Native presets and readers state what their platform documents (D2)

The native presets are `kde-breeze`, `adwaita`, `macos-sonoma` and `windows-11`, each with its `-live` twin, in both variants.

Each preset states every sizing value that platform-facts documents for its platform, using these rules:

- **Axis and side keys:** an equal pair is written with its axis key; unequal sides are written with side keys.
- **Measured values:** a value marked "(measured)" is stated, and its comment says "measured".
- **Ranges:** a value given as a range ("~8–10px") is not stated.
- **Per-context values:** a value given per context is stated for the context the connector's widget matches. For the input device, that is the desktop pointer, i.e. the mouse. Its comment names the context. Where no context clearly matches, the audit ruling decides, and if none matches the value is not stated.

These values are known now; the audit (Task 1) completes the list:

| Platform (presets) | Widget | top / right / bottom / left | Source |
|---|---|---|---|
| KDE (kde-breeze, -live) | dialog | 10 / 10 / 10 / 10 | §2.22, `Layout_TopLevelMarginWidth` ✅ |
| KDE | toolbar | 0 / 6 / 0 / 6 | §2.13, `ToolBar_ItemMargin` |
| KDE | toolbar `bar_height` | absent | §2.13, "(none), sizes to content" |
| GNOME (adwaita, -live) | toolbar | 0 / 6 / 0 / 6 | §2.13 |
| GNOME | dialog | 32 / 24 / 24 / 24 | §2.22 |
| macOS (macos-sonoma, -live) | toolbar | 0 / 8 / 0 / 8 | §2.13, measured |
| macOS | dialog | 20 / 20 / 20 / 20 | §2.22, measured |
| Windows (windows-11, -live) | toolbar | 0 / 0 / 0 / 4 | §2.13 |
| Windows | dialog | 24 / 24 / 24 / 24 | §2.22, `ContentDialogPadding` ✅ |
| Windows | button | 5 / 11 / 6 / 11 | §2.3 |
| Windows | input | 5 / 6 / 6 / 10 | §2.4 |
| Windows | tooltip | 6 / 9 / 8 / 9 | §2.7, `ToolTipBorderPadding=9,6,9,8` |
| Windows | tab | 3 / 4 / 3 / 8 | §2.11 |
| Windows | menu | 4 / 11 / 5 / 11 | §2.6, mouse context |
| Windows | combo_box | 5 / 0 / 7 / 12 | §2.24 |

Before Task 4 writes anything, the audit adds every other mismatch, and the controller rules on each one:

- a documented value that is missing;
- a value stated where the facts say (none);
- a value that differs from the facts;
- a range;
- a per-context value.

The OS readers' constants (`kde/metrics.rs`, `windows.rs`, `macos.rs`, the GNOME reader) are audited and ruled the same way. Every ruling corrects whichever side its source proves wrong: the preset, the reader, or platform-facts itself. One mismatch is already known: `kde/metrics.rs:20` gives the KDE button a vertical padding of 5 ("Breeze measured frame+margin"), where §2.3 gives 6.

### 1.5 Gate

A test in `native-theme` holds one row per (platform, widget), covering every padding the platform-facts table documents for that platform, plus the fields that §1.2 makes optional.

- Each row gives the expected four sides, or the field value, as `Some(v)` or `None`, together with the platform-facts line it comes from.
- For each row, the test reads the value stated in both variants of the platform's two presets. It applies §1.1's side-over-axis function and asserts the result.
- It reads the parsed preset rather than resolving it. The `-live` presets are geometry-only merge bases that never resolve on their own (presets.rs:51), and these fields have no inheritance (`inheritance-rules.toml:66`, `:386`), so the stated value is the value that resolves.

**Discrimination proof:** remove `kde-breeze`'s dialog padding. The test fails and names the preset, the variant, the field and the citation.

### 1.6 Status-bar padding (D4)

Research each platform's native status bar from upstream sources:

- **KDE:** Qt `QStatusBar` with Breeze.
- **GNOME:** the GTK version and theme sources that platform-facts' GNOME column uses.
- **Windows:** the Win32 status bar and WinUI.
- **macOS:** AppKit, which has no status-bar widget.

Append per-side padding rows to platform-facts §2.14, with citations. A platform with no stated value is **(none)**, with the reason. Where a platform states a value, the gate gains the row and the native preset states it.

## 2. Connectors

### 2.1 gpui (`native-theme-gpui`)

- Every `geometry::*` builder that pads sets each side (`pt`, `pr`, `pb`, `pl`) only when that side is `Some`.
- `geometry::toolbar` sets `min_h` only when `bar_height` is `Some`.
  - It refines an application's own row, which has no toolkit default. So an unstated side or height leaves that row to the application, and the showcase adds none of its own.
  - With a macOS, KDE or GNOME theme every toolbar side is stated. With Windows, its right side is 0, as stated.
- A computation that uses a padding uses the padding actually drawn. That is the stated side; where no side is stated, it is upstream's own padding for that widget at that size, read at its line and mirrored as a named constant with the citation, the way `HANDLE_PADDING` is. The computations are:
  - `control_height`: text height + top + bottom;
  - `tooltip_content`: `max_width` − left − right − 2 × `TOOLTIP_BORDER`.
- `dialog_content_padding` (lib.rs:~470) returns the dialog's `ResolvedPadding`.
- These all follow:
  - the builders' docs;
  - the tests in `geometry.rs`, covering both stated and unstated sides;
  - the seams test;
  - the README builder table;
  - the showcase's `GEOMETRY_NOTES` and infos that describe padding or `bar_height`.

### 2.2 iced (`native-theme-iced`)

- `button_padding` and `input_padding` (lib.rs:275, :286) keep returning `Padding`. Each side is the stated side, or iced's own public default for that widget where it is unstated: `iced_widget::button::DEFAULT_PADDING` (button.rs:462) and `iced_widget::text_input::DEFAULT_PADDING` (text_input.rs:125).
- Their docs say so, and the iced showcase follows.

## 3. Showcase chrome (gpui example)

### 3.1 Status bar (D5)

Left to right, the status bar holds:

- the **left-panel toggle**;
- the environment text;
- the title of the shown info;
- the **inspector toggle**.

The version leaves the status bar (D7).

Each toggle:

- is built by a `demo::` helper and reports itself;
- is a ghost icon Button with `IconName::PanelLeft` or `IconName::PanelRight`, taken from the chosen set through `chrome_icon`. Where the set lacks it, the toggle shows its tooltip text as a label, as toolbar buttons already do;
- is `selected` while its panel is open. For the left toggle, open means expanded rather than collapsed to the rail;
- dispatches `ToggleSidebar` or `ToggleInspector`;
- has a tooltip naming its action and key: Ctrl+B or Ctrl+I.

`ToggleSidebar` still collapses the Sidebar to its icon rail.

### 3.2 Toolbar (D6)

- The toolbar holds Command Palette, Reload Theme and Preferences.
- These leave it: the preset Combobox, the colour-mode ToggleGroup, the icon-set Select, the SidebarToggleButton and the inspector button.
- `SidebarToggleButton` goes into `docs/showcase-exceptions.toml`, with the reason from rationale §8.

### 3.3 Sidebar header (D6)

`Sidebar::header` (sidebar/mod.rs:276) holds three rows, each a label above its control:

- **Theme**: the preset Combobox.
- **Mode**: the colour-mode ToggleGroup.
- **Icon set**: the icon-set Select.

The labels and layout follow these rules:

- Each label is built by `demo::label` and reports itself.
- The controls take the panel's width.
- The spacing comes from `geometry::widget_gap` and `container_margin`.
- At `NAV_WIDTH` every control fits without overflow, and a test asserts it. If a control doesn't fit, `NAV_WIDTH` grows to what it needs, and stays a named constant whose comment says why.
- The header is not rendered while the Sidebar is collapsed to the rail.

### 3.4 Title (D7)

One constant holds `concat!(CARGO_PKG_NAME, " ", CARGO_PKG_VERSION, " showcase")`. Both the title bar's label and `window.set_window_title` (main.rs:929) use it.

### 3.5 Sidebar icon overlap

A Sidebar item's icon and label must not intersect, expanded or collapsed.

- A windowed test measures their bounds for every page item under two icon sets: gpui's built-in set, and one freedesktop set where one is installed.
- It asserts no overlap, and it must fail before the fix.
- The fix addresses the cause the diagnosis finds.

### 3.6 Info and tests

Every moved or new chrome element carries true info, under the existing rules:

- claims cite the lines they read;
- painted values are shown;
- notes hold under reduced motion;
- ids are unique.

These tests follow the controls to their new places:

- the toolbar tests;
- the chrome-bar hover test;
- the status-bar tests;
- the preset, mode and icon-set tests.

`the_toolbar_is_the_models_toolbar` checks `min_h` only where `bar_height` is stated.

## 4. Docs

- **CHANGELOG `[Unreleased]`.** It records:
  - the resolver no longer inventing `0.0`;
  - per-side padding;
  - the `Option` sizing fields;
  - the preset and reader values that were corrected;
  - `dialog_content_padding`'s new return type;
  - the showcase chrome changes.

  The entries the showcase-app plan added that describe the toolbar's contents, the status bar's version or the title are updated to the new arrangement.
- **Connector README.** Where it describes the showcase's chrome, it is updated.
- **`docs/todo.md`.**
  - The existing screenshot-review item is updated. It also covers the new chrome and the colour-scheme presets' new toolkit-default paddings.
  - New items are appended for any audit row left open, and for a machine-readable platform-facts, so that every native value can be gated (rationale §6).
- **These three documents** are archived when done.
