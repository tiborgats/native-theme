//! S8's queries over a resolved theme: a part's style with the scope its
//! children receive, a widget's values, the tokens and a role in a scope,
//! the size classes, the data a part reads and an element's kept flags.

use super::matching::{Matched, declarations, kept};
use super::mirror::Mirror;
use super::{Container, Element, Scope, ScopeEntry};
use crate::next::blocks::part_style::{ResolvedBinding, ResolvedPartStyle, RoleRef};
use crate::next::blocks::text::LanguageTag;
use crate::next::color::Color;
use crate::next::declare::ScalarUnit;
use crate::next::expr::Ref;
use crate::next::key::Key;
use crate::next::resolve::ResolvedTheme;
use crate::next::resolve::compose::Target;
use crate::next::resolve::evaluate::Resolve;
use crate::next::selector::{
    Density, Direction, PartRef, Selector, SizeClass, SizeClasses, StateFlags, VariantRef,
    WidgetOrPart,
};
use crate::next::tokens::{ResolvedTokens, Tokens};
use crate::next::toml::read::Named;
use crate::next::units::{Px, Scalar, Size};
use crate::next::widgets::{PartPatch, TypedPart, WidgetKind};

/// R2.7's answer for an element: its style, or why there is none to draw;
/// not an error (S8).
#[derive(Clone, Debug, PartialEq)]
pub enum Styled<T> {
    /// The style.
    Style(T),
    /// The part is not drawn: its style resolved `Absent` (S6.3). Its
    /// children take the parent's scope.
    NotDrawn,
    /// Another widget, or a part of one, draws the part: the connector builds
    /// an element of it in the host's context, in the scope the part was
    /// queried in, with these variants and the host element's states (S8).
    DrawnAs {
        /// The widget or part that draws it.
        target: WidgetOrPart,
        /// Its variants.
        variants: Vec<VariantRef>,
    },
    /// The platform has no such widget, size, variant or part (R2.7). Its
    /// children take the parent's scope.
    NotAvailable(Unavailable),
}

/// What the platform does not have (S8).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Unavailable {
    /// The widget: the preset declares it not available, or the element is
    /// another widget's.
    Widget,
    /// The size: one the element does not take.
    Size,
    /// A variant: one the application asked for that the widget does not
    /// declare, none of the forms the widget requires, or two of one
    /// exclusive group.
    Variant,
    /// The part: one the widget does not have.
    Part,
}

/// What a window's root scope starts from (S8): what the connector measures
/// and the application sets for the whole window.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct RootInputs {
    /// The density; `None`: the default.
    pub density: Option<Density>,
    /// The window's size classes.
    pub size_classes: SizeClasses,
    /// The direction.
    pub direction: Direction,
    /// The language, for icon lookup.
    pub language: Option<LanguageTag>,
    /// The window's size.
    pub window: Size,
    /// The screen's work area.
    pub screen: Size,
    /// The display's corner radii, where the platform reports them, from
    /// the top left clockwise (S4.6).
    pub display_corner_radius: Option<[Px; 4]>,
}

impl RootInputs {
    /// The inputs of a window of size `window` on a screen whose work area
    /// is `screen`, in `direction`: no density, no size classes, no
    /// language, no display radii.
    #[must_use]
    pub fn new(window: Size, screen: Size, direction: Direction) -> RootInputs {
        RootInputs {
            density: None,
            size_classes: SizeClasses::default(),
            direction,
            language: None,
            window,
            screen,
            display_corner_radius: None,
        }
    }
}

/// A typed widget, whose values `ResolvedTheme::values` answers (S8);
/// generated for each widget's values.
pub trait Widget {
    /// The widget's resolved values.
    type Values;

    /// The widget.
    #[doc(hidden)]
    const KIND: WidgetKind;

    /// The values for `element`, their kept rules those `applies` accepts,
    /// mirrored for `direction`.
    #[doc(hidden)]
    fn evaluate(
        theme: &ResolvedTheme,
        element: &Element<'_>,
        applies: &dyn Fn(&Selector) -> bool,
        direction: Direction,
    ) -> Option<Self::Values>;
}

/// Every size class, in `SizeClass`'s order (S7.3).
const SIZE_CLASSES: [SizeClass; 6] = [
    SizeClass::Compact,
    SizeClass::Medium,
    SizeClass::Expanded,
    SizeClass::Large,
    SizeClass::ExtraLarge,
    SizeClass::Regular,
];

/// Whether `part`'s catalogue entry lists `corners`, so it is the container
/// of the parts it encloses (S2.4).
fn lists_corners(part: TypedPart) -> bool {
    part.info()
        .is_some_and(|info| info.fields.iter().any(|field| field.path == "corners"))
}

/// The path of a role a `RoleRef` names, as a reference spells it.
fn keys(path: &str) -> Option<Vec<Key>> {
    path.split('.')
        .map(|segment| Key::new(segment).ok())
        .collect()
}

impl ResolvedTheme {
    /// A window's root scope (S8): no part entered, the root's inputs.
    #[must_use]
    pub fn root_scope(&self, root: &RootInputs) -> Scope {
        let mut scope = Scope::root(root.window, root.screen, root.direction);
        scope.density = root.density.clone();
        scope.size_classes = root.size_classes;
        scope.language = root.language.clone();
        let display = match root.display_corner_radius {
            Some(radii) => radii.map(Some),
            None => self.display_tokens(&scope),
        };
        scope.container = Container::Display(display);
        scope
    }

    /// The `display` tokens' corner radii, from the top left clockwise
    /// (S4.6); `None` at a corner they do not state, and at every corner
    /// where the group is not declared in px.
    fn display_tokens(&self, scope: &Scope) -> [Option<Px>; 4] {
        const CORNERS: [&str; 4] = [
            "corner_radius_top_left",
            "corner_radius_top_right",
            "corner_radius_bottom_right",
            "corner_radius_bottom_left",
        ];
        let in_px =
            self.declarations.params.iter().any(|(group, params)| {
                group.as_str() == "display" && params.unit == ScalarUnit::Px
            });
        if !in_px {
            return [None; 4];
        }
        CORNERS.map(
            |corner| match self.evaluate_param(scope, "display", corner)? {
                Scalar::Px(px) => Some(px),
                Scalar::Number(px) => Some(Px(px)),
                Scalar::Ms(_) | Scalar::Deg(_) | Scalar::Fraction(_) => None,
            },
        )
    }

    /// The style of `element`'s part and the scope its children receive
    /// (S8): its widget's declarations checked, its part's kept rules
    /// matched against it, its expressions evaluated, mirrored for its
    /// direction. A part with no style to draw (a field that cannot be
    /// absent left without a value, which validation rules out) is
    /// `NotDrawn`.
    #[must_use]
    pub fn style(&self, element: &Element<'_>) -> Styled<(ResolvedPartStyle, Scope)> {
        let matched = match Matched::new(self, element) {
            Ok(matched) => matched,
            Err(unavailable) => return Styled::NotAvailable(unavailable),
        };
        let applies = |selector: &Selector| matched.matches(selector);
        let shown = self
            .part_steps(matched.part)
            .iter()
            .rev()
            .find(|step| step.selector.as_ref().is_none_or(applies))
            .map(|step| &step.patch);
        match shown {
            Some(PartPatch::Absent) => return Styled::NotDrawn,
            Some(PartPatch::DrawnAs { target, variants }) => {
                return Styled::DrawnAs {
                    target: target.clone(),
                    variants: variants.clone(),
                };
            }
            Some(PartPatch::Style(_)) | None => {}
        }
        let Some(mut style) = self.evaluate_part(matched.part, element, &applies).value else {
            return Styled::NotDrawn;
        };
        if matched.direction == Direction::Rtl {
            style.mirror();
        }
        let mut entry = ScopeEntry {
            part: element.part.clone(),
            variants: matched.variants.clone(),
            states: matched.states.clone(),
            size: matched.size,
            surface: None,
            roles: Vec::new(),
        };
        for binding in &style.bindings {
            match binding {
                ResolvedBinding::Surface(set) => entry.surface = Some(*set),
                ResolvedBinding::Role { role, value } => {
                    entry.roles.retain(|(held, _)| held != role);
                    entry.roles.push((role.clone(), *value));
                }
            }
        }
        let mut scope = element.scope.entered(entry);
        if let Some(density) = &element.density {
            scope.density = Some(density.clone());
        }
        if let Some(direction) = element.direction {
            scope.direction = direction;
        }
        if let Some(size_classes) = element.size_classes {
            scope.size_classes = size_classes;
        }
        if let Some(language) = &element.language {
            scope.language = Some(language.clone());
        }
        if lists_corners(matched.part) {
            scope.container = Container::Part {
                part: element.part.clone(),
                corners: style.corners.clone(),
            };
        }
        Styled::Style((style, scope))
    }

    /// The values of `element`'s widget (S8): `NotAvailable(Widget)` for
    /// another widget's element; their kept rules matched against it,
    /// mirrored for its direction. Values that cannot be resolved (a field
    /// that cannot be absent left without a value, which validation rules
    /// out) are `NotDrawn`.
    #[must_use]
    pub fn values<W: Widget>(&self, element: &Element<'_>) -> Styled<W::Values> {
        let PartRef::Typed(part) = &element.part else {
            return Styled::NotAvailable(Unavailable::Widget);
        };
        if part.widget() != W::KIND {
            return Styled::NotAvailable(Unavailable::Widget);
        }
        let matched = match Matched::new(self, element) {
            Ok(matched) => matched,
            Err(unavailable) => return Styled::NotAvailable(unavailable),
        };
        let applies = |selector: &Selector| matched.matches(selector);
        match W::evaluate(self, element, &applies, matched.direction) {
            Some(values) => Styled::Style(values),
            None => Styled::NotDrawn,
        }
    }

    /// The values of `widget` for `element`, their kept rules those
    /// `applies` accepts, mirrored for `direction` (S2.4, S8).
    pub(crate) fn widget_values<V>(
        &self,
        widget: WidgetKind,
        element: &Element<'_>,
        applies: &dyn Fn(&Selector) -> bool,
        direction: Direction,
    ) -> Option<V::Resolved>
    where
        V: Resolve,
        V::Resolved: Mirror,
    {
        let mut values = self.evaluate_values::<V>(widget, element, applies).value?;
        if direction == Direction::Rtl {
            values.mirror();
        }
        Some(values)
    }

    /// The tokens read in `scope` (S8), what no part carries: their
    /// expressions' roles through its bindings, mirrored for its direction.
    /// `None` only where a token that cannot be absent has no value, which
    /// validation rules out.
    #[must_use]
    pub fn tokens(&self, scope: &Scope) -> Option<ResolvedTokens> {
        let mut tokens = self
            .evaluate_target::<Tokens>(scope, Target::Tokens)
            .value?;
        if scope.direction == Direction::Rtl {
            tokens.focus.mirror();
        }
        Some(tokens)
    }

    /// The colour `role` names in `scope` (S3.3): the nearest binding of it
    /// in scope, else the colour roles' value; `None`: not drawn.
    #[must_use]
    pub fn role(&self, scope: &Scope, role: RoleRef) -> Option<Color> {
        let reference = match role {
            RoleRef::Surface(role) => Ref::Surface(keys(role.path())?),
            RoleRef::Set(set, role) => Ref::Role {
                set,
                path: keys(role.path())?,
            },
            RoleRef::Global(role) => Ref::GlobalRole(keys(&role.path())?),
            RoleRef::Keyed(key) => Ref::KeyedRole(key),
        };
        self.evaluate_role(scope, &reference).value
    }

    /// A window's size classes from the thresholds (R16, S1): on each axis
    /// the class with the largest bound not above the window's size there,
    /// the first of two equal; `None` on an axis no class bounds at or
    /// below it.
    #[must_use]
    pub fn size_classes(&self, window: Size) -> SizeClasses {
        let scope = Scope::root(window, window, Direction::Ltr);
        let pick = |size: Px, axis: &str| -> Option<SizeClass> {
            let mut best: Option<(SizeClass, Px)> = None;
            for class in SIZE_CLASSES {
                let Some(bound) = self.evaluate_threshold(&scope, class.name(), axis) else {
                    continue;
                };
                if bound.0 <= size.0 && best.is_none_or(|(_, Px(held))| bound.0 > held) {
                    best = Some((class, bound));
                }
            }
            best.map(|(class, _)| class)
        };
        SizeClasses {
            horizontal: pick(window.width, "horizontal"),
            vertical: pick(window.height, "vertical"),
        }
    }

    /// `element`'s flags after the drops (S8, S10): a keyed flag its widget
    /// does not declare left out, and of each exclusive group the one of
    /// highest precedence kept; what a transition's `from` is matched
    /// against (S2.7).
    #[must_use]
    pub fn kept_states(&self, element: &Element<'_>) -> StateFlags {
        match &element.part {
            PartRef::Typed(part) => kept(&declarations(self, part.widget()), &element.states),
            PartRef::Component { .. } => element.states.clone(),
        }
    }
}
