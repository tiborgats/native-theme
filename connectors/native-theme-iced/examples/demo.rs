//! Comprehensive widget showcase for native-theme-iced.
//!
//! Demonstrates every styled iced widget with full theme switching (17 presets,
//! light/dark toggle) and native-theme metric helpers (border radius, padding,
//! scrollbar width). Organized with a sidebar navigation layout.
//!
//! Run with: `cargo run -p native-theme-iced --example demo`

use iced::widget::{
    button, checkbox, column, combo_box, container, pick_list, progress_bar, radio, row, rule,
    scrollable, slider, space, text, text_editor, text_input, toggler, tooltip, vertical_slider,
};
use iced::{Element, Fill, Length, Padding, Theme};

use native_theme::NativeTheme;

// ---------------------------------------------------------------------------
// Sidebar page identifiers
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Buttons,
    TextInputs,
    Selection,
    Range,
    Display,
}

impl Page {
    const ALL: &[Page] = &[
        Page::Buttons,
        Page::TextInputs,
        Page::Selection,
        Page::Range,
        Page::Display,
    ];

    fn label(self) -> &'static str {
        match self {
            Page::Buttons => "Buttons",
            Page::TextInputs => "Text Inputs",
            Page::Selection => "Selection",
            Page::Range => "Range",
            Page::Display => "Display",
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Page::Buttons => "\u{25A3}",    // filled square
            Page::TextInputs => "\u{270E}", // pencil
            Page::Selection => "\u{2611}",  // ballot box
            Page::Range => "\u{2194}",      // left-right arrow
            Page::Display => "\u{25A8}",    // hatched square
        }
    }
}

// ---------------------------------------------------------------------------
// ThemeChoice enum
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum ThemeChoice {
    OsTheme,
    Preset(String),
}

impl std::fmt::Display for ThemeChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeChoice::OsTheme => write!(f, "OS Theme"),
            ThemeChoice::Preset(name) => write!(f, "{name}"),
        }
    }
}

impl PartialEq for ThemeChoice {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ThemeChoice::OsTheme, ThemeChoice::OsTheme) => true,
            (ThemeChoice::Preset(a), ThemeChoice::Preset(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for ThemeChoice {}

fn theme_choices() -> Vec<ThemeChoice> {
    let mut choices = vec![ThemeChoice::OsTheme];
    choices.extend(
        NativeTheme::list_presets()
            .iter()
            .map(|name| ThemeChoice::Preset((*name).to_string())),
    );
    choices
}

// ---------------------------------------------------------------------------
// Radio demo choice
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Cherry,
}

impl std::fmt::Display for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fruit::Apple => write!(f, "Apple"),
            Fruit::Banana => write!(f, "Banana"),
            Fruit::Cherry => write!(f, "Cherry"),
        }
    }
}

// ---------------------------------------------------------------------------
// Application state
// ---------------------------------------------------------------------------

struct State {
    // Theme
    current_choice: ThemeChoice,
    current_theme: Theme,
    is_dark: bool,
    current_variant: native_theme::ThemeVariant,

    // Navigation
    active_page: Page,

    // Button page
    button_press_count: u32,

    // Text input page
    text_input_value: String,
    text_editor_content: text_editor::Content,

    // Selection page
    checkbox_a: bool,
    checkbox_b: bool,
    checkbox_c: bool,
    selected_fruit: Option<Fruit>,
    toggler_enabled: bool,
    pick_list_selected: Option<String>,
    combo_state: combo_box::State<String>,
    combo_selected: Option<String>,

    // Range page
    slider_value: f32,
    slider_step: f32,
    vslider_value: f32,
    progress_value: f32,
}

impl Default for State {
    fn default() -> Self {
        let preset_name = "default";
        let nt = NativeTheme::preset(preset_name).expect("default preset must exist");
        let is_dark = false;
        let variant = native_theme_iced::pick_variant(&nt, is_dark)
            .expect("must have a variant")
            .clone();
        let theme = native_theme_iced::to_theme(&variant, &nt.name);

        let languages = vec![
            "Rust".to_string(),
            "Python".to_string(),
            "JavaScript".to_string(),
            "TypeScript".to_string(),
            "Go".to_string(),
            "C++".to_string(),
            "Java".to_string(),
            "Swift".to_string(),
            "Kotlin".to_string(),
            "Zig".to_string(),
        ];

        Self {
            current_choice: ThemeChoice::Preset(preset_name.to_string()),
            current_theme: theme,
            is_dark,
            current_variant: variant,
            active_page: Page::Buttons,
            button_press_count: 0,
            text_input_value: String::new(),
            text_editor_content: text_editor::Content::with_text(
                "This is a multi-line text editor.\nEdit this text freely.\n\nIt supports:\n  - Multiple lines\n  - Scrolling\n  - Selection",
            ),
            checkbox_a: true,
            checkbox_b: false,
            checkbox_c: true,
            selected_fruit: Some(Fruit::Apple),
            toggler_enabled: false,
            pick_list_selected: Some("Rust".to_string()),
            combo_state: combo_box::State::new(languages),
            combo_selected: None,
            slider_value: 65.0,
            slider_step: 25.0,
            vslider_value: 50.0,
            progress_value: 72.0,
        }
    }
}

impl State {
    fn rebuild_theme(&mut self) {
        let nt = match &self.current_choice {
            ThemeChoice::OsTheme => {
                native_theme::from_system().unwrap_or_else(|_| {
                    NativeTheme::preset("default").expect("default preset must exist")
                })
            }
            ThemeChoice::Preset(name) => NativeTheme::preset(name).unwrap(),
        };
        if let Some(variant) = native_theme_iced::pick_variant(&nt, self.is_dark) {
            self.current_variant = variant.clone();
            self.current_theme = native_theme_iced::to_theme(variant, &nt.name);
        }
    }
}

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Message {
    // Navigation
    PageSelected(Page),

    // Theme
    ThemeSelected(ThemeChoice),
    DarkModeToggled(bool),

    // Button page
    ButtonPressed,

    // Text input page
    TextInputChanged(String),
    EditorAction(text_editor::Action),

    // Selection page
    CheckboxAToggled(bool),
    CheckboxBToggled(bool),
    CheckboxCToggled(bool),
    FruitSelected(Fruit),
    TogglerToggled(bool),
    PickListSelected(String),
    ComboBoxSelected(String),

    // Range page
    SliderChanged(f32),
    StepSliderChanged(f32),
    VSliderChanged(f32),
    ProgressChanged(f32),
}

// ---------------------------------------------------------------------------
// Update
// ---------------------------------------------------------------------------

fn update(state: &mut State, message: Message) {
    match message {
        Message::PageSelected(page) => {
            state.active_page = page;
        }
        Message::ThemeSelected(choice) => {
            state.current_choice = choice;
            state.rebuild_theme();
        }
        Message::DarkModeToggled(is_dark) => {
            state.is_dark = is_dark;
            state.rebuild_theme();
        }
        Message::ButtonPressed => {
            state.button_press_count += 1;
        }
        Message::TextInputChanged(value) => {
            state.text_input_value = value;
        }
        Message::EditorAction(action) => {
            state.text_editor_content.perform(action);
        }
        Message::CheckboxAToggled(v) => state.checkbox_a = v,
        Message::CheckboxBToggled(v) => state.checkbox_b = v,
        Message::CheckboxCToggled(v) => state.checkbox_c = v,
        Message::FruitSelected(fruit) => state.selected_fruit = Some(fruit),
        Message::TogglerToggled(v) => state.toggler_enabled = v,
        Message::PickListSelected(v) => state.pick_list_selected = Some(v),
        Message::ComboBoxSelected(v) => state.combo_selected = Some(v),
        Message::SliderChanged(v) => state.slider_value = v,
        Message::StepSliderChanged(v) => state.slider_step = v,
        Message::VSliderChanged(v) => state.vslider_value = v,
        Message::ProgressChanged(v) => state.progress_value = v,
    }
}

// ---------------------------------------------------------------------------
// View
// ---------------------------------------------------------------------------

fn view(state: &State) -> Element<'_, Message> {
    let radius = native_theme_iced::border_radius(&state.current_variant);
    let _radius_lg = native_theme_iced::border_radius_lg(&state.current_variant);
    let sb_width = native_theme_iced::scrollbar_width(&state.current_variant);
    let btn_pad = native_theme_iced::button_padding(&state.current_variant);
    let inp_pad = native_theme_iced::input_padding(&state.current_variant);

    // ---- Sidebar ----
    let sidebar = {
        let title = text("native-theme").size(18);

        let subtitle = text("iced showcase").size(12);

        let nav_buttons: Vec<Element<'_, Message>> = Page::ALL
            .iter()
            .map(|&page| {
                let label = format!("  {}  {}", page.icon(), page.label());
                let btn = button(text(label).size(14)).width(Fill);
                let btn = if page == state.active_page {
                    btn.style(button::primary)
                } else {
                    btn.style(button::secondary)
                };
                btn.on_press(Message::PageSelected(page)).into()
            })
            .collect();

        let theme_picker = column![
            text("Theme").size(12),
            pick_list(
                theme_choices(),
                Some(&state.current_choice),
                Message::ThemeSelected,
            )
            .width(Fill),
        ]
        .spacing(4);

        let dark_toggle = toggler(state.is_dark)
            .label("Dark mode")
            .on_toggle(Message::DarkModeToggled);

        let metrics_info = {
            let r = format!("Border radius: {radius:.0}px");
            let sw = format!("Scrollbar: {sb_width:.0}px");
            let bp = match btn_pad {
                Some([h, v]) => format!("Btn pad: {h:.0} x {v:.0}"),
                None => "Btn pad: default".to_string(),
            };
            let ip = match inp_pad {
                Some([h, v]) => format!("Input pad: {h:.0} x {v:.0}"),
                None => "Input pad: default".to_string(),
            };
            column![
                text("Metrics").size(12),
                text(r).size(11),
                text(sw).size(11),
                text(bp).size(11),
                text(ip).size(11),
            ]
            .spacing(2)
        };

        container(
            column![
                title,
                subtitle,
                rule::horizontal(1),
                column(nav_buttons).spacing(4),
                rule::horizontal(1),
                theme_picker,
                dark_toggle,
                rule::horizontal(1),
                metrics_info,
            ]
            .spacing(10)
            .padding(Padding::from(12))
            .width(Length::Fixed(200.0)),
        )
        .style(container::rounded_box)
        .height(Fill)
    };

    // ---- Main content area ----
    let page_content: Element<'_, Message> = match state.active_page {
        Page::Buttons => view_buttons(state, btn_pad),
        Page::TextInputs => view_text_inputs(state, radius, inp_pad),
        Page::Selection => view_selection(state),
        Page::Range => view_range(state),
        Page::Display => view_display(state, radius),
    };

    let main_area = container(
        scrollable(
            container(page_content)
                .padding(Padding::from(24))
                .width(Fill),
        )
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::new().width(sb_width).scroller_width(sb_width),
        ))
        .height(Fill),
    )
    .width(Fill)
    .height(Fill);

    row![sidebar, main_area].into()
}

// ---------------------------------------------------------------------------
// Page: Buttons
// ---------------------------------------------------------------------------

fn view_buttons<'a>(state: &'a State, btn_pad: Option<[f32; 2]>) -> Element<'a, Message> {
    let apply_pad = |b: button::Button<'a, Message>| -> button::Button<'a, Message> {
        match btn_pad {
            Some([h, v]) => b.padding(Padding::from([v, h])),
            None => b,
        }
    };

    let header = section_header("Buttons", "Interactive button styles from the theme palette");

    let primary_row = column![
        text("Primary Actions").size(16),
        row![
            apply_pad(button("Primary").on_press(Message::ButtonPressed).style(button::primary)),
            apply_pad(button("Secondary").on_press(Message::ButtonPressed).style(button::secondary)),
            apply_pad(button("Success").on_press(Message::ButtonPressed).style(button::success)),
            apply_pad(button("Danger").on_press(Message::ButtonPressed).style(button::danger)),
            apply_pad(button("Text Style").on_press(Message::ButtonPressed).style(button::text)),
        ]
        .spacing(8),
    ]
    .spacing(8);

    let disabled_row = column![
        text("Disabled State").size(16),
        text("Buttons without on_press are rendered as disabled:").size(13),
        row![
            apply_pad(button("Disabled Primary").style(button::primary)),
            apply_pad(button("Disabled Secondary").style(button::secondary)),
            apply_pad(button("Disabled Danger").style(button::danger)),
        ]
        .spacing(8),
    ]
    .spacing(8);

    let counter_text = format!(
        "Button presses this session: {}",
        state.button_press_count
    );

    let interactive = column![
        text("Interactive Demo").size(16),
        row![
            apply_pad(
                button(text("Click me!").size(14))
                    .on_press(Message::ButtonPressed)
                    .style(button::primary)
            ),
            text(counter_text).size(14),
        ]
        .spacing(12)
        .align_y(iced::Center),
    ]
    .spacing(8);

    column![
        header,
        primary_row,
        rule::horizontal(1),
        disabled_row,
        rule::horizontal(1),
        interactive,
    ]
    .spacing(20)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Page: Text Inputs
// ---------------------------------------------------------------------------

fn view_text_inputs<'a>(
    state: &'a State,
    radius: f32,
    inp_pad: Option<[f32; 2]>,
) -> Element<'a, Message> {
    let header = section_header(
        "Text Inputs",
        "Single-line TextInput and multi-line TextEditor",
    );

    let single_line = {
        let mut input = text_input("Type something here...", &state.text_input_value)
            .on_input(Message::TextInputChanged);
        if let Some([h, v]) = inp_pad {
            input = input.padding(Padding::from([v, h]));
        }

        column![
            text("TextInput (single line)").size(16),
            input,
            text(format!(
                "Characters: {}  |  Border radius from theme: {radius:.0}px",
                state.text_input_value.len()
            ))
            .size(12),
        ]
        .spacing(8)
    };

    let secure_input = {
        let mut input = text_input("Password field...", &state.text_input_value)
            .on_input(Message::TextInputChanged)
            .secure(true);
        if let Some([h, v]) = inp_pad {
            input = input.padding(Padding::from([v, h]));
        }

        column![
            text("TextInput (secure / password)").size(16),
            input,
        ]
        .spacing(8)
    };

    let multi_line = column![
        text("TextEditor (multi-line)").size(16),
        text_editor(&state.text_editor_content)
            .on_action(Message::EditorAction)
            .height(Length::Fixed(180.0)),
        text("Supports multi-line editing, selection, and scrolling").size(12),
    ]
    .spacing(8);

    column![
        header,
        single_line,
        rule::horizontal(1),
        secure_input,
        rule::horizontal(1),
        multi_line,
    ]
    .spacing(20)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Page: Selection
// ---------------------------------------------------------------------------

fn view_selection(state: &State) -> Element<'_, Message> {
    let header = section_header(
        "Selection Widgets",
        "Checkbox, Radio, Toggler, PickList, and ComboBox",
    );

    let checkboxes = column![
        text("Checkboxes").size(16),
        checkbox(state.checkbox_a)
            .label("Enable notifications")
            .on_toggle(Message::CheckboxAToggled),
        checkbox(state.checkbox_b)
            .label("Dark mode auto-detect")
            .on_toggle(Message::CheckboxBToggled),
        checkbox(state.checkbox_c)
            .label("Remember preferences")
            .on_toggle(Message::CheckboxCToggled),
        text(format!(
            "Checked: {}",
            [
                state.checkbox_a.then_some("notifications"),
                state.checkbox_b.then_some("auto-detect"),
                state.checkbox_c.then_some("remember"),
            ]
            .iter()
            .flatten()
            .copied()
            .collect::<Vec<_>>()
            .join(", ")
        ))
        .size(12),
    ]
    .spacing(8);

    let radios = column![
        text("Radio Buttons").size(16),
        radio("Apple", Fruit::Apple, state.selected_fruit, Message::FruitSelected),
        radio("Banana", Fruit::Banana, state.selected_fruit, Message::FruitSelected),
        radio("Cherry", Fruit::Cherry, state.selected_fruit, Message::FruitSelected),
        text(format!(
            "Selected: {}",
            state
                .selected_fruit
                .map(|f| f.to_string())
                .unwrap_or_else(|| "None".to_string())
        ))
        .size(12),
    ]
    .spacing(8);

    let togglers = column![
        text("Toggler (Switch)").size(16),
        toggler(state.toggler_enabled)
            .label("Feature flag enabled")
            .on_toggle(Message::TogglerToggled),
        text(format!(
            "State: {}",
            if state.toggler_enabled { "ON" } else { "OFF" }
        ))
        .size(12),
    ]
    .spacing(8);

    let languages: Vec<String> = vec![
        "Rust", "Python", "JavaScript", "TypeScript", "Go", "C++", "Java", "Swift",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let pickers = column![
        text("PickList (dropdown)").size(16),
        pick_list(
            languages,
            state.pick_list_selected.as_ref(),
            Message::PickListSelected,
        )
        .width(Length::Fixed(250.0)),
        text(format!(
            "Selected: {}",
            state
                .pick_list_selected
                .as_deref()
                .unwrap_or("None")
        ))
        .size(12),
    ]
    .spacing(8);

    let combos = column![
        text("ComboBox (searchable dropdown)").size(16),
        combo_box(
            &state.combo_state,
            "Search a language...",
            state.combo_selected.as_ref(),
            Message::ComboBoxSelected,
        )
        .width(Length::Fixed(250.0)),
        text(format!(
            "Selected: {}",
            state.combo_selected.as_deref().unwrap_or("None")
        ))
        .size(12),
    ]
    .spacing(8);

    column![
        header,
        row![
            column![checkboxes, rule::horizontal(1), togglers,].spacing(20).width(Fill),
            rule::vertical(1),
            column![radios, rule::horizontal(1), pickers, rule::horizontal(1), combos,].spacing(20).width(Fill),
        ]
        .spacing(20),
    ]
    .spacing(20)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Page: Range
// ---------------------------------------------------------------------------

fn view_range(state: &State) -> Element<'_, Message> {
    let header = section_header(
        "Range Widgets",
        "Slider, VerticalSlider, and ProgressBar",
    );

    let horiz_slider = column![
        text("Horizontal Slider").size(16),
        row![
            slider(0.0..=100.0, state.slider_value, Message::SliderChanged).width(Fill),
            text(format!("{:.1}", state.slider_value)).size(14).width(Length::Fixed(50.0)),
        ]
        .spacing(12)
        .align_y(iced::Center),
        text("Drag to change value. This slider drives the first progress bar below.").size(12),
    ]
    .spacing(8);

    let step_slider = column![
        text("Slider with Step (5-unit increments)").size(16),
        row![
            slider(0.0..=100.0, state.slider_step, Message::StepSliderChanged)
                .step(5.0)
                .width(Fill),
            text(format!("{:.0}", state.slider_step)).size(14).width(Length::Fixed(50.0)),
        ]
        .spacing(12)
        .align_y(iced::Center),
    ]
    .spacing(8);

    let vert_slider = column![
        text("Vertical Slider").size(16),
        row![
            container(
                vertical_slider(0.0..=100.0, state.vslider_value, Message::VSliderChanged)
                    .height(Length::Fixed(200.0))
            )
            .center_x(Length::Fixed(60.0)),
            column![
                text(format!("Value: {:.1}", state.vslider_value)).size(14),
                space().height(Length::Fixed(8.0)),
                text("Vertical sliders are useful\nfor volume controls,\nequalizers, etc.").size(12),
            ]
            .spacing(4),
        ]
        .spacing(16),
    ]
    .spacing(8);

    let progress = column![
        text("Progress Bars").size(16),
        text("Driven by horizontal slider value:").size(13),
        progress_bar(0.0..=100.0, state.slider_value),
        space().height(Length::Fixed(4.0)),
        text("Separate progress control:").size(13),
        row![
            slider(0.0..=100.0, state.progress_value, Message::ProgressChanged).width(Fill),
            text(format!("{:.0}%", state.progress_value)).size(14).width(Length::Fixed(50.0)),
        ]
        .spacing(12)
        .align_y(iced::Center),
        progress_bar(0.0..=100.0, state.progress_value),
    ]
    .spacing(6);

    column![
        header,
        horiz_slider,
        rule::horizontal(1),
        step_slider,
        rule::horizontal(1),
        vert_slider,
        rule::horizontal(1),
        progress,
    ]
    .spacing(20)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Page: Display
// ---------------------------------------------------------------------------

fn view_display<'a>(state: &'a State, radius: f32) -> Element<'a, Message> {
    let header = section_header(
        "Display Widgets",
        "Container, Rule, Tooltip, and layout helpers",
    );

    let containers = column![
        text("Styled Containers").size(16),
        container(
            column![
                text("Rounded Box Container").size(14),
                text(format!(
                    "This container uses the theme's rounded_box style. \
                     Border radius from theme metrics: {radius:.0}px."
                ))
                .size(12),
            ]
            .spacing(4),
        )
        .padding(Padding::from(16))
        .style(container::rounded_box)
        .width(Fill),
        container(
            text("A secondary container with different padding. Containers adapt their \
                  background and border colors from the active theme palette.")
                .size(12),
        )
        .padding(Padding::from([12, 20]))
        .style(container::rounded_box)
        .width(Fill),
    ]
    .spacing(10);

    let rules = column![
        text("Divider Rules").size(16),
        text("Horizontal rules at various thicknesses:").size(13),
        rule::horizontal(1),
        text("1px above, 2px below").size(11),
        rule::horizontal(2),
        text("2px above, 4px below").size(11),
        rule::horizontal(4),
    ]
    .spacing(6);

    let tooltips = column![
        text("Tooltips").size(16),
        row![
            tooltip(
                button("Hover: Top")
                    .on_press(Message::ButtonPressed)
                    .style(button::primary),
                text("Tooltip on top!"),
                tooltip::Position::Top,
            )
            .gap(5)
            .style(container::rounded_box),
            tooltip(
                button("Hover: Bottom")
                    .on_press(Message::ButtonPressed)
                    .style(button::secondary),
                text("Tooltip on bottom!"),
                tooltip::Position::Bottom,
            )
            .gap(5)
            .style(container::rounded_box),
            tooltip(
                button("Hover: Left")
                    .on_press(Message::ButtonPressed)
                    .style(button::success),
                text("Tooltip on left!"),
                tooltip::Position::Left,
            )
            .gap(5)
            .style(container::rounded_box),
            tooltip(
                button("Hover: Right")
                    .on_press(Message::ButtonPressed)
                    .style(button::danger),
                text("Tooltip on right!"),
                tooltip::Position::Right,
            )
            .gap(5)
            .style(container::rounded_box),
        ]
        .spacing(10),
    ]
    .spacing(8);

    let theme_info_text = format!(
        "Active theme: {}  |  Mode: {}",
        state.current_theme.to_string(),
        if state.is_dark { "Dark" } else { "Light" },
    );

    let info_box = container(
        column![
            text("Theme Information").size(14),
            text(theme_info_text).size(12),
            text(format!(
                "Available presets: {} | All presets have both light and dark variants.",
                NativeTheme::list_presets().len(),
            ))
            .size(12),
        ]
        .spacing(4),
    )
    .padding(Padding::from(16))
    .style(container::rounded_box)
    .width(Fill);

    let spacing_demo = column![
        text("Spacing & Layout").size(16),
        row![
            container(text("A").size(14))
                .padding(Padding::from(12))
                .style(container::rounded_box)
                .center_x(Length::Fixed(60.0))
                .center_y(Length::Fixed(60.0)),
            container(text("B").size(14))
                .padding(Padding::from(12))
                .style(container::rounded_box)
                .center_x(Length::Fixed(60.0))
                .center_y(Length::Fixed(60.0)),
            container(text("C").size(14))
                .padding(Padding::from(12))
                .style(container::rounded_box)
                .center_x(Length::Fixed(60.0))
                .center_y(Length::Fixed(60.0)),
            space().width(Fill),
            container(text("Right-aligned").size(12))
                .padding(Padding::from(12))
                .style(container::rounded_box),
        ]
        .spacing(8)
        .align_y(iced::Center),
    ]
    .spacing(8);

    column![
        header,
        containers,
        rule::horizontal(1),
        rules,
        rule::horizontal(1),
        tooltips,
        rule::horizontal(1),
        spacing_demo,
        rule::horizontal(1),
        info_box,
    ]
    .spacing(20)
    .width(Fill)
    .into()
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn section_header<'a>(title: &'a str, description: &'a str) -> Element<'a, Message> {
    column![
        text(title).size(24),
        text(description).size(13),
        rule::horizontal(2),
    ]
    .spacing(4)
    .into()
}

// ---------------------------------------------------------------------------
// Theme
// ---------------------------------------------------------------------------

fn theme(state: &State) -> Theme {
    state.current_theme.clone()
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() -> iced::Result {
    iced::application(State::default, update, view)
        .title("native-theme-iced Showcase")
        .theme(theme)
        .window_size((920.0, 700.0))
        .centered()
        .run()
}
