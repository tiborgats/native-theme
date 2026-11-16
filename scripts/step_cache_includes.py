#!/usr/bin/env python3
"""The files a cargo step's reached packages include from outside them.

scripts/step_cache.sh's reach runs it on the directories of the workspace
packages a step reaches, from the repository root, and adds each target it
prints to the step's inputs; a refusal refuses the reach, so the step runs:

    python3 scripts/step_cache_includes.py [--trusted <dir>]... <dir>...
    python3 scripts/step_cache_includes.py --self-test

`--trusted <dir>` names a package directory whose build script the gate has
reviewed (check_release.sh's CACHE_TRUSTED_BUILD_CODE): only there may an
include name `env!("OUT_DIR")`.

What it reads
-------------
Every `.rs` file under the directories, as `find -L <dir> -type f -name
'*.rs'` lists them: ignored files too and symbolic links followed (Cargo
compiles a `mod` file whether git ignores it or not, and rustc follows a
link). A file belongs to the deepest directory given that holds it. Comments
and literals are blanked with check_panic_patterns.py's `code_only`, loaded by
path, so that an include written in a comment or in a generated string is
not taken for one; each `include_str!`, `include_bytes!`, `include!` and
`#[path = ...]` is found in the blanked text, its argument read from the
original text at the same position.

Accepted
--------
* a string literal, resolved against the including file's directory (a
  `#[path]` too, whether or not it sits in an inline module block: rustc
  resolves one there against a deeper directory, so where this resolution
  stays inside the package, rustc's does too);
* a `concat!` of string literals without `..`, optionally led by
  `env!("CARGO_MANIFEST_DIR")`, resolved against the package's directory, or
  by `env!("OUT_DIR")` in a trusted package, the build script's output,
  which adds no input;
* inside a `macro_rules!` body, a `concat!` whose first part is a non-empty
  relative string literal and whose other parts are string literals or
  metavariables the arm's matcher declares `$name:literal`; it lies under its
  first part's directory, inside the package.

An include or `#[path]` inside a `macro_rules!` body, a literal one included,
is resolved where the outermost macro is invoked, not where it is defined, so
it is accepted only where no attribute before the macro names `macro_export`
(a `cfg_attr` one included), the macro's name occurs in the package's code in
the defining file alone (any occurrence), no occurrence there lies inside
another `macro_rules!` body, and no `use` item there names it (a re-export,
`use ... as` among them): then call site and definition share a directory.
The metavariable form also needs every string literal of the macro's
invocations free of `..` and `\\`.

Refused
-------
Any other argument (a `concat!` with another part, a metavariable outside
that form, another macro call, an identifier), a literal holding `\\` (an
escape or a Windows separator), a `#[path]` or `include!` whose target lies
outside the including package (its own includes are code the scan does not
follow), an `include!` of a file not ending in `.rs`, and every
`#[cfg_attr(..., path = ...)]`.

Output
------
Each target outside its including package, a line `<target>\\t<file>:<line>`,
the target relative to the working directory where it lies under it, and
exit 0; or each refusal and each file or directory it cannot read, a line
apiece, and exit 1.
"""

import importlib.util
import os
import re
import sys

INCLUDE = re.compile(r"(?<![A-Za-z0-9_])(include_str|include_bytes|include)\s*!\s*([\(\[\{])")
PATH_ATTR = re.compile(r"#\s*\[\s*path\s*=\s*")
CFG_ATTR = re.compile(r"#\s*!?\s*\[\s*cfg_attr\s*\(")
CFG_ATTR_PATH = re.compile(r"(?<![A-Za-z0-9_])path\s*=")
MACRO_RULES = re.compile(r"(?<![A-Za-z0-9_])macro_rules\s*!\s*([A-Za-z_][A-Za-z0-9_]*)\s*([\(\[\{])")
CONCAT = re.compile(r"(?<![A-Za-z0-9_])concat\s*!\s*([\(\[\{])")
ENV_PART = re.compile(r"env\s*!\s*\(\s*\"(CARGO_MANIFEST_DIR|OUT_DIR)\"\s*\)$")
METAVARIABLE = re.compile(r"\$([A-Za-z_][A-Za-z0-9_]*)$")
DECLARATION = re.compile(r"\$([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([A-Za-z_]+)")
RAW_START = re.compile(r"(b|c)?r(#*)\"")
IDENT_CHAR = re.compile(r"[A-Za-z0-9_]")
OPEN = "([{"
CLOSE = ")]}"


def load_code_only():
    """check_panic_patterns.py's `code_only`, loaded by path; no bytecode written."""
    sys.dont_write_bytecode = True
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "check_panic_patterns.py")
    spec = importlib.util.spec_from_file_location("step_cache_includes_lexer", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.code_only


def matching(code, i):
    """The index of the delimiter that closes the one at `i`, or -1."""
    depth = 0
    for j in range(i, len(code)):
        if code[j] in OPEN:
            depth += 1
        elif code[j] in CLOSE:
            depth -= 1
            if depth == 0:
                return j
    return -1


def skip_space(code, i, end=None):
    end = len(code) if end is None else end
    while i < end and code[i].isspace():
        i += 1
    return i


def literal_at(text, code, i):
    """The literal starting at `i`: (kind, value, end), kind "str" for a
    string literal, "other" for a byte or C string; None where none starts."""
    m = RAW_START.match(text, i)
    if m and (i == 0 or not IDENT_CHAR.match(text[i - 1])):
        close = '"' + m.group(2)
        end = code.find(close, m.end())
        if end < 0:
            return None
        kind = "other" if m.group(1) else "str"
        return kind, text[m.end():end], end + len(close)
    kind = "str"
    if text.startswith(('b"', 'c"'), i):
        kind = "other"
        i += 1
    if i < len(text) and text[i] == '"':
        end = code.find('"', i + 1)
        if end < 0:
            return None
        return kind, text[i + 1:end], end + 1
    return None


def whole_literal(text, code, start, end):
    """The string literal that is the whole of [start, end), or None."""
    i = skip_space(code, start, end)
    found = literal_at(text, code, i)
    if found is None or found[0] != "str" or skip_space(code, found[2], end) != end:
        return None
    return found[1]


def string_literals(text, code, start, end):
    """Every string literal's value in [start, end)."""
    values = []
    i = start
    while i < end:
        found = None
        if code[i] in 'bcr"' and (i == 0 or not IDENT_CHAR.match(code[i - 1])):
            found = literal_at(text, code, i)
        if found is not None and found[2] <= end:
            values.append(found[1])
            i = found[2]
        else:
            i += 1
    return values


def split_parts(code, start, end):
    """[start, end) split at its commas outside any delimiter, as (start, end)
    pairs; a last empty part (a trailing comma) left out."""
    parts = []
    depth = 0
    first = start
    for j in range(start, end):
        if code[j] in OPEN:
            depth += 1
        elif code[j] in CLOSE:
            depth -= 1
        elif code[j] == "," and depth == 0:
            parts.append((first, j))
            first = j + 1
    parts.append((first, end))
    if len(parts) > 1 and not code[parts[-1][0]:parts[-1][1]].strip():
        parts.pop()
    return parts


def attributes_before(code, i):
    """The attributes (their code) right before the item starting at `i`."""
    found = []
    while True:
        k = i - 1
        while k >= 0 and code[k].isspace():
            k -= 1
        if k < 0 or code[k] != "]":
            return found
        depth = 0
        m = k
        while m >= 0:
            if code[m] in CLOSE:
                depth += 1
            elif code[m] in OPEN:
                depth -= 1
                if depth == 0:
                    break
            m -= 1
        if m < 0 or code[m] != "[":
            return found
        h = m - 1
        while h >= 0 and code[h].isspace():
            h -= 1
        if h >= 0 and code[h] == "!":
            h -= 1
        if h < 0 or code[h] != "#":
            return found
        found.append(code[h:k + 1])
        i = h


def macro_arms(code, start, end):
    """The arms of a macro_rules! body [start, end): (matcher, transcriber)
    pairs of (start, end) ranges; None where the body does not parse."""
    arms = []
    i = skip_space(code, start, end)
    while i < end:
        if code[i] not in OPEN:
            return None
        matcher_end = matching(code, i)
        if matcher_end < 0 or matcher_end >= end:
            return None
        matcher = (i + 1, matcher_end)
        i = skip_space(code, matcher_end + 1, end)
        if not code.startswith("=>", i):
            return None
        i = skip_space(code, i + 2, end)
        if i >= end or code[i] not in OPEN:
            return None
        transcriber_end = matching(code, i)
        if transcriber_end < 0 or transcriber_end >= end:
            return None
        arms.append((matcher, (i + 1, transcriber_end)))
        i = skip_space(code, transcriber_end + 1, end)
        if i < end and code[i] == ";":
            i = skip_space(code, i + 1, end)
    return arms


def identifier(name):
    return re.compile(r"(?<![A-Za-z0-9_])" + re.escape(name) + r"(?![A-Za-z0-9_])")


def inside(path, directory):
    return path == directory or path.startswith(directory.rstrip(os.sep) + os.sep)


def shown(path):
    """`path` relative to the working directory where it lies under it."""
    cwd = os.getcwd()
    return os.path.relpath(path, cwd) if inside(path, cwd) else path


class Package:
    def __init__(self, directory, trusted):
        self.directory = os.path.normpath(os.path.abspath(directory))
        self.trusted = trusted
        self.files = []  # (path, text, code)


class Macro:
    def __init__(self, name, start, body):
        self.name = name
        self.start = start
        self.body = body  # (start, end): inside the delimiters
        self.verdict = None  # None: not yet judged; "": accepted; else a reason


def rs_files(top, problems):
    """Every .rs file under `top`, symbolic links followed (find -L)."""
    found = []

    def walk(directory, chain):
        try:
            names = sorted(os.listdir(directory))
        except OSError as error:
            problems.append("cannot list %s: %s" % (shown(directory), error.strerror))
            return
        for name in names:
            path = os.path.join(directory, name)
            if os.path.isdir(path):
                real = os.path.realpath(path)
                if real in chain:
                    problems.append("a symbolic-link loop at %s" % shown(path))
                    continue
                walk(path, chain | {real})
            elif name.endswith(".rs") and os.path.isfile(path):
                found.append(path)

    walk(top, {os.path.realpath(top)})
    return found


def macro_verdict(macro, path, code, package, macros):
    """"" where an include in `macro`'s body resolves at its definition, else why not."""
    if macro.verdict is not None:
        return macro.verdict
    reason = ""
    if any("macro_export" in attribute for attribute in attributes_before(code, macro.start)):
        reason = "is exported (#[macro_export])"
    pattern = identifier(macro.name)
    if not reason:
        for other_path, _, other_code in package.files:
            if other_path != path and pattern.search(other_code):
                reason = "is named in %s too" % shown(other_path)
                break
    if not reason:
        for m in pattern.finditer(code):
            for other in macros:
                if other is not macro and other.body[0] <= m.start() < other.body[1]:
                    reason = "is named inside the body of macro_rules! %s" % other.name
                    break
            if reason:
                break
    if not reason:
        for m in re.finditer(r"(?<![A-Za-z0-9_])use(?![A-Za-z0-9_])[^;]*;", code):
            if pattern.search(code, m.start(), m.end()):
                reason = "is named in a use item (a re-export)"
                break
    macro.verdict = reason
    return reason


def invocation_literals(macro, text, code):
    """Every string literal of `macro`'s invocations in its defining file."""
    values = []
    for m in re.finditer(r"(?<![A-Za-z0-9_])" + re.escape(macro.name) + r"\s*!\s*([\(\[\{])", code):
        open_at = m.end() - 1
        close_at = matching(code, open_at)
        if close_at < 0:
            values.append("\\")  # an invocation that does not close: refused
            continue
        values.extend(string_literals(text, code, open_at + 1, close_at))
    return values


def classify(text, code, start, end, macro, package):
    """The include argument [start, end): ("path", value) for a path to resolve
    against the including file's directory, ("absolute", path), ("out_dir",
    tail), ("under", first part, last literal or None, metavariables), or
    ("refused", why)."""
    value = whole_literal(text, code, start, end)
    if value is not None:
        if "\\" in value:
            return "refused", "a literal path holding \\"
        return "path", value
    i = skip_space(code, start, end)
    m = CONCAT.match(code, i)
    if not m:
        return "refused", "not a string literal or an accepted concat!"
    open_at = m.end() - 1
    close_at = matching(code, open_at)
    if close_at < 0 or skip_space(code, close_at + 1, end) != end:
        return "refused", "not a string literal or an accepted concat!"
    parts = split_parts(code, open_at + 1, close_at)
    lead = None
    literals = []
    metavariables = []
    last_literal = None
    for index, (part_start, part_end) in enumerate(parts):
        piece = code[part_start:part_end].strip()
        value = whole_literal(text, code, part_start, part_end)
        if value is not None:
            if "\\" in value:
                return "refused", "a concat! part holding \\"
            if ".." in value:
                return "refused", "a concat! part holding .."
            literals.append(value)
            last_literal = value
            continue
        env = ENV_PART.match(text[part_start:part_end].strip())
        if env and index == 0:
            lead = env.group(1)
            continue
        meta = METAVARIABLE.match(piece)
        if meta and index > 0 and macro is not None:
            metavariables.append(meta.group(1))
            last_literal = None
            continue
        return "refused", "a concat! part that is no string literal: %s" % " ".join(piece.split())
    if metavariables:
        first = whole_literal(text, code, parts[0][0], parts[0][1])
        if lead is not None or first is None or not first or first.startswith("/"):
            return "refused", "a concat! with a metavariable, not led by a relative string literal"
        return "under", first, last_literal, metavariables
    tail = "".join(literals)
    if lead == "OUT_DIR":
        if not package.trusted:
            return "refused", "env!(\"OUT_DIR\") in a package whose build script is not reviewed"
        return "out_dir", tail
    if lead == "CARGO_MANIFEST_DIR":
        return "absolute", os.path.normpath(package.directory + tail)
    return "path", tail


def scan(packages):
    """(targets, problems): each include target outside its package as
    (target, where), and each refusal or unreadable input."""
    problems = []
    owner = {}
    order = []
    for package in packages:
        for path in rs_files(package.directory, problems):
            key = os.path.realpath(path)
            if key not in owner:
                order.append((key, path))
            if key not in owner or len(package.directory) > len(owner[key][1].directory):
                owner[key] = (path, package)
    code_only = load_code_only()
    for key, _ in order:
        path, package = owner[key]
        try:
            with open(path, encoding="utf-8") as f:
                text = f.read()
        except (OSError, UnicodeDecodeError) as error:
            problems.append("cannot read %s: %s" % (shown(path), getattr(error, "strerror", None) or error))
            continue
        package.files.append((path, text, code_only(text)))
    targets = []
    for package in packages:
        for path, text, code in package.files:
            scan_file(path, text, code, package, targets, problems)
    return targets, problems


def scan_file(path, text, code, package, targets, problems):
    directory = os.path.dirname(os.path.abspath(path))
    macros = []
    for m in MACRO_RULES.finditer(code):
        close_at = matching(code, m.end() - 1)
        if close_at < 0:
            problems.append("%s: macro_rules! %s is not closed" % (where(path, code, m.start()), m.group(1)))
            continue
        macros.append(Macro(m.group(1), m.start(), (m.end(), close_at)))

    def enclosing(i):
        found = None
        for macro in macros:
            if macro.body[0] <= i < macro.body[1] and (found is None or macro.start > found.start):
                found = macro
        return found

    def refuse(i, why):
        problems.append("%s: %s" % (where(path, code, i), why))

    def in_macro(i, what):
        macro = enclosing(i)
        if macro is None:
            return None, True
        reason = macro_verdict(macro, path, code, package, macros)
        if reason:
            refuse(i, "%s inside the body of macro_rules! %s, which %s" % (what, macro.name, reason))
            return macro, False
        return macro, True

    for m in CFG_ATTR.finditer(code):
        close_at = matching(code, m.end() - 1)
        if close_at < 0 or CFG_ATTR_PATH.search(code, m.end(), close_at):
            refuse(m.start(), "a #[cfg_attr(..., path = ...)]")

    for m in PATH_ATTR.finditer(code):
        macro, ok = in_macro(m.start(), "a #[path]")
        if not ok:
            continue
        found = literal_at(text, code, m.end())
        if found is None or found[0] != "str":
            refuse(m.start(), "a #[path] that is no string literal")
            continue
        if "\\" in found[1]:
            refuse(m.start(), "a literal path holding \\")
            continue
        target = os.path.normpath(os.path.join(directory, found[1]))
        if not inside(target, package.directory):
            refuse(m.start(), "a #[path] to %s, outside its package" % shown(target))

    for m in INCLUDE.finditer(code):
        kind = m.group(1)
        macro, ok = in_macro(m.start(), "an %s!" % kind)
        if not ok:
            continue
        open_at = m.end() - 1
        close_at = matching(code, open_at)
        if close_at < 0:
            refuse(m.start(), "an %s! that does not close" % kind)
            continue
        verdict = classify(text, code, open_at + 1, close_at, macro, package)
        form = verdict[0]
        if form == "refused":
            refuse(m.start(), "%s!: %s" % (kind, verdict[1]))
            continue
        if form == "out_dir":
            if kind == "include" and not verdict[1].endswith(".rs"):
                refuse(m.start(), "an include! of a file not ending in .rs")
            continue
        if form == "under":
            first, last = verdict[1], verdict[2]
            arm = None
            arms = macro_arms(code, macro.body[0], macro.body[1])
            for matcher, transcriber in arms or []:
                if transcriber[0] <= m.start() < transcriber[1]:
                    arm = matcher
            if arm is None:
                refuse(m.start(), "%s!: a metavariable outside a macro arm read" % kind)
                continue
            declared = {}
            for d in DECLARATION.finditer(code, arm[0], arm[1]):
                declared.setdefault(d.group(1), set()).add(d.group(2))
            bad = None
            for name in verdict[3]:
                if declared.get(name) != {"literal"}:
                    bad = name
            if bad is not None:
                refuse(m.start(), "%s!: the metavariable $%s is not declared $%s:literal" % (kind, bad, bad))
                continue
            if any(".." in value or "\\" in value for value in invocation_literals(macro, text, code)):
                refuse(m.start(), "%s!: an invocation of %s! passes a literal holding .. or \\" % (kind, macro.name))
                continue
            if kind == "include" and (last is None or not last.endswith(".rs")):
                refuse(m.start(), "an include! of a file not ending in .rs")
                continue
            base = os.path.normpath(os.path.join(directory, os.path.dirname(first) or "."))
            if not inside(base, package.directory):
                refuse(m.start(), "%s!: %s lies outside its package" % (kind, shown(base)))
            continue
        target = verdict[1] if form == "absolute" else os.path.normpath(os.path.join(directory, verdict[1]))
        if kind == "include" and not target.endswith(".rs"):
            refuse(m.start(), "an include! of a file not ending in .rs")
            continue
        if inside(target, package.directory):
            continue
        if kind == "include":
            refuse(m.start(), "an include! of %s, outside its package" % shown(target))
            continue
        targets.append((shown(target), where(path, code, m.start())))


def where(path, code, i):
    return "%s:%d" % (shown(os.path.abspath(path)), code.count("\n", 0, i) + 1)


def run(arguments):
    packages = []
    i = 0
    while i < len(arguments):
        if arguments[i] == "--trusted" and i + 1 < len(arguments):
            packages.append(Package(arguments[i + 1], True))
            i += 2
        elif arguments[i].startswith("-"):
            print("usage: step_cache_includes.py [--trusted <dir>]... <dir>... | --self-test")
            return 2
        else:
            packages.append(Package(arguments[i], False))
            i += 1
    if not packages:
        print("usage: step_cache_includes.py [--trusted <dir>]... <dir>... | --self-test")
        return 2
    targets, problems = scan(packages)
    if problems:
        for problem in problems:
            print(problem)
        return 1
    for target, place in targets:
        print("%s\t%s" % (target, place))
    return 0


def write_files(root, files):
    for name, content in files.items():
        path = os.path.join(root, name)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as f:
            f.write(content)


def remove_tree(root):
    if not os.path.lexists(root):
        return
    for directory, names, files in os.walk(root, topdown=False):
        for name in files:
            os.unlink(os.path.join(directory, name))
        for name in names:
            path = os.path.join(directory, name)
            if os.path.islink(path):
                os.unlink(path)
            else:
                os.chmod(path, 0o700)
                os.rmdir(path)
    os.rmdir(root)


def self_test():
    """Synthetic packages under the repository's target/, one per case."""
    root = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "target",
                        "step-cache-includes-self-test.%d" % os.getpid())
    failures = []
    count = [0]

    def case(name, files, expect, trusted=False, extra=None):
        """`expect`: a list of targets (relative to the case's folder), or
        ("refused", text) where a refusal naming `text` is expected."""
        count[0] += 1
        folder = os.path.join(root, "case-%d" % count[0])
        write_files(folder, files)
        if extra:
            extra(folder)
        previous = os.getcwd()
        os.chdir(folder)
        try:
            targets, problems = scan([Package("pkg", trusted)])
        finally:
            os.chdir(previous)
        if isinstance(expect, tuple):
            ok = bool(problems) and any(expect[1] in p for p in problems)
            got = "; ".join(problems) or "accepted, targets %s" % [t for t, _ in targets]
        else:
            ok = not problems and sorted(t for t, _ in targets) == sorted(expect)
            got = "; ".join(problems) or "targets %s" % [t for t, _ in targets]
        if ok:
            print("  ok  %s" % name)
        else:
            print("include helper self-test: %s: expected %s, got %s" % (name, expect, got))
            failures.append(name)

    lib = "pkg/src/lib.rs"
    outside_md = {"docs/notes.md": "notes\n"}

    def with_docs(files):
        merged = dict(outside_md)
        merged.update(files)
        return merged

    remove_tree(root)
    os.makedirs(root)
    print("include helper self-test:")
    try:
        case("a literal include outside the package found",
             with_docs({lib: 'const N: &str = include_str!("../../docs/notes.md");\n'}), ["docs/notes.md"])
        case("an include in a line comment ignored",
             with_docs({lib: '// include_str!("../../docs/notes.md")\n'}), [])
        case("an include in a block comment ignored",
             with_docs({lib: '/* include_str!("../../docs/notes.md") */\n'}), [])
        case("an include in a doc comment ignored",
             with_docs({lib: '/// include!("../../docs/x.rs");\nfn f() {}\n'}), [])
        case("an include in a string literal ignored",
             with_docs({lib: 'const S: &str = "include_str!(\\"../../docs/notes.md\\")";\n'}), [])
        case("a concat! with .. refused",
             with_docs({lib: 'const N: &str = include_str!(concat!("../../docs/", "notes.md"));\n'}),
             ("refused", "holding .."))
        case("env!(\"CARGO_MANIFEST_DIR\") accepted",
             with_docs({lib: 'const N: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data.txt"));\n',
                        "pkg/data.txt": "x\n"}), [])
        case("env!(\"OUT_DIR\") accepted for a trusted package",
             {lib: 'include!(concat!(env!("OUT_DIR"), "/generated.rs"));\n'}, [], trusted=True)
        case("env!(\"OUT_DIR\") refused for another",
             {lib: 'include!(concat!(env!("OUT_DIR"), "/generated.rs"));\n'}, ("refused", "OUT_DIR"))
        macro = ('macro_rules! table {\n    ($($key:literal => $name:literal;)*) => {\n'
                 '        const ALL: &[&str] = &[$(include_str!(concat!("data/", $key, ".toml"))),*];\n'
                 '    };\n}\n\ntable! {\n    "one" => "One";\n    "two" => "Two";\n}\n')
        case("the metavariable form accepted",
             {lib: macro, "pkg/src/data/one.toml": "", "pkg/src/data/two.toml": ""}, [])
        case("the metavariable form, an unrelated \"/tmp/...\" literal elsewhere in the file, accepted",
             {lib: macro + 'const ELSEWHERE: &str = "/tmp/elsewhere";\n'}, [])
        case("the metavariable form refused where the macro is #[macro_export]",
             {lib: "#[macro_export]\n" + macro}, ("refused", "macro_export"))
        case("the metavariable form refused where the metavariable is $name:expr",
             {lib: macro.replace("$key:literal", "$key:expr")}, ("refused", "$key:literal"))
        case("the metavariable form refused where the macro is invoked from a second file",
             {lib: macro + "mod other;\n", "pkg/src/other.rs": 'table! { "three" => "Three"; }\n'},
             ("refused", "is named in"))
        case("the metavariable form refused where an invocation passes a literal with ..",
             {lib: macro.replace('"two" => "Two"', '"../two" => "Two"')}, ("refused", "holding .. or"))
        case("the metavariable form refused where an invocation passes a literal with \\",
             {lib: macro.replace('"two" => "Two"', '"t\\\\wo" => "Two"')}, ("refused", "holding .. or"))
        literal_macro = ('macro_rules! data {\n    () => {\n        include_str!("data.txt")\n    };\n}\n')
        case("a literal include in a macro body refused where the macro is invoked from a second file",
             {"pkg/src/a/mod.rs": literal_macro + "pub(crate) use data;\n",
              lib: "mod a;\nmod b;\n", "pkg/src/b/c/mod.rs": "fn f() -> &'static str { crate::a::data!() }\n"},
             ("refused", "is named in"))
        case("a literal include in a macro body refused where the macro is #[macro_export]",
             {lib: "#[cfg_attr(all(), macro_export)]\n" + literal_macro + "fn f() -> &'static str { data!() }\n"},
             ("refused", "macro_export"))
        case("a literal include in a macro body refused where the macro is invoked only from another macro's body",
             {lib: literal_macro + "macro_rules! outer {\n    () => { data!() };\n}\n"
              "fn f() -> &'static str { outer!() }\n"}, ("refused", "inside the body of macro_rules! outer"))
        case("a literal include in a macro body refused where the macro is re-exported by use ... as",
             {"pkg/src/a/mod.rs": literal_macro + "pub(crate) use data as other_name;\n",
              lib: "mod a;\nmod b;\n", "pkg/src/b/c/mod.rs": "fn f() -> &'static str { crate::a::other_name!() }\n"},
             ("refused", "use item"))
        case("a literal include in a macro body refused where a second file imports it by use ... as",
             {"pkg/src/a/mod.rs": literal_macro + "pub(crate) use data;\n",
              lib: "mod a;\nmod b;\n",
              "pkg/src/b/c/mod.rs": "use crate::a::data as other_name;\nfn f() -> &'static str { other_name!() }\n"},
             ("refused", "is named in"))
        case("a literal include in a macro body accepted where the macro is invoked only in its own file",
             {lib: literal_macro + "fn f() -> &'static str { data!() }\n", "pkg/src/data.txt": "x\n"}, [])
        case("a literal path with \\ refused",
             {lib: 'const N: &str = include_str!("data\\\\notes.md");\n'}, ("refused", "holding \\"))
        case("a #[path] to a .rs outside the package refused",
             {lib: '#[path = "../../shared/m.rs"]\nmod m;\n', "shared/m.rs": ""}, ("refused", "outside its package"))
        case("an include! of a .rs outside the package refused",
             {lib: 'include!("../../shared/m.rs");\n', "shared/m.rs": ""}, ("refused", "outside its package"))
        case("an include! of a .txt refused",
             {lib: 'include!("data.txt");\n', "pkg/src/data.txt": ""}, ("refused", "not ending in .rs"))
        case("#[cfg_attr(test, path = \"x.rs\")] refused",
             {lib: '#[cfg_attr(test, path = "x.rs")]\nmod m;\n', "pkg/src/x.rs": ""}, ("refused", "cfg_attr"))
        case("#[path] resolved against the including file's directory",
             {"pkg/src/sub/mod.rs": '#[path = "../../other/x.rs"]\nmod x;\n', lib: "mod sub;\n",
              "pkg/other/x.rs": 'const N: &str = include_str!("../../docs/notes.md");\n', "docs/notes.md": ""},
             ["docs/notes.md"])
        case("#[path] whose target lies outside the package refused",
             {"pkg/src/sub/mod.rs": '#[path = "../../../x.rs"]\nmod x;\n', lib: "mod sub;\n", "x.rs": ""},
             ("refused", "outside its package"))
        case("an ignored .rs file scanned",
             with_docs({lib: "", "pkg/.gitignore": "ignored.rs\n",
                        "pkg/src/ignored.rs": 'const N: &str = include_str!("../../docs/notes.md");\n'}),
             ["docs/notes.md"])

        def link(folder):
            os.symlink(os.path.join(folder, "elsewhere"), os.path.join(folder, "pkg", "src", "linked"))

        case("a .rs file reached through a symbolic link scanned",
             with_docs({lib: "", "elsewhere/m.rs": 'const N: &str = include_str!("../../../docs/notes.md");\n'}),
             ["docs/notes.md"], extra=link)

        unreadable = {"done": False}

        def lock(folder):
            path = os.path.join(folder, "pkg", "src", "locked.rs")
            os.chmod(path, 0)
            unreadable["done"] = not os.access(path, os.R_OK)

        probe_root = os.path.join(root, "probe")
        write_files(probe_root, {"probe.rs": ""})
        os.chmod(os.path.join(probe_root, "probe.rs"), 0)
        readable_anyway = os.access(os.path.join(probe_root, "probe.rs"), os.R_OK)
        os.chmod(os.path.join(probe_root, "probe.rs"), 0o600)
        if readable_anyway:
            print("  skip  an unreadable file fails: this user reads it all the same")
        else:
            case("an unreadable file fails", {lib: "", "pkg/src/locked.rs": ""}, ("refused", "cannot read"),
                 extra=lock)
            if not unreadable["done"]:
                failures.append("an unreadable file fails (the file stayed readable)")
            os.chmod(os.path.join(root, "case-%d" % count[0], "pkg", "src", "locked.rs"), 0o600)
    finally:
        remove_tree(root)
    if failures:
        print("include helper self-test: %d case(s) failed" % len(failures))
        return 1
    print("include helper self-test: every case passed")
    return 0


def main():
    if sys.argv[1:] == ["--self-test"]:
        return self_test()
    return run(sys.argv[1:])


if __name__ == "__main__":
    sys.exit(main())
