// native-theme-derive: proc-macro crate for ThemeWidget and ThemeFields derives.
//
// `ThemeWidget` generates paired Option/Resolved struct hierarchies, FIELD_NAMES,
// merge/is_empty, validate_widget, and check_ranges from field attributes.
//
// `ThemeFields` (Phase 93-05 G5) generates a single `inventory::submit!` call
// that registers a plain struct's serialized field names in the
// `crate::resolve::FieldInfo` registry for TOML linting. Replaces the
// hand-authored `FIELD_NAMES` constants on non-widget model types.

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod gen_inherit;
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
/// - `#[theme_layer(skip_inventory)]` -- skip inventory::submit! for non-per-variant widgets
/// - `#[theme_inherit(border_kind = "full"|"full_lg"|"partial")]` -- border INHERITANCE mode
///   (Phase 94-01 G6; parallel to theme_layer.border_kind which drives validation)
/// - `#[theme_inherit(font = "<field>")]` -- font field that inherits from `defaults.font`
///   (repeatable: list declares item_font + header_font, dialog declares title_font + body_font)
///
/// # Field-level attributes
///
/// - `#[theme(category = "option"|"soft_option")]` -- field merge/validation category (default: "option")
/// - `#[theme(nested, resolved_type = "ResolvedFontSpec")]` -- nested validated type
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

    // Generate code
    let structs = gen_structs::gen_structs(opt_name, &fields, &layer, &doc_attrs);
    let merge = gen_merge::gen_merge(opt_name, &fields);
    let validate = gen_validate::gen_validate(opt_name, &fields, &layer);
    let ranges = gen_ranges::gen_ranges(opt_name, &fields, &layer);
    let inherit = gen_inherit::gen_inherit(opt_name, &fields, &layer);
    let border_inherit = gen_inherit::gen_border_inherit(opt_name, &inherit_meta);
    let font_inherit = gen_inherit::gen_font_inherit(opt_name, &inherit_meta);
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

    let name_str = opt_name.to_string();
    let widget_name = to_snake_case(name_str.strip_suffix("Theme").unwrap_or(&name_str));

    quote::quote! {
        inventory::submit!(crate::resolve::WidgetFieldInfo {
            widget_name: #widget_name,
            field_names: #opt_name::FIELD_NAMES,
        });
    }
}

/// Convert a PascalCase identifier to snake_case.
fn to_snake_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len().saturating_add(4));
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(ch.to_ascii_lowercase());
    }
    result
}

/// Derive macro that registers a plain struct's serialized field names in the
/// `crate::resolve::FieldInfo` inventory for TOML linting (Phase 93-05 G5).
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
/// This derive emits ONLY an `inventory::submit!` call; it does not emit a
/// `FIELD_NAMES` associated constant. To also get widget-style codegen
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
        parse::parse_fields(&data.fields)?
            .iter()
            .map(|f| {
                f.serde_rename
                    .clone()
                    .unwrap_or_else(|| f.ident.to_string())
            })
            .collect()
    };

    let struct_name_str = struct_name.to_string();
    let entries: Vec<proc_macro2::TokenStream> =
        field_names.iter().map(|n| quote::quote! { #n, }).collect();

    // Emit at item level directly -- mirrors the pattern used by the widget
    // derive at line 111 above. The inventory registry path
    // `crate::resolve::FieldInfo` works because this derive is only ever
    // consumed inside the `native_theme` crate.
    Ok(quote::quote! {
        inventory::submit!(crate::resolve::FieldInfo {
            struct_name: #struct_name_str,
            field_names: &[#(#entries)*],
        });
    })
}
