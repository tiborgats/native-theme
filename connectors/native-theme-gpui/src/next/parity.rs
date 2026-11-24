//! Connector parity of the 0.8.0 work (docs/v0.8.0-plan.md, section 1, rule
//! 2; M0, step 4, and M4, step 1): what the connector hands to gpui, computed
//! from the 0.7.2 model by the connector's public mapping and from the 0.8.0
//! model by [`super`], is equal leaf by leaf: every field of the
//! gpui-component theme and of its stored configs, the scroll bar and the
//! resize handles, and every public builder of `src/geometry.rs` and
//! `src/variants.rs`.
//!
//! Presets covered: kde-breeze, adwaita, windows-11, macos-monterey,
//! macos-tahoe, material-3 and ios-13, light and dark ([`PRESETS`]). The
//! 0.7.2 side is resolved as the connector's tests resolve it
//! (`ResolutionContext::for_tests`), on the macOS presets and ios-13 at
//! Apple's font DPI ([`old_context`]), with no accessibility scaling; the
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
use gpui_component::theme::{
    Theme as GpuiTheme, ThemeColor, ThemeConfig, ThemeConfigColors, ThemeToken, ThemeTokens,
};
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
    ("ios-13", Preset::Ios13),
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

/// The presets whose 0.8.0 side leaves the dialog panel's sizes unstated
/// where 0.7.2 states them, as `geometry::dialog`'s entry in
/// [`DIFFERENCES`] says.
const DIALOG_SIZES_UNSTATED: &[&str] = &["kde-breeze"];

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
        presets: &[
            "kde-breeze",
            "adwaita",
            "windows-11",
            "macos-monterey",
            "macos-tahoe",
            "material-3",
            "ios-13",
        ],
        reason: "0.7.2's refinement sets no edge width, so the dialog keeps gpui-component's \
                 own, border_1 (gpui-component 0.10.0 src/dialog/dialog.rs:783), though \
                 0.7.2 states dialog.border.line_width; the 0.8.0 refinement sets the same \
                 width, dialog.panel.stroke.width, on every side (0 under material-3, which \
                 states it 0 wide; 0.5 px under the macOS presets and ios-13), as it sets the \
                 edge's colour; and on kde-breeze (DIALOG_SIZES_UNSTATED) 0.7.2 states \
                 dialog.min_width, min_height and max_height, which have no source (issue #4 \
                 Z2, docs/issues/0004_kde-leftovers-rationale.md §3, sizes that are not \
                 minimums or have no source; docs/v0.8.0-spec.md §5, notes to the table); the \
                 0.8.0 kde-breeze leaves dialog.panel's min_size and max_size[1] unstated, so \
                 the 0.8.0 refinement sets no `min_size` and no `max_size.height` and the \
                 dialog keeps gpui-component's own; every other field of the refinement is \
                 equal",
    },
    Difference {
        leaf: "geometry::popover",
        presets: &["kde-breeze"],
        reason: "0.7.2 rounds kde-breeze's popover by the uncited defaults.border.\
                 corner_radius_lg, 8; the 0.8.0 kde-breeze states popover.panel.corners 5, \
                 the radius qqc2-desktop-style's Popup rounds its background by \
                 (Kirigami.Units.cornerRadius, Popup.qml:46-47 at qqc2-desktop-style \
                 v6.31.0; docs/platform-facts.md §2.26), so the refinement's corner radii are \
                 that radius; every other field of the refinement is equal",
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
                 presets state list.highlight(HOVER).fill transparent, as AppKit draws a \
                 hovered table row as at rest (os-probe run 45198750893, macOS 15 and 26, \
                 `*.hovered.table_row` differs from the normal capture in 0 pixels, \
                 docs/evidence/os-probe/2026-11-23-run-45198750893/ on m2-fs4c; \
                 native-theme/tests/next_homes.toml, row list.hover_background, a listed \
                 difference), so list_hover and table_hover, their tokens and their stored \
                 config strings are that fill (row_hover_of); every other leaf of the theme is \
                 equal",
    },
    Difference {
        leaf: "geometry::group_box_content",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "0.7.2's macOS presets state no card.border.color, line_width or corner_radius \
                 (docs/platform-facts.md §2.26: NSBox's fill and border are not published), and \
                 0.7.2's validation fills them with transparent, 0 and 0 \
                 (native-theme/src/resolve/validate_helpers.rs), values no source gives; the \
                 0.8.0 macOS presets state NSBox's measured edge (os-probe runs 45198750893, \
                 45208260830 and 45217592058, `*.normal.box` `edge_*`, `corner_edge_*`, \
                 docs/evidence/os-probe/ on m2-fs4c; native-theme/tests/next_homes.toml, rows \
                 card.border.color, card.border.line_width and card.border.corner_radius, \
                 listed differences): on macos-monterey a 1 px line, #00000013 / #ffffff26, and \
                 corners of 5, on macos-tahoe no line and corners of 12; so the refinement's \
                 corner radius is card.container(NONE)'s, and its border width and colour the \
                 stroke's where one is drawn (card_edge_of); every other field of the \
                 refinement is equal",
    },
    Difference {
        leaf: "geometry::group_box_content",
        presets: &["ios-13"],
        reason: "0.7.2's ios-13.toml states no card.border.color, line_width or corner_radius \
                 (its [light.card] and [dark.card] tables are empty; docs/platform-facts.md \
                 §2.26 has no iOS column, and its macOS one publishes no box border), and \
                 0.7.2's validation \
                 fills them with transparent, 0 and 0 \
                 (native-theme/src/resolve/validate_helpers.rs), values no source gives; the \
                 0.8.0 ios-13 states the .insetGrouped cell's measured edge, no line and \
                 continuous corners of 10, drawn circular (ios-host run 45230061151, the card render, \
                 docs/evidence/ios-host/2026-11-23-run-45230061151/ on m2-fs4d; \
                 native-theme/tests/next_homes.toml, rows card.border.color, \
                 card.border.line_width and card.border.corner_radius, listed differences), so \
                 the refinement's corner radius is card.container(NONE)'s and its border 0.7.2's \
                 width 0 in transparent (card_edge_of); every other field of the refinement is \
                 equal",
    },
    Difference {
        leaf: "geometry::button_checked",
        presets: &["macos-tahoe", "ios-13"],
        reason: "0.7.2 states no button.checked_background or checked_text_color on these \
                 presets, so its refinement of a checked button is the pressed pair that \
                 stands in for them (crate::geometry::button_checked: on macos-tahoe the \
                 pressed label and no fill, on ios-13 the pressed fill and label); the 0.8.0 \
                 presets state the measured checked toggle \
                 (docs/v0.8.0-spec.md §10, rule 0a; docs/platform-facts.md §2.3; \
                 native-theme/tests/next_homes.toml, rows button.checked_background and \
                 button.checked_text_color, listed differences): on macos-tahoe the accent \
                 mixed with black (light) or white (dark) by 0.025 and a white label, on \
                 ios-13 the default tint at alpha 0x2e (light) or 0x40 (dark) with its label \
                 in the tint, so the refinement's background and text colour are those; every \
                 other field of the refinement is equal",
    },
    Difference {
        leaf: "geometry::accordion_hover",
        presets: &["kde-breeze", "windows-11"],
        reason: "0.7.2 states no expander.hover_background, so its refinement sets no \
                 background and the header keeps gpui-component's own; the 0.8.0 presets \
                 state the header's look under the pointer (spec §2, from_state; \
                 native-theme/tests/next_homes.toml, row expander.hover_background, a listed \
                 difference): on kde-breeze no fill, transparent, KCollapsibleGroupBox \
                 painting the arrow and the title only (docs/platform-facts.md §2.27, \
                 hover_background; kcollapsiblegroupbox.cpp:153-182 at kwidgetsaddons \
                 v6.30.0), on windows-11 the header's rest fill \
                 CardBackgroundFillColorDefault, which no PointerOver state sets \
                 (Expander.xaml:111, Expander_themeresources.xaml:29, :5, :122-165 at \
                 microsoft-ui-xaml 8463f45), so the refinement's background is that fill; it \
                 sets nothing else",
    },
    Difference {
        leaf: "geometry::accordion_hover",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "0.7.2's macos-monterey states expander.hover_background #e0e0e5 / #444446, \
                 uncited, and macos-tahoe states none (the refinement sets no background); the \
                 0.8.0 presets state no fill, transparent, at rest, hovered and pressed, as \
                 SwiftUI's DisclosureGroup draws none behind its title (os-probe run \
                 45214606115, macOS 15 and 26, `switch-label-macos-{15,26}/measure.toml` \
                 `*.hovered_disclosure_*` `label_changed_from_rest_px` 0, \
                 docs/evidence/os-probe/2026-11-23-run-45214606115/ on m2-fs4d; \
                 native-theme/tests/next_homes.toml, row expander.hover_background, a listed \
                 difference), so the refinement's background is that fill; it sets nothing \
                 else",
    },
    Difference {
        leaf: "geometry::accordion_hover",
        presets: &["material-3"],
        reason: "0.7.2's material-3 states expander.hover_background #1c1b1f14 / #e6e1e514, \
                 uncited and not v0_192's on-surface; the 0.8.0 preset states on-surface at \
                 the hover opacity 0.08, #1d1b2014 / #e6e0e914, a full-width row's hover layer, \
                 Material 3 having no expander (docs/platform-facts.md §2.27; \
                 _md-comp-list.scss:85-88, v0_192 at material-web cbd34a8; \
                 native-theme/tests/next_homes.toml, row expander.hover_background, a listed \
                 difference), so the refinement's background is that layer; it sets nothing \
                 else",
    },
    Difference {
        leaf: "variants::bordered_button",
        presets: &["adwaita", "windows-11"],
        reason: "0.7.2 states no button.hover_border_color, so the bordered variant gets no \
                 hover border (`None`) and the caller keeps gpui-component's Default variant, \
                 which draws the input token there (gpui-component 0.10.0 \
                 button/button.rs:1100-1102); the 0.8.0 presets state the rest outline for HOVER, \
                 where the platform keeps it (spec §2, from_state; \
                 native-theme/tests/next_homes.toml, row button.hover_border_color, a listed \
                 difference): adwaita, whose raised button's :hover sets only the background \
                 (_buttons.scss:28-42 at libadwaita 1.10.0), and windows-11, whose \
                 ButtonBorderBrushPointerOver is the rest brush (Button_themeresources.xaml:137, \
                 :136 at microsoft-ui-xaml 8463f45); so the hover outline is \
                 button.body(NONE)'s stroke colour",
    },
    Difference {
        leaf: "variants::bordered_button",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "0.7.2 states no button.hover_border_color, so the bordered variant gets no \
                 hover border (`None`) and the caller keeps gpui-component's Default variant; \
                 the 0.8.0 macOS presets state the push button's rest outline for HOVER, where \
                 hover draws nothing (spec §2, from_state; native-theme/tests/next_homes.toml, \
                 row button.hover_border_color, a listed difference): every hovered push \
                 button differs from its normal capture in 0 pixels \
                 (docs/facts/macos/state-models.toml on v0.8.0-rc1, hover not an axis; \
                 os-probe runs 41006842699, macOS 15, and 40969040283, macOS 26, \
                 button-pass/probe.toml `…hovered.regular-*.diff_from_normal`); so the hover \
                 outline is button.body(NONE)'s stroke colour, on macos-tahoe transparent, \
                 its push button drawing no stroke (docs/facts/macos/button.toml on \
                 v0.8.0-rc1 at 78ee0efd, `button.body.stroke` absent)",
    },
    Difference {
        leaf: "variants::bordered_button",
        presets: &["ios-13"],
        reason: "0.7.2 states no button.hover_border_color, so the bordered variant gets no \
                 hover border (`None`) and the caller keeps gpui-component's Default variant; \
                 the 0.8.0 ios-13 states WebKit's rest border for HOVER, which no :hover rule \
                 changes (html.css:889 at WebKit 31adf9d; spec §2, from_state; \
                 native-theme/tests/next_homes.toml, row button.hover_border_color, a listed \
                 difference); so the hover outline is button.body(NONE)'s stroke colour, \
                 systemBackground",
    },
    Difference {
        leaf: "variants::bordered_button",
        presets: &["material-3"],
        reason: "0.7.2 states no button.hover_border_color, so the bordered variant gets no \
                 hover border (`None`) and the caller keeps gpui-component's Default variant; \
                 the 0.8.0 material-3 states the push button's stroke for HOVER as none, the \
                 filled tonal button having no outline token in any state \
                 (_md-comp-filled-tonal-button.scss, its tokens :37-99, v0_192 at material-web \
                 cbd34a8; native-theme/tests/next_homes.toml, row button.hover_border_color, a \
                 listed difference), so the hover outline is transparent (crate::next's \
                 stroke colour of a stroke stated as none)",
    },
    Difference {
        leaf: BUTTON_STATES,
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "0.7.2's button.hover_background (#e0e0e5 / #444446) is uncited, its \
                 macos-monterey button.active_background (#d6d6db / #4e4e50) a soft option, and \
                 macos-tahoe leaves the latter unset (the connector's derived pressed layer); \
                 the 0.8.0 macOS presets state the rest fill for HOVER, where hover draws \
                 nothing (docs/facts/macos/state-models.toml on v0.8.0-rc1, hover not an axis; \
                 os-probe runs 41006842699, macOS 15, and 40969040283, macOS 26, \
                 button-pass/probe.toml `…hovered.regular-*.diff_from_normal` 0 pixels), and \
                 the measured pressed fill (docs/facts/macos/button.toml on v0.8.0-rc1 at \
                 78ee0efd, `button.body.fill` states [pressed]: macos-monterey #f0f0f0 / \
                 #ffffff5c, run 41006842699, macOS 15; macos-tahoe #0000002a / #ffffff2a, the \
                 same run, macOS 26, layers-key-push.toml \
                 `key.push.<appearance>.pressed.regular-plain.fill`); native-theme/tests/\
                 next_homes.toml, rows button.hover_background and button.active_background, \
                 listed differences. So button_hover and button_secondary_hover are the rest \
                 fill and button_active and button_secondary_active the pressed one, each the \
                 whole fill of its state (spec §9, step 2; crate::colors::assign_buttons), and \
                 secondary_hover and secondary_active, the flat button's home \
                 (button.body(HOVER; variant=Flat), button.body(PRESSED; variant=Flat)), which \
                 the same rules with no variant state, the same two; their tokens and their \
                 stored config strings; every other leaf of the theme is equal",
    },
    Difference {
        leaf: "variants::ghost_button",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "the ghost variant reads secondary_hover and secondary_active beside \
                 secondary_foreground, the flat button's home (button.body(HOVER; \
                 variant=Flat), button.body(PRESSED; variant=Flat)), which the macOS presets' \
                 button rules with no variant state as the rest fill (hover draws nothing on \
                 macOS) and the measured pressed fill \
                 (docs/facts/macos/state-models.toml and docs/facts/macos/button.toml on \
                 v0.8.0-rc1 at 78ee0efd; os-probe runs 41006842699 and 40969040283), where \
                 0.7.2 gives #e0e0e5 / #444446 and its soft option or derived layer; \
                 secondary_foreground is equal",
    },
    Difference {
        leaf: "variants::ghost_button",
        presets: &["kde-breeze"],
        reason: "the ghost variant reads secondary_hover and secondary_active beside \
                 secondary_foreground, the flat button's home (button.body(HOVER; \
                 variant=Flat), button.body(PRESSED; variant=Flat)), which the 0.8.0 \
                 kde-breeze states as Breeze's flat button draws it, hovered \
                 alphaColor(Highlight, 0), nothing, and pressed Highlight at Blend_Value 0.3 \
                 (breezehelper.cpp:688-691, :679-680 at breeze 6.8.2; \
                 docs/facts/kde/button.toml on v0.8.0-rc1 at 78ee0efd, row K1.24), where \
                 0.7.2 gives the push button's hovered and pressed fills to every variant; \
                 secondary_foreground is equal",
    },
    Difference {
        leaf: FLAT_HOVER,
        presets: &["kde-breeze"],
        reason: "0.7.2 has no flat button, so secondary_hover is the push button's hovered \
                 fill, button.hover_background; the 0.8.0 kde-breeze states the flat button's \
                 own, alphaColor(Highlight, 0), nothing (breezehelper.cpp:688-691 at breeze \
                 6.8.2; docs/facts/kde/button.toml on v0.8.0-rc1 at 78ee0efd, row K1.24), so \
                 the secondary_hover slot, its token and its stored config string are \
                 button.body(HOVER; variant=Flat)'s fill (flat_fill_of); every other leaf of \
                 the theme is equal",
    },
    Difference {
        leaf: FLAT_PRESSED,
        presets: &["kde-breeze"],
        reason: "0.7.2 has no flat button, so secondary_active is the push button's pressed \
                 fill, button.active_background; the 0.8.0 kde-breeze states the flat button's \
                 own, Highlight at Blend_Value 0.3, a layer (breezehelper.cpp:679-680 at breeze \
                 6.8.2; docs/facts/kde/button.toml on v0.8.0-rc1 at 78ee0efd, row K1.24), so \
                 the secondary_active slot, its token and its stored config string are \
                 button.body(PRESSED; variant=Flat)'s fill (flat_fill_of); every other leaf of \
                 the theme is equal",
    },
    Difference {
        leaf: "variants::ghost_button",
        presets: &["adwaita"],
        reason: "the ghost variant reads secondary_hover and secondary_active beside \
                 secondary_foreground, the flat button's home (button.body(HOVER; \
                 variant=Flat), button.body(PRESSED; variant=Flat)), which the 0.8.0 adwaita \
                 states as libadwaita's .flat button draws it, $hover_color and $active_color, \
                 the window text at 7 % and 16 % (_buttons.scss:250-256, _colors.scss:268-269 \
                 at libadwaita 1.10.0; docs/facts/gnome/button.toml on v0.8.0-rc1 at \
                 78ee0efd), where 0.7.2 gives the push button's hovered and pressed fills to \
                 every variant; secondary_foreground is equal",
    },
    Difference {
        leaf: FLAT_HOVER,
        presets: &["adwaita"],
        reason: "0.7.2 has no flat button, so secondary_hover is the push button's hovered \
                 fill, button.hover_background; the 0.8.0 adwaita states the flat button's \
                 own, $hover_color, the window text at 7 % (_buttons.scss:250-251, \
                 _colors.scss:268 at libadwaita 1.10.0; docs/facts/gnome/button.toml on \
                 v0.8.0-rc1 at 78ee0efd), so the secondary_hover slot, its token and its \
                 stored config string are button.body(HOVER; variant=Flat)'s fill \
                 (flat_fill_of); every other leaf of the theme is equal",
    },
    Difference {
        leaf: FLAT_PRESSED,
        presets: &["adwaita"],
        reason: "0.7.2 has no flat button, so secondary_active is the push button's pressed \
                 fill, button.active_background; the 0.8.0 adwaita states the flat button's \
                 own, $active_color, the window text at 16 % (_buttons.scss:255-256, \
                 _colors.scss:269 at libadwaita 1.10.0; docs/facts/gnome/button.toml on \
                 v0.8.0-rc1 at 78ee0efd), so the secondary_active slot, its token and its \
                 stored config string are button.body(PRESSED; variant=Flat)'s fill \
                 (flat_fill_of); every other leaf of the theme is equal",
    },
    Difference {
        leaf: "variants::ghost_button",
        presets: &["material-3"],
        reason: "the ghost variant reads secondary_hover and secondary_active beside \
                 secondary_foreground, the flat button's home (button.body(HOVER; \
                 variant=Flat), button.body(PRESSED; variant=Flat)), which the 0.8.0 \
                 material-3 states as M3's text button draws it, the state layer primary at \
                 the hover and the pressed state layer opacity, 0.08 and 0.12, over no \
                 container (_md-comp-button-text.scss:45-48, :57-60 at material-web 2.5.0), \
                 where 0.7.2 gives the push button's hovered and pressed fills to every \
                 variant; secondary_foreground is equal",
    },
    Difference {
        leaf: FLAT_HOVER,
        presets: &["material-3"],
        reason: "0.7.2 has no flat button, so secondary_hover is the push button's hovered \
                 fill, button.hover_background; the 0.8.0 material-3 states the text button's \
                 own, hovered-state-layer-color primary at hovered-state-layer-opacity 0.08 \
                 (_md-comp-button-text.scss:45-48 at material-web 2.5.0), so the \
                 secondary_hover slot, its token and its stored config string are \
                 button.body(HOVER; variant=Flat)'s fill (flat_fill_of); every other leaf of \
                 the theme is equal",
    },
    Difference {
        leaf: FLAT_PRESSED,
        presets: &["material-3"],
        reason: "0.7.2 has no flat button, so secondary_active is the push button's pressed \
                 fill, button.active_background; the 0.8.0 material-3 states the text \
                 button's own, pressed-state-layer-color primary at pressed-state-layer-opacity \
                 0.12 (_md-comp-button-text.scss:57-60 at material-web 2.5.0), so the \
                 secondary_active slot, its token and its stored config string are \
                 button.body(PRESSED; variant=Flat)'s fill (flat_fill_of); every other leaf of \
                 the theme is equal",
    },
    Difference {
        leaf: "variants::ghost_button",
        presets: &["windows-11"],
        reason: "the ghost variant reads secondary_hover and secondary_active beside \
                 secondary_foreground, the flat button's home (button.body(HOVER; \
                 variant=Flat), button.body(PRESSED; variant=Flat)), which the 0.8.0 \
                 windows-11 states as WinUI's subtle button draws it, hovered \
                 SubtleFillColorSecondary and pressed SubtleFillColorTertiary \
                 (Button_themeresources.xaml:18-19 at microsoft-ui-xaml 2.5.1; \
                 docs/facts/windows/button.toml on v0.8.0-rc1 at 78ee0efd, row W1-021), where \
                 0.7.2 gives the push button's hovered and pressed fills to every variant; \
                 secondary_foreground is equal",
    },
    Difference {
        leaf: FLAT_HOVER,
        presets: &["windows-11"],
        reason: "0.7.2 has no flat button, so secondary_hover is the push button's hovered \
                 fill, button.hover_background; the 0.8.0 windows-11 states the flat button's \
                 own, SubtleButtonBackgroundPointerOver = SubtleFillColorSecondary \
                 (Button_themeresources.xaml:18 at microsoft-ui-xaml 2.5.1; \
                 docs/facts/windows/button.toml on v0.8.0-rc1 at 78ee0efd, row W1-021), so the \
                 secondary_hover slot, its token and its stored config string are \
                 button.body(HOVER; variant=Flat)'s fill (flat_fill_of); every other leaf of \
                 the theme is equal",
    },
    Difference {
        leaf: FLAT_PRESSED,
        presets: &["windows-11"],
        reason: "0.7.2 has no flat button, so secondary_active is the push button's pressed \
                 fill, button.active_background; the 0.8.0 windows-11 states the flat button's \
                 own, SubtleButtonBackgroundPressed = SubtleFillColorTertiary \
                 (Button_themeresources.xaml:19 at microsoft-ui-xaml 2.5.1; \
                 docs/facts/windows/button.toml on v0.8.0-rc1 at 78ee0efd, row W1-021), so the \
                 secondary_active slot, its token and its stored config string are \
                 button.body(PRESSED; variant=Flat)'s fill (flat_fill_of); every other leaf of \
                 the theme is equal",
    },
    Difference {
        leaf: "geometry::button_checked",
        presets: &["macos-monterey"],
        reason: "neither model states a checked toggle on macos-monterey (a checked toggle in \
                 a key window of macOS 12-15 draws a gradient, pending U141 in \
                 docs/facts/macos/button.toml on v0.8.0-rc1 at 78ee0efd), so the refinement is \
                 the pressed pair that stands in for it (crate::geometry::button_checked); the \
                 0.8.0 preset states the measured pressed fill, #f0f0f0 / #ffffff5c, and the \
                 light scheme's measured pressed label, #242424 (the same file, \
                 `button.body.fill` and `button.label.text.fill` states [pressed]; os-probe run \
                 41006842699, macOS 15; native-theme/tests/next_homes.toml, rows \
                 button.active_background and button.active_text_color, listed differences), \
                 where 0.7.2 gives its soft option #d6d6db / #4e4e50 and the rest label; so the \
                 refinement's background and text colour are those of button.body(PRESSED) and \
                 button.label(PRESSED)",
    },
    Difference {
        leaf: "geometry::toolbar_button_checked",
        presets: &["macos-monterey", "macos-tahoe"],
        reason: "neither model states a toolbar button's checked or pressed fill on the macOS \
                 presets, so the refinement's fill is the push button's pressed one \
                 (crate::geometry::toolbar_button_checked); the 0.8.0 presets state the \
                 measured pressed fill (docs/facts/macos/button.toml on v0.8.0-rc1 at \
                 78ee0efd, `button.body.fill` states [pressed]: macos-monterey #f0f0f0 / \
                 #ffffff5c, os-probe run 41006842699, macOS 15; macos-tahoe #0000002a / \
                 #ffffff2a, the same run, macOS 26; native-theme/tests/next_homes.toml, row \
                 button.active_background, a listed difference), where 0.7.2 gives \
                 macos-monterey's soft option #d6d6db / #4e4e50 and on macos-tahoe none (no \
                 fill); so the refinement's background is button.body(PRESSED)'s fill; every \
                 other field of the refinement is equal",
    },
    Difference {
        leaf: "geometry::toolbar",
        presets: &["ios-13"],
        reason: "the 0.8.0 preset states the toolbar's button label, \
                 toolbar.button_label(NONE).color, in the tint, a plain bar button item's \
                 colour, #007aff / #0a84ff (docs/facts/ios/button.toml on v0.8.0-rc1 at \
                 78ee0efd, rows B-1, TB-4; native-theme/tests/next_homes.toml, row \
                 toolbar.font.color, a listed difference), where 0.7.2 gives the body text, \
                 #000000 / #ffffff; so the refinement's text colour is the tint; every other \
                 field of the refinement is equal",
    },
    Difference {
        leaf: "geometry::button",
        presets: &["macos-tahoe"],
        reason: "0.7.2's macos-tahoe outlines the push button in the default line (0.5 px, \
                 #00000019 / #ffffff19); the 0.8.0 preset states its stroke as none, as macOS 26 \
                 draws it (docs/facts/macos/button.toml on v0.8.0-rc1 at 78ee0efd, \
                 `button.body.stroke` absent; os-probe run 41006842699, layers-key-push.toml \
                 `key.push.<appearance>.normal.regular-plain.stroke` absent; \
                 native-theme/tests/next_homes.toml, rows button.border.color and \
                 button.border.line_width, listed differences), so the refinement's border is 0 \
                 wide and transparent (crate::next::stroke_color of a stroke stated as none); \
                 every other field of the refinement is equal",
    },
    Difference {
        leaf: "geometry::button",
        presets: &["ios-13"],
        reason: "0.7.2's ios-13 outlines the push button in the default border (0.5 px, \
                 #c6c6c8 / #38383a); the 0.8.0 preset states the border WebKit draws round an \
                 iOS button, `border: 1px solid -webkit-control-background`, systemBackground \
                 (html.css:889, RenderThemeIOS.mm:1027 at WebKit 31adf9d; \
                 native-theme/tests/next_homes.toml, rows button.border.color and \
                 button.border.line_width, listed differences), so the refinement's border is \
                 button.body(NONE)'s stroke, 1 px of #ffffff / #000000; every other field of the \
                 refinement is equal",
    },
    Difference {
        leaf: RESIZE_HANDLES,
        presets: &["kde-breeze"],
        reason: "0.7.2's kde-breeze takes the default border colour for the splitter's handle \
                 and its hover (#bcc0bf / #4d545b); the 0.8.0 preset states Breeze's \
                 separatorColor(), which drawSplitterControl fills in every state \
                 (breezestyle.cpp:7581-7587, breezehelper.cpp:329-332 at breeze f0b1d75; \
                 docs/platform-facts.md §2.17, hover_color; native-theme/tests/next_homes.toml, \
                 rows splitter.divider_color and splitter.hover_color, listed differences), \
                 #c6c8c9 / #4c4e51; so the handle and the active handle are the fills of \
                 splitter.handle(NONE) and splitter.handle(HOVER), in the compared \
                 ResizableTheme, in gpui-base's global and in the base overrides alike",
    },
    Difference {
        leaf: RESIZE_HANDLES,
        presets: &["adwaita"],
        reason: "0.7.2's adwaita takes the default border colour for the splitter's handle, \
                 opaque (#dcdcde in light), and states an opaque hover colour (#d5d5d5 in \
                 light, uncited); the 0.8.0 preset states the paned separator's translucent \
                 $border_color line, currentColor at 15 %, in every state, libadwaita having no \
                 :hover or :active rule for it (_paned.scss:1-36, _colors.scss:263, :257, :33, \
                 :96, :318 at libadwaita 1.10.0; docs/platform-facts.md §2.17, hover_color; \
                 native-theme/tests/next_homes.toml, rows splitter.divider_color and \
                 splitter.hover_color, listed differences), #0000061f / #ffffff26; so the \
                 handle and the active handle are the fills of splitter.handle(NONE) and \
                 splitter.handle(HOVER), in the compared ResizableTheme, in gpui-base's global \
                 and in the base overrides alike",
    },
    Difference {
        leaf: "geometry::select",
        presets: &["macos-tahoe"],
        reason: "0.7.2's macos-tahoe edges the pop-up button in the default line (0.5 px); the \
                 0.8.0 preset states its outline as none, as macOS 26 draws it (os-probe run \
                 45198750893, macOS 26, `*.normal.popup` `edge_*`: every edge pixel is the \
                 fill #00000013 / #ffffff13, docs/evidence/os-probe/2026-11-23-run-45198750893/ \
                 on m2-fs4c; native-theme/tests/next_homes.toml, rows combo_box.border.color \
                 and combo_box.border.line_width, listed differences), so the refinement's \
                 border is combo_box.field(NONE)'s stroke width, 0; the refinement carries no \
                 border colour in either model, and every other field of it is equal",
    },
    Difference {
        leaf: "geometry::select",
        presets: &["ios-13"],
        reason: "0.7.2's ios-13 edges the select in the default border width (0.5 px, \
                 uncited); the 0.8.0 preset states the border WebKit draws round an iOS select, \
                 `border: 1px solid -webkit-control-background` (html.css:1101, \
                 RenderThemeIOS.mm:1027 at WebKit 31adf9d; native-theme/tests/next_homes.toml, \
                 rows combo_box.border.color and combo_box.border.line_width, listed \
                 differences), so the refinement's border is combo_box.field(NONE)'s stroke \
                 width, 1 px; the refinement carries no border colour in either model, and \
                 every other field of it is equal",
    },
    Difference {
        leaf: "geometry::combobox",
        presets: &["macos-tahoe", "ios-13"],
        reason: "as geometry::select's on each: the combo box trigger's frame is the select's \
                 (crate::geometry::combobox), its border combo_box.field(NONE)'s stroke width",
    },
    Difference {
        leaf: "geometry::input",
        presets: &["ios-13"],
        reason: "0.7.2's ios-13 edges the text field in the default border width (0.5 px, \
                 uncited); the 0.8.0 preset states the border WebKit's iOS style sheet gives an \
                 input, `border: 1px solid -apple-system-secondary-label` (html.css:457-466 at \
                 WebKit 31adf9d; native-theme/tests/next_homes.toml, rows input.border.color \
                 and input.border.line_width, listed differences), so the refinement's border \
                 is input.field(NONE)'s stroke width, 1 px; the refinement carries no border \
                 colour in either model, and every other field of it is equal",
    },
    Difference {
        leaf: "geometry::text_area",
        presets: &["ios-13"],
        reason: "as geometry::input's: WebKit's rule gives a text area the same border \
                 (html.css:457-466; native-theme/tests/next_homes.toml, rows \
                 text_area.border.color and text_area.border.line_width, listed differences), \
                 so the refinement's border is text_area.field(NONE)'s stroke width, 1 px",
    },
    Difference {
        leaf: INPUT_OUTLINE,
        presets: &["ios-13"],
        reason: "0.7.2's ios-13 outlines the text field in the default border colour (the \
                 separator, #c6c6c8 / #38383a, uncited); the 0.8.0 preset states WebKit's \
                 `border: 1px solid -apple-system-secondary-label` (html.css:457-466 at WebKit \
                 31adf9d), secondaryLabel #3c3c4399 / #ebebf599 \
                 (LayoutTests/platform/ios-18/fast/css/ios/system-color-for-css-value-expected.txt:145, \
                 :147 at WebKit de320ba; native-theme/tests/next_homes.toml, row \
                 input.border.color, a listed difference), so the input slot, its token and \
                 its stored config string are input.field(NONE)'s stroke colour \
                 (input_outline_of); every other leaf of the theme is equal",
    },
    Difference {
        leaf: SLIDER_THUMB,
        presets: &["ios-13"],
        reason: "0.7.2's ios-13 leaves slider.thumb_color unset, so it is \
                 defaults.surface_color, #f2f2f7 / #1c1c1e; the 0.8.0 preset states UISlider's \
                 thumb as measured, white in both schemes (ios-host run 45230061151, iOS 18.6, \
                 `measure_lists.toml` `press1.\"<a>\".slider.normal` `fill` #ffffffff, \
                 docs/evidence/ios-host/2026-11-23-run-45230061151/ on m2-fs4d; \
                 native-theme/tests/next_homes.toml, row slider.thumb_color, a listed \
                 difference), so the slider_thumb slot, its token and its stored config string \
                 are slider.thumb(NONE)'s fill (slider_thumb_of); every other leaf of the theme \
                 is equal",
    },
];

/// The leaf of [`DIFFERENCES`]' entry for the text field's outline: the
/// `ThemeColor` slot `input`, its token and its stored config string,
/// compared by [`slot_as_old`] ([`STATED_SLOTS`]).
const INPUT_OUTLINE: &str = "to_theme: input";

/// The leaf of [`DIFFERENCES`]' entry for the slider's thumb: the
/// `ThemeColor` slot `slider_thumb`, its token and its stored config string,
/// compared by [`slot_as_old`] ([`STATED_SLOTS`]).
const SLIDER_THUMB: &str = "to_theme: slider_thumb";

/// The text field's outline as the connector reads it for the `input` slot
/// (`to_theme`): `input.field(NONE)`'s stroke colour, the form of
/// [`INPUT_OUTLINE`]'s entries; `None` where it is unstated.
fn input_outline_of(new: &native_theme::next::ResolvedTheme) -> Option<Hsla> {
    let field = new.input.field(native_theme::next::WidgetState::NONE);
    super::stroke_color(&field, "input.field(NONE).stroke.color").ok()
}

/// The leaf of [`DIFFERENCES`]' entries for the flat button's hovered fill:
/// the `ThemeColor` slot `secondary_hover`, its token and its stored config
/// string, compared by [`slot_as_old`] ([`STATED_SLOTS`]).
const FLAT_HOVER: &str = "to_theme: secondary_hover";

/// The leaf of [`DIFFERENCES`]' entries for the flat button's pressed fill:
/// the `ThemeColor` slot `secondary_active`, its token and its stored
/// config string, compared by [`slot_as_old`] ([`STATED_SLOTS`]).
const FLAT_PRESSED: &str = "to_theme: secondary_active";

/// The flat button's fill of `state` as the connector reads it for
/// `secondary_hover` and `secondary_active` (`super::flat_button_states`):
/// `button.body(<state>; variant=Flat)`'s fill where a rule states it for
/// the state; `None` where none does.
fn flat_fill_of(
    new: &native_theme::next::ResolvedTheme,
    state: native_theme::next::WidgetState,
) -> Option<Hsla> {
    let flat = native_theme::next::ButtonQuery::flat();
    super::state_fill_of(&new.button.body(flat.state(state)))
}

/// The flat button's hovered and pressed fills ([`flat_fill_of`]), which
/// `variants::ghost_button` reads; `None` where either is unstated.
fn flat_states_of(new: &native_theme::next::ResolvedTheme) -> Option<(Hsla, Hsla)> {
    use native_theme::next::WidgetState;
    Some((
        flat_fill_of(new, WidgetState::HOVER)?,
        flat_fill_of(new, WidgetState::PRESSED)?,
    ))
}

/// The slider's thumb as the connector reads it for the `slider_thumb` slot
/// (`next::colors`): `slider.thumb(NONE)`'s fill, the form of
/// [`SLIDER_THUMB`]'s entries; `None` where it is unstated.
fn slider_thumb_of(new: &native_theme::next::ResolvedTheme) -> Option<Hsla> {
    let thumb = new.slider.thumb(native_theme::next::WidgetState::NONE);
    super::fill(&thumb, "slider.thumb(NONE).fill").ok()
}

/// A `ThemeColor` slot whose listed difference is the colour the connector
/// reads at one home of the preset, compared by [`slot_as_old`]: the leaf of
/// its entries, what its failure names, that colour, and the slot's colour,
/// token and stored config string, read and copied.
struct StatedSlot {
    leaf: &'static str,
    what: &'static str,
    of: fn(&native_theme::next::ResolvedTheme) -> Option<Hsla>,
    color: fn(&ThemeColor) -> Hsla,
    token: fn(&ThemeTokens) -> ThemeToken,
    config: fn(&ThemeConfigColors) -> Option<SharedString>,
    /// Copies the slot's colour and token of the second theme into the first.
    copy: fn(&mut GpuiTheme, &GpuiTheme),
    /// Copies the slot's stored config string of the second into the first.
    copy_config: fn(&mut ThemeConfigColors, &ThemeConfigColors),
}

/// The slots [`DIFFERENCES`] lists as a colour stated at one home.
const STATED_SLOTS: &[StatedSlot] = &[
    StatedSlot {
        leaf: INPUT_OUTLINE,
        what: "the text field's stated outline",
        of: input_outline_of,
        color: |c| c.input,
        token: |t| t.input,
        config: |c| c.input.clone(),
        copy: |theme, from| {
            theme.colors.input = from.colors.input;
            theme.tokens.input = from.tokens.input;
        },
        copy_config: |config, from| config.input = from.input.clone(),
    },
    StatedSlot {
        leaf: SLIDER_THUMB,
        what: "the slider thumb's stated fill",
        of: slider_thumb_of,
        color: |c| c.slider_thumb,
        token: |t| t.slider_thumb,
        config: |c| c.slider_thumb.clone(),
        copy: |theme, from| {
            theme.colors.slider_thumb = from.colors.slider_thumb;
            theme.tokens.slider_thumb = from.tokens.slider_thumb;
        },
        copy_config: |config, from| config.slider_thumb = from.slider_thumb.clone(),
    },
    StatedSlot {
        leaf: FLAT_HOVER,
        what: "the flat button's stated hovered fill",
        of: |new| flat_fill_of(new, native_theme::next::WidgetState::HOVER),
        color: |c| c.secondary_hover,
        token: |t| t.secondary_hover,
        config: |c| c.secondary_hover.clone(),
        copy: |theme, from| {
            theme.colors.secondary_hover = from.colors.secondary_hover;
            theme.tokens.secondary_hover = from.tokens.secondary_hover;
        },
        copy_config: |config, from| config.secondary_hover = from.secondary_hover.clone(),
    },
    StatedSlot {
        leaf: FLAT_PRESSED,
        what: "the flat button's stated pressed fill",
        of: |new| flat_fill_of(new, native_theme::next::WidgetState::PRESSED),
        color: |c| c.secondary_active,
        token: |t| t.secondary_active,
        config: |c| c.secondary_active.clone(),
        copy: |theme, from| {
            theme.colors.secondary_active = from.colors.secondary_active;
            theme.tokens.secondary_active = from.tokens.secondary_active;
        },
        copy_config: |config, from| config.secondary_active = from.secondary_active.clone(),
    },
];

/// `theme` with its `slot` as `from`'s, where `theme` holds the colour the
/// preset states, `stated`, light then dark (the slot's `of`): the colour
/// and the token of the theme's mode, and the stored config string of each
/// scheme whose `stated` is given; the colours compared by their hex
/// strings, as an upstream rebuild of the theme from its stored config
/// (`Theme::change`) round-trips them. `None` where `theme` does not hold
/// it, or where it is `from`'s, the listed difference then describing
/// nothing.
fn slot_as_old(
    theme: &GpuiTheme,
    from: &GpuiTheme,
    stated: [Option<Hsla>; 2],
    slot: &StatedSlot,
) -> Option<GpuiTheme> {
    let hex = |c: Hsla| Some(SharedString::from(crate::colors::hsla_to_hex(c)));
    let [light, dark] = stated;
    let own = if theme.is_dark() { dark } else { light }?;
    let color = (slot.color)(&theme.colors);
    if hex(color) != hex(own)
        || hex((slot.color)(&from.colors)) == hex(own)
        || (slot.token)(&theme.tokens) != ThemeToken::from(color)
    {
        return None;
    }
    let mut patched = theme.clone();
    (slot.copy)(&mut patched, from);
    for (config, source, stated) in [
        (&mut patched.light_theme, &from.light_theme, light),
        (&mut patched.dark_theme, &from.dark_theme, dark),
    ] {
        let Some(stated) = stated else {
            continue;
        };
        if (slot.config)(&config.colors) != hex(stated) {
            return None;
        }
        let mut stored = (**config).clone();
        (slot.copy_config)(&mut stored.colors, &source.colors);
        *config = std::rc::Rc::new(stored);
    }
    Some(patched)
}

/// The leaf of [`DIFFERENCES`]' entries for the splitter's resize handles:
/// `ResizableTheme`'s `handle` and `active_handle`, compared by
/// [`resize_handles_of`], and the same two in what `apply` installs
/// (gpui-base's global and the base overrides).
const RESIZE_HANDLES: &str = "ResizableTheme: the handle and the active handle";

/// The splitter's handle at rest and hovered, `splitter.handle(NONE)`'s and
/// `splitter.handle(HOVER)`'s fills as the connector reads a fill
/// ([`super::fill_of`]), where both are stated, the hovered one by a state
/// rule: the form [`DIFFERENCES`]' [`RESIZE_HANDLES`] entries describe;
/// `None` where they are not.
fn resize_handles_of(new: &native_theme::next::ResolvedTheme) -> Option<(Hsla, Hsla)> {
    use native_theme::next::{PartStated, WidgetState};
    let s = &new.splitter;
    let hovered = s.handle(WidgetState::HOVER);
    if !hovered.from_state.contains(PartStated::FILL) {
        return None;
    }
    Some((
        super::fill_of(&s.handle(WidgetState::NONE))?,
        super::fill_of(&hovered)?,
    ))
}

/// The presets of [`DIFFERENCES`]' `geometry::button_checked` entries on
/// which no model states a checked toggle, so the pressed pair stands in.
const PRESSED_STANDS_IN: &[&str] = &["macos-monterey"];

/// The presets of [`DIFFERENCES`]' `geometry::button` entries whose push
/// button the 0.8.0 preset strokes as none.
const UNSTROKED_BUTTON: &[&str] = &["macos-tahoe"];

/// The presets of [`DIFFERENCES`]' `geometry::button` entries whose push
/// button the 0.8.0 preset strokes in an outline of its own, not 0.7.2's.
const RESTROKED_BUTTON: &[&str] = &["ios-13"];

/// The presets of [`DIFFERENCES`]' `variants::bordered_button` entries
/// whose hovered push button the 0.8.0 preset strokes as none.
const UNSTROKED_HOVER: &[&str] = &["material-3"];

/// The presets of [`DIFFERENCES`]' `geometry::accordion_hover` entries on
/// which 0.7.2 states an expander hover fill the 0.8.0 preset replaces.
const RESTATED_ACCORDION_HOVER: &[&str] = &["material-3", "macos-monterey"];

/// The leaf of [`DIFFERENCES`]' entry for the push button's and the flat
/// button's hovered and pressed fills: the `ThemeColor` slots of
/// [`STATE_SLOTS`], their tokens
/// and their stored config strings, compared by [`button_states_as`].
const BUTTON_STATES: &str = "to_theme: the push button's hovered and pressed slots";

/// One `ThemeColor` slot of [`BUTTON_STATES`]: its colour, its token and its
/// stored config string, and what the connector makes of the button states
/// for it (`super::button_states`, `super::flat_button_states`) where each
/// hovered fill is its rest fill: the flat button's for `secondary_hover`
/// and `secondary_active`, the push button's for the others.
struct StateSlot {
    color: fn(&mut ThemeColor) -> &mut Hsla,
    token: fn(&mut ThemeTokens) -> &mut ThemeToken,
    config: fn(&mut ThemeConfigColors) -> &mut Option<SharedString>,
    derived: fn(ButtonStates) -> Hsla,
}

/// The slots of [`BUTTON_STATES`].
const STATE_SLOTS: [StateSlot; 6] = [
    StateSlot {
        color: |c| &mut c.secondary_hover,
        token: |t| &mut t.secondary_hover,
        config: |s| &mut s.secondary_hover,
        derived: |s| s.flat.0,
    },
    StateSlot {
        color: |c| &mut c.secondary_active,
        token: |t| &mut t.secondary_active,
        config: |s| &mut s.secondary_active,
        derived: |s| s.flat.1,
    },
    StateSlot {
        color: |c| &mut c.button_hover,
        token: |t| &mut t.button_hover,
        config: |s| &mut s.button_hover,
        derived: |s| s.push.0,
    },
    StateSlot {
        color: |c| &mut c.button_active,
        token: |t| &mut t.button_active,
        config: |s| &mut s.button_active,
        derived: |s| s.push.1,
    },
    StateSlot {
        color: |c| &mut c.button_secondary_hover,
        token: |t| &mut t.button_secondary_hover,
        config: |s| &mut s.button_secondary_hover,
        derived: |s| s.push.0,
    },
    StateSlot {
        color: |c| &mut c.button_secondary_active,
        token: |t| &mut t.button_secondary_active,
        config: |s| &mut s.button_secondary_active,
        derived: |s| s.push.1,
    },
];

/// The hovered and pressed fills of the push button, `push`, and of the flat
/// button, `flat`, whose home the controls unfilled at rest read
/// (`secondary_hover`, `secondary_active`; `super::flat_button_states`).
#[derive(Clone, Copy)]
struct ButtonStates {
    push: (Hsla, Hsla),
    flat: (Hsla, Hsla),
}

/// The hovered and pressed fills of `new`'s push button and flat button,
/// where a rule states each for its state and each hovered one is its rest
/// fill (hover draws nothing): the form [`DIFFERENCES`]' [`BUTTON_STATES`]
/// entry describes; `None` where they are not.
fn button_states_of(new: &native_theme::next::ResolvedTheme) -> Option<ButtonStates> {
    use native_theme::next::{ButtonQuery, WidgetState};
    let b = &new.button;
    let of = |query: ButtonQuery| {
        let hover = super::state_fill_of(&b.body(query.state(WidgetState::HOVER)))?;
        let pressed = super::state_fill_of(&b.body(query.state(WidgetState::PRESSED)))?;
        (super::fill_of(&b.body(query)) == Some(hover)).then_some((hover, pressed))
    };
    Some(ButtonStates {
        push: of(ButtonQuery::new(WidgetState::NONE))?,
        flat: of(ButtonQuery::flat())?,
    })
}

/// The button states of each scheme, light and dark ([`button_states_of`]),
/// `None` where a scheme's are not given.
type SchemeStates = [Option<ButtonStates>; 2];

/// `theme` with the slots of [`BUTTON_STATES`] as 0.7.2 gives them: the
/// colours and tokens of `from`, a theme of the same scheme, and the stored
/// config strings of `stored` (light, dark), where `theme` holds what the
/// connector derives from `states`, each scheme's hovered fills (their rest
/// fills) and pressed fills ([`button_states_of`]); `None` where it does not.
/// A scheme without `states` keeps its stored config, which is to be
/// `stored`'s already. The colours are compared by their hex strings, which
/// an upstream rebuild of the theme from its stored config
/// (`Theme::change`) round-trips them through.
fn button_states_as(
    theme: &GpuiTheme,
    from: &GpuiTheme,
    stored: [&ThemeConfig; 2],
    states: SchemeStates,
) -> Option<GpuiTheme> {
    let hex = |c: Hsla| Some(SharedString::from(crate::colors::hsla_to_hex(c)));
    let [light, dark] = states;
    let own_states = if theme.is_dark() { dark } else { light }?;
    let c = &theme.colors;
    if [c.secondary, c.button, c.button_secondary]
        .iter()
        .any(|&rest| hex(rest) != hex(own_states.push.0))
    {
        return None;
    }
    let mut patched = theme.clone();
    let mut from = from.clone();
    for slot in &STATE_SLOTS {
        let derived = (slot.derived)(own_states);
        let color = (slot.color)(&mut patched.colors);
        let own = *color;
        if hex(own) != hex(derived) {
            return None;
        }
        *color = *(slot.color)(&mut from.colors);
        let token = (slot.token)(&mut patched.tokens);
        if *token != ThemeToken::from(own) {
            return None;
        }
        *token = *(slot.token)(&mut from.tokens);
    }
    for ((config, source), states) in [&mut patched.light_theme, &mut patched.dark_theme]
        .into_iter()
        .zip(stored)
        .zip([light, dark])
    {
        let mut colors = config.colors.clone();
        let mut source = source.colors.clone();
        for slot in &STATE_SLOTS {
            let (have, old) = ((slot.config)(&mut colors), (slot.config)(&mut source));
            match states {
                Some(states) if *have == hex((slot.derived)(states)) => {
                    *have = old.clone();
                }
                None if have == old => {}
                _ => return None,
            }
        }
        let mut stored = (**config).clone();
        stored.colors = colors;
        *config = std::rc::Rc::new(stored);
    }
    Some(patched)
}

/// The card's edge as `geometry::group_box_content`'s entries in
/// [`DIFFERENCES`] describe it: `card.container(NONE)`'s uniform corner
/// radius, on either curve, as gpui draws a continuous one circular
/// ([`super::radius_of`], spec §10 rule 2), and its uniform stroke's width
/// and colour where it draws one (`None` for a stroke stated as none, which
/// leaves 0.7.2's width 0 in transparent); `None` where the theme states
/// either otherwise.
fn card_edge_of(new: &native_theme::next::ResolvedTheme) -> Option<(f32, Option<(f32, Hsla)>)> {
    use native_theme::next::{CornerRadius, PartStated, Stroke, WidgetState};
    let c = new.card.container(WidgetState::NONE);
    if !c.stated.contains(PartStated::STROKE) || !c.stated.contains(PartStated::CORNERS) {
        return None;
    }
    let radius = match c.corners.radius {
        CornerRadius::Uniform(r) => r,
        _ => return None,
    };
    let border = match c.stroke {
        Stroke::None => None,
        Stroke::Uniform { width, color, .. } => Some((width, super::hsla(color))),
        _ => return None,
    };
    Some((radius, border))
}

/// The popover panel's radius, `popover.panel.corners`, where the theme
/// states one circular radius for every corner: the form of
/// `geometry::popover`'s entry in [`DIFFERENCES`]; `None` otherwise.
fn popover_radius_of(new: &native_theme::next::ResolvedTheme) -> Option<f32> {
    use native_theme::next::{CornerCurve, CornerRadius, PartStated};
    let panel = &new.popover.panel;
    if !panel.stated.contains(PartStated::CORNERS) {
        return None;
    }
    match (panel.corners.radius, panel.corners.curve) {
        (CornerRadius::Uniform(r), CornerCurve::Circular) => Some(r),
        _ => None,
    }
}

/// A hovered list row's fill where a rule states it for `HOVER`,
/// `list.highlight(HOVER).fill` as the connector reads a state's fill
/// ([`super::state_fill_of`]): the form of [`ROW_HOVER`]'s entries on a
/// preset that states it; `None` where none does, gpui-component's own then.
fn row_hover_of(new: &native_theme::next::ResolvedTheme) -> Option<Hsla> {
    super::state_fill_of(&new.list.highlight(native_theme::next::WidgetState::HOVER))
}

/// The leaf of [`DIFFERENCES`]' entry for a hovered list or table row: the
/// `ThemeColor` slots `list_hover` and `table_hover`, their tokens and their
/// stored config strings, compared by [`row_hover_as_list`].
const ROW_HOVER: &str = "to_theme: list_hover and table_hover";

/// `theme` with its hovered list and table row as 0.7.2 paints them where
/// the preset states no hover, the list's own fill (colour, token and the
/// stored config's string of the theme's mode), where `theme` holds the
/// hovered row the preset states, `stated` ([`row_hover_of`]), or where that
/// is `None` gpui-component's own, its accent at
/// [`super::UPSTREAM_LIST_HOVER_ACCENT_OPACITY`] ([`DIFFERENCES`]'
/// [`ROW_HOVER`] entry); `None` where it does not. The colours are compared
/// by their hex strings, which an upstream rebuild of the theme from its
/// stored config (`Theme::change`) round-trips them through.
fn row_hover_as_list(theme: &GpuiTheme, stated: Option<Hsla>) -> Option<GpuiTheme> {
    let hex = |c: Hsla| Some(SharedString::from(crate::colors::hsla_to_hex(c)));
    let own = hex(stated.unwrap_or_else(|| {
        theme
            .colors
            .accent
            .opacity(super::UPSTREAM_LIST_HOVER_ACCENT_OPACITY)
    }));
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
/// ([`MACOS_FONT_DPI`]), so a size in points is the size 0.7.2 gives there;
/// on ios-13 with the same, the model parity's `IOS_RESOLUTION_CONTEXT`.
pub(crate) fn old_context(preset: Preset) -> native_theme::ResolutionContext {
    match preset {
        Preset::MacosMonterey | Preset::MacosTahoe | Preset::Ios13 => {
            native_theme::ResolutionContext {
                font_dpi: MACOS_FONT_DPI,
                ..native_theme::ResolutionContext::for_tests()
            }
        }
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
            match row_hover_as_list(&new_theme, row_hover_of(new)) {
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
    // `variants::ghost_button` reads these three fields and nothing else.
    let ghost = |t: &GpuiTheme| {
        (
            t.colors.secondary_foreground,
            t.colors.secondary_hover,
            t.colors.secondary_active,
        )
    };
    let new_ghost = ghost(&new_theme);
    let new_theme = match report.listed(BUTTON_STATES) {
        Some(listed) => {
            report.count(BUTTON_STATES);
            let states = button_states_of(new);
            let states = if is_dark {
                [None, states]
            } else {
                [states, None]
            };
            let stored = [&*old_theme.light_theme, &*old_theme.dark_theme];
            match button_states_as(&new_theme, &old_theme, stored, states) {
                Some(patched) => patched,
                None => {
                    report.failures.push(format!(
                        "{}: {BUTTON_STATES}: the listed difference ({}) no longer describes \
                         it: 0.8.0 secondary_hover {:?}, secondary_active {:?}",
                        report.label,
                        listed.reason,
                        new_theme.colors.secondary_hover,
                        new_theme.colors.secondary_active
                    ));
                    new_theme
                }
            }
        }
        None => new_theme,
    };
    let mut new_theme = new_theme;
    for slot in STATED_SLOTS {
        let Some(listed) = report.listed(slot.leaf) else {
            continue;
        };
        report.count(slot.leaf);
        let stated = (slot.of)(new);
        let stated = if is_dark {
            [None, stated]
        } else {
            [stated, None]
        };
        if let Some(patched) = slot_as_old(&new_theme, &old_theme, stated, slot) {
            new_theme = patched;
        } else {
            report.failures.push(format!(
                "{}: {}: the listed difference ({}) no longer describes it: 0.7.2 {:?}, 0.8.0 \
                 {:?}",
                report.label,
                slot.leaf,
                listed.reason,
                (slot.color)(&old_theme.colors),
                (slot.color)(&new_theme.colors)
            ));
        }
    }
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
    let ghost_button = "variants::ghost_button";
    match report.listed(ghost_button) {
        Some(listed) => {
            report.count(ghost_button);
            let stated = flat_states_of(new)
                .map(|(hover, pressed)| (o.colors.secondary_foreground, hover, pressed));
            if stated != Some(new_ghost) {
                report.failures.push(format!(
                    "{}: {ghost_button}: the listed difference ({}) no longer describes it: \
                     0.7.2 {:?}, 0.8.0 {new_ghost:?}",
                    report.label,
                    listed.reason,
                    ghost(o)
                ));
            }
        }
        None => report.leaf(ghost_button, ghost(o), ghost(n)),
    }
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
    match report.listed(RESIZE_HANDLES) {
        Some(listed) => {
            report.count(RESIZE_HANDLES);
            let homes = resize_handles_of(new).map(|(handle, active)| (Some(handle), Some(active)));
            if o.handle == n.handle
                || o.active_handle == n.active_handle
                || homes != Some((n.handle, n.active_handle))
            {
                report.failures.push(format!(
                    "{}: {RESIZE_HANDLES}: the listed difference ({}) no longer describes it: \
                     0.7.2 {:?} {:?}, 0.8.0 {:?} {:?}",
                    report.label,
                    listed.reason,
                    o.handle,
                    o.active_handle,
                    n.handle,
                    n.active_handle
                ));
            }
        }
        None => {
            report.leaf("ResizableTheme.handle", o.handle, n.handle);
            report.leaf(
                "ResizableTheme.active_handle",
                o.active_handle,
                n.active_handle,
            );
        }
    }
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

/// Compare a field's framed refinement, `geometry::select`,
/// `geometry::combobox` (the frame `combo_box.field(NONE)`), `geometry::input`
/// or `geometry::text_area` (`input.field(NONE)`, `text_area.field(NONE)`):
/// where [`DIFFERENCES`] lists `leaf` for the preset, 0.7.2's refinement with
/// its border widths as the frame's stroke width at `width_home` gives them,
/// the widths differing from 0.7.2's.
fn compare_field_frame(
    report: &mut Report,
    leaf: &'static str,
    o: gpui::StyleRefinement,
    new_refinement: super::Mapped<gpui::StyleRefinement>,
    field: &native_theme::next::PartStyle,
    width_home: &'static str,
) {
    let Some(n) = report.mapped(leaf, new_refinement) else {
        return;
    };
    match report.listed(leaf) {
        Some(listed) => {
            use gpui::Styled as _;
            report.count(leaf);
            let stated = super::stroke_width(field, width_home).ok().map(|width| {
                gpui::StyleRefinement::default()
                    .border(px(width))
                    .border_widths
            });
            let mut corrected = o.clone();
            corrected.border_widths = n.border_widths.clone();
            if o.border_widths == n.border_widths
                || stated.as_ref() != Some(&n.border_widths)
                || corrected != n
            {
                report.failures.push(format!(
                    "{}: {leaf}: the listed difference ({}) no longer describes it: \
                     0.7.2 {o:?}, 0.8.0 {n:?}",
                    report.label, listed.reason
                ));
            }
        }
        None => report.leaf(leaf, o, n),
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
            Some(listed) if UNSTROKED_BUTTON.contains(&report.preset) => {
                use gpui::Styled as _;
                use native_theme::next::{PartStated, Stroke, WidgetState};
                report.count(button);
                let body = new.button.body(WidgetState::NONE);
                let none =
                    body.stated.contains(PartStated::STROKE) && matches!(body.stroke, Stroke::None);
                let mut corrected = o.clone();
                corrected.border_widths = n.border_widths.clone();
                corrected.border_color = n.border_color;
                if !none
                    || o.border_widths == n.border_widths
                    || n.border_widths
                        != gpui::StyleRefinement::default()
                            .border(px(0.0))
                            .border_widths
                    || n.border_color != super::stroke_color(&body, "button.body(NONE)").ok()
                    || corrected != n
                {
                    report.failures.push(format!(
                        "{}: {button}: the listed difference ({}) no longer describes \
                         it: 0.7.2 {o:?}, 0.8.0 {n:?}",
                        report.label, listed.reason
                    ));
                }
            }
            Some(listed) if RESTROKED_BUTTON.contains(&report.preset) => {
                use gpui::Styled as _;
                use native_theme::next::WidgetState;
                report.count(button);
                let body = new.button.body(WidgetState::NONE);
                let stated = super::stroke_width(&body, "button.body(NONE)")
                    .ok()
                    .map(|width| {
                        gpui::StyleRefinement::default()
                            .border(px(width))
                            .border_widths
                    });
                let mut corrected = o.clone();
                corrected.border_widths = n.border_widths.clone();
                corrected.border_color = n.border_color;
                if o.border_widths == n.border_widths
                    || o.border_color == n.border_color
                    || stated.as_ref() != Some(&n.border_widths)
                    || n.border_color != super::stroke_color(&body, "button.body(NONE)").ok()
                    || corrected != n
                {
                    report.failures.push(format!(
                        "{}: {button}: the listed difference ({}) no longer describes \
                         it: 0.7.2 {o:?}, 0.8.0 {n:?}",
                        report.label, listed.reason
                    ));
                }
            }
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
    let button_checked = "geometry::button_checked";
    match report.listed(button_checked) {
        Some(listed) if PRESSED_STANDS_IN.contains(&report.preset) => {
            if let Some(n) = report.mapped(button_checked, super::builders::button_checked(new)) {
                use native_theme::next::WidgetState;
                report.count(button_checked);
                let o = crate::geometry::button_checked(Native::unscaled(old));
                let b = &new.button;
                let unstated = old.button.checked_background.is_none()
                    && old.button.checked_text_color.is_none()
                    && super::state_fill_of(&b.body(WidgetState::CHECKED)).is_none()
                    && super::state_text_color_of(&b.label(WidgetState::CHECKED)).is_none();
                let pressed = super::text_color_of(&b.label(WidgetState::PRESSED)).map(|text| {
                    crate::geometry::button_checked_from(
                        None,
                        text,
                        None,
                        super::state_fill_of(&b.body(WidgetState::PRESSED)),
                    )
                });
                if !unstated || n.background.is_none() || pressed.as_ref() != Some(&n) {
                    report.failures.push(format!(
                        "{}: {button_checked}: the listed difference ({}) no longer \
                         describes it: 0.7.2 {o:?}, 0.8.0 {n:?}",
                        report.label, listed.reason
                    ));
                }
            }
        }
        Some(listed) => {
            if let Some(n) = report.mapped(button_checked, super::builders::button_checked(new)) {
                use native_theme::next::{PartStated, TextStated, WidgetState};
                report.count(button_checked);
                let o = crate::geometry::button_checked(Native::unscaled(old));
                let mut corrected = o.clone();
                corrected.background = n.background.clone();
                corrected.text.color = n.text.color;
                let unstated = old.button.checked_background.is_none()
                    && old.button.checked_text_color.is_none();
                let stated = new
                    .button
                    .body(WidgetState::CHECKED)
                    .from_state
                    .contains(PartStated::FILL)
                    && new
                        .button
                        .label(WidgetState::CHECKED)
                        .from_state
                        .contains(TextStated::COLOR);
                if !unstated || !stated || corrected != n {
                    report.failures.push(format!(
                        "{}: {button_checked}: the listed difference ({}) no longer \
                         describes it: 0.7.2 {o:?}, 0.8.0 {n:?}",
                        report.label, listed.reason
                    ));
                }
            }
        }
        None => builder!(report, old, new, button_checked),
    }
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
    let none = native_theme::next::WidgetState::NONE;
    compare_field_frame(
        report,
        "geometry::input",
        crate::geometry::input(Native::unscaled(old)),
        super::builders::input(new),
        &new.input.field(none),
        "input.field(NONE).stroke.width",
    );
    compare_field_frame(
        report,
        "geometry::text_area",
        crate::geometry::text_area(Native::unscaled(old)),
        super::builders::text_area(new),
        &new.text_area.field(none),
        "text_area.field(NONE).stroke.width",
    );
    builder!(report, old, new, input_height);
    let combo_box = new.combo_box.field(none);
    compare_field_frame(
        report,
        "geometry::select",
        crate::geometry::select(Native::unscaled(old)),
        super::builders::select(new),
        &combo_box,
        "combo_box.field(NONE).stroke.width",
    );
    compare_field_frame(
        report,
        "geometry::combobox",
        crate::geometry::combobox(Native::unscaled(old)),
        super::builders::combobox(new),
        &combo_box,
        "combo_box.field(NONE).stroke.width",
    );
    fill_builder!(report, old, new, input_fill);
    fill_builder!(report, old, new, combo_box_fill);
    builder!(report, old, new, menu_item);
    let popover = "geometry::popover";
    match report.listed(popover) {
        Some(listed) => {
            if let Some(n) = report.mapped(popover, super::builders::popover(new)) {
                report.count(popover);
                let o = crate::geometry::popover(Native::unscaled(old));
                let expected = popover_radius_of(new).map(|radius| {
                    let mut e = o.clone();
                    e.corner_radii =
                        gpui::Styled::rounded(gpui::StyleRefinement::default(), px(radius))
                            .corner_radii;
                    e
                });
                if expected.as_ref() == Some(&o) || expected != Some(n.clone()) {
                    report.failures.push(format!(
                        "{}: {popover}: the listed difference ({}) no longer describes \
                         it: 0.7.2 {o:?}, 0.8.0 {n:?}",
                        report.label, listed.reason
                    ));
                }
            }
        }
        None => builder!(report, old, new, popover),
    }
    let dialog = "geometry::dialog";
    match report.listed(dialog) {
        Some(listed) => {
            if let Some(n) = report.mapped(dialog, super::builders::dialog(new)) {
                report.count(dialog);
                let o = crate::geometry::dialog(Native::unscaled(old));
                let mut corrected =
                    gpui::Styled::border(o.clone(), px(old.dialog.border.line_width));
                let sizes_unstated = DIALOG_SIZES_UNSTATED.contains(&report.preset);
                if sizes_unstated {
                    corrected.min_size.width = None;
                    corrected.min_size.height = None;
                    corrected.max_size.height = None;
                }
                let stated = o.min_size.width.is_some()
                    && o.min_size.height.is_some()
                    && o.max_size.height.is_some();
                if o.border_widths.top.is_some() || (sizes_unstated && !stated) || corrected != n {
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
                let invented = o.border_widths.top == Some(px(0.0).into())
                    && o.border_color == Some(Hsla::transparent_black());
                let expected = card_edge_of(new).map(|(radius, border)| {
                    use gpui::Styled as _;
                    let mut e = o.clone();
                    e.corner_radii = gpui::StyleRefinement::default()
                        .rounded(px(radius))
                        .corner_radii;
                    if let Some((width, color)) = border {
                        let s = gpui::StyleRefinement::default()
                            .border(px(width))
                            .border_color(color);
                        e.border_widths = s.border_widths;
                        e.border_color = s.border_color;
                    }
                    e
                });
                if !invented || expected.as_ref() == Some(&o) || expected != Some(n.clone()) {
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
    let accordion_hover = "geometry::accordion_hover";
    match report.listed(accordion_hover) {
        Some(listed) => {
            if let Some(n) = report.mapped(accordion_hover, super::builders::accordion_hover(new)) {
                use native_theme::next::WidgetState;
                report.count(accordion_hover);
                let o = crate::geometry::accordion_hover(Native::unscaled(old));
                let stated = super::state_fill_of(&new.expander.header(WidgetState::HOVER));
                let restated = RESTATED_ACCORDION_HOVER.contains(&report.preset);
                if old.expander.hover_background.is_some() != restated
                    || n == o
                    || stated.is_none()
                    || n != crate::geometry::accordion_hover_from(stated)
                {
                    report.failures.push(format!(
                        "{}: {accordion_hover}: the listed difference ({}) no longer \
                         describes it: 0.7.2 {o:?}, 0.8.0 {n:?}",
                        report.label, listed.reason
                    ));
                }
            }
        }
        None => builder!(report, old, new, accordion_hover),
    }
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
    let toolbar = "geometry::toolbar";
    match report.listed(toolbar) {
        Some(listed) => {
            if let Some(n) = report.mapped(toolbar, super::builders::toolbar(new)) {
                use native_theme::next::{TextStated, WidgetState};
                report.count(toolbar);
                let o = crate::geometry::toolbar(Native::unscaled(old));
                let label = new.toolbar.button_label(WidgetState::NONE);
                let stated = label
                    .stated
                    .contains(TextStated::COLOR)
                    .then(|| super::hsla(label.color));
                let mut corrected = o.clone();
                corrected.text.color = n.text.color;
                if stated.is_none() || n.text.color != stated || corrected != n || o == n {
                    report.failures.push(format!(
                        "{}: {toolbar}: the listed difference ({}) no longer describes it: \
                         0.7.2 {o:?}, 0.8.0 {n:?}",
                        report.label, listed.reason
                    ));
                }
            }
        }
        None => builder!(report, old, new, toolbar),
    }
    builder!(report, old, new, icon_size_toolbar);
    builder!(report, old, new, toolbar_button_rest);
    let toolbar_checked = "geometry::toolbar_button_checked";
    match report.listed(toolbar_checked) {
        Some(listed) => {
            if let Some(n) = report.mapped(
                toolbar_checked,
                super::builders::toolbar_button_checked(new),
            ) {
                use gpui::Styled as _;
                use native_theme::next::WidgetState;
                report.count(toolbar_checked);
                let o = crate::geometry::toolbar_button_checked(Native::unscaled(old));
                let t = &new.toolbar;
                let unstated = super::state_fill_of(&t.button(WidgetState::CHECKED)).is_none()
                    && super::state_fill_of(&t.button(WidgetState::PRESSED)).is_none();
                let pressed = super::state_fill_of(&new.button.body(WidgetState::PRESSED));
                let mut corrected = o.clone();
                corrected.background = n.background.clone();
                let filled =
                    pressed.map(|fill| gpui::StyleRefinement::default().bg(fill).background);
                if !unstated || filled != Some(n.background.clone()) || corrected != n {
                    report.failures.push(format!(
                        "{}: {toolbar_checked}: the listed difference ({}) no longer \
                         describes it: 0.7.2 {o:?}, 0.8.0 {n:?}",
                        report.label, listed.reason
                    ));
                }
            }
        }
        None => builder!(report, old, new, toolbar_button_checked),
    }
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
    let bordered = "variants::bordered_button";
    match report.listed(bordered) {
        Some(listed) => {
            if let Some(n) = report.mapped(bordered, super::builders::hover_border(new)) {
                use native_theme::next::WidgetState;
                report.count(bordered);
                use native_theme::next::{PartStated, Stroke};
                let o = crate::variants::hover_border(Native::unscaled(old));
                // The outline the entry names: the hovered body's, stated as none, or the rest one.
                let hovered = new.button.body(WidgetState::HOVER);
                let named = if UNSTROKED_HOVER.contains(&report.preset) {
                    (hovered.from_state.contains(PartStated::STROKE)
                        && matches!(hovered.stroke, Stroke::None))
                    .then(|| super::stroke_color(&hovered, "button.body(HOVER)").ok())
                    .flatten()
                } else {
                    super::stroke_color(&new.button.body(WidgetState::NONE), "button.body(NONE)")
                        .ok()
                };
                if o.is_some() || n.is_none() || n != named {
                    report.failures.push(format!(
                        "{}: {bordered}: the listed difference ({}) no longer describes \
                         it: 0.7.2 {o:?}, 0.8.0 {n:?}",
                        report.label, listed.reason
                    ));
                }
            }
        }
        None => compare_mapped(
            report,
            bordered,
            crate::variants::hover_border(Native::unscaled(old)),
            super::builders::hover_border(new),
        ),
    }
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
fn the_covered_presets_are_every_platform_preset() {
    let covered: Vec<Preset> = PRESETS.iter().map(|&(_, preset)| preset).collect();
    let misnamed: Vec<&str> = PRESETS
        .iter()
        .filter(|&&(key, preset)| key != preset.as_str())
        .map(|&(key, _)| key)
        .collect();
    let missing: Vec<&Preset> = Preset::ALL
        .iter()
        .filter(|p| !covered.contains(p))
        .collect();
    assert!(
        covered.len() == Preset::ALL.len() && missing.is_empty() && misnamed.is_empty(),
        "{} presets covered, {} platform presets; not covered: {missing:?}; keys that are not \
         their preset's name: {misnamed:?}",
        covered.len(),
        Preset::ALL.len()
    );
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
/// by `next::tests`, against [`crate::apply`]. The part and text reads
/// (`fill_of` … `state_text_color_of`, `padding_of`) take a part of the model,
/// not a theme, and have no 0.7.2 function of their own: the builders and
/// colours of `next` read through them (`fill`, `stroke_color`, …, the
/// `Mapped` forms), so they are compared through those; the `Mapped` form
/// of a `state_` read (`state_fill`, …) reads through it too.
/// `label_font_unmapped` takes a text style and is `uncoloured_font`'s
/// check, which the builders read through; `body_text_unmapped` is
/// `ThemeSource::body_text`'s check, compared through it (`widgets`'s
/// parity, `ThemeSource::body_text`).
const NOT_ROOT_ACCESSORS: &[&str] = &[
    "root::label_font_unmapped",
    "root::body_text_unmapped",
    "root::to_theme",
    "root::apply",
    "root::fill_of",
    "root::stroke_color_of",
    "root::stroke_width_of",
    "root::radius_of",
    "root::padding_of",
    "root::text_color_of",
    "root::state_fill_of",
    "root::state_stroke_color_of",
    "root::state_text_color_of",
];

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
/// hold gpui-component's own; where `button_states` is given, the 0.7.2
/// side's styled theme at the same point and the 0.8.0 preset's hovered and
/// pressed fills of each scheme ([`button_states_of`]), with the button
/// state slots of [`BUTTON_STATES`] as that theme's ([`button_states_as`]),
/// or as a failure where they are not what the connector derives from the
/// fills; for each of `stated`, a slot of [`STATED_SLOTS`] with that theme
/// and the 0.8.0 preset's colour of each scheme for it, with the slot as
/// that theme's ([`slot_as_old`]), or as a failure where it is not the
/// stated colour. Where `handles` is given, the resize handles of gpui-base's global
/// and of the base overrides are given as 0.7.2's of the same scheme, or as
/// a failure where they are not the 0.8.0 preset's homes
/// ([`resize_handles_of`], [`DIFFERENCES`]' [`RESIZE_HANDLES`] entries) or
/// are 0.7.2's. The styled theme itself is returned beside.
fn installed(
    cx: &gpui::App,
    row_hover_listed: Option<[Option<Hsla>; 2]>,
    button_states: Option<(&GpuiTheme, SchemeStates)>,
    stated: &[StatedPatch<'_>],
    handles: Option<SchemeHandles>,
) -> (Vec<(&'static str, String)>, GpuiTheme) {
    let styled = GpuiTheme::global(cx);
    let is_dark = styled.is_dark();
    let row_hover = if let Some([light, dark]) = row_hover_listed {
        row_hover_as_list(styled, if is_dark { dark } else { light })
            .ok_or_else(|| format!("{ROW_HOVER}: not gpui-component's own hovered row"))
    } else {
        Ok(styled.clone())
    };
    let patched = row_hover.and_then(|theme| match button_states {
        Some((from, states)) => {
            let stored = [&*from.light_theme, &*from.dark_theme];
            button_states_as(&theme, from, stored, states).ok_or_else(|| {
                format!("{BUTTON_STATES}: not what the connector derives from the preset")
            })
        }
        None => Ok(theme),
    });
    let patched = stated
        .iter()
        .fold(patched, |patched, (slot, from, schemes)| {
            patched.and_then(|theme| {
                slot_as_old(&theme, from, *schemes, slot)
                    .ok_or_else(|| format!("{}: not {}", slot.leaf, slot.what))
            })
        });
    let styled_debug = patched.map_or_else(|failure| failure, |theme| format!("{theme:?}"));
    let base = gpui_base::Theme::global(cx);
    let as_old = |handle: Option<Hsla>, active: Option<Hsla>| match handles {
        None => format!("{handle:?} {active:?}"),
        Some([light, dark]) => {
            let (old, homes) = if is_dark { dark } else { light };
            let installed = (handle, active);
            if installed != old && homes.map(|(h, a)| (Some(h), Some(a))) == Some(installed) {
                format!("{:?} {:?}", old.0, old.1)
            } else {
                format!(
                    "{RESIZE_HANDLES}: {handle:?} {active:?}, not the homes' {homes:?} beside \
                     0.7.2's {old:?}"
                )
            }
        }
    };
    let nt = cx.try_global::<crate::NativeTheme>();
    let overrides = nt
        .and_then(|nt| crate::base_overrides_for(nt, is_dark, Some(styled)))
        .map(|(g, r)| format!("{g:?} {}", as_old(r.handle, r.active_handle)));
    let band = nt
        .and_then(|nt| nt.source(is_dark))
        .map(crate::Variant::keyboard_focus_band);
    let parts = vec![
        ("styled theme", styled_debug),
        (
            "base handles",
            as_old(base.resizable.handle, base.resizable.active_handle),
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
    ];
    (parts, styled.clone())
}

/// What [`apply_both`] gives for each point: the installed parts
/// ([`installed`]) and the styled theme.
type Installed = [(Vec<(&'static str, String)>, GpuiTheme); 2];

/// The resize handles of each scheme, light and dark: 0.7.2's
/// (`crate::base_layer::resizable_theme`'s `handle` and `active_handle`)
/// and the 0.8.0 preset's homes ([`resize_handles_of`]).
type SchemeHandles = [((Option<Hsla>, Option<Hsla>), Option<(Hsla, Hsla)>); 2];

/// A slot of [`STATED_SLOTS`] to patch in what is installed ([`installed`]):
/// the 0.7.2 side's styled theme at the same point and the 0.8.0 preset's
/// colour for the slot in each scheme, light then dark.
type StatedPatch<'a> = (&'static StatedSlot, &'a GpuiTheme, [Option<Hsla>; 2]);

/// The two schemes of `key` (0.7.2) or `preset` (0.8.0) applied, light then
/// dark, with no variant stored before: what is installed after the calls,
/// and after an upstream switch to light mode (`Theme::change`), which the
/// connector's observer answers from the stored variants. `old` is `None`
/// for the 0.7.2 side, and for the 0.8.0 side what the 0.7.2 side installed,
/// whose styled themes the push button's hovered and pressed slots are
/// patched from where [`DIFFERENCES`]' [`BUTTON_STATES`] entry names the
/// preset, and each slot of [`STATED_SLOTS`] where an entry of its leaf does.
fn apply_both(
    cx: &mut gpui::TestAppContext,
    key: &str,
    preset: Preset,
    old: Option<&Installed>,
) -> Result<Installed, String> {
    let next = old.is_some();
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
    let row_hover_listed = (next
        && DIFFERENCES
            .iter()
            .any(|d| d.leaf == ROW_HOVER && d.presets.contains(&key)))
    .then(|| {
        let mut schemes = themes.iter().map(|(_, (_, _, _, new))| row_hover_of(new));
        [schemes.next().flatten(), schemes.next().flatten()]
    });
    let states = {
        let mut schemes = themes
            .iter()
            .map(|(_, (_, _, _, new))| button_states_of(new));
        [schemes.next().flatten(), schemes.next().flatten()]
    };
    let button_states = |at: usize| {
        old.filter(|_| {
            DIFFERENCES
                .iter()
                .any(|d| d.leaf == BUTTON_STATES && d.presets.contains(&key))
        })
        .and_then(|old| old.get(at))
        .map(|(_, from)| (from, states))
    };
    let stated = |at: usize| -> Vec<StatedPatch<'_>> {
        let Some((_, from)) = old.and_then(|old| old.get(at)) else {
            return Vec::new();
        };
        STATED_SLOTS
            .iter()
            .filter(|slot| {
                DIFFERENCES
                    .iter()
                    .any(|d| d.leaf == slot.leaf && d.presets.contains(&key))
            })
            .map(|slot| {
                let mut schemes = themes.iter().map(|(_, (_, _, _, new))| (slot.of)(new));
                (
                    slot,
                    from,
                    [schemes.next().flatten(), schemes.next().flatten()],
                )
            })
            .collect()
    };
    let handles_listed = next
        && DIFFERENCES
            .iter()
            .any(|d| d.leaf == RESIZE_HANDLES && d.presets.contains(&key));
    let handles = handles_listed.then(|| {
        let mut schemes = themes.iter().map(|(_, (old, _, _, new))| {
            let r = crate::base_layer::resizable_theme(old);
            ((r.handle, r.active_handle), resize_handles_of(new))
        });
        [schemes.next(), schemes.next()]
    });
    let handles = match handles {
        Some([Some(light), Some(dark)]) => Some([light, dark]),
        Some(_) => return Err(format!("{key}: {RESIZE_HANDLES}: a scheme is missing")),
        None => None,
    };
    let after =
        cx.update(|cx| installed(cx, row_hover_listed, button_states(0), &stated(0), handles));
    cx.update(|cx| GpuiTheme::change(gpui_component::theme::ThemeMode::Light, None, cx));
    let switched =
        cx.update(|cx| installed(cx, row_hover_listed, button_states(1), &stated(1), handles));
    Ok([after, switched])
}

/// `next::apply` installs what `apply` installs, on every covered preset:
/// after both schemes are applied, and after upstream switches the mode.
#[gpui::test]
fn next_apply_installs_what_apply_installs(cx: &mut gpui::TestAppContext) {
    let mut failures = Vec::new();
    for &(key, preset) in PRESETS {
        let both = apply_both(cx, key, preset, None)
            .and_then(|old| apply_both(cx, key, preset, Some(&old)).map(|new| (old, new)));
        match both {
            Ok((old, new)) => {
                for (when, ((o, _), (n, _))) in ["after apply", "after Theme::change"]
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
