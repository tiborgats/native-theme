//! Evaluation (S2.2): a typed expression's value, its references answered by
//! an [`Env`].

use super::arith;
use super::ty::{Bound, Node, Ty, TypedExpr};
use super::{Func, Name, Ref};
use crate::next::blocks::paint::{BlendMode, ResolvedPaint, ResolvedPaintKind, ResolvedPaintLayer};
use crate::next::color::Color;
use crate::next::units::{Deg, FontSize, Ms, Px};

/// An evaluated value.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Val {
    /// A colour.
    Color(Color),
    /// A length.
    Length(Px),
    /// A duration.
    Duration(Ms),
    /// An angle.
    Angle(Deg),
    /// A plain number.
    Number(f32),
    /// A font size in logical pixels, converted from points and scaled
    /// (S9.4).
    FontSize(Px),
    /// A text.
    Text(String),
    /// A bool.
    Bool(bool),
    /// An enum variant or a keyed name.
    Name(Name),
    /// A paint: a paint reference's, or a colour expression's on a paint
    /// field, one `Color` layer (S2.2).
    Paint(ResolvedPaint),
    /// A plain reference's `Auto`, on a field that allows it (S2.2).
    Auto,
    /// A plain reference's `Absent`, on a field that may be absent (S2.2).
    Absent,
}

/// What an expression's references read: the theme, the query's element and
/// the scope it is evaluated in.
pub(crate) trait Env {
    /// The resolved value `reference` names (S2.2):
    ///
    /// - a role (`role.<set>.…`, `role.global.…`, `role.@…`, `surface.…`)
    ///   resolved against the nearest binding of that role in scope, else the
    ///   colour roles' value, in S3.3's two steps; `active(…)` the role's value
    ///   from every layer's bases, before any rule, bindings not followed;
    /// - `token.`, `input.`: the token's or input's value;
    /// - `base`: the value the rules before this one gave in this query;
    /// - `scheme`: the scheme of the colours being resolved, a
    ///   [`Name::Variant`];
    /// - `font_size`, `line_height`: the part's, in logical px after the
    ///   point conversion and text scaling, `line_height` the font's line
    ///   spacing under `auto`; `body_font_height`; `text_scale`;
    /// - `element.`: the element's data, the root's inputs and the part's own
    ///   font's metrics;
    /// - `part.`, `widget.<part>.`, `values.`: the same part's resolved field,
    ///   another part's (or its font's metric), the widget's resolved value.
    ///
    /// A value of the reference's type; `Auto` or `Absent` only where a plain
    /// reference brings it to a field that allows it. Inside a call an
    /// operand of another kind reads as zero (a transparent colour), and a
    /// non-finite number as zero, so evaluation stays total; validation
    /// (S12) refuses what could bring either.
    fn get(&self, reference: &Ref) -> Val;

    /// A stated font size in logical pixels: converted from points by the
    /// font DPI, and scaled where the expression's place scales it (S9.4).
    fn font_size(&self, size: FontSize) -> Px;
}

/// A platform function implemented to its source (S2.2), the only kind a
/// typed expression calls: typing refuses the others
/// ([`TypeError::NotImplemented`](super::ty::TypeError::NotImplemented)).
///
/// Each is implemented under `next::platform`, with its vector table, by the
/// task that needs it (the M1a design's task 13 and the value phase); none is
/// yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Platform {}

impl Platform {
    /// The implemented platform function `func` names; `None` for one not
    /// implemented yet and for the model's own arithmetic.
    pub(super) fn of(_func: Func) -> Option<Platform> {
        None
    }

    /// The function's value over its typed arguments.
    fn eval(self, _args: &[Node], _env: &dyn Env) -> Val {
        match self {}
    }
}

impl TypedExpr {
    /// The expression's value where it is used, its references read from
    /// `env`, the result limited as its field's [`Bound`] says.
    ///
    /// Total (S2.2's *Totality*): `alpha` clamps its alpha, `with_alpha` the
    /// alpha it sets, `mix` its
    /// fraction, `pick` takes the floored index's Euclidean remainder, a
    /// non-finite result or operand is zero, and the field's bound zeroes a
    /// negative, clamps a fraction and floors an integer.
    pub(crate) fn eval(&self, env: &dyn Env) -> Val {
        bounded(eval(&self.node, env), self.bound)
    }
}

fn eval(node: &Node, env: &dyn Env) -> Val {
    match node {
        Node::Lit(value) => value.clone(),
        Node::Font(size) => Val::FontSize(Px(arith::finite(env.font_size(*size).0))),
        Node::Ref(reference) => finite(env.get(reference)),
        Node::Binary(op, operands, ty) => {
            let [x, y] = operands.as_ref();
            let (x, y) = (number(&eval(x, env)), number(&eval(y, env)));
            of_type(ty, arith::binary(*op, x, y))
        }
        Node::Floor(x, ty) => of_type(ty, arith::floor(number(&eval(x, env)))),
        Node::Round(x, ty) => of_type(ty, arith::round(number(&eval(x, env)))),
        Node::Alpha(operands) => {
            let [c, a] = operands.as_ref();
            Val::Color(arith::alpha(color(&eval(c, env)), number(&eval(a, env))))
        }
        Node::WithAlpha(operands) => {
            let [c, a] = operands.as_ref();
            Val::Color(arith::with_alpha(
                color(&eval(c, env)),
                number(&eval(a, env)),
            ))
        }
        Node::Mix(operands) => {
            let [a, b, t] = operands.as_ref();
            Val::Color(arith::mix(
                color(&eval(a, env)),
                color(&eval(b, env)),
                number(&eval(t, env)),
            ))
        }
        Node::Pick(index, colors) => {
            let index = number(&eval(index, env));
            let colors: Vec<_> = colors.iter().map(|c| color(&eval(c, env))).collect();
            Val::Color(arith::pick(index, &colors).unwrap_or_else(transparent))
        }
        Node::Platform(func, args) => func.eval(args, env),
        Node::ColorLayer(c) => Val::Paint(color_layer(arith::finite_color(color(&eval(c, env))))),
    }
}

/// A paint of one colour layer drawn normally (S2.2).
fn color_layer(c: Color) -> ResolvedPaint {
    ResolvedPaint {
        layers: vec![ResolvedPaintLayer {
            kind: ResolvedPaintKind::Color(Some(c)),
            blend: BlendMode::Normal,
        }],
    }
}

/// A value of a computed number's type.
fn of_type(ty: &Ty, x: f32) -> Val {
    match ty {
        Ty::Length => Val::Length(Px(x)),
        Ty::Duration => Val::Duration(Ms(x)),
        Ty::FontSize => Val::FontSize(Px(x)),
        _ => Val::Number(x),
    }
}

/// A numeric operand's magnitude in its unit; zero for any other value.
fn number(value: &Val) -> f32 {
    match value {
        Val::Number(x)
        | Val::Length(Px(x))
        | Val::Duration(Ms(x))
        | Val::Angle(Deg(x))
        | Val::FontSize(Px(x)) => *x,
        Val::Color(_)
        | Val::Text(_)
        | Val::Bool(_)
        | Val::Name(_)
        | Val::Paint(_)
        | Val::Auto
        | Val::Absent => 0.0,
    }
}

/// A colour operand; transparent for any other value.
fn color(value: &Val) -> Color {
    match value {
        Val::Color(c) => *c,
        _ => transparent(),
    }
}

fn transparent() -> Color {
    Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    }
}

/// `value` with a non-finite number or colour channel zero.
fn finite(value: Val) -> Val {
    map_number(value, arith::finite)
}

/// `value` limited by `bound` where it is a number.
fn bounded(value: Val, bound: Bound) -> Val {
    match value {
        Val::Color(c) => Val::Color(arith::finite_color(c)),
        other => map_number(other, |x| arith::bound(x, bound)),
    }
}

fn map_number(value: Val, f: impl Fn(f32) -> f32) -> Val {
    match value {
        Val::Number(x) => Val::Number(f(x)),
        Val::Length(Px(x)) => Val::Length(Px(f(x))),
        Val::Duration(Ms(x)) => Val::Duration(Ms(f(x))),
        Val::Angle(Deg(x)) => Val::Angle(Deg(f(x))),
        Val::FontSize(Px(x)) => Val::FontSize(Px(f(x))),
        Val::Color(c) => Val::Color(arith::finite_color(c)),
        other @ (Val::Text(_)
        | Val::Bool(_)
        | Val::Name(_)
        | Val::Paint(_)
        | Val::Auto
        | Val::Absent) => other,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::next::expr::ty::{FieldType, TypeEnv};
    use crate::next::expr::{ElementDatum, Expr};
    use crate::next::key::Key;

    /// Synthetic values for the tests, none a platform's: each reference
    /// answers what `get` matches, the types from `reference`.
    struct Fixture;

    impl TypeEnv for Fixture {
        fn reference(&self, reference: &Ref) -> Option<Ty> {
            match reference.to_string().as_str() {
                "token.gap" | "part.min_width" | "widget.frame.width" => Some(Ty::Length),
                "token.speed" | "token.nan" | "values.weight" => Some(Ty::Number),
                "token.duration" => Some(Ty::Duration),
                "token.label_size" => Some(Ty::FontSize),
                "surface.foreground"
                | "role.global.accent"
                | "input.accent"
                | "role.@shade_light"
                | "role.window.background" => Some(Ty::Color),
                "part.max_width" => Some(Ty::Length),
                _ => None,
            }
        }

        fn declares(&self, _set: &str, _key: &Key) -> bool {
            false
        }
    }

    impl Env for Fixture {
        fn get(&self, reference: &Ref) -> Val {
            let rgba = |r, g, b, a| Val::Color(Color { r, g, b, a });
            match reference {
                Ref::Element(ElementDatum::Width) => Val::Length(Px(f32::INFINITY)),
                Ref::Element(ElementDatum::Height) => Val::Length(Px(40.0)),
                Ref::Element(ElementDatum::ItemIndex) => Val::Number(4.0),
                Ref::Element(ElementDatum::Text) => Val::Text("Synthetic".to_owned()),
                Ref::FontSize => Val::Length(Px(13.0)),
                Ref::Base => Val::Length(Px(6.0)),
                Ref::Active(_) => rgba(0.0, 0.0, 1.0, 1.0),
                _ => match reference.to_string().as_str() {
                    "token.gap" | "widget.frame.width" => Val::Length(Px(6.0)),
                    "token.speed" => Val::Number(0.5),
                    "token.nan" => Val::Number(f32::NAN),
                    "values.weight" => Val::Number(2.5),
                    "token.duration" => Val::Duration(Ms(200.0)),
                    "token.label_size" => Val::FontSize(Px(16.0)),
                    "surface.foreground" => rgba(1.0, 0.0, 0.0, 0.5),
                    "role.global.accent" => rgba(0.0, 0.0, 1.0, 1.0),
                    "part.max_width" => Val::Auto,
                    _ => Val::Absent,
                },
            }
        }

        /// Points at a synthetic two px per point, px as stated.
        fn font_size(&self, size: FontSize) -> Px {
            match size {
                FontSize::Pt(pt) => Px(pt + pt),
                FontSize::Px(px) => Px(px),
            }
        }
    }

    fn eval_as(text: &str, ty: Ty, bound: Bound) -> Val {
        Expr::parse(text)
            .unwrap()
            .check(&FieldType::new(ty, bound), &Fixture)
            .unwrap_or_else(|e| panic!("{text}: {e}"))
            .eval(&Fixture)
    }

    fn value(text: &str, ty: Ty) -> Val {
        eval_as(text, ty, Bound::Unbounded)
    }

    #[test]
    fn evaluates_references_through_env() {
        assert_eq!(value("token.gap", Ty::Length), Val::Length(Px(6.0)));
        assert_eq!(value("base", Ty::Length), Val::Length(Px(6.0)));
        assert_eq!(value("font_size", Ty::Length), Val::Length(Px(13.0)));
        assert_eq!(value("element.height", Ty::Length), Val::Length(Px(40.0)));
        assert_eq!(value("values.weight", Ty::Number), Val::Number(2.5));
        assert_eq!(
            value("element.text", Ty::Text),
            Val::Text("Synthetic".to_owned())
        );
        assert_eq!(
            value("active(role.global.accent)", Ty::Color),
            Val::Color(Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 1.0
            })
        );
        assert_eq!(value("part.max_width", Ty::Length), Val::Auto);
    }

    #[test]
    fn evaluates_arithmetic_in_units() {
        assert_eq!(
            value("add(token.gap, 4)", Ty::Length),
            Val::Length(Px(10.0))
        );
        assert_eq!(
            value("sub(4px, token.gap)", Ty::Length),
            Val::Length(Px(-2.0))
        );
        assert_eq!(
            value("max(token.gap, 8px)", Ty::Length),
            Val::Length(Px(8.0))
        );
        assert_eq!(
            value("min(token.gap, 8px)", Ty::Length),
            Val::Length(Px(6.0))
        );
        assert_eq!(
            value("mul(token.duration, token.speed)", Ty::Duration),
            Val::Duration(Ms(100.0))
        );
        assert_eq!(value("div(token.gap, 4px)", Ty::Number), Val::Number(1.5));
        assert_eq!(value("div(token.gap, 4)", Ty::Length), Val::Length(Px(1.5)));
        assert_eq!(
            value("floor(div(token.gap, 4))", Ty::Length),
            Val::Length(Px(1.0))
        );
        assert_eq!(
            value("round(div(token.gap, 4))", Ty::Length),
            Val::Length(Px(2.0))
        );
        assert_eq!(
            value("alpha(surface.foreground, 0.5)", Ty::Color),
            Val::Color(Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 0.25
            })
        );
        assert_eq!(
            value("with_alpha(surface.foreground, 0.8)", Ty::Color),
            Val::Color(Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 0.8
            })
        );
        assert_eq!(
            value(
                "pick(element.item_index, #ff0000, #00ff00, #0000ff)",
                Ty::Color
            ),
            Val::Color(Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0
            })
        );
        assert_eq!(
            value(
                "mix(surface.foreground, role.global.accent, 0.5)",
                Ty::Paint
            ),
            Val::Paint(color_layer(Color {
                r: 1.0 / 3.0,
                g: 0.0,
                b: 2.0 / 3.0,
                a: 0.75
            }))
        );
    }

    #[test]
    fn font_sizes_go_through_env() {
        assert_eq!(value("10pt", Ty::FontSize), Val::FontSize(Px(20.0)));
        assert_eq!(value("12px", Ty::FontSize), Val::FontSize(Px(12.0)));
        assert_eq!(value("12px", Ty::Length), Val::Length(Px(12.0)));
        assert_eq!(
            value("add(token.label_size, 2)", Ty::FontSize),
            Val::FontSize(Px(18.0))
        );
        assert_eq!(
            value("add(2pt, font_size)", Ty::FontSize),
            Val::FontSize(Px(17.0))
        );
        assert_eq!(
            value("mul(font_size, 1.5)", Ty::FontSize),
            Val::Length(Px(19.5))
        );
    }

    #[test]
    fn non_finite_is_zero() {
        assert_eq!(value("element.width", Ty::Length), Val::Length(Px(0.0)));
        assert_eq!(
            value("add(element.width, 2px)", Ty::Length),
            Val::Length(Px(2.0))
        );
        assert_eq!(value("mul(token.nan, 3)", Ty::Number), Val::Number(0.0));
        assert_eq!(value("div(token.gap, 0)", Ty::Length), Val::Length(Px(0.0)));
        assert_eq!(value("div(1, 0)", Ty::Number), Val::Number(0.0));
    }

    #[test]
    fn fraction_clamped() {
        let fraction = |text| eval_as(text, Ty::Number, Bound::Fraction);
        assert_eq!(fraction("mul(token.speed, 3)"), Val::Number(1.0));
        assert_eq!(fraction("sub(token.speed, 2)"), Val::Number(0.0));
        assert_eq!(fraction("token.speed"), Val::Number(0.5));
    }

    #[test]
    fn bound_limits_the_result() {
        assert_eq!(
            eval_as("sub(2px, token.gap)", Ty::Length, Bound::NonNegative),
            Val::Length(Px(0.0))
        );
        assert_eq!(
            eval_as("sub(2px, token.gap)", Ty::Length, Bound::Unbounded),
            Val::Length(Px(-4.0))
        );
        assert_eq!(
            eval_as(
                "mul(values.weight, 3)",
                Ty::Number,
                Bound::Integer { max: 5 }
            ),
            Val::Number(5.0)
        );
        assert_eq!(
            eval_as(
                "add(values.weight, 1)",
                Ty::Number,
                Bound::Integer { max: 5 }
            ),
            Val::Number(3.0)
        );
        assert_eq!(
            eval_as(
                "sub(1, values.weight)",
                Ty::Number,
                Bound::Integer { max: 5 }
            ),
            Val::Number(0.0)
        );
    }
}
