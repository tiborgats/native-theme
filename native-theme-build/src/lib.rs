//! Build-time code generation for native-theme custom icon roles.
//!
//! This crate reads TOML icon definitions at build time and generates a Rust
//! enum that implements `native_theme::IconProvider`. The generated enum maps
//! each icon role to platform-specific identifiers (SF Symbols, Segoe Fluent,
//! freedesktop, Material, Lucide) and optionally embeds bundled SVG data via
//! `include_bytes!`.
//!
//! # TOML Schema
//!
//! The master TOML file declares the icon set name, roles, and which themes to
//! support:
//!
//! ```toml
//! name = "app-icon"
//! roles = ["play-pause", "skip-forward", "volume-up"]
//! bundled-themes = ["material"]
//! system-themes = ["sf-symbols", "segoe-fluent", "freedesktop"]
//! ```
//!
//! - **`name`** -- used to derive the generated enum name (`AppIcon`).
//! - **`roles`** -- kebab-case role names; each becomes a PascalCase enum variant.
//! - **`bundled-themes`** -- themes whose SVGs are embedded via `include_bytes!`.
//! - **`system-themes`** -- themes resolved at runtime by the OS (no embedded SVGs).
//!
//! # Directory Layout
//!
//! ```text
//! icons/
//!   icons.toml           # Master TOML (the file passed to generate_icons)
//!   material/
//!     mapping.toml       # Role -> SVG filename mappings
//!     play_pause.svg
//!     skip_next.svg
//!     volume_up.svg
//!   sf-symbols/
//!     mapping.toml       # Role -> SF Symbol name mappings
//!   segoe-fluent/
//!     mapping.toml       # Role -> Segoe codepoint mappings
//!   freedesktop/
//!     mapping.toml       # Role -> freedesktop icon name mappings
//! ```
//!
//! # Mapping TOML
//!
//! Each theme directory contains a `mapping.toml` that maps roles to
//! theme-specific identifiers. Simple form:
//!
//! ```toml
//! play-pause = "play_pause"
//! skip-forward = "skip_next"
//! volume-up = "volume_up"
//! ```
//!
//! DE-aware form (for freedesktop themes that vary by desktop environment):
//!
//! ```toml
//! play-pause = { kde = "media-playback-start", default = "media-play" }
//! ```
//!
//! A `default` key is required for every DE-aware entry.
//!
//! # build.rs Setup
//!
//! ```rust,ignore
//! // Simple API (single TOML file):
//! native_theme_build::generate_icons("icons/icons.toml")
//!     .unwrap_or_exit()
//!     .emit_cargo_directives();
//!
//! // Builder API (multiple TOML files, custom enum name):
//! native_theme_build::IconGenerator::new()
//!     .source("icons/media.toml")
//!     .source("icons/navigation.toml")
//!     .enum_name("AppIcon")
//!     .generate()
//!     .unwrap_or_exit()
//!     .emit_cargo_directives();
//! ```
//!
//! Both APIs resolve paths relative to `CARGO_MANIFEST_DIR`, and return a
//! [`Result`] with a [`GenerateOutput`] on success or [`BuildErrors`] on
//! failure. Call [`GenerateOutput::emit_cargo_directives()`] to write the
//! output file and emit `cargo::rerun-if-changed` / `cargo::warning`
//! directives.
//!
//! The [`UnwrapOrExit`] trait provides `.unwrap_or_exit()` as a drop-in
//! replacement for the old `process::exit(1)` behaviour.
//!
//! # Using the Generated Code
//!
//! ```rust,ignore
//! // In your lib.rs or main.rs:
//! include!(concat!(env!("OUT_DIR"), "/app_icon.rs"));
//!
//! // The generated enum implements IconProvider:
//! use native_theme::load_custom_icon;
//! let icon_data = load_custom_icon(&AppIcon::PlayPause, "material");
//! ```
//!
//! # What Gets Generated
//!
//! The output is a single `.rs` file containing:
//!
//! - A `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]` enum with one
//!   variant per role.
//! - An `IconProvider` implementation with `icon_name()` returning the
//!   platform-specific identifier and `icon_svg()` returning
//!   `include_bytes!(...)` data for bundled themes.
//!
//! # Validation
//!
//! Build errors are emitted at compile time for:
//!
//! - Missing roles in mapping files (every role must be present in every theme).
//! - Missing SVG files for bundled themes.
//! - Unknown role names in mapping files (not declared in the master TOML).
//! - Duplicate roles across multiple TOML files (builder API).
//! - Missing `default` key in DE-aware mapping entries.

#![warn(missing_docs)]
#![forbid(unsafe_code)]

mod codegen;
mod error;
mod schema;
mod validate;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use heck::ToSnakeCase;

pub use error::{BuildError, BuildErrors};
use schema::{MasterConfig, ThemeMapping};

#[cfg(test)]
use schema::{MappingValue, THEME_TABLE};

/// Output of a successful icon generation pipeline.
///
/// Contains the generated code, metadata about what was generated, and all
/// information needed to emit cargo directives. Call
/// [`emit_cargo_directives()`](Self::emit_cargo_directives) to write the
/// output file and print `cargo::rerun-if-changed` / `cargo::warning` lines.
#[derive(Debug, Clone)]
#[must_use = "call .emit_cargo_directives() to write the file and emit cargo directives"]
pub struct GenerateOutput {
    /// Path where the generated `.rs` file will be written.
    pub output_path: PathBuf,
    /// Warnings collected during generation (e.g., orphan SVGs, unknown DE keys).
    pub warnings: Vec<String>,
    /// Number of icon roles in the generated enum.
    pub role_count: usize,
    /// Number of bundled themes (themes with embedded SVGs).
    pub bundled_theme_count: usize,
    /// Total number of SVG files embedded.
    pub svg_count: usize,
    /// Total byte size of all embedded SVGs.
    pub total_svg_bytes: u64,
    /// Paths that cargo should watch for changes.
    rerun_paths: Vec<PathBuf>,
    /// The generated Rust source code.
    pub code: String,
}

impl GenerateOutput {
    /// Return the paths that cargo should watch for changes.
    pub fn rerun_paths(&self) -> &[PathBuf] {
        &self.rerun_paths
    }

    /// Emit cargo directives, write the generated file, and print warnings.
    ///
    /// This prints `cargo::rerun-if-changed` for all tracked paths, writes the
    /// generated code to [`output_path`](Self::output_path), and prints warnings.
    ///
    /// On I/O failure, emits a `cargo::error=` diagnostic and exits the process
    /// with code 1 -- matching the `UnwrapOrExit` error-handling pattern.
    pub fn emit_cargo_directives(&self) {
        for path in &self.rerun_paths {
            println!("cargo::rerun-if-changed={}", path.display());
        }
        if let Err(e) = std::fs::write(&self.output_path, &self.code) {
            println!(
                "cargo::error=failed to write {}: {e}",
                self.output_path.display()
            );
            std::process::exit(1);
        }
        for w in &self.warnings {
            println!("cargo::warning={w}");
        }
    }
}

/// Extension trait for converting `Result<GenerateOutput, BuildErrors>` into
/// a direct output with `process::exit(1)` on error.
///
/// Provides a drop-in migration path from the old `generate_icons()` API
/// that called `process::exit` internally.
///
/// # Example
///
/// ```rust,ignore
/// use native_theme_build::UnwrapOrExit;
///
/// native_theme_build::generate_icons("icons/icons.toml")
///     .unwrap_or_exit()
///     .emit_cargo_directives();
/// ```
pub trait UnwrapOrExit<T> {
    /// Unwrap the `Ok` value or emit cargo errors and exit the process.
    fn unwrap_or_exit(self) -> T;
}

impl UnwrapOrExit<GenerateOutput> for Result<GenerateOutput, BuildErrors> {
    fn unwrap_or_exit(self) -> GenerateOutput {
        match self {
            Ok(output) => output,
            Err(errors) => {
                // Emit rerun-if-changed even on error so cargo re-checks when
                // the user fixes the files. We don't have the paths here, but
                // the build.rs will re-run anyway since it exited with failure.
                errors.emit_cargo_errors();
                std::process::exit(1);
            }
        }
    }
}

/// Simple API: generate icon code from a single TOML file.
///
/// Reads the master TOML at `toml_path` (relative to `CARGO_MANIFEST_DIR`),
/// validates all referenced themes and SVG files, and returns a
/// [`GenerateOutput`] on success or [`BuildErrors`] on failure.
///
/// Call [`GenerateOutput::emit_cargo_directives()`] on the result to write
/// the generated file and emit cargo directives.
///
/// # Errors
///
/// Returns [`BuildErrors`] if `CARGO_MANIFEST_DIR` or `OUT_DIR` environment
/// variables are not set, if the TOML file cannot be read or parsed, or if
/// the icon pipeline detects missing roles, SVGs, or invalid mappings.
#[must_use = "this returns the generated output; call .emit_cargo_directives() to complete the build"]
pub fn generate_icons(toml_path: impl AsRef<Path>) -> Result<GenerateOutput, BuildErrors> {
    let toml_path = toml_path.as_ref();
    let manifest_dir = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .map_err(|e| BuildErrors::io(format!("CARGO_MANIFEST_DIR not set: {e}")))?,
    );
    let out_dir = PathBuf::from(
        std::env::var("OUT_DIR").map_err(|e| BuildErrors::io(format!("OUT_DIR not set: {e}")))?,
    );
    let resolved = manifest_dir.join(toml_path);

    let content = std::fs::read_to_string(&resolved)
        .map_err(|e| BuildErrors::io(format!("failed to read {}: {e}", resolved.display())))?;
    let config: MasterConfig = toml::from_str(&content)
        .map_err(|e| BuildErrors::io(format!("failed to parse {}: {e}", resolved.display())))?;

    let base_dir = resolved
        .parent()
        .ok_or_else(|| BuildErrors::io(format!("{} has no parent directory", resolved.display())))?
        .to_path_buf();
    let file_path_str = resolved.to_string_lossy().to_string();

    let result = run_pipeline(
        &[(file_path_str, config)],
        &[base_dir],
        None,
        Some(&manifest_dir),
        None,
        &[],
    );

    pipeline_result_to_output(result, &out_dir)
}

/// Builder API for composing multiple TOML icon definitions.
#[derive(Debug)]
#[must_use = "a configured builder does nothing until .generate() is called"]
pub struct IconGenerator {
    sources: Vec<PathBuf>,
    enum_name_override: Option<String>,
    base_dir: Option<PathBuf>,
    crate_path: Option<String>,
    extra_derives: Vec<String>,
    output_dir: Option<PathBuf>,
}

impl Default for IconGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl IconGenerator {
    /// Create a new builder.
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            enum_name_override: None,
            base_dir: None,
            crate_path: None,
            extra_derives: Vec::new(),
            output_dir: None,
        }
    }

    /// Add a TOML icon definition file.
    pub fn source(mut self, path: impl AsRef<Path>) -> Self {
        self.sources.push(path.as_ref().to_path_buf());
        self
    }

    /// Override the generated enum name.
    pub fn enum_name(mut self, name: &str) -> Self {
        self.enum_name_override = Some(name.to_string());
        self
    }

    /// Set the base directory for theme resolution.
    ///
    /// When set, all theme directories (e.g., `material/`, `sf-symbols/`) are
    /// resolved relative to this path instead of the parent directory of each
    /// TOML source file.
    ///
    /// When not set and multiple sources have different parent directories,
    /// `generate()` returns an error.
    pub fn base_dir(mut self, path: impl AsRef<Path>) -> Self {
        self.base_dir = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set the Rust crate path prefix used in generated code.
    ///
    /// Defaults to `"native_theme"`. When the default is used, the generated
    /// file includes `extern crate native_theme;` to support Cargo aliases.
    ///
    /// Set this to a custom path (e.g. `"my_crate::native_theme"`) when
    /// re-exporting native-theme from another crate.
    pub fn crate_path(mut self, path: &str) -> Self {
        assert!(
            !path.is_empty() && !path.contains(' '),
            "crate_path must be non-empty and contain no spaces: {path:?}"
        );
        self.crate_path = Some(path.to_string());
        self
    }

    /// Add an extra `#[derive(...)]` trait to the generated enum.
    ///
    /// The base set (`Debug, Clone, Copy, PartialEq, Eq, Hash`) is always
    /// emitted. Each call appends one additional derive.
    ///
    /// ```rust,ignore
    /// native_theme_build::IconGenerator::new()
    ///     .source("icons/icons.toml")
    ///     .derive("Ord")
    ///     .derive("serde::Serialize")
    ///     .generate()
    ///     .unwrap_or_exit();
    /// ```
    pub fn derive(mut self, name: &str) -> Self {
        assert!(
            !name.is_empty() && !name.contains(char::is_whitespace),
            "derive name must be non-empty and contain no whitespace: {name:?}"
        );
        self.extra_derives.push(name.to_string());
        self
    }

    /// Set an explicit output directory for the generated `.rs` file.
    ///
    /// When not set, the `OUT_DIR` environment variable is used (always
    /// available during `cargo build`). Set this when running outside of
    /// a build script context (e.g., in integration tests).
    pub fn output_dir(mut self, path: impl AsRef<Path>) -> Self {
        self.output_dir = Some(path.as_ref().to_path_buf());
        self
    }

    /// Run the full pipeline: load, validate, generate.
    ///
    /// Returns a [`GenerateOutput`] on success or [`BuildErrors`] on failure.
    /// Call [`GenerateOutput::emit_cargo_directives()`] on the result to write
    /// the generated file and emit cargo directives.
    ///
    /// Source paths may be absolute or relative. Relative paths are resolved
    /// against `CARGO_MANIFEST_DIR`. When all source paths are absolute,
    /// `CARGO_MANIFEST_DIR` is not required.
    ///
    /// # Errors
    ///
    /// Returns [`BuildErrors`] if `CARGO_MANIFEST_DIR` is not set and a
    /// relative source path is used, or if neither
    /// [`output_dir()`](Self::output_dir) nor `OUT_DIR` is set.
    pub fn generate(self) -> Result<GenerateOutput, BuildErrors> {
        if self.sources.is_empty() {
            return Err(BuildErrors::io(
                "no source files added to IconGenerator (call .source() before .generate())",
            ));
        }

        let needs_manifest_dir = self.sources.iter().any(|s| !s.is_absolute())
            || self.base_dir.as_ref().is_some_and(|b| !b.is_absolute());
        let manifest_dir = if needs_manifest_dir {
            Some(PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").map_err(
                |e| BuildErrors::io(format!("CARGO_MANIFEST_DIR not set: {e}")),
            )?))
        } else {
            std::env::var("CARGO_MANIFEST_DIR").ok().map(PathBuf::from)
        };

        let out_dir = match self.output_dir {
            Some(dir) => dir,
            None => PathBuf::from(
                std::env::var("OUT_DIR")
                    .map_err(|e| BuildErrors::io(format!("OUT_DIR not set: {e}")))?,
            ),
        };

        let mut configs = Vec::new();
        let mut base_dirs = Vec::new();

        for source in &self.sources {
            let resolved = if source.is_absolute() {
                source.clone()
            } else {
                manifest_dir
                    .as_ref()
                    .ok_or_else(|| {
                        BuildErrors::io(format!(
                            "CARGO_MANIFEST_DIR required for relative path {}",
                            source.display()
                        ))
                    })?
                    .join(source)
            };
            let content = std::fs::read_to_string(&resolved).map_err(|e| {
                BuildErrors::io(format!("failed to read {}: {e}", resolved.display()))
            })?;
            let config: MasterConfig = toml::from_str(&content).map_err(|e| {
                BuildErrors::io(format!("failed to parse {}: {e}", resolved.display()))
            })?;

            let file_path_str = resolved.to_string_lossy().to_string();

            if let Some(ref explicit_base) = self.base_dir {
                let base = if explicit_base.is_absolute() {
                    explicit_base.clone()
                } else {
                    manifest_dir
                        .as_ref()
                        .ok_or_else(|| {
                            BuildErrors::io(format!(
                                "CARGO_MANIFEST_DIR required for relative base_dir {}",
                                explicit_base.display()
                            ))
                        })?
                        .join(explicit_base)
                };
                base_dirs.push(base);
            } else {
                let parent = resolved
                    .parent()
                    .ok_or_else(|| {
                        BuildErrors::io(format!("{} has no parent directory", resolved.display()))
                    })?
                    .to_path_buf();
                base_dirs.push(parent);
            }

            configs.push((file_path_str, config));
        }

        // If no explicit base_dir and multiple sources have different parent dirs, error
        if self.base_dir.is_none() && base_dirs.len() > 1 {
            let first = &base_dirs[0];
            let divergent = base_dirs.iter().any(|d| d != first);
            if divergent {
                return Err(BuildErrors::io(
                    "multiple source files have different parent directories; \
                     use .base_dir() to specify a common base directory for theme resolution",
                ));
            }
        }

        let result = run_pipeline(
            &configs,
            &base_dirs,
            self.enum_name_override.as_deref(),
            manifest_dir.as_deref(),
            self.crate_path.as_deref(),
            &self.extra_derives,
        );

        pipeline_result_to_output(result, &out_dir)
    }
}

/// Result of running the pure pipeline core.
///
/// Contains the generated code, collected errors, and collected warnings.
/// The thin outer layer ([`generate_icons()`] / [`IconGenerator::generate()`])
/// converts this into `Result<GenerateOutput, BuildErrors>`.
struct PipelineResult {
    /// Generated Rust source code (empty if errors were found).
    pub code: String,
    /// Build errors found during validation.
    pub errors: Vec<BuildError>,
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
struct SizeReport {
    /// Number of icon roles.
    pub role_count: usize,
    /// Number of bundled themes.
    pub bundled_theme_count: usize,
    /// Total bytes of all SVGs.
    pub total_svg_bytes: u64,
    /// Number of SVG files.
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
///
/// `crate_path` controls the Rust path prefix used in generated code
/// (e.g. `"native_theme"` or `"my_crate::native_theme"`).
fn run_pipeline(
    configs: &[(String, MasterConfig)],
    base_dirs: &[PathBuf],
    enum_name_override: Option<&str>,
    manifest_dir: Option<&Path>,
    crate_path: Option<&str>,
    extra_derives: &[String],
) -> PipelineResult {
    if configs.is_empty() {
        return PipelineResult {
            code: String::new(),
            errors: vec![BuildError::Io {
                message: "no icon configs provided".into(),
            }],
            warnings: Vec::new(),
            rerun_paths: Vec::new(),
            size_report: None,
            output_filename: String::new(),
        };
    }

    assert_eq!(configs.len(), base_dirs.len());

    let mut errors: Vec<BuildError> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();
    let mut rerun_paths: Vec<PathBuf> = Vec::new();
    let mut all_mappings: BTreeMap<String, ThemeMapping> = BTreeMap::new();
    let mut svg_paths: Vec<PathBuf> = Vec::new();

    // Determine output filename from first config or override
    let first_name = enum_name_override
        .map(|s| s.to_string())
        .unwrap_or_else(|| configs[0].1.name.clone());
    let output_filename = format!("{}.rs", first_name.to_snake_case());

    // Step 0: Validate each config in isolation
    for (file_path, config) in configs {
        // Warn about empty roles (likely misconfiguration)
        if config.roles.is_empty() {
            warnings.push(format!(
                "{file_path}: roles list is empty; generated enum will have no variants"
            ));
        }

        // Check for duplicate roles within a single file
        let dup_in_file_errors = validate::validate_no_duplicate_roles_in_file(config, file_path);
        errors.extend(dup_in_file_errors);

        // Check for theme overlap (same theme in bundled and system)
        let overlap_errors = validate::validate_theme_overlap(config);
        errors.extend(overlap_errors);

        // Check for duplicate theme names within the same list
        let dup_theme_errors = validate::validate_no_duplicate_themes(config);
        errors.extend(dup_theme_errors);
    }

    // Step 1: Check for duplicate roles across all files
    if configs.len() > 1 {
        let dup_errors = validate::validate_no_duplicate_roles(configs);
        errors.extend(dup_errors);
    }

    // Step 2: Merge configs first so validation uses the merged role list
    let merged = merge_configs(configs, enum_name_override);

    // Warn about empty themes (likely misconfiguration)
    if merged.bundled_themes.is_empty() && merged.system_themes.is_empty() {
        warnings.push(
            "no bundled-themes or system-themes configured; \
             generated IconProvider will always return None"
                .to_string(),
        );
    }

    // Step 2b: Validate identifiers (enum name + role names)
    let id_errors = validate::validate_identifiers(&merged);
    errors.extend(id_errors);

    // Track rerun paths for all master TOML files
    for (file_path, _config) in configs {
        rerun_paths.push(PathBuf::from(file_path));
    }

    // Validate theme names on the merged config
    let theme_errors = validate::validate_themes(&merged);
    errors.extend(theme_errors);

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
                    errors.extend(map_errors);

                    // Validate icon name values are well-formed
                    let name_errors =
                        validate::validate_mapping_values(&mapping, &mapping_path_str);
                    errors.extend(name_errors);

                    // Validate SVGs exist (only for declared master roles)
                    let svg_errors = validate::validate_svgs(&mapping, &theme_dir, &merged.roles);
                    errors.extend(svg_errors);

                    // Warn about unrecognized DE keys in DeAware values
                    let de_warnings = validate::validate_de_keys(&mapping, &mapping_path_str);
                    warnings.extend(de_warnings);

                    // Issue 7: Warn when bundled themes have DE-aware mappings
                    // (only the default SVG can be embedded).
                    for (role_name, value) in &mapping {
                        if matches!(value, schema::MappingValue::DeAware(_)) {
                            warnings.push(format!(
                                "bundled theme \"{}\" has DE-aware mapping for \"{}\": \
                                 only the default SVG will be embedded",
                                theme_name, role_name
                            ));
                        }
                    }

                    // Check orphan SVGs (warnings, not errors)
                    let orphan_warnings = check_orphan_svgs_and_collect_paths(
                        &mapping,
                        &theme_dir,
                        theme_name,
                        &mut svg_paths,
                        &mut rerun_paths,
                    );
                    warnings.extend(orphan_warnings);

                    all_mappings.insert(theme_name.clone(), mapping);
                }
                Err(e) => {
                    errors.push(BuildError::Io {
                        message: format!("failed to parse {mapping_path_str}: {e}"),
                    });
                }
            },
            Err(e) => {
                errors.push(BuildError::Io {
                    message: format!("failed to read {mapping_path_str}: {e}"),
                });
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
                    errors.extend(map_errors);

                    // Validate icon name values are well-formed
                    let name_errors =
                        validate::validate_mapping_values(&mapping, &mapping_path_str);
                    errors.extend(name_errors);

                    // Warn about unrecognized DE keys in DeAware values
                    let de_warnings = validate::validate_de_keys(&mapping, &mapping_path_str);
                    warnings.extend(de_warnings);

                    all_mappings.insert(theme_name.clone(), mapping);
                }
                Err(e) => {
                    errors.push(BuildError::Io {
                        message: format!("failed to parse {mapping_path_str}: {e}"),
                    });
                }
            },
            Err(e) => {
                errors.push(BuildError::Io {
                    message: format!("failed to read {mapping_path_str}: {e}"),
                });
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
    // Normalize to forward slashes so generated include_bytes! paths are valid
    // on all platforms (backslashes in Rust string literals are escape sequences).
    let base_dir_str = if let Some(mdir) = manifest_dir {
        base_dir
            .strip_prefix(mdir)
            .unwrap_or(base_dir)
            .to_string_lossy()
            .replace('\\', "/")
    } else {
        base_dir.to_string_lossy().replace('\\', "/")
    };

    // Step 4: Generate code
    let effective_crate_path = crate_path.unwrap_or("native_theme");
    let code = codegen::generate_code(
        &merged,
        &all_mappings,
        &base_dir_str,
        effective_crate_path,
        extra_derives,
    );

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
fn merge_configs(
    configs: &[(String, MasterConfig)],
    enum_name_override: Option<&str>,
) -> MasterConfig {
    let name = enum_name_override
        .map(|s| s.to_string())
        .unwrap_or_else(|| configs[0].1.name.clone());

    let mut roles = Vec::new();
    let mut bundled_themes = Vec::new();
    let mut system_themes = Vec::new();
    let mut seen_roles = std::collections::BTreeSet::new();
    let mut seen_bundled = std::collections::BTreeSet::new();
    let mut seen_system = std::collections::BTreeSet::new();

    for (_path, config) in configs {
        for role in &config.roles {
            if seen_roles.insert(role.clone()) {
                roles.push(role.clone());
            }
        }

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

/// Convert a `PipelineResult` into `Result<GenerateOutput, BuildErrors>`.
fn pipeline_result_to_output(
    result: PipelineResult,
    out_dir: &Path,
) -> Result<GenerateOutput, BuildErrors> {
    if !result.errors.is_empty() {
        // Emit rerun-if-changed even on error so cargo re-checks when the user
        // fixes the files.
        for path in &result.rerun_paths {
            println!("cargo::rerun-if-changed={}", path.display());
        }
        return Err(BuildErrors::new(result.errors));
    }

    let output_path = out_dir.join(&result.output_filename);

    let (role_count, bundled_theme_count, svg_count, total_svg_bytes) = match &result.size_report {
        Some(report) => (
            report.role_count,
            report.bundled_theme_count,
            report.svg_count,
            report.total_svg_bytes,
        ),
        None => (0, 0, 0, 0),
    };

    Ok(GenerateOutput {
        output_path,
        warnings: result.warnings,
        role_count,
        bundled_theme_count,
        svg_count,
        total_svg_bytes,
        rerun_paths: result.rerun_paths,
        code: result.code,
    })
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
        let toml_str = r#"play-pause = { kde = "media-playback-start", default = "play" }"#;
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
        assert!(msg.contains("nonexistent"), "should contain theme name");
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

    // === THEME_TABLE tests ===

    #[test]
    fn theme_table_has_all_five() {
        assert_eq!(THEME_TABLE.len(), 5);
        let names: Vec<&str> = THEME_TABLE.iter().map(|(k, _)| *k).collect();
        assert!(names.contains(&"sf-symbols"));
        assert!(names.contains(&"segoe-fluent"));
        assert!(names.contains(&"freedesktop"));
        assert!(names.contains(&"material"));
        assert!(names.contains(&"lucide"));
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
        write_fixture(
            &dir,
            "material/mapping.toml",
            r#"
play-pause = "play_pause"
skip-forward = "skip_next"
"#,
        );
        write_fixture(
            &dir,
            "sf-symbols/mapping.toml",
            r#"
play-pause = "play.fill"
skip-forward = "forward.fill"
"#,
        );
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);
        write_fixture(&dir, "material/skip_next.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(
            r#"
name = "sample-icon"
roles = ["play-pause", "skip-forward"]
bundled-themes = ["material"]
system-themes = ["sf-symbols"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[("sample-icons.toml".to_string(), config)],
            std::slice::from_ref(&dir),
            None,
            None,
            None,
            &[],
        );

        assert!(
            result.errors.is_empty(),
            "expected no errors: {:?}",
            result.errors
        );
        assert!(!result.code.is_empty(), "expected generated code");
        assert!(result.code.contains("pub enum SampleIcon"));
        assert!(result.code.contains("PlayPause"));
        assert!(result.code.contains("SkipForward"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_output_filename_uses_snake_case() {
        let dir = create_fixture_dir("filename");
        write_fixture(
            &dir,
            "material/mapping.toml",
            "play-pause = \"play_pause\"\n",
        );
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(
            r#"
name = "app-icon"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[("app.toml".to_string(), config)],
            std::slice::from_ref(&dir),
            None,
            None,
            None,
            &[],
        );

        assert_eq!(result.output_filename, "app_icon.rs");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_collects_rerun_paths() {
        let dir = create_fixture_dir("rerun");
        write_fixture(
            &dir,
            "material/mapping.toml",
            r#"
play-pause = "play_pause"
"#,
        );
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(
            r#"
name = "test"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            std::slice::from_ref(&dir),
            None,
            None,
            None,
            &[],
        );

        assert!(result.errors.is_empty());
        // Should include: master TOML, mapping TOML, theme dir, SVG files
        let path_strs: Vec<String> = result
            .rerun_paths
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();
        assert!(
            path_strs.iter().any(|p| p.contains("test.toml")),
            "should track master TOML"
        );
        assert!(
            path_strs.iter().any(|p| p.contains("mapping.toml")),
            "should track mapping TOML"
        );
        assert!(
            path_strs.iter().any(|p| p.contains("play_pause.svg")),
            "should track SVG files"
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_emits_size_report() {
        let dir = create_fixture_dir("size");
        write_fixture(
            &dir,
            "material/mapping.toml",
            "play-pause = \"play_pause\"\n",
        );
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(
            r#"
name = "test"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            std::slice::from_ref(&dir),
            None,
            None,
            None,
            &[],
        );

        assert!(result.errors.is_empty());
        let report = result
            .size_report
            .as_ref()
            .expect("should have size report");
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
        write_fixture(
            &dir,
            "material/mapping.toml",
            "play-pause = \"play_pause\"\n",
        );
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(
            r#"
name = "test"
roles = ["play-pause", "skip-forward"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            std::slice::from_ref(&dir),
            None,
            None,
            None,
            &[],
        );

        assert!(!result.errors.is_empty(), "should have errors");
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.to_string().contains("skip-forward")),
            "should mention missing role"
        );
        assert!(result.code.is_empty(), "no code on errors");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_returns_errors_on_missing_svg() {
        let dir = create_fixture_dir("missing_svg");
        write_fixture(
            &dir,
            "material/mapping.toml",
            r#"
play-pause = "play_pause"
skip-forward = "skip_next"
"#,
        );
        // Only create one SVG, leave skip_next.svg missing
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(
            r#"
name = "test"
roles = ["play-pause", "skip-forward"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            std::slice::from_ref(&dir),
            None,
            None,
            None,
            &[],
        );

        assert!(!result.errors.is_empty(), "should have errors");
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.to_string().contains("skip_next.svg")),
            "should mention missing SVG"
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_orphan_svgs_are_warnings() {
        let dir = create_fixture_dir("orphan_warn");
        write_fixture(
            &dir,
            "material/mapping.toml",
            "play-pause = \"play_pause\"\n",
        );
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);
        write_fixture(&dir, "material/unused.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(
            r#"
name = "test"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            std::slice::from_ref(&dir),
            None,
            None,
            None,
            &[],
        );

        assert!(result.errors.is_empty(), "orphans are not errors");
        assert!(!result.warnings.is_empty(), "should have orphan warning");
        assert!(result.warnings.iter().any(|w| w.contains("unused.svg")));

        let _ = fs::remove_dir_all(&dir);
    }

    // === merge_configs tests ===

    #[test]
    fn merge_configs_combines_roles() {
        let config_a: MasterConfig = toml::from_str(
            r#"
name = "a"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();
        let config_b: MasterConfig = toml::from_str(
            r#"
name = "b"
roles = ["skip-forward"]
bundled-themes = ["material"]
system-themes = ["sf-symbols"]
"#,
        )
        .unwrap();

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
        let config: MasterConfig = toml::from_str(
            r#"
name = "original"
roles = ["x"]
"#,
        )
        .unwrap();

        let configs = vec![("a.toml".to_string(), config)];
        let merged = merge_configs(&configs, Some("MyIcons"));

        assert_eq!(merged.name, "MyIcons");
    }

    // === Builder pipeline tests ===

    #[test]
    fn pipeline_builder_merges_two_files() {
        let dir = create_fixture_dir("builder_merge");
        write_fixture(
            &dir,
            "material/mapping.toml",
            r#"
play-pause = "play_pause"
skip-forward = "skip_next"
"#,
        );
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);
        write_fixture(&dir, "material/skip_next.svg", SVG_STUB);

        let config_a: MasterConfig = toml::from_str(
            r#"
name = "icons-a"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();
        let config_b: MasterConfig = toml::from_str(
            r#"
name = "icons-b"
roles = ["skip-forward"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[
                ("a.toml".to_string(), config_a),
                ("b.toml".to_string(), config_b),
            ],
            &[dir.clone(), dir.clone()],
            Some("AllIcons"),
            None,
            None,
            &[],
        );

        assert!(
            result.errors.is_empty(),
            "expected no errors: {:?}",
            result.errors
        );
        assert!(
            result.code.contains("pub enum AllIcons"),
            "should use override name"
        );
        assert!(result.code.contains("PlayPause"));
        assert!(result.code.contains("SkipForward"));
        assert_eq!(result.output_filename, "all_icons.rs");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_builder_detects_duplicate_roles() {
        let dir = create_fixture_dir("builder_dup");
        write_fixture(
            &dir,
            "material/mapping.toml",
            "play-pause = \"play_pause\"\n",
        );
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        let config_a: MasterConfig = toml::from_str(
            r#"
name = "a"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();
        let config_b: MasterConfig = toml::from_str(
            r#"
name = "b"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[
                ("a.toml".to_string(), config_a),
                ("b.toml".to_string(), config_b),
            ],
            &[dir.clone(), dir.clone()],
            None,
            None,
            None,
            &[],
        );

        assert!(!result.errors.is_empty(), "should detect duplicate roles");
        assert!(
            result
                .errors
                .iter()
                .any(|e| e.to_string().contains("play-pause"))
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_generates_relative_include_bytes_paths() {
        // Simulate what generate_icons does: manifest_dir + "icons/icons.toml"
        // The tmpdir acts as CARGO_MANIFEST_DIR.
        // base_dir is absolute (tmpdir/icons), but run_pipeline should strip
        // the manifest_dir prefix for codegen, producing relative paths.
        let tmpdir = create_fixture_dir("rel_paths");
        write_fixture(
            &tmpdir,
            "icons/material/mapping.toml",
            "play-pause = \"play_pause\"\n",
        );
        write_fixture(&tmpdir, "icons/material/play_pause.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(
            r#"
name = "test"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();

        // base_dir is absolute (as generate_icons would compute)
        let abs_base_dir = tmpdir.join("icons");

        let result = run_pipeline(
            &[("icons/icons.toml".to_string(), config)],
            &[abs_base_dir],
            None,
            Some(&tmpdir), // manifest_dir for stripping prefix
            None,
            &[],
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
        write_fixture(
            &dir,
            "sf-symbols/mapping.toml",
            r#"
play-pause = "play.fill"
"#,
        );

        let config: MasterConfig = toml::from_str(
            r#"
name = "test"
roles = ["play-pause"]
system-themes = ["sf-symbols"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            std::slice::from_ref(&dir),
            None,
            None,
            None,
            &[],
        );

        assert!(
            result.errors.is_empty(),
            "system themes should not require SVGs: {:?}",
            result.errors
        );

        let _ = fs::remove_dir_all(&dir);
    }

    // === BuildErrors tests ===

    #[test]
    fn build_errors_display_format() {
        let errors = BuildErrors::new(vec![
            BuildError::MissingRole {
                role: "play-pause".into(),
                mapping_file: "mapping.toml".into(),
            },
            BuildError::MissingSvg {
                path: "play.svg".into(),
            },
        ]);
        let msg = errors.to_string();
        assert!(msg.contains("2 build error(s):"));
        assert!(msg.contains("play-pause"));
        assert!(msg.contains("play.svg"));
    }

    // === New BuildError Display tests ===

    #[test]
    fn build_error_invalid_identifier_format() {
        let err = BuildError::InvalidIdentifier {
            name: "---".into(),
            reason: "PascalCase conversion produces an empty string".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("---"), "should contain the name");
        assert!(msg.contains("empty"), "should contain the reason");
    }

    #[test]
    fn build_error_identifier_collision_format() {
        let err = BuildError::IdentifierCollision {
            role_a: "play_pause".into(),
            role_b: "play-pause".into(),
            pascal: "PlayPause".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("play_pause"), "should mention first role");
        assert!(msg.contains("play-pause"), "should mention second role");
        assert!(msg.contains("PlayPause"), "should mention PascalCase");
    }

    #[test]
    fn build_error_theme_overlap_format() {
        let err = BuildError::ThemeOverlap {
            theme: "material".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("material"), "should mention theme");
        assert!(msg.contains("bundled"), "should mention bundled");
        assert!(msg.contains("system"), "should mention system");
    }

    #[test]
    fn build_error_duplicate_role_in_file_format() {
        let err = BuildError::DuplicateRoleInFile {
            role: "play-pause".into(),
            file: "icons.toml".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("play-pause"), "should mention role");
        assert!(msg.contains("icons.toml"), "should mention file");
    }

    // === Pipeline validation integration tests ===

    #[test]
    fn pipeline_detects_theme_overlap() {
        let dir = create_fixture_dir("theme_overlap");
        write_fixture(
            &dir,
            "material/mapping.toml",
            "play-pause = \"play_pause\"\n",
        );
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(
            r#"
name = "test"
roles = ["play-pause"]
bundled-themes = ["material"]
system-themes = ["material"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            std::slice::from_ref(&dir),
            None,
            None,
            None,
            &[],
        );

        assert!(!result.errors.is_empty(), "should detect theme overlap");
        assert!(
            result.errors.iter().any(|e| matches!(
                e,
                BuildError::ThemeOverlap { theme } if theme == "material"
            )),
            "should have ThemeOverlap error for 'material': {:?}",
            result.errors
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_detects_identifier_collision() {
        let dir = create_fixture_dir("id_collision");
        write_fixture(
            &dir,
            "material/mapping.toml",
            "play_pause = \"pp\"\nplay-pause = \"pp2\"\n",
        );
        write_fixture(&dir, "material/pp.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(
            r#"
name = "test"
roles = ["play_pause", "play-pause"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            std::slice::from_ref(&dir),
            None,
            None,
            None,
            &[],
        );

        assert!(
            result.errors.iter().any(|e| matches!(
                e,
                BuildError::IdentifierCollision { pascal, .. } if pascal == "PlayPause"
            )),
            "should detect PascalCase collision: {:?}",
            result.errors
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_detects_invalid_identifier() {
        let dir = create_fixture_dir("id_invalid");
        write_fixture(&dir, "material/mapping.toml", "self = \"self_icon\"\n");
        write_fixture(&dir, "material/self_icon.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(
            r#"
name = "test"
roles = ["self"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            std::slice::from_ref(&dir),
            None,
            None,
            None,
            &[],
        );

        assert!(
            result.errors.iter().any(|e| matches!(
                e,
                BuildError::InvalidIdentifier { name, .. } if name == "self"
            )),
            "should detect keyword identifier: {:?}",
            result.errors
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_detects_duplicate_role_in_file() {
        let dir = create_fixture_dir("dup_in_file");
        write_fixture(
            &dir,
            "material/mapping.toml",
            "play-pause = \"play_pause\"\n",
        );
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        // MasterConfig with duplicate role -- manually construct since TOML
        // arrays allow duplicates
        let config = MasterConfig {
            name: "test".to_string(),
            roles: vec!["play-pause".to_string(), "play-pause".to_string()],
            bundled_themes: vec!["material".to_string()],
            system_themes: Vec::new(),
        };

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            std::slice::from_ref(&dir),
            None,
            None,
            None,
            &[],
        );

        assert!(
            result.errors.iter().any(|e| matches!(
                e,
                BuildError::DuplicateRoleInFile { role, file }
                    if role == "play-pause" && file == "test.toml"
            )),
            "should detect duplicate role in file: {:?}",
            result.errors
        );

        let _ = fs::remove_dir_all(&dir);
    }

    // === Issue 7: Bundled DE-aware warning tests ===

    #[test]
    fn pipeline_bundled_de_aware_produces_warning() {
        let dir = create_fixture_dir("bundled_de_aware");
        // Bundled theme with a DE-aware mapping
        write_fixture(
            &dir,
            "material/mapping.toml",
            r#"play-pause = { kde = "media-playback-start", default = "play_pause" }"#,
        );
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(
            r#"
name = "test-icon"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            std::slice::from_ref(&dir),
            None,
            None,
            None,
            &[],
        );

        assert!(
            result.errors.is_empty(),
            "bundled DE-aware should not be an error: {:?}",
            result.errors
        );
        assert!(
            result.warnings.iter().any(|w| {
                w.contains("bundled theme \"material\"")
                    && w.contains("play-pause")
                    && w.contains("only the default SVG will be embedded")
            }),
            "should warn about bundled DE-aware mapping. warnings: {:?}",
            result.warnings
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_system_de_aware_no_bundled_warning() {
        let dir = create_fixture_dir("system_de_aware");
        // System theme with DE-aware mapping should NOT produce the bundled warning
        write_fixture(
            &dir,
            "freedesktop/mapping.toml",
            r#"play-pause = { kde = "media-playback-start", default = "play" }"#,
        );

        let config: MasterConfig = toml::from_str(
            r#"
name = "test-icon"
roles = ["play-pause"]
system-themes = ["freedesktop"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            std::slice::from_ref(&dir),
            None,
            None,
            None,
            &[],
        );

        assert!(
            result.errors.is_empty(),
            "system DE-aware should not be an error: {:?}",
            result.errors
        );
        assert!(
            !result
                .warnings
                .iter()
                .any(|w| w.contains("only the default SVG will be embedded")),
            "system themes should NOT produce bundled DE-aware warning. warnings: {:?}",
            result.warnings
        );

        let _ = fs::remove_dir_all(&dir);
    }

    // === Issue 14: crate_path tests ===

    #[test]
    fn pipeline_custom_crate_path() {
        let dir = create_fixture_dir("crate_path");
        write_fixture(
            &dir,
            "material/mapping.toml",
            "play-pause = \"play_pause\"\n",
        );
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(
            r#"
name = "test-icon"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            std::slice::from_ref(&dir),
            None,
            None,
            Some("my_crate::native_theme"),
            &[],
        );

        assert!(
            result.errors.is_empty(),
            "custom crate path should not cause errors: {:?}",
            result.errors
        );
        assert!(
            result
                .code
                .contains("impl my_crate::native_theme::IconProvider"),
            "should use custom crate path in impl. code:\n{}",
            result.code
        );
        assert!(
            !result.code.contains("extern crate"),
            "custom crate path should not emit extern crate. code:\n{}",
            result.code
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn pipeline_default_crate_path_emits_extern_crate() {
        let dir = create_fixture_dir("default_crate_path");
        write_fixture(
            &dir,
            "material/mapping.toml",
            "play-pause = \"play_pause\"\n",
        );
        write_fixture(&dir, "material/play_pause.svg", SVG_STUB);

        let config: MasterConfig = toml::from_str(
            r#"
name = "test-icon"
roles = ["play-pause"]
bundled-themes = ["material"]
"#,
        )
        .unwrap();

        let result = run_pipeline(
            &[("test.toml".to_string(), config)],
            std::slice::from_ref(&dir),
            None,
            None,
            None,
            &[],
        );

        assert!(
            result.errors.is_empty(),
            "default crate path should not cause errors: {:?}",
            result.errors
        );
        assert!(
            result.code.contains("extern crate native_theme;"),
            "default crate path should emit extern crate. code:\n{}",
            result.code
        );

        let _ = fs::remove_dir_all(&dir);
    }

    // === Builder input validation tests ===

    #[test]
    #[should_panic(expected = "derive name must be non-empty")]
    fn derive_rejects_empty_string() {
        let _ = IconGenerator::new().derive("");
    }

    #[test]
    #[should_panic(expected = "derive name must be non-empty and contain no whitespace")]
    fn derive_rejects_whitespace() {
        let _ = IconGenerator::new().derive("Ord PartialOrd");
    }

    #[test]
    #[should_panic(expected = "derive name must be non-empty and contain no whitespace")]
    fn derive_rejects_tab() {
        let _ = IconGenerator::new().derive("Ord\t");
    }

    #[test]
    fn derive_accepts_valid_name() {
        // Should not panic
        let _ = IconGenerator::new().derive("Ord");
        let _ = IconGenerator::new().derive("serde::Serialize");
    }

    #[test]
    #[should_panic(expected = "crate_path must be non-empty")]
    fn crate_path_rejects_empty_string() {
        let _ = IconGenerator::new().crate_path("");
    }

    #[test]
    #[should_panic(expected = "crate_path must be non-empty and contain no spaces")]
    fn crate_path_rejects_spaces() {
        let _ = IconGenerator::new().crate_path("foo bar");
    }

    #[test]
    fn crate_path_accepts_valid_path() {
        // Should not panic
        let _ = IconGenerator::new().crate_path("native_theme");
        let _ = IconGenerator::new().crate_path("my_crate::native_theme");
    }
}
