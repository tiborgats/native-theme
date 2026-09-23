# v0.5.9 pre-merge fixes

Status: approved by the maintainer 2026-09-24 ("Fix all the issues"). Source:
the six-area review of `v0.5.9-gpui-kit-0.6.6` against `main` (merge base
`399f45a`), every finding verified by the controller before it went in here.

Rules for every task: never invent a value (a size needs a source, an unstated
one stays `None`); no panics, no `unsafe`; no hardcoded theme values; never mix
icon sets (a missing icon is none, never another set's); a new or changed test
fails first, and the report shows the failure; stage by name, no attribution
lines; `docs/todo.md` is appended to or has its named lines corrected, nothing
else; run `env CARGO_BUILD_JOBS=4 ./pre-release-check.sh` before the last
commit of a task (expected warnings: stale visual assets, stale compat claims,
uncommitted asset sources).

## Decisions

D1. **Page samples follow the chosen icon theme.** An icon the application
draws is part of its native look, whether it sits in the chrome or on a page.
Where the chosen theme lacks the icon, the sample draws none and its info says
so. gpui-component loads its own icons as SVG paths from the application's
asset source (`icon.rs:30-33`, `:189`), so the widgets' internal icons (a
Select's caret, a Checkbox's check, a Dialog's close) can follow too, if
gpui's SVG cache lets a changed source show; Task 6 settles that with a spike
and either does it or records the measured reason it cannot.

D2. **A combobox's padding is measured to its text; the arrow column is
`arrow_area_width`.** WinUI's template: `ComboBoxPadding` 12,5,0,7, the text in
column 0, a 38px arrow column, the 12px chevron right-aligned in it with a
14px margin (`ComboBox_themeresources.xaml:341, :566-567, :582`, microsoft-ui-xaml
`2b8c775`). Breeze: the edit field is inset by `PM_ComboBoxFrameWidth` on the
left, top and bottom but not the right, and ends at a 20px arrow column
(`comboBoxSubControlRect`, breezestyle.cpp `f0b1d75`, `SC_ComboBoxEditField`,
`SC_ComboBoxArrow`). So both state right = 0, and platform-facts says why.
GNOME's dropdown draws its arrow inline (platform-facts §2.24 `arrow_area_width`:
"(none) — inline icon"), so adwaita's stated 28 is removed, as is the
colour-scheme presets' 28 (no source). A connector whose trigger draws its
caret inside the padded row (gpui's Select and Combobox, iced's pick list and
combo box) has no arrow column: where `arrow_area_width` is stated, it leaves
the right side to the toolkit instead of applying a 0 measured to a column it
does not have.

D3. **The documented-sizes gate covers `row_height` for menu and list.** Values
per platform-facts §2.6 (menu) and §2.15 (list): macOS 22 / 24; Windows menu
23, the mouse (Narrow) context the menu padding already uses, and list 40;
KDE none ("sizes to font" / "sizes to content"); GNOME menu 32, list none (the
plain list, the context the list padding already uses). kde-breeze's 28/28,
adwaita's list 34 and windows-11's menu 36 go.

D4. **Colour-scheme presets and ios state no size the gate covers.** None of
their paddings, row heights, toolbar heights or arrow widths cites a source,
and a colour scheme has no platform to cite. They are removed, as `bar_height`
already was for the colour schemes; the toolkit's own sizes stand.

D5. **macOS reports no text-scaling factor.** platform-facts §2.19 (`:1123`):
macOS's accessibility text size affects only a few Apple apps and
`preferredFont(forTextStyle:)` still returns fixed sizes. The reader's
`systemFontSize / 13` invents a factor, and the connectors would apply it on
top of a font size that already carries it.

D6. **Select and Combobox take the stated height at text scale ≤ 1**, as
Button and Input do, where the content fits the stated height; a seams check
decides, per native preset. Where the content does not fit, `min_h` stays and
the doc states the measured reason.

D7. **Shortcuts use gpui's `secondary-` modifier** (`keystroke.rs:143`): Cmd on
macOS, Ctrl elsewhere, as gpui-component's own bindings do.

D8. **iced_aw's Card draws its close button with the default class's style**
(iced_aw 0.14.1 `widget/card.rs:193-204`), so `card::Style::close_color` never
reaches it. The connector documents it as unreachable; the showcase dismisses
its card with its own themed button instead of `on_close`; `docs/todo.md`
records the upstream issue for the maintainer to file.

D9. **`system-icons` compiles on every platform.** It also enables the platform
dependencies its icon code imports (`objc2`, `objc2-foundation`,
`objc2-app-kit` on macOS, `windows` on Windows); they are target-specific, so
each is enabled only on its platform. CI and the local gate check the feature
alone on macOS and Windows.

## Tasks

### Task 1: `system-icons` builds on macOS and Windows (D9)
Gate: `cargo check --target x86_64-pc-windows-gnu -p native-theme
--no-default-features --features system-icons` and the same for
`x86_64-apple-darwin` fail before and pass after (the macOS check of
`native-theme` alone works here; gpui cannot be built for macOS on Linux).
Also `cargo check --target x86_64-pc-windows-gnu -p native-theme-iced` with
default features, and `-p native-theme-gpui` with default features. Add the
feature-alone check to CI's Windows and macOS legs of `ci.yml`, and a
cross-target `cargo check` of it to `pre-release-check.sh` where the target is
installed (soft-skip otherwise, saying so). CHANGELOG Fixed entry.

### Task 2: core sizes and docs (D2, D3, D4, D5)
- Windows and KDE combobox right = 0 with D2's citations; platform-facts §2.24
  KDE `border.padding_horizontal` cell rewritten with the Breeze source;
  adwaita's and the colour schemes' `arrow_area_width` removed.
- `row_height` in the gate (D3), presets corrected, full and `-live` alike.
  If `row_height` resolves to a required `f32`, make it optional like
  `bar_height` (soft option) and say so in CHANGELOG Breaking.
- D4 removals; `ios` too. CHANGELOG entry.
- D5 in `macos.rs`, with its doc; CHANGELOG entry.
- `documented_sizes.rs:5-9` module doc (the Windows right side is now stated).
- `model/mod.rs:543-564` `from_toml` example: the keys the parser reads.
- `ResolvedWidgetBorder::padding`: `#[serde(default)]`, as `defined_size` has.
- Stale `tests/proptest_roundtrip.proptest-regressions` seeds removed.
Gate: the gate tests fail on the new rows before the presets change.

### Task 3: gpui connector library (D2, D6, docs)
- D2 connector rule in `geometry::select` / `geometry::combobox`, documented,
  seams-tested (kde-breeze and windows-11: right side is upstream's).
- D6 with a seams check per native preset.
- Docs: README `:33-34` (overlay transparent), `:84` gpui-kit 0.6.6, `:393`
  gpui-pre 0.3.6, `:38-40` icon audit also holds for 0.6.6, `:405-406`
  table_foot; `lib.rs:55,59` coverage table (sidebar 4 of 6, defaults row as
  read, menu and segmented_control rows); every gpui-pre citation re-pointed
  to 0.3.6 (`lib.rs:800, 907-908, 911`, `icons.rs:880-881, 1190, 1198-1199,
  1860`, `tests/seams.rs:263`), `icons.rs:1190`'s `:2906`; `lib.rs:881`
  → `theme/mod.rs:283-284`; the multi-line Input advice (`geometry.rs:217-218`,
  README `:186`) says `Styled::h(input, h)`, because `Input::h` shadows it and
  is applied before the refinement (`input/input.rs:257-259, :708, :719`);
  `geometry.rs:72-76` lists what `tests/seams.rs` checks; `0.6.4` labels that
  mean "the floor" → 0.6.6 where the claim was re-read; `variants.rs:43-46`;
  `proposals/README.md:59-60` keys `_px` and `:24` repository URL.
- `compat.rs`: also check the Quick start's gpui-kit version.

### Task 4: iced connector (D2, D8, docs)
- D8: `styles::aw::card` doc, contract row/pair for `close_color` →
  unreachable with the citation; showcase card without `on_close`.
- D2 connector rule for the pick list / combo box, where the connector pads it.
- Docs: `lib.rs:66-70` (`button_padding`/`input_padding` need `widgets`),
  `lib.rs:164-167` (Tooltip has no Catalog), `lib.rs:290` `button.rs:462`,
  `contract/rows.rs:1457, :1469` ranges, `platform-facts.md:969` → `:980`
  (`styles.rs:500, :595`, showcase `:2280`), `UNREACHABLE` completed from the
  styles docs (`progress_bar.min_width`, `spinner.stroke_width`/`min_diameter`,
  `list.alternate_row_background`/`header_*`/`grid_color`, sidebar and list
  corner radii) or its doc narrowed to what it lists.
- Showcase: the Widget Info "not themeable: padding" entries (`:2002, :2169,
  :2906`) say the theme's padding is applied; under a preset the showcase does
  not show an accessibility-scaled size it does not draw; buttons styled by
  `styles::button*` take `button_padding` rather than the showcase's `SP`.

### Task 5: gpui showcase behaviour and truth
- Bugs: (1) theme installs keep the installed accessibility preferences
  (`app.rs:1302-1304`, `:1259`, the watcher path); (2) `--variant` without
  `--theme` re-installs the theme in that mode (`main.rs:949-958`), and the
  comment says what happens; (3) the icon-theme list is rebuilt on every
  preset change and "system" follows presets like "default" does
  (`app.rs:1310-1331`); (4) `--icon-theme` applies until the user picks an
  icon theme, then stops (`support.rs:361-371, :612-647`, `app.rs:532-536`),
  docs to match; (5) the preset list and the status bar name the same preset
  for `default` (`support.rs:968`, `app.rs:1263`); (6) a failed theme restores
  the name and layout of the one still installed (`app.rs:692, :1281`,
  `main.rs:962`); (7) hiding the side panel calls `screen_changed()`.
- D7 shortcuts.
- Truth: `info/theme_map.rs:173` overlay; `info/chrome.rs:509-514, :527` the
  underline is TabBar's sliding indicator (`tab_bar.rs:268-275`);
  `info/chrome.rs:1042`, `info/overlays.rs:99` dismiss rule (34px from the
  window's top, `dialog/dialog.rs:586`, gpui-base `dialog.rs:601`);
  `info/chrome.rs:897-898` status-bar gaps; `info/buttons.rs:582`,
  `info/feedback.rs:608` one toggle; `main.rs:3,19` crate doc;
  `main.rs:683-697`, `support.rs:306, :331-338` misattached docs; the
  remaining "icon set" names for the user's choice (`demo.rs:920, :960,
  :4736`, `pages/icons.rs:32`, `app.rs:320-334`); `main.rs:262-263`;
  `pages/data.rs:68-69` version and licence from `CARGO_PKG_*`;
  `pages/layout.rs:303` names only existing connectors; `inspector.rs:326-328`
  named constants; `support.rs:460` and `docs/showcase-exceptions.toml:15`
  version labels; the unused `cli_override` off Linux.
- Tests: the tab inset test measures the right side and the rule's width;
  `corner(4.)`; the toolbar test's doc; `no_inspector_toggle_remains` walks
  submenus.

### Task 6: icons follow the chosen icon theme (D1)
Spike first (report before implementing): can an application `AssetSource`
serve gpui-component's `icons/*.svg` paths from the chosen theme and have a
switch show, given gpui's SVG caching? Then: every icon the showcase builds on
a page goes through the chosen theme (as the chrome's do), none where missing,
infos say which theme; and upstream's internal icons too if the spike allows.
Gate: a test per sample class that the drawn icon is the chosen theme's, and
one that a missing icon draws none.

### Task 7: release tooling and project docs
`publish.yml` CI gate mirrors CI/pre-release-check (iced `--no-default-features`,
`--features iced_aw` test and clippy, `cargo doc -p native-theme-iced
--all-features`, widget coverage, the Task 1 check); `screenshots.yml` builds
and runs the iced showcase with `--features iced_aw`; `dependency-canary.yml`
matches CHANGELOG's description; `scripts/README.md` documents
`compat-check.sh` and what `pre-release.sh` now runs; `compat-check.sh:327`
separator; CHANGELOG: `:74` 108, `table_row_border`/`link` entry, the two
"Fixed" entries about the new script folded into Added, ios; ROADMAP v0.6.1;
`docs/todo.md:131, :1550-1552, :1660`; platform-facts §2.14 link text.

### Task 8: final review, archive
Whole-branch review of Tasks 1-7; pre-release check; this document moves to
`docs/archive/` with an "As built" note.
