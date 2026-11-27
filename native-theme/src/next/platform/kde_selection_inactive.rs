//! `kde_selection_inactive`: KColorScheme's Selection colours of the inactive state.

use super::KdeRoleKind;
use super::kde_color_common::{Rgba, enter, read, tint_q};

/// The inactive Selection colour of a role, before the inactive state effects: `selection`
/// itself when `inactive_change_selection_color` is false; otherwise, for a background role,
/// `window` tinted toward `selection_background` by 0.4, and for a foreground or decoration
/// role, `window`.
pub(crate) fn kde_selection_inactive(
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
    use super::super::vectors::{TestResult, assert_color, boolean, color, output, rows};
    use super::*;

    fn role_kind(cell: &str) -> Result<KdeRoleKind, String> {
        match cell {
            "foreground" => Ok(KdeRoleKind::Foreground),
            "decoration" => Ok(KdeRoleKind::Decoration),
            "background" => Ok(KdeRoleKind::Background),
            other => Err(format!("no role kind `{other}`")),
        }
    }

    #[test]
    fn vectors() -> TestResult {
        let rows = rows(include_str!(
            "../../../tests/vectors/kde_selection_inactive.tsv"
        ))?;
        assert_eq!(rows.len(), 70, "row count");
        for row in &rows {
            let got = kde_selection_inactive(
                color(row.get("selection")?)?,
                color(row.get("window")?)?,
                color(row.get("selection_background")?)?,
                role_kind(row.get("role_kind")?)?,
                boolean(row.get("inactive_change_selection_color")?)?,
            );
            assert_color(got, output(row.get("result")?)?, row);
        }
        Ok(())
    }
}
