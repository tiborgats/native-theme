//! S12's checks of transitions and presentation steps over combinations
//! (S2.7): what each one's merged value can be under the worlds it is in
//! effect in, its timing followed through references.

use super::super::stack::{
    Fact, OpKind, Source, Stack, Target, Val, below, is_above, part_info, tail,
};
use super::{Bound, Checks, Found, Got, Seed, World, all_parts, variant};
use crate::next::error::ProblemKind;

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
    /// Whether it is in effect under `world`: its source applies and no
    /// later item replaces it (S6.1); a field's always is.
    fn in_effect(&self, stack: &Stack<'_>, world: &World) -> bool {
        self.item.as_ref().is_none_or(|(number, source, item)| {
            stack.applies(world, source)
                && !stack
                    .replaced(world, *number, source)
                    .iter()
                    .any(|replaced| replaced == item)
        })
    }
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
        let states = stack.of(target).any(|(_, source)| {
            source.ops.iter().any(|op| {
                !matches!(op.kind, OpKind::Shown(_))
                    && (op.path == focus || is_above(&op.path, focus) || is_above(focus, &op.path))
            })
        });
        if states {
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
    matches!(found.got, Got::Stated(Fact::Number(n)) if n == 0.0)
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
        "gtk_css" => &["cubic_bezier", "linear", "steps"],
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
    /// engine does not compute. A value nothing states (a listed path's) or
    /// a reference reaching none refuses nothing.
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
        // A field `Absent` above the place (the focus indicator, or its
        // transition) states none of it.
        let at = |field: &str| -> Vec<Found<'_>> {
            stack
                .value_at(
                    world,
                    motion.target,
                    &below(&motion.place, field),
                    Bound::ALL,
                    0,
                )
                .into_iter()
                .filter(|found| !(found.above && matches!(found.got, Got::Absent | Got::Auto)))
                .collect()
        };
        let durations = at("duration");
        let timings = at("timing");
        let engines: Vec<&'static str> = at("engine").iter().filter_map(variant).collect();
        let kinds: Vec<&'static str> = at("kind").iter().filter_map(variant).collect();
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
