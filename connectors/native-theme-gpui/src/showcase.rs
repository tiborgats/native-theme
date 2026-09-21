//! Layer 2 of the theme contracts: the showcase exercises every builder
//! (spec §6a.4).
//!
//! Test-only. A builder nobody demonstrates is a builder nobody has looked at,
//! which is how `geometry::dialog` carried a wrong radius and
//! `geometry::menu_item` a wrong doc comment for two releases. The showcase is
//! read with `include_str!` -- a path known at compile time, so no test has to
//! find the file at run time -- and so are the two modules whose surface it
//! must cover, because a list written out here would be one more thing to
//! forget: the names come from the `pub fn` lines of `geometry.rs` and
//! `variants.rs` themselves.
//!
//! What counts as exercising a builder is a reference to it by path in code,
//! with or without an argument list: the showcase passes several of them as
//! function items (`.native(cx, geometry::button)`), which is a call at one
//! remove. Comments and string literals are removed first, because the
//! showcase names builders in both -- every widget's hover note says which
//! builder shaped it -- and a note about a builder is not a use of it. That is
//! the opposite of what `scripts/check-widget-coverage.py` does with the same
//! file, and for the opposite reason: a widget is often reached through an
//! extension method and its name appears only in the section label, while a
//! builder is always called by path.

/// The showcase, and the two modules whose public surface it must cover.
const SHOWCASE: &str = include_str!("../examples/showcase-gpui.rs");
const GEOMETRY: &str = include_str!("geometry.rs");
const VARIANTS: &str = include_str!("variants.rs");

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
/// code around them untouched.
///
/// Scanning has to know where a string begins and ends either way: the
/// showcase holds URLs and a raw string of Rust source with doc comments in
/// it, and a `//` inside either is not a comment -- dropping the rest of those
/// lines would hide the code that follows them.
fn without_comments_or_strings(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut rest = source;
    while let Some(next) = rest.find(['/', '"', 'r']) {
        let (before, from) = rest.split_at(next);
        out.push_str(before);
        if let Some(after) = from.strip_prefix("//") {
            rest = match after.find('\n') {
                Some(ix) => &after[ix..],
                None => "",
            };
        } else if let Some(body) = from.strip_prefix('"') {
            rest = &body[end_of_string(body, "\"")..];
        } else if let Some(hashes) = raw_string_hashes(from) {
            let open = hashes + 2;
            let close = format!("\"{}", "#".repeat(hashes));
            rest = match from[open..].find(&close) {
                Some(ix) => &from[open + ix + close.len()..],
                None => "",
            };
        } else {
            let end = char_len(from);
            out.push_str(&from[..end]);
            rest = &from[end..];
        }
    }
    out.push_str(rest);
    out
}

/// The number of `#`s of a raw string starting at `from`, or `None` if `from`
/// does not start one.
fn raw_string_hashes(from: &str) -> Option<usize> {
    let after_r = from.strip_prefix('r')?;
    let hashes = after_r.len() - after_r.trim_start_matches('#').len();
    after_r[hashes..].starts_with('"').then_some(hashes)
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

/// Whether `haystack` references `module::name` as an identifier of its own.
///
/// The boundary check is what keeps `geometry::input` from being satisfied by
/// `geometry::input_height`.
fn references(haystack: &str, module: &str, name: &str) -> bool {
    let path = format!("{module}::{name}");
    haystack.match_indices(&path).any(|(ix, _)| {
        haystack[ix + path.len()..]
            .chars()
            .next()
            .is_none_or(|c| !c.is_alphanumeric() && c != '_')
    })
}

/// Spec §6a.4: every public `geometry` and `variants` function is exercised by
/// the showcase.
#[test]
fn the_showcase_exercises_every_builder() {
    let source = without_comments_or_strings(SHOWCASE);
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
            if !references(&source, module, name) {
                missing.push(format!("{module}::{name}"));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "{} of {checked} builders are never used in examples/showcase-gpui.rs, \
         so nothing demonstrates them: {}",
        missing.len(),
        missing.join(", ")
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

    assert!(references("geometry::input(n)", "geometry", "input"));
    assert!(!references(
        "geometry::input_height(n)",
        "geometry",
        "input"
    ));
}
