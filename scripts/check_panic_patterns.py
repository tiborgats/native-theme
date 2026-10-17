#!/usr/bin/env python3
"""Scan non-test Rust code for panic-prone patterns.

A regex tripwire: the authoritative panic check is the strict-panic clippy
run (scripts/check_strict_panic.sh), which is type-aware. This one also sees
what that run's lint list does not name, `Instant::now()` arithmetic, and code
behind a cfg the clippy runs do not build.

Run from the repository root, with the directories to scan (check_release.sh
passes every workspace member's `src/`, from `cargo metadata`):

    python3 scripts/check_panic_patterns.py native-theme/src ...
    python3 scripts/check_panic_patterns.py --self-test

Exit 0 when no pattern is found, 1 when one is or when an input cannot be
read: a directory that does not exist or holds no `.rs` file, a file that
cannot be opened or is not UTF-8. A file the scan cannot read is code it does
not check, so it fails rather than passing unread.

Patterns, outside test code
---------------------------
`.unwrap()`, `.expect(`, `panic!(`, `todo!(`, `unimplemented!(`,
`unreachable!(`, and `Instant::now()` followed by `+` or `-` (use
`checked_add`/`checked_sub`).

Test code
---------
`#[cfg(test)]` or `#[cfg(all(test, ..))]` covers the next item only, from the
attribute to the item's end: the brace that closes its first `{` outside
`(..)` and `[..]` (a module, a function, an impl), or the first `;` or `,`
outside them (`mod name;`, a `use`, a field, a variant, a match arm), or the
`}`, `)` or `]` that closes the list it sits in (a last field). A line is
skipped when all of its code lies in such a span; a line that also holds
other code is scanned, and an item the file never closes is scanned: where the
scan cannot tell, it reports rather than skips. A file named `tests.rs`, and a
file whose parent declares its module under `#[cfg(test)]`
(`#[cfg(test)] mod name;`), is test code and skipped whole.

What is code
------------
Items are bounded, and the patterns matched, on the code alone: line and
block comments (nested), string and byte-string literals, raw strings with any
number of `#`, and char and byte literals are blanked first, across lines. A
`{` in a test's TOML fixture or a `'{'` would otherwise leave the count inside
the test module and skip every item after it, and a pattern inside a string or
a comment is not a call.
"""

import argparse
import glob
import os
import re
import sys
import tempfile

CFG_TEST = re.compile(r"#\[cfg\((all\()?test[,\)]")
TEST_MOD_DECL = re.compile(r"\s*(pub(\(crate\))?\s+)?mod\s+(\w+)\s*;")
RAW_STRING = re.compile(r"(?:br|cr|r)(#*)\"")
IDENT_CHAR = re.compile(r"[A-Za-z0-9_]")

PATTERNS = (
    (re.compile(r"\.unwrap\(\)"), ".unwrap()"),
    (re.compile(r"\.expect\s*\("), ".expect()"),
    (re.compile(r"\bpanic!\s*\("), "panic!"),
    (re.compile(r"\btodo!\s*\("), "todo!"),
    (re.compile(r"\bunimplemented!\s*\("), "unimplemented!"),
    (re.compile(r"\bunreachable!\s*\("), "unreachable!"),
    (re.compile(r"Instant::now\(\)\s*[-+]"),
     "Instant::now() arithmetic; use checked_add/checked_sub"),
)


def code_only(text):
    """`text` with comments and literal contents blanked, its length and newlines kept.

    A literal keeps its delimiters (`"`, `'`, `r#"`), so `.expect("why")`
    still reads as a call; a comment becomes spaces.
    """
    out = list(text)
    n = len(text)
    i = 0

    def blank(start, end):
        for k in range(start, min(end, n)):
            if out[k] != "\n":
                out[k] = " "

    while i < n:
        c = text[i]
        if text.startswith("//", i):
            end = text.find("\n", i)
            end = n if end < 0 else end
            blank(i, end)
            i = end
        elif text.startswith("/*", i):
            depth = 0
            j = i
            while j < n:
                if text.startswith("/*", j):
                    depth += 1
                    j += 2
                elif text.startswith("*/", j):
                    depth -= 1
                    j += 2
                    if depth == 0:
                        break
                else:
                    j += 1
            blank(i, j)
            i = j
        elif c in "brc" and (i == 0 or not IDENT_CHAR.match(text[i - 1])) and RAW_STRING.match(text, i):
            m = RAW_STRING.match(text, i)
            close = '"' + m.group(1)
            end = text.find(close, m.end())
            end = n if end < 0 else end
            blank(m.end(), end)
            i = end + len(close)
        elif c == '"':
            j = i + 1
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            blank(i + 1, j)
            i = j + 1
        elif c == "'":
            if text.startswith("\\", i + 1):
                end = text.find("'", i + 3)
                end = n if end < 0 else end
                blank(i + 1, end)
                i = end + 1
            elif i + 2 < n and text[i + 2] == "'" and text[i + 1] != "\n":
                blank(i + 1, i + 2)
                i += 3
            else:
                i += 1  # a lifetime or a label
        else:
            i += 1
    return "".join(out)


def matching_brace(code, i):
    """The index of the `}` that closes the `{` at `i`, or None if the file ends first."""
    depth = 0
    for j in range(i, len(code)):
        if code[j] == "{":
            depth += 1
        elif code[j] == "}":
            depth -= 1
            if depth == 0:
                return j
    return None


def item_end(code, i):
    """The index of the last character of the item that starts at or after `i`.

    The item is the first `{…}` outside `(…)` and `[…]` (with a `;` or `,`
    right after it), or it ends at the first `;` or `,` outside them (a
    `mod name;`, a field, a variant, a match arm), or just before the `}`,
    `)` or `]` that closes the enclosing list (a last field). None when the
    file ends first.
    """
    depth = 0
    n = len(code)
    while i < n:
        c = code[i]
        if c in "([":
            depth += 1
        elif c in ")]":
            depth -= 1
            if depth < 0:
                return i - 1
        elif depth == 0:
            if c == "{":
                close = matching_brace(code, i)
                if close is None:
                    return None
                k = close + 1
                while k < n and code[k].isspace():
                    k += 1
                return k if k < n and code[k] in ";," else close
            if c in ";,":
                return i
            if c == "}":
                return i - 1
        i += 1
    return None


def test_item_spans(code):
    """The character spans `(start, end)` of the items under `#[cfg(test)]`.

    Each span runs from the attribute's `#` to the end of the next item
    (`item_end`): the attribute covers that item only. An attribute whose item
    has no end in the file covers nothing, so code the scan cannot bound is
    scanned, never skipped.
    """
    spans = []
    n = len(code)
    for m in CFG_TEST.finditer(code):
        depth = 0
        j = m.start() + 1
        while j < n:
            if code[j] == "[":
                depth += 1
            elif code[j] == "]":
                depth -= 1
                if depth == 0:
                    break
            j += 1
        if j >= n:
            continue
        end = item_end(code, j + 1)
        if end is not None:
            spans.append((m.start(), end))
    return spans


def scan_file(path, code, source):
    """`code` is the file's text through `code_only`, `source` its lines.

    A line is test code when every non-blank character of its code lies in a
    span `test_item_spans` gives. A line that also holds code outside every
    span is scanned.
    """
    covered = bytearray(len(code))
    for start, end in test_item_spans(code):
        covered[start:end + 1] = b"\x01" * (end + 1 - start)
    issues = []
    offset = 0
    for number, line in enumerate(code.split("\n"), 1):
        chars = [offset + k for k, ch in enumerate(line) if not ch.isspace()]
        offset += len(line) + 1
        if chars and all(covered[k] for k in chars):
            continue
        for pattern, tag in PATTERNS:
            if pattern.search(line):
                issues.append(f"{path}:{number}: [{tag}] {source[number - 1].strip()}")
                break
    return issues


def test_only_files(code_by_file):
    """Files their parent declares under `#[cfg(test)]` as `mod name;`."""
    skip = set()
    for path, code in code_by_file.items():
        lines = code.split("\n")
        base, name = os.path.split(path)
        child_dir = base if name in ("lib.rs", "main.rs", "mod.rs") else os.path.join(base, name[:-3])
        for i, line in enumerate(lines):
            if line.strip() != "#[cfg(test)]":
                continue
            following = next((x for x in lines[i + 1:] if x.strip()), "")
            m = TEST_MOD_DECL.match(following)
            if m:
                skip.add(os.path.join(child_dir, m.group(3) + ".rs"))
                skip.add(os.path.join(child_dir, m.group(3), "mod.rs"))
    return skip


def scan(dirs):
    """Return (findings, input errors) over every `.rs` file under `dirs`."""
    errors = []
    code_by_file = {}
    source_by_file = {}
    for directory in dirs:
        if not os.path.isdir(directory):
            errors.append(f"{directory}: no such directory")
            continue
        files = sorted(glob.glob(os.path.join(directory, "**", "*.rs"), recursive=True))
        if not files:
            errors.append(f"{directory}: no .rs file")
        for path in files:
            try:
                with open(path, encoding="utf-8") as f:
                    text = f.read()
            except (OSError, UnicodeDecodeError) as error:
                errors.append(f"{path}: cannot be read ({error})")
                continue
            code_by_file[path] = code_only(text)
            source_by_file[path] = text.split("\n")
    skip = test_only_files(code_by_file)
    findings = []
    for path, code in code_by_file.items():
        if path.endswith(os.sep + "tests.rs") or path in skip:
            continue
        findings.extend(scan_file(path, code, source_by_file[path]))
    return findings, errors


SELF_TEST_FILES = {
    # A `{` in a string inside the test module; the call after it is not test code.
    "a/src/string_brace.rs": (
        'fn ok() {}\n'
        '#[cfg(test)]\n'
        'mod tests {\n'
        '    const FIXTURE: &str = "[table] key = { a = 1";\n'
        '    #[test]\n'
        '    fn t() { Some(1).unwrap(); }\n'
        '}\n'
        'pub fn after() -> u8 { "1".parse().unwrap() }\n'
    ),
    # A raw string with `#`s holding a `{` and a `"`.
    "a/src/raw_string.rs": (
        '#[cfg(test)]\n'
        'mod tests {\n'
        '    const TOML: &str = r##"\n'
        '[a] b = { c = "{"\n'
        '"##;\n'
        '    fn t() { Some(1).unwrap(); }\n'
        '}\n'
        'pub fn after() -> u8 { "1".parse().unwrap() }\n'
    ),
    # A `{` in a line comment and in a nested block comment.
    "a/src/comment_brace.rs": (
        '#[cfg(test)]\n'
        'mod tests {\n'
        '    // a stray { in a comment\n'
        '    /* outer /* nested { */ still a comment { */\n'
        '    fn t() { Some(1).unwrap(); }\n'
        '}\n'
        'pub fn after() -> u8 { "1".parse().unwrap() }\n'
    ),
    # A `'{'` char literal; lifetimes are not literals.
    "a/src/char_brace.rs": (
        "#[cfg(test)]\n"
        "mod tests {\n"
        "    const OPEN: char = '{';\n"
        "    const B: u8 = b'{';\n"
        "    const QUOTE: char = '\"';\n"
        "    const ESCAPED: [char; 3] = ['\\'', '\\u{7B}', '\\\"'];\n"
        "    const S: &str = \"{\";\n"
        "    fn f<'a>(x: &'a str) -> &'a str { x }\n"
        "    fn t() { Some(1).unwrap(); }\n"
        "}\n"
        "pub fn after<'a>(x: &'a str) -> u8 { x.parse().unwrap() }\n"
    ),
    # `#[cfg(test)] mod name;` covers that item only, so the multi-line item
    # after it is scanned, and the module's file is skipped.
    "a/src/lib.rs": (
        '#[cfg(test)]\n'
        'mod helpers;\n'
        'pub fn after() -> u8 {\n'
        '    "1".parse().expect("a number")\n'
        '}\n'
    ),
    # Escaped quotes, a byte string and a raw byte string holding `{`.
    "a/src/escapes.rs": (
        '#[cfg(test)]\n'
        'mod tests {\n'
        '    const D: &str = "say \\"{\\" now";\n'
        '    const E: &str = "\\"{";\n'
        '    const F: &[u8] = b"{";\n'
        '    const G: &[u8] = br#"{""#;\n'
        '    const H: &str = stringify!(bar"\\"{");\n'
        '    fn t() { Some(1).unwrap(); }\n'
        '}\n'
        'pub fn after() -> u8 {\n'
        '    "1".parse().unwrap()\n'
        '}\n'
    ),
    # A `#[cfg(test)]` inside a test module: the outer module stays test code.
    "a/src/nested.rs": (
        '#[cfg(test)]\n'
        'mod tests {\n'
        '    #[cfg(test)]\n'
        '    mod inner {\n'
        '        fn i() { Some(1).unwrap(); }\n'
        '    }\n'
        '    fn t() { Some(2).unwrap(); }\n'
        '}\n'
        'pub fn after() -> u8 {\n'
        '    "1".parse().unwrap()\n'
        '}\n'
    ),
    # A one-line test item is skipped; a line that also holds other code is not.
    "a/src/one_line.rs": (
        '#[cfg(test)] fn helper() -> u8 { "1".parse().unwrap() }\n'
        '#[cfg(test)] fn h2() {} pub fn shared() -> u8 { "1".parse().unwrap() }\n'
        'pub fn after() -> u8 {\n'
        '    "2".parse().unwrap()\n'
        '}\n'
    ),
    # A `;` inside the signature's `[u8; 3]` or a call's parentheses does not
    # end the item.
    "a/src/signature.rs": (
        '#[cfg(test)]\n'
        'fn helper(\n'
        '    a: [u8; 3],\n'
        ') -> u8 {\n'
        '    a.first().copied().unwrap()\n'
        '}\n'
        '#[cfg(test)]\n'
        'static X: std::sync::LazyLock<u8> = std::sync::LazyLock::new(|| {\n'
        '    "1".parse().unwrap()\n'
        '});\n'
        'pub fn after() -> u8 {\n'
        '    "1".parse().unwrap()\n'
        '}\n'
    ),
    # On a field, a variant, a last field and a match arm the attribute covers
    # that one entry; the item after it is scanned.
    "a/src/entries.rs": (
        'pub struct S {\n'
        '    #[cfg(test)]\n'
        '    pub probe: u8,\n'
        '    pub last: u8,\n'
        '}\n'
        'impl S {\n'
        '    pub fn f(&self) -> u8 { "1".parse().unwrap() }\n'
        '}\n'
        'pub enum E {\n'
        '    A,\n'
        '    #[cfg(test)]\n'
        '    B,\n'
        '}\n'
        'pub fn g() -> u8 {\n'
        '    "1".parse().unwrap()\n'
        '}\n'
        'pub struct T {\n'
        '    #[cfg(test)]\n'
        '    pub last: u8\n'
        '}\n'
        'impl T {\n'
        '    pub fn t(&self) -> u8 { "1".parse().unwrap() }\n'
        '}\n'
        'pub fn m(x: u8) -> u8 {\n'
        '    match x {\n'
        '        #[cfg(test)]\n'
        '        0 => "0".parse().unwrap(),\n'
        '        _ => "1".parse().unwrap(),\n'
        '    }\n'
        '}\n'
    ),
    "a/src/helpers.rs": 'pub fn h() { Some(1).unwrap(); }\n',
    # A test item the file never closes cannot be bounded, so it is scanned.
    "a/src/unbounded.rs": (
        '#[cfg(test)]\n'
        'fn broken() -> u8 {\n'
        '    "1".parse().unwrap()\n'
    ),
    # A pattern inside a string or a comment is not a call.
    "a/src/not_calls.rs": (
        'pub const DOC: &str = "call .unwrap() and panic!(now)";\n'
        '// .expect("x") in a comment\n'
        'pub fn f() {}\n'
    ),
}

# (file, line, tag) the scan must report, and nothing else.
SELF_TEST_EXPECTED = {
    ("a/src/string_brace.rs", 8, ".unwrap()"),
    ("a/src/raw_string.rs", 8, ".unwrap()"),
    ("a/src/comment_brace.rs", 7, ".unwrap()"),
    ("a/src/char_brace.rs", 11, ".unwrap()"),
    ("a/src/lib.rs", 4, ".expect()"),
    ("a/src/escapes.rs", 11, ".unwrap()"),
    ("a/src/nested.rs", 10, ".unwrap()"),
    ("a/src/one_line.rs", 2, ".unwrap()"),
    ("a/src/one_line.rs", 4, ".unwrap()"),
    ("a/src/signature.rs", 12, ".unwrap()"),
    ("a/src/entries.rs", 7, ".unwrap()"),
    ("a/src/entries.rs", 15, ".unwrap()"),
    ("a/src/entries.rs", 22, ".unwrap()"),
    ("a/src/entries.rs", 28, ".unwrap()"),
    ("a/src/unbounded.rs", 3, ".unwrap()"),
}


def write_tree(root, files):
    for rel, text in files.items():
        path = os.path.join(root, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as f:
            f.write(text)


def self_test():
    failures = []
    with tempfile.TemporaryDirectory() as root:
        write_tree(root, SELF_TEST_FILES)
        findings, errors = scan([os.path.join(root, "a", "src")])
        got = set()
        for finding in findings:
            m = re.match(r"(.+?):(\d+): \[([^\]]+)\]", finding)
            if m:
                got.add((os.path.relpath(m.group(1), root), int(m.group(2)), m.group(3)))
        for missing in sorted(SELF_TEST_EXPECTED - got):
            failures.append(f"not reported: {missing}")
        for extra in sorted(got - SELF_TEST_EXPECTED):
            failures.append(f"reported, not expected: {extra}")
        if errors:
            failures.append(f"input errors on the clean tree: {errors}")

        # an input the scan cannot read is a failure, not a skip
        with open(os.path.join(root, "a", "src", "latin1.rs"), "wb") as f:
            f.write(b"// caf\xe9\n")
        os.makedirs(os.path.join(root, "empty"))
        _, errors = scan([os.path.join(root, "a", "src"), os.path.join(root, "missing"),
                          os.path.join(root, "empty")])
        for label, needle in (("a non-UTF-8 file", "latin1.rs: cannot be read"),
                              ("a missing directory", "missing: no such directory"),
                              ("a directory with no .rs file", "empty: no .rs file")):
            if not any(needle in e for e in errors):
                failures.append(f"{label} is not reported: {errors}")

    for line in failures:
        print("  FAIL " + line)
    if failures:
        print(f"self-test: {len(failures)} failure(s)")
        return 1
    print("self-test: every scenario passes")
    return 0


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("dirs", nargs="*", help="directories to scan")
    parser.add_argument("--self-test", action="store_true",
                        help="run the built-in scenarios instead of the scan")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    if not args.dirs:
        parser.error("no directory to scan")
    findings, errors = scan(args.dirs)
    for line in errors + findings:
        print(line)
    return 1 if errors or findings else 0


if __name__ == "__main__":
    sys.exit(main())
