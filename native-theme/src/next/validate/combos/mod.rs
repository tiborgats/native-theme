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

use std::rc::Rc;

use super::LayerKind;
use super::outcomes::{Item, Space, scheme_and_flavors};
use super::stack::{
    BindingOp, Fact, ItemKey, Op, OpKind, Pos, Source, Stack, Target, Val, below, is_above,
    part_info, tail, widget_of,
};
use crate::next::blocks::part_style::RoleRef;
use crate::next::declare::{DataName, DataOwner, Declarations};
use crate::next::error::{Problem, ProblemKind};
use crate::next::expr::{ElementDatum, Expr, Ref};
use crate::next::key::Key;
use crate::next::presets::PresetId;
use crate::next::registry::{self, FieldInfo, FieldType, PartInfo};
use crate::next::roles::SurfaceSet;
use crate::next::selector::{
    Condition, ContextMatch, ControlSize, Density, Direction, Parity, Scheme, Selector, SetMatch,
    SiblingMatch, SiblingScope, SizeClass, StateFlag, VariantRef, WidgetOrPart, WidgetRef,
};
use crate::next::widgets::{Shown, TypedPart, WidgetKind};

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

/// Which rules a combination's evaluation reads.
#[derive(Clone, Debug, PartialEq)]
enum Seed {
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

/// What S3.3's step 1 finds for `surface.x`.
enum Answer<'s> {
    /// A `Role { Surface(x) }` binding.
    Role(Place<'s>),
    /// A `Surface(S)` binding: the lookup becomes `role.S.x`.
    Set(SurfaceSet),
}

/// How many references a chain may follow (S2.2, S12).
const MAX_CHAIN: usize = 32;

/// How deep a lookup follows references before it stops: past any chain
/// S12 allows, so a cycle ends a lookup instead of the stack.
const MAX_FOLLOW: usize = MAX_CHAIN.saturating_mul(2).saturating_add(4);

/// Every problem the stack's combinations show (S12), each with the layer
/// stating the place it names.
pub(super) fn check(stack: &Stack<'_>) -> Vec<(usize, Problem)> {
    let mut out = Checks {
        stack,
        problems: Vec::new(),
    };
    out.expressions();
    out.fields_over();
    out.marks();
    out.drawn_as_cycles();
    out.text_roles();
    out.shapes();
    out.ellipses();
    out.focus_corners();
    out.concentric();
    out.entries();
    out.problems
}

struct Checks<'s, 'a> {
    stack: &'s Stack<'a>,
    /// Each problem, with the layer stating the place it names.
    problems: Vec<(usize, Problem)>,
}

impl<'a> Stack<'a> {
    fn decl(&self) -> &'a Declarations {
        self.cx.declarations
    }

    fn preset(&self) -> Option<PresetId> {
        self.cx.preset
    }

    fn applies(&self, world: &World, source: &Source) -> bool {
        let rule = source
            .rule
            .is_none_or(|rule| world.applies.get(rule).copied().unwrap_or(false));
        rule && source.scheme.is_none_or(|scheme| scheme == world.scheme)
    }

    /// The places of the keyed items the source `number` states that later
    /// sources applying under `world` replace at every key (S6.1).
    fn replaced<'s>(&self, world: &World, number: usize, source: &'s Source) -> Vec<&'s str> {
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
    /// interleave with the patches kept on the reference by position.
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
        for (number, source) in self.of(target) {
            if !self.applies(world, source) || !bound.admits(source.pos) {
                continue;
            }
            let replaced = self.replaced(world, number, source);
            for (index, op) in source.ops.iter().enumerate() {
                if replaced
                    .iter()
                    .any(|item| op.path == *item || is_above(item, &op.path))
                {
                    continue;
                }
                let found = |got| Found {
                    got,
                    at: &op.at,
                    cx: cx_of(source, op),
                    layer: source.layer,
                    id: Some((number, index)),
                    via: false,
                };
                if op.path == q {
                    let got = match &op.kind {
                        OpKind::Set(val) => Some(got_of(val)),
                        OpKind::Append => Some(Got::Stated(Fact::Unknown)),
                        OpKind::Touch => Some(Got::Stated(Fact::Nothing)),
                        OpKind::Fields { .. } | OpKind::Clear | OpKind::Shown(_) => None,
                    };
                    // Below a reference, a patch kept on it (S2.2).
                    if let Some(got) = got {
                        local = Some(found(got));
                    }
                } else if is_above(&op.path, q) {
                    match &op.kind {
                        OpKind::Set(Val::Absent) => {
                            local = Some(found(Got::Absent));
                            via = None;
                        }
                        OpKind::Set(Val::Auto) => {
                            local = Some(found(Got::Auto));
                            via = None;
                        }
                        OpKind::Set(Val::Expr(_)) => {
                            local = None;
                            via = Some((number, source, op));
                        }
                        OpKind::Set(Val::Stated(_)) | OpKind::Clear => {
                            local = None;
                            via = None;
                        }
                        OpKind::Fields { .. }
                        | OpKind::Append
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
                    && local
                        .is_none_or(|local| !local.cx.pos.rule || referent.cx.pos > local.cx.pos);
                out.push(match local {
                    Some(local) if !later => local,
                    _ => Found {
                        via: true,
                        ..referent
                    },
                });
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
            Ref::Base => vec![Place::Field(
                cx.target,
                cx.path.to_owned(),
                Bound {
                    before: Some(cx.pos),
                    bases_only: false,
                },
            )],
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
            let mut last = None;
            for (_, source) in self.of(Target::Part(*part)) {
                if !self.applies(world, source) {
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
            let mut role = None;
            let mut set = None;
            for (_, source) in self.of(Target::Part(*part)) {
                if !self.applies(world, source) {
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

    /// What a part is drawn as under `world`: its last `Absent`, `DrawnAs` or
    /// style patch's say, and where that is stated.
    fn shown(&self, world: &World, part: TypedPart) -> (&Shown, Option<(usize, &str)>) {
        let mut out: (&Shown, Option<(usize, &str)>) = (&Shown::Drawn, None);
        for (_, source) in self.of(Target::Part(part)) {
            if !self.applies(world, source) {
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
            for place in self.places(world, reference, found.cx) {
                match place {
                    Place::Field(target, path, bound) => {
                        next.extend(self.value_at(world, target, &path, bound, 0));
                    }
                    Place::Binding(source, binding) => next.push(binding_found(source, binding)),
                    Place::Stated => {}
                }
            }
            if next.is_empty() {
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
        let mut seen: Vec<Seed> = Vec::new();
        let mut pending = seeds;
        let any = self.any_world();
        while let Some(seed) = pending.pop() {
            if seen.contains(&seed) {
                continue;
            }
            seen.push(seed.clone());
            match &seed {
                Seed::Shown(part) => {
                    // Where every step states the part drawn, it is drawn
                    // under every world: no rule decides it.
                    let varies = self.of(Target::Part(*part)).any(|(_, source)| {
                        source.ops.iter().any(
                            |op| matches!(&op.kind, OpKind::Shown(shown) if *shown != Shown::Drawn),
                        )
                    });
                    if !varies {
                        continue;
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
                                    pending.extend(self.seeds_of(
                                        &any,
                                        expr,
                                        cx_of(source, op),
                                        &rest,
                                    ));
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
        }
        rules
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
        let (_, worlds) = self.enumerate(&rules, &widgets, scheme_and_flavors);
        let worlds = Rc::new(worlds);
        self.cache.borrow_mut().insert(rules, Rc::clone(&worlds));
        worlds
    }

    /// The atoms `rules` name, completed, and one world per distinct
    /// outcome: a vector of the rules' applicability and of the decisions
    /// `outcomes` adds over the atoms, under S12's exclusions with the
    /// groups `widgets` declare.
    fn enumerate(
        &self,
        rules: &[usize],
        widgets: &[WidgetKind],
        outcomes: impl Fn(&[Atom]) -> Vec<Item>,
    ) -> (Vec<Atom>, Vec<World>) {
        let mut atoms: Vec<Atom> = Vec::new();
        for rule in rules {
            if let Some(info) = self.rules.get(*rule) {
                atoms_of(info.selector, self.preset(), &mut atoms);
            }
        }
        let decl = self.decl();
        complete(&mut atoms, decl);
        let compiled = self.compile(&atoms, rules);
        let mut items: Vec<Item> = compiled.iter().map(|(_, needs)| needs.clone()).collect();
        items.extend(outcomes(&atoms));
        let space = Space::new(&atoms, decl, widgets);
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
        id: None,
        via: false,
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

/// Every reference in `expr`, and whether it is a call's operand.
fn operands(expr: &Expr) -> Vec<(&Ref, bool)> {
    let mut out = Vec::new();
    let mut pending = vec![(expr, false)];
    while let Some((expr, in_call)) = pending.pop() {
        match expr {
            Expr::Lit(_) => {}
            Expr::Ref(reference) => out.push((reference, in_call)),
            Expr::Call(_, args) => pending.extend(args.iter().map(|arg| (arg, true))),
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
            .context
            .iter()
            .map(|context| (Atom::Context(context.clone()), Some(true))),
    );
    out.extend(
        selector
            .not_context
            .iter()
            .map(|context| (Atom::Context(context.clone()), Some(false))),
    );
    if let Some(p) = &selector.position {
        let among = &p.among;
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
fn complete(atoms: &mut Vec<Atom>, decl: &Declarations) {
    for scheme in Scheme::ALL {
        push_atom(atoms, Atom::Cond(Condition::Scheme(scheme)));
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
            Atom::Cond(Condition::Flavor(_)) | Atom::State(_) | Atom::Variant(_)
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
        Got::Stated(Fact::Number(n)) => Some(n),
        _ => None,
    }
}

fn variant(found: &Found<'_>) -> Option<&'static str> {
    match found.got {
        Got::Stated(Fact::Variant(name)) => Some(name),
        _ => None,
    }
}

/// One side's length as the inset and offset checks compare it (S2.4,
/// S2.10).
#[derive(Debug)]
enum SideLen<'s> {
    /// A sum: of stated numbers (an absent side 0), and of expressions,
    /// each by its step, the same only where stated at one step.
    Known(f32, Vec<(Option<(usize, usize)>, &'s Expr)>),
    /// No layer states it: missing, the preset's check (S2.2), so it is no
    /// difference.
    Unstated,
    /// Anything else (`auto`, or values that differ by combination): it can
    /// differ from any side.
    Unknown,
}

impl<'s> SideLen<'s> {
    /// The length the values found at one place give.
    fn of(founds: &[Found<'s>]) -> SideLen<'s> {
        let mut out: Option<SideLen<'s>> = None;
        for found in founds {
            let one = match found.got {
                Got::Stated(Fact::Number(n)) => SideLen::Known(n, Vec::new()),
                Got::Absent => SideLen::Known(0.0, Vec::new()),
                Got::Expr(expr) => SideLen::Known(0.0, vec![(found.id, expr)]),
                Got::Auto | Got::Stated(_) => SideLen::Unknown,
            };
            out = match out {
                None => Some(one),
                Some(held) if held.same(&one) => Some(held),
                Some(_) => Some(SideLen::Unknown),
            };
        }
        out.unwrap_or(SideLen::Unstated)
    }

    /// Two lengths added.
    fn plus(self, other: SideLen<'s>) -> SideLen<'s> {
        match (self, other) {
            (SideLen::Unknown, _) | (_, SideLen::Unknown) => SideLen::Unknown,
            (SideLen::Unstated, _) | (_, SideLen::Unstated) => SideLen::Unstated,
            (SideLen::Known(a, mut terms), SideLen::Known(b, more)) => {
                terms.extend(more);
                SideLen::Known(a + b, terms)
            }
        }
    }

    /// Whether two known lengths are the same under every input.
    fn same(&self, other: &SideLen<'_>) -> bool {
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
    fn can_differ(&self, other: &SideLen<'_>) -> bool {
        match (self, other) {
            (SideLen::Unstated, _) | (_, SideLen::Unstated) => false,
            _ => !self.same(other),
        }
    }
}

impl Checks<'_, '_> {
    /// A problem at `at`, a place stated in the layer `layer`.
    fn problem(&mut self, layer: usize, at: &str, kind: ProblemKind) {
        let problem = (layer, Problem::new(at, kind));
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
    fn expressions(&mut self) {
        let stack = self.stack;
        for (number, source) in stack.sources.iter().enumerate() {
            for (index, op) in source.ops.iter().enumerate() {
                let OpKind::Set(Val::Expr(expr)) = &op.kind else {
                    continue;
                };
                let mut seeds = vec![Seed::Field(source.target, op.path.clone(), false)];
                if let Target::Part(part) = source.target {
                    seeds.push(Seed::Shown(part));
                }
                let worlds = stack.worlds(seeds);
                for world in worlds.iter() {
                    let in_effect = stack
                        .value_at(world, source.target, &op.path, Bound::ALL, 0)
                        .iter()
                        .any(|found| found.id == Some((number, index)));
                    let drawn = match source.target {
                        Target::Part(part) => matches!(stack.shown(world, part).0, Shown::Drawn),
                        _ => true,
                    };
                    let scheme = match source.target {
                        Target::Roles(scheme) => scheme == world.scheme,
                        _ => true,
                    };
                    if in_effect && drawn && scheme {
                        self.expression(world, (number, index), source, op, expr);
                    }
                }
            }
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
        for (path, field) in inner {
            for found in stack.effective(world, source.target, &path) {
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
        let mut seen: Vec<(Target, String)> = Vec::new();
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

    /// Every `Fields` patch, under each world its rule applies in: refused
    /// where the value below it, references followed, can be `Absent` or
    /// `Auto` (S2.2, S12).
    fn fields_over(&mut self) {
        let stack = self.stack;
        for source in &stack.sources {
            for op in &source.ops {
                let OpKind::Fields { .. } = op.kind else {
                    continue;
                };
                let seeds = vec![Seed::Field(source.target, op.path.clone(), false)];
                let rule_seed = source.rule;
                let worlds = stack.worlds(seeds);
                for world in worlds.iter() {
                    if rule_seed.is_some() && !stack.applies(world, source) {
                        continue;
                    }
                    let bound = Bound {
                        before: Some(source.pos),
                        bases_only: false,
                    };
                    let founds = stack
                        .value_at(world, source.target, &op.path, bound, 0)
                        .into_iter()
                        .flat_map(|found| stack.follow(world, found).0);
                    for found in founds {
                        let below = match found.got {
                            Got::Absent => "absent",
                            Got::Auto => "auto",
                            Got::Expr(_) | Got::Stated(_) => continue,
                        };
                        // One problem per fault: the outermost of the
                        // source's patches over that value.
                        let outer = source.ops.iter().any(|other| {
                            matches!(other.kind, OpKind::Fields { .. })
                                && is_above(&other.path, &op.path)
                                && (other.path == found.cx.path
                                    || is_above(found.cx.path, &other.path))
                        });
                        if outer {
                            continue;
                        }
                        self.problem(source.layer, &op.at, ProblemKind::FieldsOver { below });
                    }
                }
            }
        }
    }

    /// The conditional `Absent` marks (S2.4, S2.9, S12): an edge's `dash`
    /// absent where its `width` and `paint` are drawn; an icon style's
    /// `fill` absent, outside an icon-size context, where its
    /// `rendering_mode` is not `Original`; a glyph's `fill` absent where its
    /// `stroke` is absent too; each under one world, references followed.
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
                    Some((owner @ ("Edge" | "IconStyle" | "Glyph"), name)),
                ) = (&op.kind, op.field)
                    && matches!(
                        (owner, name),
                        ("Edge", "dash") | ("IconStyle" | "Glyph", "fill")
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
                        _ => {}
                    }
                }
            }
        }
        for (target, place, owner) in places {
            if target == Target::Tokens && place.starts_with("icons.sizes.") {
                continue;
            }
            let (absent, others): (&str, &[&str]) = match owner {
                "Edge" => ("dash", &["width", "paint"]),
                "IconStyle" => ("fill", &["rendering_mode"]),
                _ => ("fill", &["stroke"]),
            };
            let mut seeds = vec![Seed::Field(target, below(&place, absent), false)];
            seeds.extend(
                others
                    .iter()
                    .map(|other| Seed::Field(target, below(&place, other), false)),
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
                let absents: Vec<Found<'_>> = stack
                    .effective(world, target, &below(&place, absent))
                    .into_iter()
                    .filter(|found| matches!(found.got, Got::Absent))
                    .collect();
                if absents.is_empty() {
                    continue;
                }
                let other = |name: &str| stack.effective(world, target, &below(&place, name));
                let refused = match owner {
                    "Edge" => ["width", "paint"]
                        .into_iter()
                        .all(|name| other(name).iter().any(|f| !matches!(f.got, Got::Absent))),
                    "IconStyle" => other("rendering_mode")
                        .iter()
                        .any(|f| variant(f) != Some("original") && !matches!(f.got, Got::Absent)),
                    _ => other("stroke").iter().any(|f| matches!(f.got, Got::Absent)),
                };
                if refused {
                    for found in absents {
                        self.problem(found.layer, found.at, ProblemKind::AbsentNotAllowed);
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
        // a `DrawnAs` reads them (`reaches_again`).
        let (atoms, all) = stack.enumerate(&rules, &widgets, |atoms| {
            atoms
                .iter()
                .enumerate()
                .filter(|(_, atom)| {
                    matches!(atom, Atom::Cond(_) | Atom::State(_) | Atom::Variant(_))
                })
                .map(|(index, _)| Some(vec![(index, true)]))
                .collect()
        });
        for first in &parts {
            for world in &all {
                let (Shown::DrawnAs { .. }, Some((layer, at))) = stack.shown(world, *first) else {
                    continue;
                };
                if self.reaches_again(&atoms, &all, *first, world) {
                    self.problem(layer, at, ProblemKind::DrawnAsCycle);
                }
            }
        }
    }

    /// Whether `first`, drawn as another widget under `world`, is drawn
    /// again through the `DrawnAs` patches it leads to.
    fn reaches_again(
        &self,
        atoms: &[Atom],
        all: &[World],
        first: TypedPart,
        world: &World,
    ) -> bool {
        let stack = self.stack;
        let mut seen: Vec<(TypedPart, usize)> = Vec::new();
        let mut pending: Vec<(TypedPart, &World)> = vec![(first, world)];
        while let Some((part, host)) = pending.pop() {
            let (Shown::DrawnAs { target, variants }, _) = stack.shown(host, part) else {
                continue;
            };
            let drawn: Vec<TypedPart> = match target {
                WidgetOrPart::Part(crate::next::selector::PartRef::Typed(part)) => vec![*part],
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
                | WidgetOrPart::Part(crate::next::selector::PartRef::Component { .. }) => {
                    Vec::new()
                }
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
                    if !matches!(stack.shown(next, *part).0, Shown::DrawnAs { .. }) {
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
        for (target, place) in places {
            let states = stack.of(target).any(|(_, source)| {
                source.ops.iter().any(|op| {
                    (op.path == place || is_above(place, &op.path))
                        && matches!(op.kind, OpKind::Set(Val::Stated(Fact::Variant("ellipse"))))
                })
            });
            if !states {
                continue;
            }
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
                        SideLen::of(&stack.effective(
                            world,
                            target,
                            &format!("{focus}.offset.{side}"),
                        ))
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
            for corner in CORNERS {
                let target = Target::Part(part);
                let corner_path = format!("corners.{corner}");
                let mut seeds = vec![Seed::Field(target, corner_path.clone(), false)];
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
                    let inset = |side: &str| {
                        chain.iter().fold(
                            SideLen::Known(0.0, Vec::new()),
                            |sum, (holder, field)| {
                                let founds = stack.effective(
                                    world,
                                    Target::Part(*holder),
                                    &format!("{field}.{side}"),
                                );
                                sum.plus(SideLen::of(&founds))
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
    /// world of the bases and rules before it leaves the field unstated.
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
                let OpKind::Fields { unset, entry: true } = &op.kind else {
                    continue;
                };
                let required: Vec<&str> = unset
                    .iter()
                    .copied()
                    .filter(|field| {
                        registry::find(table(source.target), &below(&op.path, field))
                            .is_none_or(|info| !info.may_be_absent)
                    })
                    .collect();
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

/// S4.4's owner order of the text slots.
const OWNER_ORDER: [&str; 19] = [
    "body",
    "caption",
    "title1",
    "title2",
    "title3",
    "headline",
    "subheadline",
    "large_title",
    "display_large",
    "display_medium",
    "display_small",
    "body_emphasized",
    "callout",
    "footnote",
    "caption_emphasized",
    "caption2",
    "label",
    "monospace",
    "numeric",
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
/// checks (S9.1, S14); no preset is bundled yet (M1b adds `kde-breeze`).
pub(super) fn on_kde_breeze(preset: Option<PresetId>) -> bool {
    preset.is_some_and(|preset| match preset {})
}

/// On `kde-breeze`, a mapping palette (one stating no `kde_scheme` input:
/// a port converts to them, S14) at `palette` in the stack leaving an
/// accent-dependent role (S9.1's list, every other key of every declared set
/// too wherever `token.kde.tint_factor` can be `Some`) or an inactive input
/// role to a layer below it, under some combination, for a scheme it
/// states (S9.2, S12, S14).
pub(super) fn mapping_palette(stack: &Stack<'_>, palette: usize) -> Vec<Problem> {
    let mut problems = Vec::new();
    let Some(layer) = stack.layers.get(palette).map(|stacked| stacked.layer) else {
        return problems;
    };
    let decl = stack.decl();
    let current: Vec<&crate::next::layer::Bases> = std::iter::once(&layer.bases)
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
    let tints = {
        let seeds = vec![Seed::Field(
            Target::Tokens,
            "kde.tint_factor".to_owned(),
            false,
        )];
        let worlds = stack.worlds(seeds);
        worlds.iter().any(|world| {
            stack
                .effective(world, Target::Tokens, "kde.tint_factor")
                .iter()
                .any(|found| !matches!(found.got, Got::Stated(Fact::Nothing) | Got::Absent))
        })
    };
    let mut roles: Vec<String> = Vec::new();
    for set in &decl.surface_sets {
        use crate::next::inputs::KdeSchemeKey as K;
        use crate::next::roles::SurfaceSet as S;
        let mut keys: Vec<K> = vec![
            K::ForegroundActive,
            K::ForegroundLink,
            K::DecorationFocus,
            K::DecorationHover,
        ];
        match set {
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
        if tints {
            keys = K::ALL.to_vec();
        }
        for key in keys {
            let role = format!("surfaces.{}.{}", set.name(), key.role().path());
            if !roles.contains(&role) {
                roles.push(role);
            }
        }
    }
    for key in crate::next::inputs::KdeWmKey::ALL {
        let name = format!("wm_{}", key.name());
        if decl.color_roles.iter().any(|held| held.as_str() == name) {
            roles.push(format!("keyed.{name}"));
        }
    }
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
