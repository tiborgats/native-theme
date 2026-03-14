# native-theme-iced

[iced](https://iced.rs/) toolkit connector for
[native-theme](https://crates.io/crates/native-theme).

Maps `native_theme::NativeTheme` data to iced's theming system, producing a
fully configured `iced::Theme` with correct colors for all built-in widget
styles via iced's Catalog system.

## Usage

Add both crates to your `Cargo.toml`:

```toml
[dependencies]
native-theme = "0.3"
native-theme-iced = "0.3"
```

Then create an iced theme from any native-theme preset or OS theme:

```rust
use native_theme::NativeTheme;
use native_theme_iced::to_theme;

// Load a preset
let nt = NativeTheme::preset("dracula").unwrap();

// Pick light or dark variant (with cross-fallback)
let is_dark = true;
if let Some(variant) = nt.pick_variant(is_dark) {
    let theme = to_theme(variant, "My App");
    // Use `theme` as your iced application theme
}
```

Or read the OS theme at runtime:

```rust
use native_theme::{from_system, NativeTheme};
use native_theme_iced::to_theme;

let nt = from_system().unwrap_or_else(|_| NativeTheme::preset("default").unwrap());
let is_dark = true;
if let Some(variant) = nt.pick_variant(is_dark) {
    let theme = to_theme(variant, "System Theme");
}
```

## Widget Metrics

The crate exposes helper functions for widget sizing that iced applies
per-widget rather than through the Catalog:

- `button_padding(variant)` -- horizontal and vertical padding
- `input_padding(variant)` -- text input padding
- `border_radius(variant)` -- standard corner radius
- `border_radius_lg(variant)` -- large corner radius (e.g., dialogs)
- `scrollbar_width(variant)` -- scrollbar track width
- `font_family(variant)` -- primary UI font family name
- `font_size(variant)` -- primary UI font size in pixels (converted from points)
- `mono_font_family(variant)` -- monospace font family name
- `mono_font_size(variant)` -- monospace font size in pixels

## Modules

| Module | Purpose |
|--------|---------|
| `palette` | Maps native-theme colors to iced's 6-field Palette |
| `extended` | Overrides iced's Extended palette for secondary and background.weak |
| `icons` | Icon role mapping for iced SVG widgets |

## Example

Run the showcase widget gallery to explore all 17 presets interactively:

```sh
cargo run -p native-theme-iced --example showcase
```

The showcase displays all iced widgets (buttons, inputs, sliders, checkboxes,
togglers, etc.) themed with native-theme presets, with live theme switching
and a color map inspector.

## License

Licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)
- [0BSD License](https://opensource.org/license/0bsd)

at your option.
