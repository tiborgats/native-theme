//! Derive macros internal to [`native-theme`](https://docs.rs/native-theme).
//!
//! **Do not depend on this crate directly; depend on `native-theme`.**
//!
//! [`ThemeParts`](derive@ThemeParts) generates the 0.8.0 model's blocks' and
//! typed widgets' patches, resolved types, merges and field tables; its
//! expansions name `crate::` paths (the blocks' traits and merge
//! helpers, the field-path registry) that exist only inside `native-theme`,
//! so it compiles nowhere else, and `native-theme` does not re-export it.

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod parts;
mod util;

/// Derive macro for the 0.8.0 model's blocks and typed widgets (S2, S5,
/// S6.1).
///
/// On a block marked `#[parts(block)]` it generates, beside the block:
///
/// - for a struct block, `<T>Patch`, every field an `Option` (`None`: the
///   patch does not set it), a field holding a struct block patched by
///   `BlockPatch` (`Set | Fields`) and one holding an append block by that
///   block's patch; with `#[parts(append)]`, a block whose one field is a list
///   (a paint), `<T>Patch` is instead `Set(Value<T>) | Append(list)`;
/// - `Resolved<T>`, each `Value<U>` as `Option<U>`, a `Value<FontSize>` as
///   `Option<Px>` (converted and scaled, S9.4), a `Value<Px>` or
///   `Value<FontSize>` marked `auto` as `Option<Length>`, and each block as
///   its resolved type;
/// - the merge of a later patch over an earlier one and the application of a
///   patch over a value (S6.1: a set field replaces, a `Vec` of pairs merges
///   by key, a list marked `keyed` by its items' own keys, any other list is
///   replaced whole, and a `Sides` or `Axes` field, patched as
///   `Option<Sides<Option<T>>>`, member by member; a `Vec` of pairs whose
///   values are `Vec`s of pairs merges by key twice); a map of struct blocks
///   (`Vec<(K, B)>`, `Vec<(K, Value<B>)>`, a `SurfaceMap<B>`) is patched
///   entry by entry, each entry by `BlockPatch<B>` (`Set` replaces it,
///   `Fields` applies over it), and a map of an append block's values by
///   that block's patch; an entry the map lacks is created from its patch;
/// - for a struct block, `Create`: the block built from its patch alone, for
///   a map entry the value lacks, each `Value` the patch leaves unset
///   `Absent` (left for S12's absent checks) and any other field it leaves
///   unset a `Missing` conflict, as no value is invented for it;
/// - the `Block` impl (the resolved type, sealed) and the `Table` impl: the
///   static `FIELDS` table of `FieldInfo { path, ty, may_be_absent, auto }`
///   and the `FORM`, a struct or an enum with its unit variants.
///
/// A nested block's patch and resolved types are named beside the block's
/// type, its path kept (`paint::Paint` -> `paint::PaintPatch`), so a block
/// written by its bare name needs them in scope.
///
/// An enum block is set whole only: it gets `Resolved<T>` and its table, the
/// paths of a variant's fields `variant.field` (a tuple variant's fields:
/// `variant`, several only of plain values).
///
/// On a widget's input marked `#[parts(widget)]`, `<Widget>Theme`, which
/// `scripts/generate_widgets.py` writes from the catalogue (one field per
/// part, `PartStyle`, and one `values` field, the widget's values struct, a
/// struct block), it generates `<Widget>Part`, one variant per part;
/// `<Widget>Patch`, a `PartPatch` per part and the values' patch, each an
/// `Option`, with `into_parts`, the patches in the catalogue's order; and the
/// `WidgetTable` impl: the parts' tables (`PartInfo`: the name, the root,
/// element and may-be-absent marks, the parent, and the `PartStyle` fields the
/// part lists, each with its marks, a field named in `absent` also marked may
/// be `Absent`), asserting at compile time that each listed field is one of
/// the style's, and the values' table.
///
/// # Type attributes
///
/// - `#[parts(block)]`: block mode.
/// - `#[parts(widget)]`: widget mode.
/// - `#[parts(append)]`: a struct block patched by `Set` or `Append`.
/// - `#[parts(own_resolved)]`: `Resolved<T>` is written by hand.
/// - `#[parts(resolved_patch)]`: a struct block's patch resolved,
///   `Resolved<T>Patch`, each field `Option<resolved field>` (`None`: the
///   patch leaves it, or the field beneath resolves `Absent`), for a field
///   holding the block's patch: each field the patch sets resolved over the
///   steps beneath it (a keyframe's over its part's style, S2.7), what is
///   absent beneath kept absent at any depth (`KeepAbsent`, which the derive
///   writes for every struct block's resolved type).
/// - `#[parts(added(name = "<field>", ty = "<type>", doc = "<doc>"))]`: a
///   field of a struct block's `Resolved<T>` that no patch sets and the
///   resolution adds (S6.1: `ResolvedPartStyle.breeze_tests`), resolved at its
///   default for the resolution to set; in no patch, table or registry.
///
/// # Field attributes
///
/// - `#[parts(may_be_absent)]`: the `Value` may be `Absent` (S6.3).
/// - `#[parts(auto)]`: the `Value<Px>` or `Value<FontSize>` may be `Auto`
///   (S6.3).
/// - `#[parts(block = "fields" | "append" | "set")]`: the field holds a
///   block (`B`, `Value<B>`, `Vec<B>`, `Option<B>`, a `SurfaceMap<B>`, or a
///   map `Vec<(K, B)>`, `Vec<(K, Value<B>)>` or `Vec<(K, Vec<B>)>`) of that
///   kind: a struct block, an append block or an enum block. An append block is held as `Value<B>` or a map's
///   `Value<B>`, and an `Option<B>` is `set` whole: an `append` elsewhere,
///   or `fields` on an `Option`, would act as `set`, so both are refused.
/// - `#[parts(keyed)]`: a list whose items merge by their own key, through
///   the item type's `KeyedItem` impl.
/// - `#[parts(inner_may_be_absent)]`, with `#[parts(required = "<field>")]`
///   for each field it leaves out: on a struct block's field, every `Value`
///   field of the block may be `Absent` in this place only (S6.3: an
///   icon-size context's `IconStyle` but its `size`), its table entry
///   holding the block's table so marked.
/// - `#[parts(patch)]`: the field holds a struct block's patch, `<B>Patch`
///   (a keyframe's `PartStylePatch`): it resolves to `Resolved<B>Patch`
///   (`B` marked `resolved_patch`) through `Cx::patch`, which reads the
///   holder's steps for the part it names, and its table entry reaches `B`'s table
///   through a function, as `B` can hold the field itself.
///
/// # Part attributes (widget mode)
///
/// - `#[part(root, element, may_be_absent, parent = "<part>", fields(...),
///   absent(...))]`: the root part (the first, never absent as a whole, never
///   an element, nesting in nothing); an *(element)* part; a part that may be
///   `Absent` as a whole (S6.3); the part it nests in; its fields, the
///   catalogue's list; and those of them the catalogue marks may be absent.
#[proc_macro_derive(ThemeParts, attributes(parts, part))]
pub fn derive_theme_parts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match parts::derive(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
