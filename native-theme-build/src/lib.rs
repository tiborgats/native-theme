mod codegen;
mod error;
mod schema;
mod validate;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use heck::ToSnakeCase;

use schema::{MasterConfig, ThemeMapping};

// Re-exported for unit tests via `use super::*`
#[cfg(test)]
use error::BuildError;
#[cfg(test)]
use schema::{MappingValue, KNOWN_THEMES};

/// Load a TOML file and run the pipeline on it. For integration testing only.
#[doc(hidden)]
pub fn __run_pipeline_on_files(
    toml_paths: &[&Path],
    enum_name_override: Option<&str>,
) -> PipelineResult {
    let mut configs = Vec::new();
    let mut base_dirs = Vec::new();

    for path in toml_paths {
        let content = std::fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
        let config: MasterConfig = toml::from_str(&content)
            .unwrap_or_else(|e| panic!("failed to parse {}: {e}", path.display()));
        let base_dir = path.parent().expect("TOML path has no parent").to_path_buf();
        configs.push((path.to_string_lossy().to_string(), config));
        base_dirs.push(base_dir);
    }

    run_pipeline(&configs, &base_dirs, enum_name_override, None)
}

/// Result of running the pure pipeline core.
///
/// Contains the generated code, collected errors, and collected warnings.
/// The thin outer layer (generate_icons / IconGenerator::generate) handles
/// printing cargo directives, writing files, and calling process::exit.
#[doc(hidden)]
pub struct PipelineResult {
    /// Generated Rust source code (empty if errors were found).
    pub code: String,
    /// Build errors found during validation.
    pub errors: Vec<String>,
    /// Warnings (e.g., orphan SVGs).
    pub warnings: Vec<String>,
    /// Paths that should trigger rebuild when changed.
    pub rerun_paths: Vec<PathBuf>,
    /// Size report: (role_count, bundled_theme_count, svg_paths).
    pub size_report: Option<SizeReport>,
    /// The output filename (snake_case of config name + ".rs").
    pub output_filename: String,
}

/// Size report for cargo::warning output.
#[doc(hidden)]
pub struct SizeReport {
    pub role_count: usize,
    pub bundled_theme_count: usize,
    pub total_svg_bytes: u64,
    pub svg_count: usize,
}

/// Run the full pipeline on one or more loaded configs.
///
/// This is the pure core: it takes parsed configs, validates, generates code,
/// and returns everything as data. No I/O, no process::exit.
///
/// When `manifest_dir` is `Some`, `base_dir` paths are stripped of the
/// manifest prefix before being embedded in `include_bytes!` codegen,
/// producing portable relative paths like `"/icons/material/play.svg"`
/// instead of absolute filesystem paths.
#[doc(hidden)]
pub fn run_pipeline(
    configs: &[(String, MasterConfig)],
    base_dirs: &[PathBuf],
    enum_name_override: Option<&str>,
    manifest_dir: Option<&Path>,
) -> PipelineResult {
    assert_eq!(configs.len(), base_dirs.len());

    let mut errors: Vec<String> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();
    let mut rerun_paths: Vec<PathBuf> = Vec::new();
    let mut all_mappings: BTreeMap<String, ThemeMapping> = BTreeMap::new();
    let mut svg_paths: Vec<PathBuf> = Vec::new();

    // Determine output filename from first config or override
    let first_name = enum_name_override
        .map(|s| s.to_string())
        .unwrap_or_else(|| configs[0].1.name.clone());
    let output_filename = format!("{}.rs", first_name.to_snake_case());

    // Step 1: Check for duplicate roles across all files
    if configs.len() > 1 {
        let dup_errors = validate::validate_no_duplicate_roles(configs);
        for e in dup_errors {
            errors.push(e.to_string());
        }
    }

    // Step 2: Merge configs first so validation uses the merged role list
    let merged = merge_configs(configs, enum_name_override);

    // Track rerun paths for all master TOML files
    for (file_path, _config) in configs {
        rerun_paths.push(PathBuf::from(file_path));
    }

    // Validate theme names on the merged config
    let theme_errors = validate::validate_themes(&merged);
    for e in theme_errors {
        errors.push(e.to_string());
    }

    // Use the first base_dir as the reference for loading themes.
    // For multi-file, all configs sharing a theme must use the same base_dir.
    let base_dir = &base_dirs[0];

    // Process bundled themes
    for theme_name in &merged.bundled_themes {
        let theme_dir = base_dir.join(theme_name);
        let mapping_path = theme_dir.join("mapping.toml");
        let mapping_path_str = mapping_path.to_string_lossy().to_string();

        // Add mapping TOML and theme directory to rerun paths
        rerun_paths.push(mapping_path.clone());
        rerun_paths.push(theme_dir.clone());

        match std::fs::read_to_string(&mapping_path) {
            Ok(content) => match toml::from_str::<ThemeMapping>(&content) {
                Ok(mapping) => {
                    // Validate mapping against merged roles
                    let map_errors =
                        validate::validate_mapping(&merged.roles, &mapping, &mapping_path_str);
                    for e in map_errors {
                        errors.push(e.to_string());
                    }

                    // Validate SVGs exist
                    let svg_errors =
                        validate::validate_svgs(&mapping, &theme_dir, &mapping_path_str);
                    for e in svg_errors {
                        errors.push(e.to_string());
                    }

                    // Check orphan SVGs (warnings, not errors)
                    let orphan_warnings =
                        check_orphan_svgs_and_collect_paths(&mapping, &theme_dir, theme_name, &mut svg_paths, &mut rerun_paths);
                    warnings.extend(orphan_warnings);

                    all_mappings.insert(theme_name.clone(), mapping);
                }
                Err(e) => {
                    errors.push(format!("failed to parse {mapping_path_str}: {e}"));
                }
            },
            Err(e) => {
                errors.push(format!("failed to read {mapping_path_str}: {e}"));
            }
        }
    }

    // Process system themes (no SVG validation)
    for theme_name in &merged.system_themes {
        let theme_dir = base_dir.join(theme_name);
        let mapping_path = theme_dir.join("mapping.toml");
        let mapping_path_str = mapping_path.to_string_lossy().to_string();

        // Add mapping TOML to rerun paths
        rerun_paths.push(mapping_path.clone());

        match std::fs::read_to_string(&mapping_path) {
            Ok(content) => match toml::from_str::<ThemeMapping>(&content) {
                Ok(mapping) => {
                    let map_errors =
                        validate::validate_mapping(&merged.roles, &mapping, &mapping_path_str);
                    for e in map_errors {
                        errors.push(e.to_string());
                    }
                    all_mappings.insert(theme_name.clone(), mapping);
                }
                Err(e) => {
                    errors.push(format!("failed to parse {mapping_path_str}: {e}"));
                }
            },
            Err(e) => {
                errors.push(format!("failed to read {mapping_path_str}: {e}"));
            }
        }
    }

    // If errors, return without generating code
    if !errors.is_empty() {
        return PipelineResult {
            code: String::new(),
            errors,
            warnings,
            rerun_paths,
            size_report: None,
            output_filename,
        };
    }

    // Compute base_dir for codegen -- strip manifest_dir prefix when available
    // so include_bytes! paths are relative (e.g., "icons/material/play.svg")
    // instead of absolute (e.g., "/home/user/project/icons/material/play.svg")
    let base_dir_str = if let Some(mdir) = manifest_dir {
        base_dir
            .strip_prefix(mdir)
            .unwrap_or(base_dir)
            .to_string_lossy()
            .to_string()
    } else {
        base_dir.to_string_lossy().to_string()
    };

    // Step 4: Generate code
    let code = codegen::generate_code(&merged, &all_mappings, &base_dir_str);

    // Step 5: Compute size report
    let total_svg_bytes: u64 = svg_paths
        .iter()
        .filter_map(|p| std::fs::metadata(p).ok())
        .map(|m| m.len())
        .sum();

    let size_report = Some(SizeReport {
        role_count: merged.roles.len(),
        bundled_theme_count: merged.bundled_themes.len(),
        total_svg_bytes,
        svg_count: svg_paths.len(),
    });

    PipelineResult {
        code,
        errors,
        warnings,
        rerun_paths,
        size_report,
        output_filename,
    }
}

/// Check orphan SVGs and simultaneously collect SVG paths and rerun paths.
fn check_orphan_svgs_and_collect_paths(
    mapping: &ThemeMapping,
    theme_dir: &Path,
    theme_name: &str,
    svg_paths: &mut Vec<PathBuf>,
    rerun_paths: &mut Vec<PathBuf>,
) -> Vec<String> {
    // Collect referenced SVG paths
    for value in mapping.values() {
        if let Some(name) = value.default_name() {
            let svg_path = theme_dir.join(format!("{name}.svg"));
            if svg_path.exists() {
                rerun_paths.push(svg_path.clone());
                svg_paths.push(svg_path);
            }
        }
    }

    validate::check_orphan_svgs(mapping, theme_dir, theme_name)
}

/// Merge multiple configs into a single MasterConfig for code generation.
fn merge_configs(configs: &[(String, MasterConfig)], enum_name_override: Option<&str>) -> MasterConfig {
    let name = enum_name_override
        .map(|s| s.to_string())
        .unwrap_or_else(|| configs[0].1.name.clone());

    let mut roles = Vec::new();
    let mut bundled_themes = Vec::new();
    let mut system_themes = Vec::new();
    let mut seen_bundled = std::collections::BTreeSet::new();
    let mut seen_system = std::collections::BTreeSet::new();

    for (_path, config) in configs {
        roles.extend(config.roles.iter().cloned());

        for t in &config.bundled_themes {
            if seen_bundled.insert(t.clone()) {
                bundled_themes.push(t.clone());
            }
        }
        for t in &config.system_themes {
            if seen_system.insert(t.clone()) {
                system_themes.push(t.clone());
            }
        }
    }

    MasterConfig {
        name,
        roles,
        bundled_themes,
        system_themes,
    }
}

/// Simple API: generate icon code from a single TOML file.
///
/// Reads the master TOML at `toml_path`, validates all referenced themes
/// and SVG files, and writes generated Rust code to `OUT_DIR`.
///
/// # Panics
///
/// Calls `process::exit(1)` if validation errors are found.
pub fn generate_icons(toml_path: impl AsRef<Path>) {
    let toml_path = toml_path.as_ref();
    let manifest_dir = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"),
    );
    let resolved = manifest_dir.join(toml_path);

    let content = std::fs::read_to_string(&resolved)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", resolved.display()));
    let config: MasterConfig = toml::from_str(&content)
        .unwrap_or_else(|e| panic!("failed to parse {}: {e}", resolved.display()));

    let base_dir = resolved
        .parent()
        .expect("TOML path has no parent")
        .to_path_buf();
    let file_path_str = resolved.to_string_lossy().to_string();

    let result = run_pipeline(
        &[(file_path_str, config)],
        &[base_dir],
        None,
        Some(&manifest_dir),
    );

    emit_result(result);
}

/// Builder API for composing multiple TOML icon definitions.
pub struct IconGenerator {
    sources: Vec<PathBuf>,
    enum_name_override: Option<String>,
}

impl IconGenerator {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            enum_name_override: None,
        }
    }

    /// Add a TOML icon definition file.
    pub fn add(mut self, path: impl AsRef<Path>) -> Self {
        self.sources.push(path.as_ref().to_path_buf());
        self
    }

    /// Override the generated enum name.
    pub fn enum_name(mut self, name: &str) -> Self {
        self.enum_name_override = Some(name.to_string());
        self
    }

    /// Run the full pipeline: load, validate, generate, write.
    ///
    /// # Panics
    ///
    /// Calls `process::exit(1)` if validation errors are found.
    pub fn generate(self) {
        let manifest_dir = PathBuf::from(
            std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"),
        );

        let mut configs = Vec::new();
        let mut base_dirs = Vec::new();

        for source in &self.sources {
            let resolved = manifest_dir.join(source);
            let content = std::fs::read_to_string(&resolved)
                .unwrap_or_else(|e| panic!("failed to read {}: {e}", resolved.display()));
            let config: MasterConfig = toml::from_str(&content)
                .unwrap_or_else(|e| panic!("failed to parse {}: {e}", resolved.display()));

            let base_dir = resolved
                .parent()
                .expect("TOML path has no parent")
                .to_path_buf();
            let file_path_str = resolved.to_string_lossy().to_string();

            configs.push((file_path_str, config));
            base_dirs.push(base_dir);
        }

        let result = run_pipeline(
            &configs,
            &base_dirs,
            self.enum_name_override.as_deref(),
            Some(&manifest_dir),
        );

        emit_result(result);
    }
}

/// Emit cargo directives, write output file, or exit on errors.
fn emit_result(result: PipelineResult) {
    // Emit rerun-if-changed for all tracked paths
    for path in &result.rerun_paths {
        println!("cargo::rerun-if-changed={}", path.display());
    }

    // Emit errors and exit if any
    if !result.errors.is_empty() {
        for e in &result.errors {
            println!("cargo::error={e}");
        }
        std::process::exit(1);
    }

    // Emit warnings
    for w in &result.warnings {
        println!("cargo::warning={w}");
    }

    // Write output file
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not set"));
    let out_path = out_dir.join(&result.output_filename);
    std::fs::write(&out_path, &result.code)
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", out_path.display()));

    // Emit size report
    if let Some(report) = &result.size_report {
        let kb = report.total_svg_bytes as f64 / 1024.0;
        println!(
            "cargo::warning={} roles x {} bundled themes = {} SVGs, {:.1} KB total",
            report.role_count, report.bundled_theme_count, report.svg_count, kb
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::fs;

    // === MasterConfig tests ===

    #[test]
    fn master_config_deserializes_full() {
        let toml_str = r#"
name = "app-icon"
roles = ["play-pause", "skip-forward"]
bundled-themes = ["material"]
system-themes = ["sf-symbols"]
"#;
        let config: MasterConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.name, "app-icon");
        assert_eq!(config.roles, vec!["play-pause", "skip-forward"]);
        assert_eq!(config.bundled_themes, vec!["material"]);
        assert_eq!(config.system_themes, vec!["sf-symbols"]);
    }

    #[test]
    fn master_config_empty_optional_fields() {
        let toml_str = r#"
name = "x"
roles = ["a"]
"#;
        let config: MasterConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.name, "x");
        assert_eq!(config.roles, vec!["a"]);
        assert!(config.bundled_themes.is_empty());
        assert!(config.system_themes.is_empty());
    }

    #[test]
    fn master_config_rejects_unknown_fields() {
        let toml_str = r#"
name = "x"
roles = ["a"]
bogus = "nope"
"#;
        let result = toml::from_str::<MasterConfig>(toml_str);
        assert!(result.is_err());
    }

    // === MappingValue tests ===

    #[test]
    fn mapping_value_simple() {
        let toml_str = r#"play-pause = "play_pause""#;
        let mapping: BTreeMap<String, MappingValue> = toml::from_str(toml_str).unwrap();
        match &mapping["play-pause"] {
            MappingValue::Simple(s) => assert_eq!(s, "play_pause"),
            _ => panic!("expected Simple variant"),
        }
    }

    #[test]
    fn mapping_value_de_aware() {
        let toml_str =
            r#"play-pause = { kde = "media-playback-start", default = "play" }"#;
        let mapping: BTreeMap<String, MappingValue> = toml::from_str(toml_str).unwrap();
        match &mapping["play-pause"] {
            MappingValue::DeAware(m) => {
                assert_eq!(m["kde"], "media-playback-start");
                assert_eq!(m["default"], "play");
            }
            _ => panic!("expected DeAware variant"),
        }
    }

    #[test]
    fn theme_mapping_mixed_values() {
        let toml_str = r#"
play-pause = "play_pause"
bluetooth = { kde = "preferences-system-bluetooth", default = "bluetooth" }
skip-forward = "skip_next"
"#;
        let mapping: ThemeMapping = toml::from_str(toml_str).unwrap();
        assert_eq!(mapping.len(), 3);
        assert!(matches!(&mapping["play-pause"], MappingValue::Simple(_)));
        assert!(matches!(&mapping["bluetooth"], MappingValue::DeAware(_)));
        assert!(matches!(&mapping["skip-forward"], MappingValue::Simple(_)));
    }

    // === MappingValue::default_name tests ===

    #[test]
    fn mapping_value_default_name_simple() {
        let val = MappingValue::Simple("play_pause".to_string());
        assert_eq!(val.default_name(), Some("play_pause"));
    }

    #[test]
    fn mapping_value_default_name_de_aware() {
        let mut m = BTreeMap::new();
        m.insert("kde".to_string(), "media-playback-start".to_string());
        m.insert("default".to_string(), "play".to_string());
        let val = MappingValue::DeAware(m);
        assert_eq!(val.default_name(), Some("play"));
    }

    #[test]
    fn mapping_value_default_name_de_aware_missing_default() {
        let mut m = BTreeMap::new();
        m.insert("kde".to_string(), "media-playback-start".to_string());
        let val = MappingValue::DeAware(m);
        assert_eq!(val.default_name(), None);
    }

    // === BuildError Display tests ===

    #[test]
    fn build_error_missing_role_format() {
        let err = BuildError::MissingRole {
            role: "play-pause".into(),
            mapping_file: "icons/material/mapping.toml".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("play-pause"), "should contain role name");
        assert!(
            msg.contains("icons/material/mapping.toml"),
            "should contain file path"
        );
    }

    #[test]
    fn build_error_missing_svg_format() {
        let err = BuildError::MissingSvg {
            path: "icons/material/play.svg".into(),
        };
        let msg = err.to_string();
        assert!(
            msg.contains("icons/material/play.svg"),
            "should contain SVG path"
        );
    }

    #[test]
    fn build_error_unknown_role_format() {
        let err = BuildError::UnknownRole {
            role: "bogus".into(),
            mapping_file: "icons/material/mapping.toml".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("bogus"), "should contain role name");
        assert!(
            msg.contains("icons/material/mapping.toml"),
            "should contain file path"
        );
    }

    #[test]
    fn build_error_unknown_theme_format() {
        let err = BuildError::UnknownTheme {
            theme: "nonexistent".into(),
        };
        let msg = err.to_string();
        assert!(
            msg.contains("nonexistent"),
            "should contain theme name"
        );
    }

    #[test]
    fn build_error_missing_default_format() {
        let err = BuildError::MissingDefault {
            role: "bluetooth".into(),
            mapping_file: "icons/freedesktop/mapping.toml".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("bluetooth"), "should contain role name");
        assert!(
            msg.contains("icons/freedesktop/mapping.toml"),
            "should contain file path"
        );
    }

    #[test]
    fn build_error_duplicate_role_format() {
        let err = BuildError::DuplicateRole {
            role: "play-pause".into(),
            file_a: "icons/a.toml".into(),
            file_b: "icons/b.toml".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("play-pause"), "should contain role name");
        assert!(
            msg.contains("icons/a.toml"),
            "should contain first file path"
        );
        assert!(
            msg.contains("icons/b.toml"),
            "should contain second file path"
        );
    }

    // === KNOWN_THEMES tests ===

    #[test]
    fn known_themes_has_all_five() {
        assert_eq!(KNOWN_THEMES.len(), 5);
        assert!(KNOWN_THEMES.contains(&"sf-symbols"));
        assert!(KNOWN_THEMES.contains(&"segoe-fluent"));
        assert!(KNOWN_THEMES.contains(&"freedesktop"));
        assert!(KNOWN_THEMES.contains(&"material"));
        assert!(KNOWN_THEMES.contains(&"lucide"));
    }

    // === Helper to create test fixture directories ===

    fn create_fixture_dir(suffix: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("native_theme_test_pipeline_{suffix}"));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn write_fixture(dir: &Path, path: &str, content: &str) {
        let full_path = dir.join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(full_path, content).unwrap();
    }

    const SVG_STUB: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"></svg>"#;

    // === run_pipeline tests ===

    #[test]
    fn pipeline_happy_path_generates_code() {
        let dir = create_fixture_dir("happy");
        write_fixture(&dir, "material/mapping.toml", r#"
play-pause = "play_pause"
skip-forward = "skip_next"
"#);
        write_fixture(&dir, "sf-symbols/mapping.toml", r#"
play-pause = "play.fill"
skip-forward = "forward.fill"
"#);
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);
        write_fixture(&dir, "material/skip_next.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(r#"
name = "sample-icon"
roles = ["play-pause", "skip-forward"]
bundled-themes = ["material"]
system-themes = ["sf-symbols"]
"#).unwrap();

        let result = run_pipeline(
            &[("sample-icons.toml".to_string(), config)],
            &[dir.clone()],
            None,
            None,
        );

        assert!(result.errors.is_empty(), "expected no errors: {:?}", result.errors);
        assert!(!result.code.is_empty(), "expected generated code");
        assert!(result.code.contains("pub enum SampleIcon"));
        assert!(result.code.contains("PlayPause"));
        assert!(result.code.contains("SkipForward"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_output_filename_uses_snake_case() {
        let dir = create_fixture_dir("filename");
        write_fixture(&dir, "material/mapping.toml", "play-pause = \"play_pause\"\n");
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(r#"
name = "app-icon"
roles = ["play-pause"]
bundled-themes = ["material"]
"#).unwrap();

        let result = run_pipeline(
            &[("app.toml".to_string(), config)],
            &[dir.clone()],
            None,
            None,
        );

        assert_eq!(result.output_filename, "app_icon.rs");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_collects_rerun_paths() {
        let dir = create_fixture_dir("rerun");
        write_fixture(&dir, "material/mapping.toml", r#"
play-pause = "play_pause"
"#);
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(r#"
name = "test"
roles = ["play-pause"]
bundled-themes = ["material"]
"#).unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            &[dir.clone()],
            None,
            None,
        );

        assert!(result.errors.is_empty());
        // Should include: master TOML, mapping TOML, theme dir, SVG files
        let path_strs: Vec<String> = result.rerun_paths.iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();
        assert!(path_strs.iter().any(|p| p.contains("test.toml")), "should track master TOML");
        assert!(path_strs.iter().any(|p| p.contains("mapping.toml")), "should track mapping TOML");
        assert!(path_strs.iter().any(|p| p.contains("play_pause.svg")), "should track SVG files");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_emits_size_report() {
        let dir = create_fixture_dir("size");
        write_fixture(&dir, "material/mapping.toml", "play-pause = \"play_pause\"\n");
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(r#"
name = "test"
roles = ["play-pause"]
bundled-themes = ["material"]
"#).unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            &[dir.clone()],
            None,
            None,
        );

        assert!(result.errors.is_empty());
        let report = result.size_report.as_ref().expect("should have size report");
        assert_eq!(report.role_count, 1);
        assert_eq!(report.bundled_theme_count, 1);
        assert_eq!(report.svg_count, 1);
        assert!(report.total_svg_bytes > 0, "SVGs should have nonzero size");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_returns_errors_on_missing_role() {
        let dir = create_fixture_dir("missing_role");
        // Mapping is missing "skip-forward"
        write_fixture(&dir, "material/mapping.toml", "play-pause = \"play_pause\"\n");
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(r#"
name = "test"
roles = ["play-pause", "skip-forward"]
bundled-themes = ["material"]
"#).unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            &[dir.clone()],
            None,
            None,
        );

        assert!(!result.errors.is_empty(), "should have errors");
        assert!(result.errors.iter().any(|e| e.contains("skip-forward")), "should mention missing role");
        assert!(result.code.is_empty(), "no code on errors");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_returns_errors_on_missing_svg() {
        let dir = create_fixture_dir("missing_svg");
        write_fixture(&dir, "material/mapping.toml", r#"
play-pause = "play_pause"
skip-forward = "skip_next"
"#);
        // Only create one SVG, leave skip_next.svg missing
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(r#"
name = "test"
roles = ["play-pause", "skip-forward"]
bundled-themes = ["material"]
"#).unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            &[dir.clone()],
            None,
            None,
        );

        assert!(!result.errors.is_empty(), "should have errors");
        assert!(result.errors.iter().any(|e| e.contains("skip_next.svg")), "should mention missing SVG");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_orphan_svgs_are_warnings() {
        let dir = create_fixture_dir("orphan_warn");
        write_fixture(&dir, "material/mapping.toml", "play-pause = \"play_pause\"\n");
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);
        write_fixture(&dir, "material/unused.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(r#"
name = "test"
roles = ["play-pause"]
bundled-themes = ["material"]
"#).unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            &[dir.clone()],
            None,
            None,
        );

        assert!(result.errors.is_empty(), "orphans are not errors");
        assert!(!result.warnings.is_empty(), "should have orphan warning");
        assert!(result.warnings.iter().any(|w| w.contains("unused.svg")));

        let _ = fs::remove_dir_all(&dir);
    }

    // === merge_configs tests ===

    #[test]
    fn merge_configs_combines_roles() {
        let config_a: MasterConfig = toml::from_str(r#"
name = "a"
roles = ["play-pause"]
bundled-themes = ["material"]
"#).unwrap();
        let config_b: MasterConfig = toml::from_str(r#"
name = "b"
roles = ["skip-forward"]
bundled-themes = ["material"]
system-themes = ["sf-symbols"]
"#).unwrap();

        let configs = vec![
            ("a.toml".to_string(), config_a),
            ("b.toml".to_string(), config_b),
        ];
        let merged = merge_configs(&configs, None);

        assert_eq!(merged.name, "a"); // uses first config's name
        assert_eq!(merged.roles, vec!["play-pause", "skip-forward"]);
        assert_eq!(merged.bundled_themes, vec!["material"]); // deduplicated
        assert_eq!(merged.system_themes, vec!["sf-symbols"]);
    }

    #[test]
    fn merge_configs_uses_enum_name_override() {
        let config: MasterConfig = toml::from_str(r#"
name = "original"
roles = ["x"]
"#).unwrap();

        let configs = vec![("a.toml".to_string(), config)];
        let merged = merge_configs(&configs, Some("MyIcons"));

        assert_eq!(merged.name, "MyIcons");
    }

    // === Builder pipeline tests ===

    #[test]
    fn pipeline_builder_merges_two_files() {
        let dir = create_fixture_dir("builder_merge");
        write_fixture(&dir, "material/mapping.toml", r#"
play-pause = "play_pause"
skip-forward = "skip_next"
"#);
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);
        write_fixture(&dir, "material/skip_next.svg", SVG_STUB);

        let config_a: MasterConfig = toml::from_str(r#"
name = "icons-a"
roles = ["play-pause"]
bundled-themes = ["material"]
"#).unwrap();
        let config_b: MasterConfig = toml::from_str(r#"
name = "icons-b"
roles = ["skip-forward"]
bundled-themes = ["material"]
"#).unwrap();

        let result = run_pipeline(
            &[
                ("a.toml".to_string(), config_a),
                ("b.toml".to_string(), config_b),
            ],
            &[dir.clone(), dir.clone()],
            Some("AllIcons"),
            None,
        );

        assert!(result.errors.is_empty(), "expected no errors: {:?}", result.errors);
        assert!(result.code.contains("pub enum AllIcons"), "should use override name");
        assert!(result.code.contains("PlayPause"));
        assert!(result.code.contains("SkipForward"));
        assert_eq!(result.output_filename, "all_icons.rs");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_builder_detects_duplicate_roles() {
        let dir = create_fixture_dir("builder_dup");
        write_fixture(&dir, "material/mapping.toml", "play-pause = \"play_pause\"\n");
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        let config_a: MasterConfig = toml::from_str(r#"
name = "a"
roles = ["play-pause"]
bundled-themes = ["material"]
"#).unwrap();
        let config_b: MasterConfig = toml::from_str(r#"
name = "b"
roles = ["play-pause"]
bundled-themes = ["material"]
"#).unwrap();

        let result = run_pipeline(
            &[
                ("a.toml".to_string(), config_a),
                ("b.toml".to_string(), config_b),
            ],
            &[dir.clone(), dir.clone()],
            None,
            None,
        );

        assert!(!result.errors.is_empty(), "should detect duplicate roles");
        assert!(result.errors.iter().any(|e| e.contains("play-pause")));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_generates_relative_include_bytes_paths() {
        // Simulate what generate_icons does: manifest_dir + "icons/icons.toml"
        // The tmpdir acts as CARGO_MANIFEST_DIR.
        // base_dir is absolute (tmpdir/icons), but run_pipeline should strip
        // the manifest_dir prefix for codegen, producing relative paths.
        let tmpdir = create_fixture_dir("rel_paths");
        write_fixture(&tmpdir, "icons/material/mapping.toml", "play-pause = \"play_pause\"\n");
        write_fixture(&tmpdir, "icons/material/play_pause.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(r#"
name = "test"
roles = ["play-pause"]
bundled-themes = ["material"]
"#).unwrap();

        // base_dir is absolute (as generate_icons would compute)
        let abs_base_dir = tmpdir.join("icons");

        let result = run_pipeline(
            &[("icons/icons.toml".to_string(), config)],
            &[abs_base_dir],
            None,
            Some(&tmpdir), // manifest_dir for stripping prefix
        );

        assert!(result.errors.is_empty(), "errors: {:?}", result.errors);
        // The include_bytes path should contain "/icons/material/play_pause.svg"
        assert!(
            result.code.contains("\"/icons/material/play_pause.svg\""),
            "include_bytes path should use relative base_dir 'icons'. code:\n{}",
            result.code,
        );
        // The include_bytes path should NOT contain the absolute tmpdir
        let tmpdir_str = tmpdir.to_string_lossy();
        assert!(
            !result.code.contains(&*tmpdir_str),
            "include_bytes path should NOT contain absolute tmpdir path",
        );

        let _ = fs::remove_dir_all(&tmpdir);
    }

    #[test]
    fn pipeline_no_system_svg_check() {
        // System themes should NOT validate SVGs
        let dir = create_fixture_dir("no_sys_svg");
        // sf-symbols has mapping but NO SVG files -- should be fine
        write_fixture(&dir, "sf-symbols/mapping.toml", r#"
play-pause = "play.fill"
"#);

        let config: MasterConfig = toml::from_str(r#"
name = "test"
roles = ["play-pause"]
system-themes = ["sf-symbols"]
"#).unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            &[dir.clone()],
            None,
            None,
        );

        assert!(result.errors.is_empty(), "system themes should not require SVGs: {:?}", result.errors);

        let _ = fs::remove_dir_all(&dir);
    }
}
