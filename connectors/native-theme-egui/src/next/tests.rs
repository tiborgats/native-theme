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
                assert_eq!(super::fill_of(&part), super::fill(&part, "h").ok(), "{at}");
                assert_eq!(
                    super::state_fill_of(&part),
                    super::state_fill(&part, "h").ok().flatten(),
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
            if let Some(fill) =
                super::build::fill(&inactive.window.title_bar(WindowState::default()))
            {
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
    let theme = Theme::preset(preset).with_patch(ThemePatch::from_toml(patch)?);
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

    let patch = ThemePatch::from_toml(
        "[[widgets.button.rules]]\nstates = [\"default\"]\nbody.fill = \"#ff00ff\"\n",
    )?;
    let mut own_hover = 0;
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
            assert_eq!(
                button.text,
                super::text_color_of(&b.label(WidgetState::DEFAULT))
                    .unwrap_or(stock.inactive.fg_stroke.color),
                "{at}"
            );
            let body = b.body(WidgetState::DEFAULT);
            let frame = if body.stated.contains(PartStated::STROKE) {
                super::stroke_color_of(&body).unwrap_or(egui::Color32::TRANSPARENT)
            } else {
                stock.inactive.bg_stroke.color
            };
            assert_eq!(button.stroke.color, frame, "{at}");
            let width = super::stroke_width_of(&b.body(WidgetState::NONE))
                .unwrap_or(stock.inactive.bg_stroke.width);
            assert_eq!(button.stroke.width, width, "{at}");
            match super::state_stroke_color_of(&b.body(WidgetState::HOVER)) {
                Some(hover) => assert_eq!(button.hover_stroke.color, hover, "{at}"),
                None => {
                    own_hover += 1;
                    assert_eq!(
                        button.hover_stroke.color, stock.hovered.bg_stroke.color,
                        "{at}"
                    );
                }
            }
        }
    }
    assert!(own_hover > 0, "no preset leaves the hovered frame unstated");
    Ok(())
}

/// A tab's and a segment's hovered and pressed outline are the ones a rule states for the
/// state; unstated, egui's own where egui's entries change it from rest (spec §10, rule 0);
/// and a role that states no outline at rest, hover or press keeps the base style's.
#[test]
fn a_tab_and_a_segment_outline_each_state_by_rule_0() -> Result<(), Box<dyn std::error::Error>> {
    let own = egui::Theme::Light.default_style().visuals.widgets;
    for (role, widget, part) in [
        (crate::Role::Tab, "tab", "tab"),
        (
            crate::Role::SegmentedControl,
            "segmented_control",
            "segment",
        ),
    ] {
        let rules = format!(
            "[[widgets.{widget}.rules]]\nstates = [\"hover\"]\n\
             {part}.stroke = {{ width = 3.0, color = \"#ff00ff\" }}\n"
        );
        let atlas = patched_light(Preset::KdeBreeze, &rules)?;
        let w = &atlas
            .scheme(egui::Theme::Light)
            .cell(role, crate::RoleVariant::Normal)
            .visuals
            .widgets;
        assert_eq!(
            w.hovered.bg_stroke,
            egui::Stroke::new(3.0, egui::Color32::from_rgb(0xff, 0, 0xff)),
            "{role:?}"
        );
        // kde-breeze states the outline at rest and no pressed one: egui's own pressed colour
        assert_eq!(
            w.active.bg_stroke.color, own.active.bg_stroke.color,
            "{role:?}"
        );
    }
    // adwaita states no tab outline: the tab keeps the base's in every state
    let atlas = patched_light(Preset::Adwaita, "")?;
    let styles = atlas.scheme(egui::Theme::Light);
    let tab = &styles
        .cell(crate::Role::Tab, crate::RoleVariant::Normal)
        .visuals
        .widgets;
    let base = &styles.base.visuals.widgets;
    assert_eq!(tab.hovered.bg_stroke.color, base.hovered.bg_stroke.color);
    assert_eq!(tab.active.bg_stroke.color, base.active.bg_stroke.color);
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
        let outline = &super::build::cell_source(&r).menu.outline;
        let want = (preset == Preset::Material3).then_some(7.0);
        assert_eq!(outline.edge.radius, want, "{key}");
    }
    assert!(unstated.contains(&Preset::Material3), "{unstated:?}");
    assert!(unstated.len() > 1, "{unstated:?}");
    Ok(())
}
