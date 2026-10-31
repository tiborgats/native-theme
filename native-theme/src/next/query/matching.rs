//! S7's and S8's matching: an element read as the query reads it (its
//! widget's declarations in force, the variants it holds, its flags after
//! the drops, its size, density, direction and size classes), and a part's
//! rule matched against it.

use super::{Element, ScopeEntry, Sibling, Unavailable};
use crate::next::declare::WidgetDecl;
use crate::next::key::Key;
use crate::next::registry;
use crate::next::resolve::ResolvedTheme;
use crate::next::selector::{
    ContextMatch, ContextScope, ControlSize, Density, Direction, Parity, PartRef, PositionMatch,
    Selector, SetMatch, SiblingMatch, SiblingScope, SizeClasses, StateFlag, StateFlags, VariantRef,
    WidgetOrPart, WidgetRef,
};
use crate::next::toml::read::Named;
use crate::next::widgets::{TypedPart, WidgetKind};

/// How many of `entries`, the innermost last, are the entries of the parts
/// of `part`'s own element that enclose it (S8): none for a part that starts
/// an element (a widget's root, an *(element)* part), else the innermost
/// entries whose parts enclose it within its element, through the one that
/// starts the element. Where that element was not drawn, so its children
/// take its parent's scope (S8), the host's entries are not its own.
pub(crate) fn own_entries(part: &PartRef, entries: &[ScopeEntry]) -> usize {
    let PartRef::Typed(typed) = part else {
        return 0;
    };
    if typed.starts_element() {
        return 0;
    }
    let own = within_element(*typed);
    let mut count = 0_usize;
    for entry in entries.iter().rev() {
        let PartRef::Typed(held) = &entry.part else {
            break;
        };
        if !own.contains(held) {
            break;
        }
        count = count.saturating_add(1);
        if held.starts_element() {
            break;
        }
    }
    count
}

/// The parts enclosing `part` within its own element, innermost first: the
/// part itself through the part that starts its element.
fn within_element(part: TypedPart) -> Vec<TypedPart> {
    let mut enclosing = part.enclosing();
    if let Some(at) = enclosing.iter().position(|held| held.starts_element()) {
        enclosing.truncate(at.saturating_add(1));
    }
    enclosing
}

/// The entries of `outer` (outermost first) of the instance of `head`'s
/// widget that encloses `head`'s element: the last ones, each a part
/// enclosing the one inside it, ending at the first that is not, so an
/// outer instance of the same widget (a list inside a list's view) is never
/// read as the element's own (S2.11).
pub(crate) fn instance_entries(head: TypedPart, outer: &[ScopeEntry]) -> &[ScopeEntry] {
    let enclosing: Vec<TypedPart> = head.enclosing().into_iter().skip(1).collect();
    let parts: Vec<Option<TypedPart>> = outer
        .iter()
        .map(|entry| match &entry.part {
            PartRef::Typed(part) => Some(*part),
            PartRef::Component { .. } => None,
        })
        .collect();
    let run = nested_run(&enclosing, &parts);
    outer
        .get(outer.len().saturating_sub(run)..)
        .unwrap_or_default()
}

/// How many of `outer`'s last items are, innermost last, items of
/// `enclosing` (innermost first), each further out in it than the one
/// inside it.
fn nested_run<T: PartialEq>(enclosing: &[T], outer: &[Option<T>]) -> usize {
    let mut inside: Option<usize> = None;
    let mut count = 0_usize;
    for item in outer.iter().rev() {
        let Some(at) = item
            .as_ref()
            .and_then(|item| enclosing.iter().position(|held| held == item))
        else {
            break;
        };
        if inside.is_some_and(|inside| at <= inside) {
            break;
        }
        inside = Some(at);
        count = count.saturating_add(1);
    }
    count
}

/// The variants of the elements of `part`'s widget that enclose its
/// element, read from the entries of its widget's instance
/// ([`instance_entries`]) (S2.11: an element part's query sees its
/// widget's variants as well as its own); none for a widget's root.
fn widget_variants(part: TypedPart, outer: &[ScopeEntry]) -> Vec<VariantRef> {
    let mut out = Vec::new();
    for entry in instance_entries(part.element(), outer).iter().rev() {
        for variant in &entry.variants {
            if !out.contains(variant) {
                out.push(variant.clone());
            }
        }
    }
    out
}

/// The widget a part belongs to.
fn widget_ref(part: &PartRef) -> WidgetRef {
    match part {
        PartRef::Typed(part) => WidgetRef::Typed(part.widget()),
        PartRef::Component { key, .. } => WidgetRef::Component(key.clone()),
    }
}

/// Whether `part` is `target`: any where it names none (R25.9).
fn is_target(target: Option<&WidgetOrPart>, part: &PartRef) -> bool {
    match target {
        None => true,
        Some(WidgetOrPart::Widget(widget)) => *widget == widget_ref(part),
        Some(WidgetOrPart::Part(held)) => held == part,
    }
}

/// Whether a set match holds over what `holds` holds: every one of
/// `all_of`, none of `none_of` (S7).
fn set_holds<T>(set: &SetMatch<T>, holds: impl Fn(&T) -> bool) -> bool {
    set.all_of.iter().all(&holds) && !set.none_of.iter().any(&holds)
}

/// A set match over a single value, or none.
fn one_holds<T: PartialEq>(set: &SetMatch<T>, value: Option<&T>) -> bool {
    set_holds(set, |held| Some(held) == value)
}

/// A variant's name as the catalogue's content-variant pairs write it:
/// typed in snake_case, keyed with its `@`.
fn variant_name(variant: &VariantRef) -> String {
    match variant {
        VariantRef::Typed(variant) => variant.name().to_owned(),
        VariantRef::Keyed(key) => format!("@{key}"),
    }
}

/// `states` as a widget's declarations keep them (S8, S10): a keyed flag
/// the widget does not declare selects nothing, and of each exclusive
/// group only the held flag of highest precedence, its first, stays.
pub(crate) fn kept(decl: &WidgetDecl, states: &StateFlags) -> StateFlags {
    let declared = |flag: &StateFlag| match flag {
        StateFlag::Widget(_) => decl.flags.iter().flatten().any(|held| held == flag),
        _ => true,
    };
    let mut out: StateFlags = states
        .iter()
        .filter(|flag| declared(flag))
        .cloned()
        .collect();
    for group in decl.exclusive.iter().flatten() {
        let mut held = group.iter().filter(|flag| out.contains(flag));
        if held.next().is_some() {
            let dropped: Vec<StateFlag> = held.cloned().collect();
            for flag in &dropped {
                out.remove(flag);
            }
        }
    }
    out
}

/// The latest flag of a recency set among `states`, the kept flags (S7.2):
/// of its members `order` lists, the one it lists last (a flag listed twice
/// by its last place); where it lists none, the first in the set's stated
/// order.
fn latest<'s>(
    members: &'s [StateFlag],
    states: &StateFlags,
    order: &[StateFlag],
) -> Option<&'s StateFlag> {
    let kept = members.iter().filter(|flag| states.contains(flag));
    let ordered = kept
        .clone()
        .filter_map(|flag| {
            order
                .iter()
                .rposition(|held| held == flag)
                .map(|at| (at, flag))
        })
        .max_by_key(|(at, _)| *at)
        .map(|(_, flag)| flag);
    ordered.or_else(|| kept.clone().next())
}

/// The theme's declarations in force for `widget` (S10): its own section,
/// a flavour's keys replacing its own under that flavour.
pub(crate) fn declarations(theme: &ResolvedTheme, widget: WidgetKind) -> WidgetDecl {
    match &theme.flavor {
        Some(flavor) => theme.declarations.in_flavor(widget, flavor),
        None => theme.declarations.in_default_flavor(widget),
    }
}

/// An element as the query reads it (S8).
pub(crate) struct Matched<'e> {
    theme: &'e ResolvedTheme,
    element: &'e Element<'e>,
    /// The element's part.
    pub(crate) part: TypedPart,
    decl: WidgetDecl,
    /// The variants it holds that the query keeps: every one the widget
    /// declares.
    pub(crate) variants: Vec<VariantRef>,
    /// The variants of its widget's elements that enclose it, which its
    /// rules match as well as its own (S2.11).
    inherited: Vec<VariantRef>,
    /// Its flags after the drops.
    pub(crate) states: StateFlags,
    /// Its control size: its own, else the declared default.
    pub(crate) size: Option<ControlSize>,
    density: Option<Density>,
    /// Its direction: its own, else its scope's.
    pub(crate) direction: Direction,
    size_classes: SizeClasses,
    /// The scope's entries outside the element's own parts, outermost first.
    outer: &'e [ScopeEntry],
}

impl<'e> Matched<'e> {
    /// `element` read for `theme`, or why the widget, its size or its
    /// variants are not available (S8, S10).
    pub(crate) fn new(
        theme: &'e ResolvedTheme,
        element: &'e Element<'e>,
    ) -> Result<Matched<'e>, Unavailable> {
        let PartRef::Typed(part) = element.part else {
            return Err(Unavailable::Widget);
        };
        let widget = part.widget();
        let decl = declarations(theme, widget);
        if decl.available == Some(false) {
            return Err(Unavailable::Widget);
        }
        let entries = &element.scope.entries;
        let own = own_entries(&element.part, entries);
        let outer = entries
            .get(..entries.len().saturating_sub(own))
            .unwrap_or_default();
        let declared = decl.variants.as_deref().unwrap_or_default();
        let mut variants = Vec::new();
        for variant in element.variants {
            if declared.contains(variant) {
                if !variants.contains(variant) {
                    variants.push(variant.clone());
                }
            } else if !set_by_connector(&decl, part, variant) {
                return Err(Unavailable::Variant);
            }
        }
        let required = decl.requires_variants.as_deref().unwrap_or_default();
        if !required.is_empty() && !required.iter().any(|held| variants.contains(held)) {
            return Err(Unavailable::Variant);
        }
        let two_of_a_group = decl.exclusive_variants.iter().flatten().any(|group| {
            group
                .iter()
                .filter(|variant| variants.contains(variant))
                .nth(1)
                .is_some()
        });
        if two_of_a_group {
            return Err(Unavailable::Variant);
        }
        let size = match element.size {
            Some(size) => {
                if !takes(&decl, &variants, outer).is_some_and(|sizes| sizes.contains(&size)) {
                    return Err(Unavailable::Size);
                }
                Some(size)
            }
            None => decl
                .default_size_by_variant
                .iter()
                .flatten()
                .find(|by| variants.contains(&by.variant))
                .map(|by| by.size)
                .or(decl.default_size),
        };
        let scope = element.scope;
        let density = element
            .density
            .clone()
            .or_else(|| scope.density.clone())
            .filter(|density| decl.densities.iter().flatten().any(|held| held == density));
        Ok(Matched {
            theme,
            element,
            part,
            states: kept(&decl, &element.states),
            decl,
            variants,
            size,
            density,
            direction: element.direction.unwrap_or(scope.direction),
            size_classes: element.size_classes.unwrap_or(scope.size_classes),
            inherited: widget_variants(part, outer),
            outer,
        })
    }

    /// Whether a part's rule matches the element (S7, S8): its context,
    /// position, variants, size, density, size classes, direction, flags
    /// and latest flags; its conditions held when the theme was resolved.
    pub(crate) fn matches(&self, selector: &Selector) -> bool {
        let size = &selector.size;
        selector.context.iter().all(|context| self.context(context))
            && !selector
                .not_context
                .iter()
                .any(|context| self.context(context))
            && selector
                .position
                .as_ref()
                .is_none_or(|position| self.position(position))
            && set_holds(&selector.variants, |variant| {
                self.variants.contains(variant) || self.inherited.contains(variant)
            })
            && one_holds(&size.sizes, self.size.as_ref())
            && one_holds(&size.densities, self.density.as_ref())
            && one_holds(
                &size.size_classes.horizontal,
                self.size_classes.horizontal.as_ref(),
            )
            && one_holds(
                &size.size_classes.vertical,
                self.size_classes.vertical.as_ref(),
            )
            && selector
                .direction
                .is_none_or(|direction| direction == self.direction)
            && set_holds(&selector.states, |flag| self.states.contains(flag))
            && selector
                .latest
                .iter()
                .all(|(set, flag)| self.latest_is(set, flag))
    }

    /// Whether `flag` is the latest of the element's recency set `set`
    /// (S7.2); never where the widget declares no such set.
    fn latest_is(&self, set: &Key, flag: &StateFlag) -> bool {
        let Some((_, members)) = self
            .decl
            .recency
            .iter()
            .flatten()
            .find(|(held, _)| held == set)
        else {
            return false;
        };
        latest(members, &self.states, self.element.state_order) == Some(flag)
    }

    /// Whether a context match holds over the element's ancestors, its own
    /// parts' entries skipped (S8): any of them, or the nearest ancestor
    /// instance of one of the widgets named, by any of its parts' entries.
    fn context(&self, context: &ContextMatch) -> bool {
        let holds = |entry: &ScopeEntry| {
            is_target(context.target.as_ref(), &entry.part)
                && one_holds(&context.sizes, entry.size.as_ref())
                && context
                    .surface
                    .is_none_or(|surface| entry.surface == Some(surface))
                && set_holds(&context.variants, |variant| {
                    entry.variants.contains(variant)
                })
                && set_holds(&context.states, |flag| entry.states.contains(flag))
        };
        match &context.scope {
            ContextScope::AnyAncestor => self.outer.iter().any(holds),
            ContextScope::NearestAmong(widgets) => instances(self.outer)
                .into_iter()
                .rev()
                .find(|entries| {
                    entries
                        .first()
                        .is_some_and(|head| widgets.contains(&widget_ref(&head.part)))
                })
                .is_some_and(|entries| entries.iter().any(holds)),
        }
    }

    /// Whether a position holds among the element's siblings (S7, S8):
    /// those `among` names counted, an `index` outside `siblings` as having
    /// none.
    fn position(&self, position: &PositionMatch) -> bool {
        let element = self.element;
        let counts = |sibling: &&Sibling<'_>| match &position.among {
            SiblingScope::SameKind => sibling.part == element.part,
            SiblingScope::Any => true,
            SiblingScope::Kinds(widgets) => widgets.contains(&widget_ref(&sibling.part)),
        };
        let (before, after): (Vec<&Sibling<'_>>, Vec<&Sibling<'_>>) =
            match element.siblings.get(element.index) {
                Some(_) => {
                    let before = element.siblings.get(..element.index).unwrap_or_default();
                    let after = element
                        .index
                        .checked_add(1)
                        .and_then(|from| element.siblings.get(from..))
                        .unwrap_or_default();
                    (
                        before.iter().filter(counts).collect(),
                        after.iter().filter(counts).collect(),
                    )
                }
                None => (Vec::new(), Vec::new()),
            };
        let first = before.is_empty();
        let last = after.is_empty();
        let parity = if before.len().is_multiple_of(2) {
            Parity::Odd
        } else {
            Parity::Even
        };
        position.first.is_none_or(|held| held == first)
            && position.last.is_none_or(|held| held == last)
            && position.only.is_none_or(|held| held == (first && last))
            && position.parity.is_none_or(|held| held == parity)
            && position
                .previous
                .as_ref()
                .is_none_or(|matched| self.sibling(matched, before.last().copied()))
            && position
                .next
                .as_ref()
                .is_none_or(|matched| self.sibling(matched, after.first().copied()))
    }

    /// Whether an adjacent sibling matches, its flags kept by its own
    /// widget's declarations (S8); never where there is none.
    fn sibling(&self, matched: &SiblingMatch, sibling: Option<&Sibling<'_>>) -> bool {
        let Some(sibling) = sibling else {
            return false;
        };
        let states = match &sibling.part {
            PartRef::Typed(part) => kept(&declarations(self.theme, part.widget()), &sibling.states),
            PartRef::Component { .. } => sibling.states.clone(),
        };
        is_target(matched.target.as_ref(), &sibling.part)
            && set_holds(&matched.variants, |variant| {
                sibling.variants.contains(variant)
            })
            && set_holds(&matched.states, |flag| states.contains(flag))
    }
}

/// Whether the connector sets `variant`, which the widget need not declare
/// (S8): a content variant of the part's element by the catalogue's pairs,
/// or one of the widget's breakpoints'.
fn set_by_connector(decl: &WidgetDecl, part: TypedPart, variant: &VariantRef) -> bool {
    let head = part.element();
    let own = (!head.info().is_some_and(|info| info.root)).then(|| head.name());
    let name = variant_name(variant);
    let content = registry::widgets()
        .iter()
        .filter(|info| info.kind == part.widget())
        .flat_map(|info| info.content_variants)
        .any(|(pair, held)| *pair == own && *held == name);
    content
        || decl
            .breakpoints
            .iter()
            .flatten()
            .any(|breakpoint| breakpoint.variant == *variant)
}

/// The sizes an element takes (S10): those of the first
/// `sizes_by_variant` entry, in written order, whose variant the element
/// holds, else the nearest ancestor's of `sizes_by_context`, else the
/// widget's `sizes`; `None` where none is declared.
fn takes<'d>(
    decl: &'d WidgetDecl,
    variants: &[VariantRef],
    outer: &[ScopeEntry],
) -> Option<&'d [ControlSize]> {
    let by_variant = decl
        .sizes_by_variant
        .iter()
        .flatten()
        .find(|by| variants.contains(&by.variant));
    if let Some(by) = by_variant {
        return Some(&by.sizes);
    }
    let by_context = decl.sizes_by_context.iter().flatten();
    let nearest = outer.iter().rev().find_map(|entry| {
        let widget = widget_ref(&entry.part);
        by_context.clone().find(|by| by.context == widget)
    });
    match nearest {
        Some(by) => Some(&by.sizes),
        None => decl.sizes.as_deref(),
    }
}

/// `entries` split into the widget instances they belong to, outermost
/// first: each from an entry of a widget's root (a component's part) through
/// the entries of its parts, its *(element)* parts' among them, so the
/// nearest of the widgets a context names is a widget, never one of its
/// elements (S7: `NearestAmong` names widgets).
fn instances(entries: &[ScopeEntry]) -> Vec<&[ScopeEntry]> {
    let mut starts: Vec<usize> = entries
        .iter()
        .enumerate()
        .filter(|(at, entry)| *at == 0 || starts_instance(&entry.part))
        .map(|(at, _)| at)
        .collect();
    starts.push(entries.len());
    starts
        .windows(2)
        .filter_map(|pair| match pair {
            [from, to] => entries.get(*from..*to),
            _ => None,
        })
        .collect()
}

/// Whether `part` starts a widget instance: a typed widget's root, and a
/// component's every part (none is declared yet, S10).
fn starts_instance(part: &PartRef) -> bool {
    match part {
        PartRef::Typed(part) => part.info().is_none_or(|info| info.root),
        PartRef::Component { .. } => true,
    }
}

#[cfg(test)]
mod tests {
    use super::nested_run;

    /// The run of an element's own widget instance stops at an outer
    /// instance of the same widget: a cell's row and view, then the outer
    /// list's row and view, of which only the inner two are the cell's;
    /// a view directly inside a view, of which only the inner one is.
    #[test]
    fn an_outer_instance_of_the_same_widget_is_not_the_elements() {
        let enclosing = ["row", "view"];
        let outer = [Some("view"), Some("row"), Some("view"), Some("row")];
        assert_eq!(nested_run(&enclosing, &outer), 2);
        let direct = [Some("view"), Some("view")];
        assert_eq!(nested_run(&enclosing, &direct), 1);
        let other = [Some("view"), None];
        assert_eq!(nested_run(&enclosing, &other), 0);
        assert_eq!(nested_run(&enclosing, &[Some("view"), Some("row")]), 2);
    }
}
