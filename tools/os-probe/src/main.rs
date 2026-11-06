//! Dev-only OS probe for the platform-fidelity review (M2a and M2b of the
//! v0.7.0 code review, `docs/archive/todo_v0.7.0_code-review-plan.md`).
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
//! Usage: `native-theme-os-probe [--out <dir>] [--not-key] [--button-pass
//! [--focus | --focus-transition | --not-key | --inactive | --key-not-main |
//! --main-not-key]] [--symbols | --reads]`. `--out` (or `--out=<dir>`) names the
//! output directory, `os-probe-out` by default; `--not-key`, macOS only,
//! makes the key-window section capture its window after a second window has
//! taken the key; `--button-pass`, macOS only, runs the button pass alone:
//! buttons in titled windows, captured per state, control size and
//! appearance over the window's background and three opaque backdrops, with
//! the window key, or with each button focused (`--focus`), a second window
//! key (`--not-key`) or another application active (`--inactive`); with
//! `--focus-transition` it records the screen instead while two buttons are
//! focused and unfocused. `--symbols`, macOS only, runs the symbols pass
//! alone: the SF Symbol configuration and image size each place gives a
//! symbol image; `--reads`, macOS only, the reads pass alone: the media
//! timing functions, the route button's colours and renders, and the ring
//! round a plain focusable view.

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

const USAGE: &str = "usage: native-theme-os-probe [--out <dir>] [--not-key] [--button-pass [--focus | --focus-transition | --not-key | --inactive | --key-not-main | --main-not-key]] [--symbols | --reads]   (default: os-probe-out; --not-key, macOS only: the key-window section, or the button pass, captures its windows after another window took the key; --button-pass, macOS only: the button pass alone; --focus, --focus-transition and --inactive, with --button-pass only: each button focused in turn, the screen recorded while two buttons are focused and unfocused, or another application active; --key-not-main and --main-not-key, with --button-pass only: each sheet a panel that is key while another window is main, or each sheet's window main while a panel is key; --symbols, macOS only: the symbols pass alone; --reads, macOS only: the reads pass alone)";

/// What the command line asks for.
#[derive(Debug, PartialEq, Eq)]
struct Options {
    /// The directory `probe.toml` and the renders are written to.
    out: PathBuf,
    /// Whether the macOS key-window section captures its window after a
    /// second window has taken the key.
    not_key: bool,
    /// Whether the macOS probe runs the button pass alone.
    button_pass: bool,
    /// Whether the button pass focuses each button in turn.
    focus: bool,
    /// Whether the button pass captures with another application active.
    inactive: bool,
    /// Whether the button pass records the focus ring's transition instead.
    focus_transition: bool,
    /// Whether the button pass's sheets are key but not main.
    key_not_main: bool,
    /// Whether the button pass's sheets are main but not key.
    main_not_key: bool,
    /// Whether the macOS probe runs the symbols pass alone.
    symbols: bool,
    /// Whether the macOS probe runs the reads pass alone.
    reads: bool,
}

/// The options from the command line, or the usage error.
fn parse_args() -> Result<Options, String> {
    parse_args_from(std::env::args_os().skip(1))
}

/// The options from `args`, the command line after the program's name, in
/// any order, or the usage error. An argument starting with `-psn_`, the
/// process serial number LaunchServices may pass an application `open`
/// starts, is ignored, and so are `-AppleLanguages`, `-AppleLocale`,
/// `-AppleTextDirection` and `-NSForceRightToLeftWritingDirection`, each with
/// its value: defaults AppKit reads from the arguments.
fn parse_args_from(args: impl IntoIterator<Item = std::ffi::OsString>) -> Result<Options, String> {
    let mut out = PathBuf::from("os-probe-out");
    let mut not_key = false;
    let mut button_pass = false;
    let mut focus = false;
    let mut inactive = false;
    let mut focus_transition = false;
    let mut key_not_main = false;
    let mut main_not_key = false;
    let mut symbols = false;
    let mut reads = false;
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some(s) if s.starts_with("-psn_") => {}
            // An argument-domain default (NSUserDefaults.h's
            // NSArgumentDomain): AppKit reads it from the process's
            // arguments itself, so the probe takes the pair and ignores it.
            Some(
                "-AppleLanguages"
                | "-AppleLocale"
                | "-AppleTextDirection"
                | "-NSForceRightToLeftWritingDirection",
            ) => {
                if args.next().is_none() {
                    return Err(format!("{} needs a value\n{USAGE}", arg.to_string_lossy()));
                }
            }
            Some("--not-key") => not_key = true,
            Some("--button-pass") => button_pass = true,
            Some("--focus") => focus = true,
            Some("--inactive") => inactive = true,
            Some("--focus-transition") => focus_transition = true,
            Some("--key-not-main") => key_not_main = true,
            Some("--main-not-key") => main_not_key = true,
            Some("--symbols") => symbols = true,
            Some("--reads") => reads = true,
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
    if (focus || inactive || focus_transition || key_not_main || main_not_key) && !button_pass {
        return Err(format!(
            "--focus, --focus-transition, --inactive, --key-not-main and --main-not-key need --button-pass\n{USAGE}"
        ));
    }
    if [
        focus,
        inactive,
        not_key,
        focus_transition,
        key_not_main,
        main_not_key,
    ]
    .iter()
    .filter(|set| **set)
    .count()
        > 1
        && button_pass
    {
        return Err(format!(
            "the button pass takes one of --focus, --focus-transition, --not-key, --inactive, --key-not-main and --main-not-key\n{USAGE}"
        ));
    }
    if (symbols || reads) && (button_pass || not_key || (symbols && reads)) {
        return Err(format!(
            "--symbols and --reads each run alone, without --button-pass, --not-key or each other\n{USAGE}"
        ));
    }
    Ok(Options {
        out,
        not_key,
        button_pass,
        focus,
        inactive,
        focus_transition,
        key_not_main,
        main_not_key,
        symbols,
        reads,
    })
}

fn main() -> ExitCode {
    let options = match parse_args() {
        Ok(options) => options,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(2);
        }
    };
    run(&options)
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn run(_options: &Options) -> ExitCode {
    println!(
        "native-theme-os-probe supports macOS and Windows only; nothing to probe on {}.",
        std::env::consts::OS
    );
    ExitCode::SUCCESS
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn run(options: &Options) -> ExitCode {
    let out_dir = options.out.as_path();
    if let Err(e) = std::fs::create_dir_all(out_dir) {
        eprintln!("cannot create {}: {e}", out_dir.display());
        return ExitCode::FAILURE;
    }

    let mut doc = toml::Table::new();
    #[cfg(target_os = "macos")]
    if options.symbols {
        macos::symbols_probe(&mut doc, out_dir);
    } else if options.reads {
        macos::reads_probe(&mut doc, out_dir);
    } else if options.button_pass {
        let mode = if options.focus {
            macos::ButtonMode::Focus
        } else if options.focus_transition {
            macos::ButtonMode::FocusTransition
        } else if options.inactive {
            macos::ButtonMode::Inactive
        } else if options.key_not_main {
            macos::ButtonMode::KeyNotMain
        } else if options.main_not_key {
            macos::ButtonMode::MainNotKey
        } else if options.not_key {
            macos::ButtonMode::NotKey
        } else {
            macos::ButtonMode::Key
        };
        macos::button_probe(&mut doc, out_dir, mode);
    } else {
        macos::probe(&mut doc, out_dir, options.not_key);
    }
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

    fn parse(args: &[&str]) -> Result<Options, String> {
        parse_args_from(args.iter().map(std::ffi::OsString::from))
    }

    fn options(out: &str, not_key: bool) -> Result<Options, String> {
        Ok(Options {
            out: PathBuf::from(out),
            not_key,
            button_pass: false,
            focus: false,
            inactive: false,
            focus_transition: false,
            key_not_main: false,
            main_not_key: false,
            symbols: false,
            reads: false,
        })
    }

    #[test]
    fn the_output_directory_is_taken_from_out() {
        assert_eq!(parse(&[]), options("os-probe-out", false));
        assert_eq!(parse(&["--out", "/tmp/x"]), options("/tmp/x", false));
        assert_eq!(parse(&["--out=/tmp/y"]), options("/tmp/y", false));
        assert!(parse(&["--out"]).is_err());
        assert!(parse(&["--other"]).is_err());
    }

    /// LaunchServices may add a process serial number to an application
    /// `open` starts; the probe ignores it wherever it comes.
    #[test]
    fn a_process_serial_number_argument_is_ignored() {
        assert_eq!(
            parse(&["-psn_0_12345", "--out", "/tmp/app"]),
            options("/tmp/app", false)
        );
        assert_eq!(
            parse(&["--out", "/tmp/app", "-psn_0_12345"]),
            options("/tmp/app", false)
        );
        assert_eq!(parse(&["-psn_0_1"]), options("os-probe-out", false));
    }

    /// `--not-key` is taken before or after `--out`, beside a `-psn_`.
    #[test]
    fn not_key_is_taken_in_any_order() {
        assert_eq!(parse(&["--not-key"]), options("os-probe-out", true));
        assert_eq!(
            parse(&["--out", "/tmp/nk", "--not-key"]),
            options("/tmp/nk", true)
        );
        assert_eq!(
            parse(&["--not-key", "--out", "/tmp/nk"]),
            options("/tmp/nk", true)
        );
        assert_eq!(
            parse(&["-psn_0_7", "--not-key", "--out=/tmp/nk"]),
            options("/tmp/nk", true)
        );
    }

    /// `--button-pass` is taken before or after `--out`.
    #[test]
    fn button_pass_is_taken_in_any_order() {
        let pass = |out: &str| -> Result<Options, String> {
            Ok(Options {
                out: PathBuf::from(out),
                not_key: false,
                button_pass: true,
                focus: false,
                inactive: false,
                focus_transition: false,
                key_not_main: false,
                main_not_key: false,
                symbols: false,
                reads: false,
            })
        };
        assert_eq!(parse(&["--button-pass"]), pass("os-probe-out"));
        assert_eq!(
            parse(&["--out", "/tmp/bp", "--button-pass"]),
            pass("/tmp/bp")
        );
        assert_eq!(parse(&["--button-pass", "--out=/tmp/bp"]), pass("/tmp/bp"));
    }

    /// The button pass's conditions come one at a time, with
    /// `--button-pass` only.
    #[test]
    fn a_button_condition_needs_the_button_pass_and_comes_alone() {
        let with = |focus: bool, not_key: bool, inactive: bool| -> Result<Options, String> {
            Ok(Options {
                out: PathBuf::from("os-probe-out"),
                not_key,
                button_pass: true,
                focus,
                inactive,
                focus_transition: false,
                key_not_main: false,
                main_not_key: false,
                symbols: false,
                reads: false,
            })
        };
        assert_eq!(
            parse(&["--button-pass", "--focus"]),
            with(true, false, false)
        );
        assert_eq!(
            parse(&["--inactive", "--button-pass"]),
            with(false, false, true)
        );
        assert_eq!(
            parse(&["--button-pass", "--not-key"]),
            with(false, true, false)
        );
        assert_eq!(
            parse(&["--focus-transition", "--button-pass"]),
            Ok(Options {
                out: PathBuf::from("os-probe-out"),
                not_key: false,
                button_pass: true,
                focus: false,
                inactive: false,
                focus_transition: true,
                key_not_main: false,
                main_not_key: false,
                symbols: false,
                reads: false,
            })
        );
        assert!(parse(&["--focus"]).is_err());
        assert!(parse(&["--focus-transition"]).is_err());
        assert!(parse(&["--button-pass", "--focus", "--focus-transition"]).is_err());
        assert!(parse(&["--inactive"]).is_err());
        assert!(parse(&["--button-pass", "--focus", "--inactive"]).is_err());
        assert!(parse(&["--button-pass", "--focus", "--not-key"]).is_err());
    }

    /// `--symbols` is taken in any order, and alone.
    #[test]
    fn symbols_is_taken_alone() {
        assert_eq!(
            parse(&["--symbols", "--out", "/tmp/s"]),
            Ok(Options {
                out: PathBuf::from("/tmp/s"),
                not_key: false,
                button_pass: false,
                focus: false,
                inactive: false,
                focus_transition: false,
                key_not_main: false,
                main_not_key: false,
                symbols: true,
                reads: false,
            })
        );
        assert!(parse(&["--symbols", "--button-pass"]).is_err());
        assert!(parse(&["--not-key", "--symbols"]).is_err());
        assert!(parse(&["--symbols", "--reads"]).is_err());
        assert!(parse(&["--reads", "--button-pass"]).is_err());
        assert!(parse(&["--reads", "-AppleLanguages", "(he)"]).is_ok());
        assert!(parse(&["--reads", "-AppleLanguages"]).is_err());
        assert!(parse(&["--key-not-main"]).is_err());
        assert!(parse(&["--button-pass", "--key-not-main", "--main-not-key"]).is_err());
        assert!(parse(&["--button-pass", "--main-not-key"]).is_ok());
    }
}
