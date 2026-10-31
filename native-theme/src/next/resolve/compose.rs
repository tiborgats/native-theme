//! What one step states at a place, and the steps of a place in the order
//! they compose (S2.2, S6.1, S8).
//!
//! Every block and patch [`Locate`]s a path below itself (the `ThemeParts`
//! derive writes it): the value there, a reference above it that the place
//! lies inside, an `Absent` or `Auto` above it, or nothing. A place's steps
//! are its [`Fold`]s, one per step that states something there, each kept
//! with its position in the stack; the evaluator reads them in order, a
//! `Fields` patch or an `Append` over a reference composed in place with the
//! referenced value's own steps by those positions ([`composed`]).

use std::any::Any;

use super::Pos;
use crate::next::blocks::{BlockPatch, FieldsBlock};
use crate::next::expr::Expr;
use crate::next::toml::read::MapKey;
use crate::next::units::{Axes, Sides};
use crate::next::value::{SurfaceMap, Value};
use crate::next::widgets::{TypedPart, WidgetKind};

/// What a path names in one step's value.
#[derive(Clone, Copy)]
pub(crate) enum Found<'a> {
    /// A block, a patch or a container, which paths below it lead into.
    Node(&'a dyn Locate),
    /// A leaf: a `Value` of a scalar, colour, name or text, a plain value, a
    /// list.
    Leaf(&'a dyn Any),
    /// A reference above the place: the place lies inside what it names,
    /// `rest` segments below it.
    Through {
        /// The reference.
        expr: &'a Expr,
        /// How many of the path's segments lie below the reference.
        rest: usize,
    },
    /// `Absent` or `Auto` above the place: nothing is there.
    Gone,
    /// The step states nothing there.
    Missing,
}

impl<'a> Found<'a> {
    /// What was found, as a value of a concrete type.
    pub(crate) fn any(self) -> Option<&'a dyn Any> {
        match self {
            Found::Node(node) => Some(node),
            Found::Leaf(leaf) => Some(leaf),
            Found::Through { .. } | Found::Gone | Found::Missing => None,
        }
    }

    /// What was found, if it is a `T`.
    pub(crate) fn get<T: Any>(self) -> Option<&'a T> {
        self.any().and_then(<dyn Any>::downcast_ref::<T>)
    }
}

/// A value whose places a path names (S2.2's `path`).
pub(crate) trait Locate: Any {
    /// What `path`, field names and map keys, names below `self`.
    fn locate(&self, path: &[&str]) -> Found<'_>;
}

/// A leaf at the end of the path; nothing below it.
pub(crate) fn leaf<'a, T: Any>(value: &'a T, path: &[&str]) -> Found<'a> {
    if path.is_empty() {
        Found::Leaf(value)
    } else {
        Found::Missing
    }
}

/// The entry `path`'s first segment names in a map, `inner` locating the
/// rest below its value.
#[expect(
    clippy::ptr_arg,
    reason = "a map's value as a step holds it, the type the evaluator downcasts to"
)]
pub(crate) fn entry<'a, K: MapKey + 'static, V: 'static>(
    map: &'a Vec<(K, V)>,
    path: &[&str],
    inner: impl Fn(&'a V, &[&str]) -> Found<'a>,
) -> Found<'a> {
    let Some((first, rest)) = path.split_first() else {
        return Found::Leaf(map);
    };
    map.iter()
        .rev()
        .find(|(key, _)| key.key_name() == *first)
        .map_or(Found::Missing, |(_, value)| inner(value, rest))
}

impl<T: Locate> Locate for Value<T> {
    fn locate(&self, path: &[&str]) -> Found<'_> {
        if path.is_empty() {
            return Found::Node(self);
        }
        match self {
            Value::Stated(value) => value.locate(path),
            Value::Expr(expr) => Found::Through {
                expr,
                rest: path.len(),
            },
            Value::Absent | Value::Auto => Found::Gone,
        }
    }
}

impl<B> Locate for BlockPatch<B>
where
    B: FieldsBlock + Locate,
    B::Patch: Locate,
{
    fn locate(&self, path: &[&str]) -> Found<'_> {
        if path.is_empty() {
            return Found::Node(self);
        }
        match self {
            BlockPatch::Set(value) => value.locate(path),
            BlockPatch::Fields(patch) => patch.locate(path),
        }
    }
}

impl<T: Locate> Locate for SurfaceMap<T> {
    fn locate(&self, path: &[&str]) -> Found<'_> {
        let Some((first, rest)) = path.split_first() else {
            return Found::Node(self);
        };
        self.iter()
            .find(|(set, _)| set.name() == *first)
            .map_or(Found::Missing, |(_, value)| value.locate(rest))
    }
}

impl<T: Any> Locate for Sides<T> {
    fn locate(&self, path: &[&str]) -> Found<'_> {
        match path.split_first() {
            None => Found::Node(self),
            Some((&"top", rest)) => leaf(&self.top, rest),
            Some((&"right", rest)) => leaf(&self.right, rest),
            Some((&"bottom", rest)) => leaf(&self.bottom, rest),
            Some((&"left", rest)) => leaf(&self.left, rest),
            Some(_) => Found::Missing,
        }
    }
}

impl<T: Any> Locate for Axes<T> {
    fn locate(&self, path: &[&str]) -> Found<'_> {
        match path.split_first() {
            None => Found::Node(self),
            Some((&"horizontal", rest)) => leaf(&self.horizontal, rest),
            Some((&"vertical", rest)) => leaf(&self.vertical, rest),
            Some(_) => Found::Missing,
        }
    }
}

/// What a step states values of, and where its expressions are evaluated:
/// the colour roles, the tokens, a typed widget's part or its values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Target {
    /// The colour roles of the scheme in force, and the inputs.
    Roles,
    /// The tokens.
    Tokens,
    /// A typed widget's part.
    Part(TypedPart),
    /// A typed widget's values.
    Values(WidgetKind),
}

/// One step's say at a place, and where the step lies in the stack.
#[derive(Clone, Copy)]
pub(crate) struct Fold<'a> {
    /// The step's place in the stack's order (S8).
    pub(crate) pos: Pos,
    /// What the step states values of.
    pub(crate) target: Target,
    /// What it states at the place.
    pub(crate) found: Found<'a>,
}

impl<'a> Fold<'a> {
    /// What `path` names below this step's value at its place.
    pub(crate) fn below(self, path: &[&str]) -> Fold<'a> {
        let found = match self.found {
            Found::Node(node) => node.locate(path),
            Found::Leaf(value) if path.is_empty() => Found::Leaf(value),
            Found::Through { expr, rest } => Found::Through {
                expr,
                rest: rest.saturating_add(path.len()),
            },
            Found::Gone => Found::Gone,
            Found::Leaf(_) | Found::Missing => Found::Missing,
        };
        Fold {
            pos: self.pos,
            target: self.target,
            found,
        }
    }
}

/// The steps of the place `name` below each of `folds`, the steps stating
/// nothing there left out.
pub(crate) fn children<'a>(folds: &[Fold<'a>], name: &str) -> Vec<Fold<'a>> {
    folds
        .iter()
        .map(|fold| fold.below(&[name]))
        .filter(|fold| !matches!(fold.found, Found::Missing))
        .collect()
}

/// The steps of a place whose step at `at` set it to a reference: the
/// referenced value's own steps `referenced`, composed with the steps after
/// `at` that patch it (`items[at + 1..]`), in R2.4's order (S2.2): the
/// referenced value's bases, then the patches' bases, so a base never comes
/// after a rule, then every rule in the stack's order, the referenced value's
/// own first within one rule.
pub(crate) fn composed<'a>(
    referenced: &[Fold<'a>],
    items: &[Fold<'a>],
    at: usize,
) -> Vec<Fold<'a>> {
    let kept = items.get(at.saturating_add(1)..).unwrap_or_default();
    let bases = |folds: &[Fold<'a>]| {
        folds
            .iter()
            .filter(|fold| !fold.pos.rule)
            .copied()
            .collect::<Vec<_>>()
    };
    let mut out = bases(referenced);
    out.extend(bases(kept));
    let mut rules: Vec<(Fold<'a>, bool)> = referenced
        .iter()
        .filter(|fold| fold.pos.rule)
        .map(|fold| (*fold, false))
        .chain(
            kept.iter()
                .filter(|fold| fold.pos.rule)
                .map(|fold| (*fold, true)),
        )
        .collect();
    rules.sort_by_key(|(fold, patch)| (fold.pos, *patch));
    out.extend(rules.into_iter().map(|(fold, _)| fold));
    out
}
