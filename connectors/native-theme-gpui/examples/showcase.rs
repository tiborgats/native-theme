//! native-theme-gpui Showcase
//!
//! A widget gallery demonstrating native-theme integration with gpui-component.
//! Displays themed Button, Input, Checkbox, Switch, Slider, and Progress widgets
//! with a dropdown to switch between all 17 native-theme presets plus OS Theme.
//!
//! # Platform requirements
//!
//! gpui requires macOS or Linux. On Linux, a GPU-capable display server
//! (X11 or Wayland with Vulkan) is needed for the window to render.
//!
//! # Running
//!
//! ```sh
//! cargo run -p native-theme-gpui --example showcase
//! ```

use gpui::{
    prelude::*, px, size, App, Application, Bounds, Context, Entity, IntoElement,
    ParentElement, Render, SharedString, Styled, Window, WindowBounds, WindowOptions,
};
use gpui_component::{
    button::Button,
    checkbox::Checkbox,
    h_flex,
    input::{Input, InputState},
    label::Label,
    progress::Progress,
    select::{SearchableVec, Select, SelectEvent, SelectState},
    slider::{Slider, SliderEvent, SliderState},
    switch::Switch,
    theme::Theme,
    v_flex, ActiveTheme, Root, Sizable, Size,
};

use gpui_component::button::ButtonVariants;

use native_theme::NativeTheme;
use native_theme_gpui::{pick_variant, to_theme};

/// Build the list of theme names: all 17 presets + "OS Theme".
fn theme_names() -> Vec<SharedString> {
    let mut names: Vec<SharedString> = NativeTheme::list_presets()
        .iter()
        .map(|s| SharedString::from(s.to_string()))
        .collect();
    names.push("OS Theme".into());
    names
}

/// The main showcase view holding all widget state.
struct Showcase {
    theme_select: Entity<SelectState<SearchableVec<SharedString>>>,
    input_state: Entity<InputState>,
    slider_state: Entity<SliderState>,
    checkbox_checked: bool,
    switch_on: bool,
    slider_value: f32,
}

impl Showcase {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let names = theme_names();
        let delegate = SearchableVec::new(names);

        // Create the select state with the first item ("default") selected.
        let theme_select = cx.new(|cx| {
            SelectState::new(
                delegate,
                Some(gpui_component::IndexPath::default().row(0)),
                window,
                cx,
            )
        });

        // Subscribe to select confirm events for theme switching.
        cx.subscribe_in(
            &theme_select,
            window,
            |this: &mut Self, _entity, event: &SelectEvent<SearchableVec<SharedString>>, window, cx| {
                if let SelectEvent::Confirm(Some(value)) = event {
                    this.apply_theme(value, window, cx);
                }
            },
        )
        .detach();

        let input_state = cx.new(|cx| {
            let mut state = InputState::new(window, cx);
            state.set_placeholder("Type something...", window, cx);
            state
        });

        let slider_state = cx.new(|_cx| SliderState::new().default_value(50.0));

        cx.subscribe_in(&slider_state, window, |this: &mut Self, _entity, event: &SliderEvent, _window, _cx| {
            let SliderEvent::Change(val) = event;
            this.slider_value = val.start();
        })
        .detach();

        // Apply the initial "default" preset theme.
        let nt = NativeTheme::preset("default").expect("default preset must exist");
        let is_dark = cx.theme().is_dark();
        if let Some(variant) = pick_variant(&nt, is_dark) {
            let theme = to_theme(variant, "default", is_dark);
            *Theme::global_mut(cx) = theme;
            window.refresh();
        }

        Self {
            theme_select,
            input_state,
            slider_state,
            checkbox_checked: false,
            switch_on: true,
            slider_value: 50.0,
        }
    }

    /// Apply the selected theme preset (or OS Theme) to the global gpui-component theme.
    fn apply_theme(&mut self, name: &SharedString, window: &mut Window, cx: &mut Context<Self>) {
        let name_str = name.to_string();

        if name_str == "OS Theme" {
            // Reset to gpui-component default by syncing with system appearance.
            Theme::sync_system_appearance(Some(window), cx);
            return;
        }

        let nt = match NativeTheme::preset(&name_str) {
            Ok(t) => t,
            Err(_) => return,
        };

        let is_dark = cx.theme().is_dark();
        if let Some(variant) = pick_variant(&nt, is_dark) {
            let theme = to_theme(variant, &name_str, is_dark);
            *Theme::global_mut(cx) = theme;
            window.refresh();
        }
    }
}

impl Render for Showcase {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .p_6()
            .gap_4()
            // Title
            .child(
                Label::new("native-theme-gpui Showcase")
                    .text_size(px(22.0)),
            )
            // Theme selector
            .child(
                v_flex()
                    .gap_1()
                    .child(Label::new("Theme"))
                    .child(
                        Select::new(&self.theme_select)
                            .with_size(Size::Medium)
                            .w(px(260.0)),
                    ),
            )
            // Buttons section
            .child(
                v_flex()
                    .gap_2()
                    .child(Label::new("Buttons"))
                    .child(
                        h_flex()
                            .gap_2()
                            .child(Button::new("btn-primary").label("Primary").primary())
                            .child(Button::new("btn-secondary").label("Secondary"))
                            .child(Button::new("btn-danger").label("Danger").danger())
                            .child(Button::new("btn-ghost").label("Ghost").ghost()),
                    ),
            )
            // Input section
            .child(
                v_flex()
                    .gap_1()
                    .child(Label::new("Text Input"))
                    .child(
                        Input::new(&self.input_state)
                            .with_size(Size::Medium)
                            .w(px(300.0)),
                    ),
            )
            // Checkbox and Switch section
            .child(
                v_flex()
                    .gap_2()
                    .child(Label::new("Toggles"))
                    .child(
                        h_flex()
                            .gap_6()
                            .child({
                                let checked = self.checkbox_checked;
                                Checkbox::new("cb-1")
                                    .label("Enable feature")
                                    .checked(checked)
                                    .on_click(cx.listener(|this, val: &bool, _window, _cx| {
                                        this.checkbox_checked = *val;
                                    }))
                            })
                            .child({
                                let on = self.switch_on;
                                Switch::new("sw-1")
                                    .label("Dark mode")
                                    .checked(on)
                                    .on_click(cx.listener(|this, val: &bool, window, cx| {
                                        this.switch_on = *val;
                                        // Toggle dark/light when the switch is flipped.
                                        let is_dark = *val;
                                        let current_name = cx.theme().theme_name().to_string();
                                        if current_name.is_empty() || current_name == "default" {
                                            // Re-apply current theme in the new mode.
                                            let presets = NativeTheme::list_presets();
                                            let preset_name = presets.first().unwrap_or(&"default");
                                            if let Ok(nt) = NativeTheme::preset(preset_name) {
                                                if let Some(variant) = pick_variant(&nt, is_dark) {
                                                    let theme = to_theme(variant, preset_name, is_dark);
                                                    *Theme::global_mut(cx) = theme;
                                                    window.refresh();
                                                }
                                            }
                                        }
                                    }))
                            }),
                    ),
            )
            // Slider section
            .child(
                v_flex()
                    .gap_1()
                    .child(Label::new(format!("Slider (value: {:.0})", self.slider_value)))
                    .child(
                        Slider::new(&self.slider_state)
                            .w(px(300.0)),
                    ),
            )
            // Progress bar section
            .child(
                v_flex()
                    .gap_1()
                    .child(Label::new("Progress"))
                    .child(
                        Progress::new()
                            .value(self.slider_value)
                            .w(px(300.0)),
                    ),
            )
    }
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn main() {
    Application::new().run(|cx: &mut App| {
        // Initialize gpui-component's theme and widget systems.
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(480.), px(620.)), cx);
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
