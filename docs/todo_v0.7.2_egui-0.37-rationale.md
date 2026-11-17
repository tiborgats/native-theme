# egui 0.37 in v0.7.2: rationale

Status: written 2026-11-15; revised on 2026-11-16 after two reviews, the trial build (E14), a
further review, a final check, a follow-up trial and its author's own re-reads, whose decisions it
records. Approved by the maintainer on 2026-11-17, with the specification,
`docs/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-spec.md`, and the plan,
`docs/todo_v0.7.2_egui-0.37-and-gpui-kit-0.10.0-plan.md`; being built.

Names used below. `E37` and `E36` are the `src/` folders of egui 0.37.0 and 0.36.2 in Cargo's
registry (`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/egui-0.37.0/src`); `P37`, `P36`
the same for epaint, and `eframe-0.37.0`, `egui_kittest-0.37.0` and so on name a crate's folder
there. Upstream lines carry their version, so they do not drift. `CN` is
`connectors/native-theme-egui`, `CW` is `connectors/native-theme-egui-widgets`, `SC` is
`CN/examples/showcase-egui`. Repository files are cited by path and a quoted phrase, never by line.

## 0. What this document is for

The maintainer's release order makes v0.7.2 the move of the egui connector from egui 0.36.2 to
0.37 (`docs/todo_v0.7.1_batch-speed-rationale.md`, S3's amendment and its paragraph on v0.7.2),
approved by the maintainer on 2026-11-16. v0.7.2 also takes GPUI Kit 0.10.0, which has a
rationale of its own (`docs/todo_v0.7.2_gpui-kit-0.10.0-rationale.md`). The
release's lock is the two moves together; the GPUI Kit 0.10.0 trial, run on 2026-11-16 on top of
this document's trial, resolved and checked that lock (§1.1, §1.7). This document decides the egui
move only; the version bump, the asset run and the close-out are shared and belong to the release's
plan (E12). It is outside the two lanes. Every later release is written on egui 0.36.2, and VF
rewrites their egui forms when each merges the `v0.7.2` tag (rule 25 there).

`docs/todo.md`, "Dependencies: the newer releases the outdated check did not show (2026-11-14)",
records the need: the requirement `egui = "0.36.2"` does not take 0.37, so an application on egui
0.37 cannot use native-theme-egui. It calls the move "a connector upgrade of the GPUI Kit kind,
with its own rationale". This is that rationale. It lists what egui 0.37 changes in what the
workspace uses, and decides what happens to each item.

v0.8.0's branch has nothing to reuse here. `git log v0.8.0-rc1 --oneline --
connectors/native-theme-egui` has three commits not on `main` (`78ee0efd`, `4ee73e7c`,
`8e0b189a`); none concerns egui 0.37. `git grep "egui 0.37\|0\.37" v0.8.0-rc1 -- docs/` finds only
the 0.6.0 documents' statement that the caret requirement rejects 0.37.

## 1. What was read

### 1.1 Sources

- The egui repository at tag `0.37.0` (commit `418687fa18fdac8a9b1b6c8f4a2eb1afd2df4c47`, read
  with `gh api repos/emilk/egui/git/ref/tags/0.37.0`): `CHANGELOG.md`, and
  `crates/{eframe,epaint,egui_extras,egui_kittest,ecolor,emath,egui-winit}/CHANGELOG.md`, fetched
  on 2026-11-15 from `https://raw.githubusercontent.com/emilk/egui/0.37.0/<path>`. There is no
  `crates/egui/CHANGELOG.md` at the tag (404); egui's own log is the root file. The GitHub release
  "0.37.0 - Colored emojis!" (published 2026-10-15) has the same text. Neither has a migration
  guide; the root changelog's "Breaking changes" list is the closest.
- The sources of egui, epaint, emath, ecolor, eframe, egui_extras, egui_kittest, egui-winit,
  egui-wgpu and egui_glow at 0.36.2 and 0.37.0, and of kittest 0.4.0 and 0.5.0, all already in
  Cargo's registry. Compared by script: public items and struct fields removed, added or changed
  in type; `#[deprecated]` items new in 0.37.0; `impl Widget for` types; the egui `Style`
  defaults (`E36/style.rs` against `E37/style.rs`).
- `cargo search egui --limit 3` printed `egui = "0.37.0"` first. `cargo info` printed, for egui,
  eframe, egui_extras and egui_kittest each, "version: 0.36.2 (latest 0.37.0)" and
  "rust-version: 1.95". `gh api repos/emilk/egui/releases` lists 0.37.0 (2026-10-15) as the
  newest release. So 0.37.0 is the only 0.37 release on 2026-11-15.
- The workspace at `0fa28027`: every `Cargo.toml` that names egui or eframe, `Cargo.lock`, `CN`
  and `CW` (sources, tests, READMEs, `CN/mapping.toml`, `CN/src/mapping.md`), `SC`,
  `tools/showcase-support`, `docs/showcase-exceptions.toml`, `docs/COMPATIBILITY.toml`, the CI
  workflows and `scripts/`. The places that use egui were found with searches (`git grep`) for
  each changed or removed name the comparison above produced, not from memory.
- The measuring build of E14, run on 2026-11-16 on a throwaway branch (`trial/egui-0.37`, commit
  `2027d866`, from `e866ea6c`; nothing of it merges): the four manifest edits, one
  `cargo update -p egui --precise 0.37.0`, `cargo check` of the egui crates and of the gpui
  connector, clippy, the egui crates' tests, `cargo check --workspace --all-targets`. Its
  report, lock moves, patch and one log per command are kept outside the repository, in the
  maintainer's `scratch-keep/egui-037-trial/`. "Measured" below means from there.
- The GPUI Kit 0.10.0 measuring build, run on 2026-11-16 on top of that commit (`2027d866`), so
  its lock is the two moves together: its report
  (`scratch-keep/gpui-kit-0.10.0-trial/trial/report.md`, with `lock-moves.txt` and
  `coverage.txt` beside it). The widget coverage gate of §1.6 was run
  there, on that joint state; its egui figures do not depend on the GPUI Kit move.

### 1.2 The versions

The lock as the trial resolved it (measured):

| Crate | `Cargo.lock` at 0.7.1 | The trial's lock | Its own requirement on the others |
|---|---|---|---|
| egui, epaint, emath, ecolor, epaint_default_fonts | 0.36.2 | 0.37.0 | each other at 0.37.0 |
| eframe, egui-winit, egui-wgpu, egui_glow | 0.36.2 | 0.37.0 | egui 0.37.0; eframe adds `egui_system_font_provider` 0.37.0 |
| egui_extras | 0.36.2 | 0.37.0 | egui 0.37.0 |
| egui_kittest | 0.36.2 | 0.37.0 | egui, eframe, egui-wgpu 0.37.0; kittest 0.5.0 (was 0.4.0) |
| accesskit, accesskit_winit | 0.24.1, 0.32.2 | 0.25.2, 0.34.2 (0.24.1 kept for other dependents) | 0.25.0 and 0.34.0 |
| skrifa | 0.44.0 | 0.44.0 | epaint 0.37.0 still asks `0.44.0` (`epaint-0.37.0/Cargo.toml:157`) |
| winit | 0.30.13 | 0.30.13 | still `0.30.13` |
| wgpu | 30.0.1 (and 27.0.1 for iced) | unchanged | still `30.0` |
| jiff (the showcase's) | 0.2.35 | 0.2.35 | still `0.2.35` |
| unicode-properties | 0.1.3 | 0.1.5 | epaint 0.37.0 adds `0.1.4` (`epaint-0.37.0/Cargo.toml:171-172`) |
| smallvec | 1.15.2 | 1.16.5 | egui and epaint 0.37 ask `1.16` |
| rust-i18n-support | 4.2.1 | 4.2.4 | the F1 chain (§1.7); rust-i18n and rust-i18n-macro stay 4.2.1 |
| serde-saphyr, granit-parser | 0.0.29, 0.0.7 | 1.3.0, 1.3.0 | the F1 chain (§1.7) |

Other moves (the trial's lock diff): added egui_system_font_provider 0.37.0, fontique 0.11.1,
parlance 0.1.1, poll-promise 0.3.0, accesskit_ios 0.2.2, accesskit_macos 0.27.2; moved enum-map
2.7.3 → 3.1.0, enum-map-derive 0.17.0 → 1.0.1, kittest 0.4.0 → 0.5.0, vello_cpu and vello_common
0.1.0 → 0.2.0, glifo 0.2.0 → 0.3.0, accesskit_atspi_common to 0.21.1, accesskit_consumer to
0.39.2, accesskit_unix to 0.24.1, accesskit_windows to 0.35.2. No part of the 2026-11-15 draft or
of the reviews named: enum-map-derive; granit-parser; phf, phf_generator, phf_macros and
phf_shared 0.14.0 (accesskit_atspi_common 0.21.1's); wl-clipboard-rs 0.9.5, os_pipe 1.2.3,
tree_magic_mini 3.2.2 and petgraph 0.8.3 (egui-winit 0.37's `wayland` feature now turns on
arboard's `wayland-data-control`; arboard does not move); send_wrapper 0.6.0 (eframe's, wasm32
only). None of them caused an error.

The joint lock (the GPUI Kit 0.10.0 trial's report, §1): one `cargo update -p gpui-component
--precise 0.10.0` on top of this lock resolved it, moving 14 GPUI packages; skrifa 0.44.0 stays for
the egui crates, while swash 0.2.10 moved its skrifa from 0.44.0 to 0.37.0. In 0.7.1's
`Cargo.lock` swash is reached only from the gpui connector (through gpui-fast-wgpu) and the iced
connector (through cosmic-text); the joint lock adds no dependent of it (the GPUI Kit trial's
`lock-moves.txt`: the only new edges are gpui-base → sys-locale and gpui-fast-macos → serde_json),
and the manifests of egui, epaint, eframe 0.37.0 and of their new font crates
(egui_system_font_provider, fontique, parlance) do not name it. So check_release's "one skrifa
(native-theme-egui)", `cargo tree -p native-theme-egui --target all -d` listing no second skrifa,
is untouched by swash's move (read from the lock; the check was not run).

Every egui-family crate declares `rust-version = "1.95"` in 0.37.0, as in 0.36.2
(`egui-0.37.0/Cargo.toml:14`). Of the moved packages (`cargo metadata` in the trial), the highest
declare 1.95 (egui_system_font_provider, enum-map, enum-map-derive, kittest); serde-saphyr 1.3.0
declares 1.89 and reaches the workspace through the GPUI graph (the F1 chain, §1.7), whose
connector declares `rust-version = "1.95.0"` (`connectors/native-theme-gpui/Cargo.toml`), so no
floor moves (in 0.7.1's `Cargo.lock` serde-saphyr is in native-theme-gpui's graph only); the rest
declare 1.88 or lower, or none. Their licences are MIT and/or Apache-2.0; unicode-properties is
"(MIT OR Apache-2.0) AND Unicode-3.0".

### 1.3 Breaking changes and deprecations the workspace hits

The CI and `scripts/check_release.sh` build with `-D warnings` (`ci.yml`, "RUSTFLAGS: -Dwarnings";
the clippy steps' "-- -D warnings"), so a deprecation fails the gate like a removal. The rows
were first found by search and a reading of both versions (2026-11-15); the trial's compile
errors and clippy output (2026-11-16) are added to each row as measured. B12 is the trial's own
find.

| # | What 0.37 changes | Upstream | Where the workspace uses it |
|---|---|---|---|
| B1 | `Id::new` is deprecated; `Id::unique` hashes exactly as it did (`Id::new` now calls it). `From<&'static str>` and `From<String>` for `Id` are removed, not deprecated | root changelog, "Breaking changes"; `E37/id.rs:101-105` | 28 calls in code (measured: clippy reported 27; the 28th, in `CW/src/switch.rs`, was a compile-error site, B4's `IdSalt`, rewritten before clippy ran): `CN/src` (`atlas.rs`, `icons.rs`, `plugin.rs`, `install_tests.rs`), `CW/src` (`links.rs`, `parts.rs`, `spinner.rs`, `switch.rs`), `SC` (`chrome.rs`, `demo.rs`, `pages/basic.rs`, `pages/containers.rs`, `pages/overlays.rs`); doc comments that name `Id::new` (`CN/src/icons.rs` two, `CN/src/plugin.rs`); the README example "let id = egui::Id::new(\"name\")" in `CN/README.md`. The removed `From` impls are not used (B11 still compiles) |
| B2 | `WidgetType` is now a deprecated alias of `egui::Role` (accesskit's). `Checkbox`, `CollapsingHeader` and `SelectableLabel` are not `Role` variants | `E37/lib.rs:643-659` | 13 uses (measured: 13): `WidgetType::Button` 6, `Label` 3, `ProgressIndicator` 1 (deprecated); `Checkbox` in `CW/src/switch.rs`, `CollapsingHeader` in `CW/src/expander.rs`, `SelectableLabel` in `SC/pages/basic.rs` (compile errors) |
| B3 | `Style::checkbox_style`, `button_style`, `widget_style`, `label_style` are gone; widgets get their style through `Ui::widget_style::<S>(id, &Classes)` or `Context::get_widget_style::<S>(&StyleArgs)`, both `#[doc(hidden)]` without the `experimental` feature, from the `StyleProvider` system in egui's `theme` module, which is private without that feature. `StyleArgs`, `ButtonStyle` and `CheckboxStyle` (`egui::widget_style`) and `class::Classes` are public and documented. `Classes` moved to `egui::class`; `CheckboxStyle`'s and `ButtonStyle`'s `frame` and `text_style` moved into `atom_layout` | `E36/widget_style.rs:146`, `:174`; `E37/theme/mod.rs:25-48`; `E37/context.rs:2484-2491`; `E37/lib.rs:424-427` (the module), `:434`, `:441`; `E37/theme/default_style.rs` | `CW/src/switch.rs` ("ui.style().checkbox_style(&Classes::default(), state)", "use native_theme_egui::egui::widget_style::Classes"); `SC/pages/basic.rs` (".button_style(&Classes::default(), WidgetState::Inactive)", ".checkbox_style(", reading both moved fields). Five compile errors; E15 decides the replacement |
| B4 | `AtomLayout` is renamed `WidgetAtom` (deprecated alias); `Atom::custom` and `rect` take an `IdSalt`, not an `Id`; `fallback_text_color` moved from the allocated layout to its `container`, which still compiles through `DerefMut` | root changelog (#8222); `E37/atomics/widget_atom.rs:26-31`, `:231`, `:302-316`; `E37/atomics/atom.rs:98` | `CW/src/switch.rs` ("egui::AtomLayout::new(atoms)", "egui::Atom::custom(rect_id, atom)", "laid.rect(rect_id)", "prepared.fallback_text_color"); `SC/pages/buttons.rs` ("egui::AtomLayout::new((\"AtomLayout\", \"of two atoms\"))"). Measured: two compile errors (the `IdSalt`), two deprecations |
| B5 | `scroll_area::State::offset` is no longer a public field; `clamped_offset()` or `unclamped_offset()` | `E37/containers/scroll_area.rs:121`, `:134`, `:146-147` | `SC/pages/basic.rs` ("let offset = area.state.offset.y;"), one compile error |
| B6 | `Ui::id` is deprecated, renamed `scope_id` | `E37/ui.rs:363`, `:368-370` | `SC/info.rs` ("ui.id().with(\"info copy\")"), one deprecation |
| B7 | `TextEdit::vertical_align` and `horizontal_align` are deprecated for `align(Align2)` | `E37/widgets/text_edit/builder.rs:408`, `:414-424` | 5 calls (measured: 5): `SC/pages/basic.rs` 1, `SC/pages/inputs.rs` 4 (".vertical_align(egui::Align::Center)") |
| B8 | `FontData::font` is a `Blob` (`Arc<dyn AsRef<[u8]> + Send + Sync>`), no longer a `Cow<[u8]>`; `FontData::from_blob` and `FontData::bytes()` are new | `P37/text/font_data.rs:10`, `:20`, `:40`, `:50` | Measured compile errors (12 sites): `CN/src/fonts.rs`, "data.font.as_ref()" and "len: data.font.len()" (three sites) and two of its tests; `CN/src/install_tests.rs`, `fn bundled` and `t5_line_spacing_and_slider_expansion_at_factor_two`, which match on `Cow` (E17), and the "font.as_ref()" comparisons of three T6 tests (four sites); `SC/tests.rs`, "skrifa::FontRef::from_index(&data.font" |
| B9 | `ScrollStyle` gains the field `kinetic: KineticScrollStyle` | `E37/style.rs:586`, `:851-884` | `CN/src/tripwires.rs`, "every_style_type_is_destructured_exhaustively": fails to compile by design, naming the field (E19); `CN/src/style_diff.rs`, `fn scroll`, which destructures `ScrollStyle` twice with no `..` and is `CN/mapping.toml`'s normative path list (E18). Measured: three compile errors |
| B10 | `default_fonts` is no longer a default feature of egui; epaint's emoji faces moved behind `monochrome_emoji_fonts`; a new `egui-icons` face is in the defaults | root changelog (#8583); `egui-0.37.0/Cargo.toml:62`; `P37/text/font_definitions.rs:111-171` | `CN/Cargo.toml` takes egui's default features ("egui = \"0.36.2\""); see §1.5 F |
| B11 | `Panel` constructors take `impl AsIdSalt` | root changelog (#8592) | `SC/chrome.rs`, `SC/pages/containers.rs` pass `&str` literals, which are `AsIdSalt` (`E37/id_salt.rs:7-9`): compiles unchanged (measured) |
| B12 | `Event::MouseWheel` gains a required field `source: MouseWheelSource` | `E37/data/input/event.rs:171`, `:194-195`; `E37/data/input/mouse_wheel_source.rs` | five events in `SC/tests.rs` (`the_area_stays_inside_the_page`, `the_tables_keep_their_columns_at_the_largest_text_scale` twice, `a_narrow_colour_cell_is_cut_short_and_whole_on_hover`, `the_page_tabs_scroll_the_open_tab_into_view`): compile errors; not in the 2026-11-15 reading |

Searched and not used by the workspace: `AsId`, `UiBuilder::id`, `ViewportCommand::Screenshot`
(the showcases capture through `tools/showcase-support`'s `xcap`), `global_theme_preference_switch`,
`radio_buttons`, the emath `flip*` renames, `Clipboard::new`, the removed epaint `Font` borrow bag
and `preload_characters`, `area_rect`, `ComboBox::from_label`, `show_index`. The eframe `App` trait
is unchanged; egui's `Plugin` trait gains an `on_exit` with a default body (`E37/plugin.rs:28`).
The `Fonts` methods `SC` calls (`definitions`, `row_height`, `layout_job`, `layout_no_wrap`) exist
in 0.37.0 (`P37/text/fonts.rs:336`, `:385`, `:418`, `:460`).

### 1.4 Behaviour changes that reach the workspace

| # | Change | Upstream | What it touches |
|---|---|---|---|
| C1 | egui's defaults: `button_padding` 4 × 1 → 8 × 2, `interact_size.y` 18 → 20; the widget corner radii 2–3 → 4 and 6, the dark buttons' fills and text greys | root changelog (#8617, #8445); `E36/style.rs:1458`, `:1460`; `E37/style.rs:1568`, `:1570`, `:1790-1880` | where the connector leaves a field at egui's value: `button_padding` on the sides a theme leaves unstated, and `interact_size.y` in the menu, list and toolbar cells where the theme leaves the row height unstated (§1.5 D) |
| C2 | A check box's corner radius is capped at `round(0.3 × icon_width)` "to keep it from looking like a radio button"; 0.36.2 drew the state's radius as it was | `E37/theme/default_style.rs:194-197`; `E36/widget_style.rs:183` | the «checkbox» cell (§1.5 C) |
| C3 | A `TextEdit` is styled by `TextEditStyle`: its inner margin is `spacing.button_padding` where 0.36.2 had a fixed `Margin::symmetric(4, 2)`; a margin handed to `TextEdit::margin` loses the stroke's width; its min height is `interact_size.y` | `E36/widgets/text_edit/builder.rs:136`; `E37/theme/default_style.rs:114-166`; `E37/widgets/text_edit/builder.rs:693-704` | `CN/src/accessors.rs` ("const TEXT_EDIT_MARGIN: egui::Margin = egui::Margin::symmetric(4, 2);", `input_margin`, `input_frame`, `text_area_frame`), their tests in `CN/src/install_tests.rs`, `SC/pages/inputs.rs`; with C1, a plain field on 11 presets (§1.5 D, E) |
| C4 | A `CollapsingHeader`'s accessibility role is `DisclosureTriangle`; 0.36.2 gave it `Button` | `E36/response.rs:939`; `E37/containers/collapsing_header.rs:589` | the widgets crate's expander (B2), any test that finds a header by `Role::Button`: `SC/tests.rs`'s `WIDGET_ROLES` holds `Role::Button` but not `DisclosureTriangle`, so `every_widget_reports_itself` would stop checking headers and expanders without a failure (E3, B2) |
| C5 | `egui_kittest` panics by default on a missing glyph (`MissingGlyphPolicy::Panic`) and on an input widget without an accessible name, whenever a run settles | `egui_kittest-0.37.0/src/builder.rs:50-51`; `egui_kittest-0.37.0/src/lib.rs:517-521`; `egui_kittest-0.37.0/src/accessibility.rs` | the harnesses in `CN/src/icons.rs`, `CN/src/install_tests.rs` and `SC/tests.rs`. The glyph policy is set when the harness is built, so it fires during `run_steps` too, and the harness gets no system fonts (C6 is eframe's, not the harness's). Measured: 13 showcase tests and `icons::tests::custom_icon_to_image_loads_a_role` panic on a missing emoji glyph ("📆" ×7, "🌙" ×5, "⚠️" ×1), which masks what else those tests would find; 82 showcase tests fail the accessible-name check, 80 of them naming seven unnamed inputs of the same kinds (a `ComboBox`, three `TextInput`s, a `MultilineTextInput`, a `Slider`, a `SpinButton`), two naming two `ComboBox`es (which pages they are on was not traced) |
| C6 | eframe's default features add `system_fonts` and `clipboard`; `NativeOptions::system_font_fallback` (default `true`) loads system fonts for glyphs the installed faces lack | `eframe-0.37.0/Cargo.toml:61-72`; `eframe-0.37.0/src/epi.rs:383-391`, `:496`; `eframe-0.37.0/src/native/epi_integration.rs:260-265` | the showcase's window (`SC/main.rs`, `eframe::run_native`) |
| C7 | `set_fonts` and `add_font` apply at once while no text has been laid out in the pass | root changelog (#8602, #8609) | `CN/src/atlas.rs`, `install`'s doc: "`Context::set_fonts` takes effect at the next pass" is now true only after text was laid out |
| C8 | A submenu's arrow is a painted triangle in the button's text colour, in a square of `spacing.icon_width`, not the "⏵" glyph, which the button's font sized (`E36/containers/menu.rs:347`) | root changelog (#8525); `E37/containers/menu.rs:480-501` | the Overlays page's `SubMenuButton` (`SC/pages/overlays.rs`); its size, E20 |
| C9 | The menu code was rewritten: arrow-key navigation, sub-menus that follow the menu style config, `MenuConfig::submenu_gap` (2.0) | root changelog (#8548, #8572, #8338); `E37/containers/menu.rs:97`, `:106` | the role modifiers fed to `MenuConfig::style` (`CN/src/roles.rs`, "Menu chrome has no variant"); the menu tests in `SC/tests.rs` |
| C10 | `ComboBox` is built from atoms; its arrow is an atom of `icon_width` | root changelog (#8618); `E37/containers/combo_box.rs:391` | `CW/src/combo_box.rs`, which measures egui's drop-down to restate it |
| C11 | The slider's painting moved to `slider_core.rs`; the facts are the same (rail `inactive.bg_fill`, handle `bg_fill`) | `E37/widgets/slider_core.rs:382-441` | `CW/src/tests.rs`, `the_painted_widgets_are_still_needed` (T7, §1.6) |
| C12 | Other upstream fixes the showcase meets: a `Ui`, `Area`, `Popup` or `Tooltip` is named in the accessibility tree (#8607); invisible widgets leave it (#8621); panel and scene clip rects respect the parent (#8562); kinetic scrolling with rubber-banding (#8662, #8663); the scroll bar's interaction rect matches the drawn bar (#8635) | root changelog | `SC/tests.rs` queries by label and role (a new name can make a `get_by_label` match twice) |
| C13 | A panel's resize handle is a new accessibility node, `Role::Splitter` labelled "Resize panel" (egui #8557, per the trial's report); 0.36.2's panel had none | `E37/containers/panel.rs:1189`, `:1202` | `SC/tests.rs`, `every_widget_reports_itself` fails on a node "shows Side panel" (measured); the follow-up trial confirmed the node: role `Splitter`, label "Resize panel", straddling the side panel's edge, and a second one on the Containers page. Not in the 2026-11-15 reading |
| C14 | `Response::on_hover_text` also describes the widget for accessibility | `E37/response.rs:785-787`, `:799` | the description lands on the widget's own node; the follow-up trial showed it is not the cause of the failure below. Not in the 2026-11-15 reading |

Two showcase tests failed for a cause the first trial did not determine. A follow-up trial on
2026-11-16 determined both by running them (its report and patch are in the maintainer's
`scratch-keep/v072-open-trial/`); both are C12's changes meeting the tests, and neither is a
defect of the showcase:

- `the_icon_theme_drop_down_says_on_hover_why_the_system_theme_is_unavailable`: 0.37 gives no
  accessibility node to an invisible widget (`E37/context.rs:655`, `:4957`; #8621), and a newly
  shown area's first pass is invisible in both versions (`E37/containers/area.rs:692`), so the
  tooltip's label reaches the tree one pass later than the test looks. One more
  `harness.run_steps(1)` after the hover passes it (measured).
- `the_innermost_hovered_target_wins`: 0.37 clips a `Scene` to its parent's clip
  (`E37/containers/scene.rs:229`; #8562), so the test's scene button, below the window, can no
  longer be hovered; the test relied on the old leak. Scrolling its column into view first
  passes it (measured).

What the trial counted (measured, 2026-11-16; passed / failed / ignored):

| What | Count |
|---|---|
| Compile-error sites | 31: 22 mechanical, 9 stand-ins (the report's numbered list: stand-ins #5, #11, #12, #14, #17, #22, #23, #24, #25; its #26–29 cover the five `MouseWheel` sites); 36 compiler errors in all. The stand-ins are E15–E19 |
| Deprecation sites (clippy) | 48: `Id::new` 27 (B1's 28th was a compile-error site), `WidgetType` 13, `AtomLayout` 2, `TextEdit::vertical_align` 5, `Ui::id` 1 |
| `CN` lib, all features | 214 / 3 / 3: the missing glyph (C5), "egui bundles NotoEmoji-Regular" in `t6_d_the_chosen_faces_head_the_chains_at_the_asked_weight` (B10, E17), `compat::the_readme_states_the_manifest_floors` (the README's 0.36.2, §1.6) |
| `CN` lib, no default features | 197 / 2 / 1 (the same less the glyph test) |
| `SC` tests, all features | 45 / 90 / 0; with the accessible-name check switched off as a stand-in, 119 / 16 / 0: 13 missing glyph (C5), 1 C13, 2 from C12 (both determined by the follow-up trial, above) |
| `SC`, no default features (stand-in in) | 105 / 15 / 0 (120 tests; `the_panel_icon_loads_from_a_theme_of_either_convention` not among the failures) |
| `SC`, default features (check_release's form) | 112 / 16 / 0 |
| `CW` lib | 56 / 2 / 0: T7 at its first slider assertion (C11), `a_truncated_drop_down_keeps_to_its_room` (an untruncated combo stays 160 points wide, 94 tall; C10, inferred, the wrap rule not traced) |
| Doc tests | `CN` 16 / 0 (all features), 6 / 0 (no default features); `CW` 2 / 0 |

### 1.5 Theme data: where the mapping changes

**A. New `Style` fields.** Only `spacing.scroll.kinetic`: `decay_time` 0.5, `stop_distance` 0.5,
`rubber_band` true (`E37/style.rs:851-884`). No `Spacing`, `Visuals`, `Interaction`,
`WidgetVisuals` or `Widgets` field was added, removed or renamed (field-by-field comparison).
`MenuConfig::submenu_gap` is new but is a menu's own configuration, not a `Style` field. The model
has no leaf for momentum scrolling, overscroll or a submenu's offset (searched in
`docs/property-registry.toml` and `docs/platform-facts.md` for "kinetic", "momentum", "rubber",
"elastic", "overscroll", "submenu": nothing).

**B. Readers the mapping's notes name.** `CN/src/mapping.md` says `compact_menu_style`,
`spacing.menu_spacing` and `spacing.menu_width` have "no reader anywhere in egui 0.36.2", and
`window_highlight_topmost` "no consumer". In 0.37.0 none of the four is read outside `style.rs`
(search of `E37`). `button_frame` was already read in 0.36.2 (`E36/widgets/button.rs:323`); 0.37.0
reads it in `E37/theme/default_style.rs:74-75`, still with the default `true`.

**C. The check box's radius (C2).** The connector writes `checkbox.border.corner_radius` into the
«checkbox» cell's `widgets.*.corner_radius` and `checkbox.indicator_width` into its
`spacing.icon_width` (`CN/src/mapping.md`, the rows of the two leaves). Only material-3 states a
check-box radius (2); every other preset inherits `defaults.border.corner_radius`
(`docs/inheritance-rules.toml`, "corner_radius.inherits_from"). Computed from the presets' TOML at
a text scale of 1, not through the resolver:

| Preset | Radius (inherited) | Indicator | 0.37's cap | Drawn |
|---|---:|---:|---:|---:|
| adwaita | 9 | 20 | 6 | 6 |
| the ten community presets (catppuccin ×4, dracula, gruvbox, nord, one-dark, solarized, tokyo-night) | 8 | 18 | 5 | 5 |
| ios-13 | 10 | 22 | 7 | 7 |
| macos-monterey | 5 | 14 | 4 | 4 |
| macos-tahoe | 6 | 16 | 5 | 5 |
| kde-breeze, windows-11, material-3 | 5, 4, 2 | 20, 20, 18 | 6, 6, 5 | as stated |

So on 14 of 17 presets 0.37 draws a smaller check-box radius than 0.7.1 does; only kde-breeze,
windows-11 and material-3 stay under the cap. In those 14 the radius is inherited, not stated by
the preset: `docs/platform-facts.md`'s check-box table, the `border.corner_radius` row, gives
"← `defaults.border.corner_radius`" for macOS, Windows, KDE and GNOME.

That row is not what the platforms' own sources say (read on 2026-11-16; the repository already
records two of them):

| Platform | The check box's own radius, by its source | The preset's inherited value | 0.37's cap |
|---|---|---:|---:|
| GNOME | 6 on a box of 20: `$check_radius: 6px` and `check { border-radius: $check_radius; }`, where 9 is `$button_radius` (libadwaita 1.10.0, `src/stylesheet/widgets/_checks.scss:1`, `:23-26`, `:67-68`; `_common.scss:8`). `docs/todo.md`'s open GNOME entry has it as item (4), "not the inherited 9", and the gap map as GN-1.60 | 9 | 6 |
| KDE | 4: `CheckBox_Radius = Frame_FrameRadius - 1`, on a box drawn 2 inside the 20 cell (Breeze v6.7.5, `kstyle/breezemetrics.h:87-90`, `kstyle/breezehelper.cpp:844-863`; the gap map's K1.53) | 5 | 6 |
| Windows | 4, the control radius itself: the `CheckBox` style sets `CornerRadius` to `ControlCornerRadius` (microsoft-ui-xaml `8463f451`, `CheckBox_themeresources.xaml:294`, `CornerRadius_themeresources.xaml:5`) | 4 | 6 |
| macOS | not known: the probe's check-box renders exist, and no fact states the box's radius | 5 and 6 | 4 and 5 |

So GNOME's check box is 6, exactly what egui's cap draws on adwaita, and the 9 that the gpui and
iced connectors draw there today is the preset's defect (both clamp only at half the box: adwaita
9 on 20, the community presets 8 on 18, close to a circle). With adwaita corrected, the cap binds
on 13 presets, none of which has a source that says its check box takes the general control
radius: the two macOS presets (by one point, the platform's own value unknown), ios-13, and the
ten community presets, which have no platform behind them.

**D. egui's moved defaults (C1).** The connector writes every widget state's `corner_radius`,
`weak_bg_fill` and `fg_stroke.color`, and the base style's `interact_size.y` from
`button.min_height`, which every preset states (`CN/src/mapping.md`, "Base owners"). So those
defaults never reach the base style. Two paths let egui's moved values into themed styles.

Row heights. `CN/src/style/roles.rs`, `fn row_height`, writes a cell's `interact_size.y` from
`menu.row_height`, `list.row_height` and, when `bar_height` is unstated, `toolbar.bar_height`; an
unstated one takes `input.scheme.default_style().spacing.interact_size.y`, egui's own value, which
moves from 18 to 20 (C1). From the presets' TOML (both variants; adwaita, dracula, macos-monterey
and windows-11 re-read for this revision) it reaches the menu cell on 14 presets (all but adwaita,
macos-monterey and material-3), the list cell on 14 (all but macos-monterey, macos-tahoe and
windows-11) and the toolbar cell on 14 (all but macos-monterey, material-3 and windows-11): 16 of
17 presets in all, only macos-monterey untouched. Reasons in `docs/showcase-exceptions.toml`
already name an unstated `menu.row_height_px` or `list.row_height_px` as theme silence.

Padding. `spacing.button_padding` is written per side only where the theme states the side
(v0.5.9's per-side `Option` padding). An unstated side keeps egui's value on the base style and the
base style's value in a role cell (`CN/src/convert.rs`, `to_button_padding`'s documentation;
`CN/src/style/roles.rs`, "to_button_padding(base.spacing.button_padding"). So:

- A menu's padding does not change: a menu cell starts from `menu_style`'s 2 × 0
  (`E37/containers/menu.rs:26`, the same in 0.36.2). Its row height does (above).
- The base style's `button_padding` is egui's on every side `button.border.padding` leaves
  unstated, and it becomes 8 × 2. From the presets' TOML, 11 presets state no padding anywhere
  (the ten community presets and ios-13); the other six (adwaita, kde-breeze, macos-monterey,
  macos-tahoe, material-3, windows-11) state `button.border.padding` on all four sides. On the 11,
  egui's 8 × 2 reaches every cell but the menu: the button, tab, combo_box, segmented_control,
  expander and list cells; buttons drawn in the toolbar, sidebar, status-bar and dialog scopes; and
  every plain `TextEdit`, the «input» cell's included (a multi-line one is drawn there too,
  `text_area` being no role of its own, `CN/src/roles.rs`), which goes from 0.36.2's fixed 4 × 2 to
  8 × 2 (C3). These presets state no `input.border.padding` either, so E6's sink writes nothing
  there.
- The review (finding F3) also lists cell sides other presets leave unstated: the expander on
  kde-breeze, macos-monterey, macos-tahoe and material-3; the segmented control on kde-breeze and
  macos-tahoe, material-3's vertical side and macos-monterey's horizontal one; the combo box's
  right side on macos-monterey and macos-tahoe; and (read here) macos-tahoe's tabs and material-3's
  tabs' vertical side. Those sides take the preset's own button padding through the base style,
  so egui's move does not reach them. F3 counted 15 of 17 presets; this revision counts 11, from
  the two code paths named above (read, not measured; the specification confirms it by a test).

Both counts come from the preset files without the resolver, so they do not cover values from the
live OS readers or text scales other than 1.

`docs/showcase-exceptions.toml` names the old value as the reason for parity groups ("egui's
button_padding 4 x 1, egui-0.36.2/src/style.rs:1458"). The `fg_stroke` widths the mapping leaves to
egui ("egui's own value stands") did not change.

**E. The text field (C3).** Today a plain `TextEdit` pads with egui's 4 × 2; the connector's
`input_margin` and `input_frame` give it `input.border.padding` plus the border's width, because
0.36.2 painted the stroke inside the margin. In 0.37 a plain `TextEdit` pads with
`spacing.button_padding`, which the base style takes from `button.border.padding` and which no
«input» scope rewrites (the scopes that do are «button», «combo_box», «expander», «menu»,
«segmented_control», «tab»: `CN/src/mapping.md`, the `spacing.button_padding.x` row). So a field
drawn without the accessor would be padded like a button. And a margin handed to `TextEdit::margin`
now loses the stroke's width inside egui, so `input_margin`'s added border width may count twice.
How the 0.37 frame places text against its stroke was not worked out here
(`E37/containers/frame.rs:313-323`, `apply_stroke_and_expansion_without_layout_shift`); the
specification derives it.

**F. Fonts (B10, C6, C7).** The font plan's base is `egui::FontDefinitions::default()`
(`CN/src/fonts.rs`, "pub fn with_base"; `CN/src/atlas.rs`, "egui's `Ubuntu-Light`"). In 0.37 that is
`FontDefinitions::empty()` unless egui's `default_fonts` feature is on (`P37/text/font_definitions.rs:111-117`),
and its emoji faces need `monochrome_emoji_fonts` (`:146-171`). eframe's default features turn on
`default_fonts` (`eframe-0.37.0/Cargo.toml:64`, `:73`), so an eframe application with default
features, the showcase and the connector's own tests (whose dev-dependency `eframe` takes its
defaults) keep Ubuntu-Light and Hack, but lose the two emoji faces. `native-theme-egui-widgets`
has no `eframe` dev-dependency and is tested alone (`scripts/check_release.sh` and the CI matrix):
with egui's default of no features, `FontDefinitions::default()` is empty there, so its tests draw
text with no face (finding F7, read). The trial's widgets tests showed no failure from it (56 of
58 passed, the two failures from C10 and C11); what those tests draw was not inspected further.
The connector's statements that
the base "keeps egui's emoji fallbacks" (`CN/src/fonts.rs`, "whose emoji fallbacks it keeps")
become false. `CN/src/fonts.rs`'s statement "epaint's only constructors are `FontData::from_static`
… and `FontData::from_owned`" is false in 0.37 (`from_blob`, `P37/text/font_data.rs:40`); it is the
reason the connector copies a shared buffer (`FontBytes::Shared`, "bytes.to_vec()").

**G. What does not change.** The egui fields T1b reasons from hold in 0.37.0:
`extra_text_line_spacing` 0.0, `interact_size.x` 40.0, `disabled_alpha` 0.5 (`E37/style.rs:1575`,
`:1570`, `:1670`); `icon_width_inner` is still 8.0 (`:1577`). A button's frame comes out as 0.36.2's
(fill `weak_bg_fill`, the stroke and expansion inside the padding: `E36/widget_style.rs:146-170`,
`E37/theme/default_style.rs:33-110`, `E37/containers/frame.rs:313-323`). Text still takes
`visuals.override_text_color` where the connector writes it (`E37/widget_text.rs:501-510`,
`:825-834`). The focus-ring scopes use `Ui::unique_id`, and `WidgetRect`s are still keyed by it
(`E37/ui.rs:165-170`).

### 1.6 Tests and gates that read egui

- `CN/src/tripwires.rs`: T1 fails to compile at B9, as designed (measured); T1b's three defaults
  hold (§1.5 G).
- `CN/src/style_diff.rs`, `fn scroll`, fails to compile at B9 too (measured); its paths are
  normative for `CN/mapping.toml` (E18).
- `CW/src/tests.rs`, `the_painted_widgets_are_still_needed` (T7), reads egui's sources through
  `cargo metadata` and asserts quoted lines. Checked against 0.37.0: the spinner, radio button,
  collapsing header and progress bar assertions hold; the three slider assertions fail (the code
  moved to `slider_core.rs` with its facts unchanged, C11, but "let visuals =
  ui.style().interact(response);" is now an argument of the handle call, `E37/widgets/slider.rs:571`,
  so they are rewritten, not moved); two of the three combo-box assertions fail ("let icon_size =
  Vec2::splat(ui.spacing().icon_width);", "let actual_height = galley.size().y.max(icon_size.y);";
  C10). Measured: T7 stops at its first slider assertion; the later ones were not reached.
- `CN/src/compat.rs` reads `docs/COMPATIBILITY.toml` and the manifest's floors for egui, skrifa,
  eframe, egui_extras and egui_kittest; `scripts/update_compatibility.sh` writes the stamp.
  Measured: `compat::the_readme_states_the_manifest_floors` fails until the README states 0.37.0
  for the four crates.
- `scripts/check_widget_coverage.py` discovers egui's widgets in egui's own source: types with
  `impl Widget for`, and types with a `show`, `show_*`, `ui` or `body` method under `src/containers/`
  or in `src/grid.rs` (`egui_widgets` there). Run by the GPUI Kit 0.10.0 trial on the joint state
  (§1.1; measured, 2026-11-16, its `coverage.txt`) it exits 1, for egui alone (gpui 0 missing):
  egui discovered 37, shown 34, excepted 0, missing 3, namely `Aligned`, `RangeSlider` and
  `WidgetAtom` (the showcase names the deprecated `AtomLayout`); `egui_ui` 69 of 69 and
  `egui_extras` 6 of 6 shown. `CompletionPopup` is not discovered: it lives under
  `src/widgets/text_edit/` and has a `show` but no `impl Widget`. E10 settles the three.
- `scripts/check_readme_versions.py` and the README's "Verified against" line; the
  provenance and compatibility stamps hash `Cargo.lock`, so every showcase's stamp goes stale.
- One gate reads an egui line by number: the gpui connector's `CITED_CRATES` and `NAMED_ONLY`
  (`connectors/native-theme-gpui/src/showcase.rs`) name egui, and `every_prose_citation_still_exists`
  checks the citation "egui/widgets/spinner.rs:49" in
  `connectors/native-theme-gpui/examples/showcase-gpui/info/feedback.rs` (its `not_themeable`
  note on the spinner's arc) against the egui `cargo metadata` resolves. That line is the same text
  in 0.37.0 (read), so the gate needs no edit (not run). No other such gate was found (searched
  `scripts/`, `tools/`, `CN`, `CW`).

### 1.7 Dependents, MSRV and the lock

- The crates that move: native-theme-egui (dependency `egui`; dev-dependencies `eframe`,
  `egui_extras`, `egui_kittest`) and native-theme-egui-widgets, which reaches egui through the
  connector's re-export (`CW/Cargo.toml`, "the graph holds one copy of each"). No other workspace
  crate names egui: `tools/showcase-support` only says so in its description; the iced connector
  shares the `muda` version with the egui showcase ("version the egui showcase takes", in
`connectors/native-theme-iced/Cargo.toml`).
- MSRV: egui 0.37's `rust-version` is 1.95, as 0.36.2's, so both egui crates keep
  `rust-version = "1.95"` and the workspace keeps 1.88.0. The CI `msrv` job checks the two egui
  crates at 1.95.0 (`ci.yml`, "dtolnay/rust-toolchain@1.95.0"). That every crate in the new graph
  builds at 1.95 is not verified; that job decides it. serde-saphyr 1.3.0 (`rust-version` 1.89)
  enters the GPUI graph (below); the gpui connector declares `rust-version = "1.95.0"`, so its
  floor does not move (§1.2; in 0.7.1's `Cargo.lock` serde-saphyr is in native-theme-gpui's graph
  only).
- skrifa stays at 0.44.0 and winit at 0.30.13: epaint's and egui-winit's requirements are
  unchanged. In the joint lock swash's skrifa moved to 0.37.0 while the egui crates keep 0.44.0
  (§1.2). muda is not eframe's dependency in 0.36.2 or 0.37.0 (no `muda` in either manifest), so
  `docs/todo.md`'s "skrifa … and muda … follow it: their requirements are epaint's and eframe's
  own" is true of skrifa only, and skrifa does not move. muda 0.20.0 and 0.21.2 both ask objc2
  0.6.1 and objc2-app-kit 0.3.0, which unify with egui-winit 0.37's 0.6.4 and 0.3.2.
- The lock (measured, §1.2): after the four manifest edits, one `cargo update -p egui --precise
  0.37.0` exited 0 with no resolution error, moved the whole egui family, and nothing had to be
  moved by hand; `cargo metadata --locked` passed. Beside the egui family and its new
  dependencies (`fontique` 0.11.1 opens fontconfig at run time on Linux, feature
  `fontconfig-dlopen`) it moved three things the GPUI graph uses. `smallvec` 1.15.2 → 1.16.5, since
  egui needs 1.16 and serde-saphyr 0.0.29 caps it below 1.16; the chain is gpui-component 0.9.1 →
  rust-i18n 4.2.1 → rust-i18n-support → serde-saphyr (finding F1), so rust-i18n-support moved 4.2.1
  → 4.2.4 (it asks `serde-saphyr = "1.2"`) and serde-saphyr 0.0.29 → 1.3.0, with granit-parser
  0.0.7 → 1.3.0. F1 predicted that rust-i18n and rust-i18n-macro move to 4.2.4 too; they stay at
  4.2.1. And `unicode-properties` 0.1.3 → 0.1.5 (epaint 0.37 asks 0.1.4; gpui-pre-web 0.3.12 accepts
  it, "0.1.3" being a caret requirement; gpui-fast-web 0.1.15 no longer pins it). Read from 0.7.1's
  `Cargo.lock`: `unicode-properties` is reached by the connectors only (egui's through egui_extras'
  resvg, usvg and rustybuzz; iced's through iced_tiny_skia's; gpui's through gpui-fast-web), not by
  native-theme; `smallvec` reaches every leg, native-theme's included (through ashpd and
  wayland-backend), so native-theme's Rust 1.88.0 MSRV leg builds smallvec 1.16.5, which declares
  no `rust-version` (not checked, §5).
- On that lock `cargo check -p native-theme-gpui --all-targets --features gpui-fast,native,watch`
  (check_release's feature set, not `--all-features`) passed with no warnings, GPUI Kit 0.9.1 and
  the moved crates compiled (measured). The gpui and iced tests were not run on that lock (the
  GPUI Kit 0.10.0 trial ran the gpui tests on the joint lock, below).
- The release's lock: v0.7.2 also takes GPUI Kit 0.10.0, which has its own rationale
  (`docs/todo_v0.7.2_gpui-kit-0.10.0-rationale.md`). The lock that ships is the
  two moves together. This trial measured the egui move alone, on 0.7.1's GPUI Kit 0.9.1; the GPUI
  Kit 0.10.0 trial then ran on top of it (§1.1): one `cargo update -p gpui-component --precise
  0.10.0` resolved the joint lock, `cargo metadata --locked` and `cargo check --workspace
  --all-targets` exited 0 on it, the latter with warnings only outside gpui (the egui deprecations
  and `tools/os-probe`'s unused `toml`). That trial did not run the MSRV job, macOS or Windows, or
  the full gate. This document decides the egui move only.
- Semver: the connector re-exports egui (`CN/src/lib.rs`, "pub use egui;"), so a 0.7.2 on egui
  0.37 is a breaking change for an application that names egui 0.36 beside `native-theme-egui =
  "0.7"`: `cargo update` would take 0.7.2, and such an application stays on egui 0.36 only with
  `native-theme-egui = "=0.7.1"`. v0.7.1 moved the gpui connector from GPUI Kit 0.8.1 to 0.9.1 in
  the same way, in a patch release (`CHANGELOG.md`, 0.7.1, "requires gpui-component, gpui-base and
  gpui-kit 0.9.1"). The release order, with this move as v0.7.2, was approved by the maintainer on
  2026-11-16, so the breaking change in a patch release follows from it, as it did for v0.7.1
  (§4).
- Later lanes: the KDE lane plans' P2 accepted set lists, of the egui family, "egui and egui-winit
  0.36.2 or 0.37.x" (`docs/todo_v0.7.x_kde-open-items-steps2-3-plan.md`, P2's row). Whether that
  set also has to name the other lock moves of §1.2 is checked when that plan gets its last check,
  before its release is built.

### 1.8 Citations

`CN`, `CW`, `CN/mapping.toml`, `docs/showcase-exceptions.toml` and `docs/todo.md` cite about 760
egui-family lines, almost all without a version; the crates' documentation says "Target: **egui
0.36.2**" (`CN/src/lib.rs`, `CW/src/lib.rs`), which is how they are read. Each cited line's text was
looked up in 0.37.0 by script (single lines and the first line of ranges): 177 are the same text at
the same line, about 405 are the same text at another line, 124 cite text no longer in the 0.37.0
file, and 46 cite files 0.37.0 no longer has (`egui/src/widget_style.rs` 26, now
`widget_style/mod.rs` and `theme/default_style.rs`; `epaint/src/text/font.rs` 17, split; 
`egui/src/atomics/atom_layout.rs` 3). The counts include repeats. Read at 0.36.2, all of them stay
true. The review's own script counted 826 (193 same line, 452 moved, 131 text gone, 50 file gone;
`widget_style.rs` 30): the counts depend on the script, both are approximate, and under E11
neither is acted on.

### 1.9 What 0.37 adds that could draw (for E10)

Checked on 2026-11-16 by comparing 0.36.2 with 0.37.0 in the registry, for egui and egui_extras:
the file lists, the `pub struct`/`enum`/`trait` names, the `impl Widget for` types, the new `pub fn`s
of every file under `widgets/` and `containers/`, and the fields of `style.rs`.

| Addition | 0.37.0 | What it draws, from which values | In the showcase today |
|---|---|---|---|
| `RangeSlider` | `E37/widgets/range_slider.rs:34`, `:675` | the slider's rail (`widgets.inactive` fill and radius), the fill between the handles (`selection.bg_fill`), two handles (the state's `bg_fill`, `fg_stroke`, `expansion`), through `E37/widgets/slider_core.rs:382-441` | no |
| `CompletionPopup` | `E37/widgets/text_edit/completion.rs:169`, `:240` | a `Popup` under a field (`Frame::popup`, `E37/containers/frame.rs:214`) holding `Button::selectable` rows (selection fill, the widget states) | no |
| `WidgetAtom` (`AtomLayout` renamed) | `E37/atomics/widget_atom.rs:26`, `:252` | its frame and atoms, as `AtomLayout` did | yes, under the old name |
| `ContainerAtom` | `E37/atomics/container_atom.rs:523` (`paint_at`) | the layout and frame `WidgetAtom` wraps, nothing beyond it | through `WidgetAtom` |
| `Aligned` | `E37/containers/aligned.rs:40` | nothing of its own: it places its contents | no |
| `CollapsingHeader::custom` | `E37/containers/collapsing_header.rs:436` | `CollapsingState::show_header`'s header, which it wraps | yes (Containers page, "CollapsingState with a custom header") |
| The submenu arrow (`arrow_shape`, `arrow_atom`) | `E37/containers/menu.rs:481`, `:492` | a painted triangle in the button's text colour (C8), 0.55 × 0.35 of a square `Vec2::splat(ui.spacing().icon_width)` (`:484-487`, `:495`); the menu cell writes no `icon_width`, so the square is the base style's, `checkbox.indicator_width` (`CN/src/mapping.md`, the `spacing.icon_width` row; E20) | drawn by the Overlays page's `SubMenuButton`; no Widget Info line |
| `MenuConfig::submenu_gap` | `E37/containers/menu.rs:141` | the gap between a menu and its sub-menu, 2.0 | drawn; no theme value (E4) |
| Kinetic scrolling, rubber band, `ScrollArea::overscroll` | `E37/style.rs:851-884`; `E37/containers/scroll_area.rs:142` | content moved past its end and springing back: behaviour | recorded (E4, E18) |
| `Slider::drag_value` (`DragValueSettings`), `DragValue::min_size`, `clip_text` | `E37/widgets/slider.rs:271`; `E37/widgets/drag_value.rs:111`, `:390` | options of widgets already shown; no new style value | the widgets, yes |
| `accessible_name` on `Area`, `Popup`, `Tooltip`; `Area::role` | `E37/containers/area.rs:202`, `:210` | nothing (accessibility) | not drawn |
| `egui_extras::SvgGlyph` (feature `svg`) | `egui_extras-0.37.0/src/svg_glyph.rs:33` | a text glyph from an SVG, tinted with the text colour; no style value of its own | no |
| `egui_extras` `TokenType`, now public | `egui_extras-0.37.0/src/syntax_highlighting.rs:136` | the code view's token kinds, drawn by `code_view_ui` | yes |
| The `egui-icons` face (B10) | `P37/text/font_definitions.rs:139-140` | egui's special-emoji glyphs, in the text colour | in egui's own text |

`Style`, `Spacing`, `Visuals` and their structs gain only `spacing.scroll.kinetic` (§1.5 A). The
other new public types (`ClassName`, `StyleArgs`, `TextEditStyle`, `AtomLayoutStyle`,
`DefaultStyle` and `Themes` (public only under `experimental`, B3), `MouseWheelSource`,
`ScreenshotCallback`, `ValueFormat`, `WheelScroll`, `AxisPhysics`, `CompletionQuery`,
`CompletionOutput`, `Suggestion`, the sized atom types) draw nothing themselves; `SliderCore` and `SliderGeometry` are in the private `slider_core` module.
`CN/src/roles.rs`'s rule, "exactly one variant per container chrome in egui 0.36.2 to which an
application can attach an [`egui::Frame`]", holds for 0.37.0: no new container takes a frame
(`Aligned` has none, `CompletionPopup` no frame setter, and no new `frame` method appears under
`containers/`), so the rule is re-stated for 0.37.0 with no new variant.

## 2. Decisions

### E1 — Which 0.37.x

- (a) `"0.37.0"`, a caret requirement, `>=0.37.0, <0.38.0`.
- (b) Wait for a 0.37.1.
- (c) An exact `=0.37.0`.

**(a).** 0.37.0 is the only 0.37 release on 2026-11-15 (§1.1). Waiting has no date and keeps egui
0.37 applications out. An exact pin rejects patch fixes; the README already argues this for 0.36
("An exact `=0.36.2` pin would reject patch fixes"). If a 0.37.1 appears before the plan starts, its
changelog is read and the floor is decided again in the specification.

### E2 — What moves together

- (a) egui, eframe, egui_extras and egui_kittest to 0.37.0 together; skrifa, winit, muda and jiff
  stay.
- (b) egui alone.
- (c) (a) plus muda 0.21 and the skrifa the outdated check names (0.50).

**(a).** eframe, egui_extras and egui_kittest 0.37.0 each require egui 0.37.0 (§1.2), so (b) would
put two eguis in the showcase's build, and the showcase hands one to the other. skrifa must stay
at epaint's own `0.44.0`, or the build holds two skrifas, which the connector's manifest rules out
("one `skrifa` for both"); (c) breaks that. muda and jiff are not tied to egui 0.37
(§1.7). The todo entry's sentence on skrifa and muda is corrected with the release's backlog edits.

### E3 — How the breaking changes are met

- (a) Take the new names and APIs everywhere (B1–B12).
- (b) Keep the deprecated ones under `#[expect(deprecated)]` where they still exist.

**(a).** The deprecated names are due to go in a later egui; (b) only moves the work there, and
B2's three variants, B3, B4's `IdSalt`, B8 and B12 have no deprecated form anyway. Per row:

- B1: `Id::unique` where the code needs a root `Id` (stored ids, `Context` data keys), since it
  hashes as `Id::new` did and the values stay; `IdSalt::new` where egui now asks a salt (B4's
  atom), as `Id::new`'s deprecation note says (`E37/id.rs:101-105`). The doc comments and the
  README example follow.
- B2: `egui::Role`. For the switch, `Role::CheckBox` stays the `WidgetInfo` role and the node's
  role is still set to `Switch` afterwards, as today. The expander takes `Role::DisclosureTriangle`,
  egui's own for a collapsing header (C4), so its node matches egui's header; that role is added
  to `SC/tests.rs`'s `WIDGET_ROLES`, without which `every_widget_reports_itself` would stop
  checking headers and expanders and still pass. The Basic page's list row: E16.
- B3: the switch and the Basic page: E15. `Classes` comes from `egui::class`.
- B4: `WidgetAtom`; the custom atom keyed by `IdSalt::new` from the same string; the fallback text
  colour set on the allocated atom's `container` (it compiles through `DerefMut` either way).
- B5: `clamped_offset()`: the Basic page draws a scroll bar's thumb inside its groove, and the
  rubber band (C12) can move the unclamped offset past the ends. (The trial's `unclamped_offset()`
  was a stand-in.)
- B6, B7: `scope_id()`; `align(Align2)` with the same vertical centring.
- B8: read the bytes with `FontData::bytes()` (the trial's change, which compiled). A face from a
  shared buffer is handed over with `FontData::from_blob` instead of `from_owned(bytes.to_vec())`:
  the copy existed only because epaint had no other constructor (§1.5 F), and the comment that
  says so is false in 0.37. `FontBytes::Shared` holds an `Arc<[u8]>`, which does not convert to a
  `Blob` (`Arc<dyn AsRef<[u8]> + Send + Sync>`, `P37/text/font_data.rs:10`) directly, since `[u8]`
  is unsized; `Arc::new(Arc::clone(bytes))` does, as `Arc<[u8]>` is `AsRef<[u8]>`: one small
  allocation for the outer `Arc` and no copy of the bytes (read, not compiled). The alternative,
  keeping the copy, keeps a cost whose stated reason is gone. The tests' `&'static` helper: E17.
- B9: `style_diff`: E18; T1: E19, its field counts recounted (`ScrollStyle` 17), the comments'
  existing upstream citations left as they are (E11).
- C12 and C13, the three showcase tests the follow-up trial explained (§1.4): one more step after
  the hover; the scene's column scrolled into view before the hover; and in
  `every_widget_reports_itself` a node of egui's own that no record contains is accepted where its
  centre lies in a record, which in the trial let through the two `Splitter` handles and nothing
  else. Each is a change to the test, not to the showcase.
- B12: the five test events name a source. The trial's `MouseWheelSource::Unknown` ("Set this to
  [`MouseWheelSource::Unknown`] if unknown", `E37/data/input/event.rs:194`) passed all five tests;
  the specification may name `Wheel`, the device they simulate, and runs the same five.
- T7 (§1.6): the slider's assertions are rewritten against `slider.rs` and `slider_core.rs` with
  the same facts. The combo box's two are re-derived from 0.37's atom layout; if the fact behind
  the drop-down's per-instance change no longer holds, the specification decides whether the
  widgets crate's drop-down is still needed, as T7's rule says ("a failure demotes the widget").
  `a_truncated_drop_down_keeps_to_its_room` (measured failure, C10) belongs to the same step.

### E4 — The new scroll fields

- (a) `[unwritten]` rows in `CN/mapping.toml` with the reason "input behaviour, no native leaf",
  like `always_scroll_the_only_direction`.
- (b) Write `decay_time` 0 and `rubber_band` false under reduced motion, as the connector already
  writes `scroll_animation`.
- (c) A new model leaf for momentum scrolling.

**(a).** No platform fact in `docs/platform-facts.md` says what any platform does with momentum or
overscroll, under reduced motion or otherwise (§1.5 A), so (b) would be an invented rule, and (c)
needs facts that nobody has gathered and a model change, which is not this release's. Both the
rubber band and `MenuConfig::submenu_gap` are visible, so each gets a backlog entry for the facts
(the open question for (b) and (c) included). `MenuConfig::submenu_gap` gets no row (it is no
`Style` field); egui's 2.0 stands, and the README says so. How the three new fields reach
`style_diff` and `CN/mapping.toml`: E18.

### E5 — egui's moved defaults on what a theme leaves unstated

- (a) egui 0.37's values stand there, as egui's did before; every statement and parity reason that
  names 0.36.2's value is rewritten from 0.37.0, and the parity groups are regenerated from the
  asset run.
- (b) Write 0.36.2's numbers (4 × 1, 18 and so on) into the base style and the row cells to keep
  the look.

**(a).** The connector hard-codes no theme values, and 4 × 1 and 18 are no platform's values
either: they were egui's. Theme silence has always meant egui's own value (§1.5 D). The look
changes in two ways, which the CHANGELOG says: the padding goes from 4 × 1 to 8 × 2 on the 11
presets that state none (the ten community presets and ios-13; the review counted 15, and §1.5 D
says why this revision counts 11); and the row floor of menus, lists and toolbars where the theme
states no row height goes from 18 to 20 on 16 presets (all but macos-monterey: the ten community
presets, material-3, and five platform presets, adwaita, ios-13, kde-breeze, macos-tahoe and
windows-11). A text field outside any input scope changes too (E6). The parity reasons are
re-derived from the presets in the unit, not edited by swapping a number: several in `docs/showcase-exceptions.toml` say that material-3 states
no `button.border.padding_*`, while `material-3.toml` states 24 and 10.

### E6 — The text field's padding

- (a) The «input» cell writes `spacing.button_padding` from `input.border.padding`, so a plain
  single-line `TextEdit` in that scope is padded as the platform's field; and `input_margin`,
  `input_frame` and `text_area_frame` are re-derived from 0.37's `TextEditStyle` (the margin's stroke
  handling, the frame's focus stroke, the min height), with the constant `TEXT_EDIT_MARGIN`
  removed, since egui has no fixed margin any more.
- (b) Re-derive the accessors only; a plain `TextEdit` takes the button's padding.
- (c) Re-derive the accessors only, and document that a plain `TextEdit` needs them.

**(a).** Under (b) and (c) the upgrade puts a button's theme value on every field drawn without the
accessor, those in the «input» scope included: a visible property drawn from the wrong leaf,
which the project's rule makes a defect. (a) adds one sink to one cell. In 0.37.0 `button_padding` is read by the button's and the text
field's styles (`E37/theme/default_style.rs:54`, `:155`) and the collapsing header
(`E37/containers/collapsing_header.rs:549`), and a menu sets its own (`E37/containers/menu.rs:26`);
the specification checks that no button or header drawn inside an «input» scope would then take
the field's padding. The mapping rows of `input.border.padding` move from DERIVED (T18(b)) to
SCOPED.

Two cases (a) does not reach, both decided here:

- A multi-line field. There is no «text_area» cell: `text_area` is no role of its own, and a
  multi-line `TextEdit` is drawn in the «input» scope (`CN/src/roles.rs`), so a plain one there
  takes the input's padding, although presets state a `text_area` padding of their own (adwaita's
  `text_area` 0 × 0 against its `input` 9 × 0; kde-breeze's 5 × 5 against 7 × 6). No new role in
  this release: the connector's existing public `text_area_margin` and `text_area_frame`
  (`CN/src/accessors.rs`) stay the route for a multi-line field, and the CHANGELOG and the README
  say "single-line" (E12).
- A field outside any `Role::Input` scope. It takes the base style's `button_padding`: on the six
  presets that state `button.border.padding` that is the button's padding (24 × 10 on material-3),
  where v0.7.1 drew egui's fixed 4 × 2; on the other 11, egui's 8 × 2 (§1.5 D). This changes the
  look; the CHANGELOG lists it under *Changed* (E12), and it is part of §4's item on egui's values.

Two points the plan's check raised, decided here:

- A `DragValue` in an «input» scope. The showcase scopes its three `DragValue`s that way on
  purpose (`SC/pages/inputs.rs`, "`Role::Input` for every `TextEdit` and `DragValue`"), and egui
  draws a `DragValue` as a button, so under (a) it takes the field's padding. That is the
  intent: it is a number field, and the Basic page already gives it the field's padding by hand.
  Any other button, or a collapsing header, in an «input» scope would take the field's padding
  by mistake; the unit searches for one, and one found is a stop.
- The accessors and the cell. `input_margin` and `text_area_margin` take the theme alone. They
  keep their signatures (the smallest change on the released API, and the connector parity script
  compares them). On a side the theme states they return that side's padding plus the border's
  line width, exactly per side, as in 0.7.1: egui lays the stroke inside what it is handed
  (`E37/containers/frame.rs:319-320`, `E37/theme/default_style.rs:155-158`,
  `E37/widgets/text_edit/builder.rs:696-700`). On a side the theme leaves unstated they returned
  0.36.2's fixed 4 × 2, the constant (a) removes; they now return the «input» cell's component
  for that side (the base style's: the button's, else egui's own default read from egui, never a
  number written in the connector), and an unstated `text_area` side takes `input_margin`'s.
  The cell itself is written as the other cells are, by `to_button_padding`, and egui's
  `button_padding` is one value per axis: a plain field therefore takes the mean of an axis's two
  stated sides. Only windows-11 states sides that differ on an axis, so only there a plain field
  and one built with the accessors differ; the accessors stay the exact route, and the README
  says so. A test holds this on the 17 presets: each accessor's unstated sides equal the cell's
  component, and each stated pair's mean equals it. The reason on `input_margin` in
  `scripts/connector_parity_exceptions.toml`, which states 0.36.2's stroke inside the margin, is
  rewritten with the unit's result.

### E7 — The check box's radius cap

- (a) Follow egui's cap. The mapping records it as an egui limit above `round(0.3 × icon_width)`.
- (b) Register a `StyleProvider<CheckboxStyle>` that keeps the theme's radius: needs the
  `experimental` feature, an unstable API, for every application.
- (c) A check box in the widgets crate that draws the theme's radius, painted, as no `Style`
  value lifts the cap (`E37/theme/default_style.rs:194-197`): a new public widget with its tests,
  showcase row, Widget Info and a mixed state, about one working day by analogy with the switch
  (`CW/src/switch.rs`, 428 lines plus tests).
- (d) Ask upstream for the cap to be optional (a draft for the maintainer, not sent).
- (e) Correct the value the cap is measured against: state the check box's own radius where the
  platform's source gives one (§1.5 C).

**(e) for adwaita, and (a) for what remains; (c) and (d) wait for facts.** The earlier form of
this decision was (c), on the premise that the inherited radius is the platforms' own. The
sources say otherwise (§1.5 C): the one platform preset far above the cap, adwaita, is above it
because the preset inherits the button's 9 where libadwaita's check box has 6, which is what the
cap draws. So:

- `native-theme/src/presets/adwaita.toml` states `checkbox.border.corner_radius` 6, and
  `docs/platform-facts.md`'s check-box row gives GNOME's own value with its source. That is a
  defect of the facts, recorded as open in `docs/todo.md` (the GNOME entry's item (4)), and its
  fix corrects the gpui and iced showcases too, which draw 9 today. It is a unit of its own in
  this release, small, because E7's outcome on adwaita depends on it; the asset run redraws the
  three showcases anyway.
- On the other 13 presets egui's cap stands and is recorded: in `CN/src/mapping.md` as an egui
  limit, with the capped values of §1.5 C's first table, in the check box's Widget Info, and in
  the CHANGELOG. None of the 13 has a source for the uncapped value: on the two macOS presets the
  difference is one point against an inherited value nobody has checked, and on ios-13 and the
  ten community presets the inherited radius makes a check box of 18 with a radius of 8, which
  the gpui and iced showcases draw as nearly a circle.
- (c) would spend a day to draw exactly those unverified values, and (b) puts an unstable feature
  on every application. Both are refused for this release. The backlog gets what would decide
  them: macOS's check-box radius, measured from the probe's renders; what a preset without a
  check-box radius of its own should give a check box (the rule in
  `docs/inheritance-rules.toml`, which three of the four desktops' sources contradict); KDE's 4
  with its box drawn inside the cell (K1.53), which is under the cap either way and belongs with
  the KDE items; and (d)'s draft. If that work leaves a preset whose established radius is above
  egui's cap, (c) is built then.

Not verified: whether the parity checker needs a reason for egui's capped check box against the
gpui and iced showcases on the 13 presets; the unit that regenerates the parity groups finds out.

### E8 — Fonts

- (a) The connector does not force egui features. Its documentation says the base is
  `FontDefinitions::default()` under the application's egui features, and that egui 0.37 puts its
  emoji faces behind `monochrome_emoji_fonts`.
- (b) The connector's `egui` dependency turns on `default_fonts`.
- (c) It turns on `default_fonts` and `monochrome_emoji_fonts`, which keeps 0.7.1's base chain
  exactly.

**(a).** The manifest stays as it is, and egui's own choice (#8583) reaches the application as
upstream meant. An eframe application with default features still gets Ubuntu-Light and Hack
(§1.5 F). (b) forces a feature an application cannot turn off again. (c) also puts the monochrome
emoji face before eframe's system font provider, which is asked only for glyphs the installed
faces lack, so it would hide 0.37's colour emoji from every application that uses the connector.
The fonts' statements in `CN/src/fonts.rs`, `CN/src/atlas.rs` (C7 included) and the README are
corrected. Under (a) the widgets crate tested alone has an empty `FontDefinitions::default()`
(§1.5 F, F7); its tests passed in the trial apart from C10 and C11, so none of them was seen to
need a face, but what they draw was not inspected. If the specification finds one that needs a
face, the widgets crate's own dev-dependency turns on `default_fonts` for its tests, which leaves
the published graph as (a) has it.

### E9 — The showcase's font fallback and the test harness's checks

- (a) The showcase keeps eframe's default `system_font_fallback` (on); the tests keep egui_kittest's
  defaults (missing glyphs and unnamed inputs fail), and what fails is fixed: an accessible name
  where an input has none, and `allow_missing_glyphs` only where a test draws a missing glyph on
  purpose, with its reason.
- (b) Fallback off for identical captures on every machine; both test checks off.

**(a).** A native toolkit falls back through the system's fonts too, so the showcase with the
fallback is closer to the platform. The two checks find real defects: an unnamed input is one for a
screen reader, and a missing glyph draws tofu. The cost, measured (§1.4): seven unnamed inputs
behind 80 of the 82 accessible-name failures and two `ComboBox`es behind the other two; 13
showcase tests and one connector test drawing an emoji the harness has no face for, which hides
whatever else those tests would find.

The missing glyphs are drawn by upstream widgets, not by the project: "📆" by egui_extras' date
picker button (`egui_extras-0.37.0/src/datepicker/button.rs:139`, which the Inputs page's
`DatePickerButton` shows), "🌙" by egui_extras' `CodeTheme::ui` ("🌙 Dark theme",
`egui_extras-0.37.0/src/syntax_highlighting.rs:470`, the Text page's "CodeTheme editor"), and
"⚠️" by egui's `Image` for an image that failed to load (`E37/widgets/image.rs:699`; which
showcase test draws it was not traced). None of the three is in the showcase's, the connector's or
the widgets crate's own sources (searched, the escapes `\u{1f4c6}`, `\u{1f319}` and `\u{26a0}`
included: no match). So a rule about `allow_missing_glyphs`, which is for glyphs a test draws on
purpose, does not cover them; and turning on `monochrome_emoji_fonts` in a dev-dependency would
change `FontDefinitions::default()` for every test of the package, against E8 (a) and E17.
Decided: the showcase tests' harness installs the two monochrome emoji faces from the
`epaint_default_fonts` constants (E17 (iii)) as last fallbacks, in test setup only. The real
window gets such glyphs from eframe's system font fallback (C6), which the test harness lacks.

The connector's test `icons::tests::custom_icon_to_image_loads_a_role` panics on "⚠\u{fe0f}"
(the trial's log): it builds a bare `Harness::new_ui`, which installs no image loaders, so the
image never loaded there, under 0.36.2 either; what changed is the mark egui draws for it ("⚠️"
with a variation selector, `E37/widgets/image.rs:699`, where 0.36.2 drew "⚠") and kittest's panic
on a missing glyph. Decided, from the follow-up trial: the test installs egui_extras' image
loaders in its harness, so the image loads and no mark is drawn (measured: the test passes).

### E10 — The showcase

Not a choice. The maintainer said on 2026-11-16, about a
toolkit upgrade (GPUI Kit 0.10.0), that support "is not only being able to use it without errors.
It means, any new widgets, graphical elements are themed with our crate, and the showcase includes
the new elements." This document reads that rule as applying to egui 0.37 as well. With the earlier upgrades' test, "an *element* is
shown; a behaviour or option with no theme value is not, and is recorded"
(`docs/archive/todo_v0.7.0_gpui-kit-0.8.0-rationale.md`, K12's first paragraph), §1.9's list
gives:

- Shown, each with Widget Info: `RangeSlider` beside the slider (it paints its handles through
  egui's `slider_core::paint_handle`, so it has the slider's resting-knob limitation: Widget Info
  says so, and a backlog entry asks for a range counterpart in the widgets crate);
  `CompletionPopup` on the Inputs page; `WidgetAtom`, the Buttons page's "AtomLayout" section
  renamed; the submenu's painted arrow (C8), a Widget Info line on the Overlays page naming its
  colour's source (the button's text colour) and its size's source (E20).
- Excepted in `docs/showcase-exceptions.toml`: `Aligned`, which places its contents and draws
  nothing of its own.
- Recorded in the specification, not shown: `ContainerAtom` (drawn as `WidgetAtom` is),
  `CollapsingHeader::custom` (the Containers page's `CollapsingState` header already shows what it
  draws), `MenuConfig::submenu_gap` and the rubber band (E4), the new `Slider` and `DragValue`
  options, `accessible_name` and `Area::role`, `SvgGlyph` (no style value of its own), `TokenType`,
  the `egui-icons` face.

The coverage gate's three names (§1.6, measured) are met: `RangeSlider` and `WidgetAtom` shown by
name, `Aligned` excepted. `CompletionPopup` is outside the gate's discovery rule (§1.6) and is
shown under the maintainer's rule, not the gate's. E10 rests on reading the maintainer's sentence
about GPUI Kit as applying to egui, as the earlier GPUI Kit upgrades showed their new elements;
§4 states that reading for the maintainer's information. Were it not to apply, `RangeSlider`,
`CompletionPopup`, `WidgetAtom` and the submenu arrow would be excepted with reasons instead of
shown; the coverage gate still needs each of its names shown or excepted.

### E11 — The cited egui lines

- (a) Re-point nothing, convert nothing, sweep nothing: neither the lines whose text or file is
  gone nor those that moved (§1.8). One sentence in four places: next to "Target: **egui
  0.37.0**" in `CN/src/lib.rs` and in `CW/src/lib.rs`, in `CN/mapping.toml`'s header, and in
  `docs/showcase-exceptions.toml`'s header (for its egui citations only): "an egui-family line cited
  without a version, written before 0.7.2, is egui 0.36.2's". A statement whose content 0.37
  changes (§1.3–§1.5) is rewritten in the unit that changes that code. A new citation names the
  0.37.0 file and a symbol, or a pinned `0.37.0` line.
- (b) Re-point every citation to 0.37.0, as v0.7.1 did for GPUI Kit (its rationale's N6).
- (c) Convert them to a file and a quoted phrase.

**(a).** S6 (2026-11-15, after the GPUI Kit pass) keeps old citations as they are and accepts
line numbers of pinned sources; (a) pins the unversioned ones by one sentence and leaves the
roughly 580 that still describe 0.36.2 truly. (b) is the largest single item of the release, about
580 lines with about 170 by hand, and the one gate that reads an egui line by number needs no
edit (§1.6). (c) is what S6 refused. `docs/todo.md`'s egui citations (19) are left to drift (S6).
`docs/showcase-exceptions.toml` says its citations are "at the version Cargo.lock holds"; that
sentence gives way to the one above. The parity checker's `--merge` keeps existing reasons, so
only the groups the asset run adds carry 0.37.0 citations.

### E12 — CHANGELOG, README, version and assets

- CHANGELOG, a 0.7.2 entry. *Changed*: native-theme-egui and native-theme-egui-widgets require
  egui, eframe, egui_extras and egui_kittest 0.37.0 (MSRV unchanged, 1.95.0), in the form of
  0.7.1's GPUI Kit line, with one sentence that an application on egui 0.36 stays there with
  `native-theme-egui = "=0.7.1"`; egui's check box draws its radius capped on 13 presets (E7);
  egui's own padding and row heights where a theme states none (E5); a text field outside the input scope
  takes the button's padding where the theme states one, egui's 8 × 2 elsewhere (E6); the submenu
  arrow drawn at the check box's indicator width (E20); the fonts' base chain (E8). *Fixed*: a
  plain single-line text field in the input scope takes the theme's field padding, one value per
  axis (where a theme states different sides, as windows-11 does, their mean; the accessors give
  the exact sides) (E6);
  adwaita's check box has libadwaita's own radius, 6, in every connector (E7).
  *Added*: the showcase's new sections (E10). No migration section (the project's pre-1.0 rule).
- READMEs: the Compatibility table and its caret paragraph (the floor's reason rewritten for
  0.37.0), "Verified against" from `scripts/update_compatibility.sh`, the fonts' base chain
  (E8), the submenu gap (E4), and the role-styles sentence "A `TextEdit` in a `Role::Input` scope
  takes the", which gains the padding of a single-line field (E6);
  `CN/src/lib.rs` and `CW/src/lib.rs` "Target: **egui 0.37.0**"; `ROADMAP.md`'s "Targets egui
  0.36.2"; the dependency canary's comment on the caret requirement.
- `docs/todo.md`: the 2026-11-14 entry's egui item closed, its skrifa and muda sentence corrected;
  the GNOME entry's item (4) closed (E7); new entries for E4 (the rubber band, `submenu_gap`),
  E7's facts tasks, its inheritance question and its upstream draft, E10's
  range counterpart in the widgets crate, and E20's step: the platform facts for a submenu arrow's
  size and a model leaf for it.
- The workspace version is 0.7.2 for all crates (one workspace version). The lock change stales
  every showcase's stamp, so one asset run covers the three showcases; the CHANGELOG's date follows
  it. The version, the asset run and the close-out run once, after both moves of v0.7.2 (this one
  and GPUI Kit 0.10.0), and belong to the release's plan. The two moves touch different crates and
  can be built in two worktrees in parallel, each merged whole, since the coverage gate is red for
  egui until E10 lands.

### E13 — Other upgrades

- (a) None: zbus 6, resvg, fontdb, windows-version and the ten lock-only updates stay out.
- (b) The ten lock-only updates join, since the lock changes anyway.

**(a).** One small release at a time. zbus 6 needs Rust 1.89 and a port of the GNOME watcher
(`docs/todo.md`, the zbus entry), and windows-version needs Rust 1.95 for native-theme; the
lock-only updates move the iced and GPUI graphs, and each is a review and gate of its own. Only
the moves egui 0.37 needs enter the lock (§1.7): the trial showed that one `cargo update -p egui
--precise 0.37.0` moves all that egui 0.37 needs (§1.2), the F1 chain (rust-i18n-support,
serde-saphyr, granit-parser, smallvec) included, and nothing has to be moved by hand.

### E14 — Where the specification's inventory comes from

- (a) From reading, as the 2026-11-15 draft (search plus both versions' sources).
- (b) Before the specification, a measuring build on a throwaway branch: the manifests and lock
  moved, the egui crates checked, clippy and their tests run once, every error and failure
  recorded; nothing of it merges.

**(b)**, approved by the maintainer on 2026-11-16 and run that day (§1.1): about 16 minutes in
all, about 356 s of it cargo (measured). Reading had found the names; the build found what reading
could not: B12, C13 and C14, the lock moves of §1.2 that no one had named, and the counts of
§1.4. The specification's inventory is that error list and those failures, with what the trial
did not obtain (§5).

### E15 — How the switch and the Basic page get a widget's style (the trial's stand-in (a))

The switch (`CW/src/switch.rs`, `switch_ui`) reads only `style.frame` of `checkbox_style`, and
passes it to `.frame(..)`; that frame is `Frame::new()` in 0.36.2 (`E36/widget_style.rs:177`) and in
0.37.0 (`E37/theme/default_style.rs:182`), and `Frame::new()` is `Frame::NONE`, "also the
default" (`E37/containers/frame.rs:160-175`), the frame `ContainerAtom::new` already sets
(`E37/atomics/container_atom.rs:88`). The Basic page (`SC/pages/basic.rs`, `button_label` and
`checkbox_parts`) reads the button's font and frame margin and the check box's frame margin,
`checkbox_size` and `check_size`, to place egui's own widgets' parts for the layout dump and Widget
Info. 0.37's replacement is `Ui::widget_style` and `Context::get_widget_style`, which are
`#[doc(hidden)]` without `experimental` (`E37/theme/mod.rs:25`, `E37/context.rs:2484`); the
`theme` module that holds the `StyleProvider` system is private without it (`E37/lib.rs:424-427`);
`StyleArgs`, `ButtonStyle`, `CheckboxStyle` and `class::Classes` are public and documented
(`E37/lib.rs:434`, `:441`). The `experimental` feature changes only that visibility (those lines
and the `cfg_attr`s of `theme/mod.rs`, `context.rs`, `widget_style/mod.rs`).

Where the showcase ships: `CN/Cargo.toml` has no `include` or `exclude`, so by Cargo's default
`examples/showcase-egui` is in the published `.crate`, where it cannot build, since `cargo package`
strips its path dev-dependencies (`native-theme-showcase-support`, `native-theme-egui-widgets`; the
manifest's comments say so). Read from the manifests; `cargo package --list` was not run.

- (i) Call the doc-hidden API in both: what egui's own provider computes, so what egui draws; an
  unstable, undocumented API in the published crate.
- (ii) Turn on `experimental`: on the connector's `egui` dependency every application gets it; as
  the showcase's dev-dependency it is the same API as (i), declared.
- (iii) Restate the values from `Style`'s public fields (`button_padding`, `TextStyle::Body`,
  `icon_width`, `icon_width_inner`): no hidden API, but a copy of the provider, including the
  integer rounding `apply_stroke_and_expansion_without_layout_shift` applies to the margin
  (`E37/containers/frame.rs:313-323`), that can drift from egui as T7 shows its copies do.
- (iv) In the published crate, drop the read: the switch drops `.frame(style.frame)` and the
  style call with the state lookup before it.

**(iv) for the switch; (i) for the Basic page.** The switch drops the frame read: the project's
rule keeps doc-hidden upstream API out of a published crate, and the switch loses nothing, its
frame being empty in both versions. T7 gains an assertion on the check-box provider's
`frame: Frame::new()` line (`E37/theme/default_style.rs:182`), so that the switch notices if a
later egui gives that frame a value. The showcase's Basic page calls the doc-hidden
`get_widget_style`: the Basic page must report what egui draws, which only egui's provider
computes. The rule against doc-hidden upstream items protects those who build the published
crate; the example builds only in this repository, on the committed lock (in the `.crate` it
lacks its path dev-dependencies, above), and a removal upstream is a compile error at the next
upgrade, where it is fixed. Restating from `Style`'s public fields (iii) would be sound only with
source assertions of T7's kind on the provider's lines; (ii) gives (i)'s API under a declared
feature that would reach the connector's test builds through feature unification.

### E16 — The accessibility role of the Basic page's list row (stand-in (b))

`SC/pages/basic.rs`, `list_row`, reported `WidgetType::SelectableLabel` with its selected state.
0.36.2 already turned that type into `Role::Button` (`E36/response.rs:939`), and 0.37's own
selectable button reports `Role::Button` (`E37/widgets/button.rs:384`; the table at
`E37/lib.rs:653`); a `selected` state becomes `Toggled` (`E37/response.rs:1034-1039`).

- (i) `Role::Button` with the selected state: the node stays what 0.7.1 produces (the trial's
  change, which compiled; the list tests did not fail).
- (ii) `Role::ListBoxOption` with accesskit's selected state, inside a `ListBox` node: closer to a
  list's meaning, but a new accessibility design for the showcase's list, with its tests, which no
  upstream change asks for (not checked against the platforms' toolkits).

`SC/tests.rs`'s `WIDGET_ROLES` holds `Role::Button` and not `ListBoxOption`, so under (i)
`every_widget_reports_itself` keeps checking the row, and (ii) would also need that role added.

**(i).** The upgrade keeps the node as it is, which is the smallest change on the released
behaviour; (ii) is a change of its own.

### E17 — The tests' `&'static` bundled faces (stand-in (c))

`CN/src/install_tests.rs`, `t6_fonts::bundled`, takes a bundled face's bytes as `&'static [u8]`
from the `Cow::Borrowed` of `FontDefinitions::default()`, for `FontBytes::Static` and
`FontData::from_static`; `t13_accessibility::t5_line_spacing_and_slider_expansion_at_factor_two`
does the same for Hack. 0.37's `Blob` hides that the bytes are static (`P37/text/font_data.rs:10`,
`:31-33`), and two of the faces (`NotoEmoji-Regular`, `emoji-icon-font`) are in the default
definitions only under `monochrome_emoji_fonts` (B10). The trial copied the bytes and leaked the
copy with `Box::leak` on every call.

- (i) The trial's leak: a leak in the code, however small, which the project's rules exclude.
- (ii) One `static LazyLock<FontDefinitions>`: the definitions live as long as the program, so
  `FontData::bytes()` on them gives `&'static` bytes, with no copy and no leak (read, not
  compiled); but the two emoji faces are still absent from the definitions the test reads.
- (iii) A dev-dependency `epaint_default_fonts = "0.37.0"`, epaint's own requirement, so one copy
  in the graph, with its `monochrome_emoji_fonts` feature: its `HACK_REGULAR`, `NOTO_EMOJI_REGULAR`,
  `UBUNTU_LIGHT` and `EMOJI_ICON` are `&'static [u8]` constants of the very bytes egui
  registers (`epaint_default_fonts-0.37.0/src/lib.rs:20`, `:34`, `:43`, `:57`). That feature is the
  font crate's, not epaint's, so `FontDefinitions::default()` in the tests stays E8's base
  (epaint tests its own feature, `P37/text/font_definitions.rs:10`). Licence "(MIT OR Apache-2.0)
  AND OFL-1.1 AND Ubuntu-font-1.0". The crate is already in the dev graph, through eframe's
  default `default_fonts` (`eframe-0.37.0/Cargo.toml:64`, `:73`) and egui's and epaint's
  `default_fonts` features.
- (iv) Hand the tests' faces over as `FontBytes::Shared`: no `&'static` needed, but the tests
  then no longer exercise the `Static` path they were written for.

**(iii).** No leak, no copy, no `unwrap` on a face that may be missing (the project's rules on
leaks and panic-prone code), and the tests keep testing what they test. (ii) has no leak and no
copy either, but only (iii) gives the two emoji faces, which need the font crate's
`monochrome_emoji_fonts` feature. `t6_d`'s four faces need not be egui's chain members, only
distinct static faces, which the constants are. The specification checks whether
`CN/src/compat.rs` and the README's Compatibility table must list the new egui-family
dev-dependency.

### E18 — `ScrollStyle::kinetic` in `style_diff` and `CN/mapping.toml` (stand-in (d))

`CN/src/style_diff.rs` is "the exhaustive-destructuring diff of spec §13.1 — its path spelling is
normative for `mapping.toml`", and `CN/mapping.toml`'s list of unwritten paths gives a reason for
"every other `Style` path `style_diff` emits" that no row writes (T10b). The trial wrote
`kinetic: _`.

- (i) Map the three fields from a theme value: there is no leaf (§1.5 A, E4 (c)).
- (ii) Write a stated constant, such as `decay_time` 0 under reduced motion: no platform fact
  says so (E4 (b)), an invented value.
- (iii) Leave egui's values, recorded: `fn scroll` walks `kinetic` into three paths,
  `spacing.scroll.kinetic.decay_time`, `.stop_distance` and `.rubber_band`, and the unwritten list
  gives each E4's reason, so T10b checks that no row writes them and a later write must add a row.
- (iv) The trial's `kinetic: _`: compiles, but the three paths leave the normative list, so T10b
  cannot see them and a later write goes unchecked.

**(iii).** It is E4 (a) carried into the diff that checks it; (i) has no source and (ii) would
invent a value (the project's rule against invented values).

### E19 — `kinetic` in the tripwire (stand-in (e))

T1 (`CN/src/tripwires.rs`) destructures with no `..`, and "every type reachable from a destructured
one is destructured separately below". The trial wrote `kinetic: _`.

- (i) `kinetic: _`: compiles, but a field egui later adds to `KineticScrollStyle` passes silently.
- (ii) Bind `kinetic` and destructure `KineticScrollStyle { decay_time: _, stop_distance: _,
  rubber_band: _ }` below, with its field count, 3, and its 0.37.0 lines
  (`E37/style.rs:851-875`); `ScrollStyle`'s count becomes 17.

**(ii).** T1's own rule; it is what this document said for B9 before the trial.

### E20 — The submenu arrow's size

0.37's submenu arrow (C8) is an atom of `Vec2::splat(ui.spacing().icon_width)`
(`E37/containers/menu.rs:492-505`); 0.36.2's "⏵" was sized by the button's font. The menu cell
writes no `icon_width` (`CN/src/style/roles.rs`, `fn menu`), so the arrow takes the base style's,
`checkbox.indicator_width` (`CN/src/mapping.md`, the `spacing.icon_width` row). The model has a
`menu.icon_size` leaf, which `CN/src/mapping.md` gives as DERIVED, "→ T18(a)": the atlas reports it
to the application, and no `Style` field takes it. Four presets state it (kde-breeze,
macos-monterey, macos-tahoe, windows-11: 16); elsewhere it inherits `defaults.icon_sizes.toolbar`
(`native-theme/src/model/widgets/mod.rs`, the `inherit_from` of the menu's `icon_size`): 24 on
twelve presets (the ten community ones, ios-13, material-3) and 16 on adwaita. No platform fact
gives a submenu arrow's own size (§1.5 A's search for "submenu" found nothing), and no theme datum
states it under any option below.

- (a) The menu cell writes `menu.icon_size` into its `spacing.icon_width`, as the «combo_box» cell
  writes `combo_box.arrow_icon_size` into its own (the same `mapping.md` row). Every arrow egui
  draws in a themed menu follows it, `Ui::menu_button` inside a menu included (it builds
  `SubMenuButton::new` itself, `E37/ui.rs:2928`). It also resizes what else reads `icon_width` in
  the «menu» scope; in 0.37.0 that is the check box (its box and, through C2's cap, its radius,
  `E37/theme/default_style.rs:190-197`), the radio button (`E37/widgets/radio_button.rs:52-58`), a
  combo box's arrow (`E37/containers/combo_box.rs:391`) and a `CollapsingState`'s indented toggle
  (`E37/containers/collapsing_header.rs:93`). A check box or radio button placed directly in a menu
  would be sized by the menu's icon size, not `checkbox.indicator_width`, unless it sits in a
  «checkbox» scope of its own (every cell starts from the base style, `CN/src/style/roles.rs`,
  `role_cell`). The arrow atom allocates the whole `icon_width` square
  (`E37/containers/menu.rs:495-503`), so a submenu row is at least 24 tall on the twelve presets where `menu.icon_size` is 24; on
  eleven of them (the ten community ones and ios-13) the menu's other rows take egui's 20, as they
  state no `menu.row_height` (§1.5 D), while material-3 states 48. Cost: one sink,
  `menu.icon_size`'s rows from DERIVED to SCOPED, a test, and the parity reasons for menus.
- (b) Record the arrow as a reader of `spacing.icon_width`, so of the check box's indicator width,
  a known limit, with a backlog step. No sink now; on every preset where the two leaves differ the
  arrow is drawn at the check box's size.
- (c) Size the arrow per instance. 0.37's API allows it: `SubMenuButton::from_button` with
  `Button::right_text(SubMenuButton::arrow_atom(None).atom_size(..))` (`from_button`'s
  documentation, `E37/containers/menu.rs:514-517`; `AtomExt::atom_size`, `E37/atomics/atom_ext.rs:25`,
  which the atom's size then follows, `E37/atomics/atom.rs:190-192`; its intrinsic size stays at
  least `icon_width`, `:197`, whose effect on the row was not traced). Only code that builds the
  button itself gets it: the showcase, or an application through a new connector helper.
  `Ui::menu_button` and `menu_image_button` inside a menu keep `icon_width` (`E37/ui.rs:2928`,
  `:2962`).

**(b).** No theme datum states the submenu arrow's size under any option: `menu.icon_size` is the
size of a menu item's icon, mostly inherited from the toolbar's, not of the arrow. So the rule that
a connector follows the theme's data does not select (a), and (a) would cost what it moves: a
submenu row at least 24 tall beside 20-point rows on eleven presets, and every check box or radio
button in a menu sized 16 or 24 instead of `checkbox.indicator_width`. (c) reaches buttons built
by hand only. Under (b) the arrow is recorded as a reader of `spacing.icon_width` (so of
`checkbox.indicator_width`) in `CN/src/mapping.md` and in the Overlays page's Widget Info line, the
CHANGELOG says so (E12), and the backlog gets a step of its own: the platform facts for a submenu
arrow's size and a model leaf for it. The maintainer is told of this limit (§4).

## 3. Cost and risk

**Estimate (not measured): about six to nine working days of implementation time**, plus reviews
and the close-out, in nine units: the manifests, lock and renames to a green build (0.5–1 day);
the fonts (0.5); the widgets crate's switch, expander, drop-down and T7 (1); adwaita's check-box
radius in the facts and the preset (0.25, E7); the mapping's changes, E4–E7 and E20's
record with their rows, tests and `mapping.md` (1–1.5); the tests that fail (0.5–2); the
showcase's new sections and Widget Info (1); documentation (E11's four sentences), CHANGELOG and
stamps (1); the close-out with one asset run (desktop about 5 minutes) and the release (0.5),
which runs once for both moves and belongs to the release's plan (E12). The other day figures are
the 2026-11-15 estimates; no count the trial measured plainly contradicts one, so none
was changed. E14's build, which the estimate did not have, is done (about 16 minutes, §1.1).

The release's size: this move's about six to nine days and the GPUI Kit move's about five to seven
are both estimates. The plan carries the cut: if one move meets its stop rule, the other ships
alone as v0.7.2.

What the trial measured against the units (§1.3, §1.4): 31 compile-error sites, 22 mechanical and
9 stand-ins (36 compiler errors); 48 deprecation sites; after the accessible-name stand-in, 16
failing showcase tests (13 behind a missing emoji glyph, which hides what else they would find),
3 connector and 2 widgets-crate tests; 82 showcase tests on the accessible-name check, from seven
kinds of unnamed input.

What could make it larger:

- What the 13 glyph panics hide: those tests run on only once their glyphs are fixed, and they
  assert layouts and egui defaults; C1 moves the padding on the 11 presets that state none and the
  menu, list and toolbar row heights on 16 (§1.5 D).
- The menu rewrite (C9) and the atom-built `ComboBox` (C10) behind the connector's menu modifiers
  and the widgets crate's drop-down: if T7's re-derivation shows the drop-down's reason gone or
  changed, that widget is redesigned; `a_truncated_drop_down_keeps_to_its_room` already fails
  (C10, inferred).
- E6's frame arithmetic, if 0.37's `TextEdit` cannot be given the platform's padding inside the
  border by margin or frame alone.
- The parity checker's groups: every egui difference the moved defaults bring needs a reason
  (E5), and egui's capped check box may need one on 13 presets (E7, not verified).
- The widgets crate's tests run with no font face (F7, §1.5 F): no failure measured, not
  inspected.
- The lock's moves reach the GPUI graph (the F1 chain), every leg (`smallvec`) and the connectors'
  legs (`unicode-properties`, §1.7): the gpui connector's check passed, and on the joint lock the
  GPUI Kit 0.10.0 trial ran its tests, finding no failure it could not trace to its own move (that trial's
  report, "Changed drawing or behaviour: none. Cause not determined: none."); the iced legs and
  the MSRV job were not run.
- What the GPUI Kit 0.10.0 trial left undone on the joint lock: the MSRV job, macOS and Windows,
  the full gate (§1.7).

**Stop and re-scope:** at the first unit, if a compile error or failing test has a cause that
§1.3–§1.5 does not list, or if a unit takes twice its estimate.

## 4. What is decided, and what is the maintainer's

Every decision above is made from the sources, the trial and the project's rules; none is left as
a question. What changes for an application is listed here so that it is approved knowingly:

1. E7: adwaita's check box gets libadwaita's own radius, 6, in the facts and the preset, which
   every connector then draws (the gpui and iced showcases draw 9 today). On 13 other presets
   egui 0.37 caps the check box's radius below the inherited value, and that is recorded, not
   worked around: none of those values has a source. No new check-box widget is built.
2. E5 and E6: what a theme leaves unstated takes egui 0.37's values, so the look changes: the
   padding goes from 4 × 1 to 8 × 2 on the 11 presets that state none, and the menu, list and
   toolbar row floor from 18 to 20 on 16 presets; a text field outside any input scope takes the
   button's padding on the six presets that state one (24 × 10 on material-3) where v0.7.1 drew
   egui's 4 × 2. A single-line field in the input scope takes the theme's field padding, one
   value per axis; the accessors give the exact sides.
3. E8: the connector forces no egui font feature; egui's emoji faces become the application's
   choice.
4. E12 and §1.7: v0.7.2 requires egui 0.37, by the release order the maintainer approved on
   2026-11-16; an application on egui 0.36 stays there only with `native-theme-egui = "=0.7.1"`,
   and the CHANGELOG says so.
5. E10: egui 0.37's new elements are shown, as the GPUI Kit upgrades showed theirs, and the
   coverage gate needs each new widget shown or excepted. Applying the maintainer's GPUI Kit
   sentence of 2026-11-16 to egui is this document's reading.
6. E15: the showcase's Basic page calls the doc-hidden `get_widget_style`; the rule against
   doc-hidden items protects those who build the published crate, and the example builds only
   here. The published switch does not call it.
7. E20: the submenu arrow is drawn at the check box's indicator width, a recorded limit, until
   platform facts and a model leaf for its size exist (backlog).
8. E11: old egui citations are labelled as 0.36.2's by one sentence in four places, not
   re-pointed (S6).

The maintainer's: the approval of this document, its specification and its plan; sending the
upstream draft (E7 (d)); everything that publishes.

## 5. What was checked, and what was not

Read on 2026-11-15: every row of §1.2–§1.8, the T7 assertions against 0.37.0's files, the
deprecations new in 0.37.0, the preset values of §1.5 C and D.

Measured on 2026-11-16 by the trial (§1.1), on Linux: the lock's resolution and every move
(§1.2, §1.7); the gpui connector's `cargo check` with check_release's features, green; the 31
compile-error sites and the clippy deprecations (§1.3, §1.4); the tests of `CN`, `SC` and `CW`
with all, no and default features (§1.4); `cargo check --workspace --all-targets`, exit 0 with
only the egui deprecations and a manifest warning "unused dependency `toml`" in `tools/os-probe`,
a crate the trial did not touch (very likely there before the move; no build of `e866ea6c`
verified it).

Measured on 2026-11-16 by the GPUI Kit 0.10.0 trial on the joint lock (§1.1, its report): the
lock's resolution and `cargo metadata --locked`; `cargo check --workspace --all-targets`, exit 0;
the coverage gate (§1.6).

Read on 2026-11-16 for this revision: the code at the five stand-ins (E15–E19), the padding and
row-height paths of §1.5 D, the gpui gate's egui citation (§1.6), §1.9's comparison, the submenu
arrow's sizing (E20), the visibility of egui's theme API (B3), `CN/Cargo.toml`'s packaging (E15)
and the gpui connector's `rust-version` (§1.7); later that day, the lock's reach of swash,
serde-saphyr, smallvec and unicode-properties (§1.2, §1.7), the presets' `menu.icon_size` (E20)
and text-field paddings (E6), the missing glyphs' upstream sources (E9), and the check box's own
radius in libadwaita's, Breeze's and WinUI's sources (§1.5 C, E7).

Not checked or not obtained:

- The showcase's test count with no default features before the accessible-name stand-in.
- T7 after its first assertion.
- The §1.6 script gates other than the coverage gate (`check_readme_versions.py`, the stamps).
- The `msrv` job at Rust 1.95, on either lock; native-theme's Rust 1.88.0 MSRV leg with smallvec
  1.16.5, which declares no `rust-version` (§1.7).
- check_release's "one skrifa" check on the joint lock (by the lock, swash does not reach
  native-theme-egui, §1.2).
- macOS and Windows; the gpui connector with `--all-features`; its tests on the egui-only lock
  (the GPUI Kit 0.10.0 trial ran them on the joint lock); the iced legs.
- What the 13 glyph panics hide; the drop-down test's cause, inferred (C10).
- Which showcase test draws "⚠️" (E9).
- Whether the widgets crate's tests draw text with no face (F7): no failure, not inspected.
- §1.5 D's count of 11 presets, read in the code against the review's 15, and its row-height
  counts (16 of 17), read from the presets; neither measured.
- §1.5 C's and D's preset values were read from the TOML files and the inheritance rule, not
  through the resolver, and at a text scale of 1.
- §1.5 E: how 0.37's frame places the text against the stroke.
- Whether the parity checker needs groups for egui's capped check box (E7); macOS's check-box
  radius; KStyle's part in Breeze's indicator rectangle (the 20 cell was taken from Breeze's
  metric and Qt's common style).
- §1.8's counts are of citation tokens, repeats included, compared line by line; a "same text"
  match can be a common line such as a closing brace.
- On the joint lock: the MSRV job, macOS and Windows, the full gate (the GPUI Kit 0.10.0 trial's
  report, "Not done, not checked").
- E20 (c)'s per-instance arrow: read, not built; the effect of the atom's intrinsic size on the
  row not traced.

## 6. Out of scope

- The GPUI Kit 0.10.0 move of the same release (its own rationale, §0).
- The zbus 6 move and the other upgrades of `docs/todo.md`'s "Dependencies" entries (E13).
- The lanes' releases; their VF takes the version forms from this document and from the code its
  plan builds.
- egui 0.37 features with no theme value that the connector could use: the `StyleProvider` theme
  system under `experimental`, colour emoji, the PRIMARY selection, recorded test videos.
