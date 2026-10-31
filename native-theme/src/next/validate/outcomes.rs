//! The distinct outcomes of a set of rules under S12's exclusions: one
//! assignment of the atoms for each realisable vector of the rules'
//! applicability (the M1a design's decision 5), found without enumerating
//! the assignments.
//!
//! A depth-first search decides each rule in turn, applying or not; a rule
//! that applies fixes the atoms its selector names, one that does not needs
//! one of them to differ. Each branch keeps an assignment that realises its
//! decisions (a witness): the branch the witness already takes costs
//! nothing, and the other is taken only where a search over the atoms finds
//! an assignment that honours S12's exclusions and every decision so far.
//! Every leaf is a distinct realisable vector, each reached once, so the
//! work grows with the number of distinct outcomes times the rules, not
//! with the assignments of the atoms.

use super::combos::Atom;
use super::stack::widget_of;
use crate::next::declare::{Declarations, WidgetDecl};
use crate::next::selector::{
    Condition, ContextMatch, ContextScope, Parity, PartRef, Scheme, SetMatch, SiblingScope,
    StateFlag, TiledEdges, VariantRef, WidgetOrPart, WidgetRef,
};
use crate::next::widgets::WidgetKind;

/// A decision: the atoms' values a rule's selector needs, each `(atom,
/// value)`; `None`: it never applies.
pub(super) type Item = Option<Vec<(usize, bool)>>;

/// The groups of atoms of which at most one holds, each with whether one
/// holds exactly (S12's exclusions: one value at a time of each one-of
/// condition, one contrast condition, one control size, density, direction,
/// size class per axis and parity); every other atom is free.
pub(super) fn one_of(atoms: &[Atom], decl: &Declarations) -> Vec<(Vec<usize>, bool)> {
    /// Which one-of group an atom is in.
    #[derive(PartialEq)]
    enum Group<'a> {
        Scheme,
        Contrast,
        Idiom,
        Posture,
        Device,
        Input,
        Flavor,
        Container(&'a WidgetRef),
        Size,
        Density,
        Direction,
        Class(bool),
        Parity(&'a SiblingScope),
    }
    let count = |of: fn(&Atom) -> bool| atoms.iter().filter(|atom| of(atom)).count();
    let mut keyed: Vec<(Group<'_>, bool, Vec<usize>)> = Vec::new();
    for (index, atom) in atoms.iter().enumerate() {
        let key = match atom {
            Atom::Cond(Condition::Scheme(_)) => Some((Group::Scheme, true)),
            Atom::Cond(Condition::ContrastIncreased | Condition::ContrastHigh) => {
                Some((Group::Contrast, false))
            }
            Atom::Cond(Condition::Idiom(_)) => Some((Group::Idiom, false)),
            Atom::Cond(Condition::Posture(_)) => Some((Group::Posture, false)),
            Atom::Cond(Condition::DeviceModel(_)) => Some((Group::Device, false)),
            Atom::Cond(Condition::Input(_)) => Some((
                Group::Input,
                count(|a| matches!(a, Atom::Cond(Condition::Input(_)))) == 4,
            )),
            Atom::Cond(Condition::Flavor(_)) => Some((Group::Flavor, !decl.flavors.is_empty())),
            Atom::Cond(Condition::ContainerSize { container, .. }) => {
                Some((Group::Container(container), true))
            }
            Atom::Size(_) => Some((Group::Size, count(|a| matches!(a, Atom::Size(_))) == 6)),
            Atom::Density(_) => Some((Group::Density, false)),
            Atom::Dir(_) => Some((Group::Direction, true)),
            Atom::Class(vertical, _) => Some((Group::Class(*vertical), false)),
            Atom::Parity(among, _) => Some((Group::Parity(among), true)),
            _ => None,
        };
        if let Some((key, exactly)) = key {
            match keyed.iter_mut().find(|(held, ..)| *held == key) {
                Some((_, _, members)) => members.push(index),
                None => keyed.push((key, exactly, vec![index])),
            }
        }
    }
    keyed
        .into_iter()
        .map(|(_, exactly, members)| (members, exactly))
        .collect()
}

/// The recency sets in force under one flavour (S7.2): each set's members'
/// `(latest, state)` atoms (the state `None` where no atom holds it, so
/// false), and the `latest` atoms that never hold.
type Recency = (Vec<Vec<(usize, Option<usize>)>>, Vec<usize>);

/// The exclusions over the atoms of one check (S12), each by the atoms'
/// indices.
pub(super) struct Space {
    /// How many atoms there are.
    len: usize,
    /// The one-of groups.
    groups: Vec<(Vec<usize>, bool)>,
    /// Each atom's one-of group, where it is in one.
    group_of: Vec<Option<usize>>,
    /// Each atom's fellow members of the exclusive groups in force under
    /// every assignment (the default flavour's, where no flavour is an
    /// atom): one holding, the atom does not.
    beside: Vec<Vec<usize>>,
    /// The declared flavours, in order, each with its atom where the atoms
    /// hold one.
    flavors: Vec<Option<usize>>,
    /// Under the default flavour, then under each declared one in order:
    /// the exclusive flag and variant groups of the element's widgets, by
    /// the atoms they hold, at most one of each holding.
    exclusive: Vec<Vec<Vec<usize>>>,
    /// Each `only` with its `first` and `last` (`None`: not an atom, so
    /// false): `only` holds exactly when both do.
    only: Vec<(usize, Option<usize>, Option<usize>)>,
    /// Each `first` with its `even` parity: not both.
    first_even: Vec<(usize, usize)>,
    /// Each pair of a `Tiled` and a `Tiled` within it, or `Maximized` and
    /// a `Tiled`: the first holding, the second holds.
    implies: Vec<(usize, usize)>,
    /// The context matches no one ancestor can hold.
    never: Vec<usize>,
    /// Under the default flavour, then under each declared one in order:
    /// the element's recency sets, each its members' `(latest, state)`
    /// atoms where the atoms hold them, and the `latest` atoms naming a set
    /// the flavour does not declare or a flag not a member of it, which never
    /// hold (S7.2).
    recency: Vec<Recency>,
    /// Under the default flavour, then under each declared one in order:
    /// the context and sibling matches needing two members of one exclusive
    /// group of their widget, which the S10 reduction never leaves (S8).
    reduced: Vec<Vec<usize>>,
    /// The pairs of context matches of one nearest ancestor needing sizes
    /// it cannot have together.
    exclusive_contexts: Vec<(usize, usize)>,
}

impl Space {
    /// The exclusions over `atoms` the declarations and the element's
    /// widgets give.
    pub(super) fn new(atoms: &[Atom], decl: &Declarations, widgets: &[WidgetKind]) -> Space {
        let index = |atom: &Atom| atoms.iter().position(|held| held == atom);
        let flavors = decl
            .flavors
            .iter()
            .map(|flavor| index(&Atom::Cond(Condition::Flavor(flavor.clone()))))
            .collect();
        let groups_under = |flavor: Option<&crate::next::key::Key>| {
            let mut out: Vec<Vec<usize>> = Vec::new();
            for widget in widgets {
                let decls: WidgetDecl = match flavor {
                    Some(flavor) => decl.in_flavor(*widget, flavor),
                    None => decl.in_default_flavor(*widget),
                };
                for group in decls.exclusive.iter().flatten() {
                    out.push(
                        group
                            .iter()
                            .filter_map(|flag| index(&Atom::State(flag.clone())))
                            .collect(),
                    );
                }
                for group in decls.exclusive_variants.iter().flatten() {
                    out.push(
                        group
                            .iter()
                            .filter_map(|variant| index(&Atom::Variant(variant.clone())))
                            .collect(),
                    );
                }
            }
            out.retain(|group| group.len() > 1);
            out
        };
        let mut exclusive = vec![groups_under(decl.flavors.first())];
        exclusive.extend(decl.flavors.iter().map(|flavor| groups_under(Some(flavor))));
        let reduced_under = |flavor: Option<&crate::next::key::Key>| {
            let decls = |widget: WidgetKind| match flavor {
                Some(flavor) => decl.in_flavor(widget, flavor),
                None => decl.in_default_flavor(widget),
            };
            atoms
                .iter()
                .enumerate()
                .filter(|(_, atom)| match atom {
                    Atom::Context(context) => reduced_away(
                        context.target.as_ref(),
                        &context.states,
                        &context.variants,
                        decls,
                    ),
                    Atom::Sibling(_, _, sibling) => reduced_away(
                        sibling.target.as_ref(),
                        &sibling.states,
                        &sibling.variants,
                        decls,
                    ),
                    _ => false,
                })
                .map(|(at, _)| at)
                .collect::<Vec<usize>>()
        };
        let recency_under = |flavor: Option<&crate::next::key::Key>| {
            let mut sets: Vec<(&crate::next::key::Key, Vec<StateFlag>)> = Vec::new();
            for widget in widgets {
                let decls: WidgetDecl = match flavor {
                    Some(flavor) => decl.in_flavor(*widget, flavor),
                    None => decl.in_default_flavor(*widget),
                };
                for (set, flags) in decls.recency.iter().flatten() {
                    if let Some((_, held)) = sets.iter_mut().find(|(held, _)| *held == set) {
                        held.extend(
                            flags
                                .iter()
                                .filter(|f| !held.contains(f))
                                .cloned()
                                .collect::<Vec<_>>(),
                        );
                    } else if let Some(name) = atoms.iter().find_map(|atom| match atom {
                        Atom::Latest(name, _) if name == set => Some(name),
                        _ => None,
                    }) {
                        sets.push((name, flags.clone()));
                    }
                }
            }
            let groups: Vec<Vec<(usize, Option<usize>)>> = sets
                .iter()
                .map(|(set, flags)| {
                    flags
                        .iter()
                        .filter_map(|flag| {
                            let latest = index(&Atom::Latest((*set).clone(), flag.clone()))?;
                            Some((latest, index(&Atom::State(flag.clone()))))
                        })
                        .collect()
                })
                .collect();
            let never: Vec<usize> = atoms
                .iter()
                .enumerate()
                .filter(|(_, atom)| match atom {
                    Atom::Latest(set, flag) => !sets
                        .iter()
                        .any(|(held, flags)| *held == set && flags.contains(flag)),
                    _ => false,
                })
                .map(|(at, _)| at)
                .collect();
            (groups, never)
        };
        let mut recency = vec![recency_under(decl.flavors.first())];
        recency.extend(
            decl.flavors
                .iter()
                .map(|flavor| recency_under(Some(flavor))),
        );
        let mut reduced = vec![reduced_under(decl.flavors.first())];
        reduced.extend(
            decl.flavors
                .iter()
                .map(|flavor| reduced_under(Some(flavor))),
        );
        let mut only = Vec::new();
        let mut first_even = Vec::new();
        let mut implies = Vec::new();
        let mut never = Vec::new();
        let mut exclusive_contexts = Vec::new();
        let maximized = index(&Atom::Cond(Condition::Maximized));
        for (at, atom) in atoms.iter().enumerate() {
            match atom {
                Atom::Only(among) => only.push((
                    at,
                    index(&Atom::First(among.clone())),
                    index(&Atom::Last(among.clone())),
                )),
                Atom::First(among) => {
                    if let Some(even) = index(&Atom::Parity(among.clone(), Parity::Even)) {
                        first_even.push((at, even));
                    }
                }
                Atom::Cond(Condition::Tiled(edges)) => {
                    if let Some(maximized) = maximized {
                        implies.push((maximized, at));
                    }
                    for (other, inner) in atoms.iter().enumerate() {
                        if let Atom::Cond(Condition::Tiled(inner)) = inner
                            && within(inner, edges)
                        {
                            implies.push((at, other));
                        }
                    }
                }
                Atom::Context(context) => {
                    if !one_size(context) {
                        never.push(at);
                    }
                    if let ContextScope::NearestAmong(_) = context.scope {
                        for (other, second) in atoms.iter().enumerate() {
                            if let Atom::Context(second) = second
                                && second.scope == context.scope
                                && sizes_conflict(context, second)
                            {
                                exclusive_contexts.push((at, other));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        let groups = one_of(atoms, decl);
        let mut group_of = vec![None; atoms.len()];
        for (group, (members, _)) in groups.iter().enumerate() {
            for member in members {
                if let Some(slot) = group_of.get_mut(*member) {
                    *slot = Some(group);
                }
            }
        }
        let mut beside: Vec<Vec<usize>> = vec![Vec::new(); atoms.len()];
        let flavors: Vec<Option<usize>> = flavors;
        if flavors.iter().all(Option::is_none)
            && let Some(groups) = exclusive.first()
        {
            for group in groups {
                for member in group {
                    if let Some(slot) = beside.get_mut(*member) {
                        slot.extend(group.iter().copied().filter(|m| m != member));
                    }
                }
            }
        }
        Space {
            len: atoms.len(),
            groups,
            group_of,
            beside,
            flavors,
            exclusive,
            only,
            first_even,
            implies,
            never,
            recency,
            reduced,
            exclusive_contexts,
        }
    }

    /// Whether a partial assignment breaks none of the exclusions no group
    /// expresses, as far as its values decide them (S12): each declared
    /// exclusive flag group and exclusive variant group of the element's
    /// widget holding at most one, under the flavour in force; among one
    /// `among` set, `only` exactly when `first` and `last`, and `first`
    /// odd; `Tiled(e)` implying `Tiled(e')` for every `e'` within `e`, and
    /// `Maximized` every `Tiled`; one ancestor having one control size. A
    /// complete assignment it admits is one S12 allows.
    pub(super) fn admits(&self, values: &[Option<bool>]) -> bool {
        let value = |at: usize| values.get(at).copied().flatten();
        // The flavour in force: the first declared one holding, else the
        // default; undecided while an earlier one is unassigned.
        let mut flavor = Some(0_usize);
        for (order, atom) in self.flavors.iter().enumerate() {
            match atom.map(value) {
                Some(Some(true)) => {
                    flavor = Some(order.saturating_add(1));
                    break;
                }
                Some(None) => {
                    flavor = None;
                    break;
                }
                Some(Some(false)) | None => {}
            }
        }
        if let Some(groups) = flavor.and_then(|flavor| self.exclusive.get(flavor)) {
            for group in groups {
                if group.iter().filter(|at| value(**at) == Some(true)).count() > 1 {
                    return false;
                }
            }
        }
        if let Some(reduced) = flavor.and_then(|flavor| self.reduced.get(flavor))
            && reduced.iter().any(|at| value(*at) == Some(true))
        {
            return false;
        }
        // Each recency set's latest flag (S7.2, S12): at most one, a kept
        // flag, and one exactly where any member is kept (the exclusive
        // groups above keep at most one of each, so a flag held is kept).
        if let Some((sets, never)) = flavor.and_then(|flavor| self.recency.get(flavor)) {
            if never.iter().any(|at| value(*at) == Some(true)) {
                return false;
            }
            for members in sets {
                let latest = members
                    .iter()
                    .filter(|(at, _)| value(*at) == Some(true))
                    .count();
                let unkept = members.iter().any(|(at, state)| {
                    value(*at) == Some(true) && state.map_or(Some(false), value) == Some(false)
                });
                let none_left = members.iter().all(|(at, _)| value(*at) == Some(false));
                let kept = members
                    .iter()
                    .any(|(_, state)| state.and_then(value) == Some(true));
                if latest > 1 || unkept || (none_left && kept) {
                    return false;
                }
            }
        }
        for (only, first, last) in &self.only {
            // A `first` or `last` the atoms do not hold is false.
            let side = |at: &Option<usize>| at.map_or(Some(false), value);
            if let (Some(only), Some(first), Some(last)) = (value(*only), side(first), side(last))
                && only != (first && last)
            {
                return false;
            }
        }
        let both = |(a, b): &(usize, usize)| value(*a) == Some(true) && value(*b) == Some(true);
        if self.first_even.iter().any(both) || self.exclusive_contexts.iter().any(both) {
            return false;
        }
        if self
            .implies
            .iter()
            .any(|(a, b)| value(*a) == Some(true) && value(*b) == Some(false))
        {
            return false;
        }
        !self.never.iter().any(|at| value(*at) == Some(true))
    }

    /// Whether a complete assignment honours S12's exclusions: its one-of
    /// groups and every other exclusion.
    pub(super) fn admissible(&self, values: &[bool]) -> bool {
        let partial: Vec<Option<bool>> = values.iter().copied().map(Some).collect();
        self.groups_hold(&partial) && self.admits(&partial)
    }

    /// Whether the one-of groups hold as far as `values` decides them: at
    /// most one member holding, and one where one holds exactly.
    fn groups_hold(&self, values: &[Option<bool>]) -> bool {
        self.groups.iter().all(|(members, exactly)| {
            let value = |m: &usize| values.get(*m).copied().flatten();
            let held = members.iter().filter(|m| value(m) == Some(true)).count();
            let unset = members.iter().any(|m| value(m).is_none());
            held <= 1 && (!*exactly || held == 1 || unset)
        })
    }

    /// One assignment for each distinct vector of `items`' applicability
    /// that S12's exclusions allow.
    pub(super) fn outcomes(&self, items: &[Item]) -> Vec<Vec<bool>> {
        let n = self.len;
        let items: Vec<Item> = items
            .iter()
            .map(|item| {
                let mut needs = item.clone()?;
                needs.sort_unstable();
                needs.dedup();
                let contradicts = needs
                    .windows(2)
                    .any(|pair| matches!(pair, [(a, _), (b, _)] if a == b));
                (!contradicts && needs.iter().all(|(atom, _)| *atom < n)).then_some(needs)
            })
            .collect();
        // A rule deciding as an earlier one does, or always or never
        // applying, takes no decision of its own.
        let mut decided: Vec<Vec<(usize, bool)>> = Vec::new();
        for needs in items.iter().flatten() {
            if !needs.is_empty() && !decided.contains(needs) {
                decided.push(needs.clone());
            }
        }
        let mut out = Vec::new();
        let Some(witness) = self.solve(&vec![None; n], &[]) else {
            return out;
        };
        let mut search = Search {
            space: self,
            items: &decided,
            out: &mut out,
        };
        search.walk(0, &mut vec![None; n], &mut Vec::new(), witness);
        out
    }

    /// The value `fixed` gives `atom`, or its one-of group implies: false
    /// beside a member holding, true where held exactly and every other
    /// member is false; false too beside a member of an exclusive group in
    /// force whatever the assignment.
    fn implied(&self, fixed: &[Option<bool>], atom: usize) -> Option<bool> {
        let value = |atom: usize| fixed.get(atom).copied().flatten();
        if let Some(held) = value(atom) {
            return Some(held);
        }
        if self
            .beside
            .get(atom)
            .is_some_and(|others| others.iter().any(|m| value(*m) == Some(true)))
        {
            return Some(false);
        }
        let (members, exactly) = self
            .group_of
            .get(atom)
            .copied()
            .flatten()
            .and_then(|group| self.groups.get(group))?;
        let others = members.iter().filter(|m| **m != atom);
        if others.clone().any(|m| value(*m) == Some(true)) {
            Some(false)
        } else if *exactly && others.clone().all(|m| value(*m) == Some(false)) {
            Some(true)
        } else {
            None
        }
    }

    /// Whether the atoms `fixed` fixes alone rule every completion out: a
    /// clause all of whose atoms hold the values it needs, or a one-of
    /// group with two members holding or, held exactly, none left.
    fn refutes(&self, fixed: &[Option<bool>], clauses: &[&[(usize, bool)]]) -> bool {
        clauses.iter().any(|clause| {
            clause
                .iter()
                .all(|(atom, want)| self.implied(fixed, *atom) == Some(*want))
        }) || !self.groups_hold(fixed)
    }

    /// `witness` changed to take the other decision on `needs` (`apply`:
    /// all of them held; else one of them away), its one-of groups kept: a
    /// first try before [`Space::solve`], checked whole, `None` where it
    /// fails.
    fn repair(
        &self,
        fixed: &[Option<bool>],
        clauses: &[&[(usize, bool)]],
        witness: &[bool],
        needs: &[(usize, bool)],
        apply: bool,
    ) -> Option<Vec<bool>> {
        let mut out = witness.to_vec();
        let mut set: Vec<usize> = Vec::new();
        if apply {
            for (atom, want) in needs {
                *out.get_mut(*atom)? = *want;
                set.push(*atom);
            }
        } else {
            let (atom, want) = needs
                .iter()
                .find(|(atom, _)| fixed.get(*atom).copied().flatten().is_none())?;
            *out.get_mut(*atom)? = !*want;
            set.push(*atom);
        }
        let pinned =
            |atom: usize| set.contains(&atom) || fixed.get(atom).copied().flatten().is_some();
        for (members, exactly) in &self.groups {
            let held: Vec<usize> = members
                .iter()
                .copied()
                .filter(|m| out.get(*m) == Some(&true))
                .collect();
            if held.len() > 1 {
                for m in &held {
                    if !pinned(*m) {
                        *out.get_mut(*m)? = false;
                    }
                }
            } else if held.is_empty() && *exactly {
                let free = members.iter().copied().find(|m| !pinned(*m))?;
                *out.get_mut(free)? = true;
            }
        }
        let respects = fixed
            .iter()
            .zip(&out)
            .all(|(fixed, value)| fixed.is_none_or(|fixed| fixed == *value));
        let away = clauses.iter().all(|clause| {
            clause
                .iter()
                .any(|(atom, want)| out.get(*atom) != Some(want))
        });
        (respects && away && self.admissible(&out)).then_some(out)
    }

    /// An assignment extending `fixed` that honours S12's exclusions and in
    /// which each of `clauses` (a rule not applying) has an atom away from
    /// the value it needs; `None` where there is none.
    fn solve(&self, fixed: &[Option<bool>], clauses: &[&[(usize, bool)]]) -> Option<Vec<bool>> {
        let mut values = fixed.to_vec();
        self.extend(&mut values, clauses)
            .then(|| values.into_iter().map(|v| v.unwrap_or(false)).collect())
    }

    /// Completes `values` as [`Space::solve`] asks; `false` where no
    /// completion exists.
    fn extend(&self, values: &mut Vec<Option<bool>>, clauses: &[&[(usize, bool)]]) -> bool {
        if !self.propagate(values, clauses) || !self.groups_hold(values) || !self.admits(values) {
            return false;
        }
        // An atom of a clause not yet away from its needs first, its other
        // value tried first; then any atom, false first.
        let open = clauses.iter().find_map(|clause| {
            let away = clause
                .iter()
                .any(|(atom, want)| values.get(*atom).copied().flatten() == Some(!*want));
            if away {
                return None;
            }
            clause
                .iter()
                .find(|(atom, _)| values.get(*atom).copied().flatten().is_none())
                .map(|(atom, want)| (*atom, !*want))
        });
        let next = open.or_else(|| {
            values
                .iter()
                .position(Option::is_none)
                .map(|atom| (atom, false))
        });
        let Some((atom, first)) = next else {
            let complete: Vec<bool> = values.iter().map(|v| v.unwrap_or(false)).collect();
            return self.admissible(&complete);
        };
        for value in [first, !first] {
            let mut tried = values.clone();
            if let Some(slot) = tried.get_mut(atom) {
                *slot = Some(value);
            }
            if self.extend(&mut tried, clauses) {
                *values = tried;
                return true;
            }
        }
        false
    }

    /// Sets the atoms the clauses and the one-of groups force; `false` on
    /// a contradiction.
    fn propagate(&self, values: &mut [Option<bool>], clauses: &[&[(usize, bool)]]) -> bool {
        loop {
            let mut changed = false;
            for clause in clauses {
                let mut open = None;
                let mut opens = 0_usize;
                let mut away = false;
                for (atom, want) in clause.iter() {
                    match values.get(*atom).copied().flatten() {
                        Some(value) if value != *want => away = true,
                        Some(_) => {}
                        None => {
                            opens = opens.saturating_add(1);
                            open = Some((*atom, !*want));
                        }
                    }
                }
                if away {
                    continue;
                }
                match (opens, open) {
                    (0, _) => return false,
                    (1, Some((atom, value))) => {
                        if let Some(slot) = values.get_mut(atom) {
                            *slot = Some(value);
                            changed = true;
                        }
                    }
                    _ => {}
                }
            }
            for (members, exactly) in &self.groups {
                let value = |values: &[Option<bool>], m: usize| values.get(m).copied().flatten();
                let held = members
                    .iter()
                    .filter(|m| value(values, **m) == Some(true))
                    .count();
                if held > 1 {
                    return false;
                }
                let unset: Vec<usize> = members
                    .iter()
                    .copied()
                    .filter(|m| value(values, *m).is_none())
                    .collect();
                let force = match (held, unset.as_slice()) {
                    (1, _) => Some(false),
                    (0, [_]) if *exactly => Some(true),
                    (0, []) if *exactly => return false,
                    _ => None,
                };
                if let Some(force) = force {
                    for m in &unset {
                        if let Some(slot) = values.get_mut(*m) {
                            *slot = Some(force);
                            changed = true;
                        }
                    }
                }
            }
            if !changed {
                return true;
            }
        }
    }
}

/// The depth-first search over the rules' decisions.
struct Search<'s> {
    space: &'s Space,
    /// The distinct selectors to decide, each the atoms' values it needs.
    items: &'s [Vec<(usize, bool)>],
    out: &'s mut Vec<Vec<bool>>,
}

impl Search<'_> {
    /// Decides the items from `index` on: `fixed` the atoms the applying
    /// ones fix, `away` the ones not applying, `witness` an assignment that
    /// realises every decision so far.
    fn walk(
        &mut self,
        index: usize,
        fixed: &mut Vec<Option<bool>>,
        away: &mut Vec<usize>,
        witness: Vec<bool>,
    ) {
        let Some(needs) = self.items.get(index) else {
            self.out.push(witness);
            return;
        };
        let next = index.saturating_add(1);
        let applies = needs
            .iter()
            .all(|(atom, want)| witness.get(*atom) == Some(want));
        // The other decision, where some assignment realises it.
        let other = if applies {
            let pinned = needs
                .iter()
                .all(|(atom, want)| self.space.implied(fixed, *atom) == Some(*want));
            (!pinned).then(|| {
                away.push(index);
                let clauses = self.clauses(away);
                let found = self
                    .space
                    .repair(fixed, &clauses, &witness, needs, false)
                    .or_else(|| self.space.solve(fixed, &clauses));
                away.pop();
                found
            })
        } else {
            let open = needs
                .iter()
                .all(|(atom, want)| self.space.implied(fixed, *atom) != Some(!*want));
            open.then(|| {
                let trail = fix(fixed, needs);
                let clauses = self.clauses(away);
                let found = if self.space.refutes(fixed, &clauses) {
                    None
                } else {
                    self.space
                        .repair(fixed, &clauses, &witness, needs, true)
                        .or_else(|| self.space.solve(fixed, &clauses))
                };
                unfix(fixed, &trail);
                found
            })
        }
        .flatten();
        // The decision the witness takes, then the other.
        if applies {
            let trail = fix(fixed, needs);
            self.walk(next, fixed, away, witness);
            unfix(fixed, &trail);
        } else {
            away.push(index);
            self.walk(next, fixed, away, witness);
            away.pop();
        }
        if let Some(found) = other {
            if applies {
                away.push(index);
                self.walk(next, fixed, away, found);
                away.pop();
            } else {
                let trail = fix(fixed, needs);
                self.walk(next, fixed, away, found);
                unfix(fixed, &trail);
            }
        }
    }

    /// The selectors of the items not applying.
    fn clauses(&self, away: &[usize]) -> Vec<&[(usize, bool)]> {
        away.iter()
            .filter_map(|index| self.items.get(*index).map(Vec::as_slice))
            .collect()
    }
}

/// Fixes the atoms `needs` names that `fixed` leaves open, to the values it
/// needs; the atoms fixed, to undo. The caller has checked that none
/// contradicts `fixed`.
fn fix(fixed: &mut [Option<bool>], needs: &[(usize, bool)]) -> Vec<usize> {
    let mut trail = Vec::new();
    for (atom, want) in needs {
        if let Some(slot) = fixed.get_mut(*atom)
            && slot.is_none()
        {
            *slot = Some(*want);
            trail.push(*atom);
        }
    }
    trail
}

/// Opens again the atoms `fix` fixed.
fn unfix(fixed: &mut [Option<bool>], trail: &[usize]) {
    for atom in trail {
        if let Some(slot) = fixed.get_mut(*atom) {
            *slot = None;
        }
    }
}

/// Whether every edge `inner` names `outer` names too.
pub(super) fn within(inner: &TiledEdges, outer: &TiledEdges) -> bool {
    (!inner.top || outer.top)
        && (!inner.right || outer.right)
        && (!inner.bottom || outer.bottom)
        && (!inner.left || outer.left)
}

/// Whether one ancestor can match `context`'s sizes: it has one control
/// size, so at most one size is required, and that one is not excluded.
pub(super) fn one_size(context: &ContextMatch) -> bool {
    let all = &context.sizes.all_of;
    let first = all.first();
    all.iter().all(|size| Some(size) == first)
        && first.is_none_or(|size| !context.sizes.none_of.contains(size))
}

/// Whether an ancestor or sibling matched by `states` and `variants` on
/// `target` needs two members of one exclusive flag or variant group of its
/// widget, as `decls` gives them (S10): the reduction drops all but one of
/// such flags, and two such variants are no available element (S8).
fn reduced_away(
    target: Option<&WidgetOrPart>,
    states: &SetMatch<StateFlag>,
    variants: &SetMatch<VariantRef>,
    decls: impl Fn(WidgetKind) -> WidgetDecl,
) -> bool {
    let widget = match target {
        Some(WidgetOrPart::Widget(WidgetRef::Typed(widget))) => *widget,
        Some(WidgetOrPart::Part(PartRef::Typed(part))) => widget_of(*part),
        Some(
            WidgetOrPart::Widget(WidgetRef::Component(_))
            | WidgetOrPart::Part(PartRef::Component { .. }),
        )
        | None => return false,
    };
    fn two<T: PartialEq>(group: &[T], held: &[T]) -> bool {
        group.iter().filter(|member| held.contains(member)).count() > 1
    }
    let decl = decls(widget);
    decl.exclusive
        .iter()
        .flatten()
        .any(|group| two(group, &states.all_of))
        || decl
            .exclusive_variants
            .iter()
            .flatten()
            .any(|group| two(group, &variants.all_of))
}

/// Whether two matches of one nearest ancestor require sizes it cannot have
/// together.
pub(super) fn sizes_conflict(a: &ContextMatch, b: &ContextMatch) -> bool {
    let required = |c: &ContextMatch| c.sizes.all_of.first().copied();
    match (required(a), required(b)) {
        (Some(x), Some(y)) => x != y,
        (Some(x), None) => b.sizes.none_of.contains(&x),
        (None, Some(y)) => a.sizes.none_of.contains(&y),
        (None, None) => false,
    }
}

/// The scheme atom and every flavour atom the atoms hold, as decisions of
/// their own: two worlds differing only in the scheme or the flavour in
/// force are two outcomes (S12).
pub(super) fn scheme_and_flavors(atoms: &[Atom]) -> Vec<Item> {
    atoms
        .iter()
        .enumerate()
        .filter(|(_, atom)| {
            matches!(
                atom,
                Atom::Cond(Condition::Scheme(Scheme::Dark) | Condition::Flavor(_))
            )
        })
        .map(|(index, _)| Some(vec![(index, true)]))
        .collect()
}
