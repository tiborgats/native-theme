//! Layer 2 of the theme contracts: the showcase exercises every builder
//! (spec §6a.4).
//!
//! Test-only. A builder nobody demonstrates is a builder nobody has looked at,
//! which is how `geometry::dialog` carried a wrong radius and
//! `geometry::menu_item` a wrong doc comment for two releases. The showcase's
//! files are read with `include_str!` -- paths known at compile time, so no
//! test has to find a file at run time -- and so are the two modules whose
//! surface it must cover, because a list written out here would be one more
//! thing to forget: the names come from the `pub fn` lines of `geometry.rs`
//! and `variants.rs` themselves. The list of showcase files *is* written out,
//! because `include_str!` needs a literal; `the_gates_read_every_showcase_file`
//! holds it to the directory.
//!
//! What counts as exercising a builder is a reference to it by path in code,
//! with or without an argument list: the showcase passes several of them as
//! function items (`.native(cx, geometry::button)`), which is a call at one
//! remove. Comments and string literals are removed first, because the
//! showcase names builders in both -- every widget's Widget Info says which
//! builder shaped it -- and a note about a builder is not a use of it. That is
//! the opposite of what `scripts/check-widget-coverage.py` does with the same
//! files, and for the opposite reason: a widget is often reached through an
//! extension method and its name appears only in the section label, while a
//! builder is always called by path.

/// Every `.rs` file of the showcase, as (path under
/// `examples/showcase-gpui/`, contents). Each gate reads them one at a time,
/// so a line number it reports is a line number in the file it names.
const SHOWCASE_FILES: &[(&str, &str)] = &[
    ("app.rs", include_str!("../examples/showcase-gpui/app.rs")),
    (
        "chrome.rs",
        include_str!("../examples/showcase-gpui/chrome.rs"),
    ),
    ("demo.rs", include_str!("../examples/showcase-gpui/demo.rs")),
    (
        "info/buttons.rs",
        include_str!("../examples/showcase-gpui/info/buttons.rs"),
    ),
    (
        "info/charts.rs",
        include_str!("../examples/showcase-gpui/info/charts.rs"),
    ),
    (
        "info/chrome.rs",
        include_str!("../examples/showcase-gpui/info/chrome.rs"),
    ),
    (
        "info/data.rs",
        include_str!("../examples/showcase-gpui/info/data.rs"),
    ),
    (
        "info/feedback.rs",
        include_str!("../examples/showcase-gpui/info/feedback.rs"),
    ),
    (
        "info/icons.rs",
        include_str!("../examples/showcase-gpui/info/icons.rs"),
    ),
    (
        "info/inputs.rs",
        include_str!("../examples/showcase-gpui/info/inputs.rs"),
    ),
    (
        "info/layout.rs",
        include_str!("../examples/showcase-gpui/info/layout.rs"),
    ),
    (
        "info/mod.rs",
        include_str!("../examples/showcase-gpui/info/mod.rs"),
    ),
    (
        "info/overlays.rs",
        include_str!("../examples/showcase-gpui/info/overlays.rs"),
    ),
    (
        "info/registry.rs",
        include_str!("../examples/showcase-gpui/info/registry.rs"),
    ),
    (
        "info/text.rs",
        include_str!("../examples/showcase-gpui/info/text.rs"),
    ),
    (
        "info/theme_map.rs",
        include_str!("../examples/showcase-gpui/info/theme_map.rs"),
    ),
    (
        "info/typography.rs",
        include_str!("../examples/showcase-gpui/info/typography.rs"),
    ),
    (
        "inspector.rs",
        include_str!("../examples/showcase-gpui/inspector.rs"),
    ),
    ("main.rs", include_str!("../examples/showcase-gpui/main.rs")),
    (
        "pages/buttons.rs",
        include_str!("../examples/showcase-gpui/pages/buttons.rs"),
    ),
    (
        "pages/charts.rs",
        include_str!("../examples/showcase-gpui/pages/charts.rs"),
    ),
    (
        "pages/data.rs",
        include_str!("../examples/showcase-gpui/pages/data.rs"),
    ),
    (
        "pages/feedback.rs",
        include_str!("../examples/showcase-gpui/pages/feedback.rs"),
    ),
    (
        "pages/icons.rs",
        include_str!("../examples/showcase-gpui/pages/icons.rs"),
    ),
    (
        "pages/inputs.rs",
        include_str!("../examples/showcase-gpui/pages/inputs.rs"),
    ),
    (
        "pages/layout.rs",
        include_str!("../examples/showcase-gpui/pages/layout.rs"),
    ),
    (
        "pages/mod.rs",
        include_str!("../examples/showcase-gpui/pages/mod.rs"),
    ),
    (
        "pages/overlays.rs",
        include_str!("../examples/showcase-gpui/pages/overlays.rs"),
    ),
    (
        "pages/theme_map.rs",
        include_str!("../examples/showcase-gpui/pages/theme_map.rs"),
    ),
    (
        "pages/typography.rs",
        include_str!("../examples/showcase-gpui/pages/typography.rs"),
    ),
    (
        "support.rs",
        include_str!("../examples/showcase-gpui/support.rs"),
    ),
    (
        "tests.rs",
        include_str!("../examples/showcase-gpui/tests.rs"),
    ),
];

/// The showcase's own test module.
///
/// Everything in it is test code: it builds widgets and calls builders for
/// reasons that have nothing to do with a demo, so the gates that measure
/// what the showcase shows its reader leave it out. The gates that ask what
/// the showcase does *at all* -- which builders it calls, which colours it
/// paints -- read it too.
const TEST_MODULE: &str = "tests.rs";

/// The showcase files that hold demos: every one but the test module.
fn demo_files() -> impl Iterator<Item = (&'static str, &'static str)> {
    SHOWCASE_FILES
        .iter()
        .copied()
        .filter(|(path, _)| *path != TEST_MODULE)
}

/// The two modules whose public surface the showcase must cover.
const GEOMETRY: &str = include_str!("geometry.rs");
const VARIANTS: &str = include_str!("variants.rs");

/// Every `.rs` file of the example is one the gates read.
#[test]
fn the_gates_read_every_showcase_file() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/showcase-gpui");
    let mut on_disk = Vec::new();
    collect_rs(&dir, &dir, &mut on_disk);
    on_disk.sort();
    let mut listed: Vec<String> = SHOWCASE_FILES
        .iter()
        .map(|(p, _)| (*p).to_string())
        .collect();
    listed.sort();
    assert!(
        !on_disk.is_empty(),
        "no showcase files found under {}",
        dir.display()
    );
    assert_eq!(
        listed, on_disk,
        "SHOWCASE_FILES and the example directory disagree"
    );
}

/// Every `.rs` file under `dir`, as a path relative to `root` written with
/// `/`, appended to `out`.
fn collect_rs(root: &std::path::Path, dir: &std::path::Path, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs(root, &path, out);
        } else if path.extension().is_some_and(|e| e == "rs")
            && let Ok(rel) = path.strip_prefix(root)
        {
            out.push(rel.to_string_lossy().replace('\\', "/"));
        }
    }
}

/// The names of a module's top-level `pub fn` items, in source order.
///
/// Only column-zero declarations, which is every public builder: one nested
/// inside a function or a test module is not part of the module's surface.
fn public_fns(source: &str) -> Vec<&str> {
    source
        .lines()
        .filter_map(|line| line.strip_prefix("pub fn "))
        .map(|rest| {
            let end = match rest.find(|c: char| !c.is_alphanumeric() && c != '_') {
                Some(ix) => ix,
                None => rest.len(),
            };
            &rest[..end]
        })
        .collect()
}

/// `source` with its `//` line comments and its string literals removed, the
/// code around them untouched and its line breaks kept.
///
/// Scanning has to know where a string begins and ends either way: the
/// showcase holds URLs and a raw string of Rust source with doc comments in
/// it, and a `//` inside either is not a comment -- dropping the rest of those
/// lines would hide the code that follows them.
///
/// A removed span leaves its newlines behind, so a line number in the result
/// is a line number in the file: `the_showcase_hardcodes_no_style_values`
/// names the line it found something on, and the showcase holds a multi-line
/// raw string.
fn without_comments_or_strings(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut rest = source;
    while let Some(next) = rest.find(['/', '"', 'r', '\'']) {
        let (before, from) = rest.split_at(next);
        out.push_str(before);
        if let Some(after) = from.strip_prefix("//") {
            rest = match after.find('\n') {
                Some(ix) => &after[ix..],
                None => "",
            };
        } else if let Some(len) = char_literal_len(from) {
            // Kept as written: a char literal is code, and `'"'` or `'/'`
            // would otherwise open a string or a comment that never closes.
            out.push_str(&from[..len]);
            rest = &from[len..];
        } else if let Some(body) = from.strip_prefix('"') {
            let end = end_of_string(body, "\"");
            push_newlines_of(&mut out, &body[..end]);
            rest = &body[end..];
        } else if let Some(hashes) = raw_string_hashes(from) {
            let open = hashes + 2;
            let close = format!("\"{}", "#".repeat(hashes));
            let end = match from[open..].find(&close) {
                Some(ix) => open + ix + close.len(),
                None => from.len(),
            };
            push_newlines_of(&mut out, &from[..end]);
            rest = &from[end..];
        } else {
            let end = char_len(from);
            out.push_str(&from[..end]);
            rest = &from[end..];
        }
    }
    out.push_str(rest);
    out
}

/// Keep the line breaks of a span that is being removed, so what follows it
/// stays on the line it is written on.
fn push_newlines_of(out: &mut String, removed: &str) {
    out.extend(removed.chars().filter(|c| *c == '\n'));
}

/// The number of `#`s of a raw string starting at `from`, or `None` if `from`
/// does not start one.
fn raw_string_hashes(from: &str) -> Option<usize> {
    let after_r = from.strip_prefix('r')?;
    let hashes = after_r.len() - after_r.trim_start_matches('#').len();
    after_r[hashes..].starts_with('"').then_some(hashes)
}

/// The byte length of the char literal starting at `from`, or `None` when the
/// `'` opens a lifetime instead.
///
/// `'a`, `'_` and `'static` are lifetimes and must fall through untouched;
/// `'"'` and `'/'` are literals whose contents would otherwise be read as the
/// start of a string or a comment. `scripts/check-widget-coverage.py` draws
/// the same distinction, and the same way.
fn char_literal_len(from: &str) -> Option<usize> {
    let body = from.strip_prefix('\'')?;
    if let Some(escaped) = body.strip_prefix('\\') {
        // `'\n'`, `'\''`, `'\u{2026}'`: past the escape's own first character,
        // then to the closing quote.
        let skip = char_len(escaped);
        let end = escaped[skip..].find('\'')?;
        return Some("'\\".len() + skip + end + "'".len());
    }
    let first = body.chars().next()?;
    body[first.len_utf8()..]
        .starts_with('\'')
        .then_some("'".len() + first.len_utf8() + "'".len())
}

/// The byte length of the first character of `s`, or 0 when it is empty.
fn char_len(s: &str) -> usize {
    match s.chars().next() {
        Some(c) => c.len_utf8(),
        None => 0,
    }
}

/// The offset just past the closing `delim` of a string body, escapes honoured.
fn end_of_string(body: &str, delim: &str) -> usize {
    let mut ix = 0;
    while ix < body.len() {
        let tail = &body[ix..];
        if tail.starts_with('\\') {
            ix += tail.chars().take(2).map(char::len_utf8).sum::<usize>();
        } else if tail.starts_with(delim) {
            return ix + delim.len();
        } else {
            ix += char_len(tail);
        }
    }
    ix
}

/// Whether `haystack` references `module::name` as a path of its own.
///
/// Both ends are checked. After the name, so that `geometry::input` is not
/// satisfied by `geometry::input_height`; before the module, so that it is not
/// satisfied by `my_geometry::input` either. A `:` before the module is fine
/// and deliberate: `native_theme_gpui::geometry::input` is the same builder,
/// written out.
fn references(haystack: &str, module: &str, name: &str) -> bool {
    let path = format!("{module}::{name}");
    haystack.match_indices(&path).any(|(ix, _)| {
        let before_is_boundary = haystack[..ix]
            .chars()
            .next_back()
            .is_none_or(|c| !c.is_alphanumeric() && c != '_');
        let after_is_boundary = haystack[ix + path.len()..]
            .chars()
            .next()
            .is_none_or(|c| !c.is_alphanumeric() && c != '_');
        before_is_boundary && after_is_boundary
    })
}

/// Spec §6a.4: every public `geometry` and `variants` function is exercised by
/// the showcase.
#[test]
fn the_showcase_exercises_every_builder() {
    let sources: Vec<String> = SHOWCASE_FILES
        .iter()
        .map(|(_, text)| without_comments_or_strings(text))
        .collect();
    let mut missing = Vec::new();
    let mut checked = 0usize;
    for (module, declared) in [
        ("geometry", public_fns(GEOMETRY)),
        ("variants", public_fns(VARIANTS)),
    ] {
        assert!(
            !declared.is_empty(),
            "no `pub fn` found in {module}.rs, so this test would pass vacuously"
        );
        for name in declared {
            checked += 1;
            if !sources
                .iter()
                .any(|source| references(source, module, name))
            {
                missing.push(format!("{module}::{name}"));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "{} of {checked} builders are never used in examples/showcase-gpui/, \
         so nothing demonstrates them: {}",
        missing.len(),
        missing.join(", ")
    );
}

// ---------------------------------------------------------------------------
// Geometry lines come from one table
// ---------------------------------------------------------------------------
//
// Showcase spec §3.3 and §5.2. A widget's "geometry" line is not written by
// hand: `WidgetInfo::geometry` takes it from `GEOMETRY_NOTES` in
// `info/mod.rs`, which says what each builder carries, and `native_info`
// applies a builder and records it in one call. Two things can still drift.
// The table can fall behind geometry.rs -- a builder added there has no line,
// one renamed or removed leaves a line about nothing -- and a `native_info`
// call takes the builder it applies and the name it records as two separate
// arguments, which nothing but this ties together.

/// The showcase file that holds `GEOMETRY_NOTES`.
const GEOMETRY_NOTES_FILE: &str = "info/mod.rs";

/// The table's declaration, up to the `[` its entries follow.
const GEOMETRY_NOTES_OPEN: &str = "pub const GEOMETRY_NOTES: &[(&str, &str)] = &[";

/// The byte offset of `inner` in `outer`, which it is a slice of.
fn offset_in(outer: &str, inner: &str) -> usize {
    (inner.as_ptr() as usize).saturating_sub(outer.as_ptr() as usize)
}

/// The text of the file that holds `GEOMETRY_NOTES`, or nothing if
/// `SHOWCASE_FILES` has lost it -- which `geometry_note_names` reports.
fn geometry_notes_source() -> &'static str {
    SHOWCASE_FILES
        .iter()
        .find(|(path, _)| *path == GEOMETRY_NOTES_FILE)
        .map_or("", |(_, text)| *text)
}

/// The builder each `GEOMETRY_NOTES` entry names, with the line it is on, and
/// a line for every part of the table that could not be read.
fn geometry_note_names(info: &str) -> (Vec<(usize, &str)>, Vec<String>) {
    let (entries, unreadable) = geometry_note_entries(info);
    let names = entries
        .into_iter()
        .map(|(line, name, _)| (line, name))
        .collect();
    (names, unreadable)
}

/// Every `GEOMETRY_NOTES` entry as the line it is on, the builder it names and
/// its text as written, and a line for every part of the table that could not
/// be read.
///
/// An entry is read as a tuple whose first element is a string literal,
/// wherever rustfmt breaks its lines; anything else in the array is reported,
/// so an entry the parse cannot see is never silently missing. The text is
/// `""` for an entry that has none, which leaves nothing to cite.
fn geometry_note_entries(info: &str) -> (Vec<(usize, &str, &str)>, Vec<String>) {
    let starts = line_offsets(info);
    let mut entries = Vec::new();
    let mut unreadable = Vec::new();
    let Some(open) = info.find(GEOMETRY_NOTES_OPEN) else {
        unreadable.push(format!(
            "{GEOMETRY_NOTES_FILE}: no `{GEOMETRY_NOTES_OPEN}` found"
        ));
        return (entries, unreadable);
    };
    let bracket = open + GEOMETRY_NOTES_OPEN.len() - 1;
    let Some(table) = call_args(info, bracket) else {
        unreadable.push(format!(
            "{GEOMETRY_NOTES_FILE}:{}: the table never closes",
            line_at(&starts, bracket)
        ));
        return (entries, unreadable);
    };
    for entry in table {
        let at = offset_in(info, entry) + (entry.len() - entry.trim_start().len());
        let line = line_at(&starts, at);
        let args = info
            .get(at..)
            .filter(|rest| rest.starts_with('('))
            .and_then(|_| call_args(info, at));
        let name = args
            .as_ref()
            .and_then(|args| args.first().copied())
            .and_then(unquote);
        match name {
            Some(name) => {
                let text = args
                    .as_ref()
                    .and_then(|args| args.get(1))
                    .map_or("", |text| text.trim());
                entries.push((line, name, text));
            }
            None => unreadable.push(format!(
                "{GEOMETRY_NOTES_FILE}:{line}: not a (\"builder\", \"what\") entry: {}",
                entry.trim()
            )),
        }
    }
    (entries, unreadable)
}

/// The offset of the `(` of every call of the function `name` in `raw` that
/// is code: not inside a comment or a string, not the tail of a longer
/// identifier, and not the function's own declaration.
fn code_calls(raw: &str, name: &str) -> Vec<usize> {
    let mut out = Vec::new();
    let mut at = 0usize;
    while at < raw.len() {
        let Some(rest) = raw.get(at..) else {
            break;
        };
        if let Some(after) = rest.strip_prefix("//") {
            at += 2 + after.find('\n').unwrap_or(after.len());
            continue;
        }
        if let Some(len) = char_literal_len(rest) {
            at += len;
            continue;
        }
        if let Some(hashes) = raw_string_hashes(rest) {
            let close = format!("\"{}", "#".repeat(hashes));
            let from = hashes + 2;
            at += match rest.get(from..).and_then(|t| t.find(&close)) {
                Some(ix) => from + ix + close.len(),
                None => rest.len(),
            };
            continue;
        }
        if let Some(body) = rest.strip_prefix('"') {
            at += 1 + end_of_string(body, "\"");
            continue;
        }
        let before = raw.get(..at).unwrap_or("");
        let starts_a_name = before
            .chars()
            .next_back()
            .is_none_or(|c| !c.is_alphanumeric() && c != '_');
        if starts_a_name
            && rest
                .strip_prefix(name)
                .is_some_and(|after| after.starts_with('('))
            && !before.trim_end().ends_with("fn")
        {
            out.push(at + name.len());
            at += name.len();
            continue;
        }
        at += char_len(rest);
    }
    out
}

/// Spec §3.3: `GEOMETRY_NOTES` has one entry for every `pub fn` of
/// geometry.rs and none for anything else -- the value builders as well as
/// the refinements, because an icon size or a gap is recorded with
/// `.geometry(name)` too.
#[test]
fn every_geometry_builder_has_a_note() {
    let builders = public_fns(GEOMETRY);
    assert!(
        !builders.is_empty(),
        "no `pub fn` found in geometry.rs, so this test would pass vacuously"
    );
    let (noted, mut findings) = geometry_note_names(geometry_notes_source());

    for builder in &builders {
        if !noted.iter().any(|(_, name)| name == builder) {
            let at = GEOMETRY
                .lines()
                .position(|l| {
                    l.strip_prefix("pub fn ")
                        .and_then(|rest| rest.strip_prefix(builder))
                        .is_some_and(|rest| rest.starts_with(['(', '<']))
                })
                .map_or_else(
                    || "geometry.rs".to_string(),
                    |ix| format!("geometry.rs:{}", ix + 1),
                );
            findings.push(format!(
                "{at}: geometry::{builder} has no GEOMETRY_NOTES entry"
            ));
        }
    }
    for (ix, (line, name)) in noted.iter().enumerate() {
        if !builders.contains(name) {
            findings.push(format!(
                "{GEOMETRY_NOTES_FILE}:{line}: an entry for geometry::{name}, which \
                 geometry.rs does not declare"
            ));
        } else if noted.iter().take(ix).any(|(_, earlier)| earlier == name) {
            findings.push(format!(
                "{GEOMETRY_NOTES_FILE}:{line}: a second entry for geometry::{name}"
            ));
        }
    }

    assert!(
        findings.is_empty(),
        "GEOMETRY_NOTES and geometry.rs disagree, so a widget's geometry line \
         would describe the wrong builder or none:\n  {}",
        findings.join("\n  ")
    );
}

/// Spec §5.2: every `native_info(w, cx, geometry::X, "Y", info)` records the
/// builder it applies, `X == Y`.
#[test]
fn native_info_names_the_builder_it_applies() {
    let mut findings = Vec::new();
    let mut calls = 0usize;
    for (file, raw) in SHOWCASE_FILES {
        let starts = line_offsets(raw);
        for open in code_calls(raw, "native_info") {
            calls += usize::from(*file != TEST_MODULE);
            let args = call_args(raw, open).unwrap_or_default();
            let applied = args.get(2).map(|a| a.trim());
            let recorded = args.get(3).map(|a| a.trim());
            let builder = applied
                .and_then(|a| a.rsplit_once("geometry::"))
                .map(|(_, builder)| builder);
            if builder.is_none() || builder != recorded.and_then(unquote) {
                findings.push(format!(
                    "{file}:{}: applies {} but records {}",
                    line_at(&starts, open),
                    applied.unwrap_or("?"),
                    recorded.unwrap_or("?")
                ));
            }
        }
    }
    assert!(
        calls > 0,
        "no native_info call found outside the test module, so this test would pass \
         vacuously for every widget the showcase shows"
    );
    assert!(
        findings.is_empty(),
        "a native_info call records a builder other than the one it applies, so \
         its geometry line describes a refinement the widget did not take:\n  {}",
        findings.join("\n  ")
    );
}

/// Every `.geometry(` method call in `raw` that is code, as the line it is on
/// and its argument when that is a single string literal.
///
/// A method call only: the path `geometry::x` and the declaration
/// `fn geometry(` are not calls of it. A call whose argument is not a literal
/// yields `None`; the one such call is `native_info`'s own `.geometry(name)`,
/// whose `name` `native_info_names_the_builder_it_applies` holds instead.
fn geometry_line_calls(raw: &str) -> Vec<(usize, Option<&str>)> {
    let starts = line_offsets(raw);
    method_calls(raw, "geometry")
        .into_iter()
        .map(|open| {
            let literal = call_args(raw, open)
                .filter(|args| args.len() == 1)
                .and_then(|args| args.first().copied())
                .and_then(unquote);
            (line_at(&starts, open), literal)
        })
        .collect()
}

/// Spec §3.3: a builder named straight to `.geometry("x")` -- the way a value
/// builder such as `icon_size_small` or `widget_gap` is recorded, since only a
/// refinement can go through `native_info` -- names a `GEOMETRY_NOTES` entry,
/// so "(no GEOMETRY_NOTES entry)" never reaches the inspector.
///
/// The test module is left out: it names a builder that does not exist on
/// purpose, to see that line.
#[test]
fn every_recorded_builder_has_a_note() {
    let (noted, unreadable) = geometry_note_names(geometry_notes_source());
    assert!(
        unreadable.is_empty() && !noted.is_empty(),
        "GEOMETRY_NOTES could not be read, so no recorded builder can be checked \
         against it:\n  {}",
        unreadable.join("\n  ")
    );
    let mut findings = Vec::new();
    let mut recorded = 0usize;
    for (file, raw) in demo_files() {
        for (line, literal) in geometry_line_calls(raw) {
            recorded += usize::from(literal.is_some());
            if let Some(name) = literal
                && !noted.iter().any(|(_, noted)| *noted == name)
            {
                findings.push(format!(
                    "{file}:{line}: .geometry(\"{name}\") names no GEOMETRY_NOTES entry"
                ));
            }
        }
    }
    assert!(
        recorded > 0,
        "no `.geometry(\"x\")` call found outside the test module, so this test \
         would pass vacuously"
    );
    assert!(
        findings.is_empty(),
        "a widget records a builder GEOMETRY_NOTES does not describe, so its \
         geometry line reads \"(no GEOMETRY_NOTES entry)\":\n  {}",
        findings.join("\n  ")
    );
}

/// The parsers above have to see what they are meant to: a table entry in
/// either of rustfmt's layouts, and a call that is code but not one that is
/// text, a comment, a longer name, a path or the declaration.
#[test]
fn the_geometry_note_parsers_do_their_jobs() {
    let table = "pub const GEOMETRY_NOTES: &[(&str, &str)] = &[\n\
                 \x20   (\"button\", \"a, b (c)\"),\n\
                 \x20   (\n\
                 \x20       \"input\",\n\
                 \x20       \"d\",\n\
                 \x20   ),\n\
                 \x20   \"stray\",\n\
                 ];\n";
    let (names, unreadable) = geometry_note_names(table);
    assert_eq!(names, vec![(2, "button"), (3, "input")]);
    assert_eq!(
        unreadable,
        vec!["info/mod.rs:7: not a (\"builder\", \"what\") entry: \"stray\"".to_string()]
    );

    let source = "pub fn native_info<W>(w: W) -> W { w }\n\
                  fn native_info(w: W) -> W { w }\n\
                  // native_info(w, cx, geometry::button, \"input\", info)\n\
                  let s = \"native_info(w)\";\n\
                  my_native_info(w);\n\
                  let w = native_info(w, cx, geometry::button, \"button\", &mut info);\n";
    let calls = code_calls(source, "native_info");
    let starts = line_offsets(source);
    let lines: Vec<usize> = calls.iter().map(|&at| line_at(&starts, at)).collect();
    assert_eq!(lines, vec![6]);

    let recorded = "pub fn geometry(self, builder: &'static str) -> Self {\n\
                    \x20   *info = std::mem::take(info).geometry(name);\n\
                    }\n\
                    // info.geometry(\"commented\")\n\
                    let s = \".geometry(\\\"quoted\\\")\";\n\
                    let r = geometry::button(n);\n\
                    let a = WidgetInfo::new(\"Icon\").geometry(\"icon_size_small\");\n\
                    let b = WidgetInfo::new(\"Icon\")\n\
                    \x20   .geometry(\n\
                    \x20       \"icon_size_smal\",\n\
                    \x20   );\n";
    assert_eq!(
        geometry_line_calls(recorded),
        vec![
            (2, None),
            (7, Some("icon_size_small")),
            (9, Some("icon_size_smal"))
        ]
    );
}

// ---------------------------------------------------------------------------
// Shared scanning helpers
// ---------------------------------------------------------------------------

/// The byte offset at which each line of `text` begins.
fn line_offsets(text: &str) -> Vec<usize> {
    let mut out = vec![0usize];
    for (ix, ch) in text.char_indices() {
        if ch == '\n' {
            out.push(ix + 1);
        }
    }
    out
}

/// The offset just past the `)` that closes the call whose `(` is at `open`.
///
/// `code` has had its strings and comments removed, so the only parentheses
/// left are code -- except inside a char literal (`')'` is one in this
/// showcase), which is stepped over rather than counted.
fn end_of_call(code: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut at = open;
    while at < code.len() {
        let rest = code.get(at..)?;
        if let Some(len) = char_literal_len(rest) {
            at += len;
            continue;
        }
        match rest.chars().next() {
            Some('(') => depth += 1,
            Some(')') => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(at + 1);
                }
            }
            _ => {}
        }
        at += char_len(rest);
    }
    None
}

/// The leading identifier of `s`, or `""`.
fn leading_ident(s: &str) -> &str {
    let end = s
        .find(|c: char| !(c.is_alphanumeric() || c == '_'))
        .unwrap_or(s.len());
    s.get(..end).unwrap_or("")
}

/// Whether `haystack` uses `ident` as an identifier of its own.
fn mentions(haystack: &str, ident: &str) -> bool {
    if ident.is_empty() {
        return false;
    }
    let mut from = 0usize;
    while let Some(ix) = haystack.get(from..).and_then(|t| t.find(ident)) {
        let at = from + ix;
        let before_ok = haystack
            .get(..at)
            .and_then(|t| t.chars().next_back())
            .is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
        let after_ok = haystack
            .get(at + ident.len()..)
            .and_then(|t| t.chars().next())
            .is_none_or(|c| !(c.is_alphanumeric() || c == '_'));
        if before_ok && after_ok {
            return true;
        }
        from = at + ident.len();
    }
    false
}

/// `raw` with every comment, string literal and char literal blanked to
/// spaces, its length and line breaks kept: an offset into the result is an
/// offset into `raw`, and whatever a search finds in it is code.
fn blanked(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut at = 0usize;
    while let Some(rest) = raw.get(at..).filter(|rest| !rest.is_empty()) {
        let len = if let Some(after) = rest.strip_prefix("//") {
            2 + after.find('\n').unwrap_or(after.len())
        } else if let Some(len) = char_literal_len(rest) {
            len
        } else if let Some(hashes) = raw_string_hashes(rest) {
            let close = format!("\"{}", "#".repeat(hashes));
            let from = hashes + 2;
            match rest.get(from..).and_then(|t| t.find(&close)) {
                Some(ix) => from + ix + close.len(),
                None => rest.len(),
            }
        } else if let Some(body) = rest.strip_prefix('"') {
            1 + end_of_string(body, "\"")
        } else {
            let len = char_len(rest);
            out.push_str(rest.get(..len).unwrap_or(""));
            at += len;
            continue;
        };
        for c in rest.get(..len).unwrap_or(rest).chars() {
            if c == '\n' {
                out.push('\n');
            } else {
                for _ in 0..c.len_utf8() {
                    out.push(' ');
                }
            }
        }
        at += len;
    }
    out
}

/// The identifier `s` ends with, or `""`.
fn trailing_ident(s: &str) -> &str {
    let start = s
        .char_indices()
        .rfind(|(_, c)| !(c.is_alphanumeric() || *c == '_'))
        .map(|(ix, c)| ix + c.len_utf8())
        .unwrap_or_default();
    s.get(start..).unwrap_or("")
}

/// The offset just past the `}` that closes the block whose `{` is at `open`
/// in `code`, a [`blanked`] text.
fn block_end(code: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (ix, c) in code.get(open..)?.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(open + ix + 1);
                }
            }
            _ => {}
        }
    }
    None
}

/// One function of a showcase file.
struct FnItem<'a> {
    /// The offset of its `fn`.
    at: usize,
    name: &'a str,
    /// Its body, from the `{` to just past the `}`.
    body: std::ops::Range<usize>,
}

/// Every function with a body in `raw`, whose [`blanked`] text is `code`, in
/// source order: free functions, methods and functions nested in either.
fn fn_items<'a>(raw: &'a str, code: &str) -> Vec<FnItem<'a>> {
    let mut out = Vec::new();
    for (at, _) in code.match_indices("fn ") {
        if code
            .get(..at)
            .and_then(|t| t.chars().next_back())
            .is_some_and(|c| c.is_alphanumeric() || c == '_')
        {
            continue;
        }
        let rest = code.get(at + 3..).unwrap_or("");
        let name_at = at + 3 + (rest.len() - rest.trim_start().len());
        let name = leading_ident(raw.get(name_at..).unwrap_or(""));
        if name.is_empty() {
            continue;
        }
        let mut open = name_at + name.len();
        if code.get(open..).is_some_and(|t| t.starts_with('<')) {
            // Generics, `F: Fn(A) -> B` among them: a `>` after a `-` closes
            // nothing.
            let mut depth = 0usize;
            let mut prev = ' ';
            for (ix, c) in code.get(open..).unwrap_or("").char_indices() {
                match c {
                    '<' => depth += 1,
                    '>' if prev != '-' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            open += ix + 1;
                            break;
                        }
                    }
                    _ => {}
                }
                prev = c;
            }
        }
        if !code.get(open..).is_some_and(|t| t.starts_with('(')) {
            continue;
        }
        let Some(close) = end_of_call(code, open) else {
            continue;
        };
        // The body opens at the first `{` outside brackets; a `;` there first
        // is a declaration without one. `-> [Tag; 3]` has a `;` inside them.
        let mut depth = 0usize;
        let mut body_open = None;
        for (ix, c) in code.get(close..).unwrap_or("").char_indices() {
            match c {
                '(' | '[' => depth += 1,
                ')' | ']' => depth = depth.saturating_sub(1),
                '{' if depth == 0 => {
                    body_open = Some(close + ix);
                    break;
                }
                ';' if depth == 0 => break,
                _ => {}
            }
        }
        let Some(body_open) = body_open else {
            continue;
        };
        let Some(body_end) = block_end(code, body_open) else {
            continue;
        };
        out.push(FnItem {
            at,
            name,
            body: body_open..body_end,
        });
    }
    out
}

/// Whether the function whose `fn` is at `at` in `code` is `pub` or
/// `pub(crate)`.
fn is_public(code: &str, at: usize) -> bool {
    let before = code.get(..at).unwrap_or("").trim_end();
    before.ends_with("pub") || before.ends_with("pub(crate)")
}

// ---------------------------------------------------------------------------
// Every widget reports itself
// ---------------------------------------------------------------------------
//
// Showcase spec §5.3 and §10.1. A widget the pointer can rest on and learn
// nothing from is what per-instance Widget Info exists to end: each is built
// by a `demo::` or `chrome::` helper that wraps it with `.info(`. Two rules
// keep that so. Outside those two files no gpui-component widget is
// constructed at all; and every public helper in them that constructs one
// -- in its own body or in a private function of its file it calls, however
// indirectly -- also calls `.info(` in one of those bodies. The inspector is the named exemption from the first
// rule (spec §4.4); the test module and the `info/` files, which build no
// demo, are not read by it.
//
// What a widget is comes from upstream, not from a list here: a type
// gpui-component or gpui-base draws -- one it implements `RenderOnce`,
// `Render`, `Element` or `IntoElement` for, or derives `IntoElement` on. A
// type upstream names `…State` is not one: it is a widget's model, such as
// `InputState` or `ListState`, which the view keeps across frames and hands
// to the helper that builds the widget, so constructing it puts nothing on
// screen. Which names a file can mean by those types comes from its own
// `use gpui_component::…` and `use gpui_base::…` items, read the way
// `scripts/check-widget-coverage.py` reads them (`toolkit_roots`): `Tag`
// imported by name, or reached through an imported module (`form::Form`) or
// the crate itself (`gpui_component::tag::Tag`).
//
// A constructor is any associated function called on such a type,
// `Tag::new(` and `Tag::primary(` alike. The few calls of one that put no
// widget on screen are listed in `NOT_WIDGET_CONSTRUCTORS`, each with the
// upstream code that shows it. Two shapes are not read: a constructor passed
// as a function item (`.map(Tag::new)`), and a free function that returns a
// widget (`h_resizable(`), which spec §10.1 does not count as a constructor.

/// The files whose helpers build the widgets and report them (spec §5.3).
const HELPER_FILES: &[&str] = &["demo.rs", "chrome.rs"];

/// The inspector, whose content is built without `.info()` so that the
/// pointer can move into it without replacing what it shows (spec §4.4): the
/// one module the first rule exempts by name.
const UNREPORTED_MODULE: &str = "inspector.rs";

/// The directory of the files that say what a widget reports.
const INFO_DIR: &str = "info/";

/// The crates whose widgets report themselves, as a `use` item names them.
const WIDGET_CRATES: &[&str] = &["gpui_component", "gpui_base"];

/// The traits whose implementors upstream draws.
const DRAWN_TRAITS: &[&str] = &["RenderOnce", "Render", "Element", "IntoElement"];

/// Calls of a widget type's associated functions outside the helpers that put
/// no widget on screen, as (file, `Type::function`, why).
const NOT_WIDGET_CONSTRUCTORS: &[(&str, &str, &str)] = &[
    (
        "app.rs",
        "AppMenuBar::new",
        "returns an Entity<AppMenuBar> (menu/app_menu_bar.rs, AppMenuBar::new), which \
         the view keeps so that an open menu outlives the frame; demo::menu_bar or \
         demo::title_bar draws it and reports it",
    ),
    (
        "app.rs",
        "Root::render_dialog_layer",
        "draws the Dialogs Root keeps (root.rs, Root::render_dialog_layer); each is \
         built by the demo helper its opener hands `open_dialog` or \
         `open_alert_dialog`",
    ),
    (
        "app.rs",
        "Root::render_notification_layer",
        "draws the Notifications Root keeps (root.rs, \
         Root::render_notification_layer); each is pushed by the demo helper whose \
         widget reports it",
    ),
    (
        "app.rs",
        "Root::render_sheet_layer",
        "draws the Sheet Root keeps (root.rs, Root::render_sheet_layer), built by \
         the demo helper its opener hands `open_sheet` or `open_sheet_at`",
    ),
    (
        "main.rs",
        "Root::new",
        "the window's root view: upstream finds its Dialogs, Sheets and \
         Notifications through window.root::<Root>() (root.rs, Root::update), so the \
         window has to be a Root, which no element can wrap",
    ),
];

/// Whether `file` is one the first rule of `every_widget_reports_itself`
/// leaves out: a helper file, the inspector, the test module, or an info file.
fn exempt_from_the_first_rule(file: &str) -> bool {
    HELPER_FILES.contains(&file)
        || file == UNREPORTED_MODULE
        || file == TEST_MODULE
        || file.starts_with(INFO_DIR)
}

/// Every `.rs` file under `dir`, appended to `out`.
fn collect_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// The types `source` implements one of `DRAWN_TRAITS` for, or derives
/// `IntoElement` on.
fn drawn_types(source: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut deriving = false;
    for line in source.lines().map(str::trim_start) {
        if line.starts_with("#[derive(") && line.contains("IntoElement") {
            deriving = true;
            continue;
        }
        if deriving {
            if line.starts_with("#[") || line.starts_with("//") {
                continue;
            }
            deriving = false;
            let declared = ["struct ", "enum "]
                .iter()
                .find_map(|kw| line.split_once(kw).map(|(_, rest)| leading_ident(rest)));
            if let Some(name) = declared.filter(|n| !n.is_empty()) {
                out.push(name);
            }
            continue;
        }
        let Some(rest) = line.strip_prefix("impl") else {
            continue;
        };
        let Some((head, ty)) = rest.split_once(" for ") else {
            continue;
        };
        let trait_name = head
            .split_whitespace()
            .last()
            .and_then(|t| t.rsplit("::").next())
            .unwrap_or("");
        if !DRAWN_TRAITS.contains(&trait_name) {
            continue;
        }
        let path = ty
            .trim_start()
            .split(|c: char| !(c.is_alphanumeric() || c == '_' || c == ':'))
            .next()
            .unwrap_or("");
        let name = path.rsplit("::").next().unwrap_or("");
        if !name.is_empty() {
            out.push(name);
        }
    }
    out
}

/// The widget types of gpui-component and gpui-base, read from their
/// sources: every drawn type but the `…State` models.
fn upstream_widget_types(roots: &[(String, PathBuf)]) -> BTreeSet<String> {
    let mut files = Vec::new();
    for (name, root) in roots {
        if WIDGET_CRATES.iter().any(|c| c.replace('_', "-") == *name) {
            collect_sources(root, &mut files);
        }
    }
    let mut out = BTreeSet::new();
    for file in files {
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        out.extend(
            drawn_types(&text)
                .into_iter()
                .filter(|name| !name.ends_with("State"))
                .map(str::to_string),
        );
    }
    out
}

/// The names the `use` items of `code`, a [`blanked`] file, bring in from a
/// widget crate: types, and modules a path can start from.
fn toolkit_names(code: &str) -> BTreeSet<&str> {
    let mut out = BTreeSet::new();
    for krate in WIDGET_CRATES {
        let open = format!("use {krate}::");
        for (at, _) in code.match_indices(&open) {
            if code
                .get(..at)
                .and_then(|t| t.chars().next_back())
                .is_some_and(|c| c.is_alphanumeric() || c == '_')
            {
                continue;
            }
            let from = at + open.len();
            let Some(end) = code.get(from..).and_then(|t| t.find(';')) else {
                continue;
            };
            out.extend(
                code.get(from..from + end)
                    .unwrap_or("")
                    .split(|c: char| !(c.is_alphanumeric() || c == '_'))
                    .filter(|w| !w.is_empty() && !matches!(*w, "self" | "as" | "crate" | "super")),
            );
        }
    }
    out
}

/// Every call of an associated function of a type in `widgets` in `code`, a
/// [`blanked`] file, that the file's `use` items resolve to a widget crate:
/// the offset of the type and `Type::function`.
fn widget_constructions(code: &str, widgets: &BTreeSet<String>) -> Vec<(usize, String)> {
    let names = toolkit_names(code);
    let mut out = Vec::new();
    for (sep, _) in code.match_indices("::") {
        let after = code.get(sep + 2..).unwrap_or("");
        let function = leading_ident(after);
        let is_call = after
            .get(function.len()..)
            .is_some_and(|t| t.trim_start().starts_with('('));
        if function.is_empty()
            || !function.starts_with(|c: char| c.is_lowercase() || c == '_')
            || !is_call
        {
            continue;
        }
        let before = code.get(..sep).unwrap_or("");
        let ty = trailing_ident(before);
        if !ty.starts_with(char::is_uppercase) || !widgets.contains(ty) {
            continue;
        }
        let ty_at = sep - ty.len();
        // The first segment of the path the type is written with, if any.
        let mut root = None;
        let mut head = code.get(..ty_at).unwrap_or("");
        while let Some(path) = head.strip_suffix("::") {
            let segment = trailing_ident(path);
            if segment.is_empty() {
                break;
            }
            root = Some(segment);
            head = path.get(..path.len() - segment.len()).unwrap_or("");
        }
        let resolves = match root {
            None => names.contains(ty),
            Some(root) => WIDGET_CRATES.contains(&root) || names.contains(root),
        };
        if resolves {
            out.push((ty_at, format!("{ty}::{function}")));
        }
    }
    out
}

/// Whether `body`, [`blanked`] code, wraps something in `.info(ui, id, info)`
/// -- by method or by path -- rather than calling the `ButtonVariants::info()`
/// that takes nothing.
fn reports(body: &str) -> bool {
    body.contains("InfoExt::info(")
        || body.match_indices(".info(").any(|(at, call)| {
            body.get(at + call.len()..)
                .is_some_and(|rest| !rest.trim_start().starts_with(')'))
        })
}

/// The body of `item` and of every private function of the same file it
/// calls by name, directly or through another: what a public helper builds
/// includes the parts it delegates to.
fn reached_bodies(
    code: &str,
    items: &[FnItem<'_>],
    item: &FnItem<'_>,
) -> Vec<std::ops::Range<usize>> {
    let mut reached = vec![item.body.clone()];
    let mut ix = 0;
    while let Some(body) = reached.get(ix).cloned() {
        ix += 1;
        let text = code.get(body.clone()).unwrap_or("");
        for callee in items.iter().filter(|f| !is_public(code, f.at)) {
            let called = text.match_indices(callee.name).any(|(at, _)| {
                let before = text.get(..at).unwrap_or("");
                let after = text.get(at + callee.name.len()..).unwrap_or("");
                after.starts_with('(')
                    && !before.ends_with(|c: char| {
                        c.is_alphanumeric() || c == '_' || c == '.' || c == ':'
                    })
                    && trailing_ident(before.trim_end()) != "fn"
            });
            if called && !reached.contains(&callee.body) {
                reached.push(callee.body.clone());
            }
        }
    }
    reached
}

/// What `every_widget_reports_itself` finds in `files`, given upstream's
/// `widgets`: the widgets nothing reports, the `exempt` entries that match no
/// call any more, how many constructions the helper files hold, and how many
/// public helpers construct a widget -- themselves or through a private
/// function of their file (`reached_bodies`).
fn unreported_widgets(
    files: &[(&str, &str)],
    widgets: &BTreeSet<String>,
    exempt: &[(&str, &str, &str)],
) -> (Vec<String>, Vec<String>, usize, usize) {
    let mut findings = Vec::new();
    let mut stale = Vec::new();
    let mut in_helpers = 0usize;
    let mut helpers = 0usize;
    let mut exempted = BTreeSet::new();
    for &(file, raw) in files {
        let code = blanked(raw);
        let starts = line_offsets(raw);
        let built = widget_constructions(&code, widgets);
        if HELPER_FILES.contains(&file) {
            in_helpers += built.len();
            let items = fn_items(raw, &code);
            for item in items.iter().filter(|item| is_public(&code, item.at)) {
                let bodies = reached_bodies(&code, &items, item);
                let inside: BTreeSet<&str> = built
                    .iter()
                    .filter(|(at, _)| bodies.iter().any(|body| body.contains(at)))
                    .map(|(_, call)| call.as_str())
                    .collect();
                if inside.is_empty() {
                    continue;
                }
                helpers += 1;
                let reported = bodies
                    .iter()
                    .any(|body| reports(code.get(body.clone()).unwrap_or("")));
                if !reported {
                    findings.push(format!(
                        "{file}:{}: `{}` builds {} and never calls .info(",
                        line_at(&starts, item.at),
                        item.name,
                        inside.into_iter().collect::<Vec<_>>().join(", ")
                    ));
                }
            }
        } else if !exempt_from_the_first_rule(file) {
            for (at, call) in built {
                if exempt.iter().any(|(f, c, _)| *f == file && *c == call) {
                    exempted.insert((file, call));
                    continue;
                }
                findings.push(format!(
                    "{file}:{}: {call}( builds a widget outside demo.rs and chrome.rs, \
                     so nothing reports it",
                    line_at(&starts, at)
                ));
            }
        }
    }
    for (file, call, _) in exempt {
        if files.iter().any(|(f, _)| f == file) && !exempted.contains(&(*file, (*call).to_string()))
        {
            stale.push(format!(
                "NOT_WIDGET_CONSTRUCTORS lists {call} in {file}, which no longer calls it"
            ));
        }
    }
    (findings, stale, in_helpers, helpers)
}

/// Spec §10.1: no gpui-component widget is built outside `demo.rs` and
/// `chrome.rs`, and every public helper there that builds one reports it.
#[test]
fn every_widget_reports_itself() {
    let located = cited_source_dirs();
    assert!(
        located.is_ok(),
        "the upstream sources could not be located, so no widget type is known: {}",
        located.as_ref().err().cloned().unwrap_or_default()
    );
    let Ok(roots) = located else { return };
    let widgets = upstream_widget_types(&roots);
    assert!(
        widgets.contains("Tag") && widgets.contains("Button"),
        "the upstream sources yielded {} widget types, not Tag and Button among \
         them, so this test cannot recognise a widget",
        widgets.len()
    );
    let (findings, stale, in_helpers, helpers) =
        unreported_widgets(SHOWCASE_FILES, &widgets, NOT_WIDGET_CONSTRUCTORS);
    assert!(
        in_helpers > 0 && helpers > 0,
        "no widget construction found in demo.rs or chrome.rs ({in_helpers}), or \
         no public helper building one ({helpers}), so this test would pass \
         vacuously"
    );
    assert!(
        findings.is_empty(),
        "a widget is built where nothing reports it (spec §5.3, §10.1): build it \
         in a demo:: or chrome:: helper that calls .info(:\n  {}",
        findings.join("\n  ")
    );
    assert!(
        stale.is_empty(),
        "NOT_WIDGET_CONSTRUCTORS exempts a call the showcase no longer makes; \
         remove the entry:\n  {}",
        stale.join("\n  ")
    );
}

/// The scanners behind `every_widget_reports_itself` see what they must: a
/// constructor by name, through a module and through the crate, but not one
/// in a comment or a string, not a type the file does not import from a
/// widget crate, and not a `…State`; and a public helper that builds without
/// reporting, but not a private part or a helper that reports.
#[test]
fn the_widget_scanners_do_their_jobs() {
    let upstream = "#[derive(IntoElement)]\n\
                    /// A tag.\n\
                    pub struct Tag {}\n\
                    impl RenderOnce for Label {}\n\
                    impl<D: ListDelegate> Render for ListState<D> {}\n\
                    impl gpui::Element for Form {}\n\
                    impl Styled for Button {}\n";
    let widgets: BTreeSet<String> = drawn_types(upstream)
        .into_iter()
        .filter(|name| !name.ends_with("State"))
        .map(str::to_string)
        .collect();
    assert_eq!(
        widgets.iter().map(String::as_str).collect::<Vec<_>>(),
        vec!["Form", "Label", "Tag"]
    );

    let page = r#"use gpui_component::{form, label::Label};
fn page() {
    let a = Tag::primary();
    // Label::new("commented")
    let s = "Label::new(quoted)";
    let b = Label::new("x");
    let c = form::Form::horizontal();
    let d = gpui_component::tag::Tag::new();
    let e = other::Label::new("y");
    let f = ListState::new(d);
}
"#;
    let code = blanked(page);
    let starts = line_offsets(page);
    let found: Vec<(usize, String)> = widget_constructions(&code, &widgets)
        .into_iter()
        .map(|(at, call)| (line_at(&starts, at), call))
        .collect();
    assert_eq!(
        found,
        vec![
            (6, "Label::new".to_string()),
            (7, "Form::horizontal".to_string()),
            (8, "Tag::new".to_string()),
        ]
    );

    let helpers = r#"use gpui_component::{tag::Tag, label::Label};
pub(crate) fn reported(ui: &Ui, id: &'static str) -> Stateful<Div> {
    Tag::primary().info(ui, id, info::tag())
}
pub(crate) fn by_path(ui: &Ui, id: &'static str) -> Stateful<Div> {
    InfoExt::info(Tag::primary(), ui, id, info::tag())
}
fn part() -> Label {
    Label::new("part")
}
pub(crate) fn unreported() -> Tag {
    Tag::primary().info()
}
pub(crate) fn plain() -> Div {
    div()
}
pub(crate) fn delegates() -> Div {
    div().child(part())
}
fn inner(ui: &Ui, id: &'static str) -> Stateful<Div> {
    Label::new("inner").info(ui, id, info::label())
}
pub(crate) fn through(ui: &Ui, id: &'static str) -> Stateful<Div> {
    inner(ui, id)
}
"#;
    let (findings, stale, in_helpers, helpers) =
        unreported_widgets(&[("demo.rs", helpers), ("pages/p.rs", page)], &widgets, &[]);
    assert_eq!((in_helpers, helpers), (5, 5));
    assert!(stale.is_empty(), "{stale:?}");
    assert_eq!(
        findings,
        vec![
            "demo.rs:11: `unreported` builds Tag::primary and never calls .info(".to_string(),
            "demo.rs:17: `delegates` builds Label::new and never calls .info(".to_string(),
            "pages/p.rs:6: Label::new( builds a widget outside demo.rs and chrome.rs, so \
             nothing reports it"
                .to_string(),
            "pages/p.rs:7: Form::horizontal( builds a widget outside demo.rs and chrome.rs, \
             so nothing reports it"
                .to_string(),
            "pages/p.rs:8: Tag::new( builds a widget outside demo.rs and chrome.rs, so \
             nothing reports it"
                .to_string(),
        ]
    );
    let exempt = [
        ("pages/p.rs", "Label::new", "a sample"),
        ("pages/p.rs", "Tag::gone", "stale"),
    ];
    let (findings, stale, _, _) = unreported_widgets(&[("pages/p.rs", page)], &widgets, &exempt);
    assert!(
        !findings.iter().any(|f| f.contains("Label::new")),
        "{findings:?}"
    );
    assert_eq!(
        stale,
        vec![
            "NOT_WIDGET_CONSTRUCTORS lists Tag::gone in pages/p.rs, which no longer calls it"
                .to_string()
        ]
    );
}

/// Where `text`, a slice of `raw`, starts, or `fallback` if it is not a
/// slice of `raw`.
fn written_at(raw: &str, text: &str, fallback: usize) -> usize {
    let at = offset_in(raw, text);
    if text.as_ptr() >= raw.as_ptr() && at + text.len() <= raw.len() {
        at
    } else {
        fallback
    }
}

// ---------------------------------------------------------------------------
// The Theme Map has one swatch per ThemeColor field
// ---------------------------------------------------------------------------
//
// The Theme Map shows every field of gpui-component's `ThemeColor`, one
// swatch per `ThemeToken` variant (demo.rs), whose row claims the field it
// shows (`info::theme_map::row`). The enum and the row are the showcase's;
// the field list is upstream's, and a patch release that adds a field --
// or removes one, as `ThemeColor::tiles` went -- is what this catches.

/// The file and struct upstream declares the colour fields in.
const THEME_COLOR_FILE: &str = "theme/theme_color.rs";
const THEME_COLOR_OPEN: &str = "pub struct ThemeColor {";

/// The showcase's token enum, and the function whose arms claim its fields.
const THEME_TOKEN_OPEN: &str = "pub(crate) enum ThemeToken {";
const THEME_ROW_OPEN: &str = "pub fn row(";

/// The `pub` fields of the struct that opens at `open` in `source`.
fn struct_fields<'a>(source: &'a str, open: &str) -> Vec<&'a str> {
    let code = blanked(source);
    let Some(start) = code.find(open) else {
        return Vec::new();
    };
    let brace = start + open.len() - 1;
    let Some(end) = block_end(&code, brace) else {
        return Vec::new();
    };
    code.get(brace + 1..end - 1)
        .unwrap_or("")
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix("pub "))
        .filter_map(|rest| {
            let start = offset_in(&code, rest);
            source.get(start..start + leading_ident(rest).len())
        })
        .filter(|name| !name.is_empty())
        .collect()
}

/// The variants of the enum that opens at `open` in `source`.
fn enum_variants<'a>(source: &'a str, open: &str) -> Vec<&'a str> {
    let code = blanked(source);
    let Some(start) = code.find(open) else {
        return Vec::new();
    };
    let brace = start + open.len() - 1;
    let Some(end) = block_end(&code, brace) else {
        return Vec::new();
    };
    code.get(brace + 1..end - 1)
        .unwrap_or("")
        .split(',')
        .filter_map(|variant| {
            // Past its attributes, each on a line of its own.
            let line = variant
                .lines()
                .map(str::trim)
                .rfind(|l| !l.is_empty() && !l.starts_with('#'))?;
            let start = offset_in(&code, line);
            source.get(start..start + leading_ident(line).len())
        })
        .filter(|v| !v.is_empty())
        .collect()
}

/// Every arm `ThemeToken::X =>` of the function that opens at `open` in
/// `source`, with the field the first `claim(` after it names: `(X, field)`,
/// `field` empty where the arm has none that can be read.
fn token_rows<'a>(source: &'a str, open: &str) -> Vec<(&'a str, &'a str)> {
    let code = blanked(source);
    let Some(start) = code.find(open) else {
        return Vec::new();
    };
    let Some(brace) = code
        .get(start..)
        .and_then(|t| t.find('{'))
        .map(|ix| start + ix)
    else {
        return Vec::new();
    };
    let Some(end) = block_end(&code, brace) else {
        return Vec::new();
    };
    let arms: Vec<usize> = code
        .get(brace..end)
        .unwrap_or("")
        .match_indices("ThemeToken::")
        .map(|(ix, m)| brace + ix + m.len())
        .filter(|&at| {
            let name = leading_ident(code.get(at..).unwrap_or(""));
            code.get(at + name.len()..)
                .is_some_and(|t| t.trim_start().starts_with("=>"))
        })
        .collect();
    let claims = code_calls(source, "claim");
    arms.iter()
        .enumerate()
        .map(|(ix, &at)| {
            let next = arms.get(ix + 1).copied().unwrap_or(end);
            let variant = leading_ident(source.get(at..).unwrap_or(""));
            let field = claims
                .iter()
                .find(|&&c| c > at && c < next)
                .and_then(|&c| call_args(source, c))
                .and_then(|args| args.get(1).copied())
                .and_then(unquote)
                .unwrap_or("");
            (variant, field)
        })
        .collect()
}

/// Where `name`, a slice of the file `path` whose text is `text`, is
/// written: `path:line`.
fn written_in(path: &str, text: &str, name: &str) -> String {
    let at = written_at(text, name, text.len());
    format!("{path}:{}", line_at(&line_offsets(text), at))
}

/// What `every_theme_color_field_has_one_theme_token` finds: `fields` are
/// upstream's, read from `(path, text)`, `variants` the enum's and `rows`
/// the claims of its arms, each read from its own file likewise.
fn theme_token_findings(
    (fields_path, fields_text, fields): (&str, &str, &[&str]),
    (variants_path, variants_text, variants): (&str, &str, &[&str]),
    (rows_path, rows_text, rows): (&str, &str, &[(&str, &str)]),
) -> Vec<String> {
    let mut findings = Vec::new();
    for variant in variants {
        let at = written_in(variants_path, variants_text, variant);
        match rows.iter().filter(|(v, _)| v == variant).count() {
            1 => {}
            0 => findings.push(format!(
                "{at}: ThemeToken::{variant} has no row claiming a field"
            )),
            n => findings.push(format!("{at}: ThemeToken::{variant} has {n} rows")),
        }
    }
    for (variant, field) in rows {
        let at = written_in(rows_path, rows_text, variant);
        if !variants.contains(variant) {
            findings.push(format!(
                "{at}: a row for ThemeToken::{variant}, which the enum does not declare"
            ));
        } else if field.is_empty() {
            findings.push(format!(
                "{at}: the row of ThemeToken::{variant} claims no field that can be read"
            ));
        } else if !fields.contains(field) {
            findings.push(format!(
                "{at}: ThemeToken::{variant} shows `{field}`, which ThemeColor does not declare"
            ));
        }
    }
    for field in fields {
        let at = written_in(fields_path, fields_text, field);
        let shown: Vec<&str> = rows
            .iter()
            .filter(|(v, f)| f == field && variants.contains(v))
            .map(|(v, _)| *v)
            .collect();
        match shown.as_slice() {
            [_] => {}
            [] => findings.push(format!(
                "{at}: ThemeColor::{field} has no ThemeToken, so the Theme Map leaves it out"
            )),
            many => findings.push(format!(
                "{at}: ThemeColor::{field} is shown by {} ThemeTokens: {}",
                many.len(),
                many.join(", ")
            )),
        }
    }
    findings
}

/// The Theme Map shows every upstream `ThemeColor` field exactly once, and
/// nothing else (showcase spec §7.3).
#[test]
fn every_theme_color_field_has_one_theme_token() {
    let located = cited_source_dirs();
    assert!(
        located.is_ok(),
        "the vendored sources could not be located, so nothing was checked: {}",
        located.as_ref().err().cloned().unwrap_or_default()
    );
    let Ok(roots) = located else { return };
    let upstream = roots
        .iter()
        .find(|(name, _)| name == "gpui-component")
        .map(|(_, root)| root.join(THEME_COLOR_FILE));
    let text = upstream
        .as_ref()
        .and_then(|path| std::fs::read_to_string(path).ok());
    assert!(
        text.is_some(),
        "gpui-component's {THEME_COLOR_FILE} could not be read, so nothing was checked"
    );
    let text = text.unwrap_or_default();
    let upstream_path = upstream.as_deref().map(shorten).unwrap_or_default();
    let fields = struct_fields(&text, THEME_COLOR_OPEN);
    let source = |path: &str| {
        SHOWCASE_FILES
            .iter()
            .find(|(p, _)| *p == path)
            .map(|(_, text)| *text)
            .unwrap_or_default()
    };
    let (demo, theme_map) = (source("demo.rs"), source(THEME_MAP_INFO));
    let variants = enum_variants(demo, THEME_TOKEN_OPEN);
    let rows = token_rows(theme_map, THEME_ROW_OPEN);
    assert!(
        !fields.is_empty() && !variants.is_empty() && !rows.is_empty(),
        "{} ThemeColor fields, {} ThemeToken variants and {} rows were read, so \
         this test would pass vacuously",
        fields.len(),
        variants.len(),
        rows.len()
    );
    let findings = theme_token_findings(
        (&upstream_path, &text, &fields),
        ("demo.rs", demo, &variants),
        (THEME_MAP_INFO, theme_map, &rows),
    );
    assert!(
        findings.is_empty(),
        "the Theme Map and gpui-component's ThemeColor ({} fields) disagree:\n  {}",
        fields.len(),
        findings.join("\n  ")
    );
}

/// The readers behind the Theme Map gate see a struct's fields, an enum's
/// variants past their doc comments and attributes, and each arm's claimed
/// field; the findings name the line of each.
#[test]
fn the_theme_token_readers_do_their_jobs() {
    let upstream = "pub struct ThemeColor {\n\
                    \x20   /// The background.\n\
                    \x20   pub background: Hsla,\n\
                    \x20   #[serde(default)]\n\
                    \x20   pub border: Hsla,\n\
                    \x20   pub tiles: Hsla,\n\
                    }\n";
    let fields = struct_fields(upstream, THEME_COLOR_OPEN);
    assert_eq!(fields, vec!["background", "border", "tiles"]);
    let tokens = "pub(crate) enum ThemeToken {\n\
                  \x20   /// Behind everything.\n\
                  \x20   Background,\n\
                  \x20   #[doc(hidden)]\n\
                  \x20   Border,\n\
                  \x20   Ring,\n\
                  }\n";
    let variants = enum_variants(tokens, THEME_TOKEN_OPEN);
    assert_eq!(variants, vec!["Background", "Border", "Ring"]);
    let rows = "pub fn row(t: &Theme, token: ThemeToken) -> Row {\n\
                \x20   match token {\n\
                \x20       ThemeToken::Background => (claim(\"value\", \"background\", t.background, \"c.rs:1\"), |i| i),\n\
                \x20       ThemeToken::Border => (\n\
                \x20           claim(\"value\", \"border\", t.border, \"c.rs:2\"),\n\
                \x20           |i| i.config(\"x\", \"ThemeToken::Ring is elsewhere\"),\n\
                \x20       ),\n\
                \x20       ThemeToken::Ring => (claim(\"value\", \"background\", t.ring, \"c.rs:3\"), |i| i),\n\
                \x20   }\n\
                }\n";
    let read = token_rows(rows, THEME_ROW_OPEN);
    assert_eq!(
        read,
        vec![
            ("Background", "background"),
            ("Border", "border"),
            ("Ring", "background")
        ]
    );
    assert_eq!(
        theme_token_findings(
            ("theme_color.rs", upstream, &fields),
            ("demo.rs", tokens, &variants),
            ("info/theme_map.rs", rows, &read),
        ),
        vec![
            "theme_color.rs:3: ThemeColor::background is shown by 2 ThemeTokens: \
             Background, Ring"
                .to_string(),
            "theme_color.rs:6: ThemeColor::tiles has no ThemeToken, so the Theme Map \
             leaves it out"
                .to_string(),
        ]
    );
}

// ---------------------------------------------------------------------------
// A colour claim is read at the line it cites
// ---------------------------------------------------------------------------
//
// Widget-info spec section 4. A panel says which theme field a widget reads,
// which is a statement about a *dependency's* source: nothing in this crate
// can keep it true on its own. Each claim therefore carries the line it was
// read at, and this opens that line and requires the field to be there.
//
// A line and not a symbol, although `file.rs, Symbol` is the form the prose
// notes use. `ButtonVariant::text_color` holds `Self::Link => link` and
// `Self::Text => foreground.opacity(0.9)` two lines apart
// (`button.rs:993-994`), so a symbol-scoped check finds `foreground` inside
// that function and passes Button (Link)'s claim of it. Only a line separates
// variants that share a function. The cost is drift -- an insertion earlier
// in a file moves every later line -- so a failure prints what the cited line
// *now* says, which is what tells "the field moved" from "the claim was
// wrong".
//
// The vendored paths come from `cargo metadata`, run from the test rather
// than from a build script: a test runs after the build, so it can simply ask
// cargo, and a published crate does not gain a build script to locate a
// *dev*-dependency's source.
//
// A claim is a `claim("role", "field", value, "cite")` call in one of the
// `info/` files (showcase spec §3.2 and §10.2), and a note is a
// `.config`/`.not_themeable`/`.instance` call; each is read into a `Claim` or
// a `Prose`, so the gates below ask their questions of both alike.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// The crates a citation may name, by package name.
const CITED_CRATES: &[&str] = &["gpui-component", "gpui-base", "gpui-pre"];

/// Set once the citation pass has finished, so an uncited claim fails too
/// (spec section 6.6). Until then the count is reported.
const REQUIRE_CITATIONS: bool = true;

/// The arguments of the call whose `(` is at `open`, split at depth-0 commas.
///
/// Scans the source as written, because what matters here -- the field a
/// claim names and the line it cites -- are string literals.
fn call_args(raw: &str, open: usize) -> Option<Vec<&str>> {
    let mut depth = 0usize;
    let mut at = open;
    let mut last = open + 1;
    let mut out = Vec::new();
    while at < raw.len() {
        let rest = raw.get(at..)?;
        if let Some(after) = rest.strip_prefix("//") {
            at += 2 + after.find('\n').unwrap_or(after.len());
            continue;
        }
        if let Some(len) = char_literal_len(rest) {
            at += len;
            continue;
        }
        if let Some(hashes) = raw_string_hashes(rest) {
            let close = format!("\"{}", "#".repeat(hashes));
            let from = hashes + 2;
            at += match rest.get(from..).and_then(|t| t.find(&close)) {
                Some(ix) => from + ix + close.len(),
                None => rest.len(),
            };
            continue;
        }
        if let Some(body) = rest.strip_prefix('"') {
            at += 1 + end_of_string(body, "\"");
            continue;
        }
        match rest.chars().next() {
            Some('(' | '[' | '{') => depth += 1,
            Some(')' | ']' | '}') => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    // A trailing comma before the close leaves only
                    // whitespace here; that is not an argument.
                    let tail = raw.get(last..at)?;
                    if !tail.trim().is_empty() {
                        out.push(tail);
                    }
                    return Some(out);
                }
            }
            Some(',') if depth == 1 => {
                out.push(raw.get(last..at)?);
                last = at + 1;
            }
            _ => {}
        }
        at += char_len(rest);
    }
    None
}

/// The contents of a Rust string literal, or `None` if `text` is not one.
fn unquote(text: &str) -> Option<&str> {
    text.trim().strip_prefix('"')?.strip_suffix('"')
}

/// The 1-based line that byte offset `at` falls on.
fn line_at(starts: &[usize], at: usize) -> usize {
    starts.partition_point(|&s| s <= at)
}

/// One colour claim of one panel.
struct Claim<'a> {
    file: &'a str,
    line: usize,
    widget: &'a str,
    role: &'a str,
    field: &'a str,
    /// The value expression, as written.
    value: &'a str,
    cited_at: &'a str,
}

/// Every `claim(` call in `raw` that is code: the line it is on and its
/// arguments.
fn claim_calls(raw: &str) -> Vec<(usize, Vec<&str>)> {
    let starts = line_offsets(raw);
    code_calls(raw, "claim")
        .into_iter()
        .map(|open| {
            (
                line_at(&starts, open),
                call_args(raw, open).unwrap_or_default(),
            )
        })
        .collect()
}

/// A `claim(` call's role, field, value and citation, or `None` unless it
/// has four arguments and the three besides the value are string literals --
/// the one shape spec §3.2 allows, and the only one a gate can read. The
/// value is returned as written, for `value_reads_field`.
fn claim_literals<'a>(args: &[&'a str]) -> Option<(&'a str, &'a str, &'a str, &'a str)> {
    let &[role, field, value, cited] = args else {
        return None;
    };
    Some((
        unquote(role)?,
        unquote(field)?,
        value.trim(),
        unquote(cited)?,
    ))
}

/// The helpers a claim's value may be built with in place of naming its
/// field, each for a reason:
///
/// - `opaque(` (info/icons.rs): a recoloured icon's colour is the
///   foreground the icons were recoloured with when their set was loaded
///   (`Showcase::icon_cache_fg`), with the alpha dropped as the SVG drops
///   it; that copy is passed in, not read off the theme where it is used.
/// - `.input_background(`: upstream's `Theme::input_background()`, which
///   paints input in dark mode and background in light (theme/mod.rs:379-384);
///   the one claim built with it (info/buttons.rs, `disabled_fill`) is the
///   dark-mode arm and names input.
///
/// The showcase's own `input_background(` and `ghost_hover(` need no entry:
/// they return claims whose own values name their fields.
const VALUE_HELPERS: &[&str] = &["opaque(", ".input_background("];

/// Whether a claim's `value` reads the `field` it names -- `t.<field>`,
/// `t.colors.<field>`, `base.tokens.colors.<field>`, or the field through
/// any other path -- or is built with one of `VALUE_HELPERS`. A swatch
/// whose value reads another field shows a colour its field line does not
/// name.
fn value_reads_field(value: &str, field: &str) -> bool {
    let code = without_comments_or_strings(value);
    let named = code.match_indices('.').any(|(at, _)| {
        let rest = code.get(at + 1..).unwrap_or("");
        leading_ident(rest) == field
    });
    named || VALUE_HELPERS.iter().any(|helper| code.contains(helper))
}

/// The methods a note is written with (spec §3.2).
const NOTE_METHODS: &[&str] = &["config", "not_themeable", "instance"];

/// Every `.config(`, `.not_themeable(` and `.instance(` call in `raw` that is
/// code and has two arguments, the first a string literal: the line its text
/// starts on, the `what`, and the text as written. And the line of every
/// two-argument call whose `what` is not a literal -- a note in a shape
/// spec §3.2 does not allow, which the prose gate reports rather than skips,
/// since a note nothing reads is a citation nothing checks.
///
/// The text is taken as written, not only when it is a literal: a note built
/// with `format!` cites upstream just as well, and `prose_citations` finds a
/// citation inside the literal either way. A call with another number of
/// arguments is some other type's method of the same name, not a note.
fn note_calls(raw: &str) -> (Vec<(usize, &str, &str)>, Vec<usize>) {
    let starts = line_offsets(raw);
    let mut out = Vec::new();
    let mut unreadable = Vec::new();
    for name in NOTE_METHODS {
        for open in method_calls(raw, name) {
            let Some(args) = call_args(raw, open) else {
                continue;
            };
            let &[what, text] = args.as_slice() else {
                continue;
            };
            let Some(what) = unquote(what) else {
                unreadable.push(line_at(&starts, open));
                continue;
            };
            let text = text.trim();
            out.push((line_at(&starts, offset_in(raw, text)), what, text));
        }
    }
    out.sort_by_key(|(line, _, _)| *line);
    unreadable.sort_unstable();
    (out, unreadable)
}

/// The `(` of every call of the method `name` in `raw` that is code: a
/// [`code_calls`] match with a `.` before it, wherever rustfmt breaks the
/// chain.
fn method_calls(raw: &str, name: &str) -> Vec<usize> {
    code_calls(raw, name)
        .into_iter()
        .filter(|&open| {
            raw.get(..open.saturating_sub(name.len()))
                .is_some_and(|before| before.trim_end().ends_with('.'))
        })
        .collect()
}

/// The name of the last function declared above byte offset `at`, or `"?"`:
/// what a message calls the widget a claim or a note belongs to, since each
/// kind's claims sit in a function of its own (spec §3.4).
fn enclosing_fn(raw: &str, at: usize) -> &str {
    raw.get(..at)
        .unwrap_or("")
        .lines()
        .rev()
        .map(str::trim_start)
        .filter(|l| {
            l.starts_with("fn ") || l.starts_with("pub fn ") || l.starts_with("pub(crate) fn ")
        })
        .find_map(fn_name_on)
        .unwrap_or("?")
}

/// A note's text: the file and line it is written on, the function it is in
/// -- the widget whose info it belongs to -- and the text as written.
type Prose<'a> = (&'a str, usize, &'a str, &'a str);

/// Every colour claim of the showcase, and every note's text.
fn showcase_claims() -> (Vec<Claim<'static>>, Vec<Prose<'static>>) {
    let mut claims = Vec::new();
    let mut prose = Vec::new();
    for (file, raw) in SHOWCASE_FILES {
        let (c, p) = claims_in(file, raw);
        claims.extend(c);
        prose.extend(p);
    }
    (claims, prose)
}

/// Every colour claim of one showcase file, and every note's text.
///
/// The test module is read too. Its claims are written to exercise the
/// model, but a claim anywhere in the showcase is a statement about upstream
/// and has to be as true as one on a page.
fn claims_in<'a>(file: &'a str, raw: &'a str) -> (Vec<Claim<'a>>, Vec<Prose<'a>>) {
    let mut claims = Vec::new();
    let mut prose = Vec::new();
    for (line, args) in claim_calls(raw) {
        let Some((role, field, value, cited_at)) = claim_literals(&args) else {
            continue;
        };
        let widget = args
            .first()
            .map_or("?", |arg| enclosing_fn(raw, offset_in(raw, arg)));
        claims.push(Claim {
            file,
            line,
            widget,
            role,
            field,
            value,
            cited_at,
        });
    }
    for (line, _, text) in note_calls(raw).0 {
        prose.push((file, line, enclosing_fn(raw, offset_in(raw, text)), text));
    }
    (claims, prose)
}

/// A citation `<file>.rs:<line>` or `<file>.rs:<from>-<to>`.
fn parse_citation(text: &str) -> Option<(&str, usize, usize)> {
    let (path, span) = text.rsplit_once(':')?;
    if !path.ends_with(".rs") {
        return None;
    }
    match span.split_once('-') {
        Some((a, b)) => Some((path, a.parse().ok()?, b.parse().ok()?)),
        None => {
            let n: usize = span.parse().ok()?;
            Some((path, n, n))
        }
    }
}

/// Every file a citation could name, across the crates searched.
///
/// A citation gives a path, not a crate: `button.rs` is gpui-component's
/// `button/button.rs`, `resizable/mod.rs` is gpui-base's, and `geometry.rs`
/// names a file here *and* one in gpui-pre. Rather than guess, every
/// candidate is returned and the claim holds if any bears it out.
fn candidates(citation: &str, roots: &[(String, PathBuf)]) -> Vec<PathBuf> {
    // A leading crate name picks one root: `radio.rs` sits at `src/radio.rs`
    // in gpui-component *and* gpui-base, so no directory can tell them
    // apart and the citation has to name the crate.
    if let Some((head, tail)) = citation.split_once('/')
        && let Some((_, root)) = roots.iter().find(|(name, _)| name == head)
    {
        let path = root.join(tail);
        return if path.is_file() {
            vec![path]
        } else {
            Vec::new()
        };
    }

    let mut out = Vec::new();
    for (_, root) in roots {
        let direct = root.join(citation);
        if direct.is_file() {
            out.push(direct);
        }
    }
    // Only when the path as written matches nothing: a citation that names
    // its directory has said which file it means, and a basename sweep would
    // drag the other crates' same-named files back in.
    if out.is_empty() {
        let base = citation.rsplit('/').next().unwrap_or(citation);
        for (_, root) in roots {
            collect_named(root, base, &mut out);
        }
    }
    out
}

/// A path with everything above the crate directory dropped, for a message
/// a reader can act on.
fn shorten(path: &Path) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for part in path.iter().rev().filter_map(|p| p.to_str()) {
        parts.push(part);
        if part.starts_with("gpui-") || part == "native-theme-gpui" {
            break;
        }
    }
    parts.reverse();
    parts.join("/")
}

/// Every file under `dir` named `name`, appended to `out`.
fn collect_named(dir: &Path, name: &str, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_named(&path, name, out);
        } else if path.file_name().is_some_and(|f| f == name) && !out.contains(&path) {
            out.push(path);
        }
    }
}

/// The `src` directory of each crate a citation may name, and this crate's.
///
/// From `cargo metadata` only -- never a registry path or a version literal,
/// as `scripts/check-widget-coverage.py` also insists. A crate missing from
/// the metadata fails the test rather than being skipped.
fn cited_source_dirs() -> Result<Vec<(String, PathBuf)>, String> {
    let out = std::process::Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .map_err(|e| format!("could not run `cargo metadata`: {e}"))?;
    if !out.status.success() {
        return Err(format!("`cargo metadata` failed: {:?}", out.status));
    }
    let meta: serde_json::Value =
        serde_json::from_slice(&out.stdout).map_err(|e| format!("metadata is not JSON: {e}"))?;
    let packages = meta
        .get("packages")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "cargo metadata has no packages array".to_string())?;

    let mut roots = Vec::new();
    for want in CITED_CRATES {
        let manifest = packages
            .iter()
            .find(|p| p.get("name").and_then(serde_json::Value::as_str) == Some(want))
            .and_then(|p| p.get("manifest_path"))
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| format!("`{want}` is not in the cargo metadata"))?;
        let src = Path::new(manifest)
            .parent()
            .ok_or_else(|| format!("`{want}` has no manifest directory"))?
            .join("src");
        if !src.is_dir() {
            return Err(format!("`{want}` has no src at {}", src.display()));
        }
        roots.push(((*want).to_string(), src));
    }
    roots.push((
        "native-theme-gpui".to_string(),
        Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
    ));
    // The showcase itself: a colour the *application* chooses -- a chart
    // series, say -- has no upstream line to cite, and saying so is more
    // useful than leaving the claim bare.
    roots.push((
        "showcase".to_string(),
        Path::new(env!("CARGO_MANIFEST_DIR")).join("examples"),
    ));
    Ok(roots)
}

/// Spec section 4.2: every colour claim's cited line reads the field it names.
///
/// A claim with no citation is counted, not failed: the citation pass runs
/// page by page and each commit stays green while the count falls.
#[test]
fn every_colour_claim_is_read_at_the_line_it_cites() {
    // A gate that silently skips is not a gate: if the sources cannot be
    // located, the check has not run, and that is a failure.
    let located = cited_source_dirs();
    assert!(
        located.is_ok(),
        "the vendored sources could not be located, so nothing was checked: {}",
        located.as_ref().err().cloned().unwrap_or_default()
    );
    let Ok(roots) = located else { return };
    let (claims, _) = showcase_claims();
    assert!(
        claims.iter().any(|c| c.file != TEST_MODULE),
        "no colour claims found outside the test module, so this test would pass \
         vacuously for every claim the showcase shows"
    );
    // Against the *code*, strings removed: the panel text names the field
    // too, so searching the files as written would let a claim vouch for
    // itself.
    let code: Vec<String> = SHOWCASE_FILES
        .iter()
        .map(|(_, text)| without_comments_or_strings(text))
        .collect();

    let mut wrong = Vec::new();
    // A `claim(` call in any other shape is one this gate cannot read, and a
    // claim nothing reads is a claim nothing checks.
    for (file, raw) in SHOWCASE_FILES {
        for (line, args) in claim_calls(raw) {
            if claim_literals(&args).is_none() {
                wrong.push(format!(
                    "{file}:{line}: not claim(\"role\", \"field\", value, \"cite\"), so \
                     nothing can check it"
                ));
            }
        }
    }
    // The value is what the swatch paints, and it has to be the field the
    // claim names, or the line cited vouches for a colour nobody shows.
    for claim in &claims {
        if !value_reads_field(claim.value, claim.field) {
            wrong.push(format!(
                "{} [{}] {}:{}: claims `{}`, but its value `{}` reads no `.{}` and \
                 uses none of VALUE_HELPERS",
                claim.widget,
                claim.role,
                claim.file,
                claim.line,
                claim.field,
                claim.value,
                claim.field
            ));
        }
    }
    let mut uncited = 0usize;
    for claim in &claims {
        if claim.cited_at.is_empty() {
            uncited += 1;
            continue;
        }
        // `showcase` with no line: the *application* chooses this colour, so
        // there is no upstream read to point at. A line would be worse than
        // useless -- the showcase is what is being edited, so every edit
        // above a self-citation silently invalidates it, which is exactly
        // what happened when this was first tried.
        if claim.cited_at == "showcase" {
            if code.iter().any(|text| mentions(text, claim.field)) {
                continue;
            }
            wrong.push(format!(
                "{} [{}] {}:{}: cited as the application's own, but the \
                 showcase never sets `{}`",
                claim.widget, claim.role, claim.file, claim.line, claim.field
            ));
            continue;
        }
        let Some((path, first, last)) = parse_citation(claim.cited_at) else {
            wrong.push(format!(
                "{} [{}] {}:{}: `{}` is not <file>.rs:<line>",
                claim.widget, claim.role, claim.file, claim.line, claim.cited_at
            ));
            continue;
        };
        let files = candidates(path, &roots);
        if files.is_empty() {
            wrong.push(format!(
                "{} [{}] {}:{}: cites {path}, which does not exist",
                claim.widget, claim.role, claim.file, claim.line
            ));
            continue;
        }
        // A line number against the wrong file means nothing, and a bare
        // name can match several crates -- `toggle.rs` is gpui-component's
        // `button/toggle.rs` and gpui-base's `toggle.rs`. The author knows
        // which one they read, so the citation has to say.
        if files.len() > 1 {
            wrong.push(format!(
                "{} [{}] {}:{}: `{path}` is ambiguous; name the directory too. \
                 It matches: {}",
                claim.widget,
                claim.role,
                claim.file,
                claim.line,
                files
                    .iter()
                    .map(|f| shorten(f))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            continue;
        }
        let Some(file) = files.first() else {
            continue;
        };
        let Ok(text) = std::fs::read_to_string(file) else {
            wrong.push(format!(
                "{} [{}] {}:{}: cites {path}, which could not be read",
                claim.widget, claim.role, claim.file, claim.line
            ));
            continue;
        };
        let lines: Vec<&str> = text.lines().collect();
        let span = if first == 0 || first > last || last > lines.len() {
            None
        } else {
            lines.get(first - 1..last)
        };
        let Some(span) = span else {
            wrong.push(format!(
                "{} [{}] {}:{}: cites {}, but {} has {} lines",
                claim.widget,
                claim.role,
                claim.file,
                claim.line,
                claim.cited_at,
                shorten(file),
                lines.len()
            ));
            continue;
        };
        if !mentions(&span.join("\n"), claim.field) {
            let shown = span.first().map(|l| l.trim()).unwrap_or("(blank)");
            wrong.push(format!(
                "{} [{}] {}:{}: claims `{}` at {}\n      that line now reads: {}",
                claim.widget,
                claim.role,
                claim.file,
                claim.line,
                claim.field,
                claim.cited_at,
                shown
            ));
        }
    }

    assert!(
        wrong.is_empty(),
        "{} of {} colour claims are not read at the line they cite:\n  {}",
        wrong.len(),
        claims.len(),
        wrong.join("\n  ")
    );
    assert!(
        !REQUIRE_CITATIONS || uncited == 0,
        "{uncited} of {} colour claims carry no citation",
        claims.len()
    );
    if uncited > 0 {
        println!(
            "Widget Info citations: {} of {} cited, {uncited} to go",
            claims.len() - uncited,
            claims.len()
        );
    }
}

/// Every `theme().<field>` and `theme().tokens.<field>` read in `text`.
///
/// A field, not a method: `theme().is_dark()` asks something *about* the
/// theme rather than naming a colour a widget paints, so an identifier
/// followed by `(` is skipped.
fn theme_reads(text: &str) -> BTreeSet<&str> {
    const OPEN: &str = "theme().";
    let mut out = BTreeSet::new();
    let mut from = 0usize;
    while let Some(ix) = text.get(from..).and_then(|t| t.find(OPEN)) {
        let at = from + ix + OPEN.len();
        from = at;
        let mut rest = text.get(at..).unwrap_or("");
        if let Some(after) = rest.strip_prefix("tokens.") {
            rest = after;
        }
        let ident = leading_ident(rest);
        if ident.is_empty() || rest.get(ident.len()..).is_some_and(|t| t.starts_with('(')) {
            continue;
        }
        out.insert(ident);
    }
    out
}

/// The Theme Map's infos: one swatch per ThemeColor field, which between
/// them name every field by construction. Counted as panels, they would
/// leave the omission report nothing to find, so it leaves them out;
/// `every_theme_color_field_has_one_theme_token` holds them to the field
/// list instead.
const THEME_MAP_INFO: &str = "info/theme_map.rs";

/// Whether a claim or a note in `file` is one a widget's panel shows: not the
/// test module's, and not a Theme Map swatch's.
fn is_widget_panel(file: &str) -> bool {
    file != TEST_MODULE && file != THEME_MAP_INFO
}

/// Spec section 5: the theme fields the cited files read that no widget's
/// panel names.
///
/// The plan ordered this after the citation pass for a reason -- once every
/// claim carries a citation, the widget-to-file mapping is exact rather than
/// guessed: **the files a panel cites are the files its widget is implemented
/// in**. So the scope of this report is the union of those files, and no new
/// guesser is needed.
///
/// The bar is *no panel anywhere names it*, not *this panel names it*.
/// `button.rs` holds ten Button variants; measuring each Button panel against
/// the whole file would report the other nine's tokens as omissions and bury
/// the report in noise it was already known to produce. A field some other
/// panel names is documented in the showcase, just not here.
///
/// The panels are the widgets' own: the test module's claims are checked but
/// are not what the showcase tells its reader, and the Theme Map's swatches
/// name every field by construction (`THEME_MAP_INFO`), so neither counts.
///
/// Printed, never failed (W6). A panel legitimately says nothing about states
/// and variants its demo does not show, and turning that into a gate would
/// need an exception per unshown state across a hundred widgets -- the sprawl
/// this design exists to avoid. The residual is a number someone chose: it is
/// recorded in the CHANGELOG.
///
/// Run it with `cargo test -p native-theme-gpui --lib the_omission_report --
/// --nocapture`.
#[test]
fn the_omission_report() {
    let located = cited_source_dirs();
    assert!(
        located.is_ok(),
        "the vendored sources could not be located, so nothing was measured: {}",
        located.as_ref().err().cloned().unwrap_or_default()
    );
    let Ok(roots) = located else { return };
    let (claims, _) = showcase_claims();
    let claims: Vec<&Claim<'_>> = claims.iter().filter(|c| is_widget_panel(c.file)).collect();
    assert!(
        !claims.is_empty(),
        "no widget panel's claims were found, so this report would be empty \
         for the wrong reason"
    );
    let panels: BTreeSet<&str> = claims.iter().map(|c| c.widget).collect();

    // Every file a claim cites, with the panels that cite it. An ambiguous or
    // unparseable citation is left out here; the citation gate already fails
    // on both, so this report never has to speak about them.
    let mut cited: BTreeMap<PathBuf, BTreeSet<&str>> = BTreeMap::new();
    for claim in &claims {
        let Some((path, _, _)) = parse_citation(claim.cited_at) else {
            continue;
        };
        let found = candidates(path, &roots);
        if found.len() != 1 {
            continue;
        }
        let Some(file) = found.first() else {
            continue;
        };
        cited.entry(file.clone()).or_default().insert(claim.widget);
    }
    assert!(
        !cited.is_empty(),
        "no cited file could be located, so this report would be empty for \
         the wrong reason"
    );

    // What the panels say: every argument of every `claim(` and note call,
    // since a field named in a config line or a not-themeable note is
    // described just as much as one that carries a swatch, and the geometry
    // lines' texts.
    let mut said: Vec<&str> = Vec::new();
    for (_, raw) in demo_files().filter(|(file, _)| is_widget_panel(file)) {
        said.extend(claim_calls(raw).into_iter().flat_map(|(_, args)| args));
        said.extend(
            note_calls(raw)
                .0
                .into_iter()
                .flat_map(|(_, what, text)| [what, text]),
        );
    }
    let (notes, _) = geometry_note_entries(geometry_notes_source());
    said.extend(notes.into_iter().map(|(_, _, text)| text));
    let said = said.join("\n");

    let mut unnamed: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
    let mut texts: Vec<(PathBuf, String)> = Vec::new();
    for file in cited.keys() {
        let Ok(text) = std::fs::read_to_string(file) else {
            continue;
        };
        texts.push((file.clone(), text));
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for (file, text) in &texts {
        for field in theme_reads(text) {
            seen.insert(field);
            if mentions(&said, field) {
                continue;
            }
            unnamed.entry(field).or_default().insert(shorten(file));
        }
    }

    println!(
        "\nWidget Info omission report\n  {} widget panels cite {} files, which \
         read {} distinct theme fields.\n  {} of those are named by no widget panel:",
        panels.len(),
        cited.len(),
        seen.len(),
        unnamed.len()
    );
    for (field, files) in &unnamed {
        println!(
            "    {field:<28} {}",
            files.iter().cloned().collect::<Vec<_>>().join(", ")
        );
    }
    if unnamed.is_empty() {
        println!("    (none)");
    }
}

/// Every `file.rs, Symbol` in a panel's prose.
fn prose_citations(text: &str) -> Vec<(&str, &str)> {
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(ix) = text.get(from..).and_then(|t| t.find(".rs,")) {
        let dot = from + ix;
        from = dot + ".rs,".len();
        let Some(head) = text.get(..dot) else {
            continue;
        };
        let start = match head
            .rfind(|c: char| !(c.is_alphanumeric() || c == '_' || c == '/' || c == '.' || c == '-'))
        {
            Some(i) => i + 1,
            None => 0,
        };
        let Some(path) = text.get(start..dot + 3) else {
            continue;
        };
        let Some(after) = text.get(from..) else {
            continue;
        };
        let body = after.trim_start();
        let end = body
            .find(|c: char| !(c.is_alphanumeric() || c == '_' || c == ':'))
            .unwrap_or(body.len());
        let Some(symbol) = body.get(..end).map(|s| s.trim_end_matches(':')) else {
            continue;
        };
        if !symbol.is_empty() && symbol.starts_with(char::is_alphabetic) {
            out.push((path, symbol));
        }
        from += (after.len() - body.len()) + end;
    }
    out
}

/// Spec section 4.6: a symbol a "Not themeable" note cites still exists.
///
/// A note written as a call is read whichever section it fills -- a config
/// line or an instance note citing upstream is as exposed to a deletion -- and
/// so is every `GEOMETRY_NOTES` text, which reaches the inspector as a config
/// line.
///
/// Existence, never semantics. What a note *says* about upstream is a human's
/// to keep true; a note pointing at something upstream has **deleted** is one
/// a machine should catch, and this project has been bitten by exactly that
/// when `ThemeColor::tiles` went away in a patch release.
///
/// A note with no citation is not required to gain one: many state an absence
/// -- "the model carries no circular-progress diameter" -- with no symbol to
/// point at, and demanding a citation there would invite an invented one.
#[test]
fn every_prose_citation_still_exists() {
    // A gate that silently skips is not a gate: if the sources cannot be
    // located, the check has not run, and that is a failure.
    let located = cited_source_dirs();
    assert!(
        located.is_ok(),
        "the vendored sources could not be located, so nothing was checked: {}",
        located.as_ref().err().cloned().unwrap_or_default()
    );
    let Ok(roots) = located else { return };
    let (_, mut prose) = showcase_claims();
    let note_citations: usize = prose
        .iter()
        .filter(|(file, _, _, _)| *file != TEST_MODULE)
        .map(|(_, _, _, text)| prose_citations(text).len())
        .sum();
    // The geometry lines' texts, which a widget's info carries without a note
    // call of its own (spec §3.3).
    let info = geometry_notes_source();
    let starts = line_offsets(info);
    let (notes, _) = geometry_note_entries(info);
    let geometry_citations: usize = notes
        .iter()
        .map(|(_, _, text)| prose_citations(text).len())
        .sum();
    for (_, name, text) in notes {
        let line = line_at(&starts, offset_in(info, text));
        prose.push((GEOMETRY_NOTES_FILE, line, name, text));
    }

    let unreadable: Vec<String> = SHOWCASE_FILES
        .iter()
        .flat_map(|(file, raw)| {
            note_calls(raw)
                .1
                .into_iter()
                .map(move |line| format!("{file}:{line}"))
        })
        .collect();
    assert!(
        unreadable.is_empty(),
        "{} note calls have a `what` that is not a string literal, so nothing \
         reads their text or checks what it cites (spec §3.2):\n  {}",
        unreadable.len(),
        unreadable.join("\n  ")
    );

    let mut missing = Vec::new();
    let mut checked = 0usize;
    for (file, line, widget, text) in &prose {
        for (path, symbol) in prose_citations(text) {
            checked += 1;
            let files = candidates(path, &roots);
            if files.is_empty() {
                missing.push(format!(
                    "{widget} {file}:{line}: cites {path}, which does not exist"
                ));
                continue;
            }
            let leaf = symbol.rsplit("::").next().unwrap_or(symbol);
            let found = files
                .iter()
                .any(|f| std::fs::read_to_string(f).is_ok_and(|t| mentions(&t, leaf)));
            if !found {
                missing.push(format!(
                    "{widget} {file}:{line}: cites {path}, {symbol} — `{leaf}` is in none of them"
                ));
            }
        }
    }

    assert!(
        note_citations > 0,
        "no `file.rs, Symbol` citation found in a note outside the test module, \
         so this test would pass vacuously for every note the showcase shows"
    );
    assert!(
        geometry_citations > 0,
        "no `file.rs, Symbol` citation found in GEOMETRY_NOTES, so its texts \
         went unchecked"
    );
    assert!(
        missing.is_empty(),
        "{} of {checked} prose citations point at something that is gone:\n  {}",
        missing.len(),
        missing.join("\n  ")
    );
}

/// The `pub(crate)` constants of gpui-base the showcase names again, as
/// (upstream file, constant): demo.rs repeats them, since it cannot import
/// them, to lay a handle's info target over the handle's hit area.
const MIRRORED_CONSTANTS: &[(&str, &str)] = &[
    ("gpui-base/resizable/resize_handle.rs", "HANDLE_PADDING"),
    ("gpui-base/resizable/resize_handle.rs", "HANDLE_SIZE"),
];

/// The 1-based line of `const <name>: Pixels = px(<n>)` in `source`, and
/// `<n>`; `None` where no such line is, or its value is not a number.
fn const_px(source: &str, name: &str) -> Option<(usize, f32)> {
    let decl = format!("const {name}: Pixels = px(");
    source.lines().enumerate().find_map(|(ix, line)| {
        let at = line.find(&decl)?;
        let rest = line.get(at + decl.len()..)?;
        let value = rest.get(..rest.find(')')?)?.trim();
        Some((ix + 1, value.parse().ok()?))
    })
}

/// demo.rs's copies of gpui-base's `pub(crate)` handle constants hold the
/// values the pinned gpui-base declares, and cite the lines it declares them
/// on (spec §1.3): an upstream release that moves a handle's padding would
/// otherwise leave the info target off the hit area with every gate green.
#[test]
fn the_mirrored_handle_constants_match_gpui_base() {
    let located = cited_source_dirs();
    assert!(
        located.is_ok(),
        "the upstream sources could not be located, so nothing was checked: {}",
        located.as_ref().err().cloned().unwrap_or_default()
    );
    let Ok(roots) = located else { return };
    let demo = SHOWCASE_FILES
        .iter()
        .find(|(path, _)| *path == "demo.rs")
        .map_or("", |(_, text)| *text);
    let mut wrong = Vec::new();
    for (citation, name) in MIRRORED_CONSTANTS {
        let files = candidates(citation, &roots);
        let upstream = files
            .first()
            .and_then(|file| std::fs::read_to_string(file).ok())
            .and_then(|text| const_px(&text, name));
        let Some((line, value)) = upstream else {
            wrong.push(format!("{citation} declares no `{name}: Pixels = px(..)`"));
            continue;
        };
        let Some((ours_line, ours)) = const_px(demo, name) else {
            wrong.push(format!("demo.rs declares no `{name}: Pixels = px(..)`"));
            continue;
        };
        if ours != value {
            wrong.push(format!(
                "demo.rs:{ours_line}: `{name}` is {ours}px, but {citation}:{line} \
                 declares {value}px"
            ));
        }
        let path = citation.split_once('/').map_or(*citation, |(_, p)| p);
        let cited = format!("({path}:{line})");
        if !demo.contains(&format!("`{name}` {cited}")) {
            wrong.push(format!(
                "demo.rs:{ours_line}: `{name}`'s comment does not cite {cited}, where \
                 the pinned gpui-base declares it"
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "the showcase's copies of gpui-base constants have drifted:\n  {}",
        wrong.join("\n  ")
    );
    assert_eq!(
        const_px("x\nconst HANDLE_SIZE: Pixels = px(1.);\n", "HANDLE_SIZE"),
        Some((2, 1.)),
        "the constant reader reads nothing, so the check above proves nothing"
    );
    assert_eq!(
        const_px("const HANDLE_SIZE: Pixels = px(one);", "HANDLE_SIZE"),
        None
    );
}

/// The claim and note parsers see what they are meant to: a `claim(` call in
/// either of rustfmt's layouts and a note method with a literal `what`, but
/// not a mention in a comment or a string, a longer name, a declaration, a
/// free function named like a note method, or a call of another arity -- and
/// a claim or a note whose literals are not literals is reported, not read.
#[test]
fn the_claim_and_note_parsers_do_their_jobs() {
    let source = r#"pub fn claim(role: &'static str) -> ColorClaim {}
pub fn tag(t: &Theme) -> WidgetInfo {
    // claim("bg", "primary", t.primary, "tag.rs:1")
    let s = "claim(\"bg\", \"x\", v, \"y.rs:1\")";
    my_claim("bg", "x", v, "y.rs:1");
    WidgetInfo::new("Tag")
        .color(claim("bg", "danger", t.danger, "gpui-component/tag.rs:31"))
        .color(claim(
            "text",
            "danger_foreground",
            t.danger_foreground,
            "gpui-component/tag.rs:50",
        ))
        .color(claim(role, "x", v, "y.rs:1"))
        .config("geometry", format!("see tag.rs, Tag"))
        .not_themeable(
            "padding",
            "a rem literal (tag.rs, Tag::render)",
        )
        .instance(label, "reported, not read")
        .config("one argument")
}
fn config(self, what: &'static str, text: String) {}
config("free", "fn, not a method");
"#;
    let calls: Vec<usize> = claim_calls(source).iter().map(|(line, _)| *line).collect();
    assert_eq!(calls, vec![7, 8, 14]);
    let readable: Vec<bool> = claim_calls(source)
        .iter()
        .map(|(_, args)| claim_literals(args).is_some())
        .collect();
    assert_eq!(readable, vec![true, true, false]);

    let (claims, prose) = claims_in("t.rs", source);
    let claims: Vec<_> = claims
        .iter()
        .map(|c| (c.file, c.line, c.widget, c.role, c.field, c.cited_at))
        .collect();
    assert_eq!(
        claims,
        vec![
            ("t.rs", 7, "tag", "bg", "danger", "gpui-component/tag.rs:31"),
            (
                "t.rs",
                8,
                "tag",
                "text",
                "danger_foreground",
                "gpui-component/tag.rs:50"
            ),
        ]
    );
    assert_eq!(
        prose,
        vec![
            ("t.rs", 15, "tag", r#"format!("see tag.rs, Tag")"#),
            (
                "t.rs",
                18,
                "tag",
                r#""a rem literal (tag.rs, Tag::render)""#
            ),
        ]
    );
    assert_eq!(note_calls(source).1, vec![20]);
    let cited: Vec<(&str, &str)> = prose
        .iter()
        .flat_map(|(_, _, _, text)| prose_citations(text))
        .collect();
    assert_eq!(cited, vec![("tag.rs", "Tag"), ("tag.rs", "Tag::render")]);

    let table = "pub const GEOMETRY_NOTES: &[(&str, &str)] = &[\n\
                 \x20   (\"button\", \"a (button.rs, Button)\"),\n\
                 \x20   (\n\
                 \x20       \"input\",\n\
                 \x20       \"d\",\n\
                 \x20   ),\n\
                 \x20   (\"bare\"),\n\
                 ];\n";
    let (entries, unreadable) = geometry_note_entries(table);
    assert_eq!(
        entries,
        vec![
            (2, "button", "\"a (button.rs, Button)\""),
            (3, "input", "\"d\""),
            (7, "bare", "")
        ]
    );
    assert!(unreadable.is_empty(), "{unreadable:?}");

    // A value reads the field its claim names, as a whole identifier, by any
    // path; another field, or a longer one that starts with it, does not.
    assert!(value_reads_field("t.popover", "popover"));
    assert!(value_reads_field("t.accent.opacity(0.5)", "accent"));
    assert!(value_reads_field("base.tokens.colors.border", "border"));
    assert!(value_reads_field("opaque(fg)", "foreground"));
    assert!(!value_reads_field("t.background", "popover"));
    assert!(!value_reads_field("t.popover_foreground", "popover"));
    assert!(!value_reads_field("red", "danger"));
}

// ---------------------------------------------------------------------------
// No hardcoded style values in the showcase
// ---------------------------------------------------------------------------
//
// Spec §6a: a showcase demonstrates the theme, so every radius, every colour
// and every text size it paints has to come from the theme. The call shapes
// below are the ways a literal reaches a pixel: a tailwind radius helper, a
// radius setter given a number, a colour setter given a colour constructor,
// and a text size given in pixels (showcase spec §10.4) -- a rem such as
// `text_sm()` or `rems(0.875)` follows the platform's font, which
// gpui-component makes the window's rem, and a pixel size does not. Line widths
// (`border_1()` and friends) are not listed -- gpui offers no other way to ask
// for a border, and the width the platform states arrives through
// `demo_frame`, which every frame in the showcase goes through.

/// Radius helpers that carry a number of their own.
const RADIUS_HELPERS: &[&str] = &[
    "rounded_sm",
    "rounded_md",
    "rounded_lg",
    "rounded_xl",
    "rounded_2xl",
    "rounded_3xl",
    "rounded_full",
];

/// Radius setters, which take a length: a literal one is the finding.
const RADIUS_SETTERS: &[&str] = &[
    "rounded",
    "rounded_t",
    "rounded_b",
    "rounded_l",
    "rounded_r",
    "rounded_tl",
    "rounded_tr",
    "rounded_bl",
    "rounded_br",
];

/// Text-size setters, which take a length: a literal pixel size is the
/// finding.
const TEXT_SIZE_SETTERS: &[&str] = &["text_size"];

/// Colour setters, which take an `Hsla`: a literal one is the finding.
const COLOUR_SETTERS: &[&str] = &["bg", "border_color", "text_color"];

/// The ways gpui spells a colour out in source.
const COLOUR_LITERALS: &[&str] = &[
    "hsla(",
    "rgb(",
    "rgba(",
    "rgbf(",
    "black()",
    "white()",
    "red()",
    "green()",
    "blue()",
    "yellow()",
    "transparent_black()",
    "transparent_white()",
    "opaque_grey(",
];

/// The sites the rule does not reach, by the name of the `fn` they are in,
/// with the reason.
///
/// Keyed on the enclosing function, not on a marker comment: the source is
/// read with its comments removed, so a `// swatch:` note beside the call
/// would be invisible here. A colour that IS the datum on display -- a swatch
/// of a named colour, a chart series -- belongs on this list; a colour that
/// merely paints a box does not.
const ALLOWED_STYLE_LITERALS: &[(&str, &str)] = &[(
    "the_detector_reads_the_call_and_not_the_line",
    "the detector's own sample source",
)];

/// What the showcase hardcodes, as `line: what, in fn`.
///
/// `source` is read as written, so the line numbers are the file's; the
/// scan is over the same text with comments and strings removed, which is
/// where a mention of a call is told from the call itself.
fn hardcoded_style_values(source: &str, allowed: &[(&str, &str)]) -> Vec<String> {
    let stripped = without_comments_or_strings(source);
    let mut found = Vec::new();
    let mut in_fn = "";
    for (ix, line) in stripped.lines().enumerate() {
        if let Some(name) = fn_name_on(line) {
            in_fn = name;
        }
        if allowed.iter().any(|(f, _)| *f == in_fn) {
            continue;
        }
        let mut report =
            |what: String| found.push(format!("{}: {what} in {in_fn} — {}", ix + 1, line.trim()));
        for helper in RADIUS_HELPERS {
            if line.contains(&format!(".{helper}()")) {
                report(format!(".{helper}()"));
            }
        }
        for setter in RADIUS_SETTERS {
            let call = format!(".{setter}(");
            for (at, _) in line.match_indices(&call) {
                let arg = argument_at(line, at + call.len());
                if starts_with_number_literal(arg) {
                    report(format!(".{setter}(px(…))"));
                }
            }
        }
        for setter in COLOUR_SETTERS {
            let call = format!(".{setter}(");
            for (at, _) in line.match_indices(&call) {
                let arg = argument_at(line, at + call.len());
                if let Some(lit) = COLOUR_LITERALS.iter().find(|lit| arg.starts_with(**lit)) {
                    report(format!(".{setter}({lit}…)"));
                }
            }
        }
        for setter in TEXT_SIZE_SETTERS {
            let call = format!(".{setter}(");
            for (at, _) in line.match_indices(&call) {
                let arg = line.get(at + call.len()..).unwrap_or("").trim_start();
                let arg = arg.strip_prefix("gpui::").unwrap_or(arg);
                if arg
                    .strip_prefix("px(")
                    .is_some_and(|n| starts_with_number_literal(n.trim_start()))
                {
                    report(format!(".{setter}(px(…))"));
                }
            }
        }
    }
    found
}

/// The name of the function `line` declares, if it declares one.
fn fn_name_on(line: &str) -> Option<&str> {
    let after = line.split_once("fn ")?.1;
    let end = after.find(|c: char| !c.is_alphanumeric() && c != '_')?;
    let name = after.get(..end)?;
    (!name.is_empty()).then_some(name)
}

/// The argument written at `at`, with a `gpui::`, `px(` or `gpui::px(`
/// wrapper stepped over, so `.rounded(gpui::px(4.0))` reads as `4.0))`.
fn argument_at(line: &str, at: usize) -> &str {
    let mut arg = line.get(at..).unwrap_or("").trim_start();
    for prefix in ["gpui::", "px(", "gpui::px("] {
        if let Some(rest) = arg.strip_prefix(prefix) {
            arg = rest.trim_start();
        }
    }
    arg
}

/// Whether `arg` opens with a number written out.
fn starts_with_number_literal(arg: &str) -> bool {
    arg.starts_with(|c: char| c.is_ascii_digit())
}

/// Spec §6a: no showcase paints a radius, a colour or a text size of its own
/// invention.
#[test]
fn the_showcase_hardcodes_no_style_values() {
    // The detector reads the calls it checks; a showcase in which it found
    // none would be one it could not read.
    let setters: Vec<String> = RADIUS_SETTERS
        .iter()
        .chain(COLOUR_SETTERS)
        .chain(TEXT_SIZE_SETTERS)
        .map(|setter| format!(".{setter}("))
        .collect();
    let calls: usize = SHOWCASE_FILES
        .iter()
        .map(|(_, source)| {
            let code = without_comments_or_strings(source);
            setters
                .iter()
                .map(|call| code.matches(call.as_str()).count())
                .sum::<usize>()
        })
        .sum();
    assert!(
        calls > 0,
        "no radius, colour or text-size setter call found in the showcase, so \
         this test would pass vacuously"
    );
    let found: Vec<String> = SHOWCASE_FILES
        .iter()
        .flat_map(|(file, source)| {
            hardcoded_style_values(source, ALLOWED_STYLE_LITERALS)
                .into_iter()
                .map(move |finding| format!("{file}:{finding}"))
        })
        .collect();
    assert!(
        found.is_empty(),
        "examples/showcase-gpui/ paints {} value(s) the theme did not give it; \
         a frame goes through `demo_frame`, a widget radius through its \
         `geometry` builder, a text size through a rem (`text_sm()`, \
         `rems(…)`), and a colour that IS the datum goes on \
         ALLOWED_STYLE_LITERALS with its reason:\n{}",
        found.len(),
        found.join("\n")
    );
}

/// The detector has to read the call, not the line, and the allow-list has to
/// be the only way past it.
#[test]
fn the_detector_reads_the_call_and_not_the_line() {
    let source = "fn painted() {\n\
                  div().rounded_md();\n\
                  div().rounded(px(4.0));\n\
                  div().rounded_tl(gpui::px(8.));\n\
                  div().bg(gpui::hsla(0.0, 0.0, 0.5, 0.2));\n\
                  div().border_color(rgb(0x112233));\n\
                  div().text_color(gpui::white());\n\
                  Label::new(t).text_size(px(13.0));\n\
                  div().text_size(gpui::px(12.));\n\
                  }\n";
    let found = hardcoded_style_values(source, &[]);
    assert_eq!(
        found.len(),
        8,
        "one finding per painted literal was expected: {found:?}"
    );
    assert!(
        found.iter().all(|f| f.contains("in painted")),
        "the enclosing fn was not carried into the finding: {found:?}"
    );
    assert!(
        found.iter().any(|f| f.starts_with("2: .rounded_md()")),
        "the line number is not the file's: {found:?}"
    );

    // The theme's own values, a comment and a string are not findings.
    let clean = "fn themed() {\n\
                 // div().rounded_md();\n\
                 let note = \"div().bg(hsla(0.0, 0.0, 0.5, 0.2))\";\n\
                 div().rounded(t.radius).bg(t.muted).border_color(theme.border);\n\
                 div().rounded(theme.radius_lg).text_color(t.muted_foreground);\n\
                 div().text_size(rems(0.875)).text_size(t.font_size);\n\
                 }\n";
    assert_eq!(hardcoded_style_values(clean, &[]), Vec::<String>::new());

    // A line number still points at the file after a multi-line raw string.
    let after_raw = "fn spanning() {\n\
                     let code = r#\"one\ntwo\nthree\"#;\n\
                     div().rounded_full();\n\
                     }\n";
    assert_eq!(
        hardcoded_style_values(after_raw, &[]),
        vec!["5: .rounded_full() in spanning — div().rounded_full();".to_string()]
    );

    // The allow-list is keyed on the enclosing fn.
    assert_eq!(
        hardcoded_style_values(source, &[("painted", "a sample")]),
        Vec::<String>::new()
    );
}

/// The two halves of the test above have to be real: a parser that found no
/// names, or a remover that left commented-out and quoted mentions standing,
/// would let a forgotten builder through without a word.
#[test]
fn the_parser_and_the_remover_do_their_jobs() {
    let module = "pub fn button(n: Native<'_>) -> StyleRefinement {\n\
                  }\n\
                  fn private(n: Native<'_>) {}\n\
                  pub fn icon_size_small(n: Native<'_>) -> Size {\n\
                  }\n";
    assert_eq!(public_fns(module), vec!["button", "icon_size_small"]);

    let source = "// geometry::button\n\
                  let note = \"geometry::input\"; // geometry::radio\n\
                  let raw = r#\"/// geometry::select\"#;\n\
                  geometry::tooltip(n);\n";
    let stripped = without_comments_or_strings(source);
    for named_but_not_called in ["button", "input", "radio", "select"] {
        assert!(
            !references(&stripped, "geometry", named_but_not_called),
            "geometry::{named_but_not_called} is named, not called: {stripped}"
        );
    }
    assert!(references(&stripped, "geometry", "tooltip"));

    // A `//` inside a string literal starts no comment, so the call that
    // follows it on the same line still counts.
    let url = "let url = \"https://example.com/#a\"; geometry::popover(n);\n";
    assert!(references(
        &without_comments_or_strings(url),
        "geometry",
        "popover"
    ));

    // A char literal is code and closes itself, so neither the `"` nor the
    // `/` inside one may open a string or a comment.
    let quotes = "let q = '\"'; geometry::table(n);\n\
                  let slash = '/'; geometry::progress(n);\n\
                  let esc = '\\''; geometry::popover(n);\n";
    let stripped = without_comments_or_strings(quotes);
    for called in ["table", "progress", "popover"] {
        assert!(
            references(&stripped, "geometry", called),
            "a char literal swallowed the call that follows it: {stripped}"
        );
    }

    // A lifetime is not a char literal: the string after it must still be
    // removed, or a quoted mention would count as a use.
    let lifetime = "fn f(n: Native<'_>) { let s = \"geometry::button\"; }\n";
    assert!(!references(
        &without_comments_or_strings(lifetime),
        "geometry",
        "button"
    ));

    assert!(references("geometry::input(n)", "geometry", "input"));
    assert!(!references(
        "geometry::input_height(n)",
        "geometry",
        "input"
    ));
    // Both ends of the path, not just the far one.
    assert!(!references("my_geometry::input(n)", "geometry", "input"));
    assert!(references(
        "native_theme_gpui::geometry::input(n)",
        "geometry",
        "input"
    ));
}
