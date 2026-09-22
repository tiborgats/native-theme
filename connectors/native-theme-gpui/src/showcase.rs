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
//! showcase names builders in both -- every widget's hover note says which
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
        "info/mod.rs",
        include_str!("../examples/showcase-gpui/info/mod.rs"),
    ),
    (
        "info/registry.rs",
        include_str!("../examples/showcase-gpui/info/registry.rs"),
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
/// demo blocks leave it out. The gates that ask what the showcase does *at
/// all* -- which builders it calls, which colours it paints -- read it too.
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
    for (file, raw) in SHOWCASE_FILES {
        let starts = line_offsets(raw);
        for open in code_calls(raw, "native_info") {
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
/// purpose, to see that line. No page records a builder yet (they migrate in
/// Tasks 14-23), so zero calls is a pass here;
/// `the_geometry_note_parsers_do_their_jobs` is what shows the parser finds
/// the calls it must.
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
    for (file, raw) in demo_files() {
        for (line, literal) in geometry_line_calls(raw) {
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
// Every demo block carries a Widget Info panel
// ---------------------------------------------------------------------------
//
// Widget-info spec section 2.2. A demo block is the element chain rooted at
// `div().id("tt-<slug>")`. A block with no `.on_hover(self.hover_info(` is a
// widget a reader can hover and learn nothing from.
//
// A block **ends where its own panel's call ends**, not at the next id. The
// showcase writes a demo as `div().id("tt-x").child(..).on_hover(..)`, so the
// panel closes the chain, and everything after it belongs to the page and not
// to the demo. Taking the next id as the end instead makes the last block of
// a method swallow the rest of it: `tt-tabbar` would have been credited with
// the `geometry::scrollbar_gutter` on the content scroller beside it, which
// is a sibling of the tab bar and not part of it.
//
// The boundaries are read from the file as written and not from the stripped
// copy: `without_comments_or_strings` removes string literals, and the id
// *is* a string literal. The body is then checked in the stripped copy, so a
// panel named inside a comment does not count as one. Both agree on line
// numbers, because a removed span leaves its newlines behind (see
// `without_comments_or_strings`).

/// The marker that opens a demo block, and the call that gives it a panel.
const BLOCK_ID: &str = ".id(\"tt-";
const PANEL_CALL: &str = ".on_hover(self.hover_info(";

/// The one demo whose id reaches `.id()` through a `const` table rather than
/// as a literal: the resizable groups are rendered from [`RESIZABLE_GROUPS`]
/// by a single function, so the call site reads `.id(group.id)`.
///
/// `every_demo_id_is_a_tt_id` keeps that indirection honest, so this marker
/// cannot come to stand for a block that is not a demo.
const INDIRECT_BLOCK_ID: &str = ".id(group.id)";

/// The 0-based line indices of `source` that open a demo block.
fn demo_block_starts(source: &str) -> Vec<usize> {
    source
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(BLOCK_ID) || line.contains(INDIRECT_BLOCK_ID))
        .map(|(n, _)| n)
        .collect()
}

/// A demo block: the line its id is on, the line after its last, and whether
/// a Widget Info panel closes it.
struct Block {
    start: usize,
    end: usize,
    has_panel: bool,
}

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

/// Every demo block of the showcase, in source order.
///
/// `raw` supplies the ids (they are string literals) and `code` the panel
/// call and its extent (parentheses must be code, not text). The two have the
/// same line numbering.
fn demo_blocks(raw: &str, code: &str, methods: &[usize]) -> Vec<Block> {
    let starts = demo_block_starts(raw);
    let offsets = line_offsets(code);
    let total = code.lines().count();
    let mut out = Vec::with_capacity(starts.len());
    for (i, &start) in starts.iter().enumerate() {
        // A block cannot outlive its method, nor reach the next demo.
        let next_id = starts.get(i + 1).copied().unwrap_or(total);
        let next_method = methods
            .iter()
            .find(|&&m| m > start)
            .copied()
            .unwrap_or(total);
        let limit = next_id.min(next_method).min(total);

        let from = offsets.get(start).copied().unwrap_or(code.len());
        let to = offsets.get(limit).copied().unwrap_or(code.len());
        let window = code.get(from..to).unwrap_or("");
        let panel = window.find(PANEL_CALL).and_then(|ix| {
            let open = from + ix + PANEL_CALL.len() - 1;
            end_of_call(code, open)
        });

        match panel {
            Some(close) => out.push(Block {
                start,
                end: code.get(..close).unwrap_or(code).lines().count(),
                has_panel: true,
            }),
            None => out.push(Block {
                start,
                end: limit,
                has_panel: false,
            }),
        }
    }
    out
}

/// Spec section 2.1: an id that reaches a demo block through a `const` table
/// is still a `tt-` id.
///
/// Without this, [`INDIRECT_BLOCK_ID`] would be a hole: renaming a table
/// entry to something that is not a demo would keep the marker and lose the
/// meaning.
#[test]
fn every_demo_id_is_a_tt_id() {
    let mut wrong = Vec::new();
    let mut checked = 0usize;
    for (file, source) in SHOWCASE_FILES {
        for (n, line) in source.lines().enumerate() {
            let Some(rest) = line.trim_start().strip_prefix("id: \"") else {
                continue;
            };
            let Some(id) = rest.split('"').next() else {
                continue;
            };
            checked += 1;
            if !id.starts_with("tt-") {
                wrong.push(format!("{file}:{}: {id}", n + 1));
            }
        }
    }
    assert!(
        checked > 0,
        "no `id: \"...\"` field found in the showcase, so this test would pass vacuously"
    );
    assert!(
        wrong.is_empty(),
        "a demo id reached through a const table must start with `tt-`, or \
         `demo_block_starts` counts a non-demo as a demo block: {}",
        wrong.join(", ")
    );
}

/// Spec section 2.2: every `tt-` demo block carries a Widget Info panel.
#[test]
fn every_demo_block_has_a_widget_info_panel() {
    let mut total = 0usize;
    let mut missing = Vec::new();
    for (file, demos) in demo_files() {
        let code = without_comments_or_strings(demos);
        let blocks = demo_blocks(demos, &code, &method_starts(&code));
        total += blocks.len();
        missing.extend(
            blocks
                .iter()
                .filter(|b| !b.has_panel)
                .map(|b| format!("{file}:{}", b.start + 1)),
        );
    }

    assert!(
        total > 0,
        "no `{BLOCK_ID}` found in the showcase, so this test would pass vacuously"
    );

    assert!(
        missing.is_empty(),
        "{} of {total} demo blocks in examples/showcase-gpui/ carry no Widget \
         Info panel, so hovering them says nothing; at: {}",
        missing.len(),
        missing.join(", ")
    );
}

// ---------------------------------------------------------------------------
// A demo names the builders it applies
// ---------------------------------------------------------------------------
//
// Widget-info spec section 3. A panel's geometry note is the only place a
// reader learns that the native theme shaped a widget, and it is written by
// hand beside code the compiler never relates it to. Seven panels had lost
// that relation when this was written.
//
// A builder reaches a demo in two ways, and both count (spec section 2.1):
// named in the block, or bound once in the enclosing method and applied in
// several blocks. Requiring the second to move would mean duplicating one
// computation across the ten Button demos, so the binding is read instead.

/// `source` with everything **but** its string literals removed: literal
/// bodies are kept, the code around them becomes blank, and line breaks are
/// preserved, so a line number in the result is a line number in the file.
///
/// The complement of [`without_comments_or_strings`], and the reason both
/// exist: a builder a demo *applies* is code, a builder its panel *names* is
/// a string. `every_demo_names_the_builders_it_applies` compares the two, so
/// neither may see the other's half. Comments are dropped here too -- a note
/// in a `//` comment is not what the panel shows a reader.
fn string_literals_only(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut rest = source;
    while let Some(next) = rest.find(['/', '"', 'r', '\'']) {
        let (before, from) = rest.split_at(next);
        push_newlines_of(&mut out, before);
        if let Some(after) = from.strip_prefix("//") {
            rest = match after.find('\n') {
                Some(ix) => &after[ix..],
                None => "",
            };
        } else if let Some(len) = char_literal_len(from) {
            rest = from.get(len..).unwrap_or("");
        } else if let Some(body) = from.strip_prefix('"') {
            let end = end_of_string(body, "\"");
            out.push_str(body.get(..end).unwrap_or(body));
            rest = body.get(end..).unwrap_or("");
        } else if let Some(hashes) = raw_string_hashes(from) {
            let open = hashes + 2;
            let close = format!("\"{}", "#".repeat(hashes));
            let end = match from.get(open..).and_then(|t| t.find(&close)) {
                Some(ix) => open + ix + close.len(),
                None => from.len(),
            };
            out.push_str(from.get(..end).unwrap_or(from));
            rest = from.get(end..).unwrap_or("");
        } else {
            let end = char_len(from);
            push_newlines_of(&mut out, from.get(..end).unwrap_or(""));
            rest = from.get(end..).unwrap_or("");
        }
    }
    push_newlines_of(&mut out, rest);
    out
}

/// A `let` binding that holds a `geometry` builder's value: the line it is
/// on, the variable it binds, and the builder it came from.
struct Binding<'a> {
    line: usize,
    var: &'a str,
    builder: &'a str,
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

/// Every `let <var> = … geometry::<builder> …` in `code`, in source order.
fn geometry_bindings(code: &str) -> Vec<Binding<'_>> {
    let mut out = Vec::new();
    for (line, text) in code.lines().enumerate() {
        let Some(after_let) = text.trim_start().strip_prefix("let ") else {
            continue;
        };
        let after_let = after_let.strip_prefix("mut ").unwrap_or(after_let);
        let var = leading_ident(after_let);
        if var.is_empty() {
            continue;
        }
        let mut rest = after_let;
        while let Some(ix) = rest.find("geometry::") {
            rest = rest.get(ix + "geometry::".len()..).unwrap_or("");
            let builder = leading_ident(rest);
            if !builder.is_empty() {
                out.push(Binding { line, var, builder });
            }
        }
    }
    out
}

/// The 0-based lines on which a method of the showcase's types begins.
///
/// Four-space indent is the showcase's own shape for an `impl` method, and a
/// binding shared by several demo blocks always sits in one. A method another
/// of the showcase's modules calls -- every page's `render_*_tab` -- is
/// `pub(crate)`.
fn method_starts(code: &str) -> Vec<usize> {
    code.lines()
        .enumerate()
        .filter(|(_, l)| {
            l.starts_with("    fn ")
                || l.starts_with("    pub fn ")
                || l.starts_with("    pub(crate) fn ")
        })
        .map(|(n, _)| n)
        .collect()
}

/// The 0-based line where the method enclosing `line` begins, if any.
///
/// `None` for a line above the first method; two `None`s compare equal, which
/// is what is wanted -- both are outside every method, so a binding there is
/// in scope for a block there.
fn enclosing_method(starts: &[usize], line: usize) -> Option<usize> {
    starts.iter().rev().find(|&&s| s <= line).copied()
}

/// Builders that shape a demo's *scaffolding* rather than the widget it
/// demonstrates, and so need no note in that widget's panel.
///
/// A demo opens a dialog from a `Button` and lays its parts out with the
/// platform's spacing; neither is what the panel is about, and requiring a
/// note would put the same two sentences on a dozen panels. Each of these
/// has a panel of its own where it *is* the subject -- the ten Button demos
/// and "Layout spacing" -- so nothing goes undescribed.
const AMBIENT_BUILDERS: &[&str] = &[
    "button",
    "widget_gap",
    "container_margin",
    "window_margin",
    "section_gap",
];

/// Spec section 3: a demo block names every `geometry::` builder that shaped
/// it.
///
/// One direction only. Naming a builder a demo does *not* apply is how a
/// panel says why it could not: `PopupMenu`'s note records that
/// `geometry::menu_item` has no receiver there, and `AlertDialog`'s points at
/// the Dialog above. Requiring set equality would delete both.
#[test]
fn every_demo_names_the_builders_it_applies() {
    let builders = public_fns(GEOMETRY);
    assert!(
        !builders.is_empty(),
        "no `pub fn` found in geometry.rs, so this test would pass vacuously"
    );

    let mut findings = Vec::new();
    let mut total = 0usize;
    for (file, demos) in demo_files() {
        total += unnamed_builders(file, demos, &builders, &mut findings);
    }

    assert!(
        findings.is_empty(),
        "{} of {total} demo blocks in examples/showcase-gpui/ are shaped by a \
         geometry builder their Widget Info panel never names, so hovering them \
         hides what the native theme did:\n  {}",
        findings.len(),
        findings.join("\n  ")
    );
}

/// The demo blocks of one showcase file whose panel leaves a builder that
/// shaped them unnamed, appended to `findings` as `file:line: builders`.
/// Returns how many demo blocks the file holds.
///
/// Per file, because a demo block never spans two: a block, the method
/// around it and the bindings that method makes are all in the one file.
fn unnamed_builders(
    file: &str,
    demos: &str,
    builders: &[&str],
    findings: &mut Vec<String>,
) -> usize {
    let code = without_comments_or_strings(demos);
    let notes = string_literals_only(demos);
    let code_lines: Vec<&str> = code.lines().collect();
    let note_lines: Vec<&str> = notes.lines().collect();

    let methods = method_starts(&code);
    let blocks = demo_blocks(demos, &code, &methods);
    let bindings = geometry_bindings(&code);

    let spans: Vec<(usize, usize)> = blocks.iter().map(|b| (b.start, b.end)).collect();
    let inside_a_block = |line: usize| spans.iter().any(|&(a, b)| a <= line && line < b);

    for block in &blocks {
        let to = block.end.min(code_lines.len());
        let from = block.start.min(to);
        let block_code = code_lines.get(from..to).unwrap_or(&[]).join("\n");
        let note_to = block.end.min(note_lines.len());
        let note_from = block.start.min(note_to);
        let block_note = note_lines.get(note_from..note_to).unwrap_or(&[]).join("\n");
        let method = enclosing_method(&methods, block.start);

        let mut applied: Vec<&str> = builders
            .iter()
            .copied()
            .filter(|b| references(&block_code, "geometry", b))
            .collect();
        for binding in &bindings {
            // A binding inside a block belongs to that block; one outside
            // every block is the method's, and is shared by its demos.
            let reaches = if inside_a_block(binding.line) {
                block.start <= binding.line && binding.line < block.end
            } else {
                enclosing_method(&methods, binding.line) == method
            };
            if reaches && mentions(&block_code, binding.var) && !applied.contains(&binding.builder)
            {
                applied.push(binding.builder);
            }
        }

        let unnamed: Vec<&str> = applied
            .iter()
            .copied()
            .filter(|b| !AMBIENT_BUILDERS.contains(b))
            .filter(|b| !references(&block_note, "geometry", b))
            .collect();
        if !unnamed.is_empty() {
            findings.push(format!(
                "{file}:{}: {}",
                block.start + 1,
                unnamed.join(", ")
            ));
        }
    }
    blocks.len()
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
// A claim is written in one of two forms while the pages migrate (showcase
// spec §3.2 and §10.2): a tuple in a `hover_info` array, or a
// `claim("role", "field", value, "cite")` call. Both are read into the same
// `Claim`, and a panel's prose and a `.config`/`.not_themeable`/`.instance`
// note into the same `Prose`, so each gate asks one question of both.

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
    cited_at: &'a str,
}

/// Every `hover_info` call: the line it sits on, the widget it names, and its
/// five arguments.
///
/// Shared by the citation gate and the omission report, which ask different
/// questions of the same five arguments.
fn panel_calls(raw: &str) -> Vec<(usize, &str, Vec<&str>)> {
    let starts = line_offsets(raw);
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(ix) = raw.get(from..).and_then(|t| t.find(".hover_info(")) {
        let call = from + ix;
        from = call + ".hover_info(".len();
        let Some(args) = call_args(raw, from - 1) else {
            continue;
        };
        if args.len() != 5 {
            continue;
        }
        let widget = unquote(args[1]).unwrap_or("?");
        out.push((line_at(&starts, call), widget, args));
    }
    out
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

/// A `claim(` call's role, field and citation, or `None` unless it has four
/// arguments and those three are string literals -- the one shape spec §3.2
/// allows, and the only one a gate can read. The value, `t.<field>`, is not
/// read: what is checked is the field named against the line cited.
fn claim_literals<'a>(args: &[&'a str]) -> Option<(&'a str, &'a str, &'a str)> {
    let &[role, field, _, cited] = args else {
        return None;
    };
    Some((unquote(role)?, unquote(field)?, unquote(cited)?))
}

/// The methods a note is written with (spec §3.2).
const NOTE_METHODS: &[&str] = &["config", "not_themeable", "instance"];

/// Every `.config(`, `.not_themeable(` and `.instance(` call in `raw` that is
/// code and has two arguments, the first a string literal: the line its text
/// starts on, the `what`, and the text as written.
///
/// The text is taken as written, not only when it is a literal: a note built
/// with `format!` cites upstream just as well, and `prose_citations` finds a
/// citation inside the literal either way.
fn note_calls(raw: &str) -> Vec<(usize, &str, &str)> {
    let starts = line_offsets(raw);
    let mut out = Vec::new();
    for name in NOTE_METHODS {
        for open in method_calls(raw, name) {
            let Some(args) = call_args(raw, open) else {
                continue;
            };
            let &[what, text] = args.as_slice() else {
                continue;
            };
            let Some(what) = unquote(what) else {
                continue;
            };
            let text = text.trim();
            out.push((line_at(&starts, offset_in(raw, text)), what, text));
        }
    }
    out.sort_by_key(|(line, _, _)| *line);
    out
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

/// A panel's prose argument: the file and line of its panel, the widget it
/// names, and the text.
type Prose<'a> = (&'a str, usize, &'a str, &'a str);

/// Every colour claim of the showcase, and every panel's prose argument.
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

/// Every colour claim of one showcase file, in either form, and every prose
/// text: a panel's prose argument and every note's text.
///
/// The test module is read too. Its claims are written to exercise the
/// model, but a claim anywhere in the showcase is a statement about upstream
/// and has to be as true as one on a page.
fn claims_in<'a>(file: &'a str, raw: &'a str) -> (Vec<Claim<'a>>, Vec<Prose<'a>>) {
    let mut claims = Vec::new();
    let mut prose = Vec::new();
    for (line, args) in claim_calls(raw) {
        let Some((role, field, cited_at)) = claim_literals(&args) else {
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
            cited_at,
        });
    }
    for (line, _, text) in note_calls(raw) {
        prose.push((file, line, enclosing_fn(raw, offset_in(raw, text)), text));
    }
    for (line, widget, args) in panel_calls(raw) {
        prose.push((file, line, widget, args[4]));

        let trimmed = args[2].trim();
        let Some(inner) = trimmed
            .strip_prefix("&[")
            .and_then(|t| t.trim_end().strip_suffix(']'))
        else {
            continue;
        };
        if inner.trim().is_empty() {
            continue;
        }
        // Wrapped so the array's entries are one depth-1 group each.
        let wrapped = format!("({inner})");
        let Some(tuples) = call_args(&wrapped, 0) else {
            continue;
        };
        for tup in tuples {
            let t = tup.trim();
            if !t.starts_with('(') {
                continue;
            }
            let Some(parts) = call_args(t, 0) else {
                continue;
            };
            if parts.len() != 4 {
                continue;
            }
            // Re-borrow from `raw`: `wrapped` is a local copy.
            let Some(field) = unquote(parts[1]).and_then(|f| find_in(raw, f)) else {
                continue;
            };
            let cited = unquote(parts[3]).unwrap_or("");
            let cited_at = if cited.is_empty() {
                ""
            } else {
                find_in(raw, cited).unwrap_or("")
            };
            claims.push(Claim {
                file,
                line,
                widget,
                role: unquote(parts[0])
                    .and_then(|r| find_in(raw, r))
                    .unwrap_or("?"),
                field,
                cited_at,
            });
        }
    }
    (claims, prose)
}

/// The same text, borrowed from `haystack` rather than from a temporary.
fn find_in<'a>(haystack: &'a str, needle: &str) -> Option<&'a str> {
    let at = haystack.find(needle)?;
    haystack.get(at..at + needle.len())
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
        !claims.is_empty(),
        "no colour claims found in the showcase, so this test would pass vacuously"
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

/// Spec section 5: the theme fields the cited files read that no panel names.
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
    let calls: Vec<(usize, &str, Vec<&str>)> = SHOWCASE_FILES
        .iter()
        .flat_map(|(_, raw)| panel_calls(raw))
        .collect();
    assert!(
        !calls.is_empty(),
        "no Widget Info panels were found, so this report would be empty for \
         the wrong reason"
    );
    let (claims, _) = showcase_claims();

    // Every file a claim cites, with the panels that cite it. An ambiguous or
    // unparseable citation is left out here; the citation gate already fails
    // on both, so this report never has to speak about them. The test
    // module's claims are left out too: they are checked, but they are not
    // what the showcase tells its reader.
    let mut cited: BTreeMap<PathBuf, BTreeSet<&str>> = BTreeMap::new();
    for claim in claims.iter().filter(|c| c.file != TEST_MODULE) {
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

    // What the panels say, all five arguments of all of them: a field named
    // in a config line or a not-themeable note is described just as much as
    // one that carries a swatch. In the newer form that is every argument of
    // every `claim(` and note call, and the geometry lines' texts.
    let mut said: Vec<&str> = calls
        .iter()
        .flat_map(|(_, _, args)| args.iter().copied())
        .collect();
    for (_, raw) in demo_files() {
        said.extend(claim_calls(raw).into_iter().flat_map(|(_, args)| args));
        said.extend(
            note_calls(raw)
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
        "\nWidget Info omission report\n  {} panels cite {} files, which read \
         {} distinct theme fields.\n  {} of those are named by no panel:",
        calls.len(),
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
        checked > 0,
        "no `file.rs, Symbol` citation found in any panel's prose, so this test \
         would pass vacuously"
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

/// The claim and note parsers see what they are meant to: a `claim(` call in
/// either of rustfmt's layouts and a note method with a literal `what`, but
/// not a mention in a comment or a string, a longer name, a declaration, a
/// free function named like a note method, or a call of another shape.
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
        .instance(label, "not read")
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
}

// ---------------------------------------------------------------------------
// No hardcoded style values in the showcase
// ---------------------------------------------------------------------------
//
// Spec §6a: a showcase demonstrates the theme, so every radius and every
// colour it paints has to come from the theme. The call shapes below are the
// ways a literal reaches a pixel: a tailwind radius helper, a radius setter
// given a number, and a colour setter given a colour constructor. Line widths
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

/// Spec §6a: no showcase paints a radius or a colour of its own invention.
#[test]
fn the_showcase_hardcodes_no_style_values() {
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
         `geometry` builder, and a colour that IS the datum goes on \
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
                  }\n";
    let found = hardcoded_style_values(source, &[]);
    assert_eq!(
        found.len(),
        6,
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
