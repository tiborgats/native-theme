# native-theme-derive

> **This is an internal implementation detail of [`native-theme`](../native-theme/).**
> You should not depend on this crate directly. Its only purpose is to provide
> the `#[derive(ThemeWidget)]` and `#[derive(ThemeFields)]` derive macros
> (with their `#[theme]`, `#[theme_layer]` and `#[theme_inherit]` helper
> attributes) used by `native-theme`'s own widget types. Add `native-theme` as your dependency
> instead — the macros are applied internally and do not appear in the public
> API surface you consume.

## What it does

`#[derive(ThemeWidget)]` generates, from a single annotated struct definition:

- A paired `Option` / `Resolved` struct hierarchy (sparse source vs. fully
  populated output)
- `merge()` logic for overlay composition
- Range and presence validation
- The struct's inheritance rules wired into the resolution pipeline: per-field
  rules from `#[theme(inherit_from = "...")]`, and border and font inheritance
  from the struct-level `#[theme_inherit(...)]` attribute

`#[derive(ThemeFields)]` registers a plain struct's serialized field names, so
`native-theme` can lint a theme TOML for unknown keys.

The expansions name paths that exist only inside `native-theme`
(`crate::resolve::*`), so the macros compile nowhere else.

## Links

- [API reference on docs.rs](https://docs.rs/native-theme-derive) — for `native-theme` maintainers only
- [Main crate: `native-theme`](../native-theme/)

## License

Licensed under any of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)
- [0BSD License](https://opensource.org/license/0bsd)

at your option.
