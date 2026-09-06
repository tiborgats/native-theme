# v0.5.8 — gpui connector on gpui-component 0.6: Specification

Status: Implemented on branch `v0.5.8-gpui-component-0.6` (2026-09-06); release pending the maintainer's testing and approval
Crates: `connectors/native-theme-gpui` (primary), `native-theme` (two API
additions, icon bundle rename and refresh, dependency refresh), workspace
manifests
Target toolkit: **gpui-component 0.6.0**, **gpui-base 0.6.0**, GPUI as the
**`gpui-pre` 0.3.x** package; **gpui-kit 0.6.0** for the showcase example only
Companion rationale:
[`todo_v0.5.8_gpui-component-0.6-rationale.md`](todo_v0.5.8_gpui-component-0.6-rationale.md)
Companion plan:
[`todo_v0.5.8_gpui-component-0.6-plan.md`](todo_v0.5.8_gpui-component-0.6-plan.md)

---

## 0 -- Scope

### 0.1 What v0.5.8 delivers

1. **Dependency move.** `native-theme-gpui` moves from `gpui 0.2.2` +
   `gpui-component 0.5.1` to `gpui-component 0.6.0`, `gpui-base 0.6.0` and
   GPUI published as the `gpui-pre` 0.3.x package. The showcase example moves
   to the `gpui-kit` 0.6.0 facade.
2. **Mechanical migration** of the library and showcase to the 0.6.0 API (§5).
3. **Complete colour mapping** of the 34 `ThemeColor` fields 0.6.0 added or
   renamed, and removal of the 3 it dropped (§6).
4. **Accessibility becomes a first-class input.** `to_theme` and `from_preset`
   take `&AccessibilityPreferences`; the text-scaling factor scales the theme
   fonts and, through GPUI's rem, everything rem-relative; reduce-motion is
   forwarded to GPUI; reduce-transparency keeps its existing effect (§7).
5. **Installation with automatic re-application.** `apply` installs the theme,
   stores the resolved variants in a `NativeTheme` global, projects into
   gpui-base, overrides the base scrollbar geometry and colours and the
   resize-handle colours with native values, and installs a global observer
   that restores those overrides whenever upstream rebuilds the base layer
   (§8).
6. **Per-widget geometry** for every widget where 0.6.0 applies the caller's
   style refinement after its own geometry, delivered as a `geometry` module
   of pure `StyleRefinement` builders and `Size` helpers, text sizes scaled
   by the accessibility factor (§9).
7. **Fifteen new `IconName` variants** mapped in the three icon tables; the
   tables return `Option`; the Lucide bundle moves to Lucide 1.41.0 with its
   files under Lucide's own names, the Material bundle is refreshed to
   upstream HEAD, and both gain the genuine files for the new variants, a
   provenance manifest, a refresh script and generated name tables (§10).
8. **Two additive `native-theme` API items**: `SystemTheme.layout` (already
   approved) and `AccessibilityPreferences::from_system()` (§11).
9. **Dependency refresh** of every external crate in the workspace to its
   latest release, with a recorded reason for each exception, and a
   re-measured MSRV (§4.3, §4.4).
10. **Documentation, CI and release plumbing** (§12, §13).

### 0.2 Constraints

- **No fork, no patch, no overwrite.** Nothing modifies, vendors or
  `[patch]`es gpui-pre, gpui-base, gpui-component or gpui-kit. Every value
  enters through a public API of the pinned versions, and every entry point
  is cited as `crate version path:line` against the published source.
- **No invented values.** Every pixel, colour and font value comes from a
  `ResolvedTheme` field or an `AccessibilityPreferences` field. The
  derivations that combine them are enumerated in §8.2 and §9.4 and argued
  in the rationale §5.
- **Breaking changes are allowed** (pre-1.0) and are listed in one place
  (§7.1). Everything else is additive.

### 0.3 Out of scope

§14 lists what this release does not do and why. In one line: geometry on an
inner element the caller's style cannot reach stays in v0.6.2 as upstream
work; platform research that would be needed for new theme fields (scrollbar
thumb radius, macOS and Windows accessibility readers) is recorded as todo
items rather than guessed.

---

## 1 -- Facts this specification rests on

Verified on 2026-09-05 against the published crate sources in the local
cargo registry, the upstream GitHub repositories, crates.io, and a throwaway
compile probe of the connector sources against the 0.6.0 stack. Line numbers
refer to the published 0.6.0 / 0.3.3 sources and are stable for those
versions.

### 1.1 Upstream ecosystem

| Fact | Evidence |
|------|----------|
| `longbridge/gpui-component` was renamed `longbridge/gpui-kit`; the `gpui-component` crate continues as the styled layer | GitHub redirect; gpui-kit README |
| `gpui-component 0.6.0`, `gpui-base 0.6.0`, `gpui-kit 0.6.0`, `gpui-kit-assets 0.6.0` published 2026-09-03 | crates.io |
| `gpui-component 0.6.0` depends on `gpui = { package = "gpui-pre", version = "0.3.1" }` and `gpui-base = "0.6.0"` | published `Cargo.toml` |
| `gpui-component` does **not** re-export `gpui`; `gpui-kit` does | gpui-component `src/lib.rs`; gpui-kit `src/lib.rs:86-91` |
| `gpui-pre` 0.3.0–0.3.3 were all published 2026-09-03 by the gpui-kit maintainer as snapshot republishes of Zed `main`, every other Sunday, as `0.3.N` patch bumps that consumers pick up on `cargo update` | crates.io owner list; gpui-kit `CONTRIBUTING.md:138-217` |
| The official `gpui` crate is at 0.2.2, published 2025-10-22 | crates.io |
| Zed's toolchain at the snapshot commit is Rust 1.97.1 | `rust-toolchain.toml` at `zed-industries/zed@5b055fa` |
| The 0.6.0 closure refuses rustc 1.88.0: `oo7 0.6.0` requires 1.92, `ordered-float 5.5.0` 1.90, `cosmic-text 0.19.0`, `smol_str 0.3.6`, `notify-rust 4.18.0` 1.89, all via `gpui-pre-linux` / `gpui-pre-wgpu` | `cargo +1.88.0 check` on the probe; `cargo tree -i` |
| The connector library compiles on Rust 1.95 against the 0.6.0 stack | `cargo +1.95 check --lib` on the probe |
| The 0.6.0 stack resolves `naga 29.0.4` and `codespan-reporting 0.13.1` | `cargo tree -i` on the probe |

### 1.2 The 0.6.0 theme model and GPUI runtime

| Fact | Evidence (gpui-component 0.6.0 unless noted) |
|------|----------|
| `ThemeColor` has 139 `Hsla` fields; 0.5.1 had 108; 34 added (28 `button_*`, `chart_bullish`, `chart_bearish`, `status_bar`, `status_bar_border`, `table_foot`, `table_foot_foreground`), 3 removed (`accordion_hover`, `bullish`, `bearish`) | `src/theme/theme_color.rs:59-342` vs 0.5.1 |
| `ThemeColor` derives `Default`: unassigned fields are the zero `Hsla`, transparent black | `src/theme/theme_color.rs:58` |
| `Button` reads `tokens.button*` for backgrounds and `button_*_foreground` for solid text; `ThemeTokens` is generated field-by-field from `ThemeColor` | `src/button/button.rs:884-949`; `src/theme/theme_color.rs:356-368` |
| `Button` sets its label text size on an inner element: `text_xs` for XSmall, `text_sm` for Small, `text_base` for Medium, Large and `Size::Size` | `src/button/button.rs:658-666`; `src/sizing.rs:319-324` |
| `Input` sets `input_text_size` (`text_sm` at Medium) on its root **before** applying the caller's refinement | `src/input/input.rs:572, 587`; `src/sizing.rs:225-232` |
| `Root` sets the window's rem size to `theme.font_size` and the window font family to `theme.font_family`; every `text_*`, `h_*`, `px_*` helper in gpui-component is rem-relative | `src/root.rs:579, 591` |
| `Theme` gained `tokens`, `focus_ring: bool`, `scrollbar_mode: ScrollbarMode`, `notification`, `tile_*`, `list: ListSettings`, `sheet`, `motion`; `scrollbar_show` is gone | `src/theme/mod.rs:87-146` |
| `ScrollbarMode` has `Scrolling`, `Hover`, `Always` | gpui-base `src/scrollbar.rs:47-55` |
| `Theme::change`, `sync_system_appearance` and `sync_base` rebuild the gpui-base theme via `base_theme()` with fixed scrollbar styles (colours and `radius` only, no widths); `set_scrollbar_mode` preserves existing styles | `src/theme/mod.rs:204-214, 226-234, 237-260, 268-300, 321-326` |
| `gpui_component::init` calls `Theme::change(Light)` then `sync_scrollbar_appearance`; nothing installs an appearance observer | `src/theme/mod.rs:31-35, 216-224` |
| `apply_config` fills missing `button_*` config fields with a **tinted** design: default `button` = `input.mix_oklab(transparent, 0.3)` (dark) or `background` (light); status backgrounds `X.mix_oklab(transparent, 0.2)`; status text = `X` itself | `src/theme/schema.rs:786-800, 845-856, 935` |
| gpui-base `Theme { appearance, tokens, scrollbar, resizable }` is a `Global`; `ScrollbarTheme` is builder-only with getters; `ScrollbarStyles`/`TrackStyle`/`ThumbStyle` are builder-only, private fields, derive only `Clone, Default` | gpui-base `src/theme.rs:17-112`; `src/scrollbar.rs:589-700` |
| `ScrollbarTrackStyle`: `bg`, `border_color`, `width`; `ScrollbarThumbStyle`: `bg`, `width`, `inset`, `radius`, `min_length` | gpui-base `src/scrollbar.rs:597-646` |
| Base scrollbar defaults: track `px(16.)`, min thumb `px(48.)`, thumb `px(6.)`, inset `px(4.)`, active thumb `px(8.)` | gpui-base `src/scrollbar.rs:21-30` |
| Thumb fill is anchored `inset` from the track's outer edge, `thumb_width` wide, and shortened by `inset` at both ends | gpui-base `src/scrollbar.rs:1374, 1391-1400` |
| gpui-component re-exports `ScrollbarMode`, `ScrollbarStyles`, `ScrollbarThumbStyle`, `ScrollbarTrackStyle`; not gpui-base `Theme`, `ScrollbarTheme`, `ResizableTheme` | `src/scroll/mod.rs:3-8` |
| `ResizableTheme { handle, active_handle }` is read by the base resize handle; the handle width is the constant `HANDLE_SIZE = px(1.)` | gpui-base `src/theme.rs:108-112`; `src/resizable/resize_handle.rs:12` |
| `StyledExt::refine_style(self, &StyleRefinement)` exists on every `Styled` type (blanket impl) and is re-exported by gpui-component; `StyleRefinement` is itself `Styled` | gpui-base `src/styled.rs:78-82, 205`; gpui-component `src/styled.rs:7`; gpui-pre `src/style.rs:324` |
| `Size::Size(Pixels)` exists | `src/sizing.rs:6-13` |
| `App::reduce_motion()` / `set_reduce_motion(bool)`; GPUI's animation element and gpui-component's spinner, shimmer, progress and marker honour it; the platform layer never reads the OS setting | gpui-pre `src/app.rs:1059-1068`, `src/elements/animation.rs:301, 407`; gpui-component `spinner.rs`, `shimmer.rs:211`, `progress/progress.rs:90`, `marker.rs:336` |
| `App::observe_global<G>(FnMut(&mut App)) -> Subscription`; `set_global` and `global_mut` both queue a `NotifyGlobalObservers` effect; queued notifications for one type are deduplicated until delivered; delivery removes the pending mark, then calls observers | gpui-pre `src/app.rs:2087-2099, 2040-2043, 2062-2065, 1662-1664, 1817-1821` |
| `Progress` copies the caller's corner radii onto its fill elements and applies the caller's refinement after its own height and radius | `src/progress/progress.rs:92-94, 128-130, 137, 147` |
| `IconName` is generated from the SVG files in the assets crate (PascalCase of the file name); 0.6.0 ships 101 files, 0.5.1 shipped 86; the 15 new ones are `battery`, `battery-charging`, `battery-full`, `battery-low`, `battery-medium`, `battery-warning`, `cpu`, `file-text`, `hard-drive`, `memory-stick`, `network`, `pause`, `play`, `rotate-cw`, `star-fill`; `github` now yields `Github` | `src/icon.rs:29`; gpui-kit `crates/component-macros/src/lib.rs` (`pascal_case`); `crates/assets/assets/icons` at v0.6.0 |
| gpui-kit's `star-fill.svg` is Lucide's `star` glyph with `fill="currentColor"` | file contents at v0.6.0 |
| `gpui-pre` has a `test-support` feature and re-exports the `test` attribute macro; `TestAppContext` exists | gpui-pre `Cargo.toml:70`; `src/gpui.rs:108-110`; `src/app/test_context.rs:21` |

### 1.3 native-theme and the connector today

| Fact | Evidence |
|------|----------|
| The connector seeds `ThemeColor::default()` and assigns every field individually; the tripwire asserts 108 fields by `size_of` | `connectors/native-theme-gpui/src/colors.rs:213, 231-473, 869-877` |
| `from_system` and `SystemThemeExt::to_gpui_theme` pass only `accessibility.reduce_transparency` into `to_theme`; `text_scaling_factor` and `reduce_motion` are ignored | `src/lib.rs:237-246, 266-275` |
| `AccessibilityPreferences { text_scaling_factor, reduce_motion, high_contrast, reduce_transparency }` implements `Default`; its doc says consumers multiply font sizes by the factor | `native-theme/src/lib.rs:230-247` |
| All four platform readers fill `ReaderResult.accessibility`: KDE and GNOME (`kde/mod.rs:51`, `gnome/mod.rs:194`), macOS (all four fields from `NSWorkspace` and the system font size, `macos.rs:142-150, 513-528`) and Windows (`UISettings`, `windows.rs:393-398`); only the preset-only fallback and non-KDE/GNOME Linux leave `Default` (`pipeline.rs:596`). `detect::prefers_reduced_motion()` (cached) and `detect_reduced_motion()` (uncached, `detect.rs:662`) cover Linux, macOS (feature) and Windows (feature). The design's earlier claim that only KDE and GNOME fill the struct was wrong (rationale error 55) | the cited lines |
| The three icon tables are exhaustive `match`es returning `&'static str`; `ALL_ICON_NAMES` is hand-written and asserted at 86 | `src/icons.rs:140, 250, 363, 1250, 1505-1512` |
| `ResolvedTheme` carries per-widget geometry as `f32` and a `ResolvedFontSpec { family, size, weight, style, color }` per widget; `defaults.line_height` is an `f32` multiplier | `native-theme/src/model/widgets/mod.rs`; `model/border.rs`; `model/font.rs`; `model/resolved.rs:46` |
| `LayoutTheme { widget_gap, container_margin, window_margin, section_gap }`, all `Option<f32>`, lives on `Theme`; all 16 static presets define all four keys; the 4 `*-live` presets define none and take theirs from the platform reader merge; platform-facts §2.20 records no layout defaults for Windows ("app chooses"), none for macOS `container_margin`, none for KDE `section_gap`; `SystemTheme` has no `layout` field (approved 2026-08-10, pending) | `model/mod.rs:265-266`; `src/presets/*.toml`; `docs/platform-facts.md:1427-1435`; `pipeline.rs:46, 120-126`; `lib.rs:343-351`; `docs/todo.md` |
| platform-facts treats radio buttons as checkboxes with a circular indicator: `label_gap` and `indicator_width` are defined for both, "Radio buttons use the same colors but with circular `border.corner_radius`" | `docs/platform-facts.md:947, 969, 1191-1210` |
| platform-facts has no scrollbar thumb radius or track border colour | `docs/platform-facts.md` (search) |
| `splitter.divider_color` inherits `defaults.border.color` and `splitter.hover_color` inherits `splitter.divider_color`; no preset defines a `[splitter]` table, so in every shipped preset both splitter colours resolve to the border colour | `docs/inheritance-rules.toml:238, 273`; `grep -l '^\[splitter\]' src/presets/*.toml` (empty) |
| `ThemeConfig.highlight: Option<HighlightThemeStyle>`; `Theme::apply_config` installs it as `highlight_theme` when `Some` and leaves the previous `highlight_theme` when `None`; both registry default themes carry one; `HighlightTheme::default_dark() / default_light()` expose upstream's defaults with a public `style` | `src/theme/schema.rs:77, 1066-1073`; `src/theme/default-theme.json:113, 314`; `src/highlighter/registry.rs:463-485` |
| Lucide bundle: 103 files, 99 name-table entries; **all 103 are Lucide icons** from tag **0.577.0** (byte-identical for sampled files). Ten are stored under gpui-component's icon names instead of Lucide's: `close` and `window-close` = `x`, `dash` and `window-minimize` = `minus`, `inspect` = `scan`, `resize-corner` = `grip`, `sort-ascending` = `arrow-up-narrow-wide`, `sort-descending` = `arrow-down-wide-narrow`, `window-maximize` = `maximize`, `window-restore` = `minimize-2` (path data identical after whitespace normalisation); the bundle already holds all eight of those files under their Lucide names, byte-identical, so the ten are duplicates, and the four files absent from the hand-written name table are `scan`, `grip`, `arrow-up-narrow-wide` and `arrow-down-wide-narrow`. The adding commit (`48f67c5`) records none of this. Nothing outside the connector uses those ten names: the role-based tables reference only `trash-2.svg` among the files affected by §10.2 (`bundled.rs:137, 163-164`), and the hand-written coverage test `lucide_by_name_covers_gpui_icons` (`bundled.rs:508`) lists them | `native-theme/icons/lucide`; `bundled.rs`; normalised comparison against the 0.577.0 files; repository grep |
| Lucide 1.41.0 (2026-09-04) contains the 14 new names and every underlying icon above; it lacks `github` (brand icons removed upstream, commit `aa8f74eb`) and `trash-2`, which became a deprecated alias of `trash` whose glyph is identical to the old `trash-2` (path data compared); it has no filled star: `star`, `star-off` and `star-half` exist at the tag, `star-fill` and `star-filled` do not | git tree of tag 1.41.0; `icons/trash.json` at 1.41.0; raw probes of the tag |
| Material bundle: `star.svg` and `star_border.svg` have identical path data (the Symbols outlined hollow star); `star_border` backs `IconName::StarOff`; Material Symbols has no star-off glyph (`star_off`, `star_outline`, `star_border` do not exist; `star_rate`, `star_half` do) | `native-theme/icons/material`; `bundled.rs:399`; connector `icons.rs:326`; GitHub contents API |
| Material bundle: 87 files, 76 name-table entries; `search`, `settings`, `star` are byte-identical to Material Symbols **Outlined 24px** (`symbols/web/<n>/materialsymbolsoutlined/<n>_24px.svg`); `warning` and `info` match no upstream variant probed; `font_size.svg` exists under no such upstream name and is byte-identical to `format_size_24px.svg` (found when the refresh script 404ed on it, Task 5); the old Lucide `delete.svg` was a trash-can glyph, not Lucide's `delete` (a backspace key, which gpui-kit's own `delete.svg` also is) and the refresh replaced it | `native-theme/icons/material`; comparisons; Task 5 refresh run |
| Material Symbols files exist at upstream HEAD (`0cbb08816df0`, 2026-09-04) for every name in §10.3 | GitHub contents API |
| Breeze and Adwaita names for all 15 icons verified on this machine | `find /usr/share/icons/{breeze,Adwaita}` |
| Breeze (`status/{16,22,24}`) and Adwaita (`symbolic/status`) both ship the star-state pair `non-starred` / `starred` (and `semi-starred`); the connector's table maps `Star` to `starred`, the filled star | `find /usr/share/icons/{breeze,Adwaita} -iname '*starred*'`; `icons.rs:413` |
| The connector's `[package.metadata.docs.rs]` three-target list is unreleased (added in 0.5.8, commit `ce0fdee`); 0.5.7 declared no targets and docs.rs built it for `x86_64-unknown-linux-gnu` only. The connector's only platform-gated public items are Linux-gated (the `LinuxDesktop` re-export, `freedesktop_name_for_gpui_icon`), so the default target shows every public item. gpui-pre 0.3.3, gpui-component 0.6.0 and gpui-kit 0.6.0 declare no docs.rs targets and were each built for Linux only, so no cross-target docs build of the GPUI stack has ever run. gpui-component depends on gpui-pre with its default features, which include `windows-manifest`: for a Windows target gpui-pre's build script runs `embed-resource`, which on a Linux host needs `llvm-rc` (docs.rs's image ships `llvm`, `crates-build-env` `linux/packages.txt:1045`) | `git show v0.5.7:connectors/native-theme-gpui/Cargo.toml`; `CHANGELOG.md:12`; `lib.rs:92-93`, `icons.rs:361`; docs.rs `/crate/<c>/<v>/builds`; gpui-component `Cargo.toml:266-267`; gpui-pre `Cargo.toml:59-64, 79`, `build.rs`; embed-resource 3.0.8 `src/non_windows.rs:73-86` |
| publish.yml soft-gates the connector for G11 (naga 27.0.3 vs codespan-reporting 0.12.0 on the 0.5.1 stack) | `.github/workflows/publish.yml:38-44, 58-60, 71-75, 156-175`; `docs/archive/v0.5.7_gaps.md` §G11 |
| CI installs `libxcb1-dev libxkbcommon-dev libxkbcommon-x11-dev` for the connector job | `.github/workflows/ci.yml:93-95` |
| native-theme declares no default features; the workspace MSRV 1.88.0 was measured as the maximum declared `rust-version` across the lock plus a `cargo +1.88.0 check --all-targets --locked` per member | `native-theme/Cargo.toml`; commit `0319942` |

### 1.4 Measured migration cost (throwaway probe, not committed)

| Measurement | Result |
|-------------|--------|
| Library compile errors, sources unmodified | 14 |
| …unresolved `ScrollbarShow` / unknown `scrollbar_show` | 2 |
| …unknown `bullish`, `bearish`, `accordion_hover` (colors.rs, config.rs) | 9 |
| …`IconName::GitHub` (three tables) | 3 |
| After those: non-exhaustive matches, 15 variants missing | 3 |
| After stubbing those: library compiles | 0 |
| Showcase errors after the documented renames | 31 first pass |

### 1.5 Dependency landscape (crates.io, 2026-09-05)

| Crate | Workspace requirement | Latest stable | Notes |
|-------|----------------------|---------------|-------|
| gpui-pre | new | 0.3.3 | |
| gpui-component, gpui-base, gpui-kit | new | 0.6.0 | |
| serde | 1.0.228 | 1.0.229 | |
| serde_with | 3.18.0 | 3.22.0 | |
| toml | 1.1.2 | 1.1.5 | |
| serde_json | 1.0.149 | 1.0.151 | |
| arc-swap | 1.9.1 | 1.9.2 | |
| async-trait | 0.1.89 | 0.1.92 | |
| ashpd | 0.13.10 | 0.13.13 | |
| configparser | 3.1.0 | 3.2.0 | |
| zbus | 5.14 | 5.19.0 | |
| quote / proc-macro2 | 1.0.45 / 1.0.106 | 1.0.47 / 1.0.107 | |
| pollster | 0.4 | 1.0.1 (2026-07-10) | only change: `FutureExt` for `IntoFuture`; MSRV 1.69 |
| syn | 2.0.117 | 3.0.5 (2026-09-04; 3.0.0 2026-07-18) | breaking: `*Modifiers` structs, `Type::BareFn` → `Type::FnPtr`; MSRV 1.71; the derive crate uses `Type`, `Type::Path`, `Ident`, `ItemStruct`, `Fields::Named`, `Data::Struct`, `GenericArgument`, `PathArguments`, `Expr`, `Attribute`, `parse_str`, `Error` |
| resvg | 0.47 | 0.48.1 (2026-08-02) | new font stack (skrifa, harfrust); declares `rust-version` 1.85.0, and `native-theme` uses it with `default-features = false`, so no text-stack crate enters through it (Task 1 measured the workspace floor unchanged at 1.88.0); "may result in small rendering changes" |
| inventory, objc2 family, block2, freedesktop-icons, notify, png, windows 0.62.2, proptest, heck, iced 0.14, image 0.25.10 | current | current | already latest |

---

## 2 -- Vocabulary: seams and tiers

A **seam** is a documented point in the published upstream source where a
value supplied by this crate replaces one upstream would otherwise hardcode,
without editing upstream.

| Tier | Meaning | In this milestone |
|------|---------|-------------------|
| **G** — global receiver | a field or setter on a global every widget reads | `Theme.focus_ring`, `Theme.scrollbar_mode`, `Theme.font_size` (rem), gpui-base `Theme.scrollbar` / `resizable`, `App::set_reduce_motion` |
| **R** — refinement seam | the widget applies the caller's `StyleRefinement` after its own geometry | Button, Input, MenuItem, ListItem, Tooltip, Popover, StatusBar, Dialog, DialogFooter, DialogTitle, DialogDescription, Table, Progress, Checkbox, Radio, Select, Combobox, TitleBar |
| **S** — size seam | the widget honours `Size::Size(Pixels)` | Spinner, Icon |
| **B** — builder seam | a widget-specific builder takes the value | `Dialog::max_w`, `GroupBox::content_style`, `AccordionItem::title_style`, `Input::h` |
| **U** — unreachable | inner element, or the widget discards refinements | §14 |

**Admission rule** (from `todo_egui-widgets-spec.md` §2.5): use the cheapest
tier that produces the appearance the theme specifies; promote only by citing
the upstream line that makes the cheaper tier insufficient.

---

## 3 -- Architecture

### 3.1 Pipeline

```text
ResolvedTheme + AccessibilityPreferences
   --colors::to_theme_color--> ThemeColor (139)
   --config::to_theme_config--> ThemeConfig (hex; font sizes scaled)
   --to_theme----------------> gpui_component::Theme (font_size scaled -> rem; focus_ring; scrollbar_mode)

apply(theme, resolved, prefs, cx)
   1. store variant + prefs in NativeTheme (Global)
   2. *Theme::global_mut(cx) = theme
   2b. ThemeConfig for the other stored variant     -> Theme::change(other) reproduces native colours (D34)
   3. Theme::sync_base(cx)                         upstream projection
   4. base_layer::apply_overrides(..)              native scrollbar geometry/colours, resize-handle colours
   5. cx.set_reduce_motion(prefs.reduce_motion)
   6. once: cx.observe_global::<gpui_base::Theme>(re-apply 4)   -> Theme::change etc. cannot undo 4
   6b. first install only: cx.defer(re-apply 4)      -> a rebuild in the same update as 6 (D38)
   7. cx.refresh_windows()                          paint now, not on the next input event (D37)

Native { resolved, accessibility } --geometry::<widget>--> StyleRefinement --refine_style--> widget
                                    --geometry::<widget>_size--> Size     --with_size----> widget
```

### 3.2 Module ownership

| File | Owns |
|------|------|
| `src/lib.rs` | `to_theme`, `from_preset`, `from_system`, `SystemThemeExt`, metric helpers; `apply`, `apply_system_theme`, `apply_accessibility`, `NativeTheme`, `ActiveNativeTheme`, `Native`; the re-apply observer and its helpers (`install_observer_once`, `write_base_overrides`, `base_overrides_for`) |
| `src/colors.rs` | `ResolvedTheme` → `ThemeColor`, 139 fields |
| `src/config.rs` | `ThemeColor` → `ThemeConfigColors`; scaled font sizes |
| `src/icons.rs` | three `Option`-returning tables, 101 variants |
| `src/base_layer.rs` (new) | `ScrollbarGeometry`, `scrollbar_geometry`, `scrollbar_styles`, `resizable_theme`, `apply_overrides` (pure functions plus one write; no `App` state of its own) |
| `src/geometry.rs` (new) | per-widget builders, `Size` helpers, `control_height`, layout accessors |
| `examples/showcase-gpui.rs` | ported to gpui-kit; uses `apply_system_theme` / `apply` and the geometry builders |

### 3.3 Automatic re-application

Upstream rebuilds the base theme with fixed scrollbar styles in
`Theme::change`, `sync_system_appearance` and `sync_base`
(§1.2). `apply` therefore installs, once per `App`, a global observer on
`gpui_base::Theme` that restores the native overrides. The observer:

1. Reads `NativeTheme`. If its `reapplying` flag is set, clears it and
   returns: the notification was caused by the observer's own write.
2. Determines the current mode from the styled theme (`cx.try_global`, falling
   back to the mode `apply` last installed, §8.1) and picks the stored variant
   for that mode. If no variant is stored for
   that mode, it takes geometry from the stored variant of the other mode and
   colours from the styled theme's `scrollbar`, `scrollbar_thumb`,
   `scrollbar_thumb_hover` (active = hover, as upstream does) and the
   resize-handle colours from the styled theme's `border` / `drag_border`,
   again upstream's own projection (`theme/mod.rs:296-298`).
3. Computes `ScrollbarGeometry` and `ResizableTheme` from that data, sets
   `reapplying`, and writes them through `gpui_base::Theme::global_mut(cx)`,
   preserving `mode()` and `motion()` by cloning the existing scrollbar theme.

Why this terminates: `global_mut` queues one notification; upstream's
pending-mark deduplication (gpui-pre `src/app.rs:1662-1664`) collapses any
duplicates; delivery removes the mark before calling observers
(`:1817-1821`), so the observer's own write yields exactly one further
delivery, which step 1 absorbs. `apply` itself does **not** set
`reapplying`: `observe_global` defers the subscription's activation to the end
of the current effect flush (gpui-pre `src/app.rs:2087-2099`), so on the first
`apply` the observer never receives the notification `apply` queues, and a
flag set there would stay set and swallow the next upstream rebuild. Instead,
the first delivery the active observer does receive re-applies the same values
once (idempotent), and that write is the one the flag absorbs. The
`#[gpui::test]` in §12 runs
`Theme::change` after `apply` and asserts both that the overrides survived
and that the test returns.

Activation gap: because the subscription activates only at the end of the
flush, a base-theme write made by other code in the *same* update as the first
`apply` (`Theme::sync_system_appearance` called right after it, the usual
start-up sequence) is delivered while the observer is still inactive and would
stand until the next rebuild. `install_observer_once` therefore also queues,
after the activation, one deferred re-write of step 4 (`cx.defer`); effects are
FIFO, so it runs after any such write and is the first notification the active
observer receives (D38). Later `apply` calls need nothing: the observer is
active. A `#[gpui::test]` in §12 runs `apply` and `Theme::change` inside one
update and asserts the overrides.

Limit: the same deduplication that makes the observer terminate merges a
rebuild performed by *another effect* between the observer's write and the
delivery of its notification (a deferred callback or another observer calling
`Theme::change`) into that notification, which the flag then absorbs. The
observer therefore compares gpui-base's resize-handle colours with the native
values when it absorbs its own notification and re-writes when they differ
(D42); upstream's rebuild writes `drag_border` into `active_handle`, which
differs from the native hover colour in every shipped preset, so the common
case is repaired. The scrollbar styles are opaque and cannot be compared, so a
rebuild whose handle colours happen to equal the native ones is repaired only
by the next rebuild. Another observer of `gpui_base::Theme` that also writes it
would race this one. The only global observer in gpui-base and gpui-component 0.6.0
watches `ThemeRegistry` (`src/theme/registry.rs:46`), not the base theme. That
observer does, however, replace the styled theme's `light_theme` /
`dark_theme` with registry themes **of the same name** before calling
`Theme::change` (`:41-51`); the connector's configs carry the native display
name, the default registry holds two themes, `Default Light` and
`Default Dark` (`src/theme/default-theme.json`), and the registry is notified
only through its own loading API (`watch_dir`, `load_themes_from_str`), so the
connector's colours are replaced only by an application that loads a
same-named theme through `ThemeRegistry`; the base-layer geometry is still
restored. The README states both assumptions.

### 3.4 Text scaling and rem

`Root` sets the window rem size to `theme.font_size` (`src/root.rs:579`).
`to_theme` sets `font_size = defaults.font.size × s` and
`mono_font_size = defaults.mono_font.size × s`, where `s` is
`prefs.text_scaling_factor` when finite and positive, else 1.0. Consequences:

- every rem-relative size in gpui-component (`text_sm`, `h_8`, `px_2`, …)
  scales with the platform's text scaling, as the platform's own toolkit
  scales its text;
- the geometry builders must scale the text sizes they set explicitly (§9.4)
  and grow control heights when scaled text no longer fits (§9.4);
- widths, paddings, radii, icon sizes and scrollbar metrics are not scaled:
  GNOME's text-scaling-factor scales text, not spacing.

### 3.5 No hardcoded values

Every numeric or colour literal in the connector is (a) a `ResolvedTheme` or
`AccessibilityPreferences` field, (b) one of the derivations in §8.2 and §9.4,
or (c) a clamp guard (`max(0.0)`, `> 0.0`). The pre-tool-use hook that blocks
invented theme values applies to all new code.

---

## 4 -- Manifest and dependencies

### 4.1 `connectors/native-theme-gpui/Cargo.toml`

```toml
[package]
name = "native-theme-gpui"
version.workspace = true
edition.workspace = true
license.workspace = true
# NOT inherited: the gpui-pre closure declares a higher floor than the
# workspace. Set to the lowest toolchain that compiles the library (§4.4).
rust-version = "<measured>"
repository.workspace = true
homepage.workspace = true
keywords = ["theme", "gpui", "gui", "native", "colors"]
categories = ["gui", "config"]
readme = "README.md"
description = "gpui toolkit connector for native-theme"

[package.metadata.docs.rs]
# No `targets` list (D40): every platform-gated public item of this crate is
# Linux-gated and appears on docs.rs's default target; the other two targets
# would render the same page minus those items at the cost of the first-ever
# cross-target docs build of gpui-pre's macOS and Windows platform crates.
all-features = true

[features]
default = ["material-icons", "lucide-icons", "system-icons", "svg-rasterize"]
material-icons = ["native-theme/material-icons"]
lucide-icons = ["native-theme/lucide-icons"]
system-icons = ["native-theme/system-icons"]
svg-rasterize = ["native-theme/svg-rasterize"]

[dependencies]
# GPUI under the package name gpui-component 0.6.0 uses, so both edges unify.
# gpui-component does not re-export gpui, so the connector names it. Caret:
# an exact pin in a library would conflict with any consumer whose other
# dependencies want a later snapshot.
gpui = { package = "gpui-pre", version = "0.3.3" }
gpui-component = "0.6.0"
# For gpui_base::Theme, ScrollbarTheme, ResizableTheme (not re-exported).
gpui-base = "0.6.0"
native-theme = { workspace = true }

[target.'cfg(target_os = "linux")'.dependencies]
native-theme = { workspace = true, features = ["linux"] }

[target.'cfg(target_os = "macos")'.dependencies]
native-theme = { workspace = true, features = ["macos"] }

[target.'cfg(target_os = "windows")'.dependencies]
native-theme = { workspace = true, features = ["windows"] }

[dev-dependencies]
# Headless App for the apply/observer tests (#[gpui::test], TestAppContext).
gpui = { package = "gpui-pre", version = "0.3.3", features = ["test-support"] }
# The showcase uses the facade an application would: application(), init(),
# and the Lucide assets under gpui_kit::assets.
gpui-kit = "0.6.0"
native-theme = { workspace = true, features = ["watch"] }

[target.'cfg(target_os = "macos")'.dev-dependencies]
objc2 = "0.6.4"

[target.'cfg(target_os = "windows")'.dev-dependencies]
image = { version = "0.25.10", default-features = false, features = ["png"] }
windows = { version = "0.62.2", default-features = false, features = [
    "Win32_Foundation",
    "Win32_Graphics_Dwm",
    "Win32_Graphics_Gdi",
    "Win32_UI_WindowsAndMessaging",
] }

[[example]]
name = "showcase-gpui"
```

Removed: `gpui-component-assets` (renamed upstream; reachable as
`gpui_kit::assets`).

### 4.2 Version requirements

`gpui-component = "0.6.0"`, `gpui-base = "0.6.0"` and `gpui-kit = "0.6.0"`
are caret requirements and unify with any 0.6.x an application uses. The
`gpui-pre` requirement is the latest snapshot, `0.3.3`; because upstream
publishes Zed snapshots as `0.3.N`, a downstream `cargo update` can pull a
newer one. The connector's GPUI surface is `Hsla`, `hsla`, `Rgba`,
`SharedString`, `px`, `Pixels`, `svg`, `img`, `ImageSource`, `ElementId`,
`IntoElement`, `StyleRefinement`, `FontWeight`, `App`, `Global`,
`Subscription`, which are the most stable parts of GPUI. The README states
this.

### 4.3 Workspace dependency refresh

Policy: every external requirement is set to the latest stable release at
release time, and `Cargo.lock` is regenerated; an older requirement is kept
only with a recorded reason. Applying it to §1.5:

| Crate | Action | Reason where not simply "latest" |
|-------|--------|----------------------------------|
| serde, serde_with, toml, serde_json, arc-swap, async-trait, ashpd, configparser, zbus, quote, proc-macro2 | bump requirement strings to the versions in §1.5 | patch/minor releases |
| pollster | `0.4` → `1.0` | the only change is `FutureExt` for `IntoFuture`; API used (`block_on`) unchanged |
| syn | `2.0.117` → `3.0.5` | the derive crate's syn surface (§1.5) is untouched by 3.0's breaking changes; gate: `cargo test -p native-theme-derive` passes with mechanical edits only; if it does not, stay on 2.0.119 and record why |
| resvg | `0.47` → `0.48.1` | maintained font stack; the resvg-backed rasterisation test asserts only that output has non-zero pixels (`native-theme/src/rasterize.rs:130`), so glyph-level rendering changes cannot break it; the floor is re-measured regardless (§4.4); measured on 2026-09-06: unchanged at 1.88.0, because `resvg` is used with `default-features = false` and declares 1.85.0 |
| windows, image, objc2 family, block2, iced, inventory, notify, png, proptest, heck, freedesktop-icons | none | already latest |

The iced connector is otherwise untouched by this milestone; its two
requirements are already latest.

The workspace uses `resolver = "3"` (edition 2024), so `cargo update` is
MSRV-aware: it resolves against the lowest `rust-version` among the workspace
members and falls back to a newer release only where no compatible one
satisfies the requirement. The floor is therefore measured on the refreshed
lock, declared, and `cargo update` run once more; if the lock changes, the
per-member check is repeated (§4.4).

### 4.4 MSRV

Two floors, both measured, following commit `0319942`:

- **Workspace** (`native-theme`, `native-theme-build`, `native-theme-derive`,
  `native-theme-iced`): re-measure after §4.3. Measured 2026-09-06 (Task 1): 1.88.0, the
  highest `rust-version` declared in the refreshed lock (`time`, `psm`); the
  1.89.0 the design predicted rested on `font-types`, which `resvg` with
  `default-features = false` does not pull. Procedure as in `0319942`: maximum declared `rust-version` across
  the lock, then `cargo +<floor> check -p <member> --all-targets --locked`
  per member.
- **Connector**: the 0.6.0 closure refuses 1.88.0 and declares 1.92 as its
  highest floor; the library compiles on 1.95. Install 1.92.0, 1.93.0,
  1.94.0; run `cargo +<v> check -p native-theme-gpui --lib --locked` from
  the lowest up; declare the first that passes. Provisional value until
  measured: `1.95`.

Both numbers go into the CHANGELOG, the connector README, the root README's MSRV badge, `CONTRIBUTING.md`, and the MSRV CI item in `docs/todo.md` (§13.7).

---

## 5 -- Mechanical migration of existing code

The compile is the gate. What follows is what the probe surfaced.

### 5.1 Library, first pass (14 errors)

| Location | Error | Fix |
|----------|-------|-----|
| `src/lib.rs:96` | unresolved import `gpui_component::scroll::ScrollbarShow` | import `ScrollbarMode` |
| `src/lib.rs:149` | no field `scrollbar_show` | `theme.scrollbar_mode`; `Scrolling` when `resolved.scrollbar.overlay_mode`, else `Always` |
| `src/colors.rs:285-286` | no fields `bullish`, `bearish` | `chart_bullish`, `chart_bearish`, same sources |
| `src/colors.rs:379` | no field `accordion_hover` | delete |
| `src/config.rs:84, 151-152` | same three on `ThemeConfigColors` / `ThemeColor` | delete one, rename two |
| `src/icons.rs:179, 289, 673` | no variant `IconName::GitHub` | `IconName::Github` |

Tripwire `src/colors.rs:869-877`: 108 → 139. Doc comments listed in §13.7.

### 5.2 Library, second pass (icons)

Three exhaustive tables report 15 uncovered variants; §10 gives every entry.
`ALL_ICON_NAMES` gains 15; tripwire 86 → 101.

### 5.3 Config

`ThemeConfigColors` carries every `ThemeColor` field except the twelve named
palette colours, plus one field with no `ThemeColor` counterpart,
`group_box_title_foreground`, which stays `None` (139 − 12 + 1 = 128 config
fields, `schema.rs:249`). `theme_color_to_config_colors`
exports hex for all 34 new or renamed fields. `to_theme_config` exports the
**scaled** font sizes (§3.4) so `Theme::change` reproduces them. Colours whose alpha is below 1 (`overlay`, `drag_border`, `drop_target`) are exported as `#rrggbbaa`, which gpui's `Rgba::try_from` parses (gpui-pre 0.3.3 `src/color.rs:224-262`) and gpui-component's `try_parse_color` accepts (`src/theme/color.rs:677-680`); opaque colours stay `#rrggbb` (D36). The config also carries upstream's default highlighter style for its mode (`highlight: Some(HighlightTheme::default_<mode>().style.clone())`), so a `Theme::change` to that mode switches code highlighting as it switches colours; `to_theme` sets `Theme.highlight_theme` to the same default directly, and `apply_config` would otherwise keep the previous mode's highlighter (D41).

### 5.4 Showcase (31 first-pass errors)

Documented renames, applied first:

| Old | New | Source |
|-----|-----|--------|
| `divider::Divider`, `Divider::horizontal()` | `separator::Separator`, `Separator::horizontal()` | release notes |
| `table::Table` (stateful) | `table::DataTable`; `Column`, `TableDelegate`, `TableState` unchanged | release notes |
| `gpui_component_assets::Assets` | `gpui_kit::assets::Assets` | gpui-kit README |
| `InputState::new(..).auto_grow(4, 30)` + `Input::new(&state)` | `TextareaState::new(..).auto_grow(4, 30)` + `Textarea::new(&state)` | release notes |
| `Dialog::new(window, cx)`, `.confirm(..)` | `Dialog::new(cx)` with `DialogHeader`/`DialogTitle`/`DialogDescription`/`DialogFooter` | release notes |
| `theme.scrollbar_show` | `theme.scrollbar_mode` | 0.6.0 `Theme` |
| `theme.accordion_hover`, `.bullish`, `.bearish` | removed / `chart_bullish`, `chart_bearish` | 0.6.0 `ThemeColor` |
| `theme.list` as a colour | `theme.colors.list` (`Theme.list` is now `ListSettings`) | 0.6.0 `Theme` |
| `IconName::GitHub` | `IconName::Github` | generated name |
| `Application::new()` | `gpui_kit::application()` | gpui-kit `src/lib.rs:132` |
| `gpui_component::init(cx)` | `gpui_kit::init(cx)` | gpui-kit `src/lib.rs:139-144` |

Drift from the gpui snapshot: `gpui::Timer` removed (5 uses → executor
timer); `gpui::Menu` gained `disabled` (4 literals).

gpui-component changes not in the release notes, with showcase lines:
`AppMenuBar::new` takes one argument (`:1592`); `Progress::new` takes an
argument (`:2996`, `:3003`, `:3010`); `Sidebar::left` takes two (`:3978`);
`Dialog::confirm` gone (`:4143`); `BarChart::x` gone (`:4478`);
`gpui_component::PixelsExt` no longer exported (`:39`);
`TableDelegate::column` takes `Column` by value (`:834`).

The three `*Theme::global_mut(cx) = theme;` sites become
`native_theme_gpui::apply(..)`; the system path uses `apply_system_theme`;
widgets with a geometry builder use it; the Color Map tab shows 139 fields.

### 5.5 CI and publish

- `ci.yml`: re-verify the apt list against the `gpui-pre-linux` /
  `gpui-pre-platform` build scripts; extend only if demanded.
- `publish.yml`: G11's cause is gone (naga 29.0.4, codespan-reporting
  0.13.1). Run `cargo check --workspace --all-targets` on the new lock; if
  green, remove `continue-on-error: true` from the four connector steps
  (clippy, test, documentation, publish) and the two comments. Otherwise record the new reason in place of G11.
- `screenshots.yml` builds the example on macOS and Windows; the
  `test-support` dev feature also enables gpui-pre's `wayland` and `x11`
  features (`Cargo.toml:70-77`). Both are inert off Linux: `wayland = []`
  is an empty flag, `x11 = ["scap?/x11"]` only touches the optional
  screen-capture crate, and the Linux platform crate that reads them is
  target-gated (gpui-pre `Cargo.toml:78-80`; gpui-pre-platform
  `Cargo.toml:65`). The workflow run confirms it; should it fail anyway, the
  feature moves to a `cfg(target_os = "linux")` dev-dependency table.

---

## 6 -- The complete `ThemeColor` mapping delta

### 6.1 Rule

The native theme has one ordinary push button (`resolved.button.background_color`,
`hover_background`, `active_background`, `font.color`), already mapped to
`secondary*` (`src/colors.rs:254-260`, sources `:166-228`); a primary button
(`primary_background`, `primary_text_color`), mapped to `primary*`; and four
status colour pairs, mapped to `danger*`, `info*`, `success*`, `warning*` with
`ensure_status_contrast` on the foreground. The 28 `button_*` fields take the
**same values** as the semantic fields their variant used in 0.5.1
(0.5.1 `src/button/button.rs:630-635, 924-929`). Nothing new is read; the
mapping restores 0.5.1's solid rendering under the 0.6.0 layout.

### 6.2 Table

| New field | Value | Note |
|-----------|-------|------|
| `button`, `button_hover`, `button_active`, `button_foreground` | `tc.secondary`, `tc.secondary_hover`, `tc.secondary_active`, `tc.secondary_foreground` | ordinary push button |
| `button_secondary` (+ `_hover`, `_active`, `_foreground`) | same as above | native themes do not distinguish default from secondary |
| `button_primary` (+3) | `tc.primary*` | |
| `button_danger`, `button_info`, `button_success`, `button_warning` (+3 each) | `tc.danger*`, `tc.info*`, `tc.success*`, `tc.warning*` | solid status button as in 0.5.1 |
| `chart_bullish` / `chart_bearish` | `c.success` / `c.danger` | renames |
| `status_bar` / `status_bar_border` | `rgba_to_hsla(resolved.status_bar.background_color)` / `(.border.color)` | direct |
| `table_foot` / `table_foot_foreground` | `tc.table_head` / `tc.table_head_foreground` | derivation: footer mirrors header; `ListTheme` has no footer field |

Removed: `accordion_hover`, `bullish`, `bearish`. Copies from other `tc`
fields run after their sources are assigned.

### 6.3 Why upstream's fallback is not enough

Unassigned fields are transparent black on the direct `Theme`
(`ThemeColor: Default`), and become gpui-component's tinted house style on
the `Theme::change` path (§1.2). Both are wrong for a native theme; hence the
explicit mapping in `colors.rs` **and** `config.rs`.

### 6.4 New test

`no_theme_color_field_is_left_at_default`: for one light and one dark preset
no field equals `Hsla::default()`; any legitimate exemption is listed with a
comment and recorded in the CHANGELOG.

---

## 7 -- Accessibility as an input

### 7.1 Breaking API changes (the complete list)

| Before | After | Why |
|--------|-------|-----|
| `to_theme(resolved, name, is_dark, reduce_transparency: bool)` | `to_theme(resolved, name, is_dark, prefs: &AccessibilityPreferences)` | the connector honoured one of four preferences; text scaling was silently dropped (`src/lib.rs:237-246`) |
| `from_preset(name, is_dark)` | `from_preset(name, is_dark, prefs: &AccessibilityPreferences)` | accessibility is orthogonal to theme choice: a user with large text wants it under a preset too; pass `&AccessibilityPreferences::default()` or `&AccessibilityPreferences::from_system()` (§11.2) |
| `lucide_name_for_gpui_icon`, `material_name_for_gpui_icon`, `freedesktop_name_for_gpui_icon`, all `-> &'static str` | `-> Option<&'static str>` | a set may have no equivalent; the project rule is "return None, never substitute" (§10.1); today `StarFill` in Lucide and `StarOff` in Material. The freedesktop table maps `Star` to `non-starred` instead of `starred`, so the two star states differ (§10.4) |
| Lucide bundle names `close`, `dash`, `inspect`, `resize-corner`, `sort-ascending`, `sort-descending`, `window-close`, `window-maximize`, `window-minimize`, `window-restore`, `trash-2` accepted by `LucideLoader::new(name)` | the Lucide names `x`, `minus`, `scan`, `grip`, `arrow-up-narrow-wide`, `arrow-down-wide-narrow`, `maximize`, `minimize-2`, `trash`; Material name `star_border` removed and `font_size` renamed `format_size` | the bundle mirrors upstream under upstream's names (§10.2); the duplicate file behind `star_border` drew the wrong glyph (§10.3); `font_size.svg` was upstream's `format_size` under a non-upstream stem (found by the refresh, rationale error 57) |
| dependency stack | gpui-component 0.6 / gpui-base 0.6 / gpui-pre 0.3 | §1 |

`from_system()` keeps its shape and passes `&sys.accessibility`.
`SystemThemeExt::to_gpui_theme` likewise. Everything else is additive.

### 7.2 What each preference does

| Preference | Effect | Receiver |
|------------|--------|----------|
| `text_scaling_factor` | `Theme.font_size`, `mono_font_size` and their `ThemeConfig` copies are multiplied by `s` (§3.4); geometry text sizes and control heights (§9.4); re-applied at runtime by `apply_accessibility` (§8.1) | `Theme.font_size` → rem (`root.rs:579`) |
| `reduce_motion` | `cx.set_reduce_motion(v)` in `apply` / `apply_accessibility` | gpui-pre `App::set_reduce_motion` |
| `reduce_transparency` | unchanged: overlay alpha in `colors.rs:391-396` | `ThemeColor.overlay` |
| `high_contrast` | none; no GPUI or gpui-component receiver | §14 |

`s` is `prefs.text_scaling_factor` if finite and `> 0.0`, else `1.0`.

A runtime change of any preference goes through `apply_accessibility` (§8.1):
with a variant stored for the current mode it rebuilds the styled theme from
that variant with the new preferences, so scaling and transparency take effect
without the application keeping `resolved`; reduce-motion is forwarded in every
case.

---

## 8 -- Installation, global receivers, base layer

### 8.1 API

```rust
/// Installed by `apply`; one per App.
pub struct NativeTheme { /* light: Option<ResolvedTheme>, dark: Option<ResolvedTheme>,
                            accessibility: AccessibilityPreferences, reapplying: bool,
                            observer_installed: bool, last_is_dark: bool */ }
impl Global for NativeTheme {}
impl NativeTheme {
    /// The stored variant for the styled theme's current mode, if any.
    pub fn resolved(&self, cx: &App) -> Option<&ResolvedTheme>;
    pub fn accessibility(&self) -> &AccessibilityPreferences;
    /// Borrowed view for the geometry module.
    pub fn native(&self, cx: &App) -> Option<Native<'_>>;
}
pub trait ActiveNativeTheme { fn native_theme(&self) -> Option<&NativeTheme>; }
impl ActiveNativeTheme for App { .. }

/// Borrowed inputs of every geometry builder.
#[derive(Clone, Copy)]
pub struct Native<'a> { pub resolved: &'a ResolvedTheme, pub accessibility: &'a AccessibilityPreferences }
impl<'a> Native<'a> {
    /// Unscaled, no reductions (a static default instance).
    pub fn unscaled(resolved: &'a ResolvedTheme) -> Self;
}

/// Install `theme`; store `resolved` under the theme's mode; install a
/// `ThemeConfig` for the other stored variant (if any) under the same display
/// name, so `Theme::change` reproduces native colours in either mode; project
/// into gpui-base; override base scrollbar and resize-handle styles; forward
/// reduce-motion; install the re-apply observer once.
pub fn apply(theme: GpuiTheme, resolved: &ResolvedTheme, prefs: &AccessibilityPreferences, cx: &mut App);
/// `to_gpui_theme()` for the OS mode, storing both variants (and installing
/// both configs), then `apply`.
pub fn apply_system_theme(sys: &SystemTheme, cx: &mut App);
/// Runtime preference change: with a stored variant for the current mode,
/// rebuilds the styled theme from it with `prefs` and re-installs it through
/// `apply` (text scaling and transparency take effect); always forwards
/// reduce-motion and stores `prefs`.
pub fn apply_accessibility(prefs: &AccessibilityPreferences, cx: &mut App);

pub mod base_layer {
    #[derive(Debug, Clone, PartialEq)]
    pub struct ScrollbarGeometry {
        pub track_width: Pixels, pub thumb_width: Pixels, pub thumb_inset: Pixels,
        pub thumb_radius: Pixels, pub min_thumb_length: Pixels,
        pub track: Hsla, pub track_active_border: Hsla,
        pub thumb: Hsla, pub thumb_hover: Hsla, pub thumb_active: Hsla,
    }
    pub fn scrollbar_geometry(resolved: &ResolvedTheme) -> ScrollbarGeometry;
    pub fn scrollbar_styles(g: &ScrollbarGeometry) -> ScrollbarStyles;
    pub fn resizable_theme(resolved: &ResolvedTheme) -> ResizableTheme;
    /// Write both onto gpui_base::Theme, keeping mode() and motion().
    pub fn apply_overrides(g: &ScrollbarGeometry, r: ResizableTheme, cx: &mut App);
}
```

`apply` order: store → styled global → other stored variant's `ThemeConfig`
→ `sync_base` → `apply_overrides` → `set_reduce_motion` → observer once (plus,
in the update that installs it, one deferred re-write of `apply_overrides`,
D38) → `refresh_windows` (a change made outside an input event paints at
once, D37);
`reapplying` is set only by the observer's own writes (§3.3). Mode is still set on
the styled theme in `to_theme` and reaches the base through `sync_base`.

**No-panic guards.** `gpui_component::Theme::global` and `global_mut` call
GPUI's `cx.global::<T>()` / `cx.global_mut::<T>()` (`theme/mod.rs:172-176`),
which panic when the global is absent (gpui-pre `src/app.rs:2024-2028, 2040-2047`).
Therefore: `apply` first checks `cx.has_global::<gpui_component::Theme>()`
and, if false, calls `gpui_component::init(cx)`, which is what upstream
requires before any component use and which creates the global
(`theme/mod.rs:31-35, 237-247`); the observer reads the styled theme with
`cx.try_global` and falls back to the mode `apply` last installed when it is
absent; `NativeTheme` is read with `try_global` everywhere. No code path in
the connector calls a panicking accessor.

### 8.2 Scrollbar geometry mapping

Sources: `ResolvedScrollbarTheme { track_color, thumb_color, thumb_hover_color,
thumb_active_color: Option<Rgba>, groove_width, min_thumb_length, thumb_width, overlay_mode }`
(`thumb_active_color` is a `soft_option` field, `model/widgets/mod.rs:287`: it stays
`Option` after resolution and is unset in the four `*-live` presets),
`defaults.border.corner_radius`, `defaults.border.color`.

| Receiver (gpui-base `src/scrollbar.rs`) | Value | Kind |
|------|-------|------|
| `track.width` ×3 states (`:607`) | `px(groove_width)` | direct |
| `track.bg` ×3 (`:597`) | `track_color` | direct; upstream uses one colour for all states (`theme/mod.rs:281-283`) |
| `track_active.border_color` (`:602`) | `defaults.border.color` | mirrors upstream (`theme/mod.rs:283`); no scrollbar border colour in the theme |
| `thumb.bg` / `thumb_hover.bg` / `thumb_active.bg` (`:626`) | `thumb_color` / `thumb_hover_color` / `thumb_active_color` | direct; `thumb_active` falls back to `thumb_hover_color` when the theme leaves `thumb_active_color` unset, which is upstream's own choice for the active slot (`theme/mod.rs:291-295`; rationale error 58) |
| `thumb.width` ×3 (`:631`) | `px(thumb_width)` | direct; upstream widens the active thumb, the theme has one width |
| `thumb.inset` ×3 (`:636`) | `px(((groove_width − thumb_width) / 2).max(0))` | **derivation**: centres the fill, which is anchored `inset` from the outer edge (`:1391-1400`) |
| `thumb.radius` ×3 (`:641`) | `px(defaults.border.corner_radius.max(0))` | mirrors upstream (`theme/mod.rs:284-293`); no scrollbar radius in the theme or in platform-facts |
| `thumb.min_length` ×3 (`:646`) | `px(min_thumb_length)` | direct |

### 8.3 Resize handle, focus ring, mode

- `ResizableTheme { handle: Some(splitter.divider_color), active_handle: Some(splitter.hover_color) }`
  (upstream projects `border` / `drag_border`, `theme/mod.rs:296-298`).
  Handle width is a constant (`resize_handle.rs:12`), Tier U. In every
  shipped preset both splitter colours inherit the border colour (§1.3), so
  the projection differs from upstream's only in `active_handle` (upstream:
  the translucent `drag_border`); a reader or user theme that sets
  `[splitter]` changes both.
- `Theme.focus_ring = defaults.focus_ring_width > 0.0`. When false upstream
  paints a tinted border (`src/styled.rs:182-184`); ring width is derived
  from the element's border (`:188-215`), so `focus_ring_width` /
  `focus_ring_offset` keep their helper functions and no receiver.
- `Theme.scrollbar_mode`: `Scrolling` when `overlay_mode`, else `Always`
  (existing rule).

### 8.4 Untouched

`Theme.notification`, `tile_*`, `list.active_highlight`, `sheet.margin_top`,
`motion`: no theme source. `Theme.transparent` stays as `Theme::from` sets it.

---

## 9 -- Per-widget geometry: the `geometry` module

### 9.1 Design

Every builder is pure: `fn <widget>(n: Native<'_>) -> StyleRefinement`, built
from `StyleRefinement::default()` with `Styled` setters, applied by the
application with `refine_style`:

```rust
use gpui_component::StyledExt;
let button = Button::new("save").label("Save");
// On the preset path without `apply`: `Some(Native::unscaled(&resolved))`.
match cx.native_theme().and_then(|t| t.native(cx)) {
    Some(n) => button.refine_style(&geometry::button(n)),
    None => button,
}
```

Text setters go into the refinement's `text` field and cascade through GPUI's
text-style inheritance unless an inner element sets its own size (the tables
say where that happens).

### 9.2 Widget table

Citations gpui-component 0.6.0. "Hardcoded → Refine": the upstream line the
refinement overrides and the line where the caller's style is applied after it.

| Function | Sources (`resolved.`) | Setters | Hardcoded → Refine | Not covered |
|----------|------------------------|---------|--------------------|-------------|
| `button` | `button.min_height`, `.min_width`, `.border.padding_horizontal`, `.padding_vertical`, `.corner_radius`, `.line_width`, `.color`, `button.font`, `defaults.line_height` | `h` (§9.4 height), `min_w`, `px`, `py`, `rounded`, `border`, `border_color` | `button.rs:587-601`, `:605-624` → `:650` | label text size is set on the inner content (`:658-666`, `text_base` = rem = `font_size` at Medium/Large/custom), so `button.font.size` is honoured only when it equals `defaults.font.size`; `icon_text_gap` is inner (`:658-660`) |
| `input` | `input.min_height`, `input.border.corner_radius`, `.line_width`, `.padding_vertical` (height floor via `control_height`), `input.font`, `defaults.line_height` | `h`, `rounded`, `border`, `text_size`, `font_weight` | `input.rs:572` (text), `:576` (height), `:580`, `:582` → `:587`; `Input::h` (`:232`) equivalent for height | padding lives in the inner editor; Tier U |
| `menu_item` | `menu.row_height`, `menu.border.padding_*`, `menu.icon_text_gap`, `menu.font`, `defaults.line_height` | `h`, `px`, `py`, `gap_x`, `text_size`, `font_weight` | `menu/menu_item.rs:101-103` → `:109` | only application-built `MenuItem`s; `PopupMenu` builds its own (`popup_menu.rs:749`) |
| `list_item` | `list.row_height`, `list.border.padding_*`, `list.item_font`, `defaults.line_height` | `h`, `px`, `py`, `text_size`, `font_weight` | `list/list_item.rs:188-190` → `:196` | |
| `tooltip` | `tooltip.max_width`, `tooltip.border.padding_*`, `.corner_radius`, `tooltip.font` | `max_w`, `px`, `py`, `rounded`, `text_size`, `font_weight` | `tooltip.rs:120-125` → `:126` | only application-built `Tooltip::new` (`:43`); `Button::tooltip(text)` is internal (`button.rs:360`) |
| `popover` | `popover.border.padding_*`, `.corner_radius` | `px`, `py`, `rounded` | `popover.rs:284` → `:312` | |
| `status_bar` | `status_bar.border.padding_*`, `status_bar.font` | `px`, `py`, `text_size`, `font_weight` | `status_bar.rs:87-89` → `:95` | |
| `dialog` | `dialog.border.padding_*`, `dialog.min_height`, `dialog.max_height` | `px`, `py`, `min_h`, `max_h` | paddings default `px(16.)` (`dialog.rs:502`) and are read from `self.style.padding` (`:503-512`); `min_h_24()` (`:578`) → `:582` | width via builder (§9.3) |
| `dialog_footer` | `dialog.button_gap` | `gap` | `dialog/footer.rs:47` → `:51` | |
| `dialog_title` | `dialog.title_font` | `text_size`, `font_weight` | `dialog/title.rs:42-43` (`text_base().font_semibold()`) → `:45` | |
| `dialog_description` | `dialog.body_font` | `text_size`, `font_weight` | `dialog/description.rs:49` (`text_sm()`) → `:51` | |
| `table` (declarative `Table`) | `list.item_font` | `text_size`, `font_weight` | `table/table.rs:111` (`text_sm()`) → `:114` | header font (`list.header_font`) and `DataTable` were not examined; row height and padding are inner |
| `progress` | `progress_bar.track_height`, `progress_bar.border.corner_radius`, `progress_bar.min_width` | `h`, `rounded`, `min_w` | `progress.rs:128-129` → `:130`; the fill copies the caller's radii (`:92-94, 137, 147`) | |
| `group_box_content` | `card.border.padding_*`, `.corner_radius`, `.line_width`, `.color` | `px`, `py`, `rounded`, `border`, `border_color` | `group_box.rs:156-160` → `:161` via `GroupBox::content_style` (`:105`) | |
| `accordion_title` | `expander.header_height` | `h` | `accordion.rs:300-303` → `:306` via `AccordionItem::title_style` (`:219`) | arrow icon sized by `Size` (`:281-282`) |
| `checkbox` | `checkbox.label_gap`, `checkbox.font` | `gap`, `text_size`, `font_weight` | `checkbox.rs:256` → `:271` (also re-applied in the disabled state, `:232-240`, harmless) | indicator size (`:195-199`) Tier U |
| `radio` | `checkbox.label_gap`, `checkbox.font` | `gap`, `text_size`, `font_weight` | `radio.rs:196` → `:211` | platform-facts §2.5 defines radio metrics as the checkbox's with a circular indicator (`platform-facts.md:947, 969, 1210`); indicator (`:216-220`) Tier U |
| `select` / `combobox` | `combo_box.min_height`, `.min_width`, `combo_box.border.corner_radius`, `.padding_vertical` (height floor via `control_height`), `combo_box.font`, `defaults.line_height` | `min_h`, `min_w`, `rounded`, `text_size`, `font_weight` | `select.rs:479-486` → `:490`; `combobox.rs:981-988` → `:992` | arrow size/area inner |
| `title_bar` | `window.title_bar_font` | `text_size`, `font_weight` | `title_bar.rs:334` → `:342` | height overridable but no theme field |

### 9.3 Size and builder helpers

| Function | Value | Seam |
|----------|-------|------|
| `spinner_size` | `Size::Size(px(spinner.diameter))` | `spinner.rs:53-65` → `Icon`, `icon.rs:160, 189` |
| `icon_size_toolbar` / `_small` / `_large` / `_dialog` / `_panel` | `Size::Size(px(defaults.icon_sizes.<field>))` | `icon.rs:160, 189` |
| `dialog_max_width` | `px(dialog.max_width)` for `Dialog::max_w` | `dialog.rs:393` |
| `input_height` | control height (§9.4) for `Input::h` | `input.rs:232` |

`Size::Size` is not used for Button (`button.rs:588` scales padding only),
Checkbox (`checkbox.rs:199`) or Switch (`switch.rs:136-142`); Progress uses
the refinement seam because it also carries the radius exactly.

### 9.4 Derivations in this module

- **Text size**: `px(font.size × s)`; **weight**: `FontWeight(font.weight as f32)`.
- **Control height** (`button`, `input`, `menu_item`, `list_item`, `input_height`;
  as `min_h` for `select`/`combobox`):
  `h = max(min_height_or_row_height, ceil(font.size × s × defaults.line_height) + 2 × padding_vertical)`.
  At `s = 1` this is the theme's own height wherever the platform's height
  already accommodates its text, which is the normal case (kde-breeze
  button at 96 dpi: text 19 + padding 12 = 31 against a minimum of 32;
  adwaita: 18 + 10 = 28 against 34); it grows only when scaled text would not
  fit (the same buttons at `s = 1.5`: 40 and 37). Inputs are theme values and
  the accessibility factor; the rule is how toolkits size controls (font
  metrics plus margins).
- No other arithmetic. `radio` reusing `checkbox` is a platform fact, not a
  derivation.

### 9.5 Layout accessors

`widget_gap`, `container_margin`, `window_margin`, `section_gap` take
`&LayoutTheme` and return `Option<Pixels>`; the input is `Theme::layout` on
the preset path and `SystemTheme.layout` (§11.1) on the system path. No
gpui-component widget reads the gpui-base spacing tokens, so there is no
receiver to map them into.

### 9.6 Tests

Per builder, for one light and one dark preset and for `s ∈ {1.0, 1.5}`:
read the public `StyleRefinement` fields (`size.height`, `min_size`,
`max_size`, `padding.*`, `corner_radii.*`, `border_widths.*`, `border_color`,
`gap`, `text.font_size`, `text.font_weight`) and compare with values built by
the same constructors. Height tests cover both branches of the `max`.

---

## 10 -- Icons

### 10.1 Tables return `Option`

`lucide_name_for_gpui_icon`, `material_name_for_gpui_icon` and the
freedesktop table return `Option<&'static str>`: `Some(name)` only when the
named file exists in that bundle (or, for freedesktop, when the name is a
documented theme name), `None` when the set has no equivalent. Tests assert
that every `Some` resolves through `bundled_icon_to_image_source`, and that
every `None` is listed in a documented allow-list with a reason. Today the
`&'static str` contract forces a name for every variant even where the set
has no equivalent; two cases were found: `StarOff` in Material, served by a
duplicate hollow star (§1.3), and `StarFill` in Lucide, which the first draft
served with the hollow `star`; either shows one glyph for two states. `Option`
makes the contract true, and the names the Lucide table returns become real
Lucide names (§10.2).

### 10.2 Lucide

Bundle upgraded to **Lucide 1.41.0** (2026-09-04, the latest release) and
stored under **Lucide's own file names**, so the directory is a subset
mirror of `lucide-icons/lucide/icons` at one tag with one exception:

- The ten files stored under gpui-component's names (§1.3) are byte-identical
  duplicates of files the bundle already holds under Lucide's names (`x.svg`,
  `minus.svg`, `scan.svg`, `grip.svg`, `arrow-up-narrow-wide.svg`,
  `arrow-down-wide-narrow.svg`, `maximize.svg`, `minimize-2.svg`). The ten are
  deleted: `close.svg`, `window-close.svg`, `dash.svg`, `window-minimize.svg`,
  `inspect.svg`, `resize-corner.svg`, `sort-ascending.svg`,
  `sort-descending.svg`, `window-maximize.svg`, `window-restore.svg`. Nothing
  is renamed.
- `trash-2.svg` becomes `trash.svg`, the name Lucide 1.x made canonical; the
  glyph is identical. The three role-table paths that name it
  (`bundled.rs:137, 163-164`) are updated.
- `github.svg` is kept from tag 0.577.0 as the single per-file exception:
  Lucide removed all brand icons in 1.x and offers no replacement. The file
  is a Lucide file under Lucide's ISC licence; the licence does not expire
  when upstream deletes the icon.

The connector's Lucide table returns the real names (`Close` and
`WindowClose` → `x`, `Dash` and `WindowMinimize` → `minus`, `Inspect` →
`scan`, `ResizeCorner` → `grip`, `SortAscending` → `arrow-up-narrow-wide`,
`SortDescending` → `arrow-down-wide-narrow`, `WindowMaximize` → `maximize`,
`WindowRestore` → `minimize-2`), which is what its name promises. The
connector does not use `trash-2` at all: `IconName::Delete` maps to Lucide's
`delete` (`icons.rs:167`), and `trash-2.svg` is used only by native-theme's
role tables for `ActionDelete`, `TrashEmpty` and `TrashFull`
(`bundled.rs:137, 163-164`), which are what the rename to `trash.svg`
touches. File count: Lucide 103 − 10 duplicates + 14 new = 107 (the
`trash-2` → `trash` rename is count-neutral); Material 87 − 1 (`star_border`)
+ 14 new = 100. The 14 new Lucide files come from the same tag.

| Variant | Lucide name | Action |
|---------|-------------|--------|
| `Battery`…`BatteryWarning` (6) | `battery`, `battery-charging`, `battery-full`, `battery-low`, `battery-medium`, `battery-warning` | add |
| `Cpu`, `FileText`, `HardDrive`, `MemoryStick`, `Network`, `Pause`, `Play`, `RotateCw` | same names | add |
| `StarFill` | — | `None`: Lucide ships no filled star (`star-fill`, `star-filled` absent at 1.41.0, §1.3); gpui-kit's own `star-fill.svg` is Lucide's `star` with `fill="currentColor"` added, which a bundled Lucide file cannot express; the hollow `star` would make `Star` and `StarFill` indistinguishable (rationale §2.21, D39) |

### 10.3 Material

Bundle style: Material Symbols **Outlined, 24px, default weight, no fill**,
path `symbols/web/<name>/materialsymbolsoutlined/<name>_24px.svg`. All
existing Material files are refreshed from upstream HEAD `0cbb08816df0`
(2026-09-04; the manifest records the full SHA
`0cbb08816df07faaae3dca060d4ebb10b66c214f`) so the bundle is one revision; the 14 new files come from the
same commit. Existence of every file below was verified.

| Variant | Name | Confidence | Action |
|---------|------|------------|--------|
| `Battery` | `battery_0_bar` | close | add |
| `BatteryCharging` | `battery_charging_full` | exact | add |
| `BatteryFull` | `battery_full` | exact | add |
| `BatteryLow` | `battery_2_bar` | close (one of three Lucide bars ↔ two of six) | add |
| `BatteryMedium` | `battery_4_bar` | close (two of three ↔ four of six) | add |
| `BatteryWarning` | `battery_alert` | exact | add |
| `Cpu` | `memory` | close | add |
| `FileText` | `description` | exact | bundled |
| `HardDrive` | `hard_drive` | exact | add |
| `MemoryStick` | `memory_alt` | close (a RAM module with pins, like Lucide's glyph; `sd_card`, the first choice, is a flash card) | add |
| `Network` | `lan` | close | add |
| `Pause` | `pause` | exact | add |
| `Play` | `play_arrow` | exact | add |
| `RotateCw` | `rotate_right` | exact | add |
| `StarFill` | `star_fill1` (from `star_fill1_24px.svg`) | exact | add |
| `Star` | `star` | exact (hollow star) | unchanged |
| `StarOff` | — | none: Material Symbols has no star-off glyph | `None`; `star_border.svg`, a duplicate of `star.svg` that backed this variant, is removed |

The `close` and `approximate` rows were checked visually against the gpui-kit
glyphs on 2026-09-06 (rendered side by side with the alternatives named in
the rationale): `battery_0_bar`, `battery_2_bar`, `battery_4_bar`, `memory`
and `lan` hold; `MemoryStick` moved from `sd_card` to `memory_alt`, whose
glyph is a RAM module with pins like Lucide's, whereas `sd_card` is a flash
card (rationale error 56).

### 10.4 Freedesktop

Names without the `-symbolic` suffix, as in the existing table; the Breeze
and Adwaita resolution tests are the gate.

| Variant | KDE / Breeze | GNOME / Adwaita | Confidence |
|---------|--------------|-----------------|------------|
| `Battery` | `battery` | `battery` | exact |
| `BatteryCharging` | `battery-100-charging` | `battery-full-charging` | close |
| `BatteryFull` | `battery-100` | `battery-full` | exact |
| `BatteryLow` | `battery-020` | `battery-low` | close |
| `BatteryMedium` | `battery-050` | `battery-good` | close |
| `BatteryWarning` | `battery-010` | `battery-caution` | approximate / close |
| `Cpu` | `cpu` | `computer` | exact / approximate |
| `FileText` | `text-x-generic` | `text-x-generic` | exact |
| `HardDrive` | `drive-harddisk` | `drive-harddisk` | exact |
| `MemoryStick` | `memory` (`devices/64/memory.svg`, a RAM module) | `media-flash` | exact / approximate |
| `Network` | `network-workgroup` | `network-workgroup` | close |
| `Pause` / `Play` | `media-playback-pause` / `media-playback-start` | same | exact |
| `RotateCw` | `object-rotate-right` | `object-rotate-right` | exact |
| `StarFill` | `starred` | `starred` | exact: the filled "starred" state |
| `Star` (existing row, changed) | `non-starred` | `non-starred` | close: the hollow star, the "not starred" state, which is what Lucide's `star` draws; the table had `starred`, the filled star, which `StarFill` now takes. `StarOff` keeps `non-starred`, the state it means; freedesktop names are semantic and have no slashed star (D39) |

### 10.5 Bundle provenance, manifest, generation

1. **`native-theme/icons/SOURCES.toml`** records, per set, the upstream
   repository, ref, path pattern and licence file, and per exceptional file
   its individual source. A test asserts every `.svg` under `icons/` is
   covered by a set rule or a per-file entry.
2. **`scripts/refresh-icons.sh`** re-downloads every file from the manifest,
   so refreshes and additions are reproducible.
3. **Generated name tables.** A `build.rs` in `native-theme` generates
   `lucide_svg_by_name` and `material_svg_by_name` from the directory
   listings (feature-gated as today, `rerun-if-changed` on the directories).
   The role-based tables stay hand-written. The 99-of-103 and 76-of-87 gaps
   disappear by construction.
4. **One exception, not eleven.** Because the ten duplicates are gone and
   every remaining file carries Lucide's name (§10.2), the Lucide set is described by one rule in the manifest
   (repository, tag `1.41.0`, `icons/{name}.svg`) plus one per-file entry:
   `github.svg` pinned to tag `0.577.0`. The Material set is one rule
   (repository, commit `0cbb08816df0`, the Outlined 24px path pattern) plus
   one per-file entry for `star_fill1.svg`, whose upstream file stem carries
   the `_fill1` suffix before `_24px`. The hand-written
   `lucide_by_name_covers_gpui_icons` test (`bundled.rs:508`) is retired in
   favour of the generated tables and the manifest coverage test.
5. The role-based tables are untouched; no native `IconRole` maps to a new
   variant.

### 10.6 Tests

Existing table tests adapt to `Option`; new: each of the 15 variants
resolves in Lucide and Material except the two documented gaps (`StarFill` in
Lucide, `StarOff` in Material), which the allow-lists name; `Star` and
`StarFill` map to distinct freedesktop names; the manifest coverage test; the
generated tables compile against the directories.

---

## 11 -- Additive changes in `native-theme`

### 11.1 `SystemTheme.layout: LayoutTheme`

Approved 2026-08-10 (`docs/todo.md`). Field-wise merge of the reader's layout
over the resolved preset's layout, the same precedence the pipeline already
uses for colours; both sources exist at `pipeline.rs:46, 120-126`.
`SystemTheme` derives `Clone` and `Debug` (`lib.rs:368`), both of which
`LayoutTheme` already implements. The todo item is closed.

### 11.2 `AccessibilityPreferences::from_system()`

An extraction, not new detection: on Linux it runs the same KDE/GNOME reader
code that fills the struct today (`kde/mod.rs:51`, `gnome/mod.rs:194`,
portal reads through `pollster` as `from_system` does); `reduce_motion` is
the reader's value where a reader supplies one, OR-ed with
`detect::detect_reduced_motion()` (`detect.rs:662`, the uncached variant:
`from_system()` is a fresh read every call, and the cached
`prefers_reduced_motion()` would return its first answer for the process
lifetime), a floor for the paths that leave `reduce_motion` at its default
(non-KDE/GNOME Linux, and macOS or Windows builds without their reader
feature; with the feature on, those readers fill all four fields, §1.3); the
fallback never turns a preference off; fields no reader supplies keep their
defaults. It exists so the preset path can honour system preferences without
resolving a full `SystemTheme`.

### 11.3 Not changed, and why

- **No `RadioTheme`.** platform-facts §2.5 defines radio metrics as the
  checkbox's; a struct would duplicate the same facts under a second name.
- **No `ScrollbarTheme.thumb_radius`.** platform-facts has no scrollbar
  radius; adding the field means research per platform, recorded as a todo
  item. Until then the connector mirrors upstream's choice (§8.2).
- **`LayoutTheme` stays `Option<f32>`; no resolved `f32` layout on
  `ResolvedTheme`.** Every static preset defines all four values, but a
  user theme may omit `[layout]`, and platform-facts §2.20 has no fallback
  for Windows at all and none for macOS `container_margin` or KDE
  `section_gap`. A resolved `f32` would have to invent those; `None` is the
  truthful value where the platform specifies nothing.

---

## 12 -- Testing strategy

| Gate | Kind | Proves |
|------|------|--------|
| `cargo check -p native-theme-gpui --all-targets` | compile | migration and showcase port complete |
| `theme_color_field_count_tripwire` (139); `no_theme_color_field_is_left_at_default` | headless | mapping complete |
| `all_icon_names_count_matches_gpui_component` (101); per-variant bundled `Some`; `None` allow-list; manifest coverage | headless | icons complete and honest |
| geometry builder tests (§9.6), including `s = 1.5` | headless | values and scaling correct |
| `scrollbar_geometry` tests | headless | widths, inset, min length, colours; `ScrollbarGeometry` derives `PartialEq + Debug` because `ScrollbarStyles` does not (`scrollbar.rs:589, 616, 655`) |
| `to_theme` scaling test | headless | `font_size == px(size × 1.5)`; config copies scaled |
| `hsla_to_hex_keeps_alpha_below_one`; the config test asserts the `drag_border` export has nine characters | headless | translucent colours survive the config round trip as `#rrggbbaa` (D36) |
| `#[gpui::test] apply_installs_and_survives_theme_change` | headless App | after `apply` alone (it initialises gpui-component itself, D29): styled mode as requested, base `scrollbar.mode()` as expected, `resizable.handle` and `resizable.active_handle` = the splitter colours, `cx.reduce_motion()` follows prefs; after `Theme::change` with the *same* mode (which rebuilds the base theme), run twice: `resizable.active_handle` equals the stored variant's `splitter.hover_color` after each change (proves the observer ran and left no stale `reapplying` flag) and the test returns (proves termination). The observable is `active_handle`, not `handle`: every preset inherits the divider colour from the border colour (§1.3), which is also what upstream writes into `handle`, whereas upstream writes the translucent `drag_border` into `active_handle`; the test asserts that precondition |
| `#[gpui::test] apply_then_change_in_the_same_update_keeps_overrides` | headless App | `apply` and `Theme::change` inside one `cx.update`, before the observer is active: the deferred re-write (D38) leaves `resizable.handle` at the stored variant's splitter colour |
| `#[gpui::test] observer_repairs_a_rebuild_merged_into_its_own_notification` | headless App | after `apply`, one update writes the base theme and defers a `Theme::change`: the change's notification is merged into the observer's own; the handle comparison repairs `active_handle` (D42) |
| `#[gpui::test] apply_without_stored_variant_falls_back` | headless App | observer fallback path (§3.3 step 2) |
| `#[gpui::test] apply_installs_configs_for_both_variants` | headless App | after `apply(dark)` then `apply(light)`: `Theme::change(Dark)` reproduces the dark variant's palette through the installed `ThemeConfig` (compared hex-for-hex, including a `button_*` field) and `highlight_theme.appearance` is `Dark` (D41) |
| `#[gpui::test] apply_accessibility_rescales_from_the_stored_variant` | headless App | after `apply`, `apply_accessibility` with factor 1.5: `font_size` and its config copy are scaled, `reduce_motion` forwarded, preferences stored |
| MSRV checks (§4.4) | toolchain | both floors true |
| `./pre-release-check.sh` | workspace | fmt, clippy, panic lint, package |
| screenshots workflow | visual | showcase renders on all three platforms with geometry applied |

---

## 13 -- Documentation, changelog, roadmap, CI

### 13.1 Connector README

Quick start with `apply_system_theme` and `from_preset(.., &prefs)` + `apply`;
"Per-widget geometry" with the `refine_style` idiom and §9.2's table;
"How re-application works" (§3.3, including the single-writer assumption and
the registry-name limit);
"Accessibility" (§7.2); "GPUI as `gpui-pre`" (§1.1 in plain terms, pinning
advice); the `init`-before-`apply` rule (D29); the GPUI surface of §4.2; compatibility line: gpui-component 0.6.x, gpui-base 0.6.x, gpui-pre 0.3.x, MSRV.

### 13.2 CHANGELOG `[0.5.8]`

- **Breaking** (native-theme-gpui): the rows of §7.1; `ScrollbarShow` →
  `ScrollbarMode`; `IconName::GitHub` → `Github`; connector `rust-version`.
- **Breaking** (native-theme): eleven Lucide bundle names stop resolving (ten
  were duplicates of files that already exist under Lucide's names, and
  `trash-2` is now `trash`, §10.2), `star_border` is removed and `font_size`
  is now `format_size` (its upstream name), so `LucideLoader::new` /
  `MaterialLoader::new` with the old names return `None`.
- **Added** (native-theme-gpui): `apply` (a `ThemeConfig` for every stored
  variant, so `Theme::change` reproduces native colours in both modes),
  `apply_system_theme`, `apply_accessibility` (rebuilds from the stored
  variant at runtime), `NativeTheme`, `ActiveNativeTheme`, `Native`,
  `base_layer`, `geometry`; text scaling; reduce-motion; focus-ring flag;
  the 15 new `IconName` variants covered in all three tables (`StarFill`
  has no Lucide equivalent and `StarOff` no Material one; both return
  `None`).
- **Added** (native-theme): `SystemTheme.layout`,
  `AccessibilityPreferences::from_system`; 28 bundled SVGs; `SOURCES.toml`;
  generated name tables.
- **Changed**: `ThemeColor` mapping 108 → 139; showcase on gpui-kit;
  dependency refresh (§4.3); workspace MSRV re-measured; the connector's
  docs.rs metadata keeps `all-features = true` and drops the three-target
  list (§4.1, D40), so the existing unreleased `[0.5.8]` entry that names
  three crates is amended to two.
- **Fixed**: icon bundle provenance recorded, Lucide refreshed to 1.41.0,
  Material refreshed to upstream HEAD, duplicate `star_border.svg` removed,
  the old Lucide `delete.svg` (a trash-can glyph) replaced by Lucide's real
  `delete` (§10, §1.3); the config hex export kept alpha (`#rrggbbaa`, D36); the
  `ThemeConfig` copies carried no highlighter style, so a `Theme::change` to
  the other mode kept the previous mode's code highlighting (D41); publish.yml
  soft gates removed if §5.5 passes.
- The existing docs.rs entry is amended as above. No migration guide
  (pre-1.0 rule).

### 13.3 ROADMAP

v0.6.2 becomes upstream-only: §14's Tier U rows, each phrased as a PR to
gpui-kit. The connector-side geometry the old text promised ships here.

### 13.4 `docs/todo_gpui-full-theme.md`

Gap table gains a "v0.5.8" column: delivered (tier) or upstream (citation).
The claim that gpui-component has no receiving fields is corrected.

### 13.5 `docs/todo.md`

Close: `SystemTheme` layout item. Add upstream PR candidates: styled `Theme`
`scrollbar_styles` override honoured by `base_theme()`; `Tab` keeps the
caller's height, radius and text size (its render re-sets them after
`refine_style`, `tab/tab.rs:800-808`); `Theme.shadow` honoured beyond `Button` /
`tokens.shadow` consumed; `Size::Size` honoured by Checkbox and Switch; inner
geometry (checkbox/radio indicator, switch, slider, separator thickness,
resize-handle width, button icon gap, input padding, popup-menu items,
select arrow, accordion arrow); `PopupMenu` item style hook; `Button::tooltip`
style hook; button label text size independent of rem; an iterable
`IconName::ALL` generated by `icon_named!` (the macro emits none, so
`ALL_ICON_NAMES` stays hand-written). Add research items: scrollbar thumb
radius per platform. (No research item for macOS/Windows accessibility
readers: both readers already fill all four fields, §1.3.)

### 13.6 Workspace

`Cargo.lock` regenerated; `publish.yml` per §5.5; `docs/archive/v0.5.7_gaps.md`
§G11 closure note; `docs/todo_v0.6.0_egui-connector-rationale.md:1079` and
`-spec.md:4717` cross-references to `gpui = "0.2.2"` updated.

### 13.7 Stale references outside the archive

| File | Line(s) | Stale text |
|------|---------|------------|
| `connectors/native-theme-gpui/Cargo.toml` | 30 | `gpui = "0.2.2"` |
| `connectors/native-theme-gpui/README.md` | 9 | "colors (108 fields)" |
| `connectors/native-theme-gpui/src/lib.rs` | 47, 109 | "108 color fields" |
| `connectors/native-theme-gpui/src/colors.rs` | 1, 3, 133 | "108 fields", "108-field" |
| `connectors/native-theme-gpui/src/config.rs` | 5 | "all 108 color fields" |
| `connectors/native-theme-gpui/src/icons.rs` | 64, 128, 138, 236, 360 | "gpui-component 0.5", "Covers all 86 … variants" |
| `connectors/native-theme-gpui/examples/showcase-gpui.rs` | 21, 435, 4871 | "108-field", "86 … variants", "all 86 icons" |
| `ROADMAP.md` | 52 | "108-field `ThemeColor` palette" |
| `docs/todo_v0.6.0_egui-connector-rationale.md` | 1079 | `gpui = "0.2.2"` |
| `docs/todo_v0.6.0_egui-connector-spec.md` | 4717 | `gpui = "0.2.2"` |
| `README.md` | 7 | MSRV badge `1.88.0` |
| `CONTRIBUTING.md` | 8 | "MSRV: **1.88.0**" |
| `docs/todo.md` | 83-87 | the MSRV CI item names the workspace floor `1.88.0` and only the egui connector's separate floor |

`CHANGELOG.md:879` is a past release entry and stays.

---

## 14 -- Limits

| Item | Why not in v0.5.8 |
|------|--------------------|
| Checkbox / radio indicator size | inner element sized by `Size` match; `Size::Size` falls to medium (`checkbox.rs:195-199`; `radio.rs:216-220`) |
| Switch track / thumb / radius | inner (`switch.rs:136-146`); refinement lands on a wrapper (`:168`) |
| Slider track / thumb | inner (`slider.rs:218, 288-289`); root refinement `:270` |
| Tab geometry | `Tab` applies its stored style (`tab/tab.rs:606` → gpui-base `tabs.rs:180`), but its render then re-sets height, radius and text size (`tab/tab.rs:800-808`), so those never take; `min_width` and outer padding would survive (follow-up); `TabBar`'s refinement (`tab_bar.rs:490`) styles the bar |
| Separator thickness | absolutely positioned inner line `px(1.)` (`separator.rs:79-84`); outer gets the refinement (`:137`) |
| Splitter divider width | `HANDLE_SIZE` constant (gpui-base `resize_handle.rs:12`) |
| Button icon-text gap; button label size ≠ body size | inner content row (`button.rs:658-666`) |
| Input padding | not on the root chain (`input.rs:566-587`) |
| Popup-menu rows; Button text tooltips | constructed internally (`popup_menu.rs:749`; `button.rs:360`) |
| Select/Combobox arrow; accordion arrow | inner / `Size`-sized (`accordion.rs:281-282`) |
| Segmented control | segmented `TabBar` is built from `Tab`s |
| Toolbar metrics | no gpui-component toolbar |
| Spinner stroke width | not a parameter of `Spinner`/`Icon` |
| Dialog minimum width | only fixed-width builders (`dialog.rs:381-387`) |
| Title-bar height | receiver exists (`title_bar.rs:334 → 342`), no theme field |
| Shadows off | `Theme.shadow` reaches only `Button`; six components hardcode `shadow_*()`; `tokens.shadow` unused |
| Typography scale | `TypographyTokens` drive only the markdown `TextView`; re-derived on every sync from `font_size` (`theme/mod.rs:353-360, 440-448`); `TextScale` has four named entries, not five steps |
| Spacing scale | no component reads `tokens.spacing` |
| Focus-ring width/offset | derived from the element's border (`styled.rs:188-215`) |
| High contrast | no GPUI or gpui-component receiver |
| Scrollbar thumb radius from the platform | no platform fact; upstream's `radius` mirrored |
| Corner radius after `Theme::change` | `ThemeConfig.radius` / `radius_lg` are `usize` in upstream's schema (`schema.rs:1093-1097`), so a fractional native radius is rounded to whole pixels once upstream re-applies the config; `to_theme` itself sets the exact value; no shipped preset has a fractional radius, so the loss is theoretical until one does |
| Body font weight and global line height | `Root` sets only family and size (`root.rs:579, 591`); `Theme` has no weight or line-height field; per-widget builders carry weight, and line height enters only the control-height rule |
| Button icon size | derived from the button's `Size` (`button.rs:541-542, 579-583`), inner |
| Disabled opacity | gpui-component styles disabled controls through colours (`button.rs:716-718`), not an opacity; `disabled_opacity` has no receiver |
| Popup-menu padding and radius | `PopupMenu` implements no `Styled` (grep); only `min_w` / `max_w` builders exist (`popup_menu.rs:401-408`), and the theme has no menu width |
| `DataTable` header font, table row height and padding | header not examined; rows are inner elements |
| Sidebar width and padding | width has no theme field; padding is on inner elements (`sidebar/mod.rs:432-479`) |
| Notification, sheet, tile, list highlight, motion durations | no theme source |

---

## 15 -- Implementation task list

Ordered; each step ends at a mechanical gate. The step-by-step plan with
tests, commands and per-task model routing is
[`todo_v0.5.8_gpui-component-0.6-plan.md`](todo_v0.5.8_gpui-component-0.6-plan.md).

1. **Dependency refresh** (§4.3) on `main` before the connector work:
   requirement bumps, `syn 3` attempt, `resvg 0.48.1`; per-member tests (the
   connector still on 0.5.1); workspace MSRV re-measured (§4.4).
2. **`SystemTheme.layout`** (§11.1) with a pipeline test.
3. **`AccessibilityPreferences::from_system()`** (§11.2) with a test.
4. **Generated icon name tables** (§10.5 item 3): `build.rs`,
   behaviour-preserving; the four Lucide names the hand-written table missed
   resolve.
5. **Icon bundles** (§10.2–10.5): ten duplicate deletions, `trash-2` →
   `trash` with its three role-table paths, `star_border` removal,
   `SOURCES.toml`, refresh script, refresh of both sets, 28 new files, visual
   check of the `close` / `approximate` Material rows. Gate:
   `cargo test -p native-theme`. Precedes the connector so that no commit
   leaves the connector library uncompilable (rationale §2.24).
6. **Connector on 0.6.0** (§4.1, §5.1, §5.2, §10.1–10.4, §10.6): manifest
   with provisional `rust-version = "1.95"`, mechanical first pass (about 14
   errors → 3 non-exhaustive matches), `Option` tables with the 15 new
   variants, `ALL_ICON_NAMES` 101, tripwire 139. Gate:
   `cargo test -p native-theme-gpui --lib` (the showcase compiles again at
   step 12).
7. **Colour mapping** (§6); default-field test.
8. **`to_theme` / `from_preset` API change and text scaling** (§7); config
   copies; tests.
9. **`base_layer`** (§8.2–8.3) with `ScrollbarGeometry` tests.
10. **`NativeTheme`, `apply` family, observer** (§8.1, §3.3) with the
    `#[gpui::test]`s of §12.
11. **`geometry`** (§9) with per-builder tests at `s ∈ {1.0, 1.5}`.
12. **Showcase port** (§5.4). Gate: compiles, runs on Linux;
    `screenshots.yml` green.
13. **Connector MSRV measurement** (§4.4).
14. **CI/publish** (§5.5).
15. **Docs** (§13). Gate: `./pre-release-check.sh`, link check.
16. **Release**: CHANGELOG date; tag and publish only on explicit approval;
    then confirm the docs.rs build of `native-theme-gpui` succeeds on the
    new stack (default target, as 0.5.7 did and as gpui-pre, gpui-component
    and gpui-kit do, §1.3; D40). Before tagging, `DOCS_RS=1 cargo doc -p
    native-theme-gpui --no-deps --all-features` runs locally as the
    warning-free check. If the published build fails, fix the cause in a
    patch release; never rewrite the tag.

---

## 16 -- Open questions

1. **Dialog minimum width.** Offer `dialog_width` returning `min_width` for
   `Dialog::w`, documented as a semantic approximation, or leave it out?
2. **`AccessibilityPreferences` in `to_theme` of the iced connector.** The
   same signature change applies there in v0.6.1; recorded for parity.
3. **Connector parity.** `apply`, `base_layer`, `geometry` have no iced
   counterpart until v0.6.1; the parity check will flag them; expected.
4. **Material `warning` and `info` provenance.** They match no current
   upstream variant; the refresh to HEAD replaces them with the current
   Outlined 24px files, which is the intended outcome, but the origin of the
   March files stays unexplained.
