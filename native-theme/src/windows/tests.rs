//! The Windows reader's tests: the reading built from what Windows gave.
//!
//! Every colour here is synthetic: the tests check where the reader puts
//! what it reads, not the platform's values.

use super::*;

/// The synthetic opaque colour `r,g,b`.
pub(crate) fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: f32::from(r) / 255.0,
        g: f32::from(g) / 255.0,
        b: f32::from(b) / 255.0,
        a: 1.0,
    }
}

/// A dark reading's data: a light foreground, the accent `1,2,3`, the ramp
/// `10,11,12` … `15,16,17`, no high contrast, every effect on, no text
/// scale (synthetic values).
pub(crate) fn data() -> WindowsData {
    WindowsData {
        foreground: rgb(255, 255, 255),
        accent: Ok(rgb(1, 2, 3)),
        ramp: [10, 11, 12, 13, 14, 15].map(|n| Ok(rgb(n, n + 1, n + 2))),
        high_contrast: Some(false),
        system_colors: None,
        animations_enabled: Some(true),
        advanced_effects_enabled: Some(true),
        text_scale_factor: None,
    }
}

/// The synthetic system colours, `COLOR_WINDOW` `100,100,100` up.
pub(crate) fn system_colors() -> SystemColors {
    SystemColors {
        window: rgb(100, 100, 100),
        window_text: rgb(101, 101, 101),
        button_face: rgb(102, 102, 102),
        button_text: rgb(103, 103, 103),
        highlight: rgb(104, 104, 104),
        highlight_text: rgb(105, 105, 105),
        hotlight: rgb(106, 106, 106),
        gray_text: rgb(107, 107, 107),
    }
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

/// A light foreground is dark mode, a dark one light mode, by Microsoft's
/// `IsColorLight`; its boundary, a sum of exactly 1024, is not light.
#[test]
fn the_foreground_decides_the_scheme() {
    assert_eq!(scheme(rgb(255, 255, 255)), Scheme::Dark);
    assert_eq!(scheme(rgb(0, 0, 0)), Scheme::Light);
    assert_eq!(scheme(rgb(128, 128, 128)), Scheme::Light);
    assert_eq!(scheme(rgb(128, 129, 128)), Scheme::Dark);
}

/// A COLORREF's red is its low byte (0.7.0's `colorref_to_rgba` tests).
#[test]
fn colorref_is_red_in_the_low_byte() {
    assert_eq!(colorref(0x00FF_8040), rgb(0x40, 0x80, 0xFF));
    assert_eq!(colorref(0), rgb(0, 0, 0));
    assert_eq!(colorref(0x00FF_FFFF), rgb(255, 255, 255));
}

/// The state: the scheme, high contrast, the two effects and the text
/// scale, an unusable one unscaled; no font DPI.
#[test]
fn the_state_is_what_windows_gives() {
    let state = build_reading(&data(), PresetId::Windows11).state;
    assert_eq!(state.scheme, Scheme::Dark);
    assert_eq!(state.contrast, None);
    assert!(!state.animations_off && !state.reduced_motion && !state.reduced_transparency);
    assert_eq!(state.text_scaling_factor, UNSCALED);
    assert_eq!(state.font_dpi, None);

    let state = build_reading(
        &WindowsData {
            foreground: rgb(0, 0, 0),
            high_contrast: Some(true),
            animations_enabled: Some(false),
            advanced_effects_enabled: Some(false),
            text_scale_factor: Some(1.5),
            ..data()
        },
        PresetId::Windows11,
    )
    .state;
    assert_eq!(state.scheme, Scheme::Light);
    assert_eq!(state.contrast, Some(Contrast::High));
    assert!(state.animations_off && state.reduced_motion && state.reduced_transparency);
    assert_eq!(state.text_scaling_factor, 1.5);

    let unread = build_reading(
        &WindowsData {
            animations_enabled: None,
            ..data()
        },
        PresetId::Windows11,
    )
    .state;
    assert!(!unread.animations_off && !unread.reduced_motion);

    for unusable in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        let state = build_reading(
            &WindowsData {
                text_scale_factor: Some(unusable),
                ..data()
            },
            PresetId::Windows11,
        )
        .state;
        assert_eq!(state.text_scaling_factor, UNSCALED, "{unusable}");
    }
}

/// The accent at both schemes in the common bases, its ramp under the own
/// part, with no fallback (S9.3, S9.5).
#[test]
fn the_accent_and_its_ramp_are_one_group_at_both_schemes() {
    let reading = build_reading(&data(), PresetId::Windows11);
    assert!(reading.fallback.is_none());
    let accent = Value::Stated(rgb(1, 2, 3));
    assert_eq!(
        input_value(&reading.layer.bases, "accent")
            .unwrap()
            .per_scheme,
        SchemeMap::new(accent.clone(), accent)
    );
    assert_eq!(reading.layer.bases.inputs.entries.len(), 1);
    let [(id, own)] = reading.layer.for_preset.as_slice() else {
        panic!("one own part");
    };
    assert_eq!(*id, PresetId::Windows11);
    assert_eq!(own.inputs.entries.len(), 6);
    let light2 = Value::Stated(rgb(14, 15, 16));
    assert_eq!(
        input_value(own, "accent_ramp.light2").unwrap().per_scheme,
        SchemeMap::new(light2.clone(), light2)
    );
    assert!(reading.layer.rules.is_empty());
}

/// A shade that cannot be read drops the whole group, the fallback naming
/// it; so does the accent (S9.3).
#[test]
fn a_failed_shade_drops_the_group() {
    let mut failed = data();
    failed.ramp[1] = Err("synthetic failure".to_owned());
    let reading = build_reading(&failed, PresetId::Windows11);
    assert_eq!(
        reading.fallback,
        Some(ReaderFailure::Platform(
            "input.accent_ramp.dark2: UISettings.GetColorValue(AccentDark2) failed: synthetic failure"
                .to_owned()
        ))
    );
    assert!(reading.layer.bases.inputs.entries.is_empty());
    assert!(reading.layer.for_preset.is_empty());

    let reading = build_reading(
        &WindowsData {
            accent: Err("gone".to_owned()),
            ..data()
        },
        PresetId::Windows11,
    );
    assert!(matches!(
        reading.fallback,
        Some(ReaderFailure::Platform(why)) if why.starts_with("input.accent: ")
    ));
    assert!(reading.layer.bases.inputs.entries.is_empty());
}

/// Under high contrast the system colours are a rule under
/// `contrast_high` and the own preset; outside it none is stated, even
/// where some were given (ADD-A135).
#[test]
fn the_system_colours_are_a_rule_under_high_contrast() {
    let reading = build_reading(
        &WindowsData {
            high_contrast: Some(true),
            system_colors: Some(system_colors()),
            ..data()
        },
        PresetId::Windows11,
    );
    let [rule] = reading.layer.rules.as_slice() else {
        panic!("one rule: {:?}", reading.layer.rules);
    };
    assert_eq!(
        rule.selector.conditions.all_of,
        [
            Condition::ContrastHigh,
            Condition::Preset(PresetId::Windows11)
        ]
    );
    let RuleBody::Colors(colors) = &rule.body else {
        panic!("a colours rule: {rule:?}");
    };
    let keyed = colors.keyed.as_ref().unwrap();
    assert_eq!(keyed.len(), 8);
    assert_eq!(
        keyed.first(),
        Some(&(
            crate::key!("system_window"),
            Value::Stated(rgb(100, 100, 100))
        ))
    );
    assert_eq!(
        keyed.get(7),
        Some(&(
            crate::key!("system_gray_text"),
            Value::Stated(rgb(107, 107, 107))
        ))
    );

    let reading = build_reading(
        &WindowsData {
            high_contrast: None,
            system_colors: Some(system_colors()),
            ..data()
        },
        PresetId::Windows11,
    );
    assert!(reading.layer.rules.is_empty());
    assert_eq!(reading.state.contrast, None);
}

/// Every key the reader states the system colours under is one
/// `windows-11` declares.
#[test]
fn the_system_colours_are_windows_11s_keyed_roles() {
    let file = crate::toml::layer::read_preset(PresetId::Windows11.source()).unwrap();
    for (key, _) in system_colors().roles() {
        assert!(
            file.declarations.color_roles.contains(&key),
            "{key} is not declared"
        );
    }
}
