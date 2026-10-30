//! Derive macros internal to [`native-theme`](https://docs.rs/native-theme).
//!
//! **Do not depend on this crate directly; depend on `native-theme`.** The
//! derives exist for `native-theme`'s own model types: their expansions name
//! `crate::resolve::*` paths (`WidgetFieldInfo`, `FieldInfo`,
//! `validate_helpers`, the inheritance-rule registries) that exist only
//! inside `native-theme`, so they compile nowhere else.
//!
//! - [`ThemeWidget`](derive@ThemeWidget) generates, from one annotated
//!   struct, its paired `Resolved` struct, `FIELD_NAMES`, `merge` and
//!   `is_empty`, `validate_widget`, `check_ranges` and the struct's
//!   inheritance rules; its documentation is the reference for the
//!   `#[theme]`, `#[theme_layer]` and `#[theme_inherit]` attributes.
//! - [`ThemeFields`](derive@ThemeFields) registers a plain struct's
//!   serialized field names for TOML linting.
//! - [`ThemeParts`](derive@ThemeParts) generates the 0.8.0 model's blocks'
//!   patches, resolved types, merges and field tables; its expansions name
//!   `crate::next::*` paths (the blocks' traits and merge helpers, the
//!   field-path registry) that exist only inside `native-theme`, so it too
//!   compiles nowhere else, and `native-theme` re-exports none of these
//!   derives.

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod gen_inherit;
mod gen_merge;
mod gen_ranges;
mod gen_structs;
mod gen_validate;
mod parse;
mod parts;
mod util;

/// Derive macro that generates a companion Resolved struct and impl blocks
/// for theme widget types.
///
/// # Struct-level attributes
///
/// - `#[theme_layer(border_kind = "full"|"partial"|"none")]` -- border validation mode
/// - `#[theme_layer(resolved_name = "CustomName")]` -- override resolved struct name
/// - `#[theme_layer(skip_inventory)]` -- emit no `inventory::submit!` registrations
///   (widget fields, uniform, border and font inheritance rules), for structs that
///   are not per-variant widgets
/// - `#[theme_inherit(border_kind = "full"|"full_lg"|"partial")]` -- which border
///   sub-fields inherit from `defaults.border` (`full_lg` takes `corner_radius_lg`,
///   `partial` only the colour and line width); a separate concern from
///   `theme_layer`'s `border_kind`, which drives validation
/// - `#[theme_inherit(font = "<field>")]` -- font field that inherits from `defaults.font`
///   (repeatable: list declares item_font + header_font, dialog declares title_font + body_font)
///
/// # Field-level attributes
///
/// - `#[theme(category = "option"|"soft_option")]` -- field merge/validation category (default: "option")
/// - `#[theme(nested, resolved_type = "ResolvedFontSpec")]` -- nested validated type
///   (`resolved_type` implies `nested`; `nested` alone keeps the field's own type)
/// - `#[theme(check = "non_negative"|"positive")]` -- range check
/// - `#[theme(range = "0.0..=1.0")]` -- f32 range check
/// - `#[theme(range_u16 = "100..=900")]` -- u16 range check
/// - `#[theme(min_max_pair = "other_field")]` -- min/max pair validation
/// - `#[theme(inherit_from = "defaults.accent_color")]` -- uniform inheritance source from defaults
#[proc_macro_derive(ThemeWidget, attributes(theme, theme_layer, theme_inherit))]
pub fn derive_theme_widget(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive_inner(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn derive_inner(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let opt_name = &input.ident;

    // Parse struct-level attributes (both families, parse orthogonally)
    let layer = parse::parse_layer_attrs(&input.attrs)?;
    let inherit_meta = parse::parse_inherit_attrs(&input.attrs)?;

    // Parse field metadata
    let fields = match &input.data {
        syn::Data::Struct(data) => parse::parse_fields(&data.fields)?,
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "ThemeWidget can only be derived on structs",
            ));
        }
    };

    // Collect doc attributes from the input struct
    let doc_attrs: Vec<_> = input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("doc"))
        .cloned()
        .collect();

    let resolved_name = layer
        .resolved_name
        .clone()
        .unwrap_or_else(|| quote::format_ident!("Resolved{}", opt_name));

    // Generate code
    let structs = gen_structs::gen_structs(opt_name, &resolved_name, &fields, &doc_attrs);
    let merge = gen_merge::gen_merge(opt_name, &fields);
    let validate = gen_validate::gen_validate(opt_name, &resolved_name, &fields, &layer);
    let ranges = gen_ranges::gen_ranges(&resolved_name, &fields);
    let inherit = gen_inherit::gen_inherit(opt_name, &fields, &layer);
    let border_inherit = gen_inherit::gen_border_inherit(opt_name, &inherit_meta, &layer);
    let font_inherit = gen_inherit::gen_font_inherit(opt_name, &inherit_meta, &layer);
    let inventory = gen_inventory_submit(opt_name, &layer);

    Ok(quote::quote! {
        #structs
        #merge
        #validate
        #ranges
        #inherit
        #border_inherit
        #font_inherit
        #inventory
    })
}

/// Generate `inventory::submit!` call for widget registry.
///
/// Derives the widget_name from the struct name by stripping "Theme" suffix
/// and converting to snake_case (e.g., `ButtonTheme` -> `"button"`,
/// `SegmentedControlTheme` -> `"segmented_control"`).
///
/// Skips generation if the struct has `#[theme_layer(skip_inventory)]`.
fn gen_inventory_submit(
    opt_name: &syn::Ident,
    layer: &parse::LayerMeta,
) -> proc_macro2::TokenStream {
    if layer.skip_inventory {
        return proc_macro2::TokenStream::new();
    }

    let widget_name = util::widget_name(opt_name);

    quote::quote! {
        inventory::submit!(crate::resolve::WidgetFieldInfo {
            widget_name: #widget_name,
            field_names: #opt_name::FIELD_NAMES,
        });
    }
}

/// Derive macro for the 0.8.0 model's blocks (S2, S6.1).
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
///   `Option<Sides<Option<T>>>`, member by member);
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
/// # Type attributes
///
/// - `#[parts(block)]`: block mode (required).
/// - `#[parts(append)]`: a struct block patched by `Set` or `Append`.
/// - `#[parts(own_resolved)]`: `Resolved<T>` is written by hand.
/// - `#[parts(resolved_patch)]`: a struct block's patch resolved,
///   `Resolved<T>Patch`, each field `Option<resolved field>` (`None`: the
///   patch leaves it), for a field holding the block's patch.
///
/// # Field attributes
///
/// - `#[parts(may_be_absent)]`: the `Value` may be `Absent` (S6.3).
/// - `#[parts(auto)]`: the `Value<Px>` or `Value<FontSize>` may be `Auto`
///   (S6.3).
/// - `#[parts(block = "fields" | "append" | "set")]`: the field holds a
///   block (`B`, `Value<B>`, `Vec<B>`, `Option<B>`, or a map `Vec<(K, B)>`
///   or `Vec<(K, Value<B>)>`) of that kind: a struct block, an append block
///   or an enum block. An append block is held as `Value<B>` or a map's
///   `Value<B>`, and an `Option<B>` is `set` whole: an `append` elsewhere,
///   or `fields` on an `Option`, would act as `set`, so both are refused.
/// - `#[parts(keyed)]`: a list whose items merge by their own key, through
///   the item type's `KeyedItem` impl.
/// - `#[parts(patch)]`: the field holds a struct block's patch, `<B>Patch`
///   (a keyframe's `PartStylePatch`): it resolves to `Resolved<B>Patch`
///   (`B` marked `resolved_patch`), and its table entry reaches `B`'s table
///   through a function, as `B` can hold the field itself.
#[proc_macro_derive(ThemeParts, attributes(parts))]
pub fn derive_theme_parts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match parts::derive(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derive macro that registers a plain struct's serialized field names in the
/// `crate::resolve::FieldInfo` inventory for TOML linting.
///
/// By default the macro introspects the struct's fields and honours any
/// `#[serde(rename = "...")]` attributes (so `corner_radius` annotated with
/// `#[serde(rename = "corner_radius_px")]` is registered under the wire name).
///
/// # Struct-level attributes
///
/// - `#[theme_layer(fields = "a, b_px, c")]` -- explicit field-name list.
///   Use this for serde-proxy structs (like `FontSpec`, which serializes
///   through a private `FontSpecRaw` proxy) where the user-facing struct's
///   field names do not match the wire format.
///
/// # Example
///
/// ```ignore
/// use native_theme_derive::ThemeFields;
///
/// // Introspection path: serde renames picked up automatically.
/// #[derive(ThemeFields, serde::Serialize, serde::Deserialize)]
/// pub struct Simple {
///     pub a: i32,
///     #[serde(rename = "bar")]
///     pub b: i32,
/// }
/// // Registers { struct_name: "Simple", field_names: &["a", "bar"] }.
///
/// // Explicit-override path: used when serde proxy field names differ.
/// #[derive(ThemeFields, serde::Serialize, serde::Deserialize)]
/// #[serde(try_from = "HasProxyRaw", into = "HasProxyRaw")]
/// #[theme_layer(fields = "x, y_px, z")]
/// pub struct HasProxy { /* ... */ }
/// ```
///
/// On the introspection path it also emits `toml_key(field)`, which maps a
/// field's name to its TOML key, so validation can name the key a theme
/// author writes.
///
/// This derive does not emit a `FIELD_NAMES` associated constant. To also
/// get widget-style codegen
/// (Resolved pair + merge + validate + `FIELD_NAMES`), combine with
/// `#[derive(ThemeWidget)]`.
#[proc_macro_derive(ThemeFields, attributes(theme_layer))]
pub fn derive_theme_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive_fields_inner(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn derive_fields_inner(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name = &input.ident;
    let layer = parse::parse_layer_attrs(&input.attrs)?;

    let mut renames = Vec::new();
    let field_names: Vec<String> = if let Some(ref explicit) = layer.explicit_fields {
        explicit.clone()
    } else {
        let data = match &input.data {
            syn::Data::Struct(s) => s,
            _ => {
                return Err(syn::Error::new_spanned(
                    &input.ident,
                    "ThemeFields can only be derived on structs",
                ));
            }
        };
        let fields = parse::parse_fields(&data.fields)?;
        renames = fields
            .iter()
            .filter_map(|f| {
                let rename = f.serde_rename.as_ref()?;
                Some((f.ident.to_string(), rename.clone()))
            })
            .collect();
        fields.iter().map(parse::FieldMeta::toml_key).collect()
    };

    let struct_name_str = struct_name.to_string();
    let entries: Vec<proc_macro2::TokenStream> =
        field_names.iter().map(|n| quote::quote! { #n, }).collect();

    // An explicit field list has no field-to-key mapping: no `toml_key`.
    let toml_key = if layer.explicit_fields.is_some() {
        proc_macro2::TokenStream::new()
    } else {
        let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
        let arms = renames
            .iter()
            .map(|(name, key)| quote::quote! { #name => #key, });
        let body = if renames.is_empty() {
            quote::quote! { field }
        } else {
            quote::quote! {
                match field {
                    #(#arms)*
                    _ => field,
                }
            }
        };
        quote::quote! {
            impl #impl_generics #struct_name #ty_generics #where_clause {
                /// The TOML key of the field named `field`: its serde
                /// rename, else `field` itself.
                #[allow(dead_code)]
                pub(crate) fn toml_key(field: &'static str) -> &'static str {
                    #body
                }
            }
        }
    };

    // Emit at item level directly -- mirrors `gen_inventory_submit` above.
    // The inventory registry path
    // `crate::resolve::FieldInfo` works because this derive is only ever
    // consumed inside the `native_theme` crate.
    Ok(quote::quote! {
        inventory::submit!(crate::resolve::FieldInfo {
            struct_name: #struct_name_str,
            field_names: &[#(#entries)*],
        });
        #toml_key
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_inventory_skips_inheritance_registrations() {
        let input: DeriveInput = syn::parse_quote! {
            #[theme_layer(skip_inventory)]
            #[theme_inherit(border_kind = "full", font = "font")]
            struct ProbeTheme {
                #[theme(nested, resolved_type = "ResolvedWidgetBorder")]
                border: Option<WidgetBorderSpec>,
                #[theme(nested, resolved_type = "ResolvedFontSpec")]
                font: Option<FontSpec>,
            }
        };
        let output = derive_inner(input).expect("should derive").to_string();
        assert!(output.contains("resolve_border_from_defaults"), "{output}");
        assert!(output.contains("resolve_font_from_defaults"), "{output}");
        assert!(!output.contains("inventory"), "{output}");
    }
}
