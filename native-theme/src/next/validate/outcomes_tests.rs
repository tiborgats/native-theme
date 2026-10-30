//! S12's exclusions enumerated two ways: the outcomes' search and the
//! brute force it replaced (every assignment of the atoms, kept here as the
//! reference), which must give the same applicability vectors; and the
//! search's cost at the sizes M3's presets reach. Every name here is
//! synthetic.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

use std::collections::HashSet;

use proptest::prelude::*;

use super::combos::Atom;
use super::outcomes::{Item, Space, one_size, sizes_conflict, within};
use crate::key;
use crate::next::declare::{Declarations, WidgetDecl};
use crate::next::selector::{
    Condition, ContextMatch, ContextScope, ControlSize, Direction, Parity, Scheme, SiblingScope,
    StateFlag, TiledEdges, VariantRef, WidgetRef,
};
use crate::next::toml::layer::read_preset;
use crate::next::widgets::WidgetKind;

// The brute force, as task 9 wrote it: every pattern of every one-of
// group, each assignment checked whole.

/// A group's patterns: each the atoms it sets and their values.
type Group = Vec<Vec<(usize, bool)>>;

/// The one-of groups S12's exclusions give (one value at a time of each
/// one-of condition, one contrast condition, one control size, density,
/// direction, size class per axis and parity), each a list of the
/// patterns it allows; every other atom free.
fn groups(atoms: &[Atom], decl: &Declarations) -> Vec<Group> {
    let mut keyed: Vec<(String, bool, Vec<usize>)> = Vec::new();
    let mut free: Vec<usize> = Vec::new();
    for (index, atom) in atoms.iter().enumerate() {
        let key: Option<(String, bool)> = match atom {
            Atom::Cond(Condition::Scheme(_)) => Some(("scheme".to_owned(), true)),
            Atom::Cond(Condition::ContrastIncreased | Condition::ContrastHigh) => {
                Some(("contrast".to_owned(), false))
            }
            Atom::Cond(Condition::Idiom(_)) => Some(("idiom".to_owned(), false)),
            Atom::Cond(Condition::Posture(_)) => Some(("posture".to_owned(), false)),
            Atom::Cond(Condition::DeviceModel(_)) => Some(("device".to_owned(), false)),
            Atom::Cond(Condition::Input(_)) => {
                let all = atoms
                    .iter()
                    .filter(|a| matches!(a, Atom::Cond(Condition::Input(_))))
                    .count()
                    == 4;
                Some(("input".to_owned(), all))
            }
            Atom::Cond(Condition::Flavor(_)) => {
                Some(("flavor".to_owned(), !decl.flavors.is_empty()))
            }
            Atom::Cond(Condition::ContainerSize { container, .. }) => {
                Some((format!("container:{container:?}"), true))
            }
            Atom::Size(_) => {
                let all = atoms.iter().filter(|a| matches!(a, Atom::Size(_))).count() == 6;
                Some(("size".to_owned(), all))
            }
            Atom::Density(_) => Some(("density".to_owned(), false)),
            Atom::Dir(_) => Some(("direction".to_owned(), true)),
            Atom::Class(vertical, _) => Some((format!("class:{vertical}"), false)),
            Atom::Parity(among, _) => Some((format!("parity:{among:?}"), true)),
            _ => None,
        };
        match key {
            Some((key, exactly)) => match keyed.iter_mut().find(|(held, ..)| *held == key) {
                Some((_, _, members)) => members.push(index),
                None => keyed.push((key, exactly, vec![index])),
            },
            None => free.push(index),
        }
    }
    let mut out: Vec<Group> = keyed
        .into_iter()
        .map(|(_, exactly, members)| {
            let mut patterns: Group = members
                .iter()
                .map(|chosen| members.iter().map(|m| (*m, m == chosen)).collect())
                .collect();
            if !exactly {
                patterns.push(members.iter().map(|m| (*m, false)).collect());
            }
            patterns
        })
        .collect();
    out.extend(
        free.into_iter()
            .map(|atom| vec![vec![(atom, false)], vec![(atom, true)]]),
    );
    out
}

/// The next choice of each group's pattern; `false` past the last.
fn advance(choice: &mut [usize], groups: &[Group]) -> bool {
    for (chosen, group) in choice.iter_mut().zip(groups) {
        let next = chosen.saturating_add(1);
        if next < group.len() {
            *chosen = next;
            return true;
        }
        *chosen = 0;
    }
    false
}

/// Whether an assignment honours the exclusions no group expresses (S12):
/// each declared exclusive flag group and exclusive variant group of the
/// element's widget holding at most one, under the flavour in force; among
/// one `among` set, `only` exactly when `first` and `last`, and `first`
/// odd; `Tiled(e)` implying `Tiled(e')` for every `e'` within `e`, and
/// `Maximized` every `Tiled`; one ancestor having one control size.
fn admissible(
    atoms: &[Atom],
    values: &[bool],
    decl: &Declarations,
    widgets: &[WidgetKind],
) -> bool {
    let holds = |atom: &Atom| {
        atoms
            .iter()
            .zip(values)
            .find(|(held, _)| *held == atom)
            .map(|(_, value)| *value)
    };
    let flavor = decl
        .flavors
        .iter()
        .find(|flavor| holds(&Atom::Cond(Condition::Flavor((*flavor).clone()))) == Some(true))
        .or(decl.flavors.first());
    for widget in widgets {
        let decls: WidgetDecl = match flavor {
            Some(flavor) => decl.in_flavor(*widget, flavor),
            None => decl.in_default_flavor(*widget),
        };
        for group in decls.exclusive.iter().flatten() {
            let held = group
                .iter()
                .filter(|flag| holds(&Atom::State((*flag).clone())) == Some(true))
                .count();
            if held > 1 {
                return false;
            }
        }
        for group in decls.exclusive_variants.iter().flatten() {
            let held = group
                .iter()
                .filter(|variant| holds(&Atom::Variant((*variant).clone())) == Some(true))
                .count();
            if held > 1 {
                return false;
            }
        }
    }
    for atom in atoms {
        match atom {
            Atom::Only(among) => {
                let first = holds(&Atom::First(among.clone())).unwrap_or(false);
                let last = holds(&Atom::Last(among.clone())).unwrap_or(false);
                if holds(atom) != Some(first && last) {
                    return false;
                }
            }
            Atom::First(among) => {
                if holds(atom) == Some(true)
                    && holds(&Atom::Parity(among.clone(), Parity::Even)) == Some(true)
                {
                    return false;
                }
            }
            Atom::Cond(Condition::Tiled(edges)) => {
                let maximized = holds(&Atom::Cond(Condition::Maximized)) == Some(true);
                if maximized && holds(atom) == Some(false) {
                    return false;
                }
                for other in atoms {
                    if let Atom::Cond(Condition::Tiled(inner)) = other
                        && within(inner, edges)
                        && holds(atom) == Some(true)
                        && holds(other) == Some(false)
                    {
                        return false;
                    }
                }
            }
            Atom::Context(context) => {
                if holds(atom) == Some(true) && !one_size(context) {
                    return false;
                }
                if let ContextScope::NearestAmong(_) = context.scope {
                    for other in atoms {
                        if let Atom::Context(second) = other
                            && second.scope == context.scope
                            && holds(atom) == Some(true)
                            && holds(other) == Some(true)
                            && sizes_conflict(context, second)
                        {
                            return false;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    true
}

/// Every applicability vector of `items` the brute force finds.
fn brute(
    atoms: &[Atom],
    decl: &Declarations,
    widgets: &[WidgetKind],
    items: &[Item],
) -> HashSet<Vec<bool>> {
    let groups = groups(atoms, decl);
    let mut out = HashSet::new();
    let mut choice = vec![0_usize; groups.len()];
    loop {
        let mut values = vec![false; atoms.len()];
        for (group, chosen) in groups.iter().zip(&choice) {
            for (atom, value) in group.get(*chosen).into_iter().flatten() {
                values[*atom] = *value;
            }
        }
        if admissible(atoms, &values, decl, widgets) {
            out.insert(vector(items, &values));
        }
        if !advance(&mut choice, &groups) {
            break;
        }
    }
    out
}

/// Each item's applicability under `values`.
fn vector(items: &[Item], values: &[bool]) -> Vec<bool> {
    items
        .iter()
        .map(|item| {
            item.as_ref().is_some_and(|needs| {
                needs
                    .iter()
                    .all(|(atom, want)| values.get(*atom) == Some(want))
            })
        })
        .collect()
}

/// The search's vectors, each found once, each from an assignment S12
/// allows.
fn searched(
    atoms: &[Atom],
    decl: &Declarations,
    widgets: &[WidgetKind],
    items: &[Item],
) -> HashSet<Vec<bool>> {
    let space = Space::new(atoms, decl, widgets);
    let found = space.outcomes(items);
    let vectors: Vec<Vec<bool>> = found.iter().map(|values| vector(items, values)).collect();
    for values in &found {
        assert!(admissible(atoms, values, decl, widgets), "{values:?}");
    }
    let set: HashSet<Vec<bool>> = vectors.iter().cloned().collect();
    assert_eq!(set.len(), vectors.len(), "a vector found twice");
    set
}

const DECLARE: &str = "[preset]\nid = \"probe\"\ngeneration = \"probe-1\"\n[declare]\n";

/// Synthetic declarations: two flavours whose exclusive flag and variant
/// groups differ, or none.
fn declarations(flavored: bool) -> Declarations {
    let body = if flavored {
        "flavors = [\"base\", \"alt\"]\n[declare.widget.button]\n\
         flags = [\"hover\", \"pressed\", \"focused\", \"checked\"]\n\
         variants = [\"@a\", \"@b\", \"@c\"]\n\
         exclusive = [[\"hover\", \"pressed\"]]\n\
         exclusive_variants = [[\"@a\", \"@b\"]]\n\
         [declare.flavor.alt.widget.button]\n\
         exclusive = [[\"pressed\", \"focused\", \"checked\"]]\n\
         exclusive_variants = [[\"@b\", \"@c\"]]\n"
    } else {
        "[declare.widget.button]\nflags = [\"hover\", \"pressed\", \"focused\", \"checked\"]\n\
         variants = [\"@a\", \"@b\", \"@c\"]\nexclusive = [[\"hover\", \"pressed\", \"focused\"]]\n\
         exclusive_variants = [[\"@a\", \"@b\", \"@c\"]]\n"
    };
    read_preset(&format!("{DECLARE}{body}"))
        .unwrap()
        .declarations
}

/// The atoms a random case draws from: every kind S12's exclusions treat.
fn pool(flavored: bool) -> Vec<Atom> {
    let among = SiblingScope::SameKind;
    let edges = |top, left| TiledEdges {
        top,
        right: false,
        bottom: false,
        left,
    };
    let nearest = |size: Option<ControlSize>| {
        let mut context = ContextMatch::new(ContextScope::NearestAmong(vec![WidgetRef::Typed(
            WidgetKind::Button,
        )]));
        if let Some(size) = size {
            context.sizes.all_of.push(size);
        }
        context
    };
    let mut atoms = vec![
        Atom::Cond(Condition::Scheme(Scheme::Light)),
        Atom::Cond(Condition::Scheme(Scheme::Dark)),
        Atom::Cond(Condition::ContrastIncreased),
        Atom::Cond(Condition::ContrastHigh),
        Atom::Cond(Condition::ReducedMotion),
        Atom::Cond(Condition::Maximized),
        Atom::Cond(Condition::Tiled(edges(true, false))),
        Atom::Cond(Condition::Tiled(edges(true, true))),
        Atom::State(StateFlag::Hover),
        Atom::State(StateFlag::Pressed),
        Atom::State(StateFlag::Focused),
        Atom::State(StateFlag::Checked),
        Atom::Variant(VariantRef::Keyed(key!("a"))),
        Atom::Variant(VariantRef::Keyed(key!("b"))),
        Atom::Variant(VariantRef::Keyed(key!("c"))),
        Atom::Size(ControlSize::Small),
        Atom::Size(ControlSize::Large),
        Atom::Dir(Direction::Ltr),
        Atom::Dir(Direction::Rtl),
        Atom::First(among.clone()),
        Atom::Last(among.clone()),
        Atom::Only(among.clone()),
        Atom::Parity(among.clone(), Parity::Odd),
        Atom::Parity(among, Parity::Even),
        Atom::Context(nearest(Some(ControlSize::Small))),
        Atom::Context(nearest(Some(ControlSize::Large))),
        Atom::Context(nearest(None)),
    ];
    if flavored {
        atoms.push(Atom::Cond(Condition::Flavor(key!("base"))));
        atoms.push(Atom::Cond(Condition::Flavor(key!("alt"))));
    }
    atoms
}

/// A random case: at most 12 atoms of the pool, and rules each needing up
/// to four of them, some never applying.
fn case(seed: u64) -> (Vec<Atom>, Declarations, Vec<Item>) {
    let mut state = seed | 1;
    let mut next = |below: usize| {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        usize::try_from(state % u64::try_from(below.max(1)).unwrap()).unwrap()
    };
    let flavored = next(2) == 1;
    let mut pool = pool(flavored);
    let mut atoms = Vec::new();
    let count = 1 + next(12);
    while atoms.len() < count && !pool.is_empty() {
        atoms.push(pool.remove(next(pool.len())));
    }
    let rules = next(14);
    let items = (0..rules)
        .map(|_| {
            if next(10) == 0 {
                return None;
            }
            let width = next(5);
            Some(
                (0..width)
                    .map(|_| (next(atoms.len()), next(3) != 0))
                    .collect(),
            )
        })
        .collect();
    (atoms, declarations(flavored), items)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(600))]

    /// On random rule sets over at most 12 atoms, the search yields exactly
    /// the applicability vectors the brute force yields (S12, decision 5).
    #[test]
    fn outcomes_equal_the_brute_force(seed in any::<u64>()) {
        let (atoms, decl, items) = case(seed);
        let widgets = [WidgetKind::Button];
        prop_assert_eq!(
            searched(&atoms, &decl, &widgets, &items),
            brute(&atoms, &decl, &widgets, &items)
        );
    }
}
