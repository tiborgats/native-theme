use std::fs;
use std::path::{Path, PathBuf};

use native_theme_build::IconGenerator;

const SVG_STUB: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"></svg>"#;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn create_temp_dir(suffix: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "native_theme_integ_{suffix}_{}",
        std::process::id()
    ));
    if let Err(e) = fs::remove_dir_all(&dir) {
        eprintln!("test cleanup warning: {e}");
    }
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let dest = dst.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir_recursive(&entry.path(), &dest);
        } else {
            fs::copy(entry.path(), dest).unwrap();
        }
    }
}

fn write_file(dir: &Path, path: &str, content: &str) {
    let full_path = dir.join(path);
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(full_path, content).unwrap();
}

/// Helper: run the pipeline on a single fixture TOML file.
fn generate_fixture(toml_path: &Path) -> native_theme_build::GenerateOutput {
    let out = create_temp_dir("fixture_out");
    IconGenerator::new()
        .source(toml_path)
        .output_dir(&out)
        .generate()
        .unwrap_or_else(|e| panic!("expected no errors: {e}"))
}

// =============================================================================
// Happy path: full pipeline on committed fixtures
// =============================================================================

#[test]
fn happy_path_generates_correct_enum() {
    let output = generate_fixture(&fixtures_dir().join("sample-icons.toml"));

    assert!(!output.code.is_empty(), "expected generated code");

    // Check enum name is PascalCase of "sample-icon"
    assert!(
        output.code.contains("pub enum SampleIcon"),
        "should have PascalCase enum name. code:\n{}",
        output.code
    );

    // Check variants
    assert!(
        output.code.contains("PlayPause"),
        "should have PlayPause variant"
    );
    assert!(
        output.code.contains("SkipForward"),
        "should have SkipForward variant"
    );
}

#[test]
fn happy_path_has_icon_provider_impl() {
    let output = generate_fixture(&fixtures_dir().join("sample-icons.toml"));

    assert!(
        output
            .code
            .contains("impl native_theme::theme::IconProvider for SampleIcon"),
        "should have IconProvider impl"
    );
}

#[test]
fn happy_path_icon_name_material() {
    let output = generate_fixture(&fixtures_dir().join("sample-icons.toml"));

    assert!(
        output.code.contains(
            "(Self::PlayPause, native_theme::theme::IconSet::Material) => Some(\"play_pause\")"
        ),
        "should have Material icon_name arm for PlayPause"
    );
    assert!(
        output.code.contains(
            "(Self::SkipForward, native_theme::theme::IconSet::Material) => Some(\"skip_next\")"
        ),
        "should have Material icon_name arm for SkipForward"
    );
}

#[test]
fn happy_path_icon_name_sf_symbols() {
    let output = generate_fixture(&fixtures_dir().join("sample-icons.toml"));

    assert!(
        output.code.contains(
            "(Self::PlayPause, native_theme::theme::IconSet::SfSymbols) => Some(\"play.fill\")"
        ),
        "should have SfSymbols icon_name arm for PlayPause"
    );
    assert!(
        output.code.contains(
            "(Self::SkipForward, native_theme::theme::IconSet::SfSymbols) => Some(\"forward.fill\")"
        ),
        "should have SfSymbols icon_name arm for SkipForward"
    );
}

#[test]
fn happy_path_icon_svg_bundled_only() {
    let output = generate_fixture(&fixtures_dir().join("sample-icons.toml"));

    // Material (bundled) should have include_bytes! arms
    assert!(
        output.code.contains("include_bytes!"),
        "should have include_bytes! for bundled material SVGs"
    );
    assert!(
        output.code.contains("material/play_pause.svg"),
        "should reference material/play_pause.svg in include_bytes! path"
    );

    // sf-symbols (system) should NOT have include_bytes! arms
    assert!(
        !output.code.contains("SfSymbols) => Some(include_bytes!"),
        "system themes should not have include_bytes! arms"
    );
}

#[test]
fn happy_path_has_const_all() {
    let output = generate_fixture(&fixtures_dir().join("sample-icons.toml"));

    assert!(
        output.code.contains("pub const ALL: &[Self]"),
        "should have const ALL"
    );
    assert!(
        output.code.contains("Self::PlayPause"),
        "ALL should contain PlayPause variant"
    );
    assert!(
        output.code.contains("Self::SkipForward"),
        "ALL should contain SkipForward variant"
    );
}

#[test]
fn happy_path_output_filename() {
    let output = generate_fixture(&fixtures_dir().join("sample-icons.toml"));

    assert_eq!(
        output.output_path.file_name().unwrap().to_str().unwrap(),
        "sample_icon.rs"
    );
}

#[test]
fn happy_path_size_report() {
    let output = generate_fixture(&fixtures_dir().join("sample-icons.toml"));

    assert_eq!(output.role_count, 2);
    assert_eq!(output.bundled_theme_count, 1);
    assert_eq!(output.svg_count, 2);
    assert!(output.total_svg_bytes > 0);
}

// =============================================================================
// Error paths: use temp dirs with intentional errors
// =============================================================================

#[test]
fn error_missing_role_in_mapping() {
    let dir = create_temp_dir("missing_role");
    write_file(
        &dir,
        "icons.toml",
        r#"
name = "test"
roles = ["play-pause", "skip-forward"]
bundled-themes = ["material"]
"#,
    );
    // Mapping is missing skip-forward
    write_file(
        &dir,
        "material/mapping.toml",
        "play-pause = \"play_pause\"\n",
    );
    write_file(&dir, "material/play_pause.svg", SVG_STUB);

    let result = IconGenerator::new()
        .source(dir.join("icons.toml"))
        .output_dir(&dir)
        .generate();

    let errors = result.unwrap_err();
    assert!(!errors.is_empty(), "should have errors");
    assert!(
        errors
            .errors()
            .iter()
            .any(|e| matches!(e, native_theme_build::BuildError::MissingRole { role, .. } if role == "skip-forward")),
        "should have MissingRole for 'skip-forward': {errors:?}",
    );

    if let Err(e) = fs::remove_dir_all(&dir) {
        eprintln!("test cleanup warning: {e}");
    }
}

#[test]
fn error_unknown_role_in_mapping() {
    let dir = create_temp_dir("unknown_role");
    write_file(
        &dir,
        "icons.toml",
        r#"
name = "test"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
    );
    // Mapping has extra role "bluetooth" not in master roles
    write_file(
        &dir,
        "material/mapping.toml",
        "play-pause = \"play_pause\"\nbluetooth = \"bluetooth\"\n",
    );
    write_file(&dir, "material/play_pause.svg", SVG_STUB);

    let result = IconGenerator::new()
        .source(dir.join("icons.toml"))
        .output_dir(&dir)
        .generate();

    let errors = result.unwrap_err();
    assert!(!errors.is_empty(), "should have errors");
    assert!(
        errors
            .errors()
            .iter()
            .any(|e| matches!(e, native_theme_build::BuildError::UnknownRole { role, .. } if role == "bluetooth")),
        "should have UnknownRole for 'bluetooth': {errors:?}",
    );

    if let Err(e) = fs::remove_dir_all(&dir) {
        eprintln!("test cleanup warning: {e}");
    }
}

#[test]
fn error_missing_svg_file() {
    let dir = create_temp_dir("missing_svg");
    write_file(
        &dir,
        "icons.toml",
        r#"
name = "test"
roles = ["play-pause", "skip-forward"]
bundled-themes = ["material"]
"#,
    );
    write_file(
        &dir,
        "material/mapping.toml",
        "play-pause = \"play_pause\"\nskip-forward = \"skip_next\"\n",
    );
    // Only create one SVG, leave skip_next.svg missing
    write_file(&dir, "material/play_pause.svg", SVG_STUB);

    let result = IconGenerator::new()
        .source(dir.join("icons.toml"))
        .output_dir(&dir)
        .generate();

    let errors = result.unwrap_err();
    assert!(!errors.is_empty(), "should have errors");
    assert!(
        errors
            .errors()
            .iter()
            .any(|e| matches!(e, native_theme_build::BuildError::MissingSvg { path } if path.contains("skip_next.svg"))),
        "should have MissingSvg for 'skip_next.svg': {errors:?}",
    );

    if let Err(e) = fs::remove_dir_all(&dir) {
        eprintln!("test cleanup warning: {e}");
    }
}

// =============================================================================
// Builder API: merging multiple files
// =============================================================================

#[test]
fn builder_merges_disjoint_roles() {
    let dir = create_temp_dir("builder_merge");
    write_file(
        &dir,
        "icons-a.toml",
        r#"
name = "icons-a"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
    );
    write_file(
        &dir,
        "icons-b.toml",
        r#"
name = "icons-b"
roles = ["skip-forward"]
bundled-themes = ["material"]
"#,
    );
    write_file(
        &dir,
        "material/mapping.toml",
        "play-pause = \"play_pause\"\nskip-forward = \"skip_next\"\n",
    );
    write_file(&dir, "material/play_pause.svg", SVG_STUB);
    write_file(&dir, "material/skip_next.svg", SVG_STUB);

    let output = IconGenerator::new()
        .source(dir.join("icons-a.toml"))
        .source(dir.join("icons-b.toml"))
        .enum_name("AllIcons")
        .output_dir(&dir)
        .generate()
        .unwrap_or_else(|e| panic!("expected no errors: {e}"));

    assert!(
        output.code.contains("pub enum AllIcons"),
        "should use override enum name"
    );
    assert!(
        output.code.contains("PlayPause"),
        "should have PlayPause from file A"
    );
    assert!(
        output.code.contains("SkipForward"),
        "should have SkipForward from file B"
    );

    // #61: Roles from the first source should appear before roles from the second
    let play_pos = output.code.find("PlayPause").expect("PlayPause not found");
    let skip_pos = output
        .code
        .find("SkipForward")
        .expect("SkipForward not found");
    assert!(
        play_pos < skip_pos,
        "PlayPause (from file A) should appear before SkipForward (from file B) in generated code"
    );

    assert_eq!(
        output.output_path.file_name().unwrap().to_str().unwrap(),
        "all_icons.rs"
    );

    if let Err(e) = fs::remove_dir_all(&dir) {
        eprintln!("test cleanup warning: {e}");
    }
}

// =============================================================================
// DE-aware codegen integration tests
// =============================================================================

#[test]
fn de_aware_mapping_generates_de_dispatch_code() {
    let dir = create_temp_dir("de_aware");
    write_file(
        &dir,
        "icons.toml",
        r#"
name = "de-test"
roles = ["reveal"]
system-themes = ["freedesktop"]
"#,
    );
    write_file(
        &dir,
        "freedesktop/mapping.toml",
        r#"
reveal = { kde = "view-visible", default = "view-reveal" }
"#,
    );

    let output = IconGenerator::new()
        .source(dir.join("icons.toml"))
        .output_dir(&dir)
        .generate()
        .unwrap_or_else(|e| panic!("expected no errors: {e}"));

    assert!(!output.code.is_empty(), "expected generated code");

    // Verify cfg-gated DE dispatch
    assert!(
        output.code.contains("#[cfg(target_os = \"linux\")]"),
        "should have cfg linux gate. code:\n{}",
        output.code
    );
    assert!(
        output
            .code
            .contains("native_theme::detect::detect_linux_de("),
        "should call detect_linux_de. code:\n{}",
        output.code
    );
    assert!(
        output
            .code
            .contains("native_theme::detect::LinuxDesktop::Kde => Some(\"view-visible\")"),
        "should have KDE-specific arm. code:\n{}",
        output.code
    );
    assert!(
        output.code.contains("_ => Some(\"view-reveal\")"),
        "should have default wildcard arm. code:\n{}",
        output.code
    );
    assert!(
        output.code.contains("#[cfg(not(target_os = \"linux\"))]"),
        "should have cfg not-linux gate. code:\n{}",
        output.code
    );

    // #89: Verify DE-specific match arms reference the expected DE names
    assert!(
        output.code.contains("Kde"),
        "should contain Kde variant in DE dispatch. code:\n{}",
        output.code
    );
    assert!(
        output.code.contains("view-visible"),
        "should contain KDE-specific icon name 'view-visible'. code:\n{}",
        output.code
    );
    assert!(
        output.code.contains("view-reveal"),
        "should contain default icon name 'view-reveal'. code:\n{}",
        output.code
    );

    if let Err(e) = fs::remove_dir_all(&dir) {
        eprintln!("test cleanup warning: {e}");
    }
}

#[test]
fn de_aware_unknown_key_produces_warning() {
    let dir = create_temp_dir("de_unknown_key");
    write_file(
        &dir,
        "icons.toml",
        r#"
name = "de-test"
roles = ["reveal"]
system-themes = ["freedesktop"]
"#,
    );
    write_file(
        &dir,
        "freedesktop/mapping.toml",
        r#"
reveal = { cosmic = "cosmic-reveal", default = "view-reveal" }
"#,
    );

    let output = IconGenerator::new()
        .source(dir.join("icons.toml"))
        .output_dir(&dir)
        .generate()
        .unwrap_or_else(|e| panic!("warnings should not be errors: {e}"));

    // Warnings should mention cosmic and unrecognized (checked separately for resilience)
    assert!(
        output.warnings.iter().any(|w| w.contains("cosmic")),
        "should mention 'cosmic': {:?}",
        output.warnings
    );
    assert!(
        output.warnings.iter().any(|w| w.contains("unrecognized")),
        "should mention 'unrecognized': {:?}",
        output.warnings
    );

    assert!(!output.code.is_empty(), "code should still be generated");

    // Since "cosmic" maps to None in de_key_to_variant, no DE-specific arms generated,
    // so the DeAware value collapses to a simple arm using the default
    assert!(
        output.code.contains("Some(\"view-reveal\")"),
        "should use default value. code:\n{}",
        output.code
    );

    if let Err(e) = fs::remove_dir_all(&dir) {
        eprintln!("test cleanup warning: {e}");
    }
}

#[test]
fn builder_detects_duplicate_roles() {
    let dir = create_temp_dir("builder_dup");
    write_file(
        &dir,
        "icons-a.toml",
        r#"
name = "icons-a"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
    );
    write_file(
        &dir,
        "icons-b.toml",
        r#"
name = "icons-b"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
    );
    write_file(
        &dir,
        "material/mapping.toml",
        "play-pause = \"play_pause\"\n",
    );
    write_file(&dir, "material/play_pause.svg", SVG_STUB);

    let result = IconGenerator::new()
        .source(dir.join("icons-a.toml"))
        .source(dir.join("icons-b.toml"))
        .output_dir(&dir)
        .generate();

    let errors = result.unwrap_err();
    assert!(!errors.is_empty(), "should detect duplicate roles");
    // Count only the DuplicateRole errors (filter out any identifier-related errors
    // that may also be emitted from per-file and merged validation passes).
    let dup_role_errors: Vec<_> = errors
        .errors()
        .iter()
        .filter(|e| matches!(e, native_theme_build::BuildError::DuplicateRole { .. }))
        .collect();
    assert_eq!(
        dup_role_errors.len(),
        1,
        "expected exactly 1 DuplicateRole error, got {}: {errors:?}",
        dup_role_errors.len(),
    );
    assert!(
        errors
            .errors()
            .iter()
            .any(|e| matches!(e, native_theme_build::BuildError::DuplicateRole { role, .. } if role == "play-pause")),
        "should have DuplicateRole for 'play-pause': {errors:?}",
    );

    if let Err(e) = fs::remove_dir_all(&dir) {
        eprintln!("test cleanup warning: {e}");
    }
}

// =============================================================================
// Additional test coverage
// =============================================================================

/// #28: Test generate_icons() simple API with a valid fixture TOML.
#[test]
#[allow(clippy::unwrap_used, clippy::expect_used)]
fn generate_icons_simple_api() {
    let out = create_temp_dir("simple_api_out");

    // generate_icons() requires CARGO_MANIFEST_DIR and OUT_DIR env vars.
    // CARGO_MANIFEST_DIR is set by cargo test. We set OUT_DIR explicitly.
    unsafe { std::env::set_var("OUT_DIR", &out) };

    let result = native_theme_build::generate_icons(
        // Path relative to CARGO_MANIFEST_DIR
        "tests/fixtures/sample-icons.toml",
    );

    let output = result.unwrap_or_else(|e| panic!("generate_icons() failed: {e}"));
    assert!(!output.code.is_empty(), "should generate code");
    assert!(
        output.code.contains("pub enum SampleIcon"),
        "should produce SampleIcon enum"
    );
    assert_eq!(output.role_count, 2);

    if let Err(e) = fs::remove_dir_all(&out) {
        eprintln!("test cleanup warning: {e}");
    }
}

/// #29: Test that base_dir() affects the generated include_bytes! paths.
#[test]
fn base_dir_affects_include_bytes_paths() {
    let dir = create_temp_dir("base_dir_test");

    // Create a TOML file in a subdirectory, but themes in a different directory
    write_file(
        &dir,
        "config/icons.toml",
        r#"
name = "base-test"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
    );
    write_file(
        &dir,
        "themes/material/mapping.toml",
        "play-pause = \"play_pause\"\n",
    );
    write_file(&dir, "themes/material/play_pause.svg", SVG_STUB);

    let output = IconGenerator::new()
        .source(dir.join("config/icons.toml"))
        .base_dir(dir.join("themes"))
        .output_dir(&dir)
        .generate()
        .unwrap_or_else(|e| panic!("expected no errors: {e}"));

    // The include_bytes! path should be relative to the base_dir (themes/),
    // not relative to the TOML file's parent directory (config/).
    assert!(
        output.code.contains("include_bytes!"),
        "should have include_bytes! directives"
    );
    // With base_dir set to themes/, the path should reference material/play_pause.svg
    assert!(
        output.code.contains("material/play_pause.svg"),
        "include_bytes! path should reference material/play_pause.svg. code:\n{}",
        output.code
    );

    if let Err(e) = fs::remove_dir_all(&dir) {
        eprintln!("test cleanup warning: {e}");
    }
}

/// #35: Test output_dir fallback to OUT_DIR env var.
#[test]
#[allow(clippy::unwrap_used, clippy::expect_used)]
fn output_dir_fallback_to_out_dir_env() {
    let dir = create_temp_dir("outdir_fallback");
    let out = create_temp_dir("outdir_fallback_out");

    write_file(
        &dir,
        "icons.toml",
        r#"
name = "fallback-test"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
    );
    write_file(
        &dir,
        "material/mapping.toml",
        "play-pause = \"play_pause\"\n",
    );
    write_file(&dir, "material/play_pause.svg", SVG_STUB);

    // Don't call .output_dir() -- rely on OUT_DIR env var
    unsafe { std::env::set_var("OUT_DIR", &out) };

    let output = IconGenerator::new()
        .source(dir.join("icons.toml"))
        .generate()
        .unwrap_or_else(|e| panic!("expected no errors: {e}"));

    // Output path should be inside the OUT_DIR directory
    assert!(
        output.output_path.starts_with(&out),
        "output_path {:?} should be under OUT_DIR {:?}",
        output.output_path,
        out
    );

    if let Err(e) = fs::remove_dir_all(&dir) {
        eprintln!("test cleanup warning: {e}");
    }
    if let Err(e) = fs::remove_dir_all(&out) {
        eprintln!("test cleanup warning: {e}");
    }
}

/// #33: Golden syntax test -- generated code has valid Rust structure.
#[test]
fn golden_syntax_check() {
    let output = generate_fixture(&fixtures_dir().join("sample-icons.toml"));

    // Contains enum keyword
    assert!(
        output.code.contains("enum"),
        "generated code should contain 'enum' keyword"
    );

    // Contains fn icon_name (trait impl method, not pub)
    assert!(
        output.code.contains("fn icon_name"),
        "generated code should contain 'fn icon_name'"
    );

    // Contains fn icon_svg (bundled theme present, so SVG function should exist)
    assert!(
        output.code.contains("fn icon_svg"),
        "generated code should contain 'fn icon_svg'"
    );

    // Braces are balanced
    let open_braces = output.code.chars().filter(|&c| c == '{').count();
    let close_braces = output.code.chars().filter(|&c| c == '}').count();
    assert_eq!(
        open_braces, close_braces,
        "braces should be balanced: {{ = {open_braces}, }} = {close_braces}"
    );
}

/// #33: Verify generated code compiles by writing a temp Cargo project and running `cargo check`.
///
/// This goes beyond `golden_syntax_check` (string matching) by actually invoking the
/// Rust compiler. Skips gracefully if `cargo` is unavailable or the build fails for
/// environmental reasons (e.g., missing toolchain components in CI).
#[test]
fn compile_generated_code() {
    let output = generate_fixture(&fixtures_dir().join("sample-icons.toml"));

    let dir = create_temp_dir("compile_check");
    let src_dir = dir.join("src");
    fs::create_dir_all(&src_dir).unwrap();

    // Write the generated code as lib.rs
    fs::write(src_dir.join("lib.rs"), &output.code).unwrap();

    // Copy fixture SVGs so include_bytes! paths resolve (they reference CARGO_MANIFEST_DIR)
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixture_src = manifest_dir.join("tests/fixtures");
    if fixture_src.exists() {
        copy_dir_recursive(&fixture_src, &dir.join("tests/fixtures"));
    }

    // Write a minimal Cargo.toml with native-theme as a path dependency
    let native_theme_path = manifest_dir.join("../native-theme");
    let cargo_toml = format!(
        r#"[package]
name = "compile-check"
version = "0.0.0"
edition = "2021"

[dependencies]
native-theme = {{ path = "{}" }}
"#,
        native_theme_path.display()
    );
    fs::write(dir.join("Cargo.toml"), cargo_toml).unwrap();

    // Run cargo check
    let result = std::process::Command::new("cargo")
        .args(["check", "--quiet"])
        .current_dir(&dir)
        .output();

    // Clean up
    let _ = fs::remove_dir_all(&dir);

    match result {
        Ok(output_cmd) => {
            if !output_cmd.status.success() {
                let stderr = String::from_utf8_lossy(&output_cmd.stderr);
                panic!(
                    "generated code failed to compile:\n{stderr}\n\n--- generated code ---\n{}",
                    output.code
                );
            }
        }
        Err(e) => {
            eprintln!("skipping compile_generated_code: cargo not available ({e})");
        }
    }
}

/// #31: Test that the freedesktop fixture mapping (with DE-aware values) works as a system theme.
#[test]
fn freedesktop_fixture_as_system_theme() {
    let dir = create_temp_dir("freedesktop_fixture");

    write_file(
        &dir,
        "icons.toml",
        r#"
name = "freedesktop-test"
roles = ["play-pause", "skip-forward"]
system-themes = ["freedesktop"]
"#,
    );
    // Copy the committed fixture mapping into the temp dir
    let fixture_mapping = fs::read_to_string(fixtures_dir().join("freedesktop/mapping.toml"))
        .unwrap_or_else(|e| panic!("failed to read freedesktop fixture: {e}"));
    write_file(&dir, "freedesktop/mapping.toml", &fixture_mapping);

    let output = IconGenerator::new()
        .source(dir.join("icons.toml"))
        .output_dir(&dir)
        .generate()
        .unwrap_or_else(|e| panic!("expected no errors: {e}"));

    assert!(!output.code.is_empty(), "should generate code");
    assert!(
        output.code.contains("PlayPause"),
        "should have PlayPause variant"
    );
    assert!(
        output.code.contains("SkipForward"),
        "should have SkipForward variant"
    );
    // System theme should not have include_bytes!
    assert!(
        !output.code.contains("include_bytes!"),
        "system theme should not have embedded SVGs"
    );
    // DE-aware mapping should generate DE dispatch code
    assert!(
        output.code.contains("media-playback-start"),
        "should contain KDE-specific icon name"
    );

    if let Err(e) = fs::remove_dir_all(&dir) {
        eprintln!("test cleanup warning: {e}");
    }
}

/// #31: Test that the segoe-fluent fixture mapping works as a system theme.
#[test]
fn segoe_fluent_fixture_as_system_theme() {
    let dir = create_temp_dir("segoe_fluent_fixture");

    write_file(
        &dir,
        "icons.toml",
        r#"
name = "segoe-test"
roles = ["play-pause", "skip-forward"]
system-themes = ["segoe-fluent"]
"#,
    );
    let fixture_mapping = fs::read_to_string(fixtures_dir().join("segoe-fluent/mapping.toml"))
        .unwrap_or_else(|e| panic!("failed to read segoe-fluent fixture: {e}"));
    write_file(&dir, "segoe-fluent/mapping.toml", &fixture_mapping);

    let output = IconGenerator::new()
        .source(dir.join("icons.toml"))
        .output_dir(&dir)
        .generate()
        .unwrap_or_else(|e| panic!("expected no errors: {e}"));

    assert!(!output.code.is_empty(), "should generate code");
    assert!(
        output.code.contains("PlayPause"),
        "should have PlayPause variant"
    );
    assert!(
        output.code.contains("SkipForward"),
        "should have SkipForward variant"
    );
    assert!(
        !output.code.contains("include_bytes!"),
        "system theme should not have embedded SVGs"
    );

    if let Err(e) = fs::remove_dir_all(&dir) {
        eprintln!("test cleanup warning: {e}");
    }
}

/// #31: Test that the lucide fixture mapping can be loaded as a system theme.
#[test]
fn lucide_fixture_as_system_theme() {
    let dir = create_temp_dir("lucide_fixture");

    write_file(
        &dir,
        "icons.toml",
        r#"
name = "lucide-test"
roles = ["play-pause", "skip-forward"]
system-themes = ["lucide"]
"#,
    );
    // Copy the committed fixture mapping into the temp dir
    let fixture_mapping = fs::read_to_string(fixtures_dir().join("lucide/mapping.toml"))
        .unwrap_or_else(|e| panic!("failed to read lucide fixture: {e}"));
    write_file(&dir, "lucide/mapping.toml", &fixture_mapping);

    let output = IconGenerator::new()
        .source(dir.join("icons.toml"))
        .output_dir(&dir)
        .generate()
        .unwrap_or_else(|e| panic!("expected no errors: {e}"));

    assert!(!output.code.is_empty(), "should generate code");
    assert!(
        output.code.contains("PlayPause"),
        "should have PlayPause variant"
    );
    assert!(
        output.code.contains("SkipForward"),
        "should have SkipForward variant"
    );
    // System theme should not have include_bytes!
    assert!(
        !output.code.contains("include_bytes!"),
        "system theme should not have embedded SVGs"
    );

    if let Err(e) = fs::remove_dir_all(&dir) {
        eprintln!("test cleanup warning: {e}");
    }
}

#[test]
fn system_only_theme_icon_svg_returns_none() {
    let dir = create_temp_dir("sys_only_svg");
    write_file(
        &dir,
        "icons.toml",
        r#"
name = "sys-only"
roles = ["reveal"]
system-themes = ["freedesktop"]
"#,
    );
    write_file(
        &dir,
        "freedesktop/mapping.toml",
        "reveal = \"view-reveal\"\n",
    );

    let output = IconGenerator::new()
        .source(dir.join("icons.toml"))
        .output_dir(&dir)
        .generate()
        .unwrap_or_else(|e| panic!("expected no errors: {e}"));

    // System-only themes should NOT generate include_bytes! calls
    assert!(
        !output.code.contains("include_bytes!"),
        "system-only theme should not embed SVG data via include_bytes!. code:\n{}",
        output.code,
    );

    // The icon_svg function should exist but only return None for system themes
    assert!(
        output.code.contains("icon_svg"),
        "generated code should contain icon_svg function",
    );

    if let Err(e) = fs::remove_dir_all(&dir) {
        eprintln!("test cleanup warning: {e}");
    }
}
