// native-theme-derive: proc-macro crate for ThemeWidget derive.
//
// Generates paired Option/Resolved struct hierarchies, FIELD_NAMES,
// merge/is_empty, validate_widget, and check_ranges from field attributes.

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod gen_merge;
mod gen_ranges;
mod gen_structs;
mod gen_validate;
mod parse;

/// Derive macro that generates a companion Resolved struct and impl blocks
/// for theme widget types.
///
/// # Struct-level attributes
///
/// - `#[theme_layer(border_kind = "full"|"partial"|"none")]` -- border validation mode
/// - `#[theme_layer(resolved_name = "CustomName")]` -- override resolved struct name
///
/// # Field-level attributes
///
/// - `#[theme(category = "option"|"soft_option")]` -- field merge/validation category (default: "option")
/// - `#[theme(nested, resolved_type = "ResolvedFontSpec")]` -- nested validated type
/// - `#[theme(border_partial, resolved_type = "ResolvedBorderSpec")]` -- border with partial inheritance
/// - `#[theme(border_optional, resolved_type = "ResolvedBorderSpec")]` -- border with no required fields
/// - `#[theme(check = "non_negative"|"positive")]` -- range check
/// - `#[theme(range = "0.0..=1.0")]` -- f32 range check
/// - `#[theme(range_u16 = "100..=900")]` -- u16 range check
/// - `#[theme(min_max_pair = "other_field")]` -- min/max pair validation
/// - `#[theme(inherit_from = "defaults.accent_color")]` -- inheritance source (Plan 02)
#[proc_macro_derive(ThemeWidget, attributes(theme, theme_layer))]
pub fn derive_theme_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive_inner(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn derive_inner(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let opt_name = &input.ident;

    // Parse struct-level attributes
    let layer = parse::parse_layer_attrs(&input.attrs)?;

    // Parse field metadata
    let fields = match &input.data {
        syn::Data::Struct(data) => parse::parse_fields(&data.fields)?,
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "ThemeWidget can only be derived on structs",
            ))
        }
    };

    // Collect doc attributes from the input struct
    let doc_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("doc"))
        .cloned()
        .collect();

    // Generate code
    let structs = gen_structs::gen_structs(opt_name, &fields, &layer, &doc_attrs);
    let merge = gen_merge::gen_merge(opt_name, &fields);
    let validate = gen_validate::gen_validate(opt_name, &fields, &layer);
    let ranges = gen_ranges::gen_ranges(opt_name, &fields, &layer);

    Ok(quote::quote! {
        #structs
        #merge
        #validate
        #ranges
    })
}
