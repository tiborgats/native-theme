//! The KDE reader's tests: the KConfig cascade, the reading built from
//! fixture files, and the reading over the presets.

use super::*;

#[test]
fn create_kde_parser_is_case_sensitive() {
    let mut ini = create_kde_parser();
    ini.read("[Colors:View]\nBackgroundNormal=255,255,255\n".to_string())
        .unwrap();
    assert!(ini.get("Colors:View", "BackgroundNormal").is_some());
}

#[test]
fn create_kde_parser_preserves_section_colons() {
    let mut ini = create_kde_parser();
    ini.read("[Colors:Window]\nForegroundNormal=0,0,0\n".to_string())
        .unwrap();
    assert!(ini.get("Colors:Window", "ForegroundNormal").is_some());
}

#[test]
fn create_kde_parser_equals_only_delimiter() {
    let mut ini = create_kde_parser();
    ini.read("[Test]\nsome:value=actual\n".to_string()).unwrap();
    assert_eq!(ini.get("Test", "some:value"), Some("actual".to_string()));
}

/// `content` read as KConfig reads it ([`read_kconfig`]).
fn kconfig(content: &str) -> Ini {
    let mut ini = create_kde_parser();
    read_kconfig(&mut ini, content);
    ini
}

/// KConfig skips a group header without `]` and keeps the group before
/// it; the rest of the file is still read.
#[test]
fn a_stray_group_line_is_skipped() {
    let ini =
        kconfig("[Colors:Window\nBackgroundNormal=49,54,59\n\n[General]\nColorScheme=BreezeDark\n");
    assert_eq!(
        ini.get("General", "ColorScheme").as_deref(),
        Some("BreezeDark")
    );
}

/// KConfig skips an entry with an empty key.
#[test]
fn a_line_with_an_empty_key_is_skipped() {
    let ini = kconfig("[General]\n=stray\nColorScheme=BreezeDark\n");
    assert_eq!(
        ini.get("General", "ColorScheme").as_deref(),
        Some("BreezeDark")
    );
}

/// KConfig has no inline comments: a `#` in a value is part of it.
#[test]
fn a_hash_in_a_value_is_kept() {
    let ini = kconfig("[General]\nColorScheme=My#Scheme\n");
    assert_eq!(
        ini.get("General", "ColorScheme").as_deref(),
        Some("My#Scheme")
    );
}

// === KConfig cascade ===

/// A fresh directory for one test's kdeglobals files, removed on drop.
struct TempConfigDir(std::path::PathBuf);

impl std::ops::Deref for TempConfigDir {
    type Target = std::path::Path;

    fn deref(&self) -> &std::path::Path {
        &self.0
    }
}

impl Drop for TempConfigDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn temp_config_dir(tag: &str) -> TempConfigDir {
    let dir = std::env::temp_dir().join(format!(
        "native-theme-kdeglobals-{}-{tag}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("kdedefaults")).unwrap();
    TempConfigDir(dir)
}

fn paths(paths: &[&str]) -> Vec<std::path::PathBuf> {
    paths.iter().map(std::path::PathBuf::from).collect()
}

#[test]
fn kdeglobals_files_follow_plasmas_config_dirs() {
    let files = kdeglobals_files_pure(
        Some("/home/u/.config/kdedefaults:/etc/xdg"),
        Some("/home/u/.config/kdeglobals".into()),
    );
    assert_eq!(
        files,
        paths(&[
            "/etc/xdg/kdeglobals",
            "/home/u/.config/kdedefaults/kdeglobals",
            "/home/u/.config/kdeglobals",
        ])
    );
}

/// Without `$XDG_CONFIG_DIRS`, or with only relative entries, the config
/// dirs are `/etc/xdg`, and `kdedefaults` is read as Plasma adds it.
#[test]
fn kdeglobals_files_default_to_etc_xdg_and_kdedefaults() {
    let expected = paths(&[
        "/etc/xdg/kdeglobals",
        "/home/u/.config/kdedefaults/kdeglobals",
        "/home/u/.config/kdeglobals",
    ]);
    for config_dirs in [None, Some(""), Some("relative:")] {
        let files = kdeglobals_files_pure(config_dirs, Some("/home/u/.config/kdeglobals".into()));
        assert_eq!(files, expected, "{config_dirs:?}");
    }
}

#[test]
fn kdeglobals_files_read_the_config_dirs_last_first_once_each() {
    let files = kdeglobals_files_pure(
        Some("/opt/a:/opt/b:/opt/a"),
        Some("/home/u/.config/kdeglobals".into()),
    );
    assert_eq!(
        files,
        paths(&[
            "/opt/b/kdeglobals",
            "/opt/a/kdeglobals",
            "/home/u/.config/kdedefaults/kdeglobals",
            "/home/u/.config/kdeglobals",
        ])
    );
}

/// Without a config directory there is no user file and no
/// `kdedefaults` beside it: only the config dirs' files.
#[test]
fn kdeglobals_files_without_a_config_directory_are_the_config_dirs() {
    assert_eq!(
        kdeglobals_files_pure(None, None),
        paths(&["/etc/xdg/kdeglobals"])
    );
    assert_eq!(
        kdeglobals_files_pure(Some("/opt/a:/opt/b"), None),
        paths(&["/opt/b/kdeglobals", "/opt/a/kdeglobals"])
    );
}

/// A ColorScheme set only in `kdedefaults/kdeglobals`, as Plasma 6 writes
/// it, names the theme; each file overrides the keys of the ones before.
#[test]
fn the_scheme_name_and_icon_theme_cascade_from_kdedefaults() {
    let dir = temp_config_dir("cascade");
    std::fs::create_dir_all(dir.join("xdg")).unwrap();
    std::fs::write(
        dir.join("xdg/kdeglobals"),
        "[General]\nColorScheme=SystemScheme\n\n[Icons]\nTheme=breeze\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("kdedefaults/kdeglobals"),
        "[General]\nColorScheme=BreezeDark\n\n[Icons]\nTheme=breeze-dark\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("kdeglobals"),
        "[Colors:Window]\nBackgroundNormal=49,54,59\n",
    )
    .unwrap();
    let files = kdeglobals_files_pure(
        Some(dir.join("xdg").to_str().unwrap()),
        Some(dir.join("kdeglobals")),
    );
    let ini = read_kconfig_files(&files).expect("the files are readable");
    assert_eq!(
        ini.get("General", "ColorScheme").as_deref(),
        Some("BreezeDark")
    );
    assert_eq!(
        ini.get("Colors:Window", "BackgroundNormal").as_deref(),
        Some("49,54,59")
    );
    assert_eq!(
        kconfig_files_icon_theme(&files),
        Ok("breeze-dark".to_string())
    );
}

/// The icon theme `detect` reports is the cascade's: a
/// file without the key leaves the one before it; the failures name the
/// files.
#[test]
fn the_detected_icon_theme_is_the_cascades() {
    let dir = temp_config_dir("icon-theme");
    let files = [dir.join("kdedefaults/kdeglobals"), dir.join("kdeglobals")];
    let failure = kconfig_files_icon_theme(&files).unwrap_err();
    assert!(failure.contains("no file is readable"), "got: {failure}");
    assert!(failure.contains("kdedefaults/kdeglobals"), "got: {failure}");

    std::fs::write(dir.join("kdeglobals"), "[General]\nColorScheme=Custom\n").unwrap();
    let failure = kconfig_files_icon_theme(&files).unwrap_err();
    assert!(
        failure.contains("`[Icons] Theme` is absent or empty"),
        "got: {failure}"
    );

    std::fs::write(
        dir.join("kdedefaults/kdeglobals"),
        "[Icons]\nTheme=breeze-dark\n",
    )
    .unwrap();
    assert_eq!(
        kconfig_files_icon_theme(&files),
        Ok("breeze-dark".to_string())
    );
}

/// A missing file, here the user's, reads as empty; with no file at all
/// there is nothing.
#[test]
fn a_missing_kdeglobals_file_reads_as_empty() {
    let dir = temp_config_dir("missing");
    std::fs::write(
        dir.join("kdeglobals"),
        "[General]\nColorScheme=BreezeDark\n",
    )
    .unwrap();
    let ini = read_kconfig_files(&[dir.join("kdeglobals"), dir.join("absent")])
        .expect("one file is readable");
    assert_eq!(
        ini.get("General", "ColorScheme").as_deref(),
        Some("BreezeDark")
    );
    assert!(read_kconfig_files(&[dir.join("absent")]).is_none());
}

/// A byte that is not UTF-8 is replaced; the rest of the file is read.
#[test]
fn a_non_utf8_byte_keeps_the_theme() {
    let dir = temp_config_dir("non-utf8");
    std::fs::write(
        dir.join("kdeglobals"),
        b"[General]\nColorScheme=Caf\xe9\n\n[Icons]\nTheme=breeze-dark\n",
    )
    .unwrap();
    let files = [dir.join("kdeglobals")];
    let ini = read_kconfig_files(&files).expect("the file is readable");
    assert_eq!(
        ini.get("General", "ColorScheme").as_deref(),
        Some("Caf\u{fffd}")
    );
    assert_eq!(
        kconfig_files_icon_theme(&files),
        Ok("breeze-dark".to_string())
    );
}

/// KConfig has no `;` comments: a `;` in a value is part of it, and a
/// line starting with `;` is an entry.
#[test]
fn a_semicolon_is_not_a_comment() {
    let mut ini = create_kde_parser();
    read_kconfig(
        &mut ini,
        "[General]\n;ColorScheme=Hidden\nColorScheme=My;Scheme\n",
    );
    assert_eq!(
        ini.get("General", "ColorScheme").as_deref(),
        Some("My;Scheme")
    );
    assert_eq!(
        ini.get("General", ";ColorScheme").as_deref(),
        Some("Hidden")
    );
}

// === The reading ===
//
// Every colour, font and number in these fixtures is synthetic: the tests
// check where the reader puts what it reads, not the platform's values.

/// `content` read as KConfig reads it ([`read_kconfig`]).
pub(crate) fn ini(content: &str) -> Ini {
    let mut ini = create_kde_parser();
    read_kconfig(&mut ini, content);
    ini
}

/// The synthetic colour `r,g,b` of the fixtures.
pub(crate) fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: f32::from(r) / 255.0,
        g: f32::from(g) / 255.0,
        b: f32::from(b) / 255.0,
        a: 1.0,
    }
}

/// A dark kdeglobals: every colour group but `Header`, `[Inactive]` for
/// `View` alone, no `[WM]`, the effects and contrast, the fonts, the icon
/// theme and the animation factor (synthetic values).
pub(crate) const KDEGLOBALS_DARK: &str = "\
[General]
ColorScheme=Synthetic
font=Synthetic Sans,11,-1,5,700,0,0,0,0,0,0,0,0,0,0,1
fixed=Synthetic Mono,9,-1,5,400,0,0,0,0,0,0,0,0,0,0,1
smallestReadableFont=Synthetic Sans,7,-1,5,400,1,0,0,0,0,0,0,0,0,0,1
toolBarFont=Synthetic Sans,8,-1,5,400,0,0,0,0,0,0,0,0,0,0,1

[KDE]
AnimationDurationFactor=0.5
contrast=3
frameContrast=0.25

[Icons]
Theme=synthetic-icons

[ColorEffects:Disabled]
Enable=false
IntensityEffect=2
IntensityAmount=0.15
Color=1,2,3

[ColorEffects:Inactive]
ChangeSelectionColor=false
Enable=true
ContrastEffect=2
ContrastAmount=0.3

[Colors:View]
BackgroundNormal=10,11,12
ForegroundNormal=200,201,202
DecorationFocus=13,14,15

[Colors:View][Inactive]
BackgroundNormal=16,17,18

[Colors:Window]
BackgroundNormal=20,21,22
ForegroundNormal=210,211,212

[Colors:Button]
BackgroundNormal=30,31,32

[Colors:Selection]
BackgroundNormal=40,41,42
ForegroundNormal=220,221,222

[Colors:Tooltip]
BackgroundNormal=50,51,52

[Colors:Complementary]
BackgroundNormal=60,61,62
";

/// A scheme file: `View` with an `[Inactive]` group lacking a key, `Header`
/// with a key unset, `Window`, `[WM]` with one key, a tint factor and the
/// title-bar option (synthetic values).
pub(crate) const SCHEME_FILE: &str = "\
[General]
TintFactor=0.4
accentActiveTitlebar=true

[Colors:View]
BackgroundNormal=110,111,112
ForegroundNormal=113,114,115

[Colors:View][Inactive]
BackgroundNormal=116,117,118

[Colors:Window]
BackgroundNormal=120,121,122
ForegroundNormal=123,124,125

[Colors:Header]
BackgroundNormal=130,131,132

[WM]
activeBackground=140,141,142
";

/// The fixtures' data: kdeglobals, the scheme file, no font DPI, no icon
/// theme index.
pub(crate) fn data(kdeglobals: &str, scheme_file: SchemeFile) -> KdeData {
    KdeData {
        kdeglobals: ini(kdeglobals),
        scheme_file,
        font_dpi: None,
        icon_index: None,
    }
}

/// The reading of `data` under `kde-breeze`.
fn reading(data: &KdeData) -> Reading {
    build_reading(data, PresetId::KdeBreeze)
}

/// `kde-breeze`'s part of `layer`.
fn own(layer: &Layer) -> &Bases {
    let (id, bases) = layer.for_preset.first().unwrap();
    assert_eq!(*id, PresetId::KdeBreeze);
    bases
}

/// The input `path` in `bases`, if stated.
fn input_value<'a>(bases: &'a Bases, path: &str) -> Option<&'a InputValue> {
    let name = InputName::new(path).unwrap();
    bases
        .inputs
        .entries
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, value)| value)
}

/// The input `path`'s stated colour at `scheme` in `bases`.
fn input_color(bases: &Bases, path: &str, scheme: Scheme) -> Option<Color> {
    match input_value(bases, path)?.per_scheme.get(scheme)? {
        Value::Stated(color) => Some(*color),
        _ => None,
    }
}

/// The input `path`'s expression at `scheme` in `bases`, as written.
fn input_expr(bases: &Bases, path: &str, scheme: Scheme) -> Option<Expr> {
    match input_value(bases, path)?.per_scheme.get(scheme)? {
        Value::Expr(expr) => Some(expr.clone()),
        _ => None,
    }
}

#[test]
fn parse_color_reads_three_or_four_channels() {
    assert_eq!(parse_color("61,174,233"), Some(rgb(61, 174, 233)));
    assert_eq!(parse_color("0, 0, 0"), Some(rgb(0, 0, 0)));
    assert_eq!(parse_color("255,255,255"), Some(rgb(255, 255, 255)));
    assert_eq!(parse_color("0,0,0,0"), Some(TRANSPARENT));
    for refused in ["invalid", "1,2", "1,2,3,4,5", "256,0,0", "", "1,2,-3"] {
        assert_eq!(parse_color(refused), None, "{refused:?}");
    }
}

/// The scheme is dark where the window background is dark, else light,
/// and light without one (0.7.0's `is_dark_theme` tests).
#[test]
fn the_window_background_decides_the_scheme() {
    assert_eq!(
        scheme(&ini("[Colors:Window]\nBackgroundNormal=20,22,24\n")),
        Scheme::Dark
    );
    assert_eq!(
        scheme(&ini("[Colors:Window]\nBackgroundNormal=239,240,241\n")),
        Scheme::Light
    );
    assert_eq!(scheme(&ini("")), Scheme::Light);
    assert_eq!(
        scheme(&ini("[Colors:View]\nBackgroundNormal=0,0,0\n")),
        Scheme::Light
    );
}

/// A factor of 0 turns animations off and asks for reduced motion; another
/// leaves both; the factor is the speed factor (K3.30, K3.90).
#[test]
fn the_animation_factor_sets_the_motion() {
    let off = reading(&data(
        "[KDE]\nAnimationDurationFactor=0\n",
        SchemeFile::Unnamed,
    ));
    assert!(off.state.animations_off && off.state.reduced_motion);
    let on = reading(&data(KDEGLOBALS_DARK, SchemeFile::Unnamed));
    assert!(!on.state.animations_off && !on.state.reduced_motion);
    let motion = own(&on.layer).tokens.motion.as_ref().unwrap();
    assert_eq!(motion.speed_factor, Some(Value::Stated(0.5)));
    let unset = reading(&data("", SchemeFile::Unnamed));
    assert!(!unset.state.animations_off && !unset.state.reduced_motion);
    assert!(own(&unset.layer).tokens.motion.is_none());
}

/// The state: the scheme read, the font DPI given, text unscaled; nothing
/// else the KDE reader reads at the slice.
#[test]
fn the_state_is_what_kdeglobals_gives() {
    let mut data = data(KDEGLOBALS_DARK, SchemeFile::Unnamed);
    data.font_dpi = Some(120.0);
    let state = reading(&data).state;
    assert_eq!(state.scheme, Scheme::Dark);
    assert_eq!(state.font_dpi, Some(120.0));
    assert_eq!(state.text_scaling_factor, UNSCALED);
    assert_eq!(state.contrast, None);
    assert!(!state.overlay_scrollbars && !state.reduced_transparency);
}

/// Without `AccentColor` the common accent is the `Selection` background at
/// the scheme read, and the own part's `#00000000` there (S9.3).
#[test]
fn without_an_accent_color_the_accent_is_the_selection_background() {
    let reading = reading(&data(KDEGLOBALS_DARK, SchemeFile::Unnamed));
    let accent = input_value(&reading.layer.bases, "accent").unwrap();
    assert_eq!(
        accent.per_scheme,
        SchemeMap::one(Scheme::Dark, Value::Stated(rgb(40, 41, 42)))
    );
    assert_eq!(
        input_value(own(&reading.layer), "accent")
            .unwrap()
            .per_scheme,
        SchemeMap::one(Scheme::Dark, Value::Stated(TRANSPARENT))
    );
}

/// A transparent `AccentColor` is no accent, as `applyScheme()` reads it.
#[test]
fn a_transparent_accent_color_is_no_accent() {
    let text = format!("{KDEGLOBALS_DARK}\n[General]\nAccentColor=0,0,0,0\n");
    let reading = reading(&data(&text, SchemeFile::Unnamed));
    assert_eq!(
        input_color(&reading.layer.bases, "accent", Scheme::Dark),
        Some(rgb(40, 41, 42))
    );
    assert_eq!(
        input_color(own(&reading.layer), "accent", Scheme::Dark),
        Some(TRANSPARENT)
    );
}

/// An `AccentColor` the user chose is the common accent at both schemes,
/// and the own part states none (S9.3).
#[test]
fn an_accent_color_is_the_accent_at_both_schemes() {
    let text = format!("{KDEGLOBALS_DARK}\n[General]\nAccentColor=1,2,3\n");
    let reading = reading(&data(&text, SchemeFile::Unnamed));
    let accent = Value::Stated(rgb(1, 2, 3));
    assert_eq!(
        input_value(&reading.layer.bases, "accent")
            .unwrap()
            .per_scheme,
        SchemeMap::new(accent.clone(), accent)
    );
    assert!(input_value(own(&reading.layer), "accent").is_none());
}

/// The results: each group's keys as its set's roles under the scheme
/// read, `Header`'s from `[Colors:Window]` where kdeglobals has no `Header`
/// group, nothing at the other scheme.
#[test]
fn the_results_are_the_colour_groups_under_the_scheme_read() {
    let reading = reading(&data(KDEGLOBALS_DARK, SchemeFile::Unnamed));
    let colors = &own(&reading.layer).colors;
    assert!(colors.get(Scheme::Light).is_none());
    let surfaces = colors.get(Scheme::Dark).unwrap().surfaces.as_ref().unwrap();
    let fields = |set| match surfaces.get(set) {
        Some(BlockPatch::Fields(patch)) => patch.clone(),
        other => panic!("{set:?}: {other:?}"),
    };
    let view = fields(SurfaceSet::View);
    assert_eq!(view.background, Some(Value::Stated(rgb(10, 11, 12))));
    assert_eq!(view.foreground, Some(Value::Stated(rgb(200, 201, 202))));
    assert_eq!(view.decoration_focus, Some(Value::Stated(rgb(13, 14, 15))));
    assert_eq!(view.link, None);
    let header = fields(SurfaceSet::Header);
    assert_eq!(header.background, Some(Value::Stated(rgb(20, 21, 22))));
    assert_eq!(header.foreground, Some(Value::Stated(rgb(210, 211, 212))));
}

/// Where kdeglobals has no `[WM]`, the `[WM]` roles are the references
/// Breeze falls back to; a key it states is its value (K2.11).
#[test]
fn the_wm_roles_fall_back_to_the_selection() {
    let wm = |text: &str| {
        let reading = reading(&data(text, SchemeFile::Unnamed));
        own(&reading.layer)
            .colors
            .get(Scheme::Dark)
            .unwrap()
            .keyed
            .clone()
            .unwrap()
    };
    let fallback = wm(KDEGLOBALS_DARK);
    let expected = [
        ("wm_active_background", "active(role.selection.background)"),
        ("wm_active_foreground", "active(role.selection.foreground)"),
        (
            "wm_inactive_background",
            "role.selection.disabled.background",
        ),
        (
            "wm_inactive_foreground",
            "role.selection.disabled.foreground",
        ),
    ];
    assert_eq!(fallback.len(), expected.len());
    for ((key, value), (name, text)) in fallback.iter().zip(expected) {
        assert_eq!(key.as_str(), name);
        assert_eq!(value, &Value::Expr(Expr::parse(text).unwrap()));
    }
    let stated = wm(&format!(
        "{KDEGLOBALS_DARK}\n[WM]\ninactiveForeground=7,8,9\n"
    ));
    assert_eq!(
        stated.get(3),
        Some(&(
            crate::key!("wm_inactive_foreground"),
            Value::Stated(rgb(7, 8, 9))
        ))
    );
}

/// The inactive input roles: a group's `[Inactive]` keys where it has one
/// (a key it lacks unstated), else its active values (K3.51).
#[test]
fn the_inactive_results_are_the_inactive_groups_else_the_active_ones() {
    let reading = reading(&data(KDEGLOBALS_DARK, SchemeFile::Unnamed));
    let bases = own(&reading.layer);
    let at = |path| input_color(bases, path, Scheme::Dark);
    assert_eq!(at("inactive.view.background_normal"), Some(rgb(16, 17, 18)));
    assert_eq!(at("inactive.view.foreground_normal"), None);
    assert_eq!(
        at("inactive.window.foreground_normal"),
        Some(rgb(210, 211, 212))
    );
    assert_eq!(
        at("inactive.header.background_normal"),
        Some(rgb(20, 21, 22))
    );
    assert_eq!(at("inactive.button.foreground_normal"), None);
}

/// The scheme inputs of the scheme file, under the scheme read: its
/// values, `Header`'s unset key from `Window`, the inactive inputs of a set
/// without `[Inactive]` as references to its active ones, the three lists,
/// and the `[WM]` defaults where it states none (S9.1).
#[test]
fn the_scheme_inputs_are_the_scheme_files() {
    let reading = reading(&data(KDEGLOBALS_DARK, SchemeFile::Read(ini(SCHEME_FILE))));
    assert!(reading.fallback.is_none());
    let bases = own(&reading.layer);
    let dark = Scheme::Dark;
    let at = |path| input_color(bases, path, dark);
    assert_eq!(
        at("kde_scheme.view.background_normal"),
        Some(rgb(110, 111, 112))
    );
    assert_eq!(
        at("kde_scheme.view.inactive.background_normal"),
        Some(rgb(116, 117, 118))
    );
    assert!(input_value(bases, "kde_scheme.view.inactive.foreground_normal").is_none());
    assert!(input_value(bases, "kde_scheme.view.background_alternate").is_none());
    assert_eq!(
        at("kde_scheme.header.background_normal"),
        Some(rgb(130, 131, 132))
    );
    assert_eq!(
        at("kde_scheme.header.foreground_normal"),
        Some(rgb(123, 124, 125))
    );
    assert_eq!(
        input_expr(bases, "kde_scheme.window.inactive.background_normal", dark),
        Some(Expr::parse("input.kde_scheme.window.background_normal").unwrap())
    );
    assert!(input_value(bases, "kde_scheme.button.background_normal").is_none());
    assert_eq!(
        input_expr(bases, "kde_scheme.button.inactive.background_normal", dark),
        Some(Expr::parse("input.kde_scheme.button.background_normal").unwrap())
    );
    assert_eq!(
        at("kde_scheme.wm.active_background"),
        Some(rgb(140, 141, 142))
    );
    assert_eq!(
        input_expr(bases, "kde_scheme.wm.active_foreground", dark),
        Some(Expr::parse("input.kde_scheme.header.foreground_normal").unwrap())
    );
    assert!(matches!(
        input_expr(bases, "kde_scheme.wm.inactive_background", dark),
        Some(Expr::Call(crate::expr::Func::KdeEffects, _))
    ));
    assert!(
        input_value(bases, "kde_scheme.view.background_normal")
            .unwrap()
            .per_scheme
            .get(Scheme::Light)
            .is_none()
    );

    let lists = &bases.inputs;
    assert_eq!(
        lists.kde_groups,
        SchemeMap::one(
            dark,
            vec![SurfaceSet::View, SurfaceSet::Window, SurfaceSet::Header]
        )
    );
    assert_eq!(
        lists.kde_inactive_groups,
        SchemeMap::one(dark, vec![SurfaceSet::View])
    );
    let unset = lists.kde_unset_keys.get(dark).unwrap();
    for path in [
        "kde_scheme.view.background_alternate",
        "kde_scheme.view.inactive.foreground_normal",
        "kde_scheme.header.foreground_normal",
        "kde_scheme.window.decoration_hover",
    ] {
        assert!(unset.contains(&InputName::new(path).unwrap()), "{path}");
    }
    for path in [
        "kde_scheme.view.background_normal",
        "kde_scheme.window.inactive.background_normal",
        "kde_scheme.button.background_normal",
    ] {
        assert!(!unset.contains(&InputName::new(path).unwrap()), "{path}");
    }
    // Two groups of 12 keys, two keys each stated; one `[Inactive]` group
    // with one key stated.
    assert_eq!(unset.len(), 10 + 10 + 11 + 11);
}

/// The scheme's tokens are one rule under the scheme read and `kde-breeze`:
/// the effects kdeglobals states (the disabled ones `none` where its
/// `Enable` is false), the inactive flags, the scheme file's tint and
/// title-bar options, and the contrast as stored (S4.8).
#[test]
fn the_scheme_tokens_are_a_rule_under_the_scheme_read() {
    let reading = reading(&data(KDEGLOBALS_DARK, SchemeFile::Read(ini(SCHEME_FILE))));
    let [rule] = reading.layer.rules.as_slice() else {
        panic!("one rule: {:?}", reading.layer.rules);
    };
    assert_eq!(
        rule.selector.conditions.all_of,
        [
            Condition::Scheme(Scheme::Dark),
            Condition::Preset(PresetId::KdeBreeze)
        ]
    );
    let RuleBody::Tokens(tokens) = &rule.body else {
        panic!("a tokens rule: {rule:?}");
    };
    let Some(BlockPatch::Fields(kde)) = &tokens.kde else {
        panic!("token.kde's fields: {:?}", tokens.kde);
    };
    let disabled = kde.disabled.as_ref().unwrap();
    assert_eq!(disabled.intensity, Some(KdeIntensityEffect::None));
    assert_eq!(disabled.color, Some(KdeColorEffect::None));
    assert_eq!(disabled.contrast, Some(KdeContrastEffect::None));
    assert_eq!(disabled.intensity_amount, Some(Value::Stated(0.15)));
    assert_eq!(disabled.color_color, Some(Value::Stated(rgb(1, 2, 3))));
    assert_eq!(disabled.color_amount, None);
    let inactive = kde.inactive.as_ref().unwrap();
    assert_eq!(inactive.contrast, Some(KdeContrastEffect::Tint));
    assert_eq!(inactive.contrast_amount, Some(Value::Stated(0.3)));
    assert_eq!(inactive.intensity, None);
    assert_eq!(kde.inactive_enabled, Some(Value::Stated(true)));
    assert_eq!(
        kde.inactive_change_selection_color,
        Some(Value::Stated(false))
    );
    assert_eq!(kde.tint_factor, Some(Some(Value::Stated(0.4))));
    assert_eq!(kde.titlebar_is_accent_colored, Some(Value::Stated(true)));
    assert_eq!(
        tokens.params,
        Some(vec![(
            crate::key!("color"),
            vec![
                (crate::key!("contrast"), Value::Stated(Scalar::Number(3.0))),
                (
                    crate::key!("frame_contrast"),
                    Value::Stated(Scalar::Number(0.25))
                ),
            ]
        )])
    );
}

/// Where kdeglobals states no contrast, KColorScheme's defaults; where it
/// states no inactive `Enable`, the inactive effects are off and the
/// selection takes them; with no scheme named, no scheme option is stated.
#[test]
fn unset_scheme_tokens_are_kcolorschemes_defaults() {
    let reading = reading(&data("", SchemeFile::Unnamed));
    let RuleBody::Tokens(tokens) = &reading.layer.rules.first().unwrap().body else {
        panic!("a tokens rule");
    };
    let Some(BlockPatch::Fields(kde)) = &tokens.kde else {
        panic!("token.kde's fields");
    };
    assert_eq!(kde.inactive_enabled, Some(Value::Stated(false)));
    assert_eq!(
        kde.inactive_change_selection_color,
        Some(Value::Stated(true))
    );
    assert_eq!(kde.tint_factor, None);
    assert_eq!(kde.titlebar_is_accent_colored, None);
    assert_eq!(kde.disabled.as_ref().unwrap().intensity, None);
    let params = tokens.params.as_ref().unwrap();
    assert_eq!(
        params.first().unwrap().1,
        vec![
            (
                crate::key!("contrast"),
                Value::Stated(Scalar::Number(DEFAULT_CONTRAST))
            ),
            (
                crate::key!("frame_contrast"),
                Value::Stated(Scalar::Number(DEFAULT_FRAME_CONTRAST))
            ),
        ]
    );
}

/// With no scheme named, the reader states no scheme input: Breeze Light's,
/// the preset's, apply (S9.3).
#[test]
fn an_unnamed_scheme_states_no_scheme_input() {
    let reading = reading(&data(KDEGLOBALS_DARK, SchemeFile::Unnamed));
    let bases = own(&reading.layer);
    assert!(
        bases
            .inputs
            .entries
            .iter()
            .all(|(name, _)| { !name.to_string().starts_with("kde_scheme.") })
    );
    assert_eq!(bases.inputs.kde_groups, SchemeMap::default());
    assert!(reading.fallback.is_none());
}

/// A scheme file that cannot be read: kdeglobals stands in for it, read as
/// the scheme file would be, with no tint and no accent title bar, and the
/// status's fallback names the failure (S9.3).
#[test]
fn an_unreadable_scheme_file_reads_kdeglobals_in_its_place() {
    let text = format!("{KDEGLOBALS_DARK}\n[General]\nTintFactor=0.9\n");
    let reading = reading(&data(
        &text,
        SchemeFile::Unreadable("the colour scheme `Synthetic`: gone".to_string()),
    ));
    assert_eq!(
        reading.fallback,
        Some(ReaderFailure::Platform(
            "the colour scheme `Synthetic`: gone".to_string()
        ))
    );
    let bases = own(&reading.layer);
    assert_eq!(
        input_color(bases, "kde_scheme.view.background_normal", Scheme::Dark),
        Some(rgb(10, 11, 12))
    );
    assert_eq!(
        bases.inputs.kde_inactive_groups,
        SchemeMap::one(Scheme::Dark, vec![SurfaceSet::View])
    );
    let RuleBody::Tokens(tokens) = &reading.layer.rules.first().unwrap().body else {
        panic!("a tokens rule");
    };
    let Some(BlockPatch::Fields(kde)) = &tokens.kde else {
        panic!("token.kde's fields");
    };
    assert_eq!(kde.tint_factor, Some(None));
    assert_eq!(kde.titlebar_is_accent_colored, Some(Value::Stated(false)));
}

/// The fonts: `body`, `monospace` and `caption` in the common bases, their
/// point sizes converted by the reader's DPI, else the preset's 96;
/// `toolbar` under the own part (S9.4, S9.5).
#[test]
fn the_fonts_patch_the_slots_kde_has_a_setting_for() {
    use crate::blocks::text::FontStyle;
    use crate::units::FontSize;

    let fields = |patch: &Option<BlockPatch<crate::blocks::text::TextStyle>>| match patch {
        Some(BlockPatch::Fields(style)) => style.font.clone().unwrap(),
        other => panic!("a font patch: {other:?}"),
    };
    let mut data = data(KDEGLOBALS_DARK, SchemeFile::Unnamed);
    let reading = build_reading(&data, PresetId::KdeBreeze);
    let text = reading.layer.bases.tokens.text.as_ref().unwrap();
    let roles = text.roles.as_ref().unwrap();
    let body = fields(&roles.body);
    assert_eq!(
        body.family,
        Some(Value::Stated("Synthetic Sans".to_string()))
    );
    assert_eq!(body.weight, Some(Value::Stated(700)));
    assert_eq!(body.style, Some(Value::Stated(FontStyle::Normal)));
    // 11 pt at kde-breeze's 96 DPI.
    assert_eq!(
        body.size,
        Some(Value::Stated(FontSize::Px(11.0 * 96.0 / 72.0)))
    );
    assert_eq!(
        fields(&roles.monospace).family,
        Some(Value::Stated("Synthetic Mono".to_string()))
    );
    assert_eq!(
        fields(&roles.caption).style,
        Some(Value::Stated(FontStyle::Italic))
    );
    assert!(roles.title1.is_none());
    let keyed = own(&reading.layer)
        .tokens
        .text
        .as_ref()
        .unwrap()
        .keyed
        .as_ref()
        .unwrap();
    assert_eq!(keyed.len(), 1);
    assert_eq!(keyed.first().unwrap().0, crate::key!("toolbar"));

    data.font_dpi = Some(144.0);
    let reading = build_reading(&data, PresetId::KdeBreeze);
    let roles = reading.layer.bases.tokens.text.unwrap().roles.unwrap();
    assert_eq!(
        fields(&roles.body).size,
        Some(Value::Stated(FontSize::Px(22.0)))
    );
}

/// A font entry Qt refuses patches nothing.
#[test]
fn a_refused_font_entry_patches_nothing() {
    let reading = reading(&data("[General]\nfont=\n", SchemeFile::Unnamed));
    assert!(reading.layer.bases.tokens.text.is_none());
    assert_eq!(font::font_patch(",", Some(96.0)), None);
}

/// The icon theme and its sizes under the own part, each size from its
/// `[Icon Theme]` key; a key absent or no size keeps the preset's.
#[test]
fn the_icon_theme_and_its_sizes() {
    let mut data = data(KDEGLOBALS_DARK, SchemeFile::Unnamed);
    data.icon_index = Some(ini(
        "[Icon Theme]\nToolbarDefault=24\nSmallDefault=18\nPanelDefault=0\nDialogDefault=big\n",
    ));
    let reading = build_reading(&data, PresetId::KdeBreeze);
    let icons = own(&reading.layer).tokens.icons.as_ref().unwrap();
    assert_eq!(
        icons.theme,
        Some(Value::Stated("synthetic-icons".to_string()))
    );
    let sizes = icons.sizes.as_ref().unwrap();
    let size = |patch: &Option<BlockPatch<crate::blocks::icon::IconStyle>>| match patch {
        Some(BlockPatch::Fields(style)) => style.size.clone(),
        None => None,
        other => panic!("{other:?}"),
    };
    assert_eq!(size(&sizes.toolbar), Some(Value::Stated(Px(24.0))));
    assert_eq!(size(&sizes.small), Some(Value::Stated(Px(18.0))));
    assert_eq!(size(&sizes.panel), None);
    assert_eq!(size(&sizes.dialog), None);
    assert_eq!(size(&sizes.large), None);
    assert_eq!(icon_sizes(&ini("[Icon Theme]\n")), None);
}

/// Every expression the reader writes parses.
#[test]
fn the_readers_expressions_parse() {
    let reading = reading(&data("", SchemeFile::Read(ini(""))));
    let bases = own(&reading.layer);
    let exprs = bases
        .inputs
        .entries
        .iter()
        .filter(|(_, value)| matches!(value.per_scheme.get(Scheme::Light), Some(Value::Expr(_))))
        .count();
    // Seven sets' twelve inactive inputs, four `[WM]` inputs.
    assert_eq!(exprs, 7 * 12 + 4);
    let keyed = bases
        .colors
        .get(Scheme::Light)
        .unwrap()
        .keyed
        .as_ref()
        .unwrap();
    assert_eq!(keyed.len(), 4);
}

/// The reader's colour groups are the surface sets `kde-breeze` declares.
#[test]
fn the_colour_groups_are_kde_breezes_sets() {
    let file = crate::toml::layer::read_preset(PresetId::KdeBreeze.source()).unwrap();
    let declared = &file.declarations.surface_sets;
    assert_eq!(declared.len(), COLOR_GROUPS.len());
    for (set, _) in COLOR_GROUPS {
        assert!(declared.contains(&set), "{set:?}");
    }
}

/// The scheme file `ColorScheme` names, in the first data directory that
/// holds it; unnamed, missing or no file name, the reason names it.
#[test]
fn the_scheme_file_is_found_in_the_data_directories() {
    let dir = temp_config_dir("scheme-file");
    let (first, second) = (dir.join("first"), dir.join("second"));
    std::fs::create_dir_all(second.join("color-schemes")).unwrap();
    std::fs::write(
        second.join("color-schemes/Synthetic.colors"),
        "[Colors:View]\nBackgroundNormal=1,2,3\n",
    )
    .unwrap();
    let dirs = [first, second];
    let kdeglobals = |name: &str| ini(&format!("[General]\nColorScheme={name}\n"));
    match read_scheme_file(&kdeglobals("Synthetic"), &dirs) {
        SchemeFile::Read(file) => assert_eq!(
            file.get("Colors:View", "BackgroundNormal").as_deref(),
            Some("1,2,3")
        ),
        _ => panic!("the scheme file is read"),
    }
    assert!(matches!(
        read_scheme_file(&ini(""), &dirs),
        SchemeFile::Unnamed
    ));
    match read_scheme_file(&kdeglobals("Missing"), &dirs) {
        SchemeFile::Unreadable(why) => {
            assert!(why.contains("`Missing`"), "{why}");
            assert!(why.contains("first/color-schemes/Missing.colors"), "{why}");
        }
        _ => panic!("a missing scheme file is unreadable"),
    }
    assert!(matches!(
        read_scheme_file(&kdeglobals("../Synthetic"), &dirs),
        SchemeFile::Unreadable(_)
    ));
}

/// `forceFontDPI` is read from kcmfonts, a `u32` above zero; the font DPI
/// is read on X11 only (0.7.0's `force_font_dpi_is_read_from_kcmfonts`).
#[test]
fn force_font_dpi_is_read_from_kcmfonts() {
    let dir = temp_config_dir("kcmfonts");
    std::fs::write(
        dir.join("kcmfonts"),
        "[General\nstray=1\n\n[General]\nforceFontDPI=120\n",
    )
    .unwrap();
    assert_eq!(
        kcmfonts_key_in(&dir, "General", "forceFontDPI").as_deref(),
        Some("120")
    );
    assert_eq!(force_font_dpi("120"), Some(120.0));
    for unset in ["0", "-1", "96.5", ""] {
        assert_eq!(force_font_dpi(unset), None, "{unset:?}");
    }
    assert_eq!(read_font_dpi(Some("wayland")), None);
    assert_eq!(read_font_dpi(None), None);
}
