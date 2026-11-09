//! `Palette::from_port`'s tests: the refusal, the lines KConfig skips, the
//! decoding, the conversion's form, and Breeze's two files against
//! `kde-breeze`'s own inputs and tokens.

use super::*;

use crate::tokens::{KdeEffects, KdeEffectsPatch, KdeTokens, KdeTokensPatch};

/// The one port of `from_port(file, scheme)`.
fn port(file: &str, scheme: Scheme) -> Port {
    port_of(file.as_bytes(), scheme)
}

/// The one port of `from_port(bytes, scheme)`.
fn port_of(bytes: &[u8], scheme: Scheme) -> Port {
    let palette = Palette::from_port(bytes, scheme).unwrap();
    let [port] = palette.ports.as_slice() else {
        panic!("one port: {:?}", palette.ports);
    };
    port.clone()
}

/// The port's own bases, `kde-breeze`'s part.
fn own(port: &Port) -> &Bases {
    let [(id, bases)] = port.layer.for_preset.as_slice() else {
        panic!("one preset part: {:?}", port.layer.for_preset);
    };
    assert_eq!(*id, PresetId::KdeBreeze);
    bases
}

/// The input `path`'s value at `scheme` in `bases`.
fn input_at<'a>(bases: &'a Bases, path: &str, scheme: Scheme) -> Option<&'a Value<Color>> {
    let name = InputName::new(path).unwrap();
    bases
        .inputs
        .entries
        .iter()
        .find(|(n, _)| *n == name)
        .and_then(|(_, value)| value.per_scheme.get(scheme))
}

/// The input `path`'s stated colour at `scheme` in `bases`.
fn input_color(bases: &Bases, path: &str, scheme: Scheme) -> Option<Color> {
    match input_at(bases, path, scheme)? {
        Value::Stated(color) => Some(*color),
        _ => None,
    }
}

/// The port's `token.kde`, its one rule's.
fn kde_tokens_of(port: &Port) -> &KdeTokensPatch {
    let [rule] = port.layer.rules.as_slice() else {
        panic!("one rule: {:?}", port.layer.rules);
    };
    assert_eq!(
        rule.selector.conditions.all_of,
        [
            Condition::Scheme(port.scheme),
            Condition::Preset(PresetId::KdeBreeze)
        ]
    );
    let RuleBody::Tokens(tokens) = &rule.body else {
        panic!("a tokens rule: {rule:?}");
    };
    let Some(BlockPatch::Fields(kde)) = &tokens.kde else {
        panic!("token.kde's fields: {:?}", tokens.kde);
    };
    kde
}

/// The synthetic colour `r,g,b`.
fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color {
        r: f32::from(r) / 255.0,
        g: f32::from(g) / 255.0,
        b: f32::from(b) / 255.0,
        a: 1.0,
    }
}

// Every colour in the fixtures below is synthetic: the tests check how the
// conversion reads a file, not a platform's values.

/// A file with no colour group, empty or with other groups only, is
/// refused.
#[test]
fn a_file_with_no_colour_group_is_refused() {
    for file in [
        "",
        "[General]\nName=Synthetic\n",
        "[WM]\nactiveBackground=1,2,3\n[ColorEffects:Inactive]\nEnable=true\n",
        "[Colors:Sidebar]\nBackgroundNormal=1,2,3\n",
        "[Colors:Window\nBackgroundNormal=1,2,3\n",
    ] {
        assert_eq!(
            Palette::from_port(file.as_bytes(), Scheme::Light),
            Err(PortError::NoColorGroup),
            "{file:?}"
        );
    }
}

/// A subgroup alone counts as a colour group; the file is converted, the
/// rest as KColorScheme reads it unset.
#[test]
fn a_colour_subgroup_alone_is_converted() {
    let port = port(
        "[Colors:Header][Inactive]\nBackgroundNormal=1,2,3\n",
        Scheme::Dark,
    );
    assert_eq!(port.scheme, Scheme::Dark);
    let bases = own(&port);
    assert_eq!(
        input_color(bases, "kde_scheme.view.background_normal", Scheme::Dark),
        Some(defaults::VIEW.color(KdeSchemeKey::BackgroundNormal))
    );
}

/// A file with some colour groups is converted, a group it lacks as
/// KColorScheme reads it.
#[test]
fn a_file_with_some_colour_groups_is_converted() {
    let port = port("[Colors:Window]\nBackgroundNormal=1,2,3\n", Scheme::Light);
    let bases = own(&port);
    let light = Scheme::Light;
    assert_eq!(
        input_color(bases, "kde_scheme.window.background_normal", light),
        Some(rgb(1, 2, 3))
    );
    assert_eq!(
        input_color(bases, "kde_scheme.button.background_normal", light),
        Some(defaults::BUTTON.color(KdeSchemeKey::BackgroundNormal))
    );
    // `Header` without a group reads `Window`'s values (S9.1).
    assert_eq!(
        input_color(bases, "kde_scheme.header.background_normal", light),
        Some(rgb(1, 2, 3))
    );
    assert_eq!(
        bases.inputs.kde_groups,
        SchemeMap::one(light, vec![SurfaceSet::Window])
    );
}

/// KConfig skips a group header without its closing `]`, the group before
/// it staying current: its entries are read into that group.
#[test]
fn a_header_without_its_bracket_keeps_the_group_before_it() {
    let port = port(
        "[Colors:View]\nBackgroundNormal=1,2,3\n[Colors:Window\nForegroundNormal=4,5,6\n",
        Scheme::Light,
    );
    let bases = own(&port);
    let light = Scheme::Light;
    assert_eq!(
        input_color(bases, "kde_scheme.view.foreground_normal", light),
        Some(rgb(4, 5, 6))
    );
    assert_eq!(
        input_color(bases, "kde_scheme.window.foreground_normal", light),
        Some(defaults::WINDOW.color(KdeSchemeKey::ForegroundNormal))
    );
    assert_eq!(
        bases.inputs.kde_groups,
        SchemeMap::one(light, vec![SurfaceSet::View])
    );
}

/// KConfig skips an entry with an empty key; the lines after it are read.
#[test]
fn an_entry_with_an_empty_key_is_skipped() {
    let port = port(
        "[Colors:View]\n=7,8,9\nBackgroundNormal=1,2,3\n",
        Scheme::Light,
    );
    let bases = own(&port);
    assert_eq!(
        input_color(bases, "kde_scheme.view.background_normal", Scheme::Light),
        Some(rgb(1, 2, 3))
    );
}

/// The bytes are decoded as the KDE reader decodes them: an invalid UTF-8
/// sequence is replaced, and the rest of its line and of the file are read.
#[test]
fn non_utf8_bytes_are_decoded_as_the_reader_decodes_them() {
    let mut bytes = b"[General]\nName=Synthetic ".to_vec();
    bytes.extend([0xff, 0xfe]);
    bytes.extend(b"\n[Colors:View]\nBackgroundNormal=1,2,3\n");
    let mut lossy = create_kde_parser();
    read_kconfig_bytes(&mut lossy, &bytes);
    assert_eq!(
        lossy.get("General", "Name").as_deref(),
        Some("Synthetic \u{fffd}\u{fffd}")
    );
    let port = port_of(&bytes, Scheme::Light);
    assert_eq!(
        input_color(
            own(&port),
            "kde_scheme.view.background_normal",
            Scheme::Light
        ),
        Some(rgb(1, 2, 3))
    );
}

/// The roles: the file's values under its scheme alone, the inactive
/// input roles of a set without an `[Inactive]` group its active roles,
/// those of a set with one and the `[WM]` roles colours.
#[test]
fn the_port_states_its_roles_under_its_scheme() {
    let port = port(
        "\
[Colors:View]
BackgroundNormal=1,2,3
[Colors:View][Inactive]
BackgroundNormal=4,5,6
[WM]
activeBackground=7,8,9
",
        Scheme::Dark,
    );
    let bases = own(&port);
    let dark = Scheme::Dark;
    assert!(bases.colors.get(Scheme::Light).is_none());
    let roles = bases.colors.get(dark).unwrap();
    let Some(BlockPatch::Fields(view)) = roles.surfaces.as_ref().unwrap().get(SurfaceSet::View)
    else {
        panic!("the view set's roles");
    };
    assert_eq!(view.background, Some(Value::Stated(rgb(1, 2, 3))));
    let keyed = roles.keyed.as_ref().unwrap();
    assert_eq!(keyed.len(), 4);
    assert!(
        keyed
            .iter()
            .all(|(_, value)| matches!(value, Value::Stated(_)))
    );
    // A non-tinting scheme's `[Inactive]` value, as `applyScheme()` writes
    // it with no accent.
    assert_eq!(
        input_color(bases, "inactive.view.background_normal", dark),
        Some(rgb(4, 5, 6))
    );
    assert_eq!(
        input_at(bases, "inactive.window.foreground_normal", dark),
        Some(&Value::Expr(
            crate::expr::Expr::parse("active(role.window.foreground)").unwrap()
        ))
    );
    assert_eq!(
        input_at(bases, "inactive.header.decoration_hover", dark),
        Some(&Value::Expr(
            crate::expr::Expr::parse("active(role.header.decoration_hover)").unwrap()
        ))
    );
}

/// The port's `token.kde` is the file's effects and options, as the KDE
/// reader reads them (S4.8).
#[test]
fn the_port_states_its_scheme_tokens() {
    let port = port(
        "\
[General]
TintFactor=0.25
TitlebarIsAccentColored=true
[ColorEffects:Inactive]
Enable=true
[Colors:View]
BackgroundNormal=1,2,3
",
        Scheme::Light,
    );
    let kde = kde_tokens_of(&port);
    assert_eq!(kde.tint_factor, Some(Some(Value::Stated(0.25))));
    assert_eq!(kde.titlebar_is_accent_colored, Some(Value::Stated(true)));
    assert_eq!(kde.inactive_enabled, Some(Value::Stated(true)));
}

// === Breeze's files ===

/// The source cache: `NATIVE_THEME_SOURCES`, else `scripts/citations.py`'s
/// default, as the facts gate finds it (`tests/facts_gate/facts.rs`).
fn source_cache() -> std::path::PathBuf {
    std::env::var_os("NATIVE_THEME_SOURCES")
        .filter(|value| !value.is_empty())
        .map_or_else(
            || std::path::PathBuf::from("/run/media/tibi/work/cargo-build/native-theme-sources"),
            std::path::PathBuf::from,
        )
}

/// `breeze@6.8.2 colors/<name>`, the file the KDE facts' scheme inputs
/// cite at its scheme (`docs/facts/kde/inputs.toml:1-4`), from the source
/// cache.
fn breeze_file(name: &str) -> Vec<u8> {
    let path = source_cache().join("breeze@6.8.2/colors").join(name);
    std::fs::read(&path).unwrap_or_else(|error| {
        panic!(
            "{} cannot be read ({error}): the source cache (NATIVE_THEME_SOURCES, else its \
             default) lacks the entry breeze@6.8.2, which \
             `scripts/update_sources.sh --only breeze@6.8.2` fetches \
             (CI: .github/actions/sources)",
            path.display()
        )
    })
}

/// The patch that sets each field of `tokens`.
fn tokens_patch(tokens: &KdeTokens) -> KdeTokensPatch {
    let effects = |effects: &KdeEffects| KdeEffectsPatch {
        intensity: Some(effects.intensity),
        intensity_amount: Some(effects.intensity_amount.clone()),
        color: Some(effects.color),
        color_amount: Some(effects.color_amount.clone()),
        color_color: Some(effects.color_color.clone()),
        contrast: Some(effects.contrast),
        contrast_amount: Some(effects.contrast_amount.clone()),
    };
    KdeTokensPatch {
        disabled: Some(effects(&tokens.disabled)),
        inactive: Some(effects(&tokens.inactive)),
        inactive_enabled: Some(tokens.inactive_enabled.clone()),
        inactive_change_selection_color: Some(tokens.inactive_change_selection_color.clone()),
        tint_factor: Some(tokens.tint_factor.clone()),
        titlebar_is_accent_colored: Some(tokens.titlebar_is_accent_colored.clone()),
    }
}

/// `from_port` of Breeze's file at `scheme` gives `kde-breeze`'s own scheme
/// inputs, their lists and `token.kde` there.
fn assert_breeze_port(name: &str, scheme: Scheme) {
    let port = port_of(&breeze_file(name), scheme);
    let bases = own(&port);
    let preset = crate::toml::layer::read_preset(PresetId::KdeBreeze.source()).unwrap();
    let own_inputs = &preset.layer.bases.inputs;
    let is_scheme_input = |name: &InputName| name.to_string().starts_with("kde_scheme.");
    let mut compared = 0;
    for (name, value) in own_inputs
        .entries
        .iter()
        .filter(|(n, _)| is_scheme_input(n))
    {
        let expected = value.per_scheme.get(scheme);
        let found = input_at(bases, &name.to_string(), scheme);
        match (expected, found) {
            (Some(Value::Stated(expected)), Some(Value::Stated(found))) => assert_eq!(
                expected.to_srgb8_clamped(),
                found.to_srgb8_clamped(),
                "{name}"
            ),
            (expected, found) => assert_eq!(expected, found, "{name}"),
        }
        compared += 1;
    }
    let ported = bases
        .inputs
        .entries
        .iter()
        .filter(|(n, _)| is_scheme_input(n))
        .count();
    // Seven sets' twelve keys, active and inactive, and the four `[WM]` keys.
    assert_eq!((compared, ported), (7 * 12 * 2 + 4, 7 * 12 * 2 + 4));
    // The lists are sets: compared whatever their order.
    let sets = |lists: &SchemeMap<Vec<SurfaceSet>>| {
        let mut names: Vec<_> = lists
            .get(scheme)
            .unwrap()
            .iter()
            .map(|set| set.name())
            .collect();
        names.sort_unstable();
        names
    };
    assert_eq!(sets(&bases.inputs.kde_groups), sets(&own_inputs.kde_groups));
    assert_eq!(
        sets(&bases.inputs.kde_inactive_groups),
        sets(&own_inputs.kde_inactive_groups)
    );
    let keys = |lists: &SchemeMap<Vec<InputName>>| {
        let mut names: Vec<_> = lists
            .get(scheme)
            .unwrap()
            .iter()
            .map(ToString::to_string)
            .collect();
        names.sort_unstable();
        names
    };
    assert_eq!(
        keys(&bases.inputs.kde_unset_keys),
        keys(&own_inputs.kde_unset_keys)
    );
    let Some(BlockPatch::Set(Value::Stated(own_kde))) = &preset.layer.bases.tokens.kde else {
        panic!(
            "kde-breeze's token.kde: {:?}",
            preset.layer.bases.tokens.kde
        );
    };
    assert_eq!(kde_tokens_of(&port), &tokens_patch(own_kde));
}

#[test]
fn from_port_of_breeze_light_is_kde_breezes_own_at_light() {
    assert_breeze_port("BreezeLight.colors", Scheme::Light);
}

#[test]
fn from_port_of_breeze_dark_is_kde_breezes_own_at_dark() {
    assert_breeze_port("BreezeDark.colors", Scheme::Dark);
}
