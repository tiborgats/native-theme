---
gsd_state_version: 1.0
milestone: v0.5.7
milestone_name: API Overhaul
status: in-progress
stopped_at: Phase 94 Plan 02 complete (G7 ResolutionContext migration)
last_updated: "2026-04-20T00:06:04Z"
last_activity: 2026-04-20 — Phase 94 Plan 02 committed (dc03e53 RED regression tests for G7 ResolutionContext + 01d5b80 feat introduce ResolutionContext with from_system/for_tests; migrate 43 call sites + cc41fad docs CHANGELOG G7 + README migration examples). Replaces the `font_dpi: Option<f32>` parameter threaded through `ThemeMode::into_resolved` / `OverlaySource` / `run_pipeline` / `with_overlay` with a first-class `ResolutionContext` struct bundling font_dpi + button_order + icon_theme. No `impl Default` (runtime-detected types signal intent at the call site). `&ResolutionContext` parameter paired with `resolve_system()` zero-arg shortcut (not `Option<&T>` overload). Shortcut placed on ThemeMode, not Theme (deviation from gap doc §G7 step 4 because Theme has both light/dark variants). AccessibilityPreferences stays on SystemTheme per ACCESS-01 / J.2 B4 (render-time, not resolve-time concern). 43 call sites migrated across 18 files: 12 native-theme + 15 iced connector + 14 gpui connector + 2 root README. All 5 RED regression tests GREEN. 553 native-theme lib tests pass; 97 iced tests pass; 49 doctests pass. Zero-panic compliance verified on new production code. Parallel execution collision with concurrent plan 94-01 caused two destructive wipes of in-progress edits; recovered via atomic large commit immediately after re-applying. Deferred item: 94-01's BorderInheritanceInfo / FontInheritanceInfo structs have dead_code fields that fail clippy — will self-resolve when 94-01 GREEN consumes the registries. v0.5.7 is the no-backcompat window per REQUIREMENTS.md — no deprecation shim.
last_activity_prior: 2026-04-20 — Phase 93 Plan 09 committed (9a7ef3f RED regression test + 3132d7f refactor replace IconLoader with typed-per-set loaders + 870253d docs CHANGELOG+READMEs). Phase 93-03 had introduced a silent-ignore regression where IconLoader::new(name).set(Freedesktop).theme("Adwaita").load() silently dropped .theme() for string-name lookups, making the `gnome_names_resolve_in_adwaita` test fail on KDE Plasma (12 GNOME icons present in Adwaita but missing from Breeze). Masked by run_check_soft across 93-04..08. Plan 93-09 eliminates the bug class entirely by replacing IconLoader with five typed per-set loader structs (FreedesktopLoader, SfSymbolsLoader, SegoeIconsLoader, MaterialLoader, LucideLoader); calling a set-specific method on the wrong loader is now a compile error. Also refactored freedesktop::load_freedesktop_spinner to accept theme: Option<&str>, closing a second silent-drop bug. Full pre-release-check.sh line-by-line scan confirms green: every test suite "ok. N passed; 0 failed"; gpui lib goes from 151/1 to 152/0; native-theme lib goes from 791 to 794 (+3 regression tests). API-breaking refactor; no deprecation shim (v0.5.7 pre-publication). CHANGELOG Breaking Changes appended with migration table; both connector READMEs updated.
progress:
  total_phases: 30
  completed_phases: 29
  total_plans: 60
  completed_plans: 60
  percent: 100
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-12)

**Core value:** Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.
**Current focus:** v0.5.7 API Overhaul — roadmap complete, 20 phases (69–88) ready to plan

## Current Position

Phase: 94 — close-remaining-v0-5-7-gaps-g6-g8-border-inheritance-codegen-resolutioncontext-themereader-trait
Plan: 2/3 complete (94-02 done; 94-01 concurrent in-progress; 94-03 wave 2)
Status: in-progress
Last activity: 2026-04-20 — Phase 94 Plan 02 complete (G7 ResolutionContext). Three atomic commits: dc03e53 RED regression tests (5 failing tests locking the G7 contract) + 01d5b80 feat introduce ResolutionContext + migrate 43 call sites + cc41fad docs CHANGELOG G7 section + README migrations. Replaces font_dpi: Option<f32> parameter threading with first-class ResolutionContext struct (font_dpi + button_order + icon_theme); no `impl Default` (signal-intent constructors `from_system()`/`for_tests()`); `&ctx` parameter with `resolve_system()` shortcut (not None-overload); shortcut on ThemeMode, not Theme (unambiguous variant selection). Pipeline builds ONE ctx per invocation; reader.font_dpi overrides ctx.font_dpi when Some (preserves KDE forceFontDPI). OverlaySource.font_dpi → context (internal refactor). Prelude count 7 → 8. 553 native-theme tests pass; 97 iced tests pass; 49 doctests pass. Parallel-execution collision with plan 94-01 twice wiped in-progress edits; recovered via atomic large commit. Deferred item: 94-01's `BorderInheritanceInfo` / `FontInheritanceInfo` dead_code fields fail clippy `-D warnings` — self-resolves when 94-01 GREEN consumes the registries.

Progress: [█████████▒] 95% (62/63 plans complete — Phase 94 adds 3 plans; 94-02 done, 94-01 in-progress concurrent, 94-03 wave 2)

## Accumulated Context

### Decisions (carried from v0.5.6)

All decisions logged in PROJECT.md Key Decisions table.

v0.5.6 decisions worth carrying into v0.5.7 planning:

- [Phase 61-01]: detect.rs narrow pub(crate) accessors pattern established
- [Phase 62-01]: `pub(crate)` visibility on resolve::validate for macro-generated code
- [Phase 62-02]: impl_merge! supports repeated optional_nested blocks for mixed border categories
- [Phase 62-03]: validate.rs split into validate_helpers.rs + per-widget check_ranges() methods
- [Phase 63-01]: `_pure` suffix convention for I/O-free parse functions (from_kde_content_pure)
- [Phase 65-01]: ThemeWatcher Debug derive; notify crate Linux-only target section
- [Phase 66-01]: zbus direct dep with blocking-api feature; watch feature gates notify + zbus
- [Phase 66-02]: cfg-gated match arms per DE feature
- [Phase 67-01]: Box<dyn FnOnce() + Send> platform_shutdown for immediate wakeup on Drop
- [Phase 67-02]: GetCurrentThreadId via oneshot channel for PostThreadMessageW(WM_QUIT)
- [Phase 68]: Raw string literals br##"..."## for test SVGs containing # hex colors

### v0.5.7 Roadmap Summary

**Source documents** (pre-milestone design work, 9,700+ lines, six verification passes):

- `docs/todo_v0.5.7_native-theme-api.md` — API critique §1-§33 (doc 1)
- `docs/todo_v0.5.7_native-theme-api-2.md` — Bugs/structural/API-shape §A-§M (doc 2)

**Roadmap:** 20 phases (69–88), 55 requirements, 100% coverage, granularity=fine.

**Ship unit mapping:**

- Phase 69 — Unit 1 atomic: BUG-03 + BUG-04 + BUG-05 (resolver button_order unlock)
- Phase 70 — Unit 3 atomic: ERR-01 + CLEAN-01 (drop Error::Clone)
- Phase 71 — Unit 2 atomic: BUG-01 + BUG-02 + ERR-02 (validation split + Error restructure)
- Phase 72 — Unit 4 (after Unit 1): CLEAN-02 (ENV_MUTEX test simplification)
- Phase 73 — Unit 5: WATCH-01 + WATCH-02 (ThemeChangeEvent cleanup)
- Phase 74 — Unit 6 part A: COLOR-01 + POLISH-03 (Rgba polish + must_use uniformity)
- Phase 75 — Unit 6 part B: LAYOUT-02 + WATCH-03 + ICON-05 (non_exhaustive + compile-gate + IconSet::default removal)
- Phase 76 — Unit 7 part A: NAME-01 + LAYOUT-01 (type rename + crate root partition)
- Phase 77 — Unit 7 part B: MODEL-03 + MODEL-06 (pick(ColorMode) + icon_set relocation)
- Phase 78 — Unit 8 atomic: MODEL-02 + ACCESS-01 + ACCESS-02 (OverlaySource + AccessibilityPreferences + font_dpi)
- Phase 79 — Unit 9: BORDER-01 + CLEAN-03 + READER-02 (border split + reader visibility audit)
- Phase 80 — Unit 10: MODEL-01 + VALID-01 + VALID-02 + BORDER-02 (native-theme-derive proc-macro K codegen)
- Phase 81 — Unit 11 atomic: FEATURE-01 + FEATURE-02 + FEATURE-03 (feature-matrix cleanup)

**Non-ship-unit bundles:**

- Phase 82 — Icon API rework: ICON-01, ICON-02, ICON-03, ICON-04, ICON-06, ICON-07
- Phase 83 — Detection cache layer: DETECT-01, DETECT-02
- Phase 84 — Reader output contract homogenisation: READER-01
- Phase 85 — Data model method and doc cleanup: MODEL-04, MODEL-05, NAME-02, NAME-03
- Phase 86 — Validation and lint codegen polish: VALID-03, VALID-04
- Phase 87 — Font family Arc<str> and AnimatedIcon invariants: LAYOUT-03, LAYOUT-04
- Phase 88 — Diagnostic and preset-polish sweep: POLISH-01, POLISH-02, POLISH-04, POLISH-05, POLISH-06

- [Phase 83-01]: parse_linux_desktop kept pub (not pub(crate)) because native-theme-build tests import it from external crate
- [Phase 83-02]: ArcSwapOption<String> for icon_theme (not <str>) because arc-swap RefCnt requires Sized types
- [Phase 83-02]: detect_icon_theme_inner() delegates to model::icons::detect_icon_theme() to avoid pulling detection logic into detect.rs

- [Phase 84-01]: Box<ThemeMode> in ReaderOutput variants (Single.mode and Dual.light/dark) for clippy::large_enum_variant compliance
- [Phase 84-01]: theme_to_reader_output bridge for incremental reader migration; Plan 02 eliminates it
- [Phase 84-01]: run_pipeline 8 params with #[allow(clippy::too_many_arguments)] -- reader metadata passed separately alongside ReaderOutput
- [Phase 84-01]: overlay_tests return Result<()> for zero-panic test consistency
- [Phase 84-02]: ReaderResult struct replaces 8-param run_pipeline -- bundles ReaderOutput + name + icon_set + layout + font_dpi + accessibility
- [Phase 84-02]: from_kde_content_pure unchanged (pub, returns Theme tuple) -- integration tests depend on it
- [Phase 84-02]: preset_as_reader helper consolidates 10 fallback call sites into single function
- [Phase 84-02]: #[allow(dead_code)] on Dual variant -- only used in macOS-gated code, invisible on Linux
- [Phase 85-02]: FontSize doctest uses native_theme::theme::FontSize path (pub(crate) at crate root, pub via theme module)
- [Phase 85-02]: watch/mod.rs module example uses ? instead of .expect() for zero-panic compliance
- [Phase 86-02]: check_* helpers accept (prefix, field) separately; format! only in error branches for zero-alloc happy path
- [Phase 86-02]: Font nested checks inlined in generated code to avoid sub-prefix allocation
- [Phase 87-01]: FrameList newtype with custom Deserialize rejects empty arrays at deserialization boundary (T-87-01 mitigation)
- [Phase 87-01]: FramesData/TransformData wrapper structs make AnimatedIcon variant fields private outside the crate
- [Phase 87-01]: Duration constants remain u32 with NonZeroU32::new() at call site via ? (avoids hook-blocked .unwrap() in const context)
- [Phase 87-01]: frames_or_spin_fallback takes &'static [u8] for lifetime compatibility with include_bytes!()
- [Phase 87-02]: FontSpec::family and ResolvedFontSpec::family migrated from String to Arc<str>; serde rc feature enabled; .as_ref() for Arc<str>-to-&str comparisons

- [Phase 88-01]: DiagnosticEntry::DesktopEnv gated with #[cfg(target_os = "linux")] since LinuxDesktop only exists on Linux
- [Phase 88-01]: PlatformPreset.live_name() returns String (dynamic -live suffix append)
- [Phase 88-01]: ROADMAP SC-5 scoped to user-facing return values -- -live must exist in source for internal use
- [Phase 88-01]: Diagnostic feature labels use short names ("KDE", "Portal") for name() accessor consistency
- [Phase 88-02]: Theme.name as Cow<'static, str> with manual Default impl using Cow::Borrowed("")
- [Phase 88-02]: PRESET_DISPLAY_NAMES const table maps preset keys to display names for post-parse Cow::Borrowed replacement
- [Phase 88-02]: Connector showcases convert Cow to owned String via .into_owned() for local String fields
- [Phase 88-02]: ROADMAP SC-3 updated from preset("default") to preset("dracula") since no "default" preset exists

### Pending Todos

Phase 78 Plan 04 remaining (core crate compile fixes in gnome/mod.rs, pipeline.rs, detect.rs).

### v0.5.7 Decisions

- [Phase 71-01]: Kept Vec<String> for ResolutionIncomplete::missing (not Vec<FieldPath>) for Phase 71 compatibility
- [Phase 71-01]: Preserved From<toml::ser::Error> via ReaderFailed variant (presets::to_toml needs it)
- [Phase 71-01]: PlatformUnsupported uses &'static str (no Platform enum in crate yet)
- [Phase 71-02]: check_positive uses f64::MIN_POSITIVE for RangeViolation min bound
- [Phase 71-02]: Connector crates (gpui, iced) migrated to Error::ReaderFailed alongside core crate
- [Phase 72-01]: linux_preset_for_de promoted to pub(crate) for cross-module test access
- [Phase 72-01]: Added #[allow(dead_code)] on ENV_MUTEX pending Plan 02 removal
- [Phase 73-01]: Kept #[non_exhaustive] and wildcard arm in doc example despite single variant
- [Phase 73-01]: Updated on_theme_change() doctest to use ? instead of .expect() for zero-panic rules
- [Phase 74-02]: Remove #[must_use] from Result-returning fns (double_must_use lint); bare #[must_use] only on non-Result returns
- [Phase 75-02]: Inlined icon_set validation in validate.rs to avoid Default bound after removing Default from IconSet
- [Phase 76-01]: gpui connector aliases gpui_component::{Theme,ThemeMode} as GpuiTheme/GpuiThemeMode to avoid collision with native_theme re-exports
- [Phase 75-01]: Removed unreachable wildcard arms in same-crate matches (non_exhaustive only forces wildcards in external crates)
- [Phase 75-01]: Wayland compositors use adwaita preset and org.gnome.desktop.interface gsettings for icon themes
- [Phase 76-02]: pub(crate) use re-exports preserve internal crate::Type paths without rewriting 30+ internal files
- [Phase 76-02]: pub mod theme { pub use crate::model::*; } inline facade for clean public API
- [Phase 76-02]: native-theme-build codegen emits module-qualified paths (theme::, detect::) in generated code
- [Phase 77-01]: ColorMode enum in model/mod.rs, re-exported via pub mod theme and pub(crate) use
- [Phase 77-01]: GnomePortalData.is_dark kept as bool (internal D-Bus); conversion at pipeline boundary
- [Phase 77-01]: Connector examples rename local ColorMode to AppColorMode to avoid collision
- [Phase 77-01]: Connector from_preset/to_theme keep is_dark: bool params; convert internally to ColorMode
- [Phase 77-02]: icon_set/icon_theme on Theme (shared); pipeline resolves with system_icon_set/system_icon_theme fallback
- [Phase 77-02]: KDE Breeze preset uses "breeze" (light value) as shared icon_theme; KDE reader overrides at runtime
- [Phase 77-02]: Connector examples maintain current_icon_set/current_icon_theme state parallel to current_resolved
- [Phase 77-02]: resolve_icon_choice/load_all_icons take explicit icon_set/icon_theme params
- [Phase 78-01]: validate() convenience wrapper retained alongside validate_with_dpi() to avoid 40+ call site changes
- [Phase 78-01]: from_kde_content_pure returns (Theme, Option<f32>, AccessibilityPreferences) tuple
- [Phase 78-01]: GPUI to_theme/to_theme_color accept reduce_transparency: bool parameter
- [Phase 78-01]: GPUI accessibility helpers changed from &ResolvedTheme to &SystemTheme
- [Phase 78-01]: Pipeline uses AccessibilityPreferences::default() temporarily; Plan 02 wires real OS values
- [Phase 78-02]: OverlaySource cloned unchanged on with_overlay -- base reader data and preset don't change when overlay applied
- [Phase 78-02]: unwrap_or on strip_suffix("-live") kept for non-live presets (e.g. user presets or catppuccin-mocha in tests)
- [Phase 78-04]: accessibility_from_gnome_data() helper DRYs AccessibilityPreferences extraction from GnomePortalData
- [Phase 78-04]: macOS font_dpi = Some(72.0) -- Apple points = logical pixels, 72 DPI base
- [Phase 78-04]: GNOME and Windows reduce_transparency defaults to false (neither OS exposes this setting)
- [Phase 78-04]: from_kde_content returns outer I/O-detected font_dpi, not inner pure parser dpi
- [Phase 78-03]: reduce_transparency=false default in config-only to_theme_config path (no accessibility data available)
- [Phase 79-02]: L3 audit confirmed zero external consumers; all 6 platform reader I/O functions demoted to pub(crate)
- [Phase 79-02]: from_kde_content_pure stays pub -- integration tests in native-theme/tests/reader_kde.rs depend on it
- [Phase 79-01]: WidgetBorderSpec sets corner_radius_lg=0.0 and opacity=0.0 in resolved output (defaults-only fields)
- [Phase 79-01]: D2 padding-derives-from-presence rule removed -- DefaultsBorderSpec has no padding fields
- [Phase 79-01]: Proptest strategies split into arb_defaults_border_spec and arb_widget_border_spec
- [Phase 80-01]: Field category defaults to "option" when no #[theme(category = ...)] attribute present
- [Phase 80-01]: ResolvedFontSpec nested fields auto-emit check_positive(size) and check_range_u16(weight) without explicit attributes
- [Phase 80-01]: Derive macro does NOT re-emit the Option struct -- user writes serde/Default derives manually
- [Phase 80-01]: inherit_from and border_kind parsed but gated with #[expect(dead_code)] for Plan 02
- [Phase 80-02]: 67 inherit_from attributes migrate all uniform inheritance rules to derive attributes
- [Phase 80-02]: Safety nets (6 rules) and widget-to-widget chains (7 rules) stay hand-written in inheritance.rs
- [Phase 80-02]: LayoutTheme and test widgets use #[theme_layer(skip_inventory)] for non-per-variant structs
- [Phase 80-02]: border_kind classification: menu/tab/card=none, sidebar/status_bar=partial, rest=full(default)
- [Phase 80-fix]: icon_theme relocated from Theme (shared) to ThemeDefaults (per-variant) -- KDE dark uses "breeze-dark", light uses "breeze"
- [Phase 80-fix]: KDE reader cascades to kdedefaults/kdeglobals for icon_theme when main kdeglobals lacks [Icons] section (Plasma 6)
- [Phase 80-fix]: Pre-existing kde/mod.rs test compilation with --features kde fixed (from_kde_content tuple destructuring)
- [Phase 81-01]: pollster is non-optional on Linux (not gated behind portal) -- from_system() always needs block_on
- [Phase 81-01]: Non-Linux from_system() uses Waker::noop() single-poll -- zero-dep, correct for zero-.await futures
- [Phase 81-01]: portal activates ashpd/async-io directly (no separate runtime-variant features)
- [Phase 81-01]: linux-kde and linux-portal sub-aggregators enable fine-grained feature selection
- [Phase 81-02]: 12 CI matrix entries: no-features, kde, portal, linux-kde, linux-portal, linux, native, icons, Windows(2), macOS(2)
- [Phase 81-02]: sync_consumer_no_async_runtime gated on cfg(target_os=linux) + cfg(feature=kde) to exercise pollster::block_on path
- [Phase 82-01]: bundled_icon_svg stays returning Option<&'static [u8]> -- Cow wrapping at call site avoids churn in 400-line match blocks
- [Phase 82-01]: IconRole::name() uses explicit match (not derive macro) for compile-time guaranteed kebab-case strings
- [Phase 82-01]: iced connector uses cow.to_vec()/cow.into_owned() for from_memory() compatibility
- [Phase 82-02]: IconLoader defaults to system_icon_set() and size 24; callers only override what they need
- [Phase 82-02]: Connector ?Sized functions use load_custom_via_builder helper (calls provider methods directly + IconLoader for system dispatch) to avoid unsized-to-trait-object coercion
- [Phase 82-02]: is_freedesktop_theme_available stays public (capability probe, not a loader)
- [Phase 82-02]: CLI theme override uses IconLoader::new(name).set(Freedesktop).theme(t) instead of direct load_freedesktop_icon_by_name
- [Phase 86-01]: lint_toml uses inventory::iter::<WidgetFieldInfo>() HashMap for widget discovery; STRUCTURAL_KEYS retains only defaults and text_scale
- [Phase 90-01]: Manual impl Default for Rgba (returns TRANSPARENT) instead of removing Default entirely -- ResolvedBorderSpec, ResolvedFontSpec, require() helper, and ThemeWidget derive all need Rgba: Default
- [Phase 90-04]: Theme::new() deleted; callers use struct literal with Default. NoVariant error categorized as ErrorKind::Resolution. Connector from_preset functions propagate NoVariant via ? instead of custom error wrappers
- [Phase 91-02]: is_border_type() detects border fields by resolved type last segment (ResolvedBorderSpec); struct-level border_kind is single source of truth for validation dispatch
- [Phase 92-01]: map_while(Result::ok) instead of filter_map for BufRead::lines() per clippy::lines_filter_map_ok
- [Phase 93-02]: LinuxDesktop::Wayfire inserted between Niri and CosmicDe (preserves "wlroots compositors then other desktops" grouping); XDG token is exact-case "Wayfire" (no case-insensitive fallback, matches Hyprland/Budgie/COSMIC style)
- [Phase 93-02]: Wayfire routed through the shared wlroots fallback arm (adwaita preset + portal) rather than getting a dedicated arm -- same rationale as Sway/River/Niri (wlroots compositor, no native theme engine, consumes GTK/portal config)
- [Phase 93-02]: Rule 3 deviation -- model/icons.rs detect_linux_icon_theme's exhaustive match also required a Wayfire arm (dispatched to org.gnome.desktop.interface gsettings alongside the other wlroots compositors); plan body only listed detect.rs and pipeline.rs but compile-required arm was mandatory
- [Phase 93-03]: bundled_icon_svg, bundled_icon_by_name, load_freedesktop_icon_by_name demoted to pub(crate); model/mod.rs re-export pub(crate) use; internal crate::bundled_icon_* paths preserved via lib.rs pub(crate) use re-export
- [Phase 93-03]: Bundled-set None assertion test split into per-OS checks (SfSymbols on non-macos, SegoeIcons on non-windows); Freedesktop no longer asserts None because IconLoader intentionally loads via filesystem on Linux with system-icons
- [Phase 93-03]: IconLoader::new(name).set(IconSet::Freedesktop).theme(t).size(24).load() as canonical freedesktop-by-name migration pattern; GPUI connector internal helper and showcase example migrated; bundled_icon_by_name name-lookup calls rewritten as IconLoader::new(name).set(icon_set).load() returning Option<IconData> directly (no manual IconData::Svg wrap)
- [Phase 93-01]: `require<T: Clone>` gains explicit `fallback: T` parameter (Default bound removed); call sites supply zero-value sentinels directly. Reverses Phase 90-01 "manual impl Default for Rgba" decision -- the Default bound chain is now broken at its source in validate_helpers.
- [Phase 93-01]: validate_defaults! macro split into `option_color` (Rgba fields) + `option_f32` (geometry fields) groups; `border_required` takes `field: fallback_expr` pairs. Encoding the type group at call site keeps sentinel construction local and explicit (alternative would have been a crate-private Default-equivalent trait, which would reintroduce the bound chain).
- [Phase 93-01]: native-theme-derive::gen_validate `fallback_for_ty` maps Option-inner types to zero-value sentinels (Rgba->TRANSPARENT, f32->0.0, u16->0, bool->false, Arc->empty Arc<str>, String->String::new(), DialogButtonOrder->PrimaryRight); unknown types emit compile_error!.
- [Phase 93-01]: `impl Default for Rgba` deleted (§16 footgun). 3 hand-written Resolved leaves (ResolvedBorderSpec, ResolvedFontSpec, ResolvedTextScaleEntry) drop Default from derive lists. 26 generated widget structs already lacked Default (ThemeWidget macro never emitted it). ResolvedDefaults/ResolvedTheme/ResolvedTextScale/ResolvedIconSizes likewise unchanged.
- [Phase 93-01]: Integration test trait_assertions_default_clone_debug: Rgba now asserts Clone+Debug only (no Default). Theme/ThemeMode/ThemeDefaults/FontSpec still assert full Default+Clone+Debug set.
- [Phase 93-04]: Theme.icon_theme: Option<Cow<'static, str>> added (shared across variants) with skip_serializing_if; Default impl, merge(), is_empty(), and TOP_KEYS all updated. Pipeline resolver rewritten to three-tier precedence (per-variant override -> Theme-level shared -> system detect). ThemeDefaults.icon_theme rustdoc rewritten to describe its role as per-variant override. Reverses Phase 80-fix ONLY in that it ADDS the Theme-level field; the per-variant field stays as-is and still wins at tier 1.
- [Phase 93-04]: 15 bundled presets migrated to top-level icon_theme (adwaita, catppuccin-{frappe,latte,macchiato,mocha}, dracula, gruvbox, ios, macos-sonoma, material, nord, one-dark, solarized, tokyo-night, windows-11). kde-breeze stays per-variant-only (breeze vs breeze-dark genuinely differ); kde-breeze-live stays geometry-only (KDE reader supplies runtime value).
- [Phase 93-04]: 3 live shadows (adwaita-live, macos-sonoma-live, windows-11-live) gain a top-level icon_theme matching their base. Safety net for any pipeline path that might use only a live preset without a full-preset merge behind it.
- [Phase 93-04]: Test updates to match migration: platform_facts_xref.rs adwaita/windows-11 assertions rewritten to Theme-level; proptest arb_theme_spec strategy extended to generate Theme.icon_theme so round-trip covers Some/None for the new field.
- [Phase 93-04]: Rule 3 deviation -- three exhaustive Theme literals (lib.rs::to_theme, kde/mod.rs::build_theme twice, macos.rs::to_theme) required icon_theme: None field additions beyond the three sites listed in the plan body. E0063 would have blocked compilation otherwise.
- [Phase 93-05]: `ThemeFields` proc-macro derive (in native-theme-derive) registers plain structs into a sister `FieldInfo` inventory alongside the existing `WidgetFieldInfo` registry. Emission style matches the existing widget derive (direct `inventory::submit!(...)` at item level; no anonymous const wrapper).
- [Phase 93-05]: FontSpec and TextScaleEntry declare `#[theme_layer(fields = "...")]` explicit-override attribute because they serialize through private `FontSpecRaw` / `TextScaleEntryRaw` serde proxies whose wire field names (`size_pt`/`size_px`, `line_height_pt`/`line_height_px`) differ from the user-facing struct fields. Alternative (parsing the `try_from = "..."` attribute and fetching the proxy's AST) rejected -- proc-macros cannot access sibling-type ASTs; explicit declaration keeps the wire contract visible at the struct level.
- [Phase 93-05]: LayoutTheme dual-derives `ThemeWidget` + `ThemeFields` while retaining `#[theme_layer(skip_inventory)]`. skip_inventory prevents registration in WidgetFieldInfo (correct: LayoutTheme is NOT a per-variant widget -- it lives on Theme, not ThemeMode); ThemeFields registers it in the non-widget struct registry under key "LayoutTheme" so lint_toml can look up its 4 `_px` field names.
- [Phase 93-05]: lint_toml rewritten to build both `widget_registry: HashMap<&str, &[&str]>` (from `inventory::iter::<WidgetFieldInfo>()`) and `struct_registry: HashMap<&str, &[&str]>` (from `inventory::iter::<FieldInfo>()`) at function entry, then consume both via closure captures. Former free functions lint_text_scale/lint_defaults/lint_variant converted to closures. Missing struct entry -> silent skip (matches pre-existing `continue;` behaviour; no new Error variant).
- [Phase 93-05]: Rule 2 auto-fix in native-theme-derive::parse_one_field -- `parse_nested_meta` on `#[serde(...)]` attributes previously only recognised `rename` and ignored other sub-attributes without consuming their values. On structs with non-Option fields carrying `#[serde(default, skip_serializing_if = "...")]` (e.g. `ThemeDefaults.font`, `ThemeDefaults.border`), this produced a misleading `expected ','` error that appeared to originate in `serde_with::skip_serializing_none`. parse_one_field now optionally consumes the value expression of unknown serde sub-attrs.
- [Phase 93-05]: Gap-doc correction -- the §G5 target list named `ResolvedFontSpec` but that struct has no FIELD_NAMES constant today and is not consumed by lint_toml (output type, connector-facing). Not migrated. Final migration set is 7 structs + LayoutTheme (8 total).
- [Phase 93-07]: Option D (principled deviation) selected over Options A/B/C for closing G-3b (cargo test --workspace fails due to upstream naga 27.0.3 / codespan-reporting 0.12.0 incompatibility via gpui-component v0.5.1). Option A impossible (naga 27.0.4 does not exist on crates.io, verified via `cargo info naga@27.0.4` returning "could not find"; next release is 28.x which gpui-component's 27.x pin rejects via semver). Option B weak (scope narrowing reads as hiding the problem). Option C worse (excluding native-theme-gpui from [workspace] members breaks developer ergonomics and propagates upstream defect into project layout).
- [Phase 93-07]: Phase 93 acceptance-criterion realignment — must_have truth #5 (`cargo test --workspace --all-features` passes) is replaced by per-crate equivalent tied to `./pre-release-check.sh` lines 267-294 (cargo test -p native-theme + per-crate for each workspace member, with native-theme-gpui treated as soft per run_check_soft at pre-release-check.sh:290). This matches the release gate the script has enforced since Phase 14-03 (2026-03-09).
- [Phase 93-07]: Root cause lives outside native-theme (naga 27.0.3 references `codespan_reporting::term::emit` with `&mut String` writer; codespan-reporting 0.12.0 at Cargo.lock:1064-1067 dropped `impl WriteColor for String` that the 0.11.x series provided). Fix belongs to gpui-component or naga upstream; native-theme has no path to resolve without forking.
- [Phase 93-07]: Re-evaluation trigger documented inline in G11 (runnable command chain: `cargo update -p gpui-component && cargo test --workspace --all-features`). When gpui-component ships a release past naga 27.0.3 (or pins codespan-reporting 0.11.x), the --workspace criterion is restorable.
- [Phase 93-07]: Two-file atomic commit discipline — docs/todo_v0.5.7_gaps.md (previously untracked; first commit to git history) + .planning deferred-items.md cross-reference committed together as `a6e8d4e docs(93-07)`. No Co-Authored-By trailer (user memory rule). Zero source code changes. APPEND-ONLY rule honoured: deferred-items.md git diff shows 0 `^-` true-deletion lines; gaps.md is a new tracked file (vacuously append-only) but all pre-existing local content was preserved verbatim.
- [Phase 93-06]: Option A (rewrite doctests to `IconLoader`) selected over Option B (delete the doctests) for Gaps 1 and 2. Doctests on pub(crate) functions serve maintainers reading `cargo doc --document-private-items`; showing the public replacement API directly teaches the correct external-caller pattern while remaining a compiling runnable example. Option B would leave the docstrings runnable-example-free.
- [Phase 93-06]: Option A (conditional `#[cfg_attr(not(any(feature = "material-icons", feature = "lucide-icons")), allow(dead_code))]`) selected over Options B (gate the function itself with the same #[cfg] union as callers) and C (delete + inline at call sites) for Gap 3. Option B rejected because it would ALSO gate out the unconditional `#[cfg(test)]` tests `by_name_non_bundled_sets_return_none` and `by_name_unknown_name_returns_none` at bundled.rs:691-702 which call the function and must remain live in the default test build. Option C rejected because it would lose 7 internal test call sites covering by-name dispatch. The conditional `cfg_attr` keeps the function always-compiled/always-testable while silencing the lint exactly when it is provably dead.
- [Phase 93-06]: Conditional (not unconditional) `allow(dead_code)` — the cfg_attr predicate `not(any(material-icons, lucide-icons))` is the exact complement of the caller cfg union `any(material-icons, lucide-icons)` at icons.rs:598,603. If either caller gets un-gated in the future, the allow stops firing and real dead-code regressions are unmasked. An unconditional `#[allow(dead_code)]` would silence the lint forever, masking future regressions.
- [Phase 93-06]: `./pre-release-check.sh` runs clippy WITHOUT `--all-features` (line 283: `cargo clippy -p "$crate" --all-targets -- -D warnings`). This is the release gate. The plan's verify step 4 asks for `--all-features` clippy green as a belt-and-suspenders check; pre-existing failures in spinners.rs, freedesktop.rs, reader_kde.rs are OUT of Plan 93-06's one-file scope per SCOPE BOUNDARY rule and NOT part of the release gate.
- [Phase 93-06]: Post-edit pre-release-check.sh failure locus moved from step 15 ("Running clippy (native-theme)", line 283 — the step Plan 93-06 was chartered to unblock) to step 23 ("Validating packages (core)", line 321). Step 23 failure is pre-existing at parent commit 51c386b (verified by `git checkout 51c386b -- . && cargo package ...` reproducing the same 54 errors) and is caused by Plan 93-05's `ThemeFields` derive addition not yet being published to crates.io. Fix is `cargo publish -p native-theme-derive v0.5.7` which is a release action requiring EXPLICIT user approval per user memory rule `feedback_never_bypass_checkpoints.md`. Out of scope for any automated plan execution.
- [Phase 93-08]: The Phase 93-06 step-23 diagnosis above (publish the derive first, then re-run) was directionally wrong and has been superseded. Root cause: `cargo package` tarball verification compiles each tarball as if downloaded from crates.io. For a first-ever publication of a workspace with internal path deps, the depended-on crate is not yet indexed, so the simulation cannot succeed. The original remediation ("publish first") reversed the release gate (release script must go green BEFORE publication, not as a consequence of it) AND was architecturally impossible anyway (`cargo publish` runs the same tarball verify that was failing). This is a cargo architectural constraint; overlay-registry RFCs have been pending for years (rust-lang/cargo#9227).
- [Phase 93-08]: Option B (`--no-verify` on the three cargo package invocations at pre-release-check.sh:333-335 + documented ordered-publish workflow in RELEASING.md) selected as the fix. Option C (local registry simulation via `cargo vendor` + `[source.crates-io] replace-with`) rejected after deeper analysis — cargo source replacement is all-or-nothing for crates.io, so Option C would require vendoring the entire dep tree (100MB+, hundreds of crates, regen on every dep change) with no meaningful confidence gain over the documented publish-order workflow. The real tarball-verification happens during `cargo publish` itself (each invocation in dependency order verifies against a crates.io that now has the prior crate indexed); `cargo publish` is the real release-grade validation, not the pre-release script.
- [Phase 93-08]: VERIFICATION.md edited APPEND-ONLY with two targeted frontmatter field corrections (status: human_needed->passed; re_verification.superseded_by: plan-93-08) + a trailing `## Update 2026-04-19 (Plan 93-08)` section explaining the correction. Original `human_verification:` block preserved above for audit trail. No existing narrative paragraph was modified. This is an APPEND-ONLY compliance interpretation — targeted frontmatter field edits reflect resolved state without rewriting history; the narrative record of what the original verifier said stays intact.
- [Phase 93-08]: Post-bootstrap cleanup condition recorded in both pre-release-check.sh:320-332 (inline comment) and RELEASING.md (dedicated section): once `native-theme-derive 0.5.7` is live on crates.io, remove `--no-verify` from the three `cargo package` lines to restore full tarball-verify for subsequent releases (0.5.8+). The self-unmasking property keeps the bootstrap workaround narrowly scoped to the first-ever publication.
- [Phase 93-08]: `./pre-release-check.sh` post-fix output confirms green: final three lines are `🎉 All pre-release checks passed successfully!` + `native-theme v0.5.7 is ready for release.` + the script's own "Next steps" section listing the publish order. Zero red markers, exit code 0. This meets Plan 93-06 success criterion #8 (pre-release-check.sh green banner) and closes the Phase 93 release-gate gap end-to-end.
- [Phase 93-09]: The Phase 93-08 "green banner" claim was INCOMPLETE — the banner was green only because the gpui test failure was masked by `run_check_soft`. Deeper inspection of the full 1500-line script output revealed `test icons::freedesktop_mapping_tests::gnome_names_resolve_in_adwaita ... FAILED` with 12 missing GNOME icons on KDE Plasma. Root cause traced to Phase 93-03 commit 7d6e1f1 which migrated the test from `load_freedesktop_icon_by_name(fd_name, "Adwaita", ...)` to `IconLoader::new(fd_name).set(Freedesktop).theme("Adwaita").load()` — the new form silently drops `.theme()` because `IconLoader::load_by_name()` (icons.rs:213-216) doesn't read `self.freedesktop_theme`, unlike `load_role()` which does. `system_icon_theme()` on KDE returns "breeze"; Breeze lacks those 12 GNOME-specific symbolic names.
- [Phase 93-09]: IconLoader replaced with typed-per-set loaders (FreedesktopLoader/SfSymbolsLoader/SegoeIconsLoader/MaterialLoader/LucideLoader). Each loader exposes only the methods meaningful for its set; calling `.theme()` on SfSymbolsLoader is a compile error rather than a silent no-op. Eliminates the silent-ignore bug class at the type system level — the root cause (five-field single-struct with three freedesktop-only fields, load() dispatching on id variant, any new field vulnerable to dispatch-bypass) is restructured so options and dispatch live on the same typed struct with no bypass layer.
- [Phase 93-09]: Rejected Option A (patch `load_by_name` to honor theme) in favor of the structural refactor. Option A would have fixed the one observed bug but preserved the silent-ignore design pattern; any future set-specific option (SF Symbols weight, Material filled/outlined variant, Lucide stroke-width, Segoe Mdl2 vs FluentUI) would face the same dispatch trap. User explicitly asked for "most correct, very best" solution with API-breakage accepted.
- [Phase 93-09]: Rejected Option C (local registry simulation via generic IconLoader<O: LoaderOptions> phantom-type approach) as over-engineered. Five distinct structs are clearer than one generic with sealed-trait options; no generic parameter noise at call sites; runtime dispatch is a match on IconSet + free function, not dynamic trait dispatch. Also rejected the local-file-registry Option C for `cargo package` validation — would require `cargo vendor`ing the entire dep tree (100MB+, hundreds of crates), no meaningful confidence gain over `--no-verify` + documented ordered publish.
- [Phase 93-09]: Runtime set dispatch handled by two free functions `load_icon(id, set) -> Option<IconData>` and `load_icon_indicator(set) -> Option<AnimatedIcon>` for default-options loads. Callers who need per-set options write an explicit match on IconSet with `_ => None` wildcard (IconSet is `#[non_exhaustive]` so external crates require the wildcard). Verbosity is intentional — it forces the correct mental model (options ARE per-set, and silently-ignored options in the builder are now explicit deliberate drops at the match layer).
- [Phase 93-09]: `load_freedesktop_spinner` signature changed from `fn() -> Option<AnimatedIcon>` to `fn(theme: Option<&str>) -> Option<AnimatedIcon>`. Closes a SECOND silent-ignore bug discovered during deeper inspection: the spinner function hardcoded `detect_theme()`, so `IconLoader::new(x).set(Freedesktop).theme("Adwaita").load_indicator()` always dropped the `.theme()` override for indicator loads even in the OLD API. This bug predated Phase 93-03.
- [Phase 93-09]: `FreedesktopLoader::load_indicator(theme: Option<&str>)`, `MaterialLoader::load_indicator()`, `LucideLoader::load_indicator()` are associated functions (no self) rather than instance methods. Spinner is a property of the set (and optionally theme for freedesktop), not of an icon id; associated-function form avoids wasteful id construction (`IconLoader::new(ignored_role).set(set).load_indicator()` → `MaterialLoader::load_indicator()`) and makes the per-set-ness visible at the call site.
- [Phase 93-09]: `size` field lives only on FreedesktopLoader. Bundled SVG sets (Material, Lucide) are scalable at render time; SF Symbols and Segoe are rendered by their platform APIs. Per-set discretion in field inventory reflects genuine domain heterogeneity rather than uniform API.
- [Phase 93-09]: Orphan deletion — `freedesktop::load_freedesktop_icon(role, size, fg_color)` (role-taking variant that self-detected theme) removed since its only non-test caller was the deleted `load_icon_inner`. Internal tests in freedesktop.rs migrated to use `load_freedesktop_icon_by_name(name, theme, size, fg_color)` directly (theme passed explicitly). No public API affected since it was `pub(crate)`.
- [Phase 93-09]: Dead-code annotations — `FreedesktopLoader` fields (id, size, fg_color, theme), `MaterialLoader.id`, `LucideLoader.id`, `SfSymbolsLoader.id`, `SegoeIconsLoader.id` all carry `#[allow(dead_code)]` because they are read only inside the platform/feature cfg branch of `load()`. Without the annotation, clippy's `-D warnings` fires on builds without the relevant feature (e.g. building native-theme without material-icons).
- [Phase 93-09]: Full pre-release-check.sh line-by-line scan (not just tail-3) confirms fully green: every one of the 20 test suites reports `test result: ok. N passed; 0 failed`; no `⚠` markers for any tests; final banner `🎉 All pre-release checks passed successfully!`. The gpui lib test `gnome_names_resolve_in_adwaita` that was failing 151/1 across 93-04..08 now passes 152/0. Two pre-existing unrelated warnings remain in the script output and are deferred to separate follow-ups: (a) cargo-audit `unmaintained` on 4 transitive deps via gpui-component (async-std, instant, paste, rustls-pemfile); (b) rustdoc `private_intra_doc_links` on ResolvedTextScaleEntry at model/resolved.rs:35. Neither is an IconLoader regression.
- [Phase 94-02]: `ResolutionContext` has no `impl Default` (per docs/todo_v0.5.7_gaps.md §G7 / doc 2 §J.2 refinement on B5 signal-intent). Runtime-detected types must signal intent at the call site — `from_system()` for production, `for_tests()` for deterministic test values (96 DPI, PrimaryRight, no icon_theme). A commented-out `fn assert_no_default<T: Default>() {}` in the regression test documents the contract.
- [Phase 94-02]: `&ResolutionContext` parameter, not `Option<&ResolutionContext>` (deviation from gap doc §G7 / J.2 B5 proposed signature). Rationale: the None-overload would lose the intent signal and reintroduce the silent-default anti-pattern. Explicit `resolve_system()` shortcut covers the from_system() case; callers wanting custom ctx type `into_resolved(&ctx)`.
- [Phase 94-02]: `resolve_system()` placed on `ThemeMode`, not `Theme` (deviation from gap doc §G7 step 4). Rationale: Theme has both `light: Option<ThemeMode>` and `dark: Option<ThemeMode>` variants; `Theme::resolve_system()` would need to arbitrarily pick one. The natural pairing `theme.into_variant(mode)?.resolve_system()` is one method longer but unambiguous about variant selection and matches existing call-site patterns.
- [Phase 94-02]: `AccessibilityPreferences` NOT moved to `ResolutionContext` (stays on `SystemTheme` per ACCESS-01 / J.2 B4 refinement). Accessibility is a render-time concern, not a resolve-time concern — callers needing to re-resolve with different accessibility prefs go through `SystemTheme::with_overlay()`.
- [Phase 94-02]: `validate_with_dpi(dpi: f32)` retained unchanged as the low-level entry point. Tests that exercise specific DPI values (e.g. TEST_DPI_APPLE = 72.0 for Apple pt↔px identity, TEST_DPI_STANDARD = 96.0) bypass the context struct and call validate_with_dpi directly. `ResolutionContext::for_tests()` internally feeds 96.0 into validate_with_dpi, so the context struct is a higher-level convenience over the retained low-level API.
- [Phase 94-02]: `resolve_all()` zero-arg method retained alongside new `resolve_all_with_context()` — internal pre-resolve callers in presets.rs, macos.rs, windows.rs, kde/mod.rs call `resolve_all()` before separately constructing ctx. Both methods are `#[doc(hidden)]`; only `into_resolved()` and `resolve_system()` are on the public API surface.
- [Phase 94-02]: Pipeline reader-supplied `font_dpi` (e.g. KDE forceFontDPI) overrides `ctx.font_dpi` via `if let Some(dpi) = font_dpi { ctx.font_dpi = dpi; }` — preserves existing behaviour. `ctx.button_order` and `ctx.icon_theme` come from `from_system()` unchanged; the icon_theme three-tier precedence (per-variant → Theme-level → ctx.icon_theme fallback) is now explicit.
- [Phase 94-02]: `pub mod resolve` with selective surface. Only `ResolutionContext` is publicly visible via `pub use context::ResolutionContext`; `inheritance`, `validate`, `validate_helpers` stay `pub(crate)`. The crate root also re-exports via `pub use resolve::ResolutionContext;` so `native_theme::ResolutionContext` and `native_theme::resolve::ResolutionContext` both work.
- [Phase 94-02]: Prelude expanded from 7 to 8 items: adds `ResolutionContext` alongside `Theme`, `ResolvedTheme`, `SystemTheme`, `AccessibilityPreferences`, `Rgba`, `Error`, `Result`. `prelude_smoke.rs` updated to assert the new count + types.
- [Phase 94-02]: No deprecation shim for the old `into_resolved(Option<f32>)` signature — v0.5.7 is the no-backcompat window per REQUIREMENTS.md (same policy as Phase 93-09 IconLoader typed-per-set migration and Phase 93-01 require() refactor). 43 call sites migrated atomically in one GREEN commit.
- [Phase 94-02]: Parallel-execution collision with concurrent plan 94-01 (running in another agent, declared disjoint per orchestrator notice) destructively overwrote in-progress edits twice during execution. Remediation: atomic large commit immediately after re-applying edits; `context.rs` (untracked new file) and the earlier RED commit survived both wipes via git persistence.
- [Phase 94-02]: 94-01's `BorderInheritanceInfo` / `FontInheritanceInfo` inventory structs (declared in its RED phase but not yet consumed by a reader) produce clippy `-D warnings` dead_code errors that cause `./pre-release-check.sh` to fail at the "Running clippy (native-theme)" step. Deferred to 94-01's GREEN phase (self-resolves when the consumer is wired up). Documented in `.planning/phases/94-.../deferred-items.md`.

### Roadmap Evolution

- Phase 90 added: resolve remaining v0.5.7 API overhaul gaps
- Phase 91 added: resolve remaining TODO doc gaps (15b, 15f, B1-require, B7, C6)
- Phase 92 added: implement the chosen solutions described in docs/todo_v0.5.7_icon-theme.md
- Phase 93 added: docs/todo_v0.5.7_gaps.md
- Phase 94 added: close remaining v0.5.7 gaps G6-G8 (border/font inheritance codegen, ResolutionContext, ThemeReader trait)

### Blockers/Concerns

- `AccessibilityPreferences` relocation from `ThemeDefaults` to `SystemTheme` (Phase 78) is a cross-cutting refactor; touches resolve engine, connectors, and all presets
- Proc-macro codegen (Phase 80, Unit 10) is a P1 investment with ~1 week estimate; inheritance-expressiveness unknown flagged as medium-confidence
- §1 type rename + §12 crate-root partition (Phase 76) touches connectors (gpui, iced) in lockstep
- C4 `Arc<str>` font family migration (Phase 87) needs `serde rc` feature flag and connector-side `.family` access migration
- Phase 80 depends on Phase 71 (needs new Error shape) AND Phase 79 (needs clean border target) — longest dependency chain in the milestone
- Phase 81 must ship last — absorbs every other change before the feature graph is re-cut

## Session Continuity

Last session: 2026-04-20T00:06:04Z
Stopped at: Completed 94-02-PLAN.md (G7 ResolutionContext migration). Three commits: dc03e53 RED (5 failing regression tests) + 01d5b80 GREEN (43 call-site migration + new ResolutionContext struct + resolve_system shortcut + pub mod resolve) + cc41fad docs (CHANGELOG G7 section + README migration examples). All five G7 regression tests GREEN; 553 native-theme lib tests pass; 97 iced tests pass. Phase 94 status: 94-02 complete (wave 1); 94-01 concurrent in-progress (wave 1); 94-03 wave 2. Deferred: 94-01's in-progress dead_code clippy errors blocking pre-release check — will self-resolve when 94-01 GREEN consumes the BorderInheritanceInfo / FontInheritanceInfo registries.
Resume file: None
