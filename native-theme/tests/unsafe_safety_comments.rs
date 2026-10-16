//! Every `unsafe` block, fn and impl in the crate's non-test code carries a `// SAFETY:` comment
//! within the three lines above it or heading the comment block directly above it, stating the invariant it relies on.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::path::{Path, PathBuf};

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// The line with its `//` comment and string literal contents removed.
fn code_of(line: &str) -> String {
    let mut code = String::new();
    let mut in_string = false;
    let mut prev = ' ';
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_string {
            if c == '"' && prev != '\\' {
                in_string = false;
            }
        } else if c == '"' {
            in_string = true;
        } else if c == '/' && chars.peek() == Some(&'/') {
            break;
        } else {
            code.push(c);
        }
        prev = c;
    }
    code
}

fn has_unsafe_keyword(code: &str) -> bool {
    let is_ident = |c: char| c.is_alphanumeric() || c == '_';
    code.match_indices("unsafe").any(|(i, _)| {
        let before = code[..i].chars().next_back();
        let after = code[i + "unsafe".len()..].chars().next();
        !before.is_some_and(is_ident) && !after.is_some_and(is_ident)
    })
}

/// The `unsafe` lines of a file's non-test code (the file's `#[cfg(test)] mod` and everything
/// after it is skipped) with no `SAFETY:` on the same line, the next, or the three above.
fn uncommented(source: &str) -> Vec<usize> {
    let lines: Vec<&str> = source.lines().collect();
    let mut missing = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "#[cfg(test)]" {
            let item = lines
                .get(i + 1..)
                .and_then(|rest| rest.iter().find(|l| !l.trim_start().starts_with("#[")));
            if item.is_some_and(|l| l.starts_with("mod ") || l.starts_with("pub mod ")) {
                break;
            }
        }
        if !has_unsafe_keyword(&code_of(line)) {
            continue;
        }
        let from = i.saturating_sub(3);
        let to = (i + 1).min(lines.len().saturating_sub(1));
        let near = lines
            .get(from..=to)
            .is_some_and(|window| window.iter().any(|l| l.contains("SAFETY:")));
        let block = lines[..i]
            .iter()
            .rev()
            .take_while(|l| l.trim_start().starts_with("//") || l.trim_start().starts_with("#["))
            .any(|l| l.contains("SAFETY:"));
        let documented = near || block;
        if !documented {
            missing.push(i + 1);
        }
    }
    missing
}

#[test]
fn every_unsafe_has_a_safety_comment() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    rust_files(&src, &mut files);
    files.sort();
    let mut report = Vec::new();
    for file in &files {
        let source = fs::read_to_string(file).unwrap();
        for line in uncommented(&source) {
            report.push(format!(
                "{}:{line}",
                file.strip_prefix(&src).unwrap().display()
            ));
        }
    }
    assert!(
        report.is_empty(),
        "{} `unsafe` without a `// SAFETY:` comment within three lines above: {report:?}",
        report.len()
    );
}

#[test]
fn the_checker_flags_a_bare_unsafe_and_accepts_a_commented_one() {
    let bare = "fn f() {\n    let x = 1;\n    unsafe { g() };\n}\n";
    assert_eq!(uncommented(bare), vec![3]);
    let commented = "fn f() {\n    // SAFETY: g has no preconditions.\n    unsafe { g() };\n}\n";
    assert!(uncommented(commented).is_empty());
    let prose = "// the unsafe part\nlet s = \"unsafe\"; let unsafe_code = 1;\n";
    assert!(uncommented(prose).is_empty());
}
