# v0.5.9 second pre-merge review: fixes

Status: approved by the maintainer 2026-09-24 ("go, fix all"). Source: the
second five-area review of `v0.5.9-gpui-kit-0.6.6` (merge base `399f45a`),
after the first review's fixes (`943891d..05c0a56`). Every finding below was
verified by the controller or by a reviewer against the code. The
maintainer also asked for more build tests in `pre-release-check.sh` (E1).

Rules for every task: never invent a value (a size needs a source, an unstated
one stays `None`); no panics, no `unsafe` outside the existing `#[allow(unsafe_code)]`
platform calls; no hardcoded theme values; never mix icon sets (a missing icon
is none, never another set's); a new or changed test fails first, and the
report shows the failure; stage by name, no attribution lines; `docs/todo.md`
is appended to or has its named lines corrected, nothing else; run
`env CARGO_BUILD_JOBS=4 ./pre-release-check.sh` before the last commit of a
task (expected warnings: stale visual assets, stale compat claims, uncommitted
asset sources). Scripts run under bash; the user's shell may be fish, so use
`env VAR=x cmd` in commands you run.

## Decisions

E1. **Every crate builds in every feature combination the gate names.** The
gpui connector does not compile with `--no-default-features` (it calls
`native_theme::rasterize` ungated, `connectors/native-theme-gpui/src/icons.rs:1171`),
and nothing builds it that way. A new `scripts/check-features.sh` checks, for
each workspace crate, the library with no default features, with each feature
alone (`--no-default-features --features F`), and with all features — the
coverage of `cargo hack check --each-feature`, without a new tool: features
come from `cargo metadata --no-deps --format-version 1` through `jq` (installed
locally and on GitHub's runners). `pre-release-check.sh`, `ci.yml`,
`publish.yml` and `dependency-canary.yml` run it.

E2. **Without `svg-rasterize` the gpui connector hands SVG to gpui.** gpui-pre
0.3.6 depends on resvg 0.46 unconditionally and decodes `ImageFormat::Svg`
itself (`gpui-pre-0.3.6/src/platform.rs:3034`). native-theme's
`svg-rasterize` adds resvg 0.48; turning it off saves that build, so the
feature stays. Without it, `svg_to_render_source` becomes an
`ImageSource::Image(Arc::new(Image::from_bytes(ImageFormat::Svg, bytes)))`
(colourisation still applies to the bytes first). Its cost, documented on
`to_image_source` and the feature: gpui decodes it asynchronously, so the
first frame paints nothing (the reason given at `icons.rs:1176-1182`), and
gpui chooses the raster size. No icon becomes `None` for want of the feature.

E3. **Cross-target builds deny warnings.** `native-theme` with `--all-features`
has 10 warnings on `x86_64-pc-windows-gnu` (dead code, unchecked `BOOL`s, an
unreachable expression), all from before this branch. They are fixed, and the
cross-target section of `pre-release-check.sh` also runs `cargo check --target
T -p native-theme --all-features` with `RUSTFLAGS="-D warnings"` for both
targets, as do CI's Windows and macOS legs natively.

E4. **KDE headings are weight 400, sized by Kirigami's factors.** Kirigami's
`Heading` defaults to `type: Heading.Type.Normal` (kirigami `8319acc`,
`src/templates/Heading.qml:89`) and sets `font.weight: type ===
Heading.Type.Primary ? Font.DemiBold : Font.Normal`
(`src/controls/Heading.qml:35`); level 1 is `defaultFont.pointSize * 1.35`,
level 2 `* 1.20` (`:14-33`). So `section_heading` and `dialog_title` are 400,
not 700. KDE has no `display` (platform-facts §2.19 `:1438`), so no preset
states one and it resolves from the body font
(`docs/inheritance-rules.toml [text_scale_inheritance]`). `caption` is
`smallestReadableFont` (`:1435`), default 8pt 400 (`:573`); 8.2 is the invented
0.82 ratio. The KDE reader reads `smallestReadableFont` for `caption` and
derives the two heading sizes from the font it reads, so live mode follows the
user's fonts.

E5. **A stated line height needs a source too.** The `-live` presets' text-scale
`line_height_pt` values (macOS 12.7/15.5/18.6/30.9, KDE 11.2/16.3/18.4/27.2)
are derived from the wrong sizes and cite nothing; where platform-facts gives
no line height, the preset states none and the resolver computes it.

E6. **The showcases reject what they cannot honour.** An unrecognised
`--variant`, `--icon-set` or `--theme` value is reported on stderr and ignored
(the setting stays what it would have been without the flag), never silently
replaced. `--variant system` means "follow the OS". `--icon-set freedesktop`
means the system icon theme, in both showcases. A macOS or Windows preset
named by `--theme` on Linux is rejected with a message: only Linux-native
presets run on Linux (the project rule, and the showcases' own preset lists).

## Tasks

### Task 1: feature combinations, gpui without `svg-rasterize`, cross-target warnings (E1, E2, E3)

- New `scripts/check-features.sh` (bash, `set -euo pipefail`, executable):
  for every workspace member from `cargo metadata --no-deps --format-version 1`
  (use `jq`), run `cargo check -p <crate> --lib` with `--no-default-features`,
  with `--no-default-features --features <F>` for each feature in its
  `features` table except `default`, and with `--all-features`. Print one line
  per combination; exit non-zero if any fails, naming each failure at the end.
  Fail with a clear message if `jq` is missing. Document it in
  `scripts/README.md`.
- Wire it: a "Feature combinations" section in `pre-release-check.sh`
  (a hard failure, not soft); a job in `.github/workflows/ci.yml`; the same
  job in `publish.yml` (and in its `needs`); a step in
  `dependency-canary.yml` (which mirrors CI's gates).
- Run it. Fix every failure it finds. The known one: the gpui connector
  without `svg-rasterize`, fixed per E2 — gate `native_theme::rasterize` use
  behind `#[cfg(feature = "svg-rasterize")]`, add the `ImageFormat::Svg`
  path for the other case, and document the difference on `to_image_source`,
  the other public `*_image_source` functions whose behaviour changes, and the
  feature list in `connectors/native-theme-gpui/README.md` and the crate docs.
  Tests of rasterized output stay gated on the feature; add a test that
  without the feature an SVG still yields `Some` (it runs under
  `--no-default-features`; run it that way in the report).
- E3: fix the Windows-target warnings from
  `cargo check --target x86_64-pc-windows-gnu -p native-theme --all-features`:
  - `native-theme/src/resolve/inheritance.rs:74` unreachable expression
    (make the platforms' branches exclusive with `cfg`, same behaviour);
  - `native-theme/src/icons.rs:466` unused `theme` on non-Linux
    (`let _ = theme;` under `#[cfg(not(target_os = "linux"))]`, as
    `load_all_icons` in the gpui showcase does);
  - `native-theme/src/pipeline.rs:576` `preset_as_reader` used only on Linux
    and in tests — gate it to where it is used;
  - `native-theme/src/windows.rs:179` `read_frame_width`: if
    `docs/platform-facts.md` names `SM_CXBORDER` as the Windows source of a
    field the reader should set, wire it there with that citation; otherwise
    delete it. Say which in the report;
  - `native-theme/src/windows.rs:328` `dwm_color_to_rgba` is a copy of the
    inline code at `:320-326`: make the reader call it;
  - `native-theme/src/winicons.rs:372` `load_windows_icon` is used only by
    tests: `load_icon` goes by name (`icons.rs:289-294`). Remove it and move
    its tests onto `load_windows_icon_by_name` with the names `icon_name`
    gives those roles, or gate it `#[cfg(test)]` if the tests need it — say
    which;
  - `native-theme/src/winicons.rs:137-138, :166-167` unused `BOOL` from
    `DeleteObject`: `let _ =` with a comment that a failed delete of a
    bitmap we own has no recovery.
  Then check `x86_64-apple-darwin` the same way (`cargo check --target
  x86_64-apple-darwin -p native-theme --all-features`) and fix its warnings
  the same way. Both targets must end with zero warnings.
- Extend `pre-release-check.sh`'s cross-target section (`:400-423`) to also
  run, per installed target, `env RUSTFLAGS="-D warnings" cargo check --target
  "$target" -p native-theme --all-features`. In `ci.yml`, the Windows and macOS
  legs run `cargo check -p native-theme --all-features` with
  `RUSTFLAGS: -Dwarnings`; mirror it in `publish.yml`.
- Done when: `scripts/check-features.sh` exits 0; both cross-target checks
  have zero warnings; `pre-release-check.sh` passes with the new sections.

### Task 2: preset and reader values against platform-facts, and a gate for them (E4, E5)

First extend the gate so each value below fails before it is fixed. The
existing gate is `native-theme/tests/documented_sizes.rs` (static and live
resolution of each native preset). Add rows, each citing its platform-facts
line, for: `text_scale.{caption,section_heading,dialog_title,display}` size
and weight; `dialog.title_font` size and weight; `slider.track_height`,
`slider.thumb_diameter`; `progress_bar.track_height`. A cell platform-facts
leaves to inheritance ("← `defaults.font`", "(none)") is checked as "not
stated by the preset" where the gate can tell, or skipped with a comment
naming the cell. The live resolution must include the reader constants the
gate already merges (it does for padding), so a reader value that differs
from platform-facts fails too.

Values (platform-facts line in brackets):
- **macOS** `macos-sonoma-live.toml` text scale (light and dark): caption 10 /
  400, section_heading 13 / 700, dialog_title 22 / 400, display 26 / 400
  [`:1435-1438`] — the static `macos-sonoma.toml` already has these sizes;
  make the two agree. Remove every `line_height_pt` in both files' text
  scale that has no platform-facts source (E5).
- **macOS reader** `native-theme/src/macos.rs:282`: `slider.track_height` 5
  [`:1292`], was 4.
- **macOS** `dialog.title_font`: size 13, weight 700 [`:1496-1497`]; state
  it in `macos-sonoma.toml` (light and dark). In `-live`, state the weight
  only if the size follows the system font the reader reads (the HIG source
  says "emphasized system font", i.e. the system font size) — the report
  says which.
- **GNOME** `dialog.title_font`: size 15, weight 800 [`:1496-1497`,
  `.title-2`], matching the value the adwaita presets already state for
  `text_scale.dialog_title`; state it in `adwaita.toml` and `adwaita-live.toml`
  (light and dark).
- **Windows reader** `native-theme/src/windows.rs:223`:
  `slider.thumb_diameter` 18 [`:1293`], was 22; `:224`
  `progress_bar.track_height` 1 [`:1303`, "`track_height` is the groove: 1"],
  was 4; and the same field in `windows-11.toml` (light `:178`, dark) and
  `windows-11-live.toml` (light `:92`, dark), was 3. Update the reader's
  testable build (the non-Windows `read_widget_sizing`/`winui3_widget_sizing`
  constants) and its unit tests (e.g. `windows.rs:1252`).
- **KDE** (E4) `kde-breeze.toml` and `kde-breeze-live.toml`, light and dark:
  caption 8 / 400; section_heading 12 / 400; dialog_title 13.5 / 400; no
  `display` table; no unsourced `line_height_pt`. Update platform-facts
  `:1436-1437` KDE cells to add the weight ("`Font.Normal` (400) unless
  `type: Primary`") with the kirigami `8319acc` citations from E4.
- **KDE reader** (`native-theme/src/kde/`): read `smallestReadableFont` from
  `[General]` (the same parser the reader uses for `font`) into
  `text_scale.caption` (size and weight); set `text_scale.section_heading`
  and `text_scale.dialog_title` sizes to the read `font` size × 1.20 and
  × 1.35 with weight 400 (E4). Unit tests from a kdeglobals fixture with a
  non-default font (e.g. 11pt body, 9pt smallest).
- Check with `rg` that no doc, test or showcase claim repeats an old value
  (8.2, 10.7, 15.6, 22.0 thumb, track 3/4 for Windows progress, macOS slider
  track 4); fix the ones that do.
- Done when: the extended gate passes for every native preset, static and
  live; `cargo test -p native-theme` passes.

### Task 3: iced showcase behaviour (E6)

In `connectors/native-theme-iced/examples/showcase-iced.rs`. The gpui showcase
already fixed the first three; port its logic, not a new design.
1. A failed theme install leaves the pickers showing what is not drawn:
   `ThemeSelected`/`ColorModeSelected` (`:1510-1516`) set `current_choice`
   and `rebuild_theme` sets `is_dark` from the new mode (`:1077`) before
   anything loads; on failure `current_resolved`/`current_theme` stay. Commit
   `current_choice`, `color_mode` and `is_dark` only when the install
   succeeds (gpui: `app.rs` `install_in_mode`, `keep_installed_theme`). Map
   `--theme default` to `OsTheme`, as gpui does. Test: port gpui's
   `a_failed_cli_theme_keeps_the_colour_mode_shown` idea — a failing preset
   leaves choice, mode and `is_dark` unchanged.
2. `--icon-set freedesktop` (`:1047`, passed by
   `scripts/generate_screenshots.sh:24-25`) becomes
   `IconSetChoice::Freedesktop("freedesktop")`. Map `freedesktop` to
   `System` (gpui `main.rs:975`); accept any other name as a freedesktop theme
   only if it is installed (`is_freedesktop_theme_available`), else report it
   on stderr and ignore it (E6). Test.
3. The spinner comes from the system theme, not the chosen one:
   `build_animation_caches(set)` (`:643`) calls `load_icon_indicator(set)`,
   which loads from `FreedesktopLoader::load_indicator(None)`. Pass the
   choice's freedesktop theme through `FreedesktopLoader::load_indicator`, as
   gpui does (`app.rs:503`); all callers (`:902`, `:1054`, `:1176`, `:1588`).
   A theme without an indicator gets none (never another set's). Test.
4. A `System` icon choice stops following presets (`:1158`,
   `follows_preset()` true only for `Default(_)`). Keep a separate
   "follows preset" flag that only a user pick of the icon theme clears, like
   gpui's `icon_choice_follows_preset`. Test: a fallback-to-system choice
   re-derives on the next preset change.
5. E6 for `--variant` (`:1018-1022`: anything but `dark` becomes light):
   accept `light`, `dark`, `system`; report anything else. `--theme`: reject
   a macOS/Windows preset on Linux with a message, and an unknown name.
- Done when: `cargo test -p native-theme-iced --all-targets --features
  iced_aw` passes with the new tests.

### Task 4: gpui showcase CLI and icon-theme Select (E6)

In `connectors/native-theme-gpui/examples/showcase-gpui/`.
1. With `--icon-theme` in effect, the icon-theme Select names a different
   theme from the icons drawn: `icon_choice_row()` (`app.rs:739-761`) ignores
   `icon_theme_override`, while the icons and `icon_set_label()` use it.
   While the override stands, the Select shows it (e.g. the matching
   `Freedesktop(name)` row, added if the list lacks it). Test.
2. E6: `--variant` (`main.rs:951-955`) accepts `light`, `dark`, `system`,
   reports anything else; `--icon-set` (`main.rs:975`) accepts the names the
   Select offers (including `gpui-builtin`) and `freedesktop` (= system),
   reports anything else instead of coercing to System; `--theme` with a
   macOS/Windows preset on Linux (today it installs and the preset Select goes
   empty, because upstream `set_selected_values` drops values it cannot
   resolve, `combobox.rs:314-337`) is rejected with a message, as is an
   unknown name (check what `apply_cli_args` already does for unknown names
   and keep that). Tests in `tests.rs`.
- Done when: `cargo test -p native-theme-gpui --examples` passes with the
  new tests.

### Task 5: tests that test nothing, and the `UNREACHABLE` doc

1. `connectors/native-theme-gpui/src/geometry.rs:862-865` `CASES` holds only
   catppuccin-mocha/latte, which since D4 of the previous plan state no
   padding, `row_height` or `item_gap`, so every `assert_padding`
   (`:901, :930, :964, :972, :994, :1003, :1011, :1268, :1335, :1364, :1373`)
   compares `None` with `None`. Add `("windows-11", Light)` and
   `("kde-breeze", Dark)` (both state padding sides; windows-11 both row
   heights), and make `assert_padding` count the sides it compared as
   `Some`, failing a builder test whose cases compared none.
2. `connectors/native-theme-iced/src/lib.rs:743-770`
   `stated_padding_sides_are_the_themes` uses catppuccin-mocha, which states
   no button or input padding, so its body never runs. Use
   `make_resolved_preset("windows-11", false)` and assert a side was compared.
3. `connectors/native-theme-iced/src/contract/derived.rs:784-790`: the doc
   says every value a `styles` doc calls receiverless is in `UNREACHABLE` but
   one; the styles docs also call receiverless `menu.hover_text_color`,
   `.font.color`, `.disabled_text_color`, `.separator_color`, `.row_height`,
   `.icon_size`, `.icon_text_gap` (`styles/aw.rs:116-119`), `tab.min_width`,
   `.min_height` (`:188`) and the radio's disabled fields (`styles.rs:588`).
   Narrow the sentence to what the list covers (Style-level values with no
   receiver anywhere in iced or iced_aw), and name the others' reason.
- Done when: the changed tests pass and each was shown to fail against a
  deliberately broken builder (report the break and the failure; revert it).

### Task 6: rustdoc corrections

Each verified; fix the text to match the code (file:line is where the wrong
text is):
1. `connectors/native-theme-gpui/src/icons.rs:1086` example calls
   `native_theme::loading_indicator()`, which does not exist: use
   `native_theme::icons::load_icon_indicator(IconSet::…)` returning
   `Option<AnimatedIcon>` (`native-theme/src/icons.rs:449`).
2. `native-theme/src/model/font.rs:73-74` says `FontSize` has no
   `Serialize`/`Deserialize`; `:78` derives both (for
   `ResolvedFontSpec::defined_size`).
3. `font.rs:248-250` and `:81-85`: the OS readers (macOS too, `macos.rs:203`)
   and the native presets state points; the colour-scheme presets state
   pixels.
4. `font.rs:242-243`: `Pt` is always converted at the resolution context's
   font DPI (`resolve/validate_helpers.rs:167`, `context.rs:43`); there is no
   `font_dpi` field on `ThemeDefaults`. And `:231-232` "all fields are
   required" — `defined_size` is `Option`.
5. `native-theme/src/model/widgets/mod.rs:738-740` `arrow_area_width`: `None`
   where the theme states no width — GNOME draws its arrow inline, macOS's is
   only a measured range (~16–18px, platform-facts `:1554`), colour schemes
   cite none.
6. `connectors/native-theme-gpui/src/lib.rs:219` `from_preset`: the display
   name is the preset's `name` field (`:246`, `:253`), e.g. "Dracula".
7. `lib.rs:270-273` `from_system`: the advice to call
   `SystemTheme::from_system()` to avoid resolving both variants contradicts
   itself; point to the single-variant path or drop it.
8. `connectors/native-theme-iced/src/lib.rs:115` coverage table: the fields
   are `input.placeholder_color`, `defaults.surface_color`,
   `defaults.text_color`, `defaults.{accent,success,danger,warning}_text_color`
   (`:184-192`).
9. `connectors/native-theme-iced/src/palette.rs:9`: `shadow_color`,
   `selection_inactive_background` (`model/resolved.rs:94`, `:102`).
10. `connectors/native-theme-iced/src/styles/aw.rs:3-4`: iced has no card,
    menu bar, tab bar, sidebar, spinner or selection list; its drop-down menu
    is styled by `styles::menu` (`styles.rs:836-852`) and a container card by
    `styles::container_card` (`:1241-1256`).
11. `connectors/native-theme-gpui/src/lib.rs:65, :67` coverage table: `window`
    adds the title-bar font (`geometry.rs:620-626`); `status_bar` adds the
    padding sides and font (`geometry.rs:357-364`).
12. `native-theme-build/src/lib.rs:4`: the trait is
    `native_theme::theme::IconProvider` (`codegen.rs:138`).
13. `connectors/native-theme-gpui/src/config.rs:5, :87-89`: count the
    `ThemeColor` fields exported against gpui-component 0.6.6
    `theme/schema.rs` and state the true numbers.
14. `connectors/native-theme-iced/src/styles.rs:171-172` "an opaque grey" on
    fifteen presets: kde-breeze's `button.hover_background` is `#93cee9`;
    state what is true across the presets.
15. Feature-gated public items (e.g. `watch`, `rasterize`, `system-icons`
    loaders, the iced `iced_aw` styles) say in their docs which feature they
    need, in one line each ("Requires the `watch` feature."). Where the
    crate-level docs list features, they match the Cargo.toml.
- Done when: `env RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
  --all-features` and `cargo test --doc --workspace --all-features` pass.

### Task 7: READMEs, CHANGELOG and other docs

1. `CHANGELOG.md` `[Unreleased]`:
   - `:100` InputGroup "Copy" fix: v0.5.8 had no InputGroup; fold into the
     Added text at `:56`.
   - `:69` vs `:76`: say "Cmd+Q on macOS, Ctrl+Q elsewhere (likewise B, K
     and ,)" at `:69`; delete `:76` (v0.5.8 had no key bindings).
   - `:94`: drop "the iced connector's docs did not build without `iced_aw`"
     (never shipped); keep the release-gate part.
   - `:81`: keep only the Tooltip sentence (v0.5.8 `lib.rs:101-102`).
   - `:82`: v0.5.8's iced showcase had no menu bar, menus, context menu,
     iced_aw Card/TabBar/Tabs, card containers or Theme Information panel;
     only the push buttons, page tabs and "not themeable: padding" notes
     shipped. Keep those under Fixed, move the rest to Added.
   - `:44`: "12 of the connector's 33" geometry builders (v0.5.8).
   - `:114`: drop "the status bar and" (the status bar is new).
   - Add a Changed entry: `geometry::button` now sets `button.font`'s weight
     (`geometry.rs:198`).
   - Add entries for this plan's user-visible changes (Tasks 1–5: the feature
     gate, gpui without `svg-rasterize`, each preset and reader value with
     old → new and its source, the showcase fixes). Preset/reader values that
     shipped in 0.5.8 are Fixed.
2. `README.md:36`, `native-theme/README.md:46` "Every field has a value":
   every colour, font and required metric has a value; sizes the platform
   does not state are `None`, and the toolkit's own apply.
3. `native-theme/README.md:170` `system-icons`: platform icon lookups —
   freedesktop (Linux), SF Symbols (macOS), Segoe Fluent (Windows); check what
   the feature enables in `native-theme/Cargo.toml` and say it.
4. `native-theme/src/presets/README.md:10-11`: colour-scheme presets state
   colours and fonts and no sizes (D4); `:22` names readers `from_kde` etc.
   that do not exist — readers are `ThemeReader` impls (`macos.rs:415`,
   `windows.rs:556`).
5. `connectors/native-theme-iced/README.md:197-202` "Full helper list": add
   `border_color`, `disabled_opacity`, `focus_ring_color`, `link_color`,
   `selection_color`, `info_color`, `info_foreground_color`,
   `warning_foreground_color`, `icon_sizes` (`src/lib.rs:426-480`; check the
   list against the code); `:82` the `^ is_dark` comment points at nothing.
6. `ROADMAP.md:88-89` and `docs/todo.md:265-267`: "input padding" is
   delivered (gpui README `:218`); remove it from both lists.
7. `CONTRIBUTING.md:24-31` check list: add the cross-target checks, the
   feature-combination check (Task 1), the iced configurations, widget
   coverage, the PROVENANCE stamp and compat claims — read
   `pre-release-check.sh` for the actual order; `:144`: `pre-release.sh`
   needs Python 3.11+ with Pillow and network access (`scripts/README.md`).
8. `SECURITY.md:54-55`: `cargo audit` runs in CI (`ci.yml:146-152`) on
   every push and pull request — say what the workflow's triggers are.
9. `docs/new-os-version-guide.md` `:85, :105` `macos_widget_metrics()` →
   `macos_widget_defaults()` (`macos.rs:247`); `:85-86` `from_macos()` →
   the reader's real name (`MacosReader`, `macos.rs:415-422`); `:123` the
   GNOME names that do not exist (check `gnome/mod.rs`); `:153-156` preset
   section and key names (`[light.defaults]`, `corner_radius_lg_px`, …).
10. `docs/todo.md`: `:1715-1722` change notification — all four backends
    exist (`native-theme/src/watch/{gnome,kde,macos,windows}.rs`); mark them
    done. Table B (`:1428-1463`): the `menu.row_height` and
    `list.row_height` rows are done (previous plan D3); mark them as the
    `arrow_area_width` row is marked. Append a follow-up: docs.rs
    `doc(cfg)` badges (needs nightly `doc_cfg`; Task 6 put the feature in
    prose).
11. Check every README and doc this plan touched once more for a value
    Tasks 1–5 changed.
- Done when: every item is fixed and `pre-release-check.sh` passes.

### Task 8: archive

Move this file to `docs/archive/todo_v0.5.9_second-review-fixes.md` with an
"As built" section (commits, rulings, anything left and where it is
recorded); update links to it.

## As built (2026-09-24)

All eight tasks done, 05c0a56..d84c0a3 plus the archiving commit; nothing
pushed. Commits by task:

- Task 1: 3f205b3, 1c5655e, 1dc99f8, b35432a; review fixes d45851d,
  54bf51a, 033ba08, 58ff218.
- Task 2: 318d7e4, d6dd8a1; review fixes 2ec72d3, c88b4ed.
- Task 3: 071ad16; review fixes fba7b48.
- Task 4: 157aeb7.
- Task 5: b33a7ee, 8c67ae0, e8319b2; review fixes 927d243, bed2557.
- Task 6: af72915, 86c0747, 0ffc732, 1ebd5ef; review fixes 0470586,
  3df7773.
- Task 7: 2e52bb0, 60d52e7, 2b8fe14, f7c925e; review fixes 03a7911,
  4e40952.
- Final review fixes: 748d5b1, 34f6697, a784109, 5279515, 5112ec7, 6c36ab2,
  9bafb44, de5238f, 7196083, 2fe86bc.
- Task 8: the final re-review's three residuals, 64e04af, 616736d, d84c0a3.

Controller rulings, each with its cost if wrong:

- Every Windows `GetSystemMetricsForDpi` read became logical pixels, not only
  the new `SM_CXBORDER`. Cost: small; at 100 % scaling nothing changes.
- Task 1's deferred minors (single-feature warnings, the `reader_kde` gate)
  were fixed in its first fix round. Cost: a few minutes.
- The Windows reader reads its fonts at 96 DPI and reports a `font_dpi` of
  96. Cost: Windows text size at more than 100 % scaling.
- `--theme` takes exactly the presets the picker offers on the platform
  (`kde-breeze` is rejected on non-KDE Linux), in both showcases. Cost:
  another desktop's preset cannot be previewed from the command line, as it
  already cannot from the picker.
- `--icon-set` takes exactly the icon picker's rows, in both showcases.
  Cost: an unlisted theme such as `hicolor` cannot be chosen there.
- gpui's `--theme <bad> --variant V` rejects the theme and applies the
  variant, as iced does (E6). Cost: a user expecting a rollback sees the
  variant applied.
- `--icon-theme` takes only the installed themes the Select lists. Cost:
  `--icon-theme hicolor`, and every `--icon-theme` off Linux, are rejected
  (off Linux they had no effect).
- `tab.min_width` and `tab.min_height` joined `UNREACHABLE`, as
  `progress_bar.min_width` is there. Cost: two list entries.
- The `UNREACHABLE` minima sentence names only the listed entries; the
  other minima were not moved into the contract list. Cost: the ruling
  assumed they had declared sources; they had none anywhere in the iced
  contract, and the final review moved button's and combo_box's into
  `UNREACHABLE` (`input.min_height` has `TextEditor::min_height`; dialog is
  unmapped in iced, recorded in `docs/todo.md`).
- The plan's README wording for unstated sizes was over-broad; it names the
  `Option` fields only. Cost: none.
- `input.min_height` stays out of `UNREACHABLE` (`TextEditor::min_height`
  receives it); the dialog minima are not listed alone, since iced maps no
  `DialogTheme` field. Cost: none.
- The re-review's three residuals were folded into Task 8; the iced
  showcase's startup fallback names the adwaita preset in the picker. Cost:
  the picker shows `adwaita`, not `default (adwaita)`, after a failed OS
  read, which is what is drawn.

Left, and recorded in `docs/todo.md`:

- "freedesktop icon lookup falls back to hicolor": does the fallback of
  `freedesktop-icons` break the never-mix rule? A question for the
  maintainer.
- "adwaita-live: title and heading sizes ignore the user's font".
- "iced: dialog is unmapped".
- "Font DPI": do the Linux readers' `font_dpi` sources have Windows' logical
  versus physical question under Wayland fractional scaling?
- "docs.rs feature badges" (`doc(cfg)`, needs nightly).
- "Live presets": `windows-11-live.toml` still states fields the Windows
  reader provides.

Found beyond the plan:

- `watch` alone did not compile (zbus without a runtime feature); `portal`
  now names zbus, and `watch` adds its blocking API only with it. `watch`
  with `macos` did not compile on macOS (`objc2-core-foundation`).
- The macOS reader sets `defaults.border.color` from `separatorColor`, the
  source platform-facts names; the binding had lost its use in the schema
  rename.
- The Windows reader read its metrics and fonts at the system DPI, so a
  DPI-aware process got device pixels (`SM_CXVSCROLL` 26 instead of 17 at
  150 %) and a 9pt font resolved to 18px at 144 DPI. It now reads both at
  96 DPI and reports a `font_dpi` of 96.
- The iced showcase's startup fallback left the picker on `default`, so
  after it a colour-mode change, `--variant` and the theme watcher failed;
  it now names adwaita, and a mode change installs adwaita in that mode.
  The gpui showcase falls back to gpui-component's built-in theme under
  `default`; a mode change there fails to read the OS theme again, reports
  it, and switches the built-in theme's mode.
