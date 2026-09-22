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
    let demos = showcase_demos(SHOWCASE);
    let code = without_comments_or_strings(demos);
    let blocks = demo_blocks(demos, &code, &method_starts(&code));

    assert!(
        !blocks.is_empty(),
        "no `{BLOCK_ID}` found in the showcase, so this test would pass vacuously"
    );

    let missing: Vec<usize> = blocks
        .iter()
        .filter(|b| !b.has_panel)
        .map(|b| b.start + 1)
        .collect();

    assert!(
        missing.is_empty(),
        "{} of {} demo blocks in examples/showcase-gpui.rs carry no Widget Info \
         panel, so hovering them says nothing; at lines: {:?}",
        missing.len(),
        blocks.len(),
        missing
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

/// Where the showcase's own test module begins.
///
/// Everything from there on is test code: it builds widgets and calls
/// builders for reasons that have nothing to do with a demo, and the last
/// demo block would otherwise swallow all of it.
const TEST_MODULE: &str = "\n#[cfg(test)]";

/// The showcase up to its test module.
fn showcase_demos(source: &str) -> &str {
    match source.find(TEST_MODULE) {
        Some(ix) => source.get(..ix).unwrap_or(source),
        None => source,
    }
}

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
/// binding shared by several demo blocks always sits in one.
fn method_starts(code: &str) -> Vec<usize> {
    code.lines()
        .enumerate()
        .filter(|(_, l)| l.starts_with("    fn ") || l.starts_with("    pub fn "))
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
    let demos = showcase_demos(SHOWCASE);
    let code = without_comments_or_strings(demos);
    let notes = string_literals_only(demos);
    let code_lines: Vec<&str> = code.lines().collect();
    let note_lines: Vec<&str> = notes.lines().collect();

    let builders = public_fns(GEOMETRY);
    assert!(
        !builders.is_empty(),
        "no `pub fn` found in geometry.rs, so this test would pass vacuously"
    );

    let methods = method_starts(&code);
    let blocks = demo_blocks(demos, &code, &methods);
    let bindings = geometry_bindings(&code);

    let spans: Vec<(usize, usize)> = blocks.iter().map(|b| (b.start, b.end)).collect();
    let inside_a_block = |line: usize| spans.iter().any(|&(a, b)| a <= line && line < b);

    let mut findings = Vec::new();
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
            findings.push(format!("line {}: {}", block.start + 1, unnamed.join(", ")));
        }
    }

    assert!(
        findings.is_empty(),
        "{} of {} demo blocks in examples/showcase-gpui.rs are shaped by a \
         geometry builder their Widget Info panel never names, so hovering them \
         hides what the native theme did:\n  {}",
        findings.len(),
        blocks.len(),
        findings.join("\n  ")
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
    line: usize,
    widget: &'a str,
    role: &'a str,
    field: &'a str,
    cited_at: &'a str,
}

/// Every colour claim, and every panel's prose argument with its line.
fn panel_claims(raw: &str) -> (Vec<Claim<'_>>, Vec<(usize, &str, &str)>) {
    let starts = line_offsets(raw);
    let mut claims = Vec::new();
    let mut prose = Vec::new();
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
        let line = line_at(&starts, call);
        let widget = unquote(args[1]).unwrap_or("?");
        prose.push((line, widget, args[4]));

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
    let (claims, _) = panel_claims(SHOWCASE);
    assert!(
        !claims.is_empty(),
        "no colour claims found in the showcase, so this test would pass vacuously"
    );

    let mut wrong = Vec::new();
    let mut uncited = 0usize;
    for claim in &claims {
        if claim.cited_at.is_empty() {
            uncited += 1;
            continue;
        }
        // `showcase` with no line: the *application* chooses this colour, so
        // there is no upstream read to point at. A line would be worse than
        // useless -- the showcase is the file being edited, so every edit
        // above a self-citation silently invalidates it, which is exactly
        // what happened when this was first tried.
        if claim.cited_at == "showcase" {
            // Against the *code*, strings removed: the panel text names the
            // field too, so searching the file as written would let a claim
            // vouch for itself.
            if mentions(&without_comments_or_strings(SHOWCASE), claim.field) {
                continue;
            }
            wrong.push(format!(
                "{} [{}] line {}: cited as the application's own, but the \
                 showcase never sets `{}`",
                claim.widget, claim.role, claim.line, claim.field
            ));
            continue;
        }
        let Some((path, first, last)) = parse_citation(claim.cited_at) else {
            wrong.push(format!(
                "{} [{}] line {}: `{}` is not <file>.rs:<line>",
                claim.widget, claim.role, claim.line, claim.cited_at
            ));
            continue;
        };
        let files = candidates(path, &roots);
        if files.is_empty() {
            wrong.push(format!(
                "{} [{}] line {}: cites {path}, which does not exist",
                claim.widget, claim.role, claim.line
            ));
            continue;
        }
        // A line number against the wrong file means nothing, and a bare
        // name can match several crates -- `toggle.rs` is gpui-component's
        // `button/toggle.rs` and gpui-base's `toggle.rs`. The author knows
        // which one they read, so the citation has to say.
        if files.len() > 1 {
            wrong.push(format!(
                "{} [{}] line {}: `{path}` is ambiguous; name the directory too. \
                 It matches: {}",
                claim.widget,
                claim.role,
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
                "{} [{}] line {}: cites {path}, which could not be read",
                claim.widget, claim.role, claim.line
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
                "{} [{}] line {}: cites {}, but {} has {} lines",
                claim.widget,
                claim.role,
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
                "{} [{}] line {}: claims `{}` at {}\n      that line now reads: {}",
                claim.widget, claim.role, claim.line, claim.field, claim.cited_at, shown
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
    let (_, prose) = panel_claims(SHOWCASE);

    let mut missing = Vec::new();
    let mut checked = 0usize;
    for (line, widget, text) in &prose {
        for (path, symbol) in prose_citations(text) {
            checked += 1;
            let files = candidates(path, &roots);
            if files.is_empty() {
                missing.push(format!(
                    "{widget} line {line}: cites {path}, which does not exist"
                ));
                continue;
            }
            let leaf = symbol.rsplit("::").next().unwrap_or(symbol);
            let found = files
                .iter()
                .any(|f| std::fs::read_to_string(f).is_ok_and(|t| mentions(&t, leaf)));
            if !found {
                missing.push(format!(
                    "{widget} line {line}: cites {path}, {symbol} — `{leaf}` is in none of them"
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
        missing.is_empty(),
        "{} of {checked} prose citations point at something that is gone:\n  {}",
        missing.len(),
        missing.join("\n  ")
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
