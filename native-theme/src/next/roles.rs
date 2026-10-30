//! Colour roles (S3).
//!
//! The marks of what may be absent are the catalogue's (S6.3's second
//! source), per field the union over the presets that may leave it absent.

use native_theme_derive::ThemeParts;

use super::color::Color;
use super::key::Key;
use super::value::{SurfaceMap, Value};

/// A surface set: a group of colour roles a part draws on. A preset declares
/// the sets it has; `Window` and `Selection` every preset declares.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SurfaceSet {
    /// A window's own surface.
    Window,
    /// A view: a list, a text field, a content area.
    View,
    /// A button.
    Button,
    /// A header bar or title bar.
    Header,
    /// A sidebar.
    Sidebar,
    /// A secondary sidebar.
    SecondarySidebar,
    /// A card.
    Card,
    /// A popover or menu.
    Popover,
    /// A dialog.
    Dialog,
    /// A tooltip.
    Tooltip,
    /// Selected items.
    Selection,
    /// KDE's complementary set: a surface in the opposite scheme.
    Complementary,
    /// An on-screen display.
    Osd,
}

impl SurfaceSet {
    /// Every surface set, in declaration order.
    pub(crate) const ALL: [SurfaceSet; 13] = [
        SurfaceSet::Window,
        SurfaceSet::View,
        SurfaceSet::Button,
        SurfaceSet::Header,
        SurfaceSet::Sidebar,
        SurfaceSet::SecondarySidebar,
        SurfaceSet::Card,
        SurfaceSet::Popover,
        SurfaceSet::Dialog,
        SurfaceSet::Tooltip,
        SurfaceSet::Selection,
        SurfaceSet::Complementary,
        SurfaceSet::Osd,
    ];

    /// Every surface set's name, in [`SurfaceSet::ALL`]'s order.
    pub(crate) const NAMES: [&'static str; 13] = [
        "window",
        "view",
        "button",
        "header",
        "sidebar",
        "secondary_sidebar",
        "card",
        "popover",
        "dialog",
        "tooltip",
        "selection",
        "complementary",
        "osd",
    ];

    /// The set's name in TOML and in expressions, its variant in snake_case.
    #[must_use]
    pub(crate) const fn name(self) -> &'static str {
        match self {
            SurfaceSet::Window => "window",
            SurfaceSet::View => "view",
            SurfaceSet::Button => "button",
            SurfaceSet::Header => "header",
            SurfaceSet::Sidebar => "sidebar",
            SurfaceSet::SecondarySidebar => "secondary_sidebar",
            SurfaceSet::Card => "card",
            SurfaceSet::Popover => "popover",
            SurfaceSet::Dialog => "dialog",
            SurfaceSet::Tooltip => "tooltip",
            SurfaceSet::Selection => "selection",
            SurfaceSet::Complementary => "complementary",
            SurfaceSet::Osd => "osd",
        }
    }

    /// The set `name` names.
    #[must_use]
    pub(crate) fn from_name(name: &str) -> Option<SurfaceSet> {
        SurfaceSet::ALL.into_iter().find(|set| set.name() == name)
    }
}

/// The colour roles every declared surface set states (S3): the common role
/// record, its fields as the catalogue's *role record* closes them.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct SurfaceRoles {
    /// The set's fill.
    pub background: Value<Color>,
    /// The alternate row or band fill (KDE's `BackgroundAlternate`).
    #[parts(may_be_absent)]
    pub background_alternate: Value<Color>,
    /// The primary text and glyph colour.
    pub foreground: Value<Color>,
    /// The platform's secondary (muted) text colour.
    pub foreground_inactive: Value<Color>,
    /// KDE's `ForegroundActive`, the information colour.
    #[parts(may_be_absent)]
    pub foreground_active: Value<Color>,
    /// Link text.
    pub link: Value<Color>,
    /// Visited link text.
    #[parts(may_be_absent)]
    pub link_visited: Value<Color>,
    /// The set's negative status text colour (KDE's `ForegroundNegative`).
    pub negative: Value<Color>,
    /// The set's neutral status text colour, a warning (KDE's `ForegroundNeutral`).
    #[parts(may_be_absent)]
    pub neutral: Value<Color>,
    /// The set's positive status text colour (KDE's `ForegroundPositive`).
    #[parts(may_be_absent)]
    pub positive: Value<Color>,
    /// KDE's focus decoration colour of the set.
    #[parts(may_be_absent)]
    pub decoration_focus: Value<Color>,
    /// KDE's hover decoration colour of the set.
    #[parts(may_be_absent)]
    pub decoration_hover: Value<Color>,
    /// The set's translucent shade line or overlay (libadwaita's `*_shade_color`).
    #[parts(may_be_absent)]
    pub shade: Value<Color>,
    /// The set's darker shade (libadwaita's `headerbar_darker_shade_color`).
    #[parts(may_be_absent)]
    pub shade_darker: Value<Color>,
    /// The set's stroke colour, translucent where the platform's is.
    pub border: Value<Color>,
    /// KDE's computed active background, tinted from the set's foreground roles.
    #[parts(may_be_absent)]
    pub background_active: Value<Color>,
    /// KDE's computed link background.
    #[parts(may_be_absent)]
    pub background_link: Value<Color>,
    /// KDE's computed visited-link background.
    #[parts(may_be_absent)]
    pub background_visited: Value<Color>,
    /// KDE's computed negative background.
    #[parts(may_be_absent)]
    pub background_negative: Value<Color>,
    /// KDE's computed neutral background.
    #[parts(may_be_absent)]
    pub background_neutral: Value<Color>,
    /// KDE's computed positive background.
    #[parts(may_be_absent)]
    pub background_positive: Value<Color>,
    /// The disabled roles, per element; parts' `{disabled}` rules refer to
    /// them (S3.2).
    #[parts(block = "fields")]
    pub disabled: DisabledRoles,
}

/// A surface set's disabled roles: [`SurfaceRoles`]' colour fields but
/// `disabled`, each of which some preset leaves absent (the catalogue's
/// `DisabledRoles`).
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct DisabledRoles {
    /// The set's fill.
    #[parts(may_be_absent)]
    pub background: Value<Color>,
    /// The alternate row or band fill (KDE's `BackgroundAlternate`).
    #[parts(may_be_absent)]
    pub background_alternate: Value<Color>,
    /// The primary text and glyph colour.
    #[parts(may_be_absent)]
    pub foreground: Value<Color>,
    /// The platform's secondary (muted) text colour.
    #[parts(may_be_absent)]
    pub foreground_inactive: Value<Color>,
    /// KDE's `ForegroundActive`, the information colour.
    #[parts(may_be_absent)]
    pub foreground_active: Value<Color>,
    /// Link text.
    #[parts(may_be_absent)]
    pub link: Value<Color>,
    /// Visited link text.
    #[parts(may_be_absent)]
    pub link_visited: Value<Color>,
    /// The set's negative status text colour (KDE's `ForegroundNegative`).
    #[parts(may_be_absent)]
    pub negative: Value<Color>,
    /// The set's neutral status text colour, a warning (KDE's `ForegroundNeutral`).
    #[parts(may_be_absent)]
    pub neutral: Value<Color>,
    /// The set's positive status text colour (KDE's `ForegroundPositive`).
    #[parts(may_be_absent)]
    pub positive: Value<Color>,
    /// KDE's focus decoration colour of the set.
    #[parts(may_be_absent)]
    pub decoration_focus: Value<Color>,
    /// KDE's hover decoration colour of the set.
    #[parts(may_be_absent)]
    pub decoration_hover: Value<Color>,
    /// The set's translucent shade line or overlay (libadwaita's `*_shade_color`).
    #[parts(may_be_absent)]
    pub shade: Value<Color>,
    /// The set's darker shade (libadwaita's `headerbar_darker_shade_color`).
    #[parts(may_be_absent)]
    pub shade_darker: Value<Color>,
    /// The set's stroke colour, translucent where the platform's is.
    #[parts(may_be_absent)]
    pub border: Value<Color>,
    /// KDE's computed active background, tinted from the set's foreground roles.
    #[parts(may_be_absent)]
    pub background_active: Value<Color>,
    /// KDE's computed link background.
    #[parts(may_be_absent)]
    pub background_link: Value<Color>,
    /// KDE's computed visited-link background.
    #[parts(may_be_absent)]
    pub background_visited: Value<Color>,
    /// KDE's computed negative background.
    #[parts(may_be_absent)]
    pub background_negative: Value<Color>,
    /// KDE's computed neutral background.
    #[parts(may_be_absent)]
    pub background_neutral: Value<Color>,
    /// KDE's computed positive background.
    #[parts(may_be_absent)]
    pub background_positive: Value<Color>,
}

/// One status's colours (the catalogue's `StatusRoles` entry).
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct StatusColors {
    /// The status fill.
    pub fill: Value<Color>,
    /// Text and glyphs on the fill.
    pub foreground: Value<Color>,
    /// The status as a text or icon colour on the surface.
    pub standalone: Value<Color>,
    /// The status background.
    pub background: Value<Color>,
    /// The status's solid background.
    pub background_solid: Value<Color>,
}

/// The status roles (the catalogue's `StatusRoles`): every colour may be
/// absent where the platform has no counterpart, but `error`'s `fill`.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct StatusRoles {
    /// Success.
    #[parts(block = "fields", inner_may_be_absent)]
    pub success: StatusColors,
    /// Warning.
    #[parts(block = "fields", inner_may_be_absent)]
    pub warning: StatusColors,
    /// Error.
    #[parts(block = "fields", inner_may_be_absent, required = "fill")]
    pub error: StatusColors,
    /// Information.
    #[parts(block = "fields", inner_may_be_absent)]
    pub info: StatusColors,
    /// A neutral status.
    #[parts(block = "fields", inner_may_be_absent)]
    pub neutral: StatusColors,
    /// A destructive action, apart from `error`.
    #[parts(block = "fields", inner_may_be_absent)]
    pub destructive: StatusColors,
}

/// The roles a platform states once for the whole theme (S3).
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct GlobalRoles {
    /// The accent fill.
    pub accent: Value<Color>,
    /// Text and glyphs on the accent.
    pub accent_foreground: Value<Color>,
    /// The accent as a text or icon colour on the surface, distinct from the fill
    /// (libadwaita's `accent_color`); elsewhere `= role.global.accent`.
    pub accent_standalone: Value<Color>,
    /// The accent ramp (WinUI's Light1–3, Dark1–3), keyed; empty where the
    /// platform has none.
    pub accent_ramp: Vec<(Key, Value<Color>)>,
    /// The status roles.
    #[parts(block = "fields")]
    pub status: StatusRoles,
    /// What the platform names its shadow colour.
    #[parts(may_be_absent)]
    pub shadow_reference: Value<Color>,
    /// The highlight behind selected text.
    pub selection_text_background: Value<Color>,
    /// The selected text's colour; absent where the text keeps its own.
    #[parts(may_be_absent)]
    pub selection_text_foreground: Value<Color>,
    /// An unfocused field's selection.
    #[parts(may_be_absent)]
    pub selection_text_background_unfocused: Value<Color>,
    /// An unfocused field's selected text (macOS's unemphasized selected text colour).
    #[parts(may_be_absent)]
    pub selection_text_foreground_unfocused: Value<Color>,
    /// The find or search match highlight.
    #[parts(may_be_absent)]
    pub find_highlight: Value<Color>,
    /// The drop-target highlight.
    #[parts(may_be_absent)]
    pub drop_highlight: Value<Color>,
    /// The modal backdrop colour, with its alpha.
    #[parts(may_be_absent)]
    pub scrim: Value<Color>,
}

/// The colour roles (S3): `colors.<scheme>` in TOML, `role.` in expressions.
#[derive(Clone, Debug, PartialEq, ThemeParts)]
#[parts(block)]
#[non_exhaustive]
pub struct ColorRoles {
    /// The declared surface sets' roles, patched set by set.
    #[parts(block = "fields")]
    pub surfaces: SurfaceMap<SurfaceRoles>,
    /// The global roles.
    #[parts(block = "fields")]
    pub global: GlobalRoles,
    /// The keyed roles the preset declares (`role.@<key>`: Material's ladder,
    /// WinUI's tiers).
    pub keyed: Vec<(Key, Value<Color>)>,
}

/// An enum naming a role record's colour fields by their paths, with its
/// table of every variant.
macro_rules! role_paths {
    (
        $(#[$meta:meta])*
        $name:ident { $($(#[doc = $doc:literal])+ $variant:ident = $path:literal,)+ }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum $name {
            $($(#[doc = $doc])+ $variant,)+
        }

        impl $name {
            /// Every role, in its record's field order.
            pub const ALL: &'static [$name] = &[$($name::$variant,)+];

            /// The role's path in its record, as expressions and TOML spell
            /// it (`disabled.foreground`).
            #[must_use]
            pub const fn path(self) -> &'static str {
                match self {
                    $($name::$variant => $path,)+
                }
            }

            /// The role `path` names.
            #[must_use]
            pub(crate) fn from_path(path: &str) -> Option<$name> {
                Self::ALL.iter().copied().find(|role| role.path() == path)
            }
        }
    };
}

role_paths! {
    /// A surface set's colour role: a field of [`SurfaceRoles`], by its path
    /// (`disabled.foreground`).
    SurfaceRole {
        /// `background`.
        Background = "background",
        /// `background_alternate`.
        BackgroundAlternate = "background_alternate",
        /// `foreground`.
        Foreground = "foreground",
        /// `foreground_inactive`.
        ForegroundInactive = "foreground_inactive",
        /// `foreground_active`.
        ForegroundActive = "foreground_active",
        /// `link`.
        Link = "link",
        /// `link_visited`.
        LinkVisited = "link_visited",
        /// `negative`.
        Negative = "negative",
        /// `neutral`.
        Neutral = "neutral",
        /// `positive`.
        Positive = "positive",
        /// `decoration_focus`.
        DecorationFocus = "decoration_focus",
        /// `decoration_hover`.
        DecorationHover = "decoration_hover",
        /// `shade`.
        Shade = "shade",
        /// `shade_darker`.
        ShadeDarker = "shade_darker",
        /// `border`.
        Border = "border",
        /// `background_active`.
        BackgroundActive = "background_active",
        /// `background_link`.
        BackgroundLink = "background_link",
        /// `background_visited`.
        BackgroundVisited = "background_visited",
        /// `background_negative`.
        BackgroundNegative = "background_negative",
        /// `background_neutral`.
        BackgroundNeutral = "background_neutral",
        /// `background_positive`.
        BackgroundPositive = "background_positive",
        /// `disabled.background`.
        DisabledBackground = "disabled.background",
        /// `disabled.background_alternate`.
        DisabledBackgroundAlternate = "disabled.background_alternate",
        /// `disabled.foreground`.
        DisabledForeground = "disabled.foreground",
        /// `disabled.foreground_inactive`.
        DisabledForegroundInactive = "disabled.foreground_inactive",
        /// `disabled.foreground_active`.
        DisabledForegroundActive = "disabled.foreground_active",
        /// `disabled.link`.
        DisabledLink = "disabled.link",
        /// `disabled.link_visited`.
        DisabledLinkVisited = "disabled.link_visited",
        /// `disabled.negative`.
        DisabledNegative = "disabled.negative",
        /// `disabled.neutral`.
        DisabledNeutral = "disabled.neutral",
        /// `disabled.positive`.
        DisabledPositive = "disabled.positive",
        /// `disabled.decoration_focus`.
        DisabledDecorationFocus = "disabled.decoration_focus",
        /// `disabled.decoration_hover`.
        DisabledDecorationHover = "disabled.decoration_hover",
        /// `disabled.shade`.
        DisabledShade = "disabled.shade",
        /// `disabled.shade_darker`.
        DisabledShadeDarker = "disabled.shade_darker",
        /// `disabled.border`.
        DisabledBorder = "disabled.border",
        /// `disabled.background_active`.
        DisabledBackgroundActive = "disabled.background_active",
        /// `disabled.background_link`.
        DisabledBackgroundLink = "disabled.background_link",
        /// `disabled.background_visited`.
        DisabledBackgroundVisited = "disabled.background_visited",
        /// `disabled.background_negative`.
        DisabledBackgroundNegative = "disabled.background_negative",
        /// `disabled.background_neutral`.
        DisabledBackgroundNeutral = "disabled.background_neutral",
        /// `disabled.background_positive`.
        DisabledBackgroundPositive = "disabled.background_positive",
    }
}

role_paths! {
    /// A global colour role: a typed colour field of [`GlobalRoles`], by its
    /// path (`status.error.fill`); an accent-ramp entry is a keyed value,
    /// `role.global.accent_ramp.<key>`, which no variant names.
    GlobalRole {
        /// `accent`.
        Accent = "accent",
        /// `accent_foreground`.
        AccentForeground = "accent_foreground",
        /// `accent_standalone`.
        AccentStandalone = "accent_standalone",
        /// `status.success.fill`.
        SuccessFill = "status.success.fill",
        /// `status.success.foreground`.
        SuccessForeground = "status.success.foreground",
        /// `status.success.standalone`.
        SuccessStandalone = "status.success.standalone",
        /// `status.success.background`.
        SuccessBackground = "status.success.background",
        /// `status.success.background_solid`.
        SuccessBackgroundSolid = "status.success.background_solid",
        /// `status.warning.fill`.
        WarningFill = "status.warning.fill",
        /// `status.warning.foreground`.
        WarningForeground = "status.warning.foreground",
        /// `status.warning.standalone`.
        WarningStandalone = "status.warning.standalone",
        /// `status.warning.background`.
        WarningBackground = "status.warning.background",
        /// `status.warning.background_solid`.
        WarningBackgroundSolid = "status.warning.background_solid",
        /// `status.error.fill`.
        ErrorFill = "status.error.fill",
        /// `status.error.foreground`.
        ErrorForeground = "status.error.foreground",
        /// `status.error.standalone`.
        ErrorStandalone = "status.error.standalone",
        /// `status.error.background`.
        ErrorBackground = "status.error.background",
        /// `status.error.background_solid`.
        ErrorBackgroundSolid = "status.error.background_solid",
        /// `status.info.fill`.
        InfoFill = "status.info.fill",
        /// `status.info.foreground`.
        InfoForeground = "status.info.foreground",
        /// `status.info.standalone`.
        InfoStandalone = "status.info.standalone",
        /// `status.info.background`.
        InfoBackground = "status.info.background",
        /// `status.info.background_solid`.
        InfoBackgroundSolid = "status.info.background_solid",
        /// `status.neutral.fill`.
        NeutralFill = "status.neutral.fill",
        /// `status.neutral.foreground`.
        NeutralForeground = "status.neutral.foreground",
        /// `status.neutral.standalone`.
        NeutralStandalone = "status.neutral.standalone",
        /// `status.neutral.background`.
        NeutralBackground = "status.neutral.background",
        /// `status.neutral.background_solid`.
        NeutralBackgroundSolid = "status.neutral.background_solid",
        /// `status.destructive.fill`.
        DestructiveFill = "status.destructive.fill",
        /// `status.destructive.foreground`.
        DestructiveForeground = "status.destructive.foreground",
        /// `status.destructive.standalone`.
        DestructiveStandalone = "status.destructive.standalone",
        /// `status.destructive.background`.
        DestructiveBackground = "status.destructive.background",
        /// `status.destructive.background_solid`.
        DestructiveBackgroundSolid = "status.destructive.background_solid",
        /// `shadow_reference`.
        ShadowReference = "shadow_reference",
        /// `selection_text_background`.
        SelectionTextBackground = "selection_text_background",
        /// `selection_text_foreground`.
        SelectionTextForeground = "selection_text_foreground",
        /// `selection_text_background_unfocused`.
        SelectionTextBackgroundUnfocused = "selection_text_background_unfocused",
        /// `selection_text_foreground_unfocused`.
        SelectionTextForegroundUnfocused = "selection_text_foreground_unfocused",
        /// `find_highlight`.
        FindHighlight = "find_highlight",
        /// `drop_highlight`.
        DropHighlight = "drop_highlight",
        /// `scrim`.
        Scrim = "scrim",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_set_names_round_trip() {
        assert_eq!(SurfaceSet::ALL.map(SurfaceSet::name), SurfaceSet::NAMES);
        for set in SurfaceSet::ALL {
            assert_eq!(SurfaceSet::from_name(set.name()), Some(set));
        }
        assert_eq!(SurfaceSet::from_name("Window"), None);
        assert_eq!(SurfaceSet::from_name("global"), None);
    }
}
