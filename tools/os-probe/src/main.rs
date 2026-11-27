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
//! --main-not-key | --motion | --events | --controls]] [--symbols | --reads |
//! --colors]`. `--out` (or `--out=<dir>`) names the
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
//! round a plain focusable view. `--button-pass --motion` records presses,
//! radio buttons, a search field's cancel button, interrupted focus rings
//! and carets; `--button-pass --events` posts clicks, drags, hovers and
//! typing and reads what AppKit made of them; `--button-pass --controls`
//! hovers and presses the other controls (check boxes and radio buttons,
//! pop-up, disclosure, text field, tables, segmented control, slider, the
//! knobs of a legacy and an overlay scroller, switch, tab view, split view,
//! box, toolbar) over every backdrop, captures disabled check boxes and
//! radio buttons at rest, then the toolbar button again in a toolbar sheet whose
//! content view lies under its transparent title bar, and in that window a toolbar
//! of image items, the selected one, an unselected one and a disabled one, then the toolbar
//! button over an opaque title bar beside itself disabled, as a toggle on and off, and with an
//! image and no title; `--colors`, macOS only, the
//! colours pass alone: every NSColor and Core Animation's colour
//! interpolation.

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

const USAGE: &str = "usage: native-theme-os-probe [--out <dir>] [--not-key] [--button-pass [--focus | --focus-transition | --not-key | --inactive | --key-not-main | --main-not-key | --motion | --events | --controls]] [--symbols | --reads | --colors]   (default: os-probe-out; --not-key, macOS only: the key-window section, or the button pass, captures its windows after another window took the key; --button-pass, macOS only: the button pass alone; --focus, --focus-transition and --inactive, with --button-pass only: each button focused in turn, the screen recorded while two buttons are focused and unfocused, or another application active; --key-not-main and --main-not-key, with --button-pass only: each sheet a panel that is key while another window is main, or each sheet's window main while a panel is key; --motion and --events, with --button-pass only: scripted screen recordings of presses, radios, a search field and carets, or posted clicks, drags, hovers and typing; --controls, with --button-pass only: the other controls (the scrollers' knobs and radio buttons among them) hovered and pressed by posted mouse events over every backdrop, disabled check boxes and radio buttons at rest, then the toolbar button in a toolbar sheet, image items in a toolbar and the toolbar button's states (disabled, a toggle on and off, an image and no title) in a toolbar over an opaque title bar; --symbols, macOS only: the symbols pass alone; --reads, macOS only: the reads pass alone; --colors, macOS only: the colours pass alone)";

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
    /// Whether the button pass records its motion scripts instead.
    motion: bool,
    /// Whether the button pass posts its events scripts instead.
    events: bool,
    /// Whether the button pass hovers and presses its other controls instead.
    controls: bool,
    /// Whether the macOS probe runs the colours pass alone.
    colors: bool,
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
    let mut motion = false;
    let mut events = false;
    let mut controls = false;
    let mut colors = false;
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
            Some("--motion") => motion = true,
            Some("--events") => events = true,
            Some("--controls") => controls = true,
            Some("--colors") => colors = true,
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
    if (focus
        || inactive
        || focus_transition
        || key_not_main
        || main_not_key
        || motion
        || events
        || controls)
        && !button_pass
    {
        return Err(format!(
            "--focus, --focus-transition, --inactive, --key-not-main, --main-not-key, --motion, --events and --controls need --button-pass\n{USAGE}"
        ));
    }
    if [
        focus,
        inactive,
        not_key,
        focus_transition,
        key_not_main,
        main_not_key,
        motion,
        events,
        controls,
    ]
    .iter()
    .filter(|set| **set)
    .count()
        > 1
        && button_pass
    {
        return Err(format!(
            "the button pass takes one of --focus, --focus-transition, --not-key, --inactive, --key-not-main, --main-not-key, --motion, --events and --controls\n{USAGE}"
        ));
    }
    if [symbols, reads, colors].iter().filter(|set| **set).count() > 1
        || ((symbols || reads || colors) && (button_pass || not_key))
    {
        return Err(format!(
            "--symbols, --reads and --colors each run alone, without --button-pass, --not-key or each other\n{USAGE}"
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
        motion,
        events,
        controls,
        colors,
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
    } else if options.colors {
        macos::colors_probe(&mut doc, out_dir);
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
        } else if options.motion {
            macos::ButtonMode::Motion
        } else if options.events {
            macos::ButtonMode::Events
        } else if options.controls {
            macos::ButtonMode::Controls
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
    if options.controls
        && let Some(reason) = pass_error(&doc)
    {
        eprintln!("the controls pass failed: {reason}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

/// The appearances the controls pass captures each sheet in: the keys of
/// `APPEARANCES` in `macos.rs`.
#[cfg(any(target_os = "macos", target_os = "windows", test))]
const CONTROLS_APPEARANCES: [&str; 2] = ["aqua", "darkAqua"];

/// The keys of a state's captures over the black, white and grey backdrops
/// (`Backdrop::key` in `macos/buttons.rs`); the one over the window's own
/// background is the state's own `file` or its `window.file`.
#[cfg(any(target_os = "macos", target_os = "windows", test))]
const OVER_BACKDROPS: [&str; 3] = ["over_black", "over_white", "over_grey"];

/// Why the controls pass's output is a failure, where it is:
/// `[macos.button_pass]` missing, not a table or holding an `error`;
/// `[macos.button_pass.controls]`, its `toolbar_sheet`, its
/// `toolbar_items_sheet` or its `toolbar_states_sheet` missing, not a table
/// or holding an `error`; any of the four sheets with no target outside
/// error; or, in any of them, an
/// appearance missing, recorded unavailable or
/// holding an `error` (`appearance_table`'s cannot-create one), or a state's
/// failure (`states_error`). A `targets` row's own `error` is no failure.
#[cfg(any(target_os = "macos", target_os = "windows", test))]
fn pass_error(doc: &toml::Table) -> Option<String> {
    let pass = doc
        .get("macos")
        .and_then(|macos| macos.get("button_pass"))
        .ok_or("[macos.button_pass] is missing".to_owned())
        .and_then(|pass| table_without_error(pass, "[macos.button_pass]"));
    let controls = pass.and_then(|pass| {
        let path = "[macos.button_pass.controls]";
        let controls = pass
            .get("controls")
            .ok_or(format!("{path} is missing"))
            .and_then(|controls| table_without_error(controls, path))?;
        targets_error(controls, path)?;
        states_error(controls, path)?;
        Ok(controls)
    });
    controls
        .and_then(|controls| {
            for (key, path) in [
                (
                    "toolbar_sheet",
                    "[macos.button_pass.controls.toolbar_sheet]",
                ),
                (
                    "toolbar_items_sheet",
                    "[macos.button_pass.controls.toolbar_items_sheet]",
                ),
                (
                    "toolbar_states_sheet",
                    "[macos.button_pass.controls.toolbar_states_sheet]",
                ),
            ] {
                let sheet = controls
                    .get(key)
                    .ok_or(format!("{path} is missing"))
                    .and_then(|sheet| table_without_error(sheet, path))?;
                targets_error(sheet, path)?;
                states_error(sheet, path)?;
            }
            Ok(())
        })
        .err()
}

/// An error (the reason) where, in one of the sheet's appearances, its
/// `normal` or `highlighted` state lacks one of its four captures, an
/// `error` lies under `normal.backdrop_shows` or one of its rows reads
/// `over_backdrop = true` with `differing_pixels = 0`, the cursor's move
/// (`normal.cursor`, `hovered.cursor_parked`, `pressed.cursor_parked`) is
/// missing or fails as `first_error` takes it, or a target placed in
/// `targets` (no `error` of its own) lacks its `hovered.<target>` (unless
/// its `hovered` is false) or, where its `pressed` is true, its
/// `pressed.<target>`, or that state lacks one of its four captures or
/// fails as `first_error` takes it.
#[cfg(any(target_os = "macos", target_os = "windows", test))]
fn states_error(sheet: &toml::Table, path: &str) -> Result<(), String> {
    let placed: Vec<(&String, &toml::Table)> = sheet
        .get("targets")
        .and_then(toml::Value::as_table)
        .map(|targets| {
            targets
                .iter()
                .filter_map(|(name, t)| {
                    t.as_table()
                        .filter(|t| !t.contains_key("error"))
                        .map(|t| (name, t))
                })
                .collect()
        })
        .unwrap_or_default();
    for key in CONTROLS_APPEARANCES {
        let at = format!("{path}.{key}");
        let appearance = sheet
            .get(key)
            .ok_or(format!("{at} is missing"))
            .and_then(|a| table_without_error(a, &at))?;
        let state = |names: &[&str]| -> Result<&toml::Table, String> {
            let state_path = format!("{at}.{}", names.join("."));
            names
                .iter()
                .try_fold(appearance, |t, name| {
                    t.get(*name).and_then(toml::Value::as_table)
                })
                .ok_or(format!("{state_path} is missing or not a table"))
        };
        for name in ["normal", "highlighted"] {
            captures_error(state(&[name])?, &format!("{at}.{name}"))?;
        }
        let shows = state(&["normal", "backdrop_shows"])?;
        first_error(shows, &format!("{at}.normal.backdrop_shows"))?;
        backdrop_flag_error(shows, &format!("{at}.normal.backdrop_shows"))?;
        for names in [
            ["normal", "cursor"],
            ["hovered", "cursor_parked"],
            ["pressed", "cursor_parked"],
        ] {
            first_error(state(&names)?, &format!("{at}.{}", names.join(".")))?;
        }
        for (name, target) in &placed {
            let flag = |k: &str| target.get(k).and_then(toml::Value::as_bool);
            for (kind, wanted) in [
                ("hovered", flag("hovered") != Some(false)),
                ("pressed", flag("pressed") == Some(true)),
            ] {
                if wanted {
                    let s = state(&[kind, name])?;
                    let s_path = format!("{at}.{kind}.{name}");
                    first_error(s, &s_path)?;
                    captures_error(s, &s_path)?;
                }
            }
        }
    }
    Ok(())
}

/// An error (the reason) where a row of `normal.backdrop_shows` reads
/// `over_backdrop = true` and `differing_pixels = 0`: the part was taken as
/// lying over the backdrop, yet the backdrop does not show through it.
#[cfg(any(target_os = "macos", target_os = "windows", test))]
fn backdrop_flag_error(shows: &toml::Table, path: &str) -> Result<(), String> {
    for (name, row) in shows {
        let over = row.get("over_backdrop").and_then(toml::Value::as_bool);
        let differing = row
            .get("differing_pixels")
            .and_then(toml::Value::as_integer);
        if over == Some(true) && differing == Some(0) {
            return Err(format!(
                "{path}.{name} reads over_backdrop = true with differing_pixels = 0"
            ));
        }
    }
    Ok(())
}

/// An error (the reason) where `state` lacks a decoded capture: its own
/// `file` or `window.file`, and `<backdrop>.file` for each of
/// `OVER_BACKDROPS`; a capture counts only where its table also holds the
/// decoded size (`pixels_wh`) and no `capture` or `decode` error.
#[cfg(any(target_os = "macos", target_os = "windows", test))]
fn captures_error(state: &toml::Table, path: &str) -> Result<(), String> {
    let decoded = |t: &toml::Table| {
        t.get("file").and_then(toml::Value::as_str).is_some()
            && t.get("pixels_wh").and_then(toml::Value::as_array).is_some()
            && !t.contains_key("capture")
            && !t.contains_key("decode")
    };
    let file = |t: Option<&toml::Value>| t.and_then(toml::Value::as_table).is_some_and(decoded);
    let window = decoded(state) || file(state.get("window"));
    let missing: Vec<&str> = std::iter::once(("window", window))
        .chain(OVER_BACKDROPS.iter().map(|k| (*k, file(state.get(*k)))))
        .filter(|(_, present)| !present)
        .map(|(k, _)| k)
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!("{path} has no capture {}", missing.join(", ")))
    }
}

/// An error (the reason) at the first `error` key,
/// `window_at_press_is_ours = false`, `window_at_target_is_ours = false`,
/// `window_under_cursor_is_ours = false` or non-zero
/// `CGWarpMouseCursorPosition`, anywhere under `table`, its arrays included.
#[cfg(any(target_os = "macos", target_os = "windows", test))]
fn first_error(table: &toml::Table, path: &str) -> Result<(), String> {
    for (key, value) in table {
        let at = format!("{path}.{key}");
        if key == "error" {
            return Err(format!("{at} = {value}"));
        }
        if key == "window_at_press_is_ours" && value.as_bool() == Some(false) {
            return Err(format!("{at} = false: the press was skipped"));
        }
        if key == "window_at_target_is_ours" && value.as_bool() == Some(false) {
            return Err(format!("{at} = false: the cursor's move was skipped"));
        }
        if key == "window_under_cursor_is_ours" && value.as_bool() == Some(false) {
            return Err(format!(
                "{at} = false: the cursor did not end on the probe's window"
            ));
        }
        if key == "CGWarpMouseCursorPosition" && value.as_integer().is_some_and(|r| r != 0) {
            return Err(format!("{at} = {value}: the cursor's warp failed"));
        }
        value_error(value, &at)?;
    }
    Ok(())
}

/// `first_error` of `value` where it is a table, or of each table in it
/// where it is an array.
#[cfg(any(target_os = "macos", target_os = "windows", test))]
fn value_error(value: &toml::Value, path: &str) -> Result<(), String> {
    match value {
        toml::Value::Table(t) => first_error(t, path),
        toml::Value::Array(items) => items
            .iter()
            .enumerate()
            .try_for_each(|(i, item)| value_error(item, &format!("{path}[{i}]"))),
        _ => Ok(()),
    }
}

/// `value` as a table without an `error` key, or (as `Err`, the reason)
/// what it is otherwise.
#[cfg(any(target_os = "macos", target_os = "windows", test))]
fn table_without_error<'a>(value: &'a toml::Value, path: &str) -> Result<&'a toml::Table, String> {
    let Some(table) = value.as_table() else {
        return Err(format!("{path} is not a table: {value}"));
    };
    match table.get("error") {
        Some(error) => Err(format!("{path} error = {error}")),
        None => Ok(table),
    }
}

/// An error (the reason) where `sheet`'s `targets` is missing or not a
/// table, or where none of its targets is a table without an `error`.
#[cfg(any(target_os = "macos", target_os = "windows", test))]
fn targets_error(sheet: &toml::Table, path: &str) -> Result<(), String> {
    let Some(targets) = sheet.get("targets").and_then(toml::Value::as_table) else {
        return Err(format!("{path} has no targets table"));
    };
    let placed = targets
        .values()
        .any(|target| target.as_table().is_some_and(|t| !t.contains_key("error")));
    if placed {
        Ok(())
    } else {
        Err(format!(
            "every target of {path}.targets is in error ({} targets)",
            targets.len()
        ))
    }
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
            motion: false,
            events: false,
            controls: false,
            colors: false,
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
                motion: false,
                events: false,
                controls: false,
                colors: false,
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
                motion: false,
                events: false,
                controls: false,
                colors: false,
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
                motion: false,
                events: false,
                controls: false,
                colors: false,
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
                motion: false,
                events: false,
                controls: false,
                colors: false,
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
        assert!(parse(&["--button-pass", "--motion"]).is_ok());
        assert!(parse(&["--button-pass", "--events"]).is_ok());
        assert!(parse(&["--motion"]).is_err());
        assert!(parse(&["--button-pass", "--motion", "--events"]).is_err());
        assert!(parse(&["--colors"]).is_ok());
        assert!(parse(&["--colors", "--reads"]).is_err());
        assert!(parse(&["--colors", "--button-pass"]).is_err());
    }

    /// `--controls` is taken with `--button-pass` in any order, and without
    /// another of the button pass's conditions.
    #[test]
    fn controls_is_taken_with_the_button_pass_alone() {
        let controls = |out: &str| -> Result<Options, String> {
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
                motion: false,
                events: false,
                controls: true,
                colors: false,
            })
        };
        assert_eq!(
            parse(&["--button-pass", "--controls"]),
            controls("os-probe-out")
        );
        assert_eq!(
            parse(&["--controls", "--out", "/tmp/c", "--button-pass"]),
            controls("/tmp/c")
        );
        assert!(parse(&["--controls"]).is_err());
        assert!(parse(&["--button-pass", "--controls", "--events"]).is_err());
        assert!(parse(&["--button-pass", "--controls", "--not-key"]).is_err());
    }

    const CONTROLS: &str = "macos.button_pass.controls";
    const TOOLBAR: &str = "macos.button_pass.controls.toolbar_sheet";
    const ITEMS: &str = "macos.button_pass.controls.toolbar_items_sheet";
    const STATES: &str = "macos.button_pass.controls.toolbar_states_sheet";

    /// The four captures of a state at `path`: the window's own as the
    /// state's `file` (`own`) or its `window.file`.
    fn captures(path: &str, own: bool) -> String {
        let decoded = "pixels_wh = [2, 2]\n";
        let mut text = if own {
            format!("[{path}]\nfile = \"a.png\"\n{decoded}")
        } else {
            format!("[{path}.window]\nfile = \"a.png\"\n{decoded}")
        };
        for backdrop in OVER_BACKDROPS {
            text.push_str(&format!("[{path}.{backdrop}]\nfile = \"b.png\"\n{decoded}"));
        }
        text
    }

    /// A cursor's move at `path` that reached the probe's own window.
    fn moved(path: &str) -> String {
        format!(
            "[{path}]\nwindow_at_target_is_ours = true\nCGWarpMouseCursorPosition = 0\nwindow_under_cursor_is_ours = true\n"
        )
    }

    /// One sheet at `sheet` with `targets` (name, hovered, pressed), each
    /// placed, and every state of both appearances captured whole.
    fn sheet_text(sheet: &str, targets: &[(&str, bool, bool)]) -> String {
        let mut text = String::new();
        for (name, hovered, pressed) in targets {
            text.push_str(&format!(
                "[{sheet}.targets.{name}]\nclass = \"NSButton\"\nhovered = {hovered}\npressed = {pressed}\n"
            ));
        }
        for key in CONTROLS_APPEARANCES {
            text.push_str(&captures(&format!("{sheet}.{key}.normal"), true));
            text.push_str(&captures(&format!("{sheet}.{key}.highlighted"), true));
            text.push_str(&moved(&format!("{sheet}.{key}.normal.cursor")));
            text.push_str(&moved(&format!("{sheet}.{key}.hovered.cursor_parked")));
            text.push_str(&moved(&format!("{sheet}.{key}.pressed.cursor_parked")));
            for (name, hovered, pressed) in targets {
                text.push_str(&format!(
                    "[{sheet}.{key}.normal.backdrop_shows.{name}]\nover_backdrop = true\ndiffering_pixels = 1\n"
                ));
                if *hovered {
                    text.push_str(&captures(&format!("{sheet}.{key}.hovered.{name}"), true));
                    text.push_str(&moved(&format!("{sheet}.{key}.hovered.{name}.cursor")));
                }
                if *pressed {
                    text.push_str(&captures(&format!("{sheet}.{key}.pressed.{name}"), false));
                    text.push_str(&moved(&format!(
                        "{sheet}.{key}.pressed.{name}.over_black.cursor"
                    )));
                    text.push_str(&format!(
                        "[{sheet}.{key}.pressed.{name}.window.press]\nwindow_at_press_is_ours = true\n"
                    ));
                }
            }
        }
        text
    }

    /// A whole controls pass's output: in the controls sheet a pressed
    /// target, a hovered one and one neither, in the toolbar sheet a
    /// pressed one, in the toolbar items sheet its three items and in the
    /// toolbar states sheet its nine specimens.
    fn controls_doc() -> Result<toml::Table, String> {
        let mut text = sheet_text(
            CONTROLS,
            &[
                ("push", true, true),
                ("box", true, false),
                ("radio_disabled_on", false, false),
            ],
        );
        text.push_str(&sheet_text(TOOLBAR, &[("toolbar_button", true, true)]));
        text.push_str(&sheet_text(
            ITEMS,
            &[
                ("selected", true, true),
                ("unselected", true, false),
                ("disabled", false, false),
            ],
        ));
        text.push_str(&sheet_text(
            STATES,
            &[
                ("toolbar_button", true, true),
                ("toolbar_button_disabled", false, false),
                ("toolbar_toggle_on", true, true),
                ("toolbar_toggle_off", false, false),
                ("toolbar_image", true, false),
                ("toolbar_image_on", true, true),
                ("toolbar_image_disabled", false, false),
                ("toolbar_titled_image_on", true, true),
                ("toolbar_titled_image_disabled", false, false),
            ],
        ));
        text.parse().map_err(|e: toml::de::Error| e.to_string())
    }

    /// The table at `path` (dotted keys) in `doc`.
    fn table_at<'a>(doc: &'a mut toml::Table, path: &str) -> Result<&'a mut toml::Table, String> {
        path.split('.').try_fold(doc, |t, key| {
            t.get_mut(key)
                .and_then(toml::Value::as_table_mut)
                .ok_or(format!("{path}: no table {key}"))
        })
    }

    /// `Ok` where the output was a failure, else the error.
    fn fails(failed: bool) -> Result<(), String> {
        if failed {
            Ok(())
        } else {
            Err("the output is no failure".to_owned())
        }
    }

    /// `Ok` where `doc` is no failure, else its reason.
    fn passes(doc: &toml::Table) -> Result<(), String> {
        pass_error(doc).map_or(Ok(()), Err)
    }

    #[test]
    fn a_whole_controls_pass_is_no_failure() -> Result<(), String> {
        passes(&controls_doc()?)
    }

    #[test]
    fn a_target_rows_own_error_is_no_failure() -> Result<(), String> {
        let mut doc = controls_doc()?;
        let row = table_at(&mut doc, &format!("{CONTROLS}.targets.box"))?;
        row.insert("error".to_owned(), "unavailable: the part is empty".into());
        for key in CONTROLS_APPEARANCES {
            table_at(&mut doc, &format!("{CONTROLS}.{key}.hovered"))?.remove("box");
        }
        passes(&doc)
    }

    /// `edit` applied to a whole controls pass's output, and whether that
    /// output is then a failure.
    fn fails_after(edit: impl Fn(&mut toml::Table) -> Result<(), String>) -> Result<bool, String> {
        let mut doc = controls_doc()?;
        edit(&mut doc)?;
        Ok(pass_error(&doc).is_some())
    }

    /// `key = value` set in the table at `path`.
    fn set(doc: &mut toml::Table, path: &str, key: &str, value: toml::Value) -> Result<(), String> {
        table_at(doc, path)?.insert(key.to_owned(), value);
        Ok(())
    }

    /// The key `key` removed from the table at `path`.
    fn remove(doc: &mut toml::Table, path: &str, key: &str) -> Result<(), String> {
        table_at(doc, path)?
            .remove(key)
            .map(|_| ())
            .ok_or(format!("{path} has no {key}"))
    }

    #[test]
    fn an_error_under_a_hovered_or_pressed_target_is_a_failure() -> Result<(), String> {
        let error = || toml::Value::from("unavailable: x");
        fails(fails_after(|d| {
            set(
                d,
                &format!("{CONTROLS}.aqua.hovered.push"),
                "error",
                error(),
            )
        })?)?;
        fails(fails_after(|d| {
            set(
                d,
                &format!("{CONTROLS}.darkAqua.pressed.push.over_black.cursor"),
                "error",
                error(),
            )
        })?)?;
        fails(fails_after(|d| {
            set(
                d,
                &format!("{TOOLBAR}.darkAqua.pressed.toolbar_button.window"),
                "error",
                error(),
            )
        })?)?;
        fails(fails_after(|d| {
            let mut scroll = toml::Table::new();
            scroll.insert("error".to_owned(), error());
            let scrolls = toml::Value::Array(vec![toml::Table::new().into(), scroll.into()]);
            let mut reveal = toml::Table::new();
            reveal.insert("scrolls".to_owned(), scrolls);
            set(
                d,
                &format!("{CONTROLS}.aqua.hovered.box"),
                "reveal",
                reveal.into(),
            )
        })?)?;
        Ok(())
    }

    #[test]
    fn a_press_skipped_as_not_on_the_probes_window_is_a_failure() -> Result<(), String> {
        fails(fails_after(|d| {
            set(
                d,
                &format!("{CONTROLS}.aqua.pressed.push.window.press"),
                "window_at_press_is_ours",
                false.into(),
            )
        })?)?;
        Ok(())
    }

    #[test]
    fn an_appearance_unavailable_or_in_error_is_a_failure() -> Result<(), String> {
        fails(fails_after(|d| {
            set(
                d,
                CONTROLS,
                "darkAqua",
                "unavailable: appearanceNamed(NSAppearanceNameDarkAqua) is nil".into(),
            )
        })?)?;
        fails(fails_after(|d| {
            set(
                d,
                &format!("{TOOLBAR}.aqua"),
                "error",
                "unavailable: cannot create aqua/controls_toolbar".into(),
            )
        })?)?;
        fails(fails_after(|d| remove(d, TOOLBAR, "aqua"))?)?;
        Ok(())
    }

    #[test]
    fn a_missing_capture_or_state_is_a_failure() -> Result<(), String> {
        fails(fails_after(|d| {
            remove(d, &format!("{CONTROLS}.aqua.normal"), "over_grey")
        })?)?;
        fails(fails_after(|d| {
            remove(d, &format!("{CONTROLS}.darkAqua.highlighted"), "file")
        })?)?;
        fails(fails_after(|d| {
            remove(d, &format!("{CONTROLS}.aqua.pressed.push.window"), "file")
        })?)?;
        fails(fails_after(|d| {
            remove(
                d,
                &format!("{TOOLBAR}.darkAqua.pressed.toolbar_button.over_white"),
                "file",
            )
        })?)?;
        fails(fails_after(|d| {
            remove(d, &format!("{CONTROLS}.aqua.pressed"), "push")
        })?)?;
        fails(fails_after(|d| {
            remove(d, &format!("{CONTROLS}.darkAqua.hovered"), "box")
        })?)?;
        fails(fails_after(|d| {
            remove(d, &format!("{CONTROLS}.aqua.normal"), "backdrop_shows")
        })?)?;
        Ok(())
    }

    #[test]
    fn a_target_hovered_false_needs_no_hovered_state() -> Result<(), String> {
        let mut doc = controls_doc()?;
        let hovered = table_at(&mut doc, &format!("{CONTROLS}.aqua.hovered"))?;
        if hovered.contains_key("radio_disabled_on") {
            return Err("the output holds the target's hovered state".to_owned());
        }
        passes(&doc)?;
        set(
            &mut doc,
            &format!("{CONTROLS}.targets.radio_disabled_on"),
            "hovered",
            true.into(),
        )?;
        fails(pass_error(&doc).is_some())
    }

    #[test]
    fn an_error_under_backdrop_shows_is_a_failure() -> Result<(), String> {
        fails(fails_after(|d| {
            set(
                d,
                &format!("{CONTROLS}.darkAqua.normal.backdrop_shows.push"),
                "error",
                "unavailable: over_backdrop is true, yet no pixel of the part differs".into(),
            )
        })?)?;
        Ok(())
    }

    #[test]
    fn a_target_over_the_backdrop_with_no_differing_pixel_is_a_failure() -> Result<(), String> {
        let shows = format!("{CONTROLS}.aqua.normal.backdrop_shows.box");
        fails(fails_after(|d| {
            set(d, &shows, "differing_pixels", 0.into())
        })?)?;
        let mut doc = controls_doc()?;
        set(&mut doc, &shows, "differing_pixels", 0.into())?;
        set(&mut doc, &shows, "over_backdrop", false.into())?;
        passes(&doc)
    }

    #[test]
    fn a_cursor_move_skipped_or_in_error_is_a_failure() -> Result<(), String> {
        for path in [
            format!("{CONTROLS}.aqua.normal.cursor"),
            format!("{CONTROLS}.darkAqua.hovered.cursor_parked"),
            format!("{TOOLBAR}.aqua.pressed.cursor_parked"),
            format!("{CONTROLS}.aqua.hovered.box.cursor"),
            format!("{CONTROLS}.darkAqua.pressed.push.over_black.cursor"),
        ] {
            fails(fails_after(|d| {
                set(d, &path, "window_at_target_is_ours", false.into())
            })?)?;
            fails(fails_after(|d| {
                set(d, &path, "error", "unavailable: x".into())
            })?)?;
            fails(fails_after(|d| {
                set(d, &path, "window_under_cursor_is_ours", false.into())
            })?)?;
            fails(fails_after(|d| {
                set(d, &path, "CGWarpMouseCursorPosition", 1000.into())
            })?)?;
        }
        fails(fails_after(|d| {
            remove(d, &format!("{TOOLBAR}.darkAqua.hovered"), "cursor_parked")
        })?)?;
        Ok(())
    }

    #[test]
    fn a_capture_that_did_not_decode_is_missing() -> Result<(), String> {
        fails(fails_after(|d| {
            set(
                d,
                &format!("{CONTROLS}.aqua.normal.over_white"),
                "capture",
                "unavailable: the capture is not a bitmap image".into(),
            )
        })?)?;
        fails(fails_after(|d| {
            set(
                d,
                &format!("{CONTROLS}.darkAqua.pressed.push.window"),
                "decode",
                "unavailable: x".into(),
            )
        })?)?;
        fails(fails_after(|d| {
            remove(
                d,
                &format!("{TOOLBAR}.aqua.hovered.toolbar_button"),
                "pixels_wh",
            )
        })?)?;
        Ok(())
    }

    #[test]
    fn a_toolbar_sheet_missing_or_in_error_is_a_failure() -> Result<(), String> {
        type Edit<'a> = &'a dyn Fn(&mut toml::Table) -> Result<(), String>;
        let edits: [Edit<'_>; 3] = [
            &|d| remove(d, CONTROLS, "toolbar_sheet"),
            &|d| set(d, TOOLBAR, "error", "unavailable: x".into()),
            &|d| {
                set(
                    d,
                    &format!("{TOOLBAR}.targets.toolbar_button"),
                    "error",
                    "unavailable: the view is in no window".into(),
                )
            },
        ];
        for edit in edits {
            let mut doc = controls_doc()?;
            edit(&mut doc)?;
            match pass_error(&doc) {
                Some(reason) if reason.contains("[macos.button_pass.controls.toolbar_sheet]") => {}
                other => return Err(format!("not the toolbar sheet's failure: {other:?}")),
            }
        }
        Ok(())
    }

    #[test]
    fn a_toolbar_items_sheet_missing_or_in_error_is_a_failure() -> Result<(), String> {
        type Edit<'a> = &'a dyn Fn(&mut toml::Table) -> Result<(), String>;
        let in_error = |name: &'static str| {
            move |d: &mut toml::Table| {
                set(
                    d,
                    &format!("{ITEMS}.targets.{name}"),
                    "error",
                    "unavailable: 2 views with an NSToolbarImageView found".into(),
                )
            }
        };
        let every_target = |d: &mut toml::Table| {
            ["selected", "unselected", "disabled"]
                .into_iter()
                .try_for_each(|name| in_error(name)(d))
        };
        let edits: [Edit<'_>; 4] = [
            &|d| remove(d, CONTROLS, "toolbar_items_sheet"),
            &|d| set(d, ITEMS, "error", "unavailable: x".into()),
            &every_target,
            &|d| remove(d, ITEMS, "darkAqua"),
        ];
        for edit in edits {
            let mut doc = controls_doc()?;
            edit(&mut doc)?;
            match pass_error(&doc) {
                Some(reason)
                    if reason.contains("[macos.button_pass.controls.toolbar_items_sheet]") => {}
                other => return Err(format!("not the toolbar items sheet's failure: {other:?}")),
            }
        }
        let mut doc = controls_doc()?;
        in_error("disabled")(&mut doc)?;
        match pass_error(&doc) {
            None => Ok(()),
            Some(reason) => Err(format!("one target in error failed the pass: {reason}")),
        }
    }

    #[test]
    fn a_toolbar_states_sheet_missing_or_in_error_is_a_failure() -> Result<(), String> {
        type Edit<'a> = &'a dyn Fn(&mut toml::Table) -> Result<(), String>;
        const SPECIMENS: [&str; 9] = [
            "toolbar_button",
            "toolbar_button_disabled",
            "toolbar_toggle_on",
            "toolbar_toggle_off",
            "toolbar_image",
            "toolbar_image_on",
            "toolbar_image_disabled",
            "toolbar_titled_image_on",
            "toolbar_titled_image_disabled",
        ];
        let in_error = |name: &'static str| {
            move |d: &mut toml::Table| {
                set(
                    d,
                    &format!("{STATES}.targets.{name}"),
                    "error",
                    "unavailable: the view is in no window".into(),
                )
            }
        };
        let every_target =
            |d: &mut toml::Table| SPECIMENS.into_iter().try_for_each(|name| in_error(name)(d));
        let edits: [Edit<'_>; 5] = [
            &|d| remove(d, CONTROLS, "toolbar_states_sheet"),
            &|d| set(d, STATES, "error", "unavailable: x".into()),
            &every_target,
            &|d| remove(d, STATES, "darkAqua"),
            &|d| remove(d, &format!("{STATES}.aqua.pressed"), "toolbar_toggle_on"),
        ];
        for edit in edits {
            let mut doc = controls_doc()?;
            edit(&mut doc)?;
            match pass_error(&doc) {
                Some(reason)
                    if reason.contains("[macos.button_pass.controls.toolbar_states_sheet]") => {}
                other => return Err(format!("not the toolbar states sheet's failure: {other:?}")),
            }
        }
        let mut doc = controls_doc()?;
        in_error("toolbar_image")(&mut doc)?;
        match pass_error(&doc) {
            None => Ok(()),
            Some(reason) => Err(format!("one target in error failed the pass: {reason}")),
        }
    }

    #[test]
    fn a_controls_pass_missing_or_in_error_is_a_failure() {
        for text in [
            "",
            "[macos]\nbutton_pass = \"unavailable: not on the main thread\"\n",
            "[macos.button_pass]\nerror = \"unavailable: NSScreen screens is empty\"\n",
            "[macos.button_pass]\nmode = \"controls\"\n",
            "[macos.button_pass]\ncontrols = \"unavailable: x\"\n",
            "[macos.button_pass.controls]\nerror = \"unavailable: x\"\n",
            "[macos.button_pass.controls.targets.push]\nerror = \"unavailable: x\"\n\
             [macos.button_pass.controls.toolbar_sheet.targets.toolbar_button]\nclass = \"NSButton\"\n",
            "[macos.button_pass.controls]\ntargets = {}\n\
             [macos.button_pass.controls.toolbar_sheet.targets.toolbar_button]\nclass = \"NSButton\"\n",
        ] {
            assert!(
                matches!(
                    text.parse::<toml::Table>().map(|doc| pass_error(&doc)),
                    Ok(Some(_))
                ),
                "no failure for {text:?}"
            );
        }
    }
}
