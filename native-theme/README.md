# native-theme

[![License: (MIT OR Apache-2.0 OR 0BSD) AND Apache-2.0 AND MIT](https://img.shields.io/badge/license-(MIT%20%7C%20Apache--2.0%20%7C%200BSD)%20AND%20Apache--2.0%20AND%20MIT-blue.svg)](#license)

## What it does

A theme model for Rust GUI toolkits. A `Theme` is a bundled platform preset
with the application's layers over it; resolved for a window, under that
window's conditions, it gives the style of each part of each widget (its
fill, stroke, corners, shadow, text, icon, padding, motion and the rest), the
widget's own values, the tokens and the colour roles. A connector hands them
to its toolkit. A preset is generated from the platform's verified facts
(`docs/facts/` in the repository), each generated value citing its fact, and
every layer is validated over the whole stack when it is added, so a resolved
theme has a value wherever the model requires one.

This is the 0.8.0 model while it is being built:

- Eight presets are bundled: `adwaita` (GNOME's libadwaita), `kde-breeze`
  (KDE's Breeze, Plasma 6), `macos-tahoe` (macOS 26), `macos-monterey`
  (macOS 12 to 15), `windows-11` (Windows 11, WinUI 3), `material-3`
  (Google's Material 3), `ios-26` (iOS 26) and `ios-13` (iOS 13 to 18). Each
  still lists fields whose facts are pending (its `[pending]` table), and
  `Theme::from_preset` refuses a preset with a pending field
  (`ThemeError::Preset`), so none builds a theme yet.
- The one typed widget is the button; the other widgets come unit by unit.
- The four OS readers are back, each with the `watch` feature's watcher:
  KDE's (the `kde` feature), which `Theme::from_system` reads on KDE Plasma
  over `kde-breeze`; GNOME's (the `portal` feature), on GNOME and Budgie over
  `adwaita`, with the portal-only read of the Linux desktops that have no
  reader of their own (Xfce, Cinnamon, MATE, LXQt, the Wayland compositors,
  COSMIC), also over `adwaita`; macOS's (the `macos` feature), over the
  running version's generation, `macos-tahoe` or `macos-monterey`; and
  Windows' (the `windows` feature), over `windows-11`. Each reads what
  [Feature flags](#feature-flags) says.

## How it fits

Most apps don't depend on this crate directly — they use a framework connector like
[`native-theme-gpui`](../connectors/native-theme-gpui/),
[`native-theme-iced`](../connectors/native-theme-iced/) or
[`native-theme-egui`](../connectors/native-theme-egui/), which pull `native-theme` in
transitively and re-export it. Depend on `native-theme` directly to write a new connector,
or to use the theme data without one.

## Quick start

Add the dependency:

```sh
cargo add native-theme
```

Build a theme from a bundled preset, resolve it for a window and ask for a
part's style. The connector states the window's conditions and measures text
with its toolkit's fonts (`FontMetrics`):

```rust,no_run
use native_theme::{
    ButtonPart, Direction, Element, FontMetrics, InputKind, PartRef, PresetId, Px, RootInputs,
    Scheme, Size, Styled, Theme, TypedPart, WindowConditions, WindowState,
};

fn draw_button(theme: &Theme, window: &mut WindowState, metrics: &dyn FontMetrics) {
    let resolved = theme
        .resolve(window, &WindowConditions::new(InputKind::Mouse))
        .theme;
    let size = Size { width: Px(1280.0), height: Px(720.0) };
    let scope = resolved.root_scope(&RootInputs::new(size, size, Direction::Ltr));
    let body = PartRef::Typed(TypedPart::Button(ButtonPart::Body));
    match resolved.style(&Element::new(body, &scope, metrics)) {
        Styled::Style((style, _children)) => {
            // Draw the bezel: style.fill, style.stroke, style.corners, style.padding, …
            let _ = style;
        }
        // Another widget draws the part: build an element of it instead.
        Styled::DrawnAs { .. } => {}
        // The part is not drawn, or the platform has no such widget or part.
        Styled::NotDrawn | Styled::NotAvailable(_) => {}
    }
}

let theme = Theme::from_preset(PresetId::Adwaita, Scheme::Light)?;
# let _ = (theme, draw_button);
# Ok::<(), native_theme::ThemeError>(())
```

An application changes the theme with layers of its own, in the presets'
TOML format or built in Rust. `add_layer` validates the layer over the whole
stack and refuses it whole, naming every problem, where it does not hold:

```rust,no_run
use native_theme::{Layer, PresetId, Scheme, Theme};

let mut theme = Theme::from_preset(PresetId::Adwaita, Scheme::Dark)?;
let layer = Layer::from_toml(
    r#"
    [[rules]]
    when = { states = ["hover"] }
    target = "button.body"
    set = { opacity = 0.5 }
    "#,
)?;
let id = theme.add_layer(layer)?;
theme.remove_layer(id)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Core concepts

- **`Theme`** — the one owner of the stack: the preset, the application's
  layers, the accent and the application's conditions. `from_preset(id,
  scheme)` builds one; `set_preset`, `pick` (the scheme), `set_flavor`,
  `add_layer`, `replace_layer`, `remove_layer`, `set_accent` and
  `set_contrast` change it. Every change validates the whole stack and is
  refused whole, the theme staying as it was.
- **`PresetId`** — a bundled preset: `PresetId::all()` lists them, `key()`
  names one as the TOML does (`adwaita`).
- **`Layer`** — an application's layer: bases and rules, each rule a
  selector (states, variants, sizes, the scheme, the window's conditions, …)
  and what it sets. `Layer::from_toml` reads the presets' format.
- **`Theme::resolve`** — the theme for one window: its `WindowState` records
  the last resolution, its `WindowConditions` what the connector states, and
  the `Resolution` says what changed since (`Change`).
- **`ResolvedTheme`** — what the connector queries: `style` of an
  `Element`'s part, with the scope its children take; `values` of a widget;
  `tokens`; `role` (a colour role); `size_classes`. A query answers
  `Styled::Style`, or why there is nothing to draw (`NotDrawn`, `DrawnAs`,
  `NotAvailable`).
- **The blocks** a style is made of (`Paint`, `Stroke`, `Corners`, `Shadow`,
  `TextStyle`, `IconStyle`, `Transition`, …) and their units (`Px`, `Ms`,
  `Deg`, `Fraction`); `Color` is extended sRGB, its literal forms `#rrggbb`,
  `#rrggbbaa`, `srgb(…)` and `display_p3(…)`.
- **What a connector computes itself**, at layout or per frame: `corners`,
  `shapes`, `transitions`, `timing`, `ripple` and `color::composite_over`.

## Icon sets

Semantic icon roles map to platform-appropriate glyphs via typed per-set
loaders. Each loader needs its set's feature and gives `None` without it:
`MaterialLoader` needs `material-icons` and `LucideLoader` `lucide-icons`;
the platform loaders need `system-icons` (which enables `material-icons` too)
and their platform: `FreedesktopLoader` Linux, `SfSymbolsLoader` macOS and
`SegoeFluentLoader` Windows.

```rust,no_run
use native_theme::icons::{FreedesktopLoader, IconRole, MaterialLoader};

// Bundled set (no system dependencies).
let copy = MaterialLoader::new(IconRole::ActionCopy).load();

// System icon theme on Linux: the one the desktop has active.
let sys = FreedesktopLoader::new(IconRole::ActionCopy).load();

// A named icon theme instead of the active one.
let breeze_dark = FreedesktopLoader::new(IconRole::ActionCopy)
    .theme("breeze-dark")
    .load();
```

## Feature flags

No feature is needed for the model and its bundled presets.

```toml
[dependencies]
native-theme = { version = "0.8", features = ["system-icons"] }
```

| Feature | Role |
|---|---|
| `kde` | The KDE Plasma reader, `Theme::from_system`: `kdeglobals` as KConfig cascades it, the colour scheme file it names and, on X11, `Xft.dpi`; and the KDE icon theme (Linux) |
| `macos` | The macOS reader, `Theme::from_system`: AppKit's appearances, colours and settings; the running macOS version and its generation, in `pipeline::diagnose_platform_support` |
| `windows` | The Windows reader, `Theme::from_system`: `UISettings`' colours, text scale and effects, high contrast and its system colours; the running Windows version and its generation, in `pipeline::diagnose_platform_support` |
| `portal` | The GNOME and Budgie reader, and the portal-only read of the Linux desktops with no reader of their own: the XDG Desktop Portal and gsettings (Linux) |
| `watch` | `watch::on_theme_change`, OS theme-change notifications: KDE (with `kde`), GNOME and the portal-only desktops (with `portal`), macOS (with `macos`) and Windows (with `windows`) |
| `linux-kde` / `linux-portal` | Aliases: `kde` and `portal` |
| `linux` | Meta-feature: `linux-kde` + `linux-portal` |
| `native` | Meta-feature: `linux` + `macos` + `windows` |
| `system-icons` | Platform icon lookups: freedesktop icon themes (Linux), SF Symbols (macOS), Segoe Fluent Icons and stock icons (Windows), each platform's dependencies on that platform only; also enables `material-icons` |
| `material-icons` / `lucide-icons` | Bundle those icon sets |
| `svg-rasterize` | Rasterise SVG icons to RGBA via resvg |
| `system-fonts` | The platform's own typeface for a family: `fonts::system_face` over the system font database (fontdb), and on macOS the system UI font's file through Core Text |

OS-specific dependencies are target-gated — `native` on macOS only pulls in
macOS-related deps.

The SF Symbols icons need macOS 11 or later: they load through
[`NSImage(systemSymbolName:accessibilityDescription:)`](https://developer.apple.com/documentation/appkit/nsimage/init(systemsymbolname:accessibilitydescription:)),
introduced in macOS 11.0.

The minimum supported Rust version is 1.88.0.

## Links

- [API reference on docs.rs](https://docs.rs/native-theme)
- Connectors: [`native-theme-gpui`](../connectors/native-theme-gpui/), [`native-theme-iced`](../connectors/native-theme-iced/), [`native-theme-egui`](../connectors/native-theme-egui/)
- [Showcase examples](../connectors/)
- [The bundled presets](src/presets/README.md): how each is generated from its facts and checked
- [CHANGELOG](../CHANGELOG.md)

## License

Licensed under any of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)
- [0BSD License](https://opensource.org/license/0bsd)

at your option.

Four files are not: `src/ripple.rs` ports the Android Open Source Project's
`RippleShader.java`, `RippleAnimationSession.java` and Compose's ripple and
colour code, `src/platform/android_non_linear.rs` its `FontScaleConverter`
(Android's non-linear font scaling) and Compose's line-height threshold, and
`src/platform/m3_role.rs` ports Google's material-color-utilities (its dynamic
scheme, tonal palettes and HCT colour space); these three stay under the
[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) with
their copyright notices, whichever licence you pick for the rest;
`src/platform/win_ui.rs` ports microsoft-ui-xaml's `GetScaledFontSize` (WinUI's
text-size curve) and stays under the MIT License with Microsoft's notice. The
crate's licence is therefore `(MIT OR Apache-2.0 OR 0BSD) AND Apache-2.0 AND MIT`;
the crate ships the licence (`LICENSE-APACHE`) and the ports' notices (`NOTICE`).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be triple licensed as above, without any additional terms or conditions.
