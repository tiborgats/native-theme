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
- **Mechanism.** The derived `#[serde(default)]` struct (border.rs:58-78) cannot express a shorthand or that error. `WidgetBorderSpec` deserialises through a raw struct that has all six keys, with `#[serde(try_from = "...")]` doing the expansion and the check, exactly as `FontSpec` deserialises through `FontSpecRaw` (font.rs:151-156) and registers its wire keys with `#[theme_layer(fields = …)]` (native-theme-derive lib.rs:146-151).
- **Linter.** `lint_toml` (model/mod.rs:679) builds its known keys from the struct's fields, so the two shorthand keys are registered with it through that same attribute, or every existing theme would be reported as using unknown fields. The existing `lint_toml_all_presets_clean` (model/mod.rs:1568-1580) lints `to_toml()` output, which never contains a shorthand key, so a new test lints the raw source of every bundled preset.
- Serialisation writes sides.
- `docs/property-registry.toml`'s `Border` structure lists the four side keys and the two shorthands.
- Platform-facts' padding conventions paragraph says how an asymmetric cell maps to the sides.

**Resolved (D8):**

- `ResolvedBorderSpec` is split in two:
  - `ResolvedDefaultsBorder` has `line_width`, `corner_radius`, `corner_radius_lg`, `color`, `opacity` and `shadow_enabled`.
  - `ResolvedWidgetBorder` has `color`, `corner_radius`, `line_width`, `shadow_enabled` and `padding: ResolvedPadding`.
- `ResolvedPadding` has four fields, `top`, `right`, `bottom` and `left`, each `Option<f32>`.
- The widget border no longer carries the `corner_radius_lg` and `opacity` that it resolves to `0.0` today (validate_helpers.rs:279-281, :333-335).
- The derive crate follows: `gen_validate.rs:42-47` matches the type name `ResolvedBorderSpec` literally, and `model/widgets/mod.rs` carries 19 `resolved_type = "ResolvedBorderSpec"` annotations. So does the re-export at lib.rs:212.

### 1.2 `toolbar.bar_height`

- `toolbar.bar_height` resolves to `Option<f32>`, through the derive's `#[theme(category = "soft_option")]` (native-theme-derive lib.rs:36, gen_structs.rs:42).
- KDE's value is "(none), sizes to content" (§2.13).
- No other sizing field changes type in this plan (rationale §7). The audit's list of the others goes to `docs/todo.md`.

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
  - `validate.rs:77-84` (`border_required`) and the macro doc at `validate_helpers.rs:521`;
  - `resolve/tests.rs`, which names the padding fields in 38 places;
  - the gpui showcase: `tests.rs:407` (`ResolvedBorderSpec`), `tests.rs:686-695` (`bar_height` read as `f32`), `demo.rs:1297` with `tests.rs:1780-1792` (the HeightOnly field), and `tests.rs:2884-2892` (`control_height`).
- Stale rules go:
  - the `defaults.border.padding_*` entries at `docs/inheritance-rules.toml:58-59` (the heuristic that model/border.rs:14-18 records as removed);
  - their mirrors in `implemented_targets` (`resolve/inheritance.rs:350-351`).

  The comment at `inheritance-rules.toml:66` names the side keys.

### 1.4 Native themes state what their platform documents (D2)

The native presets are `kde-breeze`, `adwaita`, `macos-sonoma` and `windows-11`, each with its `-live` twin, in both variants. For padding, `toolbar.bar_height` and `toolbar.item_gap`, each states every value platform-facts documents for its platform. Each platform-facts cell is read as follows (rationale §3, §5 point 5):

| Cell | Treatment |
|---|---|
| A number, with or without a qualifier ("(measured)", "(convention)", a leading "~") | Stated. An equal pair uses the axis shorthand; unequal sides use side keys. The comment carries the qualifier. |
| A range | Not stated, unless a source pins it; then platform-facts is corrected. |
| A per-context value | Stated for the context the widget matches (the desktop pointer, for the input device), and the comment names the context. Where no context clearly matches, a ruling decides; if none matches, not stated. |
| A derivation ("← button padding (10px)") | Stated as the derived value, and the comment cites the derivation. |
| A pointer ("(none) — use §2.20 layout margins") | Not stated here; the field it points to carries the value. |
| A cell with no number, or any "(none)" | Not stated. The audit may propose a sourced `0` for platform-facts; the preset never states one on the strength of a remark. |
| A side measured to a structure the model has no field for (WinUI's combobox: "0 right (arrow area adjacent)", with a 38px arrow area, platform-facts.md:1539-1541) | Not stated, with the reason. The structure is a recorded model extension. |

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
| Windows | tab | 3 / 4 / 3 / 8, pending the ruling below | §2.11 |
| Windows | menu | 4 / 11 / 5 / 11 | §2.6, mouse context |
| Windows | combo_box | 5 / not stated / 7 / 12 | §2.24; the right side is measured to the arrow column |

Three per-context cells need a ruling before they are written: the Windows tab's "8,3,4,3 (8/8 without close button)" (platform-facts.md:428); Windows' toolbar `bar_height`, "default = 64, compact mode = 48" (:1340), where the preset and the reader both state 48; and GNOME's list padding, "rich-list=12, plain list=2" and "rich-list=8, plain list=2" (:1377-1378).

The audit adds every other row and mismatch in scope: documented but missing, stated where not documented, different, range, per-context, derivation. It also covers the OS readers' size constants:

- `kde::metrics::populate_widget_sizing`;
- `macos_widget_defaults`;
- the WinUI3 constants of `read_widget_sizing`.

The GNOME reader states no sizes.

**Mismatches known already** (rationale §6): `kde/metrics.rs:20`; the Windows reader's 12s and tooltip 8 / 8 (windows.rs:196-233, :250-290); the Windows reader's `toolbar.item_gap` 4 (windows.rs:240, :296); `macos.rs:263`'s button padding 12.

Every ruling corrects whichever side its source proves wrong: the preset, the reader, or platform-facts.

**Colour-scheme presets** drop their unsourced `bar_height_px = 40.0` (rationale §7). Nothing else about their sizes changes here.

### 1.5 Reader constants outside the gates

- The KDE constants (`kde::metrics::populate_widget_sizing`) move out of the `kde`-feature module (lib.rs:99-100) into a function that is always compiled. They depend only on `crate::ThemeMode`.
- The WinUI3 constants move into one always-compiled function, which both builds of `read_widget_sizing` call (windows.rs:186, :246).
- Only the OS reads (`GetSystemMetricsForDpi`, `index.theme`) stay behind their gates.
- `cargo test -p native-theme` with no features then sees every reader's constants.

### 1.6 Gate

A test in `native-theme` holds one row per (platform, widget). It follows the precedent of `tests/platform_facts_xref.rs`, which already checks some preset values against platform-facts. It lives inside the crate because the reader-constant functions are `pub(crate)`. The rows cover every padding, `bar_height` and `item_gap` the platform-facts table documents for that platform, read per §1.4.

- Each row gives the expected sides or value, as `Some(v)` or `None`, with the platform-facts line it comes from.
- For each row, in both variants, the test resolves two themes and asserts the resolved value:
  - **Static:** the platform's full preset.
  - **Live:** the full preset, with the `-live` preset merged over it and then the reader's constants from §1.5, in `pipeline.rs:48-52`'s order. GNOME has no reader constants.

**Discrimination proofs:**

- Remove `kde-breeze`'s dialog padding: the static row fails, naming path, preset, variant, field and citation.
- Set a KDE reader constant wrong: the live row fails.

### 1.7 Status-bar padding (D4)

Research each platform's native status bar from upstream sources:

- **KDE:** Qt `QStatusBar` with Breeze.
- **GNOME:** the GTK version and theme sources that platform-facts' GNOME column uses.
- **Windows:** the Win32 status bar and WinUI.
- **macOS:** AppKit, which has no status-bar widget.

Append per-side padding rows to platform-facts §2.14, with citations. A platform without a sourced number gets **(none)** and its reason, and stays unstated. Where a value is stated, the gate gains the row, and the native preset states the value.

## 2. Connectors

### 2.1 gpui (`native-theme-gpui`)

**Padding:**

- Every `geometry::*` builder that pads sets each side (`pt`, `pr`, `pb`, `pl`) only when that side is `Some`.
- `geometry::input`, `select` and `combobox` apply their stated sides. Upstream sets its own padding before the refinement (input.rs:701 → :719; select.rs:544 → :546; combobox.rs:995 → :997), so the refinement wins, and neither trigger pads an inner element, so nothing doubles. The "padding is inner, Tier U" note at geometry.rs:147 is corrected. Two exceptions are documented: an Input with a suffix takes its right padding from upstream after the refinement (input.rs:736), and the combobox's right side has no receiver in gpui (§1.4), whose caret sits inside the padded root (select.rs:57-66, combobox.rs:1009-1026).
- A seams test measures each of these widgets under every native preset and asserts that the drawn content inset equals the stated side.

**Heights:**

- `control_height` goes. One private helper applies this rule to `geometry::button`, `input`, `combo_box_metrics` (select and combobox), `menu_item` and `list_item`:
  - it applies the platform's `defaults.line_height` as the control's line height, so growth follows the platform's font metrics (button.rs:689, input.rs:699 and the list and menu rows all take a refinement after their own line height, or set none);
  - at a text scale of 1 or less, the stated height through the property each builder uses today: `h` for the button, input, menu item and list item; `min_h` for the select and combobox;
  - above 1, `min_h(stated height)` together with `h_auto`, so layout grows the control around its drawn text and padding.
- The rule is for single-line controls. A multi-line Input sets its own height before the refinement (input.rs:705-708), which the refinement overrides today; the showcase's Textarea applies its own `Styled::h` after the builder, and the builder's doc says so.
- `geometry::input_height` returns a `StyleRefinement` carrying the height rule alone. The showcase's HeightOnly sample applies it, so its height follows the refined Input's at every scale.
- A seams test proves, under every native preset, for a real Button, Input, Select, Combobox, the app-drawn menu row (`demo::menu_rows`; upstream's `MenuItemElement` is `pub(crate)`) and a ListItem:
  - at text scale 1, resolved at the platform's own DPI (72 for macOS, per detect.rs:433), the height equals the stated value exactly, and the text's bounds lie inside the control;
  - at text scale 2, the height grows and the text's bounds still lie inside.

  The text's bounds are read through a probe element around the sample's text, since an Input's text element has no debug selector.
- If upstream prevents `h_auto` for a widget, the implementer reports it rather than working around it.

**Toolbar:** `geometry::toolbar` sets `min_h` only when `bar_height` is `Some`. The row is the application's own, so an unstated side or height leaves it to the application.

**Tooltip width:** `tooltip_content` computes `max_width` − left − right − 2 × `TOOLTIP_BORDER`. An unstated side is upstream's `px_2` (tooltip.rs:123), which is 0.5 rem, converted at the font size the connector installs as the rem (root.rs:582).

**Dialog:**

- `dialog_content_padding` (lib.rs:471) is removed. Nothing calls it but its own test (lib.rs:1420), and the resolved theme exposes the value.
- `geometry::dialog`'s doc notes how upstream reuses the paddings as gaps: the gap between the dialog's sections is `max(top, 8px)` (dialog.rs:620), and `DialogContent`'s gap is the bottom padding (:656). GNOME's 32 top and 24 bottom therefore also space the sections and the content.

**Follow-through.** These all follow the changes above:

- the builders' docs;
- the `geometry.rs` tests, with stated and unstated sides;
- the seams test;
- the README builder table;
- the showcase's `GEOMETRY_NOTES` and infos that describe padding, heights or `bar_height`;
- the Theme Map rows that showed `control_height` (demo.rs:4784-4850) and their test (tests.rs:2880-2893), which go with it.

### 2.2 iced (`native-theme-iced`)

- `button_padding` and `input_padding` (lib.rs:275, :286) keep returning `Padding`.
- Each side is either the stated side, or, where unstated, iced's own public default: `iced_widget::button::DEFAULT_PADDING` (button.rs:461) or `iced_widget::text_input::DEFAULT_PADDING` (text_input.rs:125).
- Their docs say so, and the iced showcase follows.

## 3. Showcase chrome (gpui example)

### 3.1 The application's own values (rationale §5 point 6)

A named showcase constant is used only where the theme states no value, for an element the showcase draws itself. Its comment says so, and so does the element's info. This applies to:

- the toolbar row's padding;
- the Sidebar header's gaps.

Where the model's layout values (`widget_gap`, `container_margin`) are stated, they are used instead. The showcase's pages keep their current `with_gap` behaviour; the same rule for them is recorded in `docs/todo.md`. The content the showcase places in a popover or hover card adds no padding of its own: those widgets now draw gpui-component's `p_3` on every preset (rationale §8). The HoverCard's doc at demo.rs:4107-4111 says the content adds none, but also names "the platform's popover padding where `geometry::popover` applies"; it is updated.

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
  - The freedesktop map's KDE arm for `PanelRight` becomes `sidebar-expand-right` (icons.rs:686-691), the pair of `PanelLeft`'s `sidebar-expand-left`. Breeze ships both files. The GTK arms stay. This is a connector change, and the CHANGELOG records it.
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

`the_toolbar_is_the_models_toolbar` changes twice: Task 3 makes it check `min_h` only where `bar_height` is stated, and Task 6 re-points its children. `chrome_icon_names()` (tests.rs:2536-2547) gains `PanelLeft` and `PanelRight` and loses `Inspector` if no chrome element uses it any more.

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
  - the input, select and combobox builders applying their padding;
  - control heights: the platform's height at text scale 1, laid out when text is scaled;
  - the removed `control_height` and `dialog_content_padding`, and the changed `input_height`;
  - the popover and hover card drawing upstream's padding;
  - KDE's `PanelRight` icon mapping;
  - the showcase's chrome changes.

  The entries the showcase-app plan added about the toolbar's contents, the status bar's version or the title are updated to the new arrangement.
- **The connector README:** Task 3 updates its builder table (padding, heights, line height, the removed functions), and Task 7 its description of the showcase's chrome. `connectors/native-theme-gpui/proposals/README.md` names the old padding fields and is updated too.
- **`docs/todo.md`:**
  - The open item "KDE's toolbar height has no source, and no preset states a toolbar padding" is closed, with what this change did about each of its three findings.
  - The line-height item (todo.md:860-870) is updated: its "`control_height` already uses it" becomes false, and the builders now apply the line height.
  - The screenshot-review item is updated to cover: the new chrome; the popover, hover card and status bar on every preset; the colour-scheme presets' dialog and toolbar; the GNOME dialog's section and content gaps; the input, select and combobox padding now applied; the control heights on Windows and macOS; the Textarea.
  - These items are appended:
    - the follow-up plan for the other unstated sizing fields (rationale §7), with the audit's Table B;
    - the colour-scheme presets' sizing provenance;
    - applying rule §3.1 to the showcase's pages;
    - a machine-readable platform-facts, so that every native value can be gated.
- **Other plan documents:** `docs/todo_iced-full-theme-geometry.md`, `docs/todo_gpui-full-theme.md` (whose line 50 also names `dialog_content_padding` as delivered), `docs/todo_egui-widgets-spec.md` and `docs/todo_v0.6.0_egui-connector-{rationale,spec}.md` name the old padding fields, the removed functions or a plain `f32` `bar_height`. Each gets an appended note describing the new model; existing text is not rewritten.
- **Archive:** these three documents are archived when the work is done.
