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

/// Where a block field's block sits in its type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Wrap {
    /// `B`.
    Bare,
    /// `Value<B>`.
    Value,
    /// `Vec<B>`.
    List,
}

/// What a field holds, which decides its patch, its merge and its resolved
/// type.
enum Shape {
    /// A value with no fields of its own.
    Leaf,
    /// A `Vec` of pairs: merged by key.
    Map,
    /// A block of the model.
    Block {
        kind: BlockKind,
        wrap: Wrap,
        inner: Box<Type>,
        name: Ident,
    },
}

struct BlockField {
    /// `None` for a tuple variant's one field.
    ident: Option<Ident>,
    vis: Visibility,
    ty: Type,
    docs: Vec<Attribute>,
    attrs: FieldAttrs,
    shape: Shape,
}

/// How a later patch's field merges over the earlier one's.
enum MergeKind {
    Replace,
    ByKey,
    Nested,
}

impl BlockField {
    fn merge_kind(&self) -> MergeKind {
        match &self.shape {
            Shape::Leaf => MergeKind::Replace,
            Shape::Map => MergeKind::ByKey,
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

    fn patch_ty(&self) -> TokenStream {
        let b = blocks();
        match &self.shape {
            Shape::Block {
                kind: BlockKind::Fields,
                wrap: Wrap::Bare,
                name,
                ..
            }
            | Shape::Block {
                kind: BlockKind::Append,
                wrap: Wrap::Value,
                name,
                ..
            } => {
                let patch = format_ident!("{name}Patch");
                quote!(Option<#patch>)
            }
            Shape::Block {
                kind: BlockKind::Fields,
                wrap: Wrap::Value,
                inner,
                ..
            } => quote!(Option<#b::BlockPatch<#inner>>),
            Shape::Leaf | Shape::Map | Shape::Block { .. } => {
                let ty = &self.ty;
                quote!(Option<#ty>)
            }
        }
    }

    fn resolved_ty(&self) -> TokenStream {
        let block = match &self.shape {
            Shape::Block { name, .. } => Some(name),
            Shape::Leaf | Shape::Map => None,
        };
        resolved_ty(&self.ty, block, self.attrs.auto)
    }

    fn field_info(&self, path: &str) -> TokenStream {
        let m = model();
        let b = blocks();
        let written = type_string(&self.ty);
        let ty = match &self.shape {
            Shape::Block { inner, .. } => quote! {
                #m::FieldType::Block { ty: #written, fields: <#inner as #b::Block>::FIELDS }
            },
            Shape::Leaf | Shape::Map => quote!(#m::FieldType::Leaf(#written)),
        };
        let may_be_absent = self.attrs.may_be_absent;
        let auto = self.attrs.auto;
        quote! {
            #m::FieldInfo { path: #path, ty: #ty, may_be_absent: #may_be_absent, auto: #auto }
        }
    }
}

/// `T` of `Wrapper<T>`, where the type's last segment is `wrapper` with one
/// type argument.
fn generic_arg<'a>(ty: &'a Type, wrapper: &str) -> Option<&'a Type> {
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

/// The field's value is a `Value`, or an `Option` of one, holding `inner`
/// when given.
fn holds_value(ty: &Type, inner: Option<&str>) -> bool {
    let ty = generic_arg(ty, "Option").unwrap_or(ty);
    generic_arg(ty, "Value").is_some_and(|held| match inner {
        Some(name) => plain_ident(held).is_some_and(|ident| ident == name),
        None => true,
    })
}

/// The type as written, without whitespace: `Value<Px>`.
fn type_string(ty: &Type) -> String {
    quote!(#ty).to_string().split_whitespace().collect()
}

/// The resolved form of a type: each `Value<T>` to `Option<T>` (a `Value<Px>`
/// that may be `Auto` to `Option<Length>`), the block named `block` to its
/// resolved type, through `Option`, `Vec` and tuples.
fn resolved_ty(ty: &Type, block: Option<&Ident>, auto: bool) -> TokenStream {
    if let Some(inner) = generic_arg(ty, "Value") {
        if auto && plain_ident(inner).is_some_and(|ident| ident == "Px") {
            let m = model();
            return quote!(Option<#m::Length>);
        }
        let inner = resolved_ty(inner, block, auto);
        return quote!(Option<#inner>);
    }
    if let Some(inner) = generic_arg(ty, "Option") {
        let inner = resolved_ty(inner, block, auto);
        return quote!(Option<#inner>);
    }
    if let Some(inner) = generic_arg(ty, "Vec") {
        let inner = resolved_ty(inner, block, auto);
        return quote!(Vec<#inner>);
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
    if let (Some(name), Some(ident)) = (block, plain_ident(ty))
        && ident == name
    {
        let resolved = format_ident!("Resolved{name}");
        return quote!(#resolved);
    }
    quote!(#ty)
}

fn parse_field(field: &syn::Field) -> Result<BlockField> {
    let attrs = attrs::parse_field_attrs(&field.attrs)?;
    let ty = field.ty.clone();
    if attrs.may_be_absent && !holds_value(&ty, None) {
        return Err(Error::new_spanned(
            &field.ty,
            "may_be_absent marks a field holding a `Value`",
        ));
    }
    if attrs.auto && !holds_value(&ty, Some("Px")) {
        return Err(Error::new_spanned(
            &field.ty,
            "auto marks a field holding a `Value<Px>`",
        ));
    }
    let shape = match attrs.block {
        Some(kind) => {
            let (wrap, inner) = if let Some(inner) = generic_arg(&ty, "Value") {
                (Wrap::Value, inner)
            } else if let Some(inner) = generic_arg(&ty, "Vec") {
                (Wrap::List, inner)
            } else {
                (Wrap::Bare, &ty)
            };
            let name = plain_ident(inner).cloned().ok_or_else(|| {
                Error::new_spanned(
                    &field.ty,
                    "a block field's type is `B`, `Value<B>` or `Vec<B>`, `B` a block",
                )
            })?;
            if kind == BlockKind::Append && wrap == Wrap::Bare {
                return Err(Error::new_spanned(
                    &field.ty,
                    "an append block is held as `Value<B>` or `Vec<B>`",
                ));
            }
            Shape::Block {
                kind,
                wrap,
                inner: Box::new(inner.clone()),
                name,
            }
        }
        None => {
            let pairs = generic_arg(&ty, "Vec")
                .is_some_and(|item| matches!(item, Type::Tuple(t) if t.elems.len() == 2));
            if pairs { Shape::Map } else { Shape::Leaf }
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
fn snake_case(name: &str) -> String {
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

/// The resolved struct, unless written by hand, and the `Block` impl.
fn expand_struct_common(
    input: &DeriveInput,
    type_attrs: &TypeAttrs,
    fields: &[BlockField],
) -> TokenStream {
    let name = &input.ident;
    let vis = &input.vis;
    let b = blocks();
    let m = model();
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

        impl #b::Block for #name {
            type Resolved = #resolved;
            const FIELDS: &'static [#m::FieldInfo] = &[#(#infos),*];
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
    let merges = fields.iter().map(|f| {
        let ident = &f.ident;
        let path = ident.as_ref().map(Ident::to_string).unwrap_or_default();
        match f.merge_kind() {
            MergeKind::Replace => quote!(#b::replace_slot(&mut self.#ident, #ident);),
            MergeKind::ByKey => quote!(#b::merge_slot_by_key(&mut self.#ident, #ident);),
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
                if let Some(value) = #ident { self.#ident = value; }
            },
            MergeKind::ByKey => quote! {
                if let Some(entries) = #ident { #b::merge_by_key(&mut self.#ident, entries); }
            },
            MergeKind::Nested => quote! {
                if let Some(field_patch) = #ident {
                    #b::Apply::apply(&mut self.#ident, field_patch).map_err(|c| c.under(#path))?;
                }
            },
        }
    });
    quote! {
        #[doc = #doc]
        #[derive(Clone, Debug, Default, PartialEq)]
        #[non_exhaustive]
        #vis struct #patch { #(#members)* }

        #common

        impl #b::FieldsBlock for #name {
            type Patch = #patch;
        }

        impl #b::Merge for #patch {
            fn merge(&mut self, later: Self) -> ::core::result::Result<(), #b::MergeConflict> {
                let Self { #(#idents),* } = later;
                #(#merges)*
                Ok(())
            }
        }

        impl #b::Apply<#patch> for #name {
            fn apply(&mut self, patch: #patch) -> ::core::result::Result<(), #b::MergeConflict> {
                let #patch { #(#idents),* } = patch;
                #(#applies)*
                Ok(())
            }
        }
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
                        Ok(())
                    }
                    Self::Append(items) => match self {
                        Self::Set(value) => #b::Apply::apply(value, Self::Append(items)),
                        Self::Append(held) => {
                            held.extend(items);
                            Ok(())
                        }
                    },
                }
            }
        }

        impl #b::Apply<#patch> for #m::Value<#name> {
            fn apply(&mut self, patch: #patch) -> ::core::result::Result<(), #b::MergeConflict> {
                match patch {
                    #patch::Set(value) => {
                        *self = value;
                        Ok(())
                    }
                    #patch::Append(items) => match self {
                        #m::Value::Stated(block) => {
                            block.#list.extend(items);
                            Ok(())
                        }
                        #m::Value::Absent => {
                            *self = #m::Value::Stated(#name { #list: items });
                            Ok(())
                        }
                        other => #b::MergeConflict::over(other).map_or(Ok(()), Err),
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
    let m = model();
    let resolved = format_ident!("Resolved{name}");
    let mut variants = Vec::new();
    let mut infos = Vec::new();
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
                let field = match unnamed.unnamed.iter().collect::<Vec<_>>().as_slice() {
                    [only] => parse_field(only)?,
                    _ => {
                        return Err(Error::new_spanned(
                            variant,
                            "a block's tuple variant holds one field",
                        ));
                    }
                };
                let ty = field.resolved_ty();
                let field_docs = &field.docs;
                variants.push(quote!(#(#docs)* #ident(#(#field_docs)* #ty)));
                infos.push(field.field_info(&snake));
            }
            Fields::Unit => variants.push(quote!(#(#docs)* #ident)),
        }
    }
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

        impl #b::Block for #name {
            type Resolved = #resolved;
            const FIELDS: &'static [#m::FieldInfo] = &[#(#infos),*];
        }
    })
}
