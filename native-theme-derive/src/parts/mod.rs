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
            "pub width : :: core :: option :: Option < Value < Px > >",
            "pub top : :: core :: option :: Option < EdgePatch >",
            "pub stroke : :: core :: option :: Option < crate :: next :: blocks :: BlockPatch < Stroke >>",
            "pub fill : :: core :: option :: Option < PaintPatch >",
            "pub corner : :: core :: option :: Option < Radius >",
            "pub struct ResolvedProbe",
            "pub width : :: core :: option :: Option < Px >",
            "pub top : ResolvedEdge",
            "pub stroke : :: core :: option :: Option < ResolvedStroke >",
            "pub fill : :: core :: option :: Option < ResolvedPaint >",
            "pub corner : ResolvedRadius",
            "pub params : :: std :: vec :: Vec < (Key , :: core :: option :: Option < Scalar >) >",
            "impl crate :: next :: blocks :: FieldsBlock for Probe",
            "impl crate :: next :: blocks :: Merge for ProbePatch",
            "impl crate :: next :: blocks :: Apply < ProbePatch > for Probe",
            "crate :: next :: blocks :: replace_slot (& mut self . width , width)",
            "crate :: next :: blocks :: merge_slot (& mut self . top , top)",
            "crate :: next :: blocks :: merge_slot_by_key (& mut self . params , params)",
            "impl crate :: next :: blocks :: sealed :: Sealed for Probe",
            "path : \"stroke\" , ty : crate :: next :: registry :: FieldType :: Block { \
             ty : \"Value<Stroke>\" , form : < Stroke as crate :: next :: blocks :: Table > :: FORM , \
             fields : < Stroke as crate :: next :: blocks :: Table > :: FIELDS , } , \
             may_be_absent : true , auto : false",
            "path : \"params\" , ty : crate :: next :: registry :: FieldType :: Leaf (\"Vec<(Key,Value<Scalar>)>\")",
            "const FORM : crate :: next :: registry :: BlockForm = crate :: next :: registry :: BlockForm :: Struct",
            "if let :: core :: option :: Option :: Some (value) = width",
            ":: core :: result :: Result :: Ok (())",
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
            out.contains("pub max_width : :: core :: option :: Option < crate :: next :: Length >"),
            "{out}"
        );
        assert!(out.contains("may_be_absent : false , auto : true"), "{out}");
    }

    #[test]
    fn auto_value_font_size_resolves_to_length() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                #[parts(auto)]
                pub line_height: Value<FontSize>,
                pub letter_spacing: Value<FontSize>,
            }
        });
        assert!(
            out.contains(
                "pub line_height : :: core :: option :: Option < crate :: next :: Length >"
            ),
            "{out}"
        );
        assert!(
            out.contains(
                "pub letter_spacing : :: core :: option :: Option < crate :: next :: Px >"
            ),
            "{out}"
        );
    }

    #[test]
    fn nested_block_types_keep_their_path() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                #[parts(block = "fields")]
                pub top: outline::Edge,
                #[parts(block = "append")]
                pub fill: Value<paint::Paint>,
            }
        });
        for expected in [
            "pub top : :: core :: option :: Option < outline :: EdgePatch >",
            "pub fill : :: core :: option :: Option < paint :: PaintPatch >",
            "pub top : outline :: ResolvedEdge",
            "pub fill : :: core :: option :: Option < paint :: ResolvedPaint >",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
    }

    #[test]
    fn a_map_of_blocks_resolves_them_and_merges_by_key() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                #[parts(block = "append")]
                pub status_colors: Vec<(Key, Value<Paint>)>,
            }
        });
        for expected in [
            "pub status_colors : :: core :: option :: Option < Vec < (Key , Value < Paint >) > >",
            "pub status_colors : :: std :: vec :: Vec < (Key , :: core :: option :: Option < ResolvedPaint >) >",
            "crate :: next :: blocks :: merge_slot_by_key (& mut self . status_colors , status_colors)",
            "fields : < Paint as crate :: next :: blocks :: Table > :: FIELDS",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
    }

    #[test]
    fn compound_fields_patch_member_by_member() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                #[parts(may_be_absent)]
                pub margin: Sides<Value<Px>>,
                pub flip: Axes<bool>,
            }
        });
        for expected in [
            "pub margin : :: core :: option :: Option < crate :: next :: Sides < :: core :: option :: Option < Value < Px > >> >",
            "pub margin : crate :: next :: Sides < :: core :: option :: Option < Px > >",
            "pub flip : crate :: next :: Axes < bool >",
            "let held = self . margin . get_or_insert_with (|| crate :: next :: Sides { \
             top : :: core :: option :: Option :: None , right : :: core :: option :: Option :: None , \
             bottom : :: core :: option :: Option :: None , left : :: core :: option :: Option :: None })",
            "crate :: next :: blocks :: replace_slot (& mut held . left , later . left)",
            "if let :: core :: option :: Option :: Some (value) = later . vertical { self . flip . vertical = value ; }",
            "path : \"margin\" , ty : crate :: next :: registry :: FieldType :: Block { ty : \"Sides<Value<Px>>\" , \
             form : crate :: next :: registry :: BlockForm :: Struct , fields : & [crate :: next :: registry :: FieldInfo { \
             path : \"top\" , ty : crate :: next :: registry :: FieldType :: Leaf (\"Value<Px>\") , \
             may_be_absent : true , auto : false , }",
            "path : \"left\" , ty : crate :: next :: registry :: FieldType :: Leaf (\"Value<Px>\") , \
             may_be_absent : true , auto : false , }] , } , may_be_absent : false , auto : false }",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
    }

    #[test]
    fn keyed_lists_merge_by_their_items_keys() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                #[parts(block = "set", keyed)]
                pub bindings: Vec<Binding>,
            }
        });
        for expected in [
            "crate :: next :: blocks :: merge_slot_keyed (& mut self . bindings , bindings)",
            "crate :: next :: blocks :: KeyedItem :: merge_keyed (& mut self . bindings , items)",
            "pub bindings : :: std :: vec :: Vec < ResolvedBinding >",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
    }

    #[test]
    fn optional_blocks_and_plain_tuple_variants() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub enum Probe {
                CubicBezier(f32, f32, f32, f32),
                Fade {
                    #[parts(block = "set")]
                    start: Option<FadeLength>,
                },
            }
        });
        for expected in [
            "CubicBezier (f32 , f32 , f32 , f32)",
            "path : \"cubic_bezier\" , ty : crate :: next :: registry :: FieldType :: Leaf (\"(f32,f32,f32,f32)\")",
            "start : :: core :: option :: Option < ResolvedFadeLength >",
            "path : \"fade.start\" , ty : crate :: next :: registry :: FieldType :: Block { ty : \"Option<FadeLength>\"",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
    }

    #[test]
    fn a_surface_map_of_blocks_patches_set_by_set() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                #[parts(block = "fields")]
                pub surfaces: SurfaceMap<SurfaceRoles>,
            }
        });
        for expected in [
            "pub surfaces : :: core :: option :: Option < crate :: next :: SurfaceMap < SurfaceRolesPatch >>",
            "pub surfaces : crate :: next :: SurfaceMap < ResolvedSurfaceRoles >",
            "crate :: next :: blocks :: merge_slot_surfaces (& mut self . surfaces , surfaces) \
             . map_err (| c | c . under (\"surfaces\")) ?",
            "crate :: next :: blocks :: apply_surfaces (& mut self . surfaces , sets)",
            "fields : < SurfaceRoles as crate :: next :: blocks :: Table > :: FIELDS",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
    }

    #[test]
    fn a_map_of_maps_merges_by_key_twice() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                pub params: Vec<(Key, Vec<(Key, Value<Scalar>)>)>,
                pub per_language: Vec<(LanguageTag, Vec<String>)>,
            }
        });
        for expected in [
            "crate :: next :: blocks :: merge_slot_by_key_nested (& mut self . params , params)",
            "crate :: next :: blocks :: merge_by_key_nested (& mut self . params , entries)",
            "pub params : :: std :: vec :: Vec < (Key , :: std :: vec :: Vec < (Key , :: core :: option :: Option < Scalar >) >) >",
            "crate :: next :: blocks :: merge_slot_by_key (& mut self . per_language , per_language)",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
    }

    #[test]
    fn a_map_of_block_lists_resolves_them() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                #[parts(block = "fields")]
                pub elevation: Vec<(Key, Vec<ShadowLayer>)>,
            }
        });
        for expected in [
            "pub elevation : :: std :: vec :: Vec < (Key , :: std :: vec :: Vec < ResolvedShadowLayer >) >",
            "fields : < ShadowLayer as crate :: next :: blocks :: Table > :: FIELDS",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
    }

    #[test]
    fn inner_may_be_absent_marks_the_place() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                #[parts(block = "fields", may_be_absent, inner_may_be_absent, required = "size")]
                pub toolbar: Value<IconStyle>,
                #[parts(block = "fields", inner_may_be_absent)]
                pub success: StatusColors,
            }
        });
        for expected in [
            "const LEN : usize = < IconStyle as crate :: next :: blocks :: Table > :: FIELDS . len () ;",
            "const MARKED : [crate :: next :: registry :: FieldInfo ; LEN] = crate :: next :: registry :: \
             absent_but (< IconStyle as crate :: next :: blocks :: Table > :: FIELDS , & [\"size\"]) ;",
            "absent_but (< StatusColors as crate :: next :: blocks :: Table > :: FIELDS , & []) ;",
            "may_be_absent : true , auto : false",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
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
            "pub layers : :: std :: vec :: Vec < ResolvedPaintLayer >",
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
            "Px (:: core :: option :: Option < Px >)",
            "Full { maximum : :: core :: option :: Option < Px > , }",
            "FollowPart",
            "path : \"px\"",
            "path : \"full.maximum\"",
            "BlockForm :: Enum { unit_variants : & [\"follow_part\"] }",
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
                "auto marks a field holding a `Value<Px>` or a `Value<FontSize>`",
            ),
            (
                syn::parse_quote! {
                    #[parts(block)]
                    pub struct Probe { #[parts(block = "append")] pub fill: Paint }
                },
                "an append block is held as `Value<B>`, `Vec<B>` or `Option<B>`",
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
                "a block's tuple variant of several fields holds plain values, unmarked",
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
                    #[parts(block)]
                    pub struct Probe { #[parts(keyed)] pub kind: Value<Px> }
                },
                "keyed marks a `Vec`",
            ),
            (
                syn::parse_quote! {
                    #[parts(block)]
                    pub struct Probe { #[parts(block = "set")] pub offset: Sides<Value<Px>> }
                },
                "a compound value's members hold no block",
            ),
            (
                syn::parse_quote! {
                    #[parts(block)]
                    pub struct Probe { #[parts(auto)] pub gap: Sides<Value<Deg>> }
                },
                "auto marks a field holding a `Value<Px>` or a `Value<FontSize>`",
            ),
            (
                syn::parse_quote! {
                    #[parts(block)]
                    pub struct Probe { #[parts(block = "fields", required = "size")] pub x: IconStyle }
                },
                "required names the fields an `inner_may_be_absent` mark leaves out",
            ),
            (
                syn::parse_quote! {
                    #[parts(block)]
                    pub struct Probe { #[parts(block = "set", inner_may_be_absent)] pub x: Timing }
                },
                "inner_may_be_absent marks a field holding a struct block, `block = \"fields\"`",
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
