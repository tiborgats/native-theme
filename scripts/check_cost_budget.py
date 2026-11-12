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
   record) is asked for by name; one not reported fails "not reported"; a
   case the probe lists as measurable that the file does not list fails
   "unlisted case". A case with no record is printed "no record yet" with the
   task that first measures it, not asked for; it passes a unit run and fails
   the release run, which needs every case armed or not binding.

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
import subprocess
import sys
import time
import tomllib
from dataclasses import dataclass, field
from pathlib import Path
from typing import Callable

ROOT = Path(__file__).resolve().parent.parent
BUDGET = ROOT / "scripts" / "cost_budget.toml"

# A line's unit, in the probe's base unit (nanoseconds for a time, bytes for bytes).
UNITS = {"ms": 1e6, "us": 1e3, "MiB": 1024.0 * 1024.0, "KiB": 1024.0}
TIME_UNITS = {"ms", "us"}
TOLERANCE_CEILING = 0.20

PROBE_LINE = re.compile(r"(cost-(?:case|pass|bytes|error|sweep)\b.*)$")


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


@dataclass
class Miss:
    case: str
    causes: list
    row: str


@dataclass
class Budget:
    load_limit: float
    bound_minutes: float
    tolerance_floor: float
    lines: dict
    cases: dict
    misses: dict  # case -> Miss, the first of each case
    defects: list = field(default_factory=list)


def case_line(name: str) -> int | None:
    head = name.split(".", 1)[0]
    return int(head) if head.isdigit() else None


def parse_budget(text: str) -> Budget:
    """The file `text` states, with its defects (rule 1's file part, rule 3's range, rule 4's not binding)."""
    data = tomllib.loads(text)
    defects: list[str] = []
    step = data.get("step", {})
    lines: dict[int, Line] = {}
    for key, spec in data.get("lines", {}).items():
        if not key.isdigit() or not isinstance(spec, dict):
            defects.append(f"file defect: line {key!r} is not a numbered line")
            continue
        unit = spec.get("unit")
        if unit not in UNITS:
            defects.append(f"file defect: line {key}'s unit {unit!r} is none of {sorted(UNITS)}")
            continue
        budget = spec.get("budget")
        if budget is not None and not isinstance(budget, (int, float)):
            defects.append(f"file defect: line {key}'s budget {budget!r} is no number")
            budget = None
        lines[int(key)] = Line(int(key), spec.get("what", ""), budget, unit, bool(spec.get("exact", False)))
    floor = step.get("tolerance_floor")
    budget = Budget(
        load_limit=step.get("load_limit"),
        bound_minutes=step.get("bound_minutes"),
        tolerance_floor=floor,
        lines=lines,
        cases={},
        misses={},
        defects=defects,
    )
    for key in ("load_limit", "bound_minutes", "tolerance_floor"):
        if not isinstance(step.get(key), (int, float)):
            defects.append(f"file defect: [step] {key} is missing or no number")
    for name, spec in data.get("cases", {}).items():
        number = case_line(name)
        if number not in lines or not isinstance(spec, dict):
            defects.append(f"file defect: case {name} names no line of the file")
            continue
        entry = Entry(
            name,
            number,
            record=spec.get("record"),
            tolerance=spec.get("tolerance"),
            row=spec.get("row"),
            first_measured_by=spec.get("first_measured_by"),
            not_binding=spec.get("not_binding"),
        )
        budget.cases[name] = entry
        states = [s for s in ("record", "first_measured_by", "not_binding") if spec.get(s) is not None]
        if len(states) != 1:
            defects.append(
                f"file defect: case {name} states {states or 'nothing'}; one of a record, "
                f"\"first measured by\" a named task or \"not binding\" (rule 4)"
            )
            continue
        line = lines[number]
        if entry.not_binding is not None or "not_binding" in spec:
            if not entry.not_binding or not entry.row:
                defects.append(f"file defect: case {name} not binding without its reason or its ledger row (rule 4)")
        elif entry.first_measured_by is not None:
            if not str(entry.first_measured_by).strip():
                defects.append(f"file defect: case {name}: \"first measured by\" names no task (rule 4)")
        else:
            if line.budget is None:
                defects.append(f"file defect: case {name} has a record on line {number}, which has no number")
                continue
            if not isinstance(entry.record, (int, float)) or not entry.row:
                defects.append(f"file defect: case {name}'s record without its figure or its ledger row (rule 1)")
                continue
            t = entry.tolerance
            if line.exact:
                if t != 0:
                    defects.append(f"file defect: case {name} is counted exactly, its tolerance must be 0 (rule 3)")
            elif not isinstance(t, (int, float)) or not (isinstance(floor, (int, float)) and floor <= t <= TOLERANCE_CEILING):
                defects.append(
                    f"file defect: case {name}'s tolerance {t!r} is outside "
                    f"[{floor}, {TOLERANCE_CEILING}] (rule 3)"
                )
    for spec in data.get("misses", []):
        case = spec.get("case")
        if case in budget.misses:
            defects.append(f"file defect: a second named miss of {case} (rule 1)")
            continue
        if case not in budget.cases:
            defects.append(f"file defect: a named miss keyed by {case}, a case the file does not list (rule 1)")
            continue
        causes = spec.get("causes") or []
        if not causes or not spec.get("row"):
            defects.append(f"file defect: the named miss of {case} without its cause or its ledger row (rule 1)")
        budget.misses[case] = Miss(case, causes, spec.get("row", ""))
    for name, entry in budget.cases.items():
        line = lines[entry.line]
        if (
            isinstance(entry.record, (int, float))
            and line.budget is not None
            and entry.record > line.budget
            and name not in budget.misses
        ):
            defects.append(
                f"file defect: case {name}'s record {entry.record:g} {line.unit} does not meet its line "
                f"{line.budget:g} {line.unit} and has no named miss of its own (rule 1)"
            )
    return budget


# ---------------------------------------------------------------- the probe's output


@dataclass
class Pass:
    figure: float  # nanoseconds
    load: float | None  # the mean runnable count over the pass; None: not readable
    sweep: int


@dataclass
class Results:
    passes: dict = field(default_factory=dict)  # case -> [Pass]
    bytes: dict = field(default_factory=dict)  # case -> int
    errors: dict = field(default_factory=dict)  # case -> why
    sweeps: list = field(default_factory=list)  # (start minutes, seconds or None)


def probe_lines(output: str) -> list[str]:
    return [m.group(1) for m in (PROBE_LINE.search(line) for line in output.splitlines()) if m]


def fields(line: str) -> tuple[str, str, dict]:
    parts = line.split()
    kind, name = parts[0], parts[1] if len(parts) > 1 else ""
    values = dict(p.split("=", 1) for p in parts[2:] if "=" in p)
    return kind, name, values


def read_listing(output: str) -> dict:
    listed = {}
    for line in probe_lines(output):
        kind, name, values = fields(line)
        if kind == "cost-case":
            listed[name] = values.get("kind", "")
    return listed


def read_output(output: str, results: Results, sweep: int) -> float | None:
    """Adds one probe run's figures to `results`; returns its sweep's seconds, if printed."""
    seconds = None
    for line in probe_lines(output):
        kind, name, values = fields(line)
        if kind == "cost-pass":
            load = values.get("load")
            mean = None if load in (None, "absent") else float(load)
            results.passes.setdefault(name, []).append(Pass(float(values["median_ns"]), mean, sweep))
        elif kind == "cost-bytes":
            results.bytes[name] = int(values["bytes"])
        elif kind == "cost-error":
            results.errors[name] = line.split(None, 2)[2] if len(line.split(None, 2)) > 2 else "error"
        elif kind == "cost-sweep":
            seconds = float(name.split("=", 1)[1]) if name.startswith("seconds=") else None
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
    sweep already running finishes and is judged). `sweep(cases, index, start)` runs one sweep and
    adds its passes to `results`; `now()` is the minutes since the build's end."""
    index = 0
    while True:
        if index < 3:
            targets = list(times)
        elif run == "release":
            targets = [c for c in times if len(counting(results.passes.get(c, []), limit)) < 2]
        else:
            break
        if not targets:
            break
        start = now()
        if run == "release" and start > bound:
            results.sweeps.append((start, None))
            break
        sweep(targets, index, start)
        index += 1


# ---------------------------------------------------------------- the verdicts (rules 1 to 4)


def show(value: float, unit: str) -> str:
    return f"{value:.4g} {unit}"


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
    for name in sorted(set(listed) - set(budget.cases)):
        fail(f"file defect: unlisted case {name}: the probe can measure it, the file does not list it (rule 4)")
    absent = sorted(n for n, ps in results.passes.items() if any(p.load is None for p in ps))
    if absent:
        fail(
            "runnable count absent: /proc/loadavg's fourth field could not be read over a pass of "
            + ", ".join(absent)
            + " (rule 2)"
        )
    for start, seconds in results.sweeps:
        if seconds is None:
            out.append(f"      sweep due at {start:.1f} min after the build's end: not started (past the bound)")
        else:
            out.append(f"      sweep started {start:.1f} min after the build's end, {seconds:.1f} s")
    for name, entry in sorted(budget.cases.items(), key=lambda item: (item[1].line, item[0])):
        line = budget.lines[entry.line]
        miss = budget.misses.get(name)
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
        if line.budget is None or not isinstance(entry.record, (int, float)):
            continue  # a file defect, reported above
        scale = UNITS[line.unit]
        L, r = float(line.budget), float(entry.record)
        t = 0.0 if line.exact else float(entry.tolerance or 0.0)
        head = f"{name}: line {show(L, line.unit)}, record {show(r, line.unit)}, tolerance {t:.0%}"
        if line.is_time():
            passes = results.passes.get(name, [])
            if not passes:
                why = results.errors.get(name, "no pass")
                fail(f"{name}: not reported ({why}) (rule 4)")
                continue
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
            if name not in results.bytes:
                why = results.errors.get(name, "no figure")
                fail(f"{name}: not reported ({why}) (rule 4)")
                continue
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
    missing = {"timings", "cost_bytes"} - set(binaries)
    if missing:
        sys.exit(f"check_cost_budget.py: the build gave no binary for {sorted(missing)}")
    return binaries


TESTS = {"timings": "bundled_preset_costs", "cost_bytes": "bundled_preset_bytes"}


def probe(binary: str, target: str, cases: list | None) -> str:
    env = dict(os.environ)
    env.pop("NT_COST_CASES", None)
    env.pop("NT_COST_CHILD", None)
    if cases is not None:
        env["NT_COST_CASES"] = ",".join(cases)
    done = subprocess.run(
        [binary, "--ignored", "--exact", TESTS[target], "--nocapture", "--test-threads=1"],
        cwd=ROOT, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
    )
    return done.stdout


def step(run: str) -> int:
    budget = parse_budget(BUDGET.read_text())
    set_priority()
    binaries = build()
    built = time.monotonic()
    listed: dict = {}
    for target, binary in binaries.items():
        listed.update(read_listing(probe(binary, target, None)))
    results = Results()
    armed = [n for n, e in budget.cases.items() if e.record is not None and e.not_binding is None]
    times = [n for n in armed if budget.lines[budget.cases[n].line].is_time()]
    sizes = [n for n in armed if n not in times]
    if sizes:
        read_output(probe(binaries["cost_bytes"], "cost_bytes", sizes), results, 0)

    def sweep(cases: list, index: int, start: float) -> None:
        seconds = read_output(probe(binaries["timings"], "timings", cases), results, index)
        results.sweeps.append((start, seconds))

    if times:
        run_sweeps(
            times, sweep, lambda: (time.monotonic() - built) / 60.0, results, run,
            budget.load_limit, budget.bound_minutes,
        )
    code, out = judge(budget, listed, results, run)
    print("\n".join(out))
    print(f"check_cost_budget.py ({run} run): {'FAIL' if code else 'pass'}")
    return code


# ---------------------------------------------------------------- the self-test

SELF_TEST_STEP = "[step]\nload_limit = 10\nbound_minutes = 30\ntolerance_floor = 0.05\n"
SELF_TEST_LINES = (
    "[lines]\n"
    '2 = { what = "a peak", budget = 16, unit = "MiB" }\n'
    '4 = { what = "a change", budget = 100, unit = "ms" }\n'
    '9 = { what = "a file", budget = 256, unit = "KiB", exact = true }\n'
    '12 = { what = "reported", unit = "ms" }\n'
)
MS = 1e6
KIB = 1024


@dataclass
class Scenario:
    name: str
    cases: str  # the [cases] table's lines
    misses: str = ""
    listed: tuple = ()  # extra cases the probe lists as measurable
    times: dict = field(default_factory=dict)  # case -> [(figure ms, mean) per sweep]; None: no pass
    starts: tuple = (0.0, 2.0, 4.0, 6.0, 8.0, 10.0)  # minutes after the build's end
    sizes: dict = field(default_factory=dict)  # case -> bytes
    expect: dict = field(default_factory=dict)  # run -> (exit, [texts the printout holds])


def quiet(figure: float, n: int = 3) -> list:
    return [(figure, 2.0)] * n


SCENARIOS = [
    Scenario(
        "met line, figure over it by more than its tolerance, no named miss",
        '"4.p.c" = { record = 95, tolerance = 0.05, row = "R1" }\n',
        times={"4.p.c": quiet(106)},
        expect={"unit": (1, ["past its bound"]), "release": (1, ["past its bound"])},
    ),
    Scenario(
        "met line, figure over it by less than its tolerance",
        '"4.p.c" = { record = 95, tolerance = 0.05, row = "R1" }\n',
        times={"4.p.c": quiet(104)},
        expect={"unit": (0, ["over its line, within its tolerance"]),
                "release": (0, ["over its line, within its tolerance"])},
    ),
    Scenario(
        "line 9's size one byte over its met line",
        '"9.p.file" = { record = 250, tolerance = 0, row = "R1" }\n',
        sizes={"9.p.file": 256 * KIB + 1},
        expect={"unit": (1, ["past its bound"]), "release": (1, ["past its bound"])},
    ),
    Scenario(
        "a record that does not meet its line, however close, without a named miss",
        '"4.p.c" = { record = 100.001, tolerance = 0.05, row = "R1" }\n',
        times={"4.p.c": quiet(99)},
        expect={"unit": (1, ["does not meet its line"]), "release": (1, ["does not meet its line"])},
    ),
    Scenario(
        "a second such case of the same line and preset under the first case's miss",
        '"4.p.a" = { record = 120, tolerance = 0.05, row = "R1" }\n'
        '"4.p.b" = { record = 120, tolerance = 0.05, row = "R1" }\n',
        misses='[[misses]]\ncase = "4.p.a"\ncauses = ["task X"]\nrow = "R2"\n',
        times={"4.p.a": quiet(120), "4.p.b": quiet(120)},
        expect={"unit": (1, ["4.p.b's record"]), "release": (1, ["4.p.b's record"])},
    ),
    Scenario(
        "a second named miss of one case",
        '"4.p.c" = { record = 120, tolerance = 0.05, row = "R1" }\n',
        misses='[[misses]]\ncase = "4.p.c"\ncauses = ["task X"]\nrow = "R2"\n'
        '[[misses]]\ncase = "4.p.c"\ncauses = ["task Y"]\nrow = "R3"\n',
        times={"4.p.c": quiet(120)},
        expect={"unit": (1, ["a second named miss"]), "release": (1, ["a second named miss"])},
    ),
    Scenario(
        "a named miss keyed by a case the file does not list",
        '"4.p.c" = { record = 95, tolerance = 0.05, row = "R1" }\n',
        misses='[[misses]]\ncase = "4.p.gone"\ncauses = ["task X"]\nrow = "R2"\n',
        times={"4.p.c": quiet(95)},
        expect={"unit": (1, ["a case the file does not list"]),
                "release": (1, ["a case the file does not list"])},
    ),
    Scenario(
        "under a named miss (record over its line), a figure over its record by more than its tolerance",
        '"4.p.c" = { record = 200, tolerance = 0.05, row = "R1" }\n',
        misses='[[misses]]\ncase = "4.p.c"\ncauses = ["task X"]\nrow = "R2"\n',
        times={"4.p.c": quiet(211)},
        expect={"unit": (1, ["past its bound"]), "release": (1, ["past its bound"])},
    ),
    Scenario(
        "under a named miss (record over its line), a figure over its record by less than its tolerance",
        '"4.p.c" = { record = 200, tolerance = 0.05, row = "R1" }\n',
        misses='[[misses]]\ncase = "4.p.c"\ncauses = ["task X"]\nrow = "R2"\n',
        times={"4.p.c": quiet(209)},
        expect={"unit": (0, ["ok    4.p.c"]), "release": (0, ["ok    4.p.c"])},
    ),
    Scenario(
        "under a named miss (record over its line), a stale record: f < record x (1 - t)",
        '"4.p.c" = { record = 200, tolerance = 0.05, row = "R1" }\n',
        misses='[[misses]]\ncase = "4.p.c"\ncauses = ["task X"]\nrow = "R2"\n',
        times={"4.p.c": quiet(150)},
        expect={"unit": (0, ["record stale: re-record 4.p.c"]),
                "release": (1, ["record stale: re-record 4.p.c"])},
    ),
    Scenario(
        "a named miss whose record is under L / (1 + t) of a raised line L, its figure at the line",
        '"4.p.c" = { record = 90, tolerance = 0.05, row = "R1" }\n',
        misses='[[misses]]\ncase = "4.p.c"\ncauses = ["task X"]\nrow = "R2"\n',
        times={"4.p.c": quiet(100)},
        expect={"unit": (0, ["may go"]), "release": (0, ["may go"])},
    ),
    Scenario(
        "the same, its figure under record x (1 - t): not stale",
        '"4.p.c" = { record = 90, tolerance = 0.05, row = "R1" }\n',
        misses='[[misses]]\ncase = "4.p.c"\ncauses = ["task X"]\nrow = "R2"\n',
        times={"4.p.c": quiet(50)},
        expect={"unit": (0, ["may go"]), "release": (0, ["may go"])},
    ),
    Scenario(
        "the same, its figure over the line by more than its tolerance",
        '"4.p.c" = { record = 90, tolerance = 0.05, row = "R1" }\n',
        misses='[[misses]]\ncase = "4.p.c"\ncauses = ["task X"]\nrow = "R2"\n',
        times={"4.p.c": quiet(106)},
        expect={"unit": (1, ["past its bound"]), "release": (1, ["past its bound"])},
    ),
    Scenario(
        "a named miss without its cause",
        '"4.p.c" = { record = 120, tolerance = 0.05, row = "R1" }\n',
        misses='[[misses]]\ncase = "4.p.c"\ncauses = []\nrow = "R2"\n',
        times={"4.p.c": quiet(120)},
        expect={"unit": (1, ["without its cause or its ledger row"]),
                "release": (1, ["without its cause or its ledger row"])},
    ),
    Scenario(
        "a named miss without its ledger row",
        '"4.p.c" = { record = 120, tolerance = 0.05, row = "R1" }\n',
        misses='[[misses]]\ncase = "4.p.c"\ncauses = ["task X"]\n',
        times={"4.p.c": quiet(120)},
        expect={"unit": (1, ["without its cause or its ledger row"]),
                "release": (1, ["without its cause or its ledger row"])},
    ),
    Scenario(
        "release: passes over 10 in the three sweeps, at most 10 in two later ones: compared",
        '"4.p.c" = { record = 95, tolerance = 0.05, row = "R1" }\n',
        times={"4.p.c": [(90, 12.0), (90, 14.0), (90, 11.0), (96, 3.0), (97, 3.0)]},
        expect={"release": (0, ["2 of 5 passes count", "figure 96 ms"])},
    ),
    Scenario(
        "release: one pass of three counts, swept again, compared once a second counts",
        '"4.p.c" = { record = 95, tolerance = 0.05, row = "R1" }\n',
        times={"4.p.c": [(96, 3.0), (90, 12.0), (90, 13.0), (90, 12.0), (97, 2.0)]},
        expect={"release": (0, ["2 of 5 passes count", "figure 96 ms"])},
    ),
    Scenario(
        "release: still short of two when the last sweep ends: machine busy",
        '"4.p.c" = { record = 95, tolerance = 0.05, row = "R1" }\n',
        times={"4.p.c": [(96, 3.0)] + [(90, 12.0)] * 20},
        starts=tuple(float(m) for m in range(0, 60, 4)),
        expect={"release": (1, ["machine busy, run again"])},
    ),
    Scenario(
        "release: a sweep due past the bound is not started",
        '"4.p.c" = { record = 95, tolerance = 0.05, row = "R1" }\n',
        times={"4.p.c": [(96, 3.0), (90, 12.0), (90, 12.0), (97, 2.0)]},
        starts=(0.0, 2.0, 4.0, 31.0),
        expect={"release": (1, ["not started (past the bound)", "machine busy, run again"])},
    ),
    Scenario(
        "release: a sweep started before the bound and ending after it is judged",
        '"4.p.c" = { record = 95, tolerance = 0.05, row = "R1" }\n',
        times={"4.p.c": [(96, 3.0), (90, 12.0), (90, 12.0), (97, 2.0)]},
        starts=(0.0, 2.0, 4.0, 29.5, 36.0),
        expect={"release": (0, ["started 29.5 min", "2 of 4 passes count"])},
    ),
    Scenario(
        "unit: a time whose three passes all average over 10, the bytes passing: not compared, exit by the bytes",
        '"4.p.c" = { record = 95, tolerance = 0.05, row = "R1" }\n'
        '"2.p.read" = { record = 10, tolerance = 0.05, row = "R1" }\n',
        times={"4.p.c": [(500, 12.0)] * 3},
        sizes={"2.p.read": 10 * 1024 * 1024},
        expect={"unit": (0, ["0 of 3 passes count", "not compared", "ok    2.p.read"])},
    ),
    Scenario(
        "unit: every time over 10, a bytes case past its bound: the bytes decide, non-zero",
        '"4.p.c" = { record = 95, tolerance = 0.05, row = "R1" }\n'
        '"2.p.read" = { record = 10, tolerance = 0.05, row = "R1" }\n',
        times={"4.p.c": [(500, 12.0)] * 3},
        sizes={"2.p.read": 17 * 1024 * 1024},
        expect={"unit": (1, ["not compared", "2.p.read: line 16 MiB"])},
    ),
    Scenario(
        "unit: one pass of three counts: not compared",
        '"4.p.c" = { record = 95, tolerance = 0.05, row = "R1" }\n',
        times={"4.p.c": [(500, 3.0), (500, 12.0), (500, 12.0)]},
        expect={"unit": (0, ["1 of 3 passes count", "not compared"])},
    ),
    Scenario(
        "unit: two passes of three count: compared",
        '"4.p.c" = { record = 95, tolerance = 0.05, row = "R1" }\n',
        times={"4.p.c": [(500, 3.0), (90, 12.0), (500, 2.0)]},
        expect={"unit": (1, ["2 of 3 passes count", "past its bound"])},
    ),
    Scenario(
        "an armed case asked for and not reported",
        '"4.p.c" = { record = 95, tolerance = 0.05, row = "R1" }\n',
        times={"4.p.c": None},
        expect={"unit": (1, ["not reported"]), "release": (1, ["not reported"])},
    ),
    Scenario(
        "a case the probe lists as measurable that the file does not list",
        '"4.p.c" = { record = 95, tolerance = 0.05, row = "R1" }\n',
        listed=("4.p.new",),
        times={"4.p.c": quiet(95)},
        expect={"unit": (1, ["unlisted case 4.p.new"]), "release": (1, ["unlisted case 4.p.new"])},
    ),
    Scenario(
        "a case with no record: no record yet, not asked for",
        '"4.p.c" = { first_measured_by = "task X" }\n',
        expect={"unit": (0, ["4.p.c: no record yet (first measured by task X)"]),
                "release": (1, ["4.p.c: no record yet (first measured by task X)"])},
    ),
    Scenario(
        "a not-binding case without its reason",
        '"12.p.c" = { not_binding = "", row = "R1" }\n',
        expect={"unit": (1, ["not binding without its reason"]),
                "release": (1, ["not binding without its reason"])},
    ),
    Scenario(
        "a not-binding case without its ledger row",
        '"12.p.c" = { not_binding = "reported, no line" }\n',
        expect={"unit": (1, ["not binding without its reason or its ledger row"]),
                "release": (1, ["not binding without its reason or its ledger row"])},
    ),
    Scenario(
        "a not-binding case with its reason and row",
        '"12.p.c" = { not_binding = "reported, no line", row = "R1" }\n',
        expect={"unit": (0, ["12.p.c: not binding"]), "release": (0, ["12.p.c: not binding"])},
    ),
    Scenario(
        "a pass whose runnable count could not be read",
        '"4.p.c" = { record = 95, tolerance = 0.05, row = "R1" }\n',
        times={"4.p.c": [(90, None), (90, 2.0), (90, 2.0)]},
        expect={"unit": (1, ["runnable count absent"]), "release": (1, ["runnable count absent"])},
    ),
    Scenario(
        "a tolerance outside [5 %, 20 %]",
        '"4.p.c" = { record = 95, tolerance = 0.25, row = "R1" }\n',
        times={"4.p.c": quiet(95)},
        expect={"unit": (1, ["outside [0.05, 0.2]"]), "release": (1, ["outside [0.05, 0.2]"])},
    ),
]


def simulate(scenario: Scenario, run: str) -> tuple[int, list[str]]:
    """The step's run over made-up figures, per-pass means and sweep start times."""
    budget = parse_budget(
        SELF_TEST_STEP + SELF_TEST_LINES + "[cases]\n" + scenario.cases + scenario.misses
    )
    listed = {name: "" for name in budget.cases}
    listed.update({name: "" for name in scenario.listed})
    results = Results()
    armed = [n for n, e in budget.cases.items() if e.record is not None]
    for name in armed:
        if name in scenario.sizes:
            results.bytes[name] = scenario.sizes[name]
    times = [n for n in armed if budget.lines[budget.cases[n].line].is_time()]
    starts = iter(scenario.starts)
    taken = {name: 0 for name in times}

    def sweep(cases: list, index: int, start: float) -> None:
        for name in cases:
            script = scenario.times.get(name)
            if script is None:
                results.errors[name] = "made-up: not reported"
                continue
            if taken[name] < len(script):
                figure, mean = script[taken[name]]
                taken[name] += 1
                results.passes.setdefault(name, []).append(Pass(figure * MS, mean, index))
        results.sweeps.append((start, 60.0))

    def now() -> float:
        return next(starts, 1e9)

    if times:
        run_sweeps(times, sweep, now, results, run, budget.load_limit, budget.bound_minutes)
    return judge(budget, listed, results, run)


def self_test() -> int:
    before = hashlib.sha256(BUDGET.read_bytes()).hexdigest()
    failures = []
    covered = 0
    for scenario in SCENARIOS:
        for run, (want, texts) in scenario.expect.items():
            code, out = simulate(scenario, run)
            text = "\n".join(out)
            missing = [t for t in texts if t not in text]
            verdict = "non-zero" if code else "zero"
            if code != want or missing:
                failures.append(f"{scenario.name} ({run}): exit {code}, wanted {want}; missing {missing}\n{text}")
                print(f"FAIL  {run:<7} {verdict:<8} {scenario.name}")
            else:
                covered += 1
                print(f"ok    {run:<7} {verdict:<8} {scenario.name}")
            if hashlib.sha256(BUDGET.read_bytes()).hexdigest() != before:
                failures.append(f"{scenario.name} ({run}): scripts/cost_budget.toml changed")
    real = parse_budget(BUDGET.read_text())
    if real.defects:
        failures.append("scripts/cost_budget.toml: " + "; ".join(real.defects))
        print("FAIL  the file check of scripts/cost_budget.toml")
    else:
        print("ok    the file check of scripts/cost_budget.toml passes")
    if hashlib.sha256(BUDGET.read_bytes()).hexdigest() != before:
        failures.append("scripts/cost_budget.toml changed during the self-test")
    else:
        print("ok    scripts/cost_budget.toml byte for byte unchanged after every case")
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
