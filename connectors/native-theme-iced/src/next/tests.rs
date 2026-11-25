//! The crate root's widgets, widget parts and fonts on the 0.8.0 model, on
//! themes built for the case: a stated value the presets do not show, or
//! one they leave unstated.

// Each test needs the feature of the function it calls.
#![cfg_attr(not(feature = "widgets"), allow(dead_code, unused_imports))]

use native_theme::next::{Color, Preset, ResolvedTheme, Theme, ThemeContext, ThemePatch};

use super::Unmapped;

type TestResult = Result<(), Box<dyn std::error::Error>>;

/// The presets, each scheme, each with `patch` laid over it.
fn every_preset_with(patch: &ThemePatch) -> Vec<(Preset, ResolvedTheme)> {
    super::parity::PRESETS
        .iter()
        .map(|&(_, p)| p)
        .flat_map(|p| {
            [ThemeContext::light(), ThemeContext::dark()]
                .map(|ctx| (p, Theme::preset(p).with_patch(patch.clone()).resolve(&ctx)))
        })
        .collect()
}

/// The presets, each scheme, as the parity test covers them.
fn every_preset() -> Vec<(Preset, ResolvedTheme)> {
    every_preset_with(&ThemePatch::new())
}

/// Each public part read gives what the mapping's own read of the part
/// gives, `None` where that read has no value, on every preset and a
/// button's states.
#[cfg(feature = "widgets")]
#[test]
fn the_public_part_reads_are_the_mappings_own() -> TestResult {
    use native_theme::next::WidgetState;

    use super::borders::{radius, state_line, stroke_width};

    let states = [
        WidgetState::NONE,
        WidgetState::HOVER,
        WidgetState::PRESSED,
        WidgetState::DISABLED,
        WidgetState::DEFAULT,
    ];
    for (preset, r) in every_preset() {
        for state in states {
            let part = r.button.body(state);
            let text = r.button.label(state);
            let at = format!("{preset:?} {state:?}");
            assert_eq!(
                super::stroke_color_of(&part),
                super::borders::line(&part, "h").ok().map(|l| l.color),
                "{at}"
            );
            assert_eq!(
                super::state_stroke_color_of(&part),
                state_line(&part, "h")
                    .ok()
                    .flatten()
                    .and_then(|_| super::stroke_color_of(&part)),
                "{at}"
            );
            assert_eq!(
                super::stroke_width_of(&part),
                stroke_width(&part, "h").ok(),
                "{at}"
            );
            assert_eq!(super::radius_of(&part), radius(&part, "h").ok(), "{at}");
            assert_eq!(
                super::text_color_of(&text),
                super::text_color(&text, "h").ok(),
                "{at}"
            );
            assert_eq!(
                super::state_text_color_of(&text),
                super::state_text_color(&text, "h").ok().flatten(),
                "{at}"
            );
        }
        let part = r.button.body(WidgetState::NONE);
        let own = iced_core::Padding::new(-1.0);
        let padding = super::padding_with(&part, 0.0, own);
        let sides = [padding.top, padding.right, padding.bottom, padding.left];
        for (side, read) in sides.into_iter().zip(super::padding_of(&part)) {
            assert_eq!(read.unwrap_or(own.top), side, "{preset:?}");
        }
    }
    Ok(())
}

const OUTER: Color = Color::new(0.1, 0.2, 0.9, 1.0);
const INNER: Color = Color::new(0.9, 0.8, 0.1, 1.0);

/// A stated period is the spinner's; no preset states one, and a period of
/// 0 is no period iced can turn by.
#[cfg(feature = "spinner")]
#[test]
fn the_spinner_turns_once_per_stated_period() -> TestResult {
    for (preset, mut r) in every_preset() {
        assert_eq!(super::spinner_look(&r)?.period_ms, None, "{preset:?}");
        r.spinner.period_ms = Some(1_200);
        assert_eq!(
            super::spinner_look(&r)?.period_ms,
            std::num::NonZeroU32::new(1_200),
            "{preset:?}"
        );
        r.spinner.period_ms = Some(0);
        assert_eq!(
            super::spinner_look(&r).err(),
            Some(Unmapped("spinner.period_ms")),
            "{preset:?}"
        );
    }
    Ok(())
}

/// A stated width class is the font's stretch, each class its variant; a
/// style that states none keeps iced's normal stretch, and the class does
/// not change the family.
#[cfg(feature = "system-fonts")]
#[test]
fn a_stated_width_class_is_the_fonts_stretch() -> TestResult {
    use iced_core::font::Stretch;
    use native_theme::next::{FontWidth, TextStated};

    let r = Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
    let mut body = r.typography.body.clone();
    assert!(!body.stated.contains(TextStated::WIDTH));
    let unstated = super::font(&body)?;
    assert_eq!(unstated.stretch, Stretch::Normal);
    body.stated |= TextStated::WIDTH;
    for (class, stretch) in [
        (1, Stretch::UltraCondensed),
        (2, Stretch::ExtraCondensed),
        (3, Stretch::Condensed),
        (4, Stretch::SemiCondensed),
        (5, Stretch::Normal),
        (6, Stretch::SemiExpanded),
        (7, Stretch::Expanded),
        (8, Stretch::ExtraExpanded),
        (9, Stretch::UltraExpanded),
    ] {
        body.width = FontWidth::new(class);
        let font = super::font(&body)?;
        assert_eq!(font.stretch, stretch, "{class}");
        assert_eq!(
            iced_core::Font {
                stretch: Stretch::Normal,
                ..font
            },
            unstated,
            "{class}"
        );
        assert_eq!(
            super::system_font_family(&body)?,
            super::system_font_family(&r.typography.body)?,
            "{class}"
        );
    }
    Ok(())
}

/// A ring with a stated inner stroke draws a second ring inside the first,
/// concentric with it; stated corners are those of the line the offset
/// places, between the two rings, the outer edge grown by the width, and
/// where none are stated the ring is concentric with the field's.
#[cfg(feature = "widgets")]
#[test]
fn a_stated_inner_stroke_and_corners_are_drawn_on_the_ring() -> TestResult {
    use iced_core::border::Radius;
    use iced_core::{Padding, Point, Rectangle, Size};
    use native_theme::next::{Corners, FocusIndicator, FocusKind, FocusTrigger, Stroke};

    let field = Rectangle::new(Point::new(10.0, 20.0), Size::new(100.0, 30.0));
    for (preset, mut r) in every_preset() {
        r.focus_ring = FocusIndicator::new(
            FocusKind::Ring,
            Stroke::uniform(2.0, OUTER),
            Stroke::uniform(1.0, INNER),
            1.0,
            Some(Corners::uniform(9.0)),
            Some(FocusTrigger::Always),
        );
        let look = super::ring_look(&r, 4.0)?.ok_or("a ring")?;
        let (ring, _) = look.quad(field).ok_or("the ring's quad")?;
        assert_eq!(ring.bounds, field.expand(Padding::new(3.0)), "{preset:?}");
        assert_eq!(ring.border.color, super::to_color(OUTER), "{preset:?}");
        assert_eq!(ring.border.width, 2.0, "{preset:?}");
        assert_eq!(ring.border.radius, Radius::from(11.0), "{preset:?}");
        let (inner, _) = look.inner_quad(field).ok_or("the inner ring's quad")?;
        assert_eq!(inner.bounds, field.expand(Padding::new(1.0)), "{preset:?}");
        assert_eq!(inner.border.color, super::to_color(INNER), "{preset:?}");
        assert_eq!(inner.border.width, 1.0, "{preset:?}");
        assert_eq!(inner.border.radius, Radius::from(9.0), "{preset:?}");

        r.focus_ring.corners = Some(Corners::continuous(4.0));
        let look = super::ring_look(&r, 4.0)?.ok_or("a ring")?;
        let (ring, _) = look.quad(field).ok_or("the ring's quad")?;
        assert_eq!(ring.border.radius, Radius::from(6.0), "{preset:?}");
        let (inner, _) = look.inner_quad(field).ok_or("the inner ring's quad")?;
        assert_eq!(inner.border.radius, Radius::from(4.0), "{preset:?}");

        r.focus_ring.corners = None;
        let look = super::ring_look(&r, 4.0)?.ok_or("a ring")?;
        let (ring, _) = look.quad(field).ok_or("the ring's quad")?;
        assert_eq!(ring.border.radius, Radius::from(7.0), "{preset:?}");
        let (inner, _) = look.inner_quad(field).ok_or("the inner ring's quad")?;
        assert_eq!(inner.border.radius, Radius::from(5.0), "{preset:?}");

        r.focus_ring.inner_stroke = Stroke::None;
        let look = super::ring_look(&r, 4.0)?.ok_or("a ring")?;
        assert_eq!(look.inner_quad(field), None, "{preset:?}");
    }
    Ok(())
}

/// A ring with no stated corners has its inner edge's corners the field's
/// radius plus the offset, floored at 0, and its outer edge's those grown
/// by the width (spec §2, `FocusIndicator`): round a square field, 3 wide
/// at offset -1, the outer radius is 3, not the 2 the field's radius grown
/// by the offset and the width together gives.
#[cfg(feature = "widgets")]
#[test]
fn a_ring_with_no_stated_corners_floors_its_inner_corners_before_the_width() -> TestResult {
    use iced_core::border::Radius;
    use iced_core::{Padding, Point, Rectangle, Size};
    use native_theme::next::{FocusIndicator, FocusKind, FocusTrigger, Stroke};

    let field = Rectangle::new(Point::new(10.0, 20.0), Size::new(100.0, 30.0));
    for (preset, mut r) in every_preset() {
        r.focus_ring = FocusIndicator::new(
            FocusKind::Ring,
            Stroke::uniform(3.0, OUTER),
            Stroke::None,
            -1.0,
            None,
            Some(FocusTrigger::Always),
        );
        let look = super::ring_look(&r, 0.0)?.ok_or("a ring")?;
        let (ring, _) = look.quad(field).ok_or("the ring's quad")?;
        assert_eq!(ring.bounds, field.expand(Padding::new(2.0)), "{preset:?}");
        assert_eq!(ring.border.width, 3.0, "{preset:?}");
        assert_eq!(ring.border.radius, Radius::from(3.0), "{preset:?}");
    }
    Ok(())
}

/// A focus indicator iced's ring cannot draw is [`Unmapped`]: a fill or an
/// underline, an inner stroke per side, concentric corners; continuous
/// corners are drawn circular (spec §10, rule 2).
#[cfg(feature = "widgets")]
#[test]
fn a_focus_indicator_iced_cannot_draw_is_unmapped() -> TestResult {
    use native_theme::next::{
        CornerRadius, Corners, FocusIndicator, FocusKind, Paint, Stroke, StrokeAlign, StrokeSide,
    };

    let ring = || {
        FocusIndicator::new(
            FocusKind::Ring,
            Stroke::uniform(2.0, OUTER),
            Stroke::None,
            1.0,
            None,
            None,
        )
    };
    let side = || StrokeSide {
        width: 1.0,
        paint: Paint::Solid(INNER),
    };
    for (preset, mut r) in every_preset() {
        r.focus_ring = ring();
        assert!(super::ring_look(&r, 4.0)?.is_some(), "{preset:?}");
        for kind in [FocusKind::Fill, FocusKind::Underline] {
            r.focus_ring = ring();
            r.focus_ring.kind = kind;
            assert_eq!(
                super::ring_look(&r, 4.0).err(),
                Some(Unmapped("focus_ring.kind")),
                "{preset:?} {kind:?}"
            );
        }
        r.focus_ring = ring();
        r.focus_ring.inner_stroke =
            Stroke::sides(side(), side(), side(), side(), StrokeAlign::Inside);
        assert_eq!(
            super::ring_look(&r, 4.0).err(),
            Some(Unmapped("focus_ring.inner_stroke")),
            "{preset:?}"
        );
        r.focus_ring = ring();
        r.focus_ring.corners = Some(Corners::continuous(4.0));
        assert_eq!(
            super::ring_look(&r, 4.0)?.and_then(|look| look.corners),
            Some(iced_core::border::Radius::from(4.0)),
            "{preset:?}"
        );
        r.focus_ring.corners = Some(Corners {
            radius: CornerRadius::Concentric {
                inset: 2.0,
                min_radius: 1.0,
            },
            ..Corners::default()
        });
        assert_eq!(
            super::ring_look(&r, 4.0).err(),
            Some(Unmapped("focus_ring.corners")),
            "{preset:?}"
        );
    }
    Ok(())
}

/// FR4's ring of a part's own: 2 px of [`OUTER`], 1 px out from the part's
/// edge, the second ring 1 px of [`INNER`], unlike every preset's global
/// ring.
fn own_ring() -> std::sync::Arc<native_theme::next::FocusIndicator> {
    use native_theme::next::{FocusIndicator, FocusKind, FocusTrigger, Stroke};
    std::sync::Arc::new(FocusIndicator::new(
        FocusKind::Ring,
        Stroke::uniform(2.0, OUTER),
        Stroke::uniform(1.0, INNER),
        1.0,
        None,
        Some(FocusTrigger::Always),
    ))
}

/// FR4: where a part states its own focus ring (spec §2, `FocusIndicator`:
/// `PartStyle.focus`), [`super::part_ring_look`] is that ring round the
/// field, in place of the global one: the text field's, patched to state
/// one, in its own stroke's colour and width, a band outward from the
/// field's edge moved out by its offset, and its second ring inward from
/// that line. A part that states none keeps the global ring: the
/// multi-line field's look is [`super::ring_look`]'s.
#[cfg(feature = "widgets")]
#[test]
fn a_part_that_states_its_own_ring_draws_it_in_place_of_the_global_one() -> TestResult {
    use iced_core::{Padding, Point, Rectangle, Size};
    use native_theme::next::{InputRule, PartPatch, PartStated, WidgetState};

    use super::RingHomes;

    let field = Rectangle::new(Point::new(10.0, 20.0), Size::new(100.0, 30.0));
    let patch =
        ThemePatch::new().input(InputRule::new().field(PartPatch::new().focus(Some(own_ring()))));
    let mut kept = 0;
    for (preset, r) in every_preset_with(&patch) {
        let global = super::ring_look(&r, 4.0)?;
        assert_ne!(
            global.map(|look| look.color),
            Some(super::to_color(OUTER)),
            "{preset:?}: the global ring is not the part's own"
        );
        let input = r.input.field(WidgetState::FOCUSED);
        let look = super::part_ring_look(&r, &input, RingHomes::INPUT, 4.0)?
            .ok_or("the field's own ring")?;
        let (ring, _) = look.quad(field).ok_or("the ring's quad")?;
        assert_eq!(ring.bounds, field.expand(Padding::new(3.0)), "{preset:?}");
        assert_eq!(ring.border.color, super::to_color(OUTER), "{preset:?}");
        assert_eq!(ring.border.width, 2.0, "{preset:?}");
        let (inner, _) = look.inner_quad(field).ok_or("the inner ring's quad")?;
        assert_eq!(inner.bounds, field.expand(Padding::new(1.0)), "{preset:?}");
        assert_eq!(inner.border.color, super::to_color(INNER), "{preset:?}");
        assert_eq!(inner.border.width, 1.0, "{preset:?}");
        assert!(!look.keyboard_only, "{preset:?}: the own ring's trigger");

        let area = r.text_area.field(WidgetState::FOCUSED);
        if !area.stated.contains(PartStated::FOCUS) {
            kept += 1;
            assert_eq!(
                super::part_ring_look(&r, &area, RingHomes::TEXT_AREA, 4.0)?,
                global,
                "{preset:?}: the multi-line field keeps the global ring"
            );
        }
    }
    assert!(
        kept > 0,
        "no preset leaves the multi-line field's ring unstated"
    );
    Ok(())
}

/// FR4: a part whose rule states `focus` with no indicator draws none where
/// the global ring is drawn; one stated in a form iced's ring cannot draw is
/// [`Unmapped`] at the part's own home, not the global ring's.
#[cfg(feature = "widgets")]
#[test]
fn a_part_that_states_no_ring_or_one_iced_cannot_draw_draws_none() -> TestResult {
    use native_theme::next::{CornerRadius, Corners, FocusKind, InputRule, PartPatch, WidgetState};

    use super::RingHomes;

    let none = ThemePatch::new().input(InputRule::new().field(PartPatch::new().focus(None)));
    let mut ringed = 0;
    for (preset, r) in every_preset_with(&none) {
        if super::ring_look(&r, 4.0)?.is_some() {
            ringed += 1;
        }
        let input = r.input.field(WidgetState::FOCUSED);
        assert_eq!(
            super::part_ring_look(&r, &input, RingHomes::INPUT, 4.0)?,
            None,
            "{preset:?}"
        );
    }
    assert!(ringed > 0, "no preset states a global ring");

    let mut fill = (*own_ring()).clone();
    fill.kind = FocusKind::Fill;
    let mut concentric = (*own_ring()).clone();
    concentric.corners = Some(Corners {
        radius: CornerRadius::Concentric {
            inset: 2.0,
            min_radius: 1.0,
        },
        ..Corners::default()
    });
    for (own, home) in [
        (fill, "text_area.field(FOCUSED).focus.kind"),
        (concentric, "text_area.field(FOCUSED).focus.corners"),
    ] {
        let patch = ThemePatch::new().text_area(
            InputRule::new().field(PartPatch::new().focus(Some(std::sync::Arc::new(own)))),
        );
        for (preset, r) in every_preset_with(&patch) {
            let area = r.text_area.field(WidgetState::FOCUSED);
            assert_eq!(
                super::part_ring_look(&r, &area, RingHomes::TEXT_AREA, 4.0).err(),
                Some(Unmapped(home)),
                "{preset:?}"
            );
        }
    }
    Ok(())
}

/// FR4: a part's own ring stated for a focus that followed a key only (a
/// rule at `FOCUS_VISIBLE`) is the part's ring read at `FOCUS_VISIBLE`,
/// while the part read at `FOCUSED` keeps the ring it has without that
/// rule, the global ring where the preset states the field no ring of its
/// own, so the field draws each for its focus
/// ([`super::focus_rings_with`]).
#[cfg(feature = "widgets")]
#[test]
fn a_ring_stated_at_focus_visible_is_the_ring_of_a_focus_that_followed_a_key() -> TestResult {
    use native_theme::next::{InputRule, PartPatch, WidgetState};

    use super::RingHomes;

    let patch = ThemePatch::new().input(
        InputRule::new()
            .states(WidgetState::FOCUS_VISIBLE)
            .field(PartPatch::new().focus(Some(own_ring()))),
    );
    let mut global = 0;
    for ((preset, r), (_, plain)) in every_preset_with(&patch).into_iter().zip(every_preset()) {
        let look = |state| super::part_ring_look(&r, &r.input.field(state), RingHomes::INPUT, 4.0);
        assert_eq!(
            look(WidgetState::FOCUS_VISIBLE)?.map(|look| look.color),
            Some(super::to_color(OUTER)),
            "{preset:?}: the key focus's ring"
        );
        let without = super::part_ring_look(
            &plain,
            &plain.input.field(WidgetState::FOCUSED),
            RingHomes::INPUT,
            4.0,
        )?;
        assert_eq!(
            look(WidgetState::FOCUSED)?,
            without,
            "{preset:?}: another focus's ring"
        );
        if without == super::ring_look(&r, 4.0)? {
            global += 1;
        }
    }
    assert!(global > 0, "no preset's field keeps the global ring");
    Ok(())
}

/// Each form of a stated radius as the one-radius read and the per-corner
/// read take it: a continuous curve drawn as the circular corner of its
/// radius (spec §10, rule 2), a pill as the radius iced clamps to half the
/// shorter side, per-corner radii corner by corner, and as one radius only
/// where the four are one; a concentric radius, which needs the container's,
/// in neither.
#[cfg(feature = "widgets")]
#[test]
fn every_radius_form_is_read_as_iced_rounds_it() -> TestResult {
    use iced_core::border::Radius;
    use native_theme::next::{CornerRadius, Corners, PartStated, PartStyle};

    use super::borders::part_radii;

    let part = |corners: Corners| {
        let mut part = PartStyle::default();
        part.corners = corners;
        part.stated |= PartStated::CORNERS;
        part
    };
    let per_corner = |top_left, top_right, bottom_right, bottom_left| Corners {
        radius: CornerRadius::PerCorner {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        },
        ..Corners::default()
    };
    let continuous = part(Corners::continuous(4.0));
    assert_eq!(super::radius_of(&continuous), Some(4.0));
    assert_eq!(part_radii(&continuous, "h")?, Radius::from(4.0));
    let pill = part(Corners::pill());
    assert_eq!(
        super::radius_of(&pill),
        Some(crate::styles::pill().top_left)
    );
    assert_eq!(part_radii(&pill, "h")?, crate::styles::pill());
    let same = part(per_corner(3.0, 3.0, 3.0, 3.0));
    assert_eq!(super::radius_of(&same), Some(3.0));
    let differ = part(per_corner(1.0, 2.0, 3.0, 4.0));
    assert_eq!(super::radius_of(&differ), None);
    assert_eq!(
        part_radii(&differ, "h")?,
        Radius {
            top_left: 1.0,
            top_right: 2.0,
            bottom_right: 3.0,
            bottom_left: 4.0,
        }
    );
    let concentric = part(Corners {
        radius: CornerRadius::Concentric {
            inset: 2.0,
            min_radius: 1.0,
        },
        ..Corners::default()
    });
    assert_eq!(super::radius_of(&concentric), None);
    assert_eq!(part_radii(&concentric, "h").err(), Some(Unmapped("h")));
    assert_eq!(
        part_radii(&PartStyle::default(), "h").err(),
        Some(Unmapped("h"))
    );
    Ok(())
}

/// A switch whose theme states no unchecked thumb diameter is
/// [`Unmapped`], not drawn with the checked thumb's; every preset states
/// both.
#[cfg(feature = "widgets")]
#[test]
fn an_unstated_unchecked_thumb_is_unmapped() -> TestResult {
    let track = [Some(40.0), Some(20.0)];
    let on = [Some(16.0), Some(16.0)];
    assert_eq!(
        super::switch_look_of(track, on, [None, Some(12.0)]).err(),
        Some(Unmapped("switch.thumb(NONE).min_size[0]"))
    );
    let look = super::switch_look_of(track, on, [Some(12.0), None])?;
    assert_eq!(
        (look.thumb_diameter, look.unchecked_thumb_diameter),
        (16.0, 12.0)
    );
    for (preset, r) in every_preset() {
        let [off, _] = r
            .switch
            .thumb(native_theme::next::WidgetState::NONE)
            .min_size;
        assert_eq!(
            Some(super::switch_look(&r)?.unchecked_thumb_diameter),
            off,
            "{preset:?}"
        );
    }
    Ok(())
}

/// A radio whose theme states no circle width is laid out at iced's own
/// size (`Radio::DEFAULT_SIZE`), its dot's seat too; a stated width is its
/// size.
#[cfg(feature = "widgets")]
#[test]
fn a_radio_of_no_stated_width_is_iceds_own_size() -> TestResult {
    use iced_core::{Element, Size, Theme, layout, widget::Tree};

    type Radio<'a> = iced_widget::Radio<'a, (), Theme, ()>;
    let gap = 6.0;
    let laid_out = |look: crate::RadioLook| -> (f32, Vec<f32>) {
        let mut element: Element<'_, (), Theme, ()> = crate::radio_from(
            look,
            iced_widget::radio::default,
            iced_widget::radio("", 1_u8, None, |_| ()),
            false,
        );
        let mut tree = Tree::new(&element);
        let node = element.as_widget_mut().layout(
            &mut tree,
            &(),
            &layout::Limits::new(Size::ZERO, Size::new(500.0, 100.0)),
        );
        (
            node.size().width,
            node.children().iter().map(|c| c.size().width).collect(),
        )
    };
    let own = Radio::DEFAULT_SIZE;
    let unstated = crate::RadioLook {
        width: None,
        label_gap: gap,
        dot: None,
    };
    assert_eq!(laid_out(unstated).0, own + gap);
    assert_eq!(
        laid_out(crate::RadioLook {
            width: Some(20.0),
            ..unstated
        })
        .0,
        20.0 + gap
    );
    let dotted = laid_out(crate::RadioLook {
        dot: Some((own / 2.0, iced_core::Color::BLACK)),
        ..unstated
    });
    assert_eq!(dotted, (own + gap, vec![own + gap, own]));
    for (preset, r) in every_preset() {
        assert!(super::radio_look(&r)?.width.is_some(), "{preset:?}");
    }
    Ok(())
}

/// A focus indicator that is not a ring is `Unmapped` at `focus_ring.kind` where the ring is
/// mapped (`ring_look`), and nowhere else: the text input and the text editor draw no ring
/// (`focus_ring` paints it over them), so their styles do not read the indicator.
#[cfg(feature = "widgets")]
#[test]
fn a_focus_indicator_that_is_not_a_ring_is_unmapped_at_the_ring_alone() -> TestResult {
    use native_theme::next::FocusKind;

    for (preset, mut r) in every_preset() {
        assert!(super::ring_look(&r, 4.0).is_ok(), "{preset:?}");
        for kind in [FocusKind::Fill, FocusKind::Underline] {
            r.focus_ring.kind = kind;
            assert_eq!(
                super::ring_look(&r, 4.0).err(),
                Some(Unmapped("focus_ring.kind")),
                "{preset:?} {kind:?}"
            );
            assert!(super::styles::text_input(&r).is_ok(), "{preset:?} {kind:?}");
            assert!(
                super::styles::text_editor(&r).is_ok(),
                "{preset:?} {kind:?}"
            );
        }
    }
    Ok(())
}

/// A radio whose circle states no width is iced's own size, the check box's stated width
/// notwithstanding: 0.7.2's fallback to the check box is the table of homes', not the
/// mapping's.
#[cfg(feature = "widgets")]
#[test]
fn a_radio_of_no_stated_width_takes_not_the_check_boxs() -> TestResult {
    use native_theme::next::WidgetState;

    for (preset, r) in every_preset() {
        let [checkbox, _] = r.checkbox.indicator(WidgetState::NONE).min_size;
        assert!(checkbox.is_some(), "{preset:?}: the check box's width");
        let mut circle = r.radio.indicator(WidgetState::NONE);
        circle.min_size = [None, None];
        assert_eq!(super::radio_look_of(&r, &circle)?.width, None, "{preset:?}");
    }
    Ok(())
}

/// `radio_look` reads the radio's own resting circle: the width a rule states for
/// `radio.indicator`, not the check box's.
#[cfg(feature = "widgets")]
#[test]
fn the_radio_look_reads_the_radios_own_circle() -> TestResult {
    use native_theme::next::WidgetState;

    let patch = ThemePatch::from_toml("[[widgets.radio.rules]]\nindicator.min_size = [23, 23]\n")?;
    for (preset, r) in every_preset_with(&patch) {
        let [checkbox, _] = r.checkbox.indicator(WidgetState::NONE).min_size;
        assert_ne!(checkbox, Some(23.0), "{preset:?}");
        assert_eq!(super::radio_look(&r)?.width, Some(23.0), "{preset:?}");
    }
    Ok(())
}

/// A chevron the theme states no size for widens no padding and sizes no
/// handle: the drop-down's right side stays the theme's, and the handle is
/// at the size iced gives its own (`Icon::size` `None`).
#[cfg(feature = "widgets")]
#[test]
fn a_chevron_of_no_stated_size_widens_nothing_and_sizes_no_handle() -> TestResult {
    use iced_widget::pick_list::Handle;

    let size = |handle: Handle<iced_core::Font>| match handle {
        Handle::Static(icon) => Some(icon.size),
        _ => None,
    };
    let r = Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light());
    let chevron = super::chevron_size(&r).ok_or("kde-breeze states its chevron's size")?;
    let column = r
        .combo_box
        .arrow_area_width
        .ok_or("kde-breeze states the chevron's column")?;
    let widened = super::combo_box_padding(&r)?;
    let unwidened = super::combo_box_padding_with(&r, None)?;
    let margin = crate::arrow_column_margin(column, chevron);
    assert!(margin > 0.0);
    assert_eq!(widened, unwidened.right(unwidened.right + margin));
    assert_eq!(size(crate::handle_at::<()>(None)), Some(None));
    assert_eq!(
        size(super::pick_list_handle::<()>(&r)),
        Some(Some(iced_core::Pixels(chevron)))
    );
    Ok(())
}

/// A flare radius that is not positive and finite draws no flares, whatever
/// the theme states for the tab: the radius is checked before the selected
/// tab is read, which a stroke per side makes unreadable.
#[cfg(feature = "widgets")]
#[test]
fn a_flare_radius_that_is_not_positive_and_finite_draws_none() -> TestResult {
    use native_theme::next::{Paint, PartPatch, Stroke, StrokeAlign, StrokeSide, TabRule};

    let side = || StrokeSide {
        width: 1.0,
        paint: Paint::Solid(OUTER),
    };
    let patch = ThemePatch::new().tab(TabRule::new().tab(PartPatch::new().stroke(Stroke::sides(
        side(),
        side(),
        side(),
        side(),
        StrokeAlign::Inside,
    ))));
    for (preset, mut r) in every_preset_with(&patch) {
        r.tab.active_outward_corner_radius = Some(4.0);
        assert!(super::tab_flares_look(&r).is_err(), "{preset:?}");
        for radius in [
            None,
            Some(0.0),
            Some(-1.0),
            Some(f32::NAN),
            Some(f32::INFINITY),
        ] {
            r.tab.active_outward_corner_radius = radius;
            assert_eq!(super::tab_flares_look(&r)?, None, "{preset:?} {radius:?}");
        }
    }
    Ok(())
}

/// A checked toolbar button on either macOS preset is filled with the
/// checked fill the preset states, `toolbar.button(CHECKED).fill`, the
/// toolbar-bezel button's as an on toggle: macos-monterey #dfdfdf /
/// #414141, macos-tahoe the accent, #007aff (os-probe run 45634379188,
/// toolbar states sheet, `*.normal.toolbar_toggle_on` `fill`), at rest.
/// Only macos-monterey proves the stated fill is read: an unstated one is the
/// primary button's, which on macos-tahoe is the same accent, and iced reads
/// no hovered or pressed checked shade (`toolbar.button(CHECKED | HOVER)`)
/// that could tell the two apart.
#[cfg(feature = "widgets")]
#[test]
fn a_checked_macos_toolbar_button_takes_the_stated_checked_fill() -> TestResult {
    use iced_core::Background;
    use iced_core::theme::Theme as IcedTheme;
    use iced_widget::button::Status;
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
            let style =
                super::styles::toolbar_button(&theme, true)?(&IcedTheme::Light, Status::Active);
            assert_eq!(
                style.background,
                Some(Background::Color(super::to_color(Color::from_srgb8(
                    r,
                    g,
                    b,
                    u8::MAX
                )))),
                "{preset:?} {:?}",
                ctx.scheme
            );
        }
    }
    Ok(())
}
