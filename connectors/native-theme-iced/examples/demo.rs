//! Widget gallery demo for native-theme-iced.
//!
//! Demonstrates all 8 core styled widgets with a theme selector dropdown.
//!
//! Run with: `cargo run -p native-theme-iced --example demo`

use iced::widget::{
    button, checkbox, column, container, pick_list, progress_bar, row, rule, scrollable, slider,
    text, text_input, tooltip,
};
use iced::{Element, Fill, Length, Padding, Theme};

use native_theme::NativeTheme;

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
    for name in NativeTheme::list_presets() {
        choices.push(ThemeChoice::Preset((*name).to_string()));
    }
    choices
}

// ---------------------------------------------------------------------------
// Application state
// ---------------------------------------------------------------------------

struct State {
    current_choice: ThemeChoice,
    current_theme: Theme,
    text_input_value: String,
    checkbox_checked: bool,
    slider_value: f32,
}

impl Default for State {
    fn default() -> Self {
        let nt = NativeTheme::preset("default").expect("default preset must exist");
        let variant = native_theme_iced::pick_variant(&nt, false).expect("must have a variant");
        let theme = native_theme_iced::to_theme(variant, "default");

        Self {
            current_choice: ThemeChoice::Preset("default".to_string()),
            current_theme: theme,
            text_input_value: String::new(),
            checkbox_checked: false,
            slider_value: 50.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Message {
    ThemeSelected(ThemeChoice),
    TextInputChanged(String),
    CheckboxToggled(bool),
    SliderChanged(f32),
    ButtonPressed,
}

// ---------------------------------------------------------------------------
// Update
// ---------------------------------------------------------------------------

fn update(state: &mut State, message: Message) {
    match message {
        Message::ThemeSelected(choice) => {
            let nt = match &choice {
                ThemeChoice::OsTheme => NativeTheme::preset("default").unwrap(),
                ThemeChoice::Preset(name) => NativeTheme::preset(name).unwrap(),
            };

            let is_dark = false;
            if let Some(variant) = native_theme_iced::pick_variant(&nt, is_dark) {
                let label = match &choice {
                    ThemeChoice::OsTheme => "OS Theme",
                    ThemeChoice::Preset(name) => name.as_str(),
                };
                state.current_theme = native_theme_iced::to_theme(variant, label);
            }

            state.current_choice = choice;
        }
        Message::TextInputChanged(value) => {
            state.text_input_value = value;
        }
        Message::CheckboxToggled(checked) => {
            state.checkbox_checked = checked;
        }
        Message::SliderChanged(value) => {
            state.slider_value = value;
        }
        Message::ButtonPressed => {
            // No-op for demo purposes
        }
    }
}

// ---------------------------------------------------------------------------
// View
// ---------------------------------------------------------------------------

fn view(state: &State) -> Element<'_, Message> {
    let title = text("native-theme-iced Demo").size(28);

    // Theme selector
    let theme_selector = row![
        text("Theme: ").size(16),
        pick_list(theme_choices(), Some(&state.current_choice), Message::ThemeSelected),
    ]
    .spacing(10)
    .align_y(iced::Center);

    let separator = rule::horizontal(2);

    // -- Buttons section --
    let buttons_section = column![
        text("Buttons").size(20),
        row![
            button("Primary")
                .on_press(Message::ButtonPressed)
                .style(button::primary),
            button("Secondary")
                .on_press(Message::ButtonPressed)
                .style(button::secondary),
            button("Success")
                .on_press(Message::ButtonPressed)
                .style(button::success),
            button("Danger")
                .on_press(Message::ButtonPressed)
                .style(button::danger),
        ]
        .spacing(10),
    ]
    .spacing(8);

    // -- Text Input section --
    let text_input_section = column![
        text("Text Input").size(20),
        text_input("Type something here...", &state.text_input_value)
            .on_input(Message::TextInputChanged),
    ]
    .spacing(8);

    // -- Checkbox section --
    let checkbox_section = column![
        text("Checkbox").size(20),
        checkbox(state.checkbox_checked)
            .label("Enable feature")
            .on_toggle(Message::CheckboxToggled),
    ]
    .spacing(8);

    // -- Slider section --
    let slider_section = column![
        text("Slider").size(20),
        row![
            slider(0.0..=100.0, state.slider_value, Message::SliderChanged),
            text(format!("{:.0}", state.slider_value)).size(14),
        ]
        .spacing(10)
        .align_y(iced::Center),
    ]
    .spacing(8);

    // -- Progress Bar section --
    let progress_section = column![
        text("Progress Bar").size(20),
        progress_bar(0.0..=100.0, state.slider_value),
    ]
    .spacing(8);

    // -- Tooltip section --
    let tooltip_section = column![
        text("Tooltip").size(20),
        tooltip(
            button("Hover me").on_press(Message::ButtonPressed),
            text("This is a tooltip!"),
            tooltip::Position::Top,
        )
        .gap(5),
    ]
    .spacing(8);

    // -- Container section --
    let container_section = column![
        text("Container").size(20),
        container(
            text("This is sample text inside a styled container. It demonstrates border and padding."),
        )
        .padding(Padding::from(16))
        .style(container::rounded_box)
        .width(Fill),
    ]
    .spacing(8);

    // Compose all sections in a scrollable column
    let content = column![
        title,
        theme_selector,
        separator,
        buttons_section,
        rule::horizontal(1),
        text_input_section,
        rule::horizontal(1),
        checkbox_section,
        rule::horizontal(1),
        slider_section,
        rule::horizontal(1),
        progress_section,
        rule::horizontal(1),
        tooltip_section,
        rule::horizontal(1),
        container_section,
    ]
    .spacing(16)
    .padding(Padding::from(20))
    .max_width(600);

    scrollable(container(content).center_x(Length::Fill)).into()
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
        .title("native-theme-iced Demo")
        .theme(theme)
        .window_size((700.0, 800.0))
        .centered()
        .run()
}
