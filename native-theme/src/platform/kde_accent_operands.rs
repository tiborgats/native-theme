//! `kde_accent`'s and `kde_accent_wm`'s operands (S2.2) as the model holds
//! them, mapped onto the clean room's types: [`KdeAccentInput`],
//! [`AccentSet`], [`AccentKey`], [`AccentGroup`] and [`AccentWmKey`]. The
//! clean room wrote `kde_accent.rs` from its description and vector tables
//! alone (D10), so it names none of the model's types, and it is placed as
//! written; evaluation and a KDE port's conversion (S14) both call it
//! through [`KdeAccentScheme`], the operands passed as plain values.
//!
//! The mapping keeps every call inside the description's domain, where its
//! two open questions do not arise. Each colour operand enters with its
//! channels within `[0, 1]` ([`entered`]: a non-finite channel zero, as
//! S2.2's *Totality* takes it, any other clamped, as a colour outside
//! sRGB's gamut is clamped where it is drawn, S2.1), and the tint factor
//! is finite (a non-finite one zero, the same rule). So the accent is
//! always a standard Qt colour, never an extended or invalid one whose
//! alpha `accentBackground` sets (the first question); and no colour the
//! functions compute from operands in that domain is invalid (the
//! description's edge cases), so none is written as `invalid` and read
//! back (the second).

use super::kde_accent::{
    AccentGroup, AccentKey, AccentSet, AccentWmKey, KdeAccentInput, kde_accent, kde_accent_wm,
};
use crate::color::Color;
use crate::inputs::{InputName, KdeSchemeKey, KdeWmKey};
use crate::roles::SurfaceSet;
use crate::tokens::KdeGroup;

/// What both functions read besides a key's own scheme inputs (S2.2): the
/// accent, the scheme's three lists and its two options, each the model's
/// value for the scheme being resolved.
#[derive(Clone, Copy, Debug)]
pub(crate) struct KdeAccentScheme<'a> {
    /// `input.accent`.
    pub(crate) accent: Color,
    /// `input.kde_scheme.groups`.
    pub(crate) groups: &'a [SurfaceSet],
    /// `input.kde_scheme.inactive_groups`.
    pub(crate) inactive_groups: &'a [SurfaceSet],
    /// `input.kde_scheme.unset_keys`.
    pub(crate) unset_keys: &'a [InputName],
    /// `token.kde.tint_factor`.
    pub(crate) tint_factor: Option<f32>,
    /// `token.kde.titlebar_is_accent_colored`.
    pub(crate) titlebar_is_accent_colored: bool,
}

/// The scheme inputs `kde_accent(set, key, group)` reads (S2.2).
#[derive(Clone, Copy, Debug)]
pub(crate) struct KdeAccentKeyInputs {
    /// `input.kde_scheme.<set>.<key>`.
    pub(crate) value: Color,
    /// `input.kde_scheme.<set>.inactive.<key>`.
    pub(crate) inactive_value: Color,
    /// `input.kde_scheme.<set>.background_normal`.
    pub(crate) background: Color,
    /// `input.kde_scheme.window.<key>`.
    pub(crate) window_value: Color,
    /// `input.kde_scheme.window.background_normal`.
    pub(crate) window_background: Color,
    /// `input.kde_scheme.view.background_normal`.
    pub(crate) view_background: Color,
}

impl KdeAccentScheme<'_> {
    /// `kde_accent(set, key, group)` over `inputs`; `None` for a set
    /// outside KDE's seven and for the group `disabled`, which validation
    /// refuses (S12).
    pub(crate) fn kde_accent(
        &self,
        set: SurfaceSet,
        key: KdeSchemeKey,
        group: KdeGroup,
        inputs: &KdeAccentKeyInputs,
    ) -> Option<Color> {
        let set = accent_set(set)?;
        let group = accent_group(group)?;
        let key = accent_key(key);
        let mut input = self.input();
        input.sets.view.active.background_normal = entered(inputs.view_background);
        input.sets.window.active.background_normal = entered(inputs.window_background);
        *input.sets.window.active.get_mut(key) = entered(inputs.window_value);
        let own = input.sets.get_mut(set);
        own.active.background_normal = entered(inputs.background);
        *own.active.get_mut(key) = entered(inputs.value);
        *own.inactive.get_mut(key) = entered(inputs.inactive_value);
        Some(from_channels(kde_accent(&input, set, key, group)))
    }

    /// `kde_accent_wm(key)` over `value`, `input.kde_scheme.wm.<key>`, and
    /// `window_background`, `input.kde_scheme.window.background_normal`.
    pub(crate) fn kde_accent_wm(
        &self,
        key: KdeWmKey,
        value: Color,
        window_background: Color,
    ) -> Color {
        let key = accent_wm_key(key);
        let mut input = self.input();
        let wm = &mut input.wm;
        let slot = match key {
            AccentWmKey::ActiveBackground => &mut wm.active_background,
            AccentWmKey::ActiveForeground => &mut wm.active_foreground,
            AccentWmKey::InactiveBackground => &mut wm.inactive_background,
            AccentWmKey::InactiveForeground => &mut wm.inactive_foreground,
        };
        *slot = entered(value);
        input.sets.window.active.background_normal = entered(window_background);
        from_channels(kde_accent_wm(&input, key))
    }

    /// The clean room's input with the accent, the options and the lists;
    /// each scheme input transparent until a call sets those it reads.
    fn input(&self) -> KdeAccentInput {
        let mut input = KdeAccentInput {
            accent: entered(self.accent),
            tint_factor: self
                .tint_factor
                .map(|factor| if factor.is_finite() { factor } else { 0.0 }),
            titlebar_is_accent_colored: self.titlebar_is_accent_colored,
            ..KdeAccentInput::default()
        };
        for set in self.groups.iter().filter_map(|set| accent_set(*set)) {
            input.sets.get_mut(set).has_group = true;
        }
        for set in self
            .inactive_groups
            .iter()
            .filter_map(|set| accent_set(*set))
        {
            input.sets.get_mut(set).has_inactive_group = true;
        }
        for (set, key, inactive) in self.unset_keys.iter().filter_map(unset_key) {
            let own = input.sets.get_mut(set);
            let unset = if inactive {
                &mut own.inactive_unset
            } else {
                &mut own.active_unset
            };
            *unset.get_mut(key) = true;
        }
        input
    }
}

/// A colour operand as it enters: its channels within `[0, 1]`, a
/// non-finite one zero.
fn entered(c: Color) -> [f32; 4] {
    [c.r, c.g, c.b, c.a].map(|x| {
        if x.is_finite() {
            x.clamp(0.0, 1.0)
        } else {
            0.0
        }
    })
}

/// The colour of the clean room's channels.
fn from_channels([r, g, b, a]: [f32; 4]) -> Color {
    Color { r, g, b, a }
}

/// The clean room's set for one of KDE's seven; `None` for another set.
fn accent_set(set: SurfaceSet) -> Option<AccentSet> {
    match set {
        SurfaceSet::Window => Some(AccentSet::Window),
        SurfaceSet::View => Some(AccentSet::View),
        SurfaceSet::Button => Some(AccentSet::Button),
        SurfaceSet::Selection => Some(AccentSet::Selection),
        SurfaceSet::Tooltip => Some(AccentSet::Tooltip),
        SurfaceSet::Complementary => Some(AccentSet::Complementary),
        SurfaceSet::Header => Some(AccentSet::Header),
        SurfaceSet::Sidebar
        | SurfaceSet::SecondarySidebar
        | SurfaceSet::Card
        | SurfaceSet::Popover
        | SurfaceSet::Dialog
        | SurfaceSet::Osd => None,
    }
}

/// The clean room's key.
fn accent_key(key: KdeSchemeKey) -> AccentKey {
    match key {
        KdeSchemeKey::BackgroundNormal => AccentKey::BackgroundNormal,
        KdeSchemeKey::BackgroundAlternate => AccentKey::BackgroundAlternate,
        KdeSchemeKey::ForegroundNormal => AccentKey::ForegroundNormal,
        KdeSchemeKey::ForegroundInactive => AccentKey::ForegroundInactive,
        KdeSchemeKey::ForegroundActive => AccentKey::ForegroundActive,
        KdeSchemeKey::ForegroundLink => AccentKey::ForegroundLink,
        KdeSchemeKey::ForegroundVisited => AccentKey::ForegroundVisited,
        KdeSchemeKey::ForegroundNegative => AccentKey::ForegroundNegative,
        KdeSchemeKey::ForegroundNeutral => AccentKey::ForegroundNeutral,
        KdeSchemeKey::ForegroundPositive => AccentKey::ForegroundPositive,
        KdeSchemeKey::DecorationFocus => AccentKey::DecorationFocus,
        KdeSchemeKey::DecorationHover => AccentKey::DecorationHover,
    }
}

/// The clean room's group; `None` for `disabled`, KColorScheme's effects
/// over the active values (R5.1), no accent derivation.
fn accent_group(group: KdeGroup) -> Option<AccentGroup> {
    match group {
        KdeGroup::Active => Some(AccentGroup::Active),
        KdeGroup::Inactive => Some(AccentGroup::Inactive),
        KdeGroup::Disabled => None,
    }
}

/// The clean room's `[WM]` key.
fn accent_wm_key(key: KdeWmKey) -> AccentWmKey {
    match key {
        KdeWmKey::ActiveBackground => AccentWmKey::ActiveBackground,
        KdeWmKey::ActiveForeground => AccentWmKey::ActiveForeground,
        KdeWmKey::InactiveBackground => AccentWmKey::InactiveBackground,
        KdeWmKey::InactiveForeground => AccentWmKey::InactiveForeground,
    }
}

/// The set and key an `unset_keys` entry names, and whether it is the
/// `[Inactive]` group's: `kde_scheme.<set>.<key>` or
/// `kde_scheme.<set>.inactive.<key>`; `None` for another name, which
/// validation refuses (S9.1, S12).
fn unset_key(name: &InputName) -> Option<(AccentSet, AccentKey, bool)> {
    let (set, key, inactive) = match name.segments() {
        [scheme, set, key] if scheme.as_str() == "kde_scheme" => (set, key, false),
        [scheme, set, group, key]
            if scheme.as_str() == "kde_scheme" && group.as_str() == "inactive" =>
        {
            (set, key, true)
        }
        _ => return None,
    };
    let set = SurfaceSet::from_name(set.as_str()).and_then(accent_set)?;
    let key = KdeSchemeKey::ALL
        .into_iter()
        .find(|held| held.name() == key.as_str())
        .map(accent_key)?;
    Some((set, key, inactive))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::{KdeAccentKeyInputs, KdeAccentScheme};
    use crate::color::Color;
    use crate::inputs::{InputName, KdeSchemeKey, KdeWmKey};
    use crate::roles::SurfaceSet;
    use crate::tokens::KdeGroup;

    /// A table's rows, each its cells by column name.
    fn rows(table: &str) -> Vec<Vec<(String, String)>> {
        let mut lines = table.lines().filter(|line| !line.starts_with('#'));
        let columns: Vec<&str> = lines.next().unwrap().split('\t').collect();
        lines
            .map(|line| {
                columns
                    .iter()
                    .zip(line.split('\t'))
                    .map(|(column, cell)| ((*column).to_owned(), cell.to_owned()))
                    .collect()
            })
            .collect()
    }

    fn cell<'r>(row: &'r [(String, String)], column: &str) -> &'r str {
        row.iter()
            .find(|(held, _)| held == column)
            .map(|(_, cell)| cell.as_str())
            .unwrap_or_else(|| panic!("no column {column}"))
    }

    /// A colour cell: a literal of S2.1, or `srgb(r, g, b, a)` with an
    /// exponent the parser does not take.
    fn color(text: &str) -> Color {
        Color::parse(text).unwrap_or_else(|| {
            let channels: Vec<f32> = text
                .strip_prefix("srgb(")
                .and_then(|rest| rest.strip_suffix(')'))
                .unwrap_or_else(|| panic!("no colour {text}"))
                .split(',')
                .map(|n| n.trim().parse().unwrap())
                .collect();
            let [r, g, b, a] = channels.try_into().unwrap();
            Color { r, g, b, a }
        })
    }

    /// A list cell's names, a JSON string of comma-separated names.
    fn names(text: &str) -> Vec<String> {
        let inner = text.strip_prefix('"').and_then(|t| t.strip_suffix('"'));
        match inner.unwrap() {
            "" => Vec::new(),
            inner => inner.split(',').map(str::to_owned).collect(),
        }
    }

    fn sets(text: &str) -> Vec<SurfaceSet> {
        names(text)
            .iter()
            .map(|name| SurfaceSet::from_name(name).unwrap())
            .collect()
    }

    fn unset(text: &str) -> Vec<InputName> {
        names(text)
            .iter()
            .map(|name| InputName::new(&format!("kde_scheme.{name}")).unwrap())
            .collect()
    }

    fn scheme_key(name: &str) -> KdeSchemeKey {
        KdeSchemeKey::ALL
            .into_iter()
            .find(|key| key.name() == name)
            .unwrap()
    }

    /// The tables' results within 1e-3 on the 0-255 scale, every channel.
    fn close(got: Color, want: Color) -> bool {
        [
            (got.r, want.r),
            (got.g, want.g),
            (got.b, want.b),
            (got.a, want.a),
        ]
        .iter()
        .all(|(g, w)| ((f64::from(*g) - f64::from(*w)) * 255.0).abs() <= 1e-3)
    }

    /// Every row of `kde_accent.tsv` through the model's operands.
    #[test]
    fn model_operands_give_every_accent_row() {
        let table = include_str!("../../tests/vectors/kde_accent.tsv");
        let mut failures = Vec::new();
        for row in rows(table) {
            let c = |column: &str| color(cell(&row, column));
            let (groups, inactive_groups) = (
                sets(cell(&row, "groups")),
                sets(cell(&row, "inactive_groups")),
            );
            let unset_keys = unset(cell(&row, "unset_keys"));
            let scheme = KdeAccentScheme {
                accent: c("accent"),
                groups: &groups,
                inactive_groups: &inactive_groups,
                unset_keys: &unset_keys,
                tint_factor: cell(&row, "tint_factor").parse().ok(),
                titlebar_is_accent_colored: cell(&row, "titlebar_is_accent_colored") == "true",
            };
            let inputs = KdeAccentKeyInputs {
                value: c("value"),
                inactive_value: c("inactive_value"),
                background: c("background"),
                window_value: c("window_value"),
                window_background: c("window_background"),
                view_background: c("view_background"),
            };
            let group = match cell(&row, "group") {
                "active" => KdeGroup::Active,
                "inactive" => KdeGroup::Inactive,
                other => panic!("no group {other}"),
            };
            let got = scheme
                .kde_accent(
                    SurfaceSet::from_name(cell(&row, "set")).unwrap(),
                    scheme_key(cell(&row, "key")),
                    group,
                    &inputs,
                )
                .unwrap();
            if !close(got, c("result")) {
                failures.push(format!("{row:?}: {got:?}"));
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    /// Every row of `kde_accent_wm.tsv` through the model's operands.
    #[test]
    fn model_operands_give_every_wm_row() {
        let table = include_str!("../../tests/vectors/kde_accent_wm.tsv");
        let mut failures = Vec::new();
        for row in rows(table) {
            let c = |column: &str| color(cell(&row, column));
            let groups = sets(cell(&row, "groups"));
            let unset_keys = unset(cell(&row, "unset_keys"));
            let scheme = KdeAccentScheme {
                accent: c("accent"),
                groups: &groups,
                inactive_groups: &[],
                unset_keys: &unset_keys,
                tint_factor: cell(&row, "tint_factor").parse().ok(),
                titlebar_is_accent_colored: cell(&row, "titlebar_is_accent_colored") == "true",
            };
            let key = KdeWmKey::ALL
                .into_iter()
                .find(|key| key.name() == cell(&row, "key"))
                .unwrap();
            let got = scheme.kde_accent_wm(key, c("value"), c("window_background"));
            if !close(got, c("result")) {
                failures.push(format!("{row:?}: {got:?}"));
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    /// An operand outside the clean room's domain enters inside it: an
    /// accent outside sRGB's gamut as the clamped one, a non-finite channel
    /// as zero, a non-finite tint factor as zero (the module's doc).
    #[test]
    fn operands_enter_the_domain() {
        let scheme = |accent: Color, tint_factor: Option<f32>| KdeAccentScheme {
            accent,
            groups: &[SurfaceSet::Window, SurfaceSet::View],
            inactive_groups: &[],
            unset_keys: &[],
            tint_factor,
            titlebar_is_accent_colored: false,
        };
        // Synthetic operands.
        let grey = color("#808080");
        let inputs = KdeAccentKeyInputs {
            value: grey,
            inactive_value: grey,
            background: grey,
            window_value: grey,
            window_background: grey,
            view_background: grey,
        };
        let link = |scheme: KdeAccentScheme<'_>| {
            scheme
                .kde_accent(
                    SurfaceSet::Selection,
                    KdeSchemeKey::BackgroundNormal,
                    KdeGroup::Active,
                    &inputs,
                )
                .unwrap()
        };
        let wide = Color::parse("display_p3(1, 0, 0)").unwrap();
        let clamped = Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };
        assert_eq!(link(scheme(wide, None)), link(scheme(clamped, None)));
        let nan = Color {
            r: f32::NAN,
            ..clamped
        };
        let zero = Color { r: 0.0, ..clamped };
        assert_eq!(link(scheme(nan, None)), link(scheme(zero, None)));
        let tinted = |factor| {
            scheme(clamped, Some(factor))
                .kde_accent(
                    SurfaceSet::Window,
                    KdeSchemeKey::ForegroundNormal,
                    KdeGroup::Active,
                    &inputs,
                )
                .unwrap()
        };
        assert_eq!(tinted(f32::NAN), tinted(0.0));
        assert_eq!(tinted(f32::INFINITY), tinted(0.0));
    }

    /// A set outside KDE's seven and the group `disabled` give no colour.
    #[test]
    fn no_colour_outside_the_calls_validation_takes() {
        let grey = color("#808080");
        let inputs = KdeAccentKeyInputs {
            value: grey,
            inactive_value: grey,
            background: grey,
            window_value: grey,
            window_background: grey,
            view_background: grey,
        };
        let scheme = KdeAccentScheme {
            accent: grey,
            groups: &[],
            inactive_groups: &[],
            unset_keys: &[],
            tint_factor: None,
            titlebar_is_accent_colored: false,
        };
        let key = KdeSchemeKey::BackgroundNormal;
        assert_eq!(
            scheme.kde_accent(SurfaceSet::Osd, key, KdeGroup::Active, &inputs),
            None
        );
        assert_eq!(
            scheme.kde_accent(SurfaceSet::Window, key, KdeGroup::Disabled, &inputs),
            None
        );
    }
}
