# v0.7.1 → 0.8.0 — The module inventory (the switch's input)

Every module, integration test and `lib.rs` item of `native-theme` with one fate (the plan, *The module inventory*),
drafted 2026-10-30 from the call sites, its open points decided by the orchestrator (*Decisions*, below). Fates are
at M1a's last task, the switch, unless a row says otherwise. "Reader part" is the code that goes with the reader at a
split and comes back at M1b from the switch's parent commit. Callers are `file:line` at commit 8483901f.

## native-theme/src

| Item | Fate | Milestone | Callers / reason |
|---|---|---|---|
| `color.rs` | ported beside, then split | M1a | S2.1's `Color` replaces `Rgba`. Deleted: `Rgba`, its `FromStr` and serde code, `ParseColorError`, the hex helpers. Stay: `composite_over` (S16); `unpremultiply_alpha` (rasterize.rs:100, sficons.rs:132, winicons.rs:174); `tint_monochrome` (sficons.rs:133, winicons.rs:525) |
| `detect.rs` | split | M1a, restored M1b | Stay: `LinuxDesktop`, `detect_linux_desktop`, `parse_linux_desktop`, `xdg_current_desktop`, `refine_linux_desktop`, `kdeglobals_exists`, `SUBPROCESS_TIMEOUT`, `run_with_timeout`/`read_on_a_thread`/`poll_intervals`/`POLL_INTERVAL_CAP`, `stdout_with_timeout`, `xdg_config_home(_from)`, `CONFIG_HOME_UNRESOLVED`, `CacheSlot`/`CacheFill`/`Stamped`, `DetectionContext::{icon_theme, linux_desktop, refined_linux_desktop_sync, invalidate_icon_theme, invalidate_linux_desktop, invalidate_all}`, `system()`, `invalidate_caches`, `mod icon_theme` (callers: native-theme-build, icon_theme.rs:268, fonts.rs:374-377, pipeline.rs:755). Reader part: the dark-mode, reduced-motion and accessibility functions, the whole font-DPI chain, their context slots and `invalidate_*`, and the async `refined_linux_desktop` with `finish_refined_desktop`, `RefinedDesktop`, `refine_desktop` and its slot (decision F3) |
| `detect/icon_theme.rs` | kept linked, internal | M1a, ported M1b | icons.rs:319, :1359, :1389, detect.rs:1443; `system_icon_set`, `system_icon_theme`, `detect_icon_theme` become `pub(crate)` (F2) |
| `error.rs` | ported beside (S1's errors), then split | M1a | Stay: `Error::{PlatformUnsupported, WatchUnavailable, ReaderFailed}`, `reader_names::{ICON_THEME, SVG_RASTERIZER}` (icon_theme.rs:202, rasterize.rs:48). Deleted (S16): `FeatureDisabled`, `UnknownPreset`, `NoVariant`, `Toml`, `Io`, `ResolutionIncomplete`, `ResolutionInvalid`, `RangeViolation`, `ErrorKind`, `kind()`, the `From<toml::…>` impls, `field_category`, `TOML_SERIALIZER`, `PRESET_CACHE`. Reader part: `WINDOWS`, `MACOS` and the three `*_WATCHER` names |
| `fonts.rs` | kept, repointed | M1a | `FontStyle` moves to S2.8's type; public API (S16) |
| `freedesktop.rs` | kept; loses the spinner loader for good | M1a | removed: `load_freedesktop_spinner(_with)`, `spinner_in_chain`, the `AnimatedIcon` import (S2.7) |
| `icons.rs` | kept, repointed (`Rgba` → `Color`, model paths) | M1a | removed for good: `load_icon_indicator`, the three `load_indicator`s, the animated re-exports; `load_icon_in` deleted (only caller lib.rs:801, `SystemTheme::icon`; M2.5's `ResolvedTheme::icon` restores what it needs, F6) |
| `kde/mod.rs` | split | M1a / M1b | Stay: `kdeglobals_path` (detect.rs:172, pipeline.rs:781), `kdeglobals_icon_theme` (icon_theme.rs:321) with `kconfig_files_icon_theme`, `icon_theme(ini)`, `kdeglobals_files(_pure)`, `read_kconfig_files`, `read_kconfig`, `create_kde_parser`. Reader part: `from_kde_content_pure`, `read_kcmfonts_key` and `kcmfonts_key_in` (only caller in the DPI chain, F3), the icon-size parsers, `KdeReader`, `read_kdeglobals`, the reduce-motion and dark functions, `parse_rgb`, `mod colors`, `mod fonts` |
| `kde/colors.rs` | split | M1a; mapping back M1b | Deleted (D10, not restored): `QtColor`, `QtHsv`, `q_round`, `mix(_qt)`, `linear_interpolation(_qt)`, `tint`, `overlay_colors`, `byte_mul`/`mul_65535`/`div_257`, `switch_*_track` and their vector tests. Unlinked: `populate_colors`, `get_color`, `frame_contrast`; restored at M1b calling `platform/` (F16) |
| `kde/fonts.rs` | unlinked; redone by the clean room at M1b | M1a → M1b | F7 |
| `kde/metrics.rs` (`kde_metrics`, lib.rs:272-275) | deleted | M1a | callers kde/mod.rs:79 (reader) and documented_sizes.rs:103 (deleted); its Breeze metrics become preset facts (F5) |
| `kde/qt_color_vectors.rs` | deleted (D10) | M1a | output of `generate_qt_color_vectors.py` |
| `gnome/mod.rs` | unlinked whole | M1a → M1b | `detect_portal_backend`'s only caller is the async refine (detect.rs:1307), reader part (F4); its Pango font parser is redone by the clean room at M1b (F7) |
| `gnome/accent.rs` | deleted (D10) | M1a | port of `adw_rgba_to_standalone`; redone by the clean room (M1a task 13) |
| `macos.rs` | split | M1a / M1b | Stay: `SystemUiFont`, `system_ui_font` (fonts.rs:260; test :777), `os_version` (pipeline.rs:824, F1); the rest reader part |
| `windows.rs` | split: `os_version` only | M1a / M1b | `LOGICAL_DPI`'s callers are detect.rs:646 (DPI chain) and documented_sizes.rs:1158 (deleted), so it goes with the reader part (F1) |
| `model/icons.rs` | kept, moved out of `model/` | M1a | `IconSet`, `IconRole` exhaustive (S21.2); used by native-theme-build, sficons.rs, winicons.rs, icons.rs (F10) |
| `model/bundled.rs` | kept, moved with `icons.rs` | M1a | icons.rs:602-668, model/icons.rs:553 (F9) |
| `model/animated.rs` | deleted | M1a | S2.7, S19.3 |
| `model/{mod, arrow_side, border, defaults, dialog_entrance_style, dialog_order, font, icon_sizes, resolved, tab_indicator_side, widgets/mod}.rs` | deleted | M1a | today's model |
| `pipeline.rs` | split | M1a / M1b | Stay: `DiagnosticEntry` (`OsGeneration.preset` → `PresetId`), `VersionMatch`, `OsVersion`, `Generation`, `GenerationTable`, `MACOS/WINDOWS_GENERATIONS`, `version_match`, `preset_for_version`, `macos_/windows_generation` (returning `PresetId`), `diagnose_platform_support` (F1). Deleted: `PlatformPreset`, `platform_preset_name`, `linux_preset_for_de` (S16: `PresetId::for_system`), `run_pipeline`, `replay`. Reader part: `preset_as_reader`, `SelectedReader`, `select_reader`, `LinuxReader`, `linux_reader(_kind)`, `from_system(_with_preset)_inner`, `linux_fallback`, `read_through_pipeline`, `pinned_pipeline`, `reader_accessibility`, `readerless_accessibility`, `macos_major` |
| `presets.rs` | deleted | M1a | replaced by `PresetId` and `Palette` |
| `presets/{adwaita, ios-13, kde-breeze, macos-monterey, macos-tahoe, material-3, windows-11}.toml` | deleted | M1a | replaced by `next/presets/` moved into place |
| `presets/` ten community TOMLs | deleted | M1a | palettes come at M2.4 |
| `presets/documented_sizes.rs` | deleted | M1a | the plan names it |
| `presets/README.md` | replaced by the README of `next/presets/` moved into place | M1a | F15 |
| `prelude.rs` | deleted | M1a | S16 states no prelude (F8) |
| `rasterize.rs`, `sficons.rs`, `winicons.rs` | kept, repointed | M1a | the plan names them |
| `reader.rs` | unlinked | M1a → M1b | `ThemeReader`; ported to S9.3 at M1b |
| `resolve/{mod, context, inheritance, tests, validate, validate_helpers}.rs` | deleted | M1a | inheritance.rs:370, :412 read `inheritance-rules.toml` |
| `spinners.rs` | deleted | M1a | S2.7, S19.3 |
| `watch/{mod, gnome, kde, macos, windows}.rs` | unlinked | M1a → M1b | `on_theme_change` is kept by S16 |

## Items of lib.rs

| Item | Fate | Reason |
|---|---|---|
| `#![deny(...)]`, `ReadmeDoctests` | kept | the README is rewritten at the switch |
| `impl_merge!` (:204) | deleted | model code |
| `mod model` (:277), `pub mod resolve` (:292), `pub mod theme {…}` (:308-319), `mod spinners` (:298) | deleted | `theme::IconSet`'s path moves (F10) |
| `pub mod presets` (:279) | deleted, replaced by `next`'s presets | — |
| `kde_metrics` (:274) | deleted | F5 |
| `mod reader` (:284), `pub mod watch` (:299), `gnome` (:254) | unlinked | M1b |
| `color`, `detect`, `error`, `icons`, `kde`, `macos`, `windows`, `pipeline` | split | as above |
| `fonts`, `freedesktop`, `rasterize`, `sficons`, `winicons` | kept | — |
| `pub mod prelude` (:302) | deleted | F8 |
| `pub type Result` (:357), `pub use error::Error` | kept | S16 |
| `ParseColorError`, `pub use color::Rgba` | deleted | `Color` replaces `Rgba` |
| `detect::{LinuxDesktop, detect_linux_desktop, invalidate_caches}` | kept | — |
| `detect::{detect_icon_theme, system_icon_set, system_icon_theme}` | internal | F2 |
| `detect::{detect_is_dark, detect_reduced_motion, prefers_reduced_motion, system_is_dark}` | deleted | S16 |
| `error::{ErrorKind, RangeViolation}` | deleted | S16 |
| `icons::{…}` | `load_icon_indicator` dropped; the rest kept | — |
| `model::icons::icon_name` | repointed | — |
| `model::{…}` (:389-400) | `IconData`, `IconProvider`, `IconRole`, `IconSet`, `bundled_*` kept (repointed), the rest deleted | `AnimatedIcon`, `TransformAnimation` and the `*Theme` types go |
| `pipeline::{DiagnosticEntry, VersionMatch, diagnose_platform_support}` | kept | F1 |
| `pipeline::{PlatformPreset, platform_preset_name}` | deleted | S16 |
| `ResolutionContext`, `theme::{ColorMode, Resolved, ResolvedTheme, Theme}` | deleted | replaced by `next`'s `Theme`, `Scheme`, `ResolvedTheme` |
| `AccessibilityPreferences` (:415) | deleted | S16 |
| `ReaderResult` (:507), `ReaderOutput` (:533), `OverlaySource` (:608), `SystemTheme` (:637) and the three test modules | deleted | the readers' old output; M1b's port uses `ReaderState` |

## Cargo features (native-theme/Cargo.toml)

`kde`, `linux-kde` kept (gate the split `kde` module and detect's KDE code); `portal`, `linux-portal` kept, inert
until M1b (`gnome` unlinked); `macos`, `windows` kept (gate `os_version`); `linux`, `native` kept (aliases); `watch`
kept, inert until M1b; `material-icons`, `lucide-icons`, `system-icons`, `svg-rasterize`, `system-fonts` kept, no
longer gating `spinners`.

## native-theme/tests

| Test | Fate | Reason |
|---|---|---|
| `icon_sources.rs` | kept | `icons/SOURCES.toml` |
| `unsafe_safety_comments.rs` | kept | walks `src/`, so unlinked files are still scanned |
| `merge_behavior.rs`, `serde_roundtrip.rs`, `preset_loading.rs` | deleted | today's model; S19.1's build test replaces the preset test |
| `proptest_roundtrip.rs` and its regressions file | deleted | today's model; the `proptest` dev-dependency and `proptest.toml` go unless the new model's tests use them (F14) |
| `platform_facts_xref.rs` | deleted | the plan, *What carries over* |
| `resolve_and_validate.rs` | deleted; its two bundled-SVG tests (:339-380) move to a kept icon test | F12 |
| `prelude_smoke.rs` | deleted | F8 |
| `reader_kde.rs` | deleted, restored at M1b | the plan names it; `tests/fixtures/kde/*.ini` stays |
| `timings.rs` | split: icon timings kept, preset and resolve timings deleted, reader timings with the reader part | F11 |

## native-theme-build's uses of native-theme

native-theme is a dev-dependency (Cargo.toml:19-20), but the generated code names these paths: codegen.rs:41, :269
`{crate}::theme::IconSet::{Variant}`, :301 `{crate}::theme::IconSet` (→ `{crate}::IconSet`, F10); :113, :141
`{crate}::icons::IconProvider`; :56 `{crate}::detect::LinuxDesktop::{Variant}`; :280
`{crate}::detect::system().refined_linux_desktop_sync()`. Its tests call `native_theme::theme::IconSet::from_name`
and its variants (schema.rs:236, :258-263) and `native_theme::detect::parse_linux_desktop` (schema.rs:402);
schema.rs:293-345 reads `native-theme/src/detect.rs` as text and needs `pub enum LinuxDesktop {` and
`parse_linux_desktop`'s arms to stay in that file (the detect split keeps both); integration.rs:742-800 compiles the
generated code against `../native-theme`.

## M2.1 unlinks (files kept, back at M5)

- native-theme-egui: `[[example]] showcase-egui` (Cargo.toml:95-102), the dev-dependency
  `native-theme-showcase-support` (:62), `autoexamples = false`.
- native-theme-gpui: `[[example]] showcase-gpui` (:117-122), the dev-dependency (:99), `autoexamples = false`,
  `#[cfg(test)] mod showcase;` (src/lib.rs:239-240; src/showcase.rs `include_str!`s the example files and uses
  showcase-support at :3901).
- native-theme-iced: `[[example]] showcase-iced` (:73-78), the dev-dependency (:89), `autoexamples = false`.
- native-theme-egui-widgets: nothing.
- `tools/showcase-support` stays a workspace member (root Cargo.toml:12), under `PLAN_EXCLUDED` until M5.

## Scripts and steps the switch removes

- `docs/inheritance-rules.toml`: deleted after S19.2's conversion; readers resolve/inheritance.rs (deleted) and
  check_section_refs.py:19, :41 (`PLAN_EXCLUDED` until M2.6, the path dropped when it is rewritten).
- `check_wrong_safety_nets.py`: deleted, with its two steps (check_release.sh:1071-1076), the ci.yml copies
  (:419-420) and the mentions at check_release.sh:1009, :1014, :1052-1056.
- `generate_gifs_spinners.py`: deleted after its `assemble_theme_switching_gif` moves to its own script (the
  theme-switching GIFs need it, generate_gifs_theme_switching.sh:52, :67, :82; F13), with generate_assets_release.sh's
  step 2 (:127-131) and `magick` check (:62-64), update_provenance.sh:69, generate_assets_local.sh's step 1 (:12-13),
  the README spinner GIFs (README.md:92-98, native-theme/README.md:186-187) and
  `native-theme/docs/assets/spinner-{material,lucide}.gif`.
- `generate_qt_color_vectors.py`: deleted, with scripts/README.md:179-194 and docs/todo.md's mention.
- `platform_facts_xref.rs`: deleted (it runs inside `cargo test`).

## Decisions (the orchestrator, 2026-10-30)

- **F1.** `diagnose_platform_support` and `DiagnosticEntry` stay linked (S16 keeps them public): pipeline.rs keeps
  its generation tables, macos.rs and windows.rs keep `os_version`; windows.rs's `LOGICAL_DPI` goes with the reader
  part (its callers go), which corrects the plan's example "windows.rs its DPI constant".
- **F2.** `detect/icon_theme.rs` stays linked and internal: S16 removes the three functions from the API, while the
  kept icon loaders still need them.
- **F3.** `read_kcmfonts_key` goes with the reader part (its only caller is the DPI chain), which corrects the plan's
  example; the detect split inlines `refine_desktop`'s match into `refine_linux_desktop` and drops the slot from
  `refined_linux_desktop_sync`, so no field is left unread.
- **F4.** `gnome/` is unlinked whole: `detect_portal_backend`'s only caller goes with the reader part.
- **F5.** `kde/metrics.rs` is deleted: it writes today's model, and its values become preset facts.
- **F6.** `load_icon_in` is deleted at the switch; M2.5's `ResolvedTheme::icon` restores what it needs.
- **F7.** `kde/fonts.rs`'s and gnome's Pango font parsers are redone by the clean room at M1b: D10's decision covers
  the whole list it names.
- **F8.** `prelude.rs` and `prelude_smoke.rs` are deleted: S16 states no prelude.
- **F9.** `model/bundled.rs` is kept and moved with `model/icons.rs`.
- **F10.** `IconSet` and `IconRole` move to the crate root (`native_theme::IconSet`), and native-theme-build's codegen
  and tests follow.
- **F11.** `timings.rs` is split as its tests' subjects are.
- **F12.** The two bundled-SVG tests move to a kept icon test.
- **F13.** `assemble_theme_switching_gif` moves to its own script before `generate_gifs_spinners.py` is deleted.
- **F14.** `proptest` goes unless a new-model test uses it.
- **F15.** `next/presets/` carries its own README, moved into place.
- **F16.** `kde/colors.rs`'s mapping is restored at M1b calling `platform/`'s functions, not the deleted ports.
