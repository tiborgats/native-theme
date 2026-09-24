# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Breaking Changes

#### native-theme

- `ResolvedFontSpec` gains `defined_size: Option<FontSize>` — the size as its source stated it, in the unit it stated. `size` is unchanged and is still always logical pixels. Until now resolution converted and then *discarded* the unit: `FontSize::Pt(10.5)` at 96 DPI and `FontSize::Px(14.0)` both became `14.0`, and nothing downstream could tell them apart. Anything that shows a size to a person needs that difference — a UI author sizing custom elements to match the platform's needs the number the platform actually gave, and back-converting `size` by the DPI would label a preset written in `size_px` as points, a unit no source ever stated. `None` only where the source stated no size and validation recorded the omission. `FontSize` also gains `Serialize`/`Deserialize` so the field can travel with the struct, and `defined_size` is `#[serde(default)]` so older serialized themes still deserialize. Code that constructs a `ResolvedFontSpec` literally adds the field; code that reads one is unaffected.
- **A size the platform does not state stays unstated.** The resolver filled a widget's missing padding with `0.0` and reported nothing, and the gpui connector applied that zero over the toolkit's own padding, so on KDE the showcase's status bar text touched the window's edges and the About dialog's content touched its frame. A padding side the theme does not state now resolves to `None` all the way to the toolkit, and a stated side below 0 is a validation error. The same holds for `toolbar.bar_height`, which resolves to `Option<f32>`: KDE's toolbar sizes to its content and GNOME's `.toolbar` sets no minimum height (the 47px adwaita stated is the headerbar's), so kde-breeze, adwaita, the colour-scheme presets (40px each), material (64px) and ios (44px), none of them sourced, no longer state one; macos-sonoma (38) and windows-11 (48) still do. The colour-scheme presets, material and ios state none of the sizes the gate covers: their paddings on every widget, their menu and list row heights (28px each, material 48px, ios 44px), their toolbar item gap (6px, material and ios 8px) and their combobox arrow width (28px) had no source, and a colour scheme has no platform to cite, nor do material and ios a platform-facts column, so the toolkit's own sizes stand. `docs/inheritance-rules.toml` drops the `defaults.border.padding_*` rules the resolver no longer had.
- `menu.row_height`, `list.row_height`, `combo_box.arrow_area_width` and `toolbar.item_gap` resolve to `Option<f32>`, as `toolbar.bar_height` does. KDE's menu items size to their font and its list rows to their content, GNOME's plain list sets no row height, GNOME's dropdown draws its arrow inline, and macOS's arrow column is a measured range, ~16–18px (`docs/platform-facts.md` §2.6, §2.15, §2.24), so no value stands for them there; a colour scheme has no source for a toolbar gap. A stated value below 0 is still a validation error.
- **Padding is per side.** `WidgetBorderSpec`'s `padding_horizontal` and `padding_vertical` are replaced by `padding_top`, `padding_right`, `padding_bottom` and `padding_left`, each `Option<f32>` and merged side by side, the overlay winning as for every other field. Platform-facts states ten padding rows with two different sides (Windows' input is 10 left / 6 right, 5 top / 6 bottom), and with one value per axis the presets had picked one number for each, the Windows tooltip an average no source gives. In TOML the sides are `padding_top_px` … `padding_left_px`; `padding_horizontal_px` and `padding_vertical_px` stay as shorthand that sets both sides of their axis, so existing themes keep parsing, and a table that states an axis key together with one of that axis's sides is a parse error naming both (``border: set `padding_horizontal_px` or `padding_left_px`, not both``). Serialisation writes sides, and `lint_toml` accepts both forms.
- **The resolved border is split in two.** `ResolvedBorderSpec` is replaced by `ResolvedDefaultsBorder` for `defaults.border` (`line_width`, `corner_radius`, `corner_radius_lg`, `color`, `opacity`, `shadow_enabled`) and `ResolvedWidgetBorder` for every widget (`color`, `corner_radius`, `line_width`, `shadow_enabled`, and `padding: ResolvedPadding` with `top`, `right`, `bottom` and `left`, each `Option<f32>`). A widget border no longer carries the `corner_radius_lg` and `opacity` that it resolved to an invented `0.0`, and the defaults border no longer carries a padding that could only ever be empty. A serialised widget border without `padding` deserialises with every side unstated.
- The public `native_theme::kde::metrics` module is gone. It held no public item; the KDE reader's size constants now compile on every target, so the tests see them without the `kde` feature.

#### native-theme-gpui

- Reduced motion is a *request*, not a write: `reduce_motion: false` no longer clears a flag the connector did not set, so gpui-base's own OS reader (new in 0.6.2) and the application keep control; `true` followed by `false` undoes only the connector's own switch and then asks gpui-base to re-read the system. Callers that relied on `apply(.., &AccessibilityPreferences::default(), ..)` to switch motion back on call `cx.set_reduce_motion(false)` themselves.
- `geometry::tooltip` no longer sets a maximum width, and the icon functions in `icons` return `ImageSource::Render` where they returned `ImageSource::Image`. The width moves to the new `geometry::tooltip_content`, on the element the application passes to `Tooltip::element`; on the bubble it clamped the bubble and not its text. A `Render` source carries a decoded `gpui::RenderImage`, which holds a sprite-atlas tile until it is handed to `App::drop_image` — an application that rebuilds its icon caches should drop the sources it replaces that way. Both changes are the subject of a `Fixed` entry below.
- `geometry::control_height` is removed: each control builder applies the height rule itself (see `Changed`). `geometry::input_height` returns a `StyleRefinement` carrying that rule alone, to apply with `refine_style`, where it returned `Pixels`. `native_theme_gpui::dialog_content_padding` is removed; nothing but its own test called it, and `dialog.border.padding` in the resolved theme carries the value.
- A `geometry` builder sets only the padding sides the theme states, each with its own `pt`, `pr`, `pb` or `pl`; an unstated side leaves the widget's own padding in place — a `StatusBar`'s `px_2 py_1`, a `Dialog`'s 16px, a `Popover`'s `p_3`. `geometry::toolbar` sets its minimum height only where `toolbar.bar_height` is stated, its gap only where `toolbar.item_gap` is, `menu_item` and `list_item` their height only where `menu.row_height` and `list.row_height` are, and `geometry::tooltip_content` subtracts the left and right sides separately, taking upstream's `px_2` (0.5 rem at the font size the connector installs as the rem) for a side the theme does not state.

#### native-theme-iced

- `font_size` and `mono_font_size` take `&AccessibilityPreferences`, and `from_system` returns the preferences as a fourth element, so the user's text-scaling setting reaches an iced application for the first time. With a preset, pass `&AccessibilityPreferences::default()`. Reduced transparency and reduced motion have no receiver in iced — a `Theme` is a palette — so `to_theme`, `from_preset` and `SystemThemeExt::to_iced_theme` are unchanged.
- iced's `secondary.base` colour now carries the platform's **placeholder** colour instead of its button surface, and `secondary.strong` is a copy of it. iced reads that slot as placeholder text in three widgets, and the placeholder was between 1.06:1 and 1.84:1 — invisible — on every preset in both modes. Two other readers take the same slot as a *fill* and change with it: `container::secondary` (`container.rs:626`) and `progress_bar::secondary` (`progress_bar.rs:294`) are now painted the placeholder colour. And because `secondary.strong` is a copy of `secondary.base`, iced's own `button::secondary`, which hovers by swapping `base` for `strong` (`button.rs:619-622`), no longer changes fill on hover. Applications that relied on `button::secondary` carrying the platform surface call `styles::button`, which carries idle, hover, pressed and label together.
- The window's default text is the platform's own: `background.base.text` is written from `defaults.text_color`. iced computes that slot with `readable()`, which replaces a text colour whose contrast is below 6.0:1, so on solarized (both modes) and tokyo-night light an iced application showed a colour the theme never stated. Nothing changes on the other 29 preset/mode combinations.
- `button_padding` and `input_padding` need the `widgets` feature (on by default), which brings in `iced_widget`. Each side of the `Padding` they return is the theme's where it states one and iced's own default where it does not — `iced_widget::button::DEFAULT_PADDING` or `iced_widget::text_input::DEFAULT_PADDING` — where they returned the two axis values, `0` wherever the theme stated none.

### Added

- `native_theme_iced::styles` (feature `widgets`, on by default): per-widget style functions built from the resolved theme, for the states iced's six-colour palette has no slot for. Twenty items — `button`, `button_primary`, `button_danger`, `button_success`, `button_warning`, `button_link`, `text_input`, `text_editor`, `checkbox`, `radio`, `toggler`, `pick_list`, `menu`, `slider`, `scrollable`, `scrollbar`, `progress_bar`, `rule`, `tooltip`, `container_card`. Every closure is `'static + Clone`; each doc comment names the setter it is passed to and the iced default it replaces. Hover and pressed colours a platform states as layers (Windows 11's 4 % overlays) are composited over the widget's own fill, a `soft_option` the platform leaves unset falls back to a copy of the base state, and a `Style` field the model does not carry is read from iced's own default at run time — never a literal. `styles::scrollbar` also embeds the scrollbar where `scrollbar.overlay_mode` is false, as the gpui connector already did. A state the platform does not state has no distinct appearance: a hovered *checked* checkbox and a hovered *selected* radio keep exactly the checked fill (`checkbox.hover_background` is the hover of the unchecked box), and a *disabled* check mark takes `checkbox.disabled_text_color`, the platform's disabled foreground on its disabled fill.
- `native_theme_iced::styles::aw` (feature `iced_aw`, off by default): the same for the [`iced_aw`](https://crates.io/crates/iced_aw) 0.14 widgets that give six native widget themes a receiver iced itself lacks — `card`, `menu` (`MenuBar`), `tab_bar` (also `Tabs`), `sidebar`, `selection_list`, and `spinner`, which styles a wrapping container because `iced_aw`'s `Spinner` has no style of its own. `card` sets `close_color`, but `iced_aw` 0.14.1 never paints with it: `Card::on_close` styles its button with the *default* card class (`widget/card.rs:193-206`), whose icon is white on every theme, so the iced showcase dismisses its card with a themed button of its own and `docs/todo.md` records the upstream issue.
- native-theme-iced: `padding_or(&stated, default)`, each side a resolved padding states and `default`'s elsewhere — what `button_padding` and `input_padding` do for their widgets, for any other widget the theme states a padding for — and `stated_padding(&stated)`, the padding only when every side is stated, for a widget whose default is private (`iced_aw`'s `Card` and `TabBar`). Neither needs a feature.
- native-theme-iced features: `widgets`, `iced_aw`, and `material-icons`, `lucide-icons`, `system-icons`, `svg-rasterize`. The icon features are in `default`, so an application depending only on `native-theme-iced` gets working icon loading; before, `load_icon` returned `None` for every icon unless the application also enabled the features on `native-theme`.
- Mapping-contract tests in both connectors: every toolkit slot the connector writes is checked against the native field of the widget that reads it, over all 16 presets in both modes, with a coverage tripwire — every `ThemeColor` field (gpui) and every palette slot and `Style` field (iced) is either equal to a named native field or listed as derived with its derivation, and a test proves that every declared row was actually checked. The iced contract also lists, with the upstream line that shows it, the sixteen values iced 0.14 and `iced_aw` 0.14.1 cannot carry: a scrollbar's minimum thumb length, a button's minimum width and height, a combo box's minimum width and height (`Button`, `PickList` and `ComboBox` take a fixed width, and only `Button` a height), a progress bar's minimum width, a spinner's stroke width and minimum diameter, a tab's minimum width and height (`TabBar` and `Tabs` take a fixed extent, which cannot carry a floor), a list's alternate-row, header and grid colours, header font and corner radius, and the card's `close_color`, which is emitted and never painted. A test holds each entry to a value some preset states and iced does not draw, naming the combinations where the two agree; `sidebar.border.corner_radius` has no receiver either, but every preset states the 0 `iced_aw` draws, so it is not listed.
- A contrast invariant in both connectors: a foreground the connector places on a fill is never less readable than the platform's own pair, measured after alpha compositing over all 32 preset/mode combinations. It is not a WCAG assertion — the platforms' own data sits below AA in many places, macOS's `#34c759` with white text among them, and those are printed on every run for `preset-validator`. Where gpui-component has no token for a platform pair (a status bar's, title bar's, selected row's or selected text's label) the pair is reported with the reason, and a reported pair that stops being worse fails the test until it is promoted.
- Showcase self-tests in both connectors, run by a plain `cargo test`: both showcases are built, laid out and clicked headlessly (gpui's `TestAppContext`; `iced_test`'s `Simulator`). Every tab or page must lay out, every resize handle must move the boundary it is dragged across, every advertised control must change the state it claims to, and in iced every widget that has a style function must carry it.
- Both showcases render every widget their toolkit offers an application — gpui 108 shown and 27 excepted of 135, iced 28 modules plus the eight `iced_aw` widgets — and `scripts/check-widget-coverage.py` fails when an upstream release adds a widget that is neither shown nor listed, with a reason, in `docs/showcase-exceptions.toml`; `pre-release-check.sh`, CI, the publish workflow's gate and the nightly dependency canary all run it. A widget counts as shown only where the showcase *constructs* it: string literals and comments are stripped first, so a Widget Info note that argues about a widget is not a demo of it, and a gpui widget must then appear as an associated item, a call or a struct literal on a path the showcase's own `use` statements take from gpui-component — so `std::process::Command::new` is not gpui-component's `Command` — or through the extension method that builds it where no call site names it (`ContextMenuExt::context_menu`, `ScrollableElement::overflow_*_scrollbar`, and the `WindowExt` methods that hand a `Dialog`, `AlertDialog` or `Sheet` to a builder closure). Those rules made `Command` the showcase's command palette, where it had never been rendered, and made three exception entries, each with the lines that prove it: `Text` is `plot::label::Text`, a chart axis label built only by `AxisText`; `Loading` sits in a private module and is built only by `ListDelegate` and `TableDelegate`, so no consumer can render one; and `WindowBorder` is drawn by `Root` itself, where a nested one would re-set the client inset. The gpui showcase previously exercised 12 of the connector's 33 geometry builders and rendered no `StatusBar`, `TitleBar` or `Combobox`; a test now requires every builder to be used. The iced showcase takes its spacing from the platform's `LayoutTheme` and its widget sizes from the builders iced offers (`Checkbox::size`, `Toggler::size`, `ProgressBar::girth`, the rule's thickness).
- The gpui showcase's Widget Info is checked rather than argued. The tests in `src/showcase.rs` read every file of `examples/showcase-gpui/` and fail on it: **every colour claim carries the upstream line it was read at, and the test opens that line and requires the field the claim names to be read there**; every `file.rs, Symbol` in a note still resolves to something that exists; every geometry line an info shows is generated from the builder the widget was built with, and `GEOMETRY_NOTES` has one entry for every `pub fn` of `geometry.rs`; no gpui-component widget is built outside `demo.rs` and `chrome.rs`, and every helper there that builds one attaches its info; and the Theme Map names every `ThemeColor` field exactly once, read from upstream's own struct. The windowed self-tests fail when a frame they draw holds two info targets under one id, and three of them compare a painted fill with its claim through gpui's test-only `Window::painted_quads`. One more test prints rather than fails (an info legitimately says nothing about a state its widget does not show): the omission report measures the widgets' infos against the theme fields the files they cite actually read, and names the ones **no widget's info** mentions. That residual is **10**, and each one is a state the showcase does not demonstrate — the outlined Danger button's hover fill, the outlined Info, Success and Warning buttons' hover and pressed fills, the segmented tab bar, and a table footer. Every test that reads upstream locates the vendored sources with `cargo metadata` from inside the test, so there is nothing to wire into a script and nothing that can be skipped; the earlier objection that a test cannot reach a dependency's source was simply wrong.
- Showcase demos for `Select` and `Textarea`, and `Command` as the showcase's command palette. The `Command` palette had never been rendered; `Select` (the icon theme) and `Combobox` (the preset) now sit together in the side panel's theme settings, which is where the one difference between them is visible — the connector gives `Select` the platform's font colour and `Combobox` only its size and weight, because upstream's disabled colour arrives through the same call and a carried colour would beat it on the one and yield to it on the other.
- `native_theme_gpui::variants::ghost_button(cx)`: a `ButtonCustomVariant` that is flat like gpui-component's `.ghost()` but hovers and presses with the platform's button colours. gpui-component hovers a ghost button with `accent` (the menu and list highlight) and an `InputGroupButton` with `muted` (the `Kbd` / code-block surface); both are hardcoded, so under a native theme those buttons hovered selection blue and near-invisible grey while every other button took the platform's hover. Apply it with `.custom(..)` in place of `.ghost()`, or on an `InputGroupButton`.
- `geometry::tooltip_content`: the platform's `tooltip.max_width` less the bubble's left and right padding and upstream's 1 px border on each side, for the element an application passes to `Tooltip::element`. It is the one place that width makes a tooltip's text wrap, and it closes the overflowing tooltip below.
- `geometry::scrollbar_gutter`: the platform's scrollbar groove width as right padding on the element a scroll container scrolls, where `scrollbar.overlay_mode` is false, and nothing where it is true. It closes the scrollbar-over-content defect below; the width is read back from `base_layer::scrollbar_geometry`, the one written onto gpui-base.
- `geometry::list`: the frame of a list view — `list.border`'s line width, colour and corner radius, and a clip to that radius. Neither gpui-component's `List` nor its `Tree` paints a border of its own, while its `DataTable` draws one from the theme, so a framed list and a table beside it disagreed; apply it to the `List` or the `Tree` itself, or to the box an application draws around one. A tree is a list view: the model has no tree theme.
- `geometry::input_group_button`: the platform's button corner radius for a button nested in an `InputGroup`, which gpui-component rounds with `radius / 2` because it sizes such a button `XSmall`. `geometry::dialog` now also carries the dialog's own corner radius, which upstream would otherwise take from `radius_lg`.
- `geometry::toolbar`: a toolbar row the application draws itself — gpui-component has no toolbar widget — from the model's `toolbar`: `bar_height` as a minimum height where the platform states one, `item_gap` between the items where the theme states one, the `border` padding sides it states, `background_color`, and `font` size and weight. A toolbar that sizes to its content, as KDE's and GNOME's do, states no height, and the row is then left to the application; there is no edge, because no platform states one. The model's toolbar had been read by nothing in either connector.
- Rendered seam tests (`connectors/native-theme-gpui/tests/seams.rs`): real gpui-component widgets are laid out headlessly, with and without the connector's geometry, and the measured height is compared with the refinement's own field. `button`, `input`, `select`, `combobox`, `list_item` and `progress` are covered; CI, the publish gate and the nightly canary all run them.
- A gate in native-theme (`presets/documented_sizes.rs`) that the native presets state every size their platform documents: every padding side, the toolbar's `bar_height` and `item_gap`, the menu's and the list's `row_height`, and the combobox's `arrow_area_width` where the platform gives one number, that `docs/platform-facts.md` gives for KDE, GNOME, macOS and Windows — 72 rows, one per platform and widget, each citing its platform-facts lines, and `None` where the platform documents nothing. Each row is checked in both variants against the full preset and against the live path (the full preset, its `-live` preset over it, and the reader's size constants over that), so a reader constant is held to the facts as well; the Windows and KDE constants moved out of their OS and feature gates so every test run sees them. Companion tests keep one row per platform and widget, keep each citation on the row it names, require a full preset and its `-live` twin to state the same sizes, and require the colour-scheme presets, material and ios to state none of them.
- Three more seam tests: under every native preset at its own platform's DPI, a real `Input`, `Select` and `Combobox` draw their content inset by the stated left padding, and a real `Button`, `Input`, `Select`, `Combobox`, `ListItem` and an application-drawn menu row are their stated height at text scale 1 and hold their text at 1, 1.1 and 2; under kde-breeze and windows-11, which state an arrow column and a right side of 0, a real `Select` and `Combobox` keep upstream's own right inset, and with kde-breeze's column removed they draw its stated right side.
- Showcase sections for `InputGroup`, `Empty`, `Carousel`, a Rust code editor and a Markdown view — the components gpui-kit 0.6.2 added or changed. The `InputGroup`'s Copy button copies the field's text to the clipboard, confirms with a notification, and takes `variants::ghost_button`, so it hovers like the buttons around it.
- The iced showcase's new parts take the theme's padding: the pane controls and the card's two buttons `button_padding`, and the menu bar's, menus' and context menu's items `menu.border.padding`, each over iced's button default for a side the theme leaves unstated. The `iced_aw` `Card`, `TabBar` and `Tabs` take `card.border.padding` and `tab.border.padding` where the theme states every side, and keep `iced_aw`'s own otherwise, and the containers dressed as a card take `card.border.padding` over a container's zero (windows-11 states 12).
- A nightly dependency canary workflow (`.github/workflows/dependency-canary.yml`, 18:10 UTC, also runnable on demand): `cargo update` on a throwaway lockfile, then the CI gate's clippy, test and doc steps on every crate. gpui-pre now publishes a GPUI snapshot every week and the connector's caret requirements let every consumer's `cargo update` pick it up, so a breaking snapshot is reported here before an application hits it. The same check run locally on 2026-09-09 against gpui-kit 0.6.1 and gpui-pre 0.3.4: the released 0.5.8 connector builds and passes its tests unchanged, so 0.6.1 needed no connector release.
- `scripts/check-features.sh`: every workspace crate's library is checked with no default features, with each feature alone and with all features — `cargo hack --each-feature`'s coverage, with the features read from `cargo metadata` — and a combination that builds with a warning fails. `pre-release-check.sh`, CI, the publish workflow's gate and the nightly dependency canary run it. The cross-target builds deny warnings as well: `pre-release-check.sh` (where the target is installed), CI's Windows and macOS legs and the publish gate check `native-theme` with all features under `-D warnings`, and the ten warnings it had on Windows are gone.
- A **Compatibility** section in both connector READMEs — the floors the crate's own manifest requires, the dated upstream set it has been verified against, and the nightly canary that covers what comes after — with `scripts/compat-check.sh` and `docs/COMPATIBILITY.toml` behind it. `compat-check.sh run` resolves the newest upstream release of a connector's family on a throwaway lockfile, runs that connector's tests, clippy, documentation and the widget-coverage script on it, and stamps the versions the lockfile ended up with — and the README's Verified line — only when every gate passed; the committed lockfile keeps its floors, which is what CI tests with `--locked`. Neither half is a claim anyone can write by hand: a test per connector checks the Required table against its `Cargo.toml` and another checks the Verified line against the stamp, and `pre-release-check.sh` runs `compat-check.sh check`, which warns while the CHANGELOG entry says "Unreleased" and fails once it carries a date, so a release cannot state a compatibility that was verified from other sources. `scripts/pre-release.sh` refreshes the claim where it stamps the screenshots. The gpui README had said `gpui-component 0.6.x · gpui-base 0.6.x · GPUI as gpui-pre 0.3.x`, which was below the floor in one direction and a promise about untested releases in the other.

### Changed

- Both connectors: status labels (`success`, `danger`, `warning`, and gpui's `info`) are the platform's own colours. The previous contrast enforcement chose between white and black by a 0.5 lightness threshold, which picked the worse of the two in 31 of the 37 labels it touched and made 7 less readable than the platform's own. It did not spare the platform presets: of the 96 status labels iced carries (16 presets × 2 modes × 3 labels) it changed 21, among them kde-breeze's near-white `#fcfcfc` — light success and danger, dark success — which it turned pure white, and windows-11 light's warning label, which it turned from the platform's near-black to white.
- native-theme-gpui: seven `geometry` builders now carry the platform's text colour as well as its size and weight — `status_bar`, `tooltip`, `dialog_description`, `list_item`, `radio`, `select` and `title_bar`. gpui-component sets its own text colour on the first six and applies the caller's refinement afterwards, so the platform's colour never arrived: a status bar and a dialog description were labelled `muted_foreground` on all 32 preset/mode combinations. It sets none at all on the title bar, so `title_bar` displaces nothing and delivers `window.title_bar_font.color` — which the KDE reader states from `[WM] activeForeground`, and a contrasting colour scheme would otherwise have shown as the window's text colour on the title bar. `checkbox` and `combobox` deliberately carry none, because upstream would let it displace the disabled colour.
- native-theme-gpui: gpui-component's `accent` / `accent_foreground` now take the platform's **menu hover pair** (`menu.hover_background` / `menu.hover_text_color`) instead of the platform's accent colour, and `sidebar_accent*` the sidebar's own selection pair. Upstream defines `accent` as the item highlight ("hover background on MenuItem, ListItem, etc."), and only KDE and macOS highlight a hovered menu row with the selection colour; Adwaita and Windows 11 use a subtle fill with unchanged text. Nothing moves on KDE or macOS. On Adwaita, Windows 11 and Material, hovered menu, completion, command-palette and calendar items stop turning saturated accent; a pressed `Toggle`, which reads the same token, turns subtle with them (upstream needs a separate token, recorded in `docs/todo.md`).
- native-theme-gpui requires **gpui-component / gpui-base 0.6.6** and **gpui-pre 0.3.6**; older GPUI Kit versions are not supported. gpui-component 0.6.2 removed `ThemeColor::tiles` in a patch release, so the connector cannot serve both sides of it. An application that pins gpui-component or gpui-pre below these floors keeps resolving to native-theme-gpui 0.5.8, which still builds there. Newer GPUI Kit releases are covered by the same requirement, and the set the connector has actually been run against is stated in its README, dated, where `scripts/compat-check.sh run` writes it from the run that earned it. Between 0.6.4 and 0.6.6 gpui-component's source changes only in `label.rs` (masked labels) and `inspector.rs`, and gpui-base's not at all — the eleven Widget Info citations into `label.rs` moved four lines each, and the citation gate is what reported them.
- `ThemeColor` mapping 139 → 138 fields and the `ThemeConfig` colour export 127 → 126: `tiles` no longer exists upstream.
- native-theme-gpui: `ThemeColor::link` takes `link.font.color` instead of `defaults.link_color`, and `table_row_border` takes `list.grid_color` instead of `defaults.border.color`. gpui-component reads the first as a link's text (`link.rs:76`, `button/button.rs:993`) and the second as the line between a table's rows and columns (`table/table.rs:203`, `table/state.rs:1424`), and those are the model's own fields for them. Every bundled preset gives each the value of the field it replaces, in both modes, so nothing changes on screen; a theme that states either one separately now reaches the toolkit.
- native-theme-gpui: `geometry::icon_size_toolbar` reads `toolbar.icon_size`, which inherits `defaults.icon_sizes.toolbar`, so the two differ only where a theme states a toolbar icon size of its own. No bundled preset does.
- The gpui showcase is an application, and its Widget Info describes the widget under the pointer rather than a demo block. The window asks the window manager to draw its frame (`WindowDecorations::Server`) and draws by what it is granted. Where the window manager draws the frame — KWin on Wayland, which offers xdg-decoration, a window manager on X11, and Windows and macOS, which keep their own title bar under gpui's default window options — the title bar, window controls, corners and shadow are the desktop's own, and the File, View, Theme and Help menus sit in a menu-bar row at the top of the window, except on macOS, where they are in the system's menu bar. Where it draws none, as GNOME's Mutter draws none for a Wayland client, gpui-component's `TitleBar` is the window's title bar and holds the menus. The OS title and the `TitleBar`'s label both read `native-theme-gpui <version> showcase`. The menus' items act; four of their actions also have key bindings: Cmd+Q on macOS, Ctrl+Q elsewhere, quits, and likewise B toggles the side panel, K opens the command palette and , the Preferences — gpui's `secondary` modifier, as gpui-component's own bindings use, so the menus show each platform's own. Under them a toolbar drawn with `geometry::toolbar` holds buttons for the command palette, a theme reload and the Preferences. The body is two panels of one resizable group, divided by a handle that drags. The side panel holds the theme settings, each labelled above its control — Theme (the preset switch, a searchable `Combobox`), Mode (a System / Light / Dark `Select`) and Icon theme (a `Select`) — and, below a `Separator`, the inspector. The content panel shows the page, with a `TabBar` above it that has a tab per page. A `StatusBar` holds the side panel's toggle at its left end, then names the desktop, the preset and mode, the theme's font in the unit its source stated, the text scale and the accessibility flags that are set, and at its right end names what the inspector shows. A command palette (Ctrl+K), a Preferences sheet for the four accessibility preferences and an About dialog open as overlays; a preference set in the sheet stays across theme switches and reloads, and the others are the OS's, read again at every theme install, so a change made on the desktop while the showcase runs lands with the next one; and a theme that fails to load is reported by an `Alert` between the page tabs and the page, so the tabs do not move when it appears. Every widget on the pages and in the chrome reports its own info, headings and list rows included, with two exceptions: the resizable group and its content panel do not yet (recorded in `docs/todo.md`), and the inspector's content below its TabBar reports nothing by design, so the pointer can move into it without replacing what it shows. The innermost one under the pointer is shown in the inspector once the pointer has rested on it for 250 ms: Theme colors (a swatch and the upstream line it was read at for each), Theme config, Not themeable, and a new This instance section for facts about the demo that no theme could set, with a Copy button for the text. The inspector's Theme tab holds the window-level facts, among them whether the window manager or `Root` draws the window's frame. The example is a module tree, `examples/showcase-gpui/`, where it was one file of 8,732 lines.
- The native presets and the OS readers state the sizes their platform documents, and no others (the gate under `Added`). Sides are top / right / bottom / left. **kde-breeze**: dialog 10, toolbar 6 and status bar 3/14/2/2, none of which it stated, combobox 6 left, top and bottom and 0 right, where Breeze's edit field meets its 20px arrow column (6 left and right only before), and no menu or list row height (28 each before). **adwaita**: dialog 32/24/24/24, toolbar 6, status bar 6/10/6/10, popover 8, checkbox 3, combobox 5/10/5/10, and list rows 2 on every side, the plain list's, where it stated the rich list's 8/12; no list row height (34 before, which neither list context gives) and no combobox arrow width (28 before). **macos-sonoma**: dialog 20, toolbar 0/8/0/8, and combobox and segmented control 3 top and bottom, their horizontal ranges unstated where it stated 9; no combobox arrow width (17 before, which platform-facts gives only as the measured range ~16–18). **windows-11**: button 5/11/6/11, input 5/6/6/10, menu 4/11/5/11, tooltip 6/9/8/9, list 0/12/0/12, toolbar 0/0/0/4, popover 15/16/17/16, dialog 24, card 12, the expander's header 16 on the left, combobox 5/0/7/12, the 0 where WinUI's text column meets its 38px arrow column, and a menu row height of 23, the mouse context's, where it stated 36. No native preset states a segmented control padding any more: no platform documents one but macOS's vertical 3, and KDE's figures are the tab bar's. The readers: KDE's button padding is 6 top and bottom (was 5); Windows' button 11 left and right, input 10 left / 6 right, tab 8 and menu 11 (each was 12), tooltip 6/9/8/9 (was 8), toolbar item gap 0 (was 4) and menu row height WinUI's 23, where it read `SM_CYMENU`, the menu *bar*'s height (its non-Windows build stated 32); macOS's button 8 left and right (was 12).
- `docs/platform-facts.md`, corrected from upstream sources pinned by commit: the Windows CommandBar's default style is Compact, so its height is 48, not 64; GNOME's toolbar row is `.toolbar`, with 6px padding on every side, a 6px gap and no minimum height (47 is the headerbar's); Qt pads a toolbar's items on all four sides, so KDE's vertical toolbar padding is 6, not 0; Breeze expands a combobox by its frame width on both axes, so KDE's vertical combobox padding is 6; the KDE and Windows combobox cells say why the right side is 0 — Breeze insets the edit field by its frame width on the left, top and bottom only and ends it at the 20px arrow column, and WinUI's `ComboBoxPadding` 12,5,0,7 pads a text column that ends at the 38px arrow column; the Windows expander's 16 is its header's left padding; the Windows tab names both of its contexts, 8 right without a close button and 4 with one; the popover's padding is GNOME's 8 and Windows' 15/16/17/16, and GNOME's checkbox 3, where the cells said none; and KDE's segmented-control cells are marked as the tab bar's, a proxy. §2.14 gains the status bar's padding per platform — KDE 3/14/2/2, the right being the 1 + 13px size grip that Breeze paints as nothing, in a window that is not maximized, GNOME 6/10/6/10, Windows and macOS none, each with its source or reason — and the padding conventions say how a cell maps to sides.
- native-theme-gpui: `geometry::input`, `select` and `combobox` apply the platform's padding sides. gpui-component pads those roots before it applies the caller's refinement (`input/input.rs:701` → `:719`, `select.rs:544` → `:546`, `combobox.rs:995` → `:997`), so the refinement wins and no inner element pads again; the builders' note that this padding was out of reach was wrong, and no preset's input or combobox padding had reached gpui. Two sides still do not arrive: an `Input` with a suffix takes its right padding from upstream after the refinement, and where the theme states `combo_box.arrow_area_width`, `select` and `combobox` leave the right side to upstream's own. That side is measured to a separate arrow column (0 on Windows and KDE, whose text meets a 38px and a 20px column), and gpui's trigger has none: its caret sits inside the padded row, so a 0 there would put the caret against the border. A theme with no arrow column states its right side to the text, and it is applied. When refining an `InputGroup` or `NumberInput` frame, clear the padding sides: the inner Input already pads (`input/group.rs:265-286`, `input/number_input.rs:158-165`, each an `Input` padded at `input/input.rs:700-702`).
- native-theme-gpui: control heights follow one rule in `button`, `input`, `select`, `combobox`, `menu_item` and `list_item`. Each applies the platform's `defaults.line_height` as the control's line height, which none of them had set, and is its stated height at a text-scaling factor of 1 or less — for a `Select` or `Combobox` trigger in place of upstream's `h_8`, which is 26px on macOS against the stated 21 — and above 1 the stated height is a minimum and the height is automatic, so the control grows around its text rather than clipping it. The stated height holds the text under every native preset; the tightest is macOS's 21px trigger, whose 15.5px text line lies 2.5px below its top and 3px above its bottom at scale 1. `control_height`, which this replaces, overshot the stated height at scale 1: the Windows button was 33px for 32, and at 96 DPI the macOS button, input and menu row 27px for 22. The rule is for single-line controls, and the showcase's Textarea, which `geometry::input` had forced to a single line's height, keeps its own 90px.
- native-theme-gpui: the freedesktop icon for `IconName::PanelRight` on KDE is `sidebar-expand-right`, Breeze's pair of `PanelLeft`'s `sidebar-expand-left`, where it was `view-right-new`. The GTK name is unchanged.
- The gpui showcase's chrome in detail. The side panel's toggle, at the status bar's left end beside the panel, is a small ghost button with the chosen icon theme's `PanelLeft`, selected while the panel is shown, with a tooltip naming its key; it hides the whole panel, which comes back at the width it was dragged to, and clears the info of a widget in it from the status bar, as a page change does. gpui-component's `SidebarToggleButton` draws its own icon whatever icon theme is chosen, so it is not used and is an exception entry. The side panel opens at `LEFT_PANEL_WIDTH`, 300px, the showcase's own width — the model states none — and the window at 1180 × 850, that width plus the 880px the pages were laid out for; a test checks that the theme settings and the inspector fit the panel under all 16 bundled presets, each native one at its platform's DPI, at text scales 1 and 2. The colour mode's System row reads "System", the desktop's mode being in the status bar. The page tabs and the inspector's tabs are inset by `layout.container_margin`, as the side panel's settings are: gpui-component's Underline `TabBar` pads neither itself nor its tabs, so the first page tab sat against the panel divider. Where the theme states no toolbar padding, the toolbar row takes `layout.container_margin` and, failing that, the showcase's own 8px, and where it states no `toolbar.item_gap`, its buttons are `layout.widget_gap` apart and, failing that, `TOOLBAR_GAP`, the showcase's own 4px; its info says which; the menu-bar row borrows the same `container_margin` for its sides, with the same 8px where none is stated, because the model states no menu-bar inset. The Layout page shows one `Sidebar` expanded and one collapsed to its icons, which come from the chosen icon theme at `geometry::icon_size_small`, and a test checks that they fit their items; while the window manager draws the window's frame, it also shows a `TitleBar` sample, which neither moves nor zooms the window. The Icons page gains an Icon Sizes section: the chosen icon theme's icon at each of the five `defaults.icon_sizes` contexts, each reporting its own info and marking a native preset's size where platform-facts documents none.
- native-theme-gpui: `geometry::button` also sets the label's font weight from `button.font`, where it set none. The refinement lands on the button's outer element and GPUI cascades text style to the label, which sets its own size and no weight, so the platform's weight arrives and its size, deliberately not set, does not.
- native-theme: the KDE reader reads `text_scale.caption` from `[General] smallestReadableFont`, parsed like the body font, and derives `section_heading` and `dialog_title` from the body font it reads, as Kirigami's `Heading` levels 2 and 1 do (× 1.20 and × 1.35, weight 400; kirigami `8319acc`, `src/controls/Heading.qml:17-35`), so in live mode the text scale follows the user's fonts; it read neither, and kde-breeze-live's fixed sizes stood.
- The gpui showcase's page samples draw the chosen icon theme's icons, as its chrome already did: the Buttons page's icon Buttons, its loading Button's spinner and its Toggles, the input groups' Search icon and Copy button, the Attachments, the Empty state, the Plain Marker, the Collapsible's toggle, the icon Stepper, the Dialog's and the AlertDialog's icons and the application-drawn menu rows had gpui-component's own Lucide icons whatever icon theme was chosen. Where the chosen theme has no icon for a sample, the sample draws none — a Button or Toggle shows its label, a Stepper step its number — and never another icon theme's, and each sample's Widget Info names the icon theme or says the icon is missing in it. The icons gpui-component draws inside its own widgets — a Select's caret, a Checkbox's check, a Dialog's close button, a Spinner's Loader and the rest — stay gpui-component's: they are loaded by asset path, and gpui draws a path at a size once and keeps it in the window's sprite atlas, which an application cannot clear, so an asset source answering those paths from the chosen theme could not show a theme switch. Each such widget's Widget Info now says which of its icons are gpui-component's, and `docs/todo.md` records what gpui would need.

### Fixed

- native-theme-iced: `to_theme`'s doc listed `Tooltip` among the widgets iced gives a `Catalog` implementation; a tooltip has none and is styled as a container (`tooltip.rs:70`).
- The iced showcase's tooltip triggers take the theme's padding through `button_padding`, as its other push buttons already did, where they had iced's default, and its page tabs take `tab.border.padding` over iced's button default for a side the theme leaves unstated, where they had the showcase's own spacing. Its Widget Info no longer calls button and input padding "not themeable" — they are `button_padding` and `input_padding`.
- The gpui showcase's Widget Info panels said things that were not true, and **no public item changed** in fixing them. Forty-four defects in the panels themselves: eighteen claims dropped because nothing paints what they asserted (a `Popover`'s border — it has none, the edge is a shadow ring; an unchecked `Checkbox`'s fill; a `Carousel`'s slide fill and focus ring; a `WindowBorder`'s window background; both of `Pagination`'s); fourteen naming a token the widget never reads (`Toggle`'s four, `Dialog` and `Sheet` described as popups when they are surfaces, both menus' row hover, `Button`'s Link and Text variants); three with the right field under the wrong role; and a `Badge` count the panel called themeable, which is a hardcoded `white()`. Eight more came from the omission report, and every existing gate had been green on all of them — among them a `Button::new("b-secondary").label("Secondary")` that never called `.secondary()`, so it rendered `ButtonVariant::Default` while its panel named the four `button_secondary*` tokens (the cited lines do read those fields, in an arm the demo never reaches); a `ColorPicker` calling its whole palette hardcoded when the featured row is twelve platform colours; and a "Tag (per variant)" panel documenting one of six variants with its border claim labelled "(outline)" when `border` is the *Secondary* variant's and every `Tag` draws one.
- The same panels' **"Not themeable"** section was a claim nobody had checked, and checking it is the same question as "could the connector do something about it". 236 entries audited, **64 of them wrong**, and 15 more were themed facts filed under that heading — the button font weight `geometry::button` now carries, and icon sizes, paddings and gaps the demos apply themselves — which are config lines now. Three mistakes recur. "Hardcoded" was said of values upstream lets the caller's refinement override: a `Button`'s size-arm padding, the paddings of an `Alert`, `Tag`, `Kbd` or `Breadcrumb`, an `Editor`'s line height, an `EmptyMedia` frame. A literal was said of what is a rem — a `Bubble`'s padding, a `Message`'s and a `Marker`'s gaps — when gpui-component sets the rem to `Theme::font_size` (`root.rs:582`), which this connector fills from the platform's font: six button label sizes, the `Headings` ladder, several icon and indicator sizes. And "hardcoded" was said of what reads the theme: `Theme::motion` is a writable field of twelve tokens that at least eighteen widgets read, and the connector overwrites it with the default because native-theme models no motion — our gap, and the notes now say whose. Where a property really has no receiver the note now says **Tier U** — the platform states it, the model carries it, upstream offers nowhere to put it — instead of "hardcoded", which reads as *nothing can be done*: a `Link`'s underline and its hover and pressed text, a disabled `Button`'s three modelled fields, and an `Input`'s placeholder, painted with the shared `muted_foreground` where the model states `input.placeholder_color` from each platform's own placeholder colour. Others described upstream wrongly: a `Tree` draws no indent or chevron at all; a disabled `Button` shows the default cursor, not `not-allowed`; a `DatePicker` defaults to `%Y/%m/%d`; an `AvatarGroup`'s overflow marker is a `⋯` avatar that only `.ellipsis()` adds, not a `+N` count; Markdown headings are sized from a fixed 14px rather than the base font; a code `Editor`'s background is a fixed `#0a0a0a` or `#ffffff`, because the highlight theme the connector installs sets one and the platform fallback never fires; a candlestick's colours are the themed `chart_bullish` and `chart_bearish`; a `Badge`'s pill is out of every refinement's reach, since `Badge` refines the wrapper around the badged element; a `Button`'s tooltip waits a fixed 500ms, where only a tooltip set on an element takes a delay; an unchecked `Checkbox`, a `Radio` and an `OtpInput` box are filled with `input_background()`, where the notes said they were not; a `Radio`'s indicator is a circle; a `Textarea`'s rows are `Input`'s 1.25rem, not the font's line height; and two Tier U verdicts were wrong the other way — a `Toggle` folds the caller's refinement into its checked style, so `segmented_control`'s active colours and size reach it, and `Switch::color` is a per-instance receiver for the model's `switch.checked_background` that nothing feeds. Notes gained `file.rs, Symbol` citations as they were corrected, so the existing gate now holds them to upstream too. What the audit found that is *ours* to fix — motion, the unwritten font weight, the editor background, the heading size, the unmodelled widgets — is filed in `docs/todo.md` rather than fixed here.
- At the time of the audit, thirty of the 374 Widget Info **colour claims named a token the widget never paints with**, and the citation gate passed every one, because it checks that the cited line reads the token, not that the line runs for this widget in this demo. The navigation `TabBar` is an Underline bar: it has no fill and marks the active tab with a `primary` underline, where the panel listed the `Tab` variant's `tab_active` and `tab_bar`. A `Calendar` has an edge and no `popover` fill; a `Collapsible` paints neither the `accordion` fill nor the border it was given; a `Rating`'s empty star and a `Pagination`'s page numbers are not `foreground` or `muted_foreground`; a `Combobox` row hovers with `accent`, not `list_hover`; a `GroupBox`'s edge is the card's, through `geometry::group_box_content`; a named `Avatar` takes no theme colour at all — its fill, initials and edge are OKLCH literals hashed from the initials — so four claims described an anonymous avatar the demo does not show; an `OtpInput`'s digits are `foreground`, not a masked asterisk's `secondary_foreground`; and `MessageScroller`'s jump button is refined to `background` over its `secondary` variant. The theme's `shadow` flag, listed as config on eleven panels, has one reader in gpui-component — a `Custom` button built with `.shadow(true)` — so those lines are gone. The commonest cause was a `geometry::` builder the demo applies: seven carry a colour and ten a corner radius, and they land last, so a Radio's and a Select's label are the platform's font colour rather than `foreground` (both claims now say "upstream"), and the "border-radius" line on eighteen panels showed a radius the builder replaces and is gone. Others described what the demo does not build — an icon badge, a `Label`'s highlights, an outline `Kbd`'s edge, an empty `ColorPicker` swatch — or what nothing paints: `ThemeColor::list` on List and Tree, `group_box` on a Settings page whose groups are unfilled, and a code `Editor` edge that is the Input frame's `input` while the claimed `border` paints its indent guides. Twenty-five more had the right token at a line borrowed from another widget, and five config lines named a radius or a size that is not the one on screen, among them a `ProgressCircle` that draws `geometry::spinner_size` at 75%.
- The gpui showcase's **`AppMenuBar` rendered no menus**. Since gpui-component 0.6.0 it reads gpui-base's `GlobalState` list, which only `set_app_menus` fills, and the showcase called only gpui's `cx.set_menus`, which feeds the platform's own menu bar; it now gives both the same menus. Its **`Loading Button`** showed no spinner either: a `Button` draws its spinner in place of its icon, and that one had none.
- Two theme slots nothing paints are now on the record rather than invisible: `ThemeColor::tab` is written on every `apply` and read by no widget in gpui-component or gpui-base, and `ThemeColor::list_even` is read only as `table_even`'s fallback — which never fires, because the connector writes `table_even` too. Both had passed every gate: they are written, so no coverage check noticed, and they are read nowhere, so no widget could disagree with them. Recorded in `docs/todo.md` with the lines that prove it; nothing is removed pre-1.0.
- native-theme-gpui: a tooltip's text no longer runs out of its bubble. `geometry::tooltip` put `tooltip.max_width` on the bubble, but a tooltip's content sits in a bare `div()` inside gpui-component's `h_flex()`, so it is a flex item with an automatic minimum size, and gpui measures text under `AvailableSpace::MinContent` without wrapping it — the item's minimum was the whole unwrapped line, which the bubble's maximum could not shrink. The width is now `geometry::tooltip_content`'s, on the content element, where it is what the text wraps at; a seam test lays a real `Tooltip` out under kde-breeze, where the line measured 1656px against the 292px the platform leaves inside the bubble.
- native-theme-gpui: a scrollbar that is not an overlay no longer sits on the content. gpui-component overlays its bar on the scroll area whatever the platform does, and where `scrollbar.overlay_mode` is false the connector also asks for an always-visible bar, so on KDE its 21px groove covered the right edge of every scrolling pane. `geometry::scrollbar_gutter` reserves that width beside the content; the showcase applies it to each of its `overflow_y_scrollbar` containers and to its Settings groups. The iced connector already embedded its bar the same way (`styles::scrollbar` sets `Scrollbar::spacing` when the platform's bars are not overlays), and its showcase applies it to every scrollable, so iced never had this.
- native-theme-gpui: an animated icon no longer flickers while gpui decodes each of its frames for the first time. The connector handed gpui encoded bytes (`ImageSource::Image`), which gpui decodes in the background, and an element holding one paints nothing until that finishes — so every frame of an animation was blank the first time it came up, and again after an icon-theme change built new images. It now builds a `gpui::RenderImage` from the RGBA it had already rasterised and returns `ImageSource::Render`, which gpui answers from the value itself; the pixels are converted exactly as gpui's own decoder converts them. `image` becomes a normal dependency of the crate, at the version gpui-pre resolves.
- The gpui showcase framed its List and its Tree with a square grey box of its own while the Table beside them was rounded: upstream's `DataTable` draws its own border from the theme's radius, and `List` and `Tree` draw none, so the frame around them was the application's — here a 1 px box in a colour no theme states. Both now take the new `geometry::list`, so all three agree under every preset. Every other box, card and bordered row the showcase draws for demonstration goes through one helper that reads `Theme::border`, `Theme::radius` and the platform's `defaults.border.line_width`; the placeholder fills and the reduced-motion labels read their colours from the theme; and a test beside the builder-coverage test lexes the showcase and fails on any radius helper, any radius written as a number, any colour setter given a colour constructor and any text size given in pixels. Twenty-five such values were in the file.
- A wheel turned inside the gpui showcase's List, Tree, scrollbar demo, Settings page, code editor, message scroller or data table no longer scrolls the page behind it as well. gpui dispatches a scroll wheel to every scroll container under the pointer and stops at none of them, and no gpui-component widget takes itself out of that list, so both moved by the same delta. Each demo box now takes the pointer out of its ancestors' hit test. The underlying upstream gap, and what a consumer can and cannot do about it, is recorded in `docs/todo.md`.
- native-theme: `checkbox.indicator_color` — the check mark and the radio dot — inherits `defaults.accent_text_color`, not `defaults.text_color`. `docs/platform-facts.md` §2.5 records the mark as the on-accent colour on all four platforms (macOS white, Fluent `TextOnAccentFillColorPrimary`, KDE `[Colors:Selection] ForegroundNormal`, Adwaita white), and it is painted on `checkbox.checked_background`, which inherits `defaults.accent_color`. No bundled preset states the field and no OS reader writes it, so every preset painted the window's text colour on the accent fill — 1.12:1 on one-dark dark, 1.16:1 on solarized dark. In the iced connector's contrast report the idle check-mark pairs below AA fall from 29 of 32 to 16 of 32, the radio dot's with them. gpui is unaffected: gpui-component draws the check mark with `primary_foreground` (`checkbox.rs:193-195`), which the connector feeds from `button.primary_text_color` — itself an inheritor of `defaults.accent_text_color`, so that mark was already the on-accent colour.
- The local release gate now sees what the tag-time gate sees, and the other way round. `pre-release-check.sh` documents with `RUSTDOCFLAGS="-D warnings"`, as CI, the nightly canary and the publish workflow already did, and all four document `native-theme-iced` with `--all-features` as well — the configuration docs.rs builds and no gate covered. The publish workflow's gate also runs what CI and `pre-release-check.sh` run and it did not: the iced connector's tests without default features and with `iced_aw`, its clippy with `iced_aw`, the widget-coverage script, and `system-icons` alone on Windows and macOS; the nightly canary's clippy and tests cover the same iced configurations; and the screenshots workflow builds the iced showcase with `iced_aw`, as the local capture scripts do. The canary's coverage and documentation steps also run with `if: always()`, so a test failure no longer hides them.
- native-theme-gpui: `link_hover` and `link_active` are the platform's link *text* colours (`link.hover_text_color`, `link.active_text_color`). gpui-component reads both tokens as a foreground, and the connector wrote the link's hover *fill* — about 9 % alpha — into the first and derived the second, so a hovered link sat at 1.06–1.22:1 on all 32 preset/mode combinations.
- native-theme-gpui: list and table headers take `list.header_background` and `list.header_font.color` instead of the window's background and muted colour (the two fills differ in nearly every preset — kde-breeze light `#ffffff` against `#eff0f1`), and the segmented tab bar's track takes `segmented_control.background_color`.
- native-theme-gpui: a filled button's hover and pressed colours are composited over the button's own fill. Windows 11 gives them as 4 % layers, and gpui-component replaces a background where the platform layers it, so a hovered button landed on `#e9e9e9` where Windows draws `#f3f3f3`. No other preset changes.
- The gpui showcase never drew a dialog, a sheet or a notification it opened: gpui-component's `Root` does not mount those layers itself, and the showcase did not either, so every such control did nothing visible. Long-standing, not a 0.6.4 regression; found by the new self-tests. Its Light/Dark/System selector also did nothing when no theme could be read, because the three failure paths returned before changing the mode.
- native-theme-gpui 0.5.8 no longer compiled on a fresh dependency resolution: gpui-component 0.6.2 (2026-09-18) removed `ThemeColor::tiles`, which the connector wrote, in a patch release that the connector's caret requirement admits. The nightly dependency canary reported it the same evening.
- The gpui showcase's Settings rows no longer sit under an always-visible scrollbar: gpui-component lays the Settings page's scrollbar over the body's right edge and reserves only 1rem, so kde-breeze's 21px groove covered the rows. Each `SettingGroup` takes `geometry::scrollbar_gutter`, which the group applies to its root.
- The gpui showcase's List, Tree and DataTable rows show the theme's list font. The showcase wrapped each row's text in a `Label` at `text_sm`, which also paints `foreground` itself (`label.rs:211`), so the size and colour `geometry::list_item` gives the row never reached its text.
- The gpui showcase's icons come from the icon theme it names. Choosing gpui-component's built-in icons showed placeholders for all 101 icons and the previous icon theme's icons under the label "lucide", and a theme switch then loaded bundled Lucide in its place; the freedesktop loading spinner came from the system icon theme whichever theme the icons came from; a "fallback" label marked any system icon byte-identical to Material's, although the loaders never substitute; and `--icon-theme` without `--icon-set` named the theme it was given while the icons on show were the ones loaded before the flag was read. Each now loads, and names, one icon theme.
- Documentation of `geometry::input_height` and the README's builder table: `Input::h` sets the height of a multi-line input only (`input/input.rs:706-709`); a single-line input takes it through `Styled::h`. `input_height` itself now returns a `StyleRefinement`, applied with `refine_style` (see `Breaking Changes`).
- Documentation of `geometry::menu_item`: gpui-component has no public menu-item element to apply it to — `MenuItemElement` is crate-private and `PopupMenu` builds its own rows — so it styles a menu row the application draws itself.
- Documentation of `geometry::dialog`: gpui-component 0.6.4 clamps the dialog to what is left of the viewport *after* applying the caller's style, so the theme's `dialog.max_height` no longer reaches it. `min_height`, the paddings and `Dialog::max_w` still do.
- The crates.io workflow's upload job installs the same system packages as its CI gate: the gpui-pre 0.3 stack needs `fontconfig.pc` when the upload step verifies the tarball, and the v0.5.8 run failed there after the four other crates had been uploaded; the gpui connector was uploaded by a later run of the fixed workflow. The upload steps recognise cargo's current "already exists on crates.io index" message, so a re-run skips crates that are already up instead of failing on the first one.
- The gpui showcase's `Popover` and `HoverCard` were padded 0 on every preset, because no preset stated a popover padding and the resolver filled in 0. They draw the platform's padding where one is sourced (GNOME 8, Windows 15/16/17/16) and gpui-component's own `p_3` elsewhere. Where a preset states no status-bar or dialog padding — the colour-scheme presets, material and ios, and the status bar on macos-sonoma and windows-11 — those two likewise draw gpui-component's padding instead of 0.
- native-theme-gpui: under `reduce_transparency` the overlay behind a dialog or sheet was fully opaque `defaults.shadow_color`, so opening the showcase's Preferences with the preference on turned the whole window black. The overlay is now transparent, gpui-component's own colour for no overlay; the dialog or sheet stays modal. macOS asks for no semitransparent backgrounds under the preference, and an opaque one hid the window.
- native-theme: the macOS reader reports no text-scaling factor. It divided `NSFont.systemFontSize` by 13, but macOS's text size setting reaches only a few Apple apps and `preferredFont(forTextStyle:)` still returns fixed sizes (`docs/platform-facts.md` §2.1.7), so the factor was invented, and a connector would have applied it on top of font sizes that already carry the system's own.
- native-theme: the `system-icons` feature builds on macOS and Windows. It enabled only the Linux and CoreGraphics dependencies, while its SF Symbols and Windows icon loaders import `objc2`, `objc2-foundation`, `objc2-app-kit` and `windows`, which only the `macos` and `windows` features enabled; so `system-icons` without them did not compile there. native-theme-iced, which now enables `system-icons` by default, would have failed to build on macOS and Windows with default features, and its docs.rs build for those two targets the same way (0.5.8 had no features and did not enable `system-icons`; the gpui connector was unaffected: it enables `macos` and `windows` on their platforms). `system-icons` now enables those four crates as well, each only on its own platform, and CI's macOS and Windows legs, the publish workflow's gate and `pre-release-check.sh` (where the target is installed) check the feature alone.
- native-theme: the text scale follows `docs/platform-facts.md` §2.19 (`:1433-1438`). **kde-breeze** and **kde-breeze-live**: `caption` is 8pt, the default of `smallestReadableFont` (`:573`), where it was 8.2, the product of a ratio no source gives; `section_heading` and `dialog_title` are weight 400, where they were 700, because Kirigami's `Heading` is `Font.Normal` unless its `type` is `Primary`, and `type` defaults to `Normal` (kirigami `8319acc`, `src/controls/Heading.qml:35`, `src/templates/Heading.qml:89`); and `display`, 20pt 700, is no longer stated: KDE has no display style, so it resolves from the body font. **macos-sonoma-live** states the full preset's text scale — `caption` 10pt (was 10.7), `dialog_title` `.title1`'s 22pt 400 (was 15.6pt 700) and `display` `.largeTitle`'s 26pt 400 (was 700); macos-sonoma itself now states the weights and the section heading's 13pt it already resolved to. The `-live` presets' text-scale line heights (macOS 12.7, 15.5, 18.6 and 30.9pt, KDE 11.2, 16.3, 18.4 and 27.2pt) are gone: they were derived from those sizes and cited nothing, and where platform-facts gives no line height the resolver computes one. The documented-sizes gate gains a row per platform for the text scale's sizes and weights, the dialog title font, the slider and the progress bar, and fails a native preset that states a text-scale line height.
- native-theme: `dialog.title_font` is stated where platform-facts documents it (§2.22, `:1496-1497`): on macos-sonoma 13pt Bold, the emphasized system font (macos-sonoma-live states the weight; the size is the full preset's 13pt, over which the pipeline merges it), and on adwaita and adwaita-live `.title-2`'s 15pt ExtraBold (800). Neither stated one, so a dialog's title took the body font: 13pt 400 on macOS, 11pt 400 on GNOME.
- native-theme: slider and progress-bar sizes follow platform-facts §2.9–§2.10 (`:1292-1293`, `:1303`). windows-11 and windows-11-live state the progress bar's track as WinUI's 1px groove, `ProgressBarTrackHeight`, where they stated 3, the control's minimum height; the Windows reader's slider thumb is 18 (was 22) and its progress track 1 (was 4); and the macOS reader's slider track is NSSlider's 5 (was 4).
- native-theme: the Windows reader's lengths are logical pixels, the model's unit. It read its system metrics — the scrollbar's width and minimum thumb length, the focus border and the icon sizes, and in 0.5.8 also `menu.row_height` from `SM_CYMENU`, which is now WinUI's constant 23 (the sizes entry under `Changed`) — with `GetSystemMetricsForDpi` at the system DPI, which in a DPI-aware process gives device pixels (26 instead of 17 for `SM_CXVSCROLL` at 150 %); it now asks at 96 DPI (`USER_DEFAULT_SCREEN_DPI`). Its fonts likewise: it read `NONCLIENTMETRICSW` at the system DPI and reported that DPI as `font_dpi`, so at 144 DPI a 9pt font resolved to 18px where the logical size is 12; it now reads them through `SystemParametersInfoForDpi` at 96 DPI and reports a `font_dpi` of 96, and the DPI the resolver falls back to on Windows when no reader provides one is 96 too, where it was `GetDpiForSystem()`. At 100 % scaling nothing changes.
- native-theme: the Windows reader sets `defaults.border.line_width` from `SM_CXBORDER` and the macOS reader `defaults.border.color` from `NSColor.separatorColor`, the sources platform-facts names; each read nothing into the field before (the Windows helper had no caller, and the macOS colour was read and dropped). windows-11-live no longer states the border width the reader provides.
- native-theme: every feature builds alone. `watch` without `portal` did not compile — `zbus` without default features needs a runtime feature, which only `ashpd`'s `async-io` turned on — so `portal` now names the `zbus` dependency and `watch` enables its blocking API only with it; `watch` with `macos` did not compile on macOS, because the macOS watcher imports `objc2-core-foundation`, which only `system-icons` enabled, and `macos` enables it now.
- native-theme-gpui: the crate builds without `svg-rasterize`. It called `native_theme::rasterize` whatever the features, so turning the feature off, as its manifest allows, failed to compile. Without it an SVG icon is now handed to gpui undecoded, as an `ImageSource::Image` of `ImageFormat::Svg` holding the colourised bytes, which gpui decodes with its own resvg: the first frame paints nothing and gpui chooses the raster size, as `to_image_source`, the README and the crate docs say. No icon is lost for want of the feature.
- The gpui showcase's `--variant` without `--theme` changed only the colour-mode labels; it now installs the current theme in that mode.
- The gpui showcase's icon-theme Select was rebuilt only while the choice was the preset's `default`: a choice that fell back to `system` because the preset's icon theme is not installed stopped following the preset, and a theme the user picked kept offering an earlier preset's `default` row. The rows are rebuilt on every theme install, and a choice that followed the preset keeps following it. `--icon-theme` overrode every later pick; it now applies until the user picks an icon theme.
- The gpui showcase's preset switch labelled `default` with `platform_preset_name`'s guess, which can differ from the preset the pipeline settles on and installs; it now names the installed one, as the status bar does. A theme that failed to load put its own name in place of the installed theme's in the preset switch, and one that failed to resolve its layout in place of the installed theme's too; and the failure ran upstream's mode switch, which rebuilt the palette from its stored configs — recomputing derived colours such as the buttons' hover and active shades, and showing upstream's own colours where the other mode had no native variant stored. A failed install now leaves the window as the installed theme drew it: its name, layout, mode and every colour, the colour-mode Select included.
- Both showcases reject a command-line value they cannot honour, reporting it on stderr and leaving the setting as it would have been without the flag, where they replaced it silently. `--variant` takes `light`, `dark` or `system`, which follows the OS; any other value, `system` included, gave the light mode. `--theme` takes `default` or a preset the showcase's preset list offers on the platform it runs on, checked before anything is installed, so a rejected `--theme` leaves `--variant` to apply; a macOS or Windows preset on Linux is refused with a message saying that only this platform's presets run here. `--icon-set freedesktop` is the system icon theme in both; the iced showcase read any name other than `material`, `lucide` and `system` as a freedesktop theme of that name, and now takes only one its icon-theme picker lists, while the gpui showcase replaced any unknown name with the system theme and now takes what its icon-theme Select lists, `gpui-builtin` included, and while `--icon-theme` stands over a freedesktop set the Select names that theme. The gpui showcase's `--icon-theme` also takes only a theme the Select lists, where it took any name: `hicolor`, which the Select does not list, is refused, and so is every value off Linux, where the list is empty. `--tab` reports a name that is no page or tab of the showcase, where it was ignored silently, and where no freedesktop icon theme is installed the icon flags' reports say so rather than end on an empty list.
- The iced showcase's theme and colour-mode pickers show the installed theme and the mode it is drawn in when a theme fails to load, where they showed the choice that failed; the choice and the mode are committed only when the install succeeds, as in the gpui showcase. An OS theme that cannot be read after startup is such a failure too, where it drew adwaita under the `default` entry; only at startup, with nothing installed yet, does adwaita stand in, and the theme picker then names adwaita, where it named `default`. Its freedesktop loading spinner comes from the chosen icon theme, and there is none where that theme has none, where it came from the system theme whichever theme the icons came from. An icon choice that followed the preset onto the system theme keeps following the preset; only the user's own pick stops it.
- native-theme: a freedesktop icon comes only from the chosen theme or a theme its `Inherits=` chain declares. `freedesktop-icons` 0.4.0 looks a missing icon up in `hicolor`, then takes a loose file from an icon base directory or `/usr/share/pixmaps`, and looks in `hicolor` from the start when the theme is not installed, so `FreedesktopLoader::load`, `load_indicator` and a custom icon's freedesktop name could return another set's icon (on CachyOS the name `cachyos` looked up in breeze gave the loose `/usr/share/icons/cachyos.svg`, and an application icon breeze lacks gave `hicolor`'s). Such a result is now `None`, and so is every icon of a theme that is not installed; `hicolor` stays allowed when it is the theme asked for.

## [0.5.8] - 2026-09-07

### Breaking Changes

#### native-theme-gpui

- Moved to **gpui-component 0.6.0**, **gpui-base 0.6.0** and GPUI published as **`gpui-pre` 0.3.x**; the crate is type-incompatible with applications on gpui-component 0.5 / gpui 0.2.
- `to_theme(resolved, name, is_dark, reduce_transparency: bool)` → `to_theme(resolved, name, is_dark, prefs: &AccessibilityPreferences)`.
- `from_preset(name, is_dark)` → `from_preset(name, is_dark, prefs: &AccessibilityPreferences)`.
- `lucide_name_for_gpui_icon`, `material_name_for_gpui_icon`, `freedesktop_name_for_gpui_icon` return `Option<&'static str>`; `StarFill` has no Lucide equivalent and `StarOff` no Material one, both return `None`. The freedesktop table maps `Star` to `non-starred` (was `starred`, the filled star), so `Star` and `StarFill` render as distinct states. The Lucide table returns Lucide's own names (`Close` → `x`, `Dash` → `minus`, `Inspector` → `scan`, `ResizeCorner` → `grip`, `SortAscending` → `arrow-up-narrow-wide`, `SortDescending` → `arrow-down-wide-narrow`, `WindowMaximize` → `maximize`, `WindowRestore` → `minimize-2`); the Material table maps `ALargeSmall` to `format_size` (was `font_size`) and `MemoryStick` to `memory_alt`.
- `ScrollbarShow` → `ScrollbarMode`; `IconName::GitHub` → `IconName::Github` (upstream renames).
- The crate declares its own `rust-version` (1.95.0), higher than the workspace's 1.88.0, because the gpui-pre closure requires it.

#### native-theme

- Bundled icon names follow upstream: the Lucide bundle no longer contains `close`, `dash`, `inspect`, `resize-corner`, `sort-ascending`, `sort-descending`, `window-close`, `window-maximize`, `window-minimize`, `window-restore` (byte-identical duplicates of `x`, `minus`, `scan`, `grip`, `arrow-up-narrow-wide`, `arrow-down-wide-narrow`, `maximize`, `minimize-2`) or `trash-2` (now `trash`); the Material bundle no longer contains `star_border` (a duplicate of `star`) and stores `font_size` under its upstream name `format_size`. `LucideLoader::new` / `MaterialLoader::new` with those names return `None`; `icon_name(role, IconSet::Lucide)` returns `trash` for the three trash roles.

### Added

- **native-theme-gpui**: `apply`, `apply_system_theme` (a `ThemeConfig` is installed for every stored variant, so upstream's `Theme::change` reproduces native colours in both modes; the 12 base-palette colours `ThemeConfigColors` keeps private are restored by the observer after every rebuild), `apply_accessibility` (rebuilds the styled theme from the stored variant at runtime); `NativeTheme` global with `cx.native_theme()` (`ActiveNativeTheme`); `Native` view; `base_layer` module (native scrollbar geometry/colours and resize-handle colours written onto gpui-base, restored automatically after upstream rebuilds the base theme); `geometry` module (per-widget `StyleRefinement` builders for Button, Input, MenuItem, ListItem, Tooltip, Popover, StatusBar, Dialog family, Table, Progress, GroupBox content, Accordion title, Checkbox, Radio, Select, Combobox, TitleBar; `Size` helpers; layout accessors); text scaling through `Theme.font_size` (rem); reduce-motion forwarded to GPUI; `focus_ring` from `focus_ring_width`; the 15 new `IconName` variants covered in all three tables (`StarFill` has no Lucide equivalent and `StarOff` no Material one; both return `None`).
- **native-theme**: `SystemTheme.layout: LayoutTheme`; `AccessibilityPreferences::from_system()`; 28 bundled SVGs (14 Lucide, 14 Material); `icons/SOURCES.toml` provenance manifest; `scripts/refresh-icons.sh` (the bundle reproduces byte-for-byte from the manifest); by-name icon tables generated by `build.rs` from the bundle directories (four Lucide and eleven Material files the hand-written tables had missed now resolve).

### Changed

- Declared explicit `[package.metadata.docs.rs]` targets (`x86_64-unknown-linux-gnu`, `x86_64-apple-darwin`, `x86_64-pc-windows-msvc`) with `all-features = true` on `native-theme` and `native-theme-iced` (`native-theme-gpui` keeps `all-features = true` only; see the `native-theme-gpui` docs.rs bullet in this section). Preserves multi-target API rendering on docs.rs after the 2026-05-01 policy change that reduces the default build set to a single target ([announcement](https://blog.rust-lang.org/2026/04/04/docsrs-only-default-targets/)). Platform-gated items such as `LinuxDesktop`, `winicons`, `sficons`, and the macOS/Windows readers remain visible across the three published target docs.
- **native-theme-gpui**: `ThemeColor` mapping 108 → 139 fields; the 28 `button_*` fields take the semantic values their variant used in 0.5.1 (solid native surfaces, not upstream's tint); `table_foot*` mirror `table_head*`; `status_bar*` from `StatusBarTheme`. The `ThemeConfig` copies carry upstream's default highlighter style for their mode, so a mode switch through `Theme::change` also switches code highlighting. Showcase on gpui-kit 0.6.0.
- **native-theme-gpui**: `[package.metadata.docs.rs]` keeps `all-features = true` and declares no `targets`: every platform-gated public item of the crate is Linux-gated and appears on the default target (v0.5.8 spec §1.3, D40).
- Dependency refresh: serde 1.0.229, serde_with 3.22.0, toml 1.1.5, serde_json 1.0.151, arc-swap 1.9.2, async-trait 0.1.92, ashpd 0.13.13, configparser 3.2.0, zbus 5.19.0, quote 1.0.47, proc-macro2 1.0.107, pollster 1.0, syn 3.0.5, resvg 0.48.1. Workspace MSRV re-measured: unchanged at 1.88.0 (`resvg` is used without default features, so its text stack never enters the lock).

- Release tooling: `scripts/pre-release.sh` now records the provenance of the visual assets in `docs/assets/PROVENANCE.toml` (workspace version, commit, and a hash over the git object ids of every path that feeds the showcases); `pre-release-check.sh` recomputes the hash at HEAD (a warning while the CHANGELOG entry is unreleased, a hard failure once it is dated) and the crates.io workflow's CI gate refuses a tag whose assets were captured from other sources or whose name differs from the workspace version.

### Fixed

- Icon bundle provenance recorded; Lucide refreshed to 1.41.0 (`github.svg` kept from 0.577.0, the last tag with brand icons); Material Symbols refreshed to upstream `0cbb08816df0`; duplicate `star_border.svg` removed; the old Lucide `delete.svg` was a trash-can glyph and is now Lucide's real `delete` (the backspace key gpui-kit's own icon draws).
- The connector honoured only `reduce_transparency`; the platform's text-scaling factor and reduce-motion preference were dropped.
- The `ThemeConfig` hex export dropped alpha, so `overlay`, `drag_border` and `drop_target` turned opaque after `Theme::change`; translucent colours are now exported as `#rrggbbaa`. The config's colour copy also ignored `reduce_transparency`, so a `Theme::change` round trip restored translucent colours for a user who had asked for opaque.
- `publish.yml`: the gpui connector is hard-gated again (the naga/codespan-reporting conflict G11 recorded does not exist on the 0.6 stack).
- Pre-existing test-only clippy failures in `native-theme` (spinners, freedesktop, icons, kde, `tests/reader_kde.rs`) fixed so `cargo clippy --all-targets -- -D warnings` passes on the whole crate.
- The `chunks_exact(4)` pixel loops (`unpremultiply_alpha`, the Windows `winicons` BGRA swap, a `rasterize` test, the BMP export in `native-theme-gpui`, the Windows screenshot code of both showcases) use `as_chunks::<4>()`, stable since Rust 1.88.0 (the workspace MSRV), so clippy 1.98's `chunks_exact_to_as_chunks` lint passes under `-D warnings`.
- CI installs `libfontconfig1-dev` and `libfreetype-dev` on every job that builds the gpui connector: the gpui-pre 0.3 stack builds `yeslogic-fontconfig-sys` without `dlopen`, so `cargo check`, clippy and rustdoc need `fontconfig.pc` on the runner (the 0.5 stack did not). `.gitignore` no longer lists `.github/`, which had silently excluded new workflow files since v0.5.0. The `watch` module's docs had four unresolved intra-doc links under the `portal` feature; the links are absolute now.

## [0.5.7] - 2026-04-21

> **Major API overhaul.** This release renames the four core vocabulary types,
> restructures the crate into public submodules, replaces free icon-loading
> functions with a builder API, migrates strings to zero-copy types, and
> replaces the `define_widget_pair!` macro with a proc-macro derive.

### Breaking Changes

#### Type renames

| Old | New |
|-----|-----|
| `ThemeSpec` | `Theme` |
| `ThemeVariant` | `ThemeMode` |
| `ResolvedThemeVariant` | `ResolvedTheme` |
| `ResolvedThemeDefaults` | `ResolvedDefaults` |

#### Module restructure

The crate root is now partitioned into public submodules. Most types that were
previously re-exported at the crate root are now accessed through their module:

- `native_theme::theme::*` -- `Theme`, `ThemeMode`, `ResolvedTheme`, `ResolvedDefaults`, `IconSet`, `IconRole`, `AnimatedIcon`, etc.
- `native_theme::icons::*` -- `IconLoader`
- `native_theme::detect::*` -- `system_is_dark()`, `prefers_reduced_motion()`, `LinuxDesktop`, etc.
- `native_theme::color::*` -- `Rgba`
- `native_theme::error::*` -- `Error`, `ErrorKind`
- `native_theme::resolve::*` -- `ResolutionContext` (post-G7; inheritance and validate internals stay `pub(crate)`)
- `native_theme::prelude` -- convenience re-exports (`Theme`, `ResolvedTheme`, `SystemTheme`, `AccessibilityPreferences`, `ResolutionContext`, `Rgba`, `Error`, `Result`)

#### Icon loading API

The 13 standalone icon-loading functions (`load_icon`, `load_custom_icon`,
`load_icon_from_theme`, `load_system_icon_by_name`, `loading_indicator`, etc.)
are replaced by `IconLoader`, a single fluent builder:

```rust,ignore
// Before
let icon = load_icon(IconRole::ActionCopy, IconSet::Material, None);
let anim = loading_indicator(IconSet::Material);

// After
let icon = IconLoader::new(IconRole::ActionCopy).set(IconSet::Material).load();
let anim = IconLoader::new(IconRole::StatusBusy).set(IconSet::Material).load_indicator();
```

#### String type migrations

- `Theme.name`, `SystemTheme.name`, `ThemeDefaults.icon_theme` -- `String`/`Option<String>` → `Cow<'static, str>`/`Option<Cow<'static, str>>`
- `FontSpec.family`, `ResolvedFontSpec.family` -- `Option<String>`/`String` → `Option<Arc<str>>`/`Arc<str>`
- `IconData::Svg` -- `Svg(Vec<u8>)` → `Svg(Cow<'static, [u8]>)`
- `IconProvider::icon_svg()` return type -- `Option<&'static [u8]>` → `Option<Cow<'static, [u8]>>`

#### Error restructure

`Error` is now a flat, `#[non_exhaustive]` enum with 9 variants. `Error::kind()`
returns `ErrorKind` for coarse dispatch.

#### Other breaking changes

- `ColorMode` enum replaces `is_dark: bool` on `SystemTheme`; `SystemTheme.mode` is now `ColorMode`, `pick()` takes `ColorMode`
- `ThemeMode::into_resolved()` signature changed from `(font_dpi: Option<f32>)` to `(ctx: &ResolutionContext)` (see G7 ResolutionContext section below)
- `BorderSpec` split into `DefaultsBorderSpec` (for `ThemeDefaults`) and `WidgetBorderSpec` (for per-widget use)
- `AnimatedIcon` variant fields made private via `FramesData`/`TransformData` wrappers; duration fields now `NonZeroU32`
- `ThemeChangeEvent::ColorSchemeChanged` renamed to `ThemeChangeEvent::Changed`; `Other` variant removed
- `FontSize::to_px()` renamed to `FontSize::to_logical_px()`
- `detect_linux_de()` split into `parse_linux_desktop()` (pure) and `detect_linux_desktop()` (reads env)
- `icon_set` and `icon_theme` relocated from `ThemeMode` to `Theme`/`ThemeDefaults`
- `from_toml_with_base()` removed
- Platform reader functions (`from_kde`, `from_gnome`, `from_macos`, `from_windows`) demoted from `pub` to `pub(crate)`
- Feature flags `portal-tokio` and `portal-async-io` replaced by single `portal` feature (async-io only)
- `from_system()` and `from_system_async()` unified; `from_system()` uses `pollster` for sync-over-async on Linux

#### Icon loading API — typed per-set loaders (Phase 93-09)

The `IconLoader` builder introduced earlier in this release is itself replaced
by five typed per-set loader structs. Phase 93-03 exposed a silent-ignore bug
where `IconLoader::new(name).set(Freedesktop).theme("Adwaita").load()` silently
dropped `.theme()` for string-name lookups; this migration makes that class of
bug impossible by construction — calling a set-specific method on the wrong
loader is now a compile error, not a silent no-op.

```rust,ignore
// Before (IconLoader)
let icon = IconLoader::new(IconRole::ActionCopy).set(IconSet::Material).load();
let fd   = IconLoader::new("edit-copy").set(IconSet::Freedesktop).theme("Adwaita").size(24).color([0,0,0]).load();
let anim = IconLoader::new(IconRole::StatusBusy).set(IconSet::Material).load_indicator();

// After (typed per-set loaders)
let icon = MaterialLoader::new(IconRole::ActionCopy).load();
let fd   = FreedesktopLoader::new("edit-copy").theme("Adwaita").size(24).color([0,0,0]).load();
let anim = MaterialLoader::load_indicator();
```

| Old | New |
|-----|-----|
| `IconLoader::new(id).set(IconSet::Freedesktop).theme("X").size(24).color(c).load()` | `FreedesktopLoader::new(id).theme("X").size(24).color(c).load()` |
| `IconLoader::new(id).set(IconSet::Material).load()` | `MaterialLoader::new(id).load()` |
| `IconLoader::new(id).set(IconSet::Lucide).load()` | `LucideLoader::new(id).load()` |
| `IconLoader::new(id).set(IconSet::SfSymbols).load()` | `SfSymbolsLoader::new(id).load()` |
| `IconLoader::new(id).set(IconSet::SegoeIcons).load()` | `SegoeIconsLoader::new(id).load()` |
| `IconLoader::new(id).set(set).load()` (runtime set) | `load_icon(id, set)` (free fn) |
| `IconLoader::new(id).set(set).load_indicator()` (runtime set) | `load_icon_indicator(set)` (free fn) |

`FreedesktopLoader::load_indicator(theme: Option<&str>)`, `MaterialLoader::load_indicator()`, and `LucideLoader::load_indicator()` are associated functions (no `self`). `SfSymbolsLoader` and `SegoeIconsLoader` do not have `load_indicator` — those sets have no animated spinner, and calling it is a compile error rather than a silent `None`.

As a secondary fix, `freedesktop::load_freedesktop_spinner` now accepts
`theme: Option<&str>` to honor theme overrides for animated spinners,
closing a latent silent-drop that existed since the original freedesktop
spinner support.

#### Resolution-time inputs — `ResolutionContext` (Phase 94-02, G7)

The `font_dpi: Option<f32>` parameter on `ThemeMode::into_resolved`, the
matching field on the internal `OverlaySource`, and the implicit
`platform_button_order()` / `system_icon_theme()` calls inside
`resolve_platform_defaults` / `run_pipeline` are consolidated into a
first-class `ResolutionContext` struct that bundles the three
resolution-time inputs captured from the OS.

```rust,ignore
// Before
let resolved = variant.into_resolved(None)?;           // auto-detect DPI
let resolved = variant.into_resolved(Some(96.0))?;     // explicit DPI

// After
use native_theme::ResolutionContext;

let resolved = variant.resolve_system()?;              // OS-detected
let resolved = variant.into_resolved(&ResolutionContext::from_system())?;
let resolved = variant.into_resolved(&ResolutionContext::for_tests())?; // tests
```

`ResolutionContext` carries:
- `font_dpi: f32` — not `Option<f32>`; the `None`→`system_font_dpi()` fallback
  is resolved once at construction.
- `button_order: DialogButtonOrder` — what `platform_button_order()` returned
  at construction time.
- `icon_theme: Option<Cow<'static, str>>` — runtime system fallback used by
  the pipeline's three-tier icon_theme precedence (per-variant → Theme-level
  → this fallback).

Key design choices (per docs/archive/v0.5.7_gaps.md §G7 and doc 2 §J.2):

- **No `impl Default`.** Runtime-detected types must signal intent at the
  call site. Use `from_system()` for production or `for_tests()` for
  deterministic test values (96 DPI, `PrimaryRight`, no `icon_theme`).
- **`&ResolutionContext` parameter, not `Option<&ResolutionContext>`.** The
  None-overload would reintroduce the silent-default anti-pattern. Explicit
  shortcut `resolve_system()` covers the OS-detected path.
- **`resolve_system()` placed on `ThemeMode`, not `Theme`** (deviation from
  gap doc §G7 step 4). Rationale: `Theme` has both light and dark variants;
  explicit variant selection via
  `theme.into_variant(mode)?.resolve_system()` is unambiguous.
- **`AccessibilityPreferences` stays on `SystemTheme`**, not on the
  context (per ACCESS-01 / J.2 B4 refinement). Accessibility is a
  render-time concern, not a resolve-time concern.

Internal change: `OverlaySource.font_dpi: Option<f32>` replaced by
`OverlaySource.context: ResolutionContext`. Consumers are not affected
(the type has always been `pub(crate)`).

Migration is mechanical across 43 call sites in 18 files. No deprecation
shim — v0.5.7 is the no-backcompat window.

### Added

#### native-theme-derive (new crate)

- `#[derive(ThemeWidget)]` proc macro replaces the old `define_widget_pair!` macro,
  generating paired Option/Resolved struct hierarchies, merge logic, validation,
  range checks, and field-level inheritance

#### native-theme (core)

- `IconLoader` builder struct for all icon loading operations
- `ColorMode` enum (`Light`, `Dark`) with `is_dark()` method
- `AccessibilityPreferences` struct on `SystemTheme` (text_scaling_factor, reduce_motion, high_contrast, reduce_transparency)
- `DiagnosticEntry` enum and `PlatformPreset` struct for diagnostic reporting
- `FrameList` newtype wrapping `Vec<IconData>` with non-empty guarantee
- `DetectionContext` struct with `ArcSwapOption` caches for is_dark, reduced_motion, icon_theme
- `prelude` module with 8 convenience re-exports (post-G7: `Theme`, `ResolvedTheme`, `SystemTheme`, `AccessibilityPreferences`, `ResolutionContext`, `Rgba`, `Error`, `Result`)
- `IconRole::name()` method
- `Rgba` named constants: `TRANSPARENT`, `BLACK`, `WHITE`
- `#[non_exhaustive]` on `LinuxDesktop` enum; new variants: `CosmicDe`, `Hyprland`, `Sway`, `River`, `Niri`
- `ThemeMode::resolve_platform_defaults()` for DE-aware `button_order` resolution
- `#[doc(hidden)]` on pipeline intermediates (`ThemeMode::resolve()`, `resolve_all()`, `validate()`, etc.)
- Uniform bare `#[must_use]` convention across the crate
- `inventory`-driven widget registration replacing hand-maintained `VARIANT_KEYS`/`widget_fields()`
- `IconSetChoice` enum, `default_icon_choice()` function, and `list_freedesktop_themes()` function for icon-set selection UIs that need a library-level "use the platform default" option and an enumeration of available freedesktop themes; all three are re-exported from the crate root
- `Theme::resolve(mode)` convenience method — one-shot `Theme`→`Resolved` resolution using a `ResolutionContext::from_system()` internally, preserving the three-tier `icon_theme` precedence (per-variant → `Theme`-level → system fallback) that the `into_variant(mode)?.resolve_system()?` two-step silently dropped at tier 2

#### native-theme-build

- Generated code paths updated for type renames and `Cow<'static, [u8]>` icon data

#### Connectors

- Both `native-theme-gpui` and `native-theme-iced` updated for all type renames, `ColorMode` API, `Arc<str>` font families, and `Cow` icon data

### Fixed

#### native-theme (core)

- Adwaita icon coverage: `IconRole::StatusBusy` now maps to `content-loading-symbolic`, and `IconRole::DialogSuccess` now maps to `object-select-symbolic`, matching freedesktop spec coverage under the Adwaita theme
- PNG-only icon themes (e.g. `AdwaitaLegacy`) now decode to RGBA at load time so they render instead of producing blank output
- `watch/kde` backend ignores non-mutation filesystem events, preventing a feedback loop that could fire repeated `ThemeChangeEvent::Changed` signals on every save

#### Platform readers

- Windows dialog `button_order` is now `PrimaryLeft` per Microsoft's Windows UX Guidelines (previously `PrimaryRight` by default)

### Removed

#### native-theme (core)

- `Rgba::to_f32_tuple` -- use `to_f32_array()` and destructure
- `IconSet::default()` -- use `system_icon_set()` for the platform-appropriate icon set
- `define_widget_pair!` macro -- replaced by `#[derive(ThemeWidget)]` proc macro
- `from_toml_with_base()` -- use `Theme::preset()` + `merge()` instead
- `ThemeChangeEvent::Other` variant
- `ENV_MUTEX` test infrastructure (`test_util.rs`)

## [0.5.6] - 2026-04-10

### Added

#### native-theme (core)
- **Runtime theme watcher** (`watch` feature): `ThemeWatcher` struct, `ThemeChangeEvent` enum, and `on_theme_change()` async entry point for receiving live OS theme changes
  - KDE backend: inotify watching on `~/.config/kdeglobals` with parent-directory watching and debounce
  - GNOME backend: D-Bus portal `SettingChanged` signal via `zbus::blocking`
  - macOS backend: `NSDistributedNotificationCenter` via CFRunLoop
  - Windows backend: COM STA `UISettings.ColorValuesChanged` event
- **GTK symbolic icon recoloring**: `fg_color: Option<[u8; 3]>` parameter added to `load_icon()`, `load_custom_icon()`, `load_icon_from_theme()`, and `load_system_icon_by_name()` for tinting monochrome SVGs
- GTK symbolic icon normalization: viewBox/dimension inference, `currentColor` fill injection, class attribute stripping
- `GnomePortalData` struct and `build_gnome_spec_pure()` pure function for testable GNOME reader
- `from_kde_content_pure()` pure function for testable KDE reader
- 7 KDE fixture `.ini` files for deterministic testing
- 10 inline tests for `build_gnome_spec_pure`
- KDE edge-case fixture tests
- `ValidateNested` trait and `validate_widget!()` codegen in `define_widget_pair!` macro

### Changed

#### native-theme (core)
- `lib.rs` module split: extracted `detect.rs`, `pipeline.rs`, `icons.rs` into standalone modules
- `validate.rs` refactored: per-widget range checks moved to generated `check_ranges()` methods, helpers extracted to `validate_helpers.rs`
- Widget validation now uses generated `validate_widget()` calls instead of hand-written extraction
- Showcase examples now include runtime theme watcher integration
- Documentation: color role count updated from 22 to 24, `watch` feature added to feature tables, stale API examples corrected

## [0.5.5] - 2026-04-09

### Breaking Changes

> **Migration required.** This release renames ~70 fields, replaces flat border fields with `BorderSpec` sub-structs, removes `ThemeSpacing`, and changes per-widget foreground fields to `font.color`. Custom theme files and code accessing theme fields must be updated.

#### Field renames (~70 fields)

Color fields gain `_color` suffix, border fields move to sub-structs, and per-widget foreground fields are replaced by `font.color`:

**Before (v0.5.4 TOML):**

```toml
[light.defaults]
accent = "#3584e4"
background = "#ffffff"
foreground = "#000000"
muted = "#929292"

[light.button]
foreground = "#000000"
border_color = "#c0c0c0"
corner_radius = 6.0
border_width = 1.0
```

**After (v0.5.5 TOML):**

```toml
[light.defaults]
accent_color = "#3584e4"
background_color = "#ffffff"
text_color = "#000000"
muted_color = "#929292"

[light.button]
font = { color = "#000000" }
border = { color = "#c0c0c0", corner_radius = 6.0, line_width = 1.0 }
```

Key renames by category:
- **Colors**: `accent` -> `accent_color`, `background` -> `background_color`, `foreground` -> `text_color`, `muted` -> `muted_color`, `selection` -> `selection_color`, `focus_ring_color` -> `focus_ring_color` (unchanged), `accent_foreground` -> `accent_text_color`, `selection_foreground` -> `selection_text_color`, `disabled_foreground` -> `disabled_text_color`, `danger` -> `danger_color`, `danger_foreground` -> `danger_text_color`
- **Borders**: flat `border_color`, `corner_radius`, `border_width` -> `border.color`, `border.corner_radius`, `border.line_width`
- **Per-widget foreground**: `button.foreground`, `menu.foreground`, etc. -> `button.font.color`, `menu.font.color`, etc.

#### ThemeSpacing removed

`ThemeSpacing` (xs/sm/md/lg/xl) is replaced by `LayoutTheme` with semantic field names.

**Before (v0.5.4 TOML):**

```toml
[light.defaults.spacing]
xs = 2.0
sm = 4.0
md = 8.0
lg = 16.0
xl = 24.0
```

**After (v0.5.5 TOML):**

```toml
[light.layout]
widget_gap = 6.0
container_margin = 6.0
window_margin = 10.0
section_gap = 18.0
```

#### BorderSpec sub-struct

Flat border fields on widgets are now nested under `[widget.border]` TOML tables.

**Before (v0.5.4 TOML):**

```toml
[light.input]
border_color = "#c0c0c0"
corner_radius = 6.0
border_width = 1.0
```

**After (v0.5.5 TOML):**

```toml
[light.input]

[light.input.border]
color = "#c0c0c0"
corner_radius = 6.0
line_width = 1.0
```

#### FontSpec expanded

`FontSpec` gains `style` (`FontStyle` enum) and `color` fields.

**New TOML structure:**

```toml
[light.defaults.font]
family = "Inter"
size = 13.0
weight = 400
style = "normal"
color = "#000000"
```

#### Per-widget foreground removed

Text color is now `widget.font.color` instead of `widget.foreground`.

**Before (v0.5.4):**

```toml
[light.button]
foreground = "#000000"
```

**After (v0.5.5):**

```toml
[light.button]
font = { color = "#000000" }
```

### Added

#### native-theme (core)
- `BorderSpec` / `ResolvedBorderSpec` sub-structs for typed border configuration (color, corner_radius, corner_radius_lg, line_width, opacity, shadow_enabled, padding_horizontal, padding_vertical)
- `FontStyle` enum (`Normal`, `Italic`, `Oblique`) with serde lowercase rename
- `LayoutTheme` / `ResolvedLayoutTheme` replacing `ThemeSpacing` (widget_gap, container_margin, window_margin, section_gap)
- ~70 interactive state color fields across 18 widgets (hover_background, hover_text_color, active_background, disabled_background, etc.)
- Property-based tests (proptest) for TOML round-trip serialization and merge semantics
- Platform-facts cross-reference tests for drift detection between documentation and preset data
- `detect_is_dark()` GTK_THEME env var and `gtk-3.0/settings.ini` fallback for non-GNOME/non-KDE Linux desktops
- iOS platform detection (`target_os = "ios"`) in `detect_platform()`
- Spinner safety guards: dimension validation (width/height > 0), empty frames guard, zero duration guard, single-quote viewBox attribute handling
- gsettings command timeout (2-second) to prevent indefinite blocking
- Iced connector WCAG contrast enforcement for status foreground colors

#### native-theme-gpui
- `from_system()` now returns `(Theme, ResolvedThemeVariant, bool)` 3-tuple (adds `is_dark` flag, matching iced connector)

### Changed

#### native-theme (core)
- Field naming convention aligned with `property-registry.toml` (~70 renames across all structs)
- Resolution engine overhauled: all safety-net invented values removed, proper inheritance per `inheritance-rules.toml`
- `resolve_border()` and `resolve_font()` functions implement sub-field inheritance for all widgets
- All 17 presets rewritten for new schema with explicit `text_scale` and interactive state color values
- `into_resolved()` `#[must_use]` message corrected to reflect consuming semantics

#### native-theme-gpui
- Color derivations (hover, active, disabled) replaced with direct theme field reads where presets now provide explicit values
- Display name in `from_preset()` uses `spec.name` for human-readable output (was using raw preset key)

#### native-theme-iced
- Display name in `from_preset()` already used `spec.name` (consistent with gpui fix)

#### CI
- `pre-release-check.sh` timeout added (30 min max)
- Publish workflow: gpui connector gate added, better error handling
- Async-io variants tested in CI
- Example names disambiguated (`showcase-gpui`, `showcase-iced`)

### Fixed

#### native-theme (core)
- `detect_is_dark()` now works on non-GNOME/non-KDE Linux desktops via GTK_THEME and gtk-3.0/settings.ini fallback (C-1)
- `detect_platform()` returns `"ios"` on iOS targets (C-2)
- `into_resolved()` `#[must_use]` message corrected (C-3)
- Inheritance bugs: `input.selection` used wrong source (INH-1), `dialog.background_color` missing per-platform fallback (INH-2), `card` border inheritance removed where inappropriate (INH-3)

#### CI
- Async-io feature variants tested to prevent compilation regressions
- Example binary names disambiguated to avoid Cargo build conflicts

### Migration Notes

**Migration checklist for v0.5.4 -> v0.5.5:**

1. **Rename color fields** in custom TOML theme files:

   | Before | After |
   |--------|-------|
   | `accent` | `accent_color` |
   | `background` | `background_color` |
   | `foreground` | `text_color` |
   | `muted` | `muted_color` |
   | `selection` | `selection_color` |
   | `accent_foreground` | `accent_text_color` |
   | `disabled_foreground` | `disabled_text_color` |
   | `danger` | `danger_color` |
   | `danger_foreground` | `danger_text_color` |

2. **Update `[spacing]` sections** to `[layout]` with new field names:
   - `xs`/`sm`/`md`/`lg`/`xl` -> `widget_gap`/`container_margin`/`window_margin`/`section_gap`

3. **Move flat border fields** to `[widget.border]` sub-tables:
   - `border_color` -> `border.color`
   - `corner_radius` -> `border.corner_radius`
   - `border_width` -> `border.line_width`

4. **Replace `widget.foreground`** with `widget.font.color` in TOML and Rust code:
   - `button.foreground` -> `button.font.color`
   - `menu.foreground` -> `menu.font.color`
   - (applies to all widgets with text)

5. **Update Rust code** accessing resolved theme structs:
   - `resolved.defaults.accent` -> `resolved.defaults.accent_color`
   - `resolved.defaults.background` -> `resolved.defaults.background_color`
   - `resolved.button.foreground` -> `resolved.button.font.color`

## [0.5.4] - 2026-04-04

### Added

#### native-theme (core)
- `xdg_current_desktop()` helper for consistent `XDG_CURRENT_DESKTOP` parsing
- `Display` implementations for `IconRole` and `IconSet`
- Resolve safety nets: `line_height`, `button_order`, `accent_foreground`, `shadow`, `disabled_foreground`, `spinner.fill` falls back to `accent`, `text_scale` size ratios and weight defaults
- Validate hardening: NaN/Infinity rejection for geometry fields, `dialog` min/max cross-field validation
- `unpremultiply_alpha()` deduplication across platform readers
- 45+ new tests

#### native-theme-build
- `emit_cargo_directives()` returns `Result` instead of calling `process::exit()`
- Builder validation deferred to `generate()` (no `assert!` panics in `crate_path()`/`derives()`)
- Path traversal rejection for TOML source paths
- `crate_path` and `derives` Rust path validation
- Invisible Unicode rejection in role names and paths
- Bundled DE-aware mapping entries now produce `BuildError`
- Post-merge theme overlap validation
- Theme directory existence check
- Name normalization warnings for non-kebab-case identifiers
- 39 new tests, 3 compiled doctests

#### native-theme-gpui
- All 96 `ThemeColor` fields populated in `ThemeConfig` (prevents `apply_config` reset)
- 20+ new helper functions: accessibility queries, typography helpers, layout utilities, spacing calculators
- Animation frames: all-or-nothing semantics (returns `None` if any frame fails)
- SVG colorization: stroke pattern support
- 37 new tests (132 total)

#### native-theme-iced
- Extended palette: all 4 status families overridden (was 2 of 5)
- `apply_overrides` restructured (was dead code)
- `from_preset()`: proper display name, accurate error messages
- `from_system()`: returns OS `is_dark` flag
- SVG colorization: stroke pattern support
- 11 new helper functions
- 31 new tests (88 total)

### Changed

#### native-theme (core)
- Preset corrections:
  - Windows 11: 16 geometry fixes + 3 color fixes
  - Adwaita: 10 geometry fixes + dialog radius + text_scale corrections
  - KDE Breeze: 6 geometry fixes
  - macOS Sonoma: 4 geometry fixes + `button_order` corrected
  - iOS: `button_order` corrected
  - Community presets: `button_order` removed where inappropriate, Solarized border colors fixed, `radius_lg` fixed

#### native-theme-gpui
- Color mapping: `muted_fg` semantic fix, `_light` mode-aware derivation, `list_active` double-opacity fix, scrollbar track derived from resolved theme, WCAG contrast enforcement for status foregrounds, chart color saturation floor, `active_color` near-black fix, overlay respects `reduce_transparency`

#### native-theme-iced
- `from_preset()` uses correct display name and error messages
- `from_system()` propagates OS `is_dark` flag

### Fixed

#### native-theme-gpui
- `list_active` double-opacity rendering
- `active_color` near-black derivation
- Chart colors losing saturation at low lightness

#### native-theme-iced
- Extended palette: missing status family overrides caused iced defaults to leak through
- `apply_overrides` dead code path that prevented custom palette entries from taking effect

## [0.5.3] - 2026-04-01

### Added

- `ThemeVariant::resolve_platform_defaults()` — separated platform-dependent resolution (icon theme detection) from the pure data-transform `resolve()`
- `ThemeVariant::resolve_all()` — convenience for `resolve()` + `resolve_platform_defaults()`
- `ThemeSpec::from_toml_with_base()` — merge custom TOML overrides onto a named base preset in one call
- `ThemeSpec::lint_toml()` — detect unrecognized field names in TOML theme files (opt-in linting for theme authors)
- `detect_is_dark()`, `detect_reduced_motion()`, `detect_icon_theme()` — uncached polling variants of `system_is_dark()`, `prefers_reduced_motion()`, `system_icon_theme()`
- `diagnose_platform_support()` — human-readable diagnostic messages for OS theme detection availability
- `FIELD_NAMES` const on all 25 per-widget `Option` structs and `ThemeDefaults` (used by `lint_toml()`)
- Range validation in `validate()` for ~50 geometry, opacity, font-size, and font-weight fields
- `switch.unchecked_background` resolve rule (falls back to `muted`)
- gpui connector: `bundled_icon_to_image_source()` — single-call `IconName` + `IconSet` to `ImageSource` conversion
- gpui connector: re-exports `Error`, `TransformAnimation`, `LinuxDesktop` (Linux-only)
- iced connector: `to_iced_weight()` — CSS font weight (100-900) to iced `Weight` enum
- iced connector: `into_image_handle()`, `into_svg_handle()` — consuming variants of the borrow-based helpers
- iced connector: re-exports `Error`, `Result`, `Rgba`, `TransformAnimation`
- Build crate: drift detection tests for `THEME_TABLE` vs `IconSet` and `DE_TABLE` vs `LinuxDesktop`
- Build crate: digit-starting identifier validation (rejects names that produce invalid Rust identifiers)
- Build crate: empty-roles and no-themes warnings for likely misconfiguration

### Changed

- `ButtonTheme`: `primary_bg` → `primary_background`, `primary_fg` → `primary_foreground`
- `SwitchTheme`: `checked_bg` → `checked_background`, `unchecked_bg` → `unchecked_background`, `thumb_bg` → `thumb_background`
- `into_resolved()` now calls `resolve_all()` (includes platform defaults) for backward compatibility
- Preset registry refactored from 20 constants + 20 `LazyLock` + 20-arm match to data-driven `HashMap`
- Community presets no longer hardcode `icon_set = "freedesktop"`
- gpui `from_preset()` and `from_system()` return `(Theme, ResolvedThemeVariant)` tuple
- gpui `to_theme()` sets all `Theme` fields directly — eliminates `apply_config` workaround and radius truncation
- gpui `animated_frames_to_image_sources()` accepts `color` and `size` parameters
- gpui `into_image_source()` delegates to `to_image_source()` instead of duplicating logic
- iced `to_theme()` captures 4 `Copy` Rgba values instead of cloning entire `ResolvedThemeVariant`
- iced `from_preset()` uses `into_variant()` (avoids clone); `from_system()` moves variant
- iced `line_height()` renamed to `line_height_multiplier()` (returns raw multiplier, not pixels)
- iced `animated_frames_to_svg_handles()` accepts `color` parameter
- Build crate: `GenerateOutput::emit_cargo_directives()` returns `()` instead of `io::Result<()>` — handles errors internally
- Build crate: `IconGenerator::crate_path()` and `derive()` validate input (assert on empty/whitespace)
- Build crate: role deduplication in `merge_configs()` via `BTreeSet`

### Removed

- gpui connector: `pick_variant()` free function (use `ThemeSpec::into_variant()`)

### Fixed

- gpui BMP encoder validates dimensions and detects overflow (returns `Option` instead of panicking)
- gpui `colorize_svg()` returns original bytes on non-UTF-8 input (prevents data corruption)
- iced `colorize_monochrome_svg()` returns original bytes on non-UTF-8 input
- Build crate: Windows path separators in generated `include_bytes!` paths (backslash → forward slash)
- Build crate: empty config early return in pipeline (prevents index panic)
- Build crate: orphan SVG check emits warning instead of silently ignoring unreadable directories

## [0.5.2] - 2026-03-31

### Added

- `Deserialize` derive on all `Resolved*` types (enables caching, IPC, test fixtures)
- `Serialize` and `Deserialize` on `IconData`, `AnimatedIcon`, and `TransformAnimation`
- `Copy`, `Eq`, `Hash` derives on `DialogButtonOrder`; `Eq`, `Hash` on `LinuxDesktop`
- `#[must_use]` on 20+ public functions across all four crates
- `#[non_exhaustive]` on `BuildError` enum
- `Debug` and `Clone` derives on `IconGenerator`, `GenerateOutput`, `AnimatedImageSources`, `AnimatedSvgHandles`
- `into_image_source()` consuming variant in gpui connector
- KDE reader: `accent_foreground`, `list.background`/`foreground` from live color scheme
- GNOME reader: portal `reduce-motion` and gsettings `high-contrast` detection

### Changed

- `Error::Unsupported` now carries a `&'static str` context payload
- `icon_set` field on `ThemeVariant` changed from `Option<String>` to `Option<IconSet>` (validated at parse time)
- `rasterize_svg()` uses `Error::Format` instead of `Error::Unavailable` for invalid dimensions
- `BuildErrors` inner field is now private; access via `errors()`, `into_errors()`, `len()`, `is_empty()`, and `IntoIterator`
- gpui icon mapping functions (`lucide_name_for_gpui_icon`, `material_name_for_gpui_icon`, `freedesktop_name_for_gpui_icon`) return `&'static str` instead of `Option<&'static str>`
- iced `from_preset()` and `from_system()` return `(Theme, ResolvedThemeVariant)` tuple

### Fixed

- KDE Breeze preset: `radius` 4 -> 5, `focus_ring_width`/`focus_ring_offset` swapped, `line_height` 1.4 -> 1.36, four incorrect `icon_sizes`, `progress_bar.min_width` mismap, `spinner.diameter`, `expander.arrow_size`, `switch` dimensions
- KDE reader: `defaults.border` no longer overwritten with accent color; `forceFontDPI` read from correct file
- Adwaita preset: `radius` 12 -> 9, `radius_lg` 14 -> 15, `line_height` 1.4 -> 1.21, `focus_ring_offset` 1 -> -2, `section_heading` weight 400 -> 700
- macOS Sonoma preset: corrected geometry and metric values across both full and live presets
- Windows 11 preset: corrected geometry and metric values across both full and live presets
- gpui connector: `colorize_svg()` now handles self-closing SVG tags correctly
- Build crate: simplified error handling pipeline, improved codegen

## [0.5.1] - 2026-03-30

### Changed

- Renamed types and tightened visibility across core and build crates
- Build crate Result-based API for validation diagnostics
- Simplified GNOME/KDE readers and polished connector APIs
- Expanded widget resolved types and cleaned up build crate tests

### Fixed

- Windows compilation — swapped `icon_name` args, Rust 2024 unsafe blocks
- macOS compilation errors
- Test compilation — stale call sites after API changes
- iced screenshot delay to avoid blank capture on Windows
- CI: removed tag trigger from docs workflow

## [0.5.0] - 2026-03-28

### Added

- Per-widget data model: 25 `XxxTheme` / `ResolvedXxx` struct pairs (Window, Button, Input, Checkbox, Menu, Tooltip, Scrollbar, Slider, ProgressBar, Tab, Sidebar, Toolbar, StatusBar, List, Popover, Splitter, Separator, Switch, Dialog, Spinner, ComboBox, SegmentedControl, Card, Expander, Link)
- `ThemeDefaults` struct with ~40 global properties (colors, fonts, spacing, icon sizes, accessibility)
- `FontSpec` for per-widget font specification (family, size, weight)
- `TextScale` with 4 typographic roles (caption, section_heading, dialog_title, display)
- `IconSizes` struct (toolbar, small, large, dialog, panel)
- `DialogButtonOrder` enum (TrailingAffirmative / LeadingAffirmative)
- `ThemeSpacing` struct (xxs through xxl)
- `define_widget_pair!` macro generating paired Option/Resolved structs from a single definition
- `ResolvedThemeVariant` type where all fields are guaranteed populated (non-optional)
- `ResolvedThemeDefaults`, `ResolvedFontSpec`, `ResolvedThemeSpacing`, `ResolvedIconSizes`, `ResolvedTextScale`, `ResolvedTextScaleEntry` types
- `ThemeResolutionError` listing missing field paths; `Error::Resolution` variant
- `ThemeVariant::resolve()` with ~90 inheritance rules in 4 phases (defaults-internal, safety-nets, widget-from-defaults, widget-to-widget)
- `ThemeVariant::validate()` producing `ResolvedThemeVariant` or listing all missing fields
- `SystemTheme` type returned by `from_system()` with `active()`, `pick()`, `with_overlay()`, `with_overlay_toml()`
- Live platform presets (geometry-only, internal): `kde-breeze-live`, `adwaita-live`, `macos-sonoma-live`, `windows-11-live`
- `platform_preset_name()` mapping the current OS to its live preset
- `list_presets_for_platform()` filtering presets by current OS
- `system_is_dark()` cross-platform cached dark-mode detection (Linux gsettings/kdeglobals, macOS AppleInterfaceStyle, Windows UISettings)
- KDE reader: per-widget fonts (menuFont, toolBarFont), WM title bar colors, text scale via Kirigami multipliers, icon sizes from index.theme, accessibility (AnimationDurationFactor, forceFontDPI)
- GNOME reader: gsettings fonts (font-name, monospace-font-name, titlebar-font), text scale via CSS percentages, accessibility (text-scaling-factor, enable-animations, overlay-scrolling), icon-theme
- macOS reader: per-widget fonts (+menuFontOfSize:, +toolTipsFontOfSize:, +titleBarFontOfSize:), NSFont.TextStyle text scale, additional NSColor values, scrollbar overlay mode, accessibility (reduce_motion, high_contrast, reduce_transparency, text_scaling_factor)
- Windows reader: NONCLIENTMETRICSW per-widget fonts, DwmGetColorizationColor title bar, GetSysColor widget colors, text scale factor, high contrast, icon sizes via GetSystemMetrics

### Changed

- `ThemeVariant` composes `ThemeDefaults` + 25 per-widget structs instead of flat `ThemeColors`/`ThemeFonts`/`ThemeGeometry`
- `from_system()` and `from_system_async()` return `SystemTheme` instead of `ThemeSpec`
- gpui and iced connector `to_theme()` accept `&ResolvedThemeVariant` instead of `&ThemeVariant`
- All 16 preset TOMLs rewritten for per-widget structure; platform presets slimmed to design constants only
- `impl_merge!` macro extended with `optional_nested` category for per-widget font fields
- Both gpui and iced showcase examples updated for `SystemTheme` / `ResolvedThemeVariant` API

### Removed

- `ThemeColors` flat struct (replaced by `ThemeDefaults` base colors + per-widget color fields)
- `ThemeFonts` struct (replaced by `FontSpec` on `ThemeDefaults` + per-widget font fields)
- `ThemeGeometry` struct (replaced by per-widget geometry fields)
- `WidgetMetrics` and its 12 sub-structs (replaced by per-widget sizing fields on each `XxxTheme`)
- `default` preset (replaced by platform detection via `platform_preset_name()` and live presets)

### Migration from v0.4.x

**Data model:** `variant.colors.accent` -> `variant.defaults.accent`, `variant.fonts.family` -> `variant.defaults.font.family`, `variant.geometry.radius` -> `variant.defaults.radius`. Per-widget fields like `variant.button.min_height` replace `variant.widget_metrics.button.min_height`.

**from_system():**

```rust,ignore
// Before (v0.4.x)
let nt: ThemeSpec = from_system().unwrap_or_else(|_| ThemeSpec::preset("adwaita").unwrap());
let variant = nt.pick_variant(true).unwrap();

// After (v0.5.0)
let system: SystemTheme = from_system().unwrap();
let resolved: &ResolvedThemeVariant = system.active(); // all fields guaranteed
```

**Connectors:**

```rust,ignore
// Before (v0.4.x)
let theme = to_theme(variant, "My App", is_dark);

// After (v0.5.0)
let mut v = variant.clone();
v.resolve();
let resolved = v.validate().unwrap();
let theme = to_theme(&resolved, "My App", is_dark);

// Or from SystemTheme (already resolved):
let theme = to_theme(system.active(), "My App", system.is_dark);
```

## [0.4.1] - 2026-03-20

### Added

- `CONTRIBUTING.md` with development workflow and testing guide
- `CODE_OF_CONDUCT.md` (Contributor Covenant 2.1)
- `SECURITY.md` with responsible disclosure policy
- GitHub issue templates (bug report, feature request) using YAML forms
- Pull request template with CI checklist
- Animated icon sections in gpui and iced connector READMEs
- Animated icon showcase demonstrations in both gpui and iced examples
- CLI argument support (`--tab`, `--preset`) for showcase examples
- GIF generation script for bundled spinner animations
- Screenshot automation (`--screenshot` flag) for iced showcase example
- CI workflow for automated screenshot generation on Linux, macOS, and Windows
- Showcase screenshots embedded in root, iced, and gpui READMEs
- Spinner GIFs embedded in root README
- `#![warn(missing_docs)]` crate-level lint attribute in all workspace crates
- Doc comments for all public API items in native-theme core crate

### Changed

- Root README updated with animated icons section
- Version references updated from 0.3.x to 0.4.x across all documentation

### Fixed

- Broken intra-doc link for `iced::time::every()` in native-theme-iced
- Missing documentation warnings that caused CI failures under `-Dwarnings`
- Formatting violations in gpui showcase example

## [0.4.0] - 2026-03-18

### Added

- `AnimatedIcon` enum with `Frames` and `Transform` variants for animated icon data
- `TransformAnimation` enum with `Spin` variant for continuous rotation
- `Repeat` enum controlling animation looping behavior
- `AnimatedIcon::first_frame()` method returning a static fallback frame
- `loading_indicator(icon_set)` function dispatching to platform-appropriate spinner animations
- `prefers_reduced_motion()` function querying OS accessibility settings (Linux gsettings, macOS NSWorkspace, Windows UISettings)
- Bundled Lucide loader spinner (spin transform) and freedesktop `process-working` sprite sheet loading
- Freedesktop sprite sheet parser for runtime `process-working.svg` animation loading
- gpui connector: `animated_frames_to_image_sources()` and `with_spin_animation()` for animation playback
- iced connector: `animated_frames_to_svg_handles()` and `spin_rotation_radians()` for animation playback

### Changed

- `IconRole::StatusLoading` renamed to `IconRole::StatusBusy` (static icon for busy state)

### Removed

- `IconRole::StatusLoading` variant (use `loading_indicator()` for animated loading indicators, or `IconRole::StatusBusy` for a static busy icon)

### Migration from v0.3.x

**Before (v0.3.x):**

```rust,ignore
use native_theme::{load_icon, IconRole};

// Static loading icon
let icon = load_icon(IconRole::StatusLoading, "material");
```

**After (v0.4.0):**

```rust,ignore
use native_theme::{loading_indicator, prefers_reduced_motion, AnimatedIcon};

// Animated loading indicator with platform-native style
if let Some(anim) = loading_indicator("material") {
    // Check accessibility preference first
    if prefers_reduced_motion() {
        let static_icon = anim.first_frame();
        // Render a single static frame
    } else {
        match &anim {
            AnimatedIcon::Frames(data) => {
                // Cycle through data.frames() on a timer (data.frame_duration_ms().get() ms)
            }
            AnimatedIcon::Transform(data) => {
                // Apply continuous rotation to data.icon() via data.animation()
            }
            _ => {}
        }
    }
}

// If you just need a static busy icon (not animated):
use native_theme::{load_icon, IconRole};
let busy = load_icon(IconRole::StatusBusy, "material");
```

## [0.3.3] - 2026-03-17

### Added

- `IconProvider` trait for defining custom icon types that integrate with native-theme's loading system
- `load_custom_icon()` function dispatching custom icons through the same platform loader chain as built-in icons
- `load_system_icon_by_name()` function for loading platform icons by arbitrary name string
- `native-theme-build` crate: TOML-driven code generation for custom icon roles with `generate_icons()` and `IconGenerator` builder API
- DE-aware code generation: freedesktop mapping TOML entries can specify per-desktop-environment icon names (e.g., `{ kde = "view-visible", default = "view-reveal" }`)
- gpui connector: `custom_icon_to_image_source()` and `custom_icon_to_image_source_colored()` for loading custom icons
- iced connector: `custom_icon_to_image_handle()`, `custom_icon_to_svg_handle()`, and `custom_icon_to_svg_handle_colored()` for loading custom icons
- Icon mapping gap fills: Freedesktop `Notification` -> "notification-active", Material/Lucide `TrashFull` mappings
- Coverage tests: `no_unexpected_icon_gaps` and `all_roles_have_bundled_svg` prevent future mapping regressions

### Changed

- `IconRole` now implements `IconProvider`, delegating to built-in mapping functions
- Platform icon loaders (freedesktop, SF Symbols, Segoe Fluent) return `None` for unmapped roles instead of falling back to Material SVGs

### Removed

- Wildcard Material SVG fallback from `load_icon()` and all platform loaders (icons not found in the requested set now return `None`)

## [0.3.2] - 2026-03-14

### Added

- `ThemeSpec::pick_variant()` method for selecting the appropriate theme variant with cross-fallback
- `#[must_use]` annotations on all public API functions and key types (`ThemeSpec`, `IconData`)

### Changed

- `system_icon_theme()` and `system_is_dark()` now cache results with `OnceLock` (eliminates redundant subprocess spawns)
- `colorize_svg` renamed to `colorize_monochrome_svg` in iced connector with documentation clarifying monochrome-only contract
- Improved `to_theme` comment in gpui connector explaining the `apply_config`/restore pattern
- `pre-release-check.sh` uses `jq` instead of `python3` for JSON parsing (with bash fallback)

### Deprecated

- `pick_variant()` free functions in gpui and iced connectors (use `ThemeSpec::pick_variant()` instead)

### Removed

- Dead `lighten`, `darken`, and `with_alpha` wrapper functions from gpui `derive` module

## [0.3.1] - 2026-03-13

### Added

- Meta-features (`linux-full`, `macos-full`, `windows-full`) for simplified feature gate configuration
- `system_icon_theme()` with DE-aware detection (KDE, GNOME, Xfce, Cinnamon, Mate, LxQt, Budgie)
- `bundled_icon_by_name()` for string-based icon lookup
- `load_freedesktop_icon_by_name()` for arbitrary freedesktop icon lookups
- `LinuxDesktop` enum expanded with Xfce, Cinnamon, Mate, LxQt, Budgie variants
- `LinuxDesktop` and `detect_linux_de()` made public
- Freedesktop icon name mapping for all 86 gpui-component icons
- SVG colorization support in iced connector (`to_svg_handle_colored`)

### Changed

- Target-gated OS dependencies so meta-features compile on all platforms
- Renamed `icon_theme` field to `icon_set` (with serde alias for backward compatibility)
- Updated bundled Material and Lucide SVGs to latest releases (86+ icons each)

### Fixed

- BMP rasterization in gpui connector (red/blue channel swap for colored SVG themes)
- Plasma 6 icon theme detection via `kdedefaults/kdeglobals` fallback
- Symbolic icon preference to avoid animation sprite sheets from freedesktop themes

## [0.3.0] - 2026-03-09

### Added

- Icon system: `IconRole` enum (42 semantic icon roles), `IconSet` enum, `IconData` type
- Bundled SVG icon sets: Material Design and Lucide (86+ icons each, ~300KB total)
- Linux freedesktop icon loading via `freedesktop-icons` crate
- macOS SF Symbols icon loading (compile-time stub with bundled fallback)
- Windows Segoe Fluent Icons loading (compile-time stub with bundled fallback)
- `load_icon()` cross-platform dispatch function
- `rasterize_svg()` for SVG-to-bitmap conversion via `resvg`
- gpui connector: `icon_name()` mapping, `to_image_source()` conversion
- iced connector: `to_svg_handle()` for SVG icon display

## [0.2.0] - 2026-03-09

### Added

- macOS reader (`from_macos()`) with light and dark variant detection
- `WidgetMetrics` with 12 per-widget sub-structs (Button, Checkbox, Input, ListItem, MenuItem, ProgressBar, Scrollbar, Slider, Splitter, Tab, Toolbar, Tooltip)
- `ThemeGeometry::radius_lg` and `shadow` fields for extended geometry support
- Linux D-Bus portal backend detection for improved desktop environment heuristics
- Portal overlay for KDE themes (`from_kde_with_portal()`)
- `native-theme-iced` connector crate for iced toolkit integration
- `native-theme-gpui` connector crate for gpui toolkit integration
- GitHub Actions CI pipeline with cross-platform matrix (Linux, macOS, Windows)
- Windows accent shade colors (AccentDark1-3, AccentLight1-3)
- Windows system font and DPI-aware geometry reading
- GNOME font data population via portal reader
- Async `from_system_async()` with D-Bus portal backend detection

### Changed

- Restructured as Cargo workspace with `native-theme`, `native-theme-iced`, and `native-theme-gpui` crates
- Flattened `ThemeColors` from nested sub-structs to 36 direct `Option<Rgba>` fields
- Moved preset API from free functions to `ThemeSpec` associated methods (`preset()`, `from_toml()`, `from_file()`, `list_presets()`, `to_toml()`)
- Renamed primary/secondary color fields with prefix (`primary_background`, `primary_foreground`, `secondary_background`, `secondary_foreground`)

### Removed

- `CoreColors`, `ActionColors`, `StatusColors`, `InteractiveColors`, `PanelColors`, `ComponentColors` nested sub-structs (replaced by flat `ThemeColors`)
- Free-standing `preset()`, `from_toml()`, `from_file()`, `list_presets()`, `to_toml()` functions (now methods on `ThemeSpec`)

## [0.1.0] - 2026-03-07

### Added

- `ThemeSpec` data model with 22 semantic color roles, fonts, geometry, and spacing
- `Rgba` color type with hex string parsing and serialization
- `ThemeVariant` composing colors, fonts, geometry, and spacing
- TOML serialization and deserialization for all theme types
- 17 bundled presets (platform and community themes)
- KDE reader (`from_kde()`) parsing kdeglobals color scheme
- GNOME portal reader (`from_gnome()`) via D-Bus Settings portal
- Windows reader (`from_windows()`) using Windows registry
- Cross-platform `from_system()` dispatch with automatic desktop detection
- `impl_merge!` macro for recursive Option-based theme merging
- Deep merge support across all theme types

[0.5.8]: https://github.com/tiborgats/native-theme/compare/v0.5.7...v0.5.8
[0.5.7]: https://github.com/tiborgats/native-theme/compare/v0.5.6...v0.5.7
[0.5.6]: https://github.com/tiborgats/native-theme/compare/v0.5.5...v0.5.6
[0.5.5]: https://github.com/tiborgats/native-theme/compare/v0.5.4...v0.5.5
[0.5.4]: https://github.com/tiborgats/native-theme/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/tiborgats/native-theme/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/tiborgats/native-theme/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/tiborgats/native-theme/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/tiborgats/native-theme/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/tiborgats/native-theme/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/tiborgats/native-theme/compare/v0.3.3...v0.4.0
[0.3.3]: https://github.com/tiborgats/native-theme/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/tiborgats/native-theme/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/tiborgats/native-theme/compare/v0.3...v0.3.1
[0.3.0]: https://github.com/tiborgats/native-theme/compare/v0.2.0...v0.3
[0.2.0]: https://github.com/tiborgats/native-theme/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/tiborgats/native-theme/releases/tag/v0.1.0
