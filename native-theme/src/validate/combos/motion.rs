//! S12's checks of transitions and presentation steps over combinations
//! (S2.7): what each one's merged value can be under the worlds it is in
//! effect in, its timing followed through references.

use super::super::checks::{Read, takes, test_holds};
use super::super::stack::{
    Fact, Item as Listed, ItemKey, Op, OpKind, Source, Stack, Target, Val, below, is_above,
    part_info, tail,
};
use super::{Bound, Checks, Found, Got, Seed, World, all_parts, cx_of, got_of, variant};
use crate::blocks::motion::{BreezeTest, PropertyGroup};
use crate::error::ProblemKind;

/// A place a transition or a presentation step is stated at, as the
/// checks read it.
struct Motion<'s> {
    target: Target,
    /// Its place: a list's item (`transitions[3:0]`), a presentation's
    /// step (`presentations[2:0].steps[1]`), or `focus.transition`.
    place: String,
    /// The keyed item holding it: its source's number, the source and the
    /// item's place; `None` for a `FocusIndicator.transition`, a field read
    /// merged.
    item: Option<(usize, &'s Source, String)>,
    /// A presentation step's.
    step: bool,
}

impl Motion<'_> {
    /// Whether it is in effect under `world`: its item is ([`in_effect`]);
    /// a field's always is.
    fn in_effect(&self, stack: &Stack<'_>, world: &World) -> bool {
        self.item
            .as_ref()
            .is_none_or(|(number, source, item)| in_effect(stack, world, *number, source, item))
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

/// Whether the source states a value at `place`.
fn states_at(source: &Source, place: &str) -> bool {
    source.ops.iter().any(|op| op.path == place)
}

/// Whether the source states a value at `place`, above it or below it.
fn states_at_or_near(source: &Source, place: &str) -> bool {
    source.ops.iter().any(|op| {
        !matches!(op.kind, OpKind::Shown(_))
            && (op.path == place || is_above(&op.path, place) || is_above(place, &op.path))
    })
}

/// Every transition and presentation step the stack states: each keyed
/// list's transitions and each presentation's steps, and the focus
/// indicator's transition of the tokens and of each part with a `focus`,
/// where a source states it.
fn motions<'s>(stack: &'s Stack<'_>) -> Vec<Motion<'s>> {
    let mut out = Vec::new();
    for (number, source) in stack.sources.iter().enumerate() {
        for item in &source.items {
            let held = Some((number, source, item.place.clone()));
            if item.place.starts_with("transitions[") {
                out.push(Motion {
                    target: source.target,
                    place: item.place.clone(),
                    item: held,
                    step: false,
                });
            } else if item.place.starts_with("presentations[") {
                let steps = format!("{}.steps[", item.place);
                let mut places: Vec<String> = Vec::new();
                for op in &source.ops {
                    if let Some((index, _)) = op
                        .path
                        .strip_prefix(&steps)
                        .and_then(|rest| rest.split_once(']'))
                    {
                        let place = format!("{steps}{index}]");
                        if !places.contains(&place) {
                            places.push(place);
                        }
                    }
                }
                out.extend(places.into_iter().map(|place| Motion {
                    target: source.target,
                    place,
                    item: held.clone(),
                    step: true,
                }));
            }
        }
    }
    let focus = "focus.transition";
    let mut targets = vec![Target::Tokens];
    targets.extend(
        all_parts()
            .into_iter()
            .filter(|part| {
                part_info(*part).is_some_and(|info| info.fields.iter().any(|f| f.path == "focus"))
            })
            .map(Target::Part),
    );
    for target in targets {
        if stack
            .of(target)
            .any(|(_, source)| states_at_or_near(source, focus))
        {
            out.push(Motion {
                target,
                place: focus.to_owned(),
                item: None,
                step: false,
            });
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
    /// The value of the field `field` of the transition or step at `place`
    /// of `target` under `world`: a field `Absent` above the place (the
    /// focus indicator, or its transition) states none of it.
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

    /// The Breeze test of a found engine, as its source states it.
    fn breeze_test(&self, engine: &Found<'_>) -> Option<&BreezeTest> {
        let (number, _) = engine.id?;
        let source = self.sources.get(number)?;
        source
            .tests
            .iter()
            .find(|(place, _)| place == engine.cx.path)
            .map(|(_, test)| test)
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
    /// Every transition's and presentation step's zero durations, absences
    /// and timings, under each world it is in effect in, the merged value
    /// read (S2.7, S12; ADD-A125, ADD-A126): a timing `Absent` where the
    /// duration is not a stated zero, but a transition's under
    /// `WinUiAnimatedIcon`; a stated zero's timing stated; a duration
    /// `Absent` where the timing, followed through references, is no
    /// spring, but under `WinUiAnimatedIcon`; an `Instant` step's duration
    /// not a stated zero, and another kind's a stated zero; and a
    /// transition's stated timing, followed through references, that its
    /// engine does not compute; a list's item's delay written as an
    /// expression that resolves to another than zero where its engine takes
    /// none, and the focus indicator's transition's engine checks
    /// ([`Checks::focus_engine`]). A value nothing states (a listed path's)
    /// or a reference reaching none refuses nothing.
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

    /// [`Checks::motions`]' checks of one transition or step under `world`.
    fn motion(&mut self, world: &World, motion: &Motion<'_>) {
        let stack = self.stack;
        let at = |field: &str| stack.motion_field(world, motion.target, &motion.place, field);
        if motion.item.is_none() {
            self.focus_engine(world, motion.target);
        }
        let durations = at("duration");
        let timings = at("timing");
        let engines: Vec<&'static str> = at("engine").iter().filter_map(variant).collect();
        let kinds: Vec<&'static str> = at("kind").iter().filter_map(variant).collect();
        // A list's item's delay written as an expression, where it
        // resolves (a literal is the structural check's, S12).
        if motion.item.is_some() && !motion.step {
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
        }
        if kinds.is_empty() || (!motion.step && engines.is_empty()) {
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
        if motion.step {
            if kinds.contains(&"instant") {
                for found in durations.iter().filter(|found| !stated_zero(found)) {
                    self.problem(found.layer, found.at, ProblemKind::InstantNotZero);
                }
            }
            if kinds.iter().any(|kind| *kind != "instant") {
                for found in durations.iter().filter(|found| stated_zero(found)) {
                    self.problem(found.layer, found.at, ProblemKind::ZeroNotInstant);
                }
            }
            return;
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
    /// S12's engine checks ([`takes`], [`test_holds`]) of the focus
    /// indicator's transition of `target`, its merged value read under
    /// `world`, as a rule patches it field by field (S2.7; ADD-A126): each
    /// stated engine against each value of the fields it reads, a refusal
    /// at the field's place, and its Breeze test against each duration. The
    /// indicator's transition is no part's `Geometry` one: it lists no group
    /// (S2.10). A value the check cannot read (a listed path's) refuses
    /// nothing.
    fn focus_engine(&mut self, world: &World, target: Target) {
        let stack = self.stack;
        let at = |field: &str| -> Vec<Found<'_>> {
            stack
                .motion_field(world, target, "focus.transition", field)
                .into_iter()
                .filter(|found| !matches!(found.got, Got::Stated(Fact::Unknown)))
                .collect()
        };
        let durations = at("duration");
        let part = match target {
            Target::Part(part) => Some(part),
            Target::Roles(_) | Target::Tokens | Target::Values(_) => None,
        };
        let mut reads: Vec<(Found<'_>, Read)> = Vec::new();
        reads.extend(durations.iter().map(|found| {
            let absent = matches!(found.got, Got::Absent);
            (*found, Read::Duration { absent })
        }));
        reads.extend(at("timing").into_iter().map(|found| {
            let absent = matches!(found.got, Got::Absent);
            (found, Read::Timing { absent })
        }));
        reads.extend(
            at("delay")
                .into_iter()
                .filter(|found| stack.delayed(world, *found))
                .map(|found| (found, Read::Delay)),
        );
        reads.extend(
            at("kind")
                .iter()
                .filter_map(|found| variant(found).map(|kind| (*found, Read::Kind(kind)))),
        );
        for engine in at("engine") {
            let Some(name) = variant(&engine) else {
                continue;
            };
            let place = Read::Place {
                geometry: false,
                part,
            };
            for (found, read) in std::iter::once((engine, place)).chain(reads.iter().copied()) {
                if let Some(takes) = takes(name, read) {
                    let kind = ProblemKind::EngineMismatch {
                        engine: name,
                        takes,
                    };
                    self.problem(found.layer, found.at, kind);
                }
            }
            let Some(test) = stack.breeze_test(&engine) else {
                continue;
            };
            let test_at = format!("{}.{name}", engine.at);
            for zero in [true, false] {
                if !durations.iter().any(|found| stated_zero(found) == zero) {
                    continue;
                }
                for (list, expected) in test_holds(test, zero, true) {
                    let at =
                        list.map_or_else(|| test_at.clone(), |list| format!("{test_at}.{list}"));
                    self.problem(
                        engine.layer,
                        &at,
                        ProblemKind::BreezeTestInvalid { expected },
                    );
                }
            }
        }
    }

    /// A `GtkCss` part's transitions in effect together under one world
    /// naming several curves, followed through references, while one of
    /// them lists a group other than `Shadow` and `Corners`, the groups GTK's
    /// engine takes a curve for by property id (S2.7, S12; ADD-A131):
    /// refused at the timing of each transition listing another group. The
    /// part's focus indicator's transition is one of them, GTK keeping one
    /// CSS list per node; it lists no group. A curve is its timing's facts;
    /// a timing the check cannot read, and a stated zero's, names none. That
    /// such a list follows the stylesheet's one CSS list is
    /// `check_fact_citations.py`'s check of the facts.
    pub(super) fn css_curves(&mut self) {
        let stack = self.stack;
        let focus = "focus.transition";
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
            let focused = stack
                .of(target)
                .any(|(_, source)| states_at_or_near(source, focus));
            if items.len().saturating_add(usize::from(focused)) < 2 {
                continue;
            }
            let mut seeds = vec![Seed::Field(target, "transitions".to_owned(), true)];
            if focused {
                seeds.push(Seed::Field(target, focus.to_owned(), true));
            }
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
                if focused {
                    Self::css_curve(stack, world, target, focus, &mut curves);
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
    /// an earlier rule of its part, applies; for a
    /// `FocusIndicator.transition`, no transition of the indicator below
    /// it. Where none can, no transition is stated. A delay is read by its
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
            self.focus_zero(number, source);
        }
    }

    /// [`Checks::zeros`] for a focus indicator's transition stated a zero
    /// by `source`, the source `number`: refused where, under some world
    /// where the source applies and its delay is none, no transition of the
    /// indicator lies below it. A delay the source leaves unstated is the
    /// one below it.
    fn focus_zero(&mut self, number: usize, source: &Source) {
        let stack = self.stack;
        let place = "focus.transition";
        let Some(duration) = zero_at(source, &below(place, "duration")) else {
            return;
        };
        let delay = below(place, "delay");
        let below_it = Bound {
            before: Some(source.pos),
            bases_only: false,
        };
        let seeds = vec![Seed::Field(source.target, place.to_owned(), true)];
        let refused = stack.worlds(seeds).iter().any(|world| {
            let no_delay = if states_at(source, &delay) {
                stack.no_delay_stated(world, number, source, &delay)
            } else {
                stack
                    .value_at(world, source.target, &delay, below_it, 0)
                    .iter()
                    .all(|found| stack.undelayed(world, *found))
            };
            let overrides = stack
                .value_at(world, source.target, &below(place, "engine"), below_it, 0)
                .iter()
                .any(|found| !matches!(found.got, Got::Absent | Got::Auto));
            stack.applies(world, source) && no_delay && !overrides
        });
        if refused {
            self.problem(source.layer, &duration.at, ProblemKind::OverridesNothing);
        }
    }
}
