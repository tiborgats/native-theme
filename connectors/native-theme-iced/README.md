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
use native_theme_iced::{pick_variant, to_theme};

// Load a preset
let nt = NativeTheme::preset("dracula").unwrap();

// Pick light or dark variant (with cross-fallback)
let is_dark = true;
if let Some(variant) = pick_variant(&nt, is_dark) {
    let theme = to_theme(variant, "My App");
    // Use `theme` as your iced application theme
}
```

## Widget Metrics

The crate also exposes helper functions for widget sizing that iced applies
per-widget rather than through the Catalog:

- `button_padding(variant)` -- horizontal and vertical padding
- `input_padding(variant)` -- text input padding
- `border_radius(variant)` -- standard corner radius
- `border_radius_lg(variant)` -- large corner radius (e.g., dialogs)
- `scrollbar_width(variant)` -- scrollbar track width

## License

Licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)

at your option.
