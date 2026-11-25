//! The public part reads on themes built for the case.

use native_theme::next::{
    Color, Paint, PartPatch, Preset, Stroke, StrokeAlign, StrokeSide, Theme, ThemeContext,
    ThemePatch, WidgetState,
};

use super::root_parity::PRESETS;

const STROKE: Color = Color::new(0.1, 0.2, 0.9, 0.5);

/// Each public read gives what the mapping's own read of the part gives,
/// `None` where that read has no value, on every preset and a button's
/// states.
#[test]
fn the_public_part_reads_are_the_mappings_own() {
    let states = [
        WidgetState::NONE,
        WidgetState::HOVER,
        WidgetState::PRESSED,
        WidgetState::DISABLED,
        WidgetState::CHECKED,
    ];
    for &(key, preset) in PRESETS {
        for ctx in [ThemeContext::light(), ThemeContext::dark()] {
            let r = Theme::preset(preset).resolve(&ctx);
            for state in states {
                let part = r.button.body(state);
                let text = r.button.label(state);
                let at = format!("{key} {state:?}");
                assert_eq!(
                    super::fill_of(&part, "h"),
                    Ok(super::build::fill(&part, "h", &mut Vec::new())
                        .map(crate::convert::to_color32)),
                    "{at}"
                );
                assert_eq!(
                    super::state_fill_of(&part, "h"),
                    Ok(super::build::state_fill(&part, "h", &mut Vec::new())
                        .map(crate::convert::to_color32)),
                    "{at}"
                );
                assert_eq!(
                    super::text_color_of(&text),
                    super::text_color(&text, "h").ok(),
                    "{at}"
                );
                assert_eq!(
                    super::state_text_color_of(&text),
                    text.from_state
                        .contains(native_theme::next::TextStated::COLOR)
                        .then(|| super::text_color_of(&text))
                        .flatten(),
                    "{at}"
                );
            }
        }
    }
}

/// A checked toolbar button on either macOS preset takes the checked fill
/// the preset states, `toolbar.button(CHECKED).fill`, the toolbar-bezel
/// button's as an on toggle: macos-monterey #dfdfdf / #414141, macos-tahoe
/// the accent, #007aff (os-probe run 45634379188, toolbar states sheet,
/// `*.normal.toolbar_toggle_on` `fill`).
#[test]
fn a_checked_macos_toolbar_button_takes_the_stated_checked_fill() {
    for (preset, light, dark) in [
        (
            Preset::MacosMonterey,
            [0xdf, 0xdf, 0xdf],
            [0x41, 0x41, 0x41],
        ),
        (Preset::MacosTahoe, [0x00, 0x7a, 0xff], [0x00, 0x7a, 0xff]),
    ] {
        for (ctx, [r, g, b]) in [(ThemeContext::light(), light), (ThemeContext::dark(), dark)] {
            let theme = Theme::preset(preset).resolve(&ctx);
            assert_eq!(
                super::toolbar_button_checked_background(&theme),
                Ok(Some(egui::Color32::from_rgb(r, g, b))),
                "{preset:?} {:?}",
                ctx.scheme
            );
        }
    }
}

/// A checked toolbar button on either macOS preset takes, at rest, under the pointer and
/// pressed, the fills and labels the preset states for `toolbar.button(CHECKED)`,
/// `(HOVER|CHECKED)` and `(PRESSED|CHECKED)` and their labels, the toolbar-bezel button's as an
/// on toggle (os-probe run 45634379188, `controls-toolbar_states_sheet.toml`,
/// `*.{normal,hovered,pressed}.toolbar_toggle_on` `fill`, `label`;
/// native-theme/src/next/presets/macos.rs, `toolbar_checked_off_accent`,
/// `toolbar_checked_on_accent`, `toolbar_checked_label`), picked by the previous response's
/// state; ios-13, which states neither for `CHECKED`, none, and egui's own selected look stays.
#[test]
fn a_checked_macos_toolbar_button_takes_its_stated_hovered_and_pressed_looks()
-> Result<(), super::Unmapped> {
    let rgb = |[r, g, b]: [u8; 3]| egui::Color32::from_rgb(r, g, b);
    let white = [0xff; 3];
    for (preset, ctx, fill, text) in [
        (
            Preset::MacosMonterey,
            ThemeContext::light(),
            [[0xdf; 3], [0xc6; 3], [0xb9; 3]],
            [[0x42; 3], [0x34; 3], [0x34; 3]],
        ),
        (
            Preset::MacosMonterey,
            ThemeContext::dark(),
            [[0x41; 3], [0x5a; 3], [0x67; 3]],
            [[0xec; 3], [0xef; 3], [0xf0; 3]],
        ),
        (
            Preset::MacosTahoe,
            ThemeContext::light(),
            [[0x00, 0x7a, 0xff], [0x00, 0x6d, 0xf2], [0x00, 0x61, 0xe6]],
            [white; 3],
        ),
        (
            Preset::MacosTahoe,
            ThemeContext::dark(),
            [[0x00, 0x7a, 0xff], [0x14, 0x8e, 0xff], [0x23, 0x9d, 0xff]],
            [white; 3],
        ),
    ] {
        let at = format!("{preset:?} {:?}", ctx.scheme);
        let look = super::toolbar_button_checked(&Theme::preset(preset).resolve(&ctx))?;
        assert_eq!(look.fill, Some(fill.map(rgb)), "{at}");
        assert_eq!(look.text, Some(text.map(rgb)), "{at}");
        let [rest, hover, press] =
            [0, 1, 2].map(|i| (fill.get(i).copied().map(rgb), text.get(i).copied().map(rgb)));
        assert_eq!(look.at(false, false), rest, "{at} at rest");
        assert_eq!(look.at(true, false), hover, "{at} hovered");
        assert_eq!(look.at(true, true), press, "{at} pressed");
        assert_eq!(look.at(false, true), press, "{at} pressed off the pointer");
    }
    let ios = super::toolbar_button_checked(
        &Theme::preset(Preset::Ios13).resolve(&ThemeContext::light()),
    )?;
    assert_eq!((ios.fill, ios.text), (None, None), "ios-13");
    assert_eq!(ios.at(true, true), (None, None), "ios-13 pressed");
    Ok(())
}

/// A stated uniform stroke gives its colour and width in every alignment; a
/// per-side stroke gives neither, which egui cannot draw as one stroke.
#[test]
fn a_stroke_is_read_as_stated_and_a_per_side_one_not_at_all() {
    let ctx = ThemeContext::light();
    for align in [
        StrokeAlign::Outside,
        StrokeAlign::Inside,
        StrokeAlign::Center,
    ] {
        let patch = ThemePatch::new().button(
            native_theme::next::ButtonRule::new()
                .body(PartPatch::new().stroke(Stroke::uniform_aligned(2.0, STROKE, align))),
        );
        let r = Theme::preset(Preset::KdeBreeze)
            .with_patch(patch)
            .resolve(&ctx);
        let part = r.button.body(WidgetState::NONE);
        assert_eq!(
            super::stroke_color_of(&part),
            Some(super::to_color32(STROKE)),
            "{align:?}"
        );
        assert_eq!(super::stroke_width_of(&part), Some(2.0), "{align:?}");
    }
    let side = || StrokeSide {
        width: 1.0,
        paint: Paint::Solid(STROKE),
    };
    let patch = ThemePatch::new().button(native_theme::next::ButtonRule::new().body(
        PartPatch::new().stroke(Stroke::sides(
            side(),
            side(),
            side(),
            side(),
            StrokeAlign::Inside,
        )),
    ));
    let r = Theme::preset(Preset::KdeBreeze)
        .with_patch(patch)
        .resolve(&ctx);
    let part = r.button.body(WidgetState::NONE);
    assert_eq!(super::stroke_color_of(&part), None);
    assert_eq!(super::stroke_width_of(&part), None);
}

/// A stroke read is handed to egui over the fill under it by its alignment: windows-11's primary
/// button outline, `#0000000f` laid `Inside`, composited over the primary fill; the same colour
/// laid `Outside`, or over no fill, as stated.
#[test]
fn a_translucent_inside_stroke_is_handed_over_the_fill() -> Result<(), String> {
    let r = Theme::preset(Preset::Windows11).resolve(&ThemeContext::light());
    let part = r
        .button
        .body(native_theme::next::ButtonQuery::primary().with_state(WidgetState::NONE));
    let (Some(color), Some(fill)) = (super::stroke_color_of(&part), super::fill_color(&part))
    else {
        return Err(format!(
            "windows-11 states no primary button outline and fill: {part:?}"
        ));
    };
    assert!(
        matches!(
            part.stroke,
            Stroke::Uniform {
                align: StrokeAlign::Inside,
                ..
            }
        ) && color.a() < u8::MAX,
        "{part:?}"
    );
    assert_eq!(
        super::stroke_color_over(&part, color, Some(fill)),
        fill.blend(color)
    );
    assert_ne!(fill.blend(color), color);
    assert_eq!(super::stroke_color_over(&part, color, None), color);
    let patch = ThemePatch::new().button(
        native_theme::next::ButtonRule::new()
            .variant(native_theme::next::ButtonVariant::Primary)
            .body(PartPatch::new().stroke(Stroke::uniform_aligned(
                1.0,
                STROKE,
                StrokeAlign::Outside,
            ))),
    );
    let r = Theme::preset(Preset::Windows11)
        .with_patch(patch)
        .resolve(&ThemeContext::light());
    let part = r
        .button
        .body(native_theme::next::ButtonQuery::primary().with_state(WidgetState::NONE));
    let outside = super::to_color32(STROKE);
    assert_eq!(
        super::stroke_color_over(&part, outside, Some(fill)),
        outside
    );
    Ok(())
}

/// Layers that are all gradients have no single colour: the fill reads
/// report them `Unmapped` at the home they are given (spec §10, rule 4),
/// never `None`, egui's own fill, in their place.
#[test]
fn an_all_gradient_fill_is_unmapped_at_its_home() {
    use native_theme::next::{ButtonRule, GradientStop, PaintLayer};
    let gradient = Paint::layers(vec![PaintLayer::LinearGradient {
        angle_deg: 90.0,
        stops: std::sync::Arc::from(vec![
            GradientStop {
                offset: 0.0,
                color: STROKE,
            },
            GradientStop {
                offset: 1.0,
                color: Color::TRANSPARENT,
            },
        ]),
        blend: Default::default(),
    }]);
    let fill = PartPatch::new().fill(gradient);
    let patch = ThemePatch::new()
        .button(ButtonRule::new().body(fill.clone()))
        .button(ButtonRule::new().states(WidgetState::HOVER).body(fill));
    let r = Theme::preset(Preset::KdeBreeze)
        .with_patch(patch)
        .resolve(&ThemeContext::light());
    assert_eq!(
        super::fill_of(&r.button.body(WidgetState::NONE), "at rest"),
        Err(super::Unmapped("at rest"))
    );
    assert_eq!(
        super::state_fill_of(&r.button.body(WidgetState::HOVER), "hovered"),
        Err(super::Unmapped("hovered"))
    );
}

/// A fill the atlas builds from, stated as layers that are all gradients, is a
/// `Note::Unmapped` at its home in the atlas's notes (spec §10, rule 4), at rest, for a
/// state, and for a checked box's hovered fill a rule states for `HOVER` alone; the preset's
/// own fills give no such note.
#[test]
fn the_atlas_notes_an_all_gradient_fill_at_its_home() -> Result<(), Box<dyn std::error::Error>> {
    let unmapped = |patch: &str| -> Result<Vec<&'static str>, Box<dyn std::error::Error>> {
        Ok(patched_light(Preset::KdeBreeze, patch)?
            .notes()
            .iter()
            .filter_map(|note| match note {
                crate::Note::Unmapped { path } => Some(*path),
                _ => None,
            })
            .collect())
    };
    assert_eq!(unmapped("")?, Vec::<&str>::new());
    let gradient = "{ kind = \"linear_gradient\", angle_deg = 0.0, stops = [{ offset = 0.0, color = \"#ff0000\" }, { offset = 1.0, color = \"#0000ff\" }] }";
    let notes = unmapped(&format!(
        "[[widgets.menu.rules]]\nstates = []\npanel.fill = {gradient}\n\
         [[widgets.toolbar.rules]]\nstates = [\"hover\"]\nbutton.fill = {gradient}\n\
         [[widgets.checkbox.rules]]\nstates = [\"hover\"]\nindicator.fill = {gradient}\n"
    ))?;
    for home in [
        "menu.panel(NONE).fill",
        "toolbar.button(HOVER).fill",
        "checkbox.indicator(HOVER).fill",
        "checkbox.indicator(HOVER|CHECKED).fill",
    ] {
        assert!(notes.contains(&home), "{home}: {notes:?}");
    }
    Ok(())
}

/// `to_theme` builds each scheme from its own theme, which the atlas then holds
/// (`next_resolved_for`; `resolved_for` is `None`): the scheme's focus ring is
/// the one its theme's ring source builds, and its title bar frame fills from
/// the theme resolved for a window that is not active, where that theme states
/// a fill. The atlas's preferences are the light theme's context's, and a
/// reduced motion stops egui's animations.
#[test]
fn to_theme_builds_each_scheme_from_its_own_theme() -> Result<(), super::Unmapped> {
    use native_theme::next::{MotionPreference, WindowState};

    for &(key, preset) in PRESETS {
        let theme = Theme::preset(preset);
        let mut light_ctx = ThemeContext::light();
        light_ctx.motion = MotionPreference::Reduced;
        let contexts = [light_ctx, ThemeContext::dark()];
        let [light, dark] = contexts.clone().map(|ctx| theme.resolve(&ctx));
        let [light_inactive, dark_inactive] =
            contexts.map(|ctx| theme.resolve(&ctx.with_window_active(false)));
        let atlas = super::to_theme(&light, &light_inactive, &dark, &dark_inactive)?;
        assert!(atlas.accessibility().reduce_motion, "{key}");
        for (scheme, r, inactive) in [
            (egui::Theme::Light, &light, &light_inactive),
            (egui::Theme::Dark, &dark, &dark_inactive),
        ] {
            let at = format!("{key} {scheme:?}");
            assert_eq!(atlas.next_resolved_for(scheme), Some(r), "{at}");
            assert!(atlas.resolved_for(scheme).is_none(), "{at}");
            let ring = crate::plugin::build_focus_ring_from(
                &super::build::focus_ring_source(r)?,
                &mut Vec::new(),
            );
            assert_eq!(atlas.scheme(scheme).focus_ring, ring, "{at}");
            if let Some(fill) = super::build::fill(
                &inactive.window.title_bar(WindowState::default()),
                "window.title_bar(normal).fill",
                &mut Vec::new(),
            ) {
                assert_eq!(
                    atlas
                        .surface_frame(scheme, crate::Surface::WindowTitleBar)
                        .fill,
                    crate::convert::to_color32(fill),
                    "{at}"
                );
            }
            let animation_time = atlas.scheme(scheme).base.animation_time;
            match scheme {
                egui::Theme::Light => assert_eq!(animation_time, 0.0, "{at}"),
                egui::Theme::Dark => assert_eq!(
                    animation_time,
                    egui::Style::default().animation_time,
                    "{at}"
                ),
            }
        }
    }
    Ok(())
}

/// The atlas `to_theme` builds from `preset` with `patch` laid over it.
fn patched_light(
    preset: Preset,
    patch: &str,
) -> Result<crate::ThemeAtlas, Box<dyn std::error::Error>> {
    atlas_of(&Theme::preset(preset).with_patch(ThemePatch::from_toml(patch)?))
}

/// The atlas `to_theme` builds from `theme`, light and dark.
fn atlas_of(theme: &Theme) -> Result<crate::ThemeAtlas, Box<dyn std::error::Error>> {
    let [light, light_inactive, dark, dark_inactive] = [
        ThemeContext::light(),
        ThemeContext::light().with_window_active(false),
        ThemeContext::dark(),
        ThemeContext::dark().with_window_active(false),
    ]
    .map(|ctx| theme.resolve(&ctx));
    let atlas = super::to_theme(&light, &light_inactive, &dark, &dark_inactive)?;
    Ok(atlas)
}

/// The paths of the `Note::Unmapped` notes of the atlas built from kde-breeze with `body` laid
/// over the button's body.
fn unmapped_paths(body: PartPatch) -> Result<Vec<&'static str>, Box<dyn std::error::Error>> {
    let patch = ThemePatch::new().button(native_theme::next::ButtonRule::new().body(body));
    let atlas = atlas_of(&Theme::preset(Preset::KdeBreeze).with_patch(patch))?;
    Ok(atlas
        .notes()
        .iter()
        .filter_map(|note| match note {
            crate::Note::Unmapped { path } => Some(*path),
            _ => None,
        })
        .collect())
}

/// Corners whose radii differ corner by corner are noted unmapped at their home, egui
/// rounding a widget by one radius; equal ones, and the preset's own, are not.
#[test]
fn per_corner_radii_that_differ_are_noted_unmapped() -> Result<(), Box<dyn std::error::Error>> {
    use native_theme::next::{CornerCurve, CornerRadius, Corners};

    let per_corner = |top_left: f32, others: f32| Corners {
        radius: CornerRadius::PerCorner {
            top_left,
            top_right: others,
            bottom_right: others,
            bottom_left: others,
        },
        curve: CornerCurve::Circular,
    };
    assert_eq!(unmapped_paths(PartPatch::new())?, Vec::<&str>::new());
    assert_eq!(
        unmapped_paths(PartPatch::new().corners(per_corner(2.0, 6.0)))?,
        ["button.body(NONE).corners"]
    );
    assert_eq!(
        unmapped_paths(PartPatch::new().corners(per_corner(4.0, 4.0)))?,
        Vec::<&str>::new()
    );
    Ok(())
}

/// Concentric corners are noted unmapped at their home: egui knows no container radius to
/// take the inset from.
#[test]
fn concentric_corners_are_noted_unmapped() -> Result<(), Box<dyn std::error::Error>> {
    use native_theme::next::{CornerCurve, CornerRadius, Corners};

    let concentric = Corners {
        radius: CornerRadius::Concentric {
            inset: 2.0,
            min_radius: 1.0,
        },
        curve: CornerCurve::Circular,
    };
    assert_eq!(
        unmapped_paths(PartPatch::new().corners(concentric))?,
        ["button.body(NONE).corners"]
    );
    Ok(())
}

/// A stroke stated per side is noted unmapped at its width's home, egui outlining a widget by
/// one stroke.
#[test]
fn a_per_side_stroke_is_noted_unmapped() -> Result<(), Box<dyn std::error::Error>> {
    let side = || StrokeSide {
        width: 1.0,
        paint: Paint::Solid(STROKE),
    };
    let stroke = Stroke::sides(side(), side(), side(), side(), StrokeAlign::Inside);
    assert_eq!(
        unmapped_paths(PartPatch::new().stroke(stroke))?,
        ["button.body(NONE).stroke.width"]
    );
    Ok(())
}

/// Each form of a stated radius as the one-radius read takes it: a continuous curve drawn as
/// the circular corner of its radius (spec §10, rule 2), a pill as the largest radius egui
/// holds, which epaint clamps to half the shorter side, per-corner radii where the four are
/// one; per-corner radii that differ and a concentric radius, which needs the container's,
/// not. On a patched theme a continuous panel's radius reaches material-3's menu outline.
#[test]
fn every_radius_form_is_read_as_egui_rounds_it() -> Result<(), Box<dyn std::error::Error>> {
    use native_theme::next::{CornerCurve, CornerRadius, Corners, PartStated, PartStyle};

    let part = |radius: CornerRadius, curve: CornerCurve| {
        let mut part = PartStyle::default();
        part.corners = Corners { radius, curve };
        part.stated |= PartStated::CORNERS;
        part
    };
    let per_corner = |top_left, top_right, bottom_right, bottom_left| CornerRadius::PerCorner {
        top_left,
        top_right,
        bottom_right,
        bottom_left,
    };
    for curve in [CornerCurve::Circular, CornerCurve::Continuous] {
        let read = |radius| super::radius_of(&part(radius, curve));
        assert_eq!(read(CornerRadius::Uniform(4.0)), Some(4.0));
        assert_eq!(read(CornerRadius::None), Some(0.0));
        assert_eq!(read(CornerRadius::Pill), Some(255.0));
        assert_eq!(read(per_corner(3.0, 3.0, 3.0, 3.0)), Some(3.0));
        assert_eq!(read(per_corner(1.0, 2.0, 3.0, 4.0)), None);
        assert_eq!(
            read(CornerRadius::Concentric {
                inset: 2.0,
                min_radius: 1.0,
            }),
            None
        );
    }
    assert_eq!(super::radius_of(&PartStyle::default()), None);

    let panel =
        "[[widgets.menu.rules]]\npanel.corners = { radius = 4.0, curve = \"continuous\" }\n";
    let r = Theme::preset(Preset::Material3)
        .with_patch(ThemePatch::from_toml(panel)?)
        .resolve(&ThemeContext::light());
    assert_eq!(
        super::build::cell_source(&r, &mut Vec::new())
            .menu
            .outline
            .edge
            .radius,
        Some(4.0)
    );
    Ok(())
}

/// The atlas rounds the button by each form of its body's corners egui can draw, and notes
/// none of them: a pill by the largest radius egui holds (epaint clamps it to half the
/// shorter side), a continuous curve as the circular corner of its radius, per-corner radii
/// that are one by that radius.
#[test]
fn the_atlas_rounds_a_pill_a_continuous_curve_and_equal_corners()
-> Result<(), Box<dyn std::error::Error>> {
    use native_theme::next::{ButtonRule, CornerCurve, CornerRadius, Corners};

    let rounded = |corners: Corners| -> Result<_, Box<dyn std::error::Error>> {
        let patch =
            ThemePatch::new().button(ButtonRule::new().body(PartPatch::new().corners(corners)));
        let atlas = atlas_of(&Theme::preset(Preset::KdeBreeze).with_patch(patch))?;
        let mut style = egui::Style::default();
        atlas
            .role_modifier(
                egui::Theme::Light,
                crate::Role::Button,
                crate::RoleVariant::Normal,
            )
            .apply(&mut style);
        let unmapped = atlas
            .notes()
            .iter()
            .any(|note| matches!(note, crate::Note::Unmapped { .. }));
        Ok((style.visuals.widgets.inactive.corner_radius, unmapped))
    };
    let equal = Corners {
        radius: CornerRadius::PerCorner {
            top_left: 3.0,
            top_right: 3.0,
            bottom_right: 3.0,
            bottom_left: 3.0,
        },
        curve: CornerCurve::Circular,
    };
    for (corners, want) in [
        (Corners::pill(), u8::MAX),
        (Corners::continuous(6.0), 6),
        (equal, 3),
    ] {
        assert_eq!(
            rounded(corners)?,
            (egui::CornerRadius::same(want), false),
            "{corners:?}"
        );
    }
    Ok(())
}

/// `text_area_frame` rounds the field by the corners egui can draw, a pill and per-corner radii
/// that are one, and returns a `Note::Unmapped` at `text_area.field(NONE).corners` for
/// per-corner radii that differ, which keep the scope's radius.
#[test]
fn the_text_area_frame_notes_the_corners_it_cannot_draw() -> Result<(), Box<dyn std::error::Error>>
{
    use native_theme::next::{CornerCurve, CornerRadius, Corners, InputRule};

    let per_corner = |top_left: f32, others: f32| Corners {
        radius: CornerRadius::PerCorner {
            top_left,
            top_right: others,
            bottom_right: others,
            bottom_left: others,
        },
        curve: CornerCurve::Circular,
    };
    let ctx = egui::Context::default();
    let id = egui::Id::unique("text-area-corners");
    let mut frames = Vec::new();
    for corners in [per_corner(4.0, 4.0), Corners::pill(), per_corner(2.0, 6.0)] {
        let patch =
            ThemePatch::new().text_area(InputRule::new().field(PartPatch::new().corners(corners)));
        let r = Theme::preset(Preset::KdeBreeze)
            .with_patch(patch)
            .resolve(&ThemeContext::light());
        let _ = crate::install_tests::pass(&ctx, egui::RawInput::default(), |ui| {
            frames.push(super::text_area_frame(ui, id, &r));
        });
    }
    let [equal, pill, differ] = <[_; 3]>::try_from(frames).map_err(|_| "three frames")?;
    let (equal, pill, (_, differ)) = (equal?, pill?, differ?);
    assert_eq!(
        (equal.0.corner_radius, equal.1),
        (egui::CornerRadius::same(4), Vec::new())
    );
    assert_eq!(
        (pill.0.corner_radius, pill.1),
        (egui::CornerRadius::same(u8::MAX), Vec::new())
    );
    assert_eq!(
        differ,
        [crate::Note::unmapped("text_area.field(NONE).corners")]
    );
    Ok(())
}

/// The primary button in a disabled scope takes the fill and the label a rule states for the
/// primary button's disabled state (`button.body(primary + DISABLED)`,
/// `button.label(primary + DISABLED)`), not the plain button's.
#[test]
fn the_disabled_primary_button_reads_its_own_state() -> Result<(), Box<dyn std::error::Error>> {
    let rule = "[[widgets.button.rules]]\nstates = [\"disabled\"]\n";
    let plain = patched_light(
        Preset::KdeBreeze,
        &format!("{rule}body.fill = \"#ff00ff\"\nlabel.color = \"#00ff00\"\n"),
    )?;
    let primary = patched_light(
        Preset::KdeBreeze,
        &format!(
            "{rule}variant = \"primary\"\nbody.fill = \"#0000ff\"\nlabel.color = \"#ffff00\"\n"
        ),
    )?;
    let cell = primary
        .scheme(egui::Theme::Light)
        .cell(crate::Role::Button, crate::RoleVariant::Disabled);
    assert_eq!(cell.visuals.selection.bg_fill, egui::Color32::BLUE);
    assert_eq!(cell.visuals.selection.stroke.color, egui::Color32::YELLOW);
    let plain_primary = &plain
        .scheme(egui::Theme::Light)
        .cell(crate::Role::Button, crate::RoleVariant::Disabled)
        .visuals
        .selection;
    // a rule for every button's disabled state holds for the primary one too
    assert_eq!(
        plain_primary.bg_fill,
        egui::Color32::from_rgb(0xff, 0, 0xff)
    );
    Ok(())
}

/// A tool button's pressed outline is the one a rule states for `toolbar.button(PRESSED)`;
/// unstated, egui's own pressed outline (spec §10, rule 0), never the hovered one.
#[test]
fn a_tool_buttons_pressed_outline_is_its_own() -> Result<(), Box<dyn std::error::Error>> {
    let hover = "[[widgets.toolbar.rules]]\nstates = [\"hover\"]\n\
                 button.stroke = { width = 1.0, color = \"#ff00ff\" }\n";
    // a preset that states no pressed tool-button outline
    let (_, preset) = PRESETS
        .iter()
        .find(|(_, p)| {
            !Theme::preset(*p)
                .resolve(&ThemeContext::light())
                .toolbar
                .button(WidgetState::PRESSED)
                .from_state
                .contains(native_theme::next::PartStated::STROKE)
        })
        .ok_or("every preset states a pressed tool-button outline")?;
    let styles = patched_light(*preset, hover)?;
    let w = &styles
        .scheme(egui::Theme::Light)
        .cell(crate::Role::Toolbar, crate::RoleVariant::Normal)
        .visuals
        .widgets;
    let magenta = egui::Color32::from_rgb(0xff, 0, 0xff);
    assert_eq!(w.hovered.bg_stroke.color, magenta);
    let own = egui::Theme::Light.default_style().visuals.widgets;
    assert_eq!(w.active.bg_stroke.color, own.active.bg_stroke.color);
    let pressed = format!(
        "{hover}[[widgets.toolbar.rules]]\nstates = [\"pressed\"]\n\
         button.stroke = {{ width = 1.0, color = \"#00ff00\" }}\n"
    );
    let styles = patched_light(Preset::KdeBreeze, &pressed)?;
    let w = &styles
        .scheme(egui::Theme::Light)
        .cell(crate::Role::Toolbar, crate::RoleVariant::Normal)
        .visuals
        .widgets;
    assert_eq!(w.active.bg_stroke.color, egui::Color32::GREEN);
    Ok(())
}

/// The flat button `super::flat_button` lays over the light `Role::Button` `Normal` cell takes
/// under the pointer the outline a rule for the flat variant states for `HOVER`, pressed or
/// focused the one it states for `FOCUS_VISIBLE`, and its stated fills, never the push button's
/// cell's; a flat button stating none of them is egui's own hovered and pressed outline, fill
/// and label (spec §10, rule 0).
#[test]
fn a_flat_buttons_states_are_its_own() -> Result<(), Box<dyn std::error::Error>> {
    use native_theme::next::{ButtonRule, ButtonVariant};

    let [magenta, green, blue] = [[0xff, 0, 0xff], [0, 0xff, 0], [0, 0, 0xff]]
        .map(|[r, g, b]| egui::Color32::from_rgb(r, g, b));
    let flat = |state: WidgetState, body: PartPatch| {
        ButtonRule::new()
            .variant(ButtonVariant::Flat)
            .states(state)
            .body(body)
    };
    let pen = |width: f32, [r, g, b]: [f32; 3]| Stroke::uniform(width, Color::new(r, g, b, 1.0));
    let patch = ThemePatch::new()
        .button(flat(
            WidgetState::HOVER,
            PartPatch::new()
                .stroke(pen(3.0, [1.0, 0.0, 1.0]))
                .fill(Paint::Solid(Color::new(0.0, 0.0, 1.0, 1.0))),
        ))
        .button(flat(
            WidgetState::FOCUS_VISIBLE,
            PartPatch::new().stroke(pen(4.0, [0.0, 1.0, 0.0])),
        ));
    let own = egui::Theme::Light.default_style().visuals.widgets;
    for &(key, preset) in PRESETS {
        let theme = Theme::preset(preset).with_patch(patch.clone());
        let r = theme.resolve(&ThemeContext::light());
        let mut visuals = atlas_of(&theme)?
            .scheme(egui::Theme::Light)
            .cell(crate::Role::Button, crate::RoleVariant::Normal)
            .visuals
            .clone();
        let cell = visuals.widgets.clone();
        let notes = super::flat_button(&r, egui::Theme::Light, &mut visuals);
        assert!(notes.is_empty(), "{key}: {notes:?}");
        let w = &visuals.widgets;
        assert_eq!(
            w.hovered.bg_stroke,
            egui::Stroke::new(3.0, magenta),
            "{key}"
        );
        assert_eq!(w.hovered.weak_bg_fill, blue, "{key}");
        assert_eq!(w.active.bg_stroke, egui::Stroke::new(4.0, green), "{key}");
        assert_eq!(
            w.inactive, cell.inactive,
            "{key}: the resting entry is the scope's"
        );
        assert_ne!(
            cell.hovered.bg_stroke, w.hovered.bg_stroke,
            "{key}: the push button's cell has the flat outline, so this proves nothing"
        );

        // Every preset states the flat button's states (a rule that names no variant is the
        // flat button's too), so the unstated case is a flat button stating none.
        let mut bare = cell.clone();
        let mut notes = Vec::new();
        crate::style::variants::flat_cell(
            &mut bare,
            &crate::style::variants::FlatButton {
                fill: [None; 2],
                label: [None; 2],
                state_border: [None; 2],
                align: crate::style::line::LineAlign::Outside,
            },
            crate::style::states::Unstated::Egui(&own),
            &mut notes,
        );
        assert!(notes.is_empty(), "{key}: {notes:?}");
        for (entry, stock, state) in [
            (&bare.hovered, &own.hovered, "hovered"),
            (&bare.active, &own.active, "pressed"),
        ] {
            assert_eq!(
                (entry.bg_stroke, entry.weak_bg_fill, entry.fg_stroke.color),
                (stock.bg_stroke, stock.weak_bg_fill, stock.fg_stroke.color),
                "{key}: a flat button stating nothing {state} is egui's own there"
            );
        }
    }
    Ok(())
}

/// A focus indicator that is not a ring is none the plugin's ring can draw: `to_theme` is
/// `Unmapped` at `focus_ring.kind`, as iced's ring is.
#[test]
fn a_focus_indicator_that_is_not_a_ring_is_unmapped() -> Result<(), super::Unmapped> {
    use native_theme::next::FocusKind;

    for &(key, preset) in PRESETS {
        let theme = Theme::preset(preset);
        let [mut light, light_inactive, dark, dark_inactive] = [
            ThemeContext::light(),
            ThemeContext::light().with_window_active(false),
            ThemeContext::dark(),
            ThemeContext::dark().with_window_active(false),
        ]
        .map(|ctx| theme.resolve(&ctx));
        super::to_theme(&light, &light_inactive, &dark, &dark_inactive)?;
        for kind in [FocusKind::Fill, FocusKind::Underline] {
            light.focus_ring.kind = kind;
            assert_eq!(
                super::to_theme(&light, &light_inactive, &dark, &dark_inactive).err(),
                Some(super::Unmapped("focus_ring.kind")),
                "{key} {kind:?}"
            );
        }
    }
    Ok(())
}

/// A default button whose fill a rule states is mapped on every preset: each property the
/// theme states is its own, each it leaves unstated egui's own resting button's, a hovered or
/// focused frame colour egui's own hovered or `active` one's (spec §10, rule 0), never an
/// `Unmapped`.
#[test]
fn a_default_buttons_unstated_properties_are_eguis_own() -> Result<(), Box<dyn std::error::Error>> {
    use native_theme::next::PartStated;

    let fill = "[[widgets.button.rules]]\nstates = [\"default\"]\nbody.fill = \"#ff00ff\"\n";
    // a DEFAULT rule stating no frame: no frame at rest (spec §10, rule 2)
    let none = format!("{fill}body.stroke = \"none\"\n");
    let [
        mut own_text,
        mut own_frame,
        mut own_width,
        mut own_hover,
        mut own_hover_width,
    ] = [0; 5];
    for patch in [fill, none.as_str()] {
        let patch = ThemePatch::from_toml(patch)?;
        for &(key, preset) in PRESETS {
            for (ctx, scheme) in [
                (ThemeContext::light(), egui::Theme::Light),
                (ThemeContext::dark(), egui::Theme::Dark),
            ] {
                let r = Theme::preset(preset)
                    .with_patch(patch.clone())
                    .resolve(&ctx);
                let at = format!("{key} {scheme:?}");
                let button = super::default_button(&r)?.ok_or(at.clone())?;
                let stock = scheme.default_style().visuals.widgets;
                let b = &r.button;
                assert_eq!(button.fill, egui::Color32::from_rgb(0xff, 0, 0xff), "{at}");
                let text = super::text_color_of(&b.label(WidgetState::DEFAULT));
                own_text += usize::from(text.is_none());
                assert_eq!(
                    button.text,
                    text.unwrap_or(stock.inactive.fg_stroke.color),
                    "{at}"
                );
                let body = b.body(WidgetState::DEFAULT);
                let frame = if body.stated.contains(PartStated::STROKE) {
                    super::stroke_color_of(&body).unwrap_or(egui::Color32::TRANSPARENT)
                } else {
                    own_frame += 1;
                    stock.inactive.bg_stroke.color
                };
                assert_eq!(button.stroke.color, frame, "{at}");
                let width = super::stroke_width_of(&body);
                own_width += usize::from(width.is_none());
                let width = width.unwrap_or(stock.inactive.bg_stroke.width);
                assert_eq!(button.stroke.width, width, "{at}");
                // a hovered frame stated as none is no line (spec §10, rule 2)
                let hover = b.body(WidgetState::HOVER);
                if hover.from_state.contains(PartStated::STROKE) {
                    let stated =
                        super::state_stroke_color_of(&hover).unwrap_or(egui::Color32::TRANSPARENT);
                    assert_eq!(button.hover_stroke.color, stated, "{at}");
                } else {
                    own_hover += 1;
                    assert_eq!(
                        button.hover_stroke.color, stock.hovered.bg_stroke.color,
                        "{at}"
                    );
                }
                // the hovered width is its own state's, egui's own where the theme states none
                if hover.from_state.contains(PartStated::STROKE) {
                    let stated = super::stroke_width_of(&hover).ok_or(at.clone())?;
                    assert_eq!(button.hover_stroke.width, stated, "{at}");
                } else {
                    own_hover_width += 1;
                    assert_eq!(
                        button.hover_stroke.width, stock.hovered.bg_stroke.width,
                        "{at}"
                    );
                }
            }
        }
    }
    // the presets' and patches' cases, both kinds of hovered frame among them where a preset
    // gives each; each unstated value alone is `each_unstated_default_button_value_is_eguis_own`'s
    assert!(
        own_hover < 4 * PRESETS.len(),
        "no preset states the hovered frame"
    );
    let fallbacks = [own_text, own_frame, own_width, own_hover, own_hover_width];
    println!("label, frame, width, hovered frame, hovered width fallbacks: {fallbacks:?}");
    Ok(())
}

/// No bundled preset leaves the default button's label colour, frame colour or frame width
/// unstated, and a patch cannot unstate one: with each of them unstated, the default button
/// takes egui's own resting button's, and its hovered and focused frames egui's own hovered and
/// `active` ones (spec §10, rule 0).
#[test]
fn each_unstated_default_button_value_is_eguis_own() {
    for scheme in [egui::Theme::Light, egui::Theme::Dark] {
        let stock = scheme.default_style().visuals.widgets;
        let reads = super::DefaultButtonReads {
            fill: egui::Color32::BLUE,
            text: None,
            width: None,
            frame: None,
            hover: None,
            focus: None,
            hover_width: None,
            focus_width: None,
        };
        let button = super::default_button_of(&reads, &stock);
        assert_eq!(button.fill, egui::Color32::BLUE);
        assert_eq!(button.text, stock.inactive.fg_stroke.color, "{scheme:?}");
        assert_eq!(button.stroke, stock.inactive.bg_stroke, "{scheme:?}");
        assert_eq!(button.hover_stroke, stock.hovered.bg_stroke, "{scheme:?}");
        assert_eq!(button.focus_stroke, stock.active.bg_stroke, "{scheme:?}");
    }
}

/// A DEFAULT rule stating the frame as none draws no frame at rest, whatever the resting
/// button's width (spec §10, rule 2), and leaves the hovered and focused frames to their own
/// states.
#[test]
fn a_default_button_stated_without_a_frame_has_none_at_rest()
-> Result<(), Box<dyn std::error::Error>> {
    let patch = "[[widgets.button.rules]]\nstates = [\"default\"]\n\
                 body.fill = \"#ff00ff\"\nbody.stroke = \"none\"\n";
    for &(key, preset) in PRESETS {
        let r = Theme::preset(preset)
            .with_patch(ThemePatch::from_toml(patch)?)
            .resolve(&ThemeContext::light());
        let button = super::default_button(&r)?.ok_or(key)?;
        assert_eq!(
            button.stroke,
            egui::Stroke::new(0.0, egui::Color32::TRANSPARENT),
            "{key}"
        );
    }
    Ok(())
}

/// A rule stating the checked box's hovered outline as none (`indicator(HOVER).stroke`) draws
/// the hovered checked box with no outline: width 0, not egui's own 1.0 (spec §10, rules 0, 2).
#[test]
fn a_checked_boxs_hovered_outline_stated_as_none_is_no_line()
-> Result<(), Box<dyn std::error::Error>> {
    let hover = "[[widgets.checkbox.rules]]\nstates = [\"hover\"]\nindicator.stroke = \"none\"\n";
    for &(key, preset) in PRESETS {
        let atlas = patched_light(preset, hover)?;
        let w = &atlas
            .scheme(egui::Theme::Light)
            .cell(crate::Role::Checkbox, crate::RoleVariant::Selected)
            .visuals
            .widgets;
        assert_eq!(w.hovered.bg_stroke.width, 0.0, "{key}");
    }
    Ok(())
}

/// The atlas of `preset` with its check box's indicator fill and stroke stated for no state at
/// `DISABLED`, in each of the four contexts: a rule of no required state, forbidding every state
/// but `DISABLED` a rest query's look depends on, restates that context's fill and stroke at
/// rest, so they stay as the preset gives them and the `from_state` bits of those queried at
/// `DISABLED` are cleared (spec §2, `from_state`: a rule without required states clears it).
fn disabled_unstated_atlas(
    preset: Preset,
) -> Result<(crate::ThemeAtlas, native_theme::next::ResolvedTheme), Box<dyn std::error::Error>> {
    use native_theme::next::CheckboxRule;
    let others = WidgetState::CHECKED
        | WidgetState::INDETERMINATE
        | WidgetState::HOVER
        | WidgetState::PRESSED;
    let [light, light_inactive, dark, dark_inactive] = [
        ThemeContext::light(),
        ThemeContext::light().with_window_active(false),
        ThemeContext::dark(),
        ThemeContext::dark().with_window_active(false),
    ]
    .map(|ctx| {
        let mut r = Theme::preset(preset).resolve(&ctx);
        let rest = r.checkbox.indicator(WidgetState::NONE);
        r.apply_patch(
            &ThemePatch::new().checkbox(
                CheckboxRule::new()
                    .not_states(others)
                    .indicator(PartPatch::new().fill(rest.fill).stroke(rest.stroke)),
            ),
        );
        r
    });
    let atlas = super::to_theme(&light, &light_inactive, &dark, &dark_inactive)?;
    Ok((atlas, light))
}

/// A disabled check box, checked or not, draws the outline a rule states for the disabled
/// state (`indicator(DISABLED).stroke`), its width and colour; where none does, it keeps its
/// resting outline, which egui dims by `disabled_alpha` alone (spec §2, `from_state`; §10,
/// rule 0): on every preset, each patched to state neither the disabled indicator's fill nor
/// its stroke ([`disabled_unstated_atlas`]), as every bundled preset states one.
#[test]
fn a_disabled_boxs_stated_outline_is_drawn() -> Result<(), Box<dyn std::error::Error>> {
    use native_theme::next::PartStated;

    let rule = "[[widgets.checkbox.rules]]\nstates = [\"disabled\"]\n\
                indicator.stroke = { width = 3.0, color = \"#ff00ff\" }\n";
    let magenta = egui::Color32::from_rgb(0xff, 0x00, 0xff);
    for &(key, preset) in PRESETS {
        let cells = |atlas: &crate::ThemeAtlas| {
            let styles = atlas.scheme(egui::Theme::Light);
            [
                crate::RoleVariant::Normal,
                crate::RoleVariant::Selected,
                crate::RoleVariant::Disabled,
                crate::RoleVariant::SelectedDisabled,
            ]
            .map(|variant| {
                styles
                    .cell(crate::Role::Checkbox, variant)
                    .visuals
                    .widgets
                    .inactive
            })
        };
        let [_, _, disabled, checked] = cells(&patched_light(preset, rule)?);
        for (name, entry) in [("unchecked", disabled), ("checked", checked)] {
            assert_eq!(
                entry.bg_stroke,
                egui::Stroke::new(3.0, magenta),
                "{key}: the {name} disabled box"
            );
        }
        let (atlas, r) = disabled_unstated_atlas(preset)?;
        let stated = r.checkbox.indicator(WidgetState::DISABLED);
        assert!(
            !stated
                .from_state
                .intersects(PartStated::STROKE | PartStated::FILL),
            "{key}: patched, it still states a disabled indicator's stroke or fill"
        );
        let [normal, selected, disabled, checked] = cells(&atlas);
        assert_eq!(disabled.bg_stroke, normal.bg_stroke, "{key}: unchecked");
        assert_eq!(checked.bg_stroke, selected.bg_stroke, "{key}: checked");
    }
    Ok(())
}

/// The mixed box `super::mixed_checkbox` lays over the light `Role::Checkbox` `Normal` cell of
/// `preset` with `patch` laid over it, with the 0.8.0 light theme it lays it from.
fn mixed_over_normal(
    preset: Preset,
    patch: &str,
) -> Result<
    (
        egui::Visuals,
        egui::Visuals,
        native_theme::next::ResolvedTheme,
    ),
    Box<dyn std::error::Error>,
> {
    let atlas = patched_light(preset, patch)?;
    let normal = atlas
        .scheme(egui::Theme::Light)
        .cell(crate::Role::Checkbox, crate::RoleVariant::Normal)
        .visuals
        .clone();
    let r = Theme::preset(preset)
        .with_patch(ThemePatch::from_toml(patch)?)
        .resolve(&ThemeContext::light());
    let mut mixed = normal.clone();
    let notes = super::mixed_checkbox(&r, egui::Theme::Light, &mut mixed);
    assert!(notes.is_empty(), "{preset:?}: {notes:?}");
    Ok((normal, mixed, r))
}

/// The mixed box takes what a rule states for `INDETERMINATE` at rest, and under the pointer and
/// pressed what a rule states for `HOVER` or `PRESSED` asked alone, the combined state's value;
/// where none does, egui's own treatment of the hover or the press, as the checked box takes it
/// (spec §2, `from_state`: the flag asked alone; §10, rule 0): on every preset with the same
/// fill and outline stated for `CHECKED` and for `INDETERMINATE`, the mixed box's hovered and
/// pressed fill and outline colour are the checked cell's, and where `indicator(HOVER)` states
/// no fill, egui's own hovered one where egui's changes between rest and hover, else the mixed
/// fill. A rule for `HOVER` and one for `HOVER` with `INDETERMINATE` give the latter.
#[test]
fn the_mixed_box_under_the_pointer_asks_the_state_alone() -> Result<(), Box<dyn std::error::Error>>
{
    use native_theme::next::{IconStated, PartStated};

    let [magenta, green, blue, cyan] =
        [[0xff, 0, 0xff], [0, 0xff, 0], [0, 0, 0xff], [0, 0xff, 0xff]]
            .map(|[r, g, b]| egui::Color32::from_rgb(r, g, b));
    let look = "indicator.fill = \"#ff00ff\"\n\
                indicator.stroke = { width = 3.0, color = \"#00ff00\" }\n";
    let rules = format!(
        "[[widgets.checkbox.rules]]\nstates = [\"indeterminate\"]\n{look}checkmark.color = \"#0000ff\"\n\
         [[widgets.checkbox.rules]]\nstates = [\"checked\"]\n{look}"
    );
    let stock = egui::Theme::Light.default_style().visuals.widgets;
    let mut unstated_hover = 0;
    for &(key, preset) in PRESETS {
        let (_, mixed, r) = mixed_over_normal(preset, &rules)?;
        let w = &mixed.widgets;
        assert_eq!(w.inactive.bg_fill, magenta, "{key}: the mixed fill");
        assert_eq!(
            w.inactive.bg_stroke,
            egui::Stroke::new(3.0, green),
            "{key}: the mixed outline"
        );
        assert_eq!(w.inactive.fg_stroke.color, blue, "{key}: the mixed mark");
        let checked = patched_light(preset, &rules)?;
        let checked = &checked
            .scheme(egui::Theme::Light)
            .cell(crate::Role::Checkbox, crate::RoleVariant::Selected)
            .visuals
            .widgets;
        for (name, state, m, c) in [
            ("hovered", WidgetState::HOVER, w.hovered, checked.hovered),
            ("pressed", WidgetState::PRESSED, w.active, checked.active),
        ] {
            let alone = r.checkbox.indicator(state).from_state;
            if alone.contains(PartStated::FILL) {
                let combined = r.checkbox.indicator(state | WidgetState::INDETERMINATE);
                assert_eq!(
                    Some(m.bg_fill),
                    super::fill_of(&combined, "combined")?,
                    "{key}: the {name} fill stated for the state"
                );
            } else {
                assert_eq!(m.bg_fill, c.bg_fill, "{key}: the {name} fill");
            }
            if !alone.contains(PartStated::STROKE) {
                assert_eq!(
                    m.bg_stroke.color, c.bg_stroke.color,
                    "{key}: the {name} outline"
                );
            }
        }
        if !r
            .checkbox
            .indicator(WidgetState::HOVER)
            .from_state
            .contains(PartStated::FILL)
        {
            unstated_hover += 1;
            let own = if stock.hovered.bg_fill == stock.inactive.bg_fill {
                magenta
            } else {
                stock.hovered.bg_fill
            };
            assert_eq!(w.hovered.bg_fill, own, "{key}: egui's own hover");
        }
        if !r
            .checkbox
            .checkmark(WidgetState::HOVER)
            .from_state
            .contains(IconStated::COLOR)
        {
            let own = if stock.hovered.fg_stroke.color == stock.inactive.fg_stroke.color {
                blue
            } else {
                stock.hovered.fg_stroke.color
            };
            assert_eq!(w.hovered.fg_stroke.color, own, "{key}: the hovered mark");
        }

        let hover = format!(
            "{rules}[[widgets.checkbox.rules]]\nstates = [\"hover\"]\nindicator.fill = \"#ffff00\"\n\
             [[widgets.checkbox.rules]]\nstates = [\"hover\", \"indeterminate\"]\n\
             indicator.fill = \"#00ffff\"\n"
        );
        let (_, mixed, _) = mixed_over_normal(preset, &hover)?;
        assert_eq!(
            mixed.widgets.hovered.bg_fill, cyan,
            "{key}: the fill stated for the hover and the mixed state"
        );
    }
    assert!(
        unstated_hover > 0,
        "no preset leaves the hovered indicator's fill unstated"
    );
    Ok(())
}

/// A see-through outline stated for the mixed state is drawn over the fill under it, as the
/// connector's cells draw one ([`crate::style::line::Line::over`]): one inside the box's edge
/// is composited over the box's fill (egui fills a rect up to the stroke's inner edge, epaint
/// 0.37.0 `src/tessellator.rs`, `stroke_and_fill_path`), one outside it is handed to egui as
/// stated. Where a rule states a mark, the label keeps the colour it has at rest.
#[test]
fn a_translucent_mixed_outline_is_drawn_over_its_fill() -> Result<(), Box<dyn std::error::Error>> {
    let magenta = egui::Color32::from_rgb(0xff, 0x00, 0xff);
    let half_green = egui::Color32::from_rgba_unmultiplied(0x00, 0xff, 0x00, 0x80);
    let rule = |align: &str| {
        format!(
            "[[widgets.checkbox.rules]]\nstates = [\"indeterminate\"]\n\
             indicator.fill = \"#ff00ff\"\n\
             indicator.stroke = {{ width = 2.0, color = \"#00ff0080\", align = \"{align}\" }}\n\
             checkmark.color = \"#0000ff\"\n"
        )
    };
    for &(key, preset) in PRESETS {
        let (normal, inside, r) = mixed_over_normal(preset, &rule("inside"))?;
        assert_eq!(
            inside.widgets.inactive.bg_stroke.color,
            magenta.blend(half_green),
            "{key}: inside"
        );
        let (_, outside, _) = mixed_over_normal(preset, &rule("outside"))?;
        assert_eq!(
            outside.widgets.inactive.bg_stroke.color, half_green,
            "{key}: outside"
        );
        let label = normal
            .override_text_color
            .unwrap_or(normal.widgets.inactive.fg_stroke.color);
        assert_eq!(inside.override_text_color, Some(label), "{key}: the label");
        // a scope whose label egui letters in the mark's colour
        let mut bare = normal.clone();
        bare.override_text_color = None;
        let notes = super::mixed_checkbox(&r, egui::Theme::Light, &mut bare);
        assert!(notes.is_empty(), "{key}: {notes:?}");
        assert_eq!(
            bare.override_text_color,
            Some(normal.widgets.inactive.fg_stroke.color),
            "{key}: the label held to its resting colour"
        );
    }
    Ok(())
}

/// The checked box's outline is drawn at the width a rule states for `CHECKED`, not the
/// unchecked box's, in every entry at rest, and under the pointer where a rule states the
/// outline for `HOVER` asked alone (the combined state's, the `CHECKED` rule's here).
#[test]
fn the_checked_boxs_stated_outline_width_is_drawn() -> Result<(), Box<dyn std::error::Error>> {
    use native_theme::next::PartStated;

    let green = egui::Color32::from_rgb(0, 0xff, 0);
    let rule = "[[widgets.checkbox.rules]]\nstates = [\"checked\"]\n\
                indicator.stroke = { width = 3.0, color = \"#00ff00\" }\n";
    for &(key, preset) in PRESETS {
        let atlas = patched_light(preset, rule)?;
        let w = &atlas
            .scheme(egui::Theme::Light)
            .cell(crate::Role::Checkbox, crate::RoleVariant::Selected)
            .visuals
            .widgets;
        for (name, e) in [
            ("noninteractive", &w.noninteractive),
            ("inactive", &w.inactive),
            ("open", &w.open),
        ] {
            assert_eq!(e.bg_stroke.width, 3.0, "{key}: the {name} outline's width");
            assert_eq!(
                e.bg_stroke.color, green,
                "{key}: the {name} outline's colour"
            );
        }
        let r = Theme::preset(preset)
            .with_patch(ThemePatch::from_toml(rule)?)
            .resolve(&ThemeContext::light());
        if r.checkbox
            .indicator(WidgetState::HOVER)
            .from_state
            .contains(PartStated::STROKE)
        {
            assert_eq!(
                w.hovered.bg_stroke.width, 3.0,
                "{key}: the hovered outline's width"
            );
        }
    }
    Ok(())
}

/// The checked box's and the mixed box's mark take the width a rule states for their flag at
/// rest, and under the pointer the width a rule states for `HOVER` asked alone, the
/// combined state's (spec §2, `from_state`); with no rule for `HOVER` or `PRESSED`, egui's own
/// width for the state where egui's changes between rest and that state, else the resting width
/// (§10, rule 0): egui strokes the mark in the state's `fg_stroke` (egui 0.37.0
/// `src/theme/default_style.rs`, `StyleProvider<CheckboxStyle>`), and its own entries widen
/// that stroke under the pointer and on a press (`src/style.rs`, `Widgets::light`).
#[test]
fn a_marks_stated_width_is_read_per_state() -> Result<(), Box<dyn std::error::Error>> {
    let stock = egui::Theme::Light.default_style().visuals.widgets;
    let own = |e: &egui::style::WidgetVisuals, rest: f32| {
        if e.fg_stroke.width == stock.inactive.fg_stroke.width {
            rest
        } else {
            e.fg_stroke.width
        }
    };
    for flag in ["checked", "indeterminate"] {
        let rest = format!(
            "[[widgets.checkbox.rules]]\nstates = [\"{flag}\"]\ncheckmark.stroke_width = 3.0\n"
        );
        let hover = format!(
            "{rest}[[widgets.checkbox.rules]]\nstates = [\"hover\"]\ncheckmark.stroke_width = 4.0\n\
             [[widgets.checkbox.rules]]\nstates = [\"hover\", \"{flag}\"]\n\
             checkmark.stroke_width = 5.0\n"
        );
        for &(key, preset) in PRESETS {
            let marks = |patch: &str| -> Result<egui::style::Widgets, Box<dyn std::error::Error>> {
                Ok(if flag == "checked" {
                    patched_light(preset, patch)?
                        .scheme(egui::Theme::Light)
                        .cell(crate::Role::Checkbox, crate::RoleVariant::Selected)
                        .visuals
                        .widgets
                        .clone()
                } else {
                    mixed_over_normal(preset, patch)?.1.widgets
                })
            };
            let w = marks(&rest)?;
            for (name, e) in [
                ("noninteractive", &w.noninteractive),
                ("inactive", &w.inactive),
                ("open", &w.open),
            ] {
                assert_eq!(e.fg_stroke.width, 3.0, "{key} {flag}: the {name} mark");
            }
            assert_eq!(
                w.hovered.fg_stroke.width,
                own(&stock.hovered, 3.0),
                "{key} {flag}: egui's own hovered mark"
            );
            assert_eq!(
                w.active.fg_stroke.width,
                own(&stock.active, 3.0),
                "{key} {flag}: egui's own pressed mark"
            );
            let w = marks(&hover)?;
            assert_eq!(w.inactive.fg_stroke.width, 3.0, "{key} {flag}: at rest");
            assert_eq!(w.hovered.fg_stroke.width, 5.0, "{key} {flag}: hovered");
            assert_eq!(
                w.active.fg_stroke.width,
                own(&stock.active, 3.0),
                "{key} {flag}: pressed"
            );
        }
    }
    // The unchecked box's cell, which the checked one starts from: the width a rule states for
    // `PRESSED`, and under the pointer egui's own where the preset states one at rest.
    let press =
        "[[widgets.checkbox.rules]]\nstates = [\"pressed\"]\ncheckmark.stroke_width = 4.0\n";
    for &(key, preset) in PRESETS {
        let atlas = patched_light(preset, press)?;
        let w = &atlas
            .scheme(egui::Theme::Light)
            .cell(crate::Role::Checkbox, crate::RoleVariant::Normal)
            .visuals
            .widgets;
        assert_eq!(w.active.fg_stroke.width, 4.0, "{key}: the pressed mark");
        let r = Theme::preset(preset).resolve(&ThemeContext::light());
        if let Some(rest) = r.checkbox.checkmark(WidgetState::NONE).stroke_width {
            assert_eq!(w.inactive.fg_stroke.width, rest, "{key}: the resting mark");
            assert_eq!(
                w.hovered.fg_stroke.width,
                own(&stock.hovered, rest),
                "{key}: egui's own hovered mark"
            );
        }
    }
    Ok(())
}

/// A tool button's and a menu item's hovered and pressed outline widths are the ones a rule
/// states for the state.
#[test]
fn a_tool_button_and_a_menu_item_draw_their_stated_widths() -> Result<(), Box<dyn std::error::Error>>
{
    let magenta = egui::Color32::from_rgb(0xff, 0, 0xff);
    let rules = "[[widgets.toolbar.rules]]\nstates = [\"hover\"]\n\
                 button.stroke = { width = 2.0, color = \"#ff00ff\" }\n\
                 [[widgets.toolbar.rules]]\nstates = [\"pressed\"]\n\
                 button.stroke = { width = 3.0, color = \"#ff00ff\" }\n\
                 [[widgets.menu.rules]]\nstates = [\"hover\"]\n\
                 item.stroke = { width = 2.0, color = \"#ff00ff\" }\n";
    let atlas = patched_light(Preset::KdeBreeze, rules)?;
    let styles = atlas.scheme(egui::Theme::Light);
    let tool = &styles
        .cell(crate::Role::Toolbar, crate::RoleVariant::Normal)
        .visuals
        .widgets;
    assert_eq!(tool.hovered.bg_stroke, egui::Stroke::new(2.0, magenta));
    assert_eq!(tool.active.bg_stroke, egui::Stroke::new(3.0, magenta));
    let menu = &styles
        .cell(crate::Role::Menu, crate::RoleVariant::Normal)
        .visuals
        .widgets;
    assert_eq!(menu.hovered.bg_stroke, egui::Stroke::new(2.0, magenta));
    assert_eq!(menu.open.bg_stroke, egui::Stroke::new(2.0, magenta));
    Ok(())
}

/// A tool button at rest is filled and rounded as `toolbar.button(NONE)` states, in the
/// toolbar's `Normal` cell, and disabled filled as a rule states `toolbar.button(DISABLED)`'s
/// fill, in its `Disabled` cell: macos-tahoe's rest capsule, #ffffff light and #252525 dark, a
/// pill, disabled unchanged; macos-monterey's disabled button none, transparent, its rest fill
/// unstated and so the base style's (spec §10, rule 0). Measured, os-probe run 45634379188,
/// toolbar states sheet, `*.normal.toolbar_button` and `*.normal.toolbar_button_disabled`
/// `fill`, `*.normal.toolbar_toggle_on` `box` (native-theme/src/next/presets/macos.rs,
/// `toolbar_button_rest`, `toolbar_button_corners`).
#[test]
fn a_tool_buttons_rest_and_disabled_capsule_is_drawn_as_stated()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::{Role, RoleVariant};
    let tahoe = atlas_of(&Theme::preset(Preset::MacosTahoe))?;
    for (scheme, fill) in [
        (egui::Theme::Light, egui::Color32::WHITE),
        (egui::Theme::Dark, egui::Color32::from_rgb(0x25, 0x25, 0x25)),
    ] {
        let styles = tahoe.scheme(scheme);
        let w = &styles
            .cell(Role::Toolbar, RoleVariant::Normal)
            .visuals
            .widgets;
        assert_eq!(
            w.inactive.weak_bg_fill, fill,
            "macos-tahoe {scheme:?} at rest"
        );
        for e in [&w.inactive, &w.hovered, &w.active, &w.open] {
            assert_eq!(
                e.corner_radius,
                egui::CornerRadius::same(u8::MAX),
                "macos-tahoe {scheme:?}"
            );
        }
        let d = &styles
            .cell(Role::Toolbar, RoleVariant::Disabled)
            .visuals
            .widgets;
        assert_eq!(
            d.inactive.weak_bg_fill, fill,
            "macos-tahoe {scheme:?} disabled"
        );
    }
    let monterey = atlas_of(&Theme::preset(Preset::MacosMonterey))?;
    for scheme in [egui::Theme::Light, egui::Theme::Dark] {
        let styles = monterey.scheme(scheme);
        assert_eq!(
            styles
                .cell(Role::Toolbar, RoleVariant::Normal)
                .visuals
                .widgets
                .inactive
                .weak_bg_fill,
            styles.base.visuals.widgets.inactive.weak_bg_fill,
            "macos-monterey {scheme:?} at rest: the base style's"
        );
        assert_eq!(
            styles
                .cell(Role::Toolbar, RoleVariant::Disabled)
                .visuals
                .widgets
                .inactive
                .weak_bg_fill,
            egui::Color32::TRANSPARENT,
            "macos-monterey {scheme:?} disabled"
        );
    }
    Ok(())
}

/// A popup's stated shadow is drawn as stated, its first outset layer's offset, blur, spread
/// and colour (spec §10, rule 2), on the popover's frame and, the menu's panel stating none,
/// on egui's popups; an inset layer alone casts no shadow.
#[test]
fn a_stated_drop_shadow_is_drawn_as_stated() -> Result<(), Box<dyn std::error::Error>> {
    let r = Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
    assert!(
        !r.menu
            .panel(WidgetState::NONE)
            .stated
            .contains(native_theme::next::PartStated::SHADOWS)
    );
    let layers = "[{ offset_x = 1.0, offset_y = 3.0, blur = 2.0, spread = 1.0, color = \"#ff00ff\", inset = true }, \
                  { offset_x = 2.0, offset_y = 4.0, blur = 18.0, spread = 3.0, color = \"#00ff00\" }]";
    let want = egui::Shadow {
        offset: [2, 4],
        blur: 18,
        spread: 3,
        color: egui::Color32::GREEN,
    };
    let atlas = patched_light(
        Preset::KdeBreeze,
        &format!(
            "[widgets.popover]\npanel.shadows = {layers}\n\
             [[widgets.card.rules]]\ncontainer.shadows = {layers}\n"
        ),
    )?;
    let styles = atlas.scheme(egui::Theme::Light);
    assert_eq!(styles.frame(crate::Surface::Popover).shadow, want);
    assert_eq!(styles.base.visuals.popup_shadow, want);
    assert_eq!(styles.frame(crate::Surface::Card).shadow, want);
    let inset = "[{ blur = 2.0, color = \"#ff00ff\", inset = true }]";
    let atlas = patched_light(
        Preset::KdeBreeze,
        &format!("[widgets.popover]\npanel.shadows = {inset}\n"),
    )?;
    assert_eq!(
        atlas
            .scheme(egui::Theme::Light)
            .frame(crate::Surface::Popover)
            .shadow,
        egui::Shadow::NONE
    );
    Ok(())
}

/// The ring's second stroke and its own corners are drawn where the theme states them: the
/// plugin's ring holds the inner stroke and the stated corners as its rect's, the line the
/// offset places, between the two rings (a continuous curve drawn circular); unstated,
/// neither; concentric corners are unmapped.
#[test]
fn the_focus_rings_inner_stroke_and_corners_are_drawn() -> Result<(), Box<dyn std::error::Error>> {
    use native_theme::next::{CornerRadius, Corners};

    let theme = Theme::preset(Preset::Windows11);
    let [mut light, light_inactive, dark, dark_inactive] = [
        ThemeContext::light(),
        ThemeContext::light().with_window_active(false),
        ThemeContext::dark(),
        ThemeContext::dark().with_window_active(false),
    ]
    .map(|ctx| theme.resolve(&ctx));
    let ring = |light: &native_theme::next::ResolvedTheme| {
        super::to_theme(light, &light_inactive, &dark, &dark_inactive)
            .map(|atlas| atlas.focus_ring(egui::Theme::Light).copied())
    };
    let stated = ring(&light)?.ok_or("windows-11 states a ring")?;
    assert_eq!(
        (stated.inner.map(|inner| inner.width), stated.corners),
        (Some(1.0), None),
        "windows-11 states the inner ring, FocusStrokeColorInner"
    );
    light.focus_ring.inner_stroke = Stroke::None;
    let plain = ring(&light)?.ok_or("the ring")?;
    assert_eq!((plain.inner, plain.corners), (None, None));
    light.focus_ring.inner_stroke = Stroke::uniform(1.0, Color::new(1.0, 1.0, 1.0, 1.0));
    light.focus_ring.corners = Some(Corners::uniform(4.0));
    let drawn = ring(&light)?.ok_or("the ring")?;
    assert_eq!(
        drawn.inner,
        Some(egui::Stroke::new(1.0, egui::Color32::WHITE))
    );
    assert_eq!(drawn.corners, Some(egui::CornerRadius::same(4)));
    light.focus_ring.corners = Some(Corners::continuous(4.0));
    let drawn = ring(&light)?.ok_or("the ring")?;
    assert_eq!(drawn.corners, Some(egui::CornerRadius::same(4)));
    light.focus_ring.corners = Some(Corners {
        radius: CornerRadius::Concentric {
            inset: 2.0,
            min_radius: 1.0,
        },
        ..Corners::default()
    });
    assert_eq!(
        ring(&light).err(),
        Some(super::Unmapped("focus_ring.corners"))
    );
    light.focus_ring.corners = None;
    light.focus_ring.inner_stroke = Stroke::sides(
        StrokeSide::default(),
        StrokeSide::default(),
        StrokeSide::default(),
        StrokeSide::default(),
        StrokeAlign::Inside,
    );
    assert_eq!(
        ring(&light).err(),
        Some(super::Unmapped("focus_ring.inner_stroke"))
    );
    Ok(())
}

/// The 0.8.0 atlas's builder takes what the 0.8.0 theme does not carry: the application's own
/// font plan (installed; none leaves the `Context`'s fonts alone across installs), the OS
/// colour mode and the style patch, applied to every style.
#[test]
fn the_next_builder_takes_fonts_os_mode_and_a_style_patch() -> Result<(), Box<dyn std::error::Error>>
{
    use crate::fonts::FontPlan;
    use crate::install_tests::pass;

    let theme = Theme::preset(Preset::KdeBreeze);
    let [light, light_inactive, dark, dark_inactive] = [
        ThemeContext::light(),
        ThemeContext::light().with_window_active(false),
        ThemeContext::dark(),
        ThemeContext::dark().with_window_active(false),
    ]
    .map(|ctx| theme.resolve(&ctx));
    let builder =
        || crate::ThemeAtlas::next_builder(&light, &light_inactive, &dark, &dark_inactive);

    // a given plan: its definitions are the atlas's and `install` sets them
    let plan = FontPlan::default();
    let (want, _) = super::font_definitions(&light, &plan)?;
    let atlas = builder().fonts(plan).build()?;
    assert_eq!(atlas.fonts(), Some(&want));
    let mut own = egui::FontDefinitions::default();
    own.families
        .insert(egui::FontFamily::Name("mine".into()), Vec::new());
    assert_ne!(own, want);
    let ctx = egui::Context::default();
    ctx.set_fonts(own.clone());
    let _ = pass(&ctx, egui::RawInput::default(), |_| {});
    atlas.install(&ctx);
    let _ = pass(&ctx, egui::RawInput::default(), |_| {});
    assert!(ctx.fonts(|f| f.definitions() == &want));

    // no plan: the `Context`'s own fonts stay, across two installs
    let ctx = egui::Context::default();
    ctx.set_fonts(own.clone());
    let _ = pass(&ctx, egui::RawInput::default(), |_| {});
    for _ in 0..2 {
        builder().build()?.install(&ctx);
        let _ = pass(&ctx, egui::RawInput::default(), |_| {});
        assert!(ctx.fonts(|f| f.definitions() == &own));
    }

    // the OS colour mode is reported; the style patch reaches every style
    let atlas = builder()
        .os_mode(crate::ColorMode::Dark)
        .style_patch(|s| s.spacing.indent = 77.0)
        .build()?;
    assert_eq!(atlas.os_mode(), Some(crate::ColorMode::Dark));
    for scheme in [egui::Theme::Light, egui::Theme::Dark] {
        let styles = atlas.scheme(scheme);
        assert_eq!(styles.base.spacing.indent, 77.0);
        for role in crate::roles::ROLES {
            for variant in crate::roles::VARIANTS {
                assert_eq!(styles.cell(role, variant).spacing.indent, 77.0, "{role:?}");
            }
        }
    }
    Ok(())
}

/// A tab's and a segment's hovered and pressed outline are the ones a rule states for the
/// state; unstated, egui's own where egui's entries change it from rest (spec §10, rule 0);
/// and a role that states no outline at rest, hover or press keeps the base style's.
#[test]
fn a_tab_and_a_segment_outline_each_state_by_rule_0() -> Result<(), Box<dyn std::error::Error>> {
    use native_theme::next::{PartStated, PartStyle, ResolvedTheme};

    let own = egui::Theme::Light.default_style().visuals.widgets;
    let tab: fn(&ResolvedTheme, WidgetState) -> PartStyle = |r, s| r.tab.tab(s);
    let segment: fn(&ResolvedTheme, WidgetState) -> PartStyle =
        |r, s| r.segmented_control.segment(s);
    for (role, widget, part, of) in [
        (crate::Role::Tab, "tab", "tab", tab),
        (
            crate::Role::SegmentedControl,
            "segmented_control",
            "segment",
            segment,
        ),
    ] {
        // a preset with no rule for a pressed outline, one stating the outline at rest where
        // a preset does
        let unpressed = |rest_stated: bool| {
            PRESETS.iter().find(|(_, p)| {
                let r = Theme::preset(*p).resolve(&ThemeContext::light());
                (!rest_stated
                    || of(&r, WidgetState::NONE)
                        .stated
                        .contains(PartStated::STROKE))
                    && !of(&r, WidgetState::PRESSED)
                        .from_state
                        .contains(PartStated::STROKE)
            })
        };
        let (key, preset) = unpressed(true)
            .or_else(|| unpressed(false))
            .ok_or_else(|| format!("{role:?}: every preset states a pressed {part}'s outline"))?;
        let rules = format!(
            "[[widgets.{widget}.rules]]\nstates = [\"hover\"]\n\
             {part}.stroke = {{ width = 3.0, color = \"#ff00ff\" }}\n"
        );
        let atlas = patched_light(*preset, &rules)?;
        let w = &atlas
            .scheme(egui::Theme::Light)
            .cell(role, crate::RoleVariant::Normal)
            .visuals
            .widgets;
        assert_eq!(
            w.hovered.bg_stroke,
            egui::Stroke::new(3.0, egui::Color32::from_rgb(0xff, 0, 0xff)),
            "{role:?} on {key}"
        );
        // no pressed outline stated: egui's own pressed colour and width, both of which egui's
        // stock entries change from rest
        assert_eq!(
            w.active.bg_stroke, own.active.bg_stroke,
            "{role:?} on {key}"
        );
    }
    // a preset stating no tab outline at rest, hover or press: the tab keeps the base's in
    // every state
    let (key, preset) = PRESETS
        .iter()
        .find(|(_, p)| {
            let r = Theme::preset(*p).resolve(&ThemeContext::light());
            [WidgetState::NONE, WidgetState::HOVER, WidgetState::PRESSED]
                .into_iter()
                .all(|s| {
                    !r.tab
                        .tab(s)
                        .stated
                        .contains(native_theme::next::PartStated::STROKE)
                })
        })
        .ok_or("every preset states a tab outline at rest, hover or press")?;
    let atlas = patched_light(*preset, "")?;
    let styles = atlas.scheme(egui::Theme::Light);
    let tab = &styles
        .cell(crate::Role::Tab, crate::RoleVariant::Normal)
        .visuals
        .widgets;
    let base = &styles.base.visuals.widgets;
    assert_eq!(tab.hovered.bg_stroke, base.hovered.bg_stroke, "{key}");
    assert_eq!(tab.active.bg_stroke, base.active.bg_stroke, "{key}");
    Ok(())
}

/// A tab whose outline a rule states only under the pointer: the hovered outline is the
/// stated one, and the pressed one egui's own by rule 0, not the base style's (spec §10).
#[test]
fn a_tab_outline_stated_only_hovered_leaves_the_press_to_rule_0()
-> Result<(), Box<dyn std::error::Error>> {
    let own = egui::Theme::Light.default_style().visuals.widgets;
    // a preset stating no tab outline at rest or pressed
    let (_, preset) = PRESETS
        .iter()
        .find(|(_, p)| {
            let r = Theme::preset(*p).resolve(&ThemeContext::light());
            [WidgetState::NONE, WidgetState::PRESSED]
                .into_iter()
                .all(|s| {
                    !r.tab
                        .tab(s)
                        .stated
                        .contains(native_theme::next::PartStated::STROKE)
                })
        })
        .ok_or("every preset states a tab outline at rest or pressed")?;
    let rules = "[[widgets.tab.rules]]\nstates = [\"hover\"]\n\
                 tab.stroke = { width = 3.0, color = \"#ff00ff\" }\n";
    let atlas = patched_light(*preset, rules)?;
    let w = &atlas
        .scheme(egui::Theme::Light)
        .cell(crate::Role::Tab, crate::RoleVariant::Normal)
        .visuals
        .widgets;
    assert_eq!(
        w.hovered.bg_stroke,
        egui::Stroke::new(3.0, egui::Color32::from_rgb(0xff, 0, 0xff))
    );
    assert_eq!(w.active.bg_stroke, own.active.bg_stroke);
    Ok(())
}

/// A checked box whose outline the theme states as none draws none: the checked cell's outline
/// is transparent, not the unchecked box's it is copied from (spec §10, rule 2).
#[test]
fn a_checked_outline_stated_as_none_is_transparent() -> Result<(), Box<dyn std::error::Error>> {
    use native_theme::next::PartStated;

    let (key, preset) = PRESETS
        .iter()
        .find(|(_, p)| {
            let r = Theme::preset(*p).resolve(&ThemeContext::light());
            let checked = r.checkbox.indicator(WidgetState::CHECKED);
            let rest = r.checkbox.indicator(WidgetState::NONE);
            checked.stated.contains(PartStated::STROKE)
                && matches!(checked.stroke, Stroke::None)
                && matches!(rest.stroke, Stroke::Uniform { .. })
        })
        .ok_or("no preset states a checked box's outline as none and an unchecked one's")?;
    let atlas = patched_light(*preset, "")?;
    let styles = atlas.scheme(egui::Theme::Light);
    let cell = |variant| {
        styles
            .cell(crate::Role::Checkbox, variant)
            .visuals
            .widgets
            .clone()
    };
    let (normal, selected) = (
        cell(crate::RoleVariant::Normal),
        cell(crate::RoleVariant::Selected),
    );
    assert_ne!(
        normal.inactive.bg_stroke.color,
        egui::Color32::TRANSPARENT,
        "{key}"
    );
    for (name, entry) in [
        ("noninteractive", &selected.noninteractive),
        ("inactive", &selected.inactive),
        ("open", &selected.open),
    ] {
        assert_eq!(
            entry.bg_stroke.color,
            egui::Color32::TRANSPARENT,
            "{key}: {name}"
        );
    }
    Ok(())
}

/// The menu item takes the panel's corners where it states none of its own on material-3,
/// whose table of homes row names the panel's corners, and on no other preset.
#[test]
fn the_menu_item_takes_the_panels_corners_on_material_3_alone()
-> Result<(), Box<dyn std::error::Error>> {
    let panel = "[[widgets.menu.rules]]\npanel.corners = 7.0\n";
    let mut unstated = Vec::new();
    for &(key, preset) in PRESETS {
        let r = Theme::preset(preset)
            .with_patch(ThemePatch::from_toml(panel)?)
            .resolve(&ThemeContext::light());
        if super::radius_of(&r.menu.item(WidgetState::NONE)).is_some() {
            continue;
        }
        unstated.push(preset);
        let outline = &super::build::cell_source(&r, &mut Vec::new()).menu.outline;
        let want = (preset == Preset::Material3).then_some(7.0);
        assert_eq!(outline.edge.radius, want, "{key}");
    }
    assert!(unstated.contains(&Preset::Material3), "{unstated:?}");
    assert!(unstated.len() > 1, "{unstated:?}");
    Ok(())
}

/// A text that leaves its family, weight or style unstated asks its chain for no face: the
/// chain keeps the plan's base, egui's own, and no note names it; the other chain takes the
/// plan's face.
#[test]
fn an_unstated_family_keeps_eguis_own_face() -> Result<(), super::Unmapped> {
    use crate::fonts::{FontBytes, FontPlan};
    use native_theme::next::TextStated;

    let own = egui::FontDefinitions::default();
    let chain = |defs: &egui::FontDefinitions, family| defs.families.get(&family).cloned();
    for unstated in [TextStated::FAMILY, TextStated::WEIGHT, TextStated::STYLE] {
        for &(key, preset) in PRESETS {
            let mut r = Theme::preset(preset).resolve(&ThemeContext::light());
            let (body, mono) = (&r.typography.body, &r.typography.monospace);
            let face = |text: &native_theme::next::TextStyle| {
                let style = match text.style {
                    native_theme::next::FontStyle::Italic => crate::FontStyle::Italic,
                    native_theme::next::FontStyle::Oblique => crate::FontStyle::Oblique,
                    _ => crate::FontStyle::Normal,
                };
                (text.family.clone(), text.weight, style)
            };
            let plan = [face(body), face(mono)].into_iter().fold(
                FontPlan::new(),
                |plan, (family, weight, style)| {
                    plan.face(
                        &family,
                        weight,
                        style,
                        FontBytes::Static(epaint_default_fonts::HACK_REGULAR),
                    )
                },
            );
            let (stated, _) = super::font_definitions(&r, &plan)?;
            assert_ne!(
                chain(&stated, egui::FontFamily::Proportional),
                chain(&own, egui::FontFamily::Proportional),
                "{key}: the stated body takes the plan's face, or the check proves nothing"
            );
            r.typography.body.stated = r.typography.body.stated.difference(unstated);
            let (defs, notes) = super::font_definitions(&r, &plan)?;
            assert_eq!(
                chain(&defs, egui::FontFamily::Proportional),
                chain(&own, egui::FontFamily::Proportional),
                "{key} {unstated:?}"
            );
            assert_eq!(
                chain(&defs, egui::FontFamily::Monospace),
                chain(&stated, egui::FontFamily::Monospace),
                "{key} {unstated:?}"
            );
            assert!(notes.is_empty(), "{key} {unstated:?}: {notes:?}");
        }
    }
    Ok(())
}

/// Each scheme's theme and the same theme for a window that is not active, light then dark.
fn schemes(theme: &Theme) -> [native_theme::next::ResolvedTheme; 4] {
    [
        ThemeContext::light(),
        ThemeContext::light().with_window_active(false),
        ThemeContext::dark(),
        ThemeContext::dark().with_window_active(false),
    ]
    .map(|ctx| theme.resolve(&ctx))
}

/// The text colour of a scheme of a 0.8.0 atlas is its theme's `colors.window.foreground`,
/// and none where that theme states none.
#[test]
fn a_next_atlas_text_colour_is_the_window_foreground() -> Result<(), super::Unmapped> {
    for &(key, preset) in PRESETS {
        let [light, light_inactive, dark, dark_inactive] = schemes(&Theme::preset(preset));
        let atlas = super::to_theme(&light, &light_inactive, &dark, &dark_inactive)?;
        for (scheme, r) in [(egui::Theme::Light, &light), (egui::Theme::Dark, &dark)] {
            let want = r.colors.window.foreground.map(super::build::rgba);
            assert!(want.is_some(), "{key} {scheme:?}: the preset states it");
            assert_eq!(atlas.text_color(scheme), want, "{key} {scheme:?}");
        }
        let mut unstated = light.clone();
        unstated.colors.window.foreground = None;
        let atlas = super::to_theme(&unstated, &light_inactive, &dark, &dark_inactive)?;
        assert_eq!(atlas.text_color(egui::Theme::Light), None, "{key}");
    }
    Ok(())
}

/// `menu_bar_modifier` of a 0.8.0 atlas is the `Role::Menu` cell with the menu bar's titles:
/// the hovered and pressed titles in `menu_bar.item(HOVER)`'s fill and `item_label(HOVER)`'s
/// colour, the open one in `(OPEN)`'s, each where a rule states it, every title outlined at
/// rest; where none is stated, the cell as it is.
#[test]
fn a_next_atlas_menu_bar_modifier_is_the_menu_cell_with_its_titles()
-> Result<(), Box<dyn std::error::Error>> {
    let rgb = egui::Color32::from_rgb;
    let titles = "[[widgets.menu_bar.rules]]\nstates = [\"hover\"]\nitem.fill = \"#f0a014\"\n\
                  item_label.color = \"#102030\"\n\
                  [[widgets.menu_bar.rules]]\nstates = [\"open\"]\nitem.fill = \"#14a0f0\"\n";
    let atlas = patched_light(Preset::Adwaita, titles)?;
    let mut bar = egui::Style::default();
    atlas.menu_bar_modifier(egui::Theme::Light).apply(&mut bar);
    let mut cell = egui::Style::default();
    atlas
        .role_modifier(
            egui::Theme::Light,
            crate::Role::Menu,
            crate::RoleVariant::Normal,
        )
        .apply(&mut cell);
    let w = &bar.visuals.widgets;
    assert_eq!(w.hovered.weak_bg_fill, rgb(0xf0, 0xa0, 0x14));
    assert_eq!(w.active.weak_bg_fill, rgb(0xf0, 0xa0, 0x14));
    assert_eq!(w.open.weak_bg_fill, rgb(0x14, 0xa0, 0xf0));
    assert_eq!(w.hovered.fg_stroke.color, rgb(0x10, 0x20, 0x30));
    assert_eq!(w.active.fg_stroke.color, rgb(0x10, 0x20, 0x30));
    for e in [&w.hovered, &w.active, &w.open] {
        assert_eq!(e.bg_stroke, w.inactive.bg_stroke);
    }
    assert_eq!(w.inactive, cell.visuals.widgets.inactive);
    assert_eq!(w.noninteractive, cell.visuals.widgets.noninteractive);
    assert_eq!(bar.text_styles, cell.text_styles);

    for &(key, preset) in PRESETS {
        let [light, light_inactive, dark, dark_inactive] = schemes(&Theme::preset(preset));
        let atlas = super::to_theme(&light, &light_inactive, &dark, &dark_inactive)?;
        for (scheme, r) in [(egui::Theme::Light, &light), (egui::Theme::Dark, &dark)] {
            let mb = &r.menu_bar;
            let stated = [WidgetState::HOVER, WidgetState::OPEN].iter().any(|&s| {
                super::build::state_fill(&mb.item(s), "menu_bar.item", &mut Vec::new()).is_some()
                    || super::state_text_color_of(&mb.item_label(s)).is_some()
            });
            if stated {
                continue;
            }
            let mut bar = egui::Style::default();
            atlas.menu_bar_modifier(scheme).apply(&mut bar);
            let mut cell = egui::Style::default();
            atlas
                .role_modifier(scheme, crate::Role::Menu, crate::RoleVariant::Normal)
                .apply(&mut cell);
            crate::style::menu_bar_titles(&atlas.scheme(scheme).menu_bar, &mut cell);
            assert_eq!(bar, cell, "{key} {scheme:?}");
        }
    }
    Ok(())
}

/// The checked box's hovered fill by rule 0 per property (spec §10): where a rule states
/// `checkbox.indicator(HOVER).fill` alone, the combined state's fill; where none does, egui's
/// own hovered fill where egui's stock entries change it from rest, else the checked fill. The
/// pressed fill follows the same rule.
#[test]
fn the_checked_boxs_state_fills_follow_rule_0() -> Result<(), Box<dyn std::error::Error>> {
    let own = egui::Theme::Light.default_style();
    let own = &own.visuals.widgets;
    let by_rule_0 = |state: &egui::style::WidgetVisuals, checked: egui::Color32| {
        if state.bg_fill == own.inactive.bg_fill {
            checked
        } else {
            state.bg_fill
        }
    };
    let hover = "[[widgets.checkbox.rules]]\nstates = [\"hover\"]\nindicator.fill = \"#f0a014\"\n";
    let mut tried = Vec::new();
    for &(key, preset) in PRESETS {
        let plain = Theme::preset(preset).resolve(&ThemeContext::light());
        let k = &plain.checkbox;
        let unstated = |r: &native_theme::next::ResolvedTheme, s: WidgetState| {
            super::build::state_fill(&r.checkbox.indicator(s), "indicator", &mut Vec::new())
                .is_none()
        };
        let Some(checked) = super::build::fill(
            &k.indicator(WidgetState::CHECKED),
            "checkbox.indicator(CHECKED).fill",
            &mut Vec::new(),
        ) else {
            continue;
        };
        let checked = crate::convert::to_color32(checked);
        let cell = |atlas: &crate::ThemeAtlas| {
            atlas
                .scheme(egui::Theme::Light)
                .cell(crate::Role::Checkbox, crate::RoleVariant::Selected)
                .visuals
                .widgets
                .clone()
        };
        let w = cell(&patched_light(preset, "")?);
        assert_eq!(w.inactive.bg_fill, checked, "{key}");
        // Every bundled preset states its pressed box (M2.FS4): where a state's fill is stated,
        // the combined state's; where it is unstated, rule 0.
        let expected = |state: WidgetState, own: &egui::style::WidgetVisuals| {
            if unstated(&plain, state) {
                Some(by_rule_0(own, checked))
            } else {
                super::build::fill(
                    &k.indicator(state | WidgetState::CHECKED),
                    "indicator",
                    &mut Vec::new(),
                )
                .map(crate::convert::to_color32)
            }
        };
        assert_eq!(
            Some(w.hovered.bg_fill),
            expected(WidgetState::HOVER, &own.hovered),
            "{key}"
        );
        assert_eq!(
            Some(w.active.bg_fill),
            expected(WidgetState::PRESSED, &own.active),
            "{key}"
        );
        tried.push(key);

        let patched = Theme::preset(preset)
            .with_patch(ThemePatch::from_toml(hover)?)
            .resolve(&ThemeContext::light());
        let both = super::build::fill(
            &patched
                .checkbox
                .indicator(WidgetState::HOVER | WidgetState::CHECKED),
            "checkbox.indicator(HOVER|CHECKED).fill",
            &mut Vec::new(),
        )
        .ok_or("the combined state has a fill")?;
        let w = cell(&patched_light(preset, hover)?);
        assert_eq!(w.hovered.bg_fill, crate::convert::to_color32(both), "{key}");
        assert_eq!(
            Some(w.active.bg_fill),
            expected(WidgetState::PRESSED, &own.active),
            "{key}"
        );
    }
    assert!(!tried.is_empty(), "no preset states a checked fill");
    Ok(())
}

/// The controls whose hovered and pressed fills the 0.7.2 model states as layers over the idle
/// fill, each with the role whose cell draws it, the widget and part whose hovered fill a rule
/// states, and whether the entry paints `weak_bg_fill` (else `bg_fill`).
const STATE_FILLS: [(crate::Role, &str, &str, bool); 5] = [
    (crate::Role::Button, "button", "body", true),
    (crate::Role::Checkbox, "checkbox", "indicator", false),
    (crate::Role::Switch, "switch", "track", true),
    (crate::Role::ComboBox, "combo_box", "field", true),
    (
        crate::Role::SegmentedControl,
        "segmented_control",
        "segment",
        true,
    ),
];

/// A state's fill on the 0.8.0 model is the whole fill the part shows in that state (spec §9,
/// step 2), written as given: a translucent hovered fill a rule states is the entry's, not
/// composited over the resting fill, and so is an opaque one.
#[test]
fn a_stated_hovered_fill_is_the_states_own() -> Result<(), Box<dyn std::error::Error>> {
    for (marker, color) in [
        (
            "#ff00ff80",
            egui::Color32::from_rgba_unmultiplied(0xff, 0, 0xff, 0x80),
        ),
        ("#f0a014", egui::Color32::from_rgb(0xf0, 0xa0, 0x14)),
    ] {
        for (role, widget, part, weak) in STATE_FILLS {
            let patch = format!(
                "[[widgets.{widget}.rules]]\nstates = [\"hover\"]\n{part}.fill = \"{marker}\"\n"
            );
            let atlas = patched_light(Preset::KdeBreeze, &patch)?;
            let cell = atlas
                .scheme(egui::Theme::Light)
                .cell(role, crate::RoleVariant::Normal);
            let hovered = &cell.visuals.widgets.hovered;
            let fill = if weak {
                hovered.weak_bg_fill
            } else {
                hovered.bg_fill
            };
            assert_eq!(fill, color, "{role:?} {marker}");
        }
    }
    Ok(())
}

/// The rows and items whose hovered and pressed fills the cells read as given, each as
/// [`STATE_FILLS`] names it.
const ROW_FILLS: [(crate::Role, &str, &str, bool); 3] = [
    (crate::Role::List, "list", "highlight", false),
    (crate::Role::Sidebar, "sidebar", "item", true),
    (crate::Role::Tab, "tab", "tab", true),
];

/// A hovered or pressed fill a rule states as none is transparent (spec §10, rule 2), not
/// unstated: the entry draws no fill, where an unstated one would keep egui's own or the base's.
#[test]
fn a_state_fill_stated_as_none_is_transparent() -> Result<(), Box<dyn std::error::Error>> {
    let plain = patched_light(Preset::KdeBreeze, "")?;
    for (state, pressed) in [("hover", false), ("pressed", true)] {
        for (role, widget, part, weak) in STATE_FILLS.into_iter().chain(ROW_FILLS) {
            let fill = |atlas: &crate::ThemeAtlas| {
                let cell = atlas
                    .scheme(egui::Theme::Light)
                    .cell(role, crate::RoleVariant::Normal);
                let w = &cell.visuals.widgets;
                let entry = if pressed { &w.active } else { &w.hovered };
                if weak {
                    entry.weak_bg_fill
                } else {
                    entry.bg_fill
                }
            };
            let patch = format!(
                "[[widgets.{widget}.rules]]\nstates = [\"{state}\"]\n{part}.fill = \"none\"\n"
            );
            let atlas = patched_light(Preset::KdeBreeze, &patch)?;
            assert_ne!(
                fill(&plain),
                egui::Color32::TRANSPARENT,
                "{role:?} {state}: the preset's own fill proves nothing"
            );
            assert_eq!(fill(&atlas), egui::Color32::TRANSPARENT, "{role:?} {state}");
        }
    }
    Ok(())
}

/// On the macOS presets, whose button the pointer does not change (`platform-facts.md` §2.3),
/// the hovered button's fill is its resting fill wherever the preset states it so: the state's
/// own fill, not a layer laid over the rest again.
#[test]
fn a_macos_buttons_hovered_fill_is_its_resting_one() -> Result<(), Box<dyn std::error::Error>> {
    let mut tried = Vec::new();
    let mut layered_would_differ = 0;
    for preset in [Preset::MacosMonterey, Preset::MacosTahoe] {
        for (scheme, ctx) in [
            (egui::Theme::Light, ThemeContext::light()),
            (egui::Theme::Dark, ThemeContext::dark()),
        ] {
            let r = Theme::preset(preset).resolve(&ctx);
            let body = |s| super::build::state_fill(&r.button.body(s), "body", &mut Vec::new());
            let rest = super::build::fill(
                &r.button.body(WidgetState::NONE),
                "button.body(NONE).fill",
                &mut Vec::new(),
            );
            let Some(rest_fill) = rest else { continue };
            if body(WidgetState::HOVER).is_none() || body(WidgetState::HOVER) != rest {
                continue;
            }
            if crate::convert::composite_over(rest_fill, rest_fill)
                != crate::convert::to_color32(rest_fill)
            {
                layered_would_differ += 1;
            }
            tried.push((preset, scheme));
            let atlas = patched_light(preset, "")?;
            let widgets = &atlas.scheme(scheme).base.visuals.widgets;
            assert_eq!(
                widgets.hovered.weak_bg_fill, widgets.inactive.weak_bg_fill,
                "{preset:?} {scheme:?}"
            );
        }
    }
    assert!(
        !tried.is_empty(),
        "no macOS preset states its hover as rest"
    );
    assert!(
        layered_would_differ > 0,
        "no macOS scheme's rest fill is translucent, so the test cannot tell a whole fill from a layered one"
    );
    Ok(())
}
