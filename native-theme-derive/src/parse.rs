// Field metadata extraction from #[theme(...)] and #[theme_layer(...)] attributes.

use proc_macro2::Span;
use syn::{Attribute, Error, Field, Ident, LitStr, Result, Type};

/// How a field participates in validation and merge.
#[derive(Debug, Clone)]
pub(crate) enum FieldCategory {
    /// Required field: `Option<T>` in source, plain `T` in resolved.
    Option,
    /// Soft-optional field: `Option<T>` in both source and resolved (pass-through).
    SoftOption,
    /// Nested validated type (font, border).
    /// Border fields are dispatched via struct-level `border_kind` in gen_validate.
    Nested { resolved_ty: Box<Type> },
}

/// What range check to emit for a field.
#[derive(Debug, Clone)]
pub(crate) enum RangeCheck {
    /// `check_non_negative(self.field, ...)`
    NonNegative,
    /// `check_positive(self.field, ...)`
    Positive,
    /// `check_range_f32(self.field, min, max, ...)`
    Range { min: f64, max: f64 },
    /// `check_range_u16(self.field, min, max, ...)`
    RangeU16 { min: u16, max: u16 },
}

/// Parsed metadata for a single struct field.
#[derive(Debug, Clone)]
pub(crate) struct FieldMeta {
    pub ident: Ident,
    pub ty: Type,
    pub category: FieldCategory,
    pub serde_rename: Option<String>,
    pub range_check: Option<RangeCheck>,
    pub min_max_pair: Option<Ident>,
    pub inherit_from: Option<String>,
    pub doc_attrs: Vec<Attribute>,
}

/// Class-level border validation mode from `#[theme_layer(border_kind = "...")]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BorderKind {
    /// Borders use `ValidateNested` dispatch (default).
    Full,
    /// Borders use `require_border_partial`.
    Partial,
    /// Borders use `border_all_optional`.
    None,
}

/// Struct-level attributes parsed from `#[theme_layer(...)]`.
#[derive(Debug, Clone)]
pub(crate) struct LayerMeta {
    /// Border validation mode -- drives dispatch in gen_validate for border fields.
    pub border_kind: BorderKind,
    pub resolved_name: Option<Ident>,
    /// Skip inventory::submit! generation (for non-per-variant widgets like LayoutTheme).
    pub skip_inventory: bool,
    /// Explicit field-name override from `#[theme_layer(fields = "a, b_px, c")]`.
    ///
    /// When present, the `ThemeFields` derive uses this list verbatim instead
    /// of introspecting struct fields. Used for serde-proxy structs like
    /// `FontSpec` (which serializes through `FontSpecRaw` and therefore emits
    /// `size_pt`/`size_px` rather than `size`) where the user-facing struct's
    /// field names do not match the wire format.
    pub explicit_fields: Option<Vec<String>>,
}

/// Parse `#[theme_layer(...)]` attributes from the struct.
pub(crate) fn parse_layer_attrs(attrs: &[Attribute]) -> Result<LayerMeta> {
    let mut border_kind = BorderKind::Full;
    let mut resolved_name = None;
    let mut skip_inventory = false;
    let mut explicit_fields: Option<Vec<String>> = None;

    for attr in attrs {
        if !attr.path().is_ident("theme_layer") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("border_kind") {
                let value = meta.value()?;
                let lit: LitStr = value.parse()?;
                border_kind = match lit.value().as_str() {
                    "full" => BorderKind::Full,
                    "partial" => BorderKind::Partial,
                    "none" => BorderKind::None,
                    other => {
                        return Err(Error::new(
                            lit.span(),
                            format!("unknown border_kind: \"{other}\", expected \"full\", \"partial\", or \"none\""),
                        ))
                    }
                };
                Ok(())
            } else if meta.path.is_ident("resolved_name") {
                let value = meta.value()?;
                let lit: LitStr = value.parse()?;
                resolved_name = Some(Ident::new(&lit.value(), lit.span()));
                Ok(())
            } else if meta.path.is_ident("skip_inventory") {
                skip_inventory = true;
                Ok(())
            } else if meta.path.is_ident("fields") {
                let value = meta.value()?;
                let lit: LitStr = value.parse()?;
                let parsed: Vec<String> = lit
                    .value()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
                if parsed.is_empty() || parsed.iter().any(String::is_empty) {
                    return Err(Error::new(
                        lit.span(),
                        "fields list must be comma-separated non-empty names",
                    ));
                }
                explicit_fields = Some(parsed);
                Ok(())
            } else {
                Err(meta.error("unknown theme_layer attribute"))
            }
        })?;
    }

    Ok(LayerMeta {
        border_kind,
        resolved_name,
        skip_inventory,
        explicit_fields,
    })
}

/// Parse all fields of the input struct into `FieldMeta` entries.
pub(crate) fn parse_fields(fields: &syn::Fields) -> Result<Vec<FieldMeta>> {
    let named = match fields {
        syn::Fields::Named(named) => named,
        _ => {
            return Err(Error::new(
                Span::call_site(),
                "ThemeWidget requires named fields",
            ));
        }
    };

    named.named.iter().map(parse_one_field).collect()
}

fn parse_one_field(field: &Field) -> Result<FieldMeta> {
    let ident = field
        .ident
        .clone()
        .ok_or_else(|| Error::new(Span::call_site(), "unnamed field"))?;
    let ty = field.ty.clone();

    let mut category: Option<FieldCategory> = None;
    let mut serde_rename: Option<String> = None;
    let mut range_check: Option<RangeCheck> = None;
    let mut min_max_pair: Option<Ident> = None;
    let mut inherit_from: Option<String> = None;
    let mut resolved_ty: Option<Type> = None;
    let mut doc_attrs = Vec::new();

    for attr in &field.attrs {
        // Collect doc comments
        if attr.path().is_ident("doc") {
            doc_attrs.push(attr.clone());
            continue;
        }

        // Extract serde rename
        if attr.path().is_ident("serde") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") {
                    let value = meta.value()?;
                    let lit: LitStr = value.parse()?;
                    serde_rename = Some(lit.value());
                }
                Ok(())
            })?;
            continue;
        }

        // Parse #[theme(...)] attributes
        if attr.path().is_ident("theme") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("category") {
                    let value = meta.value()?;
                    let lit: LitStr = value.parse()?;
                    category = Some(match lit.value().as_str() {
                        "option" => FieldCategory::Option,
                        "soft_option" => FieldCategory::SoftOption,
                        other => {
                            return Err(Error::new(
                                lit.span(),
                                format!("unknown category: \"{other}\""),
                            ));
                        }
                    });
                    Ok(())
                } else if meta.path.is_ident("nested") {
                    // Mark as nested; resolved_type parsed separately
                    if category.is_none() {
                        category = Some(FieldCategory::Nested {
                            resolved_ty: Box::new(ty.clone()), // placeholder, replaced below
                        });
                    }
                    Ok(())
                } else if meta.path.is_ident("border_partial") {
                    Err(meta.error(
                        "border_partial is no longer needed -- use #[theme_layer(border_kind = \"partial\")] on the struct instead",
                    ))
                } else if meta.path.is_ident("border_optional") {
                    Err(meta.error(
                        "border_optional is no longer needed -- use #[theme_layer(border_kind = \"none\")] on the struct instead",
                    ))
                } else if meta.path.is_ident("resolved_type") {
                    let value = meta.value()?;
                    let lit: LitStr = value.parse()?;
                    let parsed: Type = syn::parse_str(&lit.value())?;
                    resolved_ty = Some(parsed);
                    Ok(())
                } else if meta.path.is_ident("check") {
                    let value = meta.value()?;
                    let lit: LitStr = value.parse()?;
                    range_check = Some(match lit.value().as_str() {
                        "non_negative" => RangeCheck::NonNegative,
                        "positive" => RangeCheck::Positive,
                        other => {
                            return Err(Error::new(
                                lit.span(),
                                format!("unknown check: \"{other}\""),
                            ));
                        }
                    });
                    Ok(())
                } else if meta.path.is_ident("range") {
                    let value = meta.value()?;
                    let lit: LitStr = value.parse()?;
                    let (min, max) = parse_range_f64(&lit)?;
                    range_check = Some(RangeCheck::Range { min, max });
                    Ok(())
                } else if meta.path.is_ident("range_u16") {
                    let value = meta.value()?;
                    let lit: LitStr = value.parse()?;
                    let (min, max) = parse_range_u16(&lit)?;
                    range_check = Some(RangeCheck::RangeU16 { min, max });
                    Ok(())
                } else if meta.path.is_ident("min_max_pair") {
                    let value = meta.value()?;
                    let lit: LitStr = value.parse()?;
                    min_max_pair = Some(Ident::new(&lit.value(), lit.span()));
                    Ok(())
                } else if meta.path.is_ident("inherit_from") {
                    let value = meta.value()?;
                    let lit: LitStr = value.parse()?;
                    inherit_from = Some(lit.value());
                    Ok(())
                } else {
                    Err(meta.error("unknown #[theme(...)] attribute"))
                }
            })?;
        }
    }

    // If resolved_type was specified, update the category
    if let Some(ref rt) = resolved_ty {
        match category {
            Some(FieldCategory::Nested { .. }) => {
                category = Some(FieldCategory::Nested {
                    resolved_ty: Box::new(rt.clone()),
                });
            }
            None => {
                // resolved_type without nested: treat as nested
                category = Some(FieldCategory::Nested {
                    resolved_ty: Box::new(rt.clone()),
                });
            }
            _ => {} // option/soft_option ignore resolved_type
        }
    }

    let category = category.unwrap_or(FieldCategory::Option);

    Ok(FieldMeta {
        ident,
        ty,
        category,
        serde_rename,
        range_check,
        min_max_pair,
        inherit_from,
        doc_attrs,
    })
}

/// Parse "MIN..=MAX" range for f64.
fn parse_range_f64(lit: &LitStr) -> Result<(f64, f64)> {
    let s = lit.value();
    let parts: Vec<&str> = s.split("..=").collect();
    if parts.len() != 2 {
        return Err(Error::new(
            lit.span(),
            "expected range format: \"MIN..=MAX\"",
        ));
    }
    let min: f64 = parts[0]
        .trim()
        .parse()
        .map_err(|_| Error::new(lit.span(), "invalid min value in range"))?;
    let max: f64 = parts[1]
        .trim()
        .parse()
        .map_err(|_| Error::new(lit.span(), "invalid max value in range"))?;
    Ok((min, max))
}

/// Parse "MIN..=MAX" range for u16.
fn parse_range_u16(lit: &LitStr) -> Result<(u16, u16)> {
    let s = lit.value();
    let parts: Vec<&str> = s.split("..=").collect();
    if parts.len() != 2 {
        return Err(Error::new(
            lit.span(),
            "expected range format: \"MIN..=MAX\"",
        ));
    }
    let min: u16 = parts[0]
        .trim()
        .parse()
        .map_err(|_| Error::new(lit.span(), "invalid min value in range"))?;
    let max: u16 = parts[1]
        .trim()
        .parse()
        .map_err(|_| Error::new(lit.span(), "invalid max value in range"))?;
    Ok((min, max))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: parse a single field by wrapping it in a struct and extracting.
    fn parse_field_from_tokens(tokens: proc_macro2::TokenStream) -> Result<FieldMeta> {
        let wrapper = quote::quote! {
            struct Wrapper { #tokens }
        };
        let item: syn::ItemStruct = syn::parse2(wrapper)?;
        let field = item
            .fields
            .iter()
            .next()
            .ok_or_else(|| Error::new(Span::call_site(), "no field in wrapper struct"))?;
        parse_one_field(field)
    }

    #[test]
    fn nested_field_is_parsed_as_nested() {
        let tokens = quote::quote! {
            #[theme(nested, resolved_type = "ResolvedBorderSpec")]
            pub border: Option<WidgetBorderSpec>
        };
        let meta = parse_field_from_tokens(tokens).expect("should parse");
        assert!(
            matches!(meta.category, FieldCategory::Nested { .. }),
            "expected Nested, got {:?}",
            meta.category
        );
    }

    #[test]
    fn border_partial_attribute_produces_error() {
        let tokens = quote::quote! {
            #[theme(border_partial, resolved_type = "ResolvedBorderSpec")]
            pub border: Option<WidgetBorderSpec>
        };
        let result = parse_field_from_tokens(tokens);
        assert!(
            result.is_err(),
            "border_partial should produce a compile error"
        );
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("border_partial is no longer needed"),
            "error message should guide to border_kind, got: {err_msg}"
        );
    }

    #[test]
    fn border_optional_attribute_produces_error() {
        let tokens = quote::quote! {
            #[theme(border_optional, resolved_type = "ResolvedBorderSpec")]
            pub border: Option<WidgetBorderSpec>
        };
        let result = parse_field_from_tokens(tokens);
        assert!(
            result.is_err(),
            "border_optional should produce a compile error"
        );
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("border_optional is no longer needed"),
            "error message should guide to border_kind, got: {err_msg}"
        );
    }

    // === explicit_fields parsing (Phase 93-05 G5) ===

    /// Helper to parse struct-level `#[theme_layer(...)]` attributes from a token stream.
    fn parse_layer_from_tokens(tokens: proc_macro2::TokenStream) -> Result<LayerMeta> {
        let wrapper = quote::quote! {
            #tokens
            struct Wrapper { x: i32 }
        };
        let item: syn::ItemStruct = syn::parse2(wrapper)?;
        parse_layer_attrs(&item.attrs)
    }

    #[test]
    fn explicit_fields_absent_means_none() {
        let tokens = quote::quote! {};
        let layer = parse_layer_from_tokens(tokens).expect("should parse");
        assert!(layer.explicit_fields.is_none());
    }

    #[test]
    fn explicit_fields_parsed_as_trimmed_list() {
        let tokens = quote::quote! {
            #[theme_layer(fields = "family, size_pt, size_px, weight, style, color")]
        };
        let layer = parse_layer_from_tokens(tokens).expect("should parse");
        assert_eq!(
            layer.explicit_fields.as_deref(),
            Some(
                [
                    "family".to_string(),
                    "size_pt".to_string(),
                    "size_px".to_string(),
                    "weight".to_string(),
                    "style".to_string(),
                    "color".to_string()
                ]
                .as_slice()
            )
        );
    }

    #[test]
    fn explicit_fields_single_name_ok() {
        let tokens = quote::quote! {
            #[theme_layer(fields = "only")]
        };
        let layer = parse_layer_from_tokens(tokens).expect("should parse");
        assert_eq!(
            layer.explicit_fields.as_deref(),
            Some(["only".to_string()].as_slice())
        );
    }

    #[test]
    fn explicit_fields_empty_string_rejected() {
        let tokens = quote::quote! {
            #[theme_layer(fields = "")]
        };
        let result = parse_layer_from_tokens(tokens);
        assert!(result.is_err(), "empty fields string must error");
    }

    #[test]
    fn explicit_fields_trailing_comma_rejected() {
        let tokens = quote::quote! {
            #[theme_layer(fields = "a, b,")]
        };
        let result = parse_layer_from_tokens(tokens);
        assert!(result.is_err(), "trailing comma produces empty entry");
    }

    #[test]
    fn explicit_fields_coexists_with_skip_inventory() {
        let tokens = quote::quote! {
            #[theme_layer(skip_inventory, fields = "a, b")]
        };
        let layer = parse_layer_from_tokens(tokens).expect("should parse");
        assert!(layer.skip_inventory);
        assert_eq!(
            layer.explicit_fields.as_deref(),
            Some(["a".to_string(), "b".to_string()].as_slice())
        );
    }
}
