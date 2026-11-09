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
        colors: None,
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
    assert_eq!(parse_color("-0,+1,2"), Some(rgb(0, 1, 2)));
    for refused in [
        "invalid",
        "1,2",
        "1,2,3,4,5",
        "256,0,0",
        "",
        "1,2,-3",
        "#010203",
        "1;2;3",
        "1.5,2,3",
    ] {
        assert_eq!(parse_color(refused), None, "{refused:?}");
    }
}

/// The scheme is Plasma's test of the `Window` background: dark where Qt's
/// grey of it is below the threshold, so a mid-grey window 0.7.0's BT.601
/// luminance called light is dark; light where it is unset or no colour,
/// KColorScheme's default window; the `View` background plays no part.
#[test]
fn the_scheme_is_plasmas_test_of_the_window_background() {
    let scheme_of = |text: &str| scheme::scheme(&ini(text));
    assert_eq!(
        scheme_of("[Colors:Window]\nBackgroundNormal=20,22,24\n"),
        Scheme::Dark
    );
    assert_eq!(
        scheme_of("[Colors:Window]\nBackgroundNormal=150,150,150\n"),
        Scheme::Dark
    );
    assert_eq!(
        scheme_of("[Colors:Window]\nBackgroundNormal=239,240,241\n"),
        Scheme::Light
    );
    assert_eq!(scheme_of(""), Scheme::Light);
    assert_eq!(
        scheme_of("[Colors:Window]\nBackgroundNormal=#000000\n"),
        Scheme::Light
    );
    assert_eq!(
        scheme_of("[Colors:View]\nBackgroundNormal=0,0,0\n"),
        Scheme::Light
    );
}

/// A bool entry is false where it is `false`, `no`, `off` or `0` in any
/// case, else true, the empty value included; an unset one is `None`
/// (S4.8's *Unset and empty*).
#[test]
fn a_bool_entry_is_read_as_kconfig_reads_it() {
    let file = ini("[G]\na=false\nb=No\nc=OFF\nd=0\ne=\nf=true\ng=yes\nh=1\ni=anything\n");
    for key in ["a", "b", "c", "d"] {
        assert_eq!(flag(&file, "G", key), Some(false), "{key}");
    }
    for key in ["e", "f", "g", "h", "i"] {
        assert_eq!(flag(&file, "G", key), Some(true), "{key}");
    }
    assert_eq!(flag(&file, "G", "unset"), None);
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
/// read, a key kdeglobals leaves unset KColorScheme's default for the set,
/// `Header`'s from `[Colors:Window]` over `Header`'s own table where
/// kdeglobals has no `Header` group, nothing at the other scheme.
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
    assert_eq!(
        view.link,
        Some(Value::Stated(
            defaults::VIEW.color(KdeSchemeKey::ForegroundLink)
        ))
    );
    assert_eq!(
        view.decoration_hover,
        Some(Value::Stated(
            defaults::VIEW.color(KdeSchemeKey::DecorationHover)
        ))
    );
    let header = fields(SurfaceSet::Header);
    assert_eq!(header.background, Some(Value::Stated(rgb(20, 21, 22))));
    assert_eq!(header.foreground, Some(Value::Stated(rgb(210, 211, 212))));
    assert_eq!(
        header.background_alternate,
        Some(Value::Stated(
            defaults::HEADER.color(KdeSchemeKey::BackgroundAlternate)
        ))
    );
    assert_ne!(
        defaults::HEADER.color(KdeSchemeKey::BackgroundAlternate),
        defaults::WINDOW.color(KdeSchemeKey::BackgroundAlternate)
    );
    for set in [SurfaceSet::Button, SurfaceSet::Complementary] {
        let patch = fields(set);
        assert!(patch.positive.is_some() && patch.decoration_hover.is_some());
    }
}

/// The KDE facts file `name` (`docs/facts/kde/<name>`), parsed.
fn kde_facts(name: &str) -> Vec<toml::Value> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../docs/facts/kde")
        .join(name);
    let text = std::fs::read_to_string(&path).unwrap();
    let table: toml::Table = toml::from_str(&text).unwrap();
    table["fact"].as_array().unwrap().clone()
}

/// The last `{ r, g, b }` row of a table quote, as a colour.
fn last_row(quote: &str) -> Color {
    let row = quote.rsplit('{').find(|part| part.contains('}')).unwrap();
    let channels: Vec<u8> = row
        .split('}')
        .next()
        .unwrap()
        .split(',')
        .map(|channel| channel.trim().parse().unwrap())
        .collect();
    let [r, g, b] = channels.as_slice() else {
        panic!("a row of three: {quote}");
    };
    rgb(*r, *g, *b)
}

/// Every row of KColorScheme's default tables the reader holds is the row
/// its KDE fact cites (`docs/facts/kde/inputs.toml`, each `scheme:light`
/// scheme-input fact's table quote ending at the set's row for its key):
/// all seven sets' twelve keys, `DecorationHover`'s table row included,
/// where Breeze Light's value differs.
#[test]
fn the_default_tables_are_the_rows_the_facts_cite() {
    let mut checked = 0;
    for fact in kde_facts("inputs.toml") {
        let key = fact["key"].as_str().unwrap();
        let light = fact
            .get("when")
            .and_then(|when| when.get("conditions"))
            .is_some_and(|c| c.as_array().unwrap() == &[toml::Value::from("scheme:light")]);
        let parts: Vec<&str> = key.split('.').collect();
        let [_, "kde_scheme", set_name, key_name] = parts.as_slice() else {
            continue;
        };
        let Some((_, _, defaults)) = COLOR_GROUPS
            .iter()
            .find(|(set, _, _)| set.name() == *set_name)
        else {
            continue;
        };
        let Some(key) = KdeSchemeKey::ALL
            .into_iter()
            .find(|k| k.name() == *key_name)
        else {
            continue;
        };
        if !light {
            continue;
        }
        let quote = fact["cite"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|cite| cite["quote"].as_str())
            .find(|quote| quote.starts_with("static const"))
            .unwrap_or_else(|| panic!("{key:?}: no table quote"));
        assert_eq!(
            defaults.color(key),
            last_row(quote),
            "{set_name}.{key_name}"
        );
        checked += 1;
    }
    assert_eq!(checked, 7 * 12);
}

/// Each of KIconTheme's six group defaults the reader holds is the row its
/// `token.icons.sizes` fact cites (`docs/facts/kde/tokens.toml`, the
/// `m_iconGroups` quote ending at the group's row).
#[test]
fn the_icon_size_defaults_are_the_rows_the_facts_cite() {
    let sizes = &defaults::ICON_SIZES;
    let expected = [
        ("token.icons.sizes.toolbar.size", sizes.toolbar),
        (
            "token.icons.sizes.keyed.main_toolbar.size",
            sizes.main_toolbar,
        ),
        ("token.icons.sizes.small.size", sizes.small),
        ("token.icons.sizes.large.size", sizes.desktop),
        ("token.icons.sizes.panel.size", sizes.panel),
        ("token.icons.sizes.dialog.size", sizes.dialog),
    ];
    let facts = kde_facts("tokens.toml");
    for (key, size) in expected {
        let fact = facts
            .iter()
            .find(|fact| fact["key"].as_str() == Some(key))
            .unwrap();
        let quote = fact["cite"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|cite| cite["quote"].as_str())
            .find(|quote| quote.starts_with("std::array<GroupInfo"))
            .unwrap();
        let row = quote.trim_end_matches(',').trim_end_matches('}');
        let cited: u16 = row.rsplit(',').next().unwrap().trim().parse().unwrap();
        assert_eq!(size, cited, "{key}");
    }
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

/// The inactive input roles: a group's `[Inactive]` keys where it has one,
/// a key it lacks the set's default, not its active value; else its active
/// values, KColorScheme's defaults where kdeglobals leaves them unset
/// (K3.51).
#[test]
fn the_inactive_results_are_the_inactive_groups_else_the_active_ones() {
    let reading = reading(&data(KDEGLOBALS_DARK, SchemeFile::Unnamed));
    let bases = own(&reading.layer);
    let at = |path| input_color(bases, path, Scheme::Dark);
    assert_eq!(at("inactive.view.background_normal"), Some(rgb(16, 17, 18)));
    assert_eq!(
        at("inactive.view.foreground_normal"),
        Some(defaults::VIEW.color(KdeSchemeKey::ForegroundNormal))
    );
    assert_eq!(
        at("inactive.window.foreground_normal"),
        Some(rgb(210, 211, 212))
    );
    assert_eq!(
        at("inactive.header.background_normal"),
        Some(rgb(20, 21, 22))
    );
    assert_eq!(
        at("inactive.button.foreground_normal"),
        Some(defaults::BUTTON.color(KdeSchemeKey::ForegroundNormal))
    );
    assert_eq!(
        bases
            .inputs
            .entries
            .iter()
            .filter(|(name, _)| name.to_string().starts_with("inactive."))
            .count(),
        7 * 12
    );
}

/// `Header` without a group of its own reads its inactive input roles from
/// its active values, `Window`'s over `Header`'s table, as KColorScheme
/// reads `Header` with no `[Colors:Header][Inactive]` group, never from
/// `[Colors:Window][Inactive]`.
#[test]
fn header_without_a_group_reads_its_active_values_when_inactive() {
    let text = "[Colors:Window]\nBackgroundNormal=20,21,22\n\n\
                [Colors:Window][Inactive]\nBackgroundNormal=1,1,1\nForegroundNormal=2,2,2\n";
    let reading = reading(&data(text, SchemeFile::Unnamed));
    let bases = own(&reading.layer);
    let at = |path| input_color(bases, path, Scheme::Dark);
    assert_eq!(at("inactive.window.background_normal"), Some(rgb(1, 1, 1)));
    assert_eq!(
        at("inactive.header.background_normal"),
        Some(rgb(20, 21, 22))
    );
    assert_eq!(
        at("inactive.header.foreground_normal"),
        Some(defaults::HEADER.color(KdeSchemeKey::ForegroundNormal))
    );
    assert_eq!(
        at("inactive.window.background_alternate"),
        Some(defaults::WINDOW.color(KdeSchemeKey::BackgroundAlternate))
    );
}

/// The scheme inputs of the scheme file, under the scheme read: its
/// values, `Header`'s unset key from `Window` over `Header`'s table, another
/// unset key, a whole group's included, and a key its `[Inactive]` group
/// lacks KColorScheme's default, the inactive inputs of a set without
/// `[Inactive]` as references to its active ones, the three lists, and the
/// `[WM]` defaults where it states none (S9.1).
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
    assert_eq!(
        at("kde_scheme.view.inactive.foreground_normal"),
        Some(defaults::VIEW.color(KdeSchemeKey::ForegroundNormal))
    );
    assert_eq!(
        at("kde_scheme.view.background_alternate"),
        Some(defaults::VIEW.color(KdeSchemeKey::BackgroundAlternate))
    );
    assert_eq!(
        at("kde_scheme.header.foreground_link"),
        Some(defaults::HEADER.color(KdeSchemeKey::ForegroundLink))
    );
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
    assert_eq!(
        at("kde_scheme.button.background_normal"),
        Some(defaults::BUTTON.color(KdeSchemeKey::BackgroundNormal))
    );
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
    assert_eq!(
        disabled.color_amount,
        Some(Value::Stated(defaults::DISABLED.color_amount))
    );
    let inactive = kde.inactive.as_ref();
    assert_eq!(
        inactive.map(|i| i.contrast),
        Some(Some(KdeContrastEffect::Tint))
    );
    assert_eq!(
        inactive.map(|i| i.contrast_amount.clone()),
        Some(Some(Value::Stated(0.3)))
    );
    assert_eq!(
        inactive.map(|i| i.intensity),
        Some(Some(defaults::INACTIVE.intensity))
    );
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
    for (effects, defaults) in [
        (kde.disabled.as_ref(), &defaults::DISABLED),
        (kde.inactive.as_ref(), &defaults::INACTIVE),
    ] {
        let expected = KdeEffectsPatch {
            intensity: Some(defaults.intensity),
            intensity_amount: Some(Value::Stated(defaults.intensity_amount)),
            color: Some(defaults.color),
            color_amount: Some(Value::Stated(defaults.color_amount)),
            color_color: Some(Value::Stated(defaults.color_color())),
            contrast: Some(defaults.contrast),
            contrast_amount: Some(Value::Stated(defaults.contrast_amount)),
        };
        assert_eq!(effects, Some(&expected));
    }
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

/// The empty values `applyScheme()` writes for the effect keys a scheme
/// file lacks are read as KConfig reads them: an empty `Enable` and
/// `ChangeSelectionColor` true, an empty number or colour the default
/// (S4.8's *Unset and empty*).
#[test]
fn empty_effect_values_are_read_as_kconfig_reads_them() {
    let text = "[ColorEffects:Inactive]\nEnable=\nChangeSelectionColor=\nIntensityEffect=\n\
                IntensityAmount=\nColor=\n\n[ColorEffects:Disabled]\nEnable=\nContrastAmount=\n";
    let reading = reading(&data(text, SchemeFile::Unnamed));
    let RuleBody::Tokens(tokens) = &reading.layer.rules.first().unwrap().body else {
        panic!("a tokens rule");
    };
    let Some(BlockPatch::Fields(kde)) = &tokens.kde else {
        panic!("token.kde's fields");
    };
    assert_eq!(kde.inactive_enabled, Some(Value::Stated(true)));
    assert_eq!(
        kde.inactive_change_selection_color,
        Some(Value::Stated(true))
    );
    let inactive = kde.inactive.as_ref().unwrap();
    assert_eq!(inactive.intensity, Some(defaults::INACTIVE.intensity));
    assert_eq!(
        inactive.intensity_amount,
        Some(Value::Stated(defaults::INACTIVE.intensity_amount))
    );
    assert_eq!(
        inactive.color_color,
        Some(Value::Stated(defaults::INACTIVE.color_color()))
    );
    let disabled = kde.disabled.as_ref().unwrap();
    assert_eq!(disabled.intensity, Some(defaults::DISABLED.intensity));
    assert_eq!(
        disabled.contrast_amount,
        Some(Value::Stated(defaults::DISABLED.contrast_amount))
    );

    let off = reading_tokens_disabled("[ColorEffects:Disabled]\nEnable=No\n");
    assert_eq!(off.intensity, Some(KdeIntensityEffect::None));
    assert_eq!(off.contrast, Some(KdeContrastEffect::None));
}

/// The disabled effects of kdeglobals `text`.
fn reading_tokens_disabled(text: &str) -> KdeEffectsPatch {
    match &reading(&data(text, SchemeFile::Unnamed))
        .layer
        .rules
        .first()
        .map(|rule| &rule.body)
    {
        Some(RuleBody::Tokens(tokens)) => match &tokens.kde {
            Some(BlockPatch::Fields(kde)) => kde.disabled.clone().unwrap_or_default(),
            _ => KdeEffectsPatch::default(),
        },
        _ => KdeEffectsPatch::default(),
    }
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

/// The capitalization: each `QFont::Capitalization` but `MixedCase` the
/// style's `case` of the same meaning; `MixedCase`, and a number no member
/// holds, leave it.
#[test]
fn the_capitalization_is_the_styles_case() {
    use crate::blocks::text::TextCase;

    let case = |capitalization: u8| {
        let text = format!("Synthetic Sans,11,-1,5,400,0,0,0,0,0,{capitalization},0,0,0,0,1");
        font::font_patch(&text, Some(96.0)).unwrap().case
    };
    assert_eq!(case(0), None);
    assert_eq!(case(1), Some(Value::Stated(TextCase::Upper)));
    assert_eq!(case(2), Some(Value::Stated(TextCase::Lower)));
    assert_eq!(case(3), Some(Value::Stated(TextCase::SmallCaps)));
    assert_eq!(case(4), Some(Value::Stated(TextCase::Capitalize)));
    assert_eq!(case(5), None);
}

/// The icon theme and its sizes under the own part, each size from its
/// `[Icon Theme]` key, `MainToolbarDefault` the keyed `main_toolbar`; a
/// key the theme omits, or one KConfig does not convert to an integer, is
/// KIconTheme's default for its group; a size not above zero is not
/// stated.
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
    let px = |size: u16| Some(Value::Stated(Px(f32::from(size))));
    let icon_defaults = &defaults::ICON_SIZES;
    assert_eq!(size(&sizes.toolbar), px(24));
    assert_eq!(size(&sizes.small), px(18));
    assert_eq!(size(&sizes.panel), None);
    assert_eq!(size(&sizes.dialog), px(icon_defaults.dialog));
    assert_eq!(size(&sizes.large), px(icon_defaults.desktop));
    let main_toolbar = |sizes: &IconSizeTokensPatch| match sizes.keyed.as_deref() {
        Some([(key, patch)]) if *key == crate::key!("main_toolbar") => size(&Some(patch.clone())),
        _ => None,
    };
    assert_eq!(main_toolbar(sizes), px(icon_defaults.main_toolbar));

    let stated = icon_sizes(&ini(
        "[Icon Theme]\nMainToolbarDefault=30\nDesktopDefault=-4\n",
    ));
    assert_eq!(main_toolbar(&stated), px(30));
    assert_eq!(size(&stated.large), None);
    let unset = icon_sizes(&ini("[Icon Theme]\n"));
    assert_eq!(size(&unset.toolbar), px(icon_defaults.toolbar));
    assert_eq!(size(&unset.small), px(icon_defaults.small));
    assert_eq!(size(&unset.panel), px(icon_defaults.panel));
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
    for (set, _, _) in COLOR_GROUPS {
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

/// The scheme file is located by existence, as `QStandardPaths::locate`
/// finds it: a data directory whose `<name>.colors` is no file (here a
/// directory) is passed over; a file found there that cannot be read is
/// the unreadable scheme file, not a reason to look further (S9.3).
#[test]
fn the_scheme_file_is_located_by_existence() {
    use std::os::unix::fs::PermissionsExt;

    let dir = temp_config_dir("scheme-file-exists");
    let (first, second, third) = (dir.join("first"), dir.join("second"), dir.join("third"));
    std::fs::create_dir_all(first.join("color-schemes/Synthetic.colors")).unwrap();
    for data_dir in [&second, &third] {
        std::fs::create_dir_all(data_dir.join("color-schemes")).unwrap();
        std::fs::write(
            data_dir.join("color-schemes/Synthetic.colors"),
            "[Colors:View]\nBackgroundNormal=1,2,3\n",
        )
        .unwrap();
    }
    let kdeglobals = ini("[General]\nColorScheme=Synthetic\n");
    assert!(matches!(
        read_scheme_file(&kdeglobals, &[first.clone(), third.clone()]),
        SchemeFile::Read(_)
    ));

    let locked = second.join("color-schemes/Synthetic.colors");
    std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o000)).unwrap();
    if std::fs::read(&locked).is_err() {
        match read_scheme_file(&kdeglobals, &[first, second, third]) {
            SchemeFile::Unreadable(why) => {
                assert!(
                    why.contains("second/color-schemes/Synthetic.colors"),
                    "{why}"
                );
                assert!(why.contains("cannot be read"), "{why}");
            }
            _ => panic!("an unreadable scheme file found first is the fallback"),
        }
    }
    std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o644)).unwrap();
}

/// The colours the palette is built from: none read where kdeglobals has a
/// `[Colors:View]` group; else the look-and-feel package's colours, the
/// default package's where kdeglobals names none or an empty one, else the
/// scheme file, else Breeze Light's, each in the first data directory
/// holding it, a name that is not one path component naming no file; an
/// empty file where none is found (S9.3).
#[test]
fn the_palette_colours_follow_plasma_integration() -> Result<(), Box<dyn std::error::Error>> {
    let dir = temp_config_dir("palette-colors");
    let (first, second) = (dir.join("first"), dir.join("second"));
    let write = |path: std::path::PathBuf, red: u8| -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(
            path,
            format!("[Colors:Window]\nBackgroundNormal={red},0,0\n"),
        )
    };
    write(
        second.join("plasma/look-and-feel/synthetic.lnf/contents/colors"),
        1,
    )?;
    write(
        first.join("plasma/look-and-feel/org.kde.breeze.desktop/contents/colors"),
        2,
    )?;
    write(second.join("color-schemes/Synthetic.colors"), 3)?;
    write(second.join("color-schemes/BreezeLight.colors"), 4)?;
    let both = [first.clone(), second];
    let window = |kdeglobals: &str, dirs: &[std::path::PathBuf]| {
        palette_colors(&ini(kdeglobals), dirs)
            .map(|colors| colors.get("Colors:Window", "BackgroundNormal"))
    };
    let red = |value: &str| Some(Some(format!("{value},0,0")));

    assert_eq!(
        window("[Colors:View]\nBackgroundNormal=9,9,9\n", &both),
        None
    );
    assert_eq!(
        window(
            "[KDE]\nLookAndFeelPackage=synthetic.lnf\n[General]\nColorScheme=Synthetic\n",
            &both
        ),
        red("1")
    );
    assert_eq!(
        window("[General]\nColorScheme=Synthetic\n", &both),
        red("2")
    );
    assert_eq!(window("[KDE]\nLookAndFeelPackage=\n", &both), red("2"));
    assert_eq!(
        window(
            "[KDE]\nLookAndFeelPackage=missing\n[General]\nColorScheme=Synthetic\n",
            &both
        ),
        red("3")
    );
    assert_eq!(
        window(
            "[KDE]\nLookAndFeelPackage=../synthetic.lnf\n[General]\nColorScheme=Missing\n",
            &both
        ),
        red("4")
    );
    let none = palette_colors(&ini("[KDE]\nLookAndFeelPackage=missing\n"), &[first])
        .ok_or("no `[Colors:View]` group: the palette's colours are read")?;
    assert!(none.get_map_ref().is_empty());
    Ok(())
}

/// Where kdeglobals has no `[Colors:View]` group, the scheme, the results,
/// the inactive results and the accent's `Selection` background are the
/// palette's colours, not kdeglobals' colour groups; the colour effects
/// stay kdeglobals' (S9.3).
#[test]
fn without_a_view_group_the_colours_are_the_palettes() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = data(
        "[Colors:Window]\nBackgroundNormal=20,21,22\n\
         [Colors:Selection]\nBackgroundNormal=40,41,42\n\
         [ColorEffects:Disabled]\nIntensityAmount=0.15\n",
        SchemeFile::Unnamed,
    );
    data.colors = Some(ini("[Colors:Window]\nBackgroundNormal=240,241,242\n\
         [Colors:Window][Inactive]\nBackgroundNormal=230,231,232\n\
         [Colors:Selection]\nBackgroundNormal=1,2,3\n"));
    let reading = reading(&data);
    assert_eq!(reading.state.scheme, Scheme::Light);
    assert_eq!(
        input_color(&reading.layer.bases, "accent", Scheme::Light),
        Some(rgb(1, 2, 3))
    );
    let bases = own(&reading.layer);
    assert_eq!(
        input_color(bases, "inactive.window.background_normal", Scheme::Light),
        Some(rgb(230, 231, 232))
    );
    let surfaces = bases
        .colors
        .get(Scheme::Light)
        .and_then(|colors| colors.surfaces.as_ref())
        .ok_or("the results under the scheme read")?;
    match surfaces.get(SurfaceSet::Window) {
        Some(BlockPatch::Fields(window)) => {
            assert_eq!(window.background, Some(Value::Stated(rgb(240, 241, 242))))
        }
        other => return Err(format!("the window's results: {other:?}").into()),
    }
    let RuleBody::Tokens(tokens) = &reading.layer.rules.first().ok_or("a rule")?.body else {
        return Err("a tokens rule".into());
    };
    let Some(BlockPatch::Fields(kde)) = &tokens.kde else {
        return Err("token.kde's fields".into());
    };
    assert_eq!(
        kde.disabled
            .as_ref()
            .and_then(|disabled| disabled.intensity_amount.clone()),
        Some(Value::Stated(0.15))
    );
    Ok(())
}

/// The font DPI is the one Qt's xcb plugin forces from the X resources:
/// the last `Xft.dpi` line it parses, a decimal rounded, where it is above
/// zero; none without one; and it is read on X11 only.
#[test]
fn the_font_dpi_is_xft_dpi_on_x11() {
    use font_dpi::{font_dpi_of, read_font_dpi};

    assert_eq!(
        font_dpi_of("Xft.antialias:\t1\nXft.dpi:\t120\nXft.hinting:\t1\n"),
        Some(120.0)
    );
    assert_eq!(font_dpi_of("Xft.dpi:\t96\nXft.dpi:\t144\n"), Some(144.0));
    assert_eq!(font_dpi_of("Xft.dpi:\t120.6\n"), Some(121.0));
    for none in ["Xft.dpi:\t0\n", "Xft.dpi:\tbig\n", "*.dpi:\t96\n", ""] {
        assert_eq!(font_dpi_of(none), None, "{none:?}");
    }
    assert_eq!(read_font_dpi(Some("wayland")), None);
    assert_eq!(read_font_dpi(None), None);
}
