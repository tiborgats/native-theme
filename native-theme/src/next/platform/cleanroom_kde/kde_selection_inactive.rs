//! `kde_selection_inactive`: KColorScheme's Selection colours of the inactive state.

#[allow(dead_code)]
#[path = "kde_color_common.rs"]
mod kde_color_common;

pub use kde_color_common::KdeRoleKind;
use kde_color_common::{enter, read, tint_q, Rgba};

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

#[cfg(test)]
mod tests {
    use super::kde_color_common::table::{boolean, check, colour};
    use super::{kde_selection_inactive, KdeRoleKind};

    #[test]
    fn vectors() {
        let (total, failures) =
            check(include_str!("../../vectors/kde_selection_inactive.tsv"), |t, row| {
                Some(kde_selection_inactive(
                    colour(t.cell(row, "selection")?)?,
                    colour(t.cell(row, "window")?)?,
                    colour(t.cell(row, "selection_background")?)?,
                    KdeRoleKind::from_name(t.cell(row, "role_kind")?)?,
                    boolean(t.cell(row, "inactive_change_selection_color")?)?,
                ))
            });
        assert!(total > 0, "no rows");
        assert!(failures.is_empty(), "{} of {total} rows fail:\n{}", failures.len(), failures.join("\n"));
    }
}
