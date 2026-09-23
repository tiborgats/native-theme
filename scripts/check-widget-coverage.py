#!/usr/bin/env python3
"""Check that every widget the toolkits offer is shown in a showcase.

A showcase that omits a widget is a widget nobody has ever seen under a
native theme (spec §6a). A test cannot do this job: it cannot locate a
dependency's source. `cargo metadata --format-version 1` can, so this is a
script.

Run from the repository root:

    python3 scripts/check-widget-coverage.py

Exit 0 only when every discovered widget is either shown in the matching
showcase or listed in docs/showcase-exceptions.toml with a non-empty reason,
and no exception has gone stale.

Discovery
---------
Dependency source directories come only from `cargo metadata`
(`manifest_path` -> `src/`); never from a registry path or a version literal.
The metadata is read with

    cargo metadata --format-version 1 \\
        --manifest-path connectors/native-theme-iced/Cargo.toml \\
        --features iced_aw

because `iced_aw` is an optional dependency of the iced connector and is
absent from the graph otherwise. The invocation still returns the whole
workspace, so `gpui-component` comes from the same call. A dependency that is
missing from the metadata is a hard error, never a silently skipped toolkit.

gpui (`gpui-component`): every type with an `impl RenderOnce`/`IntoElement`
or a `#[derive(IntoElement)]`, outside `#[cfg(test)]` modules, that the crate
also declares as a `pub struct`. The `pub struct` gate is the crate's own
public-widget line: it discards private helpers (`ShimmerGlyphs`,
`MenuItemElement`) and the enums that are widget *arguments* rather than
widgets (`DescriptionText`, `IconName`).

iced (`iced_widget`): every source module under `src/` that declares a
crate-level `impl ... Widget<...>` or a `pub trait Catalog`. That rule
discards exactly `lib`, `helpers` and `action`, which declare no widget of
their own (spec §6a.3).

iced_aw: the widget modules the connector enables. The universe is every
`#[cfg(feature = "x")] pub mod x;` in `iced_aw`'s `src/widget.rs`; the
required set is the feature list `cargo metadata` reports for the connector's
`iced_aw` dependency. A module behind a feature the connector does not enable
is not compiled and so cannot be shown, but it stays in the universe so that
an exception naming it does not read as stale. An `iced_aw` module is looked
for in the showcase by the type names `widget.rs` re-exports from it (`Card`,
`SelectionList`, ...), not by the module name: `styles::aw::card` names the
module without rendering the widget.

Sub-parts of a compound widget (`TableRow`, `MessageHeader`, ...), layout
wrappers with no visual surface and widgets needing a GPU pipeline the
application supplies are not filtered out here — they are exception entries
with a source citation, so that every one of them is reviewable and an
upstream sub-part added tomorrow is reported rather than swallowed.

Matching
--------
"Shown" is a source-level test on the showcase, not a bare substring:

  * `//` line comments and `/* */` block comments are removed first, so a
    commented-out or merely mentioned widget does not count. The remover is
    string-aware, so a `//` inside a string literal does not start a comment,
    and char-literal-aware, so the `'"'` in the iced showcase does not open
    one.
  * the name must then match at identifier boundaries, i.e. as an identifier
    or a path segment: `Pagination`, `Pagination::new`, `widget::pane_grid`,
    `pane_grid(`. `grid` does not match `grid_rows`, and `Table` does not
    match `TableDelegate`.

String literals are removed from the **iced** showcase as well, so a section
labelled `text("Card")` no longer proves that a `Card` is rendered — the
widget has to be constructed in code. Measured on 2026-09-21 over the
finished iced showcase: all 28 `iced_widget` modules and all 8 `iced_aw`
widgets still match with the literals gone, so the rule costs nothing there.

The gpui half keeps its literals and asks a stronger question instead
(`shows_gpui`): not "is the name here" but "is the widget *constructed*
here". Three things count and nothing else —

  * an associated item, `W::new` or `W::horizontal`;
  * a call or a struct literal, `W(` or `W {`;
  * for a widget no call site names, the extension method that builds one:
    `ContextMenuExt::context_menu` returns a `ContextMenu`
    (menu/context_menu.rs:35) and `ScrollableElement::overflow_*_scrollbar`
    returns a `Scrollable` (scroll/scrollable.rs:46, :53, :60). These live in
    `GPUI_VIA`, one verified pattern each.

A name that is a segment of somebody else's path is somebody else's type.
`std::process::Command::new` used to satisfy gpui-component's `Command`
widget — a command palette the showcase did not render at all, and which
every gate passed green on for as long as the rule was a bare identifier
match. Which paths are the toolkit's is read from the showcase's own `use`
statements (`toolkit_roots`), not guessed: `form::Form::horizontal()` is
gpui-component's `Form` because `form` came from
`use gpui_component::{…, form::{self, Field}, …}`.

The tightened rule was once thought to make the string literals harmless —
"a string can no longer be followed by `::`, `(` or `{` in a way that proves
anything" — and that was wrong. The gpui showcase's Widget Info panels are
prose that names upstream types on purpose, and the resolvability audit
filled them with citations of exactly the matched shape: `(select.rs,
Caret::render)` inside a note read as a call, so `Caret` was reported shown
by a showcase that never builds one. Both sides now strip literals. A panel
arguing *about* a widget is not a demo of it.

Stripping them also removed the accident that had been covering three real
demos: `Dialog`, `AlertDialog` and `Sheet` are opened through `WindowExt`
methods that hand the widget to a builder closure, so an application never
names the type. Those are `GPUI_VIA` entries now, which is what they always
should have been.
"""

import argparse
import json
import os
import re
import subprocess
import sys
import tomllib

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.dirname(SCRIPT_DIR)

EXCEPTIONS = os.path.join(PROJECT_ROOT, "docs", "showcase-exceptions.toml")
ICED_MANIFEST = os.path.join(
    PROJECT_ROOT, "connectors", "native-theme-iced", "Cargo.toml"
)
SHOWCASE_GPUI = os.path.join(
    PROJECT_ROOT, "connectors", "native-theme-gpui", "examples", "showcase-gpui"
)
SHOWCASE_ICED = os.path.join(
    PROJECT_ROOT, "connectors", "native-theme-iced", "examples", "showcase-iced.rs"
)

RENDER_IMPL = re.compile(
    r"^\s*impl(?:\s*<[^>]*>)?\s+(?:RenderOnce|IntoElement)\s+for\s+([A-Za-z_]\w*)"
)
DERIVE = re.compile(r"#\[derive\(([^)]*)\)\]")
STRUCT = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?struct\s+([A-Za-z_]\w*)")
PUB_STRUCT = re.compile(r"^\s*pub\s+struct\s+([A-Za-z_]\w*)", re.M)
CFG_TEST_MOD = re.compile(r"#\[cfg\(test\)\]\s*(?:pub\s+)?mod\s+\w+\s*\{")
WIDGET_IMPL = re.compile(r"^impl(?:<[^>]*>)?[^\n]*\bWidget<", re.M)
CATALOG = re.compile(r"^pub trait Catalog", re.M)
AW_MODULE = re.compile(r"#\[cfg\(feature = \"(\w+)\"\)\]\s*pub mod (\w+);")
AW_REEXPORT = re.compile(
    r"#\[cfg\(feature = \"\w+\"\)\]\s*pub use (\w+)::(?:\{([^}]*)\}|(\w+));"
)


class Failure(Exception):
    """A condition that stops the check outright."""


def strip_comments(src):
    """Remove Rust comments, leaving string literals in place."""
    out = []
    i, n = 0, len(src)
    while i < n:
        c = src[i]
        if c == "/" and src.startswith("//", i):
            while i < n and src[i] != "\n":
                i += 1
        elif c == "/" and src.startswith("/*", i):
            depth, i = 1, i + 2
            while i < n and depth:
                if src.startswith("/*", i):
                    depth, i = depth + 1, i + 2
                elif src.startswith("*/", i):
                    depth, i = depth - 1, i + 2
                else:
                    i += 1
            out.append(" ")
        elif c == "r" and (m := re.match(r'r(#*)"', src[i:])):
            close = '"' + m.group(1)
            end = src.find(close, i + len(m.group(0)))
            out.append(src[i:] if end < 0 else src[i : end + len(close)])
            i = n if end < 0 else end + len(close)
        elif c == '"':
            out.append(c)
            i += 1
            while i < n:
                out.append(src[i])
                if src[i] == "\\":
                    if i + 1 < n:
                        out.append(src[i + 1])
                    i += 2
                elif src[i] == '"':
                    i += 1
                    break
                else:
                    i += 1
        elif c == "'" and (m := re.match(r"'(?:\\.|[^\\'])'", src[i:])):
            out.append(m.group(0))
            i += len(m.group(0))
        else:
            out.append(c)
            i += 1
    return "".join(out)


def strip_test_modules(src):
    """Remove `#[cfg(test)] mod ... { ... }` blocks by brace matching."""
    while (m := CFG_TEST_MOD.search(src)) is not None:
        depth, i = 1, m.end()
        while i < len(src) and depth:
            if src[i] == "{":
                depth += 1
            elif src[i] == "}":
                depth -= 1
            i += 1
        src = src[: m.start()] + src[i:]
    return src


def strip_string_literals(src):
    """Replace every string literal with a space, comments already gone.

    Char literals are kept whole, as `strip_comments` keeps them: `'"'` is a
    double quote that does not open a string, and swallowing to the next `"`
    would take the rest of the file's widgets with it. A lifetime (`'a`) does
    not match the pattern and falls through unchanged.
    """
    out = []
    i, n = 0, len(src)
    while i < n:
        c = src[i]
        if c == "'" and (m := re.match(r"'(?:\\.|[^\\'])'", src[i:])):
            out.append(m.group(0))
            i += len(m.group(0))
        elif c == "r" and (m := re.match(r'r(#*)"', src[i:])):
            close = '"' + m.group(1)
            end = src.find(close, i + len(m.group(0)))
            i = n if end < 0 else end + len(close)
            out.append(" ")
        elif c == '"':
            i += 1
            while i < n:
                if src[i] == "\\":
                    i += 2
                elif src[i] == '"':
                    i += 1
                    break
                else:
                    i += 1
            out.append(" ")
        else:
            out.append(c)
            i += 1
    return "".join(out)


def shows(haystack, name):
    return re.search(r"(?<!\w)" + re.escape(name) + r"(?!\w)", haystack) is not None


# gpui widgets the showcase renders without ever naming their type: an
# extension method builds one from the element it is called on. Each pattern is
# that call, so the proof stays as specific as a constructor would be.
GPUI_VIA = {
    # `ContextMenuExt::context_menu` -> `ContextMenu::new(id, self).menu(f)`
    # (menu/context_menu.rs:35).
    "ContextMenu": r"\.context_menu\s*\(",
    # `ScrollableElement::overflow_scrollbar` and its two axis forms each
    # return `Scrollable<Self>` (scroll/scrollable.rs:46, 53, 60).
    "Scrollable": r"\.overflow(?:_[xy])?_scrollbar\s*\(",
    # The three `WindowExt` openers hand the widget itself to a builder
    # closure -- `Fn(Dialog, …) -> Dialog` (window_ext.rs:30-32),
    # `Fn(AlertDialog, …) -> AlertDialog` (:51-53), `Fn(Sheet, …) -> Sheet`
    # (:14-16, and `open_sheet_at` at :19-20) -- so an application never names
    # the type to render one.
    "Dialog": r"\.open_dialog\s*\(",
    "AlertDialog": r"\.open_alert_dialog\s*\(",
    "Sheet": r"\.open_sheet(?:_at)?\s*\(",
    # `Accordion::item` builds the item itself and hands it to a builder
    # closure, `FnOnce(AccordionItem) -> AccordionItem` (accordion.rs:63-70),
    # so an application never names the type to render one: an item is
    # shown where an Accordion is built and given one in the same statement.
    "AccordionItem": r"\bAccordion::new\s*\([^;]*?\.item\s*\(",
}

# The `\w+::` run immediately before a name, i.e. the path it is a segment of.
PATH_TAIL = re.compile(r"((?:\w+::)+)$")


def toolkit_roots(src):
    """The identifiers a `use gpui_component::…` / `gpui_kit::…` brings in.

    A path-qualified name belongs to the toolkit when its *root* resolves
    there, and the showcase's `use` statements are what say so:
    `form::Form::horizontal()` is gpui-component's `Form` because `form` came
    from `use gpui_component::{…, form::{self, Field}, …}`, while
    `std::process::Command::new` is not gpui-component's `Command`.
    """
    roots = {"gpui_component", "gpui_kit"}
    for m in re.finditer(r"\buse\s+(?:gpui_component|gpui_kit)\s*::", src):
        end = src.find(";", m.end())
        if end < 0:
            continue
        roots.update(re.findall(r"\b[a-z_][a-z0-9_]*\b", src[m.end() : end]))
    roots.difference_update({"self", "as", "crate", "super"})
    return roots


def shows_gpui(haystack, name, roots=None):
    """Whether the gpui showcase *constructs* `name`, rather than mentioning it.

    A bare path segment is not enough. `std::process::Command::new` names a
    `Command`, and until this rule existed it satisfied gpui-component's
    `Command` widget -- which the showcase does not render at all.

    Three things count, and nothing else: an associated item (`W::new`), a
    call or a struct literal (`W(`, `W {`), and, for a widget no call site
    names, the extension method that builds it.
    """
    if roots is None:
        roots = {"gpui_component", "gpui_kit"}
    pattern = GPUI_VIA.get(name)
    if pattern and re.search(pattern, haystack):
        return True
    for m in re.finditer(r"(?<!\w)" + re.escape(name) + r"(?!\w)", haystack):
        before, after = haystack[: m.start()], haystack[m.end() :]
        tail = PATH_TAIL.search(before)
        # A segment of somebody else's path is somebody else's type.
        if tail and tail.group(1).split("::")[0] not in roots:
            continue
        if after.startswith("::") or after.startswith("(") or after.lstrip(" ").startswith("{"):
            return True
    return False


def showcase_files(path):
    """The source files of a showcase: `path` itself, or every `.rs` under it.

    The gpui showcase is a module tree, and a widget one of its pages builds
    is shown whichever file the page is in.
    """
    if os.path.isfile(path):
        return [path]
    if not os.path.isdir(path):
        raise Failure(f"showcase not found: {path}")
    files = sorted(
        os.path.join(root, name)
        for root, _, names in os.walk(path)
        for name in names
        if name.endswith(".rs")
    )
    if not files:
        raise Failure(f"no .rs files in the showcase directory {path}")
    return files


def read_showcase(path, strip_literals=False):
    texts = []
    for file in showcase_files(path):
        try:
            with open(file, encoding="utf-8") as f:
                source = f.read()
        except OSError as err:
            raise Failure(f"could not read the showcase {file}: {err}") from err
        # Each file on its own: a literal or a comment never spans two, and
        # stripping the concatenation would let an unclosed one in one file
        # swallow the next.
        text = strip_comments(source)
        texts.append(strip_string_literals(text) if strip_literals else text)
    return "\n".join(texts)


def cargo_metadata():
    cmd = [
        "cargo",
        "metadata",
        "--format-version",
        "1",
        "--manifest-path",
        ICED_MANIFEST,
        "--features",
        "iced_aw",
    ]
    try:
        done = subprocess.run(
            cmd, cwd=PROJECT_ROOT, capture_output=True, text=True, check=False
        )
    except OSError as err:
        raise Failure(f"could not run `cargo metadata`: {err}") from err
    if done.returncode != 0:
        raise Failure(
            "`cargo metadata` failed:\n"
            + " ".join(cmd)
            + "\n"
            + done.stderr.strip()
        )
    try:
        return json.loads(done.stdout)
    except json.JSONDecodeError as err:
        raise Failure(f"`cargo metadata` produced no usable JSON: {err}") from err


def package(meta, name):
    for pkg in meta.get("packages", []):
        if pkg.get("name") == name:
            return pkg
    raise Failure(
        f"`{name}` is not in the cargo metadata. Nothing is skipped silently: "
        "check that the dependency and the features this script asks for "
        "still exist."
    )


def source_dir(meta, name):
    src = os.path.join(os.path.dirname(package(meta, name)["manifest_path"]), "src")
    if not os.path.isdir(src):
        raise Failure(f"`{name}` has no src directory at {src}")
    return src


def rust_files(root):
    for base, _, files in os.walk(root):
        for name in sorted(files):
            if name.endswith(".rs"):
                yield os.path.join(base, name)


def gpui_widgets(src):
    """Public types that render themselves: `RenderOnce` / `IntoElement`."""
    found, public = set(), set()
    for path in sorted(rust_files(src)):
        with open(path, encoding="utf-8", errors="replace") as f:
            text = strip_test_modules(strip_comments(f.read()))
        public |= {m.group(1) for m in PUB_STRUCT.finditer(text)}
        lines = text.splitlines()
        for i, line in enumerate(lines):
            if m := RENDER_IMPL.match(line):
                found.add(m.group(1))
            d = DERIVE.search(line)
            if d and "IntoElement" in [x.strip() for x in d.group(1).split(",")]:
                for follow in lines[i + 1 : i + 8]:
                    if s := STRUCT.match(follow):
                        found.add(s.group(1))
                        break
    return sorted(found & public)


def iced_modules(src):
    """Source modules of `iced_widget` that declare a widget of their own."""
    names = set()
    for entry in sorted(os.listdir(src)):
        path = os.path.join(src, entry)
        if os.path.isdir(path):
            names.add(entry)
        elif entry.endswith(".rs"):
            names.add(entry[:-3])
    modules = []
    for name in sorted(names):
        paths = []
        if os.path.isfile(os.path.join(src, name + ".rs")):
            paths.append(os.path.join(src, name + ".rs"))
        if os.path.isdir(os.path.join(src, name)):
            paths += sorted(rust_files(os.path.join(src, name)))
        text = ""
        for path in paths:
            with open(path, encoding="utf-8", errors="replace") as f:
                text += strip_comments(f.read()) + "\n"
        if WIDGET_IMPL.search(text) or CATALOG.search(text):
            modules.append(name)
    return modules


def aw_modules(src):
    """Every feature-gated widget module of `iced_aw`, with its type names."""
    with open(os.path.join(src, "widget.rs"), encoding="utf-8") as f:
        text = strip_comments(f.read())
    reexports = {}
    for m in AW_REEXPORT.finditer(text):
        names = [n.strip() for n in (m.group(2) or m.group(3)).split(",") if n.strip()]
        reexports.setdefault(m.group(1), []).extend(names)
    modules = {
        m.group(2): reexports.get(m.group(2), [m.group(2)])
        for m in AW_MODULE.finditer(text)
        if m.group(1) == m.group(2)
    }
    if not modules:
        raise Failure("no feature-gated widget modules found in iced_aw/src/widget.rs")
    return modules


def aw_enabled(meta, universe):
    """The widget features the connector's own `iced_aw` dependency enables.

    The connector declares `iced_aw` twice -- once as the optional normal
    dependency the `iced_aw` feature turns on, once under `dev-dependencies`
    for the showcase. Taking whichever came first would let the two drift
    apart unnoticed, so the normal one is what counts and a dev entry that
    enables a different set is an error rather than a silent choice.
    """
    entries = {}
    for dep in package(meta, "native-theme-iced").get("dependencies", []):
        if dep.get("name") == "iced_aw":
            entries.setdefault(dep.get("kind") or "normal", set()).update(
                dep.get("features", [])
            )
    if "normal" not in entries:
        raise Failure("`native-theme-iced` declares no `iced_aw` dependency")
    for kind, features in sorted(entries.items()):
        if kind != "normal" and features != entries["normal"]:
            raise Failure(
                f"the connector's `{kind}` `iced_aw` dependency enables "
                f"{sorted(features)}, its normal one {sorted(entries['normal'])}; "
                "the showcase would then be built against a different widget "
                "set than the one this check measures"
            )
    enabled = {f: universe[f] for f in sorted(entries["normal"]) if f in universe}
    if not enabled:
        raise Failure("the connector's `iced_aw` dependency enables no widget feature")
    return enabled


def load_exceptions():
    if not os.path.isfile(EXCEPTIONS):
        raise Failure(f"exception file not found: {EXCEPTIONS}")
    try:
        with open(EXCEPTIONS, "rb") as f:
            data = tomllib.load(f)
    except OSError as err:
        raise Failure(f"could not read {EXCEPTIONS}: {err}") from err
    except tomllib.TOMLDecodeError as err:
        raise Failure(f"{EXCEPTIONS} is not valid TOML: {err}") from err
    table = {}
    for section in ("gpui", "iced_widget", "iced_aw"):
        entries = data.get(section, {})
        for name, reason in entries.items():
            if not isinstance(reason, str) or not reason.strip():
                raise Failure(
                    f"[{section}] {name}: an exception needs a non-empty reason"
                )
        table[section] = entries
    return table


def check(section, discovered, universe, showcase, exceptions, report, matches=shows):
    """Compare one toolkit and return its missing and stale names.

    `discovered` and `universe` map a widget's name to the identifiers that
    prove the showcase renders it. `matches` is the toolkit's own rule for
    what counts as proof.
    """
    excepted = exceptions.get(section, {})
    shown = [
        w for w, aliases in discovered.items() if any(matches(showcase, a) for a in aliases)
    ]
    missing = [w for w in discovered if w not in shown and w not in excepted]
    stale = []
    for name in sorted(excepted):
        if name not in universe:
            stale.append(f"{name}: excepted, but no longer offered upstream")
        elif any(matches(showcase, a) for a in universe[name]):
            stale.append(f"{name}: excepted, but the showcase shows it")
    report.append(
        f"{section:12s} discovered {len(discovered):3d}   shown {len(shown):3d}   "
        f"excepted {len(excepted):3d}   missing {len(missing):3d}"
    )
    return missing, stale


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--showcase-gpui",
        default=SHOWCASE_GPUI,
        help="gpui showcase to read instead of the connector's own: a file, "
        "or a directory whose .rs files are all read",
    )
    parser.add_argument(
        "--showcase-iced",
        default=SHOWCASE_ICED,
        help="iced showcase to read instead of the connector's own",
    )
    args = parser.parse_args()

    meta = cargo_metadata()
    # Both sides strip literals. The gpui showcase did not, and its Widget
    # Info panels are written in prose that names upstream types on purpose --
    # `(select.rs, Caret::render)` in a note read as a call, and `Caret` was
    # reported as shown by a showcase that never builds one. A panel arguing
    # *about* a widget is not a demo of it.
    gpui_show = read_showcase(args.showcase_gpui, strip_literals=True)
    iced_show = read_showcase(args.showcase_iced, strip_literals=True)
    exceptions = load_exceptions()

    gpui = {w: [w] for w in gpui_widgets(source_dir(meta, "gpui-component"))}
    iced = {m: [m] for m in iced_modules(source_dir(meta, "iced_widget"))}
    aw_all = aw_modules(source_dir(meta, "iced_aw"))
    aw = aw_enabled(meta, aw_all)

    gpui_roots = toolkit_roots(gpui_show)

    def shows_in_gpui(haystack, name):
        return shows_gpui(haystack, name, gpui_roots)

    report, missing, stale = [], [], []
    for section, discovered, universe, showcase, matches in (
        ("gpui", gpui, gpui, gpui_show, shows_in_gpui),
        ("iced_widget", iced, iced, iced_show, shows),
        ("iced_aw", aw, aw_all, iced_show, shows),
    ):
        part, rot = check(
            section, discovered, universe, showcase, exceptions, report, matches
        )
        missing += [f"{section}: {name}" for name in part]
        stale += [f"{section}: {line}" for line in rot]

    print("\n".join(report))
    print(
        f"iced_aw offers {len(aw_all)} feature-gated widget modules upstream; "
        f"the connector enables {len(aw)}."
    )

    if stale:
        print("\nStale exceptions:")
        print("\n".join("  " + line for line in stale))
    if missing:
        print("\nWidgets neither shown nor excepted:")
        print("\n".join("  " + line for line in missing))
    if missing or stale:
        return 1
    print("\nEvery widget is shown or excepted.")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Failure as error:
        print(f"check-widget-coverage: {error}", file=sys.stderr)
        sys.exit(2)
