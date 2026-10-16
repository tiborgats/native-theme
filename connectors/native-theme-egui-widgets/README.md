# native-theme-egui-widgets

egui widgets drawn from a native theme, for applications themed with
[`native-theme-egui`](../native-theme-egui/README.md).

The connector maps a `native_theme` theme onto egui's own `Style` and its role
scopes. A few native pixels no `Style` can produce, because egui draws them
from a hardcoded value or does not draw them at all. This crate draws those:

| widget | what egui cannot give its own widgets |
|---|---|
| `switch::Switch` | a switch: egui has no switch or toggle widget |
| `slider::Slider` | a slider knob in its own colour: egui paints the resting knob in the rail's |
| `spinner::Spinner` | a spinner at the theme's stroke: `egui::Spinner` hardcodes its stroke width; where the icon set has an animated indicator, that is drawn |
| `segmented_control::SegmentedControl` | a segmented control: egui has none |
| `links::link`, `links::hyperlink` | a link in its hover, pressed, disabled and visited colours |
| `combo_box::ComboBox` | a drop-down at the height the theme states, with the open chevron the platforms draw: egui's square arrow box can make its own taller, and its arrow is a filled triangle |
| `radio_button::RadioButton` | a radio button's dot at the size the theme states: egui sizes it from the check mark's box |
| `progress_bar::ProgressBar` | a progress bar with the outline the theme states: egui's draws none |
| `expander::Expander` | an expander with the arrow side, arrow gap, body indent and frame the theme states: egui's `CollapsingHeader` ties the arrow, the title and the body to one indent, leads with its arrow and frames the header alone |

Every widget reads the `ResolvedTheme` of the atlas the connector installed,
in the colour scheme egui is drawing, and contains no colour, radius or
metric of its own. With no atlas installed, each renders as egui's own
counterpart.

## Quick start

Add the connector, with the OS readers (`native`), and this crate:

```toml
[dependencies]
eframe = "0.36.2"
native-theme-egui = { version = "0.6", features = ["native"] }
native-theme-egui-widgets = "0.6"
```

Install the connector's atlas (its README's Quick start), then add a widget:

```rust,no_run
use native_theme_egui_widgets::connector::egui;
use native_theme_egui_widgets::switch::Switch;

fn settings(ui: &mut egui::Ui, wifi_on: &mut bool) {
    ui.add(Switch::new(wifi_on).label("Wi-Fi"));
}
```

Every widget but the drop-down and the expander is an `egui::Widget` added
with `ui.add(..)` (`ui.add_sized` and `ui.add_enabled` take them too); those
two, whose contents are a closure, are shown with `ComboBox::show_ui` and
`Expander::show`. There is no extension trait on `egui::Ui`:
an inherent `Ui` method of the same name would silently win at every call
site.

## Features

The connector's, forwarded one to one, with the connector's default set:
`material-icons`, `lucide-icons`, `system-icons`, `system-fonts` (default),
and `svg-rasterize`, `native` (the OS readers) and `watch` (`ThemeWatcher`),
off by default as there. This crate adds none of its own.

## Compatibility

Built for egui 0.36.2, which it reaches through the connector, with
`rust-version = "1.95"`, as the connector; the connector's
[Compatibility](../native-theme-egui/README.md#compatibility) section has the
floors and the versions verified.

## Limits

* Existing calls to egui's widgets do not change; each call site adopts a
  widget of this crate with `ui.add(..)`.
* Third-party egui crates are unaffected.
* A bare `link` has no visited colour (egui's `Link` carries no URL), and
  egui underlines every link on hover or focus.
* On macOS the spinner is an arc, not fins: the theme states no fin geometry.
* A segmented control's dividers are its border's colour: the theme states
  their width and no colour of their own.
* No slider tick marks.

## License

MIT OR Apache-2.0 OR 0BSD
