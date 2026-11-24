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
/// plugin's ring holds the inner stroke and the corners at the ring's inner edge (the stated
/// outer corners less the width); unstated, neither.
#[test]
fn the_focus_rings_inner_stroke_and_corners_are_drawn() -> Result<(), Box<dyn std::error::Error>> {
    use native_theme::next::Corners;

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
    let plain = ring(&light)?.ok_or("windows-11 states a ring")?;
    assert_eq!((plain.inner, plain.corners), (None, None));
    let width = plain.stroke.width;
    light.focus_ring.inner_stroke = Stroke::uniform(1.0, Color::new(1.0, 1.0, 1.0, 1.0));
    light.focus_ring.corners = Some(Corners::uniform(width + 4.0));
    let drawn = ring(&light)?.ok_or("the ring")?;
    assert_eq!(
        drawn.inner,
        Some(egui::Stroke::new(1.0, egui::Color32::WHITE))
    );
    assert_eq!(drawn.corners, Some(egui::CornerRadius::same(4)));
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
        let outline = &super::build::cell_source(&r).menu.outline;
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
                super::build::state_fill(&mb.item(s)).is_some()
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
/// pressed fill, which neither theme states, follows the same rule on both.
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
            super::build::state_fill(&r.checkbox.indicator(s)).is_none()
        };
        if !(unstated(&plain, WidgetState::HOVER) && unstated(&plain, WidgetState::PRESSED)) {
            continue;
        }
        let Some(checked) = super::build::fill(&k.indicator(WidgetState::CHECKED)) else {
            continue;
        };
        tried.push(key);
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
        assert_eq!(w.hovered.bg_fill, by_rule_0(&own.hovered, checked), "{key}");
        assert_eq!(w.active.bg_fill, by_rule_0(&own.active, checked), "{key}");

        let patched = Theme::preset(preset)
            .with_patch(ThemePatch::from_toml(hover)?)
            .resolve(&ThemeContext::light());
        let both = super::build::fill(
            &patched
                .checkbox
                .indicator(WidgetState::HOVER | WidgetState::CHECKED),
        )
        .ok_or("the combined state has a fill")?;
        let w = cell(&patched_light(preset, hover)?);
        assert_eq!(w.hovered.bg_fill, crate::convert::to_color32(both), "{key}");
        assert_eq!(w.active.bg_fill, by_rule_0(&own.active, checked), "{key}");
    }
    assert!(
        !tried.is_empty(),
        "no preset leaves both state fills unstated"
    );
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
            let body = |s| super::build::state_fill(&r.button.body(s));
            let rest = super::build::fill(&r.button.body(WidgetState::NONE));
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
