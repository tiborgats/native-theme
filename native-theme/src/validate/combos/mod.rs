//! S12's checks over combinations: what a value can resolve to under the
//! combinations of conditions, states and the element's other selector
//! parts its rules allow (S12's *Combinations, conservatively*).
//!
//! Each check is decided per combination of rule applicability, never field
//! by field: for the places a check reads, it gathers the rules reaching
//! them (the rules stating one of them, and, following each expression's
//! references, the rules stating what those name), finds one assignment
//! of the atoms those rules' selectors name for each distinct vector of the
//! rules' applicability that S12's exclusions allow (the M1a design's
//! decision 5, `outcomes`), and evaluates the places under each as the
//! query evaluates them (S2.2, S8): bases,
//! then the rules that apply, in the stack's order, a `Fields` patch over a
//! reference composed lazily, interleaved with the referenced value's own
//! rules by position.

#[cfg(test)]
mod bench;
mod inputs;
mod motion;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use super::LayerKind;
use super::outcomes::{Item, Space, scheme_and_flavors};
use super::stack::{
    BindingOp, Fact, ItemKey, Op, OpKind, Pos, Source, Stack, Stacked, Target, Unit, Val, below,
    is_above, part_info, tail, widget_of,
};
use crate::blocks::part_style::RoleRef;
use crate::declare::{DataName, DataOwner, Declarations};
use crate::error::{Problem, ProblemKind};
use crate::expr::{ElementDatum, Expr, Func, Literal, MAX_CHAIN, Ref};
use crate::key::Key;
use crate::layer::Layer;
use crate::presets::PresetId;
use crate::registry::{self, BlockForm, FieldInfo, FieldType, PartInfo};
use crate::resolve::evaluate::style_font_size;
use crate::roles::SurfaceSet;
use crate::selector::{
    Condition, ContextMatch, ContextScope, ControlSize, Density, Direction, Parity, PartRef,
    Scheme, Selector, SetMatch, SiblingMatch, SiblingScope, SizeClass, StateFlag, VariantRef,
    WidgetOrPart, WidgetRef,
};
use crate::widgets::{Shown, TypedPart, WidgetKind};

/// What a rule's selector names that a combination assigns (S12): a
/// condition, a state flag, a variant, a control size, a density, a
/// direction, a size class, a place among siblings, a context match.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Atom {
    /// A condition.
    Cond(Condition),
    /// One of the element's state flags.
    State(StateFlag),
    /// One of the element's variants.
    Variant(VariantRef),
    /// The element's control size.
    Size(ControlSize),
    /// The element's density.
    Density(Density),
    /// The element's direction.
    Dir(Direction),
    /// The element's size class on an axis (`true`: vertical).
    Class(bool, SizeClass),
    /// The element is the first among these siblings.
    First(SiblingScope),
    /// The last.
    Last(SiblingScope),
    /// The only one.
    Only(SiblingScope),
    /// Its parity among these siblings.
    Parity(SiblingScope, Parity),
    /// Its adjacent sibling, after it (`true`) or before it, matches.
    Sibling(bool, SiblingScope, SiblingMatch),
    /// An ancestor matches.
    Context(ContextMatch),
    /// The flag is the latest of the element's recency set (S7.2).
    Latest(Key, StateFlag),
}

/// The atoms' assignment and what it gives: each reached rule's
/// applicability, the scheme and the flavour in force.
#[derive(Clone, Debug)]
pub(super) struct World {
    /// Per rule of the stack: whether it applies; a rule the check does not
    /// reach never does.
    applies: Vec<bool>,
    /// The scheme in force.
    scheme: Scheme,
    /// The flavour in force, where the atoms choose one.
    flavor: Option<Key>,
    /// The atoms' values, in the order the check gathered them.
    values: Vec<bool>,
}

/// The rules an evaluation under one world reads, in the order it first
/// reads each ([`Stack::applies`]): an evaluation reads nothing else of the
/// world but its scheme and flavour, so another world agreeing with it on
/// these and on each rule read evaluates the same way.
#[derive(Debug, Default)]
pub(super) struct Reads {
    /// The world read, while one is.
    world: Option<*const World>,
    /// Each rule read, with its applicability.
    rules: Vec<(usize, bool)>,
    /// The world was read whole (its atoms' values), or copied.
    whole: bool,
    /// Per rule, whether it was read.
    seen: Vec<bool>,
}

impl Reads {
    fn read(&mut self, world: &World, rule: usize) {
        if self.world != Some(std::ptr::from_ref(world)) {
            return;
        }
        if self.seen.len() <= rule {
            self.seen.resize(rule.saturating_add(1), false);
        }
        if let Some(seen) = self.seen.get_mut(rule)
            && !*seen
        {
            *seen = true;
            let applies = world.applies.get(rule).copied().unwrap_or(false);
            self.rules.push((rule, applies));
        }
    }

    fn read_whole(&mut self, world: &World) {
        if self.world == Some(std::ptr::from_ref(world)) {
            self.whole = true;
        }
    }
}

/// What evaluations under worlds gave, as a decision tree over the rules
/// they read (S12's checks are deterministic, [`Reads`]): a world whose path
/// through it ends at a leaf evaluates as the earlier one did, to the
/// leaf's value.
struct Decided<T> {
    /// The tree of each scheme and flavour: its nodes, the first the root.
    trees: Vec<(Scheme, Option<Key>, Vec<Branch<T>>)>,
}

/// A node of [`Decided`]'s tree: a leaf, or the rule read next and the node
/// of each of its values.
enum Branch<T> {
    Leaf(T),
    Read(usize, [Option<usize>; 2]),
}

impl<T> Default for Decided<T> {
    fn default() -> Self {
        Decided { trees: Vec::new() }
    }
}

impl<T> Decided<T> {
    /// What an evaluation under `world` gave, where one was done.
    fn get(&self, world: &World) -> Option<&T> {
        let (_, _, nodes) = self
            .trees
            .iter()
            .find(|(scheme, flavor, _)| *scheme == world.scheme && *flavor == world.flavor)?;
        let mut at = 0_usize;
        loop {
            match nodes.get(at)? {
                Branch::Leaf(value) => return Some(value),
                Branch::Read(rule, next) => {
                    let applies = world.applies.get(*rule).copied().unwrap_or(false);
                    at = next.get(usize::from(applies)).copied().flatten()?;
                }
            }
        }
    }

    /// Records that an evaluation under `world`, reading `rules` in order,
    /// gave `value`.
    fn add(&mut self, world: &World, rules: &[(usize, bool)], value: T) {
        let index = match self
            .trees
            .iter()
            .position(|(scheme, flavor, _)| *scheme == world.scheme && *flavor == world.flavor)
        {
            Some(index) => index,
            None => {
                self.trees
                    .push((world.scheme, world.flavor.clone(), Vec::new()));
                self.trees.len().saturating_sub(1)
            }
        };
        let Some((_, _, nodes)) = self.trees.get_mut(index) else {
            return;
        };
        if nodes.is_empty() {
            match rules.first() {
                Some((rule, _)) => nodes.push(Branch::Read(*rule, [None, None])),
                None => {
                    nodes.push(Branch::Leaf(value));
                    return;
                }
            }
        }
        let mut at = 0_usize;
        for (read, (rule, applies)) in rules.iter().enumerate() {
            let len = nodes.len();
            let Some(Branch::Read(held, next)) = nodes.get_mut(at) else {
                return;
            };
            if held != rule {
                return;
            }
            let Some(slot) = next.get_mut(usize::from(*applies)) else {
                return;
            };
            if let Some(node) = slot {
                at = *node;
                continue;
            }
            *slot = Some(len);
            match rules.get(read.saturating_add(1)) {
                Some((rule, _)) => nodes.push(Branch::Read(*rule, [None, None])),
                None => {
                    nodes.push(Branch::Leaf(value));
                    return;
                }
            }
            at = len;
        }
    }
}

/// What an evaluation asked of the stack's sources (S12's checks read them
/// through these alone): the sources of a target stating a place at or
/// above a path, and below it where `subtree`; a part's presence; a part's
/// bindings and surface sets; a target's keyed items. A layer none of whose
/// sources answers an evaluation's questions leaves it as it was.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) enum Query {
    Place(Target, String, bool),
    Shown(TypedPart),
    Bindings(TypedPart),
    Items(Target),
}

impl Query {
    /// Whether `source` is among the sources the question reads.
    fn answered_by(&self, source: &Source) -> bool {
        match self {
            Query::Place(target, q, subtree) => {
                source.target == *target
                    && source.ops.iter().any(|op| {
                        op.path == *q
                            || is_above(&op.path, q)
                            || (*subtree && is_above(q, &op.path))
                    })
            }
            Query::Shown(part) => {
                source.target == Target::Part(*part)
                    && source
                        .ops
                        .iter()
                        .any(|op| matches!(op.kind, OpKind::Shown(_)))
            }
            Query::Bindings(part) => {
                source.target == Target::Part(*part)
                    && (!source.bindings.is_empty() || !source.surfaces.is_empty())
            }
            Query::Items(target) => source.target == *target && !source.items.is_empty(),
        }
    }
}

/// What the checks of one stack's layers, each over the layers below it,
/// found per expression step, kept from one layer's check to the next
/// ([`Checks::expressions`]): a step a later layer's sources answer none of
/// the questions of evaluates as it did, so its problems are taken as they
/// were, not found again.
#[derive(Default)]
pub(crate) struct Reused {
    memory: RefCell<Option<Memory>>,
}

/// [`Reused`]'s steps, and what they were found over: the layers, each
/// with what it is to the stack and what S8 leaves out of it, the preset
/// and its declarations.
struct Memory {
    layers: Vec<(Layer, LayerKind, Option<crate::resolve::LeftOut>)>,
    preset: Option<PresetId>,
    declarations: Declarations,
    steps: HashMap<StepKey, Memo>,
}

/// An expression step across the stacks of one stack's layers: its layer,
/// its source's place in the stack's order and target, its index there.
type StepKey = (usize, Pos, Target, usize);

/// One step's questions and the problems its evaluation found, each as it
/// found it, the ones found before included.
struct Memo {
    asked: Vec<Query>,
    problems: Vec<(usize, Problem)>,
}

/// What a part is under a world, and the layer and place that says so.
type ShownAt = (Shown, Option<(usize, String)>);

/// What [`Stack::reach`] meets at a seed, in order: the rules it reads,
/// the seeds that lead on, and the questions it asks of the sources.
pub(super) struct Expanded {
    rules: Vec<usize>,
    seeds: Vec<Seed>,
    asked: Vec<Query>,
}

/// Which rules a combination's evaluation reads.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) enum Seed {
    /// A place of a target, and below it where `subtree`.
    Field(Target, String, bool),
    /// What a part is drawn as.
    Shown(TypedPart),
    /// A part's bindings.
    Bindings(TypedPart),
}

/// A bound on the sources a lookup reads: those before a position, or the
/// bases only (`active(…)`, a referenced value's under a base's patch).
#[derive(Clone, Copy, Debug, PartialEq)]
struct Bound {
    before: Option<Pos>,
    bases_only: bool,
}

impl Bound {
    const ALL: Bound = Bound {
        before: None,
        bases_only: false,
    };

    fn admits(self, pos: Pos) -> bool {
        (!self.bases_only || !pos.rule) && self.before.is_none_or(|before| pos < before)
    }

    /// The bound a referenced value is read under (S2.2): its bases first,
    /// before every patch kept on the reference; its rules each at its
    /// place, so before a rule's patch only the rules before it.
    fn referent(self, bases_only: bool) -> Bound {
        let bases_only =
            self.bases_only || bases_only || self.before.is_some_and(|before| !before.rule);
        Bound {
            before: if bases_only { None } else { self.before },
            bases_only,
        }
    }
}

/// Where an expression is evaluated.
#[derive(Clone, Copy, Debug)]
struct Cx<'s> {
    target: Target,
    /// The place it is stated at.
    path: &'s str,
    pos: Pos,
    /// The `TextStyle` holding it, where one does.
    text_style: Option<&'s str>,
    /// A binding's value, evaluated in the scope outside the binding: its
    /// part's own bindings are not in scope.
    in_binding: bool,
}

/// A value a lookup found.
#[derive(Clone, Copy, Debug)]
enum Got<'s> {
    Absent,
    Auto,
    Expr(&'s Expr),
    Stated(Fact),
}

/// A value a lookup found, and where.
#[derive(Clone, Copy, Debug)]
struct Found<'s> {
    got: Got<'s>,
    /// The place it is stated at, as a problem names it.
    at: &'s str,
    cx: Cx<'s>,
    /// The source's layer.
    layer: usize,
    /// The step, by its source and place there; `None` for a binding.
    id: Option<(usize, usize)>,
    /// The value came through a reference above the place looked up.
    via: bool,
    /// The value is set at a place above the place looked up: a block
    /// holding it set whole.
    above: bool,
}

/// What a reference reads.
enum Place<'s> {
    /// A place of a target, under a bound.
    Field(Target, String, Bound),
    /// A part's colour binding.
    Binding(&'s Source, &'s BindingOp),
    /// A value a check reads as stated: element data, an input, the scheme.
    Stated,
}

/// Which bindings a role lookup follows (S2.2, S3.3).
#[derive(Clone, Copy, Debug, PartialEq)]
enum Bindings {
    /// Every binding in scope.
    All,
    /// The `Surface(S)` bindings only: `active(…)`'s.
    Surfaces,
    /// None: the colour roles every binding can fall back to.
    None,
}

/// An expression's step, as [`Checks::expressions`] checks it: its id, its
/// source and place, the worlds of the rules its evaluation reaches, and
/// those of them it is in effect in, by index.
struct Step<'s> {
    id: (usize, usize),
    source: &'s Source,
    op: &'s Op,
    expr: &'s Expr,
    worlds: Rc<Vec<World>>,
    in_effect: Vec<usize>,
    /// The questions its evaluation asks of the sources.
    asked: HashSet<Query>,
    /// Its problems as an earlier layer's check found them, taken as they
    /// were; `None`: found again.
    reused: Option<Vec<(usize, Problem)>>,
}

/// The expression steps of one target sharing their worlds, by place.
type Places<'s> = Vec<(&'s str, Vec<usize>)>;

/// The steps a place's value is found at, as [`Stack::value_at`] finds it.
type FoundAt = Vec<Option<(usize, usize)>>;

/// The sources of a target stating a place at or above a path, each with
/// its steps there ([`Stack::stating`]).
pub(super) type Stating = Rc<[(usize, Vec<usize>)]>;

/// A node of the cycle search: a place, and the step found there.
type Node = (Target, String, Option<(usize, usize)>);

/// What S3.3's step 1 finds for `surface.x`.
enum Answer<'s> {
    /// A `Role { Surface(x) }` binding.
    Role(Place<'s>),
    /// A `Surface(S)` binding: the lookup becomes `role.S.x`.
    Set(SurfaceSet),
}

/// How deep a lookup follows references before it stops: past any chain
/// S12 allows, so a cycle ends a lookup instead of the stack.
const MAX_FOLLOW: usize = MAX_CHAIN.saturating_mul(2).saturating_add(4);

/// Every problem the stack's combinations show (S12), each with the layer
/// stating the place it names.
pub(super) fn check(stack: &Stack<'_>, reused: Option<&Reused>) -> Vec<(usize, Problem)> {
    let mut out = Checks {
        stack,
        problems: Vec::new(),
        raw: None,
    };
    out.expressions(reused);
    out.marks();
    out.indication_boxes();
    out.drawn_as_cycles();
    out.redrawn();
    out.text_roles();
    out.shapes();
    out.ellipses();
    out.focus_corners();
    out.concentric();
    out.entries();
    out.motions();
    out.css_curves();
    out.zeros();
    out.input_cycles();
    out.problems
}

struct Checks<'s, 'a> {
    stack: &'s Stack<'a>,
    /// Each problem, with the layer stating the place it names.
    problems: Vec<(usize, Problem)>,
    /// Each problem one step's evaluation finds, the ones found before
    /// included, while one is evaluated.
    raw: Option<Vec<(usize, Problem)>>,
}

impl<'a> Stack<'a> {
    fn decl(&self) -> &'a Declarations {
        self.cx.declarations
    }

    fn preset(&self) -> Option<PresetId> {
        self.cx.preset
    }

    fn applies(&self, world: &World, source: &Source) -> bool {
        if let Some(rule) = source.rule {
            self.reads.borrow_mut().read(world, rule);
        }
        let rule = source
            .rule
            .is_none_or(|rule| world.applies.get(rule).copied().unwrap_or(false));
        rule && source.scheme.is_none_or(|scheme| scheme == world.scheme)
    }

    /// `evaluate`, an evaluation under `world`, with the rules it reads
    /// recorded in `decided` beside its value, unless it reads the world
    /// whole.
    fn recorded<T: Clone>(
        &self,
        world: &World,
        decided: &mut Decided<T>,
        evaluate: impl FnOnce() -> T,
    ) -> T {
        let outer = std::mem::replace(
            &mut *self.reads.borrow_mut(),
            Reads {
                world: Some(std::ptr::from_ref(world)),
                ..Reads::default()
            },
        );
        let value = evaluate();
        let reads = std::mem::replace(&mut *self.reads.borrow_mut(), outer);
        if !reads.whole {
            decided.add(world, &reads.rules, value.clone());
        }
        value
    }

    /// The sources of `target` stating a place at or above `q`, in order,
    /// each with those of its steps: a step elsewhere sets nothing there.
    /// Gathered once per place.
    fn stating(&self, target: Target, q: &str) -> Stating {
        self.ask(Query::Place(target, q.to_owned(), false));
        if let Some(held) = self.stating.borrow().get(&(target, q.to_owned())) {
            return Rc::clone(held);
        }
        let numbers: Stating = self
            .of(target)
            .filter_map(|(number, source)| {
                let indices: Vec<usize> = source
                    .ops
                    .iter()
                    .enumerate()
                    .filter(|(_, op)| op.path == q || is_above(&op.path, q))
                    .map(|(index, _)| index)
                    .collect();
                (!indices.is_empty()).then_some((number, indices))
            })
            .collect();
        self.stating
            .borrow_mut()
            .insert((target, q.to_owned()), Rc::clone(&numbers));
        numbers
    }

    /// The places of the keyed items the source `number` states that later
    /// sources applying under `world` replace at every key (S6.1).
    fn replaced<'s>(&self, world: &World, number: usize, source: &'s Source) -> Vec<&'s str> {
        if source.items.iter().any(|item| !item.keys.is_empty()) {
            self.ask(Query::Items(source.target));
        }
        source
            .items
            .iter()
            .filter(|item| {
                let list = item.place.split('[').next().unwrap_or_default();
                !item.keys.is_empty()
                    && item.keys.iter().all(|key| {
                        self.of(source.target).any(|(later, other)| {
                            later > number
                                && self.applies(world, other)
                                && other.items.iter().any(|held| {
                                    held.place.split('[').next() == Some(list)
                                        && held.keys.contains(key)
                                })
                        })
                    })
            })
            .map(|item| item.place.as_str())
            .collect()
    }

    fn roles_of(&self, world: &World) -> Target {
        Target::Roles(world.scheme)
    }

    /// What `q` of `target` resolves to under `world`, reading the sources
    /// `bound` admits: the last step setting it or a place above it, a
    /// reference above it followed to the referenced value, whose own rules
    /// interleave with the patches kept on the reference by position. A
    /// `Fields` patch or an `Append` over a value that resolves `Absent`
    /// sets nothing, the value staying absent (S2.2, S6.1, S6.3).
    fn value_at(
        &self,
        world: &World,
        target: Target,
        q: &str,
        bound: Bound,
        depth: usize,
    ) -> Vec<Found<'_>> {
        let mut local: Option<Found<'_>> = None;
        let mut via: Option<(usize, &Source, &Op)> = None;
        // The place an `Absent` or `Auto` at or above `q` was set at: the
        // steps below it patch nothing (no block is `Auto`, S6.3).
        let mut absent_at: Option<&str> = None;
        // The local value is an `Append`'s, kept on a reference above.
        let mut appended = false;
        for (number, indices) in self.stating(target, q).iter() {
            let number = *number;
            let Some(source) = self.sources.get(number) else {
                continue;
            };
            if !bound.admits(source.pos) || !self.applies(world, source) {
                continue;
            }
            let replaced = self.replaced(world, number, source);
            let ops = indices
                .iter()
                .filter_map(|index| source.ops.get(*index).map(|op| (*index, op)));
            for (index, op) in ops {
                if replaced
                    .iter()
                    .any(|item| op.path == *item || is_above(item, &op.path))
                    || absent_at.is_some_and(|absent| is_above(absent, &op.path))
                {
                    continue;
                }
                let found = |got, above| Found {
                    got,
                    at: &op.at,
                    cx: cx_of(source, op),
                    layer: source.layer,
                    id: Some((number, index)),
                    via: false,
                    above,
                };
                if op.path == q {
                    let got = match &op.kind {
                        OpKind::Set(val) => Some(got_of(val)),
                        OpKind::Append { .. } if absent_at.is_some() => None,
                        // Over a reference the appended layers keep it:
                        // what it resolves to decides (S2.2).
                        OpKind::Append { .. }
                            if local
                                .as_ref()
                                .is_some_and(|held| matches!(held.got, Got::Expr(_))) =>
                        {
                            None
                        }
                        OpKind::Append { .. } => Some(Got::Stated(Fact::Unknown)),
                        OpKind::Touch => Some(Got::Stated(Fact::Nothing)),
                        OpKind::Fields { .. } | OpKind::Clear | OpKind::Shown(_) => None,
                    };
                    // Below a reference, a patch kept on it (S2.2).
                    if let Some(got) = got {
                        absent_at = matches!(got, Got::Absent | Got::Auto).then_some(q);
                        // Appended straight over the reference above, no
                        // local value beneath.
                        appended = matches!(op.kind, OpKind::Append { .. })
                            && (local.is_none() || appended);
                        local = Some(found(got, false));
                    }
                } else if is_above(&op.path, q) {
                    match &op.kind {
                        OpKind::Set(Val::Absent) => {
                            local = Some(found(Got::Absent, true));
                            via = None;
                            absent_at = Some(&op.path);
                        }
                        OpKind::Set(Val::Auto) => {
                            local = Some(found(Got::Auto, true));
                            via = None;
                            absent_at = Some(&op.path);
                        }
                        OpKind::Set(Val::Expr(_)) => {
                            local = None;
                            via = Some((number, source, op));
                            absent_at = None;
                        }
                        OpKind::Set(Val::Stated(_)) | OpKind::Clear => {
                            local = None;
                            via = None;
                            absent_at = None;
                        }
                        OpKind::Fields { .. }
                        | OpKind::Append { .. }
                        | OpKind::Touch
                        | OpKind::Shown(_) => {}
                    }
                }
            }
        }
        let Some((_, source, op)) = via else {
            return local.into_iter().collect();
        };
        let OpKind::Set(Val::Expr(Expr::Ref(reference))) = &op.kind else {
            return local.into_iter().collect();
        };
        if depth >= MAX_FOLLOW {
            return local.into_iter().collect();
        }
        let rest = tail(&op.path, q);
        let mut out = Vec::new();
        for place in self.places(world, reference, cx_of(source, op)) {
            let referents = match place {
                Place::Field(target, path, inner) => self.value_at(
                    world,
                    target,
                    &below(&path, rest),
                    bound.referent(inner.bases_only).and(inner),
                    depth.saturating_add(1),
                ),
                Place::Binding(..) | Place::Stated => Vec::new(),
            };
            if referents.is_empty() {
                out.extend(local);
            }
            for referent in referents {
                let later = referent.cx.pos.rule
                    && local.is_none_or(|held| !held.cx.pos.rule || referent.cx.pos > held.cx.pos);
                // The patches kept on the reference set nothing over a
                // referenced value that resolves `Absent` (S2.2): a block
                // above `q`, or the paint an `Append` appends to.
                let absent =
                    matches!(referent.got, Got::Absent | Got::Auto) && (referent.above || appended);
                match local {
                    Some(_) if !later && !absent => out.extend(local),
                    _ => out.push(Found {
                        via: true,
                        ..referent
                    }),
                }
            }
        }
        out
    }

    /// The places `reference` reads, stated where `cx` says (S2.2, S3.3):
    /// a role through the bindings in scope.
    fn places(&self, world: &World, reference: &Ref, cx: Cx<'_>) -> Vec<Place<'_>> {
        self.places_in(world, reference, cx, Bindings::All)
    }

    /// The places `reference` reads, the bindings `bindings` says followed:
    /// S3.3's two steps over the parts in scope ([`scope_parts`]), each part's
    /// bindings those of its sources that apply, merged by key (S6.1); with
    /// no `Surface(S)` binding in scope, an ancestor outside the widget can
    /// bind any declared set, so `surface.x` reads each.
    fn places_in(
        &self,
        world: &World,
        reference: &Ref,
        cx: Cx<'_>,
        bindings: Bindings,
    ) -> Vec<Place<'_>> {
        let joined = |path: &[Key]| path.iter().map(Key::as_str).collect::<Vec<_>>().join(".");
        let roles = self.roles_of(world);
        let field = |target, path: String| Place::Field(target, path, Bound::ALL);
        let chain = match bindings {
            Bindings::None => Vec::new(),
            Bindings::All | Bindings::Surfaces => scope_parts(cx),
        };
        let roles_bound = bindings == Bindings::All;
        // Step 2: the nearest `Role` binding of exactly that role, else the
        // colour roles' value.
        let role = |held: &dyn Fn(&RoleRef) -> bool, path: String| {
            roles_bound
                .then(|| self.role_binding(world, &chain, held))
                .flatten()
                .unwrap_or_else(|| field(roles, path))
        };
        let in_set = |set: SurfaceSet, x: &str| {
            role(
                &|held| matches!(held, RoleRef::Set(s, r) if *s == set && r.path() == x),
                format!("surfaces.{}.{x}", set.name()),
            )
        };
        match reference {
            Ref::Role { set, path } => vec![in_set(*set, &joined(path))],
            Ref::GlobalRole(path) => {
                let x = joined(path);
                vec![role(
                    &|held| matches!(held, RoleRef::Global(r) if r.path() == x),
                    format!("global.{x}"),
                )]
            }
            Ref::KeyedRole(key) => vec![role(
                &|held| matches!(held, RoleRef::Keyed(k) if k == key),
                format!("keyed.{key}"),
            )],
            // Step 1: the nearest `Role { Surface(x) }` or `Surface(S)`
            // binding decides; `Surface(S)` makes it `role.S.x`.
            Ref::Surface(path) => {
                let x = joined(path);
                match self.surface_binding(world, &chain, &x, roles_bound) {
                    Some(Answer::Role(place)) => vec![place],
                    Some(Answer::Set(set)) => vec![in_set(set, &x)],
                    None => self
                        .decl()
                        .surface_sets
                        .iter()
                        .map(|set| in_set(*set, &x))
                        .collect(),
                }
            }
            // The role found through the `Surface(S)` bindings only, its
            // value from the bases.
            Ref::Active(inner) => {
                let inner_bindings = match bindings {
                    Bindings::None => Bindings::None,
                    Bindings::All | Bindings::Surfaces => Bindings::Surfaces,
                };
                self.places_in(world, inner, cx, inner_bindings)
                    .into_iter()
                    .map(|place| match place {
                        Place::Field(target, path, _) => Place::Field(
                            target,
                            path,
                            Bound {
                                before: None,
                                bases_only: true,
                            },
                        ),
                        other => other,
                    })
                    .collect()
            }
            Ref::Token(path) => vec![field(Target::Tokens, joined(path))],
            // In a keyframe's patch, the field's value on the part it
            // resolves over (S2.7).
            Ref::Base => match self.keyframe_at(cx.target, cx.pos, cx.path) {
                Some((Some(part), field)) => {
                    vec![Place::Field(Target::Part(part), field, Bound::ALL)]
                }
                Some((None, _)) => Vec::new(),
                None => vec![Place::Field(
                    cx.target,
                    cx.path.to_owned(),
                    Bound {
                        before: Some(cx.pos),
                        bases_only: false,
                    },
                )],
            },
            Ref::FontSize | Ref::LineHeight => {
                let field_name = if *reference == Ref::FontSize {
                    "font.size"
                } else {
                    "line_height"
                };
                match (cx.text_style, cx.target) {
                    (Some(style), target) => vec![field(target, below(style, field_name))],
                    (None, Target::Part(_)) => vec![field(cx.target, below("text", field_name))],
                    (None, _) => vec![Place::Stated],
                }
            }
            Ref::Part(path) => match cx.target {
                Target::Part(_) => vec![field(cx.target, joined(path))],
                _ => vec![Place::Stated],
            },
            Ref::Widget { part, path } => {
                let widget = match cx.target {
                    Target::Part(own) => widget_of(own),
                    Target::Values(widget) => widget,
                    Target::Roles(_) | Target::Tokens => return vec![Place::Stated],
                };
                // `.font_size` and `.line_height` are that part's text's
                // (S2.2), its other font metrics the element's data.
                let path = match joined(path).as_str() {
                    "font_size" => "text.font.size".to_owned(),
                    "line_height" => "text.line_height".to_owned(),
                    other => other.to_owned(),
                };
                match TypedPart::from_names(widget, part.as_str()) {
                    Some(other) if !is_metric_name(&path) => {
                        vec![field(Target::Part(other), path)]
                    }
                    _ => vec![Place::Stated],
                }
            }
            Ref::Values(name) => match cx.target {
                Target::Part(own) => vec![field(Target::Values(widget_of(own)), name.to_string())],
                Target::Values(_) => vec![field(cx.target, name.to_string())],
                Target::Roles(_) | Target::Tokens => vec![Place::Stated],
            },
            Ref::Input(_)
            | Ref::Scheme
            | Ref::Element(_)
            | Ref::BodyFontHeight
            | Ref::TextScale => vec![Place::Stated],
        }
    }

    /// S3.3's step 2: the `Role` binding of a role `held` matches that the
    /// nearest part of `chain` stating one binds, under `world`.
    fn role_binding(
        &self,
        world: &World,
        chain: &[TypedPart],
        held: &dyn Fn(&RoleRef) -> bool,
    ) -> Option<Place<'_>> {
        chain.iter().find_map(|part| {
            self.ask(Query::Bindings(*part));
            let mut last = None;
            for (_, source) in self.of(Target::Part(*part)) {
                if !source.bindings.iter().any(|binding| held(&binding.role))
                    || !self.applies(world, source)
                {
                    continue;
                }
                for binding in &source.bindings {
                    if held(&binding.role) {
                        last = Some(Place::Binding(source, binding));
                    }
                }
            }
            last
        })
    }

    /// S3.3's step 1 for `surface.x`: the nearest part of `chain` binding
    /// `Role { Surface(x) }` (where `roles`) or a surface set decides, under
    /// `world`; one part's `Role { Surface(x) }` binding, the role itself,
    /// before its surface set.
    fn surface_binding(
        &self,
        world: &World,
        chain: &[TypedPart],
        x: &str,
        roles: bool,
    ) -> Option<Answer<'_>> {
        chain.iter().find_map(|part| {
            self.ask(Query::Bindings(*part));
            let mut role = None;
            let mut set = None;
            for (_, source) in self.of(Target::Part(*part)) {
                if (source.surfaces.is_empty() && (!roles || source.bindings.is_empty()))
                    || !self.applies(world, source)
                {
                    continue;
                }
                for binding in source.bindings.iter().filter(|_| roles) {
                    if matches!(&binding.role, RoleRef::Surface(r) if r.path() == x) {
                        role = Some(Place::Binding(source, binding));
                    }
                }
                if let Some(last) = source.surfaces.last() {
                    set = Some(*last);
                }
            }
            role.map(Answer::Role).or(set.map(Answer::Set))
        })
    }

    /// The keyframe whose patch holds `path`, a place the source of `target`
    /// at `pos` states (S2.7): the part its patch resolves over (`None`: no
    /// typed widget's part, whose fields are all `None`) and the place below
    /// its patch.
    fn keyframe_at(
        &self,
        target: Target,
        pos: Pos,
        path: &str,
    ) -> Option<(Option<TypedPart>, String)> {
        self.sources
            .iter()
            .filter(|source| source.target == target && source.pos == pos)
            .flat_map(|source| &source.keyframes)
            .find_map(|(place, part)| {
                let patch = below(place, "patch");
                if !is_above(&patch, path) {
                    return None;
                }
                let part = match part {
                    Some(PartRef::Typed(part)) => Some(*part),
                    Some(PartRef::Component { .. }) => None,
                    None => match target {
                        Target::Part(part) => Some(part),
                        Target::Roles(_) | Target::Tokens | Target::Values(_) => None,
                    },
                };
                Some((part, tail(&patch, path).to_owned()))
            })
    }

    /// Whether a keyframe's field at `field` of `part` is evaluated under
    /// `world` (S2.7, S12): the part is drawn as itself and the field's value
    /// on it, its references followed, resolves to something other than
    /// `Absent`.
    fn keyframe_drawn(&self, world: &World, part: Option<TypedPart>, field: &str) -> bool {
        let Some(part) = part else {
            return false;
        };
        if !matches!(self.shown(world, part).0, Shown::Drawn) {
            return false;
        }
        let ends: Vec<Found<'_>> = self
            .value_at(world, Target::Part(part), field, Bound::ALL, 0)
            .into_iter()
            .flat_map(|found| self.follow(world, found).0)
            .collect();
        !ends.is_empty() && !ends.iter().any(|end| matches!(end.got, Got::Absent))
    }

    /// What a part is drawn as under `world`: its last stated presence
    /// (`Absent`, `Present`, `DrawnAs`), drawn where none is, and where
    /// that is stated.
    fn shown(&self, world: &World, part: TypedPart) -> (&Shown, Option<(usize, &str)>) {
        self.ask(Query::Shown(part));
        let mut out: (&Shown, Option<(usize, &str)>) = (&Shown::Drawn, None);
        for (_, source) in self.of(Target::Part(part)) {
            if !source
                .ops
                .iter()
                .any(|op| matches!(op.kind, OpKind::Shown(_)))
                || !self.applies(world, source)
            {
                continue;
            }
            for op in &source.ops {
                if let OpKind::Shown(shown) = &op.kind {
                    out = (shown, Some((source.layer, &op.at)));
                }
            }
        }
        out
    }

    /// The values a found value resolves to, plain references followed
    /// (calls and stated values ending the chain), and how many references
    /// the longest chain followed.
    fn follow<'s>(&'s self, world: &World, found: Found<'s>) -> (Vec<Found<'s>>, usize) {
        self.follow_to(world, found, None)
    }

    /// [`Stack::follow`]'s ends; with `fonts`, a reference that reaches a
    /// text style's font size, line height or spacing ends at that place,
    /// added there: the text scaling scales the value there by the place's
    /// kind and its style (S9.4), which the value stated further on does
    /// not carry.
    fn follow_to<'s>(
        &'s self,
        world: &World,
        found: Found<'s>,
        mut fonts: Option<&mut Vec<(Target, String)>>,
    ) -> (Vec<Found<'s>>, usize) {
        let mut done = Vec::new();
        let mut longest = 0_usize;
        let mut pending = vec![(found, 0_usize)];
        while let Some((found, hops)) = pending.pop() {
            longest = longest.max(hops);
            let Got::Expr(Expr::Ref(reference)) = found.got else {
                done.push(found);
                continue;
            };
            if hops > MAX_CHAIN {
                done.push(found);
                continue;
            }
            let mut next = Vec::new();
            let mut stopped = false;
            for place in self.places(world, reference, found.cx) {
                match (place, fonts.as_deref_mut()) {
                    (Place::Field(target, path, _), Some(fonts)) if font_place(&path) => {
                        fonts.push((target, path));
                        stopped = true;
                    }
                    (Place::Field(target, path, bound), _) => {
                        next.extend(self.value_at(world, target, &path, bound, 0));
                    }
                    (Place::Binding(source, binding), _) => {
                        next.push(binding_found(source, binding));
                    }
                    (Place::Stated, _) => {}
                }
            }
            if next.is_empty() && !stopped {
                done.push(Found {
                    got: Got::Stated(Fact::Unknown),
                    ..found
                });
            }
            pending.extend(next.into_iter().map(|f| (f, hops.saturating_add(1))));
        }
        (done, longest)
    }

    /// The value at `q` of `target` with every plain reference followed.
    fn effective(&self, world: &World, target: Target, q: &str) -> Vec<Found<'_>> {
        self.value_at(world, target, q, Bound::ALL, 0)
            .into_iter()
            .flat_map(|found| self.follow(world, found).0)
            .collect()
    }

    /// The rules the evaluation of `seeds` reads: those stating a place a
    /// seed names, above it or (for a subtree) below it, and, following
    /// every expression's references and the places a reference above a
    /// seed leads to, those stating what they read.
    fn reach(&self, seeds: Vec<Seed>) -> Vec<usize> {
        let mut rules: Vec<usize> = Vec::new();
        let mut seen: HashSet<Seed> = HashSet::new();
        let mut pending = seeds;
        while let Some(seed) = pending.pop() {
            if seen.contains(&seed) {
                continue;
            }
            let expanded = self.expanded(&seed);
            seen.insert(seed);
            for rule in &expanded.rules {
                if !rules.contains(rule) {
                    rules.push(*rule);
                }
            }
            pending.extend(expanded.seeds.iter().cloned());
        }
        rules
    }

    /// What [`Stack::reach`] meets at `seed`, in order: the rules it reads
    /// and the seeds that lead on. Gathered once per seed.
    fn expanded(&self, seed: &Seed) -> Rc<Expanded> {
        let held = self.expanded.borrow().get(seed).map(Rc::clone);
        let expanded = held.unwrap_or_else(|| {
            let outer = self.ask_begin();
            let (rules, seeds) = self.expand(seed);
            let asked = self.ask_end(outer).into_iter().collect();
            let expanded = Rc::new(Expanded {
                rules,
                seeds,
                asked,
            });
            self.expanded
                .borrow_mut()
                .insert(seed.clone(), Rc::clone(&expanded));
            expanded
        });
        for query in &expanded.asked {
            self.ask(query.clone());
        }
        expanded
    }

    /// Records `query` while an evaluation's questions are recorded.
    fn ask(&self, query: Query) {
        if let Some(asked) = self.asked.borrow_mut().as_mut() {
            asked.insert(query);
        }
    }

    /// Starts recording the questions asked, returning the record of the
    /// evaluation around, if any.
    fn ask_begin(&self) -> Option<HashSet<Query>> {
        self.asked.replace(Some(HashSet::new()))
    }

    /// The questions asked since [`Stack::ask_begin`], which also count as
    /// asked by the evaluation around, whose record `outer` is.
    fn ask_end(&self, outer: Option<HashSet<Query>>) -> HashSet<Query> {
        let asked = self.asked.replace(outer).unwrap_or_default();
        if let Some(around) = self.asked.borrow_mut().as_mut() {
            around.extend(asked.iter().cloned());
        }
        asked
    }

    fn expand(&self, seed: &Seed) -> (Vec<usize>, Vec<Seed>) {
        let mut rules: Vec<usize> = Vec::new();
        let mut pending: Vec<Seed> = Vec::new();
        let any = self.any_world();
        match seed {
            Seed::Shown(part) => {
                // Where every step states the part drawn, it is drawn
                // under every world: no rule decides it.
                let varies = self.of(Target::Part(*part)).any(|(_, source)| {
                    source.ops.iter().any(
                        |op| matches!(&op.kind, OpKind::Shown(shown) if *shown != Shown::Drawn),
                    )
                });
                if !varies {
                    return (rules, pending);
                }
                for (_, source) in self.of(Target::Part(*part)) {
                    let shows = source
                        .ops
                        .iter()
                        .any(|op| matches!(op.kind, OpKind::Shown(_)));
                    if let (true, Some(rule)) = (shows, source.rule)
                        && !rules.contains(&rule)
                    {
                        rules.push(rule);
                    }
                }
            }
            Seed::Field(target, q, subtree) => {
                let targets: Vec<Target> = match target {
                    Target::Roles(_) => Scheme::ALL.map(Target::Roles).to_vec(),
                    other => vec![*other],
                };
                for target in targets {
                    // A keyed item holding the place is gone where a
                    // later item replaces it at every key (S6.1).
                    let keys: Vec<&ItemKey> = self
                        .of(target)
                        .flat_map(|(_, source)| &source.items)
                        .filter(|item| item.place == *q || is_above(&item.place, q))
                        .flat_map(|item| &item.keys)
                        .collect();
                    for (_, source) in self.of(target) {
                        let replaces = source
                            .items
                            .iter()
                            .any(|item| item.keys.iter().any(|key| keys.contains(&key)));
                        if replaces
                            && let Some(rule) = source.rule
                            && !rules.contains(&rule)
                        {
                            rules.push(rule);
                        }
                    }
                    for (_, source) in self.of(target) {
                        let related: Vec<&Op> = source
                            .ops
                            .iter()
                            .filter(|op| {
                                !matches!(op.kind, OpKind::Shown(_))
                                    && (op.path == *q
                                        || is_above(&op.path, q)
                                        || (*subtree && is_above(q, &op.path)))
                            })
                            .collect();
                        if related.is_empty() {
                            continue;
                        }
                        if let Some(rule) = source.rule
                            && !rules.contains(&rule)
                        {
                            rules.push(rule);
                        }
                        for op in related {
                            if let OpKind::Set(Val::Expr(expr)) = &op.kind {
                                let rest = if is_above(&op.path, q) {
                                    tail(&op.path, q).to_owned()
                                } else {
                                    String::new()
                                };
                                pending.extend(self.seeds_of(&any, expr, cx_of(source, op), &rest));
                            }
                        }
                    }
                }
            }
            Seed::Bindings(part) => {
                for (_, source) in self.of(Target::Part(*part)) {
                    if source.bindings.is_empty() && source.surfaces.is_empty() {
                        continue;
                    }
                    if let Some(rule) = source.rule
                        && !rules.contains(&rule)
                    {
                        rules.push(rule);
                    }
                    for binding in &source.bindings {
                        if let Val::Expr(expr) = &binding.value {
                            let cx = binding_cx(source);
                            pending.extend(self.seeds_of(&any, expr, cx, ""));
                        }
                    }
                }
            }
        }
        (rules, pending)
    }

    /// The seeds an expression's references give, `rest` below each.
    fn seeds_of(&self, any: &[World], expr: &Expr, cx: Cx<'_>, rest: &str) -> Vec<Seed> {
        let mut out = Vec::new();
        for reference in refs(expr) {
            if let Ref::Widget { part, .. } = reference
                && let Some(other) = target_widget(cx.target)
                    .and_then(|widget| TypedPart::from_names(widget, part.as_str()))
            {
                out.push(Seed::Shown(other));
            }
            if *reference == Ref::TextScale {
                out.push(Seed::Field(
                    Target::Tokens,
                    "text.scaling".to_owned(),
                    false,
                ));
            }
            // A role's bindings in scope, and the colour roles whichever
            // apply (S3.3).
            if is_role(reference) {
                out.extend(scope_parts(cx).into_iter().map(Seed::Bindings));
            }
            for world in any {
                for place in self.places_in(world, reference, cx, Bindings::None) {
                    if let Place::Field(target, path, _) = place {
                        out.push(Seed::Field(target, below(&path, rest), true));
                    }
                }
            }
        }
        out
    }

    /// One world per scheme with every rule applying: the places a
    /// reference can read, whatever applies.
    fn any_world(&self) -> Vec<World> {
        Scheme::ALL
            .into_iter()
            .map(|scheme| World {
                applies: vec![true; self.rules.len()],
                scheme,
                flavor: None,
                values: Vec::new(),
            })
            .collect()
    }

    /// Every distinct world of the rules `seeds` reach (S12): one for each
    /// vector of the rules' applicability, the scheme and the flavour that
    /// an assignment of their atoms honouring S12's exclusions gives.
    fn worlds(&self, seeds: Vec<Seed>) -> Rc<Vec<World>> {
        let mut rules = self.reach(seeds);
        rules.sort_unstable();
        if let Some(worlds) = self.cache.borrow().get(&rules) {
            return Rc::clone(worlds);
        }
        let widgets: Vec<WidgetKind> = rules
            .iter()
            .filter_map(|rule| self.rules.get(*rule).and_then(|info| info.widget))
            .fold(Vec::new(), |mut held, widget| {
                if !held.contains(&widget) {
                    held.push(widget);
                }
                held
            });
        // Each outcome is a distinct vector of the rules' applicability, the
        // scheme and the flavour in force (S12): one world each.
        let (_, worlds) = self.enumerate(&rules, &widgets, true, scheme_and_flavors);
        let worlds = Rc::new(worlds);
        self.cache.borrow_mut().insert(rules, Rc::clone(&worlds));
        worlds
    }

    /// The atoms `rules` name, completed, and one world per distinct
    /// outcome: a vector of the rules' applicability and of the decisions
    /// `outcomes` adds over the atoms, under S12's exclusions with the
    /// groups `widgets` declare, their exclusive flag groups only where
    /// `flag_groups`.
    fn enumerate(
        &self,
        rules: &[usize],
        widgets: &[WidgetKind],
        flag_groups: bool,
        outcomes: impl Fn(&[Atom]) -> Vec<Item>,
    ) -> (Vec<Atom>, Vec<World>) {
        let mut atoms: Vec<Atom> = Vec::new();
        for rule in rules {
            if let Some(info) = self.rules.get(*rule) {
                atoms_of(info.selector, self.preset(), &mut atoms);
            }
        }
        let decl = self.decl();
        complete(&mut atoms, decl, widgets);
        let compiled = self.compile(&atoms, rules);
        let mut items: Vec<Item> = compiled.iter().map(|(_, needs)| needs.clone()).collect();
        items.extend(outcomes(&atoms));
        let space = Space::new(&atoms, decl, widgets, flag_groups);
        let worlds = space
            .outcomes(&items)
            .iter()
            .map(|values| self.world(&atoms, values, &compiled))
            .collect();
        (atoms, worlds)
    }

    /// Each of `rules`' selectors as the atoms' values it needs; `None` for
    /// a rule whose `Preset` condition excludes the current preset.
    fn compile(&self, atoms: &[Atom], rules: &[usize]) -> Vec<Compiled> {
        rules
            .iter()
            .filter_map(|rule| {
                let info = self.rules.get(*rule)?;
                Some((*rule, literals(info.selector, atoms, self.preset())))
            })
            .collect()
    }

    /// `world` as the element of `widget` sees it: its states reduced by
    /// the widget's exclusive groups under the flavour in force, each
    /// group's first held flag kept and the others dropped (S8, S10).
    fn reduced(
        &self,
        atoms: &[Atom],
        world: &World,
        widget: WidgetKind,
        compiled: &[Compiled],
    ) -> World {
        let decl = self.decl();
        let decls = match &world.flavor {
            Some(flavor) => decl.in_flavor(widget, flavor),
            None => decl.in_default_flavor(widget),
        };
        self.reads.borrow_mut().read_whole(world);
        let mut values = world.values.clone();
        for group in decls.exclusive.iter().flatten() {
            let mut kept = false;
            for flag in group {
                let Some(index) = atoms
                    .iter()
                    .position(|atom| *atom == Atom::State(flag.clone()))
                else {
                    continue;
                };
                if let Some(value) = values.get_mut(index)
                    && *value
                {
                    *value = !kept;
                    kept = true;
                }
            }
        }
        if values == world.values {
            return world.clone();
        }
        self.world(atoms, &values, compiled)
    }

    fn world(&self, atoms: &[Atom], values: &[bool], compiled: &[Compiled]) -> World {
        let holds = |condition: &Condition| {
            atoms
                .iter()
                .zip(values)
                .any(|(atom, value)| *value && *atom == Atom::Cond(condition.clone()))
        };
        let scheme = if holds(&Condition::Scheme(Scheme::Dark)) {
            Scheme::Dark
        } else {
            Scheme::Light
        };
        let mut applies = vec![false; self.rules.len()];
        for (rule, needs) in compiled {
            let holds = needs.as_ref().is_some_and(|needs| {
                needs
                    .iter()
                    .all(|(atom, want)| values.get(*atom).copied() == Some(*want))
            });
            if let Some(slot) = applies.get_mut(*rule) {
                *slot = holds;
            }
        }
        let flavor = atoms
            .iter()
            .zip(values)
            .find_map(|(atom, value)| match atom {
                Atom::Cond(Condition::Flavor(flavor)) if *value => Some(flavor.clone()),
                _ => None,
            });
        World {
            applies,
            scheme,
            flavor,
            values: values.to_vec(),
        }
    }
}

impl Bound {
    /// The stricter of two bounds.
    fn and(self, other: Bound) -> Bound {
        let before = match (self.before, other.before) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        };
        Bound {
            before,
            bases_only: self.bases_only || other.bases_only,
        }
    }
}

/// The places at or below `path`, the field `info`, that none of `ops`
/// states (S6.1, S12): stated whole by a step at or above it that sets it
/// (a value, a reference, `Absent`, nothing, a list replaced); where steps
/// lie below a struct block (a `Fields` patch's, a compound's), each of its
/// fields judged in turn; else stated by a step below it (a map's entry, a
/// list's item); an `Append` alone states nothing, having no paint to
/// append to.
fn unstated(ops: &[&Op], path: &str, info: &FieldInfo, out: &mut Vec<String>) {
    let whole = ops.iter().any(|op| {
        (op.path == path || is_above(&op.path, path))
            && matches!(op.kind, OpKind::Set(_) | OpKind::Touch | OpKind::Clear)
    });
    if whole {
        return;
    }
    let patched = ops.iter().any(|op| is_above(path, &op.path));
    match info.ty {
        FieldType::Block {
            ty,
            form: BlockForm::Struct,
            fields,
            ..
        } if patched && !ty.starts_with("Vec<") && !ty.starts_with("SurfaceMap<") => {
            for field in fields {
                unstated(ops, &below(path, field.path), field, out);
            }
        }
        _ if patched => {}
        _ => out.push(path.to_owned()),
    }
}

fn got_of(val: &Val) -> Got<'_> {
    match val {
        Val::Absent => Got::Absent,
        Val::Auto => Got::Auto,
        Val::Expr(expr) => Got::Expr(expr),
        Val::Stated(fact) => Got::Stated(*fact),
    }
}

fn cx_of<'s>(source: &Source, op: &'s Op) -> Cx<'s> {
    Cx {
        target: source.target,
        path: &op.path,
        pos: source.pos,
        text_style: op.text_style.as_deref(),
        in_binding: is_above("bindings", &op.path),
    }
}

fn binding_cx(source: &Source) -> Cx<'static> {
    Cx {
        target: source.target,
        path: "bindings",
        pos: source.pos,
        text_style: None,
        in_binding: true,
    }
}

fn binding_found<'s>(source: &'s Source, binding: &'s BindingOp) -> Found<'s> {
    Found {
        got: got_of(&binding.value),
        at: &binding.at,
        cx: binding_cx(source),
        layer: source.layer,
        id: binding.id,
        via: false,
        above: false,
    }
}

/// Whether `reference` names a colour role, which bindings can answer
/// (S3.3).
fn is_role(reference: &Ref) -> bool {
    matches!(
        reference,
        Ref::Role { .. }
            | Ref::GlobalRole(_)
            | Ref::KeyedRole(_)
            | Ref::Surface(_)
            | Ref::Active(_)
    )
}

/// The parts whose bindings are in scope where `cx` is, nearest first
/// (S2.11, S3.3): a part's own (not for a binding's value, evaluated in the
/// scope outside it), then each part enclosing it in the catalogue, up to
/// the root, whose bindings apply to every part of the widget.
fn scope_parts(cx: Cx<'_>) -> Vec<TypedPart> {
    let Target::Part(own) = cx.target else {
        return Vec::new();
    };
    let widget = widget_of(own);
    let Some(info) = registry::widgets().iter().find(|info| info.kind == widget) else {
        return Vec::new();
    };
    let mut chain = vec![own];
    let mut current = part_info(own);
    for _ in info.parts {
        let Some(part) = current.filter(|part| !part.root) else {
            break;
        };
        let parent = part
            .parent
            .and_then(|name| info.part(name))
            .or_else(|| info.parts.iter().find(|p| p.root));
        match parent.and_then(|parent| TypedPart::from_names(widget, parent.name)) {
            Some(typed) => chain.push(typed),
            None => break,
        }
        current = parent;
    }
    chain.into_iter().skip(usize::from(cx.in_binding)).collect()
}

/// Whether a `widget.<part>.` path names that part's font metric, not a
/// field (S2.2).
fn is_metric(path: &[Key]) -> bool {
    match path {
        [metric] => is_metric_name(metric.as_str()),
        _ => false,
    }
}

/// Whether a `widget.<part>.` name is a font metric of the element's data.
fn is_metric_name(name: &str) -> bool {
    matches!(
        name,
        "char_width"
            | "max_char_width"
            | "font_x_height"
            | "font_height"
            | "font_underline_offset"
            | "font_underline_thickness"
            | "font_strikeout_offset"
            | "font_strikeout_thickness"
    )
}

/// Every reference in `expr`, the tokens its calls read themselves among
/// them (S2.2), and whether it is a call's operand.
fn operands(expr: &Expr) -> Vec<(&Ref, bool)> {
    let mut out = Vec::new();
    let mut pending = vec![(expr, false)];
    while let Some((expr, in_call)) = pending.pop() {
        match expr {
            Expr::Lit(_) => {}
            Expr::Ref(reference) => out.push((reference, in_call)),
            Expr::Call(func, args) => {
                pending.extend(args.iter().map(|arg| (arg, true)));
                out.extend(func.token_operands(args).iter().map(|token| (token, true)));
            }
        }
    }
    out
}

/// Every reference in `expr`.
fn refs(expr: &Expr) -> Vec<&Ref> {
    operands(expr)
        .into_iter()
        .map(|(reference, _)| reference)
        .collect()
}

/// The atoms a selector names, each with the value it needs (S7): each
/// part of a selector is a conjunction of atoms holding or not, an atom
/// named for an exclusion only needing none; `Preset` is the current
/// preset's, no atom, so `None` where it excludes the current preset (the
/// rule never applies).
fn selector_literals(
    selector: &Selector,
    preset: Option<PresetId>,
) -> Option<Vec<(Atom, Option<bool>)>> {
    fn set<T: Clone>(
        set: &SetMatch<T>,
        atom: impl Fn(T) -> Atom,
        out: &mut Vec<(Atom, Option<bool>)>,
    ) {
        out.extend(
            set.all_of
                .iter()
                .map(|item| (atom(item.clone()), Some(true))),
        );
        out.extend(
            set.none_of
                .iter()
                .map(|item| (atom(item.clone()), Some(false))),
        );
    }
    let mut out = Vec::new();
    let mut excluded = false;
    for (conditions, want) in [
        (&selector.conditions.all_of, true),
        (&selector.conditions.none_of, false),
    ] {
        for condition in conditions {
            match condition {
                Condition::Preset(id) => excluded |= (Some(*id) == preset) != want,
                other => out.push((Atom::Cond(other.clone()), Some(want))),
            }
        }
    }
    set(&selector.states, Atom::State, &mut out);
    set(&selector.variants, Atom::Variant, &mut out);
    set(&selector.size.sizes, Atom::Size, &mut out);
    set(&selector.size.densities, Atom::Density, &mut out);
    set(
        &selector.size.size_classes.horizontal,
        |c| Atom::Class(false, c),
        &mut out,
    );
    set(
        &selector.size.size_classes.vertical,
        |c| Atom::Class(true, c),
        &mut out,
    );
    if let Some(direction) = selector.direction {
        out.push((Atom::Dir(direction), Some(true)));
    }
    out.extend(
        selector
            .latest
            .iter()
            .map(|(set, flag)| (Atom::Latest(set.clone(), flag.clone()), Some(true))),
    );
    out.extend(
        selector
            .context
            .iter()
            .map(|context| (Atom::Context(context_as_set(context)), Some(true))),
    );
    out.extend(
        selector
            .not_context
            .iter()
            .map(|context| (Atom::Context(context_as_set(context)), Some(false))),
    );
    if let Some(p) = &selector.position {
        let among = &among_as_set(&p.among);
        // `only` holds exactly when `first` and `last` do (S12): the
        // atoms of both are named with it.
        if p.first.is_some() || p.only.is_some() {
            out.push((Atom::First(among.clone()), p.first));
        }
        if p.last.is_some() || p.only.is_some() {
            out.push((Atom::Last(among.clone()), p.last));
        }
        if let Some(want) = p.only {
            out.push((Atom::Only(among.clone()), Some(want)));
        }
        if let Some(parity) = p.parity {
            out.push((Atom::Parity(among.clone(), parity), Some(true)));
        }
        for (next, sibling) in [(false, &p.previous), (true, &p.next)] {
            if let Some(sibling) = sibling {
                out.push((
                    Atom::Sibling(next, among.clone(), sibling.clone()),
                    Some(true),
                ));
            }
        }
    }
    (!excluded).then_some(out)
}

/// A list of widgets as the set it names: sorted, each once, so two lists
/// of the same widgets are one scope.
fn widgets_as_set(widgets: &[WidgetRef]) -> Vec<WidgetRef> {
    let mut out = widgets.to_vec();
    out.sort();
    out.dedup();
    out
}

/// The siblings `among` counts, its widgets as a set.
fn among_as_set(among: &SiblingScope) -> SiblingScope {
    match among {
        SiblingScope::Kinds(widgets) => SiblingScope::Kinds(widgets_as_set(widgets)),
        other @ (SiblingScope::SameKind | SiblingScope::Any) => other.clone(),
    }
}

/// A context match, the widgets its scope names as a set.
fn context_as_set(context: &ContextMatch) -> ContextMatch {
    let mut out = context.clone();
    if let ContextScope::NearestAmong(widgets) = &context.scope {
        out.scope = ContextScope::NearestAmong(widgets_as_set(widgets));
    }
    out
}

fn push_atom(atoms: &mut Vec<Atom>, atom: Atom) {
    if !atoms.contains(&atom) {
        atoms.push(atom);
    }
}

/// The atoms a selector names, under the current preset `preset` (none
/// for a rule its `Preset` condition excludes, which never applies).
fn atoms_of(selector: &Selector, preset: Option<PresetId>, atoms: &mut Vec<Atom>) {
    for (atom, _) in selector_literals(selector, preset).into_iter().flatten() {
        push_atom(atoms, atom);
    }
}

/// Adds the atoms a one-of choice needs whole: both schemes, both
/// directions, both parities, every declared flavour and every declared
/// size of a container named.
fn complete(atoms: &mut Vec<Atom>, decl: &Declarations, widgets: &[WidgetKind]) {
    for scheme in Scheme::ALL {
        push_atom(atoms, Atom::Cond(Condition::Scheme(scheme)));
    }
    // A recency set named: each member's flag and its being the latest,
    // under every flavour's declarations (S7.2, S12).
    let sets: Vec<Key> = atoms
        .iter()
        .filter_map(|atom| match atom {
            Atom::Latest(set, _) => Some(set.clone()),
            _ => None,
        })
        .collect();
    for set in sets {
        for widget in widgets {
            for section in decl.in_each_flavor(*widget) {
                for (held, flags) in section.recency.iter().flatten() {
                    if *held == set {
                        for flag in flags {
                            push_atom(atoms, Atom::State(flag.clone()));
                            push_atom(atoms, Atom::Latest(set.clone(), flag.clone()));
                        }
                    }
                }
            }
        }
    }
    if atoms.iter().any(|atom| matches!(atom, Atom::Dir(_))) {
        push_atom(atoms, Atom::Dir(Direction::Ltr));
        push_atom(atoms, Atom::Dir(Direction::Rtl));
    }
    let amongs: Vec<SiblingScope> = atoms
        .iter()
        .filter_map(|atom| match atom {
            Atom::Parity(among, _) | Atom::First(among) => Some(among.clone()),
            _ => None,
        })
        .collect();
    for among in amongs {
        if atoms
            .iter()
            .any(|a| matches!(a, Atom::Parity(held, _) if *held == among))
        {
            push_atom(atoms, Atom::Parity(among.clone(), Parity::Odd));
            push_atom(atoms, Atom::Parity(among, Parity::Even));
        }
    }
    if atoms.iter().any(|atom| {
        matches!(
            atom,
            Atom::Cond(Condition::Flavor(_)) | Atom::State(_) | Atom::Variant(_) | Atom::Latest(..)
        )
    }) {
        for flavor in &decl.flavors {
            push_atom(atoms, Atom::Cond(Condition::Flavor(flavor.clone())));
        }
    }
    let containers: Vec<WidgetRef> = atoms
        .iter()
        .filter_map(|atom| match atom {
            Atom::Cond(Condition::ContainerSize { container, .. }) => Some(container.clone()),
            _ => None,
        })
        .collect();
    for container in containers {
        for (held, sizes) in &decl.container_sizes {
            if *held == container {
                for size in sizes {
                    push_atom(
                        atoms,
                        Atom::Cond(Condition::ContainerSize {
                            container: container.clone(),
                            size: size.clone(),
                        }),
                    );
                }
            }
        }
    }
}

/// A rule, and the atoms' values its selector needs (`None`: it never
/// applies).
type Compiled = (usize, Option<Vec<(usize, bool)>>);

/// The atoms' values `selector` needs to match, each atom by its index in
/// `atoms`, which [`atoms_of`] filled from the same selector; `None` where
/// its `Preset` condition excludes the current preset.
fn literals(
    selector: &Selector,
    atoms: &[Atom],
    preset: Option<PresetId>,
) -> Option<Vec<(usize, bool)>> {
    selector_literals(selector, preset).map(|literals| {
        literals
            .into_iter()
            .filter_map(|(atom, want)| {
                let want = want?;
                atoms
                    .iter()
                    .position(|held| *held == atom)
                    .map(|index| (index, want))
            })
            .collect()
    })
}

/// The part's catalogue fields, a target's table.
fn table(target: Target) -> &'static [FieldInfo] {
    match target {
        Target::Roles(_) => registry::colors(),
        Target::Tokens => registry::tokens(),
        Target::Part(part) => part_info(part).map_or(&[], |info| info.fields),
        Target::Values(widget) => registry::widgets()
            .iter()
            .find(|info| info.kind == widget)
            .map_or(&[], |info| info.values),
    }
}

/// The `Value` fields below a block's table, by their paths below it.
fn value_fields(
    fields: &'static [FieldInfo],
    prefix: &str,
    out: &mut Vec<(String, &'static FieldInfo)>,
) {
    for field in fields {
        let path = below(prefix, field.path);
        let written = match field.ty {
            FieldType::Leaf(ty) | FieldType::Block { ty, .. } | FieldType::Patch { ty, .. } => ty,
        };
        if written.starts_with("Value<") {
            out.push((path.clone(), field));
        }
        if let FieldType::Block { ty, fields, .. } = field.ty
            && !ty.starts_with("Vec<")
            && !ty.starts_with("SurfaceMap<")
        {
            value_fields(fields, &path, out);
        }
    }
}

/// The *(element)* part a part is queried with, else the widget's root
/// (S8).
fn element_part(part: TypedPart) -> Option<TypedPart> {
    let widget = widget_of(part);
    let info = registry::widgets()
        .iter()
        .find(|info| info.kind == widget)?;
    let mut current = part_info(part)?;
    for _ in info.parts {
        if current.element || current.root {
            return TypedPart::from_names(widget, current.name);
        }
        match current.parent.and_then(|parent| info.part(parent)) {
            Some(parent) => current = parent,
            None => break,
        }
    }
    let root = info.parts.iter().find(|p| p.root)?;
    TypedPart::from_names(widget, root.name)
}

/// The widget whose part or values `target` is.
fn target_widget(target: Target) -> Option<WidgetKind> {
    match target {
        Target::Part(part) => Some(widget_of(part)),
        Target::Values(widget) => Some(widget),
        Target::Roles(_) | Target::Tokens => None,
    }
}

/// The element's own part a target's expressions are evaluated with
/// (S8): a part's *(element)* part or root, the widget's root for its
/// values.
fn element_of(target: Target) -> Option<TypedPart> {
    match target {
        Target::Part(part) => element_part(part),
        Target::Values(widget) => registry::widgets()
            .iter()
            .find(|info| info.kind == widget)
            .and_then(|info| info.parts.iter().find(|p| p.root))
            .and_then(|root| TypedPart::from_names(widget, root.name)),
        Target::Roles(_) | Target::Tokens => None,
    }
}

/// The corner a reference names: `corners.<corner>` or
/// `focus.corners.<corner>` of a part.
fn corner_of(path: &[Key]) -> Option<&str> {
    let names: Vec<&str> = path.iter().map(Key::as_str).collect();
    match names.as_slice() {
        ["corners", corner] | ["focus", "corners", corner] => Some(corner),
        _ => None,
    }
}

/// The layout fields of a part (S8, S12): those laying the element out or
/// sizing its content.
fn layout_field(target: Target, path: &str) -> bool {
    match target {
        Target::Part(_) => {
            let first = path.split(['.', '[']).next().unwrap_or_default();
            matches!(
                first,
                "padding"
                    | "margin"
                    | "min_width"
                    | "min_height"
                    | "max_width"
                    | "max_height"
                    | "touch_target"
            ) || matches!(
                path,
                "text.font.size"
                    | "text.line_height"
                    | "text.letter_spacing"
                    | "text.word_spacing"
                    | "icon.size"
                    | "glyph.size"
            )
        }
        Target::Values(_) => path.ends_with("_gap"),
        Target::Roles(_) | Target::Tokens => false,
    }
}

fn stated_number(found: &Found<'_>) -> Option<f32> {
    match found.got {
        Got::Stated(Fact::Number(n, _)) => Some(n),
        _ => None,
    }
}

fn variant(found: &Found<'_>) -> Option<&'static str> {
    match found.got {
        Got::Stated(Fact::Variant(name)) => Some(name),
        _ => None,
    }
}

/// Whether `path` names one of a text style's font sizes, by the rule the
/// resolver reads the style from (S9.4).
fn font_place(path: &str) -> bool {
    let names: Vec<&str> = path.split('.').collect();
    style_font_size(&names).is_some()
}

/// One side's length as the inset and offset checks compare it (S2.4,
/// S2.10).
#[derive(Debug)]
enum SideLen {
    /// A sum: of stated lengths in pixels (an absent side 0), and of terms,
    /// a text style's font size or an expression as [`Term`] reads it; a
    /// plain reference reaches the value it follows.
    Known(f32, Vec<Term>),
    /// No layer states it: missing, the preset's check (S2.2), so it is no
    /// difference.
    Unstated,
    /// Anything else (`auto`, or values that differ by combination): it can
    /// differ from any side.
    Unknown,
}

/// An expression with every reference it reads followed, as the query
/// follows it (S12), to the one value the reference resolves to under a
/// world: two terms are equal where they are the same under every input,
/// every DPI and text scaling among them (S9.4).
#[derive(Debug, PartialEq)]
enum Term {
    /// A literal.
    Lit(Literal),
    /// A stated number a reference resolves to, in its unit: one number in
    /// two units is two lengths under some DPI or text scaling.
    Number(f32, Unit),
    /// The value a text style's font size, line height or spacing resolves
    /// to, at its place: equal only to itself at the same place. The text
    /// scaling scales what the place states, a number or an expression's
    /// font size literals, by the place's kind (S9.4: `WinUi` a size alone)
    /// and by its style: `DynamicType` a size or a line height by the
    /// style's role, `AndroidNonLinear` a line height by the style's font
    /// size. Equal kinds and styles make one place, so the place is that
    /// unit in full: exact for one place, conservative across two, whose
    /// roles and font sizes would have to be compared. Read from a length's
    /// place, a font size is always text-scaled (the resolver's
    /// `font_size`, `line_height` and `widget.<part>` operands).
    Font(Target, String),
    /// A call of its arguments' terms.
    Call(Func, Vec<Term>),
    /// A reference followed no further (element data, an input, a binding,
    /// values that differ by combination, or a chain too long): equal only
    /// to itself stated at the same step.
    Opaque(String, Option<(usize, usize)>),
}

impl SideLen {
    /// Two lengths added.
    fn plus(self, other: SideLen) -> SideLen {
        match (self, other) {
            (SideLen::Unknown, _) | (_, SideLen::Unknown) => SideLen::Unknown,
            (SideLen::Unstated, _) | (_, SideLen::Unstated) => SideLen::Unstated,
            (SideLen::Known(a, mut terms), SideLen::Known(b, more)) => {
                terms.extend(more);
                SideLen::Known(a + b, terms)
            }
        }
    }

    /// Whether two known lengths are the same under every input: the same
    /// pixels stated, and the same terms as often, each number in its unit.
    fn same(&self, other: &SideLen) -> bool {
        match (self, other) {
            (SideLen::Known(a, x), SideLen::Known(b, y)) => {
                a == b
                    && x.len() == y.len()
                    && x.iter().all(|term| {
                        x.iter().filter(|held| *held == term).count()
                            == y.iter().filter(|held| *held == term).count()
                    })
            }
            _ => false,
        }
    }

    /// Whether the two sides can differ: an unstated side never does.
    fn can_differ(&self, other: &SideLen) -> bool {
        match (self, other) {
            (SideLen::Unstated, _) | (_, SideLen::Unstated) => false,
            _ => !self.same(other),
        }
    }
}

impl Stack<'_> {
    /// The length the value at `path` of `target` gives under `world`, its
    /// plain references followed, to a text style's font size at most.
    fn side_len(&self, world: &World, target: Target, path: &str) -> SideLen {
        let mut fonts = Vec::new();
        let founds: Vec<Found<'_>> = self
            .value_at(world, target, path, Bound::ALL, 0)
            .into_iter()
            .flat_map(|found| self.follow_to(world, found, Some(&mut fonts)).0)
            .collect();
        let fonts = fonts
            .into_iter()
            .map(|(target, path)| SideLen::Known(0.0, vec![Term::Font(target, path)]));
        let founds = founds.iter().map(|found| match found.got {
            Got::Stated(Fact::Number(n, Unit::Plain)) => SideLen::Known(n, Vec::new()),
            Got::Stated(Fact::Number(n, unit)) => SideLen::Known(0.0, vec![Term::Number(n, unit)]),
            Got::Absent => SideLen::Known(0.0, Vec::new()),
            Got::Expr(expr) => {
                SideLen::Known(0.0, vec![self.term(world, expr, found.cx, found.id, 0)])
            }
            Got::Auto | Got::Stated(_) => SideLen::Unknown,
        });
        let mut out: Option<SideLen> = None;
        for one in founds.chain(fonts) {
            out = match out {
                None => Some(one),
                Some(held) if held.same(&one) => Some(held),
                Some(_) => Some(SideLen::Unknown),
            };
        }
        out.unwrap_or(SideLen::Unstated)
    }

    /// `expr`, stated at the step `id` in `cx`, as a [`Term`] under `world`:
    /// each reference followed to the one value it resolves to, a stated
    /// number or an expression read in its own context, or to a text
    /// style's font size, which ends it.
    fn term(
        &self,
        world: &World,
        expr: &Expr,
        cx: Cx<'_>,
        id: Option<(usize, usize)>,
        depth: usize,
    ) -> Term {
        match expr {
            Expr::Lit(literal) => Term::Lit(literal.clone()),
            Expr::Call(func, args) => Term::Call(
                *func,
                args.iter()
                    .map(|arg| self.term(world, arg, cx, id, depth))
                    .collect(),
            ),
            Expr::Ref(reference) => {
                let opaque = || Term::Opaque(reference.to_string(), id);
                if depth >= MAX_FOLLOW {
                    return opaque();
                }
                let places = self.places(world, reference, cx);
                let [Place::Field(target, path, bound)] = places.as_slice() else {
                    return opaque();
                };
                if font_place(path) {
                    return Term::Font(*target, path.clone());
                }
                let mut fonts = Vec::new();
                let founds: Vec<Found<'_>> = self
                    .value_at(world, *target, path, *bound, 0)
                    .into_iter()
                    .flat_map(|found| self.follow_to(world, found, Some(&mut fonts)).0)
                    .collect();
                match (founds.as_slice(), fonts.as_slice()) {
                    ([], [(target, path)]) => Term::Font(*target, path.clone()),
                    ([found], []) => match found.got {
                        Got::Stated(Fact::Number(n, unit)) => Term::Number(n, unit),
                        Got::Expr(inner) => {
                            self.term(world, inner, found.cx, found.id, depth.saturating_add(1))
                        }
                        _ => opaque(),
                    },
                    _ => opaque(),
                }
            }
        }
    }
}

impl Checks<'_, '_> {
    /// A problem at `at`, a place stated in the layer `layer`.
    fn problem(&mut self, layer: usize, at: &str, kind: ProblemKind) {
        self.found((layer, Problem::new(at, kind)));
    }

    /// A problem found, listed once.
    fn found(&mut self, problem: (usize, Problem)) {
        if let Some(raw) = self.raw.as_mut() {
            raw.push(problem.clone());
        }
        if !self.problems.contains(&problem) {
            self.problems.push(problem);
        }
    }

    /// Every expression stated, under each world it is in effect in: its
    /// operands (absent or `Auto` in a call, a plain reference bringing
    /// either where the field does not allow it, a block reference's fields
    /// likewise), `text_scale`, a `widget.<part>` not drawn, a corner the
    /// connector computes at layout, cycles, chains, and `element.width` or
    /// `element.height` in a field laying the element out (S2.2, S8, S12).
    /// The steps at one place sharing their worlds look the place up once
    /// per world.
    fn expressions(&mut self, reused: Option<&Reused>) {
        let stack = self.stack;
        // The layers the check adds: those above the layers its context
        // names below it, several at once on the synthetic route (S17), else
        // the last.
        let added = stack
            .cx
            .below
            .len()
            .min(stack.layers.len().saturating_sub(1));
        let below = stack.layers.get(..added).unwrap_or_default();
        let mut memory = reused.map(|reused| reused.memory.borrow_mut());
        // What an earlier layer's check kept holds only over the same layers
        // below the added ones, preset and declarations; the added layers
        // are kept for the check of the layer above them.
        if let Some(memory) = memory.as_mut() {
            let same =
                memory.as_ref().is_some_and(|held| {
                    held.preset == stack.preset()
                        && held.declarations == *stack.decl()
                        && held.layers.len() == below.len()
                        && held.layers.iter().zip(below).all(
                            |((layer, kind, left_out), stacked)| {
                                layer == stacked.layer
                                    && *kind == stacked.kind
                                    && left_out.as_ref() == stacked.left_out
                            },
                        )
                });
            if !same {
                **memory = Some(Memory {
                    layers: below
                        .iter()
                        .map(|stacked| {
                            (
                                stacked.layer.clone(),
                                stacked.kind,
                                stacked.left_out.cloned(),
                            )
                        })
                        .collect(),
                    preset: stack.preset(),
                    declarations: stack.decl().clone(),
                    steps: HashMap::new(),
                });
            }
            if let Some(held) = memory.as_mut() {
                let above = stack.layers.get(added..).unwrap_or_default();
                held.layers.extend(above.iter().map(|stacked| {
                    (
                        stacked.layer.clone(),
                        stacked.kind,
                        stacked.left_out.cloned(),
                    )
                }));
            }
        }
        let added_sources: Vec<&Source> = stack
            .sources
            .iter()
            .filter(|source| source.layer >= added)
            .collect();
        let mut steps: Vec<Step<'_>> = Vec::new();
        for (number, source) in stack.sources.iter().enumerate() {
            for (index, op) in source.ops.iter().enumerate() {
                let OpKind::Set(Val::Expr(expr)) = &op.kind else {
                    continue;
                };
                let key: StepKey = (source.layer, source.pos, source.target, index);
                let kept = memory
                    .as_ref()
                    .and_then(|memory| memory.as_ref())
                    .and_then(|memory| memory.steps.get(&key))
                    .filter(|memo| {
                        source.layer < added
                            && !memo.asked.iter().any(|query| {
                                added_sources.iter().any(|added| query.answered_by(added))
                            })
                    })
                    .map(|memo| memo.problems.clone());
                if let Some(problems) = kept {
                    steps.push(Step {
                        id: (number, index),
                        source,
                        op,
                        expr,
                        worlds: Rc::new(Vec::new()),
                        in_effect: Vec::new(),
                        asked: HashSet::new(),
                        reused: Some(problems),
                    });
                    continue;
                }
                let mut seeds = vec![Seed::Field(source.target, op.path.clone(), false)];
                if let Target::Part(part) = source.target {
                    seeds.push(Seed::Shown(part));
                }
                if let Some((Some(part), field)) =
                    stack.keyframe_at(source.target, source.pos, &op.path)
                {
                    seeds.push(Seed::Shown(part));
                    seeds.push(Seed::Field(Target::Part(part), field, false));
                }
                let outer = stack.ask_begin();
                let worlds = stack.worlds(seeds);
                let asked = stack.ask_end(outer);
                steps.push(Step {
                    id: (number, index),
                    source,
                    op,
                    expr,
                    worlds,
                    in_effect: Vec::new(),
                    asked,
                    reused: None,
                });
            }
        }
        // The steps of one target sharing their worlds, by place.
        let mut groups: HashMap<(Target, *const Vec<World>), Places<'_>> = HashMap::new();
        for (number, step) in steps.iter().enumerate() {
            if step.reused.is_some() {
                continue;
            }
            let places = groups
                .entry((step.source.target, Rc::as_ptr(&step.worlds)))
                .or_default();
            match places
                .iter_mut()
                .find(|(path, _)| *path == step.op.path.as_str())
            {
                Some((_, members)) => members.push(number),
                None => places.push((&step.op.path, vec![number])),
            }
        }
        for ((target, _), places) in groups {
            let Some(worlds) = places
                .first()
                .and_then(|(_, members)| members.first())
                .and_then(|first| steps.get(*first))
                .map(|step| Rc::clone(&step.worlds))
            else {
                continue;
            };
            // The steps found at each place, where the target is drawn.
            let mut decided: Decided<Option<Vec<FoundAt>>> = Decided::default();
            let outer = stack.ask_begin();
            for (index, world) in worlds.iter().enumerate() {
                let found = match decided.get(world) {
                    Some(found) => found.clone(),
                    None => stack.recorded(world, &mut decided, || {
                        let drawn = match target {
                            Target::Part(part) => {
                                matches!(stack.shown(world, part).0, Shown::Drawn)
                            }
                            _ => true,
                        };
                        let scheme = match target {
                            Target::Roles(scheme) => scheme == world.scheme,
                            _ => true,
                        };
                        (drawn && scheme).then(|| {
                            places
                                .iter()
                                .map(|(path, _)| {
                                    stack
                                        .value_at(world, target, path, Bound::ALL, 0)
                                        .iter()
                                        .map(|found| found.id)
                                        .collect()
                                })
                                .collect()
                        })
                    }),
                };
                let Some(found) = found else {
                    continue;
                };
                for ((_, members), found) in places.iter().zip(&found) {
                    for member in members {
                        if let Some(step) = steps.get_mut(*member)
                            && found.contains(&Some(step.id))
                        {
                            step.in_effect.push(index);
                        }
                    }
                }
            }
            let asked = stack.ask_end(outer);
            for member in places.iter().flat_map(|(_, members)| members) {
                if let Some(step) = steps.get_mut(*member) {
                    step.asked.extend(asked.iter().cloned());
                }
            }
        }
        let mut kept: Vec<(StepKey, Memo)> = Vec::new();
        for step in steps {
            if let Some(problems) = step.reused {
                for problem in problems {
                    self.found(problem);
                }
                continue;
            }
            self.raw = Some(Vec::new());
            let outer = stack.ask_begin();
            let mut done: Decided<()> = Decided::default();
            for index in &step.in_effect {
                let Some(world) = step.worlds.get(*index) else {
                    continue;
                };
                if done.get(world).is_none() {
                    stack.recorded(world, &mut done, || {
                        self.expression(world, step.id, step.source, step.op, step.expr);
                    });
                }
            }
            let mut asked = stack.ask_end(outer);
            asked.extend(step.asked);
            let key = (
                step.source.layer,
                step.source.pos,
                step.source.target,
                step.id.1,
            );
            kept.push((
                key,
                Memo {
                    asked: asked.into_iter().collect(),
                    problems: self.raw.take().unwrap_or_default(),
                },
            ));
        }
        if let Some(Some(memory)) = memory.as_deref_mut() {
            memory.steps.extend(kept);
        }
    }

    fn expression(
        &mut self,
        world: &World,
        id: (usize, usize),
        source: &Source,
        op: &Op,
        expr: &Expr,
    ) {
        let stack = self.stack;
        // A keyframe's field over a part that does not draw it is not
        // evaluated (S2.7, S12).
        if let Some((part, field)) = stack.keyframe_at(source.target, source.pos, &op.path)
            && !stack.keyframe_drawn(world, part, &field)
        {
            return;
        }
        let cx = cx_of(source, op);
        let plain = matches!(expr, Expr::Ref(_));
        let in_token_style = matches!(source.target, Target::Tokens) && op.text_style.is_some();
        for (reference, in_call) in operands(expr) {
            let operand = reference.to_string();
            match reference {
                Ref::TextScale => {
                    let dynamic = stack
                        .effective(world, Target::Tokens, "text.scaling")
                        .iter()
                        .any(|found| variant(found) == Some("dynamic_type"));
                    if dynamic {
                        self.problem(source.layer, &op.at, ProblemKind::TextScaleUnderDynamicType);
                    }
                    continue;
                }
                Ref::Widget { part, path } => {
                    let other = target_widget(source.target)
                        .and_then(|widget| TypedPart::from_names(widget, part.as_str()));
                    if let Some(other) = other
                        && !matches!(stack.shown(world, other).0, Shown::Drawn)
                    {
                        let part = part.to_string();
                        self.problem(source.layer, &op.at, ProblemKind::PartNotDrawn { part });
                    }
                    if let (Some(corner), Some(other)) = (corner_of(path), other) {
                        self.corner(world, source, other, corner, &operand, op);
                    }
                    if is_metric(path) {
                        continue;
                    }
                }
                Ref::Part(path) => {
                    if let (Some(corner), Target::Part(own)) = (corner_of(path), source.target) {
                        self.corner(world, source, own, corner, &operand, op);
                    }
                }
                _ => {}
            }
            let exempt_auto = in_call
                && !in_token_style
                && (*reference == Ref::LineHeight
                    || matches!(reference, Ref::Widget { path, .. }
                        if path.len() == 1 && path.first().is_some_and(|k| k.as_str() == "line_height")));
            for place in stack.places(world, reference, cx) {
                let founds = match place {
                    Place::Field(target, path, bound) => stack
                        .value_at(world, target, &path, bound, 0)
                        .into_iter()
                        .flat_map(|found| stack.follow(world, found).0)
                        .collect(),
                    Place::Binding(source, binding) => {
                        stack.follow(world, binding_found(source, binding)).0
                    }
                    Place::Stated => Vec::new(),
                };
                for found in founds {
                    match found.got {
                        Got::Absent if in_call => {
                            let operand = operand.clone();
                            self.problem(
                                source.layer,
                                &op.at,
                                ProblemKind::AbsentOperand { operand },
                            );
                        }
                        Got::Absent if !op.marks.may_be_absent => {
                            self.problem(source.layer, &op.at, ProblemKind::AbsentNotAllowed);
                        }
                        Got::Auto if in_token_style && *reference == Ref::LineHeight => {
                            let operand = operand.clone();
                            self.problem(
                                source.layer,
                                &op.at,
                                ProblemKind::AutoOperand { operand },
                            );
                        }
                        Got::Auto if in_call && !exempt_auto => {
                            let operand = operand.clone();
                            self.problem(
                                source.layer,
                                &op.at,
                                ProblemKind::AutoOperand { operand },
                            );
                        }
                        Got::Auto if !in_call && !op.marks.auto => {
                            self.problem(source.layer, &op.at, ProblemKind::AutoNotAllowed);
                        }
                        Got::Absent | Got::Auto | Got::Expr(_) | Got::Stated(_) => {}
                    }
                }
            }
        }
        let cycle = self.cycles(world, id, cx, expr);
        if cycle {
            self.problem(source.layer, &op.at, ProblemKind::ExpressionCycle);
        }
        if plain {
            self.block_reference(world, source, op);
            let found = Found {
                got: Got::Expr(expr),
                at: &op.at,
                cx,
                layer: source.layer,
                id: None,
                via: false,
                above: false,
            };
            if !cycle && stack.follow(world, found).1 > MAX_CHAIN {
                self.problem(source.layer, &op.at, ProblemKind::ChainTooLong);
            }
        }
        if layout_field(source.target, &op.path) {
            self.element_size(world, source, op, expr);
        }
    }

    /// A plain reference on a block field: each of the block's `Value`
    /// fields the reference brings, `Absent` or `Auto` where the field's
    /// place does not allow it (S2.2, S4.5, S12).
    fn block_reference(&mut self, world: &World, source: &Source, op: &Op) {
        let stack = self.stack;
        let Some(info) = registry::find(table(source.target), &op.path) else {
            return;
        };
        let FieldType::Block { ty, fields, .. } = info.ty else {
            return;
        };
        if ty.starts_with("Vec<") || ty.starts_with("SurfaceMap<") {
            return;
        }
        let mut inner = Vec::new();
        value_fields(fields, &op.path, &mut inner);
        // The steps stating a block field absent, by the field's path: a
        // field below it that step makes absent is no field of the block.
        // A step without an id (a binding's) is never the same step. The
        // block the reference brings absent whole is the first: the plain
        // reference's own check judges it.
        let mut absent_blocks: Vec<(String, Option<(usize, usize)>)> = stack
            .effective(world, source.target, &op.path)
            .into_iter()
            .filter(|found| matches!(found.got, Got::Absent))
            .map(|found| (op.path.clone(), found.id))
            .collect();
        for (path, field) in inner {
            for found in stack.effective(world, source.target, &path) {
                if matches!(found.got, Got::Absent) {
                    let below_absent = absent_blocks.iter().any(|(block, id)| {
                        id.is_some()
                            && *id == found.id
                            && path
                                .strip_prefix(block.as_str())
                                .is_some_and(|rest| rest.starts_with('.'))
                    });
                    if below_absent {
                        continue;
                    }
                    if matches!(field.ty, FieldType::Block { .. }) {
                        absent_blocks.push((path.clone(), found.id));
                    }
                }
                if !found.via && found.cx.target == source.target {
                    continue;
                }
                let at = format!("{}.{}", op.at, tail(&op.path, &path));
                match found.got {
                    Got::Absent if !field.may_be_absent => {
                        self.problem(source.layer, &at, ProblemKind::AbsentNotAllowed);
                    }
                    Got::Auto if !field.auto => {
                        self.problem(source.layer, &at, ProblemKind::AutoNotAllowed);
                    }
                    Got::Absent | Got::Auto | Got::Expr(_) | Got::Stated(_) => {}
                }
            }
        }
    }

    /// A corner operand of `named`, read where `source` states `op` (S2.4,
    /// S8, S12): one that can resolve `Concentric` or `FollowPart` is
    /// refused anywhere, one that can resolve `Full` unless it is the corner
    /// of the element's own part.
    fn corner(
        &mut self,
        world: &World,
        source: &Source,
        named: TypedPart,
        corner: &str,
        operand: &str,
        op: &Op,
    ) {
        let stack = self.stack;
        let path = if operand.contains(".focus.corners.") {
            format!("focus.corners.{corner}")
        } else {
            format!("corners.{corner}")
        };
        for found in stack.effective(world, Target::Part(named), &path) {
            let refused = match variant(&found) {
                Some("concentric" | "follow_part" | "ellipse") => true,
                Some("full") => {
                    if element_of(source.target) == Some(named) {
                        self.size_data(world, source, op);
                    }
                    element_of(source.target) != Some(named)
                }
                _ => false,
            };
            if refused {
                let operand = operand.to_owned();
                self.problem(
                    source.layer,
                    &op.at,
                    ProblemKind::CornerAtLayout { operand },
                );
            }
        }
    }

    /// A `Full` corner operand reads `element.width` and `element.height`
    /// (S8): the data the reading part (or the widget's values) declares
    /// under the flavour in force, under each where the world chooses none.
    fn size_data(&mut self, world: &World, source: &Source, op: &Op) {
        let decl = self.stack.decl();
        let (widget, owner) = match source.target {
            Target::Part(part) => (widget_of(part), DataOwner::Part(part)),
            Target::Values(widget) => (widget, DataOwner::Values),
            Target::Roles(_) | Target::Tokens => return,
        };
        let decls = match &world.flavor {
            Some(flavor) => vec![decl.in_flavor(widget, flavor)],
            None => decl.in_each_flavor(widget),
        };
        for datum in [ElementDatum::Width, ElementDatum::Height] {
            let name = DataName::Element(datum);
            let declared = decls.iter().all(|decl| {
                decl.data
                    .iter()
                    .flatten()
                    .any(|(held, names)| *held == owner && names.contains(&name))
            });
            if !declared {
                let datum = Ref::Element(datum).to_string();
                self.problem(source.layer, &op.at, ProblemKind::UndeclaredData { datum });
            }
        }
    }

    /// Whether the expression at `op`, the step `id`, reads itself under
    /// `world`: whether a lookup its references, calls' operands and
    /// bindings lead to finds that step again (a lookup bounded to the
    /// bases, `active(…)`'s, or to the rules before it, `base`'s, never
    /// does).
    fn cycles(&self, world: &World, id: (usize, usize), cx: Cx<'_>, expr: &Expr) -> bool {
        let stack = self.stack;
        // A step is one node; a part's bindings, sharing one place, are told
        // apart by their steps.
        let mut seen: Vec<Node> = Vec::new();
        let mut pending: Vec<(Cx<'_>, &Expr)> = vec![(cx, expr)];
        while let Some((cx, expr)) = pending.pop() {
            for reference in refs(expr) {
                for place in stack.places(world, reference, cx) {
                    let founds = match place {
                        Place::Field(target, path, bound) => {
                            stack.value_at(world, target, &path, bound, 0)
                        }
                        Place::Binding(source, binding) => vec![binding_found(source, binding)],
                        Place::Stated => Vec::new(),
                    };
                    for found in founds {
                        if found.id == Some(id) {
                            return true;
                        }
                        if let Got::Expr(next) = found.got {
                            let node = (found.cx.target, found.cx.path.to_owned(), found.id);
                            if !seen.contains(&node) {
                                seen.push(node);
                                pending.push((found.cx, next));
                            }
                        }
                    }
                }
            }
        }
        false
    }

    /// A layout field reading the element's laid-out width or height,
    /// directly or through `part.`, `widget.`, `values.` or a `Full` corner
    /// (S8, S12), where the element's own part is no fixed box on that axis
    /// under every combination (or, for a glyph or icon drawn to its box,
    /// does not state that axis's minimum as a length under every one).
    fn element_size(&mut self, world: &World, source: &Source, op: &Op, expr: &Expr) {
        let stack = self.stack;
        let mut reads: Vec<ElementDatum> = Vec::new();
        let mut seen: Vec<(Target, String)> = Vec::new();
        let mut pending: Vec<(Cx<'_>, &Expr)> = vec![(cx_of(source, op), expr)];
        while let Some((cx, expr)) = pending.pop() {
            for reference in refs(expr) {
                match reference {
                    Ref::Element(datum @ (ElementDatum::Width | ElementDatum::Height)) => {
                        if !reads.contains(datum) {
                            reads.push(*datum);
                        }
                        continue;
                    }
                    Ref::Part(path) | Ref::Widget { path, .. } if corner_of(path).is_some() => {
                        for place in stack.places(world, reference, cx) {
                            if let Place::Field(target, path, _) = place {
                                for found in stack.effective(world, target, &path) {
                                    if variant(&found) == Some("full") {
                                        for datum in [ElementDatum::Width, ElementDatum::Height] {
                                            if !reads.contains(&datum) {
                                                reads.push(datum);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Ref::Part(_) | Ref::Widget { .. } | Ref::Values(_) => {}
                    _ => continue,
                }
                for place in stack.places(world, reference, cx) {
                    let Place::Field(target, path, bound) = place else {
                        continue;
                    };
                    for found in stack.value_at(world, target, &path, bound, 0) {
                        if let Got::Expr(next) = found.got {
                            let node = (found.cx.target, found.cx.path.to_owned());
                            if !seen.contains(&node) {
                                seen.push(node);
                                pending.push((found.cx, next));
                            }
                        }
                    }
                }
            }
        }
        if reads.is_empty() {
            return;
        }
        let own = element_of(source.target);
        let drawn_to_box = matches!(op.path.as_str(), "glyph.size" | "icon.size");
        for datum in reads {
            let fixed = own.is_some_and(|own| self.fixed_box(own, datum, drawn_to_box));
            if !fixed {
                let datum = Ref::Element(datum).to_string();
                self.problem(
                    source.layer,
                    &op.at,
                    ProblemKind::ElementSizeInLayout { datum },
                );
            }
        }
    }

    /// Whether `part` states its `datum` axis's minimum and maximum equal
    /// under every combination it can resolve under, a fixed box (S8); for
    /// a glyph or icon drawn to its box, its minimum as a length, never
    /// `auto`, under every one.
    fn fixed_box(&self, part: TypedPart, datum: ElementDatum, drawn_to_box: bool) -> bool {
        let stack = self.stack;
        let (min, max) = match datum {
            ElementDatum::Height => ("min_height", "max_height"),
            _ => ("min_width", "max_width"),
        };
        let target = Target::Part(part);
        let seeds = vec![
            Seed::Field(target, min.to_owned(), false),
            Seed::Field(target, max.to_owned(), false),
        ];
        let worlds = stack.worlds(seeds);
        worlds.iter().all(|world| {
            let mins = stack.effective(world, target, min);
            if drawn_to_box {
                return !mins.is_empty()
                    && mins.iter().all(|found| {
                        matches!(found.got, Got::Stated(_) | Got::Expr(Expr::Call(..)))
                    });
            }
            let maxes = stack.effective(world, target, max);
            !mins.is_empty()
                && mins.iter().all(|a| {
                    stated_number(a).is_some()
                        && maxes.iter().all(|b| stated_number(a) == stated_number(b))
                })
                && !maxes.is_empty()
        })
    }

    /// The conditional `Absent` marks (S2.4, S2.9, S12): an edge's `dash`
    /// absent where its `width` and `paint` are drawn; an icon style's
    /// `fill` absent, outside an icon-size context, where its
    /// `rendering_mode` is not `Original`; a glyph's `fill` absent where its
    /// `stroke` is absent too and its `source` is not `SystemControl`, and
    /// its `size` absent where its `source` is not `SystemControl`; a
    /// ripple's `color`, `alpha` and `padding` absent where its `effect` is
    /// stated (S2.12); each under one world, references followed.
    fn marks(&mut self) {
        let stack = self.stack;
        let mut places: Vec<(Target, String, &'static str)> = Vec::new();
        let mut add = |target: Target, path: String, owner: &'static str| {
            if !places.iter().any(|(t, p, _)| *t == target && *p == path) {
                places.push((target, path, owner));
            }
        };
        for source in &stack.sources {
            for op in &source.ops {
                if let (
                    OpKind::Set(Val::Absent),
                    Some((owner @ ("Edge" | "IconStyle" | "Glyph" | "RippleStyle"), name)),
                ) = (&op.kind, op.field)
                    && matches!(
                        (owner, name),
                        ("Edge", "dash")
                            | ("IconStyle" | "Glyph", "fill")
                            | ("Glyph", "size")
                            | ("RippleStyle", "color" | "alpha" | "padding")
                    )
                {
                    let parent = op.path.rsplit_once('.').map_or("", |(parent, _)| parent);
                    add(source.target, parent.to_owned(), owner);
                }
            }
            if let Target::Part(part) = source.target
                && let Some(info) = part_info(part)
            {
                for field in info.fields {
                    match field.path {
                        "stroke" => {
                            for edge in ["top", "right", "bottom", "left"] {
                                add(source.target, format!("stroke.{edge}"), "Edge");
                            }
                        }
                        "icon" => add(source.target, "icon".to_owned(), "IconStyle"),
                        "glyph" => add(source.target, "glyph".to_owned(), "Glyph"),
                        "interaction" => {
                            add(
                                source.target,
                                "interaction.ripple".to_owned(),
                                "RippleStyle",
                            );
                        }
                        _ => {}
                    }
                }
            }
        }
        for (target, place, owner) in places {
            if target == Target::Tokens && place.starts_with("icons.sizes.") {
                continue;
            }
            let marks: &[(&str, &[&str])] = match owner {
                "Edge" => &[("dash", &["width", "paint"])],
                "IconStyle" => &[("fill", &["rendering_mode"])],
                "RippleStyle" => &[
                    ("color", &["effect"]),
                    ("alpha", &["effect"]),
                    ("padding", &["effect"]),
                ],
                _ => &[("fill", &["stroke", "source"]), ("size", &["source"])],
            };
            for (absent, others) in marks {
                self.mark(target, &place, owner, absent, others);
            }
        }
    }

    /// One conditional mark of [`Self::marks`]: the field `absent` of the
    /// block `owner` at `place` of `target`, under each world its `others`
    /// give.
    fn mark(&mut self, target: Target, place: &str, owner: &str, absent: &str, others: &[&str]) {
        let stack = self.stack;
        let mut seeds = vec![Seed::Field(target, below(place, absent), false)];
        seeds.extend(
            others
                .iter()
                .map(|other| Seed::Field(target, below(place, other), false)),
        );
        if let Target::Part(part) = target {
            seeds.push(Seed::Shown(part));
        }
        let worlds = stack.worlds(seeds);
        for world in worlds.iter() {
            if let Target::Part(part) = target
                && !matches!(stack.shown(world, part).0, Shown::Drawn)
            {
                continue;
            }
            // A step stating the block itself absent leaves no field of it
            // to check; a step without an id (a binding's) is never that
            // step.
            let whole: Vec<Option<(usize, usize)>> = stack
                .effective(world, target, place)
                .into_iter()
                .filter(|found| matches!(found.got, Got::Absent))
                .map(|found| found.id)
                .collect();
            let absents: Vec<Found<'_>> = stack
                .effective(world, target, &below(place, absent))
                .into_iter()
                .filter(|found| {
                    matches!(found.got, Got::Absent)
                        && !(found.id.is_some() && whole.contains(&found.id))
                })
                .collect();
            if absents.is_empty() {
                continue;
            }
            let other = |name: &str| stack.effective(world, target, &below(place, name));
            // A glyph no layer states a `source` for is S12's missing field,
            // judged there alone: neither conditional mark refuses beside it.
            let not_system_control = || {
                other("source")
                    .iter()
                    .any(|f| variant(f) != Some("system_control"))
            };
            let refused = match (owner, absent) {
                ("Edge", _) => ["width", "paint"]
                    .into_iter()
                    .all(|name| other(name).iter().any(|f| !matches!(f.got, Got::Absent))),
                ("IconStyle", _) => other("rendering_mode")
                    .iter()
                    .any(|f| variant(f) != Some("original") && !matches!(f.got, Got::Absent)),
                ("RippleStyle", _) => other("effect")
                    .iter()
                    .any(|f| !matches!(f.got, Got::Absent)),
                (_, "size") => not_system_control(),
                _ => {
                    other("stroke").iter().any(|f| matches!(f.got, Got::Absent))
                        && not_system_control()
                }
            };
            if refused {
                for found in absents {
                    self.problem(found.layer, found.at, ProblemKind::AbsentNotAllowed);
                }
            }
        }
    }

    /// An indication box naming a part that can be `Absent` where the
    /// ripple can be drawn (S2.12, S12): under a world in which the
    /// ripple's part is drawn and its `effect` stated, the box in effect,
    /// references followed, names a part of the widget that resolves
    /// `Absent` there.
    fn indication_boxes(&mut self) {
        let stack = self.stack;
        let mut places: Vec<(TypedPart, &str, Vec<TypedPart>)> = Vec::new();
        for source in &stack.sources {
            let Target::Part(part) = source.target else {
                continue;
            };
            for (place, named) in &source.indication_boxes {
                let index = match places
                    .iter()
                    .position(|(held, at, _)| *held == part && *at == place.as_str())
                {
                    Some(index) => index,
                    None => {
                        places.push((part, place, Vec::new()));
                        places.len().saturating_sub(1)
                    }
                };
                if let (PartRef::Typed(named), Some((.., parts))) = (named, places.get_mut(index))
                    && !parts.contains(named)
                {
                    parts.push(*named);
                }
            }
        }
        for (part, place, named) in places {
            let target = Target::Part(part);
            let ripple = place.strip_suffix(".indication_box").unwrap_or(place);
            let effect = below(ripple, "effect");
            let mut seeds = vec![
                Seed::Field(target, place.to_owned(), true),
                Seed::Field(target, effect.clone(), false),
                Seed::Shown(part),
            ];
            seeds.extend(named.iter().map(|named| Seed::Shown(*named)));
            let worlds = stack.worlds(seeds);
            for world in worlds.iter() {
                if !matches!(stack.shown(world, part).0, Shown::Drawn)
                    || !stack
                        .effective(world, target, &effect)
                        .iter()
                        .any(|found| !matches!(found.got, Got::Absent))
                {
                    continue;
                }
                for found in stack.effective(world, target, place) {
                    let named = found
                        .id
                        .and_then(|(number, index)| {
                            let source = stack.sources.get(number)?;
                            let op = source.ops.get(index)?;
                            source
                                .indication_boxes
                                .iter()
                                .find(|(held, _)| *held == op.path)
                        })
                        .and_then(|(_, named)| match named {
                            PartRef::Typed(named) if widget_of(*named) == widget_of(part) => {
                                Some(*named)
                            }
                            PartRef::Typed(_) | PartRef::Component { .. } => None,
                        });
                    if let Some(named) = named
                        && matches!(stack.shown(world, named).0, Shown::Absent)
                    {
                        let at = format!("{}.part", found.at);
                        let part = named.name().to_owned();
                        self.problem(
                            found.layer,
                            &at,
                            ProblemKind::IndicationBoxPartAbsent { part },
                        );
                    }
                }
            }
        }
    }

    /// Each field of a part drawn again over an `Absent` or `DrawnAs` that
    /// no layer states under a combination in which the part resolves
    /// drawn (S6.3, S12): a `Present` over a part with nothing beneath is
    /// missing every field no other patch states, a block's leaves judged
    /// where only `Fields` patches state it ([`unstated`]). Only a
    /// `Present` draws a part a patch made absent, so a part none redraws
    /// needs no check here: its bases state its fields (S2.2), or leave it
    /// absent.
    fn redrawn(&mut self) {
        let stack = self.stack;
        for info in registry::widgets() {
            for part_info in info.parts {
                let Some(part) = TypedPart::from_names(info.kind, part_info.name) else {
                    continue;
                };
                let redrawn = stack.of(Target::Part(part)).any(|(_, source)| {
                    source
                        .ops
                        .iter()
                        .any(|op| matches!(op.kind, OpKind::Shown(Shown::Drawn)))
                });
                if !redrawn {
                    continue;
                }
                let mut seeds = vec![Seed::Shown(part)];
                seeds.extend(
                    part_info
                        .fields
                        .iter()
                        .map(|field| Seed::Field(Target::Part(part), field.path.to_owned(), true)),
                );
                let worlds = stack.worlds(seeds);
                let mut missing: Vec<String> = Vec::new();
                for world in worlds.iter() {
                    let (Shown::Drawn, Some((layer, _))) = stack.shown(world, part) else {
                        continue;
                    };
                    let ops: Vec<&Op> = stack
                        .of(Target::Part(part))
                        .filter(|(_, source)| stack.applies(world, source))
                        .flat_map(|(_, source)| source.ops.iter())
                        .filter(|op| !matches!(op.kind, OpKind::Shown(_)))
                        .collect();
                    let mut unstated_here = Vec::new();
                    for field in part_info.fields {
                        unstated(&ops, field.path, field, &mut unstated_here);
                    }
                    for path in unstated_here {
                        if !missing.contains(&path) {
                            let at = format!("widgets.{}.{}.{path}", info.name, part_info.name);
                            self.problems
                                .push((layer, Problem::new(&at, ProblemKind::Missing)));
                            missing.push(path);
                        }
                    }
                }
            }
        }
    }

    /// `DrawnAs` patches forming a cycle under some combination (S6.1,
    /// S12): a part drawn as another widget, or a part of one, whose parts,
    /// followed through `DrawnAs`, draw the first again. The widget drawn
    /// is queried with the variants the `DrawnAs` names and the host's
    /// conditions and states, its other selector parts any.
    fn drawn_as_cycles(&mut self) {
        let stack = self.stack;
        let parts: Vec<TypedPart> = registry::widgets()
            .iter()
            .flat_map(|info| {
                info.parts
                    .iter()
                    .filter_map(|part| TypedPart::from_names(info.kind, part.name))
            })
            .collect();
        if !stack.sources.iter().any(|s| {
            s.ops
                .iter()
                .any(|op| matches!(op.kind, OpKind::Shown(Shown::DrawnAs { .. })))
        }) {
            return;
        }
        let seeds = parts.iter().map(|part| Seed::Shown(*part)).collect();
        let rules = stack.reach(seeds);
        let widgets: Vec<WidgetKind> = registry::widgets().iter().map(|info| info.kind).collect();
        // The conditions, states and variants are outcomes of their own:
        // a `DrawnAs` reads them (`reaches_again`). The states are the host
        // element's, which each widget's element reduces by its own groups
        // (S8): no group excludes them here.
        let (atoms, all) = stack.enumerate(&rules, &widgets, false, |atoms| {
            atoms
                .iter()
                .enumerate()
                .filter(|(_, atom)| {
                    matches!(atom, Atom::Cond(_) | Atom::State(_) | Atom::Variant(_))
                })
                .map(|(index, _)| Some(vec![(index, true)]))
                .collect()
        });
        let compiled = stack.compile(&atoms, &rules);
        let shown = |world: &World, part: TypedPart| {
            let reduced = stack.reduced(&atoms, world, widget_of(part), &compiled);
            let (shown, at) = stack.shown(&reduced, part);
            (shown.clone(), at.map(|(layer, at)| (layer, at.to_owned())))
        };
        for first in &parts {
            for world in &all {
                let (Shown::DrawnAs { .. }, Some((layer, at))) = shown(world, *first) else {
                    continue;
                };
                if Self::reaches_again(&atoms, &all, *first, world, &shown) {
                    self.problem(layer, &at, ProblemKind::DrawnAsCycle);
                }
            }
        }
    }

    /// Whether `first`, drawn as another widget under `world`, is drawn
    /// again through the `DrawnAs` patches it leads to.
    /// `shown` answers what a part is under a world, the world's states
    /// reduced by the part's own widget's groups.
    fn reaches_again(
        atoms: &[Atom],
        all: &[World],
        first: TypedPart,
        world: &World,
        shown: &dyn Fn(&World, TypedPart) -> ShownAt,
    ) -> bool {
        let mut seen: Vec<(TypedPart, usize)> = Vec::new();
        let mut pending: Vec<(TypedPart, &World)> = vec![(first, world)];
        while let Some((part, host)) = pending.pop() {
            let (Shown::DrawnAs { target, variants }, _) = shown(host, part) else {
                continue;
            };
            let drawn: Vec<TypedPart> = match &target {
                WidgetOrPart::Part(crate::selector::PartRef::Typed(part)) => vec![*part],
                WidgetOrPart::Widget(WidgetRef::Typed(widget)) => registry::widgets()
                    .iter()
                    .filter(|info| info.kind == *widget)
                    .flat_map(|info| {
                        info.parts
                            .iter()
                            .filter_map(|p| TypedPart::from_names(info.kind, p.name))
                    })
                    .collect(),
                WidgetOrPart::Widget(WidgetRef::Component(_))
                | WidgetOrPart::Part(crate::selector::PartRef::Component { .. }) => Vec::new(),
            };
            for (index, next) in all.iter().enumerate() {
                let consistent =
                    atoms
                        .iter()
                        .zip(host.values.iter().zip(&next.values))
                        .all(|(atom, (h, n))| match atom {
                            Atom::Cond(_) | Atom::State(_) => h == n,
                            Atom::Variant(variant) => *n == variants.contains(variant),
                            _ => true,
                        });
                if !consistent {
                    continue;
                }
                for part in &drawn {
                    if !matches!(shown(next, *part).0, Shown::DrawnAs { .. }) {
                        continue;
                    }
                    if *part == first {
                        return true;
                    }
                    if !seen.contains(&(*part, index)) {
                        seen.push((*part, index));
                        pending.push((*part, next));
                    }
                }
            }
        }
        false
    }

    /// An input whose value in force reads itself, through inputs and
    /// colour roles, under some scheme and contrast (S12), as evaluation
    /// refuses it ([`inputs::input_cycles`]).
    fn input_cycles(&mut self) {
        let stack = self.stack;
        let left_out = stack.cx.reader_accent_left_out;
        for (layer, name) in inputs::input_cycles(&stack.layers, stack.preset(), left_out) {
            let toml = stack.layers.get(layer).is_some_and(|stacked| {
                matches!(stacked.layer.spelled, crate::layer::Spelled::Toml)
            });
            let at = if toml {
                format!("inputs.{name}")
            } else {
                format!("inputs.entries.{name}")
            };
            self.problem(layer, &at, ProblemKind::ExpressionCycle);
        }
    }

    /// `TextRoles` (S4.4): two slots referencing one keyed text style
    /// directly, or a slot referencing a slot later in the owner order,
    /// under one world.
    fn text_roles(&mut self) {
        let stack = self.stack;
        let seeds = OWNER_ORDER
            .iter()
            .map(|slot| Seed::Field(Target::Tokens, format!("text.roles.{slot}"), false))
            .collect();
        let worlds = stack.worlds(seeds);
        for world in worlds.iter() {
            let mut direct: Vec<(String, &str)> = Vec::new();
            for (order, slot) in OWNER_ORDER.iter().enumerate() {
                let path = format!("text.roles.{slot}");
                for found in stack.value_at(world, Target::Tokens, &path, Bound::ALL, 0) {
                    let Got::Expr(Expr::Ref(Ref::Token(path))) = found.got else {
                        continue;
                    };
                    let names: Vec<&str> = path.iter().map(Key::as_str).collect();
                    match names.as_slice() {
                        ["text", "keyed", style] => {
                            if direct.iter().any(|(held, _)| held == style) {
                                let style = (*style).to_owned();
                                self.problem(
                                    found.layer,
                                    found.at,
                                    ProblemKind::TextRoleShared { style },
                                );
                            } else {
                                direct.push(((*style).to_owned(), found.at));
                            }
                        }
                        ["text", "roles", other] => {
                            let later = OWNER_ORDER
                                .iter()
                                .position(|held| held == other)
                                .is_some_and(|other| other >= order);
                            if later {
                                let slot = (*other).to_owned();
                                self.problem(
                                    found.layer,
                                    found.at,
                                    ProblemKind::TextRoleOrder { slot },
                                );
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// A part's `shape` stated while its corners' curve is `Continuous`,
    /// under one world (S2.4, S12).
    fn shapes(&mut self) {
        let stack = self.stack;
        for part in all_parts() {
            let target = Target::Part(part);
            // A shape is stated only by a step at `shape`.
            let states = stack
                .of(target)
                .any(|(_, source)| source.ops.iter().any(|op| op.path == "shape"));
            if !states {
                continue;
            }
            let seeds = vec![
                Seed::Field(target, "shape".to_owned(), false),
                Seed::Field(target, "corners".to_owned(), true),
            ];
            let worlds = stack.worlds(seeds);
            for world in worlds.iter() {
                let continuous = stack
                    .effective(world, target, "corners.curve")
                    .iter()
                    .any(|found| variant(found) == Some("continuous"));
                let ellipse = CORNERS.iter().any(|corner| {
                    stack
                        .effective(world, target, &format!("corners.{corner}"))
                        .iter()
                        .any(|found| variant(found) == Some("ellipse"))
                });
                if !continuous && !ellipse {
                    continue;
                }
                for found in stack.effective(world, target, "shape") {
                    if !matches!(found.got, Got::Absent) {
                        self.problem(found.layer, found.at, ProblemKind::ShapeOverCorners);
                    }
                }
            }
        }
    }

    /// A `Corners` value stating `Ellipse` on a corner, under one world,
    /// where another of its corners can be no `Ellipse` or its curve
    /// `Continuous` (S2.4, S12, ADD-A124): a part's corners, its focus
    /// indicator's, and the focus token's.
    fn ellipses(&mut self) {
        let stack = self.stack;
        let mut places: Vec<(Target, &str)> = vec![(Target::Tokens, "focus.corners")];
        for part in all_parts() {
            let Some(info) = part_info(part) else {
                continue;
            };
            for (field, place) in [("corners", "corners"), ("focus", "focus.corners")] {
                if info.fields.iter().any(|f| f.path == field) {
                    places.push((Target::Part(part), place));
                }
            }
        }
        // An `Ellipse` a place resolves to is stated somewhere: at the place,
        // or at one a reference above it leads to (S2.2), so the places are
        // checked wherever any source states one.
        let states = stack.sources.iter().any(|source| {
            source
                .ops
                .iter()
                .any(|op| matches!(op.kind, OpKind::Set(Val::Stated(Fact::Variant("ellipse")))))
        });
        if !states {
            return;
        }
        for (target, place) in places {
            let worlds = stack.worlds(vec![Seed::Field(target, place.to_owned(), true)]);
            for world in worlds.iter() {
                let corners: Vec<Vec<Found<'_>>> = CORNERS
                    .iter()
                    .map(|corner| stack.effective(world, target, &format!("{place}.{corner}")))
                    .collect();
                let ellipses: Vec<&Found<'_>> = corners
                    .iter()
                    .flatten()
                    .filter(|found| variant(found) == Some("ellipse"))
                    .collect();
                if ellipses.is_empty() {
                    continue;
                }
                let all = corners.iter().all(|founds| {
                    !founds.is_empty() && founds.iter().all(|f| variant(f) == Some("ellipse"))
                });
                let continuous = stack
                    .effective(world, target, &format!("{place}.curve"))
                    .iter()
                    .any(|found| variant(found) == Some("continuous"));
                if !all || continuous {
                    for found in ellipses {
                        self.problem(found.layer, found.at, ProblemKind::EllipseCorners);
                    }
                }
            }
        }
    }

    /// A focus indicator's `FollowPart` corner whose two sides' offsets can
    /// differ, in either direction, under one world (S2.4, S2.10, S12).
    fn focus_corners(&mut self) {
        let stack = self.stack;
        let mut places: Vec<(Target, &str)> = vec![(Target::Tokens, "focus")];
        for part in all_parts() {
            if part_info(part).is_some_and(|info| info.fields.iter().any(|f| f.path == "focus")) {
                places.push((Target::Part(part), "focus"));
            }
        }
        for (target, focus) in places {
            for corner in CORNERS {
                let corner_path = format!("{focus}.corners.{corner}");
                let mut seeds = vec![Seed::Field(target, corner_path.clone(), false)];
                for side in SIDES {
                    seeds.push(Seed::Field(target, format!("{focus}.offset.{side}"), false));
                }
                let worlds = stack.worlds(seeds);
                for world in worlds.iter() {
                    let follows: Vec<Found<'_>> = stack
                        .effective(world, target, &corner_path)
                        .into_iter()
                        .filter(|found| variant(found) == Some("follow_part"))
                        .collect();
                    if follows.is_empty() {
                        continue;
                    }
                    let offset = |side: &str| {
                        stack.side_len(world, target, &format!("{focus}.offset.{side}"))
                    };
                    let vertical = offset(corner_side(corner));
                    let differ = ["left", "right"]
                        .into_iter()
                        .any(|side| vertical.can_differ(&offset(side)));
                    if differ {
                        for found in follows {
                            let corner = corner.to_owned();
                            self.problem(
                                found.layer,
                                found.at,
                                ProblemKind::UnequalOffsets { corner },
                            );
                        }
                    }
                }
            }
        }
    }

    /// A part's `Concentric` corner whose two insets (the paddings and
    /// margins on its two sides, up to its container, S2.4) can differ, in
    /// either direction, under one world (S12, ADD-A122).
    fn concentric(&mut self) {
        let stack = self.stack;
        for part in all_parts() {
            let Some(info) = part_info(part) else {
                continue;
            };
            if !info.fields.iter().any(|f| f.path == "corners") {
                continue;
            }
            let chain = inset_chain(part);
            let container = container_of(part);
            for corner in CORNERS {
                let target = Target::Part(part);
                let corner_path = format!("corners.{corner}");
                let mut seeds = vec![Seed::Field(target, corner_path.clone(), false)];
                if let Some(container) = container {
                    seeds.push(Seed::Field(
                        Target::Part(container),
                        corner_path.clone(),
                        false,
                    ));
                }
                for (holder, field) in &chain {
                    for side in SIDES {
                        seeds.push(Seed::Field(
                            Target::Part(*holder),
                            format!("{field}.{side}"),
                            false,
                        ));
                    }
                }
                let worlds = stack.worlds(seeds);
                for world in worlds.iter() {
                    let concentric: Vec<Found<'_>> = stack
                        .effective(world, target, &corner_path)
                        .into_iter()
                        .filter(|found| variant(found) == Some("concentric"))
                        .collect();
                    if concentric.is_empty() {
                        continue;
                    }
                    // The container's matching corner can be `Ellipse`: no
                    // one radius to be concentric with (S12, ADD-A124).
                    let ellipse = container.is_some_and(|container| {
                        stack
                            .effective(world, Target::Part(container), &corner_path)
                            .iter()
                            .any(|found| variant(found) == Some("ellipse"))
                    });
                    if ellipse {
                        for found in &concentric {
                            self.problem(found.layer, found.at, ProblemKind::EllipseCorners);
                        }
                    }
                    let inset = |side: &str| {
                        chain.iter().fold(
                            SideLen::Known(0.0, Vec::new()),
                            |sum, (holder, field)| {
                                sum.plus(stack.side_len(
                                    world,
                                    Target::Part(*holder),
                                    &format!("{field}.{side}"),
                                ))
                            },
                        )
                    };
                    let vertical = inset(corner_side(corner));
                    let differ = ["left", "right"]
                        .into_iter()
                        .any(|side| vertical.can_differ(&inset(side)));
                    if differ {
                        for found in concentric {
                            let corner = corner.to_owned();
                            self.problem(
                                found.layer,
                                found.at,
                                ProblemKind::UnequalInsets { corner },
                            );
                        }
                    }
                }
            }
        }
    }

    /// A map entry's patch that can create its entry with a field left
    /// unset that may not be `Absent` (S6.1, S12, ADD-A122): a base's where
    /// no base before it states the entry's field, a rule's where some
    /// world of the bases and rules before it leaves the field unstated. A
    /// paint map's entry an `Append` creates has its paint so unset, as it
    /// has none to append to (ADD-A131).
    fn entries(&mut self) {
        let stack = self.stack;
        for source in &stack.sources {
            // A preset's own bases state every field (S2.2): what they
            // leave out is missing, the structural check's.
            let preset = source.rule.is_none()
                && stack
                    .layers
                    .get(source.layer)
                    .is_some_and(|stacked| stacked.kind == LayerKind::Preset);
            if preset {
                continue;
            }
            for op in &source.ops {
                let required: Vec<&str> = match &op.kind {
                    OpKind::Fields { unset, entry: true } => unset
                        .iter()
                        .copied()
                        .filter(|field| {
                            let path = below(&op.path, field);
                            let marked = matches!(source.target, Target::Roles(_))
                                && registry::role_entry_may_be_absent(&path);
                            !marked
                                && registry::find(table(source.target), &path)
                                    .is_none_or(|info| !info.may_be_absent)
                        })
                        .collect(),
                    OpKind::Append { entry: true } => vec!["paint"],
                    _ => continue,
                };
                if required.is_empty() {
                    continue;
                }
                let seeds = vec![Seed::Field(source.target, op.path.clone(), true)];
                let worlds = stack.worlds(seeds);
                for world in worlds.iter() {
                    if !stack.applies(world, source) {
                        continue;
                    }
                    let bound = Bound {
                        before: Some(source.pos),
                        bases_only: false,
                    };
                    let held = stack.of(source.target).any(|(_, earlier)| {
                        stack.applies(world, earlier)
                            && bound.admits(earlier.pos)
                            && earlier
                                .ops
                                .iter()
                                .any(|e| e.path == op.path || is_above(&op.path, &e.path))
                    });
                    if !held {
                        for field in &required {
                            let field = (*field).to_owned();
                            self.problem(
                                source.layer,
                                &op.at,
                                ProblemKind::EntryCreatedUnset { field },
                            );
                        }
                    }
                }
            }
        }
    }
}

/// S4.4's owner order of the text slots at depth (the other nine are
/// staged, S6.1).
const OWNER_ORDER: [&str; 10] = [
    "body",
    "caption",
    "title1",
    "title2",
    "title3",
    "headline",
    "large_title",
    "display_large",
    "display_small",
    "monospace",
];

const CORNERS: [&str; 4] = ["top_left", "top_right", "bottom_right", "bottom_left"];

const SIDES: [&str; 4] = ["top", "right", "bottom", "left"];

/// A corner's vertical side.
fn corner_side(corner: &str) -> &'static str {
    if corner.starts_with("top") {
        "top"
    } else {
        "bottom"
    }
}

fn all_parts() -> Vec<TypedPart> {
    registry::widgets()
        .iter()
        .flat_map(|info| {
            info.parts
                .iter()
                .filter_map(|part| TypedPart::from_names(info.kind, part.name))
        })
        .collect()
}

/// A `Concentric` corner's container (S2.4): the nearest enclosing part whose
/// catalogue entry lists `corners`; `None` for a root, whose container lies
/// outside the widget.
fn container_of(part: TypedPart) -> Option<TypedPart> {
    let widget = widget_of(part);
    let info = registry::widgets()
        .iter()
        .find(|info| info.kind == widget)?;
    let mut current = part_info(part)?;
    for _ in info.parts {
        if current.root {
            return None;
        }
        let parent = current
            .parent
            .and_then(|name| info.part(name))
            .or_else(|| info.parts.iter().find(|p| p.root))?;
        if parent.fields.iter().any(|f| f.path == "corners") {
            return TypedPart::from_names(widget, parent.name);
        }
        current = parent;
    }
    None
}

/// The fields whose sides make a part's insets from its container (S2.4):
/// its own margin, the margin and padding of every part between it and the
/// nearest part enclosing it whose catalogue entry lists `corners`, and
/// that container's padding; within the widget, a root's container lying
/// outside it.
fn inset_chain(part: TypedPart) -> Vec<(TypedPart, &'static str)> {
    let widget = widget_of(part);
    let Some(info) = registry::widgets().iter().find(|info| info.kind == widget) else {
        return Vec::new();
    };
    let lists = |p: &PartInfo, name: &str| p.fields.iter().any(|f| f.path == name);
    let mut chain = Vec::new();
    let Some(mut current) = part_info(part) else {
        return chain;
    };
    if lists(current, "margin") {
        chain.push((part, "margin"));
    }
    for _ in info.parts {
        if current.root {
            break;
        }
        let parent = current
            .parent
            .and_then(|name| info.part(name))
            .or_else(|| info.parts.iter().find(|p| p.root));
        let Some(parent) = parent else { break };
        let Some(typed) = TypedPart::from_names(widget, parent.name) else {
            break;
        };
        if lists(parent, "padding") {
            chain.push((typed, "padding"));
        }
        if lists(parent, "corners") {
            break;
        }
        if lists(parent, "margin") {
            chain.push((typed, "margin"));
        }
        current = parent;
    }
    chain
}

/// Whether the current preset is `kde-breeze`, whose mapping palettes S12
/// checks (S9.1, S14).
pub(super) fn on_kde_breeze(preset: Option<PresetId>) -> bool {
    preset.is_some_and(|preset| match preset {
        PresetId::KdeBreeze => true,
        PresetId::Adwaita
        | PresetId::MacosTahoe
        | PresetId::MacosMonterey
        | PresetId::Material3
        | PresetId::Ios26
        | PresetId::Ios13
        | PresetId::Windows11 => false,
    })
}

/// One of `kde-breeze`'s accent-dependent roles (S9.1): a scheme key of a
/// declared set, or a `[WM]` keyed role.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum AccentRole {
    /// The role of `key` in the set.
    Surface(SurfaceSet, crate::inputs::KdeSchemeKey),
    /// The keyed role `wm_<key>`.
    Wm(crate::inputs::KdeWmKey),
}

impl AccentRole {
    /// The role's path in a scheme's colour roles.
    pub(crate) fn path(self) -> String {
        match self {
            AccentRole::Surface(set, key) => {
                format!("surfaces.{}.{}", set.name(), key.role().path())
            }
            AccentRole::Wm(key) => format!("keyed.wm_{}", key.name()),
        }
    }
}

/// S9.1's accent-dependent roles over `decl`, each once, in order: in every
/// declared set `foreground_active`, `foreground_link`, `decoration_focus`
/// and `decoration_hover`, `Selection`'s `background_normal`,
/// `background_alternate`, `foreground_normal` and `foreground_inactive`,
/// `Button`'s `background_alternate`, `Header`'s `background_normal` and
/// `foreground_normal`, and every key of every declared set where `tints`
/// (`token.kde.tint_factor` can be `Some`); then each `[WM]` keyed role
/// `decl` declares. The one list the accent-derivation layer states and
/// S12 checks a mapping palette against (S9.2, S14).
pub(crate) fn accent_dependent_roles(decl: &Declarations, tints: bool) -> Vec<AccentRole> {
    use crate::inputs::{KdeSchemeKey as K, KdeWmKey};
    use crate::roles::SurfaceSet as S;
    let mut roles: Vec<AccentRole> = Vec::new();
    for set in &decl.surface_sets {
        let mut keys: Vec<K> = vec![
            K::ForegroundActive,
            K::ForegroundLink,
            K::DecorationFocus,
            K::DecorationHover,
        ];
        match set {
            _ if tints => keys = K::ALL.to_vec(),
            S::Selection => keys.extend([
                K::BackgroundNormal,
                K::BackgroundAlternate,
                K::ForegroundNormal,
                K::ForegroundInactive,
            ]),
            S::Button => keys.push(K::BackgroundAlternate),
            S::Header => keys.extend([K::BackgroundNormal, K::ForegroundNormal]),
            _ => {}
        }
        for key in keys {
            let role = AccentRole::Surface(*set, key);
            if !roles.contains(&role) {
                roles.push(role);
            }
        }
    }
    for key in KdeWmKey::ALL {
        let name = format!("wm_{}", key.name());
        if decl.color_roles.iter().any(|held| held.as_str() == name) {
            roles.push(AccentRole::Wm(key));
        }
    }
    roles
}

/// On `kde-breeze`, a mapping palette (one stating no `kde_scheme` input:
/// a port converts to them, S14) at `palette` in the stack leaving an
/// accent-dependent role (S9.1's list) or an inactive input role to a layer
/// below it, under some combination, for a scheme it states (S9.2, S12,
/// S14): wherever the preset's `token.kde.tint_factor` can be `Some`
/// (conservatively, a value other than `none` under some combination of
/// the preset's own sources, the reader's or stand-in's `token.kde` kept
/// beneath the palette read by no check, S8), every key of every declared
/// set (S9.1's tinting entries).
pub(super) fn mapping_palette(stack: &Stack<'_>, palette: usize) -> Vec<Problem> {
    let mut problems = Vec::new();
    let Some(layer) = stack.layers.get(palette).map(|stacked| stacked.layer) else {
        return problems;
    };
    let decl = stack.decl();
    let current: Vec<&crate::layer::Bases> = std::iter::once(&layer.bases)
        .chain(
            layer
                .for_preset
                .iter()
                .filter(|(preset, _)| stack.preset() == Some(*preset))
                .map(|(_, bases)| bases),
        )
        .collect();
    let port = current.iter().any(|bases| {
        bases.inputs.entries.iter().any(|(name, _)| {
            name.segments()
                .first()
                .is_some_and(|s| s.as_str() == "kde_scheme")
        })
    });
    if port {
        return problems;
    }
    let tint = "kde.tint_factor";
    let presets: Vec<Stacked<'_>> = stack
        .layers
        .iter()
        .filter(|stacked| stacked.kind == LayerKind::Preset)
        .copied()
        .collect();
    let preset = Stack::new(&presets, stack.cx);
    let tints = preset
        .worlds(vec![Seed::Field(Target::Tokens, tint.to_owned(), false)])
        .iter()
        .any(|world| {
            preset
                .value_at(world, Target::Tokens, tint, Bound::ALL, 0)
                .iter()
                .any(|found| !matches!(found.got, Got::Absent | Got::Stated(Fact::Nothing)))
        });
    let roles: Vec<String> = accent_dependent_roles(decl, tints)
        .into_iter()
        .map(AccentRole::path)
        .collect();
    for scheme in Scheme::ALL {
        let states = current
            .iter()
            .any(|bases| bases.colors.get(scheme).is_some());
        if !states {
            continue;
        }
        let target = Target::Roles(scheme);
        for role in &roles {
            let worlds = stack.worlds(vec![Seed::Field(target, role.clone(), false)]);
            let left = worlds.iter().any(|world| {
                stack
                    .value_at(world, target, role, Bound::ALL, 0)
                    .iter()
                    .any(|found| found.layer < palette)
            });
            if left {
                let role = format!("colors.{}.{role}", scheme.name());
                problems.push(Problem::new(
                    &role,
                    ProblemKind::LeftToPreset { role: role.clone() },
                ));
            }
        }
        for input in &decl.inputs {
            let inactive = input
                .segments()
                .first()
                .is_some_and(|s| s.as_str() == "inactive");
            let stated =
                current.iter().any(|bases| {
                    bases.inputs.entries.iter().any(|(name, value)| {
                        name == input && value.per_scheme.get(scheme).is_some()
                    })
                });
            if inactive && !stated {
                let role = format!("inputs.{input}");
                if !problems.iter().any(|p| p.path == role) {
                    problems.push(Problem::new(
                        &role,
                        ProblemKind::LeftToPreset { role: role.clone() },
                    ));
                }
            }
        }
    }
    problems
}
