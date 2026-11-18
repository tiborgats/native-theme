#!/usr/bin/env python3
"""Check that every widget the toolkits offer is shown in a showcase.

A showcase that omits a widget is a widget nobody has ever seen under a
native theme (spec §6a). A test cannot do this job: it cannot locate a
dependency's source. `cargo metadata --format-version 1` can, so this is a
script.

Run from the repository root:

    python3 scripts/check_widget_coverage.py
    python3 scripts/check_widget_coverage.py --self-test

`--self-test` runs the guards on synthetic crate roots, with no cargo: an
empty discovery and a moved upstream file each fail (exit 2 in a real run),
and one widget found and shown passes. It also runs the gpui-base re-export
rule on synthetic gpui-base and gpui-component roots, one case per part of
the rule, and the free-function `GPUI_VIA` patterns.

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
workspace, so `gpui-component`, `egui` and `egui_extras` come from the same
call. A dependency that is
missing from the metadata is a hard error, never a silently skipped toolkit,
and so is a toolkit in which the rules below discover no widget: an upstream
layout they no longer match would otherwise pass as `missing 0`.

gpui (`gpui-component`): every type with an `impl RenderOnce`/`IntoElement`,
a `#[derive(IntoElement)]` or a `#[derive(IntoPlot)]`, outside `#[cfg(test)]`
modules, that the crate also declares as a `pub struct`. `IntoPlot` is how
the charts become elements: the derive writes their `impl IntoElement`
(gpui-component-macros 0.9.1, derive_into_plot.rs:20). The `pub struct`
gate is the crate's own public-widget line: it discards private helpers
(`ShimmerGlyphs`, `MenuItemElement`) and the enums that are widget
*arguments* rather than widgets (`DescriptionText`, `IconName`).

gpui also counts the gpui-base widgets gpui-component makes public
(`gpui_base_reexports`). A statement counts when it is `pub use
gpui_base::<tree>;` with the visibility exactly `pub`, outside comments and
`#[cfg(test)]` modules, for the names its module makes public, as Rust's
visibility reaches them, transitively (`component_exports`): all of them in
a module declared `pub mod` from the root down, or re-exported whole;
in a module that is not, the names a public module re-exports from it by
name or by glob (`pub use virtual_list::{VirtualList, …}` for the private
`mod virtual_list`), along any chain of private modules. Its tree is
expanded (nested braces, `X as Y`, `*`, `self`) and each path followed in
gpui-base as Rust resolves it: `crate::`, `self::` and `super::`; a module
declared by `mod` or `pub mod`, its file `<name>.rs` or `<name>/mod.rs` in
the declaring module's folder (the own folder of a `lib.rs`, a `mod.rs` or
a file a `#[path]` names, which rustc reads as a `mod.rs`; `x/` beside any
other `x.rs`), or the file a `#[path]` names; an inline `mod name { … }`;
and a module another `pub use` brings in. Only `pub use` brings a name in.
A path whose first segment is one of gpui-base's dependencies in the
metadata, or `std`, `core` or `alloc`, is an external crate's
(`ropey::Rope`) and yields nothing; any other module segment that names
nothing is a `Failure` (exit 2: the re-export rule no longer matches
gpui-base's layout), the first segment of a gpui-component path included,
while a last segment the module does not offer yields nothing. At the last
module the name is a `pub struct` its file declares, or an item its own
`pub use` items bring in by name, rename or glob, followed the same way; a
glob takes all of them, and a `pub mod` named whole is followed as a glob.
An item counts when the file declaring its `pub struct` also renders it, as
above, so a same-named trait, enum or non-rendering struct elsewhere does
not, and its name is gpui-component's public one (the `as` name where
renamed).

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

egui: the public types drawn by value (`impl Widget for T`, the by-value impl
only, so the `&mut` and `&` impls of style.rs and introspection.rs do not
count) or shown by a container method (`show`, `show_*`, `ui`, `body`) of an
inherent impl under `src/containers/` or in `src/grid.rs`. `body` admits
`HeaderResponse`, whose `body` shows a `CollapsingState`'s content.

egui_ui: the `pub fn`s of the `impl Ui` blocks of egui's `src/ui.rs` whose
doc heading names widget-adding methods: `# Adding widgets`, `# Colors`,
`# Adding Containers / Sub-uis:` and `# Menus`.

egui_extras: the public types drawn by value or shown by a method (`show`,
`show_*`, `ui`, `horizontal`, `vertical`, `header`, `body`), plus every
module-level `pub fn` returning `egui::Response`, e.g.
`syntax_highlighting::code_view_ui`.

The egui showcase is the module tree
`connectors/native-theme-egui/examples/showcase-egui/`, its `tests.rs` left
out: the self-tests call `By::label` and the like, which are not widgets.

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
    (menu/context_menu.rs:39) and `ScrollableElement::overflow_*_scrollbar`
    returns a `Scrollable` (scroll/scrollable.rs:46, :53, :60). These live in
    `GPUI_VIA`, one verified pattern each. A free function that returns a
    widget counts through `GPUI_VIA` too: `grid_layout` returns a
    `GridLayout` (gpui-component grid_layout.rs:40-64) and `h_resizable` a
    `ResizablePanelGroup` (gpui-component resizable.rs:29).

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

The egui showcase is matched by the gpui rule (`shows_gpui`), with its own
roots (`use egui::…` / `use egui_extras::…`) and its own `EGUI_VIA` table of
the calls that build a widget without naming it. Its `Ui` methods are matched
by `shows_ui_method`: a method counts only as a call on a receiver named `ui`
(a turbofish before the call admitted), because `small`, `strong`, `weak`,
`code`, `monospace` and `heading` are also `RichText` builders and `small` a
`Button` one, which a bare `.name(` would count.
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
SHOWCASE_EGUI = os.path.join(
    PROJECT_ROOT, "connectors", "native-theme-egui", "examples", "showcase-egui"
)
# The showcase's tests call `By::label` and the like, which are not widgets.
SHOWCASE_EGUI_EXCLUDE = ("tests.rs",)

# A by-value `impl Widget for Type`: the type name follows `for` directly, so
# the `&mut` and `&` impls of style.rs and introspection.rs do not match, and
# the blanket `impl<F> Widget for F` yields a name no `pub struct` declares.
WIDGET_FOR = re.compile(
    r"^\s*impl(?:<[^>]*>)?\s+Widget\s+for\s+([A-Za-z_]\w*)\b(?!::)", re.M
)
# An inherent impl block at column 0 (no `for`), with its type name.
IMPL_HEAD = re.compile(
    r"^impl(?:<[^>]*>)?\s+([A-Za-z_]\w*)(?:<[^>]*>)?\s*(?:where[^{]*)?\{", re.M
)
PUB_FN = re.compile(r"\s*pub fn (\w+)\b")
EGUI_CONTAINER_METHODS = re.compile(r"show|show_\w+|ui|body")
EGUI_EXTRAS_METHODS = re.compile(r"show|show_\w+|ui|horizontal|vertical|header|body")
# The `impl Ui` blocks of egui/src/ui.rs whose doc heading names widget-adding
# methods (ui.rs:1504, :2039, :2134, :2768).
UI_HEADINGS = ("# Adding widgets", "# Colors", "# Adding Containers / Sub-uis:", "# Menus")
RESPONSE_RETURN = re.compile(r"->\s*(?:egui::)?Response\b")

RENDER_IMPL = re.compile(
    r"^\s*impl(?:\s*<[^>]*>)?\s+(?:\w+::)*(?:RenderOnce|IntoElement)\s+for\s+([A-Za-z_]\w*)"
)
DERIVE = re.compile(r"#\[derive\(([^)]*)\)\]")
RENDER_DERIVES = {"IntoElement", "IntoPlot"}
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


def shows_iced(haystack, name):
    # A module of iced_widget is shown where the showcase names it, never
    # where a name only coincides with it: `iced::advanced::overlay` is
    # iced_core's overlay module, and `fn overlay` / `.overlay(` are the
    # `Widget::overlay` method a widget wrapper implements and forwards, none
    # of them iced_widget's `overlay` module.
    return (
        re.search(
            r"(?<!\w)(?<!advanced::)(?<!\.)(?<!fn )" + re.escape(name) + r"(?!\w)",
            haystack,
        )
        is not None
    )


# gpui widgets the showcase renders without ever naming their type: an
# extension method builds one from the element it is called on. Each pattern is
# that call, so the proof stays as specific as a constructor would be.
GPUI_VIA = {
    # `ContextMenuExt::context_menu` -> `ContextMenu::new(id, self).menu(f)`
    # (menu/context_menu.rs:39).
    "ContextMenu": r"\.context_menu\s*\(",
    # `ScrollableElement::overflow_scrollbar` and its two axis forms each
    # return `Scrollable<Self>` (scroll/scrollable.rs:46, 53, 60).
    "Scrollable": r"\.overflow(?:_[xy])?_scrollbar\s*\(",
    # The three `WindowExt` openers hand the widget itself to a builder
    # closure -- `Fn(Dialog, …) -> Dialog` (window_ext.rs:75-77),
    # `Fn(AlertDialog, …) -> AlertDialog` (:101-103), `Fn(Sheet, …) -> Sheet`
    # (:20-22, and `open_sheet_at` at :30-32) -- so an application never names
    # the type to render one.
    "Dialog": r"\.open_dialog\s*\(",
    "AlertDialog": r"\.open_alert_dialog\s*\(",
    "Sheet": r"\.open_sheet(?:_at)?\s*\(",
    # `Accordion::item` builds the item itself and hands it to a builder
    # closure, `FnOnce(AccordionItem) -> AccordionItem` (accordion.rs:63-70),
    # so an application never names the type to render one: an item is
    # shown where an Accordion is built and given one in the same statement.
    "AccordionItem": r"\bAccordion::new\s*\([^;]*?\.item\s*\(",
    # `grid_layout` returns gpui-component's `GridLayout`
    # (gpui-component grid_layout.rs:40-64); not a path segment of another
    # call, not a `fn` of the showcase's own.
    "GridLayout": r"(?<!fn )(?<![\w:.])grid_layout\s*\(",
    # `h_resizable` and `v_resizable` return a `ResizablePanelGroup`
    # (gpui-component resizable.rs:29, :34).
    "ResizablePanelGroup": r"(?<![\w:.])[hv]_resizable\s*\(",
    # gpui-base's `resizable_panel` returns a `ResizablePanel`
    # (gpui-base resizable/mod.rs:29; re-exported at gpui-component
    # resizable.rs:18-21).
    "ResizablePanel": r"(?<![\w:.])resizable_panel\s*\(",
    # `ScrollableElement::scrollbar`, `vertical_scrollbar`, `horizontal_scrollbar`
    # and `Scrollable`, which `overflow_*_scrollbar` returns, each draw a gpui-base
    # `Scrollbar` (gpui-component scroll/scrollable.rs:19-41, :221, :251, :327).
    "Scrollbar": r"\.(?:overflow(?:_[xy])?_|vertical_|horizontal_)?scrollbar\s*\(",
}

# The `\w+::` run immediately before a name, i.e. the path it is a segment of.
PATH_TAIL = re.compile(r"((?:\w+::)+)$")


def toolkit_roots(src, crates=("gpui_component", "gpui_kit")):
    """The identifiers a `use <crate>::…` brings in, for the given crates.

    A path-qualified name belongs to the toolkit when its *root* resolves
    there, and the showcase's `use` statements are what say so:
    `form::Form::horizontal()` is gpui-component's `Form` because `form` came
    from `use gpui_component::{…, form::{self, Field}, …}`, while
    `std::process::Command::new` is not gpui-component's `Command`.
    """
    roots = set(crates)
    for m in re.finditer(r"\buse\s+(?:" + "|".join(map(re.escape, crates)) + r")\s*::", src):
        end = src.find(";", m.end())
        if end < 0:
            continue
        roots.update(re.findall(r"\b[a-z_][a-z0-9_]*\b", src[m.end() : end]))
    roots.difference_update({"self", "as", "crate", "super"})
    return roots


def shows_gpui(haystack, name, roots=None, via=GPUI_VIA):
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
    pattern = via.get(name)
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


def showcase_files(path, exclude=()):
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
        if name.endswith(".rs") and name not in exclude
    )
    if not files:
        raise Failure(f"no .rs files in the showcase directory {path}")
    return files


def read_showcase(path, strip_literals=False, exclude=()):
    texts = []
    for file in showcase_files(path, exclude):
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


def rendered(text):
    """The `pub struct`s one source text declares, and the types it renders:
    an `impl RenderOnce`/`IntoElement`, or a `#[derive(IntoElement)]` or
    `#[derive(IntoPlot)]`."""
    found = set()
    lines = text.splitlines()
    for i, line in enumerate(lines):
        if m := RENDER_IMPL.match(line):
            found.add(m.group(1))
        d = DERIVE.search(line)
        if d and RENDER_DERIVES & {x.strip() for x in d.group(1).split(",")}:
            for follow in lines[i + 1 : i + 8]:
                if s := STRUCT.match(follow):
                    found.add(s.group(1))
                    break
    return {m.group(1) for m in PUB_STRUCT.finditer(text)}, found


def gpui_widgets(src):
    """Public types that render themselves: `RenderOnce` / `IntoElement` /
    `IntoPlot`."""
    found, public = set(), set()
    for path in sorted(rust_files(src)):
        with open(path, encoding="utf-8", errors="replace") as f:
            text = strip_test_modules(strip_comments(f.read()))
        structs, renders = rendered(text)
        public |= structs
        found |= renders
    return sorted(found & public)


LITERAL_START = re.compile(r"""\bb?r(#*)"|"|'(?:\\.|[^\\'])'""")
MOD_DECL = re.compile(
    r"((?:#\[[^\]]*\]\s*)*)(?<![\w:])(pub(?:\s*\([^)]*\))?\s+)?mod\s+([A-Za-z_]\w*)\s*([;{])"
)
PATH_ATTR = re.compile(r"#\[path\s*=\s*\"([^\"]*)\"\]")
PUB_USE = re.compile(r"(?<![\w:])pub(\s*\([^)]*\))?\s+use\s+([^;]*);")
USE_TOKEN = re.compile(r"::|[{},*]|\w+")


def blank_literals(src):
    """`src` with every string and char literal blanked to spaces, its newlines
    kept, so that offsets stay the same and a brace inside a literal is not
    counted."""
    out, i, n = [], 0, len(src)
    while (m := LITERAL_START.search(src, i)) is not None:
        if m.group(1) is not None:
            close = '"' + m.group(1)
            end = src.find(close, m.end())
            end = n if end < 0 else end + len(close)
        elif m.group(0) == '"':
            end = m.end()
            while end < n and src[end] != '"':
                end += 2 if src[end] == "\\" else 1
            end = min(end + 1, n)
        else:
            end = m.end()
        out.append(src[i : m.start()])
        out.append(re.sub(r"[^\n]", " ", src[m.start() : end]))
        i = end
    out.append(src[i:])
    return "".join(out)


def outer_blocks(blank):
    """The outermost `{ … }` blocks of a literal-blanked text, as (open, close)
    offsets; an unclosed block runs to the end."""
    blocks, depth, start = [], 0, 0
    for m in re.finditer(r"[{}]", blank):
        if m.group(0) == "{":
            if depth == 0:
                start = m.start()
            depth += 1
        elif depth:
            depth -= 1
            if depth == 0:
                blocks.append((start, m.start()))
    if depth:
        blocks.append((start, len(blank)))
    return blocks


def use_tree(tree):
    """The (path, public name) pairs of a `use` tree: braces nest, `X as Y` is
    the item `X` under the name `Y`, a path ending in `*` is a glob, and
    `self` inside braces is the module itself. `as _` names nothing."""
    tokens = USE_TOKEN.findall(tree)

    def at(i):
        return tokens[i] if i < len(tokens) else ""

    def parse(i, prefix):
        path = list(prefix)
        while True:
            token = at(i)
            if token == "{":
                pairs, i = [], i + 1
                while at(i) not in ("}", ""):
                    sub, i = parse(i, path)
                    pairs += sub
                    if at(i) == ",":
                        i += 1
                if at(i) != "}":
                    raise Failure(f"an unclosed `use` tree: {tree.strip()}")
                return pairs, i + 1
            if token == "*":
                return [(path + ["*"], None)], i + 1
            if token == "::" and not path:
                i += 1
                continue
            if not re.fullmatch(r"\w+", token):
                raise Failure(f"a `use` tree this rule cannot read: {tree.strip()}")
            path.append(token)
            i += 1
            if at(i) == "::":
                i += 1
                continue
            alias = None
            if at(i) == "as":
                alias, i = at(i + 1), i + 2
            if path[-1] == "self" and len(path) > 1:
                path.pop()
            if alias == "_":
                return [], i
            return [(path, alias or path[-1])], i

    pairs, i = parse(0, [])
    if i != len(tokens):
        raise Failure(f"a `use` tree this rule cannot read: {tree.strip()}")
    return pairs


def read_module(text):
    """One module's declarations, from its comment- and test-stripped text.

    Only items at the module's top level count. An inline `mod name { … }` is
    a module of its own (`inline`), and its body is no part of the module
    around it. `mods` maps each declared module to its visibility (`""` for
    none) and its `#[path]`; `uses` is every `pub use` whose visibility is
    exactly `pub`, as (path, public name) pairs; `structs` and `renders` are
    the discovery rule's, read in this module alone.
    """
    blank = blank_literals(text)
    blocks = outer_blocks(blank)
    opens = {open_: close for open_, close in blocks}

    def top(pos):
        return not any(open_ < pos <= close for open_, close in blocks)

    inline = {}
    own, own_blank = list(text), list(blank)
    for m in MOD_DECL.finditer(blank):
        if m.group(4) == "{" and top(m.start()) and (m.end(4) - 1) in opens:
            open_, close = m.end(4) - 1, opens[m.end(4) - 1]
            inline[m.group(3)] = read_module(text[open_ + 1 : close])
            for i in range(open_ + 1, close):
                if own[i] != "\n":
                    own[i] = own_blank[i] = " "
    text, blank = "".join(own), "".join(own_blank)
    blocks = outer_blocks(blank)
    mods, uses = {}, []
    for m in MOD_DECL.finditer(blank):
        if top(m.start()):
            path = PATH_ATTR.search(text[m.start(1) : m.end(1)])
            mods[m.group(3)] = (
                re.sub(r"\s+", "", m.group(2) or ""),
                path.group(1) if path else None,
            )
    for m in PUB_USE.finditer(blank):
        if top(m.start()) and not m.group(1):
            uses += use_tree(m.group(2))
    structs, renders = rendered(blank)
    return {
        "mods": mods,
        "inline": inline,
        "uses": uses,
        "structs": structs,
        "renders": renders & structs,
    }


def read_source(path, what):
    try:
        with open(path, encoding="utf-8", errors="replace") as f:
            return read_module(strip_test_modules(strip_comments(f.read())))
    except OSError as err:
        raise Failure(
            f"cannot read {path}, {what}; the re-export rule no longer matches "
            f"gpui-base's layout: {err}"
        ) from err


class BaseModules:
    """gpui-base's modules, read from `lib.rs` down as Rust finds them, and the
    paths of its `pub use` items followed as Rust resolves them. `externals`
    are the crate names gpui-base's dependencies are known by (`std`, `core`
    and `alloc` added): a path's first segment counts as an external crate
    only when it is one of them."""

    def __init__(self, src, externals=()):
        root = read_source(os.path.join(src, "lib.rs"), "gpui-base's root")
        self.records = {(): self.place(root, src, src)}
        self.externals = set(externals) | {"std", "core", "alloc"}

    @staticmethod
    def place(record, folder, here):
        """`folder` holds the record's file modules; `here`, the declaring
        file's folder, is what a `#[path]` is relative to."""
        record["folder"], record["here"] = folder, here
        for name, child in record["inline"].items():
            BaseModules.place(child, os.path.join(folder, name), here)
        return record

    def module(self, key):
        if key in self.records:
            return self.records[key]
        parent = self.module(key[:-1])
        name = key[-1]
        if name in parent["inline"]:
            record = parent["inline"][name]
        else:
            _, path_attr = parent["mods"][name]
            if path_attr:
                files = [os.path.join(parent["here"], path_attr)]
            else:
                files = [
                    os.path.join(parent["folder"], name + ".rs"),
                    os.path.join(parent["folder"], name, "mod.rs"),
                ]
            path = next((p for p in files if os.path.isfile(p)), files[0])
            what = "the file of gpui-base's module `" + "::".join(key) + "`"
            record = read_source(path, what)
            here = os.path.dirname(path)
            # rustc reads a file a `#[path]` names as it reads a `mod.rs`
            mod_rs = path_attr or os.path.basename(path) == "mod.rs"
            folder = here if mod_rs else path[:-3]
            self.place(record, folder, here)
        self.records[key] = record
        return record

    def walk(self, key, segments, outside, seen):
        """The module a path's module segments lead to from module `key`, or
        None for an external crate. `outside`: the path is gpui-component's,
        so its first segment must be gpui-base's too."""
        for i, seg in enumerate(segments):
            if seg == "crate":
                key = ()
                continue
            if seg == "self":
                continue
            if seg == "super":
                if not key:
                    raise Failure("the re-export rule no longer matches gpui-base's layout: "
                                  "`super` at the crate root")
                key = key[:-1]
                continue
            record = self.module(key)
            if seg in record["mods"]:
                key += (seg,)
                continue
            brought = [p for p, public in record["uses"] if public == seg and p[-1] != "*"]
            if brought and (key, seg) not in seen:
                key = self.walk(key, brought[0], False, seen | {(key, seg)})
                if key is None:
                    return None
                continue
            if i == 0 and not outside and seg in self.externals:
                return None
            raise Failure(
                "the re-export rule no longer matches gpui-base's layout: the module "
                f"`{'::'.join(key) or 'crate'}` declares or brings in no module `{seg}`"
            )
        return key

    def follow(self, key, path, outside=False, seen=frozenset()):
        """What a `use` path read in module `key` names: the (module, struct)
        pairs of a named item, and the (name, module, struct) triples of a
        glob or of a `pub mod` named whole."""
        module = self.walk(key, path[:-1], outside, seen)
        if module is None:
            return [], []
        if path[-1] == "*":
            return [], self.glob(module, seen)
        return self.lookup(module, path[-1], seen)

    def lookup(self, key, name, seen):
        if (key, name) in seen:
            return [], []
        seen = seen | {(key, name)}
        record = self.module(key)
        items, contents = [], []
        if name in record["structs"]:
            items.append((key, name))
        if record["mods"].get(name, ("",))[0] == "pub":
            contents += self.glob(key + (name,), seen)
        for path, public in record["uses"]:
            if path[-1] == "*":
                _, globbed = self.follow(key, path, False, seen)
                items += [(m, s) for n, m, s in globbed if n == name]
            elif public == name:
                found, whole = self.follow(key, path, False, seen)
                items += found
                contents += whole
        return items, contents

    def glob(self, key, seen):
        if (key, "*") in seen:
            return []
        seen = seen | {(key, "*")}
        record = self.module(key)
        found = [(s, key, s) for s in sorted(record["structs"])]
        for path, public in record["uses"]:
            items, contents = self.follow(key, path, False, seen)
            if path[-1] == "*":
                found += contents
            else:
                found += [(public, m, s) for m, s in items]
        return found

    def renders(self, key, name):
        return name in self.module(key)["renders"]


def component_exports(modules):
    """What each gpui-component module makes public, as Rust's visibility
    reaches it: transitively, along any chain of re-exports.

    Each module maps to [whole, named]. `whole`: every name it holds is
    public under its own name -- the root, a `pub mod` of a module whose
    name for it is public, and a module a public `pub use` names or globs.
    `named`: the names a public `pub use` elsewhere exports from it, each
    with the public names it is exported under. Computed as a fixpoint, so a
    name re-exported along a chain of private modules is carried along all
    of it (`mod time; pub use time::{calendar};` makes `time::calendar`
    whole; `pub use styled::*` and styled's `pub use
    crate::component_traits::{…}` export those names of `component_traits`).
    """

    def own_module(key, segments):
        for seg in segments:
            if seg == "crate":
                key = ()
            elif seg == "super" and key:
                key = key[:-1]
            elif seg != "self" and key + (seg,) in modules and seg in modules[key]["mods"]:
                key += (seg,)
            elif seg != "self":
                return None
        return key

    exports = {key: [key == (), {}] for key in modules}

    def mark(key, name=None, names=()):
        whole, named = exports[key]
        if name is None:
            exports[key][0] = True
            return not whole
        if set(names) <= named.get(name, set()):
            return False
        named.setdefault(name, set()).update(names)
        return True

    changed = True
    while changed:
        changed = False
        for key, record in modules.items():
            whole, named = exports[key]
            for name, (vis, _) in record["mods"].items():
                if vis == "pub" and key + (name,) in modules and public_names(exports, key, name):
                    changed |= mark(key + (name,))
            for path, name in record["uses"]:
                if path[-1] == "*":
                    target = own_module(key, path[:-1])
                    if target is None:
                        continue
                    if whole:
                        changed |= mark(target)
                    for item, names in list(named.items()):
                        changed |= mark(target, item, names)
                    continue
                names = public_names(exports, key, name)
                if not names:
                    continue
                target = own_module(key, path[:-1])
                if target is not None:
                    changed |= mark(target, path[-1], names)
    return exports


def public_names(exports, key, name):
    """The public names `name`, as module `key` holds it, is known by."""
    whole, named = exports[key]
    return ({name} if whole else set()) | named.get(name, set())


def gpui_base_reexports(component_src, base_src, externals=()):
    """The public names of the gpui-base widgets gpui-component makes public.

    A `pub use gpui_base::…;` (exactly `pub`) counts for the names its module
    makes public (`component_exports`): all of them in a module public from
    the root down, and in one that is not, those a public `pub use`
    re-exports from it, by name or by glob, along any chain of modules. Its
    path is followed through gpui-base's own modules (`BaseModules`, with
    `externals` the names of gpui-base's dependencies), and an item counts
    when the file that declares its `pub struct` also renders it, as
    `gpui_widgets` asks. The name is gpui-component's public one.
    """
    base = BaseModules(base_src, externals)
    modules = {}

    def add(key, record):
        modules[key] = record
        for name, child in record["inline"].items():
            add(key + (name,), child)

    for path in sorted(rust_files(component_src)):
        parts = os.path.relpath(path, component_src)[:-3].split(os.sep)
        if parts == ["lib"] or parts[-1] == "mod":
            parts = parts[:-1]
        add(tuple(parts), read_source(path, "a gpui-component source"))

    exports = component_exports(modules)
    found = set()
    for key, record in modules.items():
        for path, name in record["uses"]:
            if path[0] != "gpui_base" or len(path) < 2:
                continue
            items, contents = base.follow((), path[1:], outside=True)
            for m, s in items:
                if base.renders(m, s):
                    found |= public_names(exports, key, name)
            # a glob brings each name in; a module named whole is public
            # where its own name is
            whole = path[-1] == "*" or public_names(exports, key, name)
            for n, m, s in contents:
                if base.renders(m, s) and whole:
                    found |= public_names(exports, key, n) if path[-1] == "*" else {n}
    return sorted(found)


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
    path = os.path.join(src, "widget.rs")
    try:
        with open(path, encoding="utf-8") as f:
            text = strip_comments(f.read())
    except OSError as err:
        raise Failure(
            f"cannot read {path}, where iced_aw's feature-gated widget modules "
            f"are declared; the discovery rules no longer match its layout: {err}"
        ) from err
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


def inherent_impls(text):
    """(type name, block body) for every inherent `impl` block at column 0."""
    for m in IMPL_HEAD.finditer(text):
        if re.search(r"\bfor\b", text[m.start() : m.end()]):
            continue
        depth, i = 1, m.end()
        while i < len(text) and depth:
            if text[i] == "{":
                depth += 1
            elif text[i] == "}":
                depth -= 1
            i += 1
        yield m.group(1), text[m.end() : i - 1]


def methods_of(body):
    """The `pub fn` names declared directly in an impl body (depth 0)."""
    names, depth = [], 0
    for line in body.splitlines():
        if depth == 0 and (m := PUB_FN.match(line)):
            names.append(m.group(1))
        depth += line.count("{") - line.count("}")
    return names


def egui_widgets(src):
    """Public egui types drawn by value (`impl Widget for T`) or shown by a
    container method (`show`, `show_*`, `ui`, `body`) under src/containers/ or
    in src/grid.rs.

    `body` admits `HeaderResponse`, whose `body` shows a `CollapsingState`'s
    content (containers/collapsing_header.rs:297).
    """
    found, public = set(), set()
    for path in sorted(rust_files(src)):
        rel = os.path.relpath(path, src)
        with open(path, encoding="utf-8", errors="replace") as f:
            text = strip_test_modules(strip_comments(f.read()))
        public |= {m.group(1) for m in PUB_STRUCT.finditer(text)}
        found |= {m.group(1) for m in WIDGET_FOR.finditer(text)}
        if rel.startswith("containers" + os.sep) or rel == "grid.rs":
            for name, body in inherent_impls(text):
                if any(EGUI_CONTAINER_METHODS.fullmatch(fn) for fn in methods_of(body)):
                    found.add(name)
    return sorted(found & public)


def egui_ui_methods(src):
    """The `pub fn`s of the `impl Ui` blocks under egui's widget-adding
    headings (`UI_HEADINGS`).

    The heading is a doc comment on the line above `impl Ui {`, so it is read
    from the raw file; the block is walked on the comment-stripped text, whose
    line numbers are the raw file's as long as no block comment spans lines.
    """
    path = os.path.join(src, "ui.rs")
    try:
        with open(path, encoding="utf-8") as f:
            raw = f.read().splitlines()
    except OSError as err:
        raise Failure(
            f"cannot read {path}, where egui's `impl Ui` blocks are; the "
            f"discovery rules no longer match its layout: {err}"
        ) from err
    stripped = strip_comments("\n".join(raw)).splitlines()
    if len(stripped) != len(raw):
        raise Failure(f"{path}: a block comment spans lines; the Ui-method rule walks lines")
    names = []
    for i, line in enumerate(raw[:-1]):
        if line.strip() not in ("/// " + h for h in UI_HEADINGS):
            continue
        if not raw[i + 1].startswith("impl Ui {"):
            continue
        depth = 0
        for body in stripped[i + 1 :]:
            if depth == 1 and (m := PUB_FN.match(body)):
                names.append(m.group(1))
            depth += body.count("{") - body.count("}")
            if depth == 0:
                break
    if not names:
        raise Failure(f"{path}: none of the headings {UI_HEADINGS} found above an `impl Ui`")
    return names


def egui_extras_widgets(src):
    """Public egui_extras types drawn by value or shown by a method (`show`,
    `show_*`, `ui`, `horizontal`, `vertical`, `header`, `body`), plus every
    module-level `pub fn` returning `egui::Response`, e.g.
    `syntax_highlighting::code_view_ui` (syntax_highlighting.rs:10-15)."""
    found, public, free = set(), set(), set()
    for path in sorted(rust_files(src)):
        with open(path, encoding="utf-8", errors="replace") as f:
            text = strip_test_modules(strip_comments(f.read()))
        public |= {m.group(1) for m in PUB_STRUCT.finditer(text)}
        found |= {m.group(1) for m in WIDGET_FOR.finditer(text)}
        for name, body in inherent_impls(text):
            if any(EGUI_EXTRAS_METHODS.fullmatch(fn) for fn in methods_of(body)):
                found.add(name)
        # Module level: a `pub fn` at column 0, its signature up to the body.
        for m in re.finditer(r"^pub fn (\w+)\b[^{;]*", text, re.M):
            if RESPONSE_RETURN.search(m.group(0)):
                free.add(m.group(1))
    return sorted((found & public) | free)


# egui widgets the showcase builds without naming their type: each pattern is
# the call that returns one, as `GPUI_VIA`'s are.
EGUI_VIA = {
    # `NativeThemeUiExt::native_frame` and `ThemeAtlas::surface_frame` each
    # return an `egui::Frame` (the connector spec, §4.5 and §4.2).
    "Frame": r"\.(?:native_frame|surface_frame)\s*\(",
    # `native_theme_egui::icons::to_image` returns an `Image` (§4.10).
    "Image": r"\bicons::to_image\s*\(",
    # `Ui::menu_button`, `menu_image_button` and `menu_image_text_button` build
    # a `MenuButton` (ui.rs:2793-2797).
    "MenuButton": r"\.menu_(?:image_|image_text_)?button\s*\(",
    # `SubMenuButton::new` holds a `SubMenu` (containers/menu.rs:357).
    "SubMenu": r"\bSubMenuButton::new\s*\(",
    # `CollapsingState::show_header` returns a `HeaderResponse`
    # (containers/collapsing_header.rs:129, :141).
    "HeaderResponse": r"\.show_header\s*\(",
    # egui_extras: `TableBuilder::header` returns a `Table` and
    # `TableBuilder::body` builds one (table.rs:452, :527, :554), so a table
    # is shown where a `TableBuilder` chain reaches either call.
    "Table": r"\bTableBuilder::new\s*\([^;]*?\.(?:header|body)\s*\(",
}


def shows_ui_method(haystack, name):
    """A `Ui` method counts only on a receiver named `ui`: `small`, `strong`,
    `weak`, `code`, `monospace` and `heading` are also `RichText` builders, and
    `small` a `Button` one, which a bare `.name(` would count. The showcase
    names every `Ui` it draws into `ui`, as egui's own examples do. A
    turbofish between the name and the call counts: `dnd_drop_zone` takes its
    payload type as one (`ui.dnd_drop_zone::<Payload, _>(`, ui.rs:2694)."""
    return (
        re.search(
            r"\bui\s*\.\s*" + re.escape(name) + r"\s*(?:::\s*<[^;{}]*?>\s*)?\(",
            haystack,
        )
        is not None
    )


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
    for section in ("gpui", "iced_widget", "iced_aw", "egui", "egui_ui", "egui_extras"):
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

    An empty `discovered` is an input error: an upstream layout the discovery
    rules no longer match would otherwise report `missing 0`, and a section
    with no exceptions has no stale one to catch it.
    """
    if not discovered:
        raise Failure(
            f"[{section}]: the discovery rules found no widget in the toolkit's "
            "sources; they no longer match its layout"
        )
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


def self_test():
    """The guards against an upstream layout the rules no longer match, run on
    synthetic crate roots: an empty discovery, a moved file, and one widget
    found and shown, so the guards are not what makes every run fail."""
    import tempfile

    failures = []

    def expect(label, got, want):
        if got != want:
            failures.append(f"{label}: got {got!r}, want {want!r}")

    def fails(label, call):
        try:
            call()
            failures.append(f"{label}: no error")
        except Failure:
            pass
        except OSError as err:
            failures.append(f"{label}: {type(err).__name__} instead of a Failure")

    with tempfile.TemporaryDirectory() as root:
        empty = os.path.join(root, "empty")
        os.makedirs(empty)
        with open(os.path.join(empty, "lib.rs"), "w", encoding="utf-8") as f:
            f.write("pub struct Label;\nimpl Label { pub fn new() -> Self { Label } }\n")
        crate = os.path.join(root, "crate")
        os.makedirs(os.path.join(crate, "containers"))
        with open(os.path.join(crate, "label.rs"), "w", encoding="utf-8") as f:
            f.write("pub struct Label;\nimpl Widget for Label {\n}\n")
        with open(os.path.join(crate, "chart.rs"), "w", encoding="utf-8") as f:
            f.write(
                "#[derive(IntoPlot)]\npub struct BarChart<T>\nwhere\n    T: 'static,\n{\n}\n"
                "#[derive(Clone, IntoElement)]\npub struct Badge;\n"
                "#[derive(IntoPlot)]\nstruct PrivatePlot;\n"
            )

        # M8-30: a section whose discovery finds nothing fails, every section
        for section, discover in (
            ("gpui", gpui_widgets),
            ("iced_widget", iced_modules),
            ("egui", egui_widgets),
            ("egui_extras", egui_extras_widgets),
        ):
            found = {w: [w] for w in discover(empty)}
            expect(f"{section}: the empty crate root discovers nothing", found, {})
            fails(f"{section}: an empty discovery", lambda: check(section, found, found, "", {}, []))
        for section in ("iced_aw", "egui_ui"):
            fails(f"{section}: an empty discovery", lambda: check(section, {}, {}, "", {}, []))

        # a file the rules read that upstream moved: a Failure, not a traceback
        fails("iced_aw: no src/widget.rs", lambda: aw_modules(empty))
        fails("egui_ui: no src/ui.rs", lambda: egui_ui_methods(empty))

        # an `IntoPlot` chart is discovered as an `IntoElement` type is, public only
        expect("gpui: the `IntoPlot` and `IntoElement` derives are discovered",
               gpui_widgets(crate), ["Badge", "BarChart"])

        # one widget discovered and shown passes; not shown, it is missing
        found = {w: [w] for w in egui_widgets(crate)}
        expect("egui: a by-value `impl Widget` is discovered", found, {"Label": ["Label"]})
        report = []
        expect("a shown widget passes",
               check("egui", found, found, "ui.add(Label);", {}, report), ([], []))
        expect("an unshown widget is missing",
               check("egui", found, found, "", {}, report), (["Label"], []))

        # the gpui-base re-export rule, on synthetic gpui-base and
        # gpui-component roots: each case adds its lines to the roots below
        base_files = {
            "lib.rs": "pub mod grid;\nmod vl;\nmod fold;\npub mod traits;\npub mod plot;\n"
            "mod text;\npub mod acts;\npub use vl::VList;\npub use fold::Fold;\n"
            "pub use text::Txt;\n",
            "grid/mod.rs": "mod g;\npub use g::Grid;\n",
            "grid/g.rs": "#[derive(IntoElement)]\npub struct Grid;\n",
            "vl.rs": "pub struct VList;\nimpl IntoElement for VList {}\n",
            "fold.rs": "pub struct Fold;\nimpl RenderOnce for Fold {}\n",
            "traits.rs": "pub trait Fold {}\n",
            "plot/mod.rs": "mod label;\npub use label::*;\n",
            "plot/label.rs": "pub struct Txt;\n",
            "text.rs": "pub enum Txt {}\nimpl RenderOnce for Txt {}\n",
            "acts.rs": "pub struct Act;\nimpl RenderOnce for Act {}\n",
        }
        component_files = {
            "lib.rs": "pub mod grid;\nmod vl;\npub mod traits;\npub mod plot;\npub mod hidden;\n"
            "pub(crate) mod acts {\n    pub use gpui_base::acts::*;\n}\n"
            "pub use vl::{VList};\npub use gpui_base::{Fold as Folder};\n",
            "grid.rs": "pub use gpui_base::grid::*;\n",
            "vl.rs": "pub use gpui_base::VList;\n",
            "traits.rs": "pub use gpui_base::traits::Fold;\n",
            "plot.rs": "pub use gpui_base::plot::*;\n",
            "hidden.rs": "pub(crate) use gpui_base::Fold;\n",
        }
        roots = ["Folder", "Grid", "VList"]

        def reexports(label, base_more, component_more, externals=()):
            case = os.path.join(root, "reexport-" + re.sub(r"\W+", "-", label))
            for name, files, more in (
                ("base", base_files, base_more),
                ("component", component_files, component_more),
            ):
                for rel in sorted(set(files) | set(more)):
                    path = os.path.join(case, name, rel)
                    os.makedirs(os.path.dirname(path), exist_ok=True)
                    with open(path, "w", encoding="utf-8") as f:
                        f.write(files.get(rel, "") + more.get(rel, ""))
            return lambda: gpui_base_reexports(
                os.path.join(case, "component"), os.path.join(case, "base"), externals
            )

        def finds(label, call, want):
            try:
                expect(label, call(), want)
            except Failure as err:
                failures.append(f"{label}: Failure {err}")

        finds("gpui-base: a module re-exported whole, a name through a private module "
              "re-exported at the root, a rename; not a trait, a non-rendering struct, "
              "a `pub(crate)` module or a `pub(crate) use`",
              reexports("roots", {}, {}), roots)
        finds("gpui-base: a `pub mod` re-exported by name is followed as a glob",
              reexports("pub-mod", {
                  "lib.rs": "pub mod anim;\n",
                  "anim.rs": "pub struct Fade;\nimpl RenderOnce for Fade {}\n",
              }, {"lib.rs": "pub use gpui_base::anim;\n"}),
              ["Fade"] + roots)
        finds("gpui-base: a path-qualified impl (`gpui::IntoElement`, gpui-base 0.10.0 "
              "plot/element.rs:50) under a cfg is discovered",
              reexports("qualified", {
                  "plot/mod.rs": "mod el;\npub use el::PlotEl;\n",
                  "plot/el.rs": "pub struct PlotEl<P>(P);\n#[cfg(not(feature = \"x\"))]\n"
                  "impl<P: Plot + 'static> gpui::IntoElement for PlotEl<P> {}\n",
              }, {}),
              ["Folder", "Grid", "PlotEl", "VList"])
        finds("gpui-base: an external crate's path yields nothing",
              reexports("external", {
                  "lib.rs": "pub use vl::Rope;\n",
                  "vl.rs": "pub use ropey::Rope;\n",
              }, {"lib.rs": "pub use gpui_base::Rope;\n"}, ("ropey",)),
              roots)
        fails("gpui-base: a first segment that is no module and no dependency",
              reexports("not-external", {
                  "lib.rs": "pub mod a;\nmod b;\n",
                  "b/mod.rs": "pub mod inner;\n",
                  "b/inner.rs": "pub struct W;\nimpl RenderOnce for W {}\n",
                  "a.rs": "use crate::b::inner;\npub use inner::W;\n",
              }, {"lib.rs": "pub use gpui_base::a::W;\n"}))
        finds("gpui-base: `crate::`, `self::` and `super::` are followed",
              reexports("crate-self-super", {
                  "lib.rs": "pub mod nest;\npub use crate::nest::Deep as Root2;\n",
                  "nest/mod.rs": "mod inner;\npub use self::inner::Deep;\n"
                  "pub use super::vl::VList as Again;\n",
                  "nest/inner.rs": "pub struct Deep;\nimpl RenderOnce for Deep {}\n",
              }, {"lib.rs": "pub use gpui_base::nest::{Deep, Again};\npub use gpui_base::Root2;\n"}),
              ["Again", "Deep", "Folder", "Grid", "Root2", "VList"])
        finds("gpui-base: a `#[path]` names the module's file",
              reexports("path", {
                  "lib.rs": "pub mod pathed;\n",
                  "pathed.rs": "#[path = \"other/place.rs\"]\nmod placed;\npub use placed::Placed;\n",
                  "other/place.rs": "#[derive(IntoElement)]\npub struct Placed;\n",
              }, {"lib.rs": "pub use gpui_base::pathed::Placed;\n"}),
              ["Folder", "Grid", "Placed", "VList"])
        finds("gpui-base: a submodule of `x.rs` is in the folder `x/`",
              reexports("x-folder", {
                  "lib.rs": "pub mod outer;\n",
                  "outer.rs": "mod sub;\npub use sub::Sub;\n",
                  "outer/sub.rs": "pub struct Sub;\nimpl IntoElement for Sub {}\n",
              }, {"lib.rs": "pub use gpui_base::outer::Sub;\n"}),
              ["Folder", "Grid", "Sub", "VList"])
        finds("gpui-base: an inline module is a module of its own",
              reexports("inline", {
                  "lib.rs": "pub mod host;\n",
                  "host.rs": "mod inner {\n    pub struct Nested;\n"
                  "    impl RenderOnce for Nested {}\n}\npub use inner::Nested;\n",
              }, {"lib.rs": "pub use gpui_base::host::Nested;\n"}),
              ["Folder", "Grid", "Nested", "VList"])
        finds("gpui-base: a `pub(crate) use` brings nothing in",
              reexports("pub-crate-use", {
                  "lib.rs": "pub mod hid;\n",
                  "hid.rs": "pub(crate) use crate::fold::Fold;\n",
              }, {"lib.rs": "pub use gpui_base::hid::Fold;\n"}),
              roots)
        fails("gpui-base: a gpui-component path's first module that gpui-base lacks",
              reexports("gone", {}, {"lib.rs": "pub use gpui_base::gone::X;\n"}))
        fails("gpui-base: a module segment that names nothing",
              reexports("nowhere", {}, {"lib.rs": "pub use gpui_base::grid::nowhere::X;\n"}))
        widget_w = {
            "lib.rs": "pub mod m;\nmod w;\npub use w::W;\n",
            "w.rs": "pub struct W;\nimpl RenderOnce for W {}\n",
            "m.rs": "pub struct V;\nimpl RenderOnce for V {}\n",
        }
        finds("gpui-component: a name exported through a chain of private globs",
              reexports("glob-chain", widget_w, {
                  "lib.rs": "mod a;\nmod b;\npub use a::W;\n",
                  "a.rs": "pub use crate::b::*;\n",
                  "b.rs": "pub use gpui_base::W;\n",
              }),
              ["Folder", "Grid", "VList", "W"])
        finds("gpui-component: a gpui-base module re-exported whole by a private module "
              "nothing re-exports",
              reexports("private-whole", widget_w, {
                  "lib.rs": "mod p;\n",
                  "p.rs": "pub use gpui_base::m;\n",
              }),
              roots)
        finds("gpui-component: a gpui-base widget named by a private module nothing "
              "re-exports",
              reexports("private-named", widget_w, {
                  "lib.rs": "mod p;\n",
                  "p.rs": "pub use gpui_base::W;\n",
              }),
              roots)
        fails("gpui-base: `super` at the crate root",
              reexports("super-root", {"lib.rs": "pub use super::vl::VList as Up;\n"},
                        {"lib.rs": "pub use gpui_base::Up;\n"}))
        finds("gpui-base: a name a module's glob brings in is found by name",
              reexports("glob-name", {
                  "lib.rs": "mod rz;\npub use rz::Panel;\n",
                  "rz/mod.rs": "mod panel;\npub use panel::*;\n",
                  "rz/panel.rs": "pub struct Panel;\nimpl RenderOnce for Panel {}\n",
              }, {"lib.rs": "pub use gpui_base::Panel;\n"}),
              ["Folder", "Grid", "Panel", "VList"])
        finds("gpui-base: a module segment a `pub use` of a module brings in",
              reexports("brought-module", {
                  "lib.rs": "mod m;\npub use m::sub;\n",
                  "m/mod.rs": "pub mod sub;\n",
                  "m/sub.rs": "pub struct Subbed;\nimpl RenderOnce for Subbed {}\n",
              }, {"lib.rs": "pub use gpui_base::sub::Subbed;\n"}),
              ["Folder", "Grid", "Subbed", "VList"])
        finds("gpui-base: a file a `#[path]` names finds its own modules beside it",
              reexports("path-mod-rs", {
                  "lib.rs": "#[path = \"x/p.rs\"]\npub mod p;\n",
                  "x/p.rs": "mod s;\npub use s::Ps;\n",
                  "x/s.rs": "pub struct Ps;\nimpl RenderOnce for Ps {}\n",
              }, {"lib.rs": "pub use gpui_base::p::Ps;\n"}),
              ["Folder", "Grid", "Ps", "VList"])
        finds("gpui-base: a brace in a string, char or raw string literal opens no block",
              reexports("literals", {
                  "lib.rs": "pub mod lit;\n",
                  "lit.rs": "const A: &str = \"{\";\nconst B: char = '{';\n"
                  "const C: &str = r#\"{\"#;\nmod w;\npub use w::Lit;\n",
                  "lit/w.rs": "pub struct Lit;\nimpl RenderOnce for Lit {}\n",
              }, {"lib.rs": "pub use gpui_base::lit::Lit;\n"}),
              ["Folder", "Grid", "Lit", "VList"])
        finds("gpui-base: `as _` names nothing",
              reexports("as-underscore", {
                  "lib.rs": "mod under;\npub use under::Under;\n",
                  "under.rs": "pub struct Under;\nimpl RenderOnce for Under {}\n",
              }, {"lib.rs": "pub use gpui_base::Under as _;\n"}),
              roots)
        finds("gpui-component: a `pub mod` of a private module re-exported by name",
              reexports("private-parent", {
                  "lib.rs": "mod cal;\npub use cal::Cal;\n",
                  "cal.rs": "pub struct Cal;\nimpl RenderOnce for Cal {}\n",
              }, {
                  "lib.rs": "mod time;\npub use time::{calendar};\n",
                  "time/mod.rs": "pub mod calendar;\n",
                  "time/calendar.rs": "pub use gpui_base::Cal;\n",
              }),
              ["Cal", "Folder", "Grid", "VList"])
        finds("gpui-component: a name re-exported through two private modules",
              reexports("two-hops", {
                  "lib.rs": "mod sty;\npub use sty::Sty;\n",
                  "sty.rs": "pub struct Sty;\nimpl RenderOnce for Sty {}\n",
              }, {
                  "lib.rs": "mod ctraits;\nmod styled;\npub use styled::*;\n",
                  "styled.rs": "pub use crate::ctraits::{Sty};\n",
                  "ctraits.rs": "pub use gpui_base::Sty;\n",
              }),
              ["Folder", "Grid", "Sty", "VList"])

        # a free function that returns a widget counts through `GPUI_VIA`
        for text, name, want in (
            ('grid_layout("g", &state, cx)', "GridLayout", True),
            (".child(grid_layout(", "GridLayout", True),
            ("info::layout::grid_layout(t)", "GridLayout", False),
            ("fn grid_layout(", "GridLayout", False),
            ("demo::grid_layout_sample(", "GridLayout", False),
            ("grid_layout_sample(", "GridLayout", False),
            ("h_resizable(", "ResizablePanelGroup", True),
            ("v_resizable(", "ResizablePanelGroup", True),
            ("x::h_resizable(", "ResizablePanelGroup", False),
            ("[resizable_panel()", "ResizablePanel", True),
            ("info::chrome::resizable_panel(t)", "ResizablePanel", False),
            (".vertical_scrollbar(h)", "Scrollbar", True),
            (".horizontal_scrollbar(h)", "Scrollbar", True),
            (".overflow_y_scrollbar()", "Scrollbar", True),
            (".overflow_scrollbar()", "Scrollbar", True),
            (".scrollbar_show(", "Scrollbar", False),
        ):
            expect(f"gpui: `{text}` shows {name}", shows_gpui(text, name), want)

    for line in failures:
        print("  FAIL " + line)
    if failures:
        print(f"self-test: {len(failures)} failure(s)")
        return 1
    print("self-test: every scenario passes")
    return 0


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--self-test",
        action="store_true",
        help="run the built-in scenarios instead of the check",
    )
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
    parser.add_argument(
        "--showcase-egui",
        default=SHOWCASE_EGUI,
        help="egui showcase to read instead of the connector's own: a directory "
        "whose .rs files are all read, its tests.rs left out",
    )
    args = parser.parse_args()
    if args.self_test:
        return self_test()

    meta = cargo_metadata()
    # Both sides strip literals. The gpui showcase did not, and its Widget
    # Info panels are written in prose that names upstream types on purpose --
    # `(select.rs, Caret::render)` in a note read as a call, and `Caret` was
    # reported as shown by a showcase that never builds one. A panel arguing
    # *about* a widget is not a demo of it.
    gpui_show = read_showcase(args.showcase_gpui, strip_literals=True)
    iced_show = read_showcase(args.showcase_iced, strip_literals=True)
    egui_show = read_showcase(
        args.showcase_egui, strip_literals=True, exclude=SHOWCASE_EGUI_EXCLUDE
    )
    exceptions = load_exceptions()

    gpui = {w: [w] for w in gpui_widgets(source_dir(meta, "gpui-component"))}
    base_crates = [
        (dep.get("rename") or dep.get("name", "")).replace("-", "_")
        for dep in package(meta, "gpui-base").get("dependencies", [])
    ]
    for w in gpui_base_reexports(
        source_dir(meta, "gpui-component"), source_dir(meta, "gpui-base"), base_crates
    ):
        gpui.setdefault(w, [w])
    iced = {m: [m] for m in iced_modules(source_dir(meta, "iced_widget"))}
    aw_all = aw_modules(source_dir(meta, "iced_aw"))
    aw = aw_enabled(meta, aw_all)
    egui = {w: [w] for w in egui_widgets(source_dir(meta, "egui"))}
    egui_ui = {m: [m] for m in egui_ui_methods(source_dir(meta, "egui"))}
    extras = {w: [w] for w in egui_extras_widgets(source_dir(meta, "egui_extras"))}
    egui_roots = toolkit_roots(egui_show, ("egui", "egui_extras"))

    def shows_in_egui(haystack, name):
        return shows_gpui(haystack, name, egui_roots, EGUI_VIA)

    gpui_roots = toolkit_roots(gpui_show)

    def shows_in_gpui(haystack, name):
        return shows_gpui(haystack, name, gpui_roots)

    report, missing, stale = [], [], []
    for section, discovered, universe, showcase, matches in (
        ("gpui", gpui, gpui, gpui_show, shows_in_gpui),
        ("iced_widget", iced, iced, iced_show, shows_iced),
        ("iced_aw", aw, aw_all, iced_show, shows),
        ("egui", egui, egui, egui_show, shows_in_egui),
        ("egui_ui", egui_ui, egui_ui, egui_show, shows_ui_method),
        ("egui_extras", extras, extras, egui_show, shows_in_egui),
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
        print(f"check_widget_coverage: {error}", file=sys.stderr)
        sys.exit(2)
