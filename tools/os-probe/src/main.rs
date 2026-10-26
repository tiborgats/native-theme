//! Dev-only OS probe for the platform-fidelity review (code review M2a, M2b).
//!
//! Prints what macOS or Windows reports — fonts, colours, control sizes,
//! system metrics — and writes it to `<out>/probe.toml`; on macOS it also
//! renders each control under `<out>/renders/`. Never published; run by the
//! dispatch-only `.github/workflows/os-probe.yml` on each runner image.
//!
//! Every query that fails is recorded as a string value (`unavailable: …`
//! or `not probed: …`) and the run goes on. On any other OS the probe prints
//! that it supports macOS and Windows only and exits with success.
//!
//! Usage: `native-theme-os-probe [--out <dir>]` (default `os-probe-out`).

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

#[cfg(any(target_os = "macos", target_os = "windows"))]
mod out;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "usage: native-theme-os-probe [--out <dir>]   (default: os-probe-out)";

/// The output directory from the command line, or the usage error.
fn parse_args() -> Result<PathBuf, String> {
    parse_args_from(std::env::args_os().skip(1))
}

/// The output directory from `args`, the command line after the program's
/// name, or the usage error. An argument starting with `-psn_`, the process
/// serial number LaunchServices may pass an application `open` starts, is
/// ignored.
fn parse_args_from(args: impl IntoIterator<Item = std::ffi::OsString>) -> Result<PathBuf, String> {
    let mut out = PathBuf::from("os-probe-out");
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some(s) if s.starts_with("-psn_") => {}
            Some("--out") => match args.next() {
                Some(dir) => out = PathBuf::from(dir),
                None => return Err(format!("--out needs a directory\n{USAGE}")),
            },
            Some(s) if s.starts_with("--out=") => {
                out = PathBuf::from(s.trim_start_matches("--out="));
            }
            Some("-h" | "--help") => return Err(USAGE.to_owned()),
            _ => {
                return Err(format!(
                    "unknown argument {}\n{USAGE}",
                    arg.to_string_lossy()
                ));
            }
        }
    }
    Ok(out)
}

fn main() -> ExitCode {
    let out_dir = match parse_args() {
        Ok(dir) => dir,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(2);
        }
    };
    run(&out_dir)
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn run(_out_dir: &std::path::Path) -> ExitCode {
    println!(
        "native-theme-os-probe supports macOS and Windows only; nothing to probe on {}.",
        std::env::consts::OS
    );
    ExitCode::SUCCESS
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn run(out_dir: &std::path::Path) -> ExitCode {
    if let Err(e) = std::fs::create_dir_all(out_dir) {
        eprintln!("cannot create {}: {e}", out_dir.display());
        return ExitCode::FAILURE;
    }

    let mut doc = toml::Table::new();
    #[cfg(target_os = "macos")]
    macos::probe(&mut doc, out_dir);
    #[cfg(target_os = "windows")]
    windows::probe(&mut doc);

    let text = match toml::to_string(&doc) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("cannot serialise the probe's results: {e}");
            return ExitCode::FAILURE;
        }
    };
    println!("{text}");
    let path = out_dir.join("probe.toml");
    if let Err(e) = std::fs::write(&path, &text) {
        eprintln!("cannot write {}: {e}", path.display());
        return ExitCode::FAILURE;
    }
    eprintln!("wrote {}", path.display());
    ExitCode::SUCCESS
}

/// The `[probe]` table: what ran, where.
#[cfg(any(target_os = "macos", target_os = "windows"))]
fn probe_table() -> toml::Table {
    let mut t = toml::Table::new();
    out::put(&mut t, "tool", out::text(env!("CARGO_PKG_NAME")));
    out::put(&mut t, "arch", out::text(std::env::consts::ARCH));
    out::put(
        &mut t,
        "runner_image",
        std::env::var("ImageOS").map_or_else(|_| out::unavailable("ImageOS not set"), out::text),
    );
    out::put(
        &mut t,
        "runner_image_version",
        std::env::var("ImageVersion")
            .map_or_else(|_| out::unavailable("ImageVersion not set"), out::text),
    );
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Result<PathBuf, String> {
        parse_args_from(args.iter().map(std::ffi::OsString::from))
    }

    #[test]
    fn the_output_directory_is_taken_from_out() {
        assert_eq!(parse(&[]), Ok(PathBuf::from("os-probe-out")));
        assert_eq!(parse(&["--out", "/tmp/x"]), Ok(PathBuf::from("/tmp/x")));
        assert_eq!(parse(&["--out=/tmp/y"]), Ok(PathBuf::from("/tmp/y")));
        assert!(parse(&["--out"]).is_err());
        assert!(parse(&["--other"]).is_err());
    }

    /// LaunchServices may add a process serial number to an application
    /// `open` starts; the probe ignores it wherever it comes.
    #[test]
    fn a_process_serial_number_argument_is_ignored() {
        assert_eq!(
            parse(&["-psn_0_12345", "--out", "/tmp/app"]),
            Ok(PathBuf::from("/tmp/app"))
        );
        assert_eq!(
            parse(&["--out", "/tmp/app", "-psn_0_12345"]),
            Ok(PathBuf::from("/tmp/app"))
        );
        assert_eq!(parse(&["-psn_0_1"]), Ok(PathBuf::from("os-probe-out")));
    }
}
