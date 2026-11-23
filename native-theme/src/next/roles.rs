//! Semantic colour roles: the surfaces, the selections, the accent, the
//! status colours and the global roles a theme states.

use super::color::Color;
use super::query::Surface;

/// The colours of one surface. Each is `None` where no cited source states
/// it for that surface.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SurfaceColors {
    /// The background.
    pub background: Option<Color>,
    /// The text and icon colour.
    pub foreground: Option<Color>,
    /// The muted (secondary) text colour.
    pub foreground_muted: Option<Color>,
    /// The border colour.
    pub border: Option<Color>,
    /// The divider colour.
    pub divider: Option<Color>,
}

impl SurfaceColors {
    /// Surface colours of these fields.
    #[must_use]
    pub fn new(
        background: Option<Color>,
        foreground: Option<Color>,
        foreground_muted: Option<Color>,
        border: Option<Color>,
        divider: Option<Color>,
    ) -> Self {
        Self {
            background,
            foreground,
            foreground_muted,
            border,
            divider,
        }
    }
}

/// The colours of a selection.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SelectionColors {
    /// The selection's background.
    pub background: Color,
    /// The selected text's colour.
    pub foreground: Color,
    /// The background where the view lacks the focus.
    pub unfocused_background: Option<Color>,
    /// The text colour where the view lacks the focus.
    pub unfocused_foreground: Option<Color>,
}

impl SelectionColors {
    /// Selection colours of these fields.
    #[must_use]
    pub fn new(
        background: Color,
        foreground: Color,
        unfocused_background: Option<Color>,
        unfocused_foreground: Option<Color>,
    ) -> Self {
        Self {
            background,
            foreground,
            unfocused_background,
            unfocused_foreground,
        }
    }
}

/// The accent colours.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AccentColors {
    /// The accent's fill.
    pub fill: Color,
    /// The text and icon colour on the fill.
    pub foreground: Color,
    /// The accent as text on the window and view backgrounds, adjusted for
    /// contrast where a source states it (e.g. libadwaita's OKLab rule).
    pub standalone: Option<Color>, // WCAG-adjusted foreground text on window/view backgrounds where sourced (e.g., libadwaita OKLab)
}

/// Where a status colour comes from.
///
/// `adwaita`, `kde-breeze` and `windows-11` use dedicated `Native` tokens;
/// `macos-*` and `ios-*` use Apple's `systemRed`, `systemOrange`,
/// `systemGreen` and `systemBlue` (`SystemColor`); `material-3` uses `Native`
/// for `error` and HCT harmonisation toward the primary colour (`Harmonized`)
/// for `warning`, `success` and `info`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub enum RoleOrigin {
    /// The platform's own token for the role.
    Native,
    /// A system colour the platform names (Apple's `system*` colours).
    SystemColor,
    /// Harmonised toward the primary colour (Material's HCT `harmonize`).
    Harmonized,
    /// Supplied by a palette, a patch or a transform.
    Custom,
}

/// The colours of one status role.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StatusColors {
    /// The role's fill.
    pub fill: Color,
    /// The text and icon colour on the fill.
    pub foreground: Color,
    /// The role as text on the window and view backgrounds, where a source
    /// states it.
    pub standalone: Option<Color>,
    /// Where the colours come from.
    pub origin: RoleOrigin,
}

/// The semantic colour roles of a resolved theme.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ColorRoles {
    // 9 named surfaces (`window` always populates `background`, `foreground`, `foreground_muted`, `border`
    // on every bundled preset; every other surface slot stays `None` where no cited source states it)
    /// A window's background surface; `background`, `foreground`,
    /// `foreground_muted` and `border` are stated on every bundled preset.
    pub window: SurfaceColors,
    /// A content view; its `background` is the content surface's
    /// background, 0.7.2's `defaults.surface_color`, on every preset.
    pub view: SurfaceColors, // `view.background` is the content surface's background: 0.7.2's `defaults.surface_color`, on every preset
    /// A header bar.
    pub header: SurfaceColors,
    /// A sidebar.
    pub sidebar: SurfaceColors,
    /// A card.
    pub card: SurfaceColors,
    /// A dialog.
    pub dialog: SurfaceColors,
    /// A popover.
    pub popover: SurfaceColors,
    /// A tooltip.
    pub tooltip: SurfaceColors,
    /// An on-screen display.
    pub osd: SurfaceColors,
    // Selection & highlights
    /// The selection in a view (rows, items).
    pub selection: SelectionColors,
    /// The selection of text.
    pub text_selection: SelectionColors,
    /// The highlight of a find match.
    pub find_highlight: Option<Color>,
    /// The scrim that dims what lies behind a modal.
    pub scrim: Option<Color>,
    // Global semantic roles
    /// The accent.
    pub accent: AccentColors,
    /// The focus ring's colour.
    pub focus_ring: Color,
    /// The text caret's colour, stated here only ([`CaretSpec`] has none).
    ///
    /// [`CaretSpec`]: crate::next::CaretSpec
    pub caret: Color, // the text caret's colour, stated here only (`CaretSpec` has none)
    /// A link's colour.
    pub link: Color,
    /// A visited link's colour.
    pub link_visited: Option<Color>,
    /// Disabled text's colour.
    pub disabled_text: Color,
    /// The shadow colour.
    pub shadow: Color,
    /// The opacity of a disabled element of custom application UI; 1.0
    /// where the platform dims by colour alone. Not to be applied again on
    /// a widget part queried with `DISABLED`.
    pub disabled_opacity: f32, // 1.0 where the platform dims by colour alone; for custom app UI (do not double-apply on `DISABLED` widget parts)
    // Always-present concrete status roles
    /// The error role.
    pub error: StatusColors,
    /// The warning role.
    pub warning: StatusColors,
    /// The success role.
    pub success: StatusColors,
    /// The information role.
    pub info: StatusColors,
}

impl ColorRoles {
    /// The colours of `surface`.
    #[must_use]
    pub fn surface(&self, surface: Surface) -> &SurfaceColors {
        match surface {
            Surface::Window => &self.window,
            Surface::View => &self.view,
            Surface::HeaderBar => &self.header,
            Surface::Sidebar => &self.sidebar,
            Surface::Card => &self.card,
            Surface::Dialog => &self.dialog,
            Surface::Popover => &self.popover,
            Surface::Tooltip => &self.tooltip,
            Surface::Osd => &self.osd,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An opaque grey of the 8-bit value `v`, to tell the surfaces apart.
    fn grey(v: u8) -> Color {
        Color::from_srgb8(v, v, v, u8::MAX)
    }

    fn surface(v: u8) -> SurfaceColors {
        SurfaceColors::new(Some(grey(v)), None, None, None, None)
    }

    fn status(origin: RoleOrigin) -> StatusColors {
        StatusColors {
            fill: Color::BLACK,
            foreground: Color::WHITE,
            standalone: None,
            origin,
        }
    }

    fn roles() -> ColorRoles {
        let selection = SelectionColors::new(Color::BLACK, Color::WHITE, None, None);
        ColorRoles {
            window: surface(1),
            view: surface(2),
            header: surface(3),
            sidebar: surface(4),
            card: surface(5),
            dialog: surface(6),
            popover: surface(7),
            tooltip: surface(8),
            osd: surface(9),
            selection,
            text_selection: selection,
            find_highlight: None,
            scrim: None,
            accent: AccentColors {
                fill: Color::BLACK,
                foreground: Color::WHITE,
                standalone: None,
            },
            focus_ring: Color::BLACK,
            caret: Color::BLACK,
            link: Color::BLACK,
            link_visited: None,
            disabled_text: Color::BLACK,
            shadow: Color::BLACK,
            disabled_opacity: 1.0,
            error: status(RoleOrigin::Native),
            warning: status(RoleOrigin::Native),
            success: status(RoleOrigin::Native),
            info: status(RoleOrigin::Native),
        }
    }

    #[test]
    fn surface_maps_every_surface_to_its_slot() {
        let roles = roles();
        for (s, expected) in [
            (Surface::Window, 1),
            (Surface::View, 2),
            (Surface::HeaderBar, 3),
            (Surface::Sidebar, 4),
            (Surface::Card, 5),
            (Surface::Dialog, 6),
            (Surface::Popover, 7),
            (Surface::Tooltip, 8),
            (Surface::Osd, 9),
        ] {
            assert_eq!(roles.surface(s), &surface(expected), "{s:?}");
        }
    }

    #[test]
    fn constructors_and_defaults() {
        let red = Color::rgb(1.0, 0.0, 0.0);
        let s = SurfaceColors::new(
            Some(red),
            Some(Color::WHITE),
            None,
            Some(Color::BLACK),
            None,
        );
        assert_eq!(s.background, Some(red));
        assert_eq!(s.foreground, Some(Color::WHITE));
        assert_eq!(s.foreground_muted, None);
        assert_eq!(s.border, Some(Color::BLACK));
        assert_eq!(s.divider, None);
        let d = SurfaceColors::default();
        assert_eq!(d, SurfaceColors::new(None, None, None, None, None));
        let sel = SelectionColors::new(red, Color::WHITE, Some(Color::BLACK), None);
        assert_eq!(sel.background, red);
        assert_eq!(sel.foreground, Color::WHITE);
        assert_eq!(sel.unfocused_background, Some(Color::BLACK));
        assert_eq!(sel.unfocused_foreground, None);
    }
}
