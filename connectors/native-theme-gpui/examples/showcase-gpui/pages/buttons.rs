//! The Buttons page.

use gpui::{Context, IntoElement, ParentElement, Styled, Window, prelude::*};
use gpui_component::{IconName, Size, h_flex, v_flex};

use crate::app::Showcase;
use crate::demo::{self, ButtonKind, ButtonState, DemoButton};
use crate::{
    BUTTONS_DANGER, BUTTONS_DISABLED_SECONDARY, BUTTONS_HEADING_VARIANTS, BUTTONS_PRIMARY,
    BUTTONS_TEXT, PROBE_CLIPBOARD, probe,
};

/// The variant row: one Button per variant the showcase builds, as `(id,
/// label, variant)`.
const VARIANTS: [(&str, &str, ButtonKind); 10] = [
    (BUTTONS_PRIMARY, "Primary", ButtonKind::Primary),
    ("buttons-secondary", "Secondary", ButtonKind::Secondary),
    (BUTTONS_DANGER, "Danger", ButtonKind::Danger),
    ("buttons-success", "Success", ButtonKind::Success),
    ("buttons-warning", "Warning", ButtonKind::Warning),
    ("buttons-info", "Info", ButtonKind::Info),
    ("buttons-ghost", "Ghost", ButtonKind::Ghost),
    ("buttons-link", "Link", ButtonKind::Link),
    (BUTTONS_TEXT, "Text", ButtonKind::Text),
    ("buttons-outline", "Outline", ButtonKind::PrimaryOutline),
];

/// The size row, as `(id, label, size)`.
const SIZES: [(&str, &str, Size); 4] = [
    ("buttons-size-xsmall", "XSmall", Size::XSmall),
    ("buttons-size-small", "Small", Size::Small),
    ("buttons-size-medium", "Medium", Size::Medium),
    ("buttons-size-large", "Large", Size::Large),
];

/// The disabled row, as `(id, label, variant)`.
const DISABLED: [(&str, &str, ButtonKind); 3] = [
    (
        "buttons-disabled-primary",
        "Disabled Primary",
        ButtonKind::Primary,
    ),
    (
        BUTTONS_DISABLED_SECONDARY,
        "Disabled Secondary",
        ButtonKind::Secondary,
    ),
    (
        "buttons-disabled-danger",
        "Disabled Danger",
        ButtonKind::Danger,
    ),
];

/// The icon row, as `(id, label, variant, icon)`.
const WITH_ICONS: [(&str, &str, ButtonKind, IconName); 3] = [
    (
        "buttons-icon-save",
        "Save",
        ButtonKind::Primary,
        IconName::Check,
    ),
    (
        "buttons-icon-search",
        "Search",
        ButtonKind::Default,
        IconName::Search,
    ),
    (
        "buttons-icon-delete",
        "Delete",
        ButtonKind::Danger,
        IconName::Delete,
    ),
];

impl Showcase {
    // -----------------------------------------------------------------------
    // Page: Buttons
    // -----------------------------------------------------------------------
    pub(crate) fn render_buttons_page(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let ui = &self.info_ui;
        let on_bold = cx.listener(|this, checked: &bool, _w, _cx| {
            this.toggle_bold = *checked;
        });
        let on_italic = cx.listener(|this, checked: &bool, _w, _cx| {
            this.toggle_italic = *checked;
        });
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            .child(demo::heading(ui, cx, BUTTONS_HEADING_VARIANTS, "Button Variants: eight of upstream's, the connector's ghost_button, and Primary outlined"))
            .child(
                h_flex()
                    .gap_2()
                    .flex_wrap()
                    .children(VARIANTS.map(|(id, label, kind)| {
                        demo::button(
                            ui,
                            cx,
                            DemoButton {
                                id,
                                label,
                                kind,
                                state: ButtonState::Idle,
                                icon: None,
                            },
                        )
                    })),
            )
            .child(demo::heading(ui, cx, "buttons-heading-sizes", "Button Sizes"))
            .child(h_flex().gap_2().items_end().children(
                SIZES.map(|(id, label, size)| demo::sized_button(ui, cx, id, label, size)),
            ))
            .child(demo::heading(ui, cx, "buttons-heading-group", "ButtonGroup"))
            .child(h_flex().child(demo::button_group(
                ui,
                cx,
                "buttons-group",
                &["Left", "Center", "Right"],
            )))
            .child(demo::heading(ui, cx, "buttons-heading-disabled", "Disabled State"))
            .child(h_flex().gap_2().children(DISABLED.map(|(id, label, kind)| {
                demo::button(
                    ui,
                    cx,
                    DemoButton {
                        id,
                        label,
                        kind,
                        state: ButtonState::Disabled,
                        icon: None,
                    },
                )
            })))
            .child(demo::heading(ui, cx, "buttons-heading-loading", "Loading State"))
            .child(h_flex().gap_2().child(demo::button(
                ui,
                cx,
                DemoButton {
                    id: "buttons-loading",
                    label: "Loading...",
                    kind: ButtonKind::Primary,
                    state: ButtonState::Loading,
                    // `Button` draws its spinner in place of its icon
                    // (`button/button_icon.rs`), so a loading button
                    // needs one to show it; this one is never seen while
                    // `loading` is true.
                    icon: Some(IconName::Check),
                },
            )))
            .child(demo::heading(ui, cx, "buttons-heading-icons", "Buttons with Icons"))
            .child(
                h_flex()
                    .gap_2()
                    .children(WITH_ICONS.map(|(id, label, kind, icon)| {
                        demo::button(
                            ui,
                            cx,
                            DemoButton {
                                id,
                                label,
                                kind,
                                state: ButtonState::Idle,
                                icon: Some(icon),
                            },
                        )
                    })),
            )
            .child(demo::heading(ui, cx, "buttons-heading-dropdown", "DropdownButton"))
            .child(
                h_flex()
                    .gap_4()
                    .child(demo::dropdown_button(
                        ui,
                        cx,
                        "buttons-dropdown-save",
                        "Save",
                        ButtonKind::Primary,
                        |menu, _w, _cx| {
                            menu.menu("Save as Draft", Box::new(gpui::NoAction))
                                .separator()
                                .menu("Export as PDF", Box::new(gpui::NoAction))
                        },
                    ))
                    .child(demo::dropdown_button(
                        ui,
                        cx,
                        "buttons-dropdown-actions",
                        "Actions",
                        ButtonKind::Default,
                        |menu, _w, _cx| {
                            menu.menu("Cut", Box::new(gpui::NoAction))
                                .menu("Copy", Box::new(gpui::NoAction))
                                .menu("Paste", Box::new(gpui::NoAction))
                        },
                    )),
            )
            .child(demo::heading(ui, cx, "buttons-heading-toggle", "Toggle & ToggleGroup"))
            .child(
                h_flex()
                    .gap_6()
                    .items_center()
                    .child(demo::toggle(
                        ui,
                        cx,
                        "buttons-toggle-star",
                        IconName::Star,
                        "Star",
                        self.toggle_bold,
                        on_bold,
                    ))
                    .child(demo::toggle(
                        ui,
                        cx,
                        "buttons-toggle-heart",
                        IconName::Heart,
                        "Heart",
                        self.toggle_italic,
                        on_italic,
                    ))
                    .child(demo::toggle_group(
                        ui,
                        cx,
                        "buttons-toggle-group",
                        &["Left", "Center", "Right"],
                    )),
            )
            .child(demo::heading(ui, cx, "buttons-heading-clipboard", "Clipboard"))
            .child(
                h_flex()
                    .gap_4()
                    .child(probe(
                        PROBE_CLIPBOARD,
                        demo::clipboard(
                            ui,
                            cx,
                            "buttons-clipboard-cargo",
                            "cargo add native-theme",
                        ),
                    ))
                    .child(demo::clipboard(
                        ui,
                        cx,
                        "buttons-clipboard-npm",
                        "npm install native-theme",
                    )),
            )
    }
}
