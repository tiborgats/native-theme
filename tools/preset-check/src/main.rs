//! Dev-only preset check (never published): loads a preset as a bundled
//! preset loads, through the synthetic route where its `[pending]` table
//! lists paths (S17, ADD-A128), under each scheme, and prints every problem
//! in the reader's and validation's own words, grouped by class. For the
//! facts lanes, which run no cargo: a preset that would not load is seen
//! before it is bundled.
//!
//! Usage: `preset-check <preset.toml>`, `preset-check -` (the preset on
//! stdin) or `preset-check --draft <id>`, which loads what
//! `python3 scripts/generate_preset.py <id>` prints, run in the current
//! directory (a repository root). That draft has the hand-written parts'
//! blocks empty (`[preset] generation`, the `[declare]` keyed namespaces,
//! the `read` facts' bases), and the reader stops at the first of them
//! (`preset.generation: missing`), before validation: for a bundled preset,
//! `python3 scripts/generate_preset.py <id> --write` in the worktree carries
//! its hand-written parts into the file, which `preset-check <file>` loads.
//!
//! Exit 0 when the preset loads under every scheme, 1 otherwise: a problem,
//! a file or draft that cannot be read, or a wrong command line.
//!
//! Built once in the main checkout, then run by path from any worktree:
//!
//! ```text
//! cargo build -p native-theme-preset-check            # in /home/tibi/Rust/native-theme
//! cd <worktree>
//! python3 scripts/generate_preset.py ios-26 --write
//! /home/tibi/Rust/native-theme/target/debug/preset-check native-theme/src/presets/ios-26.toml
//! /home/tibi/Rust/native-theme/target/debug/preset-check --draft ios-26
//! ```
//!
//! A preset whose `id` names a bundled preset loads as that preset, so its
//! `preset:` conditions hold; any other loads as a preset no `PresetId`
//! names, as the facts gate's fixtures do, and the report says so.

#![deny(unsafe_code)]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::unwrap_in_result,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::todo,
    clippy::unimplemented,
    clippy::unreachable,
    clippy::manual_assert,
    clippy::indexing_slicing,
    clippy::string_slice,
    clippy::arithmetic_side_effects,
    clippy::integer_division,
    clippy::modulo_arithmetic,
    clippy::exit,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

use native_theme::gate::{self, Refusal};
use native_theme::{LayerError, PresetId, Scheme};

const USAGE: &str = "usage: preset-check <preset.toml> | - | --draft <id>   (-: the preset on stdin; --draft: python3 scripts/generate_preset.py <id>'s output, run from a repository root)";

/// Where the preset's text comes from.
#[derive(Debug, PartialEq, Eq)]
enum Source {
    /// A preset file.
    File(PathBuf),
    /// Standard input.
    Stdin,
    /// `generate_preset.py <id>`'s draft.
    Draft(String),
}

impl Source {
    /// The command line's source (the arguments after the program's name).
    fn parse(args: &[String]) -> Result<Source, String> {
        match args {
            [flag, id] if flag == "--draft" => Ok(Source::Draft(id.clone())),
            [dash] if dash == "-" => Ok(Source::Stdin),
            [path] if !path.starts_with('-') => Ok(Source::File(PathBuf::from(path))),
            _ => Err(USAGE.to_owned()),
        }
    }

    /// The source as the report names it.
    fn name(&self) -> String {
        match self {
            Source::File(path) => path.display().to_string(),
            Source::Stdin => "stdin".to_owned(),
            Source::Draft(id) => format!("generate_preset.py {id}'s draft"),
        }
    }

    /// The preset's text.
    fn read(&self) -> Result<String, String> {
        match self {
            Source::File(path) => std::fs::read_to_string(path)
                .map_err(|error| format!("{}: {error}", path.display())),
            Source::Stdin => {
                let mut text = String::new();
                std::io::stdin()
                    .read_to_string(&mut text)
                    .map_err(|error| format!("stdin: {error}"))?;
                Ok(text)
            }
            Source::Draft(id) => {
                let output = Command::new("python3")
                    .args(["scripts/generate_preset.py", id])
                    .output()
                    .map_err(|error| format!("python3 scripts/generate_preset.py {id}: {error}"))?;
                if output.stdout.is_empty() {
                    return Err(format!(
                        "python3 scripts/generate_preset.py {id} printed no draft ({}):\n{}",
                        output.status,
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
                String::from_utf8(output.stdout)
                    .map_err(|error| format!("generate_preset.py {id}'s draft: {error}"))
            }
        }
    }
}

/// One problem, in its own words, with its class and the schemes it was
/// found under.
#[derive(Debug, PartialEq, Eq)]
struct Found {
    class: String,
    text: String,
    schemes: Vec<&'static str>,
}

/// A variant's name from its `Debug` text: the identifier it starts with.
fn variant(debug: &str) -> String {
    debug
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

/// The problems of a refusal, each with its class.
fn classified(refusal: &Refusal) -> Vec<(String, String)> {
    match refusal {
        Refusal::Layer(LayerError::Invalid(problems)) => problems
            .iter()
            .map(|problem| (variant(&format!("{:?}", problem.kind)), problem.to_string()))
            .collect(),
        Refusal::Layer(other) => vec![(variant(&format!("{other:?}")), other.to_string())],
        Refusal::Synthetic(why) => vec![("SyntheticRoute".to_owned(), why.clone())],
    }
}

/// The preset `text` loaded under each scheme: the bundled preset it
/// loaded as (under the last scheme that loaded), and every problem found,
/// once, in the order first found.
fn check(text: &str) -> (Option<PresetId>, Vec<Found>) {
    let mut id = None;
    let mut found: Vec<Found> = Vec::new();
    for scheme in Scheme::ALL {
        match gate::load(text, scheme) {
            Ok(loaded) => id = loaded,
            Err(refusal) => {
                for (class, text) in classified(&refusal) {
                    match found
                        .iter_mut()
                        .find(|f| f.class == class && f.text == text)
                    {
                        Some(f) => f.schemes.push(scheme.name()),
                        None => found.push(Found {
                            class,
                            text,
                            schemes: vec![scheme.name()],
                        }),
                    }
                }
            }
        }
    }
    (id, found)
}

/// The report: a line naming the source and the outcome, then each class
/// with its count and its problems, a problem found under some schemes
/// only naming them.
fn report(source: &str, id: Option<PresetId>, found: &[Found]) -> String {
    let mut out = String::new();
    if found.is_empty() {
        out.push_str(&format!(
            "preset-check: {source}: loads ({})\n",
            as_what(id)
        ));
        return out;
    }
    out.push_str(&format!(
        "preset-check: {source}: does not load, {} problem(s)\n",
        found.len()
    ));
    let mut classes: Vec<&str> = Vec::new();
    for f in found {
        if !classes.contains(&f.class.as_str()) {
            classes.push(&f.class);
        }
    }
    for class in classes {
        let members: Vec<&Found> = found.iter().filter(|f| f.class == class).collect();
        out.push_str(&format!("== {class}: {}\n", members.len()));
        for f in members {
            if f.schemes.len() == Scheme::ALL.len() {
                out.push_str(&format!("  {}\n", f.text));
            } else {
                out.push_str(&format!("  {} ({} only)\n", f.text, f.schemes.join(", ")));
            }
        }
    }
    out
}

/// How the preset loaded: as a bundled preset or as one no `PresetId`
/// names.
fn as_what(id: Option<PresetId>) -> String {
    match id {
        Some(id) => format!("as the bundled preset {}", id.key()),
        None => "as a preset no bundled preset's id names, its `preset:` conditions not holding"
            .to_owned(),
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let source = match Source::parse(&args) {
        Ok(source) => source,
        Err(usage) => {
            eprintln!("{usage}");
            return ExitCode::FAILURE;
        }
    };
    let text = match source.read() {
        Ok(text) => text,
        Err(error) => {
            eprintln!("preset-check: {error}");
            return ExitCode::FAILURE;
        }
    };
    let (id, found) = check(&text);
    print!("{}", report(&source.name(), id, &found));
    if found.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_line() {
        let args = |list: &[&str]| list.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
        assert_eq!(
            Source::parse(&args(&["a.toml"])),
            Ok(Source::File(PathBuf::from("a.toml")))
        );
        assert_eq!(Source::parse(&args(&["-"])), Ok(Source::Stdin));
        assert_eq!(
            Source::parse(&args(&["--draft", "ios-26"])),
            Ok(Source::Draft("ios-26".to_owned()))
        );
        assert!(Source::parse(&args(&[])).is_err());
        assert!(Source::parse(&args(&["--draft"])).is_err());
        assert!(Source::parse(&args(&["--other"])).is_err());
        assert!(Source::parse(&args(&["a.toml", "b.toml"])).is_err());
    }

    #[test]
    fn variant_names() {
        assert_eq!(variant("UnknownKey"), "UnknownKey");
        assert_eq!(variant("Mistyped { expected: \"a number\" }"), "Mistyped");
        assert_eq!(variant("Toml(\"x\")"), "Toml");
    }
}
