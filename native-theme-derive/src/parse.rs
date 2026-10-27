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
    pub inherit_from: Option<InheritFrom>,
    pub doc_attrs: Vec<Attribute>,
}

/// A uniform inheritance source from `#[theme(inherit_from = "defaults.X")]`.
#[derive(Debug, Clone)]
pub(crate) struct InheritFrom {
    /// The path as written: `defaults.border.color`.
    pub path: String,
    /// The path's segments after `defaults.`: `border`, `color`.
    pub segments: Vec<Ident>,
}

impl FieldMeta {
    /// The key a theme author writes for this field in TOML: its serde
    /// rename, else its name.
    pub(crate) fn toml_key(&self) -> String {
        self.serde_rename
            .clone()
            .unwrap_or_else(|| self.ident.to_string())
    }
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

/// Class-level border INHERITANCE mode from `#[theme_inherit(border_kind = "...")]`.
///
/// Parallel to [`BorderKind`] but a DIFFERENT concern: this enum drives which
/// fields the generated `resolve_border_from_defaults()` method inherits from
/// `DefaultsBorderSpec`. `BorderKind` drives VALIDATION dispatch; this drives
/// RESOLUTION. They live on different attributes (`theme_layer` vs
/// `theme_inherit`) and must stay independent to avoid conflating the two
/// concerns.
///
/// Per-widget assignments:
/// - `Full`: 10 widgets — button, input, checkbox, tooltip, progress_bar,
///   toolbar, list, combo_box, segmented_control, expander.
/// - `FullLg`: 3 widgets — window, popover, dialog (use `corner_radius_lg`).
/// - `Partial`: 2 widgets — sidebar, status_bar (color + line_width only).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BorderInheritanceKind {
    /// All 4 sub-fields inherited: color, corner_radius, line_width, shadow_enabled.
    Full,
    /// All 4 sub-fields inherited; `corner_radius` comes from
    /// `defaults.border.corner_radius_lg` instead of `defaults.border.corner_radius`.
    FullLg,
    /// Only 2 sub-fields inherited: color, line_width.
    Partial,
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
                resolved_name = Some(parse_ident(&lit.value(), "resolved_name", &lit)?);
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

/// Struct-level INHERITANCE metadata parsed from `#[theme_inherit(...)]`.
///
/// Parallel to (but not nested inside) [`LayerMeta`]. Lives on a separate
/// attribute because validation-side concerns (`#[theme_layer]`) and
/// resolution-side concerns (`#[theme_inherit]`) stay orthogonal.
///
/// Multiple `#[theme_inherit]` attributes on the same struct are additive —
/// list declares both `item_font` and `header_font`, dialog declares both
/// `title_font` and `body_font`. At most one `border_kind` across all
/// attributes (border inheritance is single-valued per widget).
#[derive(Debug, Clone, Default)]
pub(crate) struct InheritMeta {
    /// Border inheritance mode, if the struct declares `border_kind = "..."`.
    pub border_kind: Option<BorderInheritanceKind>,
    /// Font field names to inherit from `defaults.font`. 0, 1, or 2 entries.
    pub font: Vec<Ident>,
}

/// Parse all `#[theme_inherit(...)]` attributes on the struct.
///
/// Accumulates results additively across multiple attributes. Grammar:
/// ```text
/// #[theme_inherit(border_kind = "full" | "full_lg" | "partial")]
/// #[theme_inherit(font = "font")]
/// #[theme_inherit(font = "title_font")]  // dialog's second font attr
/// ```
///
/// Both keys are optional within an attribute; the two keys may co-occur:
/// `#[theme_inherit(border_kind = "full", font = "font")]`.
pub(crate) fn parse_inherit_attrs(attrs: &[Attribute]) -> Result<InheritMeta> {
    let mut meta = InheritMeta::default();

    for attr in attrs {
        if !attr.path().is_ident("theme_inherit") {
            continue;
        }
        attr.parse_nested_meta(|nested| {
            if nested.path.is_ident("border_kind") {
                let value = nested.value()?;
                let lit: LitStr = value.parse()?;
                let kind = match lit.value().as_str() {
                    "full" => BorderInheritanceKind::Full,
                    "full_lg" => BorderInheritanceKind::FullLg,
                    "partial" => BorderInheritanceKind::Partial,
                    other => {
                        return Err(Error::new(
                            lit.span(),
                            format!(
                                "unknown theme_inherit border_kind: \"{other}\", \
                                 expected \"full\", \"full_lg\", or \"partial\""
                            ),
                        ));
                    }
                };
                meta.border_kind = Some(kind);
                Ok(())
            } else if nested.path.is_ident("font") {
                let value = nested.value()?;
                let lit: LitStr = value.parse()?;
                meta.font.push(parse_ident(&lit.value(), "font", &lit)?);
                Ok(())
            } else {
                Err(nested
                    .error("unknown theme_inherit attribute, expected \"border_kind\" or \"font\""))
            }
        })?;
    }

    Ok(meta)
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
    let mut nested = false;
    let mut serde_rename: Option<String> = None;
    let mut range_check: Option<RangeCheck> = None;
    let mut min_max_pair: Option<Ident> = None;
    let mut inherit_from: Option<InheritFrom> = None;
    let mut resolved_ty: Option<Type> = None;
    let mut doc_attrs = Vec::new();

    for attr in &field.attrs {
        // Collect doc comments
        if attr.path().is_ident("doc") {
            doc_attrs.push(attr.clone());
            continue;
        }

        // Extract serde rename. Tolerate any other serde sub-attributes
        // (e.g. `default`, `skip_serializing_if = "..."`, `try_from = "..."`)
        // by consuming their values when present -- otherwise
        // `parse_nested_meta` fails to find the following `,` separator.
        if attr.path().is_ident("serde") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") {
                    let value = meta.value()?;
                    let lit: LitStr = value.parse()?;
                    serde_rename = Some(lit.value());
                    Ok(())
                } else {
                    // Unknown serde sub-attribute: consume its value if any,
                    // otherwise move on. `meta.value()` succeeds when an `=` is
                    // present; it fails for flag-style entries like `default`.
                    if let Ok(v) = meta.value() {
                        // Consume the expression without interpreting it.
                        let _: syn::Expr = v.parse()?;
                    }
                    Ok(())
                }
            })?;
            continue;
        }

        // Parse #[theme(...)] attributes
        if attr.path().is_ident("theme") {
            attr.parse_nested_meta(|meta| {
                let is_range_check = meta.path.is_ident("check")
                    || meta.path.is_ident("range")
                    || meta.path.is_ident("range_u16");
                if is_range_check && range_check.is_some() {
                    return Err(meta.error("only one of check, range and range_u16 may be given"));
                }
                if meta.path.is_ident("category") {
                    if nested || resolved_ty.is_some() {
                        return Err(meta.error(NESTED_CATEGORY_CONFLICT));
                    }
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
                    if category.is_some() {
                        return Err(meta.error(NESTED_CATEGORY_CONFLICT));
                    }
                    nested = true;
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
                    if category.is_some() {
                        return Err(meta.error(NESTED_CATEGORY_CONFLICT));
                    }
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
                    let (min, max) = parse_range::<f64>(&lit)?;
                    // The generated check compares in f32.
                    if !(min as f32).is_finite() || !(max as f32).is_finite() {
                        return Err(Error::new(
                            lit.span(),
                            "range bounds must be finite f32 values",
                        ));
                    }
                    range_check = Some(RangeCheck::Range { min, max });
                    Ok(())
                } else if meta.path.is_ident("range_u16") {
                    let value = meta.value()?;
                    let lit: LitStr = value.parse()?;
                    let (min, max) = parse_range::<u16>(&lit)?;
                    range_check = Some(RangeCheck::RangeU16 { min, max });
                    Ok(())
                } else if meta.path.is_ident("min_max_pair") {
                    let value = meta.value()?;
                    let lit: LitStr = value.parse()?;
                    min_max_pair = Some(parse_ident(&lit.value(), "min_max_pair", &lit)?);
                    Ok(())
                } else if meta.path.is_ident("inherit_from") {
                    let value = meta.value()?;
                    let lit: LitStr = value.parse()?;
                    inherit_from = Some(parse_inherit_from(&lit)?);
                    Ok(())
                } else {
                    Err(meta.error("unknown #[theme(...)] attribute"))
                }
            })?;
        }
    }

    // `resolved_type` implies `nested`; `nested` alone keeps the field's type.
    let category = match (resolved_ty, nested) {
        (Some(rt), _) => FieldCategory::Nested {
            resolved_ty: Box::new(rt),
        },
        (None, true) => FieldCategory::Nested {
            resolved_ty: Box::new(ty.clone()),
        },
        (None, false) => category.unwrap_or(FieldCategory::Option),
    };

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

/// Parse a "MIN..=MAX" range whose MIN does not exceed its MAX.
fn parse_range<T: std::str::FromStr + PartialOrd>(lit: &LitStr) -> Result<(T, T)> {
    let s = lit.value();
    let parts: Vec<&str> = s.split("..=").collect();
    let [min_s, max_s] = parts.as_slice() else {
        return Err(Error::new(
            lit.span(),
            "expected range format: \"MIN..=MAX\"",
        ));
    };
    let min: T = min_s
        .trim()
        .parse()
        .map_err(|_| Error::new(lit.span(), "invalid min value in range"))?;
    let max: T = max_s
        .trim()
        .parse()
        .map_err(|_| Error::new(lit.span(), "invalid max value in range"))?;
    if min > max {
        return Err(Error::new(lit.span(), "range MIN exceeds its MAX"));
    }
    Ok((min, max))
}

/// Parse `value`, attribute `key`'s literal `lit` or a part of it, as an
/// identifier. An error is spanned at the literal.
fn parse_ident(value: &str, key: &str, lit: &LitStr) -> Result<Ident> {
    let mut ident: Ident = syn::parse_str(value).map_err(|_| {
        Error::new(
            lit.span(),
            format!("{key}: \"{value}\" is not a valid Rust identifier"),
        )
    })?;
    ident.set_span(lit.span());
    Ok(ident)
}

/// Parse an `inherit_from` path: `defaults.` and one or more identifiers
/// separated by dots.
fn parse_inherit_from(lit: &LitStr) -> Result<InheritFrom> {
    let path = lit.value();
    let Some(rest) = path.strip_prefix("defaults.") else {
        return Err(Error::new(
            lit.span(),
            format!("inherit_from: \"{path}\" does not start with \"defaults.\""),
        ));
    };
    let segments = rest
        .split('.')
        .map(|segment| parse_ident(segment, "inherit_from", lit))
        .collect::<Result<Vec<_>>>()?;
    Ok(InheritFrom { path, segments })
}

/// A field's `category` cannot be combined with `nested` or `resolved_type`.
const NESTED_CATEGORY_CONFLICT: &str =
    "category conflicts with nested or resolved_type: a nested field has no category";

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
            #[theme(nested, resolved_type = "ResolvedWidgetBorder")]
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
            #[theme(border_partial, resolved_type = "ResolvedWidgetBorder")]
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
            #[theme(border_optional, resolved_type = "ResolvedWidgetBorder")]
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

    /// Helper: the error message parsing a field yields; fails the test if
    /// it parses.
    fn field_error(tokens: proc_macro2::TokenStream) -> String {
        parse_field_from_tokens(tokens)
            .expect_err("the attribute must not parse")
            .to_string()
    }

    #[test]
    fn min_max_pair_not_an_identifier_errors() {
        let err = field_error(quote::quote! {
            #[theme(min_max_pair = "max width")]
            pub min_width: Option<f32>
        });
        assert!(err.contains("min_max_pair"), "got: {err}");
    }

    #[test]
    fn inherit_from_path_parsed_into_segments() {
        let meta = parse_field_from_tokens(quote::quote! {
            #[theme(inherit_from = "defaults.border.color")]
            pub color: Option<Rgba>
        })
        .expect("should parse");
        let source = meta.inherit_from.expect("inherit_from is set");
        assert_eq!(source.path, "defaults.border.color");
        let segments: Vec<String> = source.segments.iter().map(|i| i.to_string()).collect();
        assert_eq!(segments, vec!["border".to_string(), "color".to_string()]);
    }

    #[test]
    fn inherit_from_without_defaults_prefix_errors() {
        let err = field_error(quote::quote! {
            #[theme(inherit_from = "accent_color")]
            pub color: Option<Rgba>
        });
        assert!(err.contains("defaults."), "got: {err}");
    }

    #[test]
    fn inherit_from_empty_segment_errors() {
        let err = field_error(quote::quote! {
            #[theme(inherit_from = "defaults..color")]
            pub color: Option<Rgba>
        });
        assert_eq!(err, "inherit_from: \"\" is not a valid Rust identifier");
    }

    #[test]
    fn inherit_from_non_identifier_segment_errors() {
        let err = field_error(quote::quote! {
            #[theme(inherit_from = "defaults.border-color")]
            pub color: Option<Rgba>
        });
        assert_eq!(
            err,
            "inherit_from: \"border-color\" is not a valid Rust identifier"
        );
    }

    #[test]
    fn category_after_nested_errors() {
        let err = field_error(quote::quote! {
            #[theme(nested, category = "soft_option")]
            pub font: Option<FontSpec>
        });
        assert_eq!(
            err,
            "category conflicts with nested or resolved_type: a nested field has no category"
        );
    }

    #[test]
    fn nested_after_category_errors() {
        let err = field_error(quote::quote! {
            #[theme(category = "soft_option", nested)]
            pub font: Option<FontSpec>
        });
        assert_eq!(
            err,
            "category conflicts with nested or resolved_type: a nested field has no category"
        );
    }

    #[test]
    fn resolved_type_with_category_errors() {
        let err = field_error(quote::quote! {
            #[theme(category = "soft_option", resolved_type = "ResolvedFontSpec")]
            pub font: Option<FontSpec>
        });
        assert_eq!(
            err,
            "category conflicts with nested or resolved_type: a nested field has no category"
        );
    }

    #[test]
    fn second_range_attribute_errors() {
        let err = field_error(quote::quote! {
            #[theme(check = "positive", range = "0.0..=1.0")]
            pub opacity: Option<f32>
        });
        assert!(err.contains("only one"), "got: {err}");
    }

    #[test]
    fn range_min_above_max_errors() {
        let err = field_error(quote::quote! {
            #[theme(range = "1.0..=0.0")]
            pub opacity: Option<f32>
        });
        assert!(err.contains("exceeds"), "got: {err}");
    }

    #[test]
    fn range_nan_bound_errors() {
        let err = field_error(quote::quote! {
            #[theme(range = "NaN..=1.0")]
            pub opacity: Option<f32>
        });
        assert!(err.contains("finite"), "got: {err}");
    }

    #[test]
    fn range_infinite_bound_errors() {
        let err = field_error(quote::quote! {
            #[theme(range = "0.0..=inf")]
            pub opacity: Option<f32>
        });
        assert!(err.contains("finite"), "got: {err}");
    }

    #[test]
    fn range_bound_beyond_f32_errors() {
        let err = field_error(quote::quote! {
            #[theme(range = "0.0..=1e39")]
            pub opacity: Option<f32>
        });
        assert_eq!(err, "range bounds must be finite f32 values");
    }

    #[test]
    fn range_u16_min_above_max_errors() {
        let err = field_error(quote::quote! {
            #[theme(range_u16 = "900..=100")]
            pub weight: Option<u16>
        });
        assert!(err.contains("exceeds"), "got: {err}");
    }

    // === explicit_fields parsing ===

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

    #[test]
    fn resolved_name_empty_errors() {
        let result = parse_layer_from_tokens(quote::quote! {
            #[theme_layer(resolved_name = "")]
        });
        let err = result.expect_err("an empty resolved_name must error");
        assert!(err.to_string().contains("resolved_name"), "got: {err}");
    }

    #[test]
    fn resolved_name_not_an_identifier_errors() {
        let result = parse_layer_from_tokens(quote::quote! {
            #[theme_layer(resolved_name = "Resolved Button")]
        });
        let err = result.expect_err("a non-identifier resolved_name must error");
        assert!(err.to_string().contains("resolved_name"), "got: {err}");
    }

    // === theme_inherit parsing ===

    /// Helper: parse `#[theme_inherit(...)]` attributes from a token stream.
    fn parse_inherit_from_tokens(tokens: proc_macro2::TokenStream) -> Result<InheritMeta> {
        let wrapper = quote::quote! {
            #tokens
            struct Wrapper { x: i32 }
        };
        let item: syn::ItemStruct = syn::parse2(wrapper)?;
        parse_inherit_attrs(&item.attrs)
    }

    #[test]
    fn theme_inherit_absent_means_default() {
        let tokens = quote::quote! {};
        let inherit = parse_inherit_from_tokens(tokens).expect("should parse");
        assert!(inherit.border_kind.is_none());
        assert!(inherit.font.is_empty());
    }

    #[test]
    fn theme_inherit_border_full_parsed() {
        let tokens = quote::quote! {
            #[theme_inherit(border_kind = "full")]
        };
        let inherit = parse_inherit_from_tokens(tokens).expect("should parse");
        assert_eq!(inherit.border_kind, Some(BorderInheritanceKind::Full));
        assert!(inherit.font.is_empty());
    }

    #[test]
    fn theme_inherit_border_full_lg_parsed() {
        let tokens = quote::quote! {
            #[theme_inherit(border_kind = "full_lg")]
        };
        let inherit = parse_inherit_from_tokens(tokens).expect("should parse");
        assert_eq!(inherit.border_kind, Some(BorderInheritanceKind::FullLg));
    }

    #[test]
    fn theme_inherit_border_partial_parsed() {
        let tokens = quote::quote! {
            #[theme_inherit(border_kind = "partial")]
        };
        let inherit = parse_inherit_from_tokens(tokens).expect("should parse");
        assert_eq!(inherit.border_kind, Some(BorderInheritanceKind::Partial));
    }

    #[test]
    fn theme_inherit_font_single_parsed() {
        let tokens = quote::quote! {
            #[theme_inherit(font = "font")]
        };
        let inherit = parse_inherit_from_tokens(tokens).expect("should parse");
        let names: Vec<String> = inherit.font.iter().map(|i| i.to_string()).collect();
        assert_eq!(names, vec!["font".to_string()]);
    }

    #[test]
    fn theme_inherit_font_multiple_attributes_additive() {
        // List declares TWO font attributes: item_font + header_font.
        let tokens = quote::quote! {
            #[theme_inherit(border_kind = "full", font = "item_font")]
            #[theme_inherit(font = "header_font")]
        };
        let inherit = parse_inherit_from_tokens(tokens).expect("should parse");
        assert_eq!(inherit.border_kind, Some(BorderInheritanceKind::Full));
        let names: Vec<String> = inherit.font.iter().map(|i| i.to_string()).collect();
        assert_eq!(
            names,
            vec!["item_font".to_string(), "header_font".to_string()]
        );
    }

    #[test]
    fn theme_inherit_border_and_font_combined() {
        let tokens = quote::quote! {
            #[theme_inherit(border_kind = "full", font = "font")]
        };
        let inherit = parse_inherit_from_tokens(tokens).expect("should parse");
        assert_eq!(inherit.border_kind, Some(BorderInheritanceKind::Full));
        let names: Vec<String> = inherit.font.iter().map(|i| i.to_string()).collect();
        assert_eq!(names, vec!["font".to_string()]);
    }

    #[test]
    fn theme_inherit_unknown_key_errors() {
        let tokens = quote::quote! {
            #[theme_inherit(bogus = "x")]
        };
        let result = parse_inherit_from_tokens(tokens);
        assert!(result.is_err(), "unknown key must error");
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("unknown theme_inherit"),
            "error should mention unknown theme_inherit, got: {err}"
        );
    }

    #[test]
    fn theme_inherit_unknown_border_kind_errors() {
        let tokens = quote::quote! {
            #[theme_inherit(border_kind = "weird")]
        };
        let result = parse_inherit_from_tokens(tokens);
        assert!(result.is_err(), "unknown border_kind must error");
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("\"full\"") && err.contains("\"full_lg\"") && err.contains("\"partial\""),
            "error should list allowed values, got: {err}"
        );
    }

    #[test]
    fn theme_inherit_dialog_two_font_fields() {
        // Dialog declares title_font + body_font.
        let tokens = quote::quote! {
            #[theme_inherit(border_kind = "full_lg", font = "title_font")]
            #[theme_inherit(font = "body_font")]
        };
        let inherit = parse_inherit_from_tokens(tokens).expect("should parse");
        assert_eq!(inherit.border_kind, Some(BorderInheritanceKind::FullLg));
        let names: Vec<String> = inherit.font.iter().map(|i| i.to_string()).collect();
        assert_eq!(
            names,
            vec!["title_font".to_string(), "body_font".to_string()]
        );
    }
}
