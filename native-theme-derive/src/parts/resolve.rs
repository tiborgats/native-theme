// Block mode's resolution: where a path leads below a block and its patch
// (`Locate`), and a block resolved from the steps of its place (`Resolve`,
// `ResolveWhole`), each field by its shape (S2.2, S6.1, S8).

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, Type};

use super::attrs::BlockKind;
use super::block::{
    BlockField, MapBlock, Shape, Wrap, generic_arg, holds_value, map_pair, plain_ident,
};

/// The module the expansions name the steps and their places by.
fn compose() -> TokenStream {
    quote!(crate::next::resolve::compose)
}

/// The module the expansions name the evaluation context by.
fn evaluate() -> TokenStream {
    quote!(crate::next::resolve::evaluate)
}

fn name_of(field: &BlockField) -> String {
    field
        .ident
        .as_ref()
        .map(Ident::to_string)
        .unwrap_or_default()
}

/// Where `rest` leads below `value`, a field of the block as it is.
pub(super) fn whole_locate(field: &BlockField, value: &TokenStream) -> TokenStream {
    let c = compose();
    let locate = quote!(#c::Locate::locate(#value, rest));
    let leaf = quote!(#c::leaf(#value, rest));
    if field.attrs.patch {
        return locate;
    }
    match &field.shape {
        Shape::Leaf
        | Shape::Block {
            wrap: Wrap::List | Wrap::Option,
            ..
        } => leaf,
        Shape::Block { .. } | Shape::Surfaces(_) | Shape::Compound(_) => locate,
        Shape::Map(Some(map)) => map_locate(map, value),
        Shape::Map(None) => quote!(#c::entry(#value, rest, #c::leaf)),
        Shape::NestedMap => quote!(#c::entry(#value, rest, |v, r| #c::entry(v, r, #c::leaf))),
    }
}

fn map_locate(map: &MapBlock, value: &TokenStream) -> TokenStream {
    let c = compose();
    match map.wrap {
        Wrap::Bare | Wrap::Value => {
            quote!(#c::entry(#value, rest, |v, r| #c::Locate::locate(v, r)))
        }
        Wrap::List | Wrap::Option => quote!(#c::entry(#value, rest, #c::leaf)),
    }
}

/// Where `rest` leads below `value`, what a patch of the block sets the
/// field to.
pub(super) fn patch_locate(field: &BlockField, value: &TokenStream) -> TokenStream {
    let c = compose();
    let locate = quote!(#c::Locate::locate(#value, rest));
    if field.attrs.patch {
        return locate;
    }
    match &field.shape {
        Shape::Leaf
        | Shape::Block {
            wrap: Wrap::List | Wrap::Option,
            ..
        } => quote!(#c::leaf(#value, rest)),
        Shape::Block { .. } | Shape::Surfaces(_) | Shape::Compound(_) => locate,
        Shape::Map(Some(map)) if map.entry_patch().is_some() => {
            quote!(#c::entry(#value, rest, |v, r| #c::Locate::locate(v, r)))
        }
        Shape::Map(Some(map)) => map_locate(map, value),
        Shape::Map(None) => quote!(#c::entry(#value, rest, #c::leaf)),
        Shape::NestedMap => quote!(#c::entry(#value, rest, |v, r| #c::entry(v, r, #c::leaf))),
    }
}

/// `Locate` for a struct block and for its patch.
pub(super) fn struct_locate(
    name: &Ident,
    patch: Option<&Ident>,
    fields: &[BlockField],
) -> TokenStream {
    let c = compose();
    let whole_arms = fields.iter().map(|f| {
        let ident = &f.ident;
        let key = name_of(f);
        let found = whole_locate(f, &quote!(&self.#ident));
        quote!(#key => #found,)
    });
    let whole = quote! {
        impl #c::Locate for #name {
            fn locate(&self, path: &[&str]) -> #c::Found<'_> {
                let ::core::option::Option::Some((first, rest)) = path.split_first() else {
                    return #c::Found::Node(self);
                };
                let _ = rest;
                match *first {
                    #(#whole_arms)*
                    _ => #c::Found::Missing,
                }
            }
        }
    };
    let Some(patch) = patch else {
        return whole;
    };
    let patch_arms = fields.iter().map(|f| {
        let ident = &f.ident;
        let key = name_of(f);
        let found = patch_locate(f, &quote!(value));
        quote! {
            #key => match &self.#ident {
                ::core::option::Option::Some(value) => #found,
                ::core::option::Option::None => #c::Found::Missing,
            },
        }
    });
    quote! {
        #whole

        impl #c::Locate for #patch {
            fn locate(&self, path: &[&str]) -> #c::Found<'_> {
                let ::core::option::Option::Some((first, rest)) = path.split_first() else {
                    return #c::Found::Node(self);
                };
                let _ = rest;
                match *first {
                    #(#patch_arms)*
                    _ => #c::Found::Missing,
                }
            }

            fn whole(&self) -> bool {
                false
            }
        }
    }
}

/// `Locate` for an enum block: the value itself, nothing below it.
pub(super) fn enum_locate(name: &Ident) -> TokenStream {
    let c = compose();
    quote! {
        impl #c::Locate for #name {
            fn locate(&self, path: &[&str]) -> #c::Found<'_> {
                if path.is_empty() {
                    #c::Found::Node(self)
                } else {
                    #c::Found::Missing
                }
            }
        }
    }
}

/// The type a `Value<T>` field holds.
fn value_inner(ty: &Type) -> Option<&Type> {
    generic_arg(ty, "Value")
}

/// How a map's value, or its entry patch, is located below: a block's by
/// its `Locate`, a leaf's as a leaf.
fn value_locate(map: Option<&MapBlock>, patch: bool) -> TokenStream {
    let c = compose();
    let node = quote!(|v, r| #c::Locate::locate(v, r));
    match map {
        Some(map) if patch && map.entry_patch().is_some() => node,
        Some(map) => match map.wrap {
            Wrap::Bare | Wrap::Value => node,
            Wrap::List | Wrap::Option => quote!(#c::leaf),
        },
        None => quote!(#c::leaf),
    }
}

/// A map's value's resolution of one entry's steps `e`, by the map's shape.
fn entry_call(map: &MapBlock) -> TokenStream {
    let b = &map.block.ty;
    match (map.kind, map.wrap) {
        (BlockKind::Fields, Wrap::Bare) => quote!(|cx, e| cx.block::<#b>(e)),
        (BlockKind::Fields, Wrap::Value) => {
            quote!(|cx, e| ::core::option::Option::Some(cx.block::<#b>(e)))
        }
        (BlockKind::Append, _) => {
            quote!(|cx, e| ::core::option::Option::Some(cx.append::<#b>(e)))
        }
        (BlockKind::Set, Wrap::Value) => {
            quote!(|cx, e| ::core::option::Option::Some(cx.set::<#b>(e)))
        }
        (BlockKind::Set, Wrap::Bare | Wrap::Option) => quote!(|cx, e| cx.set::<#b>(e)),
        (_, Wrap::List) => quote!(|cx, e| cx.list::<#b>(e)),
        (BlockKind::Fields, Wrap::Option) => quote!(|cx, e| cx.set::<#b>(e)),
    }
}

/// A leaf map value's resolution of one entry's steps `e`.
fn leaf_entry(value: &Type, signed: bool) -> TokenStream {
    match value_inner(value) {
        Some(inner) => {
            quote!(|cx, e| ::core::option::Option::Some(cx.value::<#inner>(e, #signed)))
        }
        None => quote!(|cx, e| cx.plain::<#value>(e)),
    }
}

/// The field's resolved value from its steps `items`, by its shape, and
/// whether the field is required: a field that cannot be absent with no
/// value leaves the block none.
pub(super) fn resolve_call(owner: &Ident, field: &BlockField) -> (TokenStream, bool) {
    let signed = field.attrs.signed;
    let ty = &field.ty;
    if field.attrs.patch {
        let block = plain_ident(ty)
            .and_then(|ident| ident.to_string().strip_suffix("Patch").map(str::to_owned))
            .map(|name| format_ident!("{name}"));
        return (quote!(cx.patch::<#block>(&items)), true);
    }
    // A text style's font and line height are its frame's, resolved once
    // when the style is entered (`Cx::text_style`).
    if owner == "TextStyle" {
        match name_of(field).as_str() {
            "font" => return (quote!(cx.framed_font()), true),
            "line_height" => return (quote!(cx.framed_line_height()), false),
            _ => {}
        }
    }
    match &field.shape {
        Shape::Leaf => {
            if let Some(inner) = generic_arg(ty, "Option").and_then(value_inner) {
                return (quote!(cx.optional::<#inner>(&items, #signed)), true);
            }
            let Some(inner) = value_inner(ty) else {
                return (quote!(cx.plain::<#ty>(&items)), true);
            };
            let size = owner == "Font" && name_of(field) == "size";
            // A text style's sizes are text-scaled (S9.4); any other font
            // size, a widget value's, only converted from points (S1).
            let scaled = owner == "Font" || owner == "TextStyle";
            if holds_value(ty, Some(&["FontSize"])) {
                let call = if field.attrs.auto {
                    quote!(cx.font_auto(&items, #size))
                } else {
                    quote!(cx.font(&items, #size))
                };
                if scaled {
                    (call, false)
                } else {
                    (quote!(cx.unscaled(|cx| #call)), false)
                }
            } else if field.attrs.auto {
                (quote!(cx.auto(&items, #signed)), false)
            } else {
                (quote!(cx.value::<#inner>(&items, #signed)), false)
            }
        }
        Shape::Block { kind, wrap, block } => {
            let b = &block.ty;
            match (kind, wrap) {
                (BlockKind::Fields, Wrap::Value) => (quote!(cx.block::<#b>(&items)), false),
                (BlockKind::Fields, Wrap::Bare) => (quote!(cx.block::<#b>(&items)), true),
                (BlockKind::Set, Wrap::Value) => (quote!(cx.set::<#b>(&items)), false),
                (BlockKind::Set, Wrap::Bare) => (quote!(cx.set::<#b>(&items)), true),
                (BlockKind::Append, _) => (quote!(cx.append::<#b>(&items)), false),
                (_, Wrap::List) if field.attrs.keyed => (quote!(cx.keyed::<#b>(&items)), true),
                (_, Wrap::List) => (quote!(cx.list::<#b>(&items)), true),
                (_, Wrap::Option) => (quote!(cx.option::<#b>(&items)), true),
            }
        }
        Shape::Map(Some(map)) => {
            let (key, value) = match map_pair(ty) {
                Some((key, value)) => (key, value),
                None => return (quote!(::core::option::Option::None), true),
            };
            let entry_patch = map.entry_patch().unwrap_or_else(|| quote!(#value));
            let entry = entry_call(map);
            let (lv, lp) = (
                value_locate(Some(map), false),
                value_locate(Some(map), true),
            );
            (
                quote!(cx.map::<#key, #value, #entry_patch, _>(&items, #lv, #lp, #entry)),
                true,
            )
        }
        Shape::Map(None) => match map_pair(ty) {
            Some((key, value)) => {
                let entry = leaf_entry(value, signed);
                let leaf = value_locate(None, false);
                (
                    quote!(cx.map::<#key, #value, #value, _>(&items, #leaf, #leaf, #entry)),
                    true,
                )
            }
            None => (quote!(::core::option::Option::None), true),
        },
        Shape::NestedMap => match map_pair(ty).and_then(|(key, inner)| {
            map_pair(inner).map(|(inner_key, value)| (key, inner, inner_key, value))
        }) {
            Some((key, inner, inner_key, value)) => {
                let entry = leaf_entry(value, signed);
                let c = compose();
                let leaf = value_locate(None, false);
                let call = quote! {
                    cx.map::<#key, #inner, #inner, _>(
                        &items,
                        #c::leaf,
                        #c::leaf,
                        |cx, e| cx.map::<#inner_key, #value, #value, _>(e, #leaf, #leaf, #entry),
                    )
                };
                (call, true)
            }
            None => (quote!(::core::option::Option::None), true),
        },
        Shape::Surfaces(block) => {
            let b = &block.ty;
            (quote!(cx.surfaces::<#b>(&items)), true)
        }
        Shape::Compound(compound) => {
            let inner = &compound.inner;
            let (value, plain) = match compound.name {
                "Axes" => (format_ident!("axes"), format_ident!("axes_plain")),
                _ => (format_ident!("sides"), format_ident!("sides_plain")),
            };
            match value_inner(inner) {
                Some(inner) => (quote!(cx.#value::<#inner>(&items, #signed)), false),
                None => (quote!(cx.#plain::<#inner>(&items)), true),
            }
        }
    }
}

/// Whose declared units a `params` map of `owner` takes (S2.7, S10): a
/// material's, or a keyed kind's by its `key` binding; `None` for any other
/// field.
fn params_of(owner: &Ident, field: &BlockField) -> Option<TokenStream> {
    if name_of(field) != "params" {
        return None;
    }
    let ev = evaluate();
    match owner.to_string().as_str() {
        "Material" => Some(quote!(#ev::ParamsOf::Material)),
        "TransitionKind" => Some(quote!(#ev::ParamsOf::TransitionKind(key))),
        "PresentationKind" => Some(quote!(#ev::ParamsOf::PresentationKind(key))),
        _ => None,
    }
}

/// One field's resolved value from `items`, its steps, within its place.
fn field_value(owner: &Ident, field: &BlockField, items: &TokenStream) -> TokenStream {
    let key = name_of(field);
    let (call, required) = resolve_call(owner, field);
    let call = match params_of(owner, field) {
        Some(of) => quote!(cx.params(#of, |cx| #call)),
        None => call,
    };
    let value = if required {
        quote!(cx.at(#key, |cx| #call)?)
    } else {
        quote!(cx.at(#key, |cx| #call))
    };
    quote! {{
        let items = #items;
        #value
    }}
}

/// `Resolve` and `ResolveWhole` for a struct block resolved into
/// `Resolved<name>`.
pub(super) fn struct_resolve(name: &Ident, fields: &[BlockField]) -> TokenStream {
    let c = compose();
    let ev = evaluate();
    let resolved = format_ident!("Resolved{name}");
    let members = fields.iter().map(|f| {
        let ident = &f.ident;
        let key = name_of(f);
        let value = field_value(name, f, &quote!(#c::children(folds, #key)));
        quote!(#ident: #value,)
    });
    let body = quote! {
        ::core::option::Option::Some(#resolved { #(#members)* })
    };
    let body = if name == "TextStyle" {
        quote!(cx.text_style(folds, |cx| #body))
    } else {
        body
    };
    quote! {
        impl #ev::Resolve for #name {
            fn resolve<'t: 'f, 'f>(
                folds: &[#c::Fold<'f>],
                cx: &mut #ev::Cx<'t>,
            ) -> ::core::option::Option<#resolved> {
                #body
            }
        }

        impl #ev::ResolveWhole for #name {
            fn resolve_whole<'t: 'f, 'f>(
                &'f self,
                pos: crate::next::resolve::Pos,
                cx: &mut #ev::Cx<'t>,
            ) -> ::core::option::Option<#resolved> {
                let fold = #c::Fold { pos, target: cx.target(), found: #c::Found::Node(self) };
                <Self as #ev::Resolve>::resolve(&[fold], cx)
            }
        }
    }
}

/// One variant's resolved value: its fields' each from the one step that
/// states the value.
pub(super) fn variant_value(owner: &Ident, field: &BlockField, binding: &Ident) -> TokenStream {
    let c = compose();
    let found = whole_locate(field, &quote!(#binding));
    let items = quote! {{
        let rest: &[&str] = &[];
        [#c::Fold { pos, target: cx.target(), found: #found }]
    }};
    field_value(owner, field, &items)
}

/// `ResolveWhole` for an enum block, each variant's fields resolved.
pub(super) fn enum_resolve(name: &Ident, arms: &[TokenStream]) -> TokenStream {
    let ev = evaluate();
    let resolved = format_ident!("Resolved{name}");
    quote! {
        impl #ev::ResolveWhole for #name {
            fn resolve_whole<'t: 'f, 'f>(
                &'f self,
                pos: crate::next::resolve::Pos,
                cx: &mut #ev::Cx<'t>,
            ) -> ::core::option::Option<#resolved> {
                let _ = (pos, &cx);
                match self {
                    #(#arms)*
                }
            }
        }
    }
}
