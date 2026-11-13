//! S12's checks of transitions over combinations (S2.7): what each one's
//! merged value can be under the worlds it is in effect in, its timing
//! followed through references. The presentations' steps are staged with
//! the motion unit (S6.1).

use super::super::checks::{Read, takes};
use super::super::stack::{
    Fact, Item as Listed, ItemKey, Op, OpKind, Source, Stack, Target, Val, below, is_above, tail,
};
use super::{Bound, Checks, Found, Got, Seed, World, all_parts, cx_of, got_of, variant};
use crate::blocks::motion::PropertyGroup;
use crate::error::ProblemKind;

/// A place a transition is stated at, as the checks read it.
struct Motion<'s> {
    target: Target,
    /// Its place: a list's item (`transitions[3:0]`).
    place: String,
    /// The keyed item holding it: its source's number, the source and the
    /// item's place.
    item: (usize, &'s Source, String),
}

impl Motion<'_> {
    /// Whether it is in effect under `world`: its item is ([`in_effect`]).
    fn in_effect(&self, stack: &Stack<'_>, world: &World) -> bool {
        let (number, source, item) = &self.item;
        in_effect(stack, world, *number, source, item)
    }
}

/// Whether the keyed item at `item` the source `number` states is in effect
/// under `world`: its source applies and no later item replaces it (S6.1).
fn in_effect(stack: &Stack<'_>, world: &World, number: usize, source: &Source, item: &str) -> bool {
    stack.applies(world, source) && !stack.replaced(world, number, source).contains(&item)
}

/// The property groups a transition's item is keyed by: the groups it
/// lists (S6.1).
fn groups(item: &Listed) -> Vec<PropertyGroup> {
    item.keys
        .iter()
        .filter_map(|key| match key {
            ItemKey::Group(group, ..) => Some(*group),
            ItemKey::Role(_) | ItemKey::Surface => None,
        })
        .collect()
}

/// The groups a transition of `group` overrides where it governs (S2.7):
/// its own, and the stroke's or the glyph's whole transition for the
/// group that times their widths or their path in its place.
fn overridden(group: PropertyGroup) -> Vec<PropertyGroup> {
    match group {
        PropertyGroup::StrokeWidth => vec![group, PropertyGroup::Stroke],
        PropertyGroup::GlyphDraw => vec![group, PropertyGroup::Glyph],
        PropertyGroup::Fill
        | PropertyGroup::Stroke
        | PropertyGroup::Corners
        | PropertyGroup::Shape
        | PropertyGroup::Shadow
        | PropertyGroup::Opacity
        | PropertyGroup::Transform
        | PropertyGroup::Text
        | PropertyGroup::Icon
        | PropertyGroup::Glyph
        | PropertyGroup::Geometry
        | PropertyGroup::StateLayer => vec![group],
    }
}

/// Whether the source states a zero at `place` (a stated zero's duration,
/// a delay of none), and the step stating it.
fn zero_at<'s>(source: &'s Source, place: &str) -> Option<&'s Op> {
    source.ops.iter().find(|op| {
        op.path == place
            && matches!(op.kind, OpKind::Set(Val::Stated(Fact::Number(n, _))) if n == 0.0)
    })
}

/// The value the source `number` states at `place`, as a lookup finds it.
fn stated_at<'s>(number: usize, source: &'s Source, place: &str) -> Option<Found<'s>> {
    let (index, op) = source
        .ops
        .iter()
        .enumerate()
        .find(|(_, op)| op.path == place)?;
    let OpKind::Set(val) = &op.kind else {
        return None;
    };
    Some(Found {
        got: got_of(val),
        at: &op.at,
        cx: cx_of(source, op),
        layer: source.layer,
        id: Some((number, index)),
        via: false,
        above: false,
    })
}

/// Every transition the stack states: each keyed list's transitions. The
/// focus indicator's own transition and the presentations are staged with
/// the motion unit (S6.1).
fn motions<'s>(stack: &'s Stack<'_>) -> Vec<Motion<'s>> {
    let mut out = Vec::new();
    for (number, source) in stack.sources.iter().enumerate() {
        for item in &source.items {
            if item.place.starts_with("transitions[") {
                out.push(Motion {
                    target: source.target,
                    place: item.place.clone(),
                    item: (number, source, item.place.clone()),
                });
            }
        }
    }
    out
}

/// Whether a found duration is a stated zero (S2.7): the literal zero in
/// the field, never a reference or an expression.
fn stated_zero(found: &Found<'_>) -> bool {
    matches!(found.got, Got::Stated(Fact::Number(n, _)) if n == 0.0)
}

/// Whether a timing followed through references can be a spring: a
/// spring, or a value the check cannot read (a reference reaching nothing
/// stated, a listed path's), which refuses nothing.
fn can_be_spring(end: &Found<'_>) -> bool {
    matches!(end.got, Got::Stated(Fact::Unknown) | Got::Expr(_)) || variant(end) == Some("spring")
}

/// Whether the engine S11 spells `engine` computes the stated timing
/// `facts` describes ([`Stack::timing_facts`]), its list S12's (S2.7):
/// a `Platform` timing by its family (`qt`, `adw`, `kirigami_velocity`),
/// `GtkSwitch`'s by its one curve. A timing or an engine the check cannot
/// read refuses nothing, and neither does `WinUiAnimatedIcon`, which states
/// no timing (its own structural check).
fn computes(engine: &str, facts: &[(String, Fact)]) -> bool {
    let at = |place: &str| {
        facts.iter().find_map(|(held, fact)| match fact {
            Fact::Variant(name) if held == place => Some(*name),
            _ => None,
        })
    };
    let curve = match (at(""), at("platform")) {
        (Some("platform"), Some(family)) => family,
        (Some("platform"), None) | (None, _) => return true,
        (Some(form), _) => form,
    };
    let listed: &[&str] = match engine {
        "gtk_css" | "css_animation" => &["cubic_bezier", "linear", "steps"],
        "breeze" | "breeze_tab_bar" => &["linear", "steps", "qt"],
        "qt_quick" => &["linear", "cubic_bezier", "qt", "kirigami_velocity"],
        "adw_animation" => &["linear", "cubic_bezier", "adw", "spring"],
        "compose_animatable" | "compose_transition" => &["linear", "cubic_bezier", "spring"],
        "compose_held" => &["spring"],
        "win_ui" => &["linear", "cubic_bezier", "steps"],
        "gtk_switch" => return curve == "adw" && at("platform.adw") == Some("ease_out_cubic"),
        _ => return true,
    };
    listed.contains(&curve)
}

impl Stack<'_> {
    /// The value of the field `field` of the transition at `place`
    /// of `target` under `world`: a field `Absent` above the place states
    /// none of it.
    fn motion_field(
        &self,
        world: &World,
        target: Target,
        place: &str,
        field: &str,
    ) -> Vec<Found<'_>> {
        self.value_at(world, target, &below(place, field), Bound::ALL, 0)
            .into_iter()
            .filter(|found| !(found.above && matches!(found.got, Got::Absent | Got::Auto)))
            .collect()
    }

    /// Whether a found delay is one other than zero under `world` (S2.7,
    /// S12): its references followed, a stated number other than zero; a
    /// value the check cannot read refuses nothing.
    fn delayed(&self, world: &World, found: Found<'_>) -> bool {
        self.follow(world, found)
            .0
            .iter()
            .any(|end| matches!(end.got, Got::Stated(Fact::Number(n, _)) if n != 0.0))
    }

    /// Whether a found delay is no delay under `world` (S2.7, S12): its
    /// references followed, each end a stated zero; a value the check
    /// cannot read is not.
    fn undelayed(&self, world: &World, found: Found<'_>) -> bool {
        let (ends, _) = self.follow(world, found);
        !ends.is_empty()
            && ends
                .iter()
                .all(|end| matches!(end.got, Got::Stated(Fact::Number(n, _)) if n == 0.0))
    }

    /// Whether the delay the source `number` states at `place` is no delay
    /// under `world`: a stated zero, or a value that resolves to zero there.
    fn no_delay_stated(&self, world: &World, number: usize, source: &Source, place: &str) -> bool {
        if zero_at(source, place).is_some() {
            return true;
        }
        stated_at(number, source, place)
            .is_some_and(|found| matches!(found.got, Got::Expr(_)) && self.undelayed(world, found))
    }

    /// What the source of a stated timing states at and below its place,
    /// each fact by its place below it (`""` the timing's variant,
    /// `platform.adw`, `cubic_bezier[0]`), in order; `None` where the
    /// found value is no stated variant.
    pub(super) fn timing_facts(&self, found: &Found<'_>) -> Option<Vec<(String, Fact)>> {
        let Got::Stated(fact @ Fact::Variant(_)) = found.got else {
            return None;
        };
        let (number, _) = found.id?;
        let source = self.sources.get(number)?;
        let mut out = vec![(String::new(), fact)];
        out.extend(source.ops.iter().filter_map(|op| match &op.kind {
            OpKind::Set(Val::Stated(fact)) if is_above(found.cx.path, &op.path) => {
                Some((tail(found.cx.path, &op.path).to_owned(), *fact))
            }
            _ => None,
        }));
        Some(out)
    }
}

impl Checks<'_, '_> {
    /// Every transition's zero durations, absences and timings, under each
    /// world it is in effect in, the merged value read (S2.7, S12;
    /// ADD-A125, ADD-A126): a timing `Absent` where the duration is not a
    /// stated zero, but under `WinUiAnimatedIcon`; a stated zero's timing
    /// stated; a duration `Absent` where the timing, followed through
    /// references, is no spring, but under `WinUiAnimatedIcon`; and a
    /// transition's stated timing, followed through references, that its
    /// engine does not compute; a list's item's delay written as an
    /// expression that resolves to another than zero where its engine takes
    /// none. A value nothing states (a listed path's) or a reference
    /// reaching none refuses nothing.
    pub(super) fn motions(&mut self) {
        let stack = self.stack;
        for motion in motions(stack) {
            let seeds = vec![Seed::Field(motion.target, motion.place.clone(), true)];
            let worlds = stack.worlds(seeds);
            for world in worlds.iter() {
                if motion.in_effect(stack, world) {
                    self.motion(world, &motion);
                }
            }
        }
    }

    /// [`Checks::motions`]' checks of one transition under `world`.
    fn motion(&mut self, world: &World, motion: &Motion<'_>) {
        let stack = self.stack;
        let at = |field: &str| stack.motion_field(world, motion.target, &motion.place, field);
        let durations = at("duration");
        let timings = at("timing");
        let engines: Vec<&'static str> = at("engine").iter().filter_map(variant).collect();
        let kinds: Vec<&'static str> = at("kind").iter().filter_map(variant).collect();
        // A list's item's delay written as an expression, where it
        // resolves (a literal is the structural check's, S12).
        for found in at("delay") {
            if matches!(found.got, Got::Expr(_)) && stack.delayed(world, found) {
                for engine in &engines {
                    if let Some(takes) = takes(engine, Read::Delay) {
                        let kind = ProblemKind::EngineMismatch { engine, takes };
                        self.problem(found.layer, found.at, kind);
                    }
                }
            }
        }
        if kinds.is_empty() || engines.is_empty() {
            return;
        }
        let zero = durations.iter().any(stated_zero);
        let other = durations.iter().any(|found| !stated_zero(found));
        let win_ui = engines.contains(&"win_ui_animated_icon");
        let followed: Vec<(Found<'_>, Vec<Found<'_>>)> = timings
            .iter()
            .map(|found| (*found, stack.follow(world, *found).0))
            .collect();
        for (found, _) in &followed {
            match found.got {
                Got::Absent if other && !win_ui => {
                    self.problem(found.layer, found.at, ProblemKind::TimingAbsent);
                }
                Got::Absent => {}
                Got::Auto | Got::Expr(_) | Got::Stated(_) if zero => {
                    self.problem(found.layer, found.at, ProblemKind::TimingOnZero);
                }
                Got::Auto | Got::Expr(_) | Got::Stated(_) => {}
            }
        }
        let spring = followed
            .iter()
            .any(|(_, ends)| ends.iter().any(can_be_spring));
        if !win_ui && !spring && !timings.is_empty() {
            for found in durations.iter().filter(|f| matches!(f.got, Got::Absent)) {
                self.problem(found.layer, found.at, ProblemKind::DurationAbsent);
            }
        }
        // A stated zero states no timing, so the lists read a stated one
        // only (S12).
        if !other {
            return;
        }
        for (found, ends) in &followed {
            if matches!(found.got, Got::Absent) {
                continue;
            }
            for facts in ends.iter().filter_map(|end| stack.timing_facts(end)) {
                for engine in &engines {
                    if !computes(engine, &facts) {
                        let kind = ProblemKind::TimingNotComputed { engine };
                        self.problem(found.layer, found.at, kind);
                    }
                }
            }
        }
    }
}

impl Checks<'_, '_> {
    /// A `GtkCss` part's transitions in effect together under one world
    /// naming several curves, followed through references, while one of
    /// them lists a group other than `Shadow` and `Corners`, the groups GTK's
    /// engine takes a curve for by property id (S2.7, S12; ADD-A131):
    /// refused at the timing of each transition listing another group. A
    /// curve is its timing's facts; a timing the check cannot read, and a
    /// stated zero's, names none. That such a list follows the stylesheet's
    /// one CSS list is `check_fact_citations.py`'s check of the facts.
    pub(super) fn css_curves(&mut self) {
        let stack = self.stack;
        for part in all_parts() {
            let target = Target::Part(part);
            let items: Vec<(usize, &Source, &Listed)> = stack
                .of(target)
                .flat_map(|(number, source)| {
                    source
                        .items
                        .iter()
                        .filter(|item| item.place.starts_with("transitions["))
                        .map(move |item| (number, source, item))
                })
                .collect();
            if items.len() < 2 {
                continue;
            }
            let seeds = vec![Seed::Field(target, "transitions".to_owned(), true)];
            let worlds = stack.worlds(seeds);
            for world in worlds.iter() {
                let mut curves: Vec<Vec<(String, Fact)>> = Vec::new();
                let mut wide: Vec<Found<'_>> = Vec::new();
                for (number, source, item) in &items {
                    if !in_effect(stack, world, *number, source, &item.place) {
                        continue;
                    }
                    let timings = Self::css_curve(stack, world, target, &item.place, &mut curves);
                    let other = groups(item).iter().any(|group| {
                        !matches!(group, PropertyGroup::Shadow | PropertyGroup::Corners)
                    });
                    if other {
                        wide.extend(timings);
                    }
                }
                if curves.len() > 1 {
                    for found in wide {
                        self.problem(found.layer, found.at, ProblemKind::SeveralCurves);
                    }
                }
            }
        }
    }

    /// [`Checks::css_curves`]' read of the transition at `place` under
    /// `world`: where its engine is `GtkCss` and its duration no stated
    /// zero, its stated timings, each curve they name added to `curves`;
    /// none elsewhere. A field `Absent` above the place states none of it.
    fn css_curve<'s>(
        stack: &'s Stack<'_>,
        world: &World,
        target: Target,
        place: &str,
        curves: &mut Vec<Vec<(String, Fact)>>,
    ) -> Vec<Found<'s>> {
        let at = |field: &str| stack.motion_field(world, target, place, field);
        // A stated zero runs no curve: its stated timing is refused on its
        // own.
        if !at("engine")
            .iter()
            .any(|found| variant(found) == Some("gtk_css"))
            || at("duration").iter().any(stated_zero)
        {
            return Vec::new();
        }
        let timings: Vec<Found<'_>> = at("timing")
            .into_iter()
            .filter(|found| !matches!(found.got, Got::Absent))
            .collect();
        for found in &timings {
            for end in stack.follow(world, *found).0 {
                if let Some(facts) = stack.timing_facts(&end)
                    && !curves.contains(&facts)
                {
                    curves.push(facts);
                }
            }
        }
        timings
    }

    /// A stated zero with no delay that overrides nothing (S2.7, S12;
    /// ADD-A126; one with a delay holds its start): refused where, under
    /// some world where its own source applies and its delay is none, no
    /// other transition of one of its groups (a group timing the stroke's
    /// widths or the glyph's path also overriding the stroke's or the
    /// glyph's whole transition), earlier in its own list, in the base or in
    /// an earlier rule of its part, applies. Where none can, no transition
    /// is stated. A delay is read by its
    /// value, a reference followed, in that world: one resolving to zero
    /// there is no delay. The states the transitions apply from are not
    /// read: any two can apply from one update's flags.
    pub(super) fn zeros(&mut self) {
        let stack = self.stack;
        for (number, source) in stack.sources.iter().enumerate() {
            for (index, item) in source.items.iter().enumerate() {
                if !item.place.starts_with("transitions[") {
                    continue;
                }
                let Some(duration) = zero_at(source, &below(&item.place, "duration")) else {
                    continue;
                };
                let delay = below(&item.place, "delay");
                if zero_at(source, &delay).is_none()
                    && stated_at(number, source, &delay)
                        .is_none_or(|found| !matches!(found.got, Got::Expr(_)))
                {
                    continue;
                }
                let overriding: Vec<PropertyGroup> =
                    groups(item).into_iter().flat_map(overridden).collect();
                let overriding = &overriding;
                let earlier: Vec<(&Source, &str)> = stack
                    .of(source.target)
                    .flat_map(|(other_number, other)| {
                        other
                            .items
                            .iter()
                            .enumerate()
                            .filter(move |(other_index, other_item)| {
                                let earlier = other.pos < source.pos
                                    || (other_number == number && *other_index < index);
                                earlier
                                    && other_item.place.starts_with("transitions[")
                                    && groups(other_item)
                                        .iter()
                                        .any(|group| overriding.contains(group))
                            })
                            .map(move |(_, other_item)| (other, other_item.place.as_str()))
                    })
                    .collect();
                let mut seeds = vec![Seed::Field(source.target, item.place.clone(), true)];
                seeds.extend(
                    earlier
                        .iter()
                        .map(|(_, place)| Seed::Field(source.target, (*place).to_owned(), true)),
                );
                let refused = stack.worlds(seeds).iter().any(|world| {
                    stack.applies(world, source)
                        && stack.no_delay_stated(world, number, source, &delay)
                        && !earlier.iter().any(|(other, _)| stack.applies(world, other))
                });
                if refused {
                    self.problem(source.layer, &duration.at, ProblemKind::OverridesNothing);
                }
            }
        }
    }
}
