# native-theme-derive

Proc-macro crate providing `#[derive(ThemeWidget)]` for
[native-theme](https://crates.io/crates/native-theme).

Generates paired `Option`/`Resolved` struct hierarchies, merge logic,
validation, range checks, and field-level inheritance from a single
annotated struct definition.

This crate is an internal implementation detail of `native-theme` and is
not intended for direct use. Add `native-theme` as a dependency instead.

## License

MIT
