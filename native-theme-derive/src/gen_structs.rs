// Generation of the Resolved struct and FIELD_NAMES const.

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::parse::{FieldCategory, FieldMeta};
use crate::util::option_inner;

/// Generate the Resolved struct and FIELD_NAMES const on the Option struct.
pub(crate) fn gen_structs(
    opt_name: &Ident,
    resolved_name: &Ident,
    fields: &[FieldMeta],
    doc_attrs: &[syn::Attribute],
) -> TokenStream {
    let resolved_fields = gen_resolved_fields(fields);
    let field_names = gen_field_names(fields);

    // Collect doc attributes for the resolved struct
    let doc_tokens: Vec<_> = doc_attrs.iter().collect();

    quote! {
        #(#doc_tokens)*
        #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        #[non_exhaustive]
        pub struct #resolved_name {
            #resolved_fields
        }

        impl #opt_name {
            /// All serialized field names for this widget theme, for TOML linting.
            pub const FIELD_NAMES: &[&str] = &[#field_names];
        }
    }
}

/// Generate resolved struct fields: option -> T, soft_option -> Option<T>,
/// nested/border -> resolved type.
fn gen_resolved_fields(fields: &[FieldMeta]) -> TokenStream {
    let field_tokens: Vec<TokenStream> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let docs: Vec<_> = f.doc_attrs.iter().collect();

            match &f.category {
                FieldCategory::Option => {
                    // Extract inner type from Option<T>
                    let inner = option_inner(&f.ty);
                    quote! {
                        #(#docs)*
                        pub #ident: #inner,
                    }
                }
                FieldCategory::SoftOption => {
                    // Keep as Option<T> -- extract inner and re-wrap
                    let inner = option_inner(&f.ty);
                    quote! {
                        #(#docs)*
                        pub #ident: Option<#inner>,
                    }
                }
                FieldCategory::Nested { resolved_ty } => {
                    quote! {
                        #(#docs)*
                        pub #ident: #resolved_ty,
                    }
                }
            }
        })
        .collect();

    quote! { #(#field_tokens)* }
}

/// Generate FIELD_NAMES entries: each field's TOML key.
fn gen_field_names(fields: &[FieldMeta]) -> TokenStream {
    let entries: Vec<TokenStream> = fields
        .iter()
        .map(|f| {
            let key = f.toml_key();
            quote! { #key, }
        })
        .collect();

    quote! { #(#entries)* }
}
