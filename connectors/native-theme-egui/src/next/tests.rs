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
                &super::build::focus_ring_source(r),
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
