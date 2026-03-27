# Feature Research

**Domain:** Per-widget theme architecture and resolution pipeline for cross-platform native theming
**Researched:** 2026-03-27
**Confidence:** HIGH (primary sources: GTK/libadwaita docs, Qt docs, WinUI3/Fluent docs, Apple HIG, Material Design 3, existing codebase design docs)

## Feature Landscape

### Table Stakes (Users Expect These)

Features that consumers of native-theme assume exist. Missing these means the crate fails its core promise of "use native OS theme values without Option juggling."

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Per-widget theme structs with colors+font+sizing+geometry | Every real theme system (GTK CSS, Qt QStyle, WinUI3 resource dictionaries) exposes per-widget properties. The existing flat ThemeColors with 36 roles forces consumers to manually map `colors.button` to their button widget and `colors.tooltip` to tooltips -- tedious and error-prone. Per-widget structs let consumers write `theme.button.background` directly. | HIGH | 25 widget structs (window, button, input, checkbox, menu, tooltip, scrollbar, slider, progress_bar, tab, sidebar, toolbar, status_bar, list, popover, splitter, separator, switch, dialog, spinner, combo_box, segmented_control, card, expander, link) each with their own color + font + sizing + geometry fields. Already designed in `todo_v0.5.1_theme-variant.md`. Major restructure of the data model but the design is settled. |
| ThemeDefaults for shared base properties | All four platform theme systems have a "base" layer. GTK CSS has initial/inherited property values. Qt QPalette has role-based defaults (Window, WindowText, Button, etc.) that widgets inherit. WinUI3 has system-level brush resources (`TextFillColorPrimary`, `ControlFillColorDefault`). macOS has `NSColor.controlTextColor`, `NSColor.controlBackgroundColor`. The ThemeDefaults struct serves this exact role -- the shared pool that per-widget `None` fields inherit from. | MEDIUM | ThemeDefaults is a larger struct than any individual widget (base colors, font, mono_font, geometry, spacing, icon_sizes, accessibility). Design is documented. Complexity is in getting the field set right, not the mechanism. |
| Non-optional ResolvedTheme output | Every production theme system resolves to concrete values before rendering. GTK CSS computes final values via cascade+inheritance and every node has exactly one computed value per property. Qt QStyle::pixelMetric returns a concrete int, never "no value." WinUI3 ThemeResource evaluates to a concrete SolidColorBrush at runtime. SwiftUI's `ShapeStyle.resolve(in:)` returns a concrete resolved value. Consumers of native-theme currently unwrap Options everywhere or fabricate fallbacks -- both are wrong. | MEDIUM | ResolvedTheme mirrors ThemeVariant but with plain values (no Option). The `validate()` step that converts ThemeVariant to ResolvedTheme is a straightforward field-by-field check. Complexity is in the sheer number of fields (~200+), not conceptual difficulty. |
| Universal resolve() with inheritance rules | CSS cascade and inheritance is the gold standard: properties are either inherited (font-family, color) or non-inherited (margin, padding), and every CSS engine applies these rules universally. GTK4 explicitly supports `inherit`, `initial`, `unset` keywords per the CSS spec. Qt QPalette resolves roles via inheritance (Button inherits from Window if not explicitly set). WinUI3 uses BasedOn style chaining. resolve() implements the equivalent for native-theme: `button.font <- defaults.font`, `button.radius <- defaults.radius`, `slider.fill <- defaults.accent`. | HIGH | ~80 inheritance rules documented in `todo_v0.5.1_inheritance-rules.md`. Must handle FontSpec sub-field inheritance (family/size/weight independently), TextScaleEntry inheritance, accent-derived color propagation, and 5 platform-divergent fallbacks. The logic is not complex per-rule but the volume and testing coverage make this the most labor-intensive feature. |
| OS-first pipeline (OS reader + TOML overlay + resolve) | This is THE core architectural promise. Every platform theme system reads live OS values first: GTK reads the Adwaita stylesheet and gsettings, Qt reads kdeglobals and the current QStyle, WinUI3 reads system accent color and UISettings. The TOML overlay fills gaps the OS cannot provide (design constants), just as libadwaita's built-in CSS provides defaults that gsettings doesn't expose. The existing `from_system()` returns raw OS values without design constants or inheritance resolution -- incomplete. | HIGH | Changes `from_system()` to run the full pipeline. Requires coordination between OS readers, platform TOMLs, and resolve(). Platform-to-preset mapping (macOS->macos-sonoma, KDE->kde-breeze, GNOME->adwaita, Windows->windows-11). Two resolve passes when app TOML is provided. |
| FontSpec with family/size/weight | All four platforms have distinct per-widget fonts. macOS: `+menuFontOfSize:`, `+toolTipsFontOfSize:`, `+titleBarFontOfSize:`. KDE: `menuFont`, `toolBarFont`, `activeFont` in kdeglobals. Windows: `lfCaptionFont`, `lfMenuFont`, `lfStatusFont` in SystemParametersInfo. Even GNOME has `font-name`, `monospace-font-name`, `titlebar-font` gsettings. FontSpec (family, size, weight) captures the common denominator. Current ThemeFonts only has family+size globally -- no per-widget fonts, no weight. | LOW | FontSpec is a small 3-field struct. Already designed. The implementation is trivial; the complexity is in the OS readers that populate it. |
| Accessibility signals (reduce_motion, high_contrast, text_scaling_factor, reduce_transparency) | Every platform exposes these, and consumers need them. macOS: `accessibilityDisplayShouldReduceMotion`, `accessibilityDisplayShouldIncreaseContrast`. Windows: `SPI_GETCLIENTAREAANIMATION`, `SPI_GETHIGHCONTRAST`, `UISettings.TextScaleFactor`. KDE: `AnimationDurationFactor`, `forceFontDPI`. GNOME: `enable-animations`, `text-scaling-factor`, portal `contrast`/`reduced-motion`. These are runtime OS state, not preset-authored. They live in ThemeDefaults because they are global. | MEDIUM | The accessibility fields are already designed in ThemeDefaults. The implementation work is extending OS readers (4 platforms) to read these values. Not conceptually hard, but platform-specific FFI/API work. |
| Error reporting for unresolved fields | WinUI3 logs diagnostic messages when theme resources fail to resolve. GTK CSS logs warnings for invalid property values. SwiftUI prints diagnostics for missing environment values in debug builds. When native-theme's resolve() pipeline leaves fields as None, the consumer must know *which* fields are missing and *why*. A ThemeResolutionError listing all missing field paths is essential. | LOW | ThemeResolutionError is a single struct with `Vec<String>` of missing field paths. Already designed. |

### Differentiators (Competitive Advantage)

Features that no competing cross-platform theme crate provides. These set native-theme apart.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| TextScale type ramp (caption, section_heading, dialog_title, display) | Every major design system has a type ramp. Material Design 3 has 15 styles across display/headline/title/label/body. Apple HIG has 11 styles from Large Title (34pt Bold) down to Caption 2 (11pt Regular). Windows Fluent has 9 styles from Display (68epx Semibold) through Caption (12epx Regular). libadwaita has 8 CSS classes from `.title-1` through `.caption`. No cross-platform Rust crate maps these into portable named roles. The 4-entry TextScale (caption, section_heading, dialog_title, display) captures the roles most commonly needed by UI apps while remaining manageable in a struct. Extensible via `#[non_exhaustive]`. | MEDIUM | TextScaleEntry has size, weight, line_height. OS readers compute platform-specific values (macOS: NSFont.TextStyle, KDE: `font.size * multiplier`, GNOME: `font.size * CSS_percentage`). The mapping is well-documented. Complexity is moderate because each platform computes differently. |
| Per-widget accent-derived state colors | GTK CSS derives checked/active states from the accent color via CSS variables. WinUI3 has per-control accent resource keys (`ToggleSwitchFillOn` defaults to `SystemAccentColor`). Qt Breeze derives accent states in its QStyle subclass. native-theme makes these explicit `Option` fields: `button.primary_bg`, `checkbox.checked_bg`, `slider.fill`, `progress_bar.fill`, `switch.checked_bg`. The resolve() pipeline fills them from `defaults.accent` if not overridden. Consumers get direct access to per-widget accent colors without deriving them manually. | LOW | These are just additional fields on existing widget structs, with simple inheritance rules (all `<- defaults.accent`). Already documented. |
| DialogButtonOrder and dialog layout metrics | No other cross-platform theme crate captures dialog button ordering conventions. macOS/GNOME: affirmative action trailing (Cancel ... OK). Windows/KDE: affirmative action leading (OK Cancel). Dialog sizing (min/max width/height), content padding, and button spacing also vary significantly. This is a genuine usability differentiator -- getting dialog layout right on each platform is a known pain point. | LOW | DialogTheme and DialogButtonOrder enum are simple structs. The value is in the research and correct per-platform defaults, not code complexity. |
| IconSizes per context | macOS, Windows, KDE, and GNOME all define different icon sizes for toolbar, small, large, dialog, and panel contexts. KDE reads these from the icon theme's `index.theme`. Windows has `SM_CXSMICON` and `SM_CXICON` system metrics. native-theme's IconSizes struct captures these, allowing UI toolkits to request the correct icon size for each context without hardcoding. | LOW | 5-field struct (toolbar, small, large, dialog, panel). KDE provides all via index.theme parsing; others are mostly design constants. |
| Two-phase resolution with app TOML override | No competing crate separates "OS + platform TOML + resolve()" from "app override + re-resolve()". This lets app developers override accent color in their TOML and have it automatically propagate to primary_bg, slider.fill, etc. via the second resolve() pass. Set one field, get 8 derived fields updated. | MEDIUM | The mechanism is merge + resolve called twice. The subtlety is ensuring the second resolve() respects already-set fields from the app TOML (it must not re-derive fields the app explicitly overrode). |

### Anti-Features (Commonly Requested, Often Problematic)

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Per-widget style class system (like GTK CSS classes) | CSS-like `.primary`, `.destructive`, `.flat` variants per widget | Explodes the struct surface area multiplicatively. A ButtonTheme with 4 style variants becomes 4x the fields. GTK does this because CSS is dynamically evaluated; a static struct cannot. Also, style class semantics differ across platforms (`.flat` in GTK vs. transparent button in WinUI3 vs. no equivalent in KDE). | Use the existing per-widget color fields. `button.primary_bg`/`button.primary_fg` covers the most important variant. Additional variants are widget-level rendering decisions, not theme data. |
| Full CSS cascade with specificity and selectors | Mimicking GTK4 or web CSS with selector-based styling | CSS cascade is a runtime evaluation engine. native-theme is a data structure library that produces concrete values. Implementing specificity, selector matching, and cascade resolution would be a CSS engine, not a theme data crate. The overhead and complexity would dwarf the actual theme data. | The merge() + resolve() pipeline provides a simpler two-layer cascade (OS + TOML) that covers real use cases without selector complexity. |
| Animation/transition tokens (duration, easing curves) | Design systems like Material Design 3 define motion tokens (duration, easing). Some consumers want animation parameters from the theme. | Animation parameters are not standardized across platforms in any queryable way. macOS has implicit Core Animation durations. Windows Fluent defines motion tokens but does not expose them via system APIs. KDE has `AnimationDurationFactor` (a multiplier, not durations). GNOME has `enable-animations` (boolean). The only universally useful signal is `reduce_motion` (boolean). | Provide `reduce_motion: bool` in ThemeDefaults (already planned). Leave duration/easing to the UI toolkit. |
| Runtime theme change notifications / watchers | Consumers want to subscribe to "theme changed" events | This is a runtime observation concern, not a data model concern. Each platform has different notification mechanisms (NSNotification, Windows UISettings event, KDE file watcher, GNOME gsettings callback). Adding reactive observation would couple native-theme to async runtimes and platform event loops. | Document how to re-call `from_system()` on platform-specific change signals. Let the UI toolkit handle its own event integration. |
| Computed/derived color algebra (darken 10%, lighten 20%, mix) | Some theme systems derive hover/pressed states via color math | Color algebra is a rendering concern, not a theme data concern. Different widgets need different derivation formulas. Embedding color math in the theme struct conflates data with rendering logic. Also, the "correct" derivation differs per platform (GTK uses mix(), macOS uses alpha overlays, WinUI3 uses named state resources). | Provide the distinct named colors where platforms have them (e.g., `scrollbar.thumb_hover`). For states the theme does not cover, let the rendering layer handle derivation. |
| Font fallback chains | Multiple font families in preference order | Font fallback is a text shaping concern handled by the text layout engine (HarfBuzz, DirectWrite, Core Text). The theme system provides the preferred family name; the layout engine handles fallback when glyphs are missing. Embedding fallback chains in theme data duplicates what the layout engine already does. | Provide `font.family` as a single string. The layout engine handles fallback. |
| Per-widget state colors (hover, pressed, focused, disabled for every widget) | Full interaction state coverage like CSS `:hover`, `:active`, `:focus` | Multiplies field count by 4-5x per widget. A ButtonTheme with 14 fields would become 70+ fields. Most platforms do NOT expose per-widget state colors via APIs -- they compute states at render time. Only WinUI3 has per-state resource keys, and even there they are derived from base colors via opacity. | Provide base colors per widget. Provide `disabled_opacity` for disabled states. Let the rendering layer handle hover/pressed/focus states via standard color operations (darken, lighten, alpha). |

## Feature Dependencies

```
ThemeDefaults
    |
    +--requires--> FontSpec (font, mono_font are FontSpec)
    |
    +--requires--> ThemeSpacing (spacing field)
    |
    +--requires--> IconSizes (icon_sizes field)
    |
    +--enhances--> Accessibility signals (text_scaling_factor, reduce_motion, etc.)

Per-Widget Structs (ButtonTheme, InputTheme, etc.)
    |
    +--requires--> ThemeDefaults (inheritance source for resolve())
    |
    +--requires--> FontSpec (per-widget font fields)
    |
    +--requires--> DialogButtonOrder (DialogTheme.button_order)

TextScale
    |
    +--requires--> TextScaleEntry
    |
    +--requires--> ThemeDefaults (font.size, font.weight, line_height for inheritance)

resolve()
    |
    +--requires--> ThemeDefaults (source of inherited values)
    |
    +--requires--> Per-Widget Structs (target of inheritance fill)
    |
    +--requires--> TextScale (fills TextScaleEntry sub-fields)

ResolvedTheme
    |
    +--requires--> resolve() (must run before validate())
    |
    +--requires--> Per-Widget Structs (mirrors structure without Option)

OS-First Pipeline
    |
    +--requires--> Extended OS Readers (populate new per-widget fields)
    |
    +--requires--> Platform TOMLs (fill design constant gaps)
    |
    +--requires--> resolve() (fill inheritance after merge)
    |
    +--requires--> ResolvedTheme (final output)

Connector Updates
    |
    +--requires--> ResolvedTheme (accept &ResolvedTheme instead of &ThemeVariant)
    |
    +--conflicts--> Current connector API (breaking change)
```

### Dependency Notes

- **Per-Widget Structs require ThemeDefaults:** resolve() fills None fields from ThemeDefaults. Without ThemeDefaults, there is nothing to inherit from.
- **resolve() requires both ThemeDefaults and Per-Widget Structs:** The inheritance rules reference `defaults.X` fields as sources and `widget.Y` fields as targets. Both must exist in the same ThemeVariant.
- **ResolvedTheme requires resolve():** validate() converts Option to concrete values. If resolve() has not run, most fields will still be None and validation will fail.
- **OS-First Pipeline requires Extended OS Readers:** OS readers must populate the new per-widget fields (fonts, colors, accessibility) before the pipeline can produce a fully resolved theme.
- **Connector Updates conflict with current API:** Changing connectors from `&ThemeVariant` to `&ResolvedTheme` is a breaking change. Connectors must be updated after ResolvedTheme exists.
- **TextScale requires ThemeDefaults:** TextScaleEntry sub-field inheritance uses `defaults.font.size`, `defaults.font.weight`, and `defaults.line_height` as sources.

## MVP Definition

### Launch With (v0.5.1 -- Core Architecture)

Minimum viable per-widget architecture -- what is needed to validate the design and unblock consumers.

- [ ] **ThemeDefaults struct** -- base colors, font (FontSpec), mono_font, geometry, spacing, focus ring, accessibility signals
- [ ] **FontSpec struct** -- family, size, weight
- [ ] **Per-widget structs (all 25)** -- each with their specific color + font + sizing + geometry fields as documented
- [ ] **Restructured ThemeVariant** -- replace flat ThemeColors/ThemeFonts/ThemeGeometry with ThemeDefaults + per-widget structs
- [ ] **resolve()** -- universal inheritance function filling None from ThemeDefaults per documented rules
- [ ] **ResolvedTheme + validate()** -- non-optional output structs, error-on-missing-field
- [ ] **TextScale + TextScaleEntry** -- 4-entry type ramp (caption, section_heading, dialog_title, display)
- [ ] **DialogButtonOrder enum** -- platform dialog convention
- [ ] **IconSizes struct** -- per-context icon dimensions
- [ ] **Merge support for new structs** -- extend impl_merge! to handle nested per-widget structs with FontSpec

### Add After Validation (v0.5.2 -- OS Readers + Pipeline)

Features to add once the core data model is proven correct.

- [ ] **Extended OS readers** -- macOS, Windows, KDE, GNOME readers populate all new fields (per-widget fonts, colors, accessibility, text scale, icon sizes)
- [ ] **OS-first pipeline in from_system()** -- OS reader + platform TOML merge + resolve()
- [ ] **Platform TOML slimming** -- remove OS-readable fields from platform default TOMLs, keep only design constants
- [ ] **App TOML overlay + second resolve() pass** -- developer customization with accent propagation
- [ ] **Cross-platform preset updates** -- update catppuccin, nord, etc. to populate all new fields

### Future Consideration (v0.6+)

Features to defer until the architecture is stable and connectors are updated.

- [ ] **Connector updates** -- change native-theme-gpui and native-theme-iced to accept &ResolvedTheme
- [ ] **WindowTheme** -- title bar colors, inactive state, per-platform window chrome (depends on consumer needs)
- [ ] **Additional TextScale entries** -- if consumers need finer granularity beyond the 4-entry set

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Per-widget structs (all 25) | HIGH | HIGH | P1 |
| ThemeDefaults | HIGH | MEDIUM | P1 |
| FontSpec | HIGH | LOW | P1 |
| resolve() inheritance | HIGH | HIGH | P1 |
| ResolvedTheme + validate() | HIGH | MEDIUM | P1 |
| TextScale type ramp | MEDIUM | MEDIUM | P1 |
| DialogButtonOrder + dialog layout | MEDIUM | LOW | P1 |
| IconSizes | MEDIUM | LOW | P1 |
| Accessibility signals | HIGH | MEDIUM | P1 |
| Extended OS readers | HIGH | HIGH | P2 |
| OS-first pipeline | HIGH | MEDIUM | P2 |
| Platform TOML slimming | MEDIUM | MEDIUM | P2 |
| App TOML overlay + re-resolve | MEDIUM | MEDIUM | P2 |
| Cross-platform preset updates | MEDIUM | MEDIUM | P2 |
| Connector migration to ResolvedTheme | HIGH | MEDIUM | P3 |

**Priority key:**
- P1: Core data model -- must ship together as the architecture restructure
- P2: OS integration -- builds on P1, requires per-platform work
- P3: Breaking consumer API change -- defer until P1 and P2 are stable

## Competitor Feature Analysis

native-theme has no direct Rust competitor doing cross-platform native theme reading. The "competitors" are the platform theme systems themselves and what they expose.

| Feature | GTK CSS / libadwaita | Qt QPalette + QStyle | WinUI3 Resources | macOS NSColor + NSFont | native-theme approach |
|---------|---------------------|---------------------|------------------|----------------------|----------------------|
| Per-widget colors | Full CSS selectors per widget node, arbitrary properties | QPalette roles (Window, Button, Base, etc.) mapped per widget; QStyle::pixelMetric for sizing | Per-control resource keys: `ButtonBackground`, `TextControlBackground`, `ToggleSwitchFillOn`, etc. 100+ named keys per control template | `NSColor.controlBackgroundColor`, `buttonTextColor`, etc. ~50 named system colors | Per-widget struct with explicit color fields. Simpler than CSS selectors, richer than QPalette roles. |
| Theme inheritance | CSS cascade: inherit/initial/unset per property. Widget tree inheritance for color, font-family, font-size. Initial values for margin, padding. | QPalette role inheritance: Button defaults to Window. QProxyStyle can chain to platform style. | BasedOn style chaining. Lightweight styling overrides specific resource keys. ThemeDictionaries provide light/dark/high-contrast variants. | No formal inheritance. System colors are independent named values. Dynamic Type provides text size scaling. | resolve() fills None from ThemeDefaults. FontSpec sub-field inheritance. Two-pass resolution with app overlay. |
| Resolved output | Computed values -- GTK ensures every CSS node has exactly one computed value per property | QStyle always returns concrete values from pixelMetric(), drawPrimitive(), etc. | ThemeResource resolves to concrete brush at runtime. Never returns "no value." | NSColor returns concrete CGColor. Never nil for system colors. | ResolvedTheme with plain values. validate() fails if any field is None. |
| Type scale | .title-1 through .title-4, .heading, .caption-heading, .caption, .body, .document (8 classes with relative sizing via %) | QFont roles: Title, SmallCaption, etc. (limited) | 9 named styles: Display (68epx) through Caption (12epx) with explicit size/weight/line-height | 11 TextStyles: largeTitle (34pt) through caption2 (11pt) with weight/leading/tracking | 4 named entries: caption, section_heading, dialog_title, display. Maps the most-needed roles. Extensible via #[non_exhaustive]. |
| Accessibility signals | `text-scaling-factor` gsetting, `enable-animations` gsetting, portal `contrast`/`reduced-motion` | KDE: `AnimationDurationFactor`, `forceFontDPI`. Qt: `QApplication::font()` respects DPI. | `UISettings.TextScaleFactor` (1.0-2.25), `SPI_GETCLIENTAREAANIMATION`, `SPI_GETHIGHCONTRAST` | `accessibilityDisplayShouldReduceMotion`, `accessibilityDisplayShouldIncreaseContrast`, `accessibilityDisplayShouldReduceTransparency` | text_scaling_factor, reduce_motion, high_contrast, reduce_transparency in ThemeDefaults. Read from OS, not preset-authored. |
| Dialog layout | Adwaita: AdwAlertDialog with specific padding/spacing/button layout | KDE: KMessageBox with QDialogButtonBox (AcceptRole/RejectRole ordering) | ContentDialog with specific sizing constraints and button layout | NSAlert with system-managed layout | DialogTheme with min/max size, padding, button_spacing, button_order enum, title_font, radius, icon_size |

## Sources

### Official Documentation (HIGH confidence)
- [GTK4 CSS Properties](https://docs.gtk.org/gtk4/css-properties.html) -- supports inherit/initial/unset, length resolution via em/ex/rem
- [GTK4 CSS Overview](https://docs.gtk.org/gtk4/css-overview.html) -- widget CSS nodes, style classes, cascade
- [libadwaita Style Classes](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/style-classes.html) -- .title-1 through .caption, typography hierarchy
- [Qt Style Reference](https://doc.qt.io/qt-6/style-reference.html) -- QStyle, pixelMetric, subcontrols, palette-aware widgets
- [Qt Style Sheets](https://doc.qt.io/qt-6/stylesheet.html) -- per-widget CSS-like styling, cascade inheritance
- [WinUI3 XAML Theme Resources](https://learn.microsoft.com/en-us/windows/apps/develop/platform/xaml/xaml-theme-resources) -- ThemeDictionaries, lightweight styling, per-control resource keys
- [WinUI3 XAML Styles](https://learn.microsoft.com/en-us/windows/apps/develop/platform/xaml/xaml-styles) -- BasedOn chaining, implicit styles
- [Windows Typography](https://learn.microsoft.com/en-us/windows/apps/design/signature-experiences/typography) -- Fluent type ramp: Display (68epx) through Caption (12epx)
- [Apple HIG Typography](https://developer.apple.com/design/human-interface-guidelines/typography) -- macOS text styles, Dynamic Type, SF Pro
- [Material Design 3 Type Scale](https://m3.material.io/styles/typography/type-scale-tokens) -- 5 roles x 3 sizes = 15 tokens
- [Fluent 2 Typography](https://fluent2.microsoft.design/typography) -- platform-specific type ramps
- [GNOME HIG Typography](https://developer.gnome.org/hig/guidelines/typography.html) -- heading levels, caption usage
- [CSS Cascade and Inheritance](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Cascade) -- inherit/initial/unset, computed values
- [SwiftUI colorScheme](https://developer.apple.com/documentation/swiftui/environmentvalues/colorscheme) -- environment-based theme resolution

### Community / Analysis (MEDIUM confidence)
- [Apple Typography Gist](https://gist.github.com/eonist/b9c180a67980c6e18a5184f19bff68fa) -- macOS text style sizes/weights/tracking
- [SwiftUI Theming](https://alexanderweiss.dev/blog/2025-01-19-effortless-swiftui-theming) -- resolve() pattern in SwiftUI ShapeStyle
- [XAML Lightweight Styling](https://www.reflectionit.nl/blog/2021/xaml-lightweight-styling-done-right) -- per-control resource key patterns

---
*Feature research for: per-widget theme architecture and resolution pipeline*
*Researched: 2026-03-27*
