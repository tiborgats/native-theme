//! S11's reader over the fixtures in `tests/fixtures/next/layers/`: each file
//! states its expected problems in its header comments, `# problem: <path>:
//! <kind>` (or `# problem-prefix: <start>` where the text is the TOML
//! parser's), every one of them reported and no other.

#![allow(clippy::expect_used, clippy::panic)]

use std::fs;
use std::path::Path;

use native_theme::next::{Layer, LayerError};

#[test]
fn layer_fixtures_report_their_expected_problems() {
    let folder = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/next/layers");
    let mut paths: Vec<_> = fs::read_dir(&folder)
        .expect("the fixtures folder")
        .map(|entry| entry.expect("a fixture").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "toml"))
        .collect();
    paths.sort();
    assert!(paths.len() >= 8, "fixtures missing: {paths:?}");
    for path in paths {
        let text = fs::read_to_string(&path).expect("a readable fixture");
        let mut expected: Vec<&str> = text
            .lines()
            .filter_map(|line| line.strip_prefix("# problem: "))
            .collect();
        let prefixes: Vec<&str> = text
            .lines()
            .filter_map(|line| line.strip_prefix("# problem-prefix: "))
            .collect();
        let mut found: Vec<String> = match Layer::from_toml(&text) {
            Ok(_) => Vec::new(),
            Err(LayerError::Invalid(problems)) => {
                problems.iter().map(ToString::to_string).collect()
            }
            Err(other) => vec![other.to_string()],
        };
        let name = path.display();
        if prefixes.is_empty() {
            expected.sort_unstable();
            found.sort();
            assert_eq!(found, expected, "{name}");
        } else {
            assert_eq!(found.len(), prefixes.len(), "{name}: {found:?}");
            for (found, prefix) in found.iter().zip(&prefixes) {
                assert!(found.starts_with(prefix), "{name}: {found:?}");
            }
        }
    }
}
