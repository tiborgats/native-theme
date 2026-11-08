//! `preset-check` run as the lanes run it: a preset file that loads exits 0,
//! one that does not exits 1 and names each problem under its class.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::Path;
use std::process::Command;

fn run(path: &Path) -> (i32, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_preset-check"))
        .arg(path)
        .output()
        .unwrap();
    (
        output.status.code().unwrap(),
        String::from_utf8(output.stdout).unwrap(),
    )
}

/// A bundled preset loads, as the bundled preset its id names.
#[test]
fn bundled_preset_loads() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../native-theme/src/presets/adwaita.toml");
    let (code, out) = run(&path);
    assert_eq!(code, 0, "{out}");
    assert!(
        out.contains("loads (as the bundled preset adwaita)"),
        "{out}"
    );
}

/// A preset stating an unknown key fails, the key named under its class.
#[test]
fn unknown_key_named() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/unknown_key.toml");
    let (code, out) = run(&path);
    assert_eq!(code, 1, "{out}");
    assert!(out.contains("does not load"), "{out}");
    let class = out
        .split("== ")
        .find(|section| section.starts_with("UnknownKey: "))
        .unwrap_or_else(|| panic!("no UnknownKey class: {out}"));
    assert!(class.contains("preset.not_a_key"), "{out}");
}
