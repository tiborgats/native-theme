# native-theme-derive

Proc-macro crate providing `#[derive(ThemeWidget)]` for
[native-theme](https://crates.io/crates/native-theme).

Generates paired `Option`/`Resolved` struct hierarchies, merge logic,
validation, range checks, and field-level inheritance from a single
annotated struct definition.

This crate is an internal implementation detail of `native-theme` and is
not intended for direct use. Add `native-theme` as a dependency instead.

## License

Licensed under either of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)
- [0BSD License](https://opensource.org/license/0bsd)

at your option.
