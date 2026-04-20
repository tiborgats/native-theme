# native-theme-derive

> **This is an internal implementation detail of [`native-theme`](../native-theme/).**
> You should not depend on this crate directly. Its only purpose is to provide
> the `#[derive(ThemeWidget)]` and `#[theme_inherit]` proc-macros used by
> `native-theme`'s own widget types. Add `native-theme` as your dependency
> instead — the macros are applied internally and do not appear in the public
> API surface you consume.

## What it does

Generates, from a single annotated struct definition:

- A paired `Option` / `Resolved` struct hierarchy (sparse source vs. fully
  populated output)
- `merge()` logic for overlay composition
- Range and presence validation
- Field-level inheritance rules wired into the resolution pipeline

Also provides `#[theme_inherit]` — a codegen attribute for border/font
inheritance rules previously written by hand across the crate.

## Links

- [API reference on docs.rs](https://docs.rs/native-theme-derive) — for `native-theme` maintainers only
- [Main crate: `native-theme`](../native-theme/)

## License

Licensed under any of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)
- [0BSD License](https://opensource.org/license/0bsd)

at your option.
