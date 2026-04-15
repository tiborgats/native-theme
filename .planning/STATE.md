---
gsd_state_version: 1.0
milestone: v0.5.7
milestone_name: API Overhaul
status: in-progress
stopped_at: Completed 91-03-PLAN.md
last_updated: "2026-04-15T13:58:48Z"
last_activity: 2026-04-15 — Phase 91 plan 03 complete (require boilerplate macro)
progress:
  total_phases: 31
  completed_phases: 30
  total_plans: 65
  completed_plans: 63
  percent: 97
---

# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-04-12)

**Core value:** Any Rust GUI app can look native on any platform by loading a single theme file or reading live OS settings, without coupling to any specific toolkit.
**Current focus:** v0.5.7 API Overhaul — roadmap complete, 20 phases (69–88) ready to plan

## Current Position

Phase: 91 — resolve-remaining-todo-doc-gaps
Plan: 3/3 complete
Status: in-progress
Last activity: 2026-04-15 — Completed 91-02 (unify border inheritance paths)

Progress: [█████████░] 98% (64/65 plans complete)

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

### Roadmap Evolution

- Phase 90 added: resolve remaining v0.5.7 API overhaul gaps
- Phase 91 added: resolve remaining TODO doc gaps (15b, 15f, B1-require, B7, C6)

### Blockers/Concerns

- `AccessibilityPreferences` relocation from `ThemeDefaults` to `SystemTheme` (Phase 78) is a cross-cutting refactor; touches resolve engine, connectors, and all presets
- Proc-macro codegen (Phase 80, Unit 10) is a P1 investment with ~1 week estimate; inheritance-expressiveness unknown flagged as medium-confidence
- §1 type rename + §12 crate-root partition (Phase 76) touches connectors (gpui, iced) in lockstep
- C4 `Arc<str>` font family migration (Phase 87) needs `serde rc` feature flag and connector-side `.family` access migration
- Phase 80 depends on Phase 71 (needs new Error shape) AND Phase 79 (needs clean border target) — longest dependency chain in the milestone
- Phase 81 must ship last — absorbs every other change before the feature graph is re-cut

## Session Continuity

Last session: 2026-04-15T14:00:00Z
Stopped at: Completed 91-02-PLAN.md
Resume file: None
