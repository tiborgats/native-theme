# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Breaking Changes

- **native-theme**: the `-live` presets are gone: `kde-breeze-live`, `adwaita-live`, `macos-sonoma-live` and `windows-11-live` (`Theme::preset` now returns `UnknownPreset` for them), with `PlatformPreset::is_live` and `PlatformPreset::live_name()`. The platform readers merge over the platform preset (`kde-breeze`, `adwaita`, `macos-monterey`, `windows-11`). Every key of a `-live` file stated its platform preset's value, so the layer changed no resolved value.
- **native-theme**: the `material` preset is now `material-3`: `Theme::preset("material")` returns `UnknownPreset`, with no alias.
- **native-theme**: the `macos-sonoma` preset is now `macos-monterey`, named by the first macOS version whose look it carries: Monterey (12), whose AppKit release notes end the accent-colour highlight of a clicked button; it covers macOS 12 to 15. `Theme::preset("macos-sonoma")` returns `UnknownPreset`, with no alias.
- **native-theme**: the `ios` preset is now `ios-13`, named by the first iOS version whose look it carries: the flat look with its dark variant that iOS 13 introduced and iOS 18 still draws. `Theme::preset("ios")` returns `UnknownPreset`, with no alias.
- **native-theme**: `gnome::GnomePortalData` has no `font_dpi` field: the GNOME reader no longer reads `Xft.dpi` (see *Fixed*).
- **native-theme**: the data-model types have one public path, `native_theme::theme`: the `model` module is private, and the sub-module paths `theme::font::FontSize`, `theme::widgets::ButtonTheme` and the like are gone (`theme::FontSize`, `theme::ButtonTheme`). `theme` lists its types by name, so docs.rs shows them on the `theme` page instead of a `pub use crate::model::*` line. `detect_icon_theme()`, reachable only as `model::icons::detect_icon_theme`, is `detect::detect_icon_theme` (see the `detect` line below).
- **native-theme**: `FontSize` no longer implements `Default`: its `Px(0.0)` was a size validation always rejects, and nothing used it.
- **native-theme**: `PresetInfo` is `#[non_exhaustive]` and has no `light_only` field, which every bundled preset set to `false` (each one has both variants).
- **native-theme**: `ResolvedTheme`, `ResolvedDefaults`, `ResolvedTextScale`, `ResolvedTextScaleEntry`, `ResolvedIconSizes`, `ResolvedDefaultsBorder` and `Resolved` are `#[non_exhaustive]`, as the generated `Resolved*Theme` widget structs already were, so they can no longer be built by struct literal outside the crate; a field added to them later is not a breaking change. `ResolvedWidgetBorder`, `ResolvedPadding` and `ResolvedFontSpec` stay constructible.
- **native-theme**: `AccessibilityPreferences` is `#[non_exhaustive]`, so a flag added to it later is not a breaking change; outside the crate it is built from `AccessibilityPreferences::DEFAULT` or `default()` with its fields set, no longer by struct literal or `..Default::default()`.
- **native-theme**: `Error::Io` is `Io { path, source }`: `Theme::from_file` names the file it could not read, displayed as "cannot read theme file `{path}`: {source}" (it was "I/O error: {source}", without the path). The blanket `impl From<std::io::Error> for Error` is gone, so `?` no longer turns an `io::Error` into a `native_theme::Error`.
- **native-theme**: `IconRole::ALL` is a `&'static [IconRole]`, as the icon enums native-theme-build generates are (it was `[IconRole; 42]`), so a role added later does not change its type; iterate it with `IconRole::ALL.iter().copied()`.
- **native-theme**: `IconSetChoice` is `#[non_exhaustive]`, so a choice added later is not a breaking change; a `match` on it outside the crate needs a `_` arm.
- **native-theme**: `IconData::Rgba` holds an `RgbaData` whose fields are private (`IconData::Rgba(rgba)`, read with `rgba.width()`, `rgba.height()`, `rgba.data()` and `rgba.into_data()`), built with `IconData::rgba(width, height, data)`, which returns `None` unless `data` holds `width × height × 4` bytes; deserializing an `Rgba` with a buffer of another length fails. The serialized shape is unchanged. It was a struct variant with public fields, so `IconData::Rgba { width: 16, height: 16, data: vec![] }` was a value every consumer had to check again.
- **native-theme**: `AnimatedIcon::new_frames`, deprecated since 0.5.7, is gone; use `AnimatedIcon::frames`.
- **native-theme**: the platform modules are private: `freedesktop`, `macos`, `sficons`, `windows`, `winicons`, `kde::colors` and `kde::fonts` had no public item and showed as empty pages on docs.rs, and `gnome`'s one public item, `GnomePortalData`, was a struct no public function takes or returns. `kde::from_kde_content_pure`, public for the crate's own KDE reader tests, is hidden from the documentation.
- **native-theme**: `IconSetChoice`, `default_icon_choice` and `list_freedesktop_themes` are no longer re-exported at the crate root; their one path is `native_theme::icons`.
- **native-theme**: `system_icon_theme()`, `detect_icon_theme()` and `system_icon_set()` are in `detect`, beside `system_is_dark()` and `detect_is_dark()` and with the `DetectionContext::icon_theme()` cache they share: `native_theme::detect::system_icon_theme` (it was `theme::system_icon_theme`, and `system_icon_set` was `theme::system_icon_set`).
- **native-theme**: the icon data types are in `icons`, beside the loaders that take and return them: `IconRole`, `IconData`, `IconProvider`, `icon_name`, `AnimatedIcon`, `FrameList`, `FramesData`, `EmptyFrameListError`, `TransformAnimation` and `TransformData` (they were in `theme`: `native_theme::theme::IconRole` is `native_theme::icons::IconRole`), with `RgbaData`; one `use native_theme::icons::{IconRole, MaterialLoader};` imports a loader and its role. `IconSet` and `IconSizes`, which a theme states, stay in `theme`.
- **native-theme**: `IconSet::SegoeIcons` is `IconSet::SegoeFluent` and `SegoeIconsLoader` is `SegoeFluentLoader`, matching the set's serialized name `segoe-fluent`, which is unchanged (TOML `icon_set = "segoe-fluent"`, the native-theme-build mapping key and its `segoe-fluent/` directory); code native-theme-build generates names `IconSet::SegoeFluent`.
- **native-theme**: `FreedesktopLoader::size` takes a `u32`, the pixel type of `rasterize_svg` and `IconData::rgba` (it was `u16`); a size above 32767 looks up the largest directory, as 32767 does (see *Fixed*).
- **native-theme**: the system loaders' `color` and `color_opt` (`FreedesktopLoader`, `SfSymbolsLoader`, `SegoeFluentLoader`) take an `Rgba` and an `Option<Rgba>`, the crate's colour type that `colorize_monochrome_svg` takes (they took `[u8; 3]`); the alpha is ignored, as the icon keeps its own coverage.
- **native-theme**: `icons::load_icon_indicator(set, theme)` takes the freedesktop theme to take the spinner from (`None` for the system's, as it used before), so a freedesktop spinner from a chosen theme needs no dispatch by hand; the other sets do not use it.
- **native-theme-build**: the generated enum implements `{crate_path}::icons::IconProvider` (it was `{crate_path}::theme::IconProvider`), the trait's path in this native-theme.
- **native-theme-build**: every field of `GenerateOutput` and `BuildErrors` is private, read through accessors: `GenerateOutput::{output_path, warnings, role_count, bundled_theme_count, svg_count, total_svg_bytes, rerun_paths, code}()` and `BuildErrors::{errors, rerun_paths}()` (`output.code` is `output.code()`; `rerun_paths` was a public field on `BuildErrors` and a method on `GenerateOutput`).
- **native-theme-build**: build errors name the file that holds the mistake: `BundledDeAware` has a `mapping_file`, `DuplicateTheme` a `file`, `ThemeOverlap` and `InvalidIdentifier` a `source_file: Option<String>` (`None` for a check on the merged configs of several files), and `MissingSvg` the `role` whose mapping names the SVG. `InvalidIconName` has a `reason`, the rule the name breaks ("names must not contain \"..\""), which its message ends with; it ended with "names must be non-empty and free of control characters" whichever rule failed, and its `offending` character is now the one that breaks that rule.

### Added

- **native-theme**: `AccessibilityPreferences::DEFAULT`, the preferences `default()` returns (text scaling 1.0, no flag set) as a `const`, usable in a `static`; the gpui connector's `Native::unscaled` uses it.
- **native-theme**: `Resolved::mode`, the `ColorMode` of the variant `Theme::resolve` actually resolved: a theme with only a light variant, resolved for `ColorMode::Dark`, reports `ColorMode::Light`.
- **native-theme**: the crate root re-exports `Theme`, `ColorMode`, `Resolved`, `ResolvedTheme`, `Rgba` and `Error` (`use native_theme::{ColorMode, Theme};`), and the prelude adds `ColorMode` and `Resolved`, the argument and the return type of `Theme::resolve`; the `theme`, `color` and `error` paths stay.
- **native-theme**: `Theme::resolve_with(mode, &ResolutionContext)` resolves with a context the caller built, with the icon-theme tiering `Theme::resolve` gives, and reads nothing from the system; `Theme::resolve` builds `ResolutionContext::from_system()` on every call, which on Linux reads configuration files and, with the `kde` or `portal` feature, runs `xrdb -query` for the font DPI (with `kde`, only when KDE's `forceFontDPI` is not set; with `portal`, not on a GNOME or Budgie desktop, whose font DPI is a fixed 96).
- **native-theme**: `AccessibilityPreferences::text_scale_factor()` (the text-scaling factor when it is finite and positive, else 1.0) and `AccessibilityPreferences::scaled(size)`. The iced, gpui and egui connectors' `scaled_text_size` call them instead of a copy of the rule each, and each connector's crate docs have a *What each flag does* table: per accessibility flag, what the connector applies and what the application has to pass on.
- **native-theme**: `detect::accessibility_preferences()`, the OS accessibility preferences from the detection cache: `SystemTheme::from_system()` stores its reader's accessibility block there, the first call runs the reader only if nothing is stored yet, and `detect::invalidate_caches()` clears it (`DetectionContext::accessibility_preferences` and `invalidate_accessibility_preferences`). `AccessibilityPreferences::from_system()` stays a fresh read and runs the whole platform reader (5.2 ms measured on one KDE machine, release build); on that machine the cached call took about 50 ns, and the first one after a system read 3.2 ms, its `gsettings` reduced-motion query, cached from then on. The connectors' preset docs and the egui and gpui showcases' preset paths use it.
- **Development**: `scripts/check_docsrs.py` documents a crate the way docs.rs will (nightly rustdoc with the crate's `[package.metadata.docs.rs]` features and targets, `DOCS_RS=1`, `--cfg docsrs`, `-D warnings`), and `scripts/check_release.sh` and the crates.io workflow run it for every workspace crate, so a docs.rs failure stops a release before the tag instead of appearing after the crates are published.
- **native-theme**: the `macos-tahoe` preset, macOS 26's Liquid Glass look: AppKit's sizes and colours as an OS probe of macOS 26.6.2 reports and draws them (`docs/platform-facts.md`'s "macOS 26:" values): 24-high controls with their text 12 in on the push button and pop-up and 4 in on the text field, control corners of 6 (the text field's 5), buttons and pop-ups in `secondarySystemFillColor`'s value, a white window, the macOS 26 system colours, a 16 check box, a 6-high slider track, an 8-high progress bar and a 54 × 24 switch. Where the model needs a number and no source gives macOS 26's, the preset carries `macos-monterey`'s and says so in the file. Its display name is "macOS Tahoe".
- **native-theme**: `SystemTheme::from_system()` picks the platform preset by the running OS version's generation: on macOS `macos-monterey` on 12 to 15 and `macos-tahoe` on 26 and later, on Windows `windows-11`. `SystemTheme::version_match` (`Option<VersionMatch>`: `Exact`, `PresetOlder`, `PresetNewer`; `None` on Linux) reports how the running version relates to the preset's generation — macOS 27, which has no preset of its own yet, is `PresetOlder` under `macos-tahoe`, and Windows 10 is `PresetNewer` under `windows-11` — and `diagnose_platform_support()` lists it as `DiagnosticEntry::OsGeneration`. `pipeline::platform_preset_name()` names the same preset.
- **native-theme**: `SystemTheme::from_system_with_preset(name)` and `from_system_with_preset_async(name)` merge the platform reader's live values over a chosen generation of the running platform (on macOS 26, `macos-monterey`'s geometry under macOS 26's colours, fonts and accessibility), reported in `version_match`.
- **native-theme**: crate documentation on docs.rs: a quick start (read the OS theme, resolve a preset, apply an overlay, hand the result to a connector, watch for changes) and a map of the modules; native-theme-derive's says it is internal to native-theme. The README has a `watch` recipe.
- **native-theme**: `SfSymbolsLoader::size(points)` and `SegoeFluentLoader::size(px)`, so a system icon can be asked for at a theme's icon size: an SF Symbol is configured at that point size, a Segoe glyph drawn from the font at that em height. Without it they stay at 24 points and 32 pixels; a size of 0, or one whose `size² × 4` bytes exceed the 64 MiB icon decode bound, loads `None`. A Segoe stock shell icon keeps the system's large icon size.
- **native-theme**: `icons::load_custom_icon(provider, set, color)` loads an `IconProvider`'s icon for a set known only at runtime, and every set honours the colour: the system sets load the icon in it (freedesktop for a symbolic icon), and a Material or Lucide icon, or the provider's own SVG, is coloured through `colorize_monochrome_svg`. The gpui and iced connectors' private copies of it go in a later release.
- **native-theme**: `SystemTheme::icon(id, mode, color)` and `SystemTheme::icon_indicator(mode)` load a role, a name or a provider's icon, and the animated loading indicator, from the theme's `icon_set`; a freedesktop icon or spinner comes from `icon_theme_for(mode)`, that variant's theme (`breeze` or `breeze-dark` under `kde-breeze`).
- **native-theme-build**: the generated enum, its variants and its `ALL` constant carry doc comments, so a crate that `include!`s the file into a public module compiles under `#![deny(missing_docs)]`.
- **native-theme**: `DetectionContext::invalidate_font_dpi()` clears the cached font DPI alone, as the other `invalidate_*` methods clear their field; `invalidate_all()` and `detect::invalidate_caches()` clear it too.

### Changed

- **native-theme**: validation errors name the TOML key a theme author writes: `button.min_height_px`, `button.border.padding_left_px`, `defaults.icon_sizes.toolbar_px` (they named the model fields, `button.min_height`, `button.border.padding_left`, `defaults.icon_sizes.toolbar`), and an out-of-range font size the key in the unit its value carries, `size_pt` or `size_px`, under the table the value lands in (a `button.font` that inherits a `defaults.font` in points is `button.font.size_pt`). Errors from `Theme::resolve`, `Theme::resolve_with`, `SystemTheme::from_system()` and `SystemTheme::with_overlay` start each path with the variant that failed, `light.` or `dark.`, except `layout.*` and `context.font_dpi`, which no variant holds.
- **native-theme**: `SystemTheme::from_system()` no longer detects the font DPI a second time after the platform reader supplied it.
- **native-theme**: `SystemTheme::from_system()` and `AccessibilityPreferences::from_system()` block on `pollster` on macOS and Windows as they do on Linux, so `pollster` is a dependency on every platform; the single poll that answered `PlatformUnsupported` ("unexpected async suspension") if a reader ever awaited is gone.
- **native-theme-egui**: `from_preset` resolves both variants on one `ResolutionContext`.
- **native-theme**: the Windows reader no longer states WinUI 3's design sizes and radii; they come from the windows-11 preset alone, so on Windows the splitter divider is 1 px, the preset's and docs/platform-facts.md §2.17's, instead of 4.
- **native-theme**: outside high contrast the Windows reader writes only what the user sets and the windows-11 preset cannot know: light or dark, the accent colours, the text scale, the non-client fonts and the accessibility flags; the rest comes from the preset, which states what WinUI 3 draws. Live Windows themes no longer take classic Win32 colours from `GetSysColor` (the button's, menu's, tooltip's, text field's and list selection's, the placeholder's `COLOR_GRAYTEXT`, the title bar's text and inactive fill), which follow neither dark mode nor what WinUI 3 draws, nor the DWM colorization as the title bar's fill, `UISettings`' black or white foreground and background as the text and window colours, the accent as the focus ring and the selection, the foreground as the accent button's text (black on the dark accent fill in light mode), the window background as the surface, the midpoint of foreground and background as the disabled text, or `SM_CXFOCUSBORDER` (1) as the focus ring's width (WinUI 3 draws 2); a classic colour set in the registry no longer shows outside high contrast. From the accent shades it reads, it writes the fills of the accent and checked buttons as WinUI 3's `AccentFillColorDefault` (AccentDark1, dark AccentLight2; the dark fill was AccentLight1), the link colour as `AccentTextFillColorPrimary` (AccentDark2, dark AccentLight3) and the informational colour as `SystemFillColorAttention` (the accent, dark AccentLight2). Under high contrast the reader maps WinUI 3's HighContrast theme dictionaries (microsoft-ui-xaml 8463f45) to the system colours: the text, focus ring, informational colour, tooltip text and placeholder `COLOR_WINDOWTEXT`; the window, surface, menu and tooltip `COLOR_WINDOW`; buttons and text fields `COLOR_BTNFACE` with `COLOR_BTNTEXT` (the menu's text too); the accent `COLOR_HIGHLIGHT` with its text `COLOR_HIGHLIGHTTEXT`, so the fills and text that inherit them follow; the selection, the list's selected row and the accent and checked buttons `COLOR_HIGHLIGHT` with `COLOR_HIGHLIGHTTEXT`; links `COLOR_HOTLIGHT`; disabled text `COLOR_GRAYTEXT`; and the progress bar gets its 1 px border.
- **native-theme**: the `kde-breeze` and `adwaita` presets name the style version their values were checked against: Breeze v6.7.5 (Plasma 6.7, KDE Frameworks 6.31.0) and libadwaita 1.10.0 (GNOME 51).
- **native-theme**: the macOS reader no longer states AppKit's design sizes; they come from the macOS preset alone, as the Windows reader's do; the scroll bar's groove width, which it stated as the legacy style's 15 under either style, is the width AppKit gives the user's scroller style (under Fixed). It no longer reads `NSColor.shadowColor`, an opaque black, as the drop-shadow colour, so the preset's `NSShadow` default applies; it reads a selection's text colour from `alternateSelectedControlTextColor` (white), as `docs/platform-facts.md` names it, where it read `selectedTextColor` (black in light mode), which now gives the selected text's colour beside `selectedTextBackgroundColor`; and the text field's selection takes those text-selection colours instead of `unemphasizedSelectedContentBackgroundColor`.
- **native-theme**: the first `Theme::preset(name)` in a process parses that preset's TOML alone, not all 17 bundled ones, and each preset is parsed on its own first use: a cold `Theme::preset("kde-breeze")` took a median 0.84 ms instead of 6.3 ms (21 fresh processes each, release build, on one Linux machine). `SystemTheme::from_system()` makes that call for its platform preset.
- **native-theme**: a Segoe glyph is drawn from the font created at its size as the em height (CreateFontW with a negative height); the 32 pixels it was drawn at were the font's cell height, which includes the internal leading.
- **native-theme**: the font DPI that `ResolutionContext::from_system()` detects, which `Theme::resolve` and `ThemeMode::resolve_system` use, and the KDE reader's DPI after its own `forceFontDPI` are one chain, detected once and cached until `detect::invalidate_caches()`: `Theme::resolve` no longer runs `xrdb -query` on every call, so resolving `kde-breeze` took a median 36 µs instead of 3.5 ms, and a warm `SystemTheme::from_system()` 1.25 ms instead of 5.3 ms (three release runs each, on one KDE Wayland machine).
- **native-theme**: a freedesktop icon name a theme has only as an `.xpm` file, which the lookup never returns, is `None` at once: it passed the index of the theme's file names and ran freedesktop-icons' whole fallback search on every load (2.3–2.7 ms per load, a release build; about 310 ns now).

### Fixed

- **native-theme**: a `[layout]` value that is negative, NaN or infinite (`widget_gap_px`, `container_margin_px`, `window_margin_px`, `section_gap_px`) fails `Theme::resolve`, `SystemTheme::from_system()` and `SystemTheme::with_overlay` with `Error::ResolutionInvalid` naming its key, `layout.widget_gap_px`; it reached `SystemTheme::layout` unchecked, although the four fields declare a non-negative range.
- **native-theme**: validation range-checks the card, so a negative, NaN or infinite `card.border` padding side fails with `ResolutionInvalid` (it reached `ResolvedTheme::card`), and a widget border's `corner_radius_px` and `line_width_px` must be finite and non-negative, as `defaults.border`'s already were (`[light.button.border] corner_radius_px = -4.0` resolved).
- **native-theme**: a NaN or infinite `defaults.focus_ring_offset_px` fails validation; a negative offset, an inset ring, stays valid.
- **native-theme**: `Theme::from_toml` parses a theme file with no `name`, such as an overlay that states only `[light.defaults] accent_color`; it failed with `missing field "name"`, although `Theme::merge` keeps the base's name. The name is then empty.
- **native-theme**: a `link.font.color` a preset, a TOML overlay or a platform reader states reaches the resolved theme; resolving replaced it with `defaults.link_color`, which now fills it only when it is unset. The KDE reader no longer states `link.font.color` beside `defaults.link_color` (both were `[Colors:View] ForegroundLink`), so on KDE an overlay's `defaults.link_color` still recolours link text.
- **native-theme**: a `text_scale` entry may state its size and its line height in different units (`size_pt` with `line_height_px`), each converted to pixels on its own; `Theme::from_toml` rejected the pair, so a theme merged from an overlay's `[light.text_scale.caption] size_px = 12.0` onto `ios-13`'s `size_pt` and `line_height_pt` could be saved with `to_toml` but not read back.
- **native-theme**: on a Linux desktop without a reader (KDE without the `kde` feature, GNOME without `portal`, XFCE, Sway, an unrecognised desktop and the like), `SystemTheme::from_system()` returns the desktop's platform preset whole: on KDE without `kde` it was a mix, the active variant Adwaita's colours, the inactive one Breeze's, named "Adwaita" with the `breeze` icon theme; it is now `kde-breeze` in both variants, named "KDE Breeze". Its `mode` is read fresh from the OS (`detect::detect_is_dark`), as the readers read it; it came from the cached `detect::system_is_dark`, so a second `from_system` after the user switched to dark mode returned the old mode unless `detect::invalidate_caches()` ran first.
- **native-theme**: `pipeline::platform_preset_name()` names `kde-breeze` on an unrecognised Linux desktop with a `kdeglobals` file (with the `kde` feature) where no portal backend answers, the preset `SystemTheme::from_system()` uses there; it named `adwaita`. `from_system` asks the portal backend first and follows its answer, which `platform_preset_name()` cannot ask, so where a backend answers the two can still differ.
- **native-theme**: `Error::PlatformUnsupported` from `SystemTheme::from_system()` and `watch::on_theme_change` names the platform (`std::env::consts::OS`, e.g. "freebsd"), as the field documents; it was "unsupported".
- **native-theme**: `SystemTheme::with_overlay` overlays stack: `sys.with_overlay(&a)?.with_overlay(&b)?` keeps what `a` states unless `b` states it too; the second call replayed the platform reader and the preset alone, and dropped `a`.
- **native-theme**: `SystemTheme::with_overlay` applies the overlay's `icon_theme`, `icon_set` and `[layout]`, through the same icon-theme tiers as `SystemTheme::from_system()` (a variant's `defaults.icon_theme`, then the theme's, then the detected one); it kept the base theme's `icon_theme`, `icon_set` and `layout` whatever the overlay stated.
- **native-theme**: the `windows-11` preset states what WinUI 3 draws (`docs/platform-facts.md`, checked against microsoft-ui-xaml at 8463f45 and an OS probe of Windows 11 25H2): text on a status fill `#000000e4` in dark mode and white on the light warning fill; the focus ring in `FocusStrokeColorOuter` (`#000000e4`, dark `#ffffff`) instead of the accent, 1 px outside the control; toolbar and menu icons 16 instead of 20; the button's hover fill `ControlFillColorSecondary` as it draws over the window (`#f6f6f6`, dark `#323232`) and its pressed fill `ControlFillColorTertiary` the same way (`#f5f5f5`, dark `#272727`); the tooltip text 12 px (9 pt); the slider's track and thumb, the progress bar's track and its 1.5 radius; tabs at least 100 wide; the selected sidebar and list row in `SubtleFillColorSecondary` with primary text instead of the accent; the popover's acrylic fallback (`#f9f9f9`, dark `#2c2c2c`); the dark switch's knob black; the combo box's fill `ControlFillColorDefault`.
- **native-theme**: the `windows-11` preset states WinUI 3's translucent brushes with their alpha, as the resources give them: text `TextFillColorPrimary` (`#000000e4`), muted and placeholder text `TextFillColorSecondary` (`#0000009e`, dark `#ffffffc5`), disabled text `TextFillColorDisabled` (`#0000005c`, dark `#ffffff5d`), the surface and button fills (`#ffffffb3`; dark `#ffffff0d`, `#ffffff0f`), the unchecked check box's and switch's fill (`#00000006`, dark `#00000019`) and the check box's border (`#00000072`, dark `#ffffff8b`). Links take the accent shades WinUI draws them in (`#003e92`, dark `#99ebff`, and the next shades on hover and press), dark info `#4cc2ff`; the tooltip is light, the popover's acrylic fallback `#f9f9f9` with primary text (it was `#2c2c2c` with white text), and the dark one that fallback, `#2c2c2c` (it was `#3a3a3a`); the docked sidebar and the scroll bar's track are transparent, the dialog `SolidBackgroundFillColorBase`; menu, list and sidebar hovers are `SubtleFillColorSecondary` (`#00000009`), and the link's hover is transparent (an inline `Hyperlink` has no background); a pressed button's text is `TextFillColorSecondary`; the separator takes the border colour, and the expander's chevron the text colour.
- **native-theme**: the `windows-11` preset's border, and the separator with it, is `CardStrokeColorDefault` as the resource gives it (`#0000000f`, dark `#00000019`; it was the composite `#e5e5e5` and an unsourced dark `#454545`), and `defaults.border.opacity` that brush's alpha (0.059, dark 0.098; was 0.14); the input's fill is `TextControlBackground` = `ControlFillColorDefault` (`#ffffffb3`, dark `#ffffff0f`; was `#ffffff` and `#2d2d2d`) and its hover border its rest border, as in WinUI; the combo box's hover and the off switch's hover are WinUI's replacing fills as they draw over the window (`#f6f6f6` and `#e5e5e5`, dark `#323232` and `#2a2a2a`), the off switch's disabled track `ControlAltFillColorDisabled` (transparent) and the slider's disabled track `ControlStrongFillColorDisabled` (`#00000051`, dark `#ffffff3f`).
- **native-theme**: on macOS `SystemTheme::from_system()` reports the active appearance in `SystemTheme::mode` (it was always `Light`), so an application on a Mac in dark mode starts dark in every connector.
- **native-theme**: the documentation of the `windows` module, which docs.rs builds for its Windows target, no longer links to two private items, and the `fonts` and `watch` modules are documented in one place (their own file), so their links resolve in the same scope on stable and on the nightly rustdoc docs.rs uses, which reported their explicit targets as redundant; the `watch` summary is no longer shown twice.
- **READMEs**: the theme-switching GIFs are assembled at the captured window's size, as the screenshots are, instead of being scaled to 600 px wide, and the spinner GIFs are shown at their own 200×120 instead of 80 px high.
- **native-theme**: the `ios-13` preset takes what iOS 13-18 draw where a source gives it: the section heading (17 pt Semibold), dialog title (28 pt) and every text-scale line height from Apple's Dynamic Type table; muted text in `secondaryLabel` and disabled text in `tertiaryLabel` (a disabled select's too); a button's fill, its blue text (pressed and hovered too) and the switch's green on track and grey off track as WebKit draws them on iOS; and the dark text field's fill in `systemBackground`. Each value with a source cites it in the file, whose header states the iOS versions it covers (13-18) and was verified on (18).
- **native-theme**: the `ios-13` preset states the rest of what its sources give for iOS 13-18: a disabled switch's tracks keep 0.4 of their alpha, as WebKit draws them (they were the tracks at half alpha, unsourced); the slider's and progress bar's tracks are `opaqueFill` (`#e4e4e5`, dark `#2b2b2e`) and a text field's placeholder `placeholderText` (they took the muted text colour); a drop-down's text is systemBlue on `opaqueSecondaryFill`, as WebKit's iOS select (it was black or white on the window colour); the line height is SF Pro's 1.19 (it was 1.2); and text fields, check boxes, progress bars, drop-downs, segmented controls and expanders cast no shadow (they inherited one).
- **native-theme**: the `macos-monterey` preset's split-view divider is AppKit's default, the thick one: 9 wide (it was 6, a GNUstep value) and clear, as `NSSplitView.dividerColor` is for that style (it took the border colour).
- **native-theme**: the `macos-monterey` preset's push button is 20 high, AppKit's `intrinsicContentSize` on macOS 14 (it was 22), with its title 1 below the bezel's top and 3 above its bottom, measured from the bezel's outer edge, where AppKit's title rect puts it (it was 3 and 3).
- **native-theme**: the `macos-monterey` preset's slider knob is 20 across, as macOS 14 draws it (it was 21).
- **native-theme**: the `macos-monterey` preset's drop-down places its title 1 below the bezel's top and 3 above its bottom, where AppKit's pop-up button puts it (it was 3 and 3).
- **native-theme**: the `macos-monterey` preset's colours are the ones macOS 14 and 15 report (an OS probe of 14.8.9 and 15.7.9) for the AppKit colours `docs/platform-facts.md` names, with their alpha where AppKit gives one: window `#ececec` (dark `#323232`), text `labelColor` `#000000d8` (dark `#ffffffd8`), muted and disabled text, borders and separators (`separatorColor`), the focus ring, links, the selection (`#0064e1`, dark `#0059d1`) and text selection, and the button, text-field and list colours, among them the placeholder, column header and grid. The success green is macOS 14's `#28cd41` (dark `#32d74b`; it was the macOS 26 green), the dark accent is `#007aff`, as `controlAccentColor` is in both appearances, and text beside a status colour is `labelColor` (it was white). The drop shadow is `NSShadow`'s default colour, black at an alpha of 1/3 (`#00000055`; it was `#00000018`, dark `#00000040`).
- **native-theme**: the `macos-monterey` preset's `disabled_opacity` is 1.0 (it was 0.3, which faded the already dimmed disabled colours a second time) and `border.opacity` 0.098, `separatorColor`'s alpha; tooltip text is 11 pt, as `toolTipsFontOfSize:` returns, menu icons are 16 (they inherited the 24 toolbar size), the overlay scroll bar's track is transparent, and the text field, check box, progress bar, drop-down, segmented control and expander cast no shadow, and the spinner is drawn in black (dark white) at the leading fin's alpha, 82/255, instead of the accent colour.
- **native-theme**: `kde-breeze` light draws text on a selection and on the accent in Breeze's `#ffffff` (`[Colors:Selection] ForegroundNormal`; `#fcfcfc` was Breeze's before Plasma 5.21), and danger, success and info text in the window text `#232629`, as the KDE reader reads them on a Breeze Light desktop. `adwaita` draws its lines in libadwaita 1.10's `$border_color` over the window, `#dcdcde` light and `#434347` dark (were `#d5d5d5` and `#4a4a4e`), and a popover's outline in libadwaita 1.10's `RGB(0 0 0 / 5%)` instead of the line colour.
- **native-theme**: `kde-breeze` states the inactive title bar's text in Breeze's `[WM] inactiveForeground` (`#707d8a`, dark `#a1a9b1`; it was the window text and an unsourced `#a1a4a6`), alternate list rows in `[Colors:View] BackgroundAlternate` (`#f7f7f7`, dark `#1d1f22`; were `#f5f6f7` and `#2a2e32`) and the light pressed button's text in `#ffffff`, `[Colors:Selection] ForegroundNormal`; its popover takes the window colour and its separator the border colour, as `docs/platform-facts.md` gives them (they were `#ffffff` / `#31363b` and `#c4c8c6` / `#3e4550`). `adwaita` uses overlay scroll bars, GNOME's `overlay-scrolling` default (it stated persistent ones), and its dark surface is libadwaita 1.10's `view_bg_color` `#1d1d20` (it was `#303034`), which the dark list headers and slider thumb inherit.
- **native-theme**: `adwaita`'s text caret is the entry's text colour, libadwaita 1.10's `caret-color: currentColor` (`#2e3436`, dark `#ffffff`; were `#3584e4` and `#78aeed`). Its dark title bar is `headerbar_bg_color` `#2e2e32` and its dark cards `card_bg_color`, white at 8 % (`#ffffff14`); an inactive title bar is `headerbar_backdrop_color`, the window colour (`#fafafb`, dark `#222226`; light was the title bar's `#ffffff`, dark `#303034`).
- **native-theme**: on GNOME the text-scaling factor is applied once on Wayland; X11 and Xwayland clients also get it through the toolkit's `Xft.dpi` scale factor (winit, gpui), which the connectors do not yet leave out. The GNOME reader, and `ResolutionContext::from_system()` on a GNOME or Budgie desktop, took the font DPI from `Xft.dpi`, which gnome-settings-daemon sets to 96 × `text-scaling-factor` × the X11 UI scale, while the connectors also multiply text sizes by the factor, so at 1.5 text was drawn 2.25 times its size. GNOME's font DPI is 96 now; the factor stays in `AccessibilityPreferences`.
- **native-theme**: the `macos-monterey` preset takes what an OS probe of macOS 14.8.9 and 15.7.9 reports and draws (`docs/platform-facts.md`): the text field is 21 high with its text 2 in from each edge (it was 22, with 4 and 3) and its caret `textInsertionPointColor`, `#007aff` (it took the text colour); a check box's label sits 6 from its box (it was 4); the slider's track is 4 high (it was 5) and its dark knob white at alpha 128/255 (it took the dark surface colour); the switch's knob is 20 across (it was 18), its off track black (dark: white) at alpha 26/255 (it was `#d5d5d5`, dark `#3d3d3d`) and its dark knob white at alpha 191/255; the drop-down is 20 high (it was 21) with its title 8 in from the left edge, in `controlColor` with `controlTextColor` text (it took the window's background and text colours); the segmented control is 22 high (it was 24); and a list's rows are `controlBackgroundColor`, `#ffffff` (dark `#1e1e1e`), under the alternate rows' `#f4f5f5` (they took the window background, so the alternate rows were the lighter ones). The dark slider knob keeps its colour on hover and when disabled (it turned an unsourced `#3a3a3c` and `#2d2d2d80`, darker than the window).
- **native-theme**: serialising a widget's theme struct (`ButtonTheme`, `LayoutTheme` and the others), and a `Theme` with it, to a format other than TOML, such as JSON, leaves out the unset fields, as TOML output did; it wrote `null` for each, because their `#[serde_with::skip_serializing_none]` came after `#[derive(Serialize)]`, where it has no effect. TOML output is unchanged.
- **native-theme**: a colour string with a non-ASCII character, such as `"#é0"` in a theme file, fails with `invalid hex color "#é0": not ASCII hex digits`, quoting the string; it was reported as a wrong digit (`invalid red hex digit 'Ã'`, the first byte of `é`) or, for `"#é"`, as a length counted in bytes (`invalid hex color length 2`).
- **native-theme-derive**: a malformed `#[theme(...)]` or `#[theme_layer(...)]` attribute is a compile error at the attribute, never a proc-macro panic: a `min_max_pair`, `resolved_name` or `inherit_from` segment that is not an identifier, an `inherit_from` that does not start with `defaults.`, `category` beside `nested` or `resolved_type`, a second `check` or range, a range whose minimum exceeds its maximum and an f32 range bound that is not finite; the later attribute silently won, or the range rejected every value. `#[theme_layer(skip_inventory)]` also skips the border and font inheritance registrations.
- **native-theme**: `Error::NoVariant` displays "theme defines neither a light nor a dark variant (requested Dark)", the only case it is returned for; it said "theme has no variant for Dark".
- **native-theme**: a `ResolutionContext` whose `font_dpi` is not finite and positive (0, negative, NaN, infinite) fails `ThemeMode::into_resolved`, `Theme::resolve_with` and the OS-theme pipeline once, with `ResolutionInvalid` on the path `context.font_dpi`, before any point size is converted; it was reported as every point-sized font of the theme (`defaults.font.size` and the widgets' fonts), and a theme in pixels passed with it.
- **native-theme**: on Windows, dropping the `ThemeSubscription` of `watch::on_theme_change` right after it returned no longer hangs: the watcher thread creates its message queue before it reports that it started, so the `WM_QUIT` the drop posts arrives; it was lost when it came first, and the drop waited for the thread forever. `on_theme_change` returns `Error::ReaderFailed` (reader `windows-watcher`) when COM, `UISettings` or the `ColorValuesChanged` subscription fails to set up; it returned `Ok` with a watcher that never fired. The watcher releases its WinRT objects before `CoUninitialize`.
- **native-theme**: on macOS, `watch::on_theme_change` delivers appearance changes: its watcher thread ran a run loop of its own, which a distributed notification never reaches (Apple delivers them through the process's main run loop), so the callback never fired, and dropping the `ThemeSubscription` right after it returned could hang forever. The observer now has no thread: the change arrives through the application's main run loop, which must be running in a common mode (every GUI toolkit's event loop is), the callback runs on a private serial queue, never on the main thread and never twice at once, and dropping the subscription removes the observer and returns at once.
- **native-theme**: a malformed freedesktop icon no longer panics or aborts the process: a `process-working` sprite sheet whose viewBox is not finite or narrower than 1 (`viewBox="0 0 0 0.005"` panicked with "capacity overflow") is a single-frame spinner, and one whose frames would take more than 64 MiB (`viewBox="0 0 1 1000000000"`, a billion copies of the file) too; a PNG whose header declares a decoded or RGBA image over 64 MiB (a few bytes claiming 1 000 000 × 1 000 000 pixels aborted on a 4 TB allocation) is `None`, and a decode buffer that cannot be allocated is `None` instead of an abort. 64 MiB is the png crate's default memory limit, a 4096 × 4096 RGBA image.
- **native-theme-build**: an SVG that several roles map to (`play` and `resume` both `"play_arrow"`) counts once in `GenerateOutput::svg_count()` and `total_svg_bytes()`, and is listed once in `rerun_paths()`; it counted once per role.
- **native-theme-build**: `GenerateOutput::emit_cargo_directives()` prints the `cargo::warning` lines before it writes the generated file, as its documentation says, so a failed write no longer loses them.
- **native-theme-build**: a DE-aware mapping takes the keys `hyprland`, `sway`, `river`, `niri`, `wayfire` and `cosmic` (`LinuxDesktop::CosmicDe`), the desktops native-theme detects beside the seven it took; such a key was warned about as "will never be used at runtime" and its arm dropped.
- **native-theme-build**: the generated file has no `extern crate native_theme;`, so two generated files `include!`d into one module compile (the second was E0259, "the name `native_theme` is defined multiple times"); a renamed dependency is named with `IconGenerator::crate_path`.
- **native-theme-build**: `icon_name` and `icon_svg` return `::core::option::Option`, so an enum named `option` (`pub enum Option`) compiles, and a role or enum name whose PascalCase has a character other than an ASCII letter, digit or `_` (`"x²"`, which heck keeps) is an `InvalidIdentifier` rather than generated code that does not compile.
- **native-theme-build**: `.derive("core::hash::Hash")` or `.derive("std::fmt::Debug")` is deduplicated against the base derives by its last path segment, as the documentation says; the enum derived the trait twice (E0119).
- **native-theme-build**: the enum name is checked once, the one the generated enum takes (the `.enum_name()` override, else the first file's `name`), so `.enum_name("AppIcon")` builds a file whose unused `name = "3d-icons"` failed the build, as did an invalid `name` in a second file that only drew a warning.
- **native-theme-build**: one mistake gives one error: an unknown theme (`bundled-themes = ["materal"]`) gave the error twice, the second time without its file, and a "theme directory not found" error; an invalid role and a theme in both lists of one file were reported twice, and the overlapping theme's mapping errors twice too. The checks of the merged configs report only what crosses files: a theme one file bundles and another loads from the system, a PascalCase collision between roles of two files. `DuplicateTheme` reads "appears more than once in bundled-themes of icons.toml" (it had a double "in").
- **native-theme-build**: a TOML error in a master or mapping file is one `cargo::error=` line that carries its cause, "failed to parse icons.toml: line 3, column 9: invalid type: string \"a\", expected a sequence"; the directive held only toml's header line ("TOML parse error at line 3, column 9"), and the cause landed on plain stdout lines cargo does not show as the error.
- **native-theme**: the KDE reader reads `kdeglobals` as KConfig cascades it: `kdeglobals` in each `$XDG_CONFIG_DIRS` entry (`/etc/xdg` by default), then `kdedefaults/kdeglobals`, then the user's file, later files overriding earlier keys, so a colour scheme, icon theme or other key Plasma 6 writes only to `kdedefaults` reaches the theme (`SystemTheme::name` was "KDE" on such a desktop). A missing `kdeglobals`, a byte that is not UTF-8 and a line KConfig skips (a `[Group` header without `]`, an entry with an empty key) no longer fail `SystemTheme::from_system()`; a `#` or `;` inside a value (`ColorScheme=My#Scheme`) is kept, as KConfig has only whole-line `#` comments. `detect::system_is_dark()` reads the same cascade, on KDE and in its fallback on other desktops, and goes on to its next source (gsettings, then `gtk-3.0/settings.ini`) when no kdeglobals file states a `[Colors:Window] BackgroundNormal` it can parse.
- **native-theme**: a KDE font whose Qt 6 weight lies outside CSS 100–900 (`font=Noto Sans,10,-1,5,950,…`) gets its weight clamped to that range, after Qt's own clamp to 1–1000 (so 0 is 100, and 70000, which the cast wrapped to 4464, is 900), and a weight field that is not an integer leaves the weight to the preset while the family and size stay; such a font failed `SystemTheme::from_system()` with `ResolutionInvalid`. A Qt 5 weight converts as Qt's own `qt_legacyToOpenTypeWeight` does, to the nearest Qt 5 constant: Black (87) is 900 (it was 800), ExtraBold (81) 800 (it was 700) and ExtraLight (12) 200 (it was 100), and the values between constants move with them. A font saved with a pixel size (`Noto Sans,-1,13,…`) is read as 13 px, where the preset's font replaced it, a pixel-sized body font's headings in pixels too, and Qt's style field gives the font's style, italic or oblique, the title-bar font's too.
- **native-theme**: KDE's `forceFontDPI` is read as Plasma declares it, a whole number with 0 for unset, and on an X11 session only, where Plasma 6.7 reads it: `forceFontDPI=inf` failed `SystemTheme::from_system()`, and a key left from an X11 session scaled the fonts of a Wayland session, which Plasma does not.
- **native-theme**: the KDE reader's icon sizes are the icon theme's `[Icon Theme]` `SmallDefault`, `MainToolbarDefault`, `DesktopDefault`, `DialogDefault` and `PanelDefault`, the sizes KIconLoader uses, with KIconLoader's own fallback for a key the theme omits (Desktop 32, MainToolbar 22, Small 16, Panel 48, Dialog 32): on Breeze it reported small 12 and large 32, the smallest directories the theme lists, where Breeze states 16 and 48. The theme's `index.theme` is looked up where KIconLoader looks, `$XDG_DATA_HOME/icons` (any absolute `$XDG_DATA_HOME`) before each `$XDG_DATA_DIRS` entry's `icons`, so a user's copy of a theme wins over the system's, then `~/.icons`; the data dirs are searched without a home directory too, and a theme name that is not one directory (`../x`) is not looked up.
- **native-theme**: the GNOME reader parses `font-name`, `monospace-font-name` and `titlebar-font` as Pango parses a font description: style, weight, stretch and variant words after the family (`Noto Sans Bold Italic 11`, any case, with or without the hyphen) leave the family and give the weight and style (italic or oblique), Pango's Semi-Light (350), Book (380) and Ultra-Black (900) included; a `px` size (`Cantarell 15px`) is read in pixels; trailing `@variations` and `#features` are skipped; of a family list (`Cantarell, Sans 11`) the first family is taken. Such strings kept the style words in the family or dropped the font. A size of `NaN` or `inf` drops only that font, which keeps the preset's; it failed `SystemTheme::from_system()` with `ResolutionInvalid`.
- **native-theme**: the xdg-desktop-portal reads have a deadline: the GNOME reader's portal calls, the KDE-with-portal reader's accent read and the portal-backend probe on an unrecognised desktop each finish within 2 s, the bound each `gsettings` read already has, and proceed as if no portal answered when the time is up. A hung or slow-starting xdg-desktop-portal blocked `SystemTheme::from_system()` without limit, as zbus sets no method timeout. On a desktop `XDG_CURRENT_DESKTOP` does not name, a running KDE or GNOME portal backend picks the reader before an installed one, and when both a KDE and a GNOME backend are running (or, none running, both installed) the `kdeglobals` check, or Adwaita, decides; it took whichever installed backend the bus listed first.
- **native-theme**: on GNOME, when xdg-desktop-portal gives no colour scheme (no portal, or one that does not answer or lacks the setting), the GNOME reader takes dark or light from `gsettings` `color-scheme`, through the same function `detect::system_is_dark` uses, so the two agree; it returned the light theme while `system_is_dark` said dark. Its `gsettings` reads finish within 2 s together (each had 2 s, up to 18 s for nine keys), and `icon-theme` is read once per theme read, not twice.
- **native-theme**: on a Linux desktop `XDG_CURRENT_DESKTOP` does not name (unset, as for an app started from a systemd unit or over ssh, or a value such as `i3`), the theme watcher, the icon-theme detection, the font DPI and `detect::system_is_dark()` follow the desktop `SystemTheme::from_system()` reads by: its portal backend's, else KDE where `kdeglobals` exists. `watch::on_theme_change` returned `WatchUnavailable` there although `from_system` read KDE or GNOME; `detect::system_icon_theme()` took KDE's icon theme before GNOME's even where the GNOME reader ran, so it, an unthemed `FreedesktopLoader` and the `IconSetChoice::System` label could name another theme than the GNOME reader's; and `ResolutionContext::from_system()` (the preset path) took the font DPI from `Xft.dpi` where the GNOME reader gives 96, so text was scaled twice. On KDE reached that way `system_is_dark()` reads `kdeglobals` first, as the KDE reader does, so it agrees with `SystemTheme::mode`. These synchronous functions (`on_theme_change`, the icon theme, the font DPI, `system_is_dark`, `pipeline::platform_preset_name()`) cannot ask the portal and use the desktop the last system read refined; before one, only the `kdeglobals` check, and a system read whose portal answer names another desktop clears the font DPI, icon theme and dark mode cached from it. On KDE (with the `kde` feature) the icon theme `detect` reports is the KDE reader's, read from the same KConfig cascade (`/etc/xdg/kdeglobals` included).
- **native-theme**: a detection cache (`system_is_dark()`, `prefers_reduced_motion()`, `accessibility_preferences()`, `system_icon_theme()`, the Linux desktop, the font DPI) no longer keeps a value read before an invalidation that lands while it is read: a `SystemTheme::from_system()` or a cached read that began before the user changed a setting and ended after a watcher's `detect::invalidate_caches()` cached the old value, which every later read returned until the next invalidation. Each field counts its invalidations, and a read stores its value only if none landed since it began.
- **native-theme**: every command the detection runs is bounded by the same 2 s: the icon-theme detection's `gsettings` and `xfconf-query`, `fc-match` for `fonts::substitute_family`, macOS `defaults` without the `macos` feature and `gsettings --version` in `diagnose_platform_support()` waited as long as the command ran (`xrdb` and `xrandr` were bounded already); an icon-theme error names a command that timed out ("timed out after 2s"). A command's output is read while it runs, so an `xrdb -query` that prints more than a pipe holds (64 KiB on Linux) is no longer killed at 2 s and read as no `Xft.dpi`, and a process a command leaves running with its output no longer holds the call past 2 s.
- **native-theme**: the user's config directory is `$XDG_CONFIG_HOME` only when it is an absolute path, else `$HOME/.config`, as the XDG Base Directory Specification says, for every file read from it (`gtk-3.0/settings.ini` for dark mode, `kdeglobals`, `kcmfontsrc`, `lxqt/lxqt.conf`) and for the KDE watcher: an empty `XDG_CONFIG_HOME` made the dark-mode fallback read `/gtk-3.0/settings.ini`, and a relative one was read from the working directory. Without either, no user file is read: the KDE reader no longer took `/etc/xdg/kdeglobals` as the user's file and `/etc/xdg/kdedefaults` as its defaults, the KDE watcher fails with `ReaderFailed`, and `diagnose_platform_support()` reports `HOME` as not set.
- **native-theme**: reduced motion has one read per platform, the one its theme reader makes: `detect::prefers_reduced_motion()` and `detect_reduced_motion()` read `kdeglobals`' `[KDE] AnimationDurationFactor` on KDE (with the `kde` feature), the portal's `reduced-motion` then `gsettings` `enable-animations` on GNOME and Budgie, `NSWorkspace` on macOS and `SPI_GETCLIENTAREAANIMATION` on Windows. They asked GNOME's `enable-animations` on every Linux desktop, which on KDE can be absent or out of step with Plasma's animation speed, and `UISettings.AnimationsEnabled` on Windows. `AccessibilityPreferences::from_system()` and `detect::accessibility_preferences()` no longer add that second answer to the reader's, so they agree with `SystemTheme::from_system().accessibility`; where no reader runs or it fails, `AccessibilityPreferences::from_system()` takes `reduce_motion` from that read, as `SystemTheme::from_system()` now does on a Linux desktop without a reader (it was `false` there), and a system read whose portal answer names another desktop clears the cached reduced motion as it clears dark mode.
- **native-theme**: the theme watchers: on KDE and GNOME, `watch::on_theme_change` returns `Error::ReaderFailed` (reader `kde-watcher`, `gnome-watcher`) when the inotify watch of the config directory, or the session bus connection, the portal proxy or the `SettingChanged` subscription, fails to set up; it returned `Ok` with a watcher that never fired. A callback that panics loses that one event and the watcher keeps running, on every platform: on Windows the panic aborted the process, on Linux it ended the watcher thread silently, and on macOS it unwound through Foundation. The KDE watcher fires once per burst of writes, 300 ms after the last, so the consumer re-reads after KDE's last write; it fired on the first and dropped the rest. Dropping a KDE subscription while files in the config directory keep changing returns within 200 ms; it waited for a 200 ms gap with no event. On macOS, dropping the subscription waits for a callback already queued, so none runs after the drop returns (except for a drop from inside the callback). A callback that drops its own subscription no longer panics in `JoinHandle::join` (a thread joining itself, EDEADLK): the drop detaches the watcher thread, which exits once the callback returns.
- **native-theme**: `watch::on_theme_change` fires on the changes of what the platform's reader reads; it missed several. On GNOME and Budgie it fires on the portal's `SettingChanged` for the `org.gnome.desktop.interface` keys the reader reads (fonts, text scale, animations, overlay scrolling, icon theme, colour scheme), the title bar font in `org.gnome.desktop.wm.preferences` and high contrast in `org.gnome.desktop.a11y.interface`, which the portal's GNOME and GTK backends forward under their schema names; it listened to `org.freedesktop.appearance` alone (of which it fires on the four settings the reader reads). On KDE it watches `kdedefaults/kdeglobals`, which the reader reads beneath the user's `kdeglobals`, from the moment Plasma creates the directory. On macOS it observes `NSSystemColorsDidChangeNotification` (the accent and highlight colours), `NSPreferredScrollerStyleDidChangeNotification` (overlay scroll bars) and, on `NSWorkspace`'s center, `NSWorkspaceAccessibilityDisplayOptionsDidChangeNotification` (reduce motion, increase contrast, reduce transparency), and the appearance notification is delivered immediately while the application is inactive, which `NSApplication` suspended until it was active again. On Windows it subscribes to `UISettings::TextScaleFactorChanged`, `AccessibilitySettings::HighContrastChanged` (high contrast) and, from Windows 10 2004, `UISettings::AnimationsEnabledChanged` (client-area animation, which `UISettings.AnimationsEnabled` read the same as on the three Windows probe images) too, and the callback runs on the watcher thread; the non-client fonts and the high-contrast `GetSysColor` colours are still not watched (only a top-level window receives `WM_SETTINGCHANGE` and `WM_SYSCOLORCHANGE`). One change that raises several notifications fires the callback once, 300 ms after the last, on every platform (it was KDE only). Each reader module lists its sources, and a test checks that each has a watched notification.
- **native-theme**: the Windows reader leaves a non-client font's weight to the preset when `lfWeight` is 0 (`FW_DONTCARE`, the default weight), which it reported as 100 (Thin); it reads the tooltip font from `lfStatusFont`, "the font used in status bars and tooltips" (NONCLIENTMETRICSW), as it does the status bar's; and it reports `reduce_transparency` when Windows' *Transparency effects* setting is off (`UISettings.AdvancedEffectsEnabled` false), where it always reported `false`. `watch::on_theme_change` fires when that setting changes (`UISettings::AdvancedEffectsEnabledChanged`).
- **native-theme**: on macOS the scroll bar's groove width follows the user's scroller style: the reader writes the width AppKit gives the style `NSScroller.preferredScrollerStyle` reports (`scrollerWidth(for: .regular, scrollerStyle:)`: 15 for the legacy style and 16 for the overlay style on macOS 14 and 15, 17 for both on macOS 26), where it stated the legacy style's 15 under either style.
- **native-theme**: a freedesktop icon size from 32768 up no longer panics in a debug build: freedesktop-icons computes with the size as an `i16`, so 32768 wrapped to -32768 and overflowed its directory distance ("attempt to subtract with overflow"; in a release build it picked a directory by the wrapped size). A size above 32767 looks up as 32767, past the largest directory.
- **native-theme**: a GTK-convention theme's `process-working-symbolic` spinner has its foreground placeholders (`#2e3436` and the like) turned into `currentColor`, as every other symbolic icon of the theme has; it kept them, so the connectors, which colour `currentColor`, drew it dark grey on a dark theme.
- **native-theme**: `icons::is_freedesktop_theme_available`, `icons::list_freedesktop_themes` and the freedesktop icon lookup agree on the installed themes: a theme whose first `index.theme`, in the Icon Theme Specification's order (`~/.icons`, `$XDG_DATA_HOME/icons`, then each `$XDG_DATA_DIRS` entry's `icons`), lists `Directories` in its `[Icon Theme]` group, read once per process at the first call of any of them. `is_freedesktop_theme_available` accepted any `index.theme`, so a cursor theme (`breeze_cursors`) or `default`, which only inherits Adwaita, was available and `default_icon_choice` took it as the default (an icon is no longer looked up in either, as in any theme that is not installed); a theme installed while the process ran was listed and available but loaded no icon until a restart; and `list_freedesktop_themes` missed a theme that writes `Directories = …`. A theme installed both for the user and system-wide is read from the user's `index.theme` (the theme chain merged both, the system's `Inherits=` first); freedesktop-icons still takes each icon from the system copy where both copies have it.
- **native-theme**: `icons::default_icon_choice` takes an SF Symbols theme as the default only on macOS and a Segoe Fluent one only on Windows, each with the `system-icons` feature, where its loader is compiled; elsewhere it returns `IconSetChoice::System`, as for a freedesktop theme that is not installed (every icon loaded from such a default was `None`).
- **native-theme**: a Segoe Fluent glyph icon is its font's `size × size` em box with the glyph where the font places it (its black box at `gmptGlyphOrigin`, measured from the em box's top, the font's ascent less its internal leading), so every glyph of one size has the same raster and they line up; each came back at the size of its own ink ("Remove", a bar, was a few pixels high). A Segoe stock icon whose device context or `GetDIBits` fails is `None`, not a transparent icon, and the Segoe and SF Symbols loaders return `None` for an image whose pixels exceed the 64 MiB icon decode bound (an SF Symbol has twice its points in pixels on a 2× display).
- **native-theme**: `rasterize_svg` centres an SVG whose aspect ratio is not 1:1 at the offset it computes; the offset was scaled with the icon, so a 10 × 5 SVG rasterised at 100 landed below the pixmap and came out fully transparent. A size whose `size × size` RGBA buffer is over 64 MiB (above 4096), the decode bound of the other icon buffers, is `Err`, and the buffer is allocated fallibly; `rasterize_svg(svg, 100_000)` aborted the process on a 40 GB allocation. An `<image>` is drawn only from an inline `data:` URL: an `href` naming a file was read from disk, relative to the process's working directory. Its errors are `ReaderFailed` with reader `svg-rasterizer` (it was `svg_rasterizer`), of kind `Platform`, as its documentation now states.

## [0.6.1] - 2026-10-09

### Added

- **native-theme-gpui**: `geometry::button_label`, `geometry::toggle` and `geometry::data_table_size`; `Mic` and `Square` in the Lucide, Material and freedesktop tables.
- **native-theme**: the bundled Lucide and Material sets carry `mic` and `square`.
- **Showcases (gpui)**: the Inputs page shows `ColorSelect` and `SpeechWaveform`.
- **native-theme-gpui**: `widgets::Spinner` takes a `Size` (it is `Sizable`), drawing the platform's indicator at every size: gpui-component's own spinner ladder (× 0.75, × 0.875, × 1.5 for XSmall, Small, Large) anchored at `spinner.diameter`, never below `spinner.min_diameter`; `SpinnerLook::of_size` is that look.

### Changed

- **native-theme-gpui**: requires gpui-component, gpui-base and gpui-kit 0.7.1 and gpui-pre 0.3.8 (MSRV unchanged, 1.95.0). The three `IconName` tables return `None` for a variant newer than the connector, instead of failing to compile.

### Fixed

- **native-theme-gpui**: 0.6.0 does not build against gpui-component 0.7.1, which added `IconName::Mic` and `IconName::Square`; 0.6.1 does. A Button's label, a Toggle's text and DataTable cells can be drawn at the platform's font size again, which gpui-component 0.7.1 shrank to 0.875 of it at `Size::Medium` (through `geometry::button_label`, `geometry::toggle` and `geometry::data_table_size`).
- **Showcases (gpui)**: the Feedback page's Spinner row draws its three sizes with the platform's look; Small and Large were gpui-component's Lucide `Loader` beside the Medium platform indicator.
- **native-theme**: a name a freedesktop icon theme does not have is answered from a per-theme index of the theme's icon files instead of freedesktop-icons' whole fallback search, whose result was discarded: a missing name's load took 47-66 ms and takes under 0.01 ms, and Breeze's `face-smile`, found only on the plain pass, 0.16 ms instead of 8.8 ms (medians of 20 `FreedesktopLoader` loads in a dev build, on Breeze and Adwaita). The index is built once per theme per process (about 50 ms for Breeze, 3 ms for Adwaita); every lookup returns the file it returned before.
- **native-theme**: an OS query (`gsettings`, `xrdb -query`, `xrandr`) returns as soon as its command exits instead of on the next 50 ms poll: the poll interval starts at 1 ms and doubles up to 50 ms. Detecting the font DPI takes 3.5 ms instead of 50.5 ms, and `AccessibilityPreferences::from_system` 12.7 ms instead of 107 ms (dev profile); a command that outlives the 2 s timeout is still killed.

## [0.6.0] - 2026-10-01

### Added

- **native-theme-egui**, the egui toolkit connector (egui 0.36.2). A `ThemeAtlas` compiles a theme into egui's two base styles, a style per widget role and appearance variant, and a frame per container surface; `install` puts them into an `egui::Context`, and `native_scope`, `native_set_style`, `role_modifier` and `surface_frame` apply the per-widget ones through egui's own seams. Icons follow the theme's icon set and, per colour scheme, its freedesktop icon theme; the platform's typeface is installed with feature `system-fonts` (default); a `ThemeWatcher` (feature `watch`) rebuilds on an OS theme change. `mapping.toml` states where every theme value lands, and the crate's docs render it. Every border colour is painted as stated: `defaults.border.opacity` is already folded into it, and no connector multiplies a colour by it. The showcase, `showcase-egui`, is the gpui showcase's application in egui, with per-instance Widget Info generated from the manifest; with `--screenshot` it captures the window with its title bar on macOS and Windows (through xcap), as the gpui and iced showcases' captures do, and refuses elsewhere, where the capture scripts take the window.
- **native-theme-egui-widgets**, a companion crate of the egui connector: the widgets egui's own cannot draw from a theme — a switch, a slider whose knob is not its rail, a spinner at the theme's stroke and a segmented control — and egui's links, drop-down, radio button, progress bar and expander wrapped in what the theme states for them. Everything it draws comes from the `ResolvedTheme` of the atlas `native-theme-egui` installed. Targets egui 0.36.2.
- **native-theme**: font matching by family, weight and style — `fonts::select_face` over `fonts::FaceTraits`, and behind the new `system-fonts` feature `fonts::system_face`, whose `SystemFace::family` is the family the font database records for the face; `fonts::is_macos_system_ui_family`.
- **native-theme**: `IconId` converts from any `&T` whose `T` implements `IconProvider`, so a loader takes a generated icon enum by reference — `MaterialLoader::new(&AppIcon::PlayPause)`, as native-theme-build's README shows; it needed a cast to `&dyn IconProvider` before.
- **native-theme**: `SystemTheme::icon_theme_for(mode)`, the icon theme of either variant.
- **native-theme**: `icons::colorize_monochrome_svg`, one colouriser for every connector.
- **native-theme**: `SfSymbolsLoader::color` / `color_opt` and `SegoeIconsLoader::color` / `color_opt` draw the system sets' monochrome glyphs in a chosen colour, keeping their alpha, as `FreedesktopLoader::color` does for symbolic icons. Without a colour the output is unchanged (Segoe glyphs white, SF Symbols black); full-colour stock icons (`SIID_*`, `IDI_QUESTION`) ignore it.
- **native-theme-iced**: the `system-fonts` feature (default) and `system_font_family`, the family iced's font database holds for the stated face — on macOS the system UI font's own name for "SF Pro".
- **Showcases**: `--pointer X,Y` holds the pointer at a point of the window and `--press` holds the primary button down there, so a capture shows a control hovered or pressed where nothing can move the real pointer, as in a nested compositor. A value that is not `X,Y` is reported, and the showcase exits with 1.
- **Showcases**: a Basic page in all three, their default page and the one every capture shows: the controls all three draw — buttons, checkboxes, radio buttons, text and a link, text fields, a drop-down, a slider, a progress bar and a tooltip — in the same order, labels and states, packed onto one screen, so the three captures compare control by control. Each page is laid out by the theme: groups and columns `layout.section_gap` apart and inset by `layout.window_margin`, headings in `text_scale.section_heading`, body text in `defaults.line_height`; buttons, text fields and the drop-down at the theme's minimum sizes (in egui per call, since the connector leaves `button.min_width` and `input.min_height` to the application; in iced through `at_least` and `control_line_height`); the link underlined where `link.underline_enabled` says so, and unpadded, as the theme states no padding for a link; the gpui tooltip built by the application and refined by `geometry::tooltip`, and the iced one padded as the theme states where all four sides agree.
- **native-theme-gpui**: `geometry::input_fill(n, disabled)`, an `Input`'s fill from `input.background_color` or `input.disabled_background` (gpui-component fills a light-theme field with the window background, and no `ThemeColor` field reaches it), and `geometry::link`, which honours `link.underline_enabled` (gpui-component underlines every link at rest) and carries `link.font`'s size and weight. The showcase applies both.
- **native-theme-gpui**: `geometry::button_disabled`, a disabled `Button`'s fill and label colour from `button.disabled_background` (or `button.background_color`) and `button.disabled_text_color`: gpui-component paints its own faded literals and then replays the caller's refinement over them, so the platform's pair arrives. The showcase applies it to its disabled Buttons (kde-breeze: `#f0f0f0` and `#a8a9aa`; the window showed through at half opacity).
- **native-theme-iced**: minimum sizes. `control_line_height` gives a text input's or a pick list's text the line height that makes the control `input.min_height` or `combo_box.min_height` tall inside its padding (iced has no minimum-height setter); `button_content_min_size` and `at_least` put a floor of `button.min_width` × `button.min_height` under a button's label. `combo_box_padding` is the pick list's padding, `padding_inside_border` the rule the padding helpers share.
- **native-theme**: `checkbox.radio_dot_diameter_px` (`CheckboxTheme::radio_dot_diameter`), the dot a selected radio button draws, an optional size like `menu.row_height`: KDE 6, GNOME 8 and Windows 12 (`docs/platform-facts.md` §2.5: Breeze's `renderRadioButton`, libadwaita's `bullet.svg`, WinUI's `RadioButtonCheckGlyphSize`). macOS publishes none, so there it stays unstated and each toolkit draws its own dot. The kde-breeze, adwaita and windows-11 presets, their live presets, and the KDE and Windows readers state it.
- **native-theme-gpui**: `widgets::Radio` draws that dot, in `checkbox.indicator_color` and centred in the circle, instead of gpui-component's check glyph.
- **native-theme-iced**: `radio(&resolved, radio(..), is_selected)`, a radio at the platform's indicator size and label gap in `styles::radio`, with the stated dot laid over iced's own (iced draws its dot at half the circle, and `radio::Style` has no size for it).
- **native-theme-egui-widgets**: `radio_button::RadioButton`, egui's radio button in the checkbox role's scope with the stated dot (egui sizes its dot from the check mark's box); the egui showcase's Basic page uses it.
- **native-theme**: `checkbox.radio_indicator_width_px` (`CheckboxTheme::radio_indicator_width`), the radio circle's width where it differs from the check box's `indicator_width`: Material 20 against its 18 (`comp.radio-button` `icon-size`, `docs/platform-facts.md` §2.5); KDE, GNOME, Windows and macOS size both alike, so there it is unstated and the radio is `indicator_width` across. gpui's `widgets::Radio`, the iced connector's `radio` and egui-widgets' `RadioButton` draw it.
- **native-theme**: `switch.unchecked_thumb_diameter_px` and `switch.unchecked_thumb_background` (`SwitchTheme`), an off switch's thumb where the platform draws it apart from the on one (`docs/platform-facts.md` §2.21): Material 16 across in `outline` (`#79747e`, dark `#938f99`) against 24 in `on-primary`; Windows `ToggleSwitchKnobFillOff`, `TextFillColorSecondary` (`#0000009e`, dark `#ffffffc5`); GNOME dark libadwaita's `$slider_color`, white 80 % over the view background (`#d2d2d2`). Unstated, the off thumb is `thumb_diameter` across in `thumb_background`. gpui's `widgets::Switch`, the iced connector's `switch` and `styles::toggler`, and egui-widgets' `Switch` draw it; egui's thumb grows as it travels.
- **native-theme**: the active tab's indicator line (`docs/platform-facts.md` §2.11): `tab.active_indicator_color`, `tab.active_indicator_width_px` and `tab.active_indicator_side` (a new enum, `TabIndicatorSide::Top` / `Bottom`). Breeze fills a `TabBar_ActiveEffectSize` 3 strip of `Highlight` along a North tab's top edge; libadwaita's GtkNotebook insets 4 of `--accent-bg-color` at the bottom; Material's primary tabs a 3 `primary` indicator at the bottom; WinUI's TabView and AppKit's NSTabView mark the active tab by fill and border alone, so there all three stay unstated. The kde-breeze, adwaita and material presets state them, the live presets the width and side, and the KDE and GNOME readers the colour (the selection background, the portal accent).
- **native-theme-gpui**: `widgets::TabBar` draws the line across the selected tab's outer edge (`TabLook::indicator`, `widgets::TabIndicator`).
- **native-theme**: `button.checked_background` and `button.checked_text_color`, a toggle button that is on (`docs/platform-facts.md` §2.3): Breeze's button colour mixed an eighth towards the button text (`#e1e1e2`, dark `#43464a`) in the button text; WinUI's `ToggleButtonBackgroundChecked`, the accent fill, in `TextOnAccentFillColorPrimary`; libadwaita's `$button_checked_color`, the pressed fill's 30 %, in the button's own text; Material's `latest` tonal `selected-container-color` `secondary` in `on-secondary`. AppKit states none, so there a toggle that is on shows the pressed pair, as every toggle did before. The KDE reader computes the pair from `kdeglobals`, the Windows reader the fill from the accent.
- **native-theme-gpui**: `geometry::button_checked`, a selected Button's fill and label from the checked pair (the pressed pair where unstated); the showcases' Basic toggle `On` shows the checked pair in all three.
- **native-theme-iced**: `tab_indicator`, a tab with the line stacked over it where it is active, and `tab_indicator_line`, the line alone; the iced showcase's tab rows use the first, its `iced_aw` Basic tab bar the second.
- **native-theme-egui-widgets**: `progress_bar::ProgressBar`, egui's progress bar rounded `progress_bar.border.corner_radius` and outlined as `progress_bar.border` states (egui's draws no outline); the egui showcase's Basic page uses it.
- **native-theme-iced**: `pick_list_handle`, the drop-down arrow as the open chevron KDE, GNOME and Windows draw — iced's own `Iced-Icons` chevron glyph (U+E803) at `combo_box.arrow_icon_size`, where `Handle::Arrow` is a filled triangle; the iced showcase's drop-downs use it.
- **native-theme**: `docs/platform-facts.md` §2.10 states the progress bar's border: Breeze strokes groove and contents with a 1.001px pen, the groove in the window text at `frameContrast` 0.2 (`#23262933`, dark `#fcfcfc33`); libadwaita and WinUI draw none. kde-breeze states that colour and a 1px width, adwaita and windows-11 a width of 0; the KDE reader derives the colour from `kdeglobals`.
- **native-theme**: `tab.item_gap_px` (`TabTheme::item_gap`), the space between neighbouring tabs, an optional size like `toolbar.item_gap`: 0 on KDE (QTabBar lays tabs edge to edge), GNOME (GtkNotebook) and Windows (TabView), none on macOS (`docs/platform-facts.md` §2.11). egui writes it into the tab scope's `item_spacing.x`; gpui's `widgets::TabBar` and the iced showcase's tab rows lay their tabs that far apart.
- **native-theme**: `checkbox.check_mark_stroke_width_px` (`CheckboxTheme::check_mark_stroke_width`), the check mark's line: KDE 2 (Breeze's check pen, `PenWidth::Frame * 2`), GNOME 2 (libadwaita 1.10.0's `check.svg`; 1.7–1.9 draw 2.5), none on Windows (a font glyph) and macOS (`docs/platform-facts.md` §2.5). egui strokes its check in the checkbox scope's `fg_stroke`, which carries it; gpui's `widgets::Checkbox` paints gpui-component's own check polyline at that width; the iced showcase paints a tick at it over iced's glyph.
- **native-theme**: the expander's structure (`docs/platform-facts.md` §2.27), KDE's expander being KWidgetsAddons' `KCollapsibleGroupBox`: `expander.arrow_side` (a new enum, `ArrowSide::Leading` / `Trailing`; KDE and macOS lead, GNOME's `AdwExpanderRow` and WinUI trail), `arrow_gap_px` (KDE 9, GNOME 9, Windows 30), `content_indent_px` (KDE 20, GNOME 0, Windows 16) and `frame_enabled` (KDE no, GNOME and Windows yes), all optional.
- **native-theme**: `[text_area]` (`TextAreaTheme`), the multi-line field's padding, which differs from the single-line field's: KDE 5 (Breeze's QTextEdit frame 2 plus QTextDocument's `documentMargin` 4, less the frame line; measured on a real Breeze QTextEdit), GNOME 0 (GtkTextView), Windows 10 / 6 / 5 / 6 (the same TextBox), none on macOS (`docs/platform-facts.md` §2.29). Its border's colour, radius, width and shadow inherit `input.border`'s; everything else of a text area is `input`'s.
- **native-theme-egui**: `text_area_margin` and `text_area_frame`, a multi-line `TextEdit`'s margin and frame from `text_area.border`, as `input_margin` and `input_frame` are the single-line field's.
- **native-theme-egui-widgets**: `expander::Expander`, an expander laid out as `expander.arrow_side`, `arrow_gap`, `content_indent` and `frame_enabled` state (egui's `CollapsingHeader` ties the arrow, the title and the body to one indent, always leads with the arrow and frames the header alone); egui's own `CollapsingHeader` where the theme states none. The egui showcase's Basic page uses it.
- **native-theme-gpui**: `geometry::text_area`, the refinement for a multi-line `Input` or `Textarea` from `text_area.border`: the root padded by the rest of each stated side past the editor padding upstream gives the field in render (a side below it stays upstream's).
- **native-theme-iced**: `text_area_padding`, a `TextEditor`'s padding from `text_area.border`, iced's own 5 on a side the theme leaves unstated.
- **Showcases**: the expanders follow `expander.arrow_side`, `arrow_gap`, `content_indent` and `frame_enabled` in all three, and the text areas the text area's own padding.
- **native-theme**: `fonts::substitute_family(family)`, the family a system without `family` draws it in: fontconfig's substitute (`fc-match`) on Linux, `None` on macOS and Windows, whose fallback has no documented source to follow.
- **native-theme-egui**: a theme font family the system lacks is drawn in `fonts::substitute_family`'s substitute, as every native Linux application gets it, instead of egui's own face; `Note::FontFamilyUnavailable` gains `substitute`, the family drawn instead.
- **native-theme-gpui**: `font_family`, the family gpui draws a theme family in — the substitute where the system lacks it — behind the new `system-fonts` feature (default).
- **native-theme-gpui**: `widgets::Part` and `widgets::PartBounds`: `on_part_bounds` on `Checkbox`, `Radio`, `Switch`, `Slider`, `ProgressBar` and `TabBar` reports the rectangle each part was painted in.
- **native-theme-egui-widgets**: `parts::Parts`, the rectangles a widget painted its parts in.
- **Showcases**: `docs/showcase-elements.toml`, one list of the elements the three showcases share — chrome and Basic page, each with the theme leaves it draws. Widget Info shows the hovered element from it in one format in all three: its name and state, a row per leaf with the resolved value and one line on how that toolkit applies it, in `sidebar.font`. `--dump-layout FILE` writes every listed element's rectangle, `--open-menu theme` opens the Theme menu for a capture, and `scripts/check_showcase_parity.py` compares the three showcases' dumps and captures element by element — sizes, positions and sampled colours — failing on any difference `docs/showcase-exceptions.toml` `[parity]` does not give a reason for.
- **native-theme-iced**: the `advanced-shaping` feature (default) kerns every text of the application, as Qt, GTK, WinUI and AppKit do: it turns on `iced_core`'s `advanced-shaping`, which makes `Shaping::Advanced` the default of every text that sets none, where iced's own default shapes ASCII text with the glyphs' advances alone (`Shaping::Auto` picks `Basic`). Cargo builds one `iced_core` for the application and the connector, so depending on the connector is enough; the README and crate docs say so under *Kerning*.
- **native-theme-gpui**: icon mappings for gpui-component 0.7.0's `IconName::Ban`, `CircleAlert` and `RefreshCw` (Lucide `ban`, `circle-alert`, `refresh-cw`; Material `block`, `error`, `refresh`; freedesktop `action-unavailable`, `emblem-important`, `view-refresh`), and `icon_name(IconRole::ActionRefresh)` now returns `RefreshCw`.
- **Showcases (gpui)**: the Buttons page shows gpui-component's `Toolbar` with two labelled `ToolbarGroup`s; the Inputs page a `TimeField` and an `Input` holding an inline token; every chart states the tooltip and guide line it draws on hover; the icon gallery has gpui-component's 104 icons.

### Changed

- **native-theme**: `defaults.border.opacity` is the share of the text colour already folded into every stated border colour — Breeze's `frameContrast` 0.2, libadwaita's `--border-opacity` 0.15, WinUI none (`docs/platform-facts.md` §2.1.6) — and `border.color` is the final line colour: no connector multiplies a colour by it. native-theme-egui no longer folds it into the strokes of `defaults.border.color`, which on kde-breeze faded `#bcc0bf` to 20 %.
- **native-theme**: one disabled mechanism per platform, and connectors apply both. A disabled widget is painted in its disabled colours *and* faded by its `disabled_opacity`; the data makes the one the platform does not use an identity (`docs/platform-facts.md` §2.1.6). kde-breeze and windows-11 (and their live presets) state `defaults.disabled_opacity = 1.0` (was 0.5 and 0.3): Breeze and WinUI dim by colour alone; so does material (was 0.38), whose disabled colours carry Material 3's per-part 0.12 and 0.38. adwaita states no disabled fills (a `None` disabled fill is the enabled one) and its enabled text colours as the disabled ones (`defaults.disabled_text_color` `#2e3436` / `#ffffff`, was `#cccccc` / `#4a4a4e`): libadwaita dims by `--disabled-opacity` 50 % alone. A widget with no `disabled_opacity` of its own (menu, list, link) fades by the defaults'.
- **native-theme**: kde-breeze's disabled colours are Breeze's Disabled palette, computed with KColorScheme 6.30.0 from `[ColorEffects:Disabled]` (`docs/platform-facts.md` §2.1.6): text `#a0a1a3` / dark `#686a6c` (was `#bcc0bf` / `#4d545b`); button and drop-down `#f0f0f0` on `#a8a9aa` / dark `#272a2e` on `#6d6f72` (dark was `#3a3f44` on `#4d545b`); text field `#f3f3f3` on `#aaabac` / dark `#131517` on `#606263` (was `#f0f0f0`, `#282c30`); checkbox box `#f0f0f0` / `#272a2e`, label `#a0a1a3` / `#686a6c`; menu `#a0a1a3` / `#686a6c`; list `#aaabac` / `#606263`; link `#a3cae2` / `#164160`.
- **native-theme**: kde-breeze's `combo_box.arrow_icon_size_px` is 10, Breeze's `ArrowSize` (`renderArrow` draws `min(w, h, 10)`), in the 20px arrow column `arrow_area_width` states (was 20). `docs/platform-facts.md` §2.24 records that the drop-down arrow is an open chevron on KDE, GNOME and Windows.
- **native-theme**: kde-breeze's `progress_bar.fill_color` is Breeze's, the accent at alpha 0.7 over the window, `#72c2eb` / dark `#3584ae` (`KColorUtils::overlayColors`, computed with Qt 6.11.2); the KDE reader computes it from `kdeglobals` in Qt's own arithmetic.
- **native-theme**: windows-11's list item padding is 16 left / 12 right, WinUI's `DefaultListViewItemStyle` `Padding="16,0,12,0"` (was 12 / 12, `ListViewItemExpanded`'s); the Windows reader agrees.
- **native-theme**: `docs/platform-facts.md` §2.26: GNOME's `.card` has no CSS border — its outline is a 1px box shadow at 3 % — so its `border.color` and `border.line_width` are none.
- **native-theme-egui**: a `Disabled` cell carries its role's `disabled_opacity` as `disabled_alpha` (the defaults' for menu, list and link), which egui fades a disabled widget by on top of the disabled colours; it was `1.0`. A primary button with no stated disabled fill keeps its own pair, faded.
- **native-theme-egui-widgets**: the drop-down's arrow is an open chevron over egui's arrow rectangle, stroked in the state's `fg_stroke`, where egui fills a triangle; on kde-breeze, with its 10px arrow, the drop-down is `combo_box.min_height`, 32, tall (it was 34). A widget added to a disabled `Ui` opens its `Disabled` scope too. Disabled widgets are faded by their role's `disabled_opacity`.
- **native-theme-iced**: every disabled style multiplies the alpha of each colour it emits by the widget's `disabled_opacity` (iced has no widget opacity): buttons, links, text fields and editors, checkboxes, togglers and `iced_aw` selection lists. A disabled checkbox with no stated disabled fill is its enabled box; a disabled class button (primary, danger, …) with none keeps its own pair.
- **native-theme-gpui**: `widgets::Checkbox`, `Radio`, `Switch` and `Slider` fade a disabled control by its `disabled_opacity` (`CheckboxLook`, `SwitchLook` and `SliderLook` gain `opacity`); a disabled checkbox with no stated disabled fill is its enabled box. `geometry::button_disabled` and `geometry::input_fill(n, true)` carry `disabled_opacity` as the refinement's opacity, and a disabled field with no stated disabled fill takes its enabled fill (it kept gpui-component's own).

- **native-theme**: kde-breeze's checkbox and radio button are Breeze's: a checked indicator is the button colour under the selection colour at `Metrics::Blend_Value` 0.3 (`#c2e4f6`, dark `#2f5368`), outlined in the selection colour, with its mark in the View text colour (`#232629`, dark `#fcfcfc`); an unchecked one is outlined in Breeze's `separatorColor()` (`#c6c8c9`, dark `#4c4e51`), and the dark one is filled with the button colour (`#292c30`). They were a solid accent box with a white mark. The KDE reader computes them from `kdeglobals`.
- **native-theme**: in `docs/platform-facts.md` §2.5, GNOME's `indicator_width` is 20 — libadwaita's 14px minimum plus its 3px padding on each side, as GTK sizes it — which the adwaita preset already stated.
- **native-theme**: in `IconSet::SegoeIcons`, `ActionSearch`, `ActionSettings`, `ActionDelete` and `ActionPrint` are the Segoe Fluent glyphs `Search` (U+E721), `Settings` (U+E713), `Delete` (U+E74D) and `Print` (U+E749), like the other action roles; they were the full-colour shell stock icons `SIID_FIND`, `SIID_SETTINGS`, `SIID_DELETE` and `SIID_PRINTER`.
- **Showcases**: all three open at 1280 × 720. With `--screenshot`, or the new `--capture` for a capture another tool takes, the window opens at that size whatever size the desktop remembers for it, and every capture path fails, with the measured and the expected size, unless the capture is of a window at that size. The macOS screenshot runner's display is switched to 1920 × 1080, which no longer clamps the window to 1024px.
- **native-theme-gpui**: on macOS, a family that names the system UI font — "SF Pro", or the name the live reader reports — is drawn as gpui's `.SystemUIFont`.
- **native-theme-gpui**: `geometry::tooltip` carries `tooltip.background_color` and `tooltip.border.color`: gpui-component paints a tooltip with the popover's fill and the window's border, so kde-breeze's `#f7f7f7` tooltip was white and adwaita's dark one light.
- **native-theme-iced**: `button_padding` and `input_padding` add the border's line width to each side the theme states: the theme's padding lies inside the border, while iced lays content out from the widget's edge and paints the border over the padding, so a themed button's or field's content sat one border width closer to the edge than the platform's.
- **Showcases**: the iced showcase lays its text out in the theme's line boxes: body text at `defaults.line_height` and text-role text at the role's `line_height` (iced's own is 1.3 times the size).
- **Showcases**: the gpui showcase sets its section headings in the theme's section-heading role (`text_scale.section_heading`: size, line height and weight), as the iced showcase does; they were semibold body text (kde-breeze: 16px regular, was 13.33px semibold).
- **native-theme-iced**: `custom_icon_to_svg_handle` and `custom_icon_to_image_handle` load a provider's system-set icon through `FreedesktopLoader`, `SfSymbolsLoader` or `SegoeIconsLoader` in `color`; `custom_icon_to_image_handle` gains that `color: Option<Color>` parameter, so an SF Symbol or a Segoe glyph arrives in it instead of its fixed black or white.
- **native-theme**: the kde-breeze, adwaita and macos-sonoma presets state `link.hover_background` as transparent: the platforms draw no background under a hovered link (`docs/platform-facts.md` §2.28 gives none for macOS, KDE and GNOME), and the presets stated the accent at 9% alpha (`#3daee918`, `#3584e418`, `#007aff18`), which the iced showcase painted.
- **Development**: an MSRV job in CI (the workspace floor `1.88.0`, and the connectors' own floors); the egui connector in every gate that names connectors; the widget-coverage script covers egui and egui_extras; the screenshot pipeline captures the egui showcase; the scripts are named by task group (`check_`, `update_`, `generate_`), and `pre-release-check.sh` is now `scripts/check_release.sh`.
- **native-theme**: kde-breeze states no toolbar frame (`toolbar.border.line_width_px` 0, Breeze's `ToolBar_FrameWidth`, `docs/platform-facts.md` §2.13); it inherited the default border's 1px.
- **native-theme-iced**: `combo_box_padding` also centres the arrow in `combo_box.arrow_area_width` (`docs/platform-facts.md` §2.24).
- **Showcases**: the Basic page holds every control all three toolkits draw, in five columns: buttons (with a toggle pair), checkboxes, radio buttons, a drop-down; text inputs (with a focused one), a text area, a slider; switches, a number input, a spinner, a segmented control, a card; typography (caption to display, a link, monospace), a separator, a progress bar, a list; icon buttons and icons, tabs, an expander and a table. The list shows three of its eight rows and the table two rows, the second selected. Where a preset's controls are too wide for five columns the page takes four, or three, counted from the theme alone in all three showcases: the most columns whose width holds `combo_box.min_width`, the two tabs at `tab.min_width`, and the page's own control widths, in the content panel less `layout.window_margin` and the `scrollbar.groove_width` a scrolling page keeps free (material: four); the fifth column's groups then end the other four. A row of controls wider than its column wraps onto a further line, the drop-down is at least `combo_box.min_width` wide in all three, and a page taller than its panel scrolls vertically, at rest at the top, never sideways.
- **native-theme-gpui**: `widgets::TabBar::wrap`, tabs that wrap onto a further row where the bar is too narrow, instead of scrolling sideways.
- **native-theme-egui-widgets**: `segmented_control::SegmentedControl::wrap`, segments that wrap onto a further line inside the one outline where the `Ui` is too narrow.
- **Development**: `scripts/check_showcase_parity.py` takes a colour sample only where it is visible: inside the window and inside every ancestor the element list marks `clip = true` (the content panel a page scrolls in); a sample elsewhere is counted as "not visible" and left out of the comparison, while geometry is compared from the dumps, which hold off-screen elements too.
- **native-theme**: kde-breeze's single-line input pads 7 horizontally and 6 vertically inside its frame line (was 6 / 3): Breeze insets a line edit's contents `LineEdit_FrameWidth` 6 from its outer edge, the 1px line the outermost of those, and QLineEdit sets its text 2 across and 1 down further in; a real Breeze QLineEdit measured 2026-09-28 agrees (`docs/platform-facts.md` §2.4). The KDE reader agrees.
- **native-theme**: material's outlined text field, text area and drop-down pad 15 inside their 1px outline (the drop-down 15 leading, 11 trailing), where material-web sets their content 16 (and the arrow's 12) from the field's edge and lays the outline over that space (`field/internal/_outlined-field.scss`, `outline-width` 1; `docs/platform-facts.md` §2.4, §2.24, §2.29); they stated 16 and 12, which the connectors draw inside the outline, so the fields were 2px taller and wider than Material's.
- **native-theme-gpui**: requires gpui-component and gpui-base 0.7.0 and gpui-pre 0.3.7 (gpui-kit 0.7.0 for the showcase). `ThemeColor::chart_grid`, new in gpui-component 0.7.0, takes `list.grid_color`, on the direct and the config path; the native `Switch` draws the keyboard focus ring while focused, as gpui-component 0.7.0's own switch does; `geometry::toolbar` sets the height to the content's (at least `toolbar.bar_height`), which replaces the fixed `h_8` of gpui-component's new `Toolbar`.
- **native-theme-gpui**: documented which resize handles the splitter colours reach — groups built with gpui-base's `h_resizable` / `v_resizable`, at gpui-base's 1px; not gpui-component's, `Settings`' divider or the dock's edges, which install gpui-component's own renderer — and that `gpui_kit::init` must run before the first window opens, or the window's first dialog, sheet or notification panics in gpui-component and its rem stays gpui's default (`gpui_kit::open_window` in the Quick start).

### Fixed

- **native-theme-gpui**, **native-theme-iced**, **native-theme-egui-widgets**: a disabled switch, check box or radio button on a platform that dims by opacity (GNOME, `docs/platform-facts.md` §2.1.6) fades as one widget over what lies under it, as libadwaita's `filter: Opacity(..)` does: its thumb or mark keeps its colour on the track or fill, and the whole fades over the backdrop, the window's `defaults.background_color` unless the caller names another. Each toolkit faded every part on its own, so the thumb showed through the faded track (adwaita's disabled switch thumb `#cbdff7`, dark `#95a9c2`; now `#fcfcfd` and `#909092`). gpui's `Checkbox`, `Radio` and `Switch` gain `backdrop` (`CheckboxLook::faded_over`, `SwitchLook::faded_over`); the iced connector gains `switch_over`, `styles::checkbox_over` and `styles::toggler_over`; egui-widgets' `Switch` and `RadioButton` gain `backdrop`, and `fade::fade_as_one` does the same for egui's own check box.
- **native-theme-egui-widgets**: the painted `Spinner` arc always sweeps 240°, as the iced and gpui spinners do; egui's `240° · sin(t)` shrank it to a dot twice a cycle, which a capture could catch — on adwaita, whose icon theme (adwaita-icon-theme 50.0) ships no `process-working` indicator, so that every connector paints the arc, the egui Basic page's spinner showed as a short tick.
- **Showcases**: the egui showcase frames an open menu in `popover.border`'s colour, line width and radius, as the gpui and iced showcases do; it framed it in `defaults.border`'s, so material's menu had an outline Material's has not.
- **Showcases**: the egui showcase's toolbar is `toolbar.bar_height` tall where the theme states one, its `layout.container_margin` padding inside that height as in the gpui and iced showcases (material: 64, it was 72).
- **native-theme**: dropping a GNOME or Budgie theme subscription returns at once instead of waiting for the next portal signal.
- **native-theme-gpui**: `custom_icon_to_image_source` draws a provider's SF Symbols and Segoe glyphs in `color`; they were black and white whatever `color` was. The provider's freedesktop icons load through `FreedesktopLoader::color` too.
- **Showcases (gpui)**: on gpui-kit 0.7.0, whose dialogs and sheets sit beside the view, the command palette's entries and the keyboard shortcuts work while a dialog or sheet has the focus (the actions are handled by a root plugin around the whole window); the capture pointer shield covers dialogs, menus and popovers, which it never covered.
- **Development**: the visual-assets provenance stamp (`scripts/update_provenance.sh`) and the egui compatibility stamp (`scripts/update_compatibility.sh`, which now also runs its tests, clippy and docs) cover `native-theme-egui-widgets`, which the egui showcase draws with; before, a change there left both stamps fresh.
- **Showcases (egui)**: the theme settings' drop-down rows are truncated to the drop-down's width, the whole text on hover: the icon-theme row naming why the system's theme is unavailable could carry a reader's whole error and widen the popup far past the window, where nothing in it could be clicked.

## [0.5.9] - 2026-09-25

### Breaking Changes

#### native-theme

- **Padding is per side.** `WidgetBorderSpec`'s `padding_horizontal` / `padding_vertical` are replaced by `padding_top`, `padding_right`, `padding_bottom` and `padding_left` (`Option<f32>`; TOML `padding_top_px` …). `padding_horizontal_px` / `padding_vertical_px` stay as shorthand for both sides of their axis; stating an axis key together with one of its sides is a parse error.
- `ResolvedBorderSpec` is split into `ResolvedDefaultsBorder` (`defaults.border`) and `ResolvedWidgetBorder` (every widget; `padding: ResolvedPadding` with `Option<f32>` sides). A widget border no longer carries `corner_radius_lg` or `opacity`.
- **An unstated size stays unstated.** A padding side the theme does not state resolves to `None` (it was `0.0`, which connectors applied over the toolkit's own padding). `toolbar.bar_height`, `toolbar.item_gap`, `menu.row_height`, `list.row_height` and `combo_box.arrow_area_width` are `Option<f32>`. A stated value below 0 is a validation error.
- `ResolvedFontSpec` gains `defined_size: Option<FontSize>`, the size in the unit its source stated; `FontSize` implements `Serialize` / `Deserialize`.
- **Icon-theme detection reports its failure instead of naming a theme.** `system_icon_theme()`, `detect_icon_theme()` and `DetectionContext::icon_theme()` return `Result<String>`; the error says which source failed and why. `Resolved::icon_theme` and `SystemTheme::icon_theme` are `Option<Cow<'static, str>>`. `FreedesktopLoader` with no theme and a failed detection loads nothing, except a custom `IconProvider`'s own freedesktop SVG.
- The empty public module `native_theme::kde::metrics` is removed.

#### native-theme-gpui

- Requires **gpui-component / gpui-base 0.6.6** and **gpui-pre 0.3.6**: gpui-component 0.6.2 removed `ThemeColor::tiles` in a patch release. The versions the connector was verified against are in its README.
- `reduce_motion: false` no longer clears a reduced-motion flag the connector did not set; to switch motion back on, call `cx.set_reduce_motion(false)`.
- `geometry::tooltip` no longer sets a maximum width (see `geometry::tooltip_content`). With `svg-rasterize` (default) the `icons` functions return `ImageSource::Render`, which holds a sprite-atlas tile until handed to `App::drop_image`.
- `geometry::control_height` and `native_theme_gpui::dialog_content_padding` are removed; `geometry::input_height` returns a `StyleRefinement`.
- `geometry` builders set only the padding sides, heights and gaps the theme states, leaving the widget's own elsewhere.

#### native-theme-iced

- `font_size` and `mono_font_size` take `&AccessibilityPreferences`; `from_system` returns the preferences as a fourth element.
- iced's `secondary.base` carries the platform's placeholder colour (it was the button surface, which made placeholders invisible), and `secondary.strong` is a copy of it; `container::secondary`, `progress_bar::secondary` and `button::secondary`'s hover change with it. Use `styles::button` for the platform's button colours.
- `background.base.text` is `defaults.text_color`, no longer replaced by iced's `readable()`.
- `button_padding` and `input_padding` need the `widgets` feature (on by default); a side the theme does not state is iced's default padding, where it was 0.

### Added

- **native-theme-iced**: `styles` (feature `widgets`, default), 20 per-widget style functions for the states iced's palette has no slot for; `styles::aw` (feature `iced_aw`) for `iced_aw`'s card, menu bar, tab bar, sidebar, selection list and spinner; `padding_or` and `stated_padding`; features `widgets`, `iced_aw` and the icon features (on by default, so `load_icon` works without enabling features on `native-theme`).
- **Both connectors**: `scaled_text_size(size, &prefs)`, a text size from the theme times the user's text-scaling factor.
- **native-theme-gpui**: `variants::ghost_button`, a flat button that hovers and presses with the platform's button colours; `geometry::tooltip_content`, `scrollbar_gutter`, `list`, `input_group_button` and `toolbar`; `geometry::dialog` also carries the dialog's corner radius.
- A **Compatibility** section in both connector READMEs: the upstream versions the crate requires and the dated set it was verified against.

### Changed

- **Both connectors**: status labels (`success`, `danger`, `warning`, gpui's `info`) are the platform's own colours; the white-or-black contrast enforcement is gone.
- **native-theme-gpui**:
  - `accent` / `accent_foreground` take the platform's menu-hover pair, and `sidebar_accent*` the sidebar's selection pair.
  - `link` takes `link.font.color`, `table_row_border` takes `list.grid_color`, and `geometry::icon_size_toolbar` takes `toolbar.icon_size`.
  - `ThemeColor` mapping 139 → 138 fields, `ThemeConfig` export 127 → 126 (`tiles` is gone upstream).
- **native-theme-gpui** `geometry`:
  - `status_bar`, `tooltip`, `dialog_description`, `list_item`, `radio`, `select` and `title_bar` also carry the platform's text colour, and `button` the label's font weight.
  - `input`, `select` and `combobox` apply the platform's padding sides.
  - `button`, `input`, `select`, `combobox`, `menu_item` and `list_item` follow one height rule: the stated height at a text-scaling factor up to 1, a minimum above it, with the platform's line height.
- **native-theme presets and readers**: the native presets and OS readers state the paddings, toolbar sizes and row heights `docs/platform-facts.md` documents, and no others. For example, the Windows menu row is 23 (it was 36; the reader read the menu *bar*'s `SM_CYMENU`), KDE states no menu or list row height, and the KDE status bar is padded 3/14/2/2. The colour-scheme presets, material and ios state none of these sizes.
- **native-theme**: the KDE reader reads `smallestReadableFont` into `text_scale.caption` and derives the heading sizes from the body font, as Kirigami's `Heading` does.
- **Showcases**: the gpui showcase is an application, with menus, a command palette and a Widget Info inspector. The iced showcase adds the `iced_aw` widgets. Both follow the chosen icon theme and reject command-line values they cannot honour.
- **Development**: mapping-contract tests, a contrast invariant and showcase self-tests in both connectors, rendered seam tests in the gpui connector, a documented-sizes gate, a nightly dependency canary and `scripts/check_features.sh` (every crate in every feature combination).

### Fixed

- **native-theme-gpui** 0.5.8 no longer compiled on a fresh dependency resolution, after gpui-component 0.6.2 removed `ThemeColor::tiles` (fixed by the new version requirement).
- **native-theme**:
  - A freedesktop icon comes only from the chosen theme or a theme its `Inherits=` chain declares, searched to the chain's end. `hicolor`, loose files in the icon directories and `/usr/share/pixmaps` no longer stand in for a missing icon.
  - A failed icon-theme detection no longer names `hicolor` (or `material` outside Linux, macOS, iOS and Windows).
- **native-theme**: `checkbox.indicator_color` (the check mark and radio dot) inherits `defaults.accent_text_color`, not `defaults.text_color`, which put the window's text colour on the accent fill (as low as 1.12:1).
- **native-theme** text, title and control sizes follow `docs/platform-facts.md`:
  - kde-breeze: `caption` 8pt (was 8.2), headings weight 400 (was 700), and no `display`.
  - macos-sonoma-live's text scale matches macOS's (e.g. `dialog_title` 22pt 400, was 15.6pt 700), and the `-live` presets' unsourced text-scale line heights are gone.
  - Dialog title fonts: macOS 13pt Bold, GNOME 15pt ExtraBold.
  - Windows progress track 1 (was 3), the Windows reader's slider thumb 18 (was 22), the macOS reader's slider track 5 (was 4).
- **native-theme**: the Windows reader's metrics and fonts are logical pixels (`font_dpi` 96). They were device pixels above 100% scaling, so text resolved 1.5× too large at 150%.
- **native-theme**: the Windows reader reads `defaults.border.line_width` (`SM_CXBORDER`) and the macOS reader `defaults.border.color` (`separatorColor`); the macOS reader no longer reports an invented text-scaling factor.
- **native-theme**: `system-icons` builds on macOS and Windows, `watch` builds without `portal`, and `watch` with `macos` builds on macOS.
- **native-theme-gpui** builds without `svg-rasterize` (SVGs are then handed to gpui undecoded).
- **native-theme-gpui**:
  - A tooltip's text wraps inside its bubble, and a non-overlay scrollbar no longer covers the content.
  - Animated icons no longer flicker on their first frames.
  - Under `reduce_transparency` a dialog's overlay no longer turns the window black.
- **native-theme-gpui** colours:
  - `link_hover` / `link_active` are the platform's link text colours (they were a 9%-alpha fill).
  - List and table headers take `list.header_*`, and the segmented tab bar's track `segmented_control.background_color`.
  - Layered hover and pressed colours (Windows 11) are composited over the button's fill.
- **native-theme-iced**: `to_theme`'s docs no longer list `Tooltip` among the widgets with a `Catalog`; **native-theme-gpui**: the docs of `geometry::input_height`, `menu_item` and `dialog` say what upstream applies.

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
- **native-theme**: `SystemTheme.layout: LayoutTheme`; `AccessibilityPreferences::from_system()`; 28 bundled SVGs (14 Lucide, 14 Material); `icons/SOURCES.toml` provenance manifest; `scripts/update_icons.sh` (the bundle reproduces byte-for-byte from the manifest); by-name icon tables generated by `build.rs` from the bundle directories (four Lucide and eleven Material files the hand-written tables had missed now resolve).

### Changed

- Declared explicit `[package.metadata.docs.rs]` targets (`x86_64-unknown-linux-gnu`, `x86_64-apple-darwin`, `x86_64-pc-windows-msvc`) with `all-features = true` on `native-theme` and `native-theme-iced` (`native-theme-gpui` keeps `all-features = true` only; see the `native-theme-gpui` docs.rs bullet in this section). Preserves multi-target API rendering on docs.rs after the 2026-05-01 policy change that reduces the default build set to a single target ([announcement](https://blog.rust-lang.org/2026/04/04/docsrs-only-default-targets/)). Platform-gated items such as `LinuxDesktop`, `winicons`, `sficons`, and the macOS/Windows readers remain visible across the three published target docs.
- **native-theme-gpui**: `ThemeColor` mapping 108 → 139 fields; the 28 `button_*` fields take the semantic values their variant used in 0.5.1 (solid native surfaces, not upstream's tint); `table_foot*` mirror `table_head*`; `status_bar*` from `StatusBarTheme`. The `ThemeConfig` copies carry upstream's default highlighter style for their mode, so a mode switch through `Theme::change` also switches code highlighting. Showcase on gpui-kit 0.6.0.
- **native-theme-gpui**: `[package.metadata.docs.rs]` keeps `all-features = true` and declares no `targets`: every platform-gated public item of the crate is Linux-gated and appears on the default target (v0.5.8 spec §1.3, D40).
- Dependency refresh: serde 1.0.229, serde_with 3.22.0, toml 1.1.5, serde_json 1.0.151, arc-swap 1.9.2, async-trait 0.1.92, ashpd 0.13.13, configparser 3.2.0, zbus 5.19.0, quote 1.0.47, proc-macro2 1.0.107, pollster 1.0, syn 3.0.5, resvg 0.48.1. Workspace MSRV re-measured: unchanged at 1.88.0 (`resvg` is used without default features, so its text stack never enters the lock).

- Release tooling: `scripts/generate_assets_release.sh` now records the provenance of the visual assets in `docs/assets/PROVENANCE.toml` (workspace version, commit, and a hash over the git object ids of every path that feeds the showcases); `scripts/check_release.sh` recomputes the hash at HEAD (a warning while the CHANGELOG entry is unreleased, a hard failure once it is dated) and the crates.io workflow's CI gate refuses a tag whose assets were captured from other sources or whose name differs from the workspace version.

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
- `scripts/check_release.sh` timeout added (30 min max)
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
- `scripts/check_release.sh` uses `jq` instead of `python3` for JSON parsing (with bash fallback)

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

[0.6.1]: https://github.com/tiborgats/native-theme/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/tiborgats/native-theme/compare/v0.5.9...v0.6.0
[0.5.9]: https://github.com/tiborgats/native-theme/compare/v0.5.8...v0.5.9
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
