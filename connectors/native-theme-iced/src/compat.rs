//! The README's compatibility claims, against the manifest that makes them
//! true.
//!
//! Test-only. A version written in prose is a claim nobody runs: the gpui
//! connector's README said `gpui-component 0.6.x` while its manifest required
//! 0.6.4, and it said so for as long as it took someone to read both files
//! side by side. The table in the README's **Compatibility** section and the
//! `[dependencies]` of `Cargo.toml` are read here with `include_str!` -- paths
//! known at compile time, so no test has to find them at run time -- and
//! compared in both directions: a floor the README misstates fails, and so
//! does a floor it leaves out.
//!
//! The manifest is the authority. Nothing in this module carries a version of
//! its own; `REQUIRED` names the crates whose floor the README must state, and
//! every number comes from a file.

use std::collections::{BTreeMap, BTreeSet};

/// The README carrying the claims, and the manifests that answer for them.
const README: &str = include_str!("../README.md");
const MANIFEST: &str = include_str!("../Cargo.toml");
const WORKSPACE: &str = include_str!("../../../Cargo.toml");

/// The stamp `scripts/compat-check.sh run` writes when every gate passed on
/// the upstream set it resolved, and the name of the table that speaks for
/// this crate -- the package's own, so a rename cannot leave the test looking
/// at nothing.
const STAMP: &str = include_str!("../../../docs/COMPATIBILITY.toml");
const CRATE: &str = env!("CARGO_PKG_NAME");

/// The upstream crates whose floor the README's Compatibility table states.
///
/// `iced_widget` and `iced_aw` are optional, one on by default and one off,
/// and a floor holds wherever the feature that pulls the crate in is enabled.
/// `iced` is a dev-dependency: it is what the showcase builds against, and
/// what an application depends on beside this connector.
const REQUIRED: &[&str] = &["iced", "iced_aw", "iced_core", "iced_widget"];

/// The row the table states for the toolchain, beside the crates.
const RUST_VERSION: &str = "rust-version";

/// What this crate requires, by the key the README's table names each floor
/// with: a package name, or `rust-version`.
fn manifest_floors() -> BTreeMap<&'static str, String> {
    let stated = manifest_versions(MANIFEST);
    let mut floors = BTreeMap::new();
    for name in REQUIRED {
        // None when the crate was renamed or dropped, more than one when the
        // dependency and the dev-dependency disagree: the README can state
        // exactly one floor, so both are findings here rather than a row the
        // test quietly stops checking.
        let versions: Vec<&String> = stated.get(*name).into_iter().flatten().collect();
        assert_eq!(
            versions.len(),
            1,
            "Cargo.toml states {} versions for `{name}` ({versions:?}), and the README's \
             Compatibility table states one",
            versions.len()
        );
        if let Some(version) = versions.first() {
            floors.insert(*name, (*version).clone());
        }
    }
    let msrv = rust_version(MANIFEST, WORKSPACE);
    assert!(
        msrv.is_some(),
        "neither Cargo.toml nor the workspace manifest states a `rust-version`"
    );
    if let Some(version) = msrv {
        floors.insert(RUST_VERSION, version);
    }
    floors
}

/// Every dependency version a manifest states, by package name: the
/// `[dependencies]`, `[dev-dependencies]` and `[target.'cfg(..)'.*]` tables
/// alike, since a crate required on one platform is required.
///
/// A name can be stated more than once -- `iced_aw` is both a dependency and a
/// dev-dependency -- so the versions are collected as a set and the caller
/// decides what more than one of them means.
fn manifest_versions(manifest: &str) -> BTreeMap<String, BTreeSet<String>> {
    let mut found: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut in_dependencies = false;
    for line in manifest.lines() {
        if line.starts_with('[') {
            in_dependencies = line.trim_end().ends_with("dependencies]");
            continue;
        }
        if !in_dependencies {
            continue;
        }
        if let Some((package, version)) = dependency_on(line) {
            found.entry(package).or_default().insert(version);
        }
    }
    found
}

/// The package name and version a dependency line states, if it states a
/// version of its own.
///
/// Three shapes appear in this workspace: `iced_core = "0.14"`,
/// `iced_widget = { version = "0.14", optional = true }` and
/// `gpui = { package = "gpui-pre", version = "0.3.5" }`, whose package name is
/// not its key. A dependency inherited from the workspace
/// (`native-theme = { workspace = true }`) states no version and is not one of
/// these floors.
fn dependency_on(line: &str) -> Option<(String, String)> {
    let line = line.trim_start();
    if line.starts_with('#') {
        return None;
    }
    let (key, value) = line.split_once(" = ")?;
    if key.contains('.') {
        // `native-theme.workspace = true`
        return None;
    }
    let version = quoted_after(value, "version = ").or_else(|| quoted(value))?;
    let package = quoted_after(value, "package = ").unwrap_or_else(|| key.to_string());
    Some((package, version))
}

/// The Rust version the manifest requires, `rust-version.workspace = true`
/// resolved against the workspace manifest.
fn rust_version(manifest: &str, workspace: &str) -> Option<String> {
    if let Some(stated) = table_value(manifest, "[package]", RUST_VERSION) {
        return Some(stated);
    }
    let inherits = section(manifest, "[package]")
        .iter()
        .any(|line| line.trim_start().starts_with("rust-version.workspace"));
    if inherits {
        return table_value(workspace, "[workspace.package]", RUST_VERSION);
    }
    None
}

/// The string value of `key` in a manifest table.
fn table_value(manifest: &str, header: &str, key: &str) -> Option<String> {
    section(manifest, header)
        .into_iter()
        .filter_map(|line| line.split_once(" = "))
        .find(|(stated, _)| stated.trim() == key)
        .and_then(|(_, value)| quoted(value))
}

/// The lines of a manifest table, its header excluded.
fn section<'a>(manifest: &'a str, header: &str) -> Vec<&'a str> {
    let mut lines = Vec::new();
    let mut inside = false;
    for line in manifest.lines() {
        if line.starts_with('[') {
            inside = line.trim_end() == header;
            continue;
        }
        if inside {
            lines.push(line);
        }
    }
    lines
}

/// The string literal `value` opens with, if it opens with one.
fn quoted(value: &str) -> Option<String> {
    let body = value.trim_start().strip_prefix('"')?;
    let end = body.find('"')?;
    body.get(..end).map(str::to_string)
}

/// The string literal that follows `key` inside `value`.
fn quoted_after(value: &str, key: &str) -> Option<String> {
    let at = value.find(key)?;
    quoted(value.get(at + key.len()..)?)
}

/// The rows of the README's Compatibility table, as (line number, key,
/// version).
///
/// The key is the first backticked word of the first cell, which is the
/// package name or the manifest key the row states a floor for; the rest of
/// the cell says where it comes from and is prose. The header and separator
/// rows carry no backticks and fall out here.
fn required_rows(readme: &str) -> Vec<(usize, &str, &str)> {
    let mut rows = Vec::new();
    let mut inside = false;
    for (ix, line) in readme.lines().enumerate() {
        if line.starts_with("## ") {
            inside = line.trim_end() == "## Compatibility";
            continue;
        }
        if !inside || !line.starts_with('|') {
            continue;
        }
        let mut cells = line.split('|').skip(1);
        let (first, second) = match (cells.next(), cells.next()) {
            (Some(first), Some(second)) => (first, second),
            _ => continue,
        };
        if let Some(key) = backticked(first) {
            rows.push((ix + 1, key, second.trim()));
        }
    }
    rows
}

/// The first backticked word of a table cell.
fn backticked(cell: &str) -> Option<&str> {
    let body = cell.split_once('`')?.1;
    let end = body.find('`')?;
    body.get(..end)
}

/// The sentence `scripts/compat-check.sh run` writes between the README's
/// compat markers, rendered a second time from the stamp that same run wrote.
///
/// Rendering it again rather than storing it is what makes a README edited by
/// hand a failure instead of a difference nobody looks for: the versions and
/// the date have one source, and it is the run.
fn verified_sentence(stamp: &str, crate_name: &str) -> Option<String> {
    let generated = table_value(stamp, &format!("[{crate_name}]"), "generated")?;
    let pairs: Vec<String> = section(stamp, &format!("[{crate_name}.verified]"))
        .into_iter()
        .filter_map(|line| line.split_once(" = "))
        .filter_map(|(name, value)| Some(format!("{} {}", name.trim(), quoted(value)?)))
        .collect();
    let listed = match pairs.split_last()? {
        (last, []) => last.clone(),
        (last, rest) => format!("{} and {last}", rest.join(", ")),
    };
    Some(format!("Verified against {listed} on {generated}."))
}

/// The line the README carries between its compat markers, with its line
/// number.
fn verified_line(readme: &str) -> Option<(usize, &str)> {
    let mut lines = readme.lines().enumerate();
    lines.find(|(_, line)| line.trim_end() == "<!-- compat:begin -->")?;
    let (ix, line) = lines.next()?;
    (line.trim_end() != "<!-- compat:end -->").then_some((ix + 1, line))
}

/// The README's **Required** table states this crate's own floors, all of them
/// and nothing else.
#[test]
fn the_readme_states_the_manifest_floors() {
    let floors = manifest_floors();
    let rows = required_rows(README);
    assert!(
        !rows.is_empty(),
        "no rows found in the README's `## Compatibility` table, so this test would pass vacuously"
    );

    let mut findings = Vec::new();
    for (line, key, stated) in &rows {
        match floors.get(key) {
            Some(required) if required == stated => {}
            Some(required) => findings.push(format!(
                "README.md:{line}: `{key}` is stated as {stated}, Cargo.toml requires {required}"
            )),
            None => findings.push(format!(
                "README.md:{line}: `{key}` is no floor this crate's Cargo.toml states"
            )),
        }
    }
    for (key, required) in &floors {
        if !rows.iter().any(|(_, stated, _)| stated == key) {
            findings.push(format!(
                "Cargo.toml requires `{key}` {required}, and no row of the README's Compatibility table says so"
            ));
        }
    }
    assert!(
        findings.is_empty(),
        "the README's Compatibility table disagrees with Cargo.toml in {} place(s):\n{}",
        findings.len(),
        findings.join("\n")
    );
}

/// The README's **Verified** line is `docs/COMPATIBILITY.toml`'s entry for
/// this crate, so a line written by hand -- or one left behind when the stamp
/// moved on -- is a failure and not a quiet difference.
#[test]
fn the_verified_line_is_the_stamps() {
    let expected = verified_sentence(STAMP, CRATE);
    assert!(
        expected.is_some(),
        "docs/COMPATIBILITY.toml carries no `generated` date or no `[{CRATE}.verified]` \
         versions for {CRATE}; `scripts/compat-check.sh run` writes both"
    );
    let found = verified_line(README);
    assert!(
        found.is_some(),
        "the README carries no line between `<!-- compat:begin -->` and `<!-- compat:end -->`; \
         `scripts/compat-check.sh run` writes it there"
    );
    if let (Some(expected), Some((line, stated))) = (expected, found) {
        assert_eq!(
            stated,
            expected.as_str(),
            "README.md:{line} states a verified set that is not docs/COMPATIBILITY.toml's; \
             one run writes both, and neither is edited by hand"
        );
    }
}

/// Both halves of the test above have to be real: a stamp parser that found no
/// versions, or a marker reader that found no line, would agree with any
/// README at all.
#[test]
fn the_stamp_and_marker_parsers_do_their_jobs() {
    let stamp = "[a-crate]\n\
                 commit = \"0000000\"\n\
                 generated = \"2026-09-21\"\n\
                 sources = \"abc\"\n\
                 \n\
                 [a-crate.verified]\n\
                 one = \"1.0\"\n\
                 two = \"2.0\"\n\
                 three = \"3.0\"\n\
                 \n\
                 [b-crate]\n\
                 generated = \"2026-01-01\"\n\
                 \n\
                 [b-crate.verified]\n\
                 only = \"9.9\"\n";
    assert_eq!(
        verified_sentence(stamp, "a-crate").as_deref(),
        Some("Verified against one 1.0, two 2.0 and three 3.0 on 2026-09-21.")
    );
    // One version takes no "and", and a table belongs to the crate it is under.
    assert_eq!(
        verified_sentence(stamp, "b-crate").as_deref(),
        Some("Verified against only 9.9 on 2026-01-01.")
    );
    assert_eq!(verified_sentence(stamp, "c-crate"), None);

    let readme = "before\n\
                  <!-- compat:begin -->\n\
                  Verified against x 1.0 on 2026-09-21.\n\
                  <!-- compat:end -->\n\
                  after\n";
    assert_eq!(
        verified_line(readme),
        Some((3, "Verified against x 1.0 on 2026-09-21."))
    );
    assert_eq!(verified_line("no markers here\n"), None);
    assert_eq!(
        verified_line("<!-- compat:begin -->\n<!-- compat:end -->\n"),
        None,
        "an empty marker pair states nothing, and nothing is not a claim to accept"
    );
}

/// Both halves of the floors test have to be real: a manifest parser that
/// found no versions, or a table parser that found no rows, would let a false
/// floor through without a word.
#[test]
fn the_manifest_and_table_parsers_do_their_jobs() {
    let manifest = "[package]\n\
                    name = \"c\"\n\
                    rust-version = \"1.95.0\"\n\
                    \n\
                    [dependencies]\n\
                    # iced_core = \"0.0.1\"\n\
                    plain = \"1.2\"\n\
                    inline = { version = \"3.4\", optional = true }\n\
                    renamed = { package = \"upstream-pre\", version = \"0.3.5\" }\n\
                    inherited = { workspace = true }\n\
                    dotted.workspace = true\n\
                    \n\
                    [target.'cfg(target_os = \"linux\")'.dev-dependencies]\n\
                    plain = \"1.2\"\n\
                    only-there = \"9.9\"\n\
                    \n\
                    [features]\n\
                    default = [\"plain\"]\n";
    let versions = manifest_versions(manifest);
    let one = |name: &str| {
        versions
            .get(name)
            .map(|set| set.iter().cloned().collect::<Vec<_>>().join(","))
    };
    assert_eq!(one("plain").as_deref(), Some("1.2"));
    assert_eq!(one("inline").as_deref(), Some("3.4"));
    assert_eq!(one("only-there").as_deref(), Some("9.9"));
    // The package name, not the key it is written under.
    assert_eq!(one("upstream-pre").as_deref(), Some("0.3.5"));
    assert_eq!(one("renamed"), None);
    // No version of its own, a comment, and a `[features]` entry.
    assert_eq!(one("inherited"), None);
    assert_eq!(one("dotted"), None);
    assert_eq!(one("iced_core"), None);
    assert_eq!(one("default"), None);

    assert_eq!(rust_version(manifest, "").as_deref(), Some("1.95.0"));
    let inheriting = "[package]\nrust-version.workspace = true\n";
    let workspace = "[workspace.package]\nrust-version = \"1.88.0\"\n";
    assert_eq!(
        rust_version(inheriting, workspace).as_deref(),
        Some("1.88.0")
    );
    assert_eq!(rust_version("[package]\nname = \"c\"\n", workspace), None);

    let readme = "## Compatibility\n\
                  \n\
                  | Crate | Required |\n\
                  |---|---|\n\
                  | `alpha` | 0.6.4 |\n\
                  | `beta` (dev-dependency: the showcase) | 0.3.5 |\n\
                  | `rust-version` | 1.95.0 |\n\
                  \n\
                  ## Quick start\n\
                  \n\
                  | `gamma` | 9.9 |\n";
    assert_eq!(
        required_rows(readme),
        vec![
            (5, "alpha", "0.6.4"),
            (6, "beta", "0.3.5"),
            (7, "rust-version", "1.95.0"),
        ],
        "the header, the separator and a table in another section are not rows of this one"
    );
}
