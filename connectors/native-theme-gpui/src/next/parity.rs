//! Connector parity of the 0.8.0 work (docs/v0.8.0-plan.md, section 1, rule
//! 2; M0, step 4, and M4, step 1): what the connector hands to gpui, computed
//! from the 0.7.2 model by the connector's public mapping and from the 0.8.0
//! model by [`super`], is equal leaf by leaf: every field of the
//! gpui-component theme and of its stored configs, the scroll bar and the
//! resize handles, and every public builder of `src/geometry.rs` and
//! `src/variants.rs`.
//!
//! Presets covered: kde-breeze, adwaita, windows-11, macos-monterey,
//! macos-tahoe and material-3, light and dark ([`PRESETS`]). The 0.7.2 side
//! is resolved as the connector's tests resolve it
//! (`ResolutionContext::for_tests`), on the macOS presets at macOS's font DPI
//! ([`old_context`]), with no accessibility scaling; the
//! 0.8.0 side for the scheme's default context, whose text scale is 1, and,
//! for `geometry::title_bar_inactive`, for that context with an inactive
//! window as well.
//!
//! The form of comparison per toolkit type:
//!
//! - `gpui_component::Theme` has no `PartialEq`, and its `Serialize` skips
//!   `notification`, `motion` and `cursor_blink`; it is compared field by
//!   field, each by the `PartialEq` of its own type where it has one: the
//!   `ThemeColor` fields (`Hsla`, equal when its four `f32` are, by
//!   `total_cmp`) and the `ThemeToken` gpui-component derives from each;
//!   `mode` (`ThemeMode`); the families (`SharedString`); the sizes and radii
//!   (`Pixels`); `shadow` and `focus_ring` (`bool`); `scrollbar_mode`
//!   (`ScrollbarMode`); `cursor_blink` (`CursorBlink`); `highlight_theme`
//!   (`HighlightTheme`); `transparent` (`Hsla`); and the `ThemeConfig` in
//!   the slot of the theme's mode, by its fields (`bool`, `ThemeMode`,
//!   `SharedString`, `Option` of `SharedString`, `f32`, `usize`, `bool` and
//!   `HighlightThemeStyle`, its colours as hex strings). `notification`,
//!   `list`, `sheet` and `motion`, and the config in the other slot, have no
//!   `PartialEq` and are compared in their `Debug` form; so is the whole
//!   theme last, where no field differed, for what no field compared names.
//!   A difference is printed in the `Debug` form of both values.
//! - The scroll bar and the resize handles, which go into gpui-base's global,
//!   a type with neither `Debug` nor `PartialEq`: the scroll bar through the
//!   connector's `ScrollbarGeometry`, field by field (`Pixels`, `Hsla`), and
//!   the handles by `ResizableTheme`'s two fields (`Option<Hsla>`). The rest
//!   of that global is what gpui-component projects from its own theme.
//! - The `geometry` builders: `StyleRefinement`, `gpui_component::Size`,
//!   `Option<Pixels>`, `Pixels` and `Option<DialogEntrance>`, each by its
//!   `PartialEq`; `input_fill` and `combo_box_fill` at rest and disabled, as
//!   one pair.
//! - The `variants` builders, whose `ButtonCustomVariant` has neither
//!   `Debug` nor `PartialEq` and needs an `App`: through the connector's own
//!   values that feed them, each by its `PartialEq` — `bordered_button` by
//!   `variants::hover_border` (`Option<Hsla>`), `default_button` by
//!   `variants::default_button_states` (`Option<DefaultButtonStates>`),
//!   `toolbar_button` by `variants::toolbar_button_states`
//!   (`ToolbarButtonStates`) and `ghost_button`, which reads only the theme,
//!   by the three `ThemeColor` fields it takes; the `ThemeColor` fields the
//!   others take beside those are compared as colour leaves.
//!   `switch_color` returns an `Hsla`, compared as such.
//!
//! A difference is allowed only as an entry of [`DIFFERENCES`], with its
//! reason, and is compared in the form its entry gives; an entry that no
//! longer differs fails. A leaf that compares only the connector's fallback
//! on a preset, the preset stating none of the field it reads, is named in
//! [`FALLBACK_ONLY`], which fails once the preset states the field. The list
//! of colour leaves ([`SLOTS`]), all 140 `ThemeColor` fields, is held against
//! the contract tables: every slot they name is in it, or in [`EXCLUDED`]
//! with a reason. The builders compared are held against the public
//! functions of `src/geometry.rs` and `src/variants.rs`: each is compared.

use std::fmt::Debug;

use gpui::{Hsla, Length, SharedString, px};
use gpui_component::theme::{Theme as GpuiTheme, ThemeConfig, ThemeToken};
use native_theme::next::{Preset, ThemeContext};
use native_theme::theme::LayoutTheme;

use crate::{AccessibilityPreferences, ColorMode, Native, ResolvedTheme};

/// The presets the test covers: the 0.7.2 key and the 0.8.0 preset.
const PRESETS: &[(&str, Preset)] = &[
    ("kde-breeze", Preset::KdeBreeze),
    ("adwaita", Preset::Adwaita),
    ("windows-11", Preset::Windows11),
    ("macos-monterey", Preset::MacosMonterey),
    ("macos-tahoe", Preset::MacosTahoe),
    ("material-3", Preset::Material3),
];

/// One `ThemeColor` field: its colour, the token
/// gpui-component derives from it, and its hex string in the stored config,
/// where `ThemeConfigColors` has the field public.
struct Slot {
    name: &'static str,
    color: fn(&GpuiTheme) -> Hsla,
    token: fn(&GpuiTheme) -> ThemeToken,
    config: Option<fn(&ThemeConfig) -> Option<SharedString>>,
}

macro_rules! slot {
    ($slot:ident) => {
        Slot {
            name: stringify!($slot),
            color: |t| t.colors.$slot,
            token: |t| t.tokens.$slot,
            config: Some(|c| c.colors.$slot.clone()),
        }
    };
    // `ThemeConfigColors` keeps the 12 base-palette fields private
    // (`config::theme_color_to_config_colors`).
    ($slot:ident, base) => {
        Slot {
            name: stringify!($slot),
            color: |t| t.colors.$slot,
            token: |t| t.tokens.$slot,
            config: None,
        }
    };
}

/// The `ThemeColor` fields the connector writes (`colors::theme_color_of`):
/// the slice's (`colors::assign_slice`) first, then the others.
const SLOTS: &[Slot] = &[
    slot!(foreground),
    slot!(border),
    slot!(muted_foreground),
    slot!(ring),
    slot!(primary),
    slot!(primary_foreground),
    slot!(primary_active),
    slot!(sidebar_primary),
    slot!(sidebar_primary_foreground),
    slot!(secondary),
    slot!(secondary_foreground),
    slot!(secondary_hover),
    slot!(secondary_active),
    slot!(danger),
    slot!(danger_foreground),
    slot!(danger_active),
    slot!(success),
    slot!(success_foreground),
    slot!(success_active),
    slot!(warning),
    slot!(warning_foreground),
    slot!(warning_active),
    slot!(info),
    slot!(info_foreground),
    slot!(info_active),
    slot!(chart_bullish),
    slot!(chart_bearish),
    slot!(button),
    slot!(button_hover),
    slot!(button_active),
    slot!(button_foreground),
    slot!(button_secondary),
    slot!(button_secondary_hover),
    slot!(button_secondary_active),
    slot!(button_secondary_foreground),
    slot!(button_primary),
    slot!(button_primary_active),
    slot!(button_primary_foreground),
    slot!(button_danger),
    slot!(button_danger_active),
    slot!(button_danger_foreground),
    slot!(button_info),
    slot!(button_info_active),
    slot!(button_info_foreground),
    slot!(button_success),
    slot!(button_success_active),
    slot!(button_success_foreground),
    slot!(button_warning),
    slot!(button_warning_active),
    slot!(button_warning_foreground),
    slot!(list),
    slot!(list_hover),
    slot!(list_active),
    slot!(list_even),
    slot!(list_head),
    slot!(list_highlight_foreground),
    slot!(table),
    slot!(table_hover),
    slot!(table_active),
    slot!(table_even),
    slot!(table_head),
    slot!(table_head_foreground),
    slot!(table_row_border),
    slot!(table_foot_foreground),
    slot!(chart_1),
    slot!(chart_2),
    slot!(chart_3),
    slot!(chart_4),
    slot!(chart_5),
    slot!(group_box_foreground),
    slot!(description_list_label_foreground),
    slot!(overlay),
    slot!(scrollbar),
    slot!(scrollbar_thumb),
    slot!(scrollbar_thumb_hover),
    slot!(skeleton),
    slot!(drag_border),
    slot!(drop_target),
    slot!(red, base),
    slot!(green, base),
    slot!(blue, base),
    slot!(yellow, base),
    slot!(magenta, base),
    slot!(cyan, base),
    slot!(background),
    slot!(accent),
    slot!(accent_foreground),
    slot!(input),
    slot!(selection),
    slot!(link),
    slot!(link_hover),
    slot!(link_active),
    slot!(chart_grid),
    slot!(tab),
    slot!(tab_active),
    slot!(tab_active_foreground),
    slot!(tab_bar),
    slot!(tab_bar_segmented),
    slot!(tab_foreground),
    slot!(sidebar),
    slot!(sidebar_foreground),
    slot!(sidebar_accent),
    slot!(sidebar_accent_foreground),
    slot!(sidebar_border),
    slot!(title_bar),
    slot!(title_bar_border),
    slot!(window_border),
    slot!(popover),
    slot!(popover_foreground),
    slot!(accordion),
    slot!(slider_bar),
    slot!(slider_thumb),
    slot!(switch),
    slot!(switch_thumb),
    slot!(progress_bar),
    slot!(caret),
    slot!(status_bar),
    slot!(status_bar_border),
    slot!(muted),
    slot!(primary_hover),
    slot!(danger_hover),
    slot!(success_hover),
    slot!(warning_hover),
    slot!(info_hover),
    slot!(button_primary_hover),
    slot!(button_danger_hover),
    slot!(button_info_hover),
    slot!(button_success_hover),
    slot!(button_warning_hover),
    slot!(table_foot),
    slot!(list_active_border),
    slot!(table_active_border),
    slot!(group_box),
    slot!(description_list_label),
    slot!(red_light, base),
    slot!(green_light, base),
    slot!(blue_light, base),
    slot!(yellow_light, base),
    slot!(magenta_light, base),
    slot!(cyan_light, base),
];

/// The contract tables' slots the test does not assert, each with the
/// reason. None so far.
const EXCLUDED: &[(&str, &str)] = &[];

/// A listed difference (plan, section 1, rule 2): the leaf, the presets on
/// which it differs, and why.
struct Difference {
    leaf: &'static str,
    presets: &'static [&'static str],
    reason: &'static str,
}

/// The button height 0.7.2's kde-breeze states, `button.min_height` in
/// `native-theme/src/presets/kde-breeze.toml`, which [`DIFFERENCES`]' entry
/// names.
const KDE_BREEZE_BUTTON_HEIGHT_PX: f32 = 32.0;

/// The padding 0.8.0's kde-breeze gives a list row's content beyond 0.7.2's
/// (`list.row(NONE).padding`, issue #4 L4: Breeze's `ItemView_ItemPaddingWidth`
/// and `ItemView_ItemPaddingHeight` 2 with Qt's text margin `PM_FocusFrameHMargin`
/// 2 + 1 at the sides; Breeze `kstyle/breezemetrics.h` and Qt `qcommonstyle.cpp`
/// v6.11.0), as `[top, right, bottom, left]` in px; the connector adds it to the
/// highlight's margins, which 0.7.2 already applied.
const KDE_BREEZE_ROW_PADDING_PX: [f32; 4] = [3.0, 7.0, 3.0, 7.0];

/// The listed differences.
const DIFFERENCES: &[Difference] = &[
    Difference {
        leaf: "geometry::list_item",
        presets: &["kde-breeze"],
        reason: "0.7.2 pads a list row only by Breeze's highlight margins (list.border.padding \
                 2 / 1); 0.8.0's kde-breeze also states the row's own content padding (issue \
                 #4 L4: 7 px at the sides and 3 px above and below, from Breeze's and Qt's \
                 constants), so the refinement's padding is the margins plus that padding; \
                 every other field of the refinement is equal",
    },
    Difference {
        leaf: "geometry::button",
        presets: &["kde-breeze"],
        reason: "0.7.2 states button.min_height 32, the height of a button at the default \
             font rather than a minimum (issue #4 Z1, \
             docs/issues/0004_kde-leftovers-rationale.md, sizes that are not minimums); \
             the 0.8.0 kde-breeze leaves button.body(NONE).min_size[1] unstated, so the \
             0.8.0 refinement sets no height (`size.height`, which 0.7.2 sets to 32 px \
             at a text scale of 1) and the button keeps gpui-component's own; every \
             other field of the refinement is equal",
    },
    Difference {
        leaf: "geometry::dialog",
        presets: &["kde-breeze"],
        reason: "0.7.2 states dialog.min_width, min_height and max_height, which have no \
                 source (issue #4 Z2, docs/issues/0004_kde-leftovers-rationale.md §3, sizes \
                 that are not minimums or have no source; docs/v0.8.0-spec.md §5, notes to \
                 the table); the 0.8.0 kde-breeze leaves dialog.panel's min_size and \
                 max_size[1] unstated, so the 0.8.0 refinement sets no `min_size` and no \
                 `max_size.height` and the dialog keeps gpui-component's own; every other \
                 field of the refinement is equal",
    },
    Difference {
        leaf: "geometry::dialog_max_width",
        presets: &["kde-breeze"],
        reason: "0.7.2 states dialog.max_width, which has no source (issue #4 Z2, as \
                 geometry::dialog's entry); the 0.8.0 kde-breeze leaves \
                 dialog.panel.max_size[0] unstated, so the 0.8.0 side gives no width (`None`) \
                 and the dialog keeps gpui-component's own (docs/v0.8.0-spec.md §10, rule 0)",
    },
    Difference {
        leaf: ROW_HOVER,
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "0.7.2 fills an unstated list.hover_background with the list's own background \
                 (native-theme/src/resolve/inheritance.rs, widget-to-widget: no tint), and the \
                 connector paints a hovered list and table row with it; the 0.8.0 macOS \
                 presets leave list.highlight(HOVER).fill unstated (native-theme/tests/\
                 next_homes.toml, row list.hover_background, unstated_if_unset; \
                 docs/platform-facts.md §2.15 gives no hover colour for a table row), so the \
                 0.8.0 theme keeps gpui-component's own hovered row, its accent at 0.6 opacity \
                 (gpui-component 0.10.0 src/theme/schema.rs, ThemeColor::apply_config), in \
                 list_hover and table_hover, their tokens and their stored config strings \
                 (docs/v0.8.0-spec.md §10, rule 0); every other leaf of the theme is equal",
    },
    Difference {
        leaf: "geometry::group_box_content",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "0.7.2's macOS presets state no card.border.color, line_width or corner_radius \
                 (docs/platform-facts.md §2.26: NSBox's fill and border are not published), and \
                 0.7.2's validation fills them with transparent, 0 and 0 \
                 (native-theme/src/resolve/validate_helpers.rs), values no source gives; the \
                 0.8.0 macOS presets leave card.container(NONE).stroke and its corners unstated \
                 (native-theme/tests/next_homes.toml, rows card.border.color, \
                 card.border.line_width and card.border.corner_radius, unstated_if_unset), so the \
                 0.8.0 refinement sets no border width, no border colour and no corner radius and \
                 the group box keeps gpui-component's own (docs/v0.8.0-spec.md §10, rule 0); \
                 every other field of the refinement is equal",
    },
    Difference {
        leaf: "geometry::group_box_content",
        presets: &["adwaita", "material-3"],
        reason: "0.7.2's adwaita.toml and material-3.toml state no card.border.color or \
                 line_width, and 0.7.2's validation fills a card border's unstated fields with \
                 transparent and 0 (`BorderKind::None`, \
                 native-theme/src/resolve/validate_helpers.rs), a value no source gives; the \
                 0.8.0 adwaita and material-3 leave card.container(NONE).stroke \
                 unstated (native-theme/tests/next_homes.toml, rows card.border.color and \
                 card.border.line_width, unstated_if_unset), so the 0.8.0 refinement sets no \
                 border width and no border colour and the group box keeps gpui-component's \
                 own (docs/v0.8.0-spec.md §10, rule 0); every other field of the refinement \
                 is equal",
    },
];

/// The presets of [`DIFFERENCES`]' `geometry::group_box_content` entries
/// whose card leaves its corners unstated beside its stroke.
const UNSTATED_CARD_CORNERS: &[&str] = &["macos-monterey", "macos-tahoe"];

/// The leaf of [`DIFFERENCES`]' entry for a hovered list or table row: the
/// `ThemeColor` slots `list_hover` and `table_hover`, their tokens and their
/// stored config strings, compared by [`row_hover_as_list`].
const ROW_HOVER: &str = "to_theme: list_hover and table_hover";

/// `theme` with its hovered list and table row as 0.7.2 paints them where
/// the preset states no hover, the list's own fill (colour, token and the
/// stored config's string of the theme's mode), where `theme` holds
/// gpui-component's own hovered row there, its accent at
/// [`super::UPSTREAM_LIST_HOVER_ACCENT_OPACITY`] ([`DIFFERENCES`]'
/// [`ROW_HOVER`] entry); `None` where it does not. The colours are compared
/// by their hex strings, which an upstream rebuild of the theme from its
/// stored config (`Theme::change`) round-trips them through.
fn row_hover_as_list(theme: &GpuiTheme) -> Option<GpuiTheme> {
    let hex = |c: Hsla| Some(SharedString::from(crate::colors::hsla_to_hex(c)));
    let own = hex(theme
        .colors
        .accent
        .opacity(super::UPSTREAM_LIST_HOVER_ACCENT_OPACITY));
    let config = if theme.is_dark() {
        &theme.dark_theme
    } else {
        &theme.light_theme
    };
    let (c, t, s) = (&theme.colors, &theme.tokens, &config.colors);
    let upstream = hex(c.list_hover) == own
        && hex(c.table_hover) == own
        && t.list_hover == ThemeToken::from(c.list_hover)
        && t.table_hover == ThemeToken::from(c.table_hover)
        && s.list_hover == own
        && s.table_hover == own;
    if !upstream {
        return None;
    }
    let list = theme.colors.list;
    let mut patched = theme.clone();
    patched.colors.list_hover = list;
    patched.colors.table_hover = list;
    patched.tokens.list_hover = ThemeToken::from(list);
    patched.tokens.table_hover = ThemeToken::from(list);
    let mut stored = (**config).clone();
    stored.colors.list_hover = hex(list);
    stored.colors.table_hover = hex(list);
    let other = if theme.is_dark() {
        patched.dark_theme = std::rc::Rc::new(stored);
        &mut patched.light_theme
    } else {
        patched.light_theme = std::rc::Rc::new(stored);
        &mut patched.dark_theme
    };
    // The config of the other scheme, stored by an earlier `apply`: its
    // table row hovers as its list row does, both given the list's fill.
    if other.colors.list_hover != other.colors.table_hover {
        return None;
    }
    let mut stored = (**other).clone();
    stored.colors.list_hover = stored.colors.list.clone();
    stored.colors.table_hover = stored.colors.list.clone();
    *other = std::rc::Rc::new(stored);
    Some(patched)
}

/// A leaf that, on the presets named, compares only the connector's own
/// fallback: the 0.7.2 field it reads is unset there, and the 0.8.0 home
/// unstated. The test asserts the field unset on each, so an entry fails, and
/// is to be removed, once such a preset states it.
struct FallbackOnly {
    leaf: &'static str,
    field: &'static str,
    presets: &'static [&'static str],
    unset: fn(&ResolvedTheme) -> bool,
}

/// The leaves compared on their fallback only.
const FALLBACK_ONLY: &[FallbackOnly] = &[
    FallbackOnly {
        leaf: "geometry::data_table_size",
        field: "list.row_height",
        presets: &["kde-breeze", "adwaita"],
        unset: |t| t.list.row_height.is_none(),
    },
    FallbackOnly {
        leaf: "geometry::list_header_height",
        field: "list.header_height",
        presets: &["kde-breeze", "windows-11"],
        unset: |t| t.list.header_height.is_none(),
    },
];

/// The leaves compared and the differences found, for one preset and scheme.
struct Report {
    label: String,
    preset: &'static str,
    compared: usize,
    /// The leaves compared or found unmapped, by name.
    seen: Vec<String>,
    failures: Vec<String>,
}

impl Report {
    fn new(preset: &'static str, is_dark: bool) -> Self {
        let scheme = if is_dark { "dark" } else { "light" };
        Self {
            label: format!("{preset}/{scheme}"),
            preset,
            compared: 0,
            seen: Vec::new(),
            failures: Vec::new(),
        }
    }

    /// Count `leaf` as compared.
    fn count(&mut self, leaf: &str) {
        self.compared += 1;
        self.seen.push(leaf.to_owned());
    }

    /// Compare one leaf; a listed difference fails here, being compared in
    /// its own form by the caller.
    fn leaf<T: PartialEq + Debug>(&mut self, leaf: &str, old: T, new: T) {
        self.count(leaf);
        if old != new {
            self.failures.push(format!(
                "{}: {leaf}: 0.7.2 {old:?}, 0.8.0 {new:?}",
                self.label
            ));
        }
    }

    /// A leaf of the 0.8.0 side that is unmapped is a failure too.
    fn mapped<T>(&mut self, leaf: &str, new: super::Mapped<T>) -> Option<T> {
        match new {
            Ok(v) => Some(v),
            Err(super::Unmapped(home)) => {
                self.seen.push(leaf.to_owned());
                self.failures
                    .push(format!("{}: {leaf}: 0.8.0 lacks {home}", self.label));
                None
            }
        }
    }

    fn listed(&self, leaf: &str) -> Option<&'static Difference> {
        DIFFERENCES
            .iter()
            .find(|d| d.leaf == leaf && d.presets.contains(&self.preset))
    }
}

/// The font DPI 0.7.2 resolves a theme in on macOS,
/// `ResolutionContext::from_system`'s there, "where a point is a pixel"
/// (`native-theme/src/resolve/context.rs`; the model parity's
/// `MACOS_RESOLUTION_CONTEXT`, `native-theme/src/next/parity.rs`).
const MACOS_FONT_DPI: f32 = 72.0;

/// The context the 0.7.2 side is resolved in: `ResolutionContext::for_tests`,
/// on the macOS presets with the font DPI of the platform they describe
/// ([`MACOS_FONT_DPI`]), so a size in points is the size 0.7.2 gives there.
pub(crate) fn old_context(preset: Preset) -> native_theme::ResolutionContext {
    match preset {
        Preset::MacosMonterey | Preset::MacosTahoe => native_theme::ResolutionContext {
            font_dpi: MACOS_FONT_DPI,
            ..native_theme::ResolutionContext::for_tests()
        },
        _ => native_theme::ResolutionContext::for_tests(),
    }
}

/// The two resolved themes of `preset` for `is_dark`, and 0.7.2's name and
/// layout, which its theme holds beside the variants.
fn resolve(
    key: &str,
    preset: Preset,
    is_dark: bool,
) -> crate::Result<(
    ResolvedTheme,
    String,
    LayoutTheme,
    native_theme::next::ResolvedTheme,
)> {
    let (mode, ctx) = if is_dark {
        (ColorMode::Dark, ThemeContext::dark())
    } else {
        (ColorMode::Light, ThemeContext::light())
    };
    let theme = crate::Theme::preset(key)?;
    let name = theme.name.to_string();
    let layout = theme.layout.clone();
    let old = theme
        .into_variant(mode)?
        .into_resolved(&old_context(preset))?;
    let new = native_theme::next::Theme::preset(preset).resolve(&ctx);
    Ok((old, name, layout, new))
}

/// The 0.8.0 theme of `preset` for `is_dark`, resolved for an inactive
/// window, where the table of homes places 0.7.2's `window.inactive_*`
/// fields (their rows' `context`).
fn resolve_inactive(preset: Preset, is_dark: bool) -> native_theme::next::ResolvedTheme {
    let mut ctx = if is_dark {
        ThemeContext::dark()
    } else {
        ThemeContext::light()
    };
    ctx.window_active = false;
    native_theme::next::Theme::preset(preset).resolve(&ctx)
}

/// The theme's leaves: the colours, their tokens and config strings, the
/// theme's other fields, the config's in the slot of its mode and the other
/// slot's, and then the whole theme in its `Debug` form, which also holds
/// what no field compared here names (the base palette `ThemeConfigColors`
/// keeps private, a field upstream adds).
fn compare_theme(
    report: &mut Report,
    old: &ResolvedTheme,
    name: &str,
    new: &native_theme::next::ResolvedTheme,
    is_dark: bool,
) {
    let old_theme = crate::to_theme(old, name, is_dark, &AccessibilityPreferences::DEFAULT);
    let Some(new_theme) = report.mapped("to_theme", super::to_theme(new)) else {
        return;
    };
    let new_theme = match report.listed(ROW_HOVER) {
        Some(listed) => {
            report.count(ROW_HOVER);
            match row_hover_as_list(&new_theme) {
                Some(patched) => patched,
                None => {
                    report.failures.push(format!(
                        "{}: {ROW_HOVER}: the listed difference ({}) no longer describes it: \
                         0.8.0 list_hover {:?}, table_hover {:?}",
                        report.label,
                        listed.reason,
                        new_theme.colors.list_hover,
                        new_theme.colors.table_hover
                    ));
                    new_theme
                }
            }
        }
        None => new_theme,
    };
    let failures = report.failures.len();
    let config = |t: &GpuiTheme, own: bool| {
        if is_dark == own {
            t.dark_theme.clone()
        } else {
            t.light_theme.clone()
        }
    };
    let (old_config, new_config) = (config(&old_theme, true), config(&new_theme, true));
    for slot in SLOTS {
        report.leaf(
            &format!("colors.{}", slot.name),
            (slot.color)(&old_theme),
            (slot.color)(&new_theme),
        );
        report.leaf(
            &format!("tokens.{}", slot.name),
            (slot.token)(&old_theme),
            (slot.token)(&new_theme),
        );
        if let Some(get) = slot.config {
            report.leaf(
                &format!("config.colors.{}", slot.name),
                get(&old_config),
                get(&new_config),
            );
        }
    }
    let (o, n) = (&old_theme, &new_theme);
    report.leaf("mode", o.mode, n.mode);
    report.leaf("font_family", &o.font_family, &n.font_family);
    report.leaf("font_size", o.font_size, n.font_size);
    report.leaf("mono_font_family", &o.mono_font_family, &n.mono_font_family);
    report.leaf("mono_font_size", o.mono_font_size, n.mono_font_size);
    report.leaf("radius", o.radius, n.radius);
    report.leaf("radius_lg", o.radius_lg, n.radius_lg);
    report.leaf("shadow", o.shadow, n.shadow);
    report.leaf("focus_ring", o.focus_ring, n.focus_ring);
    report.leaf("scrollbar_mode", o.scrollbar_mode, n.scrollbar_mode);
    report.leaf("window_radius", o.window_radius, n.window_radius);
    report.leaf("cursor_blink", o.cursor_blink, n.cursor_blink);
    report.leaf("highlight_theme", &*o.highlight_theme, &*n.highlight_theme);
    report.leaf("transparent", o.transparent, n.transparent);
    // `NotificationSettings`, `ListSettings`, `SheetSettings` and
    // `MotionTokens` have no `PartialEq`: compared in their `Debug` form.
    report.leaf(
        "notification",
        format!("{:?}", o.notification),
        format!("{:?}", n.notification),
    );
    report.leaf("list", format!("{:?}", o.list), format!("{:?}", n.list));
    report.leaf("sheet", format!("{:?}", o.sheet), format!("{:?}", n.sheet));
    report.leaf(
        "motion",
        format!("{:?}", o.motion),
        format!("{:?}", n.motion),
    );
    report.leaf(
        "other_config",
        format!("{:?}", config(o, false)),
        format!("{:?}", config(n, false)),
    );
    // `variants::ghost_button` reads these three fields and nothing else.
    let ghost = |t: &GpuiTheme| {
        (
            t.colors.secondary_foreground,
            t.colors.secondary_hover,
            t.colors.secondary_active,
        )
    };
    report.leaf("variants::ghost_button", ghost(o), ghost(n));
    let (o, n) = (&old_config, &new_config);
    report.leaf("config.is_default", o.is_default, n.is_default);
    report.leaf("config.name", &o.name, &n.name);
    report.leaf("config.mode", o.mode, n.mode);
    report.leaf("config.font_family", &o.font_family, &n.font_family);
    report.leaf("config.font_size", o.font_size, n.font_size);
    report.leaf(
        "config.mono_font_family",
        &o.mono_font_family,
        &n.mono_font_family,
    );
    report.leaf("config.mono_font_size", o.mono_font_size, n.mono_font_size);
    report.leaf("config.radius", o.radius, n.radius);
    report.leaf("config.radius_lg", o.radius_lg, n.radius_lg);
    report.leaf("config.window_radius", o.window_radius, n.window_radius);
    report.leaf("config.shadow", o.shadow, n.shadow);
    report.leaf(
        "config.colors.group_box_title_foreground",
        &o.colors.group_box_title_foreground,
        &n.colors.group_box_title_foreground,
    );
    report.leaf("config.highlight", &o.highlight, &n.highlight);
    let whole = "to_theme (Debug form)";
    report.count(whole);
    let (o, n) = (format!("{old_theme:?}"), format!("{new_theme:?}"));
    if report.failures.len() == failures && o != n {
        let at = o.bytes().zip(n.bytes()).take_while(|(a, b)| a == b).count();
        let around = |s: &str| {
            s.get(at.saturating_sub(80)..)
                .unwrap_or("")
                .chars()
                .take(160)
                .collect::<String>()
        };
        report.failures.push(format!(
            "{}: {whole}: differs beyond the leaves compared, at byte {at}: 0.7.2 …{}…, \
             0.8.0 …{}…",
            report.label,
            around(&o),
            around(&n)
        ));
    }
}

/// The resize handles' colours, field by field.
fn compare_resizable(
    report: &mut Report,
    old: &ResolvedTheme,
    new: &native_theme::next::ResolvedTheme,
) {
    let o = crate::base_layer::resizable_theme(old);
    let Some(n) = report.mapped("ResizableTheme", super::resizable_theme(new)) else {
        return;
    };
    report.leaf("ResizableTheme.handle", o.handle, n.handle);
    report.leaf(
        "ResizableTheme.active_handle",
        o.active_handle,
        n.active_handle,
    );
}

/// The scroll bar's geometry, field by field.
fn compare_scrollbar(
    report: &mut Report,
    old: &ResolvedTheme,
    new: &native_theme::next::ResolvedTheme,
) {
    let o = crate::base_layer::scrollbar_geometry(old);
    let Some(n) = report.mapped("ScrollbarGeometry", super::scrollbar_geometry(new)) else {
        return;
    };
    report.leaf(
        "ScrollbarGeometry.track_width",
        o.track_width,
        n.track_width,
    );
    report.leaf(
        "ScrollbarGeometry.thumb_width",
        o.thumb_width,
        n.thumb_width,
    );
    report.leaf(
        "ScrollbarGeometry.thumb_inset",
        o.thumb_inset,
        n.thumb_inset,
    );
    report.leaf(
        "ScrollbarGeometry.thumb_radius",
        o.thumb_radius,
        n.thumb_radius,
    );
    report.leaf(
        "ScrollbarGeometry.min_thumb_length",
        o.min_thumb_length,
        n.min_thumb_length,
    );
    report.leaf("ScrollbarGeometry.track", o.track, n.track);
    report.leaf(
        "ScrollbarGeometry.track_active_border",
        o.track_active_border,
        n.track_active_border,
    );
    report.leaf("ScrollbarGeometry.thumb", o.thumb, n.thumb);
    report.leaf(
        "ScrollbarGeometry.thumb_hover",
        o.thumb_hover,
        n.thumb_hover,
    );
    report.leaf(
        "ScrollbarGeometry.thumb_active",
        o.thumb_active,
        n.thumb_active,
    );
}

/// Compare a leaf's two values, the 0.8.0 one where mapped.
fn compare_mapped<T: PartialEq + Debug>(
    report: &mut Report,
    leaf: &str,
    old: T,
    new: super::Mapped<T>,
) {
    if let Some(n) = report.mapped(leaf, new) {
        report.leaf(leaf, old, n);
    }
}

/// Compare a builder's two outputs.
macro_rules! builder {
    ($report:expr, $old:expr, $new:expr, $name:ident) => {
        compare_mapped(
            $report,
            concat!("geometry::", stringify!($name)),
            crate::geometry::$name(Native::unscaled($old)),
            super::builders::$name($new),
        )
    };
}

/// Compare a fill builder's two outputs, at rest and disabled, as one leaf.
macro_rules! fill_builder {
    ($report:expr, $old:expr, $new:expr, $name:ident) => {
        compare_mapped(
            $report,
            concat!("geometry::", stringify!($name)),
            (
                crate::geometry::$name(Native::unscaled($old), false),
                crate::geometry::$name(Native::unscaled($old), true),
            ),
            super::builders::$name($new, false)
                .and_then(|rest| Ok((rest, super::builders::$name($new, true)?))),
        )
    };
}

/// Compare a layout metric's two values.
macro_rules! layout_metric {
    ($report:expr, $layout:expr, $new:expr, $name:ident) => {
        compare_mapped(
            $report,
            concat!("geometry::", stringify!($name)),
            crate::geometry::$name($layout),
            super::builders::$name($new),
        )
    };
}

/// The `geometry` builders, and the `variants` builders through the values
/// the connector feeds them with; `inactive` is the 0.8.0 theme resolved for
/// an inactive window.
fn compare_builders(
    report: &mut Report,
    old: &ResolvedTheme,
    layout: &LayoutTheme,
    new: &native_theme::next::ResolvedTheme,
    inactive: &native_theme::next::ResolvedTheme,
) {
    let button = "geometry::button";
    if let Some(n) = report.mapped(button, super::builders::button(new)) {
        let o = crate::geometry::button(Native::unscaled(old));
        match report.listed(button) {
            Some(listed) => {
                report.count(button);
                let mut corrected = o.clone();
                corrected.size.height = None;
                let stated = Some(Length::from(px(KDE_BREEZE_BUTTON_HEIGHT_PX)));
                if o.size.height != stated || n.size.height.is_some() || corrected != n {
                    report.failures.push(format!(
                        "{}: {button}: the listed difference ({}) no longer describes \
                         it: 0.7.2 {o:?}, 0.8.0 {n:?}",
                        report.label, listed.reason
                    ));
                }
            }
            None => report.leaf(button, o, n),
        }
    }
    builder!(report, old, new, button_label);
    builder!(report, old, new, toggle);
    builder!(report, old, new, button_disabled);
    builder!(report, old, new, button_checked);
    builder!(report, old, new, default_button_rest);
    builder!(report, old, new, checkbox);
    builder!(report, old, new, radio);
    builder!(report, old, new, list);
    builder!(report, old, new, table);
    builder!(report, old, new, list_item_selected);
    builder!(report, old, new, data_table_size);
    builder!(report, old, new, list_header_height);
    builder!(report, old, new, tooltip);
    builder!(report, old, new, tooltip_content);
    builder!(report, old, new, icon_size_small);
    builder!(report, old, new, icon_size_large);
    builder!(report, old, new, icon_size_dialog);
    builder!(report, old, new, icon_size_panel);
    builder!(report, old, new, scrollbar_gutter);
    builder!(report, old, new, input_group_button);
    builder!(report, old, new, editor);
    builder!(report, old, new, input);
    builder!(report, old, new, text_area);
    builder!(report, old, new, input_height);
    builder!(report, old, new, select);
    builder!(report, old, new, combobox);
    fill_builder!(report, old, new, input_fill);
    fill_builder!(report, old, new, combo_box_fill);
    builder!(report, old, new, menu_item);
    builder!(report, old, new, popover);
    let dialog = "geometry::dialog";
    match report.listed(dialog) {
        Some(listed) => {
            if let Some(n) = report.mapped(dialog, super::builders::dialog(new)) {
                report.count(dialog);
                let o = crate::geometry::dialog(Native::unscaled(old));
                let mut corrected = o.clone();
                corrected.min_size.width = None;
                corrected.min_size.height = None;
                corrected.max_size.height = None;
                let stated = o.min_size.width.is_some()
                    && o.min_size.height.is_some()
                    && o.max_size.height.is_some();
                if !stated || corrected != n {
                    report.failures.push(format!(
                        "{}: {dialog}: the listed difference ({}) no longer describes \
                         it: 0.7.2 {o:?}, 0.8.0 {n:?}",
                        report.label, listed.reason
                    ));
                }
            }
        }
        None => builder!(report, old, new, dialog),
    }
    builder!(report, old, new, dialog_entrance);
    builder!(report, old, new, dialog_footer);
    builder!(report, old, new, dialog_title);
    builder!(report, old, new, dialog_description);
    builder!(report, old, new, progress);
    let group_box = "geometry::group_box_content";
    match report.listed(group_box) {
        Some(listed) => {
            if let Some(n) = report.mapped(group_box, super::builders::group_box_content(new)) {
                report.count(group_box);
                let o = crate::geometry::group_box_content(Native::unscaled(old));
                let mut corrected = o.clone();
                corrected.border_widths = Default::default();
                corrected.border_color = None;
                let mut invented = o.border_widths.top == Some(px(0.0).into())
                    && o.border_color == Some(Hsla::transparent_black());
                if UNSTATED_CARD_CORNERS.contains(&report.preset) {
                    use gpui::Styled as _;
                    corrected.corner_radii = Default::default();
                    invented &= o.corner_radii
                        == gpui::StyleRefinement::default()
                            .rounded(px(0.0))
                            .corner_radii;
                }
                if !invented || corrected != n {
                    report.failures.push(format!(
                        "{}: {group_box}: the listed difference ({}) no longer describes \
                         it: 0.7.2 {o:?}, 0.8.0 {n:?}",
                        report.label, listed.reason
                    ));
                }
            }
        }
        None => builder!(report, old, new, group_box_content),
    }
    builder!(report, old, new, accordion_title);
    builder!(report, old, new, accordion);
    builder!(report, old, new, accordion_hover);
    builder!(report, old, new, spinner_size);
    builder!(report, old, new, link);
    compare_mapped(
        report,
        "variants::switch_color",
        crate::variants::switch_color(Native::unscaled(old)),
        super::builders::switch_color(new),
    );
    builder!(report, old, new, status_bar);
    builder!(report, old, new, title_bar);
    compare_mapped(
        report,
        "geometry::title_bar_inactive",
        crate::geometry::title_bar_inactive(Native::unscaled(old)),
        super::builders::title_bar_inactive(new, inactive),
    );
    builder!(report, old, new, toolbar);
    builder!(report, old, new, icon_size_toolbar);
    builder!(report, old, new, toolbar_button_rest);
    builder!(report, old, new, toolbar_button_checked);
    compare_mapped(
        report,
        "variants::toolbar_button",
        crate::variants::toolbar_button_states(Native::unscaled(old)),
        super::builders::toolbar_button_states(new),
    );
    let max_width = "geometry::dialog_max_width";
    match report.listed(max_width) {
        Some(listed) => {
            report.count(max_width);
            let n = super::builders::dialog_max_width(new);
            if n != Ok(None) {
                report.failures.push(format!(
                    "{}: {max_width}: the listed difference ({}) no longer describes it: \
                     0.7.2 {:?}, 0.8.0 {n:?}",
                    report.label,
                    listed.reason,
                    crate::geometry::dialog_max_width(Native::unscaled(old))
                ));
            }
        }
        None => compare_mapped(
            report,
            max_width,
            Some(crate::geometry::dialog_max_width(Native::unscaled(old))),
            super::builders::dialog_max_width(new),
        ),
    }
    let list_item = "geometry::list_item";
    if let Some(n) = report.mapped(list_item, super::builders::list_item(new)) {
        let o = crate::geometry::list_item(Native::unscaled(old));
        match report.listed(list_item) {
            Some(listed) => {
                report.count(list_item);
                let [top, right, bottom, left] = KDE_BREEZE_ROW_PADDING_PX;
                let grown = |side: Option<gpui::DefiniteLength>, by: f32| {
                    side.and_then(|length| match length {
                        gpui::DefiniteLength::Absolute(gpui::AbsoluteLength::Pixels(p)) => {
                            Some(gpui::DefiniteLength::from(px(f32::from(p) + by)))
                        }
                        _ => None,
                    })
                };
                let mut corrected = o.clone();
                corrected.padding.top = grown(o.padding.top, top);
                corrected.padding.right = grown(o.padding.right, right);
                corrected.padding.bottom = grown(o.padding.bottom, bottom);
                corrected.padding.left = grown(o.padding.left, left);
                if corrected != n {
                    report.failures.push(format!(
                        "{}: {list_item}: the listed difference ({}) no longer describes \
                         it: 0.7.2 {o:?}, 0.8.0 {n:?}",
                        report.label, listed.reason
                    ));
                }
            }
            None => report.leaf(list_item, o, n),
        }
    }
    layout_metric!(report, layout, new, widget_gap);
    layout_metric!(report, layout, new, container_margin);
    layout_metric!(report, layout, new, window_margin);
    layout_metric!(report, layout, new, section_gap);
    compare_mapped(
        report,
        "variants::bordered_button",
        crate::variants::hover_border(Native::unscaled(old)),
        super::builders::hover_border(new),
    );
    compare_mapped(
        report,
        "variants::default_button",
        crate::variants::default_button_states(Native::unscaled(old)),
        super::builders::default_button_states(new),
    );
}

/// The crate root's accessors and their [`super`] counterparts, each named
/// `root::<name>`: colours as gpui's (`to_hsla` of 0.7.2's, the connector's
/// conversion of the model's), an unset 0.7.2 option against a `None`, and
/// `text_scale`'s four roles by their size, weight and line height.
fn compare_root(report: &mut Report, old: &ResolvedTheme, new: &native_theme::next::ResolvedTheme) {
    use native_theme::next::Color;
    let c = |color: Color| super::hsla(color);
    if let Some(n) = report.mapped("root::is_dark_resolved", super::is_dark_resolved(new)) {
        report.leaf("root::is_dark_resolved", crate::is_dark_resolved(old), n);
    }
    report.leaf(
        "root::border_line_width",
        crate::border_line_width(old),
        super::border_line_width(new),
    );
    report.leaf(
        "root::disabled_opacity",
        crate::disabled_opacity(old),
        super::disabled_opacity(new),
    );
    report.leaf(
        "root::border_opacity",
        crate::border_opacity(old),
        super::border_opacity(new),
    );
    report.leaf(
        "root::border_shadow_enabled",
        crate::border_shadow_enabled(old),
        super::border_shadow_enabled(new),
    );
    let (o, n) = (crate::icon_sizes(old), super::icon_sizes(new));
    report.leaf(
        "root::icon_sizes",
        [
            Some(o.toolbar),
            Some(o.small),
            Some(o.large),
            Some(o.dialog),
            Some(o.panel),
        ],
        [n.toolbar, n.small, n.large, n.dialog, n.panel],
    );
    let (o, n) = (crate::text_scale(old), super::text_scale(new));
    report.leaf(
        "root::text_scale",
        [&o.caption, &o.section_heading, &o.dialog_title, &o.display]
            .map(|e| (e.size, e.weight, e.line_height)),
        [n.caption, n.section_heading, n.dialog_title, n.display]
            .map(|t| (t.size_px, t.weight, t.line_height)),
    );
    report.leaf(
        "root::line_height",
        crate::line_height(old),
        super::line_height(new),
    );
    compare_mapped(
        report,
        "root::font_weight",
        crate::font_weight(old),
        super::font_weight(new),
    );
    compare_mapped(
        report,
        "root::mono_font_weight",
        crate::mono_font_weight(old),
        super::mono_font_weight(new),
    );
    compare_mapped(
        report,
        "root::dialog_button_order",
        format!("{:?}", crate::dialog_button_order(old)),
        super::dialog_button_order(new).map(|order| format!("{order:?}")),
    );
    report.leaf(
        "root::dialog_button_gap",
        crate::dialog_button_gap(old),
        super::dialog_button_gap(new),
    );
    compare_mapped(
        report,
        "root::scrollbar_groove_width",
        crate::scrollbar_groove_width(old),
        super::scrollbar_groove_width(new),
    );
    report.leaf(
        "root::selection_text_color",
        crate::to_hsla(crate::selection_text_color(old)),
        c(super::selection_text_color(new)),
    );
    compare_mapped(
        report,
        "root::selection_inactive_background",
        crate::to_hsla(crate::selection_inactive_background(old)),
        super::selection_inactive_background(new).map(c),
    );
    compare_mapped(
        report,
        "root::selection_inactive_text_color",
        crate::to_hsla(crate::selection_inactive_text_color(old)),
        super::selection_inactive_text_color(new).map(c),
    );
    compare_mapped(
        report,
        "root::list_header_hover_background",
        crate::list_header_hover_background(old).map(crate::to_hsla),
        super::list_header_hover_background(new).map(|v| v.map(c)),
    );
    compare_mapped(
        report,
        "root::list_header_active_background",
        crate::list_header_active_background(old).map(crate::to_hsla),
        super::list_header_active_background(new).map(|v| v.map(c)),
    );
    report.leaf(
        "root::disabled_text_color",
        crate::to_hsla(crate::disabled_text_color(old)),
        c(super::disabled_text_color(new)),
    );
    compare_mapped(
        report,
        "root::focus_ring_width",
        crate::focus_ring_width(old),
        super::focus_ring_width(new),
    );
    report.leaf(
        "root::focus_ring_offset",
        crate::focus_ring_offset(old),
        super::focus_ring_offset(new),
    );
    report.leaf(
        "root::tooltip_show_delay",
        crate::tooltip_show_delay(old),
        super::tooltip_show_delay(new),
    );
}

/// Every leaf of `preset` for `is_dark`, compared.
fn compare_all(key: &'static str, preset: Preset, is_dark: bool) -> crate::Result<Report> {
    let (old, name, layout, new) = resolve(key, preset, is_dark)?;
    let mut report = Report::new(key, is_dark);
    compare_theme(&mut report, &old, &name, &new, is_dark);
    compare_scrollbar(&mut report, &old, &new);
    compare_resizable(&mut report, &old, &new);
    compare_root(&mut report, &old, &new);
    compare_builders(
        &mut report,
        &old,
        &layout,
        &new,
        &resolve_inactive(preset, is_dark),
    );
    for entry in FALLBACK_ONLY {
        if entry.presets.contains(&key) && !(entry.unset)(&old) {
            report.failures.push(format!(
                "{}: {}: {} is stated, so the leaf no longer compares only the fallback: \
                 remove the preset from its FALLBACK_ONLY entry",
                report.label, entry.leaf, entry.field
            ));
        }
    }
    Ok(report)
}

#[test]
fn the_slice_maps_to_the_same_leaves_from_both_models() -> crate::Result<()> {
    let mut failures = Vec::new();
    for &(key, preset) in PRESETS {
        for is_dark in [false, true] {
            let report = compare_all(key, preset, is_dark)?;
            println!("{}: {} leaves compared", report.label, report.compared);
            failures.extend(report.failures);
        }
    }
    assert!(
        failures.is_empty(),
        "{} leaves differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
    Ok(())
}

/// The names of the top-level `pub fn`s of the source `src`, each after
/// `module::`.
fn public_fns(module: &str, src: &str) -> Vec<String> {
    src.lines()
        .filter_map(|line| line.strip_prefix("pub fn "))
        .filter_map(|rest| rest.split(['(', '<']).next())
        .map(|name| format!("{module}::{name}"))
        .collect()
}

#[test]
fn every_builder_is_compared() -> crate::Result<()> {
    let public: Vec<String> = public_fns("geometry", include_str!("../geometry.rs"))
        .into_iter()
        .chain(public_fns("variants", include_str!("../variants.rs")))
        .collect();
    let mut compared = Vec::new();
    for &(key, preset) in PRESETS {
        for is_dark in [false, true] {
            compared.extend(compare_all(key, preset, is_dark)?.seen);
        }
    }
    let missing: Vec<&String> = public.iter().filter(|b| !compared.contains(b)).collect();
    assert!(
        !public.is_empty() && missing.is_empty(),
        "public builders not compared: {missing:?}"
    );
    Ok(())
}

/// The `pub fn`s of `src/next/mod.rs` compared elsewhere than as
/// `root::<name>`: `to_theme` leaf by leaf ([`compare_theme`]), and `apply`
/// by `next::tests`, against [`crate::apply`].
const NOT_ROOT_ACCESSORS: &[&str] = &["root::to_theme", "root::apply"];

#[test]
fn every_root_accessor_is_compared() -> crate::Result<()> {
    let public: Vec<String> = public_fns("root", include_str!("mod.rs"))
        .into_iter()
        .filter(|name| !NOT_ROOT_ACCESSORS.contains(&name.as_str()))
        .collect();
    let mut compared = Vec::new();
    for &(key, preset) in PRESETS {
        for is_dark in [false, true] {
            compared.extend(compare_all(key, preset, is_dark)?.seen);
        }
    }
    let missing: Vec<&String> = public.iter().filter(|b| !compared.contains(b)).collect();
    assert!(
        !public.is_empty() && missing.is_empty(),
        "public accessors of next not compared: {missing:?}"
    );
    Ok(())
}

#[test]
fn every_fallback_only_entry_names_a_compared_leaf_and_a_covered_preset() -> crate::Result<()> {
    for entry in FALLBACK_ONLY {
        for preset in entry.presets {
            let covered = PRESETS.iter().find(|(key, _)| key == preset);
            assert!(
                covered.is_some(),
                "{}: {preset} is not a preset the test covers",
                entry.leaf
            );
            let Some(&(key, p)) = covered else {
                continue;
            };
            let report = compare_all(key, p, false)?;
            assert!(
                report.seen.iter().any(|leaf| leaf == entry.leaf),
                "{}: not a leaf the test compares",
                entry.leaf
            );
        }
    }
    Ok(())
}

#[test]
fn every_contract_slot_is_compared_or_accounted_for() {
    let named: Vec<&str> = SLOTS
        .iter()
        .map(|s| s.name)
        .chain(EXCLUDED.iter().map(|(slot, _)| *slot))
        .collect();
    let tables = crate::contract::table_slots();
    let missing: Vec<&&str> = tables.iter().filter(|s| !named.contains(s)).collect();
    let unknown: Vec<&&str> = named.iter().filter(|s| !tables.contains(s)).collect();
    let mut twice: Vec<&&str> = named
        .iter()
        .enumerate()
        .filter(|(i, s)| named.iter().skip(i + 1).any(|t| t == *s))
        .map(|(_, s)| s)
        .collect();
    twice.dedup();
    assert!(
        missing.is_empty() && unknown.is_empty() && twice.is_empty(),
        "contract slots in no list: {missing:?}; listed slots the contract tables do not \
         name: {unknown:?}; slots in more than one list: {twice:?}"
    );
}

#[test]
fn every_listed_difference_names_a_covered_preset() {
    for d in DIFFERENCES {
        for preset in d.presets {
            assert!(
                PRESETS.iter().any(|(key, _)| key == preset),
                "{}: {preset} is not a preset the test covers",
                d.leaf
            );
        }
    }
}

/// What `apply` leaves installed, each part in its `Debug` form: the styled
/// theme (its configs included), gpui-base's resize handles, the base
/// overrides the connector writes for the current mode, the tooltip show
/// delay, reduced motion, the stored preferences and the focus band's rule.
/// Where `row_hover_listed`, the styled theme is given with its hovered row
/// as 0.7.2's ([`row_hover_as_list`]), or as a failure where it does not
/// hold gpui-component's own.
fn installed(cx: &gpui::App, row_hover_listed: bool) -> Vec<(&'static str, String)> {
    let styled = GpuiTheme::global(cx);
    let is_dark = styled.is_dark();
    let styled_debug = if row_hover_listed {
        row_hover_as_list(styled).map_or_else(
            || format!("{ROW_HOVER}: not gpui-component's own hovered row"),
            |patched| format!("{patched:?}"),
        )
    } else {
        format!("{styled:?}")
    };
    let base = gpui_base::Theme::global(cx);
    let nt = cx.try_global::<crate::NativeTheme>();
    let overrides = nt
        .and_then(|nt| crate::base_overrides_for(nt, is_dark, Some(styled)))
        .map(|(g, r)| format!("{g:?} {:?} {:?}", r.handle, r.active_handle));
    let band = nt
        .and_then(|nt| nt.source(is_dark))
        .map(crate::Variant::keyboard_focus_band);
    vec![
        ("styled theme", styled_debug),
        (
            "base handles",
            format!(
                "{:?} {:?}",
                base.resizable.handle, base.resizable.active_handle
            ),
        ),
        ("base overrides", format!("{overrides:?}")),
        (
            "tooltip delay",
            format!("{:?}", gpui_base::TooltipDefaults::global(cx).show_delay()),
        ),
        ("reduce motion", format!("{}", cx.reduce_motion())),
        (
            "preferences",
            format!("{:?}", nt.map(|nt| nt.accessibility())),
        ),
        ("focus band", format!("{band:?}")),
    ]
}

/// The two schemes of `key` (0.7.2) or `preset` (0.8.0) applied, light then
/// dark, with no variant stored before: what is installed after the calls,
/// and after an upstream switch to light mode (`Theme::change`), which the
/// connector's observer answers from the stored variants.
fn apply_both(
    cx: &mut gpui::TestAppContext,
    key: &str,
    preset: Preset,
    next: bool,
) -> Result<[Vec<(&'static str, String)>; 2], String> {
    let mut themes = Vec::new();
    for is_dark in [false, true] {
        themes.push((
            is_dark,
            resolve(key, preset, is_dark).map_err(|e| e.to_string())?,
        ));
    }
    cx.update(|cx| -> Result<(), String> {
        if let Some(nt) = cx.try_global::<crate::NativeTheme>()
            && (nt.light.is_some() || nt.dark.is_some())
        {
            let nt = cx.global_mut::<crate::NativeTheme>();
            nt.light = None;
            nt.dark = None;
        }
        for (is_dark, (old, name, _, new)) in &themes {
            if next {
                super::apply(new, cx).map_err(|e| e.to_string())?;
            } else {
                let prefs = AccessibilityPreferences::DEFAULT;
                let theme = crate::to_theme(old, name, *is_dark, &prefs);
                crate::apply(theme, old, &prefs, cx);
            }
        }
        Ok(())
    })?;
    let row_hover_listed = next
        && DIFFERENCES
            .iter()
            .any(|d| d.leaf == ROW_HOVER && d.presets.contains(&key));
    let after = cx.update(|cx| installed(cx, row_hover_listed));
    cx.update(|cx| GpuiTheme::change(gpui_component::theme::ThemeMode::Light, None, cx));
    let switched = cx.update(|cx| installed(cx, row_hover_listed));
    Ok([after, switched])
}

/// `next::apply` installs what `apply` installs, on every covered preset:
/// after both schemes are applied, and after upstream switches the mode.
#[gpui::test]
fn next_apply_installs_what_apply_installs(cx: &mut gpui::TestAppContext) {
    let mut failures = Vec::new();
    for &(key, preset) in PRESETS {
        let both = apply_both(cx, key, preset, false)
            .and_then(|old| apply_both(cx, key, preset, true).map(|new| (old, new)));
        match both {
            Ok((old, new)) => {
                for (when, (o, n)) in ["after apply", "after Theme::change"]
                    .iter()
                    .zip(old.iter().zip(new.iter()))
                {
                    for ((part, o), (_, n)) in o.iter().zip(n.iter()) {
                        if o != n {
                            failures.push(format!("{key} {when}: {part}: 0.7.2 {o}, 0.8.0 {n}"));
                        }
                    }
                }
            }
            Err(e) => failures.push(format!("{key}: {e}")),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// The names of the top-level `pub fn`s of the source `src` that return a
/// [`super::Mapped`], each after `module::`.
fn mapped_fns(module: &str, src: &str) -> Vec<String> {
    src.lines()
        .filter(|line| line.contains("-> Mapped<"))
        .filter_map(|line| line.strip_prefix("pub fn "))
        .filter_map(|rest| rest.split(['(', '<']).next())
        .map(|name| format!("{module}::{name}"))
        .collect()
}

/// Call each `next::` function that returns a [`super::Mapped`], recording
/// its name, and its error as a failure.
macro_rules! succeeds {
    ($called:expr, $failures:expr, $label:expr, $name:expr, $result:expr) => {{
        $called.push(String::from($name));
        if let Err(e) = $result {
            $failures.push(format!("{}: {}: {e}", $label, $name));
        }
    }};
}

/// [`succeeds`] for each `next::geometry` builder of the form
/// `fn(&ResolvedTheme) -> Mapped<T>`.
macro_rules! geometry_succeeds {
    ($called:expr, $failures:expr, $label:expr, $r:expr, [$($name:ident),* $(,)?]) => {
        $(
            succeeds!(
                $called,
                $failures,
                $label,
                concat!("geometry::", stringify!($name)),
                super::geometry::$name($r)
            );
        )*
    };
}

/// Every `next::` function that can report an [`super::Unmapped`] source
/// succeeds on every covered preset, light and dark: a value the preset
/// leaves unstated keeps the toolkit's own (docs/v0.8.0-spec.md §10, rule 0)
/// rather than failing the call.
#[gpui::test]
fn every_next_function_succeeds_on_every_covered_preset(cx: &mut gpui::TestAppContext) {
    let mut called: Vec<String> = Vec::new();
    let mut failures: Vec<String> = Vec::new();
    cx.update(gpui_component::init);
    for &(key, preset) in PRESETS {
        for is_dark in [false, true] {
            let label = format!("{key}/{}", if is_dark { "dark" } else { "light" });
            let ctx = if is_dark {
                ThemeContext::dark()
            } else {
                ThemeContext::light()
            };
            let r = native_theme::next::Theme::preset(preset).resolve(&ctx);
            let inactive = resolve_inactive(preset, is_dark);
            let (c, f, l) = (&mut called, &mut failures, &label);
            succeeds!(c, f, l, "root::to_theme", super::to_theme(&r));
            succeeds!(
                c,
                f,
                l,
                "root::is_dark_resolved",
                super::is_dark_resolved(&r)
            );
            succeeds!(c, f, l, "root::font_weight", super::font_weight(&r));
            succeeds!(
                c,
                f,
                l,
                "root::mono_font_weight",
                super::mono_font_weight(&r)
            );
            succeeds!(
                c,
                f,
                l,
                "root::dialog_button_order",
                super::dialog_button_order(&r)
            );
            succeeds!(
                c,
                f,
                l,
                "root::scrollbar_groove_width",
                super::scrollbar_groove_width(&r)
            );
            succeeds!(
                c,
                f,
                l,
                "root::selection_inactive_background",
                super::selection_inactive_background(&r)
            );
            succeeds!(
                c,
                f,
                l,
                "root::selection_inactive_text_color",
                super::selection_inactive_text_color(&r)
            );
            succeeds!(
                c,
                f,
                l,
                "root::list_header_hover_background",
                super::list_header_hover_background(&r)
            );
            succeeds!(
                c,
                f,
                l,
                "root::list_header_active_background",
                super::list_header_active_background(&r)
            );
            succeeds!(
                c,
                f,
                l,
                "root::focus_ring_width",
                super::focus_ring_width(&r)
            );
            succeeds!(
                c,
                f,
                l,
                "base_layer::scrollbar_geometry",
                super::base_layer::scrollbar_geometry(&r)
            );
            succeeds!(
                c,
                f,
                l,
                "base_layer::resizable_theme",
                super::base_layer::resizable_theme(&r)
            );
            succeeds!(
                c,
                f,
                l,
                "variants::switch_color",
                super::variants::switch_color(&r)
            );
            geometry_succeeds!(
                c,
                f,
                l,
                &r,
                [
                    button,
                    button_label,
                    toggle,
                    input,
                    text_area,
                    editor,
                    menu_item,
                    list_item,
                    tooltip,
                    tooltip_content,
                    popover,
                    status_bar,
                    dialog,
                    dialog_entrance,
                    input_group_button,
                    dialog_footer,
                    dialog_title,
                    dialog_description,
                    list,
                    table,
                    progress,
                    group_box_content,
                    accordion_title,
                    accordion,
                    accordion_hover,
                    checkbox,
                    radio,
                    select,
                    combobox,
                    title_bar,
                    toolbar,
                    spinner_size,
                    data_table_size,
                    list_header_height,
                    icon_size_toolbar,
                    icon_size_small,
                    icon_size_large,
                    icon_size_dialog,
                    icon_size_panel,
                    scrollbar_gutter,
                    dialog_max_width,
                    input_height,
                    button_disabled,
                    button_checked,
                    toolbar_button_rest,
                    toolbar_button_checked,
                    default_button_rest,
                    list_item_selected,
                    link,
                    widget_gap,
                    container_margin,
                    window_margin,
                    section_gap,
                ]
            );
            succeeds!(
                c,
                f,
                l,
                "geometry::title_bar_inactive",
                super::geometry::title_bar_inactive(&r, &inactive)
            );
            for disabled in [false, true] {
                succeeds!(
                    c,
                    f,
                    l,
                    "geometry::input_fill",
                    super::geometry::input_fill(&r, disabled)
                );
                succeeds!(
                    c,
                    f,
                    l,
                    "geometry::combo_box_fill",
                    super::geometry::combo_box_fill(&r, disabled)
                );
            }
            cx.update(|cx| {
                succeeds!(
                    c,
                    f,
                    l,
                    "variants::bordered_button",
                    super::variants::bordered_button(&r, cx)
                );
                succeeds!(
                    c,
                    f,
                    l,
                    "variants::toolbar_button",
                    super::variants::toolbar_button(&r, cx)
                );
                succeeds!(
                    c,
                    f,
                    l,
                    "variants::default_button",
                    super::variants::default_button(&r, cx)
                );
                succeeds!(c, f, l, "root::apply", super::apply(&r, cx));
            });
        }
    }
    let public: Vec<String> = mapped_fns("root", include_str!("mod.rs"))
        .into_iter()
        .chain(mapped_fns("base_layer", include_str!("base_layer.rs")))
        .chain(mapped_fns("variants", include_str!("variants.rs")))
        .chain(public_fns("geometry", include_str!("../geometry.rs")))
        .collect();
    let missing: Vec<&String> = public.iter().filter(|n| !called.contains(n)).collect();
    assert!(
        !public.is_empty() && missing.is_empty(),
        "next:: functions not called: {missing:?}"
    );
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
