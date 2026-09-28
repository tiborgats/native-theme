//! The two axes of the atlas — a widget's *content* style (`Role`, `RoleVariant`) and a
//! container's *chrome* (`Surface`, `PanelSide`) — and the text-scale roles (spec §4.4, §4.7).

/// A widget role: the *content* style for one kind of widget.
///
/// **Exactly one variant per widget field of [`ResolvedTheme`](crate::ResolvedTheme)**
/// (`native-theme/src/model/resolved.rs:164-212`), in declaration order. That is the whole
/// rule. When native-theme grows a widget, this enum grows one variant and `mapping.toml`
/// grows one section; nothing else in this crate's API changes.
///
/// Role names are **native-theme's vocabulary**: each one is a `ResolvedTheme` field name.
/// [`Surface`] is the other axis — container *chrome* — and its names are **attachment
/// points**, not egui type names: egui 0.36.2 has no `Dialog`, `Popover` or `Card` type at all
/// (a recursive grep over `egui-0.36.2/src` returns one hit, `egui/src/viewport.rs:998`, an
/// unrelated window-type hint), and five of `Surface`'s eight names — `Window`, `Dialog`,
/// `Popover`, `Tooltip`, `Card` — are spelled identically to a `Role` variant. The two axes are
/// separate because they track two different moving sides, not because they use different
/// vocabularies.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Role {
    /// The window: `ResolvedTheme::window`.
    Window,
    /// The button: `ResolvedTheme::button`.
    Button,
    /// The input: `ResolvedTheme::input`.
    Input,
    /// The checkbox: `ResolvedTheme::checkbox`.
    Checkbox,
    /// The menu: `ResolvedTheme::menu`.
    Menu,
    /// The tooltip: `ResolvedTheme::tooltip`.
    Tooltip,
    /// The scrollbar: `ResolvedTheme::scrollbar`.
    Scrollbar,
    /// The slider: `ResolvedTheme::slider`.
    Slider,
    /// The progress bar: `ResolvedTheme::progress_bar`.
    ProgressBar,
    /// The tab: `ResolvedTheme::tab`.
    Tab,
    /// The sidebar: `ResolvedTheme::sidebar`.
    Sidebar,
    /// The toolbar: `ResolvedTheme::toolbar`.
    Toolbar,
    /// The status bar: `ResolvedTheme::status_bar`.
    StatusBar,
    /// The list: `ResolvedTheme::list`.
    List,
    /// The popover: `ResolvedTheme::popover`.
    Popover,
    /// The splitter: `ResolvedTheme::splitter`.
    Splitter,
    /// The separator: `ResolvedTheme::separator`.
    Separator,
    /// The switch: `ResolvedTheme::switch`.
    Switch,
    /// The dialog: `ResolvedTheme::dialog`.
    Dialog,
    /// The spinner: `ResolvedTheme::spinner`.
    Spinner,
    /// The combo box: `ResolvedTheme::combo_box`.
    ComboBox,
    /// The segmented control: `ResolvedTheme::segmented_control`.
    SegmentedControl,
    /// The card: `ResolvedTheme::card`.
    Card,
    /// The expander: `ResolvedTheme::expander`.
    Expander,
    /// The link: `ResolvedTheme::link`.
    Link,
}

/// Which appearance of a [`Role`] to use.
///
/// This is a *value*, not a taxonomy: `RoleVariant::Selected` is passed as data
/// (`ui.native_scope(Role::Checkbox, RoleVariant::from_selected(self.notify), ..)`),
/// so widget state stays where the state already lives and a mis-typed variant cannot silently
/// invert a condition.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum RoleVariant {
    /// Resting appearance.
    #[default]
    Normal,
    /// The checked appearance of a `Checkbox` or `RadioButton`, which takes no selected flag
    /// of its own: `checkbox.checked_background` and `checkbox.border.color` (§6.2). A
    /// button's suggested action, the active tab and segment and the switched-on switch are
    /// carried by their `Normal` cells, which the `Button`'s own `.selected(..)` flag picks
    /// from; their `Selected` cell is the `Normal` one.
    Selected,
    /// The platform's disabled appearance, written into the `inactive` entry, with the role's
    /// `disabled_opacity` as `Visuals::disabled_alpha`, which `Ui::disable` fades the widget by
    /// on top of those colours. A platform dims by one of the two, and its data makes the other
    /// an identity (docs/platform-facts.md §2.1.6). See §6.3.
    Disabled,
}

/// A container surface: the *chrome* (fill, stroke, corner radius, margins, shadow) of one
/// egui container.
///
/// A surface's frame is egui's own default frame for that container, built from the base
/// style of the same colour scheme, with the fields the theme states for that surface set on
/// it; every field the theme does not state keeps the preset's value (§3.4). The preset for
/// each surface is the one [`NativeThemeUiExt::native_frame`](crate::NativeThemeUiExt::native_frame) names.
///
/// The rule: **exactly one variant per container chrome in egui 0.36.2 to which an
/// application can attach an [`egui::Frame`]**. `Popover` and `Tooltip`
/// are two chromes behind one attachment point, `Popup::frame`, fed from different native
/// widgets. Two `Frame` setters are deliberately not surfaces, because they belong to widgets,
/// not containers: `TextEdit::frame` (`egui/src/widgets/text_edit/builder.rs:306`), whose
/// custom frame replaces egui's whole per-state painting — no hover or focus stroke
/// (`:734-735`) — and `AtomLayout::frame` (`egui/src/atomics/atom_layout.rs:112`), the
/// layout engine `Button` and `TextEdit` build on. egui has one `Panel` type
/// (`egui/src/containers/panel.rs:206`) with
/// four constructors (`left` `:249`, `right` `:256`, `top` `:265`, `bottom` `:274`), so the
/// panel case is one variant carrying the side. `egui::SidePanel` and `egui::TopBottomPanel`
/// **do not exist** in 0.36.2 and must never be named in this crate or its docs.
///
/// Menu chrome has no variant, because no menu builder takes a frame: `MenuButton`,
/// `SubMenuButton` and `MenuBar` have no `.frame(..)`, and `MenuConfig` carries no frame field
/// (`egui/src/containers/menu.rs:65-76`). egui builds the frame itself, after applying the
/// menu's `StyleModifier`: a top-level menu is a `Popup::menu` with no frame (`:325-331`), so
/// `Popup::show` applies the modifier and then builds `Frame::popup(ui.style())`
/// (`egui/src/containers/popup.rs:602-603`); a submenu's frame is `Frame::menu(ui.style())` of
/// the parent menu's already-modified `Ui` (`egui/src/containers/menu.rs:432`), handed to
/// `Popup::frame` (`:509`). Both presets read the same five `Style` fields (§3.2), and the
/// base style carries all five for every menu, which the `Role::Menu` cell inherits:
/// `window_fill` from `menu.background_color`, `window_stroke`, `menu_corner_radius` and
/// the `popup_shadow` gate from `defaults.border`, and egui's own `menu_margin` (§5.9).
/// What [`ThemeAtlas::role_modifier`](crate::ThemeAtlas::role_modifier) for `Role::Menu`, fed to `MenuConfig::style`
/// (`:107`), adds is the items' look: `menu.border`'s item border, radius and padding over
/// `menu_style`'s (§5.2, §6.1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Surface {
    /// `Window::frame` (`egui/src/containers/window.rs:265`).
    ///
    /// A `Window` given this frame and **no** `title_frame` paints its title bar with the same
    /// frame: `title_frame.unwrap_or(window_frame)` (`egui/src/containers/window.rs:630-631`),
    /// taken before the window's own inner margin is zeroed (`:634-635`). The title bar then
    /// uses this frame's inner margin as its padding (`:1352`) and paints its fill, stroke and
    /// corner radius without the shadow (`:1425`), the fill replaced by
    /// `widgets.open.weak_bg_fill` on the top-most window (`:1426-1428`) and the bottom corners
    /// squared while the window is expanded (`:1429-1432`). egui 0.36.1 fell back to
    /// `Frame::window(&style)` instead, which is one reason this crate requires 0.36.2. Hand
    /// [`Surface::WindowTitleBar`] to `title_frame` to give the title bar its own chrome.
    Window,
    /// `Window::title_frame` (`egui/src/containers/window.rs:272`). Without it the title bar
    /// takes the window's frame; see [`Surface::Window`].
    WindowTitleBar,
    /// `Modal::frame` (`egui/src/containers/modal.rs:53`).
    Dialog,
    /// `Popup::frame` (`egui/src/containers/popup.rs:369`).
    Popover,
    /// The manual tooltip path: `Tooltip::popup` is a public field
    /// (`egui/src/containers/tooltip.rs:9`), so the frame is set in place and the tooltip
    /// still shown through `Tooltip::show` (`:101`), which does the tooltip bookkeeping:
    /// `let mut t = Tooltip::for_enabled(&r); t.popup = t.popup.frame(..); t.show(..)`.
    /// **`for_enabled`** (`:53-59`), which opens the popup only while the enabled widget is
    /// hovered long enough (`should_show_tooltip`, `:57`) — never `for_widget` (`:39`), which
    /// is "Always open (as long as this function is called)" (`:38`) and would pin the tooltip
    /// on screen. `for_disabled` (`:62`) is the same for a disabled widget.
    /// `Response::on_hover_text` does **not** take a frame — see §14 item 3.
    Tooltip,
    /// `Frame::show` (`egui/src/containers/frame.rs:404`) — the frame-taking spelling of
    /// `Ui::group`, which builds `Frame::group(self.style())` itself and takes no frame
    /// (`egui/src/ui.rs:2147-2148`).
    Card,
    /// `Panel::{left,right,top,bottom}(..).frame(..)` (`egui/src/containers/panel.rs:413`).
    Panel(PanelSide),
    /// `CentralPanel::frame` (`egui/src/containers/panel.rs:1206`): `defaults` chrome with
    /// `layout.window_margin` ([`Builder::layout`](crate::Builder::layout)) as its inner margin on all four sides —
    /// the gap between the window's edge and its content, which is what the application's
    /// `CentralPanel` is. Where `window_margin` is `None` (§2, *Unstated sizes*) the inner
    /// margin stays `Frame::central_panel`'s own `8` (`egui/src/containers/frame.rs:192`).
    CentralPanel,
}

/// Which side a `Panel` is anchored to. Exhaustive on purpose: four sides is a closed fact of
/// 2-D screen geometry, not an API taxonomy that can churn.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PanelSide {
    /// Anchored to the left edge (`Panel::left`).
    Left,
    /// Anchored to the right edge (`Panel::right`).
    Right,
    /// Anchored to the top edge (`Panel::top`).
    Top,
    /// Anchored to the bottom edge (`Panel::bottom`).
    Bottom,
}

/// One of native-theme's four text-scale roles (`native-theme/src/model/resolved.rs:54-63`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TextRole {
    /// Caption / small label text: `text_scale.caption`.
    Caption,
    /// Section heading text: `text_scale.section_heading`.
    SectionHeading,
    /// Dialog title text: `text_scale.dialog_title`.
    DialogTitle,
    /// Large display / hero text: `text_scale.display`.
    Display,
}

/// Every `Role`, in declaration order: the layout of the atlas's cells (`SchemeStyles::cells`).
pub(crate) const ROLES: [Role; 25] = [
    Role::Window,
    Role::Button,
    Role::Input,
    Role::Checkbox,
    Role::Menu,
    Role::Tooltip,
    Role::Scrollbar,
    Role::Slider,
    Role::ProgressBar,
    Role::Tab,
    Role::Sidebar,
    Role::Toolbar,
    Role::StatusBar,
    Role::List,
    Role::Popover,
    Role::Splitter,
    Role::Separator,
    Role::Switch,
    Role::Dialog,
    Role::Spinner,
    Role::ComboBox,
    Role::SegmentedControl,
    Role::Card,
    Role::Expander,
    Role::Link,
];

impl Role {
    /// Every variant known to this build, in declaration order. Provided because the enum is
    /// `#[non_exhaustive]` and callers cannot write their own exhaustive list.
    #[must_use]
    pub fn all() -> &'static [Self] {
        &ROLES
    }

    /// The stable identifier used in `mapping.toml` and the generated `src/mapping.md`, e.g.
    /// `"combo_box"`. Equals the corresponding [`ResolvedTheme`](crate::ResolvedTheme) field name.
    #[must_use]
    pub fn key(self) -> &'static str {
        match self {
            Self::Window => "window",
            Self::Button => "button",
            Self::Input => "input",
            Self::Checkbox => "checkbox",
            Self::Menu => "menu",
            Self::Tooltip => "tooltip",
            Self::Scrollbar => "scrollbar",
            Self::Slider => "slider",
            Self::ProgressBar => "progress_bar",
            Self::Tab => "tab",
            Self::Sidebar => "sidebar",
            Self::Toolbar => "toolbar",
            Self::StatusBar => "status_bar",
            Self::List => "list",
            Self::Popover => "popover",
            Self::Splitter => "splitter",
            Self::Separator => "separator",
            Self::Switch => "switch",
            Self::Dialog => "dialog",
            Self::Spinner => "spinner",
            Self::ComboBox => "combo_box",
            Self::SegmentedControl => "segmented_control",
            Self::Card => "card",
            Self::Expander => "expander",
            Self::Link => "link",
        }
    }

    /// Position in [`ROLES`]: a fieldless enum's discriminant, in declaration order.
    pub(crate) const fn index(self) -> usize {
        self as usize
    }
}

/// Every `RoleVariant`, in declaration order.
pub(crate) const VARIANTS: [RoleVariant; 3] = [
    RoleVariant::Normal,
    RoleVariant::Selected,
    RoleVariant::Disabled,
];

impl RoleVariant {
    /// `Selected` when `selected`, `Normal` otherwise.
    #[must_use]
    pub const fn from_selected(selected: bool) -> Self {
        if selected {
            Self::Selected
        } else {
            Self::Normal
        }
    }

    /// Every variant known to this build, in declaration order, for the same reason as
    /// [`Role::all`].
    #[must_use]
    pub fn all() -> &'static [Self] {
        &VARIANTS
    }

    /// The stable identifier a `mapping.toml` sink's `variant` names (§13.1): `"normal"`,
    /// `"selected"`, `"disabled"`.
    #[must_use]
    pub fn key(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Selected => "selected",
            Self::Disabled => "disabled",
        }
    }

    /// Position in [`VARIANTS`].
    pub(crate) const fn index(self) -> usize {
        self as usize
    }
}

/// Every `Surface`, panel sides expanded, in the order of `Surface::key`'s list (spec §4.4).
pub(crate) const SURFACES: [Surface; 11] = [
    Surface::Window,
    Surface::WindowTitleBar,
    Surface::Dialog,
    Surface::Popover,
    Surface::Tooltip,
    Surface::Card,
    Surface::Panel(PanelSide::Left),
    Surface::Panel(PanelSide::Right),
    Surface::Panel(PanelSide::Top),
    Surface::Panel(PanelSide::Bottom),
    Surface::CentralPanel,
];

impl Surface {
    /// Every surface known to this build, panel sides expanded — 11 entries.
    #[must_use]
    pub fn all() -> &'static [Self] {
        &SURFACES
    }

    /// The stable identifier a `mapping.toml` sink's `surface` names (§13.1), one per entry
    /// of [`Surface::all`]: `"window"`, `"window_title_bar"`, `"dialog"`, `"popover"`,
    /// `"tooltip"`, `"card"`, `"panel_left"`, `"panel_right"`, `"panel_top"`,
    /// `"panel_bottom"`, `"central_panel"`.
    #[must_use]
    pub fn key(self) -> &'static str {
        match self {
            Self::Window => "window",
            Self::WindowTitleBar => "window_title_bar",
            Self::Dialog => "dialog",
            Self::Popover => "popover",
            Self::Tooltip => "tooltip",
            Self::Card => "card",
            Self::Panel(PanelSide::Left) => "panel_left",
            Self::Panel(PanelSide::Right) => "panel_right",
            Self::Panel(PanelSide::Top) => "panel_top",
            Self::Panel(PanelSide::Bottom) => "panel_bottom",
            Self::CentralPanel => "central_panel",
        }
    }

    /// Position in [`SURFACES`]. A `match`, because `Panel` carries data and cannot be cast.
    pub(crate) fn index(self) -> usize {
        match self {
            Self::Window => 0,
            Self::WindowTitleBar => 1,
            Self::Dialog => 2,
            Self::Popover => 3,
            Self::Tooltip => 4,
            Self::Card => 5,
            Self::Panel(PanelSide::Left) => 6,
            Self::Panel(PanelSide::Right) => 7,
            Self::Panel(PanelSide::Top) => 8,
            Self::Panel(PanelSide::Bottom) => 9,
            Self::CentralPanel => 10,
        }
    }
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
    use native_theme::theme::{ColorMode, Theme};
    use std::collections::BTreeSet;

    /// §4.4: exactly one `Role` per widget field of `ResolvedTheme`, and `key()` is that
    /// field's name. `serde_json`'s object keys are sorted, so the set is compared; the order
    /// is `ROLES`'s by construction. `text_area` is no role of its own: a multi-line `TextEdit`
    /// is drawn in `Role::Input`, and the one thing it does not share with the single-line
    /// field, its border, reaches it through `text_area_margin` and `text_area_frame`
    /// (`docs/platform-facts.md` §2.29).
    #[test]
    fn roles_are_the_widget_fields_of_resolved_theme() {
        let resolved = Theme::preset("kde-breeze")
            .unwrap()
            .into_variant(ColorMode::Light)
            .unwrap()
            .resolve_system()
            .unwrap();
        let value = serde_json::to_value(&resolved).unwrap();
        let fields: BTreeSet<&str> = value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .filter(|k| *k != "defaults" && *k != "text_scale" && *k != "text_area")
            .collect();
        let keys: BTreeSet<&str> = Role::all().iter().map(|r| r.key()).collect();
        assert_eq!(keys, fields);
        assert_eq!(Role::all().len(), 25);
        assert_eq!(keys.len(), 25, "keys are distinct");
        for (i, r) in Role::all().iter().enumerate() {
            assert_eq!(r.index(), i, "{r:?}");
            assert_eq!(ROLES[i], *r);
        }
    }

    #[test]
    fn role_variants_have_stable_keys_and_indices() {
        assert_eq!(
            RoleVariant::all(),
            &[
                RoleVariant::Normal,
                RoleVariant::Selected,
                RoleVariant::Disabled
            ]
        );
        assert_eq!(RoleVariant::Normal.key(), "normal");
        assert_eq!(RoleVariant::Selected.key(), "selected");
        assert_eq!(RoleVariant::Disabled.key(), "disabled");
        assert_eq!(RoleVariant::from_selected(true), RoleVariant::Selected);
        assert_eq!(RoleVariant::from_selected(false), RoleVariant::Normal);
        assert_eq!(RoleVariant::default(), RoleVariant::Normal);
        for (i, v) in RoleVariant::all().iter().enumerate() {
            assert_eq!(v.index(), i);
            assert_eq!(VARIANTS[i], *v);
        }
    }

    /// §4.4: eleven surfaces, panel sides expanded, with the eleven keys of `Surface::key`.
    #[test]
    fn surfaces_are_eleven_with_distinct_keys_and_stable_indices() {
        let keys: Vec<&str> = Surface::all().iter().map(|s| s.key()).collect();
        assert_eq!(
            keys,
            [
                "window",
                "window_title_bar",
                "dialog",
                "popover",
                "tooltip",
                "card",
                "panel_left",
                "panel_right",
                "panel_top",
                "panel_bottom",
                "central_panel"
            ]
        );
        for (i, s) in Surface::all().iter().enumerate() {
            assert_eq!(s.index(), i, "{s:?}");
            assert_eq!(SURFACES[i], *s);
        }
        assert_eq!(Surface::Panel(PanelSide::Top).key(), "panel_top");
    }

    /// §4.3: the two data-carrying `Note` variants are constructible and comparable.
    #[test]
    fn notes_carry_their_data() {
        let a = crate::Note::TransparentFill {
            path: "button.background",
        };
        let b = crate::Note::FontDataInvalid {
            family: "Inter".into(),
        };
        assert_ne!(a, b);
        assert_eq!(a.clone(), a);
        assert_eq!(format!("{b:?}"), "FontDataInvalid { family: \"Inter\" }");
    }
}
