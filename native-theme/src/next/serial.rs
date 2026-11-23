//! The `serde` forms of the model (spec §9, feature `serde`; spec §7,
//! "Concise deserialization & strict validation").
//!
//! The hand-written shorthands: [`Color`] as its text form, [`Paint`],
//! [`Stroke`], [`Corners`] and [`CornerRadius`] as spec §7 spells them,
//! [`WidgetState`] as a list of state names, [`FontWidth`] as its OS/2 value;
//! the string `"none"` for an absence a patch states (`shadows = "none"`, an
//! unset dimension of `min_size`); and the snapshot form of a resolved theme:
//! a part style as the properties it states, a ruled widget per part as its
//! exclusive state groups, its base's stated properties and its rules in
//! evaluation order, each rule's selector keys spelled as in a
//! `[[widgets.<widget>.rules]]` entry. The snapshot is write-only: nothing
//! reads it back, and a rule that states only some sides of a padding or a
//! margin, or a text's `defined_size`, is not an entry a patch reads.

use std::fmt;
use std::marker::PhantomData;
use std::sync::Arc;

use serde::de::value::{MapAccessDeserializer, SeqAccessDeserializer};
use serde::de::{
    self, DeserializeOwned, Deserializer, IntoDeserializer, MapAccess, SeqAccess, Visitor,
};
use serde::ser::{SerializeMap, SerializeSeq, Serializer};
use serde::{Deserialize, Serialize};

use super::color::Color;
use super::geometry::{
    ComplexStroke, CornerCurve, CornerRadius, Corners, Edges, Stroke, StrokeAlign, StrokeSide,
};
use super::paint::{
    BlendMode, ComplexPaint, GradientStop, MaterialKind, MaterialSpec, Paint, PaintLayer,
};
use super::preset::Preset;
use super::query::{ButtonShape, WidgetState};
use super::rules::{ButtonMatcher, ListMatcher, Patched, RuleSelector, RuleSet};
use super::style::{
    FontWidth, IconStated, IconStyle, PartStated, PartStyle, TextStated, TextStyle,
};
use super::widgets::{
    ButtonWidget, CardWidget, CheckboxWidget, ComboBoxWidget, ExpanderWidget, InputWidget,
    LinkWidget, ListWidget, MenuBarWidget, MenuWidget, ProgressBarWidget, RadioWidget,
    ScrollbarWidget, SegmentedControlWidget, SidebarWidget, SliderWidget, SplitterWidget,
    SwitchWidget, TabWidget, ToolbarWidget,
};

/// The spelling of a stated absence.
const NONE: &str = "none";

/// Whether `value` is its type's default: a patch's empty sections are left
/// out of its serialized form.
pub(crate) fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}

/// A patch field that states nothing, which its serialized form leaves out.
pub(crate) trait Skip {
    /// Whether the field states nothing.
    fn skip(&self) -> bool;
}

impl<T> Skip for Option<T> {
    fn skip(&self) -> bool {
        self.is_none()
    }
}

impl<T> Skip for Vec<T> {
    fn skip(&self) -> bool {
        self.is_empty()
    }
}

/// An optional value written as itself, or `"none"` where it is `None`.
struct NoneOr<'a, T>(&'a Option<T>);

impl<T: Serialize> Serialize for NoneOr<'_, T> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Some(value) => value.serialize(s),
            None => s.serialize_str(NONE),
        }
    }
}

/// An optional value read from itself, or `None` from `"none"`.
struct OrNone<T>(Option<T>);

impl<'de, T: Deserialize<'de>> Deserialize<'de> for OrNone<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V<T>(PhantomData<T>);

        impl<'de, T: Deserialize<'de>> Visitor<'de> for V<T> {
            type Value = OrNone<T>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("\"none\" or a value")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                if v == NONE {
                    Ok(OrNone(None))
                } else {
                    T::deserialize(v.into_deserializer()).map(|t| OrNone(Some(t)))
                }
            }

            fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
                T::deserialize(v.into_deserializer()).map(|t| OrNone(Some(t)))
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                T::deserialize(v.into_deserializer()).map(|t| OrNone(Some(t)))
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                T::deserialize(v.into_deserializer()).map(|t| OrNone(Some(t)))
            }

            fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
                T::deserialize(v.into_deserializer()).map(|t| OrNone(Some(t)))
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                T::deserialize(MapAccessDeserializer::new(map)).map(|t| OrNone(Some(t)))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
                T::deserialize(SeqAccessDeserializer::new(seq)).map(|t| OrNone(Some(t)))
            }
        }

        d.deserialize_any(V(PhantomData))
    }
}

/// A patch's `Option<Option<T>>` field: the outer `None` is a key left out,
/// `Some(None)` the string `"none"`, `Some(Some(value))` the value.
pub(crate) mod nullable {
    use super::{Deserialize, Deserializer, NoneOr, OrNone, Serialize, Serializer};

    pub(crate) fn serialize<S: Serializer, T: Serialize>(
        value: &Option<Option<T>>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(inner) => NoneOr(inner).serialize(s),
            None => s.serialize_none(),
        }
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
        d: D,
    ) -> Result<Option<Option<T>>, D::Error> {
        Ok(Some(OrNone::<T>::deserialize(d)?.0))
    }
}

/// A `[width, height]` pair of optional dimensions, as `[w, h]` with
/// `"none"` for a dimension left unset.
struct Dims<'a>(&'a [Option<f32>; 2]);

impl Serialize for Dims<'_> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let [width, height] = self.0;
        let mut seq = s.serialize_seq(Some(2))?;
        seq.serialize_element(&NoneOr(width))?;
        seq.serialize_element(&NoneOr(height))?;
        seq.end()
    }
}

/// A patch's `Option<[Option<f32>; 2]>` size field, each dimension as
/// [`Dims`] writes it.
pub(crate) mod dims {
    use super::{Deserialize, Deserializer, Dims, OrNone, Serialize, Serializer};

    pub(crate) fn serialize<S: Serializer>(
        value: &Option<[Option<f32>; 2]>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(size) => Dims(size).serialize(s),
            None => s.serialize_none(),
        }
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
        d: D,
    ) -> Result<Option<[Option<f32>; 2]>, D::Error> {
        let [width, height] = <[OrNone<f32>; 2]>::deserialize(d)?;
        Ok(Some([width.0, height.0]))
    }
}

/// A number read as an `f32` from any numeric form (TOML writes `4` and
/// `4.0` alike).
macro_rules! visit_numbers {
    ($value:ty, $make:expr) => {
        fn visit_f64<E: de::Error>(self, v: f64) -> Result<$value, E> {
            Ok($make(v as f32))
        }

        fn visit_i64<E: de::Error>(self, v: i64) -> Result<$value, E> {
            Ok($make(v as f32))
        }

        fn visit_u64<E: de::Error>(self, v: u64) -> Result<$value, E> {
            Ok($make(v as f32))
        }
    };
}

// ---------------------------------------------------------------------------
// Color
// ---------------------------------------------------------------------------

/// Its text form ([`Color`]'s `Display`, lossless).
impl Serialize for Color {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(self)
    }
}

/// Any of its text forms ([`Color`]'s `FromStr`).
impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V;

        impl Visitor<'_> for V {
            type Value = Color;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a colour: #rgb, #rgba, #rrggbb, #rrggbbaa, srgb(...) or p3(...)")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Color, E> {
                v.parse().map_err(E::custom)
            }
        }

        d.deserialize_str(V)
    }
}

// ---------------------------------------------------------------------------
// Paint
// ---------------------------------------------------------------------------

/// A paint layer as a tagged map: `kind` names the variant, `blend` may be
/// left out (`Normal`), and a material layer's own kind is `material`.
#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum LayerRepr {
    Solid {
        color: Color,
        #[serde(default, skip_serializing_if = "is_default")]
        blend: BlendMode,
    },
    LinearGradient {
        angle_deg: f32,
        stops: Arc<[GradientStop]>,
        #[serde(default, skip_serializing_if = "is_default")]
        blend: BlendMode,
    },
    RadialGradient {
        center: [f32; 2],
        radius: f32,
        stops: Arc<[GradientStop]>,
        #[serde(default, skip_serializing_if = "is_default")]
        blend: BlendMode,
    },
    Material {
        material: MaterialKind,
        blur_radius: f32,
        tint: Color,
        saturation: f32,
        luminosity_opacity: f32,
        noise_opacity: f32,
        fallback: Color,
    },
}

impl From<&PaintLayer> for LayerRepr {
    fn from(layer: &PaintLayer) -> Self {
        match layer {
            PaintLayer::Solid { color, blend } => Self::Solid {
                color: *color,
                blend: *blend,
            },
            PaintLayer::LinearGradient {
                angle_deg,
                stops,
                blend,
            } => Self::LinearGradient {
                angle_deg: *angle_deg,
                stops: stops.clone(),
                blend: *blend,
            },
            PaintLayer::RadialGradient {
                center,
                radius,
                stops,
                blend,
            } => Self::RadialGradient {
                center: *center,
                radius: *radius,
                stops: stops.clone(),
                blend: *blend,
            },
            PaintLayer::Material(m) => Self::Material {
                material: m.kind,
                blur_radius: m.blur_radius,
                tint: m.tint,
                saturation: m.saturation,
                luminosity_opacity: m.luminosity_opacity,
                noise_opacity: m.noise_opacity,
                fallback: m.fallback,
            },
        }
    }
}

impl From<LayerRepr> for PaintLayer {
    fn from(layer: LayerRepr) -> Self {
        match layer {
            LayerRepr::Solid { color, blend } => Self::Solid { color, blend },
            LayerRepr::LinearGradient {
                angle_deg,
                stops,
                blend,
            } => Self::LinearGradient {
                angle_deg,
                stops,
                blend,
            },
            LayerRepr::RadialGradient {
                center,
                radius,
                stops,
                blend,
            } => Self::RadialGradient {
                center,
                radius,
                stops,
                blend,
            },
            LayerRepr::Material {
                material,
                blur_radius,
                tint,
                saturation,
                luminosity_opacity,
                noise_opacity,
                fallback,
            } => Self::Material(MaterialSpec {
                kind: material,
                blur_radius,
                tint,
                saturation,
                luminosity_opacity,
                noise_opacity,
                fallback,
            }),
        }
    }
}

impl Serialize for PaintLayer {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        LayerRepr::from(self).serialize(s)
    }
}

impl<'de> Deserialize<'de> for PaintLayer {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        LayerRepr::deserialize(d).map(Self::from)
    }
}

/// `"none"`, a colour (solid), one layer's map or an array of layers.
impl Serialize for Paint {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Paint::None => s.serialize_str(NONE),
            Paint::Solid(color) => color.serialize(s),
            Paint::Complex(complex) => match &*complex.layers {
                [layer] => layer.serialize(s),
                layers => layers.serialize(s),
            },
        }
    }
}

impl<'de> Deserialize<'de> for Paint {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = Paint;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("\"none\", a colour, a paint layer or an array of paint layers")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Paint, E> {
                if v == NONE {
                    Ok(Paint::None)
                } else {
                    v.parse().map(Paint::Solid).map_err(E::custom)
                }
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Paint, A::Error> {
                let layer = PaintLayer::deserialize(MapAccessDeserializer::new(map))?;
                Ok(complex(Arc::from([layer])))
            }

            fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Paint, A::Error> {
                let layers = Vec::<PaintLayer>::deserialize(SeqAccessDeserializer::new(seq))?;
                Ok(complex(Arc::from(layers)))
            }
        }

        d.deserialize_any(V)
    }
}

/// The paint of `layers`.
fn complex(layers: Arc<[PaintLayer]>) -> Paint {
    Paint::Complex(Arc::new(ComplexPaint { layers }))
}

// ---------------------------------------------------------------------------
// Stroke
// ---------------------------------------------------------------------------

/// A stroke's map: `{ width, color, align? }` or four sides
/// `{ top, right, bottom, left, align? }`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct StrokeRepr {
    width: Option<f32>,
    color: Option<Color>,
    top: Option<StrokeSide>,
    right: Option<StrokeSide>,
    bottom: Option<StrokeSide>,
    left: Option<StrokeSide>,
    #[serde(default)]
    align: StrokeAlign,
}

impl Serialize for Stroke {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let (mut map, align) = match self {
            Stroke::None => return s.serialize_str(NONE),
            Stroke::Uniform {
                width,
                color,
                align,
            } => {
                let mut map = s.serialize_map(None)?;
                map.serialize_entry("width", width)?;
                map.serialize_entry("color", color)?;
                (map, align)
            }
            Stroke::Complex(sides) => {
                let mut map = s.serialize_map(None)?;
                map.serialize_entry("top", &sides.top)?;
                map.serialize_entry("right", &sides.right)?;
                map.serialize_entry("bottom", &sides.bottom)?;
                map.serialize_entry("left", &sides.left)?;
                (map, &sides.align)
            }
        };
        if !is_default(align) {
            map.serialize_entry("align", align)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Stroke {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = Stroke;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(
                    "\"none\", { width, color, align? } or { top, right, bottom, left, align? }",
                )
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Stroke, E> {
                if v == NONE {
                    Ok(Stroke::None)
                } else {
                    Err(E::invalid_value(de::Unexpected::Str(v), &self))
                }
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Stroke, A::Error> {
                let repr = StrokeRepr::deserialize(MapAccessDeserializer::new(map))?;
                match repr {
                    StrokeRepr {
                        width: Some(width),
                        color: Some(color),
                        top: None,
                        right: None,
                        bottom: None,
                        left: None,
                        align,
                    } => Ok(Stroke::Uniform {
                        width,
                        color,
                        align,
                    }),
                    StrokeRepr {
                        width: None,
                        color: None,
                        top: Some(top),
                        right: Some(right),
                        bottom: Some(bottom),
                        left: Some(left),
                        align,
                    } => Ok(Stroke::Complex(Arc::new(ComplexStroke {
                        top,
                        right,
                        bottom,
                        left,
                        align,
                    }))),
                    _ => Err(de::Error::custom(
                        "a stroke states `width` and `color`, or all four of `top`, `right`, `bottom` and `left`",
                    )),
                }
            }
        }

        d.deserialize_any(V)
    }
}

// ---------------------------------------------------------------------------
// Corners
// ---------------------------------------------------------------------------

/// A radius's map: four corners or `{ inset, min_radius }`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RadiusRepr {
    top_left: Option<f32>,
    top_right: Option<f32>,
    bottom_right: Option<f32>,
    bottom_left: Option<f32>,
    inset: Option<f32>,
    min_radius: Option<f32>,
}

/// `"none"`, `"pill"`, a uniform number, the four corners
/// `{ top_left, top_right, bottom_right, bottom_left }` or
/// `{ inset, min_radius }` (concentric).
impl Serialize for CornerRadius {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            CornerRadius::Uniform(r) => s.serialize_f32(*r),
            CornerRadius::PerCorner {
                top_left,
                top_right,
                bottom_right,
                bottom_left,
            } => {
                let mut map = s.serialize_map(Some(4))?;
                map.serialize_entry("top_left", top_left)?;
                map.serialize_entry("top_right", top_right)?;
                map.serialize_entry("bottom_right", bottom_right)?;
                map.serialize_entry("bottom_left", bottom_left)?;
                map.end()
            }
            CornerRadius::Pill => s.serialize_str("pill"),
            CornerRadius::Concentric { inset, min_radius } => {
                let mut map = s.serialize_map(Some(2))?;
                map.serialize_entry("inset", inset)?;
                map.serialize_entry("min_radius", min_radius)?;
                map.end()
            }
            _ => s.serialize_str(NONE),
        }
    }
}

/// The visitor of a [`CornerRadius`].
struct RadiusVisitor;

impl<'de> Visitor<'de> for RadiusVisitor {
    type Value = CornerRadius;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\"none\", \"pill\", a number, four corners or { inset, min_radius }")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<CornerRadius, E> {
        match v {
            NONE => Ok(CornerRadius::None),
            "pill" => Ok(CornerRadius::Pill),
            _ => Err(E::invalid_value(de::Unexpected::Str(v), &self)),
        }
    }

    visit_numbers!(CornerRadius, CornerRadius::Uniform);

    fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<CornerRadius, A::Error> {
        match RadiusRepr::deserialize(MapAccessDeserializer::new(map))? {
            RadiusRepr {
                top_left: Some(top_left),
                top_right: Some(top_right),
                bottom_right: Some(bottom_right),
                bottom_left: Some(bottom_left),
                inset: None,
                min_radius: None,
            } => Ok(CornerRadius::PerCorner {
                top_left,
                top_right,
                bottom_right,
                bottom_left,
            }),
            RadiusRepr {
                top_left: None,
                top_right: None,
                bottom_right: None,
                bottom_left: None,
                inset: Some(inset),
                min_radius: Some(min_radius),
            } => Ok(CornerRadius::Concentric { inset, min_radius }),
            _ => Err(de::Error::custom(
                "a radius map states all four corners, or `inset` and `min_radius`",
            )),
        }
    }
}

impl<'de> Deserialize<'de> for CornerRadius {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(RadiusVisitor)
    }
}

/// The map form of [`Corners`].
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CornersRepr {
    radius: CornerRadius,
    #[serde(default)]
    curve: CornerCurve,
}

/// `"none"`, `"pill"` or a uniform number where the curve is circular, else
/// `{ radius, curve? }`.
impl Serialize for Corners {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let short = matches!(
            self.radius,
            CornerRadius::None | CornerRadius::Pill | CornerRadius::Uniform(_)
        );
        if short && is_default(&self.curve) {
            return self.radius.serialize(s);
        }
        let mut map = s.serialize_map(None)?;
        map.serialize_entry("radius", &self.radius)?;
        if !is_default(&self.curve) {
            map.serialize_entry("curve", &self.curve)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Corners {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V;

        impl<'de> Visitor<'de> for V {
            type Value = Corners;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("\"none\", \"pill\", a number or { radius, curve? }")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Corners, E> {
                RadiusVisitor.visit_str(v).map(circular)
            }

            visit_numbers!(Corners, |r| circular(CornerRadius::Uniform(r)));

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Corners, A::Error> {
                let repr = CornersRepr::deserialize(MapAccessDeserializer::new(map))?;
                Ok(Corners {
                    radius: repr.radius,
                    curve: repr.curve,
                })
            }
        }

        d.deserialize_any(V)
    }
}

/// Corners of `radius` with the default (circular) curve.
fn circular(radius: CornerRadius) -> Corners {
    Corners {
        radius,
        curve: CornerCurve::default(),
    }
}

// ---------------------------------------------------------------------------
// WidgetState, FontWidth, Preset
// ---------------------------------------------------------------------------

/// Each state flag and its name, in bit order.
const STATE_NAMES: [(WidgetState, &str); 19] = [
    (WidgetState::HOVER, "hover"),
    (WidgetState::PRESSED, "pressed"),
    (WidgetState::FOCUSED, "focused"),
    (WidgetState::FOCUS_WITHIN, "focus_within"),
    (WidgetState::FOCUS_VISIBLE, "focus_visible"),
    (WidgetState::UNFOCUSED_VIEW, "unfocused_view"),
    (WidgetState::DISABLED, "disabled"),
    (WidgetState::CHECKED, "checked"),
    (WidgetState::SELECTED, "selected"),
    (WidgetState::INDETERMINATE, "indeterminate"),
    (WidgetState::OPEN, "open"),
    (WidgetState::DEFAULT, "default"),
    (WidgetState::INVALID, "invalid"),
    (WidgetState::READ_ONLY, "read_only"),
    (WidgetState::VISITED, "visited"),
    (WidgetState::DRAGGED, "dragged"),
    (WidgetState::DROP_TARGET, "drop_target"),
    (WidgetState::SCROLLED, "scrolled"),
    (WidgetState::HOVER_WITHIN, "hover_within"),
];

/// The names of the flags set, in bit order (`["hover", "focused"]`).
impl Serialize for WidgetState {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_seq(
            STATE_NAMES
                .iter()
                .filter(|(flag, _)| self.contains(*flag))
                .map(|(_, name)| *name),
        )
    }
}

impl<'de> Deserialize<'de> for WidgetState {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let names = Vec::<String>::deserialize(d)?;
        names.iter().try_fold(WidgetState::NONE, |state, name| {
            STATE_NAMES
                .iter()
                .find(|(_, known)| known == name)
                .map(|(flag, _)| state | *flag)
                .ok_or_else(|| de::Error::custom(format_args!("unknown state `{name}`")))
        })
    }
}

/// One flag of an exclusive state group: its name, or the list of names
/// where it holds several flags.
struct StateFlag(WidgetState);

impl Serialize for StateFlag {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match STATE_NAMES.iter().find(|(flag, _)| *flag == self.0) {
            Some((_, name)) => s.serialize_str(name),
            None => self.0.serialize(s),
        }
    }
}

/// The OS/2 width class, `1..=9`.
impl Serialize for FontWidth {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u16(self.value())
    }
}

/// The OS/2 width class; a value outside `1..=9` is an error.
impl<'de> Deserialize<'de> for FontWidth {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let width = u16::deserialize(d)?;
        let class = FontWidth::new(width);
        if class.value() == width {
            Ok(class)
        } else {
            Err(de::Error::custom(format_args!(
                "font width {width} outside 1..=9"
            )))
        }
    }
}

/// The machine-readable name ([`Preset::as_str`]).
impl Serialize for Preset {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}

// ---------------------------------------------------------------------------
// Rule selectors
// ---------------------------------------------------------------------------

/// The fields of a rule's variant matcher, as a rule's table spells them.
pub(crate) trait SelectorFields {
    /// Writes the fields that limit the rule.
    fn write<S: SerializeMap>(&self, map: &mut S) -> Result<(), S::Error>;

    /// Reads the value of `key` where it is one of the matcher's fields;
    /// `false` where it is not.
    fn read<'de, A: MapAccess<'de>>(&mut self, key: &str, map: &mut A) -> Result<bool, A::Error>;
}

impl SelectorFields for () {
    fn write<S: SerializeMap>(&self, _map: &mut S) -> Result<(), S::Error> {
        Ok(())
    }

    fn read<'de, A: MapAccess<'de>>(&mut self, _key: &str, _map: &mut A) -> Result<bool, A::Error> {
        Ok(false)
    }
}

/// `variant`.
impl<V: Serialize + DeserializeOwned> SelectorFields for Option<V> {
    fn write<S: SerializeMap>(&self, map: &mut S) -> Result<(), S::Error> {
        match self {
            Some(variant) => map.serialize_entry("variant", variant),
            None => Ok(()),
        }
    }

    fn read<'de, A: MapAccess<'de>>(&mut self, key: &str, map: &mut A) -> Result<bool, A::Error> {
        if key == "variant" {
            *self = Some(map.next_value()?);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// `variant` and `shape`.
impl SelectorFields for ButtonMatcher {
    fn write<S: SerializeMap>(&self, map: &mut S) -> Result<(), S::Error> {
        self.variant.write(map)?;
        if let Some(shape) = &self.shape {
            map.serialize_entry("shape", shape)?;
        }
        Ok(())
    }

    fn read<'de, A: MapAccess<'de>>(&mut self, key: &str, map: &mut A) -> Result<bool, A::Error> {
        if key == "shape" {
            self.shape = Some(map.next_value::<ButtonShape>()?);
            return Ok(true);
        }
        self.variant.read(key, map)
    }
}

/// `variant` and `alternate`.
impl SelectorFields for ListMatcher {
    fn write<S: SerializeMap>(&self, map: &mut S) -> Result<(), S::Error> {
        self.variant.write(map)?;
        if let Some(alternate) = &self.alternate {
            map.serialize_entry("alternate", alternate)?;
        }
        Ok(())
    }

    fn read<'de, A: MapAccess<'de>>(&mut self, key: &str, map: &mut A) -> Result<bool, A::Error> {
        if key == "alternate" {
            self.alternate = Some(map.next_value()?);
            return Ok(true);
        }
        self.variant.read(key, map)
    }
}

/// Writes `selector`'s fields that limit the rule: `states`, `not_states`,
/// the matcher's, `size`, `surface`, `position`, `orientation`.
pub(crate) fn write_selector<S: SerializeMap, M: SelectorFields>(
    map: &mut S,
    selector: &RuleSelector<M>,
) -> Result<(), S::Error> {
    if !selector.required_states.is_empty() {
        map.serialize_entry("states", &selector.required_states)?;
    }
    if !selector.forbidden_states.is_empty() {
        map.serialize_entry("not_states", &selector.forbidden_states)?;
    }
    selector.matcher.write(map)?;
    if let Some(size) = &selector.size {
        map.serialize_entry("size", size)?;
    }
    if let Some(surface) = &selector.surface {
        map.serialize_entry("surface", surface)?;
    }
    if let Some(position) = &selector.position {
        map.serialize_entry("position", position)?;
    }
    if let Some(orientation) = &selector.orientation {
        map.serialize_entry("orientation", orientation)?;
    }
    Ok(())
}

/// Reads the value of `key` into `selector` where it is one of a
/// selector's fields; `false` where it is not.
pub(crate) fn read_selector<'de, A: MapAccess<'de>, M: SelectorFields>(
    selector: &mut RuleSelector<M>,
    key: &str,
    map: &mut A,
) -> Result<bool, A::Error> {
    match key {
        "states" => selector.required_states = map.next_value()?,
        "not_states" => selector.forbidden_states = map.next_value()?,
        "size" => selector.size = Some(map.next_value()?),
        "surface" => selector.surface = Some(map.next_value()?),
        "position" => selector.position = Some(map.next_value()?),
        "orientation" => selector.orientation = Some(map.next_value()?),
        _ => return selector.matcher.read(key, map),
    }
    Ok(true)
}

// ---------------------------------------------------------------------------
// The snapshot form: part styles and rule sets
// ---------------------------------------------------------------------------

/// The sides of `edges` whose bit is set, as a table of those sides.
struct Sides<'a> {
    edges: &'a Edges,
    stated: [bool; 4],
}

impl Sides<'_> {
    fn any(&self) -> bool {
        self.stated.iter().any(|stated| *stated)
    }
}

impl Serialize for Sides<'_> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let [top, right, bottom, left] = self.stated;
        let mut map = s.serialize_map(None)?;
        for (stated, key, value) in [
            (top, "top", &self.edges.top),
            (right, "right", &self.edges.right),
            (bottom, "bottom", &self.edges.bottom),
            (left, "left", &self.edges.left),
        ] {
            if stated {
                map.serialize_entry(key, value)?;
            }
        }
        map.end()
    }
}

/// The properties the style states, as a part patch spells them; a padding
/// or margin of which only some sides are stated is a table of those sides.
impl Serialize for PartStyle {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let has = |bit: PartStated| self.stated.contains(bit);
        let padding = Sides {
            edges: &self.padding,
            stated: [
                has(PartStated::PADDING_TOP),
                has(PartStated::PADDING_RIGHT),
                has(PartStated::PADDING_BOTTOM),
                has(PartStated::PADDING_LEFT),
            ],
        };
        let margin = Sides {
            edges: &self.margin,
            stated: [
                has(PartStated::MARGIN_TOP),
                has(PartStated::MARGIN_RIGHT),
                has(PartStated::MARGIN_BOTTOM),
                has(PartStated::MARGIN_LEFT),
            ],
        };
        let mut map = s.serialize_map(None)?;
        if has(PartStated::FILL) {
            map.serialize_entry("fill", &self.fill)?;
        }
        if has(PartStated::STROKE) {
            map.serialize_entry("stroke", &self.stroke)?;
        }
        if has(PartStated::CORNERS) {
            map.serialize_entry("corners", &self.corners)?;
        }
        if has(PartStated::SHADOWS) {
            map.serialize_entry("shadows", &NoneOr(&self.shadows))?;
        }
        if has(PartStated::OPACITY) {
            map.serialize_entry("opacity", &self.opacity)?;
        }
        if has(PartStated::TRANSFORM) {
            map.serialize_entry("transform", &self.transform)?;
        }
        if padding.any() {
            map.serialize_entry("padding", &padding)?;
        }
        if margin.any() {
            map.serialize_entry("margin", &margin)?;
        }
        if self.min_size.iter().any(Option::is_some) {
            map.serialize_entry("min_size", &Dims(&self.min_size))?;
        }
        if self.max_size.iter().any(Option::is_some) {
            map.serialize_entry("max_size", &Dims(&self.max_size))?;
        }
        if has(PartStated::FOCUS) {
            map.serialize_entry("focus", &NoneOr(&self.focus))?;
        }
        if has(PartStated::RIPPLE) {
            map.serialize_entry("ripple", &NoneOr(&self.ripple))?;
        }
        if has(PartStated::TRANSITION) {
            map.serialize_entry("transition", &NoneOr(&self.transition))?;
        }
        if has(PartStated::CURSOR) {
            map.serialize_entry("cursor", &self.cursor)?;
        }
        map.end()
    }
}

/// The properties the style states, as a text patch spells them, then its
/// source size (`defined_size`) where it has one.
impl Serialize for TextStyle {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let has = |bit: TextStated| self.stated.contains(bit);
        let mut map = s.serialize_map(None)?;
        if has(TextStated::FAMILY) {
            map.serialize_entry("family", &self.family)?;
        }
        if has(TextStated::COLOR) {
            map.serialize_entry("color", &self.color)?;
        }
        if has(TextStated::SIZE_PX) {
            map.serialize_entry("size_px", &self.size_px)?;
        }
        if has(TextStated::WEIGHT) {
            map.serialize_entry("weight", &self.weight)?;
        }
        if has(TextStated::STYLE) {
            map.serialize_entry("style", &self.style)?;
        }
        if has(TextStated::WIDTH) {
            map.serialize_entry("width", &self.width)?;
        }
        if has(TextStated::LINE_HEIGHT) {
            map.serialize_entry("line_height", &self.line_height)?;
        }
        if has(TextStated::LETTER_SPACING) {
            map.serialize_entry("letter_spacing", &self.letter_spacing)?;
        }
        if has(TextStated::OPTICAL_SIZE) {
            map.serialize_entry("optical_size", &NoneOr(&self.optical_size))?;
        }
        if has(TextStated::DECORATION) {
            map.serialize_entry("decoration", &self.decoration)?;
        }
        if has(TextStated::BLEND_MODE) {
            map.serialize_entry("blend_mode", &self.blend_mode)?;
        }
        if let Some(size) = &self.defined_size {
            map.serialize_entry("defined_size", size)?;
        }
        map.end()
    }
}

/// The properties the style states, as an icon patch spells them.
impl Serialize for IconStyle {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let has = |bit: IconStated| self.stated.contains(bit);
        let mut map = s.serialize_map(None)?;
        if has(IconStated::SIZE) {
            map.serialize_entry("size", &self.size)?;
        }
        if has(IconStated::COLOR) {
            map.serialize_entry("color", &self.color)?;
        }
        if has(IconStated::OPACITY) {
            map.serialize_entry("opacity", &self.opacity)?;
        }
        if has(IconStated::WEIGHT) {
            map.serialize_entry("weight", &self.weight)?;
        }
        if has(IconStated::RENDERING_MODE) {
            map.serialize_entry("rendering_mode", &self.rendering_mode)?;
        }
        if has(IconStated::STROKE_WIDTH) {
            map.serialize_entry("stroke_width", &NoneOr(&self.stroke_width))?;
        }
        if has(IconStated::BLEND_MODE) {
            map.serialize_entry("blend_mode", &self.blend_mode)?;
        }
        map.end()
    }
}

/// The rule set of the part `part`: `exclusive` (its ordered exclusive state
/// groups, where it has any), `base` (the properties its base states) and
/// `rules` (in evaluation order, where it has any), each rule its selector's
/// fields and `<part>` the properties it applies.
pub(crate) struct RulesView<'a, T, P, M> {
    part: &'static str,
    rules: &'a RuleSet<T, P, M>,
}

impl<'a, T, P, M> RulesView<'a, T, P, M> {
    pub(crate) fn new(part: &'static str, rules: &'a RuleSet<T, P, M>) -> Self {
        Self { part, rules }
    }
}

impl<T, P, M> Serialize for RulesView<'_, T, P, M>
where
    T: Serialize + Default + Patched<P>,
    M: SelectorFields,
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let exclusive = self.rules.exclusive();
        let mut map = s.serialize_map(None)?;
        if !exclusive.is_empty() {
            map.serialize_entry("exclusive", &Exclusive(exclusive))?;
        }
        map.serialize_entry("base", self.rules.base())?;
        if self.rules.entries().next().is_some() {
            map.serialize_entry(
                "rules",
                &RuleList {
                    part: self.part,
                    rules: self.rules,
                },
            )?;
        }
        map.end()
    }
}

/// The exclusive state groups, each a list of its flags.
struct Exclusive(&'static [&'static [WidgetState]]);

impl Serialize for Exclusive {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.collect_seq(self.0.iter().map(|group| {
            group
                .iter()
                .map(|flag| StateFlag(*flag))
                .collect::<Vec<_>>()
        }))
    }
}

/// The rules of a rule set, in evaluation order.
struct RuleList<'a, T, P, M> {
    part: &'static str,
    rules: &'a RuleSet<T, P, M>,
}

impl<T, P, M> Serialize for RuleList<'_, T, P, M>
where
    T: Serialize + Default + Patched<P>,
    M: SelectorFields,
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(None)?;
        for (selector, props) in self.rules.entries() {
            seq.serialize_element(&RuleView::<T, P, M> {
                part: self.part,
                selector,
                props,
                style: PhantomData,
            })?;
        }
        seq.end()
    }
}

/// One rule: its selector's fields, then `<part>` the properties it applies,
/// as a style that states exactly those.
struct RuleView<'a, T, P, M> {
    part: &'static str,
    selector: &'a RuleSelector<M>,
    props: &'a [P],
    style: PhantomData<T>,
}

impl<T, P, M> Serialize for RuleView<'_, T, P, M>
where
    T: Serialize + Default + Patched<P>,
    M: SelectorFields,
{
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut applied = T::default();
        for prop in self.props {
            applied.apply(prop);
        }
        let mut map = s.serialize_map(None)?;
        write_selector(&mut map, self.selector)?;
        map.serialize_entry(self.part, &applied)?;
        map.end()
    }
}

/// `Serialize` on a ruled widget: its fields in order, then each part's rule
/// set as [`RulesView`] writes it.
macro_rules! ruled_widget {
    ($( $Widget:ident { $($field:ident),* ; $($part:ident),* $(,)? } )*) => {
        $(
            impl Serialize for $Widget {
                fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                    let mut map = s.serialize_map(None)?;
                    $( map.serialize_entry(stringify!($field), &self.$field)?; )*
                    $( map.serialize_entry(
                        stringify!($part),
                        &RulesView::new(stringify!($part), &self.$part),
                    )?; )*
                    map.end()
                }
            }
        )*
    };
}

ruled_widget! {
    ButtonWidget { origin, icon_text_gap, disabled_opacity; body, label, icon }
    InputWidget { origin, disabled_opacity; field, text, placeholder, helper_text }
    CheckboxWidget { origin, label_gap, disabled_opacity; indicator, checkmark, label }
    RadioWidget { origin, label_gap, disabled_opacity; indicator, dot, label }
    SwitchWidget {
        origin, label_side, label_gap, label_gap_stretches, show_state_glyphs, disabled_opacity;
        track, thumb, label
    }
    SliderWidget { origin, tick_length, disabled_opacity; track, active_track, thumb, tick }
    ProgressBarWidget { origin, indeterminate_period_ms; track, fill }
    ScrollbarWidget { origin, min_thumb_length, overlay_mode; track, thumb }
    TabWidget {
        origin, item_gap, indicator_side, active_outward_corner_radius;
        bar, tab, label, indicator, close_button, close_icon
    }
    MenuWidget {
        origin, icon_text_gap;
        panel, item, item_label, shortcut, icon, submenu_arrow, separator
    }
    MenuBarWidget { origin, item_gap; bar, item, item_label }
    ListWidget {
        origin, grid_color;
        container, row, highlight, item_label, header, header_label, header_separator
    }
    SidebarWidget { origin, item_gap; container, item, item_label, item_icon, section_header }
    ToolbarWidget { origin, item_gap; bar, button, button_label, button_icon, separator }
    ComboBoxWidget { origin, arrow_area_width, disabled_opacity; field, label, chevron }
    SegmentedControlWidget { origin, separator_width, disabled_opacity; track, segment, label, separator }
    ExpanderWidget {
        origin, arrow_side, arrow_gap, content_indent, frame_enabled;
        container, header, header_label, arrow
    }
    CardWidget { origin; container }
    SplitterWidget { origin, divider_width; handle }
    LinkWidget { origin; container, label }
}

#[cfg(test)]
mod tests;
