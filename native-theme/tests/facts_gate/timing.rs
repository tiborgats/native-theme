//! The time the gate's widget checks spend in each of their steps, summed
//! over every thread, printed after each preset's and flavour's check where
//! [`TIMING_VARIABLE`] is set; nothing is timed where it is unset, the
//! default. It measures the gate; the report never holds it.

use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// The environment variable that turns the timing on (the module's docs of
/// `facts_gate`).
pub const TIMING_VARIABLE: &str = "NT_FACTS_GATE_TIMING";

/// A step of the widget checks whose time is summed.
#[derive(Clone, Copy, Debug)]
pub enum Step {
    /// A condition vector resolved for both passes.
    Resolve,
    /// A combination probed, all of what follows.
    Probe,
    /// The probe's session: the names read, the scopes, the `not_drawn`
    /// entries matched, the parts queried.
    Session,
    /// The probe's facts that hold and those that apply.
    Applying,
    /// The probe's signature.
    Signature,
    /// A distinct combination's keys compared.
    Compare,
    /// A perturbed pass that replayed the normal pass's probes.
    Replayed,
    /// A perturbed pass whose resolution reads unlike the normal pass's,
    /// its combinations probed.
    Unlike,
    /// A perturbed pass one of whose distinct combinations probed unlike
    /// the normal pass's, its combinations probed.
    Differed,
    /// A pass one of whose distinct combinations probed unlike with its
    /// parts' styles queried, its combinations probed so.
    Unshaped,
}

/// Every step, each at its own place, in the order the timing prints them.
const STEPS: [Step; 10] = [
    Step::Resolve,
    Step::Probe,
    Step::Session,
    Step::Applying,
    Step::Signature,
    Step::Compare,
    Step::Replayed,
    Step::Unlike,
    Step::Differed,
    Step::Unshaped,
];

/// Each step's time in nanoseconds and how often it ran, by its place.
static NANOS: [AtomicU64; STEPS.len()] = [const { AtomicU64::new(0) }; STEPS.len()];
static COUNTS: [AtomicU64; STEPS.len()] = [const { AtomicU64::new(0) }; STEPS.len()];

/// Whether [`TIMING_VARIABLE`] is set, read once.
fn on() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os(TIMING_VARIABLE).is_some())
}

/// `f`, its time added to `step`'s where the timing is on.
pub fn timed<R>(step: Step, f: impl FnOnce() -> R) -> R {
    if !on() {
        return f();
    }
    let started = Instant::now();
    let out = f();
    let nanos = u64::try_from(started.elapsed().as_nanos()).unwrap_or(u64::MAX);
    if let Some(slot) = NANOS.get(step as usize) {
        slot.fetch_add(nanos, Ordering::Relaxed);
    }
    if let Some(slot) = COUNTS.get(step as usize) {
        slot.fetch_add(1, Ordering::Relaxed);
    }
    out
}

/// `step` counted once where the timing is on, taking no time.
pub fn note(step: Step) {
    if on()
        && let Some(slot) = COUNTS.get(step as usize)
    {
        slot.fetch_add(1, Ordering::Relaxed);
    }
}

/// Whether the timing is on, so a caller measures what [`sizes`] prints
/// only then.
pub fn measuring() -> bool {
    on()
}

/// The sizes `sizes` names, in bytes or counts, and the process's resident
/// memory (Linux's `/proc/self/status`, else unread), printed for `what`
/// where the timing is on.
pub fn sizes(what: &str, sizes: &[(&str, usize)]) {
    if !on() {
        return;
    }
    let resident = std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|status| {
            let line = status.lines().find(|line| line.starts_with("VmRSS:"))?;
            line.split_whitespace().nth(1)?.parse::<u64>().ok()
        })
        .map_or_else(|| "unread".to_owned(), |kb| format!("{} MB", kb >> 10));
    let mut line = format!("sizes {what}: resident {resident}");
    for (name, size) in sizes {
        line.push_str(&format!("; {name} {size}"));
    }
    eprintln!("{line}");
}

/// Each step's time and count since the last print, printed for `what`
/// where the timing is on, then started over.
pub fn print(what: &str, wall: std::time::Duration) {
    if !on() {
        return;
    }
    let mut line = format!("timing {what}: wall {:.1}s", wall.as_secs_f64());
    for ((step, nanos), count) in STEPS.iter().zip(&NANOS).zip(&COUNTS) {
        let nanos = nanos.swap(0, Ordering::Relaxed);
        let count = count.swap(0, Ordering::Relaxed);
        #[allow(clippy::cast_precision_loss)]
        let seconds = nanos as f64 / 1e9;
        line.push_str(&format!("; {step:?} {seconds:.1}s over {count}"));
    }
    eprintln!("{line}");
}
