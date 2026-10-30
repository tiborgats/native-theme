// The walk of a block, its patch and an enum read from TOML: every field
// handed to S12's checks with its path and marks (`next::validate::walk`).

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, Ident, Result};

use super::block::{BlockField, Shape, parse_field};

/// The module the walk's types live in.
fn walk() -> TokenStream {
    quote!(crate::next::validate::walk)
}

/// How an expression stated on a `Value` of the walked type may be formed.
pub(super) enum Expr {
    /// A literal, a reference or a call (a paint).
    Calls,
    /// A literal or a reference (an enum read from TOML).
    NoCalls,
    /// A reference only (a block).
    ReferenceOnly,
}

impl Expr {
    fn tokens(&self) -> TokenStream {
        let w = walk();
        match self {
            Expr::Calls => quote!(#w::ExprForm::Calls),
            Expr::NoCalls => quote!(#w::ExprForm::NoCalls),
            Expr::ReferenceOnly => quote!(#w::ExprForm::ReferenceOnly),
        }
    }
}

/// The field's marks as the walk takes them.
fn marks(f: &BlockField) -> TokenStream {
    let w = walk();
    let may_be_absent = f.attrs.may_be_absent;
    let auto = f.attrs.auto;
    let signed = f.attrs.signed;
    quote!(#w::Marks { may_be_absent: #may_be_absent, auto: #auto, signed: #signed })
}

/// The walk of one field's value `value` (a reference to it); `optional`:
/// the value is held in a patch's `Option`.
fn field_walk(f: &BlockField, value: &TokenStream, optional: bool) -> TokenStream {
    let w = walk();
    let call = match &f.shape {
        Shape::Map(_) => Some(quote!(#w::walk_map)),
        Shape::NestedMap => Some(quote!(#w::walk_nested_map)),
        _ => None,
    };
    let inner = match call {
        Some(call) if optional => quote! {
            if let ::core::option::Option::Some(map) = #value {
                #call(map, w);
            }
        },
        Some(call) => quote!(#call(#value, w);),
        None => quote!(#w::Walk::walk(#value, w);),
    };
    if f.attrs.inner_may_be_absent {
        let required = &f.attrs.required;
        quote!(w.place(&[#(#required),*], |w| { #inner });)
    } else {
        inner
    }
}

/// The walk of a struct block (`patch` false) or of its patch, each field
/// walked under its name with its marks.
pub(super) fn struct_walk(
    owner: &Ident,
    walked: &Ident,
    fields: &[BlockField],
    patch: bool,
    expr: &Expr,
) -> TokenStream {
    let w = walk();
    let owner_name = owner.to_string();
    let form = expr.tokens();
    let steps = fields.iter().filter_map(|f| {
        let ident = f.ident.as_ref()?;
        let name = ident.to_string();
        let marks = marks(f);
        let inner = field_walk(f, &quote!(&self.#ident), patch);
        Some(quote! {
            w.field(#owner_name, #name, #marks, |w| { #inner });
        })
    });
    let patch_fields = if patch {
        let names = fields
            .iter()
            .filter_map(|f| f.ident.as_ref().map(Ident::to_string));
        quote!(w.patch(&[#(#names),*], &#w::Walk::set_fields(self));)
    } else {
        TokenStream::new()
    };
    let set_fields = if patch {
        let checks = fields.iter().filter_map(|f| {
            let ident = f.ident.as_ref()?;
            let name = ident.to_string();
            Some(quote! {
                if self.#ident.is_some() {
                    set.push(#name);
                }
            })
        });
        quote! {
            fn set_fields(&self) -> ::std::vec::Vec<&'static str> {
                let mut set = ::std::vec::Vec::new();
                #(#checks)*
                set
            }
        }
    } else {
        TokenStream::new()
    };
    quote! {
        impl #w::Walk for #walked {
            const EXPR: #w::ExprForm = #form;

            fn walk(&self, w: &mut #w::Walker<'_>) {
                w.node(self);
                #patch_fields
                #(#steps)*
            }

            #set_fields
        }
    }
}

/// The walk of an enum: the variant's fields, a struct variant's under their
/// names, a tuple variant's one value as it is, several by position.
pub(super) fn enum_walk(name: &Ident, data: &syn::DataEnum, expr: &Expr) -> Result<TokenStream> {
    let w = walk();
    let owner = name.to_string();
    let form = expr.tokens();
    let mut arms = Vec::new();
    for variant in &data.variants {
        let ident = &variant.ident;
        match &variant.fields {
            Fields::Unit => arms.push(quote!(#name::#ident => {})),
            Fields::Named(named) => {
                let fields = named
                    .named
                    .iter()
                    .map(parse_field)
                    .collect::<Result<Vec<_>>>()?;
                let idents: Vec<&Ident> = fields.iter().filter_map(|f| f.ident.as_ref()).collect();
                let steps = fields.iter().filter_map(|f| {
                    let ident = f.ident.as_ref()?;
                    let field = ident.to_string();
                    let marks = marks(f);
                    let inner = field_walk(f, &quote!(#ident), false);
                    Some(quote!(w.field(#owner, #field, #marks, |w| { #inner });))
                });
                arms.push(quote!(#name::#ident { #(#idents),* } => { #(#steps)* }));
            }
            Fields::Unnamed(unnamed) => {
                let fields = unnamed
                    .unnamed
                    .iter()
                    .map(parse_field)
                    .collect::<Result<Vec<_>>>()?;
                let members: Vec<Ident> = (0..fields.len())
                    .map(|i| format_ident!("member{i}"))
                    .collect();
                let steps = match (fields.as_slice(), members.as_slice()) {
                    ([only], [member]) => vec![field_walk(only, &quote!(#member), false)],
                    _ => fields
                        .iter()
                        .zip(&members)
                        .enumerate()
                        .map(|(i, (f, member))| {
                            let inner = field_walk(f, &quote!(#member), false);
                            quote!(w.index(#i, |w| { #inner });)
                        })
                        .collect(),
                };
                arms.push(quote!(#name::#ident(#(#members),*) => { #(#steps)* }));
            }
        }
    }
    let body = if arms.is_empty() {
        quote!(match *self {})
    } else {
        quote!(match self { #(#arms)* })
    };
    Ok(quote! {
        impl #w::Walk for #name {
            const EXPR: #w::ExprForm = #form;

            fn walk(&self, w: &mut #w::Walker<'_>) {
                w.node(self);
                #body
            }
        }
    })
}
