//! `kde_selection_inactive`: KColorScheme's Selection colours of the inactive state.

use super::kde_color_common::{Rgba, enter, read, tint_q};
use crate::tokens::KdeRoleKind;

/// The inactive Selection colour of a role, before the inactive state effects: `selection`
/// itself when `inactive_change_selection_color` is false; otherwise, for a background role,
/// `window` tinted toward `selection_background` by 0.4, and for a foreground or decoration
/// role, `window`.
pub fn kde_selection_inactive(
    selection: Rgba,
    window: Rgba,
    selection_background: Rgba,
    role_kind: KdeRoleKind,
    inactive_change_selection_color: bool,
) -> Rgba {
    let selection = enter(selection);
    let window = enter(window);
    let selection_background = enter(selection_background);
    if !inactive_change_selection_color {
        return read(selection);
    }
    match role_kind {
        KdeRoleKind::Background => read(tint_q(window, selection_background, 0.4)),
        KdeRoleKind::Foreground | KdeRoleKind::Decoration => read(window),
    }
}
