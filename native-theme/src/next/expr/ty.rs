//! Typing (S2.2): every operand and result is typed, and an expression is
//! checked against the field it is stated on when its layer is added.

use std::borrow::Cow;
use std::fmt;

use super::eval::{Platform, Val};
use super::{ElementDatum, Expr, Func, Literal, MAX_DEPTH, Name, Ref};
use crate::next::inputs::{KdeSchemeKey, KdeWmKey};
use crate::next::key::Key;
use crate::next::platform::kde_shade::KdeShadeRole;
use crate::next::roles::SurfaceSet;
use crate::next::selector::Scheme;
use crate::next::tokens::{KdeGroup, KdeRoleKind};
use crate::next::units::{FontSize, Ms, Px};

/// An operand's, a result's or a field's type (S2.2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Ty {
    /// A colour.
    Color,
    /// A length, in logical pixels.
    Length,
    /// A duration.
    Duration,
    /// An angle.
    Angle,
    /// A plain number (a `Fraction` field's type too, its result clamped).
    Number,
    /// A font size, in logical pixels when evaluated.
    FontSize,
    /// A text.
    Text,
    /// A bool.
    Bool,
    /// An enum: a literal variant, or a reference to a field of that enum.
    Enum(&'static EnumTy),
    /// A keyed name the declaration this names lists (`color_roles`,
    /// `text_styles`, S10): a literal `@key`, or a reference to such a field.
    Key(Cow<'static, str>),
    /// A shadow.
    Shadow,
    /// A paint; a colour expression stated on a paint field is one colour
    /// layer.
    Paint,
    /// A block (`TextStyle`, `FocusIndicator`, `IconStyle`, `Timing`, …),
    /// by its type's name: reached by reference only.
    Block(&'static str),
}

impl Ty {
    /// The type of a block named `name` (S2.2): a paint's and a shadow's
    /// types of their own, every other block's by its name, reached by
    /// reference only.
    #[must_use]
    pub(crate) const fn block(name: &'static str) -> Ty {
        if same(name, "Paint") {
            Ty::Paint
        } else if same(name, "Shadow") {
            Ty::Shadow
        } else {
            Ty::Block(name)
        }
    }
}

/// Whether `a` and `b` are the same text, at compile time.
const fn same(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut rest = (a, b);
    while let ([x, a @ ..], [y, b @ ..]) = rest {
        if *x != *y {
            return false;
        }
        rest = (a, b);
    }
    true
}

/// An enum's name and its variants' names in snake_case, as expressions spell
/// them.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct EnumTy {
    /// The enum's Rust name.
    pub(crate) name: &'static str,
    /// Its variants, in snake_case.
    pub(crate) variants: &'static [&'static str],
}

/// `Scheme` (S7.1), the `scheme` operand's type.
pub(crate) static SCHEME: EnumTy = EnumTy {
    name: "Scheme",
    variants: &Scheme::NAMES,
};

/// `SurfaceSet` (S3), `kde_accent`'s `set`.
pub(crate) static SURFACE_SET: EnumTy = EnumTy {
    name: "SurfaceSet",
    variants: &SurfaceSet::NAMES,
};

/// `KdeGroup` (S4.8), `kde_effects`' and `kde_accent`'s `group`.
pub(crate) static KDE_GROUP: EnumTy = EnumTy {
    name: "KdeGroup",
    variants: &KdeGroup::NAMES,
};

/// `KdeRoleKind` (S4.8), `kde_effects`' and `kde_selection_inactive`'s
/// `role_kind`.
pub(crate) static KDE_ROLE_KIND: EnumTy = EnumTy {
    name: "KdeRoleKind",
    variants: &KdeRoleKind::NAMES,
};

/// `KdeSchemeKey` (S4.8), `kde_accent`'s `key`.
pub(crate) static KDE_SCHEME_KEY: EnumTy = EnumTy {
    name: "KdeSchemeKey",
    variants: &KdeSchemeKey::NAMES,
};

/// `KdeWmKey` (S4.8), `kde_accent_wm`'s `key`.
pub(crate) static KDE_WM_KEY: EnumTy = EnumTy {
    name: "KdeWmKey",
    variants: &KdeWmKey::NAMES,
};

/// `KdeShadeRole` (S2.2), `kde_shade`'s `role`.
pub(crate) static KDE_SHADE_ROLE: EnumTy = EnumTy {
    name: "KdeShadeRole",
    variants: &KdeShadeRole::NAMES,
};

/// `TrackingDesign` (S2.2), `sf_tracking`'s `design`.
pub(crate) static TRACKING_DESIGN: EnumTy = EnumTy {
    name: "TrackingDesign",
    variants: &["sf_pro", "sf_pro_rounded", "new_york"],
};

/// `NameIndexMethod` (S2.2), `name_index`'s `method`.
pub(crate) static NAME_INDEX_METHOD: EnumTy = EnumTy {
    name: "NameIndexMethod",
    variants: &["kirigami", "adwaita"],
};

/// `TextRole` (S1, S4.4): the nineteen `TextRoles` slots, `uifm_scaled`'s
/// `text_role` beside a declared text style.
pub(crate) static TEXT_ROLE: EnumTy = EnumTy {
    name: "TextRole",
    variants: &[
        "body",
        "body_emphasized",
        "caption",
        "caption_emphasized",
        "caption2",
        "footnote",
        "subheadline",
        "callout",
        "headline",
        "title1",
        "title2",
        "title3",
        "large_title",
        "display_large",
        "display_medium",
        "display_small",
        "label",
        "monospace",
        "numeric",
    ],
};

/// A side of a box, `shadow_extent`'s `side`: the members of `Sides` (S1).
pub(crate) static SIDE: EnumTy = EnumTy {
    name: "Side",
    variants: &["top", "right", "bottom", "left"],
};

/// `uikit_prominence`'s `prominence`, whose values S2.2 does not state yet:
/// no name is one of them until it does.
pub(crate) static UIKIT_PROMINENCE: EnumTy = EnumTy {
    name: "Prominence",
    variants: &[],
};

/// How a field limits the number an expression gives it (S2.2's *Totality*).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Bound {
    /// Any finite value.
    Unbounded,
    /// A negative result is zero: a length or duration that cannot be
    /// negative, `Material.saturation`, `MotionTokens.speed_factor`, a
    /// spring's `time_scale`.
    NonNegative,
    /// A `Fraction`: clamped to `0..=1`.
    Fraction,
    /// An integer (a `u16` weight, a `u32` count): floored, and clamped to
    /// `0..=max`.
    Integer {
        /// The integer type's largest value.
        max: u32,
    },
}

/// The type of the field an expression is stated on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FieldType {
    /// The value's type.
    pub(crate) ty: Ty,
    /// How the field limits a computed number.
    pub(crate) bound: Bound,
}

impl FieldType {
    /// A field of type `ty`, limited by `bound`.
    #[must_use]
    pub(crate) fn new(ty: Ty, bound: Bound) -> FieldType {
        FieldType { ty, bound }
    }
}

/// What typing knows of the model: the types of the fields, tokens, roles and
/// inputs references name, and the keyed names each declaration lists.
pub(crate) trait TypeEnv {
    /// The type of what `reference` names, `None` where it names nothing.
    ///
    /// The checker asks for roles, `surface.`, `token.`, `input.`, `part.`,
    /// `widget.` and `values.` references, and for the role inside
    /// `active(…)`; it types `base` (the field's own type), `scheme`,
    /// `font_size`, `line_height`, `body_font_height`, `text_scale` and
    /// `element.` itself.
    fn reference(&self, reference: &Ref) -> Option<Ty>;

    /// Whether the declaration `set` (`color_roles`, `text_styles`, …, S10)
    /// lists `key`.
    fn declares(&self, set: &str, key: &Key) -> bool;
}

/// Why an expression does not type against its field.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TypeError {
    /// The expression's type is not the field's.
    Mismatch {
        /// The field's type.
        expected: Ty,
        /// The expression's.
        found: Ty,
    },
    /// A literal or a call on a block or shadow field, whose expression is a
    /// reference only.
    ReferenceOnly {
        /// The field's type.
        field: Ty,
    },
    /// A call on a bool, text, enum or key field, which takes a literal or a
    /// reference.
    CallNotAllowed {
        /// The field's type.
        field: Ty,
    },
    /// A reference that names nothing.
    UnknownReference(Ref),
    /// `active(…)` around a reference that is not a role.
    ActiveNotRole(Ref),
    /// A call with a number of arguments the function does not take.
    Arity {
        /// The function.
        func: Func,
        /// The number of arguments stated.
        found: usize,
    },
    /// An argument of another type than its parameter's.
    Argument {
        /// The function.
        func: Func,
        /// The argument's position, from zero.
        index: usize,
        /// The parameter's type.
        expected: Ty,
        /// The argument's.
        found: Ty,
    },
    /// An arithmetic operand that is not a number, length, duration or font
    /// size.
    NotArithmetic {
        /// The function.
        func: Func,
        /// The operand's type.
        found: Ty,
    },
    /// `add`, `sub`, `min`, `max` or `div` over operands of two types (a bare
    /// number taking the other's).
    OperandsDiffer {
        /// The function.
        func: Func,
        /// The first operand's type.
        left: Ty,
        /// The second's.
        right: Ty,
    },
    /// `mul` with no number operand.
    NoNumber {
        /// The function.
        func: Func,
        /// The first operand's type.
        left: Ty,
        /// The second's.
        right: Ty,
    },
    /// A name that is not a variant of the enum expected.
    UnknownVariant {
        /// The enum's name.
        ty: &'static str,
        /// The name.
        name: Key,
    },
    /// A keyed name the declaration expected does not list.
    UndeclaredKey {
        /// The declaration.
        set: Cow<'static, str>,
        /// The key.
        key: Key,
    },
    /// A name where no enum or declared key is expected, or a variant where a
    /// key is (and the reverse).
    UnexpectedName {
        /// The name.
        name: Name,
        /// The type expected there, `None` where none is.
        expected: Option<Ty>,
    },
    /// Calls and `active(…)` nested deeper than [`MAX_DEPTH`] (S12).
    TooDeep,
    /// A call of a platform function not implemented yet: each is
    /// implemented to its source, with its vector table, by the task that
    /// needs it (S2.2), and refused until then, so evaluation stays total.
    NotImplemented(Func),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Color => f.write_str("colour"),
            Ty::Length => f.write_str("length"),
            Ty::Duration => f.write_str("duration"),
            Ty::Angle => f.write_str("angle"),
            Ty::Number => f.write_str("number"),
            Ty::FontSize => f.write_str("font size"),
            Ty::Text => f.write_str("text"),
            Ty::Bool => f.write_str("bool"),
            Ty::Enum(e) => write!(f, "enum<{}>", e.name),
            Ty::Key(set) => write!(f, "a key of `{set}`"),
            Ty::Shadow => f.write_str("shadow"),
            Ty::Paint => f.write_str("paint"),
            Ty::Block(name) => f.write_str(name),
        }
    }
}

/// A type with its article: `a colour`, `an angle`, `a key of `…``.
struct Article<'t>(&'t Ty);

impl fmt::Display for Article<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.0.to_string();
        if matches!(self.0, Ty::Key(_)) {
            f.write_str(&name)
        } else if name.starts_with(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']) {
            write!(f, "an {name}")
        } else {
            write!(f, "a {name}")
        }
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::Mismatch { expected, found } => {
                let (found, expected) = (Article(found), Article(expected));
                write!(f, "{found} where {expected} is expected")
            }
            TypeError::ReferenceOnly { field } => {
                write!(f, "a {field} field's expression is a reference only")
            }
            TypeError::CallNotAllowed { field } => {
                write!(
                    f,
                    "a {field} field takes no call, only a literal or a reference"
                )
            }
            TypeError::UnknownReference(r) => write!(f, "`{r}` names nothing"),
            TypeError::ActiveNotRole(r) => write!(f, "`{r}`: `active` takes a role"),
            TypeError::Arity { func, found } => {
                write!(f, "`{func}` does not take {found} arguments")
            }
            TypeError::Argument {
                func,
                index,
                expected,
                found,
            } => write!(
                f,
                "`{func}`'s argument {index} is {} where {} is expected",
                Article(found),
                Article(expected)
            ),
            TypeError::NotArithmetic { func, found } => write!(
                f,
                "`{func}` takes a number, length, duration or font size, not {}",
                Article(found)
            ),
            TypeError::OperandsDiffer { func, left, right } => {
                write!(f, "`{func}` over {} and {}", Article(left), Article(right))
            }
            TypeError::NoNumber { func, left, right } => write!(
                f,
                "`{func}` over {} and {}: one operand is a number",
                Article(left),
                Article(right)
            ),
            TypeError::UnknownVariant { ty, name } => write!(f, "`{name}` is not a {ty}"),
            TypeError::UndeclaredKey { set, key } => {
                write!(f, "`@{key}` is not declared in `{set}`")
            }
            TypeError::UnexpectedName { name, expected } => match expected {
                Some(ty) => write!(f, "the name `{name}` where {} is expected", Article(ty)),
                None => write!(f, "the name `{name}` where no name is expected"),
            },
            TypeError::TooDeep => write!(f, "nested deeper than {MAX_DEPTH}"),
            TypeError::NotImplemented(func) => {
                write!(f, "the platform function `{func}` is not implemented yet")
            }
        }
    }
}

impl std::error::Error for TypeError {}

/// An expression typed against its field, ready to evaluate
/// ([`TypedExpr::eval`]).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TypedExpr {
    pub(super) node: Node,
    pub(super) ty: Ty,
    pub(super) bound: Bound,
}

impl TypedExpr {
    /// The expression's type: the field's, or a length where the field is a
    /// font size; a colour expression on a paint field is a paint of one
    /// `Color` layer (S2.2).
    #[must_use]
    pub(crate) fn ty(&self) -> &Ty {
        &self.ty
    }
}

/// A typed expression's tree, its literals in the units typing gave them and
/// each call in the shape its function takes.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Node {
    /// A value as stated.
    Lit(Val),
    /// A font size as stated, which the environment converts (S9.4).
    Font(FontSize),
    /// A reference.
    Ref(Ref),
    /// `add`, `sub`, `min`, `max`, `mul` or `div`, with its result's type.
    Binary(BinOp, Box<[Node; 2]>, Ty),
    /// `floor(x)`, with its result's type.
    Floor(Box<Node>, Ty),
    /// `round(x)`, with its result's type.
    Round(Box<Node>, Ty),
    /// `alpha(c, a)`.
    Alpha(Box<[Node; 2]>),
    /// `with_alpha(c, a)`.
    WithAlpha(Box<[Node; 2]>),
    /// `mix(a, b, t)`.
    Mix(Box<[Node; 3]>),
    /// `pick(index, c0, c1, …)`.
    Pick(Box<Node>, Vec<Node>),
    /// An implemented platform function's call.
    Platform(Platform, Vec<Node>),
    /// A colour expression stated on a paint field: one `Color` layer, blend
    /// `Normal` (S2.2).
    ColorLayer(Box<Node>),
}

/// The model's own two-operand arithmetic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BinOp {
    Add,
    Sub,
    Min,
    Max,
    Mul,
    Div,
}

/// A function's parameter.
enum Param {
    /// An operand of this type.
    Of(Ty),
    /// `uifm_scaled`'s `x`: a length, or a font size (a `FontSize` widget
    /// value a platform scales, S1).
    LengthOrFontSize,
    /// `uifm_scaled`'s `text_role`: a `TextRoles` slot or a declared text
    /// style.
    SlotOrStyle,
}

const COLOR: Param = Param::Of(Ty::Color);
const NUMBER: Param = Param::Of(Ty::Number);
const LENGTH: Param = Param::Of(Ty::Length);

impl Expr {
    /// Types the expression against the field it is stated on (S2.2): a
    /// field of type `T` takes a literal `T`, a reference to a token or
    /// field of type `T`, and, for scalar and colour fields only, a call; a
    /// block or shadow field a reference only; a length is accepted where a
    /// font size is expected, and a colour expression on a paint field.
    ///
    /// The checks that need the whole stack (absent and `Auto` operands,
    /// declared element data, cycles, where an operand may be read) are
    /// S12's, made when the layer is added.
    pub(crate) fn check(
        &self,
        field: &FieldType,
        env: &dyn TypeEnv,
    ) -> Result<TypedExpr, TypeError> {
        match (&field.ty, self) {
            (Ty::Block(_) | Ty::Shadow, Expr::Lit(_) | Expr::Call(..)) => {
                return Err(TypeError::ReferenceOnly {
                    field: field.ty.clone(),
                });
            }
            (Ty::Bool | Ty::Text | Ty::Enum(_) | Ty::Key(_), Expr::Call(..)) => {
                return Err(TypeError::CallNotAllowed {
                    field: field.ty.clone(),
                });
            }
            _ => {}
        }
        let checker = Checker {
            field: &field.ty,
            env,
        };
        let (node, ty) = checker.infer(self, Some(&field.ty), 0)?;
        if field.ty == Ty::Paint && ty == Ty::Color {
            return Ok(TypedExpr {
                node: Node::ColorLayer(Box::new(node)),
                ty: Ty::Paint,
                bound: field.bound,
            });
        }
        if !accepts(&field.ty, &ty) {
            return Err(TypeError::Mismatch {
                expected: field.ty.clone(),
                found: ty,
            });
        }
        Ok(TypedExpr {
            node,
            ty,
            bound: field.bound,
        })
    }
}

/// Whether a value of type `found` is accepted where `expected` is: the same
/// type, or a length where a font size is expected (as px).
fn accepts(expected: &Ty, found: &Ty) -> bool {
    expected == found || (*expected == Ty::FontSize && *found == Ty::Length)
}

/// The type two operands of `add`, `sub`, `min`, `max` or `div` share: the
/// same, or a font size over a length and a font size.
fn unify(left: &Ty, right: &Ty) -> Option<Ty> {
    match (left, right) {
        _ if left == right => Some(left.clone()),
        (Ty::Length, Ty::FontSize) | (Ty::FontSize, Ty::Length) => Some(Ty::FontSize),
        _ => None,
    }
}

/// A bare number literal's value.
fn bare_number(expr: &Expr) -> Option<f32> {
    match expr {
        Expr::Lit(Literal::Number(n)) => Some(*n),
        _ => None,
    }
}

/// A bare number taking the other operand's type.
fn adopt(n: f32, ty: &Ty) -> Node {
    match ty {
        Ty::Length => Node::Lit(Val::Length(Px(n))),
        Ty::Duration => Node::Lit(Val::Duration(Ms(n))),
        Ty::FontSize => Node::Font(FontSize::Px(n)),
        _ => Node::Lit(Val::Number(n)),
    }
}

/// A typed operand: its node and its type.
type Typed = (Node, Ty);

struct Checker<'a> {
    /// The type of the field the expression is stated on, `base`'s type.
    field: &'a Ty,
    env: &'a dyn TypeEnv,
}

impl Checker<'_> {
    /// The node and type of `expr` at nesting `depth`, where `expected` is
    /// the type its place expects: it decides what a name and a `px` literal
    /// are.
    ///
    /// The depth is counted as the parser counts it, so an expression built
    /// in Rust is held to [`MAX_DEPTH`] too, and refused before it is
    /// followed deeper.
    fn infer(&self, expr: &Expr, expected: Option<&Ty>, depth: usize) -> Result<Typed, TypeError> {
        match expr {
            Expr::Lit(literal) => self.literal(literal, expected),
            Expr::Ref(reference) => {
                if depth.saturating_add(active_depth(reference)) > MAX_DEPTH {
                    return Err(TypeError::TooDeep);
                }
                Ok((Node::Ref(reference.clone()), self.reference(reference)?))
            }
            Expr::Call(func, args) => {
                let depth = depth
                    .checked_add(1)
                    .filter(|depth| *depth <= MAX_DEPTH)
                    .ok_or(TypeError::TooDeep)?;
                self.call(*func, args, expected, depth)
            }
        }
    }

    fn literal(&self, literal: &Literal, expected: Option<&Ty>) -> Result<Typed, TypeError> {
        Ok(match literal {
            Literal::Color(c) => (Node::Lit(Val::Color(*c)), Ty::Color),
            Literal::Number(n) => (Node::Lit(Val::Number(*n)), Ty::Number),
            Literal::Px(px) if expected == Some(&Ty::FontSize) => {
                (Node::Font(FontSize::Px(px.0)), Ty::FontSize)
            }
            Literal::Px(px) => (Node::Lit(Val::Length(*px)), Ty::Length),
            Literal::Ms(ms) => (Node::Lit(Val::Duration(*ms)), Ty::Duration),
            Literal::Deg(deg) => (Node::Lit(Val::Angle(*deg)), Ty::Angle),
            Literal::Pt(pt) => (Node::Font(FontSize::Pt(*pt)), Ty::FontSize),
            Literal::Bool(b) => (Node::Lit(Val::Bool(*b)), Ty::Bool),
            Literal::Name(name) => return self.name(name, expected),
        })
    }

    fn name(&self, name: &Name, expected: Option<&Ty>) -> Result<Typed, TypeError> {
        let ty = match (name, expected) {
            (Name::Variant(variant), Some(Ty::Enum(e))) => {
                if !e.variants.contains(&variant.as_str()) {
                    return Err(TypeError::UnknownVariant {
                        ty: e.name,
                        name: variant.clone(),
                    });
                }
                Ty::Enum(e)
            }
            (Name::Key(key), Some(Ty::Key(set))) => {
                if !self.env.declares(set, key) {
                    return Err(TypeError::UndeclaredKey {
                        set: set.clone(),
                        key: key.clone(),
                    });
                }
                Ty::Key(set.clone())
            }
            _ => {
                return Err(TypeError::UnexpectedName {
                    name: name.clone(),
                    expected: expected.cloned(),
                });
            }
        };
        Ok((Node::Lit(Val::Name(name.clone())), ty))
    }

    fn reference(&self, reference: &Ref) -> Result<Ty, TypeError> {
        let from_env = |r: &Ref| {
            self.env
                .reference(r)
                .ok_or_else(|| TypeError::UnknownReference(r.clone()))
        };
        match reference {
            Ref::Base => Ok(self.field.clone()),
            Ref::Scheme => Ok(Ty::Enum(&SCHEME)),
            Ref::FontSize | Ref::LineHeight | Ref::BodyFontHeight => Ok(Ty::Length),
            Ref::TextScale => Ok(Ty::Number),
            Ref::Element(datum) => Ok(datum_type(*datum)),
            Ref::Active(inner) => match **inner {
                Ref::Role { .. } | Ref::GlobalRole(_) | Ref::KeyedRole(_) | Ref::Surface(_) => {
                    from_env(inner)
                }
                _ => Err(TypeError::ActiveNotRole(reference.clone())),
            },
            Ref::Role { .. }
            | Ref::GlobalRole(_)
            | Ref::KeyedRole(_)
            | Ref::Surface(_)
            | Ref::Token(_)
            | Ref::Input(_)
            | Ref::Part(_)
            | Ref::Widget { .. }
            | Ref::Values(_) => from_env(reference),
        }
    }

    fn call(
        &self,
        func: Func,
        args: &[Expr],
        expected: Option<&Ty>,
        depth: usize,
    ) -> Result<Typed, TypeError> {
        let color = || Ty::Color;
        match func {
            Func::Add => self.same_type(func, BinOp::Add, args, expected, depth),
            Func::Sub => self.same_type(func, BinOp::Sub, args, expected, depth),
            Func::Min => self.same_type(func, BinOp::Min, args, expected, depth),
            Func::Max => self.same_type(func, BinOp::Max, args, expected, depth),
            Func::Mul => self.mul(args, expected, depth),
            Func::Div => self.div(args, expected, depth),
            Func::Floor | Func::Round => {
                let [x] = args else {
                    return Err(arity(func, args));
                };
                let (node, ty) = self.infer(x, expected, depth)?;
                arithmetic(func, &ty)?;
                let node = Box::new(node);
                let node = if func == Func::Floor {
                    Node::Floor(node, ty.clone())
                } else {
                    Node::Round(node, ty.clone())
                };
                Ok((node, ty))
            }
            Func::Pick => self.pick(args, depth),
            Func::Alpha | Func::WithAlpha => {
                let nodes = self.arguments(func, args, depth, &[COLOR, NUMBER])?;
                let nodes = Box::new(<[Node; 2]>::try_from(nodes).map_err(|_| arity(func, args))?);
                let node = if func == Func::Alpha {
                    Node::Alpha(nodes)
                } else {
                    Node::WithAlpha(nodes)
                };
                Ok((node, Ty::Color))
            }
            Func::Mix => {
                let nodes = self.arguments(func, args, depth, &[COLOR, COLOR, NUMBER])?;
                let nodes = <[Node; 3]>::try_from(nodes).map_err(|_| arity(func, args))?;
                Ok((Node::Mix(Box::new(nodes)), Ty::Color))
            }
            Func::QtLighter | Func::QtDarker => {
                self.fixed(func, args, depth, &[COLOR, NUMBER], color())
            }
            Func::KcuMix | Func::KcuTint | Func::KirigamiTint | Func::KirigamiLerp => {
                self.fixed(func, args, depth, &[COLOR, COLOR, NUMBER], color())
            }
            Func::KdeEffects => self.fixed(
                func,
                args,
                depth,
                &[
                    COLOR,
                    COLOR,
                    Param::Of(Ty::Enum(&KDE_GROUP)),
                    Param::Of(Ty::Enum(&KDE_ROLE_KIND)),
                ],
                color(),
            ),
            Func::KdeSelectionInactive => self.fixed(
                func,
                args,
                depth,
                &[COLOR, COLOR, COLOR, Param::Of(Ty::Enum(&KDE_ROLE_KIND))],
                color(),
            ),
            Func::KdeShade => self.fixed(
                func,
                args,
                depth,
                &[COLOR, Param::Of(Ty::Enum(&KDE_SHADE_ROLE)), NUMBER],
                color(),
            ),
            Func::KcuShade | Func::KqcSeriesColor => {
                self.fixed(func, args, depth, &[COLOR, NUMBER, NUMBER], color())
            }
            Func::AdwStandalone => self.fixed(
                func,
                args,
                depth,
                &[COLOR, Param::Of(Ty::Enum(&SCHEME))],
                color(),
            ),
            Func::M3Role => self.fixed(
                func,
                args,
                depth,
                &[Param::Of(Ty::Key(Cow::Borrowed("color_roles")))],
                color(),
            ),
            Func::KcuOverlay
            | Func::KirigamiBadgeText
            | Func::KaCardBorder
            | Func::KaBannerLine
            | Func::KteMinimapMark => self.fixed(func, args, depth, &[COLOR, COLOR], color()),
            Func::KaContrastText => self.fixed(func, args, depth, &[COLOR], color()),
            Func::KteMarkUnderlay | Func::GtkContrastPick => {
                self.fixed(func, args, depth, &[COLOR, COLOR, COLOR], color())
            }
            Func::KirigamiLabelIconSize | Func::GtkWheelStep => {
                self.fixed(func, args, depth, &[LENGTH], Ty::Length)
            }
            Func::SfTracking => self.fixed(
                func,
                args,
                depth,
                &[
                    Param::Of(Ty::FontSize),
                    Param::Of(Ty::Enum(&TRACKING_DESIGN)),
                ],
                Ty::Length,
            ),
            Func::UikitProminence => self.fixed(
                func,
                args,
                depth,
                &[COLOR, Param::Of(Ty::Enum(&UIKIT_PROMINENCE))],
                color(),
            ),
            Func::ShadowExtent => self.fixed(
                func,
                args,
                depth,
                &[Param::Of(Ty::Shadow), Param::Of(Ty::Enum(&SIDE))],
                Ty::Length,
            ),
            Func::UifmScaled => self.fixed(
                func,
                args,
                depth,
                &[Param::LengthOrFontSize, Param::SlotOrStyle],
                Ty::Length,
            ),
            Func::KdeAccent => self.fixed(
                func,
                args,
                depth,
                &[
                    Param::Of(Ty::Enum(&SURFACE_SET)),
                    Param::Of(Ty::Enum(&KDE_SCHEME_KEY)),
                    Param::Of(Ty::Enum(&KDE_GROUP)),
                ],
                color(),
            ),
            Func::KdeAccentWm => self.fixed(
                func,
                args,
                depth,
                &[Param::Of(Ty::Enum(&KDE_WM_KEY))],
                color(),
            ),
            Func::NameIndex => self.fixed(
                func,
                args,
                depth,
                &[
                    Param::Of(Ty::Text),
                    NUMBER,
                    Param::Of(Ty::Enum(&NAME_INDEX_METHOD)),
                ],
                Ty::Number,
            ),
        }
    }

    /// A platform function's call, each argument typed against its
    /// parameter; refused where the function is not implemented yet.
    fn fixed(
        &self,
        func: Func,
        args: &[Expr],
        depth: usize,
        params: &[Param],
        result: Ty,
    ) -> Result<Typed, TypeError> {
        let nodes = self.arguments(func, args, depth, params)?;
        let platform = Platform::of(func).ok_or(TypeError::NotImplemented(func))?;
        Ok((Node::Platform(platform, nodes), result))
    }

    /// The arguments of a function of fixed parameters, each typed against
    /// its parameter.
    fn arguments(
        &self,
        func: Func,
        args: &[Expr],
        depth: usize,
        params: &[Param],
    ) -> Result<Vec<Node>, TypeError> {
        if args.len() != params.len() {
            return Err(arity(func, args));
        }
        let mut nodes = Vec::with_capacity(args.len());
        for (index, (arg, param)) in args.iter().zip(params).enumerate() {
            let expected = match (param, arg) {
                (Param::SlotOrStyle, Expr::Lit(Literal::Name(Name::Key(_)))) => {
                    Ty::Key(Cow::Borrowed("text_styles"))
                }
                (Param::SlotOrStyle, _) => Ty::Enum(&TEXT_ROLE),
                (Param::LengthOrFontSize, _) => Ty::Length,
                (Param::Of(ty), _) => ty.clone(),
            };
            let (node, found) = self.infer(arg, Some(&expected), depth)?;
            let accepted = match param {
                Param::LengthOrFontSize => matches!(found, Ty::Length | Ty::FontSize),
                Param::Of(_) | Param::SlotOrStyle => accepts(&expected, &found),
            };
            if !accepted {
                return Err(TypeError::Argument {
                    func,
                    index,
                    expected,
                    found,
                });
            }
            nodes.push(node);
        }
        Ok(nodes)
    }

    /// `add`, `sub`, `min`, `max`: two operands of one type, a bare number
    /// taking the other operand's.
    fn same_type(
        &self,
        func: Func,
        op: BinOp,
        args: &[Expr],
        expected: Option<&Ty>,
        depth: usize,
    ) -> Result<Typed, TypeError> {
        let [a, b] = args else {
            return Err(arity(func, args));
        };
        let (left, right, ty) = match (bare_number(a), bare_number(b)) {
            (Some(x), Some(y)) => (
                Node::Lit(Val::Number(x)),
                Node::Lit(Val::Number(y)),
                Ty::Number,
            ),
            (Some(x), None) => {
                let (right, ty) = self.infer(b, expected, depth)?;
                arithmetic(func, &ty)?;
                (adopt(x, &ty), right, ty)
            }
            (None, Some(y)) => {
                let (left, ty) = self.infer(a, expected, depth)?;
                arithmetic(func, &ty)?;
                (left, adopt(y, &ty), ty)
            }
            (None, None) => {
                let ((left, left_ty), (right, right_ty)) = self.pair(a, b, expected, depth)?;
                arithmetic(func, &left_ty)?;
                arithmetic(func, &right_ty)?;
                let ty = unify(&left_ty, &right_ty).ok_or(TypeError::OperandsDiffer {
                    func,
                    left: left_ty,
                    right: right_ty,
                })?;
                (left, right, ty)
            }
        };
        Ok((Node::Binary(op, Box::new([left, right]), ty.clone()), ty))
    }

    /// Two operands of one type: the one that is not a literal typed first,
    /// so a `px` literal beside a font size is one too.
    fn pair(
        &self,
        a: &Expr,
        b: &Expr,
        expected: Option<&Ty>,
        depth: usize,
    ) -> Result<(Typed, Typed), TypeError> {
        if matches!(a, Expr::Lit(_)) && !matches!(b, Expr::Lit(_)) {
            let right = self.infer(b, expected, depth)?;
            let left = self.infer(a, Some(&right.1), depth)?;
            Ok((left, right))
        } else {
            let left = self.infer(a, expected, depth)?;
            let right = self.infer(b, Some(&left.1), depth)?;
            Ok((left, right))
        }
    }

    /// `mul(T, number)` or `mul(number, T)`: a `T`.
    fn mul(&self, args: &[Expr], expected: Option<&Ty>, depth: usize) -> Result<Typed, TypeError> {
        let func = Func::Mul;
        let [a, b] = args else {
            return Err(arity(func, args));
        };
        let (left, left_ty) = self.infer(a, expected, depth)?;
        let (right, right_ty) = self.infer(b, expected, depth)?;
        arithmetic(func, &left_ty)?;
        arithmetic(func, &right_ty)?;
        let ty = match (&left_ty, &right_ty) {
            (Ty::Number, ty) | (ty, Ty::Number) => ty.clone(),
            _ => {
                return Err(TypeError::NoNumber {
                    func,
                    left: left_ty,
                    right: right_ty,
                });
            }
        };
        Ok((
            Node::Binary(BinOp::Mul, Box::new([left, right]), ty.clone()),
            ty,
        ))
    }

    /// `div(T, number)`: a `T`; `div(T, T)`: a number.
    fn div(&self, args: &[Expr], expected: Option<&Ty>, depth: usize) -> Result<Typed, TypeError> {
        let func = Func::Div;
        let [a, b] = args else {
            return Err(arity(func, args));
        };
        let ((left, left_ty), (right, right_ty)) = self.pair(a, b, expected, depth)?;
        arithmetic(func, &left_ty)?;
        arithmetic(func, &right_ty)?;
        let ty = if right_ty == Ty::Number {
            left_ty
        } else if unify(&left_ty, &right_ty).is_some() {
            Ty::Number
        } else {
            return Err(TypeError::OperandsDiffer {
                func,
                left: left_ty,
                right: right_ty,
            });
        };
        Ok((
            Node::Binary(BinOp::Div, Box::new([left, right]), ty.clone()),
            ty,
        ))
    }

    /// `pick(index, c0, c1, …)`: a number and at least one colour.
    fn pick(&self, args: &[Expr], depth: usize) -> Result<Typed, TypeError> {
        let func = Func::Pick;
        let [index, colors @ ..] = args else {
            return Err(arity(func, args));
        };
        if colors.is_empty() {
            return Err(arity(func, args));
        }
        let index = self.arguments(func, std::slice::from_ref(index), depth, &[NUMBER])?;
        let [index] = <[Node; 1]>::try_from(index).map_err(|_| arity(func, args))?;
        let mut nodes = Vec::with_capacity(colors.len());
        for (position, color) in colors.iter().enumerate() {
            let (node, found) = self.infer(color, Some(&Ty::Color), depth)?;
            if found != Ty::Color {
                return Err(TypeError::Argument {
                    func,
                    index: position.saturating_add(1),
                    expected: Ty::Color,
                    found,
                });
            }
            nodes.push(node);
        }
        Ok((Node::Pick(Box::new(index), nodes), Ty::Color))
    }
}

fn arity(func: Func, args: &[Expr]) -> TypeError {
    TypeError::Arity {
        func,
        found: args.len(),
    }
}

/// Refuses an arithmetic operand that is not a number, length, duration or
/// font size.
fn arithmetic(func: Func, ty: &Ty) -> Result<(), TypeError> {
    match ty {
        Ty::Number | Ty::Length | Ty::Duration | Ty::FontSize => Ok(()),
        _ => Err(TypeError::NotArithmetic {
            func,
            found: ty.clone(),
        }),
    }
}

/// How many `active(…)` enclose the reference at the end of `reference`'s
/// chain, counted without recursion.
fn active_depth(reference: &Ref) -> usize {
    let mut depth = 0_usize;
    let mut reference = reference;
    while let Ref::Active(inner) = reference {
        depth = depth.saturating_add(1);
        reference = inner;
    }
    depth
}

/// An `element.` operand's type.
fn datum_type(datum: ElementDatum) -> Ty {
    match datum {
        ElementDatum::Text => Ty::Text,
        ElementDatum::Color => Ty::Color,
        ElementDatum::Progress
        | ElementDatum::ScrollProgress
        | ElementDatum::ItemIndex
        | ElementDatum::ItemCount => Ty::Number,
        ElementDatum::Width
        | ElementDatum::Height
        | ElementDatum::ScrollOffset
        | ElementDatum::ScreenWidth
        | ElementDatum::ScreenHeight
        | ElementDatum::WindowWidth
        | ElementDatum::WindowHeight
        | ElementDatum::MoveDistance
        | ElementDatum::CharWidth
        | ElementDatum::MaxCharWidth
        | ElementDatum::FontXHeight
        | ElementDatum::FontHeight
        | ElementDatum::FontUnderlineOffset
        | ElementDatum::FontUnderlineThickness
        | ElementDatum::FontStrikeoutOffset
        | ElementDatum::FontStrikeoutThickness => Ty::Length,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::key;

    /// Synthetic field types for the tests: one reference per type.
    struct Fixture;

    impl TypeEnv for Fixture {
        fn reference(&self, reference: &Ref) -> Option<Ty> {
            Some(match reference.to_string().as_str() {
                "token.gap" | "part.min_width" => Ty::Length,
                "token.speed" => Ty::Number,
                "token.duration" => Ty::Duration,
                "token.turn" => Ty::Angle,
                "token.label_size" => Ty::FontSize,
                "token.family" => Ty::Text,
                "token.focus_visible" => Ty::Bool,
                "token.kde_group" => Ty::Enum(&KDE_GROUP),
                "token.engine" => Ty::Key(Cow::Borrowed("rendering_engines")),
                "token.focus" => Ty::Block("FocusIndicator"),
                "token.text" => Ty::Block("TextStyle"),
                "widget.frame.shadow" => Ty::Shadow,
                "token.fill" => Ty::Paint,
                "surface.foreground" | "role.global.accent" | "role.window.background" => Ty::Color,
                _ => return None,
            })
        }

        fn declares(&self, set: &str, key: &Key) -> bool {
            matches!(
                (set, key.as_str()),
                ("color_roles", "primary") | ("text_styles", "body") | ("rendering_engines", "qt")
            )
        }
    }

    fn check(text: &str, ty: Ty) -> Result<TypedExpr, TypeError> {
        let expr = Expr::parse(text).unwrap_or_else(|e| panic!("{text}: {e}"));
        expr.check(&FieldType::new(ty, Bound::Unbounded), &Fixture)
    }

    fn typed(text: &str, ty: Ty) -> Ty {
        check(text, ty)
            .unwrap_or_else(|e| panic!("{text}: {e}"))
            .ty()
            .clone()
    }

    fn refused(text: &str, ty: Ty) -> TypeError {
        check(text, ty)
            .map(|t| panic!("{text} typed as {}", t.ty()))
            .unwrap_err()
    }

    #[test]
    fn one_rule_for_every_field() {
        // A stated value, a reference of the field's type, and for scalar and colour fields a
        // call.
        assert_eq!(typed("4px", Ty::Length), Ty::Length);
        assert_eq!(typed("token.gap", Ty::Length), Ty::Length);
        assert_eq!(typed("add(token.gap, 2px)", Ty::Length), Ty::Length);
        assert_eq!(typed("#ff0000", Ty::Color), Ty::Color);
        assert_eq!(
            typed("alpha(surface.foreground, 0.5)", Ty::Color),
            Ty::Color
        );
        assert_eq!(typed("true", Ty::Bool), Ty::Bool);
        assert_eq!(typed("token.focus_visible", Ty::Bool), Ty::Bool);
        assert_eq!(
            typed("inactive", Ty::Enum(&KDE_GROUP)),
            Ty::Enum(&KDE_GROUP)
        );
        assert_eq!(
            typed("token.kde_group", Ty::Enum(&KDE_GROUP)),
            Ty::Enum(&KDE_GROUP)
        );
        let engines = Ty::Key(Cow::Borrowed("rendering_engines"));
        assert_eq!(typed("@qt", engines.clone()), engines);
        assert_eq!(typed("token.engine", engines.clone()), engines);
        assert_eq!(typed("token.family", Ty::Text), Ty::Text);
        let focus = Ty::Block("FocusIndicator");
        assert_eq!(typed("token.focus", focus.clone()), focus);
        assert_eq!(typed("widget.frame.shadow", Ty::Shadow), Ty::Shadow);
        assert_eq!(typed("token.fill", Ty::Paint), Ty::Paint);
        assert_eq!(typed("base", Ty::Paint), Ty::Paint);
        assert_eq!(typed("10pt", Ty::FontSize), Ty::FontSize);
        assert_eq!(typed("150ms", Ty::Duration), Ty::Duration);
        assert_eq!(typed("90deg", Ty::Angle), Ty::Angle);
        assert_eq!(typed("text_scale", Ty::Number), Ty::Number);
        assert_eq!(typed("scheme", Ty::Enum(&SCHEME)), Ty::Enum(&SCHEME));
        assert_eq!(typed("element.text", Ty::Text), Ty::Text);
        assert_eq!(typed("element.color", Ty::Color), Ty::Color);
        assert_eq!(typed("element.progress", Ty::Number), Ty::Number);
        assert_eq!(typed("element.char_width", Ty::Length), Ty::Length);
    }

    #[test]
    fn length_accepted_as_font_size() {
        assert_eq!(typed("font_size", Ty::FontSize), Ty::Length);
        assert_eq!(typed("12px", Ty::FontSize), Ty::FontSize);
        assert_eq!(
            typed("add(token.label_size, 2px)", Ty::FontSize),
            Ty::FontSize
        );
        assert_eq!(
            typed("add(token.label_size, token.gap)", Ty::FontSize),
            Ty::FontSize
        );
        assert_eq!(typed("mul(font_size, 1.25)", Ty::FontSize), Ty::Length);
        assert_eq!(
            refused("token.label_size", Ty::Length),
            TypeError::Mismatch {
                expected: Ty::Length,
                found: Ty::FontSize
            }
        );
    }

    #[test]
    fn paint_field_takes_colour_expression() {
        // A colour expression on a paint field is one colour layer, so the
        // typed expression is a paint; a paint reference stays one.
        assert_eq!(typed("#ff0000", Ty::Paint), Ty::Paint);
        assert_eq!(typed("surface.foreground", Ty::Paint), Ty::Paint);
        assert_eq!(
            typed("mix(surface.foreground, #000000, 0.5)", Ty::Paint),
            Ty::Paint
        );
        assert_eq!(typed("token.fill", Ty::Paint), Ty::Paint);
    }

    #[test]
    fn bare_number_takes_other_operand_type() {
        assert_eq!(typed("add(token.gap, 4)", Ty::Length), Ty::Length);
        assert_eq!(typed("sub(4, token.gap)", Ty::Length), Ty::Length);
        assert_eq!(
            typed("min(token.duration, 100)", Ty::Duration),
            Ty::Duration
        );
        assert_eq!(
            typed("max(-1, token.label_size)", Ty::FontSize),
            Ty::FontSize
        );
        assert_eq!(typed("add(1, 2)", Ty::Number), Ty::Number);
        // A number-typed reference is not bare.
        assert_eq!(
            refused("add(token.gap, token.speed)", Ty::Length),
            TypeError::OperandsDiffer {
                func: Func::Add,
                left: Ty::Length,
                right: Ty::Number
            }
        );
    }

    #[test]
    fn mul_div_floor_types() {
        assert_eq!(
            typed("mul(token.duration, token.speed)", Ty::Duration),
            Ty::Duration
        );
        assert_eq!(typed("mul(2, token.gap)", Ty::Length), Ty::Length);
        assert_eq!(typed("div(token.gap, 2)", Ty::Length), Ty::Length);
        assert_eq!(typed("div(token.gap, 2px)", Ty::Number), Ty::Number);
        assert_eq!(
            typed("div(token.label_size, token.gap)", Ty::Number),
            Ty::Number
        );
        assert_eq!(typed("floor(token.gap)", Ty::Length), Ty::Length);
        assert_eq!(typed("floor(1.5)", Ty::Number), Ty::Number);
        assert_eq!(typed("round(token.gap)", Ty::Length), Ty::Length);
        assert_eq!(typed("round(1.5)", Ty::Number), Ty::Number);
    }

    // The mismatch classes S2.2 lists, one test each.

    #[test]
    fn typing_refuses_mismatched_literal_and_reference() {
        assert_eq!(
            refused("4px", Ty::Color),
            TypeError::Mismatch {
                expected: Ty::Color,
                found: Ty::Length
            }
        );
        assert_eq!(
            refused("surface.foreground", Ty::Length),
            TypeError::Mismatch {
                expected: Ty::Length,
                found: Ty::Color
            }
        );
        assert_eq!(
            refused("4", Ty::Length),
            TypeError::Mismatch {
                expected: Ty::Length,
                found: Ty::Number
            }
        );
        assert_eq!(
            refused("token.text", Ty::Block("FocusIndicator")),
            TypeError::Mismatch {
                expected: Ty::Block("FocusIndicator"),
                found: Ty::Block("TextStyle")
            }
        );
        assert_eq!(
            refused("token.fill", Ty::Color),
            TypeError::Mismatch {
                expected: Ty::Color,
                found: Ty::Paint
            }
        );
    }

    #[test]
    fn typing_refuses_non_reference_on_block_field() {
        for (text, ty) in [
            ("#ff0000", Ty::Block("FocusIndicator")),
            ("add(1px, 2px)", Ty::Block("TextStyle")),
            ("4px", Ty::Shadow),
        ] {
            assert_eq!(
                refused(text, ty.clone()),
                TypeError::ReferenceOnly { field: ty }
            );
        }
    }

    #[test]
    fn typing_refuses_call_on_bool_text_enum_key_field() {
        for ty in [
            Ty::Bool,
            Ty::Text,
            Ty::Enum(&KDE_GROUP),
            Ty::Key(Cow::Borrowed("rendering_engines")),
        ] {
            assert_eq!(
                refused("pick(1, #ff0000)", ty.clone()),
                TypeError::CallNotAllowed { field: ty }
            );
        }
    }

    #[test]
    fn typing_refuses_mistyped_argument() {
        assert_eq!(
            refused("alpha(4px, 0.5)", Ty::Color),
            TypeError::Argument {
                func: Func::Alpha,
                index: 0,
                expected: Ty::Color,
                found: Ty::Length
            }
        );
        assert_eq!(
            refused("mix(#000000, #ffffff, token.gap)", Ty::Color),
            TypeError::Argument {
                func: Func::Mix,
                index: 2,
                expected: Ty::Number,
                found: Ty::Length
            }
        );
        assert_eq!(
            refused("pick(1, #ff0000, 4px)", Ty::Color),
            TypeError::Argument {
                func: Func::Pick,
                index: 2,
                expected: Ty::Color,
                found: Ty::Length
            }
        );
        assert_eq!(
            refused("shadow_extent(token.fill, top)", Ty::Length),
            TypeError::Argument {
                func: Func::ShadowExtent,
                index: 0,
                expected: Ty::Shadow,
                found: Ty::Paint
            }
        );
    }

    #[test]
    fn typing_refuses_non_arithmetic_operand() {
        for (text, func, found) in [
            ("add(#ff0000, #000000)", Func::Add, Ty::Color),
            ("max(token.turn, 1deg)", Func::Max, Ty::Angle),
            ("mul(token.focus_visible, 2)", Func::Mul, Ty::Bool),
            ("floor(surface.foreground)", Func::Floor, Ty::Color),
            ("round(surface.foreground)", Func::Round, Ty::Color),
        ] {
            assert_eq!(
                refused(text, Ty::Number),
                TypeError::NotArithmetic { func, found }
            );
        }
    }

    #[test]
    fn typing_refuses_operands_of_two_types() {
        assert_eq!(
            refused("add(1px, 1ms)", Ty::Length),
            TypeError::OperandsDiffer {
                func: Func::Add,
                left: Ty::Length,
                right: Ty::Duration
            }
        );
        assert_eq!(
            refused("div(2, token.gap)", Ty::Number),
            TypeError::OperandsDiffer {
                func: Func::Div,
                left: Ty::Number,
                right: Ty::Length
            }
        );
    }

    #[test]
    fn typing_refuses_mul_without_number() {
        assert_eq!(
            refused("mul(token.gap, 2px)", Ty::Length),
            TypeError::NoNumber {
                func: Func::Mul,
                left: Ty::Length,
                right: Ty::Length
            }
        );
    }

    #[test]
    fn typing_refuses_names_outside_their_set() {
        assert_eq!(
            refused(
                "kde_effects(#000000, #ffffff, normal, foreground)",
                Ty::Color
            ),
            TypeError::UnknownVariant {
                ty: "KdeGroup",
                name: key!("normal")
            }
        );
        assert_eq!(
            refused("m3_role(@tertiary)", Ty::Color),
            TypeError::UndeclaredKey {
                set: Cow::Borrowed("color_roles"),
                key: key!("tertiary")
            }
        );
        assert_eq!(
            refused("m3_role(primary)", Ty::Color),
            TypeError::UnexpectedName {
                name: Name::Variant(key!("primary")),
                expected: Some(Ty::Key(Cow::Borrowed("color_roles")))
            }
        );
        assert_eq!(
            refused("alpha(inactive, 0.5)", Ty::Color),
            TypeError::UnexpectedName {
                name: Name::Variant(key!("inactive")),
                expected: Some(Ty::Color)
            }
        );
        assert_eq!(
            refused("uikit_prominence(#000000, secondary)", Ty::Color),
            TypeError::UnknownVariant {
                ty: "Prominence",
                name: key!("secondary")
            }
        );
        assert_eq!(
            refused("uifm_scaled(4px, @caption)", Ty::Length),
            TypeError::UndeclaredKey {
                set: Cow::Borrowed("text_styles"),
                key: key!("caption")
            }
        );
        // Arguments that type: the call reaches the platform function, not
        // implemented yet.
        for (text, ty, func) in [
            ("m3_role(@primary)", Ty::Color, Func::M3Role),
            ("uifm_scaled(4px, caption2)", Ty::Length, Func::UifmScaled),
            ("uifm_scaled(4px, @body)", Ty::Length, Func::UifmScaled),
            (
                "kde_accent(header, foreground_link, inactive)",
                Ty::Color,
                Func::KdeAccent,
            ),
            (
                "kde_accent_wm(active_background)",
                Ty::Color,
                Func::KdeAccentWm,
            ),
            (
                "sf_tracking(17pt, sf_pro_rounded)",
                Ty::Length,
                Func::SfTracking,
            ),
            (
                "name_index(element.text, 8, adwaita)",
                Ty::Number,
                Func::NameIndex,
            ),
            (
                "adw_standalone(role.global.accent, scheme)",
                Ty::Color,
                Func::AdwStandalone,
            ),
            (
                "shadow_extent(widget.frame.shadow, left)",
                Ty::Length,
                Func::ShadowExtent,
            ),
        ] {
            assert_eq!(refused(text, ty), TypeError::NotImplemented(func), "{text}");
        }
    }

    #[test]
    fn typing_refuses_platform_function_not_implemented() {
        assert_eq!(
            refused("qt_lighter(surface.foreground, 1.5)", Ty::Color),
            TypeError::NotImplemented(Func::QtLighter)
        );
        assert_eq!(
            refused(
                "mix(kcu_overlay(surface.foreground, #000000), #ffffff, 0.5)",
                Ty::Color
            ),
            TypeError::NotImplemented(Func::KcuOverlay)
        );
        // The arguments are typed first.
        assert_eq!(
            refused("qt_lighter(4px, 1.5)", Ty::Color),
            TypeError::Argument {
                func: Func::QtLighter,
                index: 0,
                expected: Ty::Color,
                found: Ty::Length
            }
        );
    }

    #[test]
    fn kirigami_lerp_is_implemented() {
        assert_eq!(
            typed(
                "kirigami_lerp(surface.foreground, #000000, 0.15)",
                Ty::Color
            ),
            Ty::Color
        );
        assert_eq!(
            refused("kirigami_lerp(surface.foreground, 4px, 0.15)", Ty::Color),
            TypeError::Argument {
                func: Func::KirigamiLerp,
                index: 1,
                expected: Ty::Color,
                found: Ty::Length
            }
        );
    }

    #[test]
    fn kde_shade_is_implemented() {
        assert_eq!(
            typed("kde_shade(surface.foreground, dark, 4)", Ty::Color),
            Ty::Color
        );
        assert_eq!(
            refused("kde_shade(surface.foreground, highlight, 4)", Ty::Color),
            TypeError::UnknownVariant {
                ty: "KdeShadeRole",
                name: Key::new("highlight").unwrap()
            }
        );
        assert_eq!(
            refused("kde_shade(surface.foreground, dark)", Ty::Color),
            TypeError::Arity {
                func: Func::KdeShade,
                found: 2
            }
        );
    }

    #[test]
    fn uifm_scaled_takes_length_or_font_size() {
        for text in [
            "uifm_scaled(4px, body)",
            "uifm_scaled(token.gap, body)",
            "uifm_scaled(17pt, body)",
            "uifm_scaled(token.label_size, body)",
            "uifm_scaled(add(token.label_size, 2px), @body)",
        ] {
            assert_eq!(
                refused(text, Ty::FontSize),
                TypeError::NotImplemented(Func::UifmScaled),
                "{text}"
            );
        }
        assert_eq!(
            refused("uifm_scaled(token.duration, body)", Ty::Length),
            TypeError::Argument {
                func: Func::UifmScaled,
                index: 0,
                expected: Ty::Length,
                found: Ty::Duration
            }
        );
    }

    /// `floor(…)` around `inner`, `calls` deep, built in Rust.
    fn floors(calls: usize, inner: Expr) -> Expr {
        (0..calls).fold(inner, |expr, _| Expr::Call(Func::Floor, vec![expr]))
    }

    /// `active(…)` around `role.global.accent`, `depth` deep, built in Rust.
    fn actives(depth: usize) -> Expr {
        let accent = Ref::GlobalRole(vec![key!("accent")]);
        Expr::Ref((0..depth).fold(accent, |inner, _| Ref::Active(Box::new(inner))))
    }

    #[test]
    fn typing_refuses_depth_33_built_in_rust() {
        let field = FieldType::new(Ty::Length, Bound::Unbounded);
        let gap = || Expr::Ref(Ref::Token(vec![key!("gap")]));
        assert!(floors(MAX_DEPTH, gap()).check(&field, &Fixture).is_ok());
        for calls in [MAX_DEPTH + 1, 1000] {
            assert_eq!(
                floors(calls, gap()).check(&field, &Fixture),
                Err(TypeError::TooDeep),
                "{calls}"
            );
        }
        let colour = FieldType::new(Ty::Color, Bound::Unbounded);
        assert_eq!(
            actives(MAX_DEPTH + 1).check(&colour, &Fixture),
            Err(TypeError::TooDeep)
        );
        // `active(…)` counts with the calls around it, as the parser counts it.
        assert_eq!(
            Expr::Call(
                Func::Alpha,
                vec![actives(MAX_DEPTH), Expr::Lit(Literal::Number(0.5))]
            )
            .check(&colour, &Fixture),
            Err(TypeError::TooDeep)
        );
        assert_ne!(
            Expr::Call(
                Func::Alpha,
                vec![actives(MAX_DEPTH - 1), Expr::Lit(Literal::Number(0.5))]
            )
            .check(&colour, &Fixture),
            Err(TypeError::TooDeep)
        );
    }

    #[test]
    fn typing_refuses_wrong_arity() {
        for (text, func, found) in [
            ("pick(1)", Func::Pick, 1),
            ("alpha(#ff0000)", Func::Alpha, 1),
            ("mix(#ff0000, #000000)", Func::Mix, 2),
            ("add(1, 2, 3)", Func::Add, 3),
            ("floor(1, 2)", Func::Floor, 2),
            (
                "kde_accent_wm(active_background, inactive_background)",
                Func::KdeAccentWm,
                2,
            ),
            ("kirigami_lerp(#ff0000, #000000)", Func::KirigamiLerp, 2),
            ("with_alpha(#ff0000)", Func::WithAlpha, 1),
        ] {
            assert_eq!(refused(text, Ty::Color), TypeError::Arity { func, found });
        }
    }

    #[test]
    fn typing_refuses_unknown_reference_and_active_of_non_role() {
        assert_eq!(
            refused("token.nowhere", Ty::Length),
            TypeError::UnknownReference(Ref::Token(vec![key!("nowhere")]))
        );
        assert_eq!(
            refused("active(token.gap)", Ty::Color),
            TypeError::ActiveNotRole(Ref::Active(Box::new(Ref::Token(vec![key!("gap")]))))
        );
        assert_eq!(typed("active(surface.foreground)", Ty::Color), Ty::Color);
        assert_eq!(
            typed("active(role.window.background)", Ty::Color),
            Ty::Color
        );
    }

    #[test]
    fn base_has_the_field_type() {
        assert_eq!(typed("add(base, 2px)", Ty::Length), Ty::Length);
        assert_eq!(typed("alpha(base, 0.5)", Ty::Color), Ty::Color);
        assert_eq!(
            refused("alpha(base, 0.5)", Ty::Length),
            TypeError::Argument {
                func: Func::Alpha,
                index: 0,
                expected: Ty::Color,
                found: Ty::Length
            }
        );
    }

    #[test]
    fn enum_descriptions_match_the_spec_lists() {
        assert_eq!(SCHEME.variants, Scheme::NAMES);
        assert_eq!(SURFACE_SET.variants, SurfaceSet::NAMES);
        // S4.4's `TextRoles` fields, in order.
        assert_eq!(
            TEXT_ROLE.variants,
            [
                "body",
                "body_emphasized",
                "caption",
                "caption_emphasized",
                "caption2",
                "footnote",
                "subheadline",
                "callout",
                "headline",
                "title1",
                "title2",
                "title3",
                "large_title",
                "display_large",
                "display_medium",
                "display_small",
                "label",
                "monospace",
                "numeric",
            ]
        );
        // S4.8's enums.
        assert_eq!(KDE_GROUP.variants, ["active", "disabled", "inactive"]);
        assert_eq!(
            KDE_ROLE_KIND.variants,
            ["foreground", "decoration", "background"]
        );
        assert_eq!(
            KDE_SCHEME_KEY.variants,
            [
                "background_normal",
                "background_alternate",
                "foreground_normal",
                "foreground_inactive",
                "foreground_active",
                "foreground_link",
                "foreground_visited",
                "foreground_negative",
                "foreground_neutral",
                "foreground_positive",
                "decoration_focus",
                "decoration_hover",
            ]
        );
        assert_eq!(
            KDE_WM_KEY.variants,
            [
                "active_background",
                "active_foreground",
                "inactive_background",
                "inactive_foreground",
            ]
        );
        // S2.2's function table.
        assert_eq!(
            TRACKING_DESIGN.variants,
            ["sf_pro", "sf_pro_rounded", "new_york"]
        );
        assert_eq!(NAME_INDEX_METHOD.variants, ["kirigami", "adwaita"]);
        // S1's `Sides`.
        assert_eq!(SIDE.variants, ["top", "right", "bottom", "left"]);
        assert!(UIKIT_PROMINENCE.variants.is_empty());
    }
}
