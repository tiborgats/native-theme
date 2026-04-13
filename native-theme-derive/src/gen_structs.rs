// Generation of the Resolved struct and FIELD_NAMES const.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::parse::{FieldCategory, FieldMeta, LayerMeta};

/// Generate the Resolved struct and FIELD_NAMES const on the Option struct.
pub(crate) fn gen_structs(
    opt_name: &Ident,
    fields: &[FieldMeta],
    layer: &LayerMeta,
    doc_attrs: &[syn::Attribute],
) -> TokenStream {
    let resolved_name = layer
        .resolved_name
        .clone()
        .unwrap_or_else(|| format_ident!("Resolved{}", opt_name));

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
                    let inner = extract_option_inner(&f.ty);
                    quote! {
                        #(#docs)*
                        pub #ident: #inner,
                    }
                }
                FieldCategory::SoftOption => {
                    // Keep as Option<T> -- extract inner and re-wrap
                    let inner = extract_option_inner(&f.ty);
                    quote! {
                        #(#docs)*
                        pub #ident: Option<#inner>,
                    }
                }
                FieldCategory::Nested { resolved_ty }
                | FieldCategory::BorderPartial { resolved_ty }
                | FieldCategory::BorderOptional { resolved_ty } => {
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

/// Generate FIELD_NAMES entries: use serde_rename if present, else stringify.
fn gen_field_names(fields: &[FieldMeta]) -> TokenStream {
    let entries: Vec<TokenStream> = fields
        .iter()
        .map(|f| {
            if let Some(ref rename) = f.serde_rename {
                quote! { #rename, }
            } else {
                let name = f.ident.to_string();
                quote! { #name, }
            }
        })
        .collect();

    quote! { #(#entries)* }
}

/// Extract T from Option<T>. Falls back to the original type if not Option.
fn extract_option_inner(ty: &syn::Type) -> syn::Type {
    if let syn::Type::Path(type_path) = ty
        && let Some(seg) = type_path.path.segments.last()
        && seg.ident == "Option"
        && let syn::PathArguments::AngleBracketed(args) = &seg.arguments
        && let Some(syn::GenericArgument::Type(inner)) = args.args.first()
    {
        return inner.clone();
    }
    // Not Option<T>, return as-is
    ty.clone()
}
