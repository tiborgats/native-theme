// Generation of merge() and is_empty() methods on the Option struct.

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::parse::{FieldCategory, FieldMeta};

/// Generate `merge()` and `is_empty()` impl block on the Option struct.
pub(crate) fn gen_merge(opt_name: &Ident, fields: &[FieldMeta]) -> TokenStream {
    let merge_stmts = gen_merge_stmts(fields);
    let is_empty_expr = gen_is_empty_expr(fields);

    quote! {
        impl #opt_name {
            /// Merge an overlay into this value. `Some` fields in the overlay
            /// replace the corresponding fields in self; `None` fields are
            /// left unchanged. Nested structs are merged recursively.
            pub fn merge(&mut self, overlay: &Self) {
                #merge_stmts
            }

            /// Returns true if all fields are at their default (None/empty) state.
            pub fn is_empty(&self) -> bool {
                true #is_empty_expr
            }
        }
    }
}

/// Generate merge statements per field category.
fn gen_merge_stmts(fields: &[FieldMeta]) -> TokenStream {
    let stmts: Vec<TokenStream> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            match &f.category {
                FieldCategory::Option | FieldCategory::SoftOption => {
                    quote! {
                        if overlay.#ident.is_some() {
                            self.#ident = overlay.#ident.clone();
                        }
                    }
                }
                FieldCategory::Nested { .. } => {
                    quote! {
                        match (&mut self.#ident, &overlay.#ident) {
                            (Some(base), Some(over)) => base.merge(over),
                            (None, Some(over)) => self.#ident = Some(over.clone()),
                            _ => {}
                        }
                    }
                }
            }
        })
        .collect();

    quote! { #(#stmts)* }
}

/// Generate is_empty chain: `&& field.is_none()` or `&& field.as_ref().map_or(true, |v| v.is_empty())`.
fn gen_is_empty_expr(fields: &[FieldMeta]) -> TokenStream {
    let checks: Vec<TokenStream> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            match &f.category {
                FieldCategory::Option | FieldCategory::SoftOption => {
                    quote! { && self.#ident.is_none() }
                }
                FieldCategory::Nested { .. } => {
                    quote! { && self.#ident.as_ref().map_or(true, |v| v.is_empty()) }
                }
            }
        })
        .collect();

    quote! { #(#checks)* }
}
