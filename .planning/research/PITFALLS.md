# Pitfalls Research

**Domain:** Per-widget theme architecture with resolution pipeline added to existing Rust theme crate
**Researched:** 2026-03-27
**Confidence:** HIGH (based on direct codebase analysis + design docs + platform API documentation)

## Critical Pitfalls

### Pitfall 1: TOML Field Path Breaking Change Breaks All User TOMLs

**What goes wrong:**
The restructure changes TOML paths fundamentally. Current presets and any user TOMLs use `[light.colors] accent = "#3daee9"` and `[light.fonts] family = "Noto Sans"`. The new structure uses `[light.defaults] accent = "#3daee9"` and `[light.defaults.font] family = "Noto Sans"`. Every existing TOML file becomes silently invalid -- fields deserialize to `None` instead of erroring, because `#[serde(default)]` swallows unknown sections. Users get a theme with all values missing and no error message explaining why.

**Why it happens:**
`#[serde(default)]` on every struct means TOML deserialization never fails -- it just produces empty structs. A TOML with `[light.colors]` deserializes cleanly into the new `ThemeVariant` that has no `colors` field; the section is silently ignored. The old field values vanish.

**How to avoid:**
1. Add `#[serde(alias = "colors")]` or a migration layer on appropriate fields during a transition period. `ThemeVariant` could accept both `defaults` and the old `colors`/`fonts`/`geometry` sections via serde aliases or a custom deserializer.
2. Add a `#[serde(deny_unknown_fields)]` on `NativeTheme` or `ThemeVariant` during development/testing to catch silent drops. Remove before release (since `#[non_exhaustive]` and forward compat need unknown field tolerance).
3. Provide a TOML migration script or validation function: `NativeTheme::validate_toml(toml_str) -> Vec<Warning>` that detects old-format field paths.
4. Document the breaking change prominently. Bump to v0.5.0 (semver: pre-1.0, minor bump signals breaking).

**Warning signs:**
- Preset tests pass but all values are `None` (the test checks `is_some()` on specific fields).
- Connector showcase renders with default/black colors instead of themed colors.
- `validate()` reports "all fields missing" but no TOML parse error occurred.

**Phase to address:**
Phase 1 (Data Model Restructure). The field path mapping must be designed before any TOML is rewritten. Serde aliases must be added in the same commit that changes struct field names.

---

### Pitfall 2: ThemeVariant and ResolvedTheme Struct Explosion -- 28 Struct Pairs = Maintenance Nightmare

**What goes wrong:**
For every `ButtonTheme` (with `Option` fields), a parallel `ResolvedButton` (with concrete fields) must exist. With 24 widget structs + `ThemeDefaults` + `TextScale` + `FontSpec` + `ThemeSpacing` + `IconSizes`, this means ~28 Resolved* structs. Each field added to a widget struct must be added to its Resolved counterpart, the `resolve()` function, and the `validate()` conversion. One forgotten field in any of the three locations causes silent bugs.

**Why it happens:**
The Option-to-concrete pattern is a well-known Rust pain point. There is no built-in language feature to say "this struct but with all Options unwrapped." Proc macros can generate the parallel struct, but add complexity. Manual maintenance of parallel structs is error-prone at this scale.

**How to avoid:**
Use a declarative macro or proc macro that generates both the Option and concrete versions from a single definition. The existing `impl_merge!` macro already demonstrates this pattern. Extend or complement it:

```rust
// Single source of truth per widget
define_widget_theme! {
    ButtonTheme {
        background: Rgba,
        foreground: Rgba,
        border: Rgba,
        font: FontSpec,
        min_width: f32,
        // ...
    }
}
// Generates: ButtonTheme (all Option), ResolvedButton (all concrete),
//            merge(), is_empty(), resolve_from(), validate()
```

If proc macros are too heavy, a build script in `native-theme-build` could generate both struct variants. At minimum, add a compile-time test that verifies field counts match between paired structs (field-counting macros or size assertions).

Do NOT use `..Default::default()` when constructing `ResolvedTheme` -- use struct literal syntax so the compiler catches missing fields.

**Warning signs:**
- A new field added to `ButtonTheme` that is absent from `ResolvedButton` -- the compiler catches this only if `resolve()` uses struct literal syntax (not if it uses `..Default::default()`).
- `validate()` returns `Ok` but the resolved field has a zero/default value instead of the actual theme value.

**Phase to address:**
Phase 2 (ResolvedTheme Module). The code generation strategy must be decided before writing 28 struct pairs. Retrofitting a macro onto hand-written structs is harder than starting with the macro.

---

### Pitfall 3: resolve() Ordering Bugs -- Dependency Chains Between Derived Fields

**What goes wrong:**
`resolve()` must fill fields in dependency order. The inheritance rules document three resolution phases: (1) defaults-internal chains like `defaults.selection <- defaults.accent` and `defaults.focus_ring_color <- defaults.accent`, (2) widget fields derived from defaults like `button.primary_bg <- defaults.accent`, (3) widget-to-widget chains like `window.inactive_title_bar_background <- window.title_bar_background`. If resolve processes phase 2 before phase 1, a field like `button.primary_bg` tries to inherit from `defaults.accent` which is itself still `None` (because the OS reader provided `defaults.selection` but not `accent`, and the safety-net `accent <- selection` has not yet run).

**Why it happens:**
With ~200 fields and ~90 inheritance rules, it is easy to introduce a dependency that the resolve function processes in the wrong order. Unlike a DAG scheduler, a flat `resolve()` function processes fields sequentially. Reordering code or adding new inheritance rules can silently break existing chains.

**How to avoid:**
1. Structure `resolve()` in explicit, commented phases:
   - Phase 1: Resolve `defaults` internal chains (accent, font, radius -- these are roots of all inheritance).
   - Phase 2: Resolve `defaults` safety nets (selection <- accent, focus_ring <- accent).
   - Phase 3: Resolve widget fields from `defaults` (button.primary_bg <- defaults.accent, etc.).
   - Phase 4: Resolve widget-to-widget chains (window.inactive_title_bar_background <- window.title_bar_background).
2. Add a test for every inheritance chain documented in `todo_v0.5.1_inheritance-rules.md`. Each test provides a minimal ThemeVariant with only the root source set, runs `resolve()`, and asserts the derived field is populated.
3. Never add bidirectional inheritance (A <- B and B <- A). The inheritance rules doc explicitly avoids this -- enforce it with a comment convention and review discipline.

**Warning signs:**
- A `resolve()` test where the derived field is `None` despite the source being `Some`.
- A `validate()` error for a field that should have been filled by inheritance.
- Different resolve results depending on whether OS reader or TOML provides the source value.

**Phase to address:**
Phase 2 (ResolvedTheme Module). The resolve function must be written with explicit phase ordering from day one.

---

### Pitfall 4: TOML Empty Nested Table Serialization -- Ghost `[light.button]` Sections

**What goes wrong:**
When serializing a `ThemeVariant` with per-widget structs, TOML outputs `[light.button]` as an empty table header even when all fields inside `ButtonTheme` are `None`. This produces confusing TOML files with dozens of empty section headers. Worse, a round-trip through serialize -> deserialize may produce different results if the empty table creates a `Some(Default::default())` instead of `None`.

The current codebase already handles this with `#[serde(skip_serializing_if = "WidgetMetrics::is_empty")]` on the `widget_metrics` field. But the new architecture makes every widget a direct field of `ThemeVariant` (not `Option<WidgetStruct>`), so `skip_serializing_if` must be applied to each widget field individually.

**Why it happens:**
`serde_with::skip_serializing_none` only skips `Option<T>` fields that are `None`. It does not skip struct fields where all interior `Option` fields are `None`. The struct itself is not `None` -- it is `Default::default()`. Each widget struct field on `ThemeVariant` is a direct struct (not `Option<WidgetStruct>`), so it always serializes unless explicitly suppressed.

**How to avoid:**
Apply `#[serde(default, skip_serializing_if = "ButtonTheme::is_empty")]` to every widget field on `ThemeVariant`. The `impl_merge!` macro already generates `is_empty()` for every struct. Ensure every new widget struct gets both `impl_merge!` (for `is_empty()`) and the `skip_serializing_if` attribute. Consider extending the struct-definition macro to automatically add the serde attribute.

**Warning signs:**
- Serialized TOML output contains empty `[light.button]` or `[dark.window]` sections.
- TOML files are 200+ lines of empty tables, making them unreadable for app developers.
- Round-trip test fails: serialize -> deserialize produces `is_empty() == false` when original was empty.

**Phase to address:**
Phase 1 (Data Model Restructure). Every widget struct must have `is_empty()` and the corresponding `skip_serializing_if` from the start.

---

### Pitfall 5: Qt Font Weight Field -- Qt5 (0-99) vs Qt6 (1-1000) Scale Mismatch

**What goes wrong:**
The KDE reader's `parse_qt_font()` currently extracts only family (field 0) and point size (field 1) from Qt font strings. The design docs require extracting weight from field 4 (0-indexed). But Qt5 and Qt6 encode weight on completely different scales:
- Qt4 `toString()`: 10 fields, field 4 on **0-99 scale** (Normal=50, Bold=75).
- Qt5 `toString()`: 15-16 fields, field 4 on **0-99 scale** (Normal=50, Bold=75).
- Qt6 `toString()`: 18 fields, field 4 on **1-1000 scale** (Normal=400, Bold=700).

If the reader assumes Qt6 (1-1000) but the user has Qt5, weight 50 (Normal in Qt5) gets interpreted as weight 50 (between Thin=100 and ExtraLight=200 in CSS terms). The font renders as ultra-thin. Conversely, if the reader assumes Qt5 (0-99), a Qt6 weight of 400 (Normal) gets interpreted as far beyond Black=87, which is nonsensical.

**Why it happens:**
KDE systems may run Qt5 (KDE Plasma 5) or Qt6 (KDE Plasma 6). The font string format is not self-describing -- you cannot tell from the string alone which weight scale is used. The field count is the only reliable heuristic for Qt version detection.

**How to avoid:**
1. Use the field count to detect Qt version: <=10 fields = Qt4 (weight 0-99), 11-16 fields = Qt5 (weight 0-99), 17+ fields = Qt6 (weight 1-1000).
2. For Qt4/Qt5 (0-99 scale), convert to CSS 100-900:
   - 0 (Thin) -> 100, 12 (ExtraLight) -> 200, 25 (Light) -> 300, 50 (Normal) -> 400, 57 (Medium) -> 500, 63 (DemiBold) -> 600, 75 (Bold) -> 700, 81 (ExtraBold) -> 800, 87 (Black) -> 900.
   - For values between enum points, use linear interpolation or nearest-match.
3. For Qt6 (1-1000 scale), the value is already effectively CSS-compatible. Clamp to 100-900.
4. Add tests with real-world font strings from both Qt5 and Qt6 systems.

**Warning signs:**
- Font weight renders as ultra-thin or ultra-bold when it should be normal.
- Weight value in `FontSpec` is outside CSS 100-900 range.
- Different font rendering behavior between KDE Plasma 5 and KDE Plasma 6 systems.

**Phase to address:**
Phase 3 (OS Reader Extensions). This is KDE reader-specific, required when adding weight parsing to `parse_qt_font()`.

---

### Pitfall 6: Platform Reader Extensions Hit Missing APIs -- Runtime Crashes or Silent Gaps

**What goes wrong:**
The design docs list version-specific APIs:
- macOS 14+: `textInsertionPointColor` (crashes on macOS 13 with unrecognized selector).
- KDE: `[Colors:Header]` section (added in KDE Frameworks 5.71; absent on older KDE).
- Windows: `DwmGetColorizationColor` (unavailable on Windows Server Core without DWM).
- GNOME: `titlebar-font` gsetting (only available with certain GNOME Shell versions; absent on GTK-only desktops like XFCE).

If OS readers unconditionally call these APIs, the result is crashes (macOS), DLL load failures (Windows), or silent validation failures (KDE/GNOME where missing keys return `None` and resolve() has no fallback for that field).

**Why it happens:**
Each platform has different failure modes for missing APIs:
- macOS: Objective-C runtime crashes on unrecognized selectors (not a compile error).
- Windows: DLL functions may not exist, causing link-time or runtime failures.
- KDE: INI key missing returns `None` silently -- no crash, but the field stays empty.
- GNOME: gsettings key missing returns a GLib error -- must be caught.

**How to avoid:**
1. macOS: Use `responds(to:)` / `respondsToSelector:` checks before calling version-specific methods. Or check `NSProcessInfo.operatingSystemVersion` at the top of the reader.
2. Windows: The `windows` crate function bindings return `Result` -- handle errors. For DWM-specific calls, wrap in `LoadLibrary`/`GetProcAddress` or catch the error.
3. KDE: `ini.get()` already returns `Option<String>` -- no crash risk. But document which fields are version-gated so resolve() expectations match.
4. GNOME: gsettings `get()` errors on missing keys -- catch the error and return `None`.
5. General: every OS reader field that uses a newer API must be documented with its minimum OS/framework version.

**Warning signs:**
- Panics or crashes when running on older OS versions during testing.
- `validate()` failures on specific OS versions but not others.
- CI passes on latest OS but fails on older versions in user reports.

**Phase to address:**
Phase 3 (OS Reader Extensions). Each reader extension must include version guards from the start.

---

### Pitfall 7: FontSpec Partial Inheritance -- Two-Level Resolution Needed

**What goes wrong:**
The inheritance rules specify that `FontSpec` supports partial overrides: a TOML that sets only `[light.menu] font = { size = 12.0 }` should inherit `family` and `weight` from `defaults.font`. Serde deserializes this as `Some(FontSpec { family: None, size: Some(12.0), weight: None })`. The `merge()` function handles this correctly for the merge step. But `resolve()` must then fill `family: None` from `defaults.font.family` -- and this requires FontSpec-aware sub-field logic.

The danger: if `resolve()` treats `menu.font` as atomic (either fully present or fully inherited), partial overrides break. resolve() sees `menu.font` is `Some(...)` and skips inheritance, leaving `family` and `weight` as `None`. Validation then fails on `menu.font.family`.

**Why it happens:**
Most resolve patterns check `if field.is_none() { field = source }`. For FontSpec, the check must be `if field.is_some() { fill_none_subfields(field, source) } else { field = source.clone() }`. This two-level inheritance (struct-level + sub-field-level) is unique to `FontSpec` and `TextScaleEntry` among the theme types.

**How to avoid:**
1. Implement `FontSpec::resolve_from(&mut self, base: &FontSpec)` that fills each `None` sub-field from the base.
2. In `resolve()`, handle font fields with explicit two-pass logic:
   ```rust
   // If widget font is None, clone entire defaults
   if self.menu.font.is_none() {
       self.menu.font = Some(self.defaults.font.clone());
   }
   // If widget font is Some but has None sub-fields, inherit them
   if let Some(ref mut font) = self.menu.font {
       font.resolve_from(&self.defaults.font);
   }
   ```
3. Apply the same pattern to `TextScaleEntry` (size, weight, line_height inherit independently).
4. Add explicit tests for the partial override case.

**Warning signs:**
- Partial font overrides in TOML cause validation failures on un-overridden sub-fields.
- All per-widget fonts are identical to `defaults.font` despite TOML overrides.
- Font size overrides work but family is always `None`.

**Phase to address:**
Phase 2 (ResolvedTheme Module). The resolve function must handle FontSpec/TextScaleEntry sub-field inheritance from the start.

---

### Pitfall 8: TextScaleEntry line_height Depends on Resolved size, Not defaults.font.size

**What goes wrong:**
The inheritance rules specify: `TextScaleEntry.line_height <- defaults.line_height * entry.size` (the entry's own resolved size, not `defaults.font.size`). If resolve() computes line_height as `defaults.line_height * defaults.font.size`, every text scale entry gets the same line_height regardless of its own size. A display heading at 26pt gets the same line_height as a caption at 10pt.

**Why it happens:**
This is a subtle dependency within the TextScaleEntry itself. The `line_height` field depends on the entry's `size` field, which itself may need to be resolved first (from `defaults.font.size` if the entry's size is `None`). The resolution order within a single struct matters: resolve `size` first, then compute `line_height` from the resolved size.

**How to avoid:**
1. In the TextScaleEntry resolution code, always resolve `size` before `line_height`:
   ```rust
   // Step 1: resolve size from defaults if needed
   let resolved_size = entry.size.unwrap_or(defaults.font.size.unwrap());
   // Step 2: compute line_height from resolved size
   if entry.line_height.is_none() {
       if let Some(multiplier) = defaults.line_height {
           entry.line_height = Some(multiplier * resolved_size);
       }
   }
   ```
2. Add a test with a TextScaleEntry where size is overridden but line_height is `None`. Verify line_height uses the overridden size, not the default font size.

**Warning signs:**
- All text scale entries have identical line heights despite different sizes.
- Large headings have cramped line spacing; captions have excessive line spacing.
- Line height tests pass but only because defaults.font.size equals the test entry's size.

**Phase to address:**
Phase 2 (ResolvedTheme Module). Part of the resolve() implementation for TextScale entries.

---

### Pitfall 9: Platform Default TOMLs Silently Drift Out of Sync with Struct Changes

**What goes wrong:**
When a new field is added to a widget struct (e.g., `ButtonTheme` gains `hover_bg: Option<Rgba>`), all 17 preset TOMLs must potentially be updated. If any TOML is missed:
- Platform default TOML: the field stays `None`. resolve() may fill it from inheritance, but if no inheritance rule exists, `validate()` fails on that platform only.
- Cross-platform preset TOML: the preset becomes incomplete. `validate()` fails when loading that preset.

The existing test `all_presets_loadable_via_preset_fn` only checks TOMLs parse without error -- it does not check that fields are populated after the full pipeline.

**Why it happens:**
TOML files are not type-checked. There is no compile-time guarantee that a TOML provides every field the struct expects. Missing TOML fields silently deserialize as `None` due to `#[serde(default)]`.

**How to avoid:**
1. Add a `validate()` integration test that loads every preset, runs the full pipeline (TOML -> resolve() -> validate()), and asserts success. This catches missing fields immediately.
2. For platform default TOMLs, the test must simulate the OS reader output (provide the OS-reader fields that the TOML relies on being filled) and run the full pipeline.
3. Document which fields are "TOML-required" vs "OS-reader-provided" vs "resolve()-derived" per platform. The inheritance rules doc already has this classification -- turn it into an automated check.

**Warning signs:**
- A new field is added to a widget struct but CI still passes (because preset tests only check parseability, not completeness).
- A specific preset fails at runtime with "missing field: button.hover_bg" but this was never caught in tests.
- Platform-specific failures: "works on GNOME, fails on KDE" because the KDE TOML was not updated.

**Phase to address:**
Phase 4 (Platform Preset Slim-Down). The completeness test must be added in Phase 2 (when validate() is created) and enforced from that point forward.

---

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Hand-writing all 28 Resolved* structs instead of macro | Faster to implement initially, easier to understand | Every struct change requires updating 3 locations (Option struct, Resolved struct, resolve function); forgotten updates cause silent bugs | Never at this scale (28 struct pairs). A generation macro is mandatory |
| Embedding Adwaita CSS colors in GNOME reader (current pattern) | Fewer files, simpler initial implementation | Violates OS-first principle; updating Adwaita CSS values requires recompilation instead of TOML edit | Only during initial reader implementation; extract to `adwaita.toml` before release |
| Skipping the second resolve() pass after app TOML overlay | Simplifies pipeline, saves ~200 field iterations | If app TOML overrides `accent`, derived fields (primary_bg, slider.fill) still have the old values; theme looks inconsistent | Never. The second resolve pass is architecturally required |
| Using `..Default::default()` in ResolvedTheme construction | Compiles even when fields are missing | Silent zero/default values instead of proper validation errors; zero-alpha colors, zero-size widgets | Never. Struct literal syntax without `..Default` catches missing fields at compile time |
| Not testing the inactive variant (variant not detected by OS) | Reduces test surface by half | Inactive variant is TOML-only (no OS reader data); missing TOML fields cause validate() failures only when user switches light/dark mode | Only during early development; must be tested before release |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| KDE `configparser` INI | Assuming all KDE sections exist; `[Colors:Header]` was added in KF 5.71, `[Colors:Complementary]` may be absent on older KDE | Always use `ini.get()` which returns `Option<String>`; never panic on missing sections |
| GNOME gsettings via `ashpd` portal | Assuming the portal is available; on XFCE/MATE the portal may not respond | Set a timeout on portal requests; fall back to preset-only mode on timeout/error |
| macOS `objc2` selectors | Calling version-specific selectors without `respondsToSelector:` check; macOS 14+ APIs crash on macOS 13 | Use runtime version checks or `respondsToSelector:` before calling new APIs |
| Windows `DwmGetColorizationColor` | Assuming DWM is always available; Server Core has no DWM | Handle `HRESULT` errors from DWM calls; fall back to `GetSysColor` values |
| TOML `toml::to_string_pretty` | Per-widget structs produce deeply nested unreadable TOML with empty sections | Use `skip_serializing_if = "is_empty"` on every widget field; consider inline table format for FontSpec |
| `impl_merge!` with nested FontSpec | FontSpec is nested but has sub-field merge semantics, not atomic merge | Ensure widget structs list `font` under `nested {}` in `impl_merge!`, not `option {}`, so merge recurses into FontSpec sub-fields |
| Qt font string parsing | Assuming Qt6 weight scale (1-1000) on all systems | Detect Qt version from field count; convert Qt5 0-99 scale to CSS 100-900 before storing in FontSpec |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Calling `resolve()` on every frame or render call | ~200 field checks per call; at 60fps = 12,000 checks/sec; CPU overhead visible in profiler | Cache the `ResolvedTheme`. Re-resolve only on OS theme change, user TOML modification, or explicit API call | Immediately visible in profiler but functionally correct. Always cache |
| Cloning entire `ThemeDefaults` into each widget during resolve | 24 unnecessary copies of defaults struct (~2KB each) | `resolve()` takes `&ThemeDefaults` by reference, clones only individual Option values into widget fields when needed | Not a real concern at ~200 fields (<1ms, <10KB total). But pointless allocation adds up if resolve() is called frequently |
| String cloning in FontSpec family fields during resolve | Each font inheritance clones the family String; 24 widget fonts = 24 string clones | Use `Arc<str>` for font family names if profiling shows string allocation is a bottleneck | Not a real concern at this scale. Short strings cloned ~24 times at theme load time |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| `validate()` fails with "32 missing fields" and no guidance on which TOML section to fix | User sees a wall of field paths like `tooltip.padding_horizontal` with no idea which file to edit | Group missing fields by source: "These should come from the OS reader (check platform support)" vs "These should be in your TOML (add [light.tooltip] section)" |
| App developer overrides accent in TOML but derived fields (primary_bg, slider.fill) do not update | Theme looks broken: buttons are one accent color, sliders are another | Second resolve() pass after app TOML overlay must propagate accent changes. Document: "Override accent, all derived colors update automatically" |
| Cross-platform preset missing a field causes cryptic error on one platform only | User loads "catppuccin-mocha" preset, works on GNOME, fails on KDE with "missing field: input.background" | Preset completeness tests must run against all 4 platform configurations. Cross-platform presets must provide ALL non-derived fields |
| Old-format TOML silently produces empty theme after upgrade | User's custom `[light.colors]` TOML loads without error but all theme values are `None` | Detect old-format sections and emit a clear deprecation warning or migration error with specific guidance |

## "Looks Done But Isn't" Checklist

- [ ] **resolve() inheritance**: every `<-` rule from the inheritance table has a corresponding unit test. Missing test = missing rule = silent None at runtime. Count tests vs rules; they must match.
- [ ] **FontSpec sub-field inheritance**: partial override test exists (set only `menu.font.size`, verify `menu.font.family` inherits from defaults). Missing = partial TOML overrides break.
- [ ] **TextScaleEntry line_height computation**: line_height uses `defaults.line_height * entry.resolved_size`, NOT `defaults.line_height * defaults.font.size`. Missing = text scale line heights are wrong.
- [ ] **Platform TOML completeness**: every preset + resolve() produces a valid ResolvedTheme. Missing = "works on my machine" platform bias.
- [ ] **Cross-platform preset completeness**: catppuccin/nord/etc. provide ALL non-derived fields. Missing = presets fail on validate().
- [ ] **Second resolve() pass**: after app TOML overlay, accent-derived fields reflect the new accent. Missing = custom accent themes look broken.
- [ ] **Qt5/Qt6 font weight detection**: KDE reader handles both 0-99 and 1-1000 weight scales. Missing = wrong font weight on KDE 5 vs 6.
- [ ] **Inactive variant**: the variant NOT detected by OS (e.g., light when user is in dark mode) validates as TOML-only. Missing = light/dark toggle produces validation errors.
- [ ] **Empty table suppression**: serialized TOML does not contain empty `[light.button]` sections. Missing = generated TOMLs unreadable.
- [ ] **TOML backward compat**: old-format TOMLs (`[light.colors]`) either parse with deprecation warnings or cause clear errors. Missing = silent data loss.
- [ ] **ResolvedTheme struct literal construction**: no `..Default::default()` in validate()/construction. Missing = silent zero values bypass validation.

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| TOML field path breakage (silent data loss) | MEDIUM | Add serde aliases for old field names; release patch with backward compat layer; publish migration guide |
| Struct duplication drift (forgotten Resolved* field) | LOW | Add compile-time field count assertion; fix the missing field; the compiler catches this if struct literal syntax is used |
| resolve() ordering bug | LOW | Reorder resolve phases; add the missing test; fix is localized to resolve() |
| Qt font weight mismatch | LOW | Add field-count-based version detection and conversion; fix is localized to KDE font parser |
| Platform API crash on older OS | MEDIUM | Add version guard; release patch; users on older OS may have been crashing |
| Connector compile breakage | LOW-MEDIUM | Keep old ThemeVariant API available during transition; update connectors incrementally |
| Platform TOML missing field | LOW | Add the missing field to the TOML; completeness test prevents recurrence |
| TextScaleEntry line_height wrong | LOW | Fix the computation to use entry size; add targeted test |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| TOML field path breakage (#1) | Phase 1 (Data Model) | Serde alias tests: old-format TOML parses correctly into new structs |
| Struct explosion (#2) | Phase 2 (ResolvedTheme) | Macro generates both Option and concrete structs; field count assertions pass |
| resolve() ordering (#3) | Phase 2 (ResolvedTheme) | One test per inheritance rule; all pass; resolve phases documented in code comments |
| Empty table serialization (#4) | Phase 1 (Data Model) | Round-trip test: serialize -> deserialize preserves equality; no empty sections in output |
| Qt font weight (#5) | Phase 3 (OS Readers) | Test Qt5 (weight=50, 16 fields) and Qt6 (weight=400, 18 fields) both produce CSS weight=400 |
| Platform API availability (#6) | Phase 3 (OS Readers) | Each new API call has a version guard; tested on minimum supported OS version |
| FontSpec sub-field inheritance (#7) | Phase 2 (ResolvedTheme) | Partial override test: set only size, verify family inherits |
| TextScaleEntry line_height (#8) | Phase 2 (ResolvedTheme) | Test: entry size=26, defaults.line_height=1.2 -> line_height=31.2, not line_height=defaults.font.size*1.2 |
| TOML-struct sync (#9) | Phase 2+4 (ResolvedTheme + Presets) | Integration test: every preset + simulated OS reader -> resolve() -> validate() succeeds |

## Sources

- Direct codebase analysis: `native-theme/src/model/`, `native-theme/src/kde/fonts.rs`, `native-theme/src/presets.rs`, `native-theme/src/lib.rs`
- Design documents: `docs/todo_v0.5.1_theme-variant.md`, `docs/todo_v0.5.1_resolution.md`, `docs/todo_v0.5.1_inheritance-rules.md`
- [Qt 6 QFont documentation](https://doc.qt.io/qt-6/qfont.html) -- weight scale 1-1000, toString() 18 fields
- [Qt 5 QFont documentation](https://doc.qt.io/qt-5/qfont.html) -- weight scale 0-99, toString() 15-16 fields
- [Qt Forum: QFont::fromString() change between Qt 4 and Qt 5](https://forum.qt.io/topic/94958/qfont-fromstring-change-between-qt-4-and-qt-5)
- [serde_with skip_serializing_none](https://docs.rs/serde_with/latest/serde_with/attr.skip_serializing_none.html) -- nested struct serialization behavior
- [serde issue #2451: Option::None exclusion](https://github.com/serde-rs/serde/issues/2451) -- discussion of nested Option handling
- [Backward Compatible Data Serialization with Serde-Flow](https://ivanbyte.medium.com/backward-compatible-data-de-serialization-with-serde-flow-in-rust-c87a2e8bc9ea) -- migration strategies

---
*Pitfalls research for: per-widget theme architecture with resolution pipeline*
*Researched: 2026-03-27*
