//! §4.7's free accessors: the values no `Style` field carries. Task 11: `scaled_text_size`;
//! Task 23 adds the rest.

use crate::AccessibilityPreferences;

/// A text size from the theme times the user's text-scaling factor; a factor that is not
/// finite and positive is ignored. Same signature and semantics as
/// `native_theme_iced::scaled_text_size` (`connectors/native-theme-iced/src/lib.rs:480`) and
/// `native_theme_gpui::scaled_text_size` (`connectors/native-theme-gpui/src/lib.rs:444`).
///
/// For a size no accessor above returns — a widget font read from the `ResolvedTheme`
/// directly (`t.button.font.size`, …) and drawn at a call site. Takes the preferences, not a
/// `&SystemTheme`, so the preset path can pass [`ThemeAtlas::accessibility`](crate::ThemeAtlas::accessibility).
#[must_use]
pub fn scaled_text_size(size: f32, prefs: &AccessibilityPreferences) -> f32 {
    size * text_scale_factor(prefs)
}

/// The text-scaling multiplier: the factor when it is finite and positive, else `1.0`, the
/// multiplicative identity (§6.17) — the rule both siblings apply, iced's own
/// `text_scale_factor` being `connectors/native-theme-iced/src/lib.rs:486-489`.
fn text_scale_factor(prefs: &AccessibilityPreferences) -> f32 {
    let s = prefs.text_scaling_factor;
    if s.is_finite() && s > 0.0 { s } else { 1.0 }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "a test fails by panicking"
)]
mod tests {
    use super::*;

    /// §4.7, the siblings' rule: the factor applies when finite and positive, else the size
    /// is returned unchanged (T13 (d) repeats this over the atlas in Task 32).
    #[test]
    fn scaled_text_size_ignores_a_factor_that_is_not_finite_and_positive() {
        let prefs = |f: f32| AccessibilityPreferences {
            text_scaling_factor: f,
            ..AccessibilityPreferences::default()
        };
        assert_eq!(scaled_text_size(10.0, &prefs(1.5)), 15.0);
        for f in [0.0, -1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert_eq!(scaled_text_size(10.0, &prefs(f)), 10.0, "{f}");
        }
    }
}
