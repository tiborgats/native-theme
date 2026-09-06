//! Every bundled SVG must be traceable through `icons/SOURCES.toml`
//! (v0.5.8 spec §10.5): one `[[set]]` rule per directory, one `[[file]]`
//! entry per exception, and nothing else under `icons/`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize)]
struct Manifest {
    set: Vec<SetRule>,
    #[serde(default)]
    file: Vec<FileRule>,
}

#[derive(Deserialize)]
struct SetRule {
    name: String,
    dir: String,
    repository: String,
    #[serde(rename = "ref")]
    git_ref: String,
    path: String,
    license: String,
}

#[derive(Deserialize)]
struct FileRule {
    set: String,
    file: String,
    #[serde(rename = "ref")]
    git_ref: String,
    path: String,
    reason: String,
}

fn icons_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("icons")
}

fn manifest() -> Manifest {
    let text = std::fs::read_to_string(icons_root().join("SOURCES.toml"))
        .expect("native-theme/icons/SOURCES.toml exists");
    toml::from_str(&text).expect("SOURCES.toml parses")
}

fn svg_names(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .expect("set directory")
        .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

#[test]
fn every_directory_under_icons_has_a_set_rule_and_holds_only_svgs() {
    let m = manifest();
    let dirs: Vec<&str> = m.set.iter().map(|s| s.dir.as_str()).collect();
    let mut total = 0;
    for entry in std::fs::read_dir(icons_root()).expect("icons dir") {
        let entry = entry.expect("entry");
        if !entry.path().is_dir() {
            continue; // LICENSE-*.txt and SOURCES.toml
        }
        let dir = entry.file_name().to_string_lossy().into_owned();
        assert!(
            dirs.contains(&dir.as_str()),
            "icons/{dir} has no [[set]] rule"
        );
        for name in svg_names(&entry.path()) {
            assert!(
                name.ends_with(".svg"),
                "unexpected non-SVG file icons/{dir}/{name}"
            );
            total += 1;
        }
    }
    assert!(total > 0);
}

#[test]
fn set_rules_are_complete() {
    for s in manifest().set {
        assert!(
            icons_root().join(&s.dir).is_dir(),
            "icons/{} missing",
            s.dir
        );
        assert!(
            icons_root().join(&s.license).is_file(),
            "{} missing",
            s.license
        );
        assert!(
            s.repository.starts_with("https://github.com/"),
            "{}",
            s.repository
        );
        assert!(
            s.path.contains("{name}"),
            "set {} path has no {{name}} placeholder",
            s.name
        );
        assert!(!s.git_ref.is_empty(), "set {} has no ref", s.name);
    }
}

#[test]
fn per_file_exceptions_point_at_existing_files() {
    let m = manifest();
    for f in &m.file {
        let set = m
            .set
            .iter()
            .find(|s| s.name == f.set)
            .unwrap_or_else(|| panic!("exception {} names unknown set {}", f.file, f.set));
        assert!(
            icons_root().join(&set.dir).join(&f.file).is_file(),
            "exception file icons/{}/{} missing",
            set.dir,
            f.file
        );
        assert!(!f.reason.is_empty(), "{} has no reason", f.file);
        assert!(!f.git_ref.is_empty(), "{} has no ref", f.file);
        assert!(f.path.ends_with(".svg"), "{} path is not an svg", f.file);
    }
}

#[test]
fn file_names_follow_each_sets_convention() {
    // Lucide: kebab-case; Material Symbols: snake_case. These are the names
    // the refresh script substitutes into the upstream path.
    for s in manifest().set {
        let allowed: fn(char) -> bool = match s.name.as_str() {
            "lucide" => |c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-',
            "material" => |c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_',
            other => panic!("unknown set {other}"),
        };
        for name in svg_names(&icons_root().join(&s.dir)) {
            let stem = name.strip_suffix(".svg").expect("svg");
            assert!(
                stem.chars().all(allowed),
                "icons/{}/{name} violates the {} naming convention",
                s.dir,
                s.name
            );
        }
    }
}

#[test]
fn retired_names_are_gone() {
    for gone in [
        "lucide/close.svg",
        "lucide/window-close.svg",
        "lucide/dash.svg",
        "lucide/window-minimize.svg",
        "lucide/inspect.svg",
        "lucide/resize-corner.svg",
        "lucide/sort-ascending.svg",
        "lucide/sort-descending.svg",
        "lucide/window-maximize.svg",
        "lucide/window-restore.svg",
        "lucide/trash-2.svg",
        "material/star_border.svg",
    ] {
        assert!(
            !icons_root().join(gone).exists(),
            "{gone} should have been removed"
        );
    }
    for present in [
        "lucide/trash.svg",
        "lucide/github.svg",
        "material/star_fill1.svg",
    ] {
        assert!(
            icons_root().join(present).is_file(),
            "{present} should exist"
        );
    }
}
