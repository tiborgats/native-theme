//! [`crate::status`] for the 0.8.0 model: the status bar's font item from a
//! `native_theme::next::TextStyle` (`typography.body`, 0.7.2's
//! `defaults.font`) and its accessibility items from the resolved theme's
//! `ThemeContext`. The desktop and the family labels read no model and are
//! [`crate::status`]'s.

use native_theme::next::{
    Contrast, FontSize, MotionPreference, TextStated, TextStyle, ThemeContext,
};

use crate::status::{DrawnFamily, desktop};

/// A font size in the unit its source stated, never converted: `10.5pt`,
/// `14px`, or `(size not stated)`, as [`crate::status::defined_size`] gives
/// it, from `TextStyle::defined_size`.
#[must_use]
pub fn defined_size(font: &TextStyle) -> String {
    match font.defined_size {
        Some(FontSize::Pt(v)) => format!("{v}pt"),
        Some(FontSize::Px(v)) => format!("{v}px"),
        None => "(size not stated)".to_string(),
    }
}

/// The status bar's items, as [`crate::status::environment_drawn`]'s: the
/// desktop, `<preset> <mode>`, the font as [`DrawnFamily::label`] of `drawn`
/// for its family (`(family not stated)` where the style states none) and
/// its [`defined_size`], then, where `context` is given (a native theme is
/// installed), `text ×<text_scale>` and each of 0.7.2's accessibility
/// preferences the context sets, by 0.7.2's field name, as the table of
/// homes maps them (`native-theme/tests/next_homes.toml`, the
/// `accessibility.*` rows): `reduce_motion` where `motion` is not
/// `MotionPreference::Full`, `high_contrast` where `contrast` is not
/// `Contrast::Normal`, and `reduce_transparency` where
/// `reduced_transparency` is set.
#[must_use]
pub fn environment_drawn(
    preset_label: &str,
    mode: &str,
    font: &TextStyle,
    drawn: DrawnFamily<'_>,
    context: Option<&ThemeContext>,
) -> Vec<String> {
    let family = if font.stated.contains(TextStated::FAMILY) {
        drawn.label(&font.family)
    } else {
        "(family not stated)".to_string()
    };
    let mut items = vec![
        desktop(),
        format!("{preset_label} {mode}"),
        format!("{family} {}", defined_size(font)),
    ];
    if let Some(context) = context {
        items.push(format!("text ×{}", context.text_scale));
        let flags = [
            ("reduce_motion", context.motion != MotionPreference::Full),
            ("high_contrast", context.contrast != Contrast::Normal),
            ("reduce_transparency", context.reduced_transparency),
        ];
        items.extend(
            flags
                .into_iter()
                .filter(|(_, set)| *set)
                .map(|(name, _)| name.to_string()),
        );
    }
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    use native_theme::next::{Preset, Theme};

    use crate::status::family_label;

    /// kde-breeze light, resolved for the light scheme's default context.
    fn kde_breeze_light() -> native_theme::next::ResolvedTheme {
        Theme::preset(Preset::KdeBreeze).resolve(&ThemeContext::light())
    }

    /// A size in the unit its source stated, or that none is stated.
    #[test]
    fn a_size_keeps_its_stated_unit() {
        let mut font = kde_breeze_light().typography.body;
        font.defined_size = Some(FontSize::Pt(10.5));
        assert_eq!(defined_size(&font), "10.5pt");
        font.defined_size = Some(FontSize::Px(14.0));
        assert_eq!(defined_size(&font), "14px");
        font.defined_size = None;
        assert_eq!(defined_size(&font), "(size not stated)");
    }

    /// The desktop, the preset and mode, the font; then, with a context, its
    /// text scale and each preference it sets by 0.7.2's field name.
    #[test]
    fn the_environment_names_its_items_in_order() {
        let r = kde_breeze_light();
        let font = &r.typography.body;
        let font_item = format!("{} {}", font.family, defined_size(font));
        assert_eq!(
            environment_drawn(
                "default (kde-breeze)",
                "light",
                font,
                DrawnFamily::from(None),
                None
            ),
            [
                desktop(),
                "default (kde-breeze) light".to_string(),
                font_item,
            ]
        );
        let mut context = r.context.clone();
        context.text_scale = 1.25;
        context.motion = MotionPreference::Reduced;
        context.contrast = Contrast::Normal;
        context.reduced_transparency = true;
        assert_eq!(
            environment_drawn(
                "adwaita",
                "dark",
                font,
                DrawnFamily::from(Some("Cantarell")),
                Some(&context)
            ),
            [
                desktop(),
                "adwaita dark".to_string(),
                format!(
                    "{} {}",
                    family_label(&font.family, Some("Cantarell")),
                    defined_size(font)
                ),
                "text ×1.25".to_string(),
                "reduce_motion".to_string(),
                "reduce_transparency".to_string(),
            ]
        );
        assert_eq!(
            environment_drawn(
                "adwaita",
                "dark",
                font,
                DrawnFamily::from(None),
                Some(&r.context)
            )
            .last(),
            Some(&format!("text ×{}", r.context.text_scale))
        );
    }

    /// Each non-default motion and contrast is a preference set; a style
    /// that states no family says so.
    #[test]
    fn every_reduced_motion_and_raised_contrast_is_named() {
        let r = kde_breeze_light();
        let mut context = r.context.clone();
        for (motion, contrast) in [
            (MotionPreference::None, Contrast::Increased),
            (MotionPreference::Reduced, Contrast::High),
        ] {
            context.motion = motion;
            context.contrast = contrast;
            let items = environment_drawn(
                "kde-breeze",
                "light",
                &r.typography.body,
                DrawnFamily::Stated,
                Some(&context),
            );
            assert_eq!(
                items.get(4..),
                Some(&["reduce_motion".to_string(), "high_contrast".to_string()][..]),
                "{motion:?} {contrast:?}"
            );
        }
        let mut font = r.typography.body.clone();
        font.stated = TextStated::default();
        let items = environment_drawn("kde-breeze", "light", &font, DrawnFamily::Stated, None);
        assert_eq!(
            items.get(2),
            Some(&format!("(family not stated) {}", defined_size(&font)))
        );
    }
}
