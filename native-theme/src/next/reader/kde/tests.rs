use super::*;

use crate::kde::{create_kde_parser, parse_icon_sizes_from_content, read_kconfig};

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// `content` parsed as the 0.7.2 reader parses a kdeglobals file.
fn parsed(content: &str) -> Ini {
    let mut ini = create_kde_parser();
    read_kconfig(&mut ini, content);
    ini
}

/// The opaque colour of these 8-bit channels.
fn rgb(r: u8, g: u8, b: u8) -> Option<Color> {
    Some(Color::from_srgb8(r, g, b, u8::MAX))
}

fn os_font(family: &str, size: FontSize, weight: u16) -> Option<OsFont> {
    Some(OsFont {
        family: Some(Arc::from(family)),
        size: Some(size),
        weight: Some(weight),
        style: Some(FontStyle::Normal),
        width: None,
    })
}

/// What `overrides_from_kdeglobals` states for every file, the scheme, the
/// font DPI and the name of a file without `[General] ColorScheme`, with
/// `colors` as the scheme's entry.
fn expected(scheme: Scheme, colors: Option<OsColors>) -> SystemOverrides {
    SystemOverrides {
        color_schemes: colors
            .map(|c| ((scheme, Contrast::Normal), c))
            .into_iter()
            .collect(),
        scheme: Some(scheme),
        theme_name: Some(Arc::from("KDE")),
        font_dpi: Some(96.0),
        ..SystemOverrides::default()
    }
}

const BREEZE_LIGHT: &str = include_str!("../../../../tests/fixtures/kde/breeze-light.ini");
const BREEZE_INDEX: &str = include_str!("../../../../tests/fixtures/kde/breeze-index.theme");
const MINIMAL: &str = include_str!("../../../../tests/fixtures/kde/minimal.ini");
const MISSING_GROUPS: &str = include_str!("../../../../tests/fixtures/kde/missing-groups.ini");
const MALFORMED: &str = include_str!("../../../../tests/fixtures/kde/malformed-values.ini");

// The expected values below are read from the fixtures by eye.

#[test]
fn breeze_light() {
    let icon_sizes = parse_icon_sizes_from_content(&parsed(BREEZE_INDEX));
    let got = overrides_from_kdeglobals(&parsed(BREEZE_LIGHT), Some(icon_sizes), None);
    let colors = OsColors {
        surfaces: Some(OsSurfaceOverrides {
            window: Some(SurfaceColors::new(
                rgb(239, 240, 241),
                rgb(35, 38, 41),
                rgb(112, 125, 138),
                None,
                None,
            )),
            view: Some(SurfaceColors::new(
                rgb(255, 255, 255),
                rgb(35, 38, 41),
                rgb(112, 125, 138),
                None,
                None,
            )),
            header: Some(SurfaceColors::new(
                rgb(222, 224, 226),
                rgb(35, 38, 41),
                None,
                None,
                None,
            )),
            tooltip: Some(SurfaceColors::new(
                rgb(247, 247, 247),
                rgb(35, 38, 41),
                None,
                None,
                None,
            )),
            title_bar: Some(SurfaceColors::new(
                rgb(227, 229, 231),
                rgb(35, 38, 41),
                None,
                None,
                None,
            )),
            title_bar_inactive: Some(SurfaceColors::new(
                rgb(239, 240, 241),
                rgb(112, 125, 138),
                None,
                None,
                None,
            )),
            kde_button: Some(SurfaceColors::new(
                rgb(252, 252, 252),
                rgb(35, 38, 41),
                None,
                None,
                None,
            )),
            ..OsSurfaceOverrides::default()
        }),
        selection: Some(OsSelectionColors::new(
            rgb(61, 174, 233),
            rgb(255, 255, 255),
            None,
            None,
        )),
        link: rgb(41, 128, 185),
        link_visited: rgb(155, 89, 182),
        status: Some([
            rgb(218, 68, 83),
            rgb(246, 116, 0),
            rgb(39, 174, 96),
            rgb(61, 174, 233),
        ]),
        alternate_row_background: rgb(247, 247, 247),
        kde_decoration_focus: rgb(61, 174, 233),
        kde_decoration_hover: rgb(61, 174, 233),
        ..OsColors::default()
    };
    let want = SystemOverrides {
        theme_name: Some(Arc::from("BreezeLight")),
        ui_font: os_font("Noto Sans", FontSize::Pt(10.0), 400),
        monospace_font: os_font("Hack", FontSize::Pt(10.0), 400),
        caption_font: os_font("Noto Sans", FontSize::Pt(7.0), 400),
        menu_font: os_font("Noto Sans", FontSize::Pt(9.0), 400),
        toolbar_font: os_font("Noto Sans", FontSize::Pt(8.0), 400),
        icon_theme: Some(Arc::from("breeze")),
        // breeze-index.theme: MainToolbarDefault, SmallDefault, DesktopDefault, DialogDefault,
        // PanelDefault.
        icon_sizes: Some(IconSizes::new(
            Some(22.0),
            Some(16.0),
            Some(48.0),
            Some(32.0),
            Some(48.0),
        )),
        animation_scale: Some(1.0),
        ..expected(Scheme::Light, Some(colors))
    };
    assert_eq!(got, want);
}

#[test]
fn minimal() {
    let got = overrides_from_kdeglobals(&parsed(MINIMAL), None, None);
    let colors = OsColors {
        surfaces: Some(OsSurfaceOverrides {
            window: Some(SurfaceColors::new(
                rgb(49, 54, 59),
                rgb(239, 240, 241),
                None,
                None,
                None,
            )),
            ..OsSurfaceOverrides::default()
        }),
        ..OsColors::default()
    };
    assert_eq!(got, expected(Scheme::Dark, Some(colors)));
}

#[test]
fn partial() {
    let got = overrides_from_kdeglobals(&parsed(MISSING_GROUPS), None, None);
    let colors = OsColors {
        surfaces: Some(OsSurfaceOverrides {
            window: Some(SurfaceColors::new(
                rgb(49, 54, 59),
                rgb(239, 240, 241),
                None,
                None,
                None,
            )),
            view: Some(SurfaceColors::new(
                rgb(35, 38, 41),
                rgb(252, 252, 252),
                None,
                None,
                None,
            )),
            kde_button: Some(SurfaceColors::new(
                rgb(49, 54, 59),
                rgb(239, 240, 241),
                None,
                None,
                None,
            )),
            ..OsSurfaceOverrides::default()
        }),
        kde_decoration_focus: rgb(61, 174, 233),
        ..OsColors::default()
    };
    let want = SystemOverrides {
        theme_name: Some(Arc::from("MissingGroups")),
        ..expected(Scheme::Dark, Some(colors))
    };
    assert_eq!(got, want);
}

#[test]
fn malformed() {
    let got = overrides_from_kdeglobals(&parsed(MALFORMED), None, None);
    // `abc,def,ghi`, `252,252`, an empty value and `256,0,0` are no colours; `forceFontDPI`
    // is no kdeglobals setting the reader reads.
    let colors = OsColors {
        surfaces: Some(OsSurfaceOverrides {
            window: Some(SurfaceColors::new(rgb(49, 54, 59), None, None, None, None)),
            view: Some(SurfaceColors::new(rgb(35, 38, 41), None, None, None, None)),
            ..OsSurfaceOverrides::default()
        }),
        kde_decoration_focus: rgb(61, 174, 233),
        ..OsColors::default()
    };
    let want = SystemOverrides {
        theme_name: Some(Arc::from("Malformed")),
        ..expected(Scheme::Dark, Some(colors))
    };
    assert_eq!(got, want);
}

#[test]
fn icon_sizes_are_dropped_without_an_icon_theme() {
    let icon_sizes = parse_icon_sizes_from_content(&parsed(BREEZE_INDEX));
    let sized =
        |content: &str| overrides_from_kdeglobals(&parsed(content), Some(icon_sizes.clone()), None);
    assert_eq!(sized("").icon_theme, None);
    assert_eq!(sized("").icon_sizes, None);
    assert_eq!(sized("[Icons]\nTheme=\n").icon_sizes, None);
    let named = sized("[Icons]\nTheme=breeze\n");
    assert_eq!(named.icon_theme, Some(Arc::from("breeze")));
    assert_eq!(
        named.icon_sizes,
        Some(IconSizes::new(
            icon_sizes.toolbar,
            icon_sizes.small,
            icon_sizes.large,
            icon_sizes.dialog,
            icon_sizes.panel,
        ))
    );
    assert!(icon_sizes.toolbar.is_some());
}

#[test]
fn an_empty_file_states_only_the_scheme_the_font_dpi_and_the_name() {
    assert_eq!(
        overrides_from_kdeglobals(&parsed(""), None, None),
        expected(Scheme::Light, None)
    );
}

/// Each fixture's scheme and motion preference, read from the files by eye:
/// `[Colors:Window] BackgroundNormal` (49,54,59 and the like dark, 239,240,241
/// light, none light) and `[KDE] AnimationDurationFactor` (0 in the two dark
/// Breeze files).
const FIXTURE_CONTEXTS: &[(&str, Scheme, MotionPreference)] = &[
    ("breeze-dark.ini", Scheme::Dark, MotionPreference::None),
    ("breeze-index.theme", Scheme::Light, MotionPreference::Full),
    ("breeze-light.ini", Scheme::Light, MotionPreference::Full),
    ("custom-accent.ini", Scheme::Dark, MotionPreference::None),
    ("high-dpi.ini", Scheme::Dark, MotionPreference::Full),
    ("malformed-values.ini", Scheme::Dark, MotionPreference::Full),
    ("minimal.ini", Scheme::Dark, MotionPreference::Full),
    ("missing-groups.ini", Scheme::Dark, MotionPreference::Full),
];

#[test]
fn every_fixture_gives_its_context_and_snapshot() -> TestResult {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/kde");
    let mut files = 0_usize;
    for entry in std::fs::read_dir(&dir)? {
        let path = entry?.path();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("a fixture's name")?;
        let &(_, scheme, motion) = FIXTURE_CONTEXTS
            .iter()
            .find(|(file, _, _)| *file == name)
            .ok_or_else(|| format!("no expected context for {name}"))?;
        let content = std::fs::read_to_string(&path)?;
        let ini = parsed(&content);
        let context = context_from_kdeglobals(&ini);
        let mut want = ThemeContext::default().with_scheme(scheme);
        want.motion = motion;
        assert_eq!(context, want, "{name}");
        // The 0.7.2 reader's `accessibility` block of the same file.
        let (_, _, accessibility) = crate::kde::from_kde_content_pure(&content, Some(96.0))?;
        assert_eq!(
            context.motion != MotionPreference::Full,
            accessibility.reduce_motion,
            "{name}"
        );
        assert_eq!(
            context.contrast != Contrast::Normal,
            accessibility.high_contrast
        );
        assert_eq!(context.text_scale, accessibility.text_scaling_factor);
        assert_eq!(
            context.reduced_transparency,
            accessibility.reduce_transparency
        );
        let snapshot = snapshot_from_kdeglobals(&ini, None, None);
        assert_eq!(snapshot.preset, Preset::KdeBreeze, "{name}");
        assert_eq!(snapshot.context, context, "{name}");
        assert_eq!(
            snapshot.overrides,
            overrides_from_kdeglobals(&ini, None, None),
            "{name}"
        );
        assert_eq!(snapshot.overrides.scheme, Some(scheme), "{name}");
        files = files.saturating_add(1);
    }
    assert_eq!(
        files,
        FIXTURE_CONTEXTS.len(),
        "the fixtures of tests/fixtures/kde/"
    );
    Ok(())
}

#[test]
fn only_a_zero_animation_factor_means_no_motion() {
    let motion = |content: &str| context_from_kdeglobals(&parsed(content)).motion;
    assert_eq!(
        motion("[KDE]\nAnimationDurationFactor=0\n"),
        MotionPreference::None
    );
    assert_eq!(
        motion("[KDE]\nAnimationDurationFactor= 0.0 \n"),
        MotionPreference::None
    );
    for factor in ["0.5", "1", "-1", "inf", "NaN", "none", ""] {
        let content = format!("[KDE]\nAnimationDurationFactor={factor}\n");
        assert_eq!(motion(&content), MotionPreference::Full, "{factor}");
    }
    assert_eq!(motion(""), MotionPreference::Full);
}

#[test]
fn the_portal_accent_is_the_scheme_independent_accent() {
    let got = overrides_from_kdeglobals(
        &parsed(BREEZE_LIGHT),
        None,
        Some(crate::Rgba::rgb(246, 116, 0)),
    );
    assert_eq!(got.accent, rgb(246, 116, 0));
    let os = got.colors_for(Scheme::Light, Contrast::Normal);
    assert_eq!(os.map(|c| c.accent), Some(None));
    assert_eq!(os.and_then(|c| c.kde_decoration_focus), rgb(61, 174, 233));
}

#[test]
fn scalars_as_the_0_7_2_reader_parses_them() {
    let read = |content: &str| overrides_from_kdeglobals(&parsed(content), None, None);
    let os = |o: &SystemOverrides| o.color_schemes.first().map(|(_, c)| c.clone());

    // frameContrast: a number clamped to 0..=1; contrast: an integer.
    let o = read("[KDE]\nframeContrast=0.5\ncontrast=4\n");
    assert_eq!(os(&o).and_then(|c| c.kde_frame_contrast), Some(0.5));
    assert_eq!(os(&o).and_then(|c| c.kde_contrast), Some(4));
    let o = read("[KDE]\nframeContrast=2\ncontrast=4.5\n");
    assert_eq!(os(&o).and_then(|c| c.kde_frame_contrast), Some(1.0));
    assert_eq!(os(&o).and_then(|c| c.kde_contrast), None);
    let o = read("[KDE]\nframeContrast=high\n");
    assert_eq!(os(&o), None);

    // ChangeSelectionColor, else Enable; neither: no effects.
    let effects = |content: &str| os(&read(content)).and_then(|c| c.kde_effects);
    assert_eq!(
        effects("[ColorEffects:Inactive]\nChangeSelectionColor=false\nEnable=true\n"),
        Some(KdeColorEffects::new(None, None, false))
    );
    assert_eq!(
        effects("[ColorEffects:Inactive]\nEnable=FALSE\n"),
        Some(KdeColorEffects::new(None, None, false))
    );
    assert_eq!(
        effects("[ColorEffects:Inactive]\nChangeSelectionColor=true\n"),
        Some(KdeColorEffects::new(None, None, true))
    );
    assert_eq!(effects("[ColorEffects:Inactive]\nEnable=maybe\n"), None);

    // CursorBlinkRate: bounded to 200..=2000 and halved; 0 or less: off 0 alone.
    let blink = |content: &str| {
        let o = read(content);
        (o.caret_blink_on_ms, o.caret_blink_off_ms)
    };
    assert_eq!(
        blink("[KDE]\nCursorBlinkRate=1000\n"),
        (Some(500), Some(500))
    );
    assert_eq!(
        blink("[KDE]\nCursorBlinkRate=5000\n"),
        (Some(1000), Some(1000))
    );
    assert_eq!(
        blink("[KDE]\nCursorBlinkRate=100\n"),
        (Some(100), Some(100))
    );
    assert_eq!(blink("[KDE]\nCursorBlinkRate=0\n"), (None, Some(0)));
    assert_eq!(blink("[KDE]\nCursorBlinkRate=fast\n"), (None, None));

    // AnimationDurationFactor: the value, a non-finite one refused.
    assert_eq!(
        read("[KDE]\nAnimationDurationFactor=0\n").animation_scale,
        Some(0.0)
    );
    assert_eq!(
        read("[KDE]\nAnimationDurationFactor=0.5\n").animation_scale,
        Some(0.5)
    );
    assert_eq!(
        read("[KDE]\nAnimationDurationFactor=inf\n").animation_scale,
        None
    );

    // [WM] activeFont, a Qt5 string: weight 75 is CSS 700.
    assert_eq!(
        read("[WM]\nactiveFont=Noto Sans,10,-1,5,75,0,0,0,0,0\n").title_bar_font,
        os_font("Noto Sans", FontSize::Pt(10.0), 700)
    );
    // An empty icon theme names none.
    assert_eq!(read("[Icons]\nTheme=\n").icon_theme, None);
}

/// Every colour setting of the inventory (`[Colors:*]` and `[WM]`) and its
/// slot in the active entry's `OsColors`.
type Slot = fn(&OsColors) -> Option<Color>;

fn surfaces(c: &OsColors) -> OsSurfaceOverrides {
    c.surfaces.clone().unwrap_or_default()
}

const COLOUR_KEYS: &[(&str, &str, Slot)] = &[
    ("Colors:Window", "BackgroundNormal", |c| {
        surfaces(c).window.and_then(|s| s.background)
    }),
    ("Colors:Window", "ForegroundNormal", |c| {
        surfaces(c).window.and_then(|s| s.foreground)
    }),
    ("Colors:Window", "ForegroundInactive", |c| {
        surfaces(c).window.and_then(|s| s.foreground_muted)
    }),
    ("Colors:View", "BackgroundNormal", |c| {
        surfaces(c).view.and_then(|s| s.background)
    }),
    ("Colors:View", "ForegroundNormal", |c| {
        surfaces(c).view.and_then(|s| s.foreground)
    }),
    ("Colors:View", "ForegroundInactive", |c| {
        surfaces(c).view.and_then(|s| s.foreground_muted)
    }),
    ("Colors:View", "DecorationFocus", |c| c.kde_decoration_focus),
    ("Colors:View", "DecorationHover", |c| c.kde_decoration_hover),
    ("Colors:View", "ForegroundLink", |c| c.link),
    ("Colors:View", "ForegroundVisited", |c| c.link_visited),
    ("Colors:View", "ForegroundNegative", |c| {
        c.status.and_then(|[s, ..]| s)
    }),
    ("Colors:View", "ForegroundNeutral", |c| {
        c.status.and_then(|[_, s, ..]| s)
    }),
    ("Colors:View", "ForegroundPositive", |c| {
        c.status.and_then(|[_, _, s, _]| s)
    }),
    ("Colors:View", "ForegroundActive", |c| {
        c.status.and_then(|[.., s]| s)
    }),
    ("Colors:View", "BackgroundAlternate", |c| {
        c.alternate_row_background
    }),
    ("Colors:Button", "BackgroundNormal", |c| {
        surfaces(c).kde_button.and_then(|s| s.background)
    }),
    ("Colors:Button", "ForegroundNormal", |c| {
        surfaces(c).kde_button.and_then(|s| s.foreground)
    }),
    ("Colors:Selection", "BackgroundNormal", |c| {
        c.selection.and_then(|s| s.background)
    }),
    ("Colors:Selection", "ForegroundNormal", |c| {
        c.selection.and_then(|s| s.foreground)
    }),
    ("Colors:Tooltip", "BackgroundNormal", |c| {
        surfaces(c).tooltip.and_then(|s| s.background)
    }),
    ("Colors:Tooltip", "ForegroundNormal", |c| {
        surfaces(c).tooltip.and_then(|s| s.foreground)
    }),
    ("Colors:Header", "BackgroundNormal", |c| {
        surfaces(c).header.and_then(|s| s.background)
    }),
    ("Colors:Header", "ForegroundNormal", |c| {
        surfaces(c).header.and_then(|s| s.foreground)
    }),
    ("WM", "activeBackground", |c| {
        surfaces(c).title_bar.and_then(|s| s.background)
    }),
    ("WM", "activeForeground", |c| {
        surfaces(c).title_bar.and_then(|s| s.foreground)
    }),
    ("WM", "inactiveBackground", |c| {
        surfaces(c).title_bar_inactive.and_then(|s| s.background)
    }),
    ("WM", "inactiveForeground", |c| {
        surfaces(c).title_bar_inactive.and_then(|s| s.foreground)
    }),
];

/// A kdeglobals colour value, `R,G,B` of three bytes, as the test reads it.
fn stated_colour(value: &str) -> Option<Color> {
    let parts: Vec<&str> = value.split(',').collect();
    let [r, g, b] = parts.as_slice() else {
        return None;
    };
    Some(Color::from_srgb8(
        r.trim().parse().ok()?,
        g.trim().parse().ok()?,
        b.trim().parse().ok()?,
        u8::MAX,
    ))
}

#[test]
fn every_fixture_carries_every_colour_it_states() -> TestResult {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/kde");
    let mut files = 0_usize;
    for entry in std::fs::read_dir(&dir)? {
        let path = entry?.path();
        let ini = parsed(&std::fs::read_to_string(&path)?);
        let got = overrides_from_kdeglobals(&ini, None, None);
        let scheme = got.scheme.ok_or("every file states a scheme")?;
        let active = got.colors_for(scheme, Contrast::Normal).cloned();
        assert!(got.color_schemes.len() <= 1, "{path:?}");
        for (section, key, slot) in COLOUR_KEYS {
            let want = ini.get(section, key).as_deref().and_then(stated_colour);
            let have = active.as_ref().and_then(slot);
            assert_eq!(have, want, "{path:?}: [{section}] {key}");
        }
        files = files.saturating_add(1);
    }
    // breeze-dark, breeze-index.theme, breeze-light, custom-accent, high-dpi,
    // malformed-values, minimal, missing-groups.
    assert_eq!(files, 8, "the fixtures of tests/fixtures/kde/");
    Ok(())
}
