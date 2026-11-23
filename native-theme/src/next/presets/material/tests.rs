use super::*;

use crate::next::preset::Theme;
use crate::next::query::{ButtonQuery, MotionPreference};
use crate::next::window::WindowState;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn both_schemes() -> [ResolvedTheme; 2] {
    [ThemeContext::light(), ThemeContext::dark()]
        .map(|ctx| Theme::preset(Preset::Material3).resolve(&ctx))
}

#[test]
fn the_preset_defines_the_normal_contrast_alone() {
    assert_eq!(CONTRAST_LEVELS, &[Contrast::Normal]);
    let ctx = ThemeContext::light().with_contrast(Contrast::High);
    let high = Theme::preset(Preset::Material3).resolve(&ctx);
    assert_eq!(high.context.contrast, Contrast::Normal);
}

#[test]
fn the_constant_colours_are_their_8_bit_channels() {
    assert_eq!(LIGHT.hover_layer.to_srgb8(), [0x1d, 0x1b, 0x20, 0x14]);
    assert_eq!(DARK.disabled_track.to_srgb8(), [0x36, 0x34, 0x3b, 0x1f]);
    assert_eq!(TRANSPARENT.to_srgb8(), [0x00, 0x00, 0x00, 0x00]);
}

/// A primary button keeps the accent under the pointer and pressed, and
/// takes the disabled container when disabled.
#[test]
fn a_primary_button_keeps_the_accent_until_disabled() {
    for theme in both_schemes() {
        let primary = |state| ButtonQuery::primary().state(state);
        let accent = Paint::Solid(theme.colors.accent.fill);
        for state in [
            WidgetState::NONE,
            WidgetState::HOVER,
            WidgetState::PRESSED,
            WidgetState::CHECKED,
        ] {
            assert_eq!(theme.button.body(primary(state)).fill, accent, "{state:?}");
        }
        assert_ne!(
            theme.button.body(primary(WidgetState::DISABLED)).fill,
            accent
        );
    }
}

/// The focused text field's outline wins over the hovered one.
#[test]
fn a_focused_field_keeps_its_outline_under_the_pointer() {
    for theme in both_schemes() {
        let focused = theme.input.field(WidgetState::FOCUSED).stroke;
        assert_eq!(
            theme
                .input
                .field(WidgetState::FOCUSED | WidgetState::HOVER)
                .stroke,
            focused
        );
        assert_ne!(theme.input.field(WidgetState::HOVER).stroke, focused);
    }
}

/// The switch's thumb grows when on, and a disabled thumb takes its own
/// colour, on or off.
#[test]
fn the_switch_s_thumb_grows_when_on() {
    for theme in both_schemes() {
        let off = theme.switch.thumb(WidgetState::NONE);
        let on = theme.switch.thumb(WidgetState::CHECKED);
        assert_eq!(off.min_size, [Some(SWITCH_UNCHECKED_THUMB_DIAMETER); 2]);
        assert_eq!(on.min_size, [Some(SWITCH_THUMB_DIAMETER); 2]);
        assert_eq!(
            theme
                .switch
                .thumb(WidgetState::DISABLED | WidgetState::CHECKED)
                .fill,
            theme.switch.thumb(WidgetState::DISABLED).fill
        );
    }
}

/// Right to left mirrors the per-side values the preset states unequal on
/// the left and the right, the combo box's padding, and nothing else.
#[test]
fn right_to_left_mirrors_the_unequal_sides() {
    for ctx in [ThemeContext::light(), ThemeContext::dark()] {
        let theme = Theme::preset(Preset::Material3);
        let ltr = theme.resolve(&ctx);
        let rtl = theme.resolve(&ctx.clone().with_direction(Direction::Rtl));
        let mirrored = |e: Edges| Edges::new(e.top, e.left, e.bottom, e.right);
        let field = ltr.combo_box.field(WidgetState::NONE).padding;
        assert_eq!(field.left, COMBO_BOX_PADDING_LEFT);
        assert_eq!(
            rtl.combo_box.field(WidgetState::NONE).padding,
            mirrored(field)
        );
        let mut same = rtl.clone();
        same.context = ltr.context.clone();
        same.combo_box = ltr.combo_box.clone();
        assert_eq!(same, ltr);
    }
}

/// The text scale of the test below.
const TEXT_SCALE_TWO: f32 = 2.0;

/// The text scale is Android's non-linear one: at 2.0 the body's 14 px is
/// the 2.0 table's 26 (`FontScaleConverterFactory`'s row for 14 sp), every
/// font size goes through the converter, each line height keeps its ratio to
/// its font's unscaled size (above Compose's threshold), each `defined_size`
/// stays the unscaled size, and nothing but the text changes.
#[test]
fn at_text_scale_2_the_sizes_follow_android_s_converter() -> TestResult {
    for scheme in [Scheme::Light, Scheme::Dark] {
        let ctx = ThemeContext::default().with_scheme(scheme);
        let base = Theme::preset(Preset::Material3).resolve(&ctx);
        let double = Theme::preset(Preset::Material3).resolve(&ctx.with_text_scale(TEXT_SCALE_TWO));
        assert_eq!(double.typography.body.size_px, 26.0, "{scheme:?}");
        let slots = |t: &ResolvedTheme| -> Result<Vec<TextStyle>, String> {
            let entry = |slot: &Option<TextStyle>, name: &str| {
                slot.clone().ok_or_else(|| format!("no {name}"))
            };
            Ok(vec![
                t.typography.body.clone(),
                t.typography.monospace.clone(),
                entry(&t.typography.display, "display")?,
                entry(&t.typography.title1, "title1")?,
                entry(&t.typography.headline, "headline")?,
                entry(&t.typography.caption, "caption")?,
                t.button.label(ButtonQuery::new(WidgetState::NONE)),
                t.tooltip.text.clone(),
                t.dialog.title.clone(),
                t.window.title(WindowState::default()),
            ])
        };
        for (one, two) in slots(&base)?.into_iter().zip(slots(&double)?) {
            let size = one.defined_size.ok_or("no defined size")?;
            assert_eq!(two.defined_size, Some(size), "{scheme:?}");
            assert_eq!(
                two.size_px,
                android_non_linear(one.size_px, TEXT_SCALE_TWO),
                "{scheme:?}"
            );
            assert_eq!(
                two.line_height,
                one.line_height / android_non_linear_inverse(two.size_px, TEXT_SCALE_TWO)
                    * two.size_px,
                "{scheme:?}"
            );
        }
        let rest = ButtonQuery::new(WidgetState::NONE);
        assert_eq!(double.button.body(rest), base.button.body(rest));
        assert_eq!(double.layout, base.layout);
        assert_eq!(double.icons, base.icons);
    }
    Ok(())
}

/// `MotionPreference::None` sets the dialog's entrance to
/// `DialogEntranceStyle::None`; Material's `ScaleFade` stays under full and
/// under reduced motion. The preset states no motion token.
#[test]
fn no_motion_removes_the_dialog_s_entrance() {
    let resolve = |motion: MotionPreference| {
        Theme::preset(Preset::Material3).resolve(&ThemeContext::light().with_motion(motion))
    };
    let none = resolve(MotionPreference::None);
    assert_eq!(none.dialog.entrance, Some(DialogEntranceStyle::None));
    assert_eq!(none.motion_tokens, MotionTokens::default());
    for motion in [MotionPreference::Full, MotionPreference::Reduced] {
        let theme = resolve(motion);
        assert_eq!(
            theme.dialog.entrance,
            Some(DialogEntranceStyle::ScaleFade),
            "{motion:?}"
        );
        assert_eq!(theme.motion_tokens, MotionTokens::default(), "{motion:?}");
    }
}
