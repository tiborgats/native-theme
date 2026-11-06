# native-theme-derive

> **This is an internal implementation detail of [`native-theme`](../native-theme/).**
> You should not depend on this crate directly. Its only purpose is to provide
> the `#[derive(ThemeParts)]` derive macro (with its `#[parts]` and `#[part]`
> helper attributes) used by `native-theme`'s own model types. Add
> `native-theme` as your dependency instead — the macro is applied internally
> and does not appear in the public API surface you consume.

## What it does

`#[derive(ThemeParts)]` generates, from a block or a typed widget of
`native-theme`'s model:

- its patch type, every field optional, which a theme's layers merge
- its resolved type, every value the resolution decided
- the merge of one patch over another
- its field table, the paths the model's keys and checks name

The expansions name paths that exist only inside `native-theme`, so the macro
compiles nowhere else.

## Links

- [API reference on docs.rs](https://docs.rs/native-theme-derive) — for `native-theme` maintainers only
- [Main crate: `native-theme`](../native-theme/)

## License

Licensed under any of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)
- [0BSD License](https://opensource.org/license/0bsd)

at your option.
