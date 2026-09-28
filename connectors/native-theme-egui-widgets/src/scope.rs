//! What every widget does first: find the installed atlas, and open its role's scope.

use native_theme_egui::convert::clamp_length;
use native_theme_egui::egui;
use native_theme_egui::{NativeThemeUiExt as _, Role, RoleVariant};

/// Run `add` in `role`'s scope: the `Disabled` variant with `Ui::disable` called inside it
/// when `enabled` is `false` or the calling `Ui` is disabled (`egui/src/ui.rs:497-502`), the
/// `Normal` one otherwise. The `Disabled` cell carries the platform's disabled colours and
/// the role's `disabled_opacity` as `disabled_alpha`, so egui fades the widget by it on top:
/// both of the platform's disabled mechanisms, one of which its data makes an identity
/// (docs/platform-facts.md §2.1.6).
pub(crate) fn open<R>(
    ui: &mut egui::Ui,
    role: Role,
    enabled: bool,
    add: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    open_selected(ui, role, false, enabled, add)
}

/// [`open`], with the `Selected` variant for an enabled control that is `selected`: the
/// checked appearance of a `Checkbox` or `RadioButton` (the connector's `RoleVariant`).
pub(crate) fn open_selected<R>(
    ui: &mut egui::Ui,
    role: Role,
    selected: bool,
    enabled: bool,
    add: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    // A disabled calling `Ui` has faded its painter already, and `Ui::disable` multiplies the
    // opacity again on every call (`egui/src/ui.rs:497-502`), so only an enabled one is
    // disabled here.
    let outer = ui.is_enabled();
    let variant = match (enabled && outer, selected) {
        (false, _) => RoleVariant::Disabled,
        (true, true) => RoleVariant::Selected,
        (true, false) => RoleVariant::Normal,
    };
    ui.native_scope(role, variant, |ui| {
        if !enabled && outer {
            ui.disable();
        }
        add(ui)
    })
    .inner
}

/// A length a painted widget's geometry is built from: `None` where the theme's value is not
/// finite, which sends the widget to egui's own counterpart, since no egui sink holds that
/// geometry; else the value through `clamp_length`.
pub(crate) fn length(v: f32) -> Option<f32> {
    v.is_finite().then(|| clamp_length(v))
}

#[cfg(test)]
mod tests {
    use super::length;

    #[test]
    fn a_length_that_is_not_finite_is_none() {
        assert_eq!(length(f32::NAN), None);
        assert_eq!(length(f32::INFINITY), None);
        assert_eq!(length(f32::NEG_INFINITY), None);
        assert_eq!(length(-0.0), Some(0.0));
        assert_eq!(length(-3.0), Some(0.0));
        assert_eq!(length(18.0), Some(18.0));
    }
}
