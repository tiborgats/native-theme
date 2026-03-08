//! native-theme-gpui Comprehensive Showcase
//!
//! A professional widget gallery demonstrating native-theme integration with
//! gpui-component. Displays every major widget family organized in tabbed
//! sections, with full theme switching and working dark mode toggle.
//!
//! # Running
//!
//! ```sh
//! cargo run -p native-theme-gpui --example showcase
//! ```

use gpui::{
    div, prelude::*, px, size, App, Application, Bounds, Context, Entity, IntoElement,
    ParentElement, Render, SharedString, Styled, Window, WindowBounds, WindowOptions,
};
use gpui_component::{
    accordion::Accordion,
    alert::Alert,
    badge::Badge,
    breadcrumb::{Breadcrumb, BreadcrumbItem},
    button::{Button, ButtonGroup, ButtonVariants},
    checkbox::Checkbox,
    description_list::DescriptionList,
    divider::Divider,
    group_box::{GroupBox, GroupBoxVariants},
    h_flex,
    input::{Input, InputState, NumberInput},
    label::Label,
    link::Link,
    progress::Progress,
    radio::RadioGroup,
    select::{SearchableVec, Select, SelectEvent, SelectState},
    skeleton::Skeleton,
    slider::{Slider, SliderEvent, SliderState},
    spinner::Spinner,
    switch::Switch,
    tab::TabBar,
    tag::Tag,
    theme::Theme,
    v_flex, ActiveTheme, Disableable, Root, Sizable, Size, StyledExt,
};

use native_theme::NativeTheme;
use native_theme_gpui::{pick_variant, to_theme};

// ---------------------------------------------------------------------------
// Tab indices for navigation
// ---------------------------------------------------------------------------

const TAB_BUTTONS: usize = 0;
const TAB_INPUTS: usize = 1;
const TAB_DATA: usize = 2;
const TAB_FEEDBACK: usize = 3;
const TAB_TYPOGRAPHY: usize = 4;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build the list of theme names: all presets + "OS Theme".
fn theme_names() -> Vec<SharedString> {
    let mut names: Vec<SharedString> = NativeTheme::list_presets()
        .iter()
        .map(|s| SharedString::from(s.to_string()))
        .collect();
    names.push("OS Theme".into());
    names
}

// ---------------------------------------------------------------------------
// Main view
// ---------------------------------------------------------------------------

struct Showcase {
    // Theme controls
    theme_select: Entity<SelectState<SearchableVec<SharedString>>>,
    current_theme_name: String,
    is_dark: bool,

    // Navigation
    active_tab: usize,

    // Widget state
    input_state: Entity<InputState>,
    number_input_state: Entity<InputState>,
    slider_state: Entity<SliderState>,
    checkbox_a: bool,
    checkbox_b: bool,
    checkbox_c: bool,
    switch_on: bool,
    radio_index: Option<usize>,
    slider_value: f32,
}

impl Showcase {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let names = theme_names();
        let delegate = SearchableVec::new(names);

        let theme_select = cx.new(|cx| {
            SelectState::new(
                delegate,
                Some(gpui_component::IndexPath::default().row(0)),
                window,
                cx,
            )
        });

        // Subscribe to theme selection changes.
        cx.subscribe_in(
            &theme_select,
            window,
            |this: &mut Self, _entity, event: &SelectEvent<SearchableVec<SharedString>>, window, cx| {
                if let SelectEvent::Confirm(Some(value)) = event {
                    let name = value.to_string();
                    this.current_theme_name = name.clone();
                    this.apply_theme_by_name(&name, window, cx);
                }
            },
        )
        .detach();

        let input_state = cx.new(|cx| {
            let mut state = InputState::new(window, cx);
            state.set_placeholder("Type something here...", window, cx);
            state
        });

        let number_input_state = cx.new(|cx| {
            let mut state = InputState::new(window, cx);
            state.set_placeholder("0", window, cx);
            state
        });

        let slider_state = cx.new(|_cx| SliderState::new().default_value(65.0));

        cx.subscribe_in(
            &slider_state,
            window,
            |this: &mut Self, _entity, event: &SliderEvent, _window, _cx| {
                let SliderEvent::Change(val) = event;
                this.slider_value = val.start();
            },
        )
        .detach();

        // Apply the initial "default" preset theme.
        let is_dark = cx.theme().is_dark();
        let nt = NativeTheme::preset("default").expect("default preset must exist");
        if let Some(variant) = pick_variant(&nt, is_dark) {
            let theme = to_theme(variant, "default", is_dark);
            *Theme::global_mut(cx) = theme;
            window.refresh();
        }

        Self {
            theme_select,
            current_theme_name: "default".into(),
            is_dark,
            active_tab: TAB_BUTTONS,
            input_state,
            number_input_state,
            slider_state,
            checkbox_a: true,
            checkbox_b: false,
            checkbox_c: false,
            switch_on: false,
            radio_index: Some(0),
            slider_value: 65.0,
        }
    }

    /// Apply a theme preset (or OS Theme) with the current dark/light mode.
    fn apply_theme_by_name(
        &mut self,
        name: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if name == "OS Theme" {
            Theme::sync_system_appearance(Some(window), cx);
            self.is_dark = cx.theme().is_dark();
            return;
        }

        let nt = match NativeTheme::preset(name) {
            Ok(t) => t,
            Err(_) => return,
        };

        if let Some(variant) = pick_variant(&nt, self.is_dark) {
            let theme = to_theme(variant, name, self.is_dark);
            *Theme::global_mut(cx) = theme;
            window.refresh();
        }
    }

    /// Toggle dark/light mode and re-apply the current theme.
    fn toggle_dark_mode(&mut self, is_dark: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.is_dark = is_dark;
        let name = self.current_theme_name.clone();
        self.apply_theme_by_name(&name, window, cx);
    }

    // -----------------------------------------------------------------------
    // Tab: Buttons & Actions
    // -----------------------------------------------------------------------
    fn render_buttons_tab(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_5()
            .p_4()
            .overflow_hidden()
            .flex_1()
            // Button variants
            .child(self.section_header("Button Variants"))
            .child(
                h_flex()
                    .gap_2()
                    .flex_wrap()
                    .child(Button::new("b-primary").label("Primary").primary())
                    .child(Button::new("b-secondary").label("Secondary"))
                    .child(Button::new("b-danger").label("Danger").danger())
                    .child(Button::new("b-success").label("Success").success())
                    .child(Button::new("b-warning").label("Warning").warning())
                    .child(Button::new("b-info").label("Info").info())
                    .child(Button::new("b-ghost").label("Ghost").ghost())
                    .child(Button::new("b-link").label("Link").link())
                    .child(Button::new("b-text").label("Text").text()),
            )
            // Button sizes
            .child(self.section_header("Button Sizes"))
            .child(
                h_flex()
                    .gap_2()
                    .items_end()
                    .child(Button::new("s-xs").label("XSmall").primary().with_size(Size::XSmall))
                    .child(Button::new("s-sm").label("Small").primary().with_size(Size::Small))
                    .child(Button::new("s-md").label("Medium").primary().with_size(Size::Medium))
                    .child(Button::new("s-lg").label("Large").primary().with_size(Size::Large)),
            )
            // Button group
            .child(self.section_header("Button Group"))
            .child(
                ButtonGroup::new("bg-1")
                    .child(Button::new("bg-a").label("Left"))
                    .child(Button::new("bg-b").label("Center"))
                    .child(Button::new("bg-c").label("Right")),
            )
            // Disabled buttons
            .child(self.section_header("Disabled State"))
            .child(
                h_flex()
                    .gap_2()
                    .child(Button::new("d-pri").label("Disabled Primary").primary().disabled(true))
                    .child(Button::new("d-sec").label("Disabled Secondary").disabled(true))
                    .child(Button::new("d-dng").label("Disabled Danger").danger().disabled(true)),
            )
            // Link element
            .child(self.section_header("Link"))
            .child(
                h_flex()
                    .gap_4()
                    .child(
                        Link::new("link-1")
                            .child("Visit Documentation")
                            .href("https://github.com/nicegui/native-theme"),
                    )
                    .child(
                        Link::new("link-2")
                            .child("Another Link")
                            .href("https://gpui.rs"),
                    ),
            )
            // Breadcrumb
            .child(self.section_header("Breadcrumb"))
            .child(
                Breadcrumb::new()
                    .child(BreadcrumbItem::new("Home"))
                    .child(BreadcrumbItem::new("Components"))
                    .child(BreadcrumbItem::new("Buttons")),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Inputs & Controls
    // -----------------------------------------------------------------------
    fn render_inputs_tab(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let checkbox_a = self.checkbox_a;
        let checkbox_b = self.checkbox_b;
        let checkbox_c = self.checkbox_c;
        let switch_on = self.switch_on;
        let radio_index = self.radio_index;
        let slider_value = self.slider_value;

        v_flex()
            .gap_5()
            .p_4()
            .overflow_hidden()
            .flex_1()
            // Text Input
            .child(self.section_header("Text Input"))
            .child(
                Input::new(&self.input_state)
                    .with_size(Size::Medium)
                    .w(px(360.0)),
            )
            // Number Input
            .child(self.section_header("Number Input"))
            .child(
                NumberInput::new(&self.number_input_state)
                    .placeholder("Enter a number")
                    .with_size(Size::Medium)
                    .w(px(200.0)),
            )
            // Checkboxes
            .child(self.section_header("Checkboxes"))
            .child(
                v_flex()
                    .gap_3()
                    .child(
                        Checkbox::new("cb-a")
                            .label("Enable notifications")
                            .checked(checkbox_a)
                            .on_click(cx.listener(|this, val: &bool, _w, _cx| {
                                this.checkbox_a = *val;
                            })),
                    )
                    .child(
                        Checkbox::new("cb-b")
                            .label("Auto-save drafts")
                            .checked(checkbox_b)
                            .on_click(cx.listener(|this, val: &bool, _w, _cx| {
                                this.checkbox_b = *val;
                            })),
                    )
                    .child(
                        Checkbox::new("cb-c")
                            .label("Disabled checkbox")
                            .checked(checkbox_c)
                            .disabled(true),
                    ),
            )
            // Radio group
            .child(self.section_header("Radio Group"))
            .child(
                RadioGroup::horizontal("rg-1")
                    .child("Option A")
                    .child("Option B")
                    .child("Option C")
                    .selected_index(radio_index)
                    .on_click(cx.listener(|this, ix: &usize, _w, _cx| {
                        this.radio_index = Some(*ix);
                    })),
            )
            // Switch
            .child(self.section_header("Switch"))
            .child(
                h_flex()
                    .gap_6()
                    .child(
                        Switch::new("sw-feature")
                            .label("Feature toggle")
                            .checked(switch_on)
                            .on_click(cx.listener(|this, val: &bool, _w, _cx| {
                                this.switch_on = *val;
                            })),
                    )
                    .child(
                        Switch::new("sw-disabled")
                            .label("Disabled")
                            .checked(true)
                            .disabled(true),
                    ),
            )
            // Slider
            .child(self.section_header(format!("Slider (value: {:.0})", slider_value)))
            .child(
                Slider::new(&self.slider_state).w(px(360.0)),
            )
            // Progress
            .child(self.section_header("Progress Bar"))
            .child(
                v_flex()
                    .gap_2()
                    .child(Progress::new().value(slider_value).w(px(360.0)))
                    .child(Progress::new().value(25.0).w(px(360.0)))
                    .child(Progress::new().value(80.0).w(px(360.0))),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Data Display
    // -----------------------------------------------------------------------
    fn render_data_tab(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_5()
            .p_4()
            .overflow_hidden()
            .flex_1()
            // Description list
            .child(self.section_header("Description List"))
            .child(
                DescriptionList::new()
                    .columns(2)
                    .item("Name", "native-theme", 1)
                    .item("Version", "0.1.0", 1)
                    .item("License", "MIT / Apache-2.0", 1)
                    .item("Platform", "Cross-platform", 1)
                    .item("Description", "Universal theme abstraction layer", 2),
            )
            // Accordion
            .child(self.section_header("Accordion"))
            .child(
                Accordion::new("acc-1")
                    .item(|item| {
                        item.title("What is native-theme?")
                            .open(true)
                            .child(
                                Label::new(
                                    "A cross-platform theme abstraction that reads OS appearance \
                                     settings and maps them to toolkit-specific theme objects."
                                )
                                .text_sm(),
                            )
                    })
                    .item(|item| {
                        item.title("Supported toolkits")
                            .child(
                                Label::new(
                                    "gpui-component, iced, egui, and more planned."
                                )
                                .text_sm(),
                            )
                    })
                    .item(|item| {
                        item.title("How many presets?")
                            .child(
                                Label::new(
                                    "17 built-in theme presets covering major OS styles."
                                )
                                .text_sm(),
                            )
                    }),
            )
            // GroupBox variants
            .child(self.section_header("Group Box"))
            .child(
                h_flex()
                    .gap_4()
                    .child(
                        GroupBox::new()
                            .title("Normal")
                            .w(px(200.0))
                            .child(Label::new("Default group box style").text_sm()),
                    )
                    .child(
                        GroupBox::new()
                            .title("Filled")
                            .fill()
                            .w(px(200.0))
                            .child(Label::new("Filled background variant").text_sm()),
                    )
                    .child(
                        GroupBox::new()
                            .title("Outline")
                            .outline()
                            .w(px(200.0))
                            .child(Label::new("Outlined border variant").text_sm()),
                    ),
            )
            // Dividers
            .child(self.section_header("Dividers"))
            .child(
                v_flex()
                    .gap_3()
                    .child(Divider::horizontal())
                    .child(Divider::horizontal().label("Section Break"))
                    .child(Divider::horizontal_dashed()),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Feedback & Status
    // -----------------------------------------------------------------------
    fn render_feedback_tab(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_5()
            .p_4()
            .overflow_hidden()
            .flex_1()
            // Alerts
            .child(self.section_header("Alerts"))
            .child(
                v_flex()
                    .gap_2()
                    .child(Alert::info("alert-info", "This is an informational message.").title("Info"))
                    .child(Alert::success("alert-ok", "Operation completed successfully.").title("Success"))
                    .child(Alert::warning("alert-warn", "Please review before proceeding.").title("Warning"))
                    .child(Alert::error("alert-err", "Something went wrong. Please try again.").title("Error")),
            )
            // Spinner
            .child(self.section_header("Spinner"))
            .child(
                h_flex()
                    .gap_6()
                    .items_center()
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(Spinner::new().with_size(Size::Small))
                            .child(Label::new("Small").text_sm()),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(Spinner::new().with_size(Size::Medium))
                            .child(Label::new("Medium").text_sm()),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .items_center()
                            .child(Spinner::new().with_size(Size::Large))
                            .child(Label::new("Large").text_sm()),
                    ),
            )
            // Skeleton
            .child(self.section_header("Skeleton Loading"))
            .child(
                v_flex()
                    .gap_2()
                    .w(px(360.0))
                    .child(Skeleton::new().h(px(12.0)).w(px(200.0)).rounded(px(4.0)))
                    .child(Skeleton::new().h(px(8.0)).w(px(300.0)).rounded(px(4.0)))
                    .child(Skeleton::new().h(px(8.0)).w(px(250.0)).rounded(px(4.0)))
                    .child(Skeleton::new().secondary().h(px(60.0)).rounded(px(6.0))),
            )
            // Progress indicators
            .child(self.section_header("Progress Indicators"))
            .child(
                v_flex()
                    .gap_3()
                    .w(px(360.0))
                    .child(
                        h_flex()
                            .justify_between()
                            .child(Label::new("Upload").text_sm())
                            .child(Label::new("73%").text_sm()),
                    )
                    .child(Progress::new().value(73.0))
                    .child(
                        h_flex()
                            .justify_between()
                            .child(Label::new("Processing").text_sm())
                            .child(Label::new("45%").text_sm()),
                    )
                    .child(Progress::new().value(45.0))
                    .child(
                        h_flex()
                            .justify_between()
                            .child(Label::new("Complete").text_sm())
                            .child(Label::new("100%").text_sm()),
                    )
                    .child(Progress::new().value(100.0)),
            )
    }

    // -----------------------------------------------------------------------
    // Tab: Typography & Decorations
    // -----------------------------------------------------------------------
    fn render_typography_tab(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_5()
            .p_4()
            .overflow_hidden()
            .flex_1()
            // Labels
            .child(self.section_header("Label"))
            .child(
                v_flex()
                    .gap_2()
                    .child(Label::new("Regular label"))
                    .child(Label::new("Label with secondary").secondary("(secondary text)"))
                    .child(Label::new("Masked label: secret123").masked(true)),
            )
            // Tags
            .child(self.section_header("Tags"))
            .child(
                h_flex()
                    .gap_2()
                    .flex_wrap()
                    .child(Tag::primary().child("Primary"))
                    .child(Tag::secondary().child("Secondary"))
                    .child(Tag::danger().child("Danger"))
                    .child(Tag::success().child("Success"))
                    .child(Tag::warning().child("Warning"))
                    .child(Tag::info().child("Info"))
                    .child(Tag::primary().outline().child("Outline")),
            )
            // Badges
            .child(self.section_header("Badges"))
            .child(
                h_flex()
                    .gap_8()
                    .child(
                        Badge::new()
                            .count(5)
                            .child(Button::new("badge-btn-1").label("Messages")),
                    )
                    .child(
                        Badge::new()
                            .count(99)
                            .child(Button::new("badge-btn-2").label("Notifications")),
                    )
                    .child(
                        Badge::new()
                            .dot()
                            .child(Button::new("badge-btn-3").label("Updates")),
                    ),
            )
            // Breadcrumb
            .child(self.section_header("Breadcrumb Navigation"))
            .child(
                v_flex()
                    .gap_2()
                    .child(
                        Breadcrumb::new()
                            .child(BreadcrumbItem::new("Home"))
                            .child(BreadcrumbItem::new("Settings"))
                            .child(BreadcrumbItem::new("Appearance"))
                            .child(BreadcrumbItem::new("Themes")),
                    ),
            )
            // Divider styles
            .child(self.section_header("Divider Styles"))
            .child(
                v_flex()
                    .gap_3()
                    .child(Divider::horizontal())
                    .child(Divider::horizontal().label("OR"))
                    .child(Divider::horizontal_dashed())
                    .child(Divider::horizontal_dashed().label("END")),
            )
            // GroupBox with content
            .child(self.section_header("Grouped Content"))
            .child(
                GroupBox::new()
                    .title("Theme Information")
                    .fill()
                    .child(
                        v_flex()
                            .gap_2()
                            .child(Label::new("This showcase demonstrates the full integration between native-theme and gpui-component.").text_sm())
                            .child(Label::new("Switch themes above to see how all widgets respond to color changes.").text_sm()),
                    ),
            )
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------
    fn section_header(&self, title: impl Into<SharedString>) -> impl IntoElement {
        Label::new(title)
            .text_size(px(13.0))
            .font_semibold()
    }
}

impl Render for Showcase {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let active_tab = self.active_tab;
        let is_dark = self.is_dark;
        let theme_name_display: SharedString = if self.current_theme_name == "OS Theme" {
            "OS Theme".into()
        } else {
            format!(
                "{} ({})",
                self.current_theme_name,
                if is_dark { "dark" } else { "light" }
            )
            .into()
        };

        v_flex()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            // ---- Header ----
            .child(
                v_flex()
                    .px_4()
                    .pt_4()
                    .pb_2()
                    .gap_3()
                    // Title row
                    .child(
                        h_flex()
                            .justify_between()
                            .items_center()
                            .child(
                                v_flex()
                                    .child(
                                        Label::new("native-theme-gpui Showcase")
                                            .text_size(px(20.0))
                                            .font_semibold(),
                                    )
                                    .child(
                                        Label::new(theme_name_display)
                                            .text_size(px(11.0))
                                            .text_color(theme.muted_foreground),
                                    ),
                            )
                            .child(
                                h_flex()
                                    .gap_3()
                                    .items_center()
                                    // Dark mode switch
                                    .child(
                                        Switch::new("dark-mode")
                                            .label("Dark")
                                            .with_size(Size::Small)
                                            .checked(is_dark)
                                            .on_click(cx.listener(
                                                |this, val: &bool, window, cx| {
                                                    this.toggle_dark_mode(*val, window, cx);
                                                },
                                            )),
                                    )
                                    // Theme selector
                                    .child(
                                        Select::new(&self.theme_select)
                                            .with_size(Size::Small)
                                            .w(px(180.0)),
                                    ),
                            ),
                    )
                    // Navigation tabs
                    .child(
                        TabBar::new("nav")
                            .underline()
                            .with_size(Size::Medium)
                            .child("Buttons")
                            .child("Inputs")
                            .child("Data")
                            .child("Feedback")
                            .child("Typography")
                            .selected_index(active_tab)
                            .on_click(cx.listener(
                                |this, ix: &usize, _window, _cx| {
                                    this.active_tab = *ix;
                                },
                            )),
                    ),
            )
            // ---- Content ----
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .child(match active_tab {
                        TAB_BUTTONS => self.render_buttons_tab(cx).into_any_element(),
                        TAB_INPUTS => self.render_inputs_tab(cx).into_any_element(),
                        TAB_DATA => self.render_data_tab(cx).into_any_element(),
                        TAB_FEEDBACK => self.render_feedback_tab(cx).into_any_element(),
                        TAB_TYPOGRAPHY => self.render_typography_tab(cx).into_any_element(),
                        _ => self.render_buttons_tab(cx).into_any_element(),
                    }),
            )
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(800.), px(900.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let showcase = cx.new(|cx| Showcase::new(window, cx));
                cx.new(|cx| Root::new(showcase, window, cx))
            },
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn main() {
    eprintln!("gpui showcase requires macOS or Linux");
}
