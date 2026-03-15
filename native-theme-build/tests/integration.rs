use std::fs;
use std::path::{Path, PathBuf};

use native_theme_build::__run_pipeline_on_files;

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

// =============================================================================
// Happy path: full pipeline on committed fixtures
// =============================================================================

#[test]
fn happy_path_generates_correct_enum() {
    let fixtures = fixtures_dir();
    let toml_path = fixtures.join("sample-icons.toml");

    let result = __run_pipeline_on_files(&[toml_path.as_path()], None);

    assert!(
        result.errors.is_empty(),
        "expected no errors: {:?}",
        result.errors
    );
    assert!(!result.code.is_empty(), "expected generated code");

    // Check enum name is PascalCase of "sample-icon"
    assert!(
        result.code.contains("pub enum SampleIcon"),
        "should have PascalCase enum name. code:\n{}",
        result.code
    );

    // Check variants
    assert!(
        result.code.contains("PlayPause"),
        "should have PlayPause variant"
    );
    assert!(
        result.code.contains("SkipForward"),
        "should have SkipForward variant"
    );
}

#[test]
fn happy_path_has_icon_provider_impl() {
    let fixtures = fixtures_dir();
    let toml_path = fixtures.join("sample-icons.toml");

    let result = __run_pipeline_on_files(&[toml_path.as_path()], None);

    assert!(result.errors.is_empty());
    assert!(
        result
            .code
            .contains("impl native_theme::IconProvider for SampleIcon"),
        "should have IconProvider impl"
    );
}

#[test]
fn happy_path_icon_name_material() {
    let fixtures = fixtures_dir();
    let toml_path = fixtures.join("sample-icons.toml");

    let result = __run_pipeline_on_files(&[toml_path.as_path()], None);

    assert!(result.errors.is_empty());
    assert!(
        result
            .code
            .contains("(Self::PlayPause, native_theme::IconSet::Material) => Some(\"play_pause\")"),
        "should have Material icon_name arm for PlayPause"
    );
    assert!(
        result
            .code
            .contains("(Self::SkipForward, native_theme::IconSet::Material) => Some(\"skip_next\")"),
        "should have Material icon_name arm for SkipForward"
    );
}

#[test]
fn happy_path_icon_name_sf_symbols() {
    let fixtures = fixtures_dir();
    let toml_path = fixtures.join("sample-icons.toml");

    let result = __run_pipeline_on_files(&[toml_path.as_path()], None);

    assert!(result.errors.is_empty());
    assert!(
        result
            .code
            .contains("(Self::PlayPause, native_theme::IconSet::SfSymbols) => Some(\"play.fill\")"),
        "should have SfSymbols icon_name arm for PlayPause"
    );
    assert!(
        result
            .code
            .contains("(Self::SkipForward, native_theme::IconSet::SfSymbols) => Some(\"forward.fill\")"),
        "should have SfSymbols icon_name arm for SkipForward"
    );
}

#[test]
fn happy_path_icon_svg_bundled_only() {
    let fixtures = fixtures_dir();
    let toml_path = fixtures.join("sample-icons.toml");

    let result = __run_pipeline_on_files(&[toml_path.as_path()], None);

    assert!(result.errors.is_empty());

    // Material (bundled) should have include_bytes! arms
    assert!(
        result.code.contains("include_bytes!") && result.code.contains("material/play_pause.svg"),
        "should have include_bytes! for bundled material SVGs"
    );

    // sf-symbols (system) should NOT have include_bytes! arms
    assert!(
        !result.code.contains("SfSymbols) => Some(include_bytes!"),
        "system themes should not have include_bytes! arms"
    );
}

#[test]
fn happy_path_has_const_all() {
    let fixtures = fixtures_dir();
    let toml_path = fixtures.join("sample-icons.toml");

    let result = __run_pipeline_on_files(&[toml_path.as_path()], None);

    assert!(result.errors.is_empty());
    assert!(
        result.code.contains("pub const ALL: &[Self]"),
        "should have const ALL"
    );
    assert!(
        result.code.contains("Self::PlayPause") && result.code.contains("Self::SkipForward"),
        "ALL should contain both variants"
    );
}

#[test]
fn happy_path_output_filename() {
    let fixtures = fixtures_dir();
    let toml_path = fixtures.join("sample-icons.toml");

    let result = __run_pipeline_on_files(&[toml_path.as_path()], None);

    assert_eq!(result.output_filename, "sample_icon.rs");
}

#[test]
fn happy_path_size_report() {
    let fixtures = fixtures_dir();
    let toml_path = fixtures.join("sample-icons.toml");

    let result = __run_pipeline_on_files(&[toml_path.as_path()], None);

    assert!(result.errors.is_empty());
    let report = result.size_report.as_ref().expect("should have size report");
    assert_eq!(report.role_count, 2);
    assert_eq!(report.bundled_theme_count, 1);
    assert_eq!(report.svg_count, 2);
    assert!(report.total_svg_bytes > 0);
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
    write_file(&dir, "material/mapping.toml", "play-pause = \"play_pause\"\n");
    write_file(&dir, "material/play_pause.svg", SVG_STUB);

    let toml_path = dir.join("icons.toml");
    let result = __run_pipeline_on_files(&[toml_path.as_path()], None);

    assert!(!result.errors.is_empty(), "should have errors");
    assert!(
        result.errors.iter().any(|e| e.contains("skip-forward")),
        "should mention missing role 'skip-forward': {:?}",
        result.errors
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

    let toml_path = dir.join("icons.toml");
    let result = __run_pipeline_on_files(&[toml_path.as_path()], None);

    assert!(!result.errors.is_empty(), "should have errors");
    assert!(
        result.errors.iter().any(|e| e.contains("bluetooth")),
        "should mention unknown role 'bluetooth': {:?}",
        result.errors
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

    let toml_path = dir.join("icons.toml");
    let result = __run_pipeline_on_files(&[toml_path.as_path()], None);

    assert!(!result.errors.is_empty(), "should have errors");
    assert!(
        result
            .errors
            .iter()
            .any(|e| e.contains("skip_next.svg")),
        "should mention missing SVG path: {:?}",
        result.errors
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

    let path_a = dir.join("icons-a.toml");
    let path_b = dir.join("icons-b.toml");
    let result =
        __run_pipeline_on_files(&[path_a.as_path(), path_b.as_path()], Some("AllIcons"));

    assert!(
        result.errors.is_empty(),
        "expected no errors: {:?}",
        result.errors
    );
    assert!(
        result.code.contains("pub enum AllIcons"),
        "should use override enum name"
    );
    assert!(
        result.code.contains("PlayPause"),
        "should have PlayPause from file A"
    );
    assert!(
        result.code.contains("SkipForward"),
        "should have SkipForward from file B"
    );
    assert_eq!(result.output_filename, "all_icons.rs");

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
    write_file(&dir, "material/mapping.toml", "play-pause = \"play_pause\"\n");
    write_file(&dir, "material/play_pause.svg", SVG_STUB);

    let path_a = dir.join("icons-a.toml");
    let path_b = dir.join("icons-b.toml");
    let result = __run_pipeline_on_files(&[path_a.as_path(), path_b.as_path()], None);

    assert!(!result.errors.is_empty(), "should detect duplicate roles");
    assert!(
        result.errors.iter().any(|e| e.contains("play-pause")),
        "should mention duplicate role: {:?}",
        result.errors
    );

    let _ = fs::remove_dir_all(&dir);
}
