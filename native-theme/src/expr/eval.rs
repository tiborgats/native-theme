//! Evaluation (S2.2): a typed expression's value, its references answered by
//! an [`Env`].

use super::arith;
use super::ty::{Bound, Node, Ty, TypedExpr};
use super::{
    Func, KDE_EFFECT_FIELDS, Name, Ref, kde_accent_operands, kde_accent_wm_operands,
    kde_group_tokens, kde_selection_tokens, m3_role_operands, scheme_key, wm_key,
};
use crate::blocks::paint::{BlendMode, ResolvedPaint, ResolvedPaintKind, ResolvedPaintLayer};
use crate::color::Color;
use crate::platform::kde_accent_operands::{KdeAccentKeyInputs, KdeAccentScheme};
use crate::platform::kde_effects::KdeGroupEffects;
use crate::platform::kde_shade::KdeShadeRole;
use crate::platform::sf_tracking::TrackingDesign;
use crate::resolve::KdeLists;
use crate::roles::SurfaceSet;
use crate::selector::Scheme;
use crate::tokens::{
    KdeColorEffect, KdeContrastEffect, KdeGroup, KdeIntensityEffect, MaterialVariant,
};
use crate::toml::read::Named;
use crate::units::{Deg, FontSize, Ms, Px};

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

    /// The DPI points convert by (S9.4): the reader's, else the preset's
    /// `token.text.font_dpi`; `None` where neither is stated, which
    /// validation refuses.
    fn font_dpi(&self) -> Option<f32>;

    /// KDE's three scheme lists under the scheme in force (S9.1), which
    /// `kde_accent` and `kde_accent_wm` read themselves; each empty where
    /// no layer states it.
    fn kde_lists(&self) -> &KdeLists;
}

/// A platform function implemented to its source (S2.2), the only kind a
/// typed expression calls: typing refuses the others
/// ([`TypeError::NotImplemented`](super::ty::TypeError::NotImplemented)).
///
/// Each is implemented under `platform` by the task that needs it (the
/// M1a design's task 13 and the value phase).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Platform {
    /// `adw_standalone(c, scheme)`.
    AdwStandalone,
    /// `kcu_mix(a, b, bias)`.
    KcuMix,
    /// `kcu_overlay(under, over)`.
    KcuOverlay,
    /// `kcu_shade(c, luma, chroma)`.
    KcuShade,
    /// `kcu_tint(a, b, amount)`.
    KcuTint,
    /// `kde_accent(set, key, group)`, which reads the accent, its scheme
    /// inputs, KDE's lists and two `token.kde` options itself.
    KdeAccent,
    /// `kde_accent_wm(key)`, which reads the same of its own itself.
    KdeAccentWm,
    /// `kde_effects(c, background, group, role_kind)`, which reads the
    /// group's `token.kde` effects itself.
    KdeEffects,
    /// `kde_selection_inactive(selection, window, selection_background,
    /// role_kind)`, which reads `token.kde.inactive_change_selection_color`
    /// itself.
    KdeSelectionInactive,
    /// `kde_shade(c, role, contrast)`.
    KdeShade,
    /// `kirigami_lerp(a, b, balance)`.
    KirigamiLerp,
    /// `kirigami_tint(c, tint, alpha)`.
    KirigamiTint,
    /// `m3_role(role)`, which reads the seed, `token.material` and the
    /// scheme itself.
    M3Role,
    /// `sf_tracking(font_size, design)`, which reads the font DPI itself.
    SfTracking,
}

impl Platform {
    /// The implemented platform function `func` names; `None` for one not
    /// implemented yet and for the model's own arithmetic.
    pub(super) fn of(func: Func) -> Option<Platform> {
        match func {
            Func::AdwStandalone => Some(Platform::AdwStandalone),
            Func::KcuMix => Some(Platform::KcuMix),
            Func::KcuOverlay => Some(Platform::KcuOverlay),
            Func::KcuShade => Some(Platform::KcuShade),
            Func::KcuTint => Some(Platform::KcuTint),
            Func::KdeAccent => Some(Platform::KdeAccent),
            Func::KdeAccentWm => Some(Platform::KdeAccentWm),
            Func::KdeEffects => Some(Platform::KdeEffects),
            Func::KdeSelectionInactive => Some(Platform::KdeSelectionInactive),
            Func::KdeShade => Some(Platform::KdeShade),
            Func::KirigamiLerp => Some(Platform::KirigamiLerp),
            Func::KirigamiTint => Some(Platform::KirigamiTint),
            Func::M3Role => Some(Platform::M3Role),
            Func::SfTracking => Some(Platform::SfTracking),
            _ => None,
        }
    }

    /// The function's value over its typed arguments.
    fn eval(self, args: &[Node], env: &dyn Env) -> Val {
        let arg = |index: usize| args.get(index).map(|node| eval(node, env));
        let color_arg =
            |index: usize| channels(arg(index).as_ref().map_or_else(transparent, color));
        let number_arg = |index: usize| arg(index).as_ref().map_or(0.0, number);
        match self {
            Platform::AdwStandalone => {
                let (c, scheme) = (arg(0), arg(1));
                let scheme = match scheme {
                    Some(Val::Name(Name::Variant(name))) => {
                        <Scheme as crate::toml::read::Named>::from_name(name.as_str())
                    }
                    _ => None,
                };
                let c = c.as_ref().map_or_else(transparent, color);
                match scheme {
                    Some(scheme) => Val::Color(from_channels(
                        crate::platform::adw_standalone::adw_standalone(channels(c), scheme),
                    )),
                    None => Val::Color(c),
                }
            }
            // The KDE colour functions take a colour outside sRGB's gamut as
            // Qt's extended RGB and read Qt's invalid colour as transparent.
            Platform::KcuMix => Val::Color(from_channels(crate::platform::kcu_mix::kcu_mix(
                color_arg(0),
                color_arg(1),
                number_arg(2),
            ))),
            Platform::KcuOverlay => Val::Color(from_channels(
                crate::platform::kcu_overlay::kcu_overlay(color_arg(0), color_arg(1)),
            )),
            Platform::KcuShade => Val::Color(from_channels(crate::platform::kcu_shade::kcu_shade(
                color_arg(0),
                number_arg(1),
                number_arg(2),
            ))),
            Platform::KcuTint => Val::Color(from_channels(crate::platform::kcu_tint::kcu_tint(
                color_arg(0),
                color_arg(1),
                number_arg(2),
            ))),
            Platform::KirigamiTint => Val::Color(from_channels(
                crate::platform::kirigami_tint::kirigami_tint(
                    color_arg(0),
                    color_arg(1),
                    number_arg(2),
                ),
            )),
            Platform::KdeAccent => {
                Val::Color(kde_accent(arg(0), arg(1), arg(2), env).unwrap_or_else(transparent))
            }
            Platform::KdeAccentWm => {
                Val::Color(kde_accent_wm(arg(0), env).unwrap_or_else(transparent))
            }
            // A group or kind of no variant (validation refuses it) leaves the
            // colour as it is, as `kde_shade` does an unknown role.
            Platform::KdeEffects => match (named::<KdeGroup>(arg(2)), named(arg(3))) {
                (Some(group), Some(role_kind)) => {
                    let (effects, inactive_enabled) = kde_group_effects(group, env);
                    Val::Color(from_channels(crate::platform::kde_effects::kde_effects(
                        color_arg(0),
                        color_arg(1),
                        group,
                        role_kind,
                        &effects,
                        inactive_enabled,
                    )))
                }
                _ => Val::Color(from_channels(color_arg(0))),
            },
            Platform::KdeSelectionInactive => match named(arg(3)) {
                Some(role_kind) => {
                    let change = kde_selection_tokens()
                        .first()
                        .is_some_and(|token| env.get(token) == Val::Bool(true));
                    Val::Color(from_channels(
                        crate::platform::kde_selection_inactive::kde_selection_inactive(
                            color_arg(0),
                            color_arg(1),
                            color_arg(2),
                            role_kind,
                            change,
                        ),
                    ))
                }
                None => Val::Color(from_channels(color_arg(0))),
            },
            Platform::KdeShade => {
                let (c, role, contrast) = (arg(0), arg(1), arg(2));
                let role = match role {
                    Some(Val::Name(Name::Variant(name))) => {
                        <KdeShadeRole as crate::toml::read::Named>::from_name(name.as_str())
                    }
                    _ => None,
                };
                let c = c.as_ref().map_or_else(transparent, color);
                match role {
                    Some(role) => Val::Color(from_channels(crate::platform::kde_shade::kde_shade(
                        channels(c),
                        role,
                        contrast.as_ref().map_or(0.0, number),
                    ))),
                    None => Val::Color(c),
                }
            }
            Platform::KirigamiLerp => {
                let (a, b, balance) = (arg(0), arg(1), arg(2));
                // A colour outside sRGB's gamut enters as Qt's extended RGB;
                // Qt's invalid colour (a NaN channel, an alpha outside
                // `0..=1`, a NaN balance) has no value in the model:
                // transparent, as a non-finite result is zero (S2.2's
                // *Totality*).
                let lerped = crate::platform::kirigami_lerp::kirigami_lerp(
                    channels(a.as_ref().map_or_else(transparent, color)),
                    channels(b.as_ref().map_or_else(transparent, color)),
                    balance.as_ref().map_or(0.0, number),
                );
                Val::Color(lerped.map_or_else(transparent, from_channels))
            }
            Platform::M3Role => {
                Val::Color(m3_role(arg(0), env).map_or_else(transparent, from_channels))
            }
            Platform::SfTracking => {
                let (size, design) = (arg(0), arg(1));
                let design = match design {
                    Some(Val::Name(Name::Variant(name))) => {
                        <TrackingDesign as crate::toml::read::Named>::from_name(name.as_str())
                    }
                    _ => None,
                };
                // Without a DPI (validation requires one, S9.4) the size has
                // no points: zero, as a non-finite result is (S2.2's
                // *Totality*).
                let dpi = env.font_dpi().unwrap_or(f32::NAN);
                let size = size.as_ref().map_or(0.0, number);
                Val::Length(Px(design.map_or(0.0, |design| {
                    crate::platform::sf_tracking::sf_tracking(size, design, dpi)
                })))
            }
        }
    }
}

/// `m3_role`'s value for its `role` operand, from the operands it reads
/// itself ([`m3_role_operands`]): `None`, which evaluates to transparent as
/// S2.2's *Totality* has it, for a role, variant or version the port does
/// not implement and for an operand of no value of its type (validation
/// refuses an absent one, S12).
fn m3_role(role: Option<Val>, env: &dyn Env) -> Option<[f32; 4]> {
    let Some(Val::Name(Name::Key(role))) = role else {
        return None;
    };
    let operand = |index: usize| {
        m3_role_operands()
            .get(index)
            .map(|reference| env.get(reference))
    };
    let Some(Val::Color(accent)) = operand(0) else {
        return None;
    };
    let variant: MaterialVariant = named(operand(1))?;
    let Some(Val::Name(Name::Key(spec_version))) = operand(2) else {
        return None;
    };
    let Some(Val::Number(contrast_level)) = operand(3) else {
        return None;
    };
    let scheme: Scheme = named(operand(4))?;
    crate::platform::m3_role::m3_role(
        role.as_str(),
        channels(accent),
        variant,
        spec_version.as_str(),
        contrast_level,
        scheme,
    )
}

/// `kde_accent`'s value for its arguments, from the operands it reads
/// itself ([`kde_accent_operands`], in that order): `None`, which evaluates
/// to transparent as S2.2's *Totality* has it, for an argument of no
/// variant, a set outside KDE's seven and the group `disabled` (validation
/// refuses each, S12).
fn kde_accent(
    set: Option<Val>,
    key: Option<Val>,
    group: Option<Val>,
    env: &dyn Env,
) -> Option<Color> {
    let set = SurfaceSet::from_name(variant(set.as_ref())?)?;
    let key = scheme_key(variant(key.as_ref())?)?;
    let group: KdeGroup = named(group)?;
    let operands = kde_accent_operands(set, key);
    let input = |index: usize| operand_color(operands, index, env);
    let inputs = KdeAccentKeyInputs {
        value: input(1),
        inactive_value: input(2),
        background: input(3),
        window_value: input(4),
        window_background: input(5),
        view_background: input(6),
    };
    let lists = env.kde_lists();
    let scheme = KdeAccentScheme {
        accent: input(0),
        groups: &lists.groups,
        inactive_groups: &lists.inactive_groups,
        unset_keys: &lists.unset_keys,
        tint_factor: tint_factor(operands.get(10), env),
        titlebar_is_accent_colored: operands
            .get(11)
            .is_some_and(|token| env.get(token) == Val::Bool(true)),
    };
    scheme.kde_accent(set, key, group, &inputs)
}

/// `kde_accent_wm`'s value for its key, from the operands it reads itself
/// ([`kde_accent_wm_operands`], in that order): `None`, which evaluates to
/// transparent, for a key of no variant.
fn kde_accent_wm(key: Option<Val>, env: &dyn Env) -> Option<Color> {
    let key = wm_key(variant(key.as_ref())?)?;
    let operands = kde_accent_wm_operands(key);
    let input = |index: usize| operand_color(operands, index, env);
    let lists = env.kde_lists();
    let scheme = KdeAccentScheme {
        accent: input(0),
        groups: &lists.groups,
        inactive_groups: &lists.inactive_groups,
        unset_keys: &lists.unset_keys,
        tint_factor: tint_factor(operands.get(5), env),
        titlebar_is_accent_colored: operands
            .get(6)
            .is_some_and(|token| env.get(token) == Val::Bool(true)),
    };
    Some(scheme.kde_accent_wm(key, input(1), input(2)))
}

/// The colour operand `operands[index]` reads; transparent for one of no
/// colour (validation refuses an absent one, S12).
fn operand_color(operands: &[Ref], index: usize, env: &dyn Env) -> Color {
    operands
        .get(index)
        .map_or_else(transparent, |reference| color(&env.get(reference)))
}

/// `token.kde.tint_factor`: `None` where the scheme does not tint, the
/// token's `None`, which reads as no number.
fn tint_factor(token: Option<&Ref>, env: &dyn Env) -> Option<f32> {
    match token.map(|token| env.get(token)) {
        Some(Val::Number(factor)) => Some(factor),
        _ => None,
    }
}

/// The name of a variant operand.
fn variant(value: Option<&Val>) -> Option<&str> {
    match value {
        Some(Val::Name(Name::Variant(name))) => Some(name.as_str()),
        _ => None,
    }
}

/// The variant an enum operand names; `None` for another value.
fn named<T: Named>(value: Option<Val>) -> Option<T> {
    match value {
        Some(Val::Name(Name::Variant(name))) => T::from_name(name.as_str()),
        _ => None,
    }
}

/// `group`'s effects and whether they apply, from the tokens
/// `kde_effects` reads itself ([`kde_group_tokens`]): a token that reads as
/// no value of its type reads as zero, its effect as none, its colour as
/// transparent and the switch as off, so evaluation stays total (S2.2's
/// *Totality*; validation refuses an absent one, S12). The active group
/// reads none.
fn kde_group_effects(group: KdeGroup, env: &dyn Env) -> (KdeGroupEffects, bool) {
    let tokens = kde_group_tokens(group);
    let token = |index: usize| tokens.get(index).map(|token| env.get(token));
    let amount = |index: usize| token(index).as_ref().map_or(0.0, number);
    let effects = KdeGroupEffects {
        intensity: named(token(0)).unwrap_or(KdeIntensityEffect::None),
        intensity_amount: amount(1),
        color: named(token(2)).unwrap_or(KdeColorEffect::None),
        color_amount: amount(3),
        color_color: channels(token(4).as_ref().map_or_else(transparent, color)),
        contrast: named(token(5)).unwrap_or(KdeContrastEffect::None),
        contrast_amount: amount(6),
    };
    let enabled = token(KDE_EFFECT_FIELDS.len()) == Some(Val::Bool(true));
    (effects, enabled)
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
        Node::RoundPt(x) => {
            let x = number(&eval(x, env));
            let dpi = env.font_dpi().unwrap_or(f32::NAN);
            Val::FontSize(Px(arith::round_pt(x, dpi)))
        }
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

/// A colour's channels, red, green, blue and alpha, as the platform functions
/// take them.
fn channels(c: Color) -> [f32; 4] {
    [c.r, c.g, c.b, c.a]
}

/// The colour of a platform function's channels.
fn from_channels([r, g, b, a]: [f32; 4]) -> Color {
    Color { r, g, b, a }
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

/// `value` limited by `bound` where it is a number, a non-finite number or
/// channel zero (S2.2's *Totality*).
pub(crate) fn bounded(value: Val, bound: Bound) -> Val {
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
    use crate::expr::ty::{FieldType, TypeEnv};
    use crate::expr::{ElementDatum, Expr};
    use crate::key::Key;

    /// Synthetic values for the tests, none a platform's: each reference
    /// answers what `get` matches, the types from `reference`.
    struct Fixture;

    impl TypeEnv for Fixture {
        fn reference(&self, reference: &Ref) -> Option<Ty> {
            match reference.to_string().as_str() {
                "token.gap" | "part.min_width" | "widget.frame.width" => Some(Ty::Length),
                "token.speed" | "token.nan" | "values.weight" => Some(Ty::Number),
                "token.duration" => Some(Ty::Duration),
                "token.label_size" | "token.title_size" => Some(Ty::FontSize),
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
                Ref::Scheme => Val::Name(Name::Variant(Key::new("dark").unwrap())),
                _ => match reference.to_string().as_str() {
                    "token.gap" | "widget.frame.width" => Val::Length(Px(6.0)),
                    "token.speed" => Val::Number(0.5),
                    "token.nan" => Val::Number(f32::NAN),
                    "values.weight" => Val::Number(2.5),
                    "token.duration" => Val::Duration(Ms(200.0)),
                    "token.label_size" => Val::FontSize(Px(16.0)),
                    "token.title_size" => Val::FontSize(Px(17.0)),
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

        /// The synthetic two px per point's DPI.
        fn font_dpi(&self) -> Option<f32> {
            Some(144.0)
        }

        fn kde_lists(&self) -> &crate::resolve::KdeLists {
            &crate::resolve::NO_KDE_LISTS
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
    fn evaluates_kde_colour_functions() {
        // Breeze Light's frame colour: its window background mixed toward its
        // text by 0.2, in Qt's 16-bit storage (`tests/vectors/kcu_mix.tsv`).
        assert_eq!(
            value("kcu_mix(#eff0f1, #232629, 0.2)", Ty::Color),
            Val::Color(Color {
                r: 50937.0 / 65535.0,
                g: 51297.0 / 65535.0,
                b: 51657.0 / 65535.0,
                a: 1.0
            })
        );
        // Every other call is its function over the operands (synthetic
        // colours and amounts).
        let [a, b, translucent] =
            ["#eff0f1", "#3daee9", "#3daee980"].map(|c| channels(Color::parse(c).unwrap()));
        for (text, expected) in [
            (
                "kcu_tint(#eff0f1, #3daee9, 0.4)",
                crate::platform::kcu_tint::kcu_tint(a, b, 0.4),
            ),
            (
                "kcu_shade(#eff0f1, -0.1, 0.2)",
                crate::platform::kcu_shade::kcu_shade(a, -0.1, 0.2),
            ),
            (
                "kcu_overlay(#eff0f1, #3daee980)",
                crate::platform::kcu_overlay::kcu_overlay(a, translucent),
            ),
            (
                "kirigami_tint(#eff0f1, #3daee980, 0.5)",
                crate::platform::kirigami_tint::kirigami_tint(a, translucent, 0.5),
            ),
        ] {
            assert_eq!(
                value(text, Ty::Color),
                Val::Color(from_channels(expected)),
                "{text}"
            );
        }
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
        // A platform function: Kirigami's interpolation of two greys, as
        // Qt's 16-bit `QColor` gives it (`platform::kirigami_lerp`).
        let grey = 42148.0 / 65535.0;
        assert_eq!(
            value("kirigami_lerp(#808080, #c8c8c8, 0.5)", Ty::Color),
            Val::Color(Color {
                r: grey,
                g: grey,
                b: grey,
                a: 1.0
            })
        );
        // An extended colour (Display P3's red lies outside sRGB) enters as
        // Qt's extended RGB, its channels clamped: drawn, not transparent.
        assert_eq!(
            value(
                "kirigami_lerp(display_p3(1, 0, 0), display_p3(1, 0, 0), 0)",
                Ty::Color
            ),
            Val::Color(Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0
            })
        );
        // KColorScheme's dark shade of Breeze Light's window background at
        // Breeze's contrast 4 (`platform::kde_shade`).
        assert_eq!(
            value("kde_shade(#eff0f1, dark, 4)", Ty::Color),
            Val::Color(Color {
                r: 34977.0 / 65535.0,
                g: 36393.0 / 65535.0,
                b: 37752.0 / 65535.0,
                a: 1.0
            })
        );
        // libadwaita's standalone colour of its blue accent: unclamped, its
        // blue channel above 1 in the dark scheme, which the fixture's
        // `scheme` names (`platform::adw_standalone`).
        let blue = Color::parse("#3584e4").unwrap();
        for (text, scheme) in [
            ("adw_standalone(#3584e4, light)", Scheme::Light),
            ("adw_standalone(#3584e4, dark)", Scheme::Dark),
            ("adw_standalone(#3584e4, scheme)", Scheme::Dark),
        ] {
            assert_eq!(
                value(text, Ty::Color),
                Val::Color(from_channels(
                    crate::platform::adw_standalone::adw_standalone(channels(blue), scheme)
                )),
                "{text}"
            );
        }
        let Val::Color(dark) = value("adw_standalone(#3584e4, dark)", Ty::Color) else {
            panic!("a colour");
        };
        assert!(dark.b > 1.0, "{dark:?}");
        // SF Pro's tracking at 17 pt, 34 px at the fixture's DPI, which the
        // function reads itself to find the points (`platform::sf_tracking`).
        assert_eq!(
            value("sf_tracking(17pt, sf_pro)", Ty::Length),
            Val::Length(Px(crate::platform::sf_tracking::sf_tracking(
                34.0,
                TrackingDesign::SfPro,
                144.0
            )))
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
        // 17px at 144 dpi is 8.5 points, rounded to 9, back to 18px.
        assert_eq!(
            value("round_pt(token.title_size)", Ty::FontSize),
            Val::FontSize(Px(18.0))
        );
        assert_eq!(
            value("mul(round_pt(token.label_size), 1.5)", Ty::FontSize),
            Val::FontSize(Px(24.0))
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
