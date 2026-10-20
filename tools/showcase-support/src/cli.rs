//! The showcases' command line: one flag set, one parser, one validator per
//! flag with one error text, and one exit rule.
//!
//! The flags: `--theme`, `--variant`, `--tab`, `--icon-set` (with `default`),
//! `--screenshot`, `--capture`, `--pointer`, `--press`, `--dump-layout` and
//! `--open-menu`. The exit rule:
//!
//! - a bad `--pointer` exits 1 before the window opens;
//! - `--screenshot` exits 1 before the window opens where the showcase cannot
//!   capture its frame ([`screenshot_support`]);
//! - a value a picker does not offer is reported and ignored ([`reported`]),
//!   `--open-menu` included;
//! - a run that writes files (`--dump-layout`, `--screenshot`) quits once
//!   every file it asked for is written, and keeps its window while a tool
//!   outside it captures it (`--capture`) ([`Outputs`]);
//! - a failed `--screenshot`, dump write or window open exits 1 ([`refuse`]),
//!   a failed file naming its flag and path ([`Output::failure`]).
//!
//! A toolkit that needs a flag of its own reads it in its own `main` after
//! [`CliArgs::parse`], which ignores what it does not know.

use native_theme::icons::IconSetChoice;
use native_theme::theme::Theme;

use crate::Kind;

/// The flags the capture pipeline passes (`.github/workflows/screenshots.yml`,
/// `scripts/generate_screenshots_*.sh`). With none the showcase opens in its
/// defaults, on the Basic page.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CliArgs {
    /// `--theme <name>`: `default` or a preset of this platform
    /// ([`theme_key`]).
    pub theme: Option<String>,
    /// `--variant <mode>`: `light`, `dark` or `system` ([`color_mode`]).
    pub variant: Option<String>,
    /// `--tab <page>`: a page's key ([`page_key`]).
    pub tab: Option<String>,
    /// `--icon-set <name>`: a row of the icon-theme picker
    /// ([`icon_set_choice`]).
    pub icon_set: Option<String>,
    /// `--screenshot <file.png>`: the showcase captures its window with its
    /// frame there and quits, on macOS and Windows ([`screenshot_support`]).
    pub screenshot: Option<String>,
    /// `--capture`: a tool outside the showcase captures its window, which
    /// opens as a `--screenshot` run's does, at its default size under an app
    /// id of its own ([`crate::capture::app_id`]).
    pub capture: bool,
    /// `--pointer X,Y`: the pointer is held at that point of the window's
    /// content, in logical pixels, so a capture shows the control under it
    /// hovered. For captures where no pointer can be driven, as in a nested
    /// compositor.
    pub pointer: Option<(u16, u16)>,
    /// A `--pointer` value that is not `X,Y`, or its missing value: the
    /// showcase reports it and exits with 1 ([`refuse`]), rather than
    /// capture the window at rest.
    pub bad_pointer: Option<String>,
    /// `--press`: with `--pointer`, the primary button is held down there, so
    /// a capture shows the control pressed.
    pub press: bool,
    /// `--dump-layout <file.json>`: once the layout has settled, where each
    /// element of `docs/showcase-elements.toml` was drawn is written there
    /// ([`crate::dump`]), at rest wherever the desktop's pointer is
    /// ([`CliArgs::shields_pointer`]); the showcase then quits, or keeps
    /// running while something captures ([`CliArgs::quits_after_dump`]).
    pub dump_layout: Option<String>,
    /// `--open-menu <title>`: the menu of that title (`theme` for the Theme
    /// menu) is open from the first frame, so a capture and a layout dump
    /// show it open with its rows ([`open_menu`]).
    pub open_menu: Option<String>,
}

impl CliArgs {
    /// The flags in `argv`, the arguments after the program name, as
    /// `std::env::args().skip(1)` gives them: `--flag value` pairs,
    /// `--capture`, `--press` and `--pointer X,Y`. An unknown argument is
    /// ignored. `--variant`, `--tab` and `--open-menu` are lower-cased; a
    /// theme or icon-theme name keeps its case, since a freedesktop theme's
    /// is a directory name.
    pub fn parse(argv: impl IntoIterator<Item = String>) -> Self {
        let mut cli = Self::default();
        let mut argv = argv.into_iter();
        while let Some(flag) = argv.next() {
            let lower = matches!(flag.as_str(), "--variant" | "--tab" | "--open-menu");
            let slot = match flag.as_str() {
                "--capture" => {
                    cli.capture = true;
                    continue;
                }
                "--press" => {
                    cli.press = true;
                    continue;
                }
                "--pointer" => {
                    let value = argv.next().unwrap_or_default();
                    match parse_point(&value) {
                        Some(point) => cli.pointer = Some(point),
                        None => cli.bad_pointer = Some(value),
                    }
                    continue;
                }
                "--theme" => &mut cli.theme,
                "--variant" => &mut cli.variant,
                "--tab" => &mut cli.tab,
                "--icon-set" => &mut cli.icon_set,
                "--screenshot" => &mut cli.screenshot,
                "--dump-layout" => &mut cli.dump_layout,
                "--open-menu" => &mut cli.open_menu,
                _ => continue,
            };
            *slot = argv
                .next()
                .map(|value| if lower { value.to_lowercase() } else { value });
        }
        cli
    }

    /// Whether the window is captured: by the showcase itself
    /// (`--screenshot`) or by a tool outside it (`--capture`).
    #[must_use]
    pub fn capturing(&self) -> bool {
        self.capture || self.screenshot.is_some()
    }

    /// Whether the window's own pointer is kept off every widget: in a
    /// capture and in a layout dump, which hold no pointer of their own.
    /// Wherever the desktop's pointer happens to be, it then hovers nothing,
    /// so two captures of one theme draw the same and two dumps of it record
    /// the chrome at rest alike. A held pointer (`--pointer`) takes the
    /// window's place, and is not shielded.
    #[must_use]
    pub fn shields_pointer(&self) -> bool {
        (self.capturing() || self.dump_layout.is_some()) && self.pointer.is_none()
    }

    /// Whether the showcase follows the OS theme as it changes: not in a
    /// capture or a layout dump, which show the theme the run started with.
    #[must_use]
    pub fn follows_os(&self) -> bool {
        !self.capturing() && self.dump_layout.is_none()
    }

    /// Whether the showcase quits once its layout dump is written: only when
    /// nothing captures the window, so `--dump-layout F --screenshot P`
    /// writes P too, and `--dump-layout F --capture` keeps the window for
    /// the capture. A run that writes both quits once both are written
    /// ([`Outputs`]).
    #[must_use]
    pub fn quits_after_dump(&self) -> bool {
        !self.capturing()
    }
}

/// A file a run writes before it quits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Output {
    /// `--dump-layout`'s layout dump ([`crate::dump`]).
    Dump,
    /// `--screenshot`'s capture of the window with its frame
    /// ([`crate::capture::FrameCapture`]).
    Screenshot,
}

impl Output {
    /// The flag that asks for the file.
    #[must_use]
    pub fn flag(self) -> &'static str {
        match self {
            Output::Dump => "--dump-layout",
            Output::Screenshot => "--screenshot",
        }
    }

    /// Why the file at `path` was not written, naming its flag and path:
    /// the text a showcase hands [`refuse`], so the run exits 1.
    #[must_use]
    pub fn failure(self, path: &str, error: &str) -> String {
        format!("{} {path}: {error}", self.flag())
    }
}

/// The showcases' exit rule for the files a run writes: `--dump-layout F`
/// and `--screenshot P`, alone or together. The run quits once every file
/// it asked for is written, whichever is written last, so neither is lost
/// to the other's quit; under `--capture` it keeps its window for the tool
/// that captures it, which closes it. A file that fails exits 1 at once
/// ([`Output::failure`], [`refuse`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Outputs {
    dump: bool,
    screenshot: bool,
    keep_window: bool,
}

impl Outputs {
    /// The files `cli` asks for, none of them written yet.
    #[must_use]
    pub fn new(cli: &CliArgs) -> Self {
        Self {
            dump: cli.dump_layout.is_some(),
            screenshot: cli.screenshot.is_some(),
            keep_window: cli.capture,
        }
    }

    /// Whether the run asked for `output` and it is not written yet: a
    /// showcase that gives up waiting for it fails the run
    /// ([`Output::failure`], [`refuse`]).
    #[must_use]
    pub fn pending(&self, output: Output) -> bool {
        match output {
            Output::Dump => self.dump,
            Output::Screenshot => self.screenshot,
        }
    }

    /// Records `output` as written, and whether the run quits now: every
    /// file it asked for is written and no `--capture` keeps the window.
    pub fn written(&mut self, output: Output) -> bool {
        match output {
            Output::Dump => self.dump = false,
            Output::Screenshot => self.screenshot = false,
        }
        !self.dump && !self.screenshot && !self.keep_window
    }
}

/// `X,Y` as two whole logical pixels, or `None`.
#[must_use]
pub fn parse_point(value: &str) -> Option<(u16, u16)> {
    let (x, y) = value.split_once(',')?;
    Some((x.trim().parse().ok()?, y.trim().parse().ok()?))
}

/// The colour-mode picker's rows: follow the OS, or a fixed mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorModeChoice {
    System,
    Light,
    Dark,
}

impl ColorModeChoice {
    /// Every mode, in the order the colour-mode pickers and the Theme menu
    /// list them.
    pub const ALL: [ColorModeChoice; 3] = [
        ColorModeChoice::System,
        ColorModeChoice::Light,
        ColorModeChoice::Dark,
    ];

    /// Whether the mode is dark: `System` as the OS reports it
    /// (`native_theme::detect::system_is_dark`).
    #[must_use]
    pub fn is_dark(self) -> bool {
        match self {
            ColorModeChoice::Light => false,
            ColorModeChoice::Dark => true,
            ColorModeChoice::System => native_theme::detect::system_is_dark(),
        }
    }

    /// The mode's label in the command palette, built each time it opens:
    /// `System` with the mode the OS reports in parentheses
    /// (`System (Dark)`), `Light`, `Dark`.
    #[must_use]
    pub fn label(self) -> String {
        match self {
            ColorModeChoice::System => {
                let actual = if native_theme::detect::system_is_dark() {
                    "Dark"
                } else {
                    "Light"
                };
                format!("System ({actual})")
            }
            ColorModeChoice::Light | ColorModeChoice::Dark => self.short_label().to_string(),
        }
    }

    /// The colour-mode picker's row and the Theme menu's item for the mode:
    /// the mode alone. The mode `System` resolves to is in the status bar
    /// while it is chosen.
    #[must_use]
    pub fn short_label(self) -> &'static str {
        match self {
            ColorModeChoice::System => "System",
            ColorModeChoice::Light => "Light",
            ColorModeChoice::Dark => "Dark",
        }
    }
}

/// The mode as a picker whose rows are their `Display` text shows it: its
/// [`ColorModeChoice::short_label`].
impl std::fmt::Display for ColorModeChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.short_label())
    }
}

/// The colour mode `--variant` names: `light`, `dark`, or `system`, which
/// follows the OS.
pub fn color_mode(variant: &str) -> Result<ColorModeChoice, String> {
    match variant {
        "light" => Ok(ColorModeChoice::Light),
        "dark" => Ok(ColorModeChoice::Dark),
        "system" => Ok(ColorModeChoice::System),
        other => Err(format!("--variant {other}: not light, dark or system")),
    }
}

/// The theme `--theme` names: the desktop's own (`default`), or a preset.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ThemeKey {
    Default,
    Preset(String),
}

/// The theme `--theme` names: `default`, the desktop's own, or a key of
/// `Theme::list_presets_for_platform()`, the presets the preset picker
/// offers. A preset of another platform is not offered, and is reported as
/// one.
pub fn theme_key(name: &str) -> Result<ThemeKey, String> {
    if name == "default" {
        return Ok(ThemeKey::Default);
    }
    let offered = Theme::list_presets_for_platform();
    if offered.iter().any(|info| info.key == name) {
        return Ok(ThemeKey::Preset(name.to_string()));
    }
    let names: Vec<&str> = std::iter::once("default")
        .chain(offered.iter().map(|info| info.key))
        .collect();
    let names = names.join(", ");
    if Theme::list_presets().iter().any(|info| info.key == name) {
        Err(format!(
            "--theme {name}: a preset of another platform; only this platform's presets run \
             here: {names}"
        ))
    } else {
        Err(format!(
            "--theme {name}: no such preset; the presets are: {names}"
        ))
    }
}

/// The icon-theme picker's row `--icon-set` names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IconPick {
    /// `default`: the icons follow the theme again, re-derived at each theme
    /// installed (`native_theme::icons::default_icon_choice`).
    FollowTheme,
    /// A row core's `IconSetChoice` names, kept across theme switches.
    Choice(IconSetChoice),
    /// A row only this toolkit's picker has (gpui's built-in icons), by the
    /// name the showcase passed in `toolkit_rows`.
    Toolkit(String),
}

/// The icon-theme picker's row `--icon-set` names: `default`, `material`,
/// `lucide`, `system` or `freedesktop` (the OS's icon theme), a theme of
/// `installed` (the freedesktop themes the picker lists), or a row of
/// `toolkit_rows`, the rows only this showcase's picker has.
pub fn icon_set_choice(
    name: &str,
    installed: &[String],
    toolkit_rows: &[&str],
) -> Result<IconPick, String> {
    match name {
        "default" => return Ok(IconPick::FollowTheme),
        "material" => return Ok(IconPick::Choice(IconSetChoice::Material)),
        "lucide" => return Ok(IconPick::Choice(IconSetChoice::Lucide)),
        "system" | "freedesktop" => return Ok(IconPick::Choice(IconSetChoice::System)),
        _ => {}
    }
    if installed.iter().any(|theme| theme == name) {
        return Ok(IconPick::Choice(IconSetChoice::Freedesktop(
            name.to_string(),
        )));
    }
    if toolkit_rows.contains(&name) {
        return Ok(IconPick::Toolkit(name.to_string()));
    }
    let words: Vec<&str> = ["default", "material", "lucide", "system", "freedesktop"]
        .into_iter()
        .chain(toolkit_rows.iter().copied())
        .collect();
    let installed = if installed.is_empty() {
        "none are installed".to_string()
    } else {
        format!("the installed icon themes are: {}", installed.join(", "))
    };
    Err(format!(
        "--icon-set {name}: not {} or an installed icon theme; {installed}",
        words.join(", ")
    ))
}

/// The page `--tab` names: one of the showcase's page `keys`.
pub fn page_key<'a>(name: &str, keys: &[&'a str]) -> Result<&'a str, String> {
    keys.iter()
        .copied()
        .find(|key| *key == name)
        .ok_or_else(|| {
            format!(
                "--tab {name}: no such page; the pages are: {}",
                keys.join(", ")
            )
        })
}

/// The menu `--open-menu` names, lower-cased: one of the menu bar's
/// `titles`, lower-cased.
pub fn open_menu(name: &str, titles: &[&str]) -> Result<String, String> {
    let name = name.to_lowercase();
    let menus: Vec<String> = titles.iter().map(|title| title.to_lowercase()).collect();
    if menus.contains(&name) {
        Ok(name)
    } else {
        Err(format!(
            "--open-menu {name}: the menu bar has no such menu; the menus are: {}",
            menus.join(", ")
        ))
    }
}

/// The value of `result`; its error reported on stderr, followed by
/// `; ignored`, and `None`.
pub fn reported<T>(result: Result<T, String>) -> Option<T> {
    result.map_err(|error| eprintln!("{error}; ignored")).ok()
}

/// Whether `--screenshot` can capture the window with its frame: on macOS
/// and Windows ([`crate::capture::FrameCapture`]). Elsewhere a toolkit's own
/// capture is its content alone, so the run is refused, naming the script
/// that captures the `kind`'s window there.
pub fn screenshot_support(kind: Kind) -> Result<(), String> {
    if cfg!(any(target_os = "macos", target_os = "windows")) {
        Ok(())
    } else {
        Err(format!(
            "--screenshot captures the window with its frame on macOS and Windows only; on \
             Linux, {} captures it with spectacle",
            kind.capture_script()
        ))
    }
}

/// `ERROR: <message>` on stderr, and exit with 1: a bad `--pointer`, a
/// refused `--screenshot`, a failed capture, dump write or window open.
#[allow(
    clippy::exit,
    reason = "the showcases' one exit with an error, before or instead of their window"
)]
pub fn refuse(message: &str) -> ! {
    eprintln!("ERROR: {message}");
    std::process::exit(1)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    fn parse(argv: &[&str]) -> CliArgs {
        CliArgs::parse(argv.iter().map(|arg| (*arg).to_string()))
    }

    /// Every flag lands in its field; `--variant`, `--tab` and `--open-menu`
    /// are lower-cased, a theme, icon-set, screenshot or dump name keeps its
    /// case, and an unknown argument is ignored.
    #[test]
    fn the_parser_reads_every_flag() {
        let cli = parse(&[
            "--theme",
            "KDE-Breeze",
            "--variant",
            "DARK",
            "--tab",
            "Buttons",
            "--icon-set",
            "Breeze-Dark",
            "--screenshot",
            "Out.png",
            "--capture",
            "--pointer",
            "10,20",
            "--press",
            "--dump-layout",
            "Dump.json",
            "--open-menu",
            "Theme",
            "--no-such-flag",
        ]);
        assert_eq!(
            cli,
            CliArgs {
                theme: Some("KDE-Breeze".into()),
                variant: Some("dark".into()),
                tab: Some("buttons".into()),
                icon_set: Some("Breeze-Dark".into()),
                screenshot: Some("Out.png".into()),
                capture: true,
                pointer: Some((10, 20)),
                bad_pointer: None,
                press: true,
                dump_layout: Some("Dump.json".into()),
                open_menu: Some("theme".into()),
            }
        );
        assert_eq!(parse(&[]), CliArgs::default());
    }

    /// `parse` takes the arguments after the program name; a program name
    /// left at the front is an unknown argument, not a flag's value.
    #[test]
    fn a_leading_program_name_is_no_flag_value() {
        let argv = ["--theme", "adwaita", "--capture"];
        let with_name = parse(&["showcase-gpui", "--theme", "adwaita", "--capture"]);
        assert_eq!(with_name, parse(&argv));
        assert_eq!(with_name.theme.as_deref(), Some("adwaita"));
    }

    /// `--pointer X,Y` is two whole logical pixels; anything else holds no
    /// pointer and is kept to be reported, a missing value included.
    #[test]
    fn the_pointer_flag_is_a_point() {
        assert_eq!(parse_point("349,142"), Some((349, 142)));
        assert_eq!(parse_point(" 349 , 142 "), Some((349, 142)));
        for bad in ["349", "x,1", "1,-2", ""] {
            assert_eq!(parse_point(bad), None, "{bad:?}");
            let cli = parse(&["--pointer", bad]);
            assert_eq!(cli.pointer, None, "{bad:?}");
            assert_eq!(cli.bad_pointer.as_deref(), Some(bad), "{bad:?}");
        }
        assert_eq!(parse(&["--pointer"]).bad_pointer.as_deref(), Some(""));
        let held = parse(&["--pointer", "349,175", "--press"]);
        assert_eq!((held.pointer, held.press), (Some((349, 175)), true));
        assert_eq!(held.bad_pointer, None);
    }

    /// The pointer is shielded in a capture and a layout dump without a held
    /// pointer; the OS is followed only by a run that neither captures nor
    /// dumps; a dump quits only when nothing captures.
    #[test]
    fn the_run_predicates_follow_the_flags() {
        // The flags, whether they capture, whether they dump.
        let cases: [(&[&str], bool, bool); 4] = [
            (&[], false, false),
            (&["--capture"], true, false),
            (&["--screenshot", "p.png"], true, false),
            (&["--dump-layout", "f.json"], false, true),
        ];
        for (argv, capturing, dumping) in cases {
            for pointer in [false, true] {
                let mut argv = argv.to_vec();
                if pointer {
                    argv.extend(["--pointer", "1,2"]);
                }
                let cli = parse(&argv);
                assert_eq!(cli.capturing(), capturing, "{argv:?}");
                assert_eq!(
                    cli.shields_pointer(),
                    (capturing || dumping) && !pointer,
                    "{argv:?}: shields_pointer"
                );
                assert_eq!(
                    cli.follows_os(),
                    !capturing && !dumping,
                    "{argv:?}: follows_os"
                );
                assert_eq!(cli.quits_after_dump(), !capturing, "{argv:?}");
            }
        }
        assert!(!parse(&["--dump-layout", "f", "--screenshot", "p"]).quits_after_dump());
        assert!(!parse(&["--dump-layout", "f", "--capture"]).quits_after_dump());
        assert!(parse(&["--dump-layout", "f"]).quits_after_dump());
    }

    /// A run quits once every file it asked for is written, in either order,
    /// and never under `--capture`; a file it asked for is pending until it
    /// is written.
    #[test]
    fn a_run_quits_once_every_file_it_asked_for_is_written() {
        let both = ["--dump-layout", "f", "--screenshot", "p"];
        for order in [
            [Output::Dump, Output::Screenshot],
            [Output::Screenshot, Output::Dump],
        ] {
            let mut outputs = Outputs::new(&parse(&both));
            assert!(
                !outputs.written(order[0]),
                "{order:?}: quit after the first"
            );
            assert!(outputs.written(order[1]), "{order:?}: no quit after both");
        }
        for (argv, output) in [
            (&["--dump-layout", "f"][..], Output::Dump),
            (&["--screenshot", "p"][..], Output::Screenshot),
        ] {
            assert!(
                Outputs::new(&parse(argv)).written(output),
                "{argv:?}: no quit after its one file"
            );
        }
        let mut pending = Outputs::new(&parse(&both));
        assert!(pending.pending(Output::Dump) && pending.pending(Output::Screenshot));
        let _ = pending.written(Output::Dump);
        assert!(!pending.pending(Output::Dump) && pending.pending(Output::Screenshot));
        let none = Outputs::new(&parse(&[]));
        assert!(!none.pending(Output::Dump) && !none.pending(Output::Screenshot));
        let mut kept = Outputs::new(&parse(&["--dump-layout", "f", "--capture"]));
        assert!(
            !kept.written(Output::Dump),
            "--capture's window was not kept"
        );
    }

    /// A file that is not written is reported with its flag and path.
    #[test]
    fn a_failed_file_names_its_flag_and_path() {
        assert_eq!(
            Output::Screenshot.failure("p.png", "the frame check failed"),
            "--screenshot p.png: the frame check failed"
        );
        assert_eq!(
            Output::Dump.failure("f.json", "denied"),
            "--dump-layout f.json: denied"
        );
    }

    /// `--variant` takes `light`, `dark` and `system`, and names the three
    /// in its error.
    #[test]
    fn the_variant_flag_takes_the_three_modes() {
        assert_eq!(color_mode("light"), Ok(ColorModeChoice::Light));
        assert_eq!(color_mode("dark"), Ok(ColorModeChoice::Dark));
        assert_eq!(color_mode("system"), Ok(ColorModeChoice::System));
        assert_eq!(
            color_mode("dim"),
            Err("--variant dim: not light, dark or system".to_string())
        );
        let labels: Vec<&str> = ColorModeChoice::ALL
            .iter()
            .map(|mode| mode.short_label())
            .collect();
        assert_eq!(labels, ["System", "Light", "Dark"]);
        for mode in ColorModeChoice::ALL {
            assert_eq!(mode.to_string(), mode.short_label());
        }
        assert!(ColorModeChoice::Dark.is_dark());
        assert!(!ColorModeChoice::Light.is_dark());
        let system = if native_theme::detect::system_is_dark() {
            "System (Dark)"
        } else {
            "System (Light)"
        };
        assert_eq!(ColorModeChoice::System.label(), system);
        assert_eq!(ColorModeChoice::Light.label(), "Light");
        assert_eq!(ColorModeChoice::Dark.label(), "Dark");
    }

    /// `--theme` takes `default` and this platform's presets; a preset of
    /// another platform and an unknown name each get their own text, which
    /// lists what is offered.
    #[test]
    fn the_theme_flag_takes_this_platforms_presets() {
        assert_eq!(theme_key("default"), Ok(ThemeKey::Default));
        let offered = Theme::list_presets_for_platform();
        for info in &offered {
            assert_eq!(
                theme_key(info.key),
                Ok(ThemeKey::Preset(info.key.to_string()))
            );
        }
        let names: Vec<&str> = std::iter::once("default")
            .chain(offered.iter().map(|info| info.key))
            .collect();
        let names = names.join(", ");
        let foreign = Theme::list_presets()
            .iter()
            .find(|info| !offered.iter().any(|o| o.key == info.key))
            .expect("some bundled preset is meant for another platform");
        assert_eq!(
            theme_key(foreign.key),
            Err(format!(
                "--theme {}: a preset of another platform; only this platform's presets run \
                 here: {names}",
                foreign.key
            ))
        );
        assert_eq!(
            theme_key("no-such-preset"),
            Err(format!(
                "--theme no-such-preset: no such preset; the presets are: {names}"
            ))
        );
    }

    /// `--icon-set` takes `default`, the bundled sets, `system` and
    /// `freedesktop`, an installed theme and a toolkit row; its error names
    /// every word it takes and the installed themes, or that none are
    /// installed.
    #[test]
    fn the_icon_set_flag_takes_what_the_picker_offers() {
        let installed = vec!["breeze".to_string(), "Adwaita".to_string()];
        let toolkit = ["gpui-builtin"];
        let pick = |name: &str| icon_set_choice(name, &installed, &toolkit);
        assert_eq!(pick("default"), Ok(IconPick::FollowTheme));
        assert_eq!(
            pick("material"),
            Ok(IconPick::Choice(IconSetChoice::Material))
        );
        assert_eq!(pick("lucide"), Ok(IconPick::Choice(IconSetChoice::Lucide)));
        assert_eq!(pick("system"), Ok(IconPick::Choice(IconSetChoice::System)));
        assert_eq!(
            pick("freedesktop"),
            Ok(IconPick::Choice(IconSetChoice::System))
        );
        assert_eq!(
            pick("Adwaita"),
            Ok(IconPick::Choice(IconSetChoice::Freedesktop(
                "Adwaita".into()
            )))
        );
        assert_eq!(
            pick("gpui-builtin"),
            Ok(IconPick::Toolkit("gpui-builtin".into()))
        );
        assert_eq!(
            pick("adwaita"),
            Err(
                "--icon-set adwaita: not default, material, lucide, system, freedesktop, \
                 gpui-builtin or an installed icon theme; the installed icon themes are: \
                 breeze, Adwaita"
                    .to_string()
            )
        );
        assert_eq!(
            icon_set_choice("gpui-builtin", &installed, &[]).ok(),
            None,
            "a toolkit row another showcase passed is taken"
        );
    }

    /// Where no freedesktop icon theme is installed -- off Linux, always --
    /// `--icon-set` says so, rather than end its report on an empty list.
    #[test]
    fn the_icon_flags_say_when_no_icon_theme_is_installed() {
        let error = icon_set_choice("no-such-icon-theme", &[], &[]).err();
        let Some(error) = error else {
            panic!("an icon theme was taken with none installed");
        };
        assert!(error.ends_with("; none are installed"), "{error}");
    }

    /// `--tab` takes each of the showcase's page keys; another name is
    /// reported with the keys it could have been.
    #[test]
    fn the_tab_flag_names_every_page() {
        let keys = ["basic", "buttons", "inputs", "theme-map"];
        for key in keys {
            assert_eq!(page_key(key, &keys), Ok(key));
        }
        assert_eq!(
            page_key("text-inputs", &keys),
            Err(
                "--tab text-inputs: no such page; the pages are: basic, buttons, inputs, \
                 theme-map"
                    .to_string()
            )
        );
    }

    /// `--open-menu` takes a menu bar title in any case, and gives it
    /// lower-cased; another name is reported with the menus.
    #[test]
    fn the_open_menu_flag_names_a_menu() {
        let titles = ["File", "View", "Theme"];
        assert_eq!(open_menu("theme", &titles), Ok("theme".to_string()));
        assert_eq!(open_menu("THEME", &titles), Ok("theme".to_string()));
        assert_eq!(
            open_menu("help", &titles),
            Err(
                "--open-menu help: the menu bar has no such menu; the menus are: file, view, \
                 theme"
                    .to_string()
            )
        );
    }

    /// A reported error is gone, a value kept.
    #[test]
    fn a_reported_error_is_ignored() {
        assert_eq!(reported(Ok::<_, String>(3)), Some(3));
        assert_eq!(reported::<u8>(Err("--tab x: no such page".into())), None);
    }

    /// `--screenshot` runs on macOS and Windows; elsewhere it is refused,
    /// naming the kind's capture script.
    #[test]
    fn the_screenshot_flag_runs_where_the_frame_can_be_captured() {
        for kind in [Kind::Gpui, Kind::Iced, Kind::Egui] {
            let support = screenshot_support(kind);
            if cfg!(any(target_os = "macos", target_os = "windows")) {
                assert_eq!(support, Ok(()));
            } else {
                assert_eq!(
                    support,
                    Err(format!(
                        "--screenshot captures the window with its frame on macOS and Windows \
                         only; on Linux, {} captures it with spectacle",
                        kind.capture_script()
                    ))
                );
            }
        }
    }
}
