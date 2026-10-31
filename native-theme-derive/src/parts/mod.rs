// `ThemeParts`: the 0.8.0 model's derive (S6.1).

use proc_macro2::TokenStream;
use syn::{DeriveInput, Error, Result};

mod attrs;
mod block;
mod read;
mod resolve;
mod walk;
mod widget;

pub(crate) fn derive(input: &DeriveInput) -> Result<TokenStream> {
    let type_attrs = attrs::parse_type_attrs(&input.attrs)?;
    match (type_attrs.block, type_attrs.widget, type_attrs.toml) {
        (true, false, false) => block::expand(input, &type_attrs),
        (false, true, false)
            if !type_attrs.append
                && !type_attrs.own_resolved
                && !type_attrs.resolved_patch
                && !type_attrs.own_toml
                && !type_attrs.check =>
        {
            widget::expand(input)
        }
        (false, false, true)
            if !type_attrs.append
                && !type_attrs.own_resolved
                && !type_attrs.resolved_patch
                && !type_attrs.own_toml
                && !type_attrs.check =>
        {
            read::expand_toml(input)
        }
        (false, false, true) => Err(Error::new_spanned(
            &input.ident,
            "append, own_resolved, resolved_patch, own_toml and check are a block's",
        )),
        (false, true, false) => Err(Error::new_spanned(
            &input.ident,
            "append, own_resolved, resolved_patch, own_toml and check are a block's",
        )),
        _ => Err(Error::new_spanned(
            &input.ident,
            "ThemeParts expands blocks, marked `#[parts(block)]`, widgets, marked \
             `#[parts(widget)]`, or enums read from TOML, marked `#[parts(toml)]`",
        )),
    }
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
            "pub stroke : :: core :: option :: Option < crate :: next :: blocks :: BlockPatch < Stroke > >",
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
             ty : \"Value<Stroke>\" , block : \"Stroke\" , form : < Stroke as crate :: next :: blocks :: Table > :: FORM , \
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

    /// A font size outside a text style (a widget value's) is converted, not
    /// text-scaled (S1, S9.4); a text style's font and line height are its
    /// frame's (`Cx::text_style`).
    #[test]
    fn font_sizes_scale_only_in_text_styles() {
        let value = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                pub min_font_size: Value<FontSize>,
            }
        });
        assert!(
            value.contains("cx . unscaled (| cx | cx . font (& items , false))"),
            "{value}"
        );
        let style = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct TextStyle {
                #[parts(block = "fields")]
                pub font: Font,
                pub letter_spacing: Value<FontSize>,
                #[parts(auto)]
                pub line_height: Value<FontSize>,
            }
        });
        for expected in [
            "cx . framed_font ()",
            "cx . framed_line_height ()",
            "cx . font (& items , false)",
        ] {
            assert!(style.contains(expected), "{expected}\n{style}");
        }
        assert!(!style.contains("cx . unscaled"), "{style}");
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
    fn a_map_of_blocks_resolves_them_and_patches_each_entry() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                #[parts(block = "append")]
                pub status_colors: Vec<(Key, Value<Paint>)>,
                #[parts(block = "fields")]
                pub keyed: Vec<(Key, TextStyle)>,
                #[parts(block = "set")]
                pub timings: Vec<(Key, Timing)>,
            }
        });
        for expected in [
            "pub status_colors : :: core :: option :: Option < :: std :: vec :: Vec < (Key , PaintPatch) > >",
            "pub keyed : :: core :: option :: Option < :: std :: vec :: Vec < (Key , crate :: next :: blocks :: BlockPatch < TextStyle >) > >",
            "pub timings : :: core :: option :: Option < Vec < (Key , Timing) > >",
            "pub status_colors : :: std :: vec :: Vec < (Key , :: core :: option :: Option < ResolvedPaint >) >",
            "crate :: next :: blocks :: merge_slot_entries (& mut self . status_colors , status_colors) \
             . map_err (| c | c . under (\"status_colors\")) ?",
            "crate :: next :: blocks :: apply_entries (& mut self . keyed , entries)",
            "crate :: next :: blocks :: merge_slot_by_key (& mut self . timings , timings)",
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
             block : \"Sides\" , form : crate :: next :: registry :: BlockForm :: Struct , fields : & [crate :: next :: registry :: FieldInfo { \
             path : \"top\" , ty : crate :: next :: registry :: FieldType :: Leaf (\"Value<Px>\") , \
             may_be_absent : true , auto : false , \
             value : < Value < Px > as crate :: next :: validate :: walk :: Walk > :: ty , }",
            "path : \"left\" , ty : crate :: next :: registry :: FieldType :: Leaf (\"Value<Px>\") , \
             may_be_absent : true , auto : false , \
             value : < Value < Px > as crate :: next :: validate :: walk :: Walk > :: ty , }] , } , \
             may_be_absent : false , auto : false , value : crate :: next :: registry :: no_type , }",
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
    fn an_optional_value_resolves_to_an_option_of_its_type() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                pub extent: Option<Value<Px>>,
            }
        });
        assert!(
            out.contains("pub extent : :: core :: option :: Option < Px >"),
            "{out}"
        );
        for mark in [quote::quote!(may_be_absent), quote::quote!(auto)] {
            let input = syn::parse_quote! {
                #[parts(block)]
                pub struct Probe {
                    #[parts(#mark)]
                    pub extent: Option<Value<Px>>,
                }
            };
            assert_eq!(
                refused(input),
                "an `Option<Value<T>>` resolves to `Option<T>`, its `None` a meaning of its own: \
                 its `Value` is never `Absent` or `Auto`"
            );
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
            "pub surfaces : :: core :: option :: Option < crate :: next :: SurfaceMap < crate :: next :: blocks :: BlockPatch < SurfaceRoles >> >",
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
            "const _ : () = :: core :: assert ! (crate :: next :: registry :: names_values \
             (< IconStyle as crate :: next :: blocks :: Table > :: FIELDS , & [\"size\"]) , \
             \"required names a field of `IconStyle` that is not a `Value`\") ;",
            "may_be_absent : true , auto : false",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
    }

    #[test]
    fn a_patch_field_resolves_and_points_to_its_blocks_table() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Keyframe {
                #[parts(patch)]
                pub patch: PartStylePatch,
            }
        });
        for expected in [
            "pub patch : ResolvedPartStylePatch",
            "ty : crate :: next :: registry :: FieldType :: Patch { ty : \"PartStylePatch\" , \
             block : \"PartStyle\" , fields : { fn fields () -> & 'static [crate :: next :: registry :: FieldInfo] { \
             < PartStyle as crate :: next :: blocks :: Table > :: FIELDS } fields } , }",
            "crate :: next :: blocks :: replace_slot (& mut self . patch , patch)",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
    }

    #[test]
    fn resolved_patch_resolves_each_field_as_an_option() {
        let out = expand(syn::parse_quote! {
            #[parts(block, resolved_patch)]
            pub struct Probe {
                /// The fill.
                #[parts(block = "append", may_be_absent)]
                pub fill: Value<Paint>,
                #[parts(auto)]
                pub min_width: Value<Px>,
            }
        });
        for expected in [
            "pub struct ResolvedProbePatch",
            "pub struct ResolvedProbePatch { # [doc = r\" The fill.\"] pub fill : :: core :: option :: Option < :: core :: option :: Option < ResolvedPaint > >",
            "pub min_width : :: core :: option :: Option < :: core :: option :: Option < crate :: next :: Length > >",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
    }

    #[test]
    fn create_leaves_absent_only_what_may_be_and_requires_the_rest() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                pub width: Value<Px>,
                #[parts(may_be_absent)]
                pub inset: Value<Px>,
                #[parts(may_be_absent)]
                pub margin: Sides<Value<Px>>,
                pub flip: Axes<bool>,
                pub case: TextCase,
                #[parts(block = "fields")]
                pub edge: Edge,
                #[parts(block = "fields", inner_may_be_absent, required = "fill")]
                pub status: StatusColors,
                #[parts(block = "fields")]
                pub stroke: Value<Stroke>,
            }
        });
        for expected in [
            "impl crate :: next :: blocks :: Create for Probe",
            "fn create_in (patch : ProbePatch , inner : :: core :: option :: Option < & [& str] > ,)",
            "width : crate :: next :: blocks :: created_value (width , false , inner , \"width\") ?",
            "inset : crate :: next :: blocks :: created_value (inset , true , inner , \"inset\") ?",
            "top : crate :: next :: blocks :: created_value (held . top , true , inner , \"margin.top\" ,) ?",
            "vertical : held . vertical . ok_or_else (|| { crate :: next :: blocks :: MergeConflict :: new \
             (crate :: next :: blocks :: Below :: Missing) . under (\"flip.vertical\") }) ?",
            "case : case . ok_or_else (|| crate :: next :: blocks :: MergeConflict :: new \
             (crate :: next :: blocks :: Below :: Missing) . under (\"case\")) ?",
            "edge : < Edge as crate :: next :: blocks :: Create > :: create_in (edge . unwrap_or_default () , \
             :: core :: option :: Option :: None)",
            "status : < StatusColors as crate :: next :: blocks :: Create > :: create_in (status . unwrap_or_default () , \
             :: core :: option :: Option :: Some (& [\"fill\"]))",
            "< crate :: next :: Value < Stroke > as crate :: next :: blocks :: Entry < crate :: next :: blocks :: \
             BlockPatch < Stroke > >> :: create (patch)",
            ":: core :: option :: Option :: None => crate :: next :: blocks :: created_value (:: core :: option :: \
             Option :: None , false , inner , \"stroke\") ?",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
    }

    #[test]
    fn widget_expands_part_enum_patch_and_tables() {
        let out = expand(syn::parse_quote! {
            #[parts(widget)]
            pub(crate) struct MenuButtonTheme {
                /// The bezel.
                #[part(root, fields(fill, shadow), absent(shadow))]
                body: PartStyle,
                /// The arrow.
                #[part(element, may_be_absent, parent = "body", fields(glyph))]
                menu_indicator: PartStyle,
                values: MenuButtonValues,
            }
        });
        for expected in [
            "pub enum MenuButtonPart { # [doc = r\" The bezel.\"] Body , # [doc = r\" The arrow.\"] MenuIndicator , }",
            "pub struct MenuButtonPatch { # [doc = r\" The bezel.\"] pub body : :: core :: option :: Option < crate :: next :: widgets :: PartPatch > ,",
            "pub values : :: core :: option :: Option < MenuButtonValuesPatch > ,",
            "pub (crate) fn into_parts (self ,)",
            "[(MenuButtonPart :: Body , body) , (MenuButtonPart :: MenuIndicator , menu_indicator)]",
            "impl crate :: next :: widgets :: WidgetTable for MenuButtonTheme",
            "name : \"body\" , root : true , element : false , parent : :: core :: option :: Option :: None , may_be_absent : false",
            "name : \"menu_indicator\" , root : false , element : true , parent : :: core :: option :: Option :: Some (\"body\") , may_be_absent : true",
            "const _ : () = :: core :: assert ! (crate :: next :: registry :: names_fields \
             (< PartStyle as crate :: next :: blocks :: Table > :: FIELDS , & [\"fill\" , \"shadow\"]) , \
             \"`menu_button.body` lists a field its style does not have\") ;",
            "const FIELDS : [crate :: next :: registry :: FieldInfo ; 2usize] = crate :: next :: registry :: select \
             (< PartStyle as crate :: next :: blocks :: Table > :: FIELDS , & [\"fill\" , \"shadow\"] , & [\"shadow\"] ,) ;",
            "const VALUES : & 'static [crate :: next :: registry :: FieldInfo] = < MenuButtonValues as crate :: next :: blocks :: Table > :: FIELDS ;",
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
            "impl crate :: next :: blocks :: Entry < PaintPatch > for crate :: next :: Value < Paint >",
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
    fn struct_block_expands_its_readers() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                pub width: Value<Px>,
                pub limit: Option<Value<Ms>>,
                #[parts(may_be_absent)]
                pub margin: Sides<Value<Px>>,
                #[parts(block = "fields")]
                pub stroke: Value<Stroke>,
                #[parts(block = "fields")]
                pub surfaces: SurfaceMap<SurfaceRoles>,
                pub params: Vec<(Key, Value<Scalar>)>,
                pub groups: Vec<(Key, Vec<(Key, Value<Scalar>)>)>,
            }
        });
        for expected in [
            "impl crate :: next :: toml :: read :: Fields for ProbePatch",
            "crate :: next :: toml :: read :: spelled (\"width\" , key , < Value < Px > as \
             crate :: next :: toml :: read :: FromToml > :: SPELLINGS)",
            "crate :: next :: toml :: read :: put (& mut self . width , read_value , < Value < Px > \
             as crate :: next :: toml :: read :: FromToml > :: combine , table , key , \"width\" , cx)",
            "crate :: next :: toml :: read :: read_map :: < Key , Value < Scalar > > (item , & at , cx)",
            "crate :: next :: toml :: read :: read_nested_map :: < Key , Key , Value < Scalar > > \
             (item , & at , cx)",
            "crate :: next :: toml :: read :: read_surfaces :: < crate :: next :: blocks :: \
             BlockPatch < SurfaceRoles >> (item , & at , cx)",
            "impl crate :: next :: toml :: read :: FromToml for ProbePatch",
            "impl crate :: next :: toml :: read :: FromToml for Probe",
            "impl crate :: next :: toml :: read :: Build for Probe",
            "let field_limit = crate :: next :: toml :: read :: optional (field_limit , options_none",
            "let field_width = crate :: next :: toml :: read :: required (field_width",
            "crate :: next :: toml :: read :: build_value :: < Stroke > (patch",
            "impl :: core :: convert :: TryFrom < ProbePatch > for Probe",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
    }

    #[test]
    fn a_block_and_its_patch_walk_their_fields_with_their_marks() {
        let out = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                #[parts(signed, may_be_absent)]
                pub margin: Sides<Value<Px>>,
                pub params: Vec<(Key, Value<Scalar>)>,
                #[parts(block = "fields", inner_may_be_absent, required = "fill")]
                pub status: StatusColors,
            }
        });
        for expected in [
            "impl crate :: next :: validate :: walk :: Walk for Probe",
            "impl crate :: next :: validate :: walk :: Walk for ProbePatch",
            "w . field (\"Probe\" , \"margin\" , crate :: next :: validate :: walk :: Marks { \
             may_be_absent : true , auto : false , signed : true }",
            "crate :: next :: validate :: walk :: walk_map (& self . params , w) ;",
            "if let :: core :: option :: Option :: Some (map) = & self . params { \
             crate :: next :: validate :: walk :: walk_map (map , w) ; }",
            "w . place (& [\"fill\"] ,",
            "w . patch (& [\"margin\" , \"params\" , \"status\"] ,",
            "if self . margin . is_some () { set . push (\"margin\") ; }",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
    }

    #[test]
    fn an_enum_walk_steps_through_its_variant_and_a_checked_block_is_handed_over() {
        let out = expand(syn::parse_quote! {
            #[parts(block, check)]
            pub enum Probe {
                #[parts(suffixed)]
                Px(Value<Px>),
                Full { #[parts(may_be_absent)] maximum: Value<Px> },
                Pair(f32, f32),
                Round,
                Keyed(Key),
            }
        });
        for expected in [
            "w . check (crate :: next :: validate :: walk :: Node :: Probe (self)) ;",
            "Probe :: Keyed (member0) => { w . variant (\"Probe\" , \"keyed\" , true ,",
            "Probe :: Px (member0) => { w . variant (\"Probe\" , \"px\" , true ,",
            "Probe :: Full { maximum } => { w . variant (\"Probe\" , \"full\" , false , | w | { \
             w . field (\"Probe\" , \"maximum\" ,",
            "crate :: next :: validate :: walk :: Walk :: suffix (maximum)",
            "w . variant (\"Probe\" , \"pair\" , false , | w | { w . index (0usize ,",
            "fn suffix (& self) -> & 'static str { match self { Probe :: Px (member0) => \
             crate :: next :: validate :: walk :: Walk :: suffix (member0) , Probe :: Full { .. } \
             => \"\" , Probe :: Pair (..) => \"\" , Probe :: Round => \"\" , Probe :: Keyed \
             (member0) => crate :: next :: validate :: walk :: Walk :: suffix (member0) ,",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
        let block = expand(syn::parse_quote! {
            #[parts(block, check)]
            pub struct Probe { pub width: Value<Px> }
        });
        for expected in [
            "w . check (crate :: next :: validate :: walk :: Node :: Probe (self)) ;",
            "w . check (crate :: next :: validate :: walk :: Node :: ProbePatch (self)) ;",
        ] {
            assert!(block.contains(expected), "{expected}\n{block}");
        }
        let unchecked = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe { pub width: Value<Px> }
        });
        assert!(!unchecked.contains("w . check"), "{unchecked}");
        assert!(
            refused(syn::parse_quote! {
                #[parts(block)]
                pub enum Probe { #[parts(suffixed)] Pair(f32, f32) }
            })
            .contains("suffixed marks a tuple variant of one member")
        );
    }

    #[test]
    fn walks_give_the_types_references_have() {
        let block = expand(syn::parse_quote! {
            #[parts(block)]
            pub struct Probe {
                pub durations: Vec<(Key, Value<Ms>)>,
                pub width: Value<Px>,
            }
        });
        for expected in [
            "fn ty () -> :: core :: option :: Option < crate :: next :: expr :: Ty > { :: core :: \
             option :: Option :: Some (crate :: next :: expr :: Ty :: block (\"Probe\")) }",
            "path : \"durations\" , ty : crate :: next :: registry :: FieldType :: Leaf \
             (\"Vec<(Key,Value<Ms>)>\") , may_be_absent : false , auto : false , value : < Value < Ms \
             > as crate :: next :: validate :: walk :: Walk > :: ty , }",
            "value : < Value < Px > as crate :: next :: validate :: walk :: Walk > :: ty",
        ] {
            assert!(block.contains(expected), "{expected}\n{block}");
        }
        let named = expand(syn::parse_quote! {
            #[parts(toml)]
            pub enum Side { Top, BottomEnd, Keyed(Key) }
        });
        assert!(
            named.contains(
                "crate :: next :: expr :: Ty :: Enum (& crate :: next :: expr :: EnumTy { name : \
                 \"Side\" , variants : & [\"top\" , \"bottom_end\"] , })"
            ),
            "{named}"
        );
    }

    #[test]
    fn own_toml_leaves_the_readers_of_the_whole_block_to_the_hand() {
        let out = expand(syn::parse_quote! {
            #[parts(block, own_toml)]
            pub struct Probe {
                pub width: Value<Px>,
            }
        });
        assert!(
            out.contains("impl crate :: next :: toml :: read :: Fields for ProbePatch"),
            "{out}"
        );
        assert!(
            out.contains("impl crate :: next :: toml :: read :: Build for Probe"),
            "{out}"
        );
        assert!(!out.contains("FromToml for ProbePatch"), "{out}");
        assert!(!out.contains("FromToml for Probe "), "{out}");
    }

    #[test]
    fn an_enum_reads_its_variants() {
        let out = expand(syn::parse_quote! {
            #[parts(toml)]
            pub enum Probe {
                Ring,
                FollowPart,
                Dashed { on: Px, limit: Option<Value<Ms>> },
                Center(f32, f32),
                Layers(Vec<Layer>),
                Keyed(Key),
            }
        });
        for expected in [
            "impl crate :: next :: toml :: read :: FromToml for Probe",
            "\"ring\" => :: core :: option :: Option :: Some (Probe :: Ring)",
            "\"follow_part\" => :: core :: option :: Option :: Some (Probe :: FollowPart)",
            "if let :: core :: option :: Option :: Some (key) = name . strip_prefix ('@')",
            "\"dashed\" =>",
            "crate :: next :: toml :: read :: optional (field_limit , true",
            "crate :: next :: toml :: read :: members :: < 2usize > (payload , \"an array of 2 values\"",
            "\"layers\" => < Vec < Layer > as crate :: next :: toml :: read :: FromToml > :: read \
             (payload , crate :: next :: toml :: read :: Spelling :: Item , & at , cx) . map \
             (Probe :: Layers)",
        ] {
            assert!(out.contains(expected), "{expected}\n{out}");
        }
        assert!(!out.contains("Named for Probe"), "{out}");
        let names = expand(syn::parse_quote! {
            #[parts(toml)]
            pub enum Side { Top, BottomEnd }
        });
        assert!(
            names.contains("impl crate :: next :: toml :: read :: Named for Side"),
            "{names}"
        );
        assert!(
            names.contains("Side :: BottomEnd => \"bottom_end\""),
            "{names}"
        );
    }

    #[test]
    fn refusals_name_the_rule() {
        for (input, message) in [
            (
                syn::parse_quote! { pub struct ButtonTheme { body: PartStyle } },
                "ThemeParts expands blocks, marked `#[parts(block)]`, widgets, marked \
                 `#[parts(widget)]`, or enums read from TOML, marked `#[parts(toml)]`",
            ),
            (
                syn::parse_quote! {
                    #[parts(toml)]
                    pub struct Probe { pub x: bool }
                },
                "`#[parts(toml)]` marks an enum",
            ),
            (
                syn::parse_quote! {
                    #[parts(toml, own_toml)]
                    pub enum Probe { A }
                },
                "append, own_resolved, resolved_patch, own_toml and check are a block's",
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
                "an append block is held as `Value<B>`, or as a map's `Value<B>`",
            ),
            (
                syn::parse_quote! {
                    #[parts(block)]
                    pub struct Probe { #[parts(block = "append")] pub fills: Vec<Paint> }
                },
                "an append block is held as `Value<B>`, or as a map's `Value<B>`",
            ),
            (
                syn::parse_quote! {
                    #[parts(block)]
                    pub struct Probe { #[parts(block = "append")] pub fill: Option<Paint> }
                },
                "an append block is held as `Value<B>`, or as a map's `Value<B>`",
            ),
            (
                syn::parse_quote! {
                    #[parts(block)]
                    pub struct Probe { #[parts(block = "append")] pub fills: Vec<(Key, Paint)> }
                },
                "an append block is held as `Value<B>`, or as a map's `Value<B>`",
            ),
            (
                syn::parse_quote! {
                    #[parts(block)]
                    pub struct Probe {
                        #[parts(block = "append")]
                        pub fills: Vec<(Key, Vec<Paint>)>,
                    }
                },
                "an append block is held as `Value<B>`, or as a map's `Value<B>`",
            ),
            (
                syn::parse_quote! {
                    #[parts(block)]
                    pub struct Probe { #[parts(block = "fields")] pub repeat: Option<KeyRepeat> }
                },
                "an `Option<B>` is set whole: mark it `block = \"set\"`",
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
                    #[parts(block)]
                    pub struct Probe { #[parts(patch)] pub patch: PartStyle }
                },
                "patch marks a field holding a struct block's patch, `<B>Patch`",
            ),
            (
                syn::parse_quote! {
                    #[parts(block, resolved_patch)]
                    pub enum Probe { A, B }
                },
                "resolved_patch is a struct block's, patched by fields",
            ),
            (
                syn::parse_quote! {
                    #[parts(wide)]
                    pub struct Probe { pub x: Value<Px> }
                },
                "unknown parts attribute: expected block, widget, append, own_resolved, \
                 resolved_patch, toml, own_toml or check",
            ),
            (
                syn::parse_quote! {
                    #[parts(widget)]
                    pub struct ProbeTheme { pub x: Value<Px>, values: ProbeValues }
                },
                "a widget's field is a part, marked `#[part(...)]`, or its one `values`",
            ),
            (
                syn::parse_quote! {
                    #[parts(widget)]
                    pub struct ProbeTheme {
                        #[part(fields(fill))]
                        body: PartStyle,
                        values: ProbeValues,
                    }
                },
                "a widget's first part, and only it, is its root, `#[part(root, ...)]`",
            ),
            (
                syn::parse_quote! {
                    #[parts(widget)]
                    pub struct ProbeTheme {
                        #[part(root, may_be_absent, fields(fill))]
                        body: PartStyle,
                        values: ProbeValues,
                    }
                },
                "the root part is never absent as a whole, never an element and nests in nothing",
            ),
            (
                syn::parse_quote! {
                    #[parts(widget)]
                    pub struct ProbeTheme {
                        #[part(root, fields(fill))]
                        body: PartStyle,
                        #[part(parent = "label", fields(text))]
                        icon: PartStyle,
                        values: ProbeValues,
                    }
                },
                "parent names another part",
            ),
            (
                syn::parse_quote! {
                    #[parts(widget)]
                    pub struct ProbeTheme {
                        #[part(root, fields(fill), absent(shadow))]
                        body: PartStyle,
                        values: ProbeValues,
                    }
                },
                "absent names a field the part lists in `fields(...)`",
            ),
            (
                syn::parse_quote! {
                    #[parts(widget)]
                    pub struct ProbeTheme {
                        #[part(root, fields(fill))]
                        body: PartStyle,
                    }
                },
                "a widget has a `values` field, its values' struct",
            ),
            (
                syn::parse_quote! {
                    #[parts(widget)]
                    pub struct Probe {
                        #[part(root, fields(fill))]
                        body: PartStyle,
                        values: ProbeValues,
                    }
                },
                "a widget's input is named `<Widget>Theme`",
            ),
            (
                syn::parse_quote! {
                    #[parts(widget)]
                    pub struct ProbeTheme {
                        #[part(root, fields(fill, stroke, fill))]
                        body: PartStyle,
                        values: ProbeValues,
                    }
                },
                "a field named twice",
            ),
            (
                syn::parse_quote! {
                    #[parts(widget)]
                    pub struct ProbeTheme {
                        #[part(root, fields(fill, stroke), absent(stroke, stroke))]
                        body: PartStyle,
                        values: ProbeValues,
                    }
                },
                "a field named twice",
            ),
            (
                syn::parse_quote! {
                    #[parts(widget)]
                    pub struct ProbeTheme {
                        #[part(root, fields(fill))]
                        body: PartStyle,
                        #[part(parent = "mark", fields(text))]
                        label: PartStyle,
                        #[part(parent = "label", fields(glyph))]
                        mark: PartStyle,
                        values: ProbeValues,
                    }
                },
                "parents form a cycle: a part nests in itself",
            ),
            (
                syn::parse_quote! {
                    #[parts(widget)]
                    pub struct ProbeTheme {
                        #[part(root, fields(fill))]
                        body: TextStyle,
                        values: ProbeValues,
                    }
                },
                "a part is styled by a part style, `PartStyle`",
            ),
            (
                syn::parse_quote! {
                    #[parts(widget)]
                    pub struct ProbeTheme {
                        #[part(root, fields(fill))]
                        body: PartStyle,
                        #[parts(may_be_absent)]
                        values: ProbeValues,
                    }
                },
                "a widget's field takes `#[part(...)]`, not `#[parts(...)]`",
            ),
            (
                syn::parse_quote! {
                    #[parts(widget)]
                    #[part(root)]
                    pub struct ProbeTheme {
                        #[part(root, fields(fill))]
                        body: PartStyle,
                        values: ProbeValues,
                    }
                },
                "`#[part(...)]` marks a widget's field, not the widget",
            ),
        ] {
            assert_eq!(refused(input), message);
        }
    }
}
