//! The Layout page.

use gpui::{
    Context, IntoElement, ParentElement, SharedString, Styled, Window, div, prelude::*, px,
};
use gpui_component::{h_flex, v_flex};

use native_theme_gpui::geometry;

use crate::app::Showcase;
use crate::demo::{
    self, ButtonKind, ButtonState, DemoButton, GroupBoxKind, SeparatorKind, SpacingBox, StepperKind,
};
use crate::support::{NativeStyled as _, STEPPER_STEPS, layout_value, with_gap};
use crate::{
    LAYOUT_GROUP_BOX_NORMAL, LAYOUT_GROUP_BOX_OUTLINE, LAYOUT_SEPARATOR_DASHED,
    LAYOUT_SEPARATOR_SOLID, PROBE_STEPPER, Page, probe,
};

/// The Separators, as `(id, kind)`.
const SEPARATORS: [(&str, SeparatorKind); 3] = [
    (LAYOUT_SEPARATOR_SOLID, SeparatorKind::Horizontal),
    (
        "layout-separator-labelled",
        SeparatorKind::Labelled("Section Break"),
    ),
    (LAYOUT_SEPARATOR_DASHED, SeparatorKind::Dashed),
];

/// The GroupBox gallery, as `(id, kind, title, content)`.
const GROUP_BOXES: [(&str, GroupBoxKind, &str, &str); 3] = [
    (
        LAYOUT_GROUP_BOX_NORMAL,
        GroupBoxKind::Normal,
        "Default",
        "Default style",
    ),
    (
        "layout-group-box-fill",
        GroupBoxKind::Fill,
        "Filled",
        "Filled background",
    ),
    (
        LAYOUT_GROUP_BOX_OUTLINE,
        GroupBoxKind::Outline,
        "Outline",
        "Outlined border",
    ),
];

/// The Buttons in the spacing row, as `(id, label)`.
const SPACING_BUTTONS: [(&str, &str); 3] = [
    ("layout-spacing-one", "One"),
    ("layout-spacing-two", "Two"),
    ("layout-spacing-three", "Three"),
];

/// The pages the Breadcrumb leads through to this one.
const BREADCRUMB_PAGES: [Page; 4] = [Page::Buttons, Page::Inputs, Page::Data, Page::Feedback];

impl Showcase {
    // -----------------------------------------------------------------------
    // Page: Layout
    // -----------------------------------------------------------------------
    pub(crate) fn render_layout_page(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + InteractiveElement {
        let ui = &self.info_ui;
        let collapsible_open = self.collapsible_open;
        // The four layout accessors. `None` where the platform specifies
        // nothing (platform-facts §2.20), and then the showcase's own spacing
        // stands — nothing is invented to fill the gap.
        let widget_gap = geometry::widget_gap(&self.layout);
        let container_margin = geometry::container_margin(&self.layout);
        let window_margin = geometry::window_margin(&self.layout);
        let section_gap = geometry::section_gap(&self.layout);
        let spacing_summary = format!(
            "widget_gap {} · container_margin {} · window_margin {} · section_gap {}",
            layout_value(widget_gap),
            layout_value(container_margin),
            layout_value(window_margin),
            layout_value(section_gap),
        );
        let steps = STEPPER_STEPS.len();
        let step = self.step;
        let presets = native_theme::theme::Theme::list_presets().len();
        let toggle_collapsible = cx.listener(|this, _ev, _w, _cx| {
            this.collapsible_open = !this.collapsible_open;
        });
        let pick_step = cx.listener(|this, step: &usize, _w, cx| {
            this.step = *step;
            cx.notify();
        });
        let pick_step_vertical = cx.listener(|this, step: &usize, _w, cx| {
            this.step = *step;
            cx.notify();
        });
        let spacing_row = demo::spacing_box(
            ui,
            cx,
            "layout-spacing-container",
            SpacingBox::Container,
            container_margin,
            widget_gap,
            SPACING_BUTTONS.map(|(id, label)| {
                demo::button(
                    ui,
                    cx,
                    DemoButton {
                        id,
                        label,
                        kind: ButtonKind::Default,
                        state: ButtonState::Idle,
                        icon: None,
                    },
                )
                .into_any_element()
            }),
        );
        v_flex()
            .gap_5()
            .p_4()
            .flex_1()
            // The four layout accessors, applied rather than printed: the outer
            // box takes the platform's window margin, the row inside it the
            // container margin and the widget gap, and the two rows are
            // separated by the section gap.
            .child(demo::heading(
                ui,
                cx,
                "layout-heading-spacing",
                "Layout spacing (the four LayoutTheme accessors)",
            ))
            .child(demo::spacing_box(
                ui,
                cx,
                "layout-spacing-window",
                SpacingBox::Window,
                window_margin,
                section_gap,
                [
                    spacing_row.into_any_element(),
                    demo::label(ui, cx, "layout-spacing-summary", spacing_summary)
                        .into_any_element(),
                ],
            ))
            .child(demo::heading(
                ui,
                cx,
                "layout-heading-separator",
                "Separator (solid / labelled / dashed)",
            ))
            .child(
                v_flex()
                    .gap_3()
                    .children(SEPARATORS.map(|(id, kind)| demo::separator(ui, cx, id, kind))),
            )
            .child(demo::heading(
                ui,
                cx,
                "layout-heading-group-box-container",
                "GroupBox as Layout Container",
            ))
            .child(demo::group_box(
                ui,
                cx,
                "layout-group-box-container",
                GroupBoxKind::Fill,
                "Contained Content",
                v_flex()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .child("GroupBox can wrap any content as a visual container."),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(demo::button(
                                ui,
                                cx,
                                DemoButton {
                                    id: "layout-group-box-action-a",
                                    label: "Action A",
                                    kind: ButtonKind::Default,
                                    state: ButtonState::Idle,
                                    icon: None,
                                },
                            ))
                            .child(demo::button(
                                ui,
                                cx,
                                DemoButton {
                                    id: "layout-group-box-action-b",
                                    label: "Action B",
                                    kind: ButtonKind::Primary,
                                    state: ButtonState::Idle,
                                    icon: None,
                                },
                            )),
                    ),
                "a line of plain text, which takes group_box_foreground from the content box, over two Buttons, which report themselves",
            ))
            .child(demo::heading(
                ui,
                cx,
                "layout-heading-scroll",
                "Scrollable Area (visible scrollbar)",
            ))
            .child(demo::scroll_area(
                ui,
                cx,
                "layout-scroll-area",
                px(150.0),
                20,
            ))
            .child(demo::heading(
                ui,
                cx,
                "layout-heading-accordion",
                "Accordion",
            ))
            .child(demo::accordion(
                ui,
                cx,
                "layout-accordion",
                [
                    (
                        "What is native-theme?",
                        "layout-accordion-answer-1",
                        "A cross-platform theme abstraction that reads OS settings and maps \
                         them to toolkit-specific themes."
                            .into(),
                    ),
                    (
                        "Supported toolkits",
                        "layout-accordion-answer-2",
                        "gpui-component, iced, egui, and more planned.".into(),
                    ),
                    (
                        "How many presets?",
                        "layout-accordion-answer-3",
                        // Counted from the list itself, so the answer cannot
                        // go stale again.
                        SharedString::from(format!(
                            "{presets} built-in theme presets covering major OS styles."
                        )),
                    ),
                ],
            ))
            .child(demo::heading(
                ui,
                cx,
                "layout-heading-collapsible",
                "Collapsible",
            ))
            .child(demo::collapsible(
                ui,
                cx,
                "layout-collapsible",
                "layout-collapsible-toggle",
                "layout-collapsible-content",
                collapsible_open,
                toggle_collapsible,
            ))
            .child(demo::heading(
                ui,
                cx,
                "layout-heading-carousel",
                "Carousel",
            ))
            .child(
                demo::carousel(
                    ui,
                    cx,
                    "layout-carousel",
                    &self.carousel_state,
                    px(360.0),
                    px(120.0),
                )
                // The slide controls are positioned outside the frame, so
                // the section leaves a button's width on either side.
                .px_16(),
            )
            .child(demo::heading(
                ui,
                cx,
                "layout-heading-group-box",
                "GroupBox (3 variants)",
            ))
            .child(h_flex().gap_4().children(GROUP_BOXES.map(
                |(id, kind, title, content)| {
                    demo::group_box(
                        ui,
                        cx,
                        id,
                        kind,
                        title,
                        div().text_sm().child(content),
                        "a line of plain text, which takes group_box_foreground from the content box",
                    )
                    .w(px(180.0))
                },
            )))
            .child(demo::heading(
                ui,
                cx,
                "layout-heading-breadcrumb",
                "Breadcrumb (click to navigate pages)",
            ))
            .child(demo::breadcrumb(
                ui,
                cx,
                "layout-breadcrumb",
                &BREADCRUMB_PAGES,
                Page::Layout,
            ))
            .child(demo::heading(
                ui,
                cx,
                "layout-heading-stepper",
                format!(
                    "Stepper (step {} of {steps}: {step} completed, one current, {} pending)",
                    step + 1,
                    steps.saturating_sub(step + 1),
                ),
            ))
            .child(
                with_gap(v_flex(), widget_gap)
                    .w(px(480.0))
                    .child(probe(
                        PROBE_STEPPER,
                        demo::stepper(
                            ui,
                            cx,
                            "layout-stepper-horizontal",
                            StepperKind::Icons,
                            step,
                            pick_step,
                        ),
                    ))
                    .child(demo::stepper(
                        ui,
                        cx,
                        "layout-stepper-vertical",
                        StepperKind::Numbers,
                        step,
                        pick_step_vertical,
                    )),
            )
            .child(demo::heading(
                ui,
                cx,
                "layout-heading-form",
                "Form / Field (horizontal layout)",
            ))
            .child(demo::form(
                ui,
                cx,
                "layout-form",
                ("layout-form-name", &self.form_name_state),
                ("layout-form-email", &self.form_email_state),
            ))
            .child(demo::heading(
                ui,
                cx,
                "layout-heading-settings",
                "Settings (page with field types)",
            ))
            .child(
                div()
                    .id("layout-settings-frame")
                    .debug_selector(|| "settings-frame".into())
                    .h(px(320.0))
                    .w_full()
                    .occlude()
                    .demo_frame(cx)
                    .overflow_y_scroll()
                    .child(demo::settings(ui, cx, "layout-settings")),
            )
    }
}
