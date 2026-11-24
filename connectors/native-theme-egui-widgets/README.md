# native-theme-egui-widgets

egui widgets drawn from a native theme, for applications themed with
[`native-theme-egui`](../native-theme-egui/README.md).

The connector maps a `native_theme` theme onto egui's own `Style` and its role
scopes. Some native details no `Style` can produce, because egui draws them
from a hardcoded value or does not draw them at all. This crate draws those:

| widget | what egui cannot give its own widgets |
|---|---|
| `switch::Switch` | a switch: egui has no switch or toggle widget |
| `slider::Slider` | a slider knob in its own colour, with the outline the theme states at rest, hovered and focused: egui paints the resting knob in the rail's and strokes it in the colour that letters its value |
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

## Under an atlas of the 0.8.0 model

Under an atlas the connector builds from the 0.8.0 model
(`native_theme_egui::next::to_theme`), every widget reads its
`native_theme::next::ResolvedTheme`, in the colour scheme egui is drawing,
where the table of homes (`native-theme/tests/next_homes.toml`) places the
0.7.2 fields it reads: each value where the theme states it, egui's own where
it does not (docs/v0.8.0-spec.md §10, rule 0). A fill stated as none is
transparent and a stroke stated as none is no outline (rule 2).

| widget | what it reads | where the theme states none |
|---|---|---|
| `switch::Switch` | `switch.track` and `switch.thumb`: fills, outlines, sizes and the track's corners, at rest, checked, hovered and disabled; the label's colour (`switch.label`, else `typography.body.color`; hovered and pressed where a rule for the state states one), its size (`switch.label(NONE).size_px`), `switch.label_gap`, its side (`switch.label_side`: before the track where `Leading`) and the gap's stretch (`switch.label_gap_stretches`: the label and the track at the ends of a switch laid out wider than its content) | no track or checked thumb size or fill, or track corners egui cannot draw as one radius: egui's `Checkbox` |
| `radio_button::RadioButton` | `radio.dot` and `radio.indicator` widths, `radio.label_gap`; `radio.indicator`'s fill and stroke at rest, hovered and pressed, `radio.dot`'s fill, `radio.label`'s colour and `radio.disabled_opacity`, laid over its `Role::Checkbox` cell | its `Role::Checkbox` cell's circle, dot, label colour and `icon_spacing` |
| `progress_bar::ProgressBar` | `progress_bar.track`: its least width, corners and outline | each its own: egui's floor, egui's radius (half the bar's height), no outline |
| `spinner::Spinner` | `spinner.color`, `diameter`, `stroke_width`; `colors.window.foreground` for a freedesktop indicator's `currentColor` | egui's own text colour for the indicator |
| `slider::Slider` | `slider.track`, `active_track` and `thumb` fills at rest and disabled, the thumb's hover fill and its outline at rest, hovered and focused, the thumb's and the rail's sizes | each its own: egui's rail height and fill, trailing fill and knob size and fill (`egui-0.37.0/src/widgets/slider_core.rs`) |
| `segmented_control::SegmentedControl` | the hover fill of `segmented_control.segment`, whether it states vertical padding, `colors.window.background` as the ground a disabled control fades over | no ground: egui's own fade of a disabled `Ui` |
| `links::link`, `links::hyperlink` | `link.label`'s colour at rest, hovered, pressed, visited and disabled, and its underline; `link.container`'s fill and hover fill | egui's link colour, no background |
| `combo_box::ComboBox` | `combo_box.field`'s left and right padding and its stroke's width, `combo_box.arrow_area_width` | each its own: egui's padding and arrow place; for a stroke unstated or per side, the scope's outline width |
| `expander::Expander` | `expander.arrow_side`, `arrow_gap`, `content_indent`, `frame_enabled`; the header's padding, the arrow's size and colour; above a text scale of 1 the title's size (which holds the scale already) times `typography.line_height_multiplier` | none of the four layout fields: egui's `CollapsingHeader` in the expander's scope, its arrow `next::expander_icon`'s |

Limits under the 0.8.0 model:

* The switch's track draws no on and off glyphs where `switch.show_state_glyphs`
  asks for them: the model states no glyph, nor its size, colour or place on
  the track, and egui's own `Checkbox` draws none
  (`Switch::next_glyphs_unmapped` names the home).
* The switch's label is lettered at `switch.label(NONE)`'s size in egui's
  `Proportional` family, as the connector's role fonts are
  (`native_theme_egui::next::text_role_font`): that family holds the face the
  atlas's font plan installed for the body text, and a family, weight or slant
  the label states otherwise is not drawn. egui reaches another family only by
  a `FontFamily::Name`, which epaint draws as boxes where no face is
  registered under it (`epaint-0.37.0/src/text/family.rs`, `Family::new`: "No
  text will be visible"; `src/text/fonts.rs`, `MissingGlyphPolicy::Tofu`, its
  default), a weight only through a run's
  `TextFormat::coords` (`epaint-0.37.0/src/text/text_layout_types.rs`), which
  moves a variable face's `wght` axis alone and leaves a static face at its
  own weight, and a slant only as `RichText::italics`'s synthetic skew
  (`egui-0.37.0/src/widget_text.rs`); the connector draws none of these in a
  role's scope.
* Three outline colours are drawn otherwise than under the 0.7.2 model, each
  the stroke of the widget's connector cell, or laid over it as the cell lays
  its own (the radio's `radio.indicator`): the radio button's circle, the
  drop-down's border and the segmented control's frame and dividers
  (windows-11, macos-monterey, macos-tahoe). The 0.8.0 theme lays these
  translucent borders inside the part, and egui ends a rect's fill at its stroke's inner edge (`epaint-0.37.0/src/tessellator.rs`,
  `stroke_and_fill_path`), so the connector hands egui each one composited
  over the part's fill: an opaque colour where 0.7.2 gave the translucent one
  (the connector's
  [What the 0.8.0 model states that egui cannot draw](../native-theme-egui/README.md#what-the-080-model-states-that-egui-cannot-draw)).
  Over the radio's, the drop-down's and the segments' fill that colour looks
  the same. The expander's frame has no fill under it, so it is drawn in
  `expander.container(NONE)`'s stroke colour as stated, not in its cell's
  composite.

## Compatibility

Built for egui 0.37.0, which it reaches through the connector, with
`rust-version = "1.95"`, as the connector; the connector's
[Compatibility](../native-theme-egui/README.md#compatibility) section has the
floors and the versions verified.

## Quick start

Add the connector, with the OS readers (`native`), and this crate:

```toml
[dependencies]
eframe = "0.37.0"
native-theme-egui = { version = "0.7", features = ["native"] }
native-theme-egui-widgets = "0.7"
```

Install the connector's atlas once, as the connector's
[Quick start](../native-theme-egui/README.md#quick-start) shows, then add a
widget. This crate re-exports the connector as `connector`, so its `egui` is
`native_theme_egui_widgets::connector::egui`:

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

## Links

- [API reference on docs.rs](https://docs.rs/native-theme-egui-widgets)
- [The connector, `native-theme-egui`](../native-theme-egui/README.md)
- [CHANGELOG](../../CHANGELOG.md)

## License

Licensed under any of

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)
- [0BSD License](https://opensource.org/license/0bsd)

at your option.
