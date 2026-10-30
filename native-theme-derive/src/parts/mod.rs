// `ThemeParts`: the 0.8.0 model's derive (S6.1).

use proc_macro2::TokenStream;
use syn::{DeriveInput, Error, Result};

mod attrs;
mod block;

pub(crate) fn derive(input: &DeriveInput) -> Result<TokenStream> {
    let type_attrs = attrs::parse_type_attrs(&input.attrs)?;
    if !type_attrs.block {
        return Err(Error::new_spanned(
            &input.ident,
            "ThemeParts expands blocks, marked `#[parts(block)]`",
        ));
    }
    block::expand(input, &type_attrs)
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    fn expand(input: DeriveInput) -> String {
        derive(&input).expect("should derive").to_string()
    }

    fn refused(input: DeriveInput) -> String {
        derive(&input).expect_err("should refuse").to_string()
    }

    #[test]
    fn struct_block_expands_patch_resolved_merge_and_table() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                pub width: Value<Px>,
                #[parts(block = "fields")]
                pub top: Edge,
                #[parts(block = "fields", may_be_absent)]
                pub stroke: Value<Stroke>,
                #[parts(block = "append")]
                pub fill: Value<Paint>,
                #[parts(block = "set")]
                pub corner: Radius,
                pub params: Vec<(Key, Value<Scalar>)>,
            }
        });
        for expected in [
            "pub struct ProbePatch",
            "pub width : Option < Value < Px > >",
            "pub top : Option < EdgePatch >",
            "pub stroke : Option < crate :: next :: blocks :: BlockPatch < Stroke >>",
            "pub fill : Option < PaintPatch >",
            "pub corner : Option < Radius >",
            "pub struct ResolvedProbe",
            "pub width : Option < Px >",
            "pub top : ResolvedEdge",
            "pub stroke : Option < ResolvedStroke >",
            "pub fill : Option < ResolvedPaint >",
            "pub corner : ResolvedRadius",
            "pub params : Vec < (Key , Option < Scalar >) >",
            "impl crate :: next :: blocks :: FieldsBlock for Probe",
            "impl crate :: next :: blocks :: Merge for ProbePatch",
            "impl crate :: next :: blocks :: Apply < ProbePatch > for Probe",
            "crate :: next :: blocks :: replace_slot (& mut self . width , width)",
            "crate :: next :: blocks :: merge_slot (& mut self . top , top)",
            "crate :: next :: blocks :: merge_slot_by_key (& mut self . params , params)",
            "path : \"stroke\" , ty : crate :: next :: FieldType :: Block { ty : \"Value<Stroke>\" , \
             fields : < Stroke as crate :: next :: blocks :: Block > :: FIELDS } , \
             may_be_absent : true , auto : false",
            "path : \"params\" , ty : crate :: next :: FieldType :: Leaf (\"Vec<(Key,Value<Scalar>)>\")",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
    }

    #[test]
    fn auto_value_px_resolves_to_length() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                #[parts(auto)]
                pub max_width: Value<Px>,
            }
        });
        assert!(
            out.contains("pub max_width : Option < crate :: next :: Length >"),
            "{out}"
        );
        assert!(out.contains("may_be_absent : false , auto : true"), "{out}");
    }

    #[test]
    fn append_block_expands_set_and_append() {
        let out = expand(syn::parse_quote! {
            #[parts(block, append)]
            pub struct Paint {
                #[parts(block = "fields")]
                pub layers: Vec<PaintLayer>,
            }
        });
        for expected in [
            "pub enum PaintPatch",
            "Set (crate :: next :: Value < Paint >)",
            "Append (Vec < PaintLayer >)",
            "pub layers : Vec < ResolvedPaintLayer >",
            "impl crate :: next :: blocks :: Apply < PaintPatch > for crate :: next :: Value < Paint >",
            "Paint { layers : items }",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
        assert!(!out.contains("FieldsBlock"), "{out}");
    }

    #[test]
    fn enum_block_paths_name_the_variant_and_field() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub enum Radius {
                Px(Value<Px>),
                Full {
                    #[parts(may_be_absent)]
                    maximum: Value<Px>,
                },
                FollowPart,
            }
        });
        for expected in [
            "pub enum ResolvedRadius",
            "Px (Option < Px >)",
            "Full { maximum : Option < Px > , }",
            "FollowPart",
            "path : \"px\"",
            "path : \"full.maximum\"",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
        assert!(!out.contains("RadiusPatch"), "{out}");
        assert!(!out.contains("path : \"follow_part\""), "{out}");
    }

    #[test]
    fn own_resolved_leaves_the_resolved_type_out() {
        let out = expand(syn::parse_quote! {
            #[parts(block, own_resolved)]
            pub enum Shadow {
                Layers(#[parts(block = "fields")] Vec<ShadowLayer>),
                Elevation(Key),
            }
        });
        assert!(!out.contains("enum ResolvedShadow"), "{out}");
        assert!(out.contains("type Resolved = ResolvedShadow"), "{out}");
    }

    #[test]
    fn refusals_name_the_rule() {
        for (input, message) in [
            (
                syn::parse_quote! { pub struct ButtonTheme { body: PartStyle } },
                "ThemeParts expands blocks, marked `#[parts(block)]`",
            ),
            (
                syn::parse_quote! {
                    #[parts(block)]
                    pub struct Probe { #[parts(may_be_absent)] pub inset: bool }
                },
                "may_be_absent marks a field holding a `Value`",
            ),
            (
                syn::parse_quote! {
                    #[parts(block)]
                    pub struct Probe { #[parts(auto)] pub rotate: Value<Deg> }
                },
                "auto marks a field holding a `Value<Px>`",
            ),
            (
                syn::parse_quote! {
                    #[parts(block)]
                    pub struct Probe { #[parts(block = "append")] pub fill: Paint }
                },
                "an append block is held as `Value<B>` or `Vec<B>`",
            ),
            (
                syn::parse_quote! {
                    #[parts(block, append)]
                    pub struct Paint { pub layers: Vec<PaintLayer>, pub extra: bool }
                },
                "an append block has one field, a `Vec`",
            ),
            (
                syn::parse_quote! {
                    #[parts(block)]
                    pub enum Probe { Pair(Value<Px>, Value<Px>) }
                },
                "a block's tuple variant holds one field",
            ),
            (
                syn::parse_quote! {
                    #[parts(block)]
                    pub struct Probe { #[parts(block = "whole")] pub top: Edge }
                },
                "unknown block kind \"whole\": expected \"fields\", \"append\" or \"set\"",
            ),
            (
                syn::parse_quote! {
                    #[parts(block, block)]
                    pub struct Probe { pub x: Value<Px> }
                },
                "duplicate parts attribute",
            ),
            (
                syn::parse_quote! {
                    #[parts(widget)]
                    pub struct Probe { pub x: Value<Px> }
                },
                "unknown parts attribute: expected block, append or own_resolved",
            ),
        ] {
            assert_eq!(refused(input), message);
        }
    }
}
