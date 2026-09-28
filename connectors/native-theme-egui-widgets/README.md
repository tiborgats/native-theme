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
| `wrap::link`, `wrap::hyperlink` | a link in its hover, pressed, disabled and visited colours |
| `combo_box::ComboBox` | a drop-down at the height the theme states: egui's square arrow box can make its own taller |

Every widget reads the `ResolvedTheme` of the atlas the connector installed,
in the colour scheme egui is drawing, and contains no colour, radius or
metric of its own. With no atlas installed, each renders as egui's own
counterpart.

```rust,no_run
use native_theme_egui_widgets::connector::egui;
use native_theme_egui_widgets::switch::Switch;

fn settings(ui: &mut egui::Ui, wifi_on: &mut bool) {
    ui.add(Switch::new(wifi_on).label("Wi-Fi"));
}
```

Widgets are `egui::Widget`s added with `ui.add(..)`; `ui.add_sized` and
`ui.add_enabled` take them too. There is no extension trait on `egui::Ui`:
an inherent `Ui` method of the same name would silently win at every call
site.

## Features

The connector's, forwarded one to one, with the connector's default set:
`material-icons`, `lucide-icons`, `system-icons`, `system-fonts` (default)
and `svg-rasterize`. This crate adds none of its own.

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
