// Block mode (`#[parts(block)]`): a block's patch, its resolved type, the
// merge of its patches (S6.1) and its field table.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Attribute, Data, DeriveInput, Error, Fields, GenericArgument, Ident, PathArguments, Result,
    Type, Visibility,
};

use super::attrs::{self, BlockKind, FieldAttrs, TypeAttrs};

/// The module the expansions name the model's types by.
fn model() -> TokenStream {
    quote!(crate::next)
}

/// The module the expansions name the blocks' traits and merge helpers by.
fn blocks() -> TokenStream {
    quote!(crate::next::blocks)
}

/// The module the expansions name the field-path registry's types by.
fn registry() -> TokenStream {
    quote!(crate::next::registry)
}

/// `Option`, named so that no item at the derive site shadows it.
fn option() -> TokenStream {
    quote!(::core::option::Option)
}

/// Where a block field's block sits in its type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Wrap {
    /// `B`.
    Bare,
    /// `Value<B>`.
    Value,
    /// `Vec<B>`.
    List,
    /// `Option<B>`.
    Option,
}

/// A block a field holds: its type as written, and the type's name.
pub(super) struct BlockRef {
    pub(super) ty: Box<Type>,
    pub(super) name: Ident,
}

/// A map's values' block: the block, how it is patched, where it sits in the
/// map's value, and the map's key type.
pub(super) struct MapBlock {
    pub(super) block: BlockRef,
    pub(super) kind: BlockKind,
    pub(super) wrap: Wrap,
    pub(super) key: Box<Type>,
}

impl MapBlock {
    /// The patch of one entry, where the map's entries are patched one by one
    /// (S6.1): a struct block's `BlockPatch<B>`, held bare or as a `Value`,
    /// or an append block's own patch; `None` where an entry is set whole (an
    /// enum block, a list of blocks).
    pub(super) fn entry_patch(&self) -> Option<TokenStream> {
        let b = blocks();
        let ty = &self.block.ty;
        match (self.kind, self.wrap) {
            (BlockKind::Fields, Wrap::Bare | Wrap::Value) => Some(quote!(#b::BlockPatch<#ty>)),
            (BlockKind::Append, Wrap::Value) => {
                let patch = renamed(ty, &format_ident!("{}Patch", self.block.name));
                Some(quote!(#patch))
            }
            _ => None,
        }
    }
}

/// What a field holds, which decides its patch, its merge and its resolved
/// type.
pub(super) enum Shape {
    /// A value with no fields of its own.
    Leaf,
    /// A `Vec` of pairs: merged by key; its values a block's where the field
    /// is marked `block`, each entry then patched by its block's patch.
    Map(Option<MapBlock>),
    /// A `Vec` of pairs whose values are `Vec`s of pairs: merged by key, an
    /// entry held on both sides merged by key in turn (S6.1: every `Vec` of
    /// pairs).
    NestedMap,
    /// A `SurfaceMap` of a struct block: patched per surface set by the
    /// block's patch, as a struct block's `Fields` patch.
    Surfaces(BlockRef),
    /// A block of the model.
    Block {
        kind: BlockKind,
        wrap: Wrap,
        block: BlockRef,
    },
    /// A compound value (`Sides<T>`, `Axes<T>`), patched member by member
    /// (S6.1).
    Compound(Compound),
}

/// A compound value of the model: its type's name, its members, and the type
/// each member holds.
pub(super) struct Compound {
    pub(super) name: &'static str,
    pub(super) members: &'static [&'static str],
    pub(super) inner: Box<Type>,
}

/// The model's compound values and their members, in declaration order.
const COMPOUNDS: &[(&str, &[&str])] = &[
    ("Sides", &["top", "right", "bottom", "left"]),
    ("Axes", &["horizontal", "vertical"]),
];

/// The compound value `ty` is, if any.
fn compound(ty: &Type) -> Option<Compound> {
    COMPOUNDS.iter().find_map(|(name, members)| {
        generic_arg(ty, name).map(|inner| Compound {
            name,
            members,
            inner: Box::new(inner.clone()),
        })
    })
}

pub(super) struct BlockField {
    /// `None` for a tuple variant's one field.
    pub(super) ident: Option<Ident>,
    vis: Visibility,
    pub(super) ty: Type,
    docs: Vec<Attribute>,
    pub(super) attrs: FieldAttrs,
    pub(super) shape: Shape,
}

/// How a later patch's field merges over the earlier one's.
enum MergeKind {
    Replace,
    ByKey,
    Entries,
    ByKeyNested,
    Surfaces,
    Keyed,
    Members,
    Nested,
}

impl BlockField {
    fn merge_kind(&self) -> MergeKind {
        if self.attrs.keyed {
            return MergeKind::Keyed;
        }
        match &self.shape {
            Shape::Compound(_) => MergeKind::Members,
            Shape::Leaf => MergeKind::Replace,
            Shape::Map(Some(map)) if map.entry_patch().is_some() => MergeKind::Entries,
            Shape::Map(_) => MergeKind::ByKey,
            Shape::NestedMap => MergeKind::ByKeyNested,
            Shape::Surfaces(_) => MergeKind::Surfaces,
            Shape::Block {
                kind: BlockKind::Fields,
                wrap: Wrap::Bare | Wrap::Value,
                ..
            }
            | Shape::Block {
                kind: BlockKind::Append,
                wrap: Wrap::Value,
                ..
            } => MergeKind::Nested,
            Shape::Block { .. } => MergeKind::Replace,
        }
    }

    /// A field holding no `Value`, no block and no mark: its resolved type is
    /// its own.
    fn is_plain_leaf(&self) -> bool {
        matches!(self.shape, Shape::Leaf)
            && !holds_value(&self.ty, None)
            && !self.attrs.may_be_absent
            && !self.attrs.auto
            && !self.attrs.keyed
    }

    fn patch_ty(&self) -> TokenStream {
        let o = option();
        let inner = self.patch_inner();
        quote!(#o<#inner>)
    }

    /// The type a patch holds for the field, inside its `Option`.
    pub(super) fn patch_inner(&self) -> TokenStream {
        let b = blocks();
        let o = option();
        match &self.shape {
            Shape::Block {
                kind: BlockKind::Fields,
                wrap: Wrap::Bare,
                block,
            }
            | Shape::Block {
                kind: BlockKind::Append,
                wrap: Wrap::Value,
                block,
            } => {
                let patch = renamed(&block.ty, &format_ident!("{}Patch", block.name));
                quote!(#patch)
            }
            Shape::Block {
                kind: BlockKind::Fields,
                wrap: Wrap::Value,
                block,
            } => {
                let inner = &block.ty;
                quote!(#b::BlockPatch<#inner>)
            }
            Shape::Compound(compound) => {
                let m = model();
                let name = format_ident!("{}", compound.name);
                let inner = &compound.inner;
                quote!(#m::#name<#o<#inner>>)
            }
            Shape::Surfaces(block) => {
                let m = model();
                let inner = &block.ty;
                quote!(#m::SurfaceMap<#b::BlockPatch<#inner>>)
            }
            Shape::Map(Some(map)) if map.entry_patch().is_some() => {
                let key = &map.key;
                let entry = map.entry_patch();
                quote!(::std::vec::Vec<(#key, #entry)>)
            }
            Shape::Leaf | Shape::Map(_) | Shape::NestedMap | Shape::Block { .. } => {
                let ty = &self.ty;
                quote!(#ty)
            }
        }
    }

    /// A compound field's type, as the model names it, and its members.
    fn members(&self) -> (TokenStream, Vec<Ident>) {
        match &self.shape {
            Shape::Compound(compound) => {
                let m = model();
                let name = format_ident!("{}", compound.name);
                let members = compound
                    .members
                    .iter()
                    .map(|member| format_ident!("{member}"))
                    .collect();
                (quote!(#m::#name), members)
            }
            Shape::Leaf
            | Shape::Map(_)
            | Shape::NestedMap
            | Shape::Surfaces(_)
            | Shape::Block { .. } => (TokenStream::new(), Vec::new()),
        }
    }

    fn block(&self) -> Option<&BlockRef> {
        match &self.shape {
            Shape::Block { block, .. } | Shape::Surfaces(block) => Some(block),
            Shape::Map(Some(map)) => Some(&map.block),
            Shape::Leaf | Shape::Map(None) | Shape::NestedMap | Shape::Compound(_) => None,
        }
    }

    fn resolved_ty(&self) -> TokenStream {
        if self.attrs.patch
            && let Some(ident) = plain_ident(&self.ty)
        {
            let resolved = renamed(&self.ty, &format_ident!("Resolved{ident}"));
            return quote!(#resolved);
        }
        resolved_ty(&self.ty, self.block(), self.attrs.auto)
    }

    /// The field's value in a block built from its patch alone (`Create`),
    /// the patch's field bound to the field's name and the place's marks to
    /// `inner`: a `Value` left unset is `Absent` where it may be (its own
    /// mark, or the place's, S6.3), left for S12's conditional checks; any
    /// other field left unset is a `Missing` conflict, as no value is
    /// invented for it (S6.1).
    fn created(&self) -> TokenStream {
        let b = blocks();
        let m = model();
        let o = option();
        let ident = &self.ident;
        let path = ident.as_ref().map(Ident::to_string).unwrap_or_default();
        let marked = self.attrs.may_be_absent;
        let missing = quote! {
            #b::MergeConflict::new(#b::Below::Missing).under(#path)
        };
        let required = quote!(#ident.ok_or_else(|| #missing)?);
        let unset_value = quote!(#b::created_value(#o::None, #marked, inner, #path)?);
        match &self.shape {
            Shape::Leaf if generic_arg(&self.ty, "Value").is_some() && !self.attrs.keyed => {
                quote!(#b::created_value(#ident, #marked, inner, #path)?)
            }
            Shape::Compound(compound) => {
                let (name, members) = self.members();
                if generic_arg(&compound.inner, "Value").is_some() {
                    let paths: Vec<String> = members
                        .iter()
                        .map(|member| format!("{path}.{member}"))
                        .collect();
                    quote! {{
                        let held = #ident.unwrap_or(#name {
                            #(#members: #o::None),*
                        });
                        #name {
                            #(#members: #b::created_value(
                                held.#members,
                                #marked,
                                inner,
                                #paths,
                            )?),*
                        }
                    }}
                } else {
                    let paths = members.iter().map(|member| format!("{path}.{member}"));
                    quote! {{
                        let held = #required;
                        #name {
                            #(#members: held.#members.ok_or_else(|| {
                                #b::MergeConflict::new(#b::Below::Missing).under(#paths)
                            })?),*
                        }
                    }}
                }
            }
            Shape::Block {
                kind: BlockKind::Fields,
                wrap: Wrap::Bare,
                block,
            } => {
                let ty = &block.ty;
                let place = if self.attrs.inner_may_be_absent {
                    let required = &self.attrs.required;
                    quote!(#o::Some(&[#(#required),*]))
                } else {
                    quote!(#o::None)
                };
                quote! {
                    <#ty as #b::Create>::create_in(#ident.unwrap_or_default(), #place)
                        .map_err(|c| c.under(#path))?
                }
            }
            Shape::Block {
                kind: kind @ (BlockKind::Fields | BlockKind::Append),
                wrap: Wrap::Value,
                block,
            } => {
                let ty = &block.ty;
                let patch = if *kind == BlockKind::Fields {
                    quote!(#b::BlockPatch<#ty>)
                } else {
                    let patch = renamed(ty, &format_ident!("{}Patch", block.name));
                    quote!(#patch)
                };
                quote! {
                    match #ident {
                        #o::Some(patch) => <#m::Value<#ty> as #b::Entry<#patch>>::create(patch)
                            .map_err(|c| c.under(#path))?,
                        #o::None => #unset_value,
                    }
                }
            }
            Shape::Block {
                kind: BlockKind::Set,
                wrap: Wrap::Value,
                ..
            } => quote!(#b::created_value(#ident, #marked, inner, #path)?),
            Shape::Surfaces(_) => quote! {{
                let mut held = #m::SurfaceMap::default();
                #b::apply_surfaces(&mut held, #required).map_err(|c| c.under(#path))?;
                held
            }},
            Shape::Map(Some(map)) if map.entry_patch().is_some() => quote! {{
                let mut held = ::std::vec::Vec::new();
                #b::apply_entries(&mut held, #required).map_err(|c| c.under(#path))?;
                held
            }},
            _ if self.attrs.keyed => quote! {{
                let mut held = ::std::vec::Vec::new();
                #b::KeyedItem::merge_keyed(&mut held, #required);
                held
            }},
            Shape::Leaf | Shape::Map(_) | Shape::NestedMap | Shape::Block { .. } => required,
        }
    }

    fn field_info(&self, path: &str) -> TokenStream {
        let r = registry();
        let b = blocks();
        let written = type_string(&self.ty);
        let may_be_absent = self.attrs.may_be_absent;
        let auto = self.attrs.auto;
        let whole = !matches!(self.shape, Shape::Compound(_));
        let patched = self
            .attrs
            .patch
            .then(|| plain_ident(&self.ty))
            .flatten()
            .and_then(|ident| ident.to_string().strip_suffix("Patch").map(str::to_owned));
        let ty = match (self.block(), &self.shape, patched) {
            (_, _, Some(block)) => {
                let block = renamed(&self.ty, &format_ident!("{block}"));
                quote! {
                    #r::FieldType::Patch {
                        ty: #written,
                        fields: {
                            fn fields() -> &'static [#r::FieldInfo] {
                                <#block as #b::Table>::FIELDS
                            }
                            fields
                        },
                    }
                }
            }
            (_, Shape::Compound(compound), None) => {
                let inner = type_string(&compound.inner);
                let members = compound.members.iter().map(|member| {
                    quote! {
                        #r::FieldInfo {
                            path: #member,
                            ty: #r::FieldType::Leaf(#inner),
                            may_be_absent: #may_be_absent,
                            auto: #auto,
                        }
                    }
                });
                quote! {
                    #r::FieldType::Block {
                        ty: #written,
                        form: #r::BlockForm::Struct,
                        fields: &[#(#members),*],
                    }
                }
            }
            (Some(block), _, None) => {
                let inner = &block.ty;
                let fields = if self.attrs.inner_may_be_absent {
                    let required = &self.attrs.required;
                    let message = format!(
                        "required names a field of `{}` that is not a `Value`",
                        block.name
                    );
                    quote! {{
                        const _: () = ::core::assert!(
                            #r::names_values(<#inner as #b::Table>::FIELDS, &[#(#required),*]),
                            #message
                        );
                        const LEN: usize = <#inner as #b::Table>::FIELDS.len();
                        const MARKED: [#r::FieldInfo; LEN] =
                            #r::absent_but(<#inner as #b::Table>::FIELDS, &[#(#required),*]);
                        &MARKED
                    }}
                } else {
                    quote!(<#inner as #b::Table>::FIELDS)
                };
                quote! {
                    #r::FieldType::Block {
                        ty: #written,
                        form: <#inner as #b::Table>::FORM,
                        fields: #fields,
                    }
                }
            }
            (None, _, None) => quote!(#r::FieldType::Leaf(#written)),
        };
        // A compound value is never absent or `Auto` as a whole: its marks are
        // its members' (S6.3).
        let (may_be_absent, auto) = (may_be_absent && whole, auto && whole);
        quote! {
            #r::FieldInfo { path: #path, ty: #ty, may_be_absent: #may_be_absent, auto: #auto }
        }
    }
}

/// The type `ty` names, its last segment renamed to `name`, so the path to
/// it is kept: `paint::Paint` -> `paint::PaintPatch`.
pub(super) fn renamed(ty: &Type, name: &Ident) -> Type {
    let mut out = ty.clone();
    if let Type::Path(type_path) = &mut out
        && let Some(seg) = type_path.path.segments.last_mut()
    {
        seg.ident = name.clone();
    }
    out
}

/// The pair a `Vec` of pairs holds: `(K, V)` of `Vec<(K, V)>`.
pub(super) fn map_pair(ty: &Type) -> Option<(&Type, &Type)> {
    match generic_arg(ty, "Vec") {
        Some(Type::Tuple(tuple)) => match tuple.elems.iter().collect::<Vec<_>>().as_slice() {
            [key, value] => Some((*key, *value)),
            _ => None,
        },
        _ => None,
    }
}

/// `T` of `Wrapper<T>`, where the type's last segment is `wrapper` with one
/// type argument.
pub(super) fn generic_arg<'a>(ty: &'a Type, wrapper: &str) -> Option<&'a Type> {
    if let Type::Path(type_path) = ty
        && type_path.qself.is_none()
        && let Some(seg) = type_path.path.segments.last()
        && seg.ident == wrapper
        && let PathArguments::AngleBracketed(args) = &seg.arguments
        && args.args.len() == 1
        && let Some(GenericArgument::Type(inner)) = args.args.first()
    {
        return Some(inner);
    }
    None
}

/// The last segment of a plain path type with no arguments.
fn plain_ident(ty: &Type) -> Option<&Ident> {
    if let Type::Path(type_path) = ty
        && type_path.qself.is_none()
        && let Some(seg) = type_path.path.segments.last()
        && seg.arguments.is_empty()
    {
        return Some(&seg.ident);
    }
    None
}

/// The field's value is a `Value`, or an `Option` of one, holding one of
/// `inner` when given.
fn holds_value(ty: &Type, inner: Option<&[&str]>) -> bool {
    let ty = generic_arg(ty, "Option").unwrap_or(ty);
    generic_arg(ty, "Value").is_some_and(|held| match inner {
        Some(names) => plain_ident(held).is_some_and(|ident| names.iter().any(|n| ident == n)),
        None => true,
    })
}

/// The units a value marked `auto` may hold: a length, and a font size (a
/// line height), both resolving to a `Length` (S6.1).
const AUTO_UNITS: &[&str] = &["Px", "FontSize"];

/// The type as written, without whitespace: `Value<Px>`.
fn type_string(ty: &Type) -> String {
    quote!(#ty).to_string().split_whitespace().collect()
}

/// The resolved form of a type: each `Value<T>` to `Option<T>`, an
/// `Option<Value<T>>` to `Option<T>` too, its `None` keeping the field's
/// meaning (S6.1), a
/// `Value<FontSize>` to `Option<Px>` (converted from points and scaled,
/// S9.4), a `Value<Px>` or `Value<FontSize>` that may be `Auto` to
/// `Option<Length>` (S6.1), the block `block` to its resolved type, through
/// `Option`, `Vec`, tuples, `Sides` and `Axes`.
fn resolved_ty(ty: &Type, block: Option<&BlockRef>, auto: bool) -> TokenStream {
    let o = option();
    let m = model();
    if let Some(inner) = generic_arg(ty, "Value") {
        if auto && holds_value(ty, Some(AUTO_UNITS)) {
            return quote!(#o<#m::Length>);
        }
        if holds_value(ty, Some(&["FontSize"])) {
            return quote!(#o<#m::Px>);
        }
        let inner = resolved_ty(inner, block, auto);
        return quote!(#o<#inner>);
    }
    if let Some(compound) = compound(ty) {
        let name = format_ident!("{}", compound.name);
        let inner = resolved_ty(&compound.inner, block, auto);
        return quote!(#m::#name<#inner>);
    }
    if let Some(inner) = generic_arg(ty, "SurfaceMap") {
        let inner = resolved_ty(inner, block, auto);
        return quote!(#m::SurfaceMap<#inner>);
    }
    if let Some(inner) = generic_arg(ty, "Option") {
        if generic_arg(inner, "Value").is_some() {
            return resolved_ty(inner, block, auto);
        }
        let inner = resolved_ty(inner, block, auto);
        return quote!(#o<#inner>);
    }
    if let Some(inner) = generic_arg(ty, "Vec") {
        let inner = resolved_ty(inner, block, auto);
        return quote!(::std::vec::Vec<#inner>);
    }
    if let Type::Tuple(tuple) = ty {
        let elems: Vec<_> = tuple
            .elems
            .iter()
            .map(|e| resolved_ty(e, block, auto))
            .collect();
        return match elems.as_slice() {
            [one] => quote!((#one,)),
            _ => quote!((#(#elems),*)),
        };
    }
    if let (Some(block), Some(ident)) = (block, plain_ident(ty))
        && *ident == block.name
    {
        let resolved = renamed(ty, &format_ident!("Resolved{ident}"));
        return quote!(#resolved);
    }
    quote!(#ty)
}

/// The block a block field's type names: `B` of `B`, `Value<B>`, `Vec<B>`,
/// `Option<B>`, `SurfaceMap<B>`, or of a map's `B`, `Value<B>` or `Vec<B>`.
fn block_ref(field: &syn::Field, inner: &Type) -> Result<BlockRef> {
    let name = plain_ident(inner).cloned().ok_or_else(|| {
        Error::new_spanned(
            &field.ty,
            "a block field's type is `B`, `Value<B>`, `Vec<B>`, `Option<B>`, `SurfaceMap<B>` or a \
             map of `B`, `Value<B>` or `Vec<B>`, `B` a block",
        )
    })?;
    Ok(BlockRef {
        ty: Box::new(inner.clone()),
        name,
    })
}

pub(super) fn parse_field(field: &syn::Field) -> Result<BlockField> {
    let attrs = attrs::parse_field_attrs(&field.attrs)?;
    let ty = field.ty.clone();
    let compound = compound(&ty);
    let held = compound.as_ref().map_or(&ty, |c| &*c.inner);
    if attrs.keyed && generic_arg(&ty, "Vec").is_none() {
        return Err(Error::new_spanned(&field.ty, "keyed marks a `Vec`"));
    }
    if (attrs.may_be_absent || attrs.auto)
        && generic_arg(held, "Option").is_some_and(|inner| generic_arg(inner, "Value").is_some())
    {
        return Err(Error::new_spanned(
            &field.ty,
            "an `Option<Value<T>>` resolves to `Option<T>`, its `None` a meaning of its own: \
             its `Value` is never `Absent` or `Auto`",
        ));
    }
    if attrs.may_be_absent && !holds_value(held, None) {
        return Err(Error::new_spanned(
            &field.ty,
            "may_be_absent marks a field holding a `Value`",
        ));
    }
    if attrs.auto && !holds_value(held, Some(AUTO_UNITS)) {
        return Err(Error::new_spanned(
            &field.ty,
            "auto marks a field holding a `Value<Px>` or a `Value<FontSize>`",
        ));
    }
    if attrs.patch
        && (attrs.block.is_some()
            || !plain_ident(&ty).is_some_and(|ident| ident.to_string().ends_with("Patch")))
    {
        return Err(Error::new_spanned(
            &field.ty,
            "patch marks a field holding a struct block's patch, `<B>Patch`",
        ));
    }
    if !attrs.required.is_empty() && !attrs.inner_may_be_absent {
        return Err(Error::new_spanned(
            &field.ty,
            "required names the fields an `inner_may_be_absent` mark leaves out",
        ));
    }
    if attrs.inner_may_be_absent && attrs.block != Some(BlockKind::Fields) {
        return Err(Error::new_spanned(
            &field.ty,
            "inner_may_be_absent marks a field holding a struct block, `block = \"fields\"`",
        ));
    }
    let shape = if let Some(compound) = compound {
        if attrs.block.is_some() {
            return Err(Error::new_spanned(
                &field.ty,
                "a compound value's members hold no block",
            ));
        }
        Shape::Compound(compound)
    } else {
        match (attrs.block, map_pair(&ty)) {
            (Some(kind), Some((key, value))) => {
                let (wrap, inner) = if let Some(inner) = generic_arg(value, "Value") {
                    (Wrap::Value, inner)
                } else if let Some(inner) = generic_arg(value, "Vec") {
                    (Wrap::List, inner)
                } else {
                    (Wrap::Bare, value)
                };
                if kind == BlockKind::Append && wrap != Wrap::Value {
                    return Err(Error::new_spanned(
                        &field.ty,
                        "an append block is held as `Value<B>`, or as a map's `Value<B>`",
                    ));
                }
                Shape::Map(Some(MapBlock {
                    block: block_ref(field, inner)?,
                    kind,
                    wrap,
                    key: Box::new(key.clone()),
                }))
            }
            (Some(BlockKind::Fields), None) if generic_arg(&ty, "SurfaceMap").is_some() => {
                let inner = generic_arg(&ty, "SurfaceMap").unwrap_or(&ty);
                Shape::Surfaces(block_ref(field, inner)?)
            }
            (Some(kind), None) => {
                let (wrap, inner) = if let Some(inner) = generic_arg(&ty, "Value") {
                    (Wrap::Value, inner)
                } else if let Some(inner) = generic_arg(&ty, "Vec") {
                    (Wrap::List, inner)
                } else if let Some(inner) = generic_arg(&ty, "Option") {
                    (Wrap::Option, inner)
                } else {
                    (Wrap::Bare, &ty)
                };
                let block = block_ref(field, inner)?;
                match (kind, wrap) {
                    (BlockKind::Append, Wrap::Bare | Wrap::List | Wrap::Option) => {
                        return Err(Error::new_spanned(
                            &field.ty,
                            "an append block is held as `Value<B>`, or as a map's `Value<B>`",
                        ));
                    }
                    (BlockKind::Fields, Wrap::Option) => {
                        return Err(Error::new_spanned(
                            &field.ty,
                            "an `Option<B>` is set whole: mark it `block = \"set\"`",
                        ));
                    }
                    _ => {}
                }
                Shape::Block { kind, wrap, block }
            }
            (None, Some((_, value))) if map_pair(value).is_some() => Shape::NestedMap,
            (None, Some(_)) => Shape::Map(None),
            (None, None) => Shape::Leaf,
        }
    };
    Ok(BlockField {
        ident: field.ident.clone(),
        vis: field.vis.clone(),
        ty,
        docs: docs(&field.attrs),
        attrs,
        shape,
    })
}

fn docs(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .filter(|a| a.path().is_ident("doc"))
        .cloned()
        .collect()
}

/// `FollowPart` -> `follow_part`.
pub(super) fn snake_case(name: &str) -> String {
    let mut out = String::with_capacity(name.len().saturating_mul(2));
    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    out
}

fn named_fields(fields: &Fields, owner: &Ident) -> Result<Vec<BlockField>> {
    match fields {
        Fields::Named(named) => named.named.iter().map(parse_field).collect(),
        Fields::Unnamed(_) | Fields::Unit => {
            Err(Error::new_spanned(owner, "a struct block has named fields"))
        }
    }
}

pub(crate) fn expand(input: &DeriveInput, type_attrs: &TypeAttrs) -> Result<TokenStream> {
    if !input.generics.params.is_empty() {
        return Err(Error::new_spanned(
            &input.generics,
            "a block has no generic parameters",
        ));
    }
    match &input.data {
        Data::Struct(data) => {
            let fields = named_fields(&data.fields, &input.ident)?;
            if type_attrs.append && type_attrs.resolved_patch {
                return Err(Error::new_spanned(
                    &input.ident,
                    "resolved_patch is a struct block's, patched by fields",
                ));
            }
            if type_attrs.append {
                expand_append(input, type_attrs, &fields)
            } else {
                Ok(expand_fields(input, type_attrs, &fields))
            }
        }
        Data::Enum(data) => {
            if type_attrs.append {
                return Err(Error::new_spanned(
                    &input.ident,
                    "append is a struct block's mode",
                ));
            }
            if type_attrs.resolved_patch {
                return Err(Error::new_spanned(
                    &input.ident,
                    "resolved_patch is a struct block's, patched by fields",
                ));
            }
            expand_enum(input, type_attrs, data)
        }
        Data::Union(_) => Err(Error::new_spanned(
            &input.ident,
            "ThemeParts derives on structs and enums",
        )),
    }
}

fn resolved_doc(name: &Ident) -> String {
    format!("[`{name}`] resolved (S6.1): each `Value` evaluated, `None` where not drawn.")
}

/// The resolved struct, unless written by hand, and the `Block` and `Table`
/// impls.
fn expand_struct_common(
    input: &DeriveInput,
    type_attrs: &TypeAttrs,
    fields: &[BlockField],
) -> TokenStream {
    let name = &input.ident;
    let vis = &input.vis;
    let b = blocks();
    let r = registry();
    let form = quote!(#r::BlockForm::Struct);
    let resolved = format_ident!("Resolved{name}");
    let resolved_struct = if type_attrs.own_resolved {
        TokenStream::new()
    } else {
        let doc = resolved_doc(name);
        let members = fields.iter().map(|f| {
            let BlockField {
                ident, vis, docs, ..
            } = f;
            let ty = f.resolved_ty();
            quote!(#(#docs)* #vis #ident: #ty,)
        });
        quote! {
            #[doc = #doc]
            #[derive(Clone, Debug, PartialEq)]
            #[non_exhaustive]
            #vis struct #resolved { #(#members)* }
        }
    };
    let infos = fields.iter().map(|f| {
        let path = f.ident.as_ref().map(Ident::to_string).unwrap_or_default();
        f.field_info(&path)
    });
    quote! {
        #resolved_struct

        impl #b::sealed::Sealed for #name {}

        impl #b::Block for #name {
            type Resolved = #resolved;
        }

        impl #b::Table for #name {
            const FIELDS: &'static [#r::FieldInfo] = &[#(#infos),*];
            const FORM: #r::BlockForm = #form;
        }
    }
}

fn expand_fields(
    input: &DeriveInput,
    type_attrs: &TypeAttrs,
    fields: &[BlockField],
) -> TokenStream {
    let name = &input.ident;
    let vis = &input.vis;
    let b = blocks();
    let patch = format_ident!("{name}Patch");
    let common = expand_struct_common(input, type_attrs, fields);
    let doc =
        format!("[`{name}`]'s patch (S6.1): each field `None` where the patch leaves it as it is.");
    let members = fields.iter().map(|f| {
        let BlockField {
            ident, vis, docs, ..
        } = f;
        let ty = f.patch_ty();
        quote!(#(#docs)* #vis #ident: #ty,)
    });
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let created = fields.iter().map(BlockField::created);
    let read = super::read::struct_impls(name, fields, type_attrs.own_toml);
    let walk = super::walk::struct_walk(
        name,
        name,
        fields,
        false,
        &super::walk::Expr::ReferenceOnly,
        type_attrs.check,
    );
    let walk_patch = super::walk::struct_walk(
        name,
        &patch,
        fields,
        true,
        &super::walk::Expr::ReferenceOnly,
        type_attrs.check,
    );
    let merges = fields.iter().map(|f| {
        let ident = &f.ident;
        let path = ident.as_ref().map(Ident::to_string).unwrap_or_default();
        match f.merge_kind() {
            MergeKind::Replace => quote!(#b::replace_slot(&mut self.#ident, #ident);),
            MergeKind::ByKey => quote!(#b::merge_slot_by_key(&mut self.#ident, #ident);),
            MergeKind::Entries => quote! {
                #b::merge_slot_entries(&mut self.#ident, #ident).map_err(|c| c.under(#path))?;
            },
            MergeKind::ByKeyNested => {
                quote!(#b::merge_slot_by_key_nested(&mut self.#ident, #ident);)
            }
            MergeKind::Surfaces => quote! {
                #b::merge_slot_surfaces(&mut self.#ident, #ident).map_err(|c| c.under(#path))?;
            },
            MergeKind::Keyed => quote!(#b::merge_slot_keyed(&mut self.#ident, #ident);),
            MergeKind::Members => {
                let (name, members) = f.members();
                let none = members
                    .iter()
                    .map(|m| quote!(#m: ::core::option::Option::None));
                quote! {
                    if let ::core::option::Option::Some(later) = #ident {
                        let held = self.#ident.get_or_insert_with(|| #name { #(#none),* });
                        #(#b::replace_slot(&mut held.#members, later.#members);)*
                    }
                }
            }
            MergeKind::Nested => quote! {
                #b::merge_slot(&mut self.#ident, #ident).map_err(|c| c.under(#path))?;
            },
        }
    });
    let applies = fields.iter().map(|f| {
        let ident = &f.ident;
        let path = ident.as_ref().map(Ident::to_string).unwrap_or_default();
        match f.merge_kind() {
            MergeKind::Replace => quote! {
                if let ::core::option::Option::Some(value) = #ident { self.#ident = value; }
            },
            MergeKind::ByKey => quote! {
                if let ::core::option::Option::Some(entries) = #ident { #b::merge_by_key(&mut self.#ident, entries); }
            },
            MergeKind::Entries => quote! {
                if let ::core::option::Option::Some(entries) = #ident {
                    #b::apply_entries(&mut self.#ident, entries).map_err(|c| c.under(#path))?;
                }
            },
            MergeKind::ByKeyNested => quote! {
                if let ::core::option::Option::Some(entries) = #ident {
                    #b::merge_by_key_nested(&mut self.#ident, entries);
                }
            },
            MergeKind::Surfaces => quote! {
                if let ::core::option::Option::Some(sets) = #ident {
                    #b::apply_surfaces(&mut self.#ident, sets).map_err(|c| c.under(#path))?;
                }
            },
            MergeKind::Keyed => quote! {
                if let ::core::option::Option::Some(items) = #ident {
                    #b::KeyedItem::merge_keyed(&mut self.#ident, items);
                }
            },
            MergeKind::Members => {
                let (_, members) = f.members();
                quote! {
                    if let ::core::option::Option::Some(later) = #ident {
                        #(
                            if let ::core::option::Option::Some(value) = later.#members {
                                self.#ident.#members = value;
                            }
                        )*
                    }
                }
            }
            MergeKind::Nested => quote! {
                if let ::core::option::Option::Some(field_patch) = #ident {
                    #b::Apply::apply(&mut self.#ident, field_patch).map_err(|c| c.under(#path))?;
                }
            },
        }
    });
    let resolved_patch = if type_attrs.resolved_patch {
        let resolved = format_ident!("Resolved{name}Patch");
        let doc = format!(
            "[`{name}`]'s patch resolved (S6.1): each field `None` where the patch leaves it \
             as it is, else its resolved value."
        );
        let members = fields.iter().map(|f| {
            let BlockField {
                ident, vis, docs, ..
            } = f;
            let ty = f.resolved_ty();
            quote!(#(#docs)* #vis #ident: ::core::option::Option<#ty>,)
        });
        quote! {
            #[doc = #doc]
            #[derive(Clone, Debug, Default, PartialEq)]
            #[non_exhaustive]
            #vis struct #resolved { #(#members)* }
        }
    } else {
        TokenStream::new()
    };
    quote! {
        #[doc = #doc]
        #[derive(Clone, Debug, Default, PartialEq)]
        #[non_exhaustive]
        #vis struct #patch { #(#members)* }

        #resolved_patch

        #common

        impl #b::FieldsBlock for #name {
            type Patch = #patch;
        }

        impl #b::Merge for #patch {
            fn merge(&mut self, later: Self) -> ::core::result::Result<(), #b::MergeConflict> {
                let Self { #(#idents),* } = later;
                #(#merges)*
                ::core::result::Result::Ok(())
            }
        }

        impl #b::Apply<#patch> for #name {
            fn apply(&mut self, patch: #patch) -> ::core::result::Result<(), #b::MergeConflict> {
                let #patch { #(#idents),* } = patch;
                #(#applies)*
                ::core::result::Result::Ok(())
            }
        }

        impl #b::Create for #name {
            fn create_in(
                patch: #patch,
                inner: ::core::option::Option<&[&str]>,
            ) -> ::core::result::Result<Self, #b::MergeConflict> {
                let _ = inner;
                let #patch { #(#idents),* } = patch;
                ::core::result::Result::Ok(Self { #(#idents: #created),* })
            }
        }

        #read

        #walk

        #walk_patch
    }
}

fn expand_append(
    input: &DeriveInput,
    type_attrs: &TypeAttrs,
    fields: &[BlockField],
) -> Result<TokenStream> {
    let name = &input.ident;
    let vis = &input.vis;
    let b = blocks();
    let m = model();
    let (list, list_ty) = match fields {
        [only] if generic_arg(&only.ty, "Vec").is_some() => (&only.ident, &only.ty),
        _ => {
            return Err(Error::new_spanned(
                &input.ident,
                "an append block has one field, a `Vec`",
            ));
        }
    };
    let list_name = list.as_ref().map(Ident::to_string).unwrap_or_default();
    let patch = format_ident!("{name}Patch");
    let common = expand_struct_common(input, type_attrs, fields);
    let doc = format!(
        "[`{name}`]'s patch (S6.1): the value set whole, or `{list_name}` appended to over \
         the value below."
    );
    let w = quote!(crate::next::validate::walk);
    let check_patch = if type_attrs.check {
        quote!(w.check(#w::Node::#patch(self));)
    } else {
        TokenStream::new()
    };
    let walk = super::walk::struct_walk(
        name,
        name,
        fields,
        false,
        &super::walk::Expr::Calls,
        type_attrs.check,
    );
    let append_doc = format!(
        "Appends to `{list_name}` over the value below; over `Absent`, the appended items \
         are the whole value. Composed lazily over a reference (S2.2)."
    );
    Ok(quote! {
        #[doc = #doc]
        #[derive(Clone, Debug, PartialEq)]
        #vis enum #patch {
            /// Sets the value: a whole block, a reference or `Absent`.
            Set(#m::Value<#name>),
            #[doc = #append_doc]
            Append(#list_ty),
        }

        #common

        impl #b::Merge for #patch {
            fn merge(&mut self, later: Self) -> ::core::result::Result<(), #b::MergeConflict> {
                match later {
                    Self::Set(value) => {
                        *self = Self::Set(value);
                        ::core::result::Result::Ok(())
                    }
                    Self::Append(items) => match self {
                        Self::Set(value) => #b::Apply::apply(value, Self::Append(items)),
                        Self::Append(held) => {
                            held.extend(items);
                            ::core::result::Result::Ok(())
                        }
                    },
                }
            }
        }

        #walk

        impl #w::Walk for #patch {
            fn walk(&self, w: &mut #w::Walker<'_>) {
                #check_patch
                match self {
                    #patch::Set(value) => #w::Walk::walk(value, w),
                    #patch::Append(items) => #w::Walk::walk(items, w),
                }
            }
        }

        impl #b::Entry<#patch> for #m::Value<#name> {
            fn apply_entry(&mut self, patch: #patch) -> ::core::result::Result<(), #b::MergeConflict> {
                #b::Apply::apply(self, patch)
            }

            fn create(patch: #patch) -> ::core::result::Result<Self, #b::MergeConflict> {
                let mut value = #m::Value::Absent;
                #b::Apply::apply(&mut value, patch)?;
                ::core::result::Result::Ok(value)
            }
        }

        impl #b::Apply<#patch> for #m::Value<#name> {
            fn apply(&mut self, patch: #patch) -> ::core::result::Result<(), #b::MergeConflict> {
                match patch {
                    #patch::Set(value) => {
                        *self = value;
                        ::core::result::Result::Ok(())
                    }
                    #patch::Append(items) => match self {
                        #m::Value::Stated(block) => {
                            block.#list.extend(items);
                            ::core::result::Result::Ok(())
                        }
                        #m::Value::Absent => {
                            *self = #m::Value::Stated(#name { #list: items });
                            ::core::result::Result::Ok(())
                        }
                        other => #b::MergeConflict::over(other).map_or(::core::result::Result::Ok(()), ::core::result::Result::Err),
                    },
                }
            }
        }
    })
}

fn expand_enum(
    input: &DeriveInput,
    type_attrs: &TypeAttrs,
    data: &syn::DataEnum,
) -> Result<TokenStream> {
    let name = &input.ident;
    let vis = &input.vis;
    let b = blocks();
    let r = registry();
    let resolved = format_ident!("Resolved{name}");
    let mut variants = Vec::new();
    let mut infos = Vec::new();
    let mut units = Vec::new();
    for variant in &data.variants {
        let ident = &variant.ident;
        let docs = docs(&variant.attrs);
        let snake = snake_case(&ident.to_string());
        match &variant.fields {
            Fields::Named(named) => {
                let fields = named
                    .named
                    .iter()
                    .map(parse_field)
                    .collect::<Result<Vec<_>>>()?;
                let members = fields.iter().map(|f| {
                    let BlockField { ident, docs, .. } = f;
                    let ty = f.resolved_ty();
                    quote!(#(#docs)* #ident: #ty,)
                });
                variants.push(quote!(#(#docs)* #ident { #(#members)* }));
                for f in &fields {
                    let field = f.ident.as_ref().map(Ident::to_string).unwrap_or_default();
                    infos.push(f.field_info(&format!("{snake}.{field}")));
                }
            }
            Fields::Unnamed(unnamed) => {
                let fields = unnamed
                    .unnamed
                    .iter()
                    .map(parse_field)
                    .collect::<Result<Vec<_>>>()?;
                match fields.as_slice() {
                    [only] => {
                        let ty = only.resolved_ty();
                        let field_docs = &only.docs;
                        variants.push(quote!(#(#docs)* #ident(#(#field_docs)* #ty)));
                        infos.push(only.field_info(&snake));
                    }
                    several => {
                        if several.iter().any(|f| !f.is_plain_leaf()) {
                            return Err(Error::new_spanned(
                                variant,
                                "a block's tuple variant of several fields holds plain values, \
                                 unmarked",
                            ));
                        }
                        let tys = several.iter().map(|f| &f.ty);
                        variants.push(quote!(#(#docs)* #ident(#(#tys),*)));
                        let written = several
                            .iter()
                            .map(|f| type_string(&f.ty))
                            .collect::<Vec<_>>()
                            .join(",");
                        let written = format!("({written})");
                        let r = registry();
                        infos.push(quote! {
                            #r::FieldInfo {
                                path: #snake,
                                ty: #r::FieldType::Leaf(#written),
                                may_be_absent: false,
                                auto: false,
                            }
                        });
                    }
                }
            }
            Fields::Unit => {
                variants.push(quote!(#(#docs)* #ident));
                units.push(snake);
            }
        }
    }
    let form = quote!(#r::BlockForm::Enum { unit_variants: &[#(#units),*] });
    let walk = super::walk::enum_walk(
        name,
        data,
        &super::walk::Expr::ReferenceOnly,
        type_attrs.check,
    )?;
    let read = if type_attrs.own_toml {
        TokenStream::new()
    } else {
        super::read::enum_impl(name, data)?
    };
    let resolved_enum = if type_attrs.own_resolved {
        TokenStream::new()
    } else {
        let doc = resolved_doc(name);
        quote! {
            #[doc = #doc]
            #[derive(Clone, Debug, PartialEq)]
            #vis enum #resolved { #(#variants),* }
        }
    };
    Ok(quote! {
        #resolved_enum

        impl #b::sealed::Sealed for #name {}

        impl #b::Block for #name {
            type Resolved = #resolved;
        }

        impl #b::Table for #name {
            const FIELDS: &'static [#r::FieldInfo] = &[#(#infos),*];
            const FORM: #r::BlockForm = #form;
        }

        #read

        #walk
    })
}
