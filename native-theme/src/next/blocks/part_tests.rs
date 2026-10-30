//! The blocks of S2.7–S2.11: their tables, marks, merges and functions
//! (S2.4, S2.7, S6.1, S6.3). Every number here is synthetic.

use super::focus::{FocusIndicator, FocusIndicatorPatch, FocusKind, FocusVisibleWhen};
use super::icon::{Glyph, GlyphStroke, IconStyle};
use super::motion::{
    AdwEasing, Keyframe, Loop, PlatformCurve, PresentationEvent, PresentationKind,
    PresentationStep, PresentationTransition, PropertyGroup, QtEasing, ResolvedKeyframe,
    ResolvedTiming, Timing, Transition, TransitionKind, damping_coefficient, evaluate,
};
use super::outline::{
    CornerCurve, Corners, CornersPatch, DashPattern, Radius, concentric, follow_part, full,
};
use super::paint::{BlendMode, Paint, PaintKind, PaintLayer, PaintPatch};
use super::part_style::{
    Binding, PartStyle, PartStylePatch, ResolvedPartStyle, ResolvedPartStylePatch, RoleRef,
};
use super::text::{
    AxisTag, Decoration, Elision, FadeLength, FeatureTag, Font, LanguageTag, ResolvedElision,
    ResolvedFadeLength, ResolvedTextStyle, TextStyle, TextStylePatch,
};
use super::{Apply, BlockPatch, KeyedItem, Merge, Table};
use crate::key;
use crate::next::color::Color;
use crate::next::registry::{FieldInfo, FieldType, join, leaves};
use crate::next::roles::SurfaceSet;
use crate::next::units::{Axes, FontSize, Fraction, Length, Ms, Px, Sides};
use crate::next::value::Value;

fn under(prefix: &str, paths: &[&str]) -> Vec<String> {
    paths.iter().map(|path| join(prefix, path)).collect()
}

/// S2.3's paint, its leaves as S2.3 states them.
fn paint(prefix: &str) -> Vec<String> {
    under(
        prefix,
        &[
            "layers.kind.color",
            "layers.kind.linear.angle",
            "layers.kind.linear.stops.position",
            "layers.kind.linear.stops.color",
            "layers.kind.linear.repeat",
            "layers.kind.linear.extent",
            "layers.kind.linear.offset",
            "layers.kind.radial.center",
            "layers.kind.radial.radius",
            "layers.kind.radial.stops.position",
            "layers.kind.radial.stops.color",
            "layers.kind.radial.repeat",
            "layers.kind.conic.center",
            "layers.kind.conic.start_angle",
            "layers.kind.conic.stops.position",
            "layers.kind.conic.stops.color",
            "layers.kind.pattern.pattern",
            "layers.kind.pattern.foreground",
            "layers.kind.pattern.background",
            "layers.kind.pattern.cell",
            "layers.kind.material.identity",
            "layers.kind.material.blur_radius",
            "layers.kind.material.tint",
            "layers.kind.material.saturation",
            "layers.kind.material.luminosity",
            "layers.kind.material.noise_opacity",
            "layers.kind.material.fallback",
            "layers.kind.material.params",
            "layers.blend",
        ],
    )
}

/// S2.4's edge.
fn edge(prefix: &str) -> Vec<String> {
    let mut out = under(prefix, &["width"]);
    out.extend(paint(&join(prefix, "paint")));
    out.extend(under(prefix, &["dash", "snap"]));
    out
}

/// S2.4's corners: each corner a `Radius`, `follow_part` its unit variant.
fn corners(prefix: &str) -> Vec<String> {
    let mut out = Vec::new();
    for corner in ["top_left", "top_right", "bottom_right", "bottom_left"] {
        out.extend(under(
            &join(prefix, corner),
            &["", "px", "full.maximum", "concentric.minimum"],
        ));
    }
    out.push(join(prefix, "curve"));
    out.into_iter()
        .map(|path| path.trim_end_matches('.').to_owned())
        .collect()
}

/// S2.5's shadow.
fn shadow(prefix: &str) -> Vec<String> {
    under(
        prefix,
        &[
            "layers.x",
            "layers.y",
            "layers.blur",
            "layers.spread",
            "layers.color",
            "layers.inset",
            "elevation",
        ],
    )
}

/// S2.7's timing: `linear` its unit variant, `platform` an enum block whose
/// unit variant is `kirigami_velocity` (ADD-A123).
fn timing(prefix: &str) -> Vec<String> {
    let mut out = vec![prefix.to_owned()];
    out.extend(under(
        prefix,
        &[
            "cubic_bezier",
            "steps.count",
            "steps.jump",
            "spring.mass",
            "spring.stiffness",
            "spring.damping_ratio",
            "spring.initial_velocity",
            "spring.rest_epsilon",
            "spring.clamp",
            "spring.time_scale",
            "platform",
            "platform.qt",
            "platform.adw",
        ],
    ));
    out
}

/// S2.7's transition: `interpolate` and `focus_move` its kind's unit
/// variants.
fn transition_paths(prefix: &str) -> Vec<String> {
    let mut out = under(prefix, &["properties", "kind"]);
    out.extend(under(
        &join(prefix, "kind"),
        &[
            "ripple.max_radius",
            "ripple.initial_scale",
            "ripple.padding",
            "ripple.soft_edge",
            "ripple.opacity",
            "ripple.fade",
            "ripple.min_press",
            "ripple.touch_delay",
            "keyed.key",
            "keyed.params",
        ],
    ));
    out.extend(under(prefix, &["duration", "delay"]));
    out.extend(timing(&join(prefix, "timing")));
    out.push(join(prefix, "suspended_by_theme_change"));
    out
}

/// S2.8's decoration.
fn decoration(prefix: &str) -> Vec<String> {
    let mut out = under(prefix, &["line", "style", "by_word", "thickness", "offset"]);
    out.extend(paint(&join(prefix, "paint")));
    out
}

/// Every leaf path S2.11's part style implies, through S2.3–S2.10's blocks.
fn part_style_paths() -> Vec<String> {
    let mut out = paint("fill");
    for side in ["top", "right", "bottom", "left"] {
        out.extend(edge(&join("stroke", side)));
    }
    out.extend(under("stroke", &["position", "outset", "fill_clip"]));
    out.extend(corners("corners"));
    out.push("shape".to_owned());
    out.extend(shadow("shadow"));
    out.push("opacity".to_owned());
    out.extend(under(
        "transform",
        &[
            "scale_x",
            "scale_y",
            "rotate",
            "translate_x",
            "translate_y",
            "origin",
        ],
    ));
    out.extend(under(
        "text.font",
        &[
            "family", "size", "weight", "style", "width", "axes", "features",
        ],
    ));
    out.extend(under(
        "text",
        &[
            "scaling_role",
            "letter_spacing",
            "word_spacing",
            "line_height",
        ],
    ));
    out.extend(decoration("text.decoration"));
    out.extend(decoration("text.mnemonic_decoration"));
    out.extend(under(
        "text",
        &[
            "case",
            "elision",
            "elision.fade.start.px",
            "elision.fade.start.fraction",
            "elision.fade.end.px",
            "elision.fade.end.fraction",
            "elision.fade.under",
            "wrap",
            "max_lines",
        ],
    ));
    out.extend(paint("text.fill"));
    out.push("icon.size".to_owned());
    out.extend(paint("icon.fill"));
    out.extend(under(
        "icon",
        &[
            "rendering_mode",
            "weight",
            "scale",
            "axes",
            "state_effect",
            "symbol_variants",
        ],
    ));
    out.extend(paint("icon.status_colors"));
    out.push("icon.animation.period".to_owned());
    out.extend(timing("icon.animation.timing"));
    out.extend(under("icon.animation.keyframes", &["at", "part"]));
    out.extend(timing("icon.animation.keyframes.timing"));
    out.push("icon.animation.keyframes.patch".to_owned());
    out.extend(under("glyph", &["source", "size"]));
    out.extend(paint("glyph.fill"));
    out.extend(paint("glyph.stroke.paint"));
    out.extend(under(
        "glyph",
        &[
            "stroke.width",
            "stroke.cap",
            "stroke.join",
            "axes",
            "draw_extent",
            "draw_start",
            "mirroring",
        ],
    ));
    out.extend(under("focus", &["kind", "width"]));
    out.extend(paint("focus.paint"));
    out.extend(under(
        "focus",
        &[
            "dash",
            "offset.top",
            "offset.right",
            "offset.bottom",
            "offset.left",
        ],
    ));
    out.extend(corners("focus.corners"));
    out.extend(shadow("focus.shadow"));
    out.extend(transition_paths("focus.transition"));
    out.extend(under(
        "",
        &[
            "padding.top",
            "padding.right",
            "padding.bottom",
            "padding.left",
            "margin.top",
            "margin.right",
            "margin.bottom",
            "margin.left",
            "min_width",
            "min_height",
            "max_width",
            "max_height",
            "cursor",
            "touch_target",
            "bindings.surface",
            "bindings.role.role",
            "bindings.role.value",
        ],
    ));
    out.extend(transition_paths("transitions"));
    out
}

#[test]
fn part_style_registry_paths() {
    let mut found = Vec::new();
    leaves("", PartStyle::FIELDS, &mut found);
    let expected = part_style_paths();
    for (index, (found, expected)) in found.iter().zip(&expected).enumerate() {
        assert_eq!(found, expected, "path {index}");
    }
    assert_eq!(found.len(), expected.len());
}

fn table_of(fields: &'static [FieldInfo], path: &str) -> Option<&'static FieldInfo> {
    fields.iter().find_map(|field| {
        if field.path == path {
            return Some(field);
        }
        let rest = path.strip_prefix(field.path)?.strip_prefix('.')?;
        match field.ty {
            FieldType::Block { fields, .. } => table_of(fields, rest),
            FieldType::Patch { fields, .. } => table_of(fields(), rest),
            FieldType::Leaf(_) => None,
        }
    })
}

fn may_be_absent(fields: &'static [FieldInfo], path: &str) -> bool {
    table_of(fields, path).is_some_and(|f| f.may_be_absent)
}

fn auto(fields: &'static [FieldInfo], path: &str) -> bool {
    table_of(fields, path).is_some_and(|f| f.auto)
}

/// Holds each field of a resolved part style and text style as S6.1 types
/// it: a length that may be `Auto` as `Option<Length>` on every part.
fn resolved_types(style: ResolvedPartStyle, text: ResolvedTextStyle) {
    let _: Option<Length> = style.min_width;
    let _: Option<Length> = style.min_height;
    let _: Option<Length> = style.max_width;
    let _: Option<Length> = style.max_height;
    let _: Option<Px> = style.touch_target;
    let _: Sides<Option<Px>> = style.padding;
    let _: Option<Length> = text.line_height;
    let _: Option<Px> = text.letter_spacing;
    let _: Option<Px> = text.font.size;
}

#[test]
fn resolved_length_auto_types() {
    let _ = resolved_types as fn(ResolvedPartStyle, ResolvedTextStyle);
    for path in ["min_width", "min_height", "max_width", "max_height"] {
        assert!(auto(PartStyle::FIELDS, path), "{path}");
    }
    assert!(auto(TextStyle::FIELDS, "line_height"));
    assert!(auto(PartStyle::FIELDS, "text.line_height"));
}

/// Every block of S2.7–S2.11 and its own table.
fn tables() -> Vec<(&'static str, &'static [FieldInfo])> {
    vec![
        ("Timing", Timing::FIELDS),
        ("Transition", Transition::FIELDS),
        ("TransitionKind", TransitionKind::FIELDS),
        ("PresentationTransition", PresentationTransition::FIELDS),
        ("PresentationStep", PresentationStep::FIELDS),
        ("PresentationKind", PresentationKind::FIELDS),
        ("Loop", Loop::FIELDS),
        ("Keyframe", Keyframe::FIELDS),
        ("Font", Font::FIELDS),
        ("TextStyle", TextStyle::FIELDS),
        ("Decoration", Decoration::FIELDS),
        ("Elision", Elision::FIELDS),
        ("FadeLength", FadeLength::FIELDS),
        ("IconStyle", IconStyle::FIELDS),
        ("Glyph", Glyph::FIELDS),
        ("GlyphStroke", GlyphStroke::FIELDS),
        ("FocusIndicator", FocusIndicator::FIELDS),
        ("FocusVisibleWhen", FocusVisibleWhen::FIELDS),
        ("PartStyle", PartStyle::FIELDS),
        ("Binding", Binding::FIELDS),
    ]
}

fn marked(block: &str, path: &str) -> bool {
    tables()
        .into_iter()
        .filter(|(name, _)| *name == block)
        .any(|(_, fields)| may_be_absent(fields, path))
}

#[test]
fn ripple_values_may_be_absent() {
    for path in [
        "max_radius",
        "initial_scale",
        "padding",
        "soft_edge",
        "opacity",
        "fade",
        "min_press",
        "touch_delay",
    ] {
        assert!(
            marked("TransitionKind", &format!("ripple.{path}")),
            "{path}"
        );
    }
}

#[test]
fn focus_shadow_and_transition_may_be_absent() {
    assert!(marked("FocusIndicator", "shadow"));
    assert!(marked("FocusIndicator", "transition"));
    assert!(!marked("FocusIndicator", "dash"));
}

#[test]
fn icon_state_effect_weight_scale_animation_may_be_absent() {
    for path in ["state_effect", "weight", "scale", "animation"] {
        assert!(marked("IconStyle", path), "{path}");
    }
}

#[test]
fn icon_fill_may_be_absent() {
    assert!(marked("IconStyle", "fill"));
}

#[test]
fn glyph_stroke_and_draw_extent_may_be_absent() {
    assert!(marked("Glyph", "stroke"));
    assert!(marked("Glyph", "draw_extent"));
}

#[test]
fn glyph_fill_may_be_absent() {
    assert!(marked("Glyph", "fill"));
}

#[test]
fn glyph_draw_start_may_be_absent() {
    assert!(marked("Glyph", "draw_start"));
}

#[test]
fn text_decorations_and_spacings_may_be_absent() {
    for path in [
        "decoration",
        "mnemonic_decoration",
        "letter_spacing",
        "word_spacing",
    ] {
        assert!(marked("TextStyle", path), "{path}");
    }
}

#[test]
fn decoration_by_word_may_be_absent() {
    assert!(marked("Decoration", "by_word"));
}

#[test]
fn font_width_may_be_absent() {
    assert!(marked("Font", "width"));
}

#[test]
fn part_shape_touch_target_and_margin_may_be_absent() {
    for path in [
        "shape",
        "touch_target",
        "margin.top",
        "margin.right",
        "margin.bottom",
        "margin.left",
    ] {
        assert!(marked("PartStyle", path), "{path}");
    }
    assert!(!marked("PartStyle", "margin"));
}

#[test]
fn part_drawing_fields_may_be_absent() {
    for path in [
        "fill", "stroke", "shadow", "text", "icon", "glyph", "focus", "cursor",
    ] {
        assert!(marked("PartStyle", path), "{path}");
    }
}

#[test]
fn spring_durations_may_be_absent() {
    assert!(marked("Transition", "duration"));
    assert!(marked("PresentationStep", "duration"));
}

#[test]
fn keyframe_timing_may_be_absent() {
    assert!(marked("Keyframe", "timing"));
}

#[test]
fn loop_period_is_never_absent() {
    assert!(!marked("Loop", "period"));
}

#[test]
fn no_other_field_may_be_absent_or_auto() {
    let mut absent: Vec<String> = Vec::new();
    let mut autos: Vec<String> = Vec::new();
    for (name, fields) in tables() {
        for field in fields {
            if field.may_be_absent {
                absent.push(format!("{name}.{}", field.path));
            }
            if field.auto {
                autos.push(format!("{name}.{}", field.path));
            }
        }
    }
    absent.sort();
    let mut expected_absent = vec![
        "Decoration.by_word",
        "FocusIndicator.shadow",
        "FocusIndicator.transition",
        "Font.width",
        "Glyph.draw_extent",
        "Glyph.draw_start",
        "Glyph.fill",
        "Glyph.stroke",
        "IconStyle.animation",
        "IconStyle.fill",
        "IconStyle.scale",
        "IconStyle.state_effect",
        "IconStyle.weight",
        "Keyframe.timing",
        "PartStyle.cursor",
        "PartStyle.fill",
        "PartStyle.focus",
        "PartStyle.glyph",
        "PartStyle.icon",
        "PartStyle.shadow",
        "PartStyle.shape",
        "PartStyle.stroke",
        "PartStyle.text",
        "PartStyle.touch_target",
        "PresentationStep.duration",
        "TextStyle.decoration",
        "TextStyle.letter_spacing",
        "TextStyle.mnemonic_decoration",
        "TextStyle.word_spacing",
        "Transition.duration",
        "TransitionKind.ripple.fade",
        "TransitionKind.ripple.initial_scale",
        "TransitionKind.ripple.max_radius",
        "TransitionKind.ripple.min_press",
        "TransitionKind.ripple.opacity",
        "TransitionKind.ripple.padding",
        "TransitionKind.ripple.soft_edge",
        "TransitionKind.ripple.touch_delay",
    ];
    expected_absent.sort_unstable();
    assert_eq!(absent, expected_absent);
    autos.sort();
    assert_eq!(
        autos,
        [
            "PartStyle.max_height",
            "PartStyle.max_width",
            "PartStyle.min_height",
            "PartStyle.min_width",
            "TextStyle.line_height",
        ]
    );
}

fn color(r: f32) -> Value<Color> {
    Value::Stated(Color {
        r,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    })
}

fn paint_value(r: f32) -> Value<Paint> {
    Value::Stated(Paint {
        layers: vec![PaintLayer {
            kind: PaintKind::Color(color(r)),
            blend: BlendMode::Normal,
        }],
    })
}

fn decoration_value() -> Decoration {
    Decoration {
        line: super::text::DecorationLine::Underline,
        style: super::text::DecorationStyle::Solid,
        by_word: Value::Absent,
        thickness: Value::Stated(Px(1.0)),
        offset: Value::Stated(Px(2.0)),
        paint: paint_value(0.1),
    }
}

fn text_style() -> TextStyle {
    TextStyle {
        font: Font {
            family: Value::Stated("Synthetic".to_owned()),
            size: Value::Stated(FontSize::Pt(10.0)),
            weight: Value::Stated(400),
            style: Value::Stated(super::text::FontStyle::Normal),
            width: Value::Absent,
            axes: Vec::new(),
            features: Vec::new(),
        },
        scaling_role: None,
        letter_spacing: Value::Absent,
        word_spacing: Value::Absent,
        line_height: Value::Auto,
        decoration: Value::Absent,
        mnemonic_decoration: Value::Stated(decoration_value()),
        case: super::text::TextCase::Mixed,
        elision: Elision::End,
        wrap: super::text::Wrap::Word,
        max_lines: None,
        fill: paint_value(0.2),
    }
}

fn spring(damping_ratio: f32) -> Timing {
    Timing::Spring {
        mass: 1.0,
        stiffness: 100.0,
        damping_ratio,
        initial_velocity: 0.0,
        rest_epsilon: 0.001,
        clamp: false,
        time_scale: Value::Stated(1.0),
    }
}

fn transition(groups: Vec<PropertyGroup>, ms: f32) -> Transition {
    Transition {
        properties: groups,
        kind: TransitionKind::Interpolate,
        duration: Value::Stated(Ms(ms)),
        delay: Value::Stated(Ms(0.0)),
        timing: Value::Stated(Timing::Linear),
        suspended_by_theme_change: false,
    }
}

#[test]
fn a_part_styles_struct_blocks_patch_by_set_or_fields() {
    let mut earlier = PartStylePatch {
        text: Some(BlockPatch::Set(Value::Stated(text_style()))),
        ..PartStylePatch::default()
    };
    let later = PartStylePatch {
        text: Some(BlockPatch::Fields(TextStylePatch {
            max_lines: Some(Some(2)),
            mnemonic_decoration: Some(BlockPatch::Fields(super::text::DecorationPatch {
                paint: Some(PaintPatch::Append(vec![PaintLayer {
                    kind: PaintKind::Color(color(0.3)),
                    blend: BlendMode::Normal,
                }])),
                ..Default::default()
            })),
            ..TextStylePatch::default()
        })),
        ..PartStylePatch::default()
    };
    assert_eq!(earlier.merge(later), Ok(()));
    let Some(BlockPatch::Set(Value::Stated(text))) = earlier.text else {
        panic!("a Fields patch over a set text style applies to it");
    };
    assert_eq!(text.max_lines, Some(2));
    let Value::Stated(decoration) = &text.mnemonic_decoration else {
        panic!("the decoration stays stated");
    };
    let Value::Stated(paint) = &decoration.paint else {
        panic!("the paint stays stated");
    };
    assert_eq!(paint.layers.len(), 2);
    assert_eq!(text.font, text_style().font);

    let mut over_absent = PartStylePatch {
        text: Some(BlockPatch::Set(Value::Absent)),
        ..PartStylePatch::default()
    };
    let fields = PartStylePatch {
        text: Some(BlockPatch::Fields(TextStylePatch::default())),
        ..PartStylePatch::default()
    };
    assert!(over_absent.merge(fields).is_err());

    let mut set_after_fields = PartStylePatch {
        text: Some(BlockPatch::Fields(TextStylePatch {
            max_lines: Some(None),
            ..TextStylePatch::default()
        })),
        ..PartStylePatch::default()
    };
    let set = PartStylePatch {
        text: Some(BlockPatch::Set(Value::Absent)),
        ..PartStylePatch::default()
    };
    assert_eq!(set_after_fields.merge(set), Ok(()));
    assert_eq!(set_after_fields.text, Some(BlockPatch::Set(Value::Absent)));
}

#[test]
fn a_focus_indicators_corners_patch_by_fields() {
    let mut earlier = FocusIndicatorPatch {
        corners: Some(CornersPatch {
            top_left: Some(Radius::FollowPart),
            ..CornersPatch::default()
        }),
        ..FocusIndicatorPatch::default()
    };
    let later = FocusIndicatorPatch {
        corners: Some(CornersPatch {
            curve: Some(CornerCurve::Continuous),
            ..CornersPatch::default()
        }),
        ..FocusIndicatorPatch::default()
    };
    assert_eq!(earlier.merge(later), Ok(()));
    assert_eq!(
        earlier.corners,
        Some(CornersPatch {
            top_left: Some(Radius::FollowPart),
            curve: Some(CornerCurve::Continuous),
            ..CornersPatch::default()
        })
    );
    let mut focus = FocusIndicator {
        kind: FocusKind::Ring,
        width: Value::Stated(Px(2.0)),
        paint: paint_value(0.4),
        dash: Value::Stated(DashPattern::Solid),
        offset: Sides {
            top: Value::Stated(Px(1.0)),
            right: Value::Stated(Px(1.0)),
            bottom: Value::Stated(Px(1.0)),
            left: Value::Stated(Px(1.0)),
        },
        corners: Corners {
            top_left: Radius::Px(Value::Stated(Px(3.0))),
            top_right: Radius::FollowPart,
            bottom_right: Radius::FollowPart,
            bottom_left: Radius::FollowPart,
            curve: CornerCurve::Circular,
        },
        shadow: Value::Absent,
        transition: Value::Absent,
    };
    assert_eq!(focus.apply(earlier), Ok(()));
    assert_eq!(focus.corners.top_left, Radius::FollowPart);
    assert_eq!(focus.corners.curve, CornerCurve::Continuous);
}

#[test]
fn sides_patch_member_by_member() {
    let side = |top: Option<f32>, left: Option<f32>| Sides {
        top: top.map(|n| Value::Stated(Px(n))),
        right: None,
        bottom: None,
        left: left.map(|n| Value::Stated(Px(n))),
    };
    let mut earlier = PartStylePatch {
        padding: Some(side(Some(1.0), Some(2.0))),
        ..PartStylePatch::default()
    };
    let later = PartStylePatch {
        padding: Some(side(Some(3.0), None)),
        margin: Some(side(None, Some(4.0))),
        ..PartStylePatch::default()
    };
    assert_eq!(earlier.merge(later), Ok(()));
    assert_eq!(earlier.padding, Some(side(Some(3.0), Some(2.0))));
    assert_eq!(earlier.margin, Some(side(None, Some(4.0))));

    let mut focus = FocusIndicatorPatch {
        offset: Some(side(Some(1.0), None)),
        ..FocusIndicatorPatch::default()
    };
    assert_eq!(
        focus.merge(FocusIndicatorPatch {
            offset: Some(side(None, Some(5.0))),
            ..FocusIndicatorPatch::default()
        }),
        Ok(())
    );
    assert_eq!(focus.offset, Some(side(Some(1.0), Some(5.0))));
}

#[test]
fn enum_blocks_are_set_whole() {
    let mut earlier = PartStylePatch {
        transitions: None,
        text: Some(BlockPatch::Fields(TextStylePatch {
            elision: Some(Elision::Fade {
                start: Some(FadeLength::Px(Value::Stated(Px(8.0)))),
                end: None,
                under: None,
            }),
            ..TextStylePatch::default()
        })),
        ..PartStylePatch::default()
    };
    let later = PartStylePatch {
        text: Some(BlockPatch::Fields(TextStylePatch {
            elision: Some(Elision::Fade {
                start: None,
                end: Some(FadeLength::Fraction(Value::Stated(Fraction(0.1)))),
                under: None,
            }),
            ..TextStylePatch::default()
        })),
        ..PartStylePatch::default()
    };
    assert_eq!(earlier.merge(later), Ok(()));
    let Some(BlockPatch::Fields(text)) = earlier.text else {
        panic!("two Fields patches merge into one");
    };
    assert_eq!(
        text.elision,
        Some(Elision::Fade {
            start: None,
            end: Some(FadeLength::Fraction(Value::Stated(Fraction(0.1)))),
            under: None,
        })
    );

    let mut step = super::motion::PresentationStepPatch {
        timing: Some(Value::Stated(spring(0.5))),
        kind: Some(PresentationKind::Fade),
        ..Default::default()
    };
    assert_eq!(
        step.merge(super::motion::PresentationStepPatch {
            timing: Some(Value::Stated(Timing::CubicBezier(0.2, 0.0, 0.0, 1.0))),
            kind: Some(PresentationKind::Scale { from: 0.9 }),
            ..Default::default()
        }),
        Ok(())
    );
    assert_eq!(
        step.timing,
        Some(Value::Stated(Timing::CubicBezier(0.2, 0.0, 0.0, 1.0)))
    );
    assert_eq!(step.kind, Some(PresentationKind::Scale { from: 0.9 }));
    let resolved = ResolvedElision::Fade {
        start: Some(ResolvedFadeLength::Px(Some(Px(8.0)))),
        end: None,
        under: None,
    };
    assert_ne!(resolved, ResolvedElision::End);
    assert_eq!(
        ResolvedTiming::CubicBezier(0.2, 0.0, 0.0, 1.0),
        ResolvedTiming::CubicBezier(0.2, 0.0, 0.0, 1.0)
    );
}

#[test]
fn bindings_merge_by_role_or_surface() {
    let role = |r| Binding::Role {
        role: RoleRef::Keyed(key!("shade_light")),
        value: color(r),
    };
    let other = Binding::Role {
        role: RoleRef::Keyed(key!("shade_dark")),
        value: color(0.9),
    };
    let mut earlier = PartStylePatch {
        bindings: Some(vec![
            Binding::Surface(SurfaceSet::View),
            role(0.1),
            other.clone(),
        ]),
        ..PartStylePatch::default()
    };
    let later = PartStylePatch {
        bindings: Some(vec![role(0.2), Binding::Surface(SurfaceSet::Card)]),
        ..PartStylePatch::default()
    };
    assert_eq!(earlier.merge(later), Ok(()));
    assert_eq!(
        earlier.bindings,
        Some(vec![Binding::Surface(SurfaceSet::Card), role(0.2), other])
    );
}

#[test]
fn transitions_merge_by_property_group_split_first() {
    let mut list = vec![transition(
        vec![PropertyGroup::Fill, PropertyGroup::Stroke],
        100.0,
    )];
    Transition::merge_keyed(
        &mut list,
        vec![
            transition(vec![PropertyGroup::Stroke], 200.0),
            transition(vec![PropertyGroup::Opacity], 300.0),
        ],
    );
    assert_eq!(
        list,
        vec![
            transition(vec![PropertyGroup::Fill], 100.0),
            transition(vec![PropertyGroup::Stroke], 200.0),
            transition(vec![PropertyGroup::Opacity], 300.0),
        ]
    );

    let mut patch = PartStylePatch {
        transitions: Some(vec![transition(vec![PropertyGroup::Fill], 100.0)]),
        ..PartStylePatch::default()
    };
    assert_eq!(
        patch.merge(PartStylePatch {
            transitions: Some(vec![transition(vec![PropertyGroup::Fill], 150.0)]),
            ..PartStylePatch::default()
        }),
        Ok(())
    );
    assert_eq!(
        patch.transitions,
        Some(vec![transition(vec![PropertyGroup::Fill], 150.0)])
    );
}

#[test]
fn a_transition_listing_no_group_is_kept_and_a_first_patch_split() {
    let mut list = vec![transition(Vec::new(), 100.0)];
    Transition::merge_keyed(
        &mut list,
        vec![transition(vec![PropertyGroup::Fill], 200.0)],
    );
    assert_eq!(
        list,
        vec![
            transition(Vec::new(), 100.0),
            transition(vec![PropertyGroup::Fill], 200.0),
        ]
    );
    Transition::merge_keyed(&mut list, vec![transition(Vec::new(), 300.0)]);
    assert_eq!(list.first(), Some(&transition(Vec::new(), 300.0)));
    assert_eq!(list.len(), 2);

    let mut patch = PartStylePatch::default();
    assert_eq!(
        patch.merge(PartStylePatch {
            transitions: Some(vec![
                transition(vec![PropertyGroup::Fill, PropertyGroup::Stroke], 100.0),
                transition(Vec::new(), 50.0),
            ]),
            ..PartStylePatch::default()
        }),
        Ok(())
    );
    assert_eq!(
        patch.transitions,
        Some(vec![
            transition(vec![PropertyGroup::Fill], 100.0),
            transition(vec![PropertyGroup::Stroke], 100.0),
            transition(Vec::new(), 50.0),
        ])
    );
}

#[test]
fn presentations_merge_by_event() {
    let presentation = |event, kind| PresentationTransition {
        event,
        steps: vec![PresentationStep {
            part: None,
            kind,
            duration: Value::Stated(Ms(100.0)),
            delay: Value::Stated(Ms(0.0)),
            timing: Value::Stated(Timing::Linear),
            stagger: Value::Stated(Ms(0.0)),
        }],
    };
    let mut list = vec![
        presentation(PresentationEvent::Appear, PresentationKind::Fade),
        presentation(PresentationEvent::Disappear, PresentationKind::Fade),
    ];
    PresentationTransition::merge_keyed(
        &mut list,
        vec![presentation(
            PresentationEvent::Appear,
            PresentationKind::Instant,
        )],
    );
    assert_eq!(
        list,
        vec![
            presentation(PresentationEvent::Appear, PresentationKind::Instant),
            presentation(PresentationEvent::Disappear, PresentationKind::Fade),
        ]
    );
}

#[test]
fn spring_damping_coefficient_from_its_ratio() {
    for (mass, stiffness, ratio, coefficient) in [
        (1.0, 100.0, 0.5, 10.0),
        (1.0, 100.0, 1.0, 20.0),
        (0.5, 50.0, 1.5, 15.0),
        (1.0, 0.0, 1.0, 0.0),
    ] {
        let found = damping_coefficient(mass, stiffness, ratio);
        assert!(
            found.is_some_and(|c| (c - coefficient).abs() < 1e-5),
            "{mass} {stiffness} {ratio}: {found:?}"
        );
    }
    assert_eq!(damping_coefficient(-1.0, 100.0, 1.0), None);
    assert_eq!(damping_coefficient(f32::MAX, f32::MAX, 1.0), None);
    let Timing::Spring { damping_ratio, .. } = spring(0.7) else {
        panic!("a spring");
    };
    assert_eq!(damping_ratio, 0.7);
}

/// The platform curves at the elapsed fractions 0, ¼, ½, ¾, 0.9 and 1, each
/// the toolkit's formula computed in doubles and rounded to `f32` (Qt's
/// `easing.cpp`, libadwaita's `adw-easing.c`; S2.7, ADD-A123).
#[test]
fn platform_curves_evaluate_to_their_sources() {
    use PlatformCurve::{Adw, Qt};
    let ts = [0.0, 0.25, 0.5, 0.75, 0.9, 1.0];
    let rows: [(PlatformCurve, [f32; 6]); 13] = [
        (
            Qt(QtEasing::InQuad),
            [0.0, 0.0625, 0.25, 0.5625, 0.809_999_94, 1.0],
        ),
        (
            Qt(QtEasing::OutQuad),
            [0.0, 0.4375, 0.75, 0.9375, 0.99, 1.0],
        ),
        (Qt(QtEasing::InOutQuad), [0.0, 0.125, 0.5, 0.875, 0.98, 1.0]),
        (
            Qt(QtEasing::InCubic),
            [0.0, 0.015625, 0.125, 0.421875, 0.728_999_9, 1.0],
        ),
        (
            Qt(QtEasing::OutCubic),
            [0.0, 0.578125, 0.875, 0.984375, 0.999, 1.0],
        ),
        (
            Qt(QtEasing::InOutCubic),
            [0.0, 0.0625, 0.5, 0.9375, 0.996, 1.0],
        ),
        (
            Qt(QtEasing::InOutSine),
            [0.0, 0.146_446_62, 0.5, 0.853_553_4, 0.975_528_24, 1.0],
        ),
        (
            Qt(QtEasing::OutExpo),
            [
                0.0,
                0.824_046_55,
                0.969_718_75,
                0.995_470_2,
                0.999_044_9,
                1.0,
            ],
        ),
        (
            Adw(AdwEasing::EaseInCubic),
            [0.0, 0.015625, 0.125, 0.421875, 0.728_999_9, 1.0],
        ),
        (
            Adw(AdwEasing::EaseOutCubic),
            [0.0, 0.578125, 0.875, 0.984375, 0.999, 1.0],
        ),
        (
            Adw(AdwEasing::EaseInOutSine),
            [0.0, 0.146_446_62, 0.5, 0.853_553_4, 0.975_528_24, 1.0],
        ),
        (
            Adw(AdwEasing::EaseInExpo),
            [
                0.0,
                0.005_524_271_6,
                0.03125,
                0.176_776_69,
                0.499_999_9,
                1.0,
            ],
        ),
        (
            Adw(AdwEasing::EaseOutExpo),
            [0.0, 0.823_223_3, 0.96875, 0.994_475_7, 0.998_046_9, 1.0],
        ),
    ];
    for (curve, expected) in rows {
        for (t, value) in ts.into_iter().zip(expected) {
            assert_eq!(evaluate(curve, t), Some(value), "{curve:?} at {t}");
        }
    }
    // Qt's out-expo scales by 1.001: it passes 1 just before its end.
    assert_eq!(evaluate(Qt(QtEasing::OutExpo), 0.999), Some(1.000_015_6));
    // The elapsed fraction is clamped; a non-finite one is 0.
    assert_eq!(evaluate(Qt(QtEasing::InQuad), 1.5), Some(1.0));
    assert_eq!(evaluate(Qt(QtEasing::InQuad), f32::NAN), Some(0.0));
    // Kirigami's curve follows the velocity, not the elapsed fraction.
    assert_eq!(evaluate(PlatformCurve::KirigamiVelocity, 0.5), None);
}

#[test]
fn platform_curve_spelling() {
    // S11: `{ platform = { qt = "in_out_sine" } }`.
    let layer = crate::next::Layer::from_toml(
        "[tokens.motion.timings]\nfocus = { platform = { qt = \"in_out_sine\" } }\n\
         spinner = { platform = { adw = \"ease_in_out_sine\" } }\n\
         page = { platform = \"kirigami_velocity\" }",
    )
    .unwrap_or_else(|error| panic!("{error}"));
    let timings = layer.bases.tokens.motion.and_then(|motion| motion.timings);
    assert_eq!(
        timings,
        Some(vec![
            (
                key!("focus"),
                Timing::Platform(PlatformCurve::Qt(QtEasing::InOutSine))
            ),
            (
                key!("spinner"),
                Timing::Platform(PlatformCurve::Adw(AdwEasing::EaseInOutSine))
            ),
            (
                key!("page"),
                Timing::Platform(PlatformCurve::KirigamiVelocity)
            ),
        ])
    );
}

#[test]
fn corner_functions() {
    assert_eq!(full(Px(40.0), Px(20.0), None), Px(10.0));
    assert_eq!(full(Px(40.0), Px(20.0), Some(Px(6.0))), Px(6.0));
    assert_eq!(follow_part(Px(4.0), Px(2.0)), Px(6.0));
    assert_eq!(follow_part(Px(2.0), Px(-4.0)), Px(0.0));
    let even = Axes {
        horizontal: Px(4.0),
        vertical: Px(4.0),
    };
    assert_eq!(concentric(Px(12.0), even, None), Some(Px(8.0)));
    assert_eq!(concentric(Px(6.0), even, Some(Px(3.0))), Some(Px(3.0)));
    assert_eq!(concentric(Px(0.0), even, None), Some(Px(0.0)));
    let uneven = Axes {
        horizontal: Px(4.0),
        vertical: Px(6.0),
    };
    assert_eq!(concentric(Px(12.0), uneven, None), None);
    assert_eq!(concentric(Px(12.0), uneven, Some(Px(3.0))), None);
}

#[test]
fn tags_follow_their_syntax() {
    assert_eq!(
        AxisTag::new("wght").map(|t| t.to_string()),
        Some("wght".to_owned())
    );
    assert_eq!(AxisTag::new("wgh"), None);
    assert_eq!(
        FeatureTag::new("tnum").map(FeatureTag::bytes),
        Some(*b"tnum")
    );
    assert_eq!(FeatureTag::new("tn\u{7f}m"), None);
}

#[test]
fn language_tags_follow_rfc_5646() {
    for (tag, canonical) in [
        ("ar", "ar"),
        ("zh-Hant", "zh-Hant"),
        ("zh-hant", "zh-Hant"),
        ("ZH-HANT-tw", "zh-Hant-TW"),
        ("en-US", "en-US"),
        ("EN-us", "en-US"),
        ("es-419", "es-419"),
        ("zh-yue-HK", "zh-yue-HK"),
        ("ar-afb", "ar-afb"),
        ("abc-def-ghi-jkl", "abc-def-ghi-jkl"),
        ("sl-rozaj-biske", "sl-rozaj-biske"),
        ("de-CH-1901", "de-CH-1901"),
        ("de-Latn-DE-1996", "de-Latn-DE-1996"),
        ("hy-Latn-IT-arevela", "hy-Latn-IT-arevela"),
        ("en-a-bbb-x-a-ccc", "en-a-bbb-x-a-ccc"),
        ("en-US-u-islamcal", "en-US-u-islamcal"),
        ("en-CA-x-CA", "en-CA-x-ca"),
        ("az-latn-X-LATN", "az-Latn-x-latn"),
        ("qaa-Qaaa-QM-x-southern", "qaa-Qaaa-QM-x-southern"),
        ("abcd", "abcd"),
        ("abcde", "abcde"),
        ("x-private", "x-private"),
        ("X-a-12345678", "x-a-12345678"),
        ("i-klingon", "i-klingon"),
        ("I-KLINGON", "i-klingon"),
        ("en-gb-OED", "en-GB-oed"),
        ("zh-min-nan", "zh-min-nan"),
        ("SGN-be-fr", "sgn-BE-FR"),
        ("art-lojban", "art-lojban"),
    ] {
        assert_eq!(
            LanguageTag::new(tag).map(|t| t.to_string()),
            Some(canonical.to_owned()),
            "{tag}"
        );
    }
    assert_eq!(LanguageTag::new("zh-hant"), LanguageTag::new("zh-Hant"));
    for tag in [
        "",
        "a",
        "a-b",
        "en-",
        "-en",
        "en--US",
        "1a",
        "en_US",
        "toolongsubtag",
        "en-x",
        "x",
        "x-123456789",
        "en-a",
        "en-a-x-y",
        "en-a-b",
        "abcd-efg",
        "en-abc-def-ghi-jkl",
        "de-419-DE",
        "en-US-x",
        "en-1234567890",
        "en-\u{e9}",
        "i-unknown",
    ] {
        assert_eq!(LanguageTag::new(tag), None, "{tag}");
    }
}

#[test]
fn a_keyframes_patch_resolves_field_by_field() {
    let _ = resolved_keyframe as fn(ResolvedKeyframe);
    let unset = ResolvedPartStylePatch::default();
    assert_eq!(
        (unset.fill, unset.opacity, unset.padding),
        (None, None, None)
    );
    let set = ResolvedPartStylePatch {
        opacity: Some(Some(Fraction(0.0))),
        fill: Some(None),
        ..ResolvedPartStylePatch::default()
    };
    assert_eq!(set.opacity, Some(Some(Fraction(0.0))));
    assert_eq!(set.fill, Some(None));

    let keyframes = table_of(PartStyle::FIELDS, "icon.animation.keyframes.patch");
    assert!(matches!(
        keyframes.map(|f| f.ty),
        Some(FieldType::Patch {
            ty: "PartStylePatch",
            ..
        })
    ));
    for path in [
        "fill.layers.blend",
        "opacity",
        "padding.left",
        "icon.animation.keyframes.patch.min_width",
    ] {
        let below = format!("icon.animation.keyframes.patch.{path}");
        let found = table_of(PartStyle::FIELDS, &below);
        assert!(found.is_some(), "{below}");
        assert_eq!(found, table_of(PartStyle::FIELDS, path), "{path}");
        assert!(
            crate::next::registry::find(PartStyle::FIELDS, &below).is_some(),
            "{below}"
        );
    }
    assert!(table_of(PartStyle::FIELDS, "icon.animation.keyframes.patch.nothing").is_none());

    let keyframe = Keyframe {
        at: Fraction(0.5),
        part: None,
        timing: Value::Absent,
        patch: PartStylePatch {
            opacity: Some(Value::Stated(Fraction(0.0))),
            ..PartStylePatch::default()
        },
    };
    let animation = Loop {
        period: Value::Stated(Ms(1000.0)),
        timing: Value::Stated(Timing::Linear),
        keyframes: vec![keyframe],
    };
    assert_eq!(animation.keyframes.len(), 1);
    let glyph_stroke = GlyphStroke {
        paint: paint_value(0.5),
        width: Value::Stated(Px(2.0)),
        cap: super::outline::LineCap::Round,
        join: super::outline::LineJoin::Round,
    };
    assert_eq!(glyph_stroke.width, Value::Stated(Px(2.0)));
    let when = FocusVisibleWhen::Keyboard { timeout: None };
    assert_ne!(when, FocusVisibleWhen::Never);
}

/// Holds a resolved keyframe's patch as S6.1 types it: every field of the
/// part style resolved, `None` where the patch leaves it.
fn resolved_keyframe(keyframe: ResolvedKeyframe) {
    let patch: ResolvedPartStylePatch = keyframe.patch;
    let _: Option<Option<super::paint::ResolvedPaint>> = patch.fill;
    let _: Option<Option<Fraction>> = patch.opacity;
    let _: Option<Option<Length>> = patch.min_width;
    let _: Option<Sides<Option<Px>>> = patch.padding;
    let _: Option<Vec<super::part_style::ResolvedBinding>> = patch.bindings;
}
