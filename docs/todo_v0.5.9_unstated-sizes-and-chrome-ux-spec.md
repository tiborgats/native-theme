# v0.5.9: unstated sizes, and the showcase's chrome UX — spec

Rationale: `todo_v0.5.9_unstated-sizes-and-chrome-ux-rationale.md` (decisions D1–D9).
Plan: `todo_v0.5.9_unstated-sizes-and-chrome-ux-plan.md`.

## 1. Model (native-theme)

### 1.1 Padding is per side (D3)

**Unresolved** (`WidgetBorderSpec`, model/border.rs):

- `padding_horizontal` and `padding_vertical` are replaced by four fields: `padding_top`, `padding_right`, `padding_bottom` and `padding_left`, each `Option<f32>`.
- They merge like every other field: per field, with the overlay winning (border.rs:80-82, pipeline.rs:48-52).
- Readers set sides in code. Where one sets `padding_horizontal = Some(12.0)` today, it sets left and right instead.

**TOML:**

- `padding_top_px`, `padding_right_px`, `padding_bottom_px` and `padding_left_px` name the sides.
- `padding_horizontal_px` and `padding_vertical_px` are parse-time shorthand, setting both sides of their axis.
- A table that states an axis key and one of that axis's sides is a parse error, and the error names the table and both keys.
- Serialisation writes sides.
- `docs/property-registry.toml`'s `Border` structure lists the four side keys and the two shorthands.
- Platform-facts' padding conventions paragraph says how an asymmetric cell maps to the sides.

**Resolved (D8):**

- `ResolvedBorderSpec` is split in two:
  - `ResolvedDefaultsBorder` has `line_width`, `corner_radius`, `corner_radius_lg`, `color`, `opacity` and `shadow_enabled`.
  - `ResolvedWidgetBorder` has `color`, `corner_radius`, `line_width`, `shadow_enabled` and `padding: ResolvedPadding`.
- `ResolvedPadding` has four fields, `top`, `right`, `bottom` and `left`, each `Option<f32>`.
- The widget border no longer carries the `corner_radius_lg` and `opacity` that it resolves to `0.0` today (validate_helpers.rs:280-282, :332-334).

### 1.2 `toolbar.bar_height`

- `toolbar.bar_height` resolves to `Option<f32>`, using the existing `soft_option` mechanism (lib.rs:21-25) where it fits.
- KDE's value is "(none), sizes to content" (§2.13).
- No other sizing field changes in this plan (rationale §7). The audit's list of the others goes to `docs/todo.md`.

### 1.3 Semantics

- `None` means the platform states no value. `Some(0.0)` means it states zero.
- An absent TOML key is `None`.
- The resolver drops its fallbacks to `0.0`:
  - `unwrap_or_default` at `resolve/validate_helpers.rs:283-284` and `:337-338`;
  - the defaults border's literal `0.0` at `:587-588`;
  - the sentinel's paddings (`:47-57`).
- Defaults padding range checks (`:700-711`) go, because the defaults border has no padding. A new check requires each stated widget padding side to be ≥ 0.
- Every consumer and test follows the new types, with no substituted number. The known ones are:
  - the field-name baseline test (model/mod.rs:1738-1748);
  - the test fixture at model/resolved.rs:297;
  - proptest_roundtrip.rs:68-73;
  - the full-versus-live comparison (resolve_and_validate.rs:204-233), which then compares all four sides;
  - `native-theme-gpui`'s `dialog_content_padding` test (connectors/native-theme-gpui/src/lib.rs:1420).
- Stale rules go:
  - the `defaults.border.padding_*` entries at `docs/inheritance-rules.toml:58-59` (the heuristic that model/border.rs:14-18 records as removed);
  - their mirrors in `implemented_targets` (`resolve/inheritance.rs:350-351`).

  The comment at `inheritance-rules.toml:66` names the side keys.

### 1.4 Native themes state what their platform documents (D2)

The native presets are `kde-breeze`, `adwaita`, `macos-sonoma` and `windows-11`, each with its `-live` twin, in both variants. For padding and `toolbar.bar_height`, each states every value platform-facts documents for its platform. Each platform-facts cell is read as follows (rationale §3, §5 point 5):

| Cell | Treatment |
|---|---|
| A number | Stated. An equal pair uses the axis shorthand; unequal sides use side keys. |
| A single "(measured)" value | Stated, commented "measured". |
| A range | Not stated. |
| A per-context value | Stated for the context the widget matches (the desktop pointer, for the input device), and the comment names the context. Where no context clearly matches, a ruling decides; if none matches, not stated. |
| A derivation ("← button padding (10px)") | Stated as the derived value, and the comment cites the derivation. |
| A "(none)" saying the thing does not exist ("content provides own padding", "checkmark fills indicator") | `0`. |
| "(none) — sizes to content", "not specified", "app-defined", or a bare "(none)" | Not stated. |

The following values are known now; the audit (plan Task 1) completes the list.

| Platform (presets) | Widget | top / right / bottom / left | Source |
|---|---|---|---|
| KDE (kde-breeze, -live) | dialog | 10 / 10 / 10 / 10 | §2.22, `Layout_TopLevelMarginWidth` |
| KDE | toolbar | 0 / 6 / 0 / 6 | §2.13, `ToolBar_ItemMargin` |
| KDE | toolbar `bar_height` | not stated | §2.13, "(none), sizes to content" |
| GNOME (adwaita, -live) | toolbar | 0 / 6 / 0 / 6 | §2.13 |
| GNOME | dialog | 32 / 24 / 24 / 24 | §2.22 |
| GNOME | combo_box | 5 / 10 / 5 / 10 | §2.24, "← button padding (10px)", "← button (5px)" |
| macOS (macos-sonoma, -live) | toolbar | 0 / 8 / 0 / 8 | §2.13, measured |
| macOS | dialog | 20 / 20 / 20 / 20 | §2.22, measured |
| Windows (windows-11, -live) | toolbar | 0 / 0 / 0 / 4 | §2.13 |
| Windows | dialog | 24 / 24 / 24 / 24 | §2.22, `ContentDialogPadding` |
| Windows | button | 5 / 11 / 6 / 11 | §2.3 |
| Windows | input | 5 / 6 / 6 / 10 | §2.4 |
| Windows | tooltip | 6 / 9 / 8 / 9 | §2.7, `ToolTipBorderPadding=9,6,9,8` |
| Windows | tab | 3 / 4 / 3 / 8 | §2.11 |
| Windows | menu | 4 / 11 / 5 / 11 | §2.6, mouse context |
| Windows | combo_box | 5 / 0 / 7 / 12 | §2.24 |
| all four | popover | 0 / 0 / 0 / 0 | §2.16, "(none) — content provides own padding" |

The audit adds every other row and mismatch in scope: documented but missing, stated where not documented, different, range, per-context, derivation. It also covers the OS readers' size constants:

- `kde::metrics::populate_widget_sizing`;
- `macos_widget_defaults`;
- the WinUI3 constants of `read_widget_sizing`.

The GNOME reader states no sizes.

**Two mismatches are known already:**

- `kde/metrics.rs:20` gives the KDE button a vertical padding of 5 ("Breeze measured frame+margin"), where §2.3 gives 6.
- The Windows reader sets `toolbar.item_gap = 4.0` (windows.rs:240, :296), where §2.13 and `windows-11.toml:217` say 0.

Every ruling corrects whichever side its source proves wrong: the preset, the reader, or platform-facts.

**Colour-scheme presets** drop their unsourced `bar_height_px = 40.0` (rationale §7). Nothing else about their sizes changes here.

### 1.5 Reader constants outside the gates

- The KDE constants (`kde::metrics::populate_widget_sizing`) move out of the `kde`-feature module (lib.rs:99-100) into a function that is always compiled.
- The WinUI3 constants move into one always-compiled function, which both builds of `read_widget_sizing` call (windows.rs:186, :246).
- Only the OS reads (`GetSystemMetricsForDpi`, `index.theme`) stay behind their gates.
- `cargo test -p native-theme` with no features then sees every reader's constants.

### 1.6 Gate

A test in `native-theme` holds one row per (platform, widget). It follows the precedent of `tests/platform_facts_xref.rs`, which already checks some preset values against platform-facts. It lives inside the crate because the reader-constant functions are `pub(crate)`. The rows cover every padding and `bar_height` the platform-facts table documents for that platform, read per §1.4.

- Each row gives the expected sides or value, as `Some(v)` or `None`, with the platform-facts line it comes from.
- For each row, in both variants, the test resolves two themes and asserts the resolved value:
  - **Static:** the platform's full preset.
  - **Live:** the full preset, with the `-live` preset merged over it and then the reader's constants from §1.5, in `pipeline.rs:48-52`'s order. GNOME has no reader constants.

**Discrimination proofs:**

- Remove `kde-breeze`'s dialog padding: the static row fails, naming path, preset, variant, field and citation.
- Set a KDE reader constant wrong: the live row fails.
- State an axis key and one of its sides in one table: parsing fails.

### 1.7 Status-bar padding (D4)

Research each platform's native status bar from upstream sources:

- **KDE:** Qt `QStatusBar` with Breeze.
- **GNOME:** the GTK version and theme sources that platform-facts' GNOME column uses.
- **Windows:** the Win32 status bar and WinUI.
- **macOS:** AppKit, which has no status-bar widget.

Append per-side padding rows to platform-facts §2.14, with citations. A platform without a value gets **(none)** and its reason, read per §1.4. Where a value is stated, the gate gains the row, and the native preset states the value.

## 2. Connectors

### 2.1 gpui (`native-theme-gpui`)

**Padding:**

- Every `geometry::*` builder that pads sets each side (`pt`, `pr`, `pb`, `pl`) only when that side is `Some`.
- Where a builder cannot apply padding because upstream sets it after the refinement (input, select, combobox: geometry.rs:148-157, :444-449, "Tier U"), its doc says upstream's padding is drawn.

**Heights:**

- `control_height` goes.
- `geometry::button` and `geometry::input` set the platform's height as `min_h` together with `h_auto`, so layout adds the padding and scaled text actually drawn.
- `geometry::input_height` returns the input's minimum height, to use with `min_h`.
- A seams test proves, for a real Button and a real Input:
  - at text scale 1, the height equals the platform's minimum;
  - at a large scale, the height grows with the text and no text clips.
- If upstream prevents `h_auto` for a widget, the implementer reports it rather than working around it.

**Toolbar:** `geometry::toolbar` sets `min_h` only when `bar_height` is `Some`. The row is the application's own, so an unstated side or height leaves it to the application.

**Tooltip width:** `tooltip_content` computes `max_width` − left − right − 2 × `TOOLTIP_BORDER`. An unstated side is upstream's `px_2` (tooltip.rs:123), which is 0.5 rem, converted at the font size the connector installs as the rem (root.rs:582).

**Dialog:**

- `dialog_content_padding` (lib.rs:471) returns the dialog's `ResolvedPadding`.
- `geometry::dialog`'s doc notes that upstream uses the dialog's top padding as the gap between its sections (dialog.rs:620). So GNOME's top padding of 32 also spaces the sections 32 apart.

**Follow-through.** These all follow the changes above:

- the builders' docs;
- the `geometry.rs` tests, with stated and unstated sides;
- the seams test;
- the README builder table;
- the showcase's `GEOMETRY_NOTES` and infos that describe padding, heights or `bar_height`;
- the Theme Map rows that showed `control_height`.

### 2.2 iced (`native-theme-iced`)

- `button_padding` and `input_padding` (lib.rs:275, :286) keep returning `Padding`.
- Each side is either the stated side, or, where unstated, iced's own public default: `iced_widget::button::DEFAULT_PADDING` (button.rs:462) or `iced_widget::text_input::DEFAULT_PADDING` (text_input.rs:125).
- Their docs say so, and the iced showcase follows.

## 3. Showcase chrome (gpui example)

### 3.1 The application's own values (rationale §5 point 6)

A named showcase constant is used only where the theme states no value, for an element the showcase draws itself. Its comment says so, and so does the element's info. This applies to:

- the toolbar row's padding;
- the Sidebar header's gaps;
- the inner padding of content the showcase places in a popover or hover card (the platform's popover states 0, "content provides own padding", §2.16).

Where the model's layout values (`widget_gap`, `container_margin`) are stated, they are used instead. The showcase's pages keep their current `with_gap` behaviour; the same rule for them is recorded in `docs/todo.md`.

### 3.2 Status bar (D5)

Left to right:

1. The **left-panel toggle**.
2. The environment text.
3. The title of the shown info.
4. The **inspector toggle**.

The version leaves the status bar.

Each toggle:

- is built by a `demo::` helper and reports itself;
- is a ghost icon Button with `IconName::PanelLeft` or `IconName::PanelRight`, taken from the chosen set through `chrome_icon`. Where the set lacks the icon, the toggle shows its tooltip text as a label, as toolbar buttons already do;
- is `selected` while its panel is open (for the left toggle, while it is expanded rather than collapsed to the rail);
- dispatches `ToggleSidebar` or `ToggleInspector`;
- has a tooltip naming its action and key (Ctrl+B, Ctrl+I).

`ToggleSidebar` still collapses the Sidebar to its icon rail.

### 3.3 Toolbar and Sidebar header (D6)

**Toolbar:**

- It holds Command Palette, Reload Theme and Preferences.
- The preset Combobox, the colour-mode ToggleGroup, the icon-set Select, the SidebarToggleButton and the inspector button leave it.

**Sidebar header:**

- `Sidebar::header` (sidebar/mod.rs:276) holds a `v_flex` of three rows, each a `demo::label` above its control:
  - **Theme:** the preset Combobox;
  - **Mode:** the colour-mode ToggleGroup;
  - **Icon set:** the icon-set Select.
- The slot's own `pt_3 px_3` pads it, so no margin is added.
- The gaps are `widget_gap` where stated, or the showcase's own constant (§3.1).
- The controls take the panel's width.
- At `NAV_WIDTH` every control fits without overflowing, and a test asserts it. If a control does not fit, `NAV_WIDTH` grows to what it needs and stays a named constant whose comment says why.
- The header is not rendered in the rail.
- The header content is plain elements, so `SidebarHeader`'s exception in `docs/showcase-exceptions.toml` stays true.

**Exceptions file:** `SidebarToggleButton` joins `docs/showcase-exceptions.toml`, and the file's list of reason kinds (:8-11) gains "it would mix icon sets".

### 3.4 Title (D7)

One constant, `concat!(CARGO_PKG_NAME, " ", CARGO_PKG_VERSION, " showcase")`, serves three places:

- the title bar's label (chrome.rs:59);
- `window.set_window_title` (main.rs:929);
- the Windows screenshot capture's `FindWindowW` lookup (main.rs:707-709).

### 3.5 Sidebar icon size

- The page icons use `geometry::icon_size_small`, not `icon_size_panel` (demo.rs:472), per rationale §9. Their info names that builder.
- A windowed test runs under every native preset and one colour-scheme preset, expanded and in the rail. It asserts:
  - each icon lies inside its own item's bounds;
  - no two items' icons intersect.
- The test must fail before the fix.

### 3.6 Info and tests

Every moved or new chrome element carries true info under the existing rules: claims cite the lines they read, painted values are shown, notes hold under reduced motion, and ids are unique.

These tests follow the controls to their new places:

- the toolbar tests (their final children are fixed once, after both moves);
- the chrome-bar hover test;
- the status-bar tests;
- the preset, mode and icon-set tests.

`the_toolbar_is_the_models_toolbar` checks `min_h` only where `bar_height` is stated. `chrome_icon_names()` (tests.rs:2536-2547) gains `PanelLeft` and `PanelRight` and loses `Inspector` if no chrome element uses it any more.

**End-to-end tests of the reported defects**, under `kde-breeze`:

- the status bar's first and last children are inset from its edges by the padding drawn;
- the About dialog's content is inset from its frame by 10px.

## 4. Docs

- **CHANGELOG `[Unreleased]`** records:
  - the resolver no longer inventing `0.0`;
  - per-side padding and its TOML shorthand rule;
  - the split border types;
  - `toolbar.bar_height` as optional;
  - the preset and reader values corrected;
  - the removed `control_height`, and the changed `dialog_content_padding` and `input_height`;
  - the showcase's chrome changes.

  The entries the showcase-app plan added about the toolbar's contents, the status bar's version or the title are updated to the new arrangement.
- **The connector README** is updated where it describes padding builders, heights or the showcase's chrome.
- **`docs/todo.md`:**
  - The open item "KDE's toolbar height has no source, and no preset states a toolbar padding" is closed, with what this change did about each of its three findings.
  - The screenshot-review item is updated to cover the new chrome, the colour-scheme presets' new paddings, the GNOME dialog's section gaps, and the laid-out control heights.
  - These items are appended:
    - the follow-up plan for the other unstated sizing fields (rationale §7), with the audit's list;
    - the colour-scheme presets' sizing provenance;
    - applying rule §3.1 to the showcase's pages;
    - a machine-readable platform-facts, so that every native value can be gated.
- **Other plan documents:** `docs/todo_iced-full-theme-geometry.md`, `docs/todo_gpui-full-theme.md`, `docs/todo_egui-widgets-spec.md` and `docs/todo_v0.6.0_egui-connector-{rationale,spec}.md` name the old padding fields or a plain `f32` `bar_height`. Each gets an appended note describing the new model; existing text is not rewritten.
- **Archive:** these three documents are archived when the work is done.
