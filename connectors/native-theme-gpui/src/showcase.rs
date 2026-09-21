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

// ---------------------------------------------------------------------------
// Every demo block carries a Widget Info panel
// ---------------------------------------------------------------------------
//
// Widget-info spec section 2.2. A demo block is the element chain rooted at
// `div().id("tt-<slug>")`; it runs to the next such id, or to the end of the
// file. A block with no `.on_hover(self.hover_info(` is a widget a reader can
// hover and learn nothing from.
//
// The boundaries are read from `SHOWCASE` itself and not from the stripped
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
    for (n, line) in SHOWCASE.lines().enumerate() {
        let Some(rest) = line.trim_start().strip_prefix("id: \"") else {
            continue;
        };
        let Some(id) = rest.split('"').next() else {
            continue;
        };
        checked += 1;
        if !id.starts_with("tt-") {
            wrong.push(format!("{}: {id}", n + 1));
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
    let stripped = without_comments_or_strings(SHOWCASE);
    let body: Vec<&str> = stripped.lines().collect();
    let starts = demo_block_starts(SHOWCASE);

    assert!(
        !starts.is_empty(),
        "no `{BLOCK_ID}` found in the showcase, so this test would pass vacuously"
    );

    let mut missing = Vec::new();
    for (i, &start) in starts.iter().enumerate() {
        let end = starts.get(i + 1).copied().unwrap_or(body.len());
        let (from, to) = (start.min(body.len()), end.min(body.len()));
        if !body[from..to].iter().any(|l| l.contains(PANEL_CALL)) {
            missing.push(start + 1);
        }
    }

    assert!(
        missing.is_empty(),
        "{} of {} demo blocks in examples/showcase-gpui.rs carry no Widget Info \
         panel, so hovering them says nothing; at lines: {:?}",
        missing.len(),
        starts.len(),
        missing
    );
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
    let found = hardcoded_style_values(SHOWCASE, ALLOWED_STYLE_LITERALS);
    assert!(
        found.is_empty(),
        "examples/showcase-gpui.rs paints {} value(s) the theme did not give it; \
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
