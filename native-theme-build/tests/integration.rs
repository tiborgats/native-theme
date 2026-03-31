use std::fs;
use std::path::{Path, PathBuf};

use native_theme_build::IconGenerator;

const SVG_STUB: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"></svg>"#;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn create_temp_dir(suffix: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("native_theme_integ_{suffix}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
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
            .contains("impl native_theme::IconProvider for SampleIcon"),
        "should have IconProvider impl"
    );
}

#[test]
fn happy_path_icon_name_material() {
    let output = generate_fixture(&fixtures_dir().join("sample-icons.toml"));

    assert!(
        output
            .code
            .contains("(Self::PlayPause, native_theme::IconSet::Material) => Some(\"play_pause\")"),
        "should have Material icon_name arm for PlayPause"
    );
    assert!(
        output.code.contains(
            "(Self::SkipForward, native_theme::IconSet::Material) => Some(\"skip_next\")"
        ),
        "should have Material icon_name arm for SkipForward"
    );
}

#[test]
fn happy_path_icon_name_sf_symbols() {
    let output = generate_fixture(&fixtures_dir().join("sample-icons.toml"));

    assert!(
        output
            .code
            .contains("(Self::PlayPause, native_theme::IconSet::SfSymbols) => Some(\"play.fill\")"),
        "should have SfSymbols icon_name arm for PlayPause"
    );
    assert!(
        output.code.contains(
            "(Self::SkipForward, native_theme::IconSet::SfSymbols) => Some(\"forward.fill\")"
        ),
        "should have SfSymbols icon_name arm for SkipForward"
    );
}

#[test]
fn happy_path_icon_svg_bundled_only() {
    let output = generate_fixture(&fixtures_dir().join("sample-icons.toml"));

    // Material (bundled) should have include_bytes! arms
    assert!(
        output.code.contains("include_bytes!") && output.code.contains("material/play_pause.svg"),
        "should have include_bytes! for bundled material SVGs"
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
        output.code.contains("Self::PlayPause") && output.code.contains("Self::SkipForward"),
        "ALL should contain both variants"
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
            .any(|e| e.to_string().contains("skip-forward")),
        "should mention missing role 'skip-forward': {errors:?}",
    );

    let _ = fs::remove_dir_all(&dir);
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
            .any(|e| e.to_string().contains("bluetooth")),
        "should mention unknown role 'bluetooth': {errors:?}",
    );

    let _ = fs::remove_dir_all(&dir);
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
            .any(|e| e.to_string().contains("skip_next.svg")),
        "should mention missing SVG path: {errors:?}",
    );

    let _ = fs::remove_dir_all(&dir);
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
    assert_eq!(
        output.output_path.file_name().unwrap().to_str().unwrap(),
        "all_icons.rs"
    );

    let _ = fs::remove_dir_all(&dir);
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
        output.code.contains("native_theme::detect_linux_de("),
        "should call detect_linux_de. code:\n{}",
        output.code
    );
    assert!(
        output
            .code
            .contains("native_theme::LinuxDesktop::Kde => Some(\"view-visible\")"),
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

    let _ = fs::remove_dir_all(&dir);
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

    // Warnings should mention cosmic and unrecognized
    assert!(
        output
            .warnings
            .iter()
            .any(|w| w.contains("cosmic") && w.contains("unrecognized DE key")),
        "should warn about unrecognized 'cosmic' DE key: {:?}",
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

    let _ = fs::remove_dir_all(&dir);
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
    assert!(
        errors
            .errors()
            .iter()
            .any(|e| e.to_string().contains("play-pause")),
        "should mention duplicate role: {errors:?}",
    );

    let _ = fs::remove_dir_all(&dir);
}
