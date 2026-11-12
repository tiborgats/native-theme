#!/usr/bin/env python3
"""The load budget's step: the probe's times and bytes against scripts/cost_budget.toml.

The 0.8.0 plan's *The load budget* (ADD-A147; the reasoning in
docs/todo_v0.7.1_model-completeness-d6-load.md, 2.4) gives the step four rules,
which this script applies:

1. Bound. A case whose record does not meet its line (is over it) has a named
   miss of its own; a named miss without its cause or its ledger row, a second
   named miss of one case and one keyed by a case the file does not list are
   file defects. With t the case's tolerance, a figure f fails where
   f > bound x (1 + t), the bound being the line or, under the case's named
   miss, its record where that exceeds the line. A figure over a met line by
   less than its tolerance passes, printed "over its line, within its
   tolerance". Under a named miss whose record exceeds its line,
   f < record x (1 - t) prints "record stale: re-record <case>", which fails
   the release run. A named miss whose figure meets its line "may go".
2. Figures. Bytes come from the probe's bytes binary (peak_alloc, one test
   thread) and the preset files' sizes; a time is the lowest median of its
   passes that count, over three sweeps (and in the release run further
   sweeps over the times with fewer than two passes that count, none started
   later than the bound after the build's end); a pass counts where the mean
   runnable count over it (the first number of /proc/loadavg's fourth field,
   sampled by the probe) is at most the load limit, and a time counts only
   with two passes that count. The step sets nice 19 and idle I/O itself.
3. Tolerance. A record's tolerance is a fraction, 0 for an exact count
   (line 9), else at least the floor and at most 20 %; the file holds it and
   this script only checks its range. The records are made by deliberate acts
   with their ledger rows, never by a run.
4. The release. The file defines the cases: every armed case (one with a
   record) is asked for by name; one the probe does not report (a cost-error
   line, or no line in a sweep or a bytes run that asked for it) fails "not
   reported" with its reason and is asked for no more; a case the probe lists
   as measurable that the file does not list fails "unlisted case", and a
   listing that fails or lists nothing fails by its binary's name. A case with
   no record is printed "no record yet" with the task that first measures it,
   not asked for; it passes a unit run and fails the release run, which needs
   every case armed or not binding.

The script never writes cost_budget.toml.

Usage:
  python3 scripts/check_cost_budget.py --self-test
  python3 scripts/check_cost_budget.py --check-file
  python3 scripts/check_cost_budget.py --unit       # a unit's gate: three sweeps
  python3 scripts/check_cost_budget.py --release    # the release run
Run from the repository root. --unit and --release build the probe's two test
binaries first (release, all features) and need the measuring machine.
"""

from __future__ import annotations

import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
import tomllib
from dataclasses import dataclass, field
from pathlib import Path
from typing import Callable

ROOT = Path(__file__).resolve().parent.parent
BUDGET = ROOT / "scripts" / "cost_budget.toml"

# A line's unit, in the probe's base unit (nanoseconds for a time, bytes for bytes).
UNITS = {"ms": 1e6, "us": 1e3, "MiB": 1024.0 * 1024.0, "bytes": 1.0}
TIME_UNITS = {"ms", "us"}
TOLERANCE_CEILING = 0.20
STEP_KEYS = ("load_limit", "bound_minutes", "tolerance_floor")

PROBE_LINE = re.compile(r"(cost-(?:case|pass|bytes|error|sweep)\b.*)$")

# The probe's two test binaries, by target, each with its test (the order of the listing runs).
TESTS = {"timings": "bundled_preset_costs", "cost_bytes": "bundled_preset_bytes"}


def number(value) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool)


# ---------------------------------------------------------------- the file


@dataclass
class Line:
    number: int
    what: str
    budget: float | None  # None: a figure reported, no line (line 12)
    unit: str
    exact: bool  # counted exactly: tolerance 0 (line 9)

    def is_time(self) -> bool:
        return self.unit in TIME_UNITS


@dataclass
class Entry:
    name: str
    line: int
    record: float | None = None
    tolerance: float | None = None
    row: str | None = None
    first_measured_by: str | None = None
    not_binding: str | None = None
    valid: bool = True  # False: a file defect is reported for it, and nothing is compared

    def armed(self) -> bool:
        return self.valid and self.record is not None


@dataclass
class Miss:
    case: str
    causes: list
    row: str


@dataclass
class Budget:
    load_limit: float | None
    bound_minutes: float | None
    tolerance_floor: float | None
    lines: dict
    cases: dict
    misses: dict  # case -> Miss, the first of each case
    defects: list = field(default_factory=list)

    def step_ok(self) -> bool:
        return all(number(getattr(self, key)) for key in STEP_KEYS)


def case_line(name: str) -> int | None:
    head = name.split(".", 1)[0]
    return int(head) if head.isdigit() else None


def case_preset(name: str) -> str:
    parts = name.split(".")
    return parts[1] if len(parts) > 1 else ""


def parse_budget(text: str) -> Budget:
    """The file `text` states, with its defects (rule 1's file part, rule 3's range, rule 4's not binding)."""
    defects: list[str] = []
    try:
        data = tomllib.loads(text)
    except tomllib.TOMLDecodeError as error:
        return Budget(None, None, None, {}, {}, {}, [f"file defect: not TOML: {error}"])
    step = data.get("step", {})
    if not isinstance(step, dict):
        defects.append("file defect: [step] is not a table")
        step = {}
    lines: dict[int, Line] = {}
    raw_lines = data.get("lines", {})
    if not isinstance(raw_lines, dict):
        defects.append("file defect: [lines] is not a table")
        raw_lines = {}
    for key, spec in raw_lines.items():
        if not key.isdigit() or not isinstance(spec, dict):
            defects.append(f"file defect: line {key!r} is not a numbered line")
            continue
        unit = spec.get("unit")
        if unit not in UNITS:
            defects.append(f"file defect: line {key}'s unit {unit!r} is none of {sorted(UNITS)}")
            continue
        budget = spec.get("budget")
        if budget is not None and not number(budget):
            defects.append(f"file defect: line {key}'s budget {budget!r} is no number")
            budget = None
        lines[int(key)] = Line(int(key), str(spec.get("what", "")), budget, unit, bool(spec.get("exact", False)))
    for key in STEP_KEYS:
        if not number(step.get(key)):
            defects.append(f"file defect: [step] {key} is missing or no number")
    floor = step.get("tolerance_floor") if number(step.get("tolerance_floor")) else None
    budget = Budget(
        load_limit=step.get("load_limit") if number(step.get("load_limit")) else None,
        bound_minutes=step.get("bound_minutes") if number(step.get("bound_minutes")) else None,
        tolerance_floor=floor,
        lines=lines,
        cases={},
        misses={},
        defects=defects,
    )
    raw_cases = data.get("cases", {})
    if not isinstance(raw_cases, dict):
        defects.append("file defect: [cases] is not a table")
        raw_cases = {}
    for name, spec in raw_cases.items():
        number_of_line = case_line(name)
        if number_of_line not in lines or not isinstance(spec, dict):
            defects.append(f"file defect: case {name} names no line of the file")
            continue
        entry = Entry(
            name,
            number_of_line,
            record=spec.get("record"),
            tolerance=spec.get("tolerance"),
            row=spec.get("row"),
            first_measured_by=spec.get("first_measured_by"),
            not_binding=spec.get("not_binding"),
        )
        budget.cases[name] = entry

        def bad(text: str) -> None:
            entry.valid = False
            defects.append(f"file defect: case {name} {text}")

        states = [s for s in ("record", "first_measured_by", "not_binding") if spec.get(s) is not None]
        if len(states) != 1:
            bad(f"states {states or 'nothing'}; one of a record, \"first measured by\" a named task or \"not binding\" (rule 4)")
            continue
        line = lines[number_of_line]
        if entry.not_binding is not None:
            if not isinstance(entry.not_binding, str) or not entry.not_binding.strip() or not entry.row:
                bad("not binding without its reason or its ledger row (rule 4)")
        elif entry.first_measured_by is not None:
            if not isinstance(entry.first_measured_by, str) or not entry.first_measured_by.strip():
                bad("\"first measured by\" names no task (rule 4)")
        else:
            if line.budget is None:
                bad(f"has a record on line {number_of_line}, which has no number")
                continue
            if not number(entry.record) or not entry.row:
                bad("has a record without its figure or its ledger row (rule 1)")
                continue
            t = entry.tolerance
            if line.exact:
                if t != 0:
                    bad("is counted exactly, its tolerance must be 0 (rule 3)")
            elif not number(t) or floor is None or not (floor <= t <= TOLERANCE_CEILING):
                bad(f"has a tolerance {t!r} outside [{floor}, {TOLERANCE_CEILING}] (rule 3)")
    raw_misses = data.get("misses", [])
    if not isinstance(raw_misses, list):
        defects.append("file defect: the named misses are not an array of tables ([[misses]])")
        raw_misses = []
    for spec in raw_misses:
        if not isinstance(spec, dict):
            defects.append(f"file defect: a named miss {spec!r} is not a table")
            continue
        case = spec.get("case")
        if case in budget.misses:
            defects.append(f"file defect: a second named miss of {case} (rule 1)")
            continue
        if case not in budget.cases:
            defects.append(f"file defect: a named miss keyed by {case}, a case the file does not list (rule 1)")
            continue
        causes = spec.get("causes")
        good_causes = (
            isinstance(causes, list) and causes and all(isinstance(c, str) and c.strip() for c in causes)
        )
        if not good_causes or not spec.get("row"):
            defects.append(f"file defect: the named miss of {case} without its cause or its ledger row (rule 1)")
        budget.misses[case] = Miss(case, causes if good_causes else [], str(spec.get("row", "")))
    for name, entry in budget.cases.items():
        line = lines[entry.line]
        if (
            entry.armed()
            and line.budget is not None
            and entry.record > line.budget
            and name not in budget.misses
        ):
            defects.append(
                f"file defect: case {name}'s record {show(entry.record, line.unit)} does not meet its line "
                f"{show(line.budget, line.unit)} and has no named miss of its own (rule 1)"
            )
    return budget


def asked_cases(budget: Budget) -> tuple[list, list]:
    """The armed cases the step asks the probe for (rule 4): the times and the bytes, each by
    preset, so that the probe keeps one preset's theme across its cases, then by line."""
    armed = [name for name, entry in budget.cases.items() if entry.armed()]
    armed.sort(key=lambda name: (case_preset(name), budget.cases[name].line, name))
    times = [name for name in armed if budget.lines[budget.cases[name].line].is_time()]
    sizes = [name for name in armed if not budget.lines[budget.cases[name].line].is_time()]
    return times, sizes


# ---------------------------------------------------------------- the probe's output


@dataclass
class Pass:
    figure: float  # nanoseconds
    load: float | None  # the mean runnable count over the pass; None: not readable
    sweep: int


@dataclass
class Sweep:
    start: float  # minutes after the build's end
    seconds: float | None  # as the probe printed it; None: not printed
    started: bool


@dataclass
class Results:
    passes: dict = field(default_factory=dict)  # case -> [Pass]
    bytes: dict = field(default_factory=dict)  # case -> int
    errors: dict = field(default_factory=dict)  # case -> why, from a cost-error line
    unreported: dict = field(default_factory=dict)  # case -> why: asked for and not reported (rule 4)
    sweeps: list = field(default_factory=list)  # [Sweep]
    listing_failures: list = field(default_factory=list)


def probe_lines(output: str) -> list[str]:
    """The probe's lines; the first may follow libtest's `test <name> ... ` on its line."""
    return [m.group(1) for m in (PROBE_LINE.search(line) for line in output.splitlines()) if m]


def fields(line: str) -> tuple[str, str, dict]:
    parts = line.split()
    kind = parts[0]
    name = parts[1] if len(parts) > 1 and "=" not in parts[1] else ""
    values = dict(p.split("=", 1) for p in parts[1:] if "=" in p)
    return kind, name, values


def read_listing(output: str) -> dict:
    listed = {}
    for line in probe_lines(output):
        kind, name, values = fields(line)
        if kind == "cost-case" and name:
            listed[name] = values.get("kind", "")
    return listed


def read_output(output: str, results: Results, sweep: int) -> float | None:
    """Adds one probe run's figures to `results`; returns its sweep's seconds, if printed."""
    seconds = None
    for line in probe_lines(output):
        kind, name, values = fields(line)
        try:
            if kind == "cost-pass":
                load = values.get("load")
                mean = None if load in (None, "absent") else float(load)
                results.passes.setdefault(name, []).append(Pass(float(values["median_ns"]), mean, sweep))
            elif kind == "cost-bytes":
                results.bytes[name] = int(values["bytes"])
            elif kind == "cost-error":
                parts = line.split(None, 2)
                results.errors[name] = parts[2] if len(parts) > 2 else "cost-error"
            elif kind == "cost-sweep":
                seconds = float(values["seconds"])
        except (KeyError, ValueError):
            results.errors.setdefault(name, f"unreadable probe line: {line}")
    return seconds


# ---------------------------------------------------------------- the sweeps (rule 2)


def counting(passes: list, limit: float) -> list:
    return [p for p in passes if p.load is not None and p.load <= limit]


def run_sweeps(
    times: list,
    sweep: Callable[[list, int, float], None],
    now: Callable[[], float],
    results: Results,
    run: str,
    limit: float,
    bound: float,
) -> None:
    """Three sweeps over `times`; in the release run further sweeps over the times with fewer than
    two passes that count, no sweep started later than `bound` minutes after the build's end (a
    sweep already running finishes and is judged). A case not reported, or with a pass whose
    runnable count could not be read, is asked for no more. `sweep(cases, index, start)` runs one
    sweep and adds its figures to `results`; `now()` is the minutes since the build's end."""
    index = 0
    while True:
        live = [
            c for c in times
            if c not in results.unreported and all(p.load is not None for p in results.passes.get(c, []))
        ]
        if index < 3:
            targets = live
        elif run == "release":
            targets = [c for c in live if len(counting(results.passes.get(c, []), limit)) < 2]
        else:
            break
        if not targets:
            break
        start = now()
        if run == "release" and start > bound:
            results.sweeps.append(Sweep(start, None, False))
            break
        sweep(targets, index, start)
        index += 1


# ---------------------------------------------------------------- one run of the step


Probe = Callable[[str, "list | None"], "tuple[int, str]"]


def unchanged(path: Path, digest: str) -> bool:
    return hashlib.sha256(path.read_bytes()).hexdigest() == digest


def run_step(budget_path: Path, run: str, probe: Probe, now: Callable[[], float]) -> tuple[int, list[str]]:
    """One run of the step over the file at `budget_path`, which it only reads: the probe's
    listings, the bytes, the sweeps, the verdicts. `probe(target, cases)` runs a binary of
    TESTS (cases None: its listing) and gives its exit code and output."""
    budget = parse_budget(budget_path.read_text())
    if not budget.step_ok():
        return 1, [f"FAIL  {defect}" for defect in budget.defects]
    results = Results()
    listed: dict = {}
    for target in TESTS:
        code, output = probe(target, None)
        cases = read_listing(output)
        if code != 0 or not cases:
            results.listing_failures.append(
                f"the probe's list of measurable cases from {target} failed: exit {code}, "
                f"{len(cases)} cost-case lines (rule 4's unlisted check needs it)"
            )
        listed.update(cases)
    times, sizes = asked_cases(budget)
    if sizes:
        _, output = probe("cost_bytes", sizes)
        read_output(output, results, 0)
        for name in sizes:
            if name in results.errors:
                results.unreported.setdefault(name, results.errors[name])
            elif name not in results.bytes:
                results.unreported.setdefault(name, "no line for it in the bytes run")

    def sweep(cases: list, index: int, start: float) -> None:
        had = {name: len(results.passes.get(name, [])) for name in cases}
        _, output = probe("timings", cases)
        seconds = read_output(output, results, index)
        for name in cases:
            if name in results.errors:
                results.unreported.setdefault(name, results.errors[name])
            elif len(results.passes.get(name, [])) == had[name]:
                results.unreported.setdefault(name, f"no line for it in sweep {index + 1}")
        results.sweeps.append(Sweep(start, seconds, True))

    if times:
        run_sweeps(times, sweep, now, results, run, budget.load_limit, budget.bound_minutes)
    return judge(budget, listed, results, run)


# ---------------------------------------------------------------- the verdicts (rules 1 to 4)


def show(value: float, unit: str) -> str:
    if unit == "bytes":
        return f"{value:.0f} bytes"
    return f"{value:.6g} {unit}"


def judge(budget: Budget, listed: dict, results: Results, run: str) -> tuple[int, list[str]]:
    """Every case's verdict; the exit code (0 pass, 1 fail) and the printout."""
    out: list[str] = []
    failed = False

    def fail(text: str) -> None:
        nonlocal failed
        failed = True
        out.append(f"FAIL  {text}")

    for defect in budget.defects:
        fail(defect)
    for failure in results.listing_failures:
        fail(failure)
    for name in sorted(set(listed) - set(budget.cases)):
        fail(f"file defect: unlisted case {name}: the probe can measure it, the file does not list it (rule 4)")
    absent = sorted(n for n, ps in results.passes.items() if any(p.load is None for p in ps))
    if absent:
        fail(
            "runnable count absent: /proc/loadavg's fourth field could not be read over a pass of "
            + ", ".join(absent)
            + " (rule 2)"
        )
    for sweep in results.sweeps:
        if not sweep.started:
            out.append(f"      sweep due at {sweep.start:.1f} min after the build's end: not started (past the bound)")
        elif sweep.seconds is None:
            out.append(f"      sweep started {sweep.start:.1f} min after the build's end, its duration not printed")
        else:
            out.append(f"      sweep started {sweep.start:.1f} min after the build's end, {sweep.seconds:.1f} s")
    for name, entry in sorted(budget.cases.items(), key=lambda item: (item[1].line, item[0])):
        line = budget.lines[entry.line]
        miss = budget.misses.get(name)
        if not entry.valid:
            continue  # a file defect, reported above
        if entry.not_binding is not None:
            out.append(f"      {name}: not binding: {entry.not_binding} ({entry.row})")
            continue
        if entry.record is None:
            text = f"{name}: no record yet (first measured by {entry.first_measured_by})"
            if run == "release":
                fail(text + ": the release needs every case armed or not binding (rule 4)")
            else:
                out.append(f"      {text}")
            continue
        if name in results.unreported:
            fail(f"{name}: not reported ({results.unreported[name]}) (rule 4)")
            continue
        scale = UNITS[line.unit]
        L, r = float(line.budget), float(entry.record)
        t = 0.0 if line.exact else float(entry.tolerance)
        head = f"{name}: line {show(L, line.unit)}, record {show(r, line.unit)}, tolerance {t:.0%}"
        if line.is_time():
            passes = results.passes.get(name, [])
            good = counting(passes, budget.load_limit)
            means = ", ".join("absent" if p.load is None else f"{p.load:.1f}" for p in passes)
            if len(good) < 2:
                text = f"{head}; {len(good)} of {len(passes)} passes count (means {means})"
                if run == "release":
                    fail(f"{text}: machine busy, run again (rule 2)")
                else:
                    out.append(f"      {text}: not compared")
                continue
            f = min(p.figure for p in good) / scale
            head += f"; {len(good)} of {len(passes)} passes count (means {means})"
        else:
            f = results.bytes[name] / scale
        head += f"; figure {show(f, line.unit)}"
        bound = r if (miss is not None and r > L) else L
        notes = []
        bad = None
        if f > bound * (1 + t):
            bad = f"past its bound {show(bound, line.unit)} by more than its tolerance (rule 1)"
        elif f > L and bound == L:
            notes.append("over its line, within its tolerance")
        if miss is not None and r > L and f < r * (1 - t):
            text = f"record stale: re-record {name}"
            if run == "release":
                bad = (bad + "; " if bad else "") + text + " (rule 1)"
            else:
                notes.append(text)
        if miss is not None and f <= L:
            notes.append(f"named miss may go (its causes: {'; '.join(miss.causes)})")
        if bad:
            fail(f"{head}: {bad}" + (f"; {'; '.join(notes)}" if notes else ""))
        else:
            out.append(f"ok    {head}" + (f": {'; '.join(notes)}" if notes else ""))
    return (1 if failed else 0), out


# ---------------------------------------------------------------- the step on the measuring machine


def set_priority() -> None:
    """Nice 19 and idle I/O for the step and every process it starts (rule 2)."""
    os.nice(19 - os.nice(0))
    try:
        subprocess.run(["ionice", "-c", "3", "-p", str(os.getpid())], check=True)
    except (OSError, subprocess.CalledProcessError) as error:
        sys.exit(f"check_cost_budget.py: cannot set idle I/O priority with ionice: {error}")


def build() -> dict:
    """Builds the probe's two test binaries; their paths by target name."""
    command = [
        "cargo", "test", "-p", "native-theme", "--release", "--all-features",
        "--test", "timings", "--test", "cost_bytes", "--no-run",
        "--message-format=json-render-diagnostics",
    ]
    started = time.monotonic()
    done = subprocess.run(command, cwd=ROOT, stdout=subprocess.PIPE, text=True)
    if done.returncode != 0:
        sys.exit("check_cost_budget.py: the probe's build failed")
    binaries = {}
    for text in done.stdout.splitlines():
        try:
            message = json.loads(text)
        except json.JSONDecodeError:
            continue
        if message.get("reason") == "compiler-artifact" and message.get("executable"):
            binaries[message["target"]["name"]] = message["executable"]
    print(f"      the probe's release build: {time.monotonic() - started:.1f} s")
    missing = set(TESTS) - set(binaries)
    if missing:
        sys.exit(f"check_cost_budget.py: the build gave no binary for {sorted(missing)}")
    return binaries


def binary_probe(binaries: dict) -> Probe:
    def probe(target: str, cases: list | None) -> tuple[int, str]:
        env = dict(os.environ)
        env.pop("NT_COST_CASES", None)
        env.pop("NT_COST_CHILD", None)
        if cases is not None:
            env["NT_COST_CASES"] = ",".join(cases)
        done = subprocess.run(
            [binaries[target], "--ignored", "--exact", TESTS[target], "--nocapture", "--test-threads=1"],
            cwd=ROOT, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
        )
        return done.returncode, done.stdout

    return probe


def step(run: str) -> int:
    budget = parse_budget(BUDGET.read_text())
    if not budget.step_ok():
        for defect in budget.defects:
            print(f"FAIL  {defect}")
        return 1
    set_priority()
    binaries = build()
    built = time.monotonic()
    code, out = run_step(BUDGET, run, binary_probe(binaries), lambda: (time.monotonic() - built) / 60.0)
    print("\n".join(out))
    print(f"check_cost_budget.py ({run} run): {'FAIL' if code else 'pass'}")
    return code


# ---------------------------------------------------------------- the self-test

SELF_TEST_STEP = "[step]\nload_limit = 10\nbound_minutes = 30\ntolerance_floor = 0.05\n"
SELF_TEST_LINES = (
    "[lines]\n"
    '2 = { what = "a peak", budget = 16, unit = "MiB" }\n'
    '4 = { what = "a change", budget = 100, unit = "ms" }\n'
    '9 = { what = "a file", budget = 262144, unit = "bytes", exact = true }\n'
    '12 = { what = "reported", unit = "ms" }\n'
)
# One case each binary lists in every scenario, as the real probe lists many: not binding, so it
# is never asked for and decides nothing.
SELF_TEST_FILLERS = (
    '"12.p.filler" = { not_binding = "a made-up listed case", row = "R0" }\n'
    '"2.p.filler" = { not_binding = "a made-up listed case", row = "R0" }\n'
)
MS = 1e6
MIB = 1024 * 1024
ARMED = 'record = 95, tolerance = 0.05, row = "R1"'


@dataclass
class Scenario:
    name: str
    cases: str  # the [cases] table's lines
    misses: str = ""
    listed: tuple = ()  # extra cases the probe lists as measurable
    unknown: tuple = ()  # armed cases the probe does not know: it prints cost-error for each
    times: dict = field(default_factory=dict)  # case -> [(figure ms, mean) per sweep]; None: cost-error
    starts: tuple = (0.0, 2.0, 4.0, 6.0, 8.0, 10.0)  # minutes after the build's end
    seconds: float | None = 60.0  # each sweep's duration as the probe prints it; None: not printed
    sizes: dict = field(default_factory=dict)  # case -> bytes; a case left out gets no line
    listing_fails: str | None = None  # a target whose listing exits non-zero with no line
    step: str = SELF_TEST_STEP
    expect: dict = field(default_factory=dict)  # run -> (exit, [texts the printout holds])
    sweeps: int | None = None  # the number of sweeps started, where checked


def quiet(figure: float, n: int = 3) -> list:
    return [(figure, 2.0)] * n


MISS = '[[misses]]\ncase = "4.p.c"\ncauses = ["task X"]\nrow = "R2"\n'

SCENARIOS = [
    Scenario("met line, figure over it by more than its tolerance, no named miss",
             f'"4.p.c" = {{ {ARMED} }}\n', times={"4.p.c": quiet(106)},
             expect={"unit": (1, ["past its bound"]), "release": (1, ["past its bound"])}),
    Scenario("met line, figure over it by less than its tolerance",
             f'"4.p.c" = {{ {ARMED} }}\n', times={"4.p.c": quiet(104)},
             expect={"unit": (0, ["over its line, within its tolerance"]),
                     "release": (0, ["over its line, within its tolerance"])}),
    Scenario("line 9's size one byte over its met line, printed in bytes",
             '"9.p.file" = { record = 262144, tolerance = 0, row = "R1" }\n',
             sizes={"9.p.file": 262145},
             expect={"unit": (1, ["figure 262145 bytes: past its bound 262144 bytes"]),
                     "release": (1, ["figure 262145 bytes: past its bound 262144 bytes"])}),
    Scenario("line 9 under a named miss, the size one byte under its record: stale",
             '"9.p.file" = { record = 316862, tolerance = 0, row = "R1" }\n',
             misses='[[misses]]\ncase = "9.p.file"\ncauses = ["task X"]\nrow = "R2"\n',
             sizes={"9.p.file": 316861},
             expect={"unit": (0, ["record stale"]), "release": (1, ["record stale"])}),
    Scenario("a record that does not meet its line, however close, without a named miss",
             '"4.p.c" = { record = 100.001, tolerance = 0.05, row = "R1" }\n', times={"4.p.c": quiet(99)},
             expect={"unit": (1, ["does not meet its line"]), "release": (1, ["does not meet its line"])}),
    Scenario("a second such case of the same line and preset under the first case's miss",
             '"4.p.c" = { record = 120, tolerance = 0.05, row = "R1" }\n'
             '"4.p.b" = { record = 120, tolerance = 0.05, row = "R1" }\n',
             misses=MISS, times={"4.p.c": quiet(120), "4.p.b": quiet(120)},
             expect={"unit": (1, ["4.p.b's record"]), "release": (1, ["4.p.b's record"])}),
    Scenario("a second named miss of one case",
             '"4.p.c" = { record = 120, tolerance = 0.05, row = "R1" }\n',
             misses=MISS + '[[misses]]\ncase = "4.p.c"\ncauses = ["task Y"]\nrow = "R3"\n',
             times={"4.p.c": quiet(120)},
             expect={"unit": (1, ["a second named miss"]), "release": (1, ["a second named miss"])}),
    Scenario("a named miss keyed by a case the file does not list",
             f'"4.p.c" = {{ {ARMED} }}\n',
             misses='[[misses]]\ncase = "4.p.gone"\ncauses = ["task X"]\nrow = "R2"\n',
             times={"4.p.c": quiet(95)},
             expect={"unit": (1, ["a case the file does not list"]),
                     "release": (1, ["a case the file does not list"])}),
    Scenario("under a named miss (record over its line), a figure over its record by more than its tolerance",
             '"4.p.c" = { record = 200, tolerance = 0.05, row = "R1" }\n', misses=MISS,
             times={"4.p.c": quiet(211)},
             expect={"unit": (1, ["past its bound"]), "release": (1, ["past its bound"])}),
    Scenario("under a named miss (record over its line), a figure over its record by less than its tolerance",
             '"4.p.c" = { record = 200, tolerance = 0.05, row = "R1" }\n', misses=MISS,
             times={"4.p.c": quiet(209)},
             expect={"unit": (0, ["ok    4.p.c"]), "release": (0, ["ok    4.p.c"])}),
    Scenario("under a named miss (record over its line), a stale record: f < record x (1 - t)",
             '"4.p.c" = { record = 200, tolerance = 0.05, row = "R1" }\n', misses=MISS,
             times={"4.p.c": quiet(150)},
             expect={"unit": (0, ["record stale: re-record 4.p.c"]),
                     "release": (1, ["record stale: re-record 4.p.c"])}),
    Scenario("a named miss whose record is under L / (1 + t) of a raised line L, its figure at the line",
             '"4.p.c" = { record = 90, tolerance = 0.05, row = "R1" }\n', misses=MISS,
             times={"4.p.c": quiet(100)},
             expect={"unit": (0, ["may go"]), "release": (0, ["may go"])}),
    Scenario("the same, its figure under record x (1 - t): not stale",
             '"4.p.c" = { record = 90, tolerance = 0.05, row = "R1" }\n', misses=MISS,
             times={"4.p.c": quiet(50)},
             expect={"unit": (0, ["may go"]), "release": (0, ["may go"])}),
    Scenario("the same, its figure over the line by more than its tolerance",
             '"4.p.c" = { record = 90, tolerance = 0.05, row = "R1" }\n', misses=MISS,
             times={"4.p.c": quiet(106)},
             expect={"unit": (1, ["past its bound"]), "release": (1, ["past its bound"])}),
    Scenario("a named miss without its cause",
             '"4.p.c" = { record = 120, tolerance = 0.05, row = "R1" }\n',
             misses='[[misses]]\ncase = "4.p.c"\ncauses = []\nrow = "R2"\n', times={"4.p.c": quiet(120)},
             expect={"unit": (1, ["without its cause or its ledger row"]),
                     "release": (1, ["without its cause or its ledger row"])}),
    Scenario("a named miss without its ledger row",
             '"4.p.c" = { record = 120, tolerance = 0.05, row = "R1" }\n',
             misses='[[misses]]\ncase = "4.p.c"\ncauses = ["task X"]\n', times={"4.p.c": quiet(120)},
             expect={"unit": (1, ["without its cause or its ledger row"]),
                     "release": (1, ["without its cause or its ledger row"])}),
    Scenario("a named miss whose causes are a string, not a list",
             '"4.p.c" = { record = 120, tolerance = 0.05, row = "R1" }\n',
             misses='[[misses]]\ncase = "4.p.c"\ncauses = "task X"\nrow = "R2"\n', times={"4.p.c": quiet(120)},
             expect={"unit": (1, ["without its cause or its ledger row"]),
                     "release": (1, ["without its cause or its ledger row"])}),
    Scenario("the named misses written as a table, not an array of tables",
             f'"4.p.c" = {{ {ARMED} }}\n', misses='[misses]\ncase = "4.p.c"\n', times={"4.p.c": quiet(95)},
             expect={"unit": (1, ["not an array of tables"]), "release": (1, ["not an array of tables"])}),
    Scenario("release: passes over 10 in the three sweeps, at most 10 in two later ones: compared",
             f'"4.p.c" = {{ {ARMED} }}\n',
             times={"4.p.c": [(90, 12.0), (90, 14.0), (90, 11.0), (96, 3.0), (97, 3.0)]},
             expect={"release": (0, ["2 of 5 passes count", "figure 96 ms"])}, sweeps=5),
    Scenario("release: one pass of three counts, swept again, compared once a second counts",
             f'"4.p.c" = {{ {ARMED} }}\n',
             times={"4.p.c": [(96, 3.0), (90, 12.0), (90, 13.0), (90, 12.0), (97, 2.0)]},
             expect={"release": (0, ["2 of 5 passes count", "figure 96 ms"])}, sweeps=5),
    Scenario("release: still short of two when the last sweep ends: machine busy",
             f'"4.p.c" = {{ {ARMED} }}\n', times={"4.p.c": [(96, 3.0)] + [(90, 12.0)] * 20},
             starts=tuple(float(m) for m in range(0, 60, 4)),
             expect={"release": (1, ["machine busy, run again"])}, sweeps=8),
    Scenario("release: a sweep due past the bound is not started",
             f'"4.p.c" = {{ {ARMED} }}\n', times={"4.p.c": [(96, 3.0), (90, 12.0), (90, 12.0), (97, 2.0)]},
             starts=(0.0, 2.0, 4.0, 31.0),
             expect={"release": (1, ["not started (past the bound)", "machine busy, run again"])}, sweeps=3),
    Scenario("release: a sweep started exactly at the bound runs and is judged",
             f'"4.p.c" = {{ {ARMED} }}\n', times={"4.p.c": [(96, 3.0), (90, 12.0), (90, 12.0), (97, 2.0)]},
             starts=(0.0, 2.0, 4.0, 30.0),
             expect={"release": (0, ["sweep started 30.0 min", "2 of 4 passes count"])}, sweeps=4),
    Scenario("release: a sweep started before the bound and ending after it (600 s from 29.5 min) is judged",
             f'"4.p.c" = {{ {ARMED} }}\n', times={"4.p.c": [(96, 3.0), (90, 12.0), (90, 12.0), (97, 2.0)]},
             starts=(0.0, 2.0, 4.0, 29.5, 39.5), seconds=600.0,
             expect={"release": (0, ["sweep started 29.5 min after the build's end, 600.0 s",
                                     "2 of 4 passes count"])}, sweeps=4),
    Scenario("release: machine busy, the bytes still compared and failing",
             f'"4.p.c" = {{ {ARMED} }}\n"2.p.read" = {{ record = 10, tolerance = 0.05, row = "R1" }}\n',
             times={"4.p.c": [(500, 12.0)] * 20}, sizes={"2.p.read": 17 * MIB},
             expect={"release": (1, ["machine busy, run again", "2.p.read: line 16 MiB", "past its bound"])}),
    Scenario("release: machine busy, the bytes compared and passing",
             f'"4.p.c" = {{ {ARMED} }}\n"2.p.read" = {{ record = 10, tolerance = 0.05, row = "R1" }}\n',
             times={"4.p.c": [(500, 12.0)] * 20}, sizes={"2.p.read": 10 * MIB},
             expect={"release": (1, ["machine busy, run again", "ok    2.p.read"])}),
    Scenario("unit: a time whose three passes all average over 10, the bytes passing: not compared, exit by the bytes",
             f'"4.p.c" = {{ {ARMED} }}\n"2.p.read" = {{ record = 10, tolerance = 0.05, row = "R1" }}\n',
             times={"4.p.c": [(500, 12.0)] * 3}, sizes={"2.p.read": 10 * MIB},
             expect={"unit": (0, ["0 of 3 passes count", "not compared", "ok    2.p.read"])}, sweeps=3),
    Scenario("unit: every time over 10, a bytes case past its bound: the bytes decide, non-zero",
             f'"4.p.c" = {{ {ARMED} }}\n"2.p.read" = {{ record = 10, tolerance = 0.05, row = "R1" }}\n',
             times={"4.p.c": [(500, 12.0)] * 3}, sizes={"2.p.read": 17 * MIB},
             expect={"unit": (1, ["not compared", "2.p.read: line 16 MiB"])}),
    Scenario("unit: one pass of three counts: not compared",
             f'"4.p.c" = {{ {ARMED} }}\n', times={"4.p.c": [(500, 3.0), (500, 12.0), (500, 12.0)]},
             expect={"unit": (0, ["1 of 3 passes count", "not compared"])}),
    Scenario("unit: two passes of three count: compared",
             f'"4.p.c" = {{ {ARMED} }}\n', times={"4.p.c": [(500, 3.0), (90, 12.0), (500, 2.0)]},
             expect={"unit": (1, ["2 of 3 passes count", "past its bound"])}),
    Scenario("a pass whose mean is exactly the limit, 10, counts",
             f'"4.p.c" = {{ {ARMED} }}\n', times={"4.p.c": [(96, 10.0)] * 3},
             expect={"unit": (0, ["3 of 3 passes count"]), "release": (0, ["3 of 3 passes count"])}),
    Scenario("an armed time the probe answers with cost-error: not reported, asked no more",
             f'"4.p.c" = {{ {ARMED} }}\n', times={"4.p.c": None},
             expect={"unit": (1, ["4.p.c: not reported (made-up failure)"]),
                     "release": (1, ["4.p.c: not reported (made-up failure)"])}, sweeps=1),
    Scenario("an armed time with one pass, then cost-error: not reported, not 'machine busy'",
             f'"4.p.c" = {{ {ARMED} }}\n', times={"4.p.c": [(96, 3.0), None]},
             expect={"unit": (1, ["4.p.c: not reported"]), "release": (1, ["4.p.c: not reported"])}, sweeps=2),
    Scenario("an armed time with no line at all in a sweep: not reported",
             f'"4.p.c" = {{ {ARMED} }}\n', times={"4.p.c": []},
             expect={"unit": (1, ["4.p.c: not reported (no line for it in sweep 1)"]),
                     "release": (1, ["4.p.c: not reported (no line for it in sweep 1)"])}, sweeps=1),
    Scenario("release: one armed name the probe does not know fails alone, the rest measured, ending at once",
             f'"4.p.c" = {{ {ARMED} }}\n"4.p.gone" = {{ {ARMED} }}\n',
             unknown=("4.p.gone",), times={"4.p.c": quiet(96)},
             starts=tuple(float(m) for m in range(0, 60, 2)),
             expect={"release": (1, ["4.p.gone: not reported (not a case this binary can measure)",
                                     "ok    4.p.c"])}, sweeps=3),
    Scenario("an armed bytes case not reported",
             '"2.p.read" = { record = 10, tolerance = 0.05, row = "R1" }\n',
             expect={"unit": (1, ["2.p.read: not reported (no line for it in the bytes run)"]),
                     "release": (1, ["2.p.read: not reported (no line for it in the bytes run)"])}),
    Scenario("a sweep whose probe printed no duration",
             f'"4.p.c" = {{ {ARMED} }}\n', times={"4.p.c": quiet(96)}, seconds=None,
             expect={"unit": (0, ["its duration not printed"])}),
    Scenario("a case the probe lists as measurable that the file does not list",
             f'"4.p.c" = {{ {ARMED} }}\n', listed=("4.p.new",), times={"4.p.c": quiet(95)},
             expect={"unit": (1, ["unlisted case 4.p.new"]), "release": (1, ["unlisted case 4.p.new"])}),
    Scenario("a listing that fails: failed by its binary's name, not passed vacuously",
             '"4.p.c" = { first_measured_by = "task X" }\n', listing_fails="timings",
             expect={"unit": (1, ["list of measurable cases from timings failed"]),
                     "release": (1, ["list of measurable cases from timings failed"])}),
    Scenario("a case with no record: no record yet, not asked for",
             '"4.p.c" = { first_measured_by = "task X" }\n',
             expect={"unit": (0, ["4.p.c: no record yet (first measured by task X)"]),
                     "release": (1, ["4.p.c: no record yet (first measured by task X)"])}, sweeps=0),
    Scenario("a not-binding case without its reason",
             '"12.p.c" = { not_binding = "", row = "R1" }\n',
             expect={"unit": (1, ["not binding without its reason"]),
                     "release": (1, ["not binding without its reason"])}),
    Scenario("a not-binding case without its ledger row",
             '"12.p.c" = { not_binding = "reported, no line" }\n',
             expect={"unit": (1, ["not binding without its reason or its ledger row"]),
                     "release": (1, ["not binding without its reason or its ledger row"])}),
    Scenario("a not-binding case with its reason and row",
             '"12.p.c" = { not_binding = "reported, no line", row = "R1" }\n',
             expect={"unit": (0, ["12.p.c: not binding"]), "release": (0, ["12.p.c: not binding"])}),
    Scenario("a pass whose runnable count could not be read: failed, asked no more",
             f'"4.p.c" = {{ {ARMED} }}\n', times={"4.p.c": [(90, None), (90, 2.0), (90, 2.0)]},
             expect={"unit": (1, ["runnable count absent"]), "release": (1, ["runnable count absent"])},
             sweeps=1),
    Scenario("a tolerance over 20 %",
             '"4.p.c" = { record = 95, tolerance = 0.25, row = "R1" }\n', times={"4.p.c": quiet(95)},
             expect={"unit": (1, ["outside [0.05, 0.2]"]), "release": (1, ["outside [0.05, 0.2]"])}),
    Scenario("a tolerance under the 5 % floor",
             '"4.p.c" = { record = 95, tolerance = 0.04, row = "R1" }\n', times={"4.p.c": quiet(95)},
             expect={"unit": (1, ["outside [0.05, 0.2]"]), "release": (1, ["outside [0.05, 0.2]"])}),
    Scenario("a tolerance that is no number",
             '"4.p.c" = { record = 95, tolerance = "5%", row = "R1" }\n', times={"4.p.c": quiet(95)},
             expect={"unit": (1, ["outside [0.05, 0.2]"]), "release": (1, ["outside [0.05, 0.2]"])}),
    Scenario("an exact line (9) with a tolerance that is not 0",
             '"9.p.file" = { record = 200000, tolerance = 0.05, row = "R1" }\n', sizes={"9.p.file": 200000},
             expect={"unit": (1, ["its tolerance must be 0"]), "release": (1, ["its tolerance must be 0"])}),
    Scenario("a [step] key missing: the defect, before any probe run",
             f'"4.p.c" = {{ {ARMED} }}\n', times={"4.p.c": quiet(95)},
             step="[step]\nload_limit = 10\ntolerance_floor = 0.05\n",
             expect={"unit": (1, ["[step] bound_minutes is missing"]),
                     "release": (1, ["[step] bound_minutes is missing"])}, sweeps=0),
]

# The probe's literal lines, as its println!s print them (native-theme/tests/timings.rs,
# cost_bytes.rs, cost_common/mod.rs), taken from a smoke run; the first after libtest's prefix.
PROBE_SAMPLE = (
    "test bundled_preset_costs ... cost-case 1.adwaita.read line=1 kind=time\n"
    "cost-case 9.adwaita.file line=9 kind=exact\n"
    "cost-pass 1.adwaita.read median_ns=7105421.0 samples=1 load=2.00 load_samples=2\n"
    "cost-pass 4.adwaita.synthetic.pick median_ns=645569643.0 samples=5 load=absent load_samples=80\n"
    "cost-bytes 3.adwaita.read bytes=2404209 rules=146 held_per_rule=16467.2\n"
    "cost-error 4.x.synthetic.pick not a case this binary can measure\n"
    "cost-sweep seconds=88.087 cases=30\n"
    "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 88.27s\n"
)


def parsers_read_the_probe() -> list[str]:
    """The parsers over PROBE_SAMPLE: what they must read; the failures."""
    listed = read_listing(PROBE_SAMPLE)
    results = Results()
    seconds = read_output(PROBE_SAMPLE, results, 0)
    checks = [
        (listed == {"1.adwaita.read": "time", "9.adwaita.file": "exact"}, f"listing {listed}"),
        (results.passes.get("1.adwaita.read") == [Pass(7105421.0, 2.0, 0)], "the pass of 1.adwaita.read"),
        (results.passes.get("4.adwaita.synthetic.pick") == [Pass(645569643.0, None, 0)], "an absent load"),
        (results.bytes == {"3.adwaita.read": 2404209}, f"bytes {results.bytes}"),
        (results.errors == {"4.x.synthetic.pick": "not a case this binary can measure"}, f"errors {results.errors}"),
        (seconds == 88.087, f"the sweep's seconds {seconds}"),
    ]
    return [what for ok, what in checks if not ok]


def fake_probe(scenario: Scenario, budget: Budget, starts: list) -> Probe:
    """A probe printing the literal lines the real one prints, from the scenario's made-up
    figures and per-pass means."""
    taken: dict = {}

    def probe(target: str, cases: list | None) -> tuple[int, str]:
        lines = []
        if cases is None:
            if scenario.listing_fails == target:
                return 101, "error: test failed\n"
            names = [n for n in budget.cases if n not in scenario.unknown] + list(scenario.listed)
            for name in names:
                line = budget.lines.get(case_line(name))
                is_time = line is not None and line.is_time()
                if is_time == (target == "timings"):
                    kind = "time" if is_time else ("exact" if line is not None and line.exact else "bytes")
                    lines.append(f"cost-case {name} line={case_line(name)} kind={kind}")
        elif target == "cost_bytes":
            for name in cases:
                if name in scenario.sizes:
                    lines.append(f"cost-bytes {name} bytes={scenario.sizes[name]}")
        else:
            for name in cases:
                if name in scenario.unknown:
                    lines.append(f"cost-error {name} not a case this binary can measure")
                    continue
                script = scenario.times.get(name, [])
                if script is None:
                    lines.append(f"cost-error {name} made-up failure")
                    continue
                at = taken.get(name, 0)
                if at >= len(script):
                    continue
                taken[name] = at + 1
                if script[at] is None:
                    lines.append(f"cost-error {name} made-up failure")
                    continue
                figure, mean = script[at]
                load = "absent" if mean is None else f"{mean:.2f}"
                lines.append(f"cost-pass {name} median_ns={figure * MS:.1f} samples=11 load={load} load_samples=3")
            if scenario.seconds is not None:
                lines.append(f"cost-sweep seconds={scenario.seconds:.3f} cases={len(cases)}")
        if lines:
            lines[0] = f"test {TESTS[target]} ... {lines[0]}"
        return 0, "\n".join(lines) + "\n"

    return probe


def simulate(scenario: Scenario, run: str, directory: Path) -> tuple[int, list[str], bool]:
    """The step's own run (run_step) over made-up figures, per-pass means and sweep start times,
    on a file in `directory`; with whether the file stayed byte for byte the same."""
    text = scenario.step + SELF_TEST_LINES + "[cases]\n" + SELF_TEST_FILLERS + scenario.cases + scenario.misses
    path = directory / f"budget-{run}.toml"
    path.write_text(text)
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    starts = iter(scenario.starts)
    budget = parse_budget(text)
    code, out = run_step(path, run, fake_probe(scenario, budget, []), lambda: next(starts, 1e9))
    return code, out, unchanged(path, digest)


def self_test() -> int:
    real_digest = hashlib.sha256(BUDGET.read_bytes()).hexdigest()
    failures = []
    covered = 0

    def outcome(ok: bool, text: str, detail: str = "") -> None:
        nonlocal covered
        print(f"{'ok  ' if ok else 'FAIL'}  {text}")
        if ok:
            covered += 1
        else:
            failures.append(f"{text}\n{detail}")

    missed = parsers_read_the_probe()
    outcome(not missed, "the parsers read the probe's literal lines", "; ".join(missed))
    with tempfile.TemporaryDirectory() as tmp:
        directory = Path(tmp)
        for scenario in SCENARIOS:
            for run, (want, texts) in scenario.expect.items():
                code, out, same = simulate(scenario, run, directory)
                text = "\n".join(out)
                missing = [t for t in texts if t not in text]
                started = sum(1 for line in out if "sweep started" in line)
                wrong_sweeps = scenario.sweeps is not None and started != scenario.sweeps
                verdict = "non-zero" if code else "zero"
                ok = code == want and not missing and not wrong_sweeps and same
                outcome(
                    ok,
                    f"{run:<7} {verdict:<8} {scenario.name}",
                    f"exit {code}, wanted {want}; missing {missing}; sweeps {started}, wanted "
                    f"{scenario.sweeps}; file unchanged {same}\n{text}",
                )
        # The real file: its check, and a unit run of the step over a copy, every case listed by a
        # made-up probe; the copy must stay byte for byte the same.
        real = parse_budget(BUDGET.read_text())
        outcome(not real.defects, "the file check of scripts/cost_budget.toml passes", "; ".join(real.defects))
        copy = directory / "cost_budget.toml"
        shutil.copyfile(BUDGET, copy)
        digest = hashlib.sha256(copy.read_bytes()).hexdigest()

        def listing_probe(target: str, cases: list | None) -> tuple[int, str]:
            names = [n for n, e in real.cases.items() if real.lines[e.line].is_time() == (target == "timings")]
            return 0, "".join(f"cost-case {n} line={case_line(n)} kind=x\n" for n in names)

        code, out = run_step(copy, "unit", listing_probe, lambda: 0.0)
        outcome(
            code == 0 and unchanged(copy, digest),
            "a unit run of the step over a copy of scripts/cost_budget.toml: zero, the copy byte for byte unchanged",
            "\n".join(out),
        )
        # The byte-for-byte check can fail: a changed copy is found changed.
        copy.write_bytes(copy.read_bytes() + b"\n")
        outcome(not unchanged(copy, digest), "the byte-for-byte check finds a changed copy (its positive control)")
    outcome(unchanged(BUDGET, real_digest), "scripts/cost_budget.toml byte for byte unchanged after every case")
    for failure in failures:
        print(f"\n{failure}", file=sys.stderr)
    print(f"check_cost_budget.py --self-test: {covered} outcomes covered, {len(failures)} failures")
    return 1 if failures else 0


def check_file() -> int:
    budget = parse_budget(BUDGET.read_text())
    for defect in budget.defects:
        print(f"FAIL  {defect}")
    records = sum(1 for e in budget.cases.values() if e.record is not None)
    print(
        f"check_cost_budget.py --check-file: {len(budget.cases)} cases, {records} with a record, "
        f"{len(budget.misses)} named misses, {len(budget.defects)} defects"
    )
    return 1 if budget.defects else 0


def main(argv: list[str]) -> int:
    modes = {"--self-test": self_test, "--check-file": check_file,
             "--unit": lambda: step("unit"), "--release": lambda: step("release")}
    if len(argv) != 2 or argv[1] not in modes:
        print(__doc__, file=sys.stderr)
        return 2
    return modes[argv[1]]()


if __name__ == "__main__":
    sys.exit(main(sys.argv))
