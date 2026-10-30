//! The blocks' patches, merges and tables (S2.3–S2.6, S6.1, S6.3). Every
//! number here is synthetic.

use native_theme_derive::ThemeParts;

use super::merge::Below;
use super::outline::{
    CornerCurve, Corners, DashPattern, Edge, EdgePatch, FillClip, PixelSnap, Radius, ResolvedEdge,
    ResolvedRadius, Stroke, StrokePatch, StrokePosition,
};
use super::paint::{
    BlendMode, Material, MaterialPatch, Paint, PaintKind, PaintLayer, PaintPatch,
    ResolvedPaintKind, ResolvedStop, Stop,
};
use super::shadow::{ResolvedShadow, Shadow, ShadowLayer};
use super::transform::Transform;
use super::{Apply, BlockPatch, Merge, MergeConflict, Table};
use crate::key;
use crate::next::color::Color;
use crate::next::expr::{Expr, Ref};
use crate::next::registry::{BlockForm, FieldInfo, FieldType};
use crate::next::units::{Deg, Length, Px, Scalar};
use crate::next::value::Value;

fn color_layer(r: f32) -> PaintLayer {
    PaintLayer {
        kind: PaintKind::Color(Value::Stated(Color {
            r,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        })),
        blend: BlendMode::Normal,
    }
}

fn paint(layers: Vec<PaintLayer>) -> Value<Paint> {
    Value::Stated(Paint { layers })
}

fn reference() -> Expr {
    Expr::Ref(Ref::Token(vec![key!("paint")]))
}

fn conflict(path: &str, below: Below) -> Result<(), MergeConflict> {
    Err(MergeConflict {
        path: path.to_owned(),
        below,
    })
}

fn edge(width: f32) -> Edge {
    Edge {
        width: Value::Stated(Px(width)),
        paint: paint(vec![color_layer(0.5)]),
        dash: Value::Stated(DashPattern::Solid),
        snap: PixelSnap::ToDevicePixel,
    }
}

fn stroke() -> Stroke {
    Stroke {
        top: edge(1.0),
        right: edge(1.0),
        bottom: edge(1.0),
        left: edge(1.0),
        position: StrokePosition::Inside,
        outset: Value::Absent,
        fill_clip: FillClip::Box,
    }
}

#[test]
fn paint_append_over_absent() {
    let mut patch = PaintPatch::Set(Value::Absent);
    assert_eq!(
        patch.merge(PaintPatch::Append(vec![color_layer(0.1)])),
        Ok(())
    );
    assert_eq!(patch, PaintPatch::Set(paint(vec![color_layer(0.1)])));

    let mut value: Value<Paint> = Value::Absent;
    assert_eq!(
        value.apply(PaintPatch::Append(vec![color_layer(0.1), color_layer(0.2)])),
        Ok(())
    );
    assert_eq!(value, paint(vec![color_layer(0.1), color_layer(0.2)]));
}

#[test]
fn append_adds_layers_over_the_paint_below() {
    let mut value = paint(vec![color_layer(0.1)]);
    assert_eq!(
        value.apply(PaintPatch::Append(vec![color_layer(0.2)])),
        Ok(())
    );
    assert_eq!(value, paint(vec![color_layer(0.1), color_layer(0.2)]));

    let mut patch = PaintPatch::Append(vec![color_layer(0.2)]);
    assert_eq!(
        patch.merge(PaintPatch::Append(vec![color_layer(0.3)])),
        Ok(())
    );
    assert_eq!(
        patch,
        PaintPatch::Append(vec![color_layer(0.2), color_layer(0.3)])
    );
    let mut value = paint(vec![color_layer(0.1)]);
    assert_eq!(value.apply(patch), Ok(()));
    assert_eq!(
        value,
        paint(vec![color_layer(0.1), color_layer(0.2), color_layer(0.3)])
    );
}

#[test]
fn set_replaces_a_paint_whole() {
    let mut patch = PaintPatch::Append(vec![color_layer(0.2)]);
    assert_eq!(patch.merge(PaintPatch::Set(Value::Absent)), Ok(()));
    assert_eq!(patch, PaintPatch::Set(Value::Absent));

    let mut value = paint(vec![color_layer(0.1), color_layer(0.2)]);
    assert_eq!(
        value.apply(PaintPatch::Set(paint(vec![color_layer(0.3)]))),
        Ok(())
    );
    assert_eq!(value, paint(vec![color_layer(0.3)]));
}

#[test]
fn append_over_reference_or_auto_is_not_merged() {
    let mut value: Value<Paint> = Value::Expr(reference());
    assert_eq!(
        value.apply(PaintPatch::Append(vec![color_layer(0.1)])),
        conflict("", Below::Reference)
    );
    let mut patch = PaintPatch::Set(Value::Auto);
    assert_eq!(
        patch.merge(PaintPatch::Append(vec![color_layer(0.1)])),
        conflict("", Below::Auto)
    );
}

#[test]
fn merge_map_by_key_list_whole() {
    let scalar = |n| Value::Stated(Scalar::Number(n));
    let mut earlier = MaterialPatch {
        params: Some(vec![(key!("a"), scalar(1.0)), (key!("b"), scalar(2.0))]),
        ..MaterialPatch::default()
    };
    let later = MaterialPatch {
        params: Some(vec![(key!("b"), scalar(3.0)), (key!("c"), scalar(4.0))]),
        ..MaterialPatch::default()
    };
    assert_eq!(earlier.merge(later.clone()), Ok(()));
    let merged = vec![
        (key!("a"), scalar(1.0)),
        (key!("b"), scalar(3.0)),
        (key!("c"), scalar(4.0)),
    ];
    assert_eq!(earlier.params, Some(merged.clone()));

    let mut material = Material {
        identity: key!("sidebar"),
        blur_radius: Value::Absent,
        tint: Value::Absent,
        saturation: Value::Absent,
        luminosity: Value::Absent,
        noise_opacity: Value::Absent,
        fallback: Value::Stated(Color {
            r: 0.5,
            g: 0.5,
            b: 0.5,
            a: 1.0,
        }),
        params: vec![(key!("a"), scalar(1.0)), (key!("b"), scalar(2.0))],
    };
    assert_eq!(material.apply(later), Ok(()));
    assert_eq!(material.params, merged);

    let stop = |position| Stop {
        position,
        color: Value::Absent,
    };
    let mut earlier = ProbePatch {
        stops: Some(vec![stop(0.0), stop(1.0)]),
        ..ProbePatch::default()
    };
    assert_eq!(
        earlier.merge(ProbePatch {
            stops: Some(vec![stop(0.5)]),
            ..ProbePatch::default()
        }),
        Ok(())
    );
    assert_eq!(earlier.stops, Some(vec![stop(0.5)]));
    assert_eq!(
        earlier.merge(ProbePatch {
            stops: Some(Vec::new()),
            ..ProbePatch::default()
        }),
        Ok(())
    );
    assert_eq!(earlier.stops, Some(Vec::new()));
    let mut probe = Probe {
        stops: vec![stop(0.0), stop(1.0)],
        width: Value::Auto,
    };
    assert_eq!(probe.apply(earlier), Ok(()));
    assert!(probe.stops.is_empty());
}

#[test]
fn a_later_field_replaces_the_earlier_and_unset_fields_stay() {
    let mut earlier = StrokePatch {
        position: Some(StrokePosition::Inside),
        outset: Some(Value::Stated(Px(1.0))),
        ..StrokePatch::default()
    };
    let later = StrokePatch {
        position: Some(StrokePosition::Outside),
        ..StrokePatch::default()
    };
    assert_eq!(earlier.merge(later), Ok(()));
    assert_eq!(earlier.position, Some(StrokePosition::Outside));
    assert_eq!(earlier.outset, Some(Value::Stated(Px(1.0))));
    assert_eq!(earlier.fill_clip, None);
}

#[test]
fn compound_fields_merge_field_by_field() {
    let mut earlier = StrokePatch {
        top: Some(EdgePatch {
            width: Some(Value::Stated(Px(1.0))),
            ..EdgePatch::default()
        }),
        ..StrokePatch::default()
    };
    let later = StrokePatch {
        top: Some(EdgePatch {
            dash: Some(Value::Stated(DashPattern::Dotted)),
            ..EdgePatch::default()
        }),
        ..StrokePatch::default()
    };
    assert_eq!(earlier.merge(later.clone()), Ok(()));
    assert_eq!(
        earlier.top,
        Some(EdgePatch {
            width: Some(Value::Stated(Px(1.0))),
            dash: Some(Value::Stated(DashPattern::Dotted)),
            ..EdgePatch::default()
        })
    );

    let mut value = stroke();
    assert_eq!(value.apply(later), Ok(()));
    assert_eq!(value.top.dash, Value::Stated(DashPattern::Dotted));
    assert_eq!(value.top.width, Value::Stated(Px(1.0)));
    assert_eq!(value.right, edge(1.0));
}

#[test]
fn fields_over_a_set_block_apply_to_it() {
    let fields = StrokePatch {
        fill_clip: Some(FillClip::InnerEdge),
        ..StrokePatch::default()
    };
    let mut patch = BlockPatch::Set(Value::Stated(stroke()));
    assert_eq!(patch.merge(BlockPatch::Fields(fields.clone())), Ok(()));
    let mut expected = stroke();
    expected.fill_clip = FillClip::InnerEdge;
    assert_eq!(patch, BlockPatch::Set(Value::Stated(expected.clone())));

    let mut patch = BlockPatch::Fields(StrokePatch {
        position: Some(StrokePosition::Center),
        ..StrokePatch::default()
    });
    assert_eq!(patch.merge(BlockPatch::Fields(fields)), Ok(()));
    assert_eq!(
        patch,
        BlockPatch::Fields(StrokePatch {
            position: Some(StrokePosition::Center),
            fill_clip: Some(FillClip::InnerEdge),
            ..StrokePatch::default()
        })
    );
    let mut value = Value::Stated(stroke());
    assert_eq!(value.apply(patch), Ok(()));
    expected.position = StrokePosition::Center;
    assert_eq!(value, Value::Stated(expected));
}

#[test]
fn set_after_fields_replaces_the_block() {
    let mut patch = BlockPatch::Fields(StrokePatch {
        position: Some(StrokePosition::Center),
        ..StrokePatch::default()
    });
    assert_eq!(patch.merge(BlockPatch::Set(Value::Absent)), Ok(()));
    assert_eq!(patch, BlockPatch::<Stroke>::Set(Value::Absent));
}

#[test]
fn fields_over_absent_auto_or_a_reference_are_not_merged() {
    let fields = || {
        BlockPatch::Fields(StrokePatch {
            position: Some(StrokePosition::Center),
            ..StrokePatch::default()
        })
    };
    for (below, expected) in [
        (Value::Absent, Below::Absent),
        (Value::Auto, Below::Auto),
        (Value::Expr(reference()), Below::Reference),
    ] {
        let mut patch = BlockPatch::<Stroke>::Set(below.clone());
        assert_eq!(patch.merge(fields()), conflict("", expected));
        let mut value = below;
        assert_eq!(value.apply(fields()), conflict("", expected));
    }
}

#[test]
fn a_conflict_names_its_field_path() {
    let mut value = stroke();
    value.top.paint = Value::Expr(reference());
    let patch = StrokePatch {
        top: Some(EdgePatch {
            paint: Some(PaintPatch::Append(vec![color_layer(0.1)])),
            ..EdgePatch::default()
        }),
        ..StrokePatch::default()
    };
    assert_eq!(
        value.clone().apply(patch.clone()),
        conflict("top.paint", Below::Reference)
    );
    let mut earlier = StrokePatch {
        top: Some(EdgePatch {
            paint: Some(PaintPatch::Set(Value::Auto)),
            ..EdgePatch::default()
        }),
        ..StrokePatch::default()
    };
    assert_eq!(earlier.merge(patch), conflict("top.paint", Below::Auto));
}

/// A probe block, for what the model's blocks do not hold: a list of blocks
/// in a struct block, and an `auto` length.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
struct Probe {
    #[parts(block = "fields")]
    stops: Vec<Stop>,
    #[parts(auto)]
    width: Value<Px>,
}

/// A probe of a map holding blocks, an `auto` font size and a nested block
/// named by its path.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
struct MapProbe {
    #[parts(block = "append")]
    paints: Vec<(crate::next::Key, Value<super::paint::Paint>)>,
    #[parts(block = "set")]
    shadows: Vec<(crate::next::Key, super::shadow::Shadow)>,
    #[parts(auto)]
    line_height: Value<crate::next::FontSize>,
    #[parts(block = "fields")]
    edge: super::outline::Edge,
}

#[test]
fn a_map_of_blocks_resolves_its_blocks_and_merges_by_key() {
    let resolved = ResolvedMapProbe {
        paints: vec![(key!("a"), None)],
        shadows: vec![(key!("b"), Vec::new())],
        line_height: Some(Length::Auto),
        edge: ResolvedEdge {
            width: None,
            paint: None,
            dash: None,
            snap: PixelSnap::None,
        },
    };
    let _: Vec<(crate::next::Key, Option<super::paint::ResolvedPaint>)> = resolved.paints;
    let _: Option<Length> = resolved.line_height;
    let _: Option<super::outline::EdgePatch> = MapProbePatch::default().edge;
    assert_eq!(
        field(MapProbe::FIELDS, "paints").map(|f| f.ty),
        Some(FieldType::Block {
            ty: "Vec<(crate::next::Key,Value<super::paint::Paint>)>",
            form: BlockForm::Struct,
            fields: Paint::FIELDS
        })
    );
    assert_eq!(
        field(MapProbe::FIELDS, "line_height").map(|f| f.auto),
        Some(true)
    );

    let mut earlier = MapProbePatch {
        paints: Some(vec![
            (key!("a"), paint(vec![color_layer(0.1)])),
            (key!("b"), Value::Absent),
        ]),
        ..MapProbePatch::default()
    };
    let later = MapProbePatch {
        paints: Some(vec![(key!("b"), paint(vec![color_layer(0.2)]))]),
        ..MapProbePatch::default()
    };
    assert_eq!(earlier.merge(later), Ok(()));
    assert_eq!(
        earlier.paints,
        Some(vec![
            (key!("a"), paint(vec![color_layer(0.1)])),
            (key!("b"), paint(vec![color_layer(0.2)])),
        ])
    );
}

#[test]
fn a_merge_can_succeed_where_applying_in_order_conflicts() {
    let earlier = BlockPatch::Fields(StrokePatch {
        position: Some(StrokePosition::Center),
        ..StrokePatch::default()
    });
    let later = BlockPatch::Set(Value::Stated(stroke()));

    let mut in_order: Value<Stroke> = Value::Absent;
    assert_eq!(in_order.apply(earlier.clone()), conflict("", Below::Absent));

    let mut merged = earlier;
    assert_eq!(merged.merge(later), Ok(()));
    let mut value: Value<Stroke> = Value::Absent;
    assert_eq!(value.apply(merged), Ok(()));
    assert_eq!(value, Value::Stated(stroke()));
}

#[test]
fn a_table_names_its_form_and_unit_variants() {
    assert_eq!(Stroke::FORM, BlockForm::Struct);
    assert_eq!(
        Radius::FORM,
        BlockForm::Enum {
            unit_variants: &["follow_part"]
        }
    );
    assert_eq!(PaintKind::FORM, BlockForm::Enum { unit_variants: &[] });
    assert_eq!(
        field(Corners::FIELDS, "top_left").map(|f| f.ty),
        Some(FieldType::Block {
            ty: "Radius",
            form: Radius::FORM,
            fields: Radius::FIELDS
        })
    );
}

#[test]
fn resolved_types_map_values_to_options() {
    let probe = ResolvedProbe {
        stops: Vec::new(),
        width: Some(Length::Auto),
    };
    assert_eq!(probe.width, Some(Length::Auto));
    let edge = ResolvedEdge {
        width: Some(Px(1.0)),
        paint: None,
        dash: Some(DashPattern::Solid),
        snap: PixelSnap::None,
    };
    assert_eq!(edge.paint, None);
    let corner = ResolvedRadius::Full { maximum: None };
    assert_ne!(corner, ResolvedRadius::FollowPart);
    let linear = ResolvedPaintKind::Linear {
        angle: Deg(90.0),
        stops: Vec::new(),
        repeat: false,
        extent: None,
        offset: Some(Some(Px(2.0))),
    };
    assert!(matches!(linear, ResolvedPaintKind::Linear { .. }));
    let shadow: ResolvedShadow = Vec::new();
    assert!(shadow.is_empty());
    assert_eq!(Probe::FIELDS.len(), 2);
}

/// Every block of S2.3–S2.6 and its own table.
fn tables() -> [(&'static str, &'static [FieldInfo]); 12] {
    [
        ("Paint", Paint::FIELDS),
        ("PaintLayer", PaintLayer::FIELDS),
        ("PaintKind", PaintKind::FIELDS),
        ("Stop", Stop::FIELDS),
        ("Material", Material::FIELDS),
        ("Corners", Corners::FIELDS),
        ("Radius", Radius::FIELDS),
        ("Stroke", Stroke::FIELDS),
        ("Edge", Edge::FIELDS),
        ("Shadow", Shadow::FIELDS),
        ("ShadowLayer", ShadowLayer::FIELDS),
        ("Transform", Transform::FIELDS),
    ]
}

fn marked(block: &str, path: &str) -> bool {
    tables()
        .into_iter()
        .filter(|(name, _)| *name == block)
        .flat_map(|(_, fields)| fields)
        .any(|f| f.path == path && f.may_be_absent)
}

#[test]
fn material_parameters_but_identity_and_fallback_may_be_absent() {
    for path in [
        "blur_radius",
        "tint",
        "saturation",
        "luminosity",
        "noise_opacity",
    ] {
        assert!(marked("Material", path), "{path}");
    }
    for path in ["identity", "fallback", "params"] {
        assert!(!marked("Material", path), "{path}");
    }
}

#[test]
fn edge_width_and_paint_may_be_absent() {
    assert!(marked("Edge", "width"));
    assert!(marked("Edge", "paint"));
}

#[test]
fn edge_dash_may_be_absent() {
    assert!(marked("Edge", "dash"));
}

#[test]
fn stroke_outset_may_be_absent() {
    assert!(marked("Stroke", "outset"));
}

#[test]
fn concentric_minimum_may_be_absent() {
    assert!(marked("Radius", "concentric.minimum"));
}

#[test]
fn full_maximum_may_be_absent() {
    assert!(marked("Radius", "full.maximum"));
}

#[test]
fn no_other_field_may_be_absent_or_auto() {
    let mut found: Vec<String> = tables()
        .into_iter()
        .flat_map(|(name, fields)| {
            fields
                .iter()
                .filter(|f| f.may_be_absent)
                .map(move |f| format!("{name}.{}", f.path))
        })
        .collect();
    found.sort();
    let mut expected = [
        "Edge.dash",
        "Edge.paint",
        "Edge.width",
        "Material.blur_radius",
        "Material.luminosity",
        "Material.noise_opacity",
        "Material.saturation",
        "Material.tint",
        "Radius.concentric.minimum",
        "Radius.full.maximum",
        "Stroke.outset",
    ];
    expected.sort_unstable();
    assert_eq!(found, expected);
    assert!(
        tables()
            .into_iter()
            .flat_map(|(_, fields)| fields)
            .all(|f| !f.auto)
    );
}

fn field(fields: &'static [FieldInfo], path: &str) -> Option<&'static FieldInfo> {
    fields.iter().find(|f| f.path == path)
}

fn below(fields: &'static [FieldInfo], path: &str) -> &'static [FieldInfo] {
    match field(fields, path).map(|f| f.ty) {
        Some(FieldType::Block { fields, .. }) => fields,
        _ => &[],
    }
}

#[test]
fn field_tables_reach_nested_blocks() {
    let paint = below(below(Stroke::FIELDS, "top"), "paint");
    let kind = below(below(paint, "layers"), "kind");
    let material = below(kind, "material");
    assert_eq!(
        field(material, "blur_radius").map(|f| (f.ty, f.may_be_absent)),
        Some((FieldType::Leaf("Value<Px>"), true))
    );
    assert_eq!(
        field(Edge::FIELDS, "paint").map(|f| f.ty),
        Some(FieldType::Block {
            ty: "Value<Paint>",
            form: BlockForm::Struct,
            fields: Paint::FIELDS
        })
    );
    assert_eq!(
        field(Material::FIELDS, "params").map(|f| f.ty),
        Some(FieldType::Leaf("Vec<(Key,Value<Scalar>)>"))
    );
    assert_eq!(
        field(Radius::FIELDS, "px").map(|f| f.ty),
        Some(FieldType::Leaf("Value<Px>"))
    );
    assert_eq!(
        field(Shadow::FIELDS, "layers").map(|f| f.ty),
        Some(FieldType::Block {
            ty: "Vec<ShadowLayer>",
            form: BlockForm::Struct,
            fields: ShadowLayer::FIELDS
        })
    );
    let linear: Vec<_> = PaintKind::FIELDS
        .iter()
        .map(|f| f.path)
        .filter(|path| path.starts_with("linear."))
        .collect();
    assert_eq!(
        linear,
        [
            "linear.angle",
            "linear.stops",
            "linear.repeat",
            "linear.extent",
            "linear.offset"
        ]
    );
    let corners: Vec<_> = Corners::FIELDS.iter().map(|f| f.path).collect();
    assert_eq!(
        corners,
        [
            "top_left",
            "top_right",
            "bottom_right",
            "bottom_left",
            "curve"
        ]
    );
    assert_eq!(Shadow::FIELDS.len(), 2);
    let _: CornerCurve = CornerCurve::Continuous;
}

#[test]
fn shadow_and_transform_are_blocks() {
    let layer = ShadowLayer {
        x: Value::Stated(Px(0.0)),
        y: Value::Stated(Px(1.0)),
        blur: Value::Stated(Px(2.0)),
        spread: Value::Stated(Px(0.0)),
        color: Value::Absent,
        inset: false,
    };
    let shadow = Shadow::Layers(vec![layer]);
    assert!(matches!(shadow, Shadow::Layers(ref layers) if layers.len() == 1));
    let mut transform = Transform {
        scale_x: Value::Stated(1.0),
        scale_y: Value::Stated(1.0),
        rotate: Value::Stated(Deg(0.0)),
        translate_x: Value::Stated(Px(0.0)),
        translate_y: Value::Stated(Px(0.0)),
        origin: (0.5, 0.5),
    };
    let patch = super::transform::TransformPatch {
        rotate: Some(Value::Stated(Deg(45.0))),
        origin: Some((0.0, 0.0)),
        ..Default::default()
    };
    assert_eq!(transform.apply(patch), Ok(()));
    assert_eq!(transform.rotate, Value::Stated(Deg(45.0)));
    assert_eq!(transform.origin, (0.0, 0.0));
    let corners = Corners {
        top_left: Radius::Px(Value::Stated(Px(4.0))),
        top_right: Radius::Full {
            maximum: Value::Absent,
        },
        bottom_right: Radius::Concentric {
            minimum: Value::Absent,
        },
        bottom_left: Radius::FollowPart,
        curve: CornerCurve::Circular,
    };
    assert!(matches!(corners.bottom_left, Radius::FollowPart));
}
