//! The macOS reader's tests: the state and layer it builds from what AppKit
//! gave (S9.3, S9.5), on every platform, and its reads on macOS.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::*;

/// A synthetic colour, distinct per `n`.
pub(crate) fn synthetic(n: u8) -> Color {
    Color {
        r: f32::from(n) / 255.0,
        g: 0.25,
        b: 0.5,
        a: 1.0,
    }
}

/// An appearance whose every colour was read: synthetic, distinct per
/// field, offset by `base`.
pub(crate) fn read_colors(base: u8) -> AppearanceColors {
    let c = |n: u8| Some(synthetic(base.saturating_add(n)));
    AppearanceColors {
        accent: c(1),
        selected_content_background: c(2),
        highlight: c(3),
        keyboard_focus_indicator: c(4),
        selected_control: c(5),
        window_background: c(6),
        label: c(7),
        secondary_label: c(8),
        link: c(9),
        separator: c(10),
        tertiary_label: c(11),
        control_background: c(12),
        alternating_content_background: c(13),
        alternate_selected_control_text: c(14),
        selected_text: c(15),
        system_red: c(16),
        system_orange: c(17),
        system_green: c(18),
        system_blue: c(19),
        secondary_selected_control: c(20),
    }
}

/// The offset of the light appearance's synthetic colours.
pub(crate) const LIGHT: u8 = 0;
/// The offset of the dark appearance's synthetic colours.
pub(crate) const DARK: u8 = 100;

/// A Mac's data: light, both appearances read, no setting on, the
/// main-thread settings read as off.
pub(crate) fn mac_data() -> MacosData {
    MacosData {
        scheme: Scheme::Light,
        increase_contrast: false,
        reduce_motion: false,
        reduce_transparency: false,
        overlay_scrollbars: Some(false),
        full_keyboard_access: Some(false),
        light: Some(read_colors(LIGHT)),
        dark: Some(read_colors(DARK)),
    }
}

/// The value `entries` gives `name`.
fn input<'a>(entries: &'a [(InputName, InputValue)], name: &str) -> Option<&'a InputValue> {
    let name = InputName::new(name).unwrap();
    entries.iter().find(|(n, _)| *n == name).map(|(_, v)| v)
}

/// The reader's own preset's part.
fn own_part(reading: &Reading, own: PresetId) -> &Bases {
    let parts: Vec<_> = reading
        .layer
        .for_preset
        .iter()
        .filter(|(id, _)| *id == own)
        .collect();
    assert_eq!(parts.len(), 1, "one part under for_preset.{}", own.key());
    &parts[0].1
}

/// The surface set's patch `colors` holds.
fn surface(colors: &ColorRolesPatch, set: SurfaceSet) -> &SurfaceRolesPatch {
    match colors.surfaces.as_ref().and_then(|map| map.get(set)) {
        Some(BlockPatch::Fields(patch)) => patch,
        other => panic!("{set:?}: {other:?}"),
    }
}

/// A per-scheme map of synthetic colours, `light` and `dark`.
fn both(light: u8, dark: u8) -> SchemeMap<Value<Color>> {
    SchemeMap::new(
        Value::Stated(synthetic(light)),
        Value::Stated(synthetic(dark)),
    )
}

/// The state is the system's: its scheme, Increase Contrast as the
/// increased contrast, reduced motion and transparency, the main-thread
/// settings as read; text unscaled and no font DPI (S9.4).
#[test]
fn the_state_is_the_systems() {
    let data = MacosData {
        scheme: Scheme::Dark,
        increase_contrast: true,
        reduce_motion: true,
        reduce_transparency: true,
        overlay_scrollbars: Some(true),
        full_keyboard_access: Some(true),
        ..mac_data()
    };
    let state = build_reading(&data, PresetId::MacosTahoe).state;
    assert_eq!(state.scheme, Scheme::Dark);
    assert_eq!(state.contrast, Some(Contrast::Increased));
    assert!(state.reduced_motion && state.reduced_transparency);
    assert!(state.overlay_scrollbars && state.full_keyboard_access);
    assert_eq!(state.text_scaling_factor, UNSCALED);
    assert_eq!(state.font_dpi, None);

    let state = build_reading(&mac_data(), PresetId::MacosTahoe).state;
    assert_eq!(state.contrast, None);
    assert!(!state.reduced_motion && !state.reduced_transparency);
}

/// Off the main thread the scroller style and keyboard navigation are not
/// read: the state keeps S16's no-reader `false` for both.
#[test]
fn unread_main_thread_settings_keep_the_no_reader_state() {
    let data = MacosData {
        overlay_scrollbars: None,
        full_keyboard_access: None,
        ..mac_data()
    };
    let state = build_reading(&data, PresetId::MacosTahoe).state;
    let no_reader = ReaderState::new(Scheme::Light, UNSCALED);
    assert_eq!(state.overlay_scrollbars, no_reader.overlay_scrollbars);
    assert_eq!(state.full_keyboard_access, no_reader.full_keyboard_access);
}

/// Both appearances are read and stated under their own scheme (S9.3): the
/// accent in the common bases, the other four accent inputs under
/// `for_preset.<own>`, each per scheme with no contrast base; nothing is a
/// fallback.
#[test]
fn both_appearances_state_the_accent_inputs_per_scheme() {
    let reading = build_reading(&mac_data(), PresetId::MacosMonterey);
    assert_eq!(reading.fallback, None);
    let common = &reading.layer.bases.inputs.entries;
    assert_eq!(common.len(), 1);
    assert_eq!(
        input(common, "accent"),
        Some(&InputValue::new(both(LIGHT + 1, DARK + 1)))
    );
    let own = &own_part(&reading, PresetId::MacosMonterey).inputs.entries;
    for (name, n) in [
        ("selected_content_background", 2),
        ("highlight", 3),
        ("keyboard_focus_indicator", 4),
        ("selected_control", 5),
    ] {
        assert_eq!(
            input(own, name),
            Some(&InputValue::new(both(LIGHT + n, DARK + n))),
            "{name}"
        );
    }
    assert_eq!(own.len(), 4);
    assert!(reading.layer.rules.is_empty());
}

/// Each role read is stated at its target, under its own scheme in the own
/// preset's part: the window's, view's and selection's roles, the two
/// global roles, the four keyed system colours and the inactive selection's
/// keyed role.
#[test]
fn the_roles_read_are_stated_at_their_targets() {
    let reading = build_reading(&mac_data(), PresetId::MacosTahoe);
    let colors = &own_part(&reading, PresetId::MacosTahoe).colors;
    for (scheme, base) in [(Scheme::Light, LIGHT), (Scheme::Dark, DARK)] {
        let at = |n: u8| Some(Value::Stated(synthetic(base + n)));
        let roles = colors.get(scheme).unwrap();
        let window = surface(roles, SurfaceSet::Window);
        assert_eq!(window.background, at(6));
        assert_eq!(window.foreground, at(7));
        assert_eq!(window.foreground_inactive, at(8));
        assert_eq!(window.link, at(9));
        assert_eq!(window.border, at(10));
        assert_eq!(
            window.disabled.as_ref().and_then(|d| d.foreground.clone()),
            at(11)
        );
        let view = surface(roles, SurfaceSet::View);
        assert_eq!(view.background, at(12));
        assert_eq!(view.background_alternate, at(13));
        assert_eq!(surface(roles, SurfaceSet::Selection).foreground, at(14));
        let global = roles.global.as_ref().unwrap();
        assert_eq!(global.accent_foreground, at(14));
        assert_eq!(global.selection_text_foreground, at(15));
        let keyed = roles.keyed.as_ref().unwrap();
        let expected: Vec<(Key, Value<Color>)> = [
            (crate::key!("system_red"), 16),
            (crate::key!("system_orange"), 17),
            (crate::key!("system_green"), 18),
            (crate::key!("system_blue"), 19),
            (crate::key!("selected_control_secondary"), 20),
        ]
        .into_iter()
        .map(|(key, n)| (key, Value::Stated(synthetic(base + n))))
        .collect();
        assert_eq!(keyed, &expected);
    }
    assert_eq!(*reading.layer.bases.colors, SchemeMap::default());
}

/// The roles the presets state as references to an input are left to the
/// preset (`docs/facts/macos/roles.toml`'s header): the reader states no
/// `role.global.accent`, `role.global.selection_text_background`,
/// `role.selection.background`, `role.window.decoration_focus` or
/// `role.@selected_control`.
#[test]
fn the_roles_that_follow_an_input_are_the_presets() {
    let roles = read_colors(LIGHT).roles();
    let global = roles.global.as_ref().unwrap();
    assert_eq!(global.accent, None);
    assert_eq!(global.selection_text_background, None);
    assert_eq!(surface(&roles, SurfaceSet::Selection).background, None);
    assert_eq!(surface(&roles, SurfaceSet::Window).decoration_focus, None);
    let keyed = roles.keyed.as_ref().unwrap();
    assert!(
        keyed
            .iter()
            .all(|(key, _)| *key != crate::key!("selected_control"))
    );
}

/// Under Increase Contrast the reads are the increased-contrast colours:
/// the accent inputs' `contrast.increased` bases, with no per-scheme value,
/// and the roles' reader rules under `scheme:<s>`, `contrast_increased` and
/// `preset:<own>`, with no colour base (S9.3).
#[test]
fn increase_contrast_states_contrast_bases_and_rules() {
    let data = MacosData {
        increase_contrast: true,
        ..mac_data()
    };
    let own = PresetId::MacosTahoe;
    let reading = build_reading(&data, own);
    assert_eq!(reading.fallback, None);
    let increased = |light: u8, dark: u8| InputValue {
        per_scheme: SchemeMap::default(),
        per_contrast: vec![(Contrast::Increased, both(light, dark))],
    };
    let common = &reading.layer.bases.inputs.entries;
    assert_eq!(input(common, "accent"), Some(&increased(1, DARK + 1)));
    let part = own_part(&reading, own);
    assert_eq!(
        input(&part.inputs.entries, "selected_control"),
        Some(&increased(5, DARK + 5))
    );
    assert_eq!(*part.colors, SchemeMap::default());
    assert_eq!(reading.layer.rules.len(), 2);
    for (rule, (scheme, base)) in reading
        .layer
        .rules
        .iter()
        .zip([(Scheme::Light, LIGHT), (Scheme::Dark, DARK)])
    {
        assert_eq!(
            rule.selector.conditions.all_of,
            [
                Condition::Scheme(scheme),
                Condition::ContrastIncreased,
                Condition::Preset(own)
            ]
        );
        assert_eq!(
            rule.body,
            RuleBody::Colors(Box::new(read_colors(base).roles()))
        );
    }
}

/// An accent input without an sRGB form at a scheme drops the whole group
/// there, the other scheme keeping its own, and is the fallback, naming
/// the input (S9.3); the roles read there are still stated.
#[test]
fn an_unread_accent_input_drops_the_group_at_its_scheme() {
    let mut dark = read_colors(DARK);
    dark.selected_control = None;
    let data = MacosData {
        dark: Some(dark),
        ..mac_data()
    };
    let reading = build_reading(&data, PresetId::MacosTahoe);
    let Some(ReaderFailure::Platform(what)) = &reading.fallback else {
        panic!("no fallback: {:?}", reading.fallback);
    };
    assert!(what.starts_with("input.selected_control:"), "{what}");
    let light_only = |n: u8| {
        InputValue::new(SchemeMap::one(
            Scheme::Light,
            Value::Stated(synthetic(LIGHT + n)),
        ))
    };
    assert_eq!(
        input(&reading.layer.bases.inputs.entries, "accent"),
        Some(&light_only(1))
    );
    let part = own_part(&reading, PresetId::MacosTahoe);
    assert_eq!(
        input(&part.inputs.entries, "highlight"),
        Some(&light_only(3))
    );
    assert!(part.colors.get(Scheme::Dark).is_some());
}

/// An appearance AppKit does not give states nothing under its scheme and
/// is the fallback; a read with neither states no input at all.
#[test]
fn a_missing_appearance_states_nothing_under_its_scheme() {
    let data = MacosData {
        light: None,
        ..mac_data()
    };
    let reading = build_reading(&data, PresetId::MacosTahoe);
    assert!(matches!(
        &reading.fallback,
        Some(ReaderFailure::Platform(what)) if what.contains(AQUA)
    ));
    let part = own_part(&reading, PresetId::MacosTahoe);
    assert!(part.colors.get(Scheme::Light).is_none());
    assert_eq!(
        input(&reading.layer.bases.inputs.entries, "accent"),
        Some(&InputValue::new(SchemeMap::one(
            Scheme::Dark,
            Value::Stated(synthetic(DARK + 1))
        )))
    );

    let none = MacosData {
        light: None,
        dark: None,
        ..mac_data()
    };
    let reading = build_reading(&none, PresetId::MacosTahoe);
    assert!(reading.layer.bases.inputs.entries.is_empty());
    assert!(reading.layer.for_preset.is_empty());
}

/// A role without an sRGB form is not stated; the others are.
#[test]
fn a_role_without_an_srgb_form_is_not_stated() {
    let colors = AppearanceColors {
        separator: None,
        system_blue: None,
        ..read_colors(LIGHT)
    };
    let roles = colors.roles();
    let window = surface(&roles, SurfaceSet::Window);
    assert_eq!(window.border, None);
    assert!(window.background.is_some());
    let keyed = roles.keyed.as_ref().unwrap();
    assert_eq!(keyed.len(), 4);
    assert_eq!(
        AppearanceColors::default().roles(),
        ColorRolesPatch::default()
    );
}

/// The reader patches no text-role slot, token or widget: System Settings
/// has no font setting (S9.5), and the reader reads no metric.
#[test]
fn the_reader_patches_no_token_or_widget() {
    let reading = build_reading(&mac_data(), PresetId::MacosTahoe);
    let common = &reading.layer.bases;
    let part = own_part(&reading, PresetId::MacosTahoe);
    for bases in [common, part] {
        assert_eq!(bases.tokens, Bases::default().tokens);
        assert_eq!(bases.widgets, Bases::default().widgets);
    }
}

/// A component that is not finite, or an alpha outside `0..=1`, is no
/// colour (S2.1); an extended-sRGB channel is kept.
#[test]
fn color_from_components_keeps_what_s2_1_allows() {
    assert_eq!(
        color_from_components(1.0, 0.5, 0.0, 1.0),
        Some(Color {
            r: 1.0,
            g: 0.5,
            b: 0.0,
            a: 1.0
        })
    );
    assert!(color_from_components(1.25, -0.25, 0.0, 0.5).is_some());
    assert_eq!(color_from_components(f64::NAN, 0.0, 0.0, 1.0), None);
    assert_eq!(color_from_components(0.0, f64::INFINITY, 0.0, 1.0), None);
    assert_eq!(color_from_components(0.0, 0.0, 0.0, 1.5), None);
    assert_eq!(color_from_components(0.0, 0.0, 0.0, -0.5), None);
}

/// `performAsCurrentDrawingAppearance:` is sent from macOS 11 on only.
#[test]
fn the_drawing_appearance_is_macos_11s() {
    use crate::pipeline::OsVersion;

    assert!(!has_drawing_appearance(OsVersion::new(10, 15, 7)));
    assert!(has_drawing_appearance(OsVersion::new(11, 0, 0)));
    assert!(has_drawing_appearance(OsVersion::new(26, 6, 2)));
}

/// Every surface set the reader patches is one both macOS presets
/// declare.
#[test]
fn every_patched_set_is_one_the_presets_declare() {
    for id in [PresetId::MacosTahoe, PresetId::MacosMonterey] {
        let file = crate::toml::layer::read_preset(id.source()).unwrap();
        let declared = &file.declarations.surface_sets;
        for set in [SurfaceSet::Window, SurfaceSet::View, SurfaceSet::Selection] {
            assert!(declared.contains(&set), "{}: {set:?}", id.key());
        }
    }
}

/// On a Mac: each appearance's colours are the class properties the facts
/// measured, in sRGB, as the OS probe reads them.
#[cfg(all(target_os = "macos", feature = "macos"))]
#[test]
fn the_appearance_colours_are_the_measured_properties() {
    let aqua = NSAppearance::appearanceNamed(&NSString::from_str(AQUA)).unwrap();
    let read = appearance_colors(&aqua);
    let expected = std::cell::Cell::new(None);
    {
        let block = RcBlock::new(|| {
            let space = NSColorSpace::sRGBColorSpace();
            #[allow(deprecated, reason = "the property the presets' facts measured")]
            let secondary_selected = NSColor::secondarySelectedControlColor();
            expected.set(Some((
                srgb(&NSColor::separatorColor(), &space),
                srgb(&NSColor::alternateSelectedControlTextColor(), &space),
                srgb(&NSColor::selectedTextColor(), &space),
                srgb(&NSColor::selectedTextBackgroundColor(), &space),
                srgb(&NSColor::tertiaryLabelColor(), &space),
                srgb(&NSColor::selectedControlColor(), &space),
                srgb(&secondary_selected, &space),
            )));
        });
        aqua.performAsCurrentDrawingAppearance(&block);
    }
    let (
        separator,
        alternate_text,
        selected_text,
        highlight,
        tertiary,
        selected_control,
        secondary_selected,
    ) = expected.into_inner().unwrap();
    assert!(separator.is_some());
    assert_eq!(read.separator, separator);
    assert_eq!(read.alternate_selected_control_text, alternate_text);
    assert_eq!(read.selected_text, selected_text);
    assert_eq!(read.highlight, highlight);
    assert_eq!(read.tertiary_label, tertiary);
    assert_eq!(read.selected_control, selected_control);
    assert_eq!(read.secondary_selected_control, secondary_selected);
}

/// On a Mac: a read states the system's scheme and contrast, both
/// appearances' accent inputs and roles, and no fallback.
#[cfg(all(target_os = "macos", feature = "macos"))]
#[test]
fn a_read_is_the_running_systems() {
    let reading = read(PresetId::MacosTahoe).unwrap();
    assert_eq!(reading.state.scheme, system_scheme());
    let increased = NSWorkspace::sharedWorkspace().accessibilityDisplayShouldIncreaseContrast();
    assert_eq!(
        reading.state.contrast,
        increased.then_some(Contrast::Increased)
    );
    assert_eq!(reading.fallback, None);
    let accent = input(&reading.layer.bases.inputs.entries, "accent").unwrap();
    let per_scheme = if increased {
        &accent.per_contrast.first().unwrap().1
    } else {
        &accent.per_scheme
    };
    assert!(per_scheme.light.is_some() && per_scheme.dark.is_some());
}
